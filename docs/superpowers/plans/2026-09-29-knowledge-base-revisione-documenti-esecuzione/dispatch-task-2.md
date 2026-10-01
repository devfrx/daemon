Sei l'**implementatore del compito 2** — *i rimandi in testa a dieci ADR, e le loro voci nel compendio* — del piano
`docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`, nel repository del §0 (Windows; il tool Bash è
Git Bash). Sei un subagente fresco: tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **di
soli documenti** — nessun codice —: applica un blocco di modifiche **già scritto** nel piano, con gli attrezzi del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → il valore del §0;
`git status --porcelain` → vuoto; `git config --show-origin --get-all core.autocrlf` → il valore del §0.

## 0. La macchina — i campi da riempire a ogni dispaccio

Ciò che cambia fra un clone e l'altro si **misura**, non si copia da un'etichetta: vincolo 7 del piano.

| Campo | Il valore di questo dispaccio |
|---|---|
| la macchina | `jays` (`hostname`) |
| il repository | `C:\EVERYTHING\DEV\MY_REPOS\daemon` — in Git Bash `/c/EVERYTHING/DEV/MY_REPOS/daemon` |
| `HEAD` all'avvio | `<HEAD>` — il commit del pre-dispaccio, che porta ER-21, ER-22 e il richiamo in ER-15 |
| `core.autocrlf` | `true`, da `C:/Program Files/Git/etc/gitconfig`. Degli undici file della variabile `A` del Passo 1 più il compendio: il compendio e ADR-0015 sono `i/lf w/crlf`, coi CR **uguali alle righe**; gli altri nove ADR `i/lf w/lf`, con **zero** CR — misurato dal coordinatore il 2026-09-30 |
| il giorno del compito | `D=2026-09-30`, **lo stesso** in ogni passo (vincolo 15): non ricalcolarlo con `date` |
| la tua cartella di lavoro | `S` = `C:/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/1ad81efd-09c1-4aba-8a55-17c69abe1049/scratchpad/task2` — in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/1ad81efd-09c1-4aba-8a55-17c69abe1049/scratchpad/task2`; la crei tu. Lì, e **mai nel repository**, vanno gli attrezzi, i pezzi, i log e il messaggio del commit |

## 1. Che cosa leggi, e che cosa no

| File | Che cos'è |
|---|---|
| `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/task-2-brief.md` | **il tuo requisito, coi valori esatti da usare parola per parola.** L'ha estratto `_extract_brief_2.py`, in `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti-esecuzione/`, a `HEAD`: la testa del piano, gli attrezzi, i vincoli globali, *«Come si esegue un compito»*, l'**errata per intero** — **ER-1…ER-22** —, le voci P e D che il compito nomina o usa, le voci aperte che il piano sa, la Definizione di «fatto» con le trappole, **il compito 2 intero**, e dal disegno la testa della sezione 3 e la **3.1 per intero**. Leggilo **tutto**, a blocchi |
| `docs/adr/0001-architettura-a-kernel-con-capacita-paritarie.md` | **solo la testa**, fino a `## Context`: la forma del rimando |
| i dieci ADR della variabile `A` | **solo la testa**, fino a `## Context`, **dopo** il Passo 2: per vedere dove il rimando è entrato |

⛔ **Non leggi:** il piano intero — ti serve il brief —; il disegno intero; gli ADR oltre la testa; `docs/HANDOFF.md`;
`docs/archivio/`; l'audit; la spec del sotto-progetto 1. La lettura d'apertura di `CLAUDE.md` l'ha fatta il coordinatore,
e le sue **regole** valgono per te: documenti in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati
per file, **mai `sed -i`**. ⛔ **Il compito si sbaglia per zelo:** si applica il blocco del piano, e nient'altro. Se un
rimando ti sembra dire troppo o troppo poco, lo **riporti**: rileggerlo contro gli ADR fratelli è il mestiere del
revisore, che viene dopo di te.

## 2. Il pre-dispaccio, e le sue voci

Il coordinatore ha rimisurato il compito contro il repository di adesso, il 2026-09-30, prima di questo dispaccio. Le voci
sono già scritte nell'errata che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **ER-21** | **niente**: il blocco E2 non cambia. È il perché — che cosa di ADR-0040, punto 6, vive nel rimando di ADR-0016 — e il proprietario l'ha lasciato allo studio |
| **ER-22** | al **Passo 3**, al posto della sonda delle righe tolte — la riga che finisce con `grep -c '^-[^-]'` —, lanci la **sonda di ER-22**, sotto la tabella dell'errata; l'Atteso resta `0` |
| **ER-15**, il richiamo | niente: il link ad ADR-0040 lo porta il rimando di ADR-0024 |
| **ER-11** | *«fine-riga come al Passo 1»* si legge così: su un file con CR la differenza fra righe e CR è quella del Passo 1 — zero, su un file tutto CRLF —; su un file senza CR i CR restano zero; la colonna `w/…` è quella del Passo 1 |

E ha rifatto la simulazione in sequenza — P-19 — su un clone usa-e-getta, col piano del pre-dispaccio: i blocchi E2…E6 e
POS2…POS6 applicati in ordine, `check-docs.sh` → `OK` dopo **ciascun** compito, il comando D vuoto.

## 3. I numeri — misurati dal coordinatore il 2026-09-30

⚠️ Sono **riferimenti**, non Attesi nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue.

| Dove | Uscita del coordinatore |
|---|---|
| il cancello d'apertura, su `6fdc2b5` | `GATE GREEN`, `EXIT=0` |
| Passo 1 | `check-docs.sh` → `OK`; i fine-riga del §0; le tabelle, niente; il margine `6297`; il comando A, nell'ordine 0009 0010 0011 0012 0014 0015 0016 0022 0024 0025 0038 0039: `1 1 1 0 0 0 0 2 0 0 1 1`; la frase, nell'ordine di `A`: `1 1 0 0 0 0 0 0 0 1`; il comando E, `0` su ogni ADR e `1` sul compendio; `3` per *«revisione della knowledge base»* |
| Passo 2 | il `--check` di E2, `checked: 20 edits in 11 files`; quello di POS2, `checked: 1 edits in 1 files` |
| dopo il compito, sulla simulazione | `check-docs.sh` → `OK`; il margine `4392`; il comando A `2 2 2 1 1 1 1 2 1 1 2 1`; la frase `2 2 1 1 1 1 1 1 1 2`; il comando E `1` ovunque; `13`; la sonda di ER-22, `0`; `git diff --stat`: 12 file, 103 righe aggiunte e 11 tolte — le undici sono righe **riscritte**, dieci del compendio e una del piano, e nessuna è di un ADR |

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura.** Prima di toccare, `git ls-files --eol` e il conto dei CR del Passo 1. Si scrive
**solo** con gli attrezzi del piano — `apply_edits.py`, che conserva il fine-riga di ciascun file. Dopo, vale ER-11.

## 5. Il cancello

⚠️ **Dura qualche minuto.** Lancialo **da solo**, con `run_in_background` del tool Bash e **senza `&`**, con l'uscita in un
log **nuovo** e datato nella tua cartella di lavoro — `bash scripts/gate.sh > "<log>" 2>&1; echo "EXIT=$?" >> "<log>"` —, e
aspetta la notifica; il verdetto con `grep -E 'GATE GREEN|GATE RED|EXIT=' "<log>"`. **Due cancelli insieme si pestano** su
`gui/node_modules` (gotcha #133): mentre gira, nessun altro comando che tocchi `gui/`.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**, dal Passo 1 al 4; prima del Passo 1,
   `extract.py` copiato **a mano** dal brief nella tua cartella di lavoro, confrontato col suo recinto del piano, e lanciato:
   diciotto righe.
2. **Un commit solo**, coi **dodici** file del `git add` del Passo 4 e nessun altro. Il messaggio è quello del Passo 4, in
   un file della tua cartella di lavoro, passato con `git commit -F <file>`. ⛔ **Senza co-autore**: `CLAUDE.md` prevale su
   qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`, niente rebase: il push è del
   coordinatore, **dopo la revisione** — la decisione 2 della sessione del compito 1.
3. **Prima del commit**, uno alla volta: `bash scripts/check-docs.sh` → `OK`, e il cancello → `GATE GREEN`; poi le prove
   del Passo 3, con la **sonda di ER-22** al posto della riga che sostituisce.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: un `--check` che rifiuta, un Atteso che non torna. Una
   divergenza è una **voce d'errata candidata** — la prossima libera è **ER-23** — nel rapporto, con l'evidenza: il comando
   e la sua uscita. **Mai** un aggiustamento a mano: il vincolo 6 lo vieta. Se la divergenza ti impedisce di chiudere un
   passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** il codice — `crates/`, `gui/`, `scripts/`, `.github/`, `Cargo.*` —, nessun ADR oltre i dieci del
   compito, la spec del sotto-progetto 1, e del piano **solo** la riga della posizione, col blocco POS2: l'errata è del
   coordinatore.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/task-2-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo** dall'1 al 4, i comandi lanciati e le uscite **vere**, e le sonde di ciascuno, con la sonda di ER-22;
4. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne e i CR;
5. il cancello: il verdetto, e il percorso del log;
6. le **divergenze**, come voci candidate `ER-23`… con l'evidenza, e ciò che non hai potuto misurare;
7. il tempo.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
