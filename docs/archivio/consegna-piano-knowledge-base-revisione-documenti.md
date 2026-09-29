# Il piano dei documenti della revisione della knowledge base — le consegne passate

> ⛔ **Non è lettura obbligatoria.** Qui stanno, **parola per parola**, le chiusure di sessione che il
> [piano](../superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md) non tiene più: un documento vivo tiene
> solo l'ultima (`CLAUDE.md`). La più vecchia per prima.

## La sospensione della scrittura, del 2026-09-29

Tolta dal piano il 2026-09-29, quando la sessione di ripresa l'ha finito. Le voci P-1…P-14 e le decisioni D1…D18 sono
passate nella testa del piano, parola per parola; i vicoli ciechi nelle sue trappole; il materiale — gli attrezzi, il
testo di ADR-0040, i sei blocchi — nei compiti, parola per parola, e qui non si ricopia: il commit `ff6f0e5` lo tiene
com'era. Il resto, com'era in quel commit, coi link riscritti per questa cartella: l'avviso in testa, la consegna, e il
capoverso che apriva il materiale.

> ⛔ **IN SCRITTURA, A METÀ — NON SI ESEGUE.** Il 2026-09-29 la sessione che lo scriveva si è fermata su richiesta del
> proprietario — *«si continua nella prossima sessione»* — col **materiale verificato** e senza la **prosa dei compiti**.
> Questo file porta il materiale parola per parola, e la consegna: *«Come si riprende»*, qui sotto. Il piano si completa
> nella prossima sessione; il pre-controllo e l'esecuzione vengono dopo, ciascuno in una sessione sua (`CLAUDE.md`,
> *«Una fase per sessione»*). Il disegno che il piano traduce è il
> [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md), chiuso e riletto
> dal proprietario il 2026-09-29.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-29, coi comandi

⛔ **Da sapere subito: niente è a metà nel repository.** Nessun file è stato toccato oltre a questo, al puntatore della §6
del compendio con la sua riga della data, e all'archivio che li tiene com'erano; nessun codice. A metà è **il piano**: il
materiale sotto è verificato, la prosa che lo avvolge manca. ⚠️ **Il materiale non si esegue così com'è:** i blocchi sono
stati controllati uno per uno sul repository di `0c0d0d4`, **non in sequenza**, e nessun compito è stato pre-controllato.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin` dopo il push di questa chiusura: `git fetch --all --prune`, `git status -sb`; nessuno stash |
| codice di prodotto | **non toccato**: `git diff --stat 0c0d0d4..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura di questa sessione, e prima del commit di questa chiusura; `bash scripts/check-docs.sh` → `OK`. Si rilanciano, non si citano |
| il margine del compendio | il comando C della 6.2 del disegno — il tetto, meno i byte senza CR, meno le righe; rendeva 8 538 su `0c0d0d4` |
| i blocchi | ciascuno si ricontrolla col suo `--check`, dalla radice del repository — il comando sotto questa tabella |

```bash
for n in 1 2 3 4 5 6; do printf 'E%s: ' $n; PYTHONIOENCODING=utf-8 python <scratchpad>/apply_edits.py --check 2026-09-30 <scratchpad>/e$n.txt 2>&1 | tail -1; done
```

Gli attrezzi e i blocchi si estraggono da questo file nello scratchpad della sessione che riprende — l'aiutante
`apply_edits.py`, e i blocchi come `e1.txt` … `e6.txt` —, ciascuno dal suo recinto, parola per parola. Su `0c0d0d4` i
blocchi **1–5** rendevano `checked` — 17, 20, 59, 24 e 31 modifiche —; il **6** rifiuta due ancore, **per costruzione**:
la riga di questo piano nella tabella dei piani di `roadmap.md` e la spunta del punto 3 della 6.6 del disegno, che non
esistono finché questo piano non è scritto — la voce 5 del compito qui sotto.

**Il compito della sessione che riprende — finire di scrivere il piano, con `superpowers:writing-plans`:**

1. La lettura d'apertura di `CLAUDE.md`; poi il disegno della revisione **per intero**, a blocchi; poi **questo file per
   intero**. Il precedente della forma è il [piano dei documenti del 2026-09-04](../superpowers/plans/2026-09-04-knowledge-base-documenti.md),
   fino al compito 1 compreso; le convenzioni più recenti — il modello `opus`, il dispaccio che viaggia con git, i vincoli
   globali — stanno nella testa del [piano del design system](../superpowers/plans/2026-09-23-design-system.md).
2. `bash scripts/gate.sh`, da solo; il comando C; il `--check` dei sei blocchi.
3. **La testa del piano**, sopra questa sezione, sulla forma del precedente: *Per chi esegue*, obiettivo, forma, strumenti
   — i tre attrezzi di sotto, al posto di `replace_unique.py` —, i vincoli globali, la tabella della posizione coi sei
   compiti di D1, *«Come si esegue un compito»* col punto sul dispaccio che viaggia con git (D17), l'errata vuota, le
   voci P e D qui sotto portate in *«Ciò che la scrittura del piano ha trovato»* e *«Le decisioni prese scrivendo il
   piano»*, la mappa dei file, le voci aperte che il piano sa, e la **Definizione di «fatto»** copiata dalla 6.2 del
   disegno, coi comandi A–E e le trappole della 6.3, più le righe che P-2 e P-3 aggiungono.
4. **La prosa dei sei compiti**, uno per volta: *Files* e *Read*; il Passo 1 con le misure prima; il Passo che applica il
   blocco — `--check`, poi senza — e, nel compito 1, il Passo che scrive ADR-0040 dal recinto e lancia `archive_head.py`
   **prima** del blocco; le prove, coi comandi A–E e i fine-riga; il commit — `knowledge-base-revisione(compito N): …`,
   senza co-autore — e il push; il criterio di chiusura. Il compito 6 vuole anche il **testo del puntatore nuovo** della
   §6, che non è scritto: dirà che la revisione è chiusa, e che vengono il 13 — sbarrato da AUD-004, col perimetro della
   4.2 del disegno — e il brainstorming dei modelli decisionali, **senza** collocarli l'uno rispetto all'altro, perché il
   proprietario non l'ha fatto.
5. **La chiusura della sessione di scrittura**: la riga di questo piano in coda alla tabella dei piani di `roadmap.md`,
   con la cella *«⏳ **scritto il 2026-09-29**; il pre-controllo e l'esecuzione, in sessioni loro»* che il blocco 6 cerca —
   o il blocco si riallinea alla cella scritta —, e l'*«Ultimo aggiornamento»*; nel disegno, il punto 3 della 6.6 riscritto
   col suo inizio *«3. ✅ il **piano dei documenti** — **scritto il 2026-09-29**, in una sessione sua, al suo»*, idem; il
   puntatore della §6 del compendio mosso al pre-controllo, con `archive_head.py --pointer` prima e `replace_pointer.py`
   dopo; questa sezione riscritta come consegna al pre-controllo.
6. ⛔ **Prima del commit del piano, la simulazione in sequenza**: un `git worktree` nello scratchpad sul commit del piano,
   i sei compiti applicati in ordine — ADR-0040 scritto, `archive_head.py`, i blocchi — e `bash scripts/check-docs.sh`
   lì dentro, che deve rendere `OK`; poi il worktree si toglie. È la prova che il `--check` a uno a uno non dà: che i blocchi
   reggano **uno dopo l'altro**, e che il cancello sia verde dopo il compito 1, che è quello che lo tocca.

**Ciò che la scrittura del piano ha trovato** — ciascuna misurata su `0c0d0d4`; si portano nella testa:

| | Trovato | Che cosa ne segue |
|---|---|---|
| P-1 | `docs/HANDOFF.md` porta già **un** link al disegno — nel gotcha #141, dal commit `fedca31` —, e `docs/riferimenti.md` tre, le fonti della 3.4: la 6.2 dice che prima del piano nessun file da toccare ne porta, tranne il compendio | nessun controllo cambia: per `HANDOFF.md` il controllo è la sola guardia dei totali; il comando E vi rende uno prima e dopo |
| P-2 | **`docs/design/10-modello-dei-dati-durevoli.md`** dice ancora ciò che la revisione supera, e il disegno non lo nomina: `AMBITO` e `CHECKPOINT` «col 5» e *«l'implementazione vera col 5»* — D15 —; `CARTELLA_KB` *«in chiaro, nel backup»* — ADR-0040, e una politica che quel file dice di non ripetere —; le specie di `NODO_KB` e le frecce dell'`INDICE_MAPPA` — la riga 11 —; `GUIDA_APPROVATA` senza la fiducia di D9 | nel compito 4, blocco E4: un richiamo sotto l'intestazione della sezione, cinque etichette, tre righe della tabella — D8 |
| P-3 | il modulo **Backup** della stella polare della GUI elenca ciò che il backup non contiene — indici, pesi, segreti — e il punto 4 di ADR-0040 vi aggiunge la root, le zone e le copie: la 5.3 nomina sette punti, questo è l'ottavo | nel compito 5, blocco E5 — D9 |
| P-4 | ADR-0022 porta, nel rimando del 2026-08-07 in fondo, *«sotto-progetto 11 … dopo il 5, il 6 e il 9»*: la seconda risposta della 5.5 toglie il 5 | lo dice il rimando nuovo in testa ad ADR-0022, blocco E1 |
| P-5 | la **§8.5.2** della spec del sotto-progetto 1 dice *«Servono inoltre il filesystem reale, che arriva con il sotto-progetto 5»*: la §8 non si tocca | **registrata**, col proprietario come chiusore, accanto a V36 e Q22 della 6.5 del disegno |
| P-6 | la riga 3 e la riga 8 della sezione 2 nominano i due vicoli ciechi; le stesse frasi vivono anche nello **scartato** delle risposte 3 e 10 del disegno del 2026-09-04 | il richiamo di quelle due righe copre risposta e scartato, blocco E3 |
| P-7 | cinque ADR — 0009, 0010, 0011, 0022, 0038 — hanno già un rimando in testa, e **nessun** ADR ne ha ancora due | il secondo va sotto il primo, una riga vuota in mezzo, prima di `## Context` — D5 |
| P-8 | la `Date` di un ADR è il giorno in cui il file nasce: cinque su cinque — 0029, 0036, 0037, 0038, 0039 —, col `git log --diff-filter=A` | ADR-0040 porta la data dell'esecuzione del compito 1 — D2 |
| P-9 | in `AVVIO-CHAT.md` il totale vive **dentro** il messaggio recintato, e la guardia toglie solo i code span | il richiamo che cita la frase vecchia la mette in un code span, `le 39 ADR` — blocco E1 |
| P-10 | la riga *«Ultimo aggiornamento»* di `roadmap.md` si riallinea in ogni commit che tocca il file — P-8 del piano del 2026-09-04 | i blocchi E1, E5 ed E6 la prendono per prefisso, `>=`, e la riscrivono |
| P-11 | il conto per stato di `tracciabilita.md` rende `47 · 54 · 76 · 0 · 1` — il comando del suo riquadro | nessuna delle undici righe cambia stato: il conto resta identico dopo il compito 5 |
| P-12 | in un'etichetta `mermaid` il segnaposto `.<nomeapp>/` sarebbe letto come un tag HTML, e il repository scrive le etichette senza apostrofi | i diagrammi dicono *«la cartella nascosta alla root»*; i blocchi E4 non portano né parentesi angolari né apostrofi nelle etichette |
| P-13 | le due tabelle delle voci aperte della porta di qualità, lette sulla colonna di chi le chiude: **nessuna** ha questo piano o *«il proprietario, prima»* come chiusore; la T5-34 è la riga stantia di E94, la contraddizione C-S5-3 | nessuna voce sbarra il piano |
| P-14 | il comando della guardia dei totali rende otto righe — quattro in `HANDOFF.md`, una in `roadmap.md`, due nel compendio, una in `AVVIO-CHAT.md` — tutte a trentanove | il blocco E1 le porta a quaranta, e in `AVVIO-CHAT.md` toglie la cifra: la risposta A alla prima domanda della 6.1 |

**Le decisioni prese scrivendo, col perché** — sono del coordinatore, e il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| D1 | **sei compiti**: 1, ADR-0040 e ciò che il cancello pretende con lui — il rimando in ADR-0022, l'indice, la voce, i totali, la §13, `CLAUDE.md`, `AVVIO-CHAT.md`; 2, i rimandi in testa a dieci ADR con le loro voci; 3, il disegno del 2026-09-04 e la cattura in tutte le sue case — ADR-0039, la sua voce, il disegno dei gesti; 4, la spec, design/09 e design/10; 5, roadmap, tracciabilità e stella polare; 6, la chiusura | `check-docs.sh` è rosso finché ADR-0040 non ha voce, riga d'indice e totali, quindi nascono in un commit; un rimedio si chiude su tutte le case della frase in un commit — la cattura. Costo: compiti più grandi di quelli del 2026-09-04 |
| D2 | ADR-0040 si chiama `0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md`, col titolo della 3.2, e la sua `Date` è il giorno del compito 1 | P-8; la 3.2 lascia titolo e nome al piano. Costo: un rinomino |
| D3 | la testa di ADR-0040 porta un quarto punto, *«Modifica»*, accanto a `Status`, `Date` e `Deciders`; il rimando di ADR-0022 finisce con *«Le altre righe reggono»* invece della frase sulla non-superazione | la forma *«Amends»* di `adr-tools`, letta per la 3.3, sta nella testa; la 3.3 dice *«al posto della frase»*. Costo: una riga |
| D4 | la riga di `CLAUDE.md` nomina il terzo caso con la forma di ADR-0040 su ADR-0022 | la 3.3. Costo: una frase |
| D5 | un secondo rimando in testa va sotto il primo, prima di `## Context` | P-7: l'ordine delle date. Costo: nessuno |
| D6 | nel disegno del 2026-09-04 ogni richiamo va **nella cella o nel capoverso** che porta le parole superate; una riga della sezione 2 che nomina più posti ne dà uno a ciascuno; la tabella della 4.2 ne riceve uno solo, nella riga dei nodi, che nomina frecce e segnali | la sezione 2 del disegno, *«nella riga stessa»*. Costo: molti richiami corti |
| D7 | il richiamo in testa al disegno del 2026-09-04 va subito sotto il capoverso di stato | chi apre il file deve leggerlo per primo. Costo: nessuno |
| D8 | design/10 entra nel compito 4, e design/09 si riscrive sul precedente del 2026-09-08 — richiamo in testa alla sezione, diagramma e tabella —, col percorso della root nella configurazione, il punto 1 di ADR-0040 | P-2; la 5.4 vuole design/09 *«riallineato ad ADR-0040»*. Costo, se il proprietario lo vuole fuori: un blocco da togliere |
| D9 | il modulo Backup entra nel compito 5 | P-3. Costo: un richiamo da togliere |
| D10 | la riga della data del compendio si riscrive al compito 1 — per il piano in esecuzione — e al 6, e ogni volta la riga di prima va in archivio con `archive_head.py`; i compiti 2 e 3 non la toccano | il precedente del 2026-09-04: il suo compito 2 non la toccò. Costo: fra il 2 e il 6 la data può restare indietro di qualche giorno, e la riga lo dice rimandando alla tabella della posizione |
| D11 | in `tracciabilita.md` i richiami si appendono all'ultima cella, e nessuno stato cambia; *«Backup ed export dei dati»* non si tocca | *«solo l'irriproducibile»* resta vero: dice *solo*, non *tutto*. Costo: una riga, se il proprietario la vuole |
| D12 | nella §12 del compendio il disegno e il piano entrano nella riga della knowledge base, non in una riga loro | il tetto. Costo: una riga lunga |
| D13 | in `README.md` il disegno entra nella tabella «Specifiche», sotto la riga del disegno del 2026-09-04 | la forma dei disegni. Costo: nessuno |
| D14 | `riferimenti.md` non si tocca | le fonti della 3.4 ci sono già, P-1. Costo: nessuno |
| D15 | tre attrezzi al posto di `replace_unique.py`: `apply_edits.py`, che applica un blocco tutto o niente e si prova prima col `--check`; `archive_head.py` e `replace_pointer.py`, per la testa del compendio | una novantina di modifiche puntuali: le coppie scritte a mano sarebbero state una novantina di occasioni di sbagliare, e il `--check` le ha provate tutte sul repository vero. Provati nelle due direzioni su file di prova, e su un file CRLF — CR uguali alle righe. Costo: tre attrezzi da leggere |
| D16 | il *«Come si riprende»* del disegno resta nel disegno; questo piano ha il suo | il precedente dei disegni del 2026-09-03, del 2026-09-04 e del 2026-09-22: un disegno tiene la sua ultima chiusura. Costo: nessuno |
| D17 | il dispaccio dell'esecuzione viaggia con git, in `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti-esecuzione/`, creata al primo dispaccio | il punto 8 del piano del design system, richiesta del proprietario del 2026-09-24. Costo: una cartella |
| D18 | la cella *«Dipende da»* della riga 11 della roadmap si riscrive, *«6, 9»*, col richiamo che cita *«5, 6, 9»* | la 5.1: nelle tabelle di stato la cella si riscrive. Costo: nessuno |

**Vicoli ciechi di questa sessione:**

| | Che cosa insegna |
|---|---|
| un rattoppo di `apply_edits.py` passato in un heredoc di Bash con dentro `\r\n` si è fermato su un'asserzione, senza scrivere niente | 📌 *Un file di attrezzi si riscrive per intero con lo strumento di scrittura, mai rattoppato da un heredoc: i backslash non sopravvivono al canale* — la memoria lo diceva |
| la prima stesura del blocco di design/10 ancorava a una recinzione `mermaid`, e quel file ne ha **due** | 📌 *Un'ancora sulla prima riga di un blocco ricorrente non è unica: si ancora all'intestazione della sezione* |
| `rev` non esiste nel Git Bash di questa macchina | le code di riga si stampano con Python |

**Le voci aperte che il piano sa, e non chiude:** AUD-004, che sbarra il 13 e non il piano; X-2 e X-4 dell'audit; le voci
della 4.6 e della 6.5 del disegno, ciascuna col suo chiusore; e P-5, nuova, col proprietario come chiusore.

## Il materiale verificato

Parola per parola com'era nello scratchpad alla chiusura, e controllato su `0c0d0d4` come dice la consegna. ⛔ **Chi
riprende lo estrae, non lo riscrive:** ogni recinto è un file.

## La consegna al pre-controllo, del 2026-09-29

Tolta dal piano il 2026-09-29, alla chiusura del pre-controllo, quando la consegna all'esecuzione ne ha preso il
posto. Parola per parola, coi link riscritti per questa cartella.

## Come si riprende — scritto alla chiusura della sessione che ha finito il piano, il 2026-09-29, coi comandi

⛔ **Da sapere subito: niente è a metà.** Il piano è **scritto**, e i sei compiti sono stati simulati **in sequenza** su un
`git worktree` nello scratchpad, poi tolto — P-19. **Non è pre-controllato.** Tutto è pushato: si riparte anche dall'altra
macchina, dopo il fetch. La chiusura precedente — la sospensione della prima sessione — sta in
[archivio](consegna-piano-knowledge-base-revisione-documenti.md), parola per parola.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, `git status -sb`; nessuno stash, e nessun worktree oltre al principale — `git worktree list` |
| codice di prodotto | **non toccato**: il comando D da `<base>` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK` all'apertura e prima del commit di questa chiusura: si rilanciano, non si citano |
| il margine del compendio | il comando C |
| i pezzi | `extract.py` copiato a mano dal suo recinto, poi il comando della sezione *«Gli attrezzi»*: diciotto righe |
| i blocchi | il `--check` di ciascuno, dal comando sotto la tabella: su questo commit tutti e sei `checked`, perché la chiusura ha scritto le due ancore del blocco E6 |
| il puntatore | la §6 del compendio: il piano è scritto, e viene il suo **pre-controllo** |

```bash
S=<scratchpad>; for n in 1 2 3 4 5 6; do printf 'E%s: ' $n; PYTHONIOENCODING=utf-8 python "$S/apply_edits.py" --check "$(date +%F)" "$S/e$n.txt" 2>&1 | tail -1; done
```

⏳ **Una domanda aperta, da porre per prima:** il **passo visivo** del compito 4 — la riga 🆕 della tabella *«Le voci
aperte che questo piano SA»*. Posta il 2026-09-29 in A/B, col consiglio A, e rimandata dal proprietario: si ripone, e se la
risposta è A il passo entra nel compito 4 come voce d'errata, col primo disegno dei due diagrammi fatto nel pre-controllo.

**Il compito della sessione che riprende — il pre-controllo, con le quattro domande di `CLAUDE.md`:**

1. La lettura d'apertura di `CLAUDE.md`; poi il disegno della revisione **per intero**, a blocchi; poi **questo piano per
   intero**.
2. `bash scripts/gate.sh`, da solo; il comando C; l'estrazione dei pezzi e il `--check` dei sei blocchi.
3. Per ogni compito, nell'ordine: le **quattro domande** — la sonda sbagliata, la sonda che manca, l'artefatto sbagliato,
   il compito già eseguito — e le righe 5–8 della tabella di `CLAUDE.md`, prima fra tutte: *il compito si legge contro i
   documenti di ADESSO*. Qui l'artefatto sbagliato è un **testo** che dice il falso contro il documento che lo riceve, o
   contro i fratelli di un ADR: si legge ogni rimando e ogni richiamo nel posto dove atterra. Ogni difetto è una voce
   d'errata, col testo corretto, **prima** di dispacciare; se una voce cambia un blocco, la simulazione in sequenza si
   rifà — P-19.
4. La cartella del dispaccio — D17 — e i modelli dei prompt; il costo di ciascun subagente, detto al proprietario **prima**
   con la banda misurata dei dispacci recenti, e il sì.
5. La chiusura della sessione: la riga in testa alla tabella della posizione — *«pre-controllo fatto»*, restando una riga
   che comincia con `✅ **IL PIANO È`, che il blocco POS6 cerca —; il puntatore della §6 mosso all'esecuzione, con
   `archive_head.py --pointer` e `replace_pointer.py`, e la riga della data; questa sezione riscritta come consegna
   all'esecuzione, e questa in archivio, nello stesso file della chiusura precedente.

**Le decisioni prese in questa sessione** sono D19…D25, e ciò che ha trovato P-15…P-19: stanno nelle due tabelle in testa,
e qui non si ricopiano. In più, fuori da questo file: il puntatore della §6 e la riga della data del compendio, con la
loro copia in archivio; la riga di questo piano in `roadmap.md`, con l'*«Ultimo aggiornamento»*; nel disegno della
revisione, il punto 3 della 6.6 e il richiamo sul comando A della 6.2 — D24.

**Vicoli ciechi di questa sessione:**

| | Che cosa insegna |
|---|---|
| la prima corsa della simulazione stampava tutto il controllo delle tabelle a ogni compito: le stesse trentaquattro righe preesistenti, sette volte | 📌 *Una sonda che rende righe preesistenti si confronta per conto, per file* — D25 |
| un `grep -c` con una barra rovesciata finale costruita con `printf` è uscito con *«Trailing backslash»* | un conto di caratteri si fa con Python, `chr(92)`, non con un `grep` costruito in linea — trappola 13 |

**Da verificare alla fonte prima del pre-controllo:** niente di esterno.
