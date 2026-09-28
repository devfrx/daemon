> ⚠️ **Per il coordinatore, prima di dispacciare** — questo file è il **modello**, e viaggia con git. Il prompt che parte
> si scrive nella cartella di lavoro `.superpowers/sdd/2026-09-23-design-system/`, ignorata, **senza** questo riquadro e
> coi campi fra `<…>` riempiti: `<repo>`, `<HEAD>` — l'ultimo commit di `main` —, `<scratchpad>`, e i valori della macchina
> del §0 misurati, non copiati. Il brief si genera **prima**, dalla radice del repository, con
> `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_9.py`, e deve dire *«piano e disegno
> coincidono con `HEAD`»*. ⛔ **Il compito è provato su `f35d3c2`**: `git log --oneline f35d3c2..HEAD -- docs/tracciabilita.md
> docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/archivio/stato-storico.md docs/porta-di-qualita.md docs/riferimenti.md
> docs/superpowers/specs/ gui/ scripts/` deve rendere il solo commit del pre-controllo, o il compito si rilegge contro i file di
> allora (`CLAUDE.md`, domanda 5). Alla chiusura il prompt spedito, il rapporto, il prompt del revisore e la revisione si
> copiano nella cartella tracciata e si committano: il punto 8 di *«Come si esegue un compito»*. Il compito 9 **non ha uno
> sguardo**; i Passi 11 e 12 sono del coordinatore, nella chiusura (**E120**); il revisore confronta il commit col testo con
> `compare_task9.py <HEAD> <commit dell'implementatore>`.

Sei l'**implementatore del compito 9** — *la chiusura: i documenti in ogni casa, e la Definizione di «fatto» coi comandi* —
del piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `<repo>` (Windows; il tool Bash è Git Bash).
Sei un subagente fresco: tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **di soli
documenti** — nessun codice —, e ⛔ **il piano non lo tocchi**: i Passi 11 e 12 sono del coordinatore (**E120**).

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `<HEAD>`;
`git status --porcelain` → vuoto; `node --version` → una versione che `gui/package.json` accetta (`engines`);
`git config --show-origin --get-all core.autocrlf` → il valore di questa macchina, nel §0; Google Chrome stabile
installato, e la sua versione **letta dal nome della cartella** —
`ls "/c/Program Files/Google/Chrome/Application/" "$LOCALAPPDATA/Google/Chrome/Application/" 2>/dev/null` rende almeno una
cartella di versione; se una delle due cartelle non esiste il comando esce **2** pur stampando la versione: conta la
cartella, non l'uscita. ⛔ **Mai `chrome.exe --version`**: su Windows apre il browser col profilo dell'utente.

## 0. La macchina

Il repository vive in **due cloni**, e ciò che cambia fra i due si **misura**, non si copia da un'etichetta: voce **E72**
del piano della parte 2, ed **E1** di questo piano.

| | macchina `Jays` | macchina `zagor` |
|---|---|---|
| il repository | `E:\ALL\DEV\MY_REPOS\daemon` | `C:\Users\zagor\Desktop\harness` |
| `core.autocrlf` | `false` in `.git/config`, quindi l'albero è `w/lf` | `true` dal file di sistema: i documenti del compito sono `w/crlf`, misurato il 2026-09-28 |
| Google Chrome | si rilegge: si aggiorna **da sé** | `154.0.8037.58`, riletta dal file il 2026-09-28: si rilegge |
| Node | `v24.19.0`, il 2026-09-27 | `v24.19.0`, il 2026-09-28 |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_9.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-9-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E111**…**E120** sono del suo pre-controllo e sono **già applicate** al suo testo —, le voci **P-26**…**P-30**, le decisioni **D19**…**D24**, **D26** e **D29**, le voci aperte che il piano sa, **il compito 9 intero** con la sua **Definizione di «fatto»**, e dal disegno la (e), la (f), la tabella dei controlli, le assunzioni e il suo *«Come si riprende»* — **copiati parola per parola**, a `HEAD` = `<HEAD>`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: la **§6**, la **§12** e la
**§13** del compendio; il riquadro in testa a `docs/tracciabilita.md`; la sezione *«IL PASSO WEB E LE SONDE DELLA PARTE 2»*
di `docs/porta-di-qualita.md`, il modello della sezione nuova; e, per il Passo 9, le tabelle *«Ciò che questa sessione ha
imparato»* delle consegne, che il comando del passo elenca (**E116**), e la sezione *«I gotcha»* di `docs/HANDOFF.md`
— **quella sezione**, non il file.

⛔ **Non leggi:** il piano intero (oltre 12 000 righe: ti serve il brief); il disegno intero; il resto di
`docs/HANDOFF.md`; la cartella `docs/adr/`; il resto di `docs/archivio/`; l'audit; la spec del sotto-progetto 1. La lettura
d'apertura di `CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te:
documenti in italiano, nessuna cifra nuova senza il suo comando, un puntatore in una casa sola, i fine-riga conservati per
file, mai `sed -i`. ⛔ **E il compito si sbaglia per ZELO**: si tocca ciò che il piano ha reso falso o ha lasciato da
scrivere, e nient'altro — lo dice il compito, per esteso.

## 2. Il pre-controllo, e le sue dieci voci

Il **pre-controllo** del compito 9 — sulla macchina `zagor`, il 2026-09-28 — ha rifatto il compito dal testo del piano su un
`git worktree` di `f35d3c2`, ed eseguito la Definizione di «fatto» sull'albero. Ha trovato **dieci voci**, già scritte
nell'errata e **già applicate** al testo che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E111** | il blocco 3 della Definizione conta le sole dichiarazioni di `LayoutPack`: `3`, non `6` |
| **E112** | il perimetro del blocco 4 ha **tre** case: le liste *Files*, l'errata, e la cartella del dispaccio |
| **E113** | nella (e) la riga N-2 di E187 dice che la deduzione *«non la peggiora»* non regge |
| **E114** | il richiamo della stella polare nomina le due corse delle prove (**E10**) e `dist/` vuotato (**E31**) |
| **E115** | la colonna Commit della riga 8 c'è già: non si tocca |
| **E116** | ⚠️ il Passo 9 legge l'errata **e** le tabelle delle lezioni delle consegne; una trappola diventa un gotcha, una regola di lavoro una **proposta** nel rapporto, perché `CLAUDE.md` è del proprietario |
| **E117** | ⚠️ ogni testo archiviato si prova **parola per parola** con `word_for_word.py`, dettato nel Passo 6 |
| **E118** | l'assunzione della barra di scorrimento è misurata: il richiamo nel disegno c'è già, e la sua misura è una riga del Passo 8 |
| **E119** | un testo che si appende a una cella va **prima di ` \|`**, lo spazio e la barra che chiudono la riga |
| **E120** | ⛔ **fai i Passi 1–10 e il commit del Passo 13, senza il piano né l'archivio delle consegne**; la Definizione di «fatto» la **esegui** e ne metti le uscite nel **rapporto**; i Passi 11 e 12 e il push sono del coordinatore |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `zagor`

**Il cancello d'apertura del pre-controllo**, a `f35d3c2`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **170** prove
passate e una saltata, il progetto `browser` con **66**; il pezzo JavaScript `698.20 kB`, compresso `213.00 kB`;
`found 0 vulnerabilities`. ⛔ Li **rimisuri tu** prima di toccare.

**Misurati dal pre-controllo** — ⚠️ sono **riferimenti**, non Attesi nuovi: tu **misuri**, e un numero diverso si **riporta**,
non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 1 | i dodici file `i/lf`, `w/crlf` su `zagor`; il compendio `90735` byte sotto un tetto di `100352`; tracciabilità ✅ 48, 🔶 53, 📋 76, ⚠️ 0, ❌ 1; la roadmap fino alla riga 13; la tabella dei passi a sette righe e nove `run`; 133 gotcha; `B` = `d10d9a5`, e nulla su `crates/` e `gui/schema/` |
| 2–8, 10 | applicati dal testo su una copia: nessun rifiuto, ogni sonda come detta, `check-docs.sh` verde, il compendio a `90239` byte |
| Definizione | blocco 2: cinque corse uguali, `237 tests, 236 passed, 0 failed, success=True`, ventotto file su ventotto; blocco 3 come detto, con **E111**; blocco 4 come detto, con **E112** |

⛔ Se una prova cade anche **una** volta nelle cinque corse, lo **riporti** con la sua uscita: una caduta è una voce d'errata,
non una corsa da ripetere finché passa (P-19).

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files*,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina. **Dopo**,
la **forma**: su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di
prima. Scrivi con Python `newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`, che conserva il fine-riga del
file che trova e che copi dagli *Strumenti* del brief **nello scratchpad** `<scratchpad>`, mai nel repository; lì vanno anche
`word_for_word.py` e `dod_suite.py`, i log, `prima.txt`, `old-6.txt` e `old-h.txt`. ⛔ **Mai `sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache. Lancialo **da solo**, in background, con l'uscita in un
  log datato nello scratchpad, e aspetta la notifica. **Due cancelli insieme si pestano** su `gui/node_modules` (gotcha
  #133): mai un `npm` o un `vitest` mentre gira — e le cinque corse della Definizione **dopo** il cancello, non accanto.
- ⚠️ **Chrome si aggiorna da sé fra una corsa e l'altra**: un rosso del progetto `browser` che non nomina una prova si
  rilancia **dopo** aver riletto la versione dal nome della cartella.
- ⚠️ **La CI** la legge il coordinatore dopo il push, che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**, dal Passo 1 al 10; poi **esegui** la
   Definizione di «fatto» (il Passo 11) e ne scrivi le uscite nel rapporto, **non** nel piano; il Passo 12 **lo salti**
   (**E120**).
2. **Un commit solo**, coi soli documenti che i Passi 1–10 toccano: `docs/tracciabilita.md`, `docs/roadmap.md`,
   `docs/README.md`, `docs/COMPENDIO.md`, `docs/archivio/stato-storico.md`, `docs/porta-di-qualita.md`,
   `docs/riferimenti.md`, `docs/superpowers/specs/2026-09-22-design-system-design.md`,
   `docs/superpowers/specs/2026-09-07-direzione-gui-design.md`, e `docs/HANDOFF.md` **solo** se il Passo 9 scrive un gotcha.
   ⛔ **Il piano no**, né `docs/archivio/consegna-piano-design-system.md` (**E120**). Il messaggio sta in un file nello
   scratchpad e si passa con `git commit -F <file>`; comincia con `design-system(compito 9): ` (vincolo 15). ⛔ **Senza
   co-autore**: `CLAUDE.md` prevale su qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`,
   niente rebase: il push è del coordinatore, **dopo la revisione**.
3. **Prima del commit**, uno alla volta: `bash scripts/check-docs.sh` → `OK` e `bash scripts/gate.sh` → `GATE GREEN`; e le
   verifiche del Passo 13 — il margine, i fine-riga, `git diff --stat` coi soli file del punto 2, nessun segnaposto fuori dal
   piano, `git status --porcelain` confrontato con `prima.txt`.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E121** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** il codice — `crates/`, `gui/`, `scripts/`, `.github/`, `Cargo.*` —, nessun ADR, la §5 del compendio, la
   spec del sotto-progetto 1, nessun piano eseguito, e `CLAUDE.md`.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-9-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo** dall'1 al 10, i comandi lanciati e le uscite **vere**, e le sonde di ciascuno;
4. al Passo 9, le voci lette — l'errata e le tabelle delle lezioni —, i gotcha scritti col perché, e le **proposte per
   `CLAUDE.md`**, ciascuna con la consegna da cui viene;
5. la **Definizione di «fatto»** eseguita blocco per blocco, ogni comando con la sua uscita, e le divergenze dalle attese;
6. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
7. il cancello: le righe `Test Files` e `Tests` dei due progetti, il pezzo JavaScript e `found 0 vulnerabilities`;
8. le **divergenze**, come voci candidate `E121`… con l'evidenza, e ciò che non hai potuto misurare;
9. il tempo, e i percorsi dei log.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
