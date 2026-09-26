"""compare_task6.py -- the commit of task 6 against the PLAN'S TEXT: every file task 6 dictates is reconstructed from
the fences of the plan and from the base commit, and compared with the file in the commit under review. Line endings
aside (CRLF is read as LF on both sides), a difference is a finding. Born from compare_task5.py -- what is dictated
COUNTS, what is not is SHOWN, and a change of a file's MODE is a finding too -- with the date of compare_task4.py back.

Usage: python compare_task6.py <base> <target>
  base    the commit task 6 starts from: the plan's fences and the original files are read there
  target  the commit under review (the implementer's)

The recipe -- the ```text fence under "La ricetta del compito 6" in the plan's "Come si riprende" -- says where each
piece is. Task 6 has two shapes, and `#` lines only name the step:
  W <file> <n>               the whole file is the fence opened at plan line n
  R <file> <find> <replace>  the unique occurrence of the first fence becomes the second
Task 6 adds no dependency, so a change to `gui/package.json` or to its lockfile is UNEXPECTED, like any other path no
step dictates, and so is a `mode change` in `git diff --summary`. The plan differs from the base by TWO cells of the
position table (step 9): in row 5 the column Commit with task 5's commit and its cures (R1-16), and row 6 to
`✅ <date>`. The date is read in row 6 of the target, and a date that is not the commit's day is CHECK BY HAND. A cell
that is not the dictated one is DIFFERS; anything else in the plan, an errata entry for instance, is shown as CHECK BY
HAND and left to the reader. Exit 1 when something DIFFERS or is UNEXPECTED, 0 otherwise.
"""
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


title = next(i for i, s in enumerate(plan) if s.startswith("📌 **La ricetta del compito 6**"))
start = next(i for i in range(title, len(plan)) if plan[i] == "```text")
end = next(i for i in range(start + 1, len(plan)) if plan[i] == "```")
recipe = plan[start + 1:end]

target_plan = show(target, PLAN)
row6 = re.search(r"^\| \*\*6\*\* \|.*\| (✅ (\d{4}-\d{2}-\d{2})) \|$", target_plan, re.M)
if not row6:
    raise SystemExit("row 6 of the position table in the target is not `✅ <date>`")
DATE = row6.group(2)
COMMIT_DAY = git("log", "-1", "--format=%cs", target).strip()

expected, originals = {}, {}


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
        raise SystemExit(f"recipe {where}: the find text occurs {count} times in {path} at {base}, not once")
    expected[path] = text.replace(old, new)


for line in recipe:
    parts = line.split(" ")
    if not line.strip() or line.startswith("#"):
        continue
    if parts[0] == "W":
        expected[parts[1]] = fence(int(parts[2])) + "\n"
    elif parts[0] == "R":
        replace_once(parts[1], fence(int(parts[2])), fence(int(parts[3])), line)
    else:
        raise SystemExit(f"unknown recipe line {line!r}")

# --- the plan: the two cells of step 9 -------------------------------------------------------------------------
want_plan = "\n".join(plan)
ROW5_OLD = "l'Assistente vocale a mano | — | ✅ 2026-09-26 — il Passo 8"
ROW5_NEW = "l'Assistente vocale a mano | `545f500`, con le cure `7e25d03` e `9e6657b` | ✅ 2026-09-26 — il Passo 8"
ROW6_OLD = "`themeAbyss` esce | — | ⬜ |"
ROW6_NEW = f"`themeAbyss` esce | — | ✅ {DATE} |"
for old, new in ((ROW5_OLD, ROW5_NEW), (ROW6_OLD, ROW6_NEW)):
    assert want_plan.count(old) == 1, old
    want_plan = want_plan.replace(old, new)

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
        cells = got.count(ROW5_NEW) == 1 and got.count(ROW6_NEW) == 1
        if got == want_plan:
            print(f"OK             {path}  (the two cells of step 9)")
        elif not cells:
            print(f"DIFFERS        {path}  -- row 5's Commit or row 6's state is not the dictated one:")
            bad += 1
            diff(want_plan, got)
        else:
            print(f"CHECK BY HAND  {path}  -- the two cells of step 9, and more:")
            diff(want_plan, got)
    else:
        print(f"UNEXPECTED     {path} -- changed by the commit, dictated by no step")
        bad += 1

for mode in modes:
    print(f"UNEXPECTED     {mode} -- no step changes a file's mode")
    bad += 1
if DATE != COMMIT_DAY:
    print(f"CHECK BY HAND  the date {DATE} of row 6 is not the commit's day {COMMIT_DAY}")
print(f"--- {len(set(expected) | set(changed))} paths, {len(modes)} mode changes, {bad} not matching the plan's text; date {DATE}")
sys.exit(1 if bad else 0)
