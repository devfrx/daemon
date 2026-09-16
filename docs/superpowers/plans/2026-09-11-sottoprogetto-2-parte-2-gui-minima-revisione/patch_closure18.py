"""patch_closure18.py -- the eighteenth closure in the plan's diary, and the recall line in the position section.
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


POSITION_OLD = r'''proprietario, in A/B, prima di scriverne una** (decisione 94).

| # | Compito | Commit | Stato |
'''
POSITION_NEW = r'''proprietario, in A/B, prima di scriverne una** (decisione 94).
✅ **RICHIAMO DEL 2026-09-16, terza ripresa del giorno: il compito 13 è rivisto in profondità (R6, un revisore su Opus)
e corretto (ondata 16), e i suoi moduli riscritti sono COMPILATI** — la **diciottesima chiusura** dice dove si riprende: la
revisione in profondità di 15, 16, 17 (R8) e la compilazione dei moduli del 12 e del 14 (R13), **un revisore alla volta su
Opus, col costo detto prima e il sì del proprietario**; poi **due A/B** al proprietario, una per volta — la forma delle P
nuove (decisione 94) e «il resto spento» della §6a (la riga nuova nelle voci aperte) — poi le P nuove.

| # | Compito | Commit | Stato |
'''

CLOSURE = r'''### La diciottesima chiusura — 2026-09-16, terza ripresa del giorno: il compito 13 rivisto in profondità (R6) e corretto (ondata 16), i suoi moduli compilati; restano 15, 16, 17 (R8) e la compilazione del 12 e del 14 (R13), poi due A/B e le P nuove; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, sei cose.** **(1)** ⛔ **Ogni `Agent` passa `model: "opus"`, UN revisore alla volta, il costo detto prima
e il sì atteso** (decisione 86): questa sessione l'ha fatto — R6 su Opus 5, **~401k token, 125 chiamate del tool, ~40 minuti**,
per il compito 13 (1975 righe) coi cinque moduli riscritti compilati sotto `jsdom` sopra il modello dell'11; la stima «il
doppio di R12» ha retto per difetto (R12: ~297k, ~29 minuti). Il proprietario ha detto sì con la stima davanti, e ha scelto
**un revisore per sessione**. **(2)** `ledger.md` resta la **casa unica** di ciò che resta: ✅ la testa e i compiti 1–17 nelle
righe note, ✅ il 3 (R11), l'8 (R12) e il 13 (R6) rivisti in profondità e corretti, `grep -c ' ⬜' ledger.md` → **1** (il
capoverso dello stato, che tiene la storia). **(3)** I mandati che restano — `R8-prompt.md` (15–17) e `R13-prompt.md` (la
compilazione dei moduli del 12 e del 14) — sono committati e **valgono così come sono**: entrambi estraggono i compiti dal piano
di **oggi**, e R13 ricostruisce da sé il 13 corretto sotto il 14; `constraints.md` porta in testa il richiamo che vince e nomina
lo scratchpad `…\4fa001a2-dff2-4d38-9775-1dfa17966e6f\scratchpad`, che a questa chiusura esiste con `review/skeleton.md`,
`review/p-titles.md`, `review/model11-lock/`, `review/probe-R11/`, `review/probe-R12/`, `review/probe-R6/` (gli estratti di R6)
e `review/R6-report.md`, più le quattro cartelle dei revisori Fable uccisi, **già spostate da parte** (decisione 93:
`probe-R6-killed-fable-0838`, `probe-R8-killed-fable-0837`, `probe-R13-killed-fable-0840`, `probe-R12-killed-fable-0835`): chi
riprende lo controlla con `ls` **prima** di dispacciare, e se non c'è più lo rigenera coi comandi del punto 3 della sedicesima
chiusura e corregge la riga «scratchpad» del richiamo. **(4)** ⛔ **Il modello compilato di R6 con l'ondata 16 APPLICATA** —
`C:\Users\zagor\AppData\Local\Temp\probe-R6\gui\` (percorso corto: lo scratchpad supera MAX_PATH per `npm` e `vite`), coi
`node_modules` alle versioni del Passo 2 più `@types/node` — è il 13 **com'è nel piano di oggi**: R13 può **copiarlo** come base
per il 14 invece di rifare l'11 e il 13 da zero (decisione 95), e la stessa cartella porta gli script del revisore
(`build_model.py`, `mutate.py`, `views_check.py`). **(5)** ⛔ **DUE A/B aspettano il proprietario**, una domanda per volta: la forma
delle P-117… (decisione 94, consiglio **B**) e «il resto spento» della §6a (la riga nuova nella tabella delle voci aperte,
consiglio **A**). Nessuna P nuova prima della prima: `grep -c '^### P-'` resta **116**. **(6)** I moduli riscritti del **12** e del
**14** restano **non compilati** (R13 li compila); quelli del 13 sono compilati da oggi, e il registro lo dice al posto del
«Non compilato» di prima.

✅ **Che cosa è stato fatto.** Tre commit, ciascuno col cancello verde prima (i log nello scratchpad di questa sessione,
`…\e1e94bbc-a370-4afd-b241-48d457b13ded\scratchpad`, che può non sopravvivere): `384ba60` — il rapporto `R6-report.md` copiato nel
repo, da solo (decisione 90); `708166e` — l'ondata 16: il compito 13 corretto sui diciassette rilievi confermati di R6 (dieci
bloccanti: `@types/node` e la riga `types` del `tsconfig`, la pinia del generatore, la pagina bianca del `.map` — **D90** —, le
diciotto chiavi `modules.*`, il `require('vitest')`, il `git checkout --` sul file non tracciato, la finta di `ResizeObserver`, e
tre commenti nel sorgente che dicevano il falso) e sui sette non bloccanti, più due voci del coordinatore dalla tabella di
copertura di R6 (C13-2, «il resto spento»), il richiamo in P-97, i riallineamenti nel 14 (R6-11) e nel 15 (R6-4), D90 e la riga
nelle voci aperte; e questo, la chiusura. ⚠️ **L'ondata intera è stata MISURATA prima di essere dettata** (decisione 95):
applicata al modello compilato di R6 — `vue-tsc` zero errori, `vitest` 23 verdi e 1 saltata, il generatore verde con tre viste
senza `"missing"`, `vite build` col timbro decimale nel bundle e senza la mappa, `npm run dev` che serve la pagina con zero
errori in console e il timbro in `/@vite/env`, il cassetto aperto col velo (`position: fixed`, `z-index` 100 e 101) e i diciotto
nomi italiani — e le due sonde nuove uccise ciascuna dalla propria mutazione. ⚠️ **Nove righe NON RIPRODOTTO di R6
(R6-18…R6-26)** stanno nel rapporto perché il coordinatore non le ricerchi: D80, D81 e D89 reggono con cinque mutazioni, le sette
versioni del Passo 2 sono la `latest` di oggi, P-80, i sette `Dialog*`, l'ancoraggio del timbro, i diciotto tipi contro la §1, le
opzioni di `createDockview` contro lo spike, la SPA vista nel browser, D46.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline d70878c..HEAD` → **tre**: `384ba60`, `708166e`, e questa chiusura; `git log --oneline 878e9ef..HEAD` → **quattordici** |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **116** (invariato: le P della revisione aspettano la decisione 94); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **90** (D90 è dell'ondata 16; le decisioni di questa sessione, 95–97, stanno qui sotto e non nella tabella D) |
| Quanto è corretto | `ledger.md`: ✅ la testa e i compiti 1–17, ✅ il 3, l'8 e il 13 in profondità; `grep -c ' ⬜' ledger.md` → **1** (il capoverso dello stato) |
| I rilievi «fatto» confermati nei rapporti | **146** il 2026-09-16, coi rapporti R1–R12 (R8 manca), col comando nel blocco **sotto** questa tabella; R6 ne porta sedici (R6-15 è «prosa») |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (baseline all'apertura, poi una corsa per commit e una in più per l'ondata rilanciata da `HEAD` dopo la correzione allo script: cinque in tutto); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> [|] wc -c` → `0`; `ledger.md`, `R6-report.md`, i `patch_*.py` LF |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1**; `<data>` e `<tempo>` non sono segnaposto (D75) |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio né la roadmap |
| Documenti fuori dal piano | nella cartella della revisione: `R6-report.md`, `patch_c13b.py`, `patch_closure18.py`; `ledger.md` con l'ondata 16 |
| File temporanei | nessuno nel repository. Lo scratchpad `…\4fa001a2-…\scratchpad\review\` porta lo scheletro, `p-titles.md`, il lockfile del modello dell'11, `probe-R11/`, `probe-R12/` (col modello compilato dell'8, decisione 92), `probe-R6/` (gli estratti), `R6-report.md` e le quattro cartelle dei revisori uccisi; `C:\Users\zagor\AppData\Local\Temp\probe-R6\` è il modello compilato del 13 **con l'ondata 16** e gli script di R6 (decisione 95); `C:\Users\zagor\AppData\Local\Temp\probe-R12-target` è il target di cargo del modello dell'8; lo scratchpad di questa sessione (`…\e1e94bbc-…`) porta i cinque log del cancello, `dispatch-R6.md`, i due messaggi di commit, `model13/` (gli undici file dell'ondata 16 come dettati) e `update_memory_reading.py` — tutto cancellabile quando i revisori hanno finito |
| Debito lasciato | **dichiarato per intero in `ledger.md`** («Mancano») e qui: la revisione in profondità di 15, 16, 17 (R8); la compilazione dei moduli riscritti del 12 e del 14 (R13); le **due A/B** al proprietario (decisione 94; «il resto spento»); le P-117… **dopo** la prima; i moduli riscritti e non compilati del 12 e del 14 |

Il comando dei rilievi «fatto» confermati — una riga per rilievo, la cella del verdetto seguita da quella della specie:

```bash
cat docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/R*-report.md | grep -cE '^\| R[0-9]+[ab]?-[0-9]+ .*\| *CONFERMATO[^|]*\| *fatto *\|'
```

#### Le decisioni prese scrivendo, oltre a quelle del registro

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 95 | l'ondata **INTERA** si applica al modello compilato del revisore e si misura **prima** di dettarla — compilatore, sonde, generatore, build, server di sviluppo e le mutazioni delle sonde nuove — non la sola misura che un rilievo dice «da rifare» (decisione 92); e il modello di un revisore **finito** si riusa e si passa al revisore dopo, invece di rifarlo | il modello c'era già coi `node_modules`, e un rimedio scritto sul testo è una previsione finché non gira (gotcha #57): **tre** rimedi di questa ondata sono usciti diversi dalla proposta del revisore misurandoli — **D90** (`define`) al posto di `?url` o di un secondo file; `!isBuilt` al posto di `isModule`, che avrebbe negato il caso «tipo sparito» della riga 8 della §2; il `git add` di R5-11 al posto del salvataggio in Python | ~15 minuti di misure per ondata; un modello mutato va rimesso com'era, o copiato prima |
| 96 | un rimedio proposto dal revisore che contraddice una forma che il repo ha già si sostituisce con la **forma del repo**, e il registro dice quale e perché (R6-6 → la forma di R5-11 e del Passo 3 del 14) | coerenza, criterio 2 di `decision-principles`: un revisore lavora con meno contesto del coordinatore, e la sua proposta è una **dichiarazione**, non una decisione | un rimedio in meno preso «così com'è» |
| 97 | una lacuna della tabella di copertura del revisore che non ha un numero di rilievo diventa una voce del coordinatore (C13-N) e, se il piano non può deciderla da sé, una **riga nella tabella *«Le voci aperte che questo piano SA, e non chiude»*** con l'A/B per il proprietario — non una D presa in silenzio, e non un'omissione | «il resto spento» (§6a) non era prodotto da nessun passo e nessuna D lo diceva: dichiararlo aperto col chiusore è la forma di questo repo, e deciderlo qui avrebbe scelto per il proprietario una lettura della §6a | una riga in più nella tabella, e una domanda in più alla rilettura |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **IN SVILUPPO `define` DI `vite` NON RISCRIVE IL SORGENTE**: la costante arriva come globale dal modulo `/@vite/env`, quindi un
  `curl` sul modulo trasformato non la mostra (misurato: 0) e **non è un oracolo**; l'oracolo in sviluppo è `/@vite/env` (1) e la
  pagina aperta nel browser; alla build è sostituita nel bundle, sotto `vitest` è un globale. Il criterio del 13 lo dice così.
- ⚠️ **IL BROWSER DELL'APP RIFIUTA `navigate` SU `localhost`**: si apre con `preview_start` e l'URL; poi `get_page_text`,
  `read_console_messages` e `javascript_tool` (`getComputedStyle`) misurano ciò che il criterio chiede di **guardare**.
- ⚠️ **UN SERVER DI SVILUPPO LANCIATO IN BACKGROUND SI SPEGNE PER PID**: `netstat -ano | grep ':PORTA' | grep LISTENING` →
  `taskkill //F //PID`, mai `taskkill //IM node.exe`, che ucciderebbe anche l'harness; l'attesa che sia su è un ciclo limitato con
  `python -c "import time; time.sleep(0.5)"`, perché il `sleep` in primo piano del tool Bash è bloccato.
- ⚠️ **LE GUARDIE DI UNO SCRIPT SI CONTANO SUL TESTO CHE LO SCRIPT PRODUCE, NON SU QUELLO CHE SI RICORDA**: tre conteggi su undici
  erano sbagliati alla prima stesura (2→3, 6→5, 4→3) e sono stati ricontati rileggendo le sostituzioni **prima** del lancio; lo
  script è passato al primo colpo.
- ⚠️ **UN `grep` CON UN BACKTICK DENTRO UN CODE SPAN SPEZZA IL MARKDOWN**: si cerca un frammento senza backtick, e lo script si
  corregge e si rilancia da `HEAD` con `restore_from_head.py`, così lo script committato riproduce lo stato finale — non si
  ritocca il piano a mano dopo lo script.
- ⚠️ **`latest` DI `@types/node` INDICA LA 22 MENTRE NODE È 24**: la versione si appunta per major, `npm view @types/node@24 version | tail -1`.
- ⚠️ **IL COMMIT DEL RAPPORTO E QUELLO DELL'ONDATA SONO DUE** (decisione 90): il rapporto è la dichiarazione del revisore com'è
  arrivata, l'ondata è ciò che il coordinatore ne ha fatto — e fra i due stanno le misure.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, le voci aperte — che ora
   hanno la riga «il resto spento»; la tabella D a 25 righe per chiamata), poi
   [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md) **per intero**. Di ogni
   rapporto si legge **la riga** del rilievo (`grep -h '^| R6-3 ' R6-report.md`), non il rapporto.
3. ⛔ **Prima di qualunque dispaccio**: `ls /c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/4fa001a2-dff2-4d38-9775-1dfa17966e6f/scratchpad/review/`.
   Se c'è (`skeleton.md`, `p-titles.md`, `model11-lock/`), i mandati valgono così come sono e le cartelle dei revisori uccisi sono
   **già** da parte; se non c'è più, si rigenera coi comandi del punto 3 della sedicesima chiusura e si corregge la riga
   «scratchpad» del richiamo in `constraints.md`, committando prima del dispaccio. ⚠️ Dal `b3422bf` che `constraints.md`
   nomina, i compiti 12, 16 e 17 **non sono cambiati**; il 13 è l'ondata 16, il 14 ha la riga *Files* e il Passo 3 sui token
   (R6-11), il 15 la riga *Consumes* (R6-4): `git diff -U0 b3422bf..HEAD -- <questo file> \| grep '^@@'` lo mostra, e il prompt di
   dispaccio dice `HEAD` di oggi e quanti commit dopo.
4. ⏭️ **LA REVISIONE IN PROFONDITÀ CHE RESTA, UN REVISORE ALLA VOLTA SU OPUS**, ciascuno annunciato col costo e dispacciato col sì
   del proprietario, nell'ordine **R8** (15–17: ~1 500 righe, niente da compilare — stimato come R12, ~300k token e ~30 minuti) e
   **R13** (la compilazione dei moduli del 12 e del 14: il modello del 13 con l'ondata 16 sta in
   `C:\Users\zagor\AppData\Local\Temp\probe-R6\gui\` e si **copia** come base, decisione 95 — stimato come R6). Il prompt di
   dispaccio è di **tre righe**, nella forma di `dispatch-R6.md` nello scratchpad di questa sessione (se c'è ancora) o del punto 4
   della diciassettesima chiusura: chi sei e i due file da leggere, lo scratchpad `4fa001a2` per esteso, la cartella di prova
   `review/probe-RN/` da creare, il rapporto a pezzi con la data, `HEAD` di oggi e quanti commit dopo `b3422bf`, che il perimetro
   non è cambiato (o com'è cambiato), `git status --porcelain` vuoto alla fine e riportato. Il rapporto si copia nel repo e si
   committa **da solo** col cancello verde (decisione 90); poi le correzioni in **una ondata per compito** — `patch_c13b.py` è il
   modello più recente: la fetta del compito, ogni ancora asserita, le guardie contate sul testo prodotto, il registro nello stesso
   commit, `check_after_write.sh`, il diff letto per intero, il cancello, il commit senza co-autore. ⛔ **L'ondata intera si
   misura sul modello compilato prima di dettarla, quando un modello esiste** (decisione 95).
5. ⛔ **Prima delle P-117…, le DUE A/B al proprietario, una per volta**: la decisione 94 (due righe, consiglio **B**), poi «il resto
   spento» (la riga delle voci aperte, consiglio **A**). Finché la prima non ha risposta, `grep -c '^### P-'` resta **116**; la
   seconda decide se il 13 riceve un passo in più o la §6a un richiamo dal 14.
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
t = sub(t, "## Come si riprende — il diario di questo piano, coi comandi\n\n### La diciassettesima chiusura",
        "## Come si riprende — il diario di questo piano, coi comandi\n\n" + CLOSURE + "### La diciassettesima chiusura")

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: eighteenth closure written;", t.count("\n") - raw.count("\n"), "plan lines added")
