"""_extract_brief_6bis.py -- regenerate the brief of task 6bis of the design-system plan from the plan and the design,
which are the only homes: the brief is copied WORD FOR WORD, by anchors, and nothing is summarised.

Usage: python _extract_brief_6bis.py
Writes task-6bis-brief.md into the git-ignored working folder `.superpowers/sdd/2026-09-23-design-system/`,
creating it -- and its `*` -- on a fresh clone. Reads the plan and the design from the WORKING TREE, and records
`git rev-parse --short HEAD` and whether either differs from HEAD, so a brief extracted before a commit says so.

What goes in -- the "Come si riprende" of the plan, pre-check of task 6bis, 2026-09-27: the head of the plan (goal,
architecture, stack, design, tools), the global constraints, where the plan stands and how a task is executed, the
errata (E59, E60 and E61 are the three items task 6bis cures, E62 and E63 wrote it, E64-E69 are its pre-check's), the
rows P-16 and P-23, the decisions D9 and D25 to D29, the open items the plan knows, task 6bis whole; from the design,
the answers 4 and 20, the visual language, sections (a), (b) and (f) whole, the bar and the band of (d), the rows 22
and 23 of the table of controls, the traps 21 to 28, and the coordinator's decisions 28 and 29. Same shape as
_extract_brief_6.py, with the subsections of the design that task 6bis names.
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
P_LIST = [16, 23]
D_LIST = [9, 25, 26, 27, 28, 29]
ANSWERS = [4, 20]
CONTROLS = [22, 23]
TRAPS = [21, 22, 23, 24, 25, 26, 27, 28]
DECISIONS = [28, 29]

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


def h2_or_h3(line):
    return line.startswith("## ") or line.startswith("### ")


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


def subsection(L, heading, title):
    return (title, block(L, index_of(L, lambda s: s.startswith(heading)), h2_or_h3))


# --- from the plan --------------------------------------------------------------------------------------------
parts = [
    ("La testa del piano — obiettivo, architettura, pila, disegno, strumenti",
     block(PLAN, 0, lambda s: s == "## Vincoli globali")),
    ("Vincoli globali", block(PLAN, index_of(PLAN, lambda s: s == "## Vincoli globali"), h2)),
    ("A che punto è il piano, e come si esegue un compito",
     block(PLAN, index_of(PLAN, lambda s: s.startswith("## ▶️ A che punto è QUESTO PIANO")), h2)),
    ("L'errata — E59, E60 ed E61 sono le tre voci che il compito 6bis cura, E62 ed E63 lo hanno scritto, E64…E69 sono "
     "del suo pre-controllo e sono già applicate al suo testo; E25, E29, E30, E33, E44 ed E46 sono della sonda dei raggi "
     "che il compito stringe; E42 è lo spazio del messaggio, e non della regione",
     block(PLAN, index_of(PLAN, lambda s: s.startswith("## ⚠️ L'errata di questo piano")), h2)),
    section_rows(PLAN, "## Ciò che la scrittura del piano ha trovato", [f"| **P-{n}** |" for n in P_LIST],
                 f"Le voci P che il compito nomina ({', '.join(f'P-{n}' for n in P_LIST)})"),
    section_rows(PLAN, "## Le decisioni prese scrivendo il piano", [f"| **D{n}** |" for n in D_LIST],
                 f"Le decisioni del piano che il compito nomina ({', '.join(f'D{n}' for n in D_LIST)})"),
    ("Le voci aperte che il piano SA, e non chiude",
     block(PLAN, index_of(PLAN, lambda s: s.startswith("## Le voci aperte che questo piano SA")), h2)),
]
task = index_of(PLAN, lambda s: s.startswith("## Compito 6bis:"))
parts.append(("Il compito 6bis, intero", block(PLAN, task, lambda s: s.startswith("## Compito 7:"))))

# --- from the design ------------------------------------------------------------------------------------------
parts.append(section_rows(DESIGN, "## Le risposte del proprietario", [f"| {n} |" for n in ANSWERS],
                          f"Dal disegno: le risposte {', '.join(map(str, ANSWERS))} del proprietario"))
parts.append(subsection(DESIGN, "### Il linguaggio visivo", "Dal disegno: il linguaggio visivo, con la regola dei raggi"))
for heading, name in (("## (a) I token", "la sezione (a), intera"), ("## (b) Il kit", "la sezione (b), intera")):
    parts.append((f"Dal disegno: {name}", block(DESIGN, index_of(DESIGN, lambda s, h=heading: s.startswith(h)), h2)))
parts.append(subsection(DESIGN, "### La barra", "Dal disegno: la (d), la barra"))
parts.append(subsection(DESIGN, "### La fascia", "Dal disegno: la (d), la fascia"))
parts.append(("Dal disegno: la sezione (f), intera",
              block(DESIGN, index_of(DESIGN, lambda s: s.startswith("## (f) Le sonde che diventano test")), h2)))
parts.append(section_rows(DESIGN, "## Il prodotto, e il controllo", [f"| {n} |" for n in CONTROLS],
                          f"Dal disegno: i controlli {', '.join(map(str, CONTROLS))} della tabella del prodotto"))
parts.append(section_rows(DESIGN, "## Le trappole che mordono", [f"| {n} |" for n in TRAPS],
                          f"Dal disegno: le trappole {TRAPS[0]}–{TRAPS[-1]}"))
parts.append(section_rows(DESIGN, "## Decisioni prese dal coordinatore", [f"| {n} |" for n in DECISIONS],
                          f"Dal disegno: le decisioni {' e '.join(map(str, DECISIONS))} del coordinatore"))

brief = [
    "# Il brief del compito 6bis del design system — estratto dal piano e dal disegno, che sono la casa unica",
    "",
    f"> Generato da `_extract_brief_6bis.py` dall'albero di lavoro, a `HEAD` = `{head}`"
    + (" — ⚠️ **il piano o il disegno differiscono da `HEAD`**: il brief va rigenerato dopo il commit." if dirty
       else " — piano e disegno coincidono con `HEAD`.")
    + " Le sezioni sono copiate **parola per parola**, per intestazione o per riga di tabella; nulla è riassunto."
    " Fine-riga: LF.",
    "",
]
for title, body in parts:
    brief += ["", "---", "", f"# ▶ {title}", ""] + body + [""]
out = "\n".join(brief).rstrip("\n") + "\n"
target = ensure_work("task-6bis-brief.md")
with io.open(target + ".tmp", "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(target + ".tmp", target)
print(f"written: {target}")
print(f"brief: {len(out.encode('utf-8'))} bytes, {out.count(chr(10))} lines; HEAD {head}; dirty: {bool(dirty)}")
for title, body in parts:
    print(f"  {len(body):5d} lines  {title}")
