### La dodicesima chiusura — 2026-09-15, terza sessione del giorno: la REVISIONE DEL PIANO INTERO è fatta per nove perimetri su undici, e le correzioni sono applicate ai compiti 1–5; nessun compito è eseguito

⛔ **DA SAPERE SUBITO, quattro cose.** **(1)** I rapporti dei revisori, il registro delle correzioni e gli attrezzi
vivono in [`2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/),
tracciata da questo commit: `ledger.md` è la **casa unica** di ciò che resta da applicare, compito per compito, con
lo stato ⬜/✅ di ogni rilievo; i nove `R*-report.md` sono l'evidenza (ogni rilievo porta il comando e la resa);
`constraints.md` e `R*-prompt.md` i mandati; `patch_*.py` gli script già applicati, **modello** per i prossimi;
`check_after_write.sh` il controllo post-scrittura. ⚠️ `R10-report.md` ha fine-riga misti (629 CR su ~700 righe),
scritto così dal revisore: è un verbale, si lascia. **(2)** ⛔ **Le correzioni ai compiti 6–17 NON sono applicate**, e le
voci **P** nuove (P-117…) **non sono scritte**: l'errata resta vuota, `grep -c '^### P-'` rende ancora **116**. **(3)** ⛔
**Sei compiti NON hanno avuto la revisione in profondità** — il 3 (R1 caduto a metà), l'8 (R3 caduto a metà), il 13 (R6
mai partito), il 15, 16 e 17 (R8 mai partito): cinque subagenti su undici sono morti per il **limite di sessione
dell'API** (HTTP 429) dopo ~30 minuti; quei compiti sono coperti **solo di traverso** da R9a, R9b, R10 e R7. **(4)** Sulla
macchina restano `cargo-audit` 0.22.2 e Node `v24.9.0` (invariati dall'undicesima chiusura).

✅ **Che cosa è stato fatto.** Undici revisori in sola lettura dispacciati in parallelo (otto fette sui diciassette
compiti, due sulla copertura dei due disegni, uno sulla coerenza fra i compiti), ciascuno con l'ordine di **rilanciare
ogni comando** e di scrivere il rapporto **a pezzi** — ed è ciò che ha salvato tre dei cinque caduti. I nove rapporti
portano **circa centoquaranta rilievi confermati**, contati dalle loro tabelle (`grep -c '| CONFERMATO |'` su ciascuno),
fra cui misure fatte **eseguendo il modello del piano**: il trasporto `ipc` del compito 2 rende `Ok(0)` a pari vivo su
Windows (quattro sonde rosse su nove), `typescript` 7.0.2 rompe `vue-tsc` (`ERR_PACKAGE_PATH_NOT_EXPORTED`), il
compito 6 non compila in quattro posti, il renderer del 14 non compila (`TS2345`). Il coordinatore ha rimisurato i
bloccanti prima di decidere. Tre ondate di correzioni, ciascuna col cancello verde prima del commit: `265631b` (la testa:
numeri stantii di prima di D25, il contratto dell'11, le **quattordici decisioni D75–D88**, la roadmap), `6d298c7` (compiti
1 e 2: il trasporto riscritto su un thread lettore per client — **D78** — la suite in `tests/contract/ipc.rs` — **D82** —
i tre richiami di P-23 e i cinque nel disegno — **D88**), e questo commit (compiti 4 e 5: il *Trova* della spec spostato
sull'ultima riga del riquadro, la tabella dei tocchi a nove righe, il commento falso di `journal.rs` corretto nel primo
compito che lo tocca, `<data>` nei richiami — **D75**).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline baf3cde..HEAD` → quattro (tre ondate più questa chiusura) |
| Codice di prodotto | **non toccato**: `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` → **116** (invariato: le P della revisione sono da scrivere); `grep -c '^[|] \*\*D[0-9]' <questo file>` → **88** (74 + D75–D88) |
| Quanto è corretto | `ledger.md`: le righe ✅ dei compiti 1, 2, 4, 5 e della testa; tutto il resto ⬜ |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di ogni commit (quattro log nello scratchpad, che non sopravvive; resta il comando); `bash scripts/check-docs.sh` → `OK` con la cartella della revisione in posto |
| Fine-riga | questo piano è **LF**: `git ls-files --eol <questo file>` → `i/lf w/lf`, `tr -cd '\r' < <questo file> \| wc -c` → `0`; `docs/roadmap.md` resta `i/lf w/crlf` (toccata con `replace_unique.py`) |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Segnaposto | ⛔ **uno solo, DICHIARATO**: la versione di `interprocess` nel manifesto del finto (compito 12) — `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1** (il secondo, nel compito 9, è stato tolto: R4-15). `<data>` e `<tempo>` non sono segnaposto (D75) |
| Margine del compendio | **invariato, `11030` byte**: questa sessione non ha toccato il compendio |
| Documenti fuori dal piano | `docs/roadmap.md` (la riga del piano: «scritto il 2026-09-15, in revisione», R10-19) e la cartella della revisione, nuova |
| File temporanei | nessuno nel repository; lo scratchpad porta `replace_unique.py`, le sonde dei revisori (`probe-R*/`) e i log del cancello, e **non sopravvive** |
| Debito lasciato | **dichiarato per intero in `ledger.md`**: le correzioni dei compiti 6–17, le P-117…, la revisione in profondità di 3, 8, 13, 15, 16, 17 |

#### Le decisioni prese scrivendo, oltre a quelle della tabella (le quattordici della tabella sono D75–D88)

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 68 | la revisione del piano intero si fa con **undici revisori paralleli in sola lettura**, mandati in file, rapporto scritto a pezzi | è lettura, non scrittura; un revisore solo su 1,2 MB si pianta; tre rapporti su cinque caduti sono sopravvissuti perché scritti a pezzi | ~400k token per revisore; due perimetri sono rimasti scoperti |
| 69 | i rapporti e il registro **entrano nel repository**, in una cartella accanto al piano | lo scratchpad non sopravvive e il proprietario lavora da più macchine; i piani sono fuori dal controllo dei link (P-7), quindi `check-docs.sh` resta verde — provato | 756 KB in più nel repository; una cartella che invecchia se nessuno la archivia alla chiusura del piano |
| 70 | le correzioni si applicano **a ondate**, una o due compiti per commit, col cancello prima di ciascuno | un commit per ondata è un punto fermo che regge a una sessione che muore a metà — e questa è morta a metà | quattro cancelli da tre minuti l'uno |
| 71 | ogni rilievo **bloccante** è rimisurato dal coordinatore prima di decidere il rimedio; i «prosa» si applicano sulla parola del revisore | ciò che torna da un subagente è una dichiarazione, e su 140 rilievi solo i bloccanti pagano la rimisura | un rilievo di prosa falso applicato — nessuno trovato finora |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **UN SUBAGENTE MUORE COL LIMITE DI SESSIONE DELL'API, e non c'è preavviso.** Cinque su undici, HTTP 429 dopo ~30
  minuti. 📌 **L'ordine di scrivere il rapporto A PEZZI ha salvato tre rapporti su cinque**: senza, sarebbe stato zero.
  Un rapporto parziale dice **fin dove** è arrivato (*«le righe del compito 3 seguono»*), e quel confine va nel registro.
- ⛔ **`assert count == 1` SUL PIANO INTERO scatta su un'ancora che ogni compito ripete** (`- [ ] **Passo 10: i fine-riga,
  il cancello, il commit**`). 📌 **Le ancore si cercano dentro la fetta del compito** (`t.index("\n## Compito N:")`,
  `t.index("\n## Compito N+1:")`) e si asseriscono lì. È scattato due volte in una sessione, e nessuna ha scritto un byte:
  l'`assert` prima della scrittura atomica è ciò per cui esiste.
- ⛔ **UN `use` DI TRE RIGHE NELLO SCRIPT PUÒ FINIRE PRIMA DELLA DEFINIZIONE CHE USA**: `NameError` dopo la modifica
  dell'ancora. 📌 Dopo ogni modifica a uno script di patch si rilancia **sapendo che non scrive niente finché tutti gli
  `assert` passano** — e si controlla `git status` dopo un errore, non si presume.
- ⛔ **`try_clone` BATTE `split`** per dare il flusso a un thread lettore: `interprocess::TryClone` è un tratto pubblico
  del crate, mentre le due metà di `split()` sono tipi associati del tratto `Stream`, la cui via di esportazione non è
  scritta da nessuna parte nel sorgente. 📌 Prima di scrivere codice su una crate esterna, il nome si cerca nel sorgente
  installato (`~/.cargo/registry/src/*/`), e ciò che non si trova non si detta.
- ⛔ **UN REVISORE PUÒ RIPORTARE UN RILIEVO CON UN `grep` CHE NON TORNA** perché ha letto il rapporto di un altro
  compito: R7-3 nomina `case "StaleBuild"` che nel 13 non è scritto così. 📌 Il coordinatore rimisura i bloccanti **col
  proprio comando**, non rilanciando quello del revisore.
- ⛔ **IL LIMITE DI SESSIONE COLPISCE ANCHE IL COORDINATORE**: il proprietario ha chiuso per contesto. 📌 Le correzioni
  di questa specie si fanno **una ondata per sessione** se serve, con il registro come casa dello stato — non si aspetta
  di aver finito per scrivere il diario.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è
   il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (vincoli, posizione, errata, la tabella D
   **fino a D88**, le voci aperte), poi
   [`…-revisione/ledger.md`](2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md) **per intero**: è la
   mappa. Di ogni rapporto si legge **la riga** del rilievo che si sta applicando (`grep -n '^| R4-20' R4-report.md`), non
   il rapporto.
3. ⏭️ **LE CORREZIONI DEI COMPITI 6–17, nell'ordine della posizione**, uno script di patch per compito sul modello di
   `patch_c45.py` (fetta del compito, ancore asserite, scrittura atomica), `check_after_write.sh` dopo ogni scrittura, il
   cancello e un commit per ondata di uno o due compiti; ogni riga del registro passa a ✅ nello stesso commit. ⚠️ Le
   decisioni sono **prese** (D75–D88): dove il registro dice «vedi rapporto» si esegue il rimedio proposto, e dove il
   rimedio proposto contraddice una D vince la D.
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

