"""restore_from_head.py -- put one or more tracked files back to their HEAD content, as LF, WITHOUT `git checkout --`.

⛔ WHY NOT `git checkout --`: with `core.autocrlf=true` the checkout rewrites an LF file as CRLF in the working tree
(`i/lf w/crlf`), and every `patch_*.py` here stops on its `assert "\\r\\n" not in raw` without writing a byte (the
thirteenth closure's first trap). This reads the blob from HEAD through git itself, normalises CRLF to LF, and writes
atomically -- so `git ls-files --eol` answers `i/lf w/lf` and `git status --porcelain` is clean afterwards.

Usage: python restore_from_head.py <path relative to the repo root> [...]
"""
import io
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"

for rel in sys.argv[1:]:
    blob = subprocess.run(["git", "show", f"HEAD:{rel.replace(os.sep, '/')}"], cwd=ROOT, capture_output=True, check=True).stdout
    text = blob.decode("utf-8").replace("\r\n", "\n")
    path = os.path.join(ROOT, rel)
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(text)
    os.replace(tmp, path)
    print(f"ok: {rel} restored from HEAD, LF, {text.count(chr(10))} lines")
