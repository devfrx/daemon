# Sfoltimento del compendio — piano di esecuzione

> **Per chi esegue:** SKILL RICHIESTA — `superpowers:subagent-driven-development` (consigliata)
> oppure `superpowers:executing-plans`, un compito per volta. I passi usano le caselle
> `- [ ]` per il tracciamento.

**Obiettivo:** portare la lettura obbligatoria di apertura sessione da **230 995** a
**≤ 45 000** token, senza perdere una decisione e senza toccare una riga di prodotto.

**Disegno:** [`specs/2026-08-28-sfoltimento-compendio-design.md`](../specs/2026-08-28-sfoltimento-compendio-design.md).
Si legge **prima** del primo compito, per intero: sono 12 KB.

**Approccio:** tre sezioni del compendio si sfoltiscono **dal basso verso l'alto** — §12,
poi §9, poi §6 — così i numeri di riga delle sezioni non ancora toccate **non si spostano**.
La storia va in `docs/archivio/`. Poi la regola in `CLAUDE.md` e il **tetto** nel cancello,
che è ciò che impedisce la ricrescita.

**Strumenti:** `python` (presente, con `tiktoken`), `git`, `bash`. ⛔ **Le riscritture di file
si fanno in `python` con `newline=''`**, non con `sed -i` né con heredoc di shell: gli
apostrofi italiani rompono il quoting, e `sed -i` normalizza i fine-riga senza dirlo.

---

## ⛔ Errata — si legge PRIMA di ogni compito

Quattro voci, tutte misurate eseguendo il Compito 1 il 2026-08-28.

| | La voce |
|---|---|
| **E1** | ⛔ **`git checkout -- <file>.md` RIPORTA IL FILE A CRLF, e questo repository lo vuole LF.** Misurato: `docs/COMPENDIO.md` è passato da `623516` byte e `CR=0` a `628803` e `CR=5287` in un solo comando, perché `core.autocrlf=true`. ⛔ **Per annullare una modifica si usa una COPIA nello scratchpad**, non `git checkout`. Se è già successo, si ripara leggendo con `newline=''` e riscrivendo con `\n` — e si **rimisura** |
| **E2** | ⚠️ **I link relativi del testo spostato si rompono, tutti.** Il testo viveva in `docs/` e va in `docs/archivio/`: ogni bersaglio relativo guadagna un `../`. Senza, `check-docs.sh` riporta **506 link rotti**. Vale per **ogni** compito che sposta testo in archivio — quindi anche il **Compito 3** |
| **E3** | ⚠️ **I riquadri di §12 sono SESSANTA, non cinquantanove.** Il disegno e questo piano dicevano 59, contati con un pattern che pretendeva la parola *«misura»* dopo l'ordinale: un riquadro si intitola *«Rimisurati lo stesso giorno»*. È il gotcha **#31** commesso dentro il lavoro che lo prende di mira. Il conto lo dà `grep -cE '^> 🔁 \*\*'`, e il numerale **esce** dai documenti vivi |
| **E4** | ⛔ **La coda di §12 NON era tutta storia.** Le ultime quattro righe vive contenevano un'istruzione operativa — *«Prima di ogni commit di documentazione: `bash scripts/check-docs.sh`»*. Un taglio *«dalla prima misura fino a §13»* l'avrebbe archiviata. ⛔ **Il confine è l'ULTIMA riga di citazione**, non l'inizio della sezione successiva. 📌 Lo stesso sospetto va portato al **Compito 3**, dove il blocco da spostare è quaranta volte più grande |

⚠️ **E il baseline del Compito 5 è cambiato:** sciolta la Precondizione con l'opzione A, il
compendio di prima dello sfoltimento è **`f19f0cb`** e fa **623 516** byte — non `d902c40` con
612 617. Il passo 2 di quel compito usa `f19f0cb`.

---

## Vincoli globali

Valgono per **ogni** compito. Non si ripetono nei passi.

| # | Vincolo | Come si verifica |
|---|---|---|
| 1 | ⛔ **I fine-riga si conservano _per file_** | `tr -cd '\r' < <file> \| wc -c` **prima e dopo**. Valori attesi: `docs/COMPENDIO.md` **0**, `CLAUDE.md` **0**, `docs/HANDOFF.md` **0**, `scripts/check-docs.sh` **314** |
| 2 | **La numerazione delle sezioni non cambia** | i titoli `## 1.` … `## 13.` restano tutti, con gli stessi numeri |
| 3 | **§5 non si tocca** | `git diff` non mostra righe fra `## 5. ` e `## 6. ` |
| 4 | **Nessuna riga di prodotto** | `git diff --stat -- crates/` **vuoto** a ogni commit |
| 5 | **Il cancello resta verde** | `bash scripts/gate.sh` → `GATE GREEN`, exit 0 |
| 6 | **L'audit documenti resta verde** | `bash scripts/check-docs.sh` → `OK — no inconsistencies` |
| 7 | **Documentazione in italiano, codice in inglese** | §1.0 della spec del sotto-progetto 1 |
| 8 | ⛔ **Ogni cifra scritta porta il comando che la produce** | è la regola che questo piano installa; vale anche sul piano stesso |
| 9 | **Commit e push a ogni compito chiuso, senza co-autore** | `CLAUDE.md` |

⛔ **Niente `git add -A` e niente `git commit -a`.** Ogni commit nomina i propri file: nell'albero
c'è lavoro di un'altra voce (vedi Precondizione), e travolgerlo è irreversibile.

---

## Precondizione — da sciogliere PRIMA del Compito 1

⛔ **L'albero di lavoro non è pulito.** Ci sono **139 righe in 6 file**, ed è il rimedio di
**AUD-013** — completo, con la misura dentro, e mai committato. `CLAUDE.md` prescrive di
committare alla chiusura di ogni voce; non è stato fatto.

```bash
git status --short && git diff --stat
```

| Opzione | |
|---|---|
| **A (consigliata)** | committare AUD-013 come voce propria, **prima** di cominciare. Il compendio è fra i file toccati: sfoltirlo sopra un diff non committato mescola due lavori in un diff solo, e il secondo diventa irrevisionabile |
| **B** | riporre con `git stash` e riapplicare alla fine — ⚠️ rischioso: il Compito 3 riscrive proprio le righe che lo stash toccherebbe, e il conflitto sarebbe su 97 righe di prosa |

⛔ **Decisione del proprietario.** Non si comincia finché non è sciolta.

---

## Struttura dei file

| File | Sorte |
|---|---|
| `docs/COMPENDIO.md` | **modificato** — §6, §9, §12 sfoltite. §1–5, 7, 8, 10, 11, 13 intatte |
| `docs/archivio/misure-dimensioni.md` | **creato** — le misure storiche di §12 |
| `docs/archivio/stato-storico.md` | **creato** — i verbali di stato di §6 |
| `CLAUDE.md` | **modificato** — regola nuova, testa sfoltita |
| `scripts/check-docs.sh` | **modificato** — il tetto, in fondo |
| `docs/HANDOFF.md` | **letto**, non modificato — resta la casa dei gotcha |
| `crates/**` | **intoccato** |

**Confini misurati il 2026-08-28**, e ⚠️ **si rimisurano prima di ogni compito** — il file
si accorcia man mano:

```bash
grep -n '^## ' docs/COMPENDIO.md
```

| Sezione | righe oggi | prima riga da tagliare |
|---|---|---|
| §6 | 587–2329 | **648** (dopo *«audit sezione-contro-ADR passato»*) |
| §9 | 2381–2573 | **2383** (dopo il titolo e la riga di rimando) |
| §12 | 2641–5253 | **2681** (la prima riga `> 🔁`) |

---

## Compito 1 — §12: la tabella resta, le misure vanno in archivio

**File:**
- Modifica: `docs/COMPENDIO.md`, §12
- Crea: `docs/archivio/misure-dimensioni.md`

**Interfacce:**
- Consuma: niente (primo compito)
- Produce: `docs/archivio/` esiste; §12 finisce con un rimando a `misure-dimensioni.md`

- [ ] **Passo 1: misurare il prima**

```bash
python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print('token:', len(e.encode(io.open('docs/COMPENDIO.md',encoding='utf-8').read())))" && printf 'byte: %s  CR: %s\n' "$(wc -c < docs/COMPENDIO.md)" "$(tr -cd '\r' < docs/COMPENDIO.md | wc -c)"
```

Atteso: `token: 207603`, `byte: 623516`, `CR: 0`.

⛔ **BYTE E CARATTERI NON SONO LA STESSA COSA, e questo piano ci è già cascato una volta:**
`wc -c` conta **byte** — 623 516 — mentre `len()` in python conta **caratteri** — 606 513. La
differenza è di **quasi 17 000**, perché emoji e lettere accentate occupano più di un byte in
UTF-8. ⚠️ **Il tetto del Compito 5 è espresso in BYTE**, perché lo misura `wc -c`.

⚠️ **Se i numeri divergono, NON si prosegue allineandosi:** si scrive di quanto e si chiede.
Divergono di sicuro se la Precondizione è stata sciolta con l'opzione A.

- [ ] **Passo 2: trovare il confine, invece di fidarsi del numero qui sopra**

```bash
awk '/^## 12\. /{s=NR} s&&/^> 🔁/{print "prima misura: riga "NR; exit}' docs/COMPENDIO.md
awk '/^## 13\. /{print "fine §12: riga "NR-1; exit}' docs/COMPENDIO.md
```

- [ ] **Passo 3: tagliare, in python, conservando i fine-riga**

Salvare come `taglia12.py` **nello scratchpad della sessione**, mai dentro il repository —
`CLAUDE.md`: *«Le misure nello scratchpad, non nel repository, e si ripulisce dopo»*. Poi
eseguire con `python <percorso-scratchpad>/taglia12.py` dalla radice del repository:

```python
import io
P = 'docs/COMPENDIO.md'
src = io.open(P, encoding='utf-8', newline='').read().split('\n')

start = next(i for i, l in enumerate(src) if l.startswith('## 12. '))
end   = next(i for i, l in enumerate(src) if l.startswith('## 13. '))
first = next(i for i in range(start, end) if src[i].startswith('> 🔁'))

misure = src[first:end]
tenuto = src[:first]

rimando = [
    '⚠️ **I pesi non stanno più in questa tabella, e non è una svista.** Un peso scritto',
    'invecchia al primo commit che tocca il file; il comando che lo produce no:',
    '',
    '```',
    "find docs -name '*.md' | xargs wc -c | sort -n",
    '```',
    '',
    '📚 **Le misure storiche** — il verbale di come i pesi sono cambiati dal',
    '2026-08-08 — stanno in [`archivio/misure-dimensioni.md`](archivio/misure-dimensioni.md).',
    '⛔ **Non è una lettura obbligatoria**: si apre solo con una domanda storica in mano.',
    '',
]

io.open(P, 'w', encoding='utf-8', newline='').write('\n'.join(tenuto + rimando + src[end:]))

io.open('docs/archivio/misure-dimensioni.md', 'w', encoding='utf-8', newline='').write(
    '\n'.join([
        '# Archivio — le misure dei pesi dei documenti',
        '',
        '⛔ **Non è una lettura obbligatoria.** Verbale datato, estratto dalla §12 del',
        'compendio il 2026-08-28 dal piano di sfoltimento. Si apre con una domanda storica',
        'in mano, non per farsi un\'idea.',
        '',
        '⚠️ **I numeri qui dentro erano veri il giorno in cui furono presi.** Il peso di oggi',
        "lo dà `find docs -name '*.md' | xargs wc -c`.",
        '',
        '---',
        '',
    ] + misure)
)
print('misure spostate:', len(misure), 'righe')
```

- [ ] **Passo 4: verificare che niente sia andato perso**

```bash
printf 'compendio ora: %s byte, CR=%s\n' "$(wc -c < docs/COMPENDIO.md)" "$(tr -cd '\r' < docs/COMPENDIO.md | wc -c)"
printf 'archivio:      %s byte, CR=%s\n' "$(wc -c < docs/archivio/misure-dimensioni.md)" "$(tr -cd '\r' < docs/archivio/misure-dimensioni.md | wc -c)"
grep -c '🔁' docs/archivio/misure-dimensioni.md
```

Atteso: **CR=0** su entrambi; **59** intestazioni `🔁` — ⚠️ `grep -c '🔁'` conta le **righe**
che lo contengono, non le intestazioni: il conto esatto è
`grep -cE '^> 🔁 \*\*' docs/archivio/misure-dimensioni.md`, e deve dare **59**.

- [ ] **Passo 5: le due porte devono restare verdi**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh 2>&1 | tail -2
```

Atteso: `OK — no inconsistencies` e `GATE GREEN`.

- [ ] **Passo 6: misurare il dopo, e scriverlo nel commit**

```bash
python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print(len(e.encode(io.open('docs/COMPENDIO.md',encoding='utf-8').read())))"
```

Atteso: **≈ 119 000** (207 603 − 89 109 + il rimando).
⚠️ È una **previsione**: si scrive il valore misurato, e se diverge si scrive di quanto.

- [ ] **Passo 7: commit**

```bash
git add docs/COMPENDIO.md docs/archivio/misure-dimensioni.md && git commit -m "docs(sfoltimento): le misure dei pesi escono dalla §12 e vanno in archivio" && git push origin spec/sottoprogetto-1-kernel
```

---

## Compito 2 — §9: il duplicato degli 82 gotcha si toglie

⛔ **Nulla va in archivio in questo compito**, e va capito prima di eseguire: la §9 è una
**seconda copia** di ciò che vive in `docs/HANDOFF.md`, sezione «I gotcha». Archiviare una
copia significherebbe averne **tre**.

**File:**
- Modifica: `docs/COMPENDIO.md`, §9
- Legge, **non modifica**: `docs/HANDOFF.md`

- [ ] **Passo 1: provare che è davvero un duplicato, prima di cancellare**

```bash
grep -n '^## I gotcha' docs/HANDOFF.md
awk '/^## I gotcha/{s=1; next} s&&/^## /{s=0} s&&/^\| [0-9]+ \|/{c++} END{print "voci in HANDOFF:", c}' docs/HANDOFF.md
awk '/^## 9\. /{s=1} /^## 10\. /{s=0} s&&/^\| *[0-9]+ *\|/{c++} END{print "voci in §9:", c}' docs/COMPENDIO.md
```

⚠️ **Il primo `awk` si ferma alla sezione successiva**, e la clausola `s&&/^## /{s=0}` non è
ornamentale: senza, conterebbe anche le righe numerate di **qualunque** tabella venga dopo.
Oggi le due forme danno lo stesso **82** — misurato — ma solo perché dopo non ce ne sono.

Atteso: **82** e **82**.
⛔ **Se i due numeri differiscono, ci si FERMA:** la §9 conterrebbe voci che HANDOFF non ha, e
cancellarla le perderebbe. In quel caso si riporta lo scarto e si chiede.

- [ ] **Passo 2: sostituire la §9 col rimando**

```python
import io
P = 'docs/COMPENDIO.md'
src = io.open(P, encoding='utf-8', newline='').read().split('\n')
s = next(i for i, l in enumerate(src) if l.startswith('## 9. '))
e = next(i for i, l in enumerate(src) if l.startswith('## 10. '))

nuovo = [
    '## 9. Gli ottantadue gotcha',
    '',
    '⛔ **La loro casa è [`HANDOFF.md`](HANDOFF.md), sezione «I gotcha»**, ed è **una sola**.',
    'Ottantadue voci in tabella, con la trappola e il perché fa male.',
    '',
    '⚠️ **Questa sezione ne portava una SECONDA copia, tolta il 2026-08-28.** Non era una',
    'sintesi: erano le stesse 82 voci, per 31 578 token — mentre la sua stessa seconda riga',
    'dichiarava che il testo completo stava in `HANDOFF.md`. È il gotcha **#68** — *un',
    'puntatore che vive in più documenti si toglie, non si ricorregge* — commesso dentro il',
    'documento che quella regola la contiene, ed è la radice **R3** dell\'audit del 2026-08-27.',
    '',
    '📌 **Quanti sono lo dice il comando, non questa riga:**',
    '',
    '```',
    "awk '/^## I gotcha/{s=1} s&&/^\\| [0-9]+ \\|/{c++} END{print c}' docs/HANDOFF.md",
    '```',
    '',
]

io.open(P, 'w', encoding='utf-8', newline='').write('\n'.join(src[:s] + nuovo + src[e:]))
print('§9 sostituita')
```

- [ ] **Passo 3: verificare**

```bash
printf 'CR=%s\n' "$(tr -cd '\r' < docs/COMPENDIO.md | wc -c)"
awk '/^## 9\. /{s=1} /^## 10\. /{s=0} s' docs/COMPENDIO.md | wc -l
grep -c '^## ' docs/COMPENDIO.md
```

Atteso: `CR=0`; §9 sotto **20** righe; **13** titoli di sezione (nessuno perso).

- [ ] **Passo 4: le due porte**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh 2>&1 | tail -2
```

- [ ] **Passo 5: commit**

```bash
git add docs/COMPENDIO.md && git commit -m "docs(sfoltimento): la §9 smette di essere la seconda casa degli 82 gotcha" && git push origin spec/sottoprogetto-1-kernel
```

---

## Compito 3 — §6: lo stato di oggi resta, i verbali vanno in archivio

⛔ **È il compito con la sola vera decisione di taglio, e il criterio va applicato invece che
sentito:** resta ciò che è **vero adesso**; esce ciò che dice *«questa riga diceva X, ed era
falso»*, e ogni verbale di un traguardo o di un audit **chiusi**.

**File:**
- Modifica: `docs/COMPENDIO.md`, §6
- Crea: `docs/archivio/stato-storico.md`

- [ ] **Passo 1: trovare il confine**

```bash
awk '/^## 6\. /{s=NR; print "inizio §6: "s} /^## 7\. /{print "fine §6: "NR-1; exit}' docs/COMPENDIO.md
grep -n 'audit sezione-contro-ADR passato' docs/COMPENDIO.md
```

La riga trovata dal secondo comando è **l'ultima che resta**. Tutto ciò che segue, fino a
`## 7. `, va in archivio.

- [ ] **Passo 2: leggere ciò che si sta per spostare**

```bash
awk '/audit sezione-contro-ADR passato/{s=NR} /^## 7\. /{e=NR} END{print "da spostare:", e-s-1, "righe"}' docs/COMPENDIO.md
```

⚠️ **Non è un passo formale.** Chi esegue **legge** l'inizio e la fine del blocco e conferma
che non contenga una decisione viva. Se ne trova una, la **nomina** e chiede: il piano non
autorizza a decidere da soli che una decisione è storia.

- [ ] **Passo 3: tagliare**

```python
import io
P = 'docs/COMPENDIO.md'
src = io.open(P, encoding='utf-8', newline='').read().split('\n')
keep_end = next(i for i, l in enumerate(src) if 'audit sezione-contro-ADR passato' in l) + 1
e = next(i for i, l in enumerate(src) if l.startswith('## 7. '))
storico = src[keep_end:e]

nuovo = [
    '',
    '### I sei traguardi del sotto-progetto 1',
    '',
    '| # | | |',
    '|---|---|---|',
    '| 1 | scheletro e porta di qualità | ✅ 2026-08-08 |',
    '| 2 | substrato iniettabile | ✅ 2026-08-10 |',
    '| 3 | giornale e formato durevole | ✅ 2026-08-10 |',
    '| 4 | simulatore DST — il guasto | ✅ 2026-08-11 |',
    '| 5 | arbitro GPU | ✅ 2026-08-25 |',
    '| 6 | gli altri meccanismi | ⬜ **davanti** |',
    '',
    '⚠️ **Nessun numeratore di compiti in questa tabella**, per costruzione: invecchierebbe a',
    'ogni compito. Il dettaglio di ciascun traguardo sta nel proprio piano, in',
    '[`superpowers/plans/`](superpowers/plans/).',
    '',
    '### Il prossimo passo',
    '',
    '⛔ **Chiudere i finding aperti dell\'audit del 2026-08-27**, poi il **Traguardo 6**.',
    '**Quanti siano lo dice la colonna «Stato»** di',
    '[`audit-2026-08-27.md`](audit-2026-08-27.md), che ne è la casa unica:',
    '',
    '```',
    "awk -F'|' 'NF>4{gsub(/^ +| +$/,\"\",$5); print $5}' docs/audit-2026-08-27.md | grep -c aperto",
    '```',
    '',
    '📚 **Gli stati passati** — i verbali dei cinque traguardi chiusi, dell\'audit del',
    '2026-08-11 e delle voci chiuse di quello del 2026-08-27 — stanno in',
    '[`archivio/stato-storico.md`](archivio/stato-storico.md). ⛔ **Non è una lettura',
    'obbligatoria.**',
    '',
]

io.open(P, 'w', encoding='utf-8', newline='').write('\n'.join(src[:keep_end] + nuovo + src[e:]))

io.open('docs/archivio/stato-storico.md', 'w', encoding='utf-8', newline='').write(
    '\n'.join([
        '# Archivio — gli stati passati del progetto',
        '',
        '⛔ **Non è una lettura obbligatoria.** Verbali datati, estratti dalla §6 del compendio',
        'il 2026-08-28 dal piano di sfoltimento.',
        '',
        '⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Lo stato di oggi',
        'sta nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md), e in nessun altro posto.',
        '',
        '---',
        '',
    ] + storico)
)
print('spostate:', len(storico), 'righe')
```

- [ ] **Passo 4: verificare che niente sia andato perso**

```bash
printf 'compendio: %s byte CR=%s\n' "$(wc -c < docs/COMPENDIO.md)" "$(tr -cd '\r' < docs/COMPENDIO.md | wc -c)"
printf 'archivio:  %s byte CR=%s\n' "$(wc -c < docs/archivio/stato-storico.md)" "$(tr -cd '\r' < docs/archivio/stato-storico.md | wc -c)"
grep -c '^## ' docs/COMPENDIO.md
```

Atteso: `CR=0` su entrambi; **13** titoli di sezione.

- [ ] **Passo 5: le due porte, e la misura**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh 2>&1 | tail -2
python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print(len(e.encode(io.open('docs/COMPENDIO.md',encoding='utf-8').read())))"
```

Atteso: **≤ 24 000** token. ⚠️ **Se è sopra, NON si alza il bersaglio:** si riporta di quanto
e si guarda quale sezione pesa più del previsto.

- [ ] **Passo 6: commit**

```bash
git add docs/COMPENDIO.md docs/archivio/stato-storico.md && git commit -m "docs(sfoltimento): la §6 porta lo stato di oggi, e i verbali passati vanno in archivio" && git push origin spec/sottoprogetto-1-kernel
```

---

## Compito 4 — `CLAUDE.md`: la regola che impedisce la ricrescita

**File:**
- Modifica: `CLAUDE.md`

- [ ] **Passo 1: aggiungere le due righe alla tabella «Come si lavora qui»**

Da inserire **dopo** la riga che comincia con `| ⛔ **Un puntatore o una cifra che vive in PIÙ
documenti si TOGLIE`:

```markdown
| ⛔ **Un numero misurato non si scrive: si scrive il COMANDO che lo produce** | un numero invecchia al primo commit che tocca ciò che misura; un comando no. Se il numero deve comparire — perché sostiene una decisione — porta accanto il comando e la data, e vive in **una** casa sola. ⚠️ Misurato il 2026-08-28: il 92% della lettura obbligatoria era storia di numeri corretti |
| ⛔ **Un verbale di correzione non resta nel documento corretto** | va in [`docs/archivio/`](docs/archivio/), con la data. Il documento vivo porta ciò che è **vero adesso**. È la metà mancante della riga qui sopra: senza di essa la regola dice *dove* mettere il numero e non *dove* mettere la sua storia, e la storia si accumula dove la si è corretta |
```

- [ ] **Passo 2: sfoltire la testa**

⛔ **La testa di `CLAUDE.md` è essa stessa un esempio della malattia:** porta pesi in KB,
conteggi di token e il verbale di quando quei numeri erano sbagliati. Si riscrive coi
**comandi** al posto delle cifre, e i verbali vanno in `docs/archivio/stato-storico.md`.

⚠️ **Non si tocca** ciò che dice **cosa fare**: la lista delle skill, le regole di lavoro, le
sette domande di pre-controllo. Esce solo la **contabilità**.

- [ ] **Passo 3: misurare**

```bash
python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print(len(e.encode(io.open('CLAUDE.md',encoding='utf-8').read())))"
printf 'CR=%s\n' "$(tr -cd '\r' < CLAUDE.md | wc -c)"
```

Atteso: **≤ 2 500** token, `CR=0`.

- [ ] **Passo 4: le due porte**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh 2>&1 | tail -2
```

⚠️ `check-docs.sh` controlla i conteggi di ADR dichiarati in prosa **anche in `CLAUDE.md`**:
se lo sfoltimento tocca una di quelle frasi, il rosso arriva qui.

- [ ] **Passo 5: commit**

```bash
git add CLAUDE.md && git commit -m "docs(sfoltimento): la regola che sostituisce un numero col comando, e la testa che smette di fare contabilita'" && git push origin spec/sottoprogetto-1-kernel
```

---

## Compito 5 — il tetto nel cancello, provato nelle due direzioni

⛔ **È il compito che rende il lavoro definitivo.** Senza, i quattro precedenti sono
rimandati: fra qualche settimana il compendio è dov'era.

⛔ **`scripts/check-docs.sh` è CRLF nell'albero di lavoro (CR=314) e LF nell'indice.** Un
editor o uno script che non lo sa produce un diff da **314 righe** che nessuno ha toccato. Si
misura **prima e dopo**, e si confronta col numero di righe del file.

**File:**
- Modifica: `scripts/check-docs.sh` — in fondo, prima dell'uscita finale

- [ ] **Passo 1: misurare i fine-riga prima**

```bash
printf 'CR=%s  righe=%s\n' "$(tr -cd '\r' < scripts/check-docs.sh | wc -c)" "$(wc -l < scripts/check-docs.sh)"
grep -n 'no inconsistencies' scripts/check-docs.sh
```

Atteso: `CR=314 righe=314`. Il secondo comando dà la riga **prima** della quale inserire.

- [ ] **Passo 2: riprodurre il rosso su un file vero, prima di scrivere il controllo**

Il compendio di **prima** dello sfoltimento è ancora in git. Il controllo, se fosse esistito,
lo avrebbe respinto:

```bash
git show d902c40:docs/COMPENDIO.md | wc -c
```

Atteso: **612617** byte — cioè **7,5 volte** la soglia di 81920. ⚠️ Si annota l'uscita nel
messaggio di commit: è la **riproduzione del rosso**, che questo repository pretende prima di
ogni rimedio.

⚠️ **`d902c40` è il compendio COMMITTATO, non quello dell'albero di lavoro**, che ne fa
623 516 perché porta il rimedio di AUD-013 non ancora committato. Le due cifre non vanno
confuse, ed è il motivo per cui la Precondizione va sciolta **prima**.

- [ ] **Passo 3: scrivere il controllo**

Inserire **subito dopo** il blocco `echo "== ADR still in Proposed =="` e **prima** della riga
`echo` vuota che precede `if [ "$failures" -eq 0 ]`, **conservando CRLF**.

⚠️ **Deve stare dopo la riga `compendium=docs/COMPENDIO.md`**, che è più su nel file: il
blocco riusa quella variabile invece di riscrivere il percorso in una seconda casa.

```sh
echo "== compendium size ceiling =="
# ⛔ THE CEILING EXISTS BECAUSE THE RULE ALONE DID NOT HOLD. On 2026-08-28 the compendium
# was 612617 bytes, 7.5x this ceiling, and 92% of it was the history of corrected numbers.
# «A principle that cannot be checked is an intention» -- CLAUDE.md.
#
# ⚠️ IT IS A CEILING, NOT A TARGET. The design aims at ~65 KB; the slack is there so a
# legitimate growth does not turn the gate red on the commit that writes it. A threshold
# that fires on the first commit teaches people to ignore it.
#
# Non-vacuity: a missing file is a FAILURE, not a silent pass -- gotcha #26.
ceiling=81920
if [ ! -f "$compendium" ]; then
  report "$compendium is missing: the size ceiling would be vacuous"
else
  size=$(wc -c < "$compendium")
  if [ "$size" -gt "$ceiling" ]; then
    report "$compendium is $size bytes, over the $ceiling ceiling -- see docs/superpowers/specs/2026-08-28-sfoltimento-compendio-design.md"
  fi
fi
```

- [ ] **Passo 4: verificare i fine-riga subito dopo la scrittura**

```bash
printf 'CR=%s  righe=%s\n' "$(tr -cd '\r' < scripts/check-docs.sh | wc -c)" "$(wc -l < scripts/check-docs.sh)"
git diff --stat -- scripts/check-docs.sh
```

⛔ **`CR` deve essere uguale al numero di righe.** Se `git diff --stat` dichiara più di ~20
righe cambiate, i fine-riga sono stati normalizzati: si annulla con
`git checkout -- scripts/check-docs.sh` e si riscrive con python e `newline=''`.

- [ ] **Passo 5: direzione ① — scatta sopra soglia**

⚠️ **`$S` è lo scratchpad della sessione**, non una cartella del repository.

```bash
cp docs/COMPENDIO.md "$S/compendio.bak"
git show d902c40:docs/COMPENDIO.md > docs/COMPENDIO.md
bash scripts/check-docs.sh; echo "exit: $?"
cp "$S/compendio.bak" docs/COMPENDIO.md
```

Atteso: un `✗` che nomina il compendio e la soglia, **exit diverso da 0**.
⛔ **Il ripristino è obbligatorio e si verifica**: `git diff --stat -- docs/COMPENDIO.md` deve
essere **vuoto**.

- [ ] **Passo 6: direzione ② — non scatta sotto soglia**

```bash
bash scripts/check-docs.sh; echo "exit: $?"
```

Atteso: `OK — no inconsistencies`, `exit: 0`.
⚠️ **È la direzione che si dimentica**, e senza di essa il controllo è una superstizione.

- [ ] **Passo 7: il cancello intero**

```bash
bash scripts/gate.sh 2>&1 | tail -3
```

- [ ] **Passo 8: commit**

```bash
git add scripts/check-docs.sh && git commit -m "feat(cancello): il compendio non supera 80 KB, e il controllo e' provato nelle due direzioni" && git push origin spec/sottoprogetto-1-kernel
```

---

## Compito 6 — la chiusura: misurare, e allineare ciò che §13 pretende

- [ ] **Passo 1: la misura che vale, quella dell'obiettivo**

```bash
python -c "
import tiktoken, io
e = tiktoken.get_encoding('cl100k_base')
def t(p, n=None):
    x = io.open(p, encoding='utf-8').read()
    if n: x = ''.join(x.splitlines(keepends=True)[:n])
    return len(e.encode(x))
a, b, c = t('CLAUDE.md'), t('docs/COMPENDIO.md'), t('docs/audit-2026-08-27.md', 397)
print('CLAUDE.md          ', a)
print('COMPENDIO.md       ', b)
print('audit (testa)      ', c)
print('TOTALE             ', a+b+c, ' (era 230995)')
print('riduzione          ', round(230995/(a+b+c), 1), 'volte')
"
```

Atteso: totale **≤ 45 000**, riduzione **≥ 5×**.
⛔ **Il numero che esce è il numero che si scrive**, anche se è peggiore del bersaglio.

- [ ] **Passo 2: niente è andato perso**

```bash
printf 'archivio: %s byte\n' "$(cat docs/archivio/*.md | wc -c)"
git diff --stat d902c40..HEAD -- docs/ CLAUDE.md scripts/ | tail -3
git diff --stat d902c40..HEAD -- crates/
```

L'ultimo comando deve essere **vuoto**: nessuna riga di prodotto toccata.

- [ ] **Passo 3: i fine-riga, tutti**

```bash
git ls-files --eol docs/ CLAUDE.md scripts/ | grep -v 'w/lf' | grep -v 'check-docs.sh'
```

Atteso: **vuoto** — `check-docs.sh` è l'unico CRLF legittimo.

- [ ] **Passo 4: aggiornare ciò che la manutenzione pretende**

`CLAUDE.md` prescrive che alla chiusura di una voce si aggiornino **`COMPENDIO.md`** e
**`HANDOFF.md`**, e **`riferimenti.md`** se la voce ha portato una misura.

| File | Cosa entra |
|---|---|
| `docs/COMPENDIO.md`, §6 | una riga: la voce è chiusa, con la misura prima/dopo |
| `docs/HANDOFF.md` | ⛔ **un gotcha nuovo**, se e solo se insegna qualcosa che nessuna riga esistente insegna. Candidato: *«una sezione di puntatori che porta i pesi diventa un diario»*. ⚠️ **Un gotcha che non insegna niente diluisce quelli che insegnano**: se è coperto dal #31, **non si aggiunge** |
| `docs/riferimenti.md` | il metodo di misura dei token, col comando e la data. La voce **ha** portato una misura |
| `docs/README.md`, `docs/roadmap.md` | ⚠️ **solo se nominano il compendio o i suoi pesi.** Si verifica con `grep -n 'COMPENDIO\|compendio' docs/README.md docs/roadmap.md` |

- [ ] **Passo 5: l'ultima corsa**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh 2>&1 | tail -2 && git status --short
```

- [ ] **Passo 6: commit finale**

```bash
git add -u docs/ CLAUDE.md && git commit -m "docs(sfoltimento): la voce si chiude, con la misura prima e dopo" && git push origin spec/sottoprogetto-1-kernel
```

---

## Ciò che questo piano NON fa, e va detto

| | Perché |
|---|---|
| **Non tocca `docs/audit-2026-08-27.md`** | è il compito aperto. Vale 18 266 token della lettura obbligatoria, e se ne va **chiudendosi**, non sfoltendosi |
| **Non tocca `docs/HANDOFF.md`** oltre l'eventuale gotcha | è la casa dei gotcha. Pesa 94 744 token, e **non è lettura obbligatoria** |
| **Non spegne plugin né server MCP** | ⚠️ pesano un ordine di grandezza confrontabile con tutto ciò che questo piano toglie. È una **sottrazione** e non una misura (§6 del disegno), e si agisce dalla configurazione |
| **Non rinumera nessuna sezione** | romperebbe i delimitatori del cancello e ogni rimando incrociato |
| **Non decide se `E64` del piano del Traguardo 5 vada corretto** | il disegno lo lascia al proprietario, e resta lì |
