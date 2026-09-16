# Rapporto R11 — Compito 3: lo schema che cresce (varianti nuove, gemelli, fixture, timbro) — 2026-09-16

## Esito in tre righe

1. Il compito 3 **non compila com'è scritto**: due rossi del compilatore, misurati in una copia del workspace
   (`cargo build --locked -p kernel` → `E0433 cannot find type Millis`; `cargo test --locked -p kernel --test ipc_wire`
   → `E0423 cannot initialize a tuple struct which contains private fields` su `BuildStamp(0)`, scritto **da fuori dalla crate**).
2. Riparati quei due punti **nella copia** (e dichiarato come), tutto il resto del compito **funziona**: 12 passati e 1 ignorato,
   14 `.bin` + 14 `.json` + `ipc_v1.map` col timbro, G6 e G8 esattamente come il piano li scrive. Ma **una sonda è vacua**
   (`tr` con un glob in redirezione: rende 0 senza leggere niente) e **una manca** (`ipc_v1.map` — l'artefatto che **D52**
   consegna al 13 — non è esercitato da nessun controllo: con G8 attiva la suite resta VERDE mentre la mappa dice
   `stamp 0x47b554435836e861` e `build_stamp()` calcola `0xee1f22ae2aaa9eeb`).
3. Tre commenti che il compito **scrive nel sorgente** dicono il falso, uno dei quali è **l'ultima casa sopravvissuta di P-35**
   (*«Eleven variants arrived»*, in inglese, che `grep -c undici` non poteva contare: le varianti nuove sono **dodici**).

## Rilievi

**Tredici**, di cui **otto** con «Blocca? sì» — contati dalla tabella qui sotto con
`probe-R11/count.py` (13 righe, 8 `sì`, 5 `no`), non dall'elenco.

| # | Compito · dove (una FRASE da cercare col grep) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R11-1 | Compito 3, Passo 1, riga `grep -c '#\[test\]' crates/kernel/tests/ipc_wire.rs` | «**sette** `#[test]` in `ipc_wire.rs`» | `grep -c '#\[test\]' crates/kernel/tests/ipc_wire.rs` → **9**; `grep -n '#\[test\]' …` → sette attributi (righe 16, 23, 37, 54, 63, 76, 100) **più due occorrenze dentro un commento** (righe 44 e 102, *«ITS OWN `#[test]` AND NOT A SECOND ASSERTION»* e *«IT IS A SEPARATE `#[test]` ON PURPOSE»*). Identico su `42b50d8` (→ **9**). I test sono davvero sette, il **comando** non lo dice | CONFERMATO | fatto | sì | Nel Passo 1 sostituire il comando con un'ancora di riga: `grep -cE '^#\[test\]' crates/kernel/tests/ipc_wire.rs` (misurato: **7**). L'Atteso «sette» resta, e «dodici passati» del criterio di chiusura **regge** (misurato) |
| R11-2 | Compito 3, Passo 3, in `stamp_set`: `preemption: Preemption::After(Millis::new(500))` | Il Passo 2 dichiara **un solo** import nuovo: *«`String` e `Vec` vogliono l'import: in testa al file `use alloc::string::String;`»*. Il blocco *Interfaces* elenca `kernel::time::Millis` sotto *Consumes* | Copiato il workspace, applicati Passi 2+3 **alla lettera**, `cargo build --locked -p kernel` → `error[E0433]: cannot find type 'Millis' in this scope --> crates\kernel\src\wire\ipc.rs:472:43 … help: consider importing this struct: use crate::time::Millis;`. Il file di oggi importa solo `crate::arbiter::{ComputeClass, Mib, Preemption}` e `crate::framing::{self, WireError}` (`sed -n '34,38p' crates/kernel/src/wire/ipc.rs`) | CONFERMATO | fatto | sì | Nella riga «⚠️ `String` e `Vec` vogliono l'import» del **Passo 2** aggiungere `use crate::time::Millis;` accanto a `use alloc::string::String;`, dicendo che è `Preemption::After(Millis::new(500))` dell'insieme canonico a chiederlo |
| R11-3 | Compito 3, Passo 5, sonda `the_stamp_changes_when_the_schema_changes`, riga `altered[0] = IpcMessage::Hello(BuildStamp(0));` | Costruisce un `BuildStamp` dentro `crates/kernel/tests/ipc_wire.rs`, che è **un banco di integrazione, fuori dalla crate** | `cargo test --locked -p kernel --test ipc_wire` → `error[E0423]: cannot initialize a tuple struct which contains private fields --> crates\kernel\tests\ipc_wire.rs:213:36 … note: constructor is not visible here due to private fields --> crates\kernel\src\wire\ipc.rs:198:23 -> pub struct BuildStamp(u64); -> private field`. ⛔ **E non è una svista del Passo 5 ma la sua contraddizione col Passo 2**, che scrive nel doc di `BuildStamp`: *«THE CONSTRUCTOR IS `crate::wire::ipc::build_stamp` AND NOT A `new` HERE … a stamp anyone can mint from any number is a stamp that proves nothing»*. È la **terza domanda** del pre-controllo: si vede solo scrivendo da fuori | CONFERMATO | fatto | sì | Nel Passo 5, `altered[0] = IpcMessage::Hello(build_stamp());` — un valore diverso da quello dell'insieme canonico, prodotto dall'unico costruttore che esiste (provato: compila e la sonda resta verde). In alternativa `altered.remove(0);`. ⛔ **Non** aggiungere un `BuildStamp::new`: disfarebbe la decisione del Passo 2 |
| R11-4 | Compito 3, Passo 5, riga `⚠️ Gli import in testa al file crescono: use kernel::wire::ipc::{build_stamp, stamp_set, Access, BuildStamp, …}` | La riga `use` è dettata per esteso, e il Passo 4 chiede *«verde a zero avvisi»* | `cargo test --locked -p kernel --test ipc_wire` sulla copia → `warning: unused imports: 'DegradationReport', 'PolicyReport', and 'StepSummary' --> crates\kernel\tests\ipc_wire.rs:6:75`. I tre nomi **non compaiono mai** nel codice dettato dei Passi 5, 6 e 7: `variant_json` distrugge quei valori col pattern (`IpcMessage::Degradation(report)`, `IpcMessage::Policy(report)`, `IpcMessage::Steps(steps)`) e non nomina mai il tipo. ⚠️ Il cancello **non** è `-D warnings` (`grep -n 'RUSTFLAGS\|-D warnings' scripts/gate.sh` → niente), quindi non va rosso: va rosso l'Atteso del Passo 4 | CONFERMATO | fatto | no | Togliere `BuildStamp` (vedi R11-3), `DegradationReport`, `PolicyReport` e `StepSummary` dalla riga `use` del Passo 5. Provato: con `{build_stamp, stamp_set, Access, Call, GrantRequest, IpcMessage, LayoutState, PolicyName, Protection, Provenance, Triple, Verdict}` il banco compila a **zero avvisi** |
| R11-5 | Compito 3, Passo 6, riga `tr -cd '\r' < gui/schema/fixtures/*.json gui/schema/fixtures/ipc_v1.map \| wc -c` | Atteso: «**zero** CR (nascono LF, vincolo 4)» | Lanciato verbatim nella copia, con le fixture vere sul disco: `/usr/bin/bash: line 1: gui/schema/fixtures/*.json: ambiguous redirect` → `wc -c` legge un ingresso vuoto e stampa **0**. ⛔ **Controprova nella direzione che deve rendere ≠ 0**: `tr -cd '\r' < crlf.txt \| wc -c` su un file CRLF → **2**. Quindi la riga rende **0 qualunque cosa contengano le fixture**: un glob con 14 riscontri in una redirezione `<` non è un ingresso, e il secondo percorso finirebbe comunque come operando di `tr`. La misura vera: `cat gui/schema/fixtures/*.json gui/schema/fixtures/ipc_v1.map \| tr -cd '\r' \| wc -c` → **0** | CONFERMATO | fatto | sì | Nel Passo 6 sostituire la riga con `cat gui/schema/fixtures/*.json gui/schema/fixtures/ipc_v1.map \| tr -cd '\r' \| wc -c` (misurato: 0), e scrivere accanto la controprova su un file CRLF — è la specie che i vincoli §4 chiamano *«un `grep -c` che deve rendere 0 e non è mai stato provato dove rende 1»* |
| R11-6 | Compito 3, Passo 7, sonda `the_committed_fixtures_match_the_schema` — e la riga *«il generatore dettato lo scrive davvero così?»* di `ipc_v1.map` | La sonda confronta `.bin` e `.json` e la lista dei file di troppo filtra `name.ends_with(".bin") \|\| name.ends_with(".json")`: **`ipc_v1.map` non è confrontato con niente**. Il solo controllo sulla mappa è, nel criterio di chiusura, `grep -c '^stamp 0x' … → 1` — che conta **una riga, non un valore** | Applicata la mutazione **G8** (tolta la lunghezza da `build_stamp`) nella copia: `cargo test --locked -p kernel --test ipc_wire` → **`test result: ok. 12 passed; 0 failed; 1 ignored`** — tutto verde — mentre `grep '^stamp 0x' gui/schema/fixtures/ipc_v1.map` dice `stamp 0x47b554435836e861` e il generatore rilanciato scrive `stamp 0xee1f22ae2aaa9eeb`. ⛔ **E la mappa è load-bearing**: **D52** — *«il timbro che la SPA manda in `Hello` si legge dall'ultima riga di `ipc_v1.map`, in `gui/src/schema/stamp.ts`»* — e la §4 del disegno del 2 la vuole *«scritta nella mappa per la GUI: schema cambiato → byte cambiati → timbro cambiato → GUI vecchia rifiutata»*. Con la mappa stantia la SPA manda un timbro che il core non calcola più: risposta `StaleBuild`, GUI che non parte, **cancello verde** | CONFERMATO | fatto | sì | Nel **Passo 7**, dentro `the_committed_fixtures_match_the_schema`, confrontare anche l'ultima riga della mappa: `let stamp_line = format!("stamp {:#018x}", build_stamp().get());` contro l'ultima riga non vuota di `ipc_v1.map`, con un messaggio che dice «REGENERATE them» come gli altri. E nel **Passo 8** una quarta mutazione (la seconda direzione): con G8 attiva la sonda nuova deve andare **rossa** — oggi non c'è niente che vada rosso |
| R11-7 | Compito 3, Passo 9 (b), richiamo datato in coda al doc di `IpcMessage::encode`: frase `Eleven variants arrived, and they put` | Il richiamo, che il compito **scrive nel sorgente**, dice *«**Eleven** variants arrived»* | `git show 42b50d8:crates/kernel/src/wire/ipc.rs \| awk '/^pub enum IpcMessage/{s=1} s&&/^}/{exit} s&&/^    [A-Z]/{c++} END{print c}'` → **2**; lo stesso `awk` sull'enum dettato dal Passo 2 (e sul modello compilato) → **14**. Le nuove sono **dodici**. ⛔ **È l'ultima casa di P-35**, che ordinava di **togliere** il numerale dalle otto case e la cui cura porta il comando `grep -c undici`: `grep -c 'undici' <piano>` non tocca questa riga perché è **in inglese**. `grep -in 'eleven' <piano>` → **una sola riga, la 5259**, che è questa. Il diario del piano (riga 21943) registra che la guardia *««undici» non deve più comparire»* «è scattata su sé stessa» — e l'inglese è passato | CONFERMATO | fatto | sì | Nel Passo 9 (b) **togliere il numerale** invece di riallinearlo a «Twelve», come P-35 prescrive: *«The variants added today put `String` and `Vec<u8>` into it»* — il conteggio lo dà già il comando `awk` del criterio di chiusura. E aggiungere `grep -ci 'eleven'` accanto a `grep -c undici` nella cura di P-35 |
| R11-8 | Compito 3, Passo 3, doc di `stamp_set`: frase `no field is left at its type's default` | *«⚠️ THE VALUES ARE ARBITRARY BUT NOT RANDOM: each one is chosen so that no two encodings are equal and **no field is left at its type's default**, because a fixture full of zeroes cannot tell a field that is written from one that is skipped»* | Tre righe sotto, nello stesso blocco: `IpcMessage::Degradation(DegradationReport { vram_exhausted: true, routing_degraded: false })`. `false` **è** `bool::default()`, quindi la fixture `03-degradation` non distingue «`routing_degraded` scritto a false» da «`routing_degraded` saltato». ⛔ **E l'alternativa è chiusa da P-34**: con due `bool` non si può avere insieme «nessuno al proprio default» e «i due campi diversi fra loro» — *«two equal strings at two offsets pin two offsets, and that is false: they pin ONE offset and its mirror image»* | CONFERMATO | fatto | sì | Nel Passo 3 **dichiarare l'eccezione invece di affermare l'assoluto**: «no field is left at its type's default **except `DegradationReport::routing_degraded`, where the two-`bool` shape forces a choice between this rule and P-34's, and P-34's wins**» — così il prossimo che legge non «corregge» la fixture cadendo nel buco del 2026-09-01 |
| R11-9 | Compito 3, Passo 6, commento di `regenerate_the_fixtures`: frase `as 'scripts/gate.sh' requires of every ignored test` | Il commento dettato, che finisce **nel sorgente**, attribuisce a `scripts/gate.sh` una regola: `#[ignore]` con una ragione «come `scripts/gate.sh` pretende da ogni test ignorato» | `grep -rn 'ignore' scripts/` → sette righe, **nessuna** sui test ignorati (sono tutte `git check-ignore` di `check-docs.sh` e `.gitignore` in un commento di `gate.sh`); `grep -n '^run ' scripts/gate.sh` → sei passi, nessuno che guardi gli `#[ignore]`. I due `#[ignore]` di oggi (`crates/simulator/tests/dst_campaign.rs:708`, `crates/platform/tests/engine_crash_consistency.rs:721`) portano la ragione **per convenzione**, citando il *«constraint 8 of §11»* della spec, non un controllo | CONFERMATO | fatto | sì | Nel Passo 6 riscrivere la clausola sulla fonte vera: «`#[ignore]` with a reason, as `crates/simulator/tests/dst_campaign.rs` and `crates/platform/tests/engine_crash_consistency.rs` do» — oppure togliere l'attribuzione e lasciare la ragione («the gate must not rewrite artefacts it is checking»), che è vera da sola |
| R11-10 | Compito 3, Passo 8, riga **G7** della tabella delle mutazioni | «il controllo delle fixture rosso su «left over» con **entrambi** i file della variante tolta, `.bin` e `.json`» | Mutazione G7 applicata e misurata nella copia. `every_variant_is_in_the_canonical_set` → `variants missing from stamp_set: [11]` ✅ **esatto**. Ma il controllo delle fixture rende **sei** file di troppo e **quattro** mancanti, non due: togliere una variante **in mezzo** sposta di uno tutti gli indici successivi. Uscita vera: `mismatched: 11-request.bin / 11-request.json / 12-verdict.bin / 12-verdict.json (os error 2)` · `left over: 11-steps.bin, 11-steps.json, 12-request.bin, 12-request.json, 13-verdict.bin, 13-verdict.json` | CONFERMATO | fatto | no | Due vie, e la seconda è più economica: (a) riscrivere l'Atteso di G7 con l'uscita misurata, spiegando che è lo **slittamento degli indici**; (b) mutare l'**ultima** variante (`IpcMessage::Verdict(...)`), dove nessun indice slitta e l'Atteso scritto — due file di troppo — diventa vero alla lettera |
| R11-11 | Compito 3, testa, riga `⚠️ Costo dichiarato: le varianti nuove portano dieci tipi nuovi in wire::ipc` | «**dieci** tipi nuovi in `wire::ipc`» | Contati sul blocco dettato dal Passo 2 con un'ancora di riga: `awk 'NR>=79 && NR<=232' c3.md \| grep -nE '^pub (struct\|enum) '` → **undici**: `BuildStamp`, `Protection`, `DegradationReport`, `PolicyName`, `PolicyReport`, `Triple`, `Access`, `Call`, `Provenance`, `LayoutState`, `StepSummary`. Il blocco *Interfaces* ne elenca **dieci** in graffa **più** `BuildStamp` su una riga propria: il numerale conta la graffa e dimentica il primo | CONFERMATO | fatto | no | Stessa cura di P-35: **togliere il numerale** («le varianti nuove portano un tipo nuovo per ogni tipo del kernel con campi, più il timbro»), o portargli accanto il comando `grep -cE '^pub (struct\|enum) '` sul blocco del Passo 2 |
| R11-12 | Compito 3, Passo 9 (a) — e il doc di modulo di `crates/kernel/src/wire/ipc.rs`, frase `returns` + `a bench fake. The trigger is milestone 2 of the subproject` | Il Passo 9 (a) riscrive **quel capoverso** con un richiamo datato, e corregge due cose: l'innesco della revoca e *«Until it exists, NOTHING REFUSES A STALE GUI»*. Non tocca la riga accanto | `grep -rnE "^ *impl Ipc for" crates/` **oggi** → **due** (`ports_are_implementable.rs:456` `FakeGui`, `simulator/src/ipc.rs:143` `DyingGui`), già non «a bench fake» al singolare; e il **compito 2** ne aggiunge una vera (`impl Ipc for LocalSocketIpc`, riga 648 del suo testo) e il suo criterio pretende *«`grep -rnE "^ *impl Ipc for" crates/ --include='*.rs'` → **tre**: le due finte e `LocalSocketIpc`»*. Il compito 2 corregge tre righe in `ports/mod.rs` e `ports_are_implementable.rs` (P-23) e **non apre `wire/ipc.rs`**; il compito 3 lo apre e non corregge questa. È la quarta domanda del pre-controllo: *un compito precedente lascia false delle righe nel file che questo apre* | CONFERMATO | fatto | no | Una terza metà al richiamo (a) del Passo 9: «`'grep -rnE "^ *impl Ipc for" crates/' returns a bench fake` is false from task 2 of the same plan — `platform::ipc::LocalSocketIpc` is the real transport, and the command that counts is the one written above». In alternativa una quarta voce nel Passo 9-bis del **compito 2**, che è dove nasce la causa |
| R11-13 | Compito 3, Passo 10, messaggio di commit: frase `i richiami datati su P-16 (l'innesco della revoca e' il 7)` | «il 7» | Il richiamo (a) che quel commit porta dice `THE TRIGGER IS THEREFORE THE 3D CONSUMER, **subproject 7**`, e P-16 conclude «il **consumatore 3D** … il **7**». Ma dentro **questo piano** «il 7» è il **compito 7** (`kernel::serving`), citato così sette volte nella tabella della posizione e nei blocchi *Interfaces*; e il compito 7 è proprio quello che **usa** il timbro. Due lettori, due letture, in una riga che resta nella storia di git | CONFERMATO | prosa | no | «l'innesco della revoca e' il **sotto-progetto 7**» nel messaggio di commit del Passo 10. (E, se si vuole essere coerenti con **D76**, `subproject 7` → `sub-project 7` nel richiamo (a): il resto del compito scrive `sub-project 2` cinque volte) |

## Le tre righe del registro (`ledger.md`, «Compito 3»), verificate

| Riga del registro | Regge? |
|---|---|
| **R10-16** — l'etichetta `*.bin` (**binari**: niente fine-riga da conservare) in *Files* | **sì**. È l'unica voce di *Files* che manca di un'etichetta di fine-riga, e «binari» è la sola giusta: i `.bin` escono da `std::fs::write(&bytes)` |
| **D75** — `<data>` nei due `DATED RECALL` di `wire/ipc.rs` e nel criterio | **sì**, misurato sul modello: `grep -c 'DATED RECALL, <data>' <modello>` → **2**, e il criterio di chiusura porta `<data>` e non una data fissa |
| **D76** — «sub-project 2» nei commenti dettati; i due «milestone 6» restano | **sì nella sostanza, no nel numerale**: `git show ab1b138:<piano> \| awk '/^## Compito 3:/…' \| grep -oi 'milestone 2' \| wc -l` → **4** prima, **0** adesso; `grep -oF 'sub-project 2'` → **0** prima, **5** adesso; `grep -oi 'milestone 6'` → **2** ✅ (sono il Traguardo 6 del SP1). ⚠️ Il registro scrive *«nei **sei** punti»*: i punti sono **cinque**. Il numerale vive solo in `ledger.md`, non nel piano — non è un rilievo sul compito, ma il coordinatore lo sappia |

## Copertura del disegno per il mio perimetro

| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|
| `<disegno2>` §4, prima tabella, le 13 righe = **14 varianti** (`Hello`, `Accepted`, `StaleBuild`, `Degradation`, `Policy`, `Invoke`, `PermissionRequired`, `Approve`, `Token`, `Layout`, `SaveLayout`, `Steps`, `Request`+`Verdict`) | 3 · Passo 2, blocco `pub enum IpcMessage` | **coperta** — nomi identici uno per uno; `awk` sul modello → **14** |
| §4, richiamo del 2026-09-09: i nomi «da provvisori a **fissati**», `Steps` per la lista dei passi | 3 · Passo 2 | **coperta** |
| §4, riga `Token`: *«Sul filo un enum a due valori gemello di `Trust`»* | 3 · Passo 2, `Provenance` | **coperta** — ⚠️ i nomi delle varianti **non** sono quelli di `Trust` (`Instruction`/`Untrusted` → `Trusted`/`Untrusted`); scelta deliberata, ma nessuna riga dichiara la corrispondenza. Chi scrive la conversione (7 e 12) la deve dedurre |
| §4, seconda tabella, riga **le fixture**: *«un comando dichiarato genera i byte di ogni variante e una mappa `indice → nome → valore` in `gui/`, e li si committa»* | 3 · Passo 6, `regenerate_the_fixtures` | **coperta** — misurata: 14 `.bin`, 14 `.json`, `ipc_v1.map` |
| §4, seconda tabella, riga **il controllo**: *«ricodifica le fixture committate e le confronta: schema cambiato senza rigenerare → rosso»* | 3 · Passo 7, `the_committed_fixtures_match_the_schema` | **coperta** — provata con G6: due righe rosse, `.bin` **e** `.json` |
| §4, seconda tabella, riga **il timbro**: *«l'impronta … calcolata dal core all'avvio … e **scritta nella mappa per la GUI**: schema cambiato → byte cambiati → timbro cambiato → GUI vecchia rifiutata»* | 3 · Passo 3 (`build_stamp`) e Passo 6 (la riga `stamp` della mappa) | **coperta a metà** — l'artefatto esiste, **nessun controllo lo esercita**: è **R11-6**, misurato con G8 |
| §8, tabella degli artefatti, riga *«lo schema (§4, più `Layout`, `SaveLayout` e la lista dei passi)»*: fixture in `gui/`, ricodificate da `ipc_wire.rs`, *«il timbro cambia coi byte»*, *«`decode` verifica i byte consumati»* | 3 · Passi 6 e 7; `the_stamp_changes_when_the_schema_changes` (Passo 5); i sette banchi di oggi, intatti | **coperta** — la terza metà era già vera prima del compito |
| `<stella>` «La GUI dentro», **sequenza 1** (l'accoglienza): `Hello`→`Accepted`→`Degradation`→`Policy`→`Layout`→«la lista dei passi»; ramo `StaleBuild` | 3 · Passo 2 per il **vocabolario**; la sequenza è del compito **7** | **coperta** — tutti e sette i messaggi della sequenza esistono come varianti |
| `<stella>` §3, riga **2** (*«il filo … stretta di mano col timbro … — invariato»*) | compiti 2 e 7; del 3 è `build_stamp()` che la stretta di mano userà | **coperta** (fuori perimetro per il resto) |
| `<stella>` §3, riga **3** (*«lo schema che cresce: le varianti della §4 del 2 più tre — `Layout`, `SaveLayout`, la lista dei passi — con le fixture e il timbro»*) | 3 · Passi 2, 3, 6, 7 | **coperta** |

⚠️ **Una riga del disegno che nessuno tocca, e che il compito 3 rende imprecisa**: §8, riga dei nove pezzi,
*«lo schema che cresce **di tre varianti**»* (`sed -n '111p' <disegno2>`). Il «di tre» conta le tre aggiunte del
2026-09-09 rispetto alla §4 già allargata, non le varianti nuove rispetto al codice (dodici). Non è un rilievo sul
compito 3 — la riga descrive la **fetta**, non l'enum — ma è la stessa specie di P-35 e vale la pena saperlo.

## Voci P rimisurate

| P | Comando rilanciato → resa | Regge? |
|---|---|---|
| **P-4** — i record congelati sono SEI | `ls crates/kernel/tests/frozen/*.cbor \| wc -l` → **6**; e sul modello, dopo il compito 3, ancora **6** | **sì** |
| **P-10** — niente di ciò che il piano detta esiste già (per il 3) | `awk` delle varianti su `HEAD` → **2**; `ls gui` → `No such file or directory`; `grep -n 'pub fn stamp_set\|pub fn build_stamp\|struct BuildStamp' crates/` → niente | **sì** |
| **P-16** — l'innesco della revoca scade oggi e non va onorato; il richiamo va nel 3, `ports/ipc.rs` non si tocca | `grep -c -F '//! ⛔ AND THE REVOCATION core -> gui IS A DECLARED NON-CONSTRUCTION' crates/kernel/src/wire/ipc.rs` → **1** (il *Trova* del Passo 9 (a) esiste ed è unico); `grep -c 'BUILD STAMP' crates/kernel/src/ports/ipc.rs` → **2**, e il compito non apre quel file | **sì** |
| **P-17** — il doc di `encode` argomenta su un grafo che il compito allarga | `grep -c -F 'pub fn encode(&self) -> Result<Vec<u8>, WireError> {' crates/kernel/src/wire/ipc.rs` → **1**; il doc di oggi enumera davvero *«`Mib(u64)`, `ComputeClass`, `Preemption`, `Millis(u64)` and unit variants»* (righe 155-156), e il Passo 2 ci mette `String` e `Vec<u8>`. La sonda `a_string_in_the_schema_still_cannot_stop_the_encoder` compila e **passa** sul modello | **sì** — ⚠️ ma il testo del richiamo che la chiude porta R11-7 |
| **P-18** — le fixture vanno in `gui/`, che nasce all'11; `gui/` non è ignorata | `git check-ignore -v gui/schema/fixtures/x.bin` → **nessuna uscita, exit 1**; `ls gui` → non esiste; il generatore scrive in `CARGO_MANIFEST_DIR/../../gui/schema/fixtures`, cioè la radice — verificato nella copia, dove ha creato `ws/gui/schema/fixtures/` | **sì** |
| **P-34** — i due campi di un dettaglio non devono essere uguali | riletta: è la ragione per cui `DegradationReport` non può avere entrambi i `bool` fuori dal default — vedi **R11-8** | **sì** |
| **P-35** — «le undici varianti» è falso, e vive in otto case | `grep -c 'undici' <piano>` → **16**, nessuna delle quali nel compito 3 (le otto case italiane sono state svuotate ✅); `grep -in 'eleven' <piano>` → **una**, riga **5259**, dentro il compito 3 | **no** — è **R11-7** |
| **P-39** — `StepSummary.done` è `bool` e non `Option<bool>` | il Passo 2 detta `pub done: bool` col capoverso che cita P-39; nel modello compila e la fixture `11-steps.json` porta `"done":true` | **sì** |
| **P-56** — gli accessori di `Core` sono cinque | fuori perimetro; citata dal compito 3 solo come precedente del divieto di numerali in prosa. Non rimisurata (è di R3/R4) | — |
| **P-62** — il valore atteso in JSON, non il `Debug` di Rust | il richiamo in testa al compito 3 lo applica: `variant_json` esiste, `python -c "import json,glob; …"` sulle 14 fixture generate → `every fixture is valid JSON`; `type(json.load(…)['value']).__name__` → `str` | **sì** |
| **P-89 / D56** — `StepSummary` porta tre campi | il Passo 2 ne detta **tre** (`step`, `function`, `done`); il richiamo alla riga 1 della tabella Passi della stella polare è dei **Files del compito 14** (`D56`), non del 3 | **sì**, e il 3 non ha nulla da scrivere |

## Numeri di compito ricensiti

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| *Interfaces*, `Produces, e i compiti 7, 9, 11, 12, 13 e 14 li usano` | 7, 9, 11, 12, 13, 14 | i blocchi *Interfaces* di 7, 9, 11, 12 dichiarano di consumare `kernel::wire::ipc::…`; il 13 consuma `ipc_v1.map` (D52) e i tipi TS dell'11; il 14 consuma i tipi TS dell'11 | **giusto** — ⚠️ il 13 e il 14 consumano da `gui/src/schema/`, non da `kernel::wire::ipc`: sono consumatori **indiretti**, ed è già ciò che la riga successiva dice (*«le fixture …, che il compito 11 legge»*) |
| *Files*, `il compito 11 legge da gui/src/schema/` | 11 | riga 11 della posizione = «`gui/` nasce … `schema/` coi tipi e le fixture» | **giusto** |
| Passo 2, doc di `Call`: `task 6 brings crate::record::Invocation, the DURABLE detail` | 6 | riga 6 = «il registro delle funzioni … il dettaglio `Invocation`» | **giusto** |
| Passo 2, doc di `Triple`: `The probe is born with that consumer, task 7` | 7 | riga 7 = «`kernel::serving`, il dispaccio» — è chi riceve l'`Approve` | **giusto** |
| Passo 9 (a): `the handshake that uses it is task 7 of the same plan` | 7 | idem | **giusto** |
| Passo 9 (a): `THE TRIGGER IS THEREFORE THE 3D CONSUMER, subproject 7` | sotto-progetto 7 | il pilastro 3D; lo stesso chiusore della riga 27 del Traguardo 6 | **giusto** (è un **sotto-progetto**, non un compito) |
| Passo 10, messaggio di commit: `l'innesco della revoca e' il 7` | 7 | dentro questo piano «il 7» è il **compito 7** | **ambiguo** — è **R11-13** |
| il compito **non** cita 8, 10, 15, 16, 17 | — | — | nessun numero stantio di **D25** (che spostò di uno dal 9 al 16), **D13**, **D21**, **D47**, **P-81**, **P-85** sopravvive nel 3 |

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

```
git -C <repo> rev-parse --short HEAD                                              → eee70d9
wc -lc <scratchpad>/review/skeleton.md <scratchpad>/review/p-titles.md            → 704/93366, 116/15009
awk '/^## Compito 3:/{f=1} /^## Compito 4:/{exit} f' <piano> > probe-R11/c3.md    → 947 righe
awk '/^### Compito 3/{f=1} /^### Compito 4/{exit} f' …/ledger.md                  → tre righe note (R10-16, D75, D76)
grep -n '^```' probe-R11/c3.md                                                    → 13 recinzioni, posizioni usate per estrarre i blocchi
tr -cd '\r' < probe-R11/c3.md | wc -c                                             → 0 (il piano è LF)
grep -c 'Request(GrantRequest)\|Verdict(Verdict)' crates/kernel/src/wire/ipc.rs   → 2   (Atteso: due varianti ✅)
grep -n 'pub enum IpcMessage' -A 8 crates/kernel/src/wire/ipc.rs                  → Request, Verdict ✅
ls gui                                                                            → No such file or directory ✅
ls crates/kernel/tests/frozen/*.cbor | wc -l                                      → 6 ✅ (P-4)
git ls-files --eol crates/kernel/src/wire/ipc.rs crates/kernel/tests/ipc_wire.rs  → i/lf w/crlf, i/lf w/crlf ✅
grep -c '#\[test\]' crates/kernel/tests/ipc_wire.rs                               → 9   (Atteso: sette ❌ → R11-1)
grep -o '#\[test\]' crates/kernel/tests/ipc_wire.rs | wc -l                       → 9
grep -n '#\[test\]' crates/kernel/tests/ipc_wire.rs                               → 7 attributi + 2 dentro commenti (righe 44, 102)
git show 42b50d8:crates/kernel/tests/ipc_wire.rs | grep -c '#\[test\]'            → 9 (identico allora)
awk '/^pub enum IpcMessage/…{print c}' crates/kernel/src/wire/ipc.rs              → 2
git show 42b50d8:…/wire/ipc.rs | awk '/^pub enum IpcMessage/…'                    → 2
grep -c 'BUILD STAMP' crates/kernel/src/ports/ipc.rs                              → 2 (il file non si tocca)
wc -l crates/kernel/src/wire/ipc.rs crates/kernel/tests/ipc_wire.rs               → 200, 116
grep -n 'ipc_wire\|cargo test' scripts/gate.sh                                    → il cancello lancia `cargo test --locked --workspace` (riga 40): ipc_wire ci gira dentro
grep -n '^run ' scripts/gate.sh                                                   → sei passi; nessun fmt, nessun clippy, nessun -D warnings
grep -n 'RUSTFLAGS|-D warnings|deny|clippy' scripts/gate.sh                       → solo il commento «clippy … ha NO voice here»
grep -rn 'ignore' scripts/                                                        → 7 righe, nessuna sui test ignorati → R11-9
grep -rn '#\[ignore' crates/                                                      → 2 (dst_campaign.rs:708, engine_crash_consistency.rs:721), entrambi con ragione
grep -rnE "^ *impl Ipc for" crates/                                               → 2 (FakeGui, DyingGui) → R11-12
grep -rn 'LENGTH_WIDTH' crates/                                                   → definita in framing.rs, usata da ipc_wire.rs e worker_wire.rs
sed -n '1,15p' crates/kernel/tests/ipc_wire.rs                                    → gli use di oggi
awk '/^## Compito 2:/…' <piano> | grep -n 'impl Ipc|wire/ipc.rs'                  → il 2 scrive `impl Ipc for LocalSocketIpc` e NON apre wire/ipc.rs
awk '/^## Compito 2:/…' <piano> | sed -n '760,800p'                               → il Passo 9-bis: tre richiami, in ports/mod.rs e ports_are_implementable.rs
cp Cargo.toml Cargo.lock rust-toolchain.toml + cp -r crates  → probe-R11/ws       → copia fatta, CR conservati (200 CR / 200 righe)
python probe-R11/apply.py                                                         → Passi 2,3,5,6,7,9 applicati; 504/504 e 511/511 CR/righe; DATED RECALL, <data> → 2
cargo build --locked -p kernel            (nella copia)                           → ❌ E0433 cannot find type `Millis`  → R11-2
[aggiunto `use crate::time::Millis;`] cargo build --locked -p kernel              → ✅ Finished, zero avvisi
cargo test --locked -p kernel --test ipc_wire                                     → ❌ E0423 BuildStamp(0) → R11-3 ; warning unused imports ×3 → R11-4
[due riparazioni dichiarate] cargo test --locked -p kernel --test ipc_wire        → 11 passed, 1 failed (fixtures assenti), 1 ignored
cargo test … -- --ignored regenerate_the_fixtures                                 → ok; crea ws/gui/schema/fixtures/
ls gui/schema/fixtures/*.bin | wc -l ; ls …/*.json | wc -l                        → 14 e 14 ✅
grep -c '^stamp 0x' gui/schema/fixtures/ipc_v1.map                                → 1 ✅ ; tail -3 → `stamp 0x47b554435836e861`
cat gui/schema/fixtures/00-hello.json                                             → {"kind":"Hello","value":"81985529216486895"} ✅ stringa
tr -cd '\r' < gui/schema/fixtures/*.json gui/schema/fixtures/ipc_v1.map | wc -c   → ambiguous redirect, poi 0 → R11-5
tr -cd '\r' < <un file CRLF> | wc -c                                              → 2 (controprova della direzione che deve rendere ≠ 0)
cat gui/schema/fixtures/*.json …/ipc_v1.map | tr -cd '\r' | wc -c                 → 0 (la misura vera)
python -c "import json,glob; [json.load(open(p)) …]"                              → every fixture is valid JSON ✅
python -c "… type(json.load(open('00-hello.json'))['value']).__name__"            → str ✅
cargo test --locked -p kernel --test ipc_wire                                     → 12 passed, 1 ignored ✅ (criterio di chiusura verificato)
[G6] cargo test …                                                                 → rosso, due righe: 06-…bin different bytes + 06-…json different value ✅
[G7] cargo test …                                                                 → variants missing from stamp_set: [11] ✅ ; ma 6 «left over» e 4 «mismatched» → R11-10
[G8] cargo test …                                                                 → 12 passed, 1 ignored — tutto VERDE
[G8] grep '^stamp 0x' …/ipc_v1.map ; poi rigenerato                               → 0x47b554435836e861 vs 0xee1f22ae2aaa9eeb → R11-6
[ripristino] cargo test …                                                         → 12 passed, 1 ignored; stamp di nuovo 0x47b554435836e861
awk 'NR>=79 && NR<=232' c3.md | grep -nE '^pub (struct|enum) '                    → 11 tipi → R11-11
awk 'NR>=239 && NR<=272' c3.md | grep -cE '^    [A-Z]'                            → 14 varianti dettate
grep -n -F 'Eleven variants' c3.md                                                → riga 890 → R11-7
grep -c 'undici' <piano> ; grep -in 'eleven' <piano>                              → 16 (nessuna nel 3) ; una sola, riga 5259
grep -o -F 'sub-project 2' c3.md | wc -l ; grep -ci 'milestone 2' c3.md           → 5 ; 0 ✅ (D76)
grep -oi 'milestone 6' (sul compito 3)                                            → 2 ✅ (restano, sono il Traguardo 6 del SP1)
git show ab1b138:<piano> | awk '/^## Compito 3:/…' | grep -oi 'milestone 2'|wc -l → 4 (prima di D76)
grep -c -F '//! ⛔ AND THE REVOCATION core -> gui …' crates/kernel/src/wire/ipc.rs → 1 (Trova unico ✅)
grep -c -F 'pub fn encode(&self) -> Result<Vec<u8>, WireError> {' …               → 1 (Trova unico ✅)
sed -n '20,32p' crates/kernel/src/degradation.rs                                  → vram_exhausted, routing_degraded ✅
sed -n '110,120p' crates/kernel/src/permission.rs                                 → tool/resource &'static str + operation ✅
sed -n '60,70p' crates/kernel/src/arbiter/policy.rs                               → VramPolicy::Remote(RemotePolicy)/Local(LocalPolicy) ✅
sed -n '45,75p' crates/kernel/src/arbiter/resource.rs                             → Mib(#[n(0)] u64) con minicbor E bincode, new/get const ✅
grep -n 'pub enum Trust' -A 8 crates/kernel/src/record.rs                         → Instruction / Untrusted
grep -n 'pub enum Operation' -A 8 crates/kernel/src/permission.rs                 → Read / Write ✅
git check-ignore -v gui/schema/fixtures/x.bin                                     → nessuna uscita, exit 1 ✅ (P-18)
grep -n '^### §' <disegno2> ; sed -n '207,246p' <disegno2>                        → §4 letta: 13 righe = 14 varianti
sed -n '111p' e '417p' <disegno2>                                                 → «di tre varianti» ; la riga «lo schema» della §8
grep -n '^### ' <stella> ; sed -n '199,245p' e '675,700p' <stella>                → sequenza 1 e §3 righe 2-3 lette
grep -n '^| \*\*D11\|D12\|D35\|D52\|D56\|D75\|D76\*\* |' <piano>                  → righe D lette
awk '/^### P-4 |P-16|P-17|P-18|P-34|P-35 /…' <piano>                              → voci P lette
git -C <repo> status --porcelain                                                  → (vuoto)
git -C <repo> ls-files --eol <piano> <disegno2> <stella>                          → i/lf w/lf ×3, invariato
```

## Non verificato, e perché

- **`bash scripts/gate.sh` → `GATE GREEN` col compito applicato** (Passo 10 e criterio di chiusura). Non lanciato:
  il cancello va lanciato **nel repository**, e sarebbe una corsa dell'intero workspace su una copia senza `.git`
  (`check-docs.sh` e `gate-deps.sh` interrogano git). Ciò che il compito 3 può rompere l'ho misurato per pezzi:
  `cargo build --locked -p kernel` e `cargo test --locked -p kernel --test ipc_wire` (il cancello li copre col suo
  `cargo test --locked --workspace`), i fine-riga, e i sei `.cbor` congelati **invariati**.
- **`bash scripts/gate-deps.sh` verde** (criterio di chiusura). Non lanciato, ma il compito **non apre nessun
  manifesto** (`Files` non elenca né `Cargo.toml` né `Cargo.lock`) e `build_stamp` è FNV-1a scritto a mano: la lista
  di ADR-0031 non può crescere.
- **`git ls-files --eol` invariato dopo il compito.** Nella copia non c'è un indice git. Ho misurato la metà che
  conta — l'albero di lavoro resta CRLF: `504 CR / 504 righe` e `511 CR / 511 righe` sui due file dopo aver applicato
  i Passi con Python `newline=""`.
- **La riga 3 della tabella della posizione a ✅ con la data** (ultimo criterio di chiusura): è un'azione, non una
  verifica; non riproducibile in sola lettura.
- **`crates/kernel/src/ports/ipc.rs` non si tocca**: verificato che il compito non lo apra e che
  `grep -c 'BUILD STAMP'` valga **2** oggi; che valga 2 **dopo** il compito è vero per costruzione.
- **La corrispondenza `Trust::Instruction` ↔ `Provenance::Trusted`**: nessun compito del mio perimetro scrive la
  conversione (la scrivono il 7 e il 12), quindi non ho potuto provarla da fuori. Segnalata nella copertura.

### Un dubbio cercato e NON riprodotto (una riga l'uno, così non si ricerca)

- **Gli indici `bincode` di `Request` e `Verdict` si spostano da 0 e 1 a 12 e 13**, perché `bincode` numera per
  posizione e il Passo 2 li conserva «in coda». **Non è un difetto**: il modulo dichiara *«NO VERSION ENUM, NO
  RETIRED-INDEX REGISTER, NO FROZEN BYTES — I4 renounces versioning»*, nessun artefatto persiste byte `ipc`
  (i `.bin` si **rigenerano**, i sei `.cbor` congelati sono del **giornale**), e il timbro è esattamente il
  meccanismo che rifiuta un pari costruito con l'ordine vecchio.
- **`grep -c 'DATED RECALL, <data>' … → 2` del criterio di chiusura** sembra contraddire «con la data del giorno
  scritta al posto di `<data>`». **Non lo è**: `<data>` è segnaposto anche nel comando, come **D75** prescrive e
  come il compito 2 già scrive (*«Atteso (con la data scritta)»*).
- **`the_committed_fixtures_match_the_schema` senza la cartella** dà un panico di `std::fs::read_dir`
  (`os error 3`) invece del messaggio «REGENERATE them». Misurato, ma **non è un caso raggiungibile**: il Passo 6
  precede il 7 e il commit porta le fixture. Registrato e non proposto come rimedio.

## Stato finale

`git -C /c/Users/zagor/Desktop/harness status --porcelain` → **(vuoto)** · `git ls-files --eol` di piano, disegno del 2
e stella polare → `i/lf w/lf` ×3, **invariato** · `git rev-parse --short HEAD` → `eee70d9`, come all'inizio.
Tutte le prove vivono in `<scratchpad>/review/probe-R11/` (`ws/`, `c3.md`, `c11.md`, `apply.py`, `mutate.py`,
`ipc.rs.clean`, `ipc_wire.rs.clean`) e i bersagli di compilazione in
`C:\Users\zagor\AppData\Local\Temp\probe-R11-target`. **Nessun file del repository è stato aperto in scrittura.**
