"""patch_closure16.py -- the sixteenth closure in the plan's diary, and the recall line in the position section.
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


POSITION_OLD = ("store — perché il `Layout` dell'accoglienza non veniva mai mostrato: trovato leggendo il flusso per scrivere D80.\n\n"
                "| # | Compito | Commit | Stato |\n")
POSITION_NEW = ("store — perché il `Layout` dell'accoglienza non veniva mai mostrato: trovato leggendo il flusso per scrivere D80.\n"
                "✅ **RICHIAMO DEL 2026-09-16, la ripresa: la quindicesima chiusura è committata, il compito 3 è rivisto in profondità\n"
                "(R11, un revisore su Opus) e corretto (ondata 14)** — la **sedicesima chiusura** dice dove si riprende: la revisione in\n"
                "profondità di 8, 13, 15, 16, 17 (**un revisore alla volta su Opus, col costo detto prima e il sì del proprietario**), la metà\n"
                "dell'8 di R9b-5, poi le P nuove. ⛔ **I subagenti si dispacciano con `model: \"opus\"`, mai col modello ereditato.**\n\n"
                "| # | Compito | Commit | Stato |\n")

CLOSURE = r'''### La sedicesima chiusura — 2026-09-16, la ripresa dopo la sesta sessione del 15: la quindicesima chiusura committata, la revisione in profondità del 3 fatta e applicata; restano 8, 13, 15, 16, 17, la metà dell'8 di R9b-5 e le P nuove; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, quattro cose.** **(1)** ⛔ **I subagenti si dispacciano SOLO con `model: "opus"` (o `sonnet` per
lavoro meccanico), MAI col modello ereditato, e più di uno alla volta SOLO dopo aver detto il costo al proprietario e
avuto il sì.** Questa sessione ha lanciato cinque revisori col modello ereditato (Fable) in parallelo e il proprietario li
ha uccisi dopo dieci minuti: *«per colpa tua ho finito 200eur di limite settimanale in 1 giorno»*. Un revisore Opus sul
compito 3 è costato 283k token, 69 comandi e 22 minuti. **(2)** `ledger.md` resta la **casa unica** di ciò che resta: ✅ la
testa e i compiti 1–17 nelle righe note, ✅ il 3 rivisto in profondità (R11: tredici rilievi, applicati dall'ondata 14), ⬜ la
sola metà dell'8 di R9b-5 — `grep -c ' ⬜' ledger.md` → **2** (la riga dello stato, e quella dell'8). **(3)** I mandati per gli
altri revisori sono **scritti e committati** nella cartella della revisione: `R12-prompt.md` (8, con la proposta del testo
esatto per la metà dell'8), `R6-prompt.md` (13) e `R8-prompt.md` (15–17) coi richiami del 2026-09-16 in coda, `R13-prompt.md`
(la compilazione dei moduli riscritti del 12 e del 14); `constraints.md` porta in testa il richiamo del 2026-09-16 che vince
sul resto. ⚠️ Quel richiamo nomina lo scratchpad `…\4fa001a2-dff2-4d38-9775-1dfa17966e6f\scratchpad`, che oggi esiste su disco
con `review/skeleton.md`, `review/p-titles.md`, `review/model11-lock/` e `review/probe-R11/`: chi riprende lo controlla con
`ls` **prima** di dispacciare, e se non c'è più lo rigenera (i comandi al punto 3 qui sotto) e corregge la riga «scratchpad»
del richiamo. **(4)** I moduli riscritti e **non compilati** del 12, del 13 e del 14 restano tali (R13 e R6 li compilano); il
3 invece è stato compilato da R11 in una copia del workspace, e i suoi due rossi sono corretti nel piano (R11-2, R11-3).

✅ **Che cosa è stato fatto.** Cinque commit, ciascuno col cancello verde prima (i log nello scratchpad della sessione, che
non sopravvive): `b3422bf` — la quindicesima chiusura e l'ondata 13 del 15, trovate **scritte e non eseguite** alla ripresa
(`patch_closure15.py` mai lanciato, `patch_c3.py` applicato e non committato; ogni riga della sua tabella dello stato
rimisurata prima del commit, tutte vere); `eee70d9` — il dispaccio: il richiamo in `constraints.md`, i richiami in coda a
R6 e R8, i tre mandati nuovi; `8567e74` — il rapporto `R11-report.md` copiato nel repo; l'ondata 14 — il compito 3 corretto
sui tredici rilievi di R11 (`patch_c3b.py`); e questo, la chiusura. ⚠️ **Tre cose che il registro NON diceva e sono entrate:**
il numerale «sei punti» di D76 nel registro del 3 era falso (cinque, contati da R11); la cura di P-35 conta ora anche la
casa in inglese (`grep -ci eleven`); e G7 muta l'**ultima** variante, non `Steps` in mezzo (decisione 89).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline 878e9ef..HEAD` → **otto**: i tre del 15 fino ad `ab1b138`, poi `b3422bf`, `eee70d9`, `8567e74`, l'ondata 14, e questa chiusura |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **116** (invariato: le P della revisione sono da scrivere); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **89** (invariato: nessuna `D` nuova oggi) |
| Quanto è corretto | `ledger.md`: ✅ la testa e i compiti 1–17, ✅ il 3 in profondità; `grep -c ' ⬜' ledger.md` → **2** |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (baseline all'apertura, poi una corsa per commit); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> [|] wc -c` → `0`; `ledger.md`, i `patch_*.py`, i `R*-prompt.md` e `constraints.md` LF |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1**; `<data>` e `<tempo>` non sono segnaposto (D75) |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio né la roadmap |
| Documenti fuori dal piano | nella cartella della revisione: `constraints.md` (richiamo in testa), `R6-prompt.md` e `R8-prompt.md` (richiami in coda), `R11-prompt.md`, `R12-prompt.md`, `R13-prompt.md`, `R11-report.md`, `patch_c3b.py`, `patch_closure16.py` |
| File temporanei | nessuno nel repository; lo scratchpad `…\4fa001a2-…` porta `review/` (scheletro, p-titles, il lockfile del modello dell'11, la copia compilata del workspace di R11 in `probe-R11/ws/`) e `C:\Users\zagor\AppData\Local\Temp\probe-R11-target` è il target di cargo di quella copia — entrambi cancellabili quando i revisori hanno finito |
| Debito lasciato | **dichiarato per intero in `ledger.md`**: la revisione in profondità di 8, 13, 15, 16, 17; i moduli riscritti e non compilati del 12, 13 e 14; la metà dell'8 di R9b-5; le P-117… (anche per i tredici rilievi di R11) |

#### Le decisioni prese scrivendo, oltre a quelle del registro

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 86 | ogni `Agent` passa `model: "opus"` (o `sonnet` per lavoro meccanico), mai il modello ereditato; **un revisore alla volta**, e il costo si dice al proprietario prima del dispaccio | decisione del **proprietario** del 2026-09-16: cinque revisori Fable in parallelo hanno bruciato il limite settimanale in un giorno | una campagna più lenta, e un giro di conferma per dispaccio |
| 87 | una correzione applicata **senza revisione** (le righe del registro) la verifica il revisore in profondità come tutto il resto, e non la ripropone se regge (verdetto «GIÀ COPERTO») | D80, D81, D89 e le righe dell'ondata 12 sono scritte dal coordinatore sul modello letto e mai compilato; R11 ha verificato le tre del 3 e trovato falso il numerale del registro | qualche riga «GIÀ COPERTO» in più nei rapporti |
| 88 | uno script di chiusura si esegue **nello stesso passo** che lo scrive, e il commit nello stesso passo del cancello | la quindicesima chiusura, scritta e non eseguita, ha lasciato due stati che la ripresa ha dovuto riconciliare | nessuno |
| 89 | una mutazione che toglie una variante toglie l'**ultima**, così l'Atteso resta leggibile alla lettera (G7: `Verdict`, non `Steps`) | R11-10: togliere una variante in mezzo fa slittare gli indici di tutte le successive — sei file di troppo e quattro mancanti, un Atteso che nessuno può leggere | la mutazione prova una variante sola |
| 90 | il rapporto di un revisore si **committa da solo**, prima di applicare le correzioni | un rapporto nello scratchpad muore con la sessione, e le correzioni si applicano in un'ondata a parte con cancello e commit propri | un commit in più per revisore |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **UN `Agent` SENZA `model` EREDITA IL MODELLO DELLA SESSIONE, cioè il più caro**: ogni dispaccio scrive `model`, e una
  campagna si annuncia col numero di agenti, la taglia del pacchetto e il modello, e aspetta il sì.
- ⛔ **UNA CHIUSURA SCRITTA COME SCRIPT E NON ESEGUITA LASCIA DUE STATI** (trappola della quindicesima, scattata su di lei):
  alla ripresa `git diff --stat` e `grep -c '<titolo>' <questo file>` → 0 lo dicono; le righe «Stato alla chiusura» si
  rimisurano prima di eseguire e committare.
- ⚠️ **L'`awk` DI QUESTA GIT BASH NON CONOSCE `\b`**: il taglio dello scheletro per compito è `/^- \[.\] \*\*Passo 1[:. ]/`;
  se i blocchi escono a centinaia di righe invece che a decine, il taglio non ha morso.
- ⚠️ **LO SCRATCHPAD DELLA SESSIONE PRECEDENTE SOPRAVVIVE SU DISCO** (contro ciò che le chiusure dicono): il modello dell'11
  installato coi `node_modules` era in `probe-R5/gui-ts5/`; gli estratti dei revisori morti sono di **prima** delle
  correzioni e non si usano.
- ⚠️ **LA SESSIONE CAMBIA SCRATCHPAD A OGNI COMPATTAZIONE** (tre percorsi in un giorno): i file che un documento del repo
  nomina (`constraints.md`) restano nel primo e si cita il percorso per esteso; `ls` prima di ogni dispaccio.
- ⚠️ **UNA GUARDIA DI FINE SCRIPT CHE CONTA UNA PAROLA INSERITA DUE VOLTE PER COSTRUZIONE** (la riga della tabella e la frase
  che la cita) scatta su sé stessa: si asserisce il numero atteso, non 1 — `patch_c3b.py`, una corsa a vuoto senza scrivere.
- ⚠️ **LA TABELLA D DEL PIANO A 60 RIGHE PER CHIAMATA TRABOCCA** (48 KB): si legge a 25, o la riga singola col
  `grep -n '^| \*\*D23\*\* |'`.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, le voci aperte; la
   tabella D a 25 righe per chiamata o a riga singola), poi
   [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md) **per intero**. Di ogni
   rapporto si legge **la riga** del rilievo (`grep -h '^| R11-6 ' R11-report.md`), non il rapporto.
3. ⛔ **Prima di qualunque dispaccio**: `ls /c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/4fa001a2-dff2-4d38-9775-1dfa17966e6f/scratchpad/review/`.
   Se c'è (`skeleton.md`, `p-titles.md`, `model11-lock/`), i mandati valgono così come sono. Se non c'è più, nello
   scratchpad della sessione:

   ```bash
   P=docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md; S=<scratchpad>/review; mkdir -p "$S"
   { echo "# Scheletro del piano della parte 2 — estratto il $(date +%F) da HEAD $(git rev-parse --short HEAD) (sola lettura; il piano vero è $P)"; echo
     echo "## [A] Vincoli globali"; awk '/^## Vincoli globali/{f=1} /^## ▶️ A che punto/{exit} f' "$P"
     echo "## [B] La tabella della posizione"; awk '/^## ▶️ A che punto/{f=1} /^### ▶️ Come si esegue/{exit} f' "$P"
     echo "## [C] Le voci aperte che il piano SA, e non chiude"; awk '/^## Le voci aperte che questo piano SA/{f=1} /^## Compito 1:/{exit} f' "$P"
     echo "## [D] Per ogni compito: intestazione, Files, Interfaces (fino al Passo 1 escluso)"
     awk '/^## Compito [0-9]+:/{f=1} /^- \[.\] \*\*Passo 1[:. ]/{f=0} /^## Come si riprende/{exit} f' "$P"; } > "$S/skeleton.md"
   grep -n '^### P-' "$P" > "$S/p-titles.md"
   ```

   Atteso: `grep -c '^## Compito' "$S/skeleton.md"` → 17, e blocchi da venti a sessanta righe l'uno; il lockfile del modello
   dell'11 si rifà dal Passo 3 del compito 11 (`npm install` alle versioni dettate, in una cartella di prova). Poi si
   corregge la riga «scratchpad» del richiamo in `constraints.md` col percorso nuovo, e si committa prima del dispaccio.
4. ⏭️ **LA REVISIONE IN PROFONDITÀ CHE RESTA, UN REVISORE ALLA VOLTA SU OPUS**, ciascuno annunciato col costo e dispacciato
   col sì del proprietario: **R12** (compito 8, con la proposta del testo esatto per la metà dell'8 di R9b-5), poi **R6**
   (13), **R8** (15–17), **R13** (la compilazione dei moduli del 12 e del 14). Il prompt di dispaccio è di tre righe e
   punta a `constraints.md` e a `RN-prompt.md`; il rapporto va in `<scratchpad>/review/RN-report.md`, si copia nel repo e
   si committa col cancello verde (decisione 90); poi le correzioni in **una ondata per compito** — `patch_c3b.py` è il
   modello: la fetta del compito, ogni ancora asserita, le guardie col numero atteso, il registro nello stesso commit,
   `check_after_write.sh`, il diff letto per intero, il cancello, il commit senza co-autore.
5. La metà dell'8 di **R9b-5** (la riga ⬜ del registro), col testo che R12 propone.
6. Poi le voci **P-117…** in coda alla sezione del pre-controllo: **una per rilievo «fatto» confermato**, raggruppate per
   compito, col comando che l'ha misurato — i tredici di R11 compresi; i conteggi `P` e `D` si confrontano col valore di
   `HEAD` dopo **ogni** scrittura.
7. ⛔ **Dopo ogni scrittura su questo file**: `check_after_write.sh`, il diff letto, `bash scripts/gate.sh` → `GATE GREEN`, e il
   commit — **senza co-autore**. Per ripartire da `HEAD`: `python <revisione>/restore_from_head.py <piano> <revisione>/ledger.md`.
8. ⏭️ **Poi l'esecuzione, in una sessione NUOVA**: `superpowers:subagent-driven-development`, un subagente fresco per
   compito **su Opus**, con revisione fra uno e l'altro. ⛔ **Prima di eseguire il compito 11 si aggiorna Node** (P-64, P-65:
   oggi `v24.9.0`, fuori da `^24.15.0`); `cargo-audit` 0.22.2 c'è. Alla chiusura del piano la cartella della revisione si
   **archivia** (decisione 69).
9. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

'''

t = sub(t, POSITION_OLD, POSITION_NEW)
t = sub(t, "## Come si riprende — il diario di questo piano, coi comandi\n\n### La quindicesima chiusura",
        "## Come si riprende — il diario di questo piano, coi comandi\n\n" + CLOSURE + "### La quindicesima chiusura")

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: sixteenth closure written;", t.count("\n") - raw.count("\n"), "plan lines added")
