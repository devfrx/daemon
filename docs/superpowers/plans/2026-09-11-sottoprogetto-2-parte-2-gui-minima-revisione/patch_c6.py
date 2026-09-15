"""patch_c6.py -- task 6 (R2-12..R2-22, R2-6 via D75, D76) plus the recalls in P-33 and P-45. Scoped, asserted, atomic."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
PLAN = r"C:\Users\zagor\Desktop\harness\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def scoped(text, start_marker, end_marker, subs):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    seg = text[lo:hi]
    for old, new, n in subs:
        c = seg.count(old)
        assert c == n, f"expected {n}, found {c}: {old[:80]!r}"
    for old, new, n in subs:
        seg = seg.replace(old, new)
    return text[:lo] + seg + text[hi:]


EOL_LIST_OLD = ("git ls-files --eol crates/kernel/src/record.rs crates/kernel/src/reconcile.rs crates/kernel/src/lib.rs "
                "crates/kernel/tests/frozen_bytes.rs crates/kernel/tests/reconciliation.rs crates/kernel/tests/frozen/record_v1.map "
                "docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md")
EOL_LIST_NEW = ("git ls-files --eol crates/kernel/src/record.rs crates/kernel/src/reconcile.rs crates/kernel/src/lib.rs "
                "crates/kernel/tests/frozen_bytes.rs crates/kernel/tests/reconciliation.rs crates/kernel/tests/record_shape.rs "
                "crates/simulator/tests/dst_campaign.rs crates/kernel/tests/frozen/record_v1.map "
                "docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md")

STEP_8BIS = r"""- [ ] **Passo 8-bis: gli altri due `match` esaustivi — `record_shape.rs` e `dst_campaign.rs` (P-45)**

⛔ **Entrato il 2026-09-15 alla revisione del piano intero (R2-12): P-45 dichiarava questi due siti «corretti nel
compito 6», e il compito non li nominava.** Misurato col `record.rs` del Passo 2 in un banco a parte:
`cargo test --locked -p kernel --test record_shape` → `error[E0004]: RecordKind::Invocation not covered` sulla chiusura
`let of = |kind| match kind`, e `cargo test --locked -p simulator --test dst_campaign --no-run` →
`error[E0004]: &RecordKind::Invocation not covered` sull'oracolo — cioè il passo *example and compile-fail tests* del
cancello **rosso** al Passo 10. ⛔ **Si leggono PRIMA di toccarli, perché i due non vogliono la stessa cosa** — è la
forma che il compito 8 ripete per l'ottava variante.

**(a)** `crates/kernel/tests/record_shape.rs` (**`i/lf w/crlf`**) — **due** metà, e la seconda il compilatore non la
sorveglia. Nel `match` di `let of = |kind| …`, dopo il braccio `RecordKind::Permission`, il braccio nuovo:

```rust
        // ⚠️ THE SEVENTH SPECIES, AND THE GUARD ABOVE PUT IT HERE AGAIN: `RecordKind::Invocation`
        // made this closure `error[E0004]` on the day it arrived (sub-project 2, task 6). Its
        // detail is not optional either, and `InvocationDetail::new` takes a `&'static str` and
        // a `u8` -- read the type, the `u8` is a decision.
        RecordKind::Invocation => RecordV1::invocation(
            EffectClass::Idempotent,
            Trust::Instruction,
            Vec::new(),
            "why this step exists",
            InvocationDetail::new("a function", 0),
        ),
```

e — ⛔ **A MANO, perché l'array NON va rosso** — la variante in coda all'array del `for kind in [ … ]` che segue:

```rust
        RecordKind::Invocation,
```

⚠️ **La testa di quel `for` porta già tre note** — `Verdict`, `Routing` e `Permission` aggiunti a mano, ciascuna con
la frase *«the `match` goes red on a new species, this ARRAY does not»*. Se ne aggiunge una **quarta** in coda alle
tre, senza cancellare le altre:

```rust
    // ⚠️ AND `Invocation` JOINED IT ON <data> FOR THE SAME REASON THE THREE BEFORE IT DID (sub-project 2,
    // task 6): the `match` goes red on a new species, this ARRAY does not.
```

⛔ **E il blocco a coppie sotto resta a TRE:** lo dice il commento accanto —
quella proprietà vive in `frozen_bytes.rs`, e asserirla anche qui sarebbe una seconda casa (§7.4.4). ⚠️ L'`use
kernel::record::{…}` in testa al file riceve `InvocationDetail`.

**(b)** `crates/simulator/tests/dst_campaign.rs` (**`i/lf w/crlf`**) — ⛔ **un `panic!` e NON un braccio vuoto.** È un
**oracolo indipendente** che controlla `reconcile`, e la voce `E50` del piano del Traguardo 6 scrive perché: *«writing
the empty arm `reconcile` writes would make this oracle agree with the implementation BY CONSTRUCTION on a case it has
never seen»*. Dopo il braccio `RecordKind::Permission`:

```rust
            // ⛔ UNREACHABLE IN THIS SCENARIO TOO, AND `panic!` RATHER THAN THE EMPTY ARM FOR THE
            // REASON ITS THREE SIBLINGS GIVE -- read them, the argument is one. Nothing here invokes
            // a function of the registry, so no invocation record can enter this trace
            // (sub-project 2, task 6).
            //
            // ⚠️ THE DAY THE SCENARIO GROWS AN INVOCATION, THE RED IS A DECISION BEING ASKED FOR
            // and not a defect being reported -- same as its siblings.
            RecordKind::Invocation => panic!(
                "step {step} carries an invocation record: this scenario has grown a registry \
                 invocation, and this oracle must decide what one does to a doubt before it can \
                 stay independent"
            ),
```

```bash
cargo test --locked -p kernel --test record_shape 2>&1 | tail -12
cargo build --locked --workspace --tests 2>&1 | tail -20
```

Atteso: `record_shape` verde, e la build dei banchi **senza** errori di `match` non esaustivo. ⛔ **Se resta un
quinto sito** che il censimento non aveva, è una voce d'errata **e** una correzione a P-45.

"""

# ------------------------------------------------------------------ task 6
t = scoped(t, "\n## Compito 6:", "\n## Compito 7:", [
    # Files (R2-12, R2-13)
    ("- Modify: `crates/kernel/tests/frozen_bytes.rs` (**`i/lf w/crlf`**) — `the_frozen_records()` da **sei a sette**, i `..._BYTES` nuovi, e il `match kind` di riga 386 (**P-33**)",
     "- Modify: `crates/kernel/tests/frozen_bytes.rs` (**`i/lf w/crlf`**) — `the_frozen_records()` da **sei a sette**, i `..._BYTES` nuovi, il `match kind` di riga 386 (**P-33**), ⛔ **più l'`use`, l'array a mano del `for kind in […]` e il `match detail`** — entrati il 2026-09-15 alla revisione del piano intero, R2-13\n"
     "- Modify: `crates/kernel/tests/record_shape.rs` (**`i/lf w/crlf`**) — ⛔ **il `match` esaustivo E l'array a mano** (**P-45**; entrato qui il 2026-09-15 alla revisione del piano intero, R2-12: la lista nominava i due siti di P-33 soli)\n"
     "- Modify: `crates/simulator/tests/dst_campaign.rs` (**`i/lf w/crlf`**) — ⛔ **il braccio `panic!` dell'oracolo indipendente** (**P-45**; R2-12)", 1),
    # Interfaces (R2-21)
    ("- Consumes: `kernel::permission::{self, Permission, PermissionError, Operation}`; `kernel::ports::ipc::ClientId`;",
     "- Consumes: `kernel::permission::{self, Permission, PermissionError}` (⚠️ `Operation` lo consuma **solo il banco**, per la tripla di comodo — R2-21); `kernel::ports::ipc::ClientId`;", 1),
    ("- Produces, e i compiti **7**, **9** e **12** li usano con questi nomi esatti:",
     "- Produces, e i compiti **7** e **10** — e il **14**, per nome — li usano con questi nomi esatti (⚠️ qui stava «7, 9 e 12»: il 9 e il 12 non ne nominano nessuno, misurato alla revisione del piano intero, R2-21):", 1),
    # Passo 1 (R2-18, R2-19, R2-12, R2-14, R2-22)
    ("grep -c '^#\\[n(' crates/kernel/src/record.rs\ngrep -n 'RecordKind::Permission' crates/kernel/src/record.rs crates/kernel/src/reconcile.rs crates/kernel/tests/frozen_bytes.rs\n",
     "grep -c '^\\s*#\\[n(' crates/kernel/src/record.rs\ngrep -n -B1 '^    Permission,' crates/kernel/src/record.rs\ngrep -n -B1 'Permission(#\\[n(0)\\] PermissionDetail)' crates/kernel/src/record.rs\ngrep -n 'RecordKind::Permission' crates/kernel/src/record.rs crates/kernel/src/reconcile.rs crates/kernel/tests/frozen_bytes.rs crates/kernel/tests/record_shape.rs crates/simulator/tests/dst_campaign.rs\n", 1),
    ("grep -rn \"RecordKind::\" crates/kernel/src/ --include=*.rs | grep -v 'src/record.rs' | grep -c .\n",
     "grep -rn 'match .*kind\\b' crates/ --include=*.rs\ngrep -rn 'match detail' crates/ --include=*.rs\ngrep -n '^fn record\\|^use kernel::record' crates/kernel/tests/reconciliation.rs\ncargo test --locked -p kernel --test reconciliation 2>&1 | tail -1\n", 1),
    (EOL_LIST_OLD, EOL_LIST_NEW, 2),
    ("Atteso: i due file **non esistono**; `RecordKind` arriva a `Permission` con l'indice **5** e `Detail` a\n"
     "`Permission` con il **2**; in `frozen/` **sei** `.cbor` più la mappa; `the_frozen_records()` rende un array di\n"
     "**sei**; i due `match` esaustivi di **P-33** sono a `reconcile.rs:90` e `frozen_bytes.rs:386` — ⛔ **si ritrovano\n"
     "col `grep` sulla frase e non col numero di riga** (gotcha #70); tutti i file da toccare `i/lf w/crlf`, la mappa\n"
     "compresa.",
     "Atteso: i due file **non esistono**; il `grep -c` degli indici → **29** (⚠️ la forma `'^#\\[n('` senza `\\s*` rendeva\n"
     "**0**, perché gli attributi sono indentati — R2-18); i due `grep -B1` mostrano `#[n(5)]` sopra `Permission,` di\n"
     "`RecordKind` e `#[n(2)]` sopra la variante `Permission` di `Detail`; in `frozen/` **sei** `.cbor` più la mappa;\n"
     "`the_frozen_records()` rende un array di **sei**; ⛔ **i `match` esaustivi su `RecordKind` sono QUATTRO e non i due di\n"
     "P-33** — `reconcile.rs` (`match body.kind()`), `frozen_bytes.rs` (`match kind`), `record_shape.rs` (la chiusura\n"
     "`let of = |kind| match kind`) e `dst_campaign.rs` (il `match kind` dell'oracolo), **P-45** — più il `match detail` di\n"
     "`frozen_bytes.rs`, e ⛔ **si ritrovano col `grep` sulla frase e non col numero di riga** (gotcha #70); in\n"
     "`reconciliation.rs` l'aiutante è `record(species, effect)` e l'`use kernel::record::{…}` **non** porta\n"
     "`InvocationDetail` (R2-14); il banco della riconciliazione → **18 passed** (la baseline del criterio di chiusura,\n"
     "R2-22); gli **otto** file di `crates/` `i/lf w/crlf`, la mappa compresa, e il disegno `i/lf w/lf` (R2-20).", 1),
    # Passo 3 (R2-15)
    ("`cargo test --locked -p kernel --test reconciliation`, si legge **quale** sonda diventa rossa, e si revoca.\n"
     "Se una delle due lascia tutto verde, la sonda corrispondente del Passo 4 **non esiste ancora** o è vacua.",
     "`cargo test --locked -p kernel --test reconciliation`, si legge **quale** sonda diventa rossa — `enter` deve arrossare\n"
     "**entrambe**, `leave` la **seconda** (misurato alla revisione del piano intero, R2-15: con la nota a `Idempotent` e\n"
     "senza la nota dopo l'`outcome`, `enter` lasciava **tutto verde**) — e si revoca. Se una delle due lascia tutto verde,\n"
     "la sonda corrispondente del Passo 4 **non esiste ancora** o è vacua.", 1),
    # Passo 4 (R2-14, R2-15)
    (".intent(step, &closed_record(RecordV1::intent))", ".intent(step, &record(RecordV1::intent, EffectClass::Idempotent))", 2),
    ("        .outcome(step, &closed_record(RecordV1::outcome))\n"
     "        .expect(\"outcome\");\n\n"
     "    assert_eq!(\n"
     "        steps_in_doubt(&journal).expect(\"the projection must answer\"),\n"
     "        Vec::new(),",
     "        .outcome(step, &record(RecordV1::outcome, EffectClass::Idempotent))\n"
     "        .expect(\"outcome\");\n"
     "    // ⛔ AND A NOTE AFTER THE OUTCOME, which is the case that separates \"does not open\" from\n"
     "    // \"does not reopen\": without it an `enter` in the arm is undone by the `leave` of the\n"
     "    // outcome, and this probe stays green under the mutation it exists to catch (measured at\n"
     "    // the plan review, 2026-09-15). The `Permission` pair above does the same, for the same reason.\n"
     "    journal\n"
     "        .note(step, &an_invocation_note())\n"
     "        .expect(\"the invocation note after the outcome\");\n\n"
     "    assert_eq!(\n"
     "        steps_in_doubt(&journal).expect(\"the projection must answer\"),\n"
     "        Vec::new(),", 1),
    ("/// An invocation note of no importance, so the two probes above say what they mean.\n"
     "///\n"
     "/// ⛔ THE LAST ASSERTION OF THE SECOND PROBE IS NOT DECORATION:",
     "/// An invocation note of no importance, so the two probes above say what they mean.\n"
     "///\n"
     "/// ⛔ THE CLASS IS `Unrepeatable` ON PURPOSE, AND IT IS WHAT KEEPS THE SECOND PROBE NON-VACUOUS:\n"
     "/// it DIFFERS from the `Idempotent` every step here declares, so an `enter` in the arm re-enters\n"
     "/// the step with a resolution it did NOT have and the vector moves. With `Idempotent` -- measured\n"
     "/// at the plan review, 2026-09-15 -- the `enter` mutation re-entered the step with the SAME\n"
     "/// `RunAgain` and `before == after` held: the mutation survived. It is the lesson `a_note()` and\n"
     "/// `a_permission()` already write out, in a third dress. ⚠️ In the REAL registry the note carries\n"
     "/// the function's own class (`noted` in `registry.rs`); this bench chooses the class that makes\n"
     "/// the probe bite, exactly as `a_note()` does.\n"
     "///\n"
     "/// ⛔ THE LAST ASSERTION OF THE SECOND PROBE IS NOT DECORATION:", 1),
    ("fn an_invocation_note() -> Vec<u8> {\n    Record::V1(RecordV1::invocation(\n        EffectClass::Idempotent,",
     "fn an_invocation_note() -> Vec<u8> {\n    Record::V1(RecordV1::invocation(\n        EffectClass::Unrepeatable,", 1),
    ("⚠️ **`closed_record` e gli `use` esistono già in quel file** — si riusano, non si riscrivono; il Passo 1 li ha\n"
     "letti. Se il nome fosse diverso, vale ciò che il file ha **oggi**, e la divergenza è una voce d'errata.",
     "⚠️ **L'aiutante del file è `record(species, effect)` e si riusa, non si riscrive** — qui stava «`closed_record`», un nome\n"
     "che il file non ha mai avuto: misurato alla revisione del piano intero (R2-14), tre `E0425` sul testo dettato. ⛔ **E\n"
     "l'`use kernel::record::{…}` in testa al file si allarga con `InvocationDetail`**, che oggi non porta (senza: un\n"
     "`E0433`). Se il file di oggi differisse ancora, vale ciò che il file ha **oggi**, e la divergenza è una voce d'errata.", 1),
    ("Atteso: **tutte verdi**, due in più del Passo 1.",
     "Atteso: **20 passed** — i **18** del Passo 1 più le due sonde nuove (R2-22).", 1),
    # Passo 6 (R2-16)
    ("Atteso: **cinque verdi**.", "Atteso: **sei verdi** (⚠️ qui stava «cinque»: i `#[test]` del file dettato sono sei, contati alla revisione del piano intero, R2-16).", 1),
    # Passo 8 (R2-13)
    ("In `crates/kernel/tests/frozen_bytes.rs`: il tipo di ritorno di `the_frozen_records()` passa da `; 6]` a `; 7]`,\n"
     "la costante `INVOCATION_BYTES` si aggiunge accanto alle altre con\n"
     "`include_bytes!(\"frozen/record_v1_invocation.cbor\")`, e la voce entra in coda all'array:",
     "In `crates/kernel/tests/frozen_bytes.rs`: ⛔ **prima l'`use`** — `InvocationDetail` entra nell'`use kernel::record::{…}`\n"
     "in testa al file, che oggi non lo porta (senza: `error[E0433]`, misurato alla revisione del piano intero, R2-13); poi\n"
     "il tipo di ritorno di `the_frozen_records()` passa da `; 6]` a `; 7]`, la costante `INVOCATION_BYTES` si aggiunge\n"
     "accanto alle altre con `include_bytes!(\"frozen/record_v1_invocation.cbor\")`, e la voce entra in coda all'array:", 1),
    ("e il `match kind` che **P-33** ha censito riceve il settimo nome nel braccio.\n",
     "e il `match kind` che **P-33** ha censito riceve il settimo nome nel braccio. ⛔ **E nello stesso test due posti che\n"
     "P-33 non censì (R2-13), dentro `every_variant_of_the_wire_enums_is_pinned_by_a_frozen_record`:** l'array **a mano**\n"
     "`for kind in [ … ]` sopra quel `match` riceve `RecordKind::Invocation,` in coda — è la metà che il compilatore **non**\n"
     "sorveglia, e senza di essa il settimo pin non viene controllato (limite dichiarato del file); e il `match detail`\n"
     "esaustivo più sotto — `Detail::Verdict(_) => {}` · `Routing` · `Permission` — riceve il braccio\n"
     "`Detail::Invocation(_) => {}`, senza il quale il banco è `error[E0004]`. Con i tre pezzi, misurato: **7 passed**.\n", 1),
    # Passo 8-bis (R2-12), inserted before Passo 9
    ("- [ ] **Passo 9: i due richiami datati sulla §5 del disegno del 2**",
     STEP_8BIS + "- [ ] **Passo 9: i due richiami datati sulla §5 del disegno del 2**", 1),
    # Passo 9 (D75)
    ("RICHIAMO DEL 2026-09-11, dal pre-controllo del compito 6", "RICHIAMO DEL <data>, dal pre-controllo del compito 6 del 2026-09-11", 2),
    # Passo 10 and criteria (R2-12, R2-20)
    ("Atteso: i due file nuovi a `CR=0`; i sette modificati **invariati** in `git ls-files --eol`;",
     "Atteso: i due file nuovi a `CR=0`; i **nove** modificati **invariati** in `git ls-files --eol`;", 1),
    ("- [ ] i fine-riga rimisurati: i due nuovi a zero CR, i sette modificati invariati",
     "- [ ] i fine-riga rimisurati: i due nuovi a zero CR, i nove modificati invariati", 1),
    # criteria (R2-16, R2-22, R2-12, R2-15, R2-17, D75)
    ("- [ ] `cargo test --locked -p kernel --test registry` → **cinque** passati",
     "- [ ] `cargo test --locked -p kernel --test registry` → **sei** passati (R2-16)", 1),
    ("- [ ] `cargo test --locked -p kernel --test reconciliation` → **due in più** del Passo 1\n"
     "- [ ] `cargo test --locked -p kernel --test frozen_bytes` → tutti passati, e `the_frozen_records()` rende **sette**\n",
     "- [ ] `cargo test --locked -p kernel --test reconciliation` → **20 passed**, i 18 del Passo 1 più due (R2-22)\n"
     "- [ ] `cargo test --locked -p kernel --test frozen_bytes` → tutti passati, e `the_frozen_records()` rende **sette**\n"
     "- [ ] ⛔ **i quattro siti di P-45 toccati**: `cargo test --locked -p kernel --test record_shape` verde, `cargo build --locked --workspace --tests` senza `E0004`; `grep -c 'RecordKind::Invocation' crates/kernel/tests/record_shape.rs` → **3** (il commento, il braccio, la voce dell'array) e `grep -c 'RecordKind::Invocation' crates/simulator/tests/dst_campaign.rs` → **1** (R2-12)\n", 1),
    ("- [ ] ⛔ le **due misure del Passo 3** fatte: `enter` al posto del braccio vuoto → rossa la seconda sonda; `leave` → rossa la prima; entrambe revocate, `git diff --stat` pulito",
     "- [ ] ⛔ le **due misure del Passo 3** fatte: `enter` al posto del braccio vuoto → rosse **entrambe** le sonde; `leave` → rossa la **seconda** (R2-15; qui stava «`enter` → la seconda, `leave` → la prima», falso in entrambe le metà sul testo dettato); entrambe revocate, `git diff --stat` pulito", 1),
    ("- [ ] ⛔ **il registro non nomina l'arbitro**: `grep -cE 'Arbiter|VramPolicy|arbiter' crates/kernel/src/registry.rs` → **0** (**D16**)",
     "- [ ] ⛔ **il registro non IMPORTA l'arbitro**: `grep -c 'use crate::arbiter\\|crate::arbiter::' crates/kernel/src/registry.rs` → **0** (**D16**) — provato dove rende 1: `grep -c 'use crate::arbiter' crates/kernel/src/degradation.rs` → **1**. ⚠️ Qui stava un `grep` sulle **parole** `Arbiter|VramPolicy|arbiter`, che sul file dettato rende **2**: il doc del modulo le *nomina* proprio per dire che non le importa (R2-17)", 1),
    ("- [ ] `grep -c 'RICHIAMO DEL 2026-09-11' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **almeno 2**",
     "- [ ] `grep -c 'RICHIAMO DEL <data>, dal pre-controllo del compito 6' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **2** (con la data scritta — D75), e `grep -c '<data>'` sullo stesso file → **0**", 1),
    # D76
    ("milestone-2 design", "sub-project 2 design", 5),
    ("MILESTONE-2 DESIGN", "SUB-PROJECT 2 DESIGN", 1),
])

# ------------------------------------------------------------------ P-33 and P-45 (R2-12)
t = scoped(t, "\n### P-33 ", "\n### P-34 ", [
    ("fatto che non li riguarda.\n",
     "fatto che non li riguarda.\n\n"
     "⛔ **RICHIAMO DEL 2026-09-15: «due e non di più» è FALSO — i `match` esaustivi su `RecordKind` sono QUATTRO.** Il\n"
     "2026-09-14 **P-45** li ha ricensiti col comando — `record_shape.rs` (la chiusura `let of = |kind| match kind`, più\n"
     "l'array a mano) e `dst_campaign.rs` (l'oracolo indipendente, a braccio `panic!`) — e ha dichiarato la correzione\n"
     "*«nel compito 6 e in P-33»*; ⛔ **ma né il compito 6 né questa voce erano stati toccati**, misurato alla revisione del\n"
     "piano intero (R2-12: `grep -n 'record_shape\\|dst_campaign'` sul compito 6 rendeva nulla). Corretti il 2026-09-15: il\n"
     "compito 6 li nomina in *Files* e li tocca al Passo 8-bis. Il capoverso sopra resta com'era, con la sua data. Il comando\n"
     "che non invecchia: `grep -rn 'match .*kind\\b' crates/ --include=*.rs`.\n", 1),
])
t = scoped(t, "\n### P-45 ", "\n### P-46 ", [
    ("e scoprirlo eseguendo costa un compito rifatto su un `cargo test` che non compila.\n",
     "e scoprirlo eseguendo costa un compito rifatto su un `cargo test` che non compila.\n\n"
     "⛔ **RICHIAMO DEL 2026-09-15: il ✅ qui sopra era una PROMESSA, non un fatto.** Alla revisione del piano intero il\n"
     "compito 6 non nominava ancora né `record_shape.rs` né `dst_campaign.rs`, e P-33 diceva ancora *«due e non di più»*\n"
     "(R2-12). Corretti il 2026-09-15 — *Files* e il Passo 8-bis del compito 6, il richiamo in P-33. La riga sopra resta\n"
     "com'era.\n", 1),
])

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: task 6 patched (plus P-33, P-45);", t.count("\n") - raw.count("\n"), "lines added")
