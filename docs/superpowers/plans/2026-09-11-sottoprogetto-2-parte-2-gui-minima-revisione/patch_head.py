"""patch_head.py -- first wave of the whole-plan review: head-level corrections + D75..D88.
Every anchor is asserted unique BEFORE anything is written; the file is written once, atomically.
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = os.path.join(ROOT, r"docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md")
S = r"C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\70220bef-5dc3-4d57-a0cd-7088a6828df6\scratchpad\review"

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw, "the plan must be LF"
text = raw

# --- 1. exact-text substitutions (each old text must be unique) ---
subs = [
    # R10-3: position row 7
    ("Il limite di giri è al 8 — richiamo del 2026-09-11, D21",
     "Il limite di giri è al 9 — richiamo del 2026-09-11, D21 (⚠️ qui stava «al 8», il numero di prima di D25: corretto il 2026-09-15 alla revisione del piano intero, R10-3)"),
    # R10-4 / R9b-8 / R5-35: D2
    ("e il compito 12 lo scrive accanto al primo uso",
     "e il compito 12 lo scrive accanto al primo uso ✅ **RICHIAMO DEL 2026-09-15, revisione del piano intero (R10-4, R9b-8): il compito è il 14** — la nota vive in `gui/src/frame/moveActive.ts`, accanto alle firme che le otto mosse esercitano; «12» era la cornice prima di D25, e la cornice (13) non porta la nota"),
    # R10-5 / R9b-10: D4
    ("⛔ **La misura si rifà al compito 10**",
     "⛔ **La misura si rifà al compito 10** (✅ **RICHIAMO DEL 2026-09-15, revisione del piano intero (R10-5): al compito 11**, che installa — «10» è il numero di prima di D25, e la stessa riga lo dice più avanti)"),
    # R10-6: D12
    ("spostare il compito 3 dopo il 10 romperebbe il taglio per artefatto (D1) e lascerebbe lo schema senza controllo per sette compiti",
     "spostare il compito 3 dopo l'11 romperebbe il taglio per artefatto (D1) e lascerebbe lo schema senza controllo per tutti i compiti fra i due (⚠️ **RICHIAMO DEL 2026-09-15, revisione del piano intero (R10-6): qui stava «dopo il 10 … per sette compiti»**, numeri di prima di D25; il numerale è tolto, vincolo globale 3)"),
    # R4-15: the second placeholder in task 9
    ('interprocess = { version = "<la versione che `platform` appunta>", default-features = false }',
     'interprocess = "2.4.4"'),
]
for old, new in subs:
    n = text.count(old)
    assert n == 1, f"anchor not unique ({n}): {old[:70]}"
for old, new in subs:
    text = text.replace(old, new, 1)

# --- 2. the Interfaces line of task 11 (R10-7 / R5-9): the old form occurs three times; pick the block line ---
lines = text.split("\n")
hits = [i for i, l in enumerate(lines)
        if "`loadFixtures(): Fixture[]`" in l and "interface Fixture { file: string; kind: string; value: unknown }" in l]
assert len(hits) == 1, f"Interfaces line of task 11: {len(hits)} hits"
lines[hits[0]] = lines[hits[0]].replace(
    "interface Fixture { file: string; kind: string; value: unknown }",
    "interface Fixture { file: string; message: IpcMessage }", 1)

# --- 3. a recall paragraph at the end of P-75 (before the P-76 header) ---
p76 = [i for i, l in enumerate(lines) if l.startswith("### P-76 ")]
assert len(p76) == 1
i = p76[0]
assert lines[i - 1] == "", "expected a blank line before the P-76 header"
lines[i - 1:i - 1] = [
    "",
    "✅ **RICHIAMO DEL 2026-09-15, dalla revisione del piano intero (R10-7, R5-9):** la correzione che questa voce, la nona chiusura del diario e la lista di lettura del compito 13 davano per fatta **non era nel file** — il blocco *Interfaces* dell'11 diceva ancora `{ file: string; kind: string; value: unknown }`. Applicata oggi. 📌 Una `P` che dice «corretto nel compito» si verifica col `grep` sul blocco, non sulla memoria di averlo fatto.",
]

# --- 4. the new D rows, right after the D74 row ---
d74 = [i for i, l in enumerate(lines) if l.startswith("| **D74** |")]
assert len(d74) == 1
rows = io.open(os.path.join(S, "drows.md"), encoding="utf-8", newline="").read().replace("\r\n", "\n").rstrip("\n").split("\n")
assert all(r.startswith("| **D") for r in rows) and len(rows) == 14, len(rows)
assert lines[d74[0] + 1] == "", "expected a blank line after the D74 row"
lines[d74[0] + 1:d74[0] + 1] = rows

out = "\n".join(lines)
tmp = PLAN + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, PLAN)
print("ok: plan patched;", out.count("\n") - raw.count("\n"), "lines added")
