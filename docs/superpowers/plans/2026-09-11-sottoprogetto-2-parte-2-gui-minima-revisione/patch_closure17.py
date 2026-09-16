"""patch_closure17.py -- the seventeenth closure in the plan's diary, and the recall line in the position section.
Two anchors, asserted; atomic write. Run it in the SAME step that writes it (decision 88).
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


POSITION_OLD = r'''dell'8 di R9b-5, poi le P nuove. ⛔ **I subagenti si dispacciano con `model: "opus"`, mai col modello ereditato.**

| # | Compito | Commit | Stato |
'''
POSITION_NEW = r'''dell'8 di R9b-5, poi le P nuove. ⛔ **I subagenti si dispacciano con `model: "opus"`, mai col modello ereditato.**
✅ **RICHIAMO DEL 2026-09-16, seconda ripresa del giorno: il compito 8 è rivisto in profondità (R12, un revisore su Opus)
e corretto (ondata 15), e la metà dell'8 di R9b-5 è chiusa** — la **diciassettesima chiusura** dice dove si riprende: la
revisione in profondità di 13, 15, 16, 17 e la compilazione dei moduli del 12 e del 14 (R6, R8, R13: **un revisore alla
volta su Opus, col costo detto prima e il sì del proprietario**), poi le P nuove — ⚠️ **la cui forma è una decisione del
proprietario, in A/B, prima di scriverne una** (decisione 94).

| # | Compito | Commit | Stato |
'''

CLOSURE = r'''### La diciassettesima chiusura — 2026-09-16, seconda ripresa del giorno: il compito 8 rivisto in profondità (R12) e corretto (ondata 15), la metà dell'8 di R9b-5 chiusa; restano 13, 15, 16, 17 e la compilazione del 12 e del 14, poi le P nuove; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, cinque cose.** **(1)** ⛔ **Ogni `Agent` passa `model: "opus"` (o `sonnet` per lavoro meccanico),
UN revisore alla volta, e prima del dispaccio si dice il costo e si aspetta il sì** (decisione 86): questa sessione l'ha
fatto — R12 su Opus 5, ~297k token, 93 chiamate del tool, 98 comandi, ~29 minuti, per un compito da 899 righe con il
modello Rust compilato — e il proprietario ha chiuso la sessione **prima** del revisore successivo: *«si continua nella
prossima sessione fai session-handoff»*. **(2)** `ledger.md` resta la **casa unica** di ciò che resta: ✅ la testa e i
compiti 1–17 nelle righe note, ✅ il 3 (R11) e l'8 (R12) rivisti in profondità e corretti, **nessuna riga ⬜** —
`grep -c ' ⬜' ledger.md` → **1**, ed è il capoverso dello stato, che tiene la storia. **(3)** I mandati per i revisori che
restano sono **scritti e committati** nella cartella della revisione — `R6-prompt.md` (13), `R8-prompt.md` (15–17),
`R13-prompt.md` (la compilazione dei moduli riscritti del 12 e del 14) — coi richiami del 2026-09-16 in coda, e
`constraints.md` porta in testa il richiamo che vince. ⚠️ Quel richiamo nomina lo scratchpad
`…\4fa001a2-dff2-4d38-9775-1dfa17966e6f\scratchpad`, che a questa chiusura esiste ancora su disco con `review/skeleton.md`,
`review/p-titles.md`, `review/model11-lock/`, `review/probe-R11/` e `review/probe-R12/`: chi riprende lo controlla con `ls`
**prima** di dispacciare, e se non c'è più lo rigenera coi comandi del punto 3 della sedicesima chiusura e corregge la
riga «scratchpad» del richiamo. ⛔ **E le cartelle `probe-R6/`, `probe-R8/`, `probe-R13/` che stanno lì sono dei revisori
Fable UCCISI la mattina del 16**: si spostano da parte prima di dispacciare lo stesso mandato (decisione 93), come è stato
fatto per `probe-R12-killed-fable-0835`. **(4)** Il modello compilato di R12 — `review/probe-R12/ws/` con il target di cargo
in `C:\Users\zagor\AppData\Local\Temp\probe-R12-target` — porta il compito 6 modellato a monte, il compito 8 coi rossi di
R12 riparati, la prima sonda del Passo 4 **nella forma di R12-1** e l'ottavo braccio a `{}`: è riutilizzabile per una
rimisura in un minuto (decisione 92), e cancellabile quando i revisori hanno finito. I moduli riscritti e **non compilati**
del 12, del 13 e del 14 restano tali (R13 e R6 li compilano). **(5)** ⛔ **Nessuna P nuova si scrive prima dell'A/B al
proprietario** (decisione 94, il punto 6 qui sotto): i rilievi «fatto» confermati oggi nei rapporti sono quanti dice il
comando nella tabella, e una P ciascuno sarebbe una terza casa per fatti che vivono già nei rapporti e nel registro.

✅ **Che cosa è stato fatto.** Tre commit, ciascuno col cancello verde prima (i log nello scratchpad di questa sessione,
`…\ea2cab1f-ff42-46d8-b96e-ec4f9f7a9d7b\scratchpad`, che può non sopravvivere): `7082cc6` — il rapporto `R12-report.md`
copiato nel repo, da solo (decisione 90); `2ba0fcb` — l'ondata 15: il compito 8 corretto sui tredici rilievi confermati di
R12 (otto bloccanti: la sonda del Passo 4 che panicava su `OutOfOrder`, la misura falsa del braccio `enter`, `VramPolicy`
senza `derive`, i sette `E0433` dei percorsi mai importati, l'`use` di `arbiter/mod.rs` senza `PolicyDetail`, il quinto
sito esaustivo `match detail`, gli `use` dei due banchi, il *Trova* del Passo 3 che non esisteva nel file) e sui cinque
non bloccanti, **più** la metà dell'8 di R9b-5 col testo esatto che R12 propone, il Passo 12 riscritto nella forma del
Passo 15 del 14 (uno script per i due disegni, quattro righe) e il richiamo in P-45; e questo, la chiusura. ⚠️ **Due misure
che il piano ora porta le ha rifatte il coordinatore**, non copiate dal rapporto: le risposte scartate del Passo 3
(`enter`, `leave`) sul modello di R12 con la sonda nella forma nuova (decisione 92), e lo script del Passo 12 eseguito su
**copie** dei due disegni con la data al posto di `<data>` — 1, 1, 2, CR 0, tabelle intatte — prima di dettarlo. ⚠️ **Otto
righe NON RIPRODOTTO di R12 (R12-14…R12-21)** sono nel rapporto perché il coordinatore non le ricerchi: gli indici 7 e 4
liberi, gli offset dell'ottavo record esatti al byte, la mutazione del Passo 7 che rende rossa la sola quarta sonda, il
blocco *Interfaces* giusto, le righe già applicate del registro che reggono (R12-20).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline 95ecc29..HEAD` → **tre**: `7082cc6`, `2ba0fcb`, e questa chiusura; `git log --oneline 878e9ef..HEAD` → **undici** |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **116** (invariato: le P della revisione aspettano la decisione 94); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **89** (invariato: le decisioni di questa sessione, 91–94, stanno qui sotto e non nella tabella D) |
| Quanto è corretto | `ledger.md`: ✅ la testa e i compiti 1–17, ✅ il 3 e l'8 in profondità; `grep -c ' ⬜' ledger.md` → **1** (il capoverso dello stato) |
| I rilievi «fatto» confermati nei rapporti | **130** il 2026-09-16, coi rapporti R1–R12 (R6 e R8 mancano), col comando nel blocco **sotto** questa tabella: è il numero che la decisione 94 mette davanti al proprietario. ⚠️ Un `awk -F'\|'` sulle stesse tabelle ne conta **meno** (91), perché le celle che contengono una barra escapata gli spostano le colonne: si conta con un `grep -cE` sulla sequenza «verdetto, specie», mai per colonna |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (baseline all'apertura, poi una corsa per commit: quattro in tutto); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> [|] wc -c` → `0`; `ledger.md`, `R12-report.md`, i `patch_*.py` LF |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1**; `<data>` e `<tempo>` non sono segnaposto (D75) |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio né la roadmap |
| Documenti fuori dal piano | nella cartella della revisione: `R12-report.md`, `patch_c8b.py`, `patch_closure17.py`; `ledger.md` con l'ondata 15 |
| File temporanei | nessuno nel repository. Lo scratchpad `…\4fa001a2-…\scratchpad\review\` porta lo scheletro, `p-titles.md`, il lockfile del modello dell'11, `probe-R11/`, `probe-R12/` (col modello compilato, decisione 92) e le tre cartelle dei revisori uccisi; `C:\Users\zagor\AppData\Local\Temp\probe-R12-target` è il target di cargo del modello; lo scratchpad di questa sessione (`…\ea2cab1f-…`) porta i quattro log del cancello, `dispatch-R12.md`, gli script della rimisura e le copie di prova del Passo 12 — tutto cancellabile quando i revisori hanno finito |
| Debito lasciato | **dichiarato per intero in `ledger.md`** («Mancano») e qui: la revisione in profondità di 13, 15, 16, 17 (R6, R8); la compilazione dei moduli riscritti del 12 e del 14 (R13); le P-117… **dopo** la decisione 94; i moduli riscritti e non compilati del 12, 13 e 14 |

Il comando dei rilievi «fatto» confermati — una riga per rilievo, la cella del verdetto seguita da quella della specie:

```bash
cat docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/R*-report.md | grep -cE '^\| R[0-9]+[ab]?-[0-9]+ .*\| *CONFERMATO[^|]*\| *fatto *\|'
```

#### Le decisioni prese scrivendo, oltre a quelle del registro

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 91 | nei banchi `steps_in_doubt` e `policy_now` si chiamano **nude**, e `policy_now` entra nell'`use kernel::arbiter::{…}` — non `use kernel::arbiter;` col percorso, la via che R12-4 proponeva per prima | coerenza col file (criterio 2): le sonde gemelle di `reconciliation.rs` chiamano `steps_in_doubt` nuda, e in `arbiter_policy.rs` `arbiter` è già il nome di una funzione del banco | sei righe di codice dettato cambiate al posto di due `use`; il modello di R12 compila in entrambe le forme |
| 92 | una misura che un rilievo dichiara **da rifare** («rimisurarlo dopo la cura, perché la coppia cambia») la rifà il **coordinatore** sul modello compilato del revisore, prima di scriverla nel piano: `probe-R12/ws/` col target in `probe-R12-target`, la sonda portata alla forma nuova con uno script nello scratchpad, tre corse (`{}`, `enter`, `leave`) e la revoca | un commento che il commit scriverebbe nel kernel non si detta su una previsione (gotcha #57), e il modello c'era già; costo: un minuto di compilazione incrementale per corsa | il modello di un revisore può divergere dal piano corretto: si legge il `git diff` del modello, o si ricopia |
| 93 | la cartella di prova di un revisore **morto** si sposta da parte (`probe-RN-killed-<modello>-<ora>`) prima di ridispacciare lo stesso mandato, così il revisore nuovo la crea da zero come il mandato dice | gli estratti di un revisore morto sono di un'altra lettura del piano, e un revisore che trova file «suoi» può riusarli senza rimisurare | niente |
| 94 | la forma delle **P-117…** è una **decisione del proprietario, in A/B, prima che una sola P nuova si scriva**: **(A)** una P per rilievo «fatto» confermato — centotrenta il 2026-09-16 (il comando sotto la tabella dello stato), raggruppate per compito, col comando che le ha misurate — oppure **(B)** una P per compito rivisto, corta, che rimanda alla sezione del rapporto e a quella del registro, col comando che le conta. Il consiglio del coordinatore è **B** | i rilievi vivono già in due case committate — i rapporti (`R*-report.md`, col comando e la resa) e il registro (rimedio e stato): centotrenta P sarebbero una terza casa da tenere allineata (gotcha #68), e la regola «una per rilievo» è del coordinatore della dodicesima chiusura, non del proprietario. La sedicesima chiusura la dava per ordine, e non lo era | un giro di domanda prima del passo 6; se il proprietario sceglie A, si generano da uno script che legge le righe dei rapporti, non a mano |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **PATCHARE UNO SCRIPT PYTHON DA UN HEREDOC DEL TOOL BASH MANGIA I BACKSLASH ANCHE SU UNA RIGA SOLA**: `"\\n"` è
  diventato un a-capo vero dentro una stringa, `SyntaxError` — la memoria lo diceva già dal 2026-09-11, ed è scattata di
  nuovo. Uno script si scrive con **Write** e si corregge con **Edit**, mai da heredoc; nulla è stato scritto nel piano,
  perché lo script scrive solo alla fine.
- ⚠️ **UNA GUARDIA DI FINE SCRIPT CHE CONTA LE OCCORRENZE DI «⬜» SCATTA SUL CAPOVERSO DELLO STATO**, che tiene la storia
  («tutto il resto ⬜», «resta ⬜ …») per costruzione: la guardia conta le **righe**, come il comando `grep -c` della
  tabella dello stato, e asserisce il numero atteso — non 0 e non 1 a caso.
- ⚠️ **IL PROPRIETARIO PUÒ RISPONDERE A UNA DOMANDA A OPZIONI CON UN TESTO LIBERO** («si continua nella prossima sessione
  fai session-handoff»): si leggono le parole, non l'opzione più vicina, e la chiusura parte subito.
- ⚠️ **UN CRITERIO A TRE `grep -c` PER QUATTRO RIGHE TOCCATE NON È UN ERRORE**: il richiamo sulla decisione 56 dice «dal
  pre-controllo del compito 8» e lo conta il primo `grep`; gli altri due contano la riga delle *Registrate* e le due del
  disegno del 2 (→ 2). Chi rilegge il Passo 12 non aggiunga un quarto `grep` per zelo.
- ⚠️ **LO SCRIPT DETTATO DA UN PASSO SI PROVA SU COPIE, NON NEL REPO**: `awk` dal piano al file, `sed 's/<data>/…/'`, `cp`
  dei due disegni in una cartella dello scratchpad con gli stessi percorsi relativi, `cd` lì e via — poi `git status
  --porcelain` prova che il repo non l'ha visto.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, le voci aperte; la
   tabella D a 25 righe per chiamata o a riga singola), poi
   [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md) **per intero**. Di ogni
   rapporto si legge **la riga** del rilievo (`grep -h '^| R12-6 ' R12-report.md`), non il rapporto.
3. ⛔ **Prima di qualunque dispaccio**: `ls /c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/4fa001a2-dff2-4d38-9775-1dfa17966e6f/scratchpad/review/`.
   Se c'è (`skeleton.md`, `p-titles.md`, `model11-lock/`), i mandati valgono così come sono e si spostano da parte
   `probe-R6/`, `probe-R8/`, `probe-R13/` (decisione 93); se non c'è più, si rigenera coi comandi del punto 3 della
   sedicesima chiusura e si corregge la riga «scratchpad» del richiamo in `constraints.md`, committando prima del dispaccio.
   ⚠️ I compiti 12–17 **non sono cambiati** da `b3422bf`, il commit che `constraints.md` nomina: le ondate 14 e 15 hanno
   toccato il 3, l'8, P-35 e P-45, e le chiusure la testa e il diario (`git diff -U0 b3422bf..HEAD -- <questo file> \| grep '^@@'` lo mostra).
4. ⏭️ **LA REVISIONE IN PROFONDITÀ CHE RESTA, UN REVISORE ALLA VOLTA SU OPUS**, ciascuno annunciato col costo e dispacciato
   col sì del proprietario, nell'ordine **R6** (13: il compito più grande mai riletto, ~1 975 righe più cinque moduli da
   compilare sotto `jsdom` col modello dell'11 — stimato il doppio di R12), **R8** (15–17: ~1 500 righe, niente da
   compilare — stimato come R12), **R13** (la compilazione dei moduli del 12 e del 14). Il riferimento di costo è R12:
   ~297k token, ~29 minuti. Il prompt di dispaccio è di **tre righe** — chi sei e i due file da leggere (`constraints.md`
   col richiamo che vince, poi `RN-prompt.md`), niente lettura d'apertura; lo scratchpad `4fa001a2` per esteso, la cartella
   di prova `review/probe-RN/` da creare, il rapporto in `review/RN-report.md` a pezzi con la data nel titolo; `HEAD` di
   oggi e quanti commit dopo `b3422bf`, che il perimetro non è cambiato, e che `git status --porcelain` deve essere vuoto
   alla fine e riportato — il modello è `dispatch-R12.md` nello scratchpad di questa sessione, se c'è ancora. Il rapporto
   si copia nel repo e si committa col cancello verde (decisione 90); poi le correzioni in **una ondata per compito** —
   `patch_c8b.py` è il modello più recente: la fetta del compito, ogni ancora asserita, le guardie col numero atteso e per
   righe, il registro nello stesso commit, `check_after_write.sh`, il diff letto per intero, il cancello, il commit senza
   co-autore. Una misura che il rilievo dice «da rifare» si rifà sul modello del revisore prima di dettarla (decisione 92).
5. ⛔ **Prima delle P-117…, l'A/B della decisione 94 al proprietario**, in due righe e col consiglio (B): finché non
   risponde, `grep -c '^### P-'` resta **116**.
6. Poi le voci **P**, nella forma scelta; i conteggi `P` e `D` si confrontano col valore di `HEAD` dopo **ogni** scrittura.
7. ⛔ **Dopo ogni scrittura su questo file**: `check_after_write.sh`, il diff letto, `bash scripts/gate.sh` → `GATE GREEN`, e il
   commit — **senza co-autore**. Per ripartire da `HEAD`: `python <revisione>/restore_from_head.py <piano> <revisione>/ledger.md`.
8. ⏭️ **Poi l'esecuzione, in una sessione NUOVA**: `superpowers:subagent-driven-development`, un subagente fresco per
   compito **su Opus**, con revisione fra uno e l'altro. ⛔ **Prima di eseguire il compito 11 si aggiorna Node** (P-64, P-65:
   oggi `v24.9.0`, fuori da `^24.15.0`); `cargo-audit` 0.22.2 c'è. Alla chiusura del piano la cartella della revisione si
   **archivia** (decisione 69).
9. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

'''

t = sub(t, POSITION_OLD, POSITION_NEW)
t = sub(t, "## Come si riprende — il diario di questo piano, coi comandi\n\n### La sedicesima chiusura",
        "## Come si riprende — il diario di questo piano, coi comandi\n\n" + CLOSURE + "### La sedicesima chiusura")

# the ledger: the heading of the P-117 section gets the recall of decision 94 (the form is the owner's, in A/B)
LEDGER = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione\ledger.md"
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = sub(lraw,
        "raggruppate per compito; i «prosa» stanno nella tabella della dodicesima chiusura",
        "raggruppate per compito; i «prosa» stanno nella tabella della dodicesima chiusura. ⚠️ **RICHIAMO DEL 2026-09-16, "
        "decisione 94 della diciassettesima chiusura del piano:** la forma — una P per rilievo, o una per compito che rimanda "
        "al rapporto e a questo registro — è del **proprietario, in A/B**, prima che una P nuova si scriva; i rilievi «fatto» "
        "confermati si contano col `grep -cE` scritto in quella chiusura (centotrenta il 2026-09-16, R6 e R8 mancanti), non "
        "con un `awk` per colonna, che sbaglia sulle celle con una barra escapata")

for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: seventeenth closure written, and the ledger recalled;", t.count("\n") - raw.count("\n"), "plan lines added")
