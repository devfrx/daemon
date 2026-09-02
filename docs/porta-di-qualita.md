# La porta di qualità — dove vive ogni controllo

> Questo file non decide niente. Il catalogo è la
> [§7.4 della spec](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md); qui c'è
> soltanto la mappa fra ogni riga del catalogo e il file che la implementa.
>
> ⛔ **Un controllo nuovo entra prima nel catalogo, poi qui.** L'ordine inverso è il
> gotcha #36: una sezione decide un meccanismo, lo scrive nella propria tabella, e il
> catalogo resta indietro — è già successo due volte.

⛔ **RICHIAMO DEL 2026-08-11 — l'audit ha trovato TRE controlli che non controllavano ciò che
dichiaravano, e nessuno di essi è una riga di catalogo nuova: sono riparazioni di righe
esistenti.** Il catalogo resta a **sei controlli** e le sue righe non cambiano; cambia il fatto
che ora fanno ciò che dicevano già di fare. Tutti e tre provati **in due direzioni**, coi codici
d'uscita veri, e il cancello è rimasto `GATE GREEN` con **171 test, 0 falliti** — identico alla
baseline presa prima di toccare qualsiasi cosa.

| Controllo | Che cosa lasciava passare | Rimedio | Gotcha |
|---|---|---|---|
| **quinto — `gate-attributes.sh`** | ⛔ **un build script dichiarato fra APICI SINGOLI**: il pattern ancorava sulla virgoletta doppia, e `build = 'gen.rs'` è lo stesso valore in TOML. Provato: cargo 1.95.0 lo **costruisce con exit 0** e lo **esegue** — il file `output` porta il `cargo:rustc-env` iniettato. Con gli altri cinque ciechi per costruzione, la porta usciva **verde su sei su sei** col kernel che legge orologio, filesystem e ambiente | àncora sulla **chiave** e non sul delimitatore: accetta `"`, `'` e `[`. Provato su cinque forme — `'…'`, `"…"` e `[…]` rossi; `build = false`, che **disattiva** e deve passare, e l'assenza della riga verdi | **#61**, e il **#28** riaperto |
| **sesto — `check-docs.sh`, esistenza della spec** | ⛔ **la spec rinominata uccideva le SEI asserzioni di §8.6.1 in silenzio**: vivono in blocchi `END` di `awk`, ed `END` **non gira** su un fatal. Variabili vuote, nessun `report`, **exit 0** | guardia d'esistenza **fuori** da `awk`, prima delle due passate. 📌 La lezione riusabile: *una guardia di non-vacuità dentro `END` non può, per costruzione, difendere dall'input che manca* | **#60** |
| **sesto — `check-docs.sh`, i due controlli su glob** | ⛔ `nullglob` è **off**: rinominando `docs/superpowers/specs/`, duplicati di sezione e **V30** davano **zero rossi** mentre tutte e ventiquattro le Q perdevano il metodo | conteggio dei file prima del ciclo, con messaggio proprio | **#60** |
| **sesto — `check-docs.sh`, V30** | ⚠️ **falso positivo**, non falso negativo: `sort -uV` in pasto a `comm`, che confronta per **collazione**. Latente finché i due insiemi coincidono; con `Q9` privo di metodo riportava **sedici** nomi invece di uno | `sort -u` su entrambi i lati | **#62** |

⛔ **E una voce aperta che l'audit ha lasciato, ed è la più grave del rapporto:** la suite di
conformità prova **V6 solo su un archivio vuoto** — gotcha **#63**. Non è riparabile qui: è
un'**aggiunta al contratto di una porta condivisa**, cioè una decisione del proprietario. Il
dettaglio, con la mutazione e la prova che è osservabile, sta in
[`audit-2026-08-11.md`](audit-2026-08-11.md) §5.1.

**Un comando solo:** `bash scripts/gate.sh`

| # | Il passo, col nome che `gate.sh` gli dà | |
|---|---|---|
| 1 | `workspace build` | rende **visibile** il livello 1 |
| 2 | `example and compile-fail tests` | banco `compile_fail`, contro-sonde, round-trip delle voci spedite |
| 3 | `no-OS gate` | livello 2 — `scripts/gate-no-os.sh` |
| 4 | `allow-list on the two graphs` | livello 2 — `scripts/gate-deps.sh` |
| 5 | `attributes of the constrained crates` | livello 2 — `scripts/gate-attributes.sh` |
| 6 | `documentation consistency` | livello 2 — `scripts/check-docs.sh` |
| 7 | `DST campaigns -- wall time` | il **tempo di parete** delle tre campagne, ristampato con `--nocapture` |

⚠️ **E IL LOG DEL CANCELLO NON È UNA CORSA SOLA: la baseline NON si aggrega da lì.** Il passo 7
ristampa le campagne con `--nocapture`, quindi il log porta **più** righe `test result:` di
`cargo test --locked --workspace --no-fail-fast`, e sommarle dà un totale più alto — misurato il
2026-08-28 cascandoci, e corretto rifacendo la misura col comando giusto. 📌 **Non è promossa a
riga di §9:** è la forma già registrata — *un contatore che parte da un valore che il soggetto
sotto esame non ha prodotto non è un oracolo su quel soggetto* — e un gotcha che non insegna
niente diluisce quelli che insegnano.

⛔ **RICHIAMO DEL 2026-08-27, finding AUD-062: questa tabella ricopiava il TESTO dei comandi e
ometteva il settimo passo.** Diceva `cargo build --workspace` e `cargo test --workspace`, cioè la
forma **senza `--locked`** che il finding **G-5** ha chiuso il 2026-08-18, e si fermava a sei
righe mentre `gate.sh` ne esegue **sette**. ⛔ **Il rimedio non è riallineare il testo: è togliere
la copia.** I comandi con le loro opzioni vivono in `scripts/gate.sh`, in una casa sola; qui i
passi si nominano con l'**etichetta** che lo script stesso dà loro, che identifica senza invitare
a eseguire. È la regola di `CLAUDE.md` — *un puntatore che vive in più documenti si toglie, non
si ricorregge*.

⚠️ **I primi due non «sono» il livello 1.** Il livello 1 *è* il compilatore e vale a ogni
compilazione, senza che nessuno lo lanci; `gate.sh` compila perché una porta che non
compila non prova niente. La stessa distinzione è scritta in testa a `scripts/gate.sh`.

La CI lancia lo stesso identico comando: `.github/workflows/quality-gate.yml` — `name: quality
gate`, job `gate`. ⚠️ **Rinominato il 2026-08-09, ed era l'ultimo residuo italiano nel codice:**
il workflow non era **mai stato eseguito** — committato lo stesso giorno, ramo non ancora
pushato — quindi nessuna regola di protezione del ramo poteva ancora riferirsi al nome vecchio.
Dopo la prima corsa non sarebbe più stato gratis.

## Livello 1 — il compilatore

Le tre righe del **blocco A** di §7.4.1, **nove righe del blocco C** e **una del blocco B** —
otto dal Traguardo 2, **due dal Traguardo 3**. ⚠️ **I conteggi si ricontano sulla tabella qui
sotto, mai si deducono** — gotcha **#31**: questa riga ne ha già portati **tre** sbagliati, e il
terzo è del **2026-08-10**: diceva *«sette righe del blocco C … queste ultime otto dal Traguardo
2»*, e il Task 1 del Traguardo 3 aveva già consegnato `record_without_version.rs` **senza
scrivere qui la propria riga**. Un numeratore invecchia anche quando nessuno tocca il
denominatore. Del blocco **B**
(i gettoni) è coperta **una riga su cinque**, `promuovere testo a istruzione ← la porta
journal` (V19); perché gli altri quattro non lo siano lo dice la riga del blocco **B** in
«Cosa la porta NON controlla», e da oggi **lo dice in un posto solo**.

⚠️ **Richiamo del 2026-08-19 — Traguardo 5, Task 1: il numeratore del blocco C è invecchiato
di nuovo, nella stessa specie già descritta sopra.** Le due righe `Q2 · §5.1` (regola A e
regola B) portano il blocco C da **nove** a **undici**, ricontate sulla tabella qui sotto e
non dedotte — gotcha **#31**. La frase «otto dal Traguardo 2, due dal Traguardo 3» resta
com'era: descrive **quelle** nove, non la cifra di oggi.

⚠️ **Richiamo del 2026-08-19 — Traguardo 5, Task 4: il numeratore si muove una terza volta, e
questa volta si muove anche quello del blocco B.** `V4` e `I2 · §5.3` portano il blocco C da
**undici** a **tredici**, e la riga del blocco **B** *«avviare un worker ← una concessione»*
passa da scoperta a **PARZIALE** — ricontate sulla tabella qui sotto, mai dedotte. ⛔ **Il
blocco B resta a «una riga su cinque» COPERTA:** una riga `parziale` non entra nel numeratore
delle chiuse, per la stessa regola con cui `Q8 · §5.2.1` non vi è entrata col Task 3.

⛔ **CORRETTO in un compito di rifinitura, stesso giorno: i due richiami qui sopra sommano
`Q2 · §5.1` come DUE righe di catalogo, e nel catalogo (§7.4.1 blocco C) è UNA riga sola** —
la stessa voce già aperta come **R3**, poco più sotto in questa stessa sezione. ⚠️ **I salti giusti sono nove
→ DIECI (Task 1) e dieci → DODICI (Task 4)**, non undici e tredici; la spiegazione intera e la
convenzione usata per contare stanno nella riga del blocco C in «Cosa la porta NON controlla».
La tabella «Livello 1» qui sotto **non cambia**: per R3 continua a mostrare `Q2 · §5.1` come
due righe di mappatura regola-a-file, una scelta dichiarata e non una svista.

⚠️ **Richiamo del 2026-08-19 — Traguardo 5, Task 5: le DUE righe che avevano un innesco
scritto si chiudono, ed è l'innesco che ha fatto il lavoro.** `admit` esiste, quindi
`Q8 · §5.2.1` passa da **parziale** a **coperta** — il caso negativo ora nomina il percorso
decisionale invece di un tipo fuori contesto — e `V2` da **scoperta** a **coperta**, con un
caso nuovo. Blocco C: **da dodici a QUATTORDICI righe su diciannove**, ricontate direttamente
su §7.4.1 blocco C e non da questa frase. Non restano righe `parziali` nel blocco C. ⛔ **E la
riga del blocco B *«avviare un worker ← una concessione»* passa da PARZIALE a COPERTA**, con
la contro-sonda che il Task 4 dichiarava non scrivibile: `crates/kernel/tests/arbiter_admission.rs`
ottiene concessioni vere da `admit`. Il blocco **B** passa quindi da **una riga su cinque** a
**DUE su cinque** coperte.

⚠️ **Richiamo del 2026-08-20 — Traguardo 5, Task 8: l'ultima riga del blocco C che l'arbitro
poteva chiudere si apre a METÀ.** `V3` — *«una seconda policy attiva: il valore consegnato ne
porta una sola»* — passa da **scoperta** a **PARZIALE**: `VramPolicy` è un enum e `Arbiter::new`
lo prende **per posizione**, quindi due policy sono un errore di **arità** — ma la contro-sonda
che la cella di catalogo pretende è doppia, e la sua seconda metà (*«e la transizione resta un
passo giornalato (§5.4)»*) è del **Task 9**. ⛔ **Il numeratore NON si muove: restano QUATTORDICI
righe su diciannove**, ricontate direttamente su §7.4.1 blocco C e non da questa frase, e il
denominatore nemmeno. ⚠️ **Questa riga diceva *«da quattordici a QUINDICI»*, corretto nell'ondata
di correzioni dello stesso giorno:** una riga parziale non è una riga chiusa, ed è la regola che
questo stesso file aveva già applicato **due volte** — a `Q8 · §5.2.1` al Task 3 e alla riga del
blocco B. ⏳ **Innesco: la chiude il TASK 9**, ed è quel compito a muovere il numeratore a
quindici. Voce `E103`. ⛔ **Il blocco B resta a DUE righe su cinque:**
questo compito non ne tocca nessuna. ⚠️ **E il caso negativo NON è quello che il piano
dettava** — il perché, con la misura delle due forme, sta nella sezione del Task 8 qui sotto.

✅ **Richiamo del 2026-08-20 — Traguardo 5, Task 9: l'innesco qui sopra è SCATTATO ed è stato
RACCOLTO.** La seconda metà della contro-sonda di catalogo — *«e la transizione resta un passo
giornalato (§5.4)»* — esiste: `Arbiter::set_policy` scrive l'**intento** nel giornale, scambia
l'oggetto e scrive l'**esito**, e **cinque** sonde nuove di `crates/kernel/tests/arbiter_policy.rs`
la esercitano **da fuori la crate**. `V3` passa quindi da **PARZIALE** a **COPERTA**, e il
numeratore del blocco C **si muove** — la cifra si riconta sulla cella del blocco C in fondo a
questo file, non da questa frase; il denominatore non si muove. ⛔ **Nel blocco C non resta
nessuna riga `parziale`.** ⛔ **Il blocco B non si muove:** questo compito non ne tocca nessuna
riga, e la cifra si riconta sulla cella del blocco B in fondo a questo file. Il dettaglio, con la
campagna di mutazione, sta nella sezione del Task 9 qui sotto.

✅ **Richiamo del 2026-08-21 — Traguardo 5, Task 11: le righe rimaste di §6.10.5 escono dallo
scaglionamento.** I quattro casi `compile_fail` di `crates/kernel/tests/compile_fail/`, con le
contro-sonde di `crates/kernel/tests/worker_tokens.rs` che ottengono un `Grant` vero da
`Arbiter::admit`, tengono il `Worker` e la ricevuta: i numeratori dei blocchi **B** e **C**
**si muovono entrambi** — le cifre **e ciò che resta scoperto** si ricontano sulla cella del
blocco B e sulla cella del blocco C in fondo a questo file, non da questa frase. Il dettaglio
sta nella sezione «P-2» più sotto.

✅ **Richiamo del 2026-09-01 — Traguardo 6, Compito 6: l'ULTIMA riga del blocco B si chiude, e
con essa il blocco.** `Q13` — *«eseguire una richiesta ← una prova di conformità»* — passa da
**scoperta** a **coperta**: `Conforming` vive in `crates/kernel/src/gateway/mod.rs` a campi
privati, coniato dal solo `resolve`, e i casi sono **due** perché le metà del gettone sono due —
`dispatching_an_unfiltered_candidate.rs` e `conforming_has_no_constructor.rs`, con la
contro-sonda in `crates/kernel/tests/gateway_decisor.rs`. ⛔ **La cifra si riconta sulla cella
del blocco B in fondo a questo file, non da questa frase**, ed è quella cella a dire anche che
cosa resta. Il dettaglio, con la passata di mutazione, sta nella sezione «Il gettone di
conformità del gateway» più sotto.

⛔ **`V2` non era nel brief del Task 5, ed è stato scritto lo stesso: l'innesco lo aveva
scritto il Task 3, in questo stesso file, alla riga «si chiude insieme a `Q8` allo stesso Task
5».** Un innesco che scatta e che nessuno raccoglie lascia il registro a mentire con
autorevolezza. Non è una riga di catalogo **nuova** — `V2` sta in §7.4.1 blocco C da sempre,
fra le scoperte — quindi il vincolo globale 7 non è toccato: qui si **copre** una riga, non se
ne aggiunge una. Divergenza registrata come **E23** nell'errata del piano.

| Regola del catalogo | Dove è dichiarata | Caso negativo |
|---|---|---|
| `#![no_std]` su `kernel` e `simulator` | `crates/kernel/src/lib.rs` · `crates/simulator/src/lib.rs` | `crates/kernel/tests/compile_fail/std_in_kernel.rs` |
| `#![forbid(unsafe_code)]` sulle stesse | idem | `crates/kernel/tests/compile_fail/unsafe_in_kernel.rs` · `allow_overrides_forbid.rs` |
| `HashMap` non nominabile | conseguenza gratuita di `no_std` | `crates/kernel/tests/compile_fail/hashmap_in_kernel.rs` |
| **blocco C** · `V29 · §2.1` — i due tempi non si scambiano, **in nessuna delle due direzioni** | `crates/kernel/src/time.rs` — `Monotonic` e `WallTime` sono due tipi distinti | `monotonic_as_wall.rs` · `wall_as_monotonic.rs` |
| **blocco C** · `V29 · §2.1` — **nessuna via `From`/`Into`** fra i due tempi | idem: nessuna conversione è dichiarata, e il divieto non è più un commento | `no_conversion_from_monotonic_to_wall.rs` · `no_conversion_from_wall_to_monotonic.rs` |
| **blocco C** · `V29 · §2.2` — la **riduzione** di `below` non è sovrascrivibile | `crates/kernel/src/rng.rs` — `below` vive su `RngExt`, con `impl<R: Rng> RngExt for R {}` | `override_below.rs` |
| **blocco C** · `V29 · §2.8` — il kernel **non nomina un default** | `crates/kernel/src/parameters.rs` — nessun `impl Default`, e `new` pretende ogni campo | `parameters_have_no_default.rs` |
| **blocco C** · `V29 · §2.8 · ADR-0034` — una decisione **senza i parametri consegnati** | `crates/kernel/src/executor.rs` — `Executor::new` prende `Parameters` **per posizione**, quindi ometterli è un errore di arità e non un valore messo di default in silenzio | `executor_without_parameters.rs` |
| **blocco C** · `Q9 · I6 · V20` — `Untrusted` **dove è attesa** un'`Instruction`, **regola A** | `crates/kernel/src/boundary.rs` — `Instruction` e `Untrusted` sono due tipi distinti | `untrusted_as_instruction.rs` |
| **blocco C** · `Q9 · I6 · V20` — **nessuna via `From`/`Into`** da `Untrusted` a `Instruction`, **regola B** | idem: nessuna conversione è dichiarata, e l'unica strada ammessa è `promote`, che pretende il giornale | `no_conversion_from_untrusted_to_instruction.rs` |
| **blocco C** · `Q2 · §5.1` — VRAM e durata non si scambiano, **in nessuna delle due direzioni** — **regola A** | `crates/kernel/src/arbiter/resource.rs` — `Mib` e `kernel::time::Millis` sono due tipi distinti | `mib_as_millis.rs` · `millis_as_mib.rs` |
| **blocco C** · `Q2 · §5.1` — **nessuna via `From`/`Into`** fra `Mib` e `Millis` — **regola B** | idem: nessuna conversione è dichiarata | `no_conversion_from_mib_to_millis.rs` · `no_conversion_from_millis_to_mib.rs` |
| **blocco C** · `Q14 · §4.9` — un record durevole **senza versione** | `crates/kernel/src/record.rs` — `Record` è un enum di versione, e l'`encode` inerente vive su di esso, non sul corpo `RecordV1` | `record_without_version.rs` |
| **blocco C** · `Q9 · I6 · V20 · §4.9` — un payload non fidato **senza la propria etichetta** | `crates/kernel/src/record.rs` — `RecordV1::trust` non è un `Option` e `Trust` non implementa `Default`. ⛔ **Le metà sono due e i casi sono due**, come per i due tempi: il primo tiene *«il campo esiste»* (`E0063`), il secondo *«e non ha default»* (`E0277`) | `record_without_trust_label.rs` · `trust_has_no_default.rs` |
| **blocco C** · `V4` — l'esito dell'arbitro trattato come **due vie** invece di tre | `crates/kernel/src/arbiter/mod.rs` — `Admission` ha **tre** varianti e nessun `is_granted()`, nessuna conversione a booleano: distinguerle non è una raccomandazione, è la firma. ⛔ **Le metà sono due e i casi sono due**, come per i due tempi: il primo tiene *«un `match` a due bracci non compila»* (`E0004`), il secondo *«e non c'è nemmeno la scorciatoia booleana»* (`E0599`) | `admission_is_not_two_ways.rs` · `admission_has_no_is_granted.rs` |
| **blocco C** · `V2` — un'**ammissione senza profilo** di risorsa | `crates/kernel/src/arbiter/mod.rs` — `Arbiter::admit` prende il `&ResourceProfile` **per posizione**, quindi ometterlo è un errore di arità e non un'ammissione che decide sul nulla | `admission_without_profile.rs` |
| **blocco C** · `V3` — una **seconda policy attiva**. ✅ **COPERTA dal Task 9:** la seconda metà della contro-sonda — *«e la transizione resta un passo giornalato (§5.4)»* — è `Arbiter::set_policy`, esercitata dalle **cinque** sonde della transizione in `crates/kernel/tests/arbiter_policy.rs` (`E103`, chiusa) | `crates/kernel/src/arbiter/policy.rs` e `mod.rs` — `VramPolicy` è un **enum**, quindi un valore ne porta una sola, e `Arbiter::new` lo prende **per posizione**: passarne due è un errore di arità. ⚠️ Il caso pinza l'**arità di `new`** e non l'assenza di ogni strada: con un secondo costruttore a due policy resta `ok`, **misurato** | `two_policies_at_once.rs` |
| **blocco C** · `Q8 · §5.2.1` — l'**ammissione legge `cold_start`** | `crates/kernel/src/arbiter/` — `admit` riceve un `ResourceProfile`, che quel campo non ce l'ha; `cold_start` vive su `WorkDescriptor`, che l'ammissione non riceve. ⚠️ Dal Task 5 il caso costruisce il profilo **e lo passa ad `admit` nello stesso `main`**: la regola è provata sull'argomento che la decisione prende davvero | `admission_reads_cold_start.rs` |
| **blocco C** · `I2 · §5.3` — la **revoca** per un profilo **non prelazionabile** | `crates/kernel/src/arbiter/mod.rs` — `Activity::NonPreemptible` è una variante **unitaria**: non ha dove metterla, quindi lo stato illegale non è vietato, è **indicibile**. ⚠️ La cella di catalogo scrive `InRevoca`; il codice si chiama `Revoking` per la §1.0 della spec — divergenza già registrata come `R5` del piano del Traguardo 5 | `revoking_a_non_preemptible_grant.rs` |
| **blocco C** · `I2 · §6.10` — **istruire un worker dopo l'uccisione**: l'uccisione **consuma** il `Worker` (§6.10.2). ✅ **COPERTA dal Task 11** — vedi la sezione «P-2» qui sotto | `crates/kernel/src/ports/process.rs` — `Worker::kill` prende `self` **per valore**, quindi istruirlo dopo non è vietato da una regola di condotta: è un uso dopo spostamento (`E0382`). ⚠️ La cella di catalogo scrive `uccidi`; il codice si chiama `kill` per la §1.0 della spec. È la stessa specie di divergenza di `R5` del piano del Traguardo 5, che però censisce solo le due celle di identificatori **nati con quel traguardo**: questa è del Traguardo 2, e dal 2026-08-24 è **registrata come voce `E140`** di quel piano — chiuderla è del **proprietario**, perché tocca la §7.4 | `instructing_after_the_kill.rs` |
| **blocco C** · `I5 · §6.10` — **leggere due volte dalla stessa ricevuta singola**: la lettura la consuma (§6.10.2). ✅ **COPERTA dal Task 11** — vedi la sezione «P-2» qui sotto | `crates/kernel/src/ports/process.rs` — `Worker::read_one` prende la `SingleReceipt` **per valore** e `SingleReceipt` non è `Copy` — e non lo può diventare, perché non deriva nemmeno `Clone` — quindi la seconda lettura è un uso dopo spostamento (`E0382`). ⚠️ **Il derive che fa da cardine è `Copy`, non `Clone`, ed è quello che l'oracolo nomina** (`reading_twice_from_one_receipt.stderr`: *«does not implement the `Copy` trait»*): chi verificasse la guardia guardando `#[derive(Clone)]` guarderebbe la cosa sbagliata | `reading_twice_from_one_receipt.rs` |
| **blocco C** · `V10` — un **sensore che modifica l'artefatto**: §6.4.2 lo consegna per riferimento immutabile. ✅ **COPERTA dal compito 5 del Traguardo 6** | `crates/kernel/src/sensor.rs` — `Sensor::observe` prende `artefact: &Untrusted`, quindi assegnarvi dentro è `error[E0594]`, *cannot assign to `*artefact`, which is behind a `&` reference*. ⚠️ **LE VIE SONO DUE E LA SECONDA È MISURATA, non dedotta:** un `impl` che **dichiara** `observe(&self, artefact: &mut Untrusted)` non combacia col tratto e dà `error[E0053]` — misurato il 2026-09-01 con una sonda usa-e-getta compilata e cancellata nella stessa corsa. ⛔ **Il caso scritto è quello del CORPO**, perché la cella di catalogo parla della **consegna** e non di ciò che un implementatore può dichiarare; la seconda via è scritta nel commento del caso perché chi allarga la riga sappia che esiste ed è già chiusa | `sensor_modifies_the_artefact.rs` |
| **blocco B** · `V19` — **promuovere testo a istruzione ← la porta `journal`** | `crates/kernel/src/boundary.rs` — `Untrusted::promote` pretende il giornale come **argomento**, e la registrazione fallita fa fallire la promozione | `promote_without_journal.rs` |
| **blocco B** · `I2` — **avviare un worker ← una concessione**. ✅ **COPERTA dal Task 5**, quando la metà che mancava è diventata scrivibile — vedi la sezione del Task 5 qui sotto | `crates/kernel/src/arbiter/mod.rs` — `Grant` vive dove lo si **emette**, con un campo privato e nessun costruttore pubblico; `Process::start` lo pretende per valore, e `Arbiter::admit` è l'**unico** che ne conia uno | `grant_has_no_constructor.rs` |
| **blocco B** · `I2` — **parlare a un worker ← l'oggetto `Worker`** che l'avvio ha restituito. ✅ **COPERTA dal Task 11** — vedi la sezione «P-2» qui sotto | `crates/kernel/src/ports/process.rs` — `instruct_one` vive sul tratto `Worker`, e ciò che si ha **prima** dell'avvio — `WorkerDescriptor` — non lo implementa: il metodo non esiste su di esso (`E0599`). La direzione *«col `Worker` → compila»* la tiene `crates/kernel/tests/worker_tokens.rs` | `talking_without_the_handle.rs` |
| **blocco B** · `I5 · Q4` — **leggere da un worker ← una ricevuta**. ✅ **COPERTA dal Task 11** — vedi la sezione «P-2» qui sotto | `crates/kernel/src/ports/process.rs` — `Worker::read_one` pretende la `SingleReceipt` come argomento, quindi ometterla è un errore di **arità** (`E0061`). ⛔ **Il caso prova l'arità e non l'autenticità della ricevuta:** `SingleReceipt::new` è `pub` e raggiungibile da fuori la crate, e il limite è dichiarato accanto al costruttore nel sorgente — è il contrario del gettone `Grant`, che un costruttore pubblico non ce l'ha. La direzione *«con la ricevuta → compila»* la tiene `crates/kernel/tests/worker_tokens.rs` | `reading_without_a_receipt.rs` |
| **blocco B** · `Q13` — **eseguire una richiesta ← una prova di conformità**. ✅ **COPERTA dal compito 6 del Traguardo 6** — vedi la sezione «Il gettone di conformità del gateway» qui sotto | `crates/kernel/src/gateway/mod.rs` — `Conforming` ha **tutti** i campi privati e l'unico che ne conia uno è `resolve`, nello stesso modulo; `dispatch` lo pretende **per valore**. ⛔ **Le metà sono due e i casi sono due**, come per il gettone `Grant`: il primo tiene *«un candidato non filtrato non è l'argomento»* (`E0308`), il secondo *«e il gettone non si conia»* (senza sigla). La direzione *«filtrato → compila»* la tiene `a_conforming_candidate_is_chosen_and_nothing_is_degraded` in `crates/kernel/tests/gateway_decisor.rs` | `dispatching_an_unfiltered_candidate.rs` · `conforming_has_no_constructor.rs` |

⛔ **La regola uscita dalla riga della regola B, e vale oltre il caso** (gotcha **#36**, terza
occorrenza): quando un compito implementa un controllo che il catalogo non ha, il posto dove
dichiararlo è **il registro**, e la dichiarazione va scritta come **voce aperta**, non come
nota. Una nota si legge e si dimentica; una voce aperta la §6 del
[compendio](COMPENDIO.md) la porta finché qualcuno la chiude.

#### Come scattano le due direzioni, e perché la differenza conta

⛔ **Le due direzioni non sono simmetriche nel modo di scattare, e la differenza conta —
gotcha #42.** `trybuild` stampa **`error`** quando un caso ha compilato e **`mismatch`**
quando l'uscita non combacia con l'oracolo:

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
| Mib e Millis non si scambiano — **regola A** (`mib_as_millis.rs` · `millis_as_mib.rs`) | `mismatch` | **sì** — stessa specie dei due tempi, vedi sotto |
| nessuna via `From`/`Into` fra Mib e Millis — **regola B** | **`error`** | no |
| l'esito dell'arbitro a **due vie** (`admission_is_not_two_ways.rs`) | **`error`** | no |
| la **revoca** su un profilo non prelazionabile (`revoking_a_non_preemptible_grant.rs`) | **`error`** | no |
| la concessione **forgiata da fuori** (`grant_has_no_constructor.rs`) | ⛔ **dipende da COME si rompe la regola** — `error` se il tipo perde i campi, `mismatch` se il campo diventa `pub` | **in parte sì** — vedi la sezione del Task 4 |
| la **scorciatoia booleana** sull'esito (`admission_has_no_is_granted.rs`) | **`error`** | no — misurato al Task 5, con `is_granted` aggiunto |
| l'**ammissione senza profilo** (`admission_without_profile.rs`) | **`error`** | no — misurato al Task 5, togliendo il profilo dalla firma di `admit` |
| l'**ammissione che legge `cold_start`** (`admission_reads_cold_start.rs`) | `mismatch` | ⛔ **sì**, e va detto invece di lasciarlo dedurre — vedi la sezione del Task 5 |
| la **maniglia** — parlare a un worker senza l'oggetto `Worker` (`talking_without_the_handle.rs`) | **`error`** | no — ⚠️ **e soltanto dal 2026-08-24**: finché il caso non importava il tratto, sotto `impl Worker for WorkerDescriptor` il metodo restava irrisolvibile e il file continuava a non compilare, cioè `mismatch`. L'import `Worker as _` è la correzione, e la ragione sta accanto a esso nel sorgente |
| **istruire dopo l'uccisione** (`instructing_after_the_kill.rs`) | ⛔ **dipende da COME si rompe la regola** — `error` se `kill` smette di prendere `self` **e** la finta del caso segue la firma nuova; `mismatch` se cambia solo il tratto, perché a rompersi per prima è la finta (`E0053`) | **in parte sì** |
| **leggere senza ricevuta** (`reading_without_a_receipt.rs`) | ⛔ **dipende da COME si rompe la regola**, stessa specie della riga qui sopra — `error` col tratto **e** la finta in passo; `mismatch` col solo tratto (`E0050`) | **in parte sì** |
| **leggere due volte dalla stessa ricevuta** (`reading_twice_from_one_receipt.rs`) | **`error`** con `Copy` su `SingleReceipt`, che è il derive che disarma | **in parte sì** — ⚠️ col solo `Clone` il caso **non** è disarmato e resta rosso, ma `mismatch`: rustc appende un `help: consider cloning the value` che l'oracolo non porta |

⛔ **Le quattro righe di §6.10.5 sono classificate MISURANDO la regressione di ciascuna, non
deducendola — 2026-08-24.** Ognuna è stata riprodotta con una sonda `rustc` **fuori dal
repository**, nella stessa forma — tratto e tipi in una crate, caso in un'altra — perché
provarla qui vorrebbe dire mutare `crates/kernel/src/ports/process.rs`, uno dei quattro file
con `CR` nell'indice, e il rischio del ripristino supera il valore. ⚠️ **Due delle quattro
hanno risposto «dipende», e la ragione è strutturale:** quei casi portano una **copia** della
finta, quindi se la firma del tratto cambia da sola è la finta a rompersi per prima — un rosso
vero, ma della specie che una rigenerazione in blocco riscrive.

⛔ **La riga della regola A è la divergenza, e si registra invece di allinearla all'attesa.**
Il gotcha #42 prevedeva un `mismatch` — rustc che aggiunge righe di `help: call Into::into`
che l'oracolo non porta — e **su questa coppia non succede**. Misurato: con
`impl From<Untrusted> for Instruction` presente, `untrusted_as_instruction.rs` resta **`ok`**,
perché lì lo scarto è fra **riferimenti** (`&Untrusted` contro `&Instruction`) e quell'impl non
dà nessun `&Untrusted: Into<&Instruction>`, quindi rustc non ha suggerimenti da appendere. Sui
due tempi lo scarto è fra **valori posseduti**, e lì il suggerimento compare. Quella guardia
non è «disarmabile da una rigenerazione»: è **cieca fin dall'inizio**, e su I6 il caso della
regola B non è una rifinitura — senza, l'`impl From` lascia la porta **verde** con il confine
già caduto.

Misurato: col solo caso «passa l'uno per l'altro», aggiungere `impl From<WallTime> for
Monotonic` lasciava la porta **verde su sei controlli su sei**. La riga `From`/`Into` esiste
perché quella era la direzione **pericolosa** — una decisione che dipende dal wall time — e
perché una regola guardata solo da casi `mismatch` non è guardata abbastanza.

⛔ **`Q2 · §5.1` ripete esattamente la specie dei due tempi, e non quella di I6 — misurato e
non dedotto, Traguardo 5 Task 1.** Comando: `cargo test --locked -p kernel --test
compile_fail 2>&1 | grep -E "error|mismatch|ok"`, eseguito due volte con una mutazione
temporanea in fondo a `crates/kernel/src/arbiter/resource.rs`, e prima di ogni corsa provato
che la mutazione fosse entrata con `grep -c "impl From<...>" crates/kernel/src/arbiter/resource.rs`
→ `1`.

| Mutazione temporanea | `no_conversion_from_*` della stessa direzione | il caso «passa l'uno per l'altro» della stessa direzione |
|---|---|---|
| `impl From<Mib> for crate::time::Millis` | `no_conversion_from_mib_to_millis.rs` → **`error`** (ha compilato) | `mib_as_millis.rs` → **`mismatch`**, non `ok` |
| `impl From<crate::time::Millis> for Mib` | `no_conversion_from_millis_to_mib.rs` → **`error`** | `millis_as_mib.rs` → **`mismatch`**, non `ok` |

⚠️ **Il rosso atteso dal compito era «gli altri restano `ok`», e non regge: è la stessa
soppressione indiretta già misurata per `Monotonic`/`WallTime` — righe 113–122 qui sopra —
perché `Mib` e `Millis` sono anch'essi **valori posseduti** e non riferimenti: con l'impl
presente rustc aggiunge un `help: call \`.into()\`` che l'oracolo non porta, quindi il caso
**non torna `ok`**, torna `mismatch`. La regola B resta comunque isolata da questo effetto —
scatta `error` in entrambe le direzioni, indipendente dall'oracolo — che è la ragione per cui
esiste. Entrambe le mutazioni **revocate** con lo strumento di edit (mai `git checkout --`,
gotcha #48 dodicesima forma), e provato con `grep -c "impl From" crates/kernel/src/arbiter/resource.rs`
→ `0` prima di rilanciare il banco verde.

⚠️ **VOCE APERTA — la riga di catalogo `Q2 · §5.1` è UNA e in UNA direzione, mentre qui sopra
sono DUE regole in DUE direzioni ciascuna (R3 del piano del Traguardo 5).** §7.4.1 blocco C
scrive *«MiB assegnati a millisecondi»*, contro-sonda *«ciascuno con sé stesso»* — la stessa
forma a una via che la riga dei due tempi aveva prima che V29 la allargasse. Qui i quattro
casi mordono già in entrambe le direzioni e su entrambe le regole; a mancare è la **riga**,
non il controllo. Aggiornare §7.4 è **spec**, quindi del proprietario — vincolo globale 7.
**Registrata come voce aperta e non come nota** (gotcha #36), stesso trattamento di PL-1 e
K-1/B-1.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo
a questo file**, che dice anche **chi la chiude** — rimando aggiunto il 2026-08-25.

#### `ComputeClass` e `Preemption` — Traguardo 5, Task 2, e nessuna riga di catalogo qui

⛔ **`ComputeClass` NON ha una riga di catalogo, ed è dichiarato invece di lasciarlo
dedurre.** L'ordine delle tre corsie di calcolo è un **valore**, non una forma: il
compilatore non ha niente da rifiutare se qualcuno riordina le varianti dell'enum, quindi
non è materia di `compile_fail`. A tenerlo è una **sonda a esempi**, nel file esistente
`crates/kernel/tests/arbiter_resource.rs` (ora **sei** test, tre del Task 1 e tre nuovi):

| Sonda nuova | Cosa tiene |
|---|---|
| `the_lane_order_is_pinned_by_name_and_realtime_comes_first` | l'ordine **per nome** — `Realtime < Interactive < Batch` — e la chiave `priority()` stessa, così un lettore non deve inferirla dalle disuguaglianze |
| `the_three_lanes_are_distinct_and_the_ordering_is_total` | la contro-sonda: l'ordinamento è **totale**, non solo le tre disuguaglianze dichiarate — nessuna coppia di corsie confronta uguale a un'altra |
| `a_grace_time_exists_exactly_when_the_profile_is_preemptible` | ciò che `Preemption` rende **non dicibile**: un profilo non preemptibile non può portare un tempo di grazia, e uno preemptibile non può esserne privo — due stati illegali insieme, non uno |

⛔ **Le DUE mutazioni del Passo 4, ed entrambe provate.** La prima deve uccidere; la seconda
— la direzione che si dimentica — deve restare verde:

| Mutazione | Applicata su | Atteso | Misurato |
|---|---|---|---|
| la **chiave**: `Realtime => 2`, `Batch => 0` (`Interactive` invariato) | `ComputeClass::priority` | rosso su `the_lane_order_is_pinned_by_name_and_realtime_comes_first` | ✅ rosso, e anche `the_three_lanes_are_distinct_and_the_ordering_is_total` cade con lei — **4 passati, 2 falliti** |
| **contro-mutazione**: le varianti riordinate `Batch`, `Interactive`, `Realtime` nella dichiarazione dell'enum, la chiave **intatta** | `enum ComputeClass` | verde, sei su sei | ✅ verde — **6 passati, 0 falliti** — è la prova che la trappola di un `Ord` derivato è stata **tolta** e non sorvegliata: con un `Ord` derivato quella stessa mutazione avrebbe rovesciato le priorità dell'arbitro senza un rosso |

Entrambe le mutazioni provate con `grep -c`/`grep -n` sulla riga mutata prima di ogni corsa e
revocate con lo strumento di edit, mai `git checkout --` (gotcha #48).

#### `ResourceProfile` e `WorkDescriptor` — Traguardo 5, Task 3, e `Q8 · §5.2.1` diventa parziale

⛔ **Le due strutture nascono in coda a `crates/kernel/src/arbiter/resource.rs`, legate SOLO
dal nome (§5.2.1).** `ResourceProfile` è ciò che l'arbitro RICEVE (§5.2): `name` (`&'static
str`, mai `String` — finding P-1), `reserved_vram`, `compute_class`, `preemption`, e SENZA
`cold_start`. `WorkDescriptor` va alla PRESENTAZIONE e mai all'ammissione: `profile_name` e
`cold_start`. `crates/kernel/tests/arbiter_resource.rs` guadagna due sonde, da **sei** a
**otto**:

| Sonda nuova | Cosa tiene |
|---|---|
| `cold_start_is_readable_outside_the_decision_path` | la contro-sonda di `Q8 · §5.2.1`: il campo È raggiungibile fuori dal percorso decisionale. ⚠️ **Il lettore è una FINTA.** Il catalogo chiama quel lettore «la proiezione di presentazione», e non esiste ancora nel codice: la sonda usa una funzione libera, `a_presentation_projection`, che prova la proprietà giusta con parole diverse da quelle della cella. Divergenza registrata per il proprietario, che la riformulerà quando la proiezione vera esisterà (§12 del disegno del Traguardo 5) |
| `a_descriptor_names_the_profile_it_describes` | che le due strutture si tengano per nome e nient'altro. ⚠️ **Forza di LIVELLO 1 soltanto — errata E5 del piano.** Il test scrive lo stesso letterale (`"trellis2-512-lean"`) in entrambe le strutture: l'`assert_eq!` a runtime **non può fallire**, qualunque cosa faccia il codice di produzione. Ciò che prova davvero è al COMPILATORE — che le due forme esistano, portino quei nomi e quei tipi di campo, e siano costruibili da FUORI la crate (l'unico posto dove un `pub` mancante si vede, gotcha #46). Registrata per non farla contare accanto alle sonde che mordono a runtime |

⛔ **Il Passo 4 misura che le sonde a ESEMPI non tengono la regola di §5.2.1, ed è il
risultato che serve — non un fallimento del compito.** Mutazione temporanea: `cold_start:
Millis` aggiunto a `ResourceProfile` (e l'unico call site esistente in
`arbiter_resource.rs` aggiornato per restare compilabile). Entrata provata con `grep -n
"cold_start" crates/kernel/src/arbiter/resource.rs` → **quattro** righe (due commenti, il
campo di `WorkDescriptor`, e il campo aggiunto). Poi, sull'intero workspace:

| Comando | Atteso | Misurato |
|---|---|---|
| `cargo test --workspace --no-fail-fast --locked` | verde: nessuna sonda a esempi tiene «`ResourceProfile` non ha `cold_start`» | ✅ verde — **33 blocchi di test, exit 0** — `arbiter_resource.rs` **8/8**, i **22** casi `compile_fail` di allora tutti `ok` |

Revocata con lo strumento di edit (mai `git checkout --`, gotcha #48), provato con `grep -c
"cold_start" crates/kernel/src/arbiter/resource.rs` → **tre** (i due commenti e il campo di
`WorkDescriptor`) prima di rilanciare il banco a otto test.

⛔ **`admission_reads_cold_start.rs` è il caso di livello 1 che tiene la regola dove le
sonde a esempi non arrivano.** Costruisce un `ResourceProfile` da fuori la crate e legge
`.cold_start` su di esso; oracolo generato **una volta** con `TRYBUILD=overwrite` e letto:
`` error[E0609]: no field `cold_start` on type `ResourceProfile` ``. ⚠️ **È la PRIMA METÀ
soltanto**, come dichiara il commento in testa al file: prova che il campo non è sulla
struttura, non che il percorso decisionale non vi arrivi — quel percorso non esiste finché
`admit` non arriva col Task 5, che riscrive questo caso per negarglielo direttamente.

`Q8 · §5.2.1` — *«l'ammissione legge `cold_start`»* — passa quindi da scoperta a
**PARZIALMENTE coperta**. ⛔ **Innesco scritto, come pretende §8.1 per ogni riga
`parziale`: si chiude al compito che porta `admit`** (Task 5). ⚠️ **`V2` (blocco C —
«un'ammissione senza profilo») resta invece scoperta**, senza innesco da scrivere qui:
nessun argomento di ammissione esiste ancora da rendere obbligatorio, e si chiude insieme a
`Q8` allo stesso Task 5.

✅ **RICHIAMO DEL 2026-08-19, TRAGUARDO 5 TASK 5 — i due inneschi qui sopra sono SCATTATI, ed
entrambe le righe sono chiuse.** `admit` esiste: `admission_reads_cold_start.rs` è stato
riscritto per costruire il profilo **e passarlo ad `admit` nello stesso `main`**, quindi la
regola è provata sull'argomento che la decisione prende davvero e non su un tipo citato fuori
contesto; `V2` ha ora un caso proprio, `admission_without_profile.rs`, che omette il profilo e
ottiene `E0061`. ⚠️ **La seconda riga non era nel brief del Task 5:** l'unica cosa che la
chiedeva era la frase qui sopra, e un innesco che scatta senza che nessuno lo raccolga lascia
questo file a mentire. Dettaglio nella sezione «Livello 1 · l'arbitro che ammette e rilascia».

#### `Grant`, `Admission` e `Activity` — Traguardo 5, Task 4, e `V4` e `I2 · §5.3` si chiudono

⛔ **`Grant` si è SPOSTATO da chi lo consuma a chi lo emette**, da
`crates/kernel/src/ports/process.rs` a `crates/kernel/src/arbiter/mod.rs`, e lo spostamento è
meccanico e non estetico: in Rust un campo privato è visibile al modulo che lo dichiara **e ai
suoi figli**, e il modulo `arbiter` è un **fratello** di `ports::process` — col tipo di là
l'arbitro non potrebbe costruire la cosa che esiste per emettere (`E0423`, decisione **D5-1**
del disegno). ⚠️ **Un costruttore `pub(crate)` lasciato di là è stato scartato sul merito:**
costa una riga e apre una strada — chiunque dentro `kernel` conierebbe una concessione senza
passare dall'ammissione (gotcha **#67**).

⛔ **E `Grant` NON si ri-esporta da `ports::process` (decisione D8), il che ha un rosso a
provarlo.** Senza ri-export ogni sito che nominava il tipo attraverso la porta si è presentato
da solo: `cargo test --locked --workspace` ha risposto
`` error[E0603]: struct `Grant` is private `` su
`crates/kernel/tests/ports_are_implementable.rs:52`, l'**unico** sito. ⚠️ **E la sigla è quella
e non un «import non risolto», per una ragione che vale la pena scrivere:** il `use
crate::arbiter::Grant;` dentro `ports::process` è **privato**, quindi da fuori il nome resta
*visibile ma vietato* — con un `pub use` nessuno si sarebbe presentato, e i due percorsi
sarebbero nati invisibili.

| Caso negativo nuovo | Riga | Errore misurato nell'oracolo |
|---|---|---|
| `admission_is_not_two_ways.rs` | `V4` | `` error[E0004]: non-exhaustive patterns: `Admission::Queued(_)` not covered `` |
| `revoking_a_non_preemptible_grant.rs` | `I2 · §5.3` | `` error[E0618]: expected function, found `Activity` `` — *«call expression requires function»* |
| `grant_has_no_constructor.rs` | blocco **B**, *«avviare un worker ← una concessione»* | ⛔ **senza sigla:** `` error: cannot construct `Grant` with struct literal syntax due to private fields ``, con `` note: private field `id` that was not provided `` |

⚠️ **Due sigle su tre erano state PREDETTE e una sola è stata indovinata — gotcha #15,
applicato al compito che lo cita.** Il piano attendeva `E0422`/`E0423` per la concessione
forgiata, e l'errore vero **non porta nessun codice**; per la revoca il piano si era
deliberatamente astenuto, ed è `E0618`, che dice *«questa cosa non è una funzione»* invece di
*«`NonPreemptible` non porta campi»* — lo stesso fatto detto dall'altro lato. Il commento in
testa a `grant_has_no_constructor.rs` è stato **corretto sulla misura**, non lasciato a
promettere una sigla che non esce.

⛔ **Le tre mutazioni del Passo 5, e una non poteva funzionare** — ciascuna applicata da sola,
provata entrata con `grep -c`, misurata e revocata con lo strumento di edit (mai `git checkout
--`, gotcha #48):

| # | Mutazione | Atteso dal piano | Misurato |
|---|---|---|---|
| 1 | il ramo `Admission::Queued(_) => false` aggiunto al `match` del caso | `error` | ✅ **`error`** — gli altri due restano `ok`, quindi il rosso viene dalla **terza via** e non da un errore di sintassi |
| 2 | il campo di `Grant` reso `pub` | `error` | ⛔ **`mismatch`, e non poteva essere altro** — vedi sotto |
| 2b | **sostitutiva**: `pub struct Grant {}`, il tipo svuotato dei campi | — | ✅ **`error`** — gli altri due restano `ok` |
| 3 | `Activity` appiattita in `NonPreemptible(PreemptibleState)` | `error` | ✅ **`error`** — gli altri due restano `ok` |

⛔ **Perché la mutazione 2 non poteva rovesciare il caso, ed è il difetto del PIANO e non
dell'esito.** Il caso scrive `kernel::arbiter::Grant {}` — **senza campo**. Rendere `id`
pubblico non fa compilare quel letterale: manca ancora il campo (`E0063`), e per giunta
`GrantId` è un tipo **privato** che una crate esterna non può nominare comunque, quindi la
strada resta chiusa da **due** porte e non da una. Misurato: il caso continua a **non**
compilare e passa da `ok` a **`mismatch`**, perché il messaggio cambia da *«cannot construct …
due to private fields»* a `` error[E0063]: missing field `id` in initializer of `Grant` ``.

⚠️ **E la mutazione 2 non è stata inutile: ha misurato di che SPECIE è questa guardia.** Rotta
per un verso (il campo diventa `pub`) scatta come **`mismatch`**, cioè **dipende
dall'oracolo** e una rigenerazione in blocco la spegnerebbe in silenzio — gotcha **#42**,
stessa specie di `mib_as_millis.rs`. Rotta per l'altro (il tipo perde i campi) scatta come
**`error`**, indipendente dall'oracolo. È la sola riga della tabella delle due direzioni che
appartiene a **entrambe** le specie, e sta scritto perché nessuno la conti fra le forti.

⛔ **Le due righe di catalogo si chiudono, e la terza resta PARZIALE con l'innesco scritto.**

| Riga | Stato | Le due direzioni |
|---|---|---|
| `V4` | ✅ **coperta** | *non compila*: `admission_is_not_two_ways.rs`. *Compila*: `an_admission_is_distinguishable_three_ways` in `crates/kernel/tests/arbiter_admission.rs` — una sonda **permanente**, non la mutazione 1. Il suo morso è **solo a compilazione**: vedi il riquadro qui sotto |
| `I2 · §5.3` | ✅ **coperta** | *non compila*: `revoking_a_non_preemptible_grant.rs`. *Compila*: `a_revocation_is_constructible_on_the_preemptible_side` in `crates/kernel/tests/arbiter_admission.rs` — una sonda **permanente**, non la mutazione 3. Vedi il riquadro qui sotto |
| blocco **B**, *«avviare un worker ← una concessione»* | ⚠️ **parziale** | *non compila*: `grant_has_no_constructor.rs`, con la mutazione **2b** a provarlo non vacuo. *Compila* — cioè *«con la concessione → compila»* — **non è scrivibile oggi**: nessuno emette concessioni finché non arriva `admit`. ⛔ **Innesco:** si chiude al compito che porta `admit` (Task 5), che è l'unico modo di ottenere la contro-sonda |

⛔ **LE SECONDE DIREZIONI DI `V4` E DI `I2 · §5.3` SONO SONDE E NON MUTAZIONI, e la differenza
è il motivo per cui esiste un file nuovo.** Una mutazione **sparisce quando la revochi**: una
direzione tenuta da qualcosa che non resta non è tenuta (§7.1.1 regola 3). ⚠️ **`V4` è stata
aggiunta in un compito di rifinitura successivo al Task 4** — errata **E16** del piano — perché
alla chiusura del Task 4 quella direzione era ancora tenuta dalla sola mutazione 1, reversibile
come tutte le altre. Stanno entrambe in `crates/kernel/tests/arbiter_admission.rs`, **due
test**:

| Sonda | Cosa tiene |
|---|---|
| `an_admission_is_distinguishable_three_ways` | la contro-sonda del catalogo per `V4` — *«distinguere le tre compila»*. Costruisce `Admission::Refused { asked, ceiling }` — l'unica via costruibile da fuori la crate: `Granted` porta un `Grant` senza costruttore pubblico, `Queued` un `TicketId` dal campo privato — e la fa passare per un `match` che nomina tutte e tre le varianti. ⚠️ **Il suo morso è SOLO a compilazione**, dichiarato così e non scoperto dopo: vedi la nota sotto la tabella delle mutazioni |
| `a_revocation_is_constructible_on_the_preemptible_side` | la contro-sonda del catalogo per `I2 · §5.3` — *«costruibile per uno prelazionabile»*. Costruisce `Activity::Preemptible(PreemptibleState::Revoking { deadline })` e lo distingue **sia** da `Preemptible(Running)` **sia** da `NonPreemptible`. ⚠️ `assert_ne!` è lecito qui e **non** viola `R2`: quella restrizione riguarda `Admission`, che porta un `Grant` e quindi non ha né `Debug` né `PartialEq`; `Activity` li deriva entrambi. ⚠️ **Il suo morso a runtime è apparente, non reale — vedi la correzione sotto la tabella delle mutazioni** |

⚠️ **Il file è NUOVO e arriva un compito prima di dove il piano lo colloca** — la tabella dei
file lo assegna ai Task 5–7 (errata **E15**). ⛔ **Non è finito in
`crates/kernel/tests/arbiter_resource.rs`**, e la separazione è per soggetto: lì vive il
vocabolario della **risorsa**, e `Activity` non è una risorsa — è ciò che una concessione
**sta facendo**. ⛔ **Chi esegue il Task 5 AGGIUNGE a quel file**: il suo *«Create»* è un
`Modify`, e il commento di modulo dettato là va **fuso**, non sovrascritto. L'avvertimento è
scritto **dentro il sorgente** oltre che qui.

✅ **E le due sonde sono state provate in negativo PRIMA di crederle** — una mutazione per
ciascuna, applicata al CODICE DI PRODUZIONE (mai al test), provata entrata con `grep -c` e
revocata con lo strumento di edit:

| Sonda | Mutazione, sul codice di produzione | Misurato |
|---|---|---|
| `an_admission_is_distinguishable_three_ways` | la variante `Queued(TicketId)` di `Admission` rinominata `Waiting(TicketId)`, in `crates/kernel/src/arbiter/mod.rs` | ✅ **rossa a compilazione** — `` error[E0599]: no variant or associated item named `Queued` found for enum `Admission` `` |
| `a_revocation_is_constructible_on_the_preemptible_side` | `Activity` appiattita in `NonPreemptible(PreemptibleState)` (la mutazione 3 di nuovo) | ✅ **rossa a compilazione** — `E0308` più `` error[E0277]: `fn(PreemptibleState) -> Activity {NonPreemptible}` doesn't implement `Debug` ``: la variante non è più un valore |

⛔ **CORREZIONE, compito di rifinitura del 2026-08-19 (errata E17 del piano): questa tabella
diceva anche *«`Running` costruito al posto di `Revoking { .. }` → rossa a RUNTIME»*, ed era la
sonda sbagliata mutata.** Quella riga mutava il LETTERALE DEL TEST (`Running` scritto al posto
di `Revoking { .. }` dentro `arbiter_admission.rs`), non il codice di produzione — prova che il
letterale conta, non che la sonda sorvegli il codice, la stessa specie di errore che il
gotcha #42 registra per `mib_as_millis.rs`. **Le due `assert_ne!` di
`a_revocation_is_constructible_on_the_preemptible_side` NON POSSONO fallire a runtime,
qualunque cosa faccia il codice di produzione, finché la `derive(PartialEq)` di `Activity`
resta:** confrontano varianti STRUTTURALMENTE distinte (`Preemptible(Revoking{..})` contro
`Preemptible(Running)`, e contro `NonPreemptible`), e con `PartialEq` derivato due varianti
diverse non sono mai uguali — stessa specie di `E5`, *«non può fallire a runtime, qualunque
cosa faccia il codice di produzione»*. ⚠️ **La forza reale e sufficiente della sonda è alla
COSTRUZIONE**, scritta da fuori la crate: `Activity::Preemptible(PreemptibleState::Revoking {
deadline })` compila solo se la nidificazione di §5.3 punto 3 esiste con quella forma esatta —
è quella, insieme al `match` esaustivo dell'altra sonda, a chiudere le due righe di catalogo.
Nessuna modifica al test: le due `assert_ne!` restano, perché documentano che `revoking` porta
davvero il valore che il test intende — un `PartialEq` derivato le rende infallibili, non
inutili.

⚠️ **Conseguenza sul conteggio dei test, e i numeri si tengono SEPARATI perché provano cose
diverse.** Dopo lo **spostamento di modulo**: **33 target, 202 passati** — identico alla
baseline, ed è quel numero a provare che lo spostamento ha solo spostato. Dopo la **sonda di
`I2 · §5.3`**: **34 target, 203 passati** — un target e un test in più, **di proposito**. Dopo
la **sonda di `V4`, in questo compito di rifinitura**: ancora **34 target** (stesso file, nessun
target nuovo), **204 passati** — un test in più. Misurato con `cargo test --locked --workspace`:
**34 target, 204 passati, 0 falliti, 2 ignorati**. Confonderli farebbe sparire la prova.

⚠️ **Un WARNING resta acceso, e il campo RESTA — decisione del proprietario del 2026-08-19.**
`cargo build --locked --workspace` stampa `` warning: field `id` is never read `` su
`crates/kernel/src/arbiter/mod.rs`, una sola. ⛔ **Il campo non è speculativo:** il suo lettore
è `Arbiter::release` (decisioni **D2** e **D3** del disegno), che deve poter rifiutare una
concessione emessa da un **altro** arbitro — senza identità non è esprimibile — e arriva al
**Task 5**, cioè a un compito di distanza dentro lo stesso traguardo. ⚠️ **Ciò che il vecchio
commento di `Grant(())` rifiutò non era il campo: era `#[allow(dead_code)]`**, un divieto
spento in permanenza. Qui non si spegne niente — nessun `allow`, nessun lettore inventato,
nessun accessore — e l'avviso **si vede**. Il cancello resta **verde**: non passa
`-D warnings`. ⏳ **E la riga porta la propria scadenza: al Task 5 quell'avviso DEVE sparire**,
perché `release` legge il campo. Se è ancora lì, il campo non serviva.

#### `Parameters::total_vram` e l'arbitro che ammette e rilascia — Traguardo 5, Task 5

⛔ **Il totale è CONSEGNATO, non chiesto**, e la §5.1 ci aveva speso un richiamo datato: la
formula del budget allocabile compare identica in tre documenti e **nessuno** diceva da dove
venga `total`. Interrogare la GPU è una chiamata al sistema operativo, che I3 vieta al kernel,
e nessuna delle sei famiglie di porte consegna la capacità dell'hardware. Quindi `Parameters`
guadagna un secondo campo, e **diciannove siti chiamanti** in quattro file lo scrivono sul
posto — un default in `Parameters` è precisamente ciò che §2.8.2 regola 2 vieta.

⛔ **`Arbiter` è logica pura dei propri ingressi**, con `BTreeMap` da `alloc` (`HashMap` non è
nominabile: `no_std`, gotcha #12 chiuso gratis), e `admit` **riscuote gli scaduti prima di
decidere**. Le sonde nuove stanno in `crates/kernel/tests/arbiter_admission.rs`, che passa da
**due** a **UNDICI** test — ⚠️ **l'undicesima è arrivata in revisione il 2026-08-19**,
non dal piano — ricontati eseguendo il binario, mai dedotti (gotcha #31):

| Sonda nuova | Cosa tiene |
|---|---|
| `a_grant_takes_exactly_its_reservation_out_of_the_budget` | ⛔ **l'asserzione è il NUMERO, non la variante:** *«ha concesso»* lo soddisfa un arbitro che concede tutto; ciò che dice che il budget è reale è che `allocated` si sia mosso **della prenotazione** |
| `releasing_gives_back_exactly_the_reservation` | la metà arbitro delle proprietà 2 e 3 di §5.7 — e sono **una** e non due: l'arbitro non deve sapere **chi** teneva una concessione, solo che rilasciarla rimetta la prenotazione |
| `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` | che l'`Err` di `release` sia **raggiungibile** — la superficie morta che questo repository ha tolto da `Record::encode`. ⚠️ **Con un limite dichiarato, vedi sotto** |
| `the_sum_of_the_grants_never_exceeds_the_total` | l'invariante, asserito **sul numero**: la somma di **tutte** le concessioni non supera il totale, e la terza richiesta torna `Refused` coi due numeri che design/02 chiede |
| `a_total_smaller_than_the_two_permanent_quotas_does_not_grant_the_second_one` | ⚠️ **RINOMINATA DAL TASK 6 — qui si chiamava `…_refuses_the_second_one`, e il nome è aggiornato invece che datato perché un PUNTATORE a un sorgente che non esiste più non è un'istantanea, è un rimando rotto.** Ciò che diceva era: che una configurazione impossibile sia **visibile invece che silenziosa** — è la sonda che paga la divergenza del disegno dalla §5.1: con le due quote **sottratte** dal totale, un totale più piccolo della loro somma darebbe budget zero **senza una parola**. ⛔ **Al Task 6 quella proprietà è andata perduta** — la seconda quota è `Queued` e nessuno la serverà mai — e la visibilità si sposta alla radice di composizione: vedi la sezione del Task 6 |
| `an_expired_grant_does_not_stay_allocated` | la riscossione pigra, scritta perché sia **osservabile**: fra due operazioni una concessione scaduta resta nei libri — non nega niente a nessuno, **non c'è** nessuno — e al primo che guarda è già liberata (§5.7 proprietà 5) |
| `a_grant_still_inside_its_window_is_not_collected` | la contro-sonda, ed è la direzione che si salta: senza, *«riscuoti sempre tutto»* passa |
| `a_grant_is_collected_at_the_instant_its_window_closes` | ⚠️ **UNDICESIMA, e non dettata dal piano: aggiunta in revisione il 2026-08-19.** È il **confine** che le due righe qui sopra scavalcano — una guarda a `5_001`, l'altra a `4_999`, e a **`5_000` esatti** non chiedeva nessuno — quindi `>` mutato in `>=` dentro `collect_expired` sopravviveva all'intera suite, **sulla funzione che quelle due esistono per tenere**. ⛔ Tiene anche **quale** delle due semantiche sia quella scelta: con `expires_at > now` la finestra è **semiaperta**, `[inizio, scadenza)`, e a `now == expires_at` la concessione è **già riscossa** |
| `a_request_larger_than_the_total_is_refused_and_not_queued` | ⚠️ **MENO di ciò che il nome promette, e la cella riscritta il 2026-08-19 diceva le due metà che non sono sue.** ⛔ Ciò che tiene è che `32_768` contro un tetto di `8_192` torni `Refused` **coi due numeri giusti**. ⚠️ **AL TASK 5, E SOLO LÌ, a consegnarlo era la seconda guardia** — `allocated + asked > ceiling` — e non quella che il nome nomina: ✅ **misurato allora cancellando `asked > ceiling` per intero, `return` compreso — `cargo test --locked -p kernel --test arbiter_admission` → 11 passati, 0 falliti**, e la cifra **resta com'è** perché è un'istantanea del Task 5 (`E35`: una misura si registra col proprio momento). La metà **mai `Queued`** era vera **per assenza di produttori**: fino al Task 6 nessuno emetteva `Queued`. ✅ **E L'INNESCO È SCATTATO — questa cella non è più in attesa:** al Task 6 la stessa identica mutazione dà **18 passate, 1 fallita** con questa sonda **sola** a morire (mutazione **5** della sezione del Task 6, errata `E43`), quindi la guardia `asked > ceiling` **non è più sussunta** e questa sonda ne è **oggi l'unica custode**. ⚠️ **Circoscritto in revisione il 2026-08-19:** la frase causale era al **presente** e il marcatore ⏳ era rimasto **pendente** dopo che il proprio innesco era scattato |

⚠️ **Il file era del Task 4 e il suo *«Create»* era un `Modify`:** il commento di modulo è
stato **fuso** e le due sonde del Task 4 restano — altrimenti sparivano le seconde direzioni
di `V4` e di `I2 · §5.3`. Le due `use` sono state unite, non sostituite.

⛔ **Sette mutazioni, una alla volta, ciascuna compilata ed eseguita a sé, provata entrata con
`grep -c` e revocata con lo strumento di edit** (mai `git checkout --`, gotcha #48):

| # | Mutazione, sul codice di produzione | Sonda attesa morta | Misurato |
|---|---|---|---|
| **1a** | `>` → `>=` nel confronto **somma contro tetto** (`self.allocated().saturating_add(asked) > ceiling`) | `the_sum_of_the_grants_never_exceeds_the_total` | ✅ **rossa** — `assertion failed: matches!(outcome, Admission::Granted(_))`. Cadono con lei anche le sonde che riempiono il totale **esatto**. **7 passati, 3 falliti** — ⚠️ **l'elenco che questa cella nominava non è più esaustivo**: vedi la nota sotto la tabella |
| **1b** | ⚠️ **aggiunta**, non dettata: `>` → `>=` nell'**altro** confronto col tetto (`asked > ceiling`) | — | ✅ **`the_sum_of_the_grants_never_exceeds_the_total` SOPRAVVIVE**, e muoiono le altre due. **8 passati, 2 falliti** — vedi la divergenza sotto la tabella |
| **1c** | ⚠️ **aggiunta in revisione il 2026-08-19**, e non è una variante della 1b: la guardia `asked > ceiling` **cancellata per intero**, `return` compreso | — | ⛔ **NESSUNA muore — 11 passati, 0 falliti**, inclusa `a_request_larger_than_the_total_is_refused_and_not_queued`, la sola che la nomini. La guardia è **interamente sussunta** dalla seconda, e oggi **nessuna sonda la tiene**. Vedi la divergenza sotto la tabella |
| **2** | `release` restituisce `Mib::ZERO` invece di `held.reserved` | `releasing_gives_back_exactly_the_reservation` | ✅ **rossa, e sola** — `` assertion `left == right` failed / left: Mib(0) / right: Mib(6144) ``. **9 passati, 1 fallito** |
| **3** | `collect_expired` con **corpo vuoto** | `an_expired_grant_does_not_stay_allocated` | ✅ **rossa**, col proprio messaggio: *«without the collection this is Refused»*. **9 passati, 1 fallito** — ⚠️ **e l'esclusività che questa cella dichiarava è caduta**: vedi la nota sotto la tabella |
| **4** | `collect_expired` con `retain(\|_, _\| false)` | `a_grant_still_inside_its_window_is_not_collected` | ✅ **rossa**, col proprio messaggio: *«the window has not closed yet»*. Cadono con lei altre tre. **6 passati, 4 falliti** |
| **5** | `cold_start: Millis` rimesso su `ResourceProfile` | `admission_reads_cold_start.rs` | ⛔ **`mismatch`, non `error`** — `E0063` al posto di `E0609`. Vedi sotto |
| **6** | `impl Admission { pub const fn is_granted(&self) -> bool { … } }` | `admission_has_no_is_granted.rs` | ✅ **`error`** — *«Expected test case to fail to compile, but it succeeded»*, e gli altri **ventisei** restano `ok` — ⚠️ **cifra del momento della misura**, quando i casi in cartella erano **ventisette**; vedi la nota sui conteggi |
| **7** | il `profile` **tolto dalla firma** di `admit` | `admission_without_profile.rs` | ✅ **`error`** — e come effetto collaterale gli altri due casi dell'ammissione vanno `mismatch`, perché passano un profilo a una firma che non lo prende più |
| **8** | ⚠️ **aggiunta in revisione il 2026-08-19**: `>` → `>=` in `collect_expired` (`held.expires_at > now`) | `a_grant_is_collected_at_the_instant_its_window_closes` | ✅ **rossa, e SOLA su tutto il workspace** — col proprio messaggio: *«at now == expires_at the window is already shut: [start, expiry)»*. **10 passati, 1 fallito** nel file, e `cargo test --workspace --no-fail-fast --locked` non porta **nessun** altro rosso. ⛔ Prima che quella sonda esistesse questa mutazione sopravviveva alla suite **intera** |

⚠️ **I conteggi delle righe 1a–7 sono della suite di DIECI sonde**, misurati chiudendo il
Task 5; quelli di **1c** e **8** sono della suite di **undici**. Dichiarato invece che riallineato:
sono misure fatte in un momento, e l'undicesima sonda è arrivata dopo, il 2026-08-19. Le righe
**1a**, **3** e **4** toccano un tetto riempito **esatto** o la riscossione, quindi la sonda nuova
sposterebbe i loro numeri: **non sono state rimisurate, e il numero nuovo non è dedotto** (gotcha
**#31**).

⛔ **E a invecchiare non è solo il numero: è il QUALIFICATORE, che datare non salva** — trovato
in revisione il 2026-08-19, ed è la ragione per cui due celle qui sopra sono state **accorciate**
invece di essere riallineate. La riga **3** diceva *«rossa, e **sola**»* e la **1a** **nominava**
le due sonde che cadevano con lei: l'esclusività e l'elenco erano veri della suite di **dieci**,
e la sonda undicesima riempie anch'essa il tetto **esatto**. ⚠️ **Quante ne cadano oggi non è
scritto da nessuna parte, ed è deliberato:** rimisurare costa due mutazioni e non è stato fatto,
quindi un numero qui sarebbe **dedotto**. Ciò che resta è ciò che regge — *«la sonda attesa
muore»* — e l'esclusività della riga **3** va **riconquistata rimisurando**, non riscrivendo.
📌 Un conteggio stantio si vede; un *«e sola»* stantio si legge come una garanzia. È il gotcha
**#31** su un aggettivo invece che su una cifra.

⛔ **Le mutazioni 3 e 4 sono ENTRAMBE necessarie, ed è la ragione per cui esistono due sonde:**
la 3 da sola sarebbe soddisfatta da *«riscuoti sempre tutto»*, che è il difetto opposto e non
meno grave — la 4 lo dimostra uccidendo la contro-sonda.

⚠️ **DIVERGENZA, misurata e non dedotta: «il confronto col tetto» ne nomina UNO e ce ne sono
DUE.** Il piano detta *«`>` diventa `>=` nel confronto col tetto»* attendendo la morte di
`the_sum_of_the_grants_never_exceeds_the_total`, e solo il confronto **somma contro tetto** la
uccide; l'altro — `asked > ceiling`, la guardia *«più grande dell'intera macchina»* — la lascia
**verde** e ne uccide altre due. Misurate entrambe perché una campagna che si ferma alla prima
avrebbe concluso *«mutazione applicata, sonda morta, fatto»* senza sapere quale delle due
guardie aveva toccato.

⛔ **E LA CONCLUSIONE ERA PIÙ FORTE DELLA MISURA, corretta il 2026-08-19.** Questa riga
chiudeva con *«Nessuna delle due è vacua, ed è quello il risultato»*, e la mutazione `>` → `>=`
non lo prova: dice che **il confronto** morde, non che **la guardia** sia portante. ✅ **Misurato
dopo, cancellando `asked > ceiling` per intero** — riga **1c** della tabella: **11 passati, 0
falliti**, inclusa `a_request_larger_than_the_total_is_refused_and_not_queued`, la sola che la
nomini. La guardia è **interamente sussunta** dalla seconda: se `asked > ceiling`, allora
`allocated + asked > ceiling` è vero **a maggior ragione**, e il valore restituito è **identico**.
⛔ **E resta lo stesso**, perché è dettata dal piano ed è **anticipatoria**: al **Task 6** il ramo
alternativo diventa `Queued`, e da lì in poi le due guardie rispondono **diverso**. Scritto
invece di lasciarla contare fra le portanti — è la specie di `E17`.

⛔ **La mutazione 5 misura la SPECIE della guardia di `Q8 · §5.2.1`, e la specie va scritta
perché nessuno la conti fra le forti.** Rimettere `cold_start` su `ResourceProfile` **non** fa
compilare il caso: il letterale del caso resta senza quel campo, quindi rustc passa da `E0609`
(*«no field `cold_start`»*) a `E0063` (*«missing field `cold_start` in initializer»*) e
trybuild dice **`mismatch`**. ⚠️ Cioè questa guardia **dipende dall'oracolo** — stessa specie di
`mib_as_millis.rs`, gotcha **#42** — e una rigenerazione in blocco la spegnerebbe in silenzio.
È il prezzo della forma di questo caso, non un difetto scoperto dopo: registrato qui e nella
tabella delle due direzioni.

⚠️ **E IL COMMENTO DEL CASO ACCREDITAVA LA CHIAMATA DELL'ERRORE, corretto il 2026-08-19.**
`crates/kernel/tests/compile_fail/admission_reads_cold_start.rs` diceva che il profilo è
costruito *«e poi passato ad `admit` nello stesso `main`, e QUELLO è il punto»*. L'`E0609` nasce
invece dal **letterale** e dall'**accesso al campo**, e la chiamata **non partecipa**:
✅ **misurato cancellandola** su una crate usa-e-getta fuori dal repository — stesso
`error[E0609]`, stessa nota coi quattro campi disponibili — e l'oracolo accanto al caso nomina
**un** errore e **una** riga, che è quella dell'accesso al campo. Ciò che la chiamata compra è
un **accoppiamento alla firma** di grado `mismatch` e non di grado `error`, che è quanto la
tabella delle due direzioni diceva già giusto. ⛔ **Il commento è stato riscritto a parità di
righe**, e non è pignoleria: l'oracolo pinza il **numero di riga** dell'accesso al campo, quindi
allungare il commento avrebbe reso il caso `mismatch` e costretto a rigenerare uno `.stderr` che
non aveva nessuna ragione di cambiare.

⛔ **Le due righe di catalogo che il Task 5 chiude, più quella che gli inneschi gli hanno
consegnato.**

| Riga | Stato | Le due direzioni |
|---|---|---|
| `Q8 · §5.2.1` | ✅ **coperta** (era parziale) | *non compila*: `admission_reads_cold_start.rs`, che ora costruisce il profilo **e lo passa ad `admit`**. *Compila*: `cold_start_is_readable_outside_the_decision_path` in `crates/kernel/tests/arbiter_resource.rs`, dal Task 3 |
| `V2` | ✅ **coperta** (era scoperta) | *non compila*: `admission_without_profile.rs` (`E0061`, *«argument #1 of type `&ResourceProfile` is missing»*). *Compila*: **tutte e nove** le sonde nuove di `arbiter_admission.rs`, che `admit` lo chiamano **col** profilo, da fuori la crate — ⚠️ **erano otto** fino alla revisione del 2026-08-19, che ha portato la nona |
| blocco **B**, *«avviare un worker ← una concessione»* | ✅ **coperta** (era parziale) | *non compila*: `grant_has_no_constructor.rs`, con la mutazione **2b** del Task 4 a provarlo non vacuo. *Compila* — cioè *«con la concessione → compila»* — **ora è scrivibile**, ed è scritta: `releasing_gives_back_exactly_the_reservation` ottiene un `Grant` da `admit` e lo consuma. ⚠️ **Il consumatore vero è `Process::start`, che resta del Traguardo 6**: ciò che questa direzione compra oggi è che una concessione **si ottiene solo da `admit`** ed è spendibile una volta sola |
| `V4` | ✅ **coperta**, e ora con **due** casi | *non compila*: `admission_is_not_two_ways.rs` (`E0004`) **e** `admission_has_no_is_granted.rs` (`E0599`) — la seconda metà, che aspettava un'ammissione vera da cui ottenere un `Admission`. *Compila*: `an_admission_is_distinguishable_three_ways`, dal Task 4 |

⚠️ **VOCE APERTA — ciò che `release` compra davvero, e ciò che non compra.** La sonda
`a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` costruisce un
secondo arbitro **vuoto**, quindi prova *«non è nei miei libri»*, **non** *«distinguo le mie
concessioni da quelle altrui»*. `GrantId` è un progressivo che riparte da zero per ogni
`Arbiter`: due arbitri che abbiano **entrambi** emesso concessioni condividono lo spazio degli
id, e il secondo accrediterebbe la concessione del primo. ⛔ **Il disegno non è stato cambiato
per chiudere il buco:** dare un'**identità** all'arbitro è una decisione del proprietario. Ciò
che protegge oggi è che un processo ha **un** arbitro — i diversi che esistono insieme esistono
nei **banchi**. Il limite è scritto **accanto a `ReleaseError` nel sorgente**, oltre che qui.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo
a questo file**, che dice anche **chi la chiude** — rimando aggiunto il 2026-08-25.

⚠️ **VOCE APERTA — `release` risponde `UnknownGrant` anche a una concessione PROPRIA ma
SCADUTA, e il nome della variante afferma il falso.** `release` chiama `collect_expired`
**prima** di cercare, quindi una concessione con `expires_at <= now` è già stata tolta dai libri:
`held.remove` dà `None` e si esce con `Err(ReleaseError::UnknownGrant)`, il cui doc diceva *«This
arbiter never issued that grant»* — che del caso scaduto è **falso**. Il chiamante non può
distinguere *«non è mia»* da *«era mia ed è scaduta»*. ✅ **Misurato il 2026-08-19 su una sonda
usa-e-getta fuori dal repository**, non dedotto: ammessa per `5_000` ms e rilasciata a `5_001` →
`Err(UnknownGrant)`; a `4_999` → `Ok(Mib(4096))`; a `5_000` **esatti** → `Err(UnknownGrant)`
anche lì, perché la finestra è **semiaperta**. ⛔ **Le due scelte che lo producono sono dettate
dal Passo 4 del piano** — la riscossione prima della ricerca, e una variante sola — quindi il
disegno **non è stato cambiato**: la conflazione è **dichiarata** accanto a `ReleaseError` nel
sorgente. ⛔ **RICHIAMO DEL 2026-08-28 — LA SCELTA NON È PIÙ «REGISTRATA E NON PRESA»: È PRESA
NEL MERITO E VINCOLATA NELLA FORMA, e la frase è RISCRITTA e non affiancata** — una frase vera
appesa sotto una falsa lascia in piedi la falsa, finding **A-2**. `release` **non** risponde
`Err` a una concessione **propria**: finestra scaduta e grazia scaduta non sono fallimenti del
rilascio, e solo la concessione **altrui** resta un errore. Le **due** forme scartate, col perché
e coi costi rimisurati prima di decidere, stanno accanto a `ReleaseError` in
`crates/kernel/src/arbiter/mod.rs`, in una casa sola. ⚖️ **Resta il tipo esatto della risposta**,
che si disegna **insieme a `R6`** al Traguardo 6, perché discende da ciò che `Worker::kill`
chiede quando restituisce la concessione — e quel chiamante non esiste ancora (gotcha **#46** dal
verso sbagliato). ⏳ **Comincia a costare** proprio lì, dove la concessione torna a lavoro
**finito**, che può benissimo cadere dopo la finestra, e *«il rilascio è fallito»* e *«era già
stato fatto per te»* sono notizie diverse.
⛔ **E QUESTA VOCE PORTAVA L'ESCLUSIVITÀ FALSA CHE `AUD-014` HA TOLTO DAL SORGENTE, in una casa
che quel rimedio non toccava:** diceva *«`release` ha **due** chiamanti in tutto il repository,
entrambi in `crates/kernel/tests/arbiter_admission.rs`»*, ed **entrambe** le clausole erano false
— il Task 6 falsificò la cifra nel commit successivo, e il Task 12 la sede, facendo nascere un
chiamante in **un'altra crate**. ⛔ **TOLTE e non riallineate**, che è la cura di `AUD-014`:
resta *«nessun consumatore di produzione esiste»*, la metà che non marcisce, e il censimento lo
rifà `grep -rn '\.release(' crates/ --include=*.rs`.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo
a questo file**, che dice anche **chi la chiude** — rimando aggiunto il 2026-08-25.

⏳ **LA SCADENZA DEL TASK 4 È RISPETTATA, e questa riga esiste per renderlo verificabile.** La
sezione del Task 4 dichiarava: *«al Task 5 quell'avviso DEVE sparire, perché `release` legge il
campo; se è ancora lì, il campo non serviva»*. ✅ Misurato dopo il Task 5:
`cargo build --locked --workspace` stampa **ZERO warning**. `Grant::id` ha il suo lettore, ed è
`Arbiter::release`.

⛔ **`Held` nasce con DUE campi e non con quattro, e non è una semplificazione: è la stessa
regola applicata due volte.** Il piano gli dà anche `lane: ComputeClass` e `activity: Activity`;
nessuno dei due ha un lettore in questo compito, quindi sarebbero stati **due** warning
`dead_code` — e la scadenza qui sopra sarebbe stata falsificata dal compito stesso. Il piano
prescriveva l'uscita per `lane` (*«si sposta il campo al compito dove nasce col proprio
consumatore»*) e la stessa uscita vale per `activity` sugli stessi fatti: `lane` arriva col
**Task 6**, quando una coda deve sapere in che corsia aspetta; `activity` col **Task 7**, che è
il primo a leggere che cosa una concessione stia **facendo**. ⛔ **Nessun `#[allow]`**, che è un
divieto spento (gotcha #13). Conseguenza sul corpo di `admit`: sparisce anche il blocco
`activity: match profile.preemption { … }`, che esisteva solo per riempire quel campo.

⚠️ **Il costo di un secondo parametro consegnato, misurato invece che temuto.** §2.8.5 promette
che aggiungere un parametro **rompe ogni chiamante**, e la promessa arriva fino all'oracolo:
`parameters_have_no_default.stderr` è andato **`mismatch`**, perché rustc chiude quell'errore
con una nota che cita la firma di `Parameters::new` **verbatim**. La rigenerazione è
**legittima e prevista** — il commento dentro il caso la descriveva già — ed è stata fatta per
la via documentata: cancellato l'oracolo stantio, ri-eseguito, `diff -u` del vecchio contro
quello in `wip/`, spostato **a mano**. ⛔ **Mai `TRYBUILD=overwrite`**, che avrebbe portato via
gli altri **ventisette** — ⚠️ **cifra del momento della misura**, col caso nuovo già in
cartella, cioè **ventotto** in tutto (gotcha #25). Il diff letto è di **due righe**: la firma e
la sua sottolineatura. La regola che quel caso difende scatta comunque come `error` e **non**
attraverso l'oracolo, quindi la rigenerazione non disarma niente.

⚠️ **`parameters_delivered.rs` passa da quattro a CINQUE test**, e il quinto è la metà che
mancava: `the_constructor_substitutes_nothing_for_the_total_it_is_handed`. La regola di §2.8.4
è sul **costruttore**, e un costruttore che lascia stare un campo e mette un pavimento
sull'altro soddisfa la sonda vecchia e viola la regola. `Mib::ZERO` e `Mib::new(u64::MAX)`
tornano indietro **identici**. ⚠️ **E `parameters_are_comparable_so_a_substitution_is_observable`
ha guadagnato una riga** — due `Parameters` che differiscono **nel solo totale** — senza la
quale una comparazione che guardasse il solo `executor_turn_limit` avrebbe passato ogni sonda
del file, e sostituire un totale sarebbe stato inosservabile.

📌 **Conteggi, ricontati eseguendo il binario e mai dedotti** (gotcha #31): workspace **34
target, 214 passati, 0 falliti, 2 ignorati** — erano **204** prima del compito e **213** alla sua
chiusura; il duecentoquattordicesimo è la sonda del **confine della scadenza**, aggiunta in
revisione il 2026-08-19. Gli altri nove sono le otto sonde dell'ammissione più quella del totale.
Nessun target nuovo: il file dell'ammissione esisteva già. Per file: `arbiter_admission.rs`
**undici**, `parameters_delivered.rs` **cinque**, `executor_determinism.rs` **tredici**
(invariato), `arbiter_resource.rs` **otto** (invariato), `dst_campaign.rs` **cinque** di cui uno
`#[ignore]` (invariato), e i casi `compile_fail` da **ventisei** a **VENTOTTO** — ✅ **ricontati
il 2026-08-19 col comando e non a memoria**, `ls crates/kernel/tests/compile_fail/*.rs | wc -l`
→ **28**. ⚠️ **È la cifra che le due righe qui sopra datano invece di riscrivere:** *«gli altri
ventisei»* e *«gli altri ventisette»* sono due istantanee di un contenitore che cresceva dentro
lo stesso compito, non due affermazioni in contraddizione.

#### Le code per corsia — Traguardo 5, Task 6, e `Admission::Queued` acquista un produttore

⛔ **Per corsia e non FIFO globale, ed è una misura e non un gusto:** §5.3.1 dice che i numeri
di **M-7** restano validi **come limite superiore** proprio perché la versione specificata
tiene l'ordine **per corsia**. Una coda unica riordinata a ogni rilascio invaliderebbe quella
misura, e allora andrebbe rifatta.

⚠️ **E L'ORDINE PER CORSIA È UNA PROPRIETÀ DI `promote`, NON DELL'ARBITRO — circoscritto in
revisione il 2026-08-19.** `Arbiter::admit` non guarda mai `queues`, quindi una richiesta che
arriva **dopo** scavalca chi è in coda; e `promote` cade sulla corsia successiva quando la sua
si ferma, quindi fra corsie serve la piccola peggiore prima della grande migliore. Entrambe
**misurate**, entrambe **registrate e non prese**: le due voci aperte in fondo a questa sezione.

⛔ **`Queued` esisteva dal Task 4 e non lo emetteva nessuno.** Il Task 6 gli dà il produttore:
in `admit` il ramo *«entra nella macchina ma non in questo momento»* **accoda** invece di
rifiutare, e `promote` serve la coda con la stanza che c'è. La costruzione della concessione è
estratta in un aiutante privato — `Arbiter::issue` — che `admit` e `promote` **condividono**:
un secondo posto in cui si costruisce un `Grant` sarebbe il secondo modo di ottenerne uno,
cioè ciò che §5.6 esiste per togliere.

⛔ **TRE SONDE DEL TASK 5 SONO STATE RISCRITTE, e il piano non lo diceva.** Il ramo nuovo
cambia la risposta a *«entra nella macchina ma non adesso»* da `Refused` a `Queued`, e tre
sonde andavano in panico dentro il proprio `let … else`. ✅ **Viste rosse prima di scrivere il
codice di produzione**, e ciascuna col proprio messaggio — **8 passate, 3 fallite** sul file di
allora. ⛔ **Nessuna è stata annacquata:** in tutte e tre l'asserzione che porta la proprietà è
su `allocated()`, che è il numero, e si è spostata solo l'asserzione sulla **variante**.

| Sonda riscritta | Che cosa è cambiato, e che cosa NON è cambiato |
|---|---|
| `the_sum_of_the_grants_never_exceeds_the_total` | la terza richiesta è `Queued` invece che `Refused`, quindi i **due numeri** di `Refused` non sono più pinzati qui — restano pinzati da `a_request_larger_than_the_total_is_refused_and_not_queued`. ⛔ **Ciò che tiene l'invariante non si è mosso:** `allocated()` fermo a `8_192`, cioè *«non è stato sovra-ammesso niente»* |
| `a_total_smaller_than_the_two_permanent_quotas_does_not_grant_the_second_one` | ⛔ **RINOMINATA — si chiamava `…_refuses_the_second_one`, e quel nome oggi afferma il falso:** la seconda quota non è rifiutata, è **accodata**. Vedi la voce aperta qui sotto: questa sonda ha **perso** la proprietà per cui era stata scritta. Ciò che tiene ancora è che la seconda quota **non prenda** VRAM che la macchina non ha — `allocated()` fermo a `1_024` |
| `a_grant_still_inside_its_window_is_not_collected` | la seconda richiesta è `Queued`. ⚠️ **A tenere la direzione della riscossione è il `let … else`, non il numero**, ed è scritto accanto alla sonda: con *«riscuoti sempre tutto»* la seconda sarebbe `Granted` e il `let … else` andrebbe in panico, mentre `allocated()` leggerebbe `4_096` **lo stesso**. Il numero coglie l'altro difetto — una richiesta accodata che **prenota** comunque |

⛔ **Otto sonde nuove**, di cui **cinque** dettate dal piano e **tre** no. Le tre in più esistono
perché una regola scritta in un commento e tenuta da niente è un'intenzione (gotcha **#42**), e
§7.1.4 vuole **due direzioni** per regola:

| Sonda nuova | Cosa tiene |
|---|---|
| `a_request_that_fits_the_machine_but_not_the_moment_is_queued` | il produttore di `Queued`: una richiesta che **non entra adesso ma potrebbe entrare dopo** è accodata e non rifiutata, e dopo il rilascio `promote` la serve col **suo** biglietto |
| `the_queue_promotes_by_lane_and_not_in_arrival_order` | ⛔ **l'asserzione che tiene validi i numeri di M-7 PER `promote`** — e non per l'arbitro intero: vedi la voce aperta sull'ammissione più sotto. È tutta la ragione per cui la coda è per corsia. ⚠️ **RISCRITTA IN REVISIONE IL 2026-08-19: erano DUE corsie, adesso sono TRE.** Le tre attese arrivano nell'ordine peggiore che ci sia — `Batch`, poi `Interactive`, poi `Realtime` — ed escono **rovesciate**: due entrano nella stanza liberata, `Batch` resta ad aspettare. Una FIFO globale la manderebbe rossa. ⛔ **Perché è stata riscritta, ed è una misura:** con due corsie sole **nessuna sonda dell'intero workspace promuoveva mai un'attesa in corsia `Realtime`** — mutazione **1c** qui sotto — cioè *«prima la corsia migliore»* era provato sulla **seconda** migliore, che è proprio la corsia di cui parlano §5.3.1, M-7 e le quote permanenti di ADR-0033 |
| `inside_one_lane_the_order_is_the_order_of_arrival` | che la regola qui sopra **non** sia *«un ordine qualsiasi»*: dentro **una** corsia l'ordine è quello d'arrivo |
| `promote_with_no_room_freed_promotes_nothing` | la contro-sonda: `promote` **non** è *«concedi tutto quello che c'è in coda»*. Senza stanza liberata non promuove niente e i libri non si muovono |
| `a_promoted_grant_is_a_grant_like_any_other` | che ciò che esce dalla coda si **rilasci** e renda esattamente la prenotazione — senza, la coda potrebbe emettere concessioni che i libri non hanno mai imparato |
| `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one` | ⚠️ **NON dettata dal piano.** È la regola che il doc di `promote` **dichiara** — *«si ferma alla prima richiesta che non entra, dentro una corsia»* — e che nessuna delle cinque dettate esercitava. La stanza liberata è **esattamente** quella della piccola, che è **servibile** e **non viene servita** perché la grande le sta davanti. ⚠️ **RICHIAMO DEL 2026-08-19, IN REVISIONE:** *«un `promote` che scavalcasse passava tutte e cinque»* era scritto come un **fatto** e non era mai stato misurato — al suo posto la campagna metteva la mutazione **3**, che ne uccide quattro e quindi non isola niente. ✅ **Adesso è misurato, con la mutazione dello scavalcamento vero (riga 3d): rossa, e SOLA — 18 passate, 1 fallita.** La frase resta perché è risultata **vera**, non perché era scritta |
| `promote_collects_the_expired_before_it_serves_the_queue` | ⚠️ **NON dettata dal piano.** *«L'arbitro riscuote prima di decidere»* è una proprietà di **ogni** operazione — è il motivo per cui `collect_expired` è privata — e **al Task 6, quando questa sonda è nata**, con `promote` le operazioni erano **tre**, di cui solo due coperte. ⚠️ **CIRCOSCRITTA IL 2026-08-20 E NON RIALLINEATA** (`E73`, la forma di `E54`): dal **Task 7** sono **quattro** — `ask_back` è la quarta, e la sezione di quel compito lo dice — e la cifra di allora **resta**, perché è l'istantanea di allora e non un puntatore da ricorreggere (gotcha **#68**). ⛔ **Nessuna chiamata a `release` qui dentro:** l'unica cosa che libera la stanza è la riscossione **dentro `promote`** |
| `promote_serves_every_request_that_fits_and_not_just_the_first` | ⚠️ **NON dettata dal piano, e nata da un MUTANTE VIVO** — vedi la riga **3c** della tabella sotto. Il doc di `promote` dice *«serve la coda con la stanza che c'è»*, al **plurale**, e un `promote` che si fermasse dopo **una** promozione per corsia sopravviveva a tutte e diciotto le altre sonde. Due attese da `2_048` nella **stessa** corsia, e l'ordine è asserito su **entrambe** le posizioni: contare non basta (gotcha **#30**) |

⛔ **QUINDICI mutazioni, una alla volta, ciascuna compilata ed eseguita a sé, provata entrata con
il conteggio delle occorrenze e revocata con una copia byte-esatta presa prima** (mai
`git checkout --`, gotcha #48; nessun `sed -i`, i fine-riga di entrambi i sorgenti sono
rimasti a **zero CR**). ✅ **Tutti i conteggi sono della suite di DICIANNOVE sonde**, cioè di
**una sola istantanea**: la sonda 3c è arrivata a metà campagna e le mutazioni già misurate
sono state **rimisurate tutte da capo** invece di essere datate, così nessuna cella qui porta
un numero di una suite diversa (gotcha **#31**).

⛔ **ERANO DIECI, E LA REVISIONE DEL 2026-08-19 NE HA AGGIUNTE TRE — la `1c`, la `3d` e la `3e` —
POI HA RIMISURATO TUTTE E DIECI LE ALTRE DA CAPO.** La ragione della rimisura non è lo scrupolo: la
sonda `the_queue_promotes_by_lane_and_not_in_arrival_order` è stata **riscritta** in quella
stessa passata da due corsie a tre, e ogni cella qui sotto è un conteggio **di quella suite**.
✅ **Rimisurate tutte e dieci: nessuna cifra si è mossa**, né i totali né quali sonde muoiono —
e il fatto si scrive perché è una **misura** e non un'aspettativa: la riscrittura è costruita
apposta perché **esattamente due** delle tre attese entrino nella stanza liberata, così la
sonda continua a morire dove moriva (la **3** e la **3b** la uccidono anche adesso) invece di
scivolare fuori dalla loro portata. La suite resta di **diciannove** sonde: la sonda delle
corsie è stata riscritta, non affiancata.

⛔ **E LA SECONDA REVISIONE DELLO STESSO GIORNO NE HA AGGIUNTE ALTRE DUE — la `1d` e la `4b`
— ed è il caso dirlo subito: sono VIVE tutte e due, e non è un difetto rimasto aperto.**
Sono le mutazioni delle voci aperte **dello scavalcamento fra corsie** (`E50`) e di **`admit` che
non consulta la coda** (`E51`) — la seconda e la terza delle **quattro** che questa sezione porta,
non *«le due in fondo»* come diceva questa riga: ciascuna esercita una frase
che un doc **afferma** e che nessuna sonda tiene, e pinzarle congelerebbe la politica che quelle
voci chiedono al **proprietario** di scegliere. ✅ **Misurate di persona e non copiate dal
rilievo che le ha segnalate**, con le cifre nelle due righe; la scelta di **dichiarare invece
che pinzare** è la voce `E53`, e il precedente è `E39`. La campagna passa da **tredici** a
**quindici**, e le due righe stanno qui perché il registro vuole **ogni** mutazione col proprio
esito misurato — anche quando l'esito è *«non uccide niente, ed è voluto»*.

| # | Mutazione, sul codice di produzione | Sonda attesa morta | Misurato |
|---|---|---|---|
| **1** | in `promote`, le corsie percorse **al contrario** (`values_mut()` → `values_mut().rev()`) | `the_queue_promotes_by_lane_and_not_in_arrival_order` | ✅ **rossa, e sola** — **18 passate, 1 fallita** |
| **1b** | ⚠️ **aggiunta**, non dettata: la **chiave** `ComputeClass::priority` (`Realtime => 2`, `Batch => 0`) | — | ✅ **rossa la stessa**, e con lei le due di `arbiter_resource.rs` (`the_lane_order_is_pinned_by_name_and_realtime_comes_first`, `the_three_lanes_are_distinct_and_the_ordering_is_total`). **18/1** e **6/2**. ⛔ È la prova che l'ordine di `promote` viene **davvero** dalla chiave e non da un elenco scritto due volte. ✅ **Rimisurata in revisione il 2026-08-19 sulla sonda a tre corsie: invariata** |
| **1c** | ⛔ **AGGIUNTA IN REVISIONE IL 2026-08-19, ed è quella che ha trovato il buco più grande del compito:** dentro il ciclo delle corsie di `promote`, `if *lane_key == ComputeClass::Realtime { continue; }` — la corsia **migliore** saltata per intero | `the_queue_promotes_by_lane_and_not_in_arrival_order` | ⛔ **PRIMA della riscrittura a tre corsie: NESSUNA moriva.** ✅ Misurato sul codice di `47941dd`: **34 target, 222 passate, 0 fallite, 2 ignorate** — il mutante era **vivo nell'intero workspace**, e *«prima la corsia migliore»* era provato sulla sola `Interactive`. ✅ **Dopo la riscrittura: rossa, e SOLA — 18 passate, 1 fallita** |
| **1d** | ⛔ **AGGIUNTA NELLA SECONDA REVISIONE DEL 2026-08-19, e NON uccide niente apposta:** l'**intera passata** fermata alla prima corsia la cui testa non entra — una bandiera sul `break` interno, poi `break` sul ciclo esterno — cioè **nessuna caduta** sulla corsia successiva | — | ⛔ **NESSUNA muore: `arbiter_admission.rs` 19 passate, 0 fallite, e il workspace 34 target, 222 passate, 0 fallite, 2 ignorate.** Mutante **vivo**, e **non pinzato apposta**: è il comportamento che la voce aperta dello **scavalcamento fra corsie** (`E50`) mette davanti al proprietario. La frase del doc di `promote` è ora **dichiarata non tenuta** invece che pinzata — `E53` |
| **2** | in `promote`, `remove(0)` → prendere la **coda** della corsia (`remove(len() - 1)`, cioè `pop`) | `inside_one_lane_the_order_is_the_order_of_arrival` | ✅ **rossa**, e cade con lei `promote_serves_every_request_that_fits_and_not_just_the_first`, che asserisce l'ordine su entrambe le posizioni. **17 passate, 2 fallite** |
| **3** | in `promote`, il controllo `saturating_add(asked) > ceiling` **cancellato** | `promote_with_no_room_freed_promotes_nothing` **e** `the_sum_of_the_grants_never_exceeds_the_total` | ⚠️ **la prima muore, la seconda NO** — vedi la divergenza sotto. Muoiono **quattro**: `promote_with_no_room_freed_promotes_nothing`, `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one`, `the_queue_promotes_by_lane_and_not_in_arrival_order`, `inside_one_lane_the_order_is_the_order_of_arrival`. **15 passate, 4 fallite** |
| **3b** | ⚠️ **aggiunta** — la seconda direzione sul **valore**: lo stesso controllo `>` → `>=` | — | ✅ **quattro rosse**: `promote_collects_the_expired_before_it_serves_the_queue`, `promote_serves_every_request_that_fits_and_not_just_the_first`, `inside_one_lane_the_order_is_the_order_of_arrival`, `the_queue_promotes_by_lane_and_not_in_arrival_order`. **15/4**. Una promozione che riempie il tetto **esatto** verrebbe rifiutata |
| **3c** | ⚠️ **aggiunta**, ed è quella che ha trovato il buco: `promote` promuove **una sola** voce per corsia (`break` dopo la prima) | — | ⛔ **NESSUNA moriva — 19 passate, 0 fallite**: mutante **vivo**. Sonda nuova scritta per lui, `promote_serves_every_request_that_fits_and_not_just_the_first`, e **rimisurato**: ✅ **rossa, e sola — 18 passate, 1 fallita** |
| **3d** | ⛔ **AGGIUNTA IN REVISIONE IL 2026-08-19, ed è la direzione negativa che PC-2 pretendeva e che la campagna non aveva:** in `promote`, il `break` sulla prima attesa che non entra sostituito da una **scansione** della prima che entra (`queue.iter().position(…)` più `remove(index)`) — cioè lo scavalcamento vero e proprio | `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one` | ✅ **rossa, e SOLA — 18 passate, 1 fallita**, misurata sul codice **dopo** la riscrittura a tre corsie. ⛔ **Prima di questa riga quella sonda non era isolata da niente:** la registrazione le attribuiva la mutazione **3**, che ne uccide **quattro** e quindi non isola nessuna delle quattro |
| **3e** | ⚠️ **AGGIUNTA IN REVISIONE IL 2026-08-19**, cercando una mutazione che isolasse `promote_with_no_room_freed_promotes_nothing`: in `promote`, `queue.remove(0)` spostato **prima** del controllo — la richiesta che non entra viene **scartata in silenzio** invece di restare in coda | `promote_with_no_room_freed_promotes_nothing`, **prevista sola** | ⚠️ **NON isola: 16 passate, 3 fallite.** Muoiono `promote_with_no_room_freed_promotes_nothing`, `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one` e `the_queue_promotes_by_lane_and_not_in_arrival_order` — tutte e tre asseriscono `queued()` alla fine. ⛔ **La previsione era di DUE e la misura dice TRE:** registrata, non appianata |
| **4** | in `admit`, restituire `Refused` invece di accodare | `a_request_that_fits_the_machine_but_not_the_moment_is_queued` | ✅ **rossa**, e cadono con lei **dieci** altre — tutte quelle che accodano, comprese le tre riscritte. **8 passate, 11 fallite** |
| **4b** | ⛔ **AGGIUNTA NELLA SECONDA REVISIONE DEL 2026-08-19, e NON uccide niente apposta:** in `admit`, `!self.queues.is_empty() \|\|` aggiunto alla **seconda** guardia — un ritardatario si **accoda dietro** chi aspetta invece di scavalcarlo | — | ⛔ **NESSUNA muore: `arbiter_admission.rs` 19 passate, 0 fallite, e il workspace 34 target, 222 passate, 0 fallite, 2 ignorate.** Mutante **vivo**, e **non pinzato apposta**: è la scelta che la voce aperta sull'ammissione lascia al **proprietario**. La frase del doc di `admit` è ora **dichiarata non tenuta** — `E53` |
| **5** | ⚠️ **aggiunta, e la deve `E28`**: la guardia *«più grande dell'intera macchina»* (`asked > ceiling`) **cancellata per intero**, `return` compreso | `a_request_larger_than_the_total_is_refused_and_not_queued` | ✅ **rossa, e SOLA — 18 passate, 1 fallita.** ⛔ **L'attesa di `E28` si avvera:** al Task 5 la stessa identica mutazione lasciava **11 passate, 0 fallite**. La guardia non è più sussunta |
| **5b** | ⚠️ **aggiunta** — la seconda direzione sul **valore**: `asked > ceiling` → `asked >= ceiling` | — | ✅ **undici rosse — 8 passate, 11 fallite**, e **non sono le stesse undici della 4**: qui cadono le sonde che chiedono il tetto **esatto** (comprese `an_expired_grant_does_not_stay_allocated` e `a_grant_is_collected_at_the_instant_its_window_closes`), là quelle che accodano |
| **6** | ⚠️ **aggiunta**: `self.collect_expired(now)` tolta da `promote` | `promote_collects_the_expired_before_it_serves_the_queue` | ✅ **rossa, e sola — 18 passate, 1 fallita**. ⛔ Prima che quella sonda esistesse questa mutazione sopravviveva alla suite **intera** |

⛔ **La 3 uccide quattro sonde, e la domanda che ne segue è se siano in concorrenza fra loro** —
la regola nata dal Task 3 del Traguardo 4: quando una mutazione ne uccide due si cerca **una
terza che lasci passare la prima**.

⚠️ **RICONTATO IN REVISIONE IL 2026-08-19, E LA RISPOSTA DI PRIMA ERA SBAGLIATA IN DUE MODI.**
Diceva: *«Qui ce ne sono tre: la 1 uccide la sola sonda delle corsie, la 2 la sola dell'ordine
d'arrivo, la 3c la sola del «quante ne serve». Nessuna delle quattro è ridondante rispetto alle
altre»*. ① **Tre mutazioni isolanti nominate per QUATTRO sonde**, e il conto non torna.
② **Una delle tre — la 3c — isola una sonda che NON è fra le quattro:**
`promote_serves_every_request_that_fits_and_not_just_the_first` non muore sotto la 3. Il conto
vero, sonda per sonda, misurato e non dedotto:

| Sonda uccisa dalla **3** | Che cosa la **isola** |
|---|---|
| `the_queue_promotes_by_lane_and_not_in_arrival_order` | la **1**, e dal 2026-08-19 anche la **1c**: entrambe **rosse e sole**, 18/1 |
| `inside_one_lane_the_order_is_the_order_of_arrival` | la **2**: è la **sola delle quattro** a morire lì (cade con lei `promote_serves_every_request_that_fits_and_not_just_the_first`, che fra le quattro non c'è) |
| `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one` | la **3d**, **rossa e sola**, 18/1 — ⛔ **ed è ciò che la revisione ha trovato mancante:** fino al 2026-08-19 questa sonda non aveva **nessuna** mutazione che la isolasse |
| `promote_with_no_room_freed_promotes_nothing` | ⛔ **NIENTE, e si scrive invece di lasciarlo intendere.** Nessuna delle **quindici** la uccide da sola, e c'è di più: **muore solo dove muore anche la sonda dello scavalcamento** — la **3**, la **3e**, la **4** e la **5b**, quattro volte su quattro. Il suo insieme di morti è **contenuto** in quello dell'altra, cioè sotto questa campagna è **dominata** |

⛔ **Che cosa se ne fa, e che cosa NON se ne fa.** La sonda dominata **resta**, e non perché
togliere una sonda costi: perché quel che dice è un'altra cosa — è la contro-sonda di *«promote
non è concedi-tutto»*, ed è l'unica che chiami `promote` su una macchina che **non guadagna
stanza in nessun modo**, né da un rilascio né da una riscossione. ⚠️ **RISTRETTA IL
2026-08-19: prima diceva *«su una macchina esattamente piena senza che sia stato rilasciato
niente»*, e non era esatto** — anche
`promote_collects_the_expired_before_it_serves_the_queue` chiama `promote` senza nessun
`release` ed entra con i libri a `4_096`; la stanza le arriva dalla **riscossione**, e sono le
sole due del file a chiamare `promote` senza rilasciare. L'**intento** era giusto,
l'esclusività **come scritta** no — specie di `E38`, dove un qualificatore stantio si legge
come una garanzia — e il rimedio è **restringere la frase a ciò che è vero**, non aggiungere
una cifra. ⚠️ **Ma la sua non-ridondanza NON è misurata**, e la
riga di prima lo affermava; quella qui la registra come quello che è — un'intenzione finché una
mutazione non la isola.

⚠️ **UNA CANDIDATA È STATA CERCATA E MISURATA, ED È LA MUTAZIONE `3e`** — togliere l'attesa dalla
coda **prima** del controllo, cioè lo **scarto silenzioso** della richiesta che non entra. ⛔ **E
la misura ha smentito la previsione, che si registra invece di appianarla:** era stata ragionata
a **due** morti — la sonda cercata più quella delle corsie, che asserisce `queued()` alla fine —
e ne uccide **TRE**, perché muore anche quella dello scavalcamento, che asserisce `queued()` a
sua volta. ✅ **16 passate, 3 fallite.** Quindi la `3e` non isola niente, e **non rompe nemmeno
la dominanza**: uccide le due insieme, esattamente come le altre tre. Un'evidenza scritta prima
della misura è un'ipotesi, anche quando è la propria.

⚠️ **DIVERGENZA, misurata e non dedotta: la mutazione 3 NON uccide
`the_sum_of_the_grants_never_exceeds_the_total`.** Il piano se l'aspettava, ed era scritto
prima che le tre sonde del Task 5 venissero riscritte: quella sonda **non chiama `promote`** —
tiene l'invariante dal lato dell'**ammissione**, che è la sua proprietà — quindi una mutazione
dentro `promote` non può toccarla. ⛔ **La sonda non è stata allargata per far tornare
l'attesa:** allargarla le avrebbe cambiato il soggetto, e l'invariante dal lato di `promote` è
tenuto dalle **quattro** che muoiono davvero. L'attesa era sbagliata, e ciò che si registra è
la misura.

⚠️ **SECONDA DIVERGENZA, sulla terza mutazione che il piano indicava per distinguere le due
sonde.** Il piano dice che un `promote` che *«promuove una sola voce e poi esce»* lascia verde
`promote_with_no_room_freed_promotes_nothing` e **rossa**
`a_promoted_grant_is_a_grant_like_any_other`. ✅ **Misurato (riga 3c):** la prima resta verde —
giusto — ma la seconda resta verde **anche lei**, e la ragione è leggibile nel suo corpo: ha
**una** sola richiesta in attesa, quindi *«una sola»* le basta. A morire, quando la sonda è
stata scritta, è `promote_serves_every_request_that_fits_and_not_just_the_first`. La
distinzione fra gli assi **regge**; la sonda che il piano nominava per provarla **no**.

⛔ **`E28` si chiude qui, ed è la ragione per cui la mutazione 5 è stata eseguita.** Quella voce
registrava che la guardia `asked > ceiling` era al Task 5 **interamente sussunta** — cancellarla
non uccideva niente — e dichiarava che *«acquista senso al Task 6, quando il ramo alternativo
diventa `Queued`»*. ✅ **L'attesa si è avverata, e la conferma è la stessa mutazione ripetuta
sullo stesso codice:** `a_request_larger_than_the_total_is_refused_and_not_queued` è oggi
**l'unica** custode di quella guardia, e senza di essa una richiesta più grande dell'intera
macchina verrebbe **accodata per sempre**.

✅ **VOCE CHIUSA — la sonda delle due quote permanenti ha PERSO la proprietà per cui era stata
scritta, e il rimedio non sta nell'arbitro.** Fino al Task 5 una configurazione impossibile era
**visibile**: la seconda quota permanente tornava `Refused` coi due numeri. Con le code torna
`Queued`, e **nessuno la servirà mai** — rilasciare una concessione permanente è esattamente
ciò che nessuno fa. È il degrado silenzioso che ADR-0005 e ADR-0019 vietano. ⛔ **L'arbitro non
può ripararlo, e la ragione è già nel sorgente:** *«Permanence is not a type -- it is nobody
calls release»*, quindi l'arbitro non sa distinguere un biglietto che **sarà** servito da uno
che non lo sarà mai. ✅ **La visibilità si è spostata alla radice di composizione, al Task 10**, che
le due concessioni permanenti le chiede **lei** e tratta un `Queued` come un fallimento
d'avvio. Registrato nell'errata del piano col Task 10 nominato come chiusore; scritto anche
**accanto alla sonda**, dove lo legge chi la tocca. 📌 **Chiusa il 2026-08-21** nella sezione «Il
grafo di produzione monta l'arbitro, il giornale e le due concessioni».

⚠️ **VOCE APERTA — LO SCAVALCAMENTO CHE `promote` RIFIUTA DENTRO UNA CORSIA, FRA CORSIE LO FA —
con un'inversione di priorità sopra.** Il doc di `promote` giustificava la propria regola d'arresto
dicendo che scavalcare *«lascerebbe una richiesta grande in una corsia affollata aspettare per
sempre dietro le piccole»*: ⛔ **fra corsie il codice produce esattamente quello.** Una corsia che
si ferma **cade sulla successiva**, quindi una richiesta piccola in una corsia **peggiore** viene
servita mentre una grande in una corsia **migliore** aspetta. ✅ **Misurato il 2026-08-19 su una
sonda usa-e-getta fuori dal repository, non dedotto:** macchina da `4_096` con `bulk` `3_072` e
`small` `1_024` residenti, un'attesa **`Realtime`** da `4_096` accodata **prima** di un'attesa
**`Batch`** da `1_024` (biglietti `0` e `1`); rilasciata `small`, `promote` restituisce **una**
promozione ed è il **biglietto 1**, quello `Batch`, e il `Realtime` resta in coda.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo
a questo file**, che dice anche **chi la chiude** — rimando aggiunto il 2026-08-25.

⚖️ **REGISTRATA, NON PRESA, e il comportamento NON è stato cambiato.** È dettato dal Passo 3 del
piano — il corpo detta il `break` interno e la caduta sulla corsia successiva — ed è una scelta
**work-conserving** difendibile: l'alternativa tiene la macchina ferma per un'attesa che potrebbe
non entrare mai. ⛔ **Il difetto non è il comportamento, è che la conseguenza non stava scritta da
nessuna parte:** §5.3, §5.3.1 e `design/02-arbitrato-gpu.md` non dicono niente sull'ordine **fra**
corsie, e la frase del doc lasciava credere il contrario. Adesso la frase è **circoscritta a
dentro una corsia** e dice per esteso che cosa succede fra corsie. Che l'ordine debba essere
work-conserving o rispettare la priorità è una decisione del **proprietario**; registrata anche
nell'errata del piano.

⛔ **E NESSUNA SONDA TIENE QUELLA FRASE DEL DOC — dichiarato il 2026-08-19, non pinzato.**
✅ **Rimisurato di persona, non copiato dal rilievo:** fermata l'**intera passata** alla prima
corsia la cui testa non entra — nessuna caduta sulla corsia successiva — **niente va rosso**:
`cargo test --locked -p kernel --test arbiter_admission` → **19 passate, 0 fallite**, e
`cargo test --workspace --no-fail-fast --locked` → **34 target, 222 passate, 0 fallite, 2
ignorate** (riga **1d** della campagna). Il mutante è **vivo**, quindi il giorno in cui il primo
ciclo di orchestrazione scegliesse un altro ordine fra corsie, quel paragrafo diventerebbe **falso
in silenzio**. ⚠️ **Richiamo del 2026-08-21:** nominava anche il **Task 7**, passato senza cambiarlo.
⚖️ **E non è stato pinzato apposta, sul merito:** una sonda che congelasse la caduta
congelerebbe la politica che **questa stessa voce** chiede al proprietario, e *«una sonda che va
cancellata per prendere una decisione è un voto contro il prenderla»* — è la ragione di `E39`.
La dichiarazione sta **nel sorgente**, accanto al paragrafo; qui e in `E53` è il suo puntatore.

⚠️ **VOCE APERTA — `admit` NON CONSULTA MAI LA CODA, quindi un ritardatario la scavalca, e questo
è ciò che circoscrive la promessa su M-7.** `admit` legge `held` e `parameters` e basta: se la
stanza c'è nel momento in cui viene chiamata, dice sì, qualunque cosa stia già aspettando.
✅ **Misurato il 2026-08-19 su una sonda usa-e-getta fuori dal repository:** macchina da `4_096`
piena, un biglietto **`Realtime`** da `4_096` in coda, rilasciato il residente; una **nuova**
richiesta `Batch` da `4_096` che arriva da `admit` è **`Granted` all'istante**, `allocated()`
torna a `Mib(4096)`, e il `promote` che segue restituisce **zero** promozioni col `Realtime`
ancora in attesa. ⛔ **Conseguenza sulla riga che apre questa sezione:** *«l'ordine per corsia è
ciò che tiene validi i numeri di M-7»* è vero **di `promote`**, non dell'arbitro — l'ordine
d'**ammissione** lo sconfessa, e la riga della sonda qui sopra è stata circoscritta di
conseguenza.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo
a questo file**, che dice anche **chi la chiude** — rimando aggiunto il 2026-08-25.

⚖️ **REGISTRATA, NON PRESA, e il comportamento NON è stato cambiato.** ⛔ **Chiuderlo dentro
`admit` significherebbe un'ammissione che RIFIUTA stanza che esiste**, cioè una politica di
scheduling che nessun ADR ha deciso; e *quando* si chiama `promote` rispetto ad `admit` è una
questione di **orchestrazione**, quindi di chi costruisce il primo ciclo. ⚠️ **E interagisce
direttamente con la voce delle due quote permanenti qui sopra**, che manda allo stesso chiusore:
una quota permanente accodata non solo non sarà mai servita, ma può vedersi passare davanti
qualunque richiesta che arrivi dopo. Registrata anche nell'errata del piano.

⛔ **E NESSUNA SONDA TIENE NEMMENO QUESTA FRASE DEL DOC — dichiarato il 2026-08-19, non
pinzato.** ✅ **Rimisurato di persona:** aggiunto `!self.queues.is_empty() ||` alla seconda
guardia di `admit`, così un ritardatario si **accoda dietro** chi aspetta invece di scavalcarlo,
**niente va rosso** — `arbiter_admission.rs` **19 passate, 0 fallite** e il workspace **34
target, 222 passate, 0 fallite, 2 ignorate** (riga **4b** della campagna). Mutante **vivo**: se
quel primo ciclo decidesse nell'altro senso, il paragrafo del doc di `admit` diventerebbe **falso
in silenzio**. ⚖️ **Non pinzato apposta**, stessa ragione di `E39` e della dichiarazione gemella
su `promote`: una sonda che asserisse `Granted` per il ritardatario congelerebbe la scelta che
questa voce chiede al proprietario. Dichiarato **nel sorgente**, accanto al paragrafo; `E53` è
la voce.

⚠️ **VOCE APERTA — `promote` restituisce un `Vec<Promotion>` senza `#[must_use]`, e ignorarlo
perde le concessioni.** ✅ **MISURATO IL 2026-08-19 SU UNA SONDA USA-E-GETTA FUORI DAL
REPOSITORY, e non più affermato** — la riga di prima diceva *«`arbiter.promote(now);` da solo
compila»* senza citare nessuna misura, dove `E30` ed `E31` la citano entrambe. Le due direzioni,
misurate una per una: ① **così com'è**, `arbiter.promote(now);` da solo compila e la crate
chiamante resta verde **anche con `-D warnings`** — nessun lint scatta affatto; ② con
`#[must_use]` messo su `promote`, la stessa riga produce
`warning: unused return value of Arbiter::promote that must be used`, `#[warn(unused_must_use)]`,
e la compilazione **riesce lo stesso, exit 0**. Nella sonda ① la promozione avviene davvero:
dopo quella riga `allocated()` legge `Mib(4096)` e `queued()` legge `0` — le concessioni sono
**nei libri** senza che nessuno le tenga, la VRAM resta prenotata fino alla scadenza della
finestra e il chiamante non ha nulla da rilasciare.
📇 **Indicizzata nella tabella *«LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA»*, in fondo
a questo file**, che dice anche **chi la chiude** — rimando aggiunto il 2026-08-25.

⚖️ **REGISTRATA, NON PRESA:** `#[must_use]` costa una riga ed è la forma che `Admission` ha già,
ma la misura ② dice che è un **lint** e non una regola che una sonda possa tenere — produce un
**avviso** e non un errore, quindi nessun caso `compile_fail` può pinzarlo. ⛔ **E la via
d'uscita apparente è proprio il gotcha #39:** un caso che dichiarasse `#![deny(unused_must_use)]`
per farlo diventare errore proverebbe che **il lint morde dove è dichiarato**, non che il kernel
dichiari l'attributo — che è la forma esatta dei quattro casi che ridichiarano `#![no_std]`, e
per cui è dovuto nascere `gate-attributes.sh`. Metterlo significherebbe aggiungere superficie che
nessuna delle due direzioni di §7.1.4 può provare; lasciarlo fuori significa che la trappola
resta. La scelta è del proprietario.

⚠️ **`Queued` è RAGGIUNGIBILE A RUNTIME da oggi, e nessuna riga di catalogo lo chiedeva.** `V4`
resta coperta come al Task 5 — *«non compila»* con i due casi negativi, *«compila»* con
`an_admission_is_distinguishable_three_ways`, che le tre varianti le **costruisce** — e ciò che
il Task 6 aggiunge è che la seconda variante ora **esce davvero** da un percorso di produzione,
il che è più di quanto `V4` pretenda. ⛔ **Nessuna riga nuova è stata aggiunta al catalogo:**
§7.4 è **spec**, e aggiungere una riga è una decisione del **proprietario** (vincolo globale 7).
Registrata nell'errata.

📌 **Conteggi, ricontati eseguendo il binario e mai dedotti** (gotcha #31): workspace **34
target, 222 passate, 0 fallite, 2 ignorate** — erano **214** all'inizio del compito, e le otto
in più sono le otto sonde nuove di questo file. Nessun target nuovo. Per file:
`arbiter_admission.rs` **diciannove** (erano undici), `arbiter_resource.rs` **otto**
(invariato), e i casi `compile_fail` **ventotto** — ✅ ricontati col comando,
`ls crates/kernel/tests/compile_fail/*.rs | wc -l` → **28**, invariati: il Task 6 non ne ha
aggiunto nessuno e non ha rigenerato nessun oracolo.

✅ **RICONTATI DOPO LA REVISIONE DEL 2026-08-19, e ricontati invece che dedotti anche quando la
deduzione era facile** (gotcha #31): workspace **34 target, 222 passate, 0 fallite, 2 ignorate**,
`arbiter_admission.rs` **diciannove**, `compile_fail` **ventotto** — ⛔ **tutte e tre invariate, ed
è il punto della passata**: la revisione ha **riscritto** la sonda delle corsie e ha aggiunto tre
mutazioni, che non lasciano traccia nei conteggi perché una mutazione si revoca. Nessuna sonda
nuova, nessun caso nuovo, nessun oracolo rigenerato. `cargo build --locked --workspace` stampa
**zero warning**.

#### La revoca e la grazia che scade — Traguardo 5, Task 7, e le sonde che vivono in `src/`

⛔ **`ask_back` MARCA, non prende, e la spazzata copre DUE scadenze.** Chiedere indietro mette la
concessione in `Revoking { deadline }` e **lascia la prenotazione nei libri** per tutta la grazia:
la §5.3 punto 4 dà al titolare quel tempo per consegnare, e un arbitro che liberasse subito
ammetterebbe un secondo consumatore sulla VRAM che il primo sta ancora usando. A riscuotere è
`collect_expired`, che da oggi guarda `expires_at` **e** `deadline` — la finestra di validità che
il richiedente ha dichiarato e la grazia che una revoca ha concesso.

⛔ **E MARCA IN DUE PASSATE, DAL 2026-08-20 E IN REVISIONE DEL COMPITO GIÀ CHIUSO.** Il ciclo
originale marcava finché il bisogno era coperto **oppure** finché le corsie finivano, senza
guardare **prima** se il recuperabile bastasse: su una macchina `8_192` con un `Batch`
prelazionabile da `2_048` e un `Batch` non prelazionabile da `6_144`, un `ask_back(4_096, …)`
**condannava** il primo — alla scadenza della grazia la spazzata lo toglie dai libri — e
rispondeva `2_048`, cioè **non faceva sedere nessuno**. ✅ **Riprodotto misurando**, non
ragionato: `` left: Mib(2048), right: Mib(0) ``, bersaglio `lib` **11 passate, 1 fallita**.
⛔ **È lo stesso danno che il doc della funzione dichiara di evitare** — *«evicts two jobs to
seat one»* — preso dall'altra strada: **uno sfrattato e nessuno seduto**. ⚖️ **E si è corretto
invece di registrarlo aperto, che è il contrario di ciò che si fa con `E50`/`E51`:** quelle sono
**politiche** senza risposta giusta, e pinzarle sarebbe un voto contro il deciderle (gotcha
**#73**); questa è un'operazione che **muta lo stato e riporta fallimento lasciando la mutazione
lì**, cioè il degrado silenzioso che **ADR-0005** e **ADR-0019** vietano. ✅ **La forma:** una
passata in **sola lettura** somma il recuperabile fra i candidati ammissibili, e **solo se copre
il bisogno** parte quella che marca; se non copre, **non si marca nulla** e la risposta è
`Mib::ZERO`. ⛔ **Nessuno stato nuovo e nessun tipo nuovo:** il criterio di ammissibilità è
scritto **una volta sola** — una chiusura `askable` che entrambe le passate interrogano — e una
copia verbatim del predicato sarebbe il difetto che questa correzione esiste per non
introdurre. ⚠️ **Chiusura e non metodo, e la ragione è una misura:** installato come
`Held::askable_by`, `cargo build --locked --workspace` stampa **tre** avvisi `dead_code` invece
dei due che il proprietario ha accettato — il terzo è `` method `askable_by` is never used ``,
perché ciò di cui `ask_back` è l'unico chiamante muore con lui. Voce `E69`.

⚖️ **DENTRO UNA CORSIA LA VITTIMA È LA PIÙ VECCHIA, E NESSUN DOCUMENTO LO DECIDE — dichiarata,
non pinzata, e la distinzione con la riga qui sopra è il punto.** La passata che marca percorre
`held`, chiavato per `GrantId`, quindi incontra le concessioni di una corsia nell'ordine in cui
sono state emesse: con un `Batch` da `4_096` e uno da `512` e un bisogno di `512` marca il
`4_096`. L'alternativa — **la più piccola che basta** — è altrettanto difendibile, e §5.3,
§5.3.1 e design/02 tacciono: la tabella delle corsie decide l'ordine **fra** corsie, non
**dentro** una. ✅ **Il mutante è VIVO, e RIMISURATO il 2026-08-20 contro il codice della seconda
ondata invece che citato** (`E80`) — riga **13** della campagna, `13 passate` e
`20 passate` con la più recente per prima — e sta **nel doc di `ask_back`, accanto alla frase**,
così il giorno in cui qualcuno cambia l'ordine quella frase diventa falsa senza che nulla lo
dica. ⛔ **Nessuna sonda la tiene, per la ragione di `E39`:** una sonda che asserisse *«la più
vecchia»* congelerebbe la scelta, e una sonda da cancellare per prendere una decisione è un voto
contro il prenderla. Voce `E70`, forma di `E50`/`E51`/`E53`.

⛔ **E LA PASSATA CHE MARCA SI FERMAVA ALLA PRIMA CORSIA SENZA CHE NESSUNO SE NE ACCORGESSE —
SECONDA ONDATA, 2026-08-20, ed è `E69` PRESA DA UNA TERZA STRADA.** La passata di sola lettura
somma **tutte** le corsie, quindi può promettere `6_144`; se quella che marca si fermasse dopo la
prima, risponderebbe `2_048` **col titolare di quella corsia già condannato** — uno sfrattato e
nessuno seduto, esattamente il danno che le due passate esistono per togliere. ⛔ **Nessuna sonda
lo chiedeva:** sette avevano un residente solo, due ne avevano due nella **stessa** corsia, una
esce dal ramo `reclaimable < needed`, e l'unica a due corsie —
`asking_back_takes_the_worst_lane_first` — chiede `1_024` contro un `Batch` da `2_048`, quindi la
corsia peggiore le basta da sola e la passata non deve mai uscirne. ✅ **Riprodotto misurando e
non ragionato:** `lanes.iter().rev()` ridotto a `.take(1)` è sopravvissuto all'**intero
workspace** — **34 target, 235 passate, 0 fallite, 2 ignorate**. ⚖️ **E non è una politica
aperta**, quindi l'argomento di `E39`/`E70` — *«una sonda da cancellare per decidere è un voto
contro il deciderla»* — **non si applica**: c'è una risposta giusta. Sonda nuova
`asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough` e riga **14** della
campagna. Voce `E75`.

⛔ **E L'INSIEME DELLE CORSIE NON ERA QUELLO DEI CANDIDATI, quindi la frase «la passata di sopra
ha già sommato lo stesso insieme» era una PROMESSA e non una costruzione.** `lanes` raccoglieva la
corsia di **ogni** concessione, ammissibile o no, quindi una corsia senza candidati veniva
percorsa a vuoto. ✅ **Misurato nelle DUE direzioni**, restituendo `lanes.len()` da lì su una
sonda usa-e-getta cancellata subito dopo: con un residente `Realtime` che `below = Interactive`
esclude e un candidato `Batch`, la passata percorreva **due** corsie **prima** del filtro e **una**
dopo. ⚠️ **Innocuo di per sé — e non è per questo che si corregge:** un `filter(|held|
askable(held).is_some())` nella stessa catena rende i due insiemi **lo stesso insieme per
costruzione**, cioè toglie la distanza che il commento avrebbe dovuto promettere via. ⛔ **E non
sostituisce la sonda della riga 14:** la struttura toglie la distanza fra i due insiemi, la sonda
uccide il mutante che **salta una corsia**. Sono due difetti diversi e nessuno copre l'altro. Voce
`E76`.

⛔ **E IL DOC DI `ReleaseError::UnknownGrant` DICEVA «DUE CAUSE»: DA QUESTO COMPITO SONO TRE.**
Con la riscossione forzata dentro `collect_expired`, una concessione **chiesta indietro** la cui
**grazia** è scaduta esce dai libri come una scaduta di finestra, quindi un `release` su di essa
risponde `UnknownGrant` per una terza ragione che nessuno dei paragrafi nominava. ✅ **Misurato
su una sonda usa-e-getta cancellata subito dopo:** chiesta indietro a `0` con grazia `500` e
rilasciata a `500` esatti → `Err(UnknownGrant)`; la stessa a `499` → `Ok(Mib(4096))`. ⛔ **Le
frasi diventate false sono state RISCRITTE e non affiancate** (finding **A-2**), con un richiamo
datato che dice che cosa dicevano prima, e **`E30` è stata allargata**: con tre cause una sola
variante `Expired` non le separa più, quindi la **forma** del rimedio è essa stessa parte della
decisione del proprietario. ⛔ **E nessuna sonda pinza quei tre valori**, per la stessa ragione
di `E39`. Voce `E72`.

⛔ **TREDICI SONDE SU QUATTORDICI VIVONO IN `crates/kernel/src/arbiter/mod.rs`, in un
`#[cfg(test)] mod tests`, ed è il TERZO modulo di test in `src/` del workspace.** ⚠️ **Erano dieci
su undici alla chiusura del compito; la prima ondata di correzioni del 2026-08-20 ne ha portate
due e la seconda una terza, e i conteggi di questa sezione sono stati rifatti da capo invece che
datati — due volte** (`E73`, poi `E80`). `ask_back` è `pub(crate)` — la
decisione del proprietario, perché il suo unico chiamante è l'ammissione sotto policy LOCALE — e
un `pub(crate)` **non si vede da un test d'integrazione**, che è una crate a sé. ✅ **Misurato e
non assunto:** chiamandolo da `crates/kernel/tests/arbiter_admission.rs` esce
`` error[E0624]: method `ask_back` is private ``. Il precedente e la forma della giustificazione
sono `crates/platform/src/rng.rs`. ⛔ **Solo la privatezza sposta una sonda lì dentro:**
`a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` non chiama `ask_back` e **resta**
in `tests/arbiter_admission.rs`, accanto alle altre sonde della finestra di validità.

⚠️ **E il commento di modulo di `arbiter_admission.rs` è stato ESTESO e non sovrascritto**, che è
l'istruzione che quel file porta dal Task 5 e che il Task 6 aveva saltato (`E56`): l'enumerazione
dei soggetti ora nomina anche la revoca, e dice **dove** stanno le sonde che chiamano `ask_back` e perché. ⚠️ **Quel conteggio è stato RISCRITTO DUE VOLTE e mai annotato** (gotcha **#31**): diceva
«dieci», la prima ondata del 2026-08-20 lo ha portato a «dodici» e la seconda a «tredici».
⛔ **E la seconda ondata ha tolto la SECONDA COPIA del numero**, che stava nel doc dell'unica sonda
del Task 7 rimasta in quel file e diceva ancora *«the other ten probes»* — **due cifre al presente
che si contraddicevano nello stesso file**, e la copia stantia sedeva **sotto** il paragrafo che
rivendicava di aver appena riscritto quel numero: gotcha **#31** più gotcha **#68**, la regola che
non lega il documento che la ospita. La cifra vive ora in **un posto solo** e la sonda ci
**rimanda** invece di ripeterla (`E77`).

⛔ **TRE CIFRE E UNA PROMESSA DI DOC RIMESSE A POSTO NELLA SECONDA ONDATA, e nessuna delle tre si
è RIALLINEATA: due sono state TOLTE e una riscritta.** ① Il doc dell'unica sonda del Task 7 rimasta
in `arbiter_admission.rs` diceva *«the other ten probes»* mentre il commento di modulo **dello
stesso file**, trenta righe più su, diceva **dodici** e rivendicava di aver appena riscritto quel
numero: la copia è stata **tolta** e la sonda **rimanda** al commento di modulo, perché una cifra
che vive in due punti marcisce in uno dei due (gotcha **#31** più **#68**). ② ⛔ **E il `grep`
chiesto prima di correggerla ne ha trovata un'ALTRA che nessuno cercava:** il doc di
`ReleaseError` diceva *«`release` has TWO callers in this repository»*, ed era vera al Task 5
finché il **Task 6 non l'ha falsificata nel commit successivo** dando alle sonde delle code i loro
rilasci. La cifra è **tolta e non ingrandita**: l'argomento ha bisogno di *«nessun consumatore di
produzione»*, che è la metà che non marcisce. ⛔ **Il riconteggio che la sostituiva è finito però
in quattro documenti a un tempo** — questa riga fra essi — **sotto un titolo che dichiarava di non
rimpiazzarla con un numero più grande**, ed è stato ristretto a **un posto solo** dalla terza
revisione: sta in **`E77`** del piano e qui non si ripete (`E85`). ③ Il doc di `Held::grace` diceva *«see the two guards
there»* mentre la chiusura `askable` dice *«THREE QUESTIONS AND NOT ONE, AND THEY STAY THREE»* —
la prima ondata aveva portato la guardia sulla **corsia** dentro il criterio — e la frase è
**riscritta**, non affiancata. Voci `E77` e `E78`.

⚠️ **UN'ASSERZIONE CHE NON PUÒ FALLIRE SOTTO UNA MUTAZIONE DI `ask_back` — E CHE SOTTO DUE RIGHE
DELLA CAMPAGNA È L'UNICA CHE SCATTA.** In
`asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`,
l'`assert_eq!(arbiter.allocated(), Mib::new(8_192))` è vera per qualunque implementazione di
**quella funzione**: `ask_back` non toglie mai dai libri — solo `collect_expired` lo fa, e allo
`ORIGIN` con finestre `LONG` non c'è niente da riscuotere. ✅ **Riprodotto misurando il 2026-08-20,
in terza revisione, sulla suite di quel momento** — **tredici** sonde nel `lib` e **venti** in
`arbiter_admission.rs`: isolata (le due asserzioni sopra tolte) resta **verde** sotto la mutazione
**12**, suo uccisore solo — **13 passate, 0 fallite**, dove a piena forza è **12/1** — sotto la
**8** (**12/1**, e a morire è un'altra) e sotto la **1** (**6/7** isolata **e** a piena forza,
identiche, perché lì la sonda non muore comunque).

⛔ **MA NON SONO LE DUE SOPRA A TENERLA, ed è la frase che questa riga portava fino alla terza
revisione.** La sonda muore sotto **5b**, **5d**, **8**, **11**, **12**, e non attraverso la stessa
asserzione. ✅ **Misurato leggendo QUALE asserzione va in panico** e non che la sonda fosse rossa:
sotto **8** e **12** scatta la **prima**, la risposta (`Mib(8192)` e `Mib(2048)` contro `Mib(0)`);
sotto **11** la **seconda**, lo stato (`1` contro `0`); sotto **5b** e **5d** — le due che mutano
`collect_expired` invece di `ask_back` — le prime due **passano** e scatta **solo questa**,
`left: Mib(0)` e `left: Mib(6144)` contro `right: Mib(8192)`. Su quelle due righe è **questa**
asserzione a uccidere la sonda, da sola. ⚠️ **La causa dell'errore vale più dell'errore:** la
misura d'isolamento qui sopra campiona **solo** mutazioni di `ask_back`, cioè precisamente la
classe sotto cui l'asserzione non può fallire, e non ha mai eseguito le due righe in cui è
portante — **un'esclusività misurata su un campione parziale si legge come una garanzia**.
⛔ **Il rimedio resta quello — non si cancella** (è la specie di `E37`), **ed era la RAGIONE
scritta accanto a essere sbagliata, per difetto:** non serve invocare il giorno in cui `ask_back`
avesse una via che tocca i libri, perché la spazzata che chiama per prima ce l'ha già. Voci `E79`,
corretta da `E81` (le cifre) ed `E82` (la tesi).

⛔ **QUATTORDICI SONDE, di cui SEI dettate dal piano e OTTO no.** Le otto in più esistono perché
una regola scritta in un commento e tenuta da niente è un'intenzione (gotcha **#42**), perché
**due** delle mutazioni dettate non uccidevano niente senza di esse, e perché le due revisioni del
2026-08-20 hanno trovato **un confine tenuto da nessuno**, **un'operazione che mutava lo stato
riportando fallimento** e **una promessa di doc che nessuna sonda teneva** — la passata che marca
che non si ferma alla prima corsia.

| Sonda | Dove | Cosa tiene |
|---|---|---|
| `asking_a_grant_back_marks_it_and_does_not_free_it_yet` | `src` | che chiedere indietro **marchi** e non prenda: `revoking()` sale a uno e `allocated()` non si muove |
| `a_grace_that_ran_out_returns_the_reservation_to_the_budget` | `src` | che oltre la scadenza della grazia la prenotazione **torni** nel budget — la metà arbitro di `Forzata` (§6.5 del disegno) |
| `a_non_preemptible_grant_is_never_asked_back` | `src` | ⛔ **DETTATA, E CORRETTA: il residente è in corsia `Batch` e non in `Realtime`.** Come dettata passava **per la ragione sbagliata** — la guardia sulla **corsia** lo scartava prima che la sua prelazionabilità fosse guardata, quindi il meccanismo che il nome promette non girava mai (gotcha **#74**). In `Batch`, strettamente sotto la corsia che chiede, a salvarlo può essere **solo** la guardia sulla grazia |
| `only_lanes_below_the_asking_one_are_asked_back` | `src` | la contro-sonda per l'altra strada: un lavoro `Realtime` **prelazionabile** non viene sfrattato per uno `Interactive` |
| `asking_back_stops_as_soon_as_the_need_is_covered` | `src` | che si fermi appena la stanza basta: *«ha fatto spazio»* è soddisfatto anche da chi revoca tutto e sfratta due lavori per sederne uno |
| `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` | `tests/` | ⛔ **DETTATA, E CORRETTA nella variante attesa:** il piano asseriva `Refused`, e dal Task 6 *«entra nella macchina ma non in questo momento»* è `Queued`. È la contro-sonda del ramo `_` della spazzata |
| `a_grant_inside_its_grace_keeps_its_reservation` | `src` | ⚠️ **NON dettata.** `allocated()` non riscuote, e **qui** un'asserzione scritta subito dopo l'`ask_back` non passa da nessuna spazzata capace di mettere alla prova la grazia: la spazzata che `ask_back` porta con sé è la sua **prima istruzione**, e corre **prima** che `ask_back` marchi il residente `Revoking`. Questa fa spazzare qualcun altro — un `admit` a `499` su una grazia che scade a `500` — mentre la grazia corre ancora. ⛔ **RICHIAMO DEL 2026-08-28 (AUD-013):** la riga diceva *«`allocated()` non riscuote e `ask_back` nemmeno»* — falso, `ask_back` riscuote per primo, per la ragione appena scritta — e *«senza di lei la mutazione 5 del piano non uccide NIENTE»* — falso anche questo: muore pure `asking_back_twice_does_not_buy_the_room_twice`, con la sua **seconda** chiamata ad `ask_back` che spazza a `200` mentre la grazia corre fino a `500`. ⚠️ **E lo scoping conta:** dopo un `ask_back` che trovi il residente **già** `Revoking` la spazzata mette eccome alla prova la grazia — è esattamente ciò che fa quella sonda. Cifre e sonde morte: **riga 5** della tabella delle mutazioni qui sotto, voce `E64` |
| `the_grace_runs_out_at_the_instant_of_its_deadline` | `src` | ⚠️ **NON dettata, ed è la specie di `E29` sulla SECONDA scadenza.** Le due sonde qui sopra chiedono a `501` e a `499`: su `500` esatti non chiedeva nessuno, e `deadline > now` mutato in `>=` sarebbe sopravvissuto all'intera suite. Dichiara anche **quale** semantica è quella scelta: grazia **semiaperta**, `[chiesta, scadenza)`, la stessa regola della finestra di validità |
| `asking_back_twice_does_not_buy_the_room_twice` | `src` | ⚠️ **NON dettata.** È ciò che la guardia su `activity` compra **da sola**: una seconda passata che rimarcasse una concessione già in uscita conterebbe la sua prenotazione **due volte** in `covered` — stanza che l'arbitro non ha — e le sposterebbe la scadenza più in là, regalando al titolare **più** tempo per essere stato chiamato prima |
| `ask_back_collects_the_expired_before_it_marks` | `src` | ⚠️ **NON dettata.** *«L'arbitro riscuote prima di decidere»* è una proprietà di **ogni** operazione, e con `ask_back` sono **quattro**. Il corpo dettato **non riscuoteva**: la riscossione è stata aggiunta per analogia con `promote`, e questa è l'unica sonda che ne esercita la riga. Nessun `release` qui dentro, come in `promote_collects_the_expired_before_it_serves_the_queue` |
| `asking_back_takes_the_worst_lane_first` | `src` | ⚠️ **NON dettata.** *«Prima la corsia peggiore, la cosa più economica da interrompere»* è una frase che il doc di `ask_back` **afferma**, e che la tabella delle corsie di [design/02](design/02-arbitrato-gpu.md) fonda — `interattivo` è *«servita prima di `batch`»*, `batch` *«può attendere indefinitamente»*. ⚠️ **Le due prenotazioni sono DIVERSE apposta:** `revoking()` conta e non nomina, quindi due vittime della stessa taglia sarebbero indistinguibili — `2_048` in `Batch` contro `4_096` in `Interactive`, ed è la **risposta** a dire quale è stata presa |
| `a_grant_in_the_asking_lane_itself_is_not_asked_back` | `src` | ⚠️ **NON dettata, e nata IN REVISIONE il 2026-08-20** (`E71`). Delle dieci sonde di allora **nessuna** metteva il residente nella **stessa** corsia dell'argomento `below` — tutte strettamente sopra o strettamente sotto — quindi il confine non lo chiedeva nessuno e `lane <= below` mutato in `lane < below` sopravviveva all'**intero workspace**, misurato. ⛔ **Ed è il caso che il Task 8 produce per primo:** l'ammissione chiede indietro **sotto la propria corsia**, quindi un **pari** è la prima cosa che `ask_back` si vedrà passare, e sfrattare un pari per un pari è ciò che *«only lanes BELOW»* esclude |
| `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need` | `src` | ⚠️ **NON dettata, e nata IN REVISIONE il 2026-08-20** (`E69`). È lo stesso danno che il doc di `ask_back` dichiara di evitare — *«evicts two jobs to seat one»* — preso dall'**altra** strada: **uno sfrattato e nessuno seduto**. Macchina `8_192`, un `Batch` prelazionabile da `2_048` e un `Batch` **non** prelazionabile da `6_144`: `2_048` è tutto il recuperabile e non siede un `4_096`. ⚠️ **Il non prelazionabile sta in `Batch` apposta:** sotto `Realtime` lo scarterebbe la guardia sulla **corsia** e la sonda parlerebbe di corsie invece che di capienza (gotcha **#74**) |
| `asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough` | `src` | ⚠️ **NON dettata, e nata nella SECONDA revisione del 2026-08-20** (`E75`). Che la passata che marca **arrivi in fondo alle corsie**: la **corsia peggiore** non deve bastare da sola, perché la passata di sola lettura le ha sommate **tutte**. Macchina `8_192`, `Interactive 4_096` e `Batch 2_048` entrambi prelazionabili, `below = Realtime`, `ask_back(4_096, …)` → **`6_144`** e `revoking() == 2`. ⚠️ **E lo scenario è quel caso e non uno più largo, detto qui perché la frase non prometta di più:** `Interactive 4_096` copre il bisogno **da sola** — a non bastare è la corsia **peggiore**, non «nessuna delle due» — ed è esattamente per questo che la risposta **eccede** (`E86`). ⛔ **`asking_back_takes_the_worst_lane_first` non lo chiedeva:** chiede `1_024` contro un `Batch` da `2_048`, quindi la corsia peggiore le basta e la passata non deve mai uscirne — il mutante `.take(1)` sopravviveva all'**intero workspace**. ⚠️ **E pinza una seconda cosa che nessuno diceva: la marcatura può ECCEDERE** — `6_144` per un bisogno di `4_096` — cioè si ferma alla prima concessione che porta **oltre** la linea e non ne cerca una che ci **atterri** (direzione negativa: mutazione **10b**) |

⛔ **VENTISEI mutazioni, una alla volta, ciascuna provata entrata col conteggio delle
occorrenze, COMPILATA in un passo separato da quello che la esegue, e revocata da una copia
byte-esatta presa prima** (mai `git checkout --`, gotcha #48; nessun `sed -i` — il rimpiazzo è
byte-a-byte in Python, e i **CR sono zero prima e dopo** su entrambi i sorgenti, verificati a ogni
passo). ✅ **Tutti i conteggi sono della suite di adesso** — **tredici** sonde nel bersaglio `lib`
di `kernel` e **venti** in `arbiter_admission.rs` — e dove ne compaiono due sono nell'ordine
`lib` poi `arbiter_admission`. ⛔ **E SONO STATE RIMISURATE TUTTE DA CAPO DUE VOLTE NELLO STESSO
GIORNO**, invece che datate: la prima ondata del 2026-08-20 ha portato il `mod tests` da dieci a
dodici sonde (`E73`), la seconda da dodici a **tredici** (`E80`), quindi a ogni giro ogni cella
qui portava il numero di una suite che non esisteva più (gotcha **#31**, ed è la ragione per cui
la revisione del Task 6 fece lo stesso). ⚠️ **La terza ondata NON le ha rimisurate tutte, e la
differenza si dichiara:** non ha aggiunto nessuna sonda, quindi i conteggi qui non sono invecchiati;
sono state **rieseguite** le sette righe che le servivano — **1**, **5b**, **5d**, **8**, **11**,
**12** e la **15** nuova — e ciascuna ha ridato **i numeri che la sua cella porta**, senza
eccezioni (`E81`, `E82`, `E83`). ⚠️ **E la seconda rimisura ha corretto la MUTAZIONE, non
solo i numeri:** la riga **1** era stata applicata mettendo il `retain` **solo in coda**, dove
l'uscita anticipata `return covered` lo saltava — misurata così dava **9/4** invece di **6/7**, ed
era una mutazione più debole di quella che la cella descrive. Rifatta con `break 'lanes`, perché la
riscossione avvenga **sempre**. Le tre righe **`D`** restano l'unica eccezione: sono **datate**,
non rimisurate, e il perché sta nella loro cella.

| # | Mutazione, sul codice di produzione | Sonda attesa morta | Misurato (suite di tredici + venti) |
|---|---|---|---|
| **1** | `ask_back` **libera subito** invece di marcare: un `retain` che toglie ogni `Revoking`, con l'uscita anticipata portata a `break 'lanes` perché quel `retain` sia **sempre** raggiunto | `asking_a_grant_back_marks_it_and_does_not_free_it_yet` | ✅ **rossa**, e cadono con lei **sei**: `a_grace_that_ran_out_…`, `a_grant_inside_its_grace_…`, `asking_back_stops_…`, `asking_back_takes_the_worst_lane_first`, `asking_back_crosses_…`, `asking_back_twice_…`. **6 passate, 7 fallite** |
| **2** | la guardia sulla **corsia** tolta dal criterio di ammissibilità | `only_lanes_below_the_asking_one_are_asked_back` | ✅ **rossa**, e con lei la sonda del **confine** `a_grant_in_the_asking_lane_itself_is_not_asked_back`. **11 passate, 2 fallite** — ⚠️ era *«sola»* al Task 7, quando quel confine non lo chiedeva nessuno |
| **2b** | ⚠️ **NUOVA del 2026-08-20**, il confine della stessa guardia: `held.lane <= below` diventa `held.lane < below` | `a_grant_in_the_asking_lane_itself_is_not_asked_back` | ✅ **rossa, e SOLA — 12 passate, 1 fallita.** ⛔ **Prima della sonda nuova questo mutante era VIVO nell'INTERO WORKSPACE** — `lib` 10/0, `arbiter_admission.rs` 20/0, ogni altro bersaglio verde. Voce `E71` |
| **2c** | ⚠️ **NUOVA del 2026-08-20**, la seconda direzione: `held.lane <= below` diventa `held.lane == below`, cioè scarta il **pari** e non ciò che sta **sopra** | `only_lanes_below_the_asking_one_are_asked_back` | ✅ **rossa, e SOLA — 12 passate, 1 fallita.** ⛔ **Cercata apposta:** con la sonda del confine la **2** non isola più nessuno, e senza questa riga `only_lanes_below_…` sarebbe rimasta dominata |
| **3** | il `return covered` anticipato tolto | `asking_back_stops_as_soon_as_the_need_is_covered` | ✅ **rossa**, e con lei `asking_back_takes_the_worst_lane_first` — senza la fermata marca **anche** la corsia migliore. **11 passate, 2 fallite** |
| **3b** | ⚠️ **aggiunta**, la seconda direzione sul **valore** della stessa guardia: `covered >= needed` diventa `covered > needed` | `asking_back_stops_as_soon_as_the_need_is_covered` | ✅ **rossa, e SOLA — 12 passate, 1 fallita.** ⛔ **Cercata apposta**, e la ragione è l'analisi di dominanza qui sotto |
| **4** | nella spazzata, il ramo `Revoking` diventa `true` (non riscuote mai) | `a_grace_that_ran_out_returns_the_reservation_to_the_budget` | ✅ **rossa**, e con lei `asking_back_twice_…` e `the_grace_runs_out_…`. **10 passate, 3 fallite** |
| **5** | nella spazzata, il ramo `Revoking` diventa `false` (riscuote subito) | il piano dice `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` | ⛔ **QUELLA SONDA NON MUORE, e non può: non c'è nessuna revoca nel suo scenario.** ✅ Muoiono **due**, ed **entrambe sono fra le NON dettate** — `a_grant_inside_its_grace_…` e `asking_back_twice_…`: **11 passate, 2 fallite**, e `arbiter_admission.rs` **20 passate, 0 fallite**. ⛔ **Quindi con le sole sei sonde dettate questo mutante era VIVO.** Registrato come `E64` |
| **5b** | nella spazzata, il **secondo** ramo: `_ => true` diventa `_ => false` | `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` | ✅ **rossa**, ma **non isola**: cadono **ventidue** sonde in tutto — `lib` **4 passate, 9 fallite** e `arbiter_admission.rs` **7 passate, 13 fallite**. Riscuotere tutto ciò che non è in revoca rompe metà del traguardo, il che prova che il ramo è **portante**, non che quella sonda sia sola a tenerlo |
| **5c** | il confine della grazia: `deadline > now` diventa `>=` | `the_grace_runs_out_at_the_instant_of_its_deadline` | ✅ **rossa, e SOLA — 12 passate, 1 fallita.** ⛔ È la ragione per cui quella sonda esiste: senza, la mutazione sopravviveva a tutte le altre |
| **5d** | ⚠️ **aggiunta**, la forma **stretta** della 5b: un ramo `Preemptible(Running) => false` davanti a `_ => true` | `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` | ✅ **rossa**, e nel proprio file è la **SOLA** — `arbiter_admission.rs` **19 passate, 1 fallita** — mentre nel `lib` ne cadono otto che tengono la revoca dall'altro capo. **5 passate, 8 fallite** |
| **6** | ⛔ **la domanda del gotcha #66**: `expires_at <= now` diventa `expires_at < now` | il piano dice `a_grant_still_inside_its_window_is_not_collected` | ⛔ **QUELLA SONDA RESTA VERDE, E NON PERCHÉ SIA DIVENTATA VACUA:** chiede a `4_999` su una finestra di `5_000` e sotto la mutazione fa la **stessa identica cosa** — quel confine non l'ha **mai** tenuto. ✅ **La sola morta è `a_grant_is_collected_at_the_instant_its_window_closes`**, la sonda che il Task 5 aggiunse con `E29` proprio per quel confine: `arbiter_admission.rs` **19 passate, 1 fallita**, `lib` **13 passate, 0 fallite**. Registrato come `E65` |
| **7** | la guardia *«è già in uscita»* (`Revoking → None`) tolta | `asking_back_twice_does_not_buy_the_room_twice` | ✅ **rossa, e SOLA — 12 passate, 1 fallita** |
| **8** | la guardia sulla grazia tolta: `held.grace` diventa `Some(held.grace.unwrap_or(Millis::new(0)))` | `a_non_preemptible_grant_is_never_asked_back` | ⚠️ **rossa, ma NON PIÙ SOLA — 11 passate, 2 fallite.** Cade anche `asking_back_marks_nothing_when_the_reclaimable_…`, perché senza la guardia il suo residente immobile diventa recuperabile. ⛔ **Era «sola» al Task 7 e non lo è più: la dominanza nuova sta qui sotto, scritta invece che taciuta** (`E73`) |
| **9** | `self.collect_expired(now)` tolta da `ask_back` | `ask_back_collects_the_expired_before_it_marks` | ✅ **rossa, e SOLA — 12 passate, 1 fallita** |
| **10** | prima la corsia **migliore** invece della peggiore (`.rev()` tolto) | `asking_back_takes_the_worst_lane_first` | ⚠️ **rossa, ma NON PIÙ SOLA — 11 passate, 2 fallite.** Cade anche `asking_back_crosses_…`: senza `.rev()` la corsia **migliore** va per prima e copre `4_096` da sola, quindi la risposta è `4_096` invece di `6_144`. ⛔ **Era «sola» prima della seconda ondata, e la perdita si scrive: la 10b la rimedia** |
| **10b** | ⚠️ **NUOVA del 2026-08-20, seconda ondata, e CERCATA APPOSTA:** una vittima **più grande del bisogno** viene saltata (`if held.reserved > needed { continue; }`) — cioè la direzione **negativa** della regola dell'eccedenza che la sonda del salto di corsia dichiara | `asking_back_takes_the_worst_lane_first` | ✅ **rossa, e SOLA — 12 passate, 1 fallita.** ⛔ **Restituisce l'uccisore solo** che la sonda nuova le aveva tolto, ed è l'unica riga della campagna che eserciti *«marcare può ECCEDERE il bisogno»*: le due sonde condividono lo scenario e differiscono solo per la **taglia** del bisogno, quindi nessuna mutazione dell'**ordine** le separa — solo una della **scelta della vittima** |
| **11** | ⚠️ **aggiunta**, la seconda direzione su `revoking()`: conta **ogni** concessione prelazionabile invece delle sole revocate | — | ✅ **cinque rosse** — `a_grant_in_the_asking_lane_…`, `asking_back_marks_nothing_…`, `only_lanes_below_…`, `asking_back_stops_…`, `asking_back_takes_the_worst_lane_first`. **8 passate, 5 fallite**: il contatore è tenuto nelle due direzioni, «zero» e «uno» |
| **12** | ⚠️ **NUOVA del 2026-08-20**: la passata di **sola lettura** tolta, cioè `ask_back` torna a marcare mentre cammina | `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need` | ✅ **rossa, e SOLA — 12 passate, 1 fallita.** È la misura che ha **riprodotto il difetto** prima di correggerlo: `` left: Mib(2048), right: Mib(0) ``. Voce `E69` |
| **12c** | ⚠️ **NUOVA del 2026-08-20**, la seconda direzione della guardia nuova: `reclaimable < needed` diventa `<=`, cioè scatta anche a **incastro esatto** | le sonde in cui il recuperabile è **esattamente** il bisogno | ✅ **8 passate, 5 fallite** — `asking_a_grant_back_marks_…`, `a_grace_that_ran_out_…`, `a_grant_inside_its_grace_…`, `the_grace_runs_out_…`, `asking_back_twice_…`. ⛔ È la direzione *«non scatta dove non deve»*: senza questa riga la guardia nuova era provata da un lato solo |
| **13** | ⚠️ **NUOVA del 2026-08-20**: dentro una corsia la vittima diventa la **più recente** invece della più vecchia (`values_mut().rev()`) | — | ⛔ **NESSUNA muore — 13 passate e 20 passate. MUTANTE VIVO**, e per scelta: dentro una corsia l'ordine è una **politica** che nessun documento decide, quindi è **dichiarata nel doc di `ask_back` col mutante vivo accanto** e messa davanti al proprietario, non pinzata. ✅ **RIMISURATA nella seconda ondata contro il codice di allora e non citata** (`E80`): la riga era stata scritta **prima** della riscrittura in due passate, e una misura citata invece che rifatta è il gotcha **#31** — è risultata ancora vera. Voce `E70`, forma di `E50`/`E51`/`E53` |
| **14** | ⚠️ **NUOVA del 2026-08-20, seconda ondata**: il **ciclo esterno** sulle corsie fermato alla prima (`lanes.iter().rev()` → `.take(1)`) | `asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough` | ✅ **rossa, e SOLA — 12 passate, 1 fallita.** ⛔ **Prima della sonda nuova questo mutante era VIVO nell'INTERO WORKSPACE** — **34 target, 235 passate, 0 fallite, 2 ignorate**, ed è il difetto che l'ondata di `E69` esisteva per togliere, preso da una terza strada: **uno sfrattato e nessuno seduto**. Voce `E75` |
| **15** | ⚠️ **NUOVA del 2026-08-20, terza ondata**: il `filter(\|held\| askable(held).is_some())` **tolto dalla catena che costruisce `lanes`** (quello della passata di sola lettura resta) | — | ⛔ **NESSUNA muore — 34 target, 236 passate, 0 fallite, 2 ignorate: identico alla baseline. MUTANTE VIVO, e GARANTITO.** Il filtro non ha comportamento: le corsie che toglie non contengono niente che il ciclo interno marcherebbe, perché quel ciclo richiede `askable` per conto suo. ⛔ **Resta comunque** — è ciò che rende *«l'insieme È lo stesso insieme»* vero **per costruzione** invece che per promessa (`E76`) — e la riga esiste perché una frase tenuta da niente sia **dichiarata** e non scoperta, come la **13** per l'ordine dentro la corsia. Voce `E83` |
| **D0** | ⚠️ **non una mutazione ma la forma che il piano DETTA**, installata per misurarla: `if let …Running` più `match held.grace { Some => …, None => continue }` | — | ✅ **VERDE — 10 passate e 20 passate.** ⏳ **Misura del Task 7, sulla suite di ALLORA, e NON rimisurata:** con le due passate di `E69` la forma dettata non esiste più in una versione confrontabile. La conclusione che ne dipende regge lo stesso — vedi sotto |
| **D1** | sulla forma **dettata**: la guardia `if let …Running` cancellata | — | ✅ **rossa `asking_back_twice_does_not_buy_the_room_twice`, e SOLA — 9 passate, 1 fallita.** ⛔ **`a_non_preemptible_grant_is_never_asked_back` resta VERDE:** a salvarla è il ramo `None`, non la guardia cancellata. ⏳ Misura del Task 7, non rimisurata |
| **D2** | sulla forma **dettata**: `None => continue` diventa `None => now` | — | ⛔ **NESSUNA muore — 10 passate e 20 passate. Mutante VIVO**, e il ramo è **irraggiungibile**: con la guardia `Running` davanti, una concessione non prelazionabile non ci arriva mai. ⏳ Misura del Task 7, non rimisurata |

⛔ **QUALI SONDE UCCIDE OGNI MUTAZIONE, E QUALI RESTANO DOMINATE — misurato, e scritto anche
dove la risposta è scomoda.** La regola nata dal Task 3 del Traguardo 4: quando una mutazione ne
uccide due si cerca **una terza che lasci passare la prima**. **NOVE sonde su quattordici** hanno
un uccisore **solo** — le mutazioni **2b**, **2c**, **3b**, **5c**, **7**, **9**, **10b**, **12**,
**14** — e `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` è **sola nel proprio
file** sotto la **5d**. ⚠️ **Le altre QUATTRO sono DOMINATE sotto questa campagna, e si scrive
invece di lasciarlo intendere:**

| Sonda dominata | Da chi, e perché |
|---|---|
| `asking_a_grant_back_marks_it_and_does_not_free_it_yet` | muore sotto **1**, **5b**, **5d**, **12c** — un sottoinsieme di dove muore `a_grace_that_ran_out_returns_the_reservation_to_the_budget`. Le due condividono lo scenario e differiscono in ciò che asseriscono **alla fine**: *«non ha liberato»* contro *«poi libera»* |
| `a_grace_that_ran_out_returns_the_reservation_to_the_budget` | muore sotto **1**, **4**, **5b**, **5d**, **12c** — un sottoinsieme di `asking_back_twice_does_not_buy_the_room_twice`, che per provare che la scadenza **non si è spostata** riammette a `501` e quindi contiene la stessa domanda come sotto-fatto |
| `a_grant_inside_its_grace_keeps_its_reservation` | muore sotto **1**, **5**, **5b**, **5d**, **12c** — anch'essa un sottoinsieme di `asking_back_twice`, per la stessa ragione |
| `a_non_preemptible_grant_is_never_asked_back` | ⚠️ **DOMINANZA NUOVA DEL 2026-08-20, e va detta perché è una PERDITA:** muore sotto **8** e **5b**, un sottoinsieme di `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need` (**5b**, **5d**, **8**, **11**, **12**). Al Task 7 la **8** era il suo uccisore **solo**; la sonda nuova di `E69` porta anch'essa un residente **non prelazionabile**, quindi da oggi la **8** ne uccide due |

⚠️ **E UNA QUINTA HA PERSO L'ESCLUSIVITÀ E SE L'È RIPRESA NELLO STESSO GIRO, che è il ciclo che
questa regola descrive e vale la pena vedere per intero.** `asking_back_takes_the_worst_lane_first`
moriva **sola** sotto la **10**; la sonda del salto di corsia, che condivide lo scenario, muore
sotto la **10** anche lei — quindi l'esclusività se n'era andata, e la sonda **non** era per questo
dominata: il suo insieme (**1**, **3**, **5b**, **5d**, **10**, **11**) non è contenuto in quello
di nessun'altra. ✅ **Cercata una candidata isolante e TROVATA — la 10b**, che non tocca l'ordine
ma la **scelta della vittima**: le due sonde differiscono per la **taglia del bisogno**, non per
l'ordine, quindi nessuna mutazione dell'ordine poteva separarle e una della scelta sì. È la terza
volta che la regola *«quando una mutazione ne uccide due si cerca una terza»* produce una riga
nuova — dopo la **3b** e la **2c** |

⛔ **Che cosa se ne fa, e che cosa NON se ne fa: restano tutte e quattro.**
E «dominata» non vuol dire «superflua». ⛔ **RICHIAMO DEL 2026-08-28 (AUD-013):** qui c'era
scritto che la **5** doveva a `a_grant_inside_its_grace_keeps_its_reservation` *«la propria
unica morte utile»* e che *«senza di lei»* la mutazione era un mutante vivo — falso in tutt'e
due le metà, e smentito dalla riga **5** e dalla tabella della dominanza qui sopra, dove
l'insieme di morte di questa sonda è un **sottoinsieme** di quello di `asking_back_twice_…`:
ogni mutazione che uccide lei uccide **anche** l'altra. ⚠️ **È la stessa frase che AUD-013
ha corretto nella tabella delle sonde, e che lì si era fermata** — gotcha **#68**, la casa che
il censimento non aveva aperto. La ragione per cui le quattro restano è il punto ①, non
un'esclusività.
① Ciascuna dice una cosa che nessun'altra dice a voce alta, ed è ciò che si legge quando una va
rossa: *«marca e non prende»*, *«poi riscuote»*, *«dentro la grazia non riscuote»*, *«quello che
non si può chiedere indietro non si tocca»*. ② ⚠️ **Ma la loro non-ridondanza NON è misurata**,
ed è registrata per quello che è — un'intenzione finché una mutazione non le isola. ✅ **DUE
candidate isolanti sono state cercate e hanno funzionato**: la **3b**, che ha rotto la dominanza
di `asking_back_stops_as_soon_as_the_need_is_covered`, la **2c** del 2026-08-20, che ha
**restituito** l'uccisore solo a `only_lanes_below_the_asking_one_are_asked_back` dopo che la
sonda del confine gliel'aveva tolto, e la **10b** della seconda ondata dello stesso giorno, che ha
fatto lo stesso per `asking_back_takes_the_worst_lane_first`. ⛔ **Per `a_non_preemptible_grant_is_never_asked_back` è
stata cercata e NON trovata, e la ricerca si registra invece dell'esito che si sarebbe voluto:**
le due sonde differiscono per la **presenza di un secondo candidato recuperabile**, e nessuna
mutazione della guardia sulla grazia distingue i due casi — riscrivere lo scenario dell'una per
farle divergere sarebbe piegare la sonda alla campagna. Per le altre tre non se n'è trovata
nessuna, e la ricerca si registra invece dell'esito che si sarebbe voluto.

⛔ **Le tre righe `D` sono la ragione per cui il corpo di `ask_back` DIVERGE dal piano, e la
divergenza è una misura e non un gusto.** Sulla forma dettata la direzione *«una concessione non
prelazionabile non si chiede indietro»* è tenuta da **due guardie che si mascherano a vicenda**:
cancellare l'una la lascia coperta dall'altra (`D1`), e rompere l'altra non è nemmeno
raggiungibile (`D2`). Una direzione che nessuna mutazione riesce a mostrare non è provata, è
**assunta**. Nella forma scritta le due guardie rispondono a **due domande diverse** — *«è già in
uscita»* e *«si può chiedere indietro»* — e ciascuna ha la **propria sonda**, le righe **7** e
**8**. Voce `E62`. ⚠️ **AGGIORNATO IL 2026-08-20, IN DUE PUNTI CHE VANNO IN DIREZIONI DIVERSE:**
① le guardie sono **tre** e non due — corsia, «già in uscita», grazia — e vivono in **un posto
solo**, la chiusura `askable` che entrambe le passate di `E69` interrogano; restano però
**separate**, che è esattamente ciò che `D1` e `D2` avevano mostrato servire. ② ⛔ **La riga 8
non ha più un uccisore solo**, per la ragione scritta nella tabella delle dominate: la sonda
nuova di `E69` porta anch'essa un residente non prelazionabile. ⏳ Le tre righe `D` **non sono
state rimisurate**: misurano la forma **dettata dal Passo 3**, che con le due passate non
esiste più in una versione confrontabile (`E73` ③).

⛔ **DUE AVVISI `dead_code` RESTANO, E SONO REGISTRATI PER IL PROPRIETARIO invece che spenti.**
`cargo build --locked --workspace` stampa `` warning: fields `lane` and `grace` are never read ``
e `` warning: method `ask_back` is never used ``: `ask_back` è `pub(crate)`, il suo unico
chiamante di produzione nasce al **Task 8**, e un modulo `#[cfg(test)]` non conta per `dead_code`
in una build non di test. ⛔ **Nessun `#[allow]`** (gotcha #13), **nessun lettore inventato**,
nessun `pub` di comodo: pubblicare `ask_back` per far tacere l'avviso pubblicherebbe
un'operazione che non lo è. Il cancello resta **verde** — non passa `-D warnings`. ⏳ **SCADENZA,
ed è la parte falsificabile: al Task 8 quei due avvisi DEVONO sparire**, perché la policy LOCALE
chiama `ask_back`, che legge `lane` e `grace`. Chi esegue il Task 8 lo verifica con
`cargo build --locked --workspace` e si aspetta **zero warning**; se sono ancora lì, il metodo non
serviva e **si toglie**. È la forma esatta di `E10` al Task 4, e la voce è `E67`. ✅ **E DAL
2026-08-20 LA SCADENZA STA ANCHE ACCANTO AL CODICE**, in inglese in coda al doc di `ask_back`,
che è ciò che a `E67` mancava e che `E10` aveva: una scadenza che vive solo nell'errata è una
scadenza che chi lavora nel sorgente non vede. Voce `E74`. ⏳ **Richiamo del 2026-08-20 — LA
SCADENZA È SCATTATA ED È STATA ONORATA**, e ciò che sta accanto al codice **non è più una
scadenza** ma il suo verbale: il Task 8 ha dato ad `ask_back` il chiamante di produzione e i due
avvisi sono spariti. ⛔ **Ma il testo è rimasto al presente fino all'ondata di correzioni dello
stesso giorno**, dove ordinava di **togliere** il metodo appena diventato la spina dorsale di
`LocalPolicy`. Voci `E91` e `E99`.

⚠️ **NESSUNA RIGA NUOVA È STATA AGGIUNTA AL CATALOGO.** §7.4 è **spec**, e aggiungere una riga è
una decisione del **proprietario** (vincolo globale 7). `I2 · §5.3` resta coperta come al Task 4 —
*«non rappresentabile»* dal caso negativo, *«costruibile dove è lecito»* da
`a_revocation_is_constructible_on_the_preemptible_side` — e ciò che il Task 7 aggiunge è che
l'arbitro **non ci prova nemmeno** a runtime, che è più di quanto la riga pretenda. Registrato
nell'errata.

📌 **Conteggi, ricontati eseguendo il binario e mai dedotti** (gotcha #31): workspace **34 target,
236 passate, 0 fallite, 2 ignorate**. ⚠️ **Erano 222 all'inizio del compito e 233 alla sua
chiusura** — le undici di allora sono le dieci sonde del `mod tests` e la sola aggiunta a
`arbiter_admission.rs`; le **due** della prima ondata di correzioni del 2026-08-20 sono
`a_grant_in_the_asking_lane_itself_is_not_asked_back` (`E71`) e
`asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need` (`E69`), e la **terza**,
della seconda ondata dello stesso giorno, è
`asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough` (`E75`). ⛔ **I TARGET
NON SI SONO MOSSI, contro l'attesa:** il pre-controllo avvertiva che un `mod tests` dentro `src`
fa nascere un bersaglio di prova nuovo, e **il bersaglio c'era già** — `cargo test --workspace`
eseguiva `unittests src\lib.rs` per `kernel` con **zero** test, e oggi ne esegue tredici. La
divergenza si registra invece di appianarla. Per file: `arbiter_admission.rs` **venti** (erano
diciannove), il bersaglio `lib` di `kernel` **tredici** (erano zero, dieci alla chiusura del
compito e dodici dopo la prima ondata), `arbiter_resource.rs` **otto**
(invariato), e i casi `compile_fail` **ventotto** — ✅ ricontati col comando,
`ls crates/kernel/tests/compile_fail/*.rs | wc -l` → **28**, invariati: il Task 7 non ne ha
aggiunto nessuno e non ha rigenerato nessun oracolo.

#### Le due policy VRAM — Traguardo 5, Task 8, e `V3` si copre a metà

⛔ **DUE OGGETTI CON UN'INTERFACCIA, E LA DIFFERENZA È UNA DECISIONE SOLA DENTRO L'AMMISSIONE.**
`crates/kernel/src/arbiter/policy.rs` porta il tratto `MakeRoom` con **una** domanda —
*«una richiesta non entra. Si può fare spazio?»* — e i due oggetti che rispondono: `RemotePolicy`
**no**, `LocalPolicy` **sì**. È il punto in cui ADR-0006 dice che sarebbe finito il condizionale
sull'origine dell'inferenza, e non c'è: l'`if` in `Arbiter::admit` chiede **alla policy**, non
all'origine, e ne esiste **uno solo** in tutta la crate.

⛔ **E AL TRAGUARDO 5 NON SONO GUSCI VUOTI, che era la domanda aperta.** *«Sfrattare un
residente»* **è** *«revocare una concessione prelazionabile»* — il meccanismo del Task 7 — quindi
le due policy si provano oggi con concessioni sintetiche dichiarate dal banco, senza nessun
modello e senza nessuna speculazione. `LocalPolicy` chiama `ask_back` col bisogno esatto
(`allocated + asked - ceiling`) e con la **propria** corsia come confine; poi, in entrambi i casi,
la richiesta si **accoda**: la stanza non è libera finché il titolare non consegna, e il ritorno
di `ask_back` è **deliberatamente non letto** — agire su quel numero vorrebbe dire sedere un
secondo consumatore sulla VRAM che il primo sta ancora usando, cioè proprio ciò che la grazia
della §5.3 punto 4 esiste per impedire.

⛔ **`enqueue` È NATO QUI, E NON ERA NEL PIANO: il corpo dettato dal Passo 3 NON COMPILAVA.** Il
ramo finiva con `return self.enqueue(profile, valid_for);` e quella funzione non esisteva —
l'accodamento era scritto **in linea** dentro `admit` dal Task 6. Estratto invece che ricopiato,
e ⛔ **l'estrazione non doveva cambiare niente**: stessi campi, stesso ordine, stesso contatore.
✅ **Provato eseguendo e non a mano** — `crates/kernel/tests/arbiter_admission.rs` resta a
**venti** sonde passate e il `#[cfg(test)] mod tests` del `lib` a **tredici**, gli stessi
identici numeri di prima del compito — e ✅ **provato anche che il corpo estratto sia VIVO**:
righe **11a** e **11b** della campagna, che lo rompono in due punti diversi e fanno rosso in
`arbiter_admission.rs`. Voce `E87`.

⛔ **LA SCADENZA DI `E67`/`E74` ERA QUI, ED È STATA ONORATA.** Il Task 7 lasciò **due** avvisi
`dead_code` — `` fields `lane` and `grace` are never read `` e `` method `ask_back` is never
used `` — accettati dal proprietario **con la scadenza a questo compito**, e con la regola
scritta accanto al codice: *«se al Task 8 sono ancora lì, il metodo non serviva e si toglie»*.
✅ **Misurato con `cargo build --locked --workspace`: ZERO avvisi.** `admit` sotto `LocalPolicy`
è il chiamante di produzione di `ask_back`, e `ask_back` legge `lane` e `grace`. Voce `E91`.

⛔ **MA LA SCADENZA ERA RIMASTA SCRITTA ACCANTO AL CODICE, E QUESTO COMMIT L'AVEVA RESA FALSA.**
Il commit del compito **non tocca** il doc di `ask_back`: l'ordine *«al Task 8 quei due avvisi
devono sparire, e se sono ancora lì il metodo non serviva e si toglie»* è rimasto lì **al
presente**, insieme alla premessa su cui poggiava la scelta chiusura-invece-di-metodo
(*«`ask_back` non ha chiamanti di produzione, quindi è `dead_code`, e ciò di cui è l'unico
chiamante è morto con lui»*). ⛔ **Chi arriva dopo vi legge l'ordine di TOGLIERE il metodo appena
diventato la spina dorsale di `LocalPolicy`** — un documento che mente con autorevolezza, e non
una rifinitura.
⛔ **E NON ERA UNA DIMENTICANZA: era un'affermazione falsa, in DUE posti.** Il rapporto del
compito diceva *«la scadenza è stata **tolta** dal doc di `ask_back` e sostituita col fatto»*, e
la stessa frase chiudeva la voce `E91`. ✅ **Riscritte — non affiancate — nell'ondata di
correzioni del 2026-08-20**, e la premessa scaduta è stata **rimisurata invece che dedotta**: con
un aiutante privato di `impl Held` raggiungibile **solo** da dentro `ask_back`,
`cargo build --locked --workspace` stampa **zero** avvisi, perché `admit` arriva ad `ask_back` e
niente dietro di lui è più morto. ⚠️ **E la §6 del compendio NON è stata toccata da
quest'ondata**, deliberatamente: quando è stata scritta questa riga il Task 8 vi era ancora
*«il prossimo passo»*, perché il commit di
consegna non c'era, e riallineare due frasi dentro una sezione ferma a un compito prima l'avrebbe
lasciata **più** contraddittoria, non meno (gotcha **#68**). ✅ **Quella sezione si è mossa come un
blocco alla consegna del 2026-08-20**, che è ciò che questa riga prevedeva. Voce `E99`.

⛔ **`E41`/`E51`/`E53` NON SI SONO MOSSE, MA LA LORO INTERAZIONE HA CAMBIATO SPECIE CON QUESTO
COMMIT: da TEORICA a RAGGIUNGIBILE IN PRODUZIONE.** `admit` riscuote e concede subito se c'è
posto, **senza guardare le code** (`E51`). Fino al Task 7 `ask_back` non aveva chiamanti di
produzione, quindi **nessuna revoca avveniva mai** fuori da una sonda e la stanza che una revoca
libera non esisteva: non c'era niente da rubare. Sotto `VramPolicy::Local` c'è — `LocalPolicy`
chiede indietro un residente **per** un biglietto in coda, la spazzata libera quella riserva alla
scadenza della grazia, e il primo `admit` **diretto** che passa si siede sulla stanza fatta per un
altro, mentre il biglietto per cui è stata fatta resta nella sua corsia.
✅ **Misurato nelle DUE direzioni il 2026-08-20**, su una sonda usa-e-getta cancellata subito
dopo, perché un'affermazione sul comportamento che nessuna sonda esercita è la specie che questo
traguardo ha già pagato. **LOCAL:** residente `Batch` da 4_096 chiesto indietro per un biglietto
`Interactive` in coda, poi a `501` una richiesta `Batch` **nuova** da 4_096 → **`Granted`**, il
`promote` che segue torna **vuoto**, e `queued()` è ancora **1**. **REMOTE** — che è esattamente il
mondo prima di questo compito, visto che nessuno veniva mai chiesto indietro: lo stesso
ritardatario è **`Queued`** e `queued()` è **2**, cioè non si era liberato niente e non c'era
niente da prendere.
⛔ **NESSUNA SONDA LO TIENE, ed è dichiarato invece che lasciato scoprire** — la forma di
`E50`/`E51`/`E53`: pinzarlo congelerebbe la scelta che quelle voci mettono davanti al
proprietario, e una sonda che va cancellata per prendere una decisione è un voto contro il
prenderla. ⚖️ **Il chiusore è chi costruisce il primo ciclo di orchestrazione**, ed è ancora una
decisione di **orchestrazione**; ciò che il Task 8 cambia è solo che il costo di lasciarla aperta
si paga ora **in produzione** e non sulla carta. Il fatto è enunciato anche accanto ad `admit`,
nel sorgente. ⚠️ **Richiamo del 2026-08-21 — qui c'era *«il chiusore resta il TASK 10»*, e il
Task 10 si è chiuso senza costruirne nessuno.** Voce `E100`.

| Sonda — `crates/kernel/tests/arbiter_policy.rs` | Cosa tiene |
|---|---|
| `the_remote_policy_does_not_make_room_it_queues` | il **default** di ADR-0006: la macchina piena, una richiesta che non entra, e `revoking()` a **zero**. Nessuno viene chiesto indietro |
| `the_local_policy_asks_the_lower_lanes_back` | lo **stesso identico scenario** con l'altro oggetto: `revoking()` a **uno**, e la risposta resta `Queued` — chiedere **marca**, non prende |
| `under_the_local_policy_the_queued_request_is_served_past_the_grace` | e che la marcatura **porti da qualche parte**: oltre la grazia il biglietto è servito, con il **suo** `TicketId`, e i libri restano a una concessione sola |
| `under_the_remote_policy_the_same_clock_advance_serves_nobody` | la contro-sonda della precedente: **stesso avanzamento d'orologio**, e non si libera niente perché non è stato chiesto niente |
| `each_policy_names_itself` | il nome, che la transizione giornalata della §5.4 dovrà scrivere — letto **due volte**, dall'enum e **attraverso l'arbitro**. La seconda metà è la sola cosa che tiene *«l'arbitro ha conservato la policy con cui è stato costruito»* |
| `a_partly_full_machine_asks_back_the_need_and_not_the_whole_request` | ⛔ **AGGIUNTA NELL'ONDATA DI CORREZIONI DEL 2026-08-20**: il **primo** dei due argomenti che `admit` calcola. Macchina **parzialmente** piena — `ceiling` 4_096, `allocated()` 3_072, `asked` 2_048, `needed` 1_024, tutti diversi — quindi `needed` è finalmente distinguibile da `asked`. Voce `E97`. ⚠️ **Il residente `Realtime` è PRELAZIONABILE come ogni altro residente di questo file** dalla seconda ondata dello stesso giorno: a tenerlo fuori dai recuperabili è la **corsia** e non la prelazionabilità (`E105`) |
| `the_admission_asks_back_below_its_own_lane_and_spares_a_peer` | ⛔ **AGGIUNTA LO STESSO GIORNO**: il **secondo** argomento, la corsia. Un pari `Interactive` **prelazionabile** non viene sfrattato per un `Interactive`, e la policy aveva detto **sì** — a fermare l'arbitro è il confine, non la policy. Voce `E97`. ⚠️ **Richiamo del 2026-08-20 — questa cella diceva *«è la direzione «non scatta dove non deve» della riga sopra»*, e le due righe non sono le due direzioni di UNA regola: sono DUE regole.** Quella sopra tiene il **primo** argomento nella direzione *«scatta dove deve»*; questa tiene il **secondo** nella direzione *«non scatta dove non deve»*. Le direzioni opposte esistono, ma **altrove**: per lo **scarto** in `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`, per la **corsia** in `the_local_policy_asks_the_lower_lanes_back` (cablaggio) e in `only_lanes_below_the_asking_one_are_asked_back` (meccanismo). Voce `E110` |

⛔ **L'AIUTANTE `never_preemptible` DELLE DUE SONDE NUOVE NON TENEVA NIENTE, ED È UN MUTANTE VIVO
CHE L'ONDATA DI CORREZIONI SI ERA PORTATA DENTRO DA SÉ.** Il suo doc dichiarava che le due sonde
*«ne hanno bisogno: un residente che NON può essere chiesto indietro è ciò che lascia la macchina
solo parzialmente recuperabile»*, e in **nessuna** delle due il residente veniva mai guardato per
la prelazionabilità: sta in corsia `Realtime` e il richiedente in `Interactive`, quindi `askable`
lo scarta sul **primo** dei tre controlli — `if held.lane <= below` — e `held.grace`, l'unica cosa
che `Preemption::Never` cambia, non viene **mai letto**. ✅ **MISURATO PRIMA DI SCEGLIERE, il
2026-08-20:** sostituendo `Preemption::Never` con `Preemption::After(Millis::new(500))` dentro
l'aiutante il banco resta **7 passate, 0 fallite** — mutante vivo, e nessun altro usava
l'aiutante. ⛔ **Tolto, e i due residenti resi `preemptible`**: sparisce insieme una duplicazione
verbatim (`never_preemptible` era `preemptible` a meno di un campo) e una frase falsa, e i due
residenti restano al riparo per la sola ragione vera — la **corsia** — che è già il motivo per cui
il pari di `the_admission_asks_back_below_…` è deliberatamente prelazionabile.
⚠️ **E l'attribuzione in linea era sbagliata a sua volta:** il commento diceva che a rendere lo
scarto (`1_024`) minore della richiesta (`2_048`) fosse la prelazionabilità del residente, e a
renderlo tale è che la macchina è **parzialmente libera** — `allocated + asked - ceiling` con
`allocated` 3_072 su un tetto di 4_096.
⚠️ **E UNA TESI SCRITTA PRIMA DELLA MISURA È CADUTA, registrata invece che taciuta:** l'attesa era
che l'aiutante fosse **dannoso** e non solo inerte — che con `Never` una mutazione del confine da
`held.lane <= below` a `held.lane < below` restasse invisibile. ✅ Misurata: quella mutazione
uccide `the_admission_asks_back_below_its_own_lane_and_spares_a_peer` **anche** con l'aiutante in
piedi (banco **6/1**), perché a diventare recuperabile è il **pari** e non il residente `Realtime`
— ed è uccisa pure nel `lib`, da `a_grant_in_the_asking_lane_itself_is_not_asked_back` (12/1).
L'aiutante era **inerte**, non dannoso. ⛔ **Nessun `Preemption::Never` resta in questo banco, e
non è una perdita di copertura:** la variante è esercitata in
`crates/kernel/tests/arbiter_admission.rs`, nel `#[cfg(test)] mod tests` del `lib`, in
`crates/kernel/tests/arbiter_resource.rs` e in due casi `compile_fail`. Voce `E105`.

⛔ **`Arbiter::policy()` NASCEVA SENZA CONSUMATORE, E NON L'AVREBBE DETTO NESSUNO** — è `pub`,
quindi `dead_code` tace: è il gotcha **#46** dal verso sbagliato. Gli è stato dato un consumatore
**vero** invece di essere rimandato: le due righe di `each_policy_names_itself` che leggono il
nome attraverso l'arbitro. ✅ **Misurato che serva:** con `policy()` che restituisce un
`VramPolicy::Remote(RemotePolicy)` fresco invece di quello conservato, quella sonda è **rossa**
(riga **9**).

⛔ **`V3` SI COPRE A METÀ QUI, E IL CASO NEGATIVO NON È QUELLO CHE IL PIANO DETTAVA.** ⚠️ **Questa
sezione diceva *«si chiude»* e contava la riga fra le coperte:** corretto il 2026-08-20 — la
contro-sonda di catalogo è doppia e la sua seconda metà è del **Task 9**, che la porta e chiude la
riga. ⚠️ **Il numeratore del blocco C è TOLTO da questa frase e non riallineato** (2026-08-20,
Task 9): diceva *«quattordici su diciannove»*, e un compito dopo era falso. **La cifra si riconta
sulla cella del blocco C in fondo a questo file**, e un rimando non può marcire
(gotcha **#68**, la stessa regola che `E102` ha applicato al conteggio delle mutazioni). `E103`.
Il piano scriveva
`VramPolicy::Remote(..) | VramPolicy::Local(..)`, che prova che `VramPolicy` non implementa
`BitOr`. ✅ **Misurate entrambe le forme, e la specie di ciascuna sotto la mutazione che la regola
teme davvero** — un `Arbiter::new` che **accetti** due policy:

| Forma del caso | Sotto la mutazione «`new` accetta due policy» | Specie |
|---|---|---|
| `BitOr` fra due policy — **la forma dettata** | ⛔ resta **`ok`**: non nomina `Arbiter::new`, quindi la regressione le è **invisibile** | nessuna: non scatta |
| **arità** — tre argomenti a `Arbiter::new` | ✅ passa a **`error`**: il caso comincia a compilare e `trybuild` lo dice **fuori dall'oracolo** | la forte, gotcha **#42** |

⛔ **Scelta la seconda, e la prima NON è stata tenuta accanto:** un caso che non può scattare per
la ragione per cui esiste è una guardia che si legge come tale e non lo è. È la stessa forma con
cui `E23` chiuse `V2` col caso di arità `admission_without_profile.rs`. Voce `E89`.
⚠️ **E il limite è dichiarato prima che qualcuno lo scopra, e MISURATO:** il caso pinza l'**arità
di `new`**, non l'assenza di ogni strada verso due policy. ✅ Con un secondo costruttore —
`pub const fn new_with_two(parameters, a, _b)` — il caso resta **`ok`** e l'intera suite resta
verde. A chiudere quella strada è la revisione, non il compilatore.

⛔ **TRE DEI SITI DI `Arbiter::new` ERANO CASI `compile_fail` CON L'ORACOLO CHE PINZA UN
NUMERO DI RIGA** — `admission_has_no_is_granted.rs` (riga **26**),
`admission_reads_cold_start.rs` (**27**), `admission_without_profile.rs` (**41**). Modificati **a
parità di righe**, l'argomento nuovo dentro la chiamata esistente: ✅ **ZERO oracoli rigenerati e
zero righe di oracolo cambiate**, verificato con `git status --porcelain` e non con `git diff`,
che i non tracciati non li vede (`E3`, `E11`). L'unico `.stderr` nuovo è quello del caso di `V3`,
**venti righe**, generato una volta e **letto**. ✅ Casi ricontati col comando —
`ls crates/kernel/tests/compile_fail/*.rs | wc -l` → **29**, erano ventotto. Voce `E90`.
⚠️ **Quanti siti, e in quanti file, NON è scritto qui:** la misura vive in un posto solo, la
voce `E101` del piano (gotcha **#68**, voce `E112`).

⚠️ **REGISTRATA, NON PRESA — LA POLICY È UN SECONDO VALORE CONSEGNATO, E LA §2.8.2 NE PARLA AL
SINGOLARE.** La regola 1 di §2.8.2 dice che il kernel *«riceve alla costruzione **un valore** che
porta i parametri risolti»*, e la conseguenza gratuita che fonda `V3` è scritta così: *«se **il
valore consegnato** porta **una** policy, "due policy attive" non è rappresentabile»*. ADR-0034
elenca *«quale policy VRAM è attiva»* fra i parametri di quel valore. Il piano consegna invece la
policy come **secondo argomento** di `Arbiter::new`, accanto a `Parameters`. ⚖️ **Non è un
indebolimento** — l'arbitro la riceve comunque alla costruzione, non legge nessuna
configurazione, e `VramPolicy` essendo un enum ne porta **una** comunque — ma i due testi non
dicono la stessa cosa, e la scelta è del proprietario: ① **lasciarlo com'è**, e la §2.8.2 va
letta come *«i valori consegnati»*; ② **spostare la policy dentro `Parameters`**, che è il testo
alla lettera e costa un campo su un tipo che la §2.8 pinza, più i **diciannove** chiamanti di
`Parameters::new` che `E18` ha contato. Voce `E94`.

⛔ **LA CAMPAGNA DI MUTAZIONE, e la 12 e la 13 esistono per una misura di isolamento e non per
uccidere.** ⚠️ **Richiamo del 2026-08-20 — questa frase diceva *«le ultime due righe»*, e le
ultime due sono ora la 14 e la 15, che sono uccisori soli nell'intero workspace:** la **prima**
ondata ha aggiunto due righe **in coda** e ha lasciato in testa un puntatore posizionale, che è
la forma di gotcha **#31** che invecchia più in fretta di una cifra. Le righe di isolamento si
nominano, non si contano dal fondo (`E110`).
⚠️ **Il conteggio delle righe è TOLTO da questa frase e non riallineato**
(2026-08-20): diceva *«tredici»*, mentre il rapporto del compito diceva *«sedici»* — due
documenti in disaccordo, e **nessuno dei due che tornasse**. La tabella qui sotto **è** la
misura, e un rimando non può marcire (gotcha **#68**).
Voce `E102`. Ogni mutazione: entrata provata col conteggio delle occorrenze,
**compilata in un passo separato** da quello che esegue, revocata da copia byte-esatta con `cmp`
identico e `git diff` vuoto; zero CR prima e dopo. ⚠️ **L'esclusività è SEMPRE dichiarata col
proprio perimetro** — *«sola nel banco»* non è *«sola nel workspace»*, ed è la lezione di `E82`:
un'esclusività misurata su un campione parziale si legge come una garanzia.

⛔ **L'INTERA TABELLA È RIMISURATA UNA SECONDA VOLTA IL 2026-08-20, perché il BANCO è cambiato
un'altra volta.** La seconda ondata di correzioni ha tolto dal banco l'aiutante `never_preemptible`
e ha reso `preemptible` i due residenti `Realtime`; un banco che cambia invalida ogni cella
misurata su di esso, quindi **tutte** le righe sono state rieseguite invece di essere riportate
(`E104` per il precedente, gotcha **#31** per la ragione). ✅ **Esito: nessuna cella si è mossa** —
gli ingressi danno gli stessi conteggi e gli stessi insiemi di morte di prima, e le
righe **14** e **15** restano sole nell'intero workspace a **242 passate, 1 fallita**. È la prova
misurata che la rimozione dell'aiutante non ha comportamento: `askable` scarta i due residenti
`Realtime` sul **primo** dei tre controlli — `held.lane <= below` — e `held.grace`, l'unica cosa
che `Preemption::Never` cambiava, non veniva mai letto. Voce `E105`.

⛔ **RICHIAMO DEL 2026-08-28, FINDING AUD-023 — QUESTA TABELLA È UN VERBALE DEL BANCO DI SETTE, E
IL BANCO OGGI È PIÙ GRANDE.** Le celle sono state misurate il **2026-08-20** su un
`arbiter_policy.rs` di **sette** sonde; oggi ne ha **dodici** — `grep -c '^#\[test\]'
crates/kernel/tests/arbiter_policy.rs` — e le due che il Task 9 ha aggiunto **cadono sotto le
mutazioni 9 e 10**. ⛔ **La regola che questo documento si dà tre capoversi più su — *un banco che
cambia invalida ogni cella misurata su di esso* — vale ANCHE per questa passata:** le cifre
restano perché un verbale datato le regge, ma **non descrivono il workspace di oggi**.

⚖️ **E CIÒ CHE NON È STATO RIMISURATO VA DETTO PRIMA DI FIDARSI:** il 2026-08-28 sono state
rieseguite **le mutazioni 9 e 10 soltanto**, quelle che il finding nomina. Le righe **14** e
**15** — che il capoverso sopra dichiara *«sole nell'intero workspace»* — **non sono state
riprovate**, e la loro esclusività è quella del 2026-08-20, non di oggi.

| # | Mutazione, sul codice di produzione | Sonda attesa morta | Misurato il 2026-08-20 — banco **di SETTE** · `arbiter_admission` venti · `lib` tredici |
|---|---|---|---|
| **1** | `RemotePolicy::may_make_room` restituisce `true` | `the_remote_policy_does_not_make_room_it_queues` e `under_the_remote_policy_…` | ✅ **le due REMOTE — 5 passate, 2 fallite.** La prima muore sull'asserzione di `revoking()`. ⚠️ **`arbiter_admission` 20/0 e `lib` 13/0**: nessuna sonda preesistente se ne accorge |
| **2** | `LocalPolicy::may_make_room` restituisce `false` | `the_local_policy_asks_the_lower_lanes_back` | ⛔ **ne uccide TRE — 4 passate, 3 fallite:** oltre a quella attesa muoiono `under_the_local_policy_the_queued_request_is_served_past_the_grace`, che il piano non nomina (divergenza `E92`), e `a_partly_full_machine_asks_back_the_need_and_not_the_whole_request`, che nasce con l'ondata di correzioni |
| **3a** | la domanda tolta da `admit`, **nessuno** fa spazio (il blocco `if` reso irraggiungibile) | come la **2** | ✅ **4 passate, 3 fallite** — `the_local_policy_…` (`revoking()` `left: 0, right: 1`), `under_the_local_policy_…` (`promoted.len()`) e `a_partly_full_machine_…` (`revoking()` `left: 0, right: 1`). La direzione *«scatta dove deve»*. ⚠️ **Richiamo del 2026-08-20 — questa cella diceva *«le tre sonde LOCAL»*, e le sonde che girano uno scenario `LocalPolicy` sono QUATTRO:** `grep -n "VramPolicy::Local(LocalPolicy)"` sul banco dà **quattro** corpi di sonda più le due letture di `each_policy_names_itself`, che non ammette niente. La quarta, `the_admission_asks_back_below_its_own_lane_and_spares_a_peer`, **sopravvive e ha ragione di sopravvivere**: asserisce `revoking() == 0`, che è anche ciò che *«nessuno fa spazio»* produce. Il **conteggio** era giusto, l'**etichetta** no. Voce `E108` |
| **3b** | la domanda tolta da `admit`, **tutti** fanno spazio (la condizione resa sempre vera) | — | ✅ **le due sonde REMOTE — 5 passate, 2 fallite.** La direzione *«non scatta dove non deve»*, che è la metà che si dimentica. ⚠️ **E `arbiter_admission` resta 20/0 e `lib` 13/0** anche qui: il ramo nuovo non è tenuto da niente fuori da questo banco (`E96`) |
| **4** | ⚠️ **il separatore che il PIANO indica**: `promote` percorre le corsie al contrario | `under_the_remote_policy_…` rossa, `the_remote_policy_…` verde | ⛔ **NESSUNA delle sette muore — 7 passate, 0 fallite.** Con una sola richiesta in attesa l'ordine fra corsie non decide niente. La riga uccide una sonda del Task 6 in `arbiter_admission` (19/1) e **non separa** la coppia della **1**. Divergenza `E92` |
| **5** | il controllo di capienza dentro `promote` tolto: promuove senza stanza | — | ✅ **`under_the_remote_policy_the_same_clock_advance_serves_nobody`, e SOLA NEL BANCO — 6 passate, 1 fallita**, sull'asserzione `promoted.is_empty()`. È il separatore che la **4** doveva essere. Fuori dal banco: `arbiter_admission` 16/4 |
| **6** | `promote` non promuove niente (il controllo di capienza sempre vero) | — | ✅ **`under_the_local_policy_the_queued_request_is_served_past_the_grace`, e SOLA NEL BANCO — 6 passate, 1 fallita.** Separa la coppia che la **2** uccide insieme. Fuori dal banco: `arbiter_admission` 14/6 |
| **7** | `revoking()` risponde sempre **0** | — | ⛔ **ne uccide DUE — 5 passate, 2 fallite:** `the_local_policy_asks_the_lower_lanes_back` e `a_partly_full_machine_…`. ⚠️ **NON È PIÙ UN UCCISORE SOLO**, e lo era fino all'ondata di correzioni (4/1): la sonda nuova legge `revoking()` anche lei. Divergenza `E104`. Fuori dal banco: `lib` 7/6 |
| **8** | `revoking()` conta **ogni** concessione nei libri | — | ⛔ **ne uccide TRE — 4 passate, 3 fallite:** `the_remote_policy_does_not_make_room_it_queues` e le **due** sonde nuove. ⚠️ **NON È PIÙ UN UCCISORE SOLO**, e lo era (4/1), per la stessa ragione della **7**. Divergenza `E104`. Fuori dal banco: `lib` 5/8 |
| **9** | `policy()` restituisce un `VramPolicy::Remote(RemotePolicy)` fresco invece di quello conservato | — | ✅ **`each_policy_names_itself`, e SOLA NELL'INTERO WORKSPACE — 6/1, `arbiter_admission` 20/0, `lib` 13/0.** Muore sull'asserzione che legge il nome **LOCAL attraverso l'arbitro**. ⛔ **RIMISURATA IL 2026-08-28 (AUD-023): l'esclusività NON vale più.** Sul banco di oggi ne uccide **due** — `265 passate, 2 fallite` — perché `a_policy_transition_writes_its_intent_before_its_outcome`, aggiunta dal Task 9 allo **stesso** banco, cade con lei. ⚠️ **L'esclusività è un'affermazione sull'INSIEME delle sonde, non sulla mutazione:** invecchia quando l'insieme cresce, e chi aggiunge una sonda non apre la tabella di un compito precedente |
| **10** | il dispatch di `VramPolicy::name` risponde `"remote"` per **entrambi** i bracci | — | ✅ **`each_policy_names_itself`, e SOLA NELL'INTERO WORKSPACE — 6/1, 20/0, 13/0.** Muore sull'asserzione che legge il nome **LOCAL dall'enum**: le due metà della sonda stanno su **assi diversi**, ed è per questo che ci sono entrambe. ⛔ **RIMISURATA IL 2026-08-28 (AUD-023): l'esclusività NON vale più** — stesso esito della riga 9, `265 passate, 2 fallite`, con `a_policy_transition_writes_its_intent_before_its_outcome` che cade insieme a `each_policy_names_itself` |
| **11a** | l'`enqueue` estratto smette di far avanzare il contatore dei biglietti | — | ✅ **`arbiter_admission` 18/2**, banco 7/0. Il corpo estratto è **vivo** ed è tenuto dalle sonde del Task 6 |
| **11b** | l'`enqueue` estratto mette **ogni** richiesta nella corsia `Batch` | — | ✅ **`arbiter_admission` 19/1**, banco 7/0. Idem, sull'altro campo |
| **12** | la spazzata riscuote una concessione `Running` (`_ => true` diventa `_ => false`) | — | ⚠️ **Riga di ISOLAMENTO, non di uccisione.** Banco **1/6**: sopravvive la sola `each_policy_names_itself`, che non ammette niente. ⛔ **RICHIAMO DEL 2026-08-20 — QUESTA CELLA DICEVA CHE IL «PRIMA» ERA *«banco 3/2»*, ED ERA FALSO**, e la cifra nuova che l'ha sostituito era invece giusta: da tre passate non si scende a una aggiungendo sonde, e l'ondata non tocca **nessuna** riga eseguibile (`git diff 4b89fea..ea0cc09` su `crates/kernel/src/arbiter/mod.rs`: **zero** righe non di commento). ✅ **Rimisurato sul banco di cinque di `4b89fea` e non dedotto: 1/4** — muoiono tutte e quattro le sonde a scenario di allora e resta in piedi la sola `each_policy_names_itself`. ⛔ **E il corollario era falso a sua volta:** diceva che le morte muoiono sul `panic!` o sull'asserzione precedente *«e non su quella di `allocated()`»*, mentre `a_partly_full_machine_…` muore **esattamente lì** — `assert_eq!(allocated(), Mib::new(3_072), "PARTLY full")`, `left: Mib(1024)` — perché la spazzata svuota i libri già al secondo `admit`. Le altre cinque: `matches!(outcome, Queued(_))` per `the_remote_policy_…`, `the_local_policy_…` e `the_admission_asks_back_below_…`, il `panic!("queued")` del `let … else` per le due `under_the_…`. Voce `E106`. Fuori dal banco: `arbiter_admission` 7/13, `lib` 4/9 |
| **13** | `admit` smette di guardare il tetto (sovra-ammissione) | — | ⚠️ **Riga di ISOLAMENTO.** Banco **1/6**, e **a piena forza nessuna muore sull'asserzione di `allocated()`** — ✅ **rimisurato il 2026-08-20 sul banco di sette invece che riportato:** `a_partly_full_machine_…` muore sul `matches!(outcome, Queued(_))`, e la sua asserzione di `allocated()`, che sta **prima**, passa. ✅ **E il «prima» di QUESTA riga regge — 1/4 sul banco di cinque, con le stesse quattro morte della 12**: è il controllo che isola l'errore della riga sopra, perché due righe con lo stesso insieme di morte non possono avere due «prima» diversi (`E106`). Fuori dal banco: `arbiter_admission` 8/12, `lib` 12/1 |
| **14** | ⛔ **NUOVA dell'ondata di correzioni del 2026-08-20:** `admit` passa ad `ask_back` la richiesta INTERA invece dello scarto (`let needed = asked;`) | `a_partly_full_machine_asks_back_the_need_and_not_the_whole_request` | ✅ **rossa, e SOLA NELL'INTERO WORKSPACE — banco 6/1, workspace 242 passate 1 fallita**, su `revoking()` `left: 0, right: 1`. ⛔ **Era un MUTANTE VIVO**: prima della sonda nuova il workspace restava a **241 passate, 0 fallite**, perché nelle cinque sonde di partenza `ceiling`, `allocated()`, `asked` e `needed` valevano **tutti 4_096**. Voce `E97` |
| **15** | ⛔ **NUOVA dello stesso giorno:** `admit` passa una corsia FISSA invece della propria (`ask_back(needed, ComputeClass::Realtime, now)`), che **allarga** le vittime perché `askable` scarta con `held.lane <= below` | `the_admission_asks_back_below_its_own_lane_and_spares_a_peer` | ✅ **rossa, e SOLA NELL'INTERO WORKSPACE — banco 6/1, workspace 242 passate 1 fallita**, su `revoking()` `left: 1, right: 0`: l'arbitro sfratta un **pari** `Interactive` per un `Interactive`. ⛔ **Era un MUTANTE VIVO** a 241/0, e `a_grant_in_the_asking_lane_itself_is_not_asked_back` **non lo vede**: chiama `ask_back` direttamente con corsie esplicite, quindi tiene il confine e non il **cablaggio**. Voce `E97` |

⛔ **QUALI SONDE HANNO UN UCCISORE SOLO, E CON QUALE PERIMETRO — RIMISURATO IL 2026-08-20 SUL
BANCO DI SETTE, non riportato dal conteggio precedente.** **Cinque su sette** ce l'hanno:
`under_the_remote_policy_…` sotto la **5**, `under_the_local_policy_…` sotto la **6**,
`each_policy_names_itself` sotto la **9** e la **10**, `a_partly_full_machine_…` sotto la **14** e
`the_admission_asks_back_below_its_own_lane_…` sotto la **15**.
⚠️ **E DUE LO HANNO PERSO, che è il prezzo misurato delle due sonde nuove e non un difetto
scoperto dopo:** `the_remote_policy_…` aveva la **8** e `the_local_policy_…` aveva la **7**, e
quelle due mutazioni rompono `revoking()`, che le sonde nuove leggono anche loro — la **7** ora ne
uccide **due** e la **8** ne uccide **tre**. Nessuna delle due è diventata più debole: è la
CAMPAGNA a separare di meno. Voce `E104`.
⚠️ **E UNA SONDA È ORA DOMINATA DENTRO QUESTA CAMPAGNA, dichiarato invece che lasciato
scoprire** — la forma di `E37`, `E79` e `E93`: `the_local_policy_asks_the_lower_lanes_back` muore
sotto **2, 3a, 7, 12, 13**, e `a_partly_full_machine_…` muore sotto **quelle stesse più 8 e 14**,
quindi ogni mutante che uccide la prima uccide anche la seconda. ⚠️ **Richiamo del 2026-08-20:
diceva *«più 8, 13 e 14»*, e la 13 era già nell'insieme di partenza** — l'insieme più grande ha
**sette** righe e non otto (`E110`). ⛔ **Non si cancella:** la
campagna è un **campione**, non una dimostrazione, e le due sonde dicono cose diverse — quella
dominata è la coppia esatta di `the_remote_policy_…` sullo **stesso scenario**, che è la forma
con cui ADR-0006 chiede che le due policy si distinguano. Voce `E104`.
⚠️ **IL PERIMETRO NON È LO STESSO PER TUTTE, e va detto invece che lasciato intendere:** sono
sole nell'**intero workspace** solo la **9**, la **10**, la **14** e la **15**; la **5** e la
**6** sono sole **dentro `tests/arbiter_policy.rs`** e uccidono fra le sonde delle code del Task 6.

⚠️ **LE ASSERZIONI FINALI SU `allocated()` SONO DOMINATE DENTRO LA PROPRIA SONDA, E RESTANO.** Non
sono vacue — **isolate** (le asserzioni sopra tolte) scattano sotto la riga **13**,
`left: Mib(8192), right: Mib(4096)` — ma **a piena forza** un'asserzione **sopra** di esse scatta
sempre per prima. ⚠️ **Il numero delle righe della campagna è tolto da questa frase**
(2026-08-20): diceva *«tutte e tredici»*; la tabella è la misura
(`E102`).
⛔ **RICHIAMO DEL 2026-08-20 — QUESTA FRASE CONTAVA *«le tre»* PIÙ *«la quarta, quella della sonda
nuova»*, E LE ASSERZIONI SU `allocated()` NEL BANCO SONO SEI.** ✅ Ricontate col comando e non a
memoria — `grep -c "arbiter.allocated()" crates/kernel/tests/arbiter_policy.rs` → **6**, e **4** sul
banco di `4b89fea`: le originali erano già **quattro** e le nuove sono **due**, quindi la frase ne
lasciava fuori due. ✅ **E la distinzione che conta non è il numero ma QUALE, misurata riga per
riga della campagna:** le **cinque** asserzioni **finali** — quelle di
`the_remote_policy_…`, `the_local_policy_…`, `under_the_local_policy_…` (*«one grant, not two»*),
`under_the_remote_policy_…` e `the_admission_asks_back_below_…` (*«the books did not move»*) — sono
dominate: in **ogni** riga che uccide la loro sonda a scattare è un'asserzione precedente
(`revoking()`, `matches!(outcome, Queued(_))`, `promoted.len()` o il `panic!("queued")` del
`let … else`). ⛔ **La sesta NON è dominata, ed è quella della sonda nuova
`a_partly_full_machine_…`:** non è finale ma una **precondizione** —
`assert_eq!(allocated(), Mib::new(3_072), "PARTLY full")` — ed è la sola asserzione su
`allocated()` di tutto il banco che decide un esito, sotto la riga **12** (`left: Mib(1024)`).
Voce `E108`.
✅ **Misurato per isolamento il 2026-08-20 e non
ragionato**, su un campione che si nomina perché un'esclusività misurata su un campione parziale
si legge come una garanzia (`E82`): cinque mutazioni eseguite isolate — le due risposte delle
policy, la **12**, la **5** e la **13**. ⛔ **Il rimedio non è cancellarle**, che è la specie di
`E37` e `E79`: dichiarano l'intento che il nome della sonda porta — i libri non si sono mossi — e
la ragione è scritta **una volta sola**, accanto alla prima di esse, con le altre che ci
rimandano. Voce `E93`.

#### La transizione fra le due policy — Traguardo 5, Task 9, e `V3` si chiude

⛔ **INTENTO, POI L'EFFETTO, POI L'ESITO, E L'ORDINE È `V6` E NON PULIZIA.**
`Arbiter::set_policy` scrive l'**intento** nel giornale, **poi** scambia l'oggetto, **poi** scrive
l'**esito**. Cambiare policy ha effetti veri sul mondo — sfratti, ricariche — e niente si esegue
prima che l'intento sia **durevole**. Una transizione tagliata a metà lascia un passo **in
dubbio**, riconciliabile come ogni altro (§4.3): è la **proprietà DST numero 4**, e questo compito
è ciò che la rende scrivibile al Task 12.

⛔ **L'ASSERZIONE STA SULL'ARCHIVIO E NON SULLA POLICY, ed è la ragione per cui le sonde sono
cinque e non una.** *«Dopo la transizione la policy è l'altra»* è **verde con zero record
scritti**, e `V6` è esattamente l'affermazione che niente accade prima che l'intento sia durevole.
Le cinque: due leggono i record **decodificati** dall'archivio, una in ciascuna direzione; una
prova che un giornale che **rifiuta l'intento** lascia la policy dov'era; una taglia la
transizione **fra intento ed esito** con `CrashingJournal::falling_at(1)` e pretende un passo in
dubbio risolto `RunAgain`; l'ultima è la sua **contro-sonda** — senza schianto, **nessun** dubbio.

⛔ **`reason` PORTA IL NOME DELLA POLICY, ED È PER QUESTO CHE `MakeRoom::name` ESISTE.** Un record
che dicesse solo *«policy transition»* renderebbe le due direzioni **indistinguibili
nell'archivio**, e l'archivio è l'unica cosa che sopravvive. ⛔ **E una sola direzione non
basterebbe:** una costante `"local"` cablata sopravviverebbe a una sonda che asserisce `"local"`.
È il gotcha **#74** — la sonda deve nominare anche l'elemento per cui la regola esiste — e qui le
direzioni sono **due**, tenute dalle righe **6** e **7** della campagna, che sono uccisori
distinti.

⛔ **LA FIRMA NON PRENDE `now`, E DIVERGE DAL PIANO,** che dettava
`set_policy(&mut self, policy, step, journal, now)` con dentro un `let _ = now;`. `set_policy`
**non tocca i libri** — non legge né scrive `held` né `queues` — quindi non ha scadute da
riscuotere, e segue il precedente di `allocated()`, che dichiara di non riscuotere nulla. Un
parametro ignorato è la **superficie morta** che questa crate ha tolto a `Record::encode` e
rifiutato a `Ipc::accept`. ⚠️ **Il giorno in cui la transizione toccherà i libri l'argomento
torna, e torna come ERRORE DI COMPILAZIONE** a ogni sito di chiamata, non come regressione
silenziosa. Voce `E113`.

⛔ **IL DOC DI `Arbiter::policy()` PORTAVA UNA PREVISIONE, E QUESTO COMPITO L'HA MISURATA FALSA.**
Diceva *«Task 9's journalled transition is the production reader»* — scritta al Task 8 su codice
che non esisteva, gotcha **#57**. `set_policy` legge `self.policy`, il **campo**: dentro l'`impl`
passare dal proprio getter non compra niente e sarebbe codice peggiore scritto per far tornare una
frase. ⛔ **La frase è stata TOLTA e non affiancata da una smentita** (gotcha **#76**), con un
richiamo datato, sul precedente del finding `A-7` dell'audit — *l'argomento regge, l'evidenza no*.
Ciò che compra `policy()` restano i **banchi**: `each_policy_names_itself` e, dal Task 9,
`a_policy_transition_writes_its_intent_before_its_outcome` e
`a_refused_intent_leaves_the_policy_where_it_was`, che leggono `arbiter.policy().name()` **da
fuori la crate**. ⚠️ **Sono nominate una per una e non contate come insieme:** delle cinque sonde
della transizione, `policy()` lo leggono **queste due** — gotcha **#67**. Voce `E114`.

⛔ **LA CAMPAGNA DI MUTAZIONE — NOVE MUTAZIONI, NOVE UCCISE.** ⚠️ **Le righe 8 e 9 sono
dell'ondata di correzioni del 2026-08-20, e questa frase diceva anche *«nessun mutante vivo»*:**
la revisione ne ha misurati **due**, `trust` e `payload`, e adesso muoiono. Il quantificatore è
**tolto** e non riscritto (gotcha **#76**): una campagna misura le righe che ha, non lo spazio
delle mutazioni possibili. Ogni mutazione applicata **una alla volta**, compilata in un passo
**separato** da quello che esegue, e revocata **ripristinando da una copia byte-esatta** presa
prima — mai risostituendo all'indietro (gotcha **#48**) — con `cmp` identico dopo ogni
ripristino. Il perimetro è l'**intero workspace**, non il solo banco, e le colonne lo dicono.
Verde di riferimento: **35 bersagli, 248 passate, 0 fallite, 2 ignorate**; il banco
`arbiter_policy` **dodici**.

| # | Mutazione, sul codice di produzione | Sonda attesa morta | Misurato — **intero workspace** |
|---|---|---|---|
| **1** | scambiare l'ordine: prima `outcome`, poi `intent` | `a_policy_transition_writes_its_intent_before_its_outcome` | ✅ **244 passate, 4 fallite.** Muore quella attesa e con lei `a_transition_names_the_policy_it_moves_to`, `a_transition_cut_between_intent_and_outcome_…` e `without_a_crash_…`: `MemoryJournal::outcome` rifiuta un passo senza intento (`OutOfOrder`), quindi la transizione fallisce **sempre**. ⚠️ Sopravvive `a_refused_intent_leaves_the_policy_where_it_was`, e ha ragione: la prima scrittura fallisce comunque |
| **2** | assegnare `self.policy` **prima** di scrivere l'intento | `a_refused_intent_leaves_the_policy_where_it_was` | ✅ **SOLA NELL'INTERO WORKSPACE — 247 passate, 1 fallita.** ⛔ **La forma ingenua NON COMPILA:** `VramPolicy` non ha derive, quindi `self.policy = policy` **muove** il valore e il `policy.name()` che segue è `` error[E0382]: borrow of moved value: `policy` ``. La riga è stata rifatta nella forma che compila — l'assegnazione prima, e il record dell'intento costruito da `self.policy.name()` — perché una mutazione che non compila non misura niente. Voce `E115` |
| **3** | `EffectClass::Idempotent` → `Unrepeatable` | `a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt` | ✅ **SOLA NELL'INTERO WORKSPACE — 247 passate, 1 fallita**, sull'asserzione `Resolution::RunAgain`. È ciò che rende la classe dell'effetto **argomentata e tenuta**, e non scelta |
| **4** | `set_policy` non scrive **nulla**, assegna e basta | ⚠️ il piano diceva **tre** sonde | ⛔ **NE UCCIDE QUATTRO — 244 passate, 4 fallite:** le due dell'archivio, `a_refused_intent_…` (il `Result` è `Ok`, quindi `is_err()` cade) e `a_transition_cut_…` (stessa ragione). Sopravvive `without_a_crash_…`, e ha ragione: senza record non c'è nessun dubbio. **Divergenza dalla tabella dettata**, registrata e non appianata. Voce `E115` |
| **5** | scrivere l'intento e **non** l'esito | `without_a_crash_…` rossa, `a_refused_intent_…` **verde** | ✅ **L'AFFERMAZIONE DELLA RIGA REGGE: `without_a_crash_a_transition_leaves_no_step_in_doubt` è ROSSA e `a_refused_intent_leaves_the_policy_where_it_was` è VERDE** — le due sonde stanno su assi diversi. ⚠️ **244 passate, 4 fallite:** muoiono anche le due dell'archivio (`entries.len()`) e `a_transition_cut_…`, che con un solo record atteso non vede più l'errore. Il conteggio non era nella riga dettata, e si scrive invece di lasciarlo intendere |
| **6** | `transition_record` cabla `"local"` invece di `policy` | `a_transition_names_the_policy_it_moves_to` | ✅ **SOLA NELL'INTERO WORKSPACE — 247 passate, 1 fallita** |
| **7** | `transition_record` cabla `"remote"` invece di `policy` | `a_policy_transition_writes_its_intent_before_its_outcome` | ✅ **SOLA NELL'INTERO WORKSPACE — 247 passate, 1 fallita.** ⛔ **Con la 6 è la coppia che chiude `reason`:** ogni costante cablata è rossa in almeno una delle due direzioni, e nessuna delle due sopravvive |
| **8** | `trust: Trust::Instruction` → `Trust::Untrusted` | `a_policy_transition_writes_its_intent_before_its_outcome` | ✅ **SOLA NELL'INTERO WORKSPACE — 247 passate, 1 fallita.** ⛔ **La riga esiste perché la revisione l'aveva misurata MUTANTE VIVO** (248 passate, 0 fallite): `trust` non era pinzato da nessuna sonda, e `Trust` ha **due** varianti sole, quindi il campo era libero per intero. Chiusa nell'**ondata di correzioni del 2026-08-20** con una riga dentro la sonda che i record li decodifica già |
| **9** | `payload: Vec::new()` → due byte non vuoti | `a_policy_transition_writes_its_intent_before_its_outcome` | ✅ **SOLA NELL'INTERO WORKSPACE — 247 passate, 1 fallita.** ⛔ **Anche questa era un MUTANTE VIVO misurato dalla revisione** (248 passate, 0 fallite). Le due asserzioni nuove pinzano un **fatto del contratto** — il doc di `set_policy` dice che nessun byte esterno raggiunge il record — e non una preferenza |

⚠️ **CIÒ CHE QUESTO COMPITO NON COPRE, dichiarato invece che taciuto.** ① Il passo in dubbio è
provato su **UNO stato costruito a mano** — `CrashingJournal::falling_at(1)` — e non su una
campagna di semi: quella è del **Task 12**, e questa sonda esiste perché quella campagna abbia una
**forma da cercare** invece di una speranza. ② `set_policy` **non ha nessun chiamante di
produzione**, e ⚠️ **RICHIAMO DEL 2026-08-21 — QUESTA RIGA DICEVA *«nasce al Task 10 col
`daemon`»*, E IL TASK 10 L'HA MISURATA FALSA ESEGUENDOLA:** il grafo di produzione **costruisce**
l'arbitro con `VramPolicy::Remote(RemotePolicy)` — la scelta è consegnata al costruttore — e non
**transita** mai; una transizione all'avvio sarebbe uno scambio che nessuno chiede, quindi non è
stata inventata per far tornare questa frase. È il gotcha **#57**, una previsione scritta su codice
che non esisteva, e si **riscrive** invece di affiancarle una smentita (gotcha **#76**). Il
chiamante nasce col **primo ciclo di orchestrazione**, che in questo repository non esiste ancora.
③ La classe `EffectClass::Idempotent` è argomentata per ciò che il Traguardo 5 fa
davvero — **scambiare un oggetto**; quando arriverà il **contenuto** di uno sfratto (L2) va
riguardata, perché una ricarica non è gratis da ripetere, e il limite è scritto anche accanto al
codice.

#### Il grafo di produzione monta l'arbitro, il giornale e le due concessioni — Traguardo 5, Task 10, e `E41` si chiude

⛔ **LE DUE QUOTE DI ADR-0033 NON SONO SOTTRAZIONI, SONO DUE CONCESSIONI, E LA DIFFERENZA È `I2`.**
Una quota sottratta al budget **senza un titolare** lascia `I2` falsa per quel consumatore — *«la
sottrazione non è un'esenzione»*, ADR-0005 e gotcha **#4** — mentre una concessione un titolare ce
l'ha per costruzione. `crates/daemon/src/main.rs` le chiede tramite `Arbiter::admit` come
qualunque altra richiesta: ⛔ **l'arbitro non sa che si chiamano *audio* e *presentazione***, ed è
ADR-0001 — nessuna capacità ha accesso privilegiato.

⛔ **`E41` SI CHIUDE QUI, E NON CON UN'ASSERZIONE DENTRO UNA SONDA.** `E41` dice che una
configurazione impossibile ha **smesso di annunciarsi** il giorno in cui l'arbitro ha avuto le
code: la seconda quota permanente torna **`Queued`** invece di `Refused`, e nessuno la servirà mai,
perché rilasciare una concessione permanente è esattamente ciò che nessuno fa. L'arbitro non può
ripararlo — *«permanence is not a type, it is nobody calls release»*, quindi non sa distinguere un
biglietto che **sarà** servito da uno che non lo sarà mai. Il chiusore è la **radice di
composizione**: `reserve` traduce l'`Admission` in `Result<Grant, StartupError>`, e qualunque cosa
non sia `Granted` diventa `StartupError::ReservedQuota { name }`, che **nomina la quota** e
**ferma l'avvio**.

⛔ **DUE SONDE PERMANENTI E NON UNA MUTAZIONE, E LE VIE SONO DUE PERCHÉ FALLISCONO DIVERSAMENTE.**
Una direzione di prova tenuta da una mutazione è tenuta da **niente** — la mutazione si revoca e il
verbale resta a dire che la riga è chiusa, gotcha **#72**. E dentro `admit` le due strade sono
distinte (gotcha **#65**): *«più grande di ciò che è libero adesso»* è `Queued`, *«più grande della
macchina intera»* è `Refused`. Una sonda sola lascerebbe scoperta la strada che non prende.

| Sonda | `total_vram` | Cosa succede | Cosa asserisce |
|---|---|---|---|
| `a_permanent_quota_that_only_queues_stops_the_start_up` — è **esattamente** lo scenario di `E41` | `Mib::new(1_500)` | `audio-reserved` (1024) entra, `presentation-reserved` (768) non ci sta e viene **accodata** | `StartupError::ReservedQuota` con `name == "presentation-reserved"` |
| `a_permanent_quota_bigger_than_the_machine_stops_the_start_up` | `Mib::new(500)` | `audio-reserved` è più grande della macchina intera → **`Refused`** | `StartupError::ReservedQuota` con `name == "audio-reserved"` |

⚠️ **E LE DUE PASSANO DAL GRAFO INTERO, non da un arbitro ricostruito nella sonda.** È ciò che le
fa tenere il **cablaggio** e non solo `reserve`: le righe **1** e **10** della campagna qui sotto
lo misurano — tolte le due prenotazioni, o tolta la chiamata a `build_the_arbiter` dal grafo,
diventano rosse. ⛔ **Perché la firma può scegliere il totale:** `run_the_graph` prende i
`Parameters` come **argomento** invece di leggerli dalla costante `TOTAL_VRAM`, e senza quello le
due sonde non sarebbero scrivibili — il ramo d'errore che chiude `E41` non sarebbe raggiungibile da
nessun controllo.

⛔ **`assert_eq!` NON COMPILA SU QUESTI TIPI, E NON È UN DETTAGLIO DI STILE.**
`platform::journal::OpenError` deriva **il solo `Debug`**, quindi `StartupError` non può derivare
`PartialEq`; `Admission` non deriva né `Debug` né `PartialEq`, perché `Grant` non li ha e non deve
averli. Le sonde quindi **filtrano con `match`** e portano il `Debug` **dentro il messaggio**: un
`is_ok()` nudo non direbbe **quale** dei rami ha sparato. **Divergenza dal brief**, che dettava un
`assert_eq!(run_the_production_graph(&…), Ok(()))`.

#### Le sonde del `daemon`, per nome

| Sonda | Che cosa tiene |
|---|---|
| `the_production_graph_assembles_and_the_executor_runs_to_completion` | il **cablaggio**: `SequentialRng`, `SystemReactor`, `FileJournal`, l'arbitro con le due concessioni, i `Parameters` consegnati e la cella `Sleep` stanno insieme, e l'esecutore torna dicendo che il giro è finito |
| `the_production_graph_leaves_its_journal_on_the_disk` | che il giornale sia davvero **aperto**. Senza questa, un cablaggio che avesse tolto la riga `FileJournal::open` sarebbe restato verde: niente in questo binario **legge** il giornale |
| `a_journal_that_cannot_be_opened_stops_the_start_up` | la direzione opposta (§7.1.1 regola 3): una cartella che non c'è → `StartupError::Journal` |
| `the_two_reserved_quotas_are_held_by_grants_and_not_subtracted` | che le due quote siano **spese** — `allocated()` vale `1792` MiB — **e** che la policy montata sia `"remote"`, il default di ADR-0006 |
| `a_permanent_grant_survives_to_the_last_instant_of_the_axis_and_is_swept_at_it` | `FOR_EVER`, nelle **due direzioni**: a `u64::MAX - 1` le due concessioni ci sono ancora, a `u64::MAX` sono **riscosse** e `allocated()` torna `Mib(0)` |
| `the_two_reservations_declare_no_preemption_and_one_lane` | i **due campi** delle due `ResourceProfile`: `Preemption::Never`, che è la parola *«non-preemptible»* di ADR-0033, e `ComputeClass::Realtime`, che è la **premessa** della frase accanto a `build_the_arbiter` — una corsia sola, quindi dentro l'arbitro l'unico spareggio è l'**arrivo** |
| `a_permanent_quota_that_only_queues_stops_the_start_up` | `E41`, via **`Queued`** |
| `a_permanent_quota_bigger_than_the_machine_stops_the_start_up` | `E41`, via **`Refused`** |

⛔ **NESSUNA RIGA DI CATALOGO NUOVA NELLA §7.4, E LA COSA SI REGISTRA INVECE DI DECIDERLA**
(vincolo globale 7): queste sonde **non hanno riga di catalogo propria**. Vivono sotto
`I2 · §5.3` e sotto la riga blocco C `V29 · §2.8 · ADR-0034` per il verso della consegna dei
parametri, ma *«la radice di composizione ferma l'avvio quando una quota permanente non entra»* non
è una riga che esista: aggiungerla è una decisione del **proprietario**.

⛔ **LA CAMPAGNA DI MUTAZIONE — QUINDICI MUTAZIONI, TREDICI UCCISE E DUE VIVE PER SCELTA.**
⚠️ **Le righe 11–15 sono dell'ondata di correzioni del 2026-08-21, e le righe 1–10 sono state
RIMISURATE nella stessa passata** invece di essere ricopiate: una sonda nuova sposta il verde di
riferimento, e una cifra ferma al giorno prima sarebbe il gotcha **#31** dentro la tabella che lo
misura. Ogni mutazione applicata **una alla volta**, compilata ed eseguita a sé, e revocata
**ripristinando da una copia byte-esatta** presa prima — mai risostituendo all'indietro (gotcha
**#48**) — con `cmp` identico dopo ogni ripristino. Il perimetro è l'**intero workspace**. Verde di
riferimento: **35 bersagli, 255 passate, 0 fallite, 2 ignorate**, zero avvisi; il bersaglio
`daemon` **otto**.

| # | Mutazione, sul codice di produzione | Sonda attesa morta | Misurato — **intero workspace** |
|---|---|---|---|
| **1** | togliere le due `reserve` da `build_the_arbiter` | il piano ne diceva **una** | ⛔ **NE UCCIDE QUATTRO — 251 passate, 4 fallite:** le due di `E41`, `the_two_reserved_quotas_…` e `a_permanent_grant_survives_…`. **Divergenza dalla tabella dettata**, scritta invece che appianata |
| **2** | `TOTAL_VRAM` → `Mib::new(1_000)` | le due concessioni non entrano entrambe | ✅ **251 passate, 4 fallite.** `audio-reserved` (1024) è più grande della macchina, quindi muoiono anche le due sonde d'assemblaggio: l'avvio **si ferma**, che è precisamente ciò che `E41` chiedeva |
| **3** | `let _journal = journal_path;` — il giornale non si apre più | `the_production_graph_leaves_its_journal_on_the_disk` | ✅ **253 passate, 2 fallite.** Muore anche `a_journal_that_cannot_be_opened_…`, e ha ragione: senza `open` non c'è nessun `OpenError` da propagare |
| **4** | `reserve` cabla `name: "audio-reserved"` | `a_permanent_quota_that_only_queues_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita** |
| **5** | `reserve` cabla `name: "presentation-reserved"` | `a_permanent_quota_bigger_than_the_machine_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita.** ⛔ **Con la 4 è la coppia che chiude `name`:** ogni costante cablata è rossa in almeno una delle due direzioni — gotcha **#74**, la sonda deve nominare anche l'elemento per cui la regola esiste |
| **6** | scambiare l'ordine delle due prenotazioni | l'ordine non era dettato da nessuna riga | ✅ **253 passate, 2 fallite.** Entrambe le sonde di `E41` cambiano nome atteso: l'**ordine di arrivo** è la sola cosa che rompe la parità dentro `ComputeClass::Realtime`, ed è pinzato |
| **7** | `FOR_EVER` → `Millis::new(1)` | `a_permanent_grant_survives_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita.** ⛔ **La sonda esiste perché questa mutazione era un MUTANTE VIVO** — misurata prima: **253 passate, 0 fallite, a due sonde assenti**, niente in questo binario fa avanzare l'orologio. Un'affermazione senza guardia è il gotcha **#14** |
| **8** | `VramPolicy::Remote(RemotePolicy)` → `Local(LocalPolicy)` | `the_two_reserved_quotas_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita.** ⛔ **Anche questa era un MUTANTE VIVO** (253 passate, 0 fallite, **a due sonde assenti**): il default di ADR-0006 era **affermato in un commento e tenuto da niente**. Chiusa con una riga dentro la sonda che l'arbitro ce l'ha già in mano |
| **9** | `EXECUTOR_TURN_LIMIT` → `0` | ⚠️ **nessuna, ed è la risposta voluta** | ⛔ **MUTANTE VIVO PER SCELTA — 255 passate, 0 fallite.** È il **residuo dichiarato** accanto alla prima sonda, rimisurato invece che ricopiato: `Executor::run` è `while !self.tasks.is_empty()`, e senza attività il corpo non gira mai, quindi qualunque valore passa. Il numero avrà la sua sonda quando ci sarà qualcosa da lanciare |
| **10** | `run_the_graph` non chiama `build_the_arbiter` | l'arbitro non è più montato | ✅ **253 passate, 2 fallite** — le due di `E41`. È la riga che dice che le due sonde tengono il **cablaggio** e non solo `reserve` |
| **11** | `AUDIO_RESERVATION.preemption` → `Preemption::After(Millis::new(500))` | `the_two_reservations_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita.** ⛔ **Era un MUTANTE VIVO** (254 passate, 0 fallite, **a sonda assente**): queste due costanti sono il solo sito di produzione che **sceglie** `Preemption::Never`, e non lo teneva niente. ⚠️ **RICHIAMO DEL 2026-08-21:** questa cifra e le tre sotto dicevano *«255»*, che è il verde **con** la sonda — cioè uno stato in cui la mutazione non può essere viva. La misura vera è a `9c91e18`, dove `daemon` aveva **sette** sonde |
| **12** | `PRESENTATION_RESERVATION.preemption` → `Preemption::After(Millis::new(500))` | `the_two_reservations_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita.** ⛔ **Anche questa era un MUTANTE VIVO** (254 passate, 0 fallite, a sonda assente). Con la **11** è la coppia che chiude il campo su **entrambi** i profili e non su quello che capita per primo — gotcha **#74** |
| **13** | `AUDIO_RESERVATION.compute_class` → `ComputeClass::Batch` | `the_two_reservations_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita.** ⛔ **Era un MUTANTE VIVO** (254 passate, 0 fallite, a sonda assente): la corsia unica è la **premessa** della frase accanto a `build_the_arbiter` — *«nothing inside the arbiter breaks the tie … except arrival»* — e `admit` è sequenziale, quindi cambiarla lasciava verdi anche le due sonde di `E41` |
| **14** | `PRESENTATION_RESERVATION.compute_class` → `ComputeClass::Batch` | `the_two_reservations_…` | ✅ **SOLA NELL'INTERO WORKSPACE — 254 passate, 1 fallita.** ⛔ **Anche questa era un MUTANTE VIVO** (254 passate, 0 fallite, a sonda assente). Con la **13** è la coppia del campo `compute_class` |
| **15** | `Monotonic::ORIGIN` → `Monotonic::from_millis(1)` dentro `reserve` | ⚠️ **nessuna, ed è la risposta voluta** | ⛔ **MUTANTE VIVO PER SCELTA — 255 passate, 0 fallite.** `FOR_EVER` satura, quindi qualunque istante di partenza dà la stessa `expires_at`: è una conseguenza aritmetica e non una decisione da difendere. Residuo **⑤**, dichiarato accanto al codice |

⚠️ **CIÒ CHE QUESTO COMPITO NON COPRE, dichiarato invece che taciuto.**

① **I tre rami d'errore di `main` non li osserva nessun controllo.** Il residuo era già dichiarato
prima di questo compito ed è **peggiorato di un ramo**: adesso sono tre stampe diverse invece di
una. Il prezzo per coprirli è ancora lanciare il binario come **processo figlio** e rileggerne i due
flussi e il codice d'uscita, per tenere righe che non prendono nessuna decisione. ⛔ **E i tre rami
esistono per una ragione misurata, non per gusto:** `#[derive(Debug)]` **non conta come lettura**
per l'analisi del codice morto, quindi un ramo unico con `{error:?}` lasciava due avvisi
`field 0 is never read` su `Journal` e `Run` — e un `#[allow]` è un divieto spento (gotcha **#13**).
⚠️ **QUALI RAMI SIANO STATI PERCORSI A MANO È NOMINATO, invece che lasciato a un *«verificato a
mano»* che parlerebbe di quattro archi avendone visti due.** Percorsi il 2026-08-21, in una cartella
fuori dal repository: il ramo **`Ok`** — uscita 0, la frase su stdout, stderr **vuoto**, e un
`journal.redb` da `1 056 768` byte lasciato lì — e il ramo **`Journal`**, provocato mettendo una
**cartella** dove va il file: uscita 1, stdout **vuoto**, e
`File(Os { code: 5, kind: PermissionDenied, … })` su stderr. ⛔ **`ReservedQuota` e `Run` NON sono
stati percorsi:** nessuno dei due si provoca da fuori senza modificare il sorgente, che è ciò che
fa la campagna di mutazione e che una prova a mano non può fare.

② **`JOURNAL_PATH` non lo esercita nessuna sonda.** Ogni sonda passa il **proprio** percorso, in
una cartella privata per sito di chiamata ricavata da `line!()` — percorso fisso in cartella
condivisa è il gotcha **#52**, e su Windows la cancellazione a file aperto fallisce in silenzio,
quindi il rosso uscirebbe **su Linux**. Il valore di produzione è quindi un letterale che nessun
controllo tocca, e **dove** debba stare il file è una decisione che nessun ADR ha preso.

③ **`E50`, `E51` ed `E100` NON si chiudono qui**, per decisione del proprietario del 2026-08-21, e
la ragione è misurata leggendo il compito: il Task 10 non costruisce **nessun ciclo di
orchestrazione** — monta il grafo e lancia l'esecutore **una volta**, senza attività — quindi il
posto in cui si sceglie fra `admit` e `promote` **non esiste ancora**. I tre mutanti restano
**vivi e non pinzati**: pinzarli sarebbe un voto contro una decisione che il proprietario si tiene
(gotcha **#73**). ⚠️ **RICHIAMO DEL 2026-08-21 — QUESTO PARAGRAFO DICEVA *«quindi
`crates/kernel/src/arbiter/` non è stato toccato»*, *«i tre paragrafi restano esattamente
com'erano»* e *«la frase è lasciata in piedi»*, E IL SECONDO COMMIT DELLO STESSO DIFF LE HA
SMENTITE TUTTE E TRE:** il **codice** dell'arbitro non è cambiato — nessuna riga di prodotto,
nessuna sonda — ma quei commenti portavano un **puntatore a compito**, *«il chiusore è il Task
10»*, e `9c91e18` lo ha **riscritto** in quattro riquadri di `crates/kernel/src/arbiter/mod.rs`.
La **designazione del chiusore** non nomina più nessun compito — *«whoever builds the first
orchestration cycle»* — ed è quella a cui questo registro si allinea. ⛔ **È il finding `A-2` e
la radice `R1`:** una correzione attraversa il documento in cui nasce e non gli altri, e qui il
documento rimasto indietro era **questo**.

④ **`FOR_EVER` non è letteralmente «mai», e il commento dettato diceva di sì.** ✅ **Misurato:**
`Monotonic::ORIGIN.saturating_add(FOR_EVER)` satura **a** `u64::MAX`, e `collect_expired` confronta
`expires_at <= now`, quindi una spazzata all'**ultimo millisecondo rappresentabile** riscuote
entrambe le quote — `allocated()` torna `Mib(0)` invece di `Mib(1792)`. ✅ **E DAL 2026-08-21 LA
SONDA PERCORRE IL CONFINE NELLE DUE DIREZIONI** (vincolo 6): sta **un millisecondo dentro** la
finestra e poi spazza a `u64::MAX` e **attende `Mib(0)`**. Prima era pinzato il solo lato interno.

⑤ **`Monotonic::ORIGIN`, il terzo argomento di `admit` dentro `reserve`, non lo tiene niente.**
✅ **Misurato il 2026-08-21:** cambiato in `Monotonic::from_millis(1)` l'intero workspace resta
verde — **255 passate, 0 fallite**, la linea di base esatta. È la riga **15** della campagna e resta
**viva per scelta**: `FOR_EVER` satura, quindi `now.saturating_add(FOR_EVER)` vale `u64::MAX`
qualunque sia `now`, e l'indifferenza è una **conseguenza aritmetica** e non una decisione da
difendere. Dichiarata accanto al codice invece che scoperta.

⑥ **I due campi delle due `ResourceProfile` sono pinzati per VALORE e non per conseguenza, e il
prezzo si scrive qui.** La strada che attacca il **meccanismo** è stata **costruita e misurata**
prima di scegliere: un arbitro sotto `VramPolicy::Local` a cui si chiede più di quanto è libero, e
poi una spazzata oltre ogni grazia. ✅ **Misurato:** con quella sonda presente ogni mutazione di un
campo solo lasciava `daemon` a **8 passate, 0 fallite**, e solo mutando **entrambi** i campi dello
stesso profilo diventava rossa — **7 passate, 1 fallita**. Una sonda che nessuna mutazione singola
uccide è la sonda vacua della **prima domanda**, quindi non è stata tenuta. ⛔ **Perché non li
uccida — le guardie di `Arbiter::ask_back` — sta accanto alla sonda nel sorgente, in un posto
solo.** ⚖️ **Ciò che resta scoperto:** che uno dei due campi cambi qualcosa che l'arbitro **fa**.

#### Le contro-sonde delle righe nuove

Per file — la direzione che si dimentica (§7.1.1 regola 3):

| File | Righe che difende | |
|---|---|---|
| `crates/kernel/tests/time_types.rs` | **blocco C** · `V29 · §2.1`, entrambe | sette test |
| `crates/simulator/tests/seeded_rng.rs` | **blocco C** · `V29 · §2.2` | otto test |
| `crates/kernel/tests/boundary_promotion.rs` | **blocco C** · `Q9 · I6 · V20`, **entrambe** — la promozione dichiarata è la contro-sonda della regola A e della regola B — **e blocco B** · `V19` | ⚠️ **quindici** test — ricontati **sul binario** il 2026-08-10 chiudendo il traguardo: la cella diceva **otto**, ed era ferma a prima che il Task 7 vi portasse le sonde della nota, dell'accordo fra `kind` e operazione, e le due in cui la sonda dettata è stata **divisa**. Gotcha **#31** |
| `crates/kernel/tests/parameters_delivered.rs` | le **due** righe **blocco C** · `V29 · §2.8 · ADR-0034` | ⚠️ **Nessun numerale qui.** ⚠️ **RICHIAMO DEL 2026-09-01, ottavo giro: questa cella diceva *«cinque test»* al presente, e il binario ne dà **sei**.** Il Traguardo 5 Task 5 ne aggiunse uno — la metà di §2.8.4 per `total_vram` — e la cifra non fu mai rimossa, solo riallineata. **Tolta e non riallineata a sei:** si contano **sul binario**, `cargo test --locked -p kernel --test parameters_delivered` |
| `crates/kernel/tests/permission_triple.rs` | ⛔ **nessuna riga di catalogo** — i due casi `compile_fail` che accompagnano questo banco non ne hanno una, ed è una voce **registrata e non presa** (§7.4 è spec, vincolo globale 7). Ciò che tiene è `V21`, che resta **`⚠️ parziale`**. ⚠️ **RICHIAMO DEL 2026-09-01, secondo giro di revisione del compito 7: qui seguiva un'enumerazione** — *«le tre direzioni della tripla, il giornale vuoto e le quattro vie d'errore»* — **e la cella accanto vieta esattamente quello**, con il precedente di `gateway_decisor.rs` già scritto dentro di sé. ⚖️ **Contate SUL BINARIO**, come quella cella prescrive: `cargo test --locked -p kernel --test permission_triple` dà **undici** sonde, e l'enumerazione ne descriveva **otto**. ⛔ **Tolta e non riallineata** — precedenti **AUD-007** e **AUD-046**, *un elenco invecchia, una regola no*. ⚠️ **E il rapporto del revisore la prezzava in DIFETTO:** ne dava **una** omessa, quella che la revisione stessa aveva aggiunto, e sono **tre** — gotcha **#65** applicato a un rapporto, nella direzione che chiede meno del necessario | ⚠️ **Nessun numerale qui, e il precedente è di `gateway_decisor.rs` (voce `E97`):** una cella che conta i propri soggetti invecchia al primo giro di revisione. Si contano **sul binario** — `cargo test --locked -p kernel --test permission_triple`. Le mutazioni e il loro esito stanno nella sezione «Il compito 7 del Traguardo 6» |
| `crates/kernel/tests/arbiter_admission.rs` | **blocco C** · `V4` — la direzione *«distinguere le tre compila»* — `I2 · §5.3` — la direzione *«quello legale SI COSTRUISCE»* — **e dal Task 5** `V2` (*«con il profilo compila»*) e la riga del **blocco B** *«avviare un worker ← una concessione»* (*«con la concessione compila»*). Nessuna tenibile dal rispettivo caso negativo. ⚠️ **File nuovo del Traguardo 5 Task 4**, e cresce coi Task 6–7 | ⚠️ **venti** test — erano **due** alla nascita; le otto dell'ammissione sono del Task 5, l'undicesima — il **confine della scadenza** — è della revisione del 2026-08-19, le **otto** delle code sono del Task 6, e la ventesima è del Task 7. ⛔ **UNA SOLA dal Task 7, e TUTTE LE ALTRE vivono in `crates/kernel/src/arbiter/mod.rs`** — ⚠️ **quante, NON è scritto qui: il conteggio di quel modulo vive in un posto solo**, la sezione del Task 7 di questo file. Questa cella diceva **dodici**, ferma alla prima ondata del 2026-08-20 mentre la seconda ne portava un'altra: gotcha **#31**, e il rimedio è **toglierlo** e non riallinearlo (gotcha **#68**) **—** `ask_back` è `pub(crate)` e da qui non si vede — `` error[E0624] ``, misurato. Ricontati **sul binario** |
| `crates/kernel/tests/arbiter_policy.rs` | **blocco C** · `V3` — la direzione *«con UNA policy compila»*, e con **entrambe** le policy: il banco costruisce un arbitro per ciascuna, da fuori la crate. ⚠️ **File nuovo del Traguardo 5 Task 8.** ✅ **E dal Task 9 la seconda metà della contro-sonda di catalogo — *«e la transizione resta un passo giornalato (§5.4)»* — È QUI:** le cinque sonde di `Arbiter::set_policy`, che chiudono la riga `V3` (`E103`) | ⚠️ **dodici** test — erano cinque alla nascita del file; le due dell'ondata di correzioni del 2026-08-20 sono `a_partly_full_machine_asks_back_the_need_and_not_the_whole_request` e `the_admission_asks_back_below_its_own_lane_and_spares_a_peer` (`E97`), e le **cinque** della transizione sono del Task 9. Ricontati **sul binario** |
| `crates/kernel/tests/record_shape.rs` | **blocco C** · `Q14 · §4.9` **e** `Q9 · I6 · V20 · §4.9` — la contro-sonda dell'etichetta è `every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes`, che scrive **entrambi** i valori e ne confronta i byte | ⚠️ **dodici** test — ricontati **sul binario** il 2026-08-10 chiudendo il traguardo: la cella diceva **dieci**, e i due mancanti sono `the_reason_survives_the_round_trip_and_travels_beside_the_payload` e `an_empty_record_is_nine_bytes_and_the_inner_array_holds_five`, arrivati col **Task 7** — verificati col `diff` fra i due commit invece che dedotti. Gotcha **#31** |
| `crates/kernel/tests/worker_tokens.rs` | le **quattro** righe di §6.10.5 — **blocco B** · `I2` (*«col `Worker` → compila»*) e `I5 · Q4` (*«con la ricevuta → compila»*), **blocco C** · `I2 · §6.10` (*«istruirlo prima dell'uccisione compila»*) e `I5 · §6.10` (*«leggerne una compila»*). ⚠️ **File nuovo del Traguardo 5 Task 11**, e ottiene un `Grant` **vero** da `Arbiter::admit`, mai un costruttore di test. ⚠️ **Una sola funzione ne tiene due:** `reading_once_with_the_receipt_compiles` è la contro-sonda della riga del blocco B *e* di `I5 · §6.10` | ⚠️ **quattro** test — contati **sul binario** con `cargo test --locked -p kernel --test worker_tokens`, non per aritmetica. ⚖️ Il quarto, `one_grant_starts_one_worker`, **non è la contro-sonda di una riga nuova, e non tiene nessuna forma che `a_started_worker` non compili già** — stessa chiamata, stessa concessione usata una volta sola, e le altre tre lo invocano: nessuna mutazione può renderlo rosso da solo. Resta un luogo dichiarato dove appendere il ragionamento sul `Grant` consumato. ⛔ **Corretto il 2026-08-24: qui la forma *«una concessione avvia un worker solo»* gli era ATTRIBUITA**, e l'attribuzione è **tolta** perché era più generosa del vero. ✅ **RICONTATI IL 2026-08-27, chiudendo la radice R6: sono CINQUE**, e il quinto è nuovo. `a_spawn_that_does_not_happen_is_start_failed` è il **primo e unico produttore** di `ProcessError::StartFailed` nel workspace — finding **AUD-051**: la variante era tenuta in vita da una scadenza in prosa (*«milestone 5»*) scattata il 2026-08-21 **senza che niente diventasse rosso**, e la sonda sostituisce la data con qualcosa che deve continuare a compilare (gotcha **#77**). ✅ **Provata portante per mutazione**, revocata da una copia byte-esatta: col fake che risponde `Ok(FakeWorker)` invece di `Err(StartFailed)`, `assert_eq!` dà `left: None, right: Some(StartFailed)` e la sonda è **rossa da sola**. ⚠️ **La sua forza è di LIVELLO 1 ed è dichiarata nel sorgente:** una finta prova che la parola sia costruibile e che `start` riporti indietro un fallimento, **non** che un avvio vero fallisca — il produttore vero arriva col traguardo che implementa la porta. ⚠️ **E `assert_eq!` invece di `is_err()` non è stile:** la finta fallisce sempre, quindi `is_err()` resterebbe verde anche se rispondesse `Died` |
| `crates/kernel/tests/gateway_decisor.rs` | **blocco B** · `Q13` — la direzione *«filtrato → compila»*, che è `a_conforming_candidate_is_chosen_and_nothing_is_degraded`. ⚠️ **File nuovo del Traguardo 6 Compito 6**, e ottiene il gettone **solo** da `resolve`, mai da un costruttore di banco — che non esiste, ed è il caso `conforming_has_no_constructor.rs` a dirlo. ⛔ **Le altre sonde non difendono nessuna riga di catalogo, e quante siano lo dice il comando nella cella accanto**; a dire quali mutazioni le rendono portanti è la sezione «Il gettone di conformità del gateway». ⚠️ **RICHIAMO DEL 2026-09-01, ottavo giro: qui seguiva un'ENUMERAZIONE** — *«le tre uscite del filtro, le due direzioni di `NoRetention` e il record risolto»* — **che descriveva sette sonde mentre il binario ne dà nove**, e stava **nella stessa riga** della cella che vieta i numerali. Il settimo giro tolse il *«sette»* dalla cella accanto e lasciò in piedi l'elenco che lo ripeteva a parole: ⛔ **un elenco È un numerale**, ed è **tolto e non riallineato** (precedenti **AUD-007**, **AUD-046**). ⚠️ E il puntatore diceva *«in fondo a questa cella»* mentre il comando vive nella cella **accanto**: corretto | ⚠️ **Nessun numerale qui, e il richiamo del 2026-09-01 dice perché:** questa cella diceva *«sette test»* e il settimo giro di revisione ne ha aggiunte **due**, falsificandola nello stesso giorno in cui era stata scritta. Si contano **sul binario** — `cargo test --locked -p kernel --test gateway_decisor` — mai per aritmetica. Voce `E97` |

⛔ **`boundary_promotion.rs` è la contro-sonda di due blocchi insieme**, e ciascuno dei suoi
test dice una cosa sola: la promozione dichiarata **compila ed è registrata**, col proprio
passo e la propria ragione — contare i record non basta, un `promote` che scrivesse il passo
sbagliato o una ragione vuota lascerebbe il conteggio a uno (gotcha #30); un giornale che
**rifiuta rifiuta anche la promozione**, altrimenti V19 poggerebbe sulla diligenza del
chiamante; `summarize` restituisce `Untrusted`, e a provarlo è **l'annotazione**, cioè il
compilatore, non un'asserzione; `summarize` conta **caratteri e non byte** — misurato: con
fixture di solo ASCII gli altri test restavano verdi mentre una fetta di byte moriva sul primo
taglio dentro un carattere multi-byte; e il `Debug` di `Untrusted` **non stampa il contenuto**,
che chiudeva una via d'uscita dal confine — con `Debug` derivato,
`Instruction::new(format!("{:?}", untrusted))` portava il testo attraverso intatto.

⛔ **`parameters_delivered.rs` colma un buco del registro, non solo del codice.** Fino a oggi
qui era nominato il **solo** caso `compile_fail` di quelle due righe e **nessuna contro-sonda**,
mentre la regola 3 di §7.1.1 ne pretende due per voce. Il file pinza che il valore **porti** i
parametri risolti, che due valori diversi si distinguano **e** che due uguali non denuncino una
sostituzione mai avvenuta (gotcha #24), e che **nessun ripiego viva dentro il costruttore** —
la via d'ingresso più economica per un default, che il compilatore **non** può vietare e che
§2.8.4 dichiara come limite.

#### I casi di `compile_fail`, e cosa provano davvero

⛔ **I dieci casi nuovi nominano `kernel::` e non ridichiarano attributi propri**, a differenza
dei quattro del Traguardo 1: è il rimedio al gotcha **#39**, e significa che i loro oracoli
sono accoppiati alla **superficie pubblica del kernel**. Un cambio di firma li rende rossi, ed
è corretto che lo faccia. ⚠️ **Ricontati il 2026-08-09:** questa riga diceva *«cinque»*, e la
cartella `crates/kernel/tests/compile_fail/` ne conteneva allora **quattordici** in tutto —
**quattro** dal Traguardo 1 e **dieci** dal Traguardo 2. ⚠️ **Ricontati una seconda volta il
2026-08-10, chiudendo il traguardo, e col comando invece che a memoria** —
`ls crates/kernel/tests/compile_fail/*.rs | wc -l` → **diciassette**: i quattordici più **tre**
dal Traguardo 3, `record_without_version.rs` dal Task 1 e la coppia
`record_without_trust_label.rs` · `trust_has_no_default.rs` dal Task 2. ⚠️ **Ricontati una
TERZA volta il 2026-08-20, Traguardo 5 Task 8, con lo stesso comando: 29.** Erano **ventotto**
dal Task 5 (`E35`) e il Task 7 non ne aggiunse nessuno; il ventinovesimo è
`two_policies_at_once.rs`. ✅ **Ricontati una QUARTA volta il 2026-08-21, Traguardo 5 Task 11,
con lo stesso comando: 33**, e altrettanti `.stderr`. I quattro nuovi sono i casi di §6.10.5 —
`talking_without_the_handle.rs`, `instructing_after_the_kill.rs`, `reading_without_a_receipt.rs`
e `reading_twice_from_one_receipt.rs`. ⛔ **Nessuna aritmetica su questa riga: il comando è la misura.** ⛔ **Erano già tre al
commit precedente, e nessuno aveva toccato questa cifra:** è il gotcha **#31** sul numeratore di
un contenitore che cresce, e chi lo muove è chi scrive un caso — che questa riga non la apre
nemmeno. La frase *«i dieci casi nuovi»* qui sopra resta **vera del Traguardo 2**, e vale identica
per i tre del Traguardo 3, che nominano `kernel::` allo stesso modo.

⛔ **La colonna «Caso negativo» prova il meccanismo, non la dichiarazione — e il registro
non deve lasciar credere altro.** I **quattro casi del Traguardo 1** — `std_in_kernel.rs`,
`unsafe_in_kernel.rs`, `allow_overrides_forbid.rs` e `hashmap_in_kernel.rs` —
**ridichiarano ciascuno i propri attributi** e non nominano mai `kernel::`. Provano che
`#![no_std]` e `#![forbid(unsafe_code)]` **mordono dove sono dichiarati**; non provano che
siano dichiarati nel kernel. Tolto `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs`
e scritto un `unsafe` vero, quei casi restano **verdi**. ⚠️ **Il limite riguarda quei quattro e
non la cartella:** i dieci del Traguardo 2 fanno l'opposto, e attribuire anche a loro questa
debolezza direbbe che il gotcha #39 è ancora aperto.

A sorvegliare la **presenza** degli attributi è `scripts/gate-attributes.sh`, che è di
**livello 2**: un controllo esterno, quindi cancellabile. La riga di `forbid` è di ramo
**1b** — sostiene la validità dei blocchi A, B e C — e poggia quindi su un controllo più
debole di quello che difende. È dichiarato, non nascosto.

**Contro-sonde:** `crates/platform/tests/counter_probes.rs` — `platform` nomina `std` e usa
`unsafe`, e **compila**. Sono la direzione che si dimentica (§7.1.1 regola 3).

**Guardia di non-vacuità del banco:** `crates/kernel/tests/compile_fail.rs` conta i `.rs`
prima di chiamare `trybuild`. Senza, il banco **vuoto** usciva **verde**: un glob che non
pesca niente non è un errore per `trybuild`, che stampa un avviso e lascia i fallimenti a
zero. Misurato, non dedotto.

#### Una guardia salita al compilatore — 2026-08-21

⛔ **`crates/simulator/tests/dst_campaign.rs` teneva `WRITES_PER_RUN > 0` con un `assert!`
DI ESECUZIONE su un operando `const`**, cioè una guardia di livello 2 su un fatto che il
compilatore conosce. È ora un **blocco `const`**, che è la preferenza dichiarata della §7.1.2 —
*una regola che può salire al compilatore ci sale* — e qui è costata **una parola**.

✅ **Provata nelle DUE direzioni su una crate usa-e-getta cancellata subito dopo**, perché una
guardia che non si è vista scattare non è una guardia (gotcha **#14**), e una che scatta dove
non deve è peggio di una assente (**#24**):

| Direzione | Misura |
|---|---|
| **deve scattare** | con la costante a `0`, `cargo build` risponde `` error[E0080]: evaluation panicked: a scenario with no writes has nothing to fall at `` — **il messaggio della guardia, a tempo di compilazione** |
| **non deve scattare** | con la costante al valore vero, `24`, la crate **compila** |

⚠️ **A nominarla è stato `clippy::assertions_on_constants`, e non è una deroga alla §7.4.3.**
Clippy non ha voce nella porta e non ne ha avuta qui: non ha deciso **se** la regola valga —
quella c'era già — ha detto **a che livello** era tenuta. La decisione di salire è della
§7.1.2. ⛔ **E non è una riga di catalogo:** la §7.4 è **spec**, vincolo globale 7, quindi si
**registra e non si prende** — stesso trattamento di `PL-1`, di `K-1`/`B-1` e delle dieci
sonde dell'audit precedente.

#### Il gettone di conformità del gateway — Traguardo 6, Compito 6, e `Q13` si chiude

⛔ **La riga del blocco B che questo file attribuiva al «filtro dei vincoli (§6.3)» dal
2026-08-09 è l'ultima delle cinque, ed è chiusa.** `Conforming` vive in
`crates/kernel/src/gateway/mod.rs` con **tre** campi privati e **nessun** costruttore pubblico;
l'unico che ne conia uno è `resolve`, nello stesso modulo; `dispatch` lo prende **per valore**.
È la forma di `Arbiter::issue` per `Grant` (§5.6), e la cifra si riconta sulla cella del blocco
**B** in fondo a questo file, mai da questa frase.

⛔ **I casi sono DUE perché le metà del gettone sono due, e la riga di catalogo ne nomina una.**
Lo stesso trattamento che `grant_has_no_constructor.rs` riceve sotto la riga della concessione.

| Metà | Caso | Sigla | Come scatta |
|---|---|---|---|
| il candidato non filtrato **non è l'argomento** | `dispatching_an_unfiltered_candidate.rs` | `E0308` | ⚠️ **`mismatch`, la forma DEBOLE** (gotcha **#42**): riporta per l'oracolo, quindi dipende da un `.stderr` |
| il gettone **non si conia** | `conforming_has_no_constructor.rs` | ⛔ **senza sigla** | **`error`**: la forma forte, indipendente dall'oracolo |

⛔ **I due oracoli sono LETTI DALL'USCITA VERA e scritti a mano, mai copiati dal gemello**, e la
differenza si vede: `grant_has_no_constructor.stderr` dice *«private fields `id` and `issuer`»*,
questo dice *«private fields `model`, `evaluated` and `degraded`»* — **tre** campi contro due.
Copiare il gemello avrebbe prodotto un `mismatch` invece di un verde.

⚖️ **UNA VOCE REGISTRATA E NON PRESA, sul precedente esatto del `Grant`.** Il doc di `dispatch`
promette che *«una risoluzione dispaccia una volta sola»*, e il tipo lo regge davvero —
`Conforming` non deriva né `Copy` né `Clone`, quindi un secondo `dispatch` è `error[E0382]` — ma
**nessun caso lo dice**. La gemella per `Grant` (*«un secondo `start` con la stessa
concessione»*) è **misurata e non presa** dal Traguardo 5 per la ragione che decide anche
questa: una riga di catalogo nuova è §7.4, cioè **spec**, cioè del **proprietario** (vincolo
globale 7). ⛔ **Dichiarata accanto al codice** — sul doc di `dispatch` — perché è lì che un
lettore di quella promessa guarda. Voce `E62` dell'errata del piano.

##### Le tre uscite del filtro sono TRE sonde, e le mutazioni dicono quali righe contano

⛔ **Tre uscite, tre `#[test]` distinti e non uno con tre asserzioni** — conforme, fallimento
chiuso (classe `Data`, ADR-0012), degrado dichiarato (classe `Quality`). Una sonda sola si
ferma alla prima asserzione, e la seconda uscita non verrebbe mai esercitata (gotcha **#14**).

| # | Mutazione | Uccide | Misura |
|---|---|---|---|
| **M1** | `RecordKind::Routing => {}` → `enter(&mut open, step, resolution_of(body.effect()))` in `crates/kernel/src/reconcile.rs` | **due** — `a_routing_record_does_not_put_a_step_in_doubt` e `a_routing_record_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them` | **42 bersagli, 305 passate, 2 fallite, 2 ignorate**, e nient'altro nel workspace |
| **M2** | lo stesso arm → `leave(&mut open, step)` | **una sola** — `a_routing_record_leaves_the_doubt_…` | **306 passate, 1 fallita**. ⛔ **E la differenza fra M1 e M2 è la prova che la coppia non è un controllo scritto due volte:** la prima sonda non raggiunge `leave`, perché il record di routing dopo l'esito chiude un passo già chiuso ed è un'operazione nulla |
| **M3** | `Constraint::NoRetention => !candidate.retains` → `candidate.retains` in `crates/kernel/src/gateway/mod.rs` | **due** — `a_retaining_candidate_is_discarded_by_no_retention` e `a_candidate_that_keeps_nothing_satisfies_no_retention` | **305 passate, 2 fallite**. ⛔ **Senza queste due sonde l'arm era un MUTANTE VIVO su tutto il workspace** — nessuna sonda dettata nominava `NoRetention` — e la mutazione ammette esattamente i provider che **trattengono i dati**, cioè la classe che ADR-0012 fa fallire chiuso. Voce `E61` |
| **M4** | `let evaluated = chain.len() as u32` → il conteggio dei candidati **camminati** (`enumerate().find(..)`, posizione + 1) | **una** — `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it`, con `left: 1, right: 3` | **306 passate, 1 fallita**. ⛔ **È la ragione per cui la catena della sonda è lunga TRE e il vincitore è il PRIMO:** con una catena di uno le due letture valgono entrambe 1 e nessuna asserzione può separarle. Voce `E59` |
| **M5** | la quinta voce tolta dall'array a mano di `crates/kernel/tests/frozen_bytes.rs`, arità riportata a **4** | **due** in quel banco | `no frozen record carries Routing: its wire index is held by nothing`, e `the map describes 5 records and there are 4 frozen files` — l'`assert!` **vista scattare** |
| **M6** | `Trust::Instruction` → `Trust::Untrusted` nel record che `dispatch` scrive, in `crates/kernel/src/gateway/mod.rs` | ⛔ **NESSUNA, prima del 2026-09-01** — `42 bersagli, 307 passate, 0 fallite`, **identico alla baseline**. Dopo il rimedio: **una**, `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it` | ⛔ **È il rilievo più grave del settimo giro:** l'etichetta `I6`/ADR-0014 su un record **durevole** poteva essere scritta falsa senza un solo rosso. `dispatch` scrive **sei** campi e la sonda ne rileggeva **quattro**. Voce `E97` |
| **M7** | `EffectClass::Idempotent` → `Unrepeatable` nello stesso record | ⛔ **NESSUNA prima**, `307` identiche; **una** dopo | ⚠️ **Pinzato benché `reconcile` non lo legga**, e la ragione è scritta accanto all'asserzione: il doc accanto alla chiamata **afferma** che il valore è vero di quel record, e un'affermazione che nessuno tiene è ciò che gli altri quattro campi già erano. Precedente del **Task 10** del Traguardo 5 — *pinzati, non dichiarati, perché la decisione l'ha presa un ADR e il doc la afferma*. Voce `E97` |
| **M8** | il letterale di `reason` → `"MUTATED reason nobody pins"` nello stesso record | ⛔ **NESSUNA prima**, `307` identiche; **una** dopo | Voce `E97` |
| **M9** | `journal.note(step, &record)` → `let _ = journal.note(step, &record); Ok(())` — l'errore del giornale **inghiottito** | ⛔ **NESSUNA prima**, `307` identiche; **una** dopo, `dispatch_does_not_swallow_the_journal_saying_no` | ⛔ **La via d'errore di `dispatch` non era raggiunta da niente:** la funzione rende `Result<(), JournalError>` e nessuna sonda ne guardava mai l'`Err`. Un `dispatch` che dichiara successo mentre il record non ha raggiunto il giornale è la perdita silenziosa che ADR-0007 esiste per vietare. ⚠️ **La sonda NON apre il passo, di proposito** — è l'unica cosa che `open_the_step` esiste per evitare ovunque altro in quel banco. Voce `E98` |
| **M10** | `let evaluated = chain.len()` → il conteggio della **PRIMA** passata (`position(..) + 1`) | ⛔ **NESSUNA prima**, `307` identiche; **una** dopo, `evaluated_is_what_the_chain_OFFERED_and_not_what_the_first_pass_walked` | ⛔ **`M4` copre la SECONDA passata e non la prima**, e la differenza è il rilievo: nello scenario di `the_dispatch_journals_…` la prima passata non trova niente e quindi **esaurisce** la catena, quindi *«offerti»* e *«camminati dalla prima passata»* valgono entrambi **tre** e nessuna asserzione li separa. La sonda nuova ferma la prima passata all'**indice 0** — cammina uno, la catena ne offre tre. ⚠️ Il commento che prometteva *«due letture»* è **corretto**: sono **tre**. Voce `E100` |

⚠️ **E due guardie di livello 1 sono state viste scattare prima di essere estese**, che è la
metà che si dimentica: `RecordKind::Routing` ha reso `error[E0004]` la riconciliazione, il
`match kind` e il `match detail` di `frozen_bytes.rs`, **e** il `match` del costruttore di
`crates/kernel/tests/record_shape.rs` — quest'ultimo **non** nominato dal piano né dalla voce
`E60`, che ne censiva uno solo.

⛔ **E il record di routing NON invoca nessun modello, che è ADR-0020 in pratica.** Verificato
col comando e non dedotto — `grep -rn "provider\|adapter" crates/kernel/src/gateway/` rende
**cinque** righe, tutte di **commento** (due doc di campo e tre di prosa), e nessuna è una
chiamata. È la stessa forma della verifica su `\bGrant\b` in `crates/kernel/src/wire/`.

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
| i **test di contratto** — porta `journal`, **due implementazioni su due** | `crates/kernel/tests/journal_contract.rs`, incluso da `crates/platform/tests/journal_contract_real.rs`. ✅ **Ricontata il 2026-08-10 col Task 9:** questa riga diceva *«una implementazione su due»* ed era giusta, perché la suite girava contro la sola finta. Ora gira contro **entrambe a ogni commit** — `MemoryJournal` e `platform::journal::FileJournal` — **otto promesse su otto verdi** su ciascuna: *«misurato una volta con un file usa-e-getta»* è diventato *«tenuto»*, ed è l'unica differenza che questa riga esisteva per non lasciar sfumare. ⛔ **E la colonna «deve scattare» aveva una riga ASSENTE, trovata il 2026-08-10 partendo dall'elenco dei bugiardi invece che dalla colonna:** **J13** esisteva nella tabella delle sonde qui sotto dal Task 11 e **non era mai entrato qui**, quindi da questa riga il bugiardo della **7b** — l'unico che sbaglia dicendo **no** — risultava inesistente. È la specie che non si vede leggendo, perché ciò che manca non si legge; a trovarla è stato il movimento inverso, e i bugiardi sono **nove**. ⚠️ Le promesse verdi su ciascuna implementazione sono oggi **nove**, non otto: la cifra qui sopra è quella del Task 9 e resta perché descrive **quella** misura. ⛔ **E il 2026-08-17 l'esecuzione dell'audit ha trovato che TRE di quelle nove erano provate SOLO nello stato in cui ogni guardia plausibile passa** — finding **T-2** e **T-1**, gotcha **#63**: le promesse 1, 5 e 8 costruivano un archivio con **un passo solo** o **nessuno**, dove *«l'archivio è vuoto»* e *«questo passo non ha un intento»* sono la stessa frase, e *«il record di questo passo»* e *«il primo record che c'è»* lo stesso record. Chiuse **senza aggiungere nessuna promessa e senza toccare nessuna implementazione**: le nove promesse dicono già la cosa giusta e le due implementazioni la rispettano già — a mancare era lo **stato che distingue una sbagliata**. Bugiardi nuovi **J14**, **J15**, **J16**; sei mutazioni sulle implementazioni vere, sei rosse | **J2** · **J3** · **J4** · **J5** · **J6** · **J7** · **J8** · **J10** · **J13** · **J14** · **J15** · **J16** | J1 · **J9** · **J11** · **J12** |
| ⛔ i **byte congelati** del record durevole — §4.9.4, riga di catalogo `Q14 · §4.9` | `crates/kernel/tests/frozen_bytes.rs`, più `tests/frozen/` — **tre** `.cbor` e **una** mappa. ⛔ **Non si rigenerano, e non c'è nessun percorso per farlo:** niente flag, niente variabile d'ambiente, niente `--bless` — è così che `trybuild` si disarma (gotcha **#25**). I byte sono stati **scritti a mano** dall'uscita di una sonda usa-e-getta, cancellata nello stesso commit | **F1** · **F2** · **F3** | **F4** · **F5** · **F6** |
| ⛔ i **byte consumati** dalla decodifica pari alla **lunghezza dichiarata** dal frame — §6.10.4, riga di catalogo `Q4 · I5 · §6.10` | la busta in `crates/kernel/src/framing.rs`, lo schema in `crates/kernel/src/wire/worker.rs`; le sonde in `crates/kernel/tests/framing.rs` e `crates/kernel/tests/worker_wire.rs`, **entrambi fuori dalla crate** — se `framing::frame` e `FromWorker` non fossero abbastanza `pub`, questi banchi non compilerebbero. ⚠️ **Quante siano lo dice il comando e non questa cella**, che diceva *«cinque ciascuno»* — un numerale nudo, marcito il 2026-08-31 alla prima sonda aggiunta, gotcha **#31**: `grep -c '^#\[test\]' crates/kernel/tests/framing.rs crates/kernel/tests/worker_wire.rs`. ⛔ **E la via del `map_err` di `FromWorker::decode` è coperta solo dal 2026-08-31:** `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode` tengono i **due** ingressi che l'argomento di contenimento di `Record::encode` nomina — corpo **vuoto** e corpo **troncato** — e che `bytes_that_are_not_a_record_decode_to_malformed` tiene per `record.rs`. ⚠️ **Fino a esse questa cella affermava una direzione IN PIÙ di quella che le sonde tenevano**, e la prova è che la mutazione **W9** lasciava tutto verde | **W1** · **W2** · **W3** · **W4** · **W5** · **W6** · **W9** · **W10** | **W7** · **W8** |

#### ⛔ Il settimo passo del cancello, che non è un settimo controllo

**Aggiunto il 2026-08-11 col Task 9 del Traguardo 4**, e la distinzione è il motivo per cui questa
sottosezione esiste invece di una riga nella tabella qui sopra: ⛔ **`scripts/gate.sh` ha ora sette
passi e il catalogo resta a sei controlli.** Le asserzioni delle campagne DST girano **già**
dentro `cargo test --workspace`, che è il secondo controllo — quella è la cadenza che il vincolo 8
della §11 chiede — e questo passo **non può diventare rosso per una ragione che quel controllo non
abbia già colto**. ⚠️ **RICHIAMO DEL 2026-08-25, TRAGUARDO 5 TASK 12: qui e nel periodo seguente
c'era scritto «le DUE campagne DST», e da oggi sono TRE** — `crates/simulator/tests/dst_campaign.rs`,
`crates/simulator/tests/arbiter_campaign.rs` e
`crates/platform/tests/engine_crash_consistency.rs`. La cifra è **corretta e non affiancata da
una smentita** (gotcha **#76**).

Esiste per una cosa sola: il vincolo **7** vuole che il **tempo di parete si stampi a ogni corsa**,
*«così l'appesantimento diventa visibile prima di diventare una tentazione»*, e `cargo test`
**inghiotte l'uscita dei test che passano**. Le campagne vengono quindi rieseguite con
`--nocapture`.

⛔ **E IL PASSO NOMINA I PROPRI BERSAGLI UNO PER UNO, quindi una campagna che non è nell'elenco è
MUTA.** ✅ **Misurato il 2026-08-25 invece che dedotto**, con `arbiter_campaign.rs` già scritto e
la riga di `gate.sh` non ancora aggiunta: il cancello esce **`GATE GREEN`** e la sua uscita
contiene le quattro righe `DST arbiter` **zero volte**. È la ragione per cui il Task 12 tocca
`scripts/gate.sh`, che non stava nella sua intestazione `Files:` — voce `E150`.

| | |
|---|---|
| **deve restare verde, e le righe si devono VEDERE** | ⚠️ **Rimisurato il 2026-08-25: le righe sono SETTE, e questa cella ne nominava DUE** — lasciava fuori `DST L1 interleaving`, che stampa dal Traguardo 4, e le quattro dell'arbitro non esistevano. L'ultima corsa del cancello di quel giorno: `DST L1 interleaving: doubt set 2 reached after 2 of 2000 seeds` · `DST L1 campaign: 2000/2000 seeds crashed, largest doubt set 3, 109 distinct doubt sets, 122.9787ms` · `DST arbiter crashes: 2000/2000 seeds crashed, 999 with a step in doubt, 149.5997ms` · `DST arbiter ceiling: highest Mib(8192) of Mib(8192) over 2000 seeds, 154.1921ms` · `DST arbiter expiry: 2000 seeds with a late holder, 1835 where the sweep made room, 155.1779ms` · `DST arbiter worlds: 7 distinct outcomes over 2000 seeds, 156.329ms` · `DST L2 short: records=3 points=35 fired=35 truncated=22 partial=17 rungs=4/4 444.8221ms`. ⛔ **E i CONTEGGI sono stabili mentre i TEMPI non lo sono, il che è la cosa da portarsi via:** in una corsa precedente dello stesso cancello, lo stesso giorno e sugli stessi byte, le quattro righe dell'arbitro davano `334.8ms`, `336.7ms`, `339.0ms` e `346.0ms` — un fattore **2,2** su questa — mentre `2000/2000`, `999`, `1835`, `Mib(8192)` e `7` **non si sono mossi**. ⚠️ Le cifre di tempo del Traguardo 4 — `38.6ms` e `127.2ms` — **non** sono state riallineate: erano la misura di quel giorno e restano tali |
| ⛔ **deve scattare** | misurato al Traguardo 4 forzando un rosso in una campagna: **`GATE RED -- 2 checks failed`**, cioè il secondo controllo **e** questo passo. ✅ **Rimisurato il 2026-08-25 SUL BERSAGLIO NUOVO**, perché una direzione provata su un bersaglio non è provata sugli altri: con la mutazione `M3` della campagna dell'arbitro — `crash_point` che risponde sempre `0` — il cancello esce **`GATE RED -- 2 checks failed`** |
| **il costo, dichiarato** | ⚠️ **Rimisurato il 2026-08-25: le tre campagne brevi girano due volte, e la seconda passata costa 1,45 s di tempo di test** — `dst_campaign` 0,39 s, `arbiter_campaign` 0,36 s, `engine_crash_consistency` 0,70 s, presi **dentro il binario** e non dall'orologio attorno a `cargo`. La cella diceva **~0,2 s**, cifra presa al Traguardo 4 quando le campagne erano due: si **rimisura**, non si riporta. ⚠️ **E la cifra è un ordine di grandezza:** lo stesso binario dell'arbitro, con lo stesso comando, è stato misurato a **0,63 s** e a **1,53 s** dentro questa stessa sessione — vedi il riquadro della sezione qui sotto. E un rosso della campagna si conta **due volte** |

⛔ **La doppia rossa non è un difetto ma l'unica prova che il passo esegua ciò che dichiara:** un
passo di sola stampa che non potesse diventare rosso sarebbe **indistinguibile da uno che non
stampa niente**. È la regola 3 di §7.1.1 applicata a un passo che non è un controllo.

⚠️ **E i due prefissi sono stati allineati nello stesso passaggio** — il livello 2 stampava
`L2 short:` contro `DST L1 campaign:` — perché quelle righe esistono **per essere lette come
coppia**, e due grafie per una cosa sola costringono chi scorre l'uscita a conoscerle entrambe.
⚠️ **La campagna dell'arbitro entra nella stessa famiglia** col prefisso `DST arbiter `, per lo
stesso motivo.

#### La campagna DST dell'arbitro — Traguardo 5, Task 12

⛔ **CHE COSA COMPRA, IN UNA RIGA: l'arbitro gira DENTRO l'esecutore.** I suoi banchi lo
esercitano su **stati costruiti a mano**: `arbiter_admission.rs` con **38** `.admit(`,
`arbiter_policy.rs` con **14**, il modulo `#[cfg(test)]` di `src/arbiter/mod.rs` con **20**, più
`arbiter_resource.rs` — contati col `grep -c` il 2026-08-25. ✅ **E che l'interlacciamento fosse
il buco è misurato, non dedotto:** al 2026-08-25 i file che nominano insieme `Arbiter` ed
`Executor` sono `crates/daemon/src/main.rs` — la radice di composizione, che li costruisce
entrambi e chiama `admit` da `fn reserve`, fuori da ogni attività — e
`crates/kernel/src/arbiter/mod.rs`, che nomina `Executor` in **un** commento e in nient'altro.
`crates/simulator/tests/arbiter_campaign.rs` è il terzo, ed è il posto in cui l'arbitro gira
**dentro** l'esecutore: porta le proprietà DST **1**, **4** e **5** di §5.7.
⛔ **RICHIAMO DEL 2026-09-02, COMPITO 9 DEL TRAGUARDO 6: qui c'era *«Le altre due hanno bisogno
di `process` e `ipc`, Traguardo 6»*, e si RISCRIVE perché è il traguardo che l'ha detta a
chiuderla.** Le altre due esistono, e **non girano dentro l'esecutore** — quindi la frase di
questa cella, che parla dei chiamanti di `Executor`, resta vera com'è riscritta: la **2** vive in
`crates/simulator/tests/worker_kill_campaign.rs` e la **3** in
`crates/simulator/tests/gui_death_campaign.rs`, e nessuna delle due costruisce un `Executor`.

⛔ **LO SCENARIO DETTATO DAL PIANO ERA UNA CORSA RIPETUTA DUEMILA VOLTE, E LA CAMPAGNA LO HA
DETTO DA SÉ.** ✅ **Riprodotto il 2026-08-25 prima di cambiarlo** — corretti i due soli scarti di
firma, voci `E142` ed `E143`, e nient'altro: **1 esito distinto** su 200, 500, 1 000, 2 000 **e
20 000** semi, sempre lo stesso, `(granted 4, queued 8, refused 0, peak 6144)`. Cioè
`the_campaign_sweeps_more_than_one_world` — che il piano stesso detta — era **ROSSA sul codice di
oggi**, con la propria diagnosi. Le tre attività erano **identiche in tutto ciò che i libri
vedono**, e permutarle cambia **chi** viene servito, non **quanti**. Il verbale sta nella voce
`E144`, e la lezione è quella del gotcha **#14**: un rimedio che non si è visto partire da rosso
non è un rimedio.

**Che cosa è cambiato, e che cosa ciascuna cosa compra.**

| Nello scenario dettato | Nello scenario di oggi | Che cosa compra |
|---|---|---|
| `now` viene dall'**indice del ciclo** | `now` viene dall'**orologio virtuale**, letto attraverso `SharedClock` | il commento di modulo dichiarava l'iniezione dell'orologio e l'orologio **non veniva mai letto**: adesso è vero |
| **tre attività identiche** | **quattro parti diverse** in taglia, corsia, finestra di validità e ritmo | è la simmetria a rendere l'esito indipendente dal seme: rotta quella, l'ordine decide **chi entra** e quindi **quanto** sta nei libri |
| una **sola** pausa | pausa **diversa** dopo una concessione e dopo un accodamento | il calendario di una parte segue le **risposte** che ha ricevuto, quindi l'istante che passa ad `admit` è funzione del seme e non dell'iterazione |
| nessuno **rilascia** | ogni concessione torna, e chi rilascia **serve la coda** con `promote` | `release` e `promote` sono il **secondo** e il **terzo** posto in cui i libri si muovono, e `promote` emette concessioni con una guardia sul tetto **tutta sua**: senza, la proprietà 1 copriva il solo `admit` |

⛔ **E `SharedClock` NON È UN OROLOGIO SU MISURA:** ogni metodo inoltra al `VirtualReactor` che
sta dentro, quindi ciò che gira è la finta spedita. Esiste perché `Executor::new` prende il
reattore **per valore** e un'attività non ne tiene uno — lo dice `Sleep::until` — quindi senza un
riferimento condiviso l'unico istante che un'attività può nominare è uno che calcola da sé.

**La chiusura dello spazio degli esiti**, che è il criterio con cui `SHORT_CAMPAIGN_SEEDS` è
scelto — non «il più grande numero tondo sotto il tetto», che insegue una cifra che satura.
Misurato il 2026-08-25, in `debug`:

| semi | esiti distinti | ultimo nuovo al seme | tempo di parete |
|---|---|---|---|
| 200 | 7 | 38 | 36 ms |
| 500 | 7 | 38 | 88 ms |
| 1 000 | 7 | 38 | 189 ms |
| 2 000 | 7 | 38 | 369 ms |
| 20 000 | 7 | 38 | 3 690 ms |

⚠️ **LE DUE COLONNE NON VALGONO LO STESSO, e dirlo batte lasciare che le cifre sembrino
ugualmente solide.** Gli esiti distinti e il seme dell'ultimo nuovo sono usciti **identici** a
ogni ripetizione; il tempo di parete no. Lo **stesso binario**, con lo **stesso comando**
(`--test-threads=1`), è stato misurato a **0,63 s** all'inizio della sessione e a **1,53 s** più
tardi — un fattore **2,4** sulla stessa macchina, dentro la stessa sessione, sugli stessi byte.
Le cifre di tempo qui sono perciò un **ordine di grandezza** e non una costante, ed è anche il
motivo per cui **nessuna asserzione le tocca**: ciò che il cancello raccoglie è la riga stampata,
perché un lettore la confronti con la corsa precedente.

⛔ **E il conteggio è proprietà dello SCENARIO e non dei semi che lo campionano** — gotcha **#24**,
ed è ciò che rende `EXPECTED_OUTCOMES` un controllo invece di una scommessa. Misurato pescando i
2 000 semi in quattro modi diversi: `seed` stesso, e `seed * K >> 33` per
`K ∈ {0xBF58476D1CE4E5B9, 0x94D049BB133111EB, 0x2545F4914F6CDD1D}` — **7 in tutti e quattro**.

⚠️ **E la lunghezza dello scenario è stata misurata invece che scelta:** a `REQUESTS` **4** lo
spazio è di **7** esiti, a **6** di **8**, a **8** di **9**, mentre una passata da 2 000 semi passa
da **369 ms** a **578 ms** a **750 ms**. Lo spazio è limitato dalla **forma** dello scenario, non
dalla sua lunghezza.

⚠️ **`crash_point` NON entra in questa misura**, e il Passo 4 del compito lo chiedeva: le passate
che contano gli esiti girano con `CrashingJournal::without_crash()`, e il punto di caduta influenza
la sola `property_4`. Le tre costanti di mescolamento sono state usate **là**: `crashes` **2000/2000**
in tutte e tre, `doubted` **999**, **1003**, **1003**. Voce `E149`.

⛔ **LA CAMPAGNA DI MUTAZIONE — NOVE MUTAZIONI, OTTO UCCISE, UN MUTANTE VIVO.** Ogni mutazione
applicata **una alla volta**, compilata in un passo **separato** da quello che esegue, e revocata
**ripristinando da una copia byte-esatta** presa prima — mai risostituendo all'indietro (gotcha
**#48**) — con `cmp` identico dopo ogni ripristino, e `git status --porcelain` riletto. Il perimetro
è l'**intero workspace**. Verde di riferimento: **37 bersagli, 264 passate, 0 fallite, 2 ignorate**.

| # | Mutazione | Dove | Sonda attesa morta | Misurato — **intero workspace** |
|---|---|---|---|---|
| **M1** | l'ammissione smette di chiedersi se la richiesta entra in ciò che è libero | prodotto | `property_1_…` | ✅ **239 passate, 25 fallite.** ⛔ **E il messaggio è stato LETTO**, che è ciò che §5.7.1 pretende: `seed 0: allocated Mib(10240) exceeds the total Mib(8192)` — il **seme** e i **due valori** |
| **M2** | ogni parte chiede **1 MiB**: tutto entra sempre | banco | oracolo 1 | ✅ **260 passate, 4 fallite**, e le quattro diagnosi sono **diverse**: oracolo 1 *«everything fitted»*, la non-vacuità di `property_1` *«the books never came within Mib(1) of Mib(8192)»*, il secondo testimone di `property_5`, e l'oracolo 2 sulla riga `> 1` |
| **M3** | `crash_point` risponde sempre `0` | banco | secondo oracolo di `property_4` | ✅ **SOLA NELL'INTERO WORKSPACE — 263 passate, 1 fallita**, sull'asserzione `doubted > 0`: *«every crash fell before the intent: 2000 crashes and ZERO doubts»* |
| **M4** | `without_crash()` al posto del giornale cadente | banco | primo oracolo di `property_4` | ✅ **SOLA NELL'INTERO WORKSPACE — 263 passate, 1 fallita**, sull'uguaglianza `crashes == SHORT_CAMPAIGN_SEEDS`: *«a seed did not reach its crash point»* |
| **M5** | `collect_expired` non riscuote niente | prodotto | `property_5` | ✅ **252 passate, 12 fallite.** La campagna muore sul **primo** testimone, `already_collected`: *«on no seed did a holder outrun its own window»* |
| **M6** | l'**ammissione** non riscuote più; `release` e `promote` sì | prodotto | **solo** il secondo testimone di `property_5` | ✅ **257 passate, 7 fallite**, e la campagna muore sul **secondo** testimone, `room_from_expiry` — *«on no seed was a request seated on room the sweep had just freed»* — col **primo testimone verde**. ⛔ **È la mutazione che prova che i due testimoni sono DUE affermazioni** — gotcha **#55**, la regola nata al Task 3 del Traguardo 4 |
| **M7** | le quattro parti diventano **una parte quattro volte** | banco | oracolo 2 | ✅ **261 passate, 3 fallite**, e l'oracolo 2 muore sull'asserzione `distinct.len() > 1` — *«one run repeated 2000 times»*, cioè esattamente il difetto dello scenario dettato, riprodotto a comando |
| **M8** | `promote` non serve mai la coda | prodotto | oracolo 1, testimone `promoted` | ✅ **256 passate, 8 fallite**, e a morire è il testimone `promoted`: *«nothing was ever taken out of the queue: `promote` is not being exercised»* |
| **M9** | `release` non riscuote più prima di cercare la concessione | prodotto | — | ⛔ **MUTANTE VIVO — 37 bersagli, 264 passate, 0 fallite, 2 ignorate: l'intero workspace resta VERDE.** Vedi il riquadro qui sotto |

⛔ **RICHIAMO DEL 2026-08-25, PRIMA ONDATA DI CORREZIONI DEL TASK 12: I SEI NUMERI DI
RIGA DI QUESTA TABELLA SONO TOLTI E NON RIALLINEATI.** Erano **tutti e sei** sbagliati, e
mandare il lettore su un'asserzione **esistente e diversa** è peggio che mandarlo nel
vuoto: la citazione di `M8` faceva credere che la mutazione uccidesse *«every request got
exactly one answer»*, mentre uccide *«`promote` is not being exercised»*. Al loro posto c'è il
**nome** di ciò che muore, che è il rimedio già scelto da `E138` per la stessa specie: un nome
si rompe quando la cosa sparisce, un numero quando qualcuno scrive una riga più sopra.
✅ **Rifatte le sei mutazioni una alla volta il 2026-08-25** — `cargo test --locked -p simulator
--test arbiter_campaign -- --test-threads=1`, ripristino da **copia byte-esatta**, `cmp` e
`git status --porcelain` dopo ognuna — e ogni riga porta ora la frase di panico che ha davvero
stampato.

⛔ **IL MUTANTE VIVO, DICHIARATO E NON NASCOSTO.** `Arbiter::release` chiama `collect_expired(now)`
prima di cercare la concessione nei libri; togliendo quella chiamata **nulla in tutto il workspace
diventa rosso**. La ragione, letta nel codice: **ogni turno di ogni parte chiama `admit`**, che
riscuote per primo, quindi quando un ritardatario torna la spazzata l'ha già fatta qualcun altro —
il testimone `already_collected` resta perciò `> 0`.

⛔ **RICHIAMO DEL 2026-08-25, PRIMA ONDATA DI CORREZIONI — QUESTO RIQUADRO DICEVA *«il doc di
`release` non promette quella riscossione, quindi non c'è nemmeno una frase da rendere falsa»*, E
LA SECONDA METÀ È MISURATA FALSA.** Si **riscrive** col richiamo datato invece di accorciarla,
perché a essere sbagliato è il **fatto** e non una qualificazione intorno a esso. La premessa
stretta regge — il doc di `release` parla del solo consumo del `Grant` — ma la conclusione no:
✅ **misurato su una sonda usa-e-getta dello scratchpad, cancellata subito dopo**, una concessione
da `4_096` ammessa per `5_000 ms` e rilasciata a `5_001` risponde `Err(UnknownGrant)` a `HEAD` e
**`Ok(Mib(4096))`** sotto `M9`.

⛔ **E LE FRASI CHE QUEL SALTO RENDE FALSE SI CERCANO, invece di elencarle a memoria.** Il comando
è `grep -rnE "collects before it|COLLECTS THE EXPIRED" crates/ --include=*.rs`, e ogni riga che
restituisce si legge **intera**: quelle che parlano di `release` o di *«ogni operazione»*
diventano false sotto la mutazione, quelle che restano **vere** parlano di `admit`. ⚠️ **`a_grant_is_collected_at_the_instant_its_window_closes` è la ragione per cui si cerca invece di
elenco:** il suo doc afferma la cosa e il suo corpo asserisce **solo** su `admit`, quindi è una
frase che nessun rosso difende, e a leggerla a occhio non la si trova.

⚖️ **E LA RAGIONE PER CUI NON SI PINZA NON È IL PERIMETRO `Files:`, CHE ERA LA RAGIONE SCRITTA:
È `E30`, E STAVA GIÀ NEL SORGENTE.** Il blocco *«AND NO PROBE PINS THOSE THREE VALUES»* sul doc
di `ReleaseError` dichiara che una sonda che asserisse `Err` a `5_001` congelerebbe la decisione
che `E30` mette davanti al proprietario — *«a probe that must be deleted to take a decision is a
vote against taking it»* — e nomina **questo** mutante come costo dichiarato: *«moving
`collect_expired` after the lookup … turns nothing red, and this paragraph would become false in
silence»*. ⛔ Il mutante non è dunque una scoperta nuova ma il **costo che quel blocco aveva già
messo per iscritto**, e chi chiudesse la voce aggiungendo la sonda prenderebbe `E30` cancellando
un paragrafo invece di deciderlo. Voce `E151`, riscritta lo stesso giorno.

⚠️ **CIÒ CHE QUESTA CAMPAGNA NON COPRE, dichiarato invece che taciuto.** ① Che il rilascio renda
**esattamente** la riserva lo tiene `releasing_gives_back_exactly_the_reservation`: qui `release`
è esercitato sotto interlacciamento e ciò che si asserisce è il **tetto**, non l'importo. ② Il
**rifiuto** non è raggiunto: nessun profilo di `PARTIES` chiede più di `TOTAL`, e la frase non è
lasciata in piedi da sola — `the_scenario_really_makes_the_admission_decide` asserisce
`refused == 0`, così un profilo che crescesse o un tetto che calasse diventano **rossi** invece
che silenziosi. Il ramo lo tiene `a_request_larger_than_the_total_is_refused_and_not_queued`. Voce
`E145`. ③ La campagna gira sotto `VramPolicy::Remote`, che è il **default** di ADR-0006, quindi
`ask_back` non parte mai: la revoca ha le proprie sonde nel modulo `#[cfg(test)]` di
`src/arbiter/mod.rs`. ④ L'ordine dentro un turno — rilascia, promuovi, ammetti — è una **scelta di
questo banco** e non una decisione sul ciclo di orchestrazione: **niente qui asserisce su
quell'ordine**, quindi le voci aperte `E51` ed `E53` restano aperte.

#### Le sonde, per nome

| | |
|---|---|
| **N1** | lo stato pulito passa — è il verde di partenza, non una violazione colta |
| **N2** | una crate **spedita** fuori lista → `I3 violated`, e il rimedio è **TOGLIERE** |
| **N3** | una crate **di build** fuori lista → l'altro messaggio, e il rimedio è **AGGIUNGERLA**. Sono due grafi proprio perché i rimedi sono opposti |
| **N4** | `getrandom` in `platform`, dove ADR-0031 lo ammette: **resta verde**. È la sonda che di solito si dimentica |
| **N5** | un nome di crate con la **maiuscola**: prima usciva **verde**, ed era un falso negativo su I3. Corretto allargando la classe di caratteri del filtro, con la ragione scritta accanto alla classe |
| **N6** | ⛔ **un manifesto DERIVATO dal lockfile → `cargo tree --locked` fallisce, e lo script lo DICE.** È il finding **G-5** dell'audit, chiuso il 2026-08-17. ⚠️ **Riprodotto prima di correggere, e il rapporto lo prezzava come «una riga»:** tolta la riga di `minicbor` da `crates/kernel/Cargo.toml`, `gate-deps.sh` com'era rispondeva `OK -- the two graphs match the two lists`, **exit 0**, avendo riscritto in silenzio il `Cargo.lock` **tracciato** — **1 inserzione, 33 cancellazioni** — cioè misurava un grafo che **nessuno ha approvato** credendo di misurare quello della lista. La guardia di non-vacuità non lo coglieva: i due grafi erano non vuoti e diversi. Col rimedio, stesso guasto: **exit 1**, il messaggio nomina il lockfile stantio, e il lockfile **non si muove**. Il cancello intero: **`GATE RED -- 5 checks failed`**, con `Cargo.lock` intatto per tutta la corsa. ⛔ **I siti `cargo` non sono uno, e stanno su tutti e tre gli script** — perché i due script si lanciano anche **da soli**, e un controllo che vale solo passando dal cancello è più debole di uno che vale sempre. ⚠️ **RICHIAMO DEL 2026-08-27, finding AUD-024:** qui stava *«sono SEI e non uno — `gate.sh` ×4, `gate-no-os.sh` ×1, `gate-deps.sh` ×3»*, e la scomposizione sommava **otto**. **Tolta, non riallineata:** la cifra e il comando che la produce vivono in [`riferimenti.md`](riferimenti.md), e ciò che regge qui è la **relazione** — *ogni sito eseguibile passa `--locked`*, provata nelle due direzioni. ⚠️ **Due limiti dichiarati.** (1) Ciò che il ramo d'errore esplicito compra è la **diagnosi**, non il rosso: senza di esso i due grafi restano **vuoti**, coincidono, e la guardia di non-vacuità in fondo al file diventa rossa lo stesso — dicendo però *«la query era stretta»* dove la verità è *«il lockfile è stantio»*, che è il rosso illeggibile del gotcha **#24**. (2) Il messaggio di **coda** dello script resta generico — *«Read the REMEDY: it is NOT the same for the two graphs»* — mentre per questa classe di guasto il rimedio è **lo stesso** per i due grafi: lasciato invece che ramificato, perché il messaggio per-finding lo dice già giusto, e un ramo in più per una riga di prosa sarebbe sovra-ingegnerizzazione |
| **N7** | ⛔ **la seconda direzione di N6, e non è una formalità:** `--locked` poteva rendere **rosso uno stato corretto**, se il lockfile committato fosse stato fuori sincrono con i manifesti senza che nessuno se ne fosse accorto. Misurato, non supposto: stato pulito → `bash scripts/gate.sh` dà **`GATE GREEN`**, `cargo test --locked --workspace --no-fail-fast` dà **32 target, 177 passati, 0 falliti, 2 ignorati** — identico alla baseline — e `git status` è **vuoto dopo la corsa**, cioè il cancello non ha più il lockfile fra i propri effetti. ⚠️ Prima del rimedio quella stessa corsa lasciava anch'essa l'albero pulito: il verde di N7 **non** prova che il rimedio morda, lo prova N6. Le due sonde vanno lette in coppia |
| **B1** | `kernel` e `simulator` compilano per il bersaglio senza OS |
| **B2** | `getrandom` in `kernel` → `target is not supported` |
| **B3** | contro-sonda: con `--workspace` il cancello fallirebbe su `platform` con `can't find crate for std` — **motivo giusto, crate sbagliata**. Per questo il comando nomina `-p kernel -p simulator` |
| **B4** | il bersaglio non installato → uscita 1 e messaggio corretto. ⚠️ Vedi sotto: è la via *offline* |
| **S1…S7d · C0 · C5 · C6** | **diciotto su diciotto**, con ripristino byte-identico della spec: le tredici del 2026-08-07 (§8.6.3) più le quattro di `S7` e la contro-sonda `C6`, aggiunte chiudendo la §7.1.1 |
| **R1** | la **finta** onora il contratto — il verde di partenza dal lato `simulator` |
| **R2** | la **vera** onora il contratto: la stessa funzione, gli stessi assert, l'altra implementazione. È ciò che la suite esiste per comprare |
| **R3** | `NullAdvanceLiar`, che risponde `Some` a una scadenza **pari** all'istante corrente → la suite scatta. ⛔ E il test **legge il payload del panic**: verifica *quale* asserzione ha sparato, perché un `is_err()` nudo direbbe «ho colto il null advance» anche se a scattare fosse stata un'altra — una misura vera di un'altra cosa, gotcha #15 |
| **R4** | `PastDeadlineLiar`, che onora `deadline == now` e mente su `deadline < now` → scatta sul **solo** caso 2b. ⛔ È la sonda che prova che il caso 2b **non è vacuo**: prima che esistesse, cancellare l'intero blocco 2b lasciava la porta **verde**, e con essa la metà `<` del ramo priva di guardia. Gotcha **#45** |
| **R5** | la plausibilità di `wall_time()` sulla **vera** — `crates/platform/tests/wall_clock_plausibility.rs`: un istante posteriore a una data fissa del passato. Coglie l'orologio fermo a **zero o all'epoca**, che era la mutazione sopravvissuta alla prima stesura |
| **J1** | il **doppio in memoria** onora tutte e **nove** le promesse — il verde di partenza. ✅ **La promessa 7 non è più soddisfatta per la ragione sbagliata:** la voce aperta più sotto è **chiusa** dal Task 11, e la metà che discrimina è la **7b**. ⚠️ **Questa riga ha detto «tutte e sei», poi «tutte e otto»**, ed erano cifre stantie nella stessa riga: le promesse sono **nove in dieci blocchi** dal 2026-08-10 — 7b è la **seconda direzione** della 7, non una regola in più. Ricontate ogni volta sul sorgente, gotcha **#31** |
| **J2** | `SilentJournal`, che risponde `Ok(())` e non scrive → scatta la **promessa 1**. ⛔ È la via **A6** di `boundary.rs` resa eseguibile, ed è la ragione per cui questa suite esiste in questo traguardo |
| **J3** | `LastWriteWinsJournal`, che rilegge l'**ultimo** record del passo invece del primo → scatta la **promessa 2**. ⛔ È la forma che una tabella chiavata sul passo ha **per natura**, cioè quella che avrà `redb`: è la promessa che la seconda implementazione **non incontra da sola** |
| **J4** | `EmptyInsteadOfMissingJournal`, che riporta l'**assenza** come lettura riuscita di **niente** → scatta la **promessa 3** |
| **J5** | `ShuffledJournal`, che restituisce il giornale **rovesciato** → scatta la **promessa 4** |
| **J6** | `PermissiveJournal`, che accetta un esito **senza intento** → scatta la **promessa 5** |
| **J7** | `UnguardedIntentJournal`, che accetta un **secondo intento** sullo stesso passo → scatta la **promessa 6**. ⛔ **Rotto per assenza e non per menzogna:** ha il proprio archivio e la guardia su `outcome`, e gli manca solo quella su `intent` — è ciò che era `MemoryJournal` fino al 2026-08-10. Non lava un rifiuto altrui come fa `PermissiveJournal`, o sarebbe lo stesso difetto scritto due volte (gotcha #45) |
| **J8** | `EagerPruner`, che pota un passo **in dubbio** → scatta la **promessa 7**. ⚠️ Dal 2026-08-10 la promessa che uccide ha **due** asserzioni: l'intento nudo, e l'intento **con una nota** — una nota non è un esito, e senza il secondo caso una nota archiviata come esito rendeva potabile un passo in dubbio con l'intero workspace verde (misurato, mutazione `M12` del Task 11) |
| **J10** | `DiscardedNoteJournal`, che **controlla la nota e non ne conserva nulla** → scatta la **promessa 8**. ⛔ **Rotto in un ottavo modo** (gotcha #45): fa la verifica, risponde `Ok(())` e non scrive — *valida e poi butta*. `SilentJournal` è il vicino più prossimo e **non è lo stesso**: quello non verifica niente e muore sulla promessa 1, sei promesse prima. ⛔ **Ed è la forma che una implementazione vera prende davvero:** `note` è l'operazione più nuova della porta, quindi la più probabile da lasciare `Ok(())` mentre il resto è scritto bene — e ciò che smette di arrivare all'archivio è **il contenuto non fidato con l'etichetta che dice che lo era**, cioè la via **A6** puntata sulla via **A4**. ⚠️ **Sta qui e non fra J8 e J9** perché gli identificatori di questo registro **non si spostano**: J9 era già la contro-sonda quando questo bugiardo è nato |
| **J9** | ⛔ **la contro-sonda, ed è di due pezzi.** (a) La **durabilità attraverso la caduta del processo** sta **fuori** dalla suite — `the_memory_journal_does_not_survive_being_dropped` vive in `memory_journal.rs` — perché pretenderla in conformità renderebbe rossa la finta, che è **corretta** (gotcha **#44**). (b) La **mutazione di controllo**: cambiato un **solo commento**, tutti e **undici** i test restano verdi e `cargo test --workspace --no-fail-fast` dà **zero** rossi. Senza (b) la tabella qui sopra non prova niente — gotcha **#48**. ⚠️ **La cifra ha detto «otto» e poi «dieci»**: rimisurata il 2026-08-10 con la promessa 8, la sonda delle sottostringhe e il bugiardo della 7b |
| **J14** | ⛔ **`StepBlindJournal`, che rilegge IL PRIMO RECORD DELL'ARCHIVIO qualunque passo lo chieda → scatta la promessa 1.** È il finding **T-1** dell'audit, ed è **un predicato di larghezza**: `e.step == step` tolto alla finta, `stored == step.get()` tolto alla vera. ⛔ **Fino al 2026-08-17 PASSAVA LA SUITE INTERA** — misurato, non dedotto: rispondeva *«THE SUITE IS VACUOUS ON promise 1»*. La ragione è che **otto blocchi su dieci** tenevano un archivio con **un passo solo**, dove *«il record di questo passo»* e *«il primo record che c'è»* sono **lo stesso record**, quindi `read_back` non è mai stato costretto a **scegliere**. ⚠️ **Rotto in un decimo modo** (gotcha #45): non perde scritture, non mente sull'esito, non rovescia niente — **non guarda il nome**, che è ciò che il doc della porta chiede per primo (*«re-reads ONE step BY NAME»*) |
| **J15** | ⛔ **`BlindGuardJournal::on_outcome`, la cui guardia chiede «l'archivio è vuoto?» invece di «questo passo ha un intento?» → scatta la promessa 5.** È il finding **T-2**, il più grave dell'audit del 2026-08-11: con le guardie di `FileJournal` sostituite così, `cargo test --workspace --no-fail-fast` dava **32 target, 171 passati, ZERO falliti**. ⛔ **Non è `PermissiveJournal` con un altro nome:** quello non ha **nessuna** guardia e lava **ogni** rifiuto, questo ne ha una **sbagliata** e lava solo dove le due divergono — cioè su un archivio **non vuoto**. La differenza è esattamente ciò che le due direzioni della promessa 5 misurano, ed è l'undicesimo modo. ⛔ **Cosa costa se passa:** un esito accettato per un passo **mai aperto**, quindi la riconciliazione legge un `Outcome` di un passo che non è mai stato eseguito e lo **toglie da un dubbio che non c'era** — la classe di guasto per cui ADR-0007 esiste |
| **J16** | ⛔ **`BlindGuardJournal::on_note`, la stessa guardia cieca su `note` → scatta la promessa 8.** ⛔ **Ed è un bugiardo distinto per NECESSITÀ e non per simmetria:** `note` e `outcome` condividono `has_intent` in **entrambe** le implementazioni, quindi una guardia cieca le acceca insieme — ma la suite muore alla **prima** promessa rotta, quindi un bugiardo cieco su tutt'e due muore sulla 5 e il blocco della 8 resterebbe **non provato mentre un test afferma il contrario** (gotcha **#45**). Un tipo solo con **due istanze**: due tipi sarebbero **lo stesso difetto scritto due volte**. ⚠️ **Misurato prima di essere scritto:** contro la suite com'era, rispondeva *«THE SUITE IS VACUOUS ON promise 8»* — un giornale che tiene note su passi che nessuno ha aperto passava tutto. ⚠️ **L'audit non l'aveva nominato**: elencava le promesse 5 e 8a come lo stesso finding e non aveva provato che servissero **due** bugiardi |
| **J13** | ⛔ **`AlwaysInDoubtJournal`, che rifiuta OGNI potatura con la parola giusta → scatta la promessa 7b**, ed è ciò che rende non-vacua la **7**. ⛔ **Rotto in un nono modo** (gotcha #45): è **l'unico bugiardo del registro che sbaglia dicendo NO** — non distrugge niente e non mente, semplicemente non pone mai la domanda. `EagerPruner` è il suo **opposto** e non il suo gemello. ⛔ **Risponde `StepInDoubt` e non `Missing`, e la scelta è il controllo:** con `Missing` morirebbe sulla promessa **7**, sei righe prima, e la 7b resterebbe non provata mentre un test afferma il contrario. ⚠️ **Ed è ciò che erano ENTRAMBE le implementazioni fino al 2026-08-10**, a meno della parola |
| **J11** | ⛔ **il vincolo che rende sicuro `contains`, e fino al 2026-08-10 era solo dichiarato.** `no_promise_message_is_a_substring_of_another` — nessuno dei **nove** messaggi è sottostringa di un altro, e nessuno è vuoto. ⚠️ **Erano otto fino al Task 11**, e il conteggio delle coppie è stato **strumentato invece che calcolato**: **72 coppie ordinate**, erano 56. I due messaggi di `prune` sono la coppia più vicina che l'insieme abbia mai avuto — cominciano entrambi con *«a step »*. Senza, un bugiardo colto sulla promessa **sbagliata** soddisferebbe comunque il test che nomina quella giusta, e la suite direbbe `ok` indicando il posto sbagliato. ⚠️ **Misurato in due direzioni**, e la prima è quella che insegna: rendere un messaggio un semplice **prefisso** di un altro **non** lo fa scattare — giustamente, perché `contains` non è ingannato da un prefisso condiviso — mentre una **vera** inclusione (`MISSING_MESSAGE` = `"a `note` upon an open step must be"`) lo fa scattare **da solo**, e un messaggio **vuoto** anche |
| **R6** | ⛔ **la contro-sonda che conta, e la si sarebbe dimenticata:** rompendo l'avanzamento dell'orologio di parete del `VirtualReactor`, la conformità **resta verde** e scatta **solo** `crates/simulator/tests/virtual_clock.rs`. È la prova che la suite condivisa non impone alla vera un comportamento della finta — se lo facesse, renderebbe rossa un'implementazione **corretta**. Gotcha **#44** |
| **J12** | la **vera** onora il contratto — `the_real_journal_honours_the_contract` in `crates/platform/tests/journal_contract_real.rs`, la stessa funzione e gli stessi assert di **J1**, l'altra implementazione. È ciò che la suite esiste per comprare, ed è l'analogo di **R2**. ⚠️ **Dal Task 11 le promesse sono nove**, e `FileJournal` le onora tutte: `prune` è la prima operazione che gli è costata un **cambio di archivio** invece di una riga — vedi la voce del Task 11 più sotto. ⛔ **E la non-vacuità è misurata in TRE direzioni, non una**, perché una sola proverebbe **una promessa su otto**: rotta `FileJournal::read_back` perché risponda sempre `Ok(Vec::new())` muore sulla **promessa 1** col messaggio `READ_BACK_MESSAGE`; tolta la guardia sul **secondo intento** muore sulla **promessa 6** col `SECOND_INTENT_MESSAGE`, cioè **dopo** aver superato le cinque precedenti sui propri meriti; rovesciato `replay` muore sulla **promessa 4** col `REPLAY_ORDER_MESSAGE`. La **mutazione di controllo** — un solo commento dentro `FileJournal` — lascia **28 target e 144 test verdi**. ⚠️ **Le quattro cifre di questo periodo sono del Task 9 e non sono state rifatte:** *«una promessa su otto»*, `28 target`, `144 test`, e le due qui sotto. Oggi le promesse sono **nove** e il workspace è a **29 target e 152 test**, ricontati il 2026-08-10 — ma il rosso **non** è stato riprodotto contro il codice di adesso, quindi le cifre restano quelle misurate, con la data accanto. ⛔ **E la separazione fra i due lati è misurata anch'essa:** con `FileJournal` rotta, `kernel --test journal_contract` resta **10 su 10 verde** e dentro il binario di `platform` restano verdi i **dieci** test inclusi, rosso il solo J12 — cifre del **Task 9**; oggi quei binari portano **undici** e **dodici** test. Se cadessero insieme, i due lati non sarebbero due |
| **F1** | una **rinumerazione fra indici esistenti** — `kind` 0 ↔ `effect` 1 → **rosso**, tre sonde su sei, col messaggio che nomina il formato cambiato. ⚠️ **La mutazione che il piano dettava** — `payload` 3 → 2, già di `trust` — **non compila**: `error: duplicate index numbers`, quindi il controllo non si sarebbe **mai visto scattare**, e proprio sull'unico oracolo che non si rigenera |
| **F2** | un campo spostato su un indice **libero** — `payload` 3 → 7 → **rosso**. Due mutazioni per la stessa regola, e in due forme diverse, com'è il contro-verso del gotcha **#48** |
| **F3** | ⛔ **le OTTO varianti dei tre enum `index_only` rinumerate UNA PER UNA** su un indice libero → **otto rossi su otto**. È ciò che rende la copertura reale invece di apparente: un record congelato solo ne fisserebbe **tre** — `RecordKind` ha 3 varianti, `EffectClass` 3, `Trust` 2 — e le altre cinque resterebbero tenute da **nulla**, come il Task 1 aveva misurato |
| **F4** | ✅ un campo **facoltativo** con `#[cbor(default)]` su un indice **libero** → **verde**: i byte congelati non si muovono. È la regola 3 di §4.9.2, e ADR-0036 è confermato dalla misura invece che citato |
| **F5** | ⛔ **la metà che rende F4 non vacua:** `Some(9)` invece di `None` → **22 byte** invece di 21, con `86` e il valore in fondo. Senza di essa, *«i byte non si sono mossi»* sarebbe compatibile con un campo che sul filo **non arriva mai**, e il verde non proverebbe l'additività. Gotcha **#54** |
| **F6** | **controllo**: una parola di un commento di `record.rs` → **6 passed**, zero rossi |
| **W1** | `to_be_bytes` → `to_le_bytes` **in entrambi i siti** di `framing.rs` → **rosso**, e ne uccide **DUE**: `the_declared_length_is_four_bytes_big_endian` e `a_frame_with_a_tail_is_refused`. ⛔ **La seconda morte è LEGITTIMA e non un difetto della sonda**, misurata invece che dedotta: la sonda della coda porta un letterale **big-endian**, `00 00 00 01`, che letto little-endian vale **16 777 216** — quindi il corpo da tre byte è *troncato* e non *con una coda*, e la risposta passa da `TrailingBytes` a `Incomplete`. ⛔ **Ciò che si temeva NON è successo:** il **round trip sopravvive**, quindi la sonda dell'ordine prova l'**ordine** e non la simmetria fra i due siti — vedi **W7** |
| **W2** | `if body.len() < declared` → `if false` → **rosso**, e **solo** `a_truncated_frame_is_refused`. È il **troncamento**, che nessun decodificatore CBOR può vedere: la coda non c'è, e il CBOR può essere completo lo stesso |
| **W3** | `if body.len() > declared` → `if false` → **rosso**, e **solo** `a_frame_with_a_tail_is_refused`. È la **coda FUORI dalla busta** |
| **W4** | tolta l'annotazione di stringa di byte da `FromWorker::Fragment` → **rosso**, e **solo** `the_byte_string_annotation_is_measured_and_not_asserted`, col messaggio `encoded … bytes`, che riporta la dimensione della forma **senza** annotazione. ⛔ **E il corpo della sonda NON PUÒ essere tutto zeri, che è ciò che rende la sonda non vacua:** CBOR codifica `0..=23` in **un** byte, quindi con `[0u8; 4096]` le due forme costano **uguale** ed **entrambe** stanno sotto il limite — la mutazione non ucciderebbe **niente**. ⛔ **Le cifre non stanno qui:** il commento di `the_byte_string_annotation_is_measured_and_not_asserted` in `crates/kernel/tests/worker_wire.rs` porta la misura del 2026-08-31, il metodo — sonda usa-e-getta da fuori la crate, contro un tipo specchio identico a meno dell'attributo, compilata, eseguita e cancellata nella stessa corsa — e la scomposizione byte per byte. ⚠️ **Ci stavano anche qui, verbatim, fino al 2026-08-31:** gotcha **#31**, e la regola dice che una cifra in due case si **toglie** invece di riallinearla |
| **W5** | tolto il controllo `decoder.position() != body.len()` da `FromWorker::decode` → **rosso**, e **solo** `junk_inside_the_declared_length_does_not_decode`. È la **coda DENTRO la busta**, dove la lunghezza dichiarata è onesta e a mentire è il corpo. È la riga che `Record::decode` porta già dal finding **AUD-047** |
| **W6** | in `FromWorker::encode`, `framing::frame(&body)` → `Ok(body)` → **rosso** su **tre**: i due round trip e `a_frame_with_a_tail_does_not_decode`. Senza busta il prefisso di lunghezza è il primo byte del CBOR, e `unframe` legge una lunghezza enorme |
| **W7** | ⛔ **la contro-direzione di W1, ed è metà dell'asserzione:** sotto W1 `a_framed_body_comes_back_exactly` **resta verde**. Se cadesse, la sonda dell'ordine non proverebbe l'ordine ma la **simmetria fra i due siti**, e l'ordine tornerebbe indifendibile appena qualcuno li cambiasse insieme — che è esattamente il caso che conta, perché **entrambi i pari vivono fuori da questo workspace** |
| **W8** | ⛔ **la contro-direzione di W5, e prova che i guasti sono DUE e non uno:** sotto W5 `a_frame_with_a_tail_does_not_decode` **resta verde**. Se cadessero entrambe, uno dei due controlli sarebbe **dominato** dall'altro (gotcha **#45**) e la tabella della §3.2 del disegno — *«prenditori diversi»* — sarebbe da riscrivere. Misurato: non cadono insieme |
| **W9** | ⛔ **la via che NESSUNA sonda raggiungeva, ed è il ramo d'errore di `FromWorker::decode`:** il `map_err` che risponde `WireError::Malformed` diventa un `.expect("decode")` — cioè *il kernel va in panico su un frame malformato che arriva da un worker*. ⚠️ **Prima delle due sonde nuove questa mutazione lasciava l'INTERO workspace VERDE:** `cargo test --locked --workspace --no-fail-fast` con la mutazione dentro dava **39 bersagli, 282 passate, 0 fallite, 2 ignorate**, identico alla baseline. Delle cinque sonde di allora nessuna ci arrivava — i due round trip **decodificano**, `a_frame_with_a_tail_does_not_decode` muore prima **dentro `unframe`**, `junk_inside_the_declared_length_does_not_decode` **decodifica con successo** e cade sul controllo di posizione, e `framing.rs` non tocca `FromWorker`. ✅ **Con le due sonde nuove la stessa mutazione uccide, ed esattamente quelle due:** **282 passate, 2 fallite** nel workspace — `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode`, e nient'altro. ⛔ **Sono due `#[test]` separati e non uno con due asserzioni, e il rosso lo prova:** i morti sono **due**, mentre una sonda sola si fermerebbe alla prima asserzione e il secondo ingresso non verrebbe mai esercitato — gotcha **#14** |
| **W10** | `if bytes.len() < LENGTH_WIDTH` → `if false` in `unframe` → **rosso**, e **solo** `bytes_shorter_than_the_prefix_are_refused`. ⛔ **Uccide per PANICO e non per asserzione**, ed è la forma di questa guardia: senza di essa `bytes.split_at(LENGTH_WIDTH)` è chiamata su meno di quattro byte — `panicked at crates\kernel\src\framing.rs:83:32: mid > len`. ⚠️ **Non è il troncamento di W2:** lì la lunghezza dichiarata c'è ed è onesta, qui **non c'è affatto**. ⚠️ **Riga aggiunta in revisione il 2026-08-31**, quando la guardia era l'unica **esercitabile** di `framing.rs` e `wire/worker.rs` senza una riga qui e la sua copertura era **dedotta** invece che eseguita: `cargo test --locked --workspace --no-fail-fast` con la mutazione dentro dà **283 passate, 1 fallita**, quella sola |

#### Lo schema del canale `ipc` — Traguardo 6, Compito 4, e NESSUNA riga di catalogo si muove

⛔ **Questa sezione registra un MECCANISMO e non una riga del catalogo, e dirlo è la metà che si
dimentica.** Misurato invece che dedotto il 2026-08-31, delimitando per intestazione come il
gotcha **#26** prescrive: nessuna riga di §7.4.1 o §7.4.2 nomina la **§6.1**.

```bash
awk '/^#### 7\.4\.1/{f=1} /^#### 7\.4\.3/{f=0} f' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | grep '§6\.1\([^0-9]\|$\)'
```

⚠️ **La classe finale del filtro non è decorazione, ed è una divergenza dal piano registrata
invece che appianata:** il piano dettava `grep '§6\.1'`, che rende **righe di §6.10** —
un'altra sezione, perché `§6.1` ne è il prefisso. Il verdetto **non** cambia; a essere sbagliato
era il comando che lo dimostrava, ed è la specie che questo file chiude scrivendo il comando e
non il numero.

⛔ **Quindi nessun numeratore si muove, e va scritto.** La riga della **campagna DST** resta
`PARZIALE` — le proprietà **2** e **3** di §5.7 si iniettano sulle porte `process` e `ipc`, e il
**trasporto è scaglionato**, quindi non è questo compito a chiuderla — e il gettone `Q13` resta
del **compito 6**. ⚠️ **Un compito che non muove un numeratore è quello su cui si è più tentati
di scrivere che l'ha mosso.**

✅ **L'INNESCO SU `Q13` È SCATTATO ED È STATO RACCOLTO, il 2026-09-01, dal compito 6** — un
innesco che scatta e che nessuno raccoglie lascia il registro a mentire con autorevolezza. La
riga del blocco **B** è coperta; la cifra si riconta sulla cella del blocco B in fondo a questo
file. ⚠️ **La riga della campagna DST resta `PARZIALE`**, e questo compito non la muove: le
proprietà **2** e **3** di §5.7 si iniettano su `process` e `ipc`, che è il **compito 9**.

⚖️ **E una voce si REGISTRA invece di prenderla.** Un caso `compile_fail` che tenesse *«un
verdetto non può portare un `Grant`»* sarebbe una **riga di catalogo nuova**, cioè §7.4, cioè
**spec** — vincolo globale 7, del **proprietario**. Oggi quella proprietà è tenuta dal **fatto
che `Verdict::Granted` è unitario** e dal doc accanto: è **livello 1 per costruzione e non per
un caso negativo**, e la distinzione si scrive invece di lasciarla intendere. ✅ **Verificata
col comando che esprime davvero il criterio** — `grep -rnE "\bGrant\b" crates/kernel/src/wire/`
— che rende **solo righe di commento**, e i commenti dicono **perché**. ⚠️ **Seconda divergenza
registrata:** senza i confini di parola lo stesso `grep` rende anche `GrantRequest` e `Granted`,
che non sono quel tipo — il criterio di chiusura del compito dettava la forma nuda, e presa alla
lettera sarebbe **insoddisfacibile** contro il codice che lo stesso compito detta.

**Le sonde** vivono in `crates/kernel/tests/ipc_wire.rs`, **fuori dalla crate** come le gemelle
di `framing.rs` e `worker_wire.rs` — se `IpcMessage` e `framing::frame` non fossero abbastanza
`pub` il banco non compilerebbe. ⚠️ **Quante siano lo dice il comando** e non questa riga
(gotcha **#31**): `grep -c '^#\[test\]' crates/kernel/tests/ipc_wire.rs`.

⛔ **E il banco nasce con le DUE sonde che il piano non poteva conoscere.** Il piano è del
2026-08-30 e ne dettava **quattro**; le gemelle
`an_empty_body_in_an_honest_envelope_does_not_decode` e
`a_truncated_body_in_an_honest_envelope_does_not_decode` sono nate in `worker_wire.rs` il
**giorno dopo**, chiudendo la mutazione **W9**. Senza di esse il ramo
`map_err(|_| WireError::Malformed)` di `IpcMessage::decode` non sarebbe stato raggiunto da
**nessuna** sonda del workspace — è **W9 rifatta sul canale nuovo**, e la mutazione **G4** qui
sotto lo misura.

📌 **Le mutazioni portano la lettera `G` come la gui è il pari di questo canale**, per simmetria
con le **W** del canale verso i **worker**. ⚠️ **Non sono il finding `G-5`**, che porta il
trattino e vive in un'altra tabella.

| Mutazione | Esito **misurato** il 2026-08-31 |
|---|---|
| **G1** | in `IpcMessage::encode`, `framing::frame(&body)` → `Ok(body)` → **rosso su tutto ciò che attraversa la busta, e su nient'altro** — `cargo test --locked --workspace --no-fail-fast` con la mutazione dentro dà **40 bersagli, 285 passate, 6 fallite, 2 ignorate**. ⛔ **E il piano ne prevedeva TRE** — i due giri completi e `a_message_with_a_tail_does_not_decode`: la divergenza si **registra** invece di appianarla, e le morti in più sono **legittime**, ciascuna con la propria causa misurata, come la seconda morte di **W1** sul canale gemello. `junk_inside_the_declared_length_does_not_decode` muore **per panico** — `range start index 4 out of range for slice of length 2` — perché senza busta `good` è più corto del prefisso che la sonda taglia; `a_truncated_body_in_an_honest_envelope_does_not_decode` muore con `left: Ok(Verdict(Queued))` perché senza busta il taglio cade su byte diversi e produce un messaggio **completo e valido** invece di uno troncato; `a_queued_verdict_survives_the_round_trip` muore con `left: Err(Incomplete)`, per la stessa causa dei giri completi. ✅ **La sola sopravvissuta è l'unica che non passa da `encode`**, `an_empty_body_in_an_honest_envelope_does_not_decode`, ed è ciò che rende l'esito leggibile: muore **tutto** ciò che attraversa la busta, e **solo** quello |
| **G2** | ⛔ **È L'ORACOLO DI QUESTO COMPITO, e la colonna «e NON» è metà dell'asserzione.** L'encoder scrive **sempre** il discriminante della prima variante — `body[0] = 0` subito dopo la codifica, ed è la forma scelta **leggendo il codice scritto**, perché col derive il tag lo scrive la macro e non c'è una riga da girare → **rosso sui DUE giri del verdetto e su nient'altro** — `cargo test --locked --workspace --no-fail-fast` con la mutazione dentro dà **40 bersagli, 289 passate, 2 fallite, 2 ignorate**: `a_verdict_survives_the_round_trip` con `left: Err(Malformed)` contro `right: Ok(Verdict(Refused { asked: Mib(4096), ceiling: Mib(1024) }))`, e `a_queued_verdict_survives_the_round_trip` con `left: Err(Malformed)` contro `right: Ok(Verdict(Queued))`. ✅ **E `a_grant_request_survives_the_round_trip` RESTA VERDE**, perché la sua è già la prima variante. È la **sola** prova che il corpo è davvero **un'enumerazione** e non un tipo solo travestito, cioè ciò per cui §6.7 chiede **due** messaggi: se morisse **anche** il giro della richiesta, il discriminante non starebbe distinguendo niente |
| **G3** | tolto il confronto `used != body.len()` da `IpcMessage::decode` → **rosso su UNA sola** — `cargo test --locked --workspace --no-fail-fast` con la mutazione dentro dà **40 bersagli, 290 passate, 1 fallita, 2 ignorate** — ed è `junk_inside_the_declared_length_does_not_decode`, con `left: Ok(Verdict(Granted))` contro `right: Err(Malformed)`. È la **coda DENTRO la busta**, dove la lunghezza dichiarata è onesta e a mentire è il corpo. ✅ **E `a_message_with_a_tail_does_not_decode` RESTA VERDE**: quella è la coda **FUORI** dalla busta, che prende `framing::unframe`. I due guasti hanno **prenditori distinti**, ed è la coppia che **W5** e **W8** misurano sul canale gemello. ⚠️ **La quaterna è misurata qui come nelle altre quattro celle**, e non per uniformità: il «e NON» di questa riga afferma qualcosa su **tutto il workspace**, e il solo banco non lo regge — `a_message_with_a_tail_does_not_decode` vive nello stesso file, il resto del workspace no |
| **G4** | ⛔ **il ramo d'errore di `decode`**: `.map_err(\|_\| WireError::Malformed)?` diventa `.expect("decode")` — cioè *il kernel va in panico su un frame malformato che arriva dalla gui*. Misurato **sull'intero workspace** e non solo sul banco — `cargo test --locked --workspace --no-fail-fast` con la mutazione dentro dà **40 bersagli, 289 passate, 2 fallite, 2 ignorate** — e le due fallite sono `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode`, **e nient'altro**. ⛔ **I morti sono DUE perché sono due `#[test]` separati** (gotcha **#14**): una sonda sola si fermerebbe alla prima asserzione e il secondo ingresso non verrebbe **mai** esercitato. ⚠️ **Le quattro sonde che il piano dettava lo avrebbero lasciato tutto verde**, che è esattamente ciò che **W9** aveva misurato sul canale gemello il giorno prima |
| **G5** | ⛔ **LA VARIANTE DI MEZZO DEL VERDETTO, CHE NESSUNA SONDA TOCCAVA:** l'encoder scrive `Verdict::Queued` col discriminante di `Granted` — `body[1] = 0` quando il messaggio è `IpcMessage::Verdict(Verdict::Queued)`, ed è la forma di **G2** un livello più sotto, scelta per la stessa ragione: col derive il tag lo scrive la macro e non c'è una riga da girare → **rosso su UNA sola**, `a_queued_verdict_survives_the_round_trip`, con `left: Ok(Verdict(Granted))` contro `right: Ok(Verdict(Queued))`. ⛔ **E il «e NON» qui è TUTTO IL RESTO DEL WORKSPACE:** `cargo test --locked --workspace --no-fail-fast` con la mutazione dentro dà **40 bersagli, 290 passate, 1 fallita, 2 ignorate**, cioè la sonda distingue qualcosa che **nient'altro** distingue — ed è la sola cosa che la rende non vacua. ⚠️ **Prima di lei `Queued` non raggiungeva mai il filo:** `Refused` fa il giro completo, `Granted` è codificata da due sonde, e la variante di mezzo da **nessuna**, quindi un tag sbagliato su di lei non sarebbe stato letto da nessuna asserzione del workspace |

⛔ **Le mutazioni sono state applicate UNA ALLA VOLTA, compilando in un passo separato
dall'eseguire, e revocate da una copia presa PRIMA** (decisione **D7**): `cmp` identico dopo
ciascuna, e il banco torna **interamente** verde. ⚠️ **`git diff --stat` qui non prova niente**,
perché `crates/kernel/src/wire/ipc.rs` non era ancora tracciato e su un file nuovo quel comando è
**vacuo** — è la ragione per cui la revoca si verifica con `cmp` contro la copia e non col diff.

#### Il build script — entrato su una lacuna misurata

⛔ **La riga del build script è entrata il 2026-08-09 su una lacuna _misurata_, e allora le
voci di livello 2 erano sei.** ⚠️ **Legata alla data e non alla posizione, il 2026-08-10:**
diceva *«è la sesta voce di livello 2»*, vero quando fu scritto e falso da quando la riga dei
test di contratto è entrata poche ore dopo. È la terza forma del gotcha **#31**: un'affermazione
vera legata al **contenitore** invece che a ciò che fu misurato, e il contenitore cresce senza
avvisare.

⛔ **RICHIAMO DEL 2026-08-28, finding AUD-063 — la correzione del 2026-08-10 aveva rimesso un
conteggio del contenitore dentro la frase che lo vieta.** Diceva, subito dopo *«poche ore
dopo»*: *«— nella tabella di oggi, che ne ha **sette**, il build script sta **quinto**»*, e le
righe di dati della tabella del Livello 2, **contate il 2026-08-28**, sono **nove** — le due che
mancavano sono entrate coi Task 9 e 10 del Traguardo 3, il contratto `journal` e i byte
congelati. ⚠️ **Quel «nove» sta qui perché è dentro un verbale datato**, che dice cosa fu
misurato quel giorno; è la distinzione della **55ª** misura, e la stessa cifra scritta al
presente nel paragrafo sopra sarebbe il difetto rifatto. ⛔ **Tolta e non riallineata, e con essa
l'ORDINALE**, che era ancora giusto **per caso**: le due righe nuove sono finite in fondo, e una
inserita prima lo avrebbe falsificato allo stesso modo — cioè era legato al contenitore quanto il
conteggio, che è precisamente ciò che la frase dichiara di aver corretto. 📌 **Al suo posto non
c'è un numero nuovo ma una regola di lettura** (precedente **AUD-007**): quante siano le voci di
livello 2 lo dice **la tabella**, che è la loro casa unica, e questa sezione narrativa non ne
tiene una seconda copia.

Un `crates/kernel/build.rs` che chiama `SystemTime::now()`,
`fs::metadata()` e `env::var()` e inietta il risultato con `cargo:rustc-env` lasciava la porta
**verde su sei controlli su sei**: `build` e `test` lo compilano perché è il mestiere di un
build script, `gate-no-os.sh` lo compila **per l'host anche con `--target`** e **lo esegue**,
`gate-deps.sh` non vede nodi nuovi se lo script non ha dipendenze proprie, `gate-attributes.sh`
leggeva solo `src/lib.rs`, e `check-docs.sh` non guarda il codice. Difende **`I3` e `V29`
direttamente** — non è di ramo 1b: `cargo:rustc-env` più `env!()` cuoce nel kernel un valore
letto dal mondo alla build, che è il gotcha #28. **Il rimedio è TOGLIERE**, come per il grafo
spedito, e il messaggio dello script lo dice insieme al perché — un controllo che sembra
pedanteria viene aggirato. Vive dentro `gate-attributes.sh` perché è **il punto cieco di quello
script**: `build.rs` ha attributi propri e il `forbid` di `lib.rs` non lo raggiunge.

#### Tre note sui limiti dichiarati di questi controlli

📌 **`gate-attributes.sh` è un controllo di testo, e non va promosso.** Cerca gli attributi
con `grep` ancorato a inizio riga: prova che il divieto sia **dichiarato**, non che nel
kernel non ci sia `unsafe`. Quella la prova il compilatore, ed è proprio ciò che questo
controllo tiene in piedi. Costo dichiarato nello script stesso: l'ancora chiude il caso
`// #![forbid(...)]`, che è quello reale, ma un attributo sepolto in un commento di blocco
`/* … */` sfugge ancora — chiuderlo richiederebbe un parser, cioè un rimedio più fragile
del buco.

⚠️ **B4 è la via _offline_, non una rete di sicurezza — e la differenza va detta.**
`rustup target list --installed` **riconcilia `rust-toolchain.toml` prima di rispondere**:
il manifesto dichiara `targets = ["x86_64-unknown-none"]`, quindi se il bersaglio manca
l'atto stesso di interrogarlo lo **reinstalla**, e la risposta è «c'è». Con la rete
disponibile la guardia **non può scattare**. Scatta quando la riconciliazione fallisce,
cioè **senza rete** — verificato: uscita 1 e messaggio corretto. È la via che rende
utilizzabile una macchina isolata, non un controllo che sorveglia la macchina connessa.

**Contro-sonde:** la colonna «deve restare verde» qui sopra, più
`crates/platform/tests/counter_probes.rs` al livello 1. ⚠️ Il punto che si dimentica è **perché**
`platform`, `secrets` e `daemon` restano verdi: non sono fra le crate vincolate, e un controllo
che scattasse anche lì sarebbe rosso per il motivo sbagliato (gotcha #24) — `platform` è **il
posto dove l'I/O deve vivere**.

#### Gli altri test che «cargo test --workspace» porta

**Cosa difende ciascuno.** ⛔ Stanno qui
perché il gotcha **#36** ha una forma pura e silenziosa: chi scrive un controllo lo considera
«scritto» e non lo riporta nel registro, e da fuori resta **indistinguibile da uno che non
esiste**. Nessuno di questi è una riga del catalogo — sono controlli di livello 2 che
sostengono righe del catalogo, o che tengono in piedi ciò che le righe presuppongono.

| File | Che cosa difende |
|---|---|
| `crates/kernel/tests/executor_determinism.rs` (**tredici** test — ⚠️ ricontati col `grep -c` il 2026-08-21: la cella diceva **dieci**, ed era giusta fino al 2026-08-18, quando le decisioni **K-1** e **B-1** dell'audit ne hanno portate **tre** — `a_request_written_before_the_run_belongs_to_nobody`, `a_request_written_by_a_destructor_belongs_to_nobody` e `the_delivered_turn_limit_is_honoured_by_its_value`. Quelle tre erano già **descritte** in questa cella e la **cifra** no: chi corregge una cella lunga aggiorna la prosa e salta il numero. Gotcha **#31**) | **C1, C2 e C3 sull'esecutore _spedito_**, non su quello dello spike: **cento** corse allo stesso seme danno una traccia sola, **duecento** semi distinti non ne danno una sola, e il tempo virtuale **non attende** — l'orologio si ferma a 20 000 ms dove il sequenziale arriverebbe a 60 000. Più le sonde di **non-vacuità**: che l'interfoliazione sia reale, che un blocco diventi **errore e non attesa infinita**, che un reattore che non avanza sia **errore e non giro a vuoto**, che un'attesa già scaduta svegli subito senza muovere l'orologio, che una richiesta di sospensione **non si erediti** fra attività, e che un rideposito perpetuo di una scadenza passata **termini comunque** |
| `crates/kernel/tests/ports_are_implementable.rs` (**quattordici** test — ⚠️ ricontati il 2026-08-21: la cella diceva **tredici**, giusta fino a **B-3** del 2026-08-18, che ha portato `a_restore_serves_the_checkpoint_it_was_asked_for_and_not_the_first_one`. ⛔ E il **tredici** vive ancora, giustamente, dentro quel test: è la misura dell'audit *«`restore` riscritta per prendere il primo lasciava verdi tutti e tredici»*, cioè il conto di **allora**. Gotcha **#31**) | il rimedio al gotcha **#46**: una **finta** per `Filesystem`, una per `Network`, due per `process` — `Worker` e `Process` — e una per `Ipc`, con chiamate che le esercitano in entrambe le direzioni. **Cinque finte per quattro famiglie**, ed è la copertura di **tutte** le porte dichiarate senza implementazione. È ciò che tiene in vita `Path::as_bytes()`, `Endpoint::as_bytes()` e il `Clone` su `Path` contro una passata YAGNI — su un tratto dichiarato **in anticipo** i chiamanti sono vuoti per costruzione, e il criterio non distingue il morto dalla sola porta d'ingresso di chi verrà — e prova che quelle firme siano **implementabili fuori dalla crate**, dove la privacy di modulo di una tuple-struct le renderebbe inutilizzabili. ⚠️ **Non** è una suite di conformità: quella pretende due implementazioni da confrontare |
| `crates/simulator/tests/memory_journal.rs` (**quindici** test — ⚠️ ricontati **sul binario** il 2026-08-10 col Task 11: la riga diceva **undici** e la cifra era già stantia prima di lui, gotcha **#31**) | il **doppio in memoria** del giornale (§4.1): che l'intento riletto torni intatto, che un passo mai scritto sia **`Missing` e non vuoto**, che un esito **senza** intento sia rifiutato **e** uno **dopo** il proprio intento accettato — le due direzioni, e la seconda mancava — che il rifiuto guardi **quale** passo, che `read_back` risponda con l'**intento** e non con l'esito, che ogni passo rilegga **il proprio** primo record, che un **secondo intento** sullo stesso passo sia **rifiutato senza scrivere** e che uno su un passo **diverso** resti **accettato** — le due direzioni della guardia decisa il 2026-08-10 — e che `prune` **rifiuti un passo in dubbio senza potarlo** — ⚠️ questa riga diceva *«rifiuti senza potare»* e descriveva la non-implementazione, superata dal Task 11 — e che risponda **`Missing`** a un passo mai scritto, che è la **terza** risposta e la sola che la conformità non tiene (voce aperta 2). ⚠️ **Non** è la suite di conformità, e dal 2026-08-10 la distinzione non è più «quella non esiste» ma **quella sta altrove**: `journal_contract.rs` porta ciò che **entrambe** le implementazioni promettono, questo file ciò che è vero **di questa sola** — l'ordine *dentro* un passo, il secondo intento, e che il giornale non sopravviva alla propria caduta. ⛔ **RICHIAMO DEL 2026-09-02 (E151):** *«l'ordine dentro un passo»* fra ciò che è vero *«di questa sola»* è **falso**: l'ordine dentro un passo è **dovuto dalla porta** — `crates/kernel/src/ports/journal.rs`, *«Re-reads EVERYTHING, in write order»* — ed è **tenuto dalla conformità** su **entrambe** le implementazioni, promessa 8c di `crates/kernel/tests/journal_contract.rs`, `[(step, intent), (step, note)]`. Questo file esercita il caso **due note su un passo** — `a_step_may_carry_many_notes_and_they_keep_their_order` — e la conformità no, misurato il 2026-09-02 con `grep -n '\.note(' crates/kernel/tests/journal_contract.rs`: le chiamate `.note` di `assert_journal_contract` che riescono stanno su passi diversi. ⛔ **E NELLA STESSA PASSATA, misurato il 2026-09-02, anche *«il secondo intento»* esce da «di questa sola»:** il **rifiuto** del secondo intento sullo stesso passo è la promessa **6** della conformità — `crates/kernel/tests/journal_contract.rs`, *«A SECOND `intent` on the same step is refused»* — su **entrambe** le implementazioni. Questo file vi aggiunge che il rifiuto **non abbia scritto** — `a_second_intent_on_the_same_step_is_refused`, dove `read_back` risponde ancora col primo — e la contro-sonda nominata `an_intent_for_another_step_is_still_accepted`, il cui commento dichiara la stessa proprietà tenuta anche dalla conformità, come effetto collaterale della promessa 4. La terza clausola, *«che il giornale non sopravviva alla propria caduta»*, è `the_memory_journal_does_not_survive_being_dropped` e non è toccata |
| `crates/simulator/tests/crashing_journal.rs` (**tredici** test — ⚠️ ricontati il 2026-08-21: la cella diceva **dieci**, giusta fino a **S-1**/**S-2** del 2026-08-18, che ne hanno portate **tre** — `the_success_path_of_note_is_exercised_and_counted`, `a_refused_note_does_not_consume_a_crash_position` e `a_refused_intent_does_not_consume_a_crash_position`. Il piano ne dettava **otto**, e le due arrivate dal pre-controllo sono la scrittura rifiutata e la potatura dopo la caduta. Gotcha **#31**) | ⛔ **il giornale che CADE** (§3.3, livello 1 dei due livelli di crash di ADR-0032) — ciò che promette **solo lui**, ed è per costruzione un bugiardo, quindi la conformità non lo tiene (gotcha **#50**): che cada **alla** scrittura dichiarata e non «da qualche parte», che dopo la caduta **ogni** scrittura successiva sia rifiutata — è la differenza fra un **crash** e un **disco cattivo**, decisione D2, ed è la permanenza a fermare *tutte* le attività interlacciate — che ciò che era stato scritto **sopravviva** e si rilegga da `into_survivor`, che `has_fallen` dica **no** finché non cade, e che un giornale a cui è stato detto di **non** cadere non cada mai in centoventotto scritture: quest'ultima è la direzione che si dimentica, e ⛔ **`C7a` poggia interamente su di lei** — misurato, è l'unica sonda **verde** sotto la mutazione «cade sempre» e **rossa** sotto «non cade mai». Più le due che il piano non chiedeva: che una scrittura **rifiutata dal protocollo** non consumi una posizione del conteggio — senza, il punto estratto scivola e con un punto vicino alla fine il guasto **non scatta**, gotcha **#17** — e che `prune`, **unica operazione mutante fuori dalle tre contate**, sia rifiutata dopo la caduta **senza armarla né consumarla**. ⚠️ **Una sonda è dichiarata non falsificabile invece che tolta:** `the_same_seed_chooses_the_same_write` non può fallire perché `from_seed` è pura, e la determinatezza vera la tiene `seeded_rng.rs` — postura del gotcha **#44** |
| `crates/simulator/tests/arbiter_campaign.rs` (**cinque** test — contati **sul binario** il 2026-08-25 con `cargo test --locked -p simulator --test arbiter_campaign`, non per aritmetica) | ⛔ **la campagna DST dell'arbitro** — le proprietà **1**, **4** e **5** di §5.7 sotto un interlacciamento scelto dal seme, che è l'unico posto in cui l'arbitro gira **dentro** l'esecutore. Due oracoli di non-vacuità distinti — *«lo scenario fa davvero decidere l'ammissione»* e *«la campagna spazza più di un mondo»* — e la sezione dedicata qui sopra porta le misure, la campagna di mutazione e il **mutante vivo** |
| `crates/simulator/tests/dying_gui.rs` (**dieci** test — contati **sul binario** il 2026-09-02 con `cargo test --locked -p simulator --test dying_gui`, non per aritmetica) | ⛔ **ciò che promette SOLO `DyingGui`**, sul precedente di `crashing_journal.rs`: muore all'operazione detta, resta morta su `send` **e** su `receive`, un client morto non viene ripresentato da `accept`, ciò che ha detto prima di morire è una richiesta **davvero codificata**, un'operazione nominata su un altro client non consuma una posizione, una finta detta immortale non muore mai, stesso seme stesso punto, il punto estratto sta **dentro** le operazioni del percorso, **ogni** operazione può essere quella, e `has_died` dice no finché non muore. ⚠️ `the_same_seed_chooses_the_same_operation` è **infalsificabile per costruzione** e lo dichiara (gotcha **#44**): non conta come copertura |
| `crates/simulator/tests/gui_death_campaign.rs` (**due** test — contati **sul binario** il 2026-09-02) | ⛔ **la proprietà 3 di §5.7** — la gui muore tenendo una concessione discrezionale e la somma torna alla linea di base. La morte si legge **solo** come `Err(IpcError::Disconnected)` attraverso la porta, mai chiedendo alla finta — `grep -cE '\.has_died\(' ` su questo file dà **0**, cioè le **chiamate**; ⛔ **il comando NON ancorato è un'altra misura e rende un numero maggiore di zero**, perché conta anche i paragrafi che dichiarano di non chiamarla. Linea di base **non zero** e di **due specie**: la quota di presentazione del core, che nessun registro tiene, e un **secondo client** registrato che non muore. ⛔ **RICHIAMO DEL 2026-09-02, PRIMA ONDATA DI REVISIONE — questa cella portava la forma NON ancorata del comando col valore *«0»*, ed era falsa** mentre la sezione del compito 9 portava già quella giusta: la stessa misura in **due** case, una vera e una no, che è la radice **R1** alla lettera. ⚠️ **E diceva *«Due oracoli di non-vacuità, visti rossi entrambi»*, accreditando un rosso a un'asserzione aggregata che non poteva produrlo:** quell'asserzione è stata **TOLTA** nella stessa ondata, e l'oracolo *«il guasto è scattato»* è tenuto dall'`assert!` **per seme** dentro `run` — che è più forte, perché vale su ogni seme invece che sulla somma, ed è da dove il rosso veniva davvero |
| `crates/simulator/tests/worker_kill_campaign.rs` (**un** test — contato **sul binario** il 2026-09-02) | ⛔ **la proprietà 2 di §5.7** — nessun processo gira senza concessione valida, asserita **sui libri** dopo **ogni** kill e non alla fine. Quattro worker avviati con concessioni vere, uccisi in istanti estratti da una mescolanza **derivata**. ⛔ **Tre controlli di non-vacuità, VISTI ROSSI tutti e tre, e la provenienza delle mutazioni non è la stessa:** *«i kill sono scattati»* — nessuna recluta sopravvive al proprio kill — con `MC3`, che muta **il banco**, perché `order` è una permutazione e nessuna mutazione di produzione lo raggiunge (dichiarato accanto all'asserzione); *«c'era qualcosa da verificare»* — almeno un `Released::Now`, cioè un kill **dentro** la finestra — con `MC1`, e *«il seme conta»* con `MC2`, entrambe sul **generatore** (`RngExt::below`, codice di produzione). ⚠️ **RICHIAMO DEL 2026-09-02, quarta ondata:** questa cella diceva *«Due oracoli … VISTI ROSSI entrambi con mutazioni sul generatore»* e accoppiava all'oracolo 1 il rilevatore di degenerazione, mentre l'oracolo 1 **non esisteva** fino alla terza ondata. ⚠️ La metà **temporale** di *«concessione valida»* si **conta e si dichiara** e non si asserisce — `E30`/`E39`, voce del proprietario, gotcha **#73** |
| `crates/simulator/tests/dst_campaign.rs` (**cinque** test, uno dei quali `#[ignore]` — ⚠️ erano **due** al Task 2 e **quattro** al Task 3; la cifra è rimasta a quattro quando il **Task 4** ha aggiunto la campagna profonda, ed è stata ricontata **sul binario** al Task 9. Gotcha **#31**) | ⛔ **la campagna DST di livello 1** (§3.3, ADR-0032): il soggetto sotto esame è la **riconciliazione del kernel**, e nulla tocca un disco. Consegna lo **scenario giornalato** — quello di M-2, tre attività per quattro passi, ora con intento ed esito attraverso la porta — la **traccia**, che sarà l'oracolo **indipendente** di `C7b` perché viene da ciò che le attività hanno saputo essere passato e non dall'archivio, e **`C7a`**: senza crash, **nessun** passo in dubbio, su cinquanta semi. Più il pin che **fissa** `WRITES_PER_RUN = 24`, senza il quale il punto di caduta si estrarrebbe contro un numero non verificato (gotcha **#17**). ⛔ **E l'oracolo di non-vacuità di `C7a`, che il piano non chiedeva ed è la riga che conta:** *«nessun passo è in dubbio»* e *«lo scenario non ha scritto niente»* erano **lo stesso verde** — misurato con un giornale che cade alla scrittura zero. Ora il ciclo pretende `writes_done() == WRITES_PER_RUN` **prima** di guardare l'insieme, ed è il gemello di `has_fallen()` sull'altra metà della campagna. ⚠️ **Ciò che questo file NON tiene, ed è dichiarato sul doc di `run`:** che lo scenario **interlacci**. Nessuna sua sonda va rossa se le tre attività girano una dopo l'altra; la proprietà la tiene `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed`, un compito più in là — e il **nome** è un vincolo, perché il rimando lo cita. ✅ **E dal Task 3 quella sonda esiste, in questo stesso file**, insieme a **`C7b`**: il crash lascia **quell'insieme e non un altro**, confrontato con l'oracolo `expected_doubt` che viene dalla **traccia** e non dall'archivio — la sola ragione per cui `C7b` non è una tautologia, e ⛔ **misurata**: rompendo l'oracolo, il confronto va rosso con `[3]` dall'archivio contro dodici passi dalla traccia. Più il **confronto ordinato** invece che insiemistico, che morde davvero — un `replay` ordinato per passo dà `left: [0, 4]` contro `right: [4, 0]`, ed è il difetto che una tabella `redb` chiavata sul passo produrrebbe da sola. ⛔ **E `C7b` ha DUE oracoli di non-vacuità e non uno, perché provano cose diverse:** che **ogni** seme raggiunga il proprio punto di caduta — uguaglianza e non `> 0`, perché un seme che non cade significa che lo scenario ha scritto meno del numero contro cui il punto è estratto — e che **almeno un seme lasci più di un passo in dubbio**, senza cui la campagna confronterebbe insiemi vuoti restando verde. ✅ **Dal Task 4 `C7b` È la campagna breve**: il corpo per-seme vive in `campaign(seeds)`, che la campagna profonda riusa sotto `#[ignore]`, e non esiste un secondo ciclo più debole accanto — ⛔ il piano ne dettava uno, e sarebbe stato **quello** a finire nel cancello. ⛔ **E il numero di semi ha una TERZA guardia, che è il criterio con cui è stato scelto:** gli insiemi in dubbio distinti che questo scenario può produrre sono **centonove**, e la campagna pretende di vederli **tutti** — `EXPECTED_DOUBT_SETS`. Non è una proprietà ma un **rilevatore di cambiamento sulla forma dello scenario**, nella postura dei byte congelati, e ⛔ **è stato adottato solo dopo aver misurato che non scattasse dove non deve**: sei costanti di mescolamento diverse danno centonove tutte e sei, quindi il conteggio è dello **scenario** e non dei semi. Provato in due direzioni — a cinquecento semi ne vede centocinque e scatta |
| `crates/kernel/tests/reconciliation.rs` (⚠️ **undici** test — ricontati **sul binario** il 2026-08-10 chiudendo il traguardo: la cella diceva **nove**, e le due arrivate dopo sono del **Task 7**, la nota che non apre un dubbio e la nota che non tocca quello del chiamante. Gotcha **#31**) | la **riconciliazione** (§4.3, ADR-0007) — il primo consumatore di `replay()`: che un crash lasci **più** passi in dubbio e non uno (gotcha **#20**, `[3, 7]` col seme 99), che un passo con intento **ed** esito **non** sia in dubbio (la direzione che si dimentica), che la **classe decida** la risoluzione sui tre valori, e che un record indecifrabile valga `SuspendAndAsk`. ⚠️ **Quattro sonde che il compito non chiedeva:** il giornale **vuoto** — il primo avvio, che nessuna sonda dettata incontrava — l'**ordine di scrittura** scritto `7, 3, 1` perché quella dettata attendeva `[3, 7]`, che è ordine di scrittura **e** ordine numerico insieme, e le **due dell'insieme**: al più una voce per passo, e un passo che rientra **conserva il posto**. ⛔ **Ciò che questo file NON tiene, ed è dichiarato in `reconcile.rs`:** che il `kind` del record concordi con l'operazione che l'ha scritto. ⚠️ **Questo rimando diceva *«vedi la voce aperta in fondo»* e puntava a una voce che non è più aperta:** la questione delle **due verità** è **chiusa dal proprietario** il 2026-08-10 — come **decisione** e non come garanzia — e a valle esiste `the_promotion_writes_through_note_and_the_record_says_note`, che l'accordo lo fissa per **l'unico scrittore che esiste** |
| `crates/platform/tests/file_journal.rs` (⚠️ **sei** test su Windows e **sette** su Linux — ed è il **primo conteggio del registro che dipende dal sistema**, dichiarato invece di scegliere un numero: il settimo è `cfg(unix)`) | ⛔ **La settima è `the_journal_file_is_not_world_readable`, finding PL-1, dal 2026-08-18.** ADR-0023 promette che il giornale a riposo sia *«protetto quanto il tuo account di sistema»*, e `OpenOptions::create(true)` da solo chiede `0o666 & !umask`, cioè **0644** su un Linux di serie: **leggibile da chiunque**, cioè **meno** dell'account. ✅ **Misurato su un Linux vero (WSL, `umask` 0022) invece che dedotto dai doc di `std`:** `open` a `0o666` dà **644**, a `0o600` dà **600**. ⚠️ **L'asserzione è «nessuno tranne il proprietario» (`mode & 0o077 == 0`) e non «esattamente 0600»**, perché `mode()` è ancora mascherato dall'umask: un'uguaglianza esatta andrebbe **rossa su un sistema più chiuso del richiesto**, cioè dove la promessa è **mantenuta**. ⚠️ **Direzione «deve scattare», provata dalla misura del sistema e non da una corsa del banco mutato:** senza la riga il file nasce **644**, e `644 & 0o077 = 0o044 ≠ 0`. ⛔ **E il difetto era INVISIBILE sull'host di sviluppo** — Windows non ha il modo Unix, quindi `cfg(unix)` lo compila via e il rosso poteva uscire **solo sul secondo sistema previsto dal progetto**: è il gotcha **#52** nella stessa forma. Il percorso Unix è stato **type-checkato** prima del push con `cargo check --target x86_64-unknown-linux-gnu --tests`; il **valore** lo misura la CI. ⚠️ **Limite dichiarato:** `mode()` è ignorato se il file **esiste già**, quindi un giornale creato prima di questa riga resta 0644 per sempre — è una **migrazione**, e la fixture cancella la cartella all'ingresso, quindi questa sonda **non può vederla**. ⛔ **Solo il file e NON la cartella**, ed è la scelta del proprietario fra le due: `0700` sulla cartella coprirebbe anche gli archivi futuri, ma **la cartella non ha un proprietario nel codice** — nessuno la crea — quindi la regola nominerebbe un chiamante che non esiste, che è il difetto di **A-7**. ⚠️ **Non è una riga di catalogo:** aggiungerla alla §7.4 è una decisione del proprietario, e finché non c'è questa sonda è **registrata qui come voce aperta** invece che come nota — gotcha **#36**. — E le altre sei sono ciò che **solo** il giornale su file promette (§4.1, ADR-0032), e che pretenderlo in conformità renderebbe rossa la finta — gotcha **#44**: che una scrittura **sopravviva alla riapertura**, che una transazione **mai confermata non lasci nulla** (requisito 1 di §10.6), che il contatore delle chiavi **riprenda dall'archivio** invece che da zero — altrimenti la seconda sessione **sovrascrive** la prima in silenzio — che la guardia sul **secondo intento** regga **attraverso una riapertura**, perché legge l'archivio e non un campo della sessione, e che il **lucchetto** rifiuti un secondo giornale sullo stesso file **mentre il primo è aperto** (l'altra direzione la tiene la prima sonda: chiuso il primo, la riapertura riesce). ⛔ **E la sesta è la prova che il confine è reale:** `CountingBackend` è una **seconda implementazione di `redb::StorageBackend` scritta da fuori la crate`**, `FileJournal` ci gira sopra invariato, e i contatori dicono che l'I/O **passa davvero di lì** — senza quell'asserzione un giornale che accettasse il backend e scrivesse altrove resterebbe verde. È il rimedio al gotcha **#46** applicato al confine su cui il **Traguardo 4** inietterà i guasti di livello 2. ⚠️ **Non** è la suite di conformità: quella sta in `journal_contract.rs` e gira contro **entrambe** dalla riga qui sotto |
| `crates/platform/tests/journal_contract_real.rs` (**un** test proprio, ⚠️ **quindici** eseguiti — ricontati **sul binario** il 2026-08-17 chiudendo T-1 e T-2: la cella diceva **dodici**, giusta fino ai tre bugiardi nuovi. Prima ancora diceva **undici**. Gotcha **#31**, terza volta su questa cella) | ⛔ **la conformità della porta `journal` contro l'implementazione VERA**, e il file è corto perché **le asserzioni non si ripetono**: `include!("../../kernel/tests/journal_contract.rs")` le raggiunge testualmente, come `reactor_contract_real.rs` fa per `reactor`. Due copie divergerebbero e **la prima che diverge mente stampando `ok`**. ⚠️ **Costo dichiarato e non nascosto:** l'inclusione porta con sé anche i `#[test]` del file incluso, quindi la finta, i **dodici bugiardi** e la sonda delle sottostringhe **girano una seconda volta** dentro il binario di `platform` — **quindici** test in tutto, di cui **uno solo** tocca il disco. ⚠️ **Questa frase diceva «otto bugiardi» e «undici test»**, ed erano le cifre del Task 9; poi «nove» e «dodici», che erano quelle del Task 11. ⛔ **E il 2026-08-17 è stata corretta l'INTESTAZIONE della cella e non il suo CORPO**, che ha continuato a dire nove e dodici per una passata intera: è la radice **R1** dell'audit — *una correzione attraversa il punto in cui nasce, non gli altri* — commessa dentro la passata che quella radice stava chiudendo. Gotcha **#31**. ⛔ **E la fabbrica dà un file NUOVO a ogni chiamata — dieci per corsa, non nove — invece di cancellarne uno fisso:** su Windows la cancellazione **fallisce in silenzio** se il file è ancora aperto e la fabbrica riaprirebbe **i dati vecchi** (gotcha **#52**), `FileJournal` tiene un **lucchetto esclusivo**, e la promessa 4 conta l'**intero** archivio. La numerazione passa da un `AtomicU64` perché `assert_journal_contract` prende **`Fn`**, non `FnMut`. La cartella è **una per call site**, dal `line!()`, con un **prefisso diverso** da quello di `file_journal.rs`: i due binari girano insieme e un numero di riga è unico dentro **un** file solo. Sonda **J12** |
| `crates/platform/tests/engine_crash_consistency.rs` (**sei** test, uno dei quali `#[ignore]` — ⚠️ erano tre al Task 5 e cinque al Task 6; ricontati **sul binario**) | ⛔ **il LIVELLO 2 dei due livelli di crash** (ADR-0032, §4.6): il soggetto sotto esame **non è il kernel** ma **`redb` stesso**, guidato attraverso un `StorageBackend` che cade a un'operazione scelta. ⛔ **Vive in un banco di prova e non in `platform/src/`, ed è il punto:** ciò che il Task 8 del Traguardo 3 comprò è che quel confine sia raggiungibile **da fuori la crate** (gotcha **#46**), e un backend cadente scritto **dentro** `platform` non proverebbe nulla su quello. Tiene: che senza caduta l'archivio **si riapra con tutto dentro** — la direzione che si dimentica, messa **per prima**, perché se cadesse ogni rosso successivo parlerebbe del backend invece che dell'iniezione — che il backend **cada all'operazione dichiarata** e non prima, e che la caduta sia **permanente**. ⛔ **E l'oracolo che chiude il gotcha #51 è `the_engine_really_syncs_and_that_is_what_closes_gotcha_51`, ed è un DELTA e non un conteggio:** sei sync su sette nascono **prima che esista un record**, quindi *«`sync_data` è stato chiamato»* è soddisfatto da un motore che **non sincronizza nessuna scrittura** — misurato, con `Durability::None` la forma assoluta resta **verde** e quella a delta va rossa. ✅ **E dal Task 6 tiene anche la coerenza dopo la riapertura, con QUATTRO oracoli e non due:** che quel che torna sia un **prefisso** di quel che è stato scritto — mai un record parziale o mescolato, misurato su trentacinque punti — che **ogni** punto scatti (uguaglianza e non `> 0`, perché l'intervallo si ferma alla **saturazione**: oltre, la corsa è indistinguibile da una senza iniezione) · che **almeno un'iniezione abbia accorciato l'archivio**, senza cui il confronto a prefisso è **banalmente vero** · che **non le abbia accorciate tutte**, che è la direzione opposta · e ⛔ **che esistano punti che restituiscono ALCUNI ma non TUTTI i record**. ⛔ **Quest'ultimo è ciò che rende il ciclo un secondo testimone del gotcha #51, e senza di esso il ciclo non contribuiva nulla:** misurato, con la sola coppia di oracoli dettata dal piano il ciclo è **interamente verde** quando la durabilità sparisce, mentre coi gradini è **rosso** — senza durabilità la scala collassa a **zero-o-tutto**. E la sua virtù è di **non dipendere dalla costante fragile**: non conta operazioni, conta gradini. ⚠️ **Ciò che la chiusura del #51 NON compra** — la morte vera del processo, l'ordine fra `write` e `sync_data`, il commit di `prune`, e un supporto che possa davvero perdere una scrittura non sincronizzata — è scritto per esteso in [`riferimenti.md`](riferimenti.md), perché *«il #51 è chiuso»* nella forma nuda mentirebbe. ✅ **E dal Task 7 i cinque controlli vivono in un CORPO SOLO che due profondità chiamano**, invece che in una campagna nuova accanto alla vecchia — ⛔ il piano ne dettava una seconda con **un** oracolo invece di cinque, e sarebbe stata **quella** a rappresentare la campagna. ⛔ **La campagna profonda approfondisce lo SCENARIO e non lo spazzamento**, e la misura che lo decide è che allargare l'intervallo **non compra niente**: a ottocento punti ne scattano sempre trentacinque, perché oltre la saturazione la corsa è indistinguibile da una senza iniezione. La profondità invece compra stati nuovi **uno per record** — le lunghezze di prefisso distinte sono `record + 1` a ogni profondità misurata — e a trenta record `partial > 0` regge, quindi il testimone del **#51** non è un accidente dello scenario piccolo |
| `crates/kernel/tests/dependencies_usable.rs` (due test) | che le voci **spedite** dell'allow-list **compilino e facciano round-trip** — gotcha #22, `cargo add bincode` risolve a una versione il cui intero sorgente è un `compile_error!`. E per `bincode` i **byte consumati** sono pari alla lunghezza dichiarata, che è la regola imposta dal gotcha **#34**: un decodificatore che si ferma al primo elemento completo e ignora la coda «ha decodificato» senza provare niente |
| `crates/kernel/tests/ipc_wire.rs` (⚠️ **il conto lo dà il comando** e non questa cella, gotcha **#31**: `grep -c '^#\[test\]' crates/kernel/tests/ipc_wire.rs`) | lo **schema del canale `ipc`** (§6.1, ADR-0037) — `crate::wire::ipc`, la busta di `crate::framing` che porta **un enum a due varianti**, una per direzione. Tiene i **giri completi delle due direzioni**, e le **due varianti** ci sono perché con un tipo solo il **discriminante** non sarebbe provato; la **coda fuori** dalla busta e la **coda dentro** la lunghezza dichiarata, che sono guasti distinti con prenditori distinti; e il **corpo vuoto** e il **corpo troncato** dentro una busta onesta, che sono i due soli ingressi che raggiungono il ramo d'errore di `IpcMessage::decode`. ⛔ **NON è una riga del catalogo**, e non ne muove nessuna: nessuna riga di §7.4 nomina la §6.1 — la sezione *«Lo schema del canale `ipc`»* qui sopra porta il comando che lo misura, le mutazioni **G** e il loro esito |

#### Le finte delle porte — cosa hanno colto, e cosa hanno potato

⛔ **`ports_are_implementable.rs` ha smesso di confermare e ha colto un difetto — il
2026-08-09, sulla porta `process`.** La porta **come il piano la dettava non era
implementabile**: `instruct_one` deve **restituire** un `SingleReceipt` il cui unico campo è
`pub(crate)`, e la privacy di un campo di struct è **di modulo** — da fuori dalla crate quel
valore non si poteva costruire. È il gotcha **#46** nella forma peggiore: non «non riesco a
**leggere** un campo» ma «non riesco a **produrre** il valore di ritorno». Il rimedio — `new` e
`id` su entrambe le ricevute — porta la ragione accanto a sé in
`crates/kernel/src/ports/process.rs`. ⚠️ `Grant` resta **senza costruttore**, ed è l'opposto
deliberato: §5.6 la vuole inedificabile.

⚠️ **E `Grant` diverge dal piano, misurato.** Il campo con nome `reserved_mib: u64` costava un
`#[allow(dead_code)]` — qui un `allow` è un divieto spento (gotcha **#13**) — e anticipava un
pezzo del modello dell'arbitro, che è del Traguardo 5. Il campo **unitario privato** dà la
garanzia **identica** a costo zero: da fuori `Grant(())` è `E0423`, senza warning e senza
`allow`. Via anche `Debug`: nessuno formatta una concessione. ⚠️ Sulle **ricevute** `Debug` si
tiene, lo pretende `unwrap_err`. ⛔ E la misura ha una trappola che vale oltre il caso: **gli
errori di rustc si mascherano fra passate** — col costruttore assente usciva `E0599`, e sul
letterale **nessun errore**, perché l'`E0451` lo emette la passata di **privacy**, che non gira
se il type-check si è già fermato. Gotcha **#47**.

⛔ **`Clone` potato da `WorkerDescriptor` e da `Frame`, e la contro-sonda è ciò che rende la
potatura difendibile.** Tolti da questi due: **verde, zero warning**. Tolto da `Path` come
contro-sonda: **rosso**, `E0277` · `E0308` · `E0599`. Su `Path` e su `Endpoint` `Clone` è
**portante** — `declare_scope` consegna un **prestito** che l'implementazione deve trattenere —
mentre `WorkerDescriptor` e `Frame` attraversano la porta **per valore**. ⚠️ «Non implementabile
oggi» e «un chiamante lo vorrà domani» sono due forme diverse, e a distinguerle è **la finta**.

⛔ **La porta `ipc` ha usato la stessa finta per il servizio opposto — il 2026-08-10 — e la
differenza fra i due esiti è la parte da tenere.** Su `process` la finta ha **colto un
difetto**; su `ipc` ha compilato al primo colpo, e ciò che ha comprato è stata una
**sottrazione**. Scritta **prima** del sorgente, non ha usato nessuna delle quattro voci che il
piano dettava per `ClientId` — misurate **una per una**, togliendo e ricompilando:

| Voce del piano | Misura | Esito |
|---|---|---|
| `ClientId::get()` | tolto → `build` e `test --workspace` **verdi, zero warning** | ⛔ **cancellato** |
| `Hash` | idem | ⛔ **cancellato** |
| `PartialOrd`/`Ord` | idem | ⛔ **cancellato** |
| `PartialEq`/`Eq` | **contro-sonda** — tolto → **rosso**, `E0369` («`==` cannot be applied») | ✅ **tenuto** |
| `Copy` | **contro-sonda** — tolto → **rosso**, `E0382`: **ventitré**, su **otto** siti di dichiarazione | ✅ **tenuto** |
| `Debug` | **contro-sonda** — tolto → **rosso**, `E0277`, lo pretende ogni `assert_eq!` | ✅ **tenuto** |
| `Clone` | **non è una scelta**: lo pretende `Copy` — tolto da solo, `kernel` **non compila** (`E0277`) | ✅ **tenuto dal compilatore** |

⛔ **Le contro-sonde non sono cerimonia: sono ciò che regge l'argomento della prima riga.**
`get()` è cancellato *perché* un'implementazione **conserva** un identificativo `Copy` e lo
confronta con `==`, come `InMemoryFilesystem` fa con `CheckpointId` — ma se `PartialEq` non
fosse davvero esercitato quel «perché» poggerebbe sul nulla, che è esattamente il modo in cui
`SingleReceipt::id` era rimasto in vita senza copertura. ⚠️ L'argomento **a favore** di `Ord` era
reale — il **#12** vieta `HashMap` e spinge su `BTreeMap` — e cade su un criterio preciso:
`Ord` **non blocca** chi implementa da fuori (una tabella più `==` basta) e si aggiunge dopo in
una riga. L'eccezione del **#46** copre *«non implementabile oggi»*, non *«comodo domani»*.
`Hash` è il peggiore dei tre: **abilita la cosa vietata**.

#### Le passate di mutazione, e le righe che contano

⛔ **Ogni finta porta la propria passata. La prosa sotto le tabelle è solo per le mutazioni
uccise da _un test solo_:** sono le uniche che dicono qualcosa che la tabella non dice già.

**`process` — dodici mutazioni, dodici uccise.**

| | Mutazione | Chi la uccide |
|---|---|---|
| M1 | ricevuta ignota → `Ok(None)` invece di `UnsolicitedFrame` | `..._refuses_where_it_must` |
| M2 | `read_one` ignora la morte del worker | `..._refuses_where_it_must` |
| M3 | il flusso non si esaurisce mai | `..._can_be_implemented_and_called` · `..._correlated_...` |
| M4 | `instruct_one` accetta un frame vuoto | `..._refuses_where_it_must` |
| M5 | `close` ammette una ricevuta mai aperta | `..._refuses_where_it_must` |
| M6a | `read_one` → costante **7** | `..._can_be_implemented_and_called` · `..._correlated_...` |
| M6b | `read_one` → costante **1** (la scelta **avversaria**) | ⛔ **solo** `..._correlated_...` |
| M7 | `read_next` prende il flusso 0 invece di cercarlo | `..._correlated_...` · `..._refuses_where_it_must` |
| M8 | `close` rimuove il flusso 0 invece di cercarlo | `..._correlated_...` · `..._refuses_where_it_must` |
| M9 | **de-correlazione totale**, zero `receipt.id()` nella finta | `..._correlated_...` · `..._refuses_where_it_must` |
| M10 | `WorkerDescriptor::new` perde un byte | `the_process_port_is_implementable` — ⚠️ **rinominata il 2026-08-21** (finding P-2): si chiamava `..._but_start_is_not_callable`, e il nome affermava il falso |
| M11 | `kill()` **acquista** una guardia di liveness | `killing_a_worker_consumes_it` |

⛔ **M6b è la riga che conta, e si vede solo provando _due_ valori.** Il registro aveva
dichiarato che «`read_one` su costante» uccideva: vero per la costante **7**, **falso** per la
costante **1**, che coincideva con l'id atteso — **verde su 8 test su 8**, correlazione persa e
nessuno se ne accorgeva. ⛔ E il fondo era più basso: con tre mutazioni combinate la finta
conteneva **zero** occorrenze di `receipt.id()` e la suite restava verde. La causa non era la
costante ma **la forma della suite**, che non teneva mai due ricevute aperte insieme. Rimedio:
`answers_are_correlated_to_the_receipt_that_asked`, con lunghezze **diverse** e chiudendo il
flusso in posizione **1** — a budget uguale, o chiudendo la posizione 0, la mutazione
sopravviveva. Gotcha **#15**.

⛔ **M11 è nata sopravvivendo.** Il file spendeva quattro righe a dichiarare che `kill` la
guardia di liveness **non ce l'ha, di proposito** (§5.3 punto 4) — e **niente lo teneva**:
aggiungendola, i nove test restavano verdi. Una riga — uccidere un worker **già morto** — la
rende rossa. ⚠️ La mutazione è nata da una **rifinitura di stile**: estratto l'aiutante
`alive()`, l'unico punto che non lo chiama è diventato **visibile**, e visibile ha fatto
chiedere se fosse provato. Un principio che non si può controllare è un'intenzione.

**`ipc` — quattordici mutazioni, quattordici uccise.**

| | Mutazione | Chi la uccide |
|---|---|---|
| M1 | `accept` riusa l'identificativo | `..._delivered_to_the_client_they_name` · `a_dead_client_...` |
| M2 | `live()` ignora la morte del client | `..._refuses_where_it_must` · `a_dead_client_...` |
| M3 | `live()` smette di guardare le identità (prende il client 0) | i tre sopra |
| M4 | `receive` risponde `Ok(None)` a un client ignoto | `..._refuses_where_it_must` |
| M5 | `receive` risponde `Err` invece di `Ok(None)` a vuoto | tre test |
| M6 | `send` consegna sempre al client 0 | `..._delivered_to_the_client_they_name` |
| M7 | `accept` consegna un client che nessuno ha chiesto | `..._can_be_implemented_and_called` |
| M8a | `accept` conia la costante **7** | `..._delivered_...` · `a_dead_client_...` |
| M8b | `accept` conia la costante **1** (la scelta **avversaria**: è l'id reale del primo) | idem |
| M9 | `receive` non consuma il messaggio | `..._can_be_implemented_...` · `..._refuses_...` |
| M10 | il controllo sul messaggio malformato sparisce | `..._refuses_where_it_must` |
| M11 | morire non uccide | `..._refuses_...` · `a_dead_client_...` |
| M12 | ⛔ la morte è **contagiosa**: ne muore uno, muoiono tutti | ⛔ **solo** `a_dead_client_...` |
| M13 | ⛔ `accept` **ricicla** l'identificativo di un client morto | ⛔ **solo** `a_dead_client_...` |

⛔ **M12 e M13 esistono perché la prima passata ha colto il gotcha #45 dentro il lavoro di
oggi.** Con le prime dodici, `a_dead_client_does_not_take_the_port_with_it` non era **l'unico
uccisore di nessuna**: cancellandolo, tutte e dodici morivano lo stesso — cioè il test che porta
la proprietà per cui la porta esiste, **la gui è sacrificabile**, era cancellabile lasciando la
porta **verde**. Le due mutazioni che lo isolano sono quelle che nessun altro test poteva
vedere: che la morte di un client **non contagi** gli altri, e che una gui che si riconnette sia
un **client nuovo** e non erediti l'identificativo del cadavere — altrimenti un messaggio
accodato per il morto verrebbe consegnato al nuovo arrivato, che per I1 nasce **senza stato
proprio** e non potrebbe accorgersene.

⛔ **E una questione resta dichiarata e aperta nel sorgente di `ipc`, sul modello di
`network`.** `accept` non può fallire, e per i due modi di fallire che il vocabolario conosce è
**coerente**: `Disconnected` è un'affermazione **su un `ClientId`**, e `accept` è l'unico metodo
che un `ClientId` **non lo prende**; e non decodifica niente, quindi nemmeno `MalformedMessage`
lo raggiunge. ⚠️ Ma un **ascoltatore** rotto — non un client — oggi **non ha parola** in questo
vocabolario, e arriverebbe come `None`, cioè un **valore sbagliato** invece di un errore
(gotcha #30). ⛔ **Ed è _anche_ un'asimmetria fra le firme:** `receive` restituisce
`Result<Option<Vec<u8>>, IpcError>`, dove «niente di pronto» e «rotto» sono già distinti;
l'argomento contro il `Result` confuta solo `Result<ClientId, IpcError>`, non la forma con
l'`Option` dentro, che il metodo lì sotto **già usa**.

| | |
|---|---|
| **il prezzo vero** | ⛔ aggiungere una terza variante domani **non chiuderebbe niente**: non c'è dove restituirla. Chiudere il residuo costa **la firma**, non l'enum |
| **perché la firma resta** | oggi `IpcError` non ha **nessuna** variante che `accept` possa restituire: un `Result` che non può mai essere `Err` è **superficie morta**, esattamente ciò che questa porta ha appena potato in tre derive e un accessore |

**`memory_journal` — diciotto passate: una di controllo, sedici uccise, una viva e dichiarata.**
⚠️ **Il ritratto è del 2026-08-09 e va letto con la data**: la decisione sul secondo intento ne ha
spostate tre — **M7a**, **M13** e le due nuove **M14** e **M15** — e la tabella che le riconta è
in fondo alla voce chiusa, non qui. Le altre quattordici reggono parola per parola.
⛔ **M0 va letta per prima:** cambia **solo un commento**, e nessun test diventa rosso. Senza di
lei la tabella non prova niente (gotcha #48).

| | Mutazione | Chi la uccide |
|---|---|---|
| M0 | *controllo* — cambia solo un commento | ⛔ **nessuno, ed è il punto** |
| M1 | `intent` risponde `Ok(())` senza scrivere | sei test |
| M2a · M2b | lettura mancante → `Ok(vuoto)` · → `NotDurable` | `a_step_never_written...` · `..._does_not_survive_being_dropped` |
| M3 | `outcome` salta la guardia | `..._without_an_intent_is_refused` · `..._belongs_to_another_step` |
| M4 | `has_intent` ignora **quale** passo | ⛔ **solo** `..._belongs_to_another_step` |
| M5 | `has_intent` ignora il **tipo** di voce | ⛔ **nessuno, e non è una lacuna** — sotto |
| M6a · M6b | `read_back` dà l'**ultimo** (`.rev()` · `.last()`) | `..._the_intent_and_not_the_outcome` · `each_step...` · `a_second_intent...` |
| M7a | gli **intenti** scritti in testa | ⛔ **solo** `a_second_intent...` — ⚠️ **rimisurata il 2026-08-10**, chiudendo la voce del secondo intento: oggi la uccide la **conformità**, promessa 4 |
| M7b | gli **esiti** scritti in testa | `..._the_intent_and_not_the_outcome` · `each_step...` |
| M8 | `read_back` ignora l'argomento `step` | ⛔ **solo** `each_step...` |
| M9a · M9b · M10 | `prune` risponde `Ok` · risponde `NotDurable` · **pota lo stesso** | `prune_refuses_and_leaves_the_record_where_it_was` — ⚠️ **rimisurate il 2026-08-10 col Task 11: ora ne uccidono TRE ciascuna**, perché la conformità pretende la parola esatta (`StepInDoubt`) e non più un `is_err()` |
| M11 | stato **globale di processo** (`static AtomicBool`) | `..._does_not_survive_being_dropped` |
| M12 | `outcome` rifiuta **sempre** | `..._after_its_intent_is_accepted` · `..._the_intent_and_not_the_outcome` · `each_step...` |
| M13 | `intent` **sovrascrive** l'intento già presente | ⛔ **solo** `a_second_intent...` — ⚠️ **irraggiungibile dal 2026-08-10**: la guardia risponde prima. La forma viva è **M14**, sotto |

⛔ **M5 non è uccisa da nessuno, e non va chiusa: distingue uno stato irraggiungibile.** Il
primo record di un passo può essere **solo** un intento, perché `outcome` esige `has_intent`;
quindi «esiste una voce per questo passo» ed «esiste un **intento** per questo passo» sono la
stessa affermazione.

⛔ **E LA PREVISIONE SCRITTA QUI ERA SBAGLIATA, misurata il 2026-08-10.** Diceva:
*«l'equivalenza cade il giorno in cui `prune` rimuoverà voci in modo selettivo — il compito 11
di questo traguardo»*. Il compito 11 è stato eseguito, `prune` rimuove voci in modo selettivo, e
**M5 sopravvive ancora all'intero workspace**. La ragione è precisa e la previsione non l'aveva
vista: `prune` non chiede `has_intent`, chiede **se esiste un ESITO** — una domanda che
`has_intent` non pone e non porrà. L'equivalenza cadrà con `has_outcome`, che nessuno ha ancora
scritto. ⚠️ La frase è **sostituita dalla misura** invece che cancellata: era plausibile, e chi
legge il prossimo `prune` la penserà di nuovo. Gotcha **#15** rivolto a chi scriveva.

✅ **LA QUESTIONE DEL SECONDO INTENTO È CHIUSA IL 2026-08-10, e la voce resta qui invece di
essere cancellata** — una voce aperta che sparisce non si distingue da una dimenticata.

**Com'era.** *Se un secondo intento sullo stesso passo debba essere accettato.* `intent` **non
aveva guardia**: lo accettava in silenzio, e `read_back` rispondeva col **primo** dei due. Non
era una decisione presa, era un comportamento mai interrogato. ⚠️ La misura che l'ha scoperta:
con gli intenti scritti in testa (**M7a**) tutti e nove i test di allora restavano **verdi**, e
il testimone era di **tre chiamate senza nessun esito** — `intent(1,"p0"); intent(1,"p1");
read_back(1)`, che rispondeva `"p0"` e col rovesciamento `"p1"`.

⛔ **La decisione: un secondo intento è RIFIUTATO.** Presa dal **coordinatore in revisione**,
non dal piano, e dichiarata come tale nell'errata perché il proprietario possa **ribaltarla
vedendola**. Le ragioni, in ordine:

| | |
|---|---|
| **il modello** | ADR-0007 dice *«l'intento di **ogni** passo»* — uno per passo. Un secondo è **fuori dal modello**, non un caso da disciplinare |
| **la simmetria** | è la metà mancante di *«un esito senza intento è rifiutato»*: V6 tenuta dalla **porta** invece che dalla diligenza del chiamante. La porta lo faceva già in una direzione sola |
| **YAGNI non si applica** | su una porta dichiarata in anticipo i chiamanti sono vuoti **per costruzione**, quindi il criterio non distingue il morto dal portante — gotcha **#46** |
| **il costo** | una riga adesso — `has_intent` c'era già — contro due implementazioni e un archivio dopo |
| **mai scartata prima** | ⛔ cercato dove fosse già stata valutata (gotcha #32): questa stessa voce diceva *«non è una decisione presa»*. Mai valutata, mai scartata |

⛔ **E perché la sede è la conformità e non `journal.rs`.** La promessa **2** costringe già la
seconda implementazione a conservare **più record per passo**, quindi a chiavare più fine
dell'identità del passo — e con una chiave così *«il primo intento vince»* cade da sé. ⚠️ **Ma
è un accordo per accidente del disegno della chiave, non per contratto:** chi chiavasse **sul
passo**, che è la scelta naturale, divergerebbe dalla finta **senza che nulla diventi rosso**.
È il caso per cui la suite esiste.

⚠️ **Nessuna variante d'errore nuova: `OutOfOrder` si allarga.** La porta dichiara il proprio
tipo d'errore *«deliberatamente povero: un tipo ricco invita il kernel a diramare sulla
ragione»*, e *«un'operazione è arrivata fuori ordine per questo passo»* copre entrambe le
direzioni. ⚠️ **RICHIAMO DEL 2026-08-27 — le vie sono TRE, e questa riga diceva che «il doc
della variante lo dice»: il doc dice TRE.** La terza è *«una `note` su un passo che non ha un
intento»*, nata **lo stesso 2026-08-10** e poche ore dopo. L'elenco vivo sta accanto alla
variante in `crates/kernel/src/ports/journal.rs`; la §4.1 della spec porta il richiamo gemello.
⛔ Era la **seconda casa viva** della stessa frase, e nessun finding dell'audit la nominava —
radice **R1**, trovata dalla revisione della passata che chiudeva la prima. Finding **AUD-015**.

⚠️ **E la decisione ha spostato due mutazioni, misurate di nuovo invece che dedotte** — un
uccisore che sparisce in silenzio è il pericolo vero:

| Mutazione | Chi la uccideva | Chi la uccide **ora** |
|---|---|---|
| **M7a** — gli intenti scritti in testa | ⛔ **solo** `a_second_intent...`, che con la guardia **non può più esistere** in quella forma | ✅ la **conformità**, promessa 4 — l'ordine di scrittura di `replay` **fra** i passi, che allora non esisteva. Tre test. **La mutazione ha cambiato padrone, non è sopravvissuta** |
| **M13** — `intent` sovrascrive l'intento già presente | ⛔ solo `a_second_intent...` | **irraggiungibile**: la guardia risponde `OutOfOrder` prima. La forma viva è **M14**, la guardia **tolta**, uccisa dalla conformità (promessa 6) e da `a_second_intent_on_the_same_step_is_refused` |
| **M15** — la guardia scritta `!entries.is_empty()` invece di `has_intent(step)` | *nuova* | **sei test**: `each_step_reads_back_its_own_first_record` e `an_intent_for_another_step_is_still_accepted` qui, più **cinque** in conformità, che muoiono sul **setup** della promessa 4. ⚠️ **La prima stesura di questa riga diceva «solo la contro-sonda nuova», ed era falsa**: scritta prima della misura, corretta dopo |

**`journal_contract` — nove promesse in dieci blocchi, DODICI bugiardi, e la corrispondenza è
stata _misurata_ su otto.** ⚠️ **La cifra dei bugiardi è passata a dodici il 2026-08-17**, coi tre
dell'audit — **J14**, **J15**, **J16** — e le promesse **non sono cambiate**: quei tre non
aggiungono niente al contratto, provano tre promesse che c'erano già in uno **stato** che la suite
non costruiva mai. I blocchi restano **dieci** perché le asserzioni nuove stanno **dentro** blocchi
esistenti; `grep -c '= build();'` risponde dieci.
⚠️ **Questa riga diceva «otto promesse, otto bugiardi», ricontate sul
sorgente il 2026-08-10 chiudendo il traguardo:** i messaggi di promessa sono **nove** —
`READ_BACK` · `READ_BACK_IS_THE_INTENT` · `MISSING` · `REPLAY_ORDER` · `OUT_OF_ORDER` ·
`SECOND_INTENT` · `PRUNE_IN_DOUBT` · `PRUNE_RECONCILED` · `NOTE` — e i bugiardi altrettanti.
Gotcha **#31**. ⛔ **E la cifra della campagna NON è stata alzata per simmetria, perché sarebbe
stata un'ipotesi:** la tabella qui sotto è la passata presa **dopo la promessa 8**, quando i
bugiardi erano otto, e la nona — la **7b** — è stata provata non-vacua **in un altro modo**, con
la mutazione `M14b` della campagna di `prune`, che toglie il **blocco intero** invece di
neutralizzare una promessa. Le due misure dicono la stessa cosa con due strumenti, e chi rifà la
passata a nove otterrà **otto verdi** per riga dove qui ne stanno sette.
⛔ Neutralizzando **una** promessa alla volta — avvolgendone i blocchi in `if false` — cade
**esattamente** il test del suo bugiardo e nessun altro, **otto volte su otto**. È la prova che
nessuna promessa è decorativa e che nessun bugiardo muore sulla promessa di un altro.
⚠️ **Rimisurata per intero il 2026-08-10 due volte:** la prima dopo l'arrivo della promessa 6, la
seconda dopo la promessa 8. Un banco che non si rifà quando l'insieme cambia è un banco che
riporta l'esito di ieri. ⚠️ **E la promessa 8 ha DUE blocchi**, non uno — il passo mai aperto sta
in un giornale suo — quindi neutralizzarla ne spegne due; scritto qui perché chi rifà la misura
con uno strumento che cerca un blocco solo otterrebbe un falso «sopravvissuta».

⚠️ **La tabella qui sotto misura la suite contro la FINTA, e dal Task 9 c'è la metà gemella
sull'implementazione VERA:** sta nella sonda **J12**, si rompe `FileJournal` invece di
neutralizzare un blocco, e vale su **tre** promesse diverse più una mutazione di controllo. Le
due misure non si sostituiscono — questa dice che nessuna promessa è decorativa, quella dice che
la suite sta davvero girando contro `redb` e non contro una descrizione di `redb`.

| Promessa neutralizzata | Chi cade | E nessun altro |
|---|---|---|
| 1 · `read_back` rende ciò che `intent` ha scritto | `SilentJournal` (**J2**) | 7 verdi |
| 2 · dopo l'esito, `read_back` rende ancora l'**intento** | `LastWriteWinsJournal` (**J3**) | 7 verdi |
| 3 · un passo mai scritto è `Missing`, non vuoto | `EmptyInsteadOfMissingJournal` (**J4**) | 7 verdi |
| 4 · `replay` in ordine di **scrittura** | `ShuffledJournal` (**J5**) | 7 verdi |
| 5 · un esito **senza** intento è rifiutato | `PermissiveJournal` (**J6**) | 7 verdi |
| 6 · un **secondo intento** sullo stesso passo è rifiutato | `UnguardedIntentJournal` (**J7**) | 7 verdi |
| 7 · un passo **in dubbio** non è potabile | `EagerPruner` (**J8**) | 7 verdi |
| 8 · una **nota** su un passo aperto è conservata e non spodesta l'intento | `DiscardedNoteJournal` (**J10**) | 7 verdi |
| ⚠️ **7b** · un passo **riconciliato** è potabile — *fuori da questa passata, e si dice invece di lasciarla sembrare assente* | `AlwaysInDoubtJournal` (**J13**) | ⛔ misurata dalla mutazione **`M14b`** della campagna di `prune`, che toglie il **blocco intero**: il bugiardo risponde *«THE SUITE IS VACUOUS ON promise 7b»* in **entrambi** i binari. La forma è diversa **per necessità** — gotcha **#55**: le tre asserzioni del blocco portano un messaggio solo, quindi neutralizzarne una lascia il rosso in piedi e non prova niente |

### T-1 e T-2 — le tre promesse provate solo dove ogni guardia passa (2026-08-17)

⛔ **La domanda che le ha colte, ed è quella del gotcha #63:** *in quale altro stato del mondo
questa asserzione resterebbe verde?* Tre promesse su nove rispondevano **«in tutti quelli in cui
l'archivio ha un passo solo»**, che è lo stato che **otto blocchi su dieci** costruivano.

⛔ **Il rimedio non aggiunge promesse e non tocca il codice di prodotto.** Le due implementazioni
filtravano già per passo — `has_intent(step)` nella finta, `stored == step.get()` nella vera — e la
porta lo dichiarava già (*«re-reads ONE step BY NAME»*, e le tre vie di `OutOfOrder`). A mancare
era un **passante**: un passo in archivio **diverso** da quello sotto esame, che è l'unico stato in
cui la guardia giusta e quella cieca danno risposte diverse. ⚠️ **Quindi non è un'aggiunta al
contratto di una porta condivisa** — che è come l'audit lo aveva prezzato — ma lo stesso contratto,
provato.

⛔ **Prima il rosso, e per il motivo giusto.** I tre bugiardi scritti **prima** di toccare i
blocchi hanno risposto tutti e tre `THE SUITE IS VACUOUS ON promise …` — 1, 5 e 8 — cioè la
riproduzione in casa di T-1 e T-2, più una terza che l'audit non aveva separato.

⛔ **E la seconda direzione è misurata sulle IMPLEMENTAZIONI VERE, non sui soli bugiardi**, perché
un bugiardo prova che il blocco morde e non che il blocco raggiunga `redb`. Sei mutazioni, **una
alla volta** — due insieme e il rosso non si attribuisce — ciascuna compilata ed eseguita a sé, poi
**revocata**; a fine campagna `git diff --stat` nomina **il solo file della suite**, cioè le due
implementazioni sono tornate identiche a `HEAD` byte per byte.

| # | Mutazione | Dove | Esito |
|---|---|---|---|
| **B-1** | guardia di `outcome` → `find_first(\|_, _, _\| Some(())).is_none()` | `platform::journal::FileJournal` | 🔴 promessa **5** |
| **B-2** | guardia di `note` → idem | `FileJournal` | 🔴 promessa **8** |
| **B-3** | predicato `stored == step.get()` tolto da `read_back` | `FileJournal` | 🔴 promessa **1** |
| **B-4** | predicato `e.step == step` tolto da `read_back` | `simulator::journal::MemoryJournal` | 🔴 promessa **1** |
| **B-5** | guardia di `outcome` → `self.entries.is_empty()` | `MemoryJournal` | 🔴 promessa **5** |
| **B-6** | guardia di `note` → idem | `MemoryJournal` | 🔴 promessa **8** |

⚠️ **Sei su sei, e ciascuna col messaggio della PROPRIA promessa** — non «è diventato rosso», ma
*«è diventato rosso lì»*: è la differenza che `assert_caught_on` compra leggendo il payload invece
di accontentarsi di `is_err()`. Prima del rimedio, **tutte e sei lasciavano il workspace verde**.

⛔ **E la mutazione di controllo è la baseline stessa:** col codice vero, `bash scripts/gate.sh`
dà `GATE GREEN` e `cargo test --workspace --no-fail-fast` **32 target, 177 passati, 0 falliti, 2
ignorati** — cioè le asserzioni nuove **non scattano dove non devono**, che è la metà che si
dimentica (gotcha **#24**).

⛔ **Due difetti reali sono stati colti dalla misura, non dalla rilettura** — entrambi nella
suite come il piano la dettava:

| | Il difetto | Come si è visto |
|---|---|---|
| **a** | la promessa **4** confrontava le **sole identità** dei passi, e la sequenza dettata `1, 2, 1` **è un palindromo**: un `replay` rovesciato rende le stesse tre identità nelle stesse tre posizioni | `ShuffledJournal` **passava la suite intera** — «la suite è vacua sulla promessa 4». Chiuso confrontando i **record**, byte compresi: `first, second, third` rovesciato è `third, second, first` |
| **b** | la promessa **1** rileggeva con un `.expect("read_back must find it")`, e la via **A6** è proprio il caso in cui `read_back` **non trova**: la suite scattava con un messaggio che **non nomina nessuna promessa** | `a_journal_that_writes_nothing_is_caught` riportava «ha sparato, ma NON sulla promessa 1» — la suite coglieva A6 e non sapeva dirlo. Chiuso mettendo il messaggio della promessa anche sull'`expect` |

✅ **LA QUESTIONE DELLA PROMESSA 7 È CHIUSA IL 2026-08-10 DAL TASK 11, e la voce resta qui invece
di essere cancellata** — una voce aperta che sparisce non si distingue da una dimenticata.
`prune` ha imparato a rifiutare **un passo in dubbio** invece di rifiutare tutto, la promessa
**7b** è la metà che discrimina, e `AlwaysInDoubtJournal` (**J13**) è il bugiardo che prova che
morde. ⛔ **E lo Step 1 del compito attendeva un ROSSO che non c'era:** la partenza era **verde**
in entrambe, esattamente per la ragione che questo capoverso descriveva. ⛔ **Al suo posto due
voci nuove restano aperte, misurate e non supposte** — la terza risposta di `prune` e la
distinzione di ADR-0018. ⚠️ **Il «e stanno SOTTO» è TOLTO il 2026-08-27, non riallineato:** le
voci aperte accanto a `prune` sono ora **tre**, e un puntatore che le conta invecchia a ogni voce
nuova mentre i due nomi non invecchiano.

**Com'era.** ⚠️ **Questo capoverso diceva «la
promessa 6» e il numero era stantio** — `prune` è la **settima** da quando il secondo intento si è
preso la sesta; corretto il 2026-08-10 ricontando sul sorgente, gotcha **#31**. L'asserzione chiede che `prune`
**rifiuti**, e `MemoryJournal` rifiuta **tutto** (decisione D7) — quindi la supera senza mai
consultare se il passo sia in dubbio. Un giornale che rifiutasse ogni potatura a caso è qui
**indistinguibile** da uno che rifiuta *questa* perché è in dubbio: è la famiglia del gotcha
**#30**. ⛔ **Non è stata forzata**, e la ragione è precisa: la metà che discrimina è un passo
**non** in dubbio la cui potatura dev'essere **accettata**, e non si può scrivere finché `prune`
non è implementata da nessuna delle due parti. Arriva col **Task 11** di questo traguardo, dove
`prune` impara a rifiutare *un passo in dubbio* invece di rifiutare tutto. Fino ad allora **J8**
prova che la promessa sa scattare, e nient'altro prova che sappia distinguere. ⚠️ **Anche questo
identificatore era sbagliato e diceva J7**, che è il bugiardo del secondo intento: il potatore
avido è **J8**.

⛔ **VOCE APERTA 1 — la regola di ADR-0018 che ENTRAMBE le implementazioni violano, e che il Task
11 dichiara invece di chiudere** (gotcha **#36**: una nota si legge e si dimentica). ADR-0018
pretende che *«un payload assente e uno mai registrato non siano indistinguibili»*. **Misurato il
2026-08-10, non argomentato:** dopo la potatura, un passo potato e un passo **mai scritto**
rispondono **entrambi `Err(Missing)`** a `read_back`, sono **entrambi assenti** da `replay`, e una
**seconda** `prune` risponde `Err(Missing)` a tutti e due. Indistinguibili in **tre** modi, su
tutte e due. ⛔ **Non è stata chiusa, e la ragione è la decisione D7:** la distinzione piena vuole
l'**impronta** e la **dimensione** che ADR-0018 chiede a un record potato, l'impronta vuole una
funzione di hash, e nel kernel quella è una **voce nuova nella lista di ADR-0031** — un atto
deliberato che nessuna misura ha preparato. ⚠️ **E la via che sembrava non costarla è stata
cercata e MISURATA:** lasciare la voce e svuotare il payload **funziona** — `Ok([])` contro
`Err(Missing)`, distinguibili, conformità verde — ma `steps_in_doubt` risponde allora
**`SuspendAndAsk`** su un passo riconciliato e potato, perché byte vuoti sono **indecifrabili** e
un record indecifrabile rimette il passo in dubbio. Il sistema si fermerebbe su **ogni** passo
potato, **a ogni ripresa**. Una traccia che serva dev'essere leggibile dalla riconciliazione,
cioè una decisione di **formato** — e i byte congelati del Task 10 la rendono un atto deliberato.
**Chi la chiude:** il traguardo che porta la ritenzione, **insieme** alla decisione sull'impronta.
Il limite è scritto anche accanto al codice, in tutte e due le implementazioni e nel blocco 7b.

⚠️ **VOCE APERTA 2 — la terza risposta di `prune` non è tenuta da nessuna promessa.** Le risposte
sono **tre**: `Missing` per un passo mai scritto, `StepInDoubt` per uno aperto, `Ok` per uno
riconciliato. Le promesse **7** e **7b** tengono le ultime due **attraverso entrambe** le
implementazioni; la prima è tenuta **solo** per il doppio in memoria, in
`crates/simulator/tests/memory_journal.rs`. **Misurato:** togliere la guardia `Missing` a
`FileJournal` (mutazione `M10` del Task 11) lascia l'**intero workspace verde**, quindi le due
potrebbero divergere in silenzio. ⛔ **Non è un buco aperto dal Task 11** — prima di lui entrambe
rifiutavano **ogni** potatura con `Missing` — e chiuderlo costa una promessa col proprio bugiardo,
che nessuna misura chiede oggi. **Chi la chiude:** il primo consumatore di `prune`, cioè la
spazzata di ritenzione.

⛔ **VOCE APERTA 3 — LE DUE NOZIONI DI «IN DUBBIO» DIVERGONO, E LA DIVERGENZA CADE DAL LATO CHE
AUTORIZZA LA DISTRUZIONE.** Nata il **2026-08-27** chiudendo il finding **AUD-006** del secondo
audit completo. ADR-0018 pone come regola **non negoziabile** che *«un passo in dubbio non è mai
potabile finché non è riconciliato»*, e nomina **esplicitamente la §4**, cioè la riconciliazione
del kernel. Ma le due parti chiedono due domande diverse:

| Chi | La domanda che fa | Con che cosa |
|---|---|---|
| la **porta**, dentro `prune` | *quale operazione è stata chiamata?* — un `intent` senza `outcome` | `EntryKind::Outcome` in `crates/simulator/src/journal.rs`, `kind == KIND_OUTCOME` in `crates/platform/src/journal.rs` |
| il **kernel**, in `steps_in_doubt` | *cosa dicono i record?* — decodificandoli | `crates/kernel/src/reconcile.rs`, il ramo `Err(_)` che **entra** nel dubbio con `SuspendAndAsk` |

⛔ **Un passo il cui record di ESITO è indecifrabile è quindi in dubbio per il kernel e POTABILE
per la porta.** ✅ **Misurato il 2026-08-27 da FUORI la crate**, su una crate usa-e-getta nello
scratchpad, cancellata nella stessa corsa — non dedotto e non ripreso dal rapporto (gotcha
**#65**). Scenario: `intent` con un `RecordV1` leggibile, poi `outcome(step, &[0xff, 0xfe, 0xfd])`.

```
[MemoryJournal] steps_in_doubt -> [InDoubt { step: StepId(1), resolution: SuspendAndAsk }]
[MemoryJournal] prune          -> Ok(())
[MemoryJournal] after prune    -> []
[FileJournal]   steps_in_doubt -> [InDoubt { step: StepId(1), resolution: SuspendAndAsk }]
[FileJournal]   prune          -> Ok(())
[FileJournal]   after prune    -> []
```

⛔ **Perché conta più di una sfumatura, e lo scenario non è ipotetico.** È la **sola** operazione
irreversibile del giornale, e viene concessa esattamente sul passo su cui ADR-0007 vuole che il
sistema **si fermi invece di indovinare**. [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md)
rende la lettura fra versioni il caso **ordinario**, e `crates/kernel/src/record.rs` dichiara che
un record che porta una variante che la build non conosce si decodifica a `RecordError::Malformed`
— *«an older build STOPS rather than guesses»*. È **quella** frase che la misura smentisce: la
build vecchia si ferma in `steps_in_doubt` **e pota lo stesso passo** con `prune`.

⛔ **NON È CHIUDIBILE SULLA PORTA, ed è la ragione per cui è una voce aperta e non un difetto da
correggere.** La porta scambia **byte** e non può decodificare (ADR-0036), quindi non può porre la
seconda domanda: l'approssimazione non è una scorciatoia, è l'unica domanda che quel livello sa
fare. ⚠️ **Era già dichiarata accanto al codice — ma come GIUSTIFICAZIONE e non come limite:**
*«"IN DOUBT" HERE IS THE PORT'S NOTION AND NOT §4.3's ... the two must not be confused»* spiega
**che** le due sono diverse, e nessun documento registrava **in quale verso** possano divergere. È
il gotcha **#64** — *due criteri coprono ciascuno la propria metà e lasciano scoperto il buco fra
loro* — con le **definizioni** al posto dei criteri, e la domanda nuova che ne esce: ⛔ ***due
strati che usano la stessa parola con due definizioni: in quale VERSO divergono, e uno dei due
versi autorizza un'azione irreversibile?***

⛔ **L'OBBLIGO È DEL CHIAMANTE, ed è scritto ORA perché il chiamante non esiste ancora.**
Misurato: `prune` non ha **nessun** chiamante di produzione — ogni chiamata nel workspace è un
banco. La **spazzata di ritenzione** che ne farà la prima vive nel kernel e **può** decodificare,
quindi dovrà consultare `crate::reconcile::steps_in_doubt` e **saltare** ciò che quello
restituisce, invece di poggiare sulla guardia della porta. La frase sta accanto a `Journal::prune`
in `crates/kernel/src/ports/journal.rs`, dove la legge chi implementa.

⚠️ **NON PINZATA DA UNA SONDA, e il costo è scritto invece che taciuto.** Una sonda che asserisse
l'`Ok(())` di oggi andrebbe **rossa il giorno in cui la spazzata chiude la cosa per bene**, cioè
per aver avuto ragione — ed è il gotcha **#73**: *una sonda che va cancellata per prendere una
decisione è un voto contro il prenderla*. La suite di conformità non può chiuderla per la stessa
ragione della porta, e lo **dichiara** nel blocco **7b** insieme alle altre due cose che non
pinza. **Chi la chiude:** il traguardo che porta la **ritenzione**, cioè lo stesso della VOCE
APERTA 1 e per un motivo imparentato — entrambe aspettano che qualcuno **chiami** `prune`.

**`reconciliation` — sedici mutazioni più una di controllo, e tre sonde isolate da una propria;
misurate il 2026-08-10, col Task 6.** ⛔ **La data mancava, ed era l'unica campagna del file a non
averla:** le cifre qui sotto — *«nove sonde su nove»*, *«`9 passed`»* — sono vere **di quel
giorno**, e il file ha ora **undici** sonde, le due che il **Task 7** vi ha aggiunto. La campagna
**non è stata rilanciata** su quelle due: si dichiara invece di allinearla, perché una cifra
rialzata per simmetria sarebbe un'ipotesi travestita da misura (gotcha **#15**).
⚠️ Applicazione verificata **per siti**, compilazione in un passo **separato** dall'esecuzione,
conteggio con `--no-fail-fast` (gotcha **#48**). **Sedici su sedici applicate**, nessuna
incompilabile; **nove sonde su nove** muoiono sotto almeno una mutazione, quindi nessuna è vacua.

| Mutazione | Chi cade |
|---|---|
| il ramo `Intent` tronca l'insieme a **1**, e a **0** | il crash a più passi, la classe, l'ordine — e con `0` anche il posto |
| il ramo `Outcome` non toglie nulla, e mette in dubbio invece di togliere | il passo con intento **ed** esito, il crash a più passi, l'ordine |
| `resolution_of` mappa tutto a `RunAgain`, a `SuspendAndAsk`, ad `AskTheWorld` | ⛔ **la prima ne uccide UNA, le altre due ne uccidono DUE**: gli altri test scrivono `Idempotent`, quindi la costante che coincide col valore atteso **nasconde metà del difetto**. È il contro-verso del #48 guadagnato di nuovo |
| il ramo `Err(_)` ignora, e risolve a `RunAgain` | il record indecifrabile, l'insieme, il posto, la gemella dopo l'esito |
| l'insieme **rovesciato** | l'ordine **e** il crash a più passi |
| ✅ l'insieme **ordinato per passo** | ⛔ **solo l'ordine**, e nient'altro: è la prova che la sonda dettata teneva l'ordine **per accidente**. ⚠️ Costa **due file**, perché `StepId` non deriva `Ord` di proposito |
| `enter` spinge sempre, e `enter` toglie-e-rispinge in coda | la prima uccide insieme e posto; ✅ **la seconda solo il posto**, ed è l'alternativa che un lettore sceglierebbe |
| `enter` tiene la **prima** risoluzione invece dell'ultima | insieme e posto |
| ✅ `MemoryJournal::replay` rifiuta un giornale **vuoto**, e ne perde la **prima** voce | ⛔ la prima uccide **solo** il giornale vuoto — ogni altra sonda scrive qualcosa prima, quindi nessuna incontra il primo avvio; la seconda ne uccide quattro |
| **controllo**: cambiata **una parola di un commento** | ✅ **nulla**, `9 passed` |

**`boundary` e l'arm `Note` — nove mutazioni più due di controllo, misurate il 2026-08-10.**
⚠️ Compilazione in un passo **separato** dall'esecuzione e conteggio con `--no-fail-fast` (gotcha
**#48**); ripristino **byte-identico** dei file, fine-riga compresi (vincolo globale 5).

| Mutazione | Chi cade |
|---|---|
| l'arm `Note` letto come un **intento** | **5**: le due sonde della nota, e tre della promozione |
| l'arm `Note` letto come un **esito** | **4** — ⚠️ `a_note_does_not_put_a_step_in_doubt` **non** cade qui, e la ragione è nel suo scenario: finisce con un esito, quindi un `leave` in più non cambia nulla. Delle due sonde della nota **una sola** vede entrambe le direzioni, ed è scritto invece che taciuto |
| `payload` e `reason` **scambiati** — il difetto che il piano dettava | **4**, fra cui quella dell'etichetta e quella del `Debug` |
| `trust: Instruction` invece di `Untrusted` | ⛔ **una sola**, ed è la prova che l'etichetta non poggia su nessun'altra asserzione |
| il record dice `Intent`, scritto comunque con `note()` | **4**, fra cui la sonda dell'accordo |
| il record dice `Outcome`, scritto comunque con `note()` | **4**, le stesse |
| ✅ `promote` scrive con **`outcome()`** invece di `note()` | ⛔ **prima: NESSUNA.** È l'opzione **F** scartata dal proprietario, e nulla la teneva. Dopo `OperationSpy`: **una sola**, la sua |
| `promote` scrive con **`intent()`** | **9** — il rifiuto della porta cade su tutto |
| ✅ `RecordKind::Note` spostata su un indice **libero** (2 → 7) | ⛔ **nulla**, ed era **atteso e dichiarato**: il derive rinumera codifica e decodifica insieme, quindi nessun andata-e-ritorno lo vede. È la stessa misura che il doc di `record_shape.rs` porta per gli altri campi. ✅ **Rimisurata il 2026-08-10 col Task 10: ora è ROSSA**, e la tengono i byte congelati — sonda **F3**, che ripete la stessa rinumerazione su **tutte e otto** le varianti |
| **controllo**: una parola di un commento in `reconcile.rs` | ✅ **nulla** |
| **controllo**: una parola di un commento in `boundary_promotion.rs` | ✅ **nulla** |

**`MemoryJournal::note` — quattro mutazioni.**

| Mutazione | Chi cade |
|---|---|
| la guardia `has_intent` **tolta** | **4**, fra cui la conformità |
| la guardia dimentica **quale** passo (`!entries.is_empty()`) | ⛔ **una sola**, `a_note_is_refused_when_the_intent_belongs_to_another_step` — è il gemello del buco misurato su `outcome`, e senza quella sonda la mutazione sopravvive |
| la nota **scartata** (il bugiardo, messo nel giornale vero) | **8** |
| ⛔ la nota archiviata come `EntryKind::Intent` | ⛔ **NULLA, e la sonda scritta per tenerla è stata TOLTA.** La variante interna è **inosservabile** da fuori: `note` rifiuta un passo senza intento, quindi un passo di sole note non è costruibile attraverso la porta e `has_intent` non può diventare vero per una nota sola. La sonda che lo affermava è stata cancellata e la misura registrata al suo posto — una sonda il cui commento rivendica un difetto che non vede insegna che il difetto è coperto, ed è peggio di un difetto scoperto che qualcuno conosce. Gotcha **#15** |

**`prune` su ENTRAMBE le implementazioni — quindici mutazioni più una di controllo, misurate il
2026-08-10 col Task 11.** Applicazione verificata dal banco stesso (rifiuta se il modello non è
unico nel file), **compilazione in un passo separato** dall'esecuzione, `--no-fail-fast`,
ripristino byte-identico.

| Mutazione | Chi cade |
|---|---|
| `M1` la guardia `Missing` tolta (simulatore) | **2** |
| `M2` *chiuso* = **una voce qualsiasi** (una nota conta) | **4**, conformità compresa |
| `M3` la guardia di ADR-0018 tolta · `M4` rifiuto giusto, **parola sbagliata** | **4** ciascuna |
| ⛔ `M5` risponde `Ok` e **non pota niente** · `M9` idem su `redb` | ⛔ **NESSUNO al primo giro** — la 7b guardava **solo il valore di ritorno**, famiglia del gotcha #30 e specie di **E42**. ✅ Chiusa con *«qualcosa è successo»*: ora **3** e **2** |
| ⛔ `M6` risponde `Ok` e pota **l'INTERO giornale** | ⛔ **NESSUNO al primo giro** — *«pota il passo 5»* era libero di distruggere ogni altro passo. ✅ Chiusa con un passo **spettatore**: ora **3** |
| `M7` qualsiasi voce conta come chiusa (`redb`) · `M8` parola sbagliata (`redb`) | **2** ciascuna |
| ⚠️ `M10` la guardia `Missing` tolta a `redb` | ⚠️ **NESSUNO, e resta dichiarata** — voce aperta 2 qui sopra |
| `M11` un **esito** archiviato come nota | **2** |
| ⛔ `M12` una **nota** archiviata come **esito** | ⛔ **NESSUNO al primo giro**: un passo con intento **e una nota** diventava potabile **mentre è in dubbio**, l'unica cosa che ADR-0018 vieta, e il difetto che il byte del `kind` esiste per impedire. ✅ Chiusa dando alla promessa 7 il caso **intento + nota**: ora **2** |
| ✅ `M13` **la contro-sonda della contro-sonda** — `prune` rifiuta **tutto**, con la parola giusta | **4**, e `the_in_memory_journal_honours_the_contract` muore col `PRUNE_RECONCILED_MESSAGE`. È ciò che rende la promessa 7 non-vacua |
| ⚠️ `M14` la sola asserzione (a) della 7b neutralizzata | ⚠️ **il bugiardo resta VERDE, e giustamente** — la (b) lo coglie con lo stesso messaggio. ⛔ **Ma allora non prova più la non-vacuità**, quindi rifatta come `M14b` |
| ✅ `M14b` il **blocco 7b intero** tolto | `a_journal_that_calls_every_step_in_doubt_is_caught` → **«THE SUITE IS VACUOUS ON promise 7b»**, in **entrambi** i binari |
| la **mutazione di controllo** — un commento in `MemoryJournal::prune` | ⛔ **nessuno, ed è il punto** |

⛔ **Quattro sopravvissute su quindici al primo giro, e tre erano difetti veri** — la lezione è
che una contro-sonda **nasce non provata** (gotcha #45, quarta volta in questo piano) e che
`M14` insegna una forma nuova: **una mutazione che deve far scattare qualcosa può fallire per aver
colpito troppo poco**, e il verde che ne esce si legge come una prova. È il gotcha **#54** girato.

⚠️ **E una sonda non muore MAI da sola, dichiarato invece che taciuto:**
`a_step_is_in_doubt_at_most_once_however_many_records_it_carries` cade solo insieme a
`a_step_that_re_enters_doubt_keeps_the_place_it_first_took`, la cui asserzione confronta il
**vettore intero** e vede quindi anche un doppione. Resta perché porta lo **scenario** — un passo,
due record, il secondo illeggibile — non perché veda un difetto che nessun'altra vede.

✅ **E LA QUESTIONE DELLE DUE VERITÀ È CHIUSA IL 2026-08-10 — DAL PROPRIETARIO — e la voce resta
qui invece di essere cancellata:** una voce aperta che sparisce non si distingue da una
dimenticata.

**Com'era.** *Il `kind` del record e l'operazione della porta sono due verità indipendenti, e
`replay()` ne restituisce una sola.* La riconciliazione si fida del **campo**; il giornale conosce
l'**operazione** — `MemoryJournal` tiene un `EntryKind` interno e `JournalError::OutOfOrder` è
definito sulle due operazioni. Misurate le due direzioni: `intent()` con un record che dice
`Outcome` fa **sparire in silenzio un dubbio vero**; `outcome()` con un record che dice `Intent`
rimette in dubbio un passo concluso.

⛔ **La decisione: `replay()` NON cambia e il `kind` RESTA nel record.** Presa dal **proprietario**,
non dal piano. Le ragioni, in ordine:

| | |
|---|---|
| **è semantica del kernel** | distinguere intento da esito è una decisione del kernel, e spostarla nella porta contraddice il doc di `replay` stesso — *«un'operazione come `steps_in_doubt()` sposterebbe una decisione del kernel dentro chi implementa la porta»* |
| **la forma durevole è del kernel** | ADR-0036, ed è la stessa regola per cui la porta scambia **byte** |
| **il costo evitato** | l'alternativa tocca **porta, conformità e due implementazioni**, e renderebbe **ridondante** il campo che `record.rs` chiama *«quello su cui poggia l'intero protocollo write-ahead»* |

⛔ **E il disaccordo si chiude da CHI SCRIVE, che oggi è una funzione sola.** `Untrusted::promote`
è l'unico codice del kernel che scrive un record, quindi non si costruisce un aiutante per un
chiamante solo: si mette una **sonda** che fissi l'accordo —
`the_promotion_writes_through_note_and_the_record_says_note` — e l'aiutante nasce col **secondo**
scrittore, quando avrà due siti per cui essere giusto.

⚠️ **RICHIAMO DEL 2026-09-01 — GLI SCRITTORI DI RECORD SONO QUATTRO, e il paragrafo qui sopra è
il verbale di quando erano uno.** Non si riscrive, si data: la sua scadenza in prosa — *«l'aiutante
nasce col secondo scrittore»* — era già scattata il 2026-08-20 **senza che niente diventasse
rosso**, ed è il gotcha **#77**. I quattro, e ciascuno con la **propria** sonda, che è la
decisione del proprietario del 2026-08-10:

| Scrittore | Dove | La sua sonda |
|---|---|---|
| `Untrusted::promote` | `crates/kernel/src/boundary.rs` | `the_promotion_writes_through_note_and_the_record_says_note` |
| `Arbiter::set_policy` | `crates/kernel/src/arbiter/mod.rs` | `a_policy_transition_writes_its_intent_before_its_outcome` |
| `run_the_ring` | `crates/kernel/src/sensor.rs` | le sonde di `crates/kernel/tests/sensor_ring.rs` |
| `gateway::dispatch` | `crates/kernel/src/gateway/mod.rs` | `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it` |

⛔ **LA VOCE RESTA APERTA E NON SI CHIUDE QUI:** se i quattro debbano condividere un aiutante è
del **proprietario** — cambia la forma di codice con quattro siti di chiamata — ed era già
**registrata e non presa**. ⚠️ **Ciò che il quarto scrittore aggiunge è UN FATTO e non un
argomento:** l'accordo fra `kind` e operazione di porta è ora tenuto da quattro sonde
indipendenti, e dal 2026-09-01 il `kind` **non è più un argomento** — `RecordV1` ha un
costruttore per specie (AUD-050), quindi «scrivere un `Outcome` con `kind: Intent`» non è più
esprimibile, mentre «chiamare `outcome()` con un record costruito da `intent()`» lo è ancora, ed
è quello che le quattro sonde coprono.

⚠️ **E la sonda è nata sbagliata, corretta dalla misura e non dalla rilettura.** La prima stesura
asseriva **solo** il `kind` e dichiarava la metà della porta *«tenuta per costruzione, perché
`note` è l'unica operazione che ammette un passo già aperto senza chiuderlo»*. **Falso, e
misurato:** `outcome` lo ammette anch'esso, quindi un `promote` riscritto per chiamare `outcome()`
— cioè l'**opzione F**, quella che il proprietario ha esaminato e **scartato** — lasciava
**l'intero workspace verde**. Chiusa con una spia conforme, `OperationSpy`, che delega tutto a
`MemoryJournal` e in più annota **quale metodo** è stato chiamato: ora quella mutazione uccide
**una sola** sonda, la sua.

⛔ **Ciò che NON è comprato, detto per intero:** nessuna regola di livello 1 impedisce a uno
scrittore futuro di chiamare `outcome()` con un record che dice `Intent`. La sonda copre
**l'unico scrittore che esiste**. È una voce **chiusa come decisione**, non come garanzia.

⚠️ **E una riga di questo registro va riletta quando quella questione si chiude, non prima:**
la voce di `V5` fra le scoperte dice *«il tipo `EffectClass` esiste ma nessun caso lo esercita»*.
Dal Task 6 la **riconciliazione** lo esercita su tutti e tre i valori, e tratta un record senza
classe leggibile come `Unrepeatable`, che è la frase di ADR-0007. ⛔ **Ma la riga di catalogo di
`V5` è di livello 1** — *«un effetto senza classe dichiarata non è esprimibile»* — e un test di
comportamento **non la copre**: sono due proprietà diverse con lo stesso nome. Scritto qui perché
chi riconta non muova il numeratore per la ragione sbagliata, e perché spostarlo richiederebbe il
**catalogo §7.4**, che è una modifica alla spec che nessuno ha deciso (vincolo globale 7).

⚠️ **Il banco di misura ha prodotto _nove_ esiti credibili e falsi in due sessioni** — gotcha
**#48**, col testo integrale in [`HANDOFF.md`](HANDOFF.md). ⛔ Gli ultimi due sono della revisione
di oggi e portano una forma nuova: **si erano corretti due strumenti gemelli, ma uno solo** — il
gemello non corretto ha riscritto di nuovo i fine-riga alla corsa successiva — e una
**rinomina** ha reso stantie due ancore di mutazione, che sono tornate «zero siti» invece che un
esito. 📌 Una cosa sola resta qui, perché
non è un inganno del banco ma un **fatto del repository**: i fine-riga sono **misti per file**
(`process.rs` e `ports_are_implementable.rs` in CRLF, `mod.rs` e `network.rs` in LF). Non c'è
una convenzione da seguire, c'è **un file da non cambiare** — uno strumento che ne riscrive uno
intero in LF produce un `git diff` di seicento righe che nessuno ha toccato.

### ⛔ VOCE APERTA CONSOLIDATA — le righe di catalogo delle sonde dell'audit (2026-08-18)

⛔ **L'esecuzione dell'audit ha prodotto DIECI sonde permanenti nuove, e NESSUNA ha una riga nel
catalogo §7.4.** La §7.4 è **spec**, e il **vincolo globale 7** la mette fuori dalla portata di
una passata di rimedio: si approva sezione per sezione. È lo stesso trattamento scelto per
**PL-1** il 2026-08-18 — *registrata come voce aperta e non come nota*, perché una nota si legge e
si dimentica (gotcha **#36**).

⚠️ **Raccolte qui in un posto solo invece che in quattro riquadri**, perché quattro voci aperte
sullo stesso oggetto sono il modo in cui una di esse smette di esserlo senza che nessuno l'abbia
chiusa — la ragione per cui la §6 del compendio ha una tabella e non quattro paragrafi.

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

⛔ **E una NONA voce che non è una riga di catalogo ma una modifica alla conformità:**
l'asserzione **4b** del `reactor` è **implicata** dalla 4a e non può scattare da sola. Toglierla o
ribasarla è una decisione sulla porta condivisa. Il verbale sta nella sezione B-2 qui sotto.

---

### K-1 e B-1 — la cella che accettava scritture da fuori un poll, e il limite mai letto (2026-08-18)

⛔ **La terza decisione della §8 dell'audit, e il rapporto la prezza sbagliata in TRE modi** — due
per difetto e uno per eccesso. È il gotcha **#65** a un'altra occorrenza, e stavolta ne insegna una
forma che il #65 non copre: il **#66**.

**Cosa dice il rapporto.** *«K-1 — drenare la cella all'ingresso di `run`, e riscrivere le due
sonde»*, con la nota che chiudendo K-1 *«due sonde permanenti diventano rosse»*.

| | Il rapporto | La misura del 2026-08-18 |
|---|---|---|
| **1** | drenare **all'ingresso di `run`** | ⛔ **non basta.** La via dei `Drop` è **dentro** la run: un distruttore gira dopo l'ultima lettura del ciclo, e col drenaggio all'ingresso il clock arrivava ancora a **9999** |
| **2** | *«due sonde diventano rosse»* | ⛔ **una** diventa rossa. L'altra resta **VERDE E DIVENTA VACUA**, che è il caso che non si vede |
| **3** | (implicito) il rimedio è un drenaggio in più | 📌 è un **cambio di invariante**, e costa **una riga** invece di due |

⛔ **IL RIMEDIO, e perché questa forma e non quella proposta.** L'invariante che serve
all'esecutore è *«ciò che leggo dopo aver girato X è stato scritto da X in quel poll»*. Drenare
**dopo** ogni poll difende dall'attività **precedente** — è V31, chiusa dal Traguardo 2 — e da
nient'altro: non dice nulla su una scrittura fatta **mentre nessun poll è in corso**, e i momenti
sono due, prima di `run` e fra un poll e il successivo. Svuotare la cella **subito prima** di ogni
poll sposta l'invariante da *«nessuno scriva mai fuori da un poll»*, che nulla può imporre, a
*«conta solo ciò che è scritto durante questo poll»*, che è imposto lì. **Tutte le vie chiudono in
un punto solo**, comprese quelle non ancora immaginate.

⛔ **E la via idiomatica è già stata scartata con una misura, quindi non si riapre:** un waker su
misura renderebbe la scrittura non falsificabile, e **non è costruibile qui** — `Waker::from_raw` è
`unsafe`, `forbid(unsafe_code)` lo rifiuta, misurato in **M-5**. Scritto perché chi legge non
riproponga il meccanismo credendo che nessuno ci abbia pensato — gotcha **#32**.
⛔ **L'altra opzione, caduta sul MERITO e non sul costo:** far possedere la cella all'`Executor`.
È più invasiva — firma pubblica e tutti i chiamanti — **e non chiude il `Drop`**, perché un
distruttore che tiene `&Sleep` scrive lo stesso. Peggiore su entrambi gli assi.

⛔ **Prima il rosso, e le sonde sono DUE perché le VIE sono due — non le cause.** La suite si ferma
al primo rosso, quindi una sonda per via è ciò che impedisce alla seconda di restare non provata
mentre un test afferma il contrario: gotcha **#65**, nella forma che la prima decisione dell'audit
aveva insegnato il 2026-08-17.

| Sonda | Via | Contro il codice non corretto |
|---|---|---|
| `a_request_written_before_the_run_belongs_to_nobody` | il banco scrive prima di `run` | 🔴 il clock arriva a **9999** su un'attività **scelta dal seme** |
| `a_request_written_by_a_destructor_belongs_to_nobody` | un `Drop` scrive dopo il ciclo | 🔴 **9999**, e **anche col drenaggio all'ingresso di `run`** |

⚠️ **La prima stesura della seconda sonda era VACUA, ed è registrata invece di essere rifatta in
silenzio.** Scritta con un blocco `async` che teneva un guardiano, passava contro l'esecutore **non
corretto**: i locali di un blocco `async` muoiono **dentro** il poll che lo completa, prima che
l'esecutore legga la cella. Serve un `Future` **scritto a mano**, il cui distruttore appartiene al
future stesso e gira quando il task finito è tolto dal vettore. È il gotcha **#17**, e per questo
la sonda definitiva **asserisce che il distruttore sia girato** prima di guardare l'oracolo.

⛔ **CIÒ CHE HA INSEGNATO L'ALTRA SONDA DEL RAPPORTO, ed è la notizia di questa passata.**
`a_wait_already_over_wakes_immediately_and_the_clock_does_not_move` scriveva la scadenza **dal
banco**, prima di `run`. Chiudendo K-1 non diventa rossa: **resta verde e smette di provare
qualcosa**. Misurato invece che dedotto — col rimedio applicato e `until <= instant` mutato in
`until < instant`, cioè **la discriminazione che il suo stesso commento dichiara di difendere**:

| Forma della sonda | Sotto la mutazione `<=` → `<` |
|---|---|
| come era, con la scrittura dal banco | 🟢 **verde** — e la mutazione è viva: **cinque** altri test vanno rossi |
| riscritta, con l'attività che dichiara la propria scadenza | 🔴 `Err(TurnLimitReached)` invece di `Ok(())` |

📌 **Un rosso lo vedi. Una vacuità no.** L'audit prevedeva il primo e ha consegnato la seconda: è
il gotcha **#66**.

⛔ **B-1 — il limite di turni consegnato e mai letto.** `parameters_delivered.rs` prova che
`Parameters` **trasporta** il numero; nulla provava che l'esecutore lo **usi**.

| | |
|---|---|
| **Riprodotto** | `turn_limit: { let _ = parameters; 10_000 }` → **32 target, 177 passati, 0 falliti**, identico alla baseline. ⚠️ **Rimisurato sui 177 di oggi** invece di riprendere i **171** del rapporto, che sono di prima di T-1/T-2 |
| **La sonda** | `the_delivered_turn_limit_is_honoured_by_its_value` |
| **L'oracolo** | il **conteggio dei poll**, non l'errore: `run` prende un turno per poll su un'attività che si limita a cedere e si ferma a `turns > limit`, quindi i poll sono **esattamente** il limite. È l'unico osservabile che porta il **valore** |
| **Due valori e non uno** | `7` e `13`, entrambi lontani da ogni default plausibile — gotcha **#48**, *«per ogni mutazione su un valore provane DUE»*. Con uno solo, un'implementazione la cui costante coincide passerebbe |
| **Morde** | 🔴 sotto la mutazione: `left: 10000, right: 7`, col messaggio che nomina il difetto |

⛔ **La mutazione di controllo è la baseline:** col codice vero `bash scripts/gate.sh` dà
`GATE GREEN`, e `cargo test --workspace --no-fail-fast --locked` dà **32 target, 180 passati, 0
falliti, 2 ignorati** — erano **177**, e le tre in più sono le tre sonde di questa passata. Le
asserzioni nuove **non scattano dove non devono**, che è la metà che si dimentica — gotcha **#24**.

⚠️ **E il commento di `Sleep` dichiarava il falso, quindi è stato RISCRITTO e non appeso.** Diceva
*«la richiesta è letta e DRENATA dopo ogni poll, quindi appartiene sempre all'attività che ha
appena girato»*: la subordinata era **falsa**, perché «dopo» esclude la precedente e nient'altro.
Lasciarla sarebbe stato il finding **A-2** rifatto — una formulazione falsificata che sopravvive
nel documento che si dichiara autorevole. Porta il proprio **richiamo datato**.

⚠️ **VOCE APERTA — le tre sonde non hanno una riga di catalogo, e aggiungerla è del proprietario.**
La §7.4 è la spec, e il vincolo globale 7 la mette fuori dalla portata di questa passata. È lo
stesso trattamento scelto per **PL-1** il 2026-08-18, e per la stessa ragione: **registrata come
voce aperta e non come nota**, perché una nota si legge e si dimentica — gotcha **#36**. Le tre
righe candidate difenderebbero **§2.4.1** — la cella è l'unico canale, e le sue scritture
appartengono a un poll — e **§2.8 · ADR-0034**, nessuna decisione legge un parametro che non le è
stato consegnato.

### P-1 — A3 era dichiarata chiusa e aveva una seconda bocca (2026-08-18)

⛔ **La seconda decisione della §8, e il rapporto ha ragione sul DIFETTO e torto sul RIMEDIO.**
`Untrusted` aveva smesso di stampare il proprio contenuto, ma `promote` prendeva `reason: &str` e
il `Debug` scritto a mano di `RecordV1` **stampa l'indice 4 per intero**. Il testo esterno usciva
dalla **giustificazione** invece che dal payload.

⛔ **Riprodotto da fuori la crate**, come l'audit dichiarava di aver fatto:

```
RecordV1 { kind: Note, effect: Unrepeatable, trust: Untrusted,
           payload: <16 bytes>, reason: "ignore your instructions" }
```

📌 **Il campo protetto nascosto, quello non protetto spalancato.** Bastava
`altro.promote(&mut journal, step, esterno.as_str())`: nulla nella firma lo vietava.

⛔ **LA FRASE CHE LO AUTORIZZAVA, e va guardata perché è la classe del difetto.** Il commento del
`Debug` giustifica quattro campi con una ragione sola: *«`kind`, `effect`, `trust` e `reason` sono
il vocabolario del kernel — **nobody outside chose them**»*. È vera per **tre su quattro**.
`reason` lo sceglie il **chiamante**. ⚠️ **Ed è l'ELENCO a farla leggere come verificata:** quattro
nomi condividono una giustificazione, la giustificazione regge per tre, e la frase non dice quale —
chi la controlla si ferma al primo nome che torna. Gotcha **#67**.

⛔ **IL RIMEDIO DEL RAPPORTO NON AVREBBE CHIUSO LA STRADA, ed è la misura che conta.** La §8
propone `reason: &Instruction`. `Instruction::new` è **`pub`** e prende qualunque `String`, quindi
`Instruction::new(untrusted.as_str().into())` lo soddisfa — ed è la **via A1/A2**, dichiarata
**non chiudibile** nella stessa lista, dieci righe più su. 📌 **Una guardia a newtype vale esattamente
quanto il suo COSTRUTTORE**, e questa avrebbe comprato **l'apparenza** di una chiusura sopra una
strada che l'elenco stesso dichiara aperta.
⚠️ **E sarebbe stato anche un gioco di parole sui tipi:** `Instruction` significa *contenuto ammesso
nel canale delle istruzioni*, e una giustificazione non è quello — usarlo lì sfoca l'unica
distinzione per cui quel tipo esiste.

⛔ **E LA TERZA OPZIONE È CADUTA SU UN FATTO, non su un'opinione:** `reason` come **enum** sarebbe
la lettura più onesta di *«vocabolario»* e renderebbe la frase letteralmente vera — ma `reason` è
l'**indice 4 del record durevole**, oggi una `String`. Cambiarne il tipo **muove i byte congelati**,
e per ADR-0036 quello non è un aggiornamento ma un **cambio di formato**, cioè una `Record::V2`.
Sproporzionato per una decisione — *quali reason esistono?* — che oggi ha **un solo chiamante di
produzione**. YAGNI.

✅ **IL RIMEDIO: `reason: &'static str`.** Il contenuto esterno è **dato di runtime**; un
`&'static str` è un letterale nel binario. La strada accidentale **smette di compilare**, a
livello 1.

| | Prezzo misurato |
|---|---|
| `crates/kernel/src/boundary.rs` | **una parola** nella firma |
| siti di chiamata da riscrivere | ⚡ **zero** — `cargo test --workspace --no-run --locked` non dà **nessun** errore: erano **tutti già letterali** |
| oracoli `.stderr` | **uno**, `promote_without_journal.stderr`, e cambia esattamente ciò che deve: `&str` → `&'static str` in due punti. **Letto e modificato a mano**, non rigenerato (gotcha #25) |
| formato durevole | ⛔ **invariato**: l'indice 4 resta una `String`, `crates/kernel/tests/frozen/` non si muove, `frozen_bytes.rs` **6 passati su 6** |

✅ **LA SONDA È UN CASO `compile_fail`, ED È NELLA FORMA FORTE.**
`tests/compile_fail/promote_reason_is_not_runtime_text.rs` — la regola scatta con
`error[E0597]: `smuggled` does not live long enough … argument requires that `smuggled` is
borrowed for `'static``, cioè col messaggio che **nomina il meccanismo**.
⛔ **Provato nell'altra direzione, e non dedotto:** rimessa la firma a `&str`, il caso **COMPILA** e
`trybuild` risponde **`error`** — *«expected compilation to fail»* — invece di `mismatch`. È il
gotcha **#42**: un caso che riporta **compilando** non si disarma con un `TRYBUILD=overwrite` in
blocco. (Nella stessa corsa `promote_without_journal` risponde `mismatch`, che è il controllo: quel
caso passa **dall'oracolo**, questo no.)
⚠️ **L'altra direzione della regola NON è qui, deliberatamente:** che un letterale promuova e si
stampi lo tiene già `boundary_promotion.rs`, che esegue la promozione intera e rilegge il record.
Una copia sarebbe gotcha **#49**.

⛔ **Ciò che RESTA APERTO, dichiarato invece che taciuto.** `String::leak` produce ancora un
`&'static str`: un chiamante deciso a contrabbandare ci riesce — lo stesso scambio con cui **A5**
liquida il `transmute`, *«non è un incidente che uno ha per sbaglio»*. E un letterale può **mentire**
— `"quoted by the user"` su una promozione che l'utente non ha chiesto — che è **provenienza e non
correttezza**, esattamente il limite che **A4** già dichiara. Ciò che ha chiuso è la strada che si
prende **senza accorgersene**.

⛔ **La mutazione di controllo è la baseline:** `GATE GREEN`, `cargo test --workspace
--no-fail-fast --locked` → **32 target, 180 passati, 0 falliti, 2 ignorati**, invariata rispetto
alla chiusura di K-1. I casi di `compile_fail` passano da **diciassette a diciotto**.

⚠️ **VOCE APERTA — la riga di catalogo, come per K-1/B-1 e per PL-1.** La §7.4.1 blocco B ha già la
riga *«promuovere testo a istruzione ← la porta journal»* (V19), e questa regola è la **seconda
metà della stessa difesa**: la porta impedisce la conversione muta, il `'static` impedisce che il
contenuto esca dalla giustificazione. Aggiungerla è **spec**, quindi del proprietario — vincolo
globale 7. **Registrata come voce aperta, non come nota** (gotcha #36).

### Decisione 7 — le cinque sonde mancanti, e l'audit si chiude (2026-08-18)

⛔ **L'ultima decisione della §8, e le cinque voci sono QUATTRO soggetti diversi.** Ciascuna è la
stessa forma di difetto — *un'asserzione vale solo lo stato in cui è fatta* — su una porta diversa.

#### B-2 — quattro gruppi su cinque della conformità `reactor` non erano mai visti scattare

⛔ **La suite più importante del progetto**, quella su cui poggia la validità dell'intera
simulazione deterministica, aveva **due bugiardi per UN gruppo**. Gli altri quattro erano tenuti da
nulla: l'audit misurò che cancellare i blocchi **1, 3, 4 e 5** lasciava l'intero workspace verde.

✅ **Cinque bugiardi nuovi, e sono cinque perché le ASSERZIONI sono cinque — non i gruppi.** La
suite muore alla prima che scatta, quindi un bugiardo rotto in due punti prova solo la prima:
gotcha **#65**.

| Bugiardo | Asserzione | Il difetto |
|---|---|---|
| `BackwardsClockLiar` | **1** | `now()` cammina all'indietro da solo |
| `ShortWaitLiar` | **3a** | il clock arriva alla scadenza, la **risposta** è corta di un ms |
| `LaggingClockLiar` | **3b** | risponde la scadenza esatta e il **proprio clock** si ferma a metà |
| `SecondWaitShortLiar` | **4a** | corretto sulla **prima** attesa, corto sulla **seconda** — conta le chiamate, o non arriverebbe al gruppo 4 |
| `PanickingWallClockLiar` | **5** | `wall_time()` non risponde |

⛔ **Il gruppo 5 non ha asserzioni**, e per questo era l'unico blocco la cui **cancellazione**
nessun oracolo poteva notare: non c'è niente che possa andare rosso. Un reactor il cui `wall_time`
esplode trasforma *«il blocco esiste»* in *«il blocco GIRA»*, che è l'unica proprietà che quel
blocco può avere.

✅ **Seconda direzione, misurata in un colpo solo.** Neutralizzate **tutte e cinque** le asserzioni
insieme, vanno rosse **esattamente le cinque sonde nuove**, ciascuna col proprio messaggio —
`THE SUITE IS VACUOUS ON GROUP 1 / ASSERTION 3a / 3b / 4a`, `GROUP 5 IS DEAD CODE` — mentre le
**tre preesistenti restano verdi**. Il controllo è quelle tre: le asserzioni nuove non scattano
dove non devono (gotcha #24).

⛔ **E SCRIVENDO I BUGIARDI È USCITO UN DIFETTO CHE L'AUDIT NON AVEVA VISTO: l'asserzione 4b è
IMPLICATA dalla 4a.** `second_deadline` è calcolata da `first_reached`, quindi
`second_reached >= second_deadline = first_reached + MARGIN > first_reached`: nessun reactor può
far scattare la 4b senza far scattare prima la 4a, e **un bugiardo per la 4b non è scrivibile**.
⚠️ **Non è vacua, è muta:** non può essere falsa dove l'altra è vera, quindi non mente mai — non
parla mai. **Voce aperta:** toglierla, o ribasare `second_deadline` su `start` così che le due
diventino indipendenti, è una modifica alla **conformità di una porta condivisa**. Registrata,
non presa.

#### B-3 — il finto filesystem poteva smettere di confrontare i `CheckpointId`

⛔ I test tenevano **un solo** checkpoint in archivio, e in quello stato *«trova quello il cui id
corrisponde»* e *«prendi il primo che c'è»* sono **la stessa frase**. Misurato dall'audit:
`restore` riscritta a «prendi il primo» lasciava **13 test su 13** verdi.

📌 **Ciò che la rende non vacua è un PASSANTE** — un secondo checkpoint che non è quello sotto
esame — ed è **il rimedio identico** che la **prima** decisione di questo audit richiese sulla
conformità del giornale (T-1/T-2, 2026-08-17). Stesso difetto, porta diversa.
⛔ **E due argomenti nel sorgente vi poggiavano:** `CheckpointId` e `ClientId` non hanno getter, e
la ragione scritta accanto è che *«un'implementazione lo ritiene e lo CONFRONTA, esattamente come
fa `InMemoryFilesystem`»* — un argomento su un confronto che nulla osservava.

✅ `a_restore_serves_the_checkpoint_it_was_asked_for_and_not_the_first_one`, con **due
direzioni**: il checkpoint **più recente** (che «prendi il primo» sbaglia) e il **più vecchio**
(che «prendi l'ultimo» sbaglierebbe). Una sola lascia metà della mappa non osservata.
✅ **Sotto la mutazione «prendi il primo» è l'UNICA rossa**, e le altre tredici restano verdi.

#### S-1 e S-2 — `note` non era mai esercitata con successo

⛔ La sonda che teneva *«il contatore si muove solo su un `Ok`»* faceva fallire una scrittura
interna **una volta sola, attraverso `outcome`**. Le metà di `intent` e `note` erano tenute da
nulla, e il **percorso di successo di `note`** — delegare, rispondere `Ok`, muovere il contatore —
non era mai stato preso: **ogni** `note` del file rispondeva `NotDurable`.
📌 **E la sonda lo aveva scritto di sé stessa:** *«esclusività su un insieme che cresce è
l'affermazione che invecchia in silenzio»*. Ha invecchiato in **sette giorni**.

✅ Tre sonde, una per via (gotcha #65): `the_success_path_of_note_is_exercised_and_counted`,
`a_refused_note_does_not_consume_a_crash_position`,
`a_refused_intent_does_not_consume_a_crash_position`.

| Mutazione | Chi la uccide |
|---|---|
| `note` incrementa **sempre**, non solo su `Ok` | 🔴 `a_refused_note_…` |
| `intent` incrementa **sempre** | 🔴 `a_refused_intent_…` |
| `note` risponde `Ok` **senza delegare** | 🔴 `a_refused_note_…` **e** `the_success_path_…` |

⛔ **E LA TERZA MUTAZIONE HA TROVATO UN BUCO NELLA MIA STESSA SONDA, corretto invece che
spedito.** La prima stesura di `the_success_path_…` controllava **il contatore** e non che la nota
**raggiungesse l'archivio**: una `note` che risponde `Ok` senza delegare muove il contatore
ugualmente, quindi la mutazione le passava davanti. Chiusa leggendo `replay()` — tre record, e
l'ultimo è la nota. 📌 È la domanda del **#66** applicata a sé stessi: *in quale altro stato del
mondo questa asserzione resterebbe verde?*

#### S-5 — i gradini della scala erano giustificati in un commento e contati da nessuno

⛔ `partial > 0` è soddisfatta da **UN** gradino: conta i **punti** che atterrano a metà, non gli
**archivi distinti** su cui atterrano. Misurato dall'audit: **un mondo a tre pioli su trentuno
supera tutte e cinque le asserzioni**.
⛔ **E ciò che difende è l'intera ragione per cui questa campagna è PROFONDA invece che LARGA.**
Il doc di `DEEP_RECORDS` argomenta che allargare non compra nulla — *800 punti, sempre 35 cadute* —
mentre più **record** comprano stati, e la sua evidenza è una tabella di pioli: **4/4, 11/11,
21/21, 31/31, 41/41**. Era la tesi portante del disegno, **ed era prosa**.

✅ **Ora è un controllo:** i pioli sono l'insieme delle **lunghezze di prefisso distinte**
recuperate nella spazzata, e l'asserzione è un'**uguaglianza** — `records + 1`, cioè l'archivio
vuoto più un piolo per record — perché è esattamente ciò che l'argomento afferma.

| Campagna | Misurato il 2026-08-18 |
|---|---|
| corta, 3 record | `points=35 fired=35 truncated=22 partial=17` **`rungs=4/4`** |
| profonda, 30 record | `points=197 fired=197 truncated=184 partial=179` **`rungs=31/31`**, 5,2 s |

✅ **La tabella del disegno regge a entrambe le profondità**, rimisurata invece che citata.
✅ **Seconda direzione:** modellato il mondo a pochi pioli dell'audit, l'asserzione scatta —
*«the sweep reached 3 distinct recoverable archives out of the 4 … Rungs seen: {0, 1, 2}»* —
mentre le **quattro precedenti restano verdi**, che è esattamente ciò che l'audit aveva misurato.

📌 **Baseline dopo la decisione 7:** `GATE GREEN`, **32 target, 194 passati, 0 falliti, 2
ignorati** — erano **180**. I quattordici in più sono le nove sonde nuove, di cui le **cinque del
`reactor` contano DOPPIO** perché `reactor_contract.rs` è `include!`d anche da `platform`.
⚠️ **E l'intestazione di quel file dichiarava «i tre test qui sotto girano una seconda volta»**:
sono **otto** dal 2026-08-18, e la frase che seguiva — *«la finta e i due bugiardi dormono per
niente»* — era sbagliata in **entrambe** le metà già prima, perché un bugiardo è un clock finto e
non dorme mai. Ricontata sul sorgente, gotcha **#31**.

#### `CrashingJournal` — cinque mutazioni, cinque uccise, e una coppia che prova di essere due difetti

**Misurate il 2026-08-11, chiudendo il Task 1 del Traguardo 4.** ⚠️ **Rifatte da zero dopo
un'interruzione:** la prima passata era stata eseguita e il suo verbale è andato perso, e un
commit senza il proprio verbale è un numero senza misura — rifarle costa meno che fidarsene.
Applicazione provata con `grep` **dopo** l'edit, compilazione in un passo **separato**
dall'esecuzione, ripristino con `git checkout --` e albero verificato vuoto fra una e l'altra
(gotcha **#48**).

| | Mutazione | Chi la uccide |
|---|---|---|
| A | `may_write` risponde sempre `true` — non cade mai | **sei** sonde. ⚠️ Il piano ne elencava **quattro**: il criterio enumerava nomi e l'insieme è cresciuto sotto di lui |
| B | `may_write` risponde sempre `false` — cade sempre | sei sonde, e la sola che conta è `a_journal_told_not_to_crash_never_falls` |
| C | il contatore avanza **anche su una scrittura rifiutata** | ⛔ **solo** `a_write_the_protocol_refuses_does_not_consume_a_crash_position`, riga 127 |
| D | la guardia di `prune` **tolta** | ⛔ solo `after_the_fall_pruning_is_refused_too_but_it_never_causes_the_fall`, riga **82** |
| D′ | la guardia di `prune` passa da `may_write()` invece che da `fallen` | ⛔ la **stessa** sonda, riga **68** |

⛔ **A e B insieme sono la prova in due direzioni, e senza la coppia una delle due sonde
sembrerebbe copertura senza esserlo.** `a_journal_told_not_to_crash_never_falls` è **verde
sotto A e rossa sotto B**: è l'unica in quella direzione, ed è quella su cui `C7a` poggerà. Il
suo specchio esatto è `after_the_fall_every_later_write_is_refused_too`, **rossa sotto A e verde
sotto B** — asserisce solo rifiuti, quindi un giornale che rifiuta sempre la soddisfa. Entrambe
sono sonde **a senso unico**, e ciascuna è la controparte dell'altra.

⛔ **D e D′ uccidono asserzioni DIVERSE, ed è la sola cosa che quella coppia esisteva per
scoprire.** Righe 82 e 68, con due messaggi di panico distinti: la sonda **distingue** *«un
processo morto pota ancora»* da *«la potatura arma la caduta»*, quindi il gotcha **#55** — più
asserzioni che condividono un esito, e una mutazione mirata a una colta da un'altra — **non si
applica qui**. ⚠️ **E una divergenza dall'atteso, registrata invece che appianata:** D′ era
attesa uccidere la riga 69 o le 70-74, e uccide la **68** — al momento della prima potatura il
contatore è già pari al punto di caduta, quindi `may_write()` rifiuta **in tronco** e il test
muore prima di poter osservare l'armamento. L'armamento avviene lo stesso; nessuna asserzione
arriva a vederlo. L'esito è **più forte** dell'atteso, e diverso.

⚠️ **Una seconda divergenza, sul modo in cui D uccide.** Il panico è `Err(Missing)` contro
`Err(NotDurable)` e non `Ok(())` contro `Err(NotDurable)`: la seconda potatura raggiunge il
giornale interno, che risponde *«manca»* perché la prima l'aveva già rimosso. L'uccisione resta
valida — l'asserzione confronta la **variante** e non l'esito, che è la lezione del gotcha
**#30** — ma il messaggio non dice *«la potatura è riuscita»*.

⛔ **E la non-vacuità della sonda nuova è provata togliendo IL BLOCCO, non un'asserzione**
(gotcha **#55** al rovescio). Con la mutazione C attiva e il corpo di
`a_write_the_protocol_refuses_does_not_consume_a_crash_position` racchiuso in un commento a
blocco, il banco torna **verde**: le altre nove sonde sono **cieche** a C, e la frase *«the
counter moves only on an `Ok`»* la tiene lei sola.

📌 **E il conteggio delle posizioni di caduta è stato verificato per una via che non passa da
Rust**, perché la cifra scritta nel commento della sonda era stata **citata** da una revisione
prima che qualcuno la rimisurasse (gotcha **#53**). Ricalcolando xorshift64 fuori dal
repository, sui cinquecento semi e con otto scritture, le **otto** posizioni escono tutte:
`{61, 69, 66, 62, 63, 57, 58, 64}`, minimo **57** e massimo **69**. La cifra era vera.

#### La campagna di livello 1 — quattro mutazioni, e una riga nuova che si è messa davanti alle altre

**Misurate il 2026-08-11, chiudendo il Task 2 del Traguardo 4.** Applicazione provata dopo
l'edit, compilazione in un passo **separato** dall'esecuzione, ripristino verificato col
`git diff` contro il commit precedente e non a `grep` (gotcha **#48**).

| | Mutazione | Sonda rossa | Asserzione che ha sparato |
|---|---|---|---|
| A | lo scenario smette di scrivere l'**esito** | **entrambe** | il pin su `writes_done` · `c7a` sull'oracolo nuovo — `left: 12, right: 24` |
| C | il giornale cade alla scrittura **zero** | solo `c7a` | ⛔ **l'oracolo nuovo**, *«nothing was written to be in doubt about»* — `left: 0, right: 24` |
| D | il giornale cade alla scrittura **cinque** | solo `c7a` | l'oracolo nuovo — `left: 5, right: 24` |
| E | l'esito **si scrive**, ma il record dice `Intent` | solo `c7a` | ⛔ **l'asserzione dell'insieme**, dodici passi in dubbio, tutti `RunAgain` |

⛔ **La mutazione E non era prevista, e senza di lei la passata avrebbe concluso il falso.** Le
mutazioni C e D uccidono **la stessa** asserzione — quella nuova — e la lettura ovvia è il
gotcha **#55**: due difetti che una sonda non distingue. ⚠️ **La lettura ovvia è sbagliata**, e a
dirlo è E: la riga nuova non oscura quella dell'insieme, la **domina** su un asse diverso.
Sono due mansioni disgiunte — la nuova giudica **lo scenario** (*ha scritto?*), quella
dell'insieme giudica **la riconciliazione** (*è d'accordo?*) — e la seconda è raggiungibile
esattamente quando lo scenario è intatto, che è il soggetto dichiarato di `C7a`. L'ordine delle
due è quello giusto. 📌 **La regola che ne esce:** quando due mutazioni uccidono la stessa
asserzione, prima di concludere che la sonda non distingue si cerca **una terza mutazione che
lasci passare la prima asserzione**. Se esiste, le due non erano in competizione.

⚠️ **E una regressione di copertura, registrata invece che taciuta:** prima dell'oracolo nuovo,
la mutazione A uccideva `c7a` **sull'insieme**, elencando dodici dubbi; ora la uccide sulla riga
nuova. Nessun rosso è andato perso, ma il **punto d'impatto si è spostato** — è il gotcha #48
nella forma in cui una riga aggiunta si mette davanti, e la sola ragione per cui si è visto è
che si è rilanciata la campagna dopo una rifinitura invece che solo dopo un cambiamento di
comportamento.

✅ **E il controfattuale sequenziale è stato costruito e misurato, non argomentato**, perché la
decisione di **non** fissare l'interlacciamento in questo file dipende da esso: commentando la
sospensione — così che ogni attività finisca al primo poll — il massimo insieme in dubbio scende
da **tre** a **uno** su tutti e cinquanta i semi. Quindi la sonda del compito successivo va
davvero rossa su uno scenario sequenziale, e una sonda in più qui sarebbe stata duplicazione
(gotcha **#49**).

#### `C7b` — sei mutazioni, e due esistono perché la non-vacuità dichiarata non era quella vera

**Misurate il 2026-08-11, chiudendo il Task 3 del Traguardo 4.** Applicazione provata dopo
l'edit, compilazione in un passo **separato**, ripristino verificato e verde riconfermato fra una
mutazione e l'altra (gotcha **#48**).

| | Mutazione | Sonda rossa | Asserzione che ha sparato |
|---|---|---|---|
| A | `RecordKind::Intent => {}` in `reconcile.rs` — la riconciliazione non riporta niente | `C7b` e la sonda dell'interlacciamento | l'**insieme**, `left: []` contro `right: [3]` |
| B | `resolution_of` risponde sempre `SuspendAndAsk` | solo `C7b` | ⛔ **la risoluzione**, `SuspendAndAsk` contro `RunAgain` |
| C | nessun crash, su **entrambi** i siti di chiamata | entrambe | il conteggio, `left: 0` contro `right: 200` |
| D | `expected_doubt` non chiude più il dubbio sull'esito | solo `C7b` | l'**insieme**, `left: [3]` dall'archivio contro dodici passi dalla traccia |
| E | il giornale cade alla scrittura **zero** su ogni seme | `C7b` e la fratella | ⛔ **la non-vacuità nuova**, non il conteggio |
| F | `ACTIVITIES = 1` — nessun interlacciamento | `C7b` e la fratella | ⛔ **la non-vacuità nuova** |

⛔ **Le mutazioni E ed F sono la ragione per cui `C7b` ha un secondo oracolo, e il difetto che
hanno scoperto era stato chiuso un compito prima su `C7a`.** L'asserzione sul conteggio dei crash
prova che **il guasto è scattato**; non prova che la **riconciliazione sia stata esercitata**. Con
un giornale che cade al primo byte, duecento semi su duecento cadono davvero **e duecento
confronti su duecento sono `[] == []`**: la campagna è verde e non ha verificato nulla. Con una
sola attività, ogni crash lascia al più un passo. ⚠️ E già senza mutazioni **sei semi su
duecento** confrontano due insiemi vuoti sui propri meriti. 📌 **La forma generale, che è la
notizia:** *«l'iniezione è avvenuta»* e *«c'era qualcosa da verificare»* sono **due** affermazioni,
e un solo oracolo ne copre una. Una campagna ne vuole due.

⛔ **E il discrimine è stato provato, non assunto:** sotto E ed F spara la **riga nuova** e non
quella del conteggio, che è ciò che dice che la riga nuova difende quello per cui è nata. Sotto C
— nessun crash — spara ancora **il conteggio**, ed è la risposta giusta: senza guasto non c'è
dubbio, e il difetto è che l'iniezione non è arrivata.

⚠️ **Una previsione del coordinatore smentita dalla misura, registrata:** la mutazione F era
attesa produrre collaterali sul pin e su `C7a`, perché `WRITES_PER_RUN` scende da ventiquattro a
otto. Non ne ha prodotti — la costante è **derivata**, quindi si ricalcola e le asserzioni
confrontano otto con otto. È il caso che il commento del pin dichiara: *«fissa una relazione, non
un numero»*.

📌 **E le righe si sono mosse senza che le asserzioni si muovessero**, ricontrollato rilanciando
A, B, C e D dopo le correzioni: nessuna è finita davanti a un'asserzione nuova. È il contro-verso
del difetto che il Task 2 aveva registrato, dove una riga aggiunta si era messa davanti.

#### I byte congelati — tredici mutazioni, e due dettate dal piano non compilavano

**Misurate il 2026-08-10, chiudendo il Task 10.** ⚠️ Applicazione provata **per byte**,
compilazione in un passo **separato** dall'esecuzione, ripristino **byte-identico** dei sorgenti
(gotcha **#48**, vincolo globale 5). ⛔ **L'oggetto è l'unico artefatto del progetto che non si
corregge:** se i byte cambiano non è un aggiornamento, è un cambio di formato.

| Mutazione | Chi cade (su **6** sonde) |
|---|---|
| ⛔ **dettata dal piano** — `payload` `#[n(3)]` → `#[n(2)]`, già di `trust` | ⛔ **non compila**: `duplicate index numbers`. Il controllo non si sarebbe mai visto scattare |
| ✅ `kind` 0 ↔ `effect` 1, scambio fra indici esistenti | **3** — encode, decode, e la sonda degli offset |
| ✅ `payload` 3 → 7, indice libero | **3** — le stesse |
| ⛔ **le otto varianti rinumerate una per una** su un indice libero | **2 ciascuna** — encode e decode — e ⛔ **otto volte su otto**. È la sonda **F3**, e il motivo per cui i record congelati sono tre |
| ⛔ **dettata dal piano** — campo facoltativo a `#[n(4)]`, già di `reason` | ⛔ **non compila**, stesso errore. ⚠️ E il piano avrebbe letto quel rosso come *«ADR-0036 smentito dalla misura»*: un errore di compilazione scambiato per una scoperta sull'architettura |
| ✅ campo facoltativo a `#[n(5)]`, indice libero, con `#[cbor(default)]` | ✅ **nessuna**, ed è la risposta voluta: i byte congelati non si muovono |
| ⛔ lo stesso campo valorizzato — `Some(9)` invece di `None` | **21 → 22 byte**, `85` → `86` e `09` in fondo: il campo **arriva sul filo**, quindi il verde qui sopra significa qualcosa. Senza questa misura sarebbe stato vacuo — gotcha **#54** |
| una **variante nuova** di `RecordKind` (`#[n(3)] Amend`) | ⛔ **la LIBRERIA non compila** — `E0004` in `crate::reconcile` — quindi il livello 1 arriva **prima** dell'oracolo e le sonde non lo vedono mai. Misurato invece che dedotto |
| **controllo**: una parola di un commento di `record.rs` | ✅ **nulla**, `6 passed` |

📌 **E la mappa non è prosa.** `record_v1.map` è **riletta** dal banco: le colonne `offset` e
`hex` di ogni riga devono ricostruire il `.cbor` byte per byte, quindi un `<fill in>` non si
analizza e un byte trascritto male non corrisponde — il gotcha **#43** chiuso da un controllo
invece che da un avviso. ⚠️ **La colonna di prosa è dichiarata NON verificata dentro la mappa
stessa:** un documento controllato a metà che non dice quale metà è peggio di uno che nessuno
controlla.

#### Tre moduli di test vivono in `src/`

📌 **Tre moduli di test vivono in `src/` invece che in `tests/`, e la deviazione è dichiarata
in tutti e tre i file.** Non è una scorciatoia: in due casi su tre non è nemmeno una scelta.
⚠️ **Erano DUE fino al Traguardo 5 Task 7**, che ha portato il terzo — ed è anche il primo
`#[cfg(test)]` di `kernel`.

| Dove | Che cosa difende, e perché sta in `src/` |
|---|---|
| `crates/daemon/src/main.rs` (⚠️ **quante sonde, NON è scritto qui:** il conteggio vive in un posto solo, la sezione del **Task 10** di questo file. Questa cella diceva **un test** ed era ferma a prima che il Task 10 vi portasse il giornale, l'arbitro e le due concessioni — gotcha **#31**, tolto invece che riallineato) | che il **grafo di produzione si monti e giri** — il cablaggio, non il dimensionamento del limite di turni: misurato in due direzioni, e col limite a `0` la suite resta **verde** perché senza attività il corpo non gira mai — che il **giornale** sia davvero aperto e che uno che non si apre **fermi l'avvio**, che le due quote di ADR-0033 siano **tenute e non sottratte** e che la policy montata sia il default di ADR-0006, che una concessione permanente **non scada**, e che una quota permanente che non entra fermi l'avvio **nominandosi** — per l'una e per l'altra delle due vie, `Queued` e `Refused` (`E41`). Sta in `src/` perché le funzioni sotto test sono **private in un target `bin`**, e nessun test d'integrazione può linkare un binario |
| `crates/platform/src/rng.rs` (cinque test) | `SequentialRng`: che le estrazioni siano 0, 1, 2 e così via, che `new` e `default` siano **lo stesso** generatore, che `below` percorra gli indici a turno **a limite costante**, il limite dichiarato quando il limite **cambia**, e che il contatore **avvolga invece di traboccare**. Sta in `src/` perché l'ultimo costruisce `SequentialRng(u64::MAX)` **col campo privato**, irraggiungibile da una crate a parte senza 2^64 estrazioni o un costruttore che esisterebbe solo per il test |
| `crates/kernel/src/arbiter/mod.rs` (⚠️ **quanti test, NON è scritto qui:** il conteggio vive in un posto solo, la sezione del Task 7 di questo file. Questa cella diceva **dodici** ed era ferma alla prima delle tre ondate del 2026-08-20 — gotcha **#31**, tolto invece che riallineato) | la **revoca** del Traguardo 5 Task 7: che chiedere indietro **marchi e non prenda**, che la grazia si riscuota quando scade e **non prima**, l'**istante** in cui scade, che una concessione non prelazionabile e una corsia non inferiore non si tocchino, che **un pari nella corsia che chiede** non si tocchi — `below` è **esclusivo** (`E71`) — che si fermi appena la stanza basta e prenda la corsia **peggiore** per prima, che **non marchi nulla** quando il recuperabile non copre il bisogno (`E69`), che chiedere **due volte** non compri la stanza due volte, e che riscuota lo scaduto **prima** di marcare. Sta in `src/` perché `ask_back` è `pub(crate)` — il suo unico chiamante di produzione è l'ammissione sotto policy LOCALE, Task 8 — e un `pub(crate)` da una crate a parte è `` error[E0624]: method `ask_back` is private ``, misurato. ⛔ **Solo la privatezza sposta una sonda lì:** l'undicesima del compito non chiama `ask_back` e **resta** in `crates/kernel/tests/arbiter_admission.rs` |

### P-2 — la ragione dello scaglionamento era falsa, e lo era dal Traguardo 2 (2026-08-21)

⛔ **Il finding.** [`audit-2026-08-11.md`](audit-2026-08-11.md) §5.3: *«due regole di livello 1
di `process` non le tiene niente, e il motivo scritto è falso: "serve un `Grant`" — ma un
`Worker` si ottiene implementando il tratto»*. È l'**unica** voce di §5.3 rimasta senza marca:
P-1, K-1 e PL-1 furono chiuse il 2026-08-18, questa no.

⛔ **Perché si chiude adesso.** Non era stata rimandata con un argomento: il pre-controllo del
Task 11 l'ha ritrovata come **premessa del compito** — il cui titolo dice *«i quattro casi che
il `Grant` sblocca»* — e il proprietario ha deciso il 2026-08-21 di chiuderla **prima** di
dispacciare.

✅ **Misurato, non ripreso dal rapporto** (gotcha **#65**: *un rapporto è un piano, e si prezza
leggendo il codice*). Una sonda d'integrazione usa-e-getta,
`crates/kernel/tests/zz_p2_throwaway_probe.rs`, scritta **da fuori dalla crate**, compilata,
passata e **cancellata nella stessa corsa** — `git status --porcelain` vuoto verificato dopo:

| | Misura | Esito |
|---|---|---|
| **1** | `impl Worker for W` da fuori dalla crate, **senza nominare `Grant`** | ✅ compila e passa — un `Worker` **non** viene solo da `start` |
| **2** | un `Grant` vero da `Arbiter::admit`, consegnato a un `Process::start` scritto da fuori | ✅ compila e passa — `start` **è chiamabile oggi** |
| **3** | `grep -rn "\.start(" crates/ --include=*.rs` | **zero** chiamanti in tutto il workspace. ⚠️ **Qualificata il 2026-08-22 come il gemello di questa frase nella cella di §6.10.5 in fondo al file** — che porta *«rimisurato prima di questo compito»* — perché **questa casa non era stata censita**: una correzione attraversa il punto in cui nasce e non gli altri (radice **R1** dell'audit). ⛔ **La cifra è VERA DELLA SUA DATA E FALSA DI OGGI, e la divergenza si registra invece di allineare la frase.** Verificata in **entrambe** le direzioni con `git grep`: a `5fceee1^` — il commit prima del prodotto del Task 11 — il comando dà **zero righe**, quindi la misura era giusta; a `HEAD` ne dà **dodici**, di cui **cinque sono siti di chiamata** — tre nei casi `compile_fail` di §6.10.5 (`instructing_after_the_kill.rs`, `reading_without_a_receipt.rs`, `reading_twice_from_one_receipt.rs`) e due in `crates/kernel/tests/worker_tokens.rs` — e le altre **sette** sono menzioni dentro commenti, che il comando **non distingue**: chi lo rilancia legge dodici e deve guardarle |

⛔ **Quindi P-2 è vero in una metà e SCADUTO nell'altra, ed è la parte che il rapporto non
poteva sapere.** La metà *«un `Worker` si ottiene implementando il tratto»* era vera quando fu
scritta e lo è ancora — `ScriptedWorker` in `crates/kernel/tests/ports_are_implementable.rs`
esiste dal **Traguardo 2**, e in quel file `Grant` compare **solo** nella firma di `start`. La
metà che il rapporto chiamava *«giusta»* — *«`Grant` non ha costruttore pubblico, quindi
`Process::start` non è davvero chiamabile»* — è **spesa dal Traguardo 5 Task 5**:
`Admission::Granted(Grant)` è pubblica. Gotcha **#77**: una condizione scritta in prosa si
avvera e non diventa rosso niente.

✅ **Le case della ragione falsa, censite col `grep` e guardate in faccia una per una**
(gotcha **#70**). Sei vive, corrette; tre verbali storici, non toccati.

| | Casa | Cosa si è fatto |
|---|---|---|
| 1 | `crates/kernel/src/ports/process.rs`, doc di modulo | ragione **tolta**, richiamo datato |
| 2 | `crates/kernel/tests/ports_are_implementable.rs`, doc di `SpawningProcess` | *«la metà che non può essere CHIAMATA da qui»* — richiamo datato |
| 3 | lo stesso file, chiusa di `killing_a_worker_consumes_it` | ragione **tolta**, richiamo datato |
| 4 | lo stesso file, **il NOME della sonda** e il suo primo capoverso | rinominata `the_process_port_is_implementable` — precedente **E40** |
| 5 | questo file, riga del blocco **B** dei gettoni | ragione **tolta**: viveva **riscritta due volte**, ed era falsa in entrambe le stesure |
| 6 | questo file, riga delle **righe 1–4 di §6.10.5** | **riscritta** col richiamo — a essere falso è il **fatto** e non una qualificazione (#76, limite dichiarato) |
| — | i piani del Traguardo 2 e del Traguardo 5, e il disegno del Traguardo 5 | ⛔ **non toccati**: sono verbali. La ragione falsa che il Task 11 **detta ancora** nel proprio codice è una voce d'errata |

⛔ **E una casa che il `grep` sul nome intero NON trovava — forma nuova del #70.** La riga
**M10** della tabella delle mutazioni nominava la sonda **in forma abbreviata**,
`..._start_is_not_callable`. Un censimento fatto sull'identificatore completo la manca; a
trovarla è stato cercare `is_implementable`, cioè il **frammento**. 📌 *Un rinomino si censisce
sul frammento più corto che resti unico, non sul nome intero.*

⚠️ **Ciò che P-2 NON chiude, dichiarato invece che taciuto.** Le **quattro righe di §6.10.5**
restano scoperte, e adesso per la **sola** ragione vera: la direzione *«deve scattare»* non è
scritta. La chiude il **Task 11**. P-2 si chiude quindi **nella metà chiudibile** — la ragione —
con la copertura **indirizzata**, che è la forma già usata per il gotcha **#51**.

✅ **Richiamo del 2026-08-21 — Traguardo 5, Task 11: l'innesco qui sopra è SCATTATO ed è stato
RACCOLTO.** La direzione *«deve scattare»* è ora scritta — i quattro casi `compile_fail` con le
contro-sonde di `crates/kernel/tests/worker_tokens.rs` — e le **quattro righe di §6.10.5** sono
**chiuse**: la riga «le righe 1–4 di §6.10.5» della tabella «Cosa la porta NON controlla» porta
✅ **CHIUSA il 2026-08-21, col Task 11**.

⚠️ **E una divergenza dal ledger, registrata perché il proprietario la veda.** Il pre-controllo
del 2026-08-21 dava *«la chiusura di P-2 tocca §7.4, che è SPEC»*, cioè fuori portata per il
vincolo globale 7. **Misurato: non la tocca.** Nessuna riga di catalogo è aggiunta, tolta o
riformulata — le due del blocco **B** e le due del blocco **C** restano esattamente com'erano.
A cambiare è la **prosa che spiega perché sono scoperte**, che vive qui e nel sorgente.
📌 È il gotcha **#65** applicato a un **pre-controllo** invece che a un rapporto d'audit: *anche
un pre-controllo è un'affermazione, e si prezza leggendo il codice.*

📌 **Baseline a passata chiusa:** rimisurata sotto, insieme al cancello.

### La revisione del compito 5 del Traguardo 6 — due mutanti vivi e un conteggio stantio (2026-08-31)

⛔ **Questa passata è il ciclo di revisione che la voce `E53` dichiarava mancante**, fatto da una
sessione fresca e **non** da un sotto-agente, che restavano disabilitati. Il residuo che resta è
scritto in `E53` stessa, che ne è la casa unica; qui stanno le **misure**.

📌 **Baseline della passata, rimisurata e non citata:** `bash scripts/gate.sh` → `GATE GREEN`;
`cargo test --locked --workspace --no-fail-fast` → **41 bersagli, 297 passate, 0 fallite,
2 ignorate**. È il termine di paragone di ogni cifra qui sotto.

⚠️ **Le cinque mutazioni dettate dal compito erano già state eseguite** e ne era uscita `E51`.
⛔ **Le due che seguono NON erano fra quelle cinque**, ed è la ragione per cui erano vive: una
campagna di mutazione prova ciò che elenca, e il criterio *«ogni mutazione che non ha ucciso
niente è dichiarata»* non può mordere una mutazione che nessuno ha scritto.

| | La mutazione | Esito misurato | Che cosa se n'è fatto |
|---|---|---|---|
| **S-1** | una **seconda variante** `#[n(1)]` su `Detail` | ⛔ **41 bersagli, 297 passate, 0 fallite, 2 ignorate** — identico alla baseline **cifra per cifra**: mutante vivo | ✅ **chiuso** — `match` esaustivo su `Detail` in `frozen_bytes.rs`, e la stessa mutazione ora dà `` error[E0004]: non-exhaustive patterns: `&Detail::Routing(_)` not covered ``. Voce `E54` |
| **S-2** | `EffectClass::Idempotent` → `Unrepeatable` nel record di **feedback** di `run_the_ring` | ⛔ **41 bersagli, 297 passate, 0 fallite, 2 ignorate** — identico: mutante vivo | ✅ **chiuso il 2026-08-31, per decisione del proprietario:** la classe la **consegna il chiamante**, e la sonda la pinza su **due** valori, nessuno dei quali è il letterale vecchio. La mutazione ora muore col proprio messaggio. Voce `E55` |
| **di controllo** | una **quinta variante** `#[n(4)]` su `RecordKind` | ✅ `` error[E0004]: non-exhaustive patterns: `RecordKind::Routing` not covered `` | è la **seconda direzione**: dice che il rimedio di `S-1` porta `Detail` dove i suoi tre fratelli già erano |

⛔ **Perché `S-1` contava, e non è una rifinitura.** Gli indici di `Detail` viaggiano **sul filo**
e non si ritirano mai (regola 4 di §4.9.2), mentre `frozen_bytes.rs` dichiara di sé che *«una
variante nuova di questi enum è un CAMBIO DI FORMATO»* e che il compilatore lo ferma. Quella
garanzia era vera di **tre** enum ed enunciata per il formato: un quarto enum sul filo entrava
senza che niente diventasse rosso. È la forma del difetto che il Task 10 del Traguardo 3 pagò —
*«un record solo fissa tre indici di variante su otto»* — su una popolazione nuova.

⚠️ **Il rimedio è deliberatamente NON più forte dei tre fratelli.** Estendere l'arm senza
congelare un record compila ancora: è il **limite dichiarato** che gli altri tre portano già,
scritto accanto a loro. Renderlo più forte per il solo `Detail` sarebbe una **seconda
convenzione** per una proprietà sola, che è ciò che §7.4.4 rifiuta.

⛔ **Perché `S-2` è dichiarato e non pinzato, ed è la distinzione che conta.** La classe del
record di verdetto — `Verifiable` — `reconcile` **non la legge mai**, ed è provato dall'arm vuoto
di `RecordKind::Verdict`; il codice lo scrive e porta cinque righe a giustificarla. La classe del
record di **feedback** — `Idempotent` — `reconcile` **la legge**, `RecordKind::Intent => enter(..,
resolution_of(body.effect))`, e decide come ogni passo correttivo dell'anello viene riconciliato
dopo una caduta: e non porta **nessuna** giustificazione. ⚠️ **È la forma che la revisione del
compito 4 aveva già trovato**, in un posto nuovo: il campo che si **rifiuta** è argomentato,
quelli a cui si **obbedisce** no.
✅ **DECISO IL 2026-08-31: la classe la consegna il chiamante**, `correction_effect: EffectClass`.
⚖️ **E la ragione che decide non è la lettera di ADR-0007 — che pure tirava da quella parte — ma la
COERENZA:** è ciò che quella stessa funzione fa **una riga sopra** per `next`, *«consegnato e non
allocato … inventarne uno qui prenderebbe quella decisione scrivendola»*, ed è la forma di
ADR-0034. L'anello non sa che cosa farà la correzione; chi lo sa è il chiamante. Così non si
sceglie fra *«riesegui alla cieca»* e *«fermati sempre»*: si smette di **indovinare**.
⚠️ **Costo dichiarato:** un sesto parametro, e quattro siti di chiamata che lo passano. La sonda
usa **due** valori e **nessuno è `Idempotent`** (gotcha #48), quindi un anello che ignorasse
l'argomento e riscrivesse il letterale vecchio fallirebbe su **entrambi** — misurato: fallisce.

✅ **E un terzo rilievo, che non è una mutazione ma un censimento** — voce `E56`. Due commenti di
sorgente dicevano **al presente** che `frozen_bytes.rs` congela *«TRE»* record, e questo compito
ne ha fatto il quarto: `crates/kernel/src/record.rs` e `crates/kernel/tests/record_shape.rs`.
⚠️ **Entrambi stanno dentro un capoverso che si dichiara datato**, ed è esattamente la distinzione
della **55ª misura** del compendio: quei capoversi datano ciò che **dicevano prima** e poi
affermano il conteggio **al presente**, cioè cadono dal lato che mente. Il conteggio è **tolto e
non riallineato a quattro**; la misura degli otto rossi resta attribuita al 2026-08-10, dove è
vera.

⚠️ **E UNA DIVERGENZA DI DATA, REGISTRATA E NON APPIANATA.** Le due celle di questo file che
datano una misura del compito 5 al **2026-09-01** — la riga `V10` del blocco C e l'ottavo
riconteggio del blocco — **restano come sono**, e `git log` data tutti e tre i commit di quel
compito al **2026-08-31**. ⛔ **Ciò che è stato tolto sono le due affermazioni di STATO** — la §6
del compendio e la tabella delle parti del piano — perché lì la data diceva *«il compito è stato
eseguito il»*, e per quella domanda l'autorità è il commit: *ogni riga porta il proprio commit, e
il commit È la sua data*. ⚠️ **Un verbale invece registra ciò che quella sessione credeva quando
misurò**, e riscriverlo distruggerebbe l'unica prova di che cosa fu creduto: è la distinzione
della **55ª misura**, applicata a una data invece che a una cifra. Il racconto sta nella tabella
delle parti del piano, in una casa sola.

⚠️ **E `riferimenti.md` NON è stato toccato, deliberatamente**, come le sei passate precedenti:
le misure di questa revisione vivono qui, accanto ai controlli che difendono. È la voce aperta
che la §6 del compendio porta da sei passate — scegliere fra *«spostarle»* e *«cambiare la
regola»* è del proprietario, e cominciare a metà traguardo produrrebbe **due** convenzioni invece
di una. Questa passata la allarga di un'occorrenza e non la chiude.

⚠️ **E IL RIMEDIO STESSO È STATO RIVISTO — che è il #45 — E CI SI È TROVATO UN DIFETTO.** Il
messaggio dell'asserzione nuova era **collassato su una riga**, con l'indentazione dentro la
stringa: **110 caratteri**, perché una continuazione di riga non sopravvive a uno script che
riscrive il sorgente. ⛔ **E `cargo fmt --all --check` esce VERDE su quella riga**, rimisurato
rimettendo il commit in albero — né il cancello né `fmt` possono vederlo. Da lì il gotcha **#97**.
Chiuso in `488cae0`.

📌 **La lezione di metodo, e vale oltre il caso.** Tutti e tre i rilievi vivono in ciò che il
compito **ha reso stantio** senza toccarlo: un enum nuovo che nessuna guardia copriva, un campo
il cui valore nessuna delle mutazioni elencate raggiungeva, e due frasi che erano vere finché i
record congelati erano tre. ⛔ **Nessuno dei tre si vede rileggendo il diff**, che è pulito e
argomentato riga per riga: si vedono **mutando** e **censendo**. È la ragione per cui `E53`
diceva che una rilettura dell'autore non vale una revisione.

### La SECONDA passata di revisione del compito 5 — quattro mutanti vivi, e una radice sola

⛔ **Fatta il 2026-08-31 da una sessione fresca, sul perimetro allargato che la prima passata ha
lasciato:** `E53` chiede *«un secondo giro, sul perimetro che ora è più largo perché la revisione
stessa ha scritto codice»*. La baseline di partenza è **41 bersagli, 298 passate, 0 fallite, 2
ignorate**, `GATE GREEN` — rimisurata e non citata.

✅ **PRIMA COSA MISURATA: IL PRODOTTO DELLA PRIMA PASSATA REGGE.** La guardia di crescita su
`Detail` — il rimedio di `E54` — dà `` error[E0004]: non-exhaustive patterns: `&Detail::Routing(_)`
not covered `` aggiungendo una variante, e la mutazione è stata revocata da copia byte-esatta. Le
due sonde nuove di `reconciliation.rs` uccidono **entrambe** le vie alternative dell'arm `Verdict`
di `steps_in_doubt`: trattato come `enter` → `a_verdict_does_not_put_a_step_in_doubt` **e**
`a_verdict_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them` rosse (**2 fallite**); trattato come `leave` → la seconda
rossa (**1 fallita**). ⛔ **Il commento accanto all'arm AFFERMA che entrambe furono provate, ed è
un'affermazione come le altre** (gotcha **#65**): rimisurata, **regge**.

⛔ **QUATTRO MUTANTI VIVI SU `run_the_ring`, e la radice è che la funzione scrive DUE record
mentre il banco ne teneva per intero uno.** Ciascuna applicata da sola, revocata da copia
byte-esatta, e ciascuna col medesimo esito **identico alla baseline**:

| | Mutazione | Prima | Dopo il rimedio |
|---|---|---|---|
| ① | record di **feedback**: `trust: Untrusted` → `Instruction` | 41 · 298 · **0 fallite** | 41 · 297 · **1 fallita** |
| ② | `passed: verdict.outcome == Pass` → `passed: false` | 41 · 298 · **0 fallite** | 41 · 297 · **1 fallita** |
| ③ | **verdetto**: `effect: Verifiable` → `Unrepeatable` | 41 · 298 · **0 fallite** | 41 · 297 · **1 fallita** |
| ④ | **verdetto**: `reason` sostituito | 41 · 298 · **0 fallite** | 41 · 297 · **1 fallita** |

⛔ **① È DI SPECIE DIVERSA DALLE ALTRE TRE, ed è I6.** Il payload di quel record è il **dettaglio
del sensore**, che ha osservato un artefatto `Untrusted`; ADR-0014 rende l'etichetta **ereditaria**,
quindi marcarlo `Instruction` fa attraversare a contenuto esterno il confine delle istruzioni —
e lo scrive nel **formato durevole**. ⚠️ **E il buco aveva una forma che si riconosce:** la riga che
asserisce **quello stesso campo** sul record del **verdetto** stava tredici righe sopra. Il banco
teneva uno dei due record che una sola funzione scrive, che è il **#96** — *una difesa protegge il
soggetto su cui è scritta e non segue il secondo che nasce* — spostato da una guardia di livello 1
a un banco.

⛔ **② HA UN DATO CHE LA SPIEGA, ed è il gotcha nuovo #98: `passed: true` NON ESISTEVA IN TUTTO IL
WORKSPACE.** Censito con `grep -rn '\.passed\|passed:' crates/ --include='*.rs'`: **ogni** sito
porta `false` — il quarto record congelato, `reconciliation.rs`, `record_shape.rs` e la sonda
negativa — e l'unico che lo **calcola** è `crates/kernel/src/sensor.rs`. Quindi la costante era
indistinguibile dal calcolo. ⛔ **E il record congelato non poteva difenderlo:** `the_frozen_records()`
costruisce i quattro record da un **letterale di banco**, quindi è un oracolo di **formato** e non
di comportamento — e il suo commento **argomenta** la scelta di `passed: false` per buone ragioni
di formato (`f4` contro `f5`), cioè la scelta giusta lì è ciò che ha lasciato l'altro valore
inesercitato ovunque.

✅ **CHIUSE ALLARGANDO LE DUE SONDE ESISTENTI E NON AGGIUNGENDONE, e il conteggio resta 298:**
mancavano **asserzioni**, non casi, e l'invarianza del numeratore è il dato. `a_passing_sensor_writes_a_verdict_and_opens_nothing`
guadagna `passed` **vero** — il primo del workspace — più `effect` e `reason`;
`a_failing_verdict_opens_a_new_step_and_carries_the_detail` guadagna `trust` e `reason` del record
di feedback. ⚠️ **Le mutazioni rimisurate dopo sono STRETTE:** ciascuna uccide **una** sonda e una
sola, che è la forma che il vicolo cieco dell'audit del 2026-08-27 prescrive — una mutazione che
ne uccide più di una non dice niente su quella che stavi provando.

⚖️ **③ e ④ sono PINZATE e non dichiarate**, ed è il confine del Task 10 del Traguardo 5: *un doc
che **afferma** un valore riceve una sonda; si lascia dichiarato solo ciò che una decisione
**aperta** può ancora cambiare* (gotcha **#73**). Qui nessuna è aperta — `reconcile` non legge
**mai** la classe di un verdetto, e non è dedotto dal commento ma **misurato** mutandone l'arm
nelle due direzioni, sopra.

⚠️ **E UNA SECONDA COSA, DI SPECIE DOCUMENTALE — `E66`.** La passata precedente aveva censito la
divergenza di data (`2026-09-01` contro i commit) **in questo file soltanto**, e deciso: le
affermazioni di **stato** si tolgono, i **verbali** di misura restano. `grep -rn '2026-09-' crates/`
rendeva **sei** righe, di cui **tre** erano affermazioni di stato mai toccate. Tolte, sostituite
dall'**evento** invece che riallineate — cura di **AUD-007** applicata a una data. ⛔ **Le due celle
di questo file restano come sono**, per la stessa decisione: sono verbali.

⚠️ **E `riferimenti.md` NON è stato toccato, deliberatamente, per la settima passata di seguito:**
le misure di questa revisione vivono qui, accanto ai controlli che difendono. La voce aperta resta
del proprietario, con una prova in più e non con una risposta.

### La TERZA passata — la prima da un sotto-agente, e il gotcha #98 riproduce lo stesso giorno

⛔ **Fatta il 2026-08-31 da un SOTTO-AGENTE, dopo che il proprietario li ha autorizzati** — ed è
la condizione che `E53` registrava come mancante da due passate. Sei rilievi, `E67`–`E72`.
⚠️ **Tutti riverificati prima di agire**, perché ciò che torna da un sotto-agente è
un'affermazione e non un fatto; e uno era **prezzato più grande del difetto**.

⛔ **`E67` — IL GOTCHA #98 ALLA SECONDA OCCORRENZA, LO STESSO GIORNO E SUL CAMPO ACCANTO.** Vale
più della sua prima misura, perché una classe che riproduce entro poche ore su un campo adiacente
non è un caso: è la forma. `spent_millis: verdict.spent.get()` sostituito con la **costante 7**
lasciava `41 · 298 · 0`, identico alla baseline, per la **stessa** ragione di `passed` — il doppio
di banco restituiva il letterale `Millis::new(7)`, quindi **un solo costo attraversava la
conversione in tutto il workspace**, e i tre letterali di banco (`frozen_bytes.rs`,
`reconciliation.rs`, `record_shape.rs`) portano anch'essi `7` senza passare dall'anello.
✅ **Chiuso col doppio a valore consegnato e due valori distinti fra le sonde** — `3` e `7`,
gotcha **#48** — e **rimisurato nelle due direzioni**: costante `7` → `41 · 297 · 1`, costante `3`
→ `41 · 297 · 1`. Nessuna delle due passa più.

⛔ **`E70` — LA TESTA DI `frozen_bytes.rs` E LA MAPPA DEI BYTE, cioè le due case che il censimento
di `E56` non ha raggiunto.** Misurato sui file veri e non dedotto:

| | |
|---|---|
| record congelati | **4** — `intent` 21, `outcome` 21, `note` 21, `verdict` **27** byte |
| varianti degli enum `index_only` | **9** — `RecordKind` 4, `EffectClass` 3, `Trust` 2 |
| coppie di **pari arità** | differiscono solo ai byte 4, 5, 6 — **regge** |
| coppie che includono il verdetto | differiscono anche al byte **3** (`85` → `86`) e in tutta la coda |

⛔ **E il fatto che pesa non è il numero: è che le SONDE di quel file erano già state corrette e il
capoverso che le giustifica no.** Nello stesso commit due sonde furono rinominate scrivendo la
regola *«a name that counts its own subjects is a count like any other»*, e il codice porta già il
salto delle coppie di arità diversa con la propria spiegazione. ✅ **Numerali tolti e non
riallineati**, sostituiti dalla **regola** e dal rimando a `the_frozen_records()`, il cui tipo di
ritorno porta il conto che **il compilatore** controlla.

⚠️ **`E72` — un'annotazione dichiarata portante da due documenti, e inerte.** Tolto
`#[cbor(default)]` da `detail`, il workspace resta `41 · 298 · 0`, **compresa** la direzione
all'indietro: i `.cbor` da 21 byte decodificano ancora a `None`, perché `minicbor` legge già un
`Option` mancante come `None`. ⚖️ **Corretta la REGOLA, non aggiunta una sonda** — l'annotazione
resta come cintura e bretelle, ma i due doc ora dicono **quale metà difende**. Il confronto che lo
rende visibile: `#[cbor(with = "minicbor::bytes")]` su `payload` è dichiarato portante **ed è
pinzato** da una sonda propria.

⚠️ **E `E69` È LA LEZIONE SUL DELEGARE: il rapporto prezzava DUE nomi scambiati, ed era sbagliato
UNO.** La coppia di `Note`, che porta i nomi giusti, mappa scenario→nome **allo stesso modo**,
quindi `a_verdict_does_not_put_a_step_in_doubt` è corretto. Gotcha **#65** applicato al rapporto di
un sotto-agente, colto leggendo la **coppia gemella** invece del rapporto — e il rinomino è stato
censito sulle **tre** case del nome vecchio, fra cui `reconcile.rs`, che lo cita per nome.

📌 **Baseline invariata a passata chiusa: `GATE GREEN`, 41 bersagli, 298 passate, 0 fallite, 2
ignorate.** ⛔ **L'invarianza del numeratore è il dato di tutte e tre le passate:** i rimedi hanno
allargato sonde e corretto prose, non aggiunto casi — a mancare erano **asserzioni**, e il
conteggio dei test non le vede.

### La QUARTA passata — un mutante vivo, e QUATTRO censimenti che i rimedi precedenti non avevano chiuso

⛔ **Fatta il 2026-09-01 da un sotto-agente fresco, sul perimetro `git diff 4e4b725^..HEAD --
crates/`** — 16 file, `+1047/−75` — con l'ordine di battere **per prime** le affermazioni
introdotte da `eba6344` e `16365f2`. Sei rilievi, `E73`–`E78`.
⚠️ **Tutti riverificati dal coordinatore PRIMA di rimediare**, perché ciò che torna da un
sotto-agente è un'affermazione e non un fatto — e stavolta **tutti e sei erano veri**, il che è
esso stesso un dato dopo che la terza passata ne aveva prezzato uno più grande del difetto.

⛔ **`E73` — L'UNICO MUTANTE VIVO, ED È IL CAMPO CHE IL COMPITO 5 HA INTRODOTTO.** `run_the_ring`
scrive **due** record; dopo `E65` il banco teneva cinque campi su sei del secondo, e il sesto —
`detail` — nessuno. ✅ **Misurato nelle due direzioni, non dedotto:**

| Direzione | Comando | Esito |
|---|---|---|
| **non deve scattare** | codice corretto, `cargo test --locked --workspace --no-fail-fast` | **41 · 298 · 0 · 2** |
| il mutante **prima** del rimedio | `detail: Some(Detail::Verdict(VerdictDetail { passed: true, spent_millis: 0 }))` in `crates/kernel/src/sensor.rs` | **41 · 298 · 0 · 2** — identico, **vivo** |
| **deve scattare** | la stessa mutazione **col rimedio** | **41 · 297 · 1 · 2**, e muore `a_failing_verdict_opens_a_new_step_and_carries_the_detail` **e nessun'altra** |

⚠️ **Gli altri due siti del campo — `crates/kernel/src/boundary.rs` e
`crates/kernel/src/arbiter/mod.rs` — portano lo stesso mutante vivo, misurato, e NON sono tre
rilievi:** sono aggiunte meccaniche imposte dall'arrivo del campo, non decisioni, e il sito che
**possiede** la coppia `kind`/`Detail` è uno solo. Gotcha **#65**.

⛔ **`E75` — UN LETTERALE DICHIARATO INERTE CHE PORTA IL KILLER DELLA SONDA GEMELLA, e la misura
è a TRE passate perché due non bastano a distinguerlo.** L'aiutante `a_verdict()` di
`crates/kernel/tests/reconciliation.rs` dichiarava `EffectClass::Verifiable` *«INERT HERE»*,
mentre il doc del fratello `a_note()` venti righe sopra dice che la classe diversa è *«tutto ciò
che rende non vacue le due sonde sotto»*.

| Che cosa | Esito |
|---|---|
| aiutante armonizzato a `Idempotent`, codice **corretto** | **41 · 298 · 0 · 2** — verde: *il commento è vero del codice corretto*, ed è ciò che lo rende ingannevole |
| mutazione `RecordKind::Verdict => enter(&mut open, step, resolution_of(body.effect))`, aiutante **intatto** | **41 · 296 · 2 · 2** — muoiono **entrambe** le sonde |
| la **stessa** mutazione con l'aiutante armonizzato | **41 · 297 · 1 · 2** — ne muore **una sola** |

⛔ **Quindi `a_verdict_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them` — la sonda la
cui unica ragione d'esistere è *«l'altra metà»* — smette di sparare, e niente diventa rosso per
dirlo.** 📌 È il gotcha **#98** in una terza forma: un letterale scelto per un campo può lasciare
**un'altra sonda** inesercitata, non solo un campo accanto.

⛔ **E LA MESSE VERA SONO I QUATTRO CENSIMENTI INCOMPLETI, che è la classe che questo ciclo non
riesce a chiudere.** `E74` — `E72` aveva corretto la regola su `#[cbor(default)]` in **due** case
su quattro, e una delle mancanti è la costante **`FORMAT_CHANGED`**, cioè la prosa che l'oracolo
stampa **quando il formato durevole si muove**. `E76` — `E70` aveva chiuso la **testa** di
`frozen_bytes.rs` e la mappa, non i **corpi**: *«three artefacts … the third»* dove
`the_frozen_records()` ha tipo di ritorno `[…; 4]`. `E78` — il letterale `85`/`86` viveva in
**cinque** case, e il terzo giro **ne aveva aggiunte due** a una cifra che ne aveva già tre.
`E77` — la rinomina di `16365f2` ha lasciato una riga di commento a **129 caratteri** dove le
vicine stanno a ≤ 83, e nessun controllo lo vede: `rustfmt` non tocca i commenti e §7.4.3 non dà
voce a `clippy` nel cancello.

✅ **LA CONTROMISURA, MISURATA SU QUESTA ONDATA: cinque rimedi su sei TOLGONO righe**, e il solo
che aggiunge è **una** asserzione dentro una sonda che esiste già. È il gotcha **#76** applicato
**prima** invece che all'ultima ondata.
⛔ **E `E78` è stato chiuso su TRE case e non sulle due nuove:** correggerne due lasciando la terza
a mentire è la forma che **AUD-018** e **AUD-060** hanno già pagato. ⚖️ **Le due che restano hanno
ciascuna la propria ragione:** la riga **89** di `record_v1.map` è la colonna **hex che il banco
ricostruisce**, e quella di `record.rs` sta dentro il **verbale datato** di **P-15** — la **55ª**
misura dice che lì una cifra regge.

📌 **Baseline invariata a passata chiusa: `GATE GREEN`, 41 bersagli, 298 passate, 0 fallite, 2
ignorate**, `cargo fmt --all --check` exit 0. ⛔ **L'invarianza del numeratore è ora il dato di
tutte e QUATTRO le passate**, e dice la stessa cosa ogni volta: a mancare sono **asserzioni** e
**censimenti**, non casi — e il conteggio dei test non vede né gli uni né gli altri.
⚠️ **Fine-riga conservati e rimisurati file per file** (`tr -cd '\r' | wc -c`), censimento
`git ls-files --eol` immutato a **quattro** `i/crlf`; ogni mutazione revocata da copia
byte-esatta, `git diff` a zero a campagna chiusa.

### La QUINTA passata — la domanda cambia, e tutti i rilievi vengono dall'ondata precedente

⛔ **Fatta il 2026-09-01 da un sotto-agente fresco, e la NOVITÀ non è il perimetro ma la DOMANDA.**
Invece di *«trova difetti»*, il primo passo dettato è stato: prendere i **sei** rimedi di `bd37d59`
uno per uno e rispondere **col `grep`** a *«ha chiuso la CLASSE, o il punto dove l'ho trovato?»*.
📌 **È la domanda che i quattro giri precedenti non avevano**, ed è quella che ha reso i rilievi.

| Rimedio del quarto giro | Censimento rifatto | Verdetto |
|---|---|---|
| `E73` — l'asserzione su `detail` | i siti di produzione che scrivono `detail: None` sono **tre**, e il rimedio ne aveva chiuso **uno** | ⛔ **occorrenza** → `E79` |
| `E74` — la regola su `#[cbor(default)]` | `grep -rn 'cbor(default)' crates/` → otto occorrenze, lette **intere**: nessuna riafferma l'obbligo | ✅ **classe chiusa** |
| `E75` — il letterale di `a_verdict()` | il fratello `a_note()` misurato in due passate: la sua dichiarazione **regge** | ✅ **classe chiusa** |
| `E76` — i numerali nei corpi | la **testa** dello stesso file conta ancora *«three `.cbor` files»*, e la mappa **una** produzione | ⛔ **occorrenza** → `E80`, `E81` |
| `E77` — l'a-capo rotto | la riga più lunga fra quelle **aggiunte** dal perimetro è **104**, contro le ~101–103 diffuse in `crates/` | ✅ **classe chiusa** |
| `E78` — il letterale `85`/`86` | censite al padre `d4c906d`: le case erano **sei** e non cinque, quindi ne restavano **tre** | ⛔ **occorrenza** → `E82` |

⛔ **`E79` È IL RILIEVO CARO, ED È L'UNICO CON MUTANTI VIVI.** Il rimedio di `E73` aveva chiuso
`crates/kernel/src/sensor.rs` e lasciato `crates/kernel/src/boundary.rs` (`Untrusted::promote`) e
`crates/kernel/src/arbiter/mod.rs` (`transition_record`).

| Sito | `detail: None` → `Some(Detail::Verdict(..))` | Col rimedio |
|---|---|---|
| `crates/kernel/src/sensor.rs` (chiuso da `E73`) | — | `41 · 297 · 1` — muore `a_failing_verdict_opens_a_new_step_and_carries_the_detail` |
| `crates/kernel/src/boundary.rs` | **`41 · 298 · 0 · 2`** — identico alla baseline, **vivo** | `41 · 297 · 1` — muore `the_promoted_content_is_the_payload_and_it_is_labelled_untrusted` |
| `crates/kernel/src/arbiter/mod.rs` | **`41 · 298 · 0 · 2`** — identico alla baseline, **vivo** | `41 · 297 · 1` — muore `a_transition_names_the_policy_it_moves_to` |

⚠️ **Ogni mutazione applicata una per volta e revocata da copia byte-esatta prima della
successiva**, e in ciascuna direzione muore **una sonda sola**, nominata. Conteggio invariato a
**298**: sono asserzioni dentro sonde che esistono già, non casi nuovi.
⛔ **E l'errata di `E73` li NOMINAVA prezzandoli male** — *«aggiunte meccaniche, non tre rilievi»*.
La prima metà era vera, la seconda no: l'argomento di `E73` vale **alla lettera** anche per un
`Note` e per una transizione di policy. È la radice **R1** dentro il rimedio, la seconda volta in
due ondate.

⛔ **E LA CURA DI CLASSE NON È STATA INVENTATA, ed è scritto perché.** Una guardia di **livello 1**
sulla coppia `kind`/`Detail` — campi privati e un costruttore per specie — è il finding
**AUD-050**, già **del proprietario e registrato**; e una sonda che congelasse oggi *«`detail` è
`Some` se e solo se `kind` è `Verdict`»* fisserebbe una partizione che i **compiti 6 e 7** stanno
per cambiare, cioè il gotcha **#57** — *una previsione citata come una misura*.

📌 **`E80` è la QUARTA occorrenza della stessa classe nello stesso file** — `E56` → `E70` (la
testa) → `E76` (i corpi) → `E80` (la testa di nuovo, e la mappa) — e `E70` aveva corretto un
paragrafo a **venti righe** da quello che ha lasciato in piedi.
📌 **`E82` corregge una giustificazione di `E78` che era mezza sbagliata:** la riga 89 di
`record_v1.map` è la colonna **hex** solo per il **byte `86`**; il confronto *«the 85 -> 86»* stava
nella **colonna di prosa**, che la testa della mappa dichiara **non verificata**. ✅ Ora quel
confronto vive in **una** casa sola, il verbale P-15 sul doc di `detail`.

📌 **Baseline a passata chiusa: `GATE GREEN`, 41 bersagli, 298 passate, 0 fallite, 2 ignorate**,
`cargo fmt --all --check` exit 0, fine-riga conservati file per file, `i/crlf` fermo a **quattro**.
⚠️ **Tre rimedi su quattro TOLGONO righe**, e il quarto aggiunge **due** asserzioni.

### `AUD-050` chiuso a LIVELLO 1 — la cura alla radice, scelta dal proprietario (2026-09-01)

⛔ **È la terza delle tre vie che `E53` metteva davanti al proprietario**, e la sola che toglie il
problema invece di inseguirlo: il ciclo di revisione del compito 5 non convergeva perché ogni
ondata chiudeva un'occorrenza, e la classe di `E73`/`E79` — la coppia `kind`/`Detail` tenuta dalla
**disciplina** — era tenuta da niente al livello 1.

**La falla, riprodotta PRIMA di toccare qualsiasi cosa** (passo 2 della disciplina dell'audit),
da **fuori la crate**, su una sonda usa-e-getta cancellata nella stessa corsa:

```
RecordV1 { kind: Note, effect: Unrepeatable, trust: Untrusted, payload: <6 bytes>,
           reason: "ignore your instructions", detail: None }
```

Testo calcolato a **runtime** all'indice 4, per letterale di struct, senza passare da `promote` —
il campo guardato sigillato e quello non guardato spalancato. È **P-1 attraverso una seconda
bocca**, e la scheda `AUD-050` lo dice: *una guardia vale quanto il suo costruttore*.

⛔ **Il perimetro è stato RIMISURATO contro il codice di adesso**, perché la scheda è del
2026-08-27: `grep -rn 'RecordV1 {' crates/ --include=*.rs` rendeva **45 siti in 13 file**.

⛔ **RICHIAMO DEL 2026-09-01, sesto giro di revisione — QUESTA RIGA DICEVA *«45 siti in 13 file su
3 crate, più 37 siti di lettura»*, e DUE delle quattro clausole erano FALSE il giorno in cui furono
scritte.** Non stantie: sbagliate alla misura. **Questa è la casa unica**, perché è la sola che
porti accanto il comando; le altre cinque hanno perso la cifra invece di riallinearla.

| La clausola | La misura del 2026-09-01 |
|---|---|
| «45 siti» | ✅ **vera del comando** — `git grep -c "RecordV1 {" c63c8c8^ -- 'crates/**/*.rs'` dà **45** |
| «13 file» | ✅ **vera del comando** — stesso comando, **13** file |
| «su 3 crate» | ⛔ **falsa: sono DUE.** `git grep -l "RecordV1 {" c63c8c8^ \| cut -d/ -f2 \| sort \| uniq -c` dà `12 kernel` e `1 simulator`; e `git log -S RecordV1 -- crates/platform crates/daemon crates/secrets` non rende **nessun commit** — quelle tre crate non hanno **mai** nominato `RecordV1` |
| «più 37 siti di lettura» | ⛔ **non riproducibile da nessuna forma del comando.** Oggi gli accessori sono **40**; al padre le letture di campo, contate con lo stesso regex, sono **51**. Il 37 non si rifà, quindi **esce** |

⚠️ **E la cifra vera del comando non è la cifra della frase, che è la parte da ricordare:** dei 45
hit solo **31** sono siti di **costruzione**, in **11** file — gli altri 14 sono la definizione,
l'`impl Debug`, un tipo di ritorno, la stringa di formato `RecordV1 {{`, due `assert_eq!` di
`record_shape.rs` e otto righe di commento. Scrivere *«45 siti di costruzione»* dove il comando
dice *«45 hit»* è la radice **R1** in miniatura: il numero è giusto, il **sostantivo** no.

| La forma | |
|---|---|
| i sei campi | **privati** |
| la lettura | sei **accessori** |
| la costruzione | **un costruttore per specie** — `intent`, `outcome`, `note`, `verdict` — ciascuno con `reason: &'static str` |
| l'unico che costruisce | un `of` **privato**, sul precedente di `Arbiter::issue` per `Grant` (§5.6) |

✅ **E la seconda metà chiude la classe di `E79` senza congelare una partizione:** la coppia
sbagliata non è *rifiutata*, è **impronunciabile** — `kind` non è parametro di niente. ⚖️ **Ed è la
ragione per cui questa forma è stata preferita a una sonda:** una sonda che fissasse oggi *«`detail`
è `Some` se e solo se `kind` è `Verdict`»* congelerebbe una partizione che i **compiti 6 e 7**
stanno per cambiare — gotcha **#57** — mentre una specie nuova porta il **proprio** costruttore, che
è additivo.

⛔ **IL CASO NUOVO È IL 35°, E LA SUA NON-VACUITÀ È MISURATA NELLE DUE DIREZIONI.**

| Direzione | Esito |
|---|---|
| **deve scattare** | `record_reason_is_not_runtime_text.rs` → `error[E0597]: `outside` does not live long enough`, `.stderr` **scritto a mano** dall'uscita vera |
| **deve smettere di scattare** | allargati i cinque `reason: &'static str` a `&str` → il caso va **`error`**, cioè trybuild dice *«expected compilation to fail»* — la forma forte del gotcha **#42**, che `TRYBUILD=overwrite` non può disarmare |
| ⛔ **e non è una copia** | sotto quella **stessa** mutazione `promote_reason_is_not_runtime_text.rs` resta **`ok`**: i due casi tengono strade **diverse**, misurato invece che argomentato (gotcha #49 evitato con una misura) |

⛔ **E UN CASO ESISTENTE HA DOVUTO MUOVERSI, NON ESSERE RIALLINEATO.**
`record_without_trust_label.rs` lasciava `trust` fuori da un **letterale** e attendeva `E0063`;
col letterale rifiutato scattava per una **seconda ragione** — *cannot construct with struct
literal syntax due to private fields* — e il suo stesso doc scrive che *«un caso negativo che
scatta per una seconda ragione smette di provare la prima»*. ✅ Riscritto sulla strada che esiste:
l'**arità** del costruttore, `error[E0061]`, la forma che `reading_without_a_receipt.rs` usa già.

✅ **I BYTE CONGELATI NON SI SONO MOSSI, ed era la cosa da controllare per prima:**
`every_frozen_record_still_encodes_to_its_frozen_bytes` e
`the_map_lists_the_bytes_that_are_really_frozen` verdi per tutto il cambiamento, e
`git status --porcelain crates/kernel/tests/frozen/` **vuoto**. ⚠️ **L'aiutante di quel banco
resta UNO**, com'era: la specie arriva come **chiusura**, quindi `FROZEN_PAYLOAD` e
`FROZEN_REASON` continuano a vivere in un posto solo — che è la proprietà per cui quell'aiutante
esiste.

⛔ **E LA TRAPPOLA DEI FINE-RIGA È SCATTATA, MISURATA INVECE CHE TEMUTA — quarta forma.**
`cargo fmt --all --check` era rosso su cinque file dopo il cambiamento; `rustfmt` lanciato **file
per file** (mai `cargo fmt`, voce `E41`) li ha portati da **`CR` = righe a `CR` = 0**. Ripristinati
in CRLF e **rimisurati uno per uno**; censimento `git ls-files --eol` **immutato a quattro**
`i/crlf`.

⚠️ **CINQUE case nel sorgente dichiaravano la falla APERTA, censite col `grep` e guardate una per
una** (#70): il doc di `Detail`, i due paragrafi del `Debug` di `record.rs`, la voce **A3** di
`boundary.rs` e il commento di `record_shape.rs`. ⛔ **I verbali datati NON sono riscritti** — 55ª
misura: il richiamo del 2026-08-28 resta e riceve il proprio.

⚠️ **VOCE APERTA REGISTRATA E NON PRESA, ed è del proprietario:** il 35° caso **non ha una riga di
catalogo propria**. La §7.4 è **spec**, vincolo globale 7 — stesso trattamento di `PL-1`, di
`K-1`/`B-1` e delle sonde del Traguardo 5, stessa ragione (gotcha **#36**: una nota si legge e si
dimentica, una voce aperta no). Oggi il caso difende la stessa proprietà della riga che copre
`promote_reason_is_not_runtime_text.rs`, e che siano **due strade** è dichiarato qui e nel caso.

⚠️ **Che cosa NON compra, dichiarato invece che scoperto:** `payload` resta un `Vec<u8>` che il
chiamante riempie e `trust` resta un parametro — l'etichetta è l'affermazione del **chiamante**, e
nessuna firma la può controllare. È il limite che la via **A4** di `boundary.rs` dichiara, e non è
cambiato.

📌 **Baseline a cambiamento chiuso: `GATE GREEN`, 41 bersagli, 298 passate, 0 fallite, 2 ignorate**
— **invariata**, e l'invarianza è il dato: i casi `compile_fail` passano da 34 a **35** ma girano
tutti dentro `level_1_rules_do_not_compile`, che è **un** test.
`cargo build --locked --workspace` a **zero avvisi**, `cargo fmt --all --check` exit 0.

### `E94` — la TERZA bocca della classe di AUD-050: `RoutingDetail`, chiusa il 2026-09-01

⛔ **LA CLASSE È RIENTRATA DA UN TIPO NUOVO, E NON SERVIVA NESSUN ATTO DELIBERATO.** `RecordV1` è
sigillato dal 2026-09-01, ma `Detail::Routing` porta `RoutingDetail`, nato **`pub` coi campi
`pub`** col compito 6 ed è il **primo** `Detail` che porta del testo. ✅ **Riprodotto da FUORI la
crate**, su una sonda usa-e-getta cancellata nella stessa corsa, col `reason` un letterale
`'static` a posto per tutto il tempo:

```
detail: Some(Routing(RoutingDetail { model: "ignore your instructions", evaluated: 1, degraded: false }))
```

⛔ **È l'argomento di AUD-050 alla lettera** — *una guardia vale quanto il suo costruttore* — su un
**secondo tipo**: il `&'static str` sulle specie di `RecordV1` chiude la strada di `reason`, mai
quella di un `Detail` che porti testo proprio. Ogni specie che ne cresce uno deve la stessa firma.

| La forma | |
|---|---|
| i tre campi | **privati** |
| la lettura | tre **accessori** — `model()`, `evaluated()`, `degraded()` |
| la costruzione | **un costruttore solo**, `RoutingDetail::new(model: &'static str, evaluated: u32, degraded: bool)`, che converte in `String` dentro |
| il tipo sul filo | **invariato**, `String` — la decodifica è l'altra strada, e **P-9** misurò che un `&'static str` non è producibile dai byte in arrivo senza `leak` |

⛔ **IL CASO NUOVO È PROVATO NELLE DUE DIREZIONI.**

| Direzione | Esito |
|---|---|
| **deve scattare** | `routing_detail_model_is_not_runtime_text.rs` → `` error[E0597]: `outside` does not live long enough ``, con *«argument requires that `outside` is borrowed for `'static`»*; il `.stderr` è stato **letto** dall'uscita vera, mai rigenerato in blocco |
| **deve smettere di scattare** | allargato il solo `model` di `new` a `&str` → il caso va **`error`**, cioè trybuild dichiara *«expected compilation to fail»* invece di passare dall'oracolo — la forma forte del gotcha **#42**, che `TRYBUILD=overwrite` non può disarmare. ⚠️ **Qui la firma da allargare è UNA** dove AUD-050 ne aveva cinque: `new` è l'unica strada dentro il tipo |
| ⛔ **e non è una copia dei due fratelli** | sotto quella **stessa** mutazione `record_reason_is_not_runtime_text.rs` e `promote_reason_is_not_runtime_text.rs` restano **`ok`**: i tre casi tengono strade **diverse**, misurato invece che argomentato |

⛔ **E LA SCHEDA `E94` PREZZAVA UN SITO IN MENO — gotcha #65 applicato a una voce d'errata.**
Elencava `gateway::dispatch`, `frozen_bytes.rs` e `record_shape.rs`; i letterali erano **quattro**,
e il quarto è `crates/kernel/tests/reconciliation.rs`. Trovato col censimento — `grep -rn
RoutingDetail crates/` — e non dedotto dall'elenco della scheda, che è precisamente ciò che quel
gotcha prescrive.

✅ **I BYTE CONGELATI NON SI SONO MOSSI, ed era la cosa da controllare per prima:** le sonde di
`frozen_bytes.rs` verdi per tutto il cambiamento, e `git status --porcelain
crates/kernel/tests/frozen/` **vuoto**. Il letterale congelato passa da `RoutingDetail { model:
String::from("frozen"), .. }` a `RoutingDetail::new("frozen", 2, true)`, che sul filo è la stessa
cosa — il precedente è `E83`.

✅ **E UN AVVISO HA CONFERMATO IL RIMEDIO INVECE DI ESSERE SILENZIATO:** tolta la conversione da
`gateway`, `use alloc::string::String` è diventato **inutilizzato** — cioè la conversione è
davvero migrata in un posto solo, che è ciò che il doc di `RoutingDetail` prometteva **e non
aveva**. Tolto l'import, `cargo build --locked --workspace` a **zero avvisi**.

⚠️ **`VerdictDetail` NON È SIGILLATA, e la differenza è MISURATA invece che dedotta:** porta un
`bool` e un `u64`, quindi nessun testo di runtime può entrarci. Sigillarla costerebbe un
costruttore e due accessori per chiudere una bocca **che non esiste**. ⛔ L'asimmetria è scritta
accanto ai due tipi, sul precedente di quella `#[cbor(array)]` della stessa cartella, perché
nessuno la «uniformi» credendo di rimediare.

⚠️ **VOCE APERTA REGISTRATA E NON PRESA, ed è del proprietario:** il caso nuovo **non ha una riga
di catalogo propria**. La §7.4 è **spec**, vincolo globale 7 — stesso trattamento del caso di
AUD-050 qui sopra, di `PL-1` e di `K-1`/`B-1`, stessa ragione (gotcha **#36**: una nota si legge e
si dimentica, una voce aperta no).

📌 **Baseline a cambiamento chiuso: `GATE GREEN`, 42 bersagli, 307 passate, 0 fallite, 2**
**ignorate** — **invariata**, e l'invarianza è il dato per la stessa ragione del blocco qui sopra:
un caso `compile_fail` in più non aggiunge un `#[test]`, gira dentro `level_1_rules_do_not_compile`
insieme a tutti gli altri.

### Il compito 7 del Traguardo 6 — la tripla del permesso, e la proiezione del giornale (2026-09-01)

⛔ **Il permesso è una TRIPLA** — `(strumento × risorsa × operazione)` — e *«quali permessi sono
attivi ora»* si risponde **rileggendo il giornale**, senza secondo archivio. Il banco nuovo è
`crates/kernel/tests/permission_triple.rs`; il meccanismo è `crates/kernel/src/permission.rs`.

| Artefatto | Che cosa lo esercita |
|---|---|
| `permission::grant` — la sua **via d'errore** | `a_journal_that_refuses_the_note_makes_the_grant_fail` |
| `permission::grant` — ciò che **scrive**: `kind`, `effect`, `trust`, `payload`, `reason`, `detail` | ⚠️ **riga aggiunta il 2026-09-01**: `grant_writes_every_field_it_says_it_writes`, che rilegge il record **dal giornale**. Prima nulla rileggeva quel record e quattro campi su sei erano scoperti |
| `permission::is_granted` | le sonde della tripla — una per componente — più `a_granted_triple_is_granted`, più il giornale vuoto, più le vie d'errore del §4 elencate sotto. ⚠️ **Nessun conteggio qui dal 2026-09-01, nono giro:** la riga ne portava due, ed è l'elenco che la cella per-file dello stesso commit vieta — si contano **sul binario**, `cargo test --locked -p kernel --test permission_triple` (`E128`) |
| `RecordKind::Permission` (indice **5**) | `crates/kernel/tests/frozen_bytes.rs` — il **sesto** record congelato |
| `Detail::Permission` (indice **2**) | idem: è l'unico record congelato che lo porta |
| `PermissionDetail` — gli indici **0** e **1** dei due nomi | ⚠️ **riga aggiunta il 2026-09-01**: `the_two_names_of_a_permission_do_not_share_one_offset_and_its_mirror`, nello stesso file. Il record congelato porta `"frozen"` due volte e **non può** vederli scambiati |
| l'arm vuoto in `reconcile` | `a_permission_does_not_put_a_step_in_doubt` e il suo gemello, in `reconciliation.rs` |
| `PermissionDetail` sigillata | **due** casi `compile_fail`, uno per parametro |

#### Le mutazioni, col proprio esito MISURATO

⛔ **RICHIAMO DEL 2026-09-01 — QUESTA SEZIONE RIVENDICAVA UNA DOMANDA CHE NON AVEVA FATTO.** Qui
stava *«Dieci mutanti su `permission.rs`, uno per volta … **Nessuno è sopravvissuto**, ed è la
domanda che il difetto `E101` del compito 6 ha insegnato a fare: per ogni via, quale controllo la
esercita?»*. I dieci sono veri e stanno qui sotto, **rimisurati**; la **domanda** no. Tutti e dieci
attaccavano la **domanda** — `is_granted` — e **nessuno** la **registrazione**: né i campi che
`grant` scrive, né gli indici del record che ne esce. La settima passata di revisione l'ha fatta
davvero e ha trovato **sei** risposte «nessuno», cinque su `permission.rs` e una su `record.rs`.
Chi leggeva questa riga ne concludeva che `permission.rs` fosse coperto.

⛔ **La campagna è stata RIFATTA il 2026-09-01**, uno per volta, ciascuno applicato da solo e
revocato da una **copia byte-esatta** presa prima. La baseline contro cui va letta la colonna «Chi
muore» è **43 bersagli, 323 passate, 0 fallite, 2 ignorate**.

⚠️ **E UN GOTCHA DELLO STRUMENTO, misurato qui e non altrove:** revocare la mutazione con
`cp -p` — che **conserva l'mtime** — lascia `cargo` convinto che il sorgente sia fresco, e la
misura successiva gira contro l'**oggetto mutato**. Costa un rosso che sembra un difetto del
prodotto e non lo è. Si revoca con `cp` semplice, o si fa `touch` dopo.

**① I DIECI SULLA DOMANDA — la prima campagna, rimisurata e confermata riga per riga:**

| Mutante | Chi muore |
|---|---|
| il confronto su `tool` cade | `a_different_TOOL_is_not_covered` **e** la sonda della specie diversa — **due** |
| il confronto su `resource` cade | `a_different_RESOURCE_is_not_covered` |
| il confronto su `write` cade | `a_different_OPERATION_is_not_covered` |
| `grant` inghiotte il rifiuto della nota | `a_journal_that_refuses_the_note_makes_the_grant_fail` |
| il record illeggibile viene **saltato** invece che propagato | `a_record_that_will_not_decode_is_not_an_answer_of_false` |
| `replay` che rifiuta diventa `Ok(false)` | `a_journal_that_will_not_replay_is_not_an_answer_of_false` |
| il `Permission` senza `detail` viene **saltato** | `a_permission_record_without_its_detail_is_not_an_answer_of_false` |
| il controllo sulla **specie** cade | **cinque** sonde |
| `is_granted` risponde sempre `true` | **otto** sonde |
| `is_granted` risponde sempre `false` | **cinque** sonde |

**② I SEI SULLA REGISTRAZIONE — che la prima campagna non aveva cercato.** Ciascuno misurato
**prima** del rimedio, da solo, sul workspace intero:

| Mutante | Prima | Dopo |
|---|---|---|
| `grant` scrive `Trust::Untrusted` | **vivo** — 43 / 321 / 0 / 2, la baseline di allora, cifra per cifra | `grant_writes_every_field_it_says_it_writes` |
| `grant` scrive `EffectClass::Idempotent` | **vivo**, stessa cifra | idem |
| `grant` scrive una `reason` vuota | **vivo**, stessa cifra | idem |
| `grant` scrive un byte di `payload` | **vivo**, stessa cifra | idem |
| `Operation::Write` registrata come `write: false` | **vivo**, stessa cifra — ⛔ tutte le chiamate a `grant` del workspace passavano `Operation::Read` (⚠️ *«e quattro»* è uscito il 2026-09-01, nono giro: erano **sei** a `8d45eea`, tutte `READ_A` — `E128`), quindi la via `Write → true` **non era percorsa da niente** | idem |
| in `record.rs`, gli indici `#[n(0)]` di `tool` e `#[n(1)]` di `resource` **scambiati** | **vivo**, stessa cifra — il sesto record congelato porta `"frozen"` **due volte**, quindi lo scambio non muove un byte | `the_two_names_of_a_permission_do_not_share_one_offset_and_its_mirror` |

⛔ **Il difetto di prodotto dietro i primi cinque è `E97` VERBATIM, sulla funzione sorella:** il
commit `9441e6d` aveva chiuso esattamente questo su `gateway::dispatch` **un'ora e quaranta prima**
di `8d45eea`, e la forma del rimedio è stata **riusata** da `crates/kernel/tests/gateway_decisor.rs`
invece che inventata. `grant` scrive **sei** campi e `is_granted` ne legge **due**: gli altri
quattro erano affermazioni che nessuno teneva — e i doc di `grant` le fanno per iscritto
(*«the label is TRUE rather than decorative»*, *«filled with what is TRUE of this record»*), con
`crates/kernel/tests/reconciliation.rs` che ne fa una **terza** su `grant` da un altro file.

⚠️ **LE DUE DIREZIONI, perché la seconda si dimentica (§7.1.1, regola 3):**

| Il controllo | Scatta dove deve | E non scatta dove non deve |
|---|---|---|
| il flag `write` che `grant` **registra** | appiattito a `false` → muore la sonda nuova, e **lei sola** | appiattito a `true` → muoiono **tre** sonde, fra cui `a_different_OPERATION_is_not_covered`: l'altro verso era già tenuto |
| i due nomi sul **filo** | indici scambiati → muore la sonda nuova, e **lei sola** | sul repository non mutato è **verde**, e la campagna ① non la fa scattare mai |
| i due nomi come **campi del tipo** | argomenti di `PermissionDetail::new` scambiati dentro `grant` → muoiono **tre** sonde di `permission_triple.rs` | ⛔ e lo **scambio degli indici** non ne fa cadere **nessuna**: è la prova che quel banco tiene i **campi** e non le **posizioni**, perché codifica e decodifica attraverso la **stessa** derive |

⚠️ **E l'asimmetria che il pre-controllo aveva ASSERITO è stata VERIFICATA invece che creduta**,
ristretta alle **quattro** sonde dettate: `true` a tutto ne passa **una**
(`a_granted_triple_is_granted`), `false` a tutto ne passa **tre**. È esattamente la ragione per cui
`nothing_is_granted_on_an_empty_journal` esiste. ⛔ **Ma va detta anche la metà scomoda:** fra i
dieci mutanti della tabella ① **nessuno è ucciso da quella sola sonda** — la uccide `true a tutto`,
insieme ad altre sette. Resta la sola sonda che parte da un giornale **vuoto**, e quella proprietà
— *nessun record ⇒ niente concesso* — non è tenuta da nient'altro.

#### Le due direzioni dei casi `compile_fail`, e perché sono DUE

⛔ **`PermissionDetail::new` ha DUE parametri di testo, quindi due strade.** Un caso solo che li
nominasse entrambi resterebbe `error` allargandone **uno qualsiasi**: è la forma **debole**, quella
che `E83` aveva prodotto per sbaglio e il sesto giro di revisione ha ripreso. Misurato il
2026-09-01, allargando un parametro per volta:

| Allargato a `&str` | `..._tool_...` | `..._resource_...` | i tre fratelli (`routing_detail`, `record_reason`, `promote_reason`) |
|---|---|---|---|
| `tool` | **`error`** | `ok` | `ok` |
| `resource` | `ok` | **`error`** | `ok` |

✅ **Ogni caso tiene una strada propria**, e nessuno dei due fa cadere i fratelli: è ciò che prova
che difendono strade **diverse** e non la stessa due volte.

#### Le quattro vie del §4 — tutte raggiungibili, tutte con una sonda

| | La via | Esito |
|---|---|---|
| ① | il giornale rifiuta la nota | **sonda**, e ne ha **due**: quella di `grant` (`note` rifiutata) e quella di `is_granted` (`replay` rifiutata) |
| ② | il record **illeggibile** | **sonda** — byte scritti attraverso la porta, che non convalida nulla (strada A4 di `boundary`) |
| ③ | un record di **specie diversa** | **sonda**, con un giornale misto che porta anche un record `Routing` — cioè una specie che un `detail` ce l'ha |
| ④ | un `Permission` **senza** `detail` | ⚠️ **RAGGIUNGIBILE, e misurato invece che dichiarato impossibile.** Impronunciabile **in sorgente** (`RecordV1::permission` prende il `detail` per valore), costruibile **dai byte** con un solo byte cambiato: `Ok(V1(RecordV1 { kind: Permission, .. detail: None }))`. Ha la sua **sonda** |

#### Le decisioni prese, e ciò che costano

| | |
|---|---|
| `is_granted` restituisce un **errore proprio** che compone `JournalError` e `RecordError` | `JournalError` **non** ha guadagnato varianti — `git diff crates/kernel/src/ports/journal.rs` è **vuoto** |
| ⛔ è il **primo errore di `kernel` con un carico** | e `framing.rs` diceva che nessuno nel repository ne porta. ⚠️ **Misurato con `git log -S`:** quella frase è del 2026-08-31 e `platform::journal::OpenError` porta `io::Error` dal **2026-08-10** — era falsa **quando fu scritta**. Corretta con un richiamo datato, non risposta sotto |
| `write: bool` sul filo e non un `enum` | quarto `index_only` sul filo, un indice che non si ritira mai, e `frozen_bytes.rs` a pinzarne le varianti **una per record congelato**. Il `bool` si pinza da sé |
| `Permission` resta a campi `pub`, `PermissionDetail` no | il primo è un tipo di **decisione** (`&'static str` **è** la guardia), il secondo è **sul filo** e porta `String` |
| ⚠️ **niente revoca**, e niente **sessione** | dichiarati sul modulo, con l'innesco: la prima revoca sarà una specie propria. `V21` resta **`⚠️ parziale`** |

⚠️ **`V21` è stata RILETTA e non solo lasciata stare** — vive nella **spec**
(`docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`), non in questo file. Il suo innesco
dice *«test a esempi sulla forma del permesso e sulla sua registrazione nel giornale (§6.6): una
tripla concessa non copre una tripla diversa»*, ed è **esattamente** ciò che il banco fa: l'innesco
è giusto e non va corretto. La metà *«e per la sessione corrente»* non ha soggetto — nessuna
sessione, nessun mediatore, nessun ciclo d'approvazione — e **non si marca**.

⚠️ **VOCE APERTA REGISTRATA E NON PRESA, ed è del proprietario:** i due casi `compile_fail` nuovi
**non hanno una riga di catalogo propria**. La §7.4 è **spec**, vincolo globale 7 — stesso
trattamento e stessa ragione del caso di `E94` qui sopra.

⛔ **E UN DIFETTO PREESISTENTE, TROVATO E NON INTRODOTTO:** `cargo fmt --all --check` era **già
rosso alla BASE `bf1e677`** — `crates/kernel/tests/gateway_decisor.rs:179`, una `assert_eq!` che il
compito 6 ha lasciato non formattata. Misurato con `rustfmt --check --edition 2024` sul file preso
da `git show bf1e677:`, uscita **1**.

⚠️ **RICHIAMO DEL 2026-09-01: qui stava «È rimediato in questo commit», e non è vero.** Il rimedio
è in **`b66295d`**, un commit **precedente e separato**, il cui messaggio lo dichiara staccato
perché *«è l'unico pezzo ORDINABILE»*; questa sezione vive in **`8d45eea`**, che ne è il
discendente — `git merge-base --is-ancestor b66295d 8d45eea` esce **0**. La *ragione* per cui è
stato rimediato invece che sorvegliato resta intatta: `cargo fmt` non è un passo del cancello, e
nessun rosso lo avrebbe mai detto.

📌 **Baseline a cambiamento chiuso: `GATE GREEN`, 43 bersagli, 321 passate, 0 fallite, 2**
**ignorate** — era **42 / 309 / 0 / 2**. Lo scarto è **+1 bersaglio** (`permission_triple`) e **+12**
**passate**: **dieci** dal banco nuovo e **due** da `reconciliation.rs`, che tiene l'arm vuoto nelle
sue due direzioni.

⚠️ **RICHIAMO DEL 2026-09-01 — LA REVISIONE HA SPOSTATO QUESTA CIFRA, e la riga sopra resta come
storia del commit `8d45eea`.** I rimedi ai cinque rilievi aggiungono **due** sonde e **nessun**
bersaglio, perché entrambe entrano in banchi che esistono già: `GATE GREEN`, **43 bersagli, 323
passate, 0 fallite, 2 ignorate**. Lo scarto è **+2 passate**.

### La passata INDIPENDENTE sul perimetro del compito 5 — un'affermazione di sicurezza falsa in `src/` (2026-09-01)

⛔ **Non è un giro del ciclo di `E53`: è una passata a sé, e la differenza è il METODO.** Fatta da
un sotto-agente fresco a cui la lettura d'apertura di [`../CLAUDE.md`](../CLAUDE.md) è stata
**VIETATA** — niente compendio, niente piano, niente questo registro. Le passate precedenti le
fecero sessioni e sotto-agenti che avevano letto la §6 e sapevano quindi **che cosa era già stato
trovato**; questa no, e ha reso **sei** rilievi che i giri precedenti non avevano visto,
`E108`–`E113`. ⚠️ **Tutti riverificati dal coordinatore prima di agire**, e uno **sovrapprezzato**.

⛔ **`E108` — LA VOCE BLOCCANTE, E STA IN `src/`: UN PARAGRAFO CHE SPIEGA PERCHÉ UN CAMPO DEL
FORMATO DUREVOLE È SICURO, E CHE LA MISURA SMENTISCE.** Il doc di `Detail` diceva *«AN UNKNOWN
VARIANT DOES NOT DECODE, and that is what makes the field safe … A build that does not know a
species STOPS instead of guessing»*. ✅ **Riprodotto dal coordinatore su una sonda usa-e-getta
scritta da fuori la crate, compilata, eseguita e cancellata nella stessa corsa**, sui byte
congelati del verdetto:

| Byte cambiato | Risposta di `Record::decode` |
|---|---|
| indice di variante di `Detail`, `00` → `03` (il primo libero) | ⛔ **`Ok(V1(RecordV1 { .. detail: None .. }))`** |
| lo stesso, `00` → `09` | ⛔ **`Ok(.. detail: None ..)`** |
| `kind` `03` → `09` | ✅ `Err(Malformed)` |
| corpo di `Detail` da `array(2)` ad `array(3)` | ✅ `Err(Malformed)` |

⛔ **Quindi il difetto è isolato all'indice di variante, e ciò che ferma una build vecchia è il
`kind` APPAIATO** — esattamente ciò che il doc di `RecordV1::detail` dice nello stesso file:
*«LOSES THE SUBSTANCE IN SILENCE — the new `kind` is what makes that build stop»*. **Due paragrafi
di una crate si contraddicevano**, e la misura dà ragione a quello dell'indice 5. 📌 **Gotcha #98
alla terza occorrenza:** la misura citata era **vera** e apparteneva agli enum sotto
`#[cbor(index_only)]`, cioè al campo **accanto**.
⛔ **E LA COPPIA TENUTA AL LIVELLO 1 DA `AUD-050` NON CHIUDE QUESTA STRADA**, che è la distinzione
da tenere e la ragione per cui il rilievo sopravvive alla cura di stamattina: il livello 1 rende
**incostruibile** una coppia sbagliata, mentre qui si **decodificano byte già scritti**. Una
**seconda specie** di `Detail` sotto un `kind` **esistente** verrebbe ingoiata da una build vecchia.
⚖️ **Il formato non è insicuro oggi** e il paragrafo è stato corretto **sulla misura**, per
sottrazione; ciò che resta è **registrato e non preso**, del proprietario, e deliberatamente **non
pinzato** — una sonda sul silenzio di oggi sarebbe un voto contro il cambiarlo (gotcha **#73**).

⛔ **`E109` — `==` NON È UNA GUARDIA, E L'ASSENZA NON SI TROVA MUTANDO.** `CostClass` e
`VerdictOutcome` erano tenuti da **soli `==` e da nessun `match`**. ⚠️ **RICHIAMO DEL 2026-09-01, nono giro:** qui e in altre due case stava *«quattro `==`»*, ed erano **tre**, tutti in `sensor.rs` — `git grep -nE 'CostClass|VerdictOutcome' 52efadd -- 'crates/*.rs' | grep -c '=='`. Tolto dalle tre case (`E128`). ⛔ **La prova è un
CENSIMENTO e non una mutazione, ed è più forte:** una mutazione dice che *una* corsa è restata
verde, il censimento dice che in **tutto** `crates/` non esiste un `match` su quei tipi, quindi
**niente in nessun punto** potrebbe cogliere una variante nuova. È la ragione per cui quattro
passate a mutazioni non l'avevano vista — **una mutazione coglie un comportamento CAMBIATO, mai un
tipo CRESCIUTO**, gotcha **#104**. ⛔ **E la caduta di `CostClass` è dalla parte SBAGLIATA:** tutto
ciò che non è `Inferential` **entra** nell'anello stretto, cioè la cosa che V11 esiste per
rifiutare — stessa forma di `E55`. ⚖️ **Decisione del proprietario: guardia su entrambi.**
✅ **Provata nelle due direzioni**, `` error[E0004]: `CostClass::Remote` not covered `` e
`` error[E0004]: `VerdictOutcome::Inconclusive` not covered ``, revocate con `cp` **più `touch`** —
la cura del gotcha **#103**, applicata invece che riscoperta.

⛔ **`E110` — l'unica strada di `run_the_ring` che risponde `Err`, e non la raggiungeva nessuna
sonda.** Il doc **afferma** il valore e la testa del banco lo **descrive**, ma descrivere non è
tenere. ✅ Misurato prima: col `?` di `journal.note` in `let _ = ..` il workspace resta identico
alla baseline. ✅ Chiusa con una sonda che asserisce **due** cose — `Err(OutOfOrder)` **e** archivio
**vuoto** — perché è la seconda che conta. ✅ **Controllo dopo:** con la stessa mutazione è
**l'unica rossa del banco**, `5 passed; 1 failed`.

⚠️ **`E111`, `E112`, `E113` — le tre di prosa, tutte chiuse per SOTTRAZIONE**, e la seconda porta
un metodo. `E111`: tre clausole del doc di `Detail` — *«`Record` ABOVE»* (è **sotto**), *«the only
two data-carrying enums»* (`wire::worker::FromWorker` è un terzo, e porta l'argomento **opposto**) e
una data smentita da `git log -L`. `E113`: *«a record is 21 bytes and not 20»* e *«before `85`, the
five fields»*, mentre i record congelati sono **sei** e vanno da **21** a **40** byte — chiusa in
**entrambe** le case, `frozen_bytes.rs` e la mappa.
⛔ **`E112` È LA PIÙ ISTRUTTIVA, e cambia la forma di `E66`:** dal 2026-09-01 la **maggior parte**
delle date `2026-09-01` nel sorgente è **legittima**, perché quei commit sono davvero di oggi.
Quindi **un `grep` non basta e uno sweep sarebbe stato un difetto peggiore di quello che correggeva**:
ogni occorrenza è stata datata con `git log -1 -L <riga>,<riga>:<file>`. Cinque occorrenze, **tre**
scritte oggi — vere, restano — e **due** scritte da commit del **2026-08-31**, quindi false e tolte.
📌 *Un censimento che sweepa una parola invece di datarla riga per riga produce l'errore che stava
correggendo.*

⚖️ **DOVE IL RAPPORTO SOVRAPPREZZAVA.** Su `E108` dava *«cinque clausole false su sei»*. Ricontate:
**tre** flatamente false, **una vera** (`decode` mappa ogni errore a `Malformed`) e **due** vere ma
su una strada mai percorsa. Non cambia gravità né rimedio, ma il rimedio doveva **sottrarre** e non
riscrivere il paragrafo intero — gotcha **#65** applicato al rapporto di un sotto-agente.

⚠️ **E UNA VOCE REGISTRATA E NON PRESA, che il sotto-agente non ha visto.** Il doc di **modulo** di
`record.rs` dice *«with and without `#[cbor(array)]` on the two types below»*, e i tipi che sotto di
esso portano quell'attributo sono più dei due che nomina — quanti, lo dice `grep -c '^#\[cbor(array)\]' crates/kernel/src/record.rs`. ⚠️ **RICHIAMO DEL 2026-09-01, nono giro:** qui stava *«tre»*, falso alla misura (`E128`). ⛔ **Non toccata, ed è una decisione:** la frase descrive
una **misura passata** e non un censimento, quindi non è chiaramente falsa, e correggerla di
iniziativa significherebbe prezzare come difetto ciò che potrebbe essere un verbale. È del
proprietario.

📌 **Baseline a passata chiusa: `GATE GREEN`, 43 bersagli, 324 passate, 0 fallite, 2 ignorate**, da
**323**. ⛔ **Lo scarto è +1, ed è una sonda sola:** `sensor_ring.rs` passa da cinque test a sei e
nessun altro banco ne guadagna — misurato con `grep -c '^#\[test\]'` contro la versione di `HEAD`,
non dedotto. `cargo fmt --all --check` pulito, fine-riga dei sei file di `crates/` **immutati**.

### La NONA passata — la domanda di classe coglie il CENSIMENTO che aveva chiuso una classe (2026-09-01)

⛔ **Fatta il 2026-09-01 da un sotto-agente fresco con la lettura d'apertura vietata**, perimetro
`git diff 9441e6d..HEAD -- crates/`; il brief è `E123` e il verbale `E130` del piano del Traguardo 6.
Sei rilievi del revisore, tutti riverificati dal coordinatore prima di rimediare e tutti veri, più
uno del coordinatore: le voci sono `E124`–`E129`. Qui stanno le **misure**.

#### La domanda di classe, rimedio per rimedio

| Rimedio | Verdetto | Prova |
|---|---|---|
| `E114` — `Operation::is_write` | ⛔ **occorrenza** → `E124` | `grep -rnE "(==\|!=) *[A-Z]\w*::[A-Z]\|matches!\(" crates/*/src` rendeva, prima di `fe52039`, oltre a `RecordKind` e ad `Admission` (tenuti — il secondo da quattro `matches!` nel modulo di test dell'arbitro, e cresciuto dà `E0004`), `Activity`/`PreemptibleState`, `TaskState` ed `EntryKind`: cresciuti di una variante, **zero `E0004`** su tutti e quattro. ⚠️ Diceva *«rende, oltre a `RecordKind`»* e ometteva `Admission`: corretto al decimo giro, `E133` |
| `E115` — `ConstraintClass` | occorrenza, stessa classe | oggi tenuto: cresciuto, `E0004` in `gateway/mod.rs` — misurato dal revisore |
| `E109` — `CostClass`, `VerdictOutcome` | occorrenza, stessa classe | oggi tenuti: cresciuti, `E0004` in `sensor.rs` — misurato dal revisore |
| `E116` — il `payload` di `dispatch` | ✅ classe chiusa | ogni sito di `src/` che scrive un record — `grep -rnE 'journal\.(note\|intent\|outcome)\(' crates/*/src` — ha un banco che ne rilegge **ogni** campo; l'unico non asserito per nome, l'`effect` di `set_policy`, è tenuto da `Resolution::RunAgain` in due sonde (mutazione `M1` sotto) |
| `E117` — la seconda scrittura di `run_the_ring` | ✅ classe chiusa | l'altra funzione a due scritture, `Arbiter::set_policy`, ha l'`outcome` tenuto da `CrashingJournal::falling_at(1)` (`M2`) |
| `E118` — `steps_in_doubt` | ✅ classe chiusa, prosa no → `E126` | i lettori del giornale in `src/` — `grep -rnE 'journal\.(replay\|read_back)\(' crates/*/src` — sono `reconcile.rs` e `permission.rs`, entrambi con la sonda del `replay` rifiutato (`M5`, `M16`) |
| `E119`, `E121` | ✅ classe chiusa, numerali no → `E129` | `grep -rn "were walked" crates/` e `grep -rn "ONLY way to build" crates/`: ciò che resta è una definizione o è già qualificato |
| `E120` — la regola A3 | occorrenza, regola sovra-generale → `E127` | `VerdictDetail` porta `bool` e `u64`, e lo dice di sé |
| le due celle de-enumerate dell'ottavo giro | ⛔ **occorrenza** → `E128` | la riga `permission::is_granted` della tabella del compito 7 enumerava ancora |
| `E105`, `E106`, `E108`, `E110`–`E113` | ✅ classe chiusa | le mutazioni sotto; `E110` per equivalenza — `dispatch`, `promote` e `grant` hanno ciascuno la propria via `Err` tenuta; `E112` con `git log -S` sulle tre date degli arm |

#### La crescita, enum per enum

`cargo check --locked --workspace --all-targets`, una variante aggiunta da sola e revocata byte-esatta.

| Enum | Prima del rimedio | Dopo il rimedio (`fe52039`) |
|---|---|---|
| `arbiter::PreemptibleState` (`pub`) | **exit 0, zero `E0004`** — rimisurato dal coordinatore | `E0004` in due punti: `Activity::is_revoking` e `collect_expired` |
| `arbiter::Activity` (`pub`) | **exit 0, zero `E0004`** — rimisurato | `E0004` in due punti, gli stessi |
| `executor::TaskState` | **exit 0, zero `E0004`** — rimisurato | `E0004` in due punti: `is_runnable`, `sleeping_until` |
| `simulator::journal::EntryKind` | **exit 0, zero `E0004`** — rimisurato | `E0004` in due punti: `is_intent`, `is_outcome` |
| `permission::Operation`, il controllo | `E0004` a `is_write` — rimisurato | invariato |
| `ConstraintClass`, `CostClass`, `VerdictOutcome`, `RecordKind`, `EffectClass`, `Constraint`, `Record`, `Detail`, `Trust` | **tenuti** — misurati dal revisore e non rifatti dal coordinatore; `Trust` nei due versi: con indice → `E0004` in `frozen_bytes.rs`, senza indice → `minicbor` rifiuta la variante | — |
| `reconcile::Resolution` | cresce a `exit 0` **e nessuno la decide oggi**, né in `src/` né nei banchi | ⚠️ **registrata, non presa:** il primo consumatore la decide con un `match` |

⛔ **La caduta dei due dell'arbitro era dalla parte PERMISSIVA:** uno stato terzo non era colto dalla
guardia di `ask_back` e veniva rimarcato `Revoking` con una grazia **fresca** — la sovra-ammissione
dalla porta di servizio che quella guardia esiste per rifiutare — e `revoking()` lo sotto-contava.
`TaskState` parcheggiava un'attività in silenzio fino al limite di turni; `EntryKind` cadeva dalla
parte conservativa, ma in silenzio. ⛔ **E il braccio `_` di `collect_expired` è la terza bocca del
censimento:** a chi cerca un `match` compare come guardia, e lascia passare la crescita come un `==`.

#### Le mutazioni di prodotto del revisore — tutte rosse

Baseline `43 · 326 · 0 · 2`; ciascuna applicata da sola, revocata con `cp` più `touch` e confermata
col `cmp`, e ogni frammento confermato su una riga di codice. ⚠️ **Misurate dal revisore; il
coordinatore ha rifatto le due di `E125` e le cinque crescite qui sopra.**

| Mutazione | Quaterna | Chi muore |
|---|---|---|
| `M1` `transition_record`: `Idempotent` → `Unrepeatable` | 43 · 324 · 2 · 2 | `a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt`, `property_4_a_severed_transition_leaves_a_reconcilable_step` |
| `M2` `set_policy` inghiotte `journal.outcome` | 43 · 325 · 1 · 2 | `a_transition_cut_between…` |
| `M3` `dispatch`, tre byte nel `payload` | 43 · 325 · 1 · 2 | `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it` |
| `M4` `run_the_ring`: `let _ = journal.intent(next, ..)` | 43 · 325 · 1 · 2 | `the_second_write_of_the_ring_is_not_swallowed_either` |
| `M5` `steps_in_doubt`: `replay().unwrap_or_default()` | 43 · 325 · 1 · 2 | `an_archive_that_will_not_replay_is_not_an_answer_of_no_doubt` |
| `M6` `promote`: `Unrepeatable` → `Idempotent` | 43 · 325 · 1 · 2 | `the_promoted_content_is_the_payload_and_it_is_labelled_untrusted` |
| `M7` `run_the_ring`: `let _ = journal.note(step, ..)` | 43 · 325 · 1 · 2 | `a_step_nobody_opened_is_refused_and_nothing_is_written` |
| `M8`–`M11` `grant`: `trust` → `Untrusted`, `effect` → `Idempotent`, un byte di `payload`, `reason` vuota | 43 · 325 · 1 · 2, ciascuna | `grant_writes_every_field_it_says_it_writes` |
| `M12a` il flag `write` di `grant` appiattito a `false` | 43 · 325 · 1 · 2 | idem, e lei sola |
| `M12b` appiattito a `true` | 43 · 323 · 3 · 2 | `a_different_OPERATION_is_not_covered`, `a_granted_triple_is_granted`, `a_record_of_another_species_…` |
| `M13a`/`b`/`c` `is_granted` senza il congiunto `tool` / `resource` / `write` | 43 · 324 · 2 · 2 / 43 · 325 · 1 · 2 / 43 · 325 · 1 · 2 | `a_different_TOOL_…` più la specie diversa / `a_different_RESOURCE_…` / `a_different_OPERATION_…` |
| `M14` il record illeggibile saltato | 43 · 325 · 1 · 2 | `a_record_that_will_not_decode_is_not_an_answer_of_false` |
| `M15` il `Permission` senza `detail` saltato | 43 · 325 · 1 · 2 | `a_permission_record_without_its_detail_is_not_an_answer_of_false` |
| `M16` `replay` rifiutato → `Ok(false)` | 43 · 325 · 1 · 2 | `a_journal_that_will_not_replay_is_not_an_answer_of_false` |
| `M17` `grant` inghiotte la nota | 43 · 325 · 1 · 2 | `a_journal_that_refuses_the_note_makes_the_grant_fail` |
| `M18` il controllo di specie disattivato | 43 · 321 · 5 · 2 | le cinque che la tabella ① del compito 7 nomina |
| `M19` `grant` scambia `tool` e `resource` | 43 · 323 · 3 · 2 | le tre che `frozen_bytes.rs` e `permission_triple.rs` nominano |
| `M20` gli indici `#[n(0)]`/`#[n(1)]` di `tool` e `resource` scambiati | 43 · 325 · 1 · 2 | `the_two_names_of_a_permission_do_not_share_one_offset_and_its_mirror`, e lei sola |
| `M21` `resource: &str` in `PermissionDetail::new` | 43 · 325 · 1 · 2 | `level_1_rules_do_not_compile`: `permission_detail_resource_…` → `error`, il gemello e i tre fratelli → `ok` |
| `E125`, rifatte dal coordinatore: l'arm `Permission` → `enter(..)` / → `leave(..)` | 43 · 324 · **2** · 2 / 43 · 325 · **1** · 2 | entrambe le sonde della coppia / la sola `…_leaves_…` — la frase *«exactly the half»* era falsa |

#### Il rilievo del coordinatore, trovato rimediando

`evaluated_is_what_the_chain_OFFERED_and_not_what_the_first_pass_walked` in
`crates/kernel/tests/gateway_decisor.rs`, nato in `9441e6d`, era l'unico nome con maiuscole del file
senza `#[allow(non_snake_case)]`: `cargo check --locked -p kernel --test gateway_decisor` stampava
**un** avviso, e la misura *«`cargo build --locked --workspace` a zero avvisi»* non lo vede perché
**non compila i banchi**. Chiuso col precedente dei tre fratelli dello stesso file; dopo, zero avvisi.
⚠️ **Registrata, non presa:** se la riga a zero avvisi debba passare a `--all-targets` tocca il
cancello, ed è del proprietario (vincolo globale 7).

📌 **Baseline a passata chiusa: `GATE GREEN`, 43 bersagli, 326 passate, 0 fallite, 2 ignorate** —
invariata, perché il rimedio di prodotto non aggiunge sonde: aggiunge `match`. `cargo fmt --all
--check` exit 0 (un hunk sistemato a mano), build a zero avvisi, fine-riga invariati. ⚠️ **E
`riferimenti.md` NON è stato toccato, deliberatamente**, per la stessa ragione di ogni passata.

### La DECIMA passata — la prima ondata di CODICE riletta senza un difetto di prodotto (2026-09-01)

⛔ **Fatta il 2026-09-01 da un sotto-agente fresco con la lettura d'apertura vietata**, perimetro
`git diff 02d2162..HEAD -- crates/` — i due commit del nono giro; il brief è `E131` e il verbale
`E135` del piano del Traguardo 6. Tre rilievi, `E132`–`E134`, tutti di prosa o di registro, tutti
riverificati dal coordinatore e tutti veri. Qui stanno le **misure**, del revisore salvo dove detto.

#### I quattro predicati di `fe52039`, braccio per braccio

Baseline `43 · 326 · 0 · 2`; ogni rovescio applicato da solo, revocato byte-esatto con `mtime`
rinfrescato, e confermato su una riga di codice.

| Predicato | Braccio rovesciato | Quaterna | Chi muore |
|---|---|---|---|
| `Activity::is_revoking` | `NonPreemptible => true` | 43 · 322 · 4 · 2 | `a_non_preemptible_grant_is_never_asked_back`, `the_grace_runs_out_at_the_instant_of_its_deadline` e altre due |
| | `Preemptible(Running) => true` | 43 · 312 · 14 · 2 | fra cui `asking_back_twice_does_not_buy_the_room_twice` |
| | `Preemptible(Revoking { .. }) => false` | 43 · 318 · 8 · 2 | fra cui `a_grant_inside_its_grace_keeps_its_reservation` |
| i bracci nominati di `collect_expired` | `NonPreemptible => false` | 43 · 306 · 20 · 2 | in quattro bersagli, `daemon` compreso |
| | `Preemptible(Running) => false` | 43 · 307 · 19 · 2 | fra cui `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` |
| `TaskState::is_runnable` | `Runnable => false` | 43 · 306 · 20 · 2 | `executor_determinism`, `arbiter_campaign`, `dst_campaign` |
| | `Sleeping(_) => true` | 43 · 321 · 5 · 2 | fra cui `a_reactor_that_will_not_advance_is_an_error_and_not_a_spin` |
| `TaskState::sleeping_until` | `Sleeping(until) => None` | 43 · 310 · 16 · 2 | sedici |
| | `Runnable => Some(..)` | ⚠️ **43 · 326 · 0 · 2, verde** | **equivalente per costruzione**: entrambi i chiamanti girano solo dopo `poll_one_turn() == false`, cioè senza attività `Runnable`. **Dichiarato nel doc del predicato** (`E135`) |
| `EntryKind::is_intent` | `Intent => false` | 43 · 248 · 78 · 2 | settantotto |
| | `Outcome => true` | ⚠️ **43 · 326 · 0 · 2, verde** | **equivalente per costruzione**: `outcome` esige `has_intent`, quindi nessun passo porta un `Outcome` senza un `Intent`. **Dichiarato nel doc del predicato** (`E135`) |
| | `Note => true` | ⚠️ **43 · 326 · 0 · 2, verde** | equivalente **e già dichiarato** sull'enum: *«Filing a note under `EntryKind::Intent` leaves the ENTIRE workspace green»* |
| `EntryKind::is_outcome` | `Outcome => false` | 43 · 319 · 7 · 2 | le sonde di `prune` nei due binari della conformità |
| | `Intent => true` | 43 · 319 · 7 · 2 | idem, più `prune_refuses_and_leaves_the_record_where_it_was` |
| | `Note => true` | 43 · 322 · 4 · 2 | fra cui `the_in_memory_journal_honours_the_contract` |

📌 **Un rovescio verde non è qui un mutante vivo da chiudere ma una proprietà da dichiarare**, ed è
la forma di `E70`: ciò che i predicati comprano è l'`E0004` sulla crescita, e la crescita è tenuta.

#### Il censimento della crescita — tutti gli enum di `crates/*/src`

`cargo check --locked --workspace --all-targets --keep-going`, una variante per volta, in una copia
del workspace; la seconda direzione con un indice `#[n(..)]` dove `minicbor` rifiuta.

| Esito | Enum |
|---|---|
| **tenuti** in `src/` | `Activity`, `PreemptibleState`, `TaskState`, `EntryKind` (i quattro di `E124`, `E0004` in due punti ciascuno) · `Operation` · `Admission` (tre punti, `daemon` compreso) · `StartupError` · `VramPolicy`, `Constraint`, `ConstraintClass`, `Preemption` (due punti) · `ComputeClass`, `CostClass`, `VerdictOutcome`, `RecordKind`, `EffectClass` · `Record` (`E0004` più `E0005` sul `let Record::V1(body)` di `is_granted`) |
| tenuti **solo da un banco** | `Trust`, `Detail` (`frozen_bytes.rs`; `Detail` è deciso in `src/` dal `let … else` di `is_granted`, verso fail-closed) · `Released`, `ReleaseError` (`arbiter_campaign.rs`) |
| a `exit 0`, **e nessuno decide** su di essi in `src/` — si nominano, non sono rilievi | `Resolution` (come registrato al nono giro) · `RunError`, `WireError`, `GatewayError`, `PermissionError`, `FilesystemError`, `IpcError`, `JournalError`, `NetworkError`, `ProcessError`, `Started`, `RecordError`, `wire::Verdict`, `IpcMessage`, `FromWorker`, `OpenError`: costruiti e propagati |

⚠️ **Il limite dichiarato dal revisore:** quando l'`E0004` cade nella libreria, i banchi non vengono
compilati, quindi un `match` in un banco resta invisibile — irrilevante per *«tenuto»*. E i quattro
di `E124` rimisurati coi blob di `02d2162` danno **exit 0, zero errori** prima del rimedio.

#### I tre rilievi, e la domanda di classe

| | Specie | Che cosa |
|---|---|---|
| `E132` | prosa del perimetro | il commento nuovo di `collect_expired` diceva *«for ever»*: la guardia `expires_at <= now` raccoglie ogni stato alla chiusura della finestra — misurato con una sonda usa-e-getta, `promote(4_999)` lascia `4_096`, `promote(5_000)` azzera |
| `E133` | registro | la riga `E114` della tabella della NONA passata ometteva `Admission` fra ciò che il comando rende |
| `E134` | prosa **fuori dal perimetro**, specie di `E129` | tre numerali di distanza falsi oggi in `dst_campaign.rs` e `ports_are_implementable.rs`, e uno di essi era un **fatto**: `ScriptedWorker` prende una concessione da `822db6d` |

✅ **Il campione delle misure del nono giro, rifatto dal revisore, coincide col registro:** `M1`,
`M5`, le due di `E125` e tutte le crescite.

📌 **Baseline a passata chiusa: `GATE GREEN`, 43 bersagli, 326 passate, 0 fallite, 2 ignorate**,
`cargo fmt --all --check` exit 0, zero avvisi su `build` e su `check --all-targets`. ⚠️ **E
`riferimenti.md` NON è stato toccato, deliberatamente.**

### Il compito 8 del Traguardo 6 — lo stato di degrado si RICALCOLA, e non si cachea (2026-09-02)

⛔ **Lo stato di degrado è un DERIVATO e non un archivio.** `crates/kernel/src/degradation.rs`
porta `Degradation`, `DegradationError` e `degradation_now`, che rilegge il giornale e interroga
l'arbitro **a ogni domanda**; il banco nuovo è `crates/kernel/tests/degradation_state.rs`. È la
forma di `permission::is_granted` e di `reconcile::steps_in_doubt`: nessuna cache accanto alle due
fonti, quindi *«mai autorevole di sé stesso»* è vero **per costruzione** e non per disciplina.
Il formato durevole **non è toccato** — nessuna variante nuova, nessun record congelato, nessun
`.stderr`.

| Artefatto | Che cosa lo esercita |
|---|---|
| `Degradation::routing_degraded` | `a_degraded_routing_shows_up_in_the_state`, `it_is_RECOMPUTED_and_not_cached` e `the_LAST_routing_wins_and_not_any_routing` |
| `Degradation::vram_exhausted` | ⛔ `a_full_arbiter_declares_its_vram_exhausted` — **la sonda che il piano non aveva scritto**: le tre dettate provavano il routing in tre modi e il tetto in **nessuno**, e il buco sta scritto nel piano stesso come `M3` invece di essere corretto in silenzio |
| *«l'ULTIMO routing e non uno qualsiasi»* | `the_LAST_routing_wins_and_not_any_routing`, che dispaccia **due** volte sullo stesso passo aperto — un routing degradato, poi uno pulito |
| il **ricalcolo** | `it_is_RECOMPUTED_and_not_cached`, che cambia il mondo **fra** le due domande e tiene fermo l'arbitro, così che l'unica cosa mossa sia il giornale |
| la **non-vacuità** dei due campi | `an_idle_machine_declares_nothing`, la sola sonda che parte da una macchina ferma **e** da un giornale vuoto |
| `DegradationError::Journal` | `a_journal_that_will_not_replay_is_not_an_answer_of_nothing_degraded` |
| `DegradationError::Record` | **due** sonde, una per strada: il record che non si decodifica, e il `Routing` senza il proprio `detail` |
| `Arbiter::ceiling` | ogni sonda che legge `vram_exhausted`; il getter nasce col chiamante che lo pretende e non ne ha altri |

#### Le mutazioni, col proprio esito MISURATO

⛔ **Applicate una per volta sul codice di PRODUZIONE**, ciascuna revocata da una copia
byte-esatta presa prima: `M1`, `M2` e `M10` verificate con `sha256sum`, le altre con
`git diff -- crates/kernel/src/degradation.rs` a **zero** righe. ⚠️ **La differenza non è
stilistica, ed è un rilievo che sembra un verde:** `git diff` è vuoto su un file **non tracciato**
qualunque cosa contenga — il caso di `M1` e `M2`, prima del `git add` — e su un file con modifiche
**non ancora committate** rende quelle, che è il caso di `M10`. Un file nuovo si mette
nell'indice **prima** della campagna, e una campagna si misura su un albero **pulito**.
La baseline contro cui va letta la colonna «Chi muore» è quella a cambiamento chiuso,
**44 bersagli, 334 passate, 0 fallite, 2 ignorate**.

| # | Mutazione | Chi muore | Misura |
|---|---|---|---|
| **M1** | `routing_degraded = routing.degraded()` → `= false` | **tre** — `a_degraded_routing_shows_up_in_the_state`, `it_is_RECOMPUTED_and_not_cached`, `the_LAST_routing_wins_and_not_any_routing` | **44 bersagli, 331 passate, 3 fallite, 2 ignorate** |
| **M2** | il ciclo si ferma al **PRIMO** record di routing — un `break` dopo l'assegnazione | **una sola**, `the_LAST_routing_wins_and_not_any_routing`, con *«a degradation that was resolved is still being reported as the state NOW»* | **333 passate, 1 fallita**. ⛔ **Il piano prevedeva che questa mutazione non uccidesse niente** e lasciava la scelta fra la sonda e una dichiarazione: la sonda è stata scritta, e il mutante muore. La frase *«l'ultimo e non uno qualsiasi»* non è più un'intenzione |
| **M3** | `vram_exhausted` → `false` fisso | **una sola**, `a_full_arbiter_declares_its_vram_exhausted` | **333 passate, 1 fallita**. ⛔ **Senza la sonda nuova era un mutante VIVO**: nessuna delle tre sonde dettate tocca il tetto |
| **M4** | `allocated() >= ceiling()` → `>` | **una sola**, `a_full_arbiter_declares_its_vram_exhausted` | **333 passate, 1 fallita**. ⚠️ **Aggiunta perché il commento della sonda AFFERMA il confine** — *«a `>` in place of the `>=` answers "fine" to a full machine»* — e un'affermazione che nessuno tiene è un'intenzione. La macchina è riempita **esattamente**, quindi la sonda sta **sopra** il confine invece di scavalcarlo |
| **M5** | il record illeggibile viene **saltato**: `let Ok(..) = Record::decode(..) else { continue }` | **una sola**, `a_record_that_will_not_decode_is_not_an_answer_of_nothing_degraded` | **333 passate, 1 fallita**. ⛔ **È ALLA LETTERA IL CODICE CHE IL PIANO DETTAVA**, corretto da `E139` prima del dispaccio: la correzione è ora **misurata** e non solo argomentata |
| **M6** | il `Routing` il cui `detail` non è un `Routing` viene saltato da un `if let` silenzioso | **una sola**, `a_routing_record_without_its_detail_is_not_an_answer_of_nothing_degraded` | **333 passate, 1 fallita**. ⛔ **Anche questo è il codice dettato dal piano**, e la strada è raggiungibile **dai byte**: impronunciabile in sorgente perché `RecordV1::routing` prende il `detail` per valore, costruibile riscrivendo **un** byte della specie |
| **M7** | `journal.replay()` che rifiuta diventa un archivio **vuoto** (`unwrap_or_default`) | **una sola**, `a_journal_that_will_not_replay_is_not_an_answer_of_nothing_degraded` | **333 passate, 1 fallita**. ⛔ È la ragione per cui `DegradationError` esiste: *«non lo so»* riportato come *«niente è degradato»* è il degrado silenzioso che ADR-0019 vieta |
| **M8** | `routing_degraded` parte da `true` | **tre** — `an_idle_machine_declares_nothing`, `it_is_RECOMPUTED_and_not_cached`, `a_full_arbiter_declares_its_vram_exhausted` | **331 passate, 3 fallite**. ⚠️ **E NON uccide `the_LAST_routing_wins_…`**, il che è giusto: quella sonda dispaccia due volte e il ciclo sovrascrive comunque il valore iniziale. La direzione *«vero a tutto»* la tiene la sonda della macchina ferma, e nient'altro |
| **M9** | `vram_exhausted` → `true` fisso | **due** — `an_idle_machine_declares_nothing` e `a_degraded_routing_shows_up_in_the_state` | **332 passate, 2 fallite**. ⚠️ La seconda muore sull'asserzione *«e l'altro campo no»*, che esiste perché i due campi sono derivati da **due mondi indipendenti** e una derivazione che li muovesse insieme passerebbe la prima asserzione |
| **M10** | il **filtro per specie** tolto: `if body.kind() != RecordKind::Routing { continue; }` | **tre** — `a_degraded_routing_shows_up_in_the_state`, `it_is_RECOMPUTED_and_not_cached`, `the_LAST_routing_wins_and_not_any_routing`, tutte con `derive: Record(Malformed)` | **331 passate, 3 fallite**. ⛔ **Misurata il 2026-09-02 perché era tenuta per DEDUZIONE:** senza il filtro l'`Intent` che apre il passo arriva al controllo del `detail`, non ne ha, e la risposta diventa `Malformed`. ⚠️ **E le tre sonde delle vie d'errore restano VERDI, DUE per la ragione sbagliata:** `a_routing_record_without_its_detail_…` e `a_record_that_will_not_decode_…` attendono entrambe `Malformed` e lo ricevono dall'`Intent` che apre il passo, il cui `detail` è `None`, invece che dai byte che hanno scritto. ⛔ **Misurato e non dedotto:** senza il filtro `it_is_RECOMPUTED_and_not_cached` muore sulla PRIMA domanda (`crates/kernel/tests/degradation_state.rs:185`), quando il giornale porta il **solo** `Intent` |

✅ **Nessun mutante è sopravvissuto, e ogni sonda del banco ha almeno un mutante che la uccide.**
La seconda metà è la domanda che si dimentica: `an_idle_machine_declares_nothing` non era uccisa
da nessuna delle mutazioni del piano, e `M8`/`M9` sono state scritte **per** interrogarla invece di
darla per buona.

#### Le decisioni prese, e ciò che costano

| | |
|---|---|
| `degradation_now` restituisce un **errore proprio** che compone `JournalError` e `RecordError` | è la forma che il proprietario decise per il compito 7 (`E104`), applicata per coerenza. Nessun chiamante esisteva, quindi nessuna firma è cambiata sotto qualcuno. Voce `E139` |
| `Arbiter` guadagna `ceiling()`, e **nient'altro** | un getter su un tipo che il Traguardo 5 aveva chiuso, sul precedente di `StepId::get`: nasce col chiamante che lo pretende. L'alternativa — passare i `Parameters` anche a `degradation_now` — avrebbe fatto tenere in passo al chiamante **due verità indipendenti** che l'arbitro già tiene insieme, la forma di `E25`. Voce `E140` |
| ⛔ **nessun terzo addendo**, e non è una scorciatoia | le due quote permanenti di ADR-0033 entrano dall'arbitro **attraverso `admit`** (`crates/daemon/src/main.rs`), quindi stanno già **dentro** `allocated()`, e i due numeri sono confrontabili come stanno. La voce aperta della §5.1 dell'arbitro **non** viene toccata |
| il tipo è **corto**, e i due ingressi senza sorgente non hanno un campo | connettività (`network` non ha implementazione reale) e salute dei provider (gli adattatori sono regola C) sono **dichiarati accanto al tipo**: un campo sempre `false` si legge *«tutto bene»* invece di *«non lo so»*, che è il più falso dei due |
| ⚠️ la **divergenza dichiarata** su *«mantiene»* | ADR-0019 e §6.7 dicono che il core *«mantiene uno stato di degrado corrente, alimentato dagli eventi»*, e quelle parole si leggono **anche** come mantenimento incrementale. Il codice legge *«mantiene»* come *«espone»*, e la divergenza è scritta **accanto alla funzione** perché la lettura opposta è del proprietario |
| il costo della rilettura | è quello che `Journal::replay` già dichiara: l'archivio intero per una domanda. Il rimedio è il **checkpoint** che quella operazione nomina, e si compra il giorno che una misura lo chiede — non qui |

⚠️ **`V27` e `Q18` sono state RILETTE e non solo lasciate stare**, e vivono nella **spec**
(§8.3 e §8.4), non in questo file. `V27` chiede che *«l'interfaccia lo dichiari prima»* e
un'interfaccia non c'è; `Q18` ha per metodo assegnato una campagna DST con iniezione del guasto di
rete, e `network` non ha implementazione reale. Restano **`⚠️ parziale`**, che è la condizione 12.

⚠️ **E il commento per modulo che il dispaccio si aspettava in `crates/kernel/src/lib.rs` NON
esiste:** le righe vicine sono `pub mod` nude separate da una riga bianca, e il doc del modulo dice
che *«la lista è l'unica risposta che non può invecchiare»*. La riga nuova segue **il file**, non
l'attesa, ed è in coda perché la lista è in ordine d'arrivo.

📌 **Baseline a compito chiuso: `GATE GREEN`, 44 bersagli, 334 passate, 0 fallite, 2 ignorate** —
era **43 / 326 / 0 / 2**. Lo scarto è **+1 bersaglio** (`degradation_state`) e **+8 passate**,
tutte del banco nuovo. `cargo fmt --all --check` exit 0, zero avvisi su
`cargo build --locked --workspace`.

### Il compito 9 del Traguardo 6 — le due proprietà di §5.7 che mancavano, e la riconciliazione che nessuno eseguiva (2026-09-02)

⛔ **La Parte E non aggiunge meccanismi: li mette sotto prova e chiude. Un'eccezione, ed è
`P-16`:** la terza proprietà di §5.7 pretende una **riconciliazione alla disconnessione** che non
esisteva. `crates/kernel/src/client.rs` porta `ClientGrants` e `on_disconnect`; le due campagne
mancanti sono `crates/simulator/tests/gui_death_campaign.rs` (proprietà **3**) e
`crates/simulator/tests/worker_kill_campaign.rs` (proprietà **2**). Ogni commit del compito è
`GATE GREEN` **da solo**, e quanti sono lo dice `git log` — nessun numerale qui, perché ne è già
invecchiato uno (gotcha **#31**).

⛔ **PASSO 0 — LE PROPRIETÀ RICONTATE SUL CODICE E NON SU «due».** Prima di scrivere,
`grep -n "property_" crates/simulator/tests/arbiter_campaign.rs` rende **1**, **4** e **5**: il
*«due»* del compito regge. A chiusura, `grep -rn "^fn property_" crates/simulator/tests/` rende
**cinque** righe, `property_1` … `property_5`, una per numero, su **tre** file.

| Artefatto | Che cosa lo esercita |
|---|---|
| `ClientGrants::on_disconnect` | `crates/kernel/tests/client_grants.rs`, **cinque** sonde — contate col comando, `grep -c '^#\[test\]' crates/kernel/tests/client_grants.rs`: la concessione torna, torna **solo quella di quel client**, un client che non tiene niente **non è un errore**, dopo la finestra la risposta è `AlreadyCollected` (sonda che il compito non aveva scritto, `E154`), e una concessione **di un altro arbitro** è il difetto del chiamante e lascia registrato il resto (sonda che la **prima ondata di revisione** ha aggiunto: era l'unica via che percorresse il ramo `Err`) |
| `ClientGrants::register` | ogni sonda del banco, e la campagna della proprietà 3 |
| `DyingGui` | `crates/simulator/tests/dying_gui.rs`, **dieci** sonde, sul precedente di `crashing_journal.rs`: il banco che `E156` ha trovato mancante nella mappa del compito |
| la proprietà **3** | `property_3_a_gui_that_dies_holding_a_grant_gives_it_back`, più `the_campaign_sweeps_every_world_this_scenario_has` |
| la proprietà **2** | `property_2_a_killed_worker_leaves_no_reservation_behind` |
| il **decodificatore** di `IpcMessage` | la campagna: la richiesta attraversa la porta **come byte** e viene decodificata dal core, non aggirata |

⛔ **LA MORTE SI LEGGE DALLA PORTA E DA NIENT'ALTRO, ed è il difetto che `P-16` nomina.** `Ipc`
non ha un evento di disconnessione — `accept`, `send`, `receive` — quindi la riconciliazione è
innescata **solo** da `Err(IpcError::Disconnected)`. ✅ **Misurato e non promesso, e COL
COMANDO CHE MISURA L'AFFERMAZIONE:** `grep -cE '\.has_died\(' <campagna>` dà **0** — le
**chiamate**, non le occorrenze. ⛔ **Il comando ovvio è un'altra misura:** `grep -c has_died`
conta anche i paragrafi che dichiarano di non chiamarla — questo compreso — quindi rende un
numero maggiore di zero per un file che non la chiama mai. ⛔ **E QUEL NUMERO NON SI SCRIVE, in
nessuna delle case:** una riga che lo dicesse verrebbe resa falsa dall'atto di scriverla, ed è
successo **due** volte, al compito e alla prima ondata di revisione. ⚠️ **E la seconda direzione,
senza la quale uno zero non misura niente:** il comando **ancorato** dà **6** su
`crates/simulator/tests/dying_gui.rs`, dove chiedere alla finta è la cosa giusta da fare.

⛔ **E LA LINEA DI BASE NON È ZERO, IN DUE SPECIE.** Con base zero *«la somma torna alla linea di
base»* è verde anche per una riconciliazione che rilascia **tutto** (`E156` ②). Quindi i libri
tengono, prima che la gui parli: la **quota di presentazione del core**, che nessun registro
tiene e che ogni corsa **riscuote** dopo la riconciliazione, e un **secondo client registrato che
non muore**. ⚠️ **Il secondo c'è perché è quello che la mutazione può raggiungere:** senza di lui
la campagna avrebbe portato un'asserzione che **nessun difetto raggiungibile** può far diventare
rossa, e `MB2` sotto lo dimostra al contrario.

#### Le mutazioni, col proprio esito MISURATO

⛔ **Applicate una per volta sul codice di PRODUZIONE o sul generatore**, ciascuna compilata ed
eseguita a sé e revocata da una copia byte-esatta presa prima, con i **file nuovi già
nell'indice** (`E146`, gotcha **#107**): `git diff -- <file>` a **zero** righe dopo ognuna, e
`sha256sum` identico al pristino.

| # | Mutazione | Sonda che diventa rossa, e come |
|---|---|---|
| `M1` | `on_disconnect` non rilascia niente | **tre** sonde di `client_grants.rs` — `left: Ok([])` contro `Ok([Now(Mib(1024))])` — e **non** la terza, che resta verde |
| `M2` | `on_disconnect` rilascia **tutte** le concessioni | `a_disconnect_gives_back_only_that_client_s_grants`, **una sola**: `Ok([Now, Now, Now])` contro `Ok([Now, Now])` |
| `M3` | un client sconosciuto è un errore | `a_disconnect_of_a_client_that_holds_nothing_changes_nothing`, **una sola**: `left: Err(UnknownGrant)` |
| `M4` | `AlreadyCollected` scartato invece che riferito | `a_disconnect_after_the_window_reports_already_collected`, **una sola**. ⛔ È la mutazione che prova che la **quarta** sonda non è un doppione della prima |
| `M5` | `drain(..)` **senza** ripartizione | rilascia le concessioni giuste e **butta le coppie di tutti gli altri**. ⛔ Uccide **due** sonde: in `a_disconnect_gives_back_only_that_client_s_grants` non la prima asserzione ma il **secondo `on_disconnect`, quello sul sopravvissuto**, e in `a_foreign_grant_is_the_caller_s_defect_and_leaves_the_rest_registered` l'asserzione *«the failure threw away a grant it had not been asked to release»*. ⚠️ **Le asserzioni si citano per TESTO e non per numero di riga:** questa cella diceva *«riga 161 contro 147»* e nessuno dei due numeri indicava più l'asserzione giusta dopo la prima ondata. ⛔ Gotcha **#55** risolto nella direzione che l'esclude: `M2` ed `M5` colpiscono la stessa sonda su **asserzioni diverse**, quindi assi diversi |
| `M6` | `drain(..)` **con** ripartizione — ⛔ **la forma che `E154` ② PRESCRIVE** | `a_foreign_grant_is_the_caller_s_defect_and_leaves_the_rest_registered`, **e nient'altro**: `4 passed; 1 failed`, `left: Ok([])` contro `right: Ok([Now(Mib(1024))])`. ⛔ **È la misura che rende la deviazione una voce d'errata e non un gusto** — e fino alla prima ondata di revisione **nessuna sonda la uccideva**, cioè la deviazione era dichiarata migliore e non provata |
| `MC1` | `RngExt::below` estrae sempre il **massimo**: ogni kill cade oltre ogni finestra | ⛔ **l'oracolo 2 di `worker_kill_campaign`**: *«no kill ever landed inside a worker's window: every reservation was already swept, so the books were compared against an empty budget on every one of 8000 kills»* |
| `MC2` | `RngExt::below` estrae sempre **0**: ogni kill cade all'origine | ⛔ **il rilevatore di degenerazione di `worker_kill_campaign`**: *«every seed produced the SAME outcome: the drawn instants are not reaching the windows, so this campaign is one run repeated 2000 times»* |
| `MC3` | un kill **saltato** nel ciclo del banco | ⛔ **l'oracolo 1 di `worker_kill_campaign`**, per seme: *«seed 0: a recruit was never killed — 1 of 4 are still running»*. ⚠️ **È una mutazione sul BANCO e non sulla produzione**, ed è il tetto onesto per questa campagna: `E155` toglie la finta guidata dal seme perché *«a decidere il kill è il banco e non il worker»*, quindi qui l'iniezione **è** il flusso di controllo del banco |
| `MB1` | la riconciliazione non avviene | `gui_death_campaign` rossa |
| `MB2` | la riconciliazione rilascia **tutto** | `gui_death_campaign` rossa **sulla somma**: `left: Mib(1024)`, `right: Mib(2048)` — la linea di base **sparita**, che è esattamente ciò che `E156` ② prescrive di cogliere |
| `MB3` | la finta non muore mai | **oracolo 1** rosso: *«the gui was told to die at operation 0 of 3 and the port never said so, so this run injected nothing»*. Uccide anche **cinque** sonde di `dying_gui.rs` |
| `MB4` | il punto di morte estratto è sempre 0, quindi la gui non chiede mai | **oracolo 2** rosso, col testo **rimisurato il 2026-09-02** e non dedotto dal sorgente: *«on no seed did the gui hold a grant when the port reported it gone: every one of the 2000 seeds died before it was granted anything, so the sum was compared against a baseline it never left»*. E il rilevatore di cambiamento con lui: *«the campaign saw 4 of the 12 worlds this scenario can produce»*. ⚠️ **Questa cella citava il testo di PRIMA della seconda ondata**, che aveva tolto il contatore `deaths` e riscritto il messaggio: un esito misurato che nessuna corsa poteva più produrre |
| `MB5` | `release` riferisce la riserva ma non la toglie dai libri | `property_2` rossa col **seme** e i **due valori**: *«seed 0: after killing indexer at Monotonic(1445) the books hold Mib(9216) and the workers still running with an open window reserve Mib(5120)»* |
| `MB6` | il costruttore *«non muore mai»* muore subito | `a_gui_told_not_to_die_never_does`, **una sola** — la direzione che si dimentica (§7.1.1 regola 3) |
| `MB7` | la richiesta attraversa come corpo **vuoto** | `what_the_gui_said_before_dying_is_a_real_encoded_request`, **una sola** |
| `MB8` | il punto estratto può cadere **oltre** l'ultima operazione | `the_drawn_point_lies_inside_the_operations_the_path_performs` e `every_operation_of_the_path_can_be_the_one_that_kills_it` — gotcha **#17** in entrambe le metà |

⛔ **I DUE ORACOLI DI NON-VACUITÀ DI CIASCUNA CAMPAGNA SONO STATI VISTI ROSSI, ed è un criterio
di chiusura e non un consiglio.** Per `gui_death_campaign`: l'oracolo **1** con `MB3`, l'oracolo
**2** con `MB4`. Per `worker_kill_campaign`: l'oracolo **1** con `MC3`, l'oracolo **2** con `MC1`,
e in più il rilevatore di degenerazione con `MC2`. ⚠️ **`MC1` ed `MC2` mutano il GENERATORE** —
`RngExt::below`, codice di produzione in `crates/kernel/src/rng.rs` — **mentre `MC3` muta il
banco**, e la differenza è dichiarata invece che nascosta: `E155` toglie a questa campagna la
finta guidata dal seme perché *«a decidere il kill è il banco e non il worker»*, quindi non
esiste un difetto di produzione che possa far mancare un kill.

⛔ **E L'ORACOLO 1 DELLE DUE CAMPAGNE È UN'ASSERZIONE PER SEME e non un'aggregata.**
⚠️ **RICHIAMO DEL 2026-09-02, QUARTA ONDATA — QUI STAVA LA DIAGNOSI CON CUI LE DUE AGGREGATE
FURONO TOLTE, ED ERA FALSA PER UNA DELLE DUE.** Diceva che `deaths == SEEDS` e
`tally.kills == SEEDS * RECRUITS.len()` *«leggevano un conteggio che il ciclo stesso produceva, e
nessuna mutazione poteva farle rosse»*. ✅ **Misurato:** la prima sì — dopo l'asserzione per seme
`death_seen_from_port` è **costante `true`**, quindi quel contatore vale `SEEDS` per costruzione.
La seconda **no**: sottoposta allo stesso kill saltato di `MC3` legge **6000** contro **8000** e va
**rossa**. ⛔ **A distinguere le due forme non è la falsificabilità** — sotto la sola restrizione
vera, *«nessuna mutazione di PRODUZIONE la raggiunge»*, l'asserzione per seme che l'ha sostituita
è irraggiungibile quanto lei — ma che la forma per seme **nomina il seme** e vale su **ogni** seme
invece che sulla somma. ⛔ **E toglierla ha aperto una lacuna vera, non chiuso una ridondanza:**
niente altro teneva *«ogni recluta è stata uccisa»*, e un kill saltato lasciava l'intero banco
**verde** per due ondate. Il rimedio è l'asserzione di `running.is_empty()` e la sua `MC3`.
⚠️ **Il rosso di
`MB3` era già venuto dall'asserzione PER SEME dentro `run`**, che è più forte — vale su ogni seme
invece che sulla somma, e nomina il seme e l'operazione — e nella seconda ondata sono caduti anche
il contatore `deaths` e il congiunto su `death_seen_from_port`, per la stessa ragione: dopo
l'asserzione per seme quel campo è **costante `true`**, quindi un congiunto su di lui si riduce al
proprio operando sinistro. ✅ **Rimisurato dopo averli tolti, e la cifra NON si scrive qui:** la
riga `DST gui death:` che `cargo test --locked -p simulator --test gui_death_campaign --
--nocapture` stampa porta lo stesso numero di semi prima e dopo la rimozione, e `MB3` ed `MB4`
restano rossi. ⚠️ **Questa riga portava quel numero nudo**, mentre la cella della campagna
sorella enuncia la regola opposta: un numero misurato non si scrive, si scrive il comando.

⚠️ **Una sola sonda non è uccisa da nessuna mutazione — `the_same_seed_chooses_the_same_operation`
— ed è INFALSIFICABILE PER COSTRUZIONE**, `from_seed` essendo una funzione pura dei suoi
argomenti: è dichiarata tale nel proprio corpo (gotcha **#44**), com'è la gemella in
`crashing_journal.rs`, e **non conta come copertura**.

#### Le decisioni prese, e ciò che costano

| | |
|---|---|
| la proprietà **2** prende un **banco proprio** e non un angolo di `arbiter_campaign.rs` | quel file è **uno** scenario e le sue costanti, il suo `Observed` e i suoi due oracoli descrivono **quello**; infilarci un secondo soggetto gli avrebbe dato due scenari sotto un'intestazione che ne descrive uno. ⛔ **Il costo, dichiarato:** è un file nuovo in più (`E155`), ed è un **bersaglio in più** che `scripts/gate.sh` deve nominare, perché il suo ultimo passo nomina le campagne **una per una**. ✅ **RICHIAMO DEL 2026-09-02: questa cella diceva che la riga di tempo NON è raccolta e che `gate.sh` è fuori dall'elenco dei file del compito, e `0976d9f` l'ha resa falsa** — il perimetro è stato allargato dal coordinatore e le due campagne sono **nel** passo 7, con la misura nelle due direzioni scritta nel commento di quel passo |
| il nome della finta è `DyingGui` | participio più cosa, come `CrashingJournal`. `FakeGui` di `crates/kernel/tests/ports_are_implementable.rs` è la **forma** e non l'artefatto: il codice di test non attraversa le crate |
| `on_disconnect` estrae **una coppia per volta** (`position` più `remove`) e non `drain` più ripartizione | `M5` è la misura: su un `Err` il `drain` avrebbe buttato **tutte** le coppie non ancora rilasciate, mentre così restano registrate e un chiamante che torna con l'arbitro giusto non ne perde nessuna. ⛔ La concessione che **produce** l'`Err` è persa comunque — `release` la consuma prima di rispondere — e il limite è dichiarato sul metodo |
| `ClientGrants` tiene un `Vec<(ClientId, Grant)>` a ricerca lineare | `ClientId` non deriva `Ord` né `Hash` — tolti **per sottrazione** in `ports::ipc` — e `HashMap` è vietata (gotcha **#12**). Ridare la derive per comodità di questo file riaprirebbe quella decisione |
| la campagna della proprietà 3 gira a **un solo istante** | il mondo *«la gui muore DOPO la propria finestra»* non è in questo spazio: sta in `a_disconnect_after_the_window_reports_already_collected`. ⛔ **Dichiarato e non taciuto**, e non cambierebbe nessuna asserzione: la somma torna alla base sia che la riserva sia tornata adesso sia che la scopa l'avesse già presa |
| la metà **temporale** di *«concessione valida»* si **conta** e non si asserisce | `Process::start` non prende `now` e non interroga l'arbitro, `GrantId` è privato, e nessuna API risponde *«questa concessione è ancora nei libri?»*. Pinzarla sarebbe un voto contro il prendere `E30`/`E39`, che è del proprietario — gotcha **#73**. Il contatore la **dichiara** invece di asserirla, e la cifra **non si scrive qui**: la stampa la campagna a ogni corsa, e il comando che la produce è `cargo test --locked -p simulator --test worker_kill_campaign -- --nocapture`, la cui riga `DST worker kills:` porta i kill totali, quelli dentro la finestra e quelli oltre. ⚠️ **Questa cella portava i due numeri nudi** — un numero misurato non si scrive, si scrive il comando |

⚠️ **`E50` ed `E51` NON sono chiuse**, e `client.rs` lo dichiara nel proprio doc di modulo:
quel ciclo decide **quando** `promote` gira rispetto ad `admit`; questo non decide niente del
genere — risponde a **un** evento con **un** rilascio.

📌 **Baseline a compito chiuso, revisione compresa: `GATE GREEN`, 48 bersagli, 352 passate,
0 fallite, 2 ignorate** — era **44 / 334 / 0 / 2**. Lo scarto è **+4 bersagli** (`client_grants`,
`dying_gui`, `gui_death_campaign`, `worker_kill_campaign`) e **+18 passate**, tutte dei banchi
nuovi. ⚠️ **La cifra si rilegge col comando e non da qui** —
`cargo test --locked --workspace --no-fail-fast` — ed è **datata al 2026-09-02, seconda ondata**:
scritta a *«351»* e *«+17»* alla chiusura del compito, l'ha mossa la sonda che la prima ondata ha
aggiunto per il ramo `Err`. `cargo fmt --all --check` exit 0, zero avvisi su
`cargo build --locked --workspace`.

## Livello 3 — vuoto, e non è una svista

`clippy` gira come igiene del codice ma **non ha voce nella porta**: nessun V dipende da
lui. Un rosso della porta deve significare sempre «invariante violata», mai «stile
discutibile», o si impara a ignorarlo.

⛔ **`cargo fmt --all --check` è stato valutato il 2026-08-09, e resta fuori per lo stesso
argomento.** La deriva era reale — **una sola** in tutto il workspace, il letterale di struct
di `Parameters::new`, ereditata dal Task 3 — ed è stata corretta **nel file** (commit
`c1aebcc`) invece che sorvegliata dalla porta. La regola 1 di §7.1.1 la rifiuta su
**entrambi** i rami, ed è la stessa coppia di domande con cui quella sezione caccia `clippy`:

| Ramo | La domanda | Su `rustfmt` |
|---|---|---|
| **1a** | difende un `V`, un'`I` o un `Q` nominato? | **no** — nessuna proprietà del sistema dipende da dove va a capo un letterale di struct |
| **1b** | quale riga del catalogo smette di essere **vera**, se lo cancelli? | **nessuna** — stesso esito che su `clippy`, e la §7.4.3 regge parola per parola |

⚠️ **E non sarebbe comunque una decisione da prendere qui.** *«Il livello 3 del catalogo è
vuoto»* sta fra le voci **non rilitigabili** della §7 del [compendio](COMPENDIO.md):
aggiungere un passo di stile alla porta richiederebbe un **ADR che superi la §7.4.3**, non un
commit. ⚠️ `rustfmt` non è nemmeno un lint, ma sulla scala di §7.1.2 si comporta da livello
**3**: si cancella, e si aggira per singolo elemento con una riga — `#[rustfmt::skip]`.

📌 **Il costo accettato, ed è reale:** `cargo fmt --all --check` resta un segnale che
**nessuno fa rispettare**. A tenerlo verde è l'igiene di chi scrive — non un controllo. Chi lo
trova rosso lo corregge nel file.

⛔ **E IL 2026-08-21 ERA ROSSO, CHE È ESATTAMENTE CIÒ CHE QUESTO PARAGRAFO PREVEDEVA.** Diceva
*«Oggi è verde su tutto il workspace»*, e la misura ha dato **ventisette hunk in sei file** —
`arbiter/mod.rs` quattordici, `arbiter_policy.rs` otto, e uno o due ciascuno in
`arbiter_admission.rs`, `arbiter_resource.rs`, `ports_are_implementable.rs` e
`engine_crash_consistency.rs`. ⚠️ **Cinque dei sei sono file del Traguardo 5**: la deriva si è
accumulata dove si lavorava, che è la sola cosa che un segnale non imposto può fare.
✅ **Corretto nel file, come il paragrafo prescrive**, e la baseline non si è mossa: `GATE GREEN`,
**35 target, 248 passate, 0 fallite, 2 ignorate**, zero avvisi di build.

⛔ **E CORREGGERLO HA FATTO SCATTARE LA TRAPPOLA DEI FINE-RIGA, misurata invece che temuta.**
`cargo fmt` è uno strumento che riscrive un sorgente, quindi ne normalizza i fine-riga **senza
dirlo**: su `ports_are_implementable.rs` — uno dei **quattro** file con `CR` **nell'indice** —
i `CR` sono passati da **972 a zero**, e `git diff --stat` dichiarava **1944 righe cambiate** su
un file il cui cambiamento vero sono **due** hunk. Stessa cosa su
`engine_crash_consistency.rs`, `751 → 0`, invisibile nel diff solo perché il suo blob
nell'indice è già LF. ✅ **Ripristinati da una copia byte-esatta presa PRIMA** — mai da
`git checkout --`, che avrebbe cancellato il lavoro non committato (gotcha **#48**, dodicesima
forma) — e rimisurati: `972` e `755`, `git diff` torna a **due** righe, e il censimento
`git ls-files --eol` è **immutato**: `140 i/lf w/crlf · 75 i/lf w/lf · 4 i/crlf w/crlf ·
3 i/-text w/-text`. 📌 **La riga di `CLAUDE.md` nomina `sed -i`; lo strumento colpevole qui è
`cargo fmt`, e la regola vale per QUALUNQUE strumento che riscriva un file.**

⛔ **E IL 2026-08-25 LA TRAPPOLA È SCATTATA DA UN TERZO STRUMENTO, CHE NON RISCRIVE
NIENTE DI SUO: `git stash` e `git stash pop`.** Nella prima ondata di correzioni del Task 12
sono stati usati per misurare se una deriva di `cargo fmt` fosse anteriore alla modifica in
corso; la coppia **rimaterializza** il file dall'indice attraverso il filtro `smudge`, e con
`core.autocrlf=true` lo restituisce **CRLF**. ⚠️ **Il diff non lo mostra**, perché il filtro
`clean` rinormalizza in scrittura: a spostarsi è il **censimento**, e a spostarsi in
silenzio. ✅ **Misurato invece che temuto:** `crates/simulator/tests/arbiter_campaign.rs` era
`i/lf w/lf` con **zero** `CR`, e dopo la coppia `git ls-files --eol` dava **141** `i/lf
w/crlf` e **84** `i/lf w/lf` invece di 140 e 85. Rimesso a LF con uno script in modalità
binaria e ricontato: il censimento è tornato **immutato**. 📌 **La regola non cambia,
cambia chi la fa scattare:** è la terza forma, e le prime due riscrivevano il file di loro
iniziativa mentre questa lo rimette a posto *da git*, che è il caso che non si sospetta.

## ⛔ LE VOCI APERTE DEL TRAGUARDO 5, IN UNA TABELLA SOLA (2026-08-25)

⛔ **Raccolte qui alla chiusura del traguardo, e la ragione è quella che la tabella
consolidata delle sonde dell'audit scrive di sé:** più voci aperte sullo stesso oggetto sono
il modo in cui una di esse smette di esserlo senza che nessuno l'abbia chiusa. Le voci del
Traguardo 5 **vivono** in **sei riquadri** di questo file, più l'errata in testa al
[piano](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) e i riquadri
della §6 del [compendio](COMPENDIO.md): questa tabella li **indicizza**, non li sostituisce —
il registro è append-only per costruzione.

⛔ **RICHIAMO DEL 2026-08-25 — questa riga diceva *«VIVEVANO … alle righe `249 · 594 · 604 ·
853 · 887 · 918`»*, e le due cose erano sbagliate insieme.** Il passato: i sei riquadri erano
intatti — `git diff --stat` su questo file dava **`+103`, `−0`** — quindi la tabella si era
**affiancata** e non sostituita, e il rimando esisteva **solo** nella direzione tabella →
riquadro. Chi arrivava al riquadro vedeva una voce aperta isolata, non sapeva che esistesse un
indice, e chiudendola lì lasciava viva e falsa la riga corrispondente: la **lettera** della
condizione 8 era soddisfatta, lo **scopo** no. ✅ **Ora ciascuno dei sei nomina questa
tabella**, e la colonna *«Dove è dichiarata»* li nomina **per titolo**: i numeri di riga sono
**tolti**, perché questo file cresce e un numero di riga è un puntatore senza guardia — è la
stessa cura che lo stesso giorno ha tolto *«la riga 16»* da [`README.md`](README.md).

⛔ **Come è stata costruita, perché il metodo vale più dell'elenco: non è un censimento
dichiarato completo, è ciò che questi comandi hanno restituito**, letti riga per riga e senza
troncarli (gotcha **#70**).

```
$ grep -n "VOCE APERTA\|VOCI APERTE" docs/porta-di-qualita.md
    → UNDICI occorrenze prima di questa raccolta: le SEI del Traguardo 5 alle righe
      qui sopra, DUE del Traguardo 3 (2573 e 2592), UNA che apre la tabella consolidata
      dell'audit (2744) e DUE che vi rimandano (2858 e 2942).
    ⚠️ Rieseguito DOPO l'inserimento ne dà TREDICI: l'intestazione di questa sezione e
      la riga di comando qui sopra sono esse stesse due case del grep che censisce. È la
      stessa nota che la §12 del compendio porta sul censimento del puntatore.

$ grep -cE '^\| \*\*`?E[0-9]+' <piano>
    ⛔ QUANTE SIANO NON È SCRITTO QUI: il commit che ha scritto questa sezione ne
      aggiunge TRE, e un totale che il proprio commit sposta non può essere verde in
      nessun mondo. Il numero lo dà il comando, sul piano che si sta guardando.
    ⛔ IL FILTRO CHE LE RESTRINGE È STATO PROVATO E NON BASTA, ed è il gotcha #70 in
      ENTRAMBE le forme. Il filtro
        "REGISTRAT[AE], NON PRES|NON PRESA|REGISTRATE E NON PRESE|DICHIARAT[AO], NON
         PINZAT|DICHIARATA E NON|per il proprietario|decisione del proprietario"
      lanciato sul piano COM'ERA ALLA RACCOLTA — git show ac65504:<piano> — ne dà 23:
        E10 E21 E30 E31 E32 E41 E47 E50 E51 E62 E67 E70 E72 E74 E94 E104 E121 E122
        E129 E137 E144 E146 E151
      Rilette una per una: DODICI sono ancora aperte ed entrano in tabella; UNDICI sono
      già chiuse da un compito successivo — E10, E41, E62, E67, E72, E74, E121, E122,
      E129, E137, E144.
    ⛔ RICHIAMO DEL 2026-08-25 — QUI ERA SCRITTO «ne dà VENTICINQUE», con E52 ed E140
      dentro l'elenco e i derivati «TREDICI aperte, DODICI chiuse», e IL COMANDO NON LO
      RESTITUISCE. Provate una per una, nessuno dei sette termini tocca E52 o E140:
      grep -icE per termine dà 0 su entrambe. Rilanciato dà 23, e i due derivati sono
      12 e 11. ⚠️ Rilanciato a HEAD dà 24, perché lo stesso commit aggiunge E153, che il
      filtro cattura: è la ragione per cui questa riga porta lo SHA della raccolta e non
      «il piano». ⛔ E la §12 del compendio lo elencava fra i valori «corretti
      RIESEGUENDO»: dichiarare di aver rieseguito è gratis, il comando accanto alla
      cifra no. La cifra vive ORA SOLO QUI, dove chi la contesta rilancia il comando.
    ⛔ E ne MANCA CINQUE che sono aperte: E53, E83, E100, E140 ed E152, misurate a zero
      contro quello stesso filtro. Trovate leggendo la §6 del compendio e i sei
      riquadri. ⚠️ E140 era contata FRA quelle che il filtro restituisce e insieme fra
      le aperte in tabella: la cifra scritta senza rilanciare sbagliava in entrambe le
      direzioni, e le CONCLUSIONI reggevano lo stesso — è il METODO a non riprodursi.
    ⛔ RICHIAMO DEL 2026-09-02 — E152 NON È PIÙ APERTA: la chiude il compito 9 del
      Traguardo 6, riga 27 della tabella qui sotto, e delle cinque quattro lo restano:
      E53, E83, E100 ed E140. ⚠️ IL VERBALE SOPRA RESTA COM'È PERCHÉ NON PARLA DI CIÒ
      CHE È APERTO OGGI: parla del FILTRO e del suo punto cieco, misurato contro
      `git show ac65504:<piano>`, e quel punto cieco non si è mosso — il filtro rende
      zero su E152 tanto ieri quanto oggi. Ciò che invecchia è il PRESENTE «sono
      aperte». ⚠️ E QUESTO BLOCCO STAVA IN MEZZO A QUELLA FRASE, fra «misurate a zero»
      e «contro quello stesso filtro», mentre dichiarava di lasciare il verbale com'è:
      spostato in coda al capoverso nella prima ondata di revisione.

$ grep -nE "riga di catalogo" docs/porta-di-qualita.md | grep -iE "non ha|non hanno|nessuna riga"
    → DIECI righe candidate sul file com'era prima di questa raccolta, e le case sono
      QUATTRO. 258 e 260 sono lo STESSO sito — intestazione e corpo; 941 e 1720 sono due
      siti; 2858 è dell'audit e sta nella tabella consolidata.
    ⚠️ Le altre SEI sono falsi positivi, e vanno dette perché sono la metà del #70 che
      costa: 2232 parla dei permessi di un file su Unix, 3300 è un verbale del finding
      P-2, e 3391 · 3395 · 3397 sono celle di «Cosa la porta NON controlla» dove il
      «non ha» governa un'altra proposizione. Il primo conteggio scritto qui diceva
      SETTE: era il risultato di un filtro DIVERSO, rilanciato invece che riletto.
```

➕ E il punto ④ del riquadro *«ciò che il Task 12 lascia al Task 13»* della §6 del
[compendio](COMPENDIO.md), che è il censimento autoritativo delle voci ereditate: **ricontato
qui contro l'errata e la §6, non ricopiato**.

⚠️ **Ciò che questa tabella NON copre, detto perché non la si legga per più di quello che
è.** Le voci aperte del **Traguardo 3** vivono nella tabella *«Cosa il Traguardo 3 lascia
aperto»* della §6 del [compendio](COMPENDIO.md) — e con esse la **guardia che manca al
puntatore** del prossimo passo, l'elenco dei semi e la guardia sui pesi, che sono voci di
**quel** traguardo; le **dieci sonde permanenti dell'audit** vivono nella tabella consolidata
del 2026-08-18 qui sopra, e la riga che le nomina qui è un **rimando** e non una seconda copia
(gotcha **#68**).
⚠️ **E una voce che questa tabella NON indicizza, detta il 2026-08-25 per non farla cercare:**
*«[`semi-dst.md`](semi-dst.md) non ha un chiudente»*, riga di *«Cosa questo piano lascia
aperto»* col chiusore **il proprietario**. È eredità del **Traguardo 4** e non del 5, cioè
fuori dal perimetro di questa tabella; resta dichiarata nel riquadro di `semi-dst.md` e
ridichiarata dal richiamo scritto lì alla chiusura. **Vale una riga di esclusione, non una
riga di tabella** — ma tacerla la rendeva indistinguibile da una voce dimenticata.

⛔ **La colonna «Chi la chiude» è la ragione per cui la tabella esiste, e non dice sempre
«il proprietario».**

| # | Voce | Che cosa resta aperto — rimisurato il 2026-08-25 | Dove è dichiarata | Chi la chiude |
|---|---|---|---|---|
| 1 | **R1** | `WorkDescriptor` (`crates/kernel/src/arbiter/resource.rs:164`) dista **una lettera** da `WorkerDescriptor` (`crates/kernel/src/ports/process.rs:76`), che è un'altra cosa | pre-controllo del piano, e §6 del compendio | il **proprietario**: un rinomino |
| 2 | **R3** | la riga di catalogo `Q2 · §5.1` è **una** e formulata in **una** direzione — *«MiB assegnati a millisecondi»* — mentre i casi che mordono sono **quattro**, su due regole e in due direzioni | riquadro *«la riga di catalogo `Q2 · §5.1` è UNA e in UNA direzione»* di questo file | il **proprietario**: §7.4 è spec, vincolo globale 7 |
| 3 | **R4** | `Preemption::{Never, After(Millis)}` è **una** voce dove la tabella dei campi di §5.2 ne elenca **due**, `preemptible` e `release_grace` | pre-controllo del piano, e §6 del compendio | il **proprietario** |
| 4 | **R5** | due celle del catalogo nominano `Concessa`, `InCoda` e `InRevoca`; dal Task 4 il codice scrive `Admission::{Granted, Queued, Refused}` e `PreemptibleState::Revoking`. ⛔ **RICHIAMO DEL 2026-08-27, finding AUD-036 — IL PERIMETRO DI QUESTA VOCE ERA PIÙ STRETTO DEL REALE, e un perimetro stretto è peggio di una voce assente: fa credere che il resto sia già stato guardato.** La colonna «Dove è dichiarata» diceva *«celle `V4` e `I2 · §5.3` di questo file»*, e il censimento non aveva mai toccato i **nove documenti di design**, che sono quelli che si dichiarano fonte di verità. ⚠️ **Misurato il 2026-08-27 col comando, non a memoria:** `grep -noE '`(Concess[ao]|Rifiutat[ao]|InCoda|InRevoca|InDubbio|Annullat[ao]|interattivo|verificabile|idempotente|irripetibile)`' docs/design/*.md` — le case sono in `design/02`, `design/03` e `design/07`, e il conto lo rifà il comando perché una cifra qui invecchierebbe al primo rimedio. Più i **nomi di stato nudi** dentro i blocchi mermaid di `design/01` e `design/02`, che sono un'altra specie: etichette del modello, non riferimenti in backtick. ✅ **UNA sola traduzione è stata fatta, e con la propria ragione:** `interattivo` → `interactive` in `design/02`, dove le altre due voci della **stessa enumerazione** erano già inglesi e la §5.5 della spec scrive `interactive` — lì si chiudeva un **dialetto misto dentro un'enumerazione sola**, che la §4 del compendio chiama *«la condizione peggiore delle due»* | celle `V4` e `I2 · §5.3` della tabella «Livello 1» di questo file, **più** il richiamo del 2026-08-27 accanto alla tabella del profilo in [`design/02`](design/02-arbitrato-gpu.md); il censimento vivo lo dà il comando qui accanto | il **proprietario**: §1.0 contro §7.4, e ora anche contro il **vocabolario degli ADR**. ⛔ **Ed è per questo che il rimedio si è FERMATO PRIMA DI DECIDERE:** tradurre `Concessa`, `InRevoca`, `InDubbio` e le tre classi d'effetto tocca parole che [ADR-0005](adr/0005-arbitrato-gpu-su-due-dimensioni.md), [ADR-0007](adr/0007-giornale-write-ahead-e-riconciliazione.md) e la **spec approvata** scrivono in italiano di proposito — la §4 del compendio dichiara **accettata** la traduzione fra la parola di un ADR e il nome nel codice. Non è un allineamento, è una **scelta di convenzione** |
| 5 | **E140** | il catalogo scrive `uccidi` in §7.4.1 blocco C, in §6.10.2 e in §6.10.5; il sorgente scrive `fn kill` in `crates/kernel/src/ports/process.rs` | cella `I2 · §6.10` di questo file, ed errata del piano | il **proprietario**: stessa specie di `R5`, ma nata col Traguardo 2 |
| 6 | il costruttore di `Grant` | `trybuild` prova la direzione **da fuori la crate**, e il caso lo dichiara nel proprio commento: da **dentro** la crate un costruttore `pub(crate)` resterebbe fuori dalla sua portata. ✅ Rimisurato: `grep -rn "impl Grant" crates/kernel/src/` non restituisce niente, e l'unico sito che ne conia una è `Arbiter::issue`, privato | commento di `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs` | il **proprietario**: sarebbe una **riga di catalogo nuova** |
| 7 | la contro-sonda di `Q8 · §5.2.1` | la cella nomina *«la proiezione di presentazione lo legge»*. ✅ Rimisurato: `grep -rn "cold_start" crates/ --include=*.rs` dà come lettore `cold_start_is_readable_outside_the_decision_path` in `crates/kernel/tests/arbiter_resource.rs`, che questo registro dichiara **una finta** | disegno del Traguardo 5, e cella `Q8 · §5.2.1` di questo file | il **proprietario**: riformulare la cella è §7.4 |
| 8 | la divergenza su §5.1 | §5.1 dice *«I tre addendi sono parametri consegnati»*; `crates/kernel/src/parameters.rs` porta `executor_turn_limit` e `total_vram`, cioè **un** addendo su tre — gli altri due sono la riserva di due **concessioni permanenti** | disegno del Traguardo 5, e §6 del compendio | il **proprietario** |
| 9 | il **quinto** caso `compile_fail` | un secondo `start` con lo stesso `Grant` è `E0382`, perché `Grant` non deriva `Copy` né `Clone`: **misurato e non preso** | §6 del compendio, voce ① del Task 11 | il **proprietario**: se pretenda una riga propria lo decide §7.4 |
| 10 | la **convenzione** sulla ricevuta | *«leggere da un worker ← una ricevuta»* entra fra le **coperte** del blocco B, ma `SingleReceipt::new` è `pub`: il caso prova l'**arità** e non l'**autenticità**, mentre `Q8 · §5.2.1` e `V3` sono state tenute a PARZIALE per lacune più strette | §6 del compendio, voce ② del Task 11, e cella blocco B `I5 · Q4` | il **proprietario**: è una domanda di **coerenza**, e va decisa vedendola |
| 11 | le sonde dei compiti **senza riga di catalogo** | ai siti che il terzo `grep` qui sopra restituisce, un compito dichiara che le proprie sonde non hanno una riga propria e vivono sotto una riga altrui | il **blocco dei comandi** qui sopra, terzo `grep` | il **proprietario**: §7.4 è spec |
| 12 | le **dieci sonde permanenti dell'audit** | nessuna ha una riga nel catalogo §7.4 — ⚠️ **questa riga è un rimando**, e il contenuto sta nella tabella consolidata del 2026-08-18 qui sopra | tabella consolidata dell'audit, in questo file | il **proprietario**, dal 2026-08-18 |
| 13 | **E21** | ✅ **CHIUSA dal compito 1 del Traguardo 6, `c4cf942`.** L'identità è `ArbiterId`, **consegnata** in `Parameters` e mai coniata (§6.1.3, ADR-0034), e `release` la confronta **prima** di guardare i propri libri. ✅ **La sonda che questa voce dichiarava debole è ora portante, misurato:** mutando `arbiter_id()` a una costante, `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` è **l'unica** che muore in tutto il workspace. ⛔ **E il banco condivideva un letterale che la rendeva a metà vacua** — voce `E8` dell'errata del piano del Traguardo 6. **Era:** `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` prova *«non è nei miei libri»*, non *«distinguo le mie concessioni da quelle altrui»*: `GrantId` è un progressivo che riparte da zero per ogni `Arbiter` | riquadro *«ciò che `release` compra davvero, e ciò che non compra»* di questo file, e accanto a `ReleaseError` nel sorgente | il **proprietario**: dare un'**identità** all'arbitro |
| 14 | **E30** | ✅ **CHIUSA dal compito 1 del Traguardo 6, `9ecc13d`.** `release` risponde `Result<Released, ReleaseError>`, e `Released` ha **due** vie — `Now(Mib)` e `AlreadyCollected`. Una concessione **propria** non è mai un `Err`, comunque sia andata la sua finestra; `UnknownGrant` significa ora **una** cosa sola: la concessione di un altro arbitro. ⚠️ **Due ingressi che il doc dichiarava misurati non li tiene nessuna sonda** — `release` al confine esatto e i due lati della grazia: voce `E12` dell'errata, **registrata e non presa**. **Era:** ✅ **DECISA NEL MERITO IL 2026-08-28, VINCOLATA NELLA FORMA.** `release` **non** risponde `Err` a una concessione **propria** — finestra scaduta e grazia scaduta non sono fallimenti del rilascio; solo la concessione **altrui** resta un errore. ⛔ **Due forme sono chiuse come scartate**, col perché e coi costi rimisurati. ⚖️ **Resta il TIPO esatto della risposta**, che discende da ciò che `Worker::kill` chiede: si disegna **insieme a `R6`**, riga 26 | accanto a `ReleaseError` in `crates/kernel/src/arbiter/mod.rs`, e riquadro *«`release` risponde `UnknownGrant` anche a una concessione PROPRIA ma SCADUTA»* di questo file | il **Traguardo 6, insieme a `R6`** — ⛔ **non sbarra più l'apertura del traguardo:** ciò che doveva essere deciso prima lo è |
| 15 | **E31** | `saturating_add` può produrre **sovra-ammissione** al limite superiore: con `ceiling = u64::MAX` un secondo `admit` da 1 MiB torna `Granted` | errata del piano | il **proprietario** |
| 16 | **E32** | `crates/kernel/src/parameters.rs` e `crates/kernel/src/arbiter/mod.rs` sono **mutuamente dipendenti** — legale in Rust, sono moduli, ed è dettato dal piano | errata del piano | il **proprietario** |
| 17 | **E47** ① | `promote` restituisce un `Vec<Promotion>` senza `#[must_use]`, e `arbiter.promote(now);` da solo compila anche con `-D warnings` | riquadro *«`promote` restituisce un `Vec<Promotion>` senza `#[must_use]`»* di questo file, ed errata | il **proprietario** |
| 18 | **E53** | due frasi di doc che nessuna sonda tiene, cioè **due mutanti vivi dichiarati**: sono i paragrafi con cui `E50` ed `E51` furono scritte nel sorgente | errata, e accanto ai due paragrafi nel sorgente | il **proprietario** |
| 19 | **E70** | dentro una corsia la vittima è la **più vecchia**, e §5.3, §5.3.1 e `design/02` tacciono: *«la più piccola che basta»* è altrettanto difendibile | errata, e accanto alla frase nel sorgente | il **proprietario**: è una politica senza risposta giusta |
| 20 | **E94** | la policy è un **secondo valore consegnato** ad `Arbiter::new`, mentre §2.8.2 e ADR-0034 parlano di **un** valore che porta i parametri risolti | errata del piano | il **proprietario** |
| 21 | **E104** | una **dominanza** fra sonde della campagna di mutazione, dichiarata e **non cancellata**: una campagna è un campione, non una dimostrazione | errata del piano | il **proprietario** |
| 22 | **E151** | mutante vivo `M9`: tolta ad `Arbiter::release` la riscossione delle scadute, l'intero workspace resta verde — **dichiarato e non pinzato**, perché la sonda che lo ucciderebbe congelerebbe la scelta che `E30` mette davanti al proprietario (gotcha **#73**) | errata, e campagna di mutazione del Task 12 in questo file | il **proprietario** |
| 23 | l'**aiutante** dei due scrittori di record | se `Untrusted::promote` e `Arbiter::set_policy` debbano condividere un aiutante che tenga in passo il `kind` e l'operazione; intanto ciascuna ha la **propria** sonda | §6 del compendio, e `crates/kernel/src/reconcile.rs` | il **proprietario** |
| 24 | **E50** | fra corsie `promote` **scavalca**, cioè fa nell'insieme delle corsie ciò che il suo stesso commento rifiuta dentro una corsia | riquadro *«LO SCAVALCAMENTO CHE `promote` RIFIUTA DENTRO UNA CORSIA, FRA CORSIE LO FA»* di questo file, ed errata | **chi costruirà il primo ciclo di orchestrazione** |
| 25 | **E51** / **E100** | `admit` non consulta mai la coda, quindi un ritardatario la scavalca; dal Task 8 l'inversione di priorità è **raggiungibile in produzione** e non più teorica | riquadro *«`admit` NON CONSULTA MAI LA CODA»* di questo file, ed errata | **chi costruirà il primo ciclo di orchestrazione** |
| 26 | **R6** | ✅ **CHIUSA dal compito 1 del Traguardo 6, `822db6d`.** `Worker::kill` restituisce `Killed`, che porta il `Grant` **fuori** da ogni `Result`, perché la riserva è un fatto dei **libri** e non della salute del processo. ⛔ **E con essa è entrata la via che nessuno aveva discusso:** `Process::start` risponde `Started`, il cui ramo `Rejected` **riporta indietro la concessione** — prima, un avvio fallito lasciava cadere una riserva che nessuno poteva ricostruire, e solo la spazzata la recuperava a finestra scaduta. ⚠️ **Il costo vero era in UNDICI implementazioni della porta e non in sei** — voce `E3` dell'errata. **Era:** `Process::start` **consuma** il `Grant` e `Arbiter::release` lo consuma pure, quindi chi avvia un worker non ha più nulla da rilasciare. ⛔ **È LA STESSA RIGA DI CODICE DI `E30`**, riga 14, **e dal 2026-08-28 le due si chiudono insieme:** `E30` fissa che cosa `release` **promette**, `R6` porta il **chiamante** da cui discende il tipo | accanto a `Grant` nel sorgente, e §6 del compendio | il **Traguardo 6**: la via scritta è che `Worker::kill` restituisca la concessione |
| 27 | **E152** | ✅ **CHIUSA dal compito 9 del Traguardo 6** — `9342fc1`, che porta la riconciliazione alla disconnessione (`kernel::client::ClientGrants`, finding **P-16**), e il commit che porta questa riga, che porta le due campagne. La riga di catalogo di livello 2 **passa da PARZIALE a coperta** e le **cinque** proprietà di §5.7 sono nominate una per una nella cella della campagna DST di questo file. ⛔ **Diceva:** *«la riga di catalogo di livello 2 è PARZIALE e non coperta: §5.7 elenca cinque proprietà e la campagna dell'arbitro ne tiene tre»*, e il chiusore atteso era *«il Traguardo 6, che porta `process` e `ipc`»* — è quello. ⚠️ **La campagna dell'arbitro ne tiene ancora tre, ed è giusto così:** le altre due non girano dentro l'esecutore e hanno banchi propri, `crates/simulator/tests/worker_kill_campaign.rs` e `crates/simulator/tests/gui_death_campaign.rs` | cella della campagna DST di questo file, ed errata | **chiusa** |
| 28 | la classe d'effetto della transizione di policy | è `Idempotent`, e la transizione oggi **scambia un oggetto**: la classe si rilegge quando la transizione avrà un contenuto | §12 del disegno del Traguardo 5, e il sorgente | **L2**, quando arriverà il contenuto dello sfratto |
| 29 | **E83** | il `filter` sull'ammissibilità in `lanes` è un **mutante vivo garantito**, a comportamento nullo: toglierlo lascia il workspace verde, e resta perché rende vera **per costruzione** la frase che i due insiemi sono lo stesso insieme | errata, e accanto alla frase nel sorgente | ⛔ **nessuno**: non c'è niente da decidere, e la non-difendibilità è scritta con la misura |
| 30 | **E146** | [`riferimenti.md`](riferimenti.md) non è stato toccato in tutto il traguardo: le misure vivono **qui**, accanto al controllo che difendono. ✅ Rimisurato il 2026-08-25: `git rev-list --count dc6ac4c~1..HEAD -- docs/riferimenti.md` dà **zero** | errata, e §6 del compendio — allargata a ogni compito dal Task 5 in poi | il **proprietario**: scegliere fra *«spostare le misure»* e *«cambiare la regola»* |
| 31 | i **fine-riga**, misura nuova su una regola già presa | `git ls-files --eol` dice che **nell'indice** i file tracciati sono LF, compresi quelli che nell'albero di lavoro sono CRLF: `core.autocrlf` vale `true`, quindi il `diff` è protetto più di quanto i documenti dichiarino | §6 del compendio, dal 2026-08-20 | il **proprietario**: riaprire una decisione presa |
| 32 | la transizione **`InCoda --> Annullata`** | la macchina a stati di [`design/02`](design/02-arbitrato-gpu.md), che la §5.3 della spec adotta come propria, dichiara che dalla coda si esce anche per **annullamento o scadenza**: nell'arbitro non esiste nessun meccanismo. ⚠️ **Misurato il 2026-08-27**, non dedotto: `grep -rniE 'cancel|annull' --include=*.rs crates/kernel/src/` non ha **nessun** riscontro; gli unici punti che mutano `queues` sono `enqueue`, `promote` e `new`, e `collect_expired` fa `retain` **solo** su `held`. Un biglietto consegnato è quindi **immortale**, mentre `design/02` promette all'utente *«l'opzione di annullare»*. ⛔ **E la conseguenza ha già costato codice:** `StartupError::ReservedQuota` esiste nella radice di composizione perché la seconda quota permanente torna `Queued` e nessuno la servirà mai — un tampone per **un** caso, mentre il buco resta per ogni altro chiamante | il richiamo del 2026-08-27 accanto alla macchina a stati in [`design/02`](design/02-arbitrato-gpu.md), col rimando in [`design/01`](design/01-topologia-dei-processi.md) | il **proprietario**: la scelta è fra **costruire** l'annullamento e **togliere** la transizione dal diagramma, e nessuna delle due è dell'agente. ⚠️ Voce nata dal finding **AUD-044**, che non lamentava il meccanismo mancante ma l'**arretrato anonimo**: la §9 del disegno del Traguardo 5 apre con *«ogni riga ha un indirizzo»*, e questo ramo non stava né fra le cose fatte né fra quelle rimandate |
| 33 | **C-1** | ✅ **CHIUSA il 2026-08-31, e la scelta è del proprietario: `bincode` 2.0.1 RESTA, §6.1.1 non si riapre.** ⛔ **Decisa contro l'evidenza e non attorno:** la compatibilità del fork è stata **misurata prima** (**M-12**) e **regge** — quindi il «no» non è un'omissione. Cinque ragioni, e l'ultima è quella che decide: ① l'avviso dice **non mantenuto**, non **rotto**, e nessuna versione corretta esiste perché il monte dichiara la 2.x **completa**; ② I4 rinuncia al versionamento e il canale è **privato**, quindi il formato è congelato **per disegno** — una libreria finita è ciò che quel canale chiede — mentre ADR-0031 esiste per tenere **piccolo** il grafo dentro I3, e il fork lo farebbe crescere di **una voce netta**; ③ restare lascia un debito **dichiarato**, adottare ne creerebbe uno **nuovo e silenzioso** — compatibilità misurata **oggi** su cinque casi, manutentore solo, e **nessun controllo** che ci direbbe di una rottura futura; ④ *novità non è maturità*, e **RustSec non raccomanda il fork**: le quattro alternative che l'avviso nomina sono cadute, ciascuna con la propria misura; ⑤ ⛔ **la radice di C-1 non è questa crate.** C-1 nomina il **buco fra due criteri** — nessuno chiede come stia la libreria al **nostro** capo — e sostituire **una** libreria cura una crate lasciando il buco aperto per le altre **sette**. ⚠️ **IL RESIDUO È NOMINATO E NON CHIUSO:** la cura alla radice è la voce **X-3** dell'audit del 2026-08-27, *nessuna scansione degli avvisi*, e resta **del proprietario** perché aggiungere un passo al cancello è il vincolo globale 7. Nominarla è **parte** di questa decisione, mai un suo sostituto. **Era:** ⚠️ **Voce del Traguardo 6 e non del 5**, messa qui perché questa è la **tabella unica**: una voce aperta che si apre una casa propria è il modo in cui smette di essere aperta senza che nessuno l'abbia chiusa. ✅ **La MISURA è fatta il 2026-08-31**, dal compito 3bis del Traguardo 6 e da fonti primarie: RUSTSEC-2025-0141 è **ancora attivo e non ritirato**, il monte di `bincode` è **archiviato** dal 2025-08-15, e l'ultima versione pubblicata è il segnaposto `compile_error!`. ⛔ **Ciò che resta aperto è la SCELTA e non la misura:** esistono alternative **mantenute** — `bincode-next` 3.1.1 dichiara **lo stesso formato sul filo**, quindi il pari resterebbe `bincode-ts`, immutato dal 2025-07-17 e già misurato conforme da M-11 — e adottarne una tocca la tabella di §6.1.1, la lista di §7.3.1 e la riga di `scripts/gate-deps.sh` sul grafo transitivo. ⛔ **RICHIAMO DEL 2026-08-31: qui stava *«la compatibilità è DICHIARATA dal candidato, non misurata da noi»*, ed è la MISURA M-12 ad averla chiusa** — decisione del proprietario, *misurare prima di scegliere*. ✅ **La compatibilità sul filo è VERA:** cinque casi byte per byte identici, andata-e-ritorno incrociata sui valori, e il pari `bincode-ts` che legge i byte del fork coi valori giusti su Node 24 — non-vacuità provata nelle due direzioni. ⛔ **E la misura ha portato due COSTI che la voce non aveva:** il grafo spedito di ADR-0031 crescerebbe di **una voce netta** (`unty` → `unty-next` + `rapidhash`), e una compatibilità misurata **oggi** non è una garanzia sulle **versioni future** del fork. ⚖️ **Resta aperta la SCELTA e non più la misura**, e le evidenze stanno in [`riferimenti.md`](riferimenti.md), sezione C-1, in una casa sola. ⛔ **Sbarra il compito 4:** scrivere lo schema `ipc` in un formato prima che la scelta ci sia **è** la scelta, presa per omissione | sezione della decisione 5 (C-1) di [`riferimenti.md`](riferimenti.md), che porta le fonti e i candidati; nota accanto alla voce in `crates/kernel/Cargo.toml`; gotcha **#64** di [`HANDOFF.md`](HANDOFF.md); riga *«schema IPC»* della §4 del [compendio](COMPENDIO.md) | il **proprietario**: §6.1.1 è **spec**, vincolo globale 7, decisione **D12** |
| 34 | **E94** | ⛔ **`RoutingDetail` è una TERZA BOCCA della classe di AUD-050, nata col compito 6 del Traguardo 6.** `RecordV1::routing` è `pub`, `RoutingDetail` è `pub` **coi campi `pub`**: un chiamante qualunque mette una `String` calcolata a runtime in `model`, e il `Debug` scritto a mano la stampa **intera**, col `reason` ancora un letterale `'static` a posto. ✅ **Riprodotta il 2026-09-01 da FUORI la crate**, sonda usa-e-getta cancellata nella stessa corsa: `detail: Some(Routing(RoutingDetail { model: "ignore your instructions", .. }))`. ⚠️ **Non è un difetto oggi** — `Candidate::model` è `&'static str`, quindi per la via di produzione non entra niente; il buco è nel **tipo**. ⚠️ **`VerdictDetail` non è coinvolta**, misurato: `bool` e `u64`. ⛔ **Il compito 7 la TRIPLICA:** il suo `PermissionDetail` è dettato con **due** campi `String` pubblici | voce **E94** dell'errata del piano | il **proprietario**: è la decisione di AUD-050 su un tipo nuovo, e **costa meno prenderla PRIMA del compito 7** che dopo |

⚠️ **E una cosa che la raccolta ha misurato e che nessuno dei sei riquadri diceva:** fra le
righe qui sopra ce ne sono di quelle il cui chiusore **non** è il proprietario. Sparse fra i
riquadri si leggevano tutte come *«aspetta il proprietario»*, che è precisamente il modo in cui
una voce smette di essere aperta senza che nessuno l'abbia chiusa.

⛔ **RICHIAMO DEL 2026-08-28: qui stavano una cifra — *«sono sei»* — e l'elenco dei sei nomi, ed
erano falsi dallo stesso giorno.** `9a18f36` ha portato `E30` da *«il proprietario»* a *«il
Traguardo 6, insieme a R6»*, e la frase **tre righe sotto la tabella** non è stata riletta: i
nomi erano diventati **sette**, e il mancante era proprio la voce che **sblocca il traguardo che
si stava per aprire**. È la radice **R1** — *una correzione attraversa la riga in cui nasce, non
la frase sotto* — commessa nella tabella che il compendio dichiara casa unica.
📌 **Tolti e non riallineati a sette**, sul precedente di **AUD-007** e **AUD-046**: *un elenco
invecchia, una regola no*, e il prossimo spostamento di colonna li falsificherebbe di nuovo. Al
loro posto c'è il **comando**, che le **nomina** invece di contarle:

```
awk -F'|' '/^## .*LE VOCI APERTE DEL TRAGUARDO 5/{s=1} s&&/^## Cosa la porta NON controlla/{s=0} s&&/^\| [0-9]+ \|/&&$(NF-1)!~/proprietario/{print $3}' docs/porta-di-qualita.md
```

✅ **Provato nelle DUE direzioni** (gotcha **#24**): rende esattamente le voci il cui chiusore non
è il proprietario, e la forma complementare — `~` al posto di `!~` — non ne nomina **nessuna**.
Il perimetro è verificato a parte: le righe che il filtro attraversa sono tutte e sole quelle
della tabella.
⚠️ **È ancorato alle due INTESTAZIONI e non a numeri di riga**, come il resto di questa sezione
dal 2026-08-25: se una delle due cambia il comando **tace**, e un silenzio si nota meglio di una
cifra sbagliata. ⚠️ **E l'ancora non può portare l'emoji dell'intestazione**: `⛔` è multi-byte e
un `.` di `awk` ne copre **un byte solo** — misurato scrivendo questo comando, la prima forma
rendeva zero righe.

## Cosa la porta NON controlla, in questo traguardo

Righe del catalogo §7.4 che oggi **nessun file implementa** — o che lo sono **in parte**, e
allora la riga dice **quale** parte. Stanno qui perché un registro che le omettesse lascerebbe
credere che siano coperte, e una riga che dicesse «scoperta» dove qualcosa c'è mentirebbe
nell'altro verso.

⚠️ **Riletta riga per riga il 2026-08-10, chiudendo il Traguardo 3, contro la condizione 11
della Definizione di «fatto» del piano** — *«dice cosa è coperto **e** cosa non lo è, con il
traguardo che lo chiude»*. **Dieci righe, e otto nominano il proprio.** Le **due** che non lo
fanno si dichiarano qui invece di lasciarle sembrare sviste: i **due residui di
`SystemReactor`** non nominano nessuno **perché nessuno li chiude con questi mezzi** — la
conformità non può coglierli e distinguerli sulla vera vorrebbe una garanzia che nessuna
piattaforma offre, quindi il «chi» è **vuoto per misura**, non per dimenticanza; il
**portachiavi** invece non ha un chiudente scritto da nessuna parte, ed è una **lacuna del
registro** e non della porta — nominarne uno adesso sarebbe inventarlo, quindi resta scritto
che manca.

⛔ **RICHIAMO DEL 2026-08-27, finding AUD-026 — IL CHIUDENTE ORA È SCRITTO, E NON QUI.** Le
righe **V34**, **Q24** e **Q17** della §8 della spec sono passate da ✅ e ⚠️ a ⏳ **rimandato**
con innesco, sul precedente di **V16** (§8.5.3.1), che parla della stessa sostanza: senza
nessuna credenziale nel perimetro un controllo proverebbe **l'assenza di una cosa che non
c'è** — gotcha #17. ⚠️ **Quale sia l'innesco non si ricopia qui**: la casa unica è la colonna
*Innesco* di §8.3 e §8.4, e un rimando non marcisce (`CLAUDE.md`, gotcha #68).
✅ **La riga della porta resta vera parola per parola** — nessuno script lo verifica — e il
conteggio *«otto su dieci»* qui sopra resta la misura del **2026-08-10**: si data, non si
riallinea. Ciò che è cambiato è che adesso una riga dice **chi**, e non che il controllo
esista.

| Riga del catalogo | Perché non c'è ancora |
|---|---|
| il **resto** del blocco **B** di §7.4.1 — i **gettoni** | ⚠️ **Non più interamente scoperto, dal 2026-08-09:** **una riga su cinque** è implementata — `promuovere testo a istruzione ← la porta journal` (V19), da `crates/kernel/tests/compile_fail/promote_without_journal.rs`, che nomina quella riga di catalogo nella propria intestazione. Degli **altri quattro**, **due** li emettono l'arbitro (§5.6) e il filtro dei vincoli (§6.3) — **Traguardi 5 e 6**; gli altri **due**, il `Worker` e la **ricevuta**, li emette già `crates/kernel/src/ports/process.rs`, e restano scoperti per la ragione della riga di §6.10.5 più sotto. ⚠️ **Corretto il 2026-08-10:** diceva che li emettevano **tutti e quattro** l'arbitro e il filtro dei vincoli, e per due era falso — `Process::start` restituisce il `Worker` e `instruct_one`/`instruct_stream` le ricevute, tutti e tre spediti da questo traguardo. ⛔ Un costruttore di `Grant` dietro una feature di test **è stato valutato e scartato**: creerebbe il secondo modo di ottenere una concessione che §5.6 esiste per togliere dal compilatore. ⚠️ **Ricontata il 2026-08-19, Traguardo 5 Task 4:** *«avviare un worker ← una concessione»* passa da scoperta a **PARZIALMENTE** coperta — metà *«senza → non compila»* da `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs`; la metà *«con → compila»* **non è scrivibile** finché nessuno emette concessioni, e l'innesco è il compito che porta `admit` (Task 5). ⛔ **Il numeratore delle COPERTE non si muove: resta una su cinque**, non due — una riga `parziale` non è chiusa, stesso trattamento di `Q8 · §5.2.1`. ⚠️ **E la frase «due li emettono l'arbitro e il filtro dei vincoli» è ora imprecisa per metà:** l'arbitro **esiste**, `Grant` vive in `crates/kernel/src/arbiter/mod.rs`, e ciò che manca non è più il modulo ma l'**emittente** — `admit`. Dettaglio nella sezione «Livello 1 · `Grant`, `Admission` e `Activity`». ✅ **Ricontata di nuovo il 2026-08-19, Traguardo 5 Task 5, e la riga è CHIUSA:** `admit` esiste, quindi la metà *«con → compila»* è diventata scrivibile ed è scritta — `releasing_gives_back_exactly_the_reservation` in `crates/kernel/tests/arbiter_admission.rs` ottiene un `Grant` da `admit` e lo consuma. ⛔ **Il numeratore si muove per la prima volta: DUE righe su cinque.** ⚠️ **E la frase «due li emettono l'arbitro e il filtro dei vincoli» è ora imprecisa per l'altra metà:** l'emittente esiste, e ciò che resta al Traguardo 6 è il **consumatore** — `Process::start`. ⚠️ **Le altre due righe** — il `Worker` e la **ricevuta** — restano scoperte, e il **perché** lo dice la riga di §6.10.5 più sotto, in una casa sola. ⛔ **RICHIAMO DEL 2026-08-21, finding P-2:** qui la ragione era **riscritta** — prima *«senza `Grant` non si ottiene un `Worker`»*, poi *«`start` non è ancora implementata da nessuno»* — ed è **tolta**, non riallineata una terza volta: era **falsa in entrambe le stesure**, e una ragione che vive in due case marcisce in quella che nessuno muove (`CLAUDE.md`, gotcha **#68**). Dettaglio nella sezione «Livello 1 · `Parameters::total_vram` e l'arbitro che ammette e rilascia». ✅ **Ricontata di nuovo il 2026-08-21, Traguardo 5 Task 11: il numeratore si muove ancora, QUATTRO righe su cinque.** Le due righe che restavano scoperte — il `Worker` (**riga 1** di §6.10.5, «parlare a un worker ← l'oggetto `Worker`») e la **ricevuta** (**riga 3**, «leggere ← una ricevuta») — sono ora tenute da `crates/kernel/tests/compile_fail/talking_without_the_handle.rs` (`E0599`) e da `crates/kernel/tests/compile_fail/reading_without_a_receipt.rs` (`E0061`), con le contro-sonde in `crates/kernel/tests/worker_tokens.rs`. ⛔ **Resta scoperta UNA riga**, quella che questa stessa cella attribuisce al filtro dei vincoli (§6.3, Traguardo 6): non è compito di questo task, e il numeratore non la conta. ✅ **Ricontata di nuovo il 2026-09-01, Traguardo 6 compito 6, E LA CELLA È CHIUSA: CINQUE righe su cinque.** L'ultima era `Q13`, *«eseguire una richiesta ← una prova di conformità»*, e il filtro dei vincoli **esiste** — `crates/kernel/src/gateway/mod.rs`, con `Conforming` a campi privati coniato dal solo `resolve`. ⛔ **Le metà sono DUE e i casi sono due**, come per il gettone `Grant`: `crates/kernel/tests/compile_fail/dispatching_an_unfiltered_candidate.rs` (`E0308`) e `crates/kernel/tests/compile_fail/conforming_has_no_constructor.rs` (senza sigla); la direzione *«filtrato → compila»* è `a_conforming_candidate_is_chosen_and_nothing_is_degraded` in `crates/kernel/tests/gateway_decisor.rs`. 📌 **Convenzione contata, la stessa già in uso:** una riga di catalogo vale UNO nel numeratore qualunque sia il numero di casi che la difendono. ✅ **Ricontato e non dedotto**, sull'enumerazione di questa cella: coperte `V19`, *«avviare un worker ← una concessione»*, il `Worker`, la **ricevuta** e `Q13` — **cinque**; scoperte **zero**. 5 + 0 = 5, e il conto torna. ⚠️ **Nel blocco B non resta nessuna riga scoperta né parziale**, per la prima volta da quando questa cella esiste. ⚖️ **E una voce si REGISTRA invece di prenderla:** un caso `compile_fail` per il **secondo dispaccio** dello stesso gettone (`E0382`, perché `Conforming` non deriva né `Copy` né `Clone`) sarebbe una **riga di catalogo nuova**, cioè §7.4, cioè **spec** — stesso trattamento e stessa ragione del quinto caso registrato per `Grant`. Dettaglio nella sezione «Il gettone di conformità del gateway» |
| il resto del blocco **C** di §7.4.1 | **nove righe su diciannove** sono implementate (sopra). ⚠️ **Ricontate una seconda volta il 2026-08-10**, eseguendo il Task 2 del Traguardo 3: diceva *«sette su diciotto»*, e **sbagliava di due nel numeratore, non di uno** — il Task 1 aveva consegnato `record_without_version.rs` senza scriverne la riga qui, e il Task 2 ne ha aggiunta un'altra insieme alla propria riga di catalogo. ⛔ **È la stessa specie di prima e va detta così:** il denominatore lo muove chi tocca il catalogo e se ne accorge; **il numeratore lo muove chi scrive un caso**, che il catalogo non lo apre nemmeno. Delle due, la seconda è quella che invecchia in silenzio. ⚠️ **Ricontate il 2026-08-10:** diceva *«sei su diciassette»*, sbagliato in **entrambi** i termini — e prima ancora *«tre su sedici»*. Il numero giusto **esisteva già** in testa alla sezione «Livello 1»: era stato rimisurato e scritto in **uno solo dei due posti dello stesso file**, e il denominatore era rimasto indietro perché la riga della **regola B** entrò nel catalogo lo stesso giorno. Le **altre dieci** non cambiano — i due termini erano bassi di uno insieme — e nominano tipi che nascono coi **Traguardi 3, 5 e 6**. ⚠️ **E la loro descrizione era stretta, corretta lo stesso giorno:** diceva *«nominano tipi dell'arbitro, del giornale e del canale worker»*, e **due delle dieci non vi rientrano** — `V5`, un effetto senza classe dichiarata, e `V10`, un sensore che modifica l'artefatto. Chi le ricontasse **dalla descrizione** ne troverebbe otto: è lo stesso difetto della riga qui accanto un livello più sotto, e si ricontano **sul catalogo**, che è l'unico posto che le enumera. ⚠️ **E `V5` merita una parola, perché il Traguardo 3 l'ha resa ingannevole:** il tipo `EffectClass` **esiste** da `crates/kernel/src/record.rs` ed è un campo obbligatorio del record, ma **nessun caso lo esercita** — un tipo che esiste non è un controllo che scatta, e la riga resta fra le scoperte. ⚠️ **Ricontate il 2026-08-19, Traguardo 5 Task 1:** `Q2 · §5.1` passa da scoperta a coperta, quindi **dieci righe su diciannove**, non nove — il denominatore non si muove, `Q2` era già fra le diciannove. ⚠️ **Ricontate di nuovo lo stesso giorno, Traguardo 5 Task 3:** `Q8 · §5.2.1` passa da scoperta a **PARZIALMENTE** coperta — prima metà `crates/kernel/tests/compile_fail/admission_reads_cold_start.rs` (`E0609`), innesco scritto per la seconda («si chiude al compito che porta `admit`», Task 5. Dettaglio nella sezione «Livello 1 · `ResourceProfile` e `WorkDescriptor`»). ⛔ **Non entra nel numeratore delle coperte:** resta **dieci righe su diciannove**, non undici — una riga `parziale` non è una riga chiusa. ⚠️ **Ricontate una terza volta il 2026-08-19, Traguardo 5 Task 4:** `V4` e `I2 · §5.3` passano da scoperte a **coperte**, entrambe con le due direzioni (la seconda per mutazione, e il limite di ciò che una mutazione compra è dichiarato nella sezione del Task 4) — quindi **dodici righe su diciannove**. Il denominatore non si muove: erano già fra le diciannove. ⛔ **CORRETTO in un compito di rifinitura, stesso giorno: questa cella diceva «undici», «undici» e «tredici», e sommava `Q2 · §5.1` come DUE righe di catalogo — nel catalogo (§7.4.1 blocco C) è UNA sola, «MiB assegnati a millisecondi», la stessa voce già aperta come R3 nella sezione «Livello 1» qui sopra.** Ricontato **direttamente su §7.4.1 blocco C** (non sulla tabella «Livello 1», che per R3 continua a mostrare `Q2` come due righe di mappatura regola-a-file — una scelta dichiarata, non toccata qui): coperte `Q9·I6·V20` (regola A e B), `Q2·§5.1`, `V29·§2.1` (entrambe), `V29·§2.2`, `I2·§5.3`, `V4`, `V29·§2.8·ADR-0034` (entrambe), `Q14·§4.9`, `Q9·I6·V20·§4.9` — **dodici** righe; scoperte `V2`, `V3`, `V5`, `V10` e le due righe di §6.10 — **sei**; parziale `Q8·§5.2.1` — **una**. 12 + 6 + 1 = 19, e il conto torna. 📌 **Convenzione contata:** una riga di catalogo vale UNO nel numeratore qualunque sia il numero di casi `compile_fail` che la difendono — la stessa già in uso per la riga `Q9 · I6 · V20 · §4.9` della tabella «Livello 1»: due casi, una riga. ⚠️ **Restano scoperte `V2`, `V3`, `V5`, `V10`**, più le due righe di §6.10 e la riga di `Q8` che è parziale: si ricontano **sul catalogo**, mai da questa frase. ✅ **Ricontate una QUARTA volta il 2026-08-19, Traguardo 5 Task 5: `Q8 · §5.2.1` e `V2` passano a COPERTE, quindi QUATTORDICI righe su diciannove.** `Q8` era l'unica `parziale` e non lo è più — il caso ora nomina `admit`; `V2` aveva un innesco scritto in questo stesso file dal Task 3 e ha ora `crates/kernel/tests/compile_fail/admission_without_profile.rs`. Il denominatore non si muove: erano già fra le diciannove. ⛔ **Restano scoperte `V3`, `V5`, `V10` e le due righe di §6.10 — CINQUE**, e 14 + 5 = 19. ⚠️ **Nel blocco C non c'è più nessuna riga `parziale`**: chi ricontasse cercando la terza colonna non ne troverebbe. ✅ **Ricontate una QUINTA volta il 2026-08-20, Traguardo 5 Task 8: `V3` passa da SCOPERTA a PARZIALE, e il numeratore NON si muove — restano QUATTORDICI righe su diciannove.** `VramPolicy` è un enum consegnato per posizione a `Arbiter::new`, quindi «due policy attive» è un errore di **arità** — `crates/kernel/tests/compile_fail/two_policies_at_once.rs`, e la contro-sonda è `crates/kernel/tests/arbiter_policy.rs`. ⛔ **MA la contro-sonda che la cella di catalogo pretende è DOPPIA:** «con una policy sola compila, **e la transizione resta un passo giornalato (§5.4)**», e la seconda metà è del **Task 9**. ⛔ **CORRETTO NELL'ONDATA DI CORREZIONI DELLO STESSO GIORNO: questa cella diceva «COPERTA» e «QUINDICI».** La regola opposta è quella che questo stesso file aveva già applicato **due volte** — a `Q8 · §5.2.1` (*«non entra nel numeratore delle coperte: una riga parziale non è una riga chiusa»*) e alla riga del blocco B — e la coerenza col precedente del file vale più di un numeratore che avanza. ✅ **Ricontato e non dedotto**, sull'enumerazione di questa cella: coperte le **dodici** di sopra più `Q8 · §5.2.1` e `V2` — **quattordici**; **parziale** `V3` — **una**; scoperte `V5`, `V10` e le due righe di §6.10 — **quattro**. 14 + 1 + 4 = 19, e il conto torna. ⚠️ **Nel blocco C torna quindi a esserci una riga `parziale`**, dopo che dal Task 5 non ce n'erano. ⏳ **INNESCO, scritto perché chi chiude sappia di doverlo muovere: la seconda metà di `V3` è del TASK 9**, e è quel compito a portare il numeratore a **quindici**. Voce `E103`. ✅ **L'INNESCO È SCATTATO ED È STATO RACCOLTO — ricontate una SESTA volta il 2026-08-20, Traguardo 5 Task 9: `V3` passa da PARZIALE a COPERTA, quindi QUINDICI righe su diciannove.** La seconda metà della contro-sonda esiste: `Arbiter::set_policy` scrive l'intento, scambia l'oggetto e scrive l'esito, e **cinque** sonde di `crates/kernel/tests/arbiter_policy.rs` la esercitano da fuori la crate. ✅ **Ricontato e non dedotto**, sull'enumerazione di questa cella: coperte le **quattordici** della quarta ricontata più `V3` — **quindici**; **nessuna** parziale; scoperte `V5`, `V10` e le due righe di §6.10 — **quattro**. 15 + 4 = 19, e il conto torna. ⚠️ **Nel blocco C torna a non esserci nessuna riga `parziale`**, come dal Task 5 al Task 8. ✅ **Ricontate una SETTIMA volta il 2026-08-21, Traguardo 5 Task 11: le due righe di §6.10 passano da scoperte a COPERTE, quindi DICIASSETTE righe su diciannove.** `I2 · §6.10` (riga 2 di §6.10.5, istruire dopo `kill`) è tenuta da `crates/kernel/tests/compile_fail/instructing_after_the_kill.rs` (`E0382`); `I5 · §6.10` (riga 4, leggere due volte la stessa ricevuta) da `crates/kernel/tests/compile_fail/reading_twice_from_one_receipt.rs` (`E0382`). ✅ **Ricontato e non dedotto**, sull'enumerazione di questa cella: coperte le **quindici** della sesta ricontata più `I2 · §6.10` e `I5 · §6.10` — **diciassette**; **nessuna** parziale; scoperte `V5` e `V10` — **due**. 17 + 2 = 19, e il conto torna. ✅ **Ricontate una OTTAVA volta il 2026-09-01, Traguardo 6 compito 5: `V10` passa da scoperta a COPERTA, quindi DICIOTTO righe su diciannove.** Il caso che deve scattare è `crates/kernel/tests/compile_fail/sensor_modifies_the_artefact.rs` (`E0594`); la contro-sonda *«osservarlo e restituire un verdetto compila»* è `a_passing_sensor_writes_a_verdict_and_opens_nothing` in `crates/kernel/tests/sensor_ring.rs`, che il tratto lo **implementa e lo esegue** — e vive lì e non accanto al caso negativo, che sarebbe il gotcha **#49**. ✅ **Ricontato e non dedotto**, sull'enumerazione di questa cella: coperte le **diciassette** della settima ricontata più `V10` — **diciotto**; **nessuna** parziale; scoperta la sola `V5` — **una**. 18 + 1 = 19, e il conto torna. ⚠️ **E `V5` resta scoperta anche adesso che il record ha un campo in più**, il che vale una riga perché il compito 5 l'ha sfiorata: `EffectClass` è un campo obbligatorio da tre traguardi e nessun caso lo esercita ancora — *un tipo che esiste non è un controllo che scatta*, come questa stessa cella scrive dal 2026-08-10 |
| i test di contratto per le **altre quattro** famiglie di porte | ✅ **`reactor` è coperta** dal Task 7 del Traguardo 2 — sonde R1…R6. ⚠️ **Ricontate il 2026-08-10:** questa riga diceva *«le altre **cinque**»* e contava `journal` fra le scoperte, e dal Task 4/5 di questo traguardo non lo è più — `crates/kernel/tests/journal_contract.rs`, sonde **J1…J8**. ⛔ **Ma è coperta a metà, e la metà va detta:** una suite di conformità vale la prova che **due** implementazioni rispondono lo stesso. ⚠️ **Ricontata il 2026-08-10 col Task 8:** la seconda implementazione **esiste** — `platform::journal::FileJournal` su `redb` — e la suite le gira contro **verde**, misurato con un file usa-e-getta; ma il file che la esegue **dentro** il repository era il **Task 9**, quindi finché non c'era, ciò che il repository comprava era la via **A6** di `boundary.rs` e otto promesse **scritte in una copia sola**, non l'accordo fra due. ⛔ *«Misurato una volta»* non è *«tenuto a ogni commit»*, ed è esattamente la differenza che questo registro esiste per non lasciar sfumare. ✅ **Ricontata una terza volta il 2026-08-10, col Task 9, e la mezza copertura è CHIUSA:** `crates/platform/tests/journal_contract_real.rs` esiste, e l'accordo fra le due implementazioni è ora **tenuto a ogni commit** — sonda **J12**, con tre contro-sonde che la provano non vacua su **tre promesse diverse** e una mutazione di controllo che non muove nulla. ✅ **Ricontata una QUARTA volta il 2026-08-10, col Task 11, e anche la promessa 7 è chiusa:** questa riga diceva *«ciò che resta scoperto di `journal` non è più il numero delle implementazioni ma la promessa 7, che entrambe soddisfano per la ragione sbagliata finché `prune` non è implementata»* — `prune` è implementata su entrambe, la promessa **7b** è la metà che discrimina, e le sonde sono **J1…J13**. ✅ **Ricontata una QUINTA volta il 2026-08-17, chiudendo T-1 e T-2 dell'audit, e le sonde sono J1…J16:** l'accordo fra le due implementazioni era tenuto **a ogni commit** ma su **tre** promesse — la 1, la 5 e la 8 — soltanto nello stato in cui ogni guardia plausibile passa, cioè un archivio **vuoto** o a **un passo solo**. Chiuse con un **passante**, senza aggiungere nessuna promessa: ciò che il registro comprava era *«le due rispondono lo stesso»*, e ciò che ora compra è *«le due rispondono lo stesso anche dove una guardia sbagliata divergerebbe»*. ⚠️ **Ciò che resta scoperto è ora dichiarato in due voci aperte** — la distinzione di ADR-0018 fra potato e mai registrato, e la terza risposta di `prune` — non più in una promessa che passa senza guardare. ✅ **Ricontate il 2026-08-27, chiudendo la radice R2 del secondo audit: sono TRE.** La terza è la **divergenza fra le due nozioni di «in dubbio»**, che cade dal lato che autorizza la potatura — finding **AUD-006**, misurato su entrambe le implementazioni. Restano scoperte `filesystem`, `network`, `process` e `ipc`: le loro implementazioni nascono coi **Traguardi 5 e 6**, e la suite di ciascuna nasce con esse |
| **due residui dichiarati dentro `SystemReactor`**, e stanno qui perché un registro che li tacesse mentirebbe | ⛔ sostituire `Some(self.now())` con `Some(deadline)` in `wait_until` **non fa scattare nulla**: la conformità non può coglierlo perché **sulla finta le due espressioni coincidono**, e distinguerle sulla vera richiederebbe l'overshoot dello sleep del sistema operativo, che nessuna piattaforma garantisce — un controllo verde per fortuna e rosso per sfortuna, cioè peggio di uno assente (gotcha #24). ⚠️ E `R5` prova che `wall_time()` non è **ferma**, non che sia **esatta**: l'esattezza vorrebbe una seconda sorgente di tempo |
| ~~i **byte congelati** del record durevole~~ ✅ **CHIUSA il 2026-08-10, col Task 10** | ⚠️ **Resta scritta perché la storia della riga è il dato.** Diceva prima *«non esiste ancora nessun record»* — falso dal Task 1 — e poi *«a non esistere è `crates/kernel/tests/frozen_bytes.rs`, e finché non esiste le tre regole sugli indici di §4.9.2 sono una convenzione e non un controllo»*, che era vero e ora non lo è: il file esiste, congela **tre** record, e le **otto** varianti sono state rinumerate una per una con **otto rossi su otto** — sonde **F1…F6** nel livello 2. ⛔ **Ciò che resta scoperto è UNA sola cosa, e va detta:** l'oracolo è un file che qualcuno può riscrivere, e la difesa è che la riscrittura **si legge nel diff** — non che sia impossibile. È la stessa forza e la stessa debolezza del gotcha **#25**, dichiarata da ADR-0036 fin dall'inizio. Vincolo 14 della §11 del [compendio](COMPENDIO.md) |
| la **campagna DST** — ✅ **COPERTA dal 2026-09-02**, parziale dal 2026-08-25 al compito 9 del Traguardo 6 — e l'elenco versionato dei **semi** di V31, che resta **SCOPERTO** | ⚠️ **Il soggetto era sbagliato, corretto il 2026-08-09:** diceva *«non esiste ancora il simulatore»*, e la crate `simulator` **esiste** e spedisce `SeededRng` e `VirtualReactor` — `crates/kernel/tests/executor_determinism.rs` gira già **C1 e C2** su di essi. ⛔ **RISCRITTA IL 2026-08-25, PRIMA ONDATA DI CORREZIONI DEL TASK 12: diceva *«A non esistere è la campagna: molti semi, guasti iniettati, e l'elenco versionato dei semi. Traguardo 4»*, e la campagna ESISTE.** Si **riscrive** col richiamo datato invece di accorciarla, perché a essere falso è il **fatto**. ⚠️ **La riga era già stantia PRIMA di questo compito** — il Traguardo 4 l'aveva smentita con `crates/simulator/tests/dst_campaign.rs` — e il Passo 6 del brief del Task 12 chiedeva **esattamente** di chiuderla: la divergenza fra ciò che quel passo chiede (*«coperta»*) e ciò che è scrivibile oggi (*«parziale»*) è la voce `E152`. ✅ **LA SONDA CHE LA CELLA DI CATALOGO PRETENDE È ESEGUITA E DATATA, il 2026-08-25** — *«si rompe l'ammissione: la campagna fallisce e nomina il seme (§5.7.1)»*: col confronto di `admit` fra l'allocato e il tetto reso `false`, `cargo test --locked -p simulator --test arbiter_campaign` esce rosso su tutte e cinque le sonde con `seed 0: allocated Mib(10240) exceeds the total Mib(8192)`, e su quella a seme fisso con `seed 20260818: allocated Mib(9216) exceeds the total Mib(8192)` — il **seme** e i **due valori**. Ripristinato da copia byte-esatta, `cmp` identico, `git status --porcelain` vuoto, e la campagna torna verde a **cinque sonde su cinque**. La contro-sonda della stessa cella — *«senza guasto iniettato, nessun passo in dubbio»*, `C7a` — è del Traguardo 4 e vive in `dst_campaign.rs`. ⛔ **PARZIALE E NON COPERTA, e il perché sta nella colonna «Difende» della riga di catalogo:** difende `Q2 · Q3 · Q4 · Q5 · Q18 · Q22 · I1 · I2 · I5 · V1 · V6`, e la §5.7 elenca **cinque** proprietà. La campagna dell'arbitro ne tiene **tre** — la **1** (la somma delle concessioni non supera il budget), la **4** (una transizione di policy interrotta lascia un passo riconciliabile) e la **5** (una concessione scaduta non resta allocata). Le altre due si iniettano sulle porte `process` (nessun processo è `Attiva` senza concessione valida) e `ipc` (la GUI muore tenendo una concessione discrezionale), che nascono al **Traguardo 6**. ⏳ **Innesco, scritto perché chi chiude sappia di doverlo muovere: le proprietà 2 e 3 di §5.7 sono del Traguardo 6**, ed è quel traguardo a portare questa riga a coperta. ✅ **Ricontato SUL CATALOGO e non per sottrazione**, delimitando per intestazione come il gotcha **#26** prescrive — `awk '/^#### 7\.4\.2/{f=1} /^#### 7\.4\.3/{f=0} f' <spec> \| grep -c '^\| '` dà **quattordici**, cioè **tredici** righe più l'intestazione — e riga per riga: **coperte** l'allow-list sul grafo spedito, l'allow-list sul grafo di build, il cancello senza OS, `check-docs.sh`, i byte congelati, gli attributi delle crate vincolate e il build script — **sette**; **parziali** i test di contratto (`reactor` e `journal` sì, `filesystem`, `network`, `process` e `ipc` no) e la campagna DST — **due**; **scoperte** il portachiavi, il punto d'uscita verso la rete, l'elenco versionato dei semi di V31 e i byte consumati dal frame — **quattro**. 7 + 2 + 4 = 13, e il conto torna. ⛔ **Il numeratore delle COPERTE non si muove: restano SETTE su tredici** — una riga parziale non è una riga chiusa, che è il trattamento già dato a `Q8 · §5.2.1`, a `V3` e alla riga del blocco **B**. ⚠️ **E l'elenco versionato dei semi di V31 resta SCOPERTO**, ed è una riga di catalogo distinta da questa: la campagna non ha trovato difetti di prodotto, quindi non c'è nessun **seme colpevole** da versionare. ✅ **RICONTATO UNA SECONDA VOLTA il 2026-08-31, Traguardo 6 Compito 3: la riga dei byte consumati dal frame passa da SCOPERTA a COPERTA, quindi OTTO righe su tredici.** Rifatto **col comando** e non per sottrazione — `awk '/^#### 7\.4\.2/{f=1} /^#### 7\.4\.3/{f=0} f' <spec> \| grep -c '^\| '` dà **quattordici**, cioè tredici righe più l'intestazione, **invariato**: il compito non tocca la spec (vincolo globale 7). Riga per riga: **coperte** le sette di sopra più i **byte consumati dal frame** — **otto**; **parziali** i test di contratto e la campagna DST — **due**, nessuna delle due mossa da questo compito; **scoperte** il portachiavi, il punto d'uscita verso la rete e l'elenco versionato dei semi di V31 — **tre**. 8 + 2 + 3 = 13, e il conto torna. ✅ **RICONTATO UNA TERZA VOLTA il 2026-09-02, Traguardo 6 Compito 9: la riga della campagna DST passa da PARZIALE a COPERTA, quindi NOVE righe su tredici.** Rifatto **col comando** e non per sottrazione — `awk '/^#### 7\.4\.2/{f=1} /^#### 7\.4\.3/{f=0} f' <spec> \| grep -c '^\| '` dà **quattordici**, cioè tredici righe più l'intestazione, **invariato**: il compito non tocca la spec (vincolo globale 7). Riga per riga: **coperte** le otto di sopra più la **campagna DST** — **nove**; **parziali** i soli test di contratto (`reactor` e `journal` sì, `filesystem`, `network`, `process` e `ipc` no) — **una**; **scoperte** il portachiavi, il punto d'uscita verso la rete e l'elenco versionato dei semi di V31 — **tre**. 9 + 1 + 3 = 13, e il conto torna. ⛔ **E LE CINQUE PROPRIETÀ DI §5.7 SONO NOMINATE UNA PER UNA, col banco che tiene ciascuna, perché «coperta» senza l'elenco è la stessa affermazione non verificabile che questa riga rimprovera altrove:** ① *la somma delle concessioni non supera il budget* — `property_1_the_sum_never_exceeds_the_total_on_any_seed`, `crates/simulator/tests/arbiter_campaign.rs`; ② *nessun processo è attivo senza concessione valida* — `property_2_a_killed_worker_leaves_no_reservation_behind`, `crates/simulator/tests/worker_kill_campaign.rs`; ③ *la gui muore tenendo una concessione discrezionale* — `property_3_a_gui_that_dies_holding_a_grant_gives_it_back`, `crates/simulator/tests/gui_death_campaign.rs`; ④ *una transizione di policy interrotta lascia un passo riconciliabile* — `property_4_a_severed_transition_leaves_a_reconcilable_step`, `arbiter_campaign.rs`; ⑤ *una concessione scaduta non resta allocata* — `property_5_expiry_frees_the_budget_under_the_scenario`, `arbiter_campaign.rs`. ⚠️ **RICONTATE SUL FILE e non per «tre più due»**, che è la regola 1 della §7.3 del disegno: `grep -rn "^fn property_" crates/simulator/tests/` rende **cinque** righe, una per numero da 1 a 5, e l'uscita sta nella sezione del compito 9 qui sotto. ⚠️ **E l'elenco versionato dei semi di V31 resta SCOPERTO anche adesso:** nessuna delle due campagne nuove ha trovato un difetto di prodotto, quindi non c'è nessun **seme colpevole** da versionare |
| ~~i **byte consumati** pari alla lunghezza dichiarata dal frame~~ ✅ **CHIUSA il 2026-08-31, col Compito 3 del Traguardo 6** | ⚠️ **Resta scritta perché la storia della riga è il dato.** Diceva *«non esiste ancora il canale verso i worker. Traguardo 6»*, ed era vero. Ora la busta esiste — `crates/kernel/src/framing.rs`, **quattro byte big-endian**, decisi lì e in nessun altro posto — e lo schema pure, `crates/kernel/src/wire/worker.rs`, il cui `decode` verifica i byte consumati. ⛔ **Chiusa nelle DUE direzioni, e sono due guasti distinti con prenditori distinti:** il **troncamento** e la **coda fuori dalla busta** li prende la lunghezza dichiarata, la **coda DENTRO la busta** la prende `position() != body.len()` — misurato, togliendo l'uno l'altro sopravvive (le sonde **W** del livello 2). ⛔ **Nessun byte congelato è nato**, e §6.10.3 lo vieta: questo canale prende la meccanica di `record.rs` e non la sua disciplina. Vincolo **15** della §11 del [compendio](COMPENDIO.md), che esce dalla tabella *«cosa resta davanti»* nello stesso passaggio |
| ~~le **righe 1–4 di §6.10.5** — i casi negativi della porta `process`~~ ✅ **CHIUSA il 2026-08-21, col Task 11** | ⛔ **RISCRITTA IL 2026-08-21 — FINDING P-2 DELL'AUDIT: LA RAGIONE DELLO SCAGLIONAMENTO ERA FALSA.** Si **riscrive** col richiamo datato invece di accorciarla, perché a essere sbagliato è il **fatto** e non una qualificazione intorno a esso: è il limite dichiarato del gotcha **#76**. Diceva: *«tutte e quattro pretendono di ottenere un `Worker`; un `Worker` lo restituisce solo `start(grant, ..)`; e nessuno emette concessioni prima del Traguardo 5»*. ✅ **Misurato su una sonda d'integrazione usa-e-getta, compilata e cancellata nella stessa corsa** — non dedotto, e non ripreso dal rapporto d'audit (gotcha **#65**): un `Worker` si ottiene **implementando il tratto da fuori dalla crate**, `impl Worker for W` con **zero** concessioni, e `crates/kernel/tests/ports_are_implementable.rs` lo fa dal **Traguardo 2** con `ScriptedWorker`. ⛔ **Ciò che una concessione sblocca è UNA cosa sola: chiamare `Process::start`.** Rimisurato prima di questo compito: `grep -rn "\.start(" crates/ --include=*.rs` dà **zero** chiamanti in tutto il workspace. ✅ **E anche quella metà è spesa dal Traguardo 5 Task 5**, misurata nella stessa corsa: `Admission::Granted(Grant)` è **pubblica**, quindi una concessione vera di `admit` raggiunge un `Process::start` scritto da fuori — la sonda ha compilato e passato. ⚠️ **Ciò che NON è stato rimisurato, e va detto invece di dedotto:** se i quattro casi fossero stati scrivibili già al Traguardo 2. Scriverli **è** il Task 11, e una misura anticipata varrebbe come previsione e mai come collaudo (gotcha **#53**). ⛔ **L'unica ragione che resta è quella vera fin dall'inizio:** una regola provata in **una direzione sola** non è ammissibile (§7.1.1 regola 3), e la direzione *«deve scattare»* — i quattro casi `compile_fail` — non è scritta. La chiude il **Task 11**. ⚠️ Un costruttore di `Grant` dietro una feature di test resta **scartato** per la ragione già scritta nella riga del blocco **B** qui sopra: il verdetto non cambia, cambia il fatto che non serviva. Quel che le sostituisce intanto è `ports_are_implementable.rs`, che le firme le esercita **in entrambe le direzioni** con una finta costruita direttamente dal test. ✅ **RICONTATA DI NUOVO IL 2026-08-21, TRAGUARDO 5 TASK 11, E LA RIGA È CHIUSA.** La direzione che mancava — «deve scattare» — è scritta: quattro casi `compile_fail` (`talking_without_the_handle.rs`, `instructing_after_the_kill.rs`, `reading_without_a_receipt.rs`, `reading_twice_from_one_receipt.rs`) e le loro contro-sonde in `crates/kernel/tests/worker_tokens.rs`, che ottengono un `Grant` vero da `Arbiter::admit` — MAI un costruttore di test, scartato per la ragione già scritta qui sopra. Oracoli letti uno per uno, non rigenerati alla cieca (vincolo globale 5): `E0599` (riga 1, senza la maniglia), `E0382` (riga 2, dopo l'uccisione), `E0061` (riga 3, senza ricevuta), `E0382` (riga 4, due letture). ⚠️ **`E0061` prova solo l'ARITÀ, non l'autenticità della ricevuta:** `SingleReceipt::new` è `pub` e raggiungibile da fuori la crate — lo dimostra `worker_tokens.rs` stesso, che la chiama da un banco d'integrazione — quindi `worker.read_one(SingleReceipt::new(7))` compila; il limite è già dichiarato accanto a `SingleReceipt::new` in `crates/kernel/src/ports/process.rs`. È il contrario del gettone `Grant`, che ha `grant_has_no_constructor.rs` proprio perché un costruttore pubblico non c'è. Le quattro funzioni passano da sole (`cargo test --locked -p kernel --test worker_tokens`) e **due** sono state provate portanti per mutazione, ciascuna revocata da una copia presa prima: **M1** (`read_one` risponde una costante invece dei byte della ricevuta) uccide da sola `reading_once_with_the_receipt_compiles`; **M2** (`instruct_one` non incrementa `next`) uccide da sola `with_the_handle_the_worker_can_be_instructed`. ⚠️ **Un terzo candidato esplorato e NON contato come copertura:** `kill` che rifiuta sempre uccide da sola `instructing_before_the_kill_compiles`, ma quel test non ha asserzioni oltre agli `expect` e la sua vera pretesa — che istruire PRIMA della `kill` COMPILI — non è una proprietà che una mutazione a runtime può provare; la tiene il fatto che il file compila, e la metà opposta la tiene `instructing_after_the_kill.rs`. ⚠️ **E il quarto, `one_grant_starts_one_worker`, non è stato esplorato per mutazione perché non ha un'asserzione a runtime sul proprio soggetto:** il panico raggiungibile condiviso con gli altri tre è quello di `a_real_grant()` — se muore, muoiono tutti e quattro insieme; quello dentro `one_grant_starts_one_worker` non scatta, perché `FakeProcess::start` risponde `Ok` incondizionatamente — e il commento nel sorgente lo dichiara già: non tiene nessuna forma che `a_started_worker` non compili già. ⚖️ **Un quinto caso `compile_fail`** — un secondo `start` con lo stesso `Grant`, `E0382` perché `Grant` non è `Copy`, e non lo può diventare perché non deriva nemmeno `Clone` — resta **registrato e non preso**: se pretenda una propria riga di catalogo lo decide la spec (vincolo globale 7), non questo compito |
| solo `secrets` raggiunge il **portachiavi** | nessuno script lo verifica oggi: `gate-deps.sh` guarda i grafi di `kernel` e `simulator`, non quelli di `platform` e `secrets` |
| un solo **punto di uscita verso la rete** | la lista delle crate autorizzate è **vuota**, e una lista vuota passa sempre. Il catalogo lo dichiara già: è l'unica voce provata in una direzione sola, e si completa nel sotto-progetto che accende la rete |
