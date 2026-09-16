"""patch_c14b.py -- task 14, the three confirmed findings of R13 (2026-09-16, Opus 5), plus the two P that count
them and the ledger. Task 12 gets no remedy: R13 found nothing on it, only the line that says it is compiled now.

Every anchor is asserted right before its write (count == 1 in the file it belongs to); the two files are written
atomically at the end. LF in, LF out. Usage: python patch_c14b.py [<root>].

The three remedies were measured by the coordinator on R13's own compiled model before being written here
(C:\\Users\\zagor\\AppData\\Local\\Temp\\probe-R13\\gui, node_modules from probe-R6, every source rewritten from
today's plan), in BOTH directions:
  * with the plan's text, `npx vitest run` gives exactly three red `it` with the messages the report quotes --
    "expected '<p>vedi <span class=\"link\" data-href=...' not to contain 'href='" and two
    "expected <div data-v-e4bfeb46 ...> to be null";
  * with the three remedies the whole suite is green: `Test Files 14 passed | 1 skipped`, `Tests 81 passed |
    1 skipped`.
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = sys.argv[1] if len(sys.argv) > 1 else r"C:\Users\zagor\Desktop\harness"
PLAN = os.path.join(ROOT, "docs", "superpowers", "plans", "2026-09-11-sottoprogetto-2-parte-2-gui-minima.md")
LEDGER = os.path.join(ROOT, "docs", "superpowers", "plans",
                      "2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione", "ledger.md")

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw, "the plan is LF: something rewrote it"
t = raw
rawl = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in rawl, "the ledger is LF: something rewrote it"
g = rawl


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


# ---- R13-1: task 14, step 7 -- the space in `" href="` is load-bearing ------------------------------------------------
t = sub(t,
        '    expect(html).not.toContain("href=");\n',
        '    // the SPACE is load-bearing: `data-href=` contains `href=` (R13-1).\n'
        '    expect(html).not.toContain(" href=");\n')

R13_1 = "\n".join([
    "\u26d4 **Lo spazio in `\" href=\"` \u00e8 PORTANTE, e non si toglie: `data-href=` contiene `href=`.** Senza di esso le due",
    "asserzioni dello **stesso** `it` si contraddicono \u2014 `not.toContain(\"href=\")` e `toContain('data-href=\"\u2026\"')` non",
    "possono essere vere insieme \u2014 e quell'`it` \u00e8 rosso **qualunque cosa faccia `renderMarkdown`**: un oracolo che non",
    "pu\u00f2 mai diventare verde, cio\u00e8 il contrario di ci\u00f2 per cui esiste. \u2705 **Misurato il 2026-09-16 (R13, e rifatto dal",
    "coordinatore nelle due direzioni sul modello compilato):** col testo di prima",
    "`npx vitest run src/components/markdown.test.ts` rende un rosso che dice `not to contain 'href='` sul reso",
    "`<p>vedi <span class=\"link\" data-href=\"https://example.com/x\">qui</span></p>`; con lo spazio, **5/5 verdi**. Lo",
    "spazio c'\u00e8 perch\u00e9 `<a href=` lo porta e `data-href=` porta un **trattino**.",
    "", ""])
t = sub(t,
        '```\n\n\u26a0\ufe0f **`import MarkdownIt from "markdown-it"`',
        '```\n\n' + R13_1 + '\u26a0\ufe0f **`import MarkdownIt from "markdown-it"`')

# ---- R13-2 and R13-3: task 14, step 13 -- three ticks, not one --------------------------------------------------------
TICKS = ("    // reka-ui unmounts DialogContent through its own dismissable layer: THREE ticks, measured (R13-2).\n"
         "    await nextTick();\n    await nextTick();\n    await nextTick();\n")
t = sub(t,
        "    (yes as HTMLButtonElement).click();\n    await nextTick();\n",
        "    (yes as HTMLButtonElement).click();\n" + TICKS)
t = sub(t,
        "    (no as HTMLButtonElement).click();\n    await nextTick();\n",
        "    (no as HTMLButtonElement).click();\n" + TICKS.replace("(R13-2)", "(R13-3)"))

R13_23 = "\n".join([
    "\u26d4 **I TRE `await nextTick()` dopo ciascun click sono MISURATI, e non sono uno:** `reka-ui` 2.10.4 smonta",
    "`DialogContent` attraverso il proprio strato *dismissable*, che costa due giri in pi\u00f9. \u2705 **Misurato il 2026-09-16",
    "(R13, con una sonda usa-e-getta, e rifatto dal coordinatore nelle due direzioni sul modello compilato):** dopo",
    "**uno** e dopo **due** `nextTick` il nodo `.confirm` \u00e8 ancora nel DOM con `data-state=\"closed\"`, al **terzo** \u00e8 via.",
    "Con un solo `nextTick` i due `it` sono rossi con `AssertionError: expected <div data-v-\u2026> to be null`. \u26a0\ufe0f **E il",
    "rosso si leggerebbe come \u00abla finestra non si chiude\u00bb, mentre la logica \u00e8 giusta:** `core.pending` \u00e8 gi\u00e0 `null` e",
    "l'`Approve` \u00e8 gi\u00e0 partito \u2014 le due righe sopra lo asseriscono e passano \u2014 \u00e8 la **sonda** a essere sotto-attesa,",
    "non il modulo a sbagliare.",
    "", ""])
t = sub(t,
        '```\n\n\u26a0\ufe0f **La tripla che le sonde confrontano',
        '```\n\n' + R13_23 + '\u26a0\ufe0f **La tripla che le sonde confrontano')

# ---- P-128: task 12 -- R13 compiled it and found nothing; the number does NOT move ------------------------------------
t = sub(t,
        "12 \u00e8 stato rivisto da **R3** (2026-09-15), **R5** (2026-09-15), **R9a** (2026-09-15), **R10** (2026-09-15). "
        "Ogni rilievo\nsta nella sezione \u00abRilievi\u00bb del proprio rapporto, in",
        "12 \u00e8 stato rivisto da **R3** (2026-09-15), **R5** (2026-09-15), **R9a** (2026-09-15), **R10** (2026-09-15) "
        "\u2014 e il suo\nmodulo di sonde \u00e8 stato **compilato** da **R13** il 2026-09-16, che sul 12 non ha trovato "
        "**nulla**, quindi il numero\nqui sotto non si muove. Ogni rilievo sta nella sezione \u00abRilievi\u00bb del proprio "
        "rapporto, in")

# ---- P-130: task 14 -- R13's three, and the command that can see its rows ---------------------------------------------
t = sub(t,
        "14 \u00e8 stato rivisto da **R7** (2026-09-15), **R10** (2026-09-15). Ogni rilievo sta nella sezione \u00abRilievi\u00bb "
        "del proprio\nrapporto, in `docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/`, "
        "e il rimedio applicato",
        "14 \u00e8 stato rivisto da **R7** (2026-09-15), **R10** (2026-09-15) \u2014 e i suoi moduli riscritti sono stati "
        "**compilati** da\n**R13** il 2026-09-16, un revisore solo su Opus 5. Ogni rilievo sta nella sezione "
        "\u00abRilievi\u00bb del proprio rapporto, in\n"
        "`docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/`, e il rimedio applicato")
t = sub(t,
        "grep -E '^\\| R[0-9]+[ab]?-[0-9]+ \\| (Compito )?14\\b'",
        "grep -E '^\\| (R[0-9]+[ab]?-[0-9]+ \\| (Compito )?14\\b|R13-[0-9]+ \\|)'")
t = sub(t,
        "\u2192 **7** il 2026-09-16: le righe \u00abCONFERMATO \u00b7 fatto\u00bb il cui primo compito nominato \u00e8 il 14; una riga che ne "
        "nomina pi\u00f9\nd'uno conta per il primo.",
        "\u2192 **10** il 2026-09-16: le righe \u00abCONFERMATO \u00b7 fatto\u00bb il cui primo compito nominato \u00e8 il 14; una riga che ne "
        "nomina pi\u00f9\nd'uno conta per il primo; \u26a0\ufe0f **le celle di R13 portano il numero in grassetto** (`**14**`) e "
        "l'alternanza nuda non\nle vedeva \u2014 il comando le prende **per prefisso**, come fa quello di P-129 con R6, "
        "perch\u00e9 R13 \u00e8 tutto sui compiti\n12 e 14 e sul 12 non ha trovato nulla. Era **7** prima di R13.")

# ---- the ledger ------------------------------------------------------------------------------------------------------
g = sub(g,
        "il Passo 15-bis del 14. Nel registro non resta nessun \u2b1c.",
        "il Passo 15-bis del 14. Nel registro non resta nessun \u2b1c. **Ondata 18 (2026-09-16):** la **compilazione** dei "
        "moduli riscritti del **12** e del **14** \u00e8 FATTA (R13, un revisore solo su Opus 5: ~271k token, 99 chiamate, "
        "~27 minuti) e i suoi **tre** rilievi confermati \u2014 tutti sul 14, tutti bloccanti \u2014 sono applicati (\u2705) con "
        "`patch_c14b.py`; il rapporto \u00e8 `R13-report.md` accanto. Il **12 non ha rilievi**. Le tre misure sono state "
        "rifatte dal coordinatore sul modello compilato di R13 nelle due direzioni prima di dettare l'ondata "
        "(decisione 95). Nel registro non resta nessun \u2b1c, e la revisione in profondit\u00e0 \u00e8 **chiusa**.")

g = sub(g,
        "- \u26a0\ufe0f **Non compilato:** il modulo delle sonde \u00e8 riscritto sul modello letto, non eseguito (il compito 2 e il "
        "7 non esistono ancora nel repo): chi esegue il 12 lo compila per primo e ogni rosso \u00e8 una voce d'errata, come "
        "dice \u00abCome si esegue\u00bb \u2014 il revisore R5 aveva compilato la forma vecchia",
        "- \u2705 **Provato il 2026-09-16, e nessun rilievo (R13):** `main.rs` ricomposto dai quattro blocchi Rust dei Passi "
        "5\u20138 (655 righe) si **parsa**, e i pezzi che si possono compilare senza i compiti 1\u20137 sono stati compilati **e "
        "girati** in una crate usa-e-getta contro `kernel`, `platform` e `simulator` di `e851b5d` \u2014 `build_the_arbiter` "
        "rende `Mib(1792)` = 1024+768, **esattamente il letterale che la sonda asserisce** (R5-7); `SharedClock` sul "
        "`SystemReactor`, `note_a_degraded_routing` su `MemoryJournal` e il pari sul tubo **verbatim** su `interprocess` "
        "2.4.4 compilano senza avvisi; `WELCOME` = 5 combacia col `greet` del 7 e col 9, `SOCKET_NAME` e `MAX_BODY` col 9 "
        "(il `diff` di **D45** uscirebbe 0); i cinque criteri di chiusura meccanici reggono, **incluse le due direzioni** "
        "di R5-5. \u26a0\ufe0f **Il 12 per intero resta incompilabile** finch\u00e9 i compiti 1\u20137 non esistono: chi lo esegue \u00e8 "
        "sempre il primo a vederlo intero. Qui stava \u00abNon compilato \u2026 chi esegue il 12 lo compila per primo\u00bb")

g = sub(g,
        "- \u26a0\ufe0f **Non compilato:** `Frame.vue`, `Chat.vue`, `markdown.ts` e le sonde toccate sono riscritti sul modello "
        "letto; chi esegue il 14 li compila per primi, ogni rosso \u00e8 errata \u2014 come il 12 e il 13",
        "- \u2705 **Compilato il 2026-09-16 (R13):** `npx vue-tsc --noEmit` esce **0 senza una riga** su tutti i file del 13 "
        "e del 14 riscritti dal piano di oggi, con le sedici dipendenze alle versioni appuntate; le tre viste del 13 si "
        "generano e `dockview` **monta sotto `jsdom`**. Le sue sonde erano **rosse in tre `it`** \u2014 i tre rilievi qui "
        "sotto \u2014 e coi rimedi la suite \u00e8 verde: `Test Files 14 passed \\| 1 skipped`, `Tests 81 passed \\| 1 skipped`. "
        "Qui stava \u00abNon compilato \u2026 chi esegue il 14 li compila per primi\u00bb")

R13_ROWS = (
    "\n- R13-1 (bloccante) Passo 7, `markdown.test.ts`: `expect(html).not.toContain(\" href=\");` con lo **spazio** in "
    "testa, il commento che lo dichiara portante e il capoverso che porta la misura \u2014 `data-href=` **contiene** "
    "`href=`, quindi le due asserzioni dello stesso `it` si contraddicevano e l'`it` era rosso **sempre**, qualunque "
    "cosa facesse `renderMarkdown`; rimisurato dal coordinatore nelle due direzioni sul modello compilato di R13 (col "
    "testo di prima un rosso col messaggio del rapporto; con lo spazio 5/5) \u2705\n"
    "- R13-2 e R13-3 (bloccanti) Passo 13, `modules.test.ts`: **tre** `await nextTick();` dopo ciascuno dei due click "
    "della finestra di conferma invece di uno, col commento e col capoverso che porta la misura \u2014 `reka-ui` 2.10.4 "
    "smonta `DialogContent` attraverso il proprio strato *dismissable* (dopo uno e dopo due giri il nodo \u00e8 ancora l\u00ec "
    "con `data-state=\"closed\"`, al terzo \u00e8 via); \u26a0\ufe0f **la logica era giusta** \u2014 `core.pending` \u00e8 `null` e l'`Approve` "
    "\u00e8 partito \u2014 era la **sonda** a essere sotto-attesa \u2705\n"
    "- \u26a0\ufe0f **Nessuna D nuova, ed \u00e8 deliberato:** nessuno dei tre rimedi \u00e8 una decisione \u2014 lo spazio lo impone la "
    "sottostringa, i tre giri li impone `reka-ui` \u2014 e una D che registra un fatto costretto \u00e8 una casa in pi\u00f9 "
    "(gotcha #68)\n"
    "- **P-130** passa da **7** a **10**, e il suo comando si allarga per vedere le celle di R13, che portano il numero "
    "di compito **in grassetto** (la forma di P-129 con R6). **P-128** resta **13**: sul 12 R13 non ha trovato nulla\n"
    "- Attrezzo: `patch_c14b.py`\n")
g = sub(g, "\n### Compito 15 \u2014 RIVISTO IN PROFONDIT\u00c0", R13_ROWS + "\n### Compito 15 \u2014 RIVISTO IN PROFONDIT\u00c0")

# ---- guards, counted on the text this script produced -----------------------------------------------------------------
assert t.count('expect(html).not.toContain(" href=");') == 1, "the space went missing"
assert t.count('expect(html).not.toContain("href=");') == 0, "the old line survived"
assert t.count("    await nextTick();\n    await nextTick();\n    await nextTick();\n") == 2, "not two triples"
# NOT a remembered count: P-122 (task 6) already reads "-> **10**", so the guard is the DELTA, measured.
assert t.count("\u2192 **10** il 2026-09-16") == raw.count("\u2192 **10** il 2026-09-16") + 1, "P-130 did not become 10"
assert t.count("\u2192 **7** il 2026-09-16") == raw.count("\u2192 **7** il 2026-09-16") - 1, "the old 7 survived"
assert t.count("le celle di R13 portano il numero in grassetto") == 1, "the why of the widened command is missing"
assert t.count("R13-[0-9]+ \\|)'") == 1, "the widened command is not there"
assert t.count("### P-") == raw.count("### P-"), "a P was added or lost"
assert t.count("| **D") == raw.count("| **D"), "a D row was added or lost"
assert t.count("## Compito") == raw.count("## Compito"), "a task was added or lost"
assert t.count("\r") == 0 and g.count("\r") == 0, "a CR crept in"
assert g.count("R13-1 (bloccante)") == 1 and g.count("R13-2 e R13-3 (bloccanti)") == 1, "ledger rows"
assert g.count("\u26a0\ufe0f **Non compilato:**") == 0, "a 'non compilato' line survived"

for path, text in ((PLAN, t), (LEDGER, g)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(text)
    os.replace(tmp, path)
print("ok: plan", raw.count("\n") + 1, "->", t.count("\n") + 1, "lines; ledger",
      rawl.count("\n") + 1, "->", g.count("\n") + 1, "lines")
