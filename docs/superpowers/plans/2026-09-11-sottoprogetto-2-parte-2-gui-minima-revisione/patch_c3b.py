"""patch_c3b.py -- task 3, the thirteen findings of the in-depth review R11 (2026-09-16, Opus 5), plus the ledger.
Every anchor is asserted right before its write (count == 1 inside the task-3 segment, or in the whole plan for the
one line outside it); the two files are written atomically at the end. LF in, LF out.
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
    # R11-11 -- the head: no numeral for the new types, the command instead
    seg = sub(seg,
              "le varianti nuove portano dieci tipi nuovi in `wire::ipc`, e ogni compito che le riempie",
              "le varianti nuove portano in `wire::ipc` un tipo nuovo per ogni tipo del kernel con campi, più il timbro "
              "(quanti lo dice `grep -cE '^pub (struct|enum) '` sul blocco del Passo 2, non un numerale — R11-11, "
              "2026-09-16), e ogni compito che le riempie")
    # R11-1 -- Passo 1: a line anchor, because two `#[test]` live inside comments of the file
    seg = sub(seg,
              "grep -c '#\\[test\\]' crates/kernel/tests/ipc_wire.rs\n",
              "grep -cE '^#\\[test\\]' crates/kernel/tests/ipc_wire.rs\n"
              "# ancora di riga: due `#[test]` del file vivono dentro commenti, e senza `^` il conteggio dice 9 (R11-1, 2026-09-16)\n")
    # R11-2 -- Passo 2: `Millis` wants its import too
    seg = sub(seg,
              "`use alloc::vec::Vec;`, che c'è già. La crate è `no_std` + `alloc`, quindi `String` **non** è nel preludio.\n",
              "`use alloc::vec::Vec;`, che c'è già. La crate è `no_std` + `alloc`, quindi `String` **non** è nel preludio.\n"
              "⛔ **E `Millis` vuole il suo:** `use crate::time::Millis;` accanto a `use crate::arbiter::{ComputeClass, Mib, Preemption};`\n"
              "— lo chiede `Preemption::After(Millis::new(500))` dell'insieme canonico del Passo 3; senza, `cargo build --locked -p kernel`\n"
              "è rosso di `E0433` (R11-2, misurato il 2026-09-16 su una copia del workspace).\n")
    # R11-8 -- Passo 3: the doc of `stamp_set` declares its one exception instead of stating an absolute
    seg = sub(seg,
              "/// are equal and no field is left at its type's default, because a fixture full of zeroes\n"
              "/// cannot tell a field that is written from one that is skipped.\n",
              "/// are equal and no field is left at its type's default, because a fixture full of zeroes\n"
              "/// cannot tell a field that is written from one that is skipped. ⚠️ ONE DECLARED EXCEPTION:\n"
              "/// `DegradationReport::routing_degraded` is `false`, its default -- with two `bool`s this rule\n"
              "/// and P-34's (two equal values at two offsets pin one offset and its mirror) cannot both\n"
              "/// hold, and P-34's wins. Do not \"fix\" it.\n")
    # R11-3 -- Passo 5: no `BuildStamp(0)` from outside the crate; the only constructor there is
    seg = sub(seg,
              "    altered[0] = IpcMessage::Hello(BuildStamp(0));\n",
              "    // `build_stamp()` is the only constructor there is (the doc of `BuildStamp` refuses a `new`),\n"
              "    // and its value differs from the arbitrary one the canonical set carries: that is all this needs.\n"
              "    altered[0] = IpcMessage::Hello(build_stamp());\n")
    # R11-4 -- Passo 5: the `use` line without the four names nothing uses (zero warnings, proved by R11)
    seg = sub(seg,
              "`use kernel::wire::ipc::{build_stamp, stamp_set, Access, BuildStamp,\n"
              "Call, DegradationReport, GrantRequest, IpcMessage, LayoutState, PolicyName, PolicyReport, Protection, Provenance,\n"
              "StepSummary, Triple, Verdict};`.",
              "`use kernel::wire::ipc::{build_stamp, stamp_set, Access, Call,\n"
              "GrantRequest, IpcMessage, LayoutState, PolicyName, Protection, Provenance, Triple, Verdict};`. ⚠️ Né `BuildStamp`\n"
              "né `DegradationReport`, `PolicyReport`, `StepSummary`: nessuna sonda li nomina (i pattern distruggono quei valori senza\n"
              "nominare il tipo), e un `use` di troppo è un avviso che il Passo 4 non ammette (R11-4, zero avvisi misurati il 2026-09-16).")
    # R11-9 -- Passo 6: the comment stops attributing a rule to `gate.sh`
    seg = sub(seg,
              "    // bytes and map cannot drift. `#[ignore]` with a reason, as `scripts/gate.sh` requires of\n"
              "    // every ignored test: the gate must not rewrite artefacts it is checking.\n",
              "    // bytes and map cannot drift. `#[ignore]` with a reason, as every ignored test of this\n"
              "    // workspace does by convention: the gate must not rewrite artefacts it is checking.\n")
    # R11-5 -- Passo 6: the CR count that could not read anything
    seg = sub(seg,
              "tr -cd '\\r' < gui/schema/fixtures/*.json gui/schema/fixtures/ipc_v1.map | wc -c\n",
              "cat gui/schema/fixtures/*.json gui/schema/fixtures/ipc_v1.map | tr -cd '\\r' | wc -c\n"
              "# la forma «tr -cd '\\r' < <glob> <file>» era «ambiguous redirect» e rendeva 0 SEMPRE (R11-5, 2026-09-16);\n"
              "# la controprova nella direzione che deve rendere più di zero:  printf 'a\\r\\nb\\r\\n' | tr -cd '\\r' | wc -c  → 2\n")
    # R11-6 -- Passo 7: the map is checked too, on its last line
    seg = sub(seg,
              "    }\n    let extra: Vec<String> = std::fs::read_dir(&root)\n",
              "    }\n"
              "    // ⛔ THE MAP IS CHECKED TOO, on its last line. D52 makes the SPA read the stamp from there,\n"
              "    // so a stale map is a SPA that sends a stamp the core no longer computes -- `StaleBuild` at\n"
              "    // the handshake -- with this bench GREEN. Measured before this check existed: with mutation\n"
              "    // G8 applied, twelve passed while the map still said the old stamp.\n"
              "    let stamp_line = format!(\"stamp {:#018x}\", build_stamp().get());\n"
              "    match std::fs::read_to_string(root.join(\"ipc_v1.map\")) {\n"
              "        Ok(map) if map.lines().rev().find(|line| !line.trim().is_empty()) == Some(stamp_line.as_str()) => {}\n"
              "        Ok(_) => wrong.push(\"  ipc_v1.map: the last line is not today's stamp\".to_string()),\n"
              "        Err(error) => wrong.push(format!(\"  ipc_v1.map: {error}\")),\n"
              "    }\n"
              "    let extra: Vec<String> = std::fs::read_dir(&root)\n")
    seg = sub(seg,
              "`gui/` per sempre, letta da nessuno e committata da tutti.\n",
              "`gui/` per sempre, letta da nessuno e committata da tutti.\n"
              "⚠️ **E la terza metà è la MAPPA** (R11-6, 2026-09-16): senza il confronto sull'ultima riga, con G8 applicata il banco restava\n"
              "**verde** mentre `ipc_v1.map` diceva un timbro che `build_stamp` non calcola più — e la SPA del 13 legge il timbro\n"
              "proprio da lì (D52). La seconda direzione è **G9** nel Passo 8.\n")
    # R11-10 -- Passo 8: G7 removes the LAST variant, where no index shifts
    seg = sub(seg,
              "| **G7** | togli la riga `IpcMessage::Steps(...)` da `stamp_set` | `every_variant_is_in_the_canonical_set` **rosso** con "
              "`variants missing from stamp_set: [11]`, e il controllo delle fixture rosso su «left over» con **entrambi** i file della "
              "variante tolta, `.bin` e `.json` |\n",
              "| **G7** | togli l'elemento `IpcMessage::Verdict(...)` da `stamp_set` — ⚠️ l'**ultima** variante, non una in mezzo | "
              "`every_variant_is_in_the_canonical_set` **rosso** con `variants missing from stamp_set: [13]`, e il controllo delle fixture "
              "rosso su «left over» con **entrambi** i file della variante tolta, `13-verdict.bin` e `13-verdict.json`. ⚠️ **Perché "
              "l'ultima (R11-10, misurato il 2026-09-16):** togliere `Steps`, in mezzo, fa slittare di uno gli indici di tutte le varianti "
              "dopo — sei file di troppo e quattro mancanti, un Atteso che nessuno può leggere |\n")
    # R11-6 -- Passo 8: G9, the second direction of the map check
    seg = sub(seg,
              "Si **registra** in coda alla voce, non si inventa un caso per farlo scattare |\n",
              "Si **registra** in coda alla voce, non si inventa un caso per farlo scattare |\n"
              "| **G9** | con **G8 ancora applicata** (la seconda direzione del confronto sulla mappa, R11-6), niente altro | "
              "`the_committed_fixtures_match_the_schema` **rosso** con la sola riga `ipc_v1.map: the last line is not today's stamp` "
              "più «REGENERATE them» — i `.bin` e i `.json` restano uguali, perché G8 cambia il timbro e non le codifiche; poi si revoca "
              "G8. ⛔ **Prima di questa riga nulla andava rosso** (misurato il 2026-09-16): la mappa è ciò che D52 consegna alla SPA |\n")
    # R11-12 -- Passo 9 (a): the third half of the recall, and `sub-project 7` (D76)
    seg = sub(seg,
              "//! THE 3D CONSUMER, subproject 7 -- the same closer row 27 of milestone 6 carries in\n",
              "//! THE 3D CONSUMER, sub-project 7 -- the same closer row 27 of milestone 6 carries in\n")
    seg = sub(seg,
              "//! prose, which nothing can go red for.\n//!\n//! ✅ AND THE OTHER HALF DID ARRIVE:",
              "//! prose, which nothing can go red for. ⛔ AND A THIRD HALF, FROM TASK 2 OF THE SAME PLAN:\n"
              "//! the sentence above that `grep -rnE \"^ *impl Ipc for\" crates/` \"returns a bench fake\" is\n"
              "//! false too -- `platform::ipc::LocalSocketIpc` is the real transport now, and the command that\n"
              "//! counts the implementations is the one written there, not this prose.\n"
              "//!\n//! ✅ AND THE OTHER HALF DID ARRIVE:")
    # R11-7 -- Passo 9 (b): the numeral goes, as P-35 prescribes (the last house, in English)
    seg = sub(seg,
              "THE ARGUMENT IS RE-READ RATHER THAN INHERITED. Eleven variants arrived, and they put\n",
              "THE ARGUMENT IS RE-READ RATHER THAN INHERITED. The variants added today put\n")
    # R11-13 -- Passo 10: the commit message names the sub-project
    seg = sub(seg,
              "P-16 (l'innesco della revoca e' il 7)",
              "P-16 (l'innesco della revoca e' il sotto-progetto 7)")
    # Passo 10 -- the criterion counts G9 too
    seg = sub(seg,
              "le tre mutazioni G6, G7, G8 provate **una per volta**",
              "le mutazioni G6, G7, G8 e G9 provate **una per volta**")
    # guards, computed on the segment
    assert "BuildStamp(0)" not in seg
    assert "Eleven variants" not in seg
    assert seg.count("**G9**") == 2  # the table row of Passo 8, and the sentence of Passo 7 that points to it
    return seg


t = scoped(t, "\n## Compito 3:", "\n## Compito 4:", task3)

# R11-7, outside task 3: the cure of P-35 counts the English house too
t = sub(t,
        "dalle otto case (`grep -c undici` le conta),",
        "dalle otto case (`grep -c undici` le conta — e `grep -ci eleven` la nona, in inglese, trovata dalla revisione del "
        "2026-09-16, R11-7),")

lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw
l = sub(l,
        "Mancano: compito 13 (R6), compiti 15–17 (R8), compito 3 (R1), compito 8 (R3) — perimetri da rivedere ancora.",
        "Mancano: compito 13 (R6), compiti 15–17 (R8), compito 8 (R3) — perimetri da rivedere ancora; il compito 3 è rivisto da "
        "R11 il 2026-09-16.")
l = sub(l,
        "resta ⬜ la sola metà dell'8 di R9b-5, con la sua revisione in profondità.\n",
        "resta ⬜ la sola metà dell'8 di R9b-5, con la sua revisione in profondità. **Ondata 14 (2026-09-16):** la revisione in "
        "profondità del 3 è FATTA (R11, un revisore solo su Opus 5: 283k token, 69 comandi, 22 minuti) e i suoi tredici rilievi sono "
        "applicati (✅) con `patch_c3b.py`; il rapporto è `R11-report.md` accanto; resta ⬜ la sola metà dell'8 di R9b-5, con la "
        "revisione in profondità dell'8.\n")
l = sub(l,
        "### Compito 3 — NON RIVISTO IN PROFONDITÀ (R1 caduto): le righe note sono applicate, la revisione in profondità resta da fare\n",
        "### Compito 3 — RIVISTO IN PROFONDITÀ da R11 il 2026-09-16 (Opus 5; R1 era caduto dopo il 2): tredici rilievi, tutti applicati\n")
l = sub(l,
        "D76 «sub-project 2» nei sei punti dei commenti dettati; i due «milestone 6» restano, perché sono il Traguardo 6 del SP1 ✅ · "
        "Attrezzo: `patch_c3.py`\n",
        "D76 «sub-project 2» nei cinque punti dei commenti dettati (il registro diceva «sei»: contati da R11); i due «milestone 6» "
        "restano, perché sono il Traguardo 6 del SP1 ✅ · Attrezzo: `patch_c3.py`\n"
        "- R11-1 Passo 1: `grep -cE '^#\\[test\\]'` — due `#[test]` del file stanno dentro commenti (9 contro 7, rimisurato dal "
        "coordinatore) ✅ · R11-2 Passo 2: `use crate::time::Millis;` con la ragione (`E0433` nella copia) ✅ · R11-3 Passo 5: "
        "`altered[0] = IpcMessage::Hello(build_stamp())` (`E0423`: `BuildStamp(0)` da fuori la crate; nessun `new`, come il Passo 2 "
        "decide) ✅ · R11-4 Passo 5: la riga `use` senza `BuildStamp`, `DegradationReport`, `PolicyReport`, `StepSummary` (zero avvisi, "
        "provato da R11) ✅ · R11-5 Passo 6: `cat … | tr -cd '\\r' | wc -c` con la controprova (la redirezione col glob rendeva 0 "
        "sempre) ✅ · R11-6 Passo 7: la mappa confrontata sull'ultima riga con `format!(\"stamp {:#018x}\", build_stamp().get())`, la "
        "terza metà nella prosa, e **G9** nel Passo 8 come seconda direzione (con G8 attiva prima nulla andava rosso; D52 consegna la "
        "mappa alla SPA) ✅ · R11-7 Passo 9 (b): «The variants added today put» — l'ultima casa di P-35, in inglese; `grep -ci eleven` "
        "accanto a `grep -c undici` nella cura di P-35 ✅ · R11-8 Passo 3: l'eccezione dichiarata nel doc di `stamp_set` "
        "(`routing_degraded` al default, P-34 vince) ✅ · R11-9 Passo 6: il commento non attribuisce più una regola a `gate.sh` ✅ · "
        "R11-10 Passo 8: G7 toglie l'**ultima** variante (`Verdict`, `[13]`, `13-verdict.bin` e `.json`) e non `Steps` in mezzo — la via "
        "(b), nessuno slittamento ✅ · R11-11 testa: via il numerale «dieci tipi», il comando al suo posto ✅ · R11-12 Passo 9 (a): la "
        "terza metà del richiamo («returns a bench fake» è falso dal compito 2) e `sub-project 7` (D76) ✅ · R11-13 Passo 10: «il "
        "sotto-progetto 7» nel messaggio di commit ✅ · Passo 10: il criterio conta anche G9 ✅ · Le P-117… per questi rilievi restano "
        "da scrivere (passo 5 della quindicesima chiusura) · Attrezzo: `patch_c3b.py`\n")

for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 3 patched (R11-1..13) and the ledger;", t.count("\n") - raw.count("\n"), "plan lines added")
