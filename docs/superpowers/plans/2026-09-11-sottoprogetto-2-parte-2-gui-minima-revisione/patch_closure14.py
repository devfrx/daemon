"""patch_closure14.py -- the fourteenth closure in the diary of the plan, and the dated recall in the head. Asserted, atomic.
The hash of the task-12 wave is read from git at run time (HEAD when this runs), never from memory."""
import io
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw

WAVE9 = subprocess.run(["git", "log", "--format=%h", "-1"], cwd=ROOT, capture_output=True, check=True, text=True).stdout.strip()
assert len(WAVE9) >= 7

HEAD_OLD = ("✅ **RICHIAMO DEL 2026-09-15, quarta sessione del giorno: le correzioni sono applicate ai compiti 1–10** — l'8 nelle sole due\n"
            "correzioni note — e la **tredicesima chiusura** dice dove si riprende: i compiti 11–17, poi la revisione in profondità di\n"
            "3, 8, 13, 15, 16, 17, poi le P nuove.\n")
HEAD_NEW = HEAD_OLD + ("✅ **RICHIAMO DEL 2026-09-15, quinta sessione del giorno: le correzioni sono applicate ai compiti 1–12** — e la\n"
                       "**quattordicesima chiusura** dice dove si riprende: i compiti 13–17, poi la revisione in profondità di 3, 8, 13, 15, 16, 17,\n"
                       "poi le P nuove. ⚠️ Il modulo delle sonde del 12 è riscritto e **non compilato**: chi esegue il 12 lo compila per primo.\n")
assert t.count(HEAD_OLD) == 1
t = t.replace(HEAD_OLD, HEAD_NEW)

CLOSURE = r"""### La quattordicesima chiusura — 2026-09-15, quinta sessione del giorno: le correzioni della revisione sono applicate ai compiti 1–12; restano 13–17, la revisione in profondità di 3, 8, 13, 15, 16, 17 e le P nuove; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, quattro cose.** **(1)** `ledger.md` nella cartella della revisione resta la **casa unica** di ciò
che resta: ✅ la testa e i compiti **1–12**, ⬜ tutto il resto — `grep -c ' ⬜' ledger.md` lo conta. **(2)** ⛔ **Il modulo
delle sonde del compito 12 è RISCRITTO PER INTERO e NON COMPILATO**: i compiti 2 e 7 non esistono ancora nel repo, quindi
il modello non si poteva costruire; la riga del registro lo dichiara e la posizione lo ripete. Chi esegue il 12 lo
compila **per primo**, e ogni rosso è una voce d'errata — la forma vecchia, quella che il revisore R5 aveva compilato,
panicava al primo messaggio (R5-3) e digitava la parola a nessuno (R5-4). **(3)** ⛔ **R5-11 è applicata in una forma
DIVERSA da quella che il registro annotava:** non `git add -N` ma `git add` vero, in scena, prima delle mutazioni
(Passi 4 e 13 dell'11, Passo 9 del 12) — misurato in un repository di prova: con *intent-to-add* `git diff --stat`
mostra il file **intero** anche a revoca fatta, quindi «vuoto» non è mai vero, e `git checkout --` lo **svuota**.
**(4)** Per ripartire da `HEAD` c'è ora `restore_from_head.py` accanto al registro: è la ricetta della trappola (1)
della tredicesima chiusura fatta comando — `git show` + Python con `newline=""`, mai `git checkout --`.

✅ **Che cosa è stato fatto.** Due ondate, ciascuna col cancello verde prima del commit: `3f13b2c` (compito 11, ondata
8) e `WAVE9` (compito 12, ondata 9), più questa chiusura. Ogni rilievo bloccante è stato **rimisurato col proprio
comando** prima del rimedio (decisione 71): `IpcMessage::decode` che comincia con `framing::unframe`; `redb = "4.1.0"`
col caret in `crates/platform/Cargo.toml` e la 4.1.0 nel lockfile di radice; `rustup show active-toolchain` da una
sottocartella; il comando di P-63 che rendeva zero righe; l'intersezione di `engines` rifatta oggi con `typescript`
5.9.3 (`jsdom` resta il più stretto su ogni ramo); i `dist-tags` del registro npm; e il comportamento di `git add -N`.
Nessun rilievo bloccante è risultato falso. ⚠️ **Cinque cose che il registro NON elencava e sono entrate**, ciascuna
con la propria riga: il richiamo **D79 su P-2** (D79 lo prometteva e la testa non lo portava); **D76** nei commenti dettati
dell'11 e del 12; la riga R5-11 per il 12; il pari del 12 **riallineato al 9 corretto** — scadenza di parete di 5 s sul
`connect` (R4-6), budget `WITH_A_PEER` (R4-7) — perché il 12 lo dichiara «nella forma del 9» ma lo aveva copiato
**prima** di quelle correzioni; e `take(WELCOME)` nella sonda dell'accoglienza, perché una `read` può portare anche il
primo `Token`.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline 9b64df0..HEAD` → tre (due ondate più questa chiusura) |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **116** (invariato: le P della revisione sono da scrivere); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **88**; nessuna `D` nuova in questa sessione |
| Quanto è corretto | `ledger.md`: ✅ la testa e i compiti 1–12; `grep -c ' ⬜' ledger.md` dice quante righe restano |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (baseline all'apertura, poi una corsa per ondata e una per questa chiusura; i log nello scratchpad, che non sopravvive); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> [|] wc -c` → `0`; `ledger.md`, i `patch_*.py` e `restore_from_head.py` LF |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1** |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio né la roadmap |
| Documenti fuori dal piano | `ledger.md`, `patch_c11.py`, `patch_c12.py`, `restore_from_head.py` e `patch_closure14.py` nella cartella della revisione |
| File temporanei | nessuno nel repository; lo scratchpad porta i log del cancello, e **non sopravvive**; il repository di prova di `git add -N` è stato cancellato |
| Debito lasciato | **dichiarato per intero in `ledger.md`**: le correzioni dei compiti 13–17 (le righe note per 13, 15, 16, 17; le R7 per il 14), la revisione in profondità di 3, 8, 13, 15, 16, 17, le P-117…, e il modulo delle sonde del 12 da compilare |

#### Le decisioni prese scrivendo, oltre a quelle del registro

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 76 | un rimedio annotato nel registro si **rimisura** prima di applicarlo, e se la misura lo smentisce si applica la forma che regge e il registro riceve la misura (R5-11: `git add`, non `-N`) | il registro è una nota del coordinatore, non una `D`; un rimedio che rende falso l'Atteso del piano è un debito con la firma della revisione | una riga di registro più lunga |
| 77 | un modulo che cinque rilievi toccano in helper e sonde si **riscrive intero** nello script, con le ancore sul suo inizio e sulla sua fine, invece di venti sostituzioni a riga | venti ancore in un blocco che cambia forma si contraddicono a vicenda; una riscrittura si legge come un file | il diff è più grande, e si legge per intero |
| 78 | un compito dichiarato «nella forma del compito N» si **ridiffa contro N corretto** prima di scriverne le ancore, e ciò che N ha guadagnato dalla revisione entra anche qui, col rilievo di N accanto | il 12 aveva copiato il pari del 9 prima di R4-6 e R4-7: era la forma vecchia con una dichiarazione di coerenza sopra | una riga di registro per compito allineato |
| 79 | i richiami che un compito deve nei disegni stanno in **un** Passo con la tabella delle ancore (13-bis dell'11, 9-bis del 12), e un dedotto vicino (D22) entra nello stesso richiamo quando il rilievo lo ammette (R3-18) | un Passo per richiamo moltiplica le ancore; la forma è quella del Passo 9-ter del compito 2 | nessuno, se il criterio conta i richiami |
| 80 | la ricetta di una trappola che si ripete diventa un **attrezzo** accanto al registro (`restore_from_head.py`) | la tredicesima chiusura la descriveva a mano, in quattro comandi; ripeterla a mano è dove si sbaglia | un file in più da tenere LF |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **`git add -N` NON RENDE `git diff` UN ORACOLO SU UN FILE NUOVO**: il diff mostra il file intero anche a revoca
  fatta, e `git checkout --` su un *intent-to-add* **svuota** il file (misurato). 📌 Prima delle mutazioni si mette in
  scena con `git add` vero; il commit del compito lo rifà comunque.
- ⛔ **UN'INSERZIONE ANCORATA ALL'INIZIO DI UN CAPOVERSO LO SPEZZA**: la nota delle sonde riscritte è finita **dentro**
  la frase *«in DUE passi** — manifesto e …»*, perché l'ancora di fine del blocco era l'inizio di quel capoverso.
  Vista solo leggendo `git diff -U1` dell'ondata prima del cancello; rifatto da `HEAD` con `restore_from_head.py` e lo
  script corretto, così lo script committato è quello applicato. 📌 Ciò che si appende dopo un blocco di codice si
  àncora alla **recinzione di chiusura** e ri-emette per intero l'inizio del capoverso che segue; e il diff dell'ondata
  si legge **sempre**, per intero, prima del cancello.
- ⛔ **`print` DI PYTHON SU QUESTA MACCHINA È cp1252**: una sonda usa-e-getta che stampa `—` o `…` muore a metà con
  `UnicodeEncodeError`, e le ancore dopo quel punto restano non verificate. 📌 `sys.stdout.reconfigure(encoding="utf-8")`
  in testa a **ogni** Python, anche a quelli da un rigo.
- ⛔ **LO SCRATCHPAD SUPERA MAX_PATH PER GIT**: `git init` lì fallisce con *Filename too long*. 📌 Un repository di prova
  va in un percorso corto (`C:/Users/zagor/AppData/Local/Temp/claude/<nome>`) e si cancella dopo.
- ⚠️ **UN COMPITO «NELLA FORMA DEL COMPITO N» PUÒ ESSERE LA FORMA VECCHIA DI N**: se N è stato corretto dopo, la
  dichiarazione di coerenza è falsa alla lettera. 📌 `grep` delle costanti e dei commenti che la correzione di N ha
  introdotto (`WITH_A_PEER`, `R4-6`) dentro il compito che lo copia: zero risultati è il sintomo.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, la tabella D fino a
   D88, le voci aperte), poi [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md)
   **per intero**: è la mappa. Di ogni rapporto si legge **la riga** del rilievo che si sta applicando
   (`grep -h '^| R7-1 ' R7-report.md`), non il rapporto; le righe lunghe si tagliano con `cut -c1-1500`.
3. ⏭️ **LE CORREZIONI DEI COMPITI 13–17, nell'ordine della posizione**: uno script di patch per ondata sul modello di
   `patch_c11.py` e `patch_c12.py` (fetta del compito, ancore asserite, scrittura atomica; il registro nello stesso
   script), `check_after_write.sh` dopo ogni scrittura, **il diff dell'ondata letto per intero**, il cancello e un
   commit per ondata; ogni riga del registro passa a ✅ nello stesso commit. ⚠️ Per il 13 le righe note sono sei
   (R9b-11 con **D80**, R9b-12 con **D81**, R10-9, R10-10, R7-7 — che si applica al **15** —, D75) e il compito è di
   1 760 righe: il blocco *Interfaces* e `dock.ts` cambiano forma con D80, e `frame.test.ts` riceve le due sonde;
   per il 14 le righe sono le R7; per 15, 16 e 17 le righe note del registro. ⚠️ Prima di scrivere un'ancora si
   rilegge il compito **intero**; un rimedio annotato si rimisura (decisione 76); un compito «nella forma di N» si
   ridiffa contro N corretto (decisione 78).
4. Poi **la revisione in profondità dei compiti 3, 8, 13, 15, 16, 17** — col metodo di `constraints.md`, o rilanciando
   `R6-prompt.md` e `R8-prompt.md` più due mandati piccoli per il 3 e l'8 quando il limite lo permette — e le correzioni
   che ne escono, nello stesso modo. ⚠️ **Il modulo delle sonde del 12 entra in questa revisione**: un revisore che
   compila il modello (il 2, il 7 e il 9 non esistono: si compila contro il codice di oggi dove si può, e si legge
   il resto) — o lo compila chi esegue il 12, per primo.
5. Poi le voci **P-117…** in coda alla sezione del pre-controllo: **una per rilievo «fatto» confermato**, raggruppate per
   compito, col comando che l'ha misurato; i rilievi di sola prosa restano nel registro. I conteggi `P` e `D` si
   confrontano col valore di `HEAD` dopo **ogni** scrittura.
6. ⛔ **Dopo ogni scrittura su questo file**: `check_after_write.sh` (tabelle spezzate, `tr -cd '\r'` a zero, i conteggi
   contro `HEAD`, `check-docs.sh` → `OK`), il diff letto, `bash scripts/gate.sh` → `GATE GREEN`, e il commit — **senza
   co-autore**. Per ripartire da `HEAD`: `python <revisione>/restore_from_head.py <piano> <revisione>/ledger.md`.
7. ⏭️ **Poi l'esecuzione, in una sessione NUOVA**: `superpowers:subagent-driven-development`, un subagente fresco per
   compito, con revisione fra uno e l'altro. ⛔ **Prima di eseguire il compito 11 si aggiorna Node** (P-64, P-65: oggi
   `v24.9.0`, fuori da `^24.15.0`), e su una macchina che non l'ha si installa `cargo-audit` (compito 16). Alla chiusura
   del piano la cartella della revisione si **archivia** (decisione 69).
8. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

"""
CLOSURE = CLOSURE.replace("`WAVE9`", f"`{WAVE9}`")

ANCHOR = "### La tredicesima chiusura — 2026-09-15, quarta sessione del giorno:"
assert t.count(ANCHOR) == 1
t = t.replace(ANCHOR, CLOSURE + ANCHOR)

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print(f"ok: the fourteenth closure is in the diary (wave 9 = {WAVE9});", t.count("\n") - raw.count("\n"), "lines added")
