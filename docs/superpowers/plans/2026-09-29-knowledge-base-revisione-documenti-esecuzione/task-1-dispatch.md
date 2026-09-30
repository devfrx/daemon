Sei l'**implementatore del compito 1** — *ADR-0040, e ciò che il cancello pretende con lui* — del piano
`docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`, nel repository del §0 (Windows; il tool Bash è
Git Bash). Sei un subagente fresco: tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **di
soli documenti** — nessun codice —: applica blocchi di modifiche **già scritti** nel piano, con gli attrezzi del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → il valore del §0;
`git status --porcelain` → vuoto; `git config --show-origin --get-all core.autocrlf` → il valore del §0.

## 0. La macchina — i campi da riempire a ogni dispaccio

Ciò che cambia fra un clone e l'altro si **misura**, non si copia da un'etichetta: vincolo 7 del piano.

| Campo | Il valore di questo dispaccio |
|---|---|
| la macchina | `jays` (`hostname`) |
| il repository | `C:\EVERYTHING\DEV\MY_REPOS\daemon` — in Git Bash `/c/EVERYTHING/DEV/MY_REPOS/daemon` |
| `HEAD` all'avvio | `8bb8440` — il commit del pre-dispaccio, che porta ER-9 ed ER-10 |
| `core.autocrlf` | `true`, da `C:/Program Files/Git/etc/gitconfig`: gli otto file della variabile `F` del Passo 1 sono `i/lf w/crlf`, coi CR **uguali alle righe** — misurato dal coordinatore il 2026-09-30 |
| il giorno del compito | `D=2026-09-30`, **lo stesso** in ogni passo (vincolo 15): non ricalcolarlo con `date` |
| la tua cartella di lavoro | `S` = `C:/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/task1` — in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/task1`; la crei tu. Lì, e **mai nel repository**, vanno gli attrezzi, i pezzi, i log e il messaggio del commit |

## 1. Che cosa leggi, e che cosa no

| File | Che cos'è |
|---|---|
| `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/task-1-brief.md` | **il tuo requisito, coi valori esatti da usare parola per parola.** L'ha estratto `_extract_brief_1.py`, in `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti-esecuzione/`, a `HEAD`: la testa del piano, gli attrezzi, i vincoli globali, *«Come si esegue un compito»*, l'**errata per intero** — **ER-1…ER-10** —, le voci P e D che il compito nomina o usa, le voci aperte che il piano sa, la Definizione di «fatto» con le trappole, **il compito 1 intero**, e dal disegno la testa della sezione 3, la riga di ADR-0022 della 3.1, la 3.2, la 3.3, la 3.4 e le due domande della 6.1. Leggilo **tutto**, a blocchi |
| `docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md` | **per intero**, coi suoi due rimandi: il compito lo chiede |
| `docs/adr/0001-architettura-a-kernel-con-capacita-paritarie.md` | **solo la testa**, fino a `## Context`: la forma del rimando |

⛔ **Non leggi:** il piano intero — ti serve il brief —; il disegno intero; `docs/HANDOFF.md` oltre le righe che il blocco
tocca; la cartella `docs/adr/` oltre i due file qui sopra; `docs/archivio/`; l'audit; la spec del sotto-progetto 1. La
lettura d'apertura di `CLAUDE.md` l'ha fatta il coordinatore, e le sue **regole** valgono per te: documenti in italiano,
nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, **mai `sed -i`**. ⛔ **Il compito si sbaglia per
zelo:** si applicano i blocchi del piano, e nient'altro.

## 2. Il pre-dispaccio, e le sue due voci

Il coordinatore ha rimisurato il compito contro il repository di adesso, il 2026-09-30, prima di questo dispaccio. Ha
trovato **due voci**, già scritte nell'errata che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **ER-9** | al Passo 1 il comando E sull'archivio **non** rende `6`: si **annota** il valore — sul commit del pre-dispaccio, `8` —, e al Passo 4 dev'essere **lo stesso** |
| **ER-10** | al Passo 4, prima del commit, la sonda di ER-10 — sotto la tabella dell'errata — deve rendere `SAME` |

E ha rifatto la simulazione in sequenza — P-19 — su una copia usa-e-getta: i sei blocchi `checked` sul repository, e dopo il
compito 1 `check-docs.sh` → `OK`.

## 3. I numeri — misurati dal coordinatore il 2026-09-30

⚠️ Sono **riferimenti**, non Attesi nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue.

| Dove | Uscita del coordinatore |
|---|---|
| il cancello d'apertura, su `dd0e265` | `GATE GREEN`, `EXIT=0` |
| Passo 1 | `check-docs.sh` → `OK`; gli otto file `i/lf w/crlf`, i CR uguali alle righe; le tabelle, `31 docs/HANDOFF.md`; il margine `8103`; i totali, otto righe a trentanove; `0` file di ADR-0040; `1` rimando in ADR-0022; il comando E `0 0 1 0 1 0 0 8`, nell'ordine di `F`; `0` per `modificato in parte da ADR-0040` nel compendio (ER-4); `0` e `0` per `superato in parte` in `CLAUDE.md` e nel compendio |
| Passo 3 | il `--check` di E1, `checked: 17 edits in 7 files`; quello di POS1, `checked: 1 edits in 1 files` |
| dopo il compito, sulla simulazione | il margine `6483`; `check-docs.sh` → `OK`; la sonda di ER-10, `SAME` |

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura.** Prima di toccare, `git ls-files --eol` e il conto dei CR del Passo 1. Si scrive
**solo** con gli attrezzi del piano — `apply_edits.py` e `archive_head.py`, che conservano il fine-riga di ciascun file — e
col `sed` del Passo 2, che crea ADR-0040: un file nuovo nasce LF. Dopo, i CR sono **quanti prima**, la colonna `w/…` è
quella di prima, e ADR-0040 è `w/lf`.

## 5. Il cancello

⚠️ **Dura qualche minuto.** Lancialo **da solo**, con `run_in_background` del tool Bash e **senza `&`**, con l'uscita in un
log datato nella tua cartella di lavoro — `bash scripts/gate.sh > "<log>" 2>&1; echo "EXIT=$?" >> "<log>"` —, e aspetta la
notifica; il verdetto con `grep -E 'GATE GREEN|GATE RED|EXIT=' "<log>"`. **Due cancelli insieme si pestano** su
`gui/node_modules` (gotcha #133): mentre gira, nessun altro comando che tocchi `gui/`.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**, dal Passo 1 al 5; prima del Passo 1,
   `extract.py` copiato **a mano** dal brief nella tua cartella di lavoro, confrontato col suo recinto del piano, e lanciato:
   diciotto righe.
2. **Un commit solo**, coi **dieci** file del `git add` del Passo 5 e nessun altro. Il messaggio è quello del Passo 5, in un
   file della tua cartella di lavoro, passato con `git commit -F <file>`. ⛔ **Senza co-autore**: `CLAUDE.md` prevale su
   qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`, niente rebase: il push è del
   coordinatore, **dopo la revisione**.
3. **Prima del commit**, uno alla volta: `bash scripts/check-docs.sh` → `OK`, e il cancello → `GATE GREEN`; poi le prove
   del Passo 4 **e la sonda di ER-10**.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: un `--check` che rifiuta, un Atteso che non torna. Una
   divergenza è una **voce d'errata candidata** — la prossima libera è **ER-11** — nel rapporto, con l'evidenza: il comando
   e la sua uscita. **Mai** un aggiustamento a mano: il vincolo 6 lo vieta. Se la divergenza ti impedisce di chiudere un
   passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** il codice — `crates/`, `gui/`, `scripts/`, `.github/`, `Cargo.*` —, nessun ADR oltre ADR-0022 e il file
   nuovo, la spec del sotto-progetto 1, e del piano **solo** la riga della posizione, col blocco POS1: l'errata è del
   coordinatore.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/task-1-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo** dall'1 al 5, i comandi lanciati e le uscite **vere**, e le sonde di ciascuno, con la sonda di ER-10;
4. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne e i CR;
5. il cancello: il verdetto, e il percorso del log;
6. le **divergenze**, come voci candidate `ER-11`… con l'evidenza, e ciò che non hai potuto misurare;
7. il tempo.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
