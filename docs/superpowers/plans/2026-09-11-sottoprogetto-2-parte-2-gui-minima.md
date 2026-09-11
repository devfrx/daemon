# Sotto-progetto 2 — parte 2, la GUI minima: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario (punto 7 della §10 del [disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md),
> decisione 48 della stella polare). I passi usano le caselle (`- [ ]`) per il tracciamento.
> ⛔ **Il pre-controllo di ogni compito è fatto nella sessione che scrive il piano, il 2026-09-11,
> contro il repository a `42b50d8`; l'esecuzione va in una sessione NUOVA.**

**Obiettivo.** Eseguire i **pezzi 2–9** della §3 della [stella polare della GUI](../specs/2026-09-07-direzione-gui-design.md) —
il filo, lo schema che cresce, il registro delle funzioni, la settima porta, il daemon che ascolta,
la SPA con la cornice, il core finto, il passo del cancello — più le due voci di CI decise dal
proprietario il 2026-09-09: **X-1**, la CI anche su Windows (decisione 44), e **X-3**, `cargo audit`
in `gate.sh` e `npm audit` in `gate-gui.sh` (decisione 45); e le righe nella §12 del compendio, in
`README.md`, nella roadmap e in tracciabilità (D14 della parte 1). ⛔ **Il pezzo 1 — SP-8, lo spike del
guscio — È ESEGUITO il 2026-09-10**: il guscio è **Electron**, [ADR-0029](../../adr/0029-guscio-della-gui.md)
è `Accepted`, `dockview` resta. Questo piano parte **coi numeri in mano** e non li rimisura.

**Forma.** Compiti in sequenza, e quanti siano lo dice la tabella della posizione qui sotto. Le decisioni
sono già prese dai due disegni; questo piano le traduce in passi, e le poche che aggiunge stanno nella
tabella *«Le decisioni prese da questo piano»*. ⚠️ **Questo piano tocca il codice di prodotto**, al
contrario della parte 1: `crates/`, `scripts/`, `.github/`, `Cargo.toml` e `Cargo.lock` cambiano, e
nasce `gui/` alla radice, fuori dal workspace.

**Strumenti.** `bash`, `awk`, `grep`, `sed -n` in lettura; **Python 3** per ogni scrittura su un file
**CRLF**, con l'aiutante qui sotto; `cargo` 1.95.0 dalla toolchain appuntata; `node` e `npm` nelle
versioni misurate dal compito che apre `gui/`; `git`. La porta di qualità è `bash scripts/gate.sh`, e
deve stampare `GATE GREEN` **prima di ogni commit**, anche di soli documenti; `bash scripts/check-docs.sh`
deve stampare `OK`.

⛔ **L'aiutante `replace_unique.py` vive nello scratchpad, mai nel repository.** È lo stesso dei piani dei
gesti, della knowledge base e della parte 1, riportato qui perché un piano porta i propri attrezzi nel
testo. Sostituisce **una** occorrenza unica, conserva i fine-riga del file, e rifiuta se il testo vecchio
manca o non è unico. I testi *Trova* e *Sostituisci con* di ogni compito si mettono in due file e si
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

⚠️ **I *Trova* lunghi si prendono DAL FILE, non da questo piano:** dove un passo dice *«la riga che
comincia con …, intera, presa dal file»*, si copia con `grep` o `sed -n` e si mette nel file `old.txt`
così com'è. Un piano che ricopia una riga di tabella di 900 caratteri la ricopia sbagliata (voce E6 del
piano dei gesti).

**Disegni:** la [stella polare](../specs/2026-09-07-direzione-gui-design.md) — le §1, §2, §3, «Il modello
della GUI», «La GUI dentro» con le tre sequenze, i vicoli ciechi — e il
[disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) — le §3, §4, §5, §6a, §7, §8,
§9, i vicoli ciechi. Si leggono **prima** dei compiti, nelle sezioni che ciascun compito nomina.
⛔ **Della parte 1 si leggono tre pezzi soli** — *«Le decisioni prese da questo piano»* (D13 e D14 sono
ereditate qui), *«Le voci aperte che questo piano SA, e non chiude»* e l'**ottava chiusura** del diario:
il resto riguarda l'esecuzione di SP-8, che è finita, e si apre **a domanda**.

---

## Vincoli globali

Valgono per ogni compito, senza che il compito li ripeta.

| # | Vincolo | Da |
|---|---|---|
| 1 | **le due spec si toccano SOLO coi richiami datati che i compiti nominano**: la §2.3 e la §3.1 della spec del sotto-progetto 1 passano da sei a sette famiglie (compito 4). Nessun'altra riga: `git diff --name-only 42b50d8..HEAD -- docs/superpowers/specs/2026-08-06-kernel-design.md` resta vuoto a ogni compito | §3 della stella polare, decisione 15; `CLAUDE.md` |
| 2 | **codice in inglese, documenti in italiano**: Rust, TypeScript, Vue, JavaScript, shell, YAML e i loro commenti in inglese; i verbali, i richiami e questo piano in italiano; un riferimento al codice dentro un documento porta il **nome esatto del sorgente** | §1.0 della spec; gotcha #40 |
| 3 | **nessuna cifra nuova in prosa**: date e comandi; una cifra che sostiene una decisione porta accanto il **comando** e la **data** e vive in **una** casa | `CLAUDE.md` |
| 4 | **i fine-riga si conservano per file** e si rimisurano dopo ogni scrittura: la mappa dei file dice CRLF o LF **oggi**; `git ls-files --eol` prima e dopo, **invariato**; i file nuovi nascono **LF**; un CRLF si tocca con `replace_unique.py` o con Python `newline=""`, **mai** con `sed -i` (che in questa Git Bash toglie i CR). ⛔ **`crates/kernel/tests/ports_are_implementable.rs` è CRLF anche NELL'INDICE** — unico fra i sorgenti (P-3): chi lo tocca lo rimisura con `git ls-files --eol`, non solo con `tr -cd '\r'` | `CLAUDE.md`; P-3 |
| 5 | **ogni conteggio si rifà col comando** prima di leggere che cosa un documento ne dice; le cifre di questo piano sono istantanee del 2026-09-11 su `42b50d8` | disegni, *Metodo* |
| 6 | ⛔ **una dipendenza si aggiunge in DUE passi**: il manifesto e il `Cargo.lock` **insieme**, e il lockfile si rinfresca **fuori** dal cancello con un `cargo build` **senza** `--locked`. Toccare un manifesto da solo lascia il cancello rosso | `CLAUDE.md`, finding G-5; ADR-0031 |
| 7 | **i lockfile si committano**: `Cargo.lock` di radice, `gui/fake-core/Cargo.lock`, `gui/package-lock.json`; nel cancello gira `npm ci`, gemello di `--locked` | §8 del 2, decisione 49 |
| 8 | **le versioni sono appuntate** come misurate il 2026-09-11 (P-2); il compito che le installa **rilancia** il comando della §9 del 2 e scrive nel proprio commit quelle del suo giorno; una **major** nuova non si prende («novità non è maturità»); una minor o patch nuova solo se l'appuntata non si installa, con voce d'errata | §9 del 2; `CLAUDE.md` |
| 9 | ⛔ **`kernel` e `simulator` restano `no_std` + `alloc` + `forbid(unsafe_code)`, e la loro lista di dipendenze NON cresce**: `bash scripts/gate-deps.sh` verde a ogni compito. Tutto ciò che tocca l'OS vive in `platform`, `secrets` o `daemon` | ADR-0031; vincolo 2 di §11 |
| 10 | **gli ADR sono append-only**: nessun ADR nuovo in questo piano — la settima porta non ne vuole uno (decisione 19 della stella polare); ciò che diventa falso riceve un **richiamo datato** accanto | `CLAUDE.md` |
| 11 | **il compendio resta sotto il tetto**: margine misurato prima e dopo ogni tocco col comando qui sotto (il 2026-09-11: `11030`, P-6); se va rosso si toglie prosa dalla §6, **non si alza il tetto** | §13 del compendio, gotcha #100 |
| 12 | **nessun link `](…)` a un file che non esiste ancora**: si nomina in code span, e il link nasce nel commit che crea il file. I piani sono fuori dal controllo dei link (P-7) | trappola 5 di `check-docs.sh` |
| 13 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore**; il cancello e `check-docs.sh` girano **prima**; il messaggio comincia con `gui(compito N…)` | `CLAUDE.md` |
| 14 | **l'accettazione condizionata del proprietario vale finché regge**: se un passo violasse uno dei cinque criteri di `anthropic-skills:decision-principles` — richiedesse una scorciatoia, duplicasse qualcosa che esiste, poggiasse su qualcosa che non è più vero — ci si **ferma e lo si riporta** | teste dei disegni |
| 15 | ⛔ **niente `#[allow]` nuovi e nessun `unsafe`**: `bash scripts/gate-attributes.sh` verde a ogni compito | vincolo 2 di §11 |

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

⏳ **IL PIANO È IN SCRITTURA dal 2026-09-11.** Il pre-controllo delle quattro domande sta nella sezione
*«Il pre-controllo del piano»* qui sotto. ⛔ **Nessun compito è ancora eseguito.**

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | il **contatore condiviso**: un tipo nuovo di `kernel`, seminato dal giornale con `replay`, consegnato a chi ne ha bisogno; i richiami datati in `ports/journal.rs` e `ports/ipc.rs` | uno | ⬜ |
| **2** | il **trasporto `ipc`** in `platform` su `interprocess`, e la **suite di conformità** `ipc_contract.rs` inclusa da `platform` coi bugiardi | uno | ⬜ |
| **3** | lo **schema che cresce**: le varianti nuove di `IpcMessage`, le **fixture** e il **timbro di build**, `ipc_wire.rs` | uno | ⬜ |
| **4** | la **settima porta**: il tratto `Custody` in `kernel::ports`, la finta di `ports_are_implementable.rs`, la suite di conformità; i richiami alla §2.3 e alla §3.1 della spec e alle cifre in prosa di `ports/mod.rs` | uno | ⬜ |
| **5** | le **due implementazioni** della settima porta: `redb` in `platform`, la finta in `simulator` | uno | ⬜ |
| **6** | il **registro delle funzioni** `kernel::registry`: la funzione registrata, `invoke`, il dettaglio `Invocation` col suo record congelato | uno | ⬜ |
| **7** | l'**attività del kernel che ascolta**: il dispaccio, il ramo `Request` **non servito** (D5), il limite di giri, `Disconnected` | uno | ⬜ |
| **8** | il **daemon**: il cablaggio dell'attività, il percorso dell'archivio come argomento, la **rilettura della policy all'avvio** col dettaglio tipizzato, «salva, riavvia, ritrova» | uno | ⬜ |
| **9** | la **campagna DST del 2** in `simulator`, e la sua riga nel settimo passo del cancello | uno | ⬜ |
| **10** | **`gui/` nasce**: Vite, Vue 3, TypeScript, `engines.node` e `.npmrc`; `schema/` coi tipi e le fixture, `transport/` col ponte e la sua finta | uno | ⬜ |
| **11** | il **core finto** `gui/fake-core/`: l'attività vera su porte in memoria, il rubinetto, le sonde | uno | ⬜ |
| **12** | la **SPA, la cornice**: `dockview`, la barra delle viste, la fascia, la striscia, il cassetto, le tre viste come JSON, `stores/`, `tokens/`, `locales/it.json` | uno | ⬜ |
| **13** | la **SPA, i moduli**: Stato, Permessi con la finestra di conferma, Chat col markdown e la provenienza, Passi, il segnaposto; l'accessibilità e le scorciatoie sopra `moveTo` | uno | ⬜ |
| **14** | il **passo del cancello**: `scripts/gate-gui.sh`, la riga in `gate.sh`, `.gitignore`, `actions/setup-node` nella CI | uno | ⬜ |
| **15** | **X-1 e X-3**: la matrice Windows nella CI, `cargo audit` in `gate.sh`, `npm audit` in `gate-gui.sh` | uno | ⬜ |
| **16** | la **chiusura**: i documenti in ogni casa — la §12 del compendio, `README.md`, la roadmap, tracciabilità, `HANDOFF.md`, `porta-di-qualita.md`, `riferimenti.md` — e la Definizione di «fatto» della parte 2, coi comandi | uno | ⬜ |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del
[`COMPENDIO.md`](../../COMPENDIO.md), in un posto solo. Ciò che resta qui è la **posizione** del piano —
la tabella qui sopra, che chi esegue aggiorna nel commit del compito — e **come** si esegue.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro — e i disegni nelle
   sezioni che il compito nomina.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-11.
3. Se il compito dice il falso, **ci si ferma e si riporta**: non si aggira. Una divergenza è una voce
   d'errata prima di essere un rimedio. ⚠️ Vale anche per il codice dettato: un `cargo build` o un
   `npm run build` rosso sul testo dettato è una voce d'errata col testo corretto, **non** un
   aggiustamento silenzioso.
4. Il cancello gira **prima** di ogni commit; il commit dice ciò che il compito ha fatto.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; per i compiti
   12 e 13 apre la SPA nel browser contro il ponte finto e **guarda**, perché un `npm run build` verde
   non prova che un pannello si veda.
6. Una seconda ondata di **sola prosa** la fa il coordinatore a mano, senza ri-revisione; se tocca un
   fatto o un comando, si ri-rivede. Dopo due ondate di prosa si chiude (gotcha #76).
7. ⛔ **Le scritture in parallelo non si fanno**: un compito per volta, sequenziale. Il parallelismo di
   questo repository è servito a **leggere**.

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota, e non resterà vuota.** Il pre-controllo ha trovato un difetto reale in **tutti** i
compiti dispacciati finora, senza una sola eccezione: quando ne trovi uno, si scrive **qui**, con il
proprio numero, prima di eseguirlo. Un piano è un'ipotesi.

| # | Voce |
|---|---|
| — | *(vuota alla scrittura del piano)* |

---

## Il pre-controllo del piano — che cosa i disegni dicono e il repository smentisce o precisa

Letto scrivendo, contro il repository a `42b50d8` e alle fonti primarie, il 2026-09-11. Ogni voce è una
delle quattro domande di `CLAUDE.md`, o una delle quattro righe che l'elenco non coglie.

### P-1 — `admit` vuole un `ResourceProfile` e il filo porta un `GrantRequest`: la §5 del 2 dice una cosa che il codice non lascia fare senza scrivere altro

**Domanda 6 — ciò che ti smentisce sta in un COMMENTO.** La riga `Request` della tabella del daemon
della §5 del 2 dice *«`admit` → `Verdict`»*. Ma `Arbiter::admit` prende `&ResourceProfile`
(`crates/kernel/src/arbiter/mod.rs:625`), e sul filo viaggia `GrantRequest`
(`crates/kernel/src/wire/ipc.rs:103`), che ha tre campi e **non** ha `name: &'static str`. I due tipi
non coincidono per costruzione, e il doc di `GrantRequest` scrive perché, in tre punti che nessuno dei
due disegni riporta:

- *«IT IS NOT A `ResourceProfile`, AND THE MISSING FIELD IS THE REASON»* — un nome prodotto dai byte
  sarebbe testo **scelto dalla GUI**, cioè contenuto non fidato (ADR-0014) dentro un tipo con cui
  l'arbitro **decide**;
- *«`compute_class` AND `preemption` REACH THE ARBITER FROM THE SAME PEER AS THE NAME THIS TYPE REFUSES,
  AND THE ARBITER OBEYS THEM … NEITHER IS CHECKED AGAINST ANYTHING»* — letto in `crate::arbiter`:
  `admit` passa `profile.compute_class` ad `ask_back` come `below`, e la chiusura `askable` scarta ogni
  titolare con `held.lane <= below`, quindi una richiesta che dichiara `Realtime` apre quella guardia
  **su tutti**; `issue` mappa `Preemption::Never` su `Activity::NonPreemptible`, quindi una richiesta che
  dichiara `Never` prenota una concessione che `ask_back` non può mai scegliere;
- *«THE TRIGGER IS THAT CONSUMER: the task that first decodes arriving bytes into a profile is where the
  verifying half is written and where its probe is born.»*

È la stessa voce che `porta-di-qualita.md` tiene aperta come riga **27** del Traguardo 6, *«l'altra metà
di ADR-0005»*, col chiusore **«il compito che per primo decodificherà byte in un `ResourceProfile` — non
il proprietario»**.

**Conseguenza per questo piano:** il compito 7 **non** costruisce un profilo, e la decisione è **D5**.

### P-2 — Le versioni del 2026-09-11, e quali si appuntano

Rilanciato il comando della §9 del 2 (registro npm e `crates.io`), più i pacchetti della §6a e della §2
che quel comando non elenca. `latest`, data, licenza:

| Pacchetto | Oggi | Nel disegno / nello spike | Si appunta |
|---|---|---|---|
| `vue` | 3.5.42 · 2026-08-27 · MIT | — | **3.5.42** |
| `vite` | 8.3.0 · 2026-09-10 · MIT | — | **8.3.0** |
| `pinia` | 4.0.3 · 2026-08-12 · MIT | — | **4.0.3** |
| `reka-ui` | 2.10.4 · 2026-08-25 · MIT | — | **2.10.4** |
| `vue-i18n` | 11.4.10 · 2026-08-25 · MIT | — | **11.4.10** |
| `dockview-core` | 8.3.1 · 2026-09-10 · MIT | **8.2.0** in SP-8 | **8.3.1** — D2 |
| `dockview` | 8.3.1 · 2026-09-10 · MIT | 8.2.0 (solo per il CSS, E2 della parte 1) | **8.3.1** — D2 |
| `typescript` | 7.0.2 · 2026-07-08 · Apache-2.0 | — | **7.0.2** |
| `vue-tsc` | 3.3.11 · 2026-08-21 · MIT | 3.3.11 | **3.3.11** |
| `markdown-it` | 15.0.2 · **2026-09-11** · MIT | 15.0.1 | **15.0.2** — D3 |
| `vitest` | 5.0.0 · 2026-09-03; tag `V4` → 4.1.11 · 2026-08-18 | 4.1.11 (decisione 52) | **4.1.11** — D4 |
| `@vue/test-utils` | 2.5.0 · 2026-08-27 · MIT | 2.5.0 | **2.5.0** |
| `jsdom` | 30.0.1 · 2026-07-29 · MIT | 30.0.1 | **30.0.1** |
| `axe-core` | 4.13.0 · 2026-08-05 · MPL-2.0 | 4.13.0 | **4.13.0** |
| `eslint` | 10.10.0 · 2026-09-04 · MIT | 10.10.0 | **10.10.0** |
| `eslint-plugin-vue` | 10.11.0 · 2026-09-06 · MIT | 10.11.0 | **10.11.0** |
| `@intlify/eslint-plugin-vue-i18n` | 4.5.1 · 2026-06-02 · MIT | 4.5.1 | **4.5.1** |
| `interprocess` (crate) | 2.4.4 · 2026-09-03 | 2.4.4 (risposta *trasporto*) | **2.4.4** |
| `redb` (crate) | 4.2.0 · 2026-08-17 | **4.1.0** in ADR-0032 e nel `Cargo.lock` | **4.1.0**, quella del lockfile — D6 |

⚠️ **`@playwright/test` e `@axe-core/playwright` non si installano in questo piano:** il capo a capo è
dichiarato fuori dal cancello di oggi dalla §8 del 2 («dopo lo spike, parte 2 … fuori dal cancello,
dichiarato»), e installarli senza una prova che li usi sarebbe una dipendenza senza consumatore.

### P-3 — `ports_are_implementable.rs` è CRLF anche NELL'INDICE, unico fra i sorgenti

`git ls-files --eol` il 2026-09-11 rende `i/crlf w/crlf` per
`crates/kernel/tests/ports_are_implementable.rs`, e `i/lf w/crlf` per ogni altro sorgente e documento
misurato. Il compito 4 tocca quel file per aggiungere la finta della settima porta: se lo riscrive con
LF, `git diff` dichiara cambiate tutte le sue righe. **Si rimisura con `git ls-files --eol`, non solo
con `tr -cd '\r'`**, che sull'albero di lavoro non distingue i due casi.

### P-4 — I record congelati sono SEI, e il settimo è `Invocation`

`ls crates/kernel/tests/frozen/` il 2026-09-11: `record_v1.map`, `record_v1_intent.cbor`,
`record_v1_note.cbor`, `record_v1_outcome.cbor`, `record_v1_permission.cbor`, `record_v1_routing.cbor`,
`record_v1_verdict.cbor` — **sei** `.cbor` più la mappa. La §5 del 2 lo dice già col richiamo del
2026-09-08 («i record congelati sono **sei**, non quattro … con `Invocation` diventano **sette**»), e la
riga è vera oggi. ⛔ **I sei vecchi restano identici al byte**: se cambiano non è un aggiornamento, è un
cambio di formato (ADR-0036).

### P-5 — Le cifre in prosa di `ports/mod.rs`, e quali diventano

`grep -niE "SIX|FIVE|SEVEN" crates/kernel/src/ports/mod.rs` il 2026-09-11 rende sei righe di prosa:
*«The SIX families of ports (§2.3)»* (riga 1), *«all six are named in this milestone»* (7), *«this module
declares SIX submodules»* (22), *«a seventh family»* (32), *«FIVE fakes»* (37), *«the simulator
substitutes SEVEN things while §2.3 enumerates SIX»* (77). Con la settima porta diventano **sette
famiglie**, **sei finte**, **otto cose sostituite**. ⚠️ La riga 43 porta già un **richiamo datato del
2026-08-28** (finding AUD-054) sul `FIVE`: il compito 4 lo **legge prima** di toccarlo, e il richiamo
nuovo si aggiunge senza cancellare il vecchio.

### P-6 — Il margine del compendio è `11030` byte

Misurato il 2026-09-11 col comando del vincolo 11. Il compito 16 lo consuma: il compendio riceve le
righe della §12 e il puntatore della §6. Si rimisura **prima e dopo**.

### P-7 — `check-docs.sh` esclude i piani dal controllo dei link, e legge `spikes/*.md`

Verificato il 2026-09-11: `bash scripts/check-docs.sh` rende `OK` col piano della parte 1 in albero, che
porta link a file dello scratchpad mai esistiti. Un link rotto in **questo** file non è un rosso; un link
rotto in un documento di `docs/` o in `spikes/*.md` sì.

### P-8 — Dove va la riga `run` in `gate.sh`, verificato per numero di riga

`grep -n '^run ' scripts/gate.sh` il 2026-09-11: `39` workspace build · `40` example and compile-fail
tests · `41` no-OS gate · `42` allow-list on the two graphs · `43` attributes of the constrained crates ·
`44` documentation consistency · `84` DST campaigns. La §8 del 2 vuole la riga nuova **fra la 43 e la
44**; `cargo audit` di X-3 è un'altra riga, e il compito 15 dice dove.

### P-9 — Lo stato di `.gitignore` e della CI, oggi

`.gitignore` ignora già `/target/`, i `Cargo.lock` degli spike e le cartelle di build di `spikes/gui-shell/`;
**non** nomina `gui/`. La CI (`.github/workflows/quality-gate.yml`) ha un lavoro solo, `gate`, su
`ubuntu-latest`, con `actions/checkout@v4`, `rustup show` e `bash scripts/gate.sh`: **nessun Node**,
nessuna matrice. Il commento sopra `rustup show` spiega perché il vincolo 4 di §11 non vuole un passo in
più — il compito 15 lo **legge prima** di aggiungere la matrice.

### P-10 — Niente di ciò che il piano detta esiste già

Misurato il 2026-09-11: `gui/`, `crates/platform/src/ipc.rs`, `crates/kernel/src/registry`,
`crates/kernel/src/ports/custody.rs` e `scripts/gate-gui.sh` **non esistono**; `ls crates/kernel/src/ports/`
rende sei file più `mod.rs`; `IpcMessage` ha **due** varianti, `Request` e `Verdict`; il manifesto di
radice porta `exclude = ["spikes"]` e **non** `gui`.

### P-11 — Il rubinetto del core finto costruisce la richiesta in Rust, e per questo NON innesca la voce 27

La §7 del 2 dice che alla parola `verdict` il rubinetto *«chiede all'arbitro vero un'ammissione con una
richiesta di comodo e manda alla GUI il `Verdict` che ne esce»*. Una richiesta **di comodo**, costruita
nel sorgente del finto, non è *«byte in arrivo decodificati in un profilo»*: l'innesco della riga 27 del
Traguardo 6 resta intatto. ✅ **È anche la ragione per cui D5 non toglie nulla alla GUI del 2**: la riga 4
del modulo Stato — *«una riga di evento per l'ultimo `Verdict`»* — si esercita col rubinetto, come la §8
del 2 già prescrive.

### P-12 — `Verdict::Refused` ha un significato preciso, e non è «non servito»

`crates/kernel/src/wire/ipc.rs:133`: `Refused { asked: Mib, ceiling: Mib }`, e il doc di
`Admission::Refused` dice *«It does not fit and it never will under this budget»*. Rispondere `Refused` a
una richiesta che **starebbe** nel budget sarebbe una falsità scritta dentro un tipo: è la via che D5 ha
esaminato e scartato, e la ragione sta lì.

### P-13 — `unframe` NON è un lettore di flusso, e il suo doc chiede la cura: una funzione NUOVA accanto

**Domanda 6 — ciò che ti smentisce sta in un COMMENTO**, la seconda volta in questo pre-controllo. La
riga `receive` della §3 del 2 dice *«lettura non bloccante nel buffer; cornice intera → `Ok(Some(corpo))`;
parziale → `Ok(None)`»*, e chi la legge scrive `framing::unframe` sul buffer. Ma la firma è
`unframe(bytes: &[u8]) -> Result<&[u8], WireError>` e **rifiuta ogni coda**: un buffer con una cornice e
mezza — il caso **ordinario** di un flusso, non quello limite — rende `WireError::TrailingBytes`. Il doc
di `unframe` lo dice di sé e scrive la cura:

> ⛔ AND IT IS NOT A STREAM READER, WHICH IS A DECLARED LIMIT AND NOT AN OVERSIGHT … ⚠️ A REAL TRANSPORT
> WILL WANT A SECOND ENTRY POINT, one that answers «this frame, and where the next starts», and it is a
> **NEW function beside this one rather than a loosening of this one** — `TrailingBytes` is the whole of
> what `a_frame_with_a_tail_is_refused` holds. Written here so the transport does not rediscover it.

⚠️ **E `frame` rende un `Result`**, non un `Vec<u8>`: `frame(body) -> Result<Vec<u8>, WireError>`, con
`WireError::TooLong` sopra `MAX_BODY_LEN`. **Conseguenza:** il compito 2 aggiunge `declared_len` e
`take_frame` **accanto** a `unframe`, con le loro sonde e la **seconda direzione** — che
`a_frame_with_a_tail_is_refused` resti verde.

### P-14 — `IpcError::MalformedMessage` non ha un produttore possibile con la busta com'è

La §3 del 2 lo pretende — *«cornice rotta → `MalformedMessage` e il client resta»* — ma la busta è un
**prefisso di lunghezza**: qualunque quaterna di byte è una lunghezza valida, e `MAX_BODY_LEN` è
`u32::MAX as usize`, che ogni `u32` soddisfa. Nessun difetto è rilevabile a livello di busta; ciò che non
decodifica lo dice lo **schema**, con `WireError::Malformed`, e lo schema non vive nel trasporto.
⚠️ **E la regola «nessun chiamante, nessuna voce» ora RAGGIUNGE quelle varianti**, che ne erano esenti:
il doc di `IpcError` scrive *«the port has no implementation, so NO variant has a producer»*, e dal
compito 2 una ce l'ha. **Conseguenza:** **D9**, il tetto consegnato.

### P-15 — `IpcMessage::encode` INCORNICIA GIA', e le due finte della porta passano i byte VERBATIM

Letto il 2026-09-11 in `crates/kernel/src/wire/ipc.rs`: `encode` finisce in `framing::frame(&body)` e
`decode` comincia in `framing::unframe(bytes)`. E le due implementazioni che la porta ha oggi — `FakeGui`
in `crates/kernel/tests/ports_are_implementable.rs` e `DyingGui` in `crates/simulator/src/ipc.rs` —
tengono i messaggi in un `Vec<Vec<u8>>` e li rendono **tali e quali**: nessuna delle due incornicia.
⛔ **Un trasporto che incorniciasse a sua volta metterebbe una SECONDA busta sul filo**, darebbe agli
stessi quattro byte due significati, e lascerebbe il pari TypeScript a sbucciarne due. **Conseguenza:**
**D10** — `send` scrive **verbatim**, `receive` rende **la cornice intera**, e la busta si legge solo per
trovare dove un messaggio finisce e comincia il prossimo: l'unica cosa che un flusso di byte non porta e
che una finta a `Vec<Vec<u8>>` ha per costruzione. ⚠️ **Il doc della porta non lo dice**, e non è una
svista da correggere qui: il compito 2 lo scrive nel doc del **trasporto**, che è chi lo deve sapere.

---

## Le decisioni prese da questo piano

⛔ **Sono decisioni del piano, non dei disegni, e chi esegue può ribaltarle** portando la misura che le
smentisce — è ciò per cui esiste l'errata.

| | Decisione | Perché |
|---|---|---|
| **D1** | i compiti sono **sedici**, tagliati per **artefatto** sulla tabella *«Il prodotto del 2, e il controllo che esercita ciascun artefatto»* della §8 del 2, e ogni compito finisce con un artefatto provato da solo | la §8 dice di sé che le righe stanno «in un posto solo perché il piano le tagli per compito»; il precedente sono i tredici compiti del Traguardo 5 in un piano solo |
| **D2** | `dockview-core` e `dockview` si appuntano a **8.3.1**, non alla **8.2.0** con cui SP-8 ha misurato le otto mosse | la v8 è additiva e ogni novità è opt-in, letto alla fonte il 2026-09-07 (tabella della §4 della stella polare); appuntare una versione che il registro non serve più come `latest` è debito al primo `npm install`. ⚠️ **Costo dichiarato:** l'evidenza delle otto mosse è sulla 8.2.0, e il compito 12 lo scrive accanto al primo uso; se un comportamento delle mosse cambia, è una voce d'errata |
| **D3** | `markdown-it` si appunta a **15.0.2**, uscita **il giorno stesso** | è una patch sulla 15.0.1 che la §9 del 2 aveva letto **dentro il pacchetto** (preset `default` con `html: false`, `BAD_PROTO_RE`); ⛔ **il compito 13 rilegge quelle tre proprietà dentro il `.tgz` della 15.0.2 prima di usarla**, perché una patch che tocca `validateLink` cambierebbe la ragione della decisione 51 |
| **D4** | `vitest` resta **4.1.11**, il tag `V4`, e non la 5.0.0 | la decisione 52 disse «sei giorni»; oggi sono otto, e otto giorni non sono maturità. ⛔ **La misura si rifà al compito 10**, e se il piano si scrivesse fra un mese la risposta cambierebbe: la regola è «novità non è maturità», non «mai la major» |
| **D5** | ⛔ **il ramo `Request` del dispaccio NON costruisce nessun `ResourceProfile`**: non chiama `admit`, non risponde, e il doc del ramo scrive perché. La riga `Request` della §5 del 2 riceve un **richiamo datato** al compito 7. La riga 27 delle voci aperte del Traguardo 6 resta aperta **col suo innesco intatto**, e il chiusore resta il **7**, il pilastro 3D | le tre vie sono state esaminate contro il codice (P-1, P-11, P-12). **Servirla** obbliga il core a nominare corsia e prelazionabilità per un consumatore che non esiste — `ComputeClass::Batch` è documentata *«3D render, indexing, background runs»* e ADR-0033 descrive un **viewer**: nessuna riga dice quale sia giusta, e sceglierne una è una deduzione presentata come disegno. **Rifiutarla** con `Verdict::Refused` mente sul significato del tipo (P-12). **Non servirla** non afferma nulla di falso, è la forma che il repo usa già — `promote` non si chiama, quattro porte su sei non hanno chiamanti, *«dichiarato, non pinzato»*, gotcha #73 — e toglie alla radice il privilegio non controllato, perché nessun valore del pari raggiunge l'arbitro. ⚠️ **Costo dichiarato:** la §5 del 2 si restringe, e la GUI del 2 vede un `Verdict` solo dal rubinetto del core finto (P-11) |
| **D6** | `redb` resta alla **4.1.0** del `Cargo.lock`, benché il registro serva la 4.2.0 | ADR-0032 nomina la 4.1.0 e il lockfile è un **ingresso** del cancello (vincolo 6): alzarla è un atto deliberato che non serve a nessun passo di questo piano. Il compito 5 usa il `FileBackend` che `platform` già ha |
| **D7** | la **seminatura** del contatore vive in `kernel::numbering::seeded_from`, non in `daemon` | la §7 del 2 costruisce l'attività del kernel **da fuori**, in `gui/fake-core`, che non può importare un binario — è il dedotto che quella sezione scrive di `build_the_arbiter`. Una seminatura in `daemon/src/main.rs` sarebbe **copiata** dal finto, e una copia del cablaggio è verde il giorno che le due divergono. ⚠️ **Costo:** `kernel` guadagna una funzione libera che legge il giornale con `replay`; **nessuna** operazione nuova nella porta |
| **D8** | il modulo si chiama **`numbering`** e non `counter` | in questo repository «counter» significa già **contro-sonda**: `crates/platform/tests/counter_probes.rs` si apre con *«The counter-probes of §7.1.1 rule 3»* e `platform::counter_probe_std_compiles` porta la stessa parola. Un `kernel::counter` numerico accanto a quelle è la specie di ambiguità che questo repo paga altrove. ⚠️ **Costo:** il doc di `ClientId` dice «the counter», e il richiamo del compito 1 nomina il tipo per esteso |
| **D9** | ⛔ **il trasporto riceve un TETTO consegnato** — `LocalSocketIpc::bound(name, numbers, max_body)` — e una lunghezza dichiarata sopra il tetto è `MalformedMessage` **col client che resta**; il flusso diventa **avvelenato** e ogni `receive` successivo dice lo stesso | è l'unico produttore possibile di quella variante (P-14), e senza di esso la riga `receive` della §3 del 2 prometterebbe qualcosa che nessuna sonda può tenere. Consegnato e non inventato (ADR-0034), come il contatore. ⚠️ **E toglie un buffer senza fondo** da un componente che parla con l'OS: un pari che dichiara quattro gibibyte verrebbe accumulato per sempre. ⚠️ **Costo dichiarato:** un flusso avvelenato non si riprende — un flusso con prefisso di lunghezza **non si risincronizza**, e fingere di poterlo fare sarebbe la pezza; chi vuole riprendersi chiude e riapre il collegamento, che è ciò che la GUI fa già quando il core manca (§6a, «riprova»). ⛔ **Chi sceglie il numero non è questo piano:** lo consegnano il daemon (compito 8) e il core finto (compito 11), e il compito 8 dice da dove |
| **D10** | ⛔ **il trasporto NON incornicia**: `send` scrive i byte **verbatim**, `receive` rende **la cornice intera** — busta compresa — e `take_frame` serve solo a trovare il confine fra due messaggi | `IpcMessage::encode` incornicia già e `decode` sbuccia (P-15), quindi un messaggio consegnato a `send` è **già auto-delimitato**: una seconda busta sarebbe quattro byte con due significati e due sbucciature al pari TypeScript. ⚠️ **Ed è la convenzione che le due finte hanno già**, verbatim. **Costo:** chi chiama `send` deve consegnare una cornice intera — il doc del trasporto lo scrive, la porta no (e non si tocca: il suo doc dice «bytes», che resta vero) |

**La baseline di partenza, misurata il 2026-09-11 su `42b50d8` e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `bash scripts/check-docs.sh` → `OK — no inconsistencies.` ·
il comando del vincolo 11 → `11030` · `git status -sb` → `## main...origin/main`, pulito.

---

## Le voci aperte che questo piano SA, e non chiude

⛔ **Lette prima di scrivere, come `CLAUDE.md` prescrive.** Si dichiarano perché chi esegue le sappia.
⚠️ **Nessuna ha come chiusore «il proprietario, PRIMA»**, quindi nessuna sbarra questo piano; le due
righe che lo **toccano** sono segnate.

| Voce | Dove vive | Chi la chiude |
|---|---|---|
| ⚠️ **riga 27 del Traguardo 6** — l'altra metà di ADR-0005, `compute_class` e `preemption` non controllati | [`porta-di-qualita.md`](../../porta-di-qualita.md), e il doc di `GrantRequest` | **il 7**, il pilastro 3D — e questo piano **non** la innesca: **D5** |
| ⚠️ **riga 24 del Traguardo 6** — `reconcile::Resolution` non è decisa da nessun `match` | idem | il primo consumatore; il compito 9 la **asserisce** in una campagna, non la decide con un `match`: resta aperta |
| **E50 / E51 / E100** — `promote` senza chiamante | righe 24 e 25 del Traguardo 5 | chi costruirà il primo ciclo di orchestrazione; la §5 del 2 lo dichiara già |
| **E12** del Traguardo 6 — i due lati della grazia di una revoca | riga 10 del Traguardo 6 | il compito che darà alla revoca un chiamante: **non** questo piano (`grep -rn revoke crates/*/src` rende solo un commento e un banco) |
| il **confine di sessione** dei permessi: `is_granted` rilegge tutto il giornale | voce 7 della §9 del 2 | il **3**, con le run |
| la **prontezza I/O** del reattore, e l'**allocatore** nella porta `journal` | voci 5 e 6 della §9 del 2 | il proprietario, confermate A il 2026-09-09 (decisioni 41 e 42): restano com'è |
| il **watchdog** e lo spegnimento pulito | voce 8 della §9 del 2 | il **10** |
| **AUD-004**, l'ADR del proprietario sulle skill | l'audit; voce 11 della §9 del 2 | il proprietario, in parallelo; sbarra il **13**, non il 2 |
| **X-2** e **X-4** dell'audit | la tabella delle voci senza numero AUD | il **proprietario**; X-1 e X-3 sono il compito 15 |
| la **metà Linux** di M3 e M5 | ADR-0029, come innesco scritto | il primo Linux vero |
| le tre voci del Traguardo 6 che aspettano il primo worker vero — 9, 26, 27 | la tabella del Traguardo 6 | il **12** |
| le **registrate** della stella polare — l'ambito come progetto, «Automazione OS», il grafo del 6, Compatta, i due passi per invocazione | la tabella «Registrate, non prese» | il 3, il 6, il 10, il proprietario |

---

## Compito 1: il contatore condiviso — un tipo di `kernel`, seminato dal giornale

**Files:**
- Create: `crates/kernel/src/numbering.rs` (**LF**)
- Create: `crates/kernel/tests/numbering.rs` (**LF**)
- Modify: `crates/kernel/src/lib.rs` (**CRLF**) — una riga di modulo
- Modify: `crates/kernel/src/ports/journal.rs` (**CRLF**) — il richiamo datato sul doc di `StepId`
- Modify: `crates/kernel/src/ports/ipc.rs` (**CRLF**) — il richiamo datato sul doc di `ClientId`
- Read: la §3 del [disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md), riga «il contatore»; la voce 6 della §9

**Interfaces:**
- Consumes: `kernel::ports::journal::{Journal, JournalError, StepId}` e `StepId::get()`; `simulator::journal::MemoryJournal` nel banco (`simulator` è già una **dev-dependency** di `kernel`, con la sua giustificazione nel manifesto)
- Produces, e i compiti 2, 7, 8 e 11 li usano con questi nomi esatti:
  - `kernel::numbering::Progressive` — `Debug + Clone + Copy + PartialEq + Eq`
  - `Progressive::starting_at(first: u64) -> Progressive` (`const fn`)
  - `Progressive::take(&mut self) -> u64`
  - `kernel::numbering::seeded_from<J: Journal>(journal: &J) -> Result<Progressive, JournalError>`

⛔ **Il nome del modulo è `numbering` e NON `counter` — D8.** In questo repository «counter» significa già
**contro-sonda**: `crates/platform/tests/counter_probes.rs` si apre con *«The counter-probes of §7.1.1 rule
3»*, e `platform::counter_probe_std_compiles` porta la stessa parola. Un `kernel::counter` numerico
accanto a quelle sarebbe la specie di ambiguità che il repo paga altrove.

- [ ] **Passo 1: le misure prima**

```bash
ls crates/kernel/src/numbering.rs 2>&1
grep -rn "pub struct StepId\|pub const fn get" crates/kernel/src/ports/journal.rs
grep -n "pub struct ClientId" crates/kernel/src/ports/ipc.rs
grep -n "next_ticket\|next_grant" crates/kernel/src/arbiter/mod.rs | head -4
git ls-files --eol crates/kernel/src/lib.rs crates/kernel/src/ports/journal.rs crates/kernel/src/ports/ipc.rs
```

Atteso: `numbering.rs` **non esiste**; `StepId(u64)` con `get`; `ClientId(u64)`; i due contatori
dell'arbitro sono campi `u64` incrementati con `+= 1`; i tre file `i/lf w/crlf`.

- [ ] **Passo 2: il banco che fallisce**

`crates/kernel/tests/numbering.rs`, **LF**, nuovo:

```rust
//! The one progressive counter of the core: it hands out ascending numbers, and it starts
//! ABOVE every step the journal already holds.
//!
//! ⛔ THE SECOND DIRECTION IS THE ONE THAT DECIDES HERE, and it is the reason this file is not
//! two asserts. A counter that always started at zero would pass "two numbers differ and
//! ascend" and would still hand a RESTARTED core the numbers it has already written -- which is
//! exactly the defect `ClientId`'s doc names: "two independent counters that look identical are
//! a divergence nothing would report". The seeding probe is that direction.

use kernel::numbering::{seeded_from, Progressive};
use kernel::ports::journal::{Journal, StepId};
use simulator::journal::MemoryJournal;

#[test]
fn it_hands_out_ascending_numbers_from_where_it_was_started() {
    let mut numbers = Progressive::starting_at(7);
    assert_eq!(numbers.take(), 7, "the FIRST number is the one it was started at");
    assert_eq!(numbers.take(), 8);
    assert_eq!(numbers.take(), 9);
}

#[test]
fn an_empty_journal_seeds_it_at_zero() {
    let journal = MemoryJournal::new();
    let mut numbers = seeded_from(&journal).expect("an empty journal replays");
    assert_eq!(numbers.take(), 0);
}

#[test]
fn a_written_journal_seeds_it_above_the_highest_step_it_holds() {
    let mut journal = MemoryJournal::new();
    journal.intent(StepId::new(4), b"intent").expect("intent");
    journal.intent(StepId::new(41), b"intent").expect("intent");
    journal.intent(StepId::new(9), b"intent").expect("intent");

    let mut numbers = seeded_from(&journal).expect("replay");
    assert_eq!(
        numbers.take(),
        42,
        "ABOVE THE HIGHEST, not above the last written: the journal is not sorted by arrival"
    );
}

#[test]
fn a_reopened_journal_never_hands_back_a_number_it_already_holds() {
    // ⛔ THE SHAPE OF THE DEFECT, not a restatement of the probe above: a core that restarts
    // takes numbers, and NONE of them may collide with a step already on the disk.
    let mut journal = MemoryJournal::new();
    let mut numbers = Progressive::starting_at(0);
    for _ in 0..5 {
        journal
            .intent(StepId::new(numbers.take()), b"intent")
            .expect("intent");
    }

    let mut after_restart = seeded_from(&journal).expect("replay");
    let written: alloc_vec_of_steps = journal
        .replay()
        .expect("replay")
        .iter()
        .map(|(step, _)| step.get())
        .collect();
    for _ in 0..3 {
        let fresh = after_restart.take();
        assert!(
            !written.contains(&fresh),
            "the restarted counter handed back {fresh}, which the journal already holds"
        );
    }
}

#[allow(non_camel_case_types)]
type alloc_vec_of_steps = std::vec::Vec<u64>;
```

⚠️ **L'alias in coda esiste per una ragione:** un banco di integrazione è una crate a sé e **ha** `std`;
il `Vec` di `alloc` che `kernel` usa non si nomina da qui senza un `extern crate alloc`. L'alias dice
che è una scelta e non una distrazione. ⛔ **Se l'implementatore preferisce `Vec<u64>` nudo, va bene
uguale**: è un banco, non il kernel — ma allora si toglie anche l'alias, e non si lasciano tutti e due.

- [ ] **Passo 3: il rosso, e che sia quello giusto**

```bash
cargo test --locked -p kernel --test numbering 2>&1 | tail -20
```

Atteso: **non compila** — `E0432: unresolved import kernel::numbering`. ⚠️ Un rosso diverso da questo si
riporta: è una voce d'errata, non un aggiustamento.

- [ ] **Passo 4: il modulo**

`crates/kernel/src/numbering.rs`, **LF**, nuovo:

```rust
//! The ONE progressive counter of the core, and the seeding that puts it above the journal.
//!
//! ⛔ ONE COUNTER, NOT ONE PER CONSUMER, and that is the whole of this module. The doc of
//! `crate::ports::ipc::ClientId` states the defect it exists to prevent -- "two independent
//! counters that look identical are a divergence nothing would report" -- and names the counter
//! as "the one `journal` will allocate for `StepId`". Today it numbers clients. The day the
//! journal allocates steps it numbers those too, FROM THE SAME INSTANCE.
//!
//! ⚠️ IT IS NOT A PORT AND NOT A PARAMETER. It holds no OS, it decides nothing, and it is
//! DELIVERED to whoever mints identities (ADR-0034): the transport in `platform` RECEIVES one at
//! construction, it does not build one. What ADR-0034 rules out is a decision reading a value
//! nobody handed it; a counter handed over at construction is the shape `Parameters` already has.
//!
//! ⛔ WHY THE SEEDING LIVES HERE AND NOT IN `daemon`. §7 of the milestone-2 design builds the
//! core's listening activity FROM OUTSIDE, in `gui/fake-core`, which cannot import a binary --
//! the same thing that section's own dedotto says about `build_the_arbiter`. A seeding written
//! inside `daemon/src/main.rs` would be COPIED by the fake core, and a copy of the wiring is
//! green on the day the two drift.
//!
//! ⛔ IT ADDS NO OPERATION TO THE `journal` PORT. Whether the allocator belongs inside that port
//! is open item 6 of §9 of the milestone-2 design, confirmed A by the owner on 2026-09-09
//! (decision 42 of the north star): it stays OUT until a second consumer asks for it. `replay`
//! is an operation the port already has.

use crate::ports::journal::{Journal, JournalError};

/// The core's progressive numbers.
///
/// ⚠️ THE SHAPE IS THE ONE `crate::arbiter::Arbiter` ALREADY USES for `next_grant` and
/// `next_ticket` -- a bare `u64` field, read and then incremented -- rather than a new idiom.
/// The overflow reckoning is theirs too: a `u64` that counts clients and steps does not reach
/// its end, and a guard here would be a branch no caller can exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progressive {
    next: u64,
}

impl Progressive {
    /// A counter whose FIRST number is `first`.
    ///
    /// ⛔ IT TAKES THE FIRST NUMBER AND NOT THE LAST ONE USED, and the difference is not
    /// cosmetic: "the last one used" has no value on an empty journal, so every caller would
    /// have to invent one, and two callers inventing separately is the divergence this module
    /// exists to prevent.
    pub const fn starting_at(first: u64) -> Self {
        Progressive { next: first }
    }

    /// The next number, and the counter moves on.
    pub fn take(&mut self) -> u64 {
        let value = self.next;
        self.next += 1;
        value
    }
}

/// A counter seeded ABOVE every step the journal already holds.
///
/// ⚠️ ABOVE THE HIGHEST, NOT AFTER THE LAST. `replay` answers in the order the records were
/// written, and nothing promises that order is ascending: a journal whose last record is
/// `StepId(9)` may still hold `StepId(41)`.
///
/// ⚠️ AN EMPTY JOURNAL SEEDS IT AT ZERO, and that is stated rather than left to arithmetic: it
/// is the only case in which there is no highest number to stand above.
pub fn seeded_from<J: Journal>(journal: &J) -> Result<Progressive, JournalError> {
    let highest = journal.replay()?.iter().map(|(step, _)| step.get()).max();
    Ok(Progressive::starting_at(match highest {
        Some(highest) => highest + 1,
        None => 0,
    }))
}
```

- [ ] **Passo 5: il modulo nella crate**

In `crates/kernel/src/lib.rs` (**CRLF**), in coda all'elenco dei moduli — l'ordine è di arrivo, non
alfabetico.

*Trova* (le ultime due righe del file, prese dal file con `tail -2`):

```
pub mod client;
```

*Sostituisci con:*

```
pub mod client;

pub mod numbering;
```

- [ ] **Passo 6: il verde**

```bash
cargo test --locked -p kernel --test numbering 2>&1 | tail -12
```

Atteso: `test result: ok. 4 passed; 0 failed`.

- [ ] **Passo 7: i due richiami datati**

⛔ **Si prendono dal file, non da qui** (`grep -n` sulla frase, poi `sed -n` sul blocco intero), e si
scrivono con `replace_unique.py`, che conserva i CRLF.

**(a)** In `crates/kernel/src/ports/journal.rs`, in coda al richiamo del 2026-08-21 sul doc di `StepId`
— la riga che finisce con *«WHEN the allocator arrives is the owner's: registered, not taken.»*. Si
aggiunge **dopo** di essa, nello stesso blocco di doc:

```
/// ⚠️ DATED RECALL, 2026-09-11 -- THE COUNTER EXISTS NOW, AND THE ALLOCATOR STILL DOES NOT.
/// `crate::numbering::Progressive` is the core's one progressive counter and
/// `crate::numbering::seeded_from` seeds it above every step this journal already holds; what
/// mints identities today is the `ipc` transport, for `ClientId`. THE PORT IS UNCHANGED: no
/// operation here allocates, and whether one should is open item 6 of §9 of the milestone-2
/// design, confirmed A by the owner on 2026-09-09 -- it stays out until a second consumer asks.
/// The sentence above is therefore still exact, and this line says which half moved.
```

**(b)** In `crates/kernel/src/ports/ipc.rs`, in coda al blocco di richiami sul doc di `ClientId` — dopo
la riga *«two independent counters that look identical, diverging with nothing to report it.»*:

```
/// ⚠️ DATED RECALL, 2026-09-11 -- THE COUNTER IT GUARDS NOW EXISTS, AND THIS PORT IS STILL NOT
/// IMPLEMENTED HERE. It is `crate::numbering::Progressive`, seeded by
/// `crate::numbering::seeded_from` above every step in the journal. "Whoever implements this
/// port draws from THAT counter rather than starting a private one of its own" is no longer a
/// promise about a type that does not exist: the type is one line away, and a private `u64`
/// inside the transport is now a visible choice rather than the only road.
```

⚠️ **Il testo del richiamo (b) NON dice «milestone 2 implementa la porta»**, e non è una svista: il doc
sopra ha già corretto due volte una frase che nominava un traguardo (2026-08-31 e 2026-09-02), e la sua
conclusione scritta è *«Read "whoever implements this port" WITH NO MILESTONE ATTACHED»*. Il compito **2**
è quello che la implementa, e lo dirà **quel** richiamo, sulla riga giusta.

- [ ] **Passo 8: i fine-riga, il cancello, il commit**

```bash
for f in crates/kernel/src/lib.rs crates/kernel/src/ports/journal.rs crates/kernel/src/ports/ipc.rs; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
tr -cd '\r' < crates/kernel/src/numbering.rs | wc -c
tr -cd '\r' < crates/kernel/tests/numbering.rs | wc -c
git ls-files --eol crates/kernel/src/lib.rs crates/kernel/src/ports/journal.rs crates/kernel/src/ports/ipc.rs
bash scripts/gate.sh 2>&1 | tail -3
```

Atteso: per i tre CRLF `CR` **uguale** alle righe e `i/lf w/crlf` invariato; **zero** CR nei due file
nuovi; `GATE GREEN`.

```bash
git add crates/kernel/src/numbering.rs crates/kernel/tests/numbering.rs crates/kernel/src/lib.rs crates/kernel/src/ports/journal.rs crates/kernel/src/ports/ipc.rs docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md
git commit -m "gui(compito 1): il contatore condiviso -- kernel::numbering::Progressive, seminato dal giornale con seeded_from; i due richiami datati su StepId e ClientId"
git push
```

#### Criterio di chiusura del compito 1

- [ ] `cargo test --locked -p kernel --test numbering` → **4 passati**
- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/kernel/src/ports/journal.rs crates/kernel/src/ports/ipc.rs` → **1** per file
- [ ] `bash scripts/gate.sh` → `GATE GREEN`; `bash scripts/gate-deps.sh` dentro di esso **verde**: la lista di ADR-0031 **non è cresciuta** (il modulo non ha dipendenze)
- [ ] i fine-riga rimisurati, `git ls-files --eol` invariato sui tre file toccati
- [ ] la riga **1** della tabella della posizione a ✅ con la data

---


## Compito 2: il lettore di flusso, il trasporto `ipc` in `platform`, e la suite di conformità

**Files:**
- Modify: `crates/kernel/src/framing.rs` (**CRLF**) — `declared_len` e `take_frame`, **accanto** a `unframe`
- Modify: `crates/kernel/tests/framing.rs` — le sonde del lettore di flusso, nelle due direzioni
- Create: `crates/platform/src/ipc.rs` (**LF**)
- Create: `crates/kernel/tests/ipc_contract.rs` (**LF**) — la suite, `include!`-abile
- Create: `crates/platform/tests/ipc_contract_real.rs` (**LF**)
- Modify: `crates/platform/src/lib.rs` (**CRLF**) — una riga di modulo
- Modify: `crates/platform/Cargo.toml` (**CRLF**) — `interprocess`, con la giustificazione accanto
- Modify: `Cargo.lock` (**CRLF**) — **nello stesso commit** del manifesto (vincolo 6)
- Read: la §3 del disegno del 2 per intero; `crates/kernel/src/ports/ipc.rs`; `crates/kernel/src/framing.rs`; `crates/kernel/tests/journal_contract.rs` e `crates/platform/tests/journal_contract_real.rs` **per la forma**

**Interfaces:**
- Consumes: `kernel::ports::ipc::{Ipc, ClientId, IpcError}`; `kernel::numbering::Progressive` dal compito 1
- Produces, e i compiti 7, 8 e 11 li usano con questi nomi esatti:
  - `kernel::framing::declared_len(bytes: &[u8]) -> Option<usize>`
  - `kernel::framing::take_frame(bytes: &[u8]) -> Option<(&[u8], usize)>`
  - `platform::ipc::LocalSocketIpc`
  - `LocalSocketIpc::bound(name: &str, numbers: Progressive, max_body: usize) -> std::io::Result<LocalSocketIpc>`
  - `impl kernel::ports::ipc::Ipc for LocalSocketIpc`

⛔ **`ipc_contract.rs` è la casa UNICA delle asserzioni**, come `journal_contract.rs`: `platform` lo
espande con `include!`, quindi **commenti normali e non `//!`** (un attributo interno non è ammesso in
posizione di item — è scritto in testa a `journal_contract.rs`).

- [ ] **Passo 1: le misure prima, e le tre cose che il pre-controllo ha trovato**

```bash
ls crates/platform/src/ipc.rs 2>&1
grep -nE "^\s{4}fn (accept|send|receive)" crates/kernel/src/ports/ipc.rs
grep -n "pub fn frame\|pub fn unframe\|pub const LENGTH_WIDTH\|pub const MAX_BODY_LEN" crates/kernel/src/framing.rs
grep -rnE "^ *impl Ipc for" crates/ --include='*.rs'
git ls-files --eol crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/platform/src/lib.rs crates/platform/Cargo.toml Cargo.lock
```

Atteso: `ipc.rs` **non esiste**; la porta ha **tre** operazioni — `accept`, `send`, `receive` — e
**nessuna chiusura**; `impl Ipc for` rende **due finte** e nessun trasporto.

✅ **(a) L'API non bloccante di `interprocess` 2.4.4 è VERIFICATA, non dedotta** — letta il 2026-09-11
nel sorgente installato (`~/.cargo/registry/src/index.crates.io-*/interprocess-2.4.4/src/local_socket/`),
e il disegno la dava 🔶 *dedotta* (§3, «che anche i flussi di `interprocess` leggano senza bloccare, e
le varianti del modo non bloccante»):

| Cosa | Dove | Che cosa dice |
|---|---|---|
| `ListenerOptions::nonblocking(mode)` | `listener/options.rs:91` | costruttore, prima di `create_sync()` |
| `ListenerNonblockingMode` | `listener/trait.rs:67` | quattro varianti: `Neither`, `Accept`, `Stream`, `Both` — **`Both`** è quella che serve |
| `Listener::accept()` | `listener/trait.rs:36` | `io::Result<Self::Stream>`; in modo `Accept` o `Both` rende **`ErrorKind::WouldBlock`** se nessuno sta collegandosi |
| `Stream::set_nonblocking(bool)` | `stream/trait.rs:44` | il flusso da solo |

⛔ **E una trappola di Windows, letta nello stesso doc di `accept`**, che nessuno dei due disegni
riporta: *«neglecting to call this periodically may result in new clients being unable to connect»* —
su named pipe un client che si collega e si scollega **senza** che `accept` sia stato chiamato lascia
un'istanza morta che blocca i collegamenti nuovi. L'attività del compito 7 chiama `accept` **a ogni
giro**, quindi la trappola è già disinnescata: si scrive nel doc del tipo perché nessuno la rimuova.

⛔ **(b) `unframe` NON È UN LETTORE DI FLUSSO, e il suo doc lo dice e chiede la cura** (P-13). La firma
è `unframe(&[u8]) -> Result<&[u8], WireError>` e **rifiuta ogni coda** con `WireError::TrailingBytes`:
un buffer di flusso che porta una cornice e mezza la fa scattare. Il doc scrive la via:

> ⚠️ A REAL TRANSPORT WILL WANT A SECOND ENTRY POINT, one that answers «this frame, and where the next
> starts», and it is a **NEW function beside this one rather than a loosening of this one** —
> `TrailingBytes` is the whole of what `a_frame_with_a_tail_is_refused` holds.

⚠️ **E `frame` rende un `Result`**, non un `Vec<u8>`: `frame(body) -> Result<Vec<u8>, WireError>`, con
`TooLong` sopra `MAX_BODY_LEN`. Ogni chiamata in questo compito lo gestisce.

⛔ **(c) `IpcError::MalformedMessage` non ha un produttore possibile con la busta com'è** (P-14), e la
§3 del 2 lo pretende (*«cornice rotta → `MalformedMessage` e il client resta»*). La busta è un prefisso
di lunghezza: **qualunque** quaterna di byte è una lunghezza valida, e `MAX_BODY_LEN` è `u32::MAX`, che
ogni `u32` soddisfa. La cura è **D9**, il tetto consegnato — sotto.

- [ ] **Passo 2: le sonde del lettore di flusso, che falliscono**

In coda a `crates/kernel/tests/framing.rs`, **coi fine-riga del file** (`git ls-files --eol` al Passo 1):

```rust
#[test]
fn two_frames_back_to_back_come_out_one_at_a_time() {
    // ⛔ THE CASE `unframe` REFUSES BY DESIGN, and the reason this function exists: a stream
    // carries frames back to back, and `unframe` answers `TrailingBytes` to the whole buffer.
    let mut stream = framing::frame(b"first").expect("frame");
    stream.extend_from_slice(&framing::frame(b"second").expect("frame"));
    assert_eq!(
        framing::unframe(&stream),
        Err(WireError::TrailingBytes),
        "the one-frame reader still refuses a tail -- THAT is what must not change"
    );

    let (first, consumed) = framing::take_frame(&stream).expect("a whole frame is there");
    assert_eq!(first, b"first");
    let (second, _) = framing::take_frame(&stream[consumed..]).expect("and so is the next");
    assert_eq!(second, b"second");
}

#[test]
fn a_stream_shorter_than_its_prefix_is_not_yet_a_frame() {
    assert_eq!(framing::take_frame(&[0, 0]), None);
    assert_eq!(framing::declared_len(&[0, 0]), None);
}

#[test]
fn a_stream_shorter_than_the_declared_body_is_not_yet_a_frame() {
    let whole = framing::frame(b"a body long enough to cut").expect("frame");
    let half = &whole[..whole.len() - 3];
    assert_eq!(framing::take_frame(half), None, "NOT YET is not an error");
    assert_eq!(
        framing::declared_len(half),
        Some(b"a body long enough to cut".len()),
        "the length is readable BEFORE the body arrives -- which is what a cap needs"
    );
}

#[test]
fn an_empty_body_is_a_whole_frame_and_not_an_absence() {
    let whole = framing::frame(b"").expect("frame");
    assert_eq!(framing::take_frame(&whole), Some((&[][..], framing::LENGTH_WIDTH)));
}
```

```bash
cargo test --locked -p kernel --test framing 2>&1 | tail -12
```

Atteso: **non compila** — `take_frame` e `declared_len` non esistono.

- [ ] **Passo 3: le due funzioni, ACCANTO a `unframe` e non al suo posto**

In `crates/kernel/src/framing.rs` (**CRLF**), **dopo** il corpo di `unframe`:

```rust
/// The length the frame at the head of `bytes` declares, once the prefix has arrived.
///
/// ⚠️ IT EXISTS FOR THE CAP AND SAYS SO. A transport owns the memory a frame will be buffered
/// into, and it has to decide whether to buffer it BEFORE the body arrives; without this it
/// would have to re-read four bytes itself, which is a second copy of the prefix format.
pub const fn declared_len(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < LENGTH_WIDTH {
        return None;
    }
    let declared = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Some(declared as usize)
}

/// Reads the FIRST frame out of a byte STREAM, and says how many bytes it consumed.
///
/// ⛔ THIS IS THE SECOND ENTRY POINT `unframe`'s doc ASKS FOR, and it is a NEW FUNCTION BESIDE
/// IT rather than a loosening of it. `unframe` refuses every tail, and that refusal is the whole
/// of what `a_frame_with_a_tail_is_refused` holds for the two private channels: loosening it
/// would let a decoder consume fewer bytes than the declared length and stay green, which is
/// what catalogue line `Q4 · I5 · §6.10` forbids.
///
/// ⚠️ `None` IS "NOT YET", NOT AN ERROR. Fewer bytes than the prefix, or fewer than the prefix
/// declares, is the ordinary state of a stream read halfway through a frame.
///
/// ⛔ IT CANNOT FAIL, AND THE ABSENCE OF A `Result` IS THE STATEMENT. The envelope knows how
/// many bytes there are, never what they mean: ANY four bytes are a length. A body that does not
/// decode is `WireError::Malformed`, produced by the schemas; a declared length nobody wants to
/// buffer is a CAP, and a cap belongs to whoever owns the memory -- the transport -- not here.
pub fn take_frame(bytes: &[u8]) -> Option<(&[u8], usize)> {
    let declared = declared_len(bytes)?;
    let body = &bytes[LENGTH_WIDTH..];
    if body.len() < declared {
        return None;
    }
    Some((&body[..declared], LENGTH_WIDTH + declared))
}
```

```bash
cargo test --locked -p kernel --test framing 2>&1 | tail -12
```

Atteso: verde, e `a_frame_with_a_tail_is_refused` **ancora verde** — è la seconda direzione: la
funzione nuova non ha allentato la vecchia.

- [ ] **Passo 4: la dipendenza, in due passi**

In `crates/platform/Cargo.toml` (**CRLF**), sotto `[dependencies]`, con la giustificazione accanto
com'è la convenzione del manifesto del kernel:

```toml
# The local socket under the `ipc` port: one named pipe on Windows, one unix socket on Linux,
# behind one type. ⛔ IT IS HERE AND NOT IN `kernel`: the port is the kernel's, the socket is the
# OS (I3), and ADR-0031 does not bind this crate -- `platform` is where the I/O must live.
# Pinned to the version measured on 2026-09-11. SP-8's emitter resolved 2.4.3 from its own
# ignored lockfile, which is a different graph and not a divergence (E6 of the part-1 plan).
interprocess = "2.4.4"
```

Poi, **fuori dal cancello** (vincolo 6):

```bash
cargo build -p platform
git status --porcelain Cargo.lock
```

Atteso: `Cargo.lock` **modificato**. ⛔ Manifesto e lockfile si committano **insieme**, o il cancello
resta rosso.

- [ ] **Passo 5: la suite di conformità**

`crates/kernel/tests/ipc_contract.rs`, **LF**, nuovo. ⛔ **Commenti normali, non `//!`.**

```rust
// THE CONFORMANCE SUITE OF THE `ipc` PORT (§3 of the milestone-2 design). It runs against every
// implementation of the port, and what it is worth is that the fakes and the real transport
// answer the SAME contract.
//
// ⛔ REGULAR COMMENTS AND NOT `//!`, BECAUSE THIS FILE IS `include!`d.
// `crates/platform/tests/ipc_contract_real.rs` expands it IN ITEM POSITION, and an inner
// attribute -- which is what `//!` desugars to -- is not permitted there. Same mechanism as
// `journal_contract.rs`, and for the same reason: two copies of the assertions would diverge,
// and the first one that diverged would print `ok` while comparing nothing.
//
// ⛔ WHAT IS DELIBERATELY ABSENT: every promise that needs a real peer writing bytes. A whole
// frame, half a frame, a body over the cap, an ended stream -- none of them can be staged
// against a fake that has no stream, and asserting them here would be a suite comparing itself.
// They live in `crates/platform/tests/ipc_contract_real.rs` beside the `include!`, which is
// where `journal_contract_real.rs` puts its own.

/// What every implementation must answer. The macro takes a constructor so each crate hands
/// over its own.
macro_rules! ipc_contract_suite {
    ($build:expr) => {
        #[test]
        fn with_nobody_connected_accept_answers_none() {
            let mut ipc = $build();
            assert_eq!(
                ipc.accept(),
                None,
                "an empty accept is the NORMAL state of a core nobody has opened a gui against"
            );
        }

        #[test]
        fn sending_to_an_unknown_client_is_disconnected() {
            let mut ipc = $build();
            assert_eq!(
                ipc.send(ClientId::new(9_999), b"anything"),
                Err(IpcError::Disconnected),
                "a client that is not in the table is gone, which is the same thing"
            );
        }

        #[test]
        fn receiving_from_an_unknown_client_is_disconnected() {
            let mut ipc = $build();
            assert_eq!(ipc.receive(ClientId::new(9_999)), Err(IpcError::Disconnected));
        }
    };
}
```

- [ ] **Passo 6: il banco di `platform`, col pari su un thread**

`crates/platform/tests/ipc_contract_real.rs`, **LF**, nuovo:

```rust
//! The `ipc` conformance suite expanded against the REAL transport, plus the promises only a
//! real peer can exercise.
//!
//! ⚠️ THE SOCKET NAME IS PER-TEST AND CARRIES THE LINE NUMBER, the shape
//! `crates/daemon/src/main.rs` already uses for its private directories: two tests sharing a
//! name pass alone and fail together, which is the flakiest red there is.
//!
//! ⚠️ THE WAITS ARE `yield_now` LOOPS AND NOT FIXED SLEEPS, except where the assertion is about
//! something that has NOT arrived -- there an interval must elapse, and those two are named.

use std::io::Write;

use kernel::framing;
use kernel::numbering::Progressive;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use platform::ipc::LocalSocketIpc;

/// The cap every test in this file hands over. Big enough for any body written here, small
/// enough that `a_body_over_the_cap_is_malformed` can exceed it without allocating.
const CAP: usize = 4_096;

fn socket_name_for_line(line: u32) -> String {
    format!("harness-ipc-contract-{}-{}", std::process::id(), line)
}

fn build() -> LocalSocketIpc {
    LocalSocketIpc::bound(&socket_name_for_line(line!()), Progressive::starting_at(0), CAP)
        .expect("the listener binds")
}

include!("../../kernel/tests/ipc_contract.rs");

ipc_contract_suite!(build);

/// Connects, writes `bytes` verbatim, and stays open for `hold`.
fn a_peer_that_writes(name: String, bytes: Vec<u8>, hold_ms: u64) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream};
        let ns = name.to_ns_name::<GenericNamespaced>().expect("name");
        let mut stream = loop {
            match Stream::connect(ns.clone()) {
                Ok(stream) => break stream,
                Err(_) => std::thread::yield_now(),
            }
        };
        stream.write_all(&bytes).expect("write");
        std::thread::sleep(std::time::Duration::from_millis(hold_ms));
    })
}

fn accept_one(ipc: &mut LocalSocketIpc) -> ClientId {
    loop {
        if let Some(client) = ipc.accept() {
            return client;
        }
        std::thread::yield_now();
    }
}

#[test]
fn a_whole_frame_comes_back_whole_envelope_included() {
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let peer = a_peer_that_writes(name, framing::frame(b"hello core").expect("frame"), 300);

    let client = accept_one(&mut ipc);
    let seen = loop {
        match ipc.receive(client).expect("a live client") {
            Some(bytes) => break bytes,
            None => std::thread::yield_now(),
        }
    };
    assert_eq!(
        seen,
        framing::frame(b"hello core").expect("frame"),
        "THE WHOLE FRAME comes back, envelope included: `IpcMessage::decode` unframes it"
    );
    peer.join().expect("the peer ends");
}

#[test]
fn two_frames_in_one_write_come_back_one_at_a_time() {
    // ⛔ THE CASE THAT BREAKS A TRANSPORT BUILT ON `unframe`: two frames in one read.
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let mut bytes = framing::frame(b"first").expect("frame");
    bytes.extend_from_slice(&framing::frame(b"second").expect("frame"));
    let peer = a_peer_that_writes(name, bytes, 300);

    let client = accept_one(&mut ipc);
    let mut seen: Vec<Vec<u8>> = Vec::new();
    while seen.len() < 2 {
        match ipc.receive(client).expect("a live client") {
            Some(bytes) => seen.push(bytes),
            None => std::thread::yield_now(),
        }
    }
    assert_eq!(seen[0], framing::frame(b"first").expect("frame"));
    assert_eq!(seen[1], framing::frame(b"second").expect("frame"));
    peer.join().expect("the peer ends");
}

#[test]
fn two_accepts_hand_out_different_and_ascending_numbers() {
    // ⛔ THE PROMISE IS THE COUNTER'S, checked HERE because `accept` is the only site that mints
    // a `ClientId`: a transport that started a private `u64` would pass every other test here.
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(41), CAP).expect("binds");
    let first_peer = a_peer_that_writes(name.clone(), Vec::new(), 300);
    let first = accept_one(&mut ipc);
    let second_peer = a_peer_that_writes(name, Vec::new(), 300);
    let second = accept_one(&mut ipc);

    assert_eq!(first, ClientId::new(41), "the counter was started at 41");
    assert_eq!(second, ClientId::new(42));
    first_peer.join().expect("peer one ends");
    second_peer.join().expect("peer two ends");
}

#[test]
fn half_a_frame_is_not_a_body_yet_and_not_an_error() {
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let whole = framing::frame(b"a body long enough to cut").expect("frame");
    let half = whole[..whole.len() - 3].to_vec();
    let peer = a_peer_that_writes(name, half, 300);

    let client = accept_one(&mut ipc);
    // ⚠️ A REAL INTERVAL, because the assertion is about what has NOT arrived.
    std::thread::sleep(std::time::Duration::from_millis(80));
    assert_eq!(ipc.receive(client), Ok(None));
    peer.join().expect("the peer ends");
}

#[test]
fn a_body_over_the_cap_is_malformed_and_the_client_stays() {
    // ⛔ THIS IS THE ONLY PRODUCER OF `MalformedMessage` IN THIS TRANSPORT, and D9 says why:
    // the envelope cannot tell a broken frame from a valid one, so the cap is what makes the
    // promise of §3 true. ⚠️ NOTHING BIG IS ALLOCATED: only the prefix is written.
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let over = ((CAP + 1) as u32).to_be_bytes().to_vec();
    let peer = a_peer_that_writes(name, over, 300);

    let client = accept_one(&mut ipc);
    let seen = loop {
        match ipc.receive(client) {
            Ok(None) => std::thread::yield_now(),
            other => break other,
        }
    };
    assert_eq!(seen, Err(IpcError::MalformedMessage));
    assert_eq!(
        ipc.receive(client),
        Err(IpcError::MalformedMessage),
        "THE CLIENT STAYS: it is the peer's mistake, not its death, and a length-prefixed \
         stream cannot be resynchronised -- so it says the same thing again"
    );
    peer.join().expect("the peer ends");
}

#[test]
fn a_peer_that_goes_away_is_disconnected_and_leaves_the_table() {
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let peer = a_peer_that_writes(name, Vec::new(), 0);
    let client = accept_one(&mut ipc);
    peer.join().expect("the peer ends");

    let seen = loop {
        match ipc.receive(client) {
            Ok(None) => std::thread::yield_now(),
            other => break other,
        }
    };
    assert_eq!(seen, Err(IpcError::Disconnected));
    assert_eq!(
        ipc.receive(client),
        Err(IpcError::Disconnected),
        "and it STAYS gone: the client left the table, it was not merely reported once"
    );
}
```

- [ ] **Passo 7: il rosso**

```bash
cargo test --locked -p platform --test ipc_contract_real 2>&1 | tail -20
```

Atteso: **non compila** — `E0432: unresolved import platform::ipc`.

- [ ] **Passo 8: il trasporto**

`crates/platform/src/ipc.rs`, **LF**, nuovo:

```rust
//! The `ipc` port over a local socket: one named pipe on Windows, one unix socket on Linux.
//!
//! ⛔ BOTH THE LISTENER AND THE STREAMS ARE NONBLOCKING, and that is `ListenerNonblockingMode::Both`
//! rather than two calls. Read in `interprocess` 2.4.4 on 2026-09-11: `accept` answers
//! `ErrorKind::WouldBlock` when nobody is connecting, and a stream in that mode answers the same
//! when there is nothing to read. The core's activity asks every turn and must block on neither
//! (§3 and §5 of the milestone-2 design).
//!
//! ⛔ WINDOWS TRAP, READ IN THE DOC OF `Listener::accept` AND NOT DEDUCED: "neglecting to call
//! this periodically may result in new clients being unable to connect" -- a peer that connects
//! and disconnects with no `accept` in between leaves a dead instance that blocks new
//! connections. The activity calls `accept` EVERY TURN, and that is what disarms it. Whoever
//! moves that call out of the loop reopens this.
//!
//! ⛔ IT READS THE STREAM WITH `framing::take_frame` AND NOT `framing::unframe`. `unframe` takes
//! ONE frame and refuses every tail -- its own doc says a real transport wants a second entry
//! point -- and two frames arriving in one read is the ordinary case, not the edge one.
//!
//! ⛔ AND IT ADDS NO ENVELOPE OF ITS OWN: THE MESSAGES ARRIVE ALREADY FRAMED. `IpcMessage::encode`
//! ends in `framing::frame` and `IpcMessage::decode` begins in `framing::unframe`, so a message
//! handed to `send` is ALREADY self-delimiting; framing it again would put a second envelope on
//! the wire, give the same four bytes two meanings, and leave the TypeScript peer stripping two.
//! `send` therefore writes VERBATIM, and `receive` gives back THE WHOLE FRAME -- envelope
//! included -- which is what `decode` expects. The envelope is read here only to find where one
//! message ends and the next begins, which is the one thing a byte stream does not carry and the
//! two `Vec<Vec<u8>>` fakes of this port get for free.
//!
//! ⛔ A `Vec` AND NOT A `HashMap` for the client table, the reckoning gotcha #12 records for the
//! kernel: the table holds ONE client in milestone 2 (the gui is 0..1, ADR-0004), and a map
//! would buy nothing while introducing an iteration order.

use std::io::{ErrorKind, Read, Write};

use interprocess::local_socket::{
    prelude::*, GenericNamespaced, Listener, ListenerNonblockingMode, ListenerOptions, Stream,
};
use kernel::framing;
use kernel::numbering::Progressive;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};

/// One connected peer, and what has arrived from it so far.
struct Connected {
    id: ClientId,
    stream: Stream,
    /// Bytes received and not yet formed into a whole frame.
    pending: Vec<u8>,
    /// Set once a declared length exceeded the cap. ⛔ IT IS NOT CLEARED: a length-prefixed
    /// stream cannot be resynchronised, so every later `receive` answers the same thing.
    poisoned: bool,
}

/// The real `ipc` transport.
pub struct LocalSocketIpc {
    listener: Listener,
    clients: Vec<Connected>,
    /// ⛔ DELIVERED, NEVER STARTED HERE (ADR-0034, and the doc of `ClientId`): a private counter
    /// inside this type would be the second one, and two that look identical diverge with
    /// nothing to report it.
    numbers: Progressive,
    /// The longest body this transport will buffer. ⛔ DELIVERED TOO, and it is what gives
    /// `IpcError::MalformedMessage` a producer: the envelope cannot tell a broken frame from a
    /// valid one -- any four bytes are a length -- so without a cap the promise of §3 would have
    /// nothing to hold, and a peer declaring four gibibytes would be buffered for ever.
    max_body: usize,
}

impl LocalSocketIpc {
    /// Binds the listener on `name`, takes the counter it mints `ClientId`s from, and the cap.
    pub fn bound(name: &str, numbers: Progressive, max_body: usize) -> std::io::Result<Self> {
        let ns = name.to_ns_name::<GenericNamespaced>()?;
        let listener = ListenerOptions::new()
            .name(ns)
            .nonblocking(ListenerNonblockingMode::Both)
            .create_sync()?;
        Ok(LocalSocketIpc {
            listener,
            clients: Vec::new(),
            numbers,
            max_body,
        })
    }

    fn position_of(&self, client: ClientId) -> Option<usize> {
        self.clients.iter().position(|held| held.id == client)
    }

    fn drop_client(&mut self, at: usize) {
        self.clients.remove(at);
    }
}

impl Ipc for LocalSocketIpc {
    fn accept(&mut self) -> Option<ClientId> {
        match self.listener.accept() {
            Ok(stream) => {
                let id = ClientId::new(self.numbers.take());
                self.clients.push(Connected {
                    id,
                    stream,
                    pending: Vec::new(),
                    poisoned: false,
                });
                Some(id)
            }
            // ⛔ `WouldBlock` IS THE NORMAL STATE and not an error: nobody is knocking.
            Err(_) => None,
        }
    }

    fn send(&mut self, client: ClientId, message: &[u8]) -> Result<(), IpcError> {
        let Some(at) = self.position_of(client) else {
            return Err(IpcError::Disconnected);
        };
        // ⛔ VERBATIM: the message already carries its envelope (see the module doc).
        match self.clients[at].stream.write_all(message) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                // The pipe is full. Nothing the caller can act on is lost: §6.1.4 has the core
                // send the changed piece again, and the port has no "try again".
                Ok(())
            }
            Err(_) => {
                self.drop_client(at);
                Err(IpcError::Disconnected)
            }
        }
    }

    fn receive(&mut self, client: ClientId) -> Result<Option<Vec<u8>>, IpcError> {
        let Some(at) = self.position_of(client) else {
            return Err(IpcError::Disconnected);
        };
        if self.clients[at].poisoned {
            return Err(IpcError::MalformedMessage);
        }

        let mut chunk = [0_u8; 4096];
        loop {
            match self.clients[at].stream.read(&mut chunk) {
                Ok(0) => {
                    // End of stream: the peer is gone, and it LEAVES THE TABLE.
                    self.drop_client(at);
                    return Err(IpcError::Disconnected);
                }
                Ok(read) => self.clients[at].pending.extend_from_slice(&chunk[..read]),
                Err(error) if error.kind() == ErrorKind::WouldBlock => break,
                Err(_) => {
                    self.drop_client(at);
                    return Err(IpcError::Disconnected);
                }
            }
        }

        if let Some(declared) = framing::declared_len(&self.clients[at].pending) {
            if declared > self.max_body {
                // ⛔ THE CLIENT STAYS. It is the peer's mistake, not its death (§3), and the
                // two outcomes must stay distinguishable to the caller.
                self.clients[at].poisoned = true;
                return Err(IpcError::MalformedMessage);
            }
        }

        let Some((_, consumed)) = framing::take_frame(&self.clients[at].pending) else {
            return Ok(None);
        };
        // ⛔ THE WHOLE FRAME, ENVELOPE INCLUDED, and not the body: `IpcMessage::decode` unframes
        // what it is given. Handing back the body would make every caller re-frame it.
        let whole = self.clients[at].pending[..consumed].to_vec();
        self.clients[at].pending.drain(..consumed);
        Ok(Some(whole))
    }
}
```

- [ ] **Passo 9: il modulo nella crate, e il verde**

In `crates/platform/src/lib.rs` (**CRLF**), accanto agli altri moduli — l'ordine è di arrivo:

```
pub mod ipc;
```

```bash
cargo test --locked -p platform --test ipc_contract_real 2>&1 | tail -12
cargo test --locked -p kernel --test framing 2>&1 | tail -6
```

Atteso: **nove** test verdi nel banco di `platform` (le tre della suite più le sei proprie); il banco
`framing` del kernel verde con le quattro sonde nuove. ⚠️ `crates/kernel/tests/ipc_contract.rs` **non è
un banco a sé**: definisce una macro e nessun `#[test]`, quindi `cargo test -p kernel` lo compila e
riporta zero test — è voluto, e vale la pena scriverlo nel commit perché sembra un errore.

- [ ] **Passo 10: i fine-riga, il cancello, il commit**

```bash
for f in crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/platform/src/lib.rs crates/platform/Cargo.toml Cargo.lock; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/platform/src/lib.rs crates/platform/Cargo.toml Cargo.lock
bash scripts/gate.sh 2>&1 | tail -3
```

Atteso: CR uguale alle righe sui CRLF e `git ls-files --eol` **invariato**; `GATE GREEN` — ⛔ e
`gate-deps.sh` verde dentro di esso: `interprocess` entra in `platform`, che ADR-0031 **non** vincola,
quindi la lista non cresce. Se diventasse rosso è una voce d'errata, e **non** si aggiunge una riga
alla lista.

```bash
git add crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/kernel/tests/ipc_contract.rs crates/platform/src/ipc.rs crates/platform/src/lib.rs crates/platform/tests/ipc_contract_real.rs crates/platform/Cargo.toml Cargo.lock docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md
git commit -m "gui(compito 2): il lettore di flusso take_frame accanto a unframe, e il trasporto ipc in platform su interprocess 2.4.4 non bloccante; la suite di conformita ipc_contract.rs inclusa da platform, col pari su un thread"
git push
```

#### Criterio di chiusura del compito 2

- [ ] `cargo test --locked -p platform --test ipc_contract_real` → **nove** passati, e `receive` rende **la cornice intera** (D10), non il corpo
- [ ] `cargo test --locked -p kernel --test framing` → verde, e `a_frame_with_a_tail_is_refused` **ancora presente e verde**: la seconda direzione
- [ ] `grep -rnE "^ *impl Ipc for" crates/ --include='*.rs'` → **tre**: le due finte e `LocalSocketIpc`
- [ ] `bash scripts/gate.sh` → `GATE GREEN`, con `Cargo.lock` committato **insieme** al manifesto
- [ ] i fine-riga rimisurati e `git ls-files --eol` invariato
- [ ] la riga **2** della tabella della posizione a ✅ con la data

---

## Come si riprende — il diario di questo piano, coi comandi

### La prima chiusura — 2026-09-11: il piano è SCRITTO FINO AL COMPITO 2 di sedici; nessun compito è eseguito

⛔ **DA SAPERE SUBITO.** Niente è a metà: albero pulito, nessuno stash, nessuna operazione git in corso,
nessun server acceso, **nessun codice di prodotto toccato** — `git diff --stat 42b50d8..HEAD -- crates/
scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla. ⛔ **E
L'ESECUZIONE NON È COMINCIATA:** la tabella della posizione è tutta ⬜, l'errata è **vuota**, e questo
piano **non è ancora finito** — i compiti **3–16 non esistono**. La sessione nuova **scrive**, non esegue.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto; nessuno stash |
| I commit di questa sessione | `git log --oneline 42b50d8..HEAD` — **uno**, `02aa514`, solo questo file |
| Codice di prodotto | **non toccato**, col comando in «Da sapere subito» |
| Quanto è scritto | `grep -c '^## Compito' <questo file>` → **2**; la tabella della posizione ne elenca **sedici** |
| L'errata | ⚠️ **la sonda si legge DENTRO la sezione**, o conta anche le voci d'errata di altri piani citate fra le voci aperte — misurato, un `grep -c` nudo rende **2**, che sono `E50/E51/E100` del Traguardo 5 ed `E12` del Traguardo 6: `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0**, nasce vuota e non resterà vuota |
| Il pre-controllo | `grep -c '^### P-' <questo file>` → **15**; le decisioni, `grep -c '^| \*\*D[0-9]' ` → **10** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN`, rilanciato all'apertura e alla chiusura (log datati nello scratchpad: `gate-2026-09-11-apertura.log`, `gate-2026-09-11-piano-parte2-a.log`, `gate-2026-09-11-chiusura-piano.log`); `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | questo piano è **LF** nell'indice e nell'albero: `git ls-files --eol <questo file>` → `i/lf w/lf`, e `tr -cd '\r' < <questo file> \| wc -c` → `0` |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Margine del compendio | il comando del vincolo 11 → `11030`, **invariato**: questa sessione non ha toccato il compendio |
| File temporanei | nessuno nel repository — `git status --porcelain` vuoto; gli script e i frammenti stanno nello scratchpad, fuori dall'albero |
| Debito lasciato | **nessuno non dichiarato**: i quattordici compiti che mancano sono la tabella della posizione; le voci aperte stanno nella sezione omonima; le dieci decisioni sono ribaltabili e ciascuna porta il costo |

#### Le decisioni prese scrivendo, oltre alle dieci della tabella

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 1 | commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md` dice *«senza co-autore»*; una direttiva di sistema chiede il contrario e la divergenza è **portata al proprietario**, come in ogni sessione di questo repository | un `--amend` |
| 2 | il piano resta **uno solo** per tutta la parte 2, e non si spezza in due (Rust e web) | domanda A/B al proprietario, risposta **A**: è ciò che la §3 della stella polare dice (*«il piano resta in due parti … non cambia»*), e il Traguardo 5 ebbe tredici compiti in un piano solo. ⚠️ La `superpowers:writing-plans` suggerisce di spezzare quando la spec copre sottosistemi indipendenti: la divergenza è stata **detta** e la decisione è del proprietario | un piano da spezzare a metà scrittura |
| 3 | il pre-controllo si fa **scrivendo ogni compito**, non tutto in testa | i difetti che ha trovato sono usciti dal **codice dettato**: P-15 è nato correggendo P-13, cioè scrivendo il trasporto vero. Un pre-controllo fatto solo sui disegni non li avrebbe visti | nessuno: le voci P si aggiungono in coda |
| 4 | i compiti portano **codice vero**, non descrizioni | è la regola «niente segnaposto» di `superpowers:writing-plans`, e qui ha pagato subito: la doppia incorniciatura di P-15 si vede solo scrivendo le righe | il piano è lungo |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **Inserire una riga in una tabella con una sostituzione che ancora il testo DOPO di essa lascia una
  riga vuota che SPEZZA la tabella in due, e `check-docs.sh` non lo vede.** Successo **due volte** in
  questa sessione, sulle stesse decisioni D7–D9 e D10. Il controllo si lancia **dopo ogni** scrittura su
  un file con tabelle, non alla fine:
  `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <file>`.
- ⛔ **`$TMPDIR` non è impostata nel tool Bash di questa macchina:** `bash scripts/gate.sh > "$TMPDIR/x.log"`
  fallisce con *«/x.log: Permission denied»* e il cancello esce **rosso per il motivo sbagliato**. Il
  percorso dello scratchpad si scrive per esteso.
- **Lo scratchpad di una sessione aperta NELLA cartella del repo è corto** (~130 caratteri) e
  `python - < percorso.py` funziona; `python percorso.py` pure. È la quinta sessione di fila che lo
  conferma.
- **I `Write` lunghi vanno nello scratchpad e si appendono col `cat`**, non con un heredoc: un heredoc di
  qualche KB con tabelle lunghe rompe Bash (vicolo cieco della seconda ripresa della stella polare). Un
  heredoc **corto** con uno script Python dentro regge, ed è quello che questa sessione ha usato per le
  sostituzioni.
- ⛔ **Il tetto di 30 000 caratteri del tool Bash va per TABELLA, non per file:** la stella polare a 150
  righe trabocca su 920–1081 (le tabelle delle decisioni del coordinatore e delle registrate) — lì **20–60**
  righe; il resto regge a 150. Quando trabocca, l'uscita finisce su file e la chiamata è persa. I tetti
  che hanno funzionato oggi: compendio **200** righe, i due disegni a blocchi calcolati col comando
  `awk 'BEGIN{b=0;s=1} {b+=length($0)+1; if(b>18000){print s"-"NR; b=0; s=NR+1}} END{print s"-"NR}' <file>`.
- ⛔ **Il sorgente di una dipendenza è leggibile, e batte il ricordo:**
  `ls -d ~/.cargo/registry/src/*/interprocess-2.4.4` rende la cartella, e da lì l'API non bloccante si
  **legge** invece di dedurla. È così che P-13 e l'API di `interprocess` sono passate da 🔶 *dedotto* a
  ✅ *verificato*. Vale per ogni crate già scaricata.

#### La lista di lettura della sessione nuova — a compito, non tutto

⛔ **I due disegni NON si rileggono per intero a ogni sessione**, e questa è una correzione alla riga del
punto 3 dell'ottava chiusura della parte 1 (*«coi due disegni letti per intero»*): misurato il 2026-09-11
con lo snippet `tiktoken` di `CLAUDE.md`, la stella polare pesa **57 388** token e il disegno del 2
**32 687** — novantamila token per scrivere un compito che ne tocca una sezione. Si legge **per compito**:

| Compito | Che cosa si legge |
|---|---|
| **3** — lo schema | §4 del 2 (le varianti e le tre righe delle fixture); «La GUI dentro», sequenza 1; `crates/kernel/src/wire/ipc.rs` e `crates/kernel/tests/ipc_wire.rs` **per intero** |
| **4, 5** — la settima porta | §2 della stella polare **per intero**; §8 del 2, la riga della settima porta; `crates/kernel/src/ports/mod.rs`, `journal.rs`, `crates/platform/src/journal.rs` |
| **6** — il registro | §5 del 2, la prima tabella; ADR-0038; `crates/kernel/src/permission.rs`, `record.rs`, `crates/kernel/tests/frozen_bytes.rs` |
| **7, 8** — l'attività e il daemon | §5 del 2 **per intero**; le **tre sequenze** di «La GUI dentro»; `crates/daemon/src/main.rs` per intero |
| **9** — la campagna DST | §8 del 2, la riga dell'attività; `crates/simulator/tests/gui_death_campaign.rs`; il settimo passo di `scripts/gate.sh` |
| **10, 12, 13** — la SPA | §6a del 2; §1 della stella polare (il catalogo: le cinque tabelle piene per i moduli del 2, la corta per i segnaposto); «Il modello della GUI»; §9 del 2 per gli attrezzi |
| **11** — il core finto | §7 del 2 **per intero** |
| **14, 15** — il cancello e la CI | §8 del 2 **per intero**; `scripts/gate.sh`, `.github/workflows/quality-gate.yml`, `.gitignore`; la tabella delle voci senza numero AUD dell'audit per X-1 e X-3 |
| **16** — la chiusura | §12 del compendio, `README.md`, `roadmap.md`, `tracciabilita.md`, `HANDOFF.md`, `porta-di-qualita.md`, `riferimenti.md` — le case che D14 della parte 1 nomina |

⚠️ **Resta obbligatoria la lettura d'apertura di `CLAUDE.md`** — questo file e il compendio, **37 270**
token misurati oggi — e la testa di **questo** piano: vincoli globali, posizione, errata, P-1…P-15, le
dieci decisioni, le voci aperte. **24 533** token oggi, e cresce a ogni compito scritto.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`:
   la testa è il commit di questa chiusura o uno dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** (non i compiti già scritti, se
   non per i nomi che il compito nuovo consuma — il blocco *Interfaces* di ciascuno li porta).
3. `superpowers:writing-plans`: scrivere i compiti **3, 4, 5, 6 …** nell'ordine della tabella della
   posizione, ciascuno col proprio **pre-controllo delle quattro domande** contro il codice di **adesso**,
   e la lista di lettura qui sopra per sapere che cosa aprire. Ogni difetto trovato diventa una voce **P**
   in coda al pre-controllo, e la decisione che ne discende una riga **D**.
4. ⛔ **Dopo ogni scrittura su questo file**: il controllo delle tabelle spezzate, `tr -cd '\r'` a zero,
   `bash scripts/check-docs.sh` → `OK`, e il commit — **senza co-autore**.
5. Quando i sedici compiti ci sono: la **revisione del piano intero** — copertura dei disegni, segnaposto,
   coerenza dei nomi fra i blocchi *Interfaces*, ogni *Trova* rilanciato — come la parte 1 fece nella terza
   sessione; poi l'esecuzione in una sessione **nuova**, un subagente fresco per compito.
6. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

---
