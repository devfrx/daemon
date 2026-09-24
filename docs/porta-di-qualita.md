# La porta di qualità — dove vive ogni controllo

> Questo file non decide niente. Il catalogo è la
> [§7.4 della spec](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md); qui c'è
> soltanto la mappa fra ogni riga del catalogo e il file che la implementa.
>
> ⛔ **Un controllo nuovo entra prima nel catalogo, poi qui.** L'ordine inverso è il
> gotcha #36: una sezione decide un meccanismo, lo scrive nella propria tabella, e il
> catalogo resta indietro — è già successo due volte.

📚 **Qui c'è ciò che è vero adesso; la cronaca sta in archivio.** Le tabelle di mutazione, i richiami datati, i
riconteggi e le passate di revisione stanno, parola per parola, in
[`archivio/porta-di-qualita-storico.md`](archivio/porta-di-qualita-storico.md), il file com'era il 2026-09-24. Dove è
finita ogni sezione di prima, e dove si risolvono i nomi che il codice usa per puntare qui — *«task 8 section»*,
*«section "P-2"»*, *«row 15 of the mutation campaign»* — lo dice l'ultima sezione, *«Dove è finita ogni sezione di
prima»*. Un'affermazione viva che il codice di oggi smentisce porta il segno `⚠️ **[C-…]**`, e la sezione *«Le
contraddizioni registrate, e non risolte»* dice quale sia la prova: si correggono in una sessione loro, non qui.

⛔ **Una voce aperta lasciata dall'audit del 2026-08-11, la più grave del suo rapporto:** la suite di conformità prova
**V6 solo su un archivio vuoto** — gotcha **#63**. Non è riparabile qui: è un'**aggiunta al contratto di una porta
condivisa**, cioè una decisione del proprietario. Il dettaglio, con la mutazione e la prova che è osservabile, sta in
[`audit-2026-08-11.md`](audit-2026-08-11.md) §5.1. ⚠️ **[C-S0-2]**

**Un comando solo:** `bash scripts/gate.sh`

| # | Il passo, col nome che `gate.sh` gli dà | |
|---|---|---|
| 1 | `workspace build` | rende **visibile** il livello 1 |
| 2 | `example and compile-fail tests` | banco `compile_fail`, contro-sonde, round-trip delle voci spedite |
| 3 | `no-OS gate` | livello 2 — `scripts/gate-no-os.sh` |
| 4 | `allow-list on the two graphs` | livello 2 — `scripts/gate-deps.sh` |
| 5 | `attributes of the constrained crates` | livello 2 — `scripts/gate-attributes.sh` |
| 6 | `documentation consistency` | livello 2 — `scripts/check-docs.sh` |
| 7 | `DST campaigns -- wall time` | il **tempo di parete** delle campagne, ristampato con `--nocapture` |

⚠️ **[C-S0-1]**

I comandi con le loro opzioni vivono in `scripts/gate.sh`, in una casa sola: qui i passi si nominano con l'**etichetta**
che lo script stesso dà loro, che identifica senza invitare a eseguire. È la regola di `CLAUDE.md` — *un puntatore che
vive in più documenti si toglie, non si ricorregge*.

⚠️ **Il log del cancello non è una corsa sola, e la baseline non si aggrega da lì.** Il passo delle campagne le
ristampa con `--nocapture`, quindi il log porta **più** righe `test result:` di
`cargo test --locked --workspace --no-fail-fast`, e sommarle dà un totale più alto: la misura giusta si prende da quel
comando.

⚠️ **I primi due passi non «sono» il livello 1.** Il livello 1 *è* il compilatore e vale a ogni compilazione, senza che
nessuno lo lanci; `gate.sh` compila perché una porta che non compila non prova niente. La stessa distinzione è scritta
in testa a `scripts/gate.sh`.

La CI lancia lo stesso identico comando: `.github/workflows/quality-gate.yml` — `name: quality gate`, job `gate`.

📌 **Leggere la CI da terra**, senza `gh`: dall'API pubblica di GitHub, le ultime corse e poi i job di ciascuna — due
per corsa, `ubuntu-latest` e `windows-latest`, la matrice di X-1. È la **casa unica** di questi due comandi: le consegne
di fine sessione rimandano qui invece di ricopiarli, e `per_page` si alza fino a coprire i commit da controllare.

```bash
python -c "import json,urllib.request as u; d=json.load(u.urlopen(u.Request('https://api.github.com/repos/devfrx/daemon/actions/runs?per_page=4',headers={'User-Agent':'harness'}))); [print(r['created_at'][:16],r['status'],r['conclusion'],r['head_sha'][:7],r['html_url']) for r in d['workflow_runs']]"
python -c "import json,urllib.request as u; d=json.load(u.urlopen(u.Request('https://api.github.com/repos/devfrx/daemon/actions/runs?per_page=2',headers={'User-Agent':'harness'}))); [print(r['head_sha'][:7], j['name'], j['status'], j['conclusion']) for r in d['workflow_runs'] for j in json.load(u.urlopen(u.Request(r['jobs_url'],headers={'User-Agent':'harness'})))['jobs']]"
```

## Livello 1 — il compilatore

La tabella mappa sui casi `compile_fail` le righe del **blocco A** di §7.4.1 (le prime tre, senza prefisso), del
**blocco C** e del **blocco B**. ⛔ **I numeratori non si scrivono e non si deducono da una frase** — gotcha **#31**: le
righe coperte si ricontano sulle celle dei blocchi B e C in «Cosa la porta NON controlla». Un numeratore invecchia anche
quando nessuno tocca il denominatore. I passaggi riga per riga, coi loro richiami datati, stanno in
[archivio](archivio/porta-di-qualita-storico.md), sezione «Livello 1 — il compilatore».

`Q2 · §5.1` è **una** riga di catalogo — la voce aperta **R3**, più sotto — e la tabella la mostra come **due** righe di
mappatura regola-a-file, regola A e regola B: una scelta dichiarata, non una svista. La convenzione con cui si conta sta
nella riga del blocco C in «Cosa la porta NON controlla».

⛔ **Un innesco scritto lo raccoglie il compito che lo fa scattare, anche se non è nel suo brief:** un innesco che scatta
e che nessuno raccoglie lascia il registro a mentire con autorevolezza.

| Regola del catalogo | Dove è dichiarata | Caso negativo |
|---|---|---|
| `#![no_std]` su `kernel` e `simulator` | `crates/kernel/src/lib.rs` · `crates/simulator/src/lib.rs` | `crates/kernel/tests/compile_fail/std_in_kernel.rs` |
| `#![forbid(unsafe_code)]` sulle stesse | idem | `crates/kernel/tests/compile_fail/unsafe_in_kernel.rs` · `allow_overrides_forbid.rs` |
| `HashMap` non nominabile | conseguenza gratuita di `no_std` | `crates/kernel/tests/compile_fail/hashmap_in_kernel.rs` |
| **blocco C** · `V29 · §2.1` — i due tempi non si scambiano, **in nessuna delle due direzioni** | `crates/kernel/src/time.rs` — `Monotonic` e `WallTime` sono due tipi distinti | `monotonic_as_wall.rs` · `wall_as_monotonic.rs` |
| **blocco C** · `V29 · §2.1` — **nessuna via `From`/`Into`** fra i due tempi | idem: nessuna conversione è dichiarata | `no_conversion_from_monotonic_to_wall.rs` · `no_conversion_from_wall_to_monotonic.rs` |
| **blocco C** · `V29 · §2.2` — la **riduzione** di `below` non è sovrascrivibile | `crates/kernel/src/rng.rs` — `below` vive su `RngExt`, con `impl<R: Rng> RngExt for R {}` | `override_below.rs` |
| **blocco C** · `V29 · §2.8` — il kernel **non nomina un default** | `crates/kernel/src/parameters.rs` — nessun `impl Default`, e `new` pretende ogni campo | `parameters_have_no_default.rs` |
| **blocco C** · `V29 · §2.8 · ADR-0034` — una decisione **senza i parametri consegnati** | `crates/kernel/src/executor.rs` — `Executor::new` prende `Parameters` **per posizione**: ometterli è un errore di arità, non un valore messo di default in silenzio | `executor_without_parameters.rs` |
| **blocco C** · `Q9 · I6 · V20` — `Untrusted` **dove è attesa** un'`Instruction`, **regola A** | `crates/kernel/src/boundary.rs` — `Instruction` e `Untrusted` sono due tipi distinti | `untrusted_as_instruction.rs` |
| **blocco C** · `Q9 · I6 · V20` — **nessuna via `From`/`Into`** da `Untrusted` a `Instruction`, **regola B** | idem: nessuna conversione è dichiarata, e l'unica strada ammessa è `promote`, che pretende il giornale | `no_conversion_from_untrusted_to_instruction.rs` |
| **blocco C** · `Q2 · §5.1` — VRAM e durata non si scambiano, **in nessuna delle due direzioni** — **regola A** | `crates/kernel/src/arbiter/resource.rs` — `Mib` e `kernel::time::Millis` sono due tipi distinti | `mib_as_millis.rs` · `millis_as_mib.rs` |
| **blocco C** · `Q2 · §5.1` — **nessuna via `From`/`Into`** fra `Mib` e `Millis` — **regola B** | idem: nessuna conversione è dichiarata | `no_conversion_from_mib_to_millis.rs` · `no_conversion_from_millis_to_mib.rs` |
| **blocco C** · `Q14 · §4.9` — un record durevole **senza versione** | `crates/kernel/src/record.rs` — `Record` è un enum di versione, e l'`encode` inerente vive su di esso, non sul corpo `RecordV1` | `record_without_version.rs` |
| **blocco C** · `Q9 · I6 · V20 · §4.9` — un payload non fidato **senza la propria etichetta** | `crates/kernel/src/record.rs` — `RecordV1::trust` non è un `Option` e `Trust` non implementa `Default`. ⛔ **Le metà sono due e i casi sono due**: il primo tiene *«il campo esiste»* (`E0063`), il secondo *«e non ha default»* (`E0277`) | `record_without_trust_label.rs` · `trust_has_no_default.rs` |
| **blocco C** · `V4` — l'esito dell'arbitro trattato come **due vie** invece di tre | `crates/kernel/src/arbiter/mod.rs` — `Admission` ha **tre** varianti, nessun `is_granted()` e nessuna conversione a booleano: distinguerle non è una raccomandazione, è la firma. ⛔ **Le metà sono due e i casi sono due**: il primo tiene *«un `match` a due bracci non compila»* (`E0004`), il secondo *«e non c'è nemmeno la scorciatoia booleana»* (`E0599`). Contro-sonda: `an_admission_is_distinguishable_three_ways` | `admission_is_not_two_ways.rs` · `admission_has_no_is_granted.rs` |
| **blocco C** · `V2` — un'**ammissione senza profilo** di risorsa | `crates/kernel/src/arbiter/mod.rs` — `Arbiter::admit` prende il `&ResourceProfile` **per posizione**: ometterlo è un errore di arità (`E0061`), non un'ammissione che decide sul nulla. Contro-sonda: le sonde di `crates/kernel/tests/arbiter_admission.rs` che chiamano `admit` col profilo, da fuori la crate | `admission_without_profile.rs` |
| **blocco C** · `V3` — una **seconda policy attiva**. La seconda metà della contro-sonda — *«e la transizione resta un passo giornalato (§5.4)»* — è `Arbiter::set_policy`, esercitata dalle sonde della transizione in `crates/kernel/tests/arbiter_policy.rs` | `crates/kernel/src/arbiter/policy.rs` e `mod.rs` — `VramPolicy` è un **enum**, quindi un valore ne porta una sola, e `Arbiter::new` lo prende **per posizione**: passarne due è un errore di arità. ⚠️ Il caso pinza l'**arità di `new`** e non l'assenza di ogni strada: con un secondo costruttore a due policy resta `ok`, misurato | `two_policies_at_once.rs` |
| **blocco C** · `Q8 · §5.2.1` — l'**ammissione legge `cold_start`** | `crates/kernel/src/arbiter/` — `admit` riceve un `ResourceProfile`, che quel campo non ce l'ha; `cold_start` vive su `WorkDescriptor`, che l'ammissione non riceve (`E0609`). ⚠️ Il caso costruisce il profilo **e lo passa ad `admit` nello stesso `main`**: la regola è provata sull'argomento che la decisione prende davvero ⚠️ **[C-S1-4]**. Contro-sonda: `cold_start_is_readable_outside_the_decision_path` in `crates/kernel/tests/arbiter_resource.rs` | `admission_reads_cold_start.rs` |
| **blocco C** · `I2 · §5.3` — la **revoca** per un profilo **non prelazionabile** | `crates/kernel/src/arbiter/mod.rs` — `Activity::NonPreemptible` è una variante **unitaria**: non ha dove metterla, quindi lo stato illegale non è vietato, è **indicibile** (`E0618`). Contro-sonda: `a_revocation_is_constructible_on_the_preemptible_side`. ⚠️ La cella di catalogo scrive `InRevoca`, il codice `Revoking` per la §1.0 della spec: è la voce `R5` | `revoking_a_non_preemptible_grant.rs` |
| **blocco C** · `I2 · §6.10` — **istruire un worker dopo l'uccisione**: l'uccisione **consuma** il `Worker` (§6.10.2) | `crates/kernel/src/ports/process.rs` — `Worker::kill` prende `self` **per valore**: istruirlo dopo non è vietato da una regola di condotta, è un uso dopo spostamento (`E0382`). ⚠️ La cella di catalogo scrive `uccidi`, il codice `kill` per la §1.0 della spec: stessa specie di `R5`, ma la cella è del Traguardo 2, quindi è la voce `E140` — chiuderla è del **proprietario**, perché tocca la §7.4 | `instructing_after_the_kill.rs` |
| **blocco C** · `I5 · §6.10` — **leggere due volte dalla stessa ricevuta singola**: la lettura la consuma (§6.10.2) | `crates/kernel/src/ports/process.rs` — `Worker::read_one` prende la `SingleReceipt` **per valore**, e `SingleReceipt` non è `Copy` — né lo può diventare, perché non deriva nemmeno `Clone` — quindi la seconda lettura è un uso dopo spostamento (`E0382`). ⚠️ **Il derive che fa da cardine è `Copy`, non `Clone`, ed è quello che l'oracolo nomina** (`reading_twice_from_one_receipt.stderr`: *«does not implement the `Copy` trait»*): chi verificasse la guardia guardando `#[derive(Clone)]` guarderebbe la cosa sbagliata | `reading_twice_from_one_receipt.rs` |
| **blocco C** · `V10` — un **sensore che modifica l'artefatto**: §6.4.2 lo consegna per riferimento immutabile | `crates/kernel/src/sensor.rs` — `Sensor::observe` prende `artefact: &Untrusted`, quindi assegnarvi dentro è `error[E0594]`, *cannot assign to `*artefact`, which is behind a `&` reference*. ⚠️ **Le vie sono due, e la seconda è misurata:** un `impl` che **dichiara** `observe(&self, artefact: &mut Untrusted)` non combacia col tratto e dà `error[E0053]`. ⛔ **Il caso scritto è quello del CORPO**, perché la cella parla della **consegna** e non di ciò che un implementatore può dichiarare; la seconda via è scritta nel commento del caso, perché chi allarga la riga sappia che esiste ed è già chiusa | `sensor_modifies_the_artefact.rs` |
| **blocco B** · `V19` — **promuovere testo a istruzione ← la porta `journal`** | `crates/kernel/src/boundary.rs` — `Untrusted::promote` pretende il giornale come **argomento**, e la registrazione fallita fa fallire la promozione | `promote_without_journal.rs` |
| **blocco B** · `I2` — **avviare un worker ← una concessione** | `crates/kernel/src/arbiter/mod.rs` — `Grant` vive dove lo si **emette**, con un campo privato e nessun costruttore pubblico; `Process::start` lo pretende per valore, e `Arbiter::admit` è l'**unico** che ne conia uno (l'oracolo è senza sigla). Contro-sonda: `releasing_gives_back_exactly_the_reservation`, che ottiene un `Grant` vero da `admit` | `grant_has_no_constructor.rs` |
| **blocco B** · `I2` — **parlare a un worker ← l'oggetto `Worker`** che l'avvio ha restituito | `crates/kernel/src/ports/process.rs` — `instruct_one` vive sul tratto `Worker`, e ciò che si ha **prima** dell'avvio — `WorkerDescriptor` — non lo implementa: il metodo non esiste su di esso (`E0599`). La direzione *«col `Worker` → compila»* la tiene `crates/kernel/tests/worker_tokens.rs` | `talking_without_the_handle.rs` |
| **blocco B** · `I5 · Q4` — **leggere da un worker ← una ricevuta** | `crates/kernel/src/ports/process.rs` — `Worker::read_one` pretende la `SingleReceipt` come argomento, quindi ometterla è un errore di **arità** (`E0061`). ⛔ **Il caso prova l'arità e non l'autenticità della ricevuta:** `SingleReceipt::new` è `pub` e raggiungibile da fuori la crate, e il limite è dichiarato accanto al costruttore nel sorgente — il contrario del gettone `Grant`, che un costruttore pubblico non ce l'ha. La direzione *«con la ricevuta → compila»* la tiene `crates/kernel/tests/worker_tokens.rs` | `reading_without_a_receipt.rs` |
| **blocco B** · `Q13` — **eseguire una richiesta ← una prova di conformità** | `crates/kernel/src/gateway/mod.rs` — `Conforming` ha **tutti** i campi privati e l'unico che ne conia uno è `resolve`, nello stesso modulo; `dispatch` lo pretende **per valore**. ⛔ **Le metà sono due e i casi sono due**, come per il gettone `Grant`: il primo tiene *«un candidato non filtrato non è l'argomento»* (`E0308`), il secondo *«e il gettone non si conia»* (senza sigla). La direzione *«filtrato → compila»* la tiene `a_conforming_candidate_is_chosen_and_nothing_is_degraded` in `crates/kernel/tests/gateway_decisor.rs` | `dispatching_an_unfiltered_candidate.rs` · `conforming_has_no_constructor.rs` |

⛔ Un controllo che un compito implementa e che il catalogo non ha si dichiara nel registro come **voce aperta**, non
come nota: la regola è il gotcha **#36**, in [`HANDOFF.md`](HANDOFF.md).

#### Come scattano le due direzioni, e perché la differenza conta

⛔ **Le due direzioni non sono simmetriche nel modo di scattare, e la differenza conta** — gotcha **#42**, in
[`HANDOFF.md`](HANDOFF.md): `trybuild` stampa **`error`** quando un caso ha compilato e **`mismatch`** quando l'uscita
non combacia con l'oracolo, e una guardia che scatta solo come `mismatch` la spegne in silenzio una rigenerazione in
blocco. La parola si legge con `cargo test --locked -p kernel --test compile_fail 2>&1 | grep -E "error|mismatch|ok"`.

| Regola | Scatta come | Dipende dall'oracolo? |
|---|---|---|
| i due tempi non si scambiano | `mismatch` | **sì** — una rigenerazione in blocco la spegnerebbe in silenzio |
| nessuna via `From`/`Into` fra i due tempi | **`error`** | no |
| `below` non sovrascrivibile | **`error`** | no |
| l'esecutore senza i parametri consegnati | **`error`** | no |
| `Untrusted` dove è attesa un'`Instruction` — **regola A** | ⛔ **niente: resta `ok`** | no, ed è **peggio** — vedi sotto |
| nessuna via `From`/`Into` da `Untrusted` a `Instruction` — **regola B** | **`error`** | no |
| un record durevole senza versione | **`error`** | no |
| un payload non fidato senza la propria etichetta — **il campo esiste** | **`error`** | no |
| la stessa riga, **e non ha default** — `trust_has_no_default.rs` | **`error`** | no |
| Mib e Millis non si scambiano — **regola A** (`mib_as_millis.rs` · `millis_as_mib.rs`) | `mismatch` | **sì** — stessa specie dei due tempi: sono **valori posseduti**, e con l'impl presente rustc appende un `help: call \`.into()\`` che l'oracolo non porta |
| nessuna via `From`/`Into` fra Mib e Millis — **regola B** | **`error`**, in entrambe le direzioni | no |
| l'esito dell'arbitro a **due vie** (`admission_is_not_two_ways.rs`) | **`error`** | no |
| la **revoca** su un profilo non prelazionabile (`revoking_a_non_preemptible_grant.rs`) | **`error`** | no |
| la concessione **forgiata da fuori** (`grant_has_no_constructor.rs`) | ⛔ **dipende da COME si rompe la regola** — `error` se il tipo perde i campi, `mismatch` se il campo diventa `pub` | **in parte sì** — vedi la sezione del Task 4 |
| la **scorciatoia booleana** sull'esito (`admission_has_no_is_granted.rs`) | **`error`** | no — misurato con `is_granted` aggiunto |
| l'**ammissione senza profilo** (`admission_without_profile.rs`) | **`error`** | no — misurato togliendo il profilo dalla firma di `admit` |
| l'**ammissione che legge `cold_start`** (`admission_reads_cold_start.rs`) | `mismatch` | ⛔ **sì**, e va detto invece di lasciarlo dedurre — vedi la sezione del Task 5 |
| la **maniglia** — parlare a un worker senza l'oggetto `Worker` (`talking_without_the_handle.rs`) | **`error`** | no — ⚠️ **soltanto grazie all'import `Worker as _`**: senza, sotto `impl Worker for WorkerDescriptor` il metodo resterebbe irrisolvibile e il caso andrebbe `mismatch`. La ragione sta accanto all'import, nel sorgente |
| **istruire dopo l'uccisione** (`instructing_after_the_kill.rs`) | ⛔ **dipende da COME si rompe la regola** — `error` se `kill` smette di prendere `self` **e** la finta del caso segue la firma nuova; `mismatch` se cambia solo il tratto, perché a rompersi per prima è la finta (`E0053`) | **in parte sì** |
| **leggere senza ricevuta** (`reading_without_a_receipt.rs`) | ⛔ **dipende da COME si rompe la regola**, stessa specie della riga qui sopra — `error` col tratto **e** la finta in passo; `mismatch` col solo tratto (`E0050`) | **in parte sì** |
| **leggere due volte dalla stessa ricevuta** (`reading_twice_from_one_receipt.rs`) | **`error`** con `Copy` su `SingleReceipt`, che è il derive che disarma | **in parte sì** — ⚠️ col solo `Clone` il caso **non** è disarmato e resta rosso, ma `mismatch`: rustc appende un `help: consider cloning the value` che l'oracolo non porta |

⛔ **Le righe di §6.10.5 sono classificate MISURANDO la regressione di ciascuna, non deducendola.** Due rispondono
«dipende», e la ragione è strutturale: quei casi portano una **copia** della finta, quindi se la firma del tratto cambia
da sola è la finta a rompersi per prima — un rosso vero, ma della specie che una rigenerazione in blocco riscrive. Come
sono state misurate, fuori dal repository, sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «Come
scattano le due direzioni, e perché la differenza conta».

⛔ **La riga della regola A di I6 è cieca fin dall'inizio.** Il gotcha #42 prevede un `mismatch`, e su questa coppia non
succede: con `impl From<Untrusted> for Instruction` presente, `untrusted_as_instruction.rs` resta **`ok`**, perché lo
scarto è fra **riferimenti** (`&Untrusted` contro `&Instruction`), quell'impl non dà nessun
`&Untrusted: Into<&Instruction>`, e rustc non ha suggerimenti da appendere. Sui due tempi, e su `Mib`/`Millis`, lo scarto è fra
**valori posseduti**, e lì il suggerimento compare. Su I6 il caso della regola B non è una rifinitura: senza,
l'`impl From` lascia la porta **verde** con il confine già caduto.

Le righe «nessuna via `From`/`Into`» esistono perché col solo caso «passa l'uno per l'altro» aggiungere
`impl From<WallTime> for Monotonic` lasciava la porta **verde**, ed era la direzione **pericolosa** — una decisione che
dipende dal wall time. Una regola guardata solo da casi `mismatch` non è guardata abbastanza. Le due mutazioni che
provano la stessa specie su `Mib`/`Millis` stanno in [archivio](archivio/porta-di-qualita-storico.md), sezione «Come
scattano le due direzioni, e perché la differenza conta».

⚠️ **VOCE APERTA — la riga di catalogo `Q2 · §5.1` è UNA e in UNA direzione, mentre qui sopra sono DUE regole in DUE
direzioni ciascuna (R3 del piano del Traguardo 5).** §7.4.1 blocco C scrive *«MiB assegnati a millisecondi»*,
contro-sonda *«ciascuno con sé stesso»*: la forma a una via che la riga dei due tempi aveva prima che V29 la allargasse.
I quattro casi mordono già in entrambe le direzioni e su entrambe le regole; a mancare è la **riga**, non il controllo.
Aggiornare §7.4 è **spec**, quindi del proprietario — vincolo globale 7. Registrata come voce aperta e non come nota
(gotcha #36), come PL-1 e K-1/B-1.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo a questo file**, che
dice anche **chi la chiude**.

#### `ComputeClass` e `Preemption` — Traguardo 5, Task 2, e nessuna riga di catalogo qui

⛔ **`ComputeClass` NON ha una riga di catalogo, ed è dichiarato invece di lasciarlo dedurre.** L'ordine delle tre corsie
di calcolo è un **valore**, non una forma: il compilatore non ha niente da rifiutare se qualcuno riordina le varianti
dell'enum, quindi non è materia di `compile_fail`. Lo tengono tre sonde a esempi in
`crates/kernel/tests/arbiter_resource.rs`:

| Sonda | Cosa tiene |
|---|---|
| `the_lane_order_is_pinned_by_name_and_realtime_comes_first` | l'ordine **per nome** — `Realtime < Interactive < Batch` — e la chiave `priority()` stessa, così un lettore non deve inferirla dalle disuguaglianze |
| `the_three_lanes_are_distinct_and_the_ordering_is_total` | la contro-sonda: l'ordinamento è **totale**, non solo le tre disuguaglianze dichiarate — nessuna coppia di corsie confronta uguale a un'altra |
| `a_grace_time_exists_exactly_when_the_profile_is_preemptible` | ciò che `Preemption` rende **non dicibile**: un profilo non preemptibile non può portare un tempo di grazia, e uno preemptibile non può esserne privo — due stati illegali insieme, non uno |

⛔ **L'ordine viene dalla chiave esplicita `ComputeClass::priority`, non da un `Ord` derivato.** Riordinare le varianti
nella dichiarazione dell'enum, con la chiave intatta, resta verde: la trappola di un `Ord` derivato è **tolta**, non
sorvegliata — con un `Ord` derivato quella stessa mutazione rovescerebbe le priorità dell'arbitro senza un rosso.
Provato nelle due direzioni; le mutazioni stanno in [archivio](archivio/porta-di-qualita-storico.md), nella sezione
dallo stesso titolo di questa.

#### `ResourceProfile` e `WorkDescriptor` — Traguardo 5, Task 3, e `Q8 · §5.2.1` diventa parziale

⛔ **Le due strutture vivono in `crates/kernel/src/arbiter/resource.rs`, legate SOLO dal nome (§5.2.1).**
`ResourceProfile` è ciò che l'arbitro RICEVE (§5.2): `name` (`&'static str`, mai `String` — finding P-1),
`reserved_vram`, `compute_class`, `preemption`, e SENZA `cold_start`. `WorkDescriptor` va alla PRESENTAZIONE e mai
all'ammissione: `profile_name` e `cold_start`. Le sonde, in `crates/kernel/tests/arbiter_resource.rs`:

| Sonda | Cosa tiene |
|---|---|
| `cold_start_is_readable_outside_the_decision_path` | la contro-sonda di `Q8 · §5.2.1`: il campo È raggiungibile fuori dal percorso decisionale. ⚠️ **Il lettore è una FINTA.** Il catalogo chiama quel lettore «la proiezione di presentazione», che nel codice non esiste ancora: la sonda usa una funzione libera, `a_presentation_projection`, che prova la proprietà giusta con parole diverse da quelle della cella. Divergenza per il proprietario, che la riformulerà quando la proiezione vera esisterà (§12 del disegno del Traguardo 5) |
| `a_descriptor_names_the_profile_it_describes` | che le due strutture si tengano per nome e nient'altro. ⚠️ **Forza di LIVELLO 1 soltanto — errata E5 del piano.** Il test scrive lo stesso letterale (`"trellis2-512-lean"`) in entrambe le strutture: l'`assert_eq!` a runtime **non può fallire**, qualunque cosa faccia il codice di produzione. Ciò che prova davvero è al COMPILATORE — che le due forme esistano, portino quei nomi e quei tipi di campo, e siano costruibili da FUORI la crate (l'unico posto dove un `pub` mancante si vede, gotcha #46). Non si conta fra le sonde che mordono a runtime |

⛔ **Nessuna sonda a esempi tiene «`ResourceProfile` non ha `cold_start`»:** aggiunto il campo, l'intero workspace resta
verde, misurato. A tenere la regola dove le sonde a esempi non arrivano è il solo caso di livello 1,
`admission_reads_cold_start.rs` (`E0609`). La mutazione sta in [archivio](archivio/porta-di-qualita-storico.md),
sezione «`ResourceProfile` e `WorkDescriptor` — Traguardo 5, Task 3, e `Q8 · §5.2.1` diventa parziale».

#### `Grant`, `Admission` e `Activity` — Traguardo 5, Task 4, e `V4` e `I2 · §5.3` si chiudono

⛔ **`Grant` vive in chi lo emette, `crates/kernel/src/arbiter/mod.rs`, e non in chi lo consuma, per una ragione
meccanica:** in Rust un campo privato è visibile al modulo che lo dichiara **e ai suoi figli**, e il modulo `arbiter` è un
**fratello** di `ports::process` — col tipo di là l'arbitro non potrebbe costruire la cosa che esiste per emettere
(`E0423`, decisione **D5-1** del disegno). ⚠️ **Un costruttore `pub(crate)` è scartato sul merito:** costa una riga e apre
una strada — chiunque dentro `kernel` conierebbe una concessione senza passare dall'ammissione (gotcha **#67**)
⚠️ **[C-S1-5]**.

⛔ **`Grant` NON si ri-esporta da `ports::process` (decisione D8).** Il `use crate::arbiter::Grant;` dentro
`ports::process` è **privato**, quindi da fuori il nome resta *visibile ma vietato* —
`` error[E0603]: struct `Grant` is private ``, com'è successo a `crates/kernel/tests/ports_are_implementable.rs` — e non
un «import non risolto»: con un `pub use` nessun sito si sarebbe presentato, e i due percorsi sarebbero nati invisibili.

| Caso negativo | Riga | Errore nell'oracolo |
|---|---|---|
| `admission_is_not_two_ways.rs` | `V4` | `` error[E0004]: non-exhaustive patterns: `Admission::Queued(_)` not covered `` |
| `revoking_a_non_preemptible_grant.rs` | `I2 · §5.3` | `` error[E0618]: expected function, found `Activity` `` — *«call expression requires function»* |
| `grant_has_no_constructor.rs` | blocco **B**, *«avviare un worker ← una concessione»* | ⛔ **senza sigla:** `` error: cannot construct `Grant` with struct literal syntax due to private fields ``, con `` note: private field `id` that was not provided `` ⚠️ **[C-S1-10]** |

⚠️ **`grant_has_no_constructor.rs` appartiene a ENTRAMBE le specie della tabella delle due direzioni.** Il caso scrive
`kernel::arbiter::Grant {}`, **senza campo**. Rendere `id` pubblico non fa compilare quel letterale: manca ancora il campo
(`E0063`), e `GrantId` è un tipo **privato** che una crate esterna non può nominare, quindi la strada resta chiusa da
**due** porte — e il caso passa da `ok` a **`mismatch`**, cioè **dipende dall'oracolo** (gotcha **#42**, stessa specie di
`mib_as_millis.rs`). Rotta per l'altro verso — il tipo svuotato dei campi — scatta come **`error`**. Sta scritto perché
nessuno la conti fra le forti; le mutazioni stanno in [archivio](archivio/porta-di-qualita-storico.md), sezione
«`Grant`, `Admission` e `Activity` — Traguardo 5, Task 4, e `V4` e `I2 · §5.3` si chiudono».

⛔ **Le seconde direzioni di `V4` e di `I2 · §5.3` sono SONDE e non mutazioni:** una mutazione **sparisce quando la
revochi**, e una direzione tenuta da qualcosa che non resta non è tenuta (§7.1.1 regola 3). Stanno in
`crates/kernel/tests/arbiter_admission.rs`:

| Sonda | Cosa tiene |
|---|---|
| `an_admission_is_distinguishable_three_ways` | la contro-sonda del catalogo per `V4` — *«distinguere le tre compila»*. Costruisce `Admission::Refused { asked, ceiling }` — l'unica via costruibile da fuori la crate: `Granted` porta un `Grant` senza costruttore pubblico, `Queued` un `TicketId` dal campo privato — e la fa passare per un `match` che nomina tutte e tre le varianti. ⚠️ **Il suo morso è SOLO a compilazione**, dichiarato così |
| `a_revocation_is_constructible_on_the_preemptible_side` | la contro-sonda del catalogo per `I2 · §5.3` — *«costruibile per uno prelazionabile»*. Costruisce `Activity::Preemptible(PreemptibleState::Revoking { deadline })` e lo distingue **sia** da `Preemptible(Running)` **sia** da `NonPreemptible`. ⚠️ `assert_ne!` è lecito qui e **non** viola `R2`: quella restrizione riguarda `Admission`, che porta un `Grant` e quindi non ha né `Debug` né `PartialEq`; `Activity` li deriva entrambi. ⚠️ **Il suo morso a runtime è apparente, non reale** — vedi sotto |

⛔ **Le due `assert_ne!` di `a_revocation_is_constructible_on_the_preemptible_side` NON POSSONO fallire a runtime,
qualunque cosa faccia il codice di produzione, finché la `derive(PartialEq)` di `Activity` resta** (errata **E17** del
piano): confrontano varianti strutturalmente distinte, e con `PartialEq` derivato due varianti diverse non sono mai
uguali — stessa specie di `E5`. ⚠️ **La forza reale e sufficiente della sonda è alla COSTRUZIONE**, scritta da fuori la
crate: `Activity::Preemptible(PreemptibleState::Revoking { deadline })` compila solo se la nidificazione di §5.3 punto 3
esiste con quella forma esatta — è quella, insieme al `match` esaustivo dell'altra sonda, a chiudere le due righe di
catalogo. Le due `assert_ne!` restano, perché documentano che `revoking` porta davvero il valore che il test intende: un
`PartialEq` derivato le rende infallibili, non inutili.

⛔ **La separazione dei file è per soggetto:** `crates/kernel/tests/arbiter_resource.rs` porta il vocabolario della
**risorsa**; `crates/kernel/tests/arbiter_admission.rs` l'ammissione e `Activity`, che non è una risorsa — è ciò che una
concessione **sta facendo**.

⛔ **`Grant::id` ha per lettore `Arbiter::release`** (decisioni **D2** e **D3** del disegno). Un campo nasce col proprio
consumatore, e un avviso `field is never read` non si spegne con `#[allow(dead_code)]`, che è un divieto spento in
permanenza (gotcha #13): niente `allow`, nessun lettore inventato, nessun accessore.

#### `Parameters::total_vram` e l'arbitro che ammette e rilascia — Traguardo 5, Task 5

⛔ **Il totale è CONSEGNATO, non chiesto.** Interrogare la GPU è una chiamata al sistema operativo, che I3 vieta al
kernel, e nessuna delle famiglie di porte consegna la capacità dell'hardware; quindi `Parameters` porta `total_vram`, e i
siti chiamanti lo scrivono sul posto — un default in `Parameters` è precisamente ciò che §2.8.2 regola 2 vieta.

⛔ **`Arbiter` è logica pura dei propri ingressi**, con `BTreeMap` da `alloc` (`HashMap` non è nominabile: `no_std`,
gotcha #12), e **ogni** operazione riscuote gli scaduti prima di decidere — è il motivo per cui `collect_expired` è
privata. Le sonde stanno in `crates/kernel/tests/arbiter_admission.rs`:

| Sonda | Cosa tiene |
|---|---|
| `a_grant_takes_exactly_its_reservation_out_of_the_budget` | ⛔ **l'asserzione è il NUMERO, non la variante:** *«ha concesso»* lo soddisfa un arbitro che concede tutto; ciò che dice che il budget è reale è che `allocated` si sia mosso **della prenotazione** |
| `releasing_gives_back_exactly_the_reservation` | la metà arbitro delle proprietà 2 e 3 di §5.7 — e sono **una** e non due: l'arbitro non deve sapere **chi** teneva una concessione, solo che rilasciarla rimetta la prenotazione. È anche la contro-sonda del blocco **B** *«avviare un worker ← una concessione»*: ottiene un `Grant` da `admit` e lo consuma |
| `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` | che l'`Err` di `release` sia **raggiungibile** — la superficie morta che questo repository ha tolto da `Record::encode`. ⚠️ **Con un limite dichiarato**, nella voce aperta qui sotto ⚠️ **[C-S1-1]** |
| `the_sum_of_the_grants_never_exceeds_the_total` | l'invariante, asserito **sul numero**: la somma di **tutte** le concessioni non supera il totale — `allocated()` fermo a `8_192`, cioè *«non è stato sovra-ammesso niente»*. La terza richiesta torna `Queued`, e i **due numeri** di `Refused` li pinza `a_request_larger_than_the_total_is_refused_and_not_queued`. ⛔ **Non chiama `promote`:** tiene l'invariante dal lato dell'**ammissione**, che è la sua proprietà; dal lato di `promote` lo tengono le sonde delle code, nella sezione del Task 6 |
| `a_total_smaller_than_the_two_permanent_quotas_does_not_grant_the_second_one` | che la seconda quota permanente **non prenda** VRAM che la macchina non ha — `allocated()` fermo a `1_024`. ⛔ **Ha perso la proprietà per cui era stata scritta**, la **visibilità** di una configurazione impossibile: la seconda quota torna `Queued` e nessuno la servirà mai, il degrado silenzioso che ADR-0005 e ADR-0019 vietano. L'arbitro non può ripararlo — *«Permanence is not a type»*, nel sorgente: non distingue un biglietto che **sarà** servito da uno che non lo sarà mai. La visibilità sta alla **radice di composizione**, che le due concessioni permanenti le chiede lei e tratta un `Queued` come un fallimento d'avvio: sezione «Il grafo di produzione monta l'arbitro, il giornale e le due concessioni». Scritto anche accanto alla sonda |
| `an_expired_grant_does_not_stay_allocated` | la riscossione pigra, scritta perché sia **osservabile**: fra due operazioni una concessione scaduta resta nei libri — non nega niente a nessuno, **non c'è** nessuno — e al primo che guarda è già liberata (§5.7 proprietà 5) |
| `a_grant_still_inside_its_window_is_not_collected` | la contro-sonda, ed è la direzione che si salta: senza, *«riscuoti sempre tutto»* passa — le due sonde servono **entrambe**, perché la prima da sola la soddisfa il difetto opposto. ⚠️ **A tenere la direzione della riscossione è il `let … else`, non il numero**, ed è scritto accanto alla sonda: con *«riscuoti sempre tutto»* la seconda richiesta sarebbe `Granted` e il `let … else` andrebbe in panico, mentre `allocated()` leggerebbe `4_096` **lo stesso**. Il numero coglie l'altro difetto — una richiesta accodata che **prenota** comunque |
| `a_grant_is_collected_at_the_instant_its_window_closes` | il **confine** che le due righe qui sopra scavalcano — una guarda a `5_001`, l'altra a `4_999` — a **`5_000` esatti**. ⛔ Tiene anche **quale** delle due semantiche sia quella scelta: con `expires_at > now` la finestra è **semiaperta**, `[inizio, scadenza)`, e a `now == expires_at` la concessione è **già riscossa** ⚠️ **[C-S1-9]** |
| `a_request_larger_than_the_total_is_refused_and_not_queued` | che `32_768` contro un tetto di `8_192` torni `Refused` **coi due numeri giusti**. ⛔ È l'**unica** custode della guardia `asked > ceiling`: senza di essa una richiesta più grande dell'intera macchina verrebbe **accodata per sempre** |

Ogni sonda è provata in negativo da una mutazione del codice di produzione; la campagna, coi conteggi di allora, sta in
[archivio](archivio/porta-di-qualita-storico.md), sezione «`Parameters::total_vram` e l'arbitro che ammette e rilascia —
Traguardo 5, Task 5».

⛔ **In `admission_reads_cold_start.rs` l'`E0609` nasce dal LETTERALE e dall'ACCESSO AL CAMPO, e la chiamata ad `admit`
non partecipa** — misurato cancellandola: ciò che la chiamata compra è un **accoppiamento alla firma** di grado
`mismatch`, non di grado `error` ⚠️ **[C-S1-4]**. E rimettere `cold_start` su `ResourceProfile` **non** fa compilare il
caso: il letterale resta senza quel campo, rustc passa da `E0609` (*«no field `cold_start`»*) a `E0063` (*«missing field
`cold_start` in initializer»*), e trybuild dice **`mismatch`**. ⚠️ La guardia di `Q8 · §5.2.1` **dipende
dall'oracolo** — stessa specie di `mib_as_millis.rs`, gotcha **#42** — ed è il prezzo della forma di questo caso,
registrato anche nella tabella delle due direzioni.

⛔ **Il commento di un caso `compile_fail` si riscrive a parità di righe:** l'oracolo pinza il **numero di riga**
dell'errore, e allungare il commento manderebbe il caso `mismatch`, costringendo a rigenerare uno `.stderr` che non ha
nessuna ragione di cambiare.

⚠️ **VOCE APERTA — ciò che `release` compra davvero, e ciò che non compra.** La sonda
`a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` costruisce un secondo arbitro **vuoto**,
quindi prova *«non è nei miei libri»*, **non** *«distinguo le mie concessioni da quelle altrui»*. `GrantId` è un
progressivo che riparte da zero per ogni `Arbiter`: due arbitri che abbiano **entrambi** emesso concessioni condividono lo
spazio degli id, e il secondo accrediterebbe la concessione del primo. ⛔ **Il disegno non è stato cambiato per chiudere
il buco:** dare un'**identità** all'arbitro è una decisione del proprietario. Ciò che protegge oggi è che un processo ha
**un** arbitro — i diversi che esistono insieme esistono nei **banchi**. Il limite è scritto **accanto a `ReleaseError`
nel sorgente**, oltre che qui ⚠️ **[C-S1-1]**.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo a questo file**, che
dice anche **chi la chiude**.

⚠️ **VOCE APERTA — `release` risponde `UnknownGrant` anche a una concessione PROPRIA ma SCADUTA, e il nome della
variante afferma il falso.** `release` chiama `collect_expired` **prima** di cercare, quindi una concessione con
`expires_at <= now` è già stata tolta dai libri: `held.remove` dà `None` e si esce con `Err(ReleaseError::UnknownGrant)`,
il cui doc diceva *«This arbiter never issued that grant»* — che del caso scaduto è **falso**. Il chiamante non può
distinguere *«non è mia»* da *«era mia ed è scaduta»*; misurato, anche a `now == expires_at`, perché la finestra è
**semiaperta**. ⛔ **La scelta è PRESA NEL MERITO E VINCOLATA NELLA FORMA** (finding **A-2**): `release` **non** risponde
`Err` a una concessione **propria** — finestra scaduta e grazia scaduta non sono fallimenti del rilascio — e solo la
concessione **altrui** resta un errore. Le **due** forme scartate, col perché e coi costi rimisurati prima di decidere,
stanno accanto a `ReleaseError` in `crates/kernel/src/arbiter/mod.rs`, in una casa sola ⚠️ **[C-S1-2]**. ⚖️ **Resta il
tipo esatto della risposta**, che si disegna **insieme a `R6`** al Traguardo 6, perché discende da ciò che `Worker::kill`
chiede quando restituisce la concessione — e quel chiamante non esiste ancora (gotcha **#46** dal verso sbagliato)
⚠️ **[C-S1-2]**. ⏳ **Comincia a costare** proprio lì, dove la concessione torna a lavoro **finito**, che può benissimo
cadere dopo la finestra, e *«il rilascio è fallito»* e *«era già stato fatto per te»* sono notizie diverse. Dei chiamanti
di `release` resta *«nessun consumatore di produzione esiste»* ⚠️ **[C-S1-3]**, e il censimento lo rifà
`grep -rn '\.release(' crates/ --include=*.rs`, mai una cifra scritta.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo a questo file**, che
dice anche **chi la chiude**.

⚠️ **`parameters_have_no_default.stderr` va `mismatch` a ogni parametro nuovo** (§2.8.5: aggiungere un parametro rompe
ogni chiamante, e la promessa arriva fino all'oracolo), perché rustc chiude quell'errore con una nota che cita la firma
di `Parameters::new` **alla lettera**. La rigenerazione è **legittima e prevista**, e non disarma niente: la regola che
quel caso difende scatta come `error`, non attraverso l'oracolo. ⛔ **Mai `TRYBUILD=overwrite`:** la procedura —
l'oracolo stantio cancellato, `wip/`, `diff -u`, lo spostamento **a mano** — è il gotcha **#25**, in
[`HANDOFF.md`](HANDOFF.md).

⛔ **`crates/kernel/tests/parameters_delivered.rs` tiene la regola di §2.8.4 sul COSTRUTTORE**, perché un costruttore che
lascia stare un campo e mette un pavimento sull'altro soddisferebbe una sonda sul solo valore:
`the_constructor_substitutes_nothing_for_the_total_it_is_handed` rende indietro **identici** `Mib::ZERO` e
`Mib::new(u64::MAX)`. E `parameters_are_comparable_so_a_substitution_is_observable` porta due `Parameters` che
differiscono **nel solo totale**, senza i quali una comparazione che guardasse il solo `executor_turn_limit` passerebbe
ogni sonda del file, e sostituire un totale sarebbe inosservabile.

📌 **I conteggi si ricontano eseguendo il binario, mai si deducono** (gotcha #31) — per file, `arbiter_admission.rs`,
`parameters_delivered.rs`, `executor_determinism.rs`, `arbiter_resource.rs`, `dst_campaign.rs` — e il numero dei casi
`compile_fail` si prende col comando, mai da una cifra scritta: `ls crates/kernel/tests/compile_fail/*.rs | wc -l`. Le
cifre di allora stanno in [archivio](archivio/porta-di-qualita-storico.md), sezione «`Parameters::total_vram` e l'arbitro
che ammette e rilascia — Traguardo 5, Task 5».

#### Le code per corsia — Traguardo 5, Task 6, e `Admission::Queued` acquista un produttore

⛔ **Per corsia e non FIFO globale, ed è una misura e non un gusto:** §5.3.1 dice che i numeri di **M-7** restano validi
**come limite superiore** proprio perché la versione specificata tiene l'ordine **per corsia**. Una coda unica riordinata
a ogni rilascio invaliderebbe quella misura, e allora andrebbe rifatta.

⚠️ **E l'ordine per corsia è una proprietà di `promote`, NON dell'arbitro.** `Arbiter::admit` non guarda mai `queues`,
quindi una richiesta che arriva **dopo** scavalca chi è in coda; e `promote` cade sulla corsia successiva quando la sua si
ferma, quindi fra corsie serve la piccola peggiore prima della grande migliore. Entrambe **misurate**, entrambe
**registrate e non prese**: le voci aperte in fondo a questa sezione.

⛔ **`admit` ACCODA ciò che entra nella macchina ma non in questo momento**, invece di rifiutarlo — è il produttore di
`Queued` — e `promote` serve la coda con la stanza che c'è. La costruzione della concessione sta in un aiutante privato,
`Arbiter::issue`, che `admit` e `promote` **condividono**: un secondo posto in cui si costruisce un `Grant` sarebbe il
secondo modo di ottenerne uno, cioè ciò che §5.6 esiste per togliere.

Le sonde delle code, in `crates/kernel/tests/arbiter_admission.rs`:

| Sonda | Cosa tiene |
|---|---|
| `a_request_that_fits_the_machine_but_not_the_moment_is_queued` | il produttore di `Queued`: una richiesta che **non entra adesso ma potrebbe entrare dopo** è accodata e non rifiutata, e dopo il rilascio `promote` la serve col **suo** biglietto |
| `the_queue_promotes_by_lane_and_not_in_arrival_order` | ⛔ **l'asserzione che tiene validi i numeri di M-7 PER `promote`** — e non per l'arbitro intero: vedi la voce aperta sull'ammissione più sotto. Le tre attese arrivano nell'ordine peggiore che ci sia — `Batch`, poi `Interactive`, poi `Realtime` — ed escono **rovesciate**: due entrano nella stanza liberata, `Batch` resta ad aspettare. Una FIFO globale la manderebbe rossa. ⛔ **Tre corsie e non due:** con due corsie sole nessuna sonda del workspace promuoveva mai un'attesa in corsia `Realtime`, e *«prima la corsia migliore»* era provato sulla **seconda** migliore (gotcha **#74**). Tiene anche la **dipendenza di `promote` dalla chiave** `ComputeClass::priority`, dato che il `BTreeMap` percorre le corsie in ordine di chiave: mutata la chiave, muore insieme alle due di `crates/kernel/tests/arbiter_resource.rs` — l'ordine di `promote` viene davvero dalla chiave e non da un elenco scritto due volte |
| `inside_one_lane_the_order_is_the_order_of_arrival` | che la regola qui sopra **non** sia *«un ordine qualsiasi»*: dentro **una** corsia l'ordine è quello d'arrivo |
| `promote_with_no_room_freed_promotes_nothing` | la contro-sonda: `promote` **non** è *«concedi tutto quello che c'è in coda»*. Senza stanza liberata non promuove niente e i libri non si muovono. ⚠️ **Sotto la campagna di mutazione è DOMINATA** — muore solo dove muore anche la sonda dello scavalcamento — e **resta**, perché dice un'altra cosa: è l'unica che chiami `promote` su una macchina che **non guadagna stanza in nessun modo**, né da un rilascio né da una riscossione. ⚠️ **La sua non-ridondanza NON è misurata:** è un'intenzione finché una mutazione non la isola (`E104`) |
| `a_promoted_grant_is_a_grant_like_any_other` | che ciò che esce dalla coda si **rilasci** e renda esattamente la prenotazione — senza, la coda potrebbe emettere concessioni che i libri non hanno mai imparato |
| `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one` | la regola che il doc di `promote` **dichiara** — *«si ferma alla prima richiesta che non entra, dentro una corsia»*. La stanza liberata è **esattamente** quella della piccola, che è **servibile** e **non viene servita** perché la grande le sta davanti. Isolata dalla mutazione dello scavalcamento vero |
| `promote_collects_the_expired_before_it_serves_the_queue` | che anche `promote` riscuota prima di decidere, la proprietà di **ogni** operazione dell'arbitro. ⛔ **Nessuna chiamata a `release` qui dentro:** l'unica cosa che libera la stanza è la riscossione **dentro `promote`** |
| `promote_serves_every_request_that_fits_and_not_just_the_first` | ⚠️ **Nata da un MUTANTE VIVO.** Il doc di `promote` dice *«serve la coda con la stanza che c'è»*, al **plurale**, e un `promote` che si fermasse dopo **una** promozione per corsia sopravviveva a tutte le altre sonde. Due attese da `2_048` nella **stessa** corsia, e l'ordine è asserito su **entrambe** le posizioni: contare non basta (gotcha **#30**) |

Ogni sonda delle code è provata da una mutazione del codice di produzione, e quando una mutazione ne uccide più d'una si
chiede se siano in concorrenza fra loro — la regola sta in [`riferimenti.md`](riferimenti.md). La campagna, con la
risposta sonda per sonda e le due mutazioni vive dichiarate delle voci qui sotto, sta in
[archivio](archivio/porta-di-qualita-storico.md), sezione «Le code per corsia — Traguardo 5, Task 6, e
`Admission::Queued` acquista un produttore».

⚠️ **VOCE APERTA — LO SCAVALCAMENTO CHE `promote` RIFIUTA DENTRO UNA CORSIA, FRA CORSIE LO FA — con un'inversione di
priorità sopra.** Il doc di `promote` giustificava la propria regola d'arresto dicendo che scavalcare *«lascerebbe una
richiesta grande in una corsia affollata aspettare per sempre dietro le piccole»*: ⛔ **fra corsie il codice produce
esattamente quello.** Una corsia che si ferma **cade sulla successiva**, quindi una richiesta piccola in una corsia
**peggiore** viene servita mentre una grande in una corsia **migliore** aspetta — misurato, non dedotto: un'attesa
**`Realtime`** accodata prima di una **`Batch`** resta in coda mentre la `Batch` è promossa.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo a questo file**, che
dice anche **chi la chiude**.

⚖️ **REGISTRATA, NON PRESA, e il comportamento NON è stato cambiato.** È dettato dal Passo 3 del piano ed è una scelta
**work-conserving** difendibile: l'alternativa tiene la macchina ferma per un'attesa che potrebbe non entrare mai. ⛔ **Il
difetto non è il comportamento, è che la conseguenza non stava scritta:** §5.3, §5.3.1 e
`design/02-arbitrato-gpu.md` non dicono niente sull'ordine **fra** corsie. La frase del doc è **circoscritta a dentro una
corsia** e dice per esteso che cosa succede fra corsie. Che l'ordine debba essere work-conserving o rispettare la priorità
è una decisione del **proprietario**; registrata anche nell'errata del piano, come `E50`.

⛔ **E NESSUNA SONDA TIENE QUELLA FRASE DEL DOC — dichiarato, non pinzato.** Fermata l'**intera passata** alla prima
corsia la cui testa non entra, niente va rosso, misurato: il mutante è **vivo**, quindi il giorno in cui il primo ciclo di
orchestrazione scegliesse un altro ordine fra corsie, quel paragrafo diventerebbe **falso in silenzio**. ⚖️ **Non pinzato
apposta, sul merito:** una sonda che congelasse la caduta congelerebbe la politica che **questa stessa voce** chiede al
proprietario, e *«una sonda che va cancellata per prendere una decisione è un voto contro il prenderla»* — è la ragione di
`E39`. La dichiarazione sta **nel sorgente**, accanto al paragrafo; qui e in `E53` è il suo
puntatore.

⚠️ **VOCE APERTA — `admit` NON CONSULTA MAI LA CODA, quindi un ritardatario la scavalca, e questo è ciò che circoscrive
la promessa su M-7.** `admit` legge `held` e `parameters` e basta: se la stanza c'è nel momento in cui viene chiamata, dice
sì, qualunque cosa stia già aspettando — misurato: una richiesta `Batch` nuova è `Granted` all'istante con un biglietto
`Realtime` ancora in coda. ⛔ **Quindi *«l'ordine per corsia è ciò che tiene validi i numeri di M-7»* è vero di
`promote`, non dell'arbitro:** l'ordine d'**ammissione** lo sconfessa.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo a questo file**, che
dice anche **chi la chiude**.

⚖️ **REGISTRATA, NON PRESA, e il comportamento NON è stato cambiato.** ⛔ **Chiuderlo dentro `admit` significherebbe
un'ammissione che RIFIUTA stanza che esiste**, cioè una politica di scheduling che nessun ADR ha deciso; e *quando* si
chiama `promote` rispetto ad `admit` è una questione di **orchestrazione**, quindi di chi costruisce il primo ciclo.
⚠️ **E interagisce direttamente con le due quote permanenti:** una quota permanente accodata non solo non sarà mai
servita, ma può vedersi passare davanti qualunque richiesta che arrivi dopo. Registrata anche nell'errata del piano,
come `E51`.

⛔ **E NESSUNA SONDA TIENE NEMMENO QUESTA FRASE DEL DOC — dichiarato, non pinzato.** Con un ritardatario che si **accoda
dietro** chi aspetta invece di scavalcarlo, niente va rosso, misurato: mutante **vivo**, e se quel primo ciclo decidesse
nell'altro senso il paragrafo del doc di `admit` diventerebbe **falso in silenzio**. ⚖️ **Non pinzato apposta**, stessa
ragione di `E39` e della dichiarazione gemella su `promote`: una sonda che asserisse `Granted` per il ritardatario
congelerebbe la scelta che questa voce chiede al proprietario. Dichiarato **nel sorgente**, accanto al paragrafo; `E53`
è la voce.

⚠️ **VOCE APERTA — `promote` restituisce un `Vec<Promotion>` senza `#[must_use]`, e ignorarlo perde le concessioni.**
Misurato nelle due direzioni: ① **così com'è**, `arbiter.promote(now);` da solo compila e la crate chiamante resta verde
**anche con `-D warnings`** — nessun lint scatta; ② con `#[must_use]` su `promote`, la stessa riga produce
`warning: unused return value of Arbiter::promote that must be used`, `#[warn(unused_must_use)]`, e la compilazione
**riesce lo stesso, exit 0**. Nel caso ① la promozione avviene davvero: le concessioni sono **nei libri** senza che
nessuno le tenga, la VRAM resta prenotata fino alla scadenza della finestra e il chiamante non ha nulla da rilasciare.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo a questo file**, che
dice anche **chi la chiude**.

⚖️ **REGISTRATA, NON PRESA:** `#[must_use]` costa una riga ed è la forma che `Admission` ha già, ma la misura ② dice che
è un **lint** e non una regola che una sonda possa tenere — produce un **avviso** e non un errore, quindi nessun caso
`compile_fail` può pinzarlo. ⛔ **E la via d'uscita apparente è proprio il gotcha #39:** un caso che dichiarasse
`#![deny(unused_must_use)]` per farlo diventare errore proverebbe che **il lint morde dove è dichiarato**, non che il
kernel dichiari l'attributo — la forma esatta dei quattro casi che ridichiarano `#![no_std]`, per cui è dovuto nascere
`gate-attributes.sh`. Metterlo significherebbe aggiungere superficie che nessuna delle due direzioni di §7.1.4 può
provare; lasciarlo fuori significa che la trappola resta. La scelta è del proprietario.

⚠️ **`Queued` è RAGGIUNGIBILE A RUNTIME da un percorso di produzione, e nessuna riga di catalogo lo chiede.** `V4`
resta coperta — *«non compila»* con i due casi negativi, *«compila»* con `an_admission_is_distinguishable_three_ways`,
che le tre varianti le **costruisce** — e che la seconda variante **esca davvero** dalla produzione è più di quanto `V4`
pretenda. ⛔ **Nessuna riga nuova è stata aggiunta al catalogo:** §7.4 è **spec**, e aggiungere una riga è una decisione
del **proprietario** (vincolo globale 7). Registrata nell'errata.

#### La revoca e la grazia che scade — Traguardo 5, Task 7, e le sonde che vivono in `src/`

⛔ **`ask_back` MARCA, non prende, e la spazzata copre DUE scadenze.** Chiedere indietro mette la concessione in
`Revoking { deadline }` e **lascia la prenotazione nei libri** per tutta la grazia: la §5.3 punto 4 dà al titolare quel
tempo per consegnare, e un arbitro che liberasse subito ammetterebbe un secondo consumatore sulla VRAM che il primo sta
ancora usando. A riscuotere è `collect_expired`, che guarda `expires_at` **e** `deadline` — la finestra di validità
dichiarata dal richiedente e la grazia concessa da una revoca.

⛔ **E MARCA IN DUE PASSATE.** Una passata in **sola lettura** somma il recuperabile fra i candidati ammissibili, e
**solo se copre il bisogno** parte quella che marca; se non copre, **non si marca nulla** e la risposta è `Mib::ZERO`.
Il perché, riprodotto misurando prima di correggerlo: marcando mentre cammina, su una macchina `8_192` con un `Batch`
prelazionabile da `2_048` e un `Batch` non prelazionabile da `6_144`, un `ask_back(4_096, …)` **condannava** il primo e
rispondeva `2_048` — **uno sfrattato e nessuno seduto**, il danno che il doc della funzione dichiara di evitare
(*«evicts two jobs to seat one»*) preso dall'altra strada. ⚖️ **Si è corretto invece di registrarlo aperto, al
contrario di `E50`/`E51`:** quelle sono **politiche** senza risposta giusta, e pinzarle sarebbe un voto contro il
deciderle (gotcha **#73**); questa è un'operazione che **muta lo stato e riporta fallimento lasciando la mutazione lì**,
il degrado silenzioso che **ADR-0005** e **ADR-0019** vietano. ⛔ **Nessuno stato nuovo e nessun tipo nuovo:** il
criterio di ammissibilità è scritto **una volta sola** — una chiusura `askable` che entrambe le passate interrogano — e
una copia verbatim del predicato sarebbe il difetto che questa forma esiste per non introdurre. ⚠️ **Chiusura e non
metodo, e la ragione è una misura:** installato come `Held::askable_by`, `cargo build --locked --workspace` stampa
**tre** avvisi `dead_code` invece dei due accettati dal proprietario — il terzo è
`` method `askable_by` is never used ``. ⚠️ **[C-S2-1]** Voce `E69`.

⚖️ **DENTRO UNA CORSIA LA VITTIMA È LA PIÙ VECCHIA, E NESSUN DOCUMENTO LO DECIDE — dichiarata, non pinzata.** La passata
che marca percorre `held`, chiavato per `GrantId`, quindi incontra le concessioni di una corsia nell'ordine in cui sono
state emesse: con un `Batch` da `4_096`, uno da `512` e un bisogno di `512` marca il `4_096`. L'alternativa — **la più
piccola che basta** — è altrettanto difendibile, e §5.3, §5.3.1 e design/02 tacciono: la tabella delle corsie decide
l'ordine **fra** corsie, non **dentro** una. Il mutante (`values_mut().rev()`, riga **13** della campagna) è **vivo** e
sta **nel doc di `ask_back`, accanto alla frase**, così il giorno in cui qualcuno cambia l'ordine quella frase diventa
falsa senza che nulla lo dica. ⛔ **Nessuna sonda la tiene, per la ragione di `E39`:** una sonda che asserisse *«la più
vecchia»* congelerebbe la scelta, e una sonda da cancellare per prendere una decisione è un voto contro il prenderla.
Voce `E70`, forma di `E50`/`E51`/`E53`.

⛔ **LA PASSATA CHE MARCA ATTRAVERSA TUTTE LE CORSIE.** Quella di sola lettura somma **tutte** le corsie, quindi può
promettere `6_144`; se quella che marca si fermasse dopo la prima, risponderebbe `2_048` **col titolare di quella
corsia già condannato** — uno sfrattato e nessuno seduto. La tiene
`asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough`: senza di lei `lanes.iter().rev()` ridotto a
`.take(1)` sopravviveva all'**intero workspace** (riga **14** della campagna). ⚖️ **Non è una politica aperta**, quindi
l'argomento di `E39`/`E70` non si applica: c'è una risposta giusta. Voce `E75`.

⛔ **L'INSIEME DELLE CORSIE PERCORSE È QUELLO DEI CANDIDATI, PER COSTRUZIONE.** `lanes` è costruita con un
`filter(|held| askable(held).is_some())` nella stessa catena, quindi *«la passata di sopra ha già sommato lo stesso
insieme»* è una costruzione e non una promessa (`E76`). Il filtro non ha comportamento — toglierlo lascia il workspace
verde, mutante vivo e **garantito** (riga **15** della campagna, voce `E83`) — e **resta** perché è ciò che rende la
frase vera. ⛔ **E non sostituisce la sonda della riga 14:** la struttura toglie la distanza fra i due insiemi, la sonda
uccide il mutante che **salta una corsia**. Sono due difetti diversi e nessuno copre l'altro.

⛔ **IL DOC DI `ReleaseError::UnknownGrant`: DA QUESTO COMPITO LE CAUSE SONO TRE.** Con la riscossione forzata dentro
`collect_expired`, una concessione **chiesta indietro** la cui **grazia** è scaduta esce dai libri come una scaduta di
finestra, quindi un `release` su di essa risponde `UnknownGrant` per una terza ragione: chiesta indietro a `0` con
grazia `500` e rilasciata a `500` esatti → `Err(UnknownGrant)`, a `499` → `Ok(Mib(4096))`. Con tre cause una sola
variante `Expired` non le separa più, quindi la **forma** del rimedio è parte della decisione del proprietario su `E30`.
⛔ **Nessuna sonda pinza quei tre valori**, per la ragione di `E39`. Voce `E72`. ⚠️ **[C-S2-2]**

⛔ **TREDICI DELLE QUATTORDICI SONDE DEL TASK 7 VIVONO IN `crates/kernel/src/arbiter/mod.rs`, in un
`#[cfg(test)] mod tests`.** `ask_back` è `pub(crate)` — decisione del proprietario, perché il suo unico chiamante è
l'ammissione sotto policy LOCALE — e un `pub(crate)` **non si vede da un test d'integrazione**, che è una crate a sé:
chiamato da `crates/kernel/tests/arbiter_admission.rs` esce `` error[E0624]: method `ask_back` is private ``. Il
precedente e la forma della giustificazione sono `crates/platform/src/rng.rs`. ⛔ **Solo la privatezza sposta una sonda
lì dentro:** `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` non chiama `ask_back` e **resta** in
`tests/arbiter_admission.rs`, accanto alle altre sonde della finestra di validità. Il commento di modulo di
`arbiter_admission.rs` enumera anche la revoca e dice **dove** stanno le sonde che chiamano `ask_back`, e perché; la
cifra vive lì sola, e la sonda del Task 7 rimasta nel file vi **rimanda** invece di ripeterla (gotcha **#31** e
**#68**, `E77`). Allo stesso modo il doc di `ReleaseError` non conta i chiamanti di `release`: l'argomento ha bisogno di
*«nessun consumatore di produzione»*, la metà che non marcisce, e il riconteggio vive in **un posto solo**, `E77` del
piano (`E85`).

⚠️ **UN'ASSERZIONE CHE NON PUÒ FALLIRE SOTTO UNA MUTAZIONE DI `ask_back` — E CHE SOTTO DUE RIGHE DELLA CAMPAGNA È
L'UNICA CHE SCATTA.** In `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`,
l'`assert_eq!(arbiter.allocated(), Mib::new(8_192))` è vera per qualunque implementazione di **quella funzione**:
`ask_back` non toglie mai dai libri — solo `collect_expired` lo fa, e allo `ORIGIN` con finestre `LONG` non c'è niente
da riscuotere. Ma sotto **5b** e **5d** — le due righe che mutano `collect_expired` invece di `ask_back` — le prime due
asserzioni **passano** e scatta **solo questa**: su quelle due righe è lei a uccidere la sonda, da sola (misurato
leggendo **quale** asserzione va in panico, non che la sonda fosse rossa). ⛔ **Il rimedio: non si cancella** (è la
specie di `E37`), e non serve invocare un giorno in cui `ask_back` tocchi i libri: la spazzata che chiama per prima lo
fa già. Voci `E79`, `E81`, `E82`. La lezione — un'esclusività misurata su un campione che esclude la classe portante si
legge come una garanzia — è il gotcha **#75** di [`HANDOFF.md`](HANDOFF.md).

⛔ **QUATTORDICI SONDE, di cui SEI dettate dal piano e OTTO no.** Le otto in più esistono perché una regola scritta in un
commento e tenuta da niente è un'intenzione (gotcha **#42**) ⚠️ **[C-S2-4]**, perché **due** delle mutazioni dettate non
uccidevano niente senza di esse, e perché la revisione ha trovato confini, operazioni e promesse di doc tenuti da
nessuno.

| Sonda | Dove | Cosa tiene |
|---|---|---|
| `asking_a_grant_back_marks_it_and_does_not_free_it_yet` | `src` | che chiedere indietro **marchi** e non prenda: `revoking()` sale a uno e `allocated()` non si muove |
| `a_grace_that_ran_out_returns_the_reservation_to_the_budget` | `src` | che oltre la scadenza della grazia la prenotazione **torni** nel budget — la metà arbitro di `Forzata` (§6.5 del disegno) |
| `a_non_preemptible_grant_is_never_asked_back` | `src` | che un non prelazionabile non si chieda indietro. ⚠️ **Il residente è in corsia `Batch` e non in `Realtime`:** in `Realtime` la guardia sulla **corsia** lo scarterebbe prima che la sua prelazionabilità fosse guardata, e il meccanismo che il nome promette non girerebbe mai (gotcha **#74**). In `Batch`, strettamente sotto la corsia che chiede, a salvarlo può essere **solo** la guardia sulla grazia |
| `only_lanes_below_the_asking_one_are_asked_back` | `src` | la contro-sonda per l'altra strada: un lavoro `Realtime` **prelazionabile** non viene sfrattato per uno `Interactive` |
| `asking_back_stops_as_soon_as_the_need_is_covered` | `src` | che si fermi appena la stanza basta: *«ha fatto spazio»* è soddisfatto anche da chi revoca tutto e sfratta due lavori per sederne uno |
| `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` | `tests/` | la contro-sonda del ramo `_` della spazzata. Asserisce `Queued` e non `Refused`: dal Task 6 *«entra nella macchina ma non in questo momento»* è `Queued` |
| `a_grant_inside_its_grace_keeps_its_reservation` | `src` | ⚠️ **Non dettata.** Che dentro la grazia la prenotazione resti. `allocated()` non riscuote, e un'asserzione scritta subito dopo l'`ask_back` non passa da nessuna spazzata capace di mettere alla prova la grazia: la spazzata che `ask_back` porta con sé è la sua **prima istruzione**, e corre **prima** che `ask_back` marchi il residente `Revoking`. Questa fa spazzare qualcun altro — un `admit` a `499` su una grazia che scade a `500` — mentre la grazia corre ancora. ⚠️ **Lo scoping conta:** dopo un `ask_back` che trovi il residente **già** `Revoking` la spazzata mette eccome alla prova la grazia, ed è ciò che fa `asking_back_twice_does_not_buy_the_room_twice`. Voce `E64` |
| `the_grace_runs_out_at_the_instant_of_its_deadline` | `src` | ⚠️ **Non dettata**, la specie di `E29` sulla **seconda** scadenza: le due sonde qui sopra chiedono a `501` e a `499`, e senza di lei `deadline > now` mutato in `>=` sopravviveva all'intera suite (riga **5c**). Dichiara anche **quale** semantica è scelta: grazia **semiaperta**, `[chiesta, scadenza)`, la stessa regola della finestra di validità |
| `asking_back_twice_does_not_buy_the_room_twice` | `src` | ⚠️ **Non dettata.** È ciò che la guardia su `activity` compra **da sola**: una seconda passata che rimarcasse una concessione già in uscita conterebbe la sua prenotazione **due volte** in `covered` — stanza che l'arbitro non ha — e le sposterebbe la scadenza più in là, regalando al titolare **più** tempo per essere stato chiamato prima |
| `ask_back_collects_the_expired_before_it_marks` | `src` | ⚠️ **Non dettata.** *«L'arbitro riscuote prima di decidere»* è una proprietà di **ogni** operazione, e con `ask_back` sono **quattro**: questa è l'unica sonda che ne esercita la riga in `ask_back` (riga **9**). Nessun `release` qui dentro, come in `promote_collects_the_expired_before_it_serves_the_queue` |
| `asking_back_takes_the_worst_lane_first` | `src` | ⚠️ **Non dettata.** *«Prima la corsia peggiore, la cosa più economica da interrompere»* è una frase che il doc di `ask_back` **afferma** e che la tabella delle corsie di [design/02](design/02-arbitrato-gpu.md) fonda — `interattivo` è *«servita prima di `batch`»*, `batch` *«può attendere indefinitamente»*. ⚠️ **Le due prenotazioni sono DIVERSE apposta:** `revoking()` conta e non nomina, quindi due vittime della stessa taglia sarebbero indistinguibili — `2_048` in `Batch` contro `4_096` in `Interactive`, ed è la **risposta** a dire quale è stata presa |
| `a_grant_in_the_asking_lane_itself_is_not_asked_back` | `src` | ⚠️ **Non dettata** (`E71`). Il confine: il residente nella **stessa** corsia dell'argomento `below`; senza di lei `lane <= below` mutato in `lane < below` sopravviveva all'**intero workspace**. ⛔ **Ed è il caso che il Task 8 produce per primo:** l'ammissione chiede indietro **sotto la propria corsia**, quindi un **pari** è la prima cosa che `ask_back` si vede passare, e sfrattare un pari per un pari è ciò che *«only lanes BELOW»* esclude |
| `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need` | `src` | ⚠️ **Non dettata** (`E69`). Uno sfrattato e nessuno seduto: macchina `8_192`, un `Batch` prelazionabile da `2_048` e un `Batch` **non** prelazionabile da `6_144`; `2_048` è tutto il recuperabile e non siede un `4_096`. ⚠️ **Il non prelazionabile sta in `Batch` apposta:** sotto `Realtime` lo scarterebbe la guardia sulla **corsia** e la sonda parlerebbe di corsie invece che di capienza (gotcha **#74**) |
| `asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough` | `src` | ⚠️ **Non dettata** (`E75`). Che la passata che marca **arrivi in fondo alle corsie**: macchina `8_192`, `Interactive 4_096` e `Batch 2_048` entrambi prelazionabili, `below = Realtime`, `ask_back(4_096, …)` → **`6_144`** e `revoking() == 2`. ⚠️ **Lo scenario è quel caso e non uno più largo:** `Interactive 4_096` copre il bisogno **da sola** — a non bastare è la corsia **peggiore**, non «nessuna delle due» — ed è per questo che la risposta **eccede** (`E86`). ⚠️ **E pinza una seconda cosa: la marcatura può ECCEDERE** — `6_144` per un bisogno di `4_096` — cioè si ferma alla prima concessione che porta **oltre** la linea e non ne cerca una che ci **atterri** (direzione negativa: riga **10b**) |

⛔ **LA CAMPAGNA DI MUTAZIONI: una mutazione alla volta, ciascuna provata entrata col conteggio delle occorrenze,
COMPILATA in un passo separato da quello che la esegue, e revocata da una copia byte-esatta presa prima**, con `cmp`
identico e `git diff` vuoto (mai `git checkout --`, gotcha **#48**; nessun `sed -i` — il rimpiazzo è byte-a-byte in
Python, e i **CR** si contano prima e dopo su ogni sorgente toccato). La tabella delle mutazioni, con le righe che questa sezione cita per numero, sta in
[archivio](archivio/porta-di-qualita-storico.md), sezione «La revoca e la grazia che scade — Traguardo 5, Task 7, e le
sonde che vivono in `src/`».

⛔ **QUATTRO SONDE SONO DOMINATE SOTTO LA CAMPAGNA, E RESTANO TUTTE E QUATTRO** —
`asking_a_grant_back_marks_it_and_does_not_free_it_yet`, `a_grace_that_ran_out_returns_the_reservation_to_the_budget`,
`a_grant_inside_its_grace_keeps_its_reservation` e `a_non_preemptible_grant_is_never_asked_back`: «dominata» non vuol
dire «superflua». ① Ciascuna dice una cosa che nessun'altra dice a voce alta, ed è ciò che si legge quando una va rossa:
*«marca e non prende»*, *«poi riscuote»*, *«dentro la grazia non riscuote»*, *«quello che non si può chiedere indietro
non si tocca»*. ② ⚠️ **Ma la loro non-ridondanza NON è misurata**, ed è registrata per quello che è — un'intenzione
finché una mutazione non le isola. La regola, nata dal Task 3 del Traguardo 4: quando una mutazione ne uccide due si
cerca **una terza che lasci passare la prima** — qui ha prodotto le righe **3b**, **2c** e **10b**. ⛔ **Per
`a_non_preemptible_grant_is_never_asked_back` la candidata isolante è stata cercata e NON trovata:** lei e
`asking_back_marks_nothing_…` differiscono per la **presenza di un secondo candidato recuperabile**, e nessuna mutazione
della guardia sulla grazia distingue i due casi — riscrivere lo scenario dell'una per farle divergere sarebbe piegare la
sonda alla campagna. Per le altre tre non se n'è trovata nessuna. La tabella di dominanza sta in archivio, nella stessa
sezione.

⛔ **Il corpo di `ask_back` DIVERGE dal piano, e la divergenza è una misura e non un gusto.** Sulla forma dettata la
direzione *«una concessione non prelazionabile non si chiede indietro»* era tenuta da **due guardie che si mascherano a
vicenda**: cancellare l'una la lascia coperta dall'altra (`D1`), e rompere l'altra non è nemmeno raggiungibile (`D2`).
Una direzione che nessuna mutazione riesce a mostrare non è provata, è **assunta**. Nella forma scritta le guardie sono
**tre** — corsia, «già in uscita», grazia — e vivono in **un posto solo**, la chiusura `askable` che entrambe le passate
di `E69` interrogano, ma restano **separate**, ciascuna con la **propria sonda** (righe **7** e **8** per le ultime due).
La riga **8** non ha più un uccisore solo: la sonda di `E69` porta anch'essa un residente non prelazionabile. Voci
`E62` e `E73`.

⚠️ **NESSUNA RIGA NUOVA AL CATALOGO.** §7.4 è **spec**, e aggiungere una riga è una decisione del **proprietario**
(vincolo globale 7). `I2 · §5.3` resta coperta come al Task 4 — *«non rappresentabile»* dal caso negativo,
*«costruibile dove è lecito»* da `a_revocation_is_constructible_on_the_preemptible_side` — e il Task 7 aggiunge che
l'arbitro **non ci prova nemmeno** a runtime, che è più di quanto la riga pretenda.

#### Le due policy VRAM — Traguardo 5, Task 8, e `V3` si copre a metà

⛔ **DUE OGGETTI CON UN'INTERFACCIA, E LA DIFFERENZA È UNA DECISIONE SOLA DENTRO L'AMMISSIONE.**
`crates/kernel/src/arbiter/policy.rs` porta il tratto `MakeRoom` con **una** domanda — *«una richiesta non entra. Si può
fare spazio?»* — e i due oggetti che rispondono: `RemotePolicy` **no**, `LocalPolicy` **sì**. È il punto in cui
ADR-0006 dice che sarebbe finito il condizionale sull'origine dell'inferenza, e non c'è: l'`if` in `Arbiter::admit`
chiede **alla policy**, non all'origine, e ne esiste **uno solo** in tutta la crate.

⛔ **`LocalPolicy` CHIAMA `ask_back` COL BISOGNO ESATTO** (`allocated + asked - ceiling`) e con la **propria** corsia come
confine; poi, in entrambe le policy, la richiesta si **accoda**: la stanza non è libera finché il titolare non consegna,
e il ritorno di `ask_back` è **deliberatamente non letto** — agire su quel numero vorrebbe dire sedere un secondo
consumatore sulla VRAM che il primo sta ancora usando, ciò che la grazia della §5.3 punto 4 esiste per impedire.
*«Sfrattare un residente»* **è** *«revocare una concessione prelazionabile»*, il meccanismo del Task 7, quindi le due
policy si provano con concessioni sintetiche dichiarate dal banco, senza nessun modello. `enqueue` è estratto da
`admit` — non era nel piano: il corpo dettato non compilava — senza cambiare comportamento (stessi campi, stesso ordine,
stesso contatore), e il corpo estratto è **vivo**, tenuto dalle sonde del Task 6 (righe **11a** e **11b** della
campagna). Voce `E87`.

⛔ **L'INVERSIONE DI PRIORITÀ È RAGGIUNGIBILE IN PRODUZIONE.** `admit` riscuote e concede subito se c'è posto, **senza
guardare le code** (`E51`). Sotto `VramPolicy::Local` c'è una stanza da rubare: `LocalPolicy` chiede indietro un
residente **per** un biglietto in coda, la spazzata libera quella riserva alla scadenza della grazia, e il primo `admit`
**diretto** che passa si siede sulla stanza fatta per un altro, mentre il biglietto per cui è stata fatta resta nella
sua corsia. ✅ **Misurato nelle DUE direzioni**, su una sonda usa-e-getta cancellata subito dopo. **LOCAL:** residente
`Batch` da 4_096 chiesto indietro per un biglietto `Interactive` in coda, poi a `501` una richiesta `Batch` **nuova** da
4_096 → **`Granted`**, il `promote` che segue torna **vuoto**, e `queued()` è ancora **1**. **REMOTE**, dove nessuno
viene chiesto indietro: lo stesso ritardatario è **`Queued`** e `queued()` è **2** — non si era liberato niente e non
c'era niente da prendere. ⛔ **NESSUNA SONDA LO TIENE, ed è dichiarato invece che lasciato scoprire** — la forma di
`E50`/`E51`/`E53`: pinzarlo congelerebbe la scelta che quelle voci mettono davanti al proprietario, e una sonda che va
cancellata per prendere una decisione è un voto contro il prenderla. ⚖️ **Il chiusore è chi costruisce il primo ciclo
di orchestrazione**, ed è una decisione di **orchestrazione**; il costo di lasciarla aperta si paga **in produzione** e
non sulla carta. Il fatto è enunciato anche accanto ad `admit`, nel sorgente. Voce `E100`.

| Sonda — `crates/kernel/tests/arbiter_policy.rs` | Cosa tiene |
|---|---|
| `the_remote_policy_does_not_make_room_it_queues` | il **default** di ADR-0006: la macchina piena, una richiesta che non entra, e `revoking()` a **zero**. Nessuno viene chiesto indietro |
| `the_local_policy_asks_the_lower_lanes_back` | lo **stesso identico scenario** con l'altro oggetto: `revoking()` a **uno**, e la risposta resta `Queued` — chiedere **marca**, non prende |
| `under_the_local_policy_the_queued_request_is_served_past_the_grace` | che la marcatura **porti da qualche parte**: oltre la grazia il biglietto è servito, con il **suo** `TicketId`, e i libri restano a una concessione sola |
| `under_the_remote_policy_the_same_clock_advance_serves_nobody` | la contro-sonda della precedente: **stesso avanzamento d'orologio**, e non si libera niente perché non è stato chiesto niente |
| `each_policy_names_itself` | il nome, che la transizione giornalata della §5.4 dovrà scrivere — letto **due volte**, dall'enum e **attraverso l'arbitro**. La seconda metà è la sola cosa che tiene *«l'arbitro ha conservato la policy con cui è stato costruito»* |
| `a_partly_full_machine_asks_back_the_need_and_not_the_whole_request` | il **primo** dei due argomenti che `admit` calcola, nella direzione *«scatta dove deve»*: macchina **parzialmente** piena — `ceiling` 4_096, `allocated()` 3_072, `asked` 2_048, `needed` 1_024, tutti diversi — quindi `needed` è distinguibile da `asked`. Voce `E97` |
| `the_admission_asks_back_below_its_own_lane_and_spares_a_peer` | il **secondo** argomento, la corsia, nella direzione *«non scatta dove non deve»*: un pari `Interactive` **prelazionabile** non viene sfrattato per un `Interactive`, e la policy aveva detto **sì** — a fermare l'arbitro è il confine, non la policy. Voce `E97`. ⚠️ **Le due righe sono DUE regole, non le due direzioni di una.** Le direzioni opposte stanno **altrove**: per lo **scarto** in `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`, per la **corsia** in `the_local_policy_asks_the_lower_lanes_back` (cablaggio) e in `only_lanes_below_the_asking_one_are_asked_back` (meccanismo). Voce `E110` |

⚠️ **NESSUN `Preemption::Never` IN QUESTO BANCO:** i residenti `Realtime` sono prelazionabili come ogni altro residente
del file, e a tenerli fuori dai recuperabili è la **corsia** (`held.lane <= below`), non la prelazionabilità. Non è una
perdita di copertura: la variante `Never` è esercitata in `crates/kernel/tests/arbiter_admission.rs`, nel
`#[cfg(test)] mod tests` del `lib`, in `crates/kernel/tests/arbiter_resource.rs` e in due casi `compile_fail`.
⚠️ **[C-S2-6]** Voce `E105`.

⛔ **`Arbiter::policy()` È `pub`, QUINDI `dead_code` TACE** — il gotcha **#46** dal verso sbagliato. Il suo consumatore
**vero** sono le due righe di `each_policy_names_itself` che leggono il nome attraverso l'arbitro: con `policy()` che
restituisce un `VramPolicy::Remote(RemotePolicy)` fresco invece di quello conservato, quella sonda è **rossa** (riga
**9**).

⛔ **`V3` SI COPRE A METÀ QUI, E IL CASO NEGATIVO NON È QUELLO CHE IL PIANO DETTAVA.** La contro-sonda di catalogo è
doppia e la sua seconda metà è del **Task 9**, che la porta e chiude la riga. Il numeratore del blocco C **non si scrive
qui**: si riconta sulla cella del blocco C in fondo a questo file, e un rimando non può marcire (gotcha **#68**,
`E103`). Il piano scriveva `VramPolicy::Remote(..) | VramPolicy::Local(..)`, che prova che `VramPolicy` non implementa
`BitOr`. ✅ **Misurate entrambe le forme sotto la mutazione che la regola teme davvero** — un `Arbiter::new` che
**accetti** due policy:

| Forma del caso | Sotto la mutazione «`new` accetta due policy» | Specie |
|---|---|---|
| `BitOr` fra due policy — **la forma dettata** | ⛔ resta **`ok`**: non nomina `Arbiter::new`, quindi la regressione le è **invisibile** | nessuna: non scatta |
| **arità** — tre argomenti a `Arbiter::new` | ✅ passa a **`error`**: il caso comincia a compilare e `trybuild` lo dice **fuori dall'oracolo** | la forte, gotcha **#42** |

⛔ **Scelta la seconda — il caso `two_policies_at_once.rs` — e la prima NON è tenuta accanto:** un caso che non può
scattare per la ragione per cui esiste è una guardia che si legge come tale e non lo è. È la stessa forma con cui `E23`
chiuse `V2` col caso di arità `admission_without_profile.rs`. Voce `E89`. ⚠️ **E il limite è dichiarato, e MISURATO:**
il caso pinza l'**arità di `new`**, non l'assenza di ogni strada verso due policy. Con un secondo costruttore —
`pub const fn new_with_two(parameters, a, _b)` — il caso resta **`ok`** e l'intera suite resta verde. A chiudere quella
strada è la revisione, non il compilatore.

⛔ **TRE CASI `compile_fail` HANNO L'ORACOLO CHE PINZA UN NUMERO DI RIGA** — `admission_has_no_is_granted.rs` (riga
**26**), `admission_reads_cold_start.rs` (**27**), `admission_without_profile.rs` (**41**) — e si modificano **a parità
di righe**, l'argomento nuovo dentro la chiamata esistente. Che nessun oracolo sia stato rigenerato si verifica con
`git status --porcelain` e non con `git diff`, che i non tracciati non li vede (`E3`, `E11`). Quanti siti di
`Arbiter::new`, e in quanti file, vive in un posto solo: la voce `E101` del piano (gotcha **#68**, `E112`).

⚠️ **REGISTRATA, NON PRESA — LA POLICY È UN SECONDO VALORE CONSEGNATO, E LA §2.8.2 NE PARLA AL SINGOLARE.** La regola 1
di §2.8.2 dice che il kernel *«riceve alla costruzione **un valore** che porta i parametri risolti»*, e la conseguenza
che fonda `V3` è scritta così: *«se **il valore consegnato** porta **una** policy, "due policy attive" non è
rappresentabile»*. ADR-0034 elenca *«quale policy VRAM è attiva»* fra i parametri di quel valore. Il piano consegna
invece la policy come **secondo argomento** di `Arbiter::new`, accanto a `Parameters`. ⚖️ **Non è un indebolimento** —
l'arbitro la riceve comunque alla costruzione, non legge nessuna configurazione, e `VramPolicy`, essendo un enum, ne
porta **una** comunque — ma i due testi non dicono la stessa cosa, e la scelta è del proprietario: ① **lasciarlo
com'è**, e la §2.8.2 va letta come *«i valori consegnati»*; ② **spostare la policy dentro `Parameters`**, che è il
testo alla lettera e costa un campo su un tipo che la §2.8 pinza, più i **diciannove** chiamanti di `Parameters::new`
che `E18` ha contato. ⚠️ **[C-S2-7]** Voce `E94`.

⚠️ **LE REGOLE DI UNA TABELLA DI MUTAZIONI.** Le righe di isolamento **si nominano, non si contano dal fondo** (`E110`);
il conteggio delle righe non si scrive — la tabella **è** la misura, e un rimando non può marcire (gotcha **#68**,
`E102`); un banco che cambia **invalida ogni cella** misurata su di esso (gotcha **#31**); e l'esclusività si dichiara
**sempre col proprio perimetro** — *«sola nel banco»* non è *«sola nel workspace»* — ed è un'affermazione **sull'insieme
delle sonde**, non sulla mutazione: invecchia quando l'insieme cresce, e chi aggiunge una sonda non apre la tabella di
un compito precedente.

⚠️ **LA CAMPAGNA DEL TASK 8 È UN VERBALE DEL BANCO DI SETTE** (2026-08-20, finding AUD-023): il banco oggi è più grande
— `grep -c '^#\[test\]' crates/kernel/tests/arbiter_policy.rs` — e le cifre **non descrivono il workspace di oggi**. Le
mutazioni **9** e **10** non sono più sole: cade con loro anche `a_policy_transition_writes_its_intent_before_its_outcome`.
Le righe **14** e **15** non sono state riprovate, e la loro esclusività è quella del banco di sette. La tabella sta in
[archivio](archivio/porta-di-qualita-storico.md), sezione «Le due policy VRAM — Traguardo 5, Task 8, e `V3` si copre a
metà».

⚠️ **UNA SONDA È DOMINATA DENTRO QUESTA CAMPAGNA, E NON SI CANCELLA:** ogni mutante che uccide
`the_local_policy_asks_the_lower_lanes_back` uccide anche `a_partly_full_machine_asks_back_the_need_and_not_the_whole_request`.
La campagna è un **campione**, non una dimostrazione, e le due sonde dicono cose diverse: quella dominata è la coppia
esatta di `the_remote_policy_does_not_make_room_it_queues` sullo **stesso scenario**, che è la forma con cui ADR-0006
chiede che le due policy si distinguano. Il perimetro, sul banco di sette: sole nell'**intero workspace** la **9**, la
**10**, la **14** e la **15**; la **5** e la **6** sole **dentro `tests/arbiter_policy.rs`**. Voce `E104`.

⚠️ **LE ASSERZIONI FINALI SU `allocated()` SONO DOMINATE DENTRO LA PROPRIA SONDA, E RESTANO.** Non sono vacue — isolate
scattano — ma a piena forza un'asserzione **sopra** di esse scatta sempre per prima: `revoking()`,
`matches!(outcome, Queued(_))`, `promoted.len()` o il `panic!("queued")` del `let … else`. ⛔ **Il rimedio non è
cancellarle**, che è la specie di `E37` e `E79`: dichiarano l'intento che il nome della sonda porta — i libri non si sono
mossi — e la ragione è scritta **una volta sola**, accanto alla prima di esse, con le altre che ci rimandano (`E93`).
⛔ **Una NON è dominata:** la precondizione `assert_eq!(allocated(), Mib::new(3_072), "PARTLY full")` di
`a_partly_full_machine_…`, la sola asserzione su `allocated()` del banco che decide un esito, sotto la riga **12**
(`E108`). Le conta `grep -c "arbiter.allocated()" crates/kernel/tests/arbiter_policy.rs`.

#### La transizione fra le due policy — Traguardo 5, Task 9, e `V3` si chiude

⛔ **INTENTO, POI L'EFFETTO, POI L'ESITO, E L'ORDINE È `V6` E NON PULIZIA.** `Arbiter::set_policy` scrive l'**intento**
nel giornale, **poi** scambia l'oggetto, **poi** scrive l'**esito**. Cambiare policy ha effetti veri sul mondo —
sfratti, ricariche — e niente si esegue prima che l'intento sia **durevole**. Una transizione tagliata a metà lascia un
passo **in dubbio**, riconciliabile come ogni altro (§4.3): è la **proprietà DST numero 4**, che questo compito rende
scrivibile al Task 12.

⛔ **L'ASSERZIONE STA SULL'ARCHIVIO E NON SULLA POLICY, ed è la ragione per cui le sonde sono cinque e non una.** *«Dopo
la transizione la policy è l'altra»* è **verde con zero record scritti**, e `V6` è esattamente l'affermazione che niente
accade prima che l'intento sia durevole. Le cinque, in `crates/kernel/tests/arbiter_policy.rs`: due leggono i record
**decodificati** dall'archivio, una per direzione — `a_policy_transition_writes_its_intent_before_its_outcome` e
`a_transition_names_the_policy_it_moves_to`; `a_refused_intent_leaves_the_policy_where_it_was` prova che un giornale che
**rifiuta l'intento** lascia la policy dov'era; `a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt`
taglia la transizione **fra intento ed esito** con `CrashingJournal::falling_at(1)` e pretende un passo in dubbio
risolto `RunAgain`; `without_a_crash_a_transition_leaves_no_step_in_doubt` è la sua **contro-sonda** — senza schianto,
**nessun** dubbio.

⛔ **`reason` PORTA IL NOME DELLA POLICY, ED È PER QUESTO CHE `MakeRoom::name` ESISTE.** Un record che dicesse solo
*«policy transition»* renderebbe le due direzioni **indistinguibili nell'archivio**, e l'archivio è l'unica cosa che
sopravvive. ⛔ **E una sola direzione non basterebbe:** una costante `"local"` cablata sopravviverebbe a una sonda che
asserisce `"local"`. È il gotcha **#74** — la sonda deve nominare anche l'elemento per cui la regola esiste — e qui le
direzioni sono **due**: `transition_record` che cabla `"local"` lo uccide `a_transition_names_the_policy_it_moves_to`,
che cabla `"remote"` lo uccide `a_policy_transition_writes_its_intent_before_its_outcome` (righe **6** e **7** della
campagna, uccisori distinti).

⛔ **LA FIRMA NON PRENDE `now`, E DIVERGE DAL PIANO,** che dettava `set_policy(&mut self, policy, step, journal, now)`
con dentro un `let _ = now;`. `set_policy` **non tocca i libri** — non legge né scrive `held` né `queues` — quindi non
ha scadute da riscuotere, e segue il precedente di `allocated()`, che dichiara di non riscuotere nulla. Un parametro
ignorato è la **superficie morta** che questa crate ha tolto a `Record::encode` e rifiutato a `Ipc::accept`. ⚠️ **Il
giorno in cui la transizione toccherà i libri l'argomento torna, e torna come ERRORE DI COMPILAZIONE** a ogni sito di
chiamata, non come regressione silenziosa. Voce `E113`.

⛔ **`set_policy` LEGGE `self.policy`, IL CAMPO:** dentro l'`impl` passare dal proprio getter non compra niente, e
sarebbe codice peggiore scritto per far tornare una frase. Ciò che compra `policy()` restano i **banchi** ⚠️
**[C-S2-9]**: `each_policy_names_itself`, `a_policy_transition_writes_its_intent_before_its_outcome` e
`a_refused_intent_leaves_the_policy_where_it_was`, che leggono `arbiter.policy().name()` **da fuori la crate**. ⚠️ **Sono
nominate una per una e non contate come insieme:** delle cinque sonde della transizione, `policy()` lo leggono le
ultime due — gotcha **#67**. Voce `E114`.

⛔ **LA CAMPAGNA DI MUTAZIONE MISURA LE RIGHE CHE HA, NON LO SPAZIO DELLE MUTAZIONI POSSIBILI:** nessun quantificatore
del tipo *«nessun mutante vivo»* (gotcha **#76**). Il metodo è quello della campagna del Task 7 (mai risostituendo
all'indietro, gotcha **#48**); il perimetro — il solo banco o l'**intero workspace** — lo dicono le colonne. Che cosa
tiene il record di transizione, campo per campo:

| Del record | Lo tiene | Riga |
|---|---|---|
| l'ordine: intento **prima** dell'esito | `a_policy_transition_writes_its_intent_before_its_outcome` | **1** |
| l'ordine: assegnazione **dopo** l'intento | `a_refused_intent_leaves_the_policy_where_it_was`, sola nell'intero workspace | **2** |
| la classe `EffectClass::Idempotent`, sull'asserzione `Resolution::RunAgain` — ciò che la rende **argomentata e tenuta**, e non scelta | `a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt`, sola nell'intero workspace | **3** |
| `reason`, nelle due direzioni | le due sonde dell'archivio, vedi sopra | **6**, **7** |
| `trust: Trust::Instruction` e `payload: Vec::new()` — il doc di `set_policy` dice che nessun byte esterno raggiunge il record: un **fatto del contratto**, non una preferenza | `a_policy_transition_writes_its_intent_before_its_outcome`, sola nell'intero workspace | **8**, **9** |

Voce `E115`. La tabella delle mutazioni, coi conteggi, sta in [archivio](archivio/porta-di-qualita-storico.md), sezione
«La transizione fra le due policy — Traguardo 5, Task 9, e `V3` si chiude».

⚠️ **CIÒ CHE QUESTO COMPITO NON COPRE, dichiarato invece che taciuto.** ① Il passo in dubbio è provato su **UNO stato
costruito a mano** — `CrashingJournal::falling_at(1)` — e non su una campagna di semi: quella è del **Task 12**, e questa
sonda esiste perché quella campagna abbia una **forma da cercare** invece di una speranza. ② `set_policy` **non ha
nessun chiamante di produzione**: il grafo di produzione **costruisce** l'arbitro con `VramPolicy::Remote(RemotePolicy)`
— la scelta è consegnata al costruttore — e non **transita** mai; una transizione all'avvio sarebbe uno scambio che
nessuno chiede. Il chiamante nasce col **primo ciclo di orchestrazione**, che in questo repository non esiste ancora.
⚠️ **[C-S2-8]** ③ La classe `EffectClass::Idempotent` è argomentata per ciò che il Traguardo 5 fa davvero —
**scambiare un oggetto**; quando arriverà il **contenuto** di uno sfratto (L2) va riguardata, perché una ricarica non è
gratis da ripetere, e il limite è scritto anche accanto al codice.

#### Il grafo di produzione monta l'arbitro, il giornale e le due concessioni — Traguardo 5, Task 10, e `E41` si chiude

⛔ **LE DUE QUOTE DI ADR-0033 NON SONO SOTTRAZIONI, SONO DUE CONCESSIONI, E LA DIFFERENZA È `I2`.** Una quota sottratta
al budget **senza un titolare** lascia `I2` falsa per quel consumatore — *«la sottrazione non è un'esenzione»*, ADR-0005
e gotcha **#4** — mentre una concessione un titolare ce l'ha per costruzione. `crates/daemon/src/main.rs` le chiede
tramite `Arbiter::admit` come qualunque altra richiesta: ⛔ **l'arbitro non sa che si chiamano *audio* e
*presentazione***, ed è ADR-0001 — nessuna capacità ha accesso privilegiato.

⛔ **`E41` SI CHIUDE QUI, E NON CON UN'ASSERZIONE DENTRO UNA SONDA.** `E41` dice che una configurazione impossibile ha
**smesso di annunciarsi** il giorno in cui l'arbitro ha avuto le code: la seconda quota permanente torna **`Queued`**
invece di `Refused`, e nessuno la servirà mai, perché rilasciare una concessione permanente è esattamente ciò che nessuno
fa. L'arbitro non può ripararlo — *«permanence is not a type, it is nobody calls release»*, quindi non sa distinguere un
biglietto che **sarà** servito da uno che non lo sarà mai. Il chiusore è la **radice di composizione**: `reserve`
traduce l'`Admission` in `Result<Grant, StartupError>`, e qualunque cosa non sia `Granted` diventa
`StartupError::ReservedQuota { name }`, che **nomina la quota** e **ferma l'avvio**.

⛔ **DUE SONDE PERMANENTI E NON UNA MUTAZIONE, E LE VIE SONO DUE PERCHÉ FALLISCONO DIVERSAMENTE.** Una direzione di
prova tenuta da una mutazione è tenuta da **niente** — la mutazione si revoca e il verbale resta a dire che la riga è
chiusa, gotcha **#72**. E dentro `admit` le due strade sono distinte (gotcha **#65**): *«più grande di ciò che è libero
adesso»* è `Queued`, *«più grande della macchina intera»* è `Refused`. Una sonda sola lascerebbe scoperta la strada che
non prende.

| Sonda | `total_vram` | Cosa succede | Cosa asserisce |
|---|---|---|---|
| `a_permanent_quota_that_only_queues_stops_the_start_up` — è **esattamente** lo scenario di `E41` | `Mib::new(1_500)` | `audio-reserved` (1024) entra, `presentation-reserved` (768) non ci sta e viene **accodata** | `StartupError::ReservedQuota` con `name == "presentation-reserved"` |
| `a_permanent_quota_bigger_than_the_machine_stops_the_start_up` | `Mib::new(500)` | `audio-reserved` è più grande della macchina intera → **`Refused`** | `StartupError::ReservedQuota` con `name == "audio-reserved"` |

⚠️ **E LE DUE PASSANO DAL GRAFO INTERO, non da un arbitro ricostruito nella sonda.** È ciò che le fa tenere il
**cablaggio** e non solo `reserve`: tolte le due prenotazioni, o tolta la chiamata a `build_the_arbiter` dal grafo,
diventano rosse (righe **1** e **10** della campagna del `daemon`, in archivio). ⛔ **Perché la firma può scegliere il
totale:** `run_the_graph` prende i `Parameters` come **argomento** invece di leggerli dalla costante `TOTAL_VRAM`, e
senza quello le due sonde non sarebbero scrivibili — il ramo d'errore che chiude `E41` non sarebbe raggiungibile da
nessun controllo.

⛔ **`assert_eq!` NON COMPILA SU QUESTI TIPI, E NON È UN DETTAGLIO DI STILE.** `platform::journal::OpenError` deriva
**il solo `Debug`**, quindi `StartupError` non può derivare `PartialEq`; `Admission` non deriva né `Debug` né
`PartialEq`, perché `Grant` non li ha e non deve averli. Le sonde quindi **filtrano con `match`** e portano il `Debug`
**dentro il messaggio**: un `is_ok()` nudo non direbbe **quale** dei rami ha sparato — né lo farebbe un
`assert_eq!(run_the_production_graph(&…), Ok(()))`.

#### Le sonde del `daemon`, per nome

| Sonda | Che cosa tiene |
|---|---|
| `the_production_graph_assembles_and_the_executor_runs_to_completion` ⚠️ **[C-S2-10]** | il **cablaggio**: `SequentialRng`, `SystemReactor`, `FileJournal`, l'arbitro con le due concessioni, i `Parameters` consegnati e la cella `Sleep` stanno insieme, e l'esecutore torna dicendo che il giro è finito |
| `the_production_graph_leaves_its_journal_on_the_disk` | che il giornale sia davvero **aperto**. Senza questa, un cablaggio che avesse tolto la riga `FileJournal::open` resterebbe verde: niente in questo binario **legge** il giornale |
| `a_journal_that_cannot_be_opened_stops_the_start_up` | la direzione opposta (§7.1.1 regola 3): una cartella che non c'è → `StartupError::Journal` |
| `the_two_reserved_quotas_are_held_by_grants_and_not_subtracted` | che le due quote siano **spese** — `allocated()` vale `1792` MiB — **e** che la policy montata sia `"remote"`, il default di ADR-0006: senza di lei quel default è affermato in un commento e tenuto da niente (riga **8**, `VramPolicy::Remote(RemotePolicy)` → `Local(LocalPolicy)`) |
| `a_permanent_grant_survives_to_the_last_instant_of_the_axis_and_is_swept_at_it` | `FOR_EVER`, nelle **due direzioni**: a `u64::MAX - 1` le due concessioni ci sono ancora, a `u64::MAX` sono **riscosse** e `allocated()` torna `Mib(0)` |
| `the_two_reservations_declare_no_preemption_and_one_lane` | i **due campi** delle due `ResourceProfile` (`AUDIO_RESERVATION`, `PRESENTATION_RESERVATION`): `Preemption::Never`, che è la parola *«non-preemptible»* di ADR-0033, e `ComputeClass::Realtime`, che è la **premessa** della frase accanto a `build_the_arbiter` — una corsia sola, quindi dentro l'arbitro l'unico spareggio è l'**arrivo** |
| `a_permanent_quota_that_only_queues_stops_the_start_up` | `E41`, via **`Queued`** |
| `a_permanent_quota_bigger_than_the_machine_stops_the_start_up` | `E41`, via **`Refused`** |

⛔ **NESSUNA RIGA DI CATALOGO NUOVA NELLA §7.4, E LA COSA SI REGISTRA INVECE DI DECIDERLA** (vincolo globale 7): queste
sonde **non hanno riga di catalogo propria**. Vivono sotto `I2 · §5.3` e sotto la riga blocco C `V29 · §2.8 · ADR-0034`
per il verso della consegna dei parametri, ma *«la radice di composizione ferma l'avvio quando una quota permanente non
entra»* non è una riga che esista: aggiungerla è una decisione del **proprietario**.

⛔ **LA CAMPAGNA DI MUTAZIONE HA DUE MUTANTI VIVI PER SCELTA**, e la tabella, coi conteggi e la riga che ciascuna sonda
uccide, sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «Le sonde del `daemon`, per nome». La riga
**9**, `EXECUTOR_TURN_LIMIT` → `0`: `Executor::run` è `while !self.tasks.is_empty()`, e senza attività il corpo non gira
mai, quindi qualunque valore passa; il numero avrà la sua sonda quando ci sarà qualcosa da lanciare. ⚠️ **[C-S2-11]**
La riga **15** è il residuo ⑤ qui sotto.

⚠️ **CIÒ CHE QUESTO COMPITO NON COPRE, dichiarato invece che taciuto.**

① **I tre rami d'errore di `main` non li osserva nessun controllo.** ⚠️ **[C-S2-11]** Coprirli vuol dire lanciare il
binario come **processo figlio** e rileggerne i due flussi e il codice d'uscita, per tenere righe che non prendono
nessuna decisione. ⛔ **E i rami sono distinti per una ragione misurata, non per gusto:** `#[derive(Debug)]` **non conta
come lettura** per l'analisi del codice morto, quindi un ramo unico con `{error:?}` lasciava due avvisi
`field 0 is never read` su `Journal` e `Run` — e un `#[allow]` è un divieto spento (gotcha **#13**). ⛔ **`ReservedQuota`
e `Run` non si provocano da fuori** senza modificare il sorgente, che è ciò che fa la campagna di mutazione e che una
prova a mano non può fare.

② **`JOURNAL_PATH` non lo esercita nessuna sonda.** Ogni sonda passa il **proprio** percorso, in una cartella privata per
sito di chiamata ricavata da `line!()` — percorso fisso in cartella condivisa è il gotcha **#52**, e su Windows la
cancellazione a file aperto fallisce in silenzio, quindi il rosso uscirebbe **su Linux**. Il valore di produzione è un
letterale che nessun controllo tocca, e **dove** debba stare il file è una decisione che nessun ADR ha preso.

③ **`E50`, `E51` ed `E100` NON si chiudono al Task 10**, per decisione del proprietario del 2026-08-21: il Task 10 non costruisce
**nessun ciclo di orchestrazione** — monta il grafo e lancia l'esecutore — quindi il posto in cui si sceglie fra `admit`
e `promote` **non esiste ancora**. I tre mutanti restano **vivi e non pinzati**: pinzarli sarebbe un voto contro una
decisione che il proprietario si tiene (gotcha **#73**). La designazione del chiusore non nomina nessun compito —
*«whoever builds the first orchestration cycle»*, come nei riquadri di `crates/kernel/src/arbiter/mod.rs`.

④ **`FOR_EVER` non è letteralmente «mai».** `Monotonic::ORIGIN.saturating_add(FOR_EVER)` satura **a** `u64::MAX`, e
`collect_expired` confronta `expires_at <= now`, quindi una spazzata all'**ultimo millisecondo rappresentabile** riscuote
entrambe le quote — `allocated()` torna `Mib(0)` invece di `Mib(1792)`. La sonda
`a_permanent_grant_survives_to_the_last_instant_of_the_axis_and_is_swept_at_it` percorre il confine nelle **due
direzioni** (vincolo 6): sta **un millisecondo dentro** la finestra e poi spazza a `u64::MAX` e **attende `Mib(0)`**.

⑤ **`Monotonic::ORIGIN`, il terzo argomento di `admit` dentro `reserve`, non lo tiene niente.** Cambiato in
`Monotonic::from_millis(1)`, l'intero workspace resta verde: è la riga **15** della campagna, **viva per scelta** —
`FOR_EVER` satura, quindi `now.saturating_add(FOR_EVER)` vale `u64::MAX` qualunque sia `now`, e l'indifferenza è una
**conseguenza aritmetica** e non una decisione da difendere. Dichiarata accanto al codice invece che scoperta.

⑥ **I due campi delle due `ResourceProfile` sono pinzati per VALORE e non per conseguenza, e il prezzo si scrive qui.**
La strada che attacca il **meccanismo** — un arbitro sotto `VramPolicy::Local` a cui si chiede più di quanto è libero, e
poi una spazzata oltre ogni grazia — è stata **costruita e misurata** prima di scegliere: nessuna mutazione di un campo
solo la uccideva, solo mutando **entrambi** i campi dello stesso profilo diventava rossa. Una sonda che nessuna mutazione
singola uccide è la sonda vacua della **prima domanda**, quindi non è stata tenuta. ⛔ **Perché non li uccida — le
guardie di `Arbiter::ask_back` — sta accanto alla sonda nel sorgente, in un posto solo.** ⚖️ **Ciò che resta scoperto:**
che uno dei due campi cambi qualcosa che l'arbitro **fa**.

#### Le contro-sonde delle righe nuove

Per file — la direzione che si dimentica (§7.1.1 regola 3). ⛔ **Nessun numerale nelle celle:** il numero di test di un
file si conta **sul binario**, `cargo test --locked -p kernel --test <file>`, mai per aritmetica; e un **elenco** di
soggetti è un numerale, e si toglie come lui — *un elenco invecchia, una regola no* (precedenti **AUD-007** e
**AUD-046**, e `gateway_decisor.rs`, voce `E97`).

| File | Righe che difende | |
|---|---|---|
| `crates/kernel/tests/time_types.rs` | **blocco C** · `V29 · §2.1`, entrambe | |
| `crates/simulator/tests/seeded_rng.rs` | **blocco C** · `V29 · §2.2` | |
| `crates/kernel/tests/boundary_promotion.rs` | **blocco C** · `Q9 · I6 · V20`, **entrambe** — la promozione dichiarata è la contro-sonda della regola A e della regola B — **e blocco B** · `V19` | vedi sotto |
| `crates/kernel/tests/parameters_delivered.rs` | le **due** righe **blocco C** · `V29 · §2.8 · ADR-0034` | `cargo test --locked -p kernel --test parameters_delivered`; vedi sotto |
| `crates/kernel/tests/permission_triple.rs` | ⛔ **nessuna riga di catalogo** — i due casi `compile_fail` che accompagnano questo banco non ne hanno una, ed è una voce **registrata e non presa** (§7.4 è spec, vincolo globale 7). Ciò che tiene è `V21`, che resta **`⚠️ parziale`** | `cargo test --locked -p kernel --test permission_triple`. Le mutazioni e il loro esito stanno nella sezione «Il compito 7 del Traguardo 6» |
| `crates/kernel/tests/arbiter_admission.rs` | **blocco C** · `V4` — la direzione *«distinguere le tre compila»* — `I2 · §5.3` — la direzione *«quello legale SI COSTRUISCE»* — `V2` (*«con il profilo compila»*) e la riga del **blocco B** *«avviare un worker ← una concessione»* (*«con la concessione compila»*). Nessuna tenibile dal rispettivo caso negativo | delle sonde del Task 7 qui ne vive **una sola**, le altre in `crates/kernel/src/arbiter/mod.rs`: `ask_back` è `pub(crate)` e da qui non si vede — `` error[E0624] `` |
| `crates/kernel/tests/arbiter_policy.rs` | **blocco C** · `V3` — la direzione *«con UNA policy compila»*, e con **entrambe** le policy: il banco costruisce un arbitro per ciascuna, da fuori la crate. E la seconda metà della contro-sonda di catalogo — *«e la transizione resta un passo giornalato (§5.4)»*: le cinque sonde di `Arbiter::set_policy`, che chiudono la riga `V3` (`E103`) | |
| `crates/kernel/tests/record_shape.rs` | **blocco C** · `Q14 · §4.9` **e** `Q9 · I6 · V20 · §4.9` — la contro-sonda dell'etichetta è `every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes`, che scrive **entrambi** i valori e ne confronta i byte | |
| `crates/kernel/tests/worker_tokens.rs` | le **quattro** righe di §6.10.5 — **blocco B** · `I2` (*«col `Worker` → compila»*) e `I5 · Q4` (*«con la ricevuta → compila»*), **blocco C** · `I2 · §6.10` (*«istruirlo prima dell'uccisione compila»*) e `I5 · §6.10` (*«leggerne una compila»*). Ottiene un `Grant` **vero** da `Arbiter::admit`, mai un costruttore di test. ⚠️ **Una sola funzione ne tiene due:** `reading_once_with_the_receipt_compiles` è la contro-sonda della riga del blocco B *e* di `I5 · §6.10` | `cargo test --locked -p kernel --test worker_tokens`; vedi sotto |
| `crates/kernel/tests/gateway_decisor.rs` | **blocco B** · `Q13` — la direzione *«filtrato → compila»*, che è `a_conforming_candidate_is_chosen_and_nothing_is_degraded`. Ottiene il gettone **solo** da `resolve`, mai da un costruttore di banco — che non esiste, ed è il caso `conforming_has_no_constructor.rs` a dirlo. ⛔ **Le altre sonde non difendono nessuna riga di catalogo**; a dire quali mutazioni le rendono portanti è la sezione «Il gettone di conformità del gateway» | `cargo test --locked -p kernel --test gateway_decisor`. Voce `E97` |

⛔ **`boundary_promotion.rs` è la contro-sonda di due blocchi insieme**, e ciascuno dei suoi test dice una cosa sola: la
promozione dichiarata **compila ed è registrata**, col proprio passo e la propria ragione — contare i record non basta,
un `promote` che scrivesse il passo sbagliato o una ragione vuota lascerebbe il conteggio a uno (gotcha #30); un giornale
che **rifiuta rifiuta anche la promozione**, altrimenti V19 poggerebbe sulla diligenza del chiamante; `summarize`
restituisce `Untrusted`, e a provarlo è **l'annotazione**, cioè il compilatore, non un'asserzione; `summarize` conta
**caratteri e non byte** — misurato: con fixture di solo ASCII gli altri test restavano verdi mentre una fetta di byte
moriva sul primo taglio dentro un carattere multi-byte; e il `Debug` di `Untrusted` **non stampa il contenuto**, che
chiudeva una via d'uscita dal confine — con `Debug` derivato, `Instruction::new(format!("{:?}", untrusted))` portava il
testo attraverso intatto.

⛔ **`parameters_delivered.rs` dà alle due righe la contro-sonda che la regola 3 di §7.1.1 pretende** — due casi per
voce, non il solo caso `compile_fail`. Il file pinza che il valore **porti** i parametri risolti, che due valori diversi
si distinguano **e** che due uguali non denuncino una sostituzione mai avvenuta (gotcha #24), e che **nessun ripiego
viva dentro il costruttore** — la via d'ingresso più economica per un default, che il compilatore **non** può vietare e
che §2.8.4 dichiara come limite.

⚖️ **`worker_tokens.rs`.** `one_grant_starts_one_worker` **non è la contro-sonda di una riga nuova, e non tiene nessuna
forma che `a_started_worker` non compili già** — stessa chiamata, stessa concessione usata una volta sola, e le altre lo
invocano: nessuna mutazione può renderlo rosso da solo. Resta un luogo dichiarato dove appendere il ragionamento sul
`Grant` consumato. `a_spawn_that_does_not_happen_is_start_failed` è l'**unico produttore** di `ProcessError::StartFailed`
nel workspace — finding **AUD-051**: la variante era tenuta in vita da una scadenza in prosa, e la sonda sostituisce la
data con qualcosa che deve continuare a compilare (gotcha **#77**). ✅ **Provata portante per mutazione:** col fake che
risponde `Ok(FakeWorker)` invece di `Err(StartFailed)`, `assert_eq!` dà `left: None, right: Some(StartFailed)` e la
sonda è **rossa da sola**. ⚠️ **La sua forza è di LIVELLO 1 ed è dichiarata nel sorgente:** una finta prova che la
parola sia costruibile e che `start` riporti indietro un fallimento, **non** che un avvio vero fallisca — il produttore
vero arriva col traguardo che implementa la porta. ⚠️ **E `assert_eq!` invece di `is_err()` non è stile:** la finta
fallisce sempre, quindi `is_err()` resterebbe verde anche se rispondesse `Died`.

#### I casi di `compile_fail`, e cosa provano davvero

⛔ **I casi dal Traguardo 2 in poi nominano `kernel::` e non ridichiarano attributi propri**, a differenza dei quattro
del Traguardo 1: è il rimedio al gotcha **#39**, e significa che i loro oracoli sono accoppiati alla **superficie
pubblica del kernel**. Un cambio di firma li rende rossi, ed è corretto che lo faccia. Vale per i dieci del Traguardo 2 e
per quelli venuti dopo — fra cui `record_without_version.rs`, `record_without_trust_label.rs` e `trust_has_no_default.rs`
del Traguardo 3, `two_policies_at_once.rs` e i casi di §6.10.5 del Traguardo 5 (`talking_without_the_handle.rs`,
`instructing_after_the_kill.rs`, `reading_without_a_receipt.rs`, `reading_twice_from_one_receipt.rs`). ⛔ **Quanti sono
si misura col comando e mai per aritmetica** — `ls crates/kernel/tests/compile_fail/*.rs | wc -l` — perché è il
numeratore di un contenitore che cresce, e chi lo muove è chi scrive un caso, che questa riga non la apre nemmeno
(gotcha **#31**).

⛔ **La colonna «Caso negativo» prova il meccanismo, non la dichiarazione — e il registro non deve lasciar credere
altro.** I **quattro casi del Traguardo 1** — `std_in_kernel.rs`, `unsafe_in_kernel.rs`, `allow_overrides_forbid.rs` e
`hashmap_in_kernel.rs` — **ridichiarano ciascuno i propri attributi** e non nominano mai `kernel::`: provano che
`#![no_std]` e `#![forbid(unsafe_code)]` **mordono dove sono dichiarati**, non che siano dichiarati nel kernel. Il limite
riguarda quei quattro e non la cartella; la misura e il comando che li trova stanno nel gotcha **#39** di
[`HANDOFF.md`](HANDOFF.md).

A sorvegliare la **presenza** degli attributi è `scripts/gate-attributes.sh`, che è di **livello 2**: un controllo
esterno, quindi cancellabile. La riga di `forbid` è di ramo **1b** — sostiene la validità dei blocchi A, B e C — e
poggia quindi su un controllo più debole di quello che difende. È dichiarato, non nascosto.

**Contro-sonde:** `crates/platform/tests/counter_probes.rs` — `platform` nomina `std` e usa `unsafe`, e **compila**. Sono
la direzione che si dimentica (§7.1.1 regola 3).

**Guardia di non-vacuità del banco:** `crates/kernel/tests/compile_fail.rs` conta i `.rs` prima di chiamare `trybuild`.
Senza, il banco **vuoto** usciva **verde**: un glob che non pesca niente non è un errore per `trybuild`, che stampa un
avviso e lascia i fallimenti a zero. Misurato, non dedotto.

#### Una guardia salita al compilatore — 2026-08-21

⛔ **`crates/simulator/tests/dst_campaign.rs` tiene `WRITES_PER_RUN > 0` con un blocco `const`, non con un `assert!` DI
ESECUZIONE su un operando `const`**, che sarebbe una guardia di livello 2 su un fatto che il compilatore conosce. È la
preferenza dichiarata della §7.1.2 — *una regola che può salire al compilatore ci sale* — e qui è costata **una
parola**.

✅ **Provata nelle DUE direzioni su una crate usa-e-getta cancellata subito dopo**, perché una guardia che non si è vista
scattare non è una guardia (gotcha **#14**), e una che scatta dove non deve è peggio di una assente (**#24**):

| Direzione | Misura |
|---|---|
| **deve scattare** | con la costante a `0`, `cargo build` risponde `` error[E0080]: evaluation panicked: a scenario with no writes has nothing to fall at `` — **il messaggio della guardia, a tempo di compilazione** |
| **non deve scattare** | con la costante al valore vero, `24`, la crate **compila** |

⚠️ **A nominarla è stato `clippy::assertions_on_constants`, e non è una deroga alla §7.4.3.** Clippy non ha voce nella
porta: non ha deciso **se** la regola valga — quella c'era già — ha detto **a che livello** era tenuta. La decisione di
salire è della §7.1.2. ⛔ **E non è una riga di catalogo:** la §7.4 è **spec**, vincolo globale 7, quindi si **registra
e non si prende** — stesso trattamento di `PL-1`, di `K-1`/`B-1` e delle dieci sonde dell'audit precedente.

#### Il gettone di conformità del gateway — Traguardo 6, Compito 6, e `Q13` si chiude

⛔ **La riga del blocco B che questo file attribuiva al «filtro dei vincoli (§6.3)» è chiusa.** `Conforming` vive in
`crates/kernel/src/gateway/mod.rs` con **tre** campi privati e **nessun** costruttore pubblico; l'unico che ne conia uno
è `resolve`, nello stesso modulo; `dispatch` lo prende **per valore**. È la forma di `Arbiter::issue` per `Grant`
(§5.6), e la cifra si riconta sulla cella del blocco **B** in fondo a questo file, mai da questa frase.

⛔ **I casi sono DUE perché le metà del gettone sono due, e la riga di catalogo ne nomina una.** Lo stesso trattamento
che `grant_has_no_constructor.rs` riceve sotto la riga della concessione.

| Metà | Caso | Sigla | Come scatta |
|---|---|---|---|
| il candidato non filtrato **non è l'argomento** | `dispatching_an_unfiltered_candidate.rs` | `E0308` | ⚠️ **`mismatch`, la forma DEBOLE** (gotcha **#42**): riporta per l'oracolo, quindi dipende da un `.stderr` |
| il gettone **non si conia** | `conforming_has_no_constructor.rs` | ⛔ **senza sigla** | **`error`**: la forma forte, indipendente dall'oracolo |

⛔ **I due oracoli sono LETTI DALL'USCITA VERA e scritti a mano, mai copiati dal gemello**, e la differenza si vede:
`grant_has_no_constructor.stderr` dice *«private fields `id` and `issuer`»*, `conforming_has_no_constructor.stderr` dice
*«private fields `model`, `evaluated` and `degraded`»* — **tre** campi contro due. Copiare il gemello avrebbe prodotto un
`mismatch` invece di un verde.

⚖️ **UNA VOCE REGISTRATA E NON PRESA, sul precedente esatto del `Grant`.** Il doc di `dispatch` promette che *«una
risoluzione dispaccia una volta sola»*, e il tipo lo regge davvero — `Conforming` non deriva né `Copy` né `Clone`, quindi
un secondo `dispatch` è `error[E0382]` — ma **nessun caso lo dice**. La gemella per `Grant` (*«un secondo `start` con la
stessa concessione»*) è **misurata e non presa** dal Traguardo 5 per la ragione che decide anche questa: una riga di
catalogo nuova è §7.4, cioè **spec**, cioè del **proprietario** (vincolo globale 7). ⛔ **Dichiarata accanto al codice**
— sul doc di `dispatch` — perché è lì che un lettore di quella promessa guarda. Voce `E62` dell'errata del piano.

##### Le tre uscite del filtro sono TRE sonde, e le mutazioni dicono quali righe contano

⛔ **Tre uscite, tre `#[test]` distinti e non uno con tre asserzioni** — conforme, fallimento chiuso (classe `Data`,
ADR-0012), degrado dichiarato (classe `Quality`). Una sonda sola si ferma alla prima asserzione, e la seconda uscita non
verrebbe mai esercitata (gotcha **#14**).

Quali righe di codice tengono le sonde, per mutazione; le misure stanno in [archivio](archivio/porta-di-qualita-storico.md),
sezione «Le tre uscite del filtro sono TRE sonde, e le mutazioni dicono quali righe contano».

| # | Mutazione | La uccide | Perché conta |
|---|---|---|---|
| **M1** | `RecordKind::Routing => {}` → `enter(&mut open, step, resolution_of(body.effect()))` in `crates/kernel/src/reconcile.rs` | `a_routing_record_does_not_put_a_step_in_doubt` e `a_routing_record_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them`, e nient'altro nel workspace | |
| **M2** | lo stesso arm → `leave(&mut open, step)` | **solo** `a_routing_record_leaves_the_doubt_…` | ⛔ **La differenza fra M1 e M2 prova che la coppia non è un controllo scritto due volte:** la prima sonda non raggiunge `leave`, perché il record di routing dopo l'esito chiude un passo già chiuso ed è un'operazione nulla |
| **M3** | `Constraint::NoRetention => !candidate.retains` → `candidate.retains` in `crates/kernel/src/gateway/mod.rs` | `a_retaining_candidate_is_discarded_by_no_retention` e `a_candidate_that_keeps_nothing_satisfies_no_retention` | ⛔ **Senza queste due l'arm è un MUTANTE VIVO su tutto il workspace**, e la mutazione ammette esattamente i provider che **trattengono i dati**, la classe che ADR-0012 fa fallire chiuso. Voce `E61` |
| **M4** | `let evaluated = chain.len() as u32` → il conteggio dei candidati **camminati** (`enumerate().find(..)`, posizione + 1) | `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it`, con `left: 1, right: 3` | ⛔ **Per questo la catena della sonda è lunga TRE e il vincitore è il PRIMO:** con una catena di uno le due letture valgono entrambe 1 e nessuna asserzione può separarle. Copre la **seconda** passata. Voce `E59` |
| **M5** | la quinta voce tolta dall'array a mano di `crates/kernel/tests/frozen_bytes.rs`, arità riportata a **4** | **due** sonde in quel banco | `no frozen record carries Routing: its wire index is held by nothing`, e `the map describes 5 records and there are 4 frozen files` — l'`assert!` d'arità **vista scattare** |
| **M6** | `Trust::Instruction` → `Trust::Untrusted` nel record che `dispatch` scrive, in `crates/kernel/src/gateway/mod.rs` | `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it` | ⛔ **L'etichetta `I6`/ADR-0014 su un record DUREVOLE:** `dispatch` scrive **sei** campi, e la sonda li rilegge tutti. Voce `E97` |
| **M7** | `EffectClass::Idempotent` → `Unrepeatable` nello stesso record | la stessa | ⚠️ **Pinzato benché `reconcile` non lo legga**, e la ragione è scritta accanto all'asserzione: il doc accanto alla chiamata **afferma** che il valore è vero di quel record. Precedente del **Task 10** del Traguardo 5 — *pinzati, non dichiarati, perché la decisione l'ha presa un ADR e il doc la afferma*. Voce `E97` |
| **M8** | il letterale di `reason` → `"MUTATED reason nobody pins"` nello stesso record | la stessa | Voce `E97` |
| **M9** | `journal.note(step, &record)` → `let _ = journal.note(step, &record); Ok(())` — l'errore del giornale **inghiottito** | `dispatch_does_not_swallow_the_journal_saying_no` | ⛔ **La via d'errore di `dispatch`:** la funzione rende `Result<(), JournalError>`, e un `dispatch` che dichiara successo mentre il record non ha raggiunto il giornale è la perdita silenziosa che ADR-0007 esiste per vietare. ⚠️ **La sonda NON apre il passo, di proposito** — è l'unica cosa che `open_the_step` esiste per evitare ovunque altro in quel banco. Voce `E98` |
| **M10** | `let evaluated = chain.len()` → il conteggio della **PRIMA** passata (`position(..) + 1`) | `evaluated_is_what_the_chain_OFFERED_and_not_what_the_first_pass_walked` | ⛔ **`M4` copre la SECONDA passata e non la prima:** nello scenario di `the_dispatch_journals_…` la prima passata non trova niente e **esaurisce** la catena, quindi *«offerti»* e *«camminati dalla prima passata»* valgono entrambi **tre** e nessuna asserzione li separa. Questa ferma la prima passata all'**indice 0** — cammina uno, la catena ne offre tre. Voce `E100` |

⚠️ **E due guardie di livello 1 sono state viste scattare prima di essere estese**, che è la metà che si dimentica:
`RecordKind::Routing` ha reso `error[E0004]` la riconciliazione, il `match kind` e il `match detail` di
`frozen_bytes.rs`, **e** il `match` del costruttore di `crates/kernel/tests/record_shape.rs` — quest'ultimo non censito
dalla voce `E60`.

⛔ **E il record di routing NON invoca nessun modello, che è ADR-0020 in pratica.** Verificato col comando e non dedotto
— `grep -rn "provider\|adapter" crates/kernel/src/gateway/` rende solo righe di **commento**, nessuna chiamata. È la
stessa forma della verifica su `\bGrant\b` in `crates/kernel/src/wire/`.

## Livello 2 — controlli esterni

Ogni voce porta **due** direzioni, per la regola 3 di §7.1.1: quella che deve scattare e
quella che deve restare verde.

| Regola del catalogo | Dove | Deve scattare | Deve restare verde |
|---|---|---|---|
| allow-list, grafo **spedito** | `scripts/gate-deps.sh` | N2 · **N5** · **N6** | N1 · **N4** · **N7** |
| allow-list, grafo **di build** | idem, e l'errore è **diverso** | N3 | N1 |
| cancello senza OS su `x86_64-unknown-none` | `scripts/gate-no-os.sh` | B2 · **B4** | B1 · **B3** |
| le crate vincolate **dichiarano davvero** i propri attributi | `scripts/gate-attributes.sh` | `forbid` tolto · `deny` al posto di `forbid` · attributi tolti a `simulator` · file atteso assente · lista dei vincolati vuota | stato pulito · `platform`, `secrets` e `daemon` |
| le crate vincolate **non hanno un build script** | idem, e l'errore è **diverso** | `crates/kernel/build.rs` · `crates/simulator/build.rs` · `build = "gen.rs"` nel manifesto · manifesto assente | `crates/platform/build.rs` · `build = false` |
| coerenza della documentazione | `scripts/check-docs.sh` | S1…S6c · S7 · S7b · S7c · S7d | C0 · C5 · **C6** |
| i **test di contratto** fra porta finta e porta vera — porta `reactor` | `crates/kernel/tests/reactor_contract.rs`, incluso da `crates/platform/tests/reactor_contract_real.rs` | **R3** · **R4** · R5 | R1 · R2 · **R6** |
| i **test di contratto** — porta `journal`, **due implementazioni su due** | `crates/kernel/tests/journal_contract.rs`, incluso da `crates/platform/tests/journal_contract_real.rs`: gira **a ogni commit** contro **entrambe**, `MemoryJournal` e `platform::journal::FileJournal`. ⛔ Le promesse 1, 5 e 8 sono provate anche nello **stato che distingue un'implementazione sbagliata** — un archivio con più di un passo — senza nessuna promessa aggiunta e senza toccare le implementazioni: finding **T-1** e **T-2**, gotcha **#63**, bugiardi **J14**, **J15**, **J16** | **J2** · **J3** · **J4** · **J5** · **J6** · **J7** · **J8** · **J10** · **J13** · **J14** · **J15** · **J16** | J1 · **J9** · **J11** · **J12** |
| ⛔ i **byte congelati** del record durevole — §4.9.4, riga di catalogo `Q14 · §4.9` | `crates/kernel/tests/frozen_bytes.rs`, più `tests/frozen/` — i `.cbor` e la mappa. ⛔ **Non si rigenerano, e non c'è nessun percorso per farlo:** niente flag, niente variabile d'ambiente, niente `--bless` — è così che `trybuild` si disarma (gotcha **#25**). I byte sono stati **scritti a mano** dall'uscita di una sonda usa-e-getta | **F1** · **F2** · **F3** | **F4** · **F5** · **F6** |
| ⛔ i **byte consumati** dalla decodifica pari alla **lunghezza dichiarata** dal frame — §6.10.4, riga di catalogo `Q4 · I5 · §6.10` | la busta in `crates/kernel/src/framing.rs`, lo schema in `crates/kernel/src/wire/worker.rs`; le sonde in `crates/kernel/tests/framing.rs` e `crates/kernel/tests/worker_wire.rs`, **entrambi fuori dalla crate** — se `framing::frame` e `FromWorker` non fossero abbastanza `pub`, questi banchi non compilerebbero. Quante siano lo dice il comando e non questa cella (gotcha **#31**): `grep -c '^#\[test\]' crates/kernel/tests/framing.rs crates/kernel/tests/worker_wire.rs`. ⛔ La via del `map_err` di `FromWorker::decode` la tengono `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode`: i **due** ingressi che l'argomento di contenimento di `Record::encode` nomina — corpo **vuoto** e corpo **troncato** — e che `bytes_that_are_not_a_record_decode_to_malformed` tiene per `record.rs` | **W1** · **W2** · **W3** · **W4** · **W5** · **W6** · **W9** · **W10** | **W7** · **W8** |

#### ⛔ Il settimo passo del cancello, che non è un settimo controllo

⛔ **Il passo `DST campaigns -- wall time` di `scripts/gate.sh` non è un controllo, e il catalogo
resta a sei controlli.** Le asserzioni delle campagne DST girano **già** dentro
`cargo test --workspace`, che è il secondo controllo — la cadenza che il vincolo 8 della §11
chiede — e questo passo **non può diventare rosso per una ragione che quel controllo non abbia già
colto**. Esiste per una cosa sola: il vincolo **7** vuole che il **tempo di parete si stampi a ogni
corsa**, *«così l'appesantimento diventa visibile prima di diventare una tentazione»*, e
`cargo test` **inghiotte l'uscita dei test che passano**; le campagne si rieseguono quindi con
`--nocapture`.

⛔ **Il passo nomina i propri bersagli uno per uno: una campagna che non è nell'elenco è MUTA**, e il
cancello resta `GATE GREEN`. L'elenco è la riga `run` del passo in `scripts/gate.sh`, e solo lei;
il commento accanto porta la regola, le volte in cui è stata imparata, e il costo con la sua data.

| | |
|---|---|
| **deve restare verde, e le righe si devono VEDERE** | le righe `DST L1 interleaving`, `DST L1 campaign`, `DST arbiter crashes` · `ceiling` · `expiry` · `worlds`, `DST L2 short` ⚠️ **[C-S3-4]**. ⛔ **I CONTEGGI sono stabili fra una corsa e l'altra, i TEMPI no**, ed è la cosa da portarsi via. Le righe di una corsa, con le cifre, stanno in [archivio](archivio/porta-di-qualita-storico.md), sezione «⛔ Il settimo passo del cancello, che non è un settimo controllo» |
| ⛔ **deve scattare** | un rosso forzato in una campagna dà **`GATE RED -- 2 checks failed`**: il secondo controllo **e** questo passo. Provato anche sul bersaglio dell'arbitro, con la mutazione `M3` — `crash_point` che risponde sempre `0` — perché una direzione provata su un bersaglio non è provata sugli altri |
| **il costo, dichiarato** | le campagne brevi girano **due volte**, e un rosso di una campagna si conta **due volte**. La cifra si prende **dentro il binario**, non dall'orologio attorno a `cargo`; è un **ordine di grandezza**, si **rimisura** e non si riporta, e vive con la sua data nel commento del passo in `scripts/gate.sh` |

⛔ **La doppia rossa non è un difetto ma l'unica prova che il passo esegua ciò che dichiara** — la
regola 3 di §7.1.1 applicata a un passo che non è un controllo; l'argomento sta nel commento del
passo in `scripts/gate.sh`.

⚠️ **I prefissi sono una famiglia sola** — `DST L1 campaign:`, `DST L2 short:` e `DST arbiter ` —
perché quelle righe esistono **per essere lette come coppia**, e due grafie per una cosa sola
costringono chi scorre l'uscita a conoscerle entrambe.

#### La campagna DST dell'arbitro — Traguardo 5, Task 12

⛔ **CHE COSA COMPRA, IN UNA RIGA: l'arbitro gira DENTRO l'esecutore.** I suoi banchi —
`arbiter_admission.rs`, `arbiter_policy.rs`, `arbiter_resource.rs` e il modulo `#[cfg(test)]` di
`src/arbiter/mod.rs` — lo esercitano su **stati costruiti a mano**, e `crates/daemon/src/main.rs`,
la radice di composizione, costruisce `Arbiter` ed `Executor` e chiama `admit` da `fn reserve`,
fuori da ogni attività. `crates/simulator/tests/arbiter_campaign.rs` è il posto in cui l'arbitro
gira **dentro** l'esecutore, e porta le proprietà DST **1**, **4** e **5** di §5.7. Le altre due
non girano dentro l'esecutore: la **2** vive in `crates/simulator/tests/worker_kill_campaign.rs` e
la **3** in `crates/simulator/tests/gui_death_campaign.rs`, e nessuna delle due costruisce un
`Executor`.

Lo scenario che il piano dettava — riprodotto rosso prima di cambiarlo, voci `E142`, `E143` ed
`E144` — con le cifre di prima e di dopo, la tabella della chiusura dello spazio degli esiti per
numero di semi, e la campagna di mutazione `M1`…`M9` stanno in
[archivio](archivio/porta-di-qualita-storico.md), sezione
«La campagna DST dell'arbitro — Traguardo 5, Task 12».

**Lo scenario, e che cosa compra ciascuna scelta.**

| Nello scenario | Che cosa compra |
|---|---|
| `now` viene dall'**orologio virtuale**, letto attraverso `SharedClock` | il commento di modulo dichiara l'iniezione dell'orologio, e l'orologio è **letto davvero** |
| **quattro parti diverse** in taglia, corsia, finestra di validità e ritmo | è la simmetria a rendere l'esito indipendente dal seme: rotta quella, l'ordine decide **chi entra** e quindi **quanto** sta nei libri |
| pausa **diversa** dopo una concessione e dopo un accodamento | il calendario di una parte segue le **risposte** che ha ricevuto, quindi l'istante che passa ad `admit` è funzione del seme e non dell'iterazione |
| ogni concessione torna, e chi rilascia **serve la coda** con `promote` | `release` e `promote` sono il **secondo** e il **terzo** posto in cui i libri si muovono, e `promote` emette concessioni con una guardia sul tetto **tutta sua**: senza, la proprietà 1 copriva il solo `admit` |

⛔ **`SharedClock` non è un orologio su misura:** ogni metodo inoltra al `VirtualReactor` che sta
dentro, quindi ciò che gira è la finta spedita. Esiste perché `Executor::new` prende il reattore
**per valore** e un'attività non ne tiene uno — lo dice `Sleep::until` — quindi senza un
riferimento condiviso l'unico istante che un'attività può nominare è uno che calcola da sé.

**La chiusura dello spazio degli esiti** è il criterio con cui `SHORT_CAMPAIGN_SEEDS` è scelto —
non «il più grande numero tondo sotto il tetto», che insegue una cifra che satura. I tempi di
parete sono un **ordine di grandezza** e non una costante, e nessuna asserzione li tocca: lo
argomenta il commento del passo in `scripts/gate.sh`.

⛔ **Il conteggio degli esiti è proprietà dello SCENARIO e non dei semi che lo campionano** —
gotcha **#24**, ed è ciò che rende `EXPECTED_OUTCOMES` un controllo invece di una scommessa.
Provato pescando i semi in quattro modi, con lo stesso conteggio in tutti: `seed` stesso, e
`seed * K >> 33` per `K ∈ {0xBF58476D1CE4E5B9, 0x94D049BB133111EB, 0x2545F4914F6CDD1D}`.

⚠️ **Lo spazio è limitato dalla FORMA dello scenario, non dalla sua lunghezza** — misurato invece
che scelto: a `REQUESTS` **4** lo spazio è di **7** esiti, a **6** di **8**, a **8** di **9**.

⚠️ **`crash_point` NON entra nel conteggio degli esiti:** le passate che li contano girano con
`CrashingJournal::without_crash()`, e il punto di caduta pesa sulla sola `property_4`. Voce `E149`.

⛔ **Il metodo di una campagna di mutazione.** Ogni mutazione è applicata **una alla volta**,
compilata in un passo **separato** da quello che esegue, e revocata **ripristinando da una copia
byte-esatta** presa prima — mai risostituendo all'indietro (gotcha **#48**) — con `cmp` identico
dopo ogni ripristino e `git status --porcelain` riletto. Il perimetro è l'**intero workspace**. La
**mutazione di controllo** — un solo commento cambiato, nessun rosso — si legge per prima: senza di
lei una tabella di mutazioni non prova niente (gotcha **#48**).

⛔ **Una tabella di mutazioni cita il NOME di ciò che muore, non il numero di riga** — il rimedio di
`E138`: un nome si rompe quando la cosa sparisce, un numero quando qualcuno scrive una riga più
sopra.

⛔ **I due testimoni di `property_5` — `already_collected` e `room_from_expiry` — sono DUE
affermazioni:** la mutazione `M6` (l'ammissione non riscuote più, `release` e `promote` sì) uccide
il secondo col primo verde. Gotcha **#55**.

⛔ **IL MUTANTE VIVO, DICHIARATO E NON NASCOSTO.** `Arbiter::release` chiama `collect_expired(now)`
prima di cercare la concessione nei libri; togliendo quella chiamata (`M9`) **nulla in tutto il
workspace diventa rosso**, perché **ogni turno di ogni parte chiama `admit`**, che riscuote per
primo: quando un ritardatario torna la spazzata l'ha già fatta qualcun altro, e il testimone
`already_collected` resta `> 0`. ⚠️ **[C-S3-6]**

⛔ **Le frasi che quel salto rende false si CERCANO, invece di elencarle a memoria:**
`grep -rnE "collects before it|COLLECTS THE EXPIRED" crates/ --include=*.rs`, e ogni riga che
restituisce si legge **intera** — quelle che parlano di `release` o di *«ogni operazione»*
diventano false sotto la mutazione, quelle che restano **vere** parlano di `admit`. ⚠️
`a_grant_is_collected_at_the_instant_its_window_closes` è la ragione per cui si cerca: il suo doc
afferma la cosa e il suo corpo asserisce **solo** su `admit`, una frase che nessun rosso difende e
che a occhio non si trova.

⚖️ **La ragione per cui `M9` non si pinza è `E30`, e sta nel sorgente.** Il blocco *«AND NO PROBE
PINS THOSE THREE VALUES»* sul doc di `ReleaseError` dichiara che una sonda che asserisse `Err` a
`5_001` congelerebbe la decisione che `E30` mette davanti al proprietario — *«a probe that must be
deleted to take a decision is a vote against taking it»* — e nomina questo mutante come costo
dichiarato. Chi chiudesse la voce aggiungendo la sonda prenderebbe `E30` cancellando un paragrafo
invece di deciderlo. Voce `E151`. ⚠️ **[C-S3-6]**

⚠️ **CIÒ CHE QUESTA CAMPAGNA NON COPRE, dichiarato invece che taciuto.** ① Che il rilascio renda
**esattamente** la riserva lo tiene `releasing_gives_back_exactly_the_reservation`: qui `release` è
esercitato sotto interlacciamento e si asserisce il **tetto**, non l'importo. ② Il **rifiuto** non
è raggiunto — nessun profilo di `PARTIES` chiede più di `TOTAL` — e
`the_scenario_really_makes_the_admission_decide` asserisce `refused == 0`, così un profilo che
crescesse o un tetto che calasse diventano **rossi** invece che silenziosi; il ramo lo tiene
`a_request_larger_than_the_total_is_refused_and_not_queued`. Voce `E145`. ③ La campagna gira
sotto `VramPolicy::Remote`, il **default** di ADR-0006, quindi `ask_back` non parte mai: la revoca
ha le proprie sonde nel modulo `#[cfg(test)]` di `src/arbiter/mod.rs`. ④ L'ordine dentro un turno
— rilascia, promuovi, ammetti — è una **scelta di questo banco** e non una decisione sul ciclo di
orchestrazione: **niente qui asserisce su quell'ordine**, quindi le voci aperte `E51` ed `E53`
restano aperte.

#### Le sonde, per nome

| | |
|---|---|
| **N1** | lo stato pulito passa — è il verde di partenza, non una violazione colta |
| **N2** | una crate **spedita** fuori lista → `I3 violated`, e il rimedio è **TOGLIERE** |
| **N3** | una crate **di build** fuori lista → l'altro messaggio, e il rimedio è **AGGIUNGERLA**. Sono due grafi proprio perché i rimedi sono opposti |
| **N4** | `getrandom` in `platform`, dove ADR-0031 lo ammette: **resta verde**. È la sonda che di solito si dimentica |
| **N5** | un nome di crate con la **maiuscola**: usciva **verde**, un falso negativo su I3. Corretto allargando la classe di caratteri del filtro, con la ragione scritta accanto alla classe |
| **N6** | ⛔ **un manifesto DERIVATO dal lockfile → `cargo tree --locked` fallisce, e lo script lo DICE**; il `Cargo.lock` **non si muove**. È il finding **G-5**: senza il rimedio, tolta la riga di `minicbor` da `crates/kernel/Cargo.toml`, `gate-deps.sh` usciva verde avendo riscritto in silenzio il `Cargo.lock` **tracciato**, cioè misurava un grafo che **nessuno ha approvato** credendo di misurare quello della lista; la guardia di non-vacuità non lo coglieva, perché i due grafi erano non vuoti e diversi. La riproduzione sta in [`riferimenti.md`](riferimenti.md). ⛔ **Ogni sito `cargo` eseguibile dei tre script passa `--locked`**, perché i due script si lanciano anche **da soli**, e un controllo che vale solo passando dal cancello è più debole di uno che vale sempre. La cifra dei siti e il comando che la produce vivono in [`riferimenti.md`](riferimenti.md); ciò che regge qui è la **relazione** — *ogni sito eseguibile passa `--locked`* — provata nelle due direzioni. ⚠️ **Due limiti dichiarati.** (1) Ciò che il ramo d'errore esplicito compra è la **diagnosi**, non il rosso: senza di esso i due grafi restano **vuoti**, coincidono, e la guardia di non-vacuità in fondo al file diventa rossa lo stesso — dicendo però *«la query era stretta»* dove la verità è *«il lockfile è stantio»*, il rosso illeggibile del gotcha **#24**. (2) Il messaggio di **coda** dello script resta generico — *«Read the REMEDY: it is NOT the same for the two graphs»* — mentre per questa classe di guasto il rimedio è **lo stesso** per i due grafi: lasciato invece che ramificato, perché il messaggio per-finding lo dice già giusto, e un ramo in più per una riga di prosa sarebbe sovra-ingegnerizzazione |
| **N7** | ⛔ **la seconda direzione di N6:** `--locked` poteva rendere **rosso uno stato corretto**, se il lockfile committato fosse stato fuori sincrono coi manifesti senza che nessuno se ne accorgesse. Misurato: sullo stato pulito `bash scripts/gate.sh` dà **`GATE GREEN`**, e `git status` è **vuoto dopo la corsa** — il cancello non ha più il lockfile fra i propri effetti. ⚠️ Il verde di N7 **non** prova che il rimedio morda, lo prova N6: le due sonde si leggono in coppia |
| **B1** | `kernel` e `simulator` compilano per il bersaglio senza OS |
| **B2** | `getrandom` in `kernel` → `target is not supported` |
| **B3** | contro-sonda: con `--workspace` il cancello fallirebbe su `platform` con `can't find crate for std` — **motivo giusto, crate sbagliata**. Per questo il comando nomina `-p kernel -p simulator` |
| **B4** | il bersaglio non installato → uscita 1 e messaggio corretto. ⚠️ Vedi sotto: è la via *offline* |
| **S1…S7d · C0 · C5 · C6** | le sonde di §8.6.3, più `S7` · `S7b` · `S7c` · `S7d` e la contro-sonda `C6`, aggiunte chiudendo la §7.1.1: tutte colte, con ripristino byte-identico della spec |
| **R1** | la **finta** onora il contratto — il verde di partenza dal lato `simulator` |
| **R2** | la **vera** onora il contratto: la stessa funzione, gli stessi assert, l'altra implementazione. È ciò che la suite esiste per comprare |
| **R3** | `NullAdvanceLiar`, che risponde `Some` a una scadenza **pari** all'istante corrente → la suite scatta. ⛔ E il test **legge il payload del panic**: verifica *quale* asserzione ha sparato, perché un `is_err()` nudo direbbe «ho colto il null advance» anche se a scattare fosse stata un'altra — gotcha **#15** |
| **R4** | `PastDeadlineLiar`, che onora `deadline == now` e mente su `deadline < now` → scatta sul **solo** caso 2b. ⛔ È la sonda che prova che il caso 2b **non è vacuo**: senza, cancellare l'intero blocco 2b lascerebbe la porta **verde**, e la metà `<` del ramo priva di guardia. Gotcha **#45** |
| **R5** | la plausibilità di `wall_time()` sulla **vera** — `crates/platform/tests/wall_clock_plausibility.rs`: un istante posteriore a una data fissa del passato. Coglie l'orologio fermo a **zero o all'epoca** |
| **J1** | il **doppio in memoria** onora tutte le promesse — il verde di partenza. La metà che discrimina la promessa 7 è la **7b**, che è la **seconda direzione** della 7 e non una regola in più. Le promesse si ricontano sul sorgente, gotcha **#31** |
| **J2** | `SilentJournal`, che risponde `Ok(())` e non scrive → scatta la **promessa 1**. ⛔ È la via **A6** di `boundary.rs` resa eseguibile, ed è la ragione per cui questa suite esiste |
| **J3** | `LastWriteWinsJournal`, che rilegge l'**ultimo** record del passo invece del primo → scatta la **promessa 2**. ⛔ È la forma che una tabella chiavata sul passo ha **per natura**, cioè quella di `redb`: la promessa che la seconda implementazione **non incontra da sola** |
| **J4** | `EmptyInsteadOfMissingJournal`, che riporta l'**assenza** come lettura riuscita di **niente** → scatta la **promessa 3** |
| **J5** | `ShuffledJournal`, che restituisce il giornale **rovesciato** → scatta la **promessa 4** |
| **J6** | `PermissiveJournal`, che accetta un esito **senza intento** → scatta la **promessa 5** |
| **J7** | `UnguardedIntentJournal`, che accetta un **secondo intento** sullo stesso passo → scatta la **promessa 6**. ⛔ **Rotto per assenza e non per menzogna:** ha il proprio archivio e la guardia su `outcome`, e gli manca solo quella su `intent`. Non lava un rifiuto altrui come fa `PermissiveJournal`, o sarebbe lo stesso difetto scritto due volte (gotcha **#45**) |
| **J8** | `EagerPruner`, che pota un passo **in dubbio** → scatta la **promessa 7**, che ha **due** asserzioni: l'intento nudo, e l'intento **con una nota** — una nota non è un esito, e senza il secondo caso una nota archiviata come esito renderebbe potabile un passo in dubbio con l'intero workspace verde (mutazione `M12` del Task 11) |
| **J10** | `DiscardedNoteJournal`, che **controlla la nota e non ne conserva nulla** → scatta la **promessa 8**. ⛔ Fa la verifica, risponde `Ok(())` e non scrive — *valida e poi butta* (gotcha **#45**). `SilentJournal` è il vicino più prossimo e **non è lo stesso**: quello non verifica niente e muore sulla promessa 1. ⛔ **Ed è la forma che un'implementazione vera prende davvero:** `note` è l'operazione più nuova della porta, la più probabile da lasciare `Ok(())` mentre il resto è scritto bene — e ciò che smette di arrivare all'archivio è **il contenuto non fidato con l'etichetta che dice che lo era**, cioè la via **A6** puntata sulla via **A4**. ⚠️ **Sta qui e non fra J8 e J9** perché gli identificatori di questo registro **non si spostano** |
| **J9** | ⛔ **la contro-sonda, in due pezzi.** (a) La **durabilità attraverso la caduta del processo** sta **fuori** dalla suite — `the_memory_journal_does_not_survive_being_dropped` vive in `memory_journal.rs` — perché pretenderla in conformità renderebbe rossa la finta, che è **corretta** (gotcha **#44**). (b) La **mutazione di controllo**: cambiato un **solo commento**, i test restano verdi e `cargo test --workspace --no-fail-fast` dà **zero** rossi. Senza (b) la tabella qui sopra non prova niente — gotcha **#48** |
| **J14** | ⛔ **`StepBlindJournal`, che rilegge IL PRIMO RECORD DELL'ARCHIVIO qualunque passo lo chieda → scatta la promessa 1.** È il finding **T-1**, ed è **un predicato di larghezza**: `e.step == step` tolto alla finta, `stored == step.get()` tolto alla vera. Passava la suite finché i blocchi tenevano un archivio con **un passo solo**, dove *«il record di questo passo»* e *«il primo record che c'è»* sono **lo stesso record**: `read_back` non era mai costretto a **scegliere**. ⚠️ Non perde scritture, non mente sull'esito, non rovescia niente — **non guarda il nome**, che è ciò che il doc della porta chiede per primo (*«re-reads ONE step BY NAME»*) (gotcha **#45**) |
| **J15** | ⛔ **`BlindGuardJournal::on_outcome`, la cui guardia chiede «l'archivio è vuoto?» invece di «questo passo ha un intento?» → scatta la promessa 5.** È il finding **T-2**. ⛔ **Non è `PermissiveJournal` con un altro nome:** quello non ha **nessuna** guardia e lava **ogni** rifiuto, questo ne ha una **sbagliata** e lava solo dove le due divergono, cioè su un archivio **non vuoto** — la differenza che le due direzioni della promessa 5 misurano. ⛔ **Cosa costa se passa:** un esito accettato per un passo **mai aperto**, quindi la riconciliazione legge un `Outcome` di un passo mai eseguito e lo **toglie da un dubbio che non c'era** — la classe di guasto per cui ADR-0007 esiste |
| **J16** | ⛔ **`BlindGuardJournal::on_note`, la stessa guardia cieca su `note` → scatta la promessa 8.** ⛔ **Bugiardo distinto per NECESSITÀ e non per simmetria:** `note` e `outcome` condividono `has_intent` in **entrambe** le implementazioni, quindi una guardia cieca le acceca insieme — ma la suite muore alla **prima** promessa rotta, e un bugiardo cieco su tutt'e due morirebbe sulla 5 lasciando il blocco della 8 **non provato mentre un test afferma il contrario** (gotcha **#45**). Un tipo solo con **due istanze**: due tipi sarebbero **lo stesso difetto scritto due volte** |
| **J13** | ⛔ **`AlwaysInDoubtJournal`, che rifiuta OGNI potatura con la parola giusta → scatta la promessa 7b**, ed è ciò che rende non-vacua la **7**. ⛔ È **l'unico bugiardo del registro che sbaglia dicendo NO** — non distrugge niente e non mente, semplicemente non pone mai la domanda (gotcha **#45**); `EagerPruner` è il suo **opposto** e non il suo gemello. ⛔ **Risponde `StepInDoubt` e non `Missing`, e la scelta è il controllo:** con `Missing` morirebbe sulla promessa **7**, e la 7b resterebbe non provata mentre un test afferma il contrario |
| **J11** | ⛔ **il vincolo che rende sicuro `contains`.** `no_promise_message_is_a_substring_of_another` — nessuno dei messaggi di promessa è sottostringa di un altro, e nessuno è vuoto. I due messaggi di `prune` sono la coppia più vicina dell'insieme: cominciano entrambi con *«a step »*. Senza, un bugiardo colto sulla promessa **sbagliata** soddisferebbe comunque il test che nomina quella giusta, e la suite direbbe `ok` indicando il posto sbagliato. ⚠️ **Provato in due direzioni:** rendere un messaggio un semplice **prefisso** di un altro **non** lo fa scattare — giustamente, perché `contains` non è ingannato da un prefisso condiviso — mentre una **vera** inclusione (`MISSING_MESSAGE` = `"a `note` upon an open step must be"`) lo fa scattare **da sola**, e un messaggio **vuoto** anche |
| **R6** | ⛔ **la contro-sonda che conta, e la si sarebbe dimenticata:** rompendo l'avanzamento dell'orologio di parete del `VirtualReactor`, la conformità **resta verde** e scatta **solo** `crates/simulator/tests/virtual_clock.rs`. È la prova che la suite condivisa non impone alla vera un comportamento della finta — se lo facesse, renderebbe rossa un'implementazione **corretta**. Gotcha **#44** |
| **J12** | la **vera** onora il contratto — `the_real_journal_honours_the_contract` in `crates/platform/tests/journal_contract_real.rs`, la stessa funzione e gli stessi assert di **J1**, l'altra implementazione: è ciò che la suite esiste per comprare, l'analogo di **R2**. `FileJournal` onora tutte le promesse; `prune` è la prima operazione che gli è costata un **cambio di archivio** invece di una riga. ⛔ **La non-vacuità è misurata in TRE direzioni**, perché una sola proverebbe una promessa sola: rotta `FileJournal::read_back` perché risponda sempre `Ok(Vec::new())` muore sulla **promessa 1** col `READ_BACK_MESSAGE`; tolta la guardia sul **secondo intento** muore sulla **promessa 6** col `SECOND_INTENT_MESSAGE`, **dopo** aver superato le cinque precedenti sui propri meriti; rovesciato `replay` muore sulla **promessa 4** col `REPLAY_ORDER_MESSAGE`. La **mutazione di controllo** — un solo commento dentro `FileJournal` — lascia tutto verde. ⛔ **E i due lati sono separati:** con `FileJournal` rotta, `kernel --test journal_contract` resta verde, e nel binario di `platform` restano verdi i test inclusi e cade il solo J12. Se cadessero insieme, i due lati non sarebbero due. Le cifre di quelle misure stanno in [archivio](archivio/porta-di-qualita-storico.md), sezione «Le sonde, per nome» |
| **F1** | una **rinumerazione fra indici esistenti** — `kind` 0 ↔ `effect` 1 → **rosso**, col messaggio che nomina il formato cambiato. ⚠️ La mutazione `payload` 3 → 2, già di `trust`, **non compila** — `error: duplicate index numbers` — quindi non avrebbe mai fatto vedere il controllo scattare, e proprio sull'unico oracolo che non si rigenera |
| **F2** | un campo spostato su un indice **libero** — `payload` 3 → 7 → **rosso**. Due mutazioni per la stessa regola, in due forme diverse: il contro-verso del gotcha **#48** |
| **F3** | ⛔ **le OTTO varianti dei tre enum `index_only` rinumerate UNA PER UNA** su un indice libero → **otto rossi**. È ciò che rende la copertura reale invece di apparente: un record congelato solo ne fisserebbe **tre** — `RecordKind` ha 3 varianti, `EffectClass` 3, `Trust` 2 — e le altre cinque resterebbero tenute da **nulla** |
| **F4** | ✅ un campo **facoltativo** con `#[cbor(default)]` su un indice **libero** → **verde**: i byte congelati non si muovono. È la regola 3 di §4.9.2, e ADR-0036 è confermato dalla misura invece che citato |
| **F5** | ⛔ **la metà che rende F4 non vacua:** `Some(9)` invece di `None` → **22 byte** invece di 21, con `86` e il valore in fondo. Senza di essa, *«i byte non si sono mossi»* sarebbe compatibile con un campo che sul filo **non arriva mai**, e il verde non proverebbe l'additività. Gotcha **#54** |
| **F6** | **controllo**: una parola di un commento di `record.rs` → zero rossi |
| **W1** | `to_be_bytes` → `to_le_bytes` **in entrambi i siti** di `framing.rs` → **rosso**, e ne uccide **DUE**: `the_declared_length_is_four_bytes_big_endian` e `a_frame_with_a_tail_is_refused`. ⛔ **La seconda morte è LEGITTIMA e non un difetto della sonda:** la sonda della coda porta un letterale **big-endian**, `00 00 00 01`, che letto little-endian vale **16 777 216** — quindi il corpo da tre byte è *troncato* e non *con una coda*, e la risposta passa da `TrailingBytes` a `Incomplete`. ⛔ **Il round trip sopravvive**, quindi la sonda dell'ordine prova l'**ordine** e non la simmetria fra i due siti — vedi **W7** |
| **W2** | `if body.len() < declared` → `if false` → **rosso**, e **solo** `a_truncated_frame_is_refused`. È il **troncamento**, che nessun decodificatore CBOR può vedere: la coda non c'è, e il CBOR può essere completo lo stesso |
| **W3** | `if body.len() > declared` → `if false` → **rosso**, e **solo** `a_frame_with_a_tail_is_refused`. È la **coda FUORI dalla busta** |
| **W4** | tolta l'annotazione di stringa di byte da `FromWorker::Fragment` → **rosso**, e **solo** `the_byte_string_annotation_is_measured_and_not_asserted`, col messaggio `encoded … bytes`, che riporta la dimensione della forma **senza** annotazione. ⛔ **Il corpo della sonda NON PUÒ essere tutto zeri, ed è ciò che la rende non vacua:** CBOR codifica `0..=23` in **un** byte, quindi con `[0u8; 4096]` le due forme costano **uguale** ed **entrambe** stanno sotto il limite — la mutazione non ucciderebbe **niente**. ⛔ **Le cifre non stanno qui:** il commento di `the_byte_string_annotation_is_measured_and_not_asserted` in `crates/kernel/tests/worker_wire.rs` porta la misura, il metodo — sonda usa-e-getta da fuori la crate, contro un tipo specchio identico a meno dell'attributo — e la scomposizione byte per byte (gotcha **#31**: una cifra in due case si **toglie**) |
| **W5** | tolto il controllo `decoder.position() != body.len()` da `FromWorker::decode` → **rosso**, e **solo** `junk_inside_the_declared_length_does_not_decode`. È la **coda DENTRO la busta**, dove la lunghezza dichiarata è onesta e a mentire è il corpo — la riga che `Record::decode` porta dal finding **AUD-047** |
| **W6** | in `FromWorker::encode`, `framing::frame(&body)` → `Ok(body)` → **rosso** su **tre**: i due round trip e `a_frame_with_a_tail_does_not_decode`. Senza busta il prefisso di lunghezza è il primo byte del CBOR, e `unframe` legge una lunghezza enorme |
| **W7** | ⛔ **la contro-direzione di W1, ed è metà dell'asserzione:** sotto W1 `a_framed_body_comes_back_exactly` **resta verde**. Se cadesse, la sonda dell'ordine proverebbe la **simmetria fra i due siti** e non l'ordine, e l'ordine tornerebbe indifendibile appena qualcuno li cambiasse insieme — il caso che conta, perché **entrambi i pari vivono fuori da questo workspace** |
| **W8** | ⛔ **la contro-direzione di W5, e prova che i guasti sono DUE e non uno:** sotto W5 `a_frame_with_a_tail_does_not_decode` **resta verde**. Se cadessero entrambe, uno dei due controlli sarebbe **dominato** dall'altro (gotcha **#45**) e la tabella della §3.2 del disegno — *«prenditori diversi»* — sarebbe da riscrivere |
| **W9** | ⛔ **il ramo d'errore di `FromWorker::decode`:** il `map_err` che risponde `WireError::Malformed` diventa un `.expect("decode")` — *il kernel va in panico su un frame malformato che arriva da un worker*. Muoiono esattamente `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode`, le sole sonde che raggiungono quel ramo, e nient'altro nel workspace. ⛔ **Sono due `#[test]` separati e non uno con due asserzioni:** una sonda sola si fermerebbe alla prima asserzione e il secondo ingresso non verrebbe mai esercitato — gotcha **#14** |
| **W10** | `if bytes.len() < LENGTH_WIDTH` → `if false` in `unframe` → **rosso**, e **solo** `bytes_shorter_than_the_prefix_are_refused`. ⛔ **Uccide per PANICO e non per asserzione**, ed è la forma di questa guardia: senza di essa `bytes.split_at(LENGTH_WIDTH)` è chiamata su meno di quattro byte — `mid > len`. ⚠️ **Non è il troncamento di W2:** lì la lunghezza dichiarata c'è ed è onesta, qui **non c'è affatto** |

#### Lo schema del canale `ipc` — Traguardo 6, Compito 4, e NESSUNA riga di catalogo si muove

⛔ **Questa sezione registra un MECCANISMO e non una riga del catalogo, e dirlo è la metà che si
dimentica:** nessuna riga di §7.4.1 o §7.4.2 nomina la **§6.1**. Lo misura, delimitando per
intestazione come il gotcha **#26** prescrive:

```bash
awk '/^#### 7\.4\.1/{f=1} /^#### 7\.4\.3/{f=0} f' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | grep '§6\.1\([^0-9]\|$\)'
```

⚠️ **La classe finale del filtro non è decorazione:** senza, `grep '§6\.1'` rende **righe di
§6.10**, un'altra sezione di cui `§6.1` è il prefisso.

⚖️ **E una voce si REGISTRA invece di prenderla.** Un caso `compile_fail` che tenesse *«un verdetto
non può portare un `Grant`»* sarebbe una **riga di catalogo nuova**, cioè §7.4, cioè **spec** —
vincolo globale 7, del **proprietario**. Quella proprietà è tenuta dal **fatto che
`Verdict::Granted` è unitario** e dal doc accanto: è **livello 1 per costruzione e non per un caso
negativo**. Si verifica col comando che esprime il criterio —
`grep -rnE "\bGrant\b" crates/kernel/src/wire/` — che rende **solo righe di commento**, e i commenti dicono **perché**.
⚠️ Senza i confini di parola lo stesso `grep` rende anche `GrantRequest` e `Granted`, che non sono
quel tipo.

**Le sonde** vivono in `crates/kernel/tests/ipc_wire.rs`, **fuori dalla crate** come le gemelle di
`framing.rs` e `worker_wire.rs` — se `IpcMessage` e `framing::frame` non fossero abbastanza `pub` il
banco non compilerebbe. ⚠️ **Quante siano lo dice il comando** e non questa riga (gotcha **#31**):
`grep -c '^#\[test\]' crates/kernel/tests/ipc_wire.rs`.

⛔ **Il banco porta le due sonde della busta onesta**,
`an_empty_body_in_an_honest_envelope_does_not_decode` e
`a_truncated_body_in_an_honest_envelope_does_not_decode`: senza di esse il ramo
`map_err(|_| WireError::Malformed)` di `IpcMessage::decode` non sarebbe raggiunto da **nessuna**
sonda del workspace — è **W9** rifatta sul canale nuovo, e la mutazione **G4** lo misura.

📌 **Le mutazioni portano la lettera `G` come la gui è il pari di questo canale**, per simmetria con
le **W** del canale verso i **worker**. ⚠️ **Non sono il finding `G-5`**, che porta il trattino e
vive in un'altra tabella.

| Mutazione | Che cosa uccide, sull'intero workspace — e che cosa NO |
|---|---|
| **G1** | in `IpcMessage::encode`, `framing::frame(&body)` → `Ok(body)` → **rosso su tutto ciò che attraversa la busta, e su nient'altro**. ✅ **La sola sopravvissuta è l'unica che non passa da `encode`**, `an_empty_body_in_an_honest_envelope_does_not_decode` |
| **G2** | ⛔ **L'ORACOLO DI QUESTO COMPITO, e ciò che NON muore è metà dell'asserzione.** L'encoder scrive **sempre** il discriminante della prima variante — `body[0] = 0` subito dopo la codifica, perché col derive il tag lo scrive la macro e non c'è una riga da girare → muoiono `a_verdict_survives_the_round_trip` e `a_queued_verdict_survives_the_round_trip`, e nient'altro. ✅ **`a_grant_request_survives_the_round_trip` RESTA VERDE**, perché la sua è già la prima variante. È la **sola** prova che il corpo è davvero **un'enumerazione** e non un tipo solo travestito, ciò per cui §6.7 chiede **due** messaggi: se morisse **anche** il giro della richiesta, il discriminante non starebbe distinguendo niente |
| **G3** | tolto il confronto `used != body.len()` da `IpcMessage::decode` → muore **solo** `junk_inside_the_declared_length_does_not_decode`: la **coda DENTRO la busta**. ✅ **`a_message_with_a_tail_does_not_decode` RESTA VERDE**: è la coda **FUORI** dalla busta, che prende `framing::unframe`. I due guasti hanno **prenditori distinti**, la coppia che **W5** e **W8** misurano sul canale gemello |
| **G4** | ⛔ **il ramo d'errore di `decode`**: `.map_err(\|_\| WireError::Malformed)?` diventa `.expect("decode")` — *il kernel va in panico su un frame malformato che arriva dalla gui* → muoiono `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode`, **e nient'altro**. ⛔ **I morti sono DUE perché sono due `#[test]` separati** (gotcha **#14**) |
| **G5** | ⛔ **la variante di mezzo del verdetto:** l'encoder scrive `Verdict::Queued` col discriminante di `Granted` — `body[1] = 0` quando il messaggio è `IpcMessage::Verdict(Verdict::Queued)`, la forma di **G2** un livello più sotto → muore **solo** `a_queued_verdict_survives_the_round_trip`. ⛔ **Il «e NON» qui è TUTTO IL RESTO DEL WORKSPACE:** la sonda distingue qualcosa che **nient'altro** distingue, ed è ciò che la rende non vacua — `Refused` fa il giro completo, `Granted` è codificata da due sonde, e senza di lei `Queued` non raggiungerebbe mai il filo |

Le quaterne misurate e i messaggi `left:`/`right:` di ciascuna mutazione stanno in
[archivio](archivio/porta-di-qualita-storico.md), sezione
«Lo schema del canale `ipc` — Traguardo 6, Compito 4, e NESSUNA riga di catalogo si muove».

⛔ **Revoca verificata con `cmp` contro la copia presa PRIMA** (decisione **D7**), non col diff:
`crates/kernel/src/wire/ipc.rs` non era ancora tracciato, e su un file **non tracciato**
`git diff --stat` è **vacuo**. Il resto del metodo è quello della sezione
«La campagna DST dell'arbitro — Traguardo 5, Task 12».

#### Il build script — entrato su una lacuna misurata

📌 **Regola di lettura:** quante siano le voci di livello 2 lo dice **la tabella**, che è la loro
casa unica, e nessuna sezione narrativa ne tiene un conteggio o un ordinale (precedente
**AUD-007**, gotcha **#31**). La cronaca di come la riga è entrata, e del conteggio tolto con
**AUD-063**, sta in [archivio](archivio/porta-di-qualita-storico.md), sezione
«Il build script — entrato su una lacuna misurata».

Il perché del controllo sta per esteso nel commento di `scripts/gate-attributes.sh`: un
`crates/kernel/build.rs` che chiama `SystemTime::now()`, `fs::metadata()` e `env::var()` e inietta
il risultato con `cargo:rustc-env` lasciava la porta **verde su tutti i controlli**, ciascuno per
la propria ragione, e viola **`I3` e `V29`** direttamente — `cargo:rustc-env` più `env!()` cuoce
nel kernel un valore letto dal mondo alla build, il gotcha #28.

**Il rimedio è TOGLIERE**, come per il grafo spedito, e il messaggio dello script lo dice insieme al
perché — un controllo che sembra pedanteria viene aggirato. Vive dentro `gate-attributes.sh`
perché è **il punto cieco di quello script**: `build.rs` ha attributi propri e il `forbid` di
`lib.rs` non lo raggiunge.

#### Tre note sui limiti dichiarati di questi controlli

📌 **`gate-attributes.sh` è un controllo di testo, e non va promosso.** Cerca gli attributi con
`grep` ancorato a inizio riga: prova che il divieto sia **dichiarato**, non che nel kernel non ci
sia `unsafe` — quella la prova il compilatore, ed è proprio ciò che questo controllo tiene in
piedi. Costo dichiarato nello script stesso: l'ancora chiude il caso `// #![forbid(...)]`, che è
quello reale, ma un attributo sepolto in un commento di blocco `/* … */` sfugge ancora — chiuderlo
richiederebbe un parser, cioè un rimedio più fragile del buco.

⚠️ **B4 è la via _offline_, non una rete di sicurezza — e la differenza va detta.**
`rustup target list --installed` **riconcilia `rust-toolchain.toml` prima di rispondere**: il
manifesto dichiara `targets = ["x86_64-unknown-none"]`, quindi se il bersaglio manca l'atto stesso
di interrogarlo lo **reinstalla**, e la risposta è «c'è». Con la rete disponibile la guardia **non
può scattare**; scatta quando la riconciliazione fallisce, cioè **senza rete** — verificato: uscita
1 e messaggio corretto. È la via che rende utilizzabile una macchina isolata, non un controllo che
sorveglia la macchina connessa.

**Contro-sonde:** la colonna «deve restare verde» qui sopra, più
`crates/platform/tests/counter_probes.rs` al livello 1. ⚠️ Il punto che si dimentica è **perché**
`platform`, `secrets` e `daemon` restano verdi: non sono fra le crate vincolate, e un controllo che
scattasse anche lì sarebbe rosso per il motivo sbagliato (gotcha #24) — `platform` è **il posto
dove l'I/O deve vivere**.

#### Gli altri test che «cargo test --workspace» porta

**Cosa difende ciascuno.** ⛔ Stanno qui perché il gotcha **#36** ha una forma pura e silenziosa:
chi scrive un controllo lo considera «scritto» e non lo riporta nel registro, e da fuori resta
**indistinguibile da uno che non esiste**. Nessuno di questi è una riga del catalogo — sono
controlli di livello 2 che sostengono righe del catalogo, o che tengono in piedi ciò che le righe
presuppongono. ⚠️ Quanti test porti ciascun file lo dice il comando e non questa tabella (gotcha
**#31**): `grep -cE '^\s*#\[test\]' <file>`, nella forma ancorata — quella nuda conta anche le
menzioni nei commenti.

| File | Che cosa difende |
|---|---|
| `crates/kernel/tests/executor_determinism.rs` | **C1, C2 e C3 sull'esecutore _spedito_**, non su quello dello spike: **cento** corse allo stesso seme danno una traccia sola, **duecento** semi distinti non ne danno una sola, e il tempo virtuale **non attende** — l'orologio si ferma a 20 000 ms dove il sequenziale arriverebbe a 60 000. Più le sonde di **non-vacuità**: che l'interfoliazione sia reale, che un blocco diventi **errore e non attesa infinita**, che un reattore che non avanza sia **errore e non giro a vuoto**, che un'attesa già scaduta svegli subito senza muovere l'orologio, che una richiesta di sospensione **non si erediti** fra attività, e che un rideposito perpetuo di una scadenza passata **termini comunque** — fra queste le tre delle decisioni **K-1** e **B-1** dell'audit, `a_request_written_before_the_run_belongs_to_nobody`, `a_request_written_by_a_destructor_belongs_to_nobody` e `the_delivered_turn_limit_is_honoured_by_its_value` |
| `crates/kernel/tests/ports_are_implementable.rs` | il rimedio al gotcha **#46**: una **finta** per `Filesystem`, una per `Network`, due per `process` — `Worker` e `Process` — e una per `Ipc`, con chiamate che le esercitano in entrambe le direzioni. **Cinque finte per quattro famiglie**, la copertura di **tutte** le porte dichiarate senza implementazione. È ciò che tiene in vita `Path::as_bytes()`, `Endpoint::as_bytes()` e il `Clone` su `Path` contro una passata YAGNI — su un tratto dichiarato **in anticipo** i chiamanti sono vuoti per costruzione, e il criterio non distingue il morto dalla sola porta d'ingresso di chi verrà — e prova che quelle firme siano **implementabili fuori dalla crate**, dove la privacy di modulo di una tuple-struct le renderebbe inutilizzabili. ⚠️ **Non** è una suite di conformità: quella pretende due implementazioni da confrontare |
| `crates/simulator/tests/memory_journal.rs` | il **doppio in memoria** del giornale (§4.1): che l'intento riletto torni intatto, che un passo mai scritto sia **`Missing` e non vuoto**, che un esito **senza** intento sia rifiutato **e** uno **dopo** il proprio intento accettato, che il rifiuto guardi **quale** passo, che `read_back` risponda con l'**intento** e non con l'esito, che ogni passo rilegga **il proprio** primo record, che `prune` **rifiuti un passo in dubbio senza potarlo** e risponda **`Missing`** a un passo mai scritto — la **terza** risposta, la sola che la conformità non tiene (voce aperta 2). ⚠️ **Non** è la suite di conformità: `journal_contract.rs` porta ciò che **entrambe** le implementazioni promettono, questo file ciò che è vero **di questa sola**. Sono: **due note su un passo** — `a_step_may_carry_many_notes_and_they_keep_their_order`; l'ordine dentro un passo in sé è **dovuto dalla porta** (`crates/kernel/src/ports/journal.rs`, *«Re-reads EVERYTHING, in write order»*) e tenuto dalla conformità, promessa 8c, ma la conformità non esercita due note sullo stesso passo; che il rifiuto del **secondo intento** sullo stesso passo **non abbia scritto** — `a_second_intent_on_the_same_step_is_refused`, dove `read_back` risponde ancora col primo, con la contro-sonda `an_intent_for_another_step_is_still_accepted`; il rifiuto in sé è la promessa **6** della conformità; e che il giornale **non sopravviva alla propria caduta** — `the_memory_journal_does_not_survive_being_dropped` |
| `crates/simulator/tests/crashing_journal.rs` | ⛔ **il giornale che CADE** (§3.3, livello 1 dei due livelli di crash di ADR-0032) — ciò che promette **solo lui**, ed è per costruzione un bugiardo, quindi la conformità non lo tiene (gotcha **#50**): che cada **alla** scrittura dichiarata e non «da qualche parte», che dopo la caduta **ogni** scrittura successiva sia rifiutata — la differenza fra un **crash** e un **disco cattivo**, decisione D2, ed è la permanenza a fermare *tutte* le attività interlacciate — che ciò che era stato scritto **sopravviva** e si rilegga da `into_survivor`, che `has_fallen` dica **no** finché non cade, e che un giornale a cui è stato detto di **non** cadere non cada mai in centoventotto scritture: la direzione che si dimentica, e ⛔ **`C7a` poggia interamente su di lei** — è l'unica sonda **verde** sotto la mutazione «cade sempre» e **rossa** sotto «non cade mai». Più: che una scrittura **rifiutata dal protocollo** non consumi una posizione del conteggio — senza, il punto estratto scivola e con un punto vicino alla fine il guasto **non scatta**, gotcha **#17** — e che `prune`, **unica operazione mutante fuori dalle tre contate**, sia rifiutata dopo la caduta **senza armarla né consumarla**. Le sonde dei finding **S-1** e **S-2** sono `the_success_path_of_note_is_exercised_and_counted`, `a_refused_note_does_not_consume_a_crash_position` e `a_refused_intent_does_not_consume_a_crash_position`. ⚠️ **Una sonda è dichiarata non falsificabile invece che tolta:** `the_same_seed_chooses_the_same_write` non può fallire perché `from_seed` è pura, e la determinatezza vera la tiene `seeded_rng.rs` — postura del gotcha **#44** |
| `crates/simulator/tests/arbiter_campaign.rs` | ⛔ **la campagna DST dell'arbitro** — le proprietà **1**, **4** e **5** di §5.7 sotto un interlacciamento scelto dal seme, l'unico posto in cui l'arbitro gira **dentro** l'esecutore. Due oracoli di non-vacuità distinti — *«lo scenario fa davvero decidere l'ammissione»* e *«la campagna spazza più di un mondo»*; il resto nella sezione dedicata qui sopra |
| `crates/simulator/tests/dying_gui.rs` | ⛔ **ciò che promette SOLO `DyingGui`**, sul precedente di `crashing_journal.rs`: muore all'operazione detta, resta morta su `send` **e** su `receive`, un client morto non viene ripresentato da `accept`, ciò che ha detto prima di morire è una richiesta **davvero codificata**, un'operazione nominata su un altro client non consuma una posizione, una finta detta immortale non muore mai, stesso seme stesso punto, il punto estratto sta **dentro** le operazioni del percorso, **ogni** operazione può essere quella, e `has_died` dice no finché non muore. ⚠️ `the_same_seed_chooses_the_same_operation` è **infalsificabile per costruzione** e lo dichiara (gotcha **#44**): non conta come copertura |
| `crates/simulator/tests/gui_death_campaign.rs` | ⛔ **la proprietà 3 di §5.7** — la gui muore tenendo una concessione discrezionale e la somma torna alla linea di base. La morte si legge **solo** come `Err(IpcError::Disconnected)` attraverso la porta, mai chiedendo alla finta — `grep -cE '\.has_died\(' crates/simulator/tests/gui_death_campaign.rs` dà **0**, cioè le **chiamate**; ⛔ **il comando NON ancorato è un'altra misura e rende un numero maggiore di zero**, perché conta anche i paragrafi che dichiarano di non chiamarla. Linea di base **non zero** e di **due specie**: la quota di presentazione del core, che nessun registro tiene, e un **secondo client** registrato che non muore. L'oracolo *«il guasto è scattato»* è l'`assert!` **per seme** dentro `run`, che vale su ogni seme invece che sulla somma |
| `crates/simulator/tests/worker_kill_campaign.rs` | ⛔ **la proprietà 2 di §5.7** — nessun processo gira senza concessione valida, asserita **sui libri** dopo **ogni** kill e non alla fine. Quattro worker avviati con concessioni vere, uccisi in istanti estratti da una mescolanza **derivata**. ⛔ **Tre controlli di non-vacuità, VISTI ROSSI tutti e tre, e la provenienza delle mutazioni non è la stessa:** *«i kill sono scattati»* — nessuna recluta sopravvive al proprio kill — con `MC3`, che muta **il banco**, perché `order` è una permutazione e nessuna mutazione di produzione lo raggiunge (dichiarato accanto all'asserzione); *«c'era qualcosa da verificare»* — almeno un `Released::Now`, cioè un kill **dentro** la finestra — con `MC1`, e *«il seme conta»* con `MC2`, entrambe sul **generatore** (`RngExt::below`, codice di produzione). ⚠️ La metà **temporale** di *«concessione valida»* si **conta e si dichiara** e non si asserisce — `E30`/`E39`, voce del proprietario, gotcha **#73** ⚠️ **[C-S3-6]** |
| `crates/simulator/tests/dst_campaign.rs` | ⛔ **la campagna DST di livello 1** (§3.3, ADR-0032): il soggetto sotto esame è la **riconciliazione del kernel**, e nulla tocca un disco. Consegna lo **scenario giornalato** — tre attività per quattro passi, con intento ed esito attraverso la porta — la **traccia**, oracolo **indipendente** di `C7b` perché viene da ciò che le attività hanno saputo essere passato e non dall'archivio, e **`C7a`**: senza crash, **nessun** passo in dubbio, su cinquanta semi. Più il pin che **fissa** `WRITES_PER_RUN = 24`, senza il quale il punto di caduta si estrarrebbe contro un numero non verificato (gotcha **#17**). ⛔ **L'oracolo di non-vacuità di `C7a`:** *«nessun passo è in dubbio»* e *«lo scenario non ha scritto niente»* sarebbero **lo stesso verde**, quindi il ciclo pretende `writes_done() == WRITES_PER_RUN` **prima** di guardare l'insieme — il gemello di `has_fallen()` sull'altra metà della campagna. ⚠️ **Che lo scenario interlacci lo tiene `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed`**, e il **nome** è un vincolo, perché il doc di `run` lo cita. **`C7b`**: il crash lascia **quell'insieme e non un altro**, confrontato con l'oracolo `expected_doubt` che viene dalla **traccia** e non dall'archivio — la sola ragione per cui `C7b` non è una tautologia — con un **confronto ordinato** invece che insiemistico, che morde davvero: un `replay` ordinato per passo dà `left: [0, 4]` contro `right: [4, 0]`, il difetto che una tabella `redb` chiavata sul passo produrrebbe da sola. ⛔ **`C7b` ha DUE oracoli di non-vacuità, perché provano cose diverse:** che **ogni** seme raggiunga il proprio punto di caduta — uguaglianza e non `> 0`, perché un seme che non cade significa che lo scenario ha scritto meno del numero contro cui il punto è estratto — e che **almeno un seme lasci più di un passo in dubbio**, senza cui la campagna confronterebbe insiemi vuoti restando verde. ✅ **`C7b` È la campagna breve**: il corpo per-seme vive in `campaign(seeds)`, che la campagna profonda riusa sotto `#[ignore]`, e non esiste un secondo ciclo più debole accanto. ⛔ **Il numero di semi ha una TERZA guardia**, il criterio con cui è stato scelto: gli insiemi in dubbio distinti che questo scenario può produrre sono **centonove**, e la campagna pretende di vederli **tutti** — `EXPECTED_DOUBT_SETS`. Non è una proprietà ma un **rilevatore di cambiamento sulla forma dello scenario**, nella postura dei byte congelati, adottato solo dopo aver misurato che non scattasse dove non deve: sei costanti di mescolamento diverse danno lo stesso conteggio, quindi è dello **scenario** e non dei semi. Provato in due direzioni — a cinquecento semi ne vede centocinque e scatta |
| `crates/kernel/tests/reconciliation.rs` | la **riconciliazione** (§4.3, ADR-0007) — il primo consumatore di `replay()`: che un crash lasci **più** passi in dubbio e non uno (gotcha **#20**, `[3, 7]` col seme 99), che un passo con intento **ed** esito **non** sia in dubbio (la direzione che si dimentica), che la **classe decida** la risoluzione sui tre valori, e che un record indecifrabile valga `SuspendAndAsk`. Più il giornale **vuoto** — il primo avvio — l'**ordine di scrittura** scritto `7, 3, 1`, perché `[3, 7]` è ordine di scrittura **e** ordine numerico insieme, e le **due dell'insieme**: al più una voce per passo, e un passo che rientra **conserva il posto**. ⛔ **Ciò che questo file NON tiene, ed è dichiarato in `reconcile.rs`:** che il `kind` del record concordi con l'operazione che l'ha scritto. La questione delle **due verità** è **chiusa dal proprietario** — come **decisione** e non come garanzia — e a valle esiste `the_promotion_writes_through_note_and_the_record_says_note` |
| `crates/platform/tests/file_journal.rs` | ⚠️ **il conteggio dei test dipende dal sistema**: una sonda è `cfg(unix)`. ⛔ **È `the_journal_file_is_not_world_readable`, finding PL-1.** ADR-0023 promette che il giornale a riposo sia *«protetto quanto il tuo account di sistema»*, e `OpenOptions::create(true)` da solo chiede `0o666 & !umask`, cioè **0644** su un Linux di serie: **leggibile da chiunque**. ⚠️ **L'asserzione è «nessuno tranne il proprietario» (`mode & 0o077 == 0`) e non «esattamente 0600»**, perché `mode()` è ancora mascherato dall'umask: un'uguaglianza esatta andrebbe **rossa su un sistema più chiuso del richiesto**, cioè dove la promessa è **mantenuta**. La direzione «deve scattare» è provata dalla misura del sistema (senza la riga il file nasce **644**, e `644 & 0o077 = 0o044 ≠ 0`), non da una corsa del banco mutato. ⛔ **Il difetto è INVISIBILE sull'host di sviluppo** — Windows non ha il modo Unix, `cfg(unix)` lo compila via, e il rosso può uscire **solo sul secondo sistema previsto dal progetto**: gotcha **#52**. Il percorso Unix è **type-checkato** con `cargo check --target x86_64-unknown-linux-gnu --tests`; il **valore** lo misura la CI. ⚠️ **Limite dichiarato:** `mode()` è ignorato se il file **esiste già**, quindi un giornale creato prima resta 0644 — è una **migrazione**, e la fixture cancella la cartella all'ingresso, quindi questa sonda **non può vederla**. ⛔ **Solo il file e NON la cartella**, scelta del proprietario: `0700` sulla cartella coprirebbe anche gli archivi futuri, ma **la cartella non ha un proprietario nel codice** — nessuno la crea — e la regola nominerebbe un chiamante che non esiste, il difetto di **A-7**. ⚠️ **Non è una riga di catalogo:** aggiungerla alla §7.4 è del proprietario, e finché non c'è questa sonda è **registrata qui come voce aperta** — gotcha **#36**. — Le altre sono ciò che **solo** il giornale su file promette (§4.1, ADR-0032), e che pretenderlo in conformità renderebbe rossa la finta — gotcha **#44**: che una scrittura **sopravviva alla riapertura**, che una transazione **mai confermata non lasci nulla** (requisito 1 di §10.6), che il contatore delle chiavi **riprenda dall'archivio** invece che da zero — altrimenti la seconda sessione **sovrascrive** la prima in silenzio — che la guardia sul **secondo intento** regga **attraverso una riapertura**, perché legge l'archivio e non un campo della sessione, e che il **lucchetto** rifiuti un secondo giornale sullo stesso file **mentre il primo è aperto** (l'altra direzione la tiene la prima sonda: chiuso il primo, la riapertura riesce). ⛔ **E una prova che il confine è reale:** `CountingBackend` è una **seconda implementazione di `redb::StorageBackend` scritta da fuori la crate**, `FileJournal` ci gira sopra invariato, e i contatori dicono che l'I/O **passa davvero di lì** — senza quell'asserzione un giornale che accettasse il backend e scrivesse altrove resterebbe verde. È il rimedio al gotcha **#46** applicato al confine su cui si iniettano i guasti di livello 2. ⚠️ **Non** è la suite di conformità: quella sta in `journal_contract.rs` e gira contro **entrambe** dalla riga qui sotto |
| `crates/platform/tests/journal_contract_real.rs` | ⛔ **la conformità della porta `journal` contro l'implementazione VERA**, e il file è corto perché **le asserzioni non si ripetono**: `include!("../../kernel/tests/journal_contract.rs")` le raggiunge testualmente, come `reactor_contract_real.rs` fa per `reactor`. Due copie divergerebbero, e **la prima che diverge mente stampando `ok`**. ⚠️ **Costo dichiarato:** l'inclusione porta con sé anche i `#[test]` del file incluso, quindi la finta, i bugiardi e la sonda delle sottostringhe **girano una seconda volta** dentro il binario di `platform`, e **uno solo** tocca il disco. ⛔ **La fabbrica dà un file NUOVO a ogni chiamata** invece di cancellarne uno fisso: su Windows la cancellazione **fallisce in silenzio** se il file è ancora aperto e la fabbrica riaprirebbe **i dati vecchi** (gotcha **#52**), `FileJournal` tiene un **lucchetto esclusivo**, e la promessa 4 conta l'**intero** archivio. La numerazione passa da un `AtomicU64` perché `assert_journal_contract` prende **`Fn`**, non `FnMut`. La cartella è **una per call site**, dal `line!()`, con un **prefisso diverso** da quello di `file_journal.rs`: i due binari girano insieme e un numero di riga è unico dentro **un** file solo. Sonda **J12** |
| `crates/platform/tests/engine_crash_consistency.rs` | ⛔ **il LIVELLO 2 dei due livelli di crash** (ADR-0032, §4.6): il soggetto sotto esame **non è il kernel** ma **`redb` stesso**, guidato attraverso un `StorageBackend` che cade a un'operazione scelta. ⛔ **Vive in un banco di prova e non in `platform/src/`, ed è il punto:** quel confine dev'essere raggiungibile **da fuori la crate** (gotcha **#46**), e un backend cadente scritto **dentro** `platform` non proverebbe nulla su quello. Tiene: che senza caduta l'archivio **si riapra con tutto dentro** — la direzione che si dimentica, messa **per prima**, perché se cadesse ogni rosso successivo parlerebbe del backend invece che dell'iniezione — che il backend **cada all'operazione dichiarata** e non prima, e che la caduta sia **permanente**. ⛔ **L'oracolo che chiude il gotcha #51 è `the_engine_really_syncs_and_that_is_what_closes_gotcha_51`, ed è un DELTA e non un conteggio:** sei sync su sette nascono **prima che esista un record**, quindi *«`sync_data` è stato chiamato»* è soddisfatto da un motore che **non sincronizza nessuna scrittura** — misurato, con `Durability::None` la forma assoluta resta **verde** e quella a delta va rossa. ✅ **Tiene anche la coerenza dopo la riapertura, con QUATTRO oracoli:** che quel che torna sia un **prefisso** di quel che è stato scritto — mai un record parziale o mescolato — che **ogni** punto scatti (uguaglianza e non `> 0`, perché l'intervallo si ferma alla **saturazione**: oltre, la corsa è indistinguibile da una senza iniezione) · che **almeno un'iniezione abbia accorciato l'archivio**, senza cui il confronto a prefisso è **banalmente vero** · che **non le abbia accorciate tutte** · e ⛔ **che esistano punti che restituiscono ALCUNI ma non TUTTI i record**: è ciò che rende il ciclo un **secondo testimone del gotcha #51** — senza durabilità la scala collassa a **zero-o-tutto** — e non dipende dalla costante fragile, perché non conta operazioni ma gradini. ⚠️ **Ciò che la chiusura del #51 NON compra** — la morte vera del processo, l'ordine fra `write` e `sync_data`, il commit di `prune`, e un supporto che possa davvero perdere una scrittura non sincronizzata — è scritto per esteso in [`riferimenti.md`](riferimenti.md), perché *«il #51 è chiuso»* nella forma nuda mentirebbe. ✅ **I controlli vivono in un CORPO SOLO che due profondità chiamano**, invece che in una campagna nuova accanto alla vecchia. ⛔ **La campagna profonda approfondisce lo SCENARIO e non lo spazzamento:** allargare l'intervallo **non compra niente**, perché oltre la saturazione i punti che scattano non crescono; la profondità invece compra stati nuovi **uno per record** — le lunghezze di prefisso distinte sono `record + 1` a ogni profondità misurata — e il testimone del **#51** non è un accidente dello scenario piccolo |
| `crates/kernel/tests/dependencies_usable.rs` | che le voci **spedite** dell'allow-list **compilino e facciano round-trip** — gotcha #22, `cargo add bincode` risolve a una versione il cui intero sorgente è un `compile_error!`. E per `bincode` i **byte consumati** sono pari alla lunghezza dichiarata, la regola imposta dal gotcha **#34**: un decodificatore che si ferma al primo elemento completo e ignora la coda «ha decodificato» senza provare niente |
| `crates/kernel/tests/ipc_wire.rs` | lo **schema del canale `ipc`** (§6.1, ADR-0037) — `crate::wire::ipc`, la busta di `crate::framing` che porta **un enum a due varianti**, una per direzione. Tiene i **giri completi delle due direzioni** — le **due varianti** ci sono perché con un tipo solo il **discriminante** non sarebbe provato — la **coda fuori** dalla busta e la **coda dentro** la lunghezza dichiarata, guasti distinti con prenditori distinti, e il **corpo vuoto** e il **corpo troncato** dentro una busta onesta, i due soli ingressi che raggiungono il ramo d'errore di `IpcMessage::decode`. ⛔ **NON è una riga del catalogo**, e non ne muove nessuna: la sezione *«Lo schema del canale `ipc`»* qui sopra porta il comando che lo misura, le mutazioni **G** e il loro esito |

#### Le finte delle porte — cosa hanno colto, e cosa hanno potato

⛔ **Sulla porta `process` la finta ha colto un difetto:** `SingleReceipt` e l'altra ricevuta hanno
`new` e `id` perché `instruct_one` deve **restituirne** una, e un campo `pub(crate)` non si
costruisce da fuori la crate — il gotcha **#46** nella forma peggiore, non «non riesco a **leggere**
un campo» ma «non riesco a **produrre** il valore di ritorno». La ragione sta accanto alle ricevute,
in `crates/kernel/src/ports/process.rs`.

⚠️ **`Grant` resta senza costruttore, ed è l'opposto deliberato:** §5.6 la vuole inedificabile.
Niente `Debug`: nessuno formatta una concessione; sulle **ricevute** `Debug` si tiene, lo pretende
`unwrap_err`. La trappola della misura — gli errori di rustc si mascherano fra passate: col
costruttore assente esce `E0599`, e sul letterale **nessun** `E0451` — è il gotcha **#47** in
[`HANDOFF.md`](HANDOFF.md).

⛔ **`Clone` potato da `WorkerDescriptor` e da `Frame`, e la contro-sonda è ciò che rende la
potatura difendibile.** Tolti da questi due: **verde, zero warning**. Tolto da `Path` come
contro-sonda: **rosso**, `E0277` · `E0308` · `E0599`. Su `Path` e su `Endpoint` `Clone` è
**portante** — `declare_scope` consegna un **prestito** che l'implementazione deve trattenere —
mentre `WorkerDescriptor` e `Frame` attraversano la porta **per valore**. ⚠️ «Non implementabile
oggi» e «un chiamante lo vorrà domani» sono due forme diverse, e a distinguerle è **la finta**.

⛔ **Sulla porta `ipc` la stessa finta ha comprato una SOTTRAZIONE.** Di `ClientId` sono cancellati
`get()`, `Hash` e `PartialOrd`/`Ord` — tolti, `build` e `test --workspace` restano verdi senza
warning — e tenuti `PartialEq`/`Eq`, `Copy` e `Debug`, ciascuno con la propria contro-sonda rossa
(`E0369`, `E0382`, `E0277`), più `Clone`, che lo pretende `Copy`. Le misure, voce per voce, stanno
in [archivio](archivio/porta-di-qualita-storico.md), sezione
«Le finte delle porte — cosa hanno colto, e cosa hanno potato».

⛔ **Le contro-sonde non sono cerimonia: sono ciò che regge l'argomento.** `get()` è cancellato
*perché* un'implementazione **conserva** un identificativo `Copy` e lo confronta con `==`, come
`InMemoryFilesystem` fa con `CheckpointId` — ma se `PartialEq` non fosse davvero esercitato quel
«perché» poggerebbe sul nulla, che è il modo in cui `SingleReceipt::id` era rimasto in vita senza
copertura. ⚠️ L'argomento **a favore** di `Ord` era reale — il **#12** vieta `HashMap` e spinge su
`BTreeMap` — e cade su un criterio preciso: `Ord` **non blocca** chi implementa da fuori (una
tabella più `==` basta) e si aggiunge dopo in una riga. L'eccezione del **#46** copre *«non
implementabile oggi»*, non *«comodo domani»*. `Hash` è il peggiore dei tre: **abilita la cosa
vietata**.

#### Le passate di mutazione, e le righe che contano

⛔ **Ogni finta porta la propria passata. La prosa è solo per le mutazioni uccise da _un test
solo_:** sono le uniche che dicono qualcosa che la tabella non dice già. Le tabelle delle passate di
`process`, `ipc`, `memory_journal` e `journal_contract` — quest'ultima con la corrispondenza
promessa → bugiardo misurata neutralizzando una promessa alla volta — stanno in
[archivio](archivio/porta-di-qualita-storico.md), sezione
«Le passate di mutazione, e le righe che contano»; il metodo è quello della sezione
«La campagna DST dell'arbitro — Traguardo 5, Task 12».

**`process`.** ⛔ **M6b** — `read_one` che risponde una costante pari all'id atteso, la scelta
**avversaria** — la uccide **solo** `answers_are_correlated_to_the_receipt_that_asked`: la suite
deve tenere **due** ricevute aperte insieme, con lunghezze **diverse**, chiudendo la posizione
**1** (gotcha **#15**); il perché della forma sta nel commento del test, in
`crates/kernel/tests/ports_are_implementable.rs`. ⛔ **M11** — `kill()` che acquista una guardia di
liveness — la uccide `killing_a_worker_consumes_it` uccidendo un worker **già morto**: `kill` la
guardia **non ce l'ha, di proposito** (§5.3 punto 4), e il commento del test lo dice.

**`ipc`.** ⛔ `a_dead_client_does_not_take_the_port_with_it` è l'**unico** uccisore di **M12** — la
morte **contagiosa**: ne muore uno, muoiono tutti — e di **M13** — `accept` **ricicla**
l'identificativo di un client morto. È il test che porta la proprietà per cui la porta esiste, **la
gui è sacrificabile**, e una gui che si riconnette è un **client nuovo** che non eredita
l'identificativo del cadavere: altrimenti un messaggio accodato per il morto verrebbe consegnato al
nuovo arrivato, che per I1 nasce **senza stato proprio** e non potrebbe accorgersene.

⛔ **E una questione resta dichiarata e aperta nel sorgente di `ipc`, sul modello di `network`.**
`accept` non può fallire — `fn accept(&mut self) -> Option<ClientId>` — e per i due modi di fallire
che il vocabolario conosce è **coerente**: `Disconnected` è un'affermazione **su un `ClientId`**, e
`accept` è l'unico metodo che un `ClientId` **non lo prende**; e non decodifica niente, quindi
nemmeno `MalformedMessage` lo raggiunge. ⚠️ Ma un **ascoltatore** rotto — non un client — **non ha
parola** in questo vocabolario, e arriverebbe come `None`, cioè un **valore sbagliato** invece di un
errore (gotcha #30). ⛔ **Ed è _anche_ un'asimmetria fra le firme:** `receive` restituisce
`Result<Option<Vec<u8>>, IpcError>`, dove «niente di pronto» e «rotto» sono già distinti;
l'argomento contro il `Result` confuta solo `Result<ClientId, IpcError>`, non la forma con
l'`Option` dentro, che `receive` **già usa**.

| | |
|---|---|
| **il prezzo vero** | ⛔ aggiungere una terza variante **non chiuderebbe niente**: non c'è dove restituirla. Chiudere il residuo costa **la firma**, non l'enum |
| **perché la firma resta** | `IpcError` non ha **nessuna** variante che `accept` possa restituire: un `Result` che non può mai essere `Err` è **superficie morta**, esattamente ciò che questa porta ha potato in tre derive e un accessore |

**`memory_journal`.** ⛔ **M5 — `has_intent` che ignora il TIPO di voce — non è uccisa da nessuno,
e non va chiusa: distingue uno stato irraggiungibile.** Il primo record di un passo può essere
**solo** un intento, perché `outcome` esige `has_intent`; quindi «esiste una voce per questo passo»
ed «esiste un **intento** per questo passo» sono la stessa affermazione. `prune` non chiede
`has_intent` ma **se esiste un ESITO**, una domanda che `has_intent` non pone: l'equivalenza cadrà
con `has_outcome`, che nessuno ha ancora scritto.

✅ **LA QUESTIONE DEL SECONDO INTENTO È CHIUSA, e la voce resta qui invece di essere cancellata** —
una voce aperta che sparisce non si distingue da una dimenticata. ⛔ **La decisione: un secondo
intento sullo stesso passo è RIFIUTATO.** Presa dal **coordinatore in revisione**, non dal piano, e
dichiarata come tale perché il proprietario possa **ribaltarla vedendola**. Le ragioni, in ordine:

| | |
|---|---|
| **il modello** | ADR-0007 dice *«l'intento di **ogni** passo»* — uno per passo. Un secondo è **fuori dal modello**, non un caso da disciplinare |
| **la simmetria** | è la metà mancante di *«un esito senza intento è rifiutato»*: V6 tenuta dalla **porta** invece che dalla diligenza del chiamante |
| **YAGNI non si applica** | su una porta dichiarata in anticipo i chiamanti sono vuoti **per costruzione**, quindi il criterio non distingue il morto dal portante — gotcha **#46** |
| **il costo** | una riga — `has_intent` c'era già — contro due implementazioni e un archivio dopo |
| **mai scartata prima** | ⛔ cercato dove fosse già stata valutata (gotcha #32): mai valutata, mai scartata |

Perché la sede è la conformità e non `journal.rs` — un'implementazione chiavata **sul passo**
divergerebbe dalla finta **senza che nulla diventi rosso**, perché l'accordo con la promessa **2** è
un accidente del disegno della chiave e non un contratto — e perché nessuna variante d'errore nuova
— `OutOfOrder` si allarga, e le sue vie sono **tre** — sta accanto alla variante in
`crates/kernel/src/ports/journal.rs`, dove vive l'elenco vivo; la §4.1 della spec porta il richiamo
gemello.

**`journal_contract`.** I messaggi di promessa sono `READ_BACK` · `READ_BACK_IS_THE_INTENT` ·
`MISSING` · `REPLAY_ORDER` · `OUT_OF_ORDER` · `SECOND_INTENT` · `PRUNE_IN_DOUBT` ·
`PRUNE_RECONCILED` · `NOTE`, e i blocchi li conta
`grep -c '= build();' crates/kernel/tests/journal_contract.rs`. I bugiardi **J14**, **J15**, **J16** non aggiungono
promesse: provano tre promesse che c'erano già in uno **stato** che la suite non costruiva mai, e le
loro asserzioni stanno **dentro** blocchi esistenti. La corrispondenza promessa → bugiardo sta nel
registro «Le sonde, per nome».

⛔ **Chi rifà la corrispondenza** neutralizza **una** promessa alla volta — avvolgendone i blocchi in
`if false` — e deve veder cadere **esattamente** il test del suo bugiardo e nessun altro: è la prova
che nessuna promessa è decorativa e che nessun bugiardo muore sulla promessa di un altro. Un banco
che non si rifà quando l'insieme cambia riporta l'esito di ieri. ⚠️ **La promessa 8 ha DUE
blocchi** — il passo mai aperto sta in un giornale suo — quindi neutralizzarla ne spegne due, e uno
strumento che cerca un blocco solo darebbe un falso «sopravvissuta». ⚠️ **La 7b si prova togliendo
il BLOCCO INTERO** (`M14b`), non neutralizzando un'asserzione: le sue tre asserzioni portano un
messaggio solo, quindi neutralizzarne una lascia il rosso in piedi e non prova niente — gotcha
**#55**.

⚠️ **Le due misure non si sostituiscono:** neutralizzare i blocchi contro la **finta** dice che
nessuna promessa è decorativa; **J12**, rompendo `FileJournal`, dice che la suite sta davvero
girando contro `redb` e non contro una descrizione di `redb`.

### T-1 e T-2 — le tre promesse provate solo dove ogni guardia passa (2026-08-17)

La domanda che le ha colte è quella del gotcha **#63** in [`HANDOFF.md`](HANDOFF.md): *in quale
altro stato del mondo questa asserzione resterebbe verde?*

⛔ **La suite di `journal` costruisce un PASSANTE:** un passo in archivio **diverso** da quello
sotto esame, l'unico stato in cui la guardia giusta e quella cieca danno risposte diverse. Le due
implementazioni filtravano già per passo — `has_intent(step)` nella finta, `stored == step.get()`
nella vera — e la porta lo dichiarava già; il rimedio a **T-1** e **T-2** non ha aggiunto promesse
né toccato codice di prodotto: è lo stesso contratto, provato.

⚠️ **`assert_caught_on` legge il payload del panic invece di accontentarsi di `is_err()`:** il
rosso di un bugiardo non è «è diventato rosso» ma *«è diventato rosso lì»*, col messaggio della
**propria** promessa.

⛔ **Due scelte di forma della suite, colte dalla misura e non dalla rilettura.** La promessa **4**
confronta i **record**, byte compresi, e non le sole identità dei passi: una sequenza `1, 2, 1` è
un **palindromo**, e un `replay` rovesciato — `ShuffledJournal` — passerebbe. La promessa **1**
mette il proprio messaggio anche sull'`expect` di `read_back`, perché la via **A6** è proprio il
caso in cui `read_back` **non trova**: senza, `a_journal_that_writes_nothing_is_caught` coglierebbe
A6 con un messaggio che non nomina nessuna promessa.

La riproduzione di T-1 e T-2, le mutazioni **B-1**…**B-6** sulle implementazioni vere, la questione
della promessa 7 chiusa dal Task 11 con la **7b**, e le campagne di `reconciliation`, di `boundary`
con l'arm `Note`, di `MemoryJournal::note` e di `prune`, con le loro tabelle, stanno in
[archivio](archivio/porta-di-qualita-storico.md), sezione
«T-1 e T-2 — le tre promesse provate solo dove ogni guardia passa (2026-08-17)».

⛔ **VOCE APERTA 1 — la regola di ADR-0018 che ENTRAMBE le implementazioni violano, e che il Task
11 dichiara invece di chiudere** (gotcha **#36**: una nota si legge e si dimentica). ADR-0018
pretende che *«un payload assente e uno mai registrato non siano indistinguibili»*. **Misurato il
2026-08-10, non argomentato:**
dopo la potatura, un passo potato e un passo **mai scritto** rispondono **entrambi `Err(Missing)`**
a `read_back`, sono **entrambi assenti** da `replay`, e una **seconda** `prune` risponde
`Err(Missing)` a tutti e due — indistinguibili in **tre** modi, su tutte e due. ⛔ **Non è chiusa,
e la ragione è la decisione D7:** la distinzione piena vuole l'**impronta** e la **dimensione** che
ADR-0018 chiede a un record potato, l'impronta vuole una funzione di hash, e nel kernel quella è una
**voce nuova nella lista di ADR-0031** — un atto deliberato che nessuna misura ha preparato. ⚠️
**La via che sembrava non costarla è stata MISURATA e cade:** lasciare la voce e svuotare il
payload **funziona** — `Ok([])` contro `Err(Missing)`, conformità verde — ma `steps_in_doubt`
risponde allora **`SuspendAndAsk`** su un passo riconciliato e potato, perché byte vuoti sono
**indecifrabili** e un record indecifrabile rimette il passo in dubbio: il sistema si fermerebbe su
**ogni** passo potato, **a ogni ripresa**. Una traccia che serva dev'essere leggibile dalla
riconciliazione, cioè una decisione di **formato**, e i byte congelati la rendono un atto
deliberato. **Chi la chiude:** il traguardo che porta la ritenzione, **insieme** alla decisione
sull'impronta. Il limite è scritto anche accanto al codice, in tutte e due le implementazioni e nel
blocco 7b.

⚠️ **VOCE APERTA 2 — la terza risposta di `prune` non è tenuta da nessuna promessa.** Le risposte
sono **tre**: `Missing` per un passo mai scritto, `StepInDoubt` per uno aperto, `Ok` per uno
riconciliato. Le promesse **7** e **7b** tengono le ultime due **attraverso entrambe** le
implementazioni; la prima è tenuta **solo** per il doppio in memoria, in
`crates/simulator/tests/memory_journal.rs`. **Misurato:** togliere la guardia `Missing` a
`FileJournal` (mutazione `M10` del Task 11) lascia l'**intero workspace verde**, quindi le due potrebbero
divergere in silenzio. ⛔ **Non è un buco aperto dal Task 11** — prima di lui entrambe rifiutavano
**ogni** potatura con `Missing` — e chiuderlo costa una promessa col proprio bugiardo, che nessuna
misura chiede oggi. **Chi la chiude:** il primo consumatore di `prune`, cioè la spazzata di
ritenzione.

⛔ **VOCE APERTA 3 — LE DUE NOZIONI DI «IN DUBBIO» DIVERGONO, E LA DIVERGENZA CADE DAL LATO CHE
AUTORIZZA LA DISTRUZIONE.** Nata il **2026-08-27** chiudendo il finding **AUD-006** del secondo
audit completo. ADR-0018 pone come regola **non
negoziabile** che *«un passo in dubbio non è mai potabile finché non è riconciliato»*, e nomina
**esplicitamente la §4**, cioè la riconciliazione del kernel. Ma le due parti fanno due domande
diverse:

| Chi | La domanda che fa | Con che cosa |
|---|---|---|
| la **porta**, dentro `prune` | *quale operazione è stata chiamata?* — un `intent` senza `outcome` | `EntryKind::Outcome` in `crates/simulator/src/journal.rs`, `kind == KIND_OUTCOME` in `crates/platform/src/journal.rs` |
| il **kernel**, in `steps_in_doubt` | *cosa dicono i record?* — decodificandoli | `crates/kernel/src/reconcile.rs`, il ramo `Err(_)` che **entra** nel dubbio con `SuspendAndAsk` |

⛔ **Un passo il cui record di ESITO è indecifrabile è quindi in dubbio per il kernel e POTABILE per
la porta.** ✅ **Misurato il 2026-08-27 da FUORI la crate**, su una crate usa-e-getta e su entrambe
le implementazioni — non dedotto e non ripreso dal rapporto (gotcha **#65**): `intent` con un
`RecordV1` leggibile, poi
`outcome(step, &[0xff, 0xfe, 0xfd])`, dà

```
steps_in_doubt -> [InDoubt { step: StepId(1), resolution: SuspendAndAsk }]
prune          -> Ok(())
```

⛔ **Perché conta più di una sfumatura.** È la **sola** operazione irreversibile del giornale, e
viene concessa esattamente sul passo su cui ADR-0007 vuole che il sistema **si fermi invece di
indovinare**. [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md) rende la lettura
fra versioni il caso **ordinario**, e `crates/kernel/src/record.rs` dichiara che un record che porta
una variante che la build non conosce si decodifica a `RecordError::Malformed` — *«an older build
STOPS rather than guesses»*. È **quella** frase che la misura smentisce: la build vecchia si ferma
in `steps_in_doubt` **e pota lo stesso passo** con `prune`.

⛔ **NON È CHIUDIBILE SULLA PORTA, ed è la ragione per cui è una voce aperta e non un difetto da
correggere.** La porta scambia **byte** e non può decodificare (ADR-0036), quindi non può porre la
seconda domanda: l'approssimazione è l'unica domanda che quel livello sa fare. La domanda generale
che ne esce è la seconda forma del gotcha **#64** in [`HANDOFF.md`](HANDOFF.md): *due strati che
usano la stessa parola con due definizioni — in quale VERSO divergono, e uno dei due versi
autorizza un'azione irreversibile?*

⛔ **L'OBBLIGO È DEL CHIAMANTE.** `prune` non ha **nessun** chiamante di produzione — ogni chiamata
nel workspace è un banco. La **spazzata di ritenzione** che ne farà la prima vive nel kernel e
**può** decodificare, quindi dovrà consultare `crate::reconcile::steps_in_doubt` e **saltare** ciò
che quello restituisce, invece di poggiare sulla guardia della porta. La frase sta accanto a
`Journal::prune` in `crates/kernel/src/ports/journal.rs`, dove la legge chi implementa.

⚠️ **NON PINZATA DA UNA SONDA, e il costo è scritto invece che taciuto.** Una sonda che asserisse
l'`Ok(())` di oggi andrebbe **rossa il giorno in cui la spazzata chiude la cosa per bene**, cioè per
aver avuto ragione — il gotcha **#73**: *una sonda che va cancellata per prendere una decisione è
un voto contro il prenderla*. La suite di conformità non può chiuderla per la stessa ragione della
porta, e lo **dichiara** nel blocco **7b** insieme alle altre due cose che non pinza. **Chi la
chiude:** il traguardo che porta la **ritenzione**, lo stesso della VOCE APERTA 1 e per un motivo
imparentato — entrambe aspettano che qualcuno **chiami** `prune`.

⚠️ **Limiti dichiarati della copertura di `boundary` e dell'arm `Note`.** Delle due sonde della nota
**una sola** vede entrambe le direzioni: `a_note_does_not_put_a_step_in_doubt` finisce con un
esito, quindi l'arm `Note` letto come un esito non la fa cadere. L'etichetta `trust: Untrusted`
poggia su **una** sonda sola. La rinumerazione di `RecordKind::Note` su un indice libero non la vede
nessuna andata-e-ritorno — il derive rinumera codifica e decodifica insieme, la stessa misura che il
doc di `record_shape.rs` porta per gli altri campi — e la tengono i byte congelati, sonda **F3**.

⚠️ **`MemoryJournal::note`: la nota archiviata come `EntryKind::Intent` è INOSSERVABILE da fuori**
— `note` rifiuta un passo senza intento, quindi un passo di sole note non è costruibile attraverso
la porta — e non ha sonda: quella che la rivendicava è stata **tolta**, perché una sonda il cui
commento rivendica un difetto che non vede insegna che il difetto è coperto (gotcha **#15**). La
guardia che dimentica **quale** passo (`!entries.is_empty()`) la uccide **solo**
`a_note_is_refused_when_the_intent_belongs_to_another_step`.

⛔ **Due lezioni della campagna di `prune`:** una contro-sonda **nasce non provata** (gotcha
**#45**), e una mutazione che deve far scattare qualcosa può fallire per aver colpito **troppo
poco** — `M14`, rifatta come `M14b` — e il verde che ne esce si legge come una prova: il gotcha
**#54** girato.

⚠️ **Una sonda non muore MAI da sola, dichiarato invece che taciuto:**
`a_step_is_in_doubt_at_most_once_however_many_records_it_carries` cade solo insieme a
`a_step_that_re_enters_doubt_keeps_the_place_it_first_took`, la cui asserzione confronta il
**vettore intero** e vede quindi anche un doppione. Resta perché porta lo **scenario** — un passo,
due record, il secondo illeggibile — non perché veda un difetto che nessun'altra vede.

✅ **LA QUESTIONE DELLE DUE VERITÀ È CHIUSA IL 2026-08-10 — DAL PROPRIETARIO — e la voce resta qui invece di essere
cancellata:** una voce aperta che sparisce non si distingue da una dimenticata. Il `kind` del
record e l'operazione della porta sono due verità indipendenti, e `replay()` ne restituisce una
sola. ⛔ **La decisione: `replay()` NON cambia e il `kind` RESTA nel record.** Le ragioni, in
ordine:

| | |
|---|---|
| **è semantica del kernel** | distinguere intento da esito è una decisione del kernel, e spostarla nella porta contraddice il doc di `replay` stesso — *«un'operazione come `steps_in_doubt()` sposterebbe una decisione del kernel dentro chi implementa la porta»* |
| **la forma durevole è del kernel** | ADR-0036, ed è la stessa regola per cui la porta scambia **byte** |
| **il costo evitato** | l'alternativa tocca **porta, conformità e due implementazioni**, e renderebbe **ridondante** il campo che `record.rs` chiama *«quello su cui poggia l'intero protocollo write-ahead»* |

⛔ **Il disaccordo si chiude da CHI SCRIVE, e ciascuno scrittore di record ha la PROPRIA sonda** —
decisione del proprietario:

| Scrittore | Dove | La sua sonda |
|---|---|---|
| `Untrusted::promote` | `crates/kernel/src/boundary.rs` | `the_promotion_writes_through_note_and_the_record_says_note` |
| `Arbiter::set_policy` | `crates/kernel/src/arbiter/mod.rs` | `a_policy_transition_writes_its_intent_before_its_outcome` |
| `run_the_ring` | `crates/kernel/src/sensor.rs` | le sonde di `crates/kernel/tests/sensor_ring.rs` |
| `gateway::dispatch` | `crates/kernel/src/gateway/mod.rs` | `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it` |

⛔ **LA VOCE RESTA APERTA:** se gli scrittori debbano condividere un aiutante è del
**proprietario** — cambia la forma di codice con più siti di chiamata — ed è **registrata e non
presa**. `RecordV1` ha un costruttore per specie (AUD-050), quindi «scrivere un `Outcome` con
`kind: Intent`» non è esprimibile, mentre «chiamare `outcome()` con un record costruito da
`intent()`» lo è ancora, ed è ciò che le sonde degli scrittori coprono.

⚠️ **`OperationSpy`**, in `crates/kernel/tests/boundary_promotion.rs`, è una spia conforme che
delega tutto a `MemoryJournal` e in più annota **quale metodo** è stato chiamato: senza, un
`promote` riscritto per chiamare `outcome()` — l'**opzione F**, che il proprietario ha esaminato e
**scartato** — lascerebbe l'intero workspace verde, perché anche `outcome` ammette un passo già
aperto; con la spia quella mutazione uccide **una sola** sonda, la sua.

⛔ **Ciò che NON è comprato, detto per intero:** nessuna regola di livello 1 impedisce a uno
scrittore futuro di chiamare `outcome()` con un record che dice `Intent`. Le sonde coprono gli
scrittori che esistono. È una voce **chiusa come decisione, non come garanzia**.

⚠️ **E una riga di questo registro va riletta quando quella questione si chiude, non prima:** la
riga di catalogo di `V5` è di **livello 1** — *«un effetto senza classe dichiarata non è
esprimibile»* — e che la **riconciliazione** eserciti `EffectClass` su tutti e tre i valori, e
tratti un record senza classe leggibile come `Unrepeatable` (la frase di ADR-0007), **non** la
copre: sono due proprietà diverse con lo stesso nome. Scritto perché chi riconta non muova il
numeratore per la ragione sbagliata, e perché spostarlo richiederebbe il **catalogo §7.4**, una
modifica alla spec che nessuno ha deciso (vincolo globale 7).

⚠️ **Il banco di misura produce esiti credibili e falsi** — strumenti gemelli corretti uno solo,
rinomine che rendono stantie le ancore di mutazione: gotcha **#48**, col testo integrale in
[`HANDOFF.md`](HANDOFF.md). 📌 I fine-riga del repository sono **misti per file** —
`process.rs` e `ports_are_implementable.rs` in CRLF, `mod.rs` e `network.rs` in LF — e la regola, un
file da non cambiare, vive in `CLAUDE.md`, riga *«I fine-riga sono misti per file»*.

### ⛔ VOCE APERTA CONSOLIDATA — le righe di catalogo delle sonde dell'audit (2026-08-18)

⛔ **L'esecuzione dell'audit ha prodotto DIECI sonde permanenti nuove, e NESSUNA ha una riga nel catalogo §7.4.**
⚠️ **[C-S4-1]** La §7.4 è **spec**, e il **vincolo globale 7** la mette fuori dalla portata di una passata di rimedio:
l'aggiunta è del proprietario. Come per **PL-1**, *registrata come voce aperta e non come nota*, perché una nota si legge
e si dimentica (gotcha **#36**).

⚠️ **Raccolte qui in un posto solo**, perché più voci aperte sullo stesso oggetto sono il modo in cui una di esse smette
di esserlo senza che nessuno l'abbia chiusa.

| Finding | Sonde | Cosa difenderebbe la riga |
|---|---|---|
| **PL-1** | `the_journal_file_is_not_world_readable` | **ADR-0023** — «protetto quanto il tuo account di sistema», su Unix |
| **K-1** | `a_request_written_before_the_run_belongs_to_nobody` · `a_request_written_by_a_destructor_belongs_to_nobody` | **§2.4.1** — la cella è l'unico canale, e le sue scritture appartengono a un poll |
| **B-1** | `the_delivered_turn_limit_is_honoured_by_its_value` | **§2.8 · ADR-0034** — nessuna decisione legge un parametro che non le è stato consegnato |
| **P-1** | `promote_reason_is_not_runtime_text` (`compile_fail`) | **§6.5 · ADR-0014 · I6** — è la seconda metà della riga di §7.4.1 blocco B che già esiste |
| **B-2** | i cinque bugiardi del `reactor` | **§7.4.6** — la conformità della porta più importante del progetto |
| **B-3** | `a_restore_serves_the_checkpoint_it_was_asked_for_and_not_the_first_one` | **§2.3** — un'implementazione ritiene e **confronta** l'identificatore |
| **S-1/S-2** | le tre di `CrashingJournal` | **§7.5** — il conteggio contro cui è estratto il punto di caduta |
| **S-5** | l'uguaglianza sui pioli | **§7.5.3** — la profondità della campagna di livello 2 |

⛔ **E una NONA voce che non è una riga di catalogo ma una modifica alla conformità:** l'asserzione **4b** del `reactor`
è **implicata** dalla 4a e non può scattare da sola. Toglierla, o ribasare `second_deadline` su `start`, è una decisione
sulla porta condivisa, del proprietario. La dimostrazione sta nella sezione B-2 qui sotto.

---

### K-1 e B-1 — la cella che accettava scritture da fuori un poll, e il limite mai letto (2026-08-18)

✅ **Il rimedio di K-1: la cella di `Sleep` si svuota subito prima di ogni poll.** L'invariante passa da *«nessuno scriva
mai fuori da un poll»*, che nulla può imporre, a *«conta solo ciò che è scritto durante questo poll»*, imposto lì: tutte
le vie — prima di `run`, un `Drop` dopo il ciclo, e quelle non ancora immaginate — chiudono in un punto solo.
⛔ **Due opzioni sono cadute e non si riaprono:** il waker su misura non è costruibile (`Waker::from_raw` è `unsafe`,
`forbid(unsafe_code)` lo rifiuta, misurato in **M-5** — gotcha **#32**), e la cella posseduta dall'`Executor` non chiude
il `Drop`. Il perché intero sta nel sorgente di `crates/kernel/src/executor.rs` e in [`riferimenti.md`](riferimenti.md),
sezione «Esecuzione dell'audit — la decisione 3 (K-1, con B-1), 2026-08-18: le misure, coi comandi».

⛔ **Le sonde sono DUE perché le VIE sono due — non le cause** (gotcha **#65**), in
`crates/kernel/tests/executor_determinism.rs`:

| Sonda | Via |
|---|---|
| `a_request_written_before_the_run_belongs_to_nobody` | il banco scrive prima di `run` |
| `a_request_written_by_a_destructor_belongs_to_nobody` | un `Drop` scrive dopo il ciclo |

La seconda vuole un `Future` scritto a mano, e asserisce che il distruttore sia girato prima di guardare l'oracolo
(gotcha **#17**): il perché sta accanto a `WritesFromItsDestructor`, nel banco.
📌 `a_wait_already_over_wakes_immediately_and_the_clock_does_not_move` fa dichiarare la scadenza all'**attività** e non
al banco: con la scrittura dal banco, dopo K-1, restava verde e vacua (gotcha **#66**).

⛔ **B-1 — il limite di turni consegnato e mai letto.** `parameters_delivered.rs` prova che `Parameters` **trasporta** il
numero; `the_delivered_turn_limit_is_honoured_by_its_value` prova che l'esecutore lo **usa**. L'oracolo è il **conteggio
dei poll**, l'unico osservabile che porta il **valore**, su due valori, `7` e `13` (gotcha **#48**).

Provate nelle due direzioni; le misure stanno in [archivio](archivio/porta-di-qualita-storico.md), sezione «K-1 e B-1 —
la cella che accettava scritture da fuori un poll, e il limite mai letto (2026-08-18)».

⚠️ **VOCE APERTA — le tre sonde non hanno una riga di catalogo**: sta nella tabella consolidata qui sopra.

### P-1 — A3 era dichiarata chiusa e aveva una seconda bocca (2026-08-18)

✅ **La regola: `promote` prende `reason: &'static str`.** Il contenuto esterno è **dato di runtime**, e la strada
accidentale **smette di compilare**, a livello 1. Il perché, e le due opzioni cadute — `reason: &Instruction`, che vale
quanto il suo costruttore `pub` (via **A1/A2**), e `reason` come **enum**, che muoverebbe i byte congelati verso una
`Record::V2` (ADR-0036) per un solo chiamante di produzione — stanno nel doc di `promote` in
`crates/kernel/src/boundary.rs` e in [`riferimenti.md`](riferimenti.md), sezione «Esecuzione dell'audit — la decisione 2
(P-1), 2026-08-18: le misure, coi comandi».

✅ **La sonda è un caso `compile_fail`, nella forma forte:** `tests/compile_fail/promote_reason_is_not_runtime_text.rs`,
`error[E0597]`. Rimessa la firma a `&str`, il caso **compila** e `trybuild` risponde **`error`** invece di `mismatch`
(gotcha **#42**): un caso che riporta compilando non si disarma con un `TRYBUILD=overwrite` in blocco. Il controllo è
`promote_without_journal.rs`, che nella stessa corsa risponde `mismatch`: quel caso passa dall'oracolo, questo no.
⚠️ **L'altra direzione non è qui, deliberatamente:** che un letterale promuova e si stampi lo tiene
`boundary_promotion.rs`. Una copia sarebbe gotcha **#49**.

⛔ **Ciò che RESTA APERTO:** `String::leak` produce ancora un `&'static str`, e un letterale può **mentire** — provenienza
e non correttezza, il limite che **A4** già dichiara. Si è chiusa la strada che si prende **senza accorgersene**.
⚠️ **[C-S4-3]**

⚠️ **VOCE APERTA — la riga di catalogo, come per K-1/B-1 e per PL-1:** sta nella tabella consolidata qui sopra. È la
seconda metà della riga V19 di §7.4.1 blocco B (*«promuovere testo a istruzione ← la porta journal»*): la porta
impedisce la conversione muta, il `'static` impedisce che il contenuto esca dalla giustificazione.

### Decisione 7 — le cinque sonde mancanti, e l'audit si chiude (2026-08-18)

#### B-2 — quattro gruppi su cinque della conformità `reactor` non erano mai visti scattare

✅ **Cinque bugiardi, uno per ASSERZIONE e non per gruppo:** la suite muore alla prima che scatta, quindi un bugiardo
rotto in due punti prova solo la prima (gotcha **#65**). Stanno in `crates/kernel/tests/reactor_contract.rs`.

| Bugiardo | Asserzione | Il difetto |
|---|---|---|
| `BackwardsClockLiar` | **1** | `now()` cammina all'indietro da solo |
| `ShortWaitLiar` | **3a** | il clock arriva alla scadenza, la **risposta** è corta di un ms |
| `LaggingClockLiar` | **3b** | risponde la scadenza esatta e il **proprio clock** si ferma a metà |
| `SecondWaitShortLiar` | **4a** | corretto sulla **prima** attesa, corto sulla **seconda** — conta le chiamate, o non arriverebbe al gruppo 4 |
| `PanickingWallClockLiar` | **5** | `wall_time()` non risponde |

⛔ **Il gruppo 5 non ha asserzioni:** un reactor il cui `wall_time` esplode trasforma *«il blocco esiste»* in *«il blocco
GIRA»*, l'unica proprietà che quel blocco può avere.

⛔ **L'asserzione 4b è IMPLICATA dalla 4a.** `second_deadline` è calcolata da `first_reached`, quindi
`second_reached >= second_deadline = first_reached + MARGIN > first_reached`: un bugiardo per la 4b **non è
scrivibile**. ⚠️ Non è vacua, è muta: non può essere falsa dove l'altra è vera. **Voce aperta** del proprietario, nella
tabella consolidata qui sopra; la stessa dimostrazione sta accanto all'asserzione, nel banco.

Provato nelle due direzioni; la misura sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «B-2 — quattro
gruppi su cinque della conformità `reactor` non erano mai visti scattare».

#### B-3 — il finto filesystem poteva smettere di confrontare i `CheckpointId`

✅ `a_restore_serves_the_checkpoint_it_was_asked_for_and_not_the_first_one`, in
`crates/kernel/tests/ports_are_implementable.rs`. La rende non vacua un **passante** — un secondo checkpoint che non è
quello sotto esame — e ha **due direzioni**: il checkpoint **più recente** (che «prendi il primo» sbaglia) e il **più
vecchio** (che «prendi l'ultimo» sbaglierebbe). Ora è osservato il confronto su cui poggia l'assenza di getter di
`CheckpointId` e `ClientId`: *«un'implementazione lo ritiene e lo CONFRONTA, esattamente come fa
`InMemoryFilesystem`»*. La misura sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «B-3 — il finto filesystem poteva smettere di confrontare i
`CheckpointId`».

#### S-1 e S-2 — `note` non era mai esercitata con successo

✅ Tre sonde, una per via (gotcha **#65**), in `crates/simulator/tests/crashing_journal.rs`:
`the_success_path_of_note_is_exercised_and_counted`, `a_refused_note_does_not_consume_a_crash_position`,
`a_refused_intent_does_not_consume_a_crash_position`. La prima controlla che la nota **raggiunga l'archivio** leggendo
`replay()` — tre record, l'ultimo è la nota — e non solo il contatore. La tabella delle mutazioni sta in
[archivio](archivio/porta-di-qualita-storico.md), sezione «S-1 e S-2 — `note` non era mai esercitata con successo».

#### S-5 — i gradini della scala erano giustificati in un commento e contati da nessuno

✅ **La tesi di `DEEP_RECORDS` è un controllo:** i pioli sono l'insieme delle **lunghezze di prefisso distinte**
recuperate nella spazzata, e l'asserzione è un'**uguaglianza** — `records + 1`, l'archivio vuoto più un piolo per record.
`partial > 0` non bastava: conta i **punti** che atterrano a metà, non gli **archivi distinti** su cui atterrano. Difende
la ragione per cui la campagna è **profonda** invece che **larga**. Provato nelle due direzioni; la misura sta in
[archivio](archivio/porta-di-qualita-storico.md), sezione «S-5 — i gradini della scala erano giustificati in un commento e contati da nessuno».
📌 `reactor_contract.rs` è `include!`d anche da `platform`, quindi ogni suo `#[test]` gira due volte: lo dichiarano
`crates/platform/tests/reactor_contract_real.rs` e `crates/platform/src/reactor.rs`.

#### `CrashingJournal` — cinque mutazioni, cinque uccise, e una coppia che prova di essere due difetti

⛔ **Due sonde a senso unico, ciascuna controparte dell'altra**, in `crates/simulator/tests/crashing_journal.rs`:
`a_journal_told_not_to_crash_never_falls` è verde sotto «non cade mai» e rossa sotto «cade sempre» — l'unica in quella
direzione, ed è quella su cui `C7a` poggia; `after_the_fall_every_later_write_is_refused_too` asserisce solo rifiuti,
quindi un giornale che rifiuta sempre la soddisfa. Senza la coppia, una delle due sembrerebbe copertura senza esserlo.
La tabella delle mutazioni sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «`CrashingJournal` — cinque mutazioni, cinque uccise, e una coppia
che prova di essere due difetti».

#### La campagna di livello 1 — quattro mutazioni, e una riga nuova che si è messa davanti alle altre

📌 **La regola:** quando due mutazioni uccidono la **stessa** asserzione, prima di concludere che la sonda non le
distingue (gotcha **#55**) si cerca **una terza mutazione che lasci passare la prima asserzione**. Se esiste, le due non
erano in competizione. Il caso: in `c7a` un oracolo giudica **lo scenario** (*ha scritto?*) e quello dell'insieme
**la riconciliazione** (*è d'accordo?*) — due mansioni disgiunte, scritte accanto agli oracoli in
`crates/simulator/tests/dst_campaign.rs`. La tabella delle mutazioni sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «La campagna di livello 1 —
quattro mutazioni, e una riga nuova che si è messa davanti alle altre».

#### I byte congelati — tredici mutazioni, e due dettate dal piano non compilavano

⛔ **I byte congelati sono l'unico artefatto del progetto che non si corregge:** se i byte cambiano non è un
aggiornamento, è un cambio di formato (ADR-0036; lo ripete la testa di `crates/kernel/tests/frozen/record_v1.map`).
📌 **E la mappa non è prosa:** `record_v1.map` è **riletta** da `frozen_bytes.rs`, e le colonne `offset` e `hex` devono
ricostruire il `.cbor` byte per byte (gotcha **#43**); la colonna di prosa è dichiarata **non** verificata dentro la
mappa stessa. La tabella delle mutazioni sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «I byte congelati — tredici mutazioni, e due dettate
dal piano non compilavano».

#### Tre moduli di test vivono in `src/`

📌 **Tre moduli di test vivono in `src/` invece che in `tests/`, e la deviazione è dichiarata in ciascun file.** ⚠️
**[C-S4-4]** Non è una scorciatoia: in due casi su tre non è nemmeno una scelta.

| Dove | Che cosa difende, e perché sta in `src/` |
|---|---|
| `crates/daemon/src/main.rs` (quante sonde lo dice la sezione del **Task 10** di questo file) | che il **grafo di produzione si monti e giri** — il cablaggio, non il dimensionamento del limite di turni: col limite a `0` la suite resta **verde**, perché senza attività il corpo non gira mai — che il **giornale** sia aperto e che uno che non si apre **fermi l'avvio**, che le due quote di ADR-0033 siano **tenute e non sottratte**, che la policy montata sia il default di ADR-0006, che una concessione permanente **non scada**, e che una quota permanente che non entra fermi l'avvio **nominandosi**, per `Queued` e per `Refused` (`E41`). Sta in `src/` perché le funzioni sono **private in un target `bin`**, e nessun test d'integrazione può linkare un binario |
| `crates/platform/src/rng.rs` | `SequentialRng`: estrazioni 0, 1, 2 e così via, `new` e `default` **lo stesso** generatore, `below` a turno **a limite costante**, il limite dichiarato quando **cambia**, il contatore che **avvolge invece di traboccare**. Sta in `src/` perché l'ultimo costruisce `SequentialRng(u64::MAX)` **col campo privato** |
| `crates/kernel/src/arbiter/mod.rs` (quanti test lo dice la sezione del Task 7 di questo file) | la **revoca** del Traguardo 5 Task 7: chiedere indietro **marca e non prende**, la grazia si riscuote quando scade e **non prima**, l'**istante** in cui scade, non si toccano una concessione non prelazionabile, una corsia non inferiore e **un pari nella corsia che chiede** — `below` è **esclusivo** (`E71`) — si ferma appena la stanza basta e prende la corsia **peggiore** per prima, **non marca nulla** se il recuperabile non copre il bisogno (`E69`), chiedere **due volte** non compra la stanza due volte, e lo scaduto si riscuote **prima** di marcare. Sta in `src/` perché `ask_back` è `pub(crate)` — da una crate a parte è `` error[E0624]: method `ask_back` is private `` — e ⛔ **solo la privatezza sposta una sonda lì:** l'undicesima del compito non chiama `ask_back` e resta in `crates/kernel/tests/arbiter_admission.rs` |

### P-2 — la ragione dello scaglionamento era falsa, e lo era dal Traguardo 2 (2026-08-21)

✅ Le **quattro righe di §6.10.5** della porta `process` sono **chiuse** dal Task 11 del Traguardo 5: i casi
`compile_fail` di §6.10.5 — fra cui `instructing_after_the_kill.rs`, `reading_without_a_receipt.rs` e
`reading_twice_from_one_receipt.rs` — con le contro-sonde di `crates/kernel/tests/worker_tokens.rs`. La ragione vera per
cui erano scoperte era la direzione *«deve scattare»* mancante, non l'assenza di un `Grant`: un `Worker` si ottiene
implementando il tratto, come fa `ScriptedWorker` in `ports_are_implementable.rs`. Lo stato vive nella riga di
`worker_tokens.rs` del registro e nella riga «le righe 1–4 di §6.10.5» di «Cosa la porta NON controlla, in questo
traguardo».
📌 Un rinomino si censisce sul **frammento** più corto che resti unico, non sul nome intero: è il gotcha **#70** di
[`HANDOFF.md`](HANDOFF.md).

Il verbale — le misure della sonda usa-e-getta, le case censite della ragione falsa, la divergenza dal ledger — sta in
[archivio](archivio/porta-di-qualita-storico.md), sezione «P-2 — la ragione dello scaglionamento era falsa, e lo era dal
Traguardo 2 (2026-08-21)».

### La revisione del compito 5 del Traguardo 6 — due mutanti vivi e un conteggio stantio (2026-08-31)

⚠️ **Il limite della guardia di crescita su `Detail`** (`E54`) — estendere l'arm senza congelare un record compila
ancora, deliberatamente come nei tre fratelli (§7.4.4) — è scritto accanto al `match` esaustivo, in `frozen_bytes.rs`.
✅ **La classe del record di feedback di `run_the_ring` la consegna il chiamante**, `correction_effect: EffectClass`
(`E55`), per coerenza con `next` (ADR-0034); la sonda la pinza su due valori, nessuno `Idempotent` (gotcha **#48**). Il
perché sta accanto a `run_the_ring` in `crates/kernel/src/sensor.rs`.
📌 I due record che `run_the_ring` scrive li tengono per intero `a_passing_sensor_writes_a_verdict_and_opens_nothing` e
`a_failing_verdict_opens_a_new_step_and_carries_the_detail`, in `crates/kernel/tests/sensor_ring.rs`; l'arm `Verdict` di
`steps_in_doubt` lo tengono `a_verdict_does_not_put_a_step_in_doubt` e
`a_verdict_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them`, in `crates/kernel/tests/reconciliation.rs`.

⚠️ **Le misure delle revisioni vivono in questo file e non in [`riferimenti.md`](riferimenti.md):** è la voce aperta
`E146` del proprietario, riga **30** della tabella delle voci aperte del Traguardo 5.

📌 **La lezione di metodo:** i difetti di una revisione vivono in ciò che il compito **ha reso stantio senza toccarlo** —
un enum nuovo che nessuna guardia copre, un valore che nessuna delle mutazioni elencate raggiunge, frasi vere finché i
record congelati erano tre. ⛔ Non si vedono rileggendo il diff: si vedono **mutando** e **censendo**, ed è per questo
che una rilettura dell'autore non vale una revisione (`E53`).

Le mutazioni di questa passata e delle due seguenti stanno in [archivio](archivio/porta-di-qualita-storico.md), sezioni
«La revisione del compito 5 del Traguardo 6 — due mutanti vivi e un conteggio stantio (2026-08-31)», «La SECONDA passata
di revisione del compito 5 — quattro mutanti vivi, e una radice sola» e «La TERZA passata — la prima da un sotto-agente,
e il gotcha #98 riproduce lo stesso giorno».

### La QUARTA passata — un mutante vivo, e QUATTRO censimenti che i rimedi precedenti non avevano chiuso

⚠️ **Un limite del cancello:** una riga di **commento** troppo lunga non la vede nessun controllo — `rustfmt` non tocca i
commenti, e §7.4.3 non dà voce a `clippy` nel cancello (`E77`). Il resto della passata sta in
[archivio](archivio/porta-di-qualita-storico.md), sotto lo stesso titolo.

### La QUINTA passata — la domanda cambia, e tutti i rilievi vengono dall'ondata precedente

📌 **La domanda di revisione:** prima di cercare difetti nuovi, per ogni rimedio dell'ondata precedente si risponde
**col `grep`** a *«ha chiuso la CLASSE, o il punto dove l'ho trovato?»*. I rilievi `E79`–`E82` stanno in
[archivio](archivio/porta-di-qualita-storico.md), sotto lo stesso titolo.

### `AUD-050` chiuso a LIVELLO 1 — la cura alla radice, scelta dal proprietario (2026-09-01)

✅ **La coppia `kind`/`Detail` è tenuta dal livello 1, non dalla disciplina.** La forma di `RecordV1` — sei campi
**privati**, sei accessori, un costruttore per specie con `reason: &'static str`, un solo `of` **privato** che
costruisce, sul precedente di `Arbiter::issue` per `Grant` (§5.6) — sta in `crates/kernel/src/record.rs`. La coppia
sbagliata non è rifiutata, è **impronunciabile**: preferita a una sonda che congelerebbe una partizione destinata a
cambiare (gotcha **#57**), perché una specie nuova porta il **proprio** costruttore, che è additivo.

📌 **Il perimetro si misura col comando, al padre del cambiamento:** `git grep -c "RecordV1 {" c63c8c8^ -- 'crates/**/*.rs'`
per i file e i hit, `git grep -l "RecordV1 {" c63c8c8^ | cut -d/ -f2 | sort | uniq -c` per le crate. ⚠️ Il comando conta
**hit**, non siti di **costruzione**: fra i hit ci sono la definizione, l'`impl Debug`, due `assert_eq!` di
`record_shape.rs` e righe di commento. È la casa unica della misura a cui rimanda `crates/kernel/src/boundary.rs`; le
cifre e le clausole corrette stanno in [archivio](archivio/porta-di-qualita-storico.md), sotto lo stesso titolo.

⛔ **Il 35° caso `compile_fail`, `record_reason_is_not_runtime_text.rs` (`error[E0597]`), è provato nelle due
direzioni:** allargati i cinque `reason: &'static str` a `&str`, il caso va **`error`** (gotcha **#42**). E **non è una
copia** di `promote_reason_is_not_runtime_text.rs`: sotto la stessa mutazione quello resta **`ok`**, perché i due casi
tengono strade **diverse**. Gira dentro `level_1_rules_do_not_compile`, che è **un** test.
📌 `record_without_trust_label.rs` prova l'**arità** del costruttore, `error[E0061]`, la forma che
`reading_without_a_receipt.rs` usa già: col letterale rifiutato scattava per una seconda ragione, e lo spiega il caso
stesso.

⚠️ **VOCE APERTA REGISTRATA E NON PRESA, del proprietario:** il 35° caso non ha una riga di catalogo propria. La §7.4 è
**spec**, vincolo globale 7 — stesso trattamento di `PL-1` e di `K-1`/`B-1` (gotcha **#36**). Difende la stessa
proprietà della riga che copre `promote_reason_is_not_runtime_text.rs`, per un'altra strada.

⚠️ **Che cosa NON compra:** `payload` resta un `Vec<u8>` che il chiamante riempie e `trust` un parametro — l'etichetta
è l'affermazione del **chiamante**, il limite che la via **A4** di `boundary.rs` dichiara.

### `E94` — la TERZA bocca della classe di AUD-050: `RoutingDetail`, chiusa il 2026-09-01

⛔ **Ogni specie di `Detail` che porta testo proprio deve la stessa firma:** il `&'static str` sulle specie di
`RecordV1` chiude la strada di `reason`, mai quella di un `Detail`. La forma di `RoutingDetail` — tre campi privati,
gli accessori `model()`, `evaluated()`, `degraded()`, il costruttore
`RoutingDetail::new(model: &'static str, evaluated: u32, degraded: bool)`, e `String` sul filo perché un `&'static str`
non si produce dai byte in arrivo senza `leak` (**P-9**) — sta nel doc del tipo, in `crates/kernel/src/record.rs`.

⛔ **Il caso `routing_detail_model_is_not_runtime_text.rs` (`error[E0597]`) è provato nelle due direzioni:** allargato
il solo `model` di `new` a `&str`, va **`error`** (gotcha **#42**). E **non è una copia dei due fratelli**: sotto la
stessa mutazione `record_reason_is_not_runtime_text.rs` e `promote_reason_is_not_runtime_text.rs` restano **`ok`**.

⚠️ **`VerdictDetail` NON è sigillata, deliberatamente:** porta un `bool` e un `u64`, nessun testo di runtime. L'asimmetria
è scritta accanto ai due tipi in `crates/kernel/src/record.rs`, perché nessuno la «uniformi».

⚠️ **VOCE APERTA REGISTRATA E NON PRESA, del proprietario:** il caso di `RoutingDetail` non ha una riga di catalogo
propria. La §7.4 è **spec**, vincolo globale 7 — stesso trattamento del caso di AUD-050 qui sopra (gotcha **#36**).

La riproduzione, il sito che la scheda non prezzava e i byte fermi stanno in
[archivio](archivio/porta-di-qualita-storico.md), sotto lo stesso titolo.

### Il compito 7 del Traguardo 6 — la tripla del permesso, e la proiezione del giornale (2026-09-01)

⛔ **Il permesso è una TRIPLA** — `(strumento × risorsa × operazione)` — e *«quali permessi sono attivi ora»* si risponde
**rileggendo il giornale**, senza secondo archivio. Il banco è `crates/kernel/tests/permission_triple.rs`; il meccanismo
è `crates/kernel/src/permission.rs`.

| Artefatto | Che cosa lo esercita |
|---|---|
| `permission::grant` — la sua **via d'errore** | `a_journal_that_refuses_the_note_makes_the_grant_fail` |
| `permission::grant` — ciò che **scrive**: `kind`, `effect`, `trust`, `payload`, `reason`, `detail` | `grant_writes_every_field_it_says_it_writes`, che rilegge il record **dal giornale** |
| `permission::is_granted` | le sonde della tripla, una per componente — `a_different_TOOL_is_not_covered`, `a_different_RESOURCE_is_not_covered`, `a_different_OPERATION_is_not_covered` — più `a_granted_triple_is_granted`, il giornale vuoto e le vie d'errore del §4 elencate sotto. Si contano **sul binario**: `cargo test --locked -p kernel --test permission_triple` (`E128`) |
| `RecordKind::Permission` (indice **5**) | `crates/kernel/tests/frozen_bytes.rs` — il **sesto** record congelato |
| `Detail::Permission` (indice **2**) | idem: è l'unico record congelato che lo porta |
| `PermissionDetail` — gli indici **0** e **1** dei due nomi | `the_two_names_of_a_permission_do_not_share_one_offset_and_its_mirror`, nello stesso file. Il record congelato porta `"frozen"` due volte e **non può** vederli scambiati |
| l'arm vuoto in `reconcile` | `a_permission_does_not_put_a_step_in_doubt` e il suo gemello, in `reconciliation.rs` |
| `PermissionDetail` sigillata | **due** casi `compile_fail`, uno per parametro |

#### Le mutazioni, col proprio esito MISURATO

📌 **`nothing_is_granted_on_an_empty_journal` è la sola sonda che parte da un giornale vuoto:** *nessun record ⇒ niente
concesso* non lo tiene nient'altro, anche se nessun mutante della campagna sulla domanda è ucciso da lei sola.
⚠️ Revocare una mutazione con `cp -p` conserva l'mtime, e `cargo` misura l'oggetto mutato: si revoca con `cp` semplice,
o si fa `touch` dopo — gotcha **#103** di [`HANDOFF.md`](HANDOFF.md).
Le due campagne — i dieci mutanti sulla domanda, i sei sulla registrazione — e le due direzioni stanno in
[archivio](archivio/porta-di-qualita-storico.md), sezione «Le mutazioni, col proprio esito MISURATO» del compito 7.

#### Le due direzioni dei casi `compile_fail`, e perché sono DUE

⛔ **`PermissionDetail::new` ha DUE parametri di testo, quindi due strade, e due casi:** `..._tool_...` e
`..._resource_...`. ⚠️ **[C-S4-5]**
Un caso solo che li nominasse entrambi resterebbe `error` allargandone **uno qualsiasi**: è la forma **debole** (`E83`).
Ciascuno tiene la propria strada, e nessuno dei due fa cadere i tre fratelli (`routing_detail`, `record_reason`,
`promote_reason`). Provato allargando un parametro per volta; la tabella sta in
[archivio](archivio/porta-di-qualita-storico.md), sotto lo stesso titolo.

#### Le quattro vie del §4 — tutte raggiungibili, tutte con una sonda

| | La via | La sonda |
|---|---|---|
| ① | il giornale rifiuta | **due**: `a_journal_that_refuses_the_note_makes_the_grant_fail` (`note` rifiutata in `grant`) e `a_journal_that_will_not_replay_is_not_an_answer_of_false` (`replay` rifiutata in `is_granted`) |
| ② | il record **illeggibile** | `a_record_that_will_not_decode_is_not_an_answer_of_false` — byte scritti attraverso la porta, che non convalida nulla (strada A4 di `boundary`) |
| ③ | un record di **specie diversa** | `a_record_of_another_species_is_stepped_over_and_not_read_as_a_permission`, con un giornale misto che porta anche un record `Routing`, una specie che un `detail` ce l'ha |
| ④ | un `Permission` **senza** `detail` | `a_permission_record_without_its_detail_is_not_an_answer_of_false`. ⚠️ **Raggiungibile:** impronunciabile **in sorgente** (`RecordV1::permission` prende il `detail` per valore), costruibile **dai byte** con un solo byte cambiato |

#### Le decisioni prese, e ciò che costano

| | |
|---|---|
| `is_granted` restituisce un **errore proprio** che compone `JournalError` e `RecordError` | `JournalError` **non** ha guadagnato varianti. ⛔ È il **primo errore di `kernel` con un carico**; la frase contraria del doc di `framing.rs` porta il proprio richiamo |
| `write: bool` sul filo e non un `enum` | un `enum` sarebbe un quarto `index_only` sul filo, un indice che non si ritira mai, pinzato una variante per record congelato. Il `bool` si pinza da sé |
| `Permission` resta a campi `pub`, `PermissionDetail` no | il primo è un tipo di **decisione** (`&'static str` **è** la guardia), il secondo è **sul filo** e porta `String` |
| ⚠️ **niente revoca**, e niente **sessione** | dichiarati sul modulo `crates/kernel/src/permission.rs`, con l'innesco: la prima revoca sarà una specie propria. `V21` resta **`⚠️ parziale`**: la metà *«e per la sessione corrente»* non ha soggetto e **non si marca** |

⚠️ **VOCE APERTA REGISTRATA E NON PRESA, del proprietario:** i due casi `compile_fail` di `PermissionDetail`
non hanno una riga di catalogo propria. La §7.4 è **spec**, vincolo globale 7 — stesso trattamento del caso di `E94`
qui sopra.

⚠️ **Un limite del cancello:** `cargo fmt` **non** è un passo del cancello, e nessun rosso direbbe un file non
formattato.

### La passata INDIPENDENTE sul perimetro del compito 5 — un'affermazione di sicurezza falsa in `src/` (2026-09-01)

⚠️ **Registrata e non presa, del proprietario (`E108`):** un indice di variante **sconosciuto** di `Detail`
decodifica a `detail: None` **in silenzio**, non a `Err`; ciò che ferma una build vecchia è il `kind` **appaiato**. La
coppia tenuta al livello 1 da `AUD-050` non chiude questa strada, perché lì si decodificano byte **già scritti**: una
seconda specie di `Detail` sotto un `kind` esistente verrebbe ingoiata da una build vecchia. Deliberatamente **non
pinzata** — una sonda sul silenzio di oggi sarebbe un voto contro il cambiarlo (gotcha **#73**). La indicizza la riga 20
della tabella delle voci aperte del Traguardo 6.

⚠️ **UNA VOCE REGISTRATA E NON PRESA, del proprietario:** il doc di **modulo** di `record.rs` dice *«with and without
`#[cbor(array)]` on the two types below»*, e i tipi che portano quell'attributo sono di più — quanti, lo dice
`grep -c '^#\[cbor(array)\]' crates/kernel/src/record.rs`. ⛔ Non toccata: la frase descrive una **misura passata**, e
correggerla d'iniziativa prezzerebbe come difetto ciò che potrebbe essere un verbale. La indicizza la riga 23 della
tabella del Traguardo 6.

I rilievi `E108`–`E113` e le loro misure stanno in [archivio](archivio/porta-di-qualita-storico.md), sotto lo stesso
titolo.

### La NONA passata — la domanda di classe coglie il CENSIMENTO che aveva chiuso una classe (2026-09-01)

#### La crescita, enum per enum

📌 La crescita di un enum si misura con `cargo check --locked --workspace --all-targets`, una variante aggiunta da sola:
un enum tenuto risponde `E0004`.
⚠️ **Registrata, non presa:** `reconcile::Resolution` cresce a `exit 0` e **nessuno la decide oggi**, né in `src/` né
nei banchi — il primo consumatore la decide con un `match`. La indicizza la riga 24 della tabella del Traguardo 6.
La tabella degli altri enum, prima e dopo `fe52039`, sta in [archivio](archivio/porta-di-qualita-storico.md), sotto lo
stesso titolo.

#### Il rilievo del coordinatore, trovato rimediando

⚠️ **Un limite del cancello, registrato e non preso:** la riga *«zero avvisi»* usa
`cargo build --locked --workspace`, che **non compila i banchi**, quindi un avviso di banco non la fa rossa — misurato
sul nome con maiuscole di `crates/kernel/tests/gateway_decisor.rs`, con
`cargo check --locked -p kernel --test gateway_decisor`. Se debba passare a `--all-targets` tocca il cancello, ed è del
proprietario (vincolo globale 7). La indicizza la riga 25 della tabella del Traguardo 6.

La domanda di classe rimedio per rimedio e le mutazioni del revisore stanno in
[archivio](archivio/porta-di-qualita-storico.md), sezioni «La domanda di classe, rimedio per rimedio» e «Le mutazioni di
prodotto del revisore — tutte rosse».

### La DECIMA passata — la prima ondata di CODICE riletta senza un difetto di prodotto (2026-09-01)

#### I quattro predicati di `fe52039`, braccio per braccio

⚠️ **Tre rovesci di predicato restano verdi e sono equivalenti per costruzione** — `Runnable => Some(..)` in
`TaskState::sleeping_until`, `Outcome => true` e `Note => true` in `EntryKind::is_intent` — e il perché è dichiarato nel
sorgente (`E135`), accanto ai predicati di `crates/kernel/src/executor.rs` e di `crates/simulator/src/journal.rs`.
📌 **Un rovescio verde qui non è un mutante vivo da chiudere ma una proprietà da dichiarare**, la forma di `E70`: ciò
che i predicati comprano è l'`E0004` sulla crescita, e la crescita è tenuta. La tabella dei rovesci — in
`executor_determinism`, `arbiter_campaign`, `dst_campaign` e negli altri banchi — e il censimento degli enum stanno in
[archivio](archivio/porta-di-qualita-storico.md), sotto lo stesso titolo e in «Il censimento della crescita — tutti gli
enum di `crates/*/src`».

### Il compito 8 del Traguardo 6 — lo stato di degrado si RICALCOLA, e non si cachea (2026-09-02)

⛔ **Lo stato di degrado è un DERIVATO e non un archivio.** `crates/kernel/src/degradation.rs` porta `Degradation`,
`DegradationError` e `degradation_now`, che rilegge il giornale e interroga l'arbitro **a ogni domanda**; il banco è
`crates/kernel/tests/degradation_state.rs`. È la forma di `permission::is_granted` e di `reconcile::steps_in_doubt`:
nessuna cache accanto alle due fonti, quindi *«mai autorevole di sé stesso»* è vero **per costruzione** e non per
disciplina. Il formato durevole **non è toccato**.

| Artefatto | Che cosa lo esercita |
|---|---|
| `Degradation::routing_degraded` | `a_degraded_routing_shows_up_in_the_state`, `it_is_RECOMPUTED_and_not_cached` e `the_LAST_routing_wins_and_not_any_routing` |
| `Degradation::vram_exhausted` | `a_full_arbiter_declares_its_vram_exhausted` |
| *«l'ULTIMO routing e non uno qualsiasi»* | `the_LAST_routing_wins_and_not_any_routing`, che dispaccia **due** volte sullo stesso passo aperto — un routing degradato, poi uno pulito |
| il **ricalcolo** | `it_is_RECOMPUTED_and_not_cached`, che cambia il mondo **fra** le due domande e tiene fermo l'arbitro, così che l'unica cosa mossa sia il giornale |
| la **non-vacuità** dei due campi | `an_idle_machine_declares_nothing`, la sola sonda che parte da una macchina ferma **e** da un giornale vuoto |
| `DegradationError::Journal` | `a_journal_that_will_not_replay_is_not_an_answer_of_nothing_degraded` |
| `DegradationError::Record` | **due** sonde, una per strada: `a_record_that_will_not_decode_is_not_an_answer_of_nothing_degraded` e `a_routing_record_without_its_detail_is_not_an_answer_of_nothing_degraded` |
| `Arbiter::ceiling` | ogni sonda che legge `vram_exhausted`; il getter nasce col chiamante che lo pretende e non ne ha altri |

#### Le mutazioni, col proprio esito MISURATO

⚠️ Su un file **non tracciato** `git diff` è vuoto qualunque cosa contenga: un file nuovo va nell'indice **prima** della
campagna, e la campagna si misura su un albero pulito — gotcha **#107** di [`HANDOFF.md`](HANDOFF.md).
La campagna `M1`–`M10` sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «Le mutazioni, col proprio esito
MISURATO» del compito 8.

#### Le decisioni prese, e ciò che costano

⚠️ **La divergenza dichiarata su *«mantiene»*, del proprietario.** ADR-0019 e §6.7 dicono che il core *«mantiene uno
stato di degrado corrente, alimentato dagli eventi»*, parole che si leggono **anche** come mantenimento incrementale. Il
codice legge *«mantiene»* come *«espone»*, e la divergenza è scritta **accanto alla funzione** perché la lettura opposta
è del proprietario. La indicizza la riga 3 della tabella del Traguardo 6.

📌 **Le altre decisioni stanno nel sorgente, col loro perché:** l'errore proprio che compone `JournalError` e
`RecordError` (`E139`, la forma di `E104`) accanto a `DegradationError`; il tipo **corto** — connettività e salute dei
provider non hanno un campo, perché un campo sempre `false` si legge *«tutto bene»* invece di *«non lo so»* — e il costo
della rilettura, che si cura col **checkpoint** quando una misura lo chiede, in `crates/kernel/src/degradation.rs`;
`Arbiter` guadagna `ceiling()` e **nient'altro** (`E140`: passare i `Parameters` terrebbe due verità indipendenti, la
forma di `E25`), e **nessun terzo addendo**, perché le quote di ADR-0033 entrano via `admit` e stanno già dentro
`allocated()`, nel doc di `Arbiter::ceiling`. La voce aperta della §5.1 dell'arbitro **non** viene toccata.
⚠️ **`V27` e `Q18` restano `⚠️ parziale`** (condizione 12), e vivono nella **spec** (§8.3 e §8.4): `V27` vuole che
l'interfaccia lo dichiari, e un'interfaccia non c'è; `Q18` vuole una campagna DST col guasto di rete, e `network` non ha
implementazione reale.

### Il compito 9 del Traguardo 6 — le due proprietà di §5.7 che mancavano, e la riconciliazione che nessuno eseguiva (2026-09-02)

⛔ **`P-16`: la terza proprietà di §5.7 pretende una riconciliazione alla disconnessione.** `crates/kernel/src/client.rs`
porta `ClientGrants` e `on_disconnect`; le due campagne sono `crates/simulator/tests/gui_death_campaign.rs` (proprietà
**3**) e `crates/simulator/tests/worker_kill_campaign.rs` (proprietà **2**). Le proprietà di §5.7 si contano sul codice,
`grep -rn "^fn property_" crates/simulator/tests/`, una riga per numero; i commit del compito li conta `git log`.

| Artefatto | Che cosa lo esercita |
|---|---|
| `ClientGrants::on_disconnect` | `crates/kernel/tests/client_grants.rs`, le sonde contate con `grep -c '^#\[test\]' crates/kernel/tests/client_grants.rs`: la concessione torna, torna **solo quella di quel client**, un client che non tiene niente **non è un errore**, dopo la finestra la risposta è `AlreadyCollected` (`E154`), e una concessione **di un altro arbitro** è il difetto del chiamante e lascia registrato il resto — l'unica via che percorra il ramo `Err` |
| `ClientGrants::register` | ogni sonda del banco, e la campagna della proprietà 3 |
| `DyingGui` | `crates/simulator/tests/dying_gui.rs`, sul precedente di `crashing_journal.rs` |
| la proprietà **3** | `property_3_a_gui_that_dies_holding_a_grant_gives_it_back`, più `the_campaign_sweeps_every_world_this_scenario_has` |
| la proprietà **2** | `property_2_a_killed_worker_leaves_no_reservation_behind` |
| il **decodificatore** di `IpcMessage` | la campagna: la richiesta attraversa la porta **come byte** e viene decodificata dal core, non aggirata |

⛔ **La morte si legge dalla porta e da nient'altro.** `Ipc` non ha un evento di disconnessione — `accept`, `send`,
`receive` — quindi la riconciliazione è innescata **solo** da `Err(IpcError::Disconnected)`. Il comando che lo misura è
`grep -cE '\.has_died\(' <campagna>`, che conta le **chiamate**, e sulla campagna dà zero; `grep -c has_died` è un'altra misura,
perché conta anche i paragrafi che dichiarano di non chiamarla. La seconda direzione: lo stesso comando ancorato su
`crates/simulator/tests/dying_gui.rs`, dove chiedere alla finta è la cosa giusta, dà più di zero.

⛔ **La linea di base della campagna della proprietà 3 NON è zero, in due specie:** la **quota di presentazione del
core**, che ogni corsa riscuote dopo la riconciliazione, e un **secondo client registrato che non muore**. Con base zero
*«la somma torna alla linea di base»* sarebbe verde anche per una riconciliazione che rilascia **tutto** (`E156` ②, e
`MB2` lo mostra); il secondo client c'è perché è quello che la mutazione può raggiungere.

#### Le mutazioni, col proprio esito MISURATO

⛔ **Criterio di chiusura, non consiglio: i due oracoli di non-vacuità di ciascuna campagna si vedono ROSSI.** Per
`gui_death_campaign` l'oracolo **1** con `MB3` e il **2** con `MB4`; per `worker_kill_campaign` l'oracolo **1** con
`MC3`, il **2** con `MC1`, e il rilevatore di degenerazione con `MC2`. ⚠️ `MC1` ed `MC2` mutano il generatore,
`RngExt::below` in `crates/kernel/src/rng.rs`, mentre `MC3` muta il **banco**: `E155` toglie a questa campagna la finta
guidata dal seme, perché *«a decidere il kill è il banco e non il worker»*, quindi l'iniezione **è** il flusso del banco
e non esiste un difetto di produzione che possa far mancare un kill.
⛔ **L'oracolo 1 delle due campagne è un'asserzione PER SEME**, non un'aggregata: nomina il seme e vale su **ogni** seme.
*«Ogni recluta è stata uccisa»* lo tiene l'asserzione `running.is_empty()`. Il numero di semi lo stampa la riga
`DST gui death:` di `cargo test --locked -p simulator --test gui_death_campaign -- --nocapture`, e non si scrive qui.

⚠️ **`the_same_seed_chooses_the_same_operation` non è uccisa da nessuna mutazione ed è INFALSIFICABILE PER
COSTRUZIONE** — `from_seed` è una funzione pura dei suoi argomenti: è dichiarata tale nel proprio corpo (gotcha **#44**),
come la gemella in `crashing_journal.rs`, e **non conta come copertura**.
📌 **Le asserzioni si citano per TESTO e non per numero di riga:** dopo un'ondata di revisione i numeri non indicano
più l'asserzione giusta.

La tabella `M1`–`MB8` sta in [archivio](archivio/porta-di-qualita-storico.md), sezione «Le mutazioni, col proprio esito
MISURATO» del compito 9.

#### Le decisioni prese, e ciò che costano

| | |
|---|---|
| la proprietà **2** prende un **banco proprio** e non un angolo di `arbiter_campaign.rs` | quel file è **uno** scenario, e le sue costanti, il suo `Observed` e i suoi due oracoli descrivono quello. ⛔ **Il costo** (`E155`): un file nuovo, e un **bersaglio in più** che `scripts/gate.sh` deve nominare, perché il suo ultimo passo, «DST campaigns -- wall time», nomina le campagne **una per una**; `gui_death_campaign` e `worker_kill_campaign` ci sono, con la misura nelle due direzioni nel commento del passo |
| `on_disconnect` estrae **una coppia per volta** (`position` più `remove`) e non `drain` più ripartizione | su un `Err` le altre coppie restano registrate. ⛔ La concessione che **produce** l'`Err` è persa comunque — `release` la consuma prima di rispondere — e il limite è dichiarato sul metodo, in `crates/kernel/src/client.rs` |
| `ClientGrants` tiene un `Vec<(ClientId, Grant)>` a ricerca lineare | `ClientId` non deriva `Ord` né `Hash`, tolti **per sottrazione** in `ports::ipc`, e `HashMap` è vietata (gotcha **#12**): ridare la derive riaprirebbe quella decisione. Il perché sta accanto al tipo |
| la campagna della proprietà 3 gira a **un solo istante** | il mondo *«la gui muore DOPO la propria finestra»* sta in `a_disconnect_after_the_window_reports_already_collected`. ⛔ **Dichiarato e non taciuto** |
| la metà **temporale** di *«concessione valida»* si **conta** e non si asserisce | `Process::start` non prende `now` e non interroga l'arbitro, `GrantId` è privato, e nessuna API risponde *«questa concessione è ancora nei libri?»*. Pinzarla sarebbe un voto contro il prendere `E30`/`E39`, che è del proprietario — gotcha **#73**. La cifra non si scrive qui: la stampa la riga `DST worker kills:` di `cargo test --locked -p simulator --test worker_kill_campaign -- --nocapture`, coi kill totali, quelli dentro la finestra e quelli oltre. La indicizza la riga 26 della tabella del Traguardo 6 |

⚠️ **`E50` ed `E51` NON sono chiuse da `P-16`**, e `client.rs` lo dichiara nel proprio doc di modulo: quel ciclo decide
**quando** `promote` gira rispetto ad `admit`; questo risponde a **un** evento con **un** rilascio.

## Livello 3 — vuoto, e non è una svista

`clippy` gira come igiene del codice ma **non ha voce nella porta**: nessun V dipende da lui. ⚠️ **[C-S5-1]** Un rosso
della porta deve significare sempre «invariante violata», mai «stile discutibile», o si impara a ignorarlo.

⛔ **`cargo fmt --all --check` resta fuori per lo stesso argomento**, valutato il 2026-08-09. La regola 1 di §7.1.1 lo
rifiuta su **entrambi** i rami, con la stessa coppia di domande con cui quella sezione caccia `clippy`:

| Ramo | La domanda | Su `rustfmt` |
|---|---|---|
| **1a** | difende un `V`, un'`I` o un `Q` nominato? | **no** — nessuna proprietà del sistema dipende da dove va a capo un letterale di struct |
| **1b** | quale riga del catalogo smette di essere **vera**, se lo cancelli? | **nessuna** — stesso esito che su `clippy`, e la §7.4.3 regge parola per parola |

⚠️ **E non sarebbe comunque una decisione da prendere qui.** *«Il livello 3 del catalogo è vuoto»* sta fra le voci
**non rilitigabili** della §7 del [compendio](COMPENDIO.md): aggiungere un passo di stile alla porta richiederebbe un
**ADR che superi la §7.4.3**, non un commit. ⚠️ `rustfmt` non è nemmeno un lint, ma sulla scala di §7.1.2 si comporta da
livello **3**: si cancella, e si aggira per singolo elemento con una riga — `#[rustfmt::skip]`.

📌 **Il costo accettato, ed è reale:** `cargo fmt --all --check` resta un segnale che **nessuno fa rispettare**. A tenerlo
verde è l'igiene di chi scrive — non un controllo. Chi lo trova rosso lo corregge nel file.

⛔ **Correggerlo fa scattare la trappola dei fine-riga.** Qualunque strumento che riscrive un sorgente ne normalizza i
fine-riga senza dirlo, e `cargo fmt` è uno di questi: su un file con `CR` nell'indice `git diff --stat` dichiara
cambiate tutte le righe. Un file si ripristina da una **copia byte-esatta presa prima**, mai con `git checkout --`, che
cancellerebbe il lavoro non committato (gotcha **#48**). ⚠️ **Anche `git stash` seguito da `git stash pop` la fa
scattare**, senza riscrivere niente di suo: con `core.autocrlf=true` la coppia rimaterializza il file dall'indice
**CRLF**, il diff non lo mostra perché il filtro `clean` rinormalizza in scrittura, e a spostarsi in silenzio è il
censimento di `git ls-files --eol`.

## ⛔ LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA (2026-08-25)

⛔ **Raccolte qui alla chiusura del traguardo:** più voci aperte sullo stesso oggetto sono il modo in cui una di esse
smette di esserlo senza che nessuno l'abbia chiusa. Le voci del Traguardo 5 **vivono** nei riquadri di questo file,
nell'errata in testa al [piano](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) e nei riquadri
della §6 del [compendio](COMPENDIO.md): questa tabella li **indicizza**, non li sostituisce.

📌 **Come si leggono le case della colonna «Dove è dichiarata».** Un riquadro si nomina **per titolo**, mai per numero
di riga, perché un numero di riga è un puntatore senza guardia. Le case *«§6 del compendio»* vivono dal 2026-09-09 in
[`archivio/stato-storico.md`](archivio/stato-storico.md), parola per parola — compreso il punto ④ del riquadro *«ciò
che il Task 12 lascia al Task 13»*, che è il censimento autorevole delle voci ereditate. Un riquadro che questo file non
porta più sta in [archivio](archivio/porta-di-qualita-storico.md), col suo titolo.

⛔ **Come è stata costruita: non è un censimento dichiarato completo, è ciò che questi comandi hanno restituito**, letti
riga per riga e senza troncarli (gotcha **#70**).

```
$ grep -n "VOCE APERTA\|VOCI APERTE" docs/porta-di-qualita.md
    → le intestazioni e i riquadri delle voci aperte. ⚠️ Il comando trova anche sé stesso e
      l'intestazione di questa sezione: sono case del grep che censisce.

$ grep -cE '^\| \*\*`?E[0-9]+' <piano>
    ⛔ QUANTE SIANO NON È SCRITTO QUI: lo dice il comando, sul piano che si sta guardando.
    ⛔ IL FILTRO CHE LE RESTRINGE NON BASTA, ed è il gotcha #70 in ENTRAMBE le forme. Il filtro
        "REGISTRAT[AE], NON PRES|NON PRESA|REGISTRATE E NON PRESE|DICHIARAT[AO], NON
         PINZAT|DICHIARATA E NON|per il proprietario|decisione del proprietario"
      ne restituisce di già chiuse, e ne manca di aperte: E53, E83, E100 ed E140 rendono
      zero — misurato su git show ac65504:<piano>, il piano com'era alla raccolta — e si
      trovano leggendo la §6 del compendio e i riquadri.

$ grep -nE "riga di catalogo" docs/porta-di-qualita.md | grep -iE "non ha|non hanno|nessuna riga"
    → i siti in cui un compito dichiara che le proprie sonde non hanno una riga di catalogo
      propria. ⚠️ Rende anche falsi positivi, in cui il «non ha» governa un'altra
      proposizione, e un sito può comparire due volte, intestazione e corpo.
```

⚠️ **Ciò che questa tabella NON copre.** Le voci aperte del **Traguardo 3** vivono nella tabella *«Cosa il Traguardo 3
lascia aperto»* della §6 del [compendio](COMPENDIO.md) — e con esse la **guardia che manca al puntatore** del prossimo
passo, l'elenco dei semi e la guardia sui pesi. ⚠️ **[C-S5-2]** Le **dieci sonde permanenti dell'audit** vivono nella
tabella consolidata del 2026-08-18 qui sopra, e la riga che le nomina qui è un **rimando** e non una seconda copia
(gotcha **#68**). ⚠️ **[C-S4-1]** E *«[`semi-dst.md`](semi-dst.md) non ha un chiudente»*, col chiusore **il
proprietario**, è eredità del **Traguardo 4**: dichiarata nel riquadro di `semi-dst.md`, vale una riga di esclusione e
non una riga di tabella.

⛔ **La colonna «Chi la chiude» è la ragione per cui la tabella esiste, e non dice sempre
«il proprietario».**

| # | Voce | Che cosa resta aperto | Dove è dichiarata | Chi la chiude |
|---|---|---|---|---|
| 1 | **R1** | `WorkDescriptor` (`crates/kernel/src/arbiter/resource.rs:164`) dista **una lettera** da `WorkerDescriptor` (`crates/kernel/src/ports/process.rs:76`), che è un'altra cosa | pre-controllo del piano, e §6 del compendio | il **proprietario**: un rinomino |
| 2 | **R3** | la riga di catalogo `Q2 · §5.1` è **una** e formulata in **una** direzione — *«MiB assegnati a millisecondi»* — mentre i casi che mordono sono **quattro**, su due regole e in due direzioni | riquadro *«la riga di catalogo `Q2 · §5.1` è UNA e in UNA direzione»* di questo file | il **proprietario**: §7.4 è spec, vincolo globale 7 |
| 3 | **R4** | `Preemption::{Never, After(Millis)}` è **una** voce dove la tabella dei campi di §5.2 ne elenca **due**, `preemptible` e `release_grace` | pre-controllo del piano, e §6 del compendio | il **proprietario** |
| 4 | **R5** | due celle del catalogo nominano `Concessa`, `InCoda` e `InRevoca`; dal Task 4 il codice scrive `Admission::{Granted, Queued, Refused}` e `PreemptibleState::Revoking`. Il perimetro comprende i **documenti di design**, che si dichiarano fonte di verità: il censimento lo dà `grep -noE '`(Concess[ao]|Rifiutat[ao]|InCoda|InRevoca|InDubbio|Annullat[ao]|interattivo|verificabile|idempotente|irripetibile)`' docs/design/*.md`, più i **nomi di stato nudi** dentro i blocchi mermaid di `design/01` e `design/02`, che sono un'altra specie: etichette del modello, non riferimenti in backtick. Una sola traduzione è fatta: `interattivo` → `interactive` in `design/02`, dove le altre due voci della **stessa enumerazione** erano inglesi — chiudeva un **dialetto misto dentro un'enumerazione sola** | celle `V4` e `I2 · §5.3` della tabella «Livello 1» di questo file, **più** il richiamo del 2026-08-27 accanto alla tabella del profilo in [`design/02`](design/02-arbitrato-gpu.md) | il **proprietario**: §1.0 contro §7.4, e contro il **vocabolario degli ADR**. ⛔ Tradurre `Concessa`, `InRevoca`, `InDubbio` e le tre classi d'effetto tocca parole che [ADR-0005](adr/0005-arbitrato-gpu-su-due-dimensioni.md), [ADR-0007](adr/0007-giornale-write-ahead-e-riconciliazione.md) e la **spec approvata** scrivono in italiano di proposito, e la §4 del compendio dichiara **accettata** la traduzione fra la parola di un ADR e il nome nel codice: non è un allineamento, è una **scelta di convenzione** |
| 5 | **E140** | il catalogo scrive `uccidi` in §7.4.1 blocco C, in §6.10.2 e in §6.10.5; il sorgente scrive `fn kill` in `crates/kernel/src/ports/process.rs` | cella `I2 · §6.10` di questo file, ed errata del piano | il **proprietario**: stessa specie di `R5`, ma nata col Traguardo 2 |
| 6 | il costruttore di `Grant` | `trybuild` prova la direzione **da fuori la crate**, e il caso lo dichiara nel proprio commento: da **dentro** la crate un costruttore `pub(crate)` resterebbe fuori dalla sua portata. `grep -rn "impl Grant" crates/kernel/src/` non restituisce niente, e l'unico sito che ne conia una è `Arbiter::issue`, privato | commento di `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs` | il **proprietario**: sarebbe una **riga di catalogo nuova** |
| 7 | la contro-sonda di `Q8 · §5.2.1` | la cella nomina *«la proiezione di presentazione lo legge»*; `grep -rn "cold_start" crates/ --include=*.rs` dà come lettore `cold_start_is_readable_outside_the_decision_path` in `crates/kernel/tests/arbiter_resource.rs`, che questo registro dichiara **una finta** | disegno del Traguardo 5, e cella `Q8 · §5.2.1` di questo file | il **proprietario**: riformulare la cella è §7.4 |
| 8 | la divergenza su §5.1 | §5.1 dice *«I tre addendi sono parametri consegnati»*; `crates/kernel/src/parameters.rs` porta `executor_turn_limit` e `total_vram`, cioè **un** addendo su tre — gli altri due sono la riserva di due **concessioni permanenti** | disegno del Traguardo 5, e §6 del compendio | il **proprietario** |
| 9 | il **quinto** caso `compile_fail` | un secondo `start` con lo stesso `Grant` è `E0382`, perché `Grant` non deriva `Copy` né `Clone`: **misurato e non preso** | §6 del compendio, voce ① del Task 11 | il **proprietario**: se pretenda una riga propria lo decide §7.4 |
| 10 | la **convenzione** sulla ricevuta | *«leggere da un worker ← una ricevuta»* entra fra le **coperte** del blocco B, ma `SingleReceipt::new` è `pub`: il caso prova l'**arità** e non l'**autenticità**, mentre `Q8 · §5.2.1` e `V3` sono state tenute a PARZIALE per lacune più strette | §6 del compendio, voce ② del Task 11, e cella blocco B `I5 · Q4` | il **proprietario**: è una domanda di **coerenza**, e va decisa vedendola |
| 11 | le sonde dei compiti **senza riga di catalogo** | ai siti che il terzo `grep` qui sopra restituisce, un compito dichiara che le proprie sonde non hanno una riga propria e vivono sotto una riga altrui | il **blocco dei comandi** qui sopra, terzo `grep` | il **proprietario**: §7.4 è spec |
| 12 | le **dieci sonde permanenti dell'audit** | nessuna ha una riga nel catalogo §7.4 — ⚠️ **questa riga è un rimando**, e il contenuto sta nella tabella consolidata del 2026-08-18 qui sopra ⚠️ **[C-S4-1]** | tabella consolidata dell'audit, in questo file | il **proprietario**, dal 2026-08-18 |
| 13 | **E21** | ✅ **CHIUSA dal compito 1 del Traguardo 6, `c4cf942`.** L'identità è `ArbiterId`, **consegnata** in `Parameters` e mai coniata (§6.1.3, ADR-0034), e `release` la confronta **prima** di guardare i propri libri. Mutando `arbiter_id()` a una costante, `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` è **l'unica** sonda che muore in tutto il workspace; il letterale condiviso che la rendeva a metà vacua è la voce `E8` dell'errata del piano del Traguardo 6 | riquadro *«ciò che `release` compra davvero, e ciò che non compra»* di questo file, e accanto a `ReleaseError` nel sorgente | il **proprietario**: dare un'**identità** all'arbitro |
| 14 | **E30** | ✅ **CHIUSA dal compito 1 del Traguardo 6, `9ecc13d`.** `release` risponde `Result<Released, ReleaseError>`, e `Released` ha **due** vie — `Now(Mib)` e `AlreadyCollected`. Una concessione **propria** non è mai un `Err`, comunque sia andata la sua finestra; `UnknownGrant` significa **una** cosa sola: la concessione di un altro arbitro. ⚠️ **Due ingressi che il doc dichiarava misurati non li tiene nessuna sonda** — `release` al confine esatto e i due lati della grazia: voce `E12` dell'errata, **registrata e non presa** | accanto a `ReleaseError` in `crates/kernel/src/arbiter/mod.rs`, e riquadro *«`release` risponde `UnknownGrant` anche a una concessione PROPRIA ma SCADUTA»* di questo file | il **Traguardo 6, insieme a `R6`** |
| 15 | **E31** | `saturating_add` può produrre **sovra-ammissione** al limite superiore: con `ceiling = u64::MAX` un secondo `admit` da 1 MiB torna `Granted` | errata del piano | il **proprietario** |
| 16 | **E32** | `crates/kernel/src/parameters.rs` e `crates/kernel/src/arbiter/mod.rs` sono **mutuamente dipendenti** — legale in Rust, sono moduli, ed è dettato dal piano | errata del piano | il **proprietario** |
| 17 | **E47** ① | `promote` restituisce un `Vec<Promotion>` senza `#[must_use]`, e `arbiter.promote(now);` da solo compila anche con `-D warnings` | riquadro *«`promote` restituisce un `Vec<Promotion>` senza `#[must_use]`»* di questo file, ed errata | il **proprietario** |
| 18 | **E53** | due frasi di doc che nessuna sonda tiene, cioè **due mutanti vivi dichiarati**: sono i paragrafi con cui `E50` ed `E51` furono scritte nel sorgente | errata, e accanto ai due paragrafi nel sorgente | il **proprietario** |
| 19 | **E70** | dentro una corsia la vittima è la **più vecchia**, e §5.3, §5.3.1 e `design/02` tacciono: *«la più piccola che basta»* è altrettanto difendibile | errata, e accanto alla frase nel sorgente | il **proprietario**: è una politica senza risposta giusta |
| 20 | **E94** | la policy è un **secondo valore consegnato** ad `Arbiter::new`, mentre §2.8.2 e ADR-0034 parlano di **un** valore che porta i parametri risolti | errata del piano | il **proprietario** |
| 21 | **E104** | una **dominanza** fra sonde della campagna di mutazione, dichiarata e **non cancellata**: una campagna è un campione, non una dimostrazione | errata del piano | il **proprietario** |
| 22 | **E151** | mutante vivo `M9`: tolta ad `Arbiter::release` la riscossione delle scadute, l'intero workspace resta verde — **dichiarato e non pinzato**, perché la sonda che lo ucciderebbe congelerebbe la scelta che `E30` mette davanti al proprietario (gotcha **#73**) ⚠️ **[C-S3-6]** | errata, e campagna di mutazione del Task 12 in questo file | il **proprietario** |
| 23 | l'**aiutante** dei due scrittori di record | se `Untrusted::promote` e `Arbiter::set_policy` debbano condividere un aiutante che tenga in passo il `kind` e l'operazione; intanto ciascuna ha la **propria** sonda | §6 del compendio, e `crates/kernel/src/reconcile.rs` | il **proprietario** |
| 24 | **E50** | fra corsie `promote` **scavalca**, cioè fa nell'insieme delle corsie ciò che il suo stesso commento rifiuta dentro una corsia | riquadro *«LO SCAVALCAMENTO CHE `promote` RIFIUTA DENTRO UNA CORSIA, FRA CORSIE LO FA»* di questo file, ed errata | **chi costruirà il primo ciclo di orchestrazione** |
| 25 | **E51** / **E100** | `admit` non consulta mai la coda, quindi un ritardatario la scavalca; dal Task 8 l'inversione di priorità è **raggiungibile in produzione** e non più teorica | riquadro *«`admit` NON CONSULTA MAI LA CODA»* di questo file, ed errata | **chi costruirà il primo ciclo di orchestrazione** |
| 26 | **R6** | ✅ **CHIUSA dal compito 1 del Traguardo 6, `822db6d`.** `Worker::kill` restituisce `Killed`, che porta il `Grant` **fuori** da ogni `Result`, perché la riserva è un fatto dei **libri** e non della salute del processo. E `Process::start` risponde `Started`, il cui ramo `Rejected` **riporta indietro la concessione**, invece di lasciar cadere una riserva che solo la spazzata recupererebbe a finestra scaduta. Il costo nelle implementazioni della porta è la voce `E3` dell'errata | accanto a `Grant` nel sorgente, e §6 del compendio | il **Traguardo 6** |
| 27 | **E152** | ✅ **CHIUSA dal compito 9 del Traguardo 6** — `9342fc1`, che porta la riconciliazione alla disconnessione (`kernel::client::ClientGrants`, finding **P-16**), e le due campagne. La riga di catalogo di livello 2 è **coperta**, e le **cinque** proprietà di §5.7 sono nominate una per una nella cella della campagna DST di questo file. ⚠️ **La campagna dell'arbitro ne tiene tre, ed è giusto così:** le altre due non girano dentro l'esecutore e hanno banchi propri, `crates/simulator/tests/worker_kill_campaign.rs` e `crates/simulator/tests/gui_death_campaign.rs` | cella della campagna DST di questo file, ed errata | **chiusa** |
| 28 | la classe d'effetto della transizione di policy | è `Idempotent`, e la transizione oggi **scambia un oggetto**: la classe si rilegge quando la transizione avrà un contenuto | §12 del disegno del Traguardo 5, e il sorgente | **L2**, quando arriverà il contenuto dello sfratto |
| 29 | **E83** | il `filter` sull'ammissibilità in `lanes` è un **mutante vivo garantito**, a comportamento nullo: toglierlo lascia il workspace verde, e resta perché rende vera **per costruzione** la frase che i due insiemi sono lo stesso insieme | errata, e accanto alla frase nel sorgente | ⛔ **nessuno**: non c'è niente da decidere, e la non-difendibilità è scritta con la misura |
| 30 | **E146** | [`riferimenti.md`](riferimenti.md) non è stato toccato in tutto il Traguardo 5: le misure vivono **qui**, accanto al controllo che difendono. Misurato il 2026-08-25 con `git rev-list --count dc6ac4c~1..HEAD -- docs/riferimenti.md`, che rendeva **zero** | errata, e §6 del compendio — allargata a ogni compito dal Task 5 in poi | il **proprietario**: scegliere fra *«spostare le misure»* e *«cambiare la regola»* |
| 31 | i **fine-riga**, misura nuova su una regola già presa | `git ls-files --eol` dice che **nell'indice** i file tracciati sono LF, compresi quelli che nell'albero di lavoro sono CRLF: `core.autocrlf` vale `true`, quindi il `diff` è protetto più di quanto i documenti dichiarino | §6 del compendio, dal 2026-08-20 | il **proprietario**: riaprire una decisione presa |
| 32 | la transizione **`InCoda --> Annullata`** | la macchina a stati di [`design/02`](design/02-arbitrato-gpu.md), che la §5.3 della spec adotta come propria, dichiara che dalla coda si esce anche per **annullamento o scadenza**: nell'arbitro non esiste nessun meccanismo. `grep -rniE 'cancel|annull' --include=*.rs crates/kernel/src/` non ha **nessun** riscontro; gli unici punti che mutano `queues` sono `enqueue`, `promote` e `new`, e `collect_expired` fa `retain` **solo** su `held`. Un biglietto consegnato è quindi **immortale**, mentre `design/02` promette all'utente *«l'opzione di annullare»*. ⛔ **E la conseguenza ha già costato codice:** `StartupError::ReservedQuota` esiste nella radice di composizione perché la seconda quota permanente torna `Queued` e nessuno la servirà mai — un tampone per **un** caso, mentre il buco resta per ogni altro chiamante | il richiamo del 2026-08-27 accanto alla macchina a stati in [`design/02`](design/02-arbitrato-gpu.md), col rimando in [`design/01`](design/01-topologia-dei-processi.md) | il **proprietario**: la scelta è fra **costruire** l'annullamento e **togliere** la transizione dal diagramma, e nessuna delle due è dell'agente. Voce nata dal finding **AUD-044**, sull'**arretrato anonimo**: la §9 del disegno del Traguardo 5 apre con *«ogni riga ha un indirizzo»*, e questo ramo non stava né fra le cose fatte né fra quelle rimandate |
| 33 | **C-1** | ✅ **CHIUSA il 2026-08-31, e la scelta è del proprietario: `bincode` 2.0.1 RESTA, §6.1.1 non si riapre.** ⛔ **Decisa contro l'evidenza e non attorno:** la compatibilità del fork è **misurata** (**M-12**) e **regge**, quindi il «no» non è un'omissione. Cinque ragioni, e l'ultima decide: ① l'avviso dice **non mantenuto**, non **rotto**, e nessuna versione corretta esiste perché il monte dichiara la 2.x **completa**; ② I4 rinuncia al versionamento e il canale è **privato**, quindi il formato è congelato **per disegno** — una libreria finita è ciò che quel canale chiede — mentre ADR-0031 esiste per tenere **piccolo** il grafo dentro I3, e il fork lo farebbe crescere di **una voce netta**; ③ restare lascia un debito **dichiarato**, adottare ne creerebbe uno **nuovo e silenzioso** — compatibilità misurata su pochi casi, manutentore solo, e **nessun controllo** che ci direbbe di una rottura futura; ④ *novità non è maturità*, e **RustSec non raccomanda il fork**: le alternative che l'avviso nomina sono cadute, ciascuna con la propria misura; ⑤ ⛔ **la radice di C-1 non è questa crate:** C-1 nomina il **buco fra due criteri** — nessuno chiede come stia la libreria al **nostro** capo — e sostituire **una** libreria cura una crate lasciando il buco aperto per le altre. ⚠️ **Il residuo è nominato e non chiuso:** la cura alla radice è la voce **X-3** dell'audit del 2026-08-27, *nessuna scansione degli avvisi*, e resta **del proprietario** perché aggiungere un passo al cancello è il vincolo globale 7; nominarla è **parte** di questa decisione, mai un suo sostituto ⚠️ **[C-S5-4]**. Le evidenze della misura stanno in [`riferimenti.md`](riferimenti.md), sezione C-1 | sezione della decisione 5 (C-1) di [`riferimenti.md`](riferimenti.md), che porta le fonti e i candidati; nota accanto alla voce in `crates/kernel/Cargo.toml`; gotcha **#64** di [`HANDOFF.md`](HANDOFF.md); riga *«schema IPC»* della §4 del [compendio](COMPENDIO.md) | il **proprietario**: §6.1.1 è **spec**, vincolo globale 7, decisione **D12** |
| 34 | **E94** | ⛔ **`RoutingDetail` è una TERZA BOCCA della classe di AUD-050, nata col compito 6 del Traguardo 6.** `RecordV1::routing` è `pub`, `RoutingDetail` è `pub` **coi campi `pub`**: un chiamante qualunque mette una `String` calcolata a runtime in `model`, e il `Debug` scritto a mano la stampa **intera**, col `reason` ancora un letterale `'static` a posto — riprodotto da **fuori** la crate. ⚠️ **Non è un difetto oggi**: `Candidate::model` è `&'static str`, quindi per la via di produzione non entra niente; il buco è nel **tipo**. ⚠️ **`VerdictDetail` non è coinvolta**: `bool` e `u64`. ⛔ **Il compito 7 la TRIPLICA:** il suo `PermissionDetail` è dettato con **due** campi `String` pubblici ⚠️ **[C-S5-3]** | voce **E94** dell'errata del piano | il **proprietario**: è la decisione di AUD-050 su un tipo nuovo, e **costa meno prenderla PRIMA del compito 7** che dopo |

⚠️ **Fra le righe qui sopra ce ne sono il cui chiusore NON è il proprietario.** Lette sparse, sembrerebbero tutte in
attesa del proprietario, che è il modo in cui una voce smette di essere aperta senza che nessuno l'abbia chiusa. Il
**comando** le **nomina** invece di contarle — un elenco invecchia, una regola no:

```
awk -F'|' '/^## .*LE VOCI APERTE DEL TRAGUARDO 5/{s=1} s&&/^## Cosa la porta NON controlla/{s=0} s&&/^\| [0-9]+ \|/&&$(NF-1)!~/proprietario/{print $3}' docs/porta-di-qualita.md
```

✅ **Provato nelle DUE direzioni** (gotcha **#24**): rende esattamente le voci il cui chiusore non è il proprietario, e la
forma complementare — `~` al posto di `!~` — non ne nomina **nessuna**; le righe che il filtro attraversa sono tutte e
sole quelle della tabella. ⚠️ **È ancorato alle due INTESTAZIONI e non a numeri di riga**: se una delle due cambia il
comando **tace**, e un silenzio si nota meglio di una cifra sbagliata. ⚠️ **E l'ancora non può portare l'emoji
dell'intestazione**: `⛔` è multi-byte e un `.` di `awk` ne copre **un byte solo** — la prima forma rendeva zero righe.

## Cosa la porta NON controlla, in questo traguardo

Righe del catalogo §7.4 che oggi **nessun file implementa** — o che lo sono **in parte**, e allora la riga dice **quale**
parte. Stanno qui perché un registro che le omettesse lascerebbe credere che siano coperte, e una riga che dicesse
«scoperta» dove qualcosa c'è mentirebbe nell'altro verso.

⚠️ **Chi le chiude.** I **due residui di `SystemReactor`** non nominano nessuno **perché nessuno li chiude con questi
mezzi**: il «chi» è **vuoto per misura**, non per dimenticanza. Il chiudente del **portachiavi** è scritto nella spec, e
non qui: le righe **V34**, **Q24** e **Q17** della §8 sono ⏳ **rimandato** con innesco, sul precedente di **V16**
(§8.5.3.1) — senza nessuna credenziale nel perimetro un controllo proverebbe **l'assenza di una cosa che non c'è**,
gotcha #17. Quale sia l'innesco lo dice la colonna *Innesco* di §8.3 e §8.4, in una casa sola.

| Riga del catalogo | Perché non c'è ancora |
|---|---|
| il **resto** del blocco **B** di §7.4.1 — i **gettoni** | ✅ **Chiusa dal 2026-09-01: nel blocco B nessuna riga è scoperta né parziale.** Le cinque righe e chi le tiene: `promuovere testo a istruzione ← la porta journal` (V19) da `crates/kernel/tests/compile_fail/promote_without_journal.rs`, che nomina quella riga di catalogo nella propria intestazione; *«avviare un worker ← una concessione»* — *«senza → non compila»* da `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs`, *«con → compila»* da `releasing_gives_back_exactly_the_reservation` in `crates/kernel/tests/arbiter_admission.rs`, che ottiene un `Grant` da `admit` e lo consuma; il `Worker` (**riga 1** di §6.10.5) da `crates/kernel/tests/compile_fail/talking_without_the_handle.rs` (`E0599`) e la **ricevuta** (**riga 3**) da `crates/kernel/tests/compile_fail/reading_without_a_receipt.rs` (`E0061`), con le contro-sonde in `crates/kernel/tests/worker_tokens.rs`; `Q13`, *«eseguire una richiesta ← una prova di conformità»*, dal filtro dei vincoli in `crates/kernel/src/gateway/mod.rs`, con `Conforming` a campi privati coniato dal solo `resolve` — le metà sono **due** e i casi due, `crates/kernel/tests/compile_fail/dispatching_an_unfiltered_candidate.rs` (`E0308`) e `crates/kernel/tests/compile_fail/conforming_has_no_constructor.rs`, e la direzione *«filtrato → compila»* è `a_conforming_candidate_is_chosen_and_nothing_is_degraded` in `crates/kernel/tests/gateway_decisor.rs`. 📌 **Convenzione:** una riga di catalogo vale UNO nel numeratore qualunque sia il numero di casi che la difendono. ⛔ Un costruttore di `Grant` dietro una feature di test **è stato valutato e scartato**: creerebbe il secondo modo di ottenere una concessione che §5.6 esiste per togliere dal compilatore. ⚖️ Un caso `compile_fail` per il **secondo dispaccio** dello stesso gettone (`E0382`, perché `Conforming` non deriva né `Copy` né `Clone`) sarebbe una **riga di catalogo nuova**, cioè spec: **registrato e non preso**, come il quinto caso di `Grant`. Dettaglio nella sezione «Il gettone di conformità del gateway» |
| il resto del blocco **C** di §7.4.1 | ✅ **Coperte tutte le righe dal 2026-09-03**: l'ultima è stata `V5`, dal caso `crates/kernel/tests/compile_fail/effect_without_its_class.rs` — `` error[E0061] `` con la nota *«argument #1 of type `EffectClass` is missing»* — con la contro-sonda *«un effetto con la classe compila»* in `every_effect_class_survives_the_round_trip_and_the_three_differ_in_the_bytes` di `crates/kernel/tests/record_shape.rs`. Gli altri casi che questa cella nomina: `record_without_version.rs`; `Q8 · §5.2.1` da `crates/kernel/tests/compile_fail/admission_reads_cold_start.rs` (`E0609`); `V2` da `crates/kernel/tests/compile_fail/admission_without_profile.rs`; `V3` da `crates/kernel/tests/compile_fail/two_policies_at_once.rs`, un errore di **arità**, con la contro-sonda doppia in `crates/kernel/tests/arbiter_policy.rs` — una policy sola compila, **e** la transizione resta un passo giornalato (§5.4) per `Arbiter::set_policy`; `I2 · §6.10` da `crates/kernel/tests/compile_fail/instructing_after_the_kill.rs` (`E0382`); `I5 · §6.10` da `crates/kernel/tests/compile_fail/reading_twice_from_one_receipt.rs` (`E0382`); `V10` da `crates/kernel/tests/compile_fail/sensor_modifies_the_artefact.rs` (`E0594`), con la contro-sonda `a_passing_sensor_writes_a_verdict_and_opens_nothing` in `crates/kernel/tests/sensor_ring.rs`. ⛔ **Una contro-sonda vive nel banco che implementa ed esegue il tratto, non accanto al caso negativo** — sarebbe il gotcha **#49**. 📌 **Convenzione:** una riga vale UNO qualunque sia il numero dei casi `compile_fail` che la difendono, e una riga `parziale` non entra fra le coperte. ⛔ **Si ricontano sul catalogo, mai da questa frase né dalla descrizione:** il denominatore lo muove chi tocca il catalogo, e se ne accorge; **il numeratore lo muove chi scrive un caso**, che il catalogo non lo apre nemmeno. 📌 **Un tipo che esiste non è un controllo che scatta**: `V5` è rimasta scoperta finché nessun caso esercitava `EffectClass`, che pure era un campo obbligatorio del record |
| i test di contratto per le **altre** famiglie di porte | ✅ **coperte:** `reactor`, sonde R1…R6; `journal`, `crates/kernel/tests/journal_contract.rs`, sonde **J1…J16**, e l'accordo fra le due implementazioni è **tenuto a ogni commit** da `crates/platform/tests/journal_contract_real.rs` (sonda **J12**), anche dove una guardia sbagliata divergerebbe; `ipc`, contro il solo trasporto vero per disegno (D82 del piano della parte 2 del sotto-progetto 2) — `crates/kernel/tests/contract/ipc.rs`, inclusa da `crates/platform/tests/ipc_contract_real.rs`; e la settima famiglia, `custody`, con la sua suite. ⛔ **Una suite di conformità vale la prova che DUE implementazioni rispondono lo stesso**, e *«misurato una volta»* non è *«tenuto a ogni commit»*. ⚠️ Ciò che resta scoperto di `journal` è dichiarato in **tre** voci aperte — la distinzione di ADR-0018 fra potato e mai registrato, la terza risposta di `prune`, e la divergenza fra le due nozioni di «in dubbio», finding **AUD-006**. Restano scoperte `filesystem`, `network` e `process` |
| **due residui dichiarati dentro `SystemReactor`**, e stanno qui perché un registro che li tacesse mentirebbe | ⛔ sostituire `Some(self.now())` con `Some(deadline)` in `wait_until` **non fa scattare nulla**: la conformità non può coglierlo perché **sulla finta le due espressioni coincidono**, e distinguerle sulla vera richiederebbe l'overshoot dello sleep del sistema operativo, che nessuna piattaforma garantisce — un controllo verde per fortuna e rosso per sfortuna, cioè peggio di uno assente (gotcha #24). ⚠️ E `R5` prova che `wall_time()` non è **ferma**, non che sia **esatta**: l'esattezza vorrebbe una seconda sorgente di tempo |
| ~~i **byte congelati** del record durevole~~ ✅ **CHIUSA il 2026-08-10, col Task 10** | `crates/kernel/tests/frozen_bytes.rs` congela i record, e le varianti sono state rinumerate una per una — sonde **F1…F6** nel livello 2. ⛔ **Resta scoperta UNA sola cosa:** l'oracolo è un file che qualcuno può riscrivere, e la difesa è che la riscrittura **si legge nel diff** — non che sia impossibile. È la stessa forza e la stessa debolezza del gotcha **#25**, dichiarata da ADR-0036 fin dall'inizio. Vincolo 14 della §11 del [compendio](COMPENDIO.md) |
| la **campagna DST** — ✅ **COPERTA dal 2026-09-02** — e l'elenco versionato dei **semi** di V31, che resta **SCOPERTO** | ⛔ **Le cinque proprietà di §5.7, col banco che tiene ciascuna**, perché «coperta» senza l'elenco è un'affermazione non verificabile: ① *la somma delle concessioni non supera il budget* — `property_1_the_sum_never_exceeds_the_total_on_any_seed`, `crates/simulator/tests/arbiter_campaign.rs`; ② *nessun processo è attivo senza concessione valida* — `property_2_a_killed_worker_leaves_no_reservation_behind`, `crates/simulator/tests/worker_kill_campaign.rs`; ③ *la gui muore tenendo una concessione discrezionale* — `property_3_a_gui_that_dies_holding_a_grant_gives_it_back`, `crates/simulator/tests/gui_death_campaign.rs`; ④ *una transizione di policy interrotta lascia un passo riconciliabile* — `property_4_a_severed_transition_leaves_a_reconcilable_step`, `arbiter_campaign.rs`; ⑤ *una concessione scaduta non resta allocata* — `property_5_expiry_frees_the_budget_under_the_scenario`, `arbiter_campaign.rs`. Si ricontano **sul file**, `grep -rn "^fn property_" crates/simulator/tests/`, non per «tre più due». ✅ **La sonda che la cella di catalogo pretende è eseguita** — si rompe l'ammissione, e `cargo test --locked -p simulator --test arbiter_campaign` fallisce nominando il **seme** e i **due valori** (§5.7.1); le uscite stanno in [archivio](archivio/porta-di-qualita-storico.md), in questa stessa riga. La contro-sonda — *«senza guasto iniettato, nessun passo in dubbio»*, `C7a` — vive in `dst_campaign.rs`, e `crates/kernel/tests/executor_determinism.rs` gira **C1 e C2** su `SeededRng` e `VirtualReactor`. 📌 **Lo stato del blocco §7.4.2, riga per riga**, ricontato **sul catalogo** e non per sottrazione — `awk '/^#### 7\.4\.2/{f=1} /^#### 7\.4\.3/{f=0} f' <spec> \| grep -c '^\| '` rende le righe più l'intestazione: **coperte** l'allow-list sul grafo spedito, l'allow-list sul grafo di build, il cancello senza OS, `check-docs.sh`, i byte congelati, gli attributi delle crate vincolate, il build script, i byte consumati dal frame e la campagna DST; **parziali** i soli test di contratto; **scoperte** il portachiavi, il punto d'uscita verso la rete e l'elenco versionato dei semi di V31. ⚠️ **L'elenco dei semi resta SCOPERTO** perché nessuna campagna ha trovato un difetto di prodotto: non c'è nessun **seme colpevole** da versionare |
| ~~i **byte consumati** pari alla lunghezza dichiarata dal frame~~ ✅ **CHIUSA il 2026-08-31, col Compito 3 del Traguardo 6** | La busta è `crates/kernel/src/framing.rs`, **quattro byte big-endian**, decisi lì e in nessun altro posto; lo schema è `crates/kernel/src/wire/worker.rs`, il cui `decode` verifica i byte consumati. ⛔ **Chiusa nelle DUE direzioni, e sono due guasti distinti con prenditori distinti:** il **troncamento** e la **coda fuori dalla busta** li prende la lunghezza dichiarata, la **coda DENTRO la busta** la prende `position() != body.len()` — togliendo l'uno l'altro sopravvive (le sonde **W** del livello 2). ⛔ **Nessun byte congelato è nato**, e §6.10.3 lo vieta: questo canale prende la meccanica di `record.rs` e non la sua disciplina. Vincolo **15** della §11 del [compendio](COMPENDIO.md) |
| ~~le **righe 1–4 di §6.10.5** — i casi negativi della porta `process`~~ ✅ **CHIUSA il 2026-08-21, col Task 11** | Quattro casi `compile_fail` — `talking_without_the_handle.rs`, `instructing_after_the_kill.rs`, `reading_without_a_receipt.rs`, `reading_twice_from_one_receipt.rs` — con gli oracoli letti uno per uno e non rigenerati: `E0599` (riga 1, senza la maniglia), `E0382` (riga 2, dopo l'uccisione), `E0061` (riga 3, senza ricevuta), `E0382` (riga 4, due letture). Le contro-sonde stanno in `crates/kernel/tests/worker_tokens.rs` (`cargo test --locked -p kernel --test worker_tokens`) e ottengono un `Grant` vero da `Arbiter::admit` — MAI un costruttore di test, scartato per la ragione della riga del blocco **B**. ⚠️ **`E0061` prova solo l'ARITÀ, non l'autenticità della ricevuta:** `SingleReceipt::new` è `pub` e raggiungibile da fuori la crate, quindi `worker.read_one(SingleReceipt::new(7))` compila; il limite è dichiarato accanto a `SingleReceipt::new` in `crates/kernel/src/ports/process.rs`. ⚠️ Che istruire **prima** della `kill` compili lo tiene il fatto che il file compila, non una mutazione; e `one_grant_starts_one_worker` non ha un'asserzione a runtime sul proprio soggetto, come dichiara il suo commento. `crates/kernel/tests/ports_are_implementable.rs` esercita le firme **in entrambe le direzioni** con una finta costruita dal test, `ScriptedWorker`. ⚖️ **Un quinto caso** — un secondo `start` con lo stesso `Grant`, `E0382` — resta **registrato e non preso**: se pretenda una riga di catalogo lo decide la spec |
| solo `secrets` raggiunge il **portachiavi** | nessuno script lo verifica oggi: `gate-deps.sh` guarda i grafi di `kernel` e `simulator`, non quelli di `platform` e `secrets` |
| un solo **punto di uscita verso la rete** | la lista delle crate autorizzate è **vuota**, e una lista vuota passa sempre. Il catalogo lo dichiara già: è l'unica voce provata in una direzione sola, e si completa nel sotto-progetto che accende la rete |

## ⛔ LE VOCI APERTE DEL TRAGUARDO 6, IN UNA TABELLA SOLA (2026-09-02)

⛔ **Raccolte alla chiusura del traguardo, per la condizione 11 della §7.2 del suo disegno.**

⚠️ **Sta in fondo al file e non accanto alla tabella del Traguardo 5, per misura:** il blocco di comandi di quella
sezione è ancorato a **due intestazioni** — la propria e quella di *«Cosa la porta NON controlla»* — quindi una sezione
infilata fra le due farebbe restituire al suo `awk` anche **queste** righe. Provato nelle due direzioni su una copia
fuori dal repository. ⚠️ **[C-S5-6]**

⛔ **Come è stata costruita: non è un censimento dichiarato completo, è ciò che questi comandi hanno restituito**, letti
riga per riga e senza troncarli (gotcha **#70**), sul
[piano del Traguardo 6](superpowers/plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md) guardato a
**`0aaa080`**.

```
$ grep -cE '^\| \*\*`?E[0-9]+' <piano>
    ⛔ QUANTE SIANO NON È SCRITTO QUI: lo dice il comando, sul piano che si sta guardando.

$ grep -nE '^\| \*\*`?E[0-9]+' <piano> | grep -E "<il filtro>"
    dove <il filtro> è quello scritto sotto il SECONDO $ del blocco di comandi
    della sezione del Traguardo 5, copiato verbatim — sta su due righe e si ricongiunge — e senza -i.
    ⛔ IL FILTRO NON BASTA, ed è il gotcha #70 in ENTRAMBE le forme, come al Traguardo 5.
      ① NE MANCA di aperte: E15, E48 ed E108 rendono zero contro quel filtro
        lanciato SENZA -i, e si trovano allargandolo a
        "registrat|non pres|non pinzat|per il proprietario|del proprietario|resta apert|è apert|e sua|APERTA".
      ⚠️ La maiuscola è PORTANTE, misurato: con -i lo stesso filtro ne cattura
        due delle tre.
      ② NE RESTITUISCE di chiuse.

$ grep -nE "riga di catalogo" docs/porta-di-qualita.md | grep -iE "non ha|non hanno|nessuna riga|sarebbe una riga"
    → i siti in cui un compito dichiara che il proprio caso non ha una riga di catalogo
      propria. Quelli del Traguardo 6 sono la riga 22 di questa tabella, che li indicizza
      invece di ricopiarli.
    ⚠️ Come al Traguardo 5, il grep rende CANDIDATE e non case — righe di altri traguardi
      e falsi positivi comprese: ogni riga si legge intera.
```

⛔ **E ciò che nessun `grep` ha reso, trovato LEGGENDO:** le voci che il disegno apre per il proprietario ancora aperte
(righe **1**–**7**); le voci che vivono in **questo** file e non nell'errata (righe **23**–**26**); l'altra metà di
**ADR-0005**, il cui **innesco** vive nel doc di `GrantRequest` in `crates/kernel/src/wire/ipc.rs` (riga **27**); la voce
che la chiusura ha misurato (riga **28**).

⚠️ **Una voce che questa tabella NON indicizza:** due citazioni di riga sbagliate restano nel rapporto del compito 9,
che vive in `.superpowers/` — **git lo ignora**, quindi è fuori da ciò che si consegna, e sono dichiarate lì. **Vale una
riga di esclusione, non una riga di tabella**.

⛔ **La colonna «Chi la chiude» è la ragione per cui la tabella esiste, e non dice sempre «il proprietario»:** quelle il
cui chiusore **non** è il proprietario le nomina il comando in fondo alla sezione.

| # | Voce | Che cosa resta aperto | Dove è dichiarata | Chi la chiude |
|---|---|---|---|---|
| 1 | la **riga C in §0.4** per il timbro di build | §0.3 dice che *«un pezzo scaglionato senza una riga C esplicita è un errore di questa sezione»*, e il rinvio del timbro (§3.4 del disegno) non ne ha una | disegno del Traguardo 6, voce **1** del proprietario | il **proprietario**: §0.4 è spec, vincolo globale 7 |
| 2 | `V10`, `V14`, `Q10` — righe **sbagliate** o **notazione in avanti** | portavano ✅ senza avere un controllo; i tre controlli ora esistono, ma **quale** delle due cose fosse la §8 non lo dice | disegno, voce **2** | il **proprietario**: §8 è spec |
| 3 | la lettura di *«mantiene»* in ADR-0019 | **espone** o **cachea**: il Traguardo 6 ha letto *«espone»* e ha costruito il degrado come **derivato**, dichiarando la divergenza invece di appianarla | disegno voce **3**, e il riquadro del compito 8 di questo file | il **proprietario**: tocca un ADR `Accepted` |
| 4 | se `check-docs.sh` possa confrontare un ✅ con l'**esistenza** del controllo | oggi verifica l'innesco di `parziale`/`rimandato` e non questo — la deduzione è dichiarata **non misurata** dal disegno stesso | disegno, voce **4** | il **proprietario**: sarebbe una riga di catalogo nuova |
| 5 | se il buco del **timbro** (§6.4 del disegno) debba avere una riga C in §0.4 | è la gemella della riga 1, su un buco diverso: là il rinvio, qui la conseguenza su I4 | disegno, voce **6** | il **proprietario**: §0.4 è spec |
| 6 | la **revoca** core → gui | ADR-0033 la nomina, nessun innesco scritto la pretende oggi, e fino ad allora una concessione discrezionale è prelazionabile **nei libri** senza che la gui lo senta | disegno voce **7**, e il doc di modulo di `crates/kernel/src/wire/ipc.rs`, che ne porta l'innesco | il **proprietario**: è un allargamento di perimetro |
| 7 | l'**inquadratura condivisa** fra i due canali privati | economia, o erosione della lettura di *«singolo»* di ADR-0035 | disegno, voce **8** | il **proprietario**: tocca la lettura di un ADR `Accepted`, come la riga 3 |
| 8 | **E5** | il `#[must_use]` su `Started` e su `Killed` non è tenuto da niente — **mutante vivo** dal giorno in cui nasce — e non è decorativo: un avvio scartato **lascia cadere una concessione**, che è il difetto per cui `R6` esiste | errata del piano | il **proprietario**: pinzarlo è un caso `compile_fail`, cioè una riga di catalogo nuova |
| 9 | **E11** | le due sonde del compito 1 provano che torna **una** concessione, non che torna **quella**: per `release` due arbitri con la stessa identità sono un arbitro solo | errata | ⛔ **la suite di conformità di §6.10**, che nasce col canale worker vero — **non** il proprietario |
| 10 | **E12** | i **due lati della grazia** di una revoca non sono tenuti da nessuna sonda: nessun banco rilascia una concessione sotto revoca | errata | ⛔ **il compito che darà alla revoca un chiamante** — **non** il proprietario |
| 11 | **E13** | il doc di `collect_expired` dice *«both comparisons are `>`»* dove il confronto è scritto `<=`. ⛔ **La proprietà è vera e misurata**, e l'attrito è che chi cerca col `grep` un `>` non lo trova | errata, e il doc in `crates/kernel/src/arbiter/mod.rs` | ⛔ **chi tocca quel doc**, sapendo che **non** sta correggendo un errore — **non** il proprietario |
| 12 | **E15** | due domande di forma, e la prima è che `Arbiter::id` è un **secondo idioma** per leggere un parametro consegnato: `total_vram` si legge da `Parameters`, l'identità è sollevata in un campo | errata | il **proprietario** |
| 13 | **E37** | `G5` e `G-5` vivono nello **stesso** documento — le mutazioni del compito 4 e il finding dell'audit del 2026-08-11 — e li distingue un trattino | errata | il **proprietario**: rinominare tocca un verbale |
| 14 | **E38** | il doc di `GrantRequest` è molto più denso di prosa del gemello, e il candidato a essere tolto è il blocco sui **derive** | errata | il **proprietario**: si accorcia **da sé** il giorno in cui risponde su `Eq` e `Clone` |
| 15 | **E40** | `crates/kernel/tests/worker_wire.rs` dichiara di essere **la casa** di certe cifre, e la casa non è una | errata | il **proprietario** |
| 16 | **E41** | la riga dei **fine-riga** di [`../CLAUDE.md`](../CLAUDE.md) nomina `sed -i`, e lo strumento che ha fatto scattare la trappola **due volte** è `cargo fmt`, che non si scrive: si lancia ⚠️ **[C-S5-5]** | errata | il **proprietario**: tocca il contratto d'ingresso |
| 17 | **E48** | un fatto d'**ambiente** — `core.autocrlf` — scritto senza dire **di quale** ambiente, e le macchine sono due: `git config --get core.autocrlf` risponde `true` su una e `false` sull'altra. La voce è la lacuna, non il valore | errata | il **proprietario**: tocca il contratto d'ingresso, come la riga 16 |
| 18 | **E62** | *«`dispatch` consuma il gettone, quindi una risoluzione dispaccia una volta sola»* non è tenuto da niente. ⚠️ **Il tipo la regge già** — `Conforming` non deriva `Copy` né `Clone` — a mancare è chi la dica | errata | il **proprietario**: sarebbe una riga di catalogo nuova, come il quinto caso di `Grant` |
| 19 | **E96** | un rimando `E<n>` nel sorgente **non è un riferimento**: il numero è unico dentro **un** piano, e i piani lo riusano dall'inizio. ⚠️ A difendere il rimando è la **prosa** accanto, che nessun controllo tiene | errata | il **proprietario**: le vie sono almeno due e cambiano una **convenzione** |
| 20 | **E108** | il doc di `Detail`: una specie **sconosciuta decodifica a `None` in silenzio**, e la coppia tenuta al livello 1 dal 2026-09-01 non chiude quella strada — lì si decodificano byte **già scritti** | errata, e il doc in `crates/kernel/src/record.rs` | il **proprietario**, e **non pinzata**: una sonda sul silenzio di oggi sarebbe un voto contro il cambiarlo (gotcha **#73**) |
| 21 | **E53**, richiamo ⑥ | se la **domanda di classe** — *«ciascun rimedio ha chiuso la CLASSE o l'occorrenza?»* — diventi una condizione **scritta** di ogni ondata di revisione. ⚠️ È aperta dentro una voce chiusa, e per questo il filtro la restituisce come chiusa | errata, voce `E53` | il **proprietario** |
| 22 | i **casi `compile_fail` senza riga di catalogo** | il 35°, quello di `RoutingDetail` e i **due** del permesso: ciascuno dichiara accanto a sé di non avere una riga propria. ⚠️ **Questa riga li indicizza e non li copia** (gotcha **#68**) | i riquadri di questo file, ai siti che il terzo `grep` qui sopra restituisce | il **proprietario**: §7.4 è spec, vincolo globale 7 |
| 23 | il doc di **modulo** di `record.rs` | dice *«with and without `#[cbor(array)]` on the two types below»*, e i tipi che portano quell'attributo sono più di due — quanti lo dice `grep -c '^#\[cbor(array)\]' crates/kernel/src/record.rs`. ⛔ **Non toccata**: la frase descrive una **misura passata**, e prezzarla come difetto invece che come verbale è una scelta | riquadro della nona passata di questo file ⚠️ **[C-S4-6]** | il **proprietario** |
| 24 | `reconcile::Resolution` | cresce a `exit 0` e **nessuno la decide oggi**, né in `src/` né nei banchi: è l'unico membro rimasto della classe che il nono giro ha censito | tabella della domanda di classe in questo file ⚠️ **[C-S4-6]** | ⛔ **il primo consumatore**, che la decide con un `match` — **non** il proprietario |
| 25 | la riga *«zero avvisi»* del cancello | `cargo build --locked --workspace` **non compila i banchi**, quindi un avviso di banco non la fa rossa — misurato sul nome con maiuscole di `gateway_decisor.rs`. Se debba passare a `--all-targets` tocca il cancello | riquadro del compito 7 di questo file ⚠️ **[C-S4-6]** | il **proprietario**: vincolo globale 7 |
| 26 | la metà **temporale** di *«concessione valida»* | `Process::start` non prende `now` e non interroga l'arbitro, `GrantId` è privato, e nessuna API risponde *«questa concessione è ancora nei libri?»*: la campagna del compito 9 la **conta e la dichiara** invece di asserirla | riquadro del compito 9 di questo file | il **proprietario** (gotcha **#73**) |
| 27 | l'altra metà di **ADR-0005** | `compute_class` e `preemption` raggiungono l'arbitro dal **medesimo pari** del nome che il tipo rifiuta, e l'arbitro **obbedisce** a entrambi: uno sceglie la corsia e apre la guardia di `ask_back`, l'altro decide se una concessione sia richiamabile. ⛔ **Nessuno dei due è controllato contro niente**, mentre `reserved_vram` deve superare il tetto in `admit`. ⚠️ Non è sfruttabile oggi: **il consumatore non esiste** | doc di `GrantRequest` in `crates/kernel/src/wire/ipc.rs`, che ne porta l'innesco per esteso | ⛔ **il compito che per primo decodificherà byte in un `ResourceProfile`** — **non** il proprietario |
| 28 | il **CR isolato** del piano del Traguardo 6 | un carattere `CR` nudo sta dentro un code span della voce `E136`, dove il testo intendeva i **due caratteri** che scrivono l'escape. ⛔ **La conseguenza non è tipografica:** git classifica allora l'intero file come **non-testo** — `git ls-files --eol` dà `i/-text w/-text` — quindi i suoi CRLF stanno **nell'indice** e nessuna normalizzazione li tocca; misurato nelle due direzioni su una copia fuori dal repository. ⛔ **Toglierlo riscriverebbe in LF ogni riga del file al primo `git add`**, che è la trappola dei fine-riga di [`../CLAUDE.md`](../CLAUDE.md) alla scala dell'intero piano. Comparso con `307fa18` | voce `E179` dell'errata del piano | ⛔ **nessuno oggi, e la ragione è misurata:** il rimedio costa più del difetto. Lo chiude chi riscriverà quel file per un'altra ragione |
| 29 | le voci **ereditate dal Traguardo 5** | ⚠️ **Questa riga è un rimando e non una seconda copia** (gotcha **#68**): la tabella unica del Traguardo 5, qui sopra, resta la loro casa. Le righe **chiuse** portano `✅ CHIUSA` nella **terza** colonna, ciascuna nella propria riga. ⛔ **Quali restino aperte lo dice il comando** qui sotto — e le chiuse lo stesso comando con `~` al posto di `!~` — non questa cella | la tabella unica del Traguardo 5, in questo file | ⚠️ **varia per riga**, e lo dice la colonna omonima di quella tabella |

⛔ **Le righe il cui chiusore NON è il proprietario, e il comando che le nomina invece di contarle** — sul precedente
della tabella del Traguardo 5, e ancorato alle **due intestazioni**:

```
awk -F'|' '/^## .*LE VOCI APERTE DEL TRAGUARDO 6/{s=1} s&&/^\| +[0-9]+ \| +le voci \*\*ereditate/{s=0} s&&/^\| [0-9]+ \|/&&$(NF-1)!~/il \*\*proprietario\*\*/{print $2": "$3}' docs/porta-di-qualita.md
```

⛔ **NON è il filtro del Traguardo 5, e la differenza è misurata:** là il discriminante è la parola `proprietario` nuda,
e qui non funziona — le celle di questa tabella scrivono *«**non** il proprietario»* **per esteso**, che è la notizia
che la colonna porta. Il discriminante è la forma **in grassetto**, `il **proprietario**`, che compare solo dove il
chiusore lo è davvero. ✅ **Provato nelle DUE direzioni** (gotcha **#24**): la forma complementare — `~` al posto di
`!~` — rende tutte le altre righe e nessuna di queste; e lo stesso discriminante, puntato sulla tabella del Traguardo 5,
rende **lo stesso insieme** del filtro che quella sezione porta da sé. ⚠️ **Si ferma alla riga 29 e non all'intestazione
successiva**, perché questa sezione è l'ultima del file ⚠️ **[C-S5-6]**: la riga 29 è un **rimando** e il suo chiusore
*«varia per riga»* non è una risposta, quindi contarla fra le une o fra le altre mentirebbe in entrambi i modi.

⛔ **E le voci del Traguardo 5 che restano aperte, col comando e non con un elenco** — un elenco di nomi invecchierebbe
alla prima che si chiude:

```
awk -F'|' '/^## .*LE VOCI APERTE DEL TRAGUARDO 5/{s=1} s&&/^## Cosa la porta NON controlla/{s=0} s&&/^\| [0-9]+ \|/&&$4!~/CHIUSA/{print $2": "$3}' docs/porta-di-qualita.md
```

⚠️ **La colonna che discrimina è la TERZA e non la quinta**, perché è lì che ogni riga chiusa porta `✅ CHIUSA`.

## ⛔ LA SONDA S3 DEL RICONOSCIMENTO GESTI — 2026-09-04

Due sonde in `crates/kernel/tests/arbiter_admission.rs`, dal compito 6 del
[piano del riconoscimento gesti](superpowers/plans/2026-09-03-riconoscimento-gesti.md), per la
§4.2 del [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md):
`a_zero_reservation_is_granted_even_on_a_full_machine` e
`on_the_same_full_machine_a_real_reservation_is_queued_and_not_granted`. ⚠️ **Non hanno una
riga di catalogo**: la §7.4 è spec (vincolo globale 7), quindi la sonda si **registra** e non si
prende — stesso trattamento di PL-1 e di K-1/B-1, stessa ragione (gotcha #36). Se pretenda una
riga propria lo decide il proprietario: voce 6 delle *«voci che questo disegno apre»* del disegno.
✅ Ciascuna è stata vista rossa sotto una mutazione di `admit`; la tabella delle mutazioni sta in
[archivio](archivio/porta-di-qualita-storico.md), in questa stessa sezione.

## ⛔ IL PASSO WEB E LE SONDE DELLA PARTE 2 — 2026-09-22

Il passo web del cancello e i tre siti di scansione degli avvisi, dai compiti 15 e 16 del
[piano della parte 2 del sotto-progetto 2](superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md), per la §8 del [disegno del 2](superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md).
`scripts/gate-gui.sh` entra in `scripts/gate.sh` con la riga `run "gui: fake core and SPA"` (compito 15). Il compito 16
aggiunge i tre siti: `run "dependency advisories"` con `cargo audit` in `gate.sh`, fra *«allow-list on the two graphs»* e
*«attributes of the constrained crates»*; `cargo audit --file fake-core/Cargo.lock` dentro `gui/`, dopo il lint (D83, E212);
`npm audit`, ultima riga di `gate-gui.sh`, dentro `gui/` (D71). L'ordine lo rileggono due comandi:
`grep -n 'run "' scripts/gate.sh` e `grep -n 'gui:' scripts/gate-gui.sh` — i sotto-passi, nell'ordine
`fake core`, `install`, `build`, `probes`, `lint`, `fake core advisories`, `advisories`. ⚠️ **Non hanno una riga di catalogo**:
la §7.4 è spec (il «vincolo globale 7» del piano del Traguardo 5, e del 6 che lo cita, idioma di questo file: una riga di catalogo nuova è decisione del proprietario — E235 ed E236 del piano della parte 2, dove il 7 è un altro), quindi le sonde si **registrano** e non si prendono — stesso trattamento di PL-1, di K-1/B-1
e di S3, stessa ragione (gotcha #36). Se pretendano una riga propria lo decide il proprietario.
✅ Il lint, il passo web dai due mondi, `cargo audit` sui due lockfile e `npm audit` sono provati nelle due direzioni, e
l'avviso «non mantenuto» di `bincode` è un **warning** che `cargo audit --deny unmaintained` rende rosso; la tabella
delle mutazioni, con le uscite del giorno, sta in [archivio](archivio/porta-di-qualita-storico.md), in questa stessa
sezione.

| Che cosa la porta NON controlla, di questa parte | Perché |
|---|---|
| la metà **Windows** della CI — che `windows-latest` onori `rust-toolchain.toml` e che Git Bash vi lanci `gate.sh` | la dice solo la corsa su GitHub, e nessun comando locale (P-111): il criterio di chiusura del compito 16 manda a guardarla |
| la prova **capo a capo** — la SPA nel guscio col core finto | *«fuori dal cancello di oggi, dichiarato»*, nella §8 del disegno del 2: il guscio non è di questo piano |
| la **validità dei JSON** sotto `gui/src` | il lint li legge con `jsonc-eslint-parser` e nessuna regola li giudica — un `{ "a": 1, }` in `panels/views/` lascia `EXIT=0` (P-101); le tre viste le prova la sonda della cornice del compito 13 |

## Le contraddizioni registrate, e non risolte

⛔ **Registrate il 2026-09-24, comprimendo, e non corrette:** il proprietario ha deciso che si comprime prima e si
corregge dopo, in una sessione loro. Ogni riga è un'affermazione **viva** di questo file che il codice di oggi, o un'altra
parte del repository, smentisce; il segno `⚠️ **[C-…]**` sta accanto all'affermazione. Le contraddizioni che stavano
nella cronaca sono andate in archivio con lei, e lì restano vere come verbale del loro giorno.

| Segno | Che cosa afferma questo file | Che cosa dice la prova |
|---|---|---|
| **C-S0-1** | la tabella dei passi di `gate.sh` ne conta **sette** | `grep -n '^run ' scripts/gate.sh` ne rende **nove**: mancano `dependency advisories` e `gui: fake core and SPA`, che la sezione del passo web nomina |
| **C-S0-2** | la conformità prova **V6 solo su un archivio vuoto**, voce aperta del proprietario | il gotcha **#63** di [`HANDOFF.md`](HANDOFF.md) la dà **CHIUSA il 2026-08-17**, con un passante nei blocchi 1, 5 e 8a — la sezione «T-1 e T-2» di questo file |
| **C-S1-1** | voce `E21`: `GrantId` riparte da zero per ogni `Arbiter`, e dare un'identità all'arbitro è del proprietario | `ArbiterId` esiste, consegnato in `Parameters`, e `release` confronta `Grant::issuer` prima dei libri (`crates/kernel/src/arbiter/mod.rs`); la riga **13** della tabella del Traguardo 5 dà `E21` ✅ **CHIUSA** (`c4cf942`) |
| **C-S1-2** | voce `E30`: resta da disegnare il tipo esatto della risposta di `release`, e le forme scartate stanno accanto a `ReleaseError` | `pub enum Released { Now(Mib), AlreadyCollected }` e `release(…) -> Result<Released, ReleaseError>` in `crates/kernel/src/arbiter/mod.rs`, dove il doc dice che l'argomento **non** è ricopiato lì; righe **14** e **26** della tabella del Traguardo 5 ✅ **CHIUSE** |
| **C-S1-3** | di `release`, *«nessun consumatore di produzione esiste»* | `crates/kernel/src/client.rs` chiama `arbiter.release(grant, now)` |
| **C-S1-4** | la cella `Q8 · §5.2.1`: *«la regola è provata sull'argomento che la decisione prende davvero»* | la sezione del Task 5 dice che la chiamata **non partecipa** all'`E0609`, misurato cancellandola; il commento di `crates/kernel/tests/compile_fail/admission_reads_cold_start.rs` porta **entrambe** le frasi |
| **C-S1-5** | cita il gotcha **#67** per *«un costruttore `pub(crate)` apre una strada»* | in `HANDOFF.md` il #67 è *«Una giustificazione scritta su un ELENCO di nomi si legge come verificata su tutti»*; `riferimenti.md` e il disegno del Traguardo 5 gli attribuiscono un'altra frase ancora |
| **C-S1-9** | l'àncora *«con `expires_at > now` la finestra è semiaperta»* | `collect_expired` in `crates/kernel/src/arbiter/mod.rs` scrive `if held.expires_at <= now`: stessa semantica, l'àncora alla lettera non esiste più |
| **C-S1-10** | l'oracolo di `grant_has_no_constructor.rs` cita la nota *«private field `id` that was not provided»* | `crates/kernel/tests/compile_fail/grant_has_no_constructor.stderr` dice *«private fields `id` and `issuer` that were not provided»* |
| **C-S2-1** | `askable` è una chiusura e non un metodo perché, installato come `Held::askable_by`, il build stampa tre avvisi | il doc di `ask_back` in `crates/kernel/src/arbiter/mod.rs`: la misura è scaduta col chiamante di produzione del Task 8 (`E91`), il build stampa **zero** avvisi, e a tenere la chiusura è che cattura `below` (`E99`) |
| **C-S2-2** | `ReleaseError::UnknownGrant` ha **tre** cause, e a grazia scaduta `release` risponde `Err(UnknownGrant)` | `UnknownGrant` significa ora **una** cosa, e una concessione propria già riscossa risponde `Ok(Released::AlreadyCollected)` (`crates/kernel/src/arbiter/mod.rs`); riga **14** del Traguardo 5, `E30` ✅ **CHIUSA** |
| **C-S2-4** | *«una regola scritta in un commento e tenuta da niente è un'intenzione»*, citata come gotcha **#42** | in `HANDOFF.md` il #42 è il caso `compile_fail` che scatta come `mismatch` e che una rigenerazione in blocco disarma |
| **C-S2-6** | `Preemption::Never` è esercitata in **due** casi `compile_fail` | `grep -l 'Preemption::Never' crates/kernel/tests/compile_fail/*.rs` ne rende di più |
| **C-S2-7** | il costo dell'opzione ② di `E94` comprende i chiamanti di `Parameters::new` che `E18` ha contato | `grep -rn "Parameters::new(" crates --include=*.rs` ne rende molti di più: la cifra del costo è datata |
| **C-S2-8** | `set_policy` **non ha nessun chiamante di produzione**, e il grafo di produzione non transita mai | il modulo `kernel::serving` chiama `arbiter.set_policy` dentro l'attività `serve`, che il daemon lancia; il grafo **legge** la policy dal giornale con `arbiter::policy_now` in `crates/daemon/src/main.rs` |
| **C-S2-9** | ciò che compra `policy()` restano i **banchi** | lettore di produzione: il modulo `kernel::serving`, `match self.arbiter.policy()` |
| **C-S2-10** | la sonda `the_production_graph_assembles_and_the_executor_runs_to_completion`, e l'elenco delle sonde del `daemon` | la sonda si chiama oggi `the_production_graph_assembles_and_the_serving_activity_takes_the_turns` (`9ba48e2`), e `crates/daemon/src/main.rs` porta sonde che l'elenco non nomina |
| **C-S2-11** | i rami d'errore di `main` sono **tre**, e senza attività il corpo dell'esecutore non gira mai | `enum StartupError` in `crates/daemon/src/main.rs` ha sei varianti, `EXECUTOR_TURN_LIMIT` vale `u64::MAX` con un'attività lanciata, e il commento in `main` dice che `run()` non può rispondere `Ok(())` |
| **C-S3-4** | le righe che il passo delle campagne DST stampa | `scripts/gate.sh` lancia oggi **sei** campagne, quindi le righe sono di più |
| **C-S3-6** | `M9` è un **mutante vivo** dichiarato e non pinzato, perché la sonda congelerebbe la scelta che `E30` mette davanti al proprietario | `E30` è chiusa (riga **14** del Traguardo 5); il doc di `crates/kernel/src/arbiter/mod.rs` dice spesa la pretesa di `E30`, e `a_grant_of_this_arbiter_released_after_its_window_is_not_an_error` in `crates/kernel/tests/arbiter_admission.rs` asserisce `Ok(Released::AlreadyCollected)`. ⚠️ **Dedotto, non misurato:** sotto `M9` quel rilascio risponderebbe `Now(…)`, quindi `M9` oggi morirebbe |
| **C-S4-1** | le **dieci** sonde permanenti dell'audit, in tre case | la tabella consolidata ne nomina di più; il «dieci» torna solo contando **una sonda per voce** |
| **C-S4-3** | P-1 elenca ciò che resta aperto di A3 e dice chiusa *«la strada che si prende senza accorgersene»* | il doc di `crates/kernel/src/boundary.rs` (richiamo di AUD-050) dice che l'elenco mancava la strada più larga, `RecordV1` coi campi `pub`, chiusa il 2026-09-01 nella sezione «`AUD-050` chiuso a LIVELLO 1»; P-1 non vi rimanda |
| **C-S4-4** | *«Tre moduli di test vivono in `src/`»* | sono **quattro**: anche `crates/platform/src/ipc.rs`, dal 2026-09-17 — `git grep -n -F "#[cfg(test)]" -- 'crates/*/src/**'` |
| **C-S4-5** | i due casi `compile_fail` di `PermissionDetail`, nominati solo in forma abbreviata | il ciclo degli orfani di [`riferimenti.md`](riferimenti.md), rilanciato, li dà **orfani** tutti e due: il nome per esteso non compare in questo file |
| **C-S4-6** | le righe **23**, **24** e **25** della tabella del Traguardo 6 nominano la nona passata, la tabella della domanda di classe e il riquadro del compito 7 | le voci stanno in «La passata INDIPENDENTE sul perimetro del compito 5», in «La crescita, enum per enum» e in «Il rilievo del coordinatore, trovato rimediando» |
| **C-S5-1** | *«`clippy` gira come igiene del codice»* | nessuno script lo esegue: è la voce **X-2** dell'[audit del 2026-08-27](audit-2026-08-27.md), aperta |
| **C-S5-2** | le voci aperte del Traguardo 3 vivono nella tabella *«Cosa il Traguardo 3 lascia aperto»* della §6 del compendio | quella tabella sta dal 2026-09-09 in [`archivio/stato-storico.md`](archivio/stato-storico.md) |
| **C-S5-3** | riga **34** della tabella del Traguardo 5, `E94`, aperta | la sezione «`E94` — la TERZA bocca della classe di AUD-050: `RoutingDetail`, chiusa il 2026-09-01» la dice chiusa, e il comando delle voci aperte del Traguardo 5 la elenca |
| **C-S5-4** | riga **33**, C-1: il residuo **X-3** resta aperto e del proprietario | l'[audit del 2026-08-27](audit-2026-08-27.md) dà **X-3 CHIUSA il 2026-09-22**, dal compito 16 del piano della parte 2 |
| **C-S5-5** | riga **16** della tabella del Traguardo 6, `E41`: la riga dei fine-riga di `CLAUDE.md` nomina `sed -i` | dal 2026-09-23 la riga di `CLAUDE.md` dice *«uno strumento che tocca file»*: forse la voce è chiusa nei fatti. **Da verificare**, non certa |
| **C-S5-6** | la tabella del Traguardo 6 sta *«in fondo al file»*, e il suo comando si ferma alla riga 29 perché *«questa sezione è l'ultima del file»* | dopo di lei vengono la sonda S3 (2026-09-04), il passo web (2026-09-22) e queste due sezioni; la ragione della riga 29 — un rimando, che non ha un chiusore — resta vera |

## Dove è finita ogni sezione di prima

📌 **Le intestazioni di prima restano qui, col titolo alla lettera, tranne sette.** In ogni sezione rimasta la cronaca —
tabelle di mutazione, conteggi, richiami datati, passate di revisione — sta in
[`archivio/porta-di-qualita-storico.md`](archivio/porta-di-qualita-storico.md) **sotto lo stesso titolo**. Una sezione
rimasta con la sola intestazione — «Decisione 7», la NONA e la DECIMA passata — è un genitore che tiene la gerarchia dei
figli vivi.

Le sette che stanno **solo in archivio**, tutte verbali di passate di revisione o di mutazione:

- «`C7b` — sei mutazioni, e due esistono perché la non-vacuità dichiarata non era quella vera»
- «La SECONDA passata di revisione del compito 5 — quattro mutanti vivi, e una radice sola»
- «La TERZA passata — la prima da un sotto-agente, e il gotcha #98 riproduce lo stesso giorno»
- «La domanda di classe, rimedio per rimedio»
- «Le mutazioni di prodotto del revisore — tutte rosse»
- «Il censimento della crescita — tutti gli enum di `crates/*/src`»
- «I tre rilievi, e la domanda di classe»

📌 **I nomi che il codice e gli altri documenti usano per puntare qui**, e dove si risolvono. Un commento nel codice non si
tocca: lo risolve questa tabella.

| Il nome | Chi lo usa | Dove si risolve |
|---|---|---|
| *«section "P-2"»*, e *«the verbal and both measurements»* della ragione falsa di `start` | `crates/kernel/tests/worker_tokens.rs`, `crates/kernel/src/ports/process.rs` | **archivio**, «P-2 — la ragione dello scaglionamento era falsa, e lo era dal Traguardo 2 (2026-08-21)»; il titolo resta anche qui |
| *«row 15 of the mutation campaign»* | `crates/daemon/src/main.rs` | qui, «Le sonde del `daemon`, per nome», che nomina la riga 15; la tabella coi conteggi in **archivio**, sotto lo stesso titolo |
| *«the task 8 section»*, le cifre della misura di `E100` | `crates/kernel/src/arbiter/mod.rs` | qui, «Le due policy VRAM — Traguardo 5, Task 8, e `V3` si copre a metà» |
| *«the task 12 section»*, la riscrittura dello scenario (`E144`) | `crates/simulator/tests/arbiter_campaign.rs` | qui, «La campagna DST dell'arbitro — Traguardo 5, Task 12»; le cifre di prima e di dopo, e la riga `M1` della guardia vista rossa, in **archivio** sotto lo stesso titolo |
| *«M10 … dies with IT»*, la passata di `process` | `crates/kernel/tests/ports_are_implementable.rs` | **archivio**, «Le passate di mutazione, e le righe che contano» |
| le tabelle complete delle passate di `process`, `ipc` e `memory_journal`, e la corrispondenza promessa ↔ bugiardo «presa a otto» | [`riferimenti.md`](riferimenti.md) | **archivio**, «Le passate di mutazione, e le righe che contano» |
| *«OPEN ENTRY»*, la voce aperta **1** e la **3** (*«where who closes it lives»*), e la **2** | `crates/kernel/tests/journal_contract.rs`, `crates/kernel/src/ports/journal.rs`, `crates/platform/src/journal.rs`, `crates/simulator/src/journal.rs`, la §6 del [compendio](COMPENDIO.md) | qui, «T-1 e T-2 — le tre promesse provate solo dove ogni guardia passa (2026-08-17)» |
| *«the open entry … that promise 7b closes»* | `crates/kernel/tests/journal_contract.rs` | **archivio**, «T-1 e T-2 — …»: la questione della promessa 7, chiusa |
| la misura dei siti di `RecordV1 {`, *«with its command … in one»* | `crates/kernel/src/boundary.rs` | qui, «`AUD-050` chiuso a LIVELLO 1 — la cura alla radice, scelta dal proprietario (2026-09-01)», il comando; le cifre in **archivio** sotto lo stesso titolo |
| la cura delle date `2026-09-01` (`E66`) | `crates/kernel/tests/record_shape.rs` | **archivio**, le passate di revisione del compito 5 |
| *«Probe N5»*, *«probe B3»*, *«mutation W9»*, le sonde **N6** e **N7** | `scripts/gate-deps.sh`, `scripts/gate-no-os.sh`, `crates/kernel/tests/worker_wire.rs`, `crates/kernel/tests/ipc_wire.rs`, [`riferimenti.md`](riferimenti.md) | qui, «Le sonde, per nome» |
| *«Mutation G5»*, *«mutation G4»* | `crates/kernel/tests/ipc_wire.rs` | qui, «Lo schema del canale `ipc` — Traguardo 6, Compito 4, e NESSUNA riga di catalogo si muove» |
| *«il settimo passo»* | `scripts/gate.sh` | qui, «⛔ Il settimo passo del cancello, che non è un settimo controllo» |
| *«milestone 5 task 9»*, *«milestone 5 task 10»* | `crates/kernel/src/reconcile.rs`, `crates/kernel/src/arbiter/mod.rs`, `crates/daemon/src/main.rs`, `crates/platform/src/journal.rs`, `crates/kernel/tests/arbiter_admission.rs` | qui, le sezioni del Task 9 e del Task 10 |
| la storia del numeratore del blocco C | [`HANDOFF.md`](HANDOFF.md) | **archivio**, «Livello 1 — il compilatore» |
| le cinque ragioni della decisione su C-1 | [`riferimenti.md`](riferimenti.md) | qui, riga **33** della tabella del Traguardo 5 |
