"""patch_closure20.py -- the twentieth closure in the diary, the position recall, and the one new open row.

Every anchor is asserted right before its write; the plan is written atomically at the end. LF in, LF out.
Usage: python patch_closure20.py [<root>].

Every number below was measured on HEAD 9428f94 before this script was written, not remembered.
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = sys.argv[1] if len(sys.argv) > 1 else r"C:\Users\zagor\Desktop\harness"
PLAN = os.path.join(ROOT, "docs", "superpowers", "plans", "2026-09-11-sottoprogetto-2-parte-2-gui-minima.md")

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw, "the plan is LF: something rewrote it"
t = raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


# ---- the position recall ---------------------------------------------------------------------------------------------
RECALL = "\n".join([
    "",
    "\u2705 **RICHIAMO DEL 2026-09-16, quinta ripresa del giorno: i moduli riscritti del 12 e del 14 sono COMPILATI (R13,",
    "un revisore su Opus 5) e i suoi tre rilievi \u2014 tutti sul 14, tutti bloccanti \u2014 sono applicati (ondata 18)** \u2014 la",
    "**ventesima chiusura** dice dove si riprende: \u26d4 **la revisione del piano \u00e8 CHIUSA e non resta nessun dispaccio**;",
    "il passo che viene ora \u00e8 l'**esecuzione**, in una sessione **nuova**, con `superpowers:subagent-driven-development`",
    "\u2014 un subagente fresco per compito su Opus, col pre-controllo delle quattro domande rifatto per ciascuno contro il",
    "codice di ADESSO.",
])
t = sub(t, "\nsessione nuova.\n\n| # | Compito | Commit | Stato |",
        "\nsessione nuova.\n" + RECALL + "\n\n| # | Compito | Commit | Stato |")

# ---- the new open row ------------------------------------------------------------------------------------------------
OPEN_ROW = (
    "| \u26a0\ufe0f **il puntatore \u23ed\ufe0f della \u00a76 del compendio descrive ancora la SCRITTURA del piano della parte 2** \u2014 "
    "*\u00abscritta **ora** coi numeri in mano \u2026 con lo stesso pre-controllo, in una sessione nuova\u00bb* \u2014 mentre il piano \u00e8 "
    "**scritto** dal 2026-09-15 e **rivisto in profondit\u00e0** dal 2026-09-16, e ci\u00f2 che viene ora \u00e8 eseguirlo. "
    "\u26d4 **A/B per il proprietario: A** \u2014 lo riscrive il **compito 17**, che \u00e8 la chiusura e tocca gi\u00e0 le altre case "
    "(\u00a712 del compendio, `README.md`, la roadmap, tracciabilit\u00e0), e fino ad allora la \u00a76 resta com'\u00e8, sbagliata sul "
    "**verbo** e giusta sul **soggetto**; **B** \u2014 lo riscrive **la prima sessione che esegue**, perch\u00e9 la \u00a713 del "
    "compendio aggancia la manutenzione a *\u00abcambio del prossimo passo\u00bb* e quel cambio \u00e8 **gi\u00e0 avvenuto**. "
    "\u2705 **Consiglio: B**, perch\u00e9 un documento di stato disallineato *\u00abmente con autorevolezza\u00bb* e il compito 17 sta "
    "diciassette compiti pi\u00f9 in l\u00e0. \u26a0\ufe0f **Registrata il 2026-09-16 e NON presa:** \u00e8 il puntatore pi\u00f9 sorvegliato del "
    "repository | la \u00a76 del [`COMPENDIO.md`](../../COMPENDIO.md), e il compito 17 di questo piano | il **proprietario** |")
t = sub(t, "\n\n---\n\n## Compito 1: il contatore condiviso",
        "\n" + OPEN_ROW + "\n\n---\n\n## Compito 1: il contatore condiviso")

# ---- the twentieth closure -------------------------------------------------------------------------------------------
CLOSURE = "\n".join([
    "### La ventesima chiusura \u2014 2026-09-16, quinta ripresa del giorno: i moduli riscritti del 12 e del 14 COMPILATI "
    "(R13) e i tre rilievi applicati (ondata 18); la revisione del piano \u00e8 CHIUSA e non resta nessun dispaccio; nessun "
    "compito \u00e8 eseguito",
    "",
    "\u26d4 **DA SAPERE SUBITO, cinque cose.** **(1)** \u26d4 **La revisione del piano \u00e8 FINITA, e non resta nessun revisore da",
    "dispacciare.** R13 era l'ultimo, e non era una revisione ma una **prova di compilazione**: i moduli che la revisione",
    "aveva riscritto nel **12** (`gui/fake-core/src/main.rs`, `mod tests`) e nel **14** (`Frame.vue`, `Chat.vue`,",
    "`markdown.ts`, `modules.test.ts`, `chat.test.ts`) non erano mai stati compilati. \u23ed\ufe0f **Il passo che viene ora \u00e8",
    "l'ESECUZIONE**, in una sessione **nuova**, con `superpowers:subagent-driven-development`: un subagente fresco per",
    "compito **su Opus**, con revisione fra uno e l'altro. **(2)** \u2705 **I moduli sono compilati.** Il **14**:",
    "`npx vue-tsc --noEmit` esce **0 senza una riga** su tutti i file del 13 e del 14 riscritti dal piano di oggi, e coi",
    "tre rimedi di questa ondata la suite \u00e8 verde \u2014 `Tests 81 passed | 1 skipped`. Il **12**: `main.rs` ricomposto",
    "(655 righe) si **parsa**, e i pezzi che si possono compilare senza i compiti 1\u20137 sono stati compilati **e girati**",
    "contro il codice di oggi. \u26a0\ufe0f **Il 12 per intero resta incompilabile** finch\u00e9 i compiti 1\u20137 non esistono: chi lo",
    "esegue \u00e8 sempre il primo a vederlo intero, e la sua riga nel registro lo dichiara. **(3)** \u26d4 **IL DIARIO DI UNA",
    "CHIUSURA PU\u00d2 DIRE IL FALSO SU CI\u00d2 CHE \u00c8 CAMBIATO, e la diciannovesima lo ha detto:** *\u00abdal `b3422bf` i compiti 12 e",
    "14 non sono cambiati\u00bb* \u2014 il **12** s\u00ec, il **14** no, era passato da **2217** a **2286** righe (il Passo 15-bis e due",
    "ritocchi dell'ondata 16). Il prompt di dispaccio lo ha corretto **prima** di partire. **(4)** \u26d4 **Prima di eseguire",
    "il compito 11 si aggiorna Node** (**P-64**, **P-65**: oggi `v24.9.0`, fuori da `^24.15.0`); `cargo-audit` 0.22.2 c'\u00e8.",
    "**(5)** \u26a0\ufe0f **Una voce aperta NUOVA, ed \u00e8 del proprietario:** il puntatore \u23ed\ufe0f della **\u00a76 del compendio** descrive",
    "ancora la **scrittura** del piano della parte 2 \u2014 la riga nella tabella *\u00abLe voci aperte che questo piano SA, e non",
    "chiude\u00bb* porta l'A/B col consiglio, e questa sessione **non l'ha presa**.",
    "",
    "\u2705 **Che cosa \u00e8 stato fatto.** Tre commit, ciascuno col cancello verde prima (i log nello scratchpad di questa",
    "sessione, `\u2026\\8a83ac76-bd8b-4305-9e56-870cd55138a1\\scratchpad`, che pu\u00f2 non sopravvivere): `fbc4d04` \u2014 il rapporto",
    "`R13-report.md` copiato nel repo, **da solo** (decisione 90); `9428f94` \u2014 l'**ondata 18**: il compito 14 corretto sui",
    "**tre** rilievi confermati di R13, tutti bloccanti, il **12** che non ne ha, e le due P che li contano; e questo, la",
    "chiusura. \u26a0\ufe0f **Le tre misure sono state rifatte dal coordinatore sul modello compilato di R13 prima di dettare",
    "l'ondata** (decisione 95), nelle **due direzioni**: col testo del piano `npx vitest run` rende esattamente **tre**",
    "`it` rossi coi messaggi che il rapporto cita, coi tre rimedi **zero**.",
    "",
    "| | Stato alla chiusura, e il comando che lo rifà |",
    "|---|---|",
    "| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` \u2192 "
    "`## main...origin/main`, niente sotto; `git stash list` vuoto |",
    "| I commit di questa sessione | `git log --oneline e851b5d..HEAD` \u2192 **tre**: `fbc4d04`, `9428f94`, e questa "
    "chiusura; `git log --oneline 878e9ef..HEAD` \u2192 **ventuno** |",
    "| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock "
    "Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |",
    "| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` \u2192 **133** e "
    "`grep -c '^[|] \\*\\*D[0-9]' <questo file>` \u2192 **91**, **invariati**: R13 non porta P nuove \u2014 ricade sui compiti 12 e "
    "14, che hanno gi\u00e0 **P-128** e **P-130**, e i loro comandi hanno contato da s\u00e9 \u2014 e nessuno dei tre rimedi \u00e8 una "
    "decisione (decisione 101) |",
    # `\\|` and not `[|]`: inside an ERE alternation `[|]` would match a literal pipe and the command would render 0.
    # `\\|` renders as `|` in the cell, which is the command that works -- the form the nineteenth closure used.
    "| I criteri di chiusura | `grep -cE '^(#### \\|\\*\\*)Criterio di chiusura' <questo file>` \u2192 **17**, uno per compito |",
    "| Quanto \u00e8 corretto | `ledger.md`: \u2705 la testa e i compiti 1\u201317, \u2705 il 3, l'8, il 13 e 15\u201317 in profondit\u00e0, \u2705 la "
    "**compilazione** del 12 e del 14; `grep -c ' \u2b1c' ledger.md` \u2192 **1** (il capoverso dello stato, che tiene la storia) |",
    "| I rilievi \u00abfatto\u00bb confermati nei rapporti | **172** il 2026-09-16, coi rapporti R1\u2013R13, col comando nel blocco "
    "**sotto** questa tabella; R13 ne porta **tre**, tutti sul 14 |",
    "| L'errata | `awk '/^## \u26a0\ufe0f L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\\| \\*\\*E[0-9]/{c++} "
    "END{print c+0}' <questo file>` \u2192 **0** |",
    "| Cancello | `bash scripts/gate.sh` \u2192 `GATE GREEN` prima di ogni commit, **quattro** corse: la baseline "
    "all'apertura e una per commit; `bash scripts/check-docs.sh` \u2192 `OK` |",
    "| Fine-riga | questo piano \u00e8 **LF**: `git ls-files --eol <questo file>` \u2192 `i/lf w/lf`, "
    "`tr -cd '\\r' < <questo file> [|] wc -c` \u2192 `0`; `ledger.md`, `R13-report.md` e i `patch_*.py` LF |",
    "| Tabelle spezzate | `awk 'prev ~ /^\\|/ && $0 == \"\" {getline nxt; if (nxt ~ /^\\|/) print NR} {prev=$0}' "
    "<questo file>` \u2192 **niente** |",
    "| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) \u2014 "
    "`grep -n '<the version' <questo file>` lo trova; `<data>` e `<tempo>` non sono segnaposto (D75) |",
    "| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio n\u00e9 la roadmap \u2014 e la \u00a76 che "
    "**andrebbe** toccata \u00e8 la voce aperta nuova, che \u00e8 del proprietario |",
    "| Documenti fuori dal piano | nella cartella della revisione: `R13-report.md`, `patch_c14b.py`, "
    "`patch_closure20.py`; `ledger.md` con l'ondata 18 e le due sezioni riviste |",
    "| File temporanei | nessuno nel repository. \u26d4 **`C:\\Users\\zagor\\AppData\\Local\\Temp\\probe-R13\\gui\\` \u00e8 il "
    "modello dell'11, del 13 e del 14 INSTALLATO E VERDE**, coi tre rimedi applicati: chi esegue i compiti 13 e 14 lo "
    "riusa invece di rifarlo, e chi lo tocca ricorda che i suoi sorgenti sono un'**estrazione** del piano, non il repo. "
    "`probe-R6\\gui\\` \u00e8 il modello del 13 **senza** l'ondata 16 nei sorgenti: se ne copiano i `node_modules`, mai i "
    "sorgenti. Lo scratchpad `\u2026\\4fa001a2-\u2026\\review\\` porta lo scheletro, `p-titles.md`, il lockfile del modello "
    "dell'11, le cartelle di prova e i rapporti \u2014 tutto cancellabile |",
    "| Debito lasciato | \u26d4 **nessuno dentro la revisione, e a dirlo sono i comandi qui sopra**: i sei perimetri sono "
    "rivisti, le due A/B decise, le diciassette P scritte, i moduli riscritti **compilati**, nessun \u2b1c nel registro. "
    "Resta l'**esecuzione**, e la voce aperta **(5)** per il proprietario |",
    "",
    "Il comando dei rilievi \u00abfatto\u00bb confermati \u2014 una riga per rilievo, la cella del verdetto seguita da quella della",
    "specie:",
    "",
    "```bash",
    "cat docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/R*-report.md | "
    "grep -cE '^\\| R[0-9]+[ab]?-[0-9]+ .*\\| *CONFERMATO[^|]*\\| *fatto *\\|'",
    "```",
    "",
    "#### Le decisioni prese scrivendo, oltre a quelle del registro",
    "",
    "| # | Decisione | Perché | Costo se sbagliata |",
    "|---|---|---|---|",
    "| 100 | quando il comando di una **P** non vede le righe di un rapporto nuovo, si allarga il **comando**, non si "
    "riscrive il rapporto: la forma della cella \u00e8 del revisore, e il rapporto \u00e8 un **reperto** | R13 scrive il numero di "
    "compito **in grassetto** (`**14**`) e l'alternanza nuda di P-130 rendeva **7** invece di **10**. Il precedente c'era "
    "gi\u00e0: P-129 prende le celle di R6 **per prefisso**, perch\u00e9 R6 non porta il numero. Riscrivere un rapporto per farlo "
    "combaciare con un comando \u00e8 alterare l'evidenza dopo averla letta | un comando per rapporto invece di uno solo, e "
    "chi ne aggiunge uno deve **misurare** che il comando lo veda invece di darlo per scontato |",
    "| 101 | un rimedio **costretto** dalla lingua o da una libreria non diventa una **D**: `data-href=` contiene "
    "`href=`, `reka-ui` smonta in tre giri, e non c'era niente da scegliere | una D registra una **decisione**, cio\u00e8 una "
    "via presa fra pi\u00f9 possibili col costo dichiarato. Una D che registra un fatto costretto \u00e8 una **casa in pi\u00f9** per "
    "quel fatto (gotcha #68), e la casa giusta \u00e8 il capoverso accanto al codice, dove chi esegue lo legge | il perch\u00e9 "
    "vive solo accanto al codice: se qualcuno cancella il capoverso, il fatto torna a sembrare arbitrario |",
    "",
    "#### Le trappole di questa sessione \u2014 istruzioni, non aneddoti",
    "",
    "- \u26d4 **IL DIARIO DI UNA CHIUSURA \u00c8 UN'IPOTESI COME IL PIANO: si ridiffa il perimetro di OGNI compito del mandato",
    "  contro il commit che la chiusura nomina**, prima di dispacciare. La forma che regge: `git show <commit>:<piano>` e",
    "  il piano di oggi, estratti ciascuno col proprio `awk` per intestazione, poi `diff -u [|] grep -c '^[+-]'`. \u26a0\ufe0f **Un",
    "  `grep '^@@'` sul diff intero NON basta**: i numeri di riga si spostano fra i commit, e un hunk \u00abdentro il compito",
    "  14\u00bb letto sulla numerazione di oggi pu\u00f2 essere di un altro compito.",
    "- \u26d4 **LA CARTELLA DI PROVA DI UN REVISORE FINITO NON \u00c8 AGGIORNATA PER CI\u00d2 CHE \u00c8 VENUTO DOPO**: i `node_modules` di",
    "  `probe-R6/gui` si riusano e risparmiano un `npm ci`, i suoi **sorgenti** no \u2014 misurato con un `grep` su una riga",
    "  che l'ondata 16 cambia (`INCOMPLETE every time` \u2192 0, `on the mounted components` \u2192 1). Si copia, si **cancella**",
    "  `src/` e `schema/`, e si riscrive tutto dal piano di oggi; un file stantio sopravvissuto falsifica il verde.",
    "- \u26d4 **UNA GUARDIA CONTATA A MEMORIA FALLISCE ANCHE QUANDO SEMBRA OVVIA**: `\u2192 **10** il 2026-09-16` esisteva gi\u00e0 in",
    "  **P-122** (compito 6), quindi la guardia \u00abdeve renderne uno\u00bb \u00e8 andata rossa alla prima corsa. La forma che regge \u00e8",
    "  il **delta** contro il testo di partenza: `t.count(x) == raw.count(x) + 1`. \u00c8 la stessa trappola della",
    "  diciannovesima chiusura, ricaduta \u2014 e a coglierla \u00e8 stata la prova su **copie**, non la rilettura.",
    "- \u26a0\ufe0f **UNA PROSA INSERITA DA UNO SCRIPT VA RIAVVOLTA COME IL FILE**: una riga di 600 caratteri passa ogni controllo",
    "  \u2014 tabelle intere, CR zero, `check-docs` OK \u2014 e stona in mezzo a un file che va a capo a 118. Si guarda il diff con",
    "  `awk '{ if (length($0) > 126) print length($0) }'` sulle sole righe aggiunte, prima del cancello.",
    "- \u26a0\ufe0f **IL MODELLO COMPILATO DI UN REVISORE \u00c8 L'ORACOLO DELL'ONDATA, E SI USA NELLE DUE DIREZIONI**: qui i tre",
    "  rimedi sono stati **revocati** sul modello per vedere i tre rossi coi messaggi esatti, poi **ripristinati** da una",
    "  copia di riserva. Verificare solo il verde avrebbe provato che la suite passa, non che i rimedi **servono**.",
    "",
    "#### Che cosa la sessione nuova fa, nell'ordine",
    "",
    "1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la",
    "   testa \u00e8 il commit di questa chiusura o uno dopo.",
    "2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, le voci",
    "   aperte, la tabella D a 25 righe per chiamata). \u26a0\ufe0f **Il registro e i rapporti NON si leggono per eseguire**:",
    "   servivano alla revisione, che \u00e8 chiusa. Si aprono a domanda, sul compito che si sta eseguendo.",
    "3. \u23ed\ufe0f **L'ESECUZIONE**, con `superpowers:subagent-driven-development`: un subagente fresco per compito **su Opus**,",
    "   con revisione fra uno e l'altro, dal **compito 1**, nell'ordine della tabella della posizione. \u26d4 **Il",
    "   pre-controllo delle quattro domande si rifà per OGNI compito prima di dispacciarlo**, contro il codice di ADESSO:",
    "   il piano \u00e8 del 2026-09-11 e le sue cifre sono istantanee di quel giorno (vincolo globale 5), e il pre-controllo",
    "   ha trovato un difetto reale in **tutti** i compiti finora.",
    "4. \u26d4 **Prima del compito 11 si aggiorna Node** (P-64, P-65). \u26d4 **Prima dei compiti 13 e 14** si guarda se",
    "   `C:\\Users\\zagor\\AppData\\Local\\Temp\\probe-R13\\gui\\` c'\u00e8 ancora: \u00e8 il modello **installato e verde**, e risparmia",
    "   un `npm ci` \u2014 ma i suoi sorgenti sono un'estrazione del piano, non il repo.",
    "5. Ogni compito: il **cancello prima del commit**, la riga della posizione aggiornata **nel commit del compito**, il",
    "   commit **senza co-autore**. Se il compito dice il falso **ci si ferma e si riporta**: una divergenza \u00e8 una voce",
    "   d'errata prima di essere un rimedio.",
    "6. La voce aperta **(5)** \u2014 il puntatore \u23ed\ufe0f della \u00a76 del compendio \u2014 si porta al **proprietario** alla prima",
    "   occasione, in A/B, col consiglio gi\u00e0 scritto nella riga.",
    "7. Alla chiusura del piano la cartella della revisione si **archivia** (decisione 69).",
    "8. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.",
    "",
    ""])
t = sub(t, "\n### La diciannovesima chiusura \u2014 2026-09-16", "\n" + CLOSURE + "### La diciannovesima chiusura \u2014 2026-09-16")

# ---- guards, counted on the text this script produced -----------------------------------------------------------------
assert t.count("### La ventesima chiusura") == 1, "the closure is not there once"
assert t.count("RICHIAMO DEL 2026-09-16, quinta ripresa del giorno") == 1, "the position recall"
assert t.count("il puntatore \u23ed\ufe0f della \u00a76 del compendio descrive ancora la SCRITTURA") == 1, "the open row"
assert t.count("\n### P-") == raw.count("\n### P-"), "a P was added or lost"
assert t.count("\n| **D") == raw.count("\n| **D"), "a D row was added or lost"
assert t.count("\n## Compito") == raw.count("\n## Compito"), "a task was added or lost"
assert t.count("\r") == 0, "a CR crept in"
# no blank line inside a table, checked here and again by check_after_write.sh
lines = t.split("\n")
broken = [i + 1 for i in range(1, len(lines) - 1)
          if lines[i] == "" and lines[i - 1].startswith("|") and lines[i + 1].startswith("|")]
assert not broken, f"blank line inside a table at {broken}"

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: plan", raw.count("\n") + 1, "->", t.count("\n") + 1, "lines")
