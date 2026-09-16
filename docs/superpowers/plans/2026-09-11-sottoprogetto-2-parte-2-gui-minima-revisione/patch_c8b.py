"""patch_c8b.py -- task 8, the thirteen confirmed findings of the in-depth review R12 (2026-09-16, Opus 5), the half
of R9b-5 that was still open in the ledger (with the exact text R12 proposed), one recall in P-45, plus the ledger.
Every anchor is asserted right before its write (count == 1 inside the task-8 segment, or in the whole plan for the
lines outside it); the two files are written atomically at the end. LF in, LF out.

The two measurements this script writes into the plan were re-run today on the reviewer's compiled model, with the
first probe of Passo 4 already in the R12-1 form: arm `{}` -> reconciliation 20 passed, arbiter_policy 16 passed;
arm `enter` -> both policy probes red (`RunAgain` against `SuspendAndAsk`; a finished step put back in doubt);
arm `leave` -> the second probe alone red (`[]` against the open doubt); arbiter_policy green under both.
"""
import io
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
LEDGER = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione\ledger.md"

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


def scoped(text, start_marker, end_marker, fn):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    return text[:lo] + fn(text[lo:hi]) + text[hi:]


def dated(text):
    """A Python expression (as source) for a recall text that carries ONE `<data>`, spliced on DATE."""
    parts = text.split("<data>")
    assert len(parts) == 2, "one <data> per recall"
    assert '"' not in text and "\\" not in text
    return '"' + parts[0] + '" + DATE + "' + parts[1] + '"'


def task8(seg):
    # ---- Files ----------------------------------------------------------------------------------------------------
    # R12-6 -- frozen_bytes.rs carries the only exhaustive `match` on `Detail` outside record.rs
    seg = sub(seg,
              "- Modify: `crates/kernel/tests/frozen_bytes.rs` (**`i/lf w/crlf`**) — `the_frozen_records()` da **sette a otto**, "
              "`POLICY_BYTES`, e il `match kind`\n",
              "- Modify: `crates/kernel/tests/frozen_bytes.rs` (**`i/lf w/crlf`**) — `the_frozen_records()` da **sette a otto**, "
              "`POLICY_BYTES`, il `match kind` **e il `match detail`** (R12-6, 2026-09-16: l'unico `match` esaustivo su `Detail` "
              "fuori da `record.rs`, `error[E0004]` senza il braccio)\n")
    # R12-10 / R9b-5 -- the north star: two rows in two tables
    seg = sub(seg,
              "- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — il richiamo datato sulla "
              "**decisione 56**\n",
              "- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — il richiamo datato sulla "
              "**decisione 56**, **e il «✅ chiusa» sulla riga delle *Registrate, non prese* «la forma con cui la transizione di "
              "policy si rilegge dal giornale»** (R9b-5, 2026-09-16: due righe in due tabelle diverse, e servono entrambe)\n")
    # R12-13 -- the design of 2: §9 row 9 AND the §5 row «il giornale»
    seg = sub(seg,
              "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — il richiamo datato "
              "sulla riga **9** della §9\n",
              "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — il richiamo datato "
              "sulla riga **9** della §9, **e quello sulla riga «il giornale» della §5** (R12-13, 2026-09-16: *«il passo B di "
              "`set_policy` com'è»* è falso da questo compito, che al passo B scrive tre record)\n")
    # R12-9 -- D1 is already done, and no step of this task names it
    seg = sub(seg,
              "- Modify: **questo piano** — la tabella della posizione e **D1** (**D25**)\n",
              "- Modify: **questo piano** — la riga **8** della tabella della posizione (⚠️ qui stava «e **D1** (**D25**)»: D1 porta "
              "già il richiamo di D25 dal 2026-09-14 e nessun passo di questo compito la nomina — R12-9, 2026-09-16)\n")

    # ---- Passo 3 --------------------------------------------------------------------------------------------------
    # R12-2 -- the two discarded answers, as MEASURED (not as feared)
    seg = sub(seg,
              "                // - `enter` makes the step RE-ENTER the doubt with this record's own class, so a\n"
              "                //   transition whose outcome had already closed it comes back open for ever —\n"
              "                //   `steps_in_doubt` answered `[InDoubt { step: StepId(1), .. }]` on a transition\n"
              "                //   that completed.\n"
              "                // - `leave` closes the doubt BEFORE the outcome is durable, so a crash between\n"
              "                //   this note and the outcome leaves a step that executed nothing looking closed —\n"
              "                //   the silent loss ADR-0007 exists to prevent.\n",
              "                // - `enter` makes the step RE-ENTER the doubt with this record's own class. On a\n"
              "                //   COMPLETE transition that is invisible — the outcome that follows the note closes\n"
              "                //   the step again, and `arbiter_policy` stays green — which is why it is caught\n"
              "                //   where no outcome follows: a step in doubt came back with THIS record's class\n"
              "                //   instead of its intent's — `steps_in_doubt` answered\n"
              "                //   `[InDoubt { step: StepId(1), resolution: RunAgain }]` against\n"
              "                //   `[InDoubt { step: StepId(1), resolution: SuspendAndAsk }]` — and a policy record\n"
              "                //   written AFTER an outcome put a finished step back in doubt.\n"
              "                // - `leave` closes the doubt BEFORE the outcome is durable, so a crash between\n"
              "                //   this note and the outcome leaves a step that executed nothing looking closed —\n"
              "                //   `steps_in_doubt` answered `[]` against the open doubt: the silent loss\n"
              "                //   ADR-0007 exists to prevent.\n")
    seg = sub(seg,
              "                RecordKind::Policy => {}\n```\n\n⛔ **E il capoverso in testa a `steps_in_doubt` diventa falso**",
              "                RecordKind::Policy => {}\n```\n\n"
              "⚠️ **Le due risposte scartate sono RIMISURATE il 2026-09-16 (R12-2), sul modello del compito con la prima sonda del\n"
              "Passo 4 nella forma di R12-1:** `enter` rende rosse **entrambe** le sonde del Passo 4 e `leave` la sola seconda, mentre\n"
              "`arbiter_policy` resta verde sotto entrambe — è la ragione per cui le sonde del Passo 4 esistono, e il commento qui\n"
              "sopra riporta le risposte **lette**, non quelle attese. ⛔ **Il testo che il commento portava prima diceva il FALSO** —\n"
              "*«a transition whose outcome had already closed it comes back open for ever … on a transition that completed»*: sotto\n"
              "`enter` una transizione completa **non** resta in dubbio, perché la nota si posa **prima** dell'esito (Passo 5(a)) e\n"
              "l'esito la richiude. Chi esegue rimisura entrambe, e una risposta diversa è una voce d'errata prima del rimedio.\n\n"
              "⛔ **E il capoverso in testa a `steps_in_doubt` diventa falso**")
    # R12-8 -- the *Trova* that did not exist in the file
    seg = sub(seg,
              "e da oggi ne scrive **tre**. *Trova* quella frase **intera,\n"
              "presa dal file**, e *Sostituisci con* la stessa più il richiamo datato:\n"
              "\n"
              "```rust\n"
              "/// `Arbiter::set_policy`, since milestone 5 task 9, writes through\n"
              "/// `intent` and `outcome` records whose `kind` matches each.\n"
              "/// ⛔ RECALL OF <data> — IT WRITES THREE NOW, NOT TWO. Between the intent and the outcome it\n",
              "e da oggi ne scrive **tre**. ⚠️ **La frase NON sta su righe sue\n"
              "(R12-8, misurato il 2026-09-16):** comincia a metà di una riga che apre con `` /// `RecordKind::Note`; `` e finisce a\n"
              "metà della riga dopo, che prosegue con `Each writer carries its OWN` — un *Trova* che la ricopiasse «intera» col\n"
              "`/// ` d'apertura non troverebbe **nulla**, e un *Sostituisci* sull'ancora che trova cancellerebbe `RecordKind::Note`\n"
              "e la frase sui due scrittori. *Trova* quindi la **seconda** riga intera, presa dal file\n"
              "(`grep -n 'matches each. Each writer' crates/kernel/src/reconcile.rs` → **una** riga):\n"
              "\n"
              "```rust\n"
              "/// `intent` and `outcome` records whose `kind` matches each. Each writer carries its OWN\n"
              "```\n"
              "\n"
              "e *Sostituisci con* la stessa, spezzata attorno al richiamo datato — che così sta su righe proprie e non tocca\n"
              "nessuna delle due frasi vicine:\n"
              "\n"
              "```rust\n"
              "/// `intent` and `outcome` records whose `kind` matches each.\n"
              "/// ⛔ RECALL OF <data> — IT WRITES THREE NOW, NOT TWO. Between the intent and the outcome it\n")
    seg = sub(seg,
              "/// still held by that writer's own probe, which now asserts THREE kinds in order.\n```\n",
              "/// still held by that writer's own probe, which now asserts THREE kinds in order.\n"
              "/// Each writer carries its OWN\n```\n")

    # ---- Passo 4 --------------------------------------------------------------------------------------------------
    # R12-1 -- the first probe cannot write the record alone (`OutOfOrder`); the twin's form instead
    seg = sub(seg,
              "in coda, sulla forma **esatta** delle due coppie\n"
              "che ci sono già per `Note` e per `Verdict` — si **rileggono** prima di scrivere queste:\n",
              "in coda, sulla forma **esatta** delle coppie\n"
              "che ci sono già per `Note`, per `Verdict` **e per `Permission`** — la gemella più vicina (R12-1) — si **rileggono**\n"
              "prima di scrivere queste; l'aiutante `record(species, effect)` in testa al file è loro, e queste lo riusano:\n")
    seg = sub(seg,
              "/// ⛔ THE FIRST DIRECTION: a policy record ALONE does not open a doubt. Without this, an arm that\n"
              "/// called `enter` would be caught by nothing — the transition's own intent already opens the step,\n"
              "/// so on a COMPLETE transition the two mistakes cancel out and the archive reads right by accident.\n"
              "/// That is why this probe writes the record BY ITSELF.\n"
              "#[test]\n"
              "fn a_policy_record_does_not_put_a_step_in_doubt() {\n"
              "    let mut journal = MemoryJournal::new();\n"
              "\n"
              "    journal\n"
              "        .note(\n"
              "            StepId::new(1),\n"
              "            &Record::V1(RecordV1::policy(\n"
              "                EffectClass::Idempotent,\n"
              "                Trust::Instruction,\n"
              "                Vec::new(),\n"
              "                \"local\",\n"
              "                PolicyDetail { local: true },\n"
              "            ))\n"
              "            .encode(),\n"
              "        )\n"
              "        .expect(\"the memory journal accepts\");\n"
              "\n"
              "    assert_eq!(\n"
              "        reconcile::steps_in_doubt(&journal).expect(\"the archive reads back\"),\n"
              "        Vec::new(),\n"
              "        \"a policy record is not an intent: it opens no doubt of its own\"\n"
              "    );\n"
              "}\n",
              "/// ⛔ THE FIRST DIRECTION: a policy record does not OPEN a doubt, and does not REOPEN one. The\n"
              "/// transition's own intent already opens the step, so on a COMPLETE transition an arm that\n"
              "/// called `enter` cancels out against the outcome that follows the note, and the archive reads\n"
              "/// right by accident — measured: `arbiter_policy` stays green under it. ⚠️ AND THE RECORD CANNOT\n"
              "/// BE WRITTEN ALONE: `Journal::note` refuses a step without an intent (`OutOfOrder`), which the\n"
              "/// design says in as many words. So this probe writes the whole transition and then ONE MORE\n"
              "/// policy record AFTER the outcome — the case that separates \"does not open\" from \"does not\n"
              "/// reopen\", the form `a_permission_does_not_put_a_step_in_doubt` already has, and the only one\n"
              "/// in which `enter` gets caught here.\n"
              "#[test]\n"
              "fn a_policy_record_does_not_put_a_step_in_doubt() {\n"
              "    let mut journal = MemoryJournal::new();\n"
              "    let step = StepId::new(1);\n"
              "    let policy = Record::V1(RecordV1::policy(\n"
              "        EffectClass::Idempotent,\n"
              "        Trust::Instruction,\n"
              "        Vec::new(),\n"
              "        \"local\",\n"
              "        PolicyDetail { local: true },\n"
              "    ))\n"
              "    .encode();\n"
              "\n"
              "    journal\n"
              "        .intent(step, &record(RecordV1::intent, EffectClass::Idempotent))\n"
              "        .expect(\"intent\");\n"
              "    journal.note(step, &policy).expect(\"the policy note\");\n"
              "    journal\n"
              "        .outcome(step, &record(RecordV1::outcome, EffectClass::Idempotent))\n"
              "        .expect(\"outcome\");\n"
              "    journal\n"
              "        .note(step, &policy)\n"
              "        .expect(\"a policy note after the outcome\");\n"
              "\n"
              "    assert!(\n"
              "        steps_in_doubt(&journal)\n"
              "            .expect(\"the archive reads back\")\n"
              "            .is_empty(),\n"
              "        \"a policy record put a finished step back in doubt\"\n"
              "    );\n"
              "}\n")
    # R12-4 -- the second probe calls `steps_in_doubt` by name, as its twins do (the file imports the element)
    seg = sub(seg, "reconcile::steps_in_doubt(&journal)", "steps_in_doubt(&journal)", n=2)
    seg = sub(seg,
              "cargo test --locked -p kernel --test reconciliation 2>&1 | tail -12\n```\n\n- [ ] **Passo 5:",
              "cargo test --locked -p kernel --test reconciliation 2>&1 | tail -12\n```\n\n"
              "⚠️ **Misurato il 2026-09-16 sul modello del compito (R12-1, R12-2):** con l'ottavo braccio a `{}` entrambe verdi; a\n"
              "`enter` **entrambe rosse** — la prima perché la nota dopo l'esito riapre il passo, la seconda perché la risoluzione\n"
              "torna `RunAgain` al posto di `SuspendAndAsk`; a `leave` la sola seconda, con `[]` contro il dubbio aperto. ⛔ **La\n"
              "forma precedente della prima sonda — la sola `note`, senza intento — non provava nulla:** `MemoryJournal::note`\n"
              "risponde `OutOfOrder` a un passo senza intento (`if !self.has_intent(step)` in `crates/simulator/src/journal.rs`) e la\n"
              "sonda panicava sull'`expect`, verde su nessuna direzione.\n\n"
              "- [ ] **Passo 5:")

    # ---- Passo 5 --------------------------------------------------------------------------------------------------
    # R12-12 -- «le tre righe» are six: the numeral goes, the anchor stays
    seg = sub(seg,
              "**(a)** Il corpo di `set_policy` — *Trova* le tre righe dal `journal.intent(step, …)` al `)` che chiude\n"
              "`journal.outcome`, **intere, prese dal file**, e *Sostituisci con*:\n",
              "**(a)** Il corpo di `set_policy` — *Trova* le righe dal `journal.intent(step, …)` al `)` che chiude\n"
              "`journal.outcome`, **intere, prese dal file** (⚠️ qui stava «le tre righe»: `journal.outcome(` si apre su più righe e\n"
              "sono di più — R12-12, 2026-09-16; il numerale se ne va e l'ancora resta, Python `count` → **1**), e *Sostituisci con*:\n")
    # R12-4 -- `policy_now` enters the `use kernel::arbiter::{…}` of the bench, and is called by name
    seg = sub(seg,
              "importa `kernel::record::{Record, RecordKind, RecordV1, Trust}`: servono anche **`Detail`**, **`EffectClass`** e\n"
              "**`PolicyDetail`** — qui e al Passo 7. ⚠️ E `crates/kernel/tests/reconciliation.rs` importa già da\n",
              "importa `kernel::record::{Record, RecordKind, RecordV1, Trust}`: servono anche **`Detail`**, **`EffectClass`** e\n"
              "**`PolicyDetail`** — qui e al Passo 7 — e nell'`use kernel::arbiter::{…}` dello stesso banco entra **`policy_now`**,\n"
              "che il Passo 7 chiama **nuda**. ⛔ **R12-4, misurato il 2026-09-16:** le sonde dettavano `arbiter::policy_now(…)` e\n"
              "`reconcile::steps_in_doubt(…)` **per percorso**, e i due banchi importano gli **elementi**, non i moduli — `E0433`\n"
              "sette volte; la forma nuda è quella delle sonde gemelle di `reconciliation.rs`, che `steps_in_doubt` lo importano già,\n"
              "e in `arbiter_policy.rs` `arbiter` è già il nome di una funzione del banco. ⚠️ E `crates/kernel/tests/reconciliation.rs` importa già da\n")

    # ---- Passo 6 --------------------------------------------------------------------------------------------------
    # R12-5 -- the `use` of arbiter/mod.rs, spelled out: `PolicyDetail` was missing (E0422)
    seg = sub(seg,
              "⚠️ **Gli `use` in testa al file crescono** di `crate::record::{Detail, RecordError}` e di `RecordKind` se non c'è\n"
              "già: `cargo build` lo dice.\n",
              "⚠️ **Gli `use` in testa al file crescono:** `use crate::record::{EffectClass, Record, RecordV1, Trust};` diventa\n"
              "`use crate::record::{Detail, EffectClass, PolicyDetail, Record, RecordError, RecordKind, RecordV1, Trust};` —\n"
              "`PolicyDetail` lo costruisce `policy_note` del Passo 5(b), `Detail`, `RecordError` e `RecordKind` li legge `policy_now`.\n"
              "⛔ **R12-5, misurato il 2026-09-16:** l'elenco diceva `{Detail, RecordError}` «e `RecordKind` se non c'è già», e senza\n"
              "`PolicyDetail` è `error[E0422]` alla riga di `policy_note` — la specie di R11-2 (`Millis` nel compito 3). `cargo build`\n"
              "dice il resto.\n")

    # ---- Passo 7 --------------------------------------------------------------------------------------------------
    # R12-3 -- `VramPolicy` has no derive: the first probe goes through `.map(.. name())` like its sisters
    seg = sub(seg,
              "    assert_eq!(\n"
              "        arbiter::policy_now(&journal).expect(\"the archive reads back\"),\n"
              "        None,\n"
              "        \"no transition was written, and the default is not the kernel's to name\"\n"
              "    );\n",
              "    // ⚠️ THROUGH `.map(.. name())` LIKE ITS THREE SISTERS: `VramPolicy` derives neither `PartialEq`\n"
              "    // nor `Debug` — its variants carry the policy objects themselves — and a derive added for a\n"
              "    // bench would be a kernel type widened for a test.\n"
              "    assert_eq!(\n"
              "        policy_now(&journal)\n"
              "            .expect(\"the archive reads back\")\n"
              "            .map(|policy| policy.name()),\n"
              "        None,\n"
              "        \"no transition was written, and the default is not the kernel's to name\"\n"
              "    );\n")
    # R12-4 -- the other three call `policy_now` by name
    seg = sub(seg, "arbiter::policy_now(&journal)", "policy_now(&journal)", n=3)
    seg = sub(seg,
              "cargo test --locked -p kernel --test arbiter_policy 2>&1 | tail -20\n```\n\n"
              "⛔ **E la mutazione della quarta sonda si ESEGUE",
              "cargo test --locked -p kernel --test arbiter_policy 2>&1 | tail -20\n```\n\n"
              "⚠️ **R12-3, misurato il 2026-09-16:** la prima sonda confrontava `Option<VramPolicy>` con `None` — `error[E0369]` ed\n"
              "`error[E0277]`, perché `crates/kernel/src/arbiter/policy.rs` non porta **nessun** `#[derive]`; passa per\n"
              "`.map(|policy| policy.name())` come le tre sorelle, e ⛔ **non si aggiunge un `derive` a `VramPolicy`**: il Passo 2\n"
              "argomenta che non è né serializzabile né confrontabile, e un tipo del kernel non si allarga per un banco.\n\n"
              "⛔ **E la mutazione della quarta sonda si ESEGUE")

    # ---- Passo 8 --------------------------------------------------------------------------------------------------
    # R12-7 -- record_shape.rs imports `PolicyDetail`
    seg = sub(seg,
              "la ragione scritta — quella proprietà vive in `frozen_bytes.rs`, e asserirla anche qui sarebbe una seconda casa\n"
              "(§7.4.4).\n",
              "la ragione scritta — quella proprietà vive in `frozen_bytes.rs`, e asserirla anche qui sarebbe una seconda casa\n"
              "(§7.4.4).\n"
              "⚠️ **E l'`use kernel::record::{…}` in testa a `record_shape.rs` riceve `PolicyDetail`**, come il compito 6 fa per\n"
              "`InvocationDetail` (R12-7, misurato il 2026-09-16: senza, `error[E0422]` sul braccio qui sopra).\n")
    # R12-6 -- the fifth site is KNOWN now
    seg = sub(seg,
              "Atteso: `record_shape` verde, e la build dei banchi **senza** errori di `match` non esaustivo. ⛔ **Se resta un\n"
              "quinto sito** che il censimento non aveva, è una voce d'errata **e** una correzione a P-45.\n",
              "Atteso: `record_shape` verde, e la build dei banchi **senza** errori di `match` non esaustivo — ⛔ **tranne UNO, noto:**\n"
              "il `match detail` di `frozen_bytes.rs`, l'unico `match` esaustivo su `Detail` fuori da `record.rs`, che il Passo 10\n"
              "chiude (R12-6, `error[E0004]` misurato il 2026-09-16: P-45 censiva i `match` su `RecordKind`, e questo è sull'altra\n"
              "enum). ⛔ **Se resta un SESTO sito** che il censimento non aveva, è una voce d'errata **e** una correzione a P-45.\n")

    # ---- Passo 9 --------------------------------------------------------------------------------------------------
    # R12-11 -- the quoted head of the map is split over two lines in the file
    seg = sub(seg,
              "⚠️ **La testa della mappa elenca le date in cui ogni file è nato** — *«the first three on 2026-08-10, the verdict on\n"
              "2026-08-31, …»*: si **aggiunge** la data di oggi, non si riscrive la riga.\n",
              "⚠️ **La testa della mappa elenca le date in cui ogni file è nato**, su una frase spezzata fra due righe del file —\n"
              "`grep -n -F 'the routing and the permission on 2026-09-01' crates/kernel/tests/frozen/record_v1.map` → **una** riga\n"
              "(R12-11: la citazione che stava qui saltava l'a-capo, e un `grep -F` su di essa rendeva 0): si **aggiunge** la data di\n"
              "oggi in coda a quella frase, non si riscrive la riga.\n")

    # ---- Passo 10 -------------------------------------------------------------------------------------------------
    # R12-6 / R12-7 -- the `match detail` arm, and the `use`
    seg = sub(seg,
              "e il `match kind` riceve l'ottavo nome nel braccio, **più** il nome nell'array sopra di esso — ⛔ **sono due metà,\n"
              "e solo la prima va rossa** (è la stessa asimmetria di `record_shape.rs`, e il commento accanto la dichiara).\n",
              "e il `match kind` riceve l'ottavo nome nel braccio, **più** il nome nell'array sopra di esso — ⛔ **sono due metà,\n"
              "e solo la prima va rossa** (è la stessa asimmetria di `record_shape.rs`, e il commento accanto la dichiara).\n"
              "⛔ **E il `match detail` della stessa sonda riceve il suo braccio — R12-6:** sotto `Detail::Invocation(_) => {}`, che il\n"
              "compito 6 vi aggiunge, entra `Detail::Policy(_) => {}`, col commento che dichiara il limite come i fratelli; senza,\n"
              "`error[E0004]: non-exhaustive patterns: '&Detail::Policy(_)' not covered` (misurato il 2026-09-16). ⚠️ **E l'`use\n"
              "kernel::record::{…}` in testa al file riceve `PolicyDetail`** (R12-7), come il 6 fa per `InvocationDetail`.\n")

    # ---- Passo 12: rewritten whole, in the form of Passo 15 of task 14 (R12-10, R9b-5, R12-13) --------------------
    P12 = "- [ ] **Passo 12: i due richiami datati nei disegni**\n"
    P13 = "- [ ] **Passo 13: i fine-riga, il cancello, la posizione, il commit**\n"
    lo = seg.index(P12)
    hi = seg.index(P13, lo)
    old12 = seg[lo:hi]
    recall_a = re.search(r"^✅ \*\*RICHIAMO DEL <data>, dal pre-controllo del compito 8 \(P-44\).*$", old12, re.M)
    recall_b = re.search(r"^✅ \*\*RICHIAMO DEL <data>, compito 8 del piano:\*\*.*$", old12, re.M)
    assert recall_a and recall_b, "the two recall texts of the old Passo 12 must be there to be kept"
    recall_c = ("— ✅ **chiusa il <data>, compito 8 del piano della parte 2**: la forma è una **specie** e non una nota con un "
                "dettaglio, che non è pronunciabile (P-44, D26) — `RecordKind::Policy` all'indice 7, `Detail::Policy` all'indice 4, "
                "`PolicyDetail { local: bool }`, `RecordV1::policy`, scritta con `Journal::note` dentro `Arbiter::set_policy`; la "
                "rilettura è `kernel::arbiter::policy_now`, e i byte in più sono `crates/kernel/tests/frozen/record_v1_policy.cbor`")
    recall_d = ("✅ **RICHIAMO DEL <data>, compito 8 del piano, sulla cella «il giornale» di questa riga:** sul passo B `set_policy` "
                "scrive **tre** record — l'intento, la nota della specie `Policy` col dettaglio strutturato, l'esito — e *«com'è»* "
                "vale per l'ordine, non più per il conteggio; il richiamo nel sorgente è in testa a `steps_in_doubt`, e la sonda che "
                "lo tiene è `a_policy_transition_writes_its_intent_before_its_outcome`")
    new12 = (
        "- [ ] **Passo 12: i richiami datati nei disegni — quattro righe in due file, UNO script**\n"
        "\n"
        "⛔ **RICHIAMO DEL 2026-09-16, dalla revisione in profondità di questo compito (R12-10, R12-13, R9b-5):** questo passo\n"
        "dettava due testi «in coda alla cella, senza toccare il testo che c'è», senza *Trova*, senza script e senza un criterio\n"
        "che li contasse — e «la cella» non è una: la tabella delle decisioni del coordinatore ha **tre** colonne e la §9 del 2\n"
        "ne ha **quattro**. E le righe da toccare sono **quattro**, non due: alla decisione 56 e alla riga 9 della §9 si\n"
        "aggiungono la riga delle *Registrate, non prese* che questo compito **chiude** (R9b-5: il suo chiusore è *«il piano ne fa\n"
        "un compito»*, e il compito è questo — un'altra riga in un'altra tabella della stella) e la riga «il giornale» della §5\n"
        "del 2, che dice *«il passo B di `set_policy` com'è»* mentre da oggi il passo B scrive **tre** record (R12-13). La forma è\n"
        "quella del Passo 15 del compito 14: le ancore si prendono **dal file**, per sezione e inizio di riga, lo script pretende\n"
        "che ciascuna sia **una** e appende il richiamo **nell'ultima cella** della riga — una riga di tabella resta una riga — e\n"
        "un `assert` sul numero di righe chiude. I due disegni sono **LF** (Passo 1): si toccano con Python, mai con `sed -i`.\n"
        "⚠️ **`<data>` si sostituisce con la data del giorno in cui il compito si esegue, nello script E nei `grep` sotto**, o i\n"
        "`grep` rendono 0. ⚠️ **La riga «il giornale» ha tre celle e la frase falsa sta nella seconda**: il richiamo si posa in coda\n"
        "alla riga e **nomina** la cella che corregge, invece di spezzare una cella a metà.\n"
        "\n"
        "```bash\n"
        "python - <<'EOF'\n"
        "import io, os, re, sys\n"
        "sys.stdout.reconfigure(encoding=\"utf-8\")\n"
        "DATE = \"<data>\"\n"
        "\n"
        "\n"
        "def patch(path, edits):\n"
        "    b = io.open(path, encoding=\"utf-8\", newline=\"\").read()\n"
        "    assert \"\\r\\n\" not in b, f\"{path} is LF: something rewrote it\"\n"
        "    lines = b.split(\"\\n\")\n"
        "\n"
        "    def section(heading):\n"
        "        \"\"\"The line indexes under the ONE heading that starts with `heading`, up to the next heading of\n"
        "        the same or a higher level.\"\"\"\n"
        "        starts = [i for i, line in enumerate(lines) if line.startswith(heading)]\n"
        "        assert len(starts) == 1, f\"{heading!r}: {len(starts)} headings\"\n"
        "        level = len(lines[starts[0]].split(\" \")[0])\n"
        "        end = next((i for i in range(starts[0] + 1, len(lines)) if re.match(r\"^#{1,%d} \" % level, lines[i])), len(lines))\n"
        "        return range(starts[0], end)\n"
        "\n"
        "    def row(heading, prefix, recall):\n"
        "        \"\"\"Appends `recall` inside the LAST cell of the one row of `heading` that starts with `prefix`.\"\"\"\n"
        "        hits = [i for i in section(heading) if lines[i].startswith(prefix)]\n"
        "        assert len(hits) == 1, f\"{prefix!r}: {len(hits)} lines match\"\n"
        "        i = hits[0]\n"
        "        assert lines[i].endswith(\" |\"), lines[i][-60:]\n"
        "        lines[i] = lines[i][:-2] + \" \" + recall + \" |\"\n"
        "\n"
        "    for heading, prefix, recall in edits:\n"
        "        row(heading, prefix, recall)\n"
        "    out = \"\\n\".join(lines)\n"
        "    assert out.count(\"\\n\") == b.count(\"\\n\"), \"a line was added or lost: every recall goes IN a line\"\n"
        "    tmp = path + \".tmp\"\n"
        "    io.open(tmp, \"w\", encoding=\"utf-8\", newline=\"\").write(out)\n"
        "    os.replace(tmp, path)\n"
        "    print(\"ok:\", len(edits), \"recalls in\", path)\n"
        "\n"
        "\n"
        "patch(\"docs/superpowers/specs/2026-09-07-direzione-gui-design.md\", [\n"
        "    (\"## Decisioni prese dal coordinatore\", \"| 56 |\",\n"
        "     " + dated(recall_a.group(0)) + "),\n"
        "    (\"## Registrate, non prese\",\n"
        "     \"| 🔶 **nata alla quarta ripresa, 2026-09-08** — la forma con cui la transizione di policy si rilegge\",\n"
        "     " + dated(recall_c) + "),\n"
        "])\n"
        "patch(\"docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\", [\n"
        "    (\"### §9\", \"| 9 |\",\n"
        "     " + dated(recall_b.group(0)) + "),\n"
        "    (\"### §5\", \"| il giornale |\",\n"
        "     " + dated(recall_d) + "),\n"
        "])\n"
        "EOF\n"
        "grep -c 'RICHIAMO DEL <data>, dal pre-controllo del compito 8' docs/superpowers/specs/2026-09-07-direzione-gui-design.md\n"
        "grep -c 'chiusa il <data>, compito 8 del piano della parte 2' docs/superpowers/specs/2026-09-07-direzione-gui-design.md\n"
        "grep -c 'RICHIAMO DEL <data>, compito 8 del piano' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n"
        "for f in docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md; do tr -cd '\\r' < \"$f\" | wc -c; awk 'prev ~ /^\\|/ && $0 == \"\" {getline nxt; if (nxt ~ /^\\|/) print NR} {prev=$0}' \"$f\"; done\n"
        "git ls-files --eol docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n"
        "```\n"
        "\n"
        "Atteso (con la data scritta al posto di `<data>`, **anche nei `grep`**): **1**, **1**, **2**; poi `0` e niente per ciascuno\n"
        "dei due file; e `i/lf w/lf` per entrambi. ⛔ **La riga delle *Registrate* riceve il «✅ chiusa» nella cella «Chiusore\n"
        "proposto», l'ultima delle due**, come le righe già chiuse che le stanno accanto — misurato il 2026-09-16: l'intestazione\n"
        "è `| Voce | Chiusore proposto |`, e la riga è una (`grep -c -F 'la forma con cui la transizione di policy si rilegge'` → 1).\n"
        "\n"
    )
    seg = seg[:lo] + new12 + seg[hi:]

    # ---- Passo 13: the closing criterion counts the recalls and the fifth site -------------------------------------
    seg = sub(seg,
              "grep -rn 'policy_now' crates/ --include='*.rs'\n```\n\nAtteso: **otto** `.cbor`;",
              "grep -rn 'policy_now' crates/ --include='*.rs'\n"
              "grep -c 'RICHIAMO DEL <data>, dal pre-controllo del compito 8' docs/superpowers/specs/2026-09-07-direzione-gui-design.md\n"
              "grep -c 'chiusa il <data>, compito 8 del piano della parte 2' docs/superpowers/specs/2026-09-07-direzione-gui-design.md\n"
              "grep -c 'RICHIAMO DEL <data>, compito 8 del piano' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n"
              "grep -rn 'match detail' crates/ --include='*.rs' | wc -l\n"
              "```\n\nAtteso: **otto** `.cbor`;")
    seg = sub(seg,
              "`policy_now` ha **il suo banco** come chiamante e **nessun altro** — il chiamante di produzione nasce col compito\n"
              "**9**, ed è per questo che i due compiti sono due.\n",
              "`policy_now` ha **il suo banco** come chiamante e **nessun altro** — il chiamante di produzione nasce col compito\n"
              "**9**, ed è per questo che i due compiti sono due; i richiami nei disegni rendono **1**, **1** e **2** — con la data\n"
              "vera al posto di `<data>`, anche nei `grep` — e `match detail` è **una** riga, che ora porta il braccio\n"
              "`Detail::Policy(_)` (R12-6).\n")

    # ---- guards, computed on the segment ----------------------------------------------------------------------------
    assert "reconcile::steps_in_doubt(&journal)" not in seg
    assert "arbiter::policy_now(&journal)" not in seg
    assert seg.count("policy_now(&journal)") == 4
    assert seg.count("steps_in_doubt(&journal)") == 3
    assert seg.count("Detail::Policy(_) => {}") == 1
    assert seg.count("**D1** (**D25**)") == 1  # the quoted old text, and nothing else
    assert "the first three on 2026-08-10" not in seg
    assert seg.count("R12-") >= 20
    return seg


t = scoped(t, "\n## Compito 8:", "\n## Compito 9:", task8)

# R12-6, outside task 8: P-45's census was of the `match` on `RecordKind`, not of every site that stops the compiler
t = sub(t,
        "riallineare (gotcha #68). Il comando resta, il numerale no.\n\n### P-46",
        "riallineare (gotcha #68). Il comando resta, il numerale no.\n"
        "\n"
        "⛔ **RICHIAMO DEL 2026-09-16, dalla revisione in profondità del compito 8 (R12-6): i «quattro siti» sono i `match` su\n"
        "`RecordKind`, e NON sono tutti i siti che fermano il compilatore.** `frozen_bytes.rs` porta anche l'unico `match` esaustivo su\n"
        "**`Detail`** fuori da `record.rs` (`grep -rn 'match detail' crates/ --include='*.rs'` → una riga): il compito 6 lo nominava\n"
        "(R2-13) e il compito 8 no — `error[E0004]` misurato sul modello del compito. Corretto nella lista *Files*, nel Passo 8 e\n"
        "nel Passo 10 del compito 8. 📌 **La lezione di questa voce, un giro dopo:** un censimento si rifà col comando **per ogni\n"
        "enum che cresce**, non per la sola che si stava guardando.\n"
        "\n"
        "### P-46")

# ---- the ledger ----------------------------------------------------------------------------------------------------
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw
l = sub(l,
        "Mancano: compito 13 (R6), compiti 15–17 (R8), compito 8 (R3) — perimetri da rivedere ancora; il compito 3 è rivisto da "
        "R11 il 2026-09-16.",
        "Mancano: compito 13 (R6), compiti 15–17 (R8) — perimetri da rivedere ancora; il compito 3 è rivisto da R11 e l'8 da R12, "
        "entrambi il 2026-09-16.")
l = sub(l,
        "resta ⬜ la sola metà dell'8 di R9b-5, con la revisione in profondità dell'8.\n",
        "resta ⬜ la sola metà dell'8 di R9b-5, con la revisione in profondità dell'8. **Ondata 15 (2026-09-16):** la revisione in "
        "profondità dell'8 è FATTA (R12, un revisore solo su Opus 5: ~297k token, 93 chiamate, 98 comandi, ~29 minuti) e i suoi "
        "tredici rilievi confermati sono applicati (✅) con `patch_c8b.py`, insieme alla metà dell'8 di R9b-5 col testo che R12 "
        "propone; il rapporto è `R12-report.md` accanto; le due misure che il piano ora porta (R12-1, R12-2) sono state rifatte dal "
        "coordinatore sul modello compilato di R12 con la sonda nella forma nuova. Nel registro non resta nessun ⬜.\n")
l = sub(l,
        "### Compito 8 — NON RIVISTO IN PROFONDITÀ (R3 caduto): resta da fare\n"
        "- R9b-5, la metà dell'8: il Passo 12(a) scrive «✅ **chiusa il <data>, compito 8 del piano della parte 2**» in coda alla "
        "cella «Chiusore» della riga *«la forma con cui la transizione di policy si rilegge»* delle registrate della stella (la "
        "forma del Passo 15 del 14: ancora presa dal file) — con la revisione in profondità dell'8 ⬜\n",
        "### Compito 8 — RIVISTO IN PROFONDITÀ da R12 il 2026-09-16 (Opus 5; R3 era caduto dopo il 7): tredici rilievi confermati, "
        "tutti applicati, e la metà di R9b-5\n"
        "- R9b-5, la metà dell'8: il Passo 12 riscritto intero nella forma del Passo 15 del 14 — UN solo script Python per i due "
        "disegni, `section`/`row`, ancore dal file, `assert` sul numero di righe — col sotto-passo che scrive «✅ **chiusa il "
        "<data>, compito 8 del piano della parte 2**» in coda alla cella «Chiusore proposto» (l'ultima delle due colonne di quella "
        "tabella) della riga *«la forma con cui la transizione di policy si rilegge»*; Files, e il criterio `grep -c` → 1 ✅\n"
        "- R12-1 Passo 4: la prima sonda nella forma della gemella `a_permission_does_not_put_a_step_in_doubt` — intento, nota "
        "`Policy`, esito, **una seconda nota dopo l'esito** — perché `MemoryJournal::note` rifiuta un passo senza intento "
        "(`OutOfOrder`, la sonda panicava sull'`expect`) ✅ · R12-2 Passo 3: il commento dettato riporta le risposte **misurate** "
        "(`enter`: `RunAgain` contro `SuspendAndAsk`, e il passo finito riaperto dalla nota dopo l'esito; `leave`: `[]` contro il "
        "dubbio aperto; `arbiter_policy` verde sotto entrambe), rimisurate dal coordinatore sul modello con la sonda di R12-1 ✅ · "
        "R12-3 Passo 7: la prima sonda passa per `.map(|policy| policy.name())` come le tre sorelle (`VramPolicy` senza `derive`, "
        "`E0369`/`E0277`), e nessun `derive` si aggiunge ✅ · R12-4 Passi 4, 5 e 7: `steps_in_doubt` e `policy_now` chiamate "
        "**nude** come le gemelle, `policy_now` entra nell'`use kernel::arbiter::{…}` del banco (i banchi importano elementi, non "
        "moduli: `E0433` ×7) ✅ · R12-5 Passo 6: l'`use crate::record::{…}` di `arbiter/mod.rs` dettato per esteso con "
        "`PolicyDetail` (`E0422`) ✅ · R12-6 Files, Passo 8 e Passo 10: il `match detail` di `frozen_bytes.rs` — il quinto sito, "
        "su `Detail` — riceve `Detail::Policy(_) => {}` (`E0004`); il richiamo in P-45 ✅ · R12-7 Passi 8(a) e 10: l'`use "
        "kernel::record::{…}` di `record_shape.rs` e di `frozen_bytes.rs` riceve `PolicyDetail` ✅ · R12-8 Passo 3: il *Trova* è "
        "la seconda riga intera del capoverso (`matches each. Each writer carries its OWN`), il *Sostituisci* la spezza attorno "
        "al richiamo — il blocco «intero» col `/// ` d'apertura non esisteva nel file ✅ · R12-9 Files: via «e **D1** (**D25**)», "
        "resta la riga 8 della posizione ✅ · R12-10 Passo 12: riscritto nella forma del Passo 15 del 14 (sopra), coi tre `grep -c` "
        "nel criterio di chiusura ✅ · R12-11 Passo 9: la citazione della mappa spezzata su due righe → il `grep -F` sulla frase che "
        "sta su una riga ✅ · R12-12 Passo 5(a): via il numerale «tre righe», resta l'ancora ✅ · R12-13 Files e Passo 12: il "
        "richiamo sulla riga «il giornale» della §5 del 2 (*«il passo B di `set_policy` com'è»*) ✅ · R12-14…R12-21 NON RIPRODOTTO: "
        "nulla da fare, e il coordinatore non le ricerca · Le P-117… per questi rilievi restano da scrivere · Attrezzo: "
        "`patch_c8b.py`\n")
l = sub(l,
        "e «milestone 5 task 9» è il Traguardo 5 del SP1 ✅ (la revisione in profondità resta da fare)",
        "e «milestone 5 task 9» è il Traguardo 5 del SP1 ✅ (verificate da R12-20: reggono)")
open_lines = [line for line in l.splitlines() if " ⬜" in line]
assert len(open_lines) == 1, len(open_lines)  # the status paragraph only, which keeps the history; the row of task 8 is closed

for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 8 patched (R12-1..13, R9b-5), P-45 recalled, and the ledger;", t.count("\n") - raw.count("\n"), "plan lines added")
