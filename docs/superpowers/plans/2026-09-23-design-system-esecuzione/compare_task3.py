"""compare_task3.py -- the commit of task 3 against the PLAN'S TEXT: every file task 3 dictates is reconstructed from
the fences of the plan and from the base commit, and compared with the file in the commit under review. Line endings
aside (CRLF is read as LF on both sides), a difference is a finding. Born from compare_task2.py as cured by M-1 of
the review of task 2: what is dictated COUNTS, what is not is SHOWN.

Usage: python compare_task3.py <base> <target>
  base    the commit task 3 starts from: the plan's fences and the original files are read there
  target  the commit under review (the implementer's)

The recipe -- the ```text fence under "La ricetta del compito 3" in the plan's "Come si riprende" -- says where each
fence is: W <file> <n> (the whole file is the fence opened at line n), R <file> <find> <replace> (the unique
occurrence of the first fence becomes the second), B <file> (the third replacement of step 2: the block taken FROM
THE FILE, from the `/**` above "Every violation axe finds" down to the line before "/** Fills the stores ...",
goes away). Beyond the recipe: package.json is the base one plus `lucide` 1.47.0 in `dependencies`, exact, in npm's
alphabetical order; package-lock.json pins it -- lucide 1.47.0, ISC, no install script, or DIFFERS -- and any other
change from the base's lockfile is CHECK BY HAND, shown by its key (M-6 of the review of task 3: before, the pin
alone was read, and a drift of another entry said OK); the plan differs from the base by the two cells of the position table
(step 9) -- row 3 to `✅ <date>`, and in row 2 the column Commit with task 2's commit and its cure (R1-16): a cell
that is not the dictated one is DIFFERS, and anything else in the plan, an errata entry for instance, is shown as
CHECK BY HAND and left to the reader. The date is read in row 3 of the target, and a date that is not the commit's
day is shown as CHECK BY HAND: the dispatch fixes the day even if the execution passes midnight. Exit 1 when
something DIFFERS or is UNEXPECTED, 0 otherwise.
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


title = next(i for i, s in enumerate(plan) if s.startswith("📌 **La ricetta del compito 3**"))
start = next(i for i in range(title, len(plan)) if plan[i] == "```text")
end = next(i for i in range(start + 1, len(plan)) if plan[i] == "```")
recipe = plan[start + 1:end]

target_plan = show(target, PLAN)
row3 = re.search(r"^\| \*\*3\*\* \|.*\| (✅ (\d{4}-\d{2}-\d{2})) \|$", target_plan, re.M)
if not row3:
    raise SystemExit("row 3 of the position table in the target is not `✅ <date>`")
DATE = row3.group(2)
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
        raise SystemExit(f"recipe {where}: the find text occurs {count} times in {path} at {base}")
    expected[path] = text.replace(old, new)


def drop_helper_block(path, where):
    """Step 2, third replacement: from the `/**` above 'Every violation axe finds' to the line before
    '/** Fills the stores ...', both found by text -- the implementer takes the block from the file the same way."""
    lines = current(path).split("\n")
    first = [i for i, s in enumerate(lines) if "Every violation axe finds" in s]
    last = [i for i, s in enumerate(lines) if s.startswith("/** Fills the stores")]
    if len(first) != 1 or len(last) != 1 or lines[first[0] - 1].strip() != "/**":
        raise SystemExit(f"recipe {where}: the helper block is not where step 2 says, in {path} at {base}")
    expected[path] = "\n".join(lines[:first[0] - 1] + lines[last[0]:])


for line in recipe:
    parts = line.split(" ")
    if parts[0] == "W":
        expected[parts[1]] = fence(int(parts[2])) + "\n"
    elif parts[0] == "R":
        replace_once(parts[1], fence(int(parts[2])), fence(int(parts[3])).replace("<data>", DATE), line)
    elif parts[0] == "B":
        drop_helper_block(parts[1], line)
    else:
        raise SystemExit(f"unknown recipe line {line!r}")

# --- the plan: the two cells of step 9 -------------------------------------------------------------------------
want_plan = "\n".join(plan)
ROW2_OLD = "il tema che segue il sistema | — | ✅ 2026-09-25 |"
ROW2_NEW = "il tema che segue il sistema | `23b3134`, con la cura `5f32158` | ✅ 2026-09-25 |"
ROW3_OLD = "e le due regole sui pezzi di base | — | ⬜ |"
ROW3_NEW = f"e le due regole sui pezzi di base | — | ✅ {DATE} |"
for old, new in ((ROW2_OLD, ROW2_NEW), (ROW3_OLD, ROW3_NEW)):
    assert want_plan.count(old) == 1, old
    want_plan = want_plan.replace(old, new)

# --- compare ---------------------------------------------------------------------------------------------------
changed = [p for p in git("diff", "--name-only", base, target).split("\n") if p]
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
    elif path == "gui/package.json":
        want = json.loads(original(path))
        want["dependencies"]["lucide"] = "1.47.0"
        mine = json.loads(got)
        keys = list(mine["dependencies"])
        ok = mine == want and keys == sorted(keys)
        print(f"{'OK' if ok else 'DIFFERS':14} {path}  (lucide 1.47.0 in dependencies, exact, in alphabetical order)")
        bad += not ok
    elif path == "gui/package-lock.json":
        # M-6 of the review of task 3: the pin COUNTS, and every other change from the base is SHOWN, by its key.
        lock, want = json.loads(got), json.loads(original(path))
        entry = lock["packages"].get("node_modules/lucide", {})
        pin = entry.get("version") == "1.47.0" and entry.get("license") == "ISC" and \
            not entry.get("hasInstallScript") and lock["packages"][""].get("dependencies", {}).get("lucide") == "1.47.0"
        # the lockfile the plan dictates: the base's, plus lucide in the root's dependencies and its own entry
        want["packages"][""].setdefault("dependencies", {})["lucide"] = "1.47.0"
        want["packages"]["node_modules/lucide"] = entry
        moved = [(f"(top) {k}", want.get(k), lock.get(k)) for k in sorted(set(want) | set(lock))
                 if k != "packages" and want.get(k) != lock.get(k)]
        moved += [(k or "(root)", want["packages"].get(k), lock["packages"].get(k))
                  for k in sorted(set(want["packages"]) | set(lock["packages"]))
                  if want["packages"].get(k) != lock["packages"].get(k)]
        if not pin:
            print(f"DIFFERS        {path}  -- the pin is not lucide 1.47.0, ISC, without an install script: {entry}")
            bad += 1
        elif moved:
            print(f"CHECK BY HAND  {path}  -- the pin, and {len(moved)} more change(s) from the base:")
            for key, old, new in moved:
                if isinstance(old, dict) and isinstance(new, dict):
                    fields = sorted(f for f in set(old) | set(new) if old.get(f) != new.get(f))
                    old, new = {f: old.get(f) for f in fields}, {f: new.get(f) for f in fields}
                print(f"    {key}: {old!r} -> {new!r}")
        else:
            print(f"OK             {path}  (the pin -- lucide 1.47.0, ISC, no install script -- and nothing else)")
    elif path == PLAN:
        cells = got.count(ROW2_NEW) == 1 and got.count(ROW3_NEW) == 1
        if got == want_plan:
            print(f"OK             {path}  (the two cells of step 9)")
        elif not cells:
            print(f"DIFFERS        {path}  -- the two cells of step 9 are not the dictated ones:")
            bad += 1
            diff(want_plan, got)
        else:
            print(f"CHECK BY HAND  {path}  -- the two cells of step 9, and more:")
            diff(want_plan, got)
    else:
        print(f"UNEXPECTED     {path} -- changed by the commit, dictated by no step")
        bad += 1

if DATE != COMMIT_DAY:
    print(f"CHECK BY HAND  the date {DATE} of row 3 is not the commit's day {COMMIT_DAY}")
print(f"--- {len(set(expected) | set(changed))} paths, {bad} not matching the plan's text; date {DATE}")
sys.exit(1 if bad else 0)
