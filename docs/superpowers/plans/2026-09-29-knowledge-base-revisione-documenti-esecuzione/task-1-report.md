# Rapporto del compito 1 — ADR-0040, e ciò che il cancello pretende con lui

Scritto dall'implementatore il 2026-09-30, sulla macchina `jays`. I log veri di ogni passo stanno nella cartella di lavoro
`S` (sezione 1) e qui sono incollati **parola per parola**, non riassunti: dove una sezione dice «uscita», è quella.

## 1. Stato e hash

| | |
|---|---|
| **stato** | **DONE_WITH_CONCERNS** — le riserve sono due imprecisioni di *formulazione* dell'Atteso (sezione 6, candidate ER-11 ed ER-12); nessuna tocca la sostanza. Ogni Atteso di sostanza è tornato, nessun `--check` ha rifiutato, nessuna voce d'errata serviva per andare avanti |
| **commit** | `025dc0f` — intero `025dc0f0b8608b6e429fb2fb5250b7d27d1c4aab`, su `main`, **dieci** file, `129 insertions(+), 13 deletions(-)` |
| **push** | **non fatto**, come dice il contratto: il push è del coordinatore, dopo la revisione. Lo stato del ramo è `## main...origin/main [ahead 1]` |
| **co-autore** | nessuno: nel messaggio di `HEAD` il conteggio di `co-authored` è `0`; il messaggio è quello del Passo 5, identico parola per parola al testo del piano (confrontato con `diff`) |
| **avvio** | `hostname` → `jays`; `git rev-parse --short HEAD` → `8bb8440`; `git status --porcelain` → vuoto; `git config --show-origin --get-all core.autocrlf` → `file:C:/Program Files/Git/etc/gitconfig` e `true` — tutti e tre come dal §0 del dispaccio |
| **giorno** | `D=2026-09-30`, lo stesso in ogni passo, mai ricalcolato |
| **cartella di lavoro `S`** | `C:/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/task1` — attrezzi, pezzi, log, messaggio del commit; **niente nel repository**. Questo rapporto sta in una cartella ignorata (`git check-ignore -v` → `.superpowers/sdd/.gitignore:1:*`) |

Letture fatte, come dal §1 del dispaccio: il brief per intero (1132 righe, a blocchi), **ADR-0022 per intero** (centodiciotto
righe) e **la testa di ADR-0001** fino a `## Context`. Non ho letto il piano intero, il disegno, `HANDOFF.md`, gli altri ADR, l'archivio né l'audit.

## 2. `git show --stat HEAD`

````text
commit 025dc0f0b8608b6e429fb2fb5250b7d27d1c4aab
Author: devfrx <zagor2012@icloud.com>
Date:   Wed Sep 30 11:01:17 2026 +0200

    knowledge-base-revisione(compito 1): ADR-0040 — dove vivono i dati, e che cosa salva il programma —, che modifica in parte ADR-0022; il rimando in testa a 0022, la riga dell'indice, la voce nella §5 del compendio col rimando nella voce di 0022, i totali a quaranta e la cifra tolta da AVVIO-CHAT col richiamo, il caso nuovo nella §13 e in CLAUDE.md, la riga della data con la sua copia in archivio

 CLAUDE.md                                                               |  2 +-
 docs/AVVIO-CHAT.md                                                      |  4 +++-
 docs/COMPENDIO.md                                                       | 19 +++++++++++++++----
 docs/HANDOFF.md                                                         |  8 ++++----
 docs/README.md                                                          |  1 +
 docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md         | 14 ++++++++++++++
 docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md       | 82 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 docs/archivio/stato-storico.md                                          |  6 ++++++
 docs/roadmap.md                                                         |  4 ++--
 docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md |  2 +-
 10 files changed, 129 insertions(+), 13 deletions(-)
````

## 3. Passo per passo

Ogni comando è quello del compito; dove ne ho aggiunto uno, è una **misura di sola lettura** e lo dico. Le sonde che devono
rendere `0` le ho provate prima dove rendono `1` (vincolo 14).

**Atteso contro misurato, in breve** — le uscite vere, riga per riga, sono nelle sottosezioni che seguono. «Torna» vuol dire
che il valore misurato è quello dell'Atteso; dove c'è una nota, la sezione 6 la spiega.

| Passo | Atteso (dispaccio e brief) | Misurato | Esito |
|---|---|---|---|
| 1 | `check-docs.sh` → `OK` | `OK` | torna |
| 1 | otto file `i/lf w/crlf`, CR uguali alle righe | idem: 118, 226, 919, 123, 1408, 345, 542, 2686 | torna |
| 1 | tabelle: la sola riga `31 docs/HANDOFF.md` | idem | torna |
| 1 | margine positivo (`8103`) | `8103` | torna |
| 1 | totali: otto righe, tutte a trentanove | otto righe, tutte a 39 | torna |
| 1 | `0` file di ADR-0040; `1` rimando in ADR-0022 | `0`; `1` | torna |
| 1 | comando E `0 0 1 0 1 0 0 8`, nell'ordine di `F` (ER-9: l'archivio si annota) | `0 0 1 0 1 0 0 8`, archivio `8` | torna |
| 1 | ER-4: `0` sul compendio; `superato in parte`: `0` e `0` | `0`; `0` e `0` (controprove su `e1.txt`: `1` e `2`) | torna |
| 2 | `0` segnaposto; la riga `Date:` col giorno; `archived: 1 piece(s) under «…»` | `0`; `4:- **Date:** 2026-09-30`; `archived: 1 piece(s) under «…»` | torna |
| 3 | `--check` di E1: `checked: 17 edits in 7 files`; di POS1: `checked: 1 edits in 1 files` | idem | torna |
| 3 | `applied: 17 edits in 7 files`, poi `applied: 1 edits in 1 files` | idem | torna |
| 4 | `check-docs.sh` → `OK`; cancello → `GATE GREEN` | `OK`; `GATE GREEN.` con `EXIT=0` | torna |
| 4 | colonna `w/…` e CR come al Passo 1; ADR-0040 `w/lf` | `crlf` per gli otto, CR uguali alle righe; ADR-0040 `w/lf`, CR 0 | torna, con nota (ER-11 ed ER-12) |
| 4 | tabelle come al Passo 1 | la sola riga `31 docs/HANDOFF.md` | torna |
| 4 | margine più piccolo di circa 1,6 KB, positivo (`6483`) | `6483`, cioè 1620 byte in meno | torna |
| 4 | totali: sette righe, tutte a quaranta; comando B: niente | sette righe, tutte a 40; niente | torna |
| 4 | `Accepted`, `Modifica:`, `Negative (accettate)`: `1 1 1`; `modificato da ADR-0040`: `1`; `superato in parte`: `1` e `1`; ER-4: `1` | `1 1 1`; `1`; `1` e `1`; `1` | torna |
| 4 | `2` rimandi in ADR-0022; `0` righe tolte negli ADR | `2`; `0` (controprova sul diff del compendio: `4`) | torna |
| 4 | niente fuori dai documenti | vuoto; e `git status --porcelain` intero: i dieci percorsi | torna |
| 4 | comando E: `1` in più su ADR-0022 e AVVIO-CHAT, `1` su ADR-0040, archivio uguale (ER-9) | `1 0 1 0 1 0 1 8`; ADR-0040 `1` | torna |
| 4 | `git diff --stat`: gli otto file di `F` e il piano | nove file, `47 insertions(+), 13 deletions(-)` | torna |
| 4 | sonda di ER-10: `SAME` | `SAME` (controprova: `DIFFERENT`) | torna |
| 5 | un commit, dieci file, senza co-autore, senza push | `025dc0f`, dieci file, co-autori `0`, `ahead 1` | torna |

### 3.0 Prima del Passo 1 — l'avvio e `extract.py`

**Avvio** — vedi la riga «avvio» della sezione 1: i tre valori tornano.

**`extract.py` copiato a mano** dal recinto del brief (righe 74–126) col tool di scrittura, in `S/extract.py`, poi:

````text
# confrontato col suo recinto NEL PIANO (estratto in sola lettura con awk, senza barre rovesciate) e con quello del brief
diff "$S/extract.py" "$S/extract-from-plan.txt"    -> IDENTICAL to plan recinto      (53 righe contro 53)
diff "$S/extract.py" "$S/extract-from-brief.txt"   -> IDENTICAL to brief recinto     (righe 74-126 del brief)
conto delle barre rovesciate in extract.py (grep -cF, la barra scritta col codice ottale)   -> 0
conto dei CR in extract.py (grep -c sul carattere CR)                                        -> 0    (il file nasce LF)
PYTHONIOENCODING=utf-8 python "$S/extract.py" docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md "$S"
````

Uscita, **diciotto righe**, una per pezzo (`EXIT=0`, Python 3.13.7):

````text
apply_edits.py: 167 lines
archive_head.py: 71 lines
replace_pointer.py: 35 lines
tables.awk: 5 lines
adr0040.md: 82 lines
e1.txt: 67 lines
e2.txt: 144 lines
e3.txt: 154 lines
e4.txt: 93 lines
e5.txt: 68 lines
e6.txt: 17 lines
pos1.txt: 4 lines
pos2.txt: 4 lines
pos3.txt: 4 lines
pos4.txt: 4 lines
pos5.txt: 4 lines
pos6.txt: 6 lines
pointer6.txt: 10 lines
````

**Controllo in più, di sola lettura:** i sei pezzi che questo compito usa — `apply_edits.py`, `archive_head.py`, `tables.awk`,
`adr0040.md`, `e1.txt`, `pos1.txt` — sono **identici** ai recinti del brief (righe 132–298, 304–374, 424–428, 779–860,
878–944, 950–953) e hanno **zero** CR ciascuno. Il piano di `HEAD` e il brief non divergono.

**Lettura di ADR-0022 contro il blocco E1** (per il revisore, gotcha #59): le citazioni del rimando di E1 stanno tutte in
ADR-0022 come scritte — le righe «artefatti» e «configurazione, guide, profili» della tabella del punto 1 della *Decision*;
la conseguenza *«La base di conoscenza sopravvive alla reinstallazione perché i documenti sorgente e la configurazione
sono nel backup»* (a capo dopo «sorgente», righe 89–90); il *«non cambia»* del rimando del 2026-09-08; il *«dopo il 5, il 6
e il 9»* del rimando del 2026-08-07 e la sua ragione, la non-vacuità (righe 108 e 110–113); `## Context` compare **una**
volta, quindi `^^ ## Context` è univoco; la frase *«Nessuna riga di questo ADR è superata»* resta dov'è, e il rimando nuovo
le va **sotto** (D5).

### 3.1 Passo 1 — le misure prima

Comandi: quelli del brief (righe 745–754), con `S` e `D` in testa e `F` come nel compito; in più, di sola lettura, il conto
delle righe per file (per confrontarlo coi CR) e le sonde di ER-4 con la loro controprova su `e1.txt`. **Uscita vera:**

````text
##### STEP 1 -- HEAD=8bb8440 D=2026-09-30
### check-docs.sh
== internal links ==
== ADR: files vs index ==
== diagrams: files vs index ==
== section numbering: duplicates ==
== every Q requirement has a verification method (V30) ==
== catalogue §7.4: every check defends something (rule 1) and has its counter-probe (rule 3) ==
== §8: every V and every Q has a state, and the deferred ones have their trigger ==
== compendium §5: one entry per ADR, and none too many ==
== ADR counts declared in the prose ==
== ADR still in Proposed ==
  (none)
== compendium size ceiling ==

OK — no inconsistencies.
check-docs exit=0
### git ls-files --eol
i/lf    w/crlf  attr/                 	CLAUDE.md
i/lf    w/crlf  attr/                 	docs/AVVIO-CHAT.md
i/lf    w/crlf  attr/                 	docs/COMPENDIO.md
i/lf    w/crlf  attr/                 	docs/HANDOFF.md
i/lf    w/crlf  attr/                 	docs/README.md
i/lf    w/crlf  attr/                 	docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
i/lf    w/crlf  attr/                 	docs/archivio/stato-storico.md
i/lf    w/crlf  attr/                 	docs/roadmap.md
### CR counts
docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md CR=118
docs/README.md CR=226
docs/COMPENDIO.md CR=919
CLAUDE.md CR=123
docs/HANDOFF.md CR=1408
docs/roadmap.md CR=345
docs/AVVIO-CHAT.md CR=542
docs/archivio/stato-storico.md CR=2686
### line counts (to compare CR with lines)
docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md LINES=118
docs/README.md LINES=226
docs/COMPENDIO.md LINES=919
CLAUDE.md LINES=123
docs/HANDOFF.md LINES=1408
docs/roadmap.md LINES=345
docs/AVVIO-CHAT.md LINES=542
docs/archivio/stato-storico.md LINES=2686
### tables.awk per-file count
     31 docs/HANDOFF.md
### margin (command C)
8103
### ADR totals (guard command)
docs/HANDOFF.md:208:quattro pilastri paritari** su kernel comune. Spec del kernel **§0–§10 completa, 39 ADR**.
docs/HANDOFF.md:1056:39 ADR in stato . Rimetterne in discussione uno **richiede un ADR
docs/HANDOFF.md:1272:| ❌ ri-derivare l'architettura | è in **39 ADR**, ciascuno con alternative scartate e motivo |
docs/HANDOFF.md:1317:| [](adr/) | **39 decisioni architetturali**. Leggi **0001** e **0004** per primi: tutto il resto ne discende. Poi **0026** (linguaggio) se devi scrivere codice |
docs/roadmap.md:23:> Spec del kernel **completa e approvata** (§0–§10, 39 ADR). Stack deciso **per intero**: core in **Rust**,
docs/COMPENDIO.md:135:Sono **39 ADR**, e **39 ADR in stato Accepted** — l'ultimo, 0029, chiuso il 2026-09-10 con SP-8.
docs/COMPENDIO.md:741:| ❌ **ri-derivare l'architettura** | è nei 39 ADR, ciascuno con alternative scartate e motivo |
docs/AVVIO-CHAT.md:163:  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 39 ADR
### ls docs/adr/0040-* | wc -l
0
### 'Rimando del' occurrences in ADR-0022
1
### command E per file (order of F)
docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md:0
docs/README.md:0
docs/COMPENDIO.md:1
CLAUDE.md:0
docs/HANDOFF.md:1
docs/roadmap.md:0
docs/AVVIO-CHAT.md:0
docs/archivio/stato-storico.md:8
### ER-4 probe, direction 0 (on the compendium, before)
modificato in parte da ADR-0040 in COMPENDIO: 0
### ER-4 probe, direction 1 (same pattern on the block e1.txt: must be 1, so the probe is not vacuous)
modificato in parte da ADR-0040 in e1.txt: 1
### superato in parte, CLAUDE.md and COMPENDIO (before: 0 and 0)
CLAUDE.md:0
docs/COMPENDIO.md:0
### same pattern on e1.txt (must be >=1)
2
### git status --porcelain (must be empty)
### pre-flight --check (writes nothing)
checked: 17 edits in 7 files
checked: 1 edits in 1 files
### git status --porcelain after the pre-flight (must still be empty)
````

**Annotazioni del Passo 1** (sono l'invariante del Passo 5): la colonna `w/…` è `crlf` per tutti e otto i file di `F`, con
`i/lf`; i CR sono **uguali alle righe** per ciascuno — 118, 226, 919, 123, 1408, 345, 542, 2686, nell'ordine di `F` —; il
comando E sull'archivio rende **`8`**, come sul commit del pre-dispaccio (ER-9: si annota, e al Passo 4 dev'essere lo
stesso). Tutto il resto come da §3 del dispaccio: `OK`; tabelle `31 docs/HANDOFF.md`; margine `8103`; otto totali a
trentanove; `0` file di ADR-0040; `1` rimando in ADR-0022; comando E `0 0 1 0 1 0 0 8`; ER-4: `0` (e `1` su `e1.txt`);
`superato in parte`: `0` e `0` (e `2` righe su `e1.txt`).

**Pre-volo, aggiunto da me** (in coda al log qui sopra): prima di scrivere qualsiasi cosa ho lanciato i due `--check`, che non
scrivono — `checked: 17 edits in 7 files` e `checked: 1 edits in 1 files` — con `git status --porcelain` vuoto prima e
dopo. Ragione: un rifiuto al Passo 3 avrebbe lasciato scritti i due effetti del Passo 2 (ADR-0040 e la riga in archivio).
L'ordine delle **scritture** è rimasto quello del compito: Passo 2, poi E1, poi POS1.

### 3.2 Passo 2 — ADR-0040 dal suo recinto, e la riga della data in archivio, prima del blocco

Comandi: quelli del brief (righe 765–769); in più, la controprova della sonda `<data>` sul recinto grezzo (deve rendere `1`)
e il conto delle righe `**Aggiornato il ` del compendio (deve essere una). **Uscita vera:**

````text
##### STEP 2 -- D=2026-09-30
### control: '<data>' in the raw recinto must be 1 line (probe not vacuous)
1
### the date line of the compendium, as it is now (the one archive_head.py will archive)
1
### create ADR-0040 with sed (stream, not -i)
### grep -c '<data>' in ADR-0040 (expected 0)
0
### grep -n 'Date:' (expected the Date line with the day)
4:- **Date:** 2026-09-30
### archive_head.py (BEFORE the block)
archived: 1 piece(s) under «L'intestazione del compendio, com'era — archiviata il 2026-09-30, al compito 1 del piano dei documenti della revisione della knowledge base»; the heading starts with «L'intestazione del compendio»
archive_head exit=0
````

**Ispezione subito dopo** (sola lettura): ADR-0040 è di `82` righe e `7446` byte — `7442` del recinto più `4`, perché `<data>`
(sei caratteri) è diventato `2026-09-30` (dieci) —, con `0` CR; l'archivio è passato da `2686` a `2692` righe (sei: una vuota,
l'intestazione, una vuota, l'avvertenza, una vuota, la riga della data), con CR `2692` = righe, quindi CRLF conservato; in
coda porta l'intestazione con il trattino lungo e l'apostrofo intatti e la riga della data **di prima** col link riscritto
(`](stato-storico.md)`). `git status --porcelain` in quel momento: ` M docs/archivio/stato-storico.md` e
`?? docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md`.

### 3.3 Passo 3 — il blocco E1, prima il `--check`, poi davvero; e la posizione

Comandi: quelli del brief (righe 867–869), concatenati con `&&` perché un rifiuto fermasse tutto. **Uscita vera:**

````text
##### STEP 3 -- D=2026-09-30
checked: 17 edits in 7 files
applied: 17 edits in 7 files
applied: 1 edits in 1 files
steps exit=0
### git status --porcelain
 M CLAUDE.md
 M docs/AVVIO-CHAT.md
 M docs/COMPENDIO.md
 M docs/HANDOFF.md
 M docs/README.md
 M docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
 M docs/archivio/stato-storico.md
 M docs/roadmap.md
 M docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
?? docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
````

Il blocco non ha toccato altro: ` M` su otto file di `F` più il piano, e `??` su ADR-0040 — esattamente i dieci percorsi del
`git add` del Passo 5.

### 3.4 Passo 4 — le prove

**`check-docs.sh`, da solo, prima:**

````text
##### STEP 4 -- check-docs.sh
== internal links ==
== ADR: files vs index ==
== diagrams: files vs index ==
== section numbering: duplicates ==
== every Q requirement has a verification method (V30) ==
== catalogue §7.4: every check defends something (rule 1) and has its counter-probe (rule 3) ==
== §8: every V and every Q has a state, and the deferred ones have their trigger ==
== compendium §5: one entry per ADR, and none too many ==
== ADR counts declared in the prose ==
== ADR still in Proposed ==
  (none)
== compendium size ceiling ==

OK — no inconsistencies.
check-docs exit=0
````

**Il cancello** — dopo `check-docs.sh`, **da solo**, in background, senza `&` (sezione 5: `GATE GREEN.`, `EXIT=0`).

**Le prove** (comandi del brief, righe 960–976, in più le sonde di ER-4 e ER-9 e le controprove). **Uscita vera:**

````text
##### STEP 4 (after the blocks) -- HEAD=8bb8440
### gate verdict
GATE GREEN.
EXIT=0
### git ls-files --eol (tracked; ADR-0040 is untracked, so absent here)
i/lf    w/crlf  attr/                 	CLAUDE.md
i/lf    w/crlf  attr/                 	docs/AVVIO-CHAT.md
i/lf    w/crlf  attr/                 	docs/COMPENDIO.md
i/lf    w/crlf  attr/                 	docs/HANDOFF.md
i/lf    w/crlf  attr/                 	docs/README.md
i/lf    w/crlf  attr/                 	docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
i/lf    w/crlf  attr/                 	docs/archivio/stato-storico.md
i/lf    w/crlf  attr/                 	docs/roadmap.md
### git ls-files --eol for ADR-0040 as an untracked file
i/      w/lf    attr/                 	docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
### CR counts / LINES (must be equal for the eight files; 0 CR for ADR-0040)
docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md CR=132 LINES=132
docs/README.md CR=227 LINES=227
docs/COMPENDIO.md CR=930 LINES=930
CLAUDE.md CR=123 LINES=123
docs/HANDOFF.md CR=1408 LINES=1408
docs/roadmap.md CR=345 LINES=345
docs/AVVIO-CHAT.md CR=544 LINES=544
docs/archivio/stato-storico.md CR=2692 LINES=2692
docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md CR=0 LINES=82
### tables.awk per-file count (with ADR-0040 added)
     31 docs/HANDOFF.md
### margin (command C)
6483
### ADR totals (guard command; expected seven lines, all at forty)
docs/HANDOFF.md:208:quattro pilastri paritari** su kernel comune. Spec del kernel **§0–§10 completa, 40 ADR**.
docs/HANDOFF.md:1056:40 ADR in stato . Rimetterne in discussione uno **richiede un ADR
docs/HANDOFF.md:1272:| ❌ ri-derivare l'architettura | è in **40 ADR**, ciascuno con alternative scartate e motivo |
docs/HANDOFF.md:1317:| [](adr/) | **40 decisioni architetturali**. Leggi **0001** e **0004** per primi: tutto il resto ne discende. Poi **0026** (linguaggio) se devi scrivere codice |
docs/roadmap.md:23:> Spec del kernel **completa e approvata** (§0–§10, 40 ADR). Stack deciso **per intero**: core in **Rust**,
docs/COMPENDIO.md:135:Sono **40 ADR**, e **40 ADR in stato Accepted** — l'ultimo, 0040, il 2026-09-30, dalla revisione della knowledge base.
docs/COMPENDIO.md:751:| ❌ **ri-derivare l'architettura** | è nei 40 ADR, ciascuno con alternative scartate e motivo |
### command B on AVVIO-CHAT (expected: nothing)
### ADR-0040: Accepted / Modifica / Negative (expected 1 1 1)
1
1
1
### modificato da ADR-0040 in ADR-0022 (expected 1)
1
### superato in parte in CLAUDE.md and COMPENDIO (expected 1 and 1)
CLAUDE.md:1
docs/COMPENDIO.md:1
### ER-4: modificato in parte da ADR-0040 in COMPENDIO (expected 1)
1
### 'Rimando del' occurrences in ADR-0022 (expected 2)
2
### lines removed in the ADR diff (expected 0), and the control on the compendium diff (must be > 0)
0
4
### code untouched: git status --porcelain on crates/ gui/ scripts/ Cargo.lock Cargo.toml (expected empty)
### control of that probe: the same command on docs/ (must list the 10 paths)
9
### whole git status --porcelain (the gate must not have left anything else)
 M CLAUDE.md
 M docs/AVVIO-CHAT.md
 M docs/COMPENDIO.md
 M docs/HANDOFF.md
 M docs/README.md
 M docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
 M docs/archivio/stato-storico.md
 M docs/roadmap.md
 M docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
?? docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
### command E per file (order of F, then ADR-0040)
docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md:1
docs/README.md:0
docs/COMPENDIO.md:1
CLAUDE.md:0
docs/HANDOFF.md:1
docs/roadmap.md:0
docs/AVVIO-CHAT.md:1
docs/archivio/stato-storico.md:8
docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md:1
### git diff --stat
 CLAUDE.md                                             |  2 +-
 docs/AVVIO-CHAT.md                                    |  4 +++-
 docs/COMPENDIO.md                                     | 19 +++++++++++++++----
 docs/HANDOFF.md                                       |  8 ++++----
 docs/README.md                                        |  1 +
 ...-layout-dei-dati-per-natura-e-backup-dichiarato.md | 14 ++++++++++++++
 docs/archivio/stato-storico.md                        |  6 ++++++
 docs/roadmap.md                                       |  4 ++--
 .../2026-09-29-knowledge-base-revisione-documenti.md  |  2 +-
 9 files changed, 47 insertions(+), 13 deletions(-)
````

**La sonda di ER-10**, prima del commit, con `HEAD` ancora `8bb8440` — comandi del brief (righe 561–562) — e la controprova
che la sonda **può scattare** (confronta l'archivio con la riga NUOVA del compendio: deve rendere `DIFFERENT`). **Uscita vera:**

````text
### ER-10 probe (HEAD is still 8bb8440, the commit before the task)
SAME
### control 1: the old-date file holds exactly one line
1
### control 2: the same comparison against the NEW date line of the compendium must be DIFFERENT (the probe can fire)
1
DIFFERENT
### the last line of the archive that says Aggiornato (cut to 160 chars) and the new date line of the compendium (cut to 160)
**Aggiornato il 2026-09-30**, con la **prima decisione della mappa del metodo**: il piano dei documenti si esegue adesso — il puntatore della §6 —; il punt
**Aggiornato il 2026-09-30**, col **piano dei documenti della revisione della knowledge base** in esecuzione: **ADR-0040** — la sua voce in §5, il rimando ne
````

### 3.5 Passo 5 — il commit (senza push)

`git add` dei **dieci** file del Passo 5 e nessun altro; verifica dell'insieme in stage; poi `git commit -F "$S/commit-msg.txt"`.
Il messaggio è il testo del Passo 5 — la sola riga, senza co-autore —, scritto col tool di scrittura in un file di `S` e
confrontato con `diff` contro la riga 990 del brief: identico. **Niente** `git push`, `--amend`, rebase, `--no-verify`
(non ci sono hook: `core.hooksPath` vuoto, nessun hook non-`.sample` in `.git/hooks`). **Uscite vere:**

````text
##### STEP 5 -- staging
warning: in the working copy of 'docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md', LF will be replaced by CRLF the next time Git touches it
add exit=0
### staged name-status (must be exactly 10: A for ADR-0040, M for nine)
M	CLAUDE.md
M	docs/AVVIO-CHAT.md
M	docs/COMPENDIO.md
M	docs/HANDOFF.md
M	docs/README.md
M	docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
A	docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
M	docs/archivio/stato-storico.md
M	docs/roadmap.md
M	docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
staged count: 10
### anything unstaged or untracked left? (must be empty)
### eol of ADR-0040 now that it is in the index
i/lf    w/lf    attr/                 	docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
### message file has no trailer:
0
````
````text
[main 025dc0f] knowledge-base-revisione(compito 1): ADR-0040 — dove vivono i dati, e che cosa salva il programma —, che modifica in parte ADR-0022; il rimando in testa a 0022, la riga dell'indice, la voce nella §5 del compendio col rimando nella voce di 0022, i totali a quaranta e la cifra tolta da AVVIO-CHAT col richiamo, il caso nuovo nella §13 e in CLAUDE.md, la riga della data con la sua copia in archivio
 10 files changed, 129 insertions(+), 13 deletions(-)
 create mode 100644 docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
````

**Dopo il commit** (sola lettura):

````text
##### AFTER THE COMMIT
### git show --stat HEAD
025dc0f0b8608b6e429fb2fb5250b7d27d1c4aab
devfrx <zagor2012@icloud.com>
knowledge-base-revisione(compito 1): ADR-0040 — dove vivono i dati, e che cosa salva il programma —, che modifica in parte ADR-0022; il rimando in testa a 0022, la riga dell'indice, la voce nella §5 del compendio col rimando nella voce di 0022, i totali a

 CLAUDE.md                                          |  2 +-
 docs/AVVIO-CHAT.md                                 |  4 +-
 docs/COMPENDIO.md                                  | 19 +++--
 docs/HANDOFF.md                                    |  8 +--
 docs/README.md                                     |  1 +
 ...yout-dei-dati-per-natura-e-backup-dichiarato.md | 14 ++++
 ...-vivono-i-dati-e-che-cosa-salva-il-programma.md | 82 ++++++++++++++++++++++
 docs/archivio/stato-storico.md                     |  6 ++
 docs/roadmap.md                                    |  4 +-
 ...026-09-29-knowledge-base-revisione-documenti.md |  2 +-
 10 files changed, 129 insertions(+), 13 deletions(-)
### git ls-files --eol of the ten files, as committed
i/lf    w/crlf  attr/                 	CLAUDE.md
i/lf    w/crlf  attr/                 	docs/AVVIO-CHAT.md
i/lf    w/crlf  attr/                 	docs/COMPENDIO.md
i/lf    w/crlf  attr/                 	docs/HANDOFF.md
i/lf    w/crlf  attr/                 	docs/README.md
i/lf    w/crlf  attr/                 	docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
i/lf    w/lf    attr/                 	docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
i/lf    w/crlf  attr/                 	docs/archivio/stato-storico.md
i/lf    w/crlf  attr/                 	docs/roadmap.md
i/lf    w/crlf  attr/                 	docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
### CR / LINES of the ten files in the working tree
docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md CR=0 LINES=82
docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md CR=132 LINES=132
docs/README.md CR=227 LINES=227
docs/COMPENDIO.md CR=930 LINES=930
CLAUDE.md CR=123 LINES=123
docs/HANDOFF.md CR=1408 LINES=1408
docs/roadmap.md CR=345 LINES=345
docs/AVVIO-CHAT.md CR=544 LINES=544
docs/archivio/stato-storico.md CR=2692 LINES=2692
docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md CR=2061 LINES=2061
### branch state (no push made)
## main...origin/main [ahead 1]
025dc0f knowledge-base-revisione(compito 1): ADR-0040 — dove vivono i dati, e che cosa salva il programma —, che modifica in parte ADR-0
8bb8440 docs(knowledge base, revisione): il pre-dispaccio del compito 1 -- ER-9 ed ER-10 nell'errata del piano dei documenti, e la cartella 
dd0e265 docs(metodo, decision map): la chiusura della sessione dei primi due ticket -- sulla mappa chiusi «Il piano dei documenti si esegue
### check-docs.sh on the committed tree
  (none)
== compendium size ceiling ==

OK — no inconsistencies.
````
````text
### plan file: lines in the blob at HEAD~1 vs HEAD (POS1 replaces one line with one line)
HEAD~1: 2061
HEAD:   2061
### committed diff of ADR-0022: removed lines (must be 0) and added lines
0
13
### archive: command E at HEAD~1 and at HEAD (ER-9: must be the same)
HEAD~1: 8
HEAD:   8
### the two claims ADR-0040 makes about crates/daemon/src/main.rs (read-only grep)
184:/// ADR has taken, and inventing one here would be that decision taken by whoever typed the
196:/// reason: where a per-user data directory belongs is a decision no ADR has taken, and inventing one
186:const JOURNAL_PATH: &str = "journal.redb";
195:/// ⚠️ RELATIVE TO THE WORKING DIRECTORY, exactly as `JOURNAL_PATH` is and declared for the same
198:const LAYOUT_PATH: &str = "layout.redb";
696:    match run_the_production_graph(Path::new(JOURNAL_PATH), Path::new(LAYOUT_PATH)) {
717:                "the journal at {JOURNAL_PATH} would not open: {error:?}"
733:                "the journal at {JOURNAL_PATH} would not say which VRAM policy is in force: {error:?}"
183:/// declared rather than defended: where a per-user data directory should be is a decision no
196:/// reason: where a per-user data directory belongs is a decision no ADR has taken, and inventing one
````

L'ultima parte di `after-commit-2.log` — i tre `grep` su `crates/daemon/src/main.rs` — è la **lettura dei due commenti del
sorgente** (solo `grep`, nessuna modifica al codice): la frase citata da ADR-0040 e i due percorsi relativi ci sono. È lo stato che ER-3 già registra —
dal commit di ADR-0040 quei commenti dicono il falso —, e non c'è niente da fare in questo compito.

## 4. I fine-riga, per ogni file toccato

Misurati col `git ls-files --eol` e col conto dei CR (`tr -cd '\r' < file | wc -c`) — **prima** = Passo 1, **dopo** = Passo 4
e, per i file tracciati, di nuovo dopo il commit. In ogni riga: colonne `i/…` e `w/…`, CR, righe.

| File | prima | dopo |
|---|---|---|
| `docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md` | `i/lf w/crlf`, CR 118, righe 118 | `i/lf w/crlf`, CR 132, righe 132 |
| `docs/README.md` | `i/lf w/crlf`, CR 226, righe 226 | `i/lf w/crlf`, CR 227, righe 227 |
| `docs/COMPENDIO.md` | `i/lf w/crlf`, CR 919, righe 919 | `i/lf w/crlf`, CR 930, righe 930 |
| `CLAUDE.md` | `i/lf w/crlf`, CR 123, righe 123 | `i/lf w/crlf`, CR 123, righe 123 |
| `docs/HANDOFF.md` | `i/lf w/crlf`, CR 1408, righe 1408 | `i/lf w/crlf`, CR 1408, righe 1408 |
| `docs/roadmap.md` | `i/lf w/crlf`, CR 345, righe 345 | `i/lf w/crlf`, CR 345, righe 345 |
| `docs/AVVIO-CHAT.md` | `i/lf w/crlf`, CR 542, righe 542 | `i/lf w/crlf`, CR 544, righe 544 |
| `docs/archivio/stato-storico.md` | `i/lf w/crlf`, CR 2686, righe 2686 | `i/lf w/crlf`, CR 2692, righe 2692 |
| `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md` | `i/lf w/crlf`; CR **non contati** (non è in `F`); righe 2061 (blob di `HEAD~1`) | `i/lf w/crlf`, CR 2061, righe 2061 — la riga di POS1 ne sostituisce una con una |
| `docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md` | non esiste | prima del `git add`: `i/` vuota, `w/lf` (con `git ls-files --eol -o --exclude-standard`); dopo: `i/lf w/lf`; CR 0, righe 82 |

**Nessuna deriva**: in ciascuno degli otto file di `F` e nel piano i CR sono ancora **uguali alle righe**, la colonna `w/…` è
quella di prima, e ADR-0040 è `w/lf`. Il numero assoluto dei CR cambia, com'è ovvio, nei file che hanno guadagnato righe:
vedi ER-11 nella sezione 6. Il `warning: in the working copy of 'docs/adr/0040-…', LF will be replaced by CRLF the next time
Git touches it` del `git add` è quello atteso con `core.autocrlf=true`: il file resta `w/lf`, misurato dopo il commit.

## 5. Il cancello

| | |
|---|---|
| **comando** | `bash scripts/gate.sh > "$S/gate-2026-09-30.log" 2>&1; echo "EXIT=$?" >> "$S/gate-2026-09-30.log"`, con `run_in_background` del tool Bash, **senza `&`**, **da solo** — mentre girava ho letto soltanto file e scritto il messaggio del commit, fuori dal repository: nessun comando toccava `gui/` né `git` |
| **verdetto** | `GATE GREEN.` e `EXIT=0`, dal `grep` prescritto dal dispaccio, riportato con la sua uscita sotto la tabella |
| **quando** | sull'albero **con i blocchi applicati e prima del commit**; inizio `10:54:42`, fine `10:59:20` — quattro minuti e trentotto secondi |
| **log** | `C:/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/task1/gate-2026-09-30.log` — 1096 righe, 56375 byte |
| **dopo il cancello** | `git status --porcelain` elenca **solo** i dieci percorsi del compito: il cancello non ha lasciato niente |

Il verdetto, col `grep` del dispaccio, lanciato sul log — uscita vera:

````text
grep -E 'GATE GREEN|GATE RED|EXIT=' "$S/gate-2026-09-30.log"
GATE GREEN.
EXIT=0
````

Le intestazioni dei passi del cancello e il fondo del log:

````text
######## workspace build
######## example and compile-fail tests
######## no-OS gate
######## allow-list on the two graphs
######## dependency advisories
######## attributes of the constrained crates
######## gui: fake core and SPA
######## documentation consistency
######## DST campaigns -- wall time
...

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.70s


GATE GREEN.
EXIT=0
````

Dentro il cancello, il passo *documentation consistency* ha stampato `OK — no inconsistencies.` (riga 1015 del log), e
`check-docs.sh` lanciato da solo dopo il commit ha ridato `OK` (in `after-commit.log`).

## 6. Le divergenze, e ciò che non ho potuto misurare

**Nessun `--check` ha rifiutato e nessun Atteso di sostanza è mancato:** non serve nessuna voce d'errata per proseguire, e il
compito non è `BLOCKED`. Ho trovato due imprecisioni di **formulazione**, che propongo come voci candidate perché i compiti
2–6 usano le stesse parole:

| Candidata | Dove | Che cosa dice il piano | Che cosa ho misurato | Proposta |
|---|---|---|---|---|
| **ER-11** | vincolo 7; Atteso del Passo 4 — *«la colonna `w/…` e i CR come al Passo 1»*; §4 del dispaccio — *«i CR sono quanti prima»* | che dopo il compito i CR siano **gli stessi** di prima | alla lettera vale solo per i file che **non** hanno cambiato numero di righe — `CLAUDE.md` 123, `HANDOFF.md` 1408, `roadmap.md` 345. Per gli altri il numero cambia: ADR-0022 `118` → `132`, `README.md` `226` → `227`, compendio `919` → `930`, `AVVIO-CHAT.md` `542` → `544`, archivio `2686` → `2692`. **Ciò che regge** — e che ho verificato, prima e dopo — è che i CR restano **uguali alle righe** di ciascun file (nessun fine-riga misto) e la colonna `w/…` non cambia | scrivere l'invariante come *«i CR restano uguali alle righe»*, che è quello che il §0 e il §3 del dispaccio già dicono. Comandi: sezione 4 e `step1.log`, `step4.log` |
| **ER-12** | Passo 4 — `git ls-files --eol $F docs/adr/0040-*.md` e l'Atteso *«ADR-0040 `w/lf`»* | che il comando mostri la riga di ADR-0040 con `w/lf` | finché il file è **non tracciato** `git ls-files` non lo elenca: nell'uscita del Passo 4 ci sono otto righe e non nove. Si misura con `git ls-files --eol -o --exclude-standard docs/adr/0040-*.md` → `i/      w/lf`, o con lo stesso comando senza `-o` **dopo** il `git add` → `i/lf    w/lf` | dire quale dei due; oppure spostare la misura dopo il `git add` del Passo 5, prima del `git commit`. Comandi: `step4.log` e `step5-stage.log` |

**Cose che ho fatto oltre la lettera del contratto, tutte di sola lettura, dette qui perché il revisore le conosca:** il pre-volo
dei due `--check` prima del Passo 2; le controprove delle sonde (`e1.txt` per ER-4 e `superato in parte`, il recinto grezzo
per `<data>`, la riga NUOVA del compendio per ER-10, il diff del compendio per «righe tolte negli ADR»); il confronto dei pezzi
estratti coi recinti del brief; il `grep` sul sorgente di `crates/daemon/src/main.rs` per due affermazioni di ADR-0040. Nessuna
di queste ha scritto niente nel repository.

**Un mio scivolone, senza effetto:** nell'uscita del Passo 4 la controprova della sonda «niente fuori dai documenti» porta
l'etichetta *«must list the 10 paths»* ma rende `9`: sotto `docs/` i percorsi sono nove — il decimo è `CLAUDE.md`, alla radice.
La sonda non è vacua (rende un numero diverso da zero), ed è ciò che la controprova doveva mostrare.

**Ciò che non ho potuto misurare, o che non è mio:**

- **i CR del piano prima del compito**: il piano non è in `F`, e non li ho contati al Passo 1. Ho `i/lf w/crlf` e le righe del
  blob di `HEAD~1` (2061); dopo, CR 2061 = righe 2061. Che prima fossero 2061 è dedotto dal fatto che `apply_edits.py`
  conserva il fine-riga di ciascun file e che POS1 sostituisce una riga con una riga.
- **il rilievo di revisione** del Criterio di chiusura — ADR-0040 riletto contro la 3.2 e la 3.3, il rimando di ADR-0022
  contro ADR-0018, ADR-0023 e ADR-0024 e contro i due rimandi che già porta — è del revisore, e non ho letto il disegno né gli
  altri ADR (§1 del dispaccio). Ho controllato soltanto, contro ADR-0022, le citazioni del rimando (sezione 3.0).
- **due affermazioni del rimando che vengono dal disegno**, non verificabili con la mia lettura: *«la seconda risposta della
  5.5 del disegno»* (che toglie il 5 dalla dipendenza dell'11) e le date di D12 e D17. Le porta il piano (P-4).
- **il push**: non fatto. Il criterio di chiusura «commit pushato» resta al coordinatore.

## 7. Il tempo

| | |
|---|---|
| inizio | `2026-09-30 10:50:27` (`S/start-time.txt`) |
| il cancello | `10:54:42` → `10:59:20`, quattro minuti e trentotto |
| il commit | `Wed Sep 30 11:01:17 2026 +0200` (dal `git show`) |
| ultime misure prima del rapporto | `2026-09-30 11:02:22` (`S/end-time.txt`) |
| rapporto scritto | alle 11:08:08 |
| totale al commit | undici minuti circa, di cui quattro e mezzo di cancello |
