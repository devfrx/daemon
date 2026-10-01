"""_extract_brief_2.py -- build the brief of task 2 from the plan and the design, word for word.

Usage, from the repository root: PYTHONIOENCODING=utf-8 python <this file>

Writes .superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/task-2-brief.md (git-ignored) and prints its
path. Every piece is cut between two marker lines, each of which must start EXACTLY ONE line of its file; a table
can be filtered to its header, its separator and the rows whose first cell is named. Refuses, writing nothing, if a
marker is absent or not unique. Holds no backslash, like extract.py. The model is _extract_brief_1.py.
"""
import io
import os
import subprocess
import sys

NL = chr(10)
CR = chr(13)
PLAN = "docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md"
DESIGN = "docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md"
OUT_DIR = ".superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti"
OUT = OUT_DIR + "/task-2-brief.md"


def read(path):
    return io.open(path, encoding="utf-8", newline="").read().replace(CR + NL, NL).split(NL)


def find(lines, marker, path):
    hits = [k for k, ln in enumerate(lines) if ln.startswith(marker)]
    if len(hits) != 1:
        sys.exit(f"refused: {len(hits)} lines of {path} starting with [{marker}]")
    return hits[0]


def cut(lines, path, start, end, rows=None):
    """Lines from the one starting with <start> to the one before the one starting with <end>.

    With <rows>, a table keeps its header, its separator and the rows whose first cell is in <rows>."""
    a = find(lines, start, path)
    b = find(lines, end, path)
    if b <= a:
        sys.exit(f"refused: [{end}] does not follow [{start}] in {path}")
    piece = lines[a:b]
    if rows is None:
        return piece
    kept = []
    header = False
    for ln in piece:
        if not ln.startswith("|"):
            header = False
            kept.append(ln)
            continue
        if not header:
            header = True
            kept.append(ln)
            continue
        if ln.startswith("|---"):
            kept.append(ln)
            continue
        first = ln.split("|")[1].strip()
        if first in rows:
            kept.append(ln)
    found = {ln.split("|")[1].strip() for ln in kept if ln.startswith("|")}
    missing = [r for r in rows if r not in found]
    if missing:
        sys.exit(f"refused: rows {missing} not found between [{start}] and [{end}] in {path}")
    return kept


plan = read(PLAN)
design = read(DESIGN)
head = subprocess.run(["git", "rev-parse", "--short", "HEAD"], capture_output=True, text=True).stdout.strip()

PIECES = [
    ("il piano, la testa", cut(plan, PLAN, "# Knowledge base, la revisione", "## Gli attrezzi")),
    ("il piano, «Gli attrezzi»", cut(plan, PLAN, "## Gli attrezzi", "## Vincoli globali")),
    ("il piano, «Vincoli globali»", cut(plan, PLAN, "## Vincoli globali", "## ▶️ A che punto è")),
    ("il piano, «A che punto è» e «Come si esegue un compito»", cut(plan, PLAN, "## ▶️ A che punto è", "## ⚠️ L'errata")),
    ("il piano, l'errata, per intero", cut(plan, PLAN, "## ⚠️ L'errata", "## Ciò che la scrittura del piano")),
    ("il piano, le voci P che il compito 2 nomina o usa",
     cut(plan, PLAN, "## Ciò che la scrittura del piano", "## Le decisioni prese scrivendo il piano",
         rows=["P-7", "P-15", "P-18", "P-19"])),
    ("il piano, le decisioni D che il compito 2 nomina o usa",
     cut(plan, PLAN, "## Le decisioni prese scrivendo il piano", "## La mappa dei file",
         rows=["D5", "D10", "D15", "D17", "D19", "D21", "D25"])),
    ("il piano, le voci aperte che il piano sa", cut(plan, PLAN, "## Le voci aperte che questo piano SA", "## La Definizione di «fatto»")),
    ("il piano, la Definizione di «fatto» e le trappole", cut(plan, PLAN, "## La Definizione di «fatto»", "## Compito 1:")),
    ("il piano, IL COMPITO 2, per intero", cut(plan, PLAN, "## Compito 2:", "## Compito 3:")),
    ("il disegno, la testa della sezione 3", cut(design, DESIGN, "## 3. Gli ADR", "### 3.1 I rimandi in testa")),
    ("il disegno, la 3.1 per intero, col capoverso sotto la tabella", cut(design, DESIGN, "### 3.1 I rimandi in testa", "### 3.2 L'ADR nuovo")),
]

out = [
    "# Il brief del compito 2 — i rimandi in testa a dieci ADR, e le loro voci nel compendio",
    "",
    f"> Estratto a `HEAD` = `{head}` da `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti-esecuzione/_extract_brief_2.py`,",
    "> **parola per parola**, dal piano e dal disegno: ogni pezzo porta sopra di sé da dove viene. Nei pezzi i link",
    "> relativi valgono dalla cartella del file d'origine, non da qui. È il **requisito**: i valori esatti stanno qui.",
    "",
]
for title, piece in PIECES:
    out += ["", "---", "", f"<!-- ═══ {title} ═══ -->", ""] + piece
os.makedirs(OUT_DIR, exist_ok=True)
with io.open(OUT, "w", encoding="utf-8", newline="") as f:
    f.write(NL.join(out) + NL)
print(f"{OUT}: {len(out)} lines, {sum(len(x) + 1 for x in out)} characters, HEAD {head}")
