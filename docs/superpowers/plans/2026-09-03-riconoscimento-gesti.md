# Riconoscimento gesti dalla telecamera: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario (§5.5, punto 4, del [disegno](../specs/2026-09-03-riconoscimento-gesti-design.md)).
> I passi usano le caselle (`- [ ]`) per il tracciamento. ⛔ **Il pre-controllo di ogni compito è
> fatto nella sessione che ha scritto il piano, il 2026-09-03, contro il repository a `1c0a633`;
> l'esecuzione va in una sessione NUOVA.**

**Obiettivo.** Tradurre in artefatti il disegno del riconoscimento gesti, riletto dal
proprietario il 2026-09-03: le due decisioni in append — ADR-0038, il registro delle funzioni
del programma; ADR-0039, la telecamera come sorgente di percezione — i tre richiami datati, la
riga 12 «Gesti» della roadmap con le sue dipendenze, le righe di tracciabilità, le fonti F1–F9,
la sonda S3 nel kernel, lo spike SP-7 col suo esito, e la chiusura nei documenti di stato.

**Forma.** Compiti in sequenza, e quanti siano lo dice la tabella della posizione qui sotto.
**Un solo file di `crates/` cambia** — il banco `crates/kernel/tests/arbiter_admission.rs`, al
compito 6 — e nessun file di `crates/*/src/`: questo piano **non costruisce il worker**, non crea
`workers/`, non tocca `Cargo.lock`. Lo spike vive in `spikes/`, fuori dal workspace. Le decisioni
sono già prese dal disegno; questo piano le traduce in passi, e le poche che aggiunge stanno
nella tabella *«Le decisioni prese da questo piano»*.

**Strumenti.** `bash`, `awk`, `grep`, `sed -n` in lettura; **Python 3** per ogni scrittura su un
file **CRLF**, con l'aiutante qui sotto; `cargo` sempre con `--locked`; `git`. La porta di
qualità è `bash scripts/gate.sh`, e deve stampare `GATE GREEN` **prima di ogni commit**, anche
di soli documenti. Per lo spike: `py -3.10` (P-4), una `venv`, e la telecamera di questa
macchina.

⛔ **L'aiutante `replace_unique.py` vive nello scratchpad, mai nel repository.** Sostituisce
**una** occorrenza unica, conserva i fine-riga del file, e rifiuta se il testo vecchio manca o
non è unico. I testi *Trova* e *Sostituisci con* di ogni compito si mettono in due file e si
passano a lui:

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

**Disegno:** [`2026-09-03-riconoscimento-gesti-design.md`](../specs/2026-09-03-riconoscimento-gesti-design.md)
— si legge **prima** dei compiti, per intero: le cinque sezioni approvate, la §6 e la §7, e le
sezioni di coda.

## Vincoli globali

Valgono per ogni compito, senza che il compito li ripeta.

| # | Vincolo | Da |
|---|---|---|
| 1 | **le due spec non si toccano**: `git diff --name-only 1c0a633..HEAD -- docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` resta vuoto a ogni compito. Il disegno **non è una spec** e si tocca dove i compiti lo dicono | testa del disegno; §7.4 e §8 sono spec |
| 2 | **nessuna riga di catalogo §7.4 nasce**; la sonda S3 si **registra** senza riga (gotcha #36) | disegno §4.2 |
| 3 | **nessun codice di prodotto**: `git diff --name-only 1c0a633..HEAD -- crates/ Cargo.lock Cargo.toml` nomina al più `crates/kernel/tests/arbiter_admission.rs`; `workers/` non nasce; i quattro file `i/crlf` — `crates/kernel/src/ports/process.rs`, `crates/kernel/tests/ports_are_implementable.rs`, `crates/kernel/tests/reactor_contract.rs`, `crates/platform/src/reactor.rs` — non si toccano | disegno §2.2 e trappola 9 |
| 4 | **codice in inglese, documenti in italiano**: la sonda, lo spike (Python, Rust, la pagina) e i loro commenti in inglese; gli ADR, il protocollo dello spike e i verbali in italiano | §1.0 della spec |
| 5 | **nessuna cifra nuova in prosa**: date e comandi; una cifra che sostiene una decisione porta accanto il comando e la data, e vive in una casa sola | `CLAUDE.md` |
| 6 | **i fine-riga si conservano per file** e si rimisurano dopo ogni scrittura: la mappa dei file dice CRLF o LF; `git ls-files --eol` prima e dopo, **invariato**; i file nuovi nascono **LF** | disegno trappola 3, `CLAUDE.md` |
| 7 | **una mutazione si revoca da una copia byte-esatta presa prima**, e la revoca si prova con `cmp` | precedente dei piani dei Traguardi 5 e 6 |
| 8 | **ogni conteggio si rifà col comando** prima di leggere che cosa un documento ne dice; le cifre di questo piano sono istantanee del 2026-09-03 su `1c0a633` | disegno, *Metodo* |
| 9 | **si toglie invece di riscrivere meglio**; un verbale non si riscrive; una correzione a una sezione approvata porta un richiamo datato | `CLAUDE.md` |
| 10 | **gli ADR sono append-only**: i richiami vanno **in testa**, sotto la riga `Deciders`, come blockquote datato — la forma di AUD-032 su ADR-0005 — e il compendio **rimanda** invece di ricopiare | disegno §3.2 |
| 11 | **un ADR nuovo, la sua voce in §5 del compendio, la sua riga nell'indice di `README.md` e i totali degli ADR nei documenti di stato entrano nello stesso commit**: `check-docs.sh` accoppia i file di `docs/adr/` alle voci di §5 e alle righe `\| [00` dell'indice, e confronta i totali. Le case dei totali le dà il comando del guardiano, che **toglie i code span** prima di contare (P-3): vedi il Passo 3 del compito 1 | disegno trappola 1 |
| 12 | **il compendio resta sotto il tetto**: margine misurato prima e dopo ogni tocco con il comando qui sotto; se va rosso si toglie prosa dalla §6, **non si alza il tetto** | disegno trappola 2 |
| 13 | **nessun link `](…)` a un file che non esiste ancora**: si nomina in code span, e il link nasce nel compito che crea il file. I piani sono fuori dal controllo dei link, gli altri documenti no | trappola 5 |
| 14 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore**; il cancello gira **prima** | `CLAUDE.md` |
| 15 | **l'accettazione condizionata del proprietario vale finché regge**: se un passo violasse uno dei cinque criteri di `anthropic-skills:decision-principles` — richiedesse una scorciatoia, duplicasse qualcosa che esiste, poggiasse su qualcosa che non è più vero — ci si **ferma e lo si riporta** | disegno, *Le regole di questo lavoro* |
| 16 | **il numeratore dei compiti** vive nella tabella della posizione e in nessun altro punto di questo file o del repository | gotcha #68 |

Il comando del margine, vincolo 12:

```bash
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
```

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

✅ **IL PIANO È SCRITTO IL 2026-09-03**, nell'ordine del punto 4 della §5.5 del disegno. Il
pre-controllo delle quattro domande su ciascun compito sta nella sezione *«Il pre-controllo del
piano»* qui sotto, fatto nella sessione che ha scritto il piano, come il disegno prescrive.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | ADR-0038, il registro delle funzioni del programma — con la voce in §5, la riga nell'indice e i totali | uno | ✅ 2026-09-03 |
| **2** | ADR-0039, la telecamera come sorgente di percezione — idem | uno | ✅ 2026-09-03 |
| **3** | i tre richiami datati in testa ad ADR-0001, ADR-0011 e ADR-0023, e i rimandi in §5 | uno | ✅ 2026-09-03 |
| **4** | la riga 12 e le dipendenze in `roadmap.md`; le righe di `tracciabilita.md`; lo spike SP-7 nella tabella degli spike | uno | ✅ 2026-09-03 |
| **5** | le fonti F1–F9 in `riferimenti.md` | uno | ✅ 2026-09-03 |
| **6** | la sonda S3 nel kernel, nelle due direzioni, e la sua riga nel registro | uno | ✅ 2026-09-04 |
| **7** | SP-7: il protocollo, **poi** il codice | **due** (D6) | ⬜ |
| **8** | SP-7: la misura col proprietario, e l'esito in `spikes/RISULTATI.md` | uno | ⬜ |
| **9** | la chiusura: §6, §8 e §12 del compendio, `README.md`, `roadmap.md`, il disegno; la Definizione di «fatto» | uno | ⬜ |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del
[`COMPENDIO.md`](../../COMPENDIO.md), in un posto solo. Ciò che resta qui è la **posizione** del
piano — la tabella qui sopra, che chi esegue aggiorna nel commit del compito — e **come** si
esegue.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro — e il
   disegno nelle sezioni che il compito nomina.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-03.
3. Se il compito dice il falso, **ci si ferma e si riporta**: non si aggira. Una divergenza è una
   voce d'errata prima di essere un rimedio.
4. Il cancello gira **prima** di ogni commit; il commit dice ciò che il compito ha fatto.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; per i
   compiti 1, 2 e 3 rilegge l'ADR **contro i suoi fratelli** — le decisioni che cita — e non solo
   contro il disegno (gotcha #59); per il compito 6 rilancia le **due** mutazioni.
6. Una seconda ondata di **sola prosa** la fa il coordinatore a mano, senza ri-revisione; se
   tocca un fatto o un comando, si ri-rivede. Dopo due ondate di prosa si chiude: il codice regge
   alla prima o alla seconda passata, le frasi no (gotcha #76 — si **toglie**, non si riscrive
   meglio).

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota, e non resterà vuota.** Il pre-controllo ha trovato un difetto reale in **tutti**
i compiti dispacciati finora, senza una sola eccezione: quando ne trovi uno, si scrive **qui**,
con il proprio numero, prima di eseguirlo. Un piano è un'ipotesi. ⚠️ **La voce 4 del
disegno** — l'etichetta di strato della riga 12 — è presa da questo piano col consiglio scritto
(D4): se il proprietario dice altro, la prima voce di questa errata la ribalta.

| # | Voce |
|---|---|
| **E1** | **Compito 1, Passo 6 — il `grep` sui link `../` dell'ADR rende quattro righe, non «due».** Scritta dal pre-controllo del 2026-09-03, prima di dispacciare, contando sul testo dettato al Passo 2: `grep -c '](\.\./'` conta le **righe** che portano un rimando relativo, e sono quattro — `../tracciabilita.md` in *Context* e in *Follow-up*, `../superpowers/specs/…` in *Context*, `../../spikes/GUI-REQUISITI.md` nella regola 7. La frase contava i **bersagli** distinti. Il criterio di chiusura non cambia: è il controllo dei link di `check-docs.sh`; il valore atteso del `grep` è **quattro** |
| **E2** | **Compito 1, Passo 6 — `git status --porcelain` nomina otto file, non «sette».** Stesso pre-controllo: l'ottavo è **questo piano**, che il compito modifica nello stesso commit — la tabella della posizione e questa errata — e che il `git add` dettato già elenca. «Sette» contava la mappa dei file del compito e non il piano |
| **E3** | **Compito 1, Passo 2 — la frase della Context dell'ADR *«rende la sola riga di ADR-0025»* è falsa dalla nascita.** Trovata dalla revisione del compito 1, il 2026-09-03, rilanciando il comando sul repository di **prima** dell'ADR: `git grep -n -i -E 'strument[oi] intern\|registro degli strumenti\|palette\|scorciatoi' 4d16f33 -- docs/superpowers/specs/2026-08-06-kernel-design.md 'docs/adr/*.md'` rende **sette** righe in cinque ADR, sei sul ramo `scorciatoi`; sul repository di oggi ne rende di più, perché ADR-0038 stesso porta le parole cercate. La conclusione regge, e il disegno (§6.1) diceva *«l'unica riga vicina»*: è il piano che l'ha irrigidita in *«la sola riga»*. Rimedio deciso dal coordinatore: **richiamo datato accanto alla frase**, come blockquote — la forma vera di AUD-032 su ADR-0005, append-only (E4) — col comando appuntato a `4d16f33` e **senza** il ramo `scorciatoi`, che rende la sola riga 52 di ADR-0025 — rilanciato prima di dettarlo; nello stesso richiamo le tre precisazioni della revisione: G20 dà la metà «tastiera» e non il click; ADR-0007 nomina il **chi** (l'utente) e tace la **strada**; la Decision non ripete che le funzioni sono gli **strumenti interni** del livello 1 di ADR-0025, che la Context dice |
| **E4** | **Vincolo 10 — «in testa, sotto la riga `Deciders`» non è la forma di AUD-032.** Il rimando di AUD-032 in ADR-0005 sta **dentro la Decision, fra i punti 2 e 3, accanto al paragrafo che corregge** (`grep -n 'Rimando del 2026-08-27' docs/adr/0005-arbitrato-gpu-su-due-dimensioni.md` letto contro `grep -n '^## ' docs/adr/0005-arbitrato-gpu-su-due-dimensioni.md`), come i richiami di AUD-004 in ADR-0015 (nella Context) e di AUD-033 in ADR-0036 (nelle Consequences): un richiamo si legge dove si legge la frase che corregge, in qualunque sezione viva. ⚠️ **Questa voce diceva *«dentro la Context»* di ADR-0005, ed era falsa** — corretta lo stesso giorno dalla ri-revisione dell'ondata 1 del compito 1: un difetto del **coordinatore**, non del piano, della stessa specie di E3. Trovato il 2026-09-03 dettando il richiamo di E3. Per i tre richiami del compito 3, che riguardano l'ADR **intero** e non una frase — «voce e gesti», il tracciamento fra gli esempi, «e la telecamera» — la posizione in testa resta quella scelta dal piano (D15) e si rilegge al pre-controllo del compito 3; per una frase falsa la posizione è accanto alla frase |
| **E5** | **Compito 2, Passo 2 — nella sezione «Costo dichiarato» dell'ADR-0039 la frase *«rende solo file di `tests/`»* è falsa di un file.** Trovata dal pre-controllo del compito 2, il 2026-09-03, rilanciando il comando: `grep -rln 'impl Process for\|impl Worker for' crates/` rende quattro banchi **e** `crates/kernel/src/ports/process.rs`, dove la frase sta in un **commento** (riga 68) che nomina questo stesso comando — il disegno (§2.4) lo diceva, il piano l'ha tolto: stessa specie di E3, colta prima del commit. La conclusione regge. **Si detta corretta, e appuntata al commit di prima del piano perché non invecchi:** al posto di *«nessuna implementazione di `Process` o `Worker` esiste fuori dai banchi — `grep -rln 'impl Process for\|impl Worker for' crates/` rende solo file di `tests/`;»* l'ADR dice *«nessuna implementazione di `Process` o `Worker` esiste fuori dai banchi — `git grep -l -e 'impl Process for' -e 'impl Worker for' 4d16f33 -- crates/` rende quattro banchi e un solo file di `src/`, `crates/kernel/src/ports/process.rs`, dove la frase sta in un **commento** che nomina questo stesso comando;»*. Il resto del paragrafo non cambia. Le altre affermazioni misurabili del testo dettato sono state rilanciate e reggono: `f32`/`f64` assenti dal kernel, `FromWorker` a due varianti e nessun messaggio in giù, il reattore che conosce solo il tempo, nessun lettore di produzione di una porta, la regola di `Degradation`, `GrantRequest` senza `name` |
| **E6** | **Compito 4, Passo 2 — il Trova *«\| **Generazione asset prima di Voce** \|»* non esiste: quella riga di «Perché quest'ordine» è senza grassetto.** Trovato dal pre-controllo del compito 4, il 2026-09-03: `grep -n 'Generazione asset prima di Voce' docs/roadmap.md` rende la riga *«\| Generazione asset prima di Voce \| chiude **SP-1** (il rischio più grande) prima; …»*. Il Trova è quella riga intera, presa dal file; la riga nuova resta come dettata, col grassetto, che è la forma della prima e dell'ultima riga di quella tabella. Riletta la tabella per intero: nessun'altra riga parla della Voce, quindi nessun richiamo datato |
| **E7** | **Compito 4, Passi 1 e 5 — il `grep` sulle righe `\| N \|` pesca anche altre due tabelle di `roadmap.md`, e il `tail` non mostra la tabella dei sotto-progetti.** Trovato dal pre-controllo del compito 4, il 2026-09-03: `grep -n -E '^\| \*{0,2}[0-9a-z]{1,3}\*{0,2} \|' docs/roadmap.md \| tail -4` rende le righe della tabella dei consumatori di ADR-0033 (`\| 1 \|`, `\| 2 \|`, `\| 3 \|`), e prima ancora quelle della tabella dei piani; la tabella dei sotto-progetti non è in coda al file. La sonda si delimita per sezione: `awk '/^## Sotto-progetti/{s=1;next} s&&/^## /{s=0} s&&/^[\|] /' docs/roadmap.md \| tail -3` — oggi `9`, `10`, `11`; dopo il compito `10`, `11`, `12`. Domanda **1**: la sonda attaccava il file, non la tabella |
| **E8** | **Compito 6, Passo 2 — il Trova *«l'ultima riga del file»* è la sola `}`, che nel banco compare venticinque volte: l'aiutante rifiuta.** Trovato dal pre-controllo del compito 6, il 2026-09-03, sul repository a `0f98839`: `last=$(tail -1 crates/kernel/tests/arbiter_admission.rs \| tr -d '\r'); grep -cxF -- "$last" crates/kernel/tests/arbiter_admission.rs` rende **25**. Il Trova si allarga alle **due** righe finali — `    assert_eq!(released, Ok(Released::Now(Mib::new(4_096))));` e `}` — la cui prima è unica nel file (`grep -cF` → 1); il Sostituisci è le stesse due righe seguite dal testo dettato, che comincia con la riga vuota. Il testo delle sonde non cambia. Domanda **1**: la sonda attaccava la riga invece del blocco unico |
| **E9** | **Compito 6, Passo 4, mutazione M-b — la riga `if self.allocated().saturating_add(asked) > ceiling {` compare DUE volte in `crates/kernel/src/arbiter/mod.rs`, e l'aiutante rifiuta.** Stesso pre-controllo: `grep -n 'saturating_add(asked) > ceiling' crates/kernel/src/arbiter/mod.rs` rende la riga di `admit` e una di `promote`, che porta la stessa riga con più rientro — e la forma con otto spazi è **contenuta** in quella con sedici, quindi anche la riga intera conta due. Il Trova si allarga alla riga di `admit` **più la riga seguente**, il commento `// ⛔ THE ONE PLACE THE TWO POLICIES DIFFER. ADR-0006 says exactly this is where`, unico nel file; la mutazione sostituisce `asked` con `Mib::ZERO` nella sola prima riga. La riga di `promote` **non si muta**: non è ciò che S3 tiene. Domanda **1**: la sonda attaccava una riga che il file porta due volte |
| **E10** | **Compito 6, Passo 2 — le due righe `assert_eq!(arbiter.allocated(), TOTAL, …);` dettate dal piano rendono `cargo fmt --all --check` ROSSO.** Trovata **eseguendo** il compito 6, il 2026-09-04, e non dal pre-controllo: il repository non ha `rustfmt.toml`, quindi vale il `fn_call_width` predefinito di **60**, e le due chiamate lo superano — `cargo fmt --all --check` chiede la forma spezzata su cinque righe. ⚠️ **Non è una preferenza di stile: è la forma che il banco già usa**, e quante volte lo dice il comando — `grep -c 'assert_eq!($' crates/kernel/tests/arbiter_admission.rs`. **Si detta corretta:** al posto di `    assert_eq!(arbiter.allocated(), TOTAL, "a zero reservation takes nothing");` la prima sonda porta le cinque righe `    assert_eq!(`, `        arbiter.allocated(),`, `        TOTAL,`, `        "a zero reservation takes nothing"`, `    );`; la seconda porta la stessa forma con `        "a queued request reserves nothing"` al posto del messaggio. Nient'altro cambia nelle due sonde, e il significato non cambia affatto. Domanda **2** — *per ogni artefatto che il compito produce, quale controllo lo esercita?*: la copia usa-e-getta `zz_probe_s3.rs` del pre-controllo provava che le sonde **passano**, mai che siano **formattate**, e `cargo fmt` non è un passo del cancello (§7.4.3): nessuno lo faceva rispettare sul testo dettato |

---

## Il pre-controllo del piano — che cosa il disegno dice e il repository smentisce

Letto scrivendo, contro il repository a `1c0a633`, il 2026-09-03. Ogni voce è una delle quattro
domande di `CLAUDE.md`, o una delle tre righe che l'elenco non coglie.

### P-1 — Il disegno manda questo file nella tabella «Dove va cosa» di `README.md`, e i disegni vivono nella tabella «Specifiche»

La §1.4 e la condizione 8 della §5.5 dicono *«nella tabella «Dove va cosa» di `README.md`, come i
disegni del Traguardo 6 e della chiusura»*. Misurato: `grep -n 'Traguardo 6 — il disegno\|La chiusura — il disegno' docs/README.md`
rende due righe della tabella **«Specifiche»**, e nessuna in «Dove va cosa». Il precedente vince
sul nome: il compito 9 scrive la riga nella tabella **«Specifiche»**, come le altre due. Domanda
**5** — il contratto cresce sotto il piano, e si legge contro il repository di adesso.

### P-2 — `roadmap.md` e `tracciabilita.md` sono CRLF nell'albero di lavoro oggi, e il piano della chiusura li chiamava LF

Misurato il 2026-09-03 con `git ls-files --eol docs/roadmap.md docs/tracciabilita.md`: `i/lf w/crlf`
entrambi, e `tr -cd '\r' < docs/roadmap.md | wc -c` rende quanto `wc -l`. Il
[piano della chiusura](2026-09-02-sottoprogetto-1-chiusura.md) li dava **LF** nel proprio
vincolo 6: era vero **su quella macchina quel giorno**, e `core.autocrlf` vale `true` qui — la
terza forma della trappola, rimaterializzazione attraverso `smudge`. ⛔ **Nessuna cifra vive in
questo piano per loro:** la mappa dei file dice CRLF **oggi**, e ogni compito rimisura prima di
scrivere. La regola non cambia: si conserva ciò che il file ha, e `git ls-files --eol` resta
invariato — che è la sola cosa che il diff difende.

### P-3 — Il comando del disegno per le case dei totali degli ADR restituisce anche gli esempi nei code span

La §1.4 del disegno dà `grep -n -o -E '[0-9]+ ADR( in stato [A-Za-z]+)?|[0-9]+ decisioni architetturali' …`;
rilanciato rende anche `2 ADR` e `4 ADR` in `HANDOFF.md` e nel compendio, che sono **esempi
dentro code span** — e `scripts/check-docs.sh` li **toglie** prima di contare (`sed 's/`[^`]*`//g'`).
Il compito 1 usa la forma del guardiano, che è l'unica che conta ciò che il guardiano conta. Le
case vere, misurate il 2026-09-03, sono **dieci** in sei file: `docs/HANDOFF.md` (quattro, fra
cui una *«in stato»* e una *«decisioni architetturali»*), `docs/roadmap.md`, `docs/COMPENDIO.md`
(tre, fra cui una *«in stato»*), `docs/AVVIO-CHAT.md`, `CLAUDE.md`. Domanda **1** — la sonda
dettata attaccava anche ciò che non è il meccanismo.

### P-4 — `mediapipe` 1.0.1 vuole Python 3.9–3.12, e `python` su questa macchina è 3.13

F1 del disegno: Python 3.9–3.12. Misurato il 2026-09-03: `python --version` → `Python 3.13.7`;
`py -0` elenca **3.14**, 3.13 e **3.10**. Lo spike gira quindi su **`py -3.10`**, ed è scritto
così in ogni comando del compito 7 e 8; né `mediapipe` né `cv2` sono installati fuori da una
`venv` (`python -c "import mediapipe"` → `ModuleNotFoundError`). Se la ruota 1.0.1 accettasse
anche il 3.13, si registra e non si cambia: 3.10 è dentro l'intervallo dichiarato dalla fonte.
Domanda **5** applicata alla **macchina**, non al codice.

### P-5 — S3 è stata misurata scrivendo il piano, nelle due direzioni, e il compito 6 la scrive per sempre

Domanda **3** — l'artefatto sbagliato che compila si coglie solo scrivendone un'implementazione
da fuori la crate. Fatto il 2026-09-03 con una sonda usa-e-getta in `crates/kernel/tests/`,
compilata con `cargo test --locked -p kernel --test zz_probe_s3`, **passata** in entrambe le
direzioni — `Mib::ZERO` `Granted` su una macchina piena esatta, un MiB `Queued` sulla stessa —
e cancellata nella stessa corsa: `git status --porcelain` vuoto dopo. Letto anche il perché nel
sorgente: `admit` fa due domande — *«più grande della macchina?»* e *«entra adesso?»* — e zero
risponde no a entrambe; una terza domanda, *«zero è una richiesta?»*, **non esiste**. Il compito
6 riscrive la sonda nella forma del banco e la tiene con **due mutazioni**, perché una sonda che
passa al primo colpo prova qualcosa solo se un mutante la fa cadere (gotcha #24).

### P-6 — «dodici sotto-progetti» vive in due documenti, e la riga 12 lo rende falso

`grep -rn 'dodici sotto-progetti' docs/*.md CLAUDE.md` rende **due** case vive: la §12 del
compendio e la mappa dei documenti di `HANDOFF.md` (la terza è dentro il gotcha #70, un verbale).
La tabella ha già **quattordici** righe — `0`, `0b`, `0c`, `1`…`11` — e la 12 ne aggiunge una.
⛔ **Il numerale si toglie, non si riallinea** (gotcha #68, precedenti AUD-007 e AUD-046): il
compito 4 lo fa nello stesso commit della riga 12. Domanda **1** — la propagazione mancava.

### P-7 — L'API Python di MediaPipe è verificata alla fonte il 2026-09-03, e il codice dello spike la segue

Letta la pagina F4 per Python (`…/hand_landmarker/python`): `HandLandmarkerOptions(base_options=BaseOptions(model_asset_path=…), running_mode=VisionRunningMode.LIVE_STREAM, num_hands=…, result_callback=…)`,
`detect_async(mp_image, timestamp_ms)` con il timestamp in **millisecondi**, la callback che riceve
`(result, output_image, timestamp_ms)`, `result.hand_landmarks` con `x`, `y`, `z` per punto, e
`mp.Image(image_format=mp.ImageFormat.SRGB, data=<numpy>)`. La pagina dei modelli dà l'URL del
modello `float16`: `https://storage.googleapis.com/mediapipe-models/hand_landmarker/hand_landmarker/float16/latest/hand_landmarker.task`.
⚠️ **Le pagine descrivono l'API delle Tasks e non nominano la versione**: se la 1.0.1 avesse
cambiato un nome, il primo `py -3.10 s1_bench.py` lo dice, e la voce d'errata lo registra — non
si «aggiusta» in silenzio. Domanda **1**, sullo stato dell'arte.

### P-8 — `spikes/PROTOCOLLO.md` è congelato per SP-5 e SP-6, quindi SP-7 ha un protocollo suo

La sezione *Congelamento* di [`spikes/PROTOCOLLO.md`](../../../spikes/PROTOCOLLO.md) dice che
il file è congelato al primo commit di codice di spike. Appendervi i criteri di SP-7 riaprirebbe
un metro chiuso; e SP-1…SP-4 hanno i propri in §9 della spec del kernel. Il protocollo di SP-7 è
un file nuovo, `spikes/gesti/PROTOCOLLO.md` (D5), commesso **prima** del codice (D6): così
*«criteri scritti prima della misura»* è provato da `git log`, non dichiarato. Domanda **2** —
per l'artefatto «criteri prima» serviva un controllo, ed è l'ordine dei commit.

### P-9 — Il margine del compendio è misurato, e i compiti lo consumano cinque volte

Il 2026-09-03 il comando del vincolo 12 rende `16581` byte. I compiti 1, 2, 3, 4 e 9 aggiungono
al compendio; le due voci di §5 sono la parte più grossa. ⛔ **Ogni compito misura prima e dopo**,
e se il cancello andasse rosso per il tetto la risposta è togliere prosa dalla §6 — non alzare
il tetto, non accorciare una voce di §5 sotto ciò che il disegno approva.

### P-10 — Le voci aperte del repository, rilette per il chiusore: nessuna ha questo piano

Come la riga di `CLAUDE.md` su `superpowers:writing-plans` prescrive, lette **prima** di scrivere:
la tabella *«Le voci aperte del Traguardo 5, in una tabella sola»* e quella del Traguardo 6 di
[`porta-di-qualita.md`](../../porta-di-qualita.md), le voci senza numero AUD
dell'[audit](../../audit-2026-08-27.md), la tabella delle voci aperte della §6 del compendio, la
§7.8 del disegno della chiusura, e le tredici decisioni del disegno. **Nessuna ha come chiusore
questo piano, né «il proprietario, prima»**: l'elenco di ciò che si sa e si dichiara sta nella
sezione *«Le voci aperte che questo piano SA, e non chiude»*. Tre voci del Traguardo 6 toccano il
**primo worker vero**, che è il sotto-progetto 12 e non questo piano.

### P-11 — La trappola 7 del disegno è verificata: rinominare la sezione 6 di `tracciabilita.md` non rompe nessun comando

`grep -n 'Voce' scripts/check-docs.sh` non rende niente; nessun documento collega la sezione con
un'ancora (`grep -rn 'tracciabilita.md#' docs/` non rende niente); il controllo dei duplicati legge
il **numero** `6`, che non cambia. Il titolo diventa «Voce e gesti» al compito 4 senza altro.

### P-12 — «Comandi rapidi e slash-command» sta nella sezione 2, non nella 8

La §5.2 del disegno dice *«accanto a «Comandi rapidi e slash-command», sede GUI»*. Misurato:
`grep -n 'Comandi rapidi' docs/tracciabilita.md` rende la riga della sezione **2, Conversazione**.
La riga del registro va lì. Domanda **1** — il posto era dedotto, ora è letto.

---

## Le decisioni prese da questo piano

⛔ **Sono decisioni del piano, non del disegno, e chi esegue può ribaltarle** portando la misura
che le smentisce — è ciò per cui esiste l'errata.

| | Decisione | Perché |
|---|---|---|
| **D1** | i numeri sono **0038** per l'ADR A e **0039** per l'ADR B — `ls docs/adr \| tail -1` rende `0037-…` il 2026-09-03 — coi file `docs/adr/0038-registro-delle-funzioni-del-programma.md` e `docs/adr/0039-telecamera-come-sorgente-di-percezione.md`. Se all'esecuzione l'ultimo ADR non fosse più lo 0037, si rinumera e l'errata lo registra | disegno §1.4: *«i numeri si danno quando si scrivono»*. Fissati qui perché i compiti 4, 5, 6 e 9 li citano: sono l'**interfaccia** fra compiti |
| **D2** | i due ADR sono **due compiti**, e ciascuno muove i totali degli ADR di uno nel proprio commit | un revisore può respingere l'uno approvando l'altro; il guardiano dei totali pretende il commit atomico per ciascuno (vincolo 11) |
| **D3** | i tre richiami datati sono un compito **a sé**, e con essi la frase dei quattro pilastri nella §1 del compendio e in `CLAUDE.md` legge «voce e gesti». La stessa frase in `README.md` (riga 4) **non si tocca**: porta un richiamo proprio, non dice il falso — il pilastro **è** la voce, coi gesti dentro — e la casa del rimando è la testa di ADR-0001 | il compendio e `CLAUDE.md` sono le due letture obbligatorie: ciò che non vi compare, per chi legge non esiste (§13 del compendio) |
| **D4** | l'etichetta di strato della riga 12 è **«L2 + L1 est.»**, sulla forma delle righe 9 (*«L1 est.»*) e 11 (*«L0 + L3»*) | voce 4 del disegno, col consiglio scritto: *«se il proprietario non dice altro, il piano la scrive così»*. Ribaltabile dall'errata |
| **D5** | il protocollo di SP-7 è `spikes/gesti/PROTOCOLLO.md`; il codice in `spikes/gesti/`; l'esito in `spikes/RISULTATI.md` come sezione **SP-7** con le quattro sottosezioni di SP-5/SP-6; i dati grezzi fuori dal repository | P-8; disegno §4.2 |
| **D6** | il compito 7 consegna **due** commit — il protocollo, poi il codice | la condizione 6 della Definizione di «fatto» vuole i criteri **prima** della misura, e due commit lo rendono leggibile in `git log` |
| **D7** | il relay di S2 è **std-only**, spawna il worker Python come processo figlio e ne legge lo stdout, e serve la pagina con Server-Sent Events su `127.0.0.1` | è la topologia core ↔ worker su una pipe; nessuna dipendenza da valutare per un usa-e-getta; la pagina misura col proprio orologio |
| **D8** | la tabella «Spike aperti» di `roadmap.md` guadagna **SP-7** al compito 4, `⬜`, e il compito 8 la chiude | SP-5 e SP-6 vi stanno: uno spike fuori da quella tabella sarebbe un elenco incompleto in un documento di stato |
| **D9** | la riga di questo piano nella tabella dei piani di `roadmap.md` è scritta dalla sessione che scrive il piano — *«scritto il 2026-09-03»* — e il compito 9 la porta a *«eseguito»* | precedente del Traguardo 5 |
| **D10** | il compito 9 aggiunge al compendio **due** righe di §12 — il disegno e questo piano — e la riga SP-7 nella tabella «Cosa NON rifare» della §8, e riscrive il puntatore della §6 al **brainstorming della knowledge base** | disegno §5.5 condizione 8, e l'ordine della §5.5 punto 5 |
| **D11** | **nessun diagramma di `design/` cambia**: il sistema non cambia con questo piano — il worker non esiste — e la regola 2 di `README.md` vuole i diagrammi aggiornati nel task che cambia il sistema. Il nodo del worker telecamera entra in `design/01` col sotto-progetto 12 | `README.md`, regole della documentazione |
| **D12** | ogni compito **rimisura** la baseline — cancello, `check-docs.sh`, margine — e non cita quella scritta qui | gotcha #31 |
| **D13** | in ADR-0038 l'ADR-0039 è **nominato senza link**, perché al commit del compito 1 il file non esiste (vincolo 13); ADR-0039 linka ADR-0038 | il controllo dei link legge `docs/adr/` |
| **D14** | il registro `porta-di-qualita.md` guadagna una sezione in coda per la sonda S3 — è dove il suo comando d'indice cerca *«riga di catalogo … non hanno»* — e non si tocca altrove | la tabella del Traguardo 5 di quel file spiega il comando d'indice |
| **D15** | il richiamo alla telecamera in ADR-0023 e quello al tracciamento in ADR-0011 sono scritti nella forma del rimando AUD-032: **nessuna riga superata**, il fatto nuovo e chi lo decide | vincolo 10 |

**La baseline di partenza, misurata il 2026-09-03 su `1c0a633` e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `bash scripts/check-docs.sh` → `OK — no inconsistencies.` ·
il comando del vincolo 12 → `16581` · `git status -sb` → `## main...origin/main`, pulito.

---

## La mappa dei file

| File | Chi lo tocca | Fine-riga il 2026-09-03 | Responsabilità |
|---|---|---|---|
| `docs/adr/0038-registro-delle-funzioni-del-programma.md` | compito 1, **creato** | LF | l'ADR A |
| `docs/adr/0039-telecamera-come-sorgente-di-percezione.md` | compito 2, **creato** | LF | l'ADR B |
| `docs/adr/0001-architettura-a-kernel-con-capacita-paritarie.md` | compito 3 | LF | il richiamo in testa |
| `docs/adr/0011-routing-risolto-e-giornalato-per-richiesta.md` | compito 3 | LF | il richiamo in testa |
| `docs/adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md` | compito 3 | **CRLF** | il richiamo in testa |
| `docs/COMPENDIO.md` | compiti 1, 2, 3, 4, 9 | **CRLF** | §5: due voci e tre rimandi; §1 e §12; i totali; §6, §8 e l'intestazione al compito 9 |
| `docs/README.md` | compiti 1, 2, 9 | **CRLF** | due righe nell'indice degli ADR; la riga del disegno nella tabella «Specifiche» |
| `docs/HANDOFF.md` | compiti 1, 2, 4 | **CRLF** | i totali degli ADR; la cella della roadmap nella mappa dei documenti |
| `docs/AVVIO-CHAT.md` | compiti 1, 2 | **CRLF** | il totale degli ADR nel messaggio |
| `CLAUDE.md` | compiti 1, 2, 3 | **CRLF** | il totale degli ADR; la frase dei quattro pilastri |
| `docs/roadmap.md` | compiti 1, 2, 4, 8, 9 | **CRLF** (P-2) | il totale; la riga 12, la riga 8, «Perché quest'ordine», la tabella degli spike, l'intestazione; la riga di questo piano |
| `docs/tracciabilita.md` | compito 4 | **CRLF** (P-2) | il titolo della sezione 6, le righe dei gesti, la riga del registro, il riquadro in testa |
| `docs/riferimenti.md` | compito 5 | **CRLF** | la sezione F1–F9 e tre righe in «Cosa NON abbiamo adottato» |
| `crates/kernel/tests/arbiter_admission.rs` | compito 6 | CRLF nell'albero, LF nell'indice | le due sonde S3 e un capoverso del doc di modulo. ⛔ L'unico file di `crates/` |
| `crates/kernel/src/arbiter/mod.rs` | compito 6, **mutato e ripristinato byte-esatto** | CRLF nell'albero, LF nell'indice | le due mutazioni. ⛔ **Nessun commit lo tocca** |
| `docs/porta-di-qualita.md` | compito 6 | **CRLF** | la sezione della sonda S3, in coda (D14) |
| `spikes/gesti/PROTOCOLLO.md` · `requirements.txt` · `requirements.lock` · `s1_bench.py` · `s2_worker.py` · `relay/Cargo.toml` · `relay/src/main.rs` · `relay/page.html` | compiti 7 e 8, **creati** | LF | lo spike SP-7 |
| `.gitignore` | compito 7 | si misura | la `venv`, il `target/` e il `Cargo.lock` del relay, il modello, i CSV |
| `spikes/RISULTATI.md` | compito 8 | **CRLF** | la sezione SP-7 e la riga della data |
| `docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` | compiti 8, 9 | LF | i richiami in §6.4 con l'esito di SP-7; la spunta della §5.5 |
| `docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md` | ogni compito | LF | la tabella della posizione, e l'errata |

---

## Le voci aperte che questo piano SA, e non chiude

⛔ **Lette prima di scrivere, come `CLAUDE.md` prescrive; nessuna ha come chiusore questo piano.**
Si dichiarano perché chi esegue le sappia, non perché le tocchi.

| Voce | Dove vive | Chi la chiude |
|---|---|---|
| decisioni **2**, **7**, **9**, **10**, **12**, **13** del disegno — il vocabolario dei gesti, la destinazione della cattura, la terza quota, l'interruttore, la posizione dei pannelli, il confinamento del worker | la tabella delle tredici decisioni del disegno | la capacità (12), il brainstorming della knowledge base, un tracciatore su GPU, l'archivio dei parametri, il sotto-progetto 12 col proprietario |
| voce **4** del disegno — l'etichetta di strato | *«Le voci che questo disegno apre»* | **questo piano la prende** col consiglio scritto, D4; il proprietario la ribalta nell'errata |
| voce **6** del disegno — la sonda S3 senza riga di catalogo | idem | il proprietario: §7.4 è spec. Il compito 6 la **registra** nel registro |
| le voci **9**, **26** e **27** del Traguardo 6 — la suite di conformità di §6.10 che nasce col canale worker vero; la metà temporale di «concessione valida»; `compute_class` e `preemption` obbediti senza controllo | la tabella *«Le voci aperte del Traguardo 6»* del registro | il **primo worker vero**, cioè il sotto-progetto 12 — non questo piano, che non lo costruisce |
| **X-1**…**X-4** dell'audit — la CI solo Linux, la riga falsa di `gate.sh`, `cargo audit`, `E64` | la tabella delle voci senza numero AUD | il proprietario |
| la tabella delle voci aperte della §6 del compendio — ritenzione, `replay()`, il registro non sorvegliato, la guardia del puntatore, l'elenco dei semi, i 25,8 µs, il portachiavi | §6 del compendio | i traguardi e il proprietario che quella tabella nomina; nessuna tocca i gesti |

---
## Compito 1: ADR-0038 — il registro delle funzioni del programma

**Files:**
- Create: `docs/adr/0038-registro-delle-funzioni-del-programma.md` (LF)
- Modify, **nello stesso commit** (vincolo 11): `docs/COMPENDIO.md` (CRLF) — la voce in §5, il titolo della §5, la riga 14, i totali, l'intestazione · `docs/README.md` (CRLF) — l'indice degli ADR · `docs/HANDOFF.md` (CRLF), `docs/roadmap.md` (CRLF), `docs/AVVIO-CHAT.md` (CRLF), `CLAUDE.md` (CRLF) — i totali
- Read: §1.1, §3.1, §3.4 del disegno; [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) per la forma di un ADR di questo repository

**Interfaces:**
- Produces: il file `docs/adr/0038-registro-delle-funzioni-del-programma.md`, `Accepted`, che i compiti 2, 4, 6 e 9 citano con questo nome; la voce `**0038 — …**` in §5

- [ ] **Passo 1: le misure prima**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
ls docs/adr | tail -1
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
git ls-files --eol docs/COMPENDIO.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md CLAUDE.md
for f in docs/COMPENDIO.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md CLAUDE.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
```

Atteso: `OK`, `GATE GREEN`, `0037-…` (D1 — altrimenti ci si ferma e si scrive l'errata), margine
positivo, sei file `i/lf w/crlf` con CR = righe.

- [ ] **Passo 2: l'ADR, LF, nuovo**

Il contenuto è quello della §3.1 del disegno — riga **A** — riscritto nella forma di ADR-0037:
`Context` con ciò che gli ADR già dicono, `Alternative considerate`, `Decision` con le regole in
tabella, `Perimetro negativo`, `Consequences` con **Positive**, **Negative (accettate)** — che il
piano **non accorcia** rispetto al disegno — e **Follow-up**. Le due colonne di destra della
tabella della §3.1 entrano come sono. Si scrive con `<data>` = `date +%F`.

```markdown
# ADR-0038: Il registro delle funzioni del programma — un registro, molti invocatori, lo stesso permesso

- **Status:** Accepted
- **Date:** <data>
- **Deciders:** proprietario del progetto

## Context

Le funzioni del programma — aprire un pannello, accendere la telecamera, catturare, cambiare
profilo, lanciare un comando rapido — esistono finora come intenzione: *«Comandi rapidi e
slash-command»* in [`tracciabilita.md`](../tracciabilita.md), sede GUI, e nient'altro. Il
brainstorming del riconoscimento gesti
([disegno del 2026-09-03](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md),
sezioni 1 e 3) ha posto la domanda che nessun ADR rispondeva: **chi può invocare una funzione
del programma, e con quale permesso?** Gli invocatori sono già quattro sulla carta — il click e
la tastiera della GUI, il gesto, la voce, e l'agente, perché il proprietario ha chiesto il
*«self-use dell'agente sulle funzioni del programma»* (terza domanda d'apertura del disegno).

Ciò che gli ADR già dicono, letto il 2026-09-03:

| ADR | Che cosa dà | Che cosa non dice |
|---|---|---|
| [ADR-0025](0025-confinamento-a-livelli.md) | il livello 1 *«resta ammesso solo per strumenti interni che non eseguono codice»* | dove quegli strumenti siano registrati, e chi possa invocarli |
| [ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) | due registri di kernel — guide e sensori — col principio *il kernel dà il meccanismo, le capacità portano il contenuto* | un registro delle **funzioni** |
| [ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md) | il permesso come tripla `(strumento × risorsa × operazione)`, e un'approvazione che **non si estende** | che la tripla valga anche per una funzione invocata a gesti o a voce |
| [ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md) | il contenuto non fidato **informa, mai autorizza** | che un evento di percezione sia di quella specie |
| [ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md) | la classe `irripetibile`, che sospende e chiede | chi possa confermare |

**Verificato nel sorgente il 2026-09-03:** non esiste un registro delle funzioni del programma.
`grep -n -i -E 'strument[oi] intern|registro degli strumenti|palette|scorciatoi' docs/superpowers/specs/2026-08-06-kernel-design.md docs/adr/*.md`
rende la sola riga di ADR-0025.

### Alternative considerate

- **Una logica «solo per gesti»** — un vocabolario di gesti che invoca direttamente le proprie
  azioni, dentro la capacità dei gesti.
  *Pro:* nessun meccanismo nuovo nel kernel.
  *Contro:* la stessa funzione avrebbe due strade con due permessi; la strada da tastiera si
  scriverebbe dopo, con un'altra regola. È la scorciatoia verso il kernel che
  [ADR-0001](0001-architettura-a-kernel-con-capacita-paritarie.md) vieta, e produce esattamente il
  **dialetto** che una capacità non può permettersi: una funzione che si comporta diversamente a
  seconda di chi la chiama.
- **Il registro dentro la GUI** — la GUI conosce i propri comandi, e li espone.
  *Pro:* il primo invocatore è la GUI, e sembra il posto naturale.
  *Contro:* la GUI è **sacrificabile** ([ADR-0004](0004-topologia-di-processo.md)): un registro che
  vive in essa muore con essa, e l'agente — che gira nel core, con la GUI chiusa — non lo
  raggiunge. E lo stato autorevole vive solo nel core (I1).
- **Un registro unico, meccanismo di kernel** — la scelta.
  *Pro:* una funzione si scrive una volta e si invoca da ogni strada con lo stesso permesso; il
  permesso e il giornale sono quelli che il kernel ha già.
  *Contro:* un meccanismo di kernel in più, prima di qualunque capacità; e ogni funzione con
  effetto va dichiarata come tripla.

## Decision

> **Il kernel espone un registro unico delle funzioni del programma — registrazione,
> invocazione, il permesso come tripla di ADR-0016, il giornale — e le capacità e la GUI portano
> le funzioni. Molti invocatori, lo stesso permesso: agente, gesto, voce, click. Nessuna logica
> «solo per gesti».**

| # | Regola |
|---|---|
| 1 | il registro è un **meccanismo di kernel** nella forma di ADR-0009: il kernel dà registrazione, invocazione, permesso e giornale; il **contenuto** — quali funzioni esistano — lo portano le capacità e la GUI, come le guide e i sensori |
| 2 | ogni funzione con effetto è dichiarata come **tripla** di ADR-0016, e l'invocazione passa dallo stesso permesso **qualunque sia l'invocatore**: un'azione invocata a gesti chiede ciò che chiederebbe da tastiera |
| 3 | un gesto, una trascrizione o un'istruzione del modello sono **eventi**: **informano, mai autorizzano** — ADR-0014 per analogia. Un evento non concede permessi |
| 4 | un effetto **irripetibile** (ADR-0007) chiede conferma a qualunque invocatore — è già così in ADR-0016 — e **per default la conferma non è gestuale**: un gesto letto male non deve poter confermare sé stesso (decisione 6 del disegno) |
| 5 | la **manipolazione** della GUI — spostare un pannello, aprire un menu virtuale — è stato di presentazione e **non passa dal registro** (ADR-0004: la GUI possiede solo presentazione) |
| 6 | **quali** funzioni siano gestuali, o vocali, lo decide la capacità che porta l'invocatore — per i gesti il sotto-progetto 12 — non questo ADR |
| 7 | un gesto **non è mai l'unica strada**: ogni funzione gestuale si raggiunge anche da tastiera e click (requisito G20 di [`spikes/GUI-REQUISITI.md`](../../spikes/GUI-REQUISITI.md)). Segue dalla regola 1: un registro con molti invocatori non ha funzioni di un invocatore solo |

### Perimetro negativo — cosa questa decisione **non** è

| Non è | |
|---|---|
| il **vocabolario** dei gesti, né l'elenco delle funzioni | contenuto, della capacità |
| un meccanismo **costruito** con questo ADR | nessun codice nasce qui: il registro lo costruisce chi porta il **primo invocatore**, il click del sotto-progetto 2 (§5.1 del disegno); il gesto lo aggiunge il 12, la voce l'8 |
| un **secondo** sistema di permessi | è la tripla di ADR-0016, con le sue tre modalità di supervisione |
| **estensibilità di terzi** | [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md) non cambia: le funzioni le portano le capacità e la GUI, non un plugin |
| le finestre dell'**OS** | fuori perimetro — sotto-progetto 10, con un ADR suo |

## Consequences

- **Positive:**
  - **Una funzione si scrive una volta** e si invoca da quattro strade con lo stesso permesso: la
    superficie di permesso non cresce con gli invocatori.
  - **L'agente usa il programma senza scorciatoie**, che è la parità di ADR-0001 resa concreta su
    un caso: passa dalla stessa porta dell'utente, con la stessa tripla.
  - **Un gesto letto male non può fare danno da solo**: informa, e ciò che ha effetto chiede il
    permesso che chiederebbe comunque.

- **Negative (accettate):**
  - **Un meccanismo di kernel in più prima di ogni capacità**: registrazione, invocazione,
    permesso e giornale vanno costruiti dal sotto-progetto 2 prima che una sola funzione esista.
  - **Ogni funzione con effetto va dichiarata come tripla**, anche quelle banali della GUI: il
    costo di dichiarazione pesa su tutte, e non solo su quelle che un gesto può invocare.
  - **Lo stesso permesso pesa anche sulle funzioni banali invocate dalla GUI**: un click che
    apre un pannello passa dalla stessa strada di un comando con effetto, e la strada deve
    restare leggera o la GUI la aggirerà.
  - **È di livello 0 finché non esiste**: niente impone che una funzione nuova passi dal
    registro finché il sotto-progetto 2 non lo costruisce e gli dà una sonda.

- **Follow-up richiesti:**
  - Il **sotto-progetto 2** costruisce il registro col primo invocatore, il click; il **12**
    aggiunge il gesto; l'**8** la voce. La riga è in [`tracciabilita.md`](../tracciabilita.md),
    sezione 2, accanto ai comandi rapidi.
  - ADR-0039 — la telecamera come sorgente di percezione, scritto insieme a questo — usa questa
    decisione: accendere la telecamera è una funzione del registro, e un gesto di comando entra
    dal registro.
```

⛔ **ADR-0039 è nominato senza link** (D13): al commit di questo compito il file non esiste, e il
controllo dei link legge `docs/adr/`.

- [ ] **Passo 3: le case dei totali, misurate col comando del guardiano**

```bash
for f in docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md; do sed 's/`[^`]*`//g' "$f" | grep -n -oE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)' | sed "s|^|$f:|"; done
```

Il 2026-09-03 rende **dieci** righe, e per ciascuna la sostituzione è di **uno**: `37 ADR` → `38 ADR`,
`36 ADR in stato` → `37 ADR in stato`, `37 decisioni architetturali` → `38 decisioni
architetturali`. Le frasi, per ritrovarle col `grep` e non col numero di riga:

| File | Trova | Sostituisci con |
|---|---|---|
| `CLAUDE.md` | `le 37 ADR, le sei` | `le 38 ADR, le sei` |
| `docs/AVVIO-CHAT.md` | `le 37 ADR` | `le 38 ADR` |
| `docs/roadmap.md` | `(§0–§10, 37 ADR)` | `(§0–§10, 38 ADR)` |
| `docs/COMPENDIO.md` | `Sono **37 ADR**, di cui **36 ADR in stato Accepted** e uno` | `Sono **38 ADR**, di cui **37 ADR in stato Accepted** e uno` |
| `docs/COMPENDIO.md` | `è nei 37 ADR, ciascuno con alternative scartate e motivo` | `è nei 38 ADR, ciascuno con alternative scartate e motivo` |
| `docs/HANDOFF.md` | `Spec del kernel **§0–§10 completa, 37 ADR**.` | `Spec del kernel **§0–§10 completa, 38 ADR**.` |
| `docs/HANDOFF.md` | `36 ADR in stato` | `37 ADR in stato` |
| `docs/HANDOFF.md` | `è in **37 ADR**, ciascuno con alternative scartate e motivo` | `è in **38 ADR**, ciascuno con alternative scartate e motivo` |
| `docs/HANDOFF.md` | `**37 decisioni architetturali**. Leggi **0001**` | `**38 decisioni architetturali**. Leggi **0001**` |

⚠️ Se il comando rendesse una riga in più o in meno di dieci, ci si ferma: è l'errata, non un
riallineamento a occhio.

- [ ] **Passo 4: il compendio, CRLF — la voce in §5, il titolo, la riga 14, l'intestazione**

| Trova | Sostituisci con |
|---|---|
| `## 5. Le trentasette decisioni` | `## 5. Le decisioni, una per ADR` — il numero di sezione **non** cambia: il delimitatore `^## 5\. ` del guardiano resta |
| `> accettati — apri **quel** file. Uno, non trentasette. La §12 dice quale.` | `> accettati — apri **quel** file. Uno, non tutti. La §12 dice quale.` |
| la riga che apre con `**0037 — Il criterio del pari.**` fino alla fine di quella voce — cioè fino alla riga `restituito \`251\` al posto di \`4096\` senza sollevare nulla.` compresa, che è l'ultima prima del `---` che precede `## 6.` | resta; **dopo** di essa, una riga vuota e la voce nuova qui sotto |
| l'intestazione: `**Aggiornato il 2026-09-03**, col **SOTTO-PROGETTO 1 CHIUSO** contro la §0.7 della sua spec;` fino a `Manutenzione: §13.` compreso, sulla stessa riga | `**Aggiornato il <data>**, con **ADR-0038** — il registro delle funzioni del programma — in §5, dal piano del riconoscimento gesti; l'ultimo contenuto di **merito** è quella voce. Manutenzione: §13.` — i richiami sotto **restano** |

La voce nuova, parola per parola (i due grassetti di apertura sono la forma di ogni voce di §5;
`**0038 —` a inizio riga è ciò che il guardiano accoppia al file):

```markdown
**0038 — Il registro delle funzioni del programma.** **Un registro unico, molti invocatori,
lo stesso permesso.** Il kernel dà registrazione, invocazione, il permesso come tripla di ADR-0016
e il giornale, nella forma dei registri di ADR-0009; le capacità e la GUI portano le funzioni.
Agente, gesto, voce e click passano dalla **stessa** porta con la **stessa** tripla, e nessuna
logica «solo per gesti» esiste. Un evento di percezione **informa, mai autorizza** (ADR-0014 per
analogia); un effetto irripetibile chiede conferma a qualunque invocatore, e **per default la
conferma non è gestuale**. La manipolazione della GUI — pannelli, menu — è presentazione e
**non passa dal registro**. ⛔ **Nessun codice nasce con l'ADR:** il registro lo costruisce il
primo invocatore, il click del sotto-progetto 2; quali funzioni siano gestuali lo decide il 12.
```

- [ ] **Passo 5: l'indice di `README.md`, CRLF**

| Trova | Sostituisci con |
|---|---|
| la riga `\| [0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) \| Il criterio del pari: il formato di un canale privato si sceglie anche sull'ecosistema di chi lo legge \| Accepted \|` | resta; **sotto** di essa: `\| [0038](adr/0038-registro-delle-funzioni-del-programma.md) \| Il registro delle funzioni del programma: un registro, molti invocatori, lo stesso permesso \| Accepted \|` |

Il guardiano conta le righe `^\| \[00` dell'indice contro i file di `docs/adr/`: senza questa
riga il cancello è rosso.

- [ ] **Passo 6: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
tr -cd '\r' < docs/adr/0038-registro-delle-funzioni-del-programma.md | wc -c
for f in docs/COMPENDIO.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md CLAUDE.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol docs/COMPENDIO.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md CLAUDE.md
grep -c '](\.\./' docs/adr/0038-registro-delle-funzioni-del-programma.md
git status --porcelain
```

Atteso: `OK`, `GATE GREEN`, margine positivo, `0` CR sull'ADR, CR = righe sui sei CRLF,
`ls-files --eol` invariato, e `git status` che nomina i sette file e nient'altro. ⚠️ Il `grep`
sui link `../` dell'ADR rende **due** — `../tracciabilita.md` e `../superpowers/specs/…` — e
`../../spikes/GUI-REQUISITI.md`: il controllo dei link li risolve da `docs/adr/`, e il cancello
verde è la prova.

```bash
git add docs/adr/0038-registro-delle-funzioni-del-programma.md docs/COMPENDIO.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md CLAUDE.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 1): ADR-0038, il registro delle funzioni del programma — un registro, molti invocatori, lo stesso permesso; la voce in §5, la riga nell'indice e i totali nello stesso commit"
git push
```

In questo piano la tabella della posizione passa il compito 1 a ✅ con la data, nel commit.

#### Criterio di chiusura del compito 1

- [ ] l'ADR esiste, `Accepted`, con `Negative (accettate)` non più corte della riga A della §3.1 del disegno
- [ ] `check-docs.sh` → `OK` con la voce in §5, la riga nell'indice e i dieci totali mossi di uno
- [ ] nessun link a un file che non esiste; ADR-0039 nominato senza link
- [ ] `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione del piano aggiornata

---

## Compito 2: ADR-0039 — la telecamera come sorgente di percezione always-on sotto il core

**Files:**
- Create: `docs/adr/0039-telecamera-come-sorgente-di-percezione.md` (LF)
- Modify, **nello stesso commit** (vincolo 11): `docs/COMPENDIO.md` (CRLF) — la voce in §5, i totali, l'intestazione · `docs/README.md` (CRLF) — l'indice · `docs/HANDOFF.md`, `docs/roadmap.md`, `docs/AVVIO-CHAT.md`, `CLAUDE.md` (CRLF) — i totali
- Read: la testa del disegno (le quattro domande, le tredici decisioni), §1.5, §2 per intero, §3.1 riga B, §4.1, §6, §7; [ADR-0038](../../adr/0038-registro-delle-funzioni-del-programma.md) dal compito 1

**Interfaces:**
- Consumes: `docs/adr/0038-registro-delle-funzioni-del-programma.md` (compito 1), che questo ADR linka
- Produces: `docs/adr/0039-telecamera-come-sorgente-di-percezione.md`, `Accepted`, che i compiti 3, 4, 6 e 9 citano

- [ ] **Passo 1: le misure prima** — come il Passo 1 del compito 1; atteso `ls docs/adr | tail -1` → `0038-…`.

- [ ] **Passo 2: l'ADR, LF, nuovo**

Il contenuto è la §2 del disegno per intero — le forme, le tre correzioni, i pezzi, le tre regole,
il costo del primo worker — più la riga **B** della §3.1, la §4.1 e le decisioni 1, 3, 4, 5, 6,
9, 10, 13. Le `Negative (accettate)` non si accorciano.

```markdown
# ADR-0039: La telecamera come sorgente di percezione always-on sotto il core

- **Status:** Accepted
- **Date:** <data>
- **Deciders:** proprietario del progetto

## Context

Il proprietario vuole l'agente **dormiente e risvegliabile con la wake word**, e gesti *«stile
Jarvis»*: menu virtuali, pannelli spostati con le mani, una cattura con un gesto. Il
[disegno del 2026-09-03](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) ha posto
quattro domande d'apertura e il proprietario ha risposto: la telecamera è **una sorgente di
eventi** — non un occhio che mette immagini nel contesto di un modello — con i fotogrammi che
**non escono mai** dal processo che la possiede; i pannelli mossi con le mani sono quelli **del
programma**, non le finestre dell'OS; e la forma è l'**approccio 1**, la sorgente di percezione
**sotto il core**, scelta dopo la sfida *«sicuro che rispetti tutti i principi?»* e tre
correzioni.

Ciò che l'architettura dice già, e che questa decisione onora:

| Il buco | Ciò che decide già |
|---|---|
| «se l'agente dorme, quando succede?» | **il core non dorme mai: dorme la run.** Il core vive a lungo, anche senza GUI ([ADR-0004](0004-topologia-di-processo.md)); la wake word non «sveglia il programma», **apre una run** ([ADR-0011](0011-routing-risolto-e-giornalato-per-richiesta.md), corollario) |
| «chi tiene i fotogrammi?» | un **worker**, in Python, senza stato, uccidibile in ogni istante ([ADR-0028](0028-ecosistema-dei-worker-ml.md), I5); al core arrivano **eventi** |
| «e la GPU?» | il porto `process` **pretende una concessione** per avviare qualunque worker — `Process::start(grant, descriptor)`, §5.6 della spec; le due concessioni permanenti di [ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md) sono il precedente |
| «il kernel decide sui gesti?» | no: un gesto è un **dato opaco** ([ADR-0020](0020-nessun-modello-nel-percorso-decisionale-del-kernel.md)), smistato come la trascrizione che diventa messaggio; il kernel resta testabile senza modello, perché l'evento si inietta a copione ([ADR-0021](0021-simulazione-deterministica-e-iniettabilita.md)) |
| «una foto può dare ordini?» | no: contenuto **non fidato**, informa e non autorizza ([ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md), I6); un gesto non concede permessi ([ADR-0038](0038-registro-delle-funzioni-del-programma.md)) |
| «telecamera assente o spenta» | si **dichiara prima**, non si fallisce dopo ([ADR-0019](0019-lo-stato-di-degrado-e-un-oggetto-osservabile.md)) |
| il profilo «riservato» | spegne la voce always-on ([ADR-0023](0023-cifratura-a-riposo-e-gestore-dei-segreti.md)): spegne **anche** la telecamera |

**Lo stato dell'arte, verificato il 2026-09-03** (le fonti F1–F9 in [`riferimenti.md`](../riferimenti.md)):
MediaPipe è mantenuto — `mediapipe` 1.0.1 su PyPI, rilasciato il 2026-08-14 — e il suo Hand
Landmarker dà **21 punti** per mano, con un modo `LIVE_STREAM` a callback; in Python **la GPU non
c'è su Windows** (F2, e F9 lo conferma), quindi il tracciatore gira su **CPU**. L'alternativa con
GPU su Windows esiste — RTMPose, via ONNX Runtime — e la sua manutenzione si rimisura il giorno
che serve: oggi l'ultimo rilascio è del 2024-07-12.

### Alternative considerate

| | Perché no |
|---|---|
| **il tracciamento dentro la GUI** — MediaPipe in JavaScript sulla webview, l'approccio 2 | muore con la GUI: niente Jarvis a GUI chiusa; contraddice lo slot di ADR-0011 — la sorgente di percezione diventerebbe stato di un processo **sacrificabile** (ADR-0004); WebGL dentro la quota di presentazione; e `detectForVideo` **blocca il thread della UI** (F5). Vince sulla latenza della manipolazione — un salto in meno — e per questo la latenza dell'approccio 1 **si misura** (SP-7, S2) invece di darsi per buona |
| **l'ibrido** — worker per l'always-on, GUI per la manipolazione | due tracciatori su una telecamera, due modelli, due codici: sfoggio |
| **una terza quota permanente** nella formula di [ADR-0005](0005-arbitrato-gpu-su-due-dimensioni.md), oggi | varrebbe **zero**, e il porto pretende già una concessione: la forma minima esiste. Si apre quando esiste un tracciatore su GPU — decisione 9 del disegno, registrata |
| il gesto di comando come **trigger** di [ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) | quel trigger innesca l'**anello di verifica**; il posto di «un evento apre un passo» è il corollario di ADR-0011 |
| il **video** nella webview | i 21 punti bastano a disegnare la mano; il video costerebbe una misura in più per [ADR-0029](0029-guscio-della-gui.md) e una telecamera condivisa fra due processi — decisione 5 |
| un **quinto pilastro** | vorrebbe un ADR che **supera** [ADR-0001](0001-architettura-a-kernel-con-capacita-paritarie.md) sulla parola «quattro», per un contendente della GPU che oggi non la usa — decisione 1 |

## Decision

> **La telecamera è una sorgente di percezione always-on sotto il core: un worker Python la
> possiede, i fotogrammi non escono mai, al core arrivano eventi — lo stato continuo della mano e
> il gesto discreto — che non sono passi. Spenta per default; solo la wake word apre una run;
> «riservato» la spegne.**

| Pezzo | Forma |
|---|---|
| il worker | Python, always-on, sotto il core come sarà il microfono (ADR-0028). **Possiede la telecamera**; i fotogrammi **non escono mai**. MediaPipe Hand Landmarker su **CPU**. Senza stato e **uccidibile in ogni istante** (I5): ciò che ADR-0004 chiama *«vita breve»* si legge, per un worker always-on, come per il worker audio — la ricevuta di stream resta aperta per tutta la vita — e a reggere sono *senza stato* e *uccidibile* |
| il canale | il porto `process` ([ADR-0035](0035-porta-verso-i-worker-e-lettura-di-i4.md)): **una** `instruct_stream` all'accensione — «traccia le mani», la prima istruzione vera della direzione core → worker — poi `read_next` per tutta la vita. `minicbor`, ogni frame dichiara la propria lunghezza ([ADR-0037](0037-criterio-del-pari-per-il-formato-dei-canali.md), §6.10 della spec) |
| le due specie di evento | lo **stato continuo della mano** — 21 punti per mano, coordinate **intere**, a N Hz — e il **gesto discreto** — `kind`: un enum chiuso, `confidence`: un intero. Due varianti nuove di `FromWorker` a indici nuovi, sotto le regole di §6.10. **Eventi, non passi** (ADR-0011): niente giornale per fotogramma |
| il core | li **smista**: la manipolazione va alla GUI con `Ipc::send`, **campionata alla frequenza che il core riceve come parametro consegnato** ([ADR-0034](0034-parametri-di-decisione-consegnati-non-letti.md)); un gesto di **comando** prende la strada della wake word: apre un **passo** nella run aperta, e passa dal registro di ADR-0038 con lo stesso permesso di ogni invocatore |
| la run | **solo la wake word apre una run** (decisione 3); un gesto di comando vale a run aperta. Il gesto di attenzione resta aggiungibile dopo senza rifare niente |
| la concessione | da **zero MiB**, `Preemption::Never`, corsia `Realtime`, chiesta come le due permanenti di ADR-0033 — **all'accensione**, non all'avvio del core; torna con `Killed.grant` ad `Arbiter::release`. **La formula di ADR-0005 non cambia** |
| l'interruttore | **spenta per default** (decisione 4): accenderla è una **funzione del registro** di ADR-0038, quindi con permesso e giornale. Dove l'interruttore si salvi fra un avvio e l'altro è l'archivio dei parametri, che non esiste: decisione 10, registrata |
| «riservato» | spegne anche la telecamera — rimando datato in testa ad ADR-0023 |
| il degrado | `Degradation` guadagna «telecamera assente o spenta» **solo quando il worker esiste** (ADR-0019): un campo sempre `false` si legge come «tutto bene» invece che come «ignoto» |
| la GUI | **disegna la mano dai 21 punti**, in un livello sopra i pannelli; **niente video** (decisione 5): compositing della webview, dentro la quota di presentazione di ADR-0033, nessuna misura in più per ADR-0029. Un **indicatore sempre visibile** quando la telecamera è accesa, acceso dal **core** con un messaggio — la GUI non lo indovina: una falsa sicurezza è peggio di nessuna sicurezza (ADR-0023) |
| il pilastro | «voce» si legge «voce e gesti»; i pilastri restano quattro — rimando datato in testa ad ADR-0001 |

### Regole di forma, verificate nel sorgente il 2026-09-03

| Regola | Perché |
|---|---|
| **niente decimali nel kernel**: sul filo viaggiano interi, la conversione la fa il worker | `grep -rnw f32 crates/kernel/src` e `grep -rnw f64 crates/kernel/src` non rendono niente; MediaPipe dà coordinate normalizzate fra 0 e 1, e il worker le scala |
| **nessun testo dal worker arriva a una decisione**: il gesto è un enum chiuso, non una stringa | il precedente è `GrantRequest` in `crates/kernel/src/wire/ipc.rs`, che attraversa il filo **senza `name`**: un testo scelto dal pari è contenuto non fidato (ADR-0014). Il **vocabolario** dei gesti resta della capacità; il **tipo** si fissa qui |
| **le regole del canale restano quelle di §6.10**: un indice per campo, niente enum di versione, niente byte congelati | testa di `crates/kernel/src/wire/worker.rs` |

### Costo dichiarato: la telecamera sarebbe il PRIMO worker vero, e il primo paga

Verificato il 2026-09-03: nessuna implementazione di `Process` o `Worker` esiste fuori dai
banchi — `grep -rln 'impl Process for\|impl Worker for' crates/` rende solo file di `tests/`; il
canale worker ha una direzione sola, in su, e in giù nessun messaggio; il **timbro di build** che
rifiuta un worker stantio non esiste, e la §6.10.7 della spec lo fa reggere su un ambiente Python
**nostro e versionato** — quindi il lockfile del worker non è cosmesi; il reattore conosce solo
il tempo, e «pronto da leggere» per una pipe non c'è; nessun codice di produzione legge ancora
una porta. Non cambia l'approccio: è il **prezzo**, lo paga chi arriva primo fra voce e gesti — e
per decisione 11 del disegno è il sotto-progetto **12**. La Voce lo **riusa**.

### Perimetro negativo — cosa questa decisione **non** è

| Non è | |
|---|---|
| il **vocabolario** dei gesti, e quali funzioni siano gestuali | la capacità, sotto-progetto 12 — decisione 2 |
| la **destinazione** di una cattura | il brainstorming della knowledge base — decisione 7 |
| il messaggio IPC verso la GUI con la mano campionata | **si definisce quando la GUI esiste** (sotto-progetto 2): prima non ha destinatario, come la revoca dichiarata in testa a `crates/kernel/src/wire/ipc.rs` |
| una terza quota nella formula di ADR-0005 | decisione 9: si apre con un tracciatore su GPU |
| il **confinamento** del processo del worker — se, oltre a separato (ADR-0028), sia **ristretto** al livello 2 di [ADR-0025](0025-confinamento-a-livelli.md) | **nessun ADR lo decide** e nel codice non c'è dove dirlo: `WorkerDescriptor` è byte opachi. Decisione 13 del disegno, registrata: la chiude il sotto-progetto 12 col proprietario, quando avvia il primo worker vero. Consiglio scritto: processo **ristretto**, perché la telecamera è un dispositivo di privacy e ADR-0025 dice che un confinamento più debole non è un ripiego |
| la telecamera come **occhio** dell'agente | un'altra cosa — fotogrammi come contenuto non fidato nel gateway — che si aggiunge dopo come capacità |
| le finestre dell'**OS** | sotto-progetto 10, con un ADR suo |
| `workers/` alla radice | decisione 8: la cartella nasce col primo worker di prodotto, cioè col sotto-progetto 12, non con questo ADR |

## Consequences

- **Positive:**
  - **Jarvis a GUI chiusa**: la sorgente di percezione vive sotto il core, che è l'unico
    processo a vita lunga.
  - **Coerenza piena con ADR-0011 e ADR-0004**: eventi e non passi, e nessuno stato in un
    processo sacrificabile.
  - **Se poi serve l'altro approccio, niente da rifare**: la GUI può aggiungere un tracciatore
    proprio senza toccare il kernel; il contrario non è vero.
  - **Nessuna misura in più per ADR-0029**, e nessuna quota in più nella formula del budget.

- **Negative (accettate):**
  - **Un processo Python su CPU finché la telecamera è accesa**, e la CPU che consuma la paga
    l'utente: ecco perché è opt-in.
  - **Un salto in più sulla manipolazione** — worker → core → GUI — e la latenza **non è nota**:
    la misura **SP-7** (S2), e il giudizio sulla mano che muove un pannello è del proprietario
    che la prova.
  - **Il conto del primo worker** (sopra): trasporto di `process`, messaggio in giù, timbro di
    build, prontezza del reattore, ciclo di lettura. Lo paga il sotto-progetto 12.
  - **Niente GPU su Windows per il tracciatore** (F2, F9): se un giorno serve, la via è ONNX
    Runtime con RTMPose, e quel giorno la manutenzione di RTMPose si rimisura.
  - **Tre ipotesi restano ipotesi fino a SP-7**: che MediaPipe su CPU regga 30 Hz su questa
    macchina (S1), che il giro a 30 Hz sia accettabile (S2), e che una riserva da zero passi
    l'ammissione (S3, sonda nel kernel).

- **Follow-up richiesti:**
  - **SP-7** in `spikes/`, coi criteri scritti prima della misura, e la sonda **S3** in
    `crates/kernel/tests/arbiter_admission.rs`.
  - I tre **rimandi datati** in testa ad ADR-0001, ADR-0011 e ADR-0023.
  - La riga **12 «Gesti»** in [`roadmap.md`](../roadmap.md), che dipende da 2 e 3, e la Voce che
    dipende anche da 12; le righe in [`tracciabilita.md`](../tracciabilita.md), sezione «Voce e
    gesti».
  - Il **sotto-progetto 12** costruisce il worker, le due varianti di `FromWorker`, il messaggio
    in giù, il parametro della frequenza, il campo di `Degradation`, e decide la 13.
```

- [ ] **Passo 3: le case dei totali** — lo stesso comando del Passo 3 del compito 1, che ora rende
i valori del compito 1; la sostituzione è ancora di **uno**: `38 ADR` → `39 ADR`, `37 ADR in stato`
→ `38 ADR in stato`, `38 decisioni architetturali` → `39 decisioni architetturali`, sulle stesse
dieci frasi.

- [ ] **Passo 4: il compendio, CRLF — la voce in §5 e l'intestazione**

| Trova | Sostituisci con |
|---|---|
| l'ultima riga della voce `**0038 — …**`: `primo invocatore, il click del sotto-progetto 2; quali funzioni siano gestuali lo decide il 12.` | resta; **dopo** di essa, una riga vuota e la voce nuova qui sotto |
| l'intestazione: `**Aggiornato il <data del compito 1>**, con **ADR-0038** — il registro delle funzioni del programma — in §5, dal piano del riconoscimento gesti; l'ultimo contenuto di **merito** è quella voce. Manutenzione: §13.` | `**Aggiornato il <data>**, con **ADR-0038** e **ADR-0039** — il registro delle funzioni del programma, la telecamera come sorgente di percezione — in §5, dal piano del riconoscimento gesti; l'ultimo contenuto di **merito** sono quelle due voci. Manutenzione: §13.` |

```markdown
**0039 — La telecamera come sorgente di percezione always-on sotto il core.** Un worker Python
**possiede** la telecamera e i fotogrammi **non escono mai**; al core arrivano **eventi** — lo
stato continuo della mano (21 punti, coordinate **intere**) e il gesto discreto (enum chiuso,
confidenza intera) — sul porto `process`, una `instruct_stream` poi `read_next` per tutta la
vita, `minicbor` (§6.10). **Eventi, non passi** (ADR-0011): il core smista la manipolazione alla
GUI con `Ipc::send`, **campionata** a una frequenza consegnata (ADR-0034); un gesto di **comando**
apre un passo nella run aperta, dal registro di ADR-0038. **Solo la wake word apre una run.**
Concessione da **zero MiB**, `Preemption::Never`, chiesta all'**accensione**: la telecamera è
**spenta per default**, e accenderla è una funzione del registro; **«riservato» la spegne**
(rimando ad ADR-0023); il campo di `Degradation` nasce col worker. La GUI **disegna la mano dai
21 punti**, niente video, e porta un **indicatore** acceso dal core. ⛔ **Il primo worker vero paga**
— trasporto di `process`, messaggio in giù, timbro di build, prontezza del reattore, ciclo di
lettura — e lo paga il sotto-progetto **12**; la Voce riusa. Tre ipotesi le misurano **SP-7** e
la sonda S3; il confinamento del worker (decisione 13) e la terza quota (decisione 9) restano
**registrati**. Le fonti F1–F9 in [`riferimenti.md`](riferimenti.md).
```

- [ ] **Passo 5: l'indice di `README.md`, CRLF** — sotto la riga di `[0038]`:
`| [0039](adr/0039-telecamera-come-sorgente-di-percezione.md) | La telecamera come sorgente di percezione always-on sotto il core | Accepted |`

- [ ] **Passo 6: le prove, il commit, il push** — le stesse del Passo 6 del compito 1, con
`0039-telecamera-come-sorgente-di-percezione.md` al posto di `0038-…`.

```bash
git add docs/adr/0039-telecamera-come-sorgente-di-percezione.md docs/COMPENDIO.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/AVVIO-CHAT.md CLAUDE.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 2): ADR-0039, la telecamera come sorgente di percezione always-on sotto il core — eventi e non passi, spenta per default, solo la wake word apre una run; la voce in §5, la riga nell'indice e i totali nello stesso commit"
git push
```

#### Criterio di chiusura del compito 2

- [ ] l'ADR esiste, `Accepted`, con le `Negative (accettate)` della riga B della §3.1 tutte presenti, più il costo del primo worker e le tre ipotesi
- [ ] il perimetro negativo porta le decisioni 2, 7, 9, 13 e `workers/` come **registrate**, non prese
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, totali a **39** e **38**, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 3: i tre richiami datati — ADR-0001, ADR-0011, ADR-0023 — e i rimandi in §5

**Files:**
- Modify: `docs/adr/0001-architettura-a-kernel-con-capacita-paritarie.md` (LF) · `docs/adr/0011-routing-risolto-e-giornalato-per-richiesta.md` (LF) · `docs/adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md` (**CRLF**) · `docs/COMPENDIO.md` (CRLF) — tre voci di §5, la §1, l'intestazione · `CLAUDE.md` (CRLF) — la frase dei quattro pilastri
- Read: §3.2 e §3.3 del disegno; il rimando di AUD-032 in testa ad [ADR-0005](../../adr/0005-arbitrato-gpu-su-due-dimensioni.md), che è la forma

**Interfaces:**
- Consumes: i due ADR dei compiti 1 e 2, che i richiami linkano

- [ ] **Passo 1: le misure prima, e le righe da ritrovare col `grep`**

```bash
bash scripts/check-docs.sh
git ls-files --eol docs/adr/0001-*.md docs/adr/0011-*.md docs/adr/0023-*.md docs/COMPENDIO.md CLAUDE.md
for f in docs/adr/0023-*.md docs/COMPENDIO.md CLAUDE.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
grep -n -i quattro docs/adr/0001-*.md | head -3
grep -n -i percettiv docs/adr/0011-*.md | head -3
grep -n -i 'voce always-on' docs/adr/0023-*.md
grep -n 'Deciders' docs/adr/0001-*.md docs/adr/0011-*.md docs/adr/0023-*.md
```

Atteso: i due ADR `w/lf`, il terzo `w/crlf` con CR = righe; le frasi della §3.2 del disegno
ancora al loro posto; la riga `- **Deciders:** proprietario del progetto` a riga **5** di tutti e
tre, seguita da una riga vuota e da `## Context`.

- [ ] **Passo 2: i tre richiami, in testa, sotto `Deciders`**

Per ciascun ADR: dopo la riga vuota che segue `- **Deciders:** proprietario del progetto`, e
**prima** di `## Context`, entra il blockquote qui sotto seguito da una riga vuota. Con
`replace_unique.py`, Trova = `- **Deciders:** proprietario del progetto` + riga vuota + `## Context`;
Sostituisci con = le stesse tre righe col blockquote in mezzo. Su ADR-0023 l'aiutante converte
in CRLF da sé.

ADR-0001:

```markdown
> ⚠️ **Rimando del <data> — il pilastro «voce» si legge «voce e gesti», e i pilastri restano
> quattro.** Il [disegno del riconoscimento gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md)
> (decisione 1, sotto accettazione condizionata) mette il riconoscimento gesti dalla telecamera
> **dentro** il pilastro della voce, come sorgente di percezione sorella della wake word
> ([ADR-0039](0039-telecamera-come-sorgente-di-percezione.md)), e costruisce quel pilastro in
> due sotto-progetti — l'8 «Voce» e il 12 «Gesti» — come il kernel lo è in quattro. **Nessuna
> riga di questo ADR è superata:** un quinto pilastro avrebbe richiesto un ADR che supera la
> parola «quattro» per un contendente della GPU che oggi non la usa, ed è stato scartato. I
> consumatori paritari guadagnano un invocatore in più — il gesto — con lo stesso permesso di
> tutti gli altri ([ADR-0038](0038-registro-delle-funzioni-del-programma.md)).
```

ADR-0011:

```markdown
> ⚠️ **Rimando del <data> — nella tabella dell'inferenza percettiva always-on entra il
> tracciamento delle mani.** Il [disegno del riconoscimento gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md)
> e [ADR-0039](0039-telecamera-come-sorgente-di-percezione.md) leggono la riga *«wake word, VAD,
> trascrizione continua»* con un esempio in più: lo **stato continuo della mano** e il **gesto
> discreto** sono eventi di una sorgente di percezione, non passi — non passano dal gateway e
> non si giornalano per fotogramma. Un gesto di **comando** fa come la trascrizione che diventa
> messaggio: **apre un passo** in una run già aperta; i fotogrammi che l'hanno prodotto no. E
> per decisione 3 del disegno **solo la wake word apre una run**: la cella *«può avviare una
> run»* resta vera della voce, non del gesto. Nessuna riga è superata.
```

ADR-0023:

```markdown
> ⚠️ **Rimando del <data> — il profilo «riservato» disattiva anche la telecamera.** Il punto 5
> della decisione dice *«disattiva avvio automatico e voce always-on»*: con
> [ADR-0039](0039-telecamera-come-sorgente-di-percezione.md) la telecamera è una seconda
> sorgente di percezione always-on sotto il core, e il
> [disegno](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) (§3.2) la mette nella
> stessa frase — *«e la telecamera»*. La ragione è quella di questo ADR: le due cose sono
> mutuamente esclusive, e fingere il contrario sarebbe disonesto. Nessuna riga è superata; il
> meccanismo arriva col profilo, che nel codice non esiste ancora.
```

- [ ] **Passo 3: il compendio, CRLF — i tre rimandi in §5, la §1, l'intestazione**

Le tre voci **rimandano**, non ricopiano (vincolo 10). Ogni Trova è l'ultima frase della voce,
unica nel file; il testo nuovo si **appende** sulla stessa riga.

| Trova | Sostituisci con |
|---|---|
| `È la decisione fondativa: tutto il resto ne discende.` | `È la decisione fondativa: tutto il resto ne discende. ⚠️ **Rimando del <data>, in testa all'ADR:** «voce» si legge **«voce e gesti»**, i pilastri restano quattro, nessuna riga superata — ADR-0039.` |
| `registra **anche per gli stream interrotti**.` | `registra **anche per gli stream interrotti**. ⚠️ **Rimando del <data>, in testa all'ADR:** nell'inferenza percettiva entra il **tracciamento delle mani**; un gesto di comando apre un passo, i fotogrammi no — ADR-0039.` |
| `e fingere il contrario sarebbe disonesto.` | `e fingere il contrario sarebbe disonesto. ⚠️ **Rimando del <data>, in testa all'ADR:** il profilo «riservato» disattiva **anche la telecamera** — ADR-0039.` |
| in §1: `coding, voce, generazione asset 3D — su un **kernel comune**. Nessun pilastro prevale,` | `coding, voce e gesti, generazione asset 3D — su un **kernel comune**. Nessun pilastro prevale,` |
| l'intestazione, da `**Aggiornato il` a `Manutenzione: §13.` sulla stessa riga | `**Aggiornato il <data>**, coi tre rimandi datati del riconoscimento gesti — ADR-0001, ADR-0011, ADR-0023 — nelle voci di §5; l'ultimo contenuto di **merito** è quello. Manutenzione: §13.` |

⚠️ Se uno dei tre Trova non fosse unico, l'aiutante rifiuta: si allarga il Trova alla riga
intera, non si sceglie «la prima».

- [ ] **Passo 4: `CLAUDE.md`, CRLF — la frase dei quattro pilastri (D3)**

| Trova | Sostituisci con |
|---|---|
| `coding, voce, generazione asset 3D — su un **kernel comune** (ADR-0001).` | `coding, voce e gesti, generazione asset 3D — su un **kernel comune** (ADR-0001, col rimando datato in testa).` |

- [ ] **Passo 5: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/adr/0001-*.md docs/adr/0011-*.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; echo; done
for f in docs/adr/0023-*.md docs/COMPENDIO.md CLAUDE.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol docs/adr/0001-*.md docs/adr/0011-*.md docs/adr/0023-*.md docs/COMPENDIO.md CLAUDE.md
grep -c 'Rimando del' docs/adr/0001-*.md docs/adr/0011-*.md docs/adr/0023-*.md
git diff --stat
```

Atteso: `OK`, `GATE GREEN`, margine positivo, `0` CR sui due LF, CR = righe sui tre CRLF,
`ls-files --eol` invariato, `1` per ciascun ADR, e un diff che aggiunge righe **senza
toglierne** nei tre ADR — append-only.

```bash
git add docs/adr/0001-architettura-a-kernel-con-capacita-paritarie.md docs/adr/0011-routing-risolto-e-giornalato-per-richiesta.md docs/adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md docs/COMPENDIO.md CLAUDE.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 3): i tre rimandi datati in testa ad ADR-0001, ADR-0011 e ADR-0023 — «voce e gesti», il tracciamento fra l'inferenza percettiva, «riservato» spegne la telecamera — e le voci di §5 che vi rimandano"
git push
```

#### Criterio di chiusura del compito 3

- [ ] i tre ADR portano il blockquote datato in testa, sotto `Deciders`, e **nessuna riga preesistente** è cambiata (`git diff` senza righe `-` in quei file)
- [ ] le tre voci di §5 rimandano in una frase; la §1 e `CLAUDE.md` dicono «voce e gesti»; `README.md` non è toccato (D3)
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 4: la riga 12 «Gesti» e le dipendenze in `roadmap.md`; le righe di `tracciabilita.md`

**Files:**
- Modify: `docs/roadmap.md` (**CRLF** oggi, P-2) · `docs/tracciabilita.md` (**CRLF** oggi, P-2) · `docs/HANDOFF.md` (CRLF) e `docs/COMPENDIO.md` (CRLF) — la sola parola «dodici» (P-6) e l'intestazione del compendio
- Read: §5.1, §5.2, §5.3 del disegno e la voce **4** delle *«voci che questo disegno apre»*; le trappole 6 e 7

**Interfaces:**
- Consumes: i due ADR, che la riga 12 linka
- Produces: la riga **12** e la tabella «Spike aperti» con **SP-7** `⬜`, che il compito 8 chiude; il titolo «Voce e gesti» della sezione 6

- [ ] **Passo 1: le misure prima**

```bash
bash scripts/check-docs.sh
git ls-files --eol docs/roadmap.md docs/tracciabilita.md docs/HANDOFF.md docs/COMPENDIO.md
for f in docs/roadmap.md docs/tracciabilita.md docs/HANDOFF.md docs/COMPENDIO.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
grep -n -E '^\| \*{0,2}[0-9a-z]{1,3}\*{0,2} \|' docs/roadmap.md | tail -4
grep -n '^## \|^### ' docs/tracciabilita.md
grep -n 'Comandi rapidi' docs/tracciabilita.md
grep -rn 'dodici sotto-progetti' docs/*.md CLAUDE.md
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
```

Atteso: l'ultima riga della tabella dei sotto-progetti è la **11**; la sezione `## 6. Voce`
esiste ed è seguita da `## 7.`; `Comandi rapidi` sta nella sezione 2; le due case di «dodici»;
e il conteggio per stato, da rilanciare **dopo** per vedere le righe nuove.

- [ ] **Passo 2: `roadmap.md` — la riga 12, la riga 8, «Perché quest'ordine», gli spike, l'intestazione**

| Trova | Sostituisci con |
|---|---|
| `\| 8 \| Voce \| L2 \| ⬜ \| 7 · chiude **SP-2** \|` | `\| 8 \| Voce \| L2 \| ⬜ \| 7, **12** · chiude **SP-2** \|` |
| la riga della tabella dei sotto-progetti che comincia con `\| 11 \| **Backup e ripristino**` | resta; **sotto** di essa: `\| 12 \| **Gesti** — il riconoscimento gesti dalla telecamera: il worker sotto il core, le due specie di evento, il primo invocatore gestuale del registro — [ADR-0038](adr/0038-registro-delle-funzioni-del-programma.md) · [ADR-0039](adr/0039-telecamera-come-sorgente-di-percezione.md) · [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) \| L2 + L1 est. \| ⬜ \| 2, 3 \|` — l'etichetta di strato è la **D4** |
| la riga di «Perché quest'ordine» che comincia con `\| **Generazione asset prima di Voce** \|` | resta; **sotto** di essa: `\| **Gesti dopo GUI minima e Conversazione, e prima di Voce** \| dal 2 i pannelli mobili e il primo invocatore del registro ([ADR-0038](adr/0038-registro-delle-funzioni-del-programma.md)); dal 3 la run che un gesto di comando comanda. Il **primo worker vero paga** — trasporto di \`process\`, messaggio in giù, timbro di build, prontezza del reattore, ciclo di lettura — e la Voce li **riusa** (§2.4 del [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md)). I gesti non usano la GPU: la ragione che mette Voce dopo Generazione asset — SP-2 vuole voce e job GPU pesante insieme — non li riguarda (decisione 11) \|` |
| la riga `\| SP-4 \| provider con annullamento senza addebito \| ordine di preferenza §3 \| ⬜ \|` | resta; **sotto** di essa (D8): `\| SP-7 \| MediaPipe su CPU regge 30 Hz su questa macchina? quanto costa il giro worker → core → GUI a 30 Hz? \| il sotto-progetto **12**: le tre ipotesi della §6.4 del [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md), coi criteri scritti prima in \`spikes/gesti/PROTOCOLLO.md\` \| ⬜ \|` |
| `Ultimo aggiornamento: **2026-09-03**, alla chiusura del **sotto-progetto 1** contro la §0.7 della sua spec, e lo stesso giorno con la riga del piano del riconoscimento gesti nella tabella dei piani.` | `Ultimo aggiornamento: **<data>**, con la riga **12 «Gesti»**, la dipendenza nuova della riga 8 e lo spike **SP-7** — dalla §5.1 del [disegno del riconoscimento gesti](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md).` |

⛔ **Senza rinumerare** (trappola 6): la riga 12 va **in coda** alla tabella, e le righe 0…11 non
cambiano posto. Il «Perché quest'ordine» è **riletto per intero** contro la riga nuova: la riga
*«Generazione asset prima di Voce»* resta vera — parla di SP-2 — e nessun'altra parla della Voce.
Se una lo facesse, riceve un richiamo datato.

- [ ] **Passo 3: `tracciabilita.md` — il titolo, le righe, il registro, il riquadro**

| Trova | Sostituisci con |
|---|---|
| `## 6. Voce` | `## 6. Voce e gesti` seguito da una riga vuota e da: `⚠️ **RICHIAMO DEL <data>:** la sezione si chiamava «Voce»; con [ADR-0039](adr/0039-telecamera-come-sorgente-di-percezione.md) il pilastro si legge «voce e gesti» — rimando datato in testa ad ADR-0001 — e le righe dei gesti stanno qui, con sede **Gesti**, il sotto-progetto 12.` |
| la riga `\| Convivenza pipeline audio ↔ job GPU \| ✅ \| §2 · quota sottratta, corsie (ADR-0005) \|` | resta; **sotto** di essa, sei righe: `\| Telecamera come sorgente di percezione always-on \| 🔶 \| porta \`process\` §6.10 (ADR-0039) · il worker, le due specie di evento, il campo di degrado · implementazione → Gesti \|` · `\| Tracciamento delle mani — 21 punti, stato continuo \| 📋 \| Gesti \|` · `\| Gesti di comando \| 📋 \| Gesti \|` · `\| Manipolazione di pannelli e menu con le mani \| 📋 \| GUI + Gesti \|` · `\| Cattura con un gesto \| 📋 \| Gesti — la destinazione la decide il brainstorming della knowledge base (decisione 7 del disegno) \|` · `\| Indicatore di telecamera accesa \| 📋 \| GUI \|` |
| la riga `\| Comandi rapidi e slash-command \| 📋 \| GUI \|` | resta; **sotto** di essa: `\| Registro delle funzioni del programma \| 🔶 \| registro di [ADR-0038](adr/0038-registro-delle-funzioni-del-programma.md), nella forma dei registri §5 (ADR-0009) · registrazione, invocazione, permesso come tripla, giornale · implementazione → GUI minima, col primo invocatore \|` |
| nel riquadro in testa, la riga `> ✅ **Sotto-progetto 1 chiuso il 2026-09-03 contro la §0.7**: la tabella si riaggiorna a ogni` e la riga seguente `> sotto-progetto chiuso, come dice la riga sotto il titolo.` | restano; **sotto** di esse, una riga `>` vuota e: `> ✅ **Aggiornata il <data> con le righe del riconoscimento gesti**, dalla §5.2 del [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) approvato dal proprietario — fuori da una chiusura di sotto-progetto, e per questo detto: la sezione 6 diventa «Voce e gesti», e la riga del registro delle funzioni entra nella sezione 2 accanto ai comandi rapidi.` |

La forma delle righe 🔶 è quella della legenda e delle righe già 🔶 del file —
`<meccanismo> §N · <che cosa si scaglia> · implementazione → <sede>` — e il numero di sezione
`6` non cambia (P-11).

- [ ] **Passo 4: «dodici» esce dalle due case (P-6)**

| File | Trova | Sostituisci con |
|---|---|---|
| `docs/HANDOFF.md` | `\| [\`roadmap.md\`](roadmap.md) \| dodici sotto-progetti, ordine, dipendenze, decisioni aperte \|` | `\| [\`roadmap.md\`](roadmap.md) \| i sotto-progetti, l'ordine, le dipendenze, le decisioni aperte — **quanti** siano lo dice la sua tabella \|` |
| `docs/COMPENDIO.md` | `\| l'ordine dei dodici sotto-progetti e le dipendenze \| [\`roadmap.md\`](roadmap.md) \|` | `\| l'ordine dei sotto-progetti e le dipendenze — quanti siano lo dice la tabella di quel file \| [\`roadmap.md\`](roadmap.md) \|` |
| `docs/COMPENDIO.md` | l'intestazione, da `**Aggiornato il` a `Manutenzione: §13.` | `**Aggiornato il <data>**, con la riga 12 «Gesti» in roadmap e le righe di tracciabilità dal piano del riconoscimento gesti; l'ultimo contenuto di **merito** è la §12 che smette di contare i sotto-progetti. Manutenzione: §13.` |

- [ ] **Passo 5: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/roadmap.md docs/tracciabilita.md docs/HANDOFF.md docs/COMPENDIO.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol docs/roadmap.md docs/tracciabilita.md docs/HANDOFF.md docs/COMPENDIO.md
grep -n -E '^\| \*{0,2}[0-9a-z]{1,3}\*{0,2} \|' docs/roadmap.md | tail -2
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
grep -rn 'dodici sotto-progetti' docs/*.md CLAUDE.md
```

Atteso: `OK`, `GATE GREEN`, CR = righe sui quattro, `ls-files --eol` invariato, l'ultima riga
della tabella è la **12** e la **11** è al suo posto, i conteggi per stato salgono di **due** 🔶
e **cinque** 📋 rispetto al Passo 1, e «dodici sotto-progetti» resta **solo** nel gotcha #70 di
`HANDOFF.md`, che è un verbale.

```bash
git add docs/roadmap.md docs/tracciabilita.md docs/HANDOFF.md docs/COMPENDIO.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 4): la riga 12 «Gesti» in roadmap, L2 + L1 est., che dipende da 2 e 3; la Voce dipende anche da 12; SP-7 fra gli spike aperti; la sezione 6 di tracciabilità diventa «Voce e gesti» con le sue righe, e il registro delle funzioni accanto ai comandi rapidi"
git push
```

#### Criterio di chiusura del compito 4

- [ ] la riga 12 esiste, in coda, con `L2 + L1 est.` e le dipendenze `2, 3`; la riga 8 dipende anche da 12; nessuna riga rinumerata
- [ ] «Perché quest'ordine» ha la riga dei gesti e nessuna riga falsa sulla Voce
- [ ] SP-7 è nella tabella degli spike, `⬜`
- [ ] la sezione 6 è «Voce e gesti» col richiamo, le sei righe e la riga del registro nella sezione 2; il riquadro in testa porta la data
- [ ] «dodici sotto-progetti» non vive più in una riga viva
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 5: le fonti F1–F9 in `riferimenti.md`

**Files:**
- Modify: `docs/riferimenti.md` (**CRLF**) — una sezione nuova prima di `## Cosa NON abbiamo adottato, e perché`, e tre righe in quella tabella
- Read: §6.2 e §7 del disegno, e la sezione *«Il formato dei canali privati e i pari non-Rust (ADR-0037)»* di `riferimenti.md`, che è la forma

**Interfaces:**
- Produces: la sezione che ADR-0039 e la voce 0039 di §5 nominano come *«le fonti F1–F9 in riferimenti.md»*

- [ ] **Passo 1: le misure prima**

```bash
git ls-files --eol docs/riferimenti.md; printf 'CR='; tr -cd '\r' < docs/riferimenti.md | wc -c; wc -l < docs/riferimenti.md
grep -n '^## Cosa NON abbiamo adottato' docs/riferimenti.md
grep -c -i mediapipe docs/riferimenti.md
```

Atteso: `w/crlf` con CR = righe; l'intestazione trovata **una** volta; **zero** occorrenze di
`mediapipe` — la sezione non esiste ancora (domanda 4 del pre-controllo).

- [ ] **Passo 2: la sezione, prima di «Cosa NON abbiamo adottato»**

Trova = la riga `## Cosa NON abbiamo adottato, e perché`; Sostituisci con = il testo qui sotto
seguito da una riga vuota, `---`, una riga vuota e la stessa intestazione. Le righe F1–F9 sono
quelle della §7 del disegno, **così come sono**, con la data.

```markdown
## Riconoscimento gesti dalla telecamera — le fonti del disegno del 2026-09-03 (ADR-0038, ADR-0039)

Consultate il **2026-09-03** scrivendo il
[disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md), e portate qui col piano,
come la §7 del disegno prescrive. **Nessuna misura in questa sezione:** sono fonti, e le tre
ipotesi che ne discendono le misurano **SP-7** — l'esito in
[`spikes/RISULTATI.md`](../spikes/RISULTATI.md) — e la sonda S3 nel kernel. **F8** e **F9** sono
state rilette alla radice quel giorno, ed è per questo che un'affermazione della consegna del
brainstorming risulta qui **più debole** di come era scritta.

| | Fonte | Che cosa sostiene | Letta il |
|---|---|---|---|
| F1 | PyPI, `mediapipe` — https://pypi.org/project/mediapipe/ | versione **1.0.1** del 2026-08-14; Python 3.9–3.12; ruote per Windows x86-64 e ARM64; Apache 2.0 | 2026-09-03 |
| F2 | Google AI Edge, `BaseOptions` — https://ai.google.dev/edge/api/mediapipe/python/mp/tasks/BaseOptions | *«GPU support is currently limited to Ubuntu platforms»* | 2026-09-03 |
| F3 | Google AI Edge, Gesture Recognizer per Python — https://developers.google.com/edge/mediapipe/solutions/vision/gesture_recognizer/python | modi IMAGE, VIDEO, **LIVE_STREAM** con risultati per callback; **otto** gesti pronti — `None`, `Closed_Fist`, `Open_Palm`, `Pointing_Up`, `Thumb_Down`, `Thumb_Up`, `Victory`, `ILoveYou`; gesti propri con `custom_gestures_classifier_options` | 2026-09-03 |
| F4 | Google AI Edge, Hand Landmarker — https://developers.google.com/edge/mediapipe/solutions/vision/hand_landmarker | **21 punti** per mano, coordinate normalizzate e in metri; su Pixel 6 **17,12 ms** su CPU e **12,27 ms** su GPU; `num_hands` e tre soglie di confidenza; il modello `hand_landmarker.task` in float16 | 2026-09-03 |
| F5 | Google AI Edge, Hand Landmarker per il Web — https://developers.google.com/edge/mediapipe/solutions/vision/hand_landmarker/web_js | `detectForVideo` è sincrono e **blocca il thread della UI**: servono web worker | 2026-09-03 |
| F6 | GitHub, issue *WebGPU support for Vision Tasks* — https://github.com/google-ai-edge/mediapipe/issues/5826 | aperta dal 2025-01-15, in attesa di Google: nel browser oggi è **WebGL** | 2026-09-03 |
| F7 | GitHub, release — https://github.com/google-ai-edge/mediapipe/releases | `v1.0.0` del 2026-07-28, dopo `v0.10.35` e `v0.10.33` | 2026-09-03 |
| F8 | OpenMMLab, RTMPose in `mmpose` — https://github.com/open-mmlab/mmpose/tree/main/projects/rtmpose, e il repository https://api.github.com/repos/open-mmlab/mmpose con l'ultimo rilascio https://api.github.com/repos/open-mmlab/mmpose/releases/latest | Apache 2.0, **non archiviato**; sezione *«Hand 2d (21 Keypoints)»* su COCO-Wholebody-Hand, OneHand10K, FreiHand2d, RHD2d e Halpe; esportazione **ONNX e TensorRT** via MMDeploy, con istruzioni per Windows e `mmdeploy_runtime` su onnxruntime. ⚠️ **Ultimo rilascio `v1.3.2` del 2024-07-12, ultimo push 2025-08-04, novità della pagina ferme a dicembre 2023.** 📌 La consegna del brainstorming citava una fonte **di terza mano** — Forasoft, https://www.forasoft.com/learn/ai-for-video-engineering/articles-ai/openpose-mediapipe-rtmpose-pose-tracking — che diceva *«mantenuta»*: la parola **non regge** letta alla fonte, ed è sostituita qui. Nessuno la citi di nuovo senza risalire a OpenMMLab | 2026-09-03 |
| F9 | GitHub, issue *GPU Delegate is not yet supported for Windows* — https://github.com/google-ai-edge/mediapipe/issues/5126, letta via https://api.github.com/repos/google-ai-edge/mediapipe/issues/5126 e i suoi commenti | aperta il 2024-02-08, **chiusa il 2024-02-25 dal bot per inattività** — *«This issue was closed due to lack of activity after being marked stale for past 7 days»* — con `state_reason: completed`, che è la parola del bot e non una risoluzione. L'unico commento di un collaboratore, del 2024-02-09: *«GPU support in Python is available for Regular Linux and macOS from version 0.10.8 onwards. Unfortunately, Windows support is not yet available.»* F9 **rafforza** F2 | 2026-09-03 |

⚠️ **Che cosa NON è una fonte primaria, dichiarato:** che il pinch si ricavi dalla distanza fra la
punta del pollice e quella dell'indice è la pratica corrente nei progetti che usano i 21 punti, e
il disegno lo dichiara **dedotto** (§6.3). Lo spike SP-7 lo usa così.

⚠️ **E che cosa la versione di Python decide:** F1 dà `mediapipe` 1.0.1 per Python **3.9–3.12**;
su questa macchina `py -0` elenca 3.14, 3.13 e 3.10, quindi lo spike gira su **`py -3.10`** —
pre-controllo P-4 del [piano](superpowers/plans/2026-09-03-riconoscimento-gesti.md).
```

- [ ] **Passo 3: tre righe in «Cosa NON abbiamo adottato, e perché»**

Dopo l'ultima riga di quella tabella — quella che comincia con `\| Passphrase come cifratura predefinita \|` — tre righe:

```markdown
| Il tracciamento delle mani **dentro la GUI** — l'approccio 2 del disegno del riconoscimento gesti | muore con la GUI, contraddice lo slot di ADR-0011, blocca il thread della UI (F5): la sorgente di percezione sta **sotto il core** ([ADR-0039](adr/0039-telecamera-come-sorgente-di-percezione.md)). La latenza in più si **misura** (SP-7) invece di darsi per buona |
| Una **terza quota** permanente nella formula di ADR-0005, oggi | varrebbe zero, e il porto `process` pretende già una concessione: si apre quando esiste un tracciatore su **GPU** — decisione 9 del disegno, registrata |
| **Forasoft** come fonte su RTMPose | di terza mano: la parola *«mantenuta»* non regge letta alla fonte (F8, qui sopra) |
```

- [ ] **Passo 4: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
printf 'CR='; tr -cd '\r' < docs/riferimenti.md | wc -c; wc -l < docs/riferimenti.md
git ls-files --eol docs/riferimenti.md
grep -c '^| F[1-9] |' docs/riferimenti.md
```

Atteso: `OK`, `GATE GREEN`, CR = righe, `w/crlf` invariato, **9** righe di fonte.

```bash
git add docs/riferimenti.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 5): le fonti F1–F9 del disegno in riferimenti.md, con la data — F8 risalita a OpenMMLab, F9 chiusa dal bot — e tre righe in «Cosa NON abbiamo adottato»"
git push
```

#### Criterio di chiusura del compito 5

- [ ] le nove fonti con data; F8 porta la fonte di terza mano sostituita; F9 porta il motivo della chiusura
- [ ] le tre righe in «Cosa NON abbiamo adottato»
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 6: la sonda S3 nel kernel, nelle due direzioni, e la sua riga nel registro

**Files:**
- Modify: `crates/kernel/tests/arbiter_admission.rs` (CRLF nell'albero, LF nell'indice) — due sonde e un capoverso del doc di modulo · `docs/porta-di-qualita.md` (**CRLF**) — una sezione in coda (D14)
- Mutate and restore, **byte-esatto, mai committato**: `crates/kernel/src/arbiter/mod.rs`
- Read: §2.5 e §4.2 del disegno; il doc di modulo del banco, che detta la forma — `matches!` e `let … else`, mai `assert_eq!` su un `Admission`; `Arbiter::admit` in `crates/kernel/src/arbiter/mod.rs`

**Interfaces:**
- Consumes: `arbiter(id, total)`, `profile(name, vram, lane)`, `TOTAL`, `LONG` del banco — esistono, misurato il 2026-09-03 con `grep -n '^fn \|^const ' crates/kernel/tests/arbiter_admission.rs`
- Produces: `a_zero_reservation_is_granted_even_on_a_full_machine` e `on_the_same_full_machine_a_real_reservation_is_queued_and_not_granted`, che il registro e il compito 9 nominano

- [ ] **Passo 1: le misure prima**

```bash
bash scripts/gate.sh
cargo test --locked -p kernel --test arbiter_admission 2>&1 | grep 'test result'
git ls-files --eol crates/kernel/tests/arbiter_admission.rs crates/kernel/src/arbiter/mod.rs docs/porta-di-qualita.md
for f in crates/kernel/tests/arbiter_admission.rs crates/kernel/src/arbiter/mod.rs docs/porta-di-qualita.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
grep -n 'ZERO' crates/kernel/tests/arbiter_admission.rs
grep -n -A3 'let asked = profile.reserved_vram;' crates/kernel/src/arbiter/mod.rs
```

Atteso: `GATE GREEN`; il conteggio del banco **prima**; i tre file `i/lf w/crlf` con CR = righe;
`Mib::ZERO` compare solo in asserzioni su `allocated()` — nessuna sonda chiede zero; le righe
di `admit` come il piano le cita.

- [ ] **Passo 2: le due sonde, in coda al banco, in inglese**

Con l'ultima riga del file — la chiusa dell'ultima sonda — come Trova, e la stessa riga seguita
dal testo qui sotto come Sostituisci. Su un file CRLF l'aiutante converte da sé.

```rust

/// ⛔ A RESERVATION OF ZERO IS GRANTED EVEN WHEN THE MACHINE IS FULL, and that is the sonda S3
/// of the gesture design (2026-09-03, §4.2): the camera worker of ADR-0039 tracks on the CPU and
/// asks for `Mib::ZERO`, and the port `process` still wants a grant to start it (§5.6). `admit`
/// asks two questions -- "bigger than the whole machine?" and "does it fit NOW?" -- and zero
/// answers no to both whatever the books say; a third question, "is zero a request at all?",
/// does not exist, and this probe is what keeps it from being added by accident. ⚠️ NO
/// CATALOGUE ROW: §7.4 is spec, so the probe is registered and not taken -- the same treatment
/// as PL-1 and K-1/B-1 (gotcha #36) -- and `docs/porta-di-qualita.md` says so in its own words.
#[test]
fn a_zero_reservation_is_granted_even_on_a_full_machine() {
    let mut arbiter = arbiter(ArbiterId::new(1), TOTAL);
    let Admission::Granted(_resident) = arbiter.admit(
        &profile("resident", 16_384, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };
    assert_eq!(arbiter.allocated(), TOTAL);

    let camera = ResourceProfile {
        name: "camera-reserved",
        reserved_vram: Mib::ZERO,
        compute_class: ComputeClass::Realtime,
        preemption: Preemption::Never,
    };
    let Admission::Granted(_camera) = arbiter.admit(&camera, LONG, Monotonic::ORIGIN) else {
        panic!("zero fits a full machine: there is nothing to wait for");
    };
    assert_eq!(arbiter.allocated(), TOTAL, "a zero reservation takes nothing");
    assert_eq!(arbiter.queued(), 0, "and it never waited");
}

/// The other direction of S3, and it is the one that makes the first non-vacuous: on the SAME
/// full machine a reservation that is not zero -- one MiB is enough -- waits. Without this probe
/// an `admit` that granted everything would keep the probe above green (gotcha #24).
#[test]
fn on_the_same_full_machine_a_real_reservation_is_queued_and_not_granted() {
    let mut arbiter = arbiter(ArbiterId::new(1), TOTAL);
    let Admission::Granted(_resident) = arbiter.admit(
        &profile("resident", 16_384, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };

    let Admission::Queued(_) = arbiter.admit(
        &profile("real", 1, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("one MiB on a full machine waits");
    };
    assert_eq!(arbiter.queued(), 1);
    assert_eq!(arbiter.allocated(), TOTAL, "a queued request reserves nothing");
}
```

`TOTAL` vale `Mib::new(16_384)` nel banco, e `profile` prende i MiB come `u64`: il residente da
`16_384` riempie la macchina **esatta**, che è il caso che il disegno chiede — *«anche a
macchina piena»*.

- [ ] **Passo 3: il doc di modulo cresce, come il suo stesso richiamo del 2026-08-19 prescrive**

Il doc di modulo enumera i soggetti per compito e dice di sé che chi aggiunge un soggetto
**estende** l'enumerazione invece di sovrascriverla. Dopo il capoverso che comincia con
`//! ⚠️ TWO DIFFERENT RULES ABOUT COMPARISON LIVE HERE, DELIBERATELY.` — che è l'ultimo prima
di `use kernel::arbiter::{` — entra:

```rust
//!
//! ⛔ AND THE ZERO RESERVATION FROM THE GESTURE DESIGN OF 2026-09-03, WHICH IS A FOURTH SUBJECT:
//! that a request of `Mib::ZERO` is `Granted` even on a machine that is exactly full, and -- the
//! other direction, on the SAME full machine -- that a reservation of one MiB waits. It is the
//! sonda S3 of that design (§4.2): the camera worker of ADR-0039 tracks on the CPU and asks for
//! zero, and `Process::start` still wants a grant. ⚠️ NO CATALOGUE ROW holds these two probes --
//! §7.4 is spec -- and the register `docs/porta-di-qualita.md` says so in its own words, where
//! its index command looks.
```

- [ ] **Passo 4: verde, poi le due mutazioni — e la revoca provata con `cmp`**

```bash
cargo test --locked -p kernel --test arbiter_admission 2>&1 | grep -E 'zero_reservation|real_reservation|test result'
cp crates/kernel/src/arbiter/mod.rs "$SCRATCH/mod.rs.orig"
```

Atteso: le due sonde `ok`, e il conteggio del Passo 1 **più due**.

**Mutazione M-a** — in `admit`, subito dopo la riga `let asked = profile.reserved_vram;`, una
riga: `if asked == Mib::ZERO { return Admission::Refused { asked, ceiling }; }`. Con
`replace_unique.py` (Trova = `        let asked = profile.reserved_vram;`, unico nel file).

```bash
cargo test --locked -p kernel --test arbiter_admission 2>&1 | grep -E 'zero_reservation|real_reservation|test result'
cp "$SCRATCH/mod.rs.orig" crates/kernel/src/arbiter/mod.rs && cmp "$SCRATCH/mod.rs.orig" crates/kernel/src/arbiter/mod.rs && echo restored
```

Atteso: `a_zero_reservation_is_granted_even_on_a_full_machine ... FAILED`, l'altra `ok`;
`restored`.

**Mutazione M-b** — in `admit`, nel confronto `if self.allocated().saturating_add(asked) > ceiling {`,
`asked` sostituito da `Mib::ZERO` — cioè l'ammissione smette di chiedere «entra adesso?».

```bash
cargo test --locked -p kernel --test arbiter_admission 2>&1 | grep -E 'zero_reservation|real_reservation|test result'
cp "$SCRATCH/mod.rs.orig" crates/kernel/src/arbiter/mod.rs && cmp "$SCRATCH/mod.rs.orig" crates/kernel/src/arbiter/mod.rs && echo restored
git status --porcelain crates/
```

Atteso: `on_the_same_full_machine_a_real_reservation_is_queued_and_not_granted ... FAILED`, e con
essa le sonde delle code che quel confronto tiene — **quante lo dice il comando**, e va nel
registro; la prima sonda `ok`; `restored`; `git status` che nomina **solo** il banco.

- [ ] **Passo 5: la riga nel registro, in coda a `docs/porta-di-qualita.md`, CRLF**

Trova = l'ultima riga del file; Sostituisci con = la stessa riga, una riga vuota, e:

```markdown
## ⛔ LA SONDA S3 DEL RICONOSCIMENTO GESTI — <data>

Due sonde in `crates/kernel/tests/arbiter_admission.rs`, dal compito 6 del
[piano del riconoscimento gesti](superpowers/plans/2026-09-03-riconoscimento-gesti.md), per la
§4.2 del [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md):
`a_zero_reservation_is_granted_even_on_a_full_machine` e
`on_the_same_full_machine_a_real_reservation_is_queued_and_not_granted`. ⚠️ **Non hanno una
riga di catalogo**: la §7.4 è spec (vincolo globale 7), quindi la sonda si **registra** e non si
prende — stesso trattamento di PL-1 e di K-1/B-1, stessa ragione (gotcha #36). Se pretenda una
riga propria lo decide il proprietario: voce 6 delle *«voci che questo disegno apre»* del disegno.

| Mutazione, su `crates/kernel/src/arbiter/mod.rs` | Esito misurato il <data> |
|---|---|
| in `admit`, dopo `let asked = profile.reserved_vram;`: `if asked == Mib::ZERO { return Admission::Refused { asked, ceiling }; }` | `a_zero_reservation_is_granted_even_on_a_full_machine` **rossa**, l'altra verde: `<la riga test result del comando>` |
| in `admit`, nel confronto `self.allocated().saturating_add(asked) > ceiling`, `asked` sostituito da `Mib::ZERO` | `on_the_same_full_machine_a_real_reservation_is_queued_and_not_granted` **rossa**, e con essa le sonde delle code che quel confronto tiene: `<la riga test result del comando>` |

Revocate entrambe da una copia byte-esatta presa prima, `cmp` a zero, e `git status` che nomina
solo il banco.
```

- [ ] **Passo 6: le prove, il commit, il push**

```bash
bash scripts/gate.sh; bash scripts/check-docs.sh
cargo fmt --all --check
for f in crates/kernel/tests/arbiter_admission.rs crates/kernel/src/arbiter/mod.rs docs/porta-di-qualita.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol crates/kernel/tests/arbiter_admission.rs crates/kernel/src/arbiter/mod.rs docs/porta-di-qualita.md
git diff --name-only -- crates/ Cargo.lock
```

Atteso: `GATE GREEN`, `OK`, `cargo fmt --all --check` **pulito** — il cancello non lo impone e
ogni compito che scrive Rust se lo chiede da sé — CR = righe sui tre, `ls-files --eol`
invariato, e il diff su `crates/` che nomina **solo** il banco.

```bash
git add crates/kernel/tests/arbiter_admission.rs docs/porta-di-qualita.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 6): la sonda S3 — una riserva da zero MiB è Granted anche a macchina piena, e sulla stessa macchina un MiB vero è Queued — nelle due direzioni, con le due mutazioni misurate e revocate; registrata senza riga di catalogo"
git push
```

#### Criterio di chiusura del compito 6

- [ ] le due sonde passano; ciascuna va rossa sotto la propria mutazione, e le mutazioni sono revocate byte-esatte
- [ ] il doc di modulo enumera il quarto soggetto; il registro porta la sezione con gli esiti misurati
- [ ] `crates/kernel/src/` non cambia in nessun commit; `cargo fmt --all --check` pulito
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 7: SP-7 — il protocollo, poi il codice

**Files:**
- Create, **primo commit**: `spikes/gesti/PROTOCOLLO.md` (LF)
- Create, **secondo commit**: `spikes/gesti/requirements.txt` · `spikes/gesti/s1_bench.py` · `spikes/gesti/s2_worker.py` · `spikes/gesti/relay/Cargo.toml` · `spikes/gesti/relay/src/main.rs` · `spikes/gesti/relay/page.html` (tutti LF)
- Modify, **secondo commit**: `.gitignore`
- Read: §4.2 e §6.4 del disegno; [`spikes/PROTOCOLLO.md`](../../../spikes/PROTOCOLLO.md) per la forma; P-4 e P-7

**Interfaces:**
- Produces: i criteri **S1** e **S2** che il compito 8 misura, e il codice che li misura; `py -3.10` come interprete in ogni comando

⛔ **Niente in `crates/` né in `workers/`**, che non esiste e non nasce da questo piano
(trappola 9). Lo spike è **usa e getta**: nessuna sua riga sale nel prodotto.

- [ ] **Passo 1: le misure prima**

```bash
py -0
py -3.10 --version
ls spikes/
grep -n 'spikes' .gitignore Cargo.toml
```

Atteso: `3.10` fra gli interpreti (P-4); `spikes/gesti/` **non esiste** (domanda 4);
`exclude = ["spikes"]` nel manifesto di radice, e le righe di `.gitignore` per `rust/` e
`gui-ipc/`, che sono la forma.

- [ ] **Passo 2: il protocollo, e il primo commit**

```markdown
# SP-7 — Riconoscimento gesti: il protocollo

Criteri scritti **prima** della misura, come vuole la §4.2 del
[disegno](../../docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md), e
**congelati al primo commit di codice di questo spike**. Un criterio soddisfatto con un
accorgimento si registra come **parziale**, non come passato — la regola di
[`PROTOCOLLO.md`](../PROTOCOLLO.md), che vale anche qui.

**Che cosa misura, e che cosa no.** Le due ipotesi **assunte** dal disegno (§6.4): S1 e S2. La
terza, S3, non è uno spike: è una sonda nel kernel, `crates/kernel/tests/arbiter_admission.rs`.
Lo spike **non** misura il riconoscimento di un gesto discreto (F3): il vocabolario è della
capacità. E non misura niente sulla GPU: su Windows, in Python, non c'è (F2, F9).

**La macchina.** Il criterio vale **su questa macchina**: la CPU la dice
`powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name"` e va nell'esito con le
versioni degli strumenti. La telecamera è quella integrata o collegata il giorno della misura, e
si dichiara.

## S1 — MediaPipe Hand Landmarker su CPU regge 30 Hz?

| | |
|---|---|
| Domanda | il tempo per fotogramma del tracciatore su CPU, a **due mani**, **640×480**, modo **LIVE_STREAM** |
| Criterio | **mediana < 33 ms e p95 < 33 ms** su almeno **600** risultati consecutivi con **entrambe le mani in campo**; il margine si **riporta**, non si promette |
| Che cosa si riporta | mediana, p95, massimo, numero di risultati, risultati al secondo, fotogrammi inviati e **scartati** (inviati senza risultato), quota di risultati a due mani |
| Come | `s1_bench.py`: la latenza è il tempo fra `detect_async` e la callback, misurato con `time.perf_counter_ns` nello stesso processo |
| Parziale | se il criterio regge solo a una mano, o solo sotto 640×480, o solo con `num_hands=1` |

## S2 — Quanto costa il giro worker → core → GUI a 30 Hz?

| | |
|---|---|
| Domanda | la latenza da **cattura** a **disegno** della mano su una pagina, passando per un relay Rust che sta al posto del core |
| Criterio | il solo salto **relay → pagina** ha **p95 < 100 ms**, che è **P2** di [`GUI-REQUISITI.md`](../GUI-REQUISITI.md); la latenza **totale** da cattura a disegno si **riporta**, mediana e p95, **senza soglia**: l'accettabilità della mano sul pannello la giudica il **proprietario provandola** — il pannello che segue il pinch — e il giudizio va nell'esito con le sue parole |
| Che cosa si riporta | mediana e p95 di cattura → disegno e di relay → disegno su almeno **600** campioni; il giudizio del proprietario |
| Come | `s2_worker.py` emette una riga JSON per risultato; `relay/` lo spawna come processo figlio, legge lo stdout, timbra, e serve la riga in Server-Sent Events a `page.html`, che disegna i 21 punti e misura col proprio orologio. Tre orologi di parete della stessa macchina, in millisecondi |
| Parziale | se regge solo sotto i 30 Hz, o solo a una mano |
| Che cosa non prova | niente sul canale `process` vero (`minicbor`, la busta): il relay usa righe JSON su una pipe, perché qui si misura il **giro**, non il formato |

## Registrazione

L'esito va in [`RISULTATI.md`](../RISULTATI.md), sezione **SP-7**, nella forma di SP-5 e SP-6:
esito per criterio, osservazioni che non sono criteri, versioni degli strumenti, evidenze con
comando e output osservato, e le **divergenze** dall'attesa. I dati grezzi (CSV) restano fuori dal
repository.

## Congelamento

Congelato al primo commit di codice di questo spike. Una modifica dopo quel commit è una
modifica al **metro**, e va detta.
```

```bash
tr -cd '\r' < spikes/gesti/PROTOCOLLO.md | wc -c
bash scripts/check-docs.sh; bash scripts/gate.sh
git add spikes/gesti/PROTOCOLLO.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 7, protocollo): i criteri di SP-7 — S1 e S2 — scritti prima della misura, e congelati al primo commit di codice dello spike"
git push
```

Atteso: `0` CR, `OK` — il controllo dei link legge anche `spikes/`, e i tre link del protocollo
sono relativi a `spikes/gesti/` — `GATE GREEN`.

- [ ] **Passo 3: `requirements.txt`, e `.gitignore`**

`spikes/gesti/requirements.txt`, una riga:

```
mediapipe==1.0.1
```

⚠️ `cv2` **non** è elencato apposta: la ruota di `mediapipe` decide se lo porta con sé, e il
compito 8 lo **misura** con `py -3.10 -c "import cv2"` nella `venv`; se manca, `opencv-python`
entra in questa lista **con la voce d'errata** che lo dice. Il `requirements.lock` lo scrive il
compito 8 con `pip freeze`.

In `.gitignore`, sotto la riga `/spikes/gui-ipc/Cargo.lock`, cinque righe nella stessa forma:

```
/spikes/gesti/.venv/
/spikes/gesti/relay/target/
/spikes/gesti/relay/Cargo.lock
/spikes/gesti/*.task
/spikes/gesti/*.csv
```

Il modello `hand_landmarker.task` è un binario scaricato, i CSV sono i dati grezzi: nessuno dei
due entra nel repository. `git ls-files --eol .gitignore` **prima** di scriverlo, e i suoi
fine-riga si conservano.

- [ ] **Passo 4: `s1_bench.py`**

```python
"""SP-7 / S1 -- does MediaPipe Hand Landmarker on the CPU hold 30 Hz on this machine?

The criteria live in PROTOCOLLO.md and were frozen before this file was written. The latency of
one frame is the time between `detect_async` and its callback, both measured in this process with
`time.perf_counter_ns`. A frame that never gets a result is counted as DROPPED, and a drop is a
signal about 30 Hz, not noise.

Usage: py -3.10 s1_bench.py --model hand_landmarker.task --seconds 30 [--csv <outside the repo>]
"""
import argparse
import statistics
import sys
import time

import cv2
import mediapipe as mp

BaseOptions = mp.tasks.BaseOptions
HandLandmarker = mp.tasks.vision.HandLandmarker
HandLandmarkerOptions = mp.tasks.vision.HandLandmarkerOptions
VisionRunningMode = mp.tasks.vision.RunningMode

WIDTH, HEIGHT = 640, 480


def percentile(values, q):
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, int(q * (len(ordered) - 1)))]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", default="hand_landmarker.task")
    ap.add_argument("--seconds", type=int, default=30)
    ap.add_argument("--camera", type=int, default=0)
    ap.add_argument("--hands", type=int, default=2)
    ap.add_argument("--csv", default=None, help="per-result latencies; keep it OUTSIDE the repository")
    args = ap.parse_args()

    submitted = {}  # timestamp_ms -> perf_counter_ns at detect_async
    latencies_ms = []
    hands_per_result = []

    def on_result(result, output_image, timestamp_ms):
        t0 = submitted.pop(timestamp_ms, None)
        if t0 is None:
            return
        latencies_ms.append((time.perf_counter_ns() - t0) / 1e6)
        hands_per_result.append(len(result.hand_landmarks))

    options = HandLandmarkerOptions(
        base_options=BaseOptions(model_asset_path=args.model),
        running_mode=VisionRunningMode.LIVE_STREAM,
        num_hands=args.hands,
        result_callback=on_result,
    )

    cap = cv2.VideoCapture(args.camera)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, WIDTH)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, HEIGHT)
    ok, frame = cap.read()
    if not ok:
        sys.exit("no frame from the camera")
    print(f"camera frame: {frame.shape[1]}x{frame.shape[0]}", file=sys.stderr)

    sent = 0
    last_ts = -1
    start = time.perf_counter_ns()
    with HandLandmarker.create_from_options(options) as landmarker:
        while time.perf_counter_ns() - start < args.seconds * 1_000_000_000:
            ok, frame = cap.read()
            if not ok:
                break
            rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
            image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb)
            # LIVE_STREAM wants strictly increasing timestamps: two frames in one ms collide.
            ts = max(last_ts + 1, (time.perf_counter_ns() - start) // 1_000_000)
            last_ts = ts
            submitted[ts] = time.perf_counter_ns()
            landmarker.detect_async(image, ts)
            sent += 1
        time.sleep(0.5)  # let the last callbacks land before the landmarker closes
    cap.release()
    elapsed_s = (time.perf_counter_ns() - start) / 1e9

    if not latencies_ms:
        sys.exit("no result came back: nothing to report")
    two_hands = sum(1 for n in hands_per_result if n >= 2)
    print(
        f"results {len(latencies_ms)}  sent {sent}  dropped {len(submitted)}  "
        f"results/s {len(latencies_ms) / elapsed_s:.1f}"
    )
    print(
        f"latency ms: median {statistics.median(latencies_ms):.2f}  "
        f"p95 {percentile(latencies_ms, 0.95):.2f}  max {max(latencies_ms):.2f}"
    )
    print(f"results with two hands: {two_hands} of {len(latencies_ms)}")
    if args.csv:
        with open(args.csv, "w", encoding="utf-8") as f:
            f.write("latency_ms,hands\n")
            for lat, n in zip(latencies_ms, hands_per_result):
                f.write(f"{lat:.3f},{n}\n")


if __name__ == "__main__":
    main()
```

- [ ] **Passo 5: `s2_worker.py`**

```python
"""SP-7 / S2 -- the worker half of the round trip worker -> relay -> page.

One JSON line per result on stdout: {"t_capture_ms": <wall-clock ms when the frame was read>,
"hands": [[[x, y], ... 21 points ...], ...]}, with x and y as INTEGER pixels of a 640x480 frame:
integers on the wire is the rule of the design (§2.3), and the worker does the scaling. The relay
spawns this script as a child process and reads its stdout, which is the topology of the core and
a worker. Stops when stdout is closed by the relay, or on Ctrl-C.

Usage: py -3.10 s2_worker.py hand_landmarker.task
"""
import json
import sys
import threading
import time

import cv2
import mediapipe as mp

BaseOptions = mp.tasks.BaseOptions
HandLandmarker = mp.tasks.vision.HandLandmarker
HandLandmarkerOptions = mp.tasks.vision.HandLandmarkerOptions
VisionRunningMode = mp.tasks.vision.RunningMode

WIDTH, HEIGHT = 640, 480


def main():
    model = sys.argv[1] if len(sys.argv) > 1 else "hand_landmarker.task"
    captured_at = {}  # timestamp_ms -> wall-clock ms at capture
    stop = threading.Event()

    def on_result(result, output_image, timestamp_ms):
        t_capture = captured_at.pop(timestamp_ms, None)
        if t_capture is None:
            return
        hands = [
            [[int(round(lm.x * WIDTH)), int(round(lm.y * HEIGHT))] for lm in hand]
            for hand in result.hand_landmarks
        ]
        try:
            sys.stdout.write(json.dumps({"t_capture_ms": t_capture, "hands": hands}) + "\n")
            sys.stdout.flush()
        except (BrokenPipeError, OSError):
            stop.set()  # the relay is gone: the worker has nobody to talk to

    options = HandLandmarkerOptions(
        base_options=BaseOptions(model_asset_path=model),
        running_mode=VisionRunningMode.LIVE_STREAM,
        num_hands=2,
        result_callback=on_result,
    )
    cap = cv2.VideoCapture(0)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, WIDTH)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, HEIGHT)
    last_ts = -1
    start = time.perf_counter_ns()
    with HandLandmarker.create_from_options(options) as landmarker:
        while not stop.is_set():
            ok, frame = cap.read()
            if not ok:
                break
            wall_ms = time.time_ns() // 1_000_000
            rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
            image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb)
            ts = max(last_ts + 1, (time.perf_counter_ns() - start) // 1_000_000)
            last_ts = ts
            captured_at[ts] = wall_ms
            landmarker.detect_async(image, ts)
    cap.release()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
```

- [ ] **Passo 6: il relay, std-only (D7)**

`spikes/gesti/relay/Cargo.toml`:

```toml
[package]
name = "sp7-relay"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
```

`spikes/gesti/relay/src/main.rs`:

```rust
//! SP-7 / S2 -- the relay that stands where the core will stand.
//!
//! It spawns the Python worker as a CHILD PROCESS and reads its stdout line by line -- the
//! topology of the core and a worker, on a pipe -- stamps each line with its own wall clock, and
//! serves it to the page over Server-Sent Events. Throwaway: no dependency, no error handling
//! beyond what the measurement needs, and nothing of it climbs into `crates/`.
//!
//! Routes: `GET /` -> the page; `GET /stream` -> `text/event-stream`, one `data:` per result.
//! Usage: cargo run --release -- <python> <worker.py> <model.task> [port]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const PAGE: &str = include_str!("../page.html");

fn wall_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_millis()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: sp7-relay <python> <worker.py> <model.task> [port]");
        std::process::exit(2);
    }
    let port = args.get(4).map(|p| p.as_str()).unwrap_or("7878");

    let mut child = Command::new(&args[1])
        .args([&args[2], &args[3]])
        .stdout(Stdio::piped())
        .spawn()
        .expect("the worker did not start");
    let stdout = child.stdout.take().expect("piped stdout");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let sinks = Arc::clone(&clients);
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            let Some(body) = line.strip_suffix('}') else { continue };
            // {"t_capture_ms":..,"hands":[..]}  ->  the same object plus the relay's own stamp.
            let event = format!("data: {body},\"t_relay_ms\":{}}}\n\n", wall_ms());
            let mut sinks = sinks.lock().expect("lock");
            sinks.retain_mut(|s| s.write_all(event.as_bytes()).and_then(|_| s.flush()).is_ok());
        }
        eprintln!("worker stdout closed");
    });

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).expect("bind");
    eprintln!("open http://127.0.0.1:{port}/ in the browser; Ctrl-C to stop");
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut request = [0u8; 1024];
        let n = stream.read(&mut request).unwrap_or(0);
        let head = String::from_utf8_lossy(&request[..n]);
        if head.starts_with("GET /stream") {
            let _ = stream.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\n\r\n",
            );
            clients.lock().expect("lock").push(stream);
        } else {
            let _ = stream.write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    PAGE.len(),
                    PAGE
                )
                .as_bytes(),
            );
        }
    }
    let _ = child.kill();
}
```

`spikes/gesti/relay/page.html`:

```html
<!doctype html>
<meta charset="utf-8">
<title>SP-7 · S2</title>
<style>
  body { margin: 0; font: 14px system-ui; background: #111; color: #ddd; }
  #stage { position: relative; width: 640px; height: 480px; }
  canvas { position: absolute; left: 0; top: 0; }
  #panel { position: absolute; left: 200px; top: 150px; width: 160px; height: 100px;
           background: #2a6; opacity: .8; border-radius: 8px; }
  #stats { white-space: pre; padding: 8px; }
</style>
<div id="stage"><div id="panel"></div><canvas id="c" width="640" height="480"></canvas></div>
<button id="dump">dump stats</button>
<pre id="stats"></pre>
<script>
// SP-7 / S2 -- the page that stands where the GUI will stand. It draws the 21 points of each
// hand (mirrored, so the hand moves the way the owner moves it), drags the panel while the first
// hand PINCHES -- thumb tip 4 and index tip 8 closer than PINCH pixels -- and measures with its
// own wall clock: capture -> draw, and relay -> draw. Nothing here is product code.
const PINCH = 40;
const CHAINS = [[0,1,2,3,4],[0,5,6,7,8],[5,9,10,11,12],[9,13,14,15,16],[13,17,18,19,20],[0,17]];
const ctx = document.getElementById('c').getContext('2d');
const panel = document.getElementById('panel');
const captureToDraw = [], relayToDraw = [];
let grab = null;

const mirror = (hand) => hand.map(([x, y]) => [640 - x, y]);
function percentile(a, q) { const s = [...a].sort((x, y) => x - y); return s[Math.min(s.length - 1, Math.floor(q * (s.length - 1)))]; }

function draw(hands) {
  ctx.clearRect(0, 0, 640, 480);
  ctx.strokeStyle = '#8cf'; ctx.fillStyle = '#fff'; ctx.lineWidth = 2;
  for (const hand of hands) {
    for (const chain of CHAINS) {
      ctx.beginPath();
      chain.forEach((i, k) => k ? ctx.lineTo(hand[i][0], hand[i][1]) : ctx.moveTo(hand[i][0], hand[i][1]));
      ctx.stroke();
    }
    for (const [x, y] of hand) { ctx.beginPath(); ctx.arc(x, y, 4, 0, Math.PI * 2); ctx.fill(); }
  }
}

function pinch(hands) {
  if (!hands.length) { grab = null; return; }
  const [tx, ty] = hands[0][4], [ix, iy] = hands[0][8];
  if (Math.hypot(tx - ix, ty - iy) > PINCH) { grab = null; return; }
  if (!grab) grab = { dx: ix - panel.offsetLeft, dy: iy - panel.offsetTop };
  panel.style.left = (ix - grab.dx) + 'px';
  panel.style.top = (iy - grab.dy) + 'px';
}

new EventSource('/stream').onmessage = (e) => {
  const m = JSON.parse(e.data);
  const hands = m.hands.map(mirror);
  draw(hands);
  pinch(hands);
  const now = Date.now();
  captureToDraw.push(now - m.t_capture_ms);
  relayToDraw.push(now - m.t_relay_ms);
};

document.getElementById('dump').onclick = () => {
  const stats = {
    samples: captureToDraw.length,
    capture_to_draw_ms: { median: percentile(captureToDraw, 0.5), p95: percentile(captureToDraw, 0.95) },
    relay_to_draw_ms: { median: percentile(relayToDraw, 0.5), p95: percentile(relayToDraw, 0.95) },
  };
  document.getElementById('stats').textContent = JSON.stringify(stats, null, 2);
};
</script>
```

- [ ] **Passo 7: il relay compila, e il secondo commit**

```bash
(cd spikes/gesti/relay && cargo build --release 2>&1 | tail -2)
for f in spikes/gesti/requirements.txt spikes/gesti/s1_bench.py spikes/gesti/s2_worker.py spikes/gesti/relay/Cargo.toml spikes/gesti/relay/src/main.rs spikes/gesti/relay/page.html; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; echo; done
git ls-files --eol .gitignore; printf '.gitignore CR='; tr -cd '\r' < .gitignore | wc -c
git status --porcelain
bash scripts/gate.sh; bash scripts/check-docs.sh
```

Atteso: `Finished` senza avvisi; `0` CR sui sei file nuovi; `.gitignore` coi fine-riga di prima;
`git status` che nomina i sei file, `.gitignore` e questo piano — **non** `target/` né
`Cargo.lock` del relay; `GATE GREEN`, `OK` — `spikes/` è fuori dal workspace e il cancello non
lo compila, misurato dal cancello stesso. ⚠️ Il codice Python **non gira** in questo compito: la
`venv`, il modello e la telecamera sono del compito 8, e ciò che qui si consegna è testo che il
compito 8 esegue **così com'è**, o registra nell'errata perché ha dovuto cambiarlo.

```bash
git add .gitignore spikes/gesti/requirements.txt spikes/gesti/s1_bench.py spikes/gesti/s2_worker.py spikes/gesti/relay/Cargo.toml spikes/gesti/relay/src/main.rs spikes/gesti/relay/page.html docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 7, codice): SP-7 — il banco di S1 su MediaPipe, il worker di S2, il relay std-only che lo spawna e serve la pagina in SSE, la pagina che disegna i 21 punti e muove un pannello col pinch; niente in crates/"
git push
```

#### Criterio di chiusura del compito 7

- [ ] due commit: il protocollo **prima** del codice, leggibile in `git log`
- [ ] il relay compila senza avvisi; nessun file di `crates/` cambia; `workers/` non esiste
- [ ] `.gitignore` copre `venv`, `target/`, `Cargo.lock` del relay, il modello e i CSV
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga LF sui file nuovi, commit pushati, posizione aggiornata

---

## Compito 8: SP-7 — la misura col proprietario, e l'esito in `spikes/RISULTATI.md`

⛔ **Questo compito vuole il proprietario davanti alla telecamera di questa macchina.** Il
subagente prepara l'ambiente e lancia S1 da solo; per S2 il giudizio sulla mano che muove il
pannello è **suo**, con le sue parole, e senza di esso il compito non si chiude.

**Files:**
- Create: `spikes/gesti/requirements.lock` (LF) — l'uscita di `pip freeze`
- Modify: `spikes/RISULTATI.md` (**CRLF**) — la sezione SP-7 e la riga della data · `docs/roadmap.md` (**CRLF**) — la riga SP-7 della tabella degli spike · `docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` (LF) — un richiamo datato in §6.4 per ciascuna delle tre ipotesi · `spikes/gesti/requirements.txt` **solo** se `cv2` manca (compito 7, Passo 3)
- Read: `spikes/gesti/PROTOCOLLO.md`; le sezioni SP-5/SP-6 di `spikes/RISULTATI.md` per la forma

**Interfaces:**
- Consumes: i criteri S1 e S2 congelati; il codice del compito 7
- Produces: la sezione **SP-7** di `RISULTATI.md`, che il compito 9 nomina in §8 del compendio

- [ ] **Passo 1: l'ambiente, misurato e non assunto**

```bash
py -3.10 -m venv spikes/gesti/.venv
spikes/gesti/.venv/Scripts/python -m pip install -r spikes/gesti/requirements.txt 2>&1 | tail -3
spikes/gesti/.venv/Scripts/python -c "import mediapipe, cv2; print('mediapipe', mediapipe.__version__, 'cv2', cv2.__version__)"
spikes/gesti/.venv/Scripts/python -m pip freeze > spikes/gesti/requirements.lock
tr -cd '\r' < spikes/gesti/requirements.lock | wc -c
```

Atteso: `mediapipe 1.0.1` e una versione di `cv2`. ⚠️ Se `import cv2` fallisce, `opencv-python`
entra in `requirements.txt` (Passo 3 del compito 7) e la voce d'errata lo registra con la versione
che `pip freeze` dà. ⚠️ Se `pip` rifiutasse la ruota su 3.10, ci si ferma: è l'errata, non un
cambio di interprete a occhio. Se `pip freeze` scrivesse CRLF, si riscrive LF con Python:
`newline=''` in lettura e `"\n"` in scrittura.

Il modello, dalla pagina F4 dei modelli letta il giorno della misura — l'URL letto il
2026-09-03 è `https://storage.googleapis.com/mediapipe-models/hand_landmarker/hand_landmarker/float16/latest/hand_landmarker.task`:

```bash
curl -L -o spikes/gesti/hand_landmarker.task "<l'URL letto quel giorno su F4>"
ls -l spikes/gesti/hand_landmarker.task
git status --porcelain spikes/
```

Atteso: il file esiste e `git status` **non** lo nomina (`.gitignore`).

- [ ] **Passo 2: S1, due mani in campo per tutta la corsa**

```bash
cd spikes/gesti && .venv/Scripts/python s1_bench.py --model hand_landmarker.task --seconds 30 --csv "$SCRATCH/s1.csv"; cd ../..
```

Si tengono **entrambe le mani in campo** per i trenta secondi. Le quattro righe stampate vanno
nell'esito così come sono. Se il criterio non regge — mediana o p95 sopra i 33 ms, o meno di 600
risultati a due mani — **si registra e non si aggiusta**: le righe *Parziale* del protocollo
dicono che cosa si può riprovare, e ogni prova in più va nelle evidenze.

- [ ] **Passo 3: S2, col proprietario**

```bash
cd spikes/gesti/relay && cargo run --release -- ../.venv/Scripts/python ../s2_worker.py ../hand_landmarker.task
```

Si apre `http://127.0.0.1:7878/` nel browser. Il proprietario muove la mano, **pinza** — pollice e
indice — e trascina il pannello verde; dopo almeno **600** campioni preme *dump stats* e il JSON
va nell'esito così com'è. Il suo giudizio sulla mano che muove il pannello va nell'esito **con le
sue parole**, non riassunto. `Ctrl-C` nel terminale chiude relay e worker.

- [ ] **Passo 4: le versioni e la macchina**

```bash
spikes/gesti/.venv/Scripts/python --version
spikes/gesti/.venv/Scripts/python -m pip show mediapipe | grep -i '^version'
rustc --version
powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name"
powershell -NoProfile -Command "Get-PnpDevice -Class Camera -Status OK | Select-Object -ExpandProperty FriendlyName"
```

- [ ] **Passo 5: `spikes/RISULTATI.md`, CRLF — la sezione SP-7 e la riga della data**

| Trova | Sostituisci con |
|---|---|
| `Data di esecuzione: **2026-08-06**` | `Data di esecuzione: **2026-08-06** per SP-5 e SP-6; **SP-7** porta la propria data nella sua sezione` |
| la riga `## SP-6 — Confine dei dati non fidati, e confini statici del kernel` | la sezione qui sotto, una riga vuota, e la stessa riga |

```markdown
## SP-7 — Riconoscimento gesti: MediaPipe su CPU, e il giro worker → core → GUI — eseguito il <data>

Criteri e soglie: [`gesti/PROTOCOLLO.md`](gesti/PROTOCOLLO.md), congelato il <data del compito 7>
al primo commit di codice dello spike, **prima** della misura. Codice in `gesti/`; dati grezzi
fuori dal repository. La terza ipotesi del disegno, S3, non è qui: è una sonda nel kernel,
registrata in `docs/porta-di-qualita.md`.

| Criterio | Esito | Misura |
|---|---|---|
| S1 — su CPU, due mani, 640×480, LIVE_STREAM: mediana e p95 < 33 ms su ≥ 600 risultati a due mani | <✅ `passa` · ⚠️ `parziale` · ❌ `non passa`> | mediana **<n> ms**, p95 **<n> ms**, massimo <n> ms; <n> risultati, <n> al secondo, <n> inviati, <n> scartati; <n> risultati a due mani |
| S2 — relay → pagina p95 < 100 ms (P2); cattura → disegno **riportato** | <esito> | relay → disegno: mediana **<n> ms**, p95 **<n> ms**; cattura → disegno: mediana **<n> ms**, p95 **<n> ms**; <n> campioni |

**Il giudizio del proprietario sulla mano che muove il pannello, con le sue parole:** «<…>».

### SP-7 · Osservazioni registrate — non criteri

| # | Osservazione |
|---|---|
| O1 | <la quota di fotogrammi scartati e che cosa dice sui 30 Hz; la stabilità del pinch; ciò che ha sorpreso> |

### SP-7 · Versioni degli strumenti

| Strumento | Comando | Output |
|---|---|---|
| Python | `spikes/gesti/.venv/Scripts/python --version` | `<…>` — **`py -3.10`**, perché `mediapipe` 1.0.1 vuole 3.9–3.12 (F1) |
| MediaPipe | `pip show mediapipe` | `<…>`; il resto dell'ambiente in `gesti/requirements.lock` |
| Rust, il relay | `rustc --version` | `<…>` |
| CPU | `Get-CimInstance Win32_Processor` | `<…>` |
| Telecamera | `Get-PnpDevice -Class Camera` | `<…>` |
| Modello | l'URL letto su F4 il <data> | `hand_landmarker.task`, float16, <byte> byte |

### SP-7 · Evidenze

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| S1 | `py -3.10 s1_bench.py --model hand_landmarker.task --seconds 30` | le quattro righe stampate, testuali | <l'attesa era i 17 ms del Pixel 6 come speranza, non come prova: che cosa è uscito di diverso> |
| S2 | `cargo run --release -- <python> ../s2_worker.py ../hand_landmarker.task`, poi *dump stats* | il JSON della pagina, testuale | <…> |
```

⛔ **Le celle fra `<…>` sono slot di misura**, e la sezione non si committa con uno slot vuoto:
ogni cifra viene dall'output, ogni giudizio dal proprietario. Un criterio non superato resta
scritto **non superato**: il metro è congelato, e il disegno dice che il margine *si riporta, non
si promette*.

- [ ] **Passo 6: la roadmap e il disegno**

In `docs/roadmap.md`, CRLF, la riga SP-7 della tabella degli spike (compito 4): `⬜` diventa
`✅ **chiuso il <data>**: <una riga con l'esito di S1 e di S2, senza cifre — le cifre stanno in RISULTATI.md>`
e il code span `spikes/gesti/PROTOCOLLO.md` diventa il link
`[spikes/gesti/PROTOCOLLO.md](../spikes/gesti/PROTOCOLLO.md)`; se un criterio è `parziale` o
`non passa`, la riga lo dice con quella parola.

Nel disegno, LF, la §6.4 *«Assunto, e lo misura SP-7»*: sotto ciascuno dei tre punti, una riga
`⚠️ **Misurato il <data>:** <l'esito in una riga, senza cifre> — l'esito in [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md), sezione SP-7.`
Per il punto 3: `— la sonda S3 è nel cancello dal compito 6 del piano, nelle due direzioni.`

- [ ] **Passo 7: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
for f in spikes/RISULTATI.md docs/roadmap.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
for f in spikes/gesti/requirements.lock docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; echo; done
git ls-files --eol spikes/RISULTATI.md docs/roadmap.md
grep -c '<' spikes/RISULTATI.md
git status --porcelain
```

Atteso: `OK`, `GATE GREEN`, CR = righe sui due CRLF, `0` sui due LF, `ls-files --eol` invariato,
il `grep` su `<` che rende **quanto rendeva prima della sezione** — nessuno slot rimasto — e
`git status` che nomina i file della mappa e **nessun** `.task`, `.csv`, `.venv`.

```bash
git add spikes/gesti/requirements.lock spikes/RISULTATI.md docs/roadmap.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 8): SP-7 misurato — S1 <esito>, S2 <esito>, col giudizio del proprietario sulla mano che muove il pannello; l'esito in RISULTATI.md, la riga chiusa in roadmap, i richiami nella §6.4 del disegno"
git push
```

#### Criterio di chiusura del compito 8

- [ ] la sezione SP-7 esiste con le quattro sottosezioni, nessuno slot vuoto, il giudizio del proprietario con le sue parole
- [ ] `requirements.lock` committato; il modello e i CSV **fuori** dal repository
- [ ] la roadmap chiude SP-7; il disegno porta i tre richiami in §6.4
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 9: la chiusura — la Definizione di «fatto», e i documenti di stato

**Files:**
- Modify: `docs/COMPENDIO.md` (CRLF) — il puntatore in §6, la riga SP-7 in §8, due righe in §12, l'intestazione · `docs/README.md` (CRLF) — la riga del disegno nella tabella «Specifiche» (P-1) · `docs/roadmap.md` (CRLF) — la riga di questo piano nella tabella dei piani · il disegno (LF) — la spunta della §5.5 e un richiamo · questo piano (LF) — la posizione
- Read: la Definizione di «fatto» nella §5.5 del disegno, che è la tabella di questo compito

**Interfaces:**
- Consumes: tutto ciò che i compiti 1–8 hanno prodotto, verificato **coi comandi** e non letto dalla tabella della posizione

- [ ] **Passo 1: la Definizione di «fatto», condizione per condizione, coi comandi**

| # | Condizione (§5.5 del disegno) | Comando, ed esito atteso |
|---|---|---|
| 1 | i due ADR esistono, `Accepted`, con `Negative (accettate)` e la voce in §5; i totali aggiornati | `bash scripts/check-docs.sh` → `OK`; `grep -c 'Negative (accettate)' docs/adr/0038-*.md docs/adr/0039-*.md` → `1` e `1` |
| 2 | i tre richiami in testa, append-only, e le voci di §5 rimandano | `grep -c 'Rimando del' docs/adr/0001-*.md docs/adr/0011-*.md docs/adr/0023-*.md` → `1`, `1`, `1`; `grep -c 'in testa all.ADR' docs/COMPENDIO.md` → `3`; `git log --format=%h -3 -- docs/adr/0001-*.md` e `git show --stat` di quel commit senza righe `-` nei tre ADR |
| 3 | la riga 12 e la dipendenza della riga 8, senza rinumerare; «Perché quest'ordine» riletto | `grep -n -E '^\| \*{0,2}[0-9a-z]{1,3}\*{0,2} \|' docs/roadmap.md` — la 12 in coda, 0…11 nell'ordine di prima; `grep -c 'Gesti dopo' docs/roadmap.md` → `1` |
| 4 | le righe della §5.2 e il titolo nuovo della sezione 6; il comando del riquadro gira | `grep -n '^## 6\.' docs/tracciabilita.md` → `Voce e gesti`; `for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done` gira; `grep -c 'Gesti' docs/tracciabilita.md` ≥ 6 |
| 5 | F1–F9 con le date, F8 e F9 nella forma della §7 | `grep -c '^| F[1-9] |' docs/riferimenti.md` → `9`; `grep -c 'Forasoft' docs/riferimenti.md` ≥ 1 |
| 6 | SP-7: criteri prima della misura, l'esito nella forma di SP-5/SP-6, il giudizio del proprietario | `git log --format='%h %ad %s' --date=short -- spikes/gesti/PROTOCOLLO.md spikes/RISULTATI.md` — il protocollo **prima**; `grep -n '^## SP-7\|^### SP-7' spikes/RISULTATI.md` → cinque righe; `grep -c 'giudizio del proprietario' spikes/RISULTATI.md` ≥ 1 |
| 7 | S3 nelle due direzioni, dentro il cancello | `bash scripts/gate.sh` → `GATE GREEN`; `cargo test --locked -p kernel --test arbiter_admission 2>&1 \| grep -E 'zero_reservation\|real_reservation'` → due `ok` |
| 8 | questo file nella §12 del compendio e nella tabella delle spec di `README.md`; il compendio sotto il tetto; il puntatore della §6 mosso | i Passi 2 e 3 qui sotto, poi `check-docs.sh` e il comando del vincolo 12 |
| 9 | i fine-riga di ogni file toccato rimisurati | `git ls-files --eol` sui file della mappa, contro le colonne della mappa; nessun `i/crlf` toccato: `git diff --name-only 1c0a633..HEAD -- crates/kernel/src/ports/process.rs crates/kernel/tests/ports_are_implementable.rs crates/kernel/tests/reactor_contract.rs crates/platform/src/reactor.rs` vuoto |

E i vincoli globali 1 e 3, rilanciati: le due spec intatte; `crates/` che cambia in un file solo.

- [ ] **Passo 2: il compendio, CRLF — §6, §8, §12, l'intestazione**

| Trova | Sostituisci con |
|---|---|
| in §6, il capoverso che comincia con `✅ **IL PIANO DEL RICONOSCIMENTO GESTI È SCRITTO IL 2026-09-03**` fino alla sua fine — `e nessun'altra li condiziona.` — letto **intero** prima di sostituirlo | `✅ **IL PIANO DEL RICONOSCIMENTO GESTI È ESEGUITO IL <data>** — [piano](superpowers/plans/2026-09-03-riconoscimento-gesti.md), dal [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md): ADR-0038 e ADR-0039 in §5, i tre rimandi datati, la riga 12 «Gesti» in roadmap, le righe di tracciabilità, le fonti, la sonda S3 nel cancello, e SP-7 misurato — l'esito in \`spikes/RISULTATI.md\`, sezione SP-7. ⏭️ **IL PROSSIMO PASSO: il brainstorming DISTINTO della KNOWLEDGE BASE** con \`superpowers:brainstorming\`, in una sessione nuova — è la voce 2 della §7.8 del [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md), e la decisione 7 del disegno dei gesti — dove finisce la cattura — vi entra come domanda. Poi il sotto-progetto 2. Nessuna voce aperta lo sbarra.` |
| in §8: `\| ❌ **rifare gli spike SP-5 e SP-6** \| esiti, seed, versioni e comandi in \`spikes/RISULTATI.md\` \|` | `\| ❌ **rifare gli spike SP-5, SP-6 e SP-7** \| esiti, seed, versioni e comandi in \`spikes/RISULTATI.md\`; per SP-7 il protocollo congelato in \`spikes/gesti/PROTOCOLLO.md\` \|` |
| in §12, la riga `\| il piano della chiusura, con l'errata in testa e la tabella della posizione \| [\`plans/2026-09-02-…-chiusura.md\`](superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md) \|` | resta; **sotto**, due righe: `\| ⛔ **il perimetro del RICONOSCIMENTO GESTI** — la forma della telecamera nel kernel, il registro delle funzioni, le tredici decisioni col loro chiusore, e per ogni artefatto il controllo che lo esercita \| [\`specs/2026-09-03-riconoscimento-gesti-design.md\`](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) — ⚠️ **non è una spec** \|` e `\| come si è **eseguito** il riconoscimento gesti — i due ADR, i rimandi, la roadmap, SP-7 e la sonda S3, con l'errata in testa e la tabella della posizione \| [\`plans/2026-09-03-riconoscimento-gesti.md\`](superpowers/plans/2026-09-03-riconoscimento-gesti.md) — ⚠️ **a compiti, mai intero** \|` |
| l'intestazione, da `**Aggiornato il` a `Manutenzione: §13.` | `**Aggiornato il <data>**, col **PIANO DEL RICONOSCIMENTO GESTI ESEGUITO** e il puntatore della §6 mosso al brainstorming della knowledge base; l'ultimo contenuto di **merito** è quel puntatore. Manutenzione: §13.` |

Dopo ogni tocco: `check-docs.sh` e il margine. ⛔ **Se il tetto va rosso si toglie prosa dalla §6,
non si alza il tetto.**

- [ ] **Passo 3: `README.md`, CRLF — la tabella «Specifiche» (P-1)**

Sotto la riga che comincia con `\| [La chiusura — il disegno]`:
`| [Riconoscimento gesti — il disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) | il riconoscimento gesti dalla telecamera | ⛔ **Non è una spec:** perimetro, forme e il controllo che esercita ciascun artefatto; le tredici decisioni col loro chiusore; la §6.4 porta l'esito di SP-7 |`

- [ ] **Passo 4: `roadmap.md`, CRLF — la riga di questo piano (D9)**

Nella riga della tabella dei piani che comincia con `\| [Riconoscimento gesti](superpowers/plans/2026-09-03-riconoscimento-gesti.md)`,
la cella di stato — da `✅ **scritto il 2026-09-03**` fino alla fine della cella — diventa
`✅ **scritto il 2026-09-03, eseguito il <data>** — `GATE GREEN` a ogni compito; SP-7 nella tabella degli spike, la sonda S3 nel cancello`.

- [ ] **Passo 5: il disegno, LF — la spunta e il richiamo**

Nella §5.5, il punto 4 — la riga che comincia con `4. ✅ **il piano è scritto il 2026-09-03**` —
guadagna, dopo `pre-controllo fatto;`, le parole `**eseguito il <data>**, nove compiti su nove;`
e la frase *«l'**esecuzione** va in una sessione nuova, e a che punto sia lo dice la tabella
della posizione del piano»* diventa *«l'esecuzione è chiusa, e il verbale per compito sta nella
tabella della posizione del piano»*. Sotto l'intestazione `## Il prossimo passo`, il richiamo
del 2026-09-03 della sera **resta** — è un verbale — e sotto di esso una riga:
`✅ **RICHIAMO DEL <data>:** il piano è eseguito; la §6 del compendio porta il passo successivo, il brainstorming della knowledge base.`

- [ ] **Passo 6: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/COMPENDIO.md docs/README.md docs/roadmap.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
printf 'disegno CR='; tr -cd '\r' < docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md | wc -c
git ls-files --eol docs/COMPENDIO.md docs/README.md docs/roadmap.md
grep -n '⏭️' docs/COMPENDIO.md
git diff --name-only 1c0a633..HEAD -- crates/ Cargo.lock Cargo.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
git status --porcelain
```

Atteso: `OK`, `GATE GREEN`, margine positivo, CR = righe sui tre CRLF e `0` sul disegno,
`ls-files --eol` invariato, **due** righe col marcatore nel compendio — il puntatore vivo in §6 e
la riga della tabella delle voci aperte che **nomina** il marcatore, le stesse due di `1c0a633` —
il diff su `crates/` e sulle spec che nomina **solo** `crates/kernel/tests/arbiter_admission.rs`,
e `git status` che nomina i cinque file di questo compito.

```bash
git add docs/COMPENDIO.md docs/README.md docs/roadmap.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md
git commit -m "gesti(compito 9): il piano del riconoscimento gesti è ESEGUITO — la Definizione di «fatto» rilanciata coi comandi, il disegno e il piano nella §12 e in README, e il prossimo passo nella §6: il brainstorming della knowledge base"
git push
```

#### Criterio di chiusura del compito 9

- [ ] le nove condizioni della Definizione di «fatto» verificate **coi comandi** del Passo 1, ed elencate nel rapporto con l'output
- [ ] il disegno e il piano stanno nei **tre** indici: §12 del compendio, «Specifiche» di `README.md`, la tabella dei piani di `roadmap.md`
- [ ] il puntatore della §6 nomina il brainstorming della knowledge base, in un posto solo
- [ ] `check-docs.sh` → `OK`, `GATE GREEN`, margine positivo, fine-riga rimisurati, commit pushato, posizione del piano a nove ✅

---

## Dopo il compito 9

⛔ **Che cosa venga dopo NON è scritto qui:** è uno stato, e la sua casa unica è la §6 del
[compendio](../../COMPENDIO.md), che il compito 9 scrive con le parole del punto 5 della §5.5 del
disegno — il brainstorming **distinto** della knowledge base, poi il sotto-progetto 2. Le voci
che questo piano lascia aperte stanno nella sezione *«Le voci aperte che questo piano SA, e non
chiude»*, e per nessuna il chiusore è questo piano; la decisione **13** — il confinamento del
worker telecamera — aspetta il sotto-progetto 12 col proprietario, e il consiglio scritto è nel
perimetro negativo di ADR-0039.

### Come si riprende — scritto alla chiusura della sessione del 2026-09-03, coi comandi

⚠️ **È il documento di consegna della sessione che ha scritto questo piano**, e sta qui e non in
un file a parte perché il repo ha già la sua convenzione: lo stato vive in file **tracciati**, il
puntatore vive nella §6 del compendio, e chi riprende legge il piano **a compiti**. Ogni riga è
stata **riletta coi comandi** prima di essere scritta, non ricordata.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb`. Nessuno stash, nessuna operazione a metà, nessun file non tracciato |
| I commit di questa sessione | `git log --oneline 1c0a633..HEAD`: **uno**, `903dca1` — questo piano, il puntatore della §6, la riga nella tabella dei piani, la spunta nella §5.5 del disegno; più il commit di questa sezione |
| Codice di prodotto | **non toccato**: `git diff --stat 1c0a633..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende nulla. La sonda usa-e-getta di S3 (P-5) è stata compilata, passata e **cancellata** nella stessa corsa |
| Cancello | `bash scripts/check-docs.sh` → `OK`; `bash scripts/gate.sh` → `GATE GREEN`, **rilanciato alla chiusura**. Si rilanciano, non si citano |
| Fine-riga | questo piano e il disegno **LF** nell'albero e nell'indice; il compendio e la roadmap LF nell'indice e **CRLF** nell'albero, con CR = righe: `git ls-files --eol docs/COMPENDIO.md docs/roadmap.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md` |
| Margine del compendio | il comando del vincolo 12 → positivo alla chiusura; i compiti 1, 2, 3, 4 e 9 lo consumano (P-9) |
| File temporanei | nessuno nel repository; lo scratchpad della sessione è stato svuotato |
| Debito lasciato | **nessuno non dichiarato**: gli slot `<data>` e `<esito>` dei compiti 6, 8 e 9 sono slot di **misura**, da riempire eseguendo; la voce 4 del disegno è presa da D4 e si ribalta nell'errata |

⛔ **Che cosa NON è verificato, detto prima di fidarsi:** il codice dello spike — `s1_bench.py`,
`s2_worker.py`, il relay e la pagina — è **scritto e non eseguito**: né `mediapipe` né `cv2` sono
installati fuori da una `venv` (P-4), e il relay non è stato compilato. L'API è riletta alla
fonte (P-7), ma il primo `py -3.10 s1_bench.py` del compito 8 è la prova, e ciò che non gira va
nell'errata. Le due **mutazioni** di S3 (compito 6) non sono state provate: la sonda usa-e-getta
ha provato solo il **verde**.

**Il compito della sessione successiva: eseguire il piano.** In ordine, e ogni riga è eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -3`: si parte da `main`,
   e la testa deve essere il commit di questa sezione o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e la testa
   dell'audit del 2026-08-27 — poi **questo piano fino alla mappa dei file**, e il disegno per
   intero. I compiti si leggono **uno per volta**, quando si dispacciano.
3. `superpowers:subagent-driven-development`: un subagente fresco per compito, revisione fra uno
   e l'altro, nell'ordine della tabella della posizione. Il brief di un compito è il compito
   stesso più l'errata **verbatim**; il revisore rilancia ogni comando (regola 5 di *«Come si
   esegue»*).
4. Prima di dispacciare un compito, le quattro domande di `CLAUDE.md` contro il codice **di
   allora**: il pre-controllo di questa sessione è del 2026-09-03 su `1c0a633`, e il contratto
   cresce sotto il piano.
5. Il compito 8 si dispaccia solo col **proprietario** presente: la telecamera e il giudizio
   sono suoi.
6. A ogni compito: la tabella della posizione aggiornata nel commit, `GATE GREEN` prima, push
   dopo.
7. A piano eseguito, il compito 9 muove la §6 del compendio al brainstorming della knowledge
   base.

📌 **Trovato scrivendo, e non promosso a gotcha:** il piano della chiusura nomina un aiutante
`mutate.py` che non è tracciato — viveva nello scratchpad di quella sessione — e per chi riprende
non esiste. Questo piano porta il proprio aiutante nel testo. Se sia un gotcha nuovo lo decide
chi lo incontra una seconda volta.

⛔ **Vicoli ciechi di questa sessione: nessuno nuovo.** L'unica cosa caduta è un'attesa: che il
compendio portasse **un** marcatore `⏭️` — ne porta due a `1c0a633`, il puntatore e la riga della
tabella delle voci aperte che lo nomina — corretta nel compito 9 prima del commit.

### Come si riprende — scritto alla chiusura della sessione di ESECUZIONE del 2026-09-03, coi comandi

⚠️ **È il documento di consegna della sessione che ha eseguito i compiti 1, 2 e 3**, chiusa su
richiesta del proprietario prima della revisione del compito 3. La sezione qui sopra è la consegna
della sessione che ha scritto il piano e resta com'è: un verbale. Ogni riga di questa è stata
**riletta coi comandi** prima di essere scritta.

⛔ **DA SAPERE SUBITO: il compito 3 è committato e pushato ma NON RIVISTO.** I compiti 1 e 2 hanno
avuto la revisione piena (e il compito 1 un'ondata di correzione con ri-revisione mirata); il
compito 3 no — l'esecutore ha consegnato `cc001b4`, il cancello è verde, ma nessun revisore ha
rilanciato i comandi né riletto i tre rimandi contro gli ADR che estendono. **La revisione del
compito 3 è il primo passo della ripresa**, prima di dispacciare il compito 4.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb`. Albero pulito, nessuno stash, nessuna operazione a metà |
| I commit di questa sessione | `git log --oneline 4d16f33..HEAD`: **quattro** — `5cdc25a` (compito 1: ADR-0038, voce §5, indice, totali), `4062f7e` (compito 1, ondata 1: il richiamo datato in ADR-0038 e le voci E3/E4), `ee4bae5` (compito 2: ADR-0039), `cc001b4` (compito 3: i tre rimandi in testa ad ADR-0001, ADR-0011, ADR-0023, le voci di §5, «voce e gesti» in §1 e in `CLAUDE.md`) — più il commit di questa sezione |
| La posizione del piano | la tabella in testa: **1, 2, 3 ✅**, 4–9 ⬜. Il numeratore vive lì e in nessun altro posto |
| Codice di prodotto | **non toccato**: `git diff --stat 1c0a633..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla; sotto `docs/adr/` ci sono i due ADR nuovi e i tre coi rimandi (solo righe aggiunte: `git diff ee4bae5..cc001b4 -- docs/adr/` non ha righe `-`) |
| Cancello | `bash scripts/check-docs.sh` → `OK`; `bash scripts/gate.sh` → `GATE GREEN` — **rilanciati alla chiusura dal coordinatore** su `cc001b4`. Si rilanciano, non si citano |
| Fine-riga | il piano `i/lf w/lf`; il compendio, `README.md`, `HANDOFF.md`, `roadmap.md`, `AVVIO-CHAT.md`, `CLAUDE.md`, ADR-0023 `i/lf w/crlf` con CR = righe; ADR-0038, ADR-0039, ADR-0001, ADR-0011 `i/lf w/lf`: `git ls-files --eol docs/adr/00{01,11,23,38,39}-*.md docs/COMPENDIO.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md` |
| Margine del compendio | positivo alla chiusura, col comando del vincolo 12; i compiti 4 e 9 lo consumano ancora |
| L'errata | **E1–E7**, tutte committate, nessuna aperta: E1–E2 e E6–E7 dal pre-controllo dei compiti 1 e 4, E3 dalla revisione del compito 1, E5 dal pre-controllo del compito 2, E4 dal coordinatore — e **corretta dalla ri-revisione**, perché diceva il falso su ADR-0005. Il pre-controllo ha trovato un difetto in **tre compiti su tre** dispacciati: la regola di `CLAUDE.md` regge ancora |
| File temporanei | nessuno nel repository. La ledger SDD e i dispacci di questa sessione stanno in `.superpowers/sdd/2026-09-03-riconoscimento-gesti/` — **git-ignorata, solo su questa macchina**: `progress.md` (baseline, pre-flight scan, ogni decisione con «Ruling:»), `task-N-brief.md`, `task-N-dispatch.md`, `task-N-report.md`, `task-N-review-*.md`, i pacchetti `review-<base>..<head>.diff`. Se non c'è, **questa sezione basta**: tutto ciò che conta è qui |

**Le decisioni prese dal coordinatore eseguendo, col perché** — nessuna è del piano, e il
proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | si lavora su `main` e si committa e pusha a ogni compito, come `CLAUDE.md` prescrive, contro la regola del worktree della skill SDD | le istruzioni del repo vincono sulla skill; costo: commit su `main` da revertire uno per uno |
| 2 | i brief si estraggono a mano con `sed -n` sulle righe del piano; i modelli sono `sonnet` per l'esecutore e `opus` per il revisore; le ri-revisioni mirate `sonnet` | `task-brief` della skill non riconosce «Compito N»; il testo dei compiti è completo ma i file CRLF vogliono cura |
| 3 | il richiamo di **E3** in ADR-0038 sta **accanto alla frase falsa**, come i richiami storici (AUD-032, AUD-004, AUD-033 — in tre sezioni diverse, mai in testa) | un ADR è append-only e un richiamo si legge dove si legge la frase; il vincolo 10 del piano descriveva male la forma di AUD-032 (E4) |
| 4 | i tre rimandi del **compito 3** stanno invece **in testa, sotto `Deciders`**, come il piano detta (D15) | nessuna riga dei tre ADR è falsa: i rimandi estendono la lettura dell'ADR intero; e ADR-0039, già `Accepted`, dice «in testa» in tre punti — spostarli lo renderebbe falso. Costo: tre note lette in testa invece che accanto alla riga |
| 5 | i **Minor** delle revisioni non entrano nelle ondate: si registrano e li giudica la revisione finale del piano | regola della skill SDD; nessuno è un fatto falso |
| 6 | la finestra in cui ADR-0039 e la voce 0039 di §5 nominano *«le fonti F1–F9 in `riferimenti.md`»* mentre quel file non le porta ancora **non è una voce d'errata**: è l'ordine che il piano dichiara, e la chiude il compito 5 | la riga *Produces* del compito 5 lo dice; costo: una finestra di falsità di poche ore su `main` |

**I rilievi Minor rimandati alla revisione finale**, con la casa: ADR-0038 dice *«tre modalità di
supervisione»* dove ADR-0016 dice *«preset»*; in ADR-0039 la riga della sezione *«Costo dichiarato»*
che porta la frase di E5 non è riavvolta (`awk 'length > 200' docs/adr/0039-*.md`); la lettura di
*«vita breve»* in ADR-0039 poggia sul solo worker audio, mentre la §2.2 del disegno portava anche
il requisito R2 di ADR-0004; e la finestra delle fonti F1–F9 (decisione 6 sopra).

**Il compito della sessione successiva: la revisione del compito 3, poi il piano dal compito 4.**
In ordine, e ogni riga è eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -6`: si parte da `main`,
   e la testa deve essere il commit di questa sezione o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e la testa
   dell'audit del 2026-08-27 — poi **questo piano fino alla mappa dei file**, l'errata per intero,
   questa sezione, e il disegno per intero.
3. **La revisione del compito 3**, con `superpowers:subagent-driven-development`: il brief è il
   compito 3 più l'errata verbatim; il diff è `git diff ee4bae5..cc001b4`; il revisore **rilancia
   ogni comando** del Passo 5 del compito, verifica che nei tre ADR il diff abbia solo righe `+`,
   che i tre blockquote siano identici al Passo 2 (data a parte), che le tre voci di §5 rimandino
   in una frase appesa, che la §1 e `CLAUDE.md` dicano «voce e gesti», che `README.md` non sia nel
   diff (D3); e rilegge ciascun rimando contro la frase che estende — ADR-0001 riga *«quattro
   aree»*, ADR-0011 la cella *«**evento**, che può *avviare* una run»*, ADR-0023 il punto 5 — e
   contro ADR-0038 e ADR-0039 che i rimandi linkano (gotcha #59). Un Important apre un'ondata
   con ri-revisione mirata; i Minor si registrano.
4. Il **compito 4**: il suo pre-controllo contro `ee4bae5` è fatto — E6 ed E7 — e si **rifà**
   contro il codice di allora: la tabella dei sotto-progetti con la sonda di E7, le due case di
   «dodici» (`grep -rn 'dodici sotto-progetti' docs/*.md CLAUDE.md` → due vive più il gotcha
   #70), i conteggi per stato di `tracciabilita.md` (misurati il 2026-09-03: ✅ 46 · 🔶 50 ·
   📋 74 · ⚠️ 0 · ❌ 1 — si rimisurano, e dopo il compito salgono di **due** 🔶 e **cinque** 📋).
   La voce **4** del disegno — l'etichetta di strato — è presa da D4, *«L2 + L1 est.»*: il
   proprietario la ribalta nell'errata prima di dispacciare, se vuole altro.
5. Poi i compiti 5, 6 e 7 nell'ordine della tabella; il compito 8 **solo col proprietario**
   presente; il compito 9 chiude e muove la §6 del compendio al brainstorming della knowledge
   base. A ogni compito le quattro domande di `CLAUDE.md` contro il codice **di allora**, la
   tabella della posizione aggiornata nel commit, `GATE GREEN` prima, push dopo.

📌 **Trovato eseguendo, e non promosso a gotcha:** ognuno dei sei difetti d'errata di questa
sessione è un'**attesa scritta senza rilanciare il comando** — nel piano (E1, E2, E6, E7), nel testo
dettato di un ADR (E3, E5) e perfino nell'errata del coordinatore (E4): la forma dei gotcha #31 e
#65, non una nuova. L'unico rimedio che ha retto è quello che il piano già prescrive: **ogni
conteggio si rifà col comando prima di leggere che cosa un documento ne dice** (vincolo 8), e la
revisione lo rifà una seconda volta.

⛔ **Vicoli ciechi di questa sessione: nessuno.** Nessuna decisione del disegno è stata riaperta.

### Come si riprende — scritto alla chiusura della sessione di RIPRESA del 2026-09-03, coi comandi

⚠️ **È il documento di consegna della sessione che ha ripreso dal compito 3 non rivisto**, lo ha
fatto rivedere, ha eseguito e fatto rivedere i compiti 4 e 5, ed è stata chiusa su richiesta del
proprietario subito dopo la revisione del compito 5 — *«a prescindere da tutto»* — e chiusa il
2026-09-04, a mezzanotte passata. Le due sezioni
qui sopra sono le consegne delle sessioni precedenti e restano com'erano: verbali. Ogni riga di
questa è stata **riletta coi comandi** prima di essere scritta.

⛔ **DA SAPERE SUBITO: niente è a metà.** I compiti 1–5 sono committati, pushati e **rivisti** —
tre revisioni in questa sessione (compiti 3, 4, 5), tutte «Approvato», nessun Critical, nessun
Important, nessuna ondata aperta. L'albero è pulito. I compiti **6** e **7** hanno brief e
dispaccio già scritti nel ledger ma **non sono stati dispacciati**. Il primo passo della ripresa è
dispacciare il compito 6.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb` dopo `git fetch --all --prune`. Albero pulito, nessuno stash, nessuna operazione a metà |
| I commit di questa sessione | `git log --oneline 0f98839..HEAD`: `60bdee7` (compito 4: riga 12 in roadmap, tracciabilità, «dodici» tolto, più le voci E8/E9 dell'errata), `684b352` (compito 5: le fonti F1–F9 in `riferimenti.md`) — più il commit di questa sezione |
| La posizione del piano | la tabella in testa: **1–5 ✅**, 6–9 ⬜. Il numeratore vive lì e in nessun altro posto |
| Le revisioni | compito 3 su `ee4bae5..cc001b4`, compito 4 su `0f98839..60bdee7`, compito 5 su `60bdee7..684b352`: tutte **Approvato**, ogni comando rilanciato dal revisore; i Minor rimandati sono elencati sotto |
| Codice di prodotto | **non toccato**: `git diff --stat 1c0a633..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla |
| Cancello | `bash scripts/check-docs.sh` → `OK`; `bash scripts/gate.sh` → `GATE GREEN`; `cargo fmt --all --check` → pulito — **rilanciati alla chiusura dal coordinatore** su `684b352`. Si rilanciano, non si citano |
| Fine-riga | il piano `i/lf w/lf`; `roadmap.md`, `tracciabilita.md`, `HANDOFF.md`, il compendio e `riferimenti.md` `i/lf w/crlf` con CR = righe: `git ls-files --eol docs/roadmap.md docs/tracciabilita.md docs/HANDOFF.md docs/COMPENDIO.md docs/riferimenti.md docs/superpowers/plans/2026-09-03-riconoscimento-gesti.md` |
| Margine del compendio | positivo, col comando del vincolo 12; il compito 9 lo consuma ancora |
| L'errata | **E1–E9**, tutte committate, nessuna aperta. **E8** ed **E9** sono del **compito 6**, trovate dal pre-controllo di questa sessione ed entrate nel commit del compito 4 (precedente: E6/E7 nel commit del compito 3): due *Trova* del compito 6 non erano unici — l'ultima riga del banco è `}`, e la riga del confronto di M-b vive anche in `promote` |
| File temporanei | nessuno nel repository. Il ledger `.superpowers/sdd/2026-09-03-riconoscimento-gesti/` — **git-ignorato, solo su questa macchina** — porta `progress.md` con ogni decisione «Ruling:», i brief, i dispacci, i rapporti e le revisioni dei compiti 1–5, e per i compiti **6** e **7** `task-N-brief.md`, `task-N-dispatch.md` e `task-N-review-dispatch.md` già scritti. ⚠️ **Quei dispacci nominano l'aiutante `replace_unique.py` nello scratchpad di QUESTA sessione, che non esiste più:** chi riprende lo riestrae dal piano — `sed -n '35,70p'` di questo file — nel proprio scratchpad e corregge il percorso nei dispacci prima di inviarli. Se il ledger non c'è, **questa sezione basta**: brief = strumenti (righe 23–78) + vincoli (84–111) + errata (156–176) + il compito, verbatim, come i precedenti |

**Le decisioni prese dal coordinatore riprendendo ed eseguendo, col perché** — nessuna è del piano,
e il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | la consegna precedente è stata **riletta coi comandi** e confermata prima di tutto, e la baseline (cancello, `check-docs.sh`) rimisurata prima di toccare | regola della ripresa: la realtà del repo vince sul racconto; costo: qualche minuto |
| 2 | il dispaccio della revisione del compito 3 — scritto dalla sessione precedente e mai inviato — è stato riscritto in due punti: *«una decisione del coordinatore che non è tua da riaprire»* è diventata contesto con l'invito a segnalare se la posizione contraddice un ADR (la skill SDD vieta di pre-giudicare un rilievo), e `git log -1 --format=%B` è stato appuntato a `cc001b4` perché `HEAD` era il commit della consegna | la revisione ha confermato la posizione in testa (il disegno §3.2 la prescrive, ADR-0039 la dice tre volte): costo zero |
| 3 | **E8** ed **E9** scritte nel piano **prima** di dispacciare il compito 4, e committate col compito 4 | un difetto trovato si scrive quando lo si trova; costo: due voci d'errata in un commit che non le riguarda |
| 4 | il pre-controllo dei compiti 4, 5, 6 e 7 **rifatto sul codice di adesso** mentre giravano le revisioni (sole letture, e prove fuori dall'albero): le due sonde S3 del compito 6 appese parola per parola a una copia usa-e-getta del banco compilano e passano (`2 passed`, copia cancellata, albero pulito); il relay del compito 7 compila **senza avvisi** in una cartella fuori dal repository (cargo 1.95, edition 2024); i due `.py` passano `py -3.10 -m py_compile`; `check-docs.sh` legge anche `spikes/*.md` e i quattro link del protocollo risolvono; nessuno script del cancello cammina `spikes/` | domanda 3 di `CLAUDE.md`: un artefatto sbagliato che compila si coglie solo compilandolo; costo: nessuno, le prove sono cancellate |
| 5 | modelli: esecutore `sonnet`, revisore `opus`, come la sessione precedente; i **Minor** non entrano in ondate, si registrano per la revisione finale (regola SDD) | tre revisioni pulite al primo giro; costo se sbagliato: un Minor che era un fatto — nessuno lo è, riletti uno per uno |
| 6 | **D4** (`L2 + L1 est.`) eseguita senza attendere il proprietario, che ha il consiglio scritto nel disegno (voce 4) e la ribalta nell'errata se vuole altro | il piano la prende con quel consiglio; costo: una parola in una riga della roadmap |
| 7 | il compito 5 dispacciato subito dopo la revisione pulita del 4, senza fermarsi; il 6 e il 7 **non** dispacciati per ordine del proprietario | la consegna precedente prescriveva 5, 6 e 7 in ordine; il proprietario ha chiuso dopo il 5 |

**I rilievi Minor rimandati alla revisione finale**, con la casa — nessuno rende falso un file
committato, e due sono per il **compito 9**:

- **compito 3** (`task-3-review-1.md`): ADR-0001, riga 12 del rimando, *«come il kernel lo è in quattro»* è una cifra in prosa senza comando, verbatim dal disegno §5.1 (*plan-mandated*); la §1 del compendio dice «voce e gesti» senza puntatore mentre `CLAUDE.md` porta *«(ADR-0001, col rimando datato in testa)»*; il rimando di ADR-0011 restringe la cella *«evento, che può avviare una run»* e sta sessanta righe sopra — attrito, non contraddizione; ⛔ **per il compito 9:** il disegno **§3.2** dice *«La forma è quella di AUD-032: il richiamo va in testa all'ADR»* — la **forma** regge, la **posizione** attribuita ad AUD-032 è falsa (E4: sta dentro la Decision di ADR-0005), e §3.2 è approvata senza richiamo datato.
- **compito 4** (`task-4-review-1.md`): in `tracciabilita.md` la riga della telecamera cita `§6.10` della spec del **sotto-progetto 1** mentre gli altri `§N` del file sono della spec del kernel — convenzione già mista, `(ADR-0039)` accanto disambigua; le due righe 🔶 nuove citano gli ADR in due stili (link e in chiaro).
- **compito 5** (`task-5-review-1.md`): solo nel **rapporto** dell'esecutore, non nel repository, un blocco detto «integrale» che non lo è; `riferimenti.md` ricopia la misura di `py -0` che vive in **P-4** (seconda casa, attenuata dal comando e dal rimando); ⛔ **per il compito 9:** la riga **F4** porta *«il modello `hand_landmarker.task` in float16»*, proposizione che la §7/§6.2 del disegno non ha (viene da P-7) — o entra nel disegno con richiamo datato, o esce da `riferimenti.md`.

**Il compito della sessione successiva: il compito 6, poi il 7.** In ordine, e ogni riga è
eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -6`: si parte da `main`,
   e la testa deve essere il commit di questa sezione o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e la testa
   dell'audit del 2026-08-27 — poi **questo piano fino alla mappa dei file**, l'errata per intero
   (E1–E9), questa sezione, e il disegno per intero.
3. `replace_unique.py` riestratto nello scratchpad (`sed -n '35,70p'` di questo file, identico al
   testo); se il ledger c'è, il percorso corretto in `task-6-dispatch.md`, `task-7-dispatch.md` e
   `task-6-review-dispatch.md`; se non c'è, brief e dispacci ricostruiti dal piano.
4. **Il compito 6** con `superpowers:subagent-driven-development`: le quattro domande di
   `CLAUDE.md` contro il codice **di allora** (il pre-controllo di questa sessione è nella
   decisione 4 qui sopra e nelle voci E8/E9); un solo file di `crates/` committato — il banco —
   e `crates/kernel/src/arbiter/mod.rs` **mutato e ripristinato byte-esatto**, mai committato; il
   capoverso del doc di modulo entra dopo la riga unica *«`assert_ne!` on it directly is fine and
   is what the first probe uses.»* e prima di `use kernel::arbiter::{`; `cargo fmt --all --check`
   alla fine e **mai `cargo fmt`** (normalizzerebbe i CRLF del banco). Il revisore **rilancia le
   due mutazioni** e ripristina con `cmp`; le righe `test result` misurate vanno nel registro al
   posto dei segnaposto.
5. **Il compito 7**: due commit (D6), il protocollo **prima** del codice; i sette file nuovi nascono
   **LF** col tool Write, `.gitignore` con l'aiutante (CRLF, Trova `/spikes/gui-ipc/Cargo.lock`
   unico); `cargo build --release` **solo** dentro `spikes/gesti/relay`, dopo `.gitignore`;
   `git status --porcelain -uall` per vedere i file nuovi uno per uno.
6. Il **compito 8 solo col proprietario** e la telecamera; il **compito 9** chiude, muove la §6 del
   compendio al brainstorming della knowledge base, e porta al disegno i due Minor marcati «per il
   compito 9» — come richiami datati o come voci d'errata, a scelta del proprietario.

📌 **Trovato eseguendo, e non promosso a gotcha:** i due difetti d'errata di questa sessione (E8,
E9) sono la stessa specie dei sette precedenti — un *Trova* scritto senza contarne le occorrenze —
e il rimedio che ha retto è sempre quello del vincolo 8: **ogni conteggio si rifà col comando**,
compreso `grep -c` sul testo che un compito dice di sostituire. Le tre revisioni hanno trovato
solo Minor: il codice e i testi dettati reggono alla prima passata, il pre-controllo no.

⛔ **Vicoli ciechi di questa sessione: nessuno.** Nessuna decisione del disegno o del piano è
stata riaperta.
