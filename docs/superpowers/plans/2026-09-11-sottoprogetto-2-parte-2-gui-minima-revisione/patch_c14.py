"""patch_c14.py -- task 14 (R7-1, R7-2, R7-3, R7-4, R7-5, R7-6, R7-8, R7-9, R7-10, R7-12, R7-13, R7-14, R7-26, R10-8,
R9b-4/R9a-13 = D87, R9b-5's half of task 14, D76), the D80/D89 cascade from the corrected task 13 onto the re-dictated
`Frame.vue` (decision 78), R7-6 applied to task 13's identical criterion, the D87 row of the plan's D table, and the
ledger (task 14 whole, task 8's half of R9b-5, task 13's extra row, the state line).
Scoped, every anchor asserted right before its write, atomic.

⚠️ Passo 15 is rewritten whole (decision 77): nine recalls in the north star, and the anchors are taken FROM THE FILE
by section and line prefix (E6 of the gestures plan, R7-26) instead of nine 900-character rows copied into the plan.
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


def between(seg, start, end, new):
    cs, ce = seg.count(start), seg.count(end)
    assert cs == 1, f"start marker found {cs} times: {start[:90]!r}"
    assert ce == 1, f"end marker found {ce} times: {end[:90]!r}"
    lo, hi = seg.index(start), seg.index(end)
    assert lo < hi, "markers out of order"
    return seg[:lo] + new + seg[hi:]


def replace_line(seg, prefix, new_line):
    """Replace the ONE line of seg that starts with `prefix` (new_line without its newline)."""
    hits = [i for i, line in enumerate(seg.split("\n")) if line.startswith(prefix)]
    assert len(hits) == 1, f"line prefix found {len(hits)} times: {prefix[:90]!r}"
    lines = seg.split("\n")
    lines[hits[0]] = new_line
    return "\n".join(lines)


def scoped(text, start_marker, end_marker, fn):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    return text[:lo] + fn(text[lo:hi]) + text[hi:]


EMPTY_IT_OLD = r"""- [ ] ⛔ **nessuna sonda col corpo vuoto:** `grep -cE '^\s*(it|describe)\([^)]*\(\) => \{\}\)' gui/src/**/*.test.ts` → **0**
"""
EMPTY_IT_NEW = r"""- [ ] ⛔ **nessuna sonda col corpo vuoto:** `grep -rcE '^\s*(it|describe)\([^)]*\(\) => \{\}\)' gui/src --include='*.test.ts' | grep -v ':0$'` → **niente** — R7-6, misurato: senza `globstar` il `**` non scende di un livello (`gui/src/a11y.test.ts` sfuggiva) e con più file `grep -c` stampa `file:conteggio`, mai uno «0» nudo; provato una volta anche su un file con un corpo vuoto, che **deve** comparire
"""

# ============================================================================================ the rewritten blocks

CHAT_WATCH_OLD = r"""watch(
  () => stream.current?.text,
  (text) => {
    const render = (): void => {
      frame = 0;
      currentHtml.value = text === undefined ? "" : renderMarkdown(text);
    };
"""
CHAT_WATCH_NEW = r"""watch(
  () => stream.current?.text,
  () => {
    // ⛔ THE TEXT IS READ WHEN THE FRAME RUNS, not captured when the frame was scheduled (R7-2):
    // the tokens that land in the same frame would otherwise render the first and hold the rest
    // until the next token. SP-8's tile read the current text, and that is the merit that rises.
    const render = (): void => {
      frame = 0;
      const text = stream.current?.text;
      currentHtml.value = text === undefined ? "" : renderMarkdown(text);
    };
"""

CHAT_PROBE_NEW = r"""  it("renders the text as it stands when the frame runs, not the first token of the frame", async () => {
    const stream = useStream();
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    stream.receive({ kind: "Token", text: "primo ", provenance: "Untrusted" });
    stream.receive({ kind: "Token", text: "secondo", provenance: "Untrusted" });
    await frame();
    // ⛔ BOTH, in one frame (R7-2): a render that captured the text at scheduling time showed
    // "primo" alone until the next token arrived -- and no probe delivered two in one frame.
    expect(wrapper.get("article.streaming .body").html()).toContain("<p>primo secondo</p>");
  });

"""

MODULES_PROBE_OLD = r"""    registerModules();
    for (const name of Object.keys(MODULES)) expect(typeof componentFor(name), name).toBe("function");
    // ⛔ THE SECOND DIRECTION: a module type nobody built is still a placeholder, not a crash and
    // not a real module -- otherwise plugging five in would have to be checked in the browser.
    const unbuilt = PANEL_TYPES.filter((type) => !(type.name in MODULES));
    expect(unbuilt.length).toBeGreaterThan(0);
    for (const type of unbuilt) expect(typeof componentFor(type.name), type.name).toBe("function");
"""
MODULES_PROBE_NEW = r"""    registerModules();
    // ⛔ WHAT TELLS A BUILT MODULE FROM A PLACEHOLDER IS WHAT IT DRAWS (R7-10): `componentFor`
    // answers a function in both cases, so the renderer is mounted and read. A placeholder says
    // who fills it; a real module never does.
    const drawn = (name: string): string => {
      const renderer = componentFor(name)();
      renderer.init({ api: { id: name }, containerApi: {}, params: placeholderParams(name), title: name } as never);
      const html = renderer.element.innerHTML;
      renderer.dispose?.();
      return html;
    };
    for (const name of Object.keys(MODULES)) expect(drawn(name), name).not.toContain("placeholder");
    // ⛔ THE SECOND DIRECTION: a module type nobody built is still the placeholder, saying who
    // fills it -- otherwise plugging five in would have to be checked in the browser.
    const unbuilt = PANEL_TYPES.filter((type) => !(type.name in MODULES));
    expect(unbuilt.length).toBeGreaterThan(0);
    for (const type of unbuilt) expect(drawn(type.name), type.name).toContain('class="placeholder"');
"""

FRAME14_SWITCH_OLD = r"""function switchTo(view: ViewName): void {
  layout.view = view;
  // ⛔ A VIEW CHANGE IS A LAYOUT CHANGE, and it goes through the same path: `apply` prefers the
  // saved package and falls back to the shipped view, so switching to a view the owner has saved
  // shows THEIR version and not ours (row 6 of §2).
  if (api !== null) apply(api, view, unpack(layout.state));
}
"""
FRAME14_SWITCH_NEW = r"""function switchTo(view: ViewName): void {
  // ⛔ ONE LINE, AND THE DOCK FOLLOWS (D89, task 13): the open view lives in the store and
  // `createDock` watches it, so the bar, the keyboard above and a package from the core all take
  // the same path -- and none of them saves a view for merely showing it (decision 11).
  layout.view = view;
}
"""

PASSO15_NEW = r'''- [ ] **Passo 15: i richiami datati nella stella polare — la riga 1 di Passi (D56), le righe 3 di Permessi e 12 di Passi (R7-14), uno per modulo costruito (D87), e la riga delle registrate (R9b-5)**

⛔ **P-89, D56:** la riga 1 della tabella *Passi* della §1 promette per il 2 *«funzione, invocatore, argomento, classe
dell'effetto, esito»*, e il filo porta **numero del passo, funzione, esito** — la §4 del 2, riscritta il 2026-09-09,
dice *«con intento ed esito»*, e il compito 3 l'ha seguita. Il richiamo va **dove la riga vive**, e lo scrive questo
compito perché è quello che la rende visibile.

⛔ **E non è l'unico richiamo che questo compito deve alla stella — arrivati dalla revisione del piano intero (R7-14,
R9b-4, R9a-13, R9b-5; D87):** la riga **3 di Permessi** promette per il 2 *«la classe dell'effetto, chi la invoca»* e la
riga **12 di Passi** *«comando: il replay»*, e nel 2 sono **costanti** — un invocatore, una funzione, una classe — e il
replay è la lista ordinata con l'esito, così com'è; la regola della §6 della stella — *«quando **costruisce** un modulo,
mette un richiamo datato nella riga di quel modulo nella §1»* — vuole un richiamo per **ciascuno** dei cinque moduli
costruiti, col nome del sorgente, e la «riga di quel modulo» è il capoverso *«Costruito dal **2**»* sotto
l'intestazione per le quattro tabelle piene e la riga *Impostazioni* per la corta; e la riga *«le scorciatoie da
tastiera … sopra `moveTo`»* delle **Registrate, non prese** ha chiusore *«il piano del 2, con G20»*, cioè questo
compito, e riceve il suo «✅ chiusa» come le righe già chiuse della stessa tabella.

⛔ **Le ancore si prendono DAL FILE, non da questo piano** (E6 del piano dei gesti; R7-26): lo script trova ogni riga per
**sezione e inizio di riga** — o, per un capoverso, per la frase che contiene — pretende che sia **una**, e appende il
richiamo **in coda**: una riga di tabella resta una riga, un capoverso resta un capoverso. Il file è **LF** (Passo 1);
si tocca con Python, mai con `sed -i`. ⚠️ **`<data>` si sostituisce con la data del giorno in cui il compito si esegue,
nello script E nei `grep` qui sotto** — altrimenti i `grep` rendono 0 (R7-26).

```bash
python - <<'EOF'
import io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8")
p = "docs/superpowers/specs/2026-09-07-direzione-gui-design.md"
b = io.open(p, encoding="utf-8", newline="").read()
assert "\r\n" not in b, "the north star is LF: something rewrote it"
lines = b.split("\n")
DATE = "<data>"


def section(heading):
    """The line indexes under the ONE heading that starts with `heading`, up to the next heading of
    the same or a higher level."""
    starts = [i for i, line in enumerate(lines) if line.startswith(heading)]
    assert len(starts) == 1, f"{heading!r}: {len(starts)} headings"
    level = len(lines[starts[0]].split(" ")[0])
    end = next((i for i in range(starts[0] + 1, len(lines)) if re.match(r"^#{1,%d} " % level, lines[i])), len(lines))
    return range(starts[0], end)


def one(heading, predicate):
    hits = [i for i in section(heading) if predicate(lines[i])]
    assert len(hits) == 1, f"{heading!r}: {len(hits)} lines match"
    return hits[0]


def row(heading, prefix, recall):
    """Appends `recall` inside the LAST cell of the one row of `heading` that starts with `prefix`."""
    i = one(heading, lambda line: line.startswith(prefix))
    assert lines[i].endswith(" |"), lines[i][-60:]
    lines[i] = lines[i][:-2] + " " + recall + " |"


def paragraph(heading, needle, recall):
    """Appends `recall` to the END of the one paragraph of `heading` that contains `needle`."""
    i = one(heading, lambda line: needle in line)
    while i + 1 < len(lines) and lines[i + 1] != "":
        i += 1
    lines[i] = lines[i] + " " + recall


RECALL = f"✅ **RICHIAMO DEL {DATE}, dal compito 14 del piano della parte 2"
BUILT = f"✅ **Costruito dal compito 14 del piano della parte 2, {DATE}:"
LESS = "le righe con «2» nella colonna «chi»; dove il 2 porta meno di quanto una riga promette, la riga lo dice col proprio richiamo"

row("#### Passi", "| 1 |", RECALL + " (P-89, D56):** sul filo la lista porta **numero del passo, funzione, esito** — "
    "`StepSummary` del compito 3, un riassunto e non il record (I4, ADR-0036), come la §4 del 2 dice dal 2026-09-09 "
    "(*«con intento ed esito»*). Nel 2 l'invocatore e la classe dell'effetto sono **costanti** — un solo invocatore, una "
    "sola funzione — e l'argomento è payload non fidato che il riassunto non porta: il modulo mostra i tre e dice a "
    "parole che il resto arriva quando il riassunto crescerà, col timbro")
row("#### Permessi", "| 3 |", RECALL + " (R7-14):** nel 2 la classe dell'effetto e chi invoca sono **costanti** — una "
    "funzione sola, `Idempotent`, e l'invocatore è il click di questa finestra — e il filo non li porta "
    "(`PermissionRequired(Triple)`, compito 3): il modulo mostra la tripla a parole e la finestra la chiamata in volo, e "
    "i due arrivano col messaggio che crescerà. Il livello di confinamento resta dedotto, com'era")
row("#### Passi", "| 12 |", RECALL + " (R7-14):** nel 2 «il replay» è la lista ordinata dei passi con l'esito, che il "
    "modulo mostra così com'è; un comando che scorra una run arriva col 3, con le run")
paragraph("#### Chat", "Costruito dal **2**", BUILT + " `gui/src/panels/Chat.vue`** sul flusso del core finto "
          "(`gui/fake-core/`, compito 12), con `gui/src/components/markdown.ts` — " + LESS)
paragraph("#### Stato", "Costruito dal **2**", BUILT + " `gui/src/panels/Status.vue`** — " + LESS)
paragraph("#### Permessi", "Costruito dal", BUILT + " `gui/src/panels/Permissions.vue`**, e la finestra di conferma in "
          "`gui/src/components/Confirm.vue` — " + LESS + " (riga 3)")
paragraph("#### Passi", "Costruito dal **2**", BUILT + " `gui/src/panels/Steps.vue`** — " + LESS + " (righe 1 e 12)")
row("#### Gli altri tredici moduli", "| **Impostazioni** |", BUILT + " `gui/src/panels/Settings.vue`** — nel 2 il solo "
    "cambio di policy VRAM (P-85), il primo invocatore del registro di ADR-0038")
row("## Registrate, non prese", "| le scorciatoie da tastiera per spostare un pannello", "— ✅ **chiusa il " + DATE +
    ", compito 14 del piano della parte 2**: `gui/src/frame/moveActive.ts`, `Ctrl+Alt+freccia` sopra `moveTo`, "
    "con la sonda `keys.test.ts`")

out = "\n".join(lines)
assert out.count("\n") == b.count("\n"), "a line was added or lost: every recall goes IN a line"
tmp = p + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(out)
os.replace(tmp, p)
print("ok: nine recalls in", p)
EOF
grep -c 'RICHIAMO DEL <data>, dal compito 14' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
grep -c 'Costruito dal compito 14 del piano della parte 2, <data>' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
grep -c 'chiusa il <data>, compito 14 del piano della parte 2' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
tr -cd '\r' < docs/superpowers/specs/2026-09-07-direzione-gui-design.md | wc -c
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
```

Atteso (con la data scritta al posto di `<data>`, **anche nei `grep`**): **3**, **5**, **1**, **0**, e niente.

⛔ **Delle righe di Passi ricevono il richiamo la 1 e la 12, benché la riga 2 dica *«con la classe dell'effetto»* e la
riga 3 *«il dettaglio del passo secondo la specie»*:** la riga 2 parla di ciò che la lista **mostra** di un passo in
dubbio, e il modulo lo mostra (`in dubbio`); la riga 3 è il dettaglio, che il riassunto non porta per **la stessa**
ragione della riga 1 e che il richiamo della riga 1 copre nominando il riassunto. Un richiamo per **fatto**, non per
riga che lo sfiora (gotcha #68).
'''


# ============================================================================================ task 14
def task14(seg):
    # Files (R7-9, R10-8)
    seg = sub(seg, "- Modify: `gui/src/tokens/tokens.css` (**LF**) — `--stop`, l'unico token che l'AA boccia (**P-86**)\n",
              "- Modify: `gui/src/tokens/tokens.css` (**LF**) — `--stop`, l'unico token che l'AA boccia (**P-86**), il capoverso in testa (Passo 3) e le regole della linguetta in coda (Passo 11) — R7-9\n")
    seg = sub(seg, "- Modify: `gui/src/main.ts` (**LF**) — i moduli registrati, gli store nuovi in ascolto, la finta esposta al revisore (**D57**)\n",
              "- Modify: `gui/src/main.ts` (**LF**) — i moduli registrati, gli store nuovi in ascolto, la finta esposta al revisore (**D57**)\n"
              "- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — i richiami datati della §1 e delle registrate: la riga 1 di Passi (**D56**), le righe 3 di Permessi e 12 di Passi (R7-14), uno per modulo costruito (**D87**), la riga delle scorciatoie (R9b-5) — Passo 15 (R10-8: mancava dalla lista mentre il Passo 15 e il commit la toccavano)\n")
    # Passo 2 (R7-13)
    seg = sub(seg, "types={m.get('types') or m.get('exports', {})", "types={m.get('types') or m.get('typings') or m.get('exports', {})")
    seg = sub(seg, "⚠️ **Nessun `@types/markdown-it`:** la 15 spedisce i propri tipi — `dist/markdown-it.d.mts`\nsotto `exports[\".\"].import.types`, misurato il 2026-09-15 — e `axe-core` spedisce `axe.d.ts`.",
              "⚠️ **Nessun `@types/markdown-it`:** la 15 spedisce i propri tipi — `types` di primo livello `./dist/markdown-it.d.cts`, e\n`./dist/markdown-it.d.mts` sotto `exports[\".\"].import.types`, misurato il 2026-09-15 (R7-13: lo script stampa il primo, perché\n`types` di primo livello vince) — e `axe-core` li dichiara in `typings: axe.d.ts`, che lo script legge come seconda scelta.")
    # Passo 3 (R7-5, R7-8)
    seg = sub(seg, "Le tre righe che cominciano con `⚠️ G20 WANTS AA CONTRAST` diventano:",
              "Le **quattro** righe, da `⚠️ G20 WANTS AA CONTRAST` fino a `reason the shape comes before the palette. */` comprese —\n`grep -c -F 'reason the shape comes before the palette. */' gui/src/tokens/tokens.css` → **1** prima di scrivere (R7-5: qui\nstava «tre», e la quarta sarebbe rimasta orfana fuori dal commento, davanti a `:root`) — diventano:")
    seg = sub(seg, "npx vitest run src/tokens/contrast.test.ts; echo \"verde atteso: EXIT=$?\"\npython - <<'EOF'\nimport io\np = \"src/tokens/tokens.css\"",
              "npx vitest run src/tokens/contrast.test.ts; echo \"verde atteso: EXIT=$?\"\ngit add src/tokens/tokens.css\npython - <<'EOF'\nimport io\np = \"src/tokens/tokens.css\"")
    seg = sub(seg, "⚠️ **`git checkout` rimette il file com'era nell'INDICE**, cioè senza la modifica di questo passo se non è ancora\nstata aggiunta: si rilancia il primo script di questo passo dopo il rosso, o si fa il `git add` prima della mutazione.\nAtteso alla fine:",
              "⚠️ **`git checkout` rimette il file com'era nell'INDICE**, ed è per questo che il `git add` sta **prima** della mutazione\n(R7-8, la forma di R5-11): senza, il `checkout` riporterebbe il file al 13 — e «si rilancia il primo script» non reggeva,\nperché il solo script di questo passo è la mutazione **inversa**, che dopo il `checkout` fallirebbe sul proprio `assert`.\nAtteso alla fine:")
    # Passo 7 (R7-1)
    seg = sub(seg, "md.renderer.rules.link_open = (tokens, index) => {\n  const href = tokens[index]?.attrGet(\"href\") ?? \"\";\n",
              "md.renderer.rules.link_open = (tokens, index) => {\n  // `attrGet` is typed `string | number | null` in the shipped `.d.mts` (R7-1): `String` narrows it.\n  const href = String(tokens[index]?.attrGet(\"href\") ?? \"\");\n")
    seg = sub(seg, "  const src = md.utils.escapeHtml(token?.attrGet(\"src\") ?? \"\");\n",
              "  const src = md.utils.escapeHtml(String(token?.attrGet(\"src\") ?? \"\"));\n")
    # Passo 9 (R7-2) and its probe
    seg = sub(seg, CHAT_WATCH_OLD, CHAT_WATCH_NEW)
    seg = sub(seg, "\n  it(\"freezes a block at the threshold and keeps its provenance label, as SP-8's tile did\", async () => {\n",
              "\n" + CHAT_PROBE_NEW + "  it(\"freezes a block at the threshold and keeps its provenance label, as SP-8's tile did\", async () => {\n")
    # Passo 10, Frame.vue: the D80/D89 cascade from task 13 (decision 78)
    seg = sub(seg, "`gui/src/frame/Frame.vue`, **LF** — **modificato**: la finestra dopo la fascia, e la tastiera del Passo 11.\n",
              "`gui/src/frame/Frame.vue`, **LF** — **modificato**: la finestra dopo la fascia, e la tastiera del Passo 11. ⛔ **Ridiffato\ncontro il 13 di adesso (decisione 78; D80, D89):** `switchTo` è una riga e il dock segue lo store — niente `apply` né\n`unpack`; `api` resta, perché `moveActive` lo usa.\n")
    seg = sub(seg, "import Confirm from \"../components/Confirm.vue\";\nimport { unpack, useLayout, type ViewName } from \"../stores/layout\";\n",
              "import Confirm from \"../components/Confirm.vue\";\nimport { useLayout, type ViewName } from \"../stores/layout\";\n")
    seg = sub(seg, "import { apply, createDock } from \"./dock\";\nimport { directionOf, moveActive } from \"./moveActive\";",
              "import { createDock } from \"./dock\";\nimport { directionOf, moveActive } from \"./moveActive\";")
    seg = sub(seg, FRAME14_SWITCH_OLD, FRAME14_SWITCH_NEW)
    # Passo 13 (R7-10)
    seg = sub(seg, "import { PANEL_TYPES, componentFor } from \"./registry\";", "import { PANEL_TYPES, componentFor, placeholderParams } from \"./registry\";")
    seg = sub(seg, MODULES_PROBE_OLD, MODULES_PROBE_NEW)
    # Passo 15 rewritten (D56, R7-14, D87, R9b-5, R7-26)
    seg = between(seg, "- [ ] **Passo 15: il richiamo datato sulle righe di Passi della stella polare**\n",
                  "\n- [ ] **Passo 16: il mondo web verde, il cancello, e il commit**", PASSO15_NEW)
    # Criteria (R7-3, R7-4, R7-6, R7-12, R7-26)
    seg = sub(seg, "nella console del browser `harnessFake.deliverAll()`; poi si vedono la fascia che **sparisce** (è arrivato `Accepted`), il chip «collegato»",
              "nella console del browser `harnessFake.deliverAll()` — ⚠️ che porta **anche `StaleBuild`**, per costruzione dell'insieme canonico (R7-3) — e subito dopo `harnessFake.deliver(\"Accepted\")`, che rimette `connected`; poi si vedono la fascia che **sparisce** (è arrivato `Accepted`), il chip «collegato»")
    seg = sub(seg, "in Impostazioni si sceglie **Locale** → la scritta «richiesta inviata» compare e il controllo **resta** su OpenRouter (è il core che decide); `harnessFake.deliver(\"PermissionRequired\")` → la finestra si apre col focus dentro, il tabulatore **non esce**",
              "in Impostazioni si sceglie **Locale** → la scritta «richiesta inviata» compare, il controllo **resta** su OpenRouter (è il core che decide) **e la finestra si apre al click** — `deliverAll` ha già lasciato la richiesta della fixture in `core.pending`, e la finestra apre su richiesta **più** chiamata in volo (R7-4; qui stava un secondo `deliver(\"PermissionRequired\")`, che non cambiava nulla di visibile) — col focus dentro, il tabulatore **non esce**")
    seg = sub(seg, EMPTY_IT_OLD, EMPTY_IT_NEW)
    seg = sub(seg, "`git diff --stat 42b50d8..HEAD -- gui/src/panels/registry.ts` rende **solo** il commit del 13 — cioè `git log --oneline -- gui/src/panels/registry.ts | wc -l` → **1**\n",
              "`git log --oneline -- gui/src/panels/registry.ts | wc -l` → **1**, il solo commit del 13 (R7-12: un `diff --stat` stampa file e righe, mai commit)\n")
    seg = replace_line(seg, "- [ ] ⛔ **il richiamo è nella stella polare, una volta, e il file resta LF e con le tabelle intere:**",
                       "- [ ] ⛔ **i richiami sono nella stella polare — tre di riga, cinque di modulo, uno sulle registrate — e il file resta LF e con le tabelle intere:** i cinque comandi del Passo 15, **con la data al posto di `<data>`** (R7-26) → **3**, **5**, **1**, **0**, e niente")
    # D76: the thirteen `milestone` in the dictated comments
    seg = sub(seg, "milestone-2 design", "sub-project 2 design", 4)
    seg = sub(seg, "what milestone 2 builds", "what sub-project 2 builds", 3)
    seg = sub(seg, "in milestone 2 the", "in sub-project 2 the", 3)
    seg = sub(seg, "ONE function in milestone 2,", "ONE function in sub-project 2,")
    seg = sub(seg, "The modules milestone 2 builds", "The modules sub-project 2 builds")
    seg = sub(seg, "MILESTONE 2's SPA", "SUB-PROJECT 2's SPA")
    assert "milestone" not in seg.lower(), "a `milestone` survived in task 14"
    return seg


t = scoped(t, "\n## Compito 14:", "\n## Compito 15:", task14)

# ============================================================================================ task 13: R7-6 holds there too
t = scoped(t, "\n## Compito 13:", "\n## Compito 14:", lambda seg: sub(seg, EMPTY_IT_OLD, EMPTY_IT_NEW))

# ============================================================================================ the head: D87's form
t = sub(t, "col nome del sorgente; **il 17 scrive i ✅ sui 🔶 dedotti della stella",
        "col nome del sorgente — ⚠️ **applicata all'ondata 11 come UN richiamo per modulo**, sul capoverso *«Costruito dal **2**»* sotto l'intestazione (la «riga di quel modulo» della §6) e non su ogni riga costruita: le righe tengono la colonna «chi», e un richiamo di riga sta dove la riga diverge (D56, R7-14); **il 17 scrive i ✅ sui 🔶 dedotti della stella")

# ============================================================================================ ledger
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw

l = sub(l, "e la cascata sul `Frame.vue` del 14 registrata come riga del 14.",
        "e la cascata sul `Frame.vue` del 14 registrata come riga del 14. **Sessione 15, ondata 11 (2026-09-15):** compito 14 applicato (✅) — resta all'8 la sua metà di R9b-5.")

NEW_C14 = """### Compito 14
- ⚠️ **cascata di D80/D89 dal 13 corretto (decisione 78):** il `Frame.vue` ridettato al Passo 10 ridiffato contro il 13 di adesso — via `apply` e `unpack`, `switchTo` è `layout.view = view` col commento di D89; `api` resta, perché `moveActive` lo usa; la riga di prosa lo dice ✅
- R7-1 `String(…attrGet(…) ?? "")` ×2 e la riga sul tipo (`string | number | null` nel `.d.mts` spedito) ✅
- R7-2 `render` legge `stream.current?.text` al fotogramma; sonda «renders the text as it stands when the frame runs» con due `Token` prima di `await frame()` e `<p>primo secondo</p>` ✅
- R7-3 criterio: `deliverAll` porta anche `StaleBuild`, per costruzione — un `deliver("Accepted")` subito dopo ✅
- R7-4 criterio: la finestra si apre **al click** (la richiesta della fixture è già in `core.pending`), via il secondo `deliver("PermissionRequired")` ✅
- R7-5 «le **quattro** righe … fino a `reason the shape comes before the palette. */`» col `grep -c -F` → 1 prima di scrivere ✅
- R7-6 `grep -rcE … gui/src --include='*.test.ts' | grep -v ':0$'` → niente, col perché misurato — **anche nel 13**, che aveva lo stesso criterio ✅
- R7-8 `git add src/tokens/tokens.css` **prima** della mutazione (la forma di R5-11) e il capoverso riscritto: «si rilancia il primo script» non reggeva ✅
- R7-9 Files: `--stop`, il capoverso in testa, le regole della linguetta in coda ✅
- R7-10 `modules.test.ts`: `drawn(name)` monta il renderer e legge l'HTML — un costruito non contiene «placeholder», un non costruito porta `class="placeholder"`; `placeholderParams` nell'import ✅
- R7-12 criterio: il solo `git log … | wc -l` → 1 ✅
- R7-13 script `types or typings or exports`; prosa `.d.cts` di primo livello e `.d.mts` sotto `exports`, `typings` di `axe-core` ✅
- R7-14 richiami sulla riga 3 di Permessi e sulla riga 12 di Passi (costanti nel 2), nel Passo 15 riscritto ✅
- R7-26 «anche nei `grep`, con la data» nel Passo 15 e nel criterio ✅
- R10-8 Files: la stella polare (LF) con l'elenco dei richiami ✅
- R9b-4/R9a-13 (D87) un richiamo per modulo costruito — Chat, Stato, Permessi, Passi sul capoverso «Costruito dal **2**», Impostazioni sulla riga della corta — col nome del sorgente; la riga D87 del piano dice che la forma è questa e perché ✅
- R9b-5 la metà del 14: «✅ chiusa il <data>, compito 14 del piano della parte 2» sulla riga delle scorciatoie nelle registrate, nel Passo 15; **la metà dell'8** (la forma della transizione) è una riga sotto l'8 ✅
- ⚠️ **Passo 15 riscritto intero (decisione 77):** nove richiami, ancore prese DAL FILE per sezione e inizio di riga (`section`/`one`/`row`/`paragraph`), `assert` sul numero di righe invariato; i cinque comandi e l'Atteso 3/5/1/0/niente ✅
- D75: nulla da fare — le date dettate nel 14 sono misure (P-86, la 15.0.2) e `<data>` sta già nel Passo 15 ✅ · D76 «sub-project 2» nei tredici punti dei commenti dettati, con `assert` che nessun `milestone` sopravviva nel 14 ✅
- ⚠️ **Non compilato:** `Frame.vue`, `Chat.vue`, `markdown.ts` e le sonde toccate sono riscritti sul modello letto; chi esegue il 14 li compila per primi, ogni rosso è errata — come il 12 e il 13
- Attrezzo: `patch_c14.py` accanto a questo file (tocca anche il criterio del 13, la riga D87 e questo registro)
"""
l = between(l, "### Compito 14\n", "\n### Compito 15", NEW_C14)
l = sub(l, "### Compito 8 — NON RIVISTO IN PROFONDITÀ (R3 caduto): resta da fare\n",
        "### Compito 8 — NON RIVISTO IN PROFONDITÀ (R3 caduto): resta da fare\n"
        "- R9b-5, la metà dell'8: il Passo 12(a) scrive «✅ **chiusa il <data>, compito 8 del piano della parte 2**» in coda alla cella «Chiusore» della riga *«la forma con cui la transizione di policy si rilegge»* delle registrate della stella (la forma del Passo 15 del 14: ancora presa dal file) — con la revisione in profondità dell'8 ⬜\n")
l = sub(l, "- Attrezzo: `patch_c13.py` accanto a questo file (tocca anche la tabella D, la posizione, la riga del 15 e questo registro)\n",
        "- Attrezzo: `patch_c13.py` accanto a questo file (tocca anche la tabella D, la posizione, la riga del 15 e questo registro)\n"
        "- R7-6 vale anche qui (stesso criterio `gui/src/**/*.test.ts`): applicata dall'ondata del 14 (`patch_c14.py`) ✅\n")

# ============================================================================================ write, atomically
for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 14 patched (plus task 13's R7-6, D87's row and the ledger);",
      t.count("\n") - raw.count("\n"), "plan lines added;", l.count("\n") - lraw.count("\n"), "ledger lines added")
