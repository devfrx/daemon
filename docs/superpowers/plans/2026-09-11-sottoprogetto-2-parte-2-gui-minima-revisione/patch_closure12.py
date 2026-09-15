"""patch_closure12.py -- insert the twelfth closure at the head of the diary, and point the position paragraph to it."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
PLAN = r"C:\Users\zagor\Desktop\harness\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
S = r"C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\70220bef-5dc3-4d57-a0cd-7088a6828df6\scratchpad\review"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw

closure = io.open(os.path.join(S, "closure12.md"), encoding="utf-8", newline="").read().replace("\r\n", "\n")
assert closure.startswith("### La dodicesima chiusura") and closure.endswith("\n\n")

# 1. the position paragraph
old = ("⛔ **Nessun compito è ancora eseguito**, e il passo che\nviene ora è la **revisione del piano intero** — l'undicesima chiusura del diario, in fondo, dice come.")
new = ("⛔ **Nessun compito è ancora eseguito**, e il passo che\nviene ora è la **revisione del piano intero** — l'undicesima chiusura del diario, in fondo, dice come.\n"
       "✅ **RICHIAMO DEL 2026-09-15, terza sessione del giorno: la revisione è FATTA per nove perimetri su undici e le correzioni sono\n"
       "applicate ai compiti 1–5** — la **dodicesima chiusura** dice dove si riprende, e il registro in\n"
       "[`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md) è la casa unica di ciò che resta.")
assert t.count(old) == 1, t.count(old)
t = t.replace(old, new, 1)

# 2. the closure, right after the diary header line and its blank line, before the eleventh closure
header = "## Come si riprende — il diario di questo piano, coi comandi\n\n### L'undicesima chiusura"
assert t.count(header) == 1
t = t.replace(header, "## Come si riprende — il diario di questo piano, coi comandi\n\n" + closure + "### L'undicesima chiusura", 1)

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: closure 12 inserted;", t.count("\n") - raw.count("\n"), "lines added")
