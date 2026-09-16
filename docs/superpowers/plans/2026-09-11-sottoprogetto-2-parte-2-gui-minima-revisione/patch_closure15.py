"""patch_closure15.py -- the fifteenth closure in the plan's diary, and the recall line in the position section.
Two anchors, asserted; atomic write.
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


POSITION_OLD = "poi le P nuove. ⚠️ Il modulo delle sonde del 12 è riscritto e **non compilato**: chi esegue il 12 lo compila per primo.\n\n| # | Compito | Commit | Stato |\n"
POSITION_NEW = ("poi le P nuove. ⚠️ Il modulo delle sonde del 12 è riscritto e **non compilato**: chi esegue il 12 lo compila per primo.\n"
                "✅ **RICHIAMO DEL 2026-09-15, sesta sessione del giorno: le correzioni note sono applicate a TUTTI i compiti, 1–17** — la\n"
                "**quindicesima chiusura** dice dove si riprende: la revisione in profondità di 3, 8, 13, 15, 16, 17 (coi moduli riscritti e\n"
                "non compilati del 12, del 13 e del 14), la metà dell'8 di R9b-5, poi le P nuove. ⚠️ **D89 è nata al 13** — il dock segue lo\n"
                "store — perché il `Layout` dell'accoglienza non veniva mai mostrato: trovato leggendo il flusso per scrivere D80.\n\n| # | Compito | Commit | Stato |\n")

CLOSURE = r'''### La quindicesima chiusura — 2026-09-15, sesta sessione del giorno: le correzioni note sono applicate a TUTTI i compiti, 1–17; restano la revisione in profondità di 3, 8, 13, 15, 16, 17, la metà dell'8 di R9b-5 e le P nuove; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, quattro cose.** **(1)** `ledger.md` nella cartella della revisione resta la **casa unica** di ciò che
resta: ✅ la testa e i compiti **1–17**, ⬜ la sola metà dell'8 di R9b-5 — `grep -c ' ⬜' ledger.md` lo conta (la riga dello
stato in testa dice «tutto il resto ⬜» dal 2026-09-15 e conta anch'essa). **(2)** ⛔ **Tre compiti portano moduli RISCRITTI e
NON COMPILATI**, perché i compiti che li alimentano non esistono nel repo: il modulo delle sonde del 12; `layout.ts`,
`dock.ts`, `Frame.vue`, `stores.test.ts` e `frame.test.ts` del 13; `Frame.vue`, `Chat.vue`, `markdown.ts`, `modules.test.ts`
e `chat.test.ts` del 14. Chi li esegue li compila **per primi**, e ogni rosso è una voce d'errata. **(3)** ⛔ **D89 è una
decisione NUOVA, nata al 13 scrivendo D80:** il 13 dettava `apply` una volta, in `onMounted`, cioè **prima** del `Hello` di
`main.ts` — e il `Layout` dell'accoglienza aggiornava lo store senza che nulla lo mostrasse. Compilava e passava le sonde.
Ora il dock **segue lo store** (`watch([view, arrivals])`), lo store conta gli arrivi che non sono l'eco del proprio
`SaveLayout` (decisione 13), e `Frame.switchTo` è `layout.view = view`; la cascata sul `Frame.vue` ridettato dal 14 è
applicata (decisione 78). **(4)** Gli Attesi degli script dettati che toccano la stella (Passo 15 del 14, Passo 8-bis del 17)
sono **misurati** su una copia del file: 3/5/1/0/niente e 2/3/0/niente — non ipotesi.

✅ **Che cosa è stato fatto.** Quattro commit, ciascuno col cancello verde prima: `1825bf9` (compito 13, ondata 10),
`bf66963` (compito 14, ondata 11), `ab1b138` (compiti 15, 16 e 17, ondata 12), e questo (le tre righe note del 3, ondata 13,
con questa chiusura). Ogni rilievo bloccante è stato **rimisurato col proprio comando** prima del rimedio (decisione 71):
`keep_layout` del 7 che risponde con un `Layout` a ogni `SaveLayout`; `git ls-files --eol` di `gate.sh`, del flusso e
dell'audit (`w/crlf` tutti e tre, CR = righe); il criterio R9b-15 sulla roadmap (0 col `sed`, 1 senza); le nove e le cinque
ancore nella stella (una ciascuna); `design/10` (`i/lf w/lf`). Nessun rilievo bloccante è risultato falso. ⚠️ **Sei cose che
il registro NON elencava e sono entrate**, ciascuna con la propria riga: **C13-1 → D89**; la cascata di D80/D89 sul 14; R7-6
anche nel 13 (stesso criterio); D76 nel 13 e nel 14 (ventuno punti); il capoverso sotto il Passo 17 del 13 che diceva «nessuna
sonda monta una griglia»; e la forma di D87 (**un** richiamo per modulo, sul capoverso «Costruito dal 2», non su ogni riga),
scritta nella riga D87.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline 878e9ef..HEAD` → quattro (tre ondate, più l'ondata del 3 con questa chiusura) |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **116** (invariato: le P della revisione sono da scrivere); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **89** — **una `D` nuova**, D89 |
| Quanto è corretto | `ledger.md`: ✅ la testa e i compiti 1–17; `grep -c ' ⬜' ledger.md` → **2** (la riga dello stato, e la metà dell'8 di R9b-5) |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (baseline all'apertura, poi una corsa per ondata; i log nello scratchpad, che non sopravvive); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> [|] wc -c` → `0`; `ledger.md` e i `patch_*.py` LF |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | **uno solo, dichiarato**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1**; `<data>` e `<tempo>` non sono segnaposto (D75) |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio né la roadmap |
| Documenti fuori dal piano | `ledger.md`, `patch_c13.py`, `patch_c14.py`, `patch_c151617.py`, `patch_c3.py` e `patch_closure15.py` nella cartella della revisione |
| File temporanei | nessuno nel repository; lo scratchpad porta i log del cancello, e **non sopravvive**; le due copie della stella usate per misurare gli Attesi sono cancellate |
| Debito lasciato | **dichiarato per intero in `ledger.md`**: la revisione in profondità di 3, 8, 13, 15, 16, 17; i moduli riscritti e non compilati del 12, 13 e 14; la metà dell'8 di R9b-5; le P-117… |

#### Le decisioni prese scrivendo, oltre a quelle del registro

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 81 | un rilievo su un modulo si applica rileggendo il modulo **e ciò che lo alimenta** — il flusso, non la riga — e un difetto trovato lì entra come `D` nuova con la sonda (C13-1 → **D89**) | D80 guardava `LayoutPack`; il difetto stava fra `main.ts`, `createDock` e `receive`: il pacchetto dell'accoglienza non veniva mai mostrato, e nessun rilievo lo diceva | una `D` in più, e un compito che cambia forma |
| 82 | l'Atteso di uno script dettato che tocca un documento del repo si **misura su una copia** del documento in una cartella corta, prima del commit, e il piano lo dice | *«un'evidenza scritta prima della misura è un'ipotesi»* (`CLAUDE.md`), e uno script con nove ancore ne sbaglia una in silenzio | una corsa in più per passo |
| 83 | le ancore di uno script dettato che tocca un disegno si prendono **dal file** — per sezione e inizio di riga, o per frase contenuta, con l'unicità asserita — mai righe di novecento caratteri ricopiate nel piano | E6 del piano dei gesti; il Passo 15 del 14 ne aveva una sola e ne servivano nove | lo script è lungo, e si legge |
| 84 | un file `w/crlf` riceve inserimenti **CRLF**, e l'Atteso sui fine-riga è *«CR uguale alle righe»*, mai *«zero CR»* | R10-1 e R10-2: due compiti dicevano «LF» di file che `git ls-files --eol` dà `w/crlf`, e il loro criterio «fine-riga invariati» non poteva reggere | una riga `if "\r\n" in text` per inserimento |
| 85 | tre compiti piccoli si correggono in **una** ondata (15, 16, 17), sul precedente di 4+5 e 8+9 | tre cancelli e tre commit per trentasei righe di registro sono costo senza controllo in più | un diff più lungo da leggere per intero |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **UNO SCRIPT DI PATCH DELIMITATO DA `"""` CHE CONTIENE UN BLOCCO PYTHON CON DOCSTRING `"""`** muore di `SyntaxError` alla
  riga della docstring, prima di leggere un byte. 📌 I blocchi che dettano Python si delimitano con tre apici **singoli** — e
  il testo dentro non può contenerli nemmeno fra backtick: questa stessa chiusura è morta così, alla riga che lo spiegava
  (scattato due volte, senza scrivere).
- ⛔ **UN COMPITO «NELLA FORMA DEL COMPITO N» QUANDO N CAMBIA NELLA STESSA SESSIONE** (il `Frame.vue` del 14 dopo D89 del 13): la
  cascata si registra nel `ledger.md` sotto il **consumatore** prima di passare oltre, o si perde fra un'ondata e l'altra.
- ⛔ **«ZERO CR» IN UN ATTESO È UNA PREVISIONE**: si rimisura `git ls-files --eol` e `tr -cd '\r' [|] wc -c` contro `wc -l`
  prima di scriverlo — oggi `gate.sh` 97 = 97, il flusso 16 = 16, l'audit 1885 = 1885, `design/10` 0 = LF.
- ⚠️ **PRE-CONTARE LE ANCORE CON `grep -c -F "$(printf … [|] sed …)"` NON SERVE E SI ROMPE** sulle barre delle ancore: lo script di
  patch asserisce ogni ancora e si ferma senza scrivere, che è il conteggio che conta.
- ⚠️ **UN RILIEVO PUÒ DIRE «SOLO IL 13» DI UN DIFETTO CHE STA ANCHE NEL 14** (R7-6, il `grep` con `**`): lo stesso criterio si
  cerca negli altri compiti prima di chiudere la riga.
- ⛔ **UNA CHIUSURA SCRITTA COME SCRIPT E NON ESEGUITA LASCIA DUE STATI** — RICHIAMO DEL 2026-09-16, alla ripresa: la
  sessione del 15 si è fermata fra la scrittura di `patch_closure15.py` e la sua esecuzione, con `patch_c3.py` applicato e
  non committato; l'albero portava l'ondata 13 senza la chiusura. Chi riprende lo vede con `git diff --stat` (piccolo: solo
  il 3 e il registro) e con `grep -c 'quindicesima chiusura' <questo file>` → 0; le righe della tabella dello stato si
  rimisurano **prima** del commit, e il 2026-09-16 hanno retto tutte. 📌 Uno script di chiusura si esegue nello stesso
  passo che lo scrive, e il commit nello stesso passo del cancello.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, la tabella D fino a
   D89, le voci aperte), poi [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md)
   **per intero**: è la mappa. Di ogni rapporto si legge **la riga** del rilievo (`grep -h '^| R7-1 ' R7-report.md`), non il
   rapporto.
3. ⏭️ **LA REVISIONE IN PROFONDITÀ DEI COMPITI 3, 8, 13, 15, 16, 17** — col metodo di `constraints.md`, rilanciando
   `R6-prompt.md` (13) e `R8-prompt.md` (15, 16, 17) più due mandati piccoli per il 3 e l'8, quando il limite lo permette
   (i subagenti muoiono col limite dell'API: rapporti a pezzi, copiati nel repo). ⛔ **I revisori COMPILANO dove si può**: il
   modulo delle sonde del 12, e i moduli riscritti del 13 e del 14 contro il codice di oggi e un modello dell'11 nello
   scratchpad (come fece R5) — un rosso è un rilievo, non un'errata, perché nessun compito è eseguito. Le correzioni che ne
   escono si applicano come qui: uno script per ondata, `check_after_write.sh`, il diff letto per intero, il cancello, un
   commit per ondata, il registro nello stesso commit.
4. La metà dell'8 di **R9b-5** (la riga ⬜ del registro), con la revisione dell'8.
5. Poi le voci **P-117…** in coda alla sezione del pre-controllo: **una per rilievo «fatto» confermato**, raggruppate per
   compito, col comando che l'ha misurato; i rilievi di sola prosa restano nel registro. I conteggi `P` e `D` si confrontano
   col valore di `HEAD` dopo **ogni** scrittura.
6. ⛔ **Dopo ogni scrittura su questo file**: `check_after_write.sh`, il diff letto, `bash scripts/gate.sh` → `GATE GREEN`, e il
   commit — **senza co-autore**. Per ripartire da `HEAD`: `python <revisione>/restore_from_head.py <piano> <revisione>/ledger.md`.
7. ⏭️ **Poi l'esecuzione, in una sessione NUOVA**: `superpowers:subagent-driven-development`, un subagente fresco per
   compito, con revisione fra uno e l'altro. ⛔ **Prima di eseguire il compito 11 si aggiorna Node** (P-64, P-65: oggi
   `v24.9.0`, fuori da `^24.15.0`), e su una macchina che non l'ha si installa `cargo-audit` (compito 16). Alla chiusura
   del piano la cartella della revisione si **archivia** (decisione 69).
8. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

'''

t = sub(t, POSITION_OLD, POSITION_NEW)
t = sub(t, "## Come si riprende — il diario di questo piano, coi comandi\n\n### La quattordicesima chiusura",
        "## Come si riprende — il diario di questo piano, coi comandi\n\n" + CLOSURE + "### La quattordicesima chiusura")

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: fifteenth closure written;", t.count("\n") - raw.count("\n"), "plan lines added")
