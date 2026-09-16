"""patch_closure19.py -- the nineteenth closure in the diary of the part-2 plan, plus the recall in the position
table. LF in, LF out; the anchor is asserted before the write and the file is written atomically."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = sys.argv[1] if len(sys.argv) > 1 else r"C:\Users\zagor\Desktop\harness"
PLAN = os.path.join(ROOT, "docs", "superpowers", "plans", "2026-09-11-sottoprogetto-2-parte-2-gui-minima.md")

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw, "the plan is LF"
t = raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


RECALL = """✅ **RICHIAMO DEL 2026-09-16, quarta ripresa del giorno: i compiti 15, 16 e 17 sono rivisti in profondità (R8, un
revisore su Opus 5) e corretti (ondata 17), il compito 7 ha finalmente il proprio criterio di chiusura, e le DUE A/B del
proprietario sono decise — la forma delle P (94 → **B**, P-117…P-133) e «il resto spento» (C13-2 → **A**, il Passo 15-bis
del 14)** — la **diciannovesima chiusura** dice dove si riprende: **resta un revisore solo, R13**, la compilazione dei
moduli riscritti del 12 e del 14, su Opus col costo detto prima e il sì del proprietario; poi l'esecuzione, in una
sessione nuova.

"""
t = sub(t, "| # | Compito | Commit | Stato |\n", RECALL + "| # | Compito | Commit | Stato |\n")

CLOSURE = r"""### La diciannovesima chiusura — 2026-09-16, quarta ripresa del giorno: i compiti 15, 16 e 17 rivisti in profondità (R8) e corretti (ondata 17), il criterio di chiusura del compito 7 scritto, le due A/B del proprietario decise; resta R13; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, sei cose.** **(1)** ⛔ **La revisione in profondità è FINITA.** R8 era l'ultimo revisore di
perimetro: i compiti 3, 8, 13 e 15–17 sono stati rivisti uno per uno su Opus 5, e ogni rilievo confermato è applicato.
**Resta un solo dispaccio, R13** — non una revisione, una **prova di compilazione**: i moduli che la revisione ha
riscritto nel **12** (`gui/fake-core/src/main.rs`, `mod tests`) e nel **14** (`Frame.vue`, `Chat.vue`, `markdown.ts`,
`modules.test.ts`, `chat.test.ts`) non sono mai stati compilati. Il mandato `R13-prompt.md` è committato e **vale così
com'è**. **(2)** ⛔ **Le due A/B sono DECISE e scritte**, e nessuna resta aperta: la **94** è **B** — una P per compito
rivisto, `P-117…P-133`, **generate da uno script dai rapporti** e non scritte a mano, ciascuna col comando che conta i
propri rilievi; **C13-2** è **A** — «spento» è ogni controllo che parlerebbe col core finché il core non ha risposto, e
il **Passo 15-bis** del 14 lo scrive nella §6a. **(3)** `ledger.md` resta la **casa unica** di ciò che resta:
`grep -c ' ⬜' ledger.md` → **1** (il capoverso dello stato, che tiene la storia). **(4)** ⛔ **Il modello compilato del
13 con l'ondata 16** — `C:\Users\zagor\AppData\Local\Temp\probe-R6\gui\` — e la **catena `eslint` installata da R8** —
`<scratchpad 4fa001a2>\review\probe-R8\eslint\` — esistono ancora: R13 **copia** il primo come base invece di rifare
l'11 e il 13 da zero (decisione 95), e chi tocca il compito 15 rimisura sul secondo in minuti. Chi riprende li controlla
con `ls` **prima** di dispacciare. **(5)** ⛔ **Il compito 7 ha ricevuto il proprio criterio di chiusura** (R8-21): era
l'unico dei diciassette senza, e il difetto è emerso dal **17**, che si impegna a riassumerli tutti —
`grep -cE '^(#### |\*\*)Criterio di chiusura' <questo file>` ora rende **17**, quanti sono i compiti. **(6)** I moduli
riscritti del **12** e del **14** restano **non compilati**: è esattamente ciò che R13 va a misurare.

✅ **Che cosa è stato fatto.** Quattro commit, ciascuno col cancello verde prima (i log nello scratchpad di questa
sessione, `…\a65fe077-0cc9-4361-afb0-79b3ca885105\scratchpad`, che può non sopravvivere): `cfb43d8` — il rapporto
`R8-report.md` copiato nel repo, da solo (decisione 90); `8d741ea` — l'**ondata 17**: i compiti 15, 16 e 17 corretti sui
ventiquattro rilievi confermati di R8 (dieci bloccanti: l'analizzatore TypeScript mancante, i sette `no-raw-text` su `:`
e `—`, gli otto siti `/tmp` che Python non risolve, il terzo `<tempo>` che nessun passo misurava, il commento che
attribuisce le chiavi al compito sbagliato, il `grep` di `audit-level` rosso su un file giusto, l'`awk` del ⏭️ che non
può rendere uno, i record congelati contati nove per otto, i criteri di chiusura contati dieci per diciassette), più il
**criterio di chiusura del compito 7** e **D91**; `1d55191` — le **due A/B** del proprietario, con `P-117…P-133`
generate dai rapporti e il Passo 15-bis del 14; e questo, la chiusura. ⚠️ **Le due misure che il piano ora porta sono
state rifatte dal coordinatore** sulla catena `eslint` che R8 aveva installato, nelle due direzioni: col parser
TypeScript i sei `Parsing error` spariscono e restano **sette** `no-raw-text`; con `ignoreText: [":", "—"]`
`npx eslint src` esce **0**, e un template con `{{ a }}: ciao — {{ a }}` resta **rosso** con
`raw text ': ciao —' is used` — la lista silenzia la punteggiatura, non una scritta.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline 5f525c0..HEAD` → **quattro**: `cfb43d8`, `8d741ea`, `1d55191`, e questa chiusura; `git log --oneline 878e9ef..HEAD` → **diciotto** |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **133** (erano 116: le diciassette P della revisione, decisione 94 → B); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **91** (D91 è dell'ondata 17; le decisioni di questa sessione, 98 e 99, stanno qui sotto e non nella tabella D) |
| I criteri di chiusura | `grep -cE '^(#### \|\*\*)Criterio di chiusura' <questo file>` → **17**, uno per compito — il **7** l'ha ricevuto oggi (R8-21) |
| Quanto è corretto | `ledger.md`: ✅ la testa e i compiti 1–17, ✅ il 3, l'8, il 13 e **15, 16, 17** in profondità; `grep -c ' ⬜' ledger.md` → **1** (il capoverso dello stato) |
| I rilievi «fatto» confermati nei rapporti | **169** il 2026-09-16, coi rapporti R1–R12 **e R8**, col comando nel blocco **sotto** questa tabella; R8 ne porta ventitré |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (baseline all'apertura, poi una corsa per commit: quattro in tutto); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> [|] wc -c` → `0`; `ledger.md`, `constraints.md`, `R8-report.md` e i `patch_*.py` LF |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1**; `<data>` e `<tempo>` non sono segnaposto (D75) |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio né la roadmap |
| Documenti fuori dal piano | nella cartella della revisione: `R8-report.md`, `patch_c151617b.py`, `patch_decisions.py`; `ledger.md` con l'ondata 17 e le due decisioni; `constraints.md` col conteggio P aggiornato a **133** |
| File temporanei | nessuno nel repository. Lo scratchpad `…\4fa001a2-…\scratchpad\review\` porta lo scheletro, `p-titles.md`, il lockfile del modello dell'11, `probe-R11/`, `probe-R12/`, `probe-R6/`, **`probe-R8/`** (con la catena `eslint` installata e i tredici `.vue` estratti) e i rapporti; `C:\Users\zagor\AppData\Local\Temp\probe-R6\` è il modello compilato del 13 **con l'ondata 16**; lo scratchpad di questa sessione (`…\a65fe077-…`) porta i quattro log del cancello, `dispatch-R8.md`, i tre messaggi di commit e i tre `patch_*.py` — tutto cancellabile quando R13 ha finito |
| Debito lasciato | **dichiarato per intero in `ledger.md` e qui**: la compilazione dei moduli riscritti del **12** e del **14** (R13), e i moduli stessi, che restano non compilati. ⛔ **Nient'altro:** le due A/B sono decise, le P sono scritte, i sei perimetri sono rivisti |

Il comando dei rilievi «fatto» confermati — una riga per rilievo, la cella del verdetto seguita da quella della specie:

```bash
cat docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/R*-report.md | grep -cE '^\| R[0-9]+[ab]?-[0-9]+ .*\| *CONFERMATO[^|]*\| *fatto *\|'
```

#### Le decisioni prese scrivendo, oltre a quelle del registro

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 98 | le voci **P** di una revisione si **GENERANO da uno script che legge i rapporti**, non si scrivono a mano: ogni P porta il comando che conta i propri rilievi, e il numero che scrive è quello che il comando ha reso in quel momento | è la decisione 94 → B resa eseguibile: diciassette voci scritte a mano sarebbero diciassette occasioni di sbagliare un numero, e il conteggio per compito **cambia** quando arriva un rapporto nuovo — infatti R8 ha portato il 15 da 3 a 12, il 16 da 1 a 8 e il 17 da 2 a 9 fra la prova e l'applicazione, senza che una riga si toccasse a mano | lo script va riletto come si rilegge un testo; una regola di estrazione sbagliata sbaglia **tutte** le P insieme, ed è il motivo per cui il comando sta **dentro** ogni P |
| 99 | quando **due script** toccano lo stesso file, l'ordine si dichiara **dentro il secondo**, accanto all'ancora che il primo cambia | `patch_decisions.py` si ancorava alla riga di stato del registro, che `patch_c151617b.py` riscrive: lanciato prima, il secondo falliva; lanciato dopo, l'ancora è un'altra. Scrivere l'ordine in un diario non basta, perché chi rilancia lo script legge lo script | un fallimento pulito invece di una scrittura sbagliata — l'`assert` regge comunque, ma costa un giro |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **PYTHON NON RISOLVE IL `/tmp` DI GIT BASH, E `cygpath` È LA CURA IN UNA RIGA**: `os.path.abspath('/tmp/x')` rende
  `C:\tmp\x`, che non esiste, mentre bash vede `C:/Users/zagor/AppData/Local/Temp`. La forma che il piano ora detta è
  `export SCRATCH="$(cygpath -w /tmp)"` e `os.path.join(os.environ["SCRATCH"], "<file>")`: il lato bash resta com'è.
- ⛔ **UNA GUARDIA DI FINE SCRIPT CONTATA A MEMORIA FALLISCE, E LE MIE HANNO FALLITO DUE VOLTE SU DODICI** (`Passo
  8-quater` 3→2, `os.environ["SCRATCH"]` 6→7): si contano **sul testo che lo script produce**, rileggendo le
  sostituzioni, prima del lancio. È la stessa trappola della diciottesima chiusura, ricaduta.
- ⛔ **UN'ANCORA `\n**X` DOVE IL FILE HA `\n\n**X` INFILA UNA RIGA VUOTA DENTRO UNA TABELLA**: la riga nuova finisce
  **dopo** la riga vuota, e `awk` sulle tabelle spezzate lo trova. L'ancora comprende la riga vuota.
- ⚠️ **I RAPPORTI NON SONO TUTTI LF**: `R10-report.md` porta **629** CR e `R11-report.md` **212**. Uno script che li
  **legge** normalizza i fine-riga per l'analisi e non li riscrive mai; uno che pretendesse LF si ferma sul primo.
- ⚠️ **LA CARTELLA DI PROVA DEL REVISORE È UN ATTREZZO DEL COORDINATORE**: la catena `eslint` che R8 aveva installato ha
  misurato due rimedi in due minuti — il parser TypeScript e `ignoreText` — che a mano avrebbero voluto un
  `npm install` e i tredici `.vue` estratti di nuovo. Si guarda `ls <scratchpad>/review/probe-RN/` **prima** di rifare.
- ⚠️ **UN RILIEVO PUÒ NON ESSERE DEL COMPITO CHE LO OSPITA**: R8-21 — il compito **7** senza criterio di chiusura — è
  uscito rivedendo il **17**, che si impegna a riassumerli tutti. Il rimedio va dove vive il difetto, non dove è emerso.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, le voci aperte,
   la tabella D a 25 righe per chiamata), poi
   [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md) **per intero**. Di ogni
   rapporto si legge **la riga** del rilievo (`grep -h '^| R8-3 ' R8-report.md`), non il rapporto.
3. ⛔ **Prima del dispaccio**: `ls /c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/4fa001a2-dff2-4d38-9775-1dfa17966e6f/scratchpad/review/`
   e `ls /c/Users/zagor/AppData/Local/Temp/probe-R6/gui/`. Se ci sono, il mandato `R13-prompt.md` vale così com'è e il
   modello del 13 si **copia**; se non ci sono più, si rigenerano coi comandi del punto 3 della sedicesima chiusura e si
   corregge la riga «scratchpad» del richiamo in `constraints.md`, committando prima del dispaccio. ⚠️ Dal `b3422bf` che
   `constraints.md` nomina, i compiti **12 e 14 non sono cambiati**; il 13 è l'ondata 16; P è **133** e D **91**:
   `git diff -U0 b3422bf..HEAD -- <questo file> \| grep '^@@'` lo mostra, e il prompt di dispaccio dice `HEAD` di oggi e
   quanti commit dopo.
4. ⏭️ **R13, L'ULTIMO DISPACCIO, SU OPUS**, annunciato col costo e dispacciato col sì del proprietario: la compilazione
   dei moduli riscritti del **12** e del **14** — stimato come R6 (~400k token, ~40 minuti), perché ricostruisce un
   modello e compila. Il prompt di dispaccio è di **tre righe**, nella forma di `dispatch-R8.md` nello scratchpad di
   questa sessione (se c'è ancora) o del punto 4 della diciottesima chiusura. Il rapporto si copia nel repo e si
   committa **da solo** col cancello verde (decisione 90); poi le correzioni in **una ondata per compito**, con
   `patch_c151617b.py` come modello più recente: la fetta del compito, ogni ancora asserita, le guardie contate sul
   testo prodotto, il registro nello stesso commit, `check_after_write.sh`, il diff letto per intero, il cancello, il
   commit senza co-autore. ⛔ **L'ondata intera si misura sul modello compilato prima di dettarla** (decisione 95).
5. ⛔ **Poi le P di R13**, nella forma della decisione 94 → **B**: si rilancia `patch_decisions.py`? **No** — quello
   script **inserisce** le diciassette P e fallirebbe sulla seconda corsa. Le P di R13 non sono P nuove: R13 ricade sui
   compiti **12** e **14**, che hanno già `P-128` e `P-130`, e i loro **comandi** contano da sé i rilievi nuovi appena
   il rapporto è nel repo. ⚠️ **Si rilancia il comando di ciascuna e si riscrive il numero**, con la data.
6. ⛔ **Dopo ogni scrittura su questo file**: `check_after_write.sh`, il diff letto, `bash scripts/gate.sh` →
   `GATE GREEN`, e il commit — **senza co-autore**. Per ripartire da `HEAD`:
   `python <revisione>/restore_from_head.py <piano> <revisione>/ledger.md`.
7. ⏭️ **Poi l'esecuzione, in una sessione NUOVA**: `superpowers:subagent-driven-development`, un subagente fresco per
   compito **su Opus**, con revisione fra uno e l'altro. ⛔ **Prima di eseguire il compito 11 si aggiorna Node** (P-64,
   P-65: oggi `v24.9.0`, fuori da `^24.15.0`); `cargo-audit` 0.22.2 c'è. Alla chiusura del piano la cartella della
   revisione si **archivia** (decisione 69).
8. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

"""
t = sub(t, "### La diciottesima chiusura — 2026-09-16, terza ripresa del giorno:",
        CLOSURE + "### La diciottesima chiusura — 2026-09-16, terza ripresa del giorno:")

assert t.count("### La diciannovesima chiusura") == 1
assert t.count("RICHIAMO DEL 2026-09-16, quarta ripresa del giorno") == 1

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: the nineteenth closure and the recall in the position table;", t.count("\n") - raw.count("\n"), "lines added")
