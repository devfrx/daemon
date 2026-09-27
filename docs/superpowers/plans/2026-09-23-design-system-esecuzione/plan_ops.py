"""plan_ops.py -- read the file edits a task of the design-system plan dictates, straight from the plan's text.

Usage:
  python plan_ops.py <plan.md> <task> list
  python plan_ops.py <plan.md> <task> apply <root> [--upto N] [--only-step S] [--dry]

<task> is the heading's number: 1 .. 9 or 6bis. An operation is one of:
  replace  -- "In `path`" sets the file; "*Trova*" then a fence is the old text, "*Sostituisci con*" then a fence the new one
  create   -- "Crea `path`" then a fence: the whole file, LF
  rewrite  -- "Riscrivi `path` ... per intero" then a fence: the whole file, with the file's terminator as it is
Each operation records the step ("Passo N") it belongs to. `apply` applies them in order, as replace_unique.py does:
the old text must occur exactly once (the texts drop one trailing newline; a CRLF file gets CRLF texts). It reports
every refusal and keeps going, so one run lists every broken anchor. Nothing else of the plan is run.
"""
import io
import os
import re
import sys

FENCE = re.compile(r"^```([A-Za-z0-9_-]*)\s*$")


def task_lines(plan, task):
    lines = io.open(plan, encoding="utf-8", newline="").read().replace("\r\n", "\n").split("\n")
    head = re.compile(r"^## Compito " + re.escape(task) + r":")
    start = next(i for i, line in enumerate(lines) if head.match(line))
    end = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("## ")), len(lines))
    return start, lines[start:end]


def parse(plan, task):
    start, lines = task_lines(plan, task)
    ops, current, pending, step = [], None, None, None
    trova = None
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r"^- \[ \] \*\*Passo (\d+[a-z]*)", line)
        if m:
            step = m.group(1)
        m = FENCE.match(line)
        if m:
            j = i + 1
            while not re.match(r"^```\s*$", lines[j]):
                j += 1
            body = "\n".join(lines[i + 1:j])
            where = start + i + 1  # 1-based line of the opening fence
            if pending in ("create", "rewrite"):
                ops.append({"kind": pending, "path": current, "text": body, "line": where, "step": step})
                pending = None
            elif pending == "find":
                trova = (body, where)
                pending = None
            elif pending == "replace":
                if trova is None:
                    sys.exit(f"parse error: a replacement without a find at line {where}")
                ops.append({"kind": "replace", "path": current, "old": trova[0], "new": body, "line": trova[1],
                            "new_line": where, "step": step})
                trova, pending = None, None
            i = j + 1
            continue
        m = re.match(r"^(In|Crea|Riscrivi) `([^`]+)`", line)
        if m:
            current = m.group(2)
            if m.group(1) == "Crea":
                pending = "create"
            elif m.group(1) == "Riscrivi":
                pending = "rewrite"
        # a find may share the line of "In `path`"
        if re.search(r"\*Trova[:*]", line) or re.search(r"\*Trova\*", line):
            pending = "find"
        if re.search(r"\*Sostituisci con[:*]", line):
            pending = "replace"
        i += 1
    return ops


def apply(ops, root, dry=False):
    refused = 0
    for n, op in enumerate(ops, 1):
        path = os.path.join(root, op["path"])
        tag = f"#{n:02d} step {op['step']} line {op['line']} {op['kind']} {op['path']}"
        if op["kind"] == "create":
            if os.path.exists(path):
                print(f"REFUSED {tag}: exists already")
                refused += 1
                continue
            if not dry:
                os.makedirs(os.path.dirname(path), exist_ok=True)
                with io.open(path, "w", encoding="utf-8", newline="") as f:
                    f.write(op["text"] + "\n")
            print(f"ok      {tag}")
            continue
        if not os.path.exists(path):
            print(f"REFUSED {tag}: no such file")
            refused += 1
            continue
        raw = io.open(path, encoding="utf-8", newline="").read()
        crlf = "\r\n" in raw
        if op["kind"] == "rewrite":
            text = op["text"] + "\n"
            out = text.replace("\n", "\r\n") if crlf else text
        else:
            def conv(t):
                return t.replace("\n", "\r\n") if crlf else t
            old, new = conv(op["old"]), conv(op["new"])
            count = raw.count(old)
            if count != 1:
                print(f"REFUSED {tag}: {count} occurrences -- {op['old'].splitlines()[0][:90]!r}")
                refused += 1
                continue
            out = raw.replace(old, new)
        if not dry:
            tmp = path + ".tmp"
            with io.open(tmp, "w", encoding="utf-8", newline="") as f:
                f.write(out)
            os.replace(tmp, path)
        print(f"ok      {tag} ({'CRLF' if crlf else 'LF'})")
    return refused


def main():
    plan, task, action = sys.argv[1:4]
    ops = parse(plan, task)
    if action == "list":
        for n, op in enumerate(ops, 1):
            first = (op.get("old") or op.get("text")).splitlines()[0] if (op.get("old") or op.get("text")) else ""
            print(f"#{n:02d} step {op['step']} line {op['line']} {op['kind']:8} {op['path']}  | {first[:80]}")
        print(f"{len(ops)} operations")
        return
    root = sys.argv[4]
    args = sys.argv[5:]
    upto = int(args[args.index("--upto") + 1]) if "--upto" in args else None
    only = args[args.index("--only-step") + 1] if "--only-step" in args else None
    chosen = [op for op in ops if (only is None or op["step"] == only)]
    if upto is not None:
        chosen = [op for op in chosen if op["step"] is not None and int(re.match(r"\d+", op["step"]).group()) <= upto]
    refused = apply(chosen, root, dry="--dry" in args)
    print(f"{len(chosen)} operations, {refused} refused")
    sys.exit(1 if refused else 0)


if __name__ == "__main__":
    main()
