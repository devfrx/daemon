"""patch_c151617.py -- tasks 15, 16 and 17 in one wave (the precedent: 4+5 and 8+9).
Task 15: R10-1 (gate.sh and the workflow are `i/lf w/crlf`: the inserted lines enter CRLF, the Atteso says CR = lines),
R7-11 (no count in the eslint comment; the recall on P-99), R9a-12/D88 (the second recall of the design's §8, on the
row of the strings), R9a-15 (P-104 says §8 and «lo scrive il 15»), R5-17 (the fake core's two times in gate-gui.sh).
Task 16: R10-2 (the same CRLF truth for gate.sh, the workflow and the audit; the workflow written with the file's own
line endings), R9a-11/D88 (the recall on «Ciò che la §8 non fa»), D83 (`cargo audit --file gui/fake-core/Cargo.lock`).
Task 17: R9b-15 (the roadmap criterion anchored to the row and stripped of quotations), R9b-14 (the grep that isolates
D56), R9b-3/D86 (the title of row 2), R9b-6 + R9b-13/D87 (the north star: the state row, «Il prossimo passo», the three
🔶 confirmed), R9b-2/D85 (design/10), R9a-16 (the perimeter in the Definition of done), D84 (riferimenti.md), the two
files in Files and in the eol lists, «otto» -> «i file della lista Files».
Plus the ledger. Scoped, every anchor asserted right before its write, atomic.
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


# ============================================================================================ task 15
STRINGS_RECALL = r'''⛔ **E un SECONDO richiamo, sulla riga «le scritte» della §8 — R9a-12, D88:** quella riga dice *«un controllo che vada
rosso su una scritta lasciata nel codice, **se** al piano esiste una regola di lint matura; altrimenti revisione»*, e la
§9 (voce 3) dice che il piano lo scrive proprio qui. La regola esiste ed è a `error` (**D63**, **D65**): la riga smette
di essere condizionale. Il testo, in coda alla cella *«come si prova»* della riga:

> ✅ **RICHIAMO DEL \<data\>, dal compito 15 del piano della parte 2 (D63, D65, D88):** la regola esiste ed è matura —
> `@intlify/vue-i18n/no-raw-text` a `error`, nella catena `eslint` di `gate-gui.sh` (`npm run lint`), provata nelle
> quattro direzioni; la seconda sonda di `copy.test.ts` resta, per le chiavi che la SPA **costruisce** e che nessun lint
> vede (P-105)

Si scrive **prima** in `/tmp/richiamo-scritte.md`, **LF**, su una riga sola e senza il `>`, come l'altro.

'''

PASSO11_PY_OLD = r'''io.open(p, "w", encoding="utf-8", newline="").write(
    text.replace(anchor, "finto e non riusi quello del workspace. " + recall + " **Assunto:** niente.", 1))
EOF
'''
PASSO11_PY_NEW = r'''text = text.replace(anchor, "finto e non riusi quello del workspace. " + recall + " **Assunto:** niente.", 1)
# The SECOND recall, on the row of the strings (R9a-12, D88): found by its prefix, appended in its last cell.
lines = text.split("\n")
hits = [i for i, line in enumerate(lines) if line.startswith("| le scritte, `locales/it.json` (G21) |")]
assert len(hits) == 1, "la riga delle scritte non e' una: %d" % len(hits)
strings = io.open("/tmp/richiamo-scritte.md", encoding="utf-8", newline="").read().strip()
assert "\r" not in strings and "<data>" not in strings, "LF, e la data va sostituita prima"
assert strings not in text and lines[hits[0]].endswith(" |"), "gia' scritto, o la riga non finisce con la barra"
lines[hits[0]] = lines[hits[0]][:-2] + " " + strings + " |"
io.open(p, "w", encoding="utf-8", newline="").write("\n".join(lines))
EOF
'''


def task15(seg):
    # Files (R10-1, R9a-12)
    seg = sub(seg, "- Modify: `scripts/gate.sh` (**LF**) — una riga `run`, fra «attributes» e «documentation consistency»\n",
              "- Modify: `scripts/gate.sh` (**`i/lf w/crlf`**, come il compito 10 lo lascia — R10-1: qui stava «LF») — una riga `run`, fra «attributes» e «documentation consistency»\n")
    seg = sub(seg, "- Modify: `.github/workflows/quality-gate.yml` (**LF**) — un passo `actions/setup-node` prima di `gate.sh` (**D66**)\n",
              "- Modify: `.github/workflows/quality-gate.yml` (**`i/lf w/crlf`** — R10-1: qui stava «LF») — un passo `actions/setup-node` prima di `gate.sh` (**D66**)\n")
    seg = sub(seg, "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — un richiamo datato sulla riga dei dedotti della §8 (**P-104**)\n",
              "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — **due** richiami datati nella §8: sul capoverso dei dedotti (**P-104**) e sulla riga «le scritte» (R9a-12, **D88**)\n")
    # Passo 1 (R10-1)
    seg = sub(seg, "li tocca; i tre file dei compiti 12, 13 e 14 **esistono**; `scripts/gate.sh` e il flusso di lavoro **LF**.\n",
              "li tocca; i tre file dei compiti 12, 13 e 14 **esistono**; `scripts/gate.sh` e il flusso di lavoro **`i/lf w/crlf`**, come il\ncompito 10 li lascia — R10-1: qui stava «LF», e un inserimento LF fra righe CRLF avrebbe reso il file `w/mixed`. Si misura:\n`tr -cd '\\r' < scripts/gate.sh | wc -c` contro `wc -l < scripts/gate.sh`, **uguali**; lo stesso sul flusso.\n")
    # Passo 3 (R7-11)
    seg = sub(seg, "       * Eleven of the twelve `.vue` files here are single-word; only `ViewBar` passes (P-99).\n",
              "       * Every `.vue` file here but `ViewBar` is single-word (P-99; recounted at the review, R7-11).\n")
    # Passo 6 (R5-17): the fake core's two times in the comment
    seg = sub(seg, "# than reusing `<root>/target`. That is the declared cost of §8, not a misconfiguration.\n",
              "# than reusing `<root>/target`. That is the declared cost of §8, not a misconfiguration.\n"
              "#   <data>: `cargo test --locked` on the fake core, cold <tempo>, warm <tempo> -- measured by task 12\n"
              "#   (step 10) and re-run here (R5-17). An order of magnitude, dated; nothing asserts on it.\n")
    # Passo 8 (R10-1)
    seg = sub(seg, "line = 'run \"gui: fake core and SPA\"              bash scripts/gate-gui.sh\\n'\nassert text.count(anchor) == 1, \"ancora non unica: %d\" % text.count(anchor)\n",
              "line = 'run \"gui: fake core and SPA\"              bash scripts/gate-gui.sh\\n'\nif \"\\r\\n\" in text:\n    line = line.replace(\"\\n\", \"\\r\\n\")  # the file is CRLF in the working tree (R10-1): the new line enters CRLF too\nassert text.count(anchor) == 1, \"ancora non unica: %d\" % text.count(anchor)\n")
    seg = sub(seg, "EOF\ntr -cd '\\r' < scripts/gate.sh | wc -c\ngit diff --stat scripts/gate.sh\nsed -n '/^run \"/,/^$/p' scripts/gate.sh\n```\n\nAtteso: **zero** CR; il diff dice **una riga aggiunta** e non seicento",
              "EOF\ntr -cd '\\r' < scripts/gate.sh | wc -c; wc -l < scripts/gate.sh\ngit ls-files --eol scripts/gate.sh\ngit diff --stat scripts/gate.sh\nsed -n '/^run \"/,/^$/p' scripts/gate.sh\n```\n\nAtteso: CR **uguale alle righe**, una in più di prima, e `i/lf w/crlf` invariato (R10-1: qui stava «zero CR», falso per\nun file `w/crlf`); il diff dice **una riga aggiunta** e non cento")
    # Passo 9 (R10-1)
    seg = sub(seg, "assert block.endswith(\"\\n\") and \"\\r\" not in block, \"il blocco deve essere LF e finire con un a capo\"\nio.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(text.replace(anchor, block + anchor, 1))\nEOF\ntr -cd '\\r' < .github/workflows/quality-gate.yml | wc -c\ngit diff .github/workflows/quality-gate.yml\n```\n\nAtteso: **zero** CR prima e dopo, e il diff dice le sole righe del blocco.\n",
              "assert block.endswith(\"\\n\") and \"\\r\" not in block, \"il blocco deve essere LF e finire con un a capo\"\nif \"\\r\\n\" in text:\n    block = block.replace(\"\\n\", \"\\r\\n\")  # the workflow is CRLF in the working tree (R10-1)\nio.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(text.replace(anchor, block + anchor, 1))\nEOF\ntr -cd '\\r' < .github/workflows/quality-gate.yml | wc -c; wc -l < .github/workflows/quality-gate.yml\ngit ls-files --eol .github/workflows/quality-gate.yml\ngit diff .github/workflows/quality-gate.yml\n```\n\nAtteso: **zero** CR in `/tmp/setup-node.yml`; nel flusso CR **uguale alle righe** dopo la scrittura e `i/lf w/crlf` invariato\n(R10-1: qui stava «zero CR prima e dopo»); e il diff dice le sole righe del blocco.\n")
    # Passo 10 (R5-17, R10-1)
    seg = sub(seg, "```bash\ntime bash scripts/gate-gui.sh > /dev/null 2>&1\ntime bash scripts/gate.sh > /dev/null 2>&1\n```\n",
              "```bash\ntime bash scripts/gate-gui.sh > /dev/null 2>&1\ntime bash scripts/gate.sh > /dev/null 2>&1\ncargo clean --manifest-path gui/fake-core/Cargo.toml\ntime cargo test --locked --manifest-path gui/fake-core/Cargo.toml > /dev/null 2>&1\ntime cargo test --locked --manifest-path gui/fake-core/Cargo.toml > /dev/null 2>&1\n```\n\n⚠️ **E i due tempi del finto — a freddo e a caldo — vanno nel commento di `gate-gui.sh`** (Passo 6), dove stanno `<data>` e i\ndue `<tempo>`: misurati al Passo 10 del compito 12 e **rimisurati qui** (R5-17), perché la §8 li dichiara *«misurati al piano»*.\n")
    seg = sub(seg, "questa riga — che non ne porta, deliberatamente:\n",
              "questa riga — che non ne porta, deliberatamente. ⚠️ **E anche il commento entra CRLF come il file** (R10-1): con\n`replace_unique.py`, che converte da sé, o con Python e `.replace(\"\\n\", \"\\r\\n\")` sul blocco:\n")
    # Passo 11 (R9a-12, D88)
    seg = sub(seg, "- [ ] **Passo 11: il richiamo datato sulla riga dei dedotti della §8**\n",
              "- [ ] **Passo 11: i due richiami datati nella §8 — il capoverso dei dedotti, e la riga delle scritte (D88)**\n")
    seg = sub(seg, "⛔ **`<data>` è la data del giorno dell'esecuzione**, e questo passo dice di sostituirla: non è un segnaposto.\n\n⛔ **Il richiamo si scrive PRIMA in `/tmp/richiamo-8.md`**",
              "⛔ **`<data>` è la data del giorno dell'esecuzione**, e questo passo dice di sostituirla: non è un segnaposto.\n\n" + STRINGS_RECALL + "⛔ **Il richiamo si scrive PRIMA in `/tmp/richiamo-8.md`**")
    seg = sub(seg, PASSO11_PY_OLD, PASSO11_PY_NEW)
    # Criteria (R5-17, R9a-12)
    seg = sub(seg, "- [ ] ⛔ **il tempo del cancello è scritto e datato**, e i tre `<tempo>` del Passo 10 sono numeri veri: `grep -c '<tempo>\\|<data>' scripts/gate.sh` → **0**\n",
              "- [ ] ⛔ **il tempo del cancello è scritto e datato**, e i tre `<tempo>` del Passo 10 e i due del finto sono numeri veri: `grep -c '<tempo>\\|<data>' scripts/gate.sh scripts/gate-gui.sh` → **0** per entrambi (R5-17)\n")
    seg = sub(seg, "- [ ] ⛔ **il richiamo alla §8 è scritto e la data è vera:** `grep -c '<data>' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **0**, e `grep -c 'RICHIAMO DEL' …` → **più di zero**\n",
              "- [ ] ⛔ **i due richiami alla §8 sono scritti e la data è vera:** `grep -c '<data>' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **0**, e `grep -c 'compito 15 del piano della parte 2' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **2** (R9a-12: qui stava un richiamo solo, contato con un `grep` su **tutti** i richiami)\n")
    return seg


t = scoped(t, "\n## Compito 15:", "\n## Compito 16:", task15)

# P-99 (R7-11) and P-104 (R9a-15), in the pre-check section
t = sub(t, "nulla** — quindi gli scoperti sono **undici**, non dodici.\n",
        "nulla** — quindi gli scoperti sono **undici**, non dodici. ✅ **RICHIAMO DEL 2026-09-15, alla revisione del piano intero (R7-11):** i `.vue` sono **tredici**, dodici a una parola — `Confirm.vue` (D60) e `Settings.vue` (P-85) sono arrivati dopo questo conteggio; li conta `grep -o 'gui/src/[A-Za-z0-9_/]*\\.vue' <questo file> | sort -u | wc -l`, e il commento del compito 15 non porta più la cifra.\n")
t = sub(t, "### P-104 — la terza deduzione della §9 è MISURATA, e non serviva aspettare il core finto\n",
        "### P-104 — la terza deduzione della §8 è MISURATA, e non serviva aspettare il core finto\n")
t = sub(t, "**Domanda 1 — un'evidenza scritta prima della misura è un'ipotesi.** La §9 del 2 dichiara 🔶 **dedotto**\n",
        "**Domanda 1 — un'evidenza scritta prima della misura è un'ipotesi.** La §8 del 2 dichiara 🔶 **dedotto**\n")
t = sub(t, "⚠️ **E la riga della §9 resta com'è**: il richiamo che la porta da «dedotto» a «misurato» lo scrive il **17**, con le\naltre case — qui si registra la misura, non si riscrive un disegno che un altro compito tocca.\n",
        "⚠️ **E la riga della §8 resta com'è qui**: il richiamo che la porta da «dedotto» a «misurato» lo scrive il **15**, Passo 11 —\nqui si registra la misura, non si riscrive un disegno. ⛔ **RICHIAMO DEL 2026-09-15 (R9a-15):** questa voce diceva «§9» tre volte\ne «lo scrive il 17»: la riga sta nel capoverso 🔶 della **§8** (la §9 la nomina solo come *«l'ambiente scelto»*), e il 17 non\ntocca il disegno del 2.\n")

# ============================================================================================ task 16
NON_FA_STEP = r'''
⛔ **E un richiamo nel disegno del 2 — R9a-11, D88:** il capoverso *«**Ciò che la §8 non fa:** la CI resta solo Linux (X-1 …)
… la scansione degli avvisi di sicurezza (X-3 …)»* è falso da questo commit in due delle sue frasi, e la voce 13 della §9 lo
dava già per superato dal 2026-09-09. Il richiamo va **in coda al capoverso**, trovato per la frase che contiene; il file è
**LF** (P-47). Il testo, scritto **prima** in `/tmp/richiamo-8-non-fa.md`, su una riga e senza il `>`:

> ✅ **RICHIAMO DEL \<data\>, dal compito 16 del piano della parte 2 (D88):** la CI gira **anche su Windows** — decisione 44,
> X-1 chiusa — e gli avvisi di sicurezza si **cercano**: `cargo audit` in `gate.sh`, `npm audit` in `gate-gui.sh` — decisione
> 45, X-3 chiusa; il verbale nelle due righe dell'audit. Il resto di questo capoverso resta com'è

```bash
python - <<'EOF'
import io
p = "docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md"
text = io.open(p, encoding="utf-8", newline="").read()
assert "\r\n" not in text, "il disegno del 2 e' LF (P-47)"
lines = text.split("\n")
hits = [i for i, line in enumerate(lines) if "**Ciò che la §8 non fa:**" in line]
assert len(hits) == 1, "il capoverso non e' uno: %d" % len(hits)
i = hits[0]
while i + 1 < len(lines) and lines[i + 1] != "":
    i += 1
add = io.open("/tmp/richiamo-8-non-fa.md", encoding="utf-8", newline="").read().strip()
assert "\r" not in add and "<data>" not in add and add not in text, "LF, la data sostituita, e non gia' scritto"
lines[i] = lines[i] + " " + add
io.open(p, "w", encoding="utf-8", newline="").write("\n".join(lines))
EOF
tr -cd '\r' < docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md | wc -c
grep -c 'compito 16 del piano della parte 2' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
bash scripts/check-docs.sh
```

Atteso: **0**, **1**, `OK`.
'''

D83_BLOCK = r'''
E, **prima** del `cd gui` — subito dopo il `cargo test` del finto, che è l'unico posto in cui `gate-gui.sh` sta ancora alla
radice — **D83**: il finto ha un lockfile **suo**, seminato da quello di radice (compito 12), e `cargo audit` in `gate.sh` legge
solo quello di radice.

```bash
# ⛔ THE FAKE CORE'S OWN LOCKFILE IS AUDITED TOO (D83). It is seeded from the root's and pins the same
# crates, but it is a SECOND lockfile, and the `cargo audit` of `gate.sh` reads the root's alone. Same
# verdict expected on the same crates; a divergence between the two is task 12's comparison script.
echo "-------- gui: fake core advisories"
cargo audit --file gui/fake-core/Cargo.lock
```
'''


def task16(seg):
    # Files (R10-2, R9a-11)
    seg = sub(seg, "- Modify: `scripts/gate.sh` (**LF**) — una riga `run`, accanto a quella delle dipendenze, col commento del prerequisito (**D68**, **D69**)\n",
              "- Modify: `scripts/gate.sh` (**`i/lf w/crlf`** — R10-2: qui stava «LF») — una riga `run`, accanto a quella delle dipendenze, col commento del prerequisito (**D68**, **D69**)\n")
    seg = sub(seg, "- Modify: `.github/workflows/quality-gate.yml` (**LF**) — la matrice a due sistemi con `fail-fast: false`, e il passo che installa `cargo audit` (**D70**)\n",
              "- Modify: `.github/workflows/quality-gate.yml` (**`i/lf w/crlf`** — R10-2) — la matrice a due sistemi con `fail-fast: false`, e il passo che installa `cargo audit` (**D70**)\n")
    seg = sub(seg, "- Modify: `docs/audit-2026-08-27.md` (**LF**) — i richiami datati che chiudono **X-1** e **X-3**\n",
              "- Modify: `docs/audit-2026-08-27.md` (**`i/lf w/crlf`** — R10-2: qui stava «LF», e il Passo 6 diceva già il contrario) — i richiami datati che chiudono **X-1** e **X-3**\n"
              "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — un richiamo datato in coda al capoverso «Ciò che la §8 non fa» (R9a-11, **D88**)\n")
    # Interfaces (D83)
    seg = sub(seg, "  - `npm audit` come **ultima** riga di `scripts/gate-gui.sh`\n",
              "  - `npm audit` come **ultima** riga di `scripts/gate-gui.sh`\n  - `cargo audit --file gui/fake-core/Cargo.lock` in `scripts/gate-gui.sh`, subito dopo il `cargo test` del finto e prima del `cd gui` (**D83**)\n")
    # Passo 1 (R10-2, D83)
    seg = sub(seg, "grep -c 'npm audit' scripts/gate-gui.sh\n", "grep -c 'npm audit' scripts/gate-gui.sh\ngrep -c 'cargo audit' scripts/gate-gui.sh\n")
    seg = sub(seg, "Atteso: **zero** per `cargo audit`, `npm audit` e `matrix`; **uno** per `gate-gui` e `setup-node`, ⛔ **e se sono\nzero il compito 15 non è eseguito** e questo compito non parte; le due righe dell'audit ci sono; tutti **LF**.\n",
              "Atteso: **zero** per `cargo audit` (in entrambi gli script), `npm audit` e `matrix`; **uno** per `gate-gui` e `setup-node`,\n⛔ **e se sono zero il compito 15 non è eseguito** e questo compito non parte; le due righe dell'audit ci sono; `gate.sh`, il\nflusso e l'audit **`i/lf w/crlf`**, `gate-gui.sh` **`i/lf w/lf`** — R10-2: qui stava «tutti LF», e il Passo 6 diceva già il\ncontrario dell'audit.\n")
    # Passo 2 (R10-2)
    seg = sub(seg, "assert block.endswith(\"\\n\") and \"\\r\" not in block, \"il blocco deve essere LF e finire con un a capo\"\nio.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(text.replace(anchor, block + \"\\n\" + anchor, 1))\nEOF\ntr -cd '\\r' < scripts/gate.sh | wc -c\ngit diff --stat scripts/gate.sh\ngrep -n 'run \"' scripts/gate.sh\n```\n\n⚠️ **L'ancora è la riga DOPO",
              "assert block.endswith(\"\\n\") and \"\\r\" not in block, \"il blocco deve essere LF e finire con un a capo\"\neol = \"\\r\\n\" if \"\\r\\n\" in text else \"\\n\"  # `gate.sh` is CRLF in the working tree (R10-2): the block enters CRLF\nio.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(text.replace(anchor, block.replace(\"\\n\", eol) + eol + anchor, 1))\nEOF\ntr -cd '\\r' < scripts/gate.sh | wc -c; wc -l < scripts/gate.sh\ngit ls-files --eol scripts/gate.sh\ngit diff --stat scripts/gate.sh\ngrep -n 'run \"' scripts/gate.sh\n```\n\nAtteso: CR **uguale alle righe** e `i/lf w/crlf` invariato (R10-2), il diff che dice le sole righe del blocco più la vuota, e\nle `run` nell'ordine scritto sotto.\n\n⚠️ **L'ancora è la riga DOPO")
    # Passo 3 (D83): the two directions on the fake core's lockfile too
    seg = sub(seg, "echo \"== rosso: lo stesso avviso, NEGATO ==\"\ncargo audit --deny unmaintained; echo \"EXIT=$?\"\ngit status --porcelain\n```\n",
              "echo \"== rosso: lo stesso avviso, NEGATO ==\"\ncargo audit --deny unmaintained; echo \"EXIT=$?\"\necho \"== e le stesse due sul lockfile del finto (D83) ==\"\ncargo audit --file gui/fake-core/Cargo.lock; echo \"EXIT=$?\"\ncargo audit --file gui/fake-core/Cargo.lock --deny unmaintained; echo \"EXIT=$?\"\ngit status --porcelain\n```\n")
    # Passo 4 (D83)
    seg = sub(seg, "echo \"-------- gui: advisories\"\nnpm audit\n```\n\nE le due direzioni, la seconda su un albero costruito apposta perché il nostro è pulito:\n",
              "echo \"-------- gui: advisories\"\nnpm audit\n```\n" + D83_BLOCK + "\nE le due direzioni, la seconda su un albero costruito apposta perché il nostro è pulito:\n")
    # Passo 5 (R10-2)
    seg = sub(seg, "Si scrive il file **intero** in `/tmp/quality-gate.yml`, **LF**, e lo si copia — è\npiù corto di un inserimento per ancore, e il diff lo mostra tutto:\n",
              "Si scrive il file **intero** in `/tmp/quality-gate.yml` — **LF** nell'editor, e\npoi convertito ai fine-riga del file, che è `w/crlf` (R10-2) — e lo si scrive sopra: è più corto di un inserimento per ancore,\ne il diff lo mostra tutto:\n")
    seg = sub(seg, "cp /tmp/quality-gate.yml .github/workflows/quality-gate.yml\ntr -cd '\\r' < .github/workflows/quality-gate.yml | wc -c\ngit diff .github/workflows/quality-gate.yml\n```\n\nAtteso: **zero** CR le due volte, il `diff` **vuoto** con la riga *«il blocco setup-node e' INVARIATO»*, e il diff\ndi git che mostra **solo** la matrice, `runs-on` e il passo di `cargo install`.\n",
              "python - <<'EOF'\nimport io\nsrc = io.open(\"/tmp/quality-gate.yml\", encoding=\"utf-8\", newline=\"\").read()\nassert \"\\r\" not in src, \"LF nell'editor\"\np = \".github/workflows/quality-gate.yml\"\nold = io.open(p, encoding=\"utf-8\", newline=\"\").read()\neol = \"\\r\\n\" if \"\\r\\n\" in old else \"\\n\"  # the file's own line endings (R10-2): a `cp` of an LF file made it `w/lf`\nio.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(src.replace(\"\\n\", eol))\nEOF\ntr -cd '\\r' < .github/workflows/quality-gate.yml | wc -c; wc -l < .github/workflows/quality-gate.yml\ngit ls-files --eol .github/workflows/quality-gate.yml\ngit diff .github/workflows/quality-gate.yml\n```\n\nAtteso: **zero** CR in `/tmp/quality-gate.yml`; nel flusso CR **uguale alle righe** e `i/lf w/crlf` invariato (R10-2: qui stava\n«zero CR le due volte», e un `cp` di un file LF sopra un `w/crlf` lo portava a `w/lf`); il `diff` **vuoto** con la riga *«il\nblocco setup-node e' INVARIATO»*; e il diff di git che mostra **solo** la matrice, `runs-on` e il passo di `cargo install`.\n")
    # Passo 6 (R9a-11)
    seg = sub(seg, "Atteso: i fine-riga **invariati** rispetto al Passo 1, il diff dice **due** righe cambiate — ⛔ **se ne dice\ncentinaia, i fine-riga sono stati normalizzati:** si revoca e si rifà — e `check-docs.sh` → `OK`.\n",
              "Atteso: i fine-riga **invariati** rispetto al Passo 1, il diff dice **due** righe cambiate — ⛔ **se ne dice\ncentinaia, i fine-riga sono stati normalizzati:** si revoca e si rifà — e `check-docs.sh` → `OK`.\n" + NON_FA_STEP)
    # Passo 7 commit
    seg = sub(seg, "git add scripts/gate.sh scripts/gate-gui.sh .github/workflows/quality-gate.yml docs/audit-2026-08-27.md\n",
              "git add scripts/gate.sh scripts/gate-gui.sh .github/workflows/quality-gate.yml docs/audit-2026-08-27.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n")
    # Criteria
    seg = sub(seg, "- [ ] `bash scripts/check-docs.sh` → `OK`; i fine-riga di tutti e quattro i file **invariati** rispetto al Passo 1\n",
              "- [ ] `bash scripts/check-docs.sh` → `OK`; i fine-riga di tutti i file della lista *Files* **invariati** rispetto al Passo 1 (R10-2)\n"
              "- [ ] ⛔ **il lockfile del finto è verificato anche lui (D83):** `grep -c 'cargo audit --file gui/fake-core/Cargo.lock' scripts/gate-gui.sh` → **1**, e nell'uscita del cancello compare `gui: fake core advisories`\n"
              "- [ ] ⛔ **il capoverso «Ciò che la §8 non fa» porta il richiamo (R9a-11, D88):** `grep -c 'compito 16 del piano della parte 2' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **1**, e il file resta `i/lf w/lf`\n")
    return seg


t = scoped(t, "\n## Compito 16:", "\n## Compito 17:", task16)

# ============================================================================================ task 17
PASSO_8BIS = r'''- [ ] **Passo 8-bis: la stella polare — la riga dello stato, «Il prossimo passo», e i 🔶 che il piano ha confermato (R9b-6, R9b-13, D87)**

⛔ **La regola del 17 — *si tocca ciò che questo piano ha reso falso* — vale anche per la stella, e tre cose lo sono.** La riga
*«codice e spec non toccati»* della tabella dello stato dice *«`crates/` intatto»* col richiamo del 2026-09-10, e dopo il
compito 1 non lo è più; «Il prossimo passo» finisce con *«Le skill della sessione che scrive il piano»*, stantio da quando il
piano è scritto; e i tre capoversi 🔶 **Dedotto** che i compiti hanno confermato nel codice — le sequenze (la lista dei passi
all'accoglienza e dopo ogni invocazione; il client rifiutato), la §2 (il pacchetto opaco, i default in `gui/`, il salvataggio
automatico, il `Layout` che torna come conferma, la tabella di `redb`), la §3 (l'ordine dei pezzi, la SPA contro il ponte
finto prima del daemon) — restano 🔶 senza che chi ha costruito lo scriva (la regola della §6, D87). ⛔ **Gli altri 🔶 della
§1 restano:** parlano dei sotto-progetti 3, 5 e oltre, non del 2.

Le ancore si prendono **dal file**, per frase contenuta, e ogni richiamo si appende **in coda** alla riga o al capoverso: il file
è **LF**. `<data>` è la data del giorno.

```bash
python - <<'EOF'
import io, os, sys
sys.stdout.reconfigure(encoding="utf-8")
p = "docs/superpowers/specs/2026-09-07-direzione-gui-design.md"
b = io.open(p, encoding="utf-8", newline="").read()
assert "\r\n" not in b
lines = b.split("\n")
DATE = "<data>"


def one(needle):
    hits = [i for i, line in enumerate(lines) if needle in line]
    assert len(hits) == 1, f"{needle!r}: {len(hits)} lines"
    return hits[0]


def row(needle, recall):
    i = one(needle)
    assert lines[i].endswith(" |"), lines[i][-60:]
    lines[i] = lines[i][:-2] + " " + recall + " |"


def paragraph(needle, recall):
    i = one(needle)
    while i + 1 < len(lines) and lines[i + 1] != "":
        i += 1
    lines[i] = lines[i] + " " + recall


ok = f"✅ **confermato dal compito"
row("| codice e spec non toccati |", f"✅ **RICHIAMO DEL {DATE}, compito 17 del piano della parte 2:** e `crates/`, `scripts/`, "
    "`.github/`, `Cargo.lock`, `Cargo.toml`, `gui/` per la parte 2 — la Definizione di «fatto» del piano dice che cosa, file per file")
paragraph("Le skill della sessione che scrive il piano:", f"✅ **RICHIAMO DEL {DATE}, compito 17 del piano della parte 2:** il piano "
          "è **scritto** (2026-09-15), **rivisto** e **eseguito** — il passo dopo lo dice la §6 del compendio, in un posto solo")
paragraph("che la lista dei passi si mandi all'accoglienza", f"{ok} 7 del piano della parte 2, {DATE}:** l'accoglienza e il "
          "riinvio dopo ogni invocazione sono asseriti dalle sonde di `kernel::serving`. ⚠️ **Con una precisazione:** il client "
          "rifiutato resta in `clients` con un altro `Stage`, non in una tabella a sé, e `attending` non lo comprende — "
          "il doc di `Core::attending`")
paragraph("confrontando il `Layout` che torna", f"{ok} 5, 7 e 13 del piano della parte 2, {DATE}:** una tabella e una chiave "
          "(5, promessa 1); il core risponde a ogni `SaveLayout` con ciò che tiene (7, `keep` → `Layout`); il pacchetto è "
          "strutturato solo nella SPA, i default restano in `gui/`, il salvataggio è automatico e lo store confronta i byte "
          "che tornano con quelli mandati (13: `VIEWS`, `settle`, D80, D89)")
paragraph("l'ordine dei pezzi 2–9", f"{ok} 1–16 del piano della parte 2, {DATE}:** l'ordine dei pezzi è quello della tabella "
          "della posizione del piano; la SPA è nata contro il ponte finto (11–13) e il core finto (12) prima del daemon in "
          "produzione (9); i passi all'accoglienza e dopo ogni invocazione: compito 7")
out = "\n".join(lines)
assert out.count("\n") == b.count("\n"), "a line was added or lost"
tmp = p + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(out)
os.replace(tmp, p)
print("ok: five recalls in the north star")
EOF
grep -c 'compito 17 del piano della parte 2' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
grep -c 'confermato dal compito' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
tr -cd '\r' < docs/superpowers/specs/2026-09-07-direzione-gui-design.md | wc -c
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-07-direzione-gui-design.md
```

Atteso (con la data al posto di `<data>`, anche nei `grep`): **2**, **3**, **0**, niente. ⚠️ **Le cinque frasi-ancora sono
state contate il 2026-09-15, una volta ciascuna nel file:** se una manca, la stella è cambiata e si rilegge prima di scrivere.

- [ ] **Passo 8-ter: `docs/design/10` — l'entità costruita passa al primo diagramma (D85)**

⛔ **La regola sta nel file stesso** (decisione 20 del proprietario): *«quando un sotto-progetto costruisce, la sua entità passa
dal secondo diagramma al primo, con richiamo datato»*. Il 6 ha costruito `InvocationDetail` (`Detail::Invocation`, indice 3) e
l'8 `PolicyDetail` (`Detail::Policy`, indice 4), e il file non lo sa: `INVOCATION_DETAIL` sta nel secondo `erDiagram` come
*«specie nuova (col 2)»* e `POLICY_DETAIL` non c'è. Il file è **LF** (`git ls-files --eol` al Passo 1: `docs/design/` è misto,
questo è `i/lf w/lf`), si tocca con Python `newline=""`.

```bash
grep -n 'erDiagram\|INVOCATION_DETAIL\|POLICY_DETAIL\|specie [0-9]' docs/design/10-modello-dei-dati-durevoli.md
grep -n 'enum Detail' -A 8 crates/kernel/src/record.rs
```

Si legge prima com'è, poi:

1. nel **primo** `erDiagram` (*«Il giornale, com'è nel codice oggi»*): la relazione `DETAIL ||--o| INVOCATION_DETAIL : "specie 3"`
   e `DETAIL ||--o| POLICY_DETAIL : "specie 4"` dopo quella della specie 2, e i due blocchi di entità dopo `PERMISSION_DETAIL`,
   **coi campi letti da `crates/kernel/src/record.rs` di oggi** — non da questo piano — nella forma dei tre blocchi che ci sono;
2. nel **secondo** `erDiagram` (*«Deciso e non costruito»*): via la relazione `INVOCATION_DETAIL` e il suo blocco;
3. nella tabella *«Deciso e non costruito, per sotto-progetto»*, in coda alla riga `| \`INVOCATION_DETAIL\` | 2 |`: *«✅ **costruita
   dal compito 6 del piano della parte 2, \<data\>** — passata al primo diagramma, con `POLICY_DETAIL` (compito 8), regola di questo file»*;
4. `bash scripts/check-docs.sh` — il controllo dei diagrammi legge `docs/design/`.

```bash
awk '/^erDiagram/{n++} n==1 && /INVOCATION_DETAIL|POLICY_DETAIL/{c++} END{print c+0}' docs/design/10-modello-dei-dati-durevoli.md
awk '/^erDiagram/{n++} n==2 && /INVOCATION_DETAIL/{c++} END{print c+0}' docs/design/10-modello-dei-dati-durevoli.md
grep -c 'costruita dal compito 6 del piano della parte 2' docs/design/10-modello-dei-dati-durevoli.md
tr -cd '\r' < docs/design/10-modello-dei-dati-durevoli.md | wc -c
```

Atteso: **più di tre** (due relazioni e due blocchi, e il blocco nomina l'entità almeno una volta), **0**, **1**, **0**.

'''


def task17(seg):
    # Files (R9b-6, R9b-13, D85)
    seg = sub(seg, "- Modify: `docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md` (**LF**) — la Definizione di «fatto» (**D74**)\n",
              "- Modify: `docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md` (**LF**) — la Definizione di «fatto» (**D74**)\n"
              "- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — la riga «codice e spec non toccati» della tabella dello stato, «Il prossimo passo», e i tre 🔶 che il piano ha confermato (R9b-6, R9b-13, **D87**) — Passo 8-bis\n"
              "- Modify: `docs/design/10-modello-dei-dati-durevoli.md` (**LF**, `docs/design/` è misto: `git ls-files --eol` al Passo 1) — `INVOCATION_DETAIL` e `POLICY_DETAIL` nel primo `erDiagram`, col richiamo (R9b-2, **D85**) — Passo 8-ter\n")
    # Passo 1 and Passo 11: the eol lists and the Atteso
    seg = sub(seg, "    docs/porta-di-qualita.md docs/riferimenti.md docs/HANDOFF.md \\\n    docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n",
              "    docs/porta-di-qualita.md docs/riferimenti.md docs/HANDOFF.md \\\n    docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/design/10-modello-dei-dati-durevoli.md \\\n    docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n", 2)
    seg = sub(seg, "le sette case `i/lf w/crlf` e il piano `i/lf w/lf`", "le sette case `i/lf w/crlf`, e la stella, `design/10` e il piano `i/lf w/lf`", 2)
    seg = sub(seg, "grep -n 'in scrittura dal 2026-09-11\\|^| 2 | GUI minima' docs/roadmap.md\n",
              "grep -n 'parte-2-gui-minima\\|^| 2 | GUI minima' docs/roadmap.md\n")
    # Passo 5 (R9b-3, D86)
    seg = sub(seg, "```bash\ngrep -n '^| 2 | GUI minima' docs/roadmap.md\ngrep -n 'in scrittura dal 2026-09-11' docs/roadmap.md\n```\n\n- la riga **2** della tabella dei sotto-progetti passa da ⬜ a ✅ con la data\n- la riga del **piano della parte 2** passa da *«in scrittura … nessuno eseguito»* a **«eseguito il \\<data\\>»**, `GATE GREEN` a ogni compito\n",
              "```bash\ngrep -n '^| 2 | GUI minima' docs/roadmap.md\ngrep -n 'parte-2-gui-minima' docs/roadmap.md\n```\n\n- la riga **2** della tabella dei sotto-progetti passa da ⬜ a ✅ con la data — ⛔ **e cambia TITOLO (D86, debito dichiarato dalla §3 della stella):** da *«GUI minima (shell, chat, stato)»* a *«GUI minima — la cornice con `dockview`, il filo, la settima porta, il registro delle funzioni; Stato, Permessi, Passi, Impostazioni e la Chat sul core finto»*, cioè il perimetro della §3; il criterio lo conta\n- la riga del **piano della parte 2** — trovata col secondo `grep`, e letta com'è **prima** di riscriverla (R10-19 l'ha già portata a *«scritto il 2026-09-15, rivisto …»*) — passa a **«eseguito il \\<data\\>»**, `GATE GREEN` a ogni compito\n")
    # Passo 8 (D84, R5-17)
    seg = sub(seg, "`--manifest-path` e i tre\n`target_directory` (**P-104**).\n",
              "`--manifest-path` e i tre\n`target_directory` (**P-104**); il **processore a riposo** del daemon, misurato al Passo 14 del compito 9 — il comando, sessanta\nsecondi, il numero **senza soglia** e la data (**D84**, decisione 41 del proprietario); i due tempi del core finto, a freddo e a\ncaldo, dal commento di `gate-gui.sh` (R5-17).\n")
    # Passo 8-bis and 8-ter before Passo 9
    seg = sub(seg, "- [ ] **Passo 9: la Definizione di «fatto» della parte 2 — comandi, non affermazioni**\n",
              PASSO_8BIS + "- [ ] **Passo 9: la Definizione di «fatto» della parte 2 — comandi, non affermazioni**\n")
    # Passo 9 (R9a-16)
    seg = sub(seg, "cd gui && npm ci --no-audit --no-fund && npm run build && npm test && npm run lint; echo $?; cd ..\ngit status --porcelain                                           # vuoto\n```\n",
              "cd gui && npm ci --no-audit --no-fund && npm run build && npm test && npm run lint; echo $?; cd ..\ngit diff --name-only 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml gui/ docs/superpowers/specs/ docs/adr/ docs/design/   # ogni nome sta in una lista Files\ngit diff --stat 42b50d8..HEAD -- docs/adr/                       # vuoto: nessun ADR (vincolo globale 10)\ngit status --porcelain                                           # vuoto\n```\n\n⛔ **Il perimetro di TUTTO il piano si prova col `--name-only`** (R9a-16): la riga *«il codice fuori dal perimetro»* della §8\ndel 2 dice che il `git diff` **a fine piano** tocca solo ciò che le tabelle nominano, e prima di questa riga nessun comando\nla eseguiva — i compiti provano il proprio commit, non l'insieme. Ogni nome che esce deve comparire in una lista *Files* dei\ncompiti 1–17: il confronto si fa **a mano, nome per nome**, e un nome senza lista è una voce d'errata.\n")
    # Passo 10 (R9b-14)
    seg = sub(seg, "```bash\ngrep -c 'RICHIAMO DEL' docs/superpowers/specs/2026-09-07-direzione-gui-design.md\ngrep -c 'finestra a parte' docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n```\n",
              "```bash\ngrep -c 'dal compito 14 del piano della parte 2 (P-89, D56)' docs/superpowers/specs/2026-09-07-direzione-gui-design.md   # 1\ngrep -c 'finestra a parte' docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n```\n\n⚠️ **Il primo `grep` isola il richiamo di D56** (R9b-14): qui stava `grep -c 'RICHIAMO DEL'`, che conta **tutti** i richiami della\nstella e sarebbe salito anche se quello del 14 fosse stato scritto altrove, o mai.\n")
    # Passo 11 and criteria: «otto» -> the Files list
    seg = sub(seg, "nomina **solo** gli otto file di questo compito — ⛔ **se ne nomina altri, lo zelo ha vinto:**",
              "nomina **solo** i file della lista *Files* di questo compito — ⛔ **se ne nomina altri, lo zelo ha vinto:**")
    seg = sub(seg, "- [ ] ⛔ **i fine-riga di tutti e otto i file sono INVARIATI** rispetto al Passo 1:",
              "- [ ] ⛔ **i fine-riga di tutti i file della lista *Files* sono INVARIATI** rispetto al Passo 1:")
    seg = sub(seg, "- [ ] ⛔ **il `git diff --stat` nomina SOLO gli otto file**, e nessun file di codice:",
              "- [ ] ⛔ **il `git diff --stat` nomina SOLO i file della lista *Files***, e nessun file di codice:")
    # Criteria (R9b-15, D86, R9b-6/R9b-13, D85)
    seg = sub(seg, "- [ ] ⛔ **il conteggio dei compiti NON è tornato nella roadmap:** `grep -cE 'sedici|diciassette|[0-9]+ compiti' docs/roadmap.md` → **zero** sulla riga del piano della parte 2 (**P-113**)\n",
              "- [ ] ⛔ **il conteggio dei compiti NON è tornato nella roadmap:** `grep -F 'parte-2-gui-minima.md' docs/roadmap.md | sed 's/«[^»]*»//g' | grep -cE 'sedici|diciassette|[0-9]+ compiti'` → **0** (**P-113**; R9b-15: ancorato alla riga del piano e depurato delle citazioni «…», perché il richiamo di P-113 sulla stessa riga **cita** il testo tolto — provato nelle due direzioni il 2026-09-15, senza il `sed` rende 1)\n"
              "- [ ] ⛔ **il titolo della riga 2 è quello del perimetro (D86):** `grep -c 'GUI minima (shell, chat, stato)' docs/roadmap.md` → **0**, e `grep -c '^| 2 | GUI minima' docs/roadmap.md` → **1**\n"
              "- [ ] ⛔ **la stella porta i cinque richiami del Passo 8-bis** (R9b-6, R9b-13, D87): i quattro comandi del passo, con la data → **2**, **3**, **0**, niente\n"
              "- [ ] ⛔ **`design/10` ha le due entità nel primo diagramma e nessuna nel secondo (D85):** i quattro comandi del Passo 8-ter → **più di tre**, **0**, **1**, **0**\n")
    return seg


t = scoped(t, "\n## Compito 17:", "\n## Come si riprende", task17)

# ============================================================================================ ledger
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw

l = sub(l, "resta all'8 la sua metà di R9b-5.",
        "resta all'8 la sua metà di R9b-5. **Ondata 12 (2026-09-15):** compiti 15, 16 e 17 applicati (✅) in una ondata sola.")
OLD_C15 = "### Compito 15 — NON RIVISTO IN PROFONDITÀ (R8 caduto): resta da fare\n"
i15 = l.index(OLD_C15)
i16 = l.index("### Compito 16 — NON RIVISTO IN PROFONDITÀ (R8 caduto): resta da fare\n")
i17 = l.index("### Compito 17 — NON RIVISTO IN PROFONDITÀ (R8 caduto): resta da fare\n")
iP = l.index("\n## Voci P nuove")
assert i15 < i16 < i17 < iP
NEW_15_17 = """### Compito 15 — NON RIVISTO IN PROFONDITÀ (R8 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R10-1 `gate.sh` e il flusso sono `i/lf w/crlf` (rimisurato oggi: CR 97 = righe 97, 16 = 16): Files, Passo 1, Passo 8 (la riga `run` entra CRLF, Atteso CR = righe, `git ls-files --eol`), Passo 9 (il blocco `setup-node` convertito, Atteso CR = righe), Passo 10 (anche il commento entra CRLF) ✅
- R7-7 «(compito 13, Passo 12)» ✅ (applicata nell'ondata del 13)
- R7-11 il commento di `eslint.config.js` senza cifra («every `.vue` file here but `ViewBar`»), e il richiamo su P-99 col comando che conta (tredici, dodici a una parola; anche P-99 fuori dal 15) ✅
- R9a-12 (D88) il secondo richiamo del Passo 11, sulla riga «le scritte» della §8 (trovata per prefisso, in coda alla cella); Files, il titolo del passo e il criterio (`grep -c 'compito 15 del piano della parte 2'` → 2) ✅
- R9a-15 P-104: «§8» nel titolo e nella prima riga, «lo scrive il 15, Passo 11» nell'ultimo capoverso, col richiamo ✅
- R5-17 i due tempi del finto (`<data>`, `<tempo>` ×2) nel commento di `gate-gui.sh` (Passo 6), rimisurati al Passo 10 con `cargo clean` e due `time`, e il criterio `grep -c '<tempo>\\|<data>'` su entrambi gli script ✅
- Attrezzo: `patch_c151617.py` accanto a questo file

### Compito 16 — NON RIVISTO IN PROFONDITÀ (R8 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R10-2 `gate.sh`, il flusso e l'audit sono `i/lf w/crlf` (rimisurato oggi), solo `gate-gui.sh` LF: Files, Passo 1 (Atteso), Passo 2 (blocco e riga vuota convertiti a CRLF, Atteso CR = righe), Passo 5 (il flusso scritto coi fine-riga del file da Python, via il `cp`), criterio ✅
- R9a-11 (D88) il richiamo in coda al capoverso «Ciò che la §8 non fa» del disegno del 2, in un sotto-passo del Passo 6 (ancora per frase contenuta, fine del capoverso), Files, `git add`, criterio (`grep -c 'compito 16 del piano della parte 2'` → 1) ✅
- D83 `cargo audit --file gui/fake-core/Cargo.lock` in `gate-gui.sh` subito dopo il `cargo test` del finto, prima del `cd gui` (Passo 4), le due direzioni anche sul lockfile del finto (Passo 3), Interfaces, Passo 1 (`grep -c 'cargo audit' scripts/gate-gui.sh`), criterio ✅
- Attrezzo: `patch_c151617.py`

### Compito 17 — NON RIVISTO IN PROFONDITÀ (R8 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R9b-15 criterio della roadmap: `grep -F 'parte-2-gui-minima.md' | sed 's/«[^»]*»//g' | grep -cE …` → 0, col perché ✅
- R9b-14 Passo 10: `grep -c 'dal compito 14 del piano della parte 2 (P-89, D56)'` → 1, col perché ✅
- R9b-3 (D86) Passo 5: la riga 2 della roadmap cambia titolo sul perimetro della §3; criterio `grep -c 'GUI minima (shell, chat, stato)'` → 0; il Passo 1 e il Passo 5 cercano la riga del piano per nome del file (R10-19 l'aveva già cambiata: «in scrittura dal 2026-09-11» non c'è più) ✅
- R9b-6 Passo 8-bis: la riga «codice e spec non toccati» della tabella dello stato e «Il prossimo passo» della stella, coi richiami ✅
- R9b-13 (D87, seconda metà) Passo 8-bis: i tre capoversi 🔶 confermati (sequenze → 7 con la precisazione `Stage`; §2 → 5, 7, 13; §3 → il piano, 11–13, 9, 7), ancore per frase contenuta contate oggi (una ciascuna); gli altri 🔶 restano, perché parlano di altri sotto-progetti ✅
- R9b-2 (D85) Passo 8-ter: `design/10` (rimisurato oggi `i/lf w/lf`, CR 0) — le due entità nel primo `erDiagram` coi campi letti da `record.rs`, via dal secondo, il richiamo sulla riga della tabella, `check-docs.sh`; quattro comandi nel criterio ✅
- R9a-16 Definizione di «fatto»: `git diff --name-only 42b50d8..HEAD -- …` contro l'unione delle liste Files, e `git diff --stat 42b50d8..HEAD -- docs/adr/` vuoto ✅
- D84 Passo 8: il processore a riposo (compito 9, Passo 14) in `riferimenti.md` con comando, sessanta secondi, numero senza soglia e data; anche i due tempi del finto (R5-17) ✅
- Files: la stella e `design/10`; le due liste `git ls-files --eol` (Passo 1 e 11) e l'Atteso; «otto file» → «i file della lista *Files*» (tre posti) ✅
- Attrezzo: `patch_c151617.py`
"""
l = l[:i15] + NEW_15_17 + l[iP:]

# ============================================================================================ write, atomically
for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: tasks 15, 16, 17 patched (plus P-99, P-104 and the ledger);",
      t.count("\n") - raw.count("\n"), "plan lines added;", l.count("\n") - lraw.count("\n"), "ledger lines added")
