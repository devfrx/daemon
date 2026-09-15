"""insert_at_anchor.py -- insert a block of text right AFTER (default) or BEFORE one unique anchor line,
keeping the file's line endings. Never leaves a blank line inside a table: the block is joined to the
anchor with exactly one newline on each side.

Usage: python insert_at_anchor.py <file> <anchor.txt> <block.txt> [before]

<anchor.txt> holds ONE line (its trailing newline is dropped) that must occur exactly once in <file>.
<block.txt> holds the lines to insert (UTF-8, LF); a single trailing newline is dropped. If <file> is
CRLF the block is converted. Builds the whole content first, writes a temporary file, then os.replace.
"""
import io
import os
import sys

path, anchor_path, block_path = sys.argv[1:4]
before = len(sys.argv) > 4 and sys.argv[4] == "before"
raw = io.open(path, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw
nl = "\r\n" if crlf else "\n"


def text(p):
    t = io.open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
    if t.endswith("\n"):
        t = t[:-1]
    return t.replace("\n", nl) if crlf else t


anchor = text(anchor_path)
block = text(block_path)
if nl in anchor:
    sys.exit("refused: the anchor must be ONE line")
lines = raw.split(nl)
hits = [i for i, l in enumerate(lines) if l == anchor]
if len(hits) != 1:
    sys.exit(f"refused: {len(hits)} lines equal to the anchor in {path}")
i = hits[0]
new_lines = block.split(nl)
if before:
    lines[i:i] = new_lines
else:
    lines[i + 1:i + 1] = new_lines
out = nl.join(lines)
tmp = path + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print(f"ok: {path} ({'CRLF' if crlf else 'LF'}), {len(new_lines)} lines inserted {'before' if before else 'after'} line {i + 1}")
