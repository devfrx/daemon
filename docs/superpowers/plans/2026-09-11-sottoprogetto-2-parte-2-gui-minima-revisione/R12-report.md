# Rapporto R12 — Compito 8: la specie `Policy` del giornale, e la policy riletta dal giornale — 2026-09-16

## Esito in tre righe

1. Il compito 8 **non compila e non passa**, com'è scritto: modellato per intero in una copia del workspace
   (con il compito 6 modellato a monte per gli indici 6/3), `cargo build --locked -p kernel --tests` rende
   **tre rossi distinti** — `E0422` su `PolicyDetail` in `arbiter/mod.rs`, `E0433` × 7 sui percorsi di modulo
   `reconcile::` e `arbiter::` nei due banchi, `E0004` su un **quinto** sito esaustivo che il compito non nomina
   (`match detail` in `frozen_bytes.rs:448`) — più `E0369`/`E0277` sulla prima sonda del Passo 7, perché
   `VramPolicy` **non deriva né `PartialEq` né `Debug`**.
2. Riparati i rossi, **una sonda del Passo 4 fallisce a runtime**: `a_policy_record_does_not_put_a_step_in_doubt`
   scrive una `note` su un passo **senza intento**, e `MemoryJournal::note` risponde `OutOfOrder` — la «prima
   direzione» di §7.1.1 regola 3 non è tenuta da nulla. E la **misura scritta nel commento dettato del Passo 3 è
   falsa**: con `enter` al posto del braccio vuoto, una transizione **completa** NON resta in dubbio (16/16 verdi
   in `arbiter_policy`), perché la nota si posa **prima** dell'esito; il solo rosso è il declassamento della
   risoluzione. Il commit scriverebbe quella frase nel sorgente del kernel.
3. Il resto del compito regge e bene: gli indici **7** e **4** sono liberi, la tabella degli offset del Passo 9 è
   **esatta al byte** (`82 00 81 86 07 …`, 26 byte, `f5` a 25), la mutazione del Passo 7 rende rossa **solo** la
   quarta sonda, il blocco *Interfaces* («il compito 9 li usa») è **giusto** — il 10 e il 12 non ne nominano
   nessuno — e le tre asserzioni `all(..)` del Passo 5 restano vere. **13 rilievi CONFERMATI, 8 bloccanti**; più
   **8 righe NON RIPRODOTTO**, perché il coordinatore non le ricerchi.

## Rilievi

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R12-1 | 8 · Passo 4, la sonda `a_policy_record_does_not_put_a_step_in_doubt` — *«That is why this probe writes the record BY ITSELF»* | scrive il record `Policy` con `journal.note(StepId::new(1), …).expect("the memory journal accepts")` **senza intento**, e la chiama «la prima direzione» | modello compilato ed eseguito: `cargo test --locked -p kernel --test reconciliation` → `test a_policy_record_does_not_put_a_step_in_doubt ... FAILED`, `panicked … the memory journal accepts: OutOfOrder`. Il perché è nel sorgente: `grep -n 'fn note' -A 10 crates/simulator/src/journal.rs` → `if !self.has_intent(step) { return Err(JournalError::OutOfOrder); }`, e il **disegno lo dice già** (`grep -n 'Journal::note. pretende un intento' <disegno2>`). Le due sonde gemelle già scritte lo sanno: `a_permission_does_not_put_a_step_in_doubt` scrive **intent → note → outcome → note** | CONFERMATO | fatto | **sì** | Passo 4, prima sonda: sulla forma della gemella — `intent` (`EffectClass::Idempotent`), poi la `note` con il record `Policy`, poi `outcome`, **poi una seconda `note` dopo l'esito** (è il caso che separa «non apre» da «non **ri**apre», ed è la sola forma in cui `enter` si fa cogliere qui), e l'asserzione resta `steps_in_doubt(&journal)…is_empty()` |
| R12-2 | 8 · Passo 3, il braccio dettato — *«`steps_in_doubt` answered `[InDoubt { step: StepId(1), .. }]` on a transition that completed»* | il commento che il commit scrive in `crates/kernel/src/reconcile.rs` dichiara **misurate** le due risposte scartate: `enter` lascerebbe aperta per sempre una transizione **completata**; e «Held in BOTH directions … by `a_policy_record_does_not_put_a_step_in_doubt` and `a_policy_record_leaves…`» | misurato nel modello, braccio sostituito con `RecordKind::Policy => enter(&mut open, step, resolution_of(body.effect())),`: `cargo test --locked -p kernel --test reconciliation --test arbiter_policy` → **`arbiter_policy`: 16 passed, 0 failed** — `without_a_crash_a_transition_leaves_no_step_in_doubt` resta **verde**. Il solo rosso è `a_policy_record_leaves_the_doubt…`: `left: [InDoubt { step: StepId(1), resolution: RunAgain }]` / `right: [… SuspendAndAsk]`. ⛔ **Non può essere altrimenti**: il Passo 5(a) posa la nota **fra lo scambio e l'esito**, quindi su una transizione completa l'`enter` è sempre seguito dal `leave` dell'esito. È la specie che R2-15 ha già colto sul compito 6 | CONFERMATO | fatto | **sì** | Passo 3: riscrivere il primo trattino sulla misura vera — *«`enter` REPLACES the caller's resolution with this record's own: the step the transition declared `Idempotent` came back … — `a_policy_record_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them` answered `[InDoubt { step: StepId(1), resolution: RunAgain }]` against `[… SuspendAndAsk]`»* — e **rimisurarlo** dopo la cura di R12-1, perché la coppia cambia. La riga «Held in BOTH directions» resta vera solo se R12-1 è chiuso |
| R12-3 | 8 · Passo 7, la sonda `an_archive_with_no_transition_names_no_policy` — *«no transition was written, and the default is not the kernel's to name»* | `assert_eq!(arbiter::policy_now(&journal).expect("the archive reads back"), None, "…")` | `cargo build --locked -p kernel --test arbiter_policy` → `error[E0369]: binary operation == cannot be applied to type Option<VramPolicy>` (`note: VramPolicy does not implement PartialEq`) **e** `error[E0277]: VramPolicy doesn't implement Debug`. Alla fonte: `grep -n 'derive\|^pub enum\|^pub struct' crates/kernel/src/arbiter/policy.rs` → **nessun `#[derive]` nel file**: `RemotePolicy`, `LocalPolicy` e `VramPolicy` non hanno né `Debug` né `PartialEq`. Le altre tre sonde del Passo 7 non lo toccano perché passano per `.map(|policy| policy.name())` | CONFERMATO | fatto | **sì** | Passo 7, prima sonda: la stessa forma delle tre sorelle — `arbiter::policy_now(&journal).expect(…).map(\|policy\| policy.name())` confrontato con `None`. ⛔ **Non aggiungere un `derive` a `VramPolicy`**: sarebbe un tipo del kernel allargato per un banco, e il Passo 2 argomenta per esteso che `VramPolicy` **non è serializzabile né confrontabile** («its variants carry the policy objects themselves») |
| R12-4 | 8 · Passi 4 e 7 — `reconcile::steps_in_doubt(&journal)` e `arbiter::policy_now(&journal)` nelle sei sonde dettate | il Passo 5 elenca la crescita degli `use`: per `arbiter_policy.rs` **`Detail`, `EffectClass`, `PolicyDetail`**; per `reconciliation.rs` **`PolicyDetail`** | `cargo build --locked -p kernel --tests` → `error[E0433]: cannot find module or crate 'reconcile'` × **3** e `error[E0433]: cannot find module or crate 'arbiter'` × **4**. I due banchi importano gli **elementi**, non il modulo: `grep -n '^use ' crates/kernel/tests/reconciliation.rs` → `use kernel::reconcile::{InDoubt, Resolution, steps_in_doubt};`; in `arbiter_policy.rs` `grep -c 'arbiter::'` → **1**, la sola riga `use kernel::arbiter::{Admission, …}`. ⚠️ In `arbiter_policy.rs` `arbiter` è già un **nome di funzione** del banco (`arbiter(4_096, VramPolicy::Remote(RemotePolicy))`): il modulo e la funzione convivono (spazi di nomi diversi), ma il modulo va importato | CONFERMATO | fatto | **sì** | due righe in più nelle liste del Passo 5: `crates/kernel/tests/reconciliation.rs` riceve **`use kernel::reconcile;`** accanto a `use kernel::reconcile::{…}`, e `crates/kernel/tests/arbiter_policy.rs` riceve **`use kernel::arbiter;`** accanto a `use kernel::arbiter::{…}`. (In alternativa le sonde chiamano `steps_in_doubt(…)` e importano `policy_now` per nome — ma allora cambiano sei righe di codice dettato, e la forma `reconcile::steps_in_doubt` è quella che le sonde gemelle non usano: `grep -n 'steps_in_doubt(' crates/kernel/tests/reconciliation.rs` le mostra tutte **nude**) |
| R12-5 | 8 · Passo 6 — *«Gli `use` in testa al file crescono di `crate::record::{Detail, RecordError}` e di `RecordKind` se non c'è già»* | l'elenco della crescita degli `use` di `crates/kernel/src/arbiter/mod.rs` | applicato alla lettera: `cargo build --locked -p kernel` → `error[E0422]: cannot find struct, variant or union type 'PolicyDetail' in this scope`, in `arbiter/mod.rs:1140` — `policy_note` del Passo 5(b) costruisce `PolicyDetail { local }`. `grep -n '^use crate::record' crates/kernel/src/arbiter/mod.rs` → `use crate::record::{EffectClass, Record, RecordV1, Trust};` | CONFERMATO | fatto | **sì** | Passo 6: l'elenco diventa `crate::record::{Detail, PolicyDetail, RecordError, RecordKind}`. È la specie di R11-2 (`use crate::time::Millis` mancante nel compito 3) |
| R12-6 | 8 · *Files*, `crates/kernel/tests/frozen_bytes.rs` — *«`the_frozen_records()` da sette a otto, `POLICY_BYTES`, e il `match kind`»* | i siti esaustivi che questo compito tocca sono **quattro**, tutti su `RecordKind` (P-45) | esiste un **quinto** sito, ed è su `Detail`: `cargo build --locked -p kernel --tests` → `error[E0004]: non-exhaustive patterns: '&Detail::Invocation(_)' and '&Detail::Policy(_)' not covered`, `crates/kernel/tests/frozen_bytes.rs:450`. È l'unico `match` esaustivo su `Detail` fuori da `record.rs`: `grep -rn 'match detail' crates/ --include='*.rs'` → una riga. ⛔ **Il compito 6 lo nomina** (*«più l'`use`, l'array a mano … e il `match detail`»*, R2-13) e il compito 8 no | CONFERMATO | fatto | **sì** | la voce *Files* di `frozen_bytes.rs` diventa *«…, il `match kind` **e il `match detail`**»*, e il Passo 10 detta il braccio `Detail::Policy(_) => {}` accanto a `Detail::Permission(_) => {}` — col commento che dichiara il limite come i tre fratelli. ⚠️ Il Passo 8 ha già la rete (*«Se resta un quinto sito…»*), ma qui il sito è **noto**: costa una riga di lista, non un compito rifatto |
| R12-7 | 8 · Passo 8(a) e Passo 10 — i due bracci dettati che usano `PolicyDetail { local: true }` | nessuna riga dice che gli `use` dei due banchi crescono | `grep -n '^use kernel::record' -A 3 crates/kernel/tests/record_shape.rs` e `… frozen_bytes.rs` → nessuno dei due importa `PolicyDetail`. Misurato togliendolo di nuovo dal modello: `cargo build --locked -p kernel --test record_shape` → `error[E0422]: cannot find struct, variant or union type 'PolicyDetail' in this scope`. ⛔ **Il compito 6 lo dice per il proprio tipo** — *«`kernel::record::{…}` in testa al file riceve `InvocationDetail`»* (Passo 8-bis) — e il compito 8 tace | CONFERMATO | fatto | **sì** | una riga in coda al Passo 8(a) e una al Passo 10: *«`kernel::record::{…}` in testa a `record_shape.rs` e a `frozen_bytes.rs` riceve `PolicyDetail`»* |
| R12-8 | 8 · Passo 3, la seconda metà — *«`Trova` quella frase intera, presa dal file, e `Sostituisci con` la stessa più il richiamo datato»* | il blocco mostrato apre con `/// \`Arbiter::set_policy\`, since milestone 5 task 9, writes through` e chiude con `/// \`intent\` and \`outcome\` records whose \`kind\` matches each.` | Python con `newline=""` su `crates/kernel/src/reconcile.rs`: il blocco **come stampato** → **0** occorrenze; senza il `/// ` iniziale → **1**. Il file dice `/// \`RecordKind::Note\`; \`Arbiter::set_policy\`, since milestone 5 task 9, writes through` e prosegue `… matches each. Each writer carries its OWN`. Quindi il testo presentato come *«la stessa»* **non è la stessa**: preso alla lettera come *Trova* non trova nulla; preso come *Sostituisci* sull'ancora che trova, cancella `\`RecordKind::Note\`; ` e la frase `Each writer carries its OWN probe that pins the agreement — …` | CONFERMATO | fatto | **sì** | Passo 3: dettare il *Trova* **esatto** (`\`Arbiter::set_policy\`, since milestone 5 task 9, writes through\n/// \`intent\` and \`outcome\` records whose \`kind\` matches each.`, **senza** il `/// ` d'apertura) e il *Sostituisci* che ripete quel testo e aggiunge le quattro righe del richiamo — così `RecordKind::Note` e `Each writer carries its OWN` restano dove sono |
| R12-9 | 8 · *Files* — *«Modify: **questo piano** — la tabella della posizione e **D1** (**D25**)»* | D1 va toccata da questo compito | **è già fatta**: `grep -n '^| \*\*D1\*\* |' <piano>` → la riga porta già *«RICHIAMO DEL 2026-09-14 (D25) — qui stava «sedici»»*, e il numerale non c'è più. E nessun Passo dell'8 la nomina: `grep -n 'D1\b' <c8>` → due righe, la voce *Files* e una citazione di merito al Passo 0. Il Passo 13 tocca la sola riga 8 della posizione | CONFERMATO | fatto | no | togliere `e **D1** (**D25**)` dalla voce *Files*: resta *«Modify: questo piano — la riga 8 della tabella della posizione»*, com'è scritto nei compiti 9 e 10 |
| R12-10 | 8 · Passo 12, *«In coda alla cella, **senza toccare il testo che c'è**»* (due volte) | due richiami datati nei disegni, dettati come testo da appendere | nessun *Trova*, nessuno script, nessun criterio che li conti — e **«la cella» non è una sola**: la tabella *«Decisioni prese dal coordinatore»* ha **tre** colonne (`\| \| Decisione \| Perché, e che cosa costa se è sbagliata \|`) e la §9 del 2 ne ha **quattro** (`\| # \| Voce \| Chi la chiude \| Il consiglio, o la decisione presa \|`). Il precedente in casa è il **Passo 15 del compito 14**, che prende le ancore dal file per sezione e inizio di riga, pretende che siano una, `assert out.count("\n") == b.count("\n")`, e chiude con tre comandi. ⛔ **E manca la metà di R9b-5** (l'unica riga ⬜ del registro): il testo è proposto qui sotto, in *«La metà dell'8 di R9b-5»* | CONFERMATO | fatto | no | Passo 12 riscritto nella forma del Passo 15 del 14 — un solo script Python per i due file, `row()` che appende nell'**ultima** cella, l'`assert` sul numero di righe, e i `grep -c` con `<data>` nel criterio di chiusura — **più** il sotto-passo **(c)** di R9b-5 proposto sotto |
| R12-11 | 8 · Passo 9, punto 3 — *«La testa della mappa elenca le date in cui ogni file è nato — «the first three on 2026-08-10, the verdict on 2026-08-31, …»»* | una citazione fra virgolette della testa di `record_v1.map` | `grep -c -F 'the first three on 2026-08-10' crates/kernel/tests/frozen/record_v1.map` → **0**. Il file spezza la riga: `… from the hexadecimal\n# output of a throwaway probe deleted in the commit that added its file — the first three\n# on 2026-08-10, the verdict on 2026-08-31, the routing and the permission on 2026-09-01.` | CONFERMATO | fatto | no | citare il testo vero, o citare la sola frase che sta su una riga (`the verdict on 2026-08-31, the routing and the permission on 2026-09-01`) — così chi esegue la ritrova col `grep -F` |
| R12-12 | 8 · Passo 5(a) — *«`Trova` le **tre righe** dal `journal.intent(step, …)` al `)` che chiude `journal.outcome`»* | tre righe | sono **sei**: `crates/kernel/src/arbiter/mod.rs`, da `journal.intent(step, &transition_record(RecordV1::intent, policy.name()))?;` a `)`. Python `text.count(<le sei righe>)` → **1**, quindi l'ancora è buona; è il numerale a essere sbagliato, e un esecutore che prendesse tre righe si fermerebbe a `journal.outcome(` | CONFERMATO | fatto | no | *«le **sei** righe»*, oppure — meglio, perché non invecchia — *«dal `journal.intent(step, …)` al `)` che chiude `journal.outcome`, **intere, prese dal file**»* senza numerale |
| R12-13 | 8 · la §5 del disegno del 2, riga `\| il giornale \|` — *«poi l'effetto, che è il passo B di `set_policy` com'è»* | il compito 8 tocca due sole righe dei disegni: la decisione 56 della stella e la riga 9 della §9 del 2 | dopo il compito 8 il passo B di `set_policy` **non è più «com'è»**: porta tre record e non due, ed è esattamente il fatto per cui il Passo 3 detta un richiamo datato dentro `crates/kernel/src/reconcile.rs` (*«IT WRITES THREE NOW, NOT TWO»*) e il Passo 11 uno dentro `tests/serving.rs`. `grep -n 'il passo B di .set_policy. com.è' <disegno2>` → una riga, e nessun compito la tocca (`grep -n 'passo B' <piano>` sui compiti 6–9) | CONFERMATO | prosa | no | una riga in *Files* e un terzo richiamo al Passo 12: sulla cella *«il giornale»* della §5 del 2, *«✅ RICHIAMO DEL `<data>`, compito 8: sul passo B `set_policy` scrive **tre** record — intento, la nota della specie `Policy`, esito»*. In alternativa, dichiarare per iscritto perché non lo merita |
| R12-14 | 8 · Passo 1, tutte le attese | indici **7** e **4** liberi; `detail()` rende `Option<&Detail>`; i `match` esaustivi su `RecordKind` fuori da `record.rs` sono **quattro**; `transition_record` prende una `fn` di specie; `ls frozen/` → sette `.cbor` dopo il 6; `serving.rs` LF, gli altri `i/lf w/crlf` | rilanciati tutti e nove i comandi: `RecordKind` arriva a `#[n(5)] Permission` e `Detail` a `#[n(2)] Permission` → dopo il 6 (che prende 6 e 3) i liberi sono **7** e **4** ✅ · `pub fn detail(&self) -> Option<&Detail>` ✅ · i quattro `match` sono `reconcile.rs:197`, `frozen_bytes.rs:392`, `record_shape.rs` (`let of = \|kind\|` + array), `dst_campaign.rs:452` ✅ · `fn transition_record(species: fn(EffectClass, Trust, Vec<u8>, &'static str) -> RecordV1, policy: &'static str) -> Vec<u8>` ✅ · `ls crates/kernel/tests/frozen/` → **sei** `.cbor` oggi, **sette** dopo il 6 ✅ · `git ls-files --eol` → i nove file esistenti tutti `i/lf w/crlf` ✅ (`serving.rs` non esiste ancora: nasce LF al 7) | NON RIPRODOTTO | fatto | no | — |
| R12-15 | 8 · Passo 9, la tabella degli offset dell'ottavo record congelato | `0:82`, `1:00`, `2:81`, `3:86` (sei campi), `4:07` `RecordKind::Policy`, `5:effect`, `6:trust`, `7:payload b"frozen"`, `14:reason "frozen"`, `21:Some(Detail::Policy{local:true})` con l'indice **4** di `Detail` | eseguita **davvero** la sonda usa e getta nel modello: `82 00 81 86 07 01 00 46 66 72 6f 7a 65 6e 66 66 72 6f 7a 65 6e 82 04 81 81 f5`, `len = 26`. Ogni offset della tabella coincide; a 22 c'è `04`, l'indice 4 di `Detail`; a 25 c'è `f5` (CBOR `true`), che infatti non somiglia a nessun indice di variante | NON RIPRODOTTO | fatto | no | — |
| R12-16 | 8 · Passo 7 — *«si sostituisce l'assegnazione con un `return` anticipato, si lancia, si osserva **rossa solo quella**»* | solo `the_archive_names_the_last_transition_and_not_the_first` va rossa | eseguita nel modello (`current = Some(…)` → `return Ok(Some(…))`): `cargo test --locked -p kernel --test arbiter_policy` → `the_archive_names_the_last_transition_and_not_the_first ... FAILED`, **15 passed; 1 failed**. Esattamente come il piano dice | NON RIPRODOTTO | fatto | no | — |
| R12-17 | 8 · Passo 5 — *«Le tre asserzioni sotto — `reason`, `trust`, `payload` — restano `all(..)` e restano VERE per tutti e tre»* | le tre `assert!(records.iter().all(…))` non si toccano | modello eseguito: `a_policy_transition_writes_its_intent_before_its_outcome ... ok` con la sequenza a tre. `policy_note` scrive `self.policy.name()` **dopo** lo scambio, `Trust::Instruction` e `Vec::new()`: i tre predicati reggono su tutti e tre i record | NON RIPRODOTTO | fatto | no | — |
| R12-18 | 8 · *Interfaces* — *«Produces, e il compito **9** li usa con questi nomi esatti»* | il solo consumatore è il 9 | `awk` per compito su `RecordKind::Policy\|Detail::Policy\|PolicyDetail\|policy_now\|PolicyError\|RecordV1::policy`: **compito 9** → `policy_now` × 6 e `PolicyError` × 3; **compiti 10, 11, 12, 13, 14, 15, 16** → **nessuna occorrenza**. La riga è giusta (è la cura che R2-21 fece al 6) | NON RIPRODOTTO | fatto | no | — |
| R12-19 | 8 · Passo 8(b), il commento dettato in `dst_campaign.rs` — *«Nothing here changes the VRAM policy, so no policy record can enter this trace»* | il braccio `panic!` non scatterà | il compito **10** invoca davvero `POLICY_FUNCTION` (nove occorrenze) e quindi scrive record `Policy` — ma **crea** `crates/simulator/tests/serving_campaign.rs` e **non tocca** `dst_campaign.rs` (`awk` sulla sua lista *Files*), e `expected_doubt` non compare mai nel compito 10. La traccia di `dst_campaign.rs` resta quella di oggi: il commento è vero | NON RIPRODOTTO | fatto | no | — |
| R12-20 | 8 · le righe già applicate del registro, e D75/D76/D77 | R10-12 (D77): `E66`, `E112`, `E50` col nome del piano; D75: i richiami dettati portano `<data>`; D76: «milestone 5 task 9» è il Traguardo 5 del SP1 e resta | `grep -n 'E66\|E112\|E50' <c8>` → due righe, **entrambe** con *«del piano del Traguardo 6»* ✅ · `grep -n 'RECALL OF\|RICHIAMO DEL' <c8>` → **sei** richiami dettati, **tutti** con `<data>`, nessuna data fissa ✅ · `grep -n 'milestone' <c8>` → una riga sola, dentro la citazione del commento esistente di `reconcile.rs` ✅ · nessun commento dettato nomina un numero di compito di **questo** piano, quindi D76 non ha altro da mordere | NON RIPRODOTTO | fatto | no | — |
| R12-21 | 8 · Passo 5(c) — *«è UNA riga sola, misurata al 2026-09-14 a `crates/kernel/tests/arbiter_policy.rs:354`»* | una riga, alla 354 | `grep -n 'vec!\[RecordKind::Intent' crates/kernel/tests/arbiter_policy.rs` → **354**, ed è una riga sola. Python `count` sulle tre ancore del Passo 5(c) → **1**, **1**, **1** (`assert_eq!(entries.len(), 2, "an intent AND an outcome");`, `vec![RecordKind::Intent, RecordKind::Outcome]`, `assert_eq!(entries.len(), 2);`) | NON RIPRODOTTO | fatto | no | — |

⚠️ **Il conto, contato sulle righe della tabella qui sopra e non su questo elenco:** 21 righe — **13 CONFERMATE**,
di cui **8 bloccanti** (R12-1, R12-2, R12-3, R12-4, R12-5, R12-6, R12-7, R12-8), e **8 NON RIPRODOTTO**
(R12-14 … R12-21).

## La metà dell'8 di R9b-5 — il testo esatto del sotto-passo, da applicare senza reinventarlo

**Misurato prima di scriverlo** (comandi in fondo): la riga vive in `docs/superpowers/specs/2026-09-07-direzione-gui-design.md`,
sotto l'intestazione `## Registrate, non prese — del proprietario`; quella tabella ha **DUE** colonne —
l'intestazione è `| Voce | Chiusore proposto |` — quindi la cella «Chiusore» è l'**ULTIMA**, ed è quella in cui
`row()` del Passo 15 del 14 appende. La riga cercata è **unica** (`grep -c -F '| 🔶 **nata alla quarta ripresa, 2026-09-08** — la forma con cui la transizione di policy si rilegge dal giornale'` → **1**), finisce con ` |`,
e il suo chiusore oggi è *«il disegno del 2, con la §4 (lo schema) e la §5 (il registro); il piano ne fa un compito»* —
cioè **questo** compito. ⛔ **Il Passo 12(a) di oggi NON dice nulla di questa riga:** tocca la **decisione 56**, che
sta nell'**altra** tabella (`## Decisioni prese dal coordinatore`, tre colonne). Sono due righe diverse in due
tabelle diverse, e servono **entrambe**.

Sotto-passo da aggiungere al Passo 12, dopo (a) e (b):

````markdown
**(c)** ⛔ **La riga delle *Registrate, non prese* che questo compito CHIUDE — R9b-5.** Il chiusore della riga
*«la forma con cui la transizione di policy si rilegge dal giornale»* è *«il disegno del 2 …; il piano ne fa un
compito»*, e il compito è **questo**: la riga riceve il suo «✅ chiusa» in coda alla cella **«Chiusore proposto»**,
che è l'**ultima** delle due colonne di quella tabella, come le sei righe già chiuse che le stanno accanto.
⛔ **L'ancora si prende DAL FILE**, per sezione e inizio di riga, e lo script pretende che sia **una**; il file è
**LF** (Passo 1) e si tocca con Python, mai con `sed -i`. ⚠️ **`<data>` si sostituisce con la data del giorno in
cui il compito si esegue, nello script E nel `grep`**, o il `grep` rende 0.

```bash
python - <<'EOF'
import io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8")
p = "docs/superpowers/specs/2026-09-07-direzione-gui-design.md"
b = io.open(p, encoding="utf-8", newline="").read()
assert "\r\n" not in b, "the north star is LF: something rewrote it"
lines = b.split("\n")
DATE = "<data>"


def section(heading):
    starts = [i for i, line in enumerate(lines) if line.startswith(heading)]
    assert len(starts) == 1, f"{heading!r}: {len(starts)} headings"
    level = len(lines[starts[0]].split(" ")[0])
    end = next((i for i in range(starts[0] + 1, len(lines)) if re.match(r"^#{1,%d} " % level, lines[i])), len(lines))
    return range(starts[0], end)


def row(heading, prefix, recall):
    """Appends `recall` inside the LAST cell of the one row of `heading` that starts with `prefix`."""
    hits = [i for i in section(heading) if lines[i].startswith(prefix)]
    assert len(hits) == 1, f"{prefix!r}: {len(hits)} lines match"
    i = hits[0]
    assert lines[i].endswith(" |"), lines[i][-60:]
    lines[i] = lines[i][:-2] + " " + recall + " |"


row("## Registrate, non prese",
    "| 🔶 **nata alla quarta ripresa, 2026-09-08** — la forma con cui la transizione di policy si rilegge",
    "— ✅ **chiusa il " + DATE + ", compito 8 del piano della parte 2**: la forma è una **specie** e non una nota "
    "con un dettaglio, che non è pronunciabile (P-44, D26) — `RecordKind::Policy` all'indice 7, `Detail::Policy` "
    "all'indice 4, `PolicyDetail { local: bool }`, `RecordV1::policy`, scritta con `Journal::note` dentro "
    "`Arbiter::set_policy`; la rilettura è `kernel::arbiter::policy_now`, e i byte in più sono "
    "`crates/kernel/tests/frozen/record_v1_policy.cbor`")

out = "\n".join(lines)
assert out.count("\n") == b.count("\n"), "a line was added or lost: the recall goes IN a line"
tmp = p + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(out)
os.replace(tmp, p)
print("ok: one recall in", p)
EOF
grep -c 'chiusa il <data>, compito 8 del piano della parte 2' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
tr -cd '\r' < docs/superpowers/specs/2026-09-07-direzione-gui-design.md | wc -c
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
```

Atteso (con la data al posto di `<data>`, **anche nel `grep`**): **1**, **0**, e niente.
````

E le due righe che cambiano fuori dal Passo 12:

- *Files*, la voce della stella polare: `- Modify: docs/superpowers/specs/2026-09-07-direzione-gui-design.md (**LF**) — il richiamo datato sulla **decisione 56**, **e il «✅ chiusa» sulla riga delle *Registrate, non prese* «la forma con cui la transizione di policy si rilegge dal giornale» (R9b-5)**`
- **Criterio di chiusura**, una riga nuova: `` grep -c 'chiusa il <data>, compito 8 del piano della parte 2' docs/superpowers/specs/2026-09-07-direzione-gui-design.md `` → **1**, con la data vera al posto di `<data>`.

⚠️ **Baseline misurata oggi**, perché il criterio non sia vacuo: `tr -cd '\r'` sul file → **0**; l'`awk` delle
tabelle spezzate → **niente**; le righe già chiuse di quella tabella → **sei**; l'intestazione
`## Registrate, non prese` → **una**.

## Copertura del disegno per il mio perimetro

| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|
| `<stella>` *Registrate, non prese*, riga *«la forma con cui la transizione di policy si rilegge dal giornale»* — chiusore *«il piano ne fa un compito»* | **nessuno**: il Passo 12(a) tocca la **decisione 56**, che è un'altra riga in un'altra tabella | **scoperta** → R12-10, e il testo è proposto sopra |
| `<stella>` decisione **56** — *«scrive sul passo B una **nota** con un **dettaglio tipizzato**»* | 8 · Passo 12(a), richiamo datato | **contraddetta, e dichiarata**: P-44 e D26 dicono che la metà «nota» non è pronunciabile; il richiamo lo scrive con la data. ✅ |
| `<stella>` decisione 56, *«la sonda `E115` ② e la campagna dell'arbitro si rileggono al piano»* | 8 · Passo 5(c), i due richiami su `a_policy_transition_writes_its_intent_before_its_outcome` e `a_transition_names_the_policy_it_moves_to` | **coperta**, e P-44 registra che le sonde erano **tre** e non due ✅ |
| `<disegno2>` §9 riga **9** — *«come il daemon rilegge la policy all'avvio … il dettaglio tipizzato scritto da `set_policy` sul passo B (decisione 56); un record congelato in più»* | 8 · Passo 12(b), richiamo datato; e Passi 2, 5, 6, 9 per l'artefatto | **coperta** ✅ (il *daemon* è il 9, come D25 vuole) |
| `<disegno2>` §5 riga `\| il giornale \|` — *«poi l'effetto, che è il passo B di `set_policy` com'è»* | **nessuno** | **contraddetta, senza richiamo** → R12-13 |
| `<disegno2>` §5 riga `\| il giornale \|` — *«con `Invocation` diventano **sette**, e i sei vecchi restano identici al byte»* | il **6** (il richiamo è suo); l'8 aggiunge l'ottavo | **coperta**: la frase resta vera di `Invocation`, e il Passo 10 dell'8 pretende che i sette vecchi non si muovano (ADR-0036) ✅ |
| `<disegno2>` testa, *«`Arbiter::set_policy(policy, step) …` scrive intento ed esito **sul passo che riceve**»* | — | **coperta per data**: la lista è sotto *«Riletto il 2026-09-09»*, cioè un'istantanea datata del codice di allora, non un'affermazione al presente ✅ |
| `<ADR-0006>` rimando del 2026-09-08 in testa — *«il profilo dà il default … la policy corrente è la proiezione del giornale»* | 8 · Passo 6, il doc di `policy_now` e **D27** | **coperta** ✅ (`head -40 docs/adr/0006-*.md` → il rimando c'è, e dice quelle due cose) |

## Voci P rimisurate

| P | Comando rilanciato → resa | Regge? |
|---|---|---|
| **P-4** — *«I record congelati sono SEI, e il settimo è `Invocation`»* | `ls crates/kernel/tests/frozen/` → sei `.cbor` più `record_v1.map` | ✅ regge; l'ottavo è di questo compito, e il criterio di chiusura (`ls … \*.cbor \| wc -l` → **otto**) è aritmeticamente giusto |
| **P-33** — i `match` esaustivi su `RecordKind` | `grep -rn 'RecordKind::Permission' crates/ --include='*.rs' \| grep -v 'src/record.rs'` → undici righe in sei file; i `match` esaustivi sono quattro | ✅ regge **col richiamo del 2026-09-15** (quattro, non due). ⚠️ **Nessuno dei due conta i `match` su `Detail`**, ed è da lì che arriva R12-6 |
| **P-44** — *«una NOTA con un dettaglio tipizzato non è costruibile»* | `grep -nE '    pub fn (intent\|outcome\|note\|verdict\|routing\|permission)\(' crates/kernel/src/record.rs` → `note(effect, trust, payload, reason)` non prende dettaglio; `sed -n '226,246p' crates/kernel/src/permission.rs` → `is_granted` filtra per `kind` e legge `Detail::Permission` | ✅ regge in pieno. La sonda che asserisce il contrario esiste davvero: `assert_eq!(record.detail(), None);` in `a_transition_names_the_policy_it_moves_to`, e il Passo 5(c) la riscrive |
| **P-45** — i quattro siti, e *«vale per il compito 8 identico … la lista *Files* del compito 8 li nomina tutti e quattro»* | la lista *Files* dell'8 nomina `reconcile.rs`, `record_shape.rs`, `dst_campaign.rs`, `frozen_bytes.rs` → ✅ quattro su quattro | ✅ regge **per i `match` su `RecordKind`**. ⛔ **Non regge come censimento dei siti che fermano il compilatore**: il quinto è `match detail` in `frozen_bytes.rs:450` (`E0004` misurato), e il compito 6 lo nomina mentre l'8 no → R12-6. È la lezione di P-45 in persona, un giro dopo |
| **P-46** — *«il compito 8 rende ROSSO il banco del compito 7, e la cura è nel compito 8»* | la sequenza citata da P-46 è quella dettata dal compito 7 (`awk` sul 7 → `vec![Intent, Invocation, Permission, Intent, Outcome, Outcome,]`), e il Passo 11 dell'8 la sostituisce con la stessa più `RecordKind::Policy` **all'indice 4**, con l'indentazione identica (16/20 spazi) | ✅ regge; il richiamo dettato («THE SEQUENCE WAS SIX … step B's own step reads as three records instead of two») è vero, e l'ordine della decisione 21 non cambia |

## Numeri di compito ricensiti

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| *Files*, `Modify: crates/kernel/tests/serving.rs (**LF, dal compito 7**)` | 7 | 7 — `serving.rs` nasce col **7** (riga 7 della posizione, `kernel::serving`) | ✅ |
| *Read* e *Interfaces*, *«il blocco *Interfaces* del compito 6»*, *«Consumes, dal compito 6»* | 6 | 6 — `RecordKind::Invocation`/`Detail::Invocation` sono del **6** | ✅ (tre occorrenze, tutte giuste) |
| testa, *«Il compito 9 scrive l'`unwrap_or`»* e *«il cablaggio … sono il compito **9** — D25»* | 9 | 9 — il daemon è la riga 9 dopo la divisione di D25 | ✅ |
| Passo 11, *«il banco del compito 7 — P-46»* | 7 | 7 | ✅ |
| Passo 12(a)/(b) e il criterio, *«compito 8»* | 8 | 8 | ✅ (tre occorrenze) |
| Passo 3 e Passo 5, il commento citato *«since milestone 5 task 9»* | Traguardo 5, compito 9 del **SP1** | invariato — è testo esistente citato, e D76 lo lascia dov'è | ✅ |
| criterio di chiusura, *«il chiamante di produzione nasce col compito **9**»* | 9 | 9 | ✅ |

⚠️ **Nessun numero stantio**: i compiti nominati dall'8 sono tutti **prima** o **subito dopo** lo spostamento di
D25, e D25 ha spostato dal 9 in su — l'8 nomina 6, 7, 8 e 9, e il 9 è già il numero **nuovo** (il daemon).

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

```
git -C <repo> rev-parse --short HEAD                                              → 95ecc29
git -C <repo> status --porcelain                                                  → (vuoto)
ls -la <scratchpad>/review/                                                       → skeleton.md, p-titles.md, model11-lock, probe-R11, R11-report.md, probe-R12-killed-fable-0835, …
wc -l -c <scratchpad>/review/skeleton.md <scratchpad>/review/p-titles.md          → 704/93366, 116/15009
wc -l -c <piano>                                                                  → 22334 righe, 1 407 829 byte
grep -c '^### P-' <piano>                                                         → 116
grep -c '^| \*\*D[0-9]' <piano>                                                    → 89
grep -c '^## Compito' <piano>                                                     → 17
grep -n '^## Compito 8:\|^## Compito 9:' <piano>                                  → 9914, 10813
grep -n '^#\|^## \|^### ' skeleton.md                                             → l'indice [A][B][C][D]
sed -n '1,101p' skeleton.md                                                       → vincoli globali, posizione, voci aperte
sed -n '275,411p' skeleton.md                                                     → compiti 6, 7, 8, 9
awk '/^### Compito 8/{f=1} /^### Compito 9/{exit} f' ledger.md                    → due righe: R9b-5 ⬜, R10-12/D75/D76 ✅
awk '/^## Compito 8:/{f=1} /^## Compito 9:/{exit} f' <piano> > probe-R12/c8.md    → 899 righe
grep -n 'Passo' c8.md                                                             → 13 Passi
sed -n '36,165p' / '166,315p' / '315,450p' / '446,650p' / '649,810p' / '808,899p' c8.md → il compito per intero
grep -nE '^\s+#\[n\([0-9]+\)\]' -A 1 crates/kernel/src/record.rs | grep -A 1 -E 'n\([4-7]\)' → #[n(4)] Routing, #[n(5)] Permission
grep -n 'pub enum RecordKind' -A 40 crates/kernel/src/record.rs | grep -E '#\[n\(' → 0..5, l'ultima è Permission
grep -n 'pub enum Detail' -A 24 crates/kernel/src/record.rs | grep -E '#\[n\('     → 0..2, l'ultima è Permission
grep -n 'pub fn detail' -A 3 crates/kernel/src/record.rs                          → Option<&Detail>
grep -rn 'RecordKind::Permission' crates/ --include='*.rs' | grep -v 'src/record.rs' → 11 righe, 6 file
grep -n 'fn transition_record' -A 14 crates/kernel/src/arbiter/mod.rs             → fn(EffectClass,Trust,Vec<u8>,&'static str)->RecordV1 + &'static str -> Vec<u8>
grep -c '^#\[test\]' crates/kernel/tests/arbiter_policy.rs                        → 12   (e grep -c '#\[test\]' → 12: nessuno dentro un commento)
ls crates/kernel/tests/frozen/                                                    → 6 .cbor + record_v1.map
git ls-files --eol <i dieci file del Passo 1>                                     → nove i/lf w/crlf; serving.rs non esiste
grep -n 'set_policy' crates/kernel/src/reconcile.rs                               → riga 54
grep -n 'milestone 5 task 9' -B 4 -A 4 crates/kernel/src/reconcile.rs             → il capoverso vero, spezzato su due righe
grep -n 'pub fn set_policy' -A 30 crates/kernel/src/arbiter/mod.rs                → sei righe di corpo (498–503)
sed -n '1,30p' crates/kernel/tests/arbiter_policy.rs                              → gli use: nessun `use kernel::arbiter;`
grep -n '^#\[test\]' -A 1 crates/kernel/tests/arbiter_policy.rs                   → 12 nomi
sed -n '329,400p' crates/kernel/tests/arbiter_policy.rs                           → le due sonde della transizione
grep -n 'pub enum RecordError' -B 6 -A 20 crates/kernel/src/record.rs             → derive(Debug,Clone,Copy,PartialEq,Eq), variante Malformed
grep -n 'pub enum VramPolicy' -B 8 -A 12 crates/kernel/src/arbiter/policy.rs      → NESSUN derive
grep -n 'pub enum JournalError' -B 4 -A 20 crates/kernel/src/ports/journal.rs     → derive(Debug,Clone,Copy,PartialEq,Eq); OutOfOrder documenta «a note upon a step that has no intent»
grep -n 'derive\|^pub struct\|^pub enum\|^pub trait\|^impl ' crates/kernel/src/arbiter/policy.rs → zero derive nel file
grep -n '^use \|^pub use \|^pub mod ' crates/kernel/src/arbiter/mod.rs            → use crate::record::{EffectClass, Record, RecordV1, Trust}; pub use policy::{…}
grep -n 'pub fn decode' -A 4 crates/kernel/src/record.rs                          → Result<Self, RecordError>
awk '/^## Compito 6:/{f=1} /^## Compito 7:/{exit} f' <piano> > c6.md              → 1210 righe
sed -n '64,190p' c6.md                                                            → il Passo 2 del 6 (la specie Invocation)
sed -n '963,1020p' c6.md                                                          → il Passo 7 del 6 (sonda usa e getta): stessa forma dell'8
sed -n '1076,1150p' c6.md | grep -n 'use '                                        → «kernel::record::{…} in testa al file riceve InvocationDetail»
grep -niE 'polic|compito 8|record|congelat|match|indice' p-titles.md              → P-3, P-4, P-33, P-38, P-45, P-46
awk per P-4/33/44/45/46 > pvoci.md                                                → 164 righe, lette
awk '/^## Compito 7:/…' <piano> | grep -n 'RecordKind::Intent' -B 6 -A 12         → il vec! dei sei in serving.rs, indentazione 16/20
awk '/^## Compito 7:/…' | grep -n 'vec!\[\|entries.len()\|replay()\|kinds'         → un solo blocco di RecordKind
sed -n '370,400p' crates/kernel/tests/frozen_bytes.rs                             → il match kind e l'array dei sei
sed -n '160,175p' / '200,255p' crates/kernel/tests/record_shape.rs                → `let of = |kind| match kind` e l'array
sed -n '430,470p' crates/simulator/tests/dst_campaign.rs                          → i bracci panic! dell'oracolo
grep -n '^use ' crates/kernel/tests/reconciliation.rs                             → use kernel::reconcile::{InDoubt, Resolution, steps_in_doubt};
sed -n '185,205p' crates/kernel/src/reconcile.rs                                  → il braccio RecordKind::Permission => {}
sed -n '435,470p' crates/kernel/tests/frozen_bytes.rs                             → il `match detail` a tre bracci
grep -rn 'match detail\|Detail::Permission(' crates/ --include='*.rs' | grep -v src/record.rs → un solo match su Detail
grep -n 'fn note' -A 30 crates/simulator/src/journal.rs                           → `if !self.has_intent(step) { return Err(JournalError::OutOfOrder) }`
grep -n 'fn a_permission_does_not_put_a_step_in_doubt' -A 40 crates/kernel/tests/reconciliation.rs → intent → note → outcome → note
sed -n '88,130p' crates/kernel/src/reconcile.rs; grep -n 'fn enter\|fn leave' -A 8 → enter/leave/resolution_of
git config --get core.autocrlf                                                    → true;  grep 'text' .gitattributes → «E NIENTE `* text=auto`, MAI»
python counts.py (Python, newline="")                                             → blocco del Passo 3 come stampato: 0 · senza `/// `: 1 · le tre ancore del 5(c): 1,1,1 · le sei righe di set_policy: 1
cp del workspace in probe-R12/ws (Cargo.toml, Cargo.lock, rust-toolchain.toml, crates/) → 1,8 MB
python patch.py    (compito 6 modellato + Passi 2 e 3 dell'8)                     → record.rs ok; **il Sostituisci del Passo 3 → 0 occorrenze**
python patch2.py   (Passi 4, 5, 6, 7, 8 dell'8, alla lettera)                     → sei file toccati
cargo build --locked -p kernel                                                    → error[E0422] PolicyDetail non in scope (arbiter/mod.rs:1140)
(aggiunto PolicyDetail all'use) cargo build --locked -p kernel                     → Finished
cargo build --locked -p kernel --tests                                            → E0004 (match detail), E0433 ×3 (reconcile), E0433 ×4 (arbiter)
cargo build --locked -p kernel --test frozen_bytes                                → E0004 a frozen_bytes.rs:450, «&Detail::Invocation(_) and &Detail::Policy(_) not covered»
python patch3.py   (bracci Detail + use kernel::arbiter + use kernel::reconcile)   → applicati
cargo build --locked -p kernel --tests                                            → E0369 + E0277 su Option<VramPolicy>
cargo build --locked -p kernel --test arbiter_policy                              → i due errori per esteso, arbiter_policy.rs:490
(prima sonda riscritta con .map(|p| p.name())) cargo test -p kernel --test arbiter_policy --test reconciliation --test record_shape → arbiter_policy 16 passed; reconciliation 19 passed/1 FAILED (OutOfOrder); record_shape 12 passed
cargo build --locked --workspace --tests                                          → nessun errore
cargo test --locked -p kernel                                                     → tutti i banchi verdi tranne reconciliation (1 failed)
mutazione Passo 5 (policy_note sempre local) + cargo test --test arbiter_policy   → 13 passed / 3 failed (a Passo 5, quando le quattro sonde non esistono ancora, sarebbe la sola a_transition_names…)
mutazione Passo 7 (return anticipato) + cargo test --test arbiter_policy          → 15 passed / 1 failed: SOLO the_archive_names_the_last_transition_and_not_the_first
mutazione Passo 3 `enter` + cargo test --test reconciliation --test arbiter_policy → arbiter_policy 16/0; reconciliation: left [InDoubt{StepId(1),RunAgain}] / right [… SuspendAndAsk]
mutazione Passo 3 `leave` + cargo test --test reconciliation --test arbiter_policy → arbiter_policy 16/0; reconciliation: left [] / right [InDoubt{StepId(1),SuspendAndAsk}]
(record_shape senza PolicyDetail nell'use) cargo build --test record_shape         → error[E0422] PolicyDetail
sonda usa e getta del Passo 9 eseguita nel modello                                → 82 00 81 86 07 01 00 46 66 72 6f 7a 65 6e 66 66 72 6f 7a 65 6e 82 04 81 81 f5, len 26
git ls-files --eol <stella> <disegno2> <piano>                                    → tutti i/lf w/lf
grep -n -F 'la forma con cui la transizione di policy si rilegge' <stella>        → una riga, la 963
grep -n '^| 56 ' <stella>                                                          → la 941, tabella «Decisioni prese dal coordinatore»
sed -n '882,890p' / '944,952p' / '960,966p' <stella>                              → le due intestazioni: 3 colonne e 2 colonne
python: lines[962].endswith(" |")                                                  → True; CRLF: False
grep -c -F '| 🔶 **nata alla quarta ripresa' <stella>                              → 1   (e col testo intero → 1)
grep -c '^## Registrate, non prese' <stella>                                       → 1
cat <stella> | tr -cd '\r' | wc -c                                                 → 0
awk delle tabelle spezzate su <stella>                                             → niente
awk '/^## Registrate/{f=1} … /^\| ✅/{c++}' <stella>                                → 6 righe già chiuse
grep -n '^### §' <disegno2>; sed -n '247,281p' / '488,520p' / '78,99p' <disegno2>  → §5, §9 riga 9, la testa datata
awk '/^## Compito 14:/…' <piano> > c14.md; sed -n '2060,2200p' c14.md              → il Passo 15, la forma da copiare
grep per compito 9..17 su RecordKind::Policy|Detail::Policy|PolicyDetail|policy_now|PolicyError|RecordV1::policy → solo il 9
awk '/^## Compito 10:/…' > c10.md; grep -noE 'expected_doubt|set_policy|POLICY_FUNCTION|Invoke' → POLICY_FUNCTION ×9, expected_doubt ×0; Files: Create serving_campaign.rs
grep -n 'vec!\[RecordKind::Intent' crates/kernel/tests/arbiter_policy.rs          → 354
grep -n 'run "' scripts/gate.sh; grep -rn 'clippy\|cargo fmt' scripts/*.sh        → né rustfmt né clippy sono nel cancello
head -20 crates/kernel/tests/frozen/record_v1.map; grep -c -F 'the first three on 2026-08-10' → 0
grep -n '^| \*\*D1\*\* |' / D25 / D26 / D27 / D75 / D76 / D77 <piano>             → letti, tutti e sette
grep -n 'E66\|E112\|E50\|E79\|E94' c8.md                                          → E66/E112/E50 col nome del piano
grep -n 'RECALL OF\|RICHIAMO DEL' c8.md                                           → sei richiami, tutti con <data>
grep -noE 'compito [0-9]+|task [0-9]+|milestone [0-9]+' c8.md                     → il censimento della tabella «Numeri di compito»
git -C <repo> status --porcelain (finale)                                          → (vuoto)
```

## Non verificato, e perché

| Che cosa | Perché |
|---|---|
| **Il Passo 9 e il Passo 10 per intero** — la scrittura a mano di `record_v1_policy.cbor`, la sezione nuova di `record_v1.map`, `the_frozen_records()` da sette a otto | il **settimo** record congelato (compito 6) non esiste e va **tipato a mano** dalla sua corsa: modellarlo avrebbe voluto dire scriverlo io. Ho modellato il `match kind` e il `match detail` — che è dove il compilatore parla — e ho verificato la tabella degli offset eseguendo la sonda usa e getta (R12-15). ⚠️ **Resta non provato**: che la mappa nuova ricostruisca il file (lo tiene `the_map_lists_the_bytes_that_are_really_frozen`) e che `git diff --stat -- crates/kernel/tests/frozen/` mostri solo la mappa e il file nuovo |
| **Il Passo 11** — la sostituzione dentro `crates/kernel/tests/serving.rs` | il file non esiste: nasce col compito 7. Ho confrontato il *Trova* col **modello** dettato dal compito 7 (indentazione 16/20, sei varianti, testo identico a quello che P-46 cita) → combacia; ma non l'ho compilato, perché servirebbe tutto il compito 7 |
| **`bash scripts/gate.sh` e `bash scripts/check-docs.sh` del Passo 13** | girano sul repository, e il repository non si tocca. Nel modello ho lanciato l'equivalente utile — `cargo build --locked --workspace --tests` (verde) e `cargo test --locked -p kernel` — e ho verificato che `gate.sh` **non** contiene né `cargo fmt` né `clippy`, quindi la forma non-rustfmt del `vec![` dettato al Passo 5(c) non è un rosso del cancello |
| **`gate-deps.sh` e `gate-attributes.sh`** | il compito non aggiunge dipendenze né attributi: nel modello `Cargo.toml` e `Cargo.lock` non sono stati toccati, e nessun `#[allow]` è dettato |
| **Il compito 6 nella sua interezza** | modellato al **minimo** che serve all'8: la variante `#[n(6)] Invocation` di `RecordKind`, `#[n(3)] Invocation(InvocationDetail)` di `Detail`, `InvocationDetail` coi suoi tre metodi, `RecordV1::invocation`, il braccio di `reconcile.rs`, e i bracci nei tre banchi. I doc lunghi non sono stati copiati: non cambiano la compilazione |
| **La seconda direzione del Passo 5**, *«si porta `local` a `true` dentro `policy_note` … si osserva il rosso su `a_transition_names_the_policy_it_moves_to`»* | eseguita, ma **nel modello completo**, dove le quattro sonde del Passo 7 esistono già: rosse **tre** (13 passed / 3 failed). Al Passo 5, dove il Passo 7 non è ancora scritto, resta la sola sonda che il piano nomina — quindi la frase è giusta **nel suo punto**, e non è un rilievo |

## Stato finale

`git -C /c/Users/zagor/Desktop/harness status --porcelain` → **(vuoto)** · `git ls-files --eol` di
`<piano>`, `<disegno2>` e `<stella>` → **`i/lf w/lf`** per tutti e tre, invariato · nessun file del repository è
stato creato, modificato o cancellato: tutte le prove vivono in
`<scratchpad>/review/probe-R12/` (la copia del workspace in `probe-R12/ws/`) e i `target` di `cargo` in
`C:\Users\zagor\AppData\Local\Temp\probe-R12-target`.
