"""_extract_brief_5.py -- regenerate the brief of task 5 of the design-system plan from the plan and the design,
which are the only homes: the brief is copied WORD FOR WORD, by anchors, and nothing is summarised.

Usage: python _extract_brief_5.py
Writes task-5-brief.md into the git-ignored working folder `.superpowers/sdd/2026-09-23-design-system/`,
creating it -- and its `*` -- on a fresh clone. Reads the plan and the design from the WORKING TREE, and records
`git rev-parse --short HEAD` and whether either differs from HEAD, so a brief extracted before a commit says so.

What goes in -- the "Come si riprende" of the plan, pre-check of task 5, 2026-09-26: the head of the plan (goal,
architecture, stack, design, tools), the global constraints, where the plan stands and how a task is executed, the
errata (E34-E37 are task 5's; E2 and E4 are the cures of task 1 that task 5 leans on), the rows P-3, P-8, P-19 and
P-20, the open items the plan knows, task 5 whole; from the design, the answers 5, 16, 17 and 21, "I due temi" of
section (a), the base pieces, `BaseStatus` and the linter rules of section (b), section (e) whole, the rows 11, 12 and
13 of the table of controls, the traps 5 and 10, and the coordinator's decision 21. Same shape as _extract_brief_4.py,
without the probes of the boards, which task 5 does not use.
"""
import io
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
# ⛔ THE REPOSITORY IS FOUND BY GIT, NOT BY COUNTING FOLDERS: the script lives in the tracked
# `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`, so that the dispatch travels with the repository.
REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=HERE, capture_output=True,
                      check=True).stdout.decode("utf-8").strip()
# The brief is a COPY of the plan: it goes to the git-ignored working folder, never next to this script.
WORK = os.path.join(REPO, ".superpowers", "sdd", "2026-09-23-design-system")


def ensure_work(name):
    """The working folder and the `*` that keeps it out of git -- on a fresh clone neither exists -- and the proof
    that the file is ignored: a brief that showed up in `git status` would be a second home of the plan."""
    os.makedirs(WORK, exist_ok=True)
    ignore = os.path.join(REPO, ".superpowers", "sdd", ".gitignore")
    if not os.path.exists(ignore):
        with io.open(ignore, "w", encoding="utf-8", newline="") as f:
            f.write("*\n")
    path = os.path.join(WORK, name)
    if subprocess.run(["git", "check-ignore", "-q", path], cwd=REPO).returncode != 0:
        raise SystemExit(f"{path} is not ignored by git: refusing to write it")
    return path


PLAN_REL = "docs/superpowers/plans/2026-09-23-design-system.md"
DESIGN_REL = "docs/superpowers/specs/2026-09-22-design-system-design.md"
P_LIST = [3, 8, 19, 20]
ANSWERS = [5, 16, 17, 21]
CONTROLS = [11, 12, 13]
TRAPS = [5, 10]
DECISIONS = [21]

sys.stdout.reconfigure(encoding="utf-8")


def git(*args):
    return subprocess.run(["git", *args], cwd=REPO, capture_output=True, check=True).stdout.decode("utf-8")


def lines(rel):
    return io.open(os.path.join(REPO, rel), encoding="utf-8", newline="").read().replace("\r\n", "\n").split("\n")


head = git("rev-parse", "--short", "HEAD").strip()
dirty = git("status", "--porcelain", "--", PLAN_REL, DESIGN_REL).strip()
PLAN = lines(PLAN_REL)
DESIGN = lines(DESIGN_REL)


def index_of(L, pred, start=0):
    for i in range(start, len(L)):
        if pred(L[i]):
            return i
    raise SystemExit(f"anchor not found after line {start + 1}")


def block(L, first, stop):
    """Lines from `first` up to (not including) the first later line where stop(line) is true; trailing blank
    and `---` lines are dropped."""
    end = index_of(L, stop, first + 1)
    out = L[first:end]
    while out and out[-1].strip() in ("", "---"):
        out.pop()
    return out


def h2(line):
    return line.startswith("## ")


def one_row(L, prefix):
    rows = [s for s in L if s.startswith(prefix)]
    assert len(rows) == 1, f"{prefix!r}: {len(rows)} rows"
    return rows[0]


def table_head(L, section):
    i = index_of(L, lambda s: s.startswith(section))
    t = index_of(L, lambda s: s.startswith("| "), i)
    return [L[t], L[t + 1]]


def section_rows(L, section, prefixes, title):
    """The header of the first table of `section`, and the rows that start with each prefix -- searched in THAT
    section only: the design has several tables whose rows open with `| 4 |`."""
    body = block(L, index_of(L, lambda s: s.startswith(section)), h2)
    rows = table_head(body, section)
    rows += [one_row(body, p) for p in prefixes]
    return (title, rows)


# --- from the plan --------------------------------------------------------------------------------------------
parts = [
    ("La testa del piano — obiettivo, architettura, pila, disegno, strumenti",
     block(PLAN, 0, lambda s: s == "## Vincoli globali")),
    ("Vincoli globali", block(PLAN, index_of(PLAN, lambda s: s == "## Vincoli globali"), h2)),
    ("A che punto è il piano, e come si esegue un compito",
     block(PLAN, index_of(PLAN, lambda s: s.startswith("## ▶️ A che punto è QUESTO PIANO")), h2)),
    ("L'errata — E34, E35, E36 ed E37 sono del compito 5, e sono già applicate al suo testo; E2 ed E4 sono le cure "
     "del compito 1 su cui il compito 5 si appoggia",
     block(PLAN, index_of(PLAN, lambda s: s.startswith("## ⚠️ L'errata di questo piano")), h2)),
    section_rows(PLAN, "## Ciò che la scrittura del piano ha trovato", [f"| **P-{n}** |" for n in P_LIST],
                 f"Le voci P che il compito nomina ({', '.join(f'P-{n}' for n in P_LIST)})"),
    ("Le voci aperte che il piano SA, e non chiude",
     block(PLAN, index_of(PLAN, lambda s: s.startswith("## Le voci aperte che questo piano SA")), h2)),
]
task = index_of(PLAN, lambda s: s.startswith("## Compito 5:"))
parts.append(("Il compito 5, intero", block(PLAN, task, lambda s: s.startswith("## Compito 6:"))))

# --- from the design ------------------------------------------------------------------------------------------
parts.append(section_rows(DESIGN, "## Le risposte del proprietario", [f"| {n} |" for n in ANSWERS],
                          f"Dal disegno: le risposte {', '.join(map(str, ANSWERS))} del proprietario"))
parts.append(("Dal disegno: «I due temi», nella sezione (a)",
              block(DESIGN, index_of(DESIGN, lambda s: s == "### I due temi"), lambda s: s.startswith("### ") or h2(s))))
for sub in ("### I pezzi di base — otto", "### `BaseStatus`", "### Le regole, come controlli del linter"):
    parts.append((f"Dal disegno: {sub[4:]}, nella sezione (b)",
                  block(DESIGN, index_of(DESIGN, lambda s, sub=sub: s.startswith(sub)), lambda s: s.startswith("### ") or h2(s))))
parts.append(("Dal disegno: la sezione (e), intera",
              block(DESIGN, index_of(DESIGN, lambda s: s.startswith("## (e) Le voci registrate")), h2)))
parts.append(section_rows(DESIGN, "## Il prodotto, e il controllo", [f"| {n} |" for n in CONTROLS],
                          f"Dal disegno: i controlli {', '.join(map(str, CONTROLS))} della tabella del prodotto"))
parts.append(section_rows(DESIGN, "## Le trappole che mordono", [f"| {n} |" for n in TRAPS],
                          f"Dal disegno: le trappole {', '.join(map(str, TRAPS))}"))
parts.append(section_rows(DESIGN, "## Decisioni prese dal coordinatore", [f"| {n} |" for n in DECISIONS],
                          f"Dal disegno: la decisione {', '.join(map(str, DECISIONS))} del coordinatore"))

brief = [
    "# Il brief del compito 5 del design system — estratto dal piano e dal disegno, che sono la casa unica",
    "",
    f"> Generato da `_extract_brief_5.py` dall'albero di lavoro, a `HEAD` = `{head}`"
    + (" — ⚠️ **il piano o il disegno differiscono da `HEAD`**: il brief va rigenerato dopo il commit." if dirty
       else " — piano e disegno coincidono con `HEAD`.")
    + " Le sezioni sono copiate **parola per parola**, per intestazione o per riga di tabella; nulla è riassunto."
    " Fine-riga: LF.",
    "",
]
for title, body in parts:
    brief += ["", "---", "", f"# ▶ {title}", ""] + body + [""]
out = "\n".join(brief).rstrip("\n") + "\n"
target = ensure_work("task-5-brief.md")
with io.open(target + ".tmp", "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(target + ".tmp", target)
print(f"written: {target}")
print(f"brief: {len(out.encode('utf-8'))} bytes, {out.count(chr(10))} lines; HEAD {head}; dirty: {bool(dirty)}")
for title, body in parts:
    print(f"  {len(body):5d} lines  {title}")
