"""compare_task9.py -- the commit of task 9 against the PLAN'S TEXT (design system; written at the pre-check of task 9,
2026-09-28, and proven there in both directions).

Usage: python compare_task9.py <base> <target> [--closure]
  base       the commit task 9 starts from: the plan's text and the original files are read there
  target     the commit under review (the implementer's)
  --closure  the target holds the coordinator's closure too (E120): the plan and the archive of the handoffs are
             then free text, and the archive is checked word for word; without it, touching them is UNEXPECTED

Task 9 writes documents only. Nine of them it DICTATES -- the rows, the pointer, the sections of Steps 2-8 and 10 --
and they are rebuilt here from the ```markdown fences of task 9 in the plan at <base>, applied to the files at <base>
the way the text says, and compared with the files at <target>: line endings aside, and a run of blank lines counted as
one. The date is the one the target writes in the roadmap (`eseguito il <date>`), and must be the same everywhere; the
free values -- the two figures of the JavaScript piece (`<prima>`, `<dopo>`, `<i due valori del giorno>`) and, if M-3
stayed open, `<Enn>` -- are matched as free text on their line, and SHOWN. `docs/HANDOFF.md` is free text, SHOWN.
Without --closure the plan and `docs/archivio/consegna-piano-design-system.md` must be untouched (E120); with it they
are SHOWN, and the archive must hold the plan's «Come si riprende» of <base> word for word, link targets aside (E117).
Any other path is UNEXPECTED, and so is a `mode change`. Exit 1 when something DIFFERS, is UNEXPECTED or MISSING, 0
otherwise.
"""
import difflib
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8")
CLOSURE = "--closure" in sys.argv
base, target = [a for a in sys.argv[1:] if a != "--closure"][:2]
PLAN = "docs/superpowers/plans/2026-09-23-design-system.md"
FREE = "\x00FREE\x00"


def git(*args):
    r = subprocess.run(["git", *args], capture_output=True)
    if r.returncode != 0:
        raise SystemExit(f"git {' '.join(args)}: {r.stderr.decode('utf-8', 'replace')}")
    return r.stdout.decode("utf-8")


def show(commit, path):
    return git("show", f"{commit}:{path}").replace("\r\n", "\n")


# --- the dictated blocks, from the plan at <base> -------------------------------------------------------------
plan = show(base, PLAN).split("\n")
start = next(i for i, l in enumerate(plan) if l.startswith("## Compito 9:"))
end = next(i for i in range(start, len(plan)) if plan[i].startswith("## Come si riprende"))
blocks, i = [], start
while i < end:
    if plan[i] == "```markdown":
        j = i + 1
        while not plan[j].startswith("```"):
            j += 1
        blocks.append("\n".join(plan[i + 1:j]))
        i = j
    i += 1
PREFIXES = ["| Accessibilità | 🔶 |", ">", "| 14 | **Design system**", "| **Design system (14)",
            "✅ **scritto il 2026-09-24, eseguito il", "Ultimo aggiornamento:", " ✅ **RICHIAMO DEL <data>:** il piano",
            "| ⛔ **il DESIGN SYSTEM**", "⏭️ **IL PROSSIMO PASSO: IL SOTTO-PROGETTO 13**", "**Aggiornato il <data>**",
            "## Il puntatore «Il prossimo passo»", "## ⛔ IL BROWSER DEI TEST", "| 5 | `dependency advisories`",
            " ✅ **Corretta il <data>**", "## Il design system della GUI — il piano e la sua esecuzione",
            " ✅ **Chiusa il <data>** dal compito 9", " ✅ **Chiusa il <data>**: `BaseStatus`",
            " ⛔ **Ancora aperta il <data>**", " 📌 **Misurato il <data>**", " ✅ **RICHIAMO DEL <data>:** toccato",
            " ✅ **RICHIAMO DEL <data>, compito 9", "✅ **IL PIANO È ESEGUITO"]
assert len(blocks) == len(PREFIXES), f"{len(blocks)} markdown fences in task 9, {len(PREFIXES)} expected"
for n, (b, p) in enumerate(zip(blocks, PREFIXES), 1):
    assert b.lstrip("\n").startswith(p), f"fence {n} starts with {b[:60]!r}, not {p!r}"

roadmap_t = show(target, "docs/roadmap.md")
# The date of the row this task writes -- not the first `eseguito il` of the file, which another plan's row carries.
m = re.search(r"^\| 14 \| \*\*Design system\*\*.*?chiuso il (\d{4}-\d{2}-\d{2})", roadmap_t, re.M)
if not m:
    raise SystemExit("the target's roadmap has no row 14 `chiuso il <date>`: nothing to compare against")
date = m.group(1)
print(f"the date the target writes: {date}")
B = [b.replace("<data>", date) for b in blocks]


def line_starting(text, prefix):
    hits = [l for l in text.split("\n") if l.startswith(prefix)]
    assert len(hits) == 1, f"{prefix!r}: {len(hits)} lines at the base"
    return hits[0]


def sub(text, old, new, label):
    assert text.count(old) == 1, f"{label}: {text.count(old)} occurrences at the base"
    return text.replace(old, new)


def append_cell(text, prefix, addition, label):
    row = line_starting(text, prefix)
    assert row.endswith(" |"), label
    return sub(text, row, row[:-2] + addition + " |", label)


expected = {}
T = "docs/tracciabilita.md"
t = show(base, T)
t = sub(t, line_starting(t, "| Accessibilità | ✅ |"), B[0], "2 row")
box = line_starting(t, "> ✅ **Aggiornata il 2026-09-22 con le sedi dei pezzi della GUI")
expected[T] = sub(t, box, box + "\n" + B[1], "2 box")

R = "docs/roadmap.md"
t = show(base, R)
row = line_starting(t, "| 13 |")
t = sub(t, row, row + "\n" + B[2], "3.1")
row = line_starting(t, "| **Registro delle guide (13)")
t = sub(t, row, row + "\n" + B[3], "3.2")
row = line_starting(t, "| [Design system della GUI](superpowers/plans/2026-09-23-design-system.md)")
t = sub(t, row, row[:row.index("⏳")] + B[4], "3.3")
expected[R] = sub(t, line_starting(t, "Ultimo aggiornamento:"), B[5], "3.4")

D = "docs/README.md"
t = show(base, D)
row = line_starting(t, "| [Design system — il disegno]")
new = row.replace("il design system della GUI", "il design system della GUI, il sotto-progetto 14")
expected[D] = sub(t, row, new[:-2] + B[6] + " |", "4")

C = "docs/COMPENDIO.md"
t = show(base, C)
row = line_starting(t, "| ⛔ **come si è ESEGUITO il sotto-progetto 2**")
t = sub(t, row, row + "\n" + B[7], "5")
L = t.split("\n")
p0 = next(k for k, l in enumerate(L) if l.startswith("⏭️ **IL PROSSIMO PASSO"))
p1 = next(k for k in range(p0, len(L)) if L[k] == "")
old6 = "\n".join(L[p0:p1])
t = sub(t, old6, B[8], "6.2")
L = t.split("\n")
h0 = next(k for k, l in enumerate(L) if l.startswith("**Aggiornato il"))
h1 = next(k for k in range(h0, len(L)) if L[k].endswith("§13."))
oldh = "\n".join(L[h0:h1 + 1])
expected[C] = sub(t, oldh, B[9], "6.3")


def rewrite(text):
    def fix(m):
        target_ = m.group(1)
        if target_.startswith("superpowers/"):
            return "](../" + target_ + ")"
        if target_.startswith("archivio/"):
            return "](" + target_[len("archivio/"):] + ")"
        if re.fullmatch(r"[^/:]+\.md(#.*)?", target_):
            return "](../" + target_ + ")"
        return m.group(0)
    return re.sub(r"\]\(([^)]+)\)", fix, text)


PH = "<il testo di old-h.txt, una riga vuota, il testo di old-6.txt — coi link riscritti>"
assert B[10].count(PH) == 1
archived_pointer = B[10].replace(PH, rewrite(oldh) + "\n\n" + rewrite(old6))
A = "docs/archivio/stato-storico.md"
expected[A] = show(base, A).rstrip("\n") + "\n\n" + archived_pointer + "\n"

Q = "docs/porta-di-qualita.md"
t = show(base, Q)
head = line_starting(t, "## Le contraddizioni registrate, e non risolte")
t = sub(t, "\n" + head, "\n" + B[11] + "\n\n" + head, "7.1")
seg = t[t.index("**Un comando solo:**"):].split("\n")[:15]
rows = [l for l in seg if re.match(r"^\| [567] \|", l)]
assert len(rows) == 3
t = sub(t, "\n".join(rows), B[12], "7.2 table")
t = sub(t, "\n⚠️ **[C-S0-1]**\n\n", "\n", "7.2 sign")
expected[Q] = append_cell(t, "| **C-S0-1** |", B[13], "7.3")

F = "docs/riferimenti.md"
t = show(base, F)
sec = B[14].replace("<i due valori del giorno>", FREE)
head = line_starting(t, "## Cosa NON abbiamo adottato, e perché")
expected[F] = sub(t, "\n" + head, "\n" + sec + "\n\n" + head, "8")

S = "docs/superpowers/specs/2026-09-22-design-system-design.md"
t = show(base, S)
t = append_cell(t, "| **N-2 di E235**", B[15], "10 N-2 of E235")
m3_open = "⛔ **Ancora aperta il" in show(target, S)
t = append_cell(t, "| **M-3 di E187**", B[17].replace("<Enn>", FREE) if m3_open else B[16], "10 M-3")
t = append_cell(t, "| **N-2 di E187**", B[18].replace("<prima>", FREE).replace("<dopo>", FREE), "10 N-2 of E187")
expected[S] = append_cell(t, "| **codice di prodotto** |", B[19], "10 product code")
print(f"M-3 in the target: {'still open' if m3_open else 'closed'}")

G = "docs/superpowers/specs/2026-09-07-direzione-gui-design.md"
expected[G] = append_cell(show(base, G), "| codice e spec non toccati |", B[20], "10 north star")


# --- the comparison -------------------------------------------------------------------------------------------
def norm(text):
    return re.sub(r"\n{3,}", "\n\n", text.rstrip("\n")) + "\n"


bad = False
for path, exp in expected.items():
    exp, got = norm(exp), norm(show(target, path))
    pattern = re.escape(exp).replace(re.escape(FREE), "([^\\n]*?)")
    m = re.fullmatch(pattern, got, re.S)
    if m:
        free = f"  free: {[g for g in m.groups()]}" if m.groups() else ""
        print(f"OK         {path}{free}")
        continue
    bad = True
    print(f"DIFFERS    {path}")
    shown = exp.replace(FREE, "<FREE>")
    for line in list(difflib.unified_diff(shown.split("\n"), got.split("\n"), "plan", "target", n=1, lineterm=""))[:40]:
        print("    " + line[:200])

ARCHIVE = "docs/archivio/consegna-piano-design-system.md"
# ⛔ E120: the implementer's commit writes neither the plan nor the archive of the handoffs -- Steps 11 and 12 are the
# coordinator's, in the closure. With --closure (a commit that holds the closure too) the two are free text, SHOWN.
FREE_TEXT = {"docs/HANDOFF.md"} | ({PLAN, ARCHIVE} if CLOSURE else set())
changed = git("diff", "--name-only", base, target).split()
for path in changed:
    if path in expected:
        continue
    if path in FREE_TEXT:
        print(f"SHOWN      {path} -- free text, read it")
        continue
    bad = True
    print(f"UNEXPECTED {path}")
for line in git("diff", "--summary", base, target).split("\n"):
    if "mode change" in line:
        bad = True
        print(f"UNEXPECTED {line.strip()}")
for path in expected:
    if path not in changed:
        bad = True
        print(f"MISSING    {path} -- dictated, and not changed")


def strip(text):
    return re.sub(r"\]\([^)]*\)", "]()", text)


if CLOSURE:
    resume = "\n".join(plan[end:]).strip("\n")
    if strip(resume) in strip(show(target, ARCHIVE)):
        print("OK         the plan's «Come si riprende» of the base is in the archive, word for word (E117)")
    else:
        bad = True
        print("DIFFERS    the plan's «Come si riprende» of the base is NOT in the archive word for word (E117)")
sys.exit(1 if bad else 0)
