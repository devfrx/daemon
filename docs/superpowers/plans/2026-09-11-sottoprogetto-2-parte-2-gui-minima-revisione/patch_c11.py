"""patch_c11.py -- task 11 (R5-1/D79, R5-9 verified, R5-11 measured, R5-13, R5-15/D88, R5-18, R5-28 verified, D76, D79 in
Read), the D79 recall on the P-2 row, and the ledger rows of task 11 (plus the R5-11 line the ledger lacked for task 12).
Scoped, every anchor asserted before any write, atomic.

⚠️ R5-11 is applied in a DIFFERENT form from the one the ledger noted (`git add -N gui`): measured on 2026-09-15 in a throwaway
repository, an intent-to-add file shows its WHOLE content in `git diff --stat` even after the mutation is revoked (so «empty»
can never be true) and `git checkout --` EMPTIES it. Staging for real (`git add gui`) is what makes the diff an oracle and
the checkout a restore. The plan says so at Passo 4, the ledger row says so too.
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


def scoped(text, start_marker, end_marker, subs):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    seg = text[lo:hi]
    for old, new, n in subs:
        c = seg.count(old)
        assert c == n, f"expected {n}, found {c}: {old[:90]!r}"
    for old, new, n in subs:
        seg = seg.replace(old, new)
    return text[:lo] + seg + text[hi:]


# ----------------------------------------------------------------------------------------------- blocks (raw: no escapes)
FILES_DESIGN = r"""- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — ⛔ **due richiami datati, D88**, arrivati qui dalla revisione del piano intero (R5-15, R9a-7): la riga *«la SPA, `schema/`»* della §8 e la frase *«la SPA parla `bincode`»* del richiamo del 2026-09-10 nella §2, che **D36** rende false senza che il disegno lo dica
"""

DIST_TAGS = r"""⛔ **E il registro si interroga sulle versioni, non solo su `engines` — vincolo globale 8 (R5-18):** lo script sopra
stampa `engines` delle versioni **appuntate** e non dice se il registro ne pubblica di nuove, che è ciò che il comando della
§9 del 2 misura. Per le sei del manifesto:

```bash
for p in vue vite @vitejs/plugin-vue typescript vue-tsc vitest; do npm view "$p" version dist-tags; done
```

La regola di lettura è il vincolo globale 8, non una scelta di chi esegue: una **major** nuova sotto `latest` **non si
prende** («novità non è maturità» — `typescript` con **D79** e `vitest` con **D4** sono già i due casi, e restano appuntate);
una minor o patch nuova **solo se l'appuntata non si installa**, con voce d'errata; e quelle del giorno entrano nel commit
del Passo 14 col manifesto.

"""

TS_REASON = r"""⛔ **`typescript` è la 5.9.3 e non la 7.0.2 di P-2 — D79, misurato alla revisione del piano intero (R5-1):** `vue-tsc`
3.3.11 risolve `typescript/lib/tsc`, che la 7 **non esporta** (`ERR_PACKAGE_PATH_NOT_EXPORTED`), quindi `npm run build` esce
**1** prima ancora di `vite build`; con la 5.9.3 — l'ultima 5.x — build e sonde verdi. È la major nuova che il vincolo
globale 8 dice di non prendere finché il pari che la guida non la sa guidare.

"""

STAGE_WHY = r"""⛔ **Il `git add gui .gitignore` in testa NON anticipa il commit: è ciò che rende `git diff` un oracolo** — arrivato qui
dalla revisione del piano intero (R5-11) e **misurato il 2026-09-15** in un repository di prova (`git add -N`, mutazione,
revoca, `git diff --stat`, poi `git checkout --`): su un file **non tracciato** `git diff` non vede nulla, e *«vuoto»* sarebbe
vero anche a mutazione **non** revocata; con `git add -N` (*intent-to-add*) il diff mostra il file **intero** anche a revoca
fatta, e `git checkout --` lo **svuota**. Messo in scena davvero, `git checkout --` ripristina dall'indice e il diff a zero
dice ciò che deve. """

STAGE_13 = r"""con `git diff` a zero alla
fine — la forma che la disciplina dell'audit chiede al quarto passo. ⛔ **Prima, i file nuovi in scena** (R5-11, la ragione
al Passo 4: un file non tracciato non ha diff), e `node_modules/` e `dist/` restano fuori per le due righe di **D38**:

```bash
git add gui
git status --porcelain gui | grep -c -e node_modules -e 'dist/'
```

Atteso: **0** — la seconda direzione di **D38**, provata una volta di più. Poi le mutazioni:
"""

PASSO_13BIS = r"""
- [ ] **Passo 13-bis: i due richiami nel disegno del 2 — D88**

`docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` è **LF**: Python con `newline=""`, ancora unica
asserita, temporaneo e `os.replace` (la forma del Passo 3). ⛔ **Ogni ancora è la riga intera presa dal file col `grep`**, mai
ricopiata da qui, e il richiamo si **appende in coda**, su **una** riga — la forma del Passo 9-ter del compito 2. Arrivato
qui dalla revisione del piano intero (R5-15, R9a-7): **D36** rende false due righe del disegno e nessun compito le toccava.

| Dove (`grep -n -F` sulla frase → una riga) | Che cosa si appende, in coda |
|---|---|
| §8, la riga `\| la SPA, \`schema/\` (§6a, risposta 10) \|` — in coda alla **seconda** cella, prima di `\| \`npm test\` \|` | `✅ **RICHIAMO DEL <data>, compito 11 del piano della parte 2 (D36, P-63):** la sonda **non decodifica i byte** — \`bincode-ts\` 1.0.0 non si carica (M-11) e chi decodifica è il processo principale Node del guscio (Q1 di SP-8), che non è un compito del piano; confronta i tipi TypeScript col \`.json\` di P-62, e che i byte siano giusti lo prova \`ipc_wire.rs\` nel cancello. Decisione del proprietario del 2026-09-14, B` |
| §2, il richiamo del 2026-09-10 — la riga che **contiene** `la SPA parla \`bincode\` sull'\`ipc\` del kernel` (`grep -c -F` → **1**) e finisce con `(decisione 10 della quinta chiusura del piano).`; il richiamo si appende in coda a quella riga | `✅ **RICHIAMO DEL <data>, compito 11 del piano della parte 2 (D36):** sull'\`ipc\` parla \`bincode\` il **guscio**, non la SPA, che riceve messaggi già decodificati (§6a) e prova lo schema sul \`.json\` delle fixture — P-63` |

```bash
grep -c 'RICHIAMO DEL <data>, compito 11' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
tr -cd '\r' < docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md | wc -c
```

Atteso (con la data scritta): **2**; niente; **0**.
"""

CRITERION_D88 = r"""- [ ] `grep -c 'RICHIAMO DEL <data>, compito 11' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **2** — i due richiami di **D36** (**D88**), e il disegno resta `i/lf w/lf` a CR zero
"""

# ----------------------------------------------------------------------------------------------- task 11
t = scoped(t, "\n## Compito 11:", "\n## Compito 12:", [
    # Files (R5-15, D88): the design enters the task
    ("- Modify: `.gitignore` (**`i/lf w/crlf`**) — due righe, **D38**\n",
     "- Modify: `.gitignore` (**`i/lf w/crlf`**) — due righe, **D38**\n" + FILES_DESIGN, 1),
    # Read: D79 and D88
    ("**D35**…**D40**\n", "**D35**…**D40**, **D79**, **D88**\n", 1),
    # Interfaces (R5-13)
    ("Produces, e i compiti 12, 13 e 14 li usano con questi nomi esatti:",
     "Produces, e i compiti 13 e 14 li usano con questi nomi esatti — ⚠️ non il 12, che è Rust e non consuma nulla da `gui/src` (R5-13; qui stava «12, 13 e 14», un numero di prima di D25):", 1),
    # Passo 2 (R5-1, R5-28, R5-18)
    ('"typescript":"7.0.2"', '"typescript":"5.9.3"', 1),
    ("tale e quale, `^22.22.2 || ^24.15.0 || >=26.0.0`.",
     "tale e quale, `^22.22.2 || ^24.15.0 || >=26.0.0`. ✅ **Rimisurato il 2026-09-15 alla revisione del piano intero, con\n"
     "`typescript` 5.9.3 (D79) e `@vitejs/plugin-vue` nel dizionario: invariato** (R5-1, R5-28) — `typescript` dichiara\n"
     "`>=14.17`, più largo di tutto.", 1),
    ("la divergenza è una voce d'errata prima di essere un valore nuovo.\n\n- [ ] **Passo 3: il manifesto",
     "la divergenza è una voce d'errata prima di essere un valore nuovo.\n\n" + DIST_TAGS + "- [ ] **Passo 3: il manifesto", 1),
    # Passo 3 (R5-1, D79)
    ('    "typescript": "7.0.2",\n', '    "typescript": "5.9.3",\n', 1),
    ("\n`gui/.npmrc`, **LF**, una riga sola:\n", "\n" + TS_REASON + "`gui/.npmrc`, **LF**, una riga sola:\n", 1),
    # Passo 4 (R5-11, measured): stage before the mutation, and say why
    ('\n```bash\ncd gui\npython -c "import json,pathlib;', '\n```bash\ngit add gui .gitignore\ncd gui\npython -c "import json,pathlib;', 1),
    ("⚠️ **`git checkout --` funziona solo se il manifesto è già in `git add`**: se non lo è, si rimette a mano il\n"
     "valore del Passo 2 e lo si verifica col `git diff`. ", STAGE_WHY, 1),
    # Passo 5 (R5-1): the sentence on the 7.0.2
    ("⚠️ **`typescript` è la 7.0.2 e chi esegue non dà per scontata\nuna sola di queste opzioni:**",
     "⚠️ **`typescript` è la 5.9.3 (D79) e chi esegue non dà comunque per scontata\nuna sola di queste opzioni:**", 1),
    # Passo 13 (R5-11): stage before the mutations
    ("con `git diff` a zero alla\nfine — la forma che la disciplina dell'audit chiede al quarto passo:\n", STAGE_13, 1),
    # Passo 13-bis (R5-15, D88), before Passo 14
    ("\n- [ ] **Passo 14: il cancello, e il commit**\n", PASSO_13BIS + "\n- [ ] **Passo 14: il cancello, e il commit**\n", 1),
    # Passo 14: the design in the commit, and the message
    ("git add gui .gitignore docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n",
     "git add gui .gitignore docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\n", 1),
    ('(D38)"\ngit push\n', '(D38); i due richiami di D36 nel disegno del 2 (D88)"\ngit push\n', 1),
    # Criteria
    ("- [ ] le cinque mutazioni del Passo 13 sono state eseguite **una per volta** e revocate, e `git diff gui` è vuoto\n",
     "- [ ] le cinque mutazioni del Passo 13 sono state eseguite **una per volta** e revocate, e `git diff gui` è vuoto — **dopo** il `git add gui` del Passo 13, senza il quale un file nuovo non ha diff (R5-11)\n", 1),
    ("- [ ] ⛔ **nessun byte è stato decodificato in TypeScript**", CRITERION_D88 + "- [ ] ⛔ **nessun byte è stato decodificato in TypeScript**", 1),
    # D76: the two dictated comments
    ("milestone-2 design", "sub-project 2 design", 2),
])

# ----------------------------------------------------------------------------------------------- P-2 (D79: «P-2 riceve il richiamo»)
t = scoped(t, "\n### P-2 ", "\n### P-3 ", [
    ("| `typescript` | 7.0.2 · 2026-07-08 · Apache-2.0 | — | **7.0.2** |\n",
     "| `typescript` | 7.0.2 · 2026-07-08 · Apache-2.0 | — | **5.9.3** — ⛔ **RICHIAMO DEL 2026-09-15, D79 (R5-1): qui stava «7.0.2»**, la major nuova: `vue-tsc` 3.3.11 risolve `typescript/lib/tsc`, che la 7 non esporta — misurato; si appunta l'ultima 5.x |\n", 1),
])

# ----------------------------------------------------------------------------------------------- ledger
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw

OLD_C11 = """### Compito 11
- R5-1 (D79) `typescript` 5.9.3; Passo 5 frase; script del Passo 2 ⬜
- R5-9 = R10-7 ⬜
- R5-11 `git add -N gui` prima delle mutazioni (anche 12) ⬜
- R5-13 «i compiti 13 e 14» ⬜
- R5-15 (D88) richiamo D36 nella §8 e nella §2 del 2 (Files) ⬜
- R5-18 Passo 2: `npm view … version dist-tags` e la regola ⬜
- R5-28 nota: aggiornare Node prima — già nel Passo 2 (verificare che sia nel compito) ⬜
"""
NEW_C11 = """### Compito 11
- R5-1 (D79) `typescript` 5.9.3 nel Passo 3 con la ragione accanto, nel dizionario del Passo 2 e nella frase del Passo 5; **P-2 riceve il richiamo** (D79 lo prometteva e nessuna riga della testa lo portava); l'Atteso del Passo 2 rimisurato il 2026-09-15 con la 5.9.3 e `@vitejs/plugin-vue` nel dizionario: `jsdom` resta il più stretto su ogni ramo ✅
- R5-9 = R10-7 — già applicata nell'ondata 1 (testa), qui solo verificata: `grep -c 'interface Fixture { file: string; message: IpcMessage }'` → 1 ✅
- R5-11 ⚠️ **NON `git add -N`: `git add gui .gitignore` (Passo 4) e `git add gui` (Passo 13)** — misurato il 2026-09-15 in un repository di prova: con *intent-to-add* `git diff --stat` mostra il file intero anche a revoca fatta (mai «vuoto») e `git checkout --` lo **svuota**; messo in scena, il diff a zero e il ripristino dall'indice reggono. Il Passo 4 lo spiega, il criterio lo nomina; **il 12 resta da fare** nella stessa forma (riga aggiunta sotto) ✅
- R5-13 «i compiti 13 e 14», col perché (il 12 è Rust) ✅
- R5-15 (D88) Files + **Passo 13-bis** con la tabella delle due ancore (§8 la riga `la SPA, schema/`; §2 la frase `la SPA parla bincode` del richiamo del 2026-09-10, `grep -c -F` → 1), il `git add` del Passo 14, il messaggio di commit e il criterio `grep -c` → 2 ✅
- R5-18 Passo 2: il `for … npm view … version dist-tags` sulle sei del manifesto e la regola di lettura del vincolo 8 (major no; minor/patch solo se l'appuntata non si installa, con errata) ✅
- R5-28 verificato: «si aggiorna Node PRIMA di proseguire» è nel Passo 2 — nulla da fare ✅
- D76 «sub-project 2 design» nei due commenti dettati (`fixtures.ts`, `bridge.ts`) — non era in questa lista: applicata per coerenza coi compiti 5–10 ✅ · D79 e D88 nella lista *Read* ✅
- Attrezzo: `patch_c11.py` accanto a questo file (tocca anche P-2 e questo registro)
"""
OLD_C12_HEAD = """### Compito 12
- R5-2/R3-16 """
NEW_C12_HEAD = """### Compito 12
- R5-11 `git add gui/fake-core` prima delle mutazioni del Passo 9 — **in scena, non `-N`**: la misura è nella riga R5-11 del compito 11 ⬜
- R5-2/R3-16 """
OLD_STATE = "**Stato al 2026-09-15 (sessione 13, tredicesima chiusura):** testa e compiti 1–10 applicati (✅) — l'8 nelle sole due correzioni note, la sua revisione in profondità resta da fare; tutto il resto ⬜."
NEW_STATE = OLD_STATE + " **Sessione 14 (2026-09-15):** compito 11 applicato (✅), con una riga R5-11 nuova sul 12."

for old, new in ((OLD_C11, NEW_C11), (OLD_C12_HEAD, NEW_C12_HEAD), (OLD_STATE, NEW_STATE)):
    c = l.count(old)
    assert c == 1, f"ledger: expected 1, found {c}: {old[:80]!r}"
for old, new in ((OLD_C11, NEW_C11), (OLD_C12_HEAD, NEW_C12_HEAD), (OLD_STATE, NEW_STATE)):
    l = l.replace(old, new)

# ----------------------------------------------------------------------------------------------- write, atomically
for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 11 patched (plus P-2 and the ledger);", t.count("\n") - raw.count("\n"), "plan lines added;",
      l.count("\n") - lraw.count("\n"), "ledger lines added")
