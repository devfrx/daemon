"""patch_c1.py -- task 1 corrections (R1-1..R1-6, D75, D76). Anchors asserted unique; atomic write."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
PLAN = r"C:\Users\zagor\Desktop\harness\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw

subs = [
    # R1-5: consumers
    ("- Produces, e i compiti 2, 7, 9 e 12 li usano con questi nomi esatti:\n  - `kernel::numbering::Progressive`",
     "- Produces, e i compiti 2, 7, 9, 10 e 12 li usano con questi nomi esatti:\n  - `kernel::numbering::Progressive`"),
    # R1-4: the alias and its #[allow]
    ("    let written: alloc_vec_of_steps = journal",
     "    let written: Vec<u64> = journal"),
    ("}\n\n#[allow(non_camel_case_types)]\ntype alloc_vec_of_steps = std::vec::Vec<u64>;\n```",
     "}\n```"),
    ("⚠️ **L'alias in coda esiste per una ragione:** un banco di integrazione è una crate a sé e **ha** `std`;\n"
     "il `Vec` di `alloc` che `kernel` usa non si nomina da qui senza un `extern crate alloc`. L'alias dice\n"
     "che è una scelta e non una distrazione. ⛔ **Se l'implementatore preferisce `Vec<u64>` nudo, va bene\n"
     "uguale**: è un banco, non il kernel — ma allora si toglie anche l'alias, e non si lasciano tutti e due.",
     "⚠️ **`Vec<u64>` nudo, e nessun alias:** un banco di integrazione è una crate a sé e **ha** `std`, e il `Vec`\n"
     "del preludio è lo stesso tipo di quello di `alloc` — quattordici banchi di `kernel` lo nominano nudo e nessuno\n"
     "scrive `extern crate alloc`. ✅ **RICHIAMO DEL 2026-09-15, revisione del piano intero (R1-4):** qui stavano un\n"
     "alias `alloc_vec_of_steps` con un `#[allow(non_camel_case_types)]` e un capoverso che lo giustificava — un\n"
     "`#[allow]` **nuovo**, che il vincolo globale 15 vieta e che `gate-attributes.sh` non avrebbe visto (legge i soli\n"
     "`lib.rs`), con una ragione che non reggeva."),
    # R1-1: anchor (a) is the second line of a wrapped sentence
    ("— la riga che finisce con *«WHEN the allocator arrives is the owner's: registered, not taken.»*. Si\naggiunge **dopo** di essa, nello stesso blocco di doc:",
     "— la riga che finisce con *«allocator arrives is the owner's: registered, not taken.»* (⚠️ la frase va a\ncapo dopo «WHEN the»: l'ancora è la **seconda** riga, unica nel file — R1-1). Si aggiunge **dopo** di essa,\nnello stesso blocco di doc:"),
    # R1-2: anchor (b)
    ("la riga *«two independent counters that look identical, diverging with nothing to report it.»*:",
     "la riga che finisce con *«with nothing to report it.»* (⚠️ la frase va a capo dopo «diverging»: l'ancora è\nl'ultima riga del blocco dei richiami, unica nel file — R1-2):"),
    # R1-3: the quotation drops "in milestone 6"
    ("`crate::numbering::seeded_from` above every step in the journal. \"Whoever implements this\n/// port draws from THAT counter rather than starting a private one of its own\" is no longer a",
     "`crate::numbering::seeded_from` above every step in the journal. \"Whoever implements this\n/// port in milestone 6 draws from THAT counter rather than starting a private one of its own\" is no longer a"),
    # D75: <data> in the two dictated recalls of task 1, and in its criterion
    ("/// ⚠️ DATED RECALL, 2026-09-11 -- THE COUNTER EXISTS NOW, AND THE ALLOCATOR STILL DOES NOT.",
     "/// ⚠️ DATED RECALL, <data> -- THE COUNTER EXISTS NOW, AND THE ALLOCATOR STILL DOES NOT."),
    ("/// ⚠️ DATED RECALL, 2026-09-11 -- THE COUNTER IT GUARDS NOW EXISTS, AND THIS PORT IS STILL NOT",
     "/// ⚠️ DATED RECALL, <data> -- THE COUNTER IT GUARDS NOW EXISTS, AND THIS PORT IS STILL NOT"),
    ("- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/kernel/src/ports/journal.rs crates/kernel/src/ports/ipc.rs` → **1** per file",
     "- [ ] `grep -c 'DATED RECALL, <data>' crates/kernel/src/ports/journal.rs crates/kernel/src/ports/ipc.rs` → **1** per file, con la data del giorno scritta al posto di `<data>` (D75), e `grep -c '<data>'` sugli stessi due file → **0**"),
    # D76: the sub-project's design in the recall (a)
    ("operation here allocates, and whether one should is open item 6 of §9 of the milestone-2\n/// design, confirmed A by the owner on 2026-09-09",
     "operation here allocates, and whether one should is open item 6 of §9 of the sub-project 2\n/// design, confirmed A by the owner on 2026-09-09"),
]
for old, new in subs:
    n = t.count(old)
    assert n == 1, f"anchor count {n}: {old[:80]!r}"
for old, new in subs:
    t = t.replace(old, new, 1)

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: task 1 patched", len(subs), "substitutions")
