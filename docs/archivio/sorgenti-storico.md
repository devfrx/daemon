# Archivio — le storie dei commenti nei sorgenti

⛔ **Non è una lettura obbligatoria.** Qui stanno, parola per parola, le storie che i commenti dei sorgenti tenevano
delle proprie correzioni — che cosa un commento diceva, chi l'ha trovato, come —: la R5 del
[terzo audit](../audit-2026-09-30.md), col `lean-docs`, e il suo ↪ AUD-545. Nel sorgente resta la riga del richiamo,
in inglese come il codice, con la data, ciò che è vero adesso e il rimando qui. Un file solo per tutti i sorgenti,
scelto dal proprietario il 2026-10-08 invece delle errate dei piani che il gotcha **#121** di
[`HANDOFF.md`](../HANDOFF.md) indicava.

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Ogni blocco è il codice com'era, in inglese — la
§1.0 della spec —, nella sua recinzione e con le righe intere; ciò che è vero oggi lo dice il sorgente vivo.

## Da `crates/kernel/src/reconcile.rs`, il `lean-docs` della R5 — archiviati il 2026-10-08, lotto 6

Il lotto 6 del `lean-docs` della R5 del [terzo audit](../audit-2026-09-30.md), mirato come vuole la P20 del proprietario: le storie che i commenti di `reconcile.rs` tenevano delle proprie correzioni — il ↪ AUD-545 —, dal sorgente a `a27ea6a`, parola per parola. Nel sorgente resta la riga del richiamo: la data, ciò che è vero adesso, dove vive la storia. Ogni taglio approvato dal proprietario.

### Il doc di `steps_in_doubt`, gli scrittori dei due `kind` — la metà falsa e il richiamo che la correggeva, e due storie

Righe 60–85 di `crates/kernel/src/reconcile.rs` a `a27ea6a`. Escono tre richiami — del 2026-09-21 e due del 2026-09-18 — con la frase che l'ultimo correggeva: «writes through `intent` and `outcome`», e «the two `kind`» della sonda. Nel vivo la frase vera, con le parole del richiamo. Dopo «own citation, because the filter drops the matched»:

```rust
/// line when a comment marker opens it. ⛔ RECALL OF 2026-09-21, FINDING m-1 OF THE THIRD REVIEW
/// OF TASK 12 OF THE PART-2 PLAN: it used to be `grep -v '///'`, and these very lines said so --
/// *«cited on a `///` line the command does not count its own citation — written into a `//`
/// comment it would»*. That was a DEFECT REPORT filed as a declared limit, and it sat here while
/// the same blindness was cured elsewhere; the form now covers every comment marker. The trap
/// `289f487` paid for is the same one, and this command was installed once WITHOUT being run:
/// see `E58` and `E134` of the part-2 plan.
/// ⛔ RECALL OF 2026-09-18, FROM THE REVIEW OF TASK 8 OF THE PART-2
/// PLAN: this sentence said "AND THERE ARE TWO OF THEM", which was already false before that
/// task — `permission::grant` has been one of them since 2026-09-01 — and the recall below names
/// it as a third three lines under a paragraph still saying two. The numeral is REMOVED and not
/// realigned (gotcha #31), on the precedent of `AUD-021` and `AUD-061`.
/// `Untrusted::promote` writes through `Journal::note` a record whose `kind` is
/// `RecordKind::Note`; `Arbiter::set_policy`, since milestone 5 task 9, writes through
/// `intent` and `outcome` records whose `kind` matches each.
/// ⛔ RECALL OF 2026-09-18 — IT WRITES THREE NOW, NOT TWO. Between the intent and the outcome it
/// writes, through `Journal::note`, a record whose `kind` is `RecordKind::Policy` — the species
/// `crate::arbiter::policy_now` reads back, and the shape `permission::grant` already had, which
/// writes through `note` a record whose `kind` is `RecordKind::Permission`. The agreement is
/// still held by that writer's own probe, which now asserts THREE kinds in order.
/// Each writer carries its OWN
/// probe that pins the agreement — `the_promotion_writes_through_note_and_the_record_says_note`
/// in `crates/kernel/tests/boundary_promotion.rs`, and
/// `a_policy_transition_writes_its_intent_before_its_outcome` in
/// `crates/kernel/tests/arbiter_policy.rs`, which asserts the two `kind` IN ORDER against the
/// archive.
```

### Il doc di `steps_in_doubt`, il richiamo del 2026-08-21 — la storia di una correzione del commento

Righe 87–91 di `crates/kernel/src/reconcile.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «the two kind IN ORDER against the archive.»:

```rust
/// ⚠️ RECALL OF 2026-08-21 — THIS PARAGRAPH SAID "TODAY THAT IS ONE FUNCTION" AND CARRIED A
/// TRIGGER THAT HAD ALREADY FIRED: "the helper is born with the SECOND writer". That writer
/// landed on 2026-08-20 and NOTHING WENT RED to say so — a deadline written in prose has no
/// mechanism behind it, unlike the `dead_code` deadlines of `E10` and `E67`, which the compiler
/// remembers. REWRITTEN and not annotated, which is finding A-2's rule.
```

### Il doc di `steps_in_doubt`, il richiamo del 2026-09-18 sull'aiutante — la storia di una correzione del commento

Righe 95–98 di `crates/kernel/src/reconcile.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «of its own — the ones named above»:

```rust
/// are the examples, that command is the census. ⛔ RECALL OF 2026-09-18, FROM THE REVIEW OF
/// TASK 8 OF THE PART-2 PLAN: this sentence said "code with two call sites" and "the two
/// probes", and both numerals are REMOVED and not realigned (gotcha #31) — the writers were
/// already more than two when it was written.
```

### Il doc di `steps_in_doubt`, la frase che ha sostituito «not a defect today» — la storia di una correzione del commento

Righe 111–113 di `crates/kernel/src/reconcile.rs` a `a27ea6a`. Nel vivo la riga del richiamo; la data è quella della riscrittura, `git blame`. Dopo «calling outcome() with a record whose kind says»:

```rust
/// `Intent`. Each probe covers its own writer. This sentence used to read "it is not
/// a defect today, because nothing in the kernel writes a record yet"; that reason expired on
/// 2026-08-10, and it is replaced rather than left standing.
```

### Il braccio `Verdict`, la data tolta — la storia di una correzione del commento

Righe 150–155 di `crates/kernel/src/reconcile.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «answers were tried one at a time, each»:

```rust
                // reverted from a byte-exact copy. ⚠️ THE DATE CAME OUT ON 2026-09-01 AND IS NOT
                // REALIGNED: it read "on 2026-09-01" while `git log` dates the commit that wrote
                // this arm 2026-08-31, so it was a session's belief the commits contradict — same
                // cure as `E66`. ⛔ THE TWO ARMS BELOW KEEP THEIR 2026-09-01, and the difference is
                // the point: `git log` dates THEIR commits to that day, so the census had to
                // DISCRIMINATE rather than sweep. Errata `E112`.
```

## Da `crates/kernel/tests/gateway_decisor.rs`, il `lean-docs` della R5 — archiviati il 2026-10-08, lotto 6

Il lotto 6 del `lean-docs` della R5 del [terzo audit](../audit-2026-09-30.md), mirato come vuole la P20 del proprietario: le storie che i commenti di `gateway_decisor.rs` tenevano delle proprie correzioni — il ↪ AUD-545 —, dal sorgente a `a27ea6a`, parola per parola. Nel sorgente resta la riga del richiamo: la data, ciò che è vero adesso, dove vive la storia. Ogni taglio approvato dal proprietario.

### Il doc di `LOCAL_PRICEY`, «how many were WALKED» — la storia di una correzione del commento

Righe 31–35 di `crates/kernel/tests/gateway_decisor.rs` a `a27ea6a`. Nel vivo la riga del richiamo, col vero di oggi preso dal richiamo. Dopo «no assertion can separate them — errata E59.»:

```rust
/// ⚠️ RICHIAMO DEL 2026-09-01: this said "how many were WALKED", unqualified, and that is false
/// of the FIRST pass — which exhausts the chain in the probe below, so its count is three as well.
/// `E100` corrected exactly this claim INSIDE that probe and left this one standing, above it:
/// the remedy closed the occurrence and not the sentence. What separates the first pass is the
/// other probe, where it stops at index 0. Errata `E119`.
```

### La sonda del record risolto, «THE THREE FIELDS» — la storia di una correzione del commento, che AUD-545 nomina

Righe 181–186 di `crates/kernel/tests/gateway_decisor.rs` a `a27ea6a`. Nel vivo la riga del richiamo, col vero di oggi preso dal richiamo. Dopo «and effect were the same shape. Errata E97.»:

```rust
    // ⚠ RICHIAMO DEL 2026-09-01: this said "THE THREE FIELDS" and "`dispatch` writes six fields
    // and only four were read back", and the arithmetic did not close with ITSELF -- six minus
    // four is two, not three. The counts mixed units: `four` counted ASSERTIONS and `six` counted
    // FIELDS. The field that fell outside both was `payload`, and it was still a live mutant
    // hours later. The numerals are GONE rather than realigned; what holds is the rule below --
    // every field this record carries is read back. Errata `E116`.
```

## Da `crates/kernel/src/arbiter/mod.rs`, il `lean-docs` della R5 — archiviati il 2026-10-08, lotto 6

Il lotto 6 del `lean-docs` della R5 del [terzo audit](../audit-2026-09-30.md), mirato come vuole la P20 del proprietario: le storie che i commenti di `arbiter/mod.rs` tenevano delle proprie correzioni — il ↪ AUD-545 —, dal sorgente a `a27ea6a`, parola per parola. Nel sorgente resta la riga del richiamo: la data, ciò che è vero adesso, dove vive la storia. Ogni taglio approvato dal proprietario.

### Il doc del modulo, le famiglie della porta — la metà falsa e il richiamo che la correggeva

Righe 4–15 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Esce la premessa delle sei famiglie con il richiamo datato del 2026-09-17; nel vivo il vero di oggi, con le parole del richiamo, e la riga del richiamo. Dopo «and the distinction is structural rather than tidy.»:

```rust
//! `crate::ports` declares SIX families and §3.1 calls that list EXHAUSTIVE; a seventh
//! would be a decision no ADR has taken. So the arbiter has no real implementation and no
//! fake: there is ONE, and in simulation that one runs. That is what makes the DST
//! campaign a proof about the product instead of about its imitation (ADR-0020).
//! ⚠️ DATED RECALL, 2026-09-17, sub-project 2 task 4: THE FAMILIES ARE SEVEN NOW, AND THE
//! PREMISE ABOVE IS FALSE -- a seventh DID arrive, `custody`, and it was taken WITHOUT an ADR
//! (decision 19 of the GUI north star). ⛔ THE CONCLUSION IS UNTOUCHED AND NEVER RESTED ON
//! THAT PREMISE: what makes the arbiter logic rather than a port is the sentence right after
//! it -- there is ONE implementation, and a port is a seam that wants two. The count is dated
//! here rather than realigned because this paragraph ARGUES about `crate::ports` instead of
//! counting it; the count lives there, and `grep -c '^pub mod ' crates/kernel/src/ports/mod.rs`
//! says it.
```

### Il doc di `Grant`, «THE ONLY FUNCTION IN THIS CRATE THAT BUILDS ONE» — la storia di una correzione del commento

Righe 81–85 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «and since Task 5 there IS an issuer.»:

```rust
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 6 -- "THE ONLY FUNCTION IN THIS CRATE THAT
/// BUILDS ONE" WAS TRUE FOR ONE TASK, and it is REWRITTEN rather than qualified: an
/// exclusivity is read as a GUARANTEE, and a stale guarantee is worse than a stale count
/// (gotcha #31 on an adjective, the species of `E38`). It said `Arbiter::admit` was that
/// function; task 6 gave the queue its own door and `Arbiter::promote` hands out grants too.
```

### Il doc di `Grant`, il paragrafo dichiarato per il Traguardo 6 — la storia di una correzione del commento

Righe 100–106 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo il vero di oggi, con le parole del paragrafo citato, e la riga del richiamo, che tiene l'antecedente di «that paragraph» del capoverso dopo. Dopo «matches! and let … else instead of assert_eq!.»:

```rust
/// ⛔ THE PARAGRAPH THAT WAS DECLARED FOR MILESTONE 6 IS NOW SPENT, AND IT CAME TRUE ON
/// 2026-08-30. It read: "`Process::start` CONSUMES the grant, and `Arbiter::release` consumes
/// it too. Whoever starts a worker therefore has nothing left to release. The natural way back
/// is for `Worker::kill` to HAND THE GRANT BACK -- killing IS the release -- and it is not
/// built now because that caller does not exist yet". ⛔ IT IS BUILT: `Worker::kill` answers a
/// `Killed`, which carries the grant, and the deadline in prose was collected rather than left
/// to rot (gotcha #77 avoided, this time).
```

### Il doc di `Grant`, il commento prima e dopo lo spostamento — la storia di una correzione del commento, che AUD-545 nomina

Righe 113–132 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Escono due richiami, del 2026-08-19 e del 2026-08-30; nel vivo il vero di oggi, con le parole del secondo, e una riga sola per i due richiami. Dopo «rebuild, and only the sweep got it back.»:

```rust
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 4 -- WHAT THIS COMMENT SAID BEFORE THE MOVE,
/// written out because a moved comment that keeps its old tense is the finding A-2 of this
/// project's audit done again. It lived in `ports::process` and said "the arbiter, which
/// arrives in milestone 5" in the FUTURE, and "today the type has no issuer": the FIRST is
/// spent -- the arbiter module is this one. ⚠️ AND THE SECOND IS SPENT TOO SINCE TASK 5,
/// which is why this sentence is rewritten instead of left standing: at task 4 it read
/// "nothing constructs one yet, `admit` arrives at Task 5", and `admit` is now below it in
/// this file. It also recorded a DIVERGENCE -- the field was a private UNIT, `Grant(())`,
/// because the named field the plan dictated then bought nothing that the unit field did not
/// buy for free and cost an `#[allow(dead_code)]`, which this repository treats as a
/// prohibition switched off (gotcha #13). ⛔ THAT PARAGRAPH IS NOT COPIED, because the shape
/// it described no longer exists: the named field is back, dictated again by the milestone 5
/// design, and it is `id` that names the grant inside the books of the arbiter that issued it.
///
/// ⛔ RECALL OF 2026-08-30, MILESTONE 6 TASK 1 -- THE SENTENCE ABOVE ENDED "and it is `id`
/// that lets `Arbiter::release` tell a grant of THIS arbiter from a grant of another one --
/// with the limit of that written beside `ReleaseError`", AND IT IS CORRECTED RATHER THAN
/// LEFT STANDING. It never was `id` that did that, which is exactly what the declared limit
/// it pointed at said: `GrantId` restarts at zero in every arbiter, so `id` alone cannot tell
/// two arbiters' grants apart. The field that does it is `issuer`, born here with `E30`.
```

### Il doc di `Admission`, «the other twenty-six» — la storia di una correzione del commento

Righe 178–182 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «no bulk regeneration disarms (gotcha #42, strong form).»:

```rust
/// ✅ MEASURED, not asserted: with one shortcut added, its own case comes back `error` and
/// EVERY OTHER CASE stays `ok`. ⚠️ RECALL OF 2026-08-28, AUD-045: this said "the other
/// twenty-six", and the count is REMOVED, not realigned -- it was already wrong at `d662644`
/// (28 cases, so 27), and `tests/compile_fail.rs` globs the directory, so any total here ages
/// on its own.
```

### Il doc di `ReleaseError`, la storia di `E30` — la storia di una correzione del commento

Righe 284–312 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Esce il blocco ①–⑤; nel vivo la riga del richiamo, e il vero di oggi che il blocco teneva, con le sue parole: il confronto di `issuer`, le due sonde, il buco a 5_000 e quello della grazia di revoca, riletti il 2026-10-08. Dopo «which is over-admission arriving by the back door.»:

```rust
/// ⛔ RECALL OF 2026-08-30, MILESTONE 6 TASK 1 -- THIS BLOCK CARRIED THE WHOLE HISTORY OF
/// `E30` AND IS REWRITTEN RATHER THAN ANNOTATED, because this commit spent every claim in it
/// and a true sentence appended under false ones leaves them standing (finding A-2). What it
/// argued, and what closed each piece:
/// ① "THREE CAUSES, ONE ANSWER" -- spent. Two of the three answer `Ok` now, and the variant
///    below states the one that is left.
/// ② The measured pairs, which said 5_001 and the grace deadline answer `Err` -- spent:
///    they answer `Ok` today. TWO of those inputs are now pinned by probes instead of by a
///    paragraph, `a_grant_of_this_arbiter_released_after_its_window_is_not_an_error` at
///    5_001 and `a_grant_released_inside_its_window_reports_what_came_back` at 4_999.
///    ⚠️ AND WHAT NOBODY HOLDS IS NARROWER THAN THE BOUNDARY: it is `release` AT
///    5_000. ✅ The boundary ITSELF is held -- measured 2026-08-30, `collect_expired` moved
///    off `<=` kills `a_grant_is_collected_at_the_instant_its_window_closes` and, in
///    `daemon`, `a_permanent_grant_survives_to_the_last_instant_of_the_axis_and_is_swept_at_it`.
///    A release probe at 5_000 would be those two plus the 5_001 one, which is why it was
///    refused rather than forgotten. ⚠️ The revocation grace is the real gap: no bench
///    releases a grant under revocation at all.
/// ③ "NO PROBE PINS THOSE THREE VALUES, WHICH IS A CHOICE RATHER THAN AN OVERSIGHT" -- half
///    spent by ②, and it was right while it stood: a probe written then would have had to
///    be DELETED to take the decision, which is a vote against taking it.
/// ④ "THE DESIGN IS NOT CHANGED TO CLOSE IT, because closing it means giving an `Arbiter` an
///    IDENTITY, and that is a decision for the owner" -- TAKEN, and the identity is
///    `ArbiterId`. With it the declared limit that paragraph guarded is CLOSED: `GrantId`
///    still restarts at zero in every arbiter, but `release` compares `Grant::issuer` BEFORE
///    it reads the books, so two arbiters no longer need disjoint id spaces to be safe.
/// ⑤ "WHAT IS DELIBERATELY NOT DECIDED: the exact type `release` answers with" -- decided,
///    and it is `Released` above.
/// ⛔ THE ARGUMENT IS NOT COPIED DOWN HERE, only its outcome. It lives in the milestone 6
/// design and in the plan's `E30`; what belongs beside a type is what the type means TODAY.
```

### Il doc di `Held`, «TWO FIELDS, AND THE OTHER TWO ARRIVE» — la storia di una correzione del commento

Righe 331–341 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo, col vero di oggi preso dal richiamo. Dopo «arbiter remembers about a grant it has issued.»:

```rust
/// ⚠️ RECALL OF 2026-08-20, MILESTONE 5 TASK 7 -- THIS COMMENT SAID "TWO FIELDS, AND THE OTHER
/// TWO ARRIVE WITH THEIR OWN READERS", AND IT IS REWRITTEN AND NOT ANNOTATED, because a true
/// sentence appended under a false one leaves the false one standing, which is finding A-2 of
/// this project's own audit done again. It carried two dated recalls of its own and both are
/// spent: task 5 held `lane` and `activity` back because neither had a reader, so both would
/// have compiled as `dead_code` warnings and this repository does not switch a warning off with
/// `#[allow]` (gotcha #13); task 6 corrected the PREMISE of the `lane` one -- a WAITING request
/// is a `Waiting`, which carries the whole `ResourceProfile` and therefore its lane, so
/// `promote` never looks at `held` -- and named `ask_back` at task 7 as the first reader of
/// BOTH. ⛔ TASK 7 IS THIS ONE, `ask_back` EXISTS BELOW, AND IT READS BOTH: it chooses its
/// victim by `lane`, and by `activity` it refuses to ask back what is already on its way out.
```

### Il doc di `Held`, «THE TWO GUARDS» — la storia di una correzione del commento

Righe 355–360 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «there, which are deliberately about three different questions.»:

```rust
/// ⚠️ RECALL OF 2026-08-20, SECOND REVIEW OF MILESTONE 5 TASK 7 -- IT SAID "THE TWO GUARDS", AND
/// THE NUMBER IS REWRITTEN RATHER THAN ANNOTATED. It was true until the first wave of corrections
/// moved the guard on the LANE inside the admissibility test, which took the count from two to
/// three; from that moment this sentence and the closure's own "THREE QUESTIONS AND NOT ONE, AND
/// THEY STAY THREE" were two present-tense figures contradicting each other in ONE file, which is
/// gotcha #31 and worse than a figure that is merely missing. Registered as `E78`.
```

### Il doc di `Arbiter`, il `Vec` tolto e tornato — la storia di una correzione del commento

Righe 405–410 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «refused a derived Ord in order to remove.»:

```rust
/// ⚠️ RECALL OF 2026-08-19, OPENED AND CLOSED THE SAME DAY. At task 5 this line said
/// "`BTreeMap` AND `Vec`" while the struct had no `Vec` -- the sentence had been copied from
/// the milestone 5 design, which was thinking of the lane queues -- so it was cut, with a
/// note saying the word would come back with the queues. It has: `queues` holds a
/// `Vec<Waiting>` per lane. The note is rewritten instead of left standing as a promise
/// already kept.
```

### Il doc di `Arbiter::policy`, la previsione sul compito 9 — la storia di una correzione del commento

Righe 467–470 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «name THROUGH the arbiter, FROM OUTSIDE THE CRATE.»:

```rust
    /// ⚠️ RECALL OF 2026-08-20, MILESTONE 5 TASK 9: this doc ended on a PREDICTION about task 9
    /// (gotcha #57, written at task 8 about code that did not exist), and task 9 MEASURED IT
    /// FALSE -- `set_policy` reads `self.policy`, the FIELD. The sentence is REMOVED, not
    /// answered beside itself (gotcha #76), on the precedent of finding `A-7`.
```

### Il doc di `Arbiter::admit`, «which is task 10's» — la storia di una correzione del commento

Righe 587–593 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo il vero di oggi, con le parole del richiamo — rilette il 2026-10-08: nessun ciclo d'orchestrazione chiama `promote` —, e la riga del richiamo. Dopo «ORCHESTRATION decision -- who calls promote, and when.»:

```rust
    /// Closing it here would mean an `admit` that can refuse room that exists. ⚠️ RECALL OF
    /// 2026-08-21 -- THIS SAID "which is task 10's". Task 10 closed on 2026-08-21 as the
    /// composition root: it assembles the graph and starts the executor ONCE, with no loop,
    /// so it builds no orchestration cycle to decide this IN. The decision belongs to whoever
    /// builds the first one -- the cycle that decides when to call `promote` relative to
    /// `admit` -- and none exists yet in this repository. REGISTERED FOR THE OWNER in the
    /// plan's errata, where it sits beside the permanent-quota voice it interacts with.
```

### Il doc di `Arbiter::admit`, «the day task 10 decides the other way» — la storia di una correzione del commento

Righe 604–607 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «paragraph becomes FALSE IN SILENCE with nothing going»:

```rust
    /// red to say so. ⚠️ RECALL OF 2026-08-21 -- THIS SAID "the day task 10 decides the other
    /// way". Task 10 closed on 2026-08-21 as the composition root, and builds no such cycle:
    /// it assembles the graph and starts the executor ONCE, with no loop. None exists yet in
    /// this repository. ⚖️ AND IT IS NOT PINNED, ON PURPOSE AND ON THE MERITS: a probe
```

### Il doc di `Arbiter::admit`, «THE CLOSER IS STILL TASK 10» — la storia di una correzione del commento

Righe 635–640 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo, col vero di oggi preso dal richiamo. Dopo «decision: what changed at task 8 is only»:

```rust
    /// that the cost of leaving it open is now paid in production and not on paper. ⚠️ RECALL
    /// OF 2026-08-21 -- THIS SAID "THE CLOSER IS STILL TASK 10". Task 10 closed on 2026-08-21
    /// as the composition root, and builds no orchestration cycle to be that closer: it
    /// assembles the graph and starts the executor ONCE, with no loop. None exists yet in this
    /// repository. REWRITTEN in place and not annotated below, which is finding A-2's rule.
    /// Registered as `E100`.
```

### Il doc di `Arbiter::promote`, «task 7 or task 10 changes the order» — la storia di una correzione del commento

Righe 754–759 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «call promote relative to admit -- picks a»:

```rust
    /// different order across lanes, this paragraph becomes FALSE IN SILENCE. ⚠️ RECALL OF
    /// 2026-08-21 -- THIS SAID "task 7 or task 10 changes the order across lanes". Task 10 closed
    /// on 2026-08-21 as the composition root, and builds no orchestration cycle: it assembles
    /// the graph and starts the executor ONCE, with no loop. None exists yet in this repository.
    /// Task 7 closed without changing the order either: `ask_back` walks the lanes from the
    /// WORST, `lanes.iter().rev()`. ⚖️ AND A PROBE IS NOT THE REMEDY: pinning the fall-through
```

### Il doc di `Arbiter::ask_back`, «With this one there are FOUR» — la storia di una correzione del commento

Righe 864–867 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «why collect_expired is private rather than a step»:

```rust
    /// somebody remembers to take. ⚠️ RECALL OF 2026-08-28, AUD-018: this said "With this one
    /// there are FOUR" and the count is REMOVED, not realigned -- it was CORRECT here and wrong
    /// in the third house, which is precisely the failure a figure in three places produces. The
    /// probe's doc in `tests/arbiter_admission.rs` carries the reasoning.
```

### Il doc di `Arbiter::ask_back`, la scadenza venuta a termine — la storia di una correzione del commento

Righe 876–890 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Escono due paragrafi, la scadenza e l'ordine di togliere il metodo; nel vivo una riga sola, col vero di oggi preso dai due. Dopo «the pub(crate) puts there instead of in tests/.»:

```rust
    /// ⏳ RECALL OF 2026-08-20 -- A DEADLINE STOOD HERE AND TASK 8 IS WHAT MADE IT COME DUE.
    /// `E67` and `E74` left two `dead_code` warnings standing on purpose -- "fields lane and
    /// grace are never read" and "method ask_back is never used" -- rather than silence them
    /// with an `#[allow]`, which this repository treats as a prohibition switched off, and the
    /// falsifiable half was written right here in the form `E10` used at task 4: AT TASK 8 THOSE
    /// TWO WARNINGS MUST BE GONE, and IF THEY ARE STILL THERE THIS METHOD WAS NOT NEEDED AND IT
    /// IS REMOVED. ✅ IT CAME DUE AND IT WAS MET, measured and not deduced:
    /// `cargo build --locked --workspace` prints ZERO warnings, against the TWO the same command
    /// printed before the task. No `#[allow]`, no invented reader, no `pub` of convenience --
    /// none was needed. Registered as `E91`.
    ///
    /// ⛔ AND THE ORDER TO REMOVE THIS METHOD IS REWRITTEN AWAY RATHER THAN LEFT BESIDE THE
    /// FACT, because it was written in the PRESENT TENSE and this task turned it into the
    /// opposite of true: `ask_back` is what `LocalPolicy` is made of, and whoever read the
    /// paragraph as it stood would have been told to delete it. Registered as `E99`.
```

### Il corpo di `Arbiter::ask_back`, la misura che comprò la chiusura — la storia di una correzione del commento

Righe 899–907 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo; il capoverso dopo, ciò che tiene la chiusura oggi, resta. Dopo «would promise room it then declines to take.»:

```rust
        // ⚠️ A CLOSURE AND NOT A METHOD, AND THE MEASUREMENT THAT BOUGHT IT HAS EXPIRED --
        // rewritten on 2026-08-20 rather than left standing beside the fact that killed it. The
        // reason WAS `dead_code`: `ask_back` had no production caller, so anything it was the
        // only caller of was dead with it, and ✅ as an associated `Held::askable_by`
        // `cargo build --locked --workspace` printed a THIRD warning on top of the two the owner
        // had accepted (`E67`). ⛔ TASK 8 GAVE `ask_back` A PRODUCTION CALLER AND THE PREMISE
        // FELL WITH IT (`E91`). ✅ RE-MEASURED the same day instead of reasoned: with a private
        // `impl Held` helper reachable ONLY from inside this closure, the build prints ZERO
        // warnings -- `admit` reaches `ask_back`, so nothing behind it is dead any more.
```

### La sonda `a_grant_inside_its_grace_keeps_its_reservation`, AUD-013 — la storia di una correzione del commento

Righe 1412–1429 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Escono il richiamo del 2026-08-28 e le due esclusività smentite, con la misura che le smentì; nel vivo il vero di oggi, con le parole del richiamo, e la riga del richiamo. Dopo «still running, and here that sweep is admit's.»:

```rust
    /// ⛔ RECALL OF 2026-08-28, finding AUD-013 -- THIS PARAGRAPH SAID "neither `ask_back` nor
    /// `revoking()` runs a sweep", and the `ask_back` half was FALSE: `self.collect_expired(now)`
    /// is its first statement, which `ask_back_collects_the_expired_before_it_marks` below
    /// already says about this SAME function. `ask_back` still does not stand in for `admit`
    /// here, for a narrower reason than "it never sweeps": in THIS scenario it sweeps at the
    /// instant the resident is admitted, before anything is marked `Revoking` -- the mark is
    /// made by the REST of that same call, after its own leading sweep already ran, so that
    /// sweep has nothing yet to test the grace-arm against.
    ///
    /// ⛔ AND TWO EXCLUSIVITY CLAIMS WERE ALSO FALSE, MEASURED and not assumed -- "would
    /// satisfy every other probe here", and "the only way to catch it ... is `admit`".
    /// Mutating `collect_expired`'s `Revoking` arm to sweep unconditionally fails TWO probes
    /// under `cargo test --locked -p kernel --lib`, not one -- this one AND
    /// `asking_back_twice_does_not_buy_the_room_twice` (11 passed, 2 failed). The second one
    /// dies on `assert_eq!(arbiter.revoking(), 1)`, BEFORE the `admit` further down its body
    /// ever runs, and the sweep that gets it there is the leading one of its own SECOND
    /// `ask_back` -- at `200`, against a grace that runs to `500`. Both removed rather than
    /// replaced: an exclusivity claim is bought by remeasuring it, and this task did not.
```

### La terza asserzione di una sonda di `ask_back`, la frase del vecchio vanto — la storia di una correzione del commento

Righe 1674–1676 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Esce la sola clausola sul paragrafo di prima; la misura resta. Dopo «in neither, so isolating it changes nothing there).»:

```rust
        // ⛔ AND YET IT IS NOT THE TWO ABOVE THAT HOLD THIS PROBE, WHICH IS WHAT THE PARAGRAPH
        // STANDING HERE UNTIL THE THIRD REVIEW CLAIMED. This probe dies under mutations 5b, 5d, 8,
        // 11 and 12, and they do not all kill it through the same assertion. ✅ MEASURED on
```

### La terza asserzione di una sonda di `ask_back`, la causa del vecchio vanto — la storia di una correzione del commento

Righe 1684–1690 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo il vero di oggi, con le parole del paragrafo, e la riga del richiamo. Dopo «is this assertion that kills the probe, alone.»:

```rust
        // ⚠️ THE CAUSE OF THE OLD CLAIM IS WORTH MORE THAN THE CLAIM. The isolation above samples
        // mutations of `ask_back` ONLY -- precisely the class under which this assertion cannot
        // fail -- and never ran the two rows on which it is load-bearing. An exclusivity measured
        // on a partial sample reads as a guarantee. ⛔ THE REMEDY WAS RIGHT AND THE REASON WRITTEN
        // BESIDE IT WAS WRONG, and wrong by UNDERSTATEMENT: keeping the assertion needs no appeal
        // to a day when `ask_back` might grow a road that touches the books, because the sweep it
        // calls first already has one. Registered as `E82`; the counts as `E81` (`E79`, corrected).
```

### La sonda delle corsie, il censimento per sonda — la storia di una correzione del commento

Righe 1740–1744 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «it. So "it goes on into the next»:

```rust
    /// lane" was asked by NOBODY. ⚠️ RECALL OF 2026-08-28, AUD-043: a per-probe breakdown stood
    /// here -- "seven have a single resident, two have two in the SAME lane, and one leaves
    /// through `reclaimable < needed`" -- and it was wrong at `83c7242` too AND did not add up to
    /// the probes it claims to partition. REMOVED, not realigned: what carries this paragraph is
    /// the mutation below, which is a measurement over the WHOLE workspace and needs no census. ✅ MEASURED on 2026-08-20 and not deduced: with `lanes.iter().rev()`
```

### La sonda `ask_back_collects_the_expired_before_it_marks`, «there are now FOUR» — la storia di una correzione del commento

Righe 1843–1845 di `crates/kernel/src/arbiter/mod.rs` a `a27ea6a`. Nel vivo la riga del richiamo. Dopo «-- it is why collect_expired is private. This»:

```rust
    /// is the only probe that exercises this one's line. ⚠️ RECALL OF 2026-08-28, AUD-018: this
    /// said "With `ask_back` there are now FOUR of them" and the count is REMOVED, not realigned,
    /// from all three houses that carried it.
```
