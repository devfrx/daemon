# Knowledge base, la revisione — i documenti: il piano

> ⛔ **IN SCRITTURA, A METÀ — NON SI ESEGUE.** Il 2026-09-29 la sessione che lo scriveva si è fermata su richiesta del
> proprietario — *«si continua nella prossima sessione»* — col **materiale verificato** e senza la **prosa dei compiti**.
> Questo file porta il materiale parola per parola, e la consegna: *«Come si riprende»*, qui sotto. Il piano si completa
> nella prossima sessione; il pre-controllo e l'esecuzione vengono dopo, ciascuno in una sessione sua (`CLAUDE.md`,
> *«Una fase per sessione»*). Il disegno che il piano traduce è il
> [disegno della revisione della knowledge base](../specs/2026-09-28-knowledge-base-revisione-design.md), chiuso e riletto
> dal proprietario il 2026-09-29.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-29, coi comandi

⛔ **Da sapere subito: niente è a metà nel repository.** Nessun file è stato toccato oltre a questo, al puntatore della §6
del compendio con la sua riga della data, e all'archivio che li tiene com'erano; nessun codice. A metà è **il piano**: il
materiale sotto è verificato, la prosa che lo avvolge manca. ⚠️ **Il materiale non si esegue così com'è:** i blocchi sono
stati controllati uno per uno sul repository di `0c0d0d4`, **non in sequenza**, e nessun compito è stato pre-controllato.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb`; nessuno stash |
| codice di prodotto | **non toccato**: `git diff --stat 0c0d0d4..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura di questa sessione, e prima del commit di questa chiusura; `bash scripts/check-docs.sh` → `OK`. Si rilanciano, non si citano |
| il margine del compendio | il comando C della 6.2 del disegno — il tetto, meno i byte senza CR, meno le righe; rendeva 8 538 su `0c0d0d4` |
| i blocchi | ciascuno si ricontrolla col suo `--check`, dalla radice del repository — il comando sotto questa tabella |

```bash
for n in 1 2 3 4 5 6; do printf 'E%s: ' $n; PYTHONIOENCODING=utf-8 python <scratchpad>/apply_edits.py --check 2026-09-30 <scratchpad>/e$n.txt 2>&1 | tail -1; done
```

Gli attrezzi e i blocchi si estraggono da questo file nello scratchpad della sessione che riprende — l'aiutante
`apply_edits.py`, e i blocchi come `e1.txt` … `e6.txt` —, ciascuno dal suo recinto, parola per parola. Su `0c0d0d4` i
blocchi **1–5** rendevano `checked` — 17, 20, 59, 24 e 31 modifiche —; il **6** rifiuta due ancore, **per costruzione**:
la riga di questo piano nella tabella dei piani di `roadmap.md` e la spunta del punto 3 della 6.6 del disegno, che non
esistono finché questo piano non è scritto — la voce 5 del compito qui sotto.

**Il compito della sessione che riprende — finire di scrivere il piano, con `superpowers:writing-plans`:**

1. La lettura d'apertura di `CLAUDE.md`; poi il disegno della revisione **per intero**, a blocchi; poi **questo file per
   intero**. Il precedente della forma è il [piano dei documenti del 2026-09-04](2026-09-04-knowledge-base-documenti.md),
   fino al compito 1 compreso; le convenzioni più recenti — il modello `opus`, il dispaccio che viaggia con git, i vincoli
   globali — stanno nella testa del [piano del design system](2026-09-23-design-system.md).
2. `bash scripts/gate.sh`, da solo; il comando C; il `--check` dei sei blocchi.
3. **La testa del piano**, sopra questa sezione, sulla forma del precedente: *Per chi esegue*, obiettivo, forma, strumenti
   — i tre attrezzi di sotto, al posto di `replace_unique.py` —, i vincoli globali, la tabella della posizione coi sei
   compiti di D1, *«Come si esegue un compito»* col punto sul dispaccio che viaggia con git (D17), l'errata vuota, le
   voci P e D qui sotto portate in *«Ciò che la scrittura del piano ha trovato»* e *«Le decisioni prese scrivendo il
   piano»*, la mappa dei file, le voci aperte che il piano sa, e la **Definizione di «fatto»** copiata dalla 6.2 del
   disegno, coi comandi A–E e le trappole della 6.3, più le righe che P-2 e P-3 aggiungono.
4. **La prosa dei sei compiti**, uno per volta: *Files* e *Read*; il Passo 1 con le misure prima; il Passo che applica il
   blocco — `--check`, poi senza — e, nel compito 1, il Passo che scrive ADR-0040 dal recinto e lancia `archive_head.py`
   **prima** del blocco; le prove, coi comandi A–E e i fine-riga; il commit — `knowledge-base-revisione(compito N): …`,
   senza co-autore — e il push; il criterio di chiusura. Il compito 6 vuole anche il **testo del puntatore nuovo** della
   §6, che non è scritto: dirà che la revisione è chiusa, e che vengono il 13 — sbarrato da AUD-004, col perimetro della
   4.2 del disegno — e il brainstorming dei modelli decisionali, **senza** collocarli l'uno rispetto all'altro, perché il
   proprietario non l'ha fatto.
5. **La chiusura della sessione di scrittura**: la riga di questo piano in coda alla tabella dei piani di `roadmap.md`,
   con la cella *«⏳ **scritto il 2026-09-29**; il pre-controllo e l'esecuzione, in sessioni loro»* che il blocco 6 cerca —
   o il blocco si riallinea alla cella scritta —, e l'*«Ultimo aggiornamento»*; nel disegno, il punto 3 della 6.6 riscritto
   col suo inizio *«3. ✅ il **piano dei documenti** — **scritto il 2026-09-29**, in una sessione sua, al suo»*, idem; il
   puntatore della §6 del compendio mosso al pre-controllo, con `archive_head.py --pointer` prima e `replace_pointer.py`
   dopo; questa sezione riscritta come consegna al pre-controllo.
6. ⛔ **Prima del commit del piano, la simulazione in sequenza**: un `git worktree` nello scratchpad sul commit del piano,
   i sei compiti applicati in ordine — ADR-0040 scritto, `archive_head.py`, i blocchi — e `bash scripts/check-docs.sh`
   lì dentro, che deve rendere `OK`; poi il worktree si toglie. È la prova che il `--check` a uno a uno non dà: che i blocchi
   reggano **uno dopo l'altro**, e che il cancello sia verde dopo il compito 1, che è quello che lo tocca.

**Ciò che la scrittura del piano ha trovato** — ciascuna misurata su `0c0d0d4`; si portano nella testa:

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

**Le decisioni prese scrivendo, col perché** — sono del coordinatore, e il proprietario può ribaltarle:

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

**Vicoli ciechi di questa sessione:**

| | Che cosa insegna |
|---|---|
| un rattoppo di `apply_edits.py` passato in un heredoc di Bash con dentro `\r\n` si è fermato su un'asserzione, senza scrivere niente | 📌 *Un file di attrezzi si riscrive per intero con lo strumento di scrittura, mai rattoppato da un heredoc: i backslash non sopravvivono al canale* — la memoria lo diceva |
| la prima stesura del blocco di design/10 ancorava a una recinzione `mermaid`, e quel file ne ha **due** | 📌 *Un'ancora sulla prima riga di un blocco ricorrente non è unica: si ancora all'intestazione della sezione* |
| `rev` non esiste nel Git Bash di questa macchina | le code di riga si stampano con Python |

**Le voci aperte che il piano sa, e non chiude:** AUD-004, che sbarra il 13 e non il piano; X-2 e X-4 dell'audit; le voci
della 4.6 e della 6.5 del disegno, ciascuna col suo chiusore; e P-5, nuova, col proprietario come chiusore.

---

## Il materiale verificato

Parola per parola com'era nello scratchpad alla chiusura, e controllato su `0c0d0d4` come dice la consegna. ⛔ **Chi
riprende lo estrae, non lo riscrive:** ogni recinto è un file.

### Gli attrezzi

#### `apply_edits.py` — applica un blocco di modifiche, tutto o niente; col `--check` non scrive

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

#### `archive_head.py` — archivia parola per parola la riga della data del compendio e, con `--pointer`, il puntatore della §6

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

#### `replace_pointer.py` — riscrive il puntatore della §6, dopo `archive_head.py`

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

### Il testo di ADR-0040 — `<data>` è il giorno del compito 1

````markdown
# ADR-0040: Dove vivono i dati, e che cosa salva il programma

- **Status:** Accepted
- **Date:** <data>
- **Deciders:** proprietario del progetto
- **Modifica:** [ADR-0022](0022-layout-dei-dati-per-natura-e-backup-dichiarato.md), in tre punti — la riga
  «artefatti» delle due tabelle, le **guide** della riga «configurazione, guide, profili», e la conseguenza *«La base
  di conoscenza sopravvive alla reinstallazione perché i documenti sorgente e la configurazione sono nel backup»*. Le
  altre righe reggono, e lo stato di ADR-0022 resta `Accepted`

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

### I blocchi dei sei compiti

#### E1

````text
# compito 1 -- ADR-0040 e cio' che il cancello pretende con lui (il file dell'ADR lo scrive il Passo 2, non questo blocco)
@@ docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
^^ ## Context
<<<
> ⚠️ **Rimando del <data> — modificato da ADR-0040, in tre punti.**
> [ADR-0040](0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md), dal
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) —
> risposte D12 e D17, approvate dal proprietario il 2026-09-29 —, cambia **tre** punti di questo ADR: la riga
> **«artefatti»** delle due tabelle; le **guide** della riga «configurazione, guide, profili»; e la conseguenza *«La base
> di conoscenza sopravvive alla reinstallazione perché i documenti sorgente e la configurazione sono nel backup»*. I file
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

#### E2

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

#### E3

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

#### E4

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

#### E5

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

#### E6

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
