"""_extract_brief_1.py -- regenerate the brief of task 1 of the design-system plan from the plan and the design,
which are the only homes: the brief is copied WORD FOR WORD, by anchors, and nothing is summarised.

Usage: python _extract_brief_1.py
Writes task-1-brief.md into the git-ignored working folder `.superpowers/sdd/2026-09-23-design-system/`,
creating it -- and its `*` -- on a fresh clone. Reads the plan and the design from the WORKING TREE,
and records `git rev-parse --short HEAD` and whether either differs from HEAD, so a brief extracted before a
commit says so.

What goes in -- the "Come si riprende" of the plan, 2026-09-24: the head of the plan (goal, architecture, stack,
design, tools), the global constraints, where the plan stands and how a task is executed, the errata, the rows
P-1, P-4, P-5, P-12, P-13, the decisions D1 and D2, the open items the plan knows, task 1 whole; and from the
design, section (a) and the rows 1-7 of the table of controls.
"""
import io
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
# ⛔ THE REPOSITORY IS FOUND BY GIT, NOT BY COUNTING FOLDERS: on 2026-09-24 this script moved from the git-ignored
# `.superpowers/sdd/` to the tracked `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`, so that the
# dispatch travels with the repository to the other machine.
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
P_LIST = [1, 4, 5, 12, 13]
D_LIST = [1, 2]
CONTROLS = range(1, 8)

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


# --- from the plan --------------------------------------------------------------------------------------------
parts = [
    ("La testa del piano — obiettivo, architettura, pila, disegno, strumenti",
     block(PLAN, 0, lambda s: s == "## Vincoli globali")),
    ("Vincoli globali", block(PLAN, index_of(PLAN, lambda s: s == "## Vincoli globali"), h2)),
    ("A che punto è il piano, e come si esegue un compito",
     block(PLAN, index_of(PLAN, lambda s: s.startswith("## ▶️ A che punto è QUESTO PIANO")), h2)),
    ("L'errata", block(PLAN, index_of(PLAN, lambda s: s.startswith("## ⚠️ L'errata di questo piano")), h2)),
]

p_rows = table_head(PLAN, "## Ciò che la scrittura del piano ha trovato")
p_rows += [one_row(PLAN, f"| **P-{n}** |") for n in P_LIST]
parts.append((f"Le voci P che il compito nomina ({', '.join(f'P-{n}' for n in P_LIST)})", p_rows))

d_rows = table_head(PLAN, "## Le decisioni prese scrivendo il piano")
d_rows += [one_row(PLAN, f"| **D{n}** |") for n in D_LIST]
parts.append((f"Le decisioni che il compito nomina ({', '.join(f'D{n}' for n in D_LIST)})", d_rows))

parts.append(("Le voci aperte che il piano SA, e non chiude",
              block(PLAN, index_of(PLAN, lambda s: s.startswith("## Le voci aperte che questo piano SA")), h2)))

task = index_of(PLAN, lambda s: s.startswith("## Compito 1:"))
parts.append(("Il compito 1, intero", block(PLAN, task, lambda s: s.startswith("## Compito 2:"))))

# --- from the design ------------------------------------------------------------------------------------------
parts.append(("Dal disegno: la sezione (a), intera",
              block(DESIGN, index_of(DESIGN, lambda s: s.startswith("## (a) I token")), h2)))
# the design has several tables whose rows open with `| 1 |`: the rows are searched in THIS section only
c_section = block(DESIGN, index_of(DESIGN, lambda s: s.startswith("## Il prodotto, e il controllo")), h2)
c_rows = table_head(c_section, "## Il prodotto, e il controllo")
c_rows += [one_row(c_section, f"| {n} |") for n in CONTROLS]
parts.append(("Dal disegno: i controlli 1–7 della tabella del prodotto", c_rows))

brief = [
    "# Il brief del compito 1 del design system — estratto dal piano e dal disegno, che sono la casa unica",
    "",
    f"> Generato da `_extract_brief_1.py` dall'albero di lavoro, a `HEAD` = `{head}`"
    + (" — ⚠️ **il piano o il disegno differiscono da `HEAD`**: il brief va rigenerato dopo il commit." if dirty else
       " — piano e disegno coincidono con `HEAD`.")
    + " Le sezioni sono copiate **parola per parola**, per intestazione o per riga di tabella; nulla è riassunto."
    " Fine-riga: LF.",
    "",
]
for title, body in parts:
    brief += ["", "---", "", f"# ▶ {title}", ""] + body + [""]
out = "\n".join(brief).rstrip("\n") + "\n"
target = ensure_work("task-1-brief.md")
with io.open(target + ".tmp", "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(target + ".tmp", target)
print(f"written: {target}")
print(f"brief: {len(out.encode('utf-8'))} bytes, {out.count(chr(10))} lines; HEAD {head}; dirty: {bool(dirty)}")
for title, body in parts:
    print(f"  {len(body):5d} lines  {title}")
