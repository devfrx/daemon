"""patch_c3.py -- task 3, the three known rows (its deep review stays to do): R10-16 (the one Files entry without a
line-ending label: the `.bin` fixtures are binary), D75 (`<data>` in the two DATED RECALLs of `wire/ipc.rs` and in the
criterion), D76 («sub-project 2» in the six dictated comments; the two «milestone 6» stay: they are milestone 6 of
sub-project 1). Plus the ledger. Scoped, every anchor asserted right before its write, atomic.
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
LEDGER = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione\ledger.md"

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


def scoped(text, start_marker, end_marker, fn):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    return text[:lo] + fn(text[lo:hi]) + text[hi:]


def task3(seg):
    # R10-16
    seg = sub(seg, "- Create: `gui/schema/fixtures/*.bin` — un file per variante, **rigenerabili**\n",
              "- Create: `gui/schema/fixtures/*.bin` (**binari**: niente fine-riga da conservare) — un file per variante, **rigenerabili** (R10-16)\n")
    # D76
    seg = sub(seg, "/// One line of the step list: in milestone 2 these are registry invocations.", "/// One line of the step list: in sub-project 2 these are registry invocations.")
    seg = sub(seg, "/// core -> gui. In milestone 2 only the fake core produces these.", "/// core -> gui. In sub-project 2 only the fake core produces these.")
    seg = sub(seg, "NO GUI OF MILESTONE 2 SENDS THIS", "NO GUI OF SUB-PROJECT 2 SENDS THIS")
    seg = sub(seg, "4 of the milestone-2 design.", "4 of the sub-project 2 design.")
    seg = sub(seg, "for the milestone-2 SPA to compare", "for the sub-project 2 SPA to compare")
    seg = sub(seg, "milestone 2 does not serve `Request` at all", "sub-project 2 does not serve `Request` at all")
    # D75
    seg = sub(seg, "//! ✅ DATED RECALL, 2026-09-11 -- THE TRIGGER WRITTEN ABOVE FELL DUE TODAY", "//! ✅ DATED RECALL, <data> -- THE TRIGGER WRITTEN ABOVE FELL DUE TODAY")
    seg = sub(seg, "/// ⚠️ DATED RECALL, 2026-09-11 -- THE GRAPH READ ABOVE IS NO LONGER THIS TYPE'S GRAPH", "/// ⚠️ DATED RECALL, <data> -- THE GRAPH READ ABOVE IS NO LONGER THIS TYPE'S GRAPH")
    seg = sub(seg, "- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/kernel/src/wire/ipc.rs` → **2**",
              "- [ ] `grep -c 'DATED RECALL, <data>' crates/kernel/src/wire/ipc.rs` → **2**, con la data del giorno scritta al posto di `<data>` (D75)")
    left = [line for line in seg.split("\n") if "milestone" in line.lower()]
    assert all("milestone 6" in line for line in left) and len(left) == 2, left
    return seg


t = scoped(t, "\n## Compito 3:", "\n## Compito 4:", task3)

lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw
l = sub(l, "### Compito 3 — NON RIVISTO IN PROFONDITÀ (R1 caduto): resta da fare\n- R10-16 etichette eol ⬜ · D75 `<data>` ⬜ · D76 «sub-project 2» ⬜\n",
        "### Compito 3 — NON RIVISTO IN PROFONDITÀ (R1 caduto): le righe note sono applicate, la revisione in profondità resta da fare\n"
        "- R10-16 l'etichetta della sola voce che ne mancava (`*.bin`: binari, niente fine-riga) ✅ · D75 `<data>` nei due `DATED RECALL` di `wire/ipc.rs` e nel criterio ✅ · D76 «sub-project 2» nei sei punti dei commenti dettati; i due «milestone 6» restano, perché sono il Traguardo 6 del SP1 ✅ · Attrezzo: `patch_c3.py`\n")
l = sub(l, "compiti 15, 16 e 17 applicati (✅) in una ondata sola.",
        "compiti 15, 16 e 17 applicati (✅) in una ondata sola. **Ondata 13 (2026-09-15):** le tre righe note del 3 applicate (✅); resta ⬜ la sola metà dell'8 di R9b-5, con la sua revisione in profondità.")

for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 3 patched (three rows) and the ledger;", t.count("\n") - raw.count("\n"), "plan lines added")
