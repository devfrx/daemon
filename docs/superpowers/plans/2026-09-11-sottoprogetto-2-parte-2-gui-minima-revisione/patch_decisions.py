"""patch_decisions.py -- the two A/B answered by the owner on 2026-09-16, both checked against the five criteria
of `anthropic-skills:decision-principles`:

  * decision 94 (seventeenth closure) -> B: ONE P per reviewed task, P-117..P-133, each pointing at the report and at
    the ledger section and carrying the command that counts its confirmed «fatto» findings. The entries are GENERATED
    from the reports that exist when this script runs (`R*-report.md`), never typed by hand.
  * C13-2, «il resto spento» of §6a -> A: «off» is every control that would talk to the core, until the core has
    answered (task 14, Passo 8, already does it for the one such control of the 2: the policy radio group); the
    display modules show their empty state; the grid stays operable. Task 14 gets Passo 15-bis, which writes the dated
    recall on the §6a row of the design of the 2; the Files row, the `git add` and the closure criterion of task 14
    count it; the open-items row is corrected (it said «no step turns anything off», and Passo 8 of the 14 does).

Every anchor is asserted right before its write (count == 1 inside the segment it belongs to); the four files are
written atomically at the end. LF in, LF out. Usage: python patch_decisions.py [<root>]  -- <root> defaults to the
repository; a copy of the repository layout in the scratchpad lets the script be rehearsed before it touches the repo.
"""
import glob
import io
import os
import re
import sys
import textwrap

sys.stdout.reconfigure(encoding="utf-8")
ROOT = sys.argv[1] if len(sys.argv) > 1 else r"C:\Users\zagor\Desktop\harness"
REV = os.path.join(ROOT, "docs", "superpowers", "plans", "2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione")
PLAN = os.path.join(ROOT, "docs", "superpowers", "plans", "2026-09-11-sottoprogetto-2-parte-2-gui-minima.md")
LEDGER = os.path.join(REV, "ledger.md")
CONSTRAINTS = os.path.join(REV, "constraints.md")
DATE = "2026-09-16"


def read(path):
    raw = io.open(path, encoding="utf-8", newline="").read()
    assert "\r\n" not in raw, f"{path} is LF: something rewrote it"
    return raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


def scoped(text, start_marker, end_marker, fn):
    assert text.count(start_marker) == 1, f"start: {text.count(start_marker)} of {start_marker[:80]!r}"
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    return text[:lo] + fn(text[lo:hi]) + text[hi:]


def append_to_line(text, prefix, tail):
    """Appends `tail` to the END of the one line that starts with `prefix`."""
    lines = text.split("\n")
    hits = [i for i, line in enumerate(lines) if line.startswith(prefix)]
    assert len(hits) == 1, f"{prefix[:80]!r}: {len(hits)} lines"
    lines[hits[0]] = lines[hits[0]] + tail
    return "\n".join(lines)


def append_in_last_cell(text, prefix, tail):
    """Appends `tail` inside the LAST cell of the one table row that starts with `prefix`."""
    lines = text.split("\n")
    hits = [i for i, line in enumerate(lines) if line.startswith(prefix)]
    assert len(hits) == 1, f"{prefix[:80]!r}: {len(hits)} lines"
    assert lines[hits[0]].endswith(" |"), lines[hits[0]][-60:]
    lines[hits[0]] = lines[hits[0]][:-2] + " " + tail + " |"
    return "\n".join(lines)


# ---- the P entries, generated from the reports -----------------------------------------------------------------------
ROW = re.compile(r"^\| (R[0-9]+[ab]?)-[0-9]+ \| (?:Compito )?([0-9]+)\b")
CONFIRMED_FATTO = re.compile(r"\| *CONFERMATO[^|]*\| *fatto *\|")
DEPTH = {3: "R11", 8: "R12", 13: "R6", 15: "R8", 16: "R8", 17: "R8"}  # the in-depth reviews, one reviewer on Opus 5
REPORT_ORDER = {"R1": 1, "R2": 2, "R3": 3, "R4": 4, "R5": 5, "R6": 6, "R7": 7, "R8": 8, "R9a": 9.1, "R9b": 9.2,
                "R10": 10, "R11": 11, "R12": 12, "R13": 13}

reports = {}
for path in sorted(glob.glob(os.path.join(REV, "R*-report.md"))):
    rid = os.path.basename(path).split("-")[0]
    # The reports are INPUTS, read as the reviewers left them: R10 carries CRs inside (measured 2026-09-16), so the
    # line endings are normalised for parsing only, and nothing writes a report back.
    text = io.open(path, encoding="utf-8", newline="").read().replace("\r\n", "\n").replace("\r", "\n")
    m = re.search(r"— (2026-[0-9]{2}-[0-9]{2})\s*$", text.split("\n")[0])
    assert m, f"no date in the title of {path}"
    reports[rid] = (text.split("\n"), m.group(1))
assert "R6" in reports and "R12" in reports and "R11" in reports, sorted(reports)

counts, who = {}, {}
for n in range(1, 18):
    counts[n], who[n] = 0, set()
for rid, (lines, _) in reports.items():
    for line in lines:
        m = ROW.match(line)
        if m and CONFIRMED_FATTO.search(line):
            n = int(m.group(2))
            if 1 <= n <= 17:
                counts[n] += 1
                who[n].add(rid)
# R6 is entirely about task 13 and its cells carry no task number: its rows count for the 13 by prefix.
for line in reports["R6"][0]:
    if re.match(r"^\| R6-[0-9]+ \|", line) and CONFIRMED_FATTO.search(line):
        counts[13] += 1
        who[13].add("R6")
for n, rid in DEPTH.items():
    if rid in reports:
        who[n].add(rid)

REVDIR = "docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione"


def command(n):
    if n == 13:
        return ("cat " + REVDIR + "/R*-report.md | grep -E '^\\| (R[0-9]+[ab]?-[0-9]+ \\| (Compito )?13\\b|R6-[0-9]+ \\|)' "
                "| grep -cE '\\| *CONFERMATO[^|]*\\| *fatto *\\|'")
    return ("cat " + REVDIR + "/R*-report.md | grep -E '^\\| R[0-9]+[ab]?-[0-9]+ \\| (Compito )?" + str(n) + "\\b' "
            "| grep -cE '\\| *CONFERMATO[^|]*\\| *fatto *\\|'")


def p_entry(num, n):
    ids = sorted(who[n], key=lambda r: REPORT_ORDER[r])
    depth = DEPTH.get(n) if DEPTH.get(n) in reports else None
    listed = [r for r in ids if r != depth]
    named = ", ".join(f"**{r}** ({reports[r][1]})" for r in listed) if listed else "nessun revisore di superficie"
    clause = f" — e **in profondità** da **{depth}** il {reports[depth][1]}, un revisore solo su Opus 5" if depth else ""
    r6 = ("; le celle di R6 non portano il numero, perché R6 è tutto sul 13, e il comando le prende per prefisso"
          if n == 13 else "")
    prose = textwrap.fill(
        f"⛔ **Decisione 94 della diciassettesima chiusura, scelta B dal proprietario il {DATE}: una P per compito rivisto, che "
        "rimanda invece di copiare** — i rilievi vivono già in due case committate, e una terza sarebbe il gotcha #68. Il compito "
        f"{n} è stato rivisto da {named}{clause}. Ogni rilievo sta nella sezione «Rilievi» del proprio rapporto, in "
        f"`{REVDIR}/`, e il rimedio applicato nella sezione «Compito {n}» di `ledger.md` "
        "accanto, con l'attrezzo che l'ha scritto. Nessun rilievo è ricopiato qui: li conta il comando, non questa riga —",
        width=120, break_long_words=False, break_on_hyphens=False)
    tail = textwrap.fill(
        f"→ **{counts[n]}** il {DATE}: le righe «CONFERMATO · fatto» il cui primo compito nominato è il {n}; una riga che ne nomina "
        f"più d'uno conta per il primo{r6}.",
        width=120, break_long_words=False, break_on_hyphens=False)
    return (
        f"### P-{num} — Compito {n}, rivisto: i rilievi «fatto» confermati stanno nel rapporto e nel registro, e li conta un comando\n"
        "\n" + prose + "\n"
        "\n"
        "```bash\n"
        f"{command(n)}\n"
        "```\n"
        "\n" + tail + "\n"
    )


P_BLOCK = "\n".join(p_entry(116 + n, n) for n in range(1, 18))

# ---- the plan --------------------------------------------------------------------------------------------------------
raw = read(PLAN)
t = raw

# (1) the P entries, before the decisions table
t = sub(t, "\n## Le decisioni prese da questo piano\n", "\n" + P_BLOCK + "\n## Le decisioni prese da questo piano\n")

# (2) the open-items row: decided A, and the imprecise sentence corrected
t = append_in_last_cell(
    t, "| ⛔ **«il resto spento»** della riga",
    "✅ **DECISA dal proprietario il " + DATE + " — A**, controllata sui cinque criteri di `anthropic-skills:decision-principles` "
    "(verificato nel piano: il Passo 8 del 14 spegne già il gruppo radio della policy, `<fieldset :disabled=\"core.policy === null\">`, "
    "con la sonda *«is off until the core has said which policy is active»* e il commento che cita «the rest off (§6a)»; coerenza: si "
    "spegne il controllo che parlerebbe col core, non la finestra; proporzione: nel 2 la SPA gira contro il core finto nel browser e "
    "«core non in esecuzione» arriva col guscio, P-53): il **Passo 15-bis** del 14 scrive il richiamo nella riga della §6a, e la riga "
    "*Files*, il `git add` e il criterio di chiusura del 14 lo contano. ⚠️ **E la frase «nessun passo del 13 o del 14 spegne nulla» "
    "qui sopra era imprecisa, misurato lo stesso giorno:** «spento» esisteva già per l'unico controllo del 2 che parla col core; resta "
    "vero che nessun passo spegne la griglia, e la decisione dice che non deve")

# (3) task 14: Files, Passo 15-bis, the git add, the closure criterion
PASSO15BIS = r'''- [ ] **Passo 15-bis: il richiamo nella §6a del disegno del 2 — «il resto spento», decisione del proprietario del 2026-09-16 (C13-2)**

⛔ **C13-2, dalla tabella di copertura di R6 (2026-09-16):** la riga *«i quattro stati della connessione»* della §6a del 2 dice,
per *core non in esecuzione*, *«una fascia che lo dice e un pulsante «riprova», il resto spento (ADR-0019)»*, e nessuna riga
del piano diceva che cosa «spento» significhi. Il proprietario ha deciso **A** il 2026-09-16, sui cinque criteri di
`anthropic-skills:decision-principles`: **spento è ogni controllo che parlerebbe col core, finché il core non ha risposto** — nel
2 ce n'è uno, il gruppo radio della policy che il Passo 8 mette in un `<fieldset :disabled="core.policy === null">` con la sonda
*«is off until the core has said which policy is active»* — e i moduli senza dati mostrano lo **stato vuoto** (`status.unknown`,
`permissions.none`, `steps.none`, `chat.noRun`); la griglia resta manovrabile, perché spostare un pannello è presentazione e non
chiede nulla al core, che custodisce il pacchetto senza aprirlo né decidere la mossa (riga 1 delle decisioni della §2 della stella
polare). ⚠️ **Costo dichiarato:** una disposizione cambiata mentre il core non gira non ha chi la riceva — `SaveLayout` senza pari
— e non sopravvive alla GUI; nel 2 il caso non si dà, perché la SPA gira contro il core finto nel browser (**D57**), e col guscio
(**P-53**) la fascia lo dichiara **prima** (ADR-0019). La via **B** — la griglia `inert` fino ad `Accepted`, un passo nuovo nel 13 —
è scartata: terrebbe ferma una cosa che il core non decide, e nel 2 nessuno la userebbe (quinto criterio).

`docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` è **LF**: la forma del Passo 12 del compito 8, un
file solo — l'ancora è la riga intera presa dal file, per sezione e inizio di riga, lo script pretende che sia **una** e appende
il richiamo **nell'ultima cella** della riga, e un `assert` sul numero di righe chiude. ⚠️ **`<data>` si sostituisce con la data
del giorno in cui il compito si esegue, nello script E nei `grep` sotto**, o i `grep` rendono 0.

```bash
python - <<'EOF'
import io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8")
DATE = "<data>"
assert DATE != "<data>", "the date goes in before the recall (R8-23)"
path = "docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md"
b = io.open(path, encoding="utf-8", newline="").read()
assert "\r\n" not in b, f"{path} is LF: something rewrote it"
lines = b.split("\n")


def section(heading):
    """The line indexes under the ONE heading that starts with `heading`, up to the next heading of
    the same or a higher level."""
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


row("### §6a", "| i quattro stati della connessione |",
    "✅ **RICHIAMO DEL " + DATE + ", compito 14 del piano della parte 2 (C13-2, decisione del proprietario del 2026-09-16 — A):** "
    "«il resto spento» è **ogni controllo che parlerebbe col core, finché il core non ha risposto** — nel 2 ce n'è uno, il gruppo "
    "radio della policy di `gui/src/panels/Settings.vue`, `disabled` finché `core.policy` è `null` — e lo **stato vuoto** nei moduli "
    "senza dati; la griglia resta manovrabile, perché spostare un pannello è presentazione e il core custodisce il pacchetto senza "
    "decidere la mossa. ⚠️ **Costo dichiarato:** una disposizione cambiata mentre il core non gira non ha chi la riceva (`SaveLayout` "
    "senza pari) e non sopravvive alla GUI; nel 2 il caso non si dà, perché la SPA gira contro il core finto nel browser, e col guscio "
    "la fascia lo dichiara **prima** (ADR-0019). La via B — la griglia `inert` fino ad `Accepted` — è scartata: terrebbe ferma una "
    "cosa che il core non decide")
out = "\n".join(lines)
assert out.count("\n") == b.count("\n"), "a line was added or lost: the recall goes IN a line"
tmp = path + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(out)
os.replace(tmp, path)
print("ok: one recall in", path)
EOF
grep -c 'RICHIAMO DEL <data>, compito 14 del piano della parte 2 (C13-2' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
tr -cd '\r' < docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md | wc -c
```

Atteso (con la data scritta al posto di `<data>`, **anche nel `grep`**): **1**; niente; **0**.

'''


def task14(seg):
    seg = sub(seg,
              "- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — i richiami datati della §1 e delle registrate",
              "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — il richiamo sulla riga "
              "*«i quattro stati della connessione»* della §6a: «il resto spento», decisione del proprietario del 2026-09-16 (**C13-2**, A) "
              "— Passo 15-bis\n"
              "- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — i richiami datati della §1 e delle registrate")
    seg = sub(seg,
              "- [ ] **Passo 16: il mondo web verde, il cancello, e il commit**\n",
              PASSO15BIS + "- [ ] **Passo 16: il mondo web verde, il cancello, e il commit**\n")
    seg = sub(seg,
              "git add gui docs/superpowers/specs/2026-09-07-direzione-gui-design.md\n",
              "git add gui docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n")
    lines = seg.split("\n")
    hits = [i for i, line in enumerate(lines)
            if line.startswith("- [ ] ⛔ **i richiami sono nella stella polare — tre di riga, cinque di modulo, uno sulle registrate")]
    assert len(hits) == 1, hits
    lines.insert(hits[0] + 1,
                 "- [ ] ⛔ **il richiamo di C13-2 è nella §6a del disegno del 2, e il file resta LF e con le tabelle intere:** i tre "
                 "comandi del Passo 15-bis, **con la data al posto di `<data>`** → **1**, niente, **0**")
    return "\n".join(lines)


t = scoped(t, "## Compito 14: la SPA, i moduli", "## Compito 15: il passo web del cancello", task14)

# ---- the ledger ------------------------------------------------------------------------------------------------------
lraw = read(LEDGER)
l = lraw
# ⛔ THE ORDER IS: `patch_c151617b.py` FIRST, then this one. Wave 17 prepends its own sentence to the status
# paragraph, so this anchor is the line as wave 17 leaves it.
l = append_to_line(
    l, "**Ondata 17 (2026-09-16):**",
    " **Le due A/B del proprietario (" + DATE + "):** la decisione 94 → **B** — P-117…P-133, una per compito rivisto, generate da "
    "`patch_decisions.py` dai rapporti — e «il resto spento» (C13-2) → **A**, il Passo 15-bis del 14. Nel registro non resta nessun ⬜.")
l = sub(l,
        "con l'A/B per il proprietario (consiglio A: lo stato vuoto di oggi) ✅\n",
        "con l'A/B per il proprietario (consiglio A: lo stato vuoto di oggi) ✅ · ✅ **DECISA dal proprietario il " + DATE + " — A** "
        "(`patch_decisions.py`): il Passo 15-bis del 14 col richiamo nella riga «i quattro stati della connessione» della §6a, la riga "
        "*Files*, il `git add` e il criterio del 14; la riga delle voci aperte corretta («il Passo 8 del 14 spegne già il radio della policy»)\n")
l = sub(l,
        "forma di D53; il Passo 3 lo verifica col `grep -c -F` e non lo riscrive, e la riga *Files* perde «il capoverso in testa "
        "(Passo 3)» (`patch_c13b.py`) ✅\n",
        "forma di D53; il Passo 3 lo verifica col `grep -c -F` e non lo riscrive, e la riga *Files* perde «il capoverso in testa "
        "(Passo 3)» (`patch_c13b.py`) ✅\n"
        "- C13-2 (" + DATE + ", decisione del proprietario — A, sui cinque criteri): **Passo 15-bis** — il richiamo `<data>` sulla riga "
        "«i quattro stati della connessione» della §6a del disegno del 2 (script nella forma del Passo 12 dell'8, un file solo), la riga "
        "*Files*, il `git add` del Passo 16 e il criterio di chiusura (`patch_decisions.py`) ✅\n")
l = append_to_line(
    l, "## Voci P nuove (P-117…)",
    " ✅ **DECISA dal proprietario il " + DATE + " — B:** una P per compito rivisto, **P-117…P-133**, generate da `patch_decisions.py` "
    "dai rapporti; ogni P porta il comando che conta i propri rilievi «fatto» confermati (le righe il cui primo compito nominato è "
    "quello; le righe senza un compito in testa — testa del piano, diario, disegni — restano nei rapporti e nella sezione «Testa del "
    "piano» qui sopra), e la sezione «Compito N» di questo registro resta la casa dei rimedi")
open_lines = [line for line in l.splitlines() if " ⬜" in line]
assert len(open_lines) == 1, len(open_lines)  # the status paragraph only, which keeps the history

# ---- the reviewers' constraints: the P count the next reviewer will measure ----------------------------------------
craw = read(CONSTRAINTS)
c = sub(craw,
        "P resta **116**, i compiti **17**,",
        "P **133** dal " + DATE + " (decisione 94 → B: P-117…P-133, una per compito rivisto, che rimandano ai rapporti e a "
        "`ledger.md`; erano **116** a `b3422bf`), i compiti **17**,")

# ---- the guards, counted on the text produced --------------------------------------------------------------------------
assert t.count("\n### P-") == 133, t.count("\n### P-")
assert t.count("- [ ] **Passo 15-bis") == 1
assert t.count("RICHIAMO DEL <data>, compito 14 del piano della parte 2 (C13-2") == 1  # the grep of Passo 15-bis
assert t.count('"✅ **RICHIAMO DEL " + DATE + ", compito 14 del piano della parte 2 (C13-2') == 1  # its script
assert t.count("git add gui docs/superpowers/specs/2026-09-07-direzione-gui-design.md "
               "docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n") == 1  # the 14's commit
assert t.count("DECISA dal proprietario il " + DATE + " — A") == 1
assert l.count("DECISA dal proprietario il " + DATE) == 2
assert c.count("P **133** dal " + DATE) == 1

for path, content in ((PLAN, t), (LEDGER, l), (CONSTRAINTS, c)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: P-117..P-133 generated from", sorted(reports, key=lambda r: REPORT_ORDER[r]), "counts", [counts[n] for n in range(1, 18)],
      "sum", sum(counts.values()), "| Passo 15-bis of the 14, the open-items row, the ledger, the constraints;",
      t.count("\n") - raw.count("\n"), "plan lines added")
