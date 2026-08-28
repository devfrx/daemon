# Sfoltimento del compendio — il disegno

- **Data:** 2026-08-28
- **Stato:** approvato a voce dal proprietario il 2026-08-28, prima della scrittura
- **Specie del lavoro:** rimedio strutturale su documenti. **Nessuna riga di prodotto.**

⚠️ **Questo documento è anche il primo esempio della regola che introduce.** Se cresce oltre
dodici kilobyte, ha fallito prima di essere eseguito.

---

## 0. Il problema, misurato il 2026-08-28

Il costo di **apertura di sessione** è la lettura che `CLAUDE.md` dichiara obbligatoria.
Misurato con `tiktoken`, codifica `cl100k_base`:

| File | token |
|---|---|
| `CLAUDE.md` | 5 126 |
| `docs/COMPENDIO.md` | **207 603** |
| `docs/audit-2026-08-27.md`, testa | 18 266 |
| **totale** | **≈ 231 000** |

⚠️ **Il tokenizzatore è quello di OpenAI**, l'unico presente sulla macchina. Su italiano con
emoji il conto di Claude è più alto — indicativamente del 10–20% — quindi le cifre sono un
**limite inferiore**. Il **rapporto** fra prima e dopo non dipende dal tokenizzatore.
Il comando che le rifà sta in §8.

Dentro il compendio il peso è concentrato in tre sezioni:

| Sezione | token | quota |
|---|---|---|
| §12 «Dove guardare» | **89 109** | 43% |
| §6 «Dove siamo» | **69 483** | 34% |
| §9 «Gli ottantadue gotcha» | **31 578** | 15% |
| le altre **dieci** sezioni insieme | 16 656 | 8% |

📌 **Le decisioni del progetto stanno nell'8%.** Il 92% è storia.

⛔ **Una cifra dichiarata sbagliata, e va scritta perché è la stessa malattia:** la testa di
`CLAUDE.md` prezzava questa lettura *«circa venticinquemila token»* sulla base di una misura
mai rifatta. Il rapporto che la sosteneva — `25148` token per quattrocento righe — dà
**1,02 caratteri per token**, che è impossibile per qualunque testo. La misura era rotta, non
solo stantia.

---

## 1. La causa, e perché spostare non basta

Le tre sezioni grasse non sono cresciute per caso. Le fa crescere una regola di `CLAUDE.md`:
**ogni correzione registra perché il numero vecchio era sbagliato**, con richiamo datato.

La regola è buona e non si tocca. Ciò che manca è la sua metà: **dove** finisce quel verbale.
Oggi finisce nel file che si legge a ogni sessione, quindi ogni correzione tassa ogni sessione
futura, per sempre.

⛔ **Un intervento che sposta 555 KB e lascia la regola com'è è pigrizia con l'aspetto del
lavoro:** in poche settimane il compendio torna dov'era. Il disegno vale solo se porta anche
il **freno**, ed è la §4.

---

## 2. Cosa si fa — tre sezioni, tre regole già scritte in questo repository

Nessuna regola nuova. Ogni sezione grassa viola una regola che il repository ha già
formulato, e che non ha mai applicato a sé stesso.

### 2.1 §9 — è una **seconda casa**, e si toglie

Gli 82 gotcha vivono in `docs/HANDOFF.md`, sezione «I gotcha»: una tabella di **82 voci**.
La §9 del compendio ne tiene **82** in prosa, per 31 578 token — e la propria seconda riga
dichiara: *«Il testo completo, con le misure, è in `HANDOFF.md`»*.

Ammette di essere un rimando, e pesa quanto la fonte.

| | |
|---|---|
| Regola violata | *un puntatore che vive in PIÙ documenti si **toglie**, non si ricorregge* (`CLAUDE.md`) |
| È il gotcha | **#68** — la regola violata dentro il documento che la contiene |
| Ed è la radice | **R3** dell'audit del 2026-08-27, in corso |
| Rimedio | la §9 diventa **un rimando** a `HANDOFF.md` più la regola di lettura. Bersaglio: **≤ 600 token** |
| Si archivia? | **No.** Togliere un duplicato non perde niente: la fonte resta dov'era |

### 2.2 §12 — è un **diario di dimensioni**, e il numero non si scrive

La §12 è **4 KB di tabella dei puntatori** più **~248 KB di verbali**: cinquantanove «misure»
che registrano di quanto sono cresciuti i file (*«spec `263 → 266`, HANDOFF `105 → 109`»*).

| | |
|---|---|
| Regola violata | *un **comando** non marcisce* — formulata dal repository stesso: `grep -rc 'comando non marcisce' docs/ CLAUDE.md` |
| Rimedio | resta la **tabella dei puntatori**, **senza la colonna dei kilobyte**, col comando che la rifà. Bersaglio: **≤ 2 000 token** |
| Si archivia? | **Sì** — i cinquantanove verbali sono datati e legittimi: vanno in archivio, non si cancellano |

📌 La colonna dei pesi esisteva per decidere *se aprire un file*. Quella decisione la prende
un `wc -c` al momento, e non marcisce.

### 2.3 §6 — è lo **stato di oggi** sepolto sotto tutti gli stati di ieri

| | |
|---|---|
| Regola violata | *un file chiuso non è una lettura obbligatoria* — già applicata a `audit-2026-08-11.md` |
| Rimedio | la §6 tiene **solo lo stato corrente e il prossimo passo**. Bersaglio: **≤ 4 000 token** |
| Si archivia? | **Sì** — i verbali passati sono datati, e vanno in archivio |

⛔ **Il criterio di taglio, perché non sia a occhio:** resta in §6 ciò che è **vero adesso**.
Esce ciò che dice *«questa riga diceva X, ed era falso»*. La seconda specie è un **verbale**,
e un verbale invecchia onestamente **in archivio**.

---

## 3. Dove va la storia

| File nuovo | Contenuto | Origine |
|---|---|---|
| `docs/archivio/misure-dimensioni.md` | le 59 misure | §12 |
| `docs/archivio/stato-storico.md` | i verbali di stato passati | §6 |

⚠️ **`docs/archivio/` non è una lettura obbligatoria e non lo diventerà.** Il compendio vi
rimanda da §6 e §12, una riga per file. Chi non ha una domanda storica non lo apre mai.

⛔ **Nessun contenuto si cancella**, tranne il duplicato della §9 — che non è un contenuto ma
una copia. È la stessa distinzione con cui il repository tratta gli ADR: `Superseded by` per
ciò che è superato, cancellazione mai.

---

## 4. Il freno — perché non ricresca

Due aggiunte, ed è la parte che rende il lavoro definitivo invece che rimandato.

### 4.1 Una regola in `CLAUDE.md`

> **Un numero misurato non si scrive in un documento: si scrive il comando che lo produce.**
> Se il numero deve comparire — perché sostiene una decisione — porta accanto il comando e
> la data, e vive in **una** casa sola.

E la sua metà mancante:

> **Un verbale di correzione non resta nel documento corretto:** va in `docs/archivio/`, con
> la data. Il documento vivo porta ciò che è vero adesso.

### 4.2 Un tetto nel cancello

`scripts/check-docs.sh` guadagna un controllo:

| | |
|---|---|
| Asserzione | `docs/COMPENDIO.md` **non supera 80 KB** (byte, `wc -c`) |
| Perché una soglia e non un ideale | *«un principio che non si può controllare è un'intenzione. Gli invarianti diventano test»* (`CLAUDE.md`) |
| Perché **80** e non 65 | il bersaglio è ~65 KB: la soglia lascia margine di crescita legittima e scatta **prima** che il file torni illeggibile. Una soglia che scatta al primo commit insegna a ignorarla |
| Due direzioni (obbligatorie) | ① scatta con un file sopra soglia; ② **non** scatta col file di oggi. La seconda si dimentica |
| Non-vacuità | file mancante → **rosso**, non verde (gotcha #26) |

⚠️ **La soglia è un tetto, non un obiettivo.** Non autorizza a crescere fino a 80.

---

## 5. Vincoli da non rompere, misurati sul cancello

Dal blocco che in `scripts/check-docs.sh` comincia a `compendium=docs/COMPENDIO.md`.
⚠️ **Niente numeri di riga:** si spostano, ed è la malattia che questo documento cura.

| # | Vincolo | Conseguenza sul disegno |
|---|---|---|
| 1 | i titoli `## 5. ` e `## 6. ` devono esistere in `COMPENDIO.md` | **§5 e §6 restano dove sono, coi loro numeri.** La §6 dimagrisce, non si sposta |
| 2 | ogni ADR di `docs/adr/` ha la sua voce `**NNNN — ` in §5, e viceversa | **§5 non si tocca** (8 902 token, restano) |
| 3 | i conteggi di ADR scritti in prosa devono essere veri, in sei file | ogni riscrittura ricontrolla di non aver mosso un numerale |

⛔ **La numerazione delle sezioni non cambia.** Rinumerare romperebbe i delimitatori del
cancello e ogni rimando incrociato del repository.

---

## 6. Perimetro — cosa NON si tocca

| | Perché |
|---|---|
| `docs/audit-2026-08-27.md` | è il **compito aperto**. Si sfoltisce da sé chiudendosi |
| `docs/HANDOFF.md` | è la **casa** dei gotcha. Il disegno gliene manda altri, non gliene toglie |
| i piani e i disegni dei traguardi | sono **verbali di esecuzione**, e l'audit li dichiara fuori perimetro |
| `crates/` | **nessuna riga di prodotto**. Il cancello deve restare verde per invarianza, non per correzione |
| i plugin e i server MCP dell'ambiente | **fuori perimetro, e registrato.** ⚠️ **~170 000 token è una SOTTRAZIONE, non una misura:** 400 000 osservati meno 230 995 misurati. Resta un'ipotesi finché un contatore dell'ambiente non la conferma — ma anche sbagliata di metà pesa quanto tutto ciò che questa spec toglie, e si spegne dalla configurazione, non da qui |

---

## 7. I costi accettati

| Costo | Perché si accetta |
|---|---|
| **Chi cerca un fatto storico deve aprire un secondo file** | è il punto: la storia si consulta, non si rilegge a ogni sessione |
| **Due file nuovi in `docs/`** | il repository ne ha 67; due in più che nessuno apre pesano meno di 555 KB che tutti aprono |
| **Il tetto può diventare rosso su una crescita legittima** | è il comportamento voluto: costringe a decidere *dove* va il testo nuovo, invece di lasciarlo cadere nel compendio |
| **La soglia di 80 KB è un giudizio, non una misura** | dichiarato come tale. Si sposta con un ADR se il progetto dimostra che serve |
| **Il taglio di §6 e §12 è una lettura, e una lettura può sbagliare** | mitigato: **niente si cancella**, tutto va in archivio. Un errore di taglio si annulla spostando indietro un blocco |
| **Una sessione va spesa senza risultato visibile** | il ritorno è misurabile alla prima sessione dopo: §8 |

---

## 8. Come si verifica

⛔ **Nessuna di queste righe è un'evidenza finché non è stata eseguita.** Si misura, e dove
diverge **si registra la divergenza** invece di allinearsi all'attesa.

| # | Controllo | Comando | Atteso |
|---|---|---|---|
| 1 | il cancello resta verde | `bash scripts/gate.sh` | `GATE GREEN`, exit 0 |
| 2 | il codice non è stato toccato | `git diff --stat -- crates/` | **vuoto** |
| 3 | il tetto scatta | file di prova sopra soglia | **rosso** |
| 4 | il tetto non scatta a torto | file di oggi | **verde** |
| 5 | la lettura obbligatoria è scesa | comando qui sotto | **≤ 26 500** token (i due file che il comando somma) |
| 6 | i fine-riga non sono cambiati | `git ls-files --eol docs/`, colonna `w/` | invariata |
| 7 | niente è andato perso | `wc -c docs/archivio/*.md` contro il taglio | somma coerente |

Il comando della riga 5, che è anche quello che rifà la §0:

```
python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print(sum(len(e.encode(io.open(p,encoding='utf-8',errors='replace').read())) for p in ['CLAUDE.md','docs/COMPENDIO.md']))"
```

⚠️ **La riga 6 non è formale.** Uno script che riscrive un sorgente ne normalizza i fine-riga
senza dirlo, ed è successo **tre volte** in questo repository. Qui si riscrivono file grandi
con strumenti: è la condizione esatta in cui il difetto si presenta.

---

## 9. Bersagli numerici

| | oggi | bersaglio |
|---|---|---|
| §6 | 69 483 | ≤ 4 000 |
| §9 | 31 578 | ≤ 600 |
| §12 | 89 109 | ≤ 2 000 |
| `COMPENDIO.md` | 207 603 | **≤ 24 000** |
| `CLAUDE.md` | 5 126 | **≤ 2 500** |
| **lettura obbligatoria**, audit aperto | 230 995 | **≤ 45 000** |
| **lettura obbligatoria**, audit chiuso | — | **≤ 26 500** |

⛔ **Il bersaglio del compendio è DERIVATO, non scelto**, e la derivazione va scritta o è un
desiderio: le **dieci** sezioni che non si toccano pesano **16 656** token misurati, più i tre
bersagli di §2 — `4 000 + 600 + 2 000` — fa **23 256**. Il bersaglio è **24 000** e non 23 000,
perché 23 000 sarebbe **irraggiungibile senza toccare le dieci sezioni**, che il perimetro
esclude. ⚠️ **Questa riga è nata da una revisione di questo stesso documento**, dove il
bersaglio diceva 23 000 e la somma lo smentiva: è il gotcha **#31** colto dentro la spec che
lo prende di mira.

⚠️ **Sono bersagli di progetto, non misure.** Si verificano col comando della §8, e se il
risultato diverge si scrive di quanto — non si riscrive il bersaglio.
