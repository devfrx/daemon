# Knowledge base — i documenti: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario (§5.5, punto 4, del [disegno](../specs/2026-09-04-knowledge-base-design.md)).
> I passi usano le caselle (`- [ ]`) per il tracciamento. ⛔ **Il pre-controllo di ogni compito è
> fatto nella sessione che ha scritto il piano, il 2026-09-04, contro il repository a `7c4881c`;
> l'esecuzione va in una sessione NUOVA.**

**Obiettivo.** Tradurre in documenti il disegno della knowledge base, riletto dal proprietario il
2026-09-04: i rimandi datati in testa a ADR-0008, ADR-0009, ADR-0010 e ADR-0038 e nella riga di
ADR-0039; la riga **13** «Registro delle guide, trigger e proiezione» in `roadmap.md`, con le
celle 3 e 6 e la riga di «Perché quest'ordine»; le sei righe di `tracciabilita.md`; la decisione 7
del disegno dei gesti **chiusa in tutte le sue case**; e la chiusura, col disegno negli indici e il
puntatore della §6 mosso.

**Forma.** Compiti in sequenza, e quanti siano lo dice la tabella della posizione qui sotto.
⛔ **Nessun file di `crates/`, `scripts/` o dei manifesti cambia**, ed è la condizione 7 della
Definizione di «fatto»: questo piano scrive **solo documenti**. Le decisioni sono già prese dal
disegno e dalla rilettura; questo piano le traduce in passi, e le poche che aggiunge stanno nella
tabella *«Le decisioni prese da questo piano»*.

**Strumenti.** `bash`, `awk`, `grep`, `sed -n` in lettura; **Python 3** per ogni scrittura, con
l'aiutante qui sotto — che conserva i fine-riga del file, rifiuta se il testo vecchio manca o non
è unico, e scrive su un temporaneo prima di rinominare (gotcha #82); `git`. La porta di qualità
è `bash scripts/gate.sh`, e deve stampare `GATE GREEN` **prima di ogni commit**, anche di soli
documenti: gira `check-docs.sh` come sesto passo, ed è quello che qui morde.

⛔ **L'aiutante `replace_unique.py` vive nello scratchpad, mai nel repository.** È lo stesso del
[piano dei gesti](2026-09-03-riconoscimento-gesti.md), ricopiato perché chi esegue legge **questo**
file. I testi *Trova* e *Sostituisci con* di ogni compito si mettono in due file e si passano a lui:

```python
"""replace_unique.py -- replace ONE unique occurrence of a text, keeping the file's line endings.

Usage: python replace_unique.py <file> <old.txt> <new.txt>

<old.txt> and <new.txt> hold the exact texts (UTF-8). A single trailing newline in each is
dropped, so a file written with an editor works. If <file> is CRLF, the texts are converted to
CRLF before matching and writing. Refuses when the old text is absent or not unique. Builds the
whole content first, writes a temporary file, then os.replace -- a rename cannot fail halfway
(gotcha #82).
"""
import io
import os
import sys

path, old_path, new_path = sys.argv[1:4]
raw = io.open(path, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw


def text(p):
    t = io.open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
    if t.endswith("\n"):
        t = t[:-1]
    return t.replace("\n", "\r\n") if crlf else t


old, new = text(old_path), text(new_path)
n = raw.count(old)
if n != 1:
    sys.exit(f"refused: {n} occurrences of the old text in {path}")
out = raw.replace(old, new)
tmp = path + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print(f"ok: {path} ({'CRLF' if crlf else 'LF'})")
```

Dopo ogni scrittura su un file CRLF si rimisura, e CR deve essere uguale alle righe:

```bash
for f in <i file toccati>; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol <i file toccati>
```

⛔ **Le scritture «resta; sotto di essa» si fanno con lo stesso aiutante:** Trova = la riga intera
esistente; Sostituisci con = la stessa riga, un a-capo, la riga nuova. L'aiutante non «inserisce»:
sostituisce, e la riga vecchia sta nel testo nuovo.

**Disegno:** [`2026-09-04-knowledge-base-design.md`](../specs/2026-09-04-knowledge-base-design.md)
— chi coordina lo legge **prima** dei compiti, per intero; a ciascun subagente si danno le sezioni
che il suo compito nomina, non il file. ⛔ **`riferimenti.md` non si tocca** (§7 del disegno):
nessuna fonte esterna, nessuna misura che non sia un comando sul repository.

## Vincoli globali

Valgono per ogni compito, senza che il compito li ripeta. `<base>` è il commit in cui questo piano
è nato, e lo dà un comando: `git log --format=%h --diff-filter=A -- docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md`.

| # | Vincolo | Da |
|---|---|---|
| 1 | **le due spec non si toccano**: `git diff --name-only <base>..HEAD -- docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` resta vuoto a ogni compito | §7.4 e §8 sono spec; vincolo globale 7 del repository |
| 2 | **nessun codice**: `git diff --stat <base>..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml` resta vuoto a ogni compito — è la condizione 7 della Definizione di «fatto» | disegno §1.5 e §5.5 |
| 3 | **documenti in italiano**, e un riferimento al codice dentro un documento in inglese col nome esatto del sorgente | §1.0 della spec |
| 4 | **nessuna cifra nuova in prosa** senza il comando accanto; una cifra che sostiene una decisione vive in **una** casa sola; per i numeri piccoli le parole | `CLAUDE.md` |
| 5 | **i fine-riga si conservano per file** e si rimisurano dopo ogni scrittura: la mappa dei file dice CRLF o LF; `git ls-files --eol` prima e dopo, **invariato** | disegno, trappola 3 e misura 7; `CLAUDE.md` |
| 6 | **ogni conteggio si rifà col comando** prima di leggere che cosa un documento ne dice; le cifre di questo piano sono istantanee del 2026-09-04 su `7c4881c` | disegno, *Metodo* |
| 7 | **si toglie invece di riscrivere meglio**; un verbale non si riscrive; una correzione a una sezione approvata porta un richiamo datato | `CLAUDE.md`, gotcha #76 |
| 8 | **gli ADR sono append-only**: i rimandi vanno **in testa**, sotto la riga `Deciders`, come blockquote datato — la forma viva di ADR-0001 (misura 8 del disegno) — con la frase *«Nessuna riga di questo ADR è superata»*; per ADR-0039 il rimando entra **nella cella** che il disegno nomina (D3). Il compendio **rimanda** in una frase, non ricopia | disegno §3.2, trappola 1 |
| 9 | **il compendio resta sotto il tetto**: margine misurato prima e dopo ogni tocco con il comando qui sotto; se va rosso si toglie prosa dalla §6, **non si alza il tetto** | disegno, trappola 2 |
| 10 | **nessun link `](…)` a un file che non esiste**: tutti i file che questo piano linka esistono già al `<base>`; i piani sono fuori dal controllo dei link, gli altri documenti no, e un'**ancora** (`#…`) non si scrive mai | disegno, trappola 5 |
| 11 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore**; il cancello gira **prima** | `CLAUDE.md` |
| 12 | **l'accettazione condizionata del proprietario vale finché regge**: se un passo violasse uno dei cinque criteri di `anthropic-skills:decision-principles` — richiedesse una scorciatoia, duplicasse qualcosa che esiste, poggiasse su qualcosa che non è più vero — ci si **ferma e lo si riporta** | disegno, *Le regole di questo lavoro* |
| 13 | **il numeratore dei compiti** vive nella tabella della posizione e in nessun altro punto di questo file o del repository | gotcha #68 |
| 14 | **nessun ADR nasce**, quindi i totali degli ADR nei documenti di stato **non cambiano** e `check-docs.sh` non deve mai chiedere una voce nuova in §5: un compito che scoprisse di aver bisogno di un ADR si **ferma** — sarebbe una modifica al disegno, del proprietario | disegno §3.1 |
| 15 | **il `grep` di questa macchina** (GNU grep 3.0, Git Bash): un'alternanza si scrive con il `grep` **di base** e `\|`, mai con `-E` e `\|` — che rende la barra letterale e un verde vacuo — e mai `-i` con più di un `-e`; ogni `grep -c` che deve rendere **0** si prova **prima** su un input dove deve rendere **1** | disegno, trappola 14 |

Il comando del margine, vincolo 9:

```bash
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
```

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

✅ **IL PIANO È SCRITTO IL 2026-09-04**, nell'ordine del punto 4 della §5.5 del disegno. Il
pre-controllo delle quattro domande su ciascun compito sta nella sezione *«Il pre-controllo del
piano»* qui sotto, fatto nella sessione che ha scritto il piano, come il disegno prescrive.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | i quattro rimandi datati in testa a ADR-0008, ADR-0009, ADR-0010 e ADR-0038, e le loro voci di §5 nel compendio | uno | ✅ 2026-09-05 |
| **2** | la decisione 7 dei gesti **chiusa in tutte le sue case**: la riga di ADR-0039 e la sua voce di §5, le sette case del disegno dei gesti, la riga di `tracciabilita.md` | uno | ✅ 2026-09-05 |
| **3** | `roadmap.md`: la riga 13, le celle 3 e 6, la riga di «Perché quest'ordine», l'intestazione | uno | ⬜ |
| **4** | `tracciabilita.md`: le cinque righe che restano, e il riquadro in testa — `sede da assegnare` da tre a uno | uno | ⬜ |
| **5** | la chiusura: la Definizione di «fatto» coi comandi; il disegno e questo piano nella §12 del compendio e in `README.md`; il puntatore della §6; la riga di questo piano in roadmap; la spunta nel disegno | uno | ⬜ |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del
[`COMPENDIO.md`](../../COMPENDIO.md), in un posto solo. Ciò che resta qui è la **posizione** del
piano — la tabella qui sopra, che chi esegue aggiorna nel commit del compito — e **come** si
esegue un compito.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro — e il
   disegno nelle sezioni che il compito nomina.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-04.
3. Se il compito dice il falso, **ci si ferma e si riporta**: non si aggira. Una divergenza è una
   voce d'errata prima di essere un rimedio.
4. Il cancello gira **prima** di ogni commit; il commit dice ciò che il compito ha fatto.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; per il
   compito 1 rilegge ciascun rimando **contro l'ADR che lo ospita e contro i fratelli che nomina**
   (gotcha #59); per il compito 2 rilancia il censimento delle case nelle **due** forme.
6. Una seconda ondata di **sola prosa** la fa il coordinatore a mano, senza ri-revisione; se
   tocca un fatto o un comando, si ri-rivede. Dopo due ondate di prosa si chiude: le frasi non
   reggono alla terza come alla prima (gotcha #76 — si **toglie**, non si riscrive meglio).

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota, e non resterà vuota.** Il pre-controllo ha trovato un difetto reale in **tutti**
i compiti dispacciati finora, senza una sola eccezione: quando ne trovi uno, si scrive **qui**,
con il proprio numero, prima di eseguirlo. Un piano è un'ipotesi. ⚠️ Le voci **4** e **5** del
disegno — il nome della riga 13 e la frase di «Perché quest'ordine» — sono state **scelte dal
proprietario** nella rilettura del 2026-09-04, non prese dal piano col consiglio: se dicesse altro
prima dell'esecuzione, la prima voce di questa errata lo registra.

| # | Voce |
|---|---|
| — | *(vuota alla scrittura, 2026-09-04)* |
| **E1** | **Compito 2, Passo 6 — la sonda sulle quattro righe del primo censimento è VACUA: rende 4 anche PRIMA del compito.** Scritta dal pre-controllo del 2026-09-05, prima di dispacciare, rilanciando il comando sul repository a `1a8ee6d`: `grep -n -i 'decisione 7' "$G" \| grep -c -i 'decis\|chiusa\|sciolta'` rende **4** oggi, perché «decision*e*» contiene «decis» e tutte e quattro le righe la portano — la sonda non distingue prima da dopo, ed è la domanda **1** del pre-controllo, un verde vacuo. La sonda vera è il segno di spunta, che oggi nessuna delle quattro porta: `grep -n -i 'decisione 7' "$G" \| grep -c '✅'` rende **0** prima e deve rendere **4** dopo; la direzione «deve trovare» è provata sullo stesso file, dove `grep -c '✅' "$G"` rende più di zero oggi. L'attesa «→ **4**» del Passo 6 si legge su questa sonda, e il Passo 6 del brief resta com'è: è un verbale |
| **E2** | **Compito 1, Passo 4 — la sonda `grep -c 'Nessuna riga di questo ADR è superata'` rende 0 su ADR-0010: la frase dettata va a capo fra «riga» e «di», e un grep a riga singola non la vede.** Misurata dall'implementatore il 2026-09-05 dopo il commit `8f7d078` e confermata dal coordinatore: il testo nel file è quello del brief carattere per carattere, e il precedente — la testa di ADR-0001 — spezza la stessa frase allo stesso modo («**Nessuna» / «riga di questo ADR è superata:**»), quindi la sonda non avrebbe retto nemmeno sul modello. È la domanda **1** del pre-controllo: la sonda attacca la riga, il testo va a capo. Il testo **non** è stato riscritto per farla passare (vincolo 7). La sonda che regge ai capoversi, provata su tutti e quattro e su ADR-0001: `for f in docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md; do tr -d '\r' < "$f" \| tr '\n' ' ' \| sed 's/> //g' \| grep -o 'Nessuna riga di questo ADR è superata' \| wc -l; done` → **1** ciascuno. La seconda cifra dell'attesa «`1` e `1` per ciascun ADR» del Passo 4 si legge su questa sonda |

---

## Il pre-controllo del piano — che cosa il disegno dice e il repository smentisce

Letto scrivendo, contro il repository a `7c4881c`, il 2026-09-04. Ogni voce è una delle quattro
domande di `CLAUDE.md`, o una delle righe che l'elenco non coglie.

### P-1 — Il disegno manda questo file «in `README.md`», e i disegni vivono nella tabella «Specifiche»

La §5.5 dice *«questo file nella §12 del compendio e in `README.md`»*. Misurato:
`grep -n 'riconoscimento-gesti-design' docs/README.md` rende **una** riga, nella tabella
**«Specifiche»**, con la forma *«⛔ Non è una spec»* — la trappola 11 del disegno lo dice già. Il
compito 5 scrive la riga lì, sotto quella dei gesti. Domanda **5** — il contratto si legge contro il
repository di adesso, e il precedente vince sul nome.

### P-2 — I fine-riga sono quelli che il disegno misura (misura 7), rimisurati oggi

`git ls-files --eol` sui file della mappa: `docs/COMPENDIO.md`, `docs/README.md`, `docs/roadmap.md`
e `docs/tracciabilita.md` sono `i/lf w/crlf`; i cinque ADR, il disegno dei gesti e il disegno della
knowledge base sono `i/lf w/lf`. Nessuna divergenza dalla misura 7. Chi esegue **rimisura**.

### P-3 — Per ADR-0039 il rimando entra in una CELLA, e la sonda «nessuna riga tolta» sarebbe sbagliata

La condizione 1 della Definizione di «fatto» dice *«in testa a 0009, 0008, 0010, 0038 e **nella
riga** di 0039, append-only»*. Per i quattro rimandi in testa la sonda giusta è quella del piano
dei gesti — `git diff` **senza righe `-`** in quei file. Per ADR-0039 no: la riga
`| la **destinazione** di una cattura | il brainstorming della knowledge base — decisione 7 |` del
perimetro negativo (`grep -n 'destinazione' docs/adr/0039-*.md`, unica) viene **riscritta con il
testo vecchio in testa e il rimando in coda**, quindi il diff porta **una** riga `-` e **una** `+`,
e la `+` comincia con la `-`. Domanda **1** — la sonda del precedente attaccava il caso sbagliato:
il compito 2 detta la sonda giusta, e la scelta è **D3**.

### P-4 — La decisione 7 dei gesti vive in NOVE case, e la trappola 10 del disegno ne nomina UNA

La trappola 10 dice *«si riscrive la cella della riga 7 — stato e chiusura — senza aggiungere
righe»*. Misurato con **due** censimenti, perché nessuno dei due basta (gotcha #70):
`grep -n -i 'decisione 7' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md`
rende **quattro** righe — 59, 120, 165, 387 — e **non** rende né la riga 7 della tabella delle
decisioni (che porta il numero, non la parola) né la frase della §5.4 (*«la **7** (la cattura →
brainstorming 2)»*); `grep -n -i cattura` sullo stesso file le rende entrambe, più l'elenco delle righe di
tracciabilità della §5.2 — righe 379–380, *«cattura con un gesto (Gesti + brainstorming 2)»* — che è
una casa anch'esso, e una riga che **non** lo è, S2 (*«latenza da cattura a disegno»*). Le case nel
disegno dei gesti sono quindi **sette**: la risposta 1 (riga 59, *«la
destinazione **aperta**»*), la riga 7 della tabella (93), la tabella «Rimandato → A chi» (120), il
buco logico *«La foto va in che contesto?»* (165), l'elenco della §5.2 (379–380), la §5.3 (387) e la §5.4 (395). Fuori dal
disegno ne stanno **due**: la riga del perimetro negativo di ADR-0039 e la riga *«Cattura con un
gesto»* di `tracciabilita.md`. ⛔ **Il compito 2 le chiude tutte e nove in un commit**, sul
precedente di AUD-013 e AUD-024: chiuderne una e lasciare le altre a dire *«aperta»* è la radice
R1 dentro il rimedio. Domanda **2** — il controllo che mancava è il doppio censimento, e il compito
lo detta nelle due forme.

### P-5 — Le cinque ancore di §5 del compendio sono uniche, e la forma del rimando in §5 esiste già

`grep -c` sulle ultime frasi delle voci 0008, 0009, 0010, 0038 e 0039 di §5 rende **1** per
ciascuna (i Trova del compito 1 e del compito 2). La forma *«⚠️ **Rimando del ‹data›, in testa
all'ADR:** …»* vive nelle voci 0001, 0011 e 0023 — `grep -c "in testa all.ADR" docs/COMPENDIO.md`
rende **3** oggi — e i quattro rimandi del compito 1 la copiano: dopo il compito 1 rende **7**. La
voce di 0039 dice *«nella riga del perimetro negativo»* e **non** «in testa» (D6), quindi non
muove quel conteggio: è la seconda direzione della sonda.

### P-6 — Niente di ciò che il piano detta esiste già (domanda 4), misurato

`grep -c 'Rimando del'` sui cinque ADR rende **0** su tutti; `grep -c '^| 13 |' docs/roadmap.md`
rende **0**; `grep -cE 'sede da assegnare' docs/tracciabilita.md` rende **3**; la riga 7 del
disegno dei gesti dice ancora *«⏳ aperta, dipendenza dichiarata»*;
`grep -c 'knowledge-base-design'` rende **0** in `README.md`, `roadmap.md` e `tracciabilita.md`, e
**1** nel compendio — il puntatore della §6. Le teste dei quattro ADR portano
`- **Deciders:** proprietario del progetto`, una riga vuota e `## Context` alle righe **5–7**, e
`## Context` è unico in ciascuno.

### P-7 — La coda della tabella dei sotto-progetti si legge delimitando per SEZIONE, non per file

Il piano dei gesti lo ha imparato due volte (le sue voci E7 ed E15): un `grep` sulle righe
`| N |` pesca anche altre tabelle di `roadmap.md`. La sonda è
`awk '/^## Sotto-progetti/{s=1;next} s&&/^## /{s=0} s&&/^[|] /' docs/roadmap.md | tail -3`, che
rende oggi `10`, `11`, `12` e dopo il compito 3 `11`, `12`, `13`. Domanda **1**.

### P-8 — La riga «Ultimo aggiornamento» di `roadmap.md` si riallinea in OGNI commit che tocca il file

È la voce E14 del piano dei gesti, avverata al primo tocco successivo, e la E17 ④ che ne ha fatto
una regola. Qui `roadmap.md` è toccato da questa sessione (la riga di questo piano), dal compito 3
e dal compito 5: tutti e tre riscrivono quella riga. La sua forma oggi:
`grep -n '^Ultimo aggiornamento' docs/roadmap.md`, **una** riga.

### P-9 — Il margine del compendio è misurato, e i compiti lo consumano due volte

Il comando del vincolo 9 rende **12268** il 2026-09-04 su `7c4881c`. Il compito 1 aggiunge quattro
frasi alle voci di §5, il compito 2 una, il compito 5 due righe in §12 e riscrive il puntatore
della §6 e l'intestazione: in tutto meno di tremila byte, se le frasi restano quelle dettate. Ogni
compito misura prima e dopo (vincolo 9); se una frase cresce, si accorcia **lei**.

### P-10 — Le voci aperte del repository, rilette per il chiusore: UNA ha questo piano

Come la riga di `CLAUDE.md` su `superpowers:writing-plans` prescrive, lette **prima** di scrivere:
le tabelle *«Le voci aperte del Traguardo 5»* e *«del Traguardo 6»* di
[`porta-di-qualita.md`](../../porta-di-qualita.md) — trentaquattro e ventinove righe, ciascuna con
la colonna *«Chi la chiude»* rilanciata con l'`awk` che quelle sezioni portano — le voci senza
numero AUD dell'[audit](../../audit-2026-08-27.md), la tabella 3.4 del disegno della knowledge
base, e la tabella delle tredici decisioni del disegno dei gesti. **Una sola ha come chiusore ciò
che questo piano scrive:** la decisione **7** dei gesti, il cui chiusore è *«il brainstorming 2, la
knowledge base»* — fatto, e il compito 2 la chiude. **Nessuna ha «il proprietario, prima» di
questo piano**: la decisione 11 della knowledge base (AUD-004) è *«prima del 13»*, e il disegno lo
dice tre volte. L'elenco di ciò che si sa e si dichiara sta nella sezione *«Le voci aperte che
questo piano SA, e non chiude»*.

### P-11 — Le sei righe di `tracciabilita.md` sono sei, alle righe misurate, e nessuna cambia stato

`grep -n` su `Skills`, `Regole e vincoli`, `Collezioni e knowledge base`, `Memoria persistente`,
`File watching`, `Cattura con un gesto` rende **sei** righe di funzionalità (112, 133, 134, 153,
164, 209 il 2026-09-04) più una di commento nel riquadro; la terza riga con `sede da assegnare` è
*«Storage e cifratura a riposo»* e **resta**. Nessuna delle sei cambia il proprio simbolo di stato:
il conteggio per stato del riquadro — `46 · 52 · 79 · 0 · 1` oggi — deve restare **identico** dopo
i compiti 2 e 4, ed è la seconda direzione della sonda.

### P-12 — «Conoscenza / RAG» vive in altre case, e nessuna diventa falsa

`grep -rn 'Conoscenza / RAG' docs/ CLAUDE.md`: la riga 6 di `roadmap.md` (che il compito 3
riscrive); la spec del kernel (**spec**, vincolo 1); il disegno e l'archivio della consegna, che la
**citano** come nome vecchio. E `README.md` porta *«Conoscenza/RAG»* nella tabella della
decomposizione (riga 212): è l'etichetta di una **categoria** L2, non il nome della riga di
roadmap, e non dice il falso — **non si tocca** (D5).

### P-13 — Il comando del disegno dei gesti che conta le decisioni deve rendere ancora 13

`awk '/^\*\*Le decisioni, numerate/{s=1} s&&/^## 1\./{s=0} s&&/^\| [0-9]+ \|/{n++} END{print n}' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md`
rende **13** oggi. Il compito 2 riscrive **celle**, non aggiunge righe: dopo, ancora 13 — trappola
10 del disegno, provata nelle due direzioni dal compito.

### P-14 — La frase di «Perché quest'ordine» scelta dal proprietario nomina «questo disegno», e in roadmap diventa un link

La voce 5 del disegno, scelta il 2026-09-04, dice *«(§1.1d di questo disegno)»*: scritta dentro
`roadmap.md` quel «questo» non ha referente. Il compito 3 la scrive con il link al disegno al posto
di «questo disegno», e **nient'altro cambia** della frase (D4).

### P-15 — La guardia dei conteggi legge sei documenti di stato, e nessun testo dettato vi scrive «‹cifra› ADR»

`sed -n '288p' scripts/check-docs.sh` nomina `HANDOFF.md`, `roadmap.md`, `README.md`,
`COMPENDIO.md`, `AVVIO-CHAT.md` e `CLAUDE.md`. Riletti i testi dettati per roadmap, README e
compendio: le forme sono `ADR-0009` e *«quattro rimandi»*, mai una cifra in numero seguita da
«ADR» — trappola 9 del disegno.

### P-16 — `HANDOFF.md` non si tocca, e il perché è misurato

`grep -c 'riconoscimento-gesti' docs/HANDOFF.md` rende **0**: la mappa dei documenti di quel file
non elenca i disegni, quindi nemmeno questo vi entra (D12). E nessun totale di ADR cambia
(vincolo 14).

---

## Le decisioni prese da questo piano

⛔ **Sono decisioni del piano, non del disegno, e chi esegue può ribaltarle** portando la misura
che le smentisce — è ciò per cui esiste l'errata.

| | Decisione | Perché |
|---|---|---|
| **D1** | **cinque compiti**: i quattro rimandi in testa (1); la decisione 7 chiusa in tutte le sue case (2); `roadmap.md` (3); `tracciabilita.md` (4); la chiusura (5). La riga *«Cattura con un gesto»* di `tracciabilita.md` sta nel **2** e non nel 4 | un revisore respinge l'uno approvando l'altro; e un rimedio si chiude su **tutte** le case della frase in un commit solo — AUD-013, AUD-024, AUD-055/057 |
| **D2** | in `tracciabilita.md` la sede del registro delle guide e dei trigger è scritta **col numero**, `**13**`, com'è nella §5.2 approvata del disegno, e la riga del riquadro in testa porta il **nome** una volta | il nome vive in `roadmap.md`, una casa sola; il numero è ciò che il proprietario ha approvato |
| **D3** | il rimando di ADR-0039 entra **nella cella** del perimetro negativo, col testo vecchio in testa; la sonda è *«una riga `-`, una `+`, e la `+` comincia con la `-`»* | P-3; la condizione 1 della Definizione di «fatto» dice *«nella riga»* |
| **D4** | la frase di «Perché quest'ordine» scelta dal proprietario si scrive con il **link** al disegno al posto di *«questo disegno»*, e nient'altro cambia | P-14 |
| **D5** | *«Conoscenza/RAG»* nella tabella della decomposizione di `README.md` **non si tocca** | P-12: è un'etichetta di categoria e non dice il falso — la forma di D3 del piano dei gesti |
| **D6** | nel compendio la voce di §5 di ADR-0039 dice *«nella riga del perimetro negativo»*, non «in testa all'ADR» | P-5: il conteggio *«in testa all'ADR»* resta una sonda vera |
| **D7** | ogni compito **rimisura** la baseline — cancello, `check-docs.sh`, margine — e non cita quella scritta qui | gotcha #31 |
| **D8** | la riga di questo piano nella tabella dei piani di `roadmap.md` è scritta dalla sessione che scrive il piano — *«scritto il 2026-09-04»* — e il compito 5 la porta a *«eseguito»* | precedente D9 del piano dei gesti |
| **D9** | il compito 5 aggiunge al compendio **due** righe di §12 — il disegno e questo piano — sotto la riga del piano dei gesti, e riscrive il puntatore della §6 al **sotto-progetto 2 con AUD-004 in parallelo** (§5.5, punti 5 e 6; decisione 16) | precedente D10 del piano dei gesti |
| **D10** | **nessun diagramma di `design/` cambia**: il sistema non cambia con questo piano | regola 2 di `README.md` |
| **D11** | `riferimenti.md` **non si tocca** | §7 del disegno, trappola 13 |
| **D12** | `HANDOFF.md` **non si tocca** | P-16 |
| **D13** | la spunta del punto 4 della §5.5 del disegno è scritta da questa sessione — *«scritto, pre-controllo fatto; l'esecuzione va in una sessione nuova»* — e il compito 5 la porta a *«eseguito»*; sotto *«Il prossimo passo»* del disegno il compito 5 aggiunge un richiamo datato, come fece il piano dei gesti | disegno §5.5; precedente del compito 9 dei gesti |
| **D14** | nel disegno dei gesti la colonna *«Stato al 2026-09-03»* **non si rinomina**: la cella della riga 7 porta la propria data | un'intestazione datata è un verbale; la data vera sta nella cella |
| **D15** | la cella *«Dipende da»* della riga 13 porta *«1 · AUD-004 deciso»* con il rimando all'audit, e la tabella *«Decisioni ancora da prendere»* di `roadmap.md` **non** guadagna una riga | la casa della decisione è la tabella *«Le decisioni prese rimediando»* dell'audit e la 3.4 del disegno: una terza sarebbe il gotcha #68 |
| **D16** | i **cinque rimandi** nelle voci di §5 del compendio nominano il disegno **senza link** — *«disegno della knowledge base»* — perché il link vive nella §6 e, dal compito 5, nella §12 | il tetto del compendio; e la forma delle voci 0001, 0011 e 0023, che nominano ADR-0039 senza link |

**La baseline di partenza, misurata il 2026-09-04 su `7c4881c` e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `bash scripts/check-docs.sh` → `OK — no inconsistencies.` ·
il comando del vincolo 9 → `12268` · `git status -sb` → `## main...origin/main`, pulito.

---

## La mappa dei file

| File | Chi lo tocca | Fine-riga il 2026-09-04 | Responsabilità |
|---|---|---|---|
| `docs/adr/0008-contesto-come-proiezione-dello-stato.md` | compito 1 | LF | il rimando in testa |
| `docs/adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md` | compito 1 | LF | il rimando in testa |
| `docs/adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md` | compito 1 | LF | il rimando in testa |
| `docs/adr/0038-registro-delle-funzioni-del-programma.md` | compito 1 | LF | il rimando in testa |
| `docs/adr/0039-telecamera-come-sorgente-di-percezione.md` | compito 2 | LF | il rimando **nella cella** del perimetro negativo |
| `docs/COMPENDIO.md` | compiti 1, 2, 5 | **CRLF** | §5: cinque rimandi; §6, §12 e l'intestazione al compito 5 |
| `docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` | compito 2 | LF | le sette case della decisione 7 |
| `docs/tracciabilita.md` | compiti 2, 4 | **CRLF** | la riga della cattura (2); le altre cinque e il riquadro (4) |
| `docs/roadmap.md` | compiti 3, 5 | **CRLF** | la riga 13, le celle 3 e 6, «Perché quest'ordine», l'intestazione; la riga di questo piano |
| `docs/README.md` | compito 5 | **CRLF** | la riga del disegno nella tabella «Specifiche» |
| `docs/superpowers/specs/2026-09-04-knowledge-base-design.md` | compito 5 | LF | la spunta della §5.5 e il richiamo sotto «Il prossimo passo» |
| `docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md` | ogni compito | LF | la tabella della posizione, e l'errata |

⛔ **Nessun altro file.** In particolare non `crates/`, non `scripts/`, non `riferimenti.md`, non
`HANDOFF.md`, non `AVVIO-CHAT.md`, non `CLAUDE.md`, non le due spec.

---

## Le voci aperte che questo piano SA, e non chiude

⛔ **Lette prima di scrivere, come `CLAUDE.md` prescrive.** Si dichiarano perché chi esegue le
sappia, non perché le tocchi.

| Voce | Dove vive | Chi la chiude |
|---|---|---|
| decisione **11** della knowledge base — se le difese di ADR-0015 si estendano alle skill (AUD-004) | la tabella 3.4 del disegno; *«Le decisioni prese rimediando»* dell'audit | il **proprietario**, con un ADR suo, **prima del 13** e in parallelo al 2 — voce 3 della rilettura. ⛔ Sbarra il 13, non questo piano |
| decisioni **13, 14, 15, 18** della knowledge base — «privato ma non segreto», l'esportazione HTML, la forma dell'ambito, ricerca ibrida o vettori | la tabella 3.4 del disegno | il proprietario nel 6; nessuno finché non serve; il 13; il 6 — voce 6 della rilettura |
| decisioni **2, 9, 10, 12, 13** del disegno dei gesti | la tabella delle tredici decisioni di quel disegno | la capacità (12), un tracciatore su GPU, l'archivio dei parametri, il sotto-progetto 12 col proprietario |
| **X-1**…**X-4** dell'audit — la CI solo Linux, la riga falsa di `gate.sh`, `cargo audit`, `E64` | la tabella delle voci senza numero AUD | il proprietario |
| le tabelle *«Le voci aperte del Traguardo 5»* e *«del Traguardo 6»* del registro | [`porta-di-qualita.md`](../../porta-di-qualita.md) | quasi tutte il proprietario; le altre nominano un compito futuro che **non** è in questo piano |
| la tabella delle voci aperte della §6 del compendio | §6 del compendio | i traguardi e il proprietario che quella tabella nomina; nessuna tocca la knowledge base |
| le quattro assunzioni della §6.5 del disegno — il 3 prima capacità che inietta una guida; la mappa nel budget del modello più piccolo; «al primo salto»; il costo della misura per categoria | §6.5 del disegno | il 13 e il 6, costruendo e misurando |

---

## Compito 1: i quattro rimandi datati in testa a ADR-0008, ADR-0009, ADR-0010 e ADR-0038, e le voci di §5

**Files:**
- Modify: `docs/adr/0008-contesto-come-proiezione-dello-stato.md` (LF) · `docs/adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md` (LF) · `docs/adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md` (LF) · `docs/adr/0038-registro-delle-funzioni-del-programma.md` (LF) · `docs/COMPENDIO.md` (**CRLF**) — quattro voci di §5, l'intestazione · questo piano (LF) — la posizione
- Read: §1.1, §2, §3.1, §3.2 e §3.3 del disegno; la testa di [ADR-0001](../../adr/0001-architettura-a-kernel-con-capacita-paritarie.md), che è la forma (misura 8 del disegno)

**Interfaces:**
- Produces: la frase *«Nessuna riga di questo ADR è superata»* in ciascun rimando, che la revisione legge contro l'ADR intero; e il sotto-progetto **13** nominato per numero, che il compito 3 crea in roadmap

- [ ] **Passo 1: le misure prima, e le righe da ritrovare col `grep`**

```bash
bash scripts/check-docs.sh
git ls-files --eol docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md docs/COMPENDIO.md
printf 'compendio CR='; tr -cd '\r' < docs/COMPENDIO.md | wc -c; printf '   righe='; wc -l < docs/COMPENDIO.md
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
grep -n 'Deciders\|^## Context' docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md
grep -c 'Rimando del' docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md
grep -c "in testa all.ADR" docs/COMPENDIO.md
grep -c 'nel tritacarne del riassunto\.' docs/COMPENDIO.md
grep -c 'skill dichiarative di 0003 sono \*\*guide\*\*\.' docs/COMPENDIO.md
grep -c "contesto è troppo pieno» è un'impressione e non un dato\." docs/COMPENDIO.md
grep -c 'quali funzioni siano gestuali lo decide il 12\.' docs/COMPENDIO.md
```

Atteso: i quattro ADR `w/lf`, il compendio `w/crlf` con CR = righe; margine positivo; in ogni ADR
la riga `- **Deciders:** proprietario del progetto` a riga **5** e `## Context` a riga **7**, con
una riga vuota in mezzo; `0` rimandi in ciascun ADR; **3** «in testa all'ADR»; **1** per ciascuna
delle quattro ancore di §5.

- [ ] **Passo 2: i quattro rimandi, in testa, sotto `Deciders`**

Per ciascun ADR: con `replace_unique.py`, Trova = le tre righe
`- **Deciders:** proprietario del progetto`, riga vuota, `## Context`; Sostituisci con = le stesse
tre righe col blockquote in mezzo, cioè `Deciders`, riga vuota, il blockquote, riga vuota,
`## Context`. Il `<data>` è il giorno dell'esecuzione.

ADR-0009:

```markdown
> ⚠️ **Rimando del <data> — il registro delle guide e i trigger si costruiscono in un sotto-progetto
> di kernel PRIMA della prima capacità che inietta una guida, e il registro porta due pretese.** Il
> [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md) (§1.1d–e
> e §2, sotto accettazione condizionata, riletto dal proprietario il 2026-09-04) ha misurato che al
> 2026-09-04 nessuno dei meccanismi qui decisi — registro delle guide, trigger — né la proiezione di
> [ADR-0008](0008-contesto-come-proiezione-dello-stato.md) ha una riga di codice (§6.1 del disegno,
> col comando), e ha fissato l'**ordine**: nascono nel sotto-progetto **13** della roadmap, prima del
> 3, o la prima capacità si inietta le skill a modo suo e nascono due strade. Le due pretese: la
> **chiave di contesto** con cui il registro inietta esprime *ambito, run, modello*; ciò che il
> registro conserva porta **provenienza e impronta** all'approvazione, e «approvate ora» è una
> **proiezione del giornale**, non un secondo archivio — la forma che `crates/kernel/src/permission.rs`
> già usa per i permessi. Il sensore d'integrità dei router e l'anello che propone una voce quando
> un puntatore marcisce sono **usi** del contratto del sensore e dell'anello di miglioramento, non
> cambiamenti. **Nessuna riga di questo ADR è superata.** ⚠️ Se le difese di
> [ADR-0015](0015-descrizioni-degli-strumenti-fissate-all-approvazione.md) si estendano alle skill —
> la decisione registrata dal rimedio di AUD-004 — resta del proprietario, con un ADR suo, prima
> del 13.
```

ADR-0008:

```markdown
> ⚠️ **Rimando del <data> — la proiezione si costruisce nello stesso sotto-progetto di kernel del
> registro delle guide, prima della prima capacità che la usa; e la mappa della knowledge base è una
> categoria della proiezione.** Il
> [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md) (§1.1d e
> §2.2, sotto accettazione condizionata, riletto dal proprietario il 2026-09-04) ha misurato che al
> 2026-09-04 la proiezione qui decisa non ha una riga di codice (§6.1 del disegno, col comando), e
> la colloca nel sotto-progetto **13** della roadmap con il registro delle guide e i trigger di
> [ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md). Per la knowledge base il
> router centrale e quello dell'ambito entrano nella proiezione a ogni passo **per chiave** —
> ambito, run, modello — mai per lettura del loro testo
> ([ADR-0020](0020-nessun-modello-nel-percorso-decisionale-del-kernel.md)); le foglie entrano come
> **riferimenti** e si rileggono su richiesta, che è già la regola di questo ADR per gli artefatti.
> **Nessuna riga di questo ADR è superata.**
```

ADR-0010:

```markdown
> ⚠️ **Rimando del <data> — la proiezione nasce nel sotto-progetto 13, e la mappa della knowledge
> base è una categoria del budget.** Il
> [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md) (§1.1d,
> §2.2 e §6.5, sotto accettazione condizionata, riletto dal proprietario il 2026-09-04) colloca la
> proiezione — decisa qui e in [ADR-0008](0008-contesto-come-proiezione-dello-stato.md), senza
> codice al 2026-09-04 — nel sotto-progetto **13** della roadmap, insieme al registro delle guide e
> ai trigger di [ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md), prima della
> prima capacità che la usa. Per la knowledge base la **mappa** — il router centrale e quello
> dell'ambito — è una **categoria** del budget, misurata per categoria nel giornale come tutte le
> altre; il budget resta **per modello**, e ciò che cambia col modello è la guida e il budget, non la
> mappa, che è una. Che la mappa stia nel budget del modello più piccolo che il proprietario userà è
> un'**assunzione dichiarata** (§6.5 del disegno), e la misura il sotto-progetto 6. **Nessuna riga
> di questo ADR è superata.**
```

ADR-0038:

```markdown
> ⚠️ **Rimando del <data> — la knowledge base registra le CRUD dei propri file e gruppi come
> funzioni del registro, e «aggiungi al contesto» ha due invocatori.** Il
> [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md) (§1.1g e
> §2.3, sotto accettazione condizionata, riletto dal proprietario il 2026-09-04) è il caso che
> questo ADR prevedeva con *«molti invocatori»*: la capacità del sotto-progetto 6 registra creare,
> leggere, aggiungere al contesto, aggiornare, spostare — dentro lo spazio, dentro da fuori, fuori —
> e cancellare, ciascuna un effetto con classe dichiarata, giornalato e checkpointato dentro
> l'ambito di [ADR-0024](0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md); il pannello della
> mappa e il modello sono **invocatori** delle stesse funzioni, con la stessa tripla di permesso, e
> nessuno dei due tocca un file di lato. Disegnare il grafo, filtrare e cercare sui nomi restano
> **presentazione** e non passano dal registro, come questo ADR già dice della manipolazione della
> GUI. Il registro lo costruisce ancora il primo invocatore, il click del sotto-progetto 2; la
> knowledge base vi si registra col 6. **Nessuna riga di questo ADR è superata.**
```

- [ ] **Passo 3: il compendio, CRLF — i quattro rimandi in §5, e l'intestazione**

Le quattro voci **rimandano**, non ricopiano (vincolo 8, D16). Ogni Trova è l'ultima frase della
voce, unica nel file (Passo 1); il testo nuovo si **appende** sulla stessa riga.

| Trova | Sostituisci con |
|---|---|
| `nel tritacarne del riassunto.` | `nel tritacarne del riassunto. ⚠️ **Rimando del <data>, in testa all'ADR:** la proiezione si costruisce nel sotto-progetto **13**, prima della prima capacità che la usa; la mappa della knowledge base entra **per chiave**, le foglie come riferimenti — disegno della knowledge base.` |
| `skill dichiarative di 0003 sono **guide**.` | `skill dichiarative di 0003 sono **guide**. ⚠️ **Rimando del <data>, in testa all'ADR:** registro delle guide e trigger si costruiscono nel sotto-progetto **13**, prima della prima capacità che inietta una guida; le due pretese della mappa — chiave di contesto (ambito, run, modello); provenienza e impronta, con «approvate ora» come proiezione del giornale — disegno della knowledge base.` |
| `contesto è troppo pieno» è un'impressione e non un dato.` | `contesto è troppo pieno» è un'impressione e non un dato. ⚠️ **Rimando del <data>, in testa all'ADR:** la proiezione nasce nel sotto-progetto **13**; la mappa della knowledge base è una **categoria** del budget, per modello — disegno della knowledge base.` |
| `quali funzioni siano gestuali lo decide il 12.` | `quali funzioni siano gestuali lo decide il 12. ⚠️ **Rimando del <data>, in testa all'ADR:** la knowledge base registra le **CRUD** dei propri file e gruppi come funzioni del registro, spostamenti compresi; «aggiungi al contesto» ha **due invocatori**, il click e il modello — disegno della knowledge base.` |
| l'intestazione, da `**Aggiornato il` a `Manutenzione: §13.` sulla stessa riga | `**Aggiornato il <data>**, coi quattro rimandi datati della knowledge base — ADR-0008, ADR-0009, ADR-0010, ADR-0038 — nelle voci di §5; l'ultimo contenuto di **merito** è quello. Manutenzione: §13.` |

⚠️ Se un Trova non fosse unico, l'aiutante rifiuta: si allarga il Trova alla riga intera, non si
sceglie «la prima».

- [ ] **Passo 4: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; echo; done
printf 'compendio CR='; tr -cd '\r' < docs/COMPENDIO.md | wc -c; printf '   righe='; wc -l < docs/COMPENDIO.md
git ls-files --eol docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md docs/COMPENDIO.md
grep -c 'Rimando del' docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md
grep -c 'Nessuna riga di questo ADR è superata' docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md
grep -c "in testa all.ADR" docs/COMPENDIO.md
git diff -- docs/adr/ | grep -c '^-[^-]'
git diff --stat
```

Atteso: `OK`, `GATE GREEN`, margine positivo, `0` CR sui quattro LF, CR = righe sul compendio,
`ls-files --eol` invariato, `1` e `1` per ciascun ADR, **7** «in testa all'ADR», **0** righe tolte
negli ADR — append-only — e un diff che nomina i quattro ADR, il compendio e questo piano.

```bash
git add docs/adr/0008-contesto-come-proiezione-dello-stato.md docs/adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md docs/adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md docs/adr/0038-registro-delle-funzioni-del-programma.md docs/COMPENDIO.md docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md
git commit -m "knowledge-base(compito 1): i quattro rimandi datati in testa ad ADR-0008, ADR-0009, ADR-0010 e ADR-0038 — il sotto-progetto 13 prima della prima capacità che inietta una guida, le due pretese del registro, la mappa come categoria della proiezione, le CRUD nel registro delle funzioni — e le voci di §5 che vi rimandano"
git push
```

#### Criterio di chiusura del compito 1

- [ ] i quattro ADR portano il blockquote datato in testa, sotto `Deciders`, con la frase sulla non-superazione, e **nessuna riga preesistente** è cambiata
- [ ] le quattro voci di §5 rimandano in una frase, senza link; «in testa all'ADR» conta sette
- [ ] la revisione ha riletto ogni rimando contro l'ADR intero **e** contro ADR-0001, ADR-0014, ADR-0020, ADR-0022 e ADR-0024, che i rimandi nominano o presuppongono (gotcha #59), e non ha trovato una riga superata
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 2: la decisione 7 dei gesti chiusa in tutte le sue case — ADR-0039, il disegno dei gesti, `tracciabilita.md`, la voce di §5

**Files:**
- Modify: `docs/adr/0039-telecamera-come-sorgente-di-percezione.md` (LF) — la cella del perimetro negativo · `docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` (LF) — sette case · `docs/tracciabilita.md` (**CRLF**) — la riga *«Cattura con un gesto»* · `docs/COMPENDIO.md` (**CRLF**) — la voce di §5 di ADR-0039, l'intestazione · questo piano (LF) — la posizione
- Read: la risposta 7 e la §2.3 (regola 4) del disegno della knowledge base; P-3 e P-4 di questo piano

**Interfaces:**
- Consumes: nulla del compito 1
- Produces: `sede da assegnare` **non** cambia qui (resta a tre); il conteggio per stato di `tracciabilita.md` resta identico

- [ ] **Passo 1: le misure prima — il doppio censimento delle case**

```bash
bash scripts/check-docs.sh
G=docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md
git ls-files --eol docs/adr/0039-*.md "$G" docs/tracciabilita.md docs/COMPENDIO.md
for f in docs/tracciabilita.md docs/COMPENDIO.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
grep -n -i 'decisione 7' "$G"
grep -n -i 'cattura' "$G"
awk '/^\*\*Le decisioni, numerate/{s=1} s&&/^## 1\./{s=0} s&&/^\| [0-9]+ \|/{n++} END{print n}' "$G"
grep -n 'destinazione' docs/adr/0039-*.md
grep -c 'Rimando del' docs/adr/0039-*.md
grep -n 'Cattura con un gesto' docs/tracciabilita.md
grep -cE 'sede da assegnare' docs/tracciabilita.md
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
grep -c 'Le fonti F1–F9 in \[`riferimenti.md`\](riferimenti.md)\.' docs/COMPENDIO.md
```

Atteso: il primo censimento rende **quattro** righe (59, 120, 165, 387 il 2026-09-04), il secondo
le tre che il primo non vede — la riga 7 della tabella, l'elenco della §5.2 e la frase della §5.4 —
più una che **non** è una casa, S2; **13** decisioni; **una** riga `destinazione`
nell'ADR e `0` rimandi; la riga della cattura in tracciabilità con *«la destinazione la decide il
brainstorming»*; `sede da assegnare` a **3**; il conteggio per stato annotato; **1** per l'ancora
di §5. ⚠️ **Ogni riga che i due censimenti rendono si legge intera** (gotcha #70): se una riga
oltre le sette elencate qui sotto affermasse la decisione **aperta**, riceve lo stesso trattamento e
una voce d'errata.

- [ ] **Passo 2: ADR-0039 — la cella del perimetro negativo (D3)**

| Trova | Sostituisci con |
|---|---|
| `\| la **destinazione** di una cattura \| il brainstorming della knowledge base — decisione 7 \|` | `\| la **destinazione** di una cattura \| il brainstorming della knowledge base — decisione 7. ✅ **Rimando del <data>: decisa** — la cattura atterra nello spazio della knowledge base come **artefatto**, un file in un gruppo con il router che segue, e la run la vede come **riferimento**; [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md), risposta 7 e regola 4 della §2.3. Nessuna riga di questo ADR è superata \|` |

- [ ] **Passo 3: il disegno dei gesti, LF — le sette case**

Tutte con `replace_unique.py`; i Trova sono unici (P-4, misurato). Le celle **non** si
accorciano: il testo vecchio resta e il richiamo si appende, perché le tabelle di quel disegno sono
verbali del 2026-09-03 (vincolo 7).

| # | Trova | Sostituisci con |
|---|---|---|
| 1 | `un posto nella knowledge base?»* — decisione 7 \|` | `un posto nella knowledge base?»* — decisione 7, ✅ **decisa il 2026-09-04** (riga 7 della tabella) \|` |
| 2 | `\| 7 \| dove finisce la **cattura** con un gesto \| ⏳ aperta, dipendenza dichiarata \| il **brainstorming 2**, la knowledge base: run corrente · knowledge base · entrambe \| — \|` | `\| 7 \| dove finisce la **cattura** con un gesto \| ✅ **presa il 2026-09-04** dal brainstorming della knowledge base, sotto accettazione condizionata. ⚠️ Questa cella diceva *«⏳ aperta, dipendenza dichiarata»* \| **nella knowledge base**: la cattura atterra come file in un gruppo dello spazio, il router segue, la run riceve il **riferimento** — [disegno della knowledge base](2026-09-04-knowledge-base-design.md), risposta 7 e regola 4 della §2.3 \| *solo nella run, poi decide l'assistente*: due posti per un file, e una foto dimenticata non è nella mappa; *entrambe*: idem \|` |
| 3 | `\| dove finisce la foto catturata \| il brainstorming 2, la knowledge base — decisione 7 \|` | `\| dove finisce la foto catturata \| il brainstorming 2, la knowledge base — decisione 7. ✅ **Decisa il 2026-09-04**: nella knowledge base, la run la vede — riga 7 della tabella delle decisioni \|` |
| 4 | `\| se vada **anche** nella knowledge base: brainstorming 2, decisione 7 \|` | `\| se vada **anche** nella knowledge base: brainstorming 2, decisione 7 — ✅ **sì, decisa il 2026-09-04**: la cartella della knowledge base **è** l'archivio, la foto atterra in un gruppo, la run riceve il riferimento (riga 7 della tabella delle decisioni) \|` |
| 5 | `\| dove finisce la **cattura** (decisione 7) \| il brainstorming 2, la knowledge base \|` | `\| dove finisce la **cattura** (decisione 7) \| il brainstorming 2, la knowledge base — ✅ **sciolta il 2026-09-04**: nella knowledge base, la run la vede (riga 7 della tabella delle decisioni) \|` |
| 6 | `la **7** (la cattura → brainstorming 2), la **9**` | `la **7** (la cattura → brainstorming 2 — ✅ **chiusa il 2026-09-04**, riga 7 della tabella), la **9**` |
| 7 | le **due righe** `cattura con un gesto (Gesti +` e `brainstorming 2), indicatore di telecamera accesa (GUI). Il registro delle funzioni va accanto a`, prese dal file **con il loro a-capo** — l'elenco della §5.2 | le stesse due righe, dove la seconda diventa `brainstorming 2 — ✅ **decisa il 2026-09-04**, riga 7 della tabella), indicatore di telecamera accesa (GUI). Il registro delle funzioni va accanto a` |

⚠️ La riga 2 è una riga di tabella con **cinque** celle: contare le barre prima di scrivere il
file *Sostituisci con*. La colonna *«Stato al 2026-09-03»* non si rinomina (D14).

- [ ] **Passo 4: `tracciabilita.md`, CRLF — la riga della cattura**

| Trova | Sostituisci con |
|---|---|
| `\| Cattura con un gesto \| 📋 \| Gesti — la destinazione la decide il brainstorming della knowledge base (decisione 7 del disegno) \|` | `\| Cattura con un gesto \| 📋 \| Gesti — la destinazione è **decisa il 2026-09-04**: nello spazio della knowledge base come file, la run la vede come riferimento (decisione 7 del disegno dei gesti, chiusa dal [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md)) \|` |

- [ ] **Passo 5: il compendio, CRLF — la voce di §5 di ADR-0039 (D6), e l'intestazione**

| Trova | Sostituisci con |
|---|---|
| `Le fonti F1–F9 in [\`riferimenti.md\`](riferimenti.md).` | `Le fonti F1–F9 in [\`riferimenti.md\`](riferimenti.md). ✅ **Rimando del <data>, nella riga del perimetro negativo:** la destinazione di una cattura è **decisa** — nella knowledge base come artefatto, la run la vede come riferimento (decisione 7 dei gesti, chiusa dal disegno della knowledge base).` |
| l'intestazione, da `**Aggiornato il` a `Manutenzione: §13.` sulla stessa riga | `**Aggiornato il <data>**, col rimando della cattura nella voce di ADR-0039 — la decisione 7 dei gesti chiusa dalla knowledge base; l'ultimo contenuto di **merito** è quello. Manutenzione: §13.` |

- [ ] **Passo 6: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
G=docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md
for f in docs/adr/0039-*.md "$G"; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; echo; done
for f in docs/tracciabilita.md docs/COMPENDIO.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol docs/adr/0039-*.md "$G" docs/tracciabilita.md docs/COMPENDIO.md
git diff -- docs/adr/0039-*.md | grep -c '^-[^-]'; git diff -- docs/adr/0039-*.md | grep -c '^+[^+]'
grep -c 'Rimando del' docs/adr/0039-*.md
grep -n -i 'decisione 7' "$G" | grep -c -i 'decis\|chiusa\|sciolta'
awk '/^\*\*Le decisioni, numerate/{s=1} s&&/^## 1\./{s=0} s&&/^\| [0-9]+ \|/{n++} END{print n}' "$G"
grep -c '2026-09-04' "$G"
grep -c 'aperta, dipendenza dichiarata' "$G"
grep -cE 'sede da assegnare' docs/tracciabilita.md
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
grep -c "in testa all.ADR" docs/COMPENDIO.md
```

Atteso: `OK`, `GATE GREEN`, margine positivo, `0` CR sui due LF, CR = righe sui due CRLF,
`ls-files --eol` invariato; nell'ADR **1** riga `-` e **1** riga `+`, e `1` rimando; le quattro
righe del primo censimento portano tutte una parola fra *decisa*, *chiusa*, *sciolta* → **4**;
ancora **13** decisioni; le occorrenze di `2026-09-04` nel disegno dei gesti salite di **sette** rispetto a prima del compito (si misura prima, al Passo 1, e si sottrae); `0` celle *«aperta,
dipendenza dichiarata»*; `sede da assegnare` ancora **3**; il conteggio per stato **identico** al
Passo 1; «in testa all'ADR» **invariato** rispetto alla fine del compito 1 (D6).

```bash
git add docs/adr/0039-telecamera-come-sorgente-di-percezione.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md docs/tracciabilita.md docs/COMPENDIO.md docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md
git commit -m "knowledge-base(compito 2): la decisione 7 dei gesti — dove finisce la cattura — chiusa in tutte le sue nove case: la cella di ADR-0039, le sette case del disegno dei gesti, la riga di tracciabilità, e la voce di §5 che vi rimanda"
git push
```

#### Criterio di chiusura del compito 2

- [ ] la cella di ADR-0039 porta il rimando datato con il testo vecchio in testa; nessun'altra riga dell'ADR è cambiata
- [ ] le sette case del disegno dei gesti dicono *decisa*, *chiusa* o *sciolta* con la data, e il comando delle decisioni rende ancora tredici
- [ ] la riga della cattura in tracciabilità dice la destinazione; `sede da assegnare` e il conteggio per stato invariati
- [ ] la revisione ha rilanciato **entrambi** i censimenti e letto intera ogni riga resa
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 3: `roadmap.md` — la riga 13, le celle 3 e 6, «Perché quest'ordine», l'intestazione

**Files:**
- Modify: `docs/roadmap.md` (**CRLF**) · questo piano (LF) — la posizione
- Read: §5.1 e §5.3 del disegno; le voci **4** e **5** della rilettura (scelte dal proprietario il 2026-09-04); le trappole 6 e 9; P-7, P-8, P-14 e D15 di questo piano

**Interfaces:**
- Consumes: il numero **13**, che i rimandi del compito 1 già nominano
- Produces: la riga **13** in coda alla tabella dei sotto-progetti, che il compito 5 verifica; la riga di «Perché quest'ordine» che comincia con `| **Registro delle guide (13) dopo`

- [ ] **Passo 1: le misure prima**

```bash
bash scripts/check-docs.sh
git ls-files --eol docs/roadmap.md
printf 'roadmap CR='; tr -cd '\r' < docs/roadmap.md | wc -c; printf '   righe='; wc -l < docs/roadmap.md
awk '/^## Sotto-progetti/{s=1;next} s&&/^## /{s=0} s&&/^[|] /' docs/roadmap.md | tail -3 | cut -c1-40
grep -n '^| 3 | Conversazione | L2 | ⬜ | 1, 2 |$' docs/roadmap.md
grep -n '^| 6 | Conoscenza / RAG | L2 | ⬜ | 3 |$' docs/roadmap.md
grep -c '^| 13 |' docs/roadmap.md
grep -n '^| \*\*Gesti dopo GUI minima' docs/roadmap.md
grep -n '^Ultimo aggiornamento' docs/roadmap.md
awk '/^## Decisioni ancora da prendere/{s=1} s&&/^## Regola/{s=0} s' docs/roadmap.md | grep -ci guid
awk '/^## Decisioni ancora da prendere/{s=1} s&&/^## Regola/{s=0} s' docs/roadmap.md | grep -ci knowledge
```

Atteso: `w/crlf` con CR = righe; la coda `10`, `11`, `12`; le righe 3 e 6 nella forma esatta;
`0` righe 13; **una** riga di «Gesti dopo»; **una** riga «Ultimo aggiornamento»; `0` e `0` nella
tabella delle decisioni (misura 10 del disegno — due `grep` per la trappola 14). ⚠️ Prima di
fidarsi degli `0`, la seconda direzione: `awk '/^## Sotto-progetti/{s=1} s&&/^## Perché/{s=0} s' docs/roadmap.md | grep -ci gesti`
deve rendere **almeno 1**.

- [ ] **Passo 2: le celle 3 e 6, e la riga 13 in coda**

| Trova | Sostituisci con |
|---|---|
| `\| 3 \| Conversazione \| L2 \| ⬜ \| 1, 2 \|` | `\| 3 \| Conversazione \| L2 \| ⬜ \| 1, 2, **13** — è la prima capacità che inietta una guida, assunzione dichiarata nella §6.5 del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md): se non lo fosse, il 13 va comunque prima della prima che lo fa \|` |
| `\| 6 \| Conoscenza / RAG \| L2 \| ⬜ \| 3 \|` | `\| 6 \| **Conoscenza — la mappa, poi la ricerca** — in due metà: la **mappa** (archivio unico di file a router → gruppi → foglie, skill comprese, una guida per modello; il sensore d'integrità; l'indice tenuto dal core e il pannello della GUI; le misure nel giornale), poi la **ricerca** per somiglianza sulla stessa cartella, via arbitro — [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md), §4.4 \| L2 \| ⬜ \| 3 (→ 13), e **2** per il pannello \|` |
| la riga della tabella dei sotto-progetti che comincia con `\| 12 \| **Gesti**` — **intera**, presa dal file | resta; **sotto** di essa: `\| 13 \| **Registro delle guide, trigger e proiezione** — i tre meccanismi decisi in [ADR-0009](adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md), [ADR-0008](adr/0008-contesto-come-proiezione-dello-stato.md) e [ADR-0010](adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md), senza codice al 2026-09-04, costruiti **prima** della prima capacità che inietta una guida; le due pretese della mappa — chiave di contesto (ambito, run, modello); provenienza e impronta, con «approvate ora» come proiezione del giornale — e l'ambito della cartella ([ADR-0024](adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md)) — [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md), §1.1d–e e §2 \| L0 + L1 \| ⬜ \| 1 · **AUD-004 deciso**: se le difese di ADR-0015 si estendano alle skill è un ADR del proprietario — la decisione registrata in [\`audit-2026-08-27.md\`](audit-2026-08-27.md), *«Le decisioni prese rimediando»* \|` |

⛔ **Senza rinumerare** (trappola 6): la riga 13 va **in coda** alla tabella, e le righe 0…12 non
cambiano posto. Il nome della riga è la voce **4** della rilettura; la cella *«Dipende da»* è
**D15**.

- [ ] **Passo 3: «Perché quest'ordine» — la riga del 13 (voce 5, D4)**

| Trova | Sostituisci con |
|---|---|
| la riga che comincia con `\| **Gesti dopo GUI minima e Conversazione, e prima di Voce** \|` — **intera**, presa dal file | resta; **sotto** di essa: `\| **Registro delle guide (13) dopo la GUI minima e prima di Conversazione** \| i tre meccanismi senza codice si costruiscono prima della prima capacità che inietta una guida (§1.1d del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md)), e il 2 non li usa. ⛔ A sbarrare il 13 è la decisione registrata da AUD-004 — un ADR del proprietario, da scrivere in parallelo al 2 — non il 2 \|` |

Il «Perché quest'ordine» è **riletto per intero** contro la riga nuova: *«Conversazione come prima
capacità»* resta vera — la prima **capacità**, e il 13 è kernel — e nessun'altra riga parla del
registro delle guide. Se una lo facesse, riceve un richiamo datato.

- [ ] **Passo 4: l'intestazione (P-8)**

| Trova | Sostituisci con |
|---|---|
| la riga che comincia con `Ultimo aggiornamento: **` — **intera**, presa dal file | `Ultimo aggiornamento: **<data>**, con la riga **13 «Registro delle guide, trigger e proiezione»**, la dipendenza nuova della riga 3, la riga 6 riscritta in due metà e la riga di «Perché quest'ordine» — dalla §5.1 del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md).` |

- [ ] **Passo 5: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
printf 'roadmap CR='; tr -cd '\r' < docs/roadmap.md | wc -c; printf '   righe='; wc -l < docs/roadmap.md
git ls-files --eol docs/roadmap.md
awk '/^## Sotto-progetti/{s=1;next} s&&/^## /{s=0} s&&/^[|] /' docs/roadmap.md | tail -3 | cut -c1-40
grep -c '^| 3 | Conversazione | L2 | ⬜ | 1, 2, \*\*13\*\*' docs/roadmap.md
grep -c '^| 6 | \*\*Conoscenza — la mappa, poi la ricerca\*\*' docs/roadmap.md
grep -c '^| \*\*Registro delle guide (13) dopo' docs/roadmap.md
grep -c 'Conoscenza / RAG' docs/roadmap.md
grep -c 'knowledge-base-design' docs/roadmap.md
git log -1 --format=%ad --date=short -- docs/roadmap.md; grep -o '^Ultimo aggiornamento: \*\*[0-9-]*' docs/roadmap.md
git diff --stat
```

Atteso: `OK`, `GATE GREEN`, CR = righe, `ls-files --eol` invariato, la coda `11`, `12`, `13`,
`1`, `1`, `1`, `0` «Conoscenza / RAG», i link al disegno **saliti di cinque** rispetto al Passo 1
(righe 3, 6, 13, «Perché», intestazione), e la data dell'intestazione uguale a quella del commit
che seguirà — cioè `<data>`.

```bash
git add docs/roadmap.md docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md
git commit -m "knowledge-base(compito 3): la riga 13 «Registro delle guide, trigger e proiezione» in roadmap, L0 + L1, che dipende da 1 e da AUD-004 deciso; Conversazione dipende anche da 13; Conoscenza in due metà, la mappa poi la ricerca, col 2 per il pannello; la riga di «Perché quest'ordine»"
git push
```

#### Criterio di chiusura del compito 3

- [ ] la riga 13 esiste, in coda, con `L0 + L1` e la dipendenza da 1 e da AUD-004; le righe 0…12 non sono rinumerate
- [ ] la cella 3 dipende anche da 13 con l'assunzione dichiarata; la cella 6 dice le due metà e dipende anche dal 2
- [ ] «Perché quest'ordine» ha la riga del 13 nella forma scelta dal proprietario, col link al posto di «questo disegno», e nessuna riga falsa
- [ ] l'intestazione porta la data del commit
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 4: `tracciabilita.md` — le cinque righe che restano, e il riquadro in testa

**Files:**
- Modify: `docs/tracciabilita.md` (**CRLF**) · questo piano (LF) — la posizione
- Read: §5.2 del disegno; P-11 e D2 di questo piano; la trappola 7

**Interfaces:**
- Consumes: la riga **13** creata dal compito 3, che le celle nominano per numero (D2)
- Produces: `sede da assegnare` da **3** a **1**

- [ ] **Passo 1: le misure prima, nelle due direzioni**

```bash
bash scripts/check-docs.sh
git ls-files --eol docs/tracciabilita.md
printf 'tracciabilita CR='; tr -cd '\r' < docs/tracciabilita.md | wc -c; printf '   righe='; wc -l < docs/tracciabilita.md
grep -cE 'sede da assegnare' docs/tracciabilita.md
grep -n 'sede da assegnare' docs/tracciabilita.md | cut -c1-60
grep -n '^| Memoria persistente | 🔶 | stato durevole §4 · politica → Conoscenza |$' docs/tracciabilita.md
grep -n '^| Collezioni e knowledge base | 📋 | Conoscenza |$' docs/tracciabilita.md
grep -n '^| File watching e awareness del progetto | 🔶 | trigger anello 3 §5 · politica → Conoscenza |$' docs/tracciabilita.md
grep -n '^| Skills | 🔶 | guide §5 (ADR-0003) · registro delle guide → sede da assegnare |$' docs/tracciabilita.md
grep -n '^| Regole e vincoli di progetto | 🔶 | guide §5 · registro delle guide → sede da assegnare |$' docs/tracciabilita.md
grep -n '^> ✅ \*\*Aggiornata il 2026-09-03 con le righe del riconoscimento gesti\*\*' docs/tracciabilita.md
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
awk '/^## 3\. Conoscenza/{s=1;next} s&&/^## /{s=0} s&&/^[|] /&&!/^[|] Funzionalità/{n++} END{print n}' docs/tracciabilita.md
```

Atteso: `w/crlf` con CR = righe; **3** `sede da assegnare` — `Skills`, `Regole e vincoli`,
`Storage e cifratura`; ciascun `grep -n` rende **una** riga; il conteggio per stato annotato; la
sezione 3 a **sedici** righe (§6.2 del disegno).

- [ ] **Passo 2: le cinque righe**

| Trova | Sostituisci con |
|---|---|
| `\| Memoria persistente \| 🔶 \| stato durevole §4 · politica → Conoscenza \|` | `\| Memoria persistente \| 🔶 \| stato durevole §4 · politica → **6**: l'assistente scrive una nota quando giudica che vale, come scrittura giornalata, e il giornale resta la verità — risposta 5 del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md) \|` |
| `\| Collezioni e knowledge base \| 📋 \| Conoscenza \|` | `\| Collezioni e knowledge base \| 📋 \| **6, la mappa** — archivio unico a mappa (router → gruppi → foglie, skill comprese, una guida per modello), il sensore d'integrità, l'indice e il pannello; poi la ricerca, seconda metà — [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md) \|` |
| `\| File watching e awareness del progetto \| 🔶 \| trigger anello 3 §5 · politica → Conoscenza \|` | `\| File watching e awareness del progetto \| 🔶 \| trigger anello 3 §5 → **13**, il meccanismo in \`kernel\` e la sorveglianza dei file in \`platform\` · politica → **6** \|` |
| `\| Skills \| 🔶 \| guide §5 (ADR-0003) · registro delle guide → sede da assegnare \|` | `\| Skills \| 🔶 \| guide §5 (ADR-0003) · registro delle guide → **13**, con provenienza e impronta all'approvazione (§1.1e del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md)) · se le difese di ADR-0015 si estendano alle skill: AUD-004, un ADR del proprietario prima del 13 \|` |
| `\| Regole e vincoli di progetto \| 🔶 \| guide §5 · registro delle guide → sede da assegnare \|` | `\| Regole e vincoli di progetto \| 🔶 \| guide §5 · registro delle guide → **13** \|` |

- [ ] **Passo 3: il riquadro in testa**

| Trova | Sostituisci con |
|---|---|
| la riga che comincia con `> ✅ **Aggiornata il 2026-09-03 con le righe del riconoscimento gesti**` — **intera**, presa dal file | resta; **sotto** di essa, una riga `>` vuota e: `> ✅ **Aggiornata il <data> con le righe della knowledge base**, dalla §5.2 del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md) approvato e riletto dal proprietario — fuori da una chiusura di sotto-progetto, e per questo detto: il registro delle guide e i trigger hanno la sede **13**, «Registro delle guide, trigger e proiezione»; la Conoscenza è il **6** in due metà, la mappa e poi la ricerca; e la cattura con un gesto ha la destinazione decisa. Il marcatore \`sede da assegnare\` resta sulla sola cifratura reale.` |

- [ ] **Passo 4: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
printf 'tracciabilita CR='; tr -cd '\r' < docs/tracciabilita.md | wc -c; printf '   righe='; wc -l < docs/tracciabilita.md
git ls-files --eol docs/tracciabilita.md
grep -cE 'sede da assegnare' docs/tracciabilita.md
grep -n 'sede da assegnare' docs/tracciabilita.md | cut -c1-40
grep -c '→ \*\*13\*\*' docs/tracciabilita.md
grep -c 'knowledge-base-design' docs/tracciabilita.md
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
awk '/^## 3\. Conoscenza/{s=1;next} s&&/^## /{s=0} s&&/^[|] /&&!/^[|] Funzionalità/{n++} END{print n}' docs/tracciabilita.md
git diff --stat
```

Atteso: `OK`, `GATE GREEN`, CR = righe, `ls-files --eol` invariato, **1** `sede da assegnare` ed è
*«Storage e cifratura a riposo»*, **3** celle `→ **13**`, i link al disegno **saliti di quattro**
rispetto al Passo 1 (memoria, collezioni, skills, riquadro — la riga della cattura ne portava già
uno dal compito 2), il conteggio per stato **identico**, ancora **sedici** righe nella sezione 3.

```bash
git add docs/tracciabilita.md docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md
git commit -m "knowledge-base(compito 4): le righe di tracciabilità — Skills e Regole e vincoli al 13, la Conoscenza al 6 in due metà, la memoria all'assistente che scrive, il file watching col trigger al 13; sede da assegnare da tre a uno, resta la cifratura reale"
git push
```

#### Criterio di chiusura del compito 4

- [ ] le cinque righe dicono la sede nuova; `sede da assegnare` è a uno e resta sulla cifratura
- [ ] nessuna riga cambia simbolo di stato; il conteggio per stato è identico a prima
- [ ] il riquadro in testa porta la riga datata col nome del 13
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 5: la chiusura — la Definizione di «fatto», e i documenti di stato

**Files:**
- Modify: `docs/COMPENDIO.md` (**CRLF**) — il puntatore in §6, due righe in §12, l'intestazione · `docs/README.md` (**CRLF**) — la riga del disegno nella tabella «Specifiche» (P-1) · `docs/roadmap.md` (**CRLF**) — la riga di questo piano nella tabella dei piani, e l'intestazione (P-8) · il disegno della knowledge base (LF) — la spunta della §5.5 e un richiamo (D13) · questo piano (LF) — la posizione
- Read: la Definizione di «fatto» nella §5.5 del disegno, che è la tabella di questo compito; D9 e D13

**Interfaces:**
- Consumes: tutto ciò che i compiti 1–4 hanno prodotto, verificato **coi comandi** e non letto dalla tabella della posizione

- [ ] **Passo 1: la Definizione di «fatto», condizione per condizione, coi comandi**

`<base>` è il comando del vincolo 9 in testa a questo piano.

| # | Condizione (§5.5 del disegno) | Comando, ed esito atteso |
|---|---|---|
| 1 | i rimandi datati in testa a 0009, 0008, 0010, 0038 e nella riga di 0039, append-only; le voci §5 del compendio vi rimandano | `grep -c 'Rimando del' docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md docs/adr/0039-*.md` → `1` ciascuno; `grep -c "in testa all.ADR" docs/COMPENDIO.md` → `7`; `git diff <base>..HEAD -- docs/adr/0008-*.md docs/adr/0009-*.md docs/adr/0010-*.md docs/adr/0038-*.md \| grep -c '^-[^-]'` → `0`; `git diff <base>..HEAD -- docs/adr/0039-*.md \| grep -c '^-[^-]'` → `1` |
| 2 | `roadmap.md`: riga 13 senza rinumerare; celle 3 e 6 riscritte; «Perché quest'ordine» riletto | `awk '/^## Sotto-progetti/{s=1;next} s&&/^## /{s=0} s&&/^[\|] /' docs/roadmap.md \| tail -3 \| cut -c1-12` → `11`, `12`, `13`; `grep -c '^\| \*\*Registro delle guide (13) dopo' docs/roadmap.md` → `1`; `bash scripts/check-docs.sh` → `OK` |
| 3 | `tracciabilita.md`: le sei righe; `sede da assegnare` da 3 a 1 nelle due direzioni; il comando del riquadro conta | `grep -cE 'sede da assegnare' docs/tracciabilita.md` → `1`; `grep -c '→ \*\*13\*\*' docs/tracciabilita.md` → `3`; `grep -c 'decisa il 2026-09-04' docs/tracciabilita.md` → `1`; il `for s in …` del riquadro gira e rende `46 · 52 · 79 · 0 · 1` — le cifre del 2026-09-04, che nessun compito muove |
| 4 | disegno gesti: decisione 7 → ✅ col richiamo datato; ADR-0039 idem | `grep -c 'aperta, dipendenza dichiarata' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` → `0`; `awk '/^\*\*Le decisioni, numerate/{s=1} s&&/^## 1\./{s=0} s&&/^\| [0-9]+ \|/{n++} END{print n}' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` → `13`; `grep -c 'Rimando del' docs/adr/0039-*.md` → `1` |
| 5 | il disegno in §12 e in `README.md`; il puntatore §6 mosso; compendio sotto il tetto | i Passi 2 e 3 qui sotto, poi `check-docs.sh` e il comando del vincolo 9 |
| 6 | fine-riga rimisurati per ogni file toccato | `git ls-files --eol` sui file della mappa, contro la colonna della mappa: **invariato** |
| 7 | `git diff --stat <base>..HEAD -- crates/ scripts/ Cargo.lock` vuoto | il comando, più `Cargo.toml rust-toolchain.toml`: **vuoto** |

E il vincolo globale 1, rilanciato: le due spec intatte.

- [ ] **Passo 2: il compendio, CRLF — §6, §12, l'intestazione**

| Trova | Sostituisci con |
|---|---|
| in §6, dal testo `⏭️ **IL PROSSIMO PASSO: ESEGUIRE QUEL PIANO in una sessione NUOVA**` fino a `⛔ **AUD-004 sbarra il 13**, non il piano.` compreso — letto **intero** prima di sostituirlo, e preso dal file perché attraversa più righe | `✅ **E IL PIANO È ESEGUITO IL <data>**, `GATE GREEN` a ogni compito: i quattro rimandi in testa a ADR-0008, 0009, 0010 e 0038 e la riga di ADR-0039; la riga 13 «Registro delle guide, trigger e proiezione» in roadmap con le celle 3 e 6; le sei righe di tracciabilità; la decisione 7 dei gesti chiusa in tutte le sue case. ⏭️ **IL PROSSIMO PASSO: IL SOTTO-PROGETTO 2, la GUI minima** — dal suo brainstorming con \`superpowers:brainstorming\`, in una sessione nuova; ADR-0029 si chiude con M1–M5 al suo inizio (§4) — **e in parallelo AUD-004**, l'ADR del proprietario che sbarra il 13 (voce 3 della rilettura del disegno della knowledge base); poi il 13, poi il 3 — decisione 16. ⛔ **AUD-004 sbarra il 13**, non il 2.` |
| in §12, la riga `\| come si è **eseguito** il riconoscimento gesti — i due ADR, i rimandi, la roadmap, SP-7 e la sonda S3, con l'errata in testa e la tabella della posizione \| [\`plans/2026-09-03-riconoscimento-gesti.md\`](superpowers/plans/2026-09-03-riconoscimento-gesti.md) — ⚠️ **a compiti, mai intero** \|` | resta; **sotto**, due righe: `\| ⛔ **il perimetro della KNOWLEDGE BASE** — che cosa la mappa chiede al kernel e dove va: la strada B, i tre meccanismi del sotto-progetto 13 con le due pretese, le CRUD nel registro delle funzioni, il pannello col 6, le decisioni col loro chiusore, e per ogni artefatto il controllo che lo esercita \| [\`specs/2026-09-04-knowledge-base-design.md\`](superpowers/specs/2026-09-04-knowledge-base-design.md) — ⚠️ **non è una spec**, e **non disegna la capacità** \|` e `\| come si è **eseguito** il piano dei documenti della knowledge base — i rimandi in testa a quattro ADR e nella riga di ADR-0039, la riga 13 in roadmap, le righe di tracciabilità, la decisione 7 dei gesti chiusa, con l'errata in testa e la tabella della posizione \| [\`plans/2026-09-04-knowledge-base-documenti.md\`](superpowers/plans/2026-09-04-knowledge-base-documenti.md) — ⚠️ **a compiti, mai intero** \|` |
| l'intestazione, da `**Aggiornato il` a `Manutenzione: §13.` sulla stessa riga | `**Aggiornato il <data>**, col **PIANO DEI DOCUMENTI DELLA KNOWLEDGE BASE ESEGUITO** e il puntatore della §6 mosso al sotto-progetto 2 con AUD-004 in parallelo; l'ultimo contenuto di **merito** è quel puntatore. Manutenzione: §13.` |

Dopo ogni tocco: `check-docs.sh` e il margine. ⛔ **Se il tetto va rosso si toglie prosa dalla §6,
non si alza il tetto.**

- [ ] **Passo 3: `README.md`, CRLF — la tabella «Specifiche» (P-1)**

Trova = la riga che comincia con `| [Riconoscimento gesti — il disegno]` — **intera**, presa dal
file, unica; resta, e **sotto** di essa:
`| [Knowledge base — il disegno](superpowers/specs/2026-09-04-knowledge-base-design.md) | la knowledge base: che cosa chiede al kernel, e dove va | ⛔ **Non è una spec**, e **non disegna la capacità**: perimetro, la forma nel kernel, i rimandi in append, la GUI e il sotto-progetto 6 in due metà, le voci aperte col loro chiusore, e per ogni artefatto il controllo che lo esercita; il verdetto — nessuna sesta proprietà «che non si aggiunge dopo», ma un vincolo d'ordine: il sotto-progetto 13 prima del 3 |`

- [ ] **Passo 4: `roadmap.md`, CRLF — la riga di questo piano (D8), e l'intestazione (P-8)**

| Trova | Sostituisci con |
|---|---|
| nella riga della tabella dei piani che comincia con `\| [Knowledge base — i documenti](superpowers/plans/2026-09-04-knowledge-base-documenti.md)`, la cella di stato da `✅ **scritto il 2026-09-04**` fino alla fine della cella, presa dal file | `✅ **scritto il 2026-09-04, eseguito il <data>** — \`GATE GREEN\` a ogni compito; nessun file di \`crates/\` toccato` |
| la riga che comincia con `Ultimo aggiornamento: **` — **intera**, presa dal file | `Ultimo aggiornamento: **<data>**, col **piano dei documenti della knowledge base eseguito** nella tabella dei piani; la riga **13** e le celle 3 e 6 erano arrivate col suo compito 3, dalla §5.1 del [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md).` |

- [ ] **Passo 5: il disegno della knowledge base, LF — la spunta e il richiamo (D13)**

| Trova | Sostituisci con |
|---|---|
| nella §5.5, punto 4: `l'**esecuzione** va in una sessione nuova, e a che punto sia lo dice la tabella della posizione del piano` | `**eseguito il <data>**, tutti i compiti con \`GATE GREEN\`; il verbale per compito sta nella tabella della posizione del piano` |
| sotto `## Il prossimo passo`, la riga `✅ **RICHIAMO DEL 2026-09-04, sera:** il piano è scritto col pre-controllo; l'esecuzione va in una sessione nuova, e la §6 del compendio lo dice.` | resta — è un verbale — e **sotto** di essa: `✅ **RICHIAMO DEL <data>:** il piano è eseguito; la §6 del compendio porta il passo successivo, il sotto-progetto 2 con AUD-004 in parallelo.` |

- [ ] **Passo 6: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/COMPENDIO.md docs/README.md docs/roadmap.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
printf 'disegno CR='; tr -cd '\r' < docs/superpowers/specs/2026-09-04-knowledge-base-design.md | wc -c
git ls-files --eol docs/COMPENDIO.md docs/README.md docs/roadmap.md docs/superpowers/specs/2026-09-04-knowledge-base-design.md
grep -n '⏭️' docs/COMPENDIO.md | cut -c1-80
grep -c 'knowledge-base-design' docs/COMPENDIO.md docs/README.md
grep -c 'knowledge-base-documenti' docs/COMPENDIO.md docs/roadmap.md
git log -1 --format=%ad --date=short -- docs/roadmap.md; grep -o '^Ultimo aggiornamento: \*\*[0-9-]*' docs/roadmap.md
git diff --name-only <base>..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
git status --porcelain
```

Atteso: `OK`, `GATE GREEN`, margine positivo, CR = righe sui tre CRLF e `0` sul disegno,
`ls-files --eol` invariato, **due** righe col marcatore nel compendio — il puntatore vivo in §6 e
la riga della tabella delle voci aperte che **nomina** il marcatore, le stesse due di `<base>` —
`2` e `1` link al disegno, `1` e `1` link a questo piano, la data dell'intestazione di roadmap
uguale al `<data>` del commit, il diff sul codice e sulle spec **vuoto**, e `git status` che
nomina i cinque file di questo compito.

```bash
git add docs/COMPENDIO.md docs/README.md docs/roadmap.md docs/superpowers/specs/2026-09-04-knowledge-base-design.md docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md
git commit -m "knowledge-base(compito 5): il piano dei documenti è ESEGUITO — la Definizione di «fatto» rilanciata coi comandi, il disegno e il piano nella §12 e in README, e il prossimo passo nella §6: il sotto-progetto 2 con AUD-004 in parallelo"
git push
```

#### Criterio di chiusura del compito 5

- [ ] le sette condizioni della Definizione di «fatto» verificate **coi comandi** del Passo 1, ed elencate nel rapporto con l'output
- [ ] il disegno sta nei **due** indici che il disegno nomina — §12 del compendio, «Specifiche» di `README.md` — e questo piano nella §12 e nella tabella dei piani di `roadmap.md`
- [ ] il puntatore della §6 nomina il sotto-progetto 2 con AUD-004 in parallelo, in un posto solo
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, margine positivo, fine-riga rimisurati, commit pushato, posizione del piano a cinque ✅

---

## Dopo il compito 5

Il piano è chiuso quando la tabella della posizione porta cinque ✅ e la §6 del compendio nomina il
sotto-progetto 2. Ciò che il piano lascia aperto sta nella tabella *«Le voci aperte che questo piano
SA, e non chiude»*: nessuna voce nasce da questo piano, e se una nascesse eseguendo, va nell'errata
con il proprio chiusore.

### Come si riprende — scritto alla chiusura della sessione del 2026-09-04, coi comandi

⚠️ **È il documento di consegna della sessione che ha scritto questo piano**, e sta qui perché il
repo ha già la sua convenzione: lo stato vive in file tracciati, e chi esegue legge questo file per
intero prima del primo compito. Ogni riga è stata **riletta coi comandi** prima di essere scritta.

⛔ **DA SAPERE SUBITO: niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a
metà, tutto pushato, nessun codice toccato. La sessione ha fatto tre cose, tutte chiuse: la
rilettura del disegno da parte del proprietario (sei voci, sei consigli scelti, registrata nel
disegno), questo piano col pre-controllo, e i puntatori.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git status -sb` dopo `git fetch --all --prune`. Nessuno stash, nessuna operazione a metà |
| I commit di questa sessione | `git log --oneline 8fb9a94..HEAD` — la rilettura registrata; questo piano coi puntatori; la chiusura della sessione, che ha riletto questa sezione coi comandi e corretto la riga del cancello |
| Codice di prodotto | **non toccato**: `git diff --stat 8fb9a94..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende nulla |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN`, rilanciato all'apertura, prima del commit del piano e prima del commit di questa chiusura. ⚠️ Il commit della rilettura, `7c4881c`, di soli documenti, ha avuto il solo `bash scripts/check-docs.sh` → `OK`, e il cancello intero è passato subito dopo sullo stesso contenuto — detto invece che arrotondato. Si rilanciano, non si citano |
| Fine-riga | questo piano e i due disegni sono **LF** nell'indice e nell'albero; il compendio e `roadmap.md` sono LF nell'indice e **CRLF** nell'albero, con CR = righe: `git ls-files --eol docs/COMPENDIO.md docs/roadmap.md docs/superpowers/specs/2026-09-04-knowledge-base-design.md docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md` |
| File temporanei | nessuno nel repository: l'aiutante e i ritagli stanno nello scratchpad, fuori dall'albero |
| Debito lasciato | **nessuno non dichiarato**: le voci aperte sono nella tabella *«Le voci aperte che questo piano SA»*, col loro chiusore |

**Le decisioni prese dal coordinatore, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | **commit senza il trailer `Co-Authored-By`** | `CLAUDE.md` dice *«senza co-autore»*, e `git log --format='%b' -40 \| grep -ci 'co-authored-by'` rende **0**; una direttiva di sistema chiedeva il contrario, e la divergenza è **portata al proprietario** — quinta sessione di fila. Costo se sbagliato: un `--amend` |
| 2 | la rilettura del proprietario è **registrata nel disegno** — testa, §5.5, tabella delle voci, misura 12 — e non in un file a parte | il disegno stesso prescriveva *«ciò che la rilettura aggiunge si registra nella stessa sezione, come fece il disegno dei gesti»*. Costo se sbagliato: zero |
| 3 | la decisione 7 dei gesti si chiude in **nove** case in un compito solo (D1), mentre la trappola 10 del disegno ne nominava una | P-4, misurato con due censimenti; AUD-013. Costo se sbagliato: un compito più lungo del previsto |
| 4 | il piano è scritto **nella stessa sessione** della rilettura, come la strada B della consegna precedente prevedeva | il contesto reggeva, e il proprietario aveva scelto B. Costo se sbagliato: zero |
| 5 | `riferimenti.md`, `HANDOFF.md`, `AVVIO-CHAT.md` e `CLAUDE.md` **non si toccano** (D11, D12, vincolo 14) | nessuna fonte, nessun ADR nuovo, nessun totale che cambia. Costo se sbagliato: una riga da aggiungere |

**Il compito della sessione successiva: ESEGUIRE questo piano.** In ordine, e ogni riga è
eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -3`: si parte da `main`, e
   la testa deve essere il commit di questa chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e la testa
   dell'audit del 2026-08-27 — poi il **disegno della knowledge base per intero**, poi **questo
   piano fino al compito 1 compreso**. I compiti si leggono uno per volta, quando si dispacciano.
3. `superpowers:subagent-driven-development`: un subagente fresco per compito, con il testo del
   compito, l'errata, i vincoli globali e le sezioni del disegno che il compito nomina — non il
   disegno intero, non questo piano intero; revisione fra un compito e l'altro, col revisore
   istruito a **rilanciare ogni comando** accanto a un'affermazione misurabile e a elencarli.
4. Prima di dispacciare ciascun compito, il coordinatore lo rilegge contro i documenti di
   **allora** (le quattro domande di `CLAUDE.md`): il pre-controllo di questo piano è del
   2026-09-04 su `7c4881c`, e un compito eseguito dopo un altro legge un repository che quell'altro
   ha mosso — i Trova dei compiti 2, 3, 4 e 5 sono presi **dal file**, non da qui, dove il piano lo
   dice.
5. Ogni difetto trovato va nell'errata **prima** del rimedio, col proprio numero.
6. A piano eseguito, il compito 5 ha già mosso la §6 del compendio: la sessione successiva apre
   il **sotto-progetto 2** dal suo brainstorming, e il proprietario scrive **AUD-004** in
   parallelo.

📌 **Ciò che questa sessione consegna a chi esegue**, ed è suo e non un puntatore: i Trova e i
Sostituisci di ogni compito, verificati unici il 2026-09-04; il doppio censimento delle case della
decisione 7 (P-4); la sonda giusta per la cella di ADR-0039 (P-3); e la regola di P-8 sulla riga
«Ultimo aggiornamento» di `roadmap.md`, che ha già ceduto due volte nel piano precedente.

📌 **La lezione di questa sessione, e non è un gotcha nuovo.** Un disegno che nomina *«la cella
della riga 7»* come casa unica di una decisione era stato scritto guardando la tabella: il
censimento con **due** forme ne ha trovate nove. È il gotcha **#70** — *un `grep` restituisce
candidate, non case* — nella direzione che costa di più: quella in cui la parola cercata non c'è.

⛔ **Vicoli ciechi di questa sessione: nessuno nuovo.**
