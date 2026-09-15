"""patch_closure13.py -- the thirteenth closure in the diary of the plan, and the dated recall in the head. Asserted, atomic."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
PLAN = r"C:\Users\zagor\Desktop\harness\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw

HEAD_OLD = ("applicate ai compiti 1–5** — la **dodicesima chiusura** dice dove si riprende, e il registro in\n"
            "[`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md) è la casa unica di ciò che resta.\n")
HEAD_NEW = HEAD_OLD + ("✅ **RICHIAMO DEL 2026-09-15, quarta sessione del giorno: le correzioni sono applicate ai compiti 1–10** — l'8 nelle sole due\n"
                       "correzioni note — e la **tredicesima chiusura** dice dove si riprende: i compiti 11–17, poi la revisione in profondità di\n"
                       "3, 8, 13, 15, 16, 17, poi le P nuove.\n")
assert t.count(HEAD_OLD) == 1
t = t.replace(HEAD_OLD, HEAD_NEW)

CLOSURE = r"""### La tredicesima chiusura — 2026-09-15, quarta sessione del giorno: le correzioni della revisione sono applicate ai compiti 1–10; restano 11–17, la revisione in profondità di 3, 8, 13, 15, 16, 17 e le P nuove; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, tre cose.** **(1)** `ledger.md` nella cartella della revisione resta la **casa unica** di ciò
che resta: ✅ la testa e i compiti **1–10** (l'8 nelle sole due correzioni note, D77: la sua revisione in profondità
è ancora da fare), ⬜ tutto il resto. Accanto ci sono `patch_c6.py`, `patch_c7.py`, `patch_c89.py` e `patch_c10.py`:
uno script per ondata, **fetta del compito, TUTTE le ancore asserite prima di scrivere, scrittura atomica** — il
modello per i compiti 11–17. **(2)** ⛔ **Il piano NON si ripristina con `git checkout --`**: con `core.autocrlf=true`
il checkout lo riscrive **CRLF** nell'albero (`i/lf w/crlf`), e ogni script di patch — che asserisce `"\r\n" not in raw`
— si ferma **senza scrivere**. La ricetta che regge è sotto, nelle trappole. **(3)** Sulla macchina restano
`cargo-audit` 0.22.2 e Node `v24.9.0`, invariati; `python` è 3.13.7.

✅ **Che cosa è stato fatto.** Quattro ondate, ciascuna col cancello verde prima del commit: `a265af8` (compito 6),
`cb5c378` (compito 7), `1893c29` (compiti 8 e 9) e questo commit (compito 10, più questa chiusura). Ogni rilievo
bloccante è stato **rimisurato dal coordinatore col proprio comando** prima del rimedio (decisione 71): i quattro
`match` esaustivi su `RecordKind`, i ventinove attributi indentati, il censimento di `Parameters::new`, `unframe` su
tre byte, `FILE_FLAG_FIRST_PIPE_INSTANCE` letto nel sorgente di `interprocess` 2.4.4, `E50`/`E66`/`E112` ritrovati
nel piano del Traguardo 6. Le decisioni **D75–D88** sono applicate dove i compiti le dovevano. ⚠️ **Tre cose che il
registro NON elencava e sono entrate**, ciascuna con la propria riga di registro: l'etichetta eol di `record_v1.map`
nel 6 (R10-16 la nominava, la lista del 6 no); le costanti `WELCOME` e `WITH_A_PEER` del banco del 9, che il piano
usava e nessun passo definiva; il richiamo in **P-45**, il cui ✅ *«corretto nel compito 6»* era una promessa.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline fb5b576..HEAD` → quattro (tre ondate più questa chiusura) |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **116** (invariato: le P della revisione sono da scrivere); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **88**; nessuna `D` nuova in questa sessione |
| Quanto è corretto | `ledger.md`: ✅ la testa e i compiti 1–10; `grep -c ' ⬜' ledger.md` dice quante righe restano |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (baseline all'apertura alle 13:47, poi una corsa per ondata; i log nello scratchpad, che non sopravvive); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> [|] wc -c` → `0`; `ledger.md` e i `patch_*.py` LF |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1**; i corpi `/* … */` del compito 9 sono dettati: `awk '/^## Come si riprende/{exit} /\{ \/\* … \*\/ \}/{c++} END{print c+0}' <questo file>` → **0** |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio né la roadmap |
| Documenti fuori dal piano | `ledger.md` e i quattro `patch_c*.py` nella cartella della revisione, più `patch_closure13.py` |
| File temporanei | nessuno nel repository; lo scratchpad porta i log del cancello e `plan_head.md` (la copia LF di `HEAD` per ripartire), e **non sopravvive** |
| Debito lasciato | **dichiarato per intero in `ledger.md`**: le correzioni dei compiti 11–17 (le righe R5 per 11 e 12, R7 per 14, le R9a/R9b/R10 per 13, 15, 16, 17), la revisione in profondità di 3, 8, 13, 15, 16, 17, le P-117… |

#### Le decisioni prese scrivendo, oltre a quelle del registro

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 72 | una decisione `D` globale (D76) si applica anche dove il registro non la elencava (il 6), e il registro riceve la riga | la coerenza fra compiti vale più della lista: il 5 l'aveva ricevuta, il 6 no per svista del censimento | una riga di registro in più per ondata |
| 73 | un compito «non rivisto in profondità» (l'8) riceve le **sole** correzioni note e resta ⬜ per la profondità | applicare due righe non è rivedere; fingerlo farebbe sparire il debito dal registro | nessuno, se il registro lo dice |
| 74 | una sonda dettata ex novo dalla revisione (le due del 7, l'undicesima del 7, le quattro del 12 del 9) porta nel commento il rilievo che l'ha generata e la data | il prossimo censimento deve sapere da dove viene, come per i richiami datati | una riga di commento per sonda |
| 75 | il numero liberato da uno spostamento (Passo 14 → 7-bis nel 9) si riusa per il passo nuovo (la misura del processore, D84) invece di un «15-bis» | i passi restano numerati senza buchi, e il 14 è dove la misura ha senso — prima dei richiami e del cancello | nessuno, se il registro nomina lo spostamento |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **`git checkout -- <piano>` LO RISCRIVE CRLF** (`core.autocrlf=true`), e lo script si ferma sull'`assert`. 📌 La
  ricetta: `git show HEAD:<piano> > <scratchpad>/plan_head.md`, poi Python che legge quel file con `newline=""`,
  fa `.replace("\r\n", "\n")` e riscrive il piano (temporaneo e `os.replace`); `git ls-files --eol` → `i/lf w/lf` e
  `git status --porcelain` vuoto prima di rilanciare la patch.
- ⛔ **PYTHON SU WINDOWS NON RISOLVE `/tmp` DI GIT BASH**: un file scritto da bash in `/tmp/` non esiste per
  `io.open("/tmp/…")`. 📌 Ciò che Python deve leggere va nello scratchpad, col percorso Windows.
- ⛔ **UN BLOCCO SI SPOSTA PRIMA DI INSERIRE UN PASSO CHE NE RIUSA IL NUMERO**: l'ancora «fino a `Passo 15`» del
  blocco da spostare ha portato via anche il «Passo 14» nuovo, inserito prima. Scattato una volta, visto ai
  controlli mirati (l'elenco dei passi), rifatto da `HEAD`. 📌 Nello script, gli spostamenti stanno **prima** delle
  sostituzioni.
- ⛔ **UN'ANCORA CHE DIFFERISCE PER UN CARATTERE FERMA TUTTO SENZA SCRIVERE**, ed è il comportamento voluto:
  `--include=*.rs` contro `--include='*.rs'`, `--` contro `—`. 📌 `grep -n <frase> <piano> [|] cat -A` mostra i
  byte; e poiché `scoped()` asserisce **tutte** le ancore prima di scrivere, un fallimento lascia `git status`
  pulito — si controlla lo stesso.
- ⛔ **`\\<data\\>` DENTRO UNA STRINGA BASH DIVENTA `\<`, CHE PER `grep` È UN CONFINE DI PAROLA**: il conteggio
  dei richiami rendeva 0 su righe che c'erano. 📌 Si conta con `grep -c -F`.
- ⚠️ **UN CONTEGGIO DI RIGHE `⬜` DEL REGISTRO È UN'ANCORA**: `assert seg.count(" ⬜") == N` per la fetta del compito,
  contato prima con `grep`, così una riga dimenticata dal registro (come l'eol di `record_v1.map`) si vede al
  momento di spuntare e non dopo.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, la tabella D fino a
   D88, le voci aperte), poi [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md)
   **per intero**: è la mappa. Di ogni rapporto si legge **la riga** del rilievo che si sta applicando
   (`grep -n '^| R5-4 ' R5-report.md`), non il rapporto; le righe lunghe si tagliano con `cut -c1-1500` e si
   riprendono da `-c1500-`.
3. ⏭️ **LE CORREZIONI DEI COMPITI 11–17, nell'ordine della posizione**: uno script di patch per ondata sul modello di
   `patch_c89.py` (fetta del compito, ancore asserite, spostamenti prima delle sostituzioni, scrittura atomica),
   `check_after_write.sh` dopo ogni scrittura, il cancello e un commit per ondata di uno o due compiti; ogni riga del
   registro passa a ✅ nello stesso commit. ⚠️ Le decisioni sono **prese** (D75–D88): dove il registro dice «vedi
   rapporto» si esegue il rimedio proposto, e dove il rimedio contraddice una D vince la D. ⚠️ Per l'11 e il 12 le
   righe R5-1…R5-22, R5-28 e R5-35 sono già state lette in questa sessione e i loro rimedi sono nel registro: si
   parte da lì, rileggendo il compito **intero** prima di scrivere le ancore.
4. Poi **la revisione in profondità dei compiti 3, 8, 13, 15, 16, 17** — col metodo di `constraints.md`, o rilanciando
   `R6-prompt.md` e `R8-prompt.md` più due mandati piccoli per il 3 e l'8 quando il limite lo permette — e le correzioni
   che ne escono, nello stesso modo.
5. Poi le voci **P-117…** in coda alla sezione del pre-controllo: **una per rilievo «fatto» confermato**, raggruppate per
   compito, col comando che l'ha misurato; i rilievi di sola prosa restano nel registro. I conteggi `P` e `D` si
   confrontano col valore di `HEAD` dopo **ogni** scrittura.
6. ⛔ **Dopo ogni scrittura su questo file**: `check_after_write.sh` (tabelle spezzate, `tr -cd '\r'` a zero, i conteggi
   contro `HEAD`, `check-docs.sh` → `OK`), `bash scripts/gate.sh` → `GATE GREEN`, e il commit — **senza co-autore**.
7. ⏭️ **Poi l'esecuzione, in una sessione NUOVA**: `superpowers:subagent-driven-development`, un subagente fresco per
   compito, con revisione fra uno e l'altro. ⛔ **Prima di eseguire il compito 11 si aggiorna Node** (P-64, P-65), e su
   una macchina che non l'ha si installa `cargo-audit` (compito 16). Alla chiusura del piano la cartella della revisione
   si **archivia** (decisione 69).
8. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

"""

ANCHOR = "### La dodicesima chiusura — 2026-09-15, terza sessione del giorno:"
assert t.count(ANCHOR) == 1
t = t.replace(ANCHOR, CLOSURE + ANCHOR)

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: the thirteenth closure is in the diary;", t.count("\n") - raw.count("\n"), "lines added")
