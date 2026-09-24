"""compare_task1.py -- the comparison the copy `t1` gave on the machine `zagor`, rebuilt from the PLAN'S TEXT: every
file task 1 dictates is reconstructed from the fences of the plan and from the base commit, and compared with the
file in the commit under review. Line endings aside (CRLF is read as LF on both sides), a difference is a finding.

Usage: python compare_task1.py <base> <target>
  base    the commit task 1 starts from (d10d9a5): the plan's fences and the original files are read there
  target  the commit under review (the implementer's)

The recipe -- the ```text fence under "La ricetta del compito 1" in the plan's "Come si riprende" -- says where each
fence is: W <file> <n> (the whole file is the fence opened at line n), A <file> <n> (the fence is appended at the end
of the file), S <n> <name> (a script, not in the repository), R <file> <find> <replace> (the unique occurrence of the
first fence becomes the second), RL <file> with one `<` line and one `>` line (the same, for one line).
Beyond the recipe: base.css and themes.css are the board's cuts (the Passo 9 script's rule, repeated here), the .vue
files are the base ones with the Passo 13 renames (the RENAMES literal, read from the plan's fence), tokens.css is
gone, and the compendium differs by the one line of Passo 14. Exit 0 when everything matches, 1 otherwise.
"""
import ast
import difflib
import re
import subprocess
import sys

base, target = sys.argv[1:3]
PLAN = "docs/superpowers/plans/2026-09-23-design-system.md"
BOARD = "docs/superpowers/specs/2026-09-22-design-system-tavole/token.html"
sys.stdout.reconfigure(encoding="utf-8")


def git(*args, ok_missing=False):
    r = subprocess.run(["git", *args], capture_output=True)
    if r.returncode != 0:
        if ok_missing:
            return None
        raise SystemExit(f"git {' '.join(args)}: {r.stderr.decode('utf-8', 'replace')}")
    return r.stdout.decode("utf-8")


def show(commit, path, ok_missing=False):
    text = git("show", f"{commit}:{path}", ok_missing=ok_missing)
    return None if text is None else text.replace("\r\n", "\n")


plan = show(base, PLAN).split("\n")


def fence(n):
    """The body of the fence opened at line n (1-based), joined with LF, no trailing LF added."""
    i = n - 1
    if not plan[i].startswith("```"):
        raise SystemExit(f"plan line {n} does not open a fence: {plan[i]!r}")
    j = i + 1
    while not plan[j].startswith("```"):
        j += 1
    return "\n".join(plan[i + 1:j])


# --- the recipe ------------------------------------------------------------------------------------------------
title = next(i for i, s in enumerate(plan) if s.startswith("📌 **La ricetta del compito 1**"))
start = next(i for i in range(title, len(plan)) if plan[i] == "```text")
end = next(i for i in range(start + 1, len(plan)) if plan[i] == "```")
recipe = plan[start + 1:end]

expected = {}      # path -> expected text (LF), or None for a deleted file
notes = []
originals = {}


def original(path):
    if path not in originals:
        originals[path] = show(base, path)
    return originals[path]


def current(path):
    return expected[path] if path in expected else original(path)


def replace_once(path, old, new, where):
    text = current(path)
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"recipe {where}: the find text occurs {count} times in {path} at {base}")
    expected[path] = text.replace(old, new)


k = 0
while k < len(recipe):
    parts = recipe[k].split(" ")
    kind = parts[0]
    if kind == "W":
        expected[parts[1]] = fence(int(parts[2])) + "\n"
    elif kind == "A":
        # appended at the end: kept as a marker, checked below with its own rule
        expected[parts[1]] = ("APPEND", fence(int(parts[2])))
    elif kind == "S":
        pass
    elif kind == "R":
        replace_once(parts[1], fence(int(parts[2])), fence(int(parts[3])), recipe[k])
    elif kind == "RL":
        old = recipe[k + 1]
        new = recipe[k + 2]
        assert old.startswith("< ") and new.startswith("> "), f"RL without its two lines: {recipe[k:k + 3]}"
        replace_once(parts[1], old[2:], new[2:], recipe[k])
        k += 2
    else:
        raise SystemExit(f"unknown recipe line {recipe[k]!r}")
    k += 1

# --- beyond the recipe -----------------------------------------------------------------------------------------
board = show(base, BOARD)
MARKS = ["/* ===== proposta/base.css ===== */", "/* ===== proposta/themes.css ===== */",
         "/* ===== the board itself, only tokens ===== */"]


def cut(a, b):
    return board[board.index(a) + len(a):board.index(b)].lstrip("\n").rstrip() + "\n"


expected["gui/src/tokens/base.css"] = cut(MARKS[0], MARKS[1])
expected["gui/src/tokens/themes.css"] = cut(MARKS[1], MARKS[2])
expected["gui/src/tokens/tokens.css"] = None

rename_title = next(i for i, s in enumerate(plan) if s.startswith('"""rename_tokens.py'))
rename_src = "\n".join(plan[rename_title:])
literal = re.search(r"RENAMES = (\[.*?\n\])", rename_src, re.S).group(1)
RENAMES = ast.literal_eval(literal)
vues = [p for p in git("ls-tree", "-r", "--name-only", base, "--", "gui/src").split("\n") if p.endswith(".vue")]
found = {old: 0 for old, _, _ in RENAMES}
renamed = {}
for p in vues:
    text = current(p)
    for old, new, _ in RENAMES:
        found[old] += text.count(old)
        text = text.replace(old, new)
    renamed[p] = text
for old, _, want in RENAMES:
    if found[old] != want:
        notes.append(f"RENAMES: {old!r} found {found[old]} times at {base}, the plan expects {want}")
for p, text in renamed.items():
    # only the files the renames change: the others must stay as they were, and the diff of the commit says so
    if p not in expected and text != original(p):
        expected[p] = text

OLD_LINE = next(s for s in original("docs/COMPENDIO.md").split("\n") if s.startswith("Lo stile di oggi è un **segnaposto dichiarato**"))
pass14 = next(i for i, s in enumerate(plan) if s.startswith("- [ ] **Passo 14:"))
new_line = fence(next(i for i in range(pass14, len(plan)) if plan[i] == "```markdown") + 1)
replace_once("docs/COMPENDIO.md", OLD_LINE, new_line, "Passo 14")

# --- compare ---------------------------------------------------------------------------------------------------
changed = [p for p in git("diff", "--name-only", base, target).split("\n") if p]
bad = 0
for path in sorted(set(expected) | set(changed)):
    want = expected.get(path, "UNEXPECTED")
    got = show(target, path, ok_missing=True)
    if want == "UNEXPECTED":
        if path in ("gui/package.json", "gui/package-lock.json", PLAN):
            print(f"CHECK BY HAND  {path}")
        else:
            print(f"UNEXPECTED     {path} -- changed by the commit, dictated by no step")
            bad += 1
        continue
    if want is None:
        ok = got is None
        print(f"{'OK' if ok else 'NOT DELETED':14} {path}")
        bad += not ok
        continue
    if isinstance(want, tuple):
        block = want[1]
        orig = original(path)
        good = got is not None and got.endswith(block + "\n") and got.startswith(orig.rstrip("\n"))
        middle = None if not good else got[len(orig.rstrip("\n")):len(got) - len(block) - 1]
        ok = good and middle.strip() == ""
        print(f"{'OK' if ok else 'DIFFERS':14} {path}  (the fence appended to the base file; separator {middle!r})")
        bad += not ok
        continue
    if got == want:
        print(f"OK             {path}")
        continue
    bad += 1
    print(f"DIFFERS        {path}")
    for line in difflib.unified_diff((want or "").split("\n"), (got or "").split("\n"), "dictated", "committed", lineterm="", n=1):
        print("    " + line)

for n in notes:
    print("NOTE           " + n)
print(f"--- {len(set(expected) | set(changed))} paths, {bad} not matching the plan's text")
sys.exit(1 if bad else 0)
