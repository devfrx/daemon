# Archivio — la consegna della sessione che ha scritto i due disegni del sotto-progetto 2, 2026-09-09

⛔ **Non è una lettura obbligatoria.** È il **verbale** della chiusura del 2026-09-09 — la sedicesima ripresa della
stella polare della GUI, la sessione che ha scritto sul posto i due disegni —, che stava nella §10 del
[disegno del 2](../superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md), *«Come si riprende»*, tenuto
**parola per parola** coi soli link riscritti per questa cartella. Spostato qui il 2026-10-07 col `lean-docs` della R5
del [terzo audit](../audit-2026-09-30.md): il richiamo del 2026-10-02 in testa alla sezione — AUD-658, AUD-660,
AUD-661 — la diceva già un verbale, e `CLAUDE.md` vuole che un documento vivo porti ciò che è vero adesso. Le consegne
di prima dello stesso file stanno in [`consegna-brainstorming-sottoprogetto-2.md`](consegna-brainstorming-sottoprogetto-2.md)
e in [`consegna-avvio-brainstorming-sottoprogetto-2.md`](consegna-avvio-brainstorming-sottoprogetto-2.md).

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Il 2 è chiuso — lo stato lo dice la
[roadmap](../roadmap.md) —, e il prossimo passo sta nella §6 di [`COMPENDIO.md`](../COMPENDIO.md), in un posto solo.
Nel disegno resta il titolo della §10, con la riga del richiamo che porta qui: i rimandi «la §10 del disegno del 2»
degli altri file passano da lì.

---

### §10 — Come si riprende · approvata il 2026-09-09 (delegata, «decidi secondo la skill»: A, decisione 37 della stella polare) — scritta alla chiusura della sedicesima ripresa, 2026-09-09, coi comandi

✅ **RICHIAMO DEL 2026-10-02, audit del 2026-09-30 (AUD-658, AUD-660, AUD-661):** questa sezione è il **verbale** della chiusura del
2026-09-09, non una consegna da eseguire: ogni suo punto è eseguito, e il 2 è chiuso — lo stato lo dice la
[roadmap](../roadmap.md). Lo stato, il compito e le misure che porta sono di quel giorno, e il prossimo passo vive solo nella
§6 del [compendio](../COMPENDIO.md).

⚠️ **È il documento di consegna della sessione che ha scritto i due disegni sul posto**, e sta qui e non in un file a parte
perché il repo ha già la sua convenzione: lo stato vive in file **tracciati**, e chi riprende legge i due disegni per intero.
**Una sola** «Come si riprende» (decisione 37): la [stella polare](../superpowers/specs/2026-09-07-direzione-gui-design.md) tiene la propria tabella
dello stato e rimanda alla §6 del compendio. Ogni riga è stata **rilanciata coi comandi** prima di essere scritta, non
ricordata. Le quattro parti che la §10 approvata prescriveva stanno qui nell'ordine: lo stato, il compito della sessione
successiva, ciò che i disegni consegnano al piano, la Definizione di «fatto» della parte 1.

⛔ **DA SAPERE SUBITO: niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato, **nessun
codice toccato**: questa sessione ha prodotto solo documenti — i due disegni, i due archivi, una sezione di `riferimenti.md`, il
puntatore della §6 del compendio.

⚠️ **La sessione si è chiusa PRIMA della rilettura del proprietario.** Il sì del proprietario è condizionato e si dà in chat,
non si deduce da una chiusura. La **domanda minima** con cui la sessione nuova apre, dopo la lettura obbligatoria e i due
disegni per intero: *«i due disegni sono riletti?»* — **A**, sì: si scrive la parte 1 del piano (punti 5–7 qui sotto); **B**,
no: si presentano una per volta, in forma A/B col consiglio scritto, le voci che sono del proprietario — le righe 5, 6, 11 e 13
della §9, e le decisioni di questa sessione qui sotto che tocca a lui tenere o ribaltare: i nomi (9), la §1 per rimando (2) —
e **poi** si scrive il piano nella stessa sessione, se il contesto regge. ✅ **Alla chiusura il proprietario ha risposto A** (decisione 40 della stella polare): la rilettura in una sessione nuova. ✅ **RILETTURA FATTA il 2026-09-09, nella sessione nuova:** B alla domanda minima; poi le sei voci una per volta, in forma piena (esiste, arriva, regge, i cinque criteri, verificato/dedotto/assunto), tutte **A** — la riga 5 della §9 (decisione 41), la 6 (42), l'11 (43), la 13 in due metà, X-1 e X-3 (44 e 45), i nomi (46), la §1 per rimando (47); ogni risposta scritta nei due disegni e, per X-1 e X-3, nell'audit, un commit l'una. Si passa ai punti 5–7 qui sotto. ✅ **Alla chiusura della rilettura il proprietario ha risposto A** (decisione 48 della stella polare): la parte 1 del piano si scrive in una **sessione nuova**, che apre con la lettura obbligatoria e i due disegni per intero e poi esegue i punti 5–7; nessuna domanda resta pendente. ✅ **PIANO DELLA PARTE 1 SCRITTO il 2026-09-09 e RILETTO il 2026-09-10**, in tre sessioni — la prima fino al compito 5, la seconda i compiti 6–8, la terza la revisione — in [`../plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md`](../superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md); l'esecuzione in una sessione nuova, un subagente fresco per compito (decisione 48). ✅ **PARTE 1 ESEGUITA il 2026-09-10:** il verbale nel piano — «A che punto è», l'errata, «Come si riprende»; la parte 2 si scrive ora, dalla §6 del compendio.

#### Lo stato alla chiusura, e il comando che lo rifà

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb` dopo `git fetch --all --prune`. Nessuno stash, nessuna operazione a metà |
| I commit di questa sessione | `git log --oneline a539f2a..HEAD` — i due disegni scritti sul posto, i due archivi, `riferimenti.md`, il puntatore della §6 del compendio; poi la chiusura, se separata |
| I commit della rilettura, 2026-09-09 | `git log --oneline bbf95fb..HEAD` — la correzione della riga «codice di prodotto» qui sotto, poi le decisioni 41–47, un commit l'una |
| Codice di prodotto | **non toccato**: `git diff --stat 664265a..HEAD -- crates/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende nulla; di `scripts/` solo `check-docs.sh` (il tetto del compendio, taglio 3) e della spec del sotto-progetto 1 solo la §8.2 (decisione 19): `git diff --stat 664265a..HEAD -- scripts/ docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` ⚠️ **RICHIAMO DEL 2026-09-09, alla ripresa per la rilettura del proprietario:** la prima metà di questa cella diceva il falso. Rilanciato, il comando rende **tre** file di `docs/adr/` — 0006, 0019 e 0022, tredici righe aggiunte ciascuno — che sono i **rimandi datati del 2026-09-08** della passata sui diagrammi (decisioni 17 e 18 della stella polare; commit `f6055b0` e `64892df`), non codice: `crates/`, `Cargo.lock`, `Cargo.toml` e `rust-toolchain.toml` non rendono nulla. La tabella dello stato della stella polare, che lancia il comando **senza** `docs/adr/`, era già giusta. Trovato rilanciando la riga alla ripresa, com'è la regola ✅ **RICHIAMO DEL 2026-09-22, dal compito 17 del piano della parte 2 (R8-26):** da questo piano il codice di prodotto **è** toccato — `crates/`, `scripts/`, `.github/`, i due lockfile e `gui/`, che nasce — e la **Definizione di «fatto»** del piano dice che cosa, file per file, col comando. ⚠️ **E questa §10 resta il verbale della sessione che ha scritto i disegni:** il diario della parte 2 vive nel piano (D74), non qui |
| Cancello | `bash scripts/check-docs.sh` → `OK`; `bash scripts/gate.sh` → **`GATE GREEN`, rilanciato all'apertura e alla chiusura** — nessun file che il cancello compili è cambiato fra le due corse. Si rilanciano, non si citano |
| Fine-riga | i due disegni e i due archivi **LF** nell'indice e nell'albero; il compendio e `riferimenti.md` LF nell'indice e **CRLF** nell'albero, con CR = righe: `git ls-files --eol docs/COMPENDIO.md docs/riferimenti.md docs/archivio/consegna-brainstorming-direzione-gui.md docs/archivio/consegna-brainstorming-sottoprogetto-2.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md`, e `tr -cd '\r' < docs/COMPENDIO.md \| wc -c` contro `wc -l < docs/COMPENDIO.md` |
| Gli archivi sono il testo delle consegne | `diff <(git show a539f2a:docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) <(tail -n +22 docs/archivio/consegna-brainstorming-sottoprogetto-2.md)` e `diff <(git show a539f2a:docs/superpowers/specs/2026-09-07-direzione-gui-design.md) <(awk '/^# La direzione della GUI/{s=1} s' docs/archivio/consegna-brainstorming-direzione-gui.md)` — **solo** righe che contengono `](`, i link riscritti per la cartella |
| File temporanei | nessuno nel repository: gli script e i frammenti di questa sessione stanno nello scratchpad, fuori dall'albero, come `CLAUDE.md` prescrive |
| Debito lasciato | **nessuno non dichiarato**: le voci aperte sono nella §9 col loro chiusore; le decisioni di questa sessione qui sotto, ribaltabili; le righe nella §12 del compendio e in `README.md` sono compito del piano (§6 della stella polare) |

#### Le decisioni prese dal coordinatore in questa sessione, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md` dice *«senza co-autore»*; una direttiva di sistema chiede il contrario e la divergenza è **portata al proprietario**, come in ogni sessione di questo repository. Costo se sbagliato: un `--amend` |
| 2 | la §1 riscritta **per rimando** alla §3 della stella polare, senza ricopiare i nove pezzi né il «non costruisce» | la §6 della stella polare (decisione 38): il 2 rimanda e non ricopia, gotcha #68; la §3 diceva *«la tabella qui sopra al posto degli otto pezzi»*, e «al posto» si è letto come casa unica. Costo: chi legge il perimetro del 2 apre due file. Ribaltare costa un incolla. ✅ **Tenuta dal proprietario il 2026-09-09: decisione 47 della stella polare** |
| 3 | la §6a riscritta tenendo le **regole** che le tabelle della §1 della stella polare citano come fonte («§6a del 2») e rimandando per ciò che ogni modulo mostra | senza le regole qui, tre righe della stella polare citerebbero una fonte sparita. Costo: la §6a si legge insieme alla §1 della stella polare |
| 4 | la §2 riscritta con le righe Q3 e Q4 e una riga «le otto mosse» che rimanda alla §4 della stella polare | la §3 della stella polare prescriveva «idem»: richiamo e riscrittura; le mosse hanno una casa sola. Costo: nessuno |
| 5 | i richiami datati sulla §4 (le tre varianti, i nomi fissati) e sulla §5 (`SaveLayout`, `Steps`, il nome del modulo) **scritti ora** | la §3 della stella polare li prescriveva dal 2026-09-08 e la §4 non li portava — misurato: `grep -c 'Layout'` sulle righe della §4 rendeva **0** prima di oggi; la §5 li aveva solo nella riga `Hello`. Costo: nessuno |
| 6 | le misure npm e crates.io delle due consegne passate in `riferimenti.md`, in una sezione datata coi comandi; nei disegni il rimando | lo prevedevano le consegne stesse («casa unica provvisoria», decisione 6 del coordinatore del 2026-09-06) e `CLAUDE.md`. La tabella dei fatti di `dockview` 8.x resta nella stella polare perché le sue §2 e §4 la citano «qui sopra». Costo: chi vuole una versione apre `riferimenti.md`, e comunque la rimisura |
| 7 | nella stella polare restano vive le due tabelle delle decisioni, le registrate e i vicoli ciechi; escono la cronaca, le «sezioni che mancano» (tutte chiuse) e il «prossimo passo» (la §6 del compendio); il file **intero** com'era va in coda all'archivio come istantanea | le sezioni citano le decisioni per numero; un'istantanea intera è ciò che la decisione 37 del coordinatore della stella polare aveva già scelto. Costo: l'archivio cresce del file intero, e non è lettura obbligatoria |
| 8 | in questo file escono lo stato, il «fatto», le «sezioni che mancano» e il «prossimo passo» superato dal 2026-09-07; restano le risposte del proprietario, i fatti del codice, le decisioni del coordinatore del 2026-09-06 e i vicoli ciechi | stessa regola: il vivo porta ciò che è vero adesso. Costo: nessuno, l'archivio ha tutto |
| 9 | i **nomi inglesi** fissati nella §1: la settima porta `custody` (`Custody`, `keep`, `retrieve`, `CustodyKey::Layout`), il registro `registry`, i messaggi della §4 confermati e la lista dei passi `Steps` | le sezioni li lasciavano «al disegno scritto» (voce 16 della §9); `custody` è la parola dei disegni stessi, i verbi corti come le altre porte. Costo se sbagliato: un rinomina prima che esista codice ✅ **Tenuta dal proprietario il 2026-09-09: decisione 46 della stella polare** |
| 10 | nessuna riga nuova in `README.md`, `roadmap.md`, `tracciabilita.md` e nella §12 del compendio | compito del piano, decisione 9 del coordinatore della stella polare e §6. Costo: fino al piano, i due disegni li trova solo chi parte dalla §6 del compendio |
| 11 | la rilettura del proprietario **non è data per fatta**: la riga «⏳» in testa ai due disegni, e la domanda minima qui sopra | il sì è condizionato e si dà in chat. Costo: nessuno. ✅ **Fatta il 2026-09-09**, le righe «⏳» chiuse |

#### Il compito della sessione successiva: la rilettura del proprietario, poi il piano in due parti

In ordine, e ogni riga è eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -3`: si parte da `main`, e la testa deve essere il
   commit di questa chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e i due pezzi dell'audit del 2026-08-27 — poi
   la **stella polare per intero** e **questo file per intero**. Gli archivi **non** sono lettura obbligatoria. ⚠️ Col tool Bash
   le tabelle lunghe traboccano: la tabella delle decisioni del proprietario della stella polare a **30** righe per chiamata,
   il resto a 75–150 (i vicoli ciechi della stella polare dicono dove).
3. Il proprietario **rilegge** i due disegni sotto la sua accettazione condizionata — la domanda minima in testa a questa
   sezione. Le voci per lui: le righe 5, 6, 11 e 13 della §9 e le decisioni 2 e 9 di questa sessione; se non dice altro, il
   piano scrive i consigli. ✅ **Fatto il 2026-09-09:** tutte e sei decise A, coi consigli (decisioni 41–47).
4. Prima di scrivere il piano, la regola di `CLAUDE.md` su `superpowers:writing-plans`: le voci aperte **si sanno prima**. Dove
   stanno: la §9 di questo file; le registrate della stella polare; le voci senza numero AUD
   dell'[audit](../audit-2026-08-27.md) — X-1 e X-3 pesano sul 2, voce 13 della §9; le tabelle delle voci aperte dei
   Traguardi 5 e 6 di [`porta-di-qualita.md`](../porta-di-qualita.md), coi due `awk` della §6 del compendio. ⚠️ Quali
   abbiano come chiusore **questo piano** o *«il proprietario, prima»* lo decide chi lo scrive leggendo la colonna «Chi la
   chiude», non questa riga: qui sbarra solo AUD-004, e sbarra il **13**, non il 2.
5. `superpowers:writing-plans`: la **parte 1** in `docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md` (⚠️ **RICHIAMO DEL 2026-09-22, compito 17 del piano della parte 2 — E192:** qui il nome portava un segnaposto al posto della data, scritto quando quel piano non esisteva; è `2026-09-09-…` dal 2026-09-09) —
   il pezzo 1 della §3 della stella polare: `spikes/gui-shell/`, il protocollo congelato al primo commit di codice con M1–M5,
   Q1–Q4 e le otto mosse, i due gusci, la Home finta con `dockview-core`, la mossa 8 col worker e il relay di SP-7, l'esito in
   `spikes/RISULTATI.md`, ADR-0029 chiuso, `.gitignore` e i lockfile — con la Definizione di «fatto» qui sotto **copiata da
   qui**. In testa: modalità subagent-driven, errata, pre-controllo — la forma dei piani precedenti, `ls docs/superpowers/plans/`.
   ⚠️ Le versioni di ogni attrezzo si rimisurano quel giorno coi comandi di `riferimenti.md` e della §9.
6. Il pre-controllo delle quattro domande di `CLAUDE.md` su ciascun compito, **nella sessione che scrive il piano**; ogni
   compito si legge contro il codice di **adesso**, e le sezioni «Vicoli ciechi e trappole» dei due disegni dicono dove
   guardare.
7. L'esecuzione in una sessione **nuova**, un subagente fresco per compito, revisione fra uno e l'altro
   (`superpowers:subagent-driven-development`): la regola del proprietario. Le due domande A/B per la CI — la metà Windows
   (X-1) e `npm audit` (X-3), voce 13 della §9 — si pongono al proprietario **quando si scrive il piano**. ✅ **Già poste e decise A il 2026-09-09** (decisioni 44 e 45): il piano le esegue.
8. Dopo la misura: la **parte 2** — i pezzi 2–9 della §3 della stella polare, con ciò che dipende dal guscio deciso coi numeri
   (chi decodifica, `gui/shell/` se vince Tauri; ⚠️ **RICHIAMO DEL 2026-10-07** — audit del 2026-09-30: il 2026-09-10
   [ADR-0029](../adr/0029-guscio-della-gui.md) ha scelto Electron) — scritta con lo stesso pre-controllo ed eseguita allo stesso modo; le righe
   nella §12 del compendio, in `README.md`, nella roadmap (il titolo della riga 2) e in tracciabilità entrano con essa (§6 della
   stella polare; voce 15 della §9).
9. A piano eseguito: la §6 del compendio porta il passo successivo, e i richiami datati nella §1 della stella polare per i
   moduli che il 2 ha costruito. ✅ **RICHIAMO DEL 2026-10-02, audit del 2026-09-30 (AUD-659):** quale sia il passo lo dice solo la
   §6, e l'ordine dei sotto-progetti la [roadmap](../roadmap.md).

#### Ciò che i due disegni consegnano a chi scriverà il piano

📌 È suo e non un puntatore: i **nove pezzi** del 2 in ordine, con dove vive ciascuno — la §3 della stella polare; la **forma**
di ciascuno — le §3–§7 qui (il filo, lo schema, il registro e il daemon, la GUI, il core finto) e le §2 e §4 della stella polare
(la settima porta, lo spike); la tabella **artefatto → controllo** della §8, da cui i compiti si tagliano; i **nomi** fissati
nella §1; le decisioni aperte col chiusore e i **consigli** della §9, che il piano scrive se il proprietario non dice altro;
l'**ordine dei messaggi** nelle tre sequenze della stella polare, e chi apre il passo; le **fonti** con le date in
`riferimenti.md`, da rimisurare; le **trappole** — le sezioni «Vicoli ciechi e trappole» dei due disegni; e la Definizione di
«fatto» qui sotto.

#### La Definizione di «fatto» della parte 1 del piano

Il piano la copia da qui, e la parte 2 ne scrive la propria dopo la misura.

| # | Condizione | Chi la verifica |
|---|---|---|
| 1 | il protocollo dello spike congelato in `spikes/gui-shell/PROTOCOLLO.md` **al primo commit di codice**, con M1–M5, Q1–Q4 e le otto mosse | `git log` sul file: il primo commit precede ogni misura |
| 2 | i numeri di M1–M5 e le righe Q1–Q4 in ADR-0029, lo stato ad `Accepted` con l'innesco Linux scritto; la riga del guscio nella §4 del compendio chiusa | `check-docs.sh`, che non elenca più ADR in `Proposed`; il richiamo datato |
| 3 | le otto mosse giudicate dal proprietario con le sue parole in `spikes/RISULTATI.md`, la mossa 7 come due JSON uguali; l'esito di `dockview` scritto — resta, o la tela libera | la sezione nuova di `RISULTATI.md` |
| 4 | `.gitignore` con le cartelle di build dello spike; i lockfile dello spike committati | `git status --porcelain` vuoto dopo lo spike |
| 5 | **nessun codice di prodotto toccato** dalla parte 1: `git diff --stat <base>..HEAD -- crates/ scripts/ Cargo.lock` vuoto | il comando |
| 6 | fine-riga rimisurati per ogni file toccato | chi esegue |
| 7 | la parte 2 del piano scritta **dopo** la misura, con lo stesso pre-controllo | la sessione che la scrive |

#### Ciò che questa riscrittura ha misurato, e che non era scritto da nessuna parte

| # | Misura | Che cosa cambia |
|---|---|---|
| 1 | la §4 non portava il richiamo datato per le tre varianti che la §3 della stella polare prescriveva dal 2026-09-08 (`grep -c Layout` sulle righe della §4: 0); la §5 lo aveva solo nella riga `Hello` | chiusa oggi: decisione 5 qui sopra |
| 2 | il codice non è cambiato da `664265a` (2026-09-06), il comando nella tabella dello stato: ogni «verificato nel codice» delle sedici riprese vale ancora | nessuna riga da rileggere |
| 3 | la tabella delle decisioni del proprietario della stella polare, letta col tool Bash a 65 righe, **trabocca** (33 KB): si legge a 30 | una trappola in più nei vicoli ciechi della stella polare |
| 4 | i nomi che tre sezioni lasciavano «al disegno scritto» non erano raccolti da nessuna parte come compito: la tabella dei nomi nella §1 li raccoglie | decisione 9 |

**I tre controlli (decisione 18).** *Esiste:* le sezioni «Come si riprende» dei disegni dei
[gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) e della [knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md), da cui la
forma; la §10 approvata, che diceva le quattro parti. *Arriva:* la parte 2 del piano, che riscrive questa sezione come diario.
*Regge crescendo:* ogni chiusura riscrive lo stato e le righe fatte, nello stesso file.

**Controllo sui cinque criteri.** Verificato coi comandi il 2026-09-09 ciò che sta nella tabella dello stato. Coerenza: stessa
forma dei due precedenti. Debito: nessuno non dichiarato. Stato dell'arte: le versioni si rimisurano al piano. Proporzione: una
sezione, quattro parti. **Dedotto:** niente. **Assunto:** niente.
