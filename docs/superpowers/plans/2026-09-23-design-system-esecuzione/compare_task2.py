"""compare_task2.py -- the commit of task 2 against the PLAN'S TEXT: every file task 2 dictates is reconstructed from
the fences of the plan and from the base commit, and compared with the file in the commit under review. Line endings
aside (CRLF is read as LF on both sides), a difference is a finding. Same shape as compare_task1.py.

Usage: python compare_task2.py <base> <target>
  base    the commit task 2 starts from: the plan's fences and the original files are read there
  target  the commit under review (the implementer's)

The recipe -- the ```text fence under "La ricetta del compito 2" in the plan's "Come si riprende" -- says where each
fence is: W <file> <n> (the whole file is the fence opened at line n), R <file> <find> <replace> (the unique
occurrence of the first fence becomes the second), S <n> <file> (the throwaway probe of step 5: it must NOT be in
the target). The `<data>` of the dated recall is the date of row 2 in the target's position table.
Beyond the recipe: package.json is the base one plus the two devDependencies, exact, in npm's alphabetical order;
package-lock.json pins them, with @vitest/browser and vitest at 4.1.11; the plan differs from the base by the two
cells of the position table (step 7): a cell that is not the dictated one is DIFFERS, and anything else in the
plan, an errata entry for instance, is shown as CHECK BY HAND and left to the reader. The date is read in row 2 of
the target, so a date that is not the commit's day is shown as CHECK BY HAND too: the dispatch fixes the day even
if the execution passes midnight. Exit 1 when something DIFFERS, is LEFT BEHIND or UNEXPECTED, 0 otherwise --
the plan's cells and the date counted since M-1 of the review of task 2, 2026-09-25.
"""
import difflib
import json
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


title = next(i for i, s in enumerate(plan) if s.startswith("📌 **La ricetta del compito 2**"))
start = next(i for i in range(title, len(plan)) if plan[i] == "```text")
end = next(i for i in range(start + 1, len(plan)) if plan[i] == "```")
recipe = plan[start + 1:end]

target_plan = show(target, PLAN)
row2 = re.search(r"^\| \*\*2\*\* \|.*\| (✅ (\d{4}-\d{2}-\d{2})) \|$", target_plan, re.M)
if not row2:
    raise SystemExit("row 2 of the position table in the target is not `✅ <date>`")
DATE = row2.group(2)
COMMIT_DAY = git("log", "-1", "--format=%cs", target).strip()

expected, originals, throwaway = {}, {}, []


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


for line in recipe:
    parts = line.split(" ")
    if parts[0] == "W":
        expected[parts[1]] = fence(int(parts[2])) + "\n"
    elif parts[0] == "R":
        replace_once(parts[1], fence(int(parts[2])), fence(int(parts[3])).replace("<data>", DATE), line)
    elif parts[0] == "S":
        throwaway.append(parts[2])
    else:
        raise SystemExit(f"unknown recipe line {line!r}")

# --- the plan: the two cells of step 7 -------------------------------------------------------------------------
want_plan = "\n".join(plan)
ROW1_OLD = "il tema nelle due direzioni | — | ✅ 2026-09-24 |"
ROW1_NEW = "il tema nelle due direzioni | `95068bb`, con le cure `69d10fa` ed `e2cd7df` | ✅ 2026-09-24 |"
ROW2_OLD = "il tema che segue il sistema | — | ⬜ |"
ROW2_NEW = f"il tema che segue il sistema | — | ✅ {DATE} |"
for old, new in ((ROW1_OLD, ROW1_NEW), (ROW2_OLD, ROW2_NEW)):
    assert want_plan.count(old) == 1, old
    want_plan = want_plan.replace(old, new)

# --- compare ---------------------------------------------------------------------------------------------------
changed = [p for p in git("diff", "--name-only", base, target).split("\n") if p]
bad = 0


def diff(want, got):
    for line in difflib.unified_diff(want.split("\n"), (got or "").split("\n"), "dictated", "committed", lineterm="", n=1):
        print("    " + line)


for path in sorted(set(expected) | set(changed) | set(throwaway)):
    got = show(target, path, ok_missing=True)
    if path in throwaway:
        ok = got is None
        print(f"{'OK' if ok else 'LEFT BEHIND':14} {path}  (the throwaway probe of step 5)")
        bad += not ok
    elif path in expected:
        ok = got == expected[path]
        print(f"{'OK' if ok else 'DIFFERS':14} {path}")
        if not ok:
            bad += 1
            diff(expected[path], got)
    elif path == "gui/package.json":
        want = json.loads(original(path))
        want["devDependencies"].update({"@vitest/browser-playwright": "4.1.11", "playwright": "1.63.0"})
        mine = json.loads(got)
        keys = list(mine["devDependencies"])
        ok = mine == want and keys == sorted(keys)
        print(f"{'OK' if ok else 'DIFFERS':14} {path}  (the two devDependencies, exact, in alphabetical order)")
        bad += not ok
    elif path == "gui/package-lock.json":
        lock = json.loads(got)["packages"]
        pins = {"": None, "node_modules/@vitest/browser-playwright": "4.1.11", "node_modules/@vitest/browser": "4.1.11",
                "node_modules/playwright": "1.63.0", "node_modules/playwright-core": "1.63.0",
                "node_modules/vitest": "4.1.11"}
        ok = all(lock.get(k, {}).get("version") == v for k, v in pins.items() if v) and \
            lock[""]["devDependencies"].get("@vitest/browser-playwright") == "4.1.11" and \
            lock[""]["devDependencies"].get("playwright") == "1.63.0"
        print(f"{'OK' if ok else 'DIFFERS':14} {path}  (the pins; the rest comes from the registry -- CHECK BY HAND)")
        bad += not ok
    elif path == PLAN:
        cells = got.count(ROW1_NEW) == 1 and got.count(ROW2_NEW) == 1
        if got == want_plan:
            print(f"OK             {path}  (the two cells of step 7)")
        elif not cells:
            print(f"DIFFERS        {path}  -- the two cells of step 7 are not the dictated ones:")
            bad += 1
            diff(want_plan, got)
        else:
            print(f"CHECK BY HAND  {path}  -- the two cells of step 7, and more:")
            diff(want_plan, got)
    else:
        print(f"UNEXPECTED     {path} -- changed by the commit, dictated by no step")
        bad += 1

if DATE != COMMIT_DAY:
    print(f"CHECK BY HAND  the date {DATE} of row 2 and of the dated recall is not the commit's day {COMMIT_DAY}")
print(f"--- {len(set(expected) | set(changed) | set(throwaway))} paths, {bad} not matching the plan's text; date {DATE}")
sys.exit(1 if bad else 0)
