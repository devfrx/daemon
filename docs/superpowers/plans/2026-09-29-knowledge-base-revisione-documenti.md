# Knowledge base, la revisione — i documenti: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`, un subagente fresco per
> compito, su `model: "opus"` — `"sonnet"` per il lavoro meccanico, mai Fable —, con revisione fra uno e l'altro: la
> modalità scelta dal proprietario (`CLAUDE.md`). I passi usano le caselle (`- [ ]`) per il tracciamento.
> ⛔ **Una fase per sessione** (`CLAUDE.md`): il piano è **scritto il 2026-09-29**, in due sessioni — la prima si è fermata
> a metà su richiesta del proprietario, e la sua consegna sta in
> [archivio](../../archivio/consegna-piano-knowledge-base-revisione-documenti.md) —; il **pre-controllo** delle quattro
> domande si fa in un'altra, **prima** di dispacciare; ogni compito si esegue in un'altra ancora. ⛔ **Non si esegue prima
> del pre-controllo.**

**Obiettivo.** Tradurre in documenti il [disegno della revisione della knowledge base](../specs/2026-09-28-knowledge-base-revisione-design.md),
chiuso e riletto dal proprietario il 2026-09-29: **ADR-0040** — dove vivono i dati, e che cosa salva il programma —, che
**modifica in parte** ADR-0022, con ciò che il cancello pretende insieme a lui; i **rimandi datati** in testa a dieci ADR e
nella riga di ADR-0039, con la loro riga nelle voci della §5 del compendio; i **richiami datati** nel disegno del
2026-09-04, nel disegno dei gesti, nella spec del sotto-progetto 1, in design/09, in design/10 e nella stella polare della
GUI; le righe di `roadmap.md` e di `tracciabilita.md`; e la chiusura, col disegno negli indici e il puntatore della §6
mosso.

**Forma.** Sei compiti in sequenza, un commit ciascuno; il numeratore vive nella tabella della posizione qui sotto. Ogni
compito applica un **blocco** di modifiche già scritto, parola per parola, con `apply_edits.py`: prima col `--check`, che
non scrive, poi davvero — tutto o niente. ⛔ **Nessun file di `crates/`, `gui/`, `scripts/` o dei manifesti cambia**: il
piano scrive **solo documenti**, e lo prova il comando D. Le decisioni sono del proprietario — le risposte D1–D20 e le sei
sezioni approvate del disegno —; il piano le traduce, e quelle che aggiunge stanno in *«Le decisioni prese scrivendo il
piano»*, col perché.

**Strumenti.** `bash`, `awk`, `grep`, `sed -n` in lettura; **Python 3** per ogni scrittura, con gli attrezzi della sezione
*«Gli attrezzi»* qui sotto, al posto di `replace_unique.py` dei piani di prima — D15 e D19; `git`. La porta di qualità è
`bash scripts/gate.sh`, **da sola** (gotcha **#133**), e deve stampare `GATE GREEN` prima di ogni commit, anche di soli
documenti: gira `check-docs.sh`, ed è quello che qui morde; `bash scripts/check-docs.sh` deve stampare `OK`. Nei comandi dei
compiti `S` è la cartella dello scratchpad in una forma che Python capisca — su Git Bash, `S=$(cygpath -m <scratchpad>)` —,
`D` è il giorno dell'esecuzione, `D=$(date +%F)`, e ogni comando parte dalla radice del repository; gli attrezzi si
lanciano con `PYTHONIOENCODING=utf-8`, perché la console di una macchina Windows può essere cp1252.

**Disegno:** [`specs/2026-09-28-knowledge-base-revisione-design.md`](../specs/2026-09-28-knowledge-base-revisione-design.md)
— chi coordina lo legge **per intero** prima dei compiti; a ciascun subagente si danno le sezioni che il suo compito
nomina, non il file. Il [disegno del 2026-09-04](../specs/2026-09-04-knowledge-base-design.md) resta il disegno approvato
della knowledge base: questo piano ne corregge le righe superate, e **non disegna la capacità** — il compendio, §8.

## Gli attrezzi — vivono nello scratchpad, mai nel repository

Quattro, più il controllo delle tabelle — D15, D19 e D25. `extract.py` si copia **a mano**, col tool di scrittura, dal suo
recinto qui sotto: non porta nessuna barra rovesciata, che il canale di un tool può cambiare in silenzio. Tutto il resto lo
copia **lui**, parola per parola, dai recinti di questo file — gli altri tre attrezzi, `tables.awk`, il testo di ADR-0040,
i blocchi dei compiti e il puntatore nuovo del compito 6:

```bash
PYTHONIOENCODING=utf-8 python "$S/extract.py" docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md "$S"
```

Atteso: diciotto righe, una per pezzo; un rifiuto non scrive niente. I tre attrezzi di scrittura sono provati il 2026-09-29
nelle due direzioni su file di prova e su un file CRLF — CR uguali alle righe —; i sei blocchi col `--check` sul
repository, uno per uno, e **in sequenza** con la simulazione della sessione che ha finito il piano — P-19.

### `extract.py` — copia nello scratchpad i pezzi di questo piano, parola per parola; l'unico che si copia a mano

````python
"""extract.py -- copy the tools, the text of ADR-0040 and the blocks of this plan into files, word for word.

Usage: PYTHONIOENCODING=utf-8 python extract.py <plan.md> <outdir>

Each piece is the fence (four backticks and a language) that follows the ONE line of the plan starting with its
marker, up to the next line made of exactly four backticks. Refuses, writing nothing, if a marker is absent or not
unique, or its fence is missing. It is the only tool of this plan copied by hand, and it holds no backslash: the
channel of a tool call can change a backslash in silence.
"""
import io
import os
import sys

NL = chr(10)
FENCE = chr(96) * 4
PIECES = [
    ("### `apply_edits.py`", "apply_edits.py"),
    ("### `archive_head.py`", "archive_head.py"),
    ("### `replace_pointer.py`", "replace_pointer.py"),
    ("### `tables.awk`", "tables.awk"),
    ("Il testo di ADR-0040 —", "adr0040.md"),
    ("Il blocco E1,", "e1.txt"),
    ("Il blocco E2,", "e2.txt"),
    ("Il blocco E3,", "e3.txt"),
    ("Il blocco E4,", "e4.txt"),
    ("Il blocco E5,", "e5.txt"),
    ("Il blocco E6,", "e6.txt"),
    ("Il blocco POS1,", "pos1.txt"),
    ("Il blocco POS2,", "pos2.txt"),
    ("Il blocco POS3,", "pos3.txt"),
    ("Il blocco POS4,", "pos4.txt"),
    ("Il blocco POS5,", "pos5.txt"),
    ("Il blocco POS6,", "pos6.txt"),
    ("Il puntatore nuovo della §6 —", "pointer6.txt"),
]

plan, outdir = sys.argv[1:3]
lines = io.open(plan, encoding="utf-8", newline="").read().splitlines()
found = []
for marker, name in PIECES:
    hits = [k for k, ln in enumerate(lines) if ln.startswith(marker)]
    if len(hits) != 1:
        sys.exit(f"refused: {len(hits)} lines starting with [{marker}]")
    k = hits[0]
    start = next((m for m in range(k + 1, len(lines)) if lines[m].startswith(FENCE) and len(lines[m]) > 4), None)
    end = None if start is None else next((m for m in range(start + 1, len(lines)) if lines[m] == FENCE), None)
    if end is None:
        sys.exit(f"refused: no fence after [{marker}]")
    found.append((name, NL.join(lines[start + 1:end]) + NL))
for name, body in found:
    with io.open(os.path.join(outdir, name), "w", encoding="utf-8", newline="") as f:
        f.write(body)
    print(f"{name}: {body.count(NL)} lines")
````

### `apply_edits.py` — applica un blocco di modifiche, tutto o niente; col `--check` non scrive

````python
"""apply_edits.py -- apply a block of edits from the plan, all or nothing, keeping each file's line endings.

Usage: PYTHONIOENCODING=utf-8 python apply_edits.py [--check] <date> <edits.txt>

<edits.txt> is the fenced block a task of the plan dictates, copied verbatim into a file:

  # a comment
  @@ <path>          the file the following edits touch, relative to the repository root
  >> <anchor>        an exact single-line text that must occur EXACTLY ONCE in the file
  >= <prefix>        the anchor is the WHOLE line that starts with <prefix>; exactly one such line
  ++ <text>          append: if the anchor ends with " |" (the end of a table cell) the text goes
                     before that bar after one space, otherwise right after the anchor after one space
  == <text>          replace the anchor with the text
  +P <text>          a new paragraph after the anchor, which must end its line: anchor, blank line, text
  +L <text>          a new line right after the anchor, which must end its line (a table row)
  ^^ <line>          an exact whole line, unique in the file: the block that follows goes BEFORE it,
                     followed by one blank line
  +E                 the block that follows goes at the END of the file, after one blank line
  <<<                a block runs from the line after <<< to the line before >>>; an operation whose
  >>>                text is empty and whose next line is <<< takes the block as its text

Every occurrence of the literal <data> in a text or block becomes <date>. Nothing else is expanded.
Every anchor is checked against the file AS THE PREVIOUS EDITS OF THE SAME BLOCK LEFT IT; if one
anchor is absent or not unique the whole block is refused and NO file is written. Each file is
written once, fully built first, on a temporary file and then os.replace -- a rename cannot fail
halfway (gotcha #82). A CRLF file gets its texts converted to CRLF. --check does everything but write.
"""
import io
import os
import sys

args = sys.argv[1:]
check = bool(args) and args[0] == "--check"
if check:
    args = args[1:]
date, edits_path = args
lines = io.open(edits_path, encoding="utf-8", newline="").read().replace("\r\n", "\n").split("\n")

files = {}  # path -> [content, crlf]
order = []
current = None
anchor = None
errors = []
done = 0
i = 0


def load(path):
    if path not in files:
        raw = io.open(path, encoding="utf-8", newline="").read()
        files[path] = [raw, "\r\n" in raw]
        order.append(path)


def conv(text, crlf):
    text = text.replace("<data>", date)
    return text.replace("\n", "\r\n") if crlf else text


def take_block():
    """Read the <<< ... >>> block starting at lines[i]; return its text, or None."""
    global i
    if lines[i:i + 1] != ["<<<"] or ">>>" not in lines[i + 1:]:
        return None
    j = lines.index(">>>", i + 1)
    text = "\n".join(lines[i + 1:j])
    i = j + 1
    return text


while i < len(lines):
    line = lines[i]
    i += 1
    if not line.strip() or line.startswith("#"):
        continue
    op, text = line[:2], line[3:]
    if op == "@@":
        current = text.strip()
        anchor = None
        if not os.path.isfile(current):
            errors.append(f"no such file: {current}")
            current = None
        else:
            load(current)
        continue
    if current is None:
        errors.append(f"line {i}: an edit with no file")
        continue
    content, crlf = files[current]
    eol = "\r\n" if crlf else "\n"
    if op == ">>":
        anchor = conv(text, crlf)
        n = content.count(anchor)
        if n != 1:
            errors.append(f"{current}: {n} occurrences of the anchor [{text[:70]}]")
            anchor = None
        continue
    if op == ">=":
        prefix = conv(text, crlf)
        found = [ln for ln in content.split(eol) if ln.startswith(prefix)]
        if len(found) != 1:
            errors.append(f"{current}: {len(found)} lines starting with [{text[:70]}]")
            anchor = None
        else:
            anchor = found[0]
        continue
    if op == "+E":
        block = take_block()
        if block is None:
            errors.append(f"line {i}: +E must be followed by a <<< >>> block")
            continue
        body = content if content.endswith(eol) else content + eol
        files[current][0] = body + eol + conv(block, crlf) + eol
        done += 1
        continue
    if op == "^^":
        target = conv(text, crlf)
        block = take_block()
        if block is None:
            errors.append(f"line {i}: ^^ must be followed by a <<< >>> block")
            continue
        n = (eol + content).count(eol + target + eol)
        if n != 1:
            errors.append(f"{current}: {n} whole lines equal to [{text[:70]}]")
            continue
        pos = (eol + content).index(eol + target + eol)
        files[current][0] = content[:pos] + conv(block, crlf) + eol + eol + content[pos:]
        done += 1
        continue
    if op not in ("++", "==", "+P", "+L"):
        errors.append(f"line {i}: unknown operation [{op}]")
        continue
    if text == "":
        block = take_block()
        if block is not None:
            text = block
    if anchor is None:
        errors.append(f"line {i}: {op} with no valid anchor before it")
        continue
    new = conv(text, crlf)
    if op in ("+P", "+L") and not content.split(anchor, 1)[1].startswith(eol):
        errors.append(f"{current}: {op} anchor does not end its line")
        anchor = None
        continue
    if op == "++":
        repl = anchor[:-2] + " " + new + " |" if anchor.endswith(" |") else anchor + " " + new
    elif op == "==":
        repl = new
    elif op == "+P":
        repl = anchor + eol + eol + new
    else:
        repl = anchor + eol + new
    files[current][0] = content.replace(anchor, repl, 1)
    anchor = None
    done += 1

if errors:
    for e in errors:
        print("REFUSED:", e)
    sys.exit(f"refused: {len(errors)} problem(s), no file written")
if not check:
    for path in order:
        tmp = path + ".tmp"
        with io.open(tmp, "w", encoding="utf-8", newline="") as f:
            f.write(files[path][0])
        os.replace(tmp, path)
print(f"{'checked' if check else 'applied'}: {done} edits in {len(order)} files")
````

### `archive_head.py` — archivia parola per parola la riga della data del compendio e, con `--pointer`, il puntatore della §6

````python
"""archive_head.py -- copy the compendium's date line (and, with --pointer, the pointer of §6) word for word
to the end of docs/archivio/stato-storico.md, BEFORE they are rewritten.

Usage: PYTHONIOENCODING=utf-8 python archive_head.py [--pointer] "<heading>"

The date line is the one line that starts with "**Aggiornato il ". The pointer runs from the line that starts with
"⏭️ **IL PROSSIMO PASSO" to the line before the first one, after it, that starts with "⏳ " -- the cut the archive
has used since 2026-09-28 ("il ⏭️ della §6 dal suo inizio alla riga prima del ⏳"). Links are rewritten for
docs/archivio/: "](archivio/x" -> "](x", any other relative "](x" -> "](../x". The archive keeps its own line
endings; the file is written on a temporary file and then os.replace (gotcha #82). Refuses, writing nothing,
if a piece is not found exactly once.
"""
import io
import os
import re
import sys

args = sys.argv[1:]
pointer = bool(args) and args[0] == "--pointer"
if pointer:
    args = args[1:]
heading = args[0]
comp = io.open("docs/COMPENDIO.md", encoding="utf-8", newline="").read()
lines = comp.replace("\r\n", "\n").split("\n")

dates = [ln for ln in lines if ln.startswith("**Aggiornato il ")]
if len(dates) != 1:
    sys.exit(f"refused: {len(dates)} date lines")
pieces = [dates[0]]
if pointer:
    starts = [k for k, ln in enumerate(lines) if ln.startswith("⏭️ **IL PROSSIMO PASSO")]
    if len(starts) != 1:
        sys.exit(f"refused: {len(starts)} pointer starts")
    k = starts[0]
    end = next((m for m in range(k + 1, len(lines)) if lines[m].startswith("⏳ ")), None)
    if end is None:
        sys.exit("refused: no line starting with ⏳ after the pointer")
    pieces.append("\n".join(lines[k:end]).rstrip("\n"))


def relink(text):
    def fix(m):
        target = m.group(1)
        if target.startswith(("http", "../", "#", "/")):
            return m.group(0)
        if target.startswith("archivio/"):
            return "](" + target[len("archivio/"):]
        return "](../" + target
    return re.sub(r"\]\(([^)]+)", fix, text)


what = "L'intestazione del compendio" if not pointer else "Il puntatore «Il prossimo passo» e l'intestazione del compendio"
verb = "Vera il giorno in cui fu scritta.** Uscita" if not pointer else "Veri il giorno in cui furono scritti.** Usciti"
links = "col link riscritto" if not pointer else "coi link riscritti"
block = f"## {heading}\n\n⚠️ **{verb} dal compendio parola per parola, {links} per questa cartella.\n\n"
block += "\n\n".join(relink(p) for p in pieces) + "\n"

path = "docs/archivio/stato-storico.md"
raw = io.open(path, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw
body = raw.replace("\r\n", "\n")
if not body.endswith("\n"):
    body += "\n"
out = body + "\n" + block
if crlf:
    out = out.replace("\n", "\r\n")
tmp = path + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print(f"archived: {len(pieces)} piece(s) under «{heading}»; the heading starts with «{what}»")
````

### `replace_pointer.py` — riscrive il puntatore della §6, dopo `archive_head.py`

````python
"""replace_pointer.py -- replace the pointer of §6 of the compendium with the text of a file.

Usage: PYTHONIOENCODING=utf-8 python replace_pointer.py <date> <new-pointer.txt>

The pointer runs from the line that starts with "⏭️ **IL PROSSIMO PASSO" to the line before the first one, after it,
that starts with "⏳ " -- the same cut as archive_head.py, which must run FIRST. The literal <data> in the new text
becomes <date>. Keeps the compendium's line endings; writes a temporary file, then os.replace (gotcha #82). Refuses,
writing nothing, if a boundary is not found exactly once.
"""
import io
import os
import sys

date, new_path = sys.argv[1:3]
path = "docs/COMPENDIO.md"
raw = io.open(path, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw
lines = raw.replace("\r\n", "\n").split("\n")
starts = [k for k, ln in enumerate(lines) if ln.startswith("⏭️ **IL PROSSIMO PASSO")]
if len(starts) != 1:
    sys.exit(f"refused: {len(starts)} pointer starts")
k = starts[0]
end = next((m for m in range(k + 1, len(lines)) if lines[m].startswith("⏳ ")), None)
if end is None:
    sys.exit("refused: no line starting with ⏳ after the pointer")
new = io.open(new_path, encoding="utf-8", newline="").read().replace("\r\n", "\n").rstrip("\n")
new = new.replace("<data>", date)
out = "\n".join(lines[:k] + new.split("\n") + lines[end:])
if crlf:
    out = out.replace("\n", "\r\n")
tmp = path + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print(f"replaced: lines {k + 1}-{end} with {len(new.split(chr(10)))} line(s)")
````

### `tables.awk` — le righe di tabella che non hanno le colonne della loro intestazione

Il controllo delle tabelle del disegno, in un file perché porta le barre rovesciate — trappola 13. Il conto per file:
`awk -f "$S/tables.awk" <file> | cut -d: -f1 | sort | uniq -c`. **Non** rende vuoto — tre file hanno già righe così, e
nessuna è del piano: P-18 —; deve rendere **lo stesso** prima e dopo il compito.

````awk
FNR == 1 { t = 0; c = 0 }
/^```/ { c = !c; next }
c { next }
/^\|/ { l = $0; gsub(/\\\|/, "", l); n = gsub(/\|/, "|", l); if (!t) { t = 1; h = n; s = FNR } else if (n != h) print FILENAME ":" FNR ": " n " contro " h " (riga " s ")"; next }
{ t = 0 }
````

---

## Vincoli globali

Valgono per ogni compito, senza che il compito li ripeta. `<base>` è il commit in cui questo piano è nato, e lo dà un
comando: `git log --format=%h --diff-filter=A -- docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`.

| # | Vincolo | Da |
|---|---|---|
| 1 | **il merito approvato non si tocca**: le risposte D1–D20 e le sei sezioni del disegno, riletto dal proprietario. Se un passo lo violerebbe — una scorciatoia, una duplicazione, un fatto che non è più vero — ci si **ferma e lo si riporta**: è l'accettazione condizionata del proprietario, e vale finché regge | disegno, *«Le regole di questo lavoro»*; `anthropic-skills:decision-principles` |
| 2 | **nessun codice**: il comando D resta vuoto a ogni compito | disegno, sezione 4; riga 15 della Definizione di «fatto» |
| 3 | **della spec del sotto-progetto 1 si toccano due punti soli** — la riga `filesystem` della §2.3 e la testa della §6.6 —, e la **§8 non cambia** | disegno, 4.5; vincolo globale 1 del piano della chiusura |
| 4 | **documenti in italiano**, e un riferimento al codice dentro un documento in inglese, col **nome esatto del sorgente** | §1.0 della spec; gotcha #40 |
| 5 | **nessuna cifra nuova in prosa** senza il comando accanto; nei documenti della lista della guardia dei conteggi — `CLAUDE.md` e `HANDOFF.md` compresi — i numeri piccoli **a parole**, e nessuna cifra seguita da «ADR» fuori da un code span | `CLAUDE.md`; trappole 1 e 8 |
| 6 | ⛔ **i blocchi si applicano, non si riscrivono**: il `--check` prima; se un'ancora manca o non è unica il blocco è rifiutato **per intero** e nessun file è scritto — e allora ci si **ferma**: è una voce d'errata, non un rattoppo a mano | D15 |
| 7 | **i fine-riga si conservano per file**: si misurano col `git ls-files --eol` del Passo 1 di ogni compito — la colonna `w/…` dipende dalla macchina, e un'etichetta scritta qui mentirebbe sull'altra (E72 del piano della parte 2) —; dopo il compito i CR sono **quanti prima**, e la colonna `w/…` è quella del Passo 1. Un file **nuovo** nasce LF | `CLAUDE.md`; trappola 3 |
| 8 | **ogni conteggio si rifà col comando**: le cifre di questo piano sono istantanee del 2026-09-29, su `ff6f0e5` o sulla simulazione — P-19 | `CLAUDE.md`, gotcha #31 |
| 9 | **gli ADR sono append-only**: un rimando va in testa, prima di `## Context`; un secondo sotto il primo — D5; per ADR-0039 il rimando entra **nella cella** della riga del perimetro negativo — P-17; **nessuna riga preesistente** di un ADR cambia, salvo quella cella | `CLAUDE.md`; disegno, 3.1 e 3.3 |
| 10 | **il compendio resta sotto il tetto**: il comando C prima e dopo ogni compito che lo tocca; se va rosso si toglie prosa dalla §6, **non si alza il tetto** | §13 del compendio; gotcha #100; trappola 2 |
| 11 | **nessun link a un file che non esiste ancora**, fuori da `plans/`: il link ad ADR-0040 nasce col compito 1, che crea il file; un'**ancora** (`#…`) non si scrive mai | trappole 5 e 6 della §10 del compendio; trappola 7 |
| 12 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore**; `check-docs.sh` e il cancello girano **prima**, uno alla volta; il messaggio comincia con `knowledge-base-revisione(compito N): …` | `CLAUDE.md` |
| 13 | **il numeratore dei compiti** vive nella tabella della posizione e in nessun altro punto del repository | gotcha #68 |
| 14 | **il `grep` di Git Bash**: mai `-i` con più di un `-e` — va in crash, `Aborted`, e con lo stderr scartato sembra un niente —; ogni `grep -c` che deve rendere **0** si prova prima dove deve rendere **1**; e un `grep -c` conta le **righe**, non le occorrenze | trappole 5 e 10 |
| 15 | **la data**: `<data>` nei blocchi è il giorno dell'esecuzione del compito, lo stesso in tutto il compito; ADR-0040 porta quello del compito 1 | D2; P-8 |

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

✅ **IL PIANO È SCRITTO E PRE-CONTROLLATO, il 2026-09-29.** Ciò che il pre-controllo ha trovato sta nell'errata, e i blocchi
che ha cambiato sono provati in sequenza — P-19. ⏳ **L'esecuzione non è cominciata**: un compito per sessione — il
*«Come si riprende»*, in fondo.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | ADR-0040, e ciò che il cancello pretende con lui | — | ⏳ |
| **2** | i rimandi in testa a dieci ADR, e le loro voci nel compendio | — | ⏳ |
| **3** | il disegno del 2026-09-04, e la cattura in tutte le sue case | — | ⏳ |
| **4** | la spec del sotto-progetto 1 in due punti, design/09 e design/10 | — | ⏳ |
| **5** | `roadmap.md`, `tracciabilita.md` e la stella polare della GUI | — | ⏳ |
| **6** | la chiusura: gli indici, la data e il puntatore, la Definizione di «fatto» eseguita | — | ⏳ |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del [`COMPENDIO.md`](../../COMPENDIO.md). Qui resta la
**posizione** — la tabella, che chi esegue aggiorna nel commit del compito col suo blocco POS — e **come** si esegue. I
commit di un compito li trova `git log --oneline --grep='knowledge-base-revisione(compito N)'`.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro — e le sezioni del disegno che nomina.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-29.
3. Se il compito dice il falso — un `--check` che rifiuta, un Atteso che non torna — **ci si ferma e si riporta**: una
   divergenza è una voce d'errata prima di essere un rimedio.
4. `check-docs.sh` e il cancello girano **prima** di ogni commit, uno alla volta; il commit dice ciò che il compito ha fatto.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; nei compiti **1**, **2** e **3**
   rilegge ciascun rimando **contro l'ADR che lo ospita e contro i fratelli** della sua riga nella 3.1 del disegno —
   gotcha #59 —; nel **3**, **4** e **5**, ciascun richiamo contro la riga del disegno che lo detta.
6. Una seconda ondata di **sola prosa** la chiude il coordinatore a mano; dopo due ondate di prosa si chiude (gotcha #76).
7. ⛔ **Le scritture in parallelo non si fanno**: un compito per volta. E più di un subagente solo dopo aver detto al
   proprietario il costo, con la banda misurata, e avuto il sì (`CLAUDE.md`).
8. ⛔ **Il dispaccio viaggia con git**, perché un compito si riprende anche dall'altra macchina — D17. Prompt, rapporti e
   revisioni stanno in `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti-esecuzione/`, tracciata e
   creata al primo dispaccio, e il prompt di un compito vi nasce come **modello**, coi campi della macchina da riempire.
   Nella cartella di lavoro `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/`, ignorata, nascono i brief e
   scrivono implementatore e revisore; alla chiusura del compito il coordinatore copia nella cartella tracciata il prompt
   spedito, il rapporto, il prompt del revisore e la revisione, e li committa.

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota.** La riempiono il pre-controllo e l'esecuzione: una voce per difetto trovato, col testo corretto, la data
e chi l'ha trovata. Ciò che la **scrittura** del piano ha trovato sta nella sezione dopo, perché è già dentro i compiti.
Le voci si chiamano **ER-1**, **ER-2**…, e non *E1*…: *E1*…*E6* sono i nomi dei blocchi — deciso nel pre-controllo, il
2026-09-29.

| # | Voce |
|---|---|
| — | *(vuota alla scrittura, 2026-09-29)* |
| **ER-1** | ✅ **Compito 4, Passi 1 e 3 e criterio — il passo visivo, deciso dal proprietario nel pre-controllo, il 2026-09-29: A.** I due diagrammi che il blocco E4 cambia — il primo di design/09, il secondo di design/10 — sono disegnati prima e dopo il blocco, applicato a una copia nello scratchpad: in un widget della chat con mermaid 11, sul precedente della passata sui diagrammi del 2026-09-08, e nel browser integrato, dove la stessa pagina li disegna tutti e quattro senza errori e mermaid 12.0.0 — l'ultima su npm quel giorno — li legge tutti e quattro. Il proprietario li ha guardati e accettati, sotto la condizione che siano coerenti e corretti con ciò che esiste: riletti contro ADR-0040, la 5.4 del disegno, la tabella di design/10 dopo il blocco e il codice — `declare_scope`, `preserve`, `restore` e `CheckpointId` ci sono con quei nomi. ⛔ **Che cosa entra nel compito 4:** la sonda dell'impronta qui sotto, al **Passo 1** — atteso `93abe5d5066ebdd9` e `4b8d9f87d0ca24e8` — e al **Passo 3** — atteso `558f5f1e51c0be72` e `0836d966284fab09`, le impronte dei sorgenti che il proprietario ha visto —; e nel criterio di chiusura la riga *«le due impronte del Passo 3 sono quelle di ER-1»*. Se al Passo 3 un'impronta è diversa ci si **ferma prima del commit**: il diagramma non è quello visto, e servono una voce d'errata, il disegno rifatto e lo sguardo del proprietario. Con l'impronta uguale un secondo sguardo non aggiunge controllo, e non si chiede. La sonda rende lo stesso su un file CRLF, e non porta barre rovesciate — trappola 13. La rilancia anche il compito 6, al Passo 4, per le righe 12 e 17 della Definizione di «fatto»: le due impronte di dopo |
| **ER-2** | ⚠️ **Compito 1, Passo 2 e blocco E1 — *«la riga «artefatti» delle due tabelle»* dice il falso di una delle due.** Trovata dal pre-controllo del 2026-09-29, rileggendo ADR-0022 per intero. Nella tabella del *Context* la riga è «artefatti prodotti» — non ricostruibili, file dell'utente, crescono con l'uso —, e con ADR-0040 resta vera; ciò che ADR-0040 cambia sta nella tabella del punto 1 della *Decision*, la colonna «Nel backup». Il disegno, nella 3.1 e nella 3.3, dice solo *«la riga «artefatti»»*: le parole in più erano del piano, e in un ADR, che è append-only, sarebbero rimaste. ✅ **Corretto nei recinti**, nella riga `Modifica:` del testo di ADR-0040 e nel rimando di ADR-0022 del blocco E1: *«nella tabella del punto 1 della* Decision*, la riga «artefatti» e le guide della riga «configurazione, guide, profili»; e la conseguenza …»*. Le voci del compendio dicevano già *«la riga «artefatti»»*, e non cambiano. La simulazione in sequenza è rifatta — P-19 |
| **ER-3** | ⚠️ **Compito 1 — dal commit di ADR-0040 due commenti del codice dicono il falso, e il piano non li può toccare.** Trovata dal pre-controllo del 2026-09-29: è la riga 6 della tabella di `CLAUDE.md`, ciò che smentisce può stare in un commento. In `crates/daemon/src/main.rs`, sopra `JOURNAL_PATH` e sopra `LAYOUT_PATH`, il commento dice che dove stia la cartella dati per utente è *«a decision no ADR has taken»*: ADR-0040, punto 1, la prende. La consegna del brainstorming, in archivio, lo aveva scritto — i due percorsi e il commento *«cadono con D4»* —, e la 4.2 del disegno dà la cartella dati al 6; mancava che fino ad allora i commenti dicono il falso. Il vincolo 2 vieta il codice: la voce è **registrata** in *«Le voci aperte che questo piano SA»*, col suo chiusore. Il `grep` sulla frase ne trova uno solo, perché sopra `JOURNAL_PATH` la frase va a capo dopo *«no»* |
| **ER-4** | ⚠️ **Compito 1, Passi 1 e 4 — la riga del rimando nella voce di ADR-0022 del compendio non ha una sonda.** Trovata dal pre-controllo del 2026-09-29, la seconda domanda: le altre modifiche del blocco E1 hanno la loro — la guardia dei totali, `superato in parte`, `modificato da ADR-0040` —, questa no. ✅ **La sonda:** `grep -c 'modificato in parte da ADR-0040' docs/COMPENDIO.md`, che rende `0` al Passo 1 e `1` al Passo 4; provata il 2026-09-29 nella direzione dello `0`, e nel blocco E1 la frase c'è una volta |
| **ER-5** | ⚠️ **Compito 5, blocco E5 — la dipendenza vecchia dell'11 vive in altre due case, e nessun blocco le tocca.** Trovata dal pre-controllo del 2026-09-29, cercando *«5, 6 e 9»* e le sue forme fuori da archivio e piani: oltre ad ADR-0022 — il rimando di E1 —, alla riga 11 — E5 — e alla §8.5.2 della spec — P-5 —, la dicono la voce chiusa *«Dove vive backup e ripristino»* di `roadmap.md`, *«chiusa il 2026-08-07: sotto-progetto 11, dopo 5, 6 e 9»*, e la riga 2 della tabella di F2 in `HANDOFF.md`, *«dipendente da 5, 6 e 9»*. ✅ **La roadmap:** un richiamo in fondo alla cella, nel blocco E5 — il file che il compito tocca, dove la riga 11 direbbe *«6, 9»* poco sopra —; quindi al Passo 2 `checked: 32 edits in 3 files` e `applied: 32 edits in 3 files`, e il comando E sulla roadmap rende `8` e non `7`, al Passo 3 del compito 5 e ai Passi 1 e 5 del compito 6. ✅ **`HANDOFF.md` non si tocca:** la riga 14 della Definizione di «fatto» ne vuole solo i totali, e la riga è il consuntivo della §8.5.2 — sta con P-5, **registrata** col proprietario come chiusore. La simulazione in sequenza è rifatta — P-19 |
| **ER-6** | ⚠️ **Compito 1, Passi 1 e 4 — il comando E sull'archivio rende `6`, non `5`.** Trovata dalla simulazione del pre-controllo, il 2026-09-29: la chiusura del pre-controllo ha copiato in `docs/archivio/stato-storico.md` il puntatore della §6 com'era, e il puntatore porta il link al disegno. Al Passo 4 resta `6`; al compito 6 l'archivio riceve il puntatore di adesso, e passa a `7` — quel Passo lo annota, non lo prescrive |
| **ER-7** | ⛔ **Compito 5, Passi 1 e 3 — la sonda delle sei righe della roadmap ne conta nove, e l'«Atteso» `6` non torna mai.** Trovata dalla simulazione del pre-controllo, il 2026-09-29, e vera già su `ff6f0e5`: la sonda prende anche le righe 5 e 6 della tabella dei traguardi e la riga 3 della tabella della GPU della GUI. ✅ **La sonda, ristretta alla sezione dei sotto-progetti**, sta sotto questa tabella, accanto a quella di ER-1 — la trappola 4 —: `6` al Passo 1 e `6` al Passo 3, misurato sul repository e sulla simulazione; su una sezione che non c'è rende `0` |
| **ER-8** | ⚠️ **Compito 6 — dal 2026-09-30 il puntatore della §6 porta il metodo con la decision map, e `replace_pointer.py` lo riscrive intero.** Trovata aprendo il fronte del metodo, il 2026-09-30, prima del compito 1: il puntatore dice che l'esecuzione di questo piano **aspetta**, e rimanda alla [consegna del metodo](../specs/2026-09-30-metodo-decision-map-design.md). Il testo nuovo del puntatore, al compito 6, **conserva** quel rimando finché il metodo non è deciso: senza questa voce `archive_head.py` e `replace_pointer.py` lo toglierebbero senza che nulla diventi rosso. Quando questo piano si esegue lo decide il ticket *«Il piano dei documenti si esegue adesso o aspetta il metodo nuovo?»* della mappa del metodo. ✅ **Richiamo del 2026-09-30, dal ticket:** il proprietario ha deciso **adesso**, e il ticket è chiuso; il puntatore non dice più che l'esecuzione aspetta, e porta ancora il rimando alla consegna del metodo, che al compito 6 si conserva come qui sopra finché il metodo non è deciso |
| **ER-9** | ⚠️ **Compito 1, Passi 1 e 4 — il comando E sull'archivio non rende `6`, e il suo numero non si prescrive.** Trovata dal coordinatore il 2026-09-30, rimisurando il Passo 1 prima del primo dispaccio: su `dd0e265` rende `8`. Dopo la chiusura del pre-controllo, due chiusure del fronte del metodo — `a9e8265` e `fc81594` — hanno copiato in `docs/archivio/stato-storico.md` il puntatore della §6 com'era, con `archive_head.py`, e il puntatore porta il link al disegno: `6`, poi `7`, poi `8` — la misura sotto questa tabella. Ogni sessione che archivia il puntatore ne aggiunge uno, quindi il numero di ER-6 è **tolto, non riallineato** — gotcha **#31** —: al Passo 1 il valore sull'archivio si **annota**, e al Passo 4 è **lo stesso**, perché il compito 1 archivia la sola riga della data, che il link non lo porta. Al compito 6 vale ciò che ER-6 dice già: si annota. ✅ **La simulazione in sequenza, rifatta** il 2026-09-30 su `dd0e265`, coi blocchi datati quel giorno — P-19 —: i sei blocchi applicati in ordine, `check-docs.sh` → `OK` dopo ciascun compito, il comando D vuoto; il margine del compendio — comando C — parte da `8103`, e il più basso è `4390`, dopo i compiti 3–5 |
| **ER-10** | ⚠️ **Compito 1, Passi 2 e 4 — che l'archivio tenga la riga della data DI PRIMA non lo prova niente.** Trovata dal coordinatore il 2026-09-30, la seconda domanda di `CLAUDE.md`: `archive_head.py` stampa `archived: 1 piece(s)`, ma se girasse **dopo** il blocco E1 — l'ordine che D10 vieta — l'archivio terrebbe la riga nuova, e nessun controllo diventerebbe rosso. ✅ **La sonda**, sotto questa tabella per la trappola 4: al Passo 4, prima del commit, rende `SAME` — l'ultima riga della data che l'archivio porta è quella di `HEAD`, col solo link riscritto per la cartella dell'archivio. Provata il 2026-09-30 nelle due direzioni su un `git worktree` nello scratchpad, poi tolto: `SAME` con l'ordine del compito, `DIFFERENT` con `archive_head.py` lanciato dopo il blocco. Il compito 6 la può rilanciare per la riga della data |

La sonda di ER-1, fuori dalla tabella perché è lunga: `H` è il programma, e i due argomenti sono il file e il numero del
blocco `mermaid` dentro il file.

```bash
H='import sys,hashlib,io;F=chr(96)*3;t=io.open(sys.argv[1],encoding="utf-8",newline="").read().replace(chr(13)+chr(10),chr(10)).split(chr(10));i=[k for k,l in enumerate(t) if l.startswith(F+"mermaid")][int(sys.argv[2])-1];j=next(m for m in range(i+1,len(t)) if t[m].startswith(F));print(hashlib.sha256(chr(10).join(t[i+1:j]).encode("utf-8")).hexdigest()[:16])'
python -c "$H" docs/design/09-l0-fisico.md 1; python -c "$H" docs/design/10-modello-dei-dati-durevoli.md 2
```

La sonda di ER-7, fuori dalla tabella per la trappola 4: la prima riga è quella del compito 5, che rende `9`; la seconda la
sostituisce.

```bash
grep -cE '^[|] (3|5|6|10|11|13) [|]' docs/roadmap.md
awk '/^## Sotto-progetti/{s=1;next} s&&/^## /{s=0} s' docs/roadmap.md | grep -cE '^[|] (3|5|6|10|11|13) [|]'
```

La misura di ER-9, fuori dalla tabella per la trappola 4: il comando E sull'archivio, commit per commit.

```bash
for c in 93ae91d a9e8265 fc81594 dd0e265; do printf '%s ' $c; git show $c:docs/archivio/stato-storico.md | grep -c '2026-09-28-knowledge-base-revisione-design'; done
```

La sonda di ER-10, fuori dalla tabella per la trappola 4: `S` è lo scratchpad, come nei compiti, e `HEAD` è ancora il commit
di prima del compito.

```bash
git show HEAD:docs/COMPENDIO.md | tr -d '\r' | grep -F '**Aggiornato il ' | sed 's#](archivio/#](#' > "$S/old-date.txt"
tr -d '\r' < docs/archivio/stato-storico.md | grep -F '**Aggiornato il ' | tail -1 | diff -q - "$S/old-date.txt" > /dev/null && echo SAME || echo DIFFERENT
```

---

## Ciò che la scrittura del piano ha trovato — e i compiti già portano

P-1…P-14 le ha trovate la prima sessione, misurate su `0c0d0d4`; P-15…P-19 la seconda, su `ff6f0e5` e nella simulazione
in sequenza. Il testo delle prime è quello della consegna in archivio, parola per parola.

| | Trovato | Che cosa ne segue |
|---|---|---|
| P-1 | `docs/HANDOFF.md` porta già **un** link al disegno — nel gotcha #141, dal commit `fedca31` —, e `docs/riferimenti.md` tre, le fonti della 3.4: la 6.2 dice che prima del piano nessun file da toccare ne porta, tranne il compendio | nessun controllo cambia: per `HANDOFF.md` il controllo è la sola guardia dei totali; il comando E vi rende uno prima e dopo |
| P-2 | **`docs/design/10-modello-dei-dati-durevoli.md`** dice ancora ciò che la revisione supera, e il disegno non lo nomina: `AMBITO` e `CHECKPOINT` «col 5» e *«l'implementazione vera col 5»* — D15 —; `CARTELLA_KB` *«in chiaro, nel backup»* — ADR-0040, e una politica che quel file dice di non ripetere —; le specie di `NODO_KB` e le frecce dell'`INDICE_MAPPA` — la riga 11 —; `GUIDA_APPROVATA` senza la fiducia di D9 | nel compito 4, blocco E4: un richiamo sotto l'intestazione della sezione, cinque etichette, tre righe della tabella — D8 |
| P-3 | il modulo **Backup** della stella polare della GUI elenca ciò che il backup non contiene — indici, pesi, segreti — e il punto 4 di ADR-0040 vi aggiunge la root, le zone e le copie: la 5.3 nomina sette punti, questo è l'ottavo | nel compito 5, blocco E5 — D9 |
| P-4 | ADR-0022 porta, nel rimando del 2026-08-07 in fondo, *«sotto-progetto 11 … dopo il 5, il 6 e il 9»*: la seconda risposta della 5.5 toglie il 5 | lo dice il rimando nuovo in testa ad ADR-0022, blocco E1 |
| P-5 | la **§8.5.2** della spec del sotto-progetto 1 dice *«Servono inoltre il filesystem reale, che arriva con il sotto-progetto 5»*: la §8 non si tocca | **registrata**, col proprietario come chiusore, accanto a V36 e Q22 della 6.5 del disegno |
| P-6 | la riga 3 e la riga 8 della sezione 2 nominano i due vicoli ciechi; le stesse frasi vivono anche nello **scartato** delle risposte 3 e 10 del disegno del 2026-09-04 | il richiamo di quelle due righe copre risposta e scartato, blocco E3 |
| P-7 | cinque ADR — 0009, 0010, 0011, 0022, 0038 — hanno già un rimando in testa, e **nessun** ADR ne ha ancora due | il secondo va sotto il primo, una riga vuota in mezzo, prima di `## Context` — D5 |
| P-8 | la `Date` di un ADR è il giorno in cui il file nasce: cinque su cinque — 0029, 0036, 0037, 0038, 0039 —, col `git log --diff-filter=A` | ADR-0040 porta la data dell'esecuzione del compito 1 — D2 |
| P-9 | in `AVVIO-CHAT.md` il totale vive **dentro** il messaggio recintato, e la guardia toglie solo i code span | il richiamo che cita la frase vecchia la mette in un code span, `le 39 ADR` — blocco E1 |
| P-10 | la riga *«Ultimo aggiornamento»* di `roadmap.md` si riallinea in ogni commit che tocca il file — P-8 del piano del 2026-09-04 | i blocchi E1, E5 ed E6 la prendono per prefisso, `>=`, e la riscrivono |
| P-11 | il conto per stato di `tracciabilita.md` rende `47 · 54 · 76 · 0 · 1` — il comando del suo riquadro | nessuna delle undici righe cambia stato: il conto resta identico dopo il compito 5 |
| P-12 | in un'etichetta `mermaid` il segnaposto `.<nomeapp>/` sarebbe letto come un tag HTML, e il repository scrive le etichette senza apostrofi | i diagrammi dicono *«la cartella nascosta alla root»*; i blocchi E4 non portano né parentesi angolari né apostrofi nelle etichette |
| P-13 | le due tabelle delle voci aperte della porta di qualità, lette sulla colonna di chi le chiude: **nessuna** ha questo piano o *«il proprietario, prima»* come chiusore; la T5-34 è la riga stantia di E94, la contraddizione C-S5-3 | nessuna voce sbarra il piano |
| P-14 | il comando della guardia dei totali rende otto righe — quattro in `HANDOFF.md`, una in `roadmap.md`, due nel compendio, una in `AVVIO-CHAT.md` — tutte a trentanove | il blocco E1 le porta a quaranta, e in `AVVIO-CHAT.md` toglie la cifra: la risposta A alla prima domanda della 6.1 |
| P-15 | il **comando A** della 6.2 del disegno conta le **righe**, e in ADR-0039 il rimando nuovo entra nella riga che porta già quello del 2026-09-05 — la cella del perimetro negativo —: renderebbe `1` prima e dopo, misurato nella simulazione | il piano conta le **occorrenze** — il comando A della Definizione di «fatto» —, che su ADR-0039 rende `1` e poi `2`; la 6.2 del disegno riceve un richiamo datato, con la chiusura della sessione di scrittura — D24 |
| P-16 | il segnaposto `<data>` esiste già, letterale, una volta nel disegno dei gesti e una in quello del 2026-09-04 | un `<data>` rimasto non si cerca col solo `grep -l`: si contano le occorrenze prima e dopo, e restano **uguali** — compito 3 |
| P-17 | nella cella di ADR-0039 il testo nuovo entra **dentro** una riga, e `git diff` la dà tolta e rimessa: *«nessuna riga tolta negli ADR»* renderebbe `1` dopo il compito 3 | per la cella, la sonda del piano del 2026-09-04 (E9): una riga `-`, una `+`, e la `+` comincia con la `-` meno la sua barra finale — misurata nella simulazione |
| P-18 | il controllo delle tabelle rende già righe fuori colonna in tre file — `HANDOFF.md`, il disegno del 2026-09-04, la stella polare —, e nessuna è del piano: il conto per file è lo stesso prima e dopo i sei compiti, nella simulazione | le prove delle tabelle confrontano il **conto per file** col Passo 1, e non pretendono il vuoto — D25 |
| P-19 | **la simulazione in sequenza**, il 2026-09-29, su un `git worktree` a `ff6f0e5` con la chiusura della sessione di scrittura applicata, e `<data>` = `2026-09-30`: i sei blocchi `checked` e applicati in ordine — 17, 20, 59, 24, 31 e 6 modifiche —; `check-docs.sh` → `OK` dopo **ciascun** compito, anche dopo il 1, che è quello che lo tocca; il comando D vuoto; il margine del compendio sempre positivo, il più basso dopo i compiti 3–5 | la prova che il `--check` a uno a uno non dà. Si rifà quando una voce d'errata cambia un blocco, o un compito tocca un file che un altro blocco ancora |

---

## Le decisioni prese scrivendo il piano

⛔ **Sono del coordinatore, non del disegno, e il proprietario può ribaltarle**; chi esegue le ribalta portando la misura
che le smentisce — è ciò per cui esiste l'errata. D1…D18 le ha prese la prima sessione, e il testo è quello della
consegna in archivio; D19…D25 la seconda.

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| D1 | **sei compiti**: 1, ADR-0040 e ciò che il cancello pretende con lui — il rimando in ADR-0022, l'indice, la voce, i totali, la §13, `CLAUDE.md`, `AVVIO-CHAT.md`; 2, i rimandi in testa a dieci ADR con le loro voci; 3, il disegno del 2026-09-04 e la cattura in tutte le sue case — ADR-0039, la sua voce, il disegno dei gesti; 4, la spec, design/09 e design/10; 5, roadmap, tracciabilità e stella polare; 6, la chiusura | `check-docs.sh` è rosso finché ADR-0040 non ha voce, riga d'indice e totali, quindi nascono in un commit; un rimedio si chiude su tutte le case della frase in un commit — la cattura. Costo: compiti più grandi di quelli del 2026-09-04 |
| D2 | ADR-0040 si chiama `0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md`, col titolo della 3.2, e la sua `Date` è il giorno del compito 1 | P-8; la 3.2 lascia titolo e nome al piano. Costo: un rinomino |
| D3 | la testa di ADR-0040 porta un quarto punto, *«Modifica»*, accanto a `Status`, `Date` e `Deciders`; il rimando di ADR-0022 finisce con *«Le altre righe reggono»* invece della frase sulla non-superazione | la forma *«Amends»* di `adr-tools`, letta per la 3.3, sta nella testa; la 3.3 dice *«al posto della frase»*. Costo: una riga |
| D4 | la riga di `CLAUDE.md` nomina il terzo caso con la forma di ADR-0040 su ADR-0022 | la 3.3. Costo: una frase |
| D5 | un secondo rimando in testa va sotto il primo, prima di `## Context` | P-7: l'ordine delle date. Costo: nessuno |
| D6 | nel disegno del 2026-09-04 ogni richiamo va **nella cella o nel capoverso** che porta le parole superate; una riga della sezione 2 che nomina più posti ne dà uno a ciascuno; la tabella della 4.2 ne riceve uno solo, nella riga dei nodi, che nomina frecce e segnali | la sezione 2 del disegno, *«nella riga stessa»*. Costo: molti richiami corti |
| D7 | il richiamo in testa al disegno del 2026-09-04 va subito sotto il capoverso di stato | chi apre il file deve leggerlo per primo. Costo: nessuno |
| D8 | design/10 entra nel compito 4, e design/09 si riscrive sul precedente del 2026-09-08 — richiamo in testa alla sezione, diagramma e tabella —, col percorso della root nella configurazione, il punto 1 di ADR-0040 | P-2; la 5.4 vuole design/09 *«riallineato ad ADR-0040»*. Costo, se il proprietario lo vuole fuori: un blocco da togliere |
| D9 | il modulo Backup entra nel compito 5 | P-3. Costo: un richiamo da togliere |
| D10 | la riga della data del compendio si riscrive al compito 1 — per il piano in esecuzione — e al 6, e ogni volta la riga di prima va in archivio con `archive_head.py`; i compiti 2 e 3 non la toccano | il precedente del 2026-09-04: il suo compito 2 non la toccò. Costo: fra il 2 e il 6 la data può restare indietro di qualche giorno, e la riga lo dice rimandando alla tabella della posizione |
| D11 | in `tracciabilita.md` i richiami si appendono all'ultima cella, e nessuno stato cambia; *«Backup ed export dei dati»* non si tocca | *«solo l'irriproducibile»* resta vero: dice *solo*, non *tutto*. Costo: una riga, se il proprietario la vuole |
| D12 | nella §12 del compendio il disegno e il piano entrano nella riga della knowledge base, non in una riga loro | il tetto. Costo: una riga lunga |
| D13 | in `README.md` il disegno entra nella tabella «Specifiche», sotto la riga del disegno del 2026-09-04 | la forma dei disegni. Costo: nessuno |
| D14 | `riferimenti.md` non si tocca | le fonti della 3.4 ci sono già, P-1. Costo: nessuno |
| D15 | tre attrezzi al posto di `replace_unique.py`: `apply_edits.py`, che applica un blocco tutto o niente e si prova prima col `--check`; `archive_head.py` e `replace_pointer.py`, per la testa del compendio | una novantina di modifiche puntuali: le coppie scritte a mano sarebbero state una novantina di occasioni di sbagliare, e il `--check` le ha provate tutte sul repository vero. Provati nelle due direzioni su file di prova, e su un file CRLF — CR uguali alle righe. Costo: tre attrezzi da leggere |
| D16 | il *«Come si riprende»* del disegno resta nel disegno; questo piano ha il suo | il precedente dei disegni del 2026-09-03, del 2026-09-04 e del 2026-09-22: un disegno tiene la sua ultima chiusura. Costo: nessuno |
| D17 | il dispaccio dell'esecuzione viaggia con git, in `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti-esecuzione/`, creata al primo dispaccio | il punto 8 del piano del design system, richiesta del proprietario del 2026-09-24. Costo: una cartella |
| D18 | la cella *«Dipende da»* della riga 11 della roadmap si riscrive, *«6, 9»*, col richiamo che cita *«5, 6, 9»* | la 5.1: nelle tabelle di stato la cella si riscrive. Costo: nessuno |
| D19 | un quarto attrezzo, `extract.py`, copia dal piano nello scratchpad ogni pezzo — i tre attrezzi, `tables.awk`, il testo di ADR-0040, i blocchi, il puntatore — per **marcatore**; si copia a mano, e non porta barre rovesciate | ricopiare a mano centocinquanta righe di blocco è dove il canale di un tool muta una barra in silenzio — la trappola 13 —; il rifiuto non scrive niente. Costo: un attrezzo da leggere, e i marcatori da tenere unici |
| D20 | i blocchi stanno **dentro** i loro compiti, gli attrezzi in una sezione della testa; la sezione *«Il materiale verificato»* si scioglie: i recinti passano parola per parola, e la consegna che li accompagnava va in archivio | un compito si legge *«tutto e nient'altro»*, e il subagente riceve il compito; i piani del 2026-09-04 e del design system tengono gli attrezzi in testa. Costo: nessuno — la simulazione ha girato sui pezzi estratti dal piano finito |
| D21 | la posizione si aggiorna con un blocco piccolo per compito, POS1…POS6, che riscrive la riga intera; la colonna *Commit* dice quanti, a parole, e il commit lo trova `git log --grep` | un commit non conosce il proprio hash: il piano del design system lo faceva scrivere al compito dopo (E79), una complicazione senza guadagno. Costo: la descrizione del compito vive due volte, nella tabella e nel suo blocco — corta apposta |
| D22 | il puntatore nuovo del compito 6 dice la revisione chiusa e i due fronti che vengono — il 13, sbarrato da AUD-004, col perimetro della 4.2, e il brainstorming dei modelli decisionali — **senza** metterli in ordine; il puntatore della chiusura di questa sessione cambia una frase sola, quella del piano | il proprietario non li ha collocati l'uno rispetto all'altro: il paragrafo 🆕 della §6 del compendio. Costo: nessuno |
| D23 | la consegna della prima sessione va in `archivio/consegna-piano-knowledge-base-revisione-documenti.md`, parola per parola coi link riscritti per la cartella | `CLAUDE.md`: un documento vivo tiene una chiusura sola; il precedente è `archivio/consegna-piano-design-system.md`. Costo: un file nuovo in archivio |
| D24 | la Definizione di «fatto» è quella della 6.2 con una colonna in più, il compito, e le righe di P-2 e P-3; il comando A conta le **occorrenze**, e la 6.2 del disegno riceve un richiamo datato che lo dice | P-15; *«il piano la copia da qui»*: due copie di un comando che divergono in silenzio sono il gotcha #68, e il richiamo le tiene legate. Costo: un richiamo nel disegno chiuso |
| D25 | le prove delle tabelle confrontano il conto per file col Passo 1, invece di pretendere il vuoto | P-18. Costo: nessuno |

---

## La mappa dei file

⛔ **Nessuna etichetta di fine-riga**: la forma la dice il `git ls-files --eol` del Passo 1 di ogni compito — vincolo 7.

| File | Chi lo tocca | Responsabilità |
|---|---|---|
| `docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md` | 1, lo crea | l'ADR nuovo |
| `docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md` | 1 | il rimando «modificato da ADR-0040» |
| `docs/adr/0009-…`, `0010-…`, `0011-…`, `0012-…`, `0014-…`, `0015-…`, `0016-…`, `0024-…`, `0025-…`, `0038-…` | 2 | il rimando in testa |
| `docs/adr/0039-telecamera-come-sorgente-di-percezione.md` | 3 | il rimando **nella cella** del perimetro negativo |
| `docs/COMPENDIO.md` | 1, 2, 3, 6 | la riga della data (1, 6); la voce di ADR-0040, il rimando nella voce di 0022, i totali, la §8 e la §13 (1); le righe dei rimandi (2, 3); la §12 e il puntatore (6) |
| `docs/archivio/stato-storico.md` | 1, 6 | la riga della data (1), e la riga col puntatore (6), com'erano — le scrive `archive_head.py` |
| `docs/README.md` | 1, 6 | la riga di ADR-0040 nell'indice (1); il disegno fra le specifiche (6) |
| `CLAUDE.md` | 1 | la riga «ADR append-only» |
| `docs/HANDOFF.md` | 1 | i totali |
| `docs/AVVIO-CHAT.md` | 1 | la cifra tolta, col richiamo |
| `docs/roadmap.md` | 1, 5, 6 | il totale e l'«Ultimo aggiornamento» (1); le righe della 5.1 (5); la riga di questo piano (6) |
| `docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` | 3 | due richiami |
| `docs/superpowers/specs/2026-09-04-knowledge-base-design.md` | 3 | il richiamo in testa, e uno per ogni posto della sezione 2 |
| `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | 4 | la riga `filesystem` della §2.3, la testa della §6.6 |
| `docs/design/09-l0-fisico.md` · `docs/design/10-modello-dei-dati-durevoli.md` | 4 | il richiamo, i diagrammi, le tabelle |
| `docs/tracciabilita.md` | 5 | undici righe |
| `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` | 5 | i sette punti della 5.3, e il modulo Backup — P-3 |
| `docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md` | 6 | la spunta del punto 3 della 6.6 |
| questo piano | ogni compito | la posizione; l'errata, se serve |
| la cartella del dispaccio | il coordinatore | prompt, rapporti, revisioni — D17 |

⛔ **Nessun altro file.** In particolare non `crates/`, `gui/`, `scripts/`, non `riferimenti.md` (D14), non la §8 della spec.

---

## Le voci aperte che questo piano SA, e non chiude

⛔ **Lette prima di scrivere, come `CLAUDE.md` prescrive**: a sbarrare è la colonna *«Chi la chiude»* di
[`porta-di-qualita.md`](../../porta-di-qualita.md), e **nessuna** voce ha questo piano o *«il proprietario, prima»* come
chiusore — P-13. Si dichiarano perché chi esegue le sappia, non perché le tocchi.

| Voce | Dove vive | Chi la chiude |
|---|---|---|
| **AUD-004** — le difese di ADR-0015 per le skill | la 6.5 del disegno; il rapporto dell'audit | il **proprietario**, con un ADR suo, **prima del 13**. ⛔ Sbarra il 13, non questo piano |
| **X-2** e **X-4** | [`audit-2026-08-27.md`](../../audit-2026-08-27.md), le voci senza numero AUD | il proprietario |
| K10, K35, K44, K47, K49, K51, K52, K36, K12 | la 4.6 del disegno | il 6, il 5, il 3, il 4 — la sua colonna |
| K11, K17, K19, K21, K22, K29, K34, K38, K41, K48; il testo scelto a runtime nei record del giornale; V36, Q22 e la riga `filesystem` della §8.2.2 | la 6.5 del disegno | ciascuna col suo chiusore |
| **P-5** — la §8.5.2 della spec del sotto-progetto 1 dice ancora *«il filesystem reale, che arriva con il sotto-progetto 5»*; e il suo consuntivo, la riga 2 della tabella di F2 in `HANDOFF.md`, dice *«dipendente da 5, 6 e 9»* — ER-5 | la spec; `HANDOFF.md`; P-5 qui sopra, ER-5 | il **proprietario**, con la §8: il vincolo 3 la lascia com'è, e la riga 14 della Definizione di «fatto» vuole da `HANDOFF.md` solo i totali |
| **i due commenti di `crates/daemon/src/main.rs`**, sopra `JOURNAL_PATH` e sopra `LAYOUT_PATH`: dove stia la cartella dati per utente è *«a decision no ADR has taken»*, e dal compito 1 la prende ADR-0040, punto 1 — ER-3 | il sorgente; ER-3 | chi costruisce per primo la **cartella dati per utente**, e con lei riscrive i due percorsi e i loro commenti: il **6**, se nessuno la porta prima — la 4.2 del disegno, D15 |
| 🆕 **il passo visivo** — i due diagrammi che il compito 4 cambia, in design/09 e design/10, **nessuno li disegna**: il piano li controlla come testo, e il cancello non conosce mermaid — `grep -rn -i mermaid scripts/` non rende niente, 2026-09-29. Verificato che i nomi delle caselle nuove, `R` e `P`, non si scontrano con quelli che ci sono; **non** verificato che i due diagrammi si disegnino | posta al proprietario il 2026-09-29 in A/B — **A**, il consiglio: un passo nel compito 4, i due diagrammi disegnati prima e dopo nel browser integrato, lo sguardo del revisore e il suo; **B**: i soli controlli sul testo — e rimandata da lui, *«tempo al tempo»* | ✅ **deciso dal proprietario il 2026-09-29, A**, riposta per prima nel pre-controllo: la voce **ER-1** dell'errata |

---

## La Definizione di «fatto»

È quella della **6.2** del disegno — *«il piano la copia da qui»* —, con una colonna in più, il compito che scrive
l'artefatto, e le righe **17** e **18**, che P-2 e P-3 aggiungono (D24). La casa di **che cosa** dice ciascun artefatto
resta la sezione del disegno nella terza colonna; qui sta **come si prova** che c'è. Ogni controllo si prova nelle due
direzioni: prima del piano nessun file da toccare porta un link al disegno, tranne il compendio col puntatore della §6,
`HANDOFF.md` col gotcha #141 — P-1 —, l'archivio e il disegno stesso; dopo, ciascuno ne porta.

| # | Artefatto | Che cosa dice | Il controllo | Compito |
|---|---|---|---|---|
| 1 | il disegno del 2026-09-04: il richiamo in testa, e un richiamo per ogni riga della sezione 2 | la sezione 2 | la revisione, riga per riga contro la sezione 2; il link al disegno, comando E | 3 |
| 2 | i rimandi in testa agli ADR della 3.1, e il richiamo nella riga di ADR-0039 | la 3.1 | ciascun ADR riletto **contro i fratelli** della sua riga — gotcha #59; il comando A prima e dopo: un rimando in più per ADR, contato per **occorrenze** — P-15; la voce di ciascuno nella §5 del compendio con la riga che rimanda, **ADR-0039 compreso** | 1, 2, 3 |
| 3 | ADR-0040, e la sua riga nell'indice di `README.md` | la 3.2 e la 3.3 | `check-docs.sh`, rosso in tre modi finché manca qualcosa: la voce della §5 del compendio, la riga dell'indice, un totale vecchio. E, cercati: lo stato `Accepted`; la riga `Modifica:` in testa ad ADR-0040 e *«modificato da ADR-0040»* in testa ad ADR-0022; le *«Negative (accettate)»* | 1 |
| 4 | i totali degli ADR nei documenti di stato | la 3.3 | la guardia dei conteggi; e per `AVVIO-CHAT.md` il comando B, che prima rende una riga e dopo niente: la cifra è tolta, domanda 1 della 6.1 | 1 |
| 5 | la riga *«ADR append-only»* di `CLAUDE.md` | la 3.3 | la frase del terzo caso, `superato in parte`, cercata; nessuna cifra seguita da «ADR», perché `CLAUDE.md` è nella lista della guardia | 1 |
| 6 | il compendio: la voce di ADR-0040; le righe dei rimandi; la riga della §13; il disegno nella §12, nella riga della knowledge base; la data in testa | la 3.3; la §12 e la data, la 6.2 | `check-docs.sh`, col tetto e l'accoppiamento della §5; il margine misurato prima di scrivere, comando C; il comando E sul compendio rende almeno due righe, il puntatore e la §12. La data: la §13 del compendio dice perché è la riga più facile da lasciare indietro | 1, 2, 3, 6 |
| 7 | `README.md`: il disegno fra le specifiche, sulla forma della riga del disegno del 2026-09-04 | la 6.2 | il comando E su `README.md` | 6 |
| 8 | la spec del sotto-progetto 1: il richiamo nella riga `filesystem` della §2.3, e quello in testa alla §6.6 | la 4.5 | il comando E; ⛔ la **§8 non cambia**: il blocco A della §6 del compendio rende lo stesso prima e dopo, e il testo dalla §8 in giù ha la stessa impronta | 4 |
| 9 | `roadmap.md` | la 5.1 | ogni riga della 5.1 col rimando alla 4.2, contata col comando E; nessuna riga rinumerata; la riga 10 intatta | 5 |
| 10 | `tracciabilita.md` | la 5.2 | il comando della 5.2 rende ancora undici righe, e ciascuna porta il link; il conto per stato del riquadro non cambia — P-11 | 5 |
| 11 | la stella polare della GUI | la 5.3 | i sette punti, ciascuno col richiamo datato e il link | 5 |
| 12 | design/09 | la 5.4 | i sei punti; il diagramma e la tabella dicono la stessa cosa | 4 |
| 13 | il disegno dei gesti | la 5.8 | le due righe col richiamo; nessuna riga nuova nella sua tabella delle decisioni | 3 |
| 14 | `HANDOFF.md` | la 6.2 | solo i totali degli ADR, alla guardia | 1 |
| 15 | ⛔ **nessun codice** | la sezione 4 | il comando D, vuoto | ogni compito |
| 16 | i fine-riga di ogni file toccato | — | la trappola 3 | ogni compito |
| 17 | 🆕 design/10 — P-2 | il richiamo sotto *«Deciso e non costruito, per sotto-progetto»*, cinque etichette, tre righe della tabella | il comando E; nei diagrammi di design/09 e design/10 nessun apostrofo e nessun `nomeapp` — P-12 | 4 |
| 18 | 🆕 la stella polare, il modulo **Backup** — P-3 | ciò che ADR-0040 toglie dal backup del programma, detto quando il backup si crea | il richiamo col link ad ADR-0040, cercato | 5 |

I comandi, fuori dalla tabella per la trappola 4. **A** — i rimandi, uno in più per ADR a piano eseguito; conta le
**occorrenze**, non le righe — P-15:

```bash
for n in 0009 0010 0011 0012 0014 0015 0016 0022 0024 0025 0038 0039; do printf '%s ' $n; grep -o 'Rimando del' docs/adr/$n-*.md | wc -l; done
```

**B** — il totale in `AVVIO-CHAT.md`, come lo legge la guardia; a piano eseguito non rende niente:

```bash
sed 's/`[^`]*`//g' docs/AVVIO-CHAT.md | grep -nE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)'
```

**C** — il margine del compendio sotto il suo tetto, nella forma **CRLF**, che è quella che vincola: lo stesso numero su ogni
clone; **D** — il codice, da `<base>`; **E** — il link al disegno, su ciascun file:

```bash
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
git diff --stat <base>..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' <file>
```

E il comando della guardia dei totali, dalla 3.3 del disegno — la lista dei documenti è quella del passo *«ADR counts
declared in the prose»* di `scripts/check-docs.sh`, e se cambia là vale quella:

```bash
for f in docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md; do sed 's/`[^`]*`//g' "$f" | grep -nE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)' | sed "s|^|$f:|"; done
```

### Le trappole

Le righe 1–9 sono quelle della 6.3 del disegno, misurate il 2026-09-29; le altre le ha trovate la scrittura del piano.

| # | Trappola | Che cosa fare |
|---|---|---|
| 1 | il totale degli ADR non vive solo nel compendio e nella roadmap: la guardia legge più documenti, e oggi lo trova anche in `HANDOFF.md` e in `AVVIO-CHAT.md` | aggiunto il file di ADR-0040, `bash scripts/check-docs.sh` nomina ogni documento e ogni totale; in `AVVIO-CHAT.md` la cifra si toglie, comando B |
| 2 | il compendio ha un tetto, e il verde non è un margine; il margine che vincola è quello in forma **CRLF** — comando C | si rimisura **prima**, e ciò che è verbale va in archivio |
| 3 | i fine-riga sono misti fra i file di questo piano, e cambiano da una macchina all'altra | `git ls-files --eol` su ogni file prima di toccarlo — conta la colonna `i/` —; gli attrezzi scrivono con `newline=""`, su un temporaneo e `os.replace` — gotcha #82 —; il conto dei CR rifatto dopo |
| 4 | F10: un comando con la barra verticale dentro una cella di tabella | fuori dalla tabella |
| 5 | il `grep` 3.0 di Git Bash con `-i` e più di un `-e` **va in crash**, e bash stampa `Aborted`; con lo stderr scartato, o in fondo a una pipeline, sembra un niente | un `grep` per parola, e la controprova su un input che deve rendere uno |
| 6 | la §4 della spec del sotto-progetto 1 non è la porta dei file | il richiamo va nella riga `filesystem` della §2.3 — domanda 2 della 6.1 |
| 7 | ADR-0024 si chiama `…-ad-ambiti-dichiarati.md`: un link dedotto dal titolo è rotto | `ls docs/adr` prima del link |
| 8 | `CLAUDE.md` e `HANDOFF.md` sono nella lista della guardia | nelle righe nuove, i numeri piccoli a parole |
| 9 | il pre-controllo ha trovato un difetto in ogni compito dispacciato | ogni compito si rilegge contro i documenti di allora, non contro il disegno |
| 10 | `grep -c` conta le **righe**: dove un testo nuovo entra in una riga che c'è già — la cella di ADR-0039, le celle del disegno del 2026-09-04 — il conto non si muove | si contano le occorrenze, con `grep -o` e `wc -l` — il comando A, P-15 |
| 11 | dove il testo entra **dentro** una riga, `git diff` la dà tolta e rimessa | per la cella di ADR-0039, la sonda di P-17 |
| 12 | il segnaposto `<data>` c'è già in due disegni | il conto delle occorrenze prima e dopo, uguale — P-16 |
| 13 | le barre rovesciate non sopravvivono sempre al canale di un tool: un rattoppo di `apply_edits.py` passato in un heredoc, con le sequenze di fine-riga scritte con la barra rovesciata, si è fermato su un'asserzione; e una sonda in linea con la barra verticale protetta ha dato rossi falsi | un attrezzo o una sonda con barre rovesciate si scrive in un file, per intero, col tool di scrittura — `extract.py` ne è senza, `tables.awk` sta in un file —; nelle sonde dei compiti la barra verticale si scrive come classe di caratteri, fra parentesi quadre |
| 14 | un'ancora sulla prima riga di un blocco ricorrente non è unica: design/10 ha **due** recinti `mermaid` | si ancora all'intestazione della sezione, come fa il blocco E4 |
| 15 | `rev` non esiste nel Git Bash di questa macchina | le code di riga si stampano con Python |

---

⛔ **In ogni blocco di comandi dei compiti** la prima riga è `S=<scratchpad>; D=<data>`: `S` nella forma che Python capisce,
`D` il giorno del compito, **lo stesso** in tutti i suoi passi — lo stato della shell non sopravvive da una chiamata
all'altra, e un compito che passa la mezzanotte non cambia giorno a metà (vincolo 15). Gli Atteso sono le misure del
2026-09-29 — P-19 —, e si rimisurano (vincolo 8).

## Compito 1: ADR-0040, e ciò che il cancello pretende con lui

**Files:**
- Create: `docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md`
- Modify: `docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md` — il rimando in testa · `docs/README.md` — la riga dell'indice · `docs/COMPENDIO.md` — la riga della data, la voce di ADR-0040 nella §5, il rimando nella voce di 0022, i totali, la riga della §8, la riga nuova della §13 · `CLAUDE.md` — la riga *«ADR append-only»* · `docs/HANDOFF.md` — i totali · `docs/roadmap.md` — il totale e l'*«Ultimo aggiornamento»* · `docs/AVVIO-CHAT.md` — la cifra tolta, col richiamo · `docs/archivio/stato-storico.md` — la riga della data com'era, da `archive_head.py` · questo piano — la posizione
- Read: la 3.2, la 3.3 e la 3.4 del disegno, e la domanda 1 della 6.1; P-4, P-8, P-9, P-10, P-14; D2, D3, D4, D10; ADR-0022 **per intero**, coi rimandi del 2026-09-08 e del 2026-08-07; la testa di [ADR-0001](../../adr/0001-architettura-a-kernel-con-capacita-paritarie.md), la forma del rimando

- [ ] **Passo 1: le misure prima**

```bash
S=<scratchpad>; D=<data>
F="docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md docs/README.md docs/COMPENDIO.md CLAUDE.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md docs/archivio/stato-storico.md"
bash scripts/check-docs.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
for f in docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md; do sed 's/`[^`]*`//g' "$f" | grep -nE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)' | sed "s|^|$f:|"; done
ls docs/adr/0040-* 2>/dev/null | wc -l
grep -o 'Rimando del' docs/adr/0022-*.md | wc -l
grep -c '2026-09-28-knowledge-base-revisione-design' $F
```

Atteso: `OK`; la colonna `w/…` e i CR si **annotano**, sono l'invariante del Passo 5; le tabelle, una riga sola,
`31 docs/HANDOFF.md` — P-18 —; il margine positivo — la simulazione dava `8384`, e il compito ne consuma circa 1,6 KB —;
i totali, **otto** righe, tutte a trentanove — P-14 —; `0` file di ADR-0040: il compito non è già eseguito; `1` rimando in
ADR-0022; il comando E, `1` sul compendio e su `HANDOFF.md`, `5` sull'archivio, `0` sugli altri.

- [ ] **Passo 2: ADR-0040 dal suo recinto, e la riga della data in archivio — PRIMA del blocco**

```bash
S=<scratchpad>; D=<data>
sed "s/<data>/$D/" "$S/adr0040.md" > docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
grep -c '<data>' docs/adr/0040-*.md
grep -n 'Date:' docs/adr/0040-*.md
PYTHONIOENCODING=utf-8 python "$S/archive_head.py" "L'intestazione del compendio, com'era — archiviata il $D, al compito 1 del piano dei documenti della revisione della knowledge base"
```

Atteso: `0` segnaposto rimasti, e la riga `- **Date:**` col giorno; `archived: 1 piece(s) under «…»`. ⚠️ `archive_head.py` gira
**prima** del blocco, perché il blocco riscrive la riga della data: dopo, l'archivio avrebbe quella nuova (D10). Il file
nuovo nasce LF (vincolo 7).

Il testo di ADR-0040 — `<data>` è il giorno del compito, e lo mette il `sed` qui sopra; lo copia in `adr0040.md` `extract.py`:

````markdown
# ADR-0040: Dove vivono i dati, e che cosa salva il programma

- **Status:** Accepted
- **Date:** <data>
- **Deciders:** proprietario del progetto
- **Modifica:** [ADR-0022](0022-layout-dei-dati-per-natura-e-backup-dichiarato.md), in tre punti — nella tabella del
  punto 1 della *Decision*, la riga «artefatti» e le **guide** della riga «configurazione, guide, profili»; e la
  conseguenza *«La base di conoscenza sopravvive alla reinstallazione perché i documenti sorgente e la configurazione
  sono nel backup»*. Le altre righe reggono, e lo stato di ADR-0022 resta `Accepted`

## Context

[ADR-0022](0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) separa i dati per natura e mette nel backup gli
**artefatti** e le **guide**, perché sono file dell'utente e non si rifanno. La revisione della knowledge base — il
[disegno](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md), approvato dal proprietario il
2026-09-29 — ha cambiato la premessa: i file del proprietario non stanno in una cartella del programma. Stanno nella
**root**, una cartella qualsiasi scelta da lui, anche quella in cui tiene tutto, e nelle **zone di lavoro**, le cartelle
che apre per lavorarci, anche fuori dalla root; e il programma non è il solo a scriverli, perché il proprietario li
cambia da fuori, con qualunque strumento. Le risposte che decidono — le D e i K sono le risposte e i buchi di quel
disegno:

| | La risposta del proprietario | Data |
|---|---|---|
| D4 | i router in `.<nomeapp>/`, alla root; l'indice nella cartella dati del programma, fra i dati che si rifanno; i dati del programma nella cartella dati per utente del sistema | 2026-09-29 |
| D12 | la root è del proprietario, e il suo backup pure, come in Obsidian; il programma salva il giornale, la configurazione e i router — mai i segreti, e non l'indice —, e quando crea il backup dice che cosa resta fuori | 2026-09-29 |
| D17 | nessun file del proprietario nel backup del programma, la root **e** le zone di lavoro; la storia lunga è dei backup del proprietario e di git, come in Claude Code | 2026-09-29 |

E una cosa che nessun ADR aveva preso: **dove** stanno i dati del programma. Il sorgente lo dice in
`crates/daemon/src/main.rs` — *«where a per-user data directory belongs is a decision no ADR has taken»* —, e il daemon
apre oggi `journal.redb` e `layout.redb` nella cartella da cui parte.

**Lo stato dell'arte, letto alla fonte il 2026-09-29** e portato in [`riferimenti.md`](../riferimenti.md): in Claude
Code le copie per annullare stanno nella cartella dell'applicazione e non nel progetto, in chiaro, protette dai permessi
del sistema operativo, cancellate dopo trenta giorni per default, e in nessun backup; `adr-tools` lega due ADR con
*«Amends»* e *«Amended by»* quando il nuovo ne cambia una parte, accanto al superamento che cambia lo stato del vecchio.

### Alternative considerate

- **Router e indice insieme in `.<nomeapp>/`, alla root** — la B di D4. *Contro:* l'indice si rifà, e starebbe nella
  cartella del proprietario, dentro i suoi backup.
- **Anche gli artefatti e le guide della root nel backup del programma**, con un inseguitore di artefatti — la B di D12.
  *Contro:* il programma copierebbe una cartella che il proprietario salva già coi suoi strumenti, inseguendo file che
  cambiano da fuori.
- **Gli artefatti delle zone di lavoro nel backup** — la B di D17. *Contro:* le zone sono repo e progetti, che hanno git.
- **Le copie del checkpoint nel backup, cifrate**, perché l'annulla sopravviva al ripristino. *Contro:* va contro D17, e
  porta i file grandi due volte — K21.

## Decision

| # | Decide | Da |
|---|---|---|
| 1 | **I dati del programma** stanno nella cartella dati per utente del sistema — `%LOCALAPPDATA%\<nomeapp>\` su Windows, le cartelle XDG su Linux —, in sottocartelle per natura, come vuole ADR-0022; la configurazione porta il percorso della root | D4 · K1 |
| 2 | **I router** stanno in `.<nomeapp>/`, alla root, marcata nascosta dal modulo di piattaforma — su Windows il punto nel nome non basta; **l'indice** nella cartella dati del programma, fra i dati che si rifanno — su Linux, la cache —, uno per root | D4 · K30 · K32 |
| 3 | **I file del proprietario** — la root e le zone di lavoro, coi file che le run vi producono e con le guide — stanno al loro posto, riferiti dal giornale, e **non** entrano nel backup del programma: li salvano i backup del proprietario e git. ⚠️ **L'eccezione è una:** `.<nomeapp>/`, coi router, che il programma salva — punto 4 | D12 · D17 |
| 4 | **Il backup del programma** contiene il giornale, cifrato, la configurazione e i router; mai i segreti, né l'indice, né i pesi; e **quando lo crea dice che cosa resta fuori** — la root, le zone, le copie —: è il seguito di ADR-0022 | D12 · ADR-0022 |
| 5 | **Le copie del checkpoint** di [ADR-0024](0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) stanno nella cartella dati del programma, in una sottocartella loro; **in chiaro**, come i file che copiano; **fuori** dal backup; potate con la logica di [ADR-0018](0018-ritenzione-a-livelli-del-giornale.md), e mai quelle di un passo in dubbio non ancora riconciliato. Dopo un ripristino un passo di prima non si annulla più, e il programma lo dice: una copia assente non è una copia mai fatta — ADR-0018 | K53 · lo stato dell'arte |
| 6 | la cartella dati è un **percorso protetto e privato**: l'agente non ci scrive, non la legge, e non si indicizza. La regola vive nel rimando di [ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md), e qui si nomina | D19 |

## Consequences

- **Positive:**
  - Il programma non copia né insegue una cartella che non è sua: la root e le zone le salvano i backup del proprietario
    e git, e il backup del programma resta ciò che ADR-0022 voleva, l'irriproducibile **suo**.
  - I dati del programma hanno un posto deciso, quello che il sistema operativo prevede per un utente, invece della
    cartella da cui il daemon parte.
  - Chi ripristina sa **prima** che cosa il backup non porta — il seguito di ADR-0022, ora con la root, le zone e le
    copie.
- **Negative (accettate):**
  - **Il backup dei file del proprietario è suo:** il programma non lo fa, e lo dice quando crea il proprio.
  - **Dopo un ripristino** i passi di prima non si annullano; un passo in dubbio al momento del backup si riconcilia
    **senza** la sua copia, e allora si ferma e chiede — [ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md).
    ⚠️ Dedotto.
  - **Una copia in chiaro** resta finché non è potata, anche se nel frattempo il proprietario cancella il file o lo rende
    privato; la protegge il sistema operativo, come in Claude Code.
  - **I router finiscono in due backup**, quello del programma e quello del proprietario, perché stanno nella root.
  - **Una root spostata** fa rifare l'indice con una scansione.
- **Follow-up richiesti:**
  - Il limite di dimensione delle copie resta di ADR-0024 — K21.
  - Il tempo di ritenzione delle copie è un parametro consegnato
    ([ADR-0034](0034-parametri-di-decisione-consegnati-non-letti.md)), col valore a chi le costruisce.
  - Chi costruisce la cartella dati per utente e la cartella nascosta dei router lo dice la sezione 4 del disegno della
    revisione, che è la casa unica di chi costruisce che cosa: il sotto-progetto 6, se nessuno le porta prima.
````

- [ ] **Passo 3: il blocco E1 — prima il `--check`, poi davvero; e la posizione**

```bash
S=<scratchpad>; D=<data>
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$D" "$S/e1.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/e1.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/pos1.txt"
```

Atteso: `checked: 17 edits in 7 files`, poi `applied: 17 edits in 7 files`, poi `applied: 1 edits in 1 files`. Un rifiuto
non scrive niente: ci si ferma, ed è una voce d'errata (vincolo 6).

Il blocco E1, in `e1.txt`:

````text
# compito 1 -- ADR-0040 e cio' che il cancello pretende con lui (il file dell'ADR lo scrive il Passo 2, non questo blocco)
@@ docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — modificato da ADR-0040, in tre punti.**
> [ADR-0040](0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md), dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) —
> risposte D12 e D17, approvate dal proprietario il 2026-09-29 —, cambia **tre** punti di questo ADR: nella tabella del
> punto 1 della *Decision*, la riga **«artefatti»** e le **guide** della riga «configurazione, guide, profili»; e la
> conseguenza *«La base di conoscenza sopravvive alla reinstallazione perché i documenti sorgente e la configurazione
> sono nel backup»*. I file
> del proprietario — la root e le zone di lavoro, guide comprese — **non** entrano nel backup del programma; l'eccezione
> sono i **router**, in `.<nomeapp>/` alla root, che il programma salva. Il rimando del 2026-09-08, che per le guide diceva
> *«la politica della riga … non cambia»*, si legge con ADR-0040. E nel rimando del 2026-08-07, in fondo, il backup viene
> *«dopo il 5, il 6 e il 9»*: il 5 esce — l'11 dipende da 6 e 9, la seconda risposta della 5.5 del disegno —, e la
> ragione di 6 e 9, la non-vacuità, regge. **Le altre righe reggono:** la separazione per natura, il giornale cifrato, i
> segreti mai, gli indici fuori, i requisiti del motore; lo stato resta `Accepted`.
>>>
@@ docs/README.md
>> | [0039](adr/0039-telecamera-come-sorgente-di-percezione.md) | La telecamera come sorgente di percezione always-on sotto il core | Accepted |
+L | [0040](adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md) | Dove vivono i dati, e che cosa salva il programma — modifica in parte ADR-0022 | Accepted |
@@ docs/COMPENDIO.md
>= **Aggiornato il
== **Aggiornato il <data>**, col **piano dei documenti della revisione della knowledge base** in esecuzione: **ADR-0040** — la sua voce in §5, il rimando nella voce di ADR-0022, i totali, la riga del caso nuovo in §13 — e, compito per compito, i rimandi nelle voci della §5; fin dove, lo dice la tabella della posizione del piano. Questa riga com'era è in [`archivio/stato-storico.md`](archivio/stato-storico.md). Il contenuto di merito nuovo è la voce di ADR-0040, coi rimandi. Manutenzione, e perché questa riga è la più facile da lasciare indietro: §13.
>> Sono **39 ADR**, e **39 ADR in stato Accepted** — l'ultimo, 0029, chiuso il 2026-09-10 con SP-8.
== Sono **40 ADR**, e **40 ADR in stato Accepted** — l'ultimo, 0040, il <data>, dalla revisione della knowledge base.
>> da una **settima porta** — stella polare della GUI, §2.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** **modificato in parte da ADR-0040** — la riga «artefatti», le guide e la conseguenza sulla base di conoscenza; il resto regge, e lo stato resta `Accepted`.
>> (decisione 7 dei gesti, chiusa dal disegno della knowledge base).
+P
<<<
**0040 — Dove vivono i dati, e che cosa salva il programma.** **Modifica ADR-0022 in tre punti** — la riga
«artefatti», le guide, la conseguenza sulla base di conoscenza —, e lo stato di 0022 resta `Accepted`. I **dati del
programma** — giornale, configurazione, indice, copie del checkpoint — stanno nella cartella dati per utente del sistema,
e la configurazione porta il percorso della **root**. I **file del proprietario** — la root e le zone di lavoro, guide
comprese — **non** entrano nel backup del programma: li salvano i suoi backup e git. L'eccezione sono i **router**, in
`.<nomeapp>/` alla root, che il programma salva. Il backup porta giornale, configurazione e router — mai i segreti, né
l'indice, né i pesi — e **dice che cosa resta fuori** quando lo crea. Le **copie del checkpoint** sono in chiaro, fuori
dal backup, potate come vuole ADR-0018: dopo un ripristino un passo di prima non si annulla più. La cartella dati è un
**percorso protetto e privato** — il rimando di ADR-0016.
>>>
>> | ❌ **ri-derivare l'architettura** | è nei 39 ADR, ciascuno con alternative scartate e motivo |
== | ❌ **ri-derivare l'architettura** | è nei 40 ADR, ciascuno con alternative scartate e motivo |
>> | ADR superato | la voce resta e si marca; gli ADR sono **append-only** |
+L | ADR **superato in parte** | la voce resta e riceve la riga del rimando; l'ADR nuovo ha la sua voce, che dice quali righe modifica — la forma di ADR-0040 su ADR-0022 |
@@ CLAUDE.md
>> | **ADR append-only** | superato → `Superseded by`; completato → un **rimando**. Completare una riga di verifica **non** è superare l'ADR |
== | **ADR append-only** | superato → `Superseded by`; completato → un **rimando**; **superato in parte** → un rimando in testa che nomina le righe, e l'ADR nuovo lo dichiara — lo stato resta `Accepted`, la forma di ADR-0040 su ADR-0022. Completare una riga di verifica **non** è superare l'ADR |
@@ docs/HANDOFF.md
>> Spec del kernel **§0–§10 completa, 39 ADR**.
== Spec del kernel **§0–§10 completa, 40 ADR**.
>> 39 ADR in stato `Accepted`. Rimetterne in discussione uno
== 40 ADR in stato `Accepted`. Rimetterne in discussione uno
>> | ❌ ri-derivare l'architettura | è in **39 ADR**, ciascuno con alternative scartate e motivo |
== | ❌ ri-derivare l'architettura | è in **40 ADR**, ciascuno con alternative scartate e motivo |
>> **39 decisioni architetturali**. Leggi **0001**
== **40 decisioni architetturali**. Leggi **0001**
@@ docs/roadmap.md
>= Ultimo aggiornamento:
== Ultimo aggiornamento: **<data>**, col compito 1 del [piano dei documenti della revisione della knowledge base](superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md): il totale degli ADR, con ADR-0040.
>> (§0–§10, 39 ADR)
== (§0–§10, 40 ADR)
@@ docs/AVVIO-CHAT.md
>>   2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 39 ADR
==   2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le ADR
>> siano lo dice `ls crates/kernel/tests/frozen/*.cbor`, e un comando non marcisce.
+P ⚠️ **RICHIAMO DEL <data>: il messaggio diceva `le 39 ADR`, e con ADR-0040 il totale è cambiato.** Il numerale è **tolto e non riallineato** — gotcha **#31**, e la risposta A del proprietario alla prima domanda della 6.1 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md): una cifra che vive in più documenti si toglie, non si ricorregge. Quanti siano lo dice `ls docs/adr/*.md`.
````

Il blocco POS1, in `pos1.txt`:

````text
# la posizione del compito 1, nello stesso commit
@@ docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
>= | **1** |
== | **1** | ADR-0040, e ciò che il cancello pretende con lui | uno | ✅ <data> |
````

- [ ] **Passo 4: le prove**

```bash
S=<scratchpad>; D=<data>
F="docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md docs/README.md docs/COMPENDIO.md CLAUDE.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md docs/archivio/stato-storico.md"
bash scripts/check-docs.sh
bash scripts/gate.sh
git ls-files --eol $F docs/adr/0040-*.md
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F docs/adr/0040-*.md | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
for f in docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md; do sed 's/`[^`]*`//g' "$f" | grep -nE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)' | sed "s|^|$f:|"; done
sed 's/`[^`]*`//g' docs/AVVIO-CHAT.md | grep -nE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)'
grep -c 'Status:.. Accepted' docs/adr/0040-*.md; grep -c 'Modifica:' docs/adr/0040-*.md; grep -c 'Negative (accettate)' docs/adr/0040-*.md
grep -c 'modificato da ADR-0040' docs/adr/0022-*.md
grep -c 'superato in parte' CLAUDE.md docs/COMPENDIO.md
grep -o 'Rimando del' docs/adr/0022-*.md | wc -l
git diff -- docs/adr/ | grep -c '^-[^-]'
git status --porcelain -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' $F docs/adr/0040-*.md
git diff --stat
```

Atteso: `OK` e `GATE GREEN` — ⚠️ è il compito che tocca il cancello: senza il blocco, ADR-0040 da solo fa rosso
`check-docs.sh` in tre modi, la voce, la riga dell'indice, i totali —; la colonna `w/…` e i CR come al Passo 1, e ADR-0040
`w/lf`; le tabelle come al Passo 1; il margine più piccolo di circa 1,6 KB, e positivo; i totali, **sette** righe, tutte a
quaranta — `AVVIO-CHAT.md` non ne rende più —, e il comando B niente; `1`, `1`, `1`; `1`; `1` e `1`; `2` rimandi in
ADR-0022; `0` righe tolte negli ADR; niente fuori dai documenti; il comando E, `1` in più su ADR-0022 e su `AVVIO-CHAT.md`,
e `1` su ADR-0040; un diff che nomina gli otto file di `F` e questo piano, più ADR-0040 da aggiungere.

- [ ] **Passo 5: il commit e il push**

```bash
git add docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md docs/README.md docs/COMPENDIO.md CLAUDE.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md docs/archivio/stato-storico.md docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
git commit -m "knowledge-base-revisione(compito 1): ADR-0040 — dove vivono i dati, e che cosa salva il programma —, che modifica in parte ADR-0022; il rimando in testa a 0022, la riga dell'indice, la voce nella §5 del compendio col rimando nella voce di 0022, i totali a quaranta e la cifra tolta da AVVIO-CHAT col richiamo, il caso nuovo nella §13 e in CLAUDE.md, la riga della data con la sua copia in archivio"
git push
```

#### Criterio di chiusura del compito 1

- [ ] ADR-0040 esiste, `Accepted`, con la riga `Modifica:` e le *Negative (accettate)*, e dice punto per punto la 3.2
- [ ] ADR-0022 porta in testa il rimando *«modificato da ADR-0040»*, e **nessuna riga preesistente** è cambiata
- [ ] la revisione ha riletto ADR-0040 contro la 3.2 e la 3.3, e il rimando di ADR-0022 contro ADR-0018, ADR-0023 e ADR-0024 — i fratelli della sua riga nella 3.1, gotcha #59 — e contro i due rimandi che ADR-0022 porta già
- [ ] i totali a quaranta nei documenti della guardia; la cifra tolta da `AVVIO-CHAT.md`, col richiamo; `superato in parte` in `CLAUDE.md` e nella §13
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga e tabelle come al Passo 1, commit pushato, posizione aggiornata

---

## Compito 2: i rimandi in testa a dieci ADR, e le loro voci nel compendio

**Files:**
- Modify: `docs/adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md` · `docs/adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md` · `docs/adr/0011-routing-risolto-e-giornalato-per-richiesta.md` · `docs/adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md` · `docs/adr/0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md` · `docs/adr/0015-descrizioni-degli-strumenti-fissate-all-approvazione.md` · `docs/adr/0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md` · `docs/adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md` · `docs/adr/0025-confinamento-a-livelli.md` · `docs/adr/0038-registro-delle-funzioni-del-programma.md` — il rimando in testa · `docs/COMPENDIO.md` — la riga di ciascuno nella sua voce della §5 · questo piano — la posizione
- Read: la 3.1 del disegno, riga per riga, con la colonna *«Riletto contro»*; P-7; D5; la testa di ADR-0001, la forma

- [ ] **Passo 1: le misure prima**

```bash
S=<scratchpad>; D=<data>
A="docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0011-*.md docs/adr/0012-*.md docs/adr/0014-*.md docs/adr/0015-*.md docs/adr/0016-*.md docs/adr/0024-*.md docs/adr/0025-*.md docs/adr/0038-*.md"
bash scripts/check-docs.sh
git ls-files --eol $A docs/COMPENDIO.md
for f in $A docs/COMPENDIO.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $A docs/COMPENDIO.md | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
for n in 0009 0010 0011 0012 0014 0015 0016 0022 0024 0025 0038 0039; do printf '%s ' $n; grep -o 'Rimando del' docs/adr/$n-*.md | wc -l; done
for f in $A; do printf '%s ' "$f"; tr -d '\r' < "$f" | tr '\n' ' ' | sed 's/> //g' | grep -o 'Nessuna riga di questo ADR è superata' | wc -l; done
grep -c '2026-09-28-knowledge-base-revisione-design' $A docs/COMPENDIO.md
grep -c 'revisione della knowledge base' docs/COMPENDIO.md
```

Atteso: `OK`; la colonna `w/…` e i CR si annotano; le tabelle, niente; il margine positivo — la simulazione dava `6746`, e il
compito ne consuma circa 1,9 KB —; il comando A, `1` per 0009, 0010, 0011 e 0038, `2` per 0022, `1` per 0039, `0` per gli
altri sei; la frase *«Nessuna riga di questo ADR è superata»*, `1` per 0009, 0010 e 0038, `0` per gli altri sette —
ADR-0011 porta un rimando che non la dice —; il comando E, `0` su ogni ADR e `1` sul compendio; `3` per *«revisione della
knowledge base»* nel compendio.

- [ ] **Passo 2: il blocco E2 — prima il `--check`, poi davvero; e la posizione**

```bash
S=<scratchpad>; D=<data>
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$D" "$S/e2.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/e2.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/pos2.txt"
```

Atteso: `checked: 20 edits in 11 files`, `applied: 20 edits in 11 files`, `applied: 1 edits in 1 files`.

Il blocco E2, in `e2.txt`:

````text
# compito 2 -- i rimandi in testa a dieci ADR, e la riga di ciascuno nella sua voce del compendio
@@ docs/adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — il file-guida di una zona si carica per fiducia alla cartella, e i trigger sanno dire «ho
> perso eventi».** Dal [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposta D9 e sezione 3.1 —: il **file-guida** di una zona di lavoro —
> `CLAUDE.md`, `AGENTS.md` e simili — si carica per **fiducia alla cartella**, data una volta dal proprietario e scritta
> nel giornale, e ogni caricamento scrive nel giornale l'**impronta** della versione caricata; per le skill della
> knowledge base decide la decisione registrata da AUD-004. I **trigger** sono il sorvegliante dei file più la scansione
> all'avvio, e sanno dire *«ho perso eventi, riscansiona»*. ⚠️ **Dedotto:** il **riconciliatore**, che corregge da solo i
> casi certi dei router — risposte D6 e D14 —, **non** è l'anello di miglioramento e non tocca la sua regola *«non si
> auto-modifica in silenzio»*, che riguarda le guide e i sensori del sistema: i router sono dati del proprietario,
> osservati come `Untrusted`, e ogni correzione è giornalata, con la copia, e visibile. **Nessuna riga di questo ADR è
> superata.**
>>>
@@ docs/adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — in un ripiego la proiezione si compone per il candidato.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposta D10 e sezione 3.1 —: in un ripiego la proiezione si compone **per
> il candidato** — la sua finestra e la sua guida — e si controlla prima di mandarla; un candidato la cui finestra non
> tiene ciò che [ADR-0008](0008-contesto-come-proiezione-dello-stato.md) dice mai sacrificabile si **salta**; la
> compressione *middle-out* di OpenRouter è **spenta**. **Nessuna riga di questo ADR è superata.**
>>>
@@ docs/adr/0011-routing-risolto-e-giornalato-per-richiesta.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — la sessione della contabilità è quella della risposta D11.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposta D11 e sezione 3.1 —: la **sessione**, una delle quattro
> granularità della contabilità, è la run radice che il proprietario apre, coi suoi discendenti — uguale sul lato della
> chat e su quello del coding —, e finisce quando lui la chiude, dopo un tempo di inattività, dopo un tempo massimo, o al
> riavvio del core. **Nessuna riga di questo ADR è superata.**
>>>
@@ docs/adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — che cosa fa scattare la catena, e chi la scrive.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposta D10 e sezione 3.1 —: la catena di riserva la scrive il
> **proprietario** nella configurazione — senza catena, nessun ripiego — e la percorre il gateway, un modello per
> richiesta. La fanno scattare il sovraccarico, l'indisponibilità e l'errore del server; **non** i limiti di frequenza —
> resta da misurare su OpenRouter, K48 —; il contesto eccessivo si controlla **prima**, per candidato, col rimando di
> [ADR-0010](0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md). Un ripiego per disponibilità dura il turno;
> un rifiuto per contenuto ripiega e **resta**. Il *Context* di questo ADR, che nomina i limiti di frequenza e il contesto
> eccessivo, è contesto e non decisione. **Nessuna riga di questo ADR è superata.**
>>>
@@ docs/adr/0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — per il file-guida di una zona fidata, il passaggio esplicito è la fiducia alla cartella.**
> Dal [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposta D9 e sezione 3.1 —: per il file-guida di una zona di lavoro
> fidata, il **passaggio esplicito e giornalato** che questo ADR chiede è la **fiducia alla cartella**, data una volta
> dal proprietario e scritta nel giornale; ogni caricamento scrive l'impronta, la provenienza; un file-guida **non
> concede permessi** — [ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md). **Nessuna riga di questo
> ADR è superata.**
>>>
@@ docs/adr/0015-descrizioni-degli-strumenti-fissate-all-approvazione.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — il file-guida di una zona fidata non si riapprova quando cambia.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposta D9 e sezione 3.1 —, come nei software letti per D9. ⚠️ È il *rug
> pull* che il richiamo AUD-004 di questo ADR descrive per le skill, **accettato** dal proprietario con D9 per la zona
> che ha dichiarato fidata: un `git pull` arriva all'agente senza che lui lo guardi. Restano il punto 5 di questo ADR,
> perché un file-guida non concede permessi; l'impronta di ogni caricamento nel giornale; e la modalità ristretta per la
> zona non fidata, dove i file-guida non si caricano da soli. Le **descrizioni degli strumenti** — anche dei server MCP
> di una zona fidata — restano sotto questo ADR; per le skill della knowledge base decide AUD-004, che ha qui il caso
> scritto. **Nessuna riga di questo ADR è superata.**
>>>
@@ docs/adr/0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — la sessione, i percorsi protetti, l'irripetibile e la lettura fuori da ogni zona.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposte D11, D14, D16, D18 e D19, e sezione 3.1 —: la **sessione** del
> punto 3 è quella di D11, e finisce anche al riavvio del core. Il permesso lo chiede chi agisce per un modello o invoca
> una funzione; **la correzione deterministica di un fatto, che non cambia una scelta del proprietario**, segue
> un'impostazione — da solo, chiedi, mai. I **percorsi protetti** — il file delle esclusioni alla root e la cartella dati
> del programma —: nessun sì copre una scrittura dell'agente su di loro, e il controllo lo fa la porta dei file prima di
> ogni sì. Un effetto **irripetibile** chiede a **ogni invocazione**, non per la sessione: un'eccezione al punto 3, che
> la regola 4 di [ADR-0038](0038-registro-delle-funzioni-del-programma.md) dice già per ogni invocatore. Fuori da ogni
> zona la **lettura** è una tripla `(file, percorso, lettura)` per la sessione, un file alla volta, e un'impostazione
> blocca ogni lettura fuori. Il sì **oltre** la sessione, che le app di oggi offrono, resta registrato — K38, del
> proprietario. **Nessuna riga di questo ADR è superata.**
>>>
@@ docs/adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — l'ambito di una zona si chiude con la sessione, e le copie le dice ADR-0040.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposte D2, D3, D11 e D18, e sezione 3.1 —: l'ambito di una **zona di
> lavoro** si **chiude** con la sessione; gli ambiti sono della porta e non della run, e fra due run li separa la sola
> tripla — K35; un ambito di **sola lettura**, per un file fuori da ogni zona, non è un ambito di lavoro e non tiene
> copie. ⚠️ **Dedotto:** il checkpoint copre le scritture del **programma**, non quelle del proprietario da fuori — D2.
> Dove stanno le copie e quanto durano lo dice
> [ADR-0040](0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md); il limite di dimensione resta da fissare — K21.
> **Nessuna riga di questo ADR è superata.**
>>>
@@ docs/adr/0025-confinamento-a-livelli.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — il livello 2 tiene il privato e i percorsi protetti per i comandi.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposte D5, D16, D18 e D19, e sezione 3.1 —: per i comandi che l'agente
> esegue, il livello 2 nega i percorsi del **privato**, la **scrittura** sui percorsi protetti e, con l'impostazione
> accesa, la **lettura** fuori da ogni zona: la porta dei file non vede uno script che apre i file da sé. **Nessuna riga
> di questo ADR è superata.**
>>>
@@ docs/adr/0038-registro-delle-funzioni-del-programma.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — il riconciliatore non è un invocatore, e «fuori» vuol dire verso una zona aperta.** Dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
> approvato dal proprietario il 2026-09-29 — risposte D3 e D14, e sezione 3.1 —: la regola 2 — lo stesso permesso per
> ogni invocatore — **non** copre il riconciliatore, che non è un invocatore: la correzione deterministica di un fatto,
> che non cambia una scelta del proprietario, segue la sua impostazione; un router toccato dalla GUI resta una funzione
> del registro. Nel rimando del 2026-09-05, *«spostare … fuori»* vuol dire verso una **zona aperta**: fuori da ogni zona
> non scrive nessuno, nemmeno il click. **Nessuna riga di questo ADR è superata.**
>>>
@@ docs/COMPENDIO.md
>> con «approvate ora» come proiezione del giornale — disegno della knowledge base.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** il file-guida di una zona si carica per fiducia alla cartella, con l'impronta di ogni caricamento; i trigger sanno dire «ho perso eventi, riscansiona» — revisione della knowledge base.
>> è una **categoria** del budget, per modello — disegno della knowledge base.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** in un ripiego la proiezione si compone per il candidato, e uno che non la tiene si salta — revisione della knowledge base.
>> un gesto di comando apre un passo, i fotogrammi no — ADR-0039.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** la sessione della contabilità è quella di D11, la run radice coi suoi discendenti — revisione della knowledge base.
>> **Un ritentativo non è un passo nuovo.**
++ ⚠️ **Rimando del <data>, in testa all'ADR:** la catena la scrive il proprietario; i limiti di frequenza non la fanno scattare, il contesto eccessivo si controlla prima — revisione della knowledge base.
>> la stessa autorizzazione che richiederebbe se l'utente non l'avesse chiesta.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** per il file-guida di una zona fidata il passaggio esplicito è la fiducia alla cartella — revisione della knowledge base.
>> terzi**. **Una descrizione non concede permessi**: è testo, non autorità.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** il file-guida di una zona fidata non si riapprova quando cambia, accettato con D9; gli strumenti restano qui — revisione della knowledge base.
>> che blocca.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** la sessione di D11; i percorsi protetti; l'irripetibile chiede a ogni invocazione; la lettura fuori da ogni zona per tripla — revisione della knowledge base.
>> effetti fuori dagli ambiti non sono coperti.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** l'ambito di una zona si chiude con la sessione; dove stanno le copie lo dice ADR-0040 — revisione della knowledge base.
>> ripiego, è un'altra cosa.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** il livello 2 nega ai comandi il privato e la scrittura sui percorsi protetti — revisione della knowledge base.
>> ha **due invocatori**, il click e il modello — disegno della knowledge base.
++ ⚠️ **Rimando del <data>, in testa all'ADR:** il riconciliatore non è un invocatore; «fuori» vuol dire verso una zona aperta — revisione della knowledge base.
````

Il blocco POS2, in `pos2.txt`:

````text
# la posizione del compito 2, nello stesso commit
@@ docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
>= | **2** |
== | **2** | i rimandi in testa a dieci ADR, e le loro voci nel compendio | uno | ✅ <data> |
````

- [ ] **Passo 3: le prove**

```bash
S=<scratchpad>; D=<data>
A="docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0011-*.md docs/adr/0012-*.md docs/adr/0014-*.md docs/adr/0015-*.md docs/adr/0016-*.md docs/adr/0024-*.md docs/adr/0025-*.md docs/adr/0038-*.md"
bash scripts/check-docs.sh
bash scripts/gate.sh
git ls-files --eol $A docs/COMPENDIO.md
for f in $A docs/COMPENDIO.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $A docs/COMPENDIO.md | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
for n in 0009 0010 0011 0012 0014 0015 0016 0022 0024 0025 0038 0039; do printf '%s ' $n; grep -o 'Rimando del' docs/adr/$n-*.md | wc -l; done
for f in $A; do printf '%s ' "$f"; tr -d '\r' < "$f" | tr '\n' ' ' | sed 's/> //g' | grep -o 'Nessuna riga di questo ADR è superata' | wc -l; done
for f in docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0011-*.md docs/adr/0038-*.md; do grep -n -e 'Rimando del' -e '^## Context' "$f" | cut -c1-40; done
git diff -- docs/adr/ | grep -c '^-[^-]'
git status --porcelain -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' $A docs/COMPENDIO.md
grep -c 'revisione della knowledge base' docs/COMPENDIO.md
git diff --stat
```

Atteso: `OK` e `GATE GREEN`; fine-riga e tabelle come al Passo 1; il margine più piccolo di circa 1,9 KB, e positivo; il
comando A, **uno in più** per ciascuno dei dieci — 0022 e 0039 fermi a `2` e `1` —; la frase, **uno in più** per ciascuno
dei dieci; in 0009, 0010, 0011 e 0038 il rimando di questa data **sotto** quello di prima e sopra `## Context` — D5 —;
`0` righe tolte; niente fuori dai documenti; il comando E, `1` su ogni ADR e ancora `1` sul compendio — le voci della §5
rimandano **senza** link —; `13` per *«revisione della knowledge base»*; un diff che nomina i dieci ADR, il compendio e
questo piano.

- [ ] **Passo 4: il commit e il push**

```bash
git add docs/adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md docs/adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md docs/adr/0011-routing-risolto-e-giornalato-per-richiesta.md docs/adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md docs/adr/0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md docs/adr/0015-descrizioni-degli-strumenti-fissate-all-approvazione.md docs/adr/0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md docs/adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md docs/adr/0025-confinamento-a-livelli.md docs/adr/0038-registro-delle-funzioni-del-programma.md docs/COMPENDIO.md docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
git commit -m "knowledge-base-revisione(compito 2): i rimandi datati in testa ad ADR-0009, 0010, 0011, 0012, 0014, 0015, 0016, 0024, 0025 e 0038 — la fiducia alla cartella, la proiezione per candidato, la sessione, la catena di riserva, i percorsi protetti, l'irripetibile, la lettura fuori da ogni zona, il riconciliatore — e la riga di ciascuno nella sua voce della §5 del compendio"
git push
```

#### Criterio di chiusura del compito 2

- [ ] i dieci ADR portano il rimando datato in testa, sotto quello di prima dove c'era, con la frase sulla non-superazione, e **nessuna riga preesistente** è cambiata
- [ ] le dieci voci della §5 rimandano in una frase, senza link
- [ ] la revisione ha riletto ogni rimando contro l'ADR intero **e** contro i fratelli della colonna *«Riletto contro»* della 3.1 — gotcha #59 —, e non ha trovato una riga superata
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga e tabelle come al Passo 1, commit pushato, posizione aggiornata

---

## Compito 3: il disegno del 2026-09-04, e la cattura in tutte le sue case

**Files:**
- Modify: `docs/adr/0039-telecamera-come-sorgente-di-percezione.md` — il rimando **nella cella** del perimetro negativo · `docs/COMPENDIO.md` — la riga nella voce di 0039 · `docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` — due richiami · `docs/superpowers/specs/2026-09-04-knowledge-base-design.md` — il richiamo in testa, e uno per ogni posto della sezione 2 · questo piano — la posizione
- Read: la sezione 2 del disegno, **tutte** le righe 0–23 con la domanda e la risposta; la 5.8; la riga di 0039 nella 3.1; P-6, P-16, P-17; D6, D7

- [ ] **Passo 1: le misure prima**

```bash
S=<scratchpad>; D=<data>
F="docs/adr/0039-telecamera-come-sorgente-di-percezione.md docs/COMPENDIO.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md docs/superpowers/specs/2026-09-04-knowledge-base-design.md"
K=docs/superpowers/specs/2026-09-04-knowledge-base-design.md
bash scripts/check-docs.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
grep -o 'Rimando del' docs/adr/0039-*.md | wc -l; grep -c 'Rimando del' docs/adr/0039-*.md
grep -c 'Richiamo del' $K
for f in $F; do printf '%s ' "$f"; grep -o '<data>' "$f" | wc -l; done
grep -c '2026-09-28-knowledge-base-revisione-design' $F
grep -n 'router che segue' docs/adr/0039-*.md | cut -c1-60
grep -n 'in un gruppo' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md | cut -c1-60
```

Atteso: `OK`; la colonna `w/…` e i CR si annotano; le tabelle, `2` righe nel disegno del 2026-09-04 — P-18 —; il margine
positivo — la simulazione dava `4841` —; ADR-0039, `1` occorrenza su `1` riga; `0` richiami nel disegno del 2026-09-04;
`<data>`, `0` `0` `1` `1` — P-16 —; il comando E, `0` `1` `0` `0`; una riga per *«router che segue»*, due per *«in un
gruppo»*.

- [ ] **Passo 2: il blocco E3 — prima il `--check`, poi davvero; e la posizione**

```bash
S=<scratchpad>; D=<data>
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$D" "$S/e3.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/e3.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/pos3.txt"
```

Atteso: `checked: 59 edits in 4 files`, `applied: 59 edits in 4 files`, `applied: 1 edits in 1 files`.

Il blocco E3, in `e3.txt` — i commenti `# riga N` sono le righe della sezione 2 del disegno:

````text
# compito 3 -- il disegno del 2026-09-04, e la cattura in tutte le sue case
@@ docs/adr/0039-telecamera-come-sorgente-di-percezione.md
>> risposta 7 e regola 4 della §2.3. Nessuna riga di questo ADR è superata |
++ ⚠️ **Rimando del <data>:** la cattura entra nella root come **ogni file nuovo** — nel livello strutturale, nel grafo e nella ricerca —, e in un router solo se il proprietario o l'agente la promuovono; la run la vede, come riferimento. È la risposta A del proprietario del 2026-09-29, riga 10 della sezione 2 del [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md). Nessuna riga di questo ADR è superata
@@ docs/COMPENDIO.md
>> (decisione 7 dei gesti, chiusa dal disegno della knowledge base).
++ ⚠️ **Rimando del <data>, nella riga del perimetro negativo:** la cattura entra nella root come ogni file nuovo, e in un router solo se promossa — revisione della knowledge base.
@@ docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md
>> risposta 7 e regola 4 della §2.3 |
++ ⚠️ **Richiamo del <data>:** la cattura entra nella root come **ogni file nuovo** — nel livello strutturale, nel grafo e nella ricerca —, e in un router solo se il proprietario o l'agente la promuovono; la run riceve il riferimento — la risposta A del proprietario alla domanda della sezione 2 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md), riga 10
>> la foto atterra in un gruppo, la run riceve il riferimento (riga 7 della tabella delle decisioni) |
++ ⚠️ **Richiamo del <data>:** la stessa correzione della riga 7; e *«la cartella della knowledge base è l'archivio»* si legge: la **root**, una cartella qualsiasi del proprietario, dalla configurazione — righe 1 e 10 della sezione 2 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
@@ docs/superpowers/specs/2026-09-04-knowledge-base-design.md
# riga 0 -- la testa
>> ed è riscritta invece di ricevere una riga sotto, sul precedente del disegno dei gesti.
+P
<<<
⛔ **RICHIAMO DEL <data> — questo disegno è CORRETTO dalla revisione della knowledge base del 2026-09-28 e 2026-09-29.**
Il proprietario ha rivisto il modello con le risposte D1–D20 del
[disegno della revisione](2026-09-28-knowledge-base-revisione-design.md): la cartella non è più *«un archivio unico»* ma
una **root** qualsiasi, gli attori che scrivono sono **due**, l'agente cerca anche nel **livello strutturale**, e ADR-0040
dice che cosa salva il programma. Ogni riga che le risposte hanno superato porta qui il proprio **richiamo datato**, e la
riga di prima resta leggibile; che cosa vale adesso, e perché, sta nella sezione 2 di quel disegno, riga per riga. **Il
resto regge:** la strada B, l'ordine 2, 13, 3, AUD-004 che sbarra il 13, nessuna sesta proprietà.
>>>
# riga 1 -- le premesse, la risposta 1, la 1.1a
>> trova il file giusto attraverso i router al primo salto»*.
++ ⚠️ **Richiamo del <data>, dalla revisione:** *«un archivio unico»* non regge più: la knowledge base è una **root**, una cartella qualsiasi, anche quella in cui il proprietario tiene tutto, e il percorso arriva dalla configurazione — ADR-0034; una repo dentro la root sta nel livello strutturale, una fuori è una **zona di lavoro**, con la sua scheda progetto — riga 1 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
>> Più una pagina visuale della mappa |
++ ⚠️ **Richiamo del <data>:** non più un archivio unico ma la **root**, dalla configurazione; e *«progetti, note, tutto dentro»*: una repo dentro la root sta nel livello strutturale, una fuori è una zona di lavoro con la sua scheda progetto — riga 1 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> le catture atterrano qui |
++ ⚠️ **Richiamo del <data>:** non più *«archivio unico»*: la root, una cartella qualsiasi, dalla configurazione — riga 1 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# righe 2 e 3 -- la risposta 3 e il suo scartato, la 1.1b, la 1.3, il vicolo cieco
>> sarà un'**esportazione** a parte |
++ ⚠️ **Richiamo del <data>:** la risposta e lo scartato non reggono più com'erano. Gli attori sono **due**: il proprietario scrive **da fuori**, con qualunque strumento, anche un altro agente, e l'agente da dentro; *«anche altri agenti da fuori»* è riaperto dal documento del proprietario, e i due rischi di allora li reggono il riconciliatore e AUD-004. ⚠️ **La metà sul modello resta:** lo sceglie il proprietario, e il routing lo applica — D10 la conferma, col selettore. Righe 2 e 3 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> il routing lo applica e lo annota (ADR-0011) |
++ ⚠️ **Richiamo del <data>:** non più *«solo il nostro assistente»*: gli attori sono due, e il proprietario scrive da fuori con qualunque strumento; la metà sul modello resta — D10 — riga 2 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> - Altri strumenti che leggono o scrivono la cartella (decisione **b**).
++ ⚠️ **Richiamo del <data>:** cade — il proprietario scrive da fuori con qualunque strumento, e il programma non assume di essere il solo a scrivere; riga 2 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
>> skill riscritte da altri che entrano nel contesto |
++ ⚠️ **Richiamo del <data>:** **riaperto** dal proprietario col suo documento: i due rischi li reggono il riconciliatore e AUD-004 — riga 3 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 4 -- la regola 2 della 2.3
>> né il pannello, né il modello, né la capacità |
++ ⚠️ **Richiamo del <data>:** *«ogni scrittura nella cartella»* si legge *«ogni scrittura del programma»*: il proprietario scrive da fuori, e il riconciliatore scrive come effetto giornalato — riga 4 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 5 -- tre righe della 1.4
>> è un **verdetto negativo** che rientra nell'anello |
++ ⚠️ **Richiamo del <data>:** l'agente aggiorna i router **nello stesso turno**, con un sì per la sessione sulla cartella dei router — D7; riga 5 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> — e non nella buona volontà |
++ ⚠️ **Richiamo del <data>:** il **riconciliatore**, senza modello, corregge da solo i due casi certi e, nel dubbio, segna **rotto** e chiede; l'anello di miglioramento resta per ciò che si ripete — D6, D14; riga 5 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> Una skill cambiata → AUD-004 |
++ ⚠️ **Richiamo del <data>:** una modifica del proprietario da fuori **non si approva**, perché è sua: il riconciliatore riallinea lo stato derivato, e una skill cambiata resta ad AUD-004 — riga 5 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 6 -- la risposta 9, la 1.1g, la regola 4 della 2.3, la decisione 10
>> Lo spazio designato **è** l'ambito di ADR-0024 |
++ ⚠️ **Richiamo del <data>:** lo spazio non è più uno: la root e le zone di lavoro aperte; da una zona alla root l'agente **copia**; fuori da ogni zona **legge** col permesso, un file alla volta, e non scrive nessuno, nemmeno il click; **cancellare** è spostare nel cestino di sistema, con conferma — riga 6 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> è **una** funzione con **due invocatori**: il click e il modello |
++ ⚠️ **Richiamo del <data>:** *«spostare … fuori»* vuol dire verso una zona aperta: fuori da ogni zona non scrive nessuno — riga 6 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> il limite dichiarato di ADR-0024 |
++ ⚠️ **Richiamo del <data>:** lo spazio sono la root e le zone di lavoro aperte; *«da dentro a fuori»* vuol dire verso una zona aperta, perché fuori da ogni zona la porta non scrive per nessuno; fuori da ogni zona l'agente legge col permesso, un file alla volta; cancellare è spostare nel cestino di sistema, con conferma — riga 6 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> | 10 | le CRUD come funzioni del registro; lo spazio è l'ambito; il confine dentro/fuori | ✅ presa, sua |
++ ⚠️ **Richiamo del <data>:** lo spazio sono la root e le zone di lavoro aperte — riga 6 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# righe 7 e 8 -- le risposte 2, 4 e 10, la 4.1, il vicolo cieco
>> (il RAG classico: niente struttura leggibile, e non è ciò che ha descritto) |
++ ⚠️ **Richiamo del <data>:** l'agente cerca anche nel **livello strutturale**: un file che nessun router punta si trova lo stesso, con un costo in più, e *«ciò che non è nella mappa non esiste»* non regge più; la ricerca **testuale**, senza modello, arriva con la prima metà del 6, quella per **somiglianza** resta dopo — riga 7 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> più rigido, e più lavoro del proprietario per tenere le chiavi — offerto e non scelto |
++ ⚠️ **Richiamo del <data>:** dopo il piano 1 l'agente cerca nel livello strutturale, prima nelle cartelle dell'area e poi in tutta la knowledge base — riga 7; e *«quello dell'ambito»* del piano 0 si legge con la riga 9: per una zona la chiave è la sua scheda progetto, per la knowledge base il router master — sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> *le cartelle come struttura*: il pannello mostrerebbe una cosa e l'agente ne userebbe un'altra |
++ ⚠️ **Richiamo del <data>:** la risposta e lo scartato non reggono più com'erano: l'agente cerca anche nelle **cartelle** dell'area, il pannello raggruppa per cartella **e** per area, e *«per l'agente non esistono»* non regge — righe 7 e 8 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> cinque cartelle, e `check-docs.sh` verifica i **collegamenti**, non le cartelle.
++ ⚠️ **Richiamo del <data>:** l'agente naviga la mappa **e** cerca nel livello strutturale — nelle cartelle dell'area, poi in tutta la knowledge base —, e il pannello raggruppa per cartella e per area — righe 7 e 8 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
>> il sensore di integrità, **nelle due direzioni**, reso visibile |
++ ⚠️ **Richiamo del <data>:** un orfano l'agente lo trova lo stesso, nel livello strutturale, con un costo in più: *«non esistono»* non regge — riga 7 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> | le **cartelle come struttura della mappa** | il pannello mostrerebbe una cosa e l'agente ne userebbe un'altra |
++ ⚠️ **Richiamo del <data>:** **riaperto** dal documento del proprietario: cartella e area insieme, per il pannello e per la ricerca dell'agente, che si limita alle cartelle dell'area — riga 8 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 9 -- la 1.1c, la 1.2, la decisione 15
>> la ricerca per somiglianza, **dopo**, dentro il 6 |
++ ⚠️ **Richiamo del <data>:** l'«ambito» di qui erano **due cose**: il confine della porta — la root o una zona aperta, ADR-0024 — e la **chiave** del piano 0, opaca per il kernel: per una zona la sua scheda progetto, per la knowledge base il router master, e l'area la sceglie l'agente leggendolo. E fra il piano 1 e la somiglianza entra la ricerca testuale nel livello strutturale — righe 7 e 9 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> a parole; la forma la dà il 13 |
++ ⚠️ **Richiamo del <data>:** sciolto: sono due cose, il confine della porta e la chiave del piano 0; la forma della risorsa di un permesso su un percorso scelto a runtime è **registrata**, e la chiude il 6 — K44 — riga 9 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> | 15 | l'ambito della cartella = ambito di ADR-0024 (forma) | ⏳ dedotta | il sotto-progetto 13 |
++ ⚠️ **Richiamo del <data>:** l'ambito della porta è la root, dalla configurazione, e lo dichiara il 13; la chiave del piano 0 è un'altra cosa, opaca per il kernel; la forma della risorsa su un percorso scelto a runtime la chiude il **6**, K44 — riga 9 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 10 -- la risposta 7
>> la foto atterra in un gruppo, il router segue, la run riceve il riferimento |
++ ⚠️ **Richiamo del <data>:** la cartella è la root, e la cattura vi entra come **ogni file nuovo** — nel livello strutturale, nel grafo e nella ricerca —, e in un router solo se il proprietario o l'agente la promuovono; la run la vede, come riferimento. È la risposta A del proprietario, il 2026-09-29 — riga 10 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 11 -- la 4.2, nella prima riga della sua tabella
>> | **nodi** | router · gruppo · foglia · skill · guida-modello · cattura |
++ ⚠️ **Richiamo del <data>:** ogni file **indicizzato** è un nodo, il rumore compreso e il privato no; le frecce di questa tabella stanno nelle **tre specie di linea** — l'**area**, cioè router master → indice d'area → file chiave; il **link** scritto dal proprietario, che porta anche il ritorno dalla skill al suo router; e la **cartella** —; il segnale **rotto** vale anche per un file chiave che il riconciliatore non ritrova e per un link verso un file che non c'è — riga 11 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 12 -- la 4.3
>> tenere stato (ADR-0004). Se muore, non si perde niente.
++ ⚠️ **Richiamo del <data>:** la ricerca mentre si scrive cerca nel **testo**, coi filtri per tipo e per cartella, non più solo sui nomi; la **griglia** accanto al grafo; al click l'**anteprima**, accanto al percorso col suo «copia» — riga 12 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
# riga 13 -- la 4.4
>> la valutazione probabilistica di L2 (ADR-0020) |
++ ⚠️ **Richiamo del <data>:** la prima metà guadagna il **livello strutturale**, con la ricerca **testuale** senza modello; la seconda resta per la somiglianza — riga 13 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 14 -- la 2.2, la cartella e l'indice
>> | **la cartella** su disco | artefatti dell'utente (ADR-0022): non cifrata, **nel backup** | — | router, foglie, guide, catture |
++ ⚠️ **Richiamo del <data>:** la cartella non è più *«nel backup»* per intero: la root sta nei backup del proprietario, e il programma salva i router, che stanno in `.<nomeapp>/` — ADR-0040; riga 14 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> | ciò che il pannello disegna, via `ipc` |
++ ⚠️ **Richiamo del <data>:** l'indice sta nella cartella dati del programma, uno per root — ADR-0040; riga 14 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 15 -- la 2.2, i trigger
>> Una modifica a mano diventa «da fuori» |
++ ⚠️ **Richiamo del <data>:** il **sorvegliante** più la **scansione** all'avvio; il meccanismo sa dire *«ho perso eventi, riscansiona»*, e un'indicizzazione in corso si dichiara prima — ADR-0019; riga 15 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 16 -- la 2.2, il registro delle guide; la regola 3 della 2.3; la 1.1e; la decisione 9
>> **inietta per chiave** — ambito, run, modello |
++ ⚠️ **Richiamo del <data>:** per il file-guida di una zona l'approvazione è la **fiducia alla cartella**, e l'impronta si scrive a **ogni caricamento**; per le skill della knowledge base decide AUD-004 — riga 16 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> il registro delle guide **rifiuta** una guida senza impronta | ADR-0014 · 1.1e |
++ ⚠️ **Richiamo del <data>:** per un file-guida di zona l'approvazione è la fiducia alla cartella, con l'impronta a ogni caricamento — riga 16 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> e «approvato ora» è una **proiezione del giornale** — la forma di `permission.rs` |
++ ⚠️ **Richiamo del <data>:** per il file-guida di una zona l'approvazione è la fiducia alla cartella, e l'impronta si scrive a ogni caricamento — D9; riga 16 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> | 9 | le due pretese sul registro delle guide (1.1e) | ✅ presa qui, sotto accettazione: da rileggere col disegno |
++ ⚠️ **Richiamo del <data>:** con la fiducia alla cartella per i file-guida di zona — D9, riga 16 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 17 -- la 2.2, la proiezione; la 1.4, i modelli diversi
>> la mappa è una **categoria con budget per modello**; le foglie sono **riferimenti** |
++ ⚠️ **Richiamo del <data>:** in un ripiego la proiezione si compone per il **candidato** — la sua finestra e la sua guida —, e un candidato che non la tiene si salta; senza catena di riserva il ripiego non c'è — D10; riga 17 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> il meccanismo esiste e non scatta mai: costo zero |
++ ⚠️ **Richiamo del <data>:** in un ripiego la proiezione si compone per il candidato, e un candidato che non la tiene si salta; senza catena il costo resta zero — riga 17 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 18 -- la 1.1d, la 2.4, la riga 13 della 5.1
>> o la prima capacità si inietta le skill a modo suo e nascono due strade |
++ ⚠️ **Richiamo del <data>:** il vincolo d'ordine resta, e il 13 **cresce**: la lettura e la sorgente degli eventi della porta dei file, la finestra del candidato nel gateway, il «riscansiona», la forma per la fiducia di D9, la proiezione per candidato; l'ordine vive nella roadmap — riga 18 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> questo le due pretese di 1.1e sono scritte **prima**.
++ ⚠️ **Richiamo del <data>:** il 13 paga anche la lettura e la sorgente degli eventi della porta dei file, e la finestra del candidato; chi paga che cosa sta nella sezione 4 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md) — riga 18 della sua sezione 2.
>> le due pretese di 1.1e; l'ambito della cartella |
++ ⚠️ **Richiamo del <data>:** il 13 cresce — la sezione 4 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md), riga 18 della sua sezione 2
# riga 19 -- la 1.3
>>   nuove: **niente di tutto questo**, ed è misurabile.
++ ⚠️ **Richiamo del <data>:** non regge più — record nuovi, la fine della sessione e la fiducia; la porta che cresce; le regole di backup di ADR-0040 — riga 19 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
>>   dell'utente), ADR-0020 (nessun modello nel kernel).
++ ⚠️ **Richiamo del <data>:** ADR-0022 è **modificato in parte** da ADR-0040, che ne supera tre righe per i file del proprietario; ADR-0009 e ADR-0020 reggono — riga 19 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
# riga 20 -- l'approccio scelto, la 3.1, la 3.2, la 3.3, la 6.4, il vicolo cieco
>> e **AUD-004 va decisa prima** |
++ ⚠️ **Richiamo del <data>:** *«senza ADR nuovi»* cade con D12: arriva ADR-0040, del backup, che non è un ADR della knowledge base — riga 20 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> Nessuno supera niente.
++ ⚠️ **Richiamo del <data>:** *«nessun ADR nuovo»* cade con D12: ADR-0040 **modifica in parte** ADR-0022 per i file del proprietario, e non è un ADR della knowledge base — riga 20 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
>> | un uso non è un cambiamento |
++ ⚠️ **Richiamo del <data>:** la riga cade: 0022 è modificato da ADR-0040, e 0024 e 0014 ricevono un rimando — la sezione 3 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md), riga 20 della sua sezione 2
>> «approvate ora» è una **proiezione** |
++ ⚠️ **Richiamo del <data>:** la cartella è del **proprietario**, e il giornale, lo stato autorevole, sta nella cartella dati del programma, che è protetta — riga 20 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> | la regola append-only di `CLAUDE.md`: *completato → un rimando* |
++ ⚠️ **Richiamo del <data>:** smentita da D12 — ADR-0040 —, riga 20 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> tutti i meccanismi sono già decisi — §3.1 |
++ ⚠️ **Richiamo del <data>:** regge per la knowledge base, e un ADR nuovo arriva lo stesso: ADR-0040, del backup — riga 20 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
# riga 21 -- la 1.4, il privato; la decisione 13; la 4.5
>> **voce aperta** per il sotto-progetto 6, decisione 13 della §3.4 |
++ ⚠️ **Richiamo del <data>:** **chiusa**: rumore e privato nel file delle esclusioni alla root, che è un percorso protetto come la cartella dati del programma; per le zone, K41 — D5, D16, D19, D20; riga 21 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md)
>> | 13 | «privato ma non segreto»: cosa resta fuori dagli indici | ⏳ registrata |
== | 13 | «privato ma non segreto»: cosa resta fuori dagli indici | ✅ **chiusa il 2026-09-29** dalla revisione: rumore e privato nel file delle esclusioni, un percorso protetto — D5, D16, D19, D20. ⚠️ **Richiamo del <data>:** questa cella diceva *«⏳ registrata»* — riga 21 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md) |
>> Il pannello nasce col 6 (17, presa).
++ ⚠️ **Richiamo del <data>:** la 13 è **chiusa** dalla revisione — riga 21 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
# riga 22 -- la 6.3 e la 7
>> qui non si disegna.
++ ⚠️ **Richiamo del <data>:** *«nessuna fonte esterna»* non regge più: l'idea dei router viene dalla guida ARMS, datata il giorno di questo disegno, e le fonti della revisione stanno in [`riferimenti.md`](../../riferimenti.md) — riga 22 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
>> all'affermazione che sostengono.
++ ⚠️ **Richiamo del <data>:** non regge più — la guida ARMS, e le fonti della revisione in `riferimenti.md`; riga 22 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md).
# riga 23 -- la riga 1-7 della 3.4
>> · l'assistente scrive · pannello GUI · la foto nella KB) | ✅ **prese**, sue |
++ ⚠️ **Richiamo del <data>:** le risposte 1, 2, 3, 4 e 7 sono **corrette** dalla revisione — righe 1, 2, 7 e 10 della sezione 2 del [disegno della revisione](2026-09-28-knowledge-base-revisione-design.md); la 5 e la 6 reggono, ⚠️ dedotto
````

Il blocco POS3, in `pos3.txt`:

````text
# la posizione del compito 3, nello stesso commit
@@ docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
>= | **3** |
== | **3** | il disegno del 2026-09-04, e la cattura in tutte le sue case | uno | ✅ <data> |
````

- [ ] **Passo 3: le prove**

```bash
S=<scratchpad>; D=<data>
F="docs/adr/0039-telecamera-come-sorgente-di-percezione.md docs/COMPENDIO.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md docs/superpowers/specs/2026-09-04-knowledge-base-design.md"
K=docs/superpowers/specs/2026-09-04-knowledge-base-design.md
bash scripts/check-docs.sh
bash scripts/gate.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
grep -o 'Rimando del' docs/adr/0039-*.md | wc -l; grep -c 'Rimando del' docs/adr/0039-*.md
git diff -- docs/adr/0039-*.md | grep -c '^-[^-]'; git diff -- docs/adr/0039-*.md | grep -c '^+[^+]'
git diff -- docs/adr/0039-*.md | awk '/^-[^-]/{m=substr($0,2)} /^[+][^+]/{p=substr($0,2)} END{print (index(p, substr(m,1,length(m)-2))==1) ? "la + comincia con la -" : "NO"}'
grep -c "Richiamo del $D" $K; grep -c "RICHIAMO DEL $D" $K
grep -c "Richiamo del $D" docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md
for f in $F; do printf '%s ' "$f"; grep -o '<data>' "$f" | wc -l; done
git status --porcelain -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' $F
git diff --stat
```

Atteso: `OK` e `GATE GREEN`; fine-riga come al Passo 1; le tabelle, ancora `2` righe nel disegno del 2026-09-04 — più in
basso di otto righe, per il richiamo in testa —; il margine più piccolo di circa 0,2 KB; ADR-0039, `2` occorrenze su `1`
riga — P-15 —; `1`, `1` e *«la + comincia con la -»* — P-17 —; `54` e `1`; `2`; `<data>` come al Passo 1; niente fuori dai
documenti; il comando E, `1` `1` `2` `55`; un diff che nomina i quattro file e questo piano.

- [ ] **Passo 4: il commit e il push**

```bash
git add docs/adr/0039-telecamera-come-sorgente-di-percezione.md docs/COMPENDIO.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md docs/superpowers/specs/2026-09-04-knowledge-base-design.md docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
git commit -m "knowledge-base-revisione(compito 3): il disegno della knowledge base del 2026-09-04 corretto dalla revisione — il richiamo in testa e uno per ogni riga superata, righe 0–23 della sezione 2 — e la cattura come ogni file nuovo in tutte le sue case: la riga di ADR-0039 e la sua voce della §5, le due righe del disegno dei gesti"
git push
```

#### Criterio di chiusura del compito 3

- [ ] il disegno del 2026-09-04 porta il richiamo in testa, sotto il capoverso di stato — D7 —, e un richiamo **nella cella o nel capoverso** di ogni posto che la sezione 2 nomina — D6 —; la cella della decisione 13 dice *«chiusa»*, col suo richiamo
- [ ] la revisione ha riletto ogni richiamo contro la sua riga della sezione 2, e la cattura contro la risposta A della domanda di quella sezione; ADR-0039 contro ADR-0038 e ADR-0018, che nominano la cattura e gli artefatti
- [ ] ADR-0039 cambia nella sola cella del perimetro negativo — P-17 —, e la sua voce della §5 rimanda
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga e tabelle come al Passo 1, commit pushato, posizione aggiornata

---

## Compito 4: la spec del sotto-progetto 1 in due punti, design/09 e design/10

**Files:**
- Modify: `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` — la riga `filesystem` della §2.3, la testa della §6.6 · `docs/design/09-l0-fisico.md` — ADR-0040 nella riga dei rimandi, il richiamo, il diagramma, la tabella · `docs/design/10-modello-dei-dati-durevoli.md` — il richiamo, cinque etichette, tre righe · questo piano — la posizione
- Read: la 4.5 e la 5.4 del disegno, e la domanda 2 della 6.1; P-2, P-12; D8

- [ ] **Passo 1: le misure prima**

```bash
S=<scratchpad>; D=<data>
SPEC=docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
F="$SPEC docs/design/09-l0-fisico.md docs/design/10-modello-dei-dati-durevoli.md"
bash scripts/check-docs.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
awk '/^## 8[.] /{p=1} p' $SPEC | md5sum
awk '/^```mermaid/{m=1;next} /^```/{m=0} m' docs/design/09-l0-fisico.md docs/design/10-modello-dei-dati-durevoli.md | grep -c -e "'" -e nomeapp
grep -c '2026-09-28-knowledge-base-revisione-design' $F
```

Atteso: `OK`; la colonna `w/…` e i CR si annotano; le tabelle, niente; l'impronta della §8 si **annota** — ⛔ è l'invariante
del vincolo 3 —; `0` nei diagrammi — P-12 —; il comando E, `0` `0` `0`.

- [ ] **Passo 2: il blocco E4 — prima il `--check`, poi davvero; e la posizione**

```bash
S=<scratchpad>; D=<data>
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$D" "$S/e4.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/e4.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/pos4.txt"
```

Atteso: `checked: 24 edits in 3 files`, `applied: 24 edits in 3 files`, `applied: 1 edits in 1 files`.

Il blocco E4, in `e4.txt`:

````text
# compito 4 -- la spec del sotto-progetto 1 (due punti soli), design/09 e design/10
@@ docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
>> | `filesystem` — ambiti di checkpoint, artefatti | §4 |
++ — ⚠️ **Richiamo del <data>:** la §4 è il giornale, e **non descrive** questa porta. La porta cresce **a pezzi**: la lettura, la sorgente degli eventi e l'ambito della root col **13**; lo scrivere, anche condizionato, l'elenco, i metadati, spostare, cancellare, conservare e ripristinare, le esclusioni del privato e i percorsi protetti col **6**; aprire e chiudere le zone di lavoro col **5**. Il testo esatto di ciascun pezzo lo scrive chi lo costruisce, col suo richiamo; chi costruisce che cosa sta nella sezione 4 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
^^ Entra la **forma** e la sua registrazione; il mediatore completo, i preset, il ciclo di
<<<
> ⚠️ **RICHIAMO DEL <data> — il permesso cresce, e chi lo costruisce ne scrive qui il testo esatto.** Dal
> [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md), approvato dal
> proprietario il 2026-09-29: la **sessione** di V21 è quella della risposta D11 — la run radice coi suoi discendenti,
> che finisce anche al riavvio del core —, e il record del permesso la porta su un indice nuovo e facoltativo, col **3**;
> i **percorsi protetti** — il file delle esclusioni e la cartella dati del programma —, che nessun sì copre, li
> controlla la porta dei file prima di ogni sì, col **6**; un effetto **irripetibile** chiede a ogni invocazione e non
> per la sessione, con la modifica di `invoke`, col **6**; la correzione deterministica di un fatto segue l'impostazione
> del riconciliatore, col **6**; fuori da ogni zona la **lettura** è una tripla `(file, percorso, lettura)` per la
> sessione, al primo che legge fuori. Chi costruisce che cosa: la sezione 4 del disegno. ⛔ La §8 non cambia.
>>>
@@ docs/design/09-l0-fisico.md
>> [ADR-0025](../adr/0025-confinamento-a-livelli.md).
==
<<<
[ADR-0025](../adr/0025-confinamento-a-livelli.md) ·
[ADR-0040](../adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md).
>>>
>> [stella polare](../superpowers/specs/2026-09-07-direzione-gui-design.md), §2 e decisioni 14–18.
+P
<<<
⚠️ **RICHIAMO DEL <data> — la revisione della knowledge base, e [ADR-0040](../adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md).**
Il diagramma e la tabella si riallineano ad ADR-0040. I **file del proprietario** — la root e le zone di lavoro, guide
comprese — **non** sono nel backup del programma, e la frase del richiamo del 2026-09-08 *«le guide sono file della
cartella della knowledge base, artefatti dell'utente»* si legge così; i **router** stanno in `.<nomeapp>/` alla root, e
sono l'eccezione che il programma salva; le **copie del checkpoint** e l'**indice** del livello strutturale stanno nella
**cartella dati per utente** del programma — il punto 1 di ADR-0040, dove sta ogni suo dato —, fuori dal backup; la
configurazione porta il percorso della root; la porta `filesystem` arriva **a pezzi**, col 13, col 6 e col 5, e non più
*«l'implementazione vera col 5»*; il giornale porta anche la **fiducia alle zone** (col 5) e l'**impronta di ogni
caricamento** di un file-guida (col 13). Il perché sta nella 5.4 del
[disegno della revisione](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md).
>>>
>>         G[("giornale<br/>run, passi, routing, verdetti, costi<br/>permessi, transizioni di policy<br/>invocazioni del registro (col 2)<br/>guide approvate (col 13)")]
==         G[("giornale<br/>run, passi, routing, verdetti, costi<br/>permessi, transizioni di policy<br/>invocazioni del registro (col 2)<br/>guide approvate e impronta di ogni caricamento (col 13)<br/>fiducia alle zone (col 5)")]
>>         C[("configurazione<br/>profili<br/>disposizione dei pannelli (col 2)")]
==         C[("configurazione<br/>profili, percorso della root<br/>disposizione dei pannelli (col 2)")]
>>         A[("artefatti<br/>file prodotti<br/>la cartella della knowledge base (col 6):<br/>router, foglie, guide, catture")]
==
<<<
        R[("router<br/>nella cartella nascosta<br/>alla root (col 6)")]
        A[("file del proprietario<br/>la root e le zone di lavoro<br/>note, guide, catture, file prodotti")]
        P[("copie del checkpoint<br/>nella cartella dati (col 6)")]
>>>
>>         I[("indici<br/>embedding, RAG<br/>indice della mappa (col 6)")]
==         I[("indici<br/>embedding, RAG<br/>indice strutturale, uno per root (col 6)")]
>>     A --> B
==
<<<
    R --> B
    A -.->|"fuori:<br/>backup del proprietario e git"| B
    P -.->|"fuori:<br/>si potano"| B
>>>
>>     class C,A,I,M plain
==     class C,R,A,P,I,M plain
>> | configurazione: i profili e, col 2, la disposizione dei pannelli |
++ e il percorso della root (ADR-0040)
>> | artefatti prodotti e, col 6, la cartella della knowledge base | no — sono già file dell'utente | sì | no | il kernel dalla porta `filesystem` (ambiti e checkpoint, ADR-0024); l'implementazione vera col 5 |
==
<<<
| i file del proprietario: la root — note, guide, catture, i file che le run vi producono — e le zone di lavoro | no — sono suoi | **no**: li salvano i suoi backup e git (ADR-0040) | no | il kernel dalla porta `filesystem` (ambiti e checkpoint, ADR-0024), a pezzi: la lettura col 13, lo scrivere col 6, le zone col 5 |
| i router, in `.<nomeapp>/` alla root, marcata nascosta dal modulo di piattaforma (col 6) | no | **sì** — l'eccezione di ADR-0040 | no: portano le scelte del proprietario | li scrive il 6, con un sì per la sessione |
| le copie del checkpoint, nella cartella dati del programma (col 6) | no — in chiaro, come i file che copiano | **no** | no | la porta `filesystem`, `preserve` e `restore`, col 6; potate come vuole ADR-0018 |
>>>
>> | indici ed embedding e, col 6, l'indice della mappa | no | **no** | sì, dai documenti |
== | indici ed embedding e, col 6, l'indice del livello strutturale, uno per root, nella cartella dati del programma (ADR-0040) | no | **no** | sì, dai documenti |
@@ docs/design/10-modello-dei-dati-durevoli.md
>> ## Deciso e non costruito, per sotto-progetto
+P ⚠️ **RICHIAMO DEL <data> — la revisione della knowledge base.** Cinque etichette di questo diagramma e tre righe della tabella dicevano ciò che il [disegno della revisione](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) ha superato: l'`AMBITO` è la root dichiarata dalla configurazione col **13**, e le zone di lavoro aperte col **5**; il `CHECKPOINT` — conservare e ripristinare sui file veri — arriva col **6**, la sezione 4 di quel disegno; la `CARTELLA_KB` è la root, una cartella qualsiasi del proprietario, e le sue politiche stanno in design/09, come questo file dice in testa; un `NODO_KB` è **ogni file indicizzato**, e l'`INDICE_MAPPA` lega i nodi con tre specie di linea — la cartella, l'area, il link —, righe 11 e 14 della sua sezione 2; per il file-guida di una zona la `GUIDA_APPROVATA` si approva per fiducia alla cartella — D9.
>>     AMBITO ||--o| CARTELLA_KB : "la cartella e un ambito dichiarato (col 6)"
==     AMBITO ||--o| CARTELLA_KB : "la root, un ambito dichiarato dalla configurazione (col 13)"
>>     NODO_KB }|--|| INDICE_MAPPA : "frecce router, gruppo, foglia, skill, router - segnali orfano e rotto (col 6)"
==     NODO_KB }|--|| INDICE_MAPPA : "tre specie di linea - cartella, area, link - segnali orfano e rotto (col 6)"
>>         arriva col_5 "la porta filesystem esiste gia - declare_scope, preserve, restore"
==         arriva col_13_e_col_5 "la root dichiarata col 13, le zone aperte col 5 - declare_scope esiste gia"
>>         arriva col_5 "CheckpointId esiste gia nella porta"
==         arriva col_6 "CheckpointId esiste gia - preserve e restore veri col 6"
>>         arriva col_6 "artefatti dell utente - in chiaro, nel backup"
==         arriva col_6 "la root - una cartella qualsiasi del proprietario, dalla configurazione"
>>         enum specie "router gruppo foglia skill guida-modello cattura"
==         enum specie "ogni file indicizzato - il rumore compreso, il privato no"
>> | `AMBITO`, `CHECKPOINT` | 5 |
== | `AMBITO`, `CHECKPOINT` | 13, 6 e 5 |
>> l'implementazione vera col 5 |
++ ⚠️ **Richiamo del <data>:** a pezzi — la root dichiarata col 13, conservare e ripristinare col 6, le zone col 5: D15, la sezione 4 del [disegno della revisione](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> ; la cartella è un ambito dichiarato |
++ ⚠️ **Richiamo del <data>:** la cartella è la root, dalla configurazione; ogni file indicizzato è un nodo; tre specie di linea — righe 11 e 14 della sezione 2 del [disegno della revisione](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> provenienza e impronta come dettaglio di una nota |
++ ⚠️ **Richiamo del <data>:** per il file-guida di una zona l'approvazione è la fiducia alla cartella, col 5, e l'impronta si scrive a ogni caricamento, col 13 — D9, la 4.3 del [disegno della revisione](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
````

Il blocco POS4, in `pos4.txt`:

````text
# la posizione del compito 4, nello stesso commit
@@ docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
>= | **4** |
== | **4** | la spec del sotto-progetto 1 in due punti, design/09 e design/10 | uno | ✅ <data> |
````

- [ ] **Passo 3: le prove**

```bash
S=<scratchpad>; D=<data>
SPEC=docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
F="$SPEC docs/design/09-l0-fisico.md docs/design/10-modello-dei-dati-durevoli.md"
bash scripts/check-docs.sh
bash scripts/gate.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
awk '/^## 8[.] /{p=1} p' $SPEC | md5sum
git diff -U0 -- $SPEC | grep -c '^@@'
awk '/^```mermaid/{m=1;next} /^```/{m=0} m' docs/design/09-l0-fisico.md docs/design/10-modello-dei-dati-durevoli.md | grep -c -e "'" -e nomeapp
grep -n 'class C,R,A,P,I,M plain' docs/design/09-l0-fisico.md | cut -c1-40
git status --porcelain -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' $F
git diff --stat
```

Atteso: `OK` e `GATE GREEN`; fine-riga e tabelle come al Passo 1; l'impronta della §8 **uguale** a quella del Passo 1; `2`
pezzi nella spec — la riga della §2.3 e la testa della §6.6 —; `0` nei diagrammi; una riga, i nodi nuovi `R` e `P` nella
classe; niente fuori dai documenti; il comando E, `2` `1` `4`; un diff che nomina i tre file e questo piano. Il blocco A
della §6 del compendio — il ritratto della §8 — rende lo stesso di prima, e lo dice già l'impronta.

- [ ] **Passo 4: il commit e il push**

```bash
git add docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md docs/design/09-l0-fisico.md docs/design/10-modello-dei-dati-durevoli.md docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
git commit -m "knowledge-base-revisione(compito 4): la spec del sotto-progetto 1 in due punti — la porta dei file che cresce a pezzi, nella riga filesystem della §2.3, e il permesso che cresce, in testa alla §6.6 —, design/09 riallineato ad ADR-0040 nel diagramma e nella tabella, e design/10 coi richiami sulle etichette superate; la §8 non cambia"
git push
```

#### Criterio di chiusura del compito 4

- [ ] la spec cambia nei **due** punti soli, e la §8 ha la stessa impronta
- [ ] design/09 dice nel diagramma e nella tabella la stessa cosa — i router nel backup, i file del proprietario e le copie fuori, l'indice uno per root —, e la revisione l'ha riletto contro la 5.4 e contro ADR-0040
- [ ] design/10 porta il richiamo, le cinque etichette e le tre righe — P-2 —, e nessun diagramma ha un apostrofo o un segnaposto fra parentesi angolari — P-12
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga e tabelle come al Passo 1, commit pushato, posizione aggiornata

---

## Compito 5: `roadmap.md`, `tracciabilita.md` e la stella polare della GUI

**Files:**
- Modify: `docs/roadmap.md` — l'*«Ultimo aggiornamento»*, le righe 3, 5, 6, 11 e 13, *«Backup dopo indici e pesi»*, *«Il primo valore utile»* · `docs/tracciabilita.md` — undici righe · `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` — i sette punti della 5.3, e il modulo Backup · questo piano — la posizione
- Read: la 5.1, la 5.2, la 5.3 e la 5.5 del disegno; P-3, P-10, P-11; D9, D11, D18

- [ ] **Passo 1: le misure prima**

```bash
S=<scratchpad>; D=<data>
F="docs/roadmap.md docs/tracciabilita.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md"
TR='^[|] (Multi-repo|Mappa del progetto|Git e gestione|Collezioni|File watching|Sessioni multiple|Selettore di modello|Backup della KB|Sandboxing|Permessi e sandbox|Modalità di permessi)'
bash scripts/check-docs.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^[|] .* [|] $s [|]" docs/tracciabilita.md; done
grep -cE "$TR" docs/tracciabilita.md
grep -cE '^[|] (3|5|6|10|11|13) [|]' docs/roadmap.md
grep -c '2026-09-28-knowledge-base-revisione-design' $F
```

Atteso: `OK`; la colonna `w/…` e i CR si annotano; le tabelle, `1` riga nella stella polare — P-18 —; per stato
`✅ 47`, `🔶 54`, `📋 76`, `⚠️ 0`, `❌ 1` — P-11 —; `11` righe della 5.2; `6` righe della roadmap; il comando E, `0` `0`
`0`.

- [ ] **Passo 2: il blocco E5 — prima il `--check`, poi davvero; e la posizione**

```bash
S=<scratchpad>; D=<data>
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$D" "$S/e5.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/e5.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/pos5.txt"
```

Atteso: `checked: 31 edits in 3 files`, `applied: 31 edits in 3 files`, `applied: 1 edits in 1 files`.

Il blocco E5, in `e5.txt`:

````text
# compito 5 -- roadmap, tracciabilita' e stella polare
@@ docs/roadmap.md
>= Ultimo aggiornamento:
== Ultimo aggiornamento: **<data>**, col compito 5 del [piano dei documenti della revisione della knowledge base](superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md): le righe 3, 5, 6, 11 e 13 dei sotto-progetti, *«Backup dopo indici e pesi»* e *«Il primo valore utile»*.
>> | 3 | Conversazione | L2 |
== | 3 | Conversazione — ⚠️ **Richiamo del <data>:** porta anche la sessione, il gateway vero e il selettore del modello; chi costruisce che cosa sta nella sezione 4 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) | L2 |
>> | 5 | Coding | L2 | ⬜ | 4, 0b |
== | 5 | Coding — ⚠️ **Richiamo del <data>:** porta anche aprire e chiudere le zone di lavoro, la domanda di fiducia alla zona col suo record, e il livello 2 per i comandi; la sezione 4 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) | L2 | ⬜ | 4, 0b |
>> §4.4 | L2 |
== §4.4. ⚠️ **Richiamo del <data>:** cade *«archivio unico»*: la root è una cartella qualsiasi, dalla configurazione. La prima metà porta anche il livello strutturale con la ricerca testuale, la scansione e il riconciliatore con la sua impostazione, lo scrivere e il resto della porta dei file, i percorsi protetti, la cartella dati per utente e la cartella nascosta dei router, se nessuno le porta prima; la seconda resta la somiglianza — la sezione 4 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) | L2 |
>> ), §1.1d–e e §2 | L0 + L1 |
== ), §1.1d–e e §2. ⚠️ **Richiamo del <data>:** cresce — la lettura della porta dei file con l'ambito della root, la sorgente degli eventi col «riscansiona», l'implementazione vera in `platform` col doppio del simulatore e la suite di conformità, la finestra in `gateway::Candidate` con la proiezione per candidato, il registro delle guide nella forma che ammette la fiducia alla cartella; la dipendenza non cambia — la sezione 4 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) | L0 + L1 |
>> | 11 | **Backup e ripristino** — [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md); chiude **V32 · V33 · Q21** | L0 + L3 | ⬜ | 5, 6, 9 |
== | 11 | **Backup e ripristino** — [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md), modificato in parte da [ADR-0040](adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md); chiude **V32 · V33 · Q21** | L0 + L3 | ⬜ | 6, 9 — ⚠️ **richiamo del <data>:** qui stava *«5, 6, 9»*: il solo motivo del 5 era il filesystem reale, che arriva a pezzi — la seconda domanda della 5.5 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md), risposta A |
>> Serve inoltre il filesystem reale, che arriva con 5, e l'interfaccia che dichiara le esclusioni **al momento del backup** (follow-up di ADR-0022) |
== Serve inoltre l'interfaccia che dichiara le esclusioni **al momento del backup** — il punto 4 di [ADR-0040](adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md), seguito di ADR-0022. ⚠️ **Richiamo del <data>:** qui stava anche *«il filesystem reale, che arriva con 5»*: la porta dei file arriva a pezzi, col 13, col 6 e col 5, e il 5 non è più un motivo — la 5.5 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) |
>> Sotto-progetti 1 + 2 + 3:
== Sotto-progetti 1 + 2 + 13 + 3:
>> smette di essere solo documenti.
++ ⚠️ **Richiamo del <data>:** qui stava *«1 + 2 + 3»*: il 13 sta prima del 3 dal 2026-09-04 — la decisione 16 del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md) —, e la riga non lo diceva; lo ha trovato la 5.1 del [disegno della revisione](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md).
>> che non riusciva a dare un numero all'innesco di V32, V33 e Q21 |
++ ⚠️ **Richiamo del <data>:** dopo **6 e 9** — il 5 esce, perché il suo solo motivo era il filesystem reale, che arriva a pezzi: la riga 11 qui sopra, e la seconda domanda della 5.5 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md), risposta A
@@ docs/tracciabilita.md
>> | Selettore di modello per compito | ✅ | §3 · profili |
++ ⚠️ **Richiamo del <data>:** il selettore della sessione, come Claude Desktop, → **3**, e il modello scritto nella definizione di un sotto-agente — D10 del [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> | Sessioni multiple | 🔶 | gerarchia §3 (ADR-0011) · politica → Conversazione |
++ ⚠️ **Richiamo del <data>:** la sessione di D11 — la run radice coi suoi discendenti — → **3** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> il sensore d'integrità, l'indice e il pannello; poi la ricerca, seconda metà — [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md) |
++ ⚠️ **Richiamo del <data>:** cade *«archivio unico»*: la root, una cartella qualsiasi dalla configurazione; i due livelli, lo strutturale e i router; il riconciliatore — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> la sorveglianza dei file in `platform` · politica → **6** |
++ ⚠️ **Richiamo del <data>:** al **13** il sorvegliante e la scansione all'avvio, col *«riscansiona»*; la politica al **6** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> | Multi-repo/multi-progetto | 📋 | Conoscenza |
++ ⚠️ **Richiamo del <data>:** le zone di lavoro → **5**, e la scheda progetto nella knowledge base → **6** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> | Mappa del progetto | 📋 | Conoscenza |
++ ⚠️ **Richiamo del <data>:** le zone di lavoro → **5**, e la scheda progetto nella knowledge base → **6** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> | Git e gestione branch | 📋 | Coding |
++ ⚠️ **Richiamo del <data>:** la repo è una zona di lavoro, col **5** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> | Backup della KB indipendente dall'app | 🔶 | §10 · ADR-0022 — documenti nel backup, indice ricostruito · implementazione → Backup e ripristino |
++ ⚠️ **Richiamo del <data>:** cade *«documenti nel backup»*: la root è nei backup del proprietario, i router in quello del programma — [ADR-0040](adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md), e il [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> | Sandboxing ed esecuzione | 🔶 | permessi §6 + §10 · ADR-0025 — confinamento reale → Coding |
++ ⚠️ **Richiamo del <data>:** i percorsi protetti col **6**; la lettura fuori da ogni zona, col permesso e l'impostazione che la blocca, al primo che legge fuori; il livello 2 col **5** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> `PermissionRequired` e `Approve` — confinamento reale → Coding |
++ ⚠️ **Richiamo del <data>:** i percorsi protetti col **6**; la lettura fuori da ogni zona, col permesso e l'impostazione che la blocca, al primo che legge fuori; il livello 2 col **5** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
>> | Modalità di permessi a più livelli | 🔶 | preset §6 · implementazione → Agenti |
++ ⚠️ **Richiamo del <data>:** i percorsi protetti col **6**; la lettura fuori da ogni zona, col permesso e l'impostazione che la blocca, al primo che legge fuori; il livello 2 col **5** — [disegno della revisione della knowledge base](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md)
@@ docs/superpowers/specs/2026-09-07-direzione-gui-design.md
>> La rete al centro ha **tutto**: artefatti e file della knowledge base messi dal proprietario |
++ ⚠️ **Richiamo del <data>:** la rete ha tutto ciò che sta **nella root**; un file che l'agente scrive in una zona fuori dalla root sta nell'anello, e nella rete attraverso la scheda della zona — D13 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> decisione strutturale che non si prende a fine giornata |
++ ⚠️ **Richiamo del <data>:** **deciso** — il modello si sceglie a mano accanto al pulsante di invio, per la sessione o come default, come Claude Desktop, e ogni risposta porta il nome del modello — D10 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> | 3 | verificato, decisione 11; il selettore a mano nelle registrate |
++ ⚠️ **Richiamo del <data>:** il selettore a mano è **deciso** — D10 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> | se nella barra della chat il **modello** si possa anche **scegliere a mano, per run**
== | ✅ **chiusa il <data> — D10 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md):** il modello si sceglie a mano accanto al pulsante di invio, per la sessione o come default, come Claude Desktop, e ogni risposta porta il nome del modello. Era — se nella barra della chat il **modello** si possa anche **scegliere a mano, per run**
>> Registrata: se il preset «auto-approva sicuri» approvi le letture ovunque o solo dentro l'ambito |
++ ⚠️ **Richiamo del <data>:** l'ambito della run diventa la **zona di lavoro**, aperta per la sessione, e la root c'è sempre; il resto — la copia prima delle modifiche, la domanda che dice se il percorso è dentro o fuori — regge; e la registrata sulle letture è chiusa da D18 — [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> dentro l'ambito (copia di sicurezza) o fuori (nessuna) |
++ ⚠️ **Richiamo del <data>:** l'ambito della run diventa la **zona di lavoro**, aperta per la sessione, e la root c'è sempre; il resto regge — D3 e D11 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> | se il preset «auto-approva sicuri» approvi le **letture ovunque**
== | ✅ **chiusa il <data> — D18 del [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md):** dentro la root e le zone la lettura procede; fuori chiede, un file alla volta, per la sessione. Era — se il preset «auto-approva sicuri» approvi le **letture ovunque**
>> propone l'ultima usata | il **3** |
++ ⚠️ **Richiamo del <data>:** la zona è della sessione, e gli ambiti sono della porta e non della run — D3, D11, K35; il resto della domanda, la cartella proposta a una nuova run, resta al 3 — [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> nell'anello solo file, per data (domanda 1) |
++ ⚠️ **Richiamo del <data>:** la ricerca nel **testo**, coi filtri per tipo e per cartella; la griglia accanto al grafo; l'anteprima al click; tre specie di linea; il segnale rotto anche per il file chiave perso — [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> indici ed embedding, pesi dei modelli, e mai i segreti; il ripristino |
++ ⚠️ **Richiamo del <data>:** resta fuori anche ciò che [ADR-0040](../../adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md) toglie dal backup del programma — la root, le zone di lavoro, le copie del checkpoint —, e il modulo lo dice quando il backup si crea; i router invece ci sono — [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
>> se sia un modulo a sé lo decide il 5 |
++ ⚠️ **Richiamo del <data>:** le copie stanno nella cartella dati del programma, fuori dal backup, e dopo un ripristino un passo di prima non si annulla — [ADR-0040](../../adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md), e il [disegno della revisione della knowledge base](2026-09-28-knowledge-base-revisione-design.md)
````

Il blocco POS5, in `pos5.txt`:

````text
# la posizione del compito 5, nello stesso commit
@@ docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
>= | **5** |
== | **5** | `roadmap.md`, `tracciabilita.md` e la stella polare della GUI | uno | ✅ <data> |
````

- [ ] **Passo 3: le prove**

```bash
S=<scratchpad>; D=<data>
F="docs/roadmap.md docs/tracciabilita.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md"
TR='^[|] (Multi-repo|Mappa del progetto|Git e gestione|Collezioni|File watching|Sessioni multiple|Selettore di modello|Backup della KB|Sandboxing|Permessi e sandbox|Modalità di permessi)'
G=docs/superpowers/specs/2026-09-07-direzione-gui-design.md
bash scripts/check-docs.sh
bash scripts/gate.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^[|] .* [|] $s [|]" docs/tracciabilita.md; done
grep -cE "$TR" docs/tracciabilita.md; grep -E "$TR" docs/tracciabilita.md | grep -c '2026-09-28-knowledge-base-revisione-design'
grep -cE '^[|] (3|5|6|10|11|13) [|]' docs/roadmap.md
git diff -U0 -- docs/roadmap.md | grep -c '^-| 10 |'
grep -c "Richiamo del $D" $G; grep -c "chiusa il $D" $G
grep -c '0040-dove-vivono-i-dati' $G
git status --porcelain -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' $F
git diff --stat
```

Atteso: `OK` e `GATE GREEN`; fine-riga e tabelle come al Passo 1; il conto per stato **identico** al Passo 1; `11` righe, e
`11` col link; `6` righe; `0` — la riga 10 intatta —; `9` richiami e `2` voci chiuse nella stella polare, cioè gli undici
posti dei sette punti della 5.3 e del modulo Backup; `2` link ad ADR-0040, il Backup e il Checkpoint; niente fuori dai
documenti; il comando E, `7` `11` `11`; un diff che nomina i tre file e questo piano.

- [ ] **Passo 4: il commit e il push**

```bash
git add docs/roadmap.md docs/tracciabilita.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
git commit -m "knowledge-base-revisione(compito 5): la roadmap — le righe 3, 5, 6, 11 e 13 col rimando alla 4.2, l'11 che dipende da 6 e 9, Backup dopo indici e pesi, il primo valore utile con il 13 —, undici righe di tracciabilita e la stella polare della GUI coi richiami dei sette punti e del modulo Backup"
git push
```

#### Criterio di chiusura del compito 5

- [ ] ogni riga della 5.1 porta la frase corta e il rimando alla 4.2 — la risposta 1 della 5.5 —; l'11 dipende da **6, 9**, col richiamo — la risposta 2 —; la riga 10 intatta, nessuna riga rinumerata
- [ ] le undici righe della 5.2 col link, e il conto per stato identico — nessuno stato cambia, D11
- [ ] la stella polare: i sette punti della 5.3 e il modulo Backup — P-3 —, ciascuno col richiamo datato; le due voci registrate chiuse, col loro richiamo
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga e tabelle come al Passo 1, commit pushato, posizione aggiornata

---

## Compito 6: la chiusura — gli indici, la data e il puntatore, la Definizione di «fatto» eseguita

**Files:**
- Modify: `docs/README.md` — il disegno fra le specifiche · `docs/COMPENDIO.md` — la riga della data, la §12, il puntatore della §6 · `docs/roadmap.md` — l'*«Ultimo aggiornamento»* e la riga di questo piano · `docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md` — la spunta del punto 3 della 6.6 · `docs/archivio/stato-storico.md` — la riga della data e il puntatore com'erano · questo piano — la posizione, la riga in testa alla tabella
- Read: la 6.2 e la 6.6 del disegno; D12, D13, D16, D22; la §6 del compendio, il puntatore e il paragrafo 🆕 dei modelli decisionali

- [ ] **Passo 1: le misure prima**

```bash
S=<scratchpad>; D=<data>
F="docs/README.md docs/COMPENDIO.md docs/roadmap.md docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md docs/archivio/stato-storico.md"
bash scripts/check-docs.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
grep -c '^⏭️ [*][*]IL PROSSIMO PASSO' docs/COMPENDIO.md
grep -c '2026-09-28-knowledge-base-revisione-design' $F
```

Atteso: `OK`; la colonna `w/…` e i CR si annotano; le tabelle, niente; il margine positivo — la simulazione dava `4653` —;
`1` inizio del puntatore; il comando E, `0` `1` `7` e, sul disegno e sull'archivio, ciò che rendono — si annota.

- [ ] **Passo 2: il puntatore e la riga della data in archivio — PRIMA del blocco**

```bash
S=<scratchpad>; D=<data>
PYTHONIOENCODING=utf-8 python "$S/archive_head.py" --pointer "Il puntatore «Il prossimo passo» e l'intestazione del compendio, com'erano — archiviati il $D, alla chiusura del piano dei documenti della revisione della knowledge base"
```

Atteso: `archived: 2 piece(s) under «…»`.

- [ ] **Passo 3: il blocco E6, poi il puntatore nuovo, poi la posizione**

```bash
S=<scratchpad>; D=<data>
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$D" "$S/e6.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/e6.txt"
PYTHONIOENCODING=utf-8 python "$S/replace_pointer.py" "$D" "$S/pointer6.txt"
PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" "$D" "$S/pos6.txt"
```

Atteso: `checked: 6 edits in 4 files`, `applied: 6 edits in 4 files`, `replaced: lines …-… with 10 line(s)`,
`applied: 2 edits in 1 files`. ⚠️ Il blocco E6 cerca la cella *«⏳ **scritto il 2026-09-29**; il pre-controllo e
l'esecuzione, in sessioni loro»* in `roadmap.md` e l'inizio del punto 3 della 6.6 del disegno: le ha scritte la chiusura
della sessione che ha finito il piano, e se una sessione dopo le cambia, il blocco si riallinea con una voce d'errata.

Il blocco E6, in `e6.txt`:

````text
# compito 6 -- la chiusura (il puntatore lo scrive replace_pointer.py, dopo archive_head.py; non questo blocco)
@@ docs/README.md
>= | [Knowledge base — il disegno](superpowers/specs/2026-09-04-knowledge-base-design.md)
+L | [Knowledge base, la revisione — il disegno](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) | la revisione della knowledge base: dove vivono i file, chi li scrive e chi li legge, e che cosa salva il programma | ⛔ **Non è una spec**, e **non disegna la capacità**: il modello nuovo dopo le risposte D1–D20 del proprietario, i richiami al disegno del 2026-09-04, ADR-0040 e i rimandi in testa agli ADR, chi costruisce che cosa, e per ogni artefatto il controllo che lo esercita; eseguito dal [piano dei documenti](superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md) |
@@ docs/COMPENDIO.md
>= **Aggiornato il
== **Aggiornato il <data>**, con la **chiusura del piano dei documenti della revisione della knowledge base** — il puntatore della §6 e la riga della §12 —; il puntatore e questa riga com'erano sono in [`archivio/stato-storico.md`](archivio/stato-storico.md). Il contenuto di merito nuovo è la voce di ADR-0040, coi rimandi nelle voci della §5. Manutenzione, e perché questa riga è la più facile da lasciare indietro: §13.
>> il [piano](superpowers/plans/2026-09-04-knowledge-base-documenti.md), con l'errata in testa e la tabella della posizione — ⚠️ **a compiti, mai intero** |
++ · la [revisione del 2026-09-28](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) — il modello nuovo, chi costruisce che cosa, ADR-0040 — col suo [piano](superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md): ⚠️ **a sezioni e a compiti, mai interi**
@@ docs/roadmap.md
>= Ultimo aggiornamento:
== Ultimo aggiornamento: **<data>**, con la **chiusura del [piano dei documenti della revisione della knowledge base](superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md)**: la sua riga nella tabella dei piani portata a «eseguito» dal compito 6.
>> | ⏳ **scritto il 2026-09-29**; il pre-controllo e l'esecuzione, in sessioni loro |
== | ✅ **scritto il 2026-09-29, eseguito il <data>** — `GATE GREEN` a ogni compito; nessun file di `crates/`, `gui/` o `scripts/` toccato |
@@ docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md
>> 3. ✅ il **piano dei documenti** — **scritto il 2026-09-29**, in una sessione sua, al suo
== 3. ✅ il **piano dei documenti** — **scritto il 2026-09-29** ed **eseguito il <data>**, al suo
````

Il puntatore nuovo della §6 — D22: la revisione chiusa, i due fronti **senza** un ordine fra loro; `<data>` lo mette
`replace_pointer.py`; in `pointer6.txt`:

````text
⏭️ **IL PROSSIMO PASSO. La REVISIONE DELLA KNOWLEDGE BASE è CHIUSA il <data>**: il [disegno](superpowers/specs/2026-09-28-knowledge-base-revisione-design.md),
riletto dal proprietario il 2026-09-29, e il suo [piano dei documenti](superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md), eseguito —
ADR-0040, i rimandi e i richiami. Vengono **due fronti**, e il proprietario **non** li ha messi in ordine l'uno rispetto all'altro.
**IL SOTTO-PROGETTO 13** — i meccanismi che la knowledge base chiede al kernel, che la **decisione 16** mette **prima** del 3 —, col
perimetro della **4.2** del disegno della revisione: la lettura della porta dei file, la sorgente degli eventi, la finestra del
candidato, il registro delle guide che ammette la fiducia alla cartella. ⛔ **Lo sbarra AUD-004**, l'ADR del proprietario sulle skill
dichiarative: è una sua decisione, e viene prima. Chi riprende il 13 legge **per intero** il
[disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md) e la sezione 4 della revisione; brainstorming e
disegno del 13 vengono prima del piano, in sessioni distinte. E **il brainstorming dei modelli decisionali**, il fronte nuovo qui sotto,
che il proprietario ha messo **dopo** questa revisione.
````

Il blocco POS6, in `pos6.txt` — la riga del compito e quella in testa alla tabella:

````text
# la posizione del compito 6, e la riga in testa alla tabella, nello stesso commit
@@ docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
>= ✅ **IL PIANO È
== ✅ **IL PIANO È ESEGUITO, il <data>.** A dirlo non è questa riga ma la tabella qui sotto — ogni riga ✅ — e la **Definizione di «fatto»**, eseguita al compito 6. Il passo dopo lo dice la §6 del compendio.
>= | **6** |
== | **6** | la chiusura: gli indici, la data e il puntatore, la Definizione di «fatto» eseguita | uno | ✅ <data> — la Definizione di «fatto» eseguita, ogni riga verde; le uscite nel rapporto del compito, nella cartella del dispaccio |
````

- [ ] **Passo 4: la Definizione di «fatto», riga per riga**

Ogni riga della sezione *«La Definizione di «fatto»»*, col suo controllo, sul repository di adesso: i comandi A–E, la
guardia dei totali, e le righe che si leggono. Le uscite vanno nel rapporto del compito, nella cartella del dispaccio —
non in questo piano: una cifra scritta qui invecchierebbe (vincolo 8). Atteso: ogni riga verde; il comando A, uno in più
per ciascuno dei dodici ADR rispetto al Passo 1 del compito 1 e del compito 2 — `2` per 0009, 0010, 0011, 0022, 0038 e
0039, `1` per gli altri sei —; B niente; C positivo — la simulazione dava `4774` —; D niente da `<base>`; E almeno `1` su
ogni file della mappa che il piano linka.

- [ ] **Passo 5: le prove**

```bash
S=<scratchpad>; D=<data>
F="docs/README.md docs/COMPENDIO.md docs/roadmap.md docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md docs/archivio/stato-storico.md"
bash scripts/check-docs.sh
bash scripts/gate.sh
git ls-files --eol $F
for f in $F; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
grep -c 'CHIUSA il' docs/COMPENDIO.md
grep -c "eseguito il $D" docs/roadmap.md docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md
grep -n '^## Il puntatore' docs/archivio/stato-storico.md | tail -1 | cut -c1-80
git status --porcelain -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
git diff --stat <base>..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' $F
git diff --stat
```

Atteso: `OK` e `GATE GREEN`; fine-riga e tabelle come al Passo 1; il margine positivo; `1`; `1` e `1`; l'intestazione di
questo giorno; niente e niente; il comando E, `1` su `README.md` e `2` sul compendio — il puntatore e la §12 —, `7` sulla
roadmap; un diff che nomina i cinque file e questo piano.

- [ ] **Passo 6: il commit e il push**

```bash
git add docs/README.md docs/COMPENDIO.md docs/roadmap.md docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md docs/archivio/stato-storico.md docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
git commit -m "knowledge-base-revisione(compito 6): la chiusura — il disegno della revisione in README e nella §12 del compendio, la riga della data e il puntatore della §6 con le loro copie in archivio, la riga del piano in roadmap e la spunta nel disegno portate a eseguito, la Definizione di «fatto» eseguita"
git push
```

#### Criterio di chiusura del compito 6

- [ ] il puntatore della §6 dice la revisione chiusa e i due fronti, **senza** un ordine fra loro; il puntatore e la riga della data com'erano stanno in `stato-storico.md`, parola per parola coi link riscritti
- [ ] il disegno in `README.md` e nella §12; la riga di questo piano in roadmap e il punto 3 della 6.6 a *«eseguito»*
- [ ] la Definizione di «fatto» eseguita riga per riga, e le uscite nel rapporto
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga e tabelle come al Passo 1, commit pushato, posizione aggiornata

---

## Dopo il compito 6

La chiusura della **sessione** di esecuzione — `CLAUDE.md`, *«Manutenzione della documentazione»* — scrive il *«Come si
riprende»* di questo piano, con la precedente in archivio; il puntatore lo ha già mosso il compito 6. ⛔ **Nessun
sotto-progetto si chiude** con questo piano: `roadmap.md`, `README.md` e `tracciabilita.md` li tocca già il piano, e
`HANDOFF.md` solo se l'esecuzione trova un gotcha nuovo.

---

## Come si riprende — scritto alla chiusura del pre-controllo, il 2026-09-29, coi comandi

⛔ **Da sapere subito: niente è a metà.** Il piano è **pre-controllato**: sette voci nell'errata, ER-1…ER-7, e i sei
compiti simulati **in sequenza** su un `git worktree` nello scratchpad, poi tolto — `check-docs.sh` → `OK` dopo ciascuno, e
il margine del compendio sempre positivo. **Nessun compito è eseguito.** Tutto è pushato: si riparte anche dall'altra
macchina, dopo il fetch. La consegna precedente, quella al pre-controllo, sta in
[archivio](../../archivio/consegna-piano-knowledge-base-revisione-documenti.md), parola per parola.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, `git status -sb`; nessuno stash, e nessun worktree oltre al principale — `git worktree list` |
| i commit del pre-controllo | `git log --oneline c8322f1..HEAD`: la voce ER-1, e la chiusura con le altre sei |
| codice di prodotto | **non toccato**: il comando D da `<base>` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK` all'apertura e prima di ogni commit: si rilanciano, non si citano |
| il margine del compendio | il comando C |
| i pezzi | `extract.py` copiato a mano dal suo recinto, poi il comando della sezione *«Gli attrezzi»*: diciotto righe |
| i blocchi | il `--check` di ciascuno, dal comando sotto la tabella: tutti e sei `checked`, E5 con trentadue modifiche — ER-5 |
| il puntatore | la §6 del compendio: il piano è pre-controllato, e viene l'**esecuzione** |

```bash
S=<scratchpad>; for n in 1 2 3 4 5 6; do printf 'E%s: ' $n; PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$(date +%F)" "$S/e$n.txt" 2>&1 | tail -1; done
```

**Il compito della sessione che riprende — il compito 1, e prima il dispaccio:**

1. La lettura d'apertura di `CLAUDE.md`; poi l'**errata** di questo piano per intero, e il compito 1 — tutto e
   nient'altro — con le sezioni del disegno che nomina. Lo toccano ER-2, ER-3, ER-4 ed ER-6.
2. `bash scripts/gate.sh`, da solo; l'estrazione dei pezzi e il `--check` dei sei blocchi.
3. ⏳ **Il passo 4 del pre-controllo, spostato qui dal proprietario il 2026-09-29 — risposta A:** la cartella del dispaccio —
   D17 —, che nasce al primo dispaccio, col prompt del compito 1 come **modello** e i campi della macchina da riempire; il
   precedente è `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`. Poi il **costo** dell'implementatore e del
   revisore, detto al proprietario **prima**, con la banda misurata dei dispacci recenti scalata sul peso del brief, il
   modello di ciascuno — `opus`, `sonnet` per il lavoro meccanico —, e il sì.
4. Il compito 1, un subagente fresco e la revisione con la regola 5 di *«Come si esegue un compito di questo piano»*; poi la
   chiusura della sessione: questa sezione riscritta, e questa in archivio, nello stesso file delle consegne di prima.

**Le decisioni prese in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | le voci d'errata si chiamano **ER-n** | *E1…E6* sono i blocchi, e una voce «E4» si leggerebbe come il blocco. Costo: un prefisso nuovo nel repository |
| 2 | il passo visivo disegnato con mermaid **11**, e letto anche con la **12.0.0** | la 11 è quella del precedente del 2026-09-08 e del widget; la 12.0.0 è l'ultima su npm, appena uscita, e un diagramma che le due leggono regge in entrambe. Costo: nessuno |
| 3 | con l'impronta uguale, al compito 4 **nessun secondo sguardo** del proprietario | un'impronta uguale è il disegno già visto, e richiederlo sarebbe una conferma per rito; la A presentata diceva *«due tuoi sguardi»*, e la differenza è detta qui. Costo: uno sguardo, se il proprietario lo vuole comunque |
| 4 | alla chiusura, la cella di questo piano in `roadmap.md` e il punto 3 della 6.6 del disegno **non** si toccano | sono le due ancore del blocco E6, e cambiarle vorrebbe dire riallinearlo; la cella dice già che pre-controllo ed esecuzione stanno in sessioni loro. Costo: la roadmap non dice *«pre-controllato»* fino al compito 6 |
| 5 | `HANDOFF.md`, la riga 2 della tabella di F2, **registrata** con P-5 e non toccata — ER-5 | la riga 14 della Definizione di «fatto», approvata col disegno, vuole da `HANDOFF.md` solo i totali, e quella riga è il consuntivo della §8.5.2. Costo: fino alla decisione del proprietario dice *«5, 6 e 9»* |
| 6 | la voce chiusa della roadmap *«Dove vive backup e ripristino»* **toccata** — ER-5 | è lo stesso file del compito, dove la riga 11 direbbe *«6, 9»* poco sopra, e il merito è la risposta A della 5.5. Costo: una modifica in più nel blocco E5 |

**Vicoli ciechi di questa sessione:**

| | Che cosa insegna |
|---|---|
| il widget della chat non si legge da qui — `read_widget_context` non ha contesto per `show_widget` —, e le righe di verifica del widget le vede solo il proprietario | 📌 *una verifica che l'agente deve leggere si serve nel browser integrato, con `http.server` su una porta sua, e si legge dal DOM; il widget resta per lo sguardo del proprietario* |
| il `grep` sulla frase *«no ADR has taken»* in `main.rs` ha trovato un commento, e sono due | 📌 *nel sorgente un commento va a capo: una frase si cerca per un pezzo corto, o su righe unite* |
| una sonda con la barra verticale scritta dentro una cella dell'errata, e tolta prima del commit | la trappola 4 vale anche per chi scrive l'errata: il comando va sotto la tabella |

**Da verificare alla fonte prima del compito 1:** niente di esterno.
