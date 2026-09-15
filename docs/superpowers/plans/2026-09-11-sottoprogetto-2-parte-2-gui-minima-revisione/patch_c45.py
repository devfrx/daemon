"""patch_c45.py -- tasks 4 and 5 (R2-1..R2-11, R9a-6/R9b-1 via D14, D75, D76, D88). Scoped, asserted, atomic."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
PLAN = r"C:\Users\zagor\Desktop\harness\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def scoped(text, start_marker, end_marker, subs):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    seg = text[lo:hi]
    for old, new, n in subs:
        c = seg.count(old)
        assert c == n, f"expected {n}, found {c}: {old[:80]!r}"
    for old, new, n in subs:
        seg = seg.replace(old, new)
    return text[:lo] + seg + text[hi:]


# ------------------------------------------------------------------ task 4
t = scoped(t, "\n## Compito 4:", "\n## Compito 5:", [
    # R2-7 consumers
    ("- Produces, e i compiti 5, 7, 9 e 12 li usano con questi nomi esatti:",
     "- Produces, e i compiti 5, 7, 9, 10 e 12 li usano con questi nomi esatti:", 1),
    # R2-4 baseline in step 1
    ("grep -c '^struct \\|^impl .* for ' crates/kernel/tests/ports_are_implementable.rs\n",
     "grep -c '^struct \\|^impl .* for ' crates/kernel/tests/ports_are_implementable.rs\ncargo test --locked -p kernel --test ports_are_implementable 2>&1 | tail -1\n", 1),
    ("Atteso: `custody.rs` **non esiste**; **sei** file più `mod.rs`; le cifre in prosa alle righe",
     "Atteso: `custody.rs` **non esiste**; **sei** file più `mod.rs`; il `grep -c` sul banco delle finte → **11** e il banco\n→ **14 passed** (la baseline del criterio di chiusura, R2-4); le cifre in prosa alle righe", 1),
    # R2-3 / R2-2: the table of touches
    ("rende si legge **intera**. Cinque tocchi, e il quinto è quello che nessuno si aspetta:",
     "rende si legge **intera**. Nove righe in tabella — le nove del censimento del Passo 1 (R2-3) — di cui quattro si\ntoccano, una riceve un richiamo sotto (la 43, col tocco 5), e quattro si lasciano stare **con la ragione scritta**; il\nquinto tocco è quello che nessuno si aspetta:", 1),
    ("| 5 | *«FIVE fakes, because `process` needs two of them»* | **SIX fakes**. ⚠️ La riga sotto porta già un **richiamo datato del 2026-08-28** (AUD-054): si **legge prima**, e il richiamo nuovo si aggiunge **senza cancellarlo** |",
     "| 5 | *«— FIVE fakes, because»* — ⚠️ la frase va a capo dopo «because»: il *Trova* è la sola riga **37**, unica nel file (R2-2) | **SIX fakes**. ⚠️ La riga **43** porta già un **richiamo datato del 2026-08-28** (AUD-054) che dice *«The FIVE is still TRUE OF THAT FILE»* e diventa **falso** con questo tocco: si **legge prima**, non si cancella, e riceve **sotto** la riga nuova ``//! ⚠️ DATED RECALL, <data>, sub-project 2 task 4: SIX from here on -- `custody` has a fake of its own; the FIVE above is dated, not realigned (gotcha #31).`` (R2-3) |", 1),
    ("| 6 | ⛔ *«the simulator substitutes SEVEN things while §2.3 enumerates SIX … so that nobody \"fixes\" the discrepancy … **or by writing \"seven families\" in the line above**»* | **EIGHT contro SEVEN** — ed è **P-21** |",
     "| 6 | ⛔ *«the simulator substitutes SEVEN things while §2.3 enumerates SIX … so that nobody \"fixes\" the discrepancy … **or by writing \"seven families\" in the line above**»* | **EIGHT contro SEVEN** — ed è **P-21** |\n"
     "| 7 | riga **29**, *«only place the sixth family could be named at all»* | resta: parla del **meccanismo** — un `pub mod` che nomina un file — e non conta le famiglie di oggi. ⛔ **Non si tocca** (R2-3) |\n"
     "| 8 | riga **32**, *«Remove it and a seventh family …»* | resta: con `custody` la frase diventa **vera alla lettera** — la settima famiglia è arrivata proprio così — non falsa. ⛔ **Non si tocca**, ed è scritto qui perché il prossimo censimento non la «aggiorni» (R2-3) |\n"
     "| 9 | riga **43**, il richiamo di AUD-054: *«The FIVE is still TRUE OF THAT FILE»* | ⛔ **diventa FALSO col tocco 5**, che dice come: il richiamo nuovo si appende **sotto**, senza cancellare quello del 2026-08-28 (R2-3) |", 1),
    # R2-1: the anchor of (a)
    ("(a) La riga **209**, il riquadro dell'anello 3. Il *Trova* è la riga intera presa dal file; si **aggiunge** in\ncoda al riquadro, senza toccare ciò che c'è:",
     "(a) Il riquadro dell'anello 3, che la riga **209** apre. ⛔ **Il *Trova* NON è la riga 209: è l'ULTIMA riga del\nriquadro, `> come \\`process\\` in §2.3.1.`, unica nel file (R2-1)** — la 209 finisce a metà frase (*«ed è la»*), e un\nrichiamo inserito dopo di essa spezzerebbe la frase e il riquadro. Si **aggiunge** dopo l'ultima riga, senza toccare\nciò che c'è:", 1),
    # R2-5: the error code of C3
    ("| **C3** | in `custody.rs`, cambia `retrieve` in `-> Result<Vec<u8>, CustodyError>` | **il banco non compila**, `E0308`: è la prova che la firma è esercitata davvero e non solo dichiarata |",
     "| **C3** | in `custody.rs`, cambia `retrieve` in `-> Result<Vec<u8>, CustodyError>` | **il banco non compila** — `E0053` sull'`impl` (il metodo ha un tipo incompatibile col tratto) più tre `E0308` sulle asserzioni, misurato alla revisione del piano intero (R2-5): è la prova che la firma è esercitata davvero e non solo dichiarata |", 1),
    # D76
    ("(decision 35 of the milestone-2 design)", "(decision 35 of the sub-project 2 design)", 1),
    # D75: the dictated recalls and the criteria
    ("DATED RECALL, 2026-09-11 -- THE NUMBERS MOVED", "DATED RECALL, <data> -- THE NUMBERS MOVED", 1),
    ("> ⛔ **RICHIAMO DEL 2026-09-11 — le famiglie sono SETTE", "> ⛔ **RICHIAMO DEL <data> — le famiglie sono SETTE", 1),
    ("> ⛔ **RICHIAMO DEL 2026-09-11 — la tabella passa da SEI a SETTE", "> ⛔ **RICHIAMO DEL <data> — la tabella passa da SEI a SETTE", 1),
    ("> ⛔ **RICHIAMO DEL 2026-09-11:** i due numeri erano", "> ⛔ **RICHIAMO DEL <data>:** i due numeri erano", 1),
    ("dichiarata qui il 2026-09-11, progettata nella", "dichiarata qui il <data>, progettata nella", 1),
    ("- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/kernel/src/ports/mod.rs` → **almeno 1**",
     "- [ ] `grep -c 'DATED RECALL, <data>' crates/kernel/src/ports/mod.rs` → **almeno 2** (la guardia di `rng` e la riga sotto AUD-054, con la data scritta — D75), e `grep -c '<data>' crates/kernel/src/ports/mod.rs` → **0**", 1),
    ("- [ ] `grep -c 'RICHIAMO DEL 2026-09-11' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` → **3**",
     "- [ ] `grep -c 'RICHIAMO DEL <data>' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` → **3** (con la data scritta), e `grep -c '<data>'` sullo stesso file → **0**", 1),
])

# ------------------------------------------------------------------ task 5
t = scoped(t, "\n## Compito 5:", "\n## Compito 6:", [
    # Files: journal.rs is two regions now (R9a-6/R9b-1), and the design joins (D88)
    ("- Modify: `crates/platform/src/journal.rs` (**`i/lf w/crlf`**) — ⛔ **una parola**: `engine` da privata a `pub(crate)`, con la riga che dice chi la chiama adesso (**D14**)",
     "- Modify: `crates/platform/src/journal.rs` (**`i/lf w/crlf`**) — ⛔ **due regioni**: `engine` da privata a `pub(crate)`, con il capoverso di doc che dice chi la chiama adesso (**D14**), **e** il commento falso della `TableDefinition` riscritto al vero — la voce che la stella polare registrava con chiusore *«il primo compito che tocca questo file»*, ed è questo (R9a-6, R9b-1)\n- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — la voce 10 della §9 chiusa (**D88**)", 1),
    # the second region, after the `engine` block
    ("pub(crate) fn engine(error: impl Into<redb::Error>) -> OpenError {\n```\n\nPoi `crates/platform/src/custody.rs`, **LF**:",
     "pub(crate) fn engine(error: impl Into<redb::Error>) -> OpenError {\n```\n\n"
     "⛔ **E nello stesso file una SECONDA regione, arrivata dalla revisione del piano intero (R9a-6, R9b-1):** il commento\n"
     "della `TableDefinition` dice che `boundary.rs` scrive byte che non sono un `Record`, ed è **falso** — `Untrusted::promote`\n"
     "scrive un `Record::V1` normale, e byte grezzi li scrivono solo i banchi. La stella polare lo registrava con chiusore *«il\n"
     "piano del 2, nel primo compito che tocca `crates/platform/src/journal.rs`»*, e il primo è questo. *Trova* (una riga, unica:\n"
     "`grep -c -F` → 1):\n\n"
     "```\n/// port exchanges BYTES, and `boundary.rs` writes some that are not a `Record` at all), so the\n```\n\n"
     "*Sostituisci con*:\n\n"
     "```\n/// port exchanges BYTES -- and, DATED RECALL <data>, sub-project 2 task 5: the bytes that are NOT\n"
     "/// a `Record` are written ONLY by benches; `boundary.rs` writes an ordinary `Record::V1` through\n"
     "/// `Untrusted::promote`, item 10 of §9 of the sub-project 2 design), so the\n```\n\n"
     "E la voce **10** della §9 del disegno del 2 (LF: Python `newline=\"\"`, temporaneo e `os.replace`): l'ancora è la riga\n"
     "intera che comincia con `| 10 | il commento falso in` (`grep -n -F` → una riga), e in coda alla sua ultima cella si\n"
     "appende ` ✅ **chiusa il <data>, compito 5 del piano della parte 2**`.\n\n"
     "Poi `crates/platform/src/custody.rs`, **LF**:", 1),
    # R2-9: the unused import
    ("use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};",
     "// `ReadableTable` is NOT imported: in redb 4.1.0 `ReadOnlyTable::get` is inherent, so the import\n// would be an unused-import warning at every build. The mutation of step 7 imports it where it\n// needs `iter`, which IS the trait's (measured at the plan review, 2026-09-15).\nuse redb::{Database, ReadableDatabase, TableDefinition};", 1),
    ("        let mut rows = table.iter().map_err(|_| CustodyError::Unavailable)?;",
     "        use redb::ReadableTable; // `iter` is the trait's; the shipped file does not import it\n        let mut rows = table.iter().map_err(|_| CustodyError::Unavailable)?;", 1),
    # R2-10 + D75: the recall on lib.rs
    ("//! ⛔ DATED RECALL, 2026-09-11 -- THE SENTENCE ABOVE CALLED ITSELF \"not a fixed set\" AND IT WAS",
     "//! ⛔ DATED RECALL, <data> -- THE SENTENCE ABOVE CALLED ITSELF \"not written here as a fixed set\" AND IT WAS", 1),
    ("/// ⚠️ `pub(crate)` SINCE 2026-09-11, AND THE SECOND CALLER", "/// ⚠️ `pub(crate)` SINCE <data>, AND THE SECOND CALLER", 1),
    # D76
    ("//! milestone 2 of the sub-project substitutes the WORKING one.", "//! sub-project 2 substitutes the WORKING one.", 1),
    # criteria (R2-8, R9a-6, D75)
    ("- [ ] ⛔ **nessun `with_backend` su `FileCustody`**: `grep -c 'with_backend' crates/platform/src/custody.rs` → **0**",
     "- [ ] ⛔ **nessun `with_backend` su `FileCustody`**: `grep -c 'fn with_backend' crates/platform/src/custody.rs` → **0** — provato dove rende 1: `grep -c 'fn with_backend' crates/platform/src/journal.rs` → **1** (R2-8: il file dettato nomina `with_backend` nel doc e in `create_with_backend`, e il `grep` nudo rendeva **2** sul file stesso)", 1),
    ("- [ ] `grep -c 'pub(crate) fn engine' crates/platform/src/journal.rs` → **1**, e nessun altro tocco a quel file: `git diff --stat -- crates/platform/src/journal.rs` mostra **una sola** regione",
     "- [ ] `grep -c 'pub(crate) fn engine' crates/platform/src/journal.rs` → **1**, `grep -c 'DATED RECALL <data>' crates/platform/src/journal.rs` → **1** (con la data scritta), e nessun altro tocco a quel file: `git diff -U0 -- crates/platform/src/journal.rs | grep -c '^@@'` → **2** regioni, `engine` e il commento della `TableDefinition` (R9a-6)\n- [ ] `grep -c 'chiusa il <data>, compito 5' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **1** (con la data scritta), e `grep -c '<data>'` sui due file → **0**", 1),
    ("- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/platform/src/lib.rs` → **almeno 1**, e il richiamo del compito 2 è **ancora lì**",
     "- [ ] `grep -c 'DATED RECALL, <data>' crates/platform/src/lib.rs` → **1** (con la data di oggi) e `grep -c 'DATED RECALL' crates/platform/src/lib.rs` → **2**: il richiamo del compito 2 è **ancora lì**", 1),
    ("crates/platform/tests/file_custody.rs docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n",
     "crates/platform/tests/file_custody.rs docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n", 1),
])

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: tasks 4 and 5 patched;", t.count("\n") - raw.count("\n"), "lines added")
