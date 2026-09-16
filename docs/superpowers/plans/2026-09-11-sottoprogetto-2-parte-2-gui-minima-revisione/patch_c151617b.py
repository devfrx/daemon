"""patch_c151617b.py -- tasks 15, 16 and 17, the twenty-four confirmed findings of the in-depth review R8
(2026-09-16, Opus 5), plus the closure criterion the task 7 never had (R8-21), one new D row (D91) and the ledger.

Every anchor is asserted right before its write (count == 1 inside the segment it belongs to); the two files are
written atomically at the end. LF in, LF out. Usage: python patch_c151617b.py [<root>].

The two remedies that needed a measure were measured on R8's own installed eslint chain before being written here
(the short path <scratchpad>/review/probe-R8/eslint, with the thirteen `.vue` extracted from the plan):
  * with `@typescript-eslint/parser` in a `files: ["**/*.vue"]` block the six parsing errors are gone and exactly
    seven `no-raw-text` remain, on `:` and `—` (R8-1, R8-2);
  * with `"@intlify/vue-i18n/no-raw-text": ["error", { ignoreText: [":", "—"] }]` the seven are gone and
    `npx eslint src` exits 0, while a template with `{{ a }}: ciao — {{ a }}` still goes red with
    `raw text ': ciao —' is used` -- the rule reads the whole text node, so `ignoreText` silences punctuation and
    not a string (D91, measured in both directions).
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


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


def scoped(text, start_marker, end_marker, fn):
    assert text.count(start_marker) == 1, f"start: {text.count(start_marker)} of {start_marker[:70]!r}"
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    return text[:lo] + fn(text[lo:hi]) + text[hi:]


# ---- D91, after D90 --------------------------------------------------------------------------------------------------
D91 = (
    "| **D91** | ⛔ **`@intlify/vue-i18n/no-raw-text` riceve `ignoreText: [\":\", \"—\"]`, e i due caratteri RESTANO nei "
    "template** — non si spostano in `it.json` | R8-2, misurato il 2026-09-16 sulla catena installata da R8 coi tredici "
    "`.vue` estratti dal piano: la regola a `error` nudo rende **sette** errori su `:` e `—` nudi fra due mustache — "
    "`ViewBar.vue`, `Status.vue` ×3, `Strip.vue` ×3 — che i compiti 13 e 14 dettano e la rete del 13 non vede (pretende "
    "`[A-Za-zÀ-ÿ]{2,}`): il 13 e il 14 chiuderebbero verdi e il **15 nascerebbe rosso**. La via scartata è spostare i sette "
    "caratteri nelle chiavi di `it.json`, che è una modifica a **due compiti già rivisti in profondità** (R6, R7) per "
    "punteggiatura che non è una scritta — *«chi lo userà, oggi?»*, quinto criterio. ⚠️ **E `ignoreText` NON spegne la "
    "regola**, misurato nelle due direzioni: con la lista, `npx eslint src` esce **0**; un template con `{{ a }}: ciao — "
    "{{ a }}` esce **1** con `raw text ': ciao —' is used`, perché la regola legge il **nodo di testo intero** e la lista "
    "vale solo quando il nodo è esattamente uno di quei caratteri. ⚠️ **Costo dichiarato:** due caratteri non sono più "
    "traducibili — e non lo erano comunque, perché `:` e `—` non cambiano con la lingua |\n")
t = sub(t, "\n\n**La baseline di partenza, misurata il 2026-09-11 su `42b50d8` e da NON citare nei compiti:**",
        "\n" + D91 + "\n**La baseline di partenza, misurata il 2026-09-11 su `42b50d8` e da NON citare nei compiti:**")


# ---- task 15 ---------------------------------------------------------------------------------------------------------
def task15(seg):
    # R8-5: Chat.vue is the 14's
    seg = sub(seg,
              "`gui/src/panels/Chat.vue` e le diciotto chiavi `modules.*` di `gui/src/locales/it.json` (compito **13**, "
              "Passo 5 — ⚠️ qui stava «14», R7-7, e poi «Passo 12»: le chiavi sono dettate per esteso al Passo 5 dal "
              "2026-09-16, R6-4)",
              "`gui/src/panels/Chat.vue` (compito **14**, che lo crea); le diciotto chiavi `modules.*` di "
              "`gui/src/locales/it.json` (compito **13**, Passo 5 — ⚠️ qui stava «14», R7-7, e poi «Passo 12»: le chiavi "
              "sono dettate per esteso al Passo 5 dal 2026-09-16, R6-4; ⛔ **e `Chat.vue` è tornato al 14, dov'è creato:** "
              "la correzione delle chiavi se l'era portato dietro — R8-5, 2026-09-16)")
    # R8-4: the third <tempo> is measured at Passo 1
    seg = sub(seg,
              "git ls-files --eol scripts/gate.sh .github/workflows/quality-gate.yml gui/package.json\nnode --version\n```\n",
              "git ls-files --eol scripts/gate.sh .github/workflows/quality-gate.yml gui/package.json\nnode --version\n"
              "time bash scripts/gate.sh > /dev/null 2>&1\n```\n")
    seg = sub(seg,
              "in `.gitignore` e **quattro** lockfile dello spike tracciati — sono di `8fc9696`, **P-102**, e questo compito non\n"
              "li tocca;",
              "in `.gitignore` e **quattro** lockfile dello spike tracciati — ⚠️ **da TRE commit e non da uno**, "
              "`git blame -L 34,41 .gitignore`: `01694e3` le due di `app/`, `8fc9696` le tre di `electron/`, `d5eb0b8` le tre "
              "di `tauri/`, cioè i compiti 2, 4 e 5 della parte 1 (R8-8, 2026-09-16: **P-102** ne attribuiva otto a `8fc9696` "
              "perché il suo comando cercava `electron/out`, che uno solo ha aggiunto; il **conteggio** su cui il passo "
              "asserisce regge) — e questo compito non li tocca;")
    seg = sub(seg, "**prima** di inserire la riga: si prende al Passo 1 e non si ricostruisce dopo.",
              "**prima** di inserire la riga: lo "
              "produce l'ultimo comando del **Passo 1**, `time bash scripts/gate.sh`, e non si ricostruisce dopo. "
              "⚠️ **RICHIAMO DEL 2026-09-16, R8-4:** il Passo 1 non misurava **nessun** tempo — i suoi comandi erano `ls`, "
              "sei conteggi e `node --version` — quindi questa riga ordinava di scrivere un numero che nessun comando aveva "
              "prodotto; il `time` è stato aggiunto là, ed è anche la prova che il cancello è verde **prima** di cominciare.")
    # R8-1: the fourth dependency
    seg = sub(seg,
              "rilette il 2026-09-15 scrivendo questo compito: `eslint` **10.10.0**, `eslint-plugin-vue` **10.11.0**,\n"
              "`@intlify/eslint-plugin-vue-i18n` **4.5.1**.",
              "rilette il 2026-09-15 scrivendo questo compito: `eslint` **10.10.0**, `eslint-plugin-vue` **10.11.0**,\n"
              "`@intlify/eslint-plugin-vue-i18n` **4.5.1**; e `@typescript-eslint/parser` **8.70.0**, che entra in P-2 dal "
              "2026-09-16 (R8-1, misurato: MIT, `engines.node` `^18.18.0 || ^20.9.0 || >=21.1.0`, più largo di **D37**).")
    seg = sub(seg,
              'for p, v in {"eslint": "10.10.0", "eslint-plugin-vue": "10.11.0",\n'
              '             "@intlify/eslint-plugin-vue-i18n": "4.5.1"}.items():',
              'for p, v in {"eslint": "10.10.0", "eslint-plugin-vue": "10.11.0",\n'
              '             "@intlify/eslint-plugin-vue-i18n": "4.5.1",\n'
              '             "@typescript-eslint/parser": "8.70.0"}.items():')
    seg = sub(seg,
              '    "@intlify/eslint-plugin-vue-i18n": "4.5.1",\n'
              '    "eslint": "10.10.0",\n'
              '    "eslint-plugin-vue": "10.11.0"\n',
              '    "@intlify/eslint-plugin-vue-i18n": "4.5.1",\n'
              '    "@typescript-eslint/parser": "8.70.0",\n'
              '    "eslint": "10.10.0",\n'
              '    "eslint-plugin-vue": "10.11.0"\n')
    seg = sub(seg,
              "⚠️ **Nessun `@typescript-eslint/parser`, e non è una dimenticanza:** è un peer **opzionale** di\n"
              "`eslint-plugin-vue` (`peerDependenciesMeta` dice `{\"optional\":true}`, misurato il 2026-09-15) e i `.ts` questa\n"
              "catena **non li guarda affatto** — li guarda `vue-tsc` dentro `npm run build`, che è il livello 1 del mondo web\n"
              "(§8). Una dipendenza in meno, e **P-101** è la misura.",
              "⛔ **RICHIAMO DEL 2026-09-16, dalla revisione in profondità di questo compito (R8-1): `@typescript-eslint/parser` "
              "SERVE, e qui stava il contrario.** La riga diceva che nessun analizzatore TypeScript è necessario perché «i `.ts` "
              "questa catena non li guarda affatto» — vero per i `.ts` (**P-101**), e **irrilevante**: il difetto è nei `.vue`. "
              "`vue-eslint-parser` analizza il blocco `<script setup lang=\"ts\">` con **espree** quando `parserOptions.parser` "
              "non è dato, e la sintassi TypeScript lo ferma. Misurato installando la catena alle versioni qui sopra ed "
              "estraendo dal piano i tredici `.vue`: `npx eslint src` esce **1** con **dodici** problemi, di cui **sei "
              "`Parsing error`** — `Confirm.vue`, `Frame.vue`, `ViewBar.vue`, `Chat.vue`, `Placeholder.vue`, `Settings.vue`. "
              "⛔ **E un file che non si parsa non riceve NESSUNA regola**, quindi la riga 4 della tabella del Passo 4 — "
              "l'eccezione di `Chat.vue` provata su `EXIT=0` — sarebbe **vacua**: `npx eslint src/panels/Chat.vue` esce **1** "
              "con *Parsing error*. Col blocco del Passo 3 i sei spariscono e la riga 4 rende **0**. ⚠️ **È un peer "
              "OPZIONALE** di `eslint-plugin-vue` (`peerDependenciesMeta` → `{\"optional\":true}`, rimisurato il 2026-09-16), "
              "quindi npm **non** lo tira dentro da sé: va nel manifesto.")
    # R8-1 and R8-6 and D91, in the config of Passo 3
    seg = sub(seg,
              'import i18n from "@intlify/eslint-plugin-vue-i18n";\nimport vue from "eslint-plugin-vue";\n',
              'import i18n from "@intlify/eslint-plugin-vue-i18n";\nimport tsParser from "@typescript-eslint/parser";\n'
              'import vue from "eslint-plugin-vue";\n')
    seg = sub(seg,
              '  {\n    name: "harness/settings",',
              '  {\n'
              '    /**\n'
              '     * ⛔ THE TypeScript PARSER FOR THE `.vue` FILES, AND WITHOUT IT SIX OF THE THIRTEEN DO NOT PARSE.\n'
              '     * `vue-eslint-parser` reads a `<script setup lang="ts">` block with espree unless it is given one,\n'
              '     * and TypeScript syntax stops espree -- measured on 2026-09-16 on the thirteen `.vue` of this plan:\n'
              '     * `Parsing error` on `Confirm`, `Frame`, `ViewBar`, `Chat`, `Placeholder`, `Settings`. A file that\n'
              '     * does not parse gets NO rule at all, so without this block the scoped exception below could not be\n'
              '     * proven either (R8-1). The `.ts` files are still nobody\'s business here (P-101): `vue-tsc` inside\n'
              '     * `npm run build` is the level 1 of the web world.\n'
              '     */\n'
              '    name: "harness/ts-in-vue",\n'
              '    files: ["**/*.vue"],\n'
              '    languageOptions: { parserOptions: { parser: tsParser } },\n'
              '  },\n'
              '  {\n    name: "harness/settings",')
    seg = sub(seg,
              "       * `PANEL_TYPES` registry (task 13) and IS the `modules.*` key of the locale (task 14).",
              "       * `PANEL_TYPES` registry (task 13) and IS the `modules.*` key of the locale (task 13 too, step 5).")
    seg = sub(seg,
              '       * watches keys the SPA BUILDS and no lint can see those (P-105).\n'
              '       */\n'
              '      "@intlify/vue-i18n/no-raw-text": "error",',
              '       * watches keys the SPA BUILDS and no lint can see those (P-105).\n'
              '       *\n'
              '       * ⛔ `ignoreText` IS PUNCTUATION AND NOT AN ESCAPE HATCH (D91). Measured on 2026-09-16 on the\n'
              '       * thirteen `.vue` of this plan: seven errors on bare `:` and `—` between two mustaches, in\n'
              '       * `ViewBar`, `Status` and `Strip` -- which tasks 13 and 14 dictate and whose own net does not see\n'
              '       * (it wants two letters), so 13 and 14 would close green and THIS task would be born red. The list\n'
              '       * only silences a text node that IS one of those characters: `{{ a }}: ciao — {{ a }}` still goes\n'
              '       * red with `raw text \': ciao —\' is used`, measured in both directions.\n'
              '       */\n'
              '      "@intlify/vue-i18n/no-raw-text": ["error", { ignoreText: [":", "—"] }],')
    # R8-2 / R8-1: the Passo 4 table gains what the mutations prove
    seg = sub(seg,
              "| 4 | `Chat.vue` **non toccato** | ⚠️ **avviso `File ignored because no matching configuration was supplied`? NO** — il file è `.vue` e la configurazione lo copre: atteso `EXIT=0` senza righe, che è la contro-prova dell'eccezione |",
              "| 4 | `Chat.vue` **non toccato** | ⚠️ **avviso `File ignored because no matching configuration was supplied`? NO** — il file è `.vue` e la configurazione lo copre: atteso `EXIT=0` senza righe, che è la contro-prova dell'eccezione. ⛔ **RICHIAMO DEL 2026-09-16, R8-1:** senza il blocco `harness/ts-in-vue` questa riga rende **1** con `Parsing error`, non `0`, e l'eccezione resta **non provata** |")
    seg = sub(seg,
              "⚠️ **La quarta non è un di più:** senza di essa il blocco `files` sarebbe indistinguibile da un `no-v-html`\nspento per tutti, che è la mutazione che la 3 esiste per cogliere.",
              "⚠️ **La quarta non è un di più:** senza di essa il blocco `files` sarebbe indistinguibile da un `no-v-html`\n"
              "spento per tutti, che è la mutazione che la 3 esiste per cogliere.\n\n"
              "⛔ **E la prima è ANCHE la contro-prova di `ignoreText`** (**D91**, R8-2): la lista `[\":\", \"—\"]` vale solo "
              "quando il nodo di testo **è** uno di quei caratteri, quindi *«riprova piu tardi»* resta rosso. Misurato il "
              "2026-09-16 anche nella forma mista, `{{ a }}: ciao — {{ a }}` → `raw text ': ciao —' is used`, `EXIT=1`: chi "
              "esegue non aggiunge una mutazione, la **riconosce** in questa.")
    # R8-3: the /tmp sites Python reads
    seg = sub(seg,
              "accorgersene. Con l'editor si scrive il blocco qui sopra, **così com'è**, in `/tmp/setup-node.yml` — **LF**, e\nl'ultima riga termina con un a capo — e poi:",
              "accorgersene. Con l'editor si scrive il blocco qui sopra, **così com'è**, in `/tmp/setup-node.yml` — **LF**, e\n"
              "l'ultima riga termina con un a capo — e poi: ⛔ **Python NON risolve il `/tmp` di Git Bash** — su Windows "
              "`os.path.abspath('/tmp/x')` rende `C:\\tmp\\x`, che non esiste (R8-3, 2026-09-16; è la trappola che la "
              "tredicesima chiusura di questo piano registra) — quindi il percorso glielo consegna `cygpath`, e il `tr` di "
              "bash resta com'è.")
    seg = sub(seg,
              "tr -cd '\\r' < /tmp/setup-node.yml | wc -c\npython - <<'EOF'",
              "tr -cd '\\r' < /tmp/setup-node.yml | wc -c\nexport SCRATCH=\"$(cygpath -w /tmp)\"   # bash's /tmp as Windows sees it: Python does not resolve it (R8-3)\npython - <<'EOF'")
    seg = sub(seg,
              'block = io.open("/tmp/setup-node.yml", encoding="utf-8", newline="").read()',
              'block = io.open(os.path.join(os.environ["SCRATCH"], "setup-node.yml"), encoding="utf-8", newline="").read()')
    seg = sub(seg,
              'python - <<\'EOF\'\nimport io\np = ".github/workflows/quality-gate.yml"',
              'python - <<\'EOF\'\nimport io, os\np = ".github/workflows/quality-gate.yml"')
    seg = sub(seg,
              "Si scrive **prima** in `/tmp/richiamo-scritte.md`, **LF**, su una riga sola e senza il `>`, come l'altro.",
              "Si scrive **prima** in `/tmp/richiamo-scritte.md`, **LF**, su una riga sola e senza il `>`, come l'altro — e "
              "anche qui il percorso lo consegna a Python `cygpath`, mai `/tmp` (R8-3).")
    seg = sub(seg,
              "tr -cd '\\r' < /tmp/richiamo-8.md | wc -c\npython - <<'EOF'\nimport io\np = \"docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\"",
              "tr -cd '\\r' < /tmp/richiamo-8.md | wc -c\nexport SCRATCH=\"$(cygpath -w /tmp)\"   # R8-3: Python does not resolve bash's /tmp\npython - <<'EOF'\nimport io, os\np = \"docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\"")
    seg = sub(seg,
              'recall = io.open("/tmp/richiamo-8.md", encoding="utf-8", newline="").read().strip()',
              'recall = io.open(os.path.join(os.environ["SCRATCH"], "richiamo-8.md"), encoding="utf-8", newline="").read().strip()')
    seg = sub(seg,
              'strings = io.open("/tmp/richiamo-scritte.md", encoding="utf-8", newline="").read().strip()',
              'strings = io.open(os.path.join(os.environ["SCRATCH"], "richiamo-scritte.md"), encoding="utf-8", newline="").read().strip()')
    # R8-10: the third recall, on the §8 row of gate-gui.sh
    seg = sub(seg,
              "⛔ **Il richiamo si scrive PRIMA in `/tmp/richiamo-8.md`**",
              "⛔ **E un TERZO richiamo, sulla riga `scripts/gate-gui.sh` della §8 — R8-10:** la sua cella *Forma* elenca "
              "*«`npm ci`, `npm run build`, `npm test`»*, cioè **quattro** comandi col `cargo test` del finto, e dopo questo "
              "compito sono **cinque** e dopo il 16 **sette**; il commento dello script lo dice, la §8 no, e chi la rilegge "
              "ricostruirebbe uno script che non esiste. Il testo, in coda alla riga intera che comincia con "
              "`| \\`scripts/gate-gui.sh\\` |`, scritto prima in `/tmp/richiamo-forma.md`:\n\n"
              "> ✅ **RICHIAMO DEL \\<data\\>, dal compito 15 del piano della parte 2 (R8-10):** in coda arriva `npm run lint` "
              "da questo compito, e dal **16** `cargo audit --file gui/fake-core/Cargo.lock` prima del `cd gui` e `npm audit` "
              "in fondo (**D83**, **D71**): l'ordine che questa cella fissa **non cambia**, la catena si allunga\n\n"
              "⛔ **Il richiamo si scrive PRIMA in `/tmp/richiamo-8.md`**")
    seg = sub(seg,
              "lines[hits[0]] = lines[hits[0]][:-2] + \" \" + strings + \" |\"\n"
              "io.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(\"\\n\".join(lines))\nEOF",
              "    lines[hits[0]] = lines[hits[0]][:-2] + \" \" + strings + \" |\"\n"
              "# The THIRD recall, on the row of `gate-gui.sh` in §8 (R8-10): same shape, its own anchor.\n"
              "hits = [i for i, line in enumerate(lines) if line.startswith(\"| `scripts/gate-gui.sh` |\")]\n"
              "assert len(hits) == 1, \"la riga di gate-gui.sh non e' una: %d\" % len(hits)\n"
              "shape = io.open(os.path.join(os.environ[\"SCRATCH\"], \"richiamo-forma.md\"), encoding=\"utf-8\", newline=\"\").read().strip()\n"
              "assert \"\\r\" not in shape and \"<data>\" not in shape, \"LF, e la data va sostituita prima\"\n"
              "assert shape not in text and lines[hits[0]].endswith(\" |\"), \"gia' scritto, o la riga non finisce con la barra\"\n"
              "lines[hits[0]] = lines[hits[0]][:-2] + \" \" + shape + \" |\"\n"
              "io.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(\"\\n\".join(lines))\nEOF")
    # R8-7: GATE RED on the gate itself, at Passo 12
    seg = sub(seg,
              "bash scripts/gate.sh 2>&1 | tee /tmp/gate-15.log | tail -3\ngrep -c 'gui: fake core and SPA' /tmp/gate-15.log\n"
              "bash scripts/check-docs.sh\ngit status --porcelain\ngit diff --stat -- .gitignore\n```\n",
              "bash scripts/gate.sh 2>&1 | tee /tmp/gate-15.log | tail -3\ngrep -c 'gui: fake core and SPA' /tmp/gate-15.log\n"
              "bash scripts/check-docs.sh\ngit status --porcelain\ngit diff --stat -- .gitignore\n```\n\n"
              "⛔ **E `GATE RED` dal CANCELLO, non solo dallo script — R8-10bis, 2026-09-16 (R8-7).** La §8 chiede la seconda "
              "direzione *«un test della SPA reso rosso → `GATE RED`»*, e il Passo 7 la prova su `gate-gui.sh` **lanciato a "
              "mano**: che la riga `run` porti quel rosso fino in fondo al cancello non lo provava nessuno. Una mutazione "
              "sola, la stessa del mondo web:\n\n"
              "```bash\ncp gui/src/locales/copy.test.ts /tmp/copy.bak\npython - <<'EOF'\nimport io\np = \"gui/src/locales/copy.test.ts\"\n"
              "b = io.open(p, encoding=\"utf-8\", newline=\"\").read()\n"
              "io.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(b.replace(\"toContain(type.module)\", 'toContain(\"non-esiste\")', 1))\nEOF\n"
              "bash scripts/gate.sh 2>&1 | tail -3\ncp /tmp/copy.bak gui/src/locales/copy.test.ts\ngit diff --stat gui/src\n```\n\n"
              "Atteso: **`GATE RED`** col nome del passo web nell'uscita, e `git diff --stat gui/src` **vuoto** dopo la "
              "revoca. ⛔ **Se il cancello resta verde, la riga `run` non è nel posto giusto o non propaga l'uscita:** si "
              "rilegge il Passo 8 prima di committare.\n")
    seg = sub(seg,
              "- [ ] ⛔ **il passo web va rosso dai DUE mondi**, eseguito come al Passo 7, con `git status --porcelain` **vuoto** alla fine",
              "- [ ] ⛔ **il passo web va rosso dai DUE mondi**, eseguito come al Passo 7, con `git status --porcelain` **vuoto** alla fine\n"
              "- [ ] ⛔ **e il CANCELLO dice `GATE RED`** con la mutazione del Passo 12, eseguita e revocata, `git diff --stat gui/src` vuoto (R8-7: la §8 chiede `GATE RED`, e prima del 2026-09-16 nessun passo lo pretendeva)")
    seg = sub(seg,
              "- [ ] ⛔ **nessuna regola è rimasta ad avviso nel nostro blocco:**",
              "- [ ] ⛔ **l'analizzatore TypeScript c'è e i tredici `.vue` si parsano:** `grep -c 'harness/ts-in-vue' gui/eslint.config.js` → **1**, `grep -c '@typescript-eslint/parser' gui/package.json` → **1**, e `cd gui && npx eslint src` **non** stampa nessun `Parsing error` (R8-1)\n"
              "- [ ] ⛔ **nessuna regola è rimasta ad avviso nel nostro blocco:**")
    return seg


t = scoped(t, "## Compito 15: il passo web del cancello", "## Compito 16: X-1 e X-3", task15)

# P-101's pointer (R8-9) lives outside the task
t = sub(t, "**Conseguenza:** nessuna `D`; la forma dello script `lint` nel Passo 4.",
        "**Conseguenza:** nessuna `D`; la forma dello script `lint` nel **Passo 2** — ⚠️ qui stava «Passo 4», che è il lint "
        "nelle quattro direzioni (R8-9, 2026-09-16). ⛔ **E la conclusione di questa voce vale per i `.ts` e NON per i `.vue`:** "
        "il blocco `<script setup lang=\"ts\">` vuole `@typescript-eslint/parser`, misurato — **D91** e il Passo 3 del compito 15 "
        "(R8-1).")


# ---- task 16 ---------------------------------------------------------------------------------------------------------
def task16(seg):
    seg = sub(seg,
              "Atteso: **zero** per `cargo audit` (in entrambi gli script), `npm audit` e `matrix`; **uno** per `gate-gui` e `setup-node`,\n"
              "⛔ **e se sono zero il compito 15 non è eseguito**",
              "Atteso: **zero** per `cargo audit` (in entrambi gli script), `npm audit` e `matrix`; **più di zero** per "
              "`gate-gui` — la riga `run` più il commento del Passo 10 del 15, che lo nomina su due righe (R8-16, "
              "2026-09-16: qui stava «uno», e il conteggio vero è **tre**) — e **uno** per `setup-node`,\n"
              "⛔ **e se sono zero il compito 15 non è eseguito**")
    seg = sub(seg,
              "# lockfile: `-n` gives the SAME verdict in ~1.3s instead of ~10s, so somebody will add it as an",
              "# lockfile: `-n` gives the SAME verdict in a fraction of the time, so somebody will add it as an")
    seg = sub(seg,
              "# ⚠️ TWO COSTS, DECLARED. This step wants the NETWORK, and about nine of its ten seconds are that.\n"
              "# And it can go red WITHOUT A COMMIT, because the world published an advisory -- which is the point\n"
              "# rather than the price.",
              "# ⚠️ TWO COSTS, DECLARED. This step wants the NETWORK, and most of its wall time is that:\n"
              "#   <data>: `cargo audit` <tempo>, `cargo audit -n` <tempo> -- measured at step 3, same verdict.\n"
              "# ⛔ THE FIRST RUN ON A MACHINE IS DIFFERENT AND THAT IS WHY NO CONSTANT IS WRITTEN HERE: it CLONES the\n"
              "# ~45 MB advisory database (about ten seconds), and every run after it is an incremental fetch.\n"
              "# And it can go red WITHOUT A COMMIT, because the world published an advisory -- which is the point\n"
              "# rather than the price.")
    seg = sub(seg,
              "scratchpad, `/tmp/audit-comment.sh`, **LF**, e poi:\n\n```bash\ntr -cd '\\r' < /tmp/audit-comment.sh | wc -c\npython - <<'EOF'\nimport io\n",
              "scratchpad, `/tmp/audit-comment.sh`, **LF**, e poi — ⛔ **il percorso lo consegna `cygpath`, perché Python non "
              "risolve il `/tmp` di Git Bash** (R8-13, la stessa trappola del Passo 9 del 15):\n\n```bash\n"
              "tr -cd '\\r' < /tmp/audit-comment.sh | wc -c\nexport SCRATCH=\"$(cygpath -w /tmp)\"\npython - <<'EOF'\nimport io, os\n")
    seg = sub(seg,
              'block = io.open("/tmp/audit-comment.sh", encoding="utf-8", newline="").read()',
              'block = io.open(os.path.join(os.environ["SCRATCH"], "audit-comment.sh"), encoding="utf-8", newline="").read()')
    seg = sub(seg,
              "⚠️ **Se la prima uscisse ROSSA**",
              "⚠️ **E i due tempi si annotano qui**, per il commento del Passo 2 (R8-14): `time cargo audit` e "
              "`time cargo audit -n`, con la data. ⛔ **Su una macchina che non ha mai scaricato la base degli avvisi la prima "
              "corsa CLONA ~45 MB** — `ls -la ~/.cargo/advisory-db` dice quando è nata — e vale una decina di secondi; le "
              "corse dopo sono un fetch incrementale, misurate **~2 s** contro **~0,55 s** il 2026-09-16. È il motivo per cui "
              "il commento porta `<tempo>` e non una costante: **l'argomento di D69 non cambia**, `-n` resta più veloce e "
              "resta cieco.\n\n⚠️ **Se la prima uscisse ROSSA**")
    seg = sub(seg,
              "passo che installa l'attrezzo. Si scrive il file **intero** in `/tmp/quality-gate.yml` — **LF** nell'editor, e",
              "passo che installa l'attrezzo. Si scrive il file **intero** in `/tmp/quality-gate.yml` — **LF** nell'editor, "
              "letto da Python col percorso di `cygpath` (R8-13), e")
    seg = sub(seg,
              "     <(grep -A 4 'setup-node@' /tmp/quality-gate.yml) && echo \"il blocco setup-node e' INVARIATO\"\npython - <<'EOF'\nimport io\n"
              'src = io.open("/tmp/quality-gate.yml", encoding="utf-8", newline="").read()',
              "     <(grep -A 4 'setup-node@' /tmp/quality-gate.yml) && echo \"il blocco setup-node e' INVARIATO\"\n"
              "export SCRATCH=\"$(cygpath -w /tmp)\"\npython - <<'EOF'\nimport io, os\n"
              'src = io.open(os.path.join(os.environ["SCRATCH"], "quality-gate.yml"), encoding="utf-8", newline="").read()')
    seg = sub(seg,
              "testi si scrivono **prima** in `/tmp/x1.md` e `/tmp/x2.md`, su una riga sola ciascuno e **senza** il `>`, poi:\n\n"
              "```bash\npython - <<'EOF'\nimport io\np = \"docs/audit-2026-08-27.md\"\ntext = io.open(p, encoding=\"utf-8\", newline=\"\").read()\n"
              'pairs = [("la voce resta aperta finché il passo non esiste", "/tmp/x1.md"),\n'
              '         ("la voce resta aperta finché i passi non esistono", "/tmp/x2.md")]\n'
              "for anchor, src in pairs:\n"
              '    add = io.open(src, encoding="utf-8", newline="").read().strip()',
              "testi si scrivono **prima** in `/tmp/x1.md` e `/tmp/x2.md`, su una riga sola ciascuno e **senza** il `>`, e il "
              "percorso lo consegna `cygpath` (R8-13), poi:\n\n"
              "```bash\nexport SCRATCH=\"$(cygpath -w /tmp)\"\npython - <<'EOF'\nimport io, os\np = \"docs/audit-2026-08-27.md\"\n"
              "text = io.open(p, encoding=\"utf-8\", newline=\"\").read()\n"
              'pairs = [("la voce resta aperta finché il passo non esiste", "x1.md"),\n'
              '         ("la voce resta aperta finché i passi non esistono", "x2.md")]\n'
              "for anchor, src in pairs:\n"
              '    add = io.open(os.path.join(os.environ["SCRATCH"], src), encoding="utf-8", newline="").read().strip()')
    # R8-15: the cfg(unix) numeral in the X-1 cell
    seg = sub(seg,
              "> `rust-toolchain.toml` lo dice **la corsa**, non un comando locale — **P-111**, e il criterio di chiusura del 16\n> manda a guardarla.",
              "> `rust-toolchain.toml` lo dice **la corsa**, non un comando locale — **P-111**, e il criterio di chiusura del 16\n"
              "> manda a guardarla. ⚠️ **E la sonda `#[cfg(unix)]` di questa cella è UNA**, non due — "
              "`the_journal_file_is_not_world_readable` in `crates/platform/tests/file_journal.rs`; il secondo `cfg(unix)` è la "
              "`mode(0o600)` di `FileBackend::open`, codice di produzione: quante siano lo dice\n"
              "> `grep -rn 'cfg(unix)' crates/ --include='*.rs'` (R8-15, 2026-09-16; era già così al commit dell'audit).")
    # R8-11, R8-12, R8-17: the criterion
    seg = sub(seg,
              "  ```bash\n  grep -cE 'cargo audit ((-n|--no-fetch)\\b|.*--no-fetch)' scripts/gate.sh\n  grep -c 'audit-level' scripts/gate-gui.sh\n  ```\n\n  → **zero** ed **zero** (**D69**, **D71**)",
              "  ```bash\n  grep -cE 'cargo audit ((-n|--no-fetch)\\b|.*--no-fetch)' scripts/gate.sh scripts/gate-gui.sh\n"
              "  grep -cE '^[^#]*audit-level' scripts/gate-gui.sh\n  ```\n\n"
              "  → **zero** per entrambi i file e **zero** (**D69**, **D71**). ⛔ **RICHIAMO DEL 2026-09-16, R8-11 e R8-17:** il "
              "secondo comando era `grep -c 'audit-level'` e rendeva **1** su un file **giusto**, perché la parola sta nel "
              "commento che il Passo 4 stesso scrive — `^[^#]*` la cerca fuori dai commenti, provato nelle due direzioni sul "
              "file ricostruito (0 com'è, 1 con `npm audit --audit-level=high`); e il primo guardava **un** file solo, mentre da "
              "**D83** i siti di `cargo audit` sono due")
    seg = sub(seg,
              "- [ ] ⛔ **`npm audit` è l'ULTIMA riga di `gate-gui.sh`**, dentro `gui/`: `tail -3 scripts/gate-gui.sh` lo mostra, e sopra c'è `npm run lint` del compito 15",
              "- [ ] ⛔ **`npm audit` è l'ULTIMA riga di `gate-gui.sh`**, dentro `gui/`: `tail -2 scripts/gate-gui.sh` lo mostra, e `grep -n 'npm run lint\\|npm audit' scripts/gate-gui.sh` rende **due** righe, `lint` prima di `audit` (R8-12: `tail -3` non arrivava a `npm run lint`, che sta **undici** righe più su per via del commento di otto righe del Passo 4 — un numero fisso di righe invecchia col commento)")
    return seg


t = scoped(t, "## Compito 16: X-1 e X-3", "## Compito 17: la chiusura", task16)


# ---- task 17 ---------------------------------------------------------------------------------------------------------
def task17(seg):
    seg = sub(seg,
              "- Modify: `docs/design/10-modello-dei-dati-durevoli.md` (**LF**",
              "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — la riga "
              "*«Codice di prodotto»* della tabella dello stato della **§10**, che dice *«non toccato»* e da questo piano è "
              "falsa (R8-26) — Passo 8-quater\n"
              "- Modify: `docs/design/10-modello-dei-dati-durevoli.md` (**LF**")
    seg = sub(seg,
              "Atteso: le sette case `i/lf w/crlf`, e la stella, `design/10` e il piano `i/lf w/lf` (**P-114**); il tetto e il peso del compendio, da cui il\n"
              "**margine** (**P-115**); il numero dei gotcha, che è la baseline del Passo 8; **zero** per i nomi del 2 in `README.md`\n"
              "(**P-116**); **uno** per la sezione S3, che è il modello; le due righe della roadmap; `GATE GREEN`.",
              "Atteso: le sette case `i/lf w/crlf`, e la stella, `design/10` e il piano `i/lf w/lf` (**P-114**); il tetto e il peso del compendio, da cui il\n"
              "**margine** (**P-115**); il numero dei gotcha, che è la baseline del Passo 8; **zero** per i nomi del 2 in `README.md`\n"
              "(**P-116**); **uno** per la sezione S3, che è il modello; `GATE GREEN`. ⚠️ **Della roadmap escono TRE righe e non due** "
              "(R8-22, 2026-09-16): l'intestazione *«Ultimo aggiornamento»*, che il Passo 5 riscrive, la riga **2** dei "
              "sotto-progetti e la riga del piano nella tabella dei piani.")
    seg = sub(seg,
              "⚠️ **E si rileggono i sedici criteri di chiusura**, uno per compito, perché il Passo 9 li riassume e un riassunto\n"
              "scritto a memoria è il modo in cui una consegna sembra più completa di quanto sia:\n\n```bash\n"
              "grep -n '^\\*\\*Criterio di chiusura' docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n```",
              "⚠️ **E si rileggono i criteri di chiusura**, uno per compito — quanti siano lo dice il comando e non questa riga —\n"
              "perché il Passo 9 li riassume e un riassunto scritto a memoria è il modo in cui una consegna sembra più completa\n"
              "di quanto sia:\n\n```bash\n"
              "grep -nE '^(#### |\\*\\*)Criterio di chiusura' docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n```\n\n"
              "⛔ **RICHIAMO DEL 2026-09-16, R8-20 e R8-21.** Il comando era ancorato a `^\\*\\*` e rendeva **dieci** righe su "
              "diciassette compiti — i compiti **1–6** intestano il proprio criterio con `#### ` — quindi il Passo 9 avrebbe "
              "riassunto dieci criteri credendoli tutti. E il numerale *«sedici»* è **tolto e non riallineato**: era esatto "
              "sui criteri **esistenti** e falso sui compiti, perché il **compito 7 non ne aveva nessuno** — gliene è stato "
              "scritto uno nello stesso commit, e ora il comando e i compiti coincidono.")
    seg = sub(seg,
              "- la riga del **piano della parte 2** — trovata col secondo `grep`,",
              "- la riga del **piano della parte 2** — trovata con `grep -n '^| \\[Sotto-progetto 2 · parte 2' docs/roadmap.md`, "
              "che rende **una** riga (R8-22, 2026-09-16: il `grep -n 'parte-2-gui-minima'` del Passo 1 ne rende **due**, "
              "perché l'intestazione *«Ultimo aggiornamento»* cita lo stesso file),")
    # R8-23: the assert in the script of Passo 8-bis
    seg = sub(seg,
              "lines = b.split(\"\\n\")\nDATE = \"<data>\"\n\n\ndef one(needle):",
              "lines = b.split(\"\\n\")\nDATE = \"<data>\"\nassert DATE != \"<data>\", \"la data va sostituita PRIMA di lanciare (R8-23)\"\n\n\ndef one(needle):")
    # R8-26: Passo 8-quater
    seg = sub(seg,
              "- [ ] **Passo 9: la Definizione di «fatto» della parte 2 — comandi, non affermazioni**",
              "- [ ] **Passo 8-quater: la §10 del disegno del 2 — la riga «Codice di prodotto» (R8-26)**\n\n"
              "⛔ **È il GEMELLO della riga che il Passo 8-bis corregge nella stella, e nessun passo lo toccava.** La tabella "
              "*«Lo stato alla chiusura, e il comando che lo rifà»* della §10 dice *«Codice di prodotto | **non toccato**: "
              "`git diff --stat 664265a..HEAD -- crates/ …` non rende nulla»*, e dal **compito 1** quel comando rende. "
              "⚠️ **E la §10 resta il verbale della sessione che ha scritto i disegni:** non diventa il diario della parte 2, "
              "che vive in questo piano (**D74**) — il richiamo lo dice, così nessuno la riscrive per zelo.\n\n"
              "Il file è **LF**; l'ancora è la riga intera presa dal file, e il richiamo si appende **nell'ultima cella**.\n\n"
              "```bash\npython - <<'EOF'\nimport io, os, sys\nsys.stdout.reconfigure(encoding=\"utf-8\")\nDATE = \"<data>\"\n"
              "assert DATE != \"<data>\", \"la data va sostituita PRIMA di lanciare (R8-23)\"\n"
              "p = \"docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\"\n"
              "b = io.open(p, encoding=\"utf-8\", newline=\"\").read()\nassert \"\\r\\n\" not in b, \"il disegno del 2 e' LF (P-47)\"\n"
              "lines = b.split(\"\\n\")\nhits = [i for i, line in enumerate(lines) if line.startswith(\"| Codice di prodotto |\")]\n"
              "assert len(hits) == 1, f\"la riga non e' una: {len(hits)}\"\ni = hits[0]\nassert lines[i].endswith(\" |\"), lines[i][-60:]\n"
              "lines[i] = lines[i][:-2] + \" ✅ **RICHIAMO DEL \" + DATE + \", dal compito 17 del piano della parte 2 (R8-26):** \"\n"
              "    \"da questo piano il codice di prodotto **è** toccato — `crates/`, `scripts/`, `.github/`, i due lockfile e \"\n"
              "    \"`gui/`, che nasce — e la **Definizione di «fatto»** del piano dice che cosa, file per file, col comando. \"\n"
              "    \"⚠️ **E questa §10 resta il verbale della sessione che ha scritto i disegni:** il diario della parte 2 vive \"\n"
              "    \"nel piano (D74), non qui \" + \"|\"\n"
              "out = \"\\n\".join(lines)\nassert out.count(\"\\n\") == b.count(\"\\n\"), \"a line was added or lost\"\n"
              "tmp = p + \".tmp\"\nio.open(tmp, \"w\", encoding=\"utf-8\", newline=\"\").write(out)\nos.replace(tmp, p)\n"
              "print(\"ok: one recall in\", p)\nEOF\ngrep -c 'compito 17 del piano della parte 2 (R8-26)' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n"
              "tr -cd '\\r' < docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md | wc -c\n"
              "awk 'prev ~ /^\\|/ && $0 == \"\" {getline nxt; if (nxt ~ /^\\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n```\n\n"
              "Atteso (con la data scritta, **anche nel `grep`**): **1**, **0**, niente.\n\n"
              "- [ ] **Passo 9: la Definizione di «fatto» della parte 2 — comandi, non affermazioni**")
    # R8-19
    seg = sub(seg,
              "ls crates/kernel/tests/frozen/ | wc -l                           # i record congelati, otto",
              "ls crates/kernel/tests/frozen/*.cbor | wc -l                      # i record congelati -- R8-19: senza `*.cbor` conta anche `record_v1.map`")
    seg = sub(seg,
              "Le righe vengono dai\n**sedici criteri di chiusura**, riletti al Passo 1, e non dalla memoria. Almeno:",
              "Le righe vengono dai\n**criteri di chiusura**, riletti al Passo 1 **col comando che li conta** — uno per compito, "
              "compreso quello che il **7** ha ricevuto in questo commit (R8-20, R8-21) — e non dalla memoria. Almeno:")
    # R8-18 and R8-23, in the criterion
    seg = sub(seg,
              "- [ ] ⛔ **il ⏭️ della §6 nomina il sotto-progetto 13 e AUD-004**, e **non** è una catena di ✅: `awk '/⏭️/{print}' docs/COMPENDIO.md` rende una riga sola, e `grep -c '<data>' docs/COMPENDIO.md` → **zero**",
              "- [ ] ⛔ **il ⏭️ della §6 nomina il sotto-progetto 13 e AUD-004**, e **non** è una catena di ✅: `grep -c '^⏭️ \\*\\*IL PROSSIMO PASSO' docs/COMPENDIO.md` → **1** — ⚠️ **qui stava `awk '/⏭️/{print}'` «una riga sola»**, che oggi ne rende **tre** e ne renderà due qualunque cosa faccia il 17: l'elemento 2 dell'elenco numerato porta un ⏭️ a metà riga e una riga della tabella delle voci aperte lo cita come **letterale** (R8-18, 2026-09-16) — e il Passo 3 riscrive **anche l'elenco numerato** sotto il puntatore, o il suo ⏭️ sopravvive dicendo «la parte 2» a parte 2 finita\n"
              "- [ ] ⛔ **nessun `<data>` e nessun `<tempo>` è sopravvissuto, in NESSUNA casa:** `grep -rn '<data>\\|<tempo>' docs/ --include='*.md' | grep -v 'parte-2-gui-minima' ` → **niente** (R8-23: il criterio guardava il solo compendio, e il 17 scrive `<data>` in sette case più la stella, `design/10` e questo piano; il piano si esclude perché ne **detta** le occorrenze)")
    return seg


t = scoped(t, "## Compito 17: la chiusura", "## Come si riprende", task17)


# ---- task 7: the closure criterion it never had (R8-21) --------------------------------------------------------------
CRITERION7 = """
**Criterio di chiusura del compito 7**

⛔ **RICHIAMO DEL 2026-09-16, dalla revisione in profondità dei compiti 15, 16 e 17 (R8-21): questo criterio NON
esisteva.** Il compito 7 era l'unico dei diciassette senza, e il difetto è saltato fuori dal **17**, che si impegna a
riassumerli tutti: `grep -nE '^(#### |\\*\\*)Criterio di chiusura' <questo file>` ne contava sedici su diciassette
compiti. Le righe qui sotto sono ciò che i Passi di questo compito già pretendono, messe dove i fratelli le mettono.

```bash
grep -c '^#\\[test\\]' crates/kernel/tests/serving.rs
grep -rn 'fn ipc' crates/kernel/src/serving.rs
grep -c 'POLICY_FUNCTION' crates/kernel/src/serving.rs
grep -rn 'admit' crates/kernel/src/serving.rs
grep -c 'RICHIAMO DEL <data>, compito 7' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
grep -c 'RICHIAMO DEL <data>, compito 7' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
grep -c '<data>' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md
```

- [ ] `bash scripts/gate.sh` → `GATE GREEN`; `gate-deps.sh` e `gate-attributes.sh` verdi, e la lista di ADR-0031 **non cresciuta**
- [ ] ⛔ **le sedici sonde di `crates/kernel/tests/serving.rs` sono verdi**, e il loro numero è quello che il Passo 6 detta: il primo comando le conta
- [ ] ⛔ **`Core::ipc` NON esiste ancora:** il secondo comando **non rende nulla** — il suo chiamante è il rubinetto del compito **12**, e *«un elemento d'API senza chiamante si cancella»* (`crates/kernel/src/boundary.rs`)
- [ ] ⛔ **il ramo `Request` non chiama l'arbitro (D5):** il quarto comando **non rende nulla**, e la sonda `a_request_reaches_neither_the_arbiter_nor_the_journal` è verde
- [ ] ⛔ **la funzione registrata è UNA**, `POLICY_FUNCTION`, e `kernel::registry` non nomina l'arbitro (**D16**): `grep -c 'use crate::arbiter\\|crate::arbiter::' crates/kernel/src/registry.rs` → **0**
- [ ] ⛔ **i sette richiami datati sono scritti e la data è vera:** **5** nel disegno del 2 e **2** nella stella, e `grep -c '<data>'` sui due disegni → **0** (D75)
- [ ] ⛔ **i `compile_fail` sono riletti, non rigenerati in blocco:** `git diff --stat -- crates/kernel/tests/compile_fail/` nomina il **solo** `parameters_have_no_default.stderr`, e il commit dice perché
- [ ] ⛔ **i fine-riga sono invariati:** i due file nuovi a **zero** CR, `git ls-files --eol` uguale al Passo 1 su ogni file modificato
- [ ] `bash scripts/check-docs.sh` → `OK`; `git status --porcelain` vuoto; la riga **7** della tabella della posizione a ✅ col proprio commit

---
"""
t = sub(t,
        "- [ ] commit `gui(compito 7, sotto-progetto 2 parte 2): …`, **senza co-autore**, e push\n\n---\n",
        "- [ ] commit `gui(compito 7, sotto-progetto 2 parte 2): …`, **senza co-autore**, e push\n" + CRITERION7)


# ---- the ledger ------------------------------------------------------------------------------------------------------
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw, "the ledger is LF"
l = lraw
l = sub(l, "Mancano: compiti 15–17 (R8) — perimetro da rivedere ancora;",
        "Mancano: nulla — i compiti 15–17 sono stati rivisti da R8 il 2026-09-16;")
l = sub(l, "**Stato al 2026-09-15 (sessione 13",
        "**Ondata 17 (2026-09-16):** la revisione in profondità di **15, 16 e 17** è FATTA (R8, un revisore solo su Opus 5: "
        "~436k token, 132 chiamate, ~43 minuti) e i suoi **ventiquattro** rilievi confermati sono applicati (✅) con "
        "`patch_c151617b.py`, insieme al **criterio di chiusura del compito 7**, che non esisteva (R8-21), e a **D91**; il "
        "rapporto è `R8-report.md` accanto. Le due misure che il piano ora porta — l'analizzatore TypeScript e `ignoreText` — "
        "sono state rifatte dal coordinatore sulla catena `eslint` che R8 aveva installato, nelle due direzioni. Nel registro "
        "non resta nessun ⬜. **Stato al 2026-09-15 (sessione 13")
for n, rows in (
    ("15", "- R8-1 (bloccante) Passi 2 e 3: `@typescript-eslint/parser` **8.70.0** fra le devDependencies, nel dizionario del "
           "`npm view` e nella frase delle versioni; `import tsParser` e il blocco `harness/ts-in-vue` (`files: [\"**/*.vue\"]`, "
           "`languageOptions.parserOptions.parser`) **prima** di `harness/settings`, col commento; il capoverso «Nessun "
           "`@typescript-eslint/parser`» riscritto (resta vero per i `.ts`, falso per i `.vue`); la riga 4 della tabella del "
           "Passo 4 dice che senza il blocco rende 1 e l'eccezione è **vacua**; una riga nel criterio ✅\n"
           "- R8-2 (bloccante) **D91**: `\"@intlify/vue-i18n/no-raw-text\": [\"error\", { ignoreText: [\":\", \"—\"] }]` col "
           "commento, e il capoverso del Passo 4 che dichiara la prima mutazione **contro-prova** di `ignoreText` — misurato "
           "dal coordinatore sulla catena di R8: sette errori senza la lista, zero con, e `{{ a }}: ciao — {{ a }}` ancora "
           "rosso ✅\n"
           "- R8-3 (bloccante) Passi 9 e 11: `export SCRATCH=\"$(cygpath -w /tmp)\"` e `os.path.join(os.environ[\"SCRATCH\"], …)` "
           "nei tre siti che Python legge, `import io, os`, col perché accanto ✅\n"
           "- R8-4 (bloccante) Passo 1: `time bash scripts/gate.sh > /dev/null 2>&1` in coda al blocco, e il Passo 10 dice che "
           "il terzo `<tempo>` viene da lì ✅\n"
           "- R8-5 Interfaces *Consumes*: `Chat.vue` torna al compito **14**, le chiavi restano al 13 Passo 5 ✅\n"
           "- R8-6 (bloccante) Passo 3: il commento dice `(task 13 too, step 5)` ✅\n"
           "- R8-7 Passo 12: la mutazione che porta il cancello a **`GATE RED`**, revocata, e la riga nel criterio ✅\n"
           "- R8-8 Passo 1: le otto righe di `.gitignore` vengono da **tre** commit (`01694e3`, `8fc9696`, `d5eb0b8`), col perché "
           "il comando di P-102 ne indicava uno ✅\n"
           "- R8-9 **P-101**: «Passo 4» → «Passo 2», e la conclusione ristretta ai `.ts` ✅\n"
           "- R8-10 Passo 11: il **terzo** richiamo, sulla riga `scripts/gate-gui.sh` della §8 (la catena si allunga a cinque e "
           "poi a sette comandi), con la sua ancora e il suo `assert` ✅\n"
           "- Attrezzo: `patch_c151617b.py`"),
    ("16", "- R8-11 (bloccante) criterio: `grep -cE '^[^#]*audit-level' scripts/gate-gui.sh` → 0 — il `grep` di prima rendeva **1** "
           "sul commento che il Passo 4 scrive; provato nelle due direzioni ✅\n"
           "- R8-12 criterio: `tail -2` più `grep -n 'npm run lint\\|npm audit'` (due righe, `lint` prima), perché `tail -3` non "
           "arriva a `npm run lint` ✅\n"
           "- R8-13 (bloccante) Passi 2, 5 e 6: i cinque siti `/tmp` che Python legge passano da `cygpath` ✅\n"
           "- R8-14 Passo 2: i due tempi diventano `<tempo>` misurati al Passo 3, e il commento distingue la **prima** corsa "
           "(clona ~45 MB) dal fetch incrementale; l'argomento di D69 non cambia ✅\n"
           "- R8-15 Passo 6: il richiamo di X-1 chiude anche il numerale — la sonda `#[cfg(unix)]` è **una**, col comando ✅\n"
           "- R8-16 Passo 1: «più di zero» per `gate-gui` (la riga `run` più il commento del Passo 10 del 15), «uno» per "
           "`setup-node` ✅\n"
           "- R8-17 criterio: il divieto di `-n` si conta su **due** file, perché da D83 i siti sono due ✅\n"
           "- Attrezzo: `patch_c151617b.py`"),
    ("17", "- R8-18 (bloccante) criterio: `grep -c '^⏭️ \\*\\*IL PROSSIMO PASSO'` → 1 al posto dell'`awk` su `⏭️`, che rende "
           "**tre** righe e non può mai renderne una; e il Passo 3 riscrive **anche l'elenco numerato** sotto il puntatore ✅\n"
           "- R8-19 (bloccante) Passo 9: `ls crates/kernel/tests/frozen/*.cbor | wc -l`, perché senza `*.cbor` conta anche "
           "`record_v1.map` e la Definizione di «fatto» direbbe nove per otto ✅\n"
           "- R8-20 (bloccante) Passo 1: `grep -nE '^(#### |\\*\\*)Criterio di chiusura'` — l'ancora `^\\*\\*` ne vedeva **dieci**, "
           "perché i compiti 1–6 usano `#### ` — e il numerale «sedici» **tolto** ✅\n"
           "- R8-21 il **compito 7** riceve il proprio criterio di chiusura, nella forma dei fratelli (sonde, `Core::ipc` che non "
           "esiste, il ramo `Request` che non chiama l'arbitro, la funzione registrata, i sette richiami, i `compile_fail`, i "
           "fine-riga, il cancello): era l'unico dei diciassette senza ✅\n"
           "- R8-22 Passo 1 e Passo 5: le righe della roadmap sono **tre**, e la riga del piano si trova con "
           "`grep -n '^| \\[Sotto-progetto 2 · parte 2'`, che ne rende una ✅\n"
           "- R8-23 Passo 8-bis e criterio: `assert DATE != \"<data>\"` in testa allo script, e **un** comando per tutte le case "
           "(`grep -rn '<data>\\|<tempo>' docs/ --include='*.md'`, escluso il piano, che le detta) ✅\n"
           "- R8-26 **Passo 8-quater**: il richiamo sulla riga «Codice di prodotto» della §10 del disegno del 2 — il gemello di "
           "quella che il Passo 8-bis corregge nella stella — e la riga *Files*; la §10 resta il verbale della sessione dei "
           "disegni e **non** diventa il diario ✅\n"
           "- R8-24 e R8-25 NON RIPRODOTTO: nulla da fare, e il coordinatore non le ricerca (R8-25 dice che la §6 della stella "
           "nomina «Dove va cosa» mentre D14 della parte 1 e il precedente vero dicono «Specifiche»: il piano ha ragione)\n"
           "- Attrezzo: `patch_c151617b.py`"),
):
    old = f"### Compito {n} — NON RIVISTO IN PROFONDITÀ (R8 caduto): le righe note sono applicate, la revisione in profondità resta da fare"
    new = f"### Compito {n} — RIVISTO IN PROFONDITÀ da R8 il 2026-09-16 (Opus 5): ventiquattro rilievi confermati su tre compiti, tutti applicati"
    l = sub(l, old, new)
    # The rows go at the END of their own section: the boundary is the NEXT heading, which is unique.
    boundary = {"15": "\n### Compito 16 — ", "16": "\n### Compito 17 — ", "17": "\n## Voci P nuove"}[n]
    l = sub(l, boundary, "\n" + rows + "\n" + boundary)

open_lines = [line for line in l.splitlines() if " ⬜" in line]
assert len(open_lines) == 1, len(open_lines)  # the status paragraph only, which keeps the history

# ---- the guards, counted on the text produced --------------------------------------------------------------------------
assert t.count("| **D91** |") == 1
assert t.count("**Criterio di chiusura del compito 7**") == 1
assert t.count("harness/ts-in-vue") == 3          # the config block, its comment, and the closure criterion
assert t.count("ignoreText") == 5                 # D91 (x2), the rule, its comment, and the Passo 4 paragraph
assert t.count('export SCRATCH="$(cygpath -w /tmp)"') == 5
assert t.count("Passo 8-quater") == 2             # the Files row and the step heading
assert t.count("os.environ[\"SCRATCH\"]") == 7   # 4 in task 15, 3 in task 16 -- counted on the produced text
assert l.count("R8-") >= 24
assert l.count("NON RIVISTO IN PROFONDITÀ (R8 caduto)") == 0

for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: tasks 15, 16 and 17 patched (R8-1..R8-26), task 7's closure criterion, D91, and the ledger;",
      t.count("\n") - raw.count("\n"), "plan lines added")
