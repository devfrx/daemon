"""blocks.py -- list or extract fenced code blocks of the design-system plan.

Usage:
  python blocks.py list <from_line> <to_line>
      lists every fenced block whose opening fence lies in [from_line, to_line] (1-based):
      fence line, closing line, language, content line count, first content line.
  python blocks.py get <fence_line> <out_file>
      writes the content of the block opened at <fence_line> to <out_file>, LF, UTF-8,
      with the fence's indentation stripped and one trailing newline.
From the repository root: python docs/superpowers/plans/2026-09-23-design-system-esecuzione/blocks.py list 1300 1700
"""
import io
import os
import subprocess
import sys

# The plan is found through git, so the script runs from any clone: it came from a machine-local test folder
# with the path of one machine written in, and moved to the tracked execution folder on 2026-09-24.
_ROOT = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=os.path.dirname(os.path.abspath(__file__)),
                       capture_output=True, check=True).stdout.decode("utf-8").strip()
PLAN = os.path.join(_ROOT, "docs/superpowers/plans/2026-09-23-design-system.md")
# The console of a Windows machine is cp1252, and the plan prints emoji: UTF-8 always.
sys.stdout.reconfigure(encoding="utf-8")


def load():
    raw = io.open(PLAN, encoding="utf-8", newline="").read()
    return raw.replace("\r\n", "\n").split("\n")


def parse(lines):
    blocks = []
    i = 0
    n = len(lines)
    while i < n:
        line = lines[i]
        stripped = line.lstrip(" ")
        indent = len(line) - len(stripped)
        ch = stripped[:1]
        if ch in ("`", "~") and stripped.startswith(ch * 3):
            k = len(stripped) - len(stripped.lstrip(ch))
            fence = ch * k
            lang = stripped[k:].strip()
            j = i + 1
            while j < n:
                s = lines[j].lstrip(" ")
                if s.startswith(fence) and s.strip() == ch * (len(s.strip())) and len(s.strip()) >= k:
                    break
                j += 1
            content = []
            for l in lines[i + 1:j]:
                lead = len(l) - len(l.lstrip(" "))
                content.append(l[min(lead, indent):])
            blocks.append((i + 1, j + 1, lang, content))
            i = j + 1
        else:
            i += 1
    return blocks


def main():
    lines = load()
    blocks = parse(lines)
    if sys.argv[1] == "list":
        lo, hi = int(sys.argv[2]), int(sys.argv[3])
        for (a, b, lang, content) in blocks:
            if lo <= a <= hi:
                first = content[0] if content else ""
                print(f"{a}-{b} [{lang}] {len(content)} lines | {first[:90]}")
    elif sys.argv[1] == "get":
        at = int(sys.argv[2])
        for (a, b, lang, content) in blocks:
            if a == at:
                with io.open(sys.argv[3], "w", encoding="utf-8", newline="") as f:
                    f.write("\n".join(content) + "\n")
                print(f"ok: block {a}-{b} [{lang}] -> {sys.argv[3]} ({len(content)} lines)")
                return
        sys.exit(f"no block opens at line {at}")


if __name__ == "__main__":
    main()
