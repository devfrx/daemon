"""compare_task5.py -- the commit of task 5 against the PLAN'S TEXT: every file task 5 dictates is reconstructed from
the fences of the plan and from the base commit, and compared with the file in the commit under review. Line endings
aside (CRLF is read as LF on both sides), a difference is a finding. Born from compare_task4.py: what is dictated
COUNTS, what is not is SHOWN -- and N-3 of the review of task 4 is cured: a change of a file's MODE is a finding too.

Usage: python compare_task5.py <base> <target>
  base    the commit task 5 starts from: the plan's fences and the original files are read there
  target  the commit under review (the implementer's)

The recipe -- the ```text fence under "La ricetta del compito 5" in the plan's "Come si riprende" -- says where each
piece is. Task 5 has five shapes, and `#` lines only name the step:
  W <file> <n>               the whole file is the fence opened at plan line n
  R <file> <find> <replace>  the unique occurrence of the first fence becomes the second
  RL <file>                  then `< ` lines and `> ` lines: an inline find/replace of the prose, the old and new texts
  X <file> <n>               then `[ ` and `] ` lines: the lines from the first anchor (in) to the second (out) become
                             the fence opened at line n -- step 1's block "taken from the file with the two anchors"
  S <n>                      the fence at line n is step 1's script, `retarget_confirm.py`: its PLAN literal is applied
                             here with its counts, as the script applies it
Task 5 adds no dependency, so a change to `gui/package.json` or to its lockfile is UNEXPECTED, like any other path no
step dictates, and so is a `mode change` in `git diff --summary` (N-3). The plan differs from the base by ONE cell of
the position table (step 9): in row 4 the column Commit with task 4's commit and its cure (R1-16). Row 5 stays `⬜` --
the coordinator writes it after the owner's step 8 (E35) -- so task 5 dictates no date, and the dispatch's date has
nothing here to be compared with (the other half of N-3). A cell that is not the dictated one is DIFFERS; anything else
in the plan, an errata entry for instance, is shown as CHECK BY HAND and left to the reader. Exit 1 when something
DIFFERS or is UNEXPECTED, 0 otherwise.
"""
import ast
import difflib
import re
import subprocess
import sys

base, target = sys.argv[1:3]
PLAN = "docs/superpowers/plans/2026-09-23-design-system.md"
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


title = next(i for i, s in enumerate(plan) if s.startswith("📌 **La ricetta del compito 5**"))
start = next(i for i in range(title, len(plan)) if plan[i] == "```text")
end = next(i for i in range(start + 1, len(plan)) if plan[i] == "```")
recipe = plan[start + 1:end]

expected, originals = {}, {}


def original(path):
    if path not in originals:
        originals[path] = show(base, path)
    return originals[path]


def current(path):
    return expected[path] if path in expected else original(path)


def replace_once(path, old, new, where, times=1):
    text = current(path)
    count = text.count(old)
    if count != times:
        raise SystemExit(f"recipe {where}: the find text occurs {count} times in {path} at {base}, not {times}")
    expected[path] = text.replace(old, new)


k = 0
while k < len(recipe):
    line = recipe[k]
    parts = line.split(" ")
    if not line.strip() or line.startswith("#"):
        k += 1
        continue
    if parts[0] == "W":
        expected[parts[1]] = fence(int(parts[2])) + "\n"
    elif parts[0] == "R":
        replace_once(parts[1], fence(int(parts[2])), fence(int(parts[3])), line)
    elif parts[0] == "RL":
        old, new = [], []
        while k + 1 < len(recipe) and recipe[k + 1].startswith("< "):
            k += 1
            old.append(recipe[k][2:])
        while k + 1 < len(recipe) and recipe[k + 1].startswith("> "):
            k += 1
            new.append(recipe[k][2:])
        replace_once(parts[1], "\n".join(old), "\n".join(new), line)
    elif parts[0] == "X":
        first, stop = recipe[k + 1], recipe[k + 2]
        if not (first.startswith("[ ") and stop.startswith("] ")):
            raise SystemExit(f"recipe {line}: the two anchors are missing")
        lines = current(parts[1]).split("\n")
        a = [i for i, s in enumerate(lines) if s == first[2:]]
        b = [i for i, s in enumerate(lines) if s == stop[2:]]
        if len(a) != 1 or len(b) != 1 or a[0] >= b[0]:
            raise SystemExit(f"recipe {line}: the anchors occur {len(a)} and {len(b)} times in {parts[1]} at {base}")
        expected[parts[1]] = "\n".join(lines[:a[0]] + fence(int(parts[2])).split("\n") + lines[b[0]:])
        k += 2
    elif parts[0] == "S":
        literal = re.search(r"^PLAN = (\{.*?^\})", fence(int(parts[1])), re.S | re.M)
        if not literal:
            raise SystemExit(f"recipe {line}: no PLAN literal in the script's fence")
        for path, renames in ast.literal_eval(literal.group(1)).items():
            for old, new, times in renames:
                replace_once(path, old, new, f"{line} ({old})", times)
    else:
        raise SystemExit(f"unknown recipe line {line!r}")
    k += 1

# --- the plan: the one cell of step 9 --------------------------------------------------------------------------
want_plan = "\n".join(plan)
ROW4_OLD = "icone disegnate e centrate, `axe` col contrasto | — | ✅ 2026-09-25 |"
ROW4_NEW = "icone disegnate e centrate, `axe` col contrasto | `841dc54`, con la cura `8d098d0` | ✅ 2026-09-25 |"
ROW5 = "M-3** chiusa per costruzione, e l'Assistente vocale a mano | — | ⬜ |"
assert want_plan.count(ROW4_OLD) == 1, ROW4_OLD
assert want_plan.count(ROW5) == 1, ROW5
want_plan = want_plan.replace(ROW4_OLD, ROW4_NEW)

# --- compare ---------------------------------------------------------------------------------------------------
changed = [p for p in git("diff", "--name-only", base, target).split("\n") if p]
modes = [s.strip() for s in git("diff", "--summary", base, target).split("\n") if s.strip().startswith("mode change")]
bad = 0


def diff(want, got):
    for line in difflib.unified_diff(want.split("\n"), (got or "").split("\n"), "dictated", "committed", lineterm="", n=1):
        print("    " + line)


for path in sorted(set(expected) | set(changed)):
    got = show(target, path, ok_missing=True)
    if path in expected:
        ok = got == expected[path]
        print(f"{'OK' if ok else 'DIFFERS':14} {path}")
        if not ok:
            bad += 1
            diff(expected[path], got)
    elif path == PLAN:
        cells = got.count(ROW4_NEW) == 1 and got.count(ROW5) == 1
        if got == want_plan:
            print(f"OK             {path}  (the one cell of step 9)")
        elif not cells:
            print(f"DIFFERS        {path}  -- row 4's Commit is not the dictated one, or row 5 is not `⬜`:")
            bad += 1
            diff(want_plan, got)
        else:
            print(f"CHECK BY HAND  {path}  -- the one cell of step 9, and more:")
            diff(want_plan, got)
    else:
        print(f"UNEXPECTED     {path} -- changed by the commit, dictated by no step")
        bad += 1

for mode in modes:
    print(f"UNEXPECTED     {mode} -- no step changes a file's mode (N-3)")
    bad += 1
print(f"--- {len(set(expected) | set(changed))} paths, {len(modes)} mode changes, {bad} not matching the plan's text")
sys.exit(1 if bad else 0)
