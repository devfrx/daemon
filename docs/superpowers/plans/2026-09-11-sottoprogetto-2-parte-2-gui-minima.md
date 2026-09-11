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
| 1 | **le due spec si toccano SOLO coi richiami datati che i compiti nominano**: la §2.3 e la §3.1 della spec del sotto-progetto 1 passano da sei a sette famiglie (compito 4). Nessun'altra riga: `git diff --name-only 42b50d8..HEAD -- docs/superpowers/specs/2026-08-06-kernel-design.md` resta vuoto a ogni compito ⛔ **RICHIAMO DEL 2026-09-11, dal pre-controllo del compito 4 (P-22): i posti sono TRE, non due.** Oltre alla §2.3 (riga 575) e alla §3.1 (riga 906), la riga **209** apre con *«Le famiglie di porte restano sei»* al presente e in assoluto, e dal compito 4 è falsa alla lettera benché il suo merito — l'anello 3 non ne aggiunge — resti vero. Si toccano **tutte e tre nello stesso commit** (quinta riga della disciplina dell'audit). ✅ **E la riga 1527 NON si tocca:** *«nessuna delle famiglie di porte della §2.3 fornisce la capacità dell'hardware»* resta vera, perché `custody` custodisce byte e non interroga la GPU — scritto qui perché il prossimo censimento non la corregga per zelo. | §3 della stella polare, decisione 15; `CLAUDE.md` |
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
| **2** | il **trasporto `ipc`** in `platform` su `interprocess`, e la **suite di conformità** `ipc_contract.rs` inclusa da `platform` coi bugiardi; ⛔ **più i tre richiami che `ipc` rende falsi** — richiamo del 2026-09-11, **P-23** | uno | ⬜ |
| **3** | lo **schema che cresce**: le varianti nuove di `IpcMessage`, le **fixture** e il **timbro di build**, `ipc_wire.rs` | uno | ⬜ |
| **4** | la **settima porta**: il tratto `Custody` in `kernel::ports`, la finta di `ports_are_implementable.rs`; i richiami alle cifre in prosa di `ports/mod.rs` con la guardia di `rng` (P-21) e i **tre** nella spec — la riga dell'anello 3, la §2.3 e la §3.1 (P-22). ⛔ **La suite di conformità è al 5 — richiamo del 2026-09-11, D13** | uno | ⬜ |
| **5** | le **due implementazioni** della settima porta: `redb` in `platform`, la finta in `simulator`, **e la suite di conformità che le confronta** — arrivata qui dalla riga 4 col richiamo del 2026-09-11 (**D13**): una suite ne vuole due, e al 4 ce n'erano zero | uno | ⬜ |
| **6** | il **registro delle funzioni** `kernel::registry`: la funzione registrata, `invoke`, il dettaglio `Invocation` col suo record congelato | uno | ⬜ |
| **7** | l'**attività del kernel che serve la GUI** — `kernel::serving`: il dispaccio, il ramo `Request` **non servito** (D5), `Disconnected`, il tick in `Parameters`. ⛔ **Il limite di giri è al 8 — richiamo del 2026-09-11, D21** | uno | ⬜ |
| **8** | il **daemon**: il cablaggio dell'attività, il percorso dell'archivio come argomento, la **rilettura della policy all'avvio** col dettaglio tipizzato, «salva, riavvia, ritrova», **e il limite di giri** — arrivato qui dalla riga 7 col richiamo del 2026-09-11 (**D21**): la sua sonda vuole il grafo con la GUI | uno | ⬜ |
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

### P-16 — L'innesco della revoca `core -> gui` SCADE OGGI e non va onorato: il doc nomina il guscio, il meccanismo vuole un destinatario

**Domanda 6 — ciò che ti smentisce sta in un COMMENTO**, la terza volta in questo pre-controllo. Il doc del modulo
`crates/kernel/src/wire/ipc.rs` dichiara la revoca `core -> gui` una **non-costruzione dichiarata** e le scrive
l'innesco: *«ITS TRIGGER IS THE SAME SHELL … a revocation needs an ADDRESSEE, and until milestone 2 of the
subproject there is nobody to tell»*. ⛔ **Il guscio è arrivato** — ADR-0029 `Accepted` il 2026-09-10, Electron —
quindi letto alla lettera l'innesco è **questo piano**, e la §4 del 2 non elenca nessuna variante di revoca fra le
tredici: sarebbe un buco di copertura.

**Non lo è, e la ragione è nel piano stesso:** **D5** non serve `Request`, quindi nel 2 **nessuna concessione
ordinaria esiste**, e il destinatario che la frase chiede continua a non esserci. L'innesco scritto è **più largo
del meccanismo**: nomina il guscio, mentre ciò che serve è il **consumatore 3D** — lo stesso chiusore che la riga
27 del Traguardo 6 porta già in `porta-di-qualita.md`, il **7**.

⚠️ **È gotcha #77 una seconda volta nello stesso file:** una scadenza scritta in prosa, per cui nulla può andare
rosso. Il doc della porta `crates/kernel/src/ports/ipc.rs` ne ha già corretta una identica il 2026-08-31 — *«A
DEADLINE WRITTEN IN PROSE (gotcha #77), and this is the run in which it falls due»* — e questa è la gemella, nel
file accanto.

**Conseguenza per questo piano:** nessuna variante nuova; il **richiamo datato** del compito 3, passo 9 (a). E
`ports/ipc.rs` **non si tocca**: il suo richiamo dice già che l'innesco vive «accanto allo schema, UNA casa».

### P-17 — Il doc di `IpcMessage::encode` argomenta su un GRAFO che questo compito allarga

`crates/kernel/src/wire/ipc.rs`, doc di `encode`: un'encodifica fallita diventa un **corpo vuoto** invece di un
errore, e l'argomento è letto *«contro il grafo di QUESTO tipo invece che copiato»* — grafo che il doc enumera:
*«`Mib(u64)`, `ComputeClass`, `Preemption`, `Millis(u64)` e varianti unit»*. ⛔ **Le varianti nuove del compito 3
ci mettono dentro `String` e `Vec<u8>`**, quindi l'enumerazione smette di descrivere il tipo.

✅ **Riletto alla fonte il 2026-09-11**, in `bincode` 2.0.1, `src/error.rs` — la crate è già scaricata,
`ls -d ~/.cargo/registry/src/*/bincode-2.0.1`. Le varianti di `EncodeError` raggiungibili **senza `std`** sono
quattro: `UnexpectedEnd`, `RefCellAlreadyBorrowed`, `Other(&'static str)` e `OtherString(String)` dietro `alloc`;
le altre sono dietro `#[cfg(feature = "std")]` o `serde`. **Nessuna è producibile** da un `Encode` **derivato**
sopra `String` e `Vec<u8>` che scrive in un `Vec` che cresce: le ultime due nascono solo da un'implementazione
**scritta a mano**, che qui non esiste.

**Quindi la relazione regge e l'enumerazione no** — ed è la distinzione che questo repository usa altrove: *«la
relazione sopravvive a una variante aggiunta; un elenco no»*, scritta nel doc stesso a proposito di
`#[non_exhaustive]`. **Conseguenza:** il **richiamo datato** del compito 3, passo 9 (b), **più una sonda** —
`a_string_in_the_schema_still_cannot_stop_the_encoder` — perché fin qui era **dedotto leggendo**, e una lettura
non è una misura.

### P-18 — Le fixture vanno in `gui/`, che nasce al compito 10

La §4 del 2 e la riga «lo schema» della tabella degli artefatti della §8 dicono entrambe che le fixture si
committano **in `gui/`**. ⛔ **`gui/` non esiste**, misurato il 2026-09-11 (`ls gui` → niente), e a crearla è il
**compito 10**, sette compiti dopo. Un compito che scrive in una cartella che nascerà è la specie di dipendenza
all'indietro che il taglio per artefatto (D1) esiste per evitare.

✅ **Verificato che non ci siano effetti collaterali:** `git check-ignore -v gui/schema/fixtures/x.bin` non rende
nulla — `gui/` **non è ignorata** oggi, quindi i file si committano; e la cartella che il compito 3 crea porta
**solo dati**, nessun `Cargo.toml` e nessun `package.json`, quindi `cargo` non la vede e il manifesto di radice
non va toccato. **Conseguenza: D12.**

### P-19 — Le varianti nuove portano tipi del kernel che NON parlano `bincode`: il disegno dice a parole ciò che il codice non lascia fare

**Domanda 3 — l'artefatto è sbagliato, e non compilerebbe.** La §4 del 2 dice che `Degradation` porta *«i due
campi di oggi»*, `Policy` *«quale policy VRAM è attiva»*, `PermissionRequired` *«la tripla»*. Misurati i quattro
tipi il 2026-09-11:

| Tipo | Derive di oggi | Sul filo? |
|---|---|---|
| `degradation::Degradation` (`degradation.rs:23`) | `Debug, Clone, Copy, PartialEq, Eq` | **no**: nessun `bincode` |
| `permission::Permission` (`permission.rs:112`) e `Operation` (`:46`) | idem | **no**, e non potrebbe: `tool` e `resource` sono `&'static str` |
| `arbiter::policy::VramPolicy` (`policy.rs:63`) | **nessuno** | **no**, e non potrebbe: le varianti contengono le policy stesse |
| `record::Trust` (`record.rs:162`) | `… Encode, Decode` + `#[cbor(index_only)]` — **minicbor**, non `bincode` | il disegno ordina già un **gemello** |

⚠️ **E il repository porta un precedente che sembra dire il contrario:** `Mib` ha **entrambe** le lingue sullo
stesso derive — `Encode, Decode, bincode::Encode, bincode::Decode` con `#[cbor(array)]`,
`crates/kernel/src/arbiter/resource.rs:50-61` — ed è già sul filo dentro `GrantRequest`. Due precedenti opposti,
e il disegno ne copre **uno su quattro**.

⛔ **Portato al proprietario in A/B il 2026-09-11, che ha delegato — *«scegli secondo decision-principles»*.** La
scelta e la regola che distingue i due precedenti sono **D11**. ⚠️ **E non è una preferenza: per `Permission` e
`VramPolicy` il gemello è FORZATO**, il primo dall'argomento che questo stesso file scrive contro
`ResourceProfile` (`&'static str` da byte in arrivo, ADR-0014), il secondo perché non è serializzabile per
costruzione.

---

### P-20 — La suite di conformità della settima porta NON può nascere al compito 4: le vuole due, e al 4 ce n'è zero

**Domanda 2 — la sonda manca, e non si vede leggendo.** La riga 4 della tabella della posizione dà al compito 4
*«il tratto `Custody`, la finta di `ports_are_implementable.rs`, la suite di conformità»*, e la riga 5 dà al 5 le
**due implementazioni**. ⛔ **Una suite di conformità confronta due implementazioni**, e lo scrive
`crates/kernel/src/ports/mod.rs` di sé: la finta di `ports_are_implementable.rs` *«buys that the signatures compile
FROM OUTSIDE THE CRATE and can be called; it does not buy that they are the right signatures, and IT IS NOT THE
CONFORMANCE SUITE, WHICH NEEDS TWO IMPLEMENTATIONS TO COMPARE»*. Al compito 4 le implementazioni sono **zero**: la
suite girerebbe su nulla, o su una finta minima che non pretende di essere conforme.

✅ **Letto il precedente invece di dedurlo**, il 2026-09-11: `crates/kernel/tests/journal_contract.rs` è
`include!`-ata da `crates/platform/tests/journal_contract_real.rs:32` ed esercita l'implementazione **vera**. E la
sua testa registra che il file **è nato prima** di essere incluso, col tempo futuro — *«this file WILL BE
`include!`d … task 9 of this milestone»* — e ha poi richiesto un **richiamo datato** per correggerlo: *«A tense is
a status claim like any other (gotcha #31)»*. È una lezione già pagata una volta.

**Conseguenza: D13.** ⚠️ **E il compito 4 resta un artefatto provato da solo**, che è ciò che D1 chiede: la finta
in `ports_are_implementable.rs` prova che il tratto è implementabile **da fuori dalla crate**, cioè la **terza**
domanda del pre-controllo di `CLAUDE.md` in persona.

### P-21 — `ports/mod.rs` porta una GUARDIA CONTRO chi scrive «sette famiglie», ed è per un'altra ragione

**Domanda 6 — ciò che ti smentisce sta in un COMMENTO**, la quarta volta in questo pre-controllo, e questa è la
più insidiosa perché **vieta letteralmente ciò che il compito deve fare**. `crates/kernel/src/ports/mod.rs:76-79`,
riletto il 2026-09-11:

> `rng` IS DECLARED IN §2.2 AND LIVES IN `crate::rng`, NOT HERE … the simulator substitutes SEVEN things while
> §2.3 enumerates SIX, and §3.1 says so in those words. Repeated here so that nobody "fixes" the discrepancy by
> moving `rng` under this module, **or by writing "seven families" in the line above**.

⛔ **Chi esegue il compito 4 legge quella riga e ha due modi di sbagliare**: fermarsi credendo di violare una
guardia, oppure ignorarla — e allora la guardia smette di valere anche per il caso che esiste per cogliere.
**Non sono lo stesso «sette»:** quello vietato conterebbe `rng`, che non è una famiglia di I/O; quello del compito
4 conta una famiglia **vera**, `custody`. ⚠️ **E la discrepanza non sparisce, si SPOSTA:** dopo il compito 4 il
simulatore sostituisce **otto** cose mentre le famiglie sono **sette**, quindi la riga va riscritta **coi numeri
nuovi**, non cancellata — cancellarla rimetterebbe in piedi il «fix» che essa impedisce.

**Conseguenza per il compito 4:** il richiamo datato su quel capoverso dice le tre cose insieme — `rng` resta
fuori, il sette di oggi è un'altra cosa da quello vietato, e i numeri diventano otto contro sette. **Nessuna D:**
è un rimedio, non una scelta.

### P-22 — La frase «le famiglie restano sei» vive in TRE posti della spec, e il vincolo 1 ne nomina due

**Riga 5 — il contratto cresce sotto il piano**, applicata al vincolo globale. Il vincolo 1 dice *«la §2.3 e la
§3.1 della spec del sotto-progetto 1 passano da sei a sette famiglie (compito 4). **Nessun'altra riga**»*.
Censite le occorrenze il 2026-09-11 in `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`, col `grep`
sulla frase e leggendo **intera** ogni riga che il censimento rende (gotcha #70):

| Riga | Che cosa dice | Dopo il compito 4 |
|---|---|---|
| **575**, §2.3 | *«I/O — le famiglie di porte»*, la sezione che le enumera | **da toccare**, come il vincolo 1 dice |
| **906**, §3.1 | *«Cosa sostituisce»* — le sei enumerate e le sette sostituite | **da toccare**, come il vincolo 1 dice |
| **209** | *«✅ **Le famiglie di porte restano sei.** L'anello 3 **non** ne aggiunge una»* | ⛔ **DA TOCCARE, e il vincolo 1 non la nomina.** Il **merito** resta vero — l'anello 3 non ne aggiunge — ma la frase d'apertura è al **presente e in assoluto**, e dal compito 4 è falsa alla lettera. È la radice **R1** dell'audit: *«una correzione attraversa il documento in cui nasce, non gli altri»* |
| **1527** | *«nessuna delle famiglie di porte della §2.3 fornisce la capacità dell'hardware»* | ✅ **resta vera**: `custody` custodisce byte, non interroga la GPU. **Non si tocca**, e sta scritto qui perché chi censisce la ritrovi e non la tocchi due volte |

**Conseguenza:** il vincolo globale 1 riceve il proprio **richiamo datato** — tre posti, non due — e il compito 4
li tocca tutti e tre **nello stesso commit**, che è la quinta riga della disciplina dell'audit: *«un rimedio si
chiude su TUTTE le case della frase, non su quella dove l'hai trovata»*.

---

### P-23 — Il compito 2 dà a `ipc` un'implementazione vera e lascia TRE righe false, in due file che non tocca

**Riga 5 — il contratto cresce sotto il piano**, e questa volta sotto il piano **stesso**: il difetto è del
compito **2**, e si vede solo scrivendo il **4**, perché è il 4 che apre quei due file. Misurato il 2026-09-11:
la lista *Files* del compito 2 non nomina né `crates/kernel/src/ports/mod.rs` né
`crates/kernel/tests/ports_are_implementable.rs`, e `grep -n 'ports_are_implementable\|FIVE fakes'` dentro il
compito 2 non rende **nulla**. Ma il compito 2 crea `platform::ipc::LocalSocketIpc`, e da quel momento:

| Dove | Che cosa dice | Dopo il compito 2 |
|---|---|---|
| `crates/kernel/tests/ports_are_implementable.rs:1` | *«One fake per port declared **WITHOUT an implementation**»* | **falsa per `ipc`**: una ce l'ha |
| `crates/kernel/src/ports/mod.rs:22-25` | *«The other FOUR — `filesystem`, `network`, `process` and `ipc` — have **NO CALLER AT ALL**»* | **falsa per `ipc`**, e il conteggio scende a **tre** |
| `crates/kernel/src/ports/mod.rs:18`, la tabella | riga `ipc`, colonna *«Real implementation arrives in»* → *«milestone 6»* | **falsa**: arriva col **sotto-progetto 2**, compito 2 |

⛔ **E non è un dettaglio di prosa: è la tabella che `ports/mod.rs` dichiara essere IL DISEGNO** — *«THE TABLE IS
THE DESIGN, NOT AN INVENTORY OF FILES»* — e la frase delle quattro senza chiamante è l'argomento con cui il banco
delle finte esiste. Lasciarle false toglie il significato a entrambe.

**Conseguenza: la lista *Files* e i passi del compito 2 crescono di due file e di tre richiami datati**, scritti
nel compito 2 col richiamo che dice **perché** sono arrivati lì da un pre-controllo successivo. ⚠️ **Corretto
adesso e non con una voce d'errata**, e la differenza è che il compito 2 **non è eseguito**: un'errata è per ciò
che un compito eseguito ha smentito, e qui il piano è ancora in scrittura — correggere costa due righe, scoprirlo
eseguendo costa un compito rifatto.

### P-24 — Il `grep` di `platform/src/lib.rs` è un ELENCO CHIUSO travestito da comando, e non potrà MAI trovare `Custody`

**Domanda 6 — ciò che ti smentisce sta in un COMMENTO**, e qui il commento **dichiara di essere un comando**, che è
la forma peggiore: chi legge si fida e non ricontrolla. Il doc di modulo di `crates/platform/src/lib.rs`, riletto
il 2026-09-11, dice *«the list is not written here as a fixed set, because milestone 6 adds to it; it comes from
`grep -rEn "^impl (Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for " crates/platform/src/`, which answered
those three on 2026-08-28»*.

⛔ **Il regex enumera SETTE nomi di tratto, e `Custody` non è uno di essi.** Quindi la frase *«non è un elenco
fisso»* è falsa: l'elenco **è** il regex. Rilanciato il 2026-09-11 rende tre righe — `Journal`, `Reactor`, `Rng` —
e dopo il compito 5 ne renderà ancora tre mentre le implementazioni sono quattro: il comando non sbaglia il
conteggio, **non vede proprio il file**.

⚠️ **E la cura NON è aprire il regex**, che è la prima cosa che viene in mente. Misurato il 2026-09-11:
`grep -rEn "^impl [A-Za-z_]+ for " crates/platform/src/` rende cinque righe, e due non sono porte del kernel —
`impl StorageBackend for FileBackend` e `impl Default for SequentialRng`. Un regex aperto scambia il rumore per
copertura, che è l'errore opposto e altrettanto silenzioso. La cura è **aggiungere `Custody` all'enumerazione e
dire che è un'enumerazione**: chi aggiunge la porta è l'unico che può saperlo, esattamente come per la §1.0 nel
compendio (gotcha #40).

⛔ **E c'è una TERZA falsità nella stessa frase, che è una scadenza in prosa — gotcha #77.** *«because milestone 6
adds to it»*: il Traguardo 6 è **chiuso il 2026-09-02** e non ha aggiunto nulla — `ls crates/platform/src/` rende
`journal.rs`, `lib.rs`, `reactor.rs`, `rng.rs`. La ragione per cui l'elenco sarebbe dovuto crescere è scaduta senza
che nulla diventasse rosso, ed è la gemella di **P-16**, due file più in là.

✅ **Esaminata e NON toccata:** la riga `process` della tabella di `crates/kernel/src/ports/mod.rs` dice ancora
*«milestone 6»* ed è falsa per lo stesso motivo — ma è un'**altra frase**, di un'altra famiglia, e il suo chiusore
è il sotto-progetto **12**, il primo worker vero. Toccarla qui allargherebbe il perimetro che il vincolo 1
delimita. Scritto perché il prossimo censimento la ritrovi e sappia che è stata **vista**, come la riga 1527 in
**P-22**.

**Conseguenza per il compito 5:** `crates/platform/src/lib.rs` entra nella lista *Files* per **due** cose — la riga
di modulo e il **richiamo datato** sulla frase — e il richiamo dice le tre cose insieme: `Custody` nel regex,
l'enumerazione dichiarata tale, la scadenza sostituita da un fatto.

### P-25 — Il compito 2 lascia FALSA la prosa di `platform/src/lib.rs`, in un file che GIÀ modifica

**Riga 5 girata all'indietro** — *«i compiti PRIMA del mio lasciano false delle righe nei file che io apro?»*, la
trappola che la seconda chiusura ha scritto dopo **P-23**. È la stessa specie un compito più in là, e questa volta
il file è **già nella lista** del compito che lo rompe: il Passo 9 del compito 2 aggiunge `pub mod ipc;` e **non
tocca il doc di modulo sopra**, che da quel momento dice *«Today they are `Journal` …, `Reactor` … and `Rng` …»*
mentre sono quattro.

⚠️ **E per `Ipc` il regex FUNZIONA** — `Ipc` è fra i sette nomi — quindi dopo il compito 2 il comando risponde
quattro e la prosa accanto dice tre: le due metà della stessa frase si contraddicono, che è la radice **R1**
dell'audit nella sua forma più piccola.

⛔ **Corretto nel compito 2 e non con una voce d'errata**, per la stessa ragione di **P-23**: il compito 2 **non è
eseguito**. Un'errata è per ciò che un compito eseguito ha smentito.

**Conseguenza:** la lista *Files* e il Passo 9 del compito 2 crescono di un **richiamo datato** sulla frase.
⚠️ **Il compito 5 NON lo rifà**: quando arriva, la frase nomina già `Ipc`, e il richiamo del 5 si aggiunge
**sotto** quello del 2 senza cancellarlo — due richiami datati nello stesso capoverso, la forma che
`crates/kernel/src/ports/mod.rs` porta già dal 2026-08-28.

### P-26 — `OpenError` si RIUSA, e il suo percorso è citato da DUE doc del kernel come precedente di forma

**Domanda 3 — l'artefatto è sbagliato, e compila.** La §2 della stella polare dice che l'implementazione vera è
*«`redb` sul `FileBackend` che il giornale già usa — è `pub`, con `open(path)`»*, e **tace sull'errore di
apertura**. Scrivere un `custody::OpenError` gemello compilerebbe benissimo e sarebbe una seconda definizione delle
stesse tre varianti.

Misurato il 2026-09-11 con `grep -rn "OpenError" crates/ --include=*.rs`, tolto il file che lo definisce: le case
fuori da `crates/platform/src/journal.rs` sono cinque, e **due sono doc del KERNEL** che ne citano il percorso per
esteso — `crates/kernel/src/framing.rs` e `crates/kernel/src/permission.rs`, la seconda con *«The shape is
`platform::journal::OpenError`'s, which composes `io::Error`»*. Le altre tre sono `crates/daemon/src/main.rs`
(`StartupError::Journal(OpenError)`), `crates/platform/tests/file_journal.rs` e un commento in
`journal_contract_real.rs`.

⛔ **Quindi SPOSTARLO in `platform::OpenError` renderebbe false due righe del kernel**, in due file che questo
compito non apre — la radice **R1** commessa per fare ordine. E **duplicarlo** darebbe due tipi da tenere in passo,
di cui il primo che diverge mente in silenzio.

⚠️ **E la funzione `engine` di `journal.rs` è PRIVATA**, misurato: `fn engine(error: impl Into<redb::Error>) ->
OpenError`. La custodia le serve la stessa conversione, quindi o sale a `pub(crate)` — una parola — o si riscrive.
**Conseguenza: D14.**

### P-27 — DENTRO QUESTO PIANO ci sono già DUE forme di suite di conformità, e vanno tenute distinte

**Riga 5 — il contratto cresce sotto il piano**, e qui cresce sotto il piano **stesso**. La §2 della stella polare
dice *«la suite di conformità sulle due implementazioni coi bugiardi, **come `journal_contract`**»*, e
`journal_contract.rs` è una **funzione** — `pub fn assert_journal_contract<J: Journal, F: Fn() -> J>(build: F)` —
più un `#[test]` per la finta in memoria, il tutto raggiunto con `include!`. Ma il **compito 2** ha scelto un'altra
forma per `ipc`: `include!` **più una `macro_rules!`** che genera i `#[test]`, e il suo Passo 9 scrive che il banco
del kernel *«riporta zero test — è voluto»*.

⚠️ **Non è un'incoerenza da sanare, e la causa è scritta nel compito 2:** la suite di `ipc` dichiara *«what is
deliberately absent: every promise that needs a real peer writing bytes»*, quindi le sue promesse non girano contro
una fabbrica sola e ogni crate porta la propria. `Custody` **non ha un pari**: una fabbrica basta, e `kernel` può
costruire `simulator::custody::MemoryCustody` da sé perché `simulator` è già una sua **dev-dependency** —
verificato il 2026-09-11 in `crates/kernel/Cargo.toml`, dove il ciclo `kernel -> (dev) -> simulator -> kernel` è
dichiarato e *«run once rather than assumed»*.

**Conseguenza: D15**, e la testa di `custody_contract.rs` scrive perché le forme sono due, così che un revisore non
le «uniformi».

### P-28 — Delle sette prove che la riga della §8 elenca per la settima porta, TRE non sono di questo compito

**Domanda 2 — la sonda manca**, nella sua direzione meno ovvia: qui il rischio è **costruirne troppe**. La riga
*«la settima porta»* della tabella degli artefatti della §8 del 2 ne elenca sette in una cella sola, e il taglio per
artefatto (**D1**) le distribuisce:

| La prova, come la §8 la scrive | Compito |
|---|---|
| la finta in `ports_are_implementable.rs` | **4** — già scritto |
| la suite di conformità sulle due implementazioni coi bugiardi | **5** |
| `redb` in `platform`: apri, scrivi, riapri, rileggi | **5** |
| byte che non sono JSON tornano identici | **5** |
| archivio vuoto → «niente» | **5** |
| `SaveLayout` su un archivio che rifiuta la scrittura → `Layout` col vecchio | **7**, l'attività che dispaccia |
| archivio che non si apre → il core parte e `Layout` dice «non disponibile» (decisione 35) | **8**, il daemon che cabla |

⛔ **Le ultime due non si POSSONO scrivere al 5, e non è una scelta:** parlano di `SaveLayout` e di `Layout`, che
sono **messaggi** — varianti dello schema del compito 3 — e di un core che parte, che è il daemon. Al compito 5 il
dispaccio non esiste. Scritto qui perché un censimento della §8 contro il compito 5 le trovi assenti e sappia che è
voluto, invece di aggiungerle dove non possono vivere.

### P-29 — Con UNA chiave sola la suite NON può cogliere un'implementazione CIECA ALLA CHIAVE: si dichiara e si MISURA

**Domanda 1 — la sonda è vacua.** È la lezione di `StepBlindJournal` (finding **AUD-019**) applicata alla porta
nuova, e questa volta cade dall'altra parte. Il giornale la chiuse mettendo un **passante** in archivio: su un
archivio con un passo solo, *«il record di questo passo»* e *«il primo record che c'è»* sono lo stesso record,
quindi il cercare non era mai messo alla prova. ⛔ **Qui il passante NON È COSTRUIBILE:** `CustodyKey` ha **una
variante**, e su un archivio con una chiave sola *«il pacchetto di questa chiave»* e *«l'unico pacchetto che c'è»*
sono lo stesso pacchetto — **per costruzione, non per come è scritto il banco**.

⛔ **Quindi un `FileCustody` il cui `retrieve` ignorasse la chiave passerebbe la suite intera.** Le tre vie,
esaminate:

| Via | Perché cade |
|---|---|
| una **seconda variante** di `CustodyKey` per il banco | una variante senza chiamante, che è ciò che il doc di `CustodyError` del compito 4 rifiuta con le sue stesse parole. E nessun controllo la coglierebbe: sarebbe **codice**, non un documento |
| scrivere il **bugiardo cieco** e pretendere che sia colto | passerebbe. Un bugiardo che non muore è un verde che non prova nulla — la definizione di sonda vacua |
| **dichiarare il limite e MISURARLO** | è ciò che il repository fa già: la testa di `journal_contract.rs` ha la sezione *«what is deliberately absent»*, e AUD-019 fu chiuso **misurando** che il bugiardo passava, non ragionandoci |

**Conseguenza:** la terza via. La testa di `custody_contract.rs` porta il limite col proprio **innesco** — la
**seconda chiave**, chiunque la aggiunga — e il compito 5 porta un **passo di misura**: si scrive la mutazione
cieca, si lancia la suite, si **osserva che passa**, si revoca. ⚠️ **La misura è metà del valore:** un limite
dichiarato e non misurato è un'ipotesi, e questo repository ha già pagato per la differenza — gotcha **#57**.

⚠️ **E la finta del simulatore NON è cieca alla chiave**, benché nulla lo pretenda: tiene `Vec<(CustodyKey,
Vec<u8>)>` come `MemoryJournal` tiene `Vec<Entry>`, mentre un `Option<Vec<u8>>` lo sarebbe **per costruzione**. È
un argomento, non un rosso, ed è etichettato come tale — la stessa forma di `EntryKind::Note` in
`crates/simulator/src/journal.rs`.

### P-30 — L'«argomento» dell'invocazione arriva come `String` dal filo, e la §5 non dice che tipo abbia nel kernel

**Domanda 3 — l'artefatto è sbagliato, e compila.** La §5 del 2 dà a `invoke` *«nome, invocatore, argomento»* e
tace sui tipi. Ma il compito 3 mette sul filo `Call { function: String, argument: String }`, e **ogni**
costruttore di dettaglio del giornale prende `&'static str` — misurato il 2026-09-11:
`RoutingDetail::new(model: &'static str, …)` e `PermissionDetail::new(tool: &'static str, resource: &'static
str, …)`, entrambi con `String::from` **dentro**. La ragione è scritta in tutti e due: *«a caller cannot hand this
type text it computed at runtime»*.

⛔ **Quindi due strade si aprono, e sono di specie diversa.**

| Cosa arriva | Dove va | Perché |
|---|---|---|
| il **nome** della funzione | **non diventa mai** un `&'static str`: si **confronta** contro i nomi registrati e si scarta | è la disciplina che `permission::is_granted` scrive già — *«the decoded values never leave this function: they are compared against `&'static str` the caller already held, and dropped»*. Nel giornale finisce il nome **registrato**, che è nostro |
| l'**argomento** | il **`payload` del record**, sotto `Trust::Untrusted` | ⛔ **è il precedente di `VerdictDetail`, alla lettera:** *«THE DETAIL TEXT IS NOT HERE: it is untrusted by inheritance (ADR-0014) and travels in the record's `payload`, under the `trust` label that exists to say so. What lives here is what is OURS and structured»*. L'argomento è testo **scelto dalla GUI**: nel dettaglio strutturato non può stare, e nel payload ha già l'etichetta che lo dice |

⚠️ **E ne discende che `InvocationDetail` porta DUE campi e non tre**, benché la §5 dica *«funzione, invocatore,
argomento»*: il terzo c'è, ma nel payload accanto — dove il record ha già un posto per il non fidato. La §5 riceve
il proprio **richiamo datato** al compito 6, insieme a quello di **P-32**.

### P-31 — `invoke` NON può eseguire l'effetto da sé: sarebbe il contenuto dentro il meccanismo

**Riga 7 — un ADR si legge contro i propri fratelli.** La regola **1** di ADR-0038 dice che il kernel dà
*«registrazione, invocazione, il permesso e il giornale»* e che **il contenuto — quali funzioni esistano — lo
portano le capacità e la GUI**, nella forma dei registri di ADR-0009. Ma la §5 del 2 dice *«se sì → l'effetto»* e
nomina `Arbiter::set_policy`, e la sonda che chiede è *«tripla non concessa → `set_policy` mai chiamato»*.

⛔ **Un `kernel::registry` che chiamasse `set_policy` metterebbe il contenuto dentro il meccanismo**, e si
vedrebbe subito: dovrebbe **nominare** `Arbiter` e `VramPolicy`. Il secondo invocatore — il gesto, col 12 —
dovrebbe allora aggiungere il proprio effetto **qui dentro**, che è esattamente la *«logica solo per gesti»* che
l'ADR rifiuta, al rovescio.

**Conseguenza: D16** — `invoke` prende l'**effetto come chiusura**, e la regola 1 sale al **livello 1**: un
registro che nomina l'arbitro non compilerebbe senza importarlo, e l'assenza di quell'`use` è la prova.
⚠️ **E la chiusura riceve il giornale**, perché `set_policy` lo vuole: `E: FnOnce(&mut J) -> Result<T,
JournalError>`. Prestarlo due volte non si può, quindi passa **attraverso**.

### P-32 — La §5 cita la regola SBAGLIATA per l'invocatore, e il costo non è zero

**Riga 5 — il contratto cresce sotto il piano**, e qui cresce contro un artefatto che **non si rigenera**. La §5
dice: *«l'invocatore | un enum con una sola variante oggi … il 12 aggiunge il gesto con un indice nuovo, **senza
cambio di formato** (ADR-0036, regola 3)»*.

⛔ **La regola 3 di §4.9.2 parla di CAMPI** — *«un campo nuovo è facoltativo e prende un indice nuovo»* — e una
variante di enum non è un campo. Ciò che vale per le varianti lo scrive la testa di
`crates/kernel/tests/frozen_bytes.rs`, al suo punto **3**: *«A VARIANT ADDED TO ONE OF THE THREE `index_only`
ENUMS makes every record carrying it UNDECODABLE to an older build. The direction is safe … but it was free only
while the archive did not exist.»*

✅ **Il merito regge — la direzione è sicura** (`Malformed` riconcilia come `SuspendAndAsk`, quindi una build
vecchia si ferma invece di indovinare) — **ma «senza cambio di formato» è falso**, e la citazione manda chi legge
alla regola che non governa il caso. **Richiamo datato sulla riga**, al compito 6.

⛔ **E c'è una conseguenza di progetto, non solo di prosa:** `PermissionDetail` **rifiuta** un enum sul filo con
un argomento scritto per esteso — *«an enum here would be a FOURTH `index_only` enum ON THE WIRE, whose variant
indices `tests/frozen_bytes.rs` would then have to pin ONE PER FROZEN RECORD, and an index on the wire never
retires»* — e sceglie un `bool`. `VerdictDetail` fa lo stesso, per lo stesso motivo. Un `Invoker` messo sul filo
**come enum** sarebbe precisamente quella quarta. **Conseguenza: D17.**

### P-33 — `RecordKind` cresce, e il compilatore ferma DUE posti: censiti, non scoperti

**Domanda 2 — la sonda manca**, e qui il compilatore è la sonda. Censiti i `match` esaustivi su `RecordKind` il
2026-09-11:

| Dove | Che cosa | Che cosa deve diventare |
|---|---|---|
| `crates/kernel/src/reconcile.rs:90` | il `match body.kind()` con sei bracci, quattro dei quali **vuoti col proprio argomento misurato** | un **settimo braccio vuoto**, ⛔ **misurato per QUESTA variante e non ereditato** — è la disciplina che le quattro righe accanto scrivono una per una, e le due sonde in `tests/reconciliation.rs` vanno **nelle due direzioni** |
| `crates/kernel/tests/frozen_bytes.rs:386` | un `match kind` con i sei nomi in un braccio solo | il settimo nome nel braccio |

⚠️ **Sono due e non di più, e la differenza fra i due è che il primo è una DECISIONE e il secondo un elenco:**
`permission.rs`, `degradation.rs` e `gateway/mod.rs` **confrontano** un `kind` con `==` o con un `if let` invece
di fare `match`, quindi crescono senza dire nulla — e va bene, perché ciascuno cerca **una** specie e la trova o
no. ⛔ **Scritto qui perché chi esegue non vada a «uniformarli»**: allargarli sarebbe cambiare tre siti per un
fatto che non li riguarda.

### P-34 — I due campi di un dettaglio non devono ESSERE UGUALI, ed è una lezione già pagata

**Domanda 1 — la sonda è vacua.** Il record congelato di `Permission` porta `tool: "frozen"` e
`resource: "frozen"`, e il commento accanto porta un **richiamo del 2026-09-01** che dice perché è un **buco**:
*«two equal strings at two offsets pin two offsets, and that is false: they pin ONE offset and its mirror
image»* — misurato scambiando i due `#[n(..)]`, nessun byte mosso, workspace verde.

✅ **Per `InvocationDetail` il buco non si ripresenta**, e non per fortuna: i due campi hanno **tipi diversi** —
un testo e un numero — quindi scambiarne gli indici muove i byte e il record congelato lo dice. ⚠️ **Va scritto
accanto al record**, perché il prossimo dettaglio a due campi dello stesso tipo ricadrebbe nel buco, e il
richiamo del 2026-09-01 vive in un file che quel giorno nessuno riaprirà.

### P-35 — «le undici varianti» è FALSO contro l'enum che il compito 3 detta, e vive in otto case dentro questo file

**Riga 5 — il contratto cresce sotto il piano**, e qui il piano smentisce **sé stesso in due punti dello stesso
compito**. Trovato preparando il compito 7, che consuma quelle varianti per nome.

Misurato il 2026-09-11, contando invece di rileggere:

| Che cosa | Il comando | Risposta |
|---|---|---|
| le varianti che `IpcMessage` ha **oggi** | `grep -n "pub enum IpcMessage" -A 40 crates/kernel/src/wire/ipc.rs` | **due**, `Request` e `Verdict` — è **P-10**, e regge |
| i bracci dell'enum che il **compito 3 detta** | il blocco `pub enum IpcMessage` del Passo 2, contati | **quattordici** |
| quindi le varianti **nuove** | quattordici meno due | ⛔ **dodici, non undici** |

⛔ **E il compito 3 si contraddice da solo:** il suo criterio di chiusura pretende
`ls gui/schema/fixtures/*.bin | wc -l` → **14**, cioè **una fixture per variante, tutte e quattordici** — che è
giusto, e che è aritmeticamente incompatibile con *«le undici varianti nuove»* scritte quattro righe sopra. Le due
cifre non sono mai state confrontate perché vivono in due punti diversi dello stesso compito: la radice **R1**
dell'audit, dentro un compito solo.

⚠️ **Da dove viene l'errore, perché non si ripeta:** l'intestazione della §4 del disegno del 2 dice *«approvata il
2026-09-06, **poi allargata di due varianti**»*. Il conteggio fu scritto a dieci, l'allargamento ne portò due, e
qualcuno sommò **uno**. Nessun controllo poteva accorgersene: è una cifra in prosa.

⛔ **LA CURA NON È RIALLINEARE A «DODICI».** È la regola di `CLAUDE.md` — *«un numero misurato non si scrive: si
scrive il COMANDO che lo produce»* — e il gotcha **#68**: un numerale in prosa che descrive un artefatto è già
marcito una volta e marcirà di nuovo alla prima variante che si aggiunge o si toglie. Il numerale si **toglie**
dalle otto case (`grep -c undici` le conta), e dove il conteggio deve essere verificabile lo dice un **comando**,
nel criterio di chiusura del compito 3, accanto a quello delle fixture che già c'è.

⛔ **Corretto nel compito 3 e non con una voce d'errata**, come **P-23** e **P-25**: il compito 3 **non è
eseguito**. ✅ **E l'enum stesso NON si tocca:** i suoi quattordici bracci sono quelli che la §4 del 2 e la §2
della stella polare dettano, nomi compresi (decisione 9), e nessuno di essi è in discussione. Ciò che era falso è
la **prosa che li conta**.

---

### P-36 — «il dispaccio» NON può chiamarsi `dispatch`: `kernel::gateway::dispatch` esiste già

La riga 7 della tabella della posizione chiama questo compito *«il dispaccio»*, ed è la parola giusta in
italiano. In inglese non è libera:

```bash
grep -rnE '\b(fn|struct|enum|mod) (dispatch|session|serve|serving|listen)' crates/ --include='*.rs'
```

Il 2026-09-11 rende `pub fn dispatch` in `crates/kernel/src/gateway/mod.rs`, che è il **gateway
d'inferenza**: dispaccia un token conforme al provider. Un secondo `dispatch` nel kernel, che significa
un'altra cosa, è parola per parola l'ambiguità che **D8** ha rifiutato per `counter`.

⛔ **E `session` è peggio, non meglio:** `crates/kernel/src/permission.rs` apre dichiarando *«NOTHING HERE
IS SCOPED TO A SESSION … there is no session in the kernel»*, e la voce aperta 7 della §9 del 2 assegna
**al 3** il confine di sessione dei permessi. Un `kernel::session` di oggi renderebbe falsa quella riga e
occuperebbe il nome che il 3 vuole.

✅ **Libero e misurato:** `serving` compare **in un solo commento** di `ports/process.rs`, mai come
identificatore. Da qui **D19**.

### P-37 — il tick in `Parameters` rompe ogni chiamante, e NOVE sono `compile_fail` col proprio `.stderr`

La §5 del 2 dice che l'attività *«dorme un **tick** consegnato via `Parameters` (ADR-0034)»*. Il campo
nuovo è un atto dovuto; il suo **prezzo** non era scritto da nessuna parte, e si misura:

```bash
grep -rc 'Parameters::new' crates/ --include='*.rs' | grep -v ':0'
ls crates/kernel/tests/compile_fail/*.rs | while read f; do grep -q 'Parameters::new' "$f" && printf '%s %s\n' "$f" "$(test -e "${f%.rs}.stderr" && echo stderr || echo -)"; done
```

Il 2026-09-11: i siti sono **cinquantasette** in **ventitré** file, e **nove** di quei file sono casi
`compile_fail`, ciascuno col proprio `.stderr`. ⛔ **È la metà che un piano dimentica:** un caso
`compile_fail` che passa tre argomenti a una funzione che ne vuole quattro guadagna un `error[E0061]` che
**non è l'errore che quel caso esiste per cogliere**, e il suo `.stderr` diventa rosso per la ragione
sbagliata. Il vincolo **10** di §11 e il gotcha **#25** dicono che quei file si **leggono**, non si
rigenerano in blocco.

✅ **E la cura non è ovvia in una direzione sola:** aggiornare la chiamata **sulla stessa riga** lascia
il `.stderr` identico; aggiungere un argomento a una chiamata **su più righe** sposta ogni numero di riga
che il `.stderr` cita. Quale dei nove sia quale si misura **file per file**, non si assume — è il passo 3
del compito.

### P-38 — `PolicyReport.allocatable` porta il numero OPPOSTO al suo nome, e i due addendi non esistono nel kernel

Il compito 3 detta `pub struct PolicyReport { policy, allocatable: Mib, total: Mib }`. La riga **2** della
tabella *Stato* della stella polare dice che cosa ci va: *«la policy VRAM attiva e il budget **allocato**
sul totale; il totale è il budget **allocabile**: tutto meno la quota audio e la quota di presentazione»*.

⛔ **Due difetti in una struttura di tre campi.**

| | Il difetto | Misurato |
|---|---|---|
| 1 | il campo si chiama `allocatable` e il disegno gli mette dentro il budget **allocato** — il nome dice il contrario del contenuto, e sul filo un nome sbagliato non ha nessuna guardia | la riga 2 della tabella *Stato*, contro il blocco dettato dal compito 3 |
| 2 | l'altro campo dovrebbe essere `totale meno le due quote`, e **il kernel non ha i due addendi**: `AUDIO_QUOTA` e `PRESENTATION_QUOTA` sono letterali di `crates/daemon/src/main.rs`, e il doc di `Parameters::total_vram` **argomenta di non consegnarli** — *«two fields no kernel decision reads would be dead surface inside the kernel»* | `grep -n 'AUDIO_QUOTA\|PRESENTATION_QUOTA' crates/daemon/src/main.rs` e il doc di `total_vram` |

⚠️ **E il disegno lo marca DEDOTTO di suo:** l'ultima colonna della riga 2 dice *«verificato; le due quote
sottratte visibili: **dedotto**»*. Non è una decisione approvata da contraddire: è un dedotto che questo
piano deve chiudere. Da qui **D20**.

### P-39 — `StepSummary.outcome` è `Option<bool>`, e `Some(false)` non ha nessun produttore

Il compito 3 detta `pub outcome: Option<bool>`, col doc *«`None` while the step is still in doubt»*. Letto
contro il giornale che il compito 6 scrive:

```bash
grep -nE '    pub fn (intent|outcome|note)\(' crates/kernel/src/record.rs
```

`RecordV1::outcome(effect, trust, payload, reason)` **non porta un esito booleano**: un esito scritto dice
che il passo si è chiuso, e non esiste un esito «chiuso male». E dentro `Registry::invoke` (compito 6) un
effetto che fallisce torna col `?` **prima** che `outcome` sia scritto, quindi lascia il passo con intento e
senza esito — cioè **in dubbio**, cioè `None`. ⛔ **Gli stati producibili sono due, non tre**, e il terzo
sarebbe superficie morta in un tipo **sul filo**, dove una variante non si ritira mai (ADR-0036, regola 4).

✅ **Corretto nel compito 3 e non con una voce d'errata**, come P-23, P-25 e P-35: il compito 3 **non è
eseguito**. Il campo diventa `pub done: bool`, e il doc scrive che `false` significa *in dubbio* nel senso di
ADR-0007 — il vocabolario che il kernel ha davvero.

### P-40 — il limite di giri è del compito 8, non di questo

La riga 7 della tabella della posizione elenca *«il limite di giri»*. `EXECUTOR_TURN_LIMIT` vive in
`crates/daemon/src/main.rs`, che è il file del compito **8**, e la §5 del 2 detta per esso una sonda precisa:
*«`daemon` guadagna una sonda: il grafo con la GUI resta vivo oltre centomila giri»*. ⛔ **Quella sonda
richiede il grafo con la GUI**, cioè il cablaggio, cioè il compito 8: al 7 nulla lancia l'attività in
produzione, quindi il cambio sarebbe **inerte** e la sua sonda **non scrivibile**.

✅ **È l'argomento di D13 alla lettera**, che spostò la suite di conformità dal 4 al 5 perché una suite ne
vuole due implementazioni e al 4 ce n'erano zero. Stessa forma, stesso rimedio: la riga 7 perde le parole
«il limite di giri», la riga 8 le guadagna, col richiamo datato. Da qui **D21**.

### P-41 — `Protection` «consegnato» sarebbe un parametro con un solo valore possibile

La §5 del 2 dice che `Accepted` porta *«la protezione dell'archivio (valore che `platform` conosce,
consegnato)»*. Letto contro il compito 3, che detta il tipo:

```bash
grep -n 'pub enum Protection' -A 4 crates/kernel/src/wire/ipc.rs
```

`Protection` ha **una** variante, `AsSystemAccount`. Un parametro consegnato che può assumere un valore solo
non è un parametro: è superficie morta dentro `Parameters`, che il doc di `total_vram` rifiuta con quelle
parole esatte, e la friction di §2.8.5 — *«adding a parameter breaks every caller»* — si pagherebbe
**cinquantasette volte** per un valore che non può variare.

✅ **La forma del repo è «dichiarato, non pinzato»** (gotcha #73): l'attività manda
`Protection::AsSystemAccount` e il doc scrive **l'innesco** — il giorno che `Protection` guadagna una seconda
variante, il valore diventa consegnato e questo doc va con esso. Da qui **D22**. ⚠️ **Divergenza dal
disegno, dichiarata e non silenziosa:** la §5 riceve il proprio richiamo datato al passo 7 del compito.

### P-42 — come l'attività si accorga che il degrado è cambiato: il disegno DELEGA al piano, e la sonda del §7 esclude una delle due vie

La §7 del 2 lascia questo esplicitamente aperto, nei debiti dichiarati: *«perché il `degrade` arrivi alla
GUI, l'attività vera deve **accorgersi** che `degradation_now` è cambiato e mandare `Degradation` — è la riga
«rimanda il pezzo che è cambiato» della §5, e il finto è il primo che la esercita: **come** se ne accorga, a
ogni giro o dopo ogni scrittura, lo fissa il piano del 2 con la sonda della §8»*.

⛔ **Le due vie non sono equivalenti, e a scegliere è il rubinetto del core finto.** La parola `degrade`
scrive nel giornale in memoria **dalla seconda attività** — il rubinetto — e non attraverso il dispaccio
(§7, la riga del rubinetto). Una rilevazione *«dopo ogni scrittura mia»* non vedrebbe mai quella scrittura, e
la sonda che la §7 pretende — *«`degrade` → il pari riceve `Degradation`»* — sarebbe **rossa per
costruzione**.

✅ **Quindi: a ogni giro**, e solo quando c'è almeno un client accolto, confrontando col valore che quel
client ha già ricevuto. ⚠️ **Il costo è quello che `degradation_now` dichiara di sé** — rilegge tutto il
giornale — e il **chiusore è già scritto e non si inventa qui**: è lo stesso checkpoint che `Journal::replay`
nomina, nella voce aperta *«`replay()` carica TUTTO in memoria»* della §6 del compendio. Da qui **D23**.

---

### P-43 — `Registry::invoke` NON ha una via per l'`Approve`, e il suo stesso controllo la chiude

È la quinta domanda girata all'indietro, e ha pagato una terza volta: il compito **6** è scritto, e ciò che
detta rende il compito 7 **non scrivibile** su una delle due strade della sequenza 3.

`Registry::invoke` (compito 6) apre così:

```rust
        if !permission::is_granted(journal, &function.permission)
            .map_err(InvokeError::Permission)?
        {
            return Err(InvokeError::PermissionRequired(function.permission));
        }
```

⛔ **Sulla via dell'`Approve` quel controllo risponde ancora NO.** La decisione **21** della stella polare
dice che dopo un `Approve` il core *«apre il passo A, scrive `Invocation`, poi `Permission` con `grant`, poi
l'effetto, poi l'esito»*: il permesso si posa **dentro** il passo A, perché `permission::grant` scrive una
**nota** e vuole un passo già aperto — è il suo doc, ed è la ragione per cui la decisione 21 esiste. Ma
`invoke` chiede `is_granted` **prima** di aprire il passo, quindi al secondo giro risponde di nuovo
`PermissionRequired` e il giro non si chiude mai.

⛔ **E le tre vie che NON si prendono, ciascuna col suo difetto misurato contro il codice:**

| | La via | Perché cade |
|---|---|---|
| 1 | far scrivere il `grant` alla **chiusura dell'effetto**, che gira già nello slot giusto — fra la nota e l'esito | il controllo `is_granted` scatta **prima**, quindi l'effetto non viene mai raggiunto |
| 2 | `grant` su un passo **a parte**, prima di `invoke` | contraddice la decisione 21 alla lettera, e `grant` vuole comunque un passo aperto: ne servirebbe uno di comodo, cioè un passo che non è un'interazione col mondo — ADR-0007 |
| 3 | il dispaccio scrive **da sé** la fila intento → nota → grant → effetto → esito | è una **seconda copia** della sequenza che `invoke` possiede, e il giorno che le due divergono nulla diventa rosso. È ciò che il §7 del 2 rifiuta per il core finto, con le stesse parole |

✅ **La cura è nel compito 6, non in un'errata** — il compito 6 **non è eseguito**, come P-23, P-25, P-35 e
P-39: `invoke` guadagna un argomento `approval`, e l'enum `Approval` dice quale delle due strade è. Con
`Approval::JustGiven` il registro **non chiede** e scrive il `grant` su A, fra la nota `Invocation` e
l'effetto — cioè la decisione 21 tenuta **dentro il meccanismo** che possiede l'ordine, invece che nel
chiamante. ⚠️ **Un enum e non un `bool`**, che è la lezione già pagata da `Access` e da `Operation::is_write`
nel compito 3.

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
| **D11** | ⛔ **un tipo del kernel raggiunge il filo TALE E QUALE solo se è CHIUSO** — un newtype su un numero o un enum senza dati, che non può guadagnare campi; **ogni tipo con CAMPI porta un gemello** in `kernel::wire::ipc`, e la conversione è il posto dove un campo nuovo del giornale si ferma | la regola distingue i **due precedenti opposti** del repo invece di sceglierne uno: `Mib` porta entrambe le lingue perché è un numero con un nome e non può crescere; `Trust` vuole il gemello perché il disegno lo dice. ⛔ **E per due dei quattro non è una scelta** (P-19): `Permission` porta `&'static str`, che non nasce da byte in arrivo — l'argomento che `GrantRequest` scrive contro `ResourceProfile`, ADR-0014 — e `VramPolicy` contiene le policy stesse. ⚠️ **Il beneficio è il compilatore:** ADR-0036 vuole che il giornale **evolva**, I4 rinuncia al versionamento sul filo, e senza gemelli un campo aggiunto al giornale cambierebbe i byte del filo **in silenzio**. ⚠️ **Costo:** dieci tipi nuovi in `wire::ipc` e una conversione per variante. ⚖️ **Portata al proprietario in A/B il 2026-09-11 e delegata** — *«scegli secondo decision-principles»* |
| **D12** | le **fixture nascono al compito 3**, che crea `gui/schema/fixtures/` con **soli dati** — nessun `package.json`, nessun `Cargo.toml` | la §4 e la §8 del 2 le vogliono in `gui/`, che però nasce al **compito 10** (P-18). Le tre vie: spostare il compito 3 dopo il 10 romperebbe il taglio per artefatto (D1) e lascerebbe lo schema senza controllo per sette compiti; farle nascere in `crates/kernel/tests/` e copiarle contraddirebbe il disegno e creerebbe **due case**; crearle dove il disegno dice è il minimo. ✅ **Verificato che non ci siano effetti collaterali:** `gui/` non è ignorata, e senza manifesti `cargo` non la vede. ⚠️ **Costo:** il compito 10 trova la cartella già lì e ci costruisce intorno, invece di crearla vuota |
| **D13** | ⛔ **la suite di conformità della settima porta nasce al compito 5, non al 4**: il 4 porta il tratto, la finta di `ports_are_implementable.rs` e i tre richiami; il 5 porta le due implementazioni **e** la suite che le confronta | una suite di conformità confronta **due** implementazioni, e `crates/kernel/src/ports/mod.rs` lo scrive di sé; al compito 4 ce ne sono **zero** (P-20). Le due vie scartate: farla nascere al 4 **col tempo futuro** è ciò che `journal_contract.rs` fece davvero e che ha richiesto un richiamo datato — *«a tense is a status claim like any other»*, gotcha #31, lezione già pagata; farla nascere al 4 su una finta minima la renderebbe **vacua**, che è la prima domanda del pre-controllo. ⚠️ **Il compito 4 resta provato da solo**, che è ciò che D1 chiede: la finta prova il tratto **da fuori dalla crate** — la terza domanda di `CLAUDE.md` in persona. ⚠️ **Costo:** il compito 5 cresce di un artefatto, e la riga 4 della tabella della posizione perde le parole «la suite di conformità» |
| **D14** | ⛔ **`FileCustody::open` rende `platform::journal::OpenError`, il tipo che c'è**, e `crates/platform/src/journal.rs` entra nella lista *Files* del compito 5 per **una parola**: `engine` passa da privata a `pub(crate)` | le tre varianti — `File(io::Error)`, `AlreadyOpen`, `Engine(redb::Error)` — sono esattamente quelle che *aprire un archivio `redb` su un `FileBackend`* può produrre, e la custodia apre la stessa cosa. Le due vie scartate (**P-26**): **spostarlo** in `platform::OpenError` renderebbe false due righe di doc **del kernel** che ne citano il percorso come precedente di forma — la radice **R1** commessa per fare ordine; **duplicarlo** darebbe due tipi da tenere in passo, e il primo che diverge mente in silenzio. ⚠️ **Costo dichiarato:** il percorso si legge male — un errore della custodia raggiunto attraverso il modulo `journal` — e per questo il doc di `FileCustody::open` lo spiega dove chi lo incontra lo cerca. Se un **terzo** archivio arriverà, allora il tipo sale di un livello e i due doc del kernel si correggono nello stesso commit |
| **D15** | ⛔ **la suite della settima porta prende la forma del GIORNALE** — una funzione `assert_custody_contract` più `include!` — **e non quella della suite `ipc` del compito 2**, che è `include!` più una `macro_rules!` | la §2 della stella polare dice *«come `journal_contract`»*, e la ragione regge alla lettura: la suite di `ipc` genera i `#[test]` con una macro perché **ogni crate porta la propria fabbrica** — le sue promesse vere vogliono un pari che scrive byte, e una finta non ne ha uno. `Custody` non ha un pari: una fabbrica basta, e `kernel` costruisce `simulator::custody::MemoryCustody` da sé perché `simulator` è già sua **dev-dependency** (**P-27**). ⚠️ **Costo dichiarato:** due forme di suite convivono in questo piano e un revisore può leggerlo come un'incoerenza; per questo la testa di `custody_contract.rs` scrive **perché** sono due, invece di lasciarlo dedurre |
| **D16** | ⛔ **`Registry::invoke` prende l'EFFETTO COME CHIUSURA**, e `kernel::registry` non nomina `Arbiter` né `VramPolicy`: la funzione vera — il cambio di policy — la registra il **dispaccio**, compito 7 | la regola **1** di ADR-0038 dice che il kernel dà il meccanismo e che *«il contenuto — quali funzioni esistano — lo portano le capacità e la GUI»* (**P-31**). Un registro che chiamasse `set_policy` dovrebbe **importare** l'arbitro, e il secondo invocatore — il gesto, col 12 — dovrebbe aggiungere il proprio effetto lì dentro: è la *«logica solo per gesti»* che l'ADR rifiuta, al rovescio. ⚠️ **Così la regola 1 sale al LIVELLO 1:** l'assenza di quell'`use` è la prova, e il criterio di chiusura la misura col `grep`. ⚠️ **Costo dichiarato:** la chiusura riceve il **giornale** — `set_policy` lo vuole, e prestarlo due volte non è esprimibile — quindi l'effetto potrebbe scrivere qualunque cosa; `invoke` non può sorvegliarlo e lo **dice** invece di fingere |
| **D17** | ⛔ **l'invocatore raggiunge il record come `u8`, non come enum**, e il codice lo assegna un `match` esaustivo in `Invoker::code` | un enum lì sarebbe la **quarta** enum `index_only` **sul filo**, ed è esattamente l'argomento che `PermissionDetail` scrive per esteso per rifiutarla: *«whose variant indices `tests/frozen_bytes.rs` would then have to pin ONE PER FROZEN RECORD, and an index on the wire never retires»*. Quel tipo se la cavò con un `bool` perché aveva **due** valori; qui ne arrivano **quattro** — click, gesto, voce, agente — e un `bool` non serve. ⛔ **E il `match` è `Operation::is_write` alla lettera:** `as u8` numererebbe una variante **per posizione**, e riordinare l'enum ripunterebbe in silenzio ogni record già scritto. ⚠️ **Costo dichiarato:** un `u8` torna indietro senza `from_code`, perché nessuno **decide** sull'invocatore oggi; il primo consumatore che vi si dirama lo scrive con la sua sonda (**P-32**) |
| **D18** | ⛔ **l'argomento dell'invocazione viaggia nel `payload` del record, non nel dettaglio strutturato**, sotto `Trust::Untrusted` | è il precedente di `VerdictDetail` parola per parola — *«THE DETAIL TEXT IS NOT HERE: it is untrusted by inheritance (ADR-0014) and travels in the record's `payload`, under the `trust` label that exists to say so»*. E c'è anche la metà meccanica: **ogni** `*Detail::new` del giornale prende `&'static str` per chiudere la strada del testo calcolato a runtime, e l'argomento è precisamente quel testo (**P-30**). ⚠️ **Costo dichiarato:** la §5 del 2 dice *«funzione, invocatore, argomento»* e il terzo sta **accanto** al dettaglio invece che dentro; la cella riceve il proprio richiamo datato al compito 6 |
| **D19** | ⛔ **il modulo dell'attività si chiama `serving`, e il suo item `serve`** — non `dispatch`, non `session` | `kernel::gateway::dispatch` esiste già e significa un'altra cosa — dispacciare un token conforme al provider — e un secondo `dispatch` nel kernel è l'ambiguità che **D8** rifiutò per `counter`; `session` è occupato al contrario, perché `crates/kernel/src/permission.rs` dichiara *«there is no session in the kernel»* e la voce aperta 7 della §9 del 2 assegna **al 3** il confine di sessione. `serving` è misurato libero (**P-36**). ⚠️ **Costo dichiarato:** i disegni dicono *«il dispaccio»* in italiano e il codice dice `serving`, quindi ogni richiamo datato nomina il sorgente **per esteso** — vincolo 2 |
| **D20** | ⛔ **`PolicyReport` porta `allocated` e `total`, e NON sottrae le due quote**: `Arbiter::allocated()` e `Parameters::total_vram()`, i due numeri che il kernel ha davvero | il campo si chiamava `allocatable` e il disegno gli metteva dentro il budget **allocato** (**P-38**): il nome va corretto comunque. Sottrarre le due quote chiede due addendi che il kernel **non ha** e che il doc di `Parameters::total_vram` argomenta di non consegnare; e ⛔ **sottrarle sarebbe anche sbagliato nel merito**, perché le due quote sono **concessioni con un titolare** (ADR-0033) e non sottrazioni — nasconderle dal numeratore è *«la sottrazione non è un'esenzione»*, gotcha **#4**, commesso allo strato che l'utente guarda. ⚠️ **Costo dichiarato:** la riga 2 della tabella *Stato* della stella polare dice *«tutto meno la quota audio e la quota di presentazione»*, e la sua stessa ultima colonna la marca **dedotto**; riceve un richiamo datato al passo 7. Se il proprietario vorrà il denominatore netto, i due addendi diventano parametri consegnati e questa riga si riapre con quella misura |
| **D21** | ⛔ **il limite di giri passa al compito 8**, e la riga 7 della tabella della posizione perde quelle parole | `EXECUTOR_TURN_LIMIT` vive nel file del compito 8, e la sonda che la §5 del 2 detta per esso — *«il grafo con la GUI resta vivo oltre centomila giri»* — **richiede il grafo con la GUI**, che nasce al cablaggio (**P-40**). Al 7 il cambio sarebbe inerte e la sua sonda non scrivibile: è l'argomento di **D13** alla lettera. ⚠️ **Costo:** due righe della tabella della posizione si riscrivono col richiamo datato, come fece D13 |
| **D22** | ⛔ **`Protection::AsSystemAccount` è mandato dall'attività e NON consegnato**, col proprio innesco scritto accanto | un parametro che può assumere **un** valore solo non è un parametro: è la superficie morta che il doc di `Parameters::total_vram` rifiuta con quelle parole, e la friction di §2.8.5 si pagherebbe su ogni chiamante per un valore che non può variare (**P-41**). È la forma *«dichiarato, non pinzato»* del gotcha #73. ⚠️ **Costo dichiarato:** è una divergenza dalla §5 del 2, che dice *«consegnato»*; riceve il richiamo datato al passo 7, e l'innesco è la **seconda variante** di `Protection` |
| **D23** | ⛔ **il degrado si rilegge a OGNI GIRO**, e solo con almeno un client accolto, e si manda solo quando **differisce** da quello già ricevuto | la §7 del 2 lascia la scelta al piano fra *«a ogni giro»* e *«dopo ogni scrittura»*, e la seconda è **falsificata dalla sonda che la stessa sezione pretende**: il `degrade` del rubinetto scrive dalla **seconda attività**, quindi una rilevazione legata alle scritture del dispaccio non la vedrebbe mai (**P-42**). ⚠️ **Costo dichiarato, e il chiusore esiste già:** `degradation_now` rilegge tutto il giornale — il costo che quella funzione dichiara di sé — e il rimedio è il **checkpoint** che `Journal::replay` nomina, nella voce aperta omonima della §6 del compendio. Non si inventa qui |
| **D24** | ⛔ **`Registry::invoke` prende un argomento `approval`, e con `Approval::JustGiven` scrive lui il `grant` su A** — correzione **nel compito 6**, non nell'errata | senza di esso la via dell'`Approve` **non esiste**: il controllo `is_granted` risponde no una seconda volta e il giro non si chiude mai (**P-43**). La decisione 21 della stella polare vuole il permesso **dentro** il passo A, fra la nota `Invocation` e l'effetto, perché `permission::grant` scrive una **nota** e vuole un passo già aperto. ⛔ **L'ordine sta nel meccanismo che lo possiede**, non nel chiamante: un dispaccio che scrivesse la fila da sé sarebbe la seconda copia che il §7 del 2 rifiuta con le stesse parole. ⚠️ **Un enum e non un `bool`**, che è la lezione di `Operation::is_write` e di `Access`. ⚠️ **Costo dichiarato:** la firma di `invoke` cresce di un argomento, il blocco *Interfaces* del compito 6 lo dice, e il registro guadagna `held` — che ha un chiamante dal giorno in cui è scritto |

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
- Modify: `crates/platform/src/lib.rs` (**CRLF**) — una riga di modulo, ⛔ **più il richiamo datato che P-25 impone**: il doc di modulo dice *«Today they are `Journal`, `Reactor` and `Rng`»* e da questo compito sono quattro
- Modify: `crates/platform/Cargo.toml` (**CRLF**) — `interprocess`, con la giustificazione accanto
- Modify: `Cargo.lock` (**CRLF**) — **nello stesso commit** del manifesto (vincolo 6)
- Modify: `crates/kernel/src/ports/mod.rs` (**`i/lf w/crlf`**) — ⛔ **due richiami datati, P-23**: la riga della tabella `ipc` (*«Real implementation arrives in»* → non più il Traguardo 6, ma questo compito) e la frase *«The other FOUR … have NO CALLER AT ALL»*, che scende a **tre**
- Modify: `crates/kernel/tests/ports_are_implementable.rs` (⛔ **`i/crlf w/crlf`**, P-3) — il richiamo sulla riga 1, *«One fake per port declared WITHOUT an implementation»*, falsa per `ipc` da questo compito
- Read: la §3 del disegno del 2 per intero; `crates/kernel/src/ports/ipc.rs`; `crates/kernel/src/framing.rs`; `crates/kernel/tests/journal_contract.rs` e `crates/platform/tests/journal_contract_real.rs` **per la forma**

⛔ **I TRE RICHIAMI SONO ARRIVATI QUI DAL PRE-CONTROLLO DEL COMPITO 4, il 2026-09-11 — P-23.** Questo compito
dà a `ipc` la sua prima implementazione vera, e da quel momento tre frasi in due file diventano false: la
tabella che `ports/mod.rs` dichiara essere **il disegno**, la frase con cui il banco delle finte giustifica la
propria esistenza, e la riga che manda il lettore al Traguardo 6. ⚠️ **Si correggono QUI e non al compito 4**,
che è dove sono state trovate: il rimedio sta dove nasce la causa, non dove si è visto il sintomo.

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

⛔ **E il doc di modulo sopra, che questo compito rende falso — P-25.** Il richiamo si aggiunge in coda al doc, **senza riscrivere la frase**:

```rust
//! ⚠️ DATED RECALL, 2026-09-11 -- "Today they are `Journal`, `Reactor` and `Rng`" IS FALSE FROM THIS TASK:
//! `ipc::LocalSocketIpc` is the fourth. The `grep` above DOES find it -- `Ipc` is one of the names it
//! enumerates -- so the two halves of that sentence now contradict each other, which is why the prose is
//! dated here rather than left to be believed. The count is deliberately NOT rewritten into the sentence:
//! the command answers it, and a figure inside prose is gotcha #31.
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

## Compito 3: lo schema che cresce — le varianti nuove, i gemelli del filo, le fixture e il timbro di build

**Files:**
- Modify: `crates/kernel/src/wire/ipc.rs` (**`i/lf w/crlf`**) — i gemelli, le varianti nuove, l'insieme canonico, il timbro; i due richiami datati di **P-16** e **P-17**
- Modify: `crates/kernel/tests/ipc_wire.rs` (**`i/lf w/crlf`**) — il controllo delle fixture, il generatore dichiarato, le sonde del timbro e del grafo
- Create: `gui/schema/fixtures/*.bin` — un file per variante, **rigenerabili**
- Create: `gui/schema/fixtures/ipc_v1.map` (**LF**) — la mappa `indice → nome → valore`, e il timbro
- Read: la §4 del [disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md), le due tabelle; la riga «lo schema» della tabella degli artefatti della §8; la **sequenza 1** di «La GUI dentro» nella [stella polare](../specs/2026-09-07-direzione-gui-design.md); `crates/kernel/src/wire/ipc.rs` e `crates/kernel/tests/ipc_wire.rs` **per intero**

**Interfaces:**
- Consumes: `kernel::arbiter::{ComputeClass, Mib, Preemption}`; `kernel::framing::{self, LENGTH_WIDTH, WireError}`; `kernel::time::Millis`
- Produces, e i compiti 7, 8, 10, 11, 12 e 13 li usano con questi nomi esatti:
  - `kernel::wire::ipc::BuildStamp` — `BuildStamp(u64)`, con `BuildStamp::get(&self) -> u64`
  - `kernel::wire::ipc::{Protection, DegradationReport, PolicyReport, PolicyName, Triple, Access, Call, Provenance, LayoutState, StepSummary}`
  - le **varianti nuove** di `kernel::wire::ipc::IpcMessage` — ⛔ **quante siano lo dice il comando del criterio di chiusura, non questa riga: un numerale in prosa qui è già stato falso una volta (P-35)**
  - `kernel::wire::ipc::stamp_set() -> alloc::vec::Vec<IpcMessage>` — l'insieme canonico
  - `kernel::wire::ipc::build_stamp() -> BuildStamp`
  - le fixture in `gui/schema/fixtures/`, che il compito 10 legge da `gui/src/schema/`

⛔ **I nomi delle VARIANTI sono fissati dal disegno** (§4, decisione 9) e non si ritoccano. I nomi dei **tipi
trasportati** li decide questo compito, e la convenzione è quella di `GrantRequest`: il tipo dice **cosa porta**,
non di chi è gemello — nessun suffisso meccanico, perché il modulo si chiama già `wire::ipc`.

### La regola dei gemelli — D11, e non è una preferenza

⛔ **Un tipo del kernel raggiunge il filo TALE E QUALE solo se è CHIUSO** — un newtype su un numero o un enum
senza dati, che non può guadagnare campi. `Mib`, `ComputeClass` e `Preemption` sono già così e **non si toccano**:
portano entrambe le lingue (`crates/kernel/src/arbiter/resource.rs:50-61`, minicbor e `bincode::Encode` sullo
stesso derive). **Ogni tipo con CAMPI porta un gemello** in questo modulo, e la conversione è il posto dove un
campo nuovo del giornale si ferma e va tradotto **a mano**, col compilatore che lo dice.

⛔ **E per due dei quattro non è nemmeno una scelta, il che è la prova che la regola non è arbitraria:**

| Tipo del kernel | Perché il gemello | Misurato |
|---|---|---|
| `permission::Permission` | porta `tool: &'static str` e `resource: &'static str`, che **non nascono da byte in arrivo** — è parola per parola l'argomento che il doc di `GrantRequest` scrive contro `ResourceProfile`: testo **scelto dalla GUI**, cioè contenuto non fidato (ADR-0014), dentro un tipo con cui il core **decide** | `crates/kernel/src/permission.rs:113-117`, il 2026-09-11 |
| `arbiter::policy::VramPolicy` | le sue due varianti **contengono le policy stesse** (`Remote(RemotePolicy)`, `Local(LocalPolicy)`): non è serializzabile per costruzione, e nessun derive la renderebbe tale | `crates/kernel/src/arbiter/policy.rs:63-66` |
| `degradation::Degradation` | due `bool` oggi, ma ADR-0019 dichiara la **lista degli eventi aperta** col rimando del 2026-09-08 — la telecamera arriva con ADR-0039 — e la §4 del 2 dice *«i due campi **di oggi**»*: cresce per costruzione | `crates/kernel/src/degradation.rs:23-27` |
| `record::Trust` | già deciso dal disegno: *«Sul filo un enum a due valori gemello di `Trust`, per non appendere derive `bincode` a un tipo del giornale»* | §4 del 2 |

⚠️ **Costo dichiarato:** le varianti nuove portano dieci tipi nuovi in `wire::ipc`, e ogni compito che le riempie
scrive una conversione. **Il beneficio è il compilatore:** ADR-0036 vuole che il giornale **evolva**, I4 rinuncia
al versionamento sul filo, e senza i gemelli un campo aggiunto al giornale cambierebbe i byte del filo **in
silenzio** — con una GUI vecchia che legge byte diversi e nessun rosso da nessuna parte.

- [ ] **Passo 1: le misure prima**

```bash
grep -c 'Request(GrantRequest)\|Verdict(Verdict)' crates/kernel/src/wire/ipc.rs
grep -n 'pub enum IpcMessage' -A 8 crates/kernel/src/wire/ipc.rs
ls gui 2>&1
ls crates/kernel/tests/frozen/*.cbor | wc -l
git ls-files --eol crates/kernel/src/wire/ipc.rs crates/kernel/tests/ipc_wire.rs
grep -c '#\[test\]' crates/kernel/tests/ipc_wire.rs
```

Atteso: `IpcMessage` ha **due** varianti, `Request` e `Verdict`; `gui` **non esiste**; **sei** `.cbor` congelati
(P-4, e questo compito non li tocca); i due file `i/lf w/crlf`; **sette** `#[test]` in `ipc_wire.rs`.

- [ ] **Passo 2: i gemelli e le varianti nuove**

In `crates/kernel/src/wire/ipc.rs`, **sopra** `impl IpcMessage`, con Python `newline=""` (il file è `w/crlf`).
Ogni tipo porta gli stessi derive dei due che ci sono già — `Debug, Clone, PartialEq, Eq, Encode, Decode` — e
`Copy` **solo** dove tutti i campi sono `Copy`, che è la differenza fra quelli che portano `String` e gli altri:

```rust
/// The identity of a build, §6.1.2. ⛔ IT IS NOT A CONTRACT, IT IS AN IDENTITY: one accepted
/// value, and a gui carrying a different one does not start and says so. I4 renounces
/// versioning, and this is the mechanism that stands in its place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct BuildStamp(u64);

impl BuildStamp {
    /// ⛔ THE CONSTRUCTOR IS `crate::wire::ipc::build_stamp` AND NOT A `new` HERE, and that is
    /// the point: a stamp anyone can mint from any number is a stamp that proves nothing. The
    /// only value that exists is the one the canonical set produces.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// What the core knows about the journal's protection, as a VALUE and not as fixed text in
/// the gui (G16, ADR-0023).
///
/// ⛔ ONE VARIANT AND NOT A `bool`, AND THE REASON IS ADR-0023 ITSELF: "encrypted at rest"
/// here means PROTECTED AS MUCH AS YOUR SYSTEM ACCOUNT, and that sentence has to reach the
/// interface -- a false sense of security is worse than none. A `bool` would let the gui
/// write its own sentence beside it; an enum makes the sentence the core's, and a second
/// level of protection a VARIANT rather than a silent change of meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum Protection {
    /// The keys are the OS's, reached through the platform module. ADR-0023.
    AsSystemAccount,
}

/// The degradation, on the wire. Twin of `crate::degradation::Degradation` -- D11.
///
/// ⚠️ THE TWO FIELDS ARE TODAY'S. ADR-0019 declares the event list OPEN (dated recall of
/// 2026-09-08), so a field added there must STOP HERE and be translated by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct DegradationReport {
    pub vram_exhausted: bool,
    pub routing_degraded: bool,
}

/// Which VRAM policy is active, with the budget. Twin of `crate::arbiter::policy::VramPolicy`
/// -- and here the twin is FORCED: that enum carries the policies themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum PolicyName {
    /// ADR-0006's default: OpenRouter, VRAM free.
    Remote,
    Local,
}

/// The policy with what the gui shows beside it (G15/G16).
///
/// ⛔ `allocated` IS WHAT THE BOOKS SPEAK FOR, THE TWO PERMANENT QUOTAS INCLUDED, and that
/// is a decision rather than an oversight (D20). ADR-0033 holds those two as GRANTS WITH A
/// HOLDER and not as subtractions -- "the subtraction is not an exemption", gotcha #4 -- so
/// hiding them from this number would commit at the layer the user looks at the very mistake
/// the arbiter was built to avoid. `total` is the machine, delivered through
/// `Parameters::total_vram`; the kernel does not hold the audio and presentation quotas, and
/// deliberately does not -- the doc of that accessor argues it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct PolicyReport {
    pub policy: PolicyName,
    pub allocated: Mib,
    pub total: Mib,
}

/// The permission triple of ADR-0016, on the wire. Twin of `crate::permission::Permission`
/// -- and FORCED, for `GrantRequest`'s own reason: that type carries `&'static str`.
///
/// ⛔ `String` AND NOT `&'static str`, WHICH IS THE WHOLE OF THIS TYPE. A `&'static str`
/// cannot be produced from arriving bytes without leaking, and what would leak is text CHOSEN
/// BY THE PEER -- untrusted content (ADR-0014) inside a type a permission decision reads.
/// ⚠️ AND THE DIRECTION MATTERS: the core sends this DOWN (`PermissionRequired`) and receives
/// it back UP (`Approve`). The returning one is untrusted, so whoever consumes it matches it
/// against the triple IT asked for rather than trusting the strings -- written here so the
/// consumer does not rediscover it. The probe is born with that consumer, task 7.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Triple {
    pub tool: String,
    pub resource: String,
    pub operation: Access,
}

/// The operation of the triple. Twin of `crate::permission::Operation` -- D11.
///
/// ⛔ AN ENUM AND NOT A `bool`, AND THE KERNEL ALREADY PAID FOR THIS LESSON: the doc of
/// `Operation::is_write` records that `matches!` folds EVERY other variant into `false`, so a
/// third operation would reach the DURABLE record as a read -- measured on 2026-09-01 with
/// `Execute` added. On the wire the same fold would make a third operation arrive as a read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum Access {
    Read,
    Write,
}

/// Which registry function, and with what argument. ADR-0038.
///
/// ⛔ NAMED `Call` AND NOT `Invocation`, and that is deliberate: task 6 brings
/// `crate::record::Invocation`, the DURABLE detail, and two types one letter apart in the two
/// worlds this plan keeps separate is the ambiguity D8 refused for `counter`.
///
/// ⚠️ THE ARGUMENT IS A `String` AND IT IS UNTRUSTED. Nothing here validates that `function`
/// names a registered function: the registry does, by REFUSING (task 6). A schema that could
/// only express registered names would be a second registry, kept aligned by hand.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Call {
    pub function: String,
    pub argument: String,
}

/// Where a piece of text came from. Twin of `crate::record::Trust` -- and the twin the design
/// asked for by name, "so as not to hang `bincode` derives on a journal type".
///
/// ⛔ A MODEL'S TEXT IS UNTRUSTED (ADR-0014) and the gui marks it (G13). The label is
/// HEREDITARY: summarising or concatenating untrusted text leaves it untrusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum Provenance {
    Trusted,
    Untrusted,
}

/// What the seventh port holds under the layout key -- and its three states, decision 35.
///
/// ⛔ `Unavailable` IS NOT `Nothing`, AND CONFLATING THEM WOULD BE THE SILENT DEGRADATION
/// ADR-0019 FORBIDS. "Nothing" is an archive that opened and is empty -- a first run.
/// "Unavailable" is an archive that would not open at all, and the core starts anyway and
/// SAYS SO. A gui told "nothing" would offer to save; one told "unavailable" knows the save
/// will not stick.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum LayoutState {
    /// The opaque package: `toJSON()` of `dockview` plus the active view.
    Package(Vec<u8>),
    /// The archive opened and holds nothing under the key.
    Nothing,
    /// The archive would not open. The core started anyway (decision 35).
    Unavailable,
}

/// One line of the step list: in milestone 2 these are registry invocations.
///
/// ⛔ A SUMMARY AND NOT THE RECORD. The journal's records must EVOLVE (ADR-0036) and this wire
/// renounces versioning (I4): sending the record itself would tie the two, and a field added
/// to a durable record would change these bytes with nothing going red.
///
/// ⛔ `done` IS A `bool` AND NOT AN `Option<bool>`, AND IT IS THE JOURNAL'S OWN VOCABULARY
/// (P-39). `RecordV1::outcome` carries no success flag: an outcome written says the step
/// CLOSED, and there is no "closed badly". A step whose effect failed returns before the
/// outcome is written and stays IN DOUBT (ADR-0007), which is `false` here. A third state
/// would be a variant on the wire with no producer, and an index on the wire never retires.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct StepSummary {
    pub step: u64,
    pub function: String,
    /// `false` while the step is still in doubt -- ADR-0007's intent written and no outcome yet.
    pub done: bool,
}
```

E l'enum, che **sostituisce** quello di oggi conservando le due varianti in coda, con la direzione nel doc di
ciascuna come il modulo già prescrive:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum IpcMessage {
    /// gui -> core. The handshake, §6.1.2.
    Hello(BuildStamp),
    /// core -> gui. The stamp matched.
    Accepted(Protection),
    /// core -> gui. The stamp did not match; this carries the EXPECTED one, and the core
    /// stops listening to that client -- the port has no close (decision 22).
    StaleBuild(BuildStamp),
    /// core -> gui.
    Degradation(DegradationReport),
    /// core -> gui.
    Policy(PolicyReport),
    /// gui -> core.
    Invoke(Call),
    /// core -> gui. The triple the user must grant, before the invocation can run.
    PermissionRequired(Triple),
    /// gui -> core. ⛔ IT CARRIES THE CALL TOO, decision 21: `permission::grant` writes a NOTE
    /// and wants a step already open, and the core opens that step only when the invocation
    /// arrives. So `Approve` is enough, and the gui does not resend `Invoke`.
    Approve { triple: Triple, call: Call },
    /// core -> gui. In milestone 2 only the fake core produces these.
    Token { text: String, provenance: Provenance },
    /// core -> gui. At the welcome after `Policy`, and again after every `SaveLayout`.
    Layout(LayoutState),
    /// gui -> core. The opaque package, when the layout settles and when the window closes.
    SaveLayout(Vec<u8>),
    /// core -> gui. At the welcome, and after every invocation.
    Steps(Vec<StepSummary>),
    /// gui -> core. ⚠️ NO GUI OF MILESTONE 2 SENDS THIS -- D5, and the 3D is not in the 2.
    Request(GrantRequest),
    /// core -> gui.
    Verdict(Verdict),
}
```

⚠️ **`String` e `Vec` vogliono l'import:** in testa al file `use alloc::string::String;` accanto a
`use alloc::vec::Vec;`, che c'è già. La crate è `no_std` + `alloc`, quindi `String` **non** è nel preludio.

- [ ] **Passo 3: l'insieme canonico e il timbro, nello stesso posto**

⛔ **Una funzione sola produce l'insieme, e da lì lo usano ENTRAMBI** — il generatore delle fixture e il timbro.
Due elenchi sarebbero due posti da tenere allineati per una proprietà sola, e il primo che smette mente in
silenzio: è l'argomento che `record_v1.map` scrive di sé. In coda a `crates/kernel/src/wire/ipc.rs`:

```rust
/// The canonical set: ONE message per variant, in a fixed order.
///
/// ⛔ ONE FUNCTION AND NOT TWO LISTS. The fixtures are generated from this and the stamp is
/// computed from this, so THEY CANNOT DRIFT APART: a variant added here changes both, and a
/// variant added to `IpcMessage` and forgotten here is caught by
/// `every_variant_is_in_the_canonical_set` below. Two lists would be two places to keep
/// aligned for one property, and the first one to stop being updated lies in silence -- the
/// argument `crates/kernel/tests/frozen/record_v1.map` makes about itself.
///
/// ⚠️ THE VALUES ARE ARBITRARY BUT NOT RANDOM: each one is chosen so that no two encodings
/// are equal and no field is left at its type's default, because a fixture full of zeroes
/// cannot tell a field that is written from one that is skipped.
pub fn stamp_set() -> Vec<IpcMessage> {
    alloc::vec![
        IpcMessage::Hello(BuildStamp(0x0123_4567_89AB_CDEF)),
        IpcMessage::Accepted(Protection::AsSystemAccount),
        IpcMessage::StaleBuild(BuildStamp(0xFEDC_BA98_7654_3210)),
        IpcMessage::Degradation(DegradationReport {
            vram_exhausted: true,
            routing_degraded: false,
        }),
        IpcMessage::Policy(PolicyReport {
            policy: PolicyName::Remote,
            allocated: Mib::new(12288),
            total: Mib::new(16384),
        }),
        IpcMessage::Invoke(Call {
            function: String::from("arbiter.set_policy"),
            argument: String::from("local"),
        }),
        IpcMessage::PermissionRequired(Triple {
            tool: String::from("arbiter"),
            resource: String::from("policy"),
            operation: Access::Write,
        }),
        IpcMessage::Approve {
            triple: Triple {
                tool: String::from("arbiter"),
                resource: String::from("policy"),
                operation: Access::Write,
            },
            call: Call {
                function: String::from("arbiter.set_policy"),
                argument: String::from("local"),
            },
        },
        IpcMessage::Token {
            text: String::from("ciao"),
            provenance: Provenance::Untrusted,
        },
        IpcMessage::Layout(LayoutState::Package(alloc::vec![0x7B, 0x7D])),
        IpcMessage::SaveLayout(alloc::vec![0x5B, 0x5D]),
        IpcMessage::Steps(alloc::vec![StepSummary {
            step: 42,
            function: String::from("arbiter.set_policy"),
            done: true,
        }]),
        IpcMessage::Request(GrantRequest {
            reserved_vram: Mib::new(2048),
            compute_class: ComputeClass::Interactive,
            preemption: Preemption::After(Millis::new(500)),
        }),
        IpcMessage::Verdict(Verdict::Refused {
            asked: Mib::new(4096),
            ceiling: Mib::new(1024),
        }),
    ]
}

/// The build stamp: FNV-1a over the encoding of the canonical set.
///
/// ⛔ WRITTEN BY HAND AND NOT A DEPENDENCY, and the two reasons are different. ADR-0031 makes
/// adding a crate to the kernel's list a deliberate act, and this is SIX LINES. And it is an
/// IDENTITY, not a defence: nothing here resists a peer that wants to forge a stamp, because a
/// peer that can forge one is already inside the process boundary. ⚠️ THE DAY THIS IS ASKED TO
/// BE A DEFENCE IT IS THE WRONG FUNCTION, and the note is here rather than in the design
/// because this is where someone would reach for it. ⛔ AND IT IS NOT ADR-0018's FINGERPRINT
/// for pruned payloads, which remains a registered decision of the owner
/// (`crate::ports::journal`, the doc of `prune`).
///
/// ⚠️ THE LENGTH GOES IN TOO, not just the bytes: without it two adjacent messages could be
/// re-split differently and hash the same.
pub fn build_stamp() -> BuildStamp {
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for message in stamp_set() {
        let bytes = message.encode().unwrap_or_default();
        for byte in (bytes.len() as u64).to_be_bytes().iter().chain(bytes.iter()) {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    BuildStamp(hash)
}
```

⚠️ **`Mib::new` e `Millis::new` sono `const fn` pubbliche** — `crates/kernel/tests/ipc_wire.rs` le usa già così,
quindi non serve nient'altro. E `alloc::vec!` si scrive per esteso perché la crate è `no_std`.

- [ ] **Passo 4: il banco che fallisce — la compilazione prima**

```bash
cargo build --locked -p kernel 2>&1 | tail -20
```

Atteso al primo colpo: **rosso**, con `cannot find type` sui gemelli finché il Passo 2 non è scritto per intero;
poi **verde** a zero avvisi. ⛔ **Se è verde PRIMA del Passo 2, il compito è già eseguito** — quarta domanda del
pre-controllo: ci si ferma e si riporta.

- [ ] **Passo 5: le sonde nuove in `ipc_wire.rs`**

In coda a `crates/kernel/tests/ipc_wire.rs`, con Python `newline=""` (il file è `w/crlf`). ⛔ **Le sette sonde
che ci sono già NON si toccano**: reggono, e la prima direzione del compito è che restino verdi.

```rust
#[test]
fn every_variant_is_in_the_canonical_set() {
    // ⛔ THE GUARD THAT MAKES THE OTHER PROBES WORTH SOMETHING. A variant added to
    // `IpcMessage` and forgotten in `stamp_set` would leave the fixtures short, the stamp
    // unchanged, and every probe in this file GREEN -- the schema would have grown and
    // nothing would say so. The `match` is exhaustive on purpose: adding a variant makes THIS
    // a compile error, which is level 1 rather than a test at level 2.
    let mut seen = [false; 14];
    for message in stamp_set() {
        let slot = match message {
            IpcMessage::Hello(_) => 0,
            IpcMessage::Accepted(_) => 1,
            IpcMessage::StaleBuild(_) => 2,
            IpcMessage::Degradation(_) => 3,
            IpcMessage::Policy(_) => 4,
            IpcMessage::Invoke(_) => 5,
            IpcMessage::PermissionRequired(_) => 6,
            IpcMessage::Approve { .. } => 7,
            IpcMessage::Token { .. } => 8,
            IpcMessage::Layout(_) => 9,
            IpcMessage::SaveLayout(_) => 10,
            IpcMessage::Steps(_) => 11,
            IpcMessage::Request(_) => 12,
            IpcMessage::Verdict(_) => 13,
        };
        assert!(!seen[slot], "slot {slot} appears twice in the canonical set");
        seen[slot] = true;
    }
    let missing: Vec<usize> = (0..14).filter(|i| !seen[*i]).collect();
    assert!(missing.is_empty(), "variants missing from stamp_set: {missing:?}");
}

#[test]
fn every_message_of_the_canonical_set_survives_the_round_trip() {
    // ⚠️ ONE LOOP THAT COLLECTS RATHER THAN FOURTEEN ASSERTS IN A ROW -- gotcha #14 from the
    // other side. A row of asserts stops at the first red and hides the other thirteen; a
    // loop that stops does the same. This one records every failure and reports them together,
    // so one red tells the whole story.
    let mut broken = Vec::new();
    for message in stamp_set() {
        let bytes = match message.encode() {
            Ok(bytes) => bytes,
            Err(_) => {
                broken.push(alloc_fmt(&message, "encode failed"));
                continue;
            }
        };
        match IpcMessage::decode(&bytes) {
            Ok(back) if back == message => {}
            Ok(_) => broken.push(alloc_fmt(&message, "decoded to a different value")),
            Err(error) => broken.push(alloc_fmt(&message, &format!("{error:?}"))),
        }
    }
    assert!(broken.is_empty(), "round trip failed for:\n{}", broken.join("\n"));
}

fn alloc_fmt(message: &IpcMessage, why: &str) -> String {
    format!("  {message:?} -- {why}")
}

#[test]
fn a_string_in_the_schema_still_cannot_stop_the_encoder() {
    // ⛔ THIS IS P-17 MEASURED RATHER THAN DEDUCED. The doc of `IpcMessage::encode` argues that
    // a failed encoding cannot happen by reading THIS TYPE'S GRAPH, and until today that graph
    // was `Mib(u64)`, `ComputeClass`, `Preemption`, `Millis(u64)` and unit variants. The
    // variants added today put `String` and `Vec<u8>` in it, so the argument is RE-READ rather
    // than inherited: of the four `EncodeError` variants reachable without `std`
    // (`UnexpectedEnd`, `RefCellAlreadyBorrowed`, `Other`, `OtherString`) none is producible by
    // a derived `Encode` over `String` and `Vec<u8>` into a growing `Vec`. Read in
    // bincode 2.0.1's `src/error.rs` on 2026-09-11.
    //
    // ⚠️ AND THE INPUT IS THE AWKWARD ONE, not a convenient short string: multi-byte UTF-8, an
    // empty string, and an empty byte vector in the same message.
    let message = IpcMessage::Token {
        text: String::from("caffè ☕ \u{0}\u{7F}"),
        provenance: Provenance::Untrusted,
    };
    let bytes = message.encode().expect("a String does not stop the encoder");
    assert_ne!(bytes.len(), LENGTH_WIDTH, "an empty body would mean the encoder stopped");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));

    let empty = IpcMessage::Layout(LayoutState::Package(Vec::new()));
    let bytes = empty.encode().expect("an empty Vec does not stop the encoder");
    assert_eq!(IpcMessage::decode(&bytes), Ok(empty));
}

#[test]
fn the_stamp_changes_when_the_schema_changes() {
    // ⛔ THE SECOND DIRECTION, AND WITHOUT IT THE STAMP PROVES NOTHING. That `build_stamp()`
    // returns the same value twice is what a constant would also do. What must hold is that a
    // DIFFERENT set gives a DIFFERENT stamp -- computed here over a set with one message
    // altered, using the same function the real one uses.
    assert_eq!(build_stamp(), build_stamp(), "the stamp is stable within a build");

    let mut altered = stamp_set();
    altered[0] = IpcMessage::Hello(BuildStamp(0));
    assert_ne!(
        fnv_over(&altered),
        build_stamp().get(),
        "a changed message must change the stamp"
    );

    let shortened: Vec<IpcMessage> = stamp_set().into_iter().skip(1).collect();
    assert_ne!(
        fnv_over(&shortened),
        build_stamp().get(),
        "a missing message must change the stamp"
    );
}

/// The same arithmetic as `build_stamp`, over a set the probe chooses. ⚠️ WRITTEN OUT RATHER
/// THAN CALLING `build_stamp`, which takes no argument: an oracle that calls the thing it
/// checks would be vacuous.
fn fnv_over(messages: &[IpcMessage]) -> u64 {
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for message in messages {
        let bytes = message.encode().expect("encode");
        for byte in (bytes.len() as u64).to_be_bytes().iter().chain(bytes.iter()) {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    hash
}
```

⚠️ **Gli import in testa al file crescono:** `use kernel::wire::ipc::{build_stamp, stamp_set, Access, BuildStamp,
Call, DegradationReport, GrantRequest, IpcMessage, LayoutState, PolicyName, PolicyReport, Protection, Provenance,
StepSummary, Triple, Verdict};`. ⛔ **Un test di integrazione compila con `std`**, quindi `String`, `Vec` e
`format!` sono nel preludio qui, al contrario di `src/`.

- [ ] **Passo 6: il generatore dichiarato, e le fixture**

⛔ **È un comando DICHIARATO e non un rimedio nascosto:** le fixture **si rigenerano** quando lo schema cambia, al
contrario dei byte congelati di ADR-0036, che non si rigenerano mai. Il generatore vive accanto al controllo,
ignorato dal cancello, in coda a `crates/kernel/tests/ipc_wire.rs`:

```rust
#[test]
#[ignore = "generator, not a check: run it on purpose when the schema changes -- see the map"]
fn regenerate_the_fixtures() {
    // ⛔ THE ONLY WRITER OF `gui/schema/fixtures/`, and it writes the map in the same pass, so
    // bytes and map cannot drift. `#[ignore]` with a reason, as `scripts/gate.sh` requires of
    // every ignored test: the gate must not rewrite artefacts it is checking.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../gui/schema/fixtures");
    std::fs::create_dir_all(&root).expect("create the fixtures directory");
    let mut map = String::from(
        "# The map of the ipc fixtures -- \u{a7}4 of the milestone-2 design.\n\
         # REGENERATED, not frozen: when the schema changes, run\n\
         #   cargo test --locked -p kernel --test ipc_wire -- --ignored regenerate_the_fixtures\n\
         # and commit what changes. The frozen bytes of ADR-0036 are the OPPOSITE artefact and\n\
         # live in crates/kernel/tests/frozen/.\n\
         # Produced by regenerate_the_fixtures in ../../../crates/kernel/tests/ipc_wire.rs.\n\n",
    );
    for (index, message) in stamp_set().into_iter().enumerate() {
        let name = variant_name(&message);
        let bytes = message.encode().expect("encode");
        std::fs::write(root.join(format!("{index:02}-{name}.bin")), &bytes).expect("write");
        map.push_str(&format!(
            "{index:02} {name} {len} bytes\n    {message:?}\n",
            len = bytes.len()
        ));
    }
    map.push_str(&format!("\nstamp {:#018x}\n", build_stamp().get()));
    std::fs::write(root.join("ipc_v1.map"), map).expect("write the map");
}

/// The name a fixture file carries. ⚠️ AN EXHAUSTIVE `match` AND NOT `{:?}` TRUNCATED: a
/// variant added must make this a compile error, the same reason `Operation::is_write` gives.
fn variant_name(message: &IpcMessage) -> &'static str {
    match message {
        IpcMessage::Hello(_) => "hello",
        IpcMessage::Accepted(_) => "accepted",
        IpcMessage::StaleBuild(_) => "stale-build",
        IpcMessage::Degradation(_) => "degradation",
        IpcMessage::Policy(_) => "policy",
        IpcMessage::Invoke(_) => "invoke",
        IpcMessage::PermissionRequired(_) => "permission-required",
        IpcMessage::Approve { .. } => "approve",
        IpcMessage::Token { .. } => "token",
        IpcMessage::Layout(_) => "layout",
        IpcMessage::SaveLayout(_) => "save-layout",
        IpcMessage::Steps(_) => "steps",
        IpcMessage::Request(_) => "request",
        IpcMessage::Verdict(_) => "verdict",
    }
}
```

Poi si lancia, una volta:

```bash
cargo test --locked -p kernel --test ipc_wire -- --ignored regenerate_the_fixtures
ls gui/schema/fixtures/
cat gui/schema/fixtures/ipc_v1.map | tail -3
tr -cd '\r' < gui/schema/fixtures/ipc_v1.map | wc -c
```

Atteso: **quattordici** `.bin` più `ipc_v1.map`; la mappa finisce con la riga `stamp 0x…`; **zero** CR nella mappa
(nasce LF, vincolo 4).

- [ ] **Passo 7: il controllo che dice «rigenera»**

⛔ **È il rovescio dei byte congelati, e il messaggio di rosso è la differenza:** lì il rosso dice *«hai cambiato
formato»*, qui dice *«rigenera»*. In coda a `ipc_wire.rs`:

```rust
#[test]
fn the_committed_fixtures_match_the_schema() {
    // ⛔ THE CHECK THE GATE RUNS. A schema changed without regenerating is RED here, and the
    // message says what to do rather than leaving the reader to work it out -- because the
    // right answer is to regenerate, which is exactly the wrong answer for `frozen_bytes.rs`.
    //
    // ⚠️ READ AT RUN TIME AND NOT `include_bytes!`, and the difference from `frozen_bytes.rs`
    // is the artefact, not an oversight: a frozen record is an ORACLE that must enter the
    // binary, while these are regenerable and a MISSING one has to be a red rather than a
    // compile error the generator itself could not fix.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../gui/schema/fixtures");
    let mut wrong = Vec::new();
    for (index, message) in stamp_set().into_iter().enumerate() {
        let name = variant_name(&message);
        let path = root.join(format!("{index:02}-{name}.bin"));
        let expected = message.encode().expect("encode");
        match std::fs::read(&path) {
            Ok(found) if found == expected => {}
            Ok(_) => wrong.push(format!("  {index:02}-{name}.bin: different bytes")),
            Err(error) => wrong.push(format!("  {index:02}-{name}.bin: {error}")),
        }
    }
    let extra: Vec<String> = std::fs::read_dir(&root)
        .expect("read the fixtures directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".bin"))
        .filter(|name| {
            !stamp_set()
                .iter()
                .enumerate()
                .any(|(i, m)| *name == format!("{i:02}-{}.bin", variant_name(m)))
        })
        .collect();
    assert!(
        wrong.is_empty() && extra.is_empty(),
        "the committed fixtures do not match the schema. REGENERATE them:\n  \
         cargo test --locked -p kernel --test ipc_wire -- --ignored regenerate_the_fixtures\n\
         mismatched:\n{}\nleft over:\n  {}",
        wrong.join("\n"),
        extra.join("\n  ")
    );
}
```

⚠️ **La seconda metà è il file di TROPPO**, e senza di essa una variante tolta lascerebbe la sua fixture in
`gui/` per sempre, letta da nessuno e committata da tutti.

- [ ] **Passo 8: le due direzioni, misurate**

⛔ **Un controllo si prova in due direzioni**, e la seconda è quella che si dimentica. Una per volta, compilata,
eseguita, e **revocata** con `git diff` a zero:

| | La mutazione | Atteso |
|---|---|---|
| **G6** | in `stamp_set`, `Access::Write` → `Access::Read` nella `PermissionRequired` | `the_committed_fixtures_match_the_schema` **rosso**, col nome `06-permission-required.bin` e la riga «REGENERATE them» |
| **G7** | togli la riga `IpcMessage::Steps(...)` da `stamp_set` | `every_variant_is_in_the_canonical_set` **rosso** con `variants missing from stamp_set: [11]`, e il controllo delle fixture rosso su «left over» |
| **G8** | in `build_stamp`, togli `(bytes.len() as u64).to_be_bytes().iter().chain(...)` e lascia `bytes.iter()` | `the_stamp_changes_when_the_schema_changes` resta **verde** — ⚠️ **e questo è il limite dichiarato della sonda**, non un difetto da correggere qui: la lunghezza difende contro due messaggi ri-tagliati, che l'insieme canonico non contiene. Si **registra** in coda alla voce, non si inventa un caso per farlo scattare |

```bash
cargo test --locked -p kernel --test ipc_wire 2>&1 | tail -5
git diff --stat
```

Atteso dopo ogni revoca: **dodici passati**, e `git diff --stat` **vuoto**.

- [ ] **Passo 9: i due richiami datati — P-16 e P-17**

(a) In `crates/kernel/src/wire/ipc.rs`, **sotto** il capoverso del modulo che comincia con
`//! ⛔ AND THE REVOCATION core -> gui IS A DECLARED NON-CONSTRUCTION` — il *Trova* si prende dal file, non da
qui:

```rust
//! ✅ DATED RECALL, 2026-09-11 -- THE TRIGGER WRITTEN ABOVE FELL DUE TODAY AND IS NOT HONOURED,
//! WHICH IS THE OPPOSITE OF WHAT "the same shell" READS LIKE. The shell arrived (ADR-0029,
//! Electron, 2026-09-10) and the revocation is STILL not built, because the sentence names the
//! shell while the mechanism needs an ADDRESSEE: milestone 2 does not serve `Request` at all
//! (decision D5 of its plan, argued against this file's own doc of `GrantRequest`), so NO
//! ORDINARY GRANT EXISTS to revoke and there is nobody to tell. ⛔ THE TRIGGER IS THEREFORE
//! THE 3D CONSUMER, subproject 7 -- the same closer row 27 of milestone 6 carries in
//! docs/porta-di-qualita.md. ⚠️ CORRECTED RATHER THAN DELETED: the paragraph above is the only
//! place that says what the revocation IS, and it is gotcha #77 again -- a deadline written in
//! prose, which nothing can go red for.
//!
//! ✅ AND THE OTHER HALF DID ARRIVE: the BUILD STAMP of §6.1.2 exists as of today,
//! `crate::wire::ipc::build_stamp` over `stamp_set`. "Until it exists, NOTHING REFUSES A STALE
//! GUI" above is now false, and the handshake that uses it is task 7 of the same plan.
```

(b) In `crates/kernel/src/wire/ipc.rs`, in coda al doc di `IpcMessage::encode`:

```rust
    /// ⚠️ DATED RECALL, 2026-09-11 -- THE GRAPH READ ABOVE IS NO LONGER THIS TYPE'S GRAPH, AND
    /// THE ARGUMENT IS RE-READ RATHER THAN INHERITED. Eleven variants arrived, and they put
    /// `String` and `Vec<u8>` into it. Re-read in bincode 2.0.1's `src/error.rs` the same day:
    /// the variants reachable without `std` are `UnexpectedEnd` (a writer out of room),
    /// `RefCellAlreadyBorrowed`, `Other(&'static str)` and `OtherString(String)` behind
    /// `alloc` -- and the last two are produced only by a HAND-WRITTEN `Encode`, which nothing
    /// here has. The relation holds; the enumeration did not, which is why it is dated instead
    /// of left standing. ⛔ AND IT IS NOW MEASURED RATHER THAN ARGUED:
    /// `a_string_in_the_schema_still_cannot_stop_the_encoder` in
    /// `crates/kernel/tests/ipc_wire.rs` is the probe, and it feeds the awkward string rather
    /// than a convenient one.
```

⛔ **Anche `crates/kernel/src/ports/ipc.rs` porta la frase *«milestone 6 brings … the BUILD STAMP»* col proprio
richiamo del 2026-08-31 che ne corregge il *quando*.** Quel file **non si tocca qui**: il suo richiamo dice
già che l'innesco *«vive accanto allo schema in `crate::wire::ipc` — UNA casa, così i due non divergono»*, e (a)
è quella casa. ⚠️ **Toccarlo sarebbe ricreare la seconda casa che quel richiamo ha tolto** — gotcha #68.

- [ ] **Passo 10: i fine-riga, il cancello, il commit**

```bash
for f in crates/kernel/src/wire/ipc.rs crates/kernel/tests/ipc_wire.rs; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol crates/kernel/src/wire/ipc.rs crates/kernel/tests/ipc_wire.rs
tr -cd '\r' < gui/schema/fixtures/ipc_v1.map | wc -c
bash scripts/gate.sh 2>&1 | tail -3
bash scripts/check-docs.sh 2>&1 | tail -2
git status --porcelain | head -20
```

Atteso: per i due file `CR` **uguale** alle righe e `i/lf w/crlf` **invariato**; **zero** CR nella mappa;
`GATE GREEN`; `OK`; in `git status` i due sorgenti e i quindici file di `gui/schema/fixtures/`, **niente altro**.

```bash
git add crates/kernel/src/wire/ipc.rs crates/kernel/tests/ipc_wire.rs gui/schema/fixtures docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md
git commit -m "gui(compito 3): lo schema che cresce -- le varianti nuove di IpcMessage coi gemelli del filo (D11), l'insieme canonico e il timbro di build, le quattordici fixture rigenerabili in gui/schema/fixtures e il controllo che dice rigenera; i richiami datati su P-16 (l'innesco della revoca e' il 7) e P-17 (il grafo di encode riletto)"
git push
```

#### Criterio di chiusura del compito 3

- [ ] `cargo test --locked -p kernel --test ipc_wire` → **dodici passati**, uno ignorato
- [ ] ⛔ **le varianti si CONTANO, non si rileggono — P-35:**
  `awk '/^pub enum IpcMessage/{s=1} s&&/^}/{exit} s&&/^    [A-Z]/{c++} END{print c}' crates/kernel/src/wire/ipc.rs`
  → **14**; lo stesso `awk` sull'uscita di `git show 42b50d8:crates/kernel/src/wire/ipc.rs` → **2**. Le nuove
  sono la **differenza fra i due**, e nessun numerale in prosa la ripete
- [ ] `ls gui/schema/fixtures/*.bin | wc -l` → **14**; `grep -c '^stamp 0x' gui/schema/fixtures/ipc_v1.map` → **1**
- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/kernel/src/wire/ipc.rs` → **2**
- [ ] `grep -c 'BUILD STAMP' crates/kernel/src/ports/ipc.rs` → **invariato rispetto al Passo 1**: quel file non si tocca
- [ ] le tre mutazioni G6, G7, G8 provate **una per volta** e revocate, con `git diff --stat` vuoto
- [ ] `bash scripts/gate.sh` → `GATE GREEN`; `bash scripts/gate-deps.sh` **verde**: la lista di ADR-0031 **non è cresciuta** (il timbro è scritto a mano)
- [ ] `crates/kernel/tests/frozen/` **invariato**: `ls crates/kernel/tests/frozen/*.cbor | wc -l` → **6**, e `git diff --stat crates/kernel/tests/frozen/` vuoto
- [ ] i fine-riga rimisurati, `git ls-files --eol` invariato sui due file toccati
- [ ] la riga **3** della tabella della posizione a ✅ con la data

---

## Compito 4: la settima porta — il tratto `Custody`, la finta che lo prova da fuori, e i richiami in quattro case

**Files:**
- Create: `crates/kernel/src/ports/custody.rs` (**LF**) — il tratto, `CustodyKey`, `CustodyError`
- Modify: `crates/kernel/src/ports/mod.rs` (**`i/lf w/crlf`**) — `pub mod custody;`, la riga nella tabella, le **cifre in prosa** e la guardia di **P-21**
- Modify: `crates/kernel/tests/ports_are_implementable.rs` (⛔ **`i/crlf w/crlf`** — vedi il vincolo 4 e **P-3**) — la finta e il suo banco
- Modify: `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` (**CRLF**) — **tre** richiami: la riga 209, la §2.3 e la §3.1 (**P-22**)
- Read: la **§2 della stella polare** per intero; la riga della settima porta della tabella degli artefatti della §8 del 2; `crates/kernel/src/ports/mod.rs` **per intero** (91 righe); `crates/kernel/src/ports/journal.rs` per la forma del tratto e di `JournalError`; le righe **209**, **575** e **906** della spec, **una per volta**, col `grep` sulla frase

**Interfaces:**
- Consumes: nulla di nuovo — il modulo non ha dipendenze
- Produces, e i compiti 5, 7, 8 e 11 li usano con questi nomi esatti:
  - `kernel::ports::custody::Custody` — il tratto
  - `Custody::keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError>`
  - `Custody::retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError>`
  - `kernel::ports::custody::CustodyKey` — `Debug + Clone + Copy + PartialEq + Eq`, oggi una variante: `CustodyKey::Layout`
  - `kernel::ports::custody::CustodyError` — `Debug + Clone + Copy + PartialEq + Eq`, oggi una variante: `CustodyError::Unavailable`

⛔ **LA SUITE DI CONFORMITÀ NON È DI QUESTO COMPITO — D13.** La riga 4 della tabella della posizione diceva *«la
suite di conformità»* e il **richiamo di D13 la sposta al compito 5**, che porta le due implementazioni: una suite
ne confronta **due**, e qui ce ne sono **zero** (**P-20**). Ciò che questo compito prova è un'altra cosa, ed è la
**terza domanda** del pre-controllo di `CLAUDE.md` in persona: che il tratto sia implementabile **da fuori dalla
crate**.

- [ ] **Passo 1: le misure prima**

```bash
ls crates/kernel/src/ports/custody.rs 2>&1
ls crates/kernel/src/ports/ | tr '\n' ' '
grep -niE "SIX|FIVE|SEVEN" crates/kernel/src/ports/mod.rs
grep -c '^struct \|^impl .* for ' crates/kernel/tests/ports_are_implementable.rs
grep -n 'Le famiglie di porte restano sei\|### 2.3 I/O\|### 3.1 Cosa sostituisce' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
git ls-files --eol crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

Atteso: `custody.rs` **non esiste**; **sei** file più `mod.rs`; le cifre in prosa alle righe **1, 7, 22, 29, 32,
37, 43, 77, 79** (P-5 e P-21 le hanno misurate il 2026-09-11: si **rimisurano**, e se una riga si è mossa vale il
*Dove*, mai il numero — gotcha #70); le tre righe della spec a **209**, **575**, **906**;
⛔ `ports_are_implementable.rs` **`i/crlf w/crlf`**, unico fra i sorgenti, e gli altri due `i/lf w/crlf`.

- [ ] **Passo 2: il tratto, nuovo**

`crates/kernel/src/ports/custody.rs`, **LF**:

```rust
//! The SEVENTH family of ports: keeping bytes the kernel never opens.
//!
//! ⛔ WHY A PORT AND NOT THE JOURNAL, IN ONE PARAGRAPH, because whoever reads this file is
//! exactly who would ask. The journal keeps "the small part" for ever (ADR-0018): permissions,
//! policy, approved guides -- SMALL DECISIONS. A panel layout is written dozens of times a day
//! and would sit there for ever, encrypted and in the backup, and the retention milestone would
//! owe it an exception written just for it. ADR-0022 had already decided that "configuration"
//! is an archive of its own; THIS PORT IS THAT ARCHIVE, in its smallest form. The full argument
//! is decision 15 of the GUI north star.
//!
//! ⛔ AND IT IS NOT CONFIGURATION OF THE KERNEL, WHICH IS THE OTHER THING IT LOOKS LIKE. A value
//! the kernel DECIDES with is handed to it (ADR-0034, and its negative perimeter says the kernel
//! is not a configuration system). Here the kernel CUSTODIES what the gui entrusts to it and
//! never reads it to decide anything -- it cannot, because the package is opaque. If a future
//! caller wants to READ what is kept here in order to decide, that is ADR-0034's question and
//! not this port's.
//!
//! ⛔ NO PATH AND NO FILE NAME IS NAMEABLE HERE (I3). The key is a closed enum; where the bytes
//! land is `platform`'s business, and the simulator substitutes the whole thing.
//!
//! ⚠️ ONE KEY TODAY, AND THAT IS NOT A SYSTEM WAITING TO HAPPEN. Checked before deciding
//! whether others would need this: approved guides, permissions and the VRAM policy are all
//! PROJECTIONS OF THE JOURNAL (ADR-0009), so the layout is the only PACKAGE in sight. Two
//! operations and one key are not a configuration system -- no format, no schema, no
//! validation, no hot reload.

use alloc::vec::Vec;

/// What is being kept. ⛔ A CLOSED ENUM AND NOT A STRING: a string key is a namespace, and a
/// namespace is the configuration system this port is deliberately not. A second thing to keep
/// is a VARIANT, added deliberately, which is the same shape ADR-0031 asks of a dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustodyKey {
    /// The gui's layout: `toJSON()` of `dockview` plus the active view, as ONE opaque package.
    Layout,
}

/// What can go wrong. ⛔ ONE VARIANT, AND THE REASON IS THE RULE THIS REPOSITORY USES
/// EVERYWHERE: no caller, no variant. Distinguishing "the archive would not open" from "the
/// write was refused" would be two variants with ONE producer between them today, and the
/// caller does not need the distinction to be in the type -- IT READS IT FROM WHICH OPERATION
/// FAILED. `keep` fails and `retrieve` answers: the write was refused, and the activity sends
/// back the old package. Both fail: the archive is unavailable, and the activity says so
/// (decision 35 of the milestone-2 design). Written here so the consumer does not rediscover it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustodyError {
    /// The archive could not be reached -- it would not open, or the write did not land.
    Unavailable,
}

/// Keeping bytes under a key, and handing them back.
///
/// ⛔ TWO OPERATIONS AND NO DELETE. Nothing in the design asks to forget a layout: replacing it
/// is `keep` with other bytes, and "restore the default view" is presentation -- the gui drops
/// the saved entry (§2 of the north star, "examined and without a source today"). A third
/// operation with no caller would be the speculative layer criterion 5 refuses.
pub trait Custody {
    /// Keeps these bytes under this key, replacing whatever was there.
    ///
    /// ⛔ THE BYTES ARE OPAQUE AND STAY THAT WAY. No implementation may parse, validate or
    /// canonicalise them: the day `dockview` changes format, the core does not change. The
    /// probe that holds it is "bytes that are not JSON come back identical", and it lives with
    /// the implementations (task 5).
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError>;

    /// Hands back what is kept under this key, or `None` if nothing is.
    ///
    /// ⛔ `Ok(None)` AND NOT AN ERROR, AND IT IS A DELIBERATE DIVERGENCE FROM `Journal`, WHOSE
    /// `read_back` ANSWERS `Err(JournalError::Missing)`. The two are not the same question. A
    /// step the journal has no record of is a FAULT -- someone asked about a step that should be
    /// there. A key with nothing under it is the FIRST RUN, the ordinary case, and folding it
    /// into an error would make the commonest path look like a failure and push every caller to
    /// match an error variant to find out that everything is fine. ⚠️ AND IT IS WHAT THE DESIGN
    /// ALREADY ASKS FOR: `Layout` carries "the package, NOTHING, or unavailable", three states,
    /// and `Ok(None)` is the middle one arriving as a value rather than as a failure.
    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError>;
}
```

- [ ] **Passo 3: la riga di modulo**

In `crates/kernel/src/ports/mod.rs`, la lista dei `pub mod` è in **ordine alfabetico** — misurato al Passo 1:
`filesystem`, `ipc`, `journal`, `network`, `process`, `reactor`. `custody` va **prima** di `filesystem`:

```rust
pub mod custody;

pub mod filesystem;
```

- [ ] **Passo 4: la finta che lo prova DA FUORI DALLA CRATE**

⛔ **È l'artefatto che questo compito produce davvero**, e la terza domanda del pre-controllo dice perché: *«un
tratto che nessuno implementa non è un tratto provato implementabile»*. Il banco stesso registra che su `process`
questa prova **trovò un difetto reale** — un valore di ritorno non costruibile fuori da `kernel`. In
`crates/kernel/tests/ports_are_implementable.rs`, ⛔ **con Python `newline=""`, e il file è CRLF anche
nell'INDICE (P-3)**: si rimisura con `git ls-files --eol`, non solo con `tr -cd '\r'`.

Accanto alle altre finte, con la sua intestazione a righe di `=` come le altre sezioni del file:

```rust
// ============================================================================================
// THE `custody` FAKE
// ============================================================================================

/// ⛔ A `Vec` OF PAIRS AND NOT A MAP, for the reason `crate::arbiter` writes down: `HashMap`
/// lives in `std`, and a bench that reaches for one teaches the wrong reflex about the crate it
/// is testing. With one key today the lookup is a scan of length one.
#[derive(Default)]
struct InMemoryCustody {
    kept: Vec<(CustodyKey, Vec<u8>)>,
    refuse: bool,
}

impl Custody for InMemoryCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        if self.refuse {
            return Err(CustodyError::Unavailable);
        }
        self.kept.retain(|(kept, _)| *kept != key);
        self.kept.push((key, bytes.to_vec()));
        Ok(())
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        if self.refuse {
            return Err(CustodyError::Unavailable);
        }
        Ok(self
            .kept
            .iter()
            .find(|(kept, _)| *kept == key)
            .map(|(_, bytes)| bytes.clone()))
    }
}
```

E il banco, con **entrambe le direzioni**, che è la forma che ogni finta di questo file già usa
(*«And refusable, same rule 3 as above»*):

```rust
#[test]
fn the_custody_port_can_be_implemented_and_called() {
    let mut custody = InMemoryCustody::default();

    // Nothing kept yet -- and that is a VALUE, not a failure: the first run.
    assert_eq!(custody.retrieve(CustodyKey::Layout), Ok(None));

    // ⛔ BYTES THAT ARE NOT JSON, on purpose: the package is opaque, and a fake that only ever
    // sees well-formed JSON would let a parsing implementation through. The real probe of this
    // property lives with the implementations (task 5); here it keeps the FAKE honest.
    let package = vec![0x00, 0xFF, 0x7B, 0x00];
    custody
        .keep(CustodyKey::Layout, &package)
        .expect("the fake kept it");
    assert_eq!(custody.retrieve(CustodyKey::Layout), Ok(Some(package)));

    // Replacing, not appending.
    custody
        .keep(CustodyKey::Layout, b"second")
        .expect("the fake kept it");
    assert_eq!(
        custody.retrieve(CustodyKey::Layout),
        Ok(Some(b"second".to_vec()))
    );

    // And refusable, same rule 3 as above -- BOTH operations, because the consumer reads the
    // difference between "write refused" and "unavailable" from WHICH ONE fails.
    custody.refuse = true;
    assert_eq!(
        custody.keep(CustodyKey::Layout, b"third"),
        Err(CustodyError::Unavailable)
    );
    assert_eq!(
        custody.retrieve(CustodyKey::Layout),
        Err(CustodyError::Unavailable)
    );
}
```

⚠️ **L'`use` in testa al file cresce:** `use kernel::ports::custody::{Custody, CustodyError, CustodyKey};`, nello
stile dei `use` già presenti. E la riga 1 del modulo — *«One fake per port declared WITHOUT an implementation»* —
riceve il proprio richiamo **nel compito 2** (**P-23**), non qui.

- [ ] **Passo 5: le cifre in prosa di `ports/mod.rs`, e la guardia di P-21**

⛔ **Le righe si ritrovano col `grep` sulla FRASE, mai col numero** (gotcha #70), e ogni riga che il censimento
rende si legge **intera**. Cinque tocchi, e il quinto è quello che nessuno si aspetta:

| | La frase | Diventa |
|---|---|---|
| 1 | *«The SIX families of ports (§2.3)»* | **SEVEN**, col richiamo datato accanto |
| 2 | *«all six are named in this milestone»* | resta **six**: parla del **Traguardo 1**, che ne nominò sei — è un fatto datato, non un conteggio di oggi. ⛔ **Non si tocca**, ed è scritto qui perché il prossimo censimento non lo corregga per zelo |
| 3 | *«this module declares SIX submodules, one per row»* | **SEVEN** |
| 4 | *«The other FOUR … have NO CALLER AT ALL»* | ⛔ **non si tocca QUI:** è **P-23**, e la corregge il compito **2**, che è quello che dà a `ipc` il chiamante. Toccarla qui sarebbe correggere il sintomo nel file sbagliato |
| 5 | *«FIVE fakes, because `process` needs two of them»* | **SIX fakes**. ⚠️ La riga sotto porta già un **richiamo datato del 2026-08-28** (AUD-054): si **legge prima**, e il richiamo nuovo si aggiunge **senza cancellarlo** |
| 6 | ⛔ *«the simulator substitutes SEVEN things while §2.3 enumerates SIX … so that nobody "fixes" the discrepancy … **or by writing "seven families" in the line above**»* | **EIGHT contro SEVEN** — ed è **P-21** |

Il richiamo del sesto, che è il delicato — il *Trova* si prende **dal file**:

```rust
//! ✅ DATED RECALL, 2026-09-11 -- THE NUMBERS MOVED AND THE WARNING STANDS, WHICH IS THE WHOLE
//! POINT OF DATING IT RATHER THAN REWRITING IT. A SEVENTH FAMILY ARRIVED -- `custody`, the
//! layout the gui entrusts to the core (decision 15 of the GUI north star) -- so the simulator
//! now substitutes EIGHT things while §2.3 enumerates SEVEN. ⛔ THE DISCREPANCY DID NOT CLOSE,
//! IT MOVED: `rng` is still declared in §2.2 and still lives in `crate::rng`, and moving it
//! under this module is still the wrong fix.
//! ⛔ AND "seven families" IS NOW WRITTEN IN THE LINE ABOVE -- BY THIS RECALL, AND IT IS NOT THE
//! THING THE SENTENCE FORBIDS. The forbidden seven was the one that COUNTS `rng` as a family of
//! I/O; today's seven counts a real family and leaves `rng` exactly where it was. A reader who
//! sees the two sentences side by side should read this one: the warning is about WHAT IS
//! COUNTED, not about the digit.
```

- [ ] **Passo 6: i tre richiami nella spec — P-22**

⛔ **Tutti e tre nello stesso commit**, che è la quinta riga della disciplina dell'audit: *«un rimedio si chiude
su TUTTE le case della frase»*. Il file è **CRLF**: `replace_unique.py`, e si rimisura dopo.

(a) La riga **209**, il riquadro dell'anello 3. Il *Trova* è la riga intera presa dal file; si **aggiunge** in
coda al riquadro, senza toccare ciò che c'è:

```markdown
> ⛔ **RICHIAMO DEL 2026-09-11 — le famiglie sono SETTE, e il merito di questo riquadro RESTA VERO.** La frase
> d'apertura è al presente e in assoluto, e dal sotto-progetto 2 è falsa alla lettera: `custody` è la settima
> (§2.3, e decisione 15 della stella polare della GUI). ✅ **Ciò che il riquadro afferma non cambia:** l'anello 3
> non ne aggiunge una, ed è ancora la ragione per cui quella voce costò una sezione invece di una riscrittura. Si
> data invece di riscriverla perché è l'argomento di una voce chiusa, non un conteggio.
```

(b) La **§2.3**, riga della tabella. Si aggiunge in coda alla tabella delle famiglie:

```markdown
| `custody` — **tenere i byte che la GUI affida al core, e ridarli** | dichiarata qui il 2026-09-11, progettata nella §2 della [stella polare della GUI](2026-09-07-direzione-gui-design.md) |
```

più, **sotto** la tabella, il richiamo:

```markdown
> ⛔ **RICHIAMO DEL 2026-09-11 — la tabella passa da SEI a SETTE famiglie**, ed è la prima volta dal 2026-08-07
> (§2.3.1). La settima è `custody`: due operazioni, `keep` e `retrieve`, e **una chiave sola**, un enum chiuso.
> Il perché — e perché non il giornale — è la decisione 15 della stella polare della GUI, riassunta nel doc di
> `crates/kernel/src/ports/custody.rs`. ⚠️ **Il costo, dichiarato lì e qui:** la §3.1 dichiara di sostituire
> *«esattamente le porte della §2.3»*, quindi cresce con questa; e la campagna **C1** verifica da oggi un mondo
> più largo — **detto** invece che scoperto, che è gotcha #17 nella direzione giusta.
```

(c) La **§3.1**: la riga nella tabella, dopo `ipc` e prima di `rng`:

```markdown
| `custody` | `redb` su un file suo, una tabella, una chiave (ADR-0022, l'archivio «configurazione») | in memoria, come `MemoryJournal` |
```

e la **nota di lettura** in fondo alla §3.1 — ⛔ **la gemella della guardia di `ports/mod.rs`, e va coi numeri
nuovi**. Il *Trova* è il capoverso che comincia con `📌 Nota di lettura`, preso dal file:

```markdown
> 📌 Nota di lettura, senza conseguenze: `rng` è dichiarata in **§2.2**, non in §2.3. La
> frase qui sopra resta vera in ciò che afferma — non esistono altri punti in cui il mondo
> tocchi il kernel — ma l'elenco è di **otto** porte e la §2.3 ne enumera **sette**.
> ⛔ **RICHIAMO DEL 2026-09-11:** i due numeri erano **sette** e **sei**; sono cresciuti insieme con `custody`, e
> lo scarto resta **uno** ed è sempre `rng`. La stessa avvertenza, coi numeri di oggi, sta in
> `crates/kernel/src/ports/mod.rs`, che è l'altra casa di questa discrepanza — e le due si toccano **insieme**,
> o la prima che resta indietro mente in silenzio.
```

- [ ] **Passo 7: le due direzioni, misurate**

Una per volta, compilata, eseguita, e **revocata** con `git diff` a zero:

| | La mutazione | Atteso |
|---|---|---|
| **C1** | in `InMemoryCustody::keep`, togli la riga `self.kept.retain(...)` | `the_custody_port_can_be_implemented_and_called` **rosso** sulla sostituzione: `Some(b"second")` atteso, trovato il pacchetto vecchio — la sonda del rimpiazzo non è decorativa |
| **C2** | in `retrieve`, sostituisci il ramo `refuse` con `Ok(None)` | **rosso** sull'ultima asserzione: `Err(Unavailable)` atteso, `Ok(None)` trovato. ⛔ **È la direzione che decide**, perché senza di essa una porta che confonde «niente» con «non disponibile» passerebbe — ed è esattamente la distinzione che la decisione 35 compra |
| **C3** | in `custody.rs`, cambia `retrieve` in `-> Result<Vec<u8>, CustodyError>` | **il banco non compila**, `E0308`: è la prova che la firma è esercitata davvero e non solo dichiarata |

```bash
cargo test --locked -p kernel --test ports_are_implementable 2>&1 | tail -5
git diff --stat
```

Atteso dopo ogni revoca: il banco **verde**, e `git diff --stat` **vuoto**.

- [ ] **Passo 8: i fine-riga, il cancello, il commit**

```bash
for f in crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
tr -cd '\r' < crates/kernel/src/ports/custody.rs | wc -c
git diff --name-only -- docs/superpowers/specs/2026-08-06-kernel-design.md
bash scripts/gate.sh 2>&1 | tail -3
bash scripts/check-docs.sh 2>&1 | tail -2
```

Atteso: per i tre CRLF `CR` **uguale** alle righe; `git ls-files --eol` **invariato** — in particolare
`ports_are_implementable.rs` ancora **`i/crlf w/crlf`**; **zero** CR in `custody.rs`; ⛔ **l'altra spec NON
toccata**, il comando non rende nulla (vincolo 1); `GATE GREEN`; `OK`.

```bash
git add crates/kernel/src/ports/custody.rs crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md
git commit -m "gui(compito 4): la settima porta -- il tratto Custody con keep e retrieve e la chiave Layout, la finta InMemoryCustody che lo prova da fuori dalla crate nelle due direzioni; le cifre in prosa di ports/mod.rs da sei a sette con la guardia di rng riscritta coi numeri nuovi (P-21), e i tre richiami nella spec: la riga dell'anello 3, la 2.3 e la 3.1 con la sua nota di lettura (P-22)"
git push
```

#### Criterio di chiusura del compito 4

- [ ] `cargo test --locked -p kernel --test ports_are_implementable` → tutti passati, **uno in più** del Passo 1
- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/kernel/src/ports/mod.rs` → **almeno 1**, e la guardia di `rng` porta i numeri **otto/sette**
- [ ] `grep -c 'RICHIAMO DEL 2026-09-11' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` → **3**
- [ ] ⛔ `grep -c 'NO CALLER AT ALL' crates/kernel/src/ports/mod.rs` → **invariato**: quella riga è del compito **2** (P-23)
- [ ] `git diff --name-only -- docs/superpowers/specs/2026-08-06-kernel-design.md` **vuoto** (vincolo 1)
- [ ] le tre mutazioni C1, C2, C3 provate **una per volta** e revocate, con `git diff --stat` vuoto
- [ ] `bash scripts/gate.sh` → `GATE GREEN`; `bash scripts/gate-deps.sh` **verde**: la lista di ADR-0031 **non è cresciuta** (il modulo non ha dipendenze)
- [ ] i fine-riga rimisurati, `git ls-files --eol` invariato sui tre file toccati — `ports_are_implementable.rs` ancora `i/crlf`
- [ ] la riga **4** della tabella della posizione a ✅ con la data

---

## Compito 5: le due implementazioni della settima porta — `redb` in `platform`, la finta in `simulator`, e la suite che le confronta

**Files:**
- Create: `crates/simulator/src/custody.rs` (**LF**) — `MemoryCustody`
- Create: `crates/kernel/tests/custody_contract.rs` (**LF**) — le cinque promesse, i cinque bugiardi, il limite dichiarato di **P-29**
- Create: `crates/platform/src/custody.rs` (**LF**) — `FileCustody`, la tabella, `open(path)`
- Create: `crates/platform/tests/custody_contract_real.rs` (**LF**) — l'`include!` e la fabbrica su file
- Create: `crates/platform/tests/file_custody.rs` (**LF**) — ciò che **solo** l'implementazione vera promette
- Modify: `crates/simulator/src/lib.rs` (**`i/lf w/crlf`**) — `pub mod custody;`
- Modify: `crates/platform/src/lib.rs` (**`i/lf w/crlf`**) — la riga di modulo **e il richiamo datato di P-24**, sotto quello che il compito 2 ha già scritto
- Modify: `crates/platform/src/journal.rs` (**`i/lf w/crlf`**) — ⛔ **una parola**: `engine` da privata a `pub(crate)`, con la riga che dice chi la chiama adesso (**D14**)
- Read: la **§2 della stella polare**, pezzi 1, 3, 4 e 6, e la riga della settima porta della tabella degli artefatti della **§8 del 2**; `crates/platform/src/journal.rs` **per intero** — `FileBackend`, `OpenError`, `with_backend`, la ragione per cui la tabella si crea a ogni apertura; la **testa** di `crates/kernel/tests/journal_contract.rs` e i suoi `assert_caught_on` / `message_the_suite_fails_with` / `panic_message`; `crates/platform/tests/journal_contract_real.rs` **per intero**; `crates/platform/tests/file_journal.rs` per `private_dir_for_line` e le due direzioni del lucchetto

**Interfaces:**
- Consumes, dal **compito 4**: `kernel::ports::custody::{Custody, CustodyKey, CustodyError}`, con
  `keep(&mut self, CustodyKey, &[u8]) -> Result<(), CustodyError>` e
  `retrieve(&self, CustodyKey) -> Result<Option<Vec<u8>>, CustodyError>`; `CustodyKey::Layout`;
  `CustodyError::Unavailable`. Tutti e tre derivano `Debug + Clone + Copy + PartialEq + Eq`
- Consumes, da `platform`: `platform::journal::{FileBackend, OpenError}` — `FileBackend::open(&Path)` è già `pub`,
  e `OpenError` si **riusa** al suo percorso (**D14**)
- Produces, e i compiti **8**, **9** e **11** li usano con questi nomi esatti:
  - `platform::custody::FileCustody`, con `FileCustody::open(path: &Path) -> Result<FileCustody, platform::journal::OpenError>`
  - `simulator::custody::MemoryCustody`, con `MemoryCustody::new() -> MemoryCustody` — ⚠️ **`const fn`, e nessun `impl Default`**, come `MemoryJournal`
  - `assert_custody_contract<C: Custody, F: Fn() -> C>(build: F)` in `crates/kernel/tests/custody_contract.rs`, raggiunta con `include!`

⛔ **NESSUN `with_backend` SU `FileCustody`, e non è una dimenticanza.** Il giornale ne ha uno perché è lì che il
Traguardo 4 inietta il guasto di livello 2, e `crates/platform/tests/engine_crash_consistency.rs` lo chiama. Per la
custodia nessuno lo chiamerebbe: il confine `FileBackend` è **già** provato sostituibile da fuori dalla crate dal
banco del giornale, e ciò che la DST sostituisce qui è la **porta intera** — la finta del simulatore, §2 pezzo 4
della stella polare. Un secondo punto d'ingresso senza chiamante è lo strato speculativo che il criterio 5 rifiuta.
✅ Se una campagna lo chiederà, arriva **con** il suo chiamante.

- [ ] **Passo 1: le misure prima**

```bash
for f in crates/simulator/src/custody.rs crates/kernel/tests/custody_contract.rs crates/platform/src/custody.rs crates/platform/tests/custody_contract_real.rs crates/platform/tests/file_custody.rs; do printf '%-52s ' "$f"; test -e "$f" && echo ESISTE || echo no; done
ls crates/kernel/src/ports/custody.rs
grep -rEn "^impl (Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for " crates/platform/src/
grep -n 'fn engine' crates/platform/src/journal.rs
grep -c '^#\[test\]' crates/kernel/tests/journal_contract.rs
grep -rn 'daemon-file-journal-\|daemon-journal-contract-' crates/platform/tests/
git ls-files --eol crates/platform/src/lib.rs crates/simulator/src/lib.rs crates/platform/src/journal.rs
```

Atteso: i **cinque** file **non esistono**; `crates/kernel/src/ports/custody.rs` **esiste** (compito 4);
il `grep` chiuso rende le implementazioni di oggi — ⛔ **quante, non si scrive qui: il compito 2 ne ha aggiunta una
e il numero dipende da quali compiti sono passati**; `fn engine` è **privata**; i due prefissi di directory in uso
sono `daemon-file-journal-` e `daemon-journal-contract-`, e i due nuovi devono essere **diversi da entrambi**;
i tre file da modificare sono `i/lf w/crlf`.

- [ ] **Passo 2: la suite, e il rosso che deve dare**

`crates/kernel/tests/custody_contract.rs`, **LF**. ⛔ **Commenti normali e non `//!`**: il file si espande con
`include!` in posizione di item, e un attributo interno lì non è ammesso.

```rust
// THE CONFORMANCE SUITE OF THE `custody` PORT (§2 of the GUI north star). It is worth what the
// two implementations answering the SAME questions is worth: the in-memory double that the DST
// campaign runs against, and `redb` under `platform`.
//
// ⛔ REGULAR COMMENTS AND NOT `//!`, BECAUSE THIS FILE IS `include!`d.
// `crates/platform/tests/custody_contract_real.rs` expands it IN ITEM POSITION, and an inner
// attribute -- which is what `//!` desugars to -- is not permitted there. Same mechanism as
// `journal_contract.rs`, and for the same reason: two copies of the assertions would drift, and
// THE FIRST ONE TO DRIFT WOULD PRINT `ok` WHILE COMPARING NOTHING.
//
// ⛔ A FUNCTION AND NOT A `macro_rules!`, WHICH IS THE OTHER SHAPE IN THIS VERY MILESTONE, so the
// difference is written down rather than left to look like an inconsistency somebody should
// tidy. `ipc_contract.rs` generates its `#[test]`s from a macro because EACH CRATE HANDS ITS OWN
// FACTORY: its real promises need a peer writing bytes, which a fake has not got. `custody` has
// no peer -- one factory is enough, and `kernel` builds `simulator::custody::MemoryCustody`
// itself, because `simulator` is already one of its dev-dependencies. Shape borrowed from
// `journal_contract.rs`, which the north star names by name.
//
// ⛔ WHAT THIS SUITE CANNOT HOLD, AND IT IS MEASURED RATHER THAN SUSPECTED: AN IMPLEMENTATION
// THAT IGNORES THE KEY. `CustodyKey` has ONE variant, so on an archive holding one key "the
// package under THIS key" and "the only package there is" ARE THE SAME PACKAGE -- by
// construction, not by how this bench is written. A `retrieve` that never looked at its argument
// would pass every promise below. The journal closed the twin defect (finding AUD-019) by
// putting a BYSTANDER in the archive; here a bystander IS NOT BUILDABLE.
// ⚠️ MEASURED, not reasoned: task 5 wrote the key-blind mutation, ran this suite against it and
// WATCHED IT PASS, then revoked it -- the record is in that task's commit.
// ⛔ THE TRIGGER IS THE SECOND KEY: whoever adds a variant to `CustodyKey` adds a bystander here
// in the same commit, and this paragraph goes. A second variant added without it would leave the
// port's central promise unheld and nothing would go red. Written here because nobody would
// rediscover it.
//
// ⚠️ WHAT IS DELIBERATELY ABSENT, the other half: SURVIVING A REOPENING. It is a promise of the
// REAL implementation alone -- the in-memory double cannot make it and is CORRECT not to, and
// asserting it here would turn a correct implementation red (gotcha #44). It lives in
// `crates/platform/tests/file_custody.rs`.

use kernel::ports::custody::{Custody, CustodyError, CustodyKey};

/// ⛔ ONE MESSAGE PER PROMISE, AND NOT ONE SHARED -- the rule `journal_contract.rs` states and
/// `reactor_contract.rs` learned the hard way: with a shared message a liar caught by promise 1
/// would be indistinguishable from one caught by promise 4, in exactly the place built to tell
/// them apart. They share `custody contract violated: ` and diverge immediately after, because
/// the negative tests match with `contains`.
pub const KEPT_COMES_BACK_MESSAGE: &str =
    "custody contract violated: what `keep` wrote must come back from `retrieve` byte for byte";

/// Promise 1 of §2 of the north star, in one line: the package is OPAQUE.
pub const OPAQUE_MESSAGE: &str =
    "custody contract violated: bytes that are not text must be kept and handed back untouched";

pub const NOTHING_KEPT_MESSAGE: &str =
    "custody contract violated: a key with nothing under it answers Ok(None), never an error";

pub const REPLACES_MESSAGE: &str =
    "custody contract violated: a second `keep` under one key must REPLACE what was there";

/// The gotcha #30 family: a bench that only looks at `Ok`/`Err` does not see the WRONG ANSWER.
pub const EMPTY_IS_NOT_ABSENT_MESSAGE: &str =
    "custody contract violated: an empty package is KEPT, and must not come back as nothing";

/// Every promise the `custody` port makes, checked against ONE implementation.
///
/// It takes a FACTORY and not a custody because several blocks need one that has never been
/// written to, and `keep` has no undo.
///
/// ⛔ THE ORDER OF THE BLOCKS IS PART OF THE SUITE, because it stops at the FIRST promise an
/// implementation breaks. Every liar below therefore has to survive every promise ahead of its
/// own and die on that one -- which is the property each negative test measures by reading the
/// panic payload instead of settling for `is_err()`.
pub fn assert_custody_contract<C: Custody, F: Fn() -> C>(build: F) {
    // ── 1. What `keep` writes, `retrieve` hands back byte for byte ────────────────────────
    {
        let mut custody = build();
        let written: &[u8] = br#"{"grid":{"root":{"type":"branch"}},"activeView":"home"}"#;

        custody
            .keep(CustodyKey::Layout, written)
            .expect(KEPT_COMES_BACK_MESSAGE);

        let read = custody
            .retrieve(CustodyKey::Layout)
            .expect(KEPT_COMES_BACK_MESSAGE);

        assert_eq!(read.as_deref(), Some(written), "{}", KEPT_COMES_BACK_MESSAGE);
    }

    // ── 2. Bytes that are not text survive unchanged ──────────────────────────────────────
    // ⛔ THIS IS PIECE 1 OF §2 OF THE NORTH STAR MADE EXECUTABLE: "the day `dockview` changes
    // format, the core does not change". An implementation that parsed, validated or
    // canonicalised the package would satisfy promise 1 -- the payload there IS valid JSON and
    // valid UTF-8 -- and die here. That is why the two blocks are not one.
    //
    // ⚠️ THE PAYLOAD IS CHOSEN, NOT RANDOM: `0xff` and `0x80` are not valid UTF-8 in any
    // position, `0x00` is what a C string stops at, and the `{` in the middle is there so that
    // something looking for JSON finds a plausible start and then fails.
    {
        let mut custody = build();
        let written: &[u8] = &[0xff, 0x00, 0x1b, b'{', 0xfe, b'\n', 0x80, 0x7f];

        custody
            .keep(CustodyKey::Layout, written)
            .expect(OPAQUE_MESSAGE);

        let read = custody.retrieve(CustodyKey::Layout).expect(OPAQUE_MESSAGE);

        assert_eq!(read.as_deref(), Some(written), "{}", OPAQUE_MESSAGE);
    }

    // ── 3. A key with nothing under it answers Ok(None) ───────────────────────────────────
    // ⛔ `Ok(None)` AND NOT AN ERROR, and the port's doc argues why: this is the FIRST RUN, the
    // ordinary case, and folding it into an error would make the commonest path look like a
    // failure. The assertion is on the WHOLE `Result`, so an implementation answering
    // `Err(Unavailable)` dies here instead of being quietly unwrapped away.
    {
        let custody = build();
        assert_eq!(
            custody.retrieve(CustodyKey::Layout),
            Ok(None),
            "{}",
            NOTHING_KEPT_MESSAGE
        );
    }

    // ── 4. A second `keep` REPLACES ───────────────────────────────────────────────────────
    // ⛔ AND THE TWO PAYLOADS DIFFER IN LENGTH AS WELL AS IN CONTENT, deliberately: an
    // implementation that APPENDED and handed back the concatenation would be caught by the
    // content alone, but one that kept the longer of the two would not -- so the replacement is
    // the SHORTER one. The same care promise 1 of `journal_contract.rs` takes with its bystander.
    {
        let mut custody = build();
        let first: &[u8] = b"the layout as it was when the window opened";
        let second: &[u8] = b"and as it is now";

        custody.keep(CustodyKey::Layout, first).expect(REPLACES_MESSAGE);
        custody
            .keep(CustodyKey::Layout, second)
            .expect(REPLACES_MESSAGE);

        let read = custody.retrieve(CustodyKey::Layout).expect(REPLACES_MESSAGE);

        assert_eq!(read.as_deref(), Some(second), "{}", REPLACES_MESSAGE);
    }

    // ── 5. An empty package is kept, and is NOT "nothing" ─────────────────────────────────
    // ⛔ THE DISTINCTION THAT AN `Option` INVITES YOU TO LOSE. An implementation holding
    // `Option<Vec<u8>>` and treating an empty slice as "no package" satisfies promises 1 to 4 --
    // none of them ever keeps an empty one -- and makes "kept, and empty" indistinguishable from
    // "never kept". That is the same family as gotcha #30, and the same sentence ADR-0018 spends
    // on pruned payloads: an absence and an emptiness must not look alike.
    //
    // ⚠️ AND IT IS NOT A HYPOTHETICAL SHAPE: it is the FIRST shape an in-memory double takes if
    // nobody says otherwise, which is why the liar for it is written below.
    {
        let mut custody = build();

        custody
            .keep(CustodyKey::Layout, b"")
            .expect(EMPTY_IS_NOT_ABSENT_MESSAGE);

        let read = custody
            .retrieve(CustodyKey::Layout)
            .expect(EMPTY_IS_NOT_ABSENT_MESSAGE);

        assert_eq!(
            read.as_deref(),
            Some(&b""[..]),
            "{}",
            EMPTY_IS_NOT_ABSENT_MESSAGE
        );
    }
}

#[test]
fn the_in_memory_custody_honours_the_contract() {
    assert_custody_contract(simulator::custody::MemoryCustody::new);
}
```

```bash
cargo test --locked -p kernel --test custody_contract 2>&1 | tail -12
```

Atteso: **rosso di compilazione**, `E0433` o `E0432` su `simulator::custody` — il modulo non esiste ancora.
⛔ **Questo rosso è il passo, non un intoppo:** è la sola prova che il banco stia davvero cercando la finta e non
un'altra cosa.

- [ ] **Passo 3: la finta del simulatore, e il verde**

`crates/simulator/src/custody.rs`, **LF**:

```rust
//! The in-memory `custody` (§2 of the GUI north star). One of the two implementations the
//! conformance suite runs against; the other is `redb` under `platform`.
//!
//! ⛔ THERE IS NO FALLING DOUBLE HERE, unlike `journal.rs`. Failing at an operation chosen by
//! the seed is fault injection, and nothing asks for it on this port yet: the campaign of
//! milestone 2 of the sub-project substitutes the WORKING one. The day a campaign wants a
//! falling custody it wraps this type, exactly as `CrashingJournal` wraps `MemoryJournal`, and
//! for the same reason -- one archive, not two truths to hold in step.

use alloc::vec::Vec;

use kernel::ports::custody::{Custody, CustodyError, CustodyKey};

/// Keeps the packages in memory, one per key.
///
/// ⛔ A `Vec` OF PAIRS AND NOT AN `Option`, AND WITH ONE KEY THAT LOOKS LIKE CEREMONY. It is not.
/// An `Option<Vec<u8>>` would be BLIND TO THE KEY BY CONSTRUCTION -- `retrieve` would have
/// nothing to compare -- and the conformance suite CANNOT CATCH THAT with one variant, which is
/// measured and written in that file's head. So the shape is the guard: this type looks the key
/// up, and keeps on looking it up when a second one arrives.
///
/// ⚠️ AND NOT A `HashMap`, which is the rule of this crate rather than a preference:
/// `RandomState` is seeded per process and the iteration order is not reproducible in a
/// deterministic world -- gotcha #12. `MemoryJournal` keeps a `Vec` for the same reason.
pub struct MemoryCustody {
    packages: Vec<(CustodyKey, Vec<u8>)>,
}

// ⛔ NO `impl Default`, AND ITS ABSENCE IS THE DECISION -- the same one, for the same reason, as
// `MemoryJournal`, `SystemReactor` and `VirtualReactor`: nothing calls it, and this repository
// removes such items rather than keeping them for symmetry. `cargo clippy` asks for one
// (`new_without_default`); the warning is ACCEPTED and NOT silenced, because §7.4.3 gives clippy
// no voice in the gate and an `#[allow]` would hide the next occurrence too. The argument is
// written out once, in `crates/platform/src/reactor.rs`.
impl MemoryCustody {
    pub const fn new() -> Self {
        MemoryCustody {
            packages: Vec::new(),
        }
    }
}

impl Custody for MemoryCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        // ⛔ REPLACE IN PLACE AND NOT PUSH. Pushing would make `retrieve` depend on whether it
        // reads the first match or the last, which is the defect `AppendingCustody` wears in the
        // suite -- and the port's doc says `keep` replaces "whatever was there".
        match self.packages.iter_mut().find(|(kept, _)| *kept == key) {
            Some((_, package)) => {
                package.clear();
                package.extend_from_slice(bytes);
            }
            None => self.packages.push((key, bytes.to_vec())),
        }
        Ok(())
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self
            .packages
            .iter()
            .find(|(kept, _)| *kept == key)
            .map(|(_, package)| package.clone()))
    }
}
```

In `crates/simulator/src/lib.rs` (**`i/lf w/crlf`**), i moduli sono in ordine alfabetico — `custody` va **prima**
di `ipc`:

```
pub mod custody;
```

```bash
cargo test --locked -p kernel --test custody_contract 2>&1 | tail -8
```

Atteso: **verde**, un test — `the_in_memory_custody_honours_the_contract`.

- [ ] **Passo 4: i cinque bugiardi, e i cinque rossi**

In coda a `crates/kernel/tests/custody_contract.rs`. ⛔ **Gli aiutanti sono gli stessi di
`journal_contract.rs`, riscritti per questo tratto e non importati:** un banco d'integrazione è una crate a sé e
non può usare gli item di un altro.

```rust
// ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3): a suite never seen to fail is not a suite. The
// five below break the port's promises ONE EACH, and demand that the suite notices each -- and
// notices it ON THE RIGHT PROMISE, which is what reading the payload buys over `is_err()`.
//
// ⚠️ AND EACH IS BROKEN IN A DIFFERENT WAY (gotcha #45): the write dropped, the bytes
// canonicalised, an absence reported as a failure, a replacement turned into an append, and an
// empty package filed as no package. Two liars broken the same way prove one thing twice and
// leave the other promise unguarded.
//
// ⛔ AND THE ONE THAT IS NOT HERE: a custody BLIND TO THE KEY. It would pass, and the head of
// this file says why and records the measurement. Writing it and asserting it is caught would be
// a green that proves nothing.

#[test]
fn a_custody_that_writes_nothing_is_caught() {
    assert_caught_on(SilentCustody::new, KEPT_COMES_BACK_MESSAGE, "promise 1");
}

#[test]
fn a_custody_that_canonicalises_the_bytes_is_caught() {
    assert_caught_on(TextCustody::new, OPAQUE_MESSAGE, "promise 2");
}

#[test]
fn a_custody_that_errors_instead_of_answering_nothing_is_caught() {
    assert_caught_on(
        ErrorInsteadOfNothingCustody::new,
        NOTHING_KEPT_MESSAGE,
        "promise 3",
    );
}

#[test]
fn a_custody_that_appends_instead_of_replacing_is_caught() {
    assert_caught_on(AppendingCustody::new, REPLACES_MESSAGE, "promise 4");
}

#[test]
fn a_custody_that_files_an_empty_package_as_no_package_is_caught() {
    assert_caught_on(
        EmptyIsAbsentCustody::new,
        EMPTY_IS_NOT_ABSENT_MESSAGE,
        "promise 5",
    );
}

#[test]
fn no_promise_message_is_a_substring_of_another() {
    // ⛔ THE CONSTRAINT THAT MAKES `contains` SAFE. If one message were a substring of another, a
    // liar caught on the WRONG promise would still satisfy the test that names the right one --
    // the suite would keep printing `ok` while pointing at the wrong place. It is a property of
    // the SET, so every message added has to be checked against ALL the others, which is exactly
    // the check nobody repeats by eye.
    let messages = [
        ("KEPT_COMES_BACK", KEPT_COMES_BACK_MESSAGE),
        ("OPAQUE", OPAQUE_MESSAGE),
        ("NOTHING_KEPT", NOTHING_KEPT_MESSAGE),
        ("REPLACES", REPLACES_MESSAGE),
        ("EMPTY_IS_NOT_ABSENT", EMPTY_IS_NOT_ABSENT_MESSAGE),
    ];

    for (name, message) in messages {
        for (other_name, other) in messages {
            if name == other_name {
                continue;
            }
            assert!(
                !other.contains(message),
                "{name} is a substring of {other_name}: a liar caught on {other_name} would \
                 satisfy the test that names {name}"
            );
        }
    }

    // The other direction, the one that gets forgotten (§7.1.1 rule 3): a bench where every
    // message were distinct BY BEING EMPTY would pass the loop above without saying anything.
    for (name, message) in messages {
        assert!(!message.is_empty(), "{name} is empty");
    }
}

fn assert_caught_on<C, F>(build: F, expected: &str, promise: &str)
where
    C: Custody,
    F: Fn() -> C + std::panic::RefUnwindSafe,
{
    let message = message_the_suite_fails_with(build).unwrap_or_else(|| {
        panic!("THE SUITE IS VACUOUS ON {promise}: a custody that breaks it passed the suite")
    });
    assert!(
        message.contains(expected),
        "the suite did fire, but NOT on {promise} — so {promise} is still unproven.\n\
         expected to contain: {expected}\n\
         actual payload: {message}"
    );
}

/// Runs the suite and returns the message it failed with, or `None` if it passed.
///
/// ⚠️ The panic hook is silenced for the duration of the call: the panic is EXPECTED, and its
/// backtrace in the test output would train the reader to ignore backtraces. Restored
/// immediately. ⛔ DECLARED LIMIT, the same one `journal_contract.rs` declares: the hook is
/// PROCESS-WIDE and libtest runs tests on parallel threads, so a panic raised in another test
/// landing inside this window is reported with no stdout section. The failure is never hidden,
/// only its message, and the window is microseconds wide.
fn message_the_suite_fails_with<C, F>(build: F) -> Option<String>
where
    C: Custody,
    F: Fn() -> C + std::panic::RefUnwindSafe,
{
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(|| assert_custody_contract(&build));
    std::panic::set_hook(previous);

    match outcome {
        Ok(()) => None,
        Err(payload) => Some(panic_message(payload.as_ref())),
    }
}

/// The text of a panic, dug out of the payload. `assert!`/`assert_eq!` with a format argument
/// panic with a `String`; a `panic!("literal")` carries a `&str` instead, and both are handled so
/// that this helper cannot report nothing for a message that is right there.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string()
    } else {
        String::from("<panic payload that is neither String nor &str>")
    }
}

/// Answers `Ok(())` and keeps nothing. Caught by promise 1.
struct SilentCustody;

impl SilentCustody {
    fn new() -> Self {
        SilentCustody
    }
}

impl Custody for SilentCustody {
    fn keep(&mut self, _key: CustodyKey, _bytes: &[u8]) -> Result<(), CustodyError> {
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(None)
    }
}

/// Keeps only what is valid text and drops the rest. ⛔ THIS IS THE SHAPE AN IMPLEMENTATION
/// TAKES THE DAY SOMEBODY DECIDES THE PACKAGE "IS JSON ANYWAY": it survives promise 1, whose
/// payload is valid UTF-8, and dies on promise 2. Caught by promise 2.
struct TextCustody {
    kept: Option<Vec<u8>>,
}

impl TextCustody {
    fn new() -> Self {
        TextCustody { kept: None }
    }
}

impl Custody for TextCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept = Some(String::from_utf8_lossy(bytes).into_owned().into_bytes());
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self.kept.clone())
    }
}

/// Keeps correctly, and reports an absence as a failure. Caught by promise 3.
struct ErrorInsteadOfNothingCustody {
    kept: Option<Vec<u8>>,
}

impl ErrorInsteadOfNothingCustody {
    fn new() -> Self {
        ErrorInsteadOfNothingCustody { kept: None }
    }
}

impl Custody for ErrorInsteadOfNothingCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept = Some(bytes.to_vec());
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        match &self.kept {
            Some(package) => Ok(Some(package.clone())),
            None => Err(CustodyError::Unavailable),
        }
    }
}

/// Piles the packages up and hands back the FIRST. ⛔ It sails through promises 1, 2 and 3,
/// which never keep twice, and the concatenating variant of the same defect would too. Caught by
/// promise 4.
struct AppendingCustody {
    kept: Vec<Vec<u8>>,
}

impl AppendingCustody {
    fn new() -> Self {
        AppendingCustody { kept: Vec::new() }
    }
}

impl Custody for AppendingCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept.push(bytes.to_vec());
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self.kept.first().cloned())
    }
}

/// Files an empty package as no package. ⛔ THE SHAPE AN `Option` INVITES, and the reason
/// promise 5 exists: it answers every other promise correctly. Caught by promise 5.
struct EmptyIsAbsentCustody {
    kept: Option<Vec<u8>>,
}

impl EmptyIsAbsentCustody {
    fn new() -> Self {
        EmptyIsAbsentCustody { kept: None }
    }
}

impl Custody for EmptyIsAbsentCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept = if bytes.is_empty() {
            None
        } else {
            Some(bytes.to_vec())
        };
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self.kept.clone())
    }
}
```

```bash
cargo test --locked -p kernel --test custody_contract 2>&1 | tail -14
```

Atteso: **tutti verdi**, e il conteggio lo dà `grep -c '^#\[test\]' crates/kernel/tests/custody_contract.rs`.
⛔ **Se uno dei cinque dicesse `THE SUITE IS VACUOUS ON promise N`, la promessa N è vacua e si corregge la
PROMESSA, non il bugiardo**; se dicesse `fired, but NOT on promise N`, l'ordine dei blocchi è sbagliato e il
bugiardo muore prima.

- [ ] **Passo 5: l'implementazione vera, e il banco che la include**

Prima, in `crates/platform/src/journal.rs` (**`i/lf w/crlf`**), **una parola** — **D14**:

```rust
/// Every `redb` error type converts into `redb::Error`, so the five that `open` can meet are
/// folded into one variant here instead of five.
///
/// ⚠️ `pub(crate)` SINCE 2026-09-11, AND THE SECOND CALLER IS `crate::custody`: the seventh port
/// opens a `redb` archive of its own on the same `FileBackend`, so it meets the same five errors
/// and folds them the same way. The alternative was a twin `OpenError` under `custody`, and the
/// reason it was refused is written there, beside `FileCustody::open`.
pub(crate) fn engine(error: impl Into<redb::Error>) -> OpenError {
```

Poi `crates/platform/src/custody.rs`, **LF**:

```rust
//! The real `custody` (§2 of the GUI north star): a `redb` archive OF ITS OWN, on the
//! `FileBackend` the journal already uses.
//!
//! ⛔ ITS OWN FILE AND ITS OWN TABLE, NOT A SECOND TABLE IN THE JOURNAL'S. ADR-0022 separates
//! archives BY NATURE and gives each its own policy: the journal is encrypted, pruned and in the
//! backup; this is the "configuration" archive -- NOT encrypted, in the backup, permanent. Two
//! natures in one file would be one policy for both, and the retention milestone would owe this
//! package an exception written just for it. That is decision 15 of the north star in full.
//!
//! ⚠️ AND IT IS THE SAME `FileBackend`, WHICH IS NOT A CONTRADICTION: the backend is the boundary
//! at which level-2 faults are injected (ADR-0032 requirement 4), and sharing the TYPE is what
//! would make this archive injectable the day a campaign asks. What is not shared is the FILE.
//!
//! ⛔ NO DEPENDENCY IS ADDED, and it is measured rather than assumed: `redb` is already in this
//! crate's manifest for the journal, and `scripts/gate-deps.sh` measures the graphs of `kernel`
//! and `simulator`, neither of which sees it.
//!
//! ⛔ WHAT BOTH IMPLEMENTATIONS PROMISE IS NOT WRITTEN HERE: it is in
//! `crates/kernel/tests/custody_contract.rs`, reached by `include!` from
//! `crates/platform/tests/custody_contract_real.rs`. What only THIS one promises -- surviving a
//! reopening, and refusing a second custody on an open file -- is in
//! `crates/platform/tests/file_custody.rs`.

use std::path::Path;

use kernel::ports::custody::{Custody, CustodyError, CustodyKey};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};

use crate::journal::{FileBackend, OpenError, engine};

/// The one table: one package per key.
///
/// ⚠️ `u8` AND NOT `&str`, and the reason is the port's rather than this file's: `CustodyKey` is
/// a CLOSED ENUM precisely so that the archive has no namespace, and a string key here would put
/// the namespace back where nobody would see it. `u8` is a `redb` `Key` -- read in
/// `redb-4.1.0/src/types.rs`, `le_impl!(u8)`, which gives both `Value` and `Key` with a NUMERIC
/// comparison -- so nothing about the ordering can surprise us.
const PACKAGES: TableDefinition<u8, &[u8]> = TableDefinition::new("custody-packages");

/// The byte `CustodyKey::Layout` is stored under.
const KEY_LAYOUT: u8 = 0;

/// ⛔ A `match` AND NOT `key as u8`, AND THE DIFFERENCE IS THE COMPILER. A cast would give a
/// SECOND variant its byte BY POSITION, so reordering the enum would silently repoint every
/// package already on the disk -- a migration nobody asked for, arriving in silence. A `match`
/// makes a new variant an `E0004` right here, which is where that decision belongs.
fn byte_of(key: CustodyKey) -> u8 {
    match key {
        CustodyKey::Layout => KEY_LAYOUT,
    }
}

/// The `custody` port against a real file.
///
/// ⛔ NO `with_backend`, UNLIKE `FileJournal`, AND THE ABSENCE IS THE DECISION. The journal has
/// one because milestone 4 injects level-2 faults through it and
/// `crates/platform/tests/engine_crash_consistency.rs` really calls it. Here nobody would: the
/// `FileBackend` boundary is ALREADY proven substitutable from outside the crate by the
/// journal's bench, and what the simulation substitutes on this port is the WHOLE PORT -- piece
/// 4 of §2 of the north star. A second entry point with no caller is the speculative layer this
/// repository refuses. It arrives WITH its caller, or not at all.
pub struct FileCustody {
    database: Database,
}

impl FileCustody {
    /// Opens the archive at `path`, creating it if it is not there.
    ///
    /// ⛔ IT ANSWERS `platform::journal::OpenError`, AND THE ODD PATH IS DELIBERATE -- written
    /// here because whoever reads the signature is exactly who would ask. That type's three
    /// variants -- a file that would not open, a file another handle already holds, an engine
    /// that refused it -- are exactly what OPENING A `redb` ARCHIVE ON A `FileBackend` can
    /// produce, and this opens the same thing. The two alternatives were examined on 2026-09-11:
    /// MOVING it up to `platform::OpenError` would falsify two doc paragraphs IN THE KERNEL --
    /// `crates/kernel/src/framing.rs` and `crates/kernel/src/permission.rs` both name
    /// `platform::journal::OpenError` as the shape they follow -- and DUPLICATING it would leave
    /// two types to hold in step, the first to drift lying in silence. If a THIRD archive ever
    /// arrives, the type moves up and those two kernel paragraphs are corrected in the same
    /// commit; until then it stays where its callers can find it.
    ///
    /// ⚠️ THE LOCK COMES WITH `FileBackend` AND IS NOT RE-ARGUED HERE: a second custody on an
    /// open file is refused, which `crates/platform/tests/file_custody.rs` holds in BOTH
    /// directions.
    pub fn open(path: &Path) -> Result<Self, OpenError> {
        let backend = FileBackend::open(path)?;
        let database = Database::builder()
            .create_with_backend(backend)
            .map_err(engine)?;

        // ⛔ THE TABLE IS CREATED HERE, ON EVERY OPEN, so that every later READ finds it. A
        // `redb` table springs into existence when a WRITE transaction opens it; a read
        // transaction on a fresh file would answer `TableDoesNotExist`, and `retrieve` would have
        // to launder that into `Unavailable` -- reporting a broken archive where the truth is a
        // FIRST RUN, which is exactly the confusion promise 3 exists to forbid. The journal does
        // the same, for the same reason. ⚠️ DECLARED COST: opening always commits, so `open`
        // writes to the disk even when nothing is ever kept.
        let transaction = database.begin_write().map_err(engine)?;
        transaction.open_table(PACKAGES).map_err(engine)?;
        transaction.commit().map_err(engine)?;

        Ok(FileCustody { database })
    }
}

impl Custody for FileCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        // ⛔ ONE VARIANT FOR EVERY FAILURE, AND IT IS THE PORT'S DECISION AND NOT A SHORTCUT:
        // `CustodyError` has one variant because the caller reads the reason FROM WHICH
        // OPERATION FAILED, and the argument is written out on the type. Nothing is lost here
        // that the caller could use.
        let transaction = self
            .database
            .begin_write()
            .map_err(|_| CustodyError::Unavailable)?;
        {
            let mut table = transaction
                .open_table(PACKAGES)
                .map_err(|_| CustodyError::Unavailable)?;
            table
                .insert(byte_of(key), bytes)
                .map_err(|_| CustodyError::Unavailable)?;
        }
        // ⛔ THE HANDLE IS DROPPED BEFORE THE COMMIT, which the block above is for: `redb`
        // refuses to commit with a table handle still live, and the journal's
        // `abandon_without_commit` carries the twin of this note about `TableAlreadyOpen`.
        transaction
            .commit()
            .map_err(|_| CustodyError::Unavailable)?;
        Ok(())
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        let transaction = self
            .database
            .begin_read()
            .map_err(|_| CustodyError::Unavailable)?;
        let table = transaction
            .open_table(PACKAGES)
            .map_err(|_| CustodyError::Unavailable)?;

        // ⛔ `Ok(None)` FOR A KEY WITH NOTHING UNDER IT, never an error -- promise 3. And the
        // bytes are copied out: the `AccessGuard` borrows the transaction, which dies here.
        Ok(table
            .get(byte_of(key))
            .map_err(|_| CustodyError::Unavailable)?
            .map(|found| found.value().to_vec()))
    }
}
```

E `crates/platform/tests/custody_contract_real.rs`, **LF**:

```rust
// THE SAME CONFORMANCE SUITE, RUN AGAINST THE REAL CUSTODY (§2 of the GUI north star).
//
// ⛔ THE ASSERTIONS ARE NOT REPEATED HERE, and that is the whole point of this file being short.
// They live in ONE place -- `crates/kernel/tests/custody_contract.rs` -- and are reached from
// here textually, because two copies would drift and THE FIRST ONE TO DRIFT WOULD LIE IN
// SILENCE. `include!` is the mechanism because an integration test is A CRATE OF ITS OWN: it
// cannot `use` the items of another test target. The path is relative to this file's directory,
// which is why it climbs out of `crates/platform/tests/`.
//
// ⛔ AND THIS FILE HAS NO `use` OF ITS OWN, which is the mechanism's constraint and not a
// preference: the included file brings `use kernel::ports::custody::{Custody, CustodyError,
// CustodyKey}` along with it, and naming any of those three again here is `E0252`. Everything
// below is spelt out in full for that reason -- `journal_contract_real.rs` has no imports for
// the same one.
//
// ⚠️ DECLARED COST, accepted rather than unnoticed: `include!` brings the included file's
// `#[test]` functions with it, so the suite's tests RUN A SECOND TIME inside this binary. None
// of them touches the disk, so it costs a few milliseconds and it buys the single copy of the
// assertions. ⚠️ HOW MANY IS NOT WRITTEN HERE, and that is deliberate: a figure inside a
// sentence that stays true is gotcha #31, and `grep -c '^#\[test\]'` on the two files answers it
// whenever it is asked. Only `the_real_custody_honours_the_contract` below reaches a file at
// all, which is what makes a red in this binary readable.

include!("../../kernel/tests/custody_contract.rs");

#[test]
fn the_real_custody_honours_the_contract() {
    // ⛔ A FILE OF ITS OWN FOR EVERY CALL OF THE FACTORY. The suite takes a factory precisely
    // because several promises need an archive that has never been written to -- promise 3 asks
    // for `Ok(None)`, promise 5 asks about an EMPTY package -- so a factory handing back the same
    // archive twice would let promise 1's bytes be found by promise 3, and the suite would go red
    // ON THE BENCH instead of on the implementation.
    //
    // ⛔ AND `FileBackend` TAKES AN EXCLUSIVE LOCK. A factory reusing one path would be one
    // refactor away from `OpenError::AlreadyOpen`, which is a failure of the bench wearing the
    // mask of a failure of the port.
    //
    // ⛔ A NAME THAT HAS NEVER EXISTED RATHER THAN A DELETION -- gotcha #52 avoided instead of met
    // again. `remove_file` before each open FAILS SILENTLY on Windows while the file is still
    // open, so the factory would reopen the OLD DATA with nothing saying so.
    //
    // ⚠️ `AtomicU64` AND NOT A PLAIN COUNTER: `assert_custody_contract` takes `F: Fn() -> C` and
    // not `FnMut`, so the closure cannot mutate what it captures.
    let dir = private_dir_for_line(line!());
    let calls = std::sync::atomic::AtomicU64::new(0);

    assert_custody_contract(|| {
        let nth = calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = dir.join(format!("custody-{nth}.redb"));
        platform::custody::FileCustody::open(&path).expect("open")
    });
}

/// A directory of this test's own, emptied on entry -- the same mechanism
/// `crates/platform/tests/file_journal.rs` uses, and for the same reason.
///
/// ⛔ ONE DIRECTORY PER CALL SITE, AND THE LINE NUMBER IS WHAT MAKES IT ONE. `cargo test` runs the
/// test BINARIES in parallel and libtest runs the tests inside one binary on parallel threads, so
/// a directory emptied on entry is only safe if nothing else can be inside it. Two call sites
/// cannot share a line number, so the directories are distinct BY CONSTRUCTION.
///
/// ⛔ AND THE PREFIX IS UNIQUE TO THIS FILE, which is the half a line number alone does not cover:
/// line 40 of this file and line 40 of `file_journal.rs`, `journal_contract_real.rs` or
/// `file_custody.rs` would name the SAME directory, and those binaries run at the same time. The
/// four prefixes in use are `daemon-file-journal-`, `daemon-journal-contract-`,
/// `daemon-custody-contract-` and `daemon-file-custody-`.
fn private_dir_for_line(line: u32) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("daemon-custody-contract-{line}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    dir
}
```

In `crates/platform/src/lib.rs` (**`i/lf w/crlf`**), accanto agli altri moduli:

```
pub mod custody;
```

```bash
cargo test --locked -p platform --test custody_contract_real 2>&1 | tail -14
```

Atteso: **tutti verdi**, e sono quelli della suite **più uno** — il conteggio lo dà
`grep -c '^#\[test\]'` sui due file, non questa riga.

- [ ] **Passo 6: ciò che SOLO la vera promette**

`crates/platform/tests/file_custody.rs`, **LF**:

```rust
// WHAT ONLY THE REAL CUSTODY PROMISES: that a package survives the process, and that two
// handles on one file are refused. Neither is askable of the in-memory double, which is CORRECT
// not to make them -- asserting them in the conformance suite would turn a correct
// implementation red (gotcha #44).
//
// ⛔ WHAT IS NOT HERE, AND IT WAS EXAMINED RATHER THAN FORGOTTEN: a probe on the file's
// permission bits. `file_journal.rs` has one because ADR-0023 promises the JOURNAL is "protected
// as much as your system account is", and 0644 would be less. This archive is the
// "configuration" one of ADR-0022 -- NOT encrypted, in the backup -- so no ADR promises anything
// about its mode, and a probe without a source is a probe nobody can read. It gets 0600 anyway,
// because `FileBackend::open` sets it for every archive, and THAT is already held by
// `the_journal_file_is_not_world_readable`. Written down so the next census finds this absence
// explained instead of missing.

use std::path::Path;

use kernel::ports::custody::{Custody, CustodyKey};
use platform::custody::FileCustody;
use platform::journal::OpenError;

/// A directory of this test's own, emptied on entry. Same mechanism and same reasoning as
/// `file_journal.rs`; ⛔ the PREFIX differs from every other bench's, because a line number alone
/// does not keep two files apart.
fn private_dir_for_line(line: u32) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("daemon-file-custody-{line}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    dir
}

fn archive_in(dir: &Path) -> std::path::PathBuf {
    dir.join("custody.redb")
}

#[test]
fn what_was_kept_survives_reopening_the_file() {
    // §2 of the north star, piece 3: "apri, scrivi, riapri, rileggi". It is the promise the
    // whole port exists for -- the layout has to outlive the window that drew it.
    //
    // ⛔ AND THE BYTES ARE NOT TEXT, on purpose: this way the probe holds the OPACITY across a
    // reopening too, which is the one place a format-aware implementation would still be free to
    // "tidy" the package -- on the way in, or on the way back out.
    let dir = private_dir_for_line(line!());
    let path = archive_in(&dir);
    let written: &[u8] = &[0xff, b'{', 0x00, 0x9c, b'}'];

    {
        let mut custody = FileCustody::open(&path).expect("open");
        custody
            .keep(CustodyKey::Layout, written)
            .expect("keep must succeed");
    }

    let reopened = FileCustody::open(&path).expect("reopen");
    assert_eq!(
        reopened.retrieve(CustodyKey::Layout),
        Ok(Some(written.to_vec())),
        "a package kept before the handle was dropped must be there after it is opened again"
    );
}

#[test]
fn a_second_custody_on_an_open_file_is_refused() {
    // ⛔ THE LOCK IS NOT AN EXTRA: two writers on one archive is the corruption the refusal
    // exists to prevent, and `redb`'s own backend takes one, so a replacement that did not would
    // drop a guarantee IN SILENCE.
    let dir = private_dir_for_line(line!());
    let path = archive_in(&dir);

    let _first = FileCustody::open(&path).expect("the first open must succeed");

    match FileCustody::open(&path) {
        Err(OpenError::AlreadyOpen) => {}
        Err(other) => panic!("refused, but for the wrong reason: {other:?}"),
        Ok(_) => panic!("a second custody on an open file must be refused"),
    }
}

#[test]
fn a_second_custody_on_a_closed_file_is_not_refused() {
    // ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3). Without it, an `open` that refused ALWAYS
    // would satisfy the test above -- and the port would be unusable for the reason it exists,
    // which is being reopened at every start-up.
    let dir = private_dir_for_line(line!());
    let path = archive_in(&dir);

    {
        let _first = FileCustody::open(&path).expect("the first open must succeed");
    }

    FileCustody::open(&path).expect("a custody on a closed file must open");
}
```

```bash
cargo test --locked -p platform --test file_custody 2>&1 | tail -8
```

Atteso: **tre verdi**.

- [ ] **Passo 7: ⛔ LA MISURA DEL LIMITE — la mutazione cieca alla chiave PASSA**

**P-29 in persona, e questo passo è la metà senza la quale il limite è un'ipotesi.** Si applica **una** mutazione,
si lancia, si legge l'esito, si **revoca**.

In `crates/platform/src/custody.rs`, `byte_of` diventa cieca:

```rust
fn byte_of(_key: CustodyKey) -> u8 {
    KEY_LAYOUT
}
```

⚠️ **Non è una mutazione osservabile con una chiave sola** — è precisamente il punto. La mutazione che si vuole
provare è quella su `retrieve`, che **ignora l'argomento**:

```rust
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        // MUTATION, TO BE REVOKED: the key is never consulted; the first package in the table
        // comes back whatever was asked for.
        let transaction = self
            .database
            .begin_read()
            .map_err(|_| CustodyError::Unavailable)?;
        let table = transaction
            .open_table(PACKAGES)
            .map_err(|_| CustodyError::Unavailable)?;
        let mut rows = table.iter().map_err(|_| CustodyError::Unavailable)?;
        match rows.next() {
            Some(Ok((_, value))) => Ok(Some(value.value().to_vec())),
            Some(Err(_)) => Err(CustodyError::Unavailable),
            None => Ok(None),
        }
    }
```

```bash
cargo test --locked -p platform --test custody_contract_real 2>&1 | tail -8
git diff --stat
git checkout -- crates/platform/src/custody.rs
git diff --stat
```

⛔ **Atteso: TUTTI VERDI.** È il risultato che il passo cerca — la suite **non** coglie la cecità alla chiave — e
va scritto **nel messaggio del commit** con le parole della corsa, non riassunto. Se invece uscisse **rosso**, il
limite di **P-29** è più stretto di come è scritto e la testa di `custody_contract.rs` va **corretta**: sarebbe una
voce d'errata, non un sollievo.
⚠️ Il secondo `git diff --stat` deve essere **vuoto**: la mutazione non si committa.

- [ ] **Passo 8: il richiamo datato di P-24 su `platform/src/lib.rs`**

Nel doc di modulo di `crates/platform/src/lib.rs` (**`i/lf w/crlf`**), **sotto** il richiamo che il compito 2 ha già
scritto e senza cancellarlo:

```rust
//! ⛔ DATED RECALL, 2026-09-11 -- THE SENTENCE ABOVE CALLED ITSELF "not a fixed set" AND IT WAS
//! ONE. The `grep` it hands over ENUMERATES SEVEN TRAIT NAMES, so it can never answer with a
//! family added later: `Custody` -- the seventh port, §2 of the GUI north star -- was invisible
//! to it the moment `custody::FileCustody` existed. Measured, not reasoned. ⛔ AND THE CURE IS
//! NOT AN OPEN REGEX, which was measured too: `^impl [A-Za-z_]+ for ` catches `StorageBackend for
//! FileBackend` and `Default for SequentialRng`, neither of which is a port -- noise mistaken for
//! coverage, the opposite error and just as silent. So the list IS an enumeration, it is now said
//! to be one, and WHOEVER ADDS A PORT ADDS ITS NAME HERE, because nobody else can know.
//! ⚠️ AND THE THIRD FALSEHOOD IN THE SAME SENTENCE WAS A DEADLINE IN PROSE (gotcha #77):
//! "because milestone 6 adds to it". Milestone 6 CLOSED on 2026-09-02 and added nothing. It is
//! replaced by a fact instead of by another deadline.
//!
//! ⛔ The list comes from
//! `grep -rEn "^impl (Custody|Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for " crates/platform/src/`,
//! WHICH IS AN ENUMERATION OF THE PORT TRAITS AND NOT A DISCOVERY.
```

⚠️ **Non si riscrive la frase originale**: si lascia dov'è e la si smentisce sotto, con la data — la regola dei
richiami di `CLAUDE.md`, e la stessa forma che `ports/mod.rs` porta dal 2026-08-28.

```bash
grep -rEn "^impl (Custody|Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for " crates/platform/src/
```

Atteso: **una riga in più** del Passo 1, `impl Custody for FileCustody`.

- [ ] **Passo 9: i fine-riga, il cancello, il commit**

```bash
for f in crates/simulator/src/custody.rs crates/kernel/tests/custody_contract.rs crates/platform/src/custody.rs crates/platform/tests/custody_contract_real.rs crates/platform/tests/file_custody.rs crates/platform/src/lib.rs crates/simulator/src/lib.rs crates/platform/src/journal.rs; do printf '%-52s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
git ls-files --eol crates/platform/src/lib.rs crates/simulator/src/lib.rs crates/platform/src/journal.rs
bash scripts/gate.sh 2>&1 | tail -3
bash scripts/gate-deps.sh 2>&1 | tail -3
bash scripts/check-docs.sh 2>&1 | tail -3
```

Atteso: i **cinque file nuovi** a `CR=0`; i **tre modificati** `i/lf w/crlf` come al Passo 1 e col loro numero di
CR **invariato**; `GATE GREEN`; la lista di ADR-0031 **non cresciuta**; `OK — no inconsistencies.`

Poi la riga **5** della tabella della posizione a ✅ con la data, e il commit — **senza co-autore**:

```bash
git add crates/simulator/src/custody.rs crates/simulator/src/lib.rs crates/kernel/tests/custody_contract.rs crates/platform/src/custody.rs crates/platform/src/lib.rs crates/platform/src/journal.rs crates/platform/tests/custody_contract_real.rs crates/platform/tests/file_custody.rs docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md
```

⛔ **Nel messaggio del commit va la misura del Passo 7 con le parole della corsa**, perché è l'unico posto in cui
un lettore futuro può verificare che il limite di **P-29** fu misurato e non supposto.

#### Criterio di chiusura del compito 5

- [ ] `cargo test --locked -p kernel --test custody_contract` → tutti passati
- [ ] `cargo test --locked -p platform --test custody_contract_real` → tutti passati, **uno in più** del banco del kernel
- [ ] `cargo test --locked -p platform --test file_custody` → **tre** passati
- [ ] ⛔ i **cinque bugiardi** verdi, e **nessuno** ha detto `THE SUITE IS VACUOUS` né `fired, but NOT on promise N`
- [ ] ⛔ il **Passo 7 eseguito**: la mutazione cieca alla chiave provata, **passata**, revocata, `git diff --stat` **vuoto**, e l'esito **nel messaggio del commit**
- [ ] `grep -c 'DATED RECALL, 2026-09-11' crates/platform/src/lib.rs` → **almeno 1**, e il richiamo del compito 2 è **ancora lì**
- [ ] `grep -rEn "^impl (Custody|Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for " crates/platform/src/` → include `impl Custody for FileCustody`
- [ ] `grep -c 'pub(crate) fn engine' crates/platform/src/journal.rs` → **1**, e nessun altro tocco a quel file: `git diff --stat -- crates/platform/src/journal.rs` mostra **una sola** regione
- [ ] ⛔ **nessun `with_backend` su `FileCustody`**: `grep -c 'with_backend' crates/platform/src/custody.rs` → **0**
- [ ] `git diff --name-only -- docs/superpowers/specs/` **vuoto**: questo compito non tocca nessuna spec
- [ ] `bash scripts/gate.sh` → `GATE GREEN`; `bash scripts/gate-deps.sh` verde, la lista **non cresciuta**; `bash scripts/gate-attributes.sh` verde
- [ ] i fine-riga rimisurati: i cinque nuovi a zero CR, i tre modificati invariati in `git ls-files --eol`
- [ ] la riga **5** della tabella della posizione a ✅ con la data

---

## Compito 6: il registro delle funzioni — `kernel::registry`, la specie `Invocation`, e il settimo record congelato

**Files:**
- Modify: `crates/kernel/src/record.rs` (**`i/lf w/crlf`**) — `RecordKind::Invocation` all'indice **6**, `Detail::Invocation` all'indice **3**, `InvocationDetail`, il costruttore di specie `RecordV1::invocation`
- Modify: `crates/kernel/src/reconcile.rs` (**`i/lf w/crlf`**) — il **settimo braccio vuoto**, col proprio argomento **misurato** (**P-33**)
- Modify: `crates/kernel/tests/reconciliation.rs` (**`i/lf w/crlf`**) — le **due** sonde del braccio nuovo, nelle due direzioni
- Create: `crates/kernel/src/registry.rs` (**LF**) — `Invoker`, `Function`, `InvokeError`, `Registry`
- Modify: `crates/kernel/src/lib.rs` (**`i/lf w/crlf`**) — `pub mod registry;`
- Create: `crates/kernel/tests/registry.rs` (**LF**) — le sonde del registro
- Create: `crates/kernel/tests/frozen/record_v1_invocation.cbor` — ⛔ **il settimo record congelato, TIPATO A MANO**
- Modify: `crates/kernel/tests/frozen/record_v1.map` — la sezione nuova, **tipata a mano**
- Modify: `crates/kernel/tests/frozen_bytes.rs` (**`i/lf w/crlf`**) — `the_frozen_records()` da **sei a sette**, i `..._BYTES` nuovi, e il `match kind` di riga 386 (**P-33**)
- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**CRLF**) — **due** richiami datati sulla §5: la riga dell'argomento (**P-30**) e quella dell'invocatore (**P-32**)
- Read: la **§5 del 2**, la prima tabella, per intero; **ADR-0038**, la *Decision* e il perimetro negativo; `crates/kernel/src/permission.rs` **per intero** (250 righe: `Operation::is_write`, `Permission`, `PermissionError`, `grant`, `is_granted`); `crates/kernel/src/record.rs` — `RecordKind`, `EffectClass`, `Trust`, `Detail`, `PermissionDetail` **e i suoi due richiami**, `RecordV1::permission`, `RecordV1::of`; la **testa** di `crates/kernel/tests/frozen_bytes.rs` (i tre divieti) e `the_frozen_records()`; la **testa** di `crates/kernel/tests/frozen/record_v1.map` (che cosa è controllato e che cosa è prosa); ⚠️ **più `Triple`, `Access`, `Call` e `PolicyName` del compito 3**, che sono i gemelli sul filo di ciò che il registro decide — il blocco *Interfaces* del 3 li porta

**Interfaces:**
- Consumes: `kernel::permission::{self, Permission, PermissionError, Operation}`; `kernel::ports::ipc::ClientId`; `kernel::ports::journal::{Journal, JournalError, StepId}`; `kernel::record::{EffectClass, Record, RecordV1, Trust}`
- Consumes, dal compito 3 e **solo per leggerli**: `kernel::wire::ipc::{Call, Triple, Access}` — ⛔ **il registro NON li nomina**: li traduce il dispaccio, compito 7
- Produces, e i compiti **7**, **8** e **11** li usano con questi nomi esatti:
  - `kernel::registry::Registry`, con `Registry::new() -> Registry` (**`const fn`**), `Registry::register(&mut self, Function)`, `Registry::held(&self, name: &str) -> Option<Function>` e `Registry::invoke(journal, step, name, invoker, argument, approval, effect)`
  - `kernel::registry::Approval` — `Checked` e `JustGiven`. ⛔ **Arrivato col richiamo del 2026-09-11 (P-43, D24):** senza di esso la via dell'`Approve` non esiste, perché il controllo `is_granted` risponde no una seconda volta
  - `kernel::registry::Function` — `{ name: &'static str, permission: Permission, effect: EffectClass }`, `Copy`
  - `kernel::registry::Invoker` — oggi una variante, `Invoker::Gui(ClientId)`, con `Invoker::code(self) -> u8`
  - `kernel::registry::InvokeError` — `NotRegistered`, `PermissionRequired(Permission)`, `Permission(PermissionError)`, `Journal(JournalError)`
  - `kernel::record::{RecordKind::Invocation, Detail::Invocation, InvocationDetail}`, con `InvocationDetail::new(function: &'static str, invoker: u8)`, `function()` e `invoker()`
  - `RecordV1::invocation(effect, trust, payload, reason, detail) -> RecordV1`

⛔ **LA FUNZIONE REGISTRATA NON NASCE QUI.** Il registro è il **meccanismo**; *quali* funzioni esistano lo porta chi
le usa (regola 1 di ADR-0038). Il cambio di policy VRAM — nome, tripla «registro × arbitro × scrittura», classe
`Idempotent` — lo **registra il dispaccio**, compito 7, ed è lì che `Arbiter::set_policy` viene nominato. ⚠️ Le
sonde di questo compito registrano **funzioni di comodo** e un effetto che scrive nel giornale: provano il
meccanismo, non il contenuto.

- [ ] **Passo 1: le misure prima**

```bash
ls crates/kernel/src/registry.rs crates/kernel/tests/registry.rs 2>&1
grep -c '^#\[n(' crates/kernel/src/record.rs
grep -n 'RecordKind::Permission' crates/kernel/src/record.rs crates/kernel/src/reconcile.rs crates/kernel/tests/frozen_bytes.rs
ls crates/kernel/tests/frozen/
grep -n 'fn the_frozen_records' crates/kernel/tests/frozen_bytes.rs
grep -rn "RecordKind::" crates/kernel/src/ --include=*.rs | grep -v 'src/record.rs' | grep -c .
git ls-files --eol crates/kernel/src/record.rs crates/kernel/src/reconcile.rs crates/kernel/src/lib.rs crates/kernel/tests/frozen_bytes.rs crates/kernel/tests/reconciliation.rs crates/kernel/tests/frozen/record_v1.map docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
```

Atteso: i due file **non esistono**; `RecordKind` arriva a `Permission` con l'indice **5** e `Detail` a
`Permission` con il **2**; in `frozen/` **sei** `.cbor` più la mappa; `the_frozen_records()` rende un array di
**sei**; i due `match` esaustivi di **P-33** sono a `reconcile.rs:90` e `frozen_bytes.rs:386` — ⛔ **si ritrovano
col `grep` sulla frase e non col numero di riga** (gotcha #70); tutti i file da toccare `i/lf w/crlf`, la mappa
compresa.

- [ ] **Passo 2: la specie nuova nel record**

In `crates/kernel/src/record.rs`, **in coda a `RecordKind`**:

```rust
    /// ⛔ UN'INVOCAZIONE DEL REGISTRO (ADR-0038). Like the four before it, it neither opens a
    /// doubt nor closes one: the note says WHO asked for WHAT, and the step it names still owes
    /// its own outcome — the invocation's own `intent` and `outcome` carry that, and they are
    /// `Intent` and `Outcome` like anybody's. ⚠️ AND THE EMPTY ARM IN `reconcile` WAS MEASURED FOR
    /// THIS VARIANT rather than inherited from the four above it: see the arm itself.
    #[n(6)]
    Invocation,
```

**in coda a `Detail`**:

```rust
    /// Who invoked which function of the registry (ADR-0038).
    #[n(3)]
    Invocation(#[n(0)] InvocationDetail),
```

e il tipo, **accanto a `PermissionDetail`**:

```rust
/// The structured half of an invocation (ADR-0038): WHAT was invoked, and BY WHOM.
///
/// ⛔ THE ARGUMENT IS NOT HERE, AND IT IS THE SAME DECISION `VerdictDetail` TOOK FOR ITS DETAIL
/// TEXT — read that type, the argument is one. The argument is text the GUI CHOSE, so it is
/// untrusted by inheritance (ADR-0014) and travels in the record's `payload`, under the `trust`
/// label that exists to say so. What lives here is what is OURS and structured: a name this
/// crate registered, and a code this crate assigned. ⚠️ SO THE §5 OF THE MILESTONE-2 DESIGN SAYS
/// "funzione, invocatore, argomento" AND THE THIRD IS BESIDE THIS TYPE RATHER THAN IN IT; the
/// dated recall is on that line.
///
/// ⛔ THE TWO FIELDS HAVE DIFFERENT TYPES, AND THAT IS LOAD-BEARING RATHER THAN INCIDENTAL. The
/// recall of 2026-09-01 beside the frozen `Permission` record measured that TWO EQUAL STRINGS at
/// two offsets pin ONE offset and its mirror image — exchanging the two `#[n(..)]` moved no byte
/// and the workspace stayed green. A text and a number cannot mirror each other, so the frozen
/// record here really does pin both indices. Written down because the NEXT two-field detail may
/// not be so lucky.
///
/// ⚠️ THE QUALIFIER `RoutingDetail` AND `PermissionDetail` BOTH CARRY APPLIES HERE WORD FOR WORD:
/// this type derives `Decode` and `Record::decode` is `pub`, so BYTES build one without passing
/// through `new`. That is road A4 of `crate::boundary`. What `new` shuts is every road a caller
/// can WRITE IN SOURCE.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub struct InvocationDetail {
    #[n(0)]
    function: String,
    /// ⛔ A `u8` AND NOT THE `registry::Invoker` ENUM, AND IT IS `PermissionDetail`'s ARGUMENT ONE
    /// SIZE UP. An enum here would be a FOURTH `index_only` enum ON THE WIRE, whose variant
    /// indices `tests/frozen_bytes.rs` would then have to pin ONE PER FROZEN RECORD, and an index
    /// on the wire never retires (rule 4 of §4.9.2). That type could use a `bool` because it had
    /// two values; this one has four coming — click, gesture, voice, agent — so a `bool` cannot
    /// serve and a `u8` is the same trade at the next size.
    ///
    /// ⛔ AND THE CODE IS ASSIGNED BY AN EXHAUSTIVE `match` IN `registry::Invoker::code`, never by
    /// `as u8`: a cast would number a new variant BY POSITION, and a reordering would silently
    /// repoint every record already written. Read that function; it is `Operation::is_write`'s
    /// lesson at four values instead of two.
    #[n(1)]
    invoker: u8,
}

impl InvocationDetail {
    /// The ONLY way to build one in source. ⛔ `function` IS `&'static str` FOR THE REASON
    /// `PermissionDetail::new` GIVES: the name that ARRIVES is compared against the registered
    /// ones and dropped — it never becomes the kernel's own vocabulary (I6, ADR-0014). What
    /// reaches here is the REGISTERED name.
    pub fn new(function: &'static str, invoker: u8) -> Self {
        Self {
            function: String::from(function),
            invoker,
        }
    }

    /// The function that was invoked, as it was named THEN.
    pub fn function(&self) -> &str {
        &self.function
    }

    /// The code of whoever invoked it. ⚠️ A `u8` COMES BACK AND NOT AN `Invoker`, and there is
    /// deliberately NO `from_code`: nothing decides on the invoker today — the `Steps` list shows
    /// the function and its outcome — so a reverse conversion would buy a failing branch no
    /// caller has, which is the rule this crate applies to `simulator`'s `EntryKind` and to
    /// `platform`'s three stored kinds. The FIRST consumer that BRANCHES on who invoked writes
    /// it, with its probe.
    pub fn invoker(&self) -> u8 {
        self.invoker
    }
}
```

e il **costruttore di specie**, accanto a `permission`:

```rust
    /// AN INVOCATION OF THE REGISTRY (ADR-0038). ⛔ ITS DETAIL IS NOT OPTIONAL EITHER, for the
    /// reason the three species before it give: a record that says an invocation happened and
    /// names neither the function nor the invoker would be a record that records nothing while
    /// claiming to be a record of something.
    pub fn invocation(
        effect: EffectClass,
        trust: Trust,
        payload: Vec<u8>,
        reason: &'static str,
        detail: InvocationDetail,
    ) -> Self {
        Self::of(
            RecordKind::Invocation,
            effect,
            trust,
            payload,
            reason,
            Some(Detail::Invocation(detail)),
        )
    }
```

```bash
cargo build --locked -p kernel 2>&1 | grep -E "^error|E0004" | head
```

Atteso: **`error[E0004]`** su `crates/kernel/src/reconcile.rs` — il `match` esaustivo. ⛔ **È il passo, non un
intoppo:** il compilatore sta chiedendo la decisione che il Passo 3 prende, ed è §7.4.4 che funziona.

- [ ] **Passo 3: il settimo braccio della riconciliazione, MISURATO**

In `crates/kernel/src/reconcile.rs`, dopo il braccio di `Permission`:

```rust
                // ⛔ AN INVOCATION RECORD NEITHER OPENS A DOUBT NOR CLOSES ONE, and it was
                // MEASURED for THIS variant rather than inherited from the four above it. An
                // invocation note says WHO asked for WHAT; the doubt of ADR-0007 is about an
                // EFFECT that may or may not have reached the world, and asking is not an effect.
                // The step it names owes its own outcome, and writes one.
                //
                // ⛔ BOTH OTHER ANSWERS WERE TRIED BEFORE THIS ARM WAS WRITTEN, which is what
                // "measured" means here: `enter` would leave EVERY invoked step in doubt for
                // ever, because the note arrives after the step's own `intent` and a second
                // `enter` on an open step is not what `leave` undoes; and `leave` would CLOSE the
                // doubt the invocation's own `intent` opened, so a crash between the note and the
                // effect would reconcile as "finished" — the silent loss of a real doubt, the one
                // failure ADR-0007 exists to prevent. ⚠️ AND THAT SECOND ONE IS WORSE HERE THAN
                // IT WAS FOR `Routing`: the note sits between the intent and the effect BY
                // DESIGN (§5 of the milestone-2 design), so the window it would swallow is not
                // hypothetical — it is the ordinary shape of every invocation.
                //
                // Held in BOTH directions (§7.1.1 rule 3) by
                // `an_invocation_note_does_not_put_a_step_in_doubt` and
                // `an_invocation_note_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them`
                // in `tests/reconciliation.rs`, exactly as the `Note`, `Verdict`, `Routing` and
                // `Permission` pairs are.
                RecordKind::Invocation => {}
```

⛔ **E le due misure si FANNO, non si citano.** Prima di scrivere il braccio vuoto si prova `enter(&mut open,
step, resolution_of(body.effect()))` al suo posto, poi `leave(&mut open, step)`, una per volta, si lancia
`cargo test --locked -p kernel --test reconciliation`, si legge **quale** sonda diventa rossa, e si revoca.
Se una delle due lascia tutto verde, la sonda corrispondente del Passo 4 **non esiste ancora** o è vacua.

- [ ] **Passo 4: le due sonde della riconciliazione, nelle due direzioni**

In coda a `crates/kernel/tests/reconciliation.rs`, sul modello della coppia di `Permission` che è già lì:

```rust
#[test]
fn an_invocation_note_does_not_put_a_step_in_doubt() {
    // ⛔ THE FIRST DIRECTION: the arm must not `enter`. A step whose intent and outcome are both
    // written is CLOSED, and an invocation note landing between them must not reopen it.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(3);

    journal
        .intent(step, &closed_record(RecordV1::intent))
        .expect("intent");
    journal
        .note(step, &an_invocation_note())
        .expect("the invocation note");
    journal
        .outcome(step, &closed_record(RecordV1::outcome))
        .expect("outcome");

    assert_eq!(
        steps_in_doubt(&journal).expect("the projection must answer"),
        Vec::new(),
        "an invocation note upon a step that closed must leave nothing in doubt"
    );
}

#[test]
fn an_invocation_note_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them() {
    // ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3): the arm must not `leave` either. Here the
    // step has an intent and NO outcome — the crash between the note and the effect — so the
    // doubt must survive the note WITH ITS RESOLUTION UNCHANGED. Without this probe a `leave` in
    // the arm would pass the test above and swallow every real doubt.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(4);

    journal
        .intent(step, &closed_record(RecordV1::intent))
        .expect("intent");

    let before = steps_in_doubt(&journal).expect("the projection must answer");

    journal
        .note(step, &an_invocation_note())
        .expect("the invocation note");

    let after = steps_in_doubt(&journal).expect("the projection must answer");

    assert_eq!(
        before, after,
        "an invocation note must change neither the doubt nor its resolution"
    );
    assert!(
        !after.is_empty(),
        "the bench is vacuous if the step was never in doubt to begin with"
    );
}

/// An invocation note of no importance, so the two probes above say what they mean.
///
/// ⛔ THE LAST ASSERTION OF THE SECOND PROBE IS NOT DECORATION: `before == after` is satisfied by
/// two empty lists, so without it a bench that never opened a doubt would pass while proving
/// nothing. It is the non-vacuity guard of §8.6.2 written at the size of one test.
fn an_invocation_note() -> Vec<u8> {
    Record::V1(RecordV1::invocation(
        EffectClass::Idempotent,
        Trust::Untrusted,
        Vec::from(&b"an argument the gui chose"[..]),
        "who invoked what",
        InvocationDetail::new("a function", 0),
    ))
    .encode()
}
```

⚠️ **`closed_record` e gli `use` esistono già in quel file** — si riusano, non si riscrivono; il Passo 1 li ha
letti. Se il nome fosse diverso, vale ciò che il file ha **oggi**, e la divergenza è una voce d'errata.

```bash
cargo test --locked -p kernel --test reconciliation 2>&1 | tail -8
```

Atteso: **tutte verdi**, due in più del Passo 1.

- [ ] **Passo 5: il registro**

`crates/kernel/src/registry.rs`, **LF**:

```rust
//! The registry of the program's functions (ADR-0038): ONE registry, MANY invokers, ONE
//! permission.
//!
//! ⛔ THE MECHANISM AND NOT THE CONTENT, which is rule 1 of ADR-0038 and the shape ADR-0009 gives
//! every kernel registry. This module knows how to HOLD a function, how to check its triple and
//! how to JOURNAL an invocation; WHICH functions exist is brought by the gui and by the
//! capabilities. ⛔ AND THE RULE IS HELD AT LEVEL 1 RATHER THAN BY GOOD INTENTIONS: nothing here
//! names `Arbiter`, `VramPolicy` or any other effect, and it could not without an `import` a
//! reader would see. The day the gesture arrives (sub-project 12) it registers ITS functions from
//! outside, exactly as the click does — which is what "no gesture-only logic" means when it stops
//! being a sentence.
//!
//! ⛔ SO `invoke` TAKES THE EFFECT AS A CLOSURE. That is the only shape in which the mechanism can
//! run something it does not know, and the closure receives the journal because the effect writes
//! its own step (§5 of the milestone-2 design: "l'effetto, che è il passo B di `set_policy`
//! com'è").
//!
//! ⛔ AND THE ARRIVING NAME NEVER BECOMES A `&'static str`. It is COMPARED against the registered
//! names and dropped — the discipline `permission::is_granted` already writes out for the triple:
//! a name from outside sitting in a type the kernel decides with is untrusted text inside a
//! decision (ADR-0014). What reaches the journal is the REGISTERED name, which is ours.
//!
//! ⚠️ WHAT THIS MODULE IS NOT, so the next reader does not go looking: it is not a second
//! permission system (ADR-0038, negative perimeter) — it asks `permission::is_granted` and
//! nothing else; and it does not hold the SESSION boundary of ADR-0016, because
//! `permission::is_granted` re-reads the whole journal and a granted triple therefore survives a
//! restart. That limit is declared in §5 of the milestone-2 design and belongs to whoever brings
//! the runs, sub-project 3.

use alloc::vec::Vec;

use crate::permission::{self, Permission, PermissionError};
use crate::ports::ipc::ClientId;
use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, InvocationDetail, Record, RecordV1, Trust};

/// Who asked.
///
/// ⛔ ONE VARIANT TODAY, AND THE OTHERS ARE NAMED IN ADR-0038 RATHER THAN GUESSED AT: gesture
/// (sub-project 12), voice (8), and the agent. Each arrives WITH its invoker, and none of them
/// touches this file's `invoke`.
///
/// ⚠️ IT CARRIES THE `ClientId` AND THE RECORD DOES NOT, which is deliberate and is worth the
/// line. The identifier is what the DISPATCH needs — it is how the answer finds its way back to
/// the client that asked — while the record keeps the CLASS of invoker and not the connection: a
/// `ClientId` is a handle on a socket that reconnecting changes, so journalling it would durably
/// record something that means nothing an hour later. The trigger for recording it is a consumer
/// that needs to tell two simultaneous invokers apart, and ADR-0004 says there is at most one gui.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Invoker {
    /// The gui's client, over the `ipc` port.
    Gui(ClientId),
}

impl Invoker {
    /// How the invoker reaches the durable record.
    ///
    /// ⛔ AN EXHAUSTIVE `match` AND NOT `as u8`, AND THAT IS THE ONLY REASON THIS FUNCTION EXISTS
    /// — `Operation::is_write`'s lesson at four values instead of two. A cast numbers a variant
    /// BY POSITION, so inserting `Gesture` before `Gui` would silently repoint every record
    /// already written, and the archive is the one thing that cannot be migrated cheaply. A
    /// `match` makes that day `error[E0004]`, here, where the decision belongs.
    ///
    /// ⚠️ AND THE CODES ARE WRITTEN OUT rather than derived: a new invoker takes THE NEXT FREE
    /// NUMBER and never one that has been used, which is rule 4 of §4.9.2 applied to a value
    /// instead of an index — the same promise, one level down.
    pub fn code(self) -> u8 {
        match self {
            Invoker::Gui(_) => 0,
        }
    }
}

/// A function of the program, as the registry holds it.
///
/// ⛔ THE NAME IS `&'static str`, LIKE BOTH NAMES OF `Permission` AND FOR THE SAME REASON (I6):
/// this is a type the kernel DECIDES with — `invoke` compares against it — and a name that
/// arrived from outside would be untrusted text sitting inside a decision (ADR-0014).
///
/// ⚠️ `Copy`, so `invoke` can hand the whole thing about without borrowing the registry across
/// the effect. Three small fields; nothing here owns anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    /// How it is named on the wire and in the record.
    pub name: &'static str,
    /// The triple of ADR-0016 that protects it — rule 2 of ADR-0038: the same permission
    /// whatever the invoker.
    pub permission: Permission,
    /// How its effect reconciles after a crash (ADR-0007). It is the class the invocation's own
    /// step carries, and it is MANDATORY here for the reason it is mandatory on the record:
    /// "an effect without a declared class" must not be expressible.
    pub effect: EffectClass,
}

/// Whether the registry must ASK about the permission, or has just been told.
///
/// ⛔ AN ENUM AND NOT A `bool`, which is the lesson `crate::permission::Operation::is_write`
/// already recorded and `crate::wire::ipc::Access` repeats: a `bool` folds every future third
/// case into one of the two, in silence.
///
/// ⛔ AND IT EXISTS BECAUSE THE APPROVAL PATH IS OTHERWISE UNREACHABLE (P-43).
/// `permission::grant` writes a NOTE and wants a step somebody else opened -- its own doc --
/// so decision 21 of the north star puts the grant INSIDE step A, between the invocation note
/// and the effect. The check in `invoke` runs BEFORE step A is opened, so on the second pass
/// it would answer "not granted" again and the round would never close. The order belongs to
/// the mechanism that owns it, not to the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approval {
    /// The invocation arrived on its own: the registry asks `is_granted` and refuses if not.
    Checked,
    /// The user has just approved the triple. The registry does not ask, and writes the grant
    /// on step A, between the invocation note and the effect.
    JustGiven,
}

/// What can go wrong invoking.
///
/// ⛔ `NotRegistered` AND `PermissionRequired` ARE NOT THE SAME REFUSAL, and folding them would
/// lose exactly the distinction the gui needs: one is a bug in the caller, the other is a
/// question for the user. The gui turns the second into the confirmation window (§6a) and the
/// first into nothing at all.
///
/// ⛔ AND `PermissionRequired` CARRIES THE TRIPLE, because the gui has to SHOW it: ADR-0016 wants
/// the user to approve `(tool, resource, operation)` and not "something". It is the REGISTERED
/// triple, so all three names are ours.
///
/// ⚠️ `Permission(PermissionError)` IS NOT `PermissionRequired`: the first says the archive would
/// not answer, the second that it answered no. `permission::PermissionError`'s own doc spends
/// three paragraphs on why "unknown" reported as "not granted" is forbidden, and folding them
/// here would undo that at the call site that matters most.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokeError {
    /// No function of that name is registered. ⛔ AND NOTHING IS WRITTEN — not an intent, not a
    /// note. A name nobody registered is not an event of this program, and journalling it would
    /// let a peer fill the durable archive with words it chose.
    NotRegistered,
    /// The triple is not granted. Nothing is written, and the effect is not run.
    PermissionRequired(Permission),
    /// The archive would not say whether it is granted.
    Permission(PermissionError),
    /// A write was refused.
    Journal(JournalError),
}

/// The registry itself.
///
/// ⛔ IT HOLDS STATE AND `permission::is_granted` DOES NOT, and the asymmetry is not an
/// inconsistency. That one is a PROJECTION of the journal, and a projection with state would
/// answer from when it was built instead of from what the archive says. This is a REGISTRATION
/// TABLE — the shape ADR-0009 gives the registries of guides and sensors — and it lives in the
/// core, where I1 says the authoritative state lives.
pub struct Registry {
    functions: Vec<Function>,
}

// ⛔ NO `impl Default` — the same decision, for the same reason, as `MemoryJournal` and
// `SystemReactor`: nothing calls it. The argument is written out once, in
// `crates/platform/src/reactor.rs`.
impl Registry {
    pub const fn new() -> Self {
        Registry {
            functions: Vec::new(),
        }
    }

    /// Registers a function.
    ///
    /// ⛔ A NAME ALREADY REGISTERED REPLACES, AND IT DOES NOT PILE UP. Two entries under one name
    /// would make `invoke` depend on whether the search reads the first or the last, which is a
    /// silent difference in what a permission protects. ⚠️ AND IT IS NOT AN ERROR, because there
    /// is no caller for one: registration happens once at start-up, from source, and a `Result`
    /// nobody can fail would be ceremony. The day registration is driven by data, it grows one.
    pub fn register(&mut self, function: Function) {
        match self
            .functions
            .iter_mut()
            .find(|held| held.name == function.name)
        {
            Some(held) => *held = function,
            None => self.functions.push(function),
        }
    }

    /// The function held under this name, if the registry holds one.
    ///
    /// ⛔ IT EXISTS FOR THE APPROVAL PATH AND HAS A CALLER FROM THE DAY IT IS WRITTEN:
    /// `crate::serving` compares the triple a peer sends back against the triple THE REGISTRY
    /// holds, rather than trusting the strings that came in (ADR-0014, and the doc of
    /// `crate::wire::ipc::Triple` asks that consumer for it by name).
    pub fn held(&self, name: &str) -> Option<Function> {
        self.functions.iter().find(|held| held.name == name).copied()
    }

    /// Invokes the function named `name`, if it is registered and its triple is granted.
    ///
    /// The order is the one §5 of the milestone-2 design fixes, and each line is load-bearing:
    ///
    /// 1. the name is looked up — not registered, nothing is written;
    /// 2. with `Approval::Checked`, the triple is asked of the journal — not granted, nothing
    ///    is written and the effect is NOT run, which is the probe
    ///    `a_triple_that_is_not_granted_never_reaches_the_effect`;
    /// 3. `intent` on step A, carrying the FUNCTION'S class;
    /// 4. a `note` on step A with the structured detail — who invoked what;
    /// 5. with `Approval::JustGiven`, the GRANT on step A — decision 21 of the north star puts
    ///    the permission on the step it UNLOCKS, and `permission::grant` wants a step already
    ///    open;
    /// 6. the EFFECT, which opens and closes its own step B if it has one;
    /// 7. `outcome` on step A.
    ///
    /// ⛔ THE NOTE COMES AFTER THE INTENT AND BEFORE THE EFFECT, and `Journal::note` enforces half
    /// of that by refusing a note on a step with no intent. The other half — before the effect —
    /// is what makes a crash mid-invocation reconstructible: step A is in doubt WITH ITS CLASS,
    /// and the note says what was being attempted.
    ///
    /// ⛔ `argument` IS `&[u8]` AND GOES INTO THE PAYLOAD, NOT INTO THE DETAIL. It is text the
    /// peer chose, so it is untrusted by inheritance (ADR-0014) and travels under the `trust`
    /// label that says so — the shape `VerdictDetail` established for its detail text. Bytes and
    /// not `&str` because the port exchanges bytes and this function has no reason to require
    /// that an argument be text at all.
    ///
    /// ⚠️ THE EFFECT RECEIVES THE JOURNAL, and it has to: `Arbiter::set_policy` writes step B
    /// through it, and lending it twice is not expressible. The cost is that the effect could
    /// write anything at all — this function cannot police it, and says so rather than pretending.
    pub fn invoke<J, T, E>(
        &self,
        journal: &mut J,
        step: StepId,
        name: &str,
        invoker: Invoker,
        argument: &[u8],
        approval: Approval,
        effect: E,
    ) -> Result<T, InvokeError>
    where
        J: Journal,
        E: FnOnce(&mut J) -> Result<T, JournalError>,
    {
        let Some(function) = self.held(name) else {
            return Err(InvokeError::NotRegistered);
        };

        if approval == Approval::Checked
            && !permission::is_granted(journal, &function.permission)
                .map_err(InvokeError::Permission)?
        {
            return Err(InvokeError::PermissionRequired(function.permission));
        }

        journal
            .intent(step, &opened(function))
            .map_err(InvokeError::Journal)?;
        journal
            .note(step, &noted(function, invoker, argument))
            .map_err(InvokeError::Journal)?;

        // ⛔ THE GRANT GOES ON STEP A, BETWEEN THE INVOCATION NOTE AND THE EFFECT -- decision 21
        // of the north star, held here rather than by the caller. The permission settles on the
        // step it UNLOCKS, exactly as `sensor::run_the_ring` settles a verdict on the step it
        // judges.
        if approval == Approval::JustGiven {
            permission::grant(journal, step, &function.permission)
                .map_err(InvokeError::Journal)?;
        }

        let produced = effect(journal).map_err(InvokeError::Journal)?;

        journal
            .outcome(step, &closed(function))
            .map_err(InvokeError::Journal)?;

        Ok(produced)
    }
}

/// Step A's intent. ⚠️ THE PAYLOAD IS EMPTY AND THE ARGUMENT IS NOT HERE: it belongs to the note,
/// with the detail that says what it is an argument TO. An intent that carried it would put the
/// same untrusted bytes in the archive twice.
fn opened(function: Function) -> Vec<u8> {
    Record::V1(RecordV1::intent(
        function.effect,
        Trust::Instruction,
        Vec::new(),
        "a function of the registry was invoked",
    ))
    .encode()
}

/// The note that says who invoked what, with the argument in the payload.
fn noted(function: Function, invoker: Invoker, argument: &[u8]) -> Vec<u8> {
    Record::V1(RecordV1::invocation(
        function.effect,
        // ⛔ `Untrusted` BECAUSE OF THE PAYLOAD AND NOT BECAUSE OF THE DETAIL. The label describes
        // the payload (ADR-0014), and the payload is the argument the peer chose. The detail
        // beside it is ours, which is exactly why it is a detail and not a payload.
        Trust::Untrusted,
        Vec::from(argument),
        "who invoked which function, and with what",
        InvocationDetail::new(function.name, invoker.code()),
    ))
    .encode()
}

/// Step A's outcome.
fn closed(function: Function) -> Vec<u8> {
    Record::V1(RecordV1::outcome(
        function.effect,
        Trust::Instruction,
        Vec::new(),
        "the invocation finished",
    ))
    .encode()
}
```

In `crates/kernel/src/lib.rs` (**`i/lf w/crlf`**), accanto agli altri moduli — l'ordine è **di arrivo**, come il
file mostra:

```
pub mod registry;
```

- [ ] **Passo 6: le sonde del registro**

`crates/kernel/tests/registry.rs`, **LF**. ⛔ **Da FUORI la crate**, che è la terza domanda del pre-controllo di
`CLAUDE.md`: un tratto o un tipo si prova implementabile e chiamabile solo da fuori.

```rust
//! The registry of the program's functions (ADR-0038), exercised FROM OUTSIDE THE CRATE.
//!
//! ⛔ THE FUNCTIONS REGISTERED HERE ARE OF NO CONSEQUENCE, and that is the point rather than a
//! shortcut: this bench proves the MECHANISM. The one real function — the VRAM policy change —
//! is registered by the dispatch, and its probe lives with it.

use kernel::permission::{self, Operation, Permission};
use kernel::ports::ipc::ClientId;
use kernel::ports::journal::{Journal, JournalError, StepId};
use kernel::record::{Detail, EffectClass, Record, RecordKind};
use kernel::registry::{Approval, Function, InvokeError, Invoker, Registry};
use simulator::journal::MemoryJournal;

const GUARDED: Permission = Permission {
    tool: "registry",
    resource: "arbiter",
    operation: Operation::Write,
};

fn a_function() -> Function {
    Function {
        name: "set-policy",
        permission: GUARDED,
        effect: EffectClass::Idempotent,
    }
}

fn a_registry() -> Registry {
    let mut registry = Registry::new();
    registry.register(a_function());
    registry
}

#[test]
fn a_name_that_is_not_registered_is_refused_and_writes_nothing() {
    // §5 of the milestone-2 design, in its own words: "un nome non registrato → rifiutato, nessun
    // record". ⛔ AND THE SECOND HALF IS THE ONE THAT MATTERS: a peer that could make the core
    // write a record by naming anything would own the durable archive.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    let outcome = registry.invoke(
        &mut journal,
        StepId::new(1),
        "set-polizy",
        Invoker::Gui(ClientId::new(1)),
        b"local",
        Approval::Checked,
        |_| -> Result<(), JournalError> { panic!("the effect must not run") },
    );

    assert_eq!(outcome, Err(InvokeError::NotRegistered));
    assert!(
        journal.replay().expect("replay").is_empty(),
        "a name nobody registered must leave the archive exactly as it was"
    );
}

#[test]
fn a_triple_that_is_not_granted_never_reaches_the_effect() {
    // The first of the two directions §5 asks for. ⛔ THE EFFECT PANICS, so "never reached" is
    // held by the test failing loudly rather than by an assertion that could be forgotten.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    let outcome = registry.invoke(
        &mut journal,
        StepId::new(1),
        "set-policy",
        Invoker::Gui(ClientId::new(1)),
        b"local",
        Approval::Checked,
        |_| -> Result<(), JournalError> { panic!("the effect must not run without the triple") },
    );

    assert_eq!(outcome, Err(InvokeError::PermissionRequired(GUARDED)));
    assert!(
        journal.replay().expect("replay").is_empty(),
        "a refused invocation must leave the archive exactly as it was"
    );
}

#[test]
fn a_granted_triple_reaches_the_effect_and_the_step_closes() {
    // The direction one forgets (§7.1.1 rule 3). Without it, an `invoke` that refused ALWAYS
    // would satisfy the two probes above.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    // The grant is a note upon a step of its own, which is what `permission::grant` promises.
    journal
        .intent(StepId::new(1), &a_bare_intent())
        .expect("the granting step");
    permission::grant(&mut journal, StepId::new(1), &GUARDED).expect("grant");
    journal
        .outcome(StepId::new(1), &a_bare_outcome())
        .expect("the granting step closes");

    let produced = registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(7)),
            b"local",
            Approval::Checked,
            |journal| {
                // The effect writes its OWN step B, which is what `set_policy` really does.
                journal.intent(StepId::new(3), &a_bare_intent())?;
                journal.outcome(StepId::new(3), &a_bare_outcome())?;
                Ok(42u32)
            },
        )
        .expect("a granted invocation must run");

    assert_eq!(produced, 42, "invoke must hand back what the effect produced");
}

#[test]
fn the_note_carries_the_registered_name_the_invoker_and_the_argument() {
    // §5: "una sonda legge il dettaglio dopo `replay`". ⛔ AND IT READS ALL THREE, because each
    // is a different road: the name proves the REGISTERED one reached the record and not the
    // arriving text, the code proves `Invoker::code` was consulted, and the payload proves the
    // argument did NOT end up in the structured half.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    journal
        .intent(StepId::new(1), &a_bare_intent())
        .expect("the granting step");
    permission::grant(&mut journal, StepId::new(1), &GUARDED).expect("grant");
    journal
        .outcome(StepId::new(1), &a_bare_outcome())
        .expect("the granting step closes");

    registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(7)),
            b"local",
            Approval::Checked,
            |_| -> Result<(), JournalError> { Ok(()) },
        )
        .expect("a granted invocation must run");

    let mut seen = None;
    for (_, bytes) in journal.replay().expect("replay") {
        let Record::V1(body) = Record::decode(&bytes).expect("every record must decode");
        if body.kind() != RecordKind::Invocation {
            continue;
        }
        let Some(Detail::Invocation(detail)) = body.detail() else {
            panic!("an invocation record must carry an invocation detail");
        };
        seen = Some((
            detail.function().to_string(),
            detail.invoker(),
            body.payload().to_vec(),
        ));
    }

    let (function, invoker, payload) = seen.expect("the invocation note must be in the archive");
    assert_eq!(function, "set-policy");
    assert_eq!(invoker, 0, "the gui's code, from `Invoker::code`");
    assert_eq!(payload, b"local".to_vec(), "the argument travels in the payload");
}

#[test]
fn an_approval_just_given_skips_the_question_and_writes_the_grant_on_step_a() {
    // ⛔ THE OTHER ROAD OF SEQUENCE 3, AND `a_triple_that_is_not_granted_never_reaches_the_effect`
    // IS ITS MIRROR: the SAME journal, with the SAME ungranted triple, and the only difference
    // is the word `JustGiven`. Without this probe `Approval` would be an enum whose second
    // variant nothing ever takes -- and the round the gui really walks would be held only by
    // another task's bench.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    let produced = registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(1)),
            b"local",
            Approval::JustGiven,
            |_| -> Result<(), JournalError> { Ok(()) },
        )
        .expect("an approval just given must not be asked about");

    assert_eq!(produced, ());

    let kinds: Vec<RecordKind> = journal
        .replay()
        .expect("replay")
        .iter()
        .map(|(_, bytes)| match Record::decode(bytes).expect("every record must decode") {
            Record::V1(body) => body.kind(),
        })
        .collect();
    // ⛔ THE ORDER IS THE ASSERTION: the grant sits BETWEEN the invocation note and the
    // outcome, which is decision 21 of the north star. A grant written before the intent would
    // be a note on a step nobody opened, which `Journal::note` refuses outright; one written
    // after the effect would leave the effect running on a permission not yet recorded.
    assert_eq!(
        kinds,
        vec![
            RecordKind::Intent,
            RecordKind::Invocation,
            RecordKind::Permission,
            RecordKind::Outcome,
        ],
        "the grant goes on step A, between the note and the outcome"
    );

    // ⛔ AND THE SECOND DIRECTION, without which the probe above would pass over a registry
    // that simply stopped checking: the triple is now REALLY granted, so a later `Checked`
    // invocation goes through where the mirror probe refuses it.
    assert!(
        permission::is_granted(&journal, &GUARDED).expect("is_granted answers"),
        "the grant that was written must be the one `is_granted` reads back"
    );
}

#[test]
fn registering_the_same_name_twice_replaces_rather_than_piling_up() {
    // ⛔ THE PROPERTY THAT KEEPS A PERMISSION MEANING ONE THING. With two entries under one name,
    // which triple protects the function would depend on the order of a search.
    let mut registry = Registry::new();
    registry.register(a_function());
    registry.register(Function {
        effect: EffectClass::Unrepeatable,
        ..a_function()
    });

    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), &a_bare_intent())
        .expect("the granting step");
    permission::grant(&mut journal, StepId::new(1), &GUARDED).expect("grant");
    journal
        .outcome(StepId::new(1), &a_bare_outcome())
        .expect("the granting step closes");

    registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(1)),
            b"",
            Approval::Checked,
            |_| -> Result<(), JournalError> { Ok(()) },
        )
        .expect("the second registration must be the one that answers");

    // The SECOND registration's class is what reached the archive — which is the observable
    // difference between replacing and piling up.
    let classes: Vec<EffectClass> = journal
        .replay()
        .expect("replay")
        .into_iter()
        .filter_map(|(_, bytes)| {
            let Record::V1(body) = Record::decode(&bytes).ok()?;
            (body.kind() == RecordKind::Invocation).then(|| body.effect())
        })
        .collect();

    assert_eq!(classes, vec![EffectClass::Unrepeatable]);
}

fn a_bare_intent() -> Vec<u8> {
    Record::V1(kernel::record::RecordV1::intent(
        EffectClass::Idempotent,
        kernel::record::Trust::Instruction,
        Vec::new(),
        "a step of no consequence",
    ))
    .encode()
}

fn a_bare_outcome() -> Vec<u8> {
    Record::V1(kernel::record::RecordV1::outcome(
        EffectClass::Idempotent,
        kernel::record::Trust::Instruction,
        Vec::new(),
        "and it closed",
    ))
    .encode()
}
```

```bash
cargo test --locked -p kernel --test registry 2>&1 | tail -10
```

Atteso: **cinque verdi**.

- [ ] **Passo 7: il settimo record congelato — la sonda usa e getta, e i byte A MANO**

⛔ **La disciplina è quella che la testa di `frozen_bytes.rs` scrive, e non si aggira:** *«Every `.cbor` file here
was TYPED BY HAND from the hexadecimal output of a throwaway probe, and each probe was deleted in the commit that
added its file»*. Nessun `--bless`, nessuna variabile d'ambiente.

1. Nello **scratchpad** (non nel repository) un banco usa e getta che stampa l'esadecimale:

```rust
#[test]
fn print_it() {
    let bytes = Record::V1(RecordV1::invocation(
        EffectClass::Verifiable,
        Trust::Untrusted,
        Vec::from(&b"frozen"[..]),
        "frozen",
        InvocationDetail::new("frozen", 3),
    ))
    .encode();
    for b in &bytes { print!("{b:02x} "); }
    println!("\nlen = {}", bytes.len());
    panic!("read me");
}
```

⚠️ **`invoker: 3` e non `0`**, e la ragione è quella che il commento del quarto record scrive: `00` è già mezza
tabella — `RecordKind::Intent`, `EffectClass::Verifiable`, `Trust::Instruction` — e un byte che somiglia a troppe
cose rende la mappa più difficile da leggere. ⛔ **`3` non è un codice di `Invoker` che esista**, ed è **voluto**:
questo record congela il **formato** del campo, non un valore del dominio; il campo è un `u8` proprio perché il
codice non è un enum sul filo. Va scritto nella mappa.
⚠️ **`EffectClass::Verifiable` e `Trust::Untrusted`** perché la tabella è disposta per **copertura** delle enum del
filo, non per modellare chi scrive — è la stessa nota che il sesto record porta.

2. Si legge l'esadecimale e si **scrive a mano** `crates/kernel/tests/frozen/record_v1_invocation.cbor`.
La forma attesa, che serve a riconoscere una corsa sbagliata — **le cifre si prendono dalla corsa, non da qui**:

| Offset | Che cosa |
|---|---|
| 0 | `82` — array(2), l'enum di versione |
| 1 | `00` — variante 0 = `Record::V1` |
| 2 | `81` — array(1), il corpo della variante |
| 3 | `86` — array(6), i SEI campi di `RecordV1` |
| 4 | indice 0 · `kind` · **`06`**, `RecordKind::Invocation` — ⛔ **l'indice nuovo, e questo record è l'unico posto che lo tiene** |
| 5 | indice 1 · `effect` · `EffectClass::Verifiable` |
| 6 | indice 2 · `trust` · `Trust::Untrusted` |
| 7 | indice 3 · `payload` · byte string(6) `b"frozen"` |
| 14 | indice 4 · `reason` · text(6) `"frozen"` |
| 21 | indice 5 · `detail` · `Some(Detail::Invocation { function: "frozen", invoker: 3 })` — ⛔ **e qui c'è l'indice 3 di `Detail`, che fino a questo file nulla teneva** |

3. Si aggiunge la **sezione alla mappa**, `crates/kernel/tests/frozen/record_v1.map` (**CRLF**), col formato
`offset | byte esadecimali | prosa` delle sei sezioni che già ci sono; gli offset devono essere **contigui** e i
byte devono **ricostruire il file**, perché `the_map_lists_the_bytes_that_are_really_frozen` li rilegge.

4. Si **cancella la sonda usa e getta** dallo scratchpad.

⛔ **Nel commit va scritto che la sonda è stata cancellata**, come i sei commit prima di questo.

- [ ] **Passo 8: il settimo posto in `frozen_bytes.rs`**

In `crates/kernel/tests/frozen_bytes.rs`: il tipo di ritorno di `the_frozen_records()` passa da `; 6]` a `; 7]`,
la costante `INVOCATION_BYTES` si aggiunge accanto alle altre con
`include_bytes!("frozen/record_v1_invocation.cbor")`, e la voce entra in coda all'array:

```rust
        // ⛔ THE SEVENTH IS THE FOURTH SPECIES THAT CARRIES A `detail`, AND WHAT IT PINS THAT THE
        // OTHER THREE CANNOT IS INDEX 3 OF `Detail` — and, with it, index 6 of `RecordKind`. A
        // wire index never retires (rule 4 of §4.9.2), so until this file both were held by
        // nothing at all.
        //
        // ⛔ AND IT IS THE FIRST FROZEN DETAIL WHOSE TWO FIELDS HAVE DIFFERENT TYPES, which is
        // what saves it from the hole the recall of 2026-09-01 measured on the `Permission`
        // record: two EQUAL STRINGS at two offsets pin one offset and its mirror, and exchanging
        // their `#[n(..)]` moved no byte. A text and a `u8` cannot mirror each other, so this
        // record really does pin both indices — measured by exchanging them, not assumed.
        //
        // ⚠️ `invoker: 3` IS NOT A CODE ANY `Invoker` HAS, and that is deliberate rather than a
        // slip: what this record freezes is the FIELD'S FORMAT, and the field is a `u8` precisely
        // because the codes are not an enum on the wire. `0` would also have been the byte of
        // `RecordKind::Intent`, of `EffectClass::Verifiable` and of `Trust::Instruction`, and a
        // byte that resembles too many things makes the map harder to read — the reason the
        // fourth and fifth records give for their own values.
        (
            "record_v1_invocation.cbor",
            INVOCATION_BYTES,
            record(|p, r| {
                RecordV1::invocation(
                    EffectClass::Verifiable,
                    Trust::Untrusted,
                    p,
                    r,
                    InvocationDetail::new("frozen", 3),
                )
            }),
        ),
```

e il `match kind` che **P-33** ha censito riceve il settimo nome nel braccio.

```bash
cargo test --locked -p kernel --test frozen_bytes 2>&1 | tail -12
git diff --stat -- crates/kernel/tests/frozen/
```

Atteso: **tutte verdi**; e ⛔ **`git diff --stat` sui file congelati mostra SOLO la mappa modificata e il file
nuovo** — i **sei** `.cbor` vecchi non compaiono. Se uno di essi cambiasse, non è un aggiornamento: è un **cambio
di formato**, e va aperta una versione nuova (ADR-0036).

- [ ] **Passo 9: i due richiami datati sulla §5 del disegno del 2**

In `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**CRLF**), **dentro** le due celle
della prima tabella della §5, senza riscrivere ciò che c'è:

Nella cella **«il giornale»**, in coda:

> ⛔ **RICHIAMO DEL 2026-09-11, dal pre-controllo del compito 6 (P-30): il dettaglio strutturato porta DUE cose e
> non tre.** *«funzione, invocatore, argomento»* resta vero dell'invocazione, ma **l'argomento non sta nel
> dettaglio**: è testo che la GUI ha scelto, quindi non fidato per eredità (ADR-0014), e viaggia nel **`payload`
> del record** sotto l'etichetta `trust` che esiste per dirlo — è il precedente che `VerdictDetail` scrive per il
> proprio testo, *«what lives here is what is OURS and structured»*. Il motivo è anche meccanico: ogni
> `*Detail::new` del giornale prende `&'static str` per chiudere la strada del testo calcolato a runtime, e
> l'argomento è esattamente quel testo.

Nella cella **«l'invocatore»**, in coda:

> ⛔ **RICHIAMO DEL 2026-09-11, dal pre-controllo del compito 6 (P-32): la regola citata è quella sbagliata, e il
> costo non è zero.** La regola 3 di §4.9.2 parla di **campi** — *«un campo nuovo è facoltativo e prende un indice
> nuovo»* — e una variante di enum non è un campo. Ciò che vale per le varianti lo scrive la testa di
> `crates/kernel/tests/frozen_bytes.rs`: una variante aggiunta a un enum `index_only` rende **indecodificabile a
> una build vecchia** ogni record che la porta. ✅ **La direzione è sicura** — `Malformed` riconcilia come
> `SuspendAndAsk`, quindi una build vecchia si ferma invece di indovinare — quindi il **merito regge**; è *«senza
> cambio di formato»* che è falso. ⛔ **E ne discende una scelta:** l'invocatore raggiunge il record come **`u8`**
> e non come enum, perché un enum lì sarebbe la **quarta** enum `index_only` sul filo — l'argomento che
> `PermissionDetail` scrive per esteso per rifiutarla. Il codice lo assegna un `match` esaustivo in
> `Invoker::code`, così che una variante nuova sia `error[E0004]` e non una rinumerazione silenziosa.

- [ ] **Passo 10: i fine-riga, il cancello, il commit**

```bash
for f in crates/kernel/src/registry.rs crates/kernel/tests/registry.rs; do printf '%-46s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
git ls-files --eol crates/kernel/src/record.rs crates/kernel/src/reconcile.rs crates/kernel/src/lib.rs crates/kernel/tests/frozen_bytes.rs crates/kernel/tests/reconciliation.rs crates/kernel/tests/frozen/record_v1.map docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
bash scripts/gate.sh 2>&1 | tail -3
bash scripts/gate-deps.sh 2>&1 | tail -3
bash scripts/check-docs.sh 2>&1 | tail -3
```

Atteso: i due file nuovi a `CR=0`; i sette modificati **invariati** in `git ls-files --eol`; `GATE GREEN`; la lista
di ADR-0031 **non cresciuta** — il registro non aggiunge dipendenze; `OK — no inconsistencies.`

Poi la riga **6** della tabella della posizione a ✅ con la data, e il commit — **senza co-autore**.

#### Criterio di chiusura del compito 6

- [ ] `cargo test --locked -p kernel --test registry` → **cinque** passati
- [ ] `cargo test --locked -p kernel --test reconciliation` → **due in più** del Passo 1
- [ ] `cargo test --locked -p kernel --test frozen_bytes` → tutti passati, e `the_frozen_records()` rende **sette**
- [ ] ⛔ `git diff --stat -- crates/kernel/tests/frozen/` mostra **solo** la mappa e il file nuovo: i sei `.cbor` vecchi **identici al byte**
- [ ] ⛔ le **due misure del Passo 3** fatte: `enter` al posto del braccio vuoto → rossa la seconda sonda; `leave` → rossa la prima; entrambe revocate, `git diff --stat` pulito
- [ ] ⛔ la sonda usa e getta del Passo 7 **cancellata**, e il commit lo dice
- [ ] ⛔ **il registro non nomina l'arbitro**: `grep -cE 'Arbiter|VramPolicy|arbiter' crates/kernel/src/registry.rs` → **0** (**D16**)
- [ ] `grep -c 'RICHIAMO DEL 2026-09-11' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **almeno 2**
- [ ] `git diff --name-only -- docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` **vuoto** (vincolo 1: questo compito non tocca le due spec)
- [ ] `bash scripts/gate.sh` → `GATE GREEN`; `gate-deps.sh` verde, la lista **non cresciuta**; `gate-attributes.sh` verde
- [ ] i fine-riga rimisurati: i due nuovi a zero CR, i sette modificati invariati
- [ ] la riga **6** della tabella della posizione a ✅ con la data

---

## Compito 7: l'attività che serve la GUI — `kernel::serving`, il dispaccio, e il ramo `Request` non servito

**Files:**
- Create: `crates/kernel/src/serving.rs` — **LF**
- Create: `crates/kernel/tests/serving.rs` — **LF**
- Modify: `crates/kernel/src/lib.rs` — la riga `pub mod serving;`
- Modify: `crates/kernel/src/parameters.rs` — il campo `gui_tick`
- Modify: `crates/kernel/src/executor.rs` — `nap`, la sospensione pubblica
- Modify: `crates/kernel/src/registry.rs` — **dal compito 6**, `Approval` e `Registry::held` (**P-43**)
- Modify: `crates/kernel/src/wire/ipc.rs` — **dal compito 3**, `allocated` e `done` (**P-38**, **P-39**)
- Modify: i **ventitré** file che chiamano `Parameters::new`, i nove `.stderr` che si muovono compresi
- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` — **tre** richiami datati nella §5
- Modify: **questo piano**, le righe **7** e **8** della tabella della posizione (**D21**)
- Read: la **§5 del 2** per intero; le **tre sequenze** di *«La GUI dentro»* della stella polare; la riga **2** della tabella *Stato* della stella polare; `crates/kernel/src/executor.rs` per intero (`spawn`, `Sleep`, `TaskState`, il giro di `run`); le **tre firme** di `crates/kernel/src/ports/ipc.rs` con i loro doc; `crates/kernel/src/client.rs`; `crates/kernel/src/degradation.rs`; `crates/kernel/src/permission.rs` (`Operation`, `Permission`, `grant`); `crates/kernel/src/arbiter/policy.rs`; il blocco *Interfaces* dei compiti **1**, **2**, **3**, **4**, **5** e **6** per i nomi esatti

**Interfaces:**
- Consumes, dal **compito 1**: `kernel::numbering::Progressive`, con `take(&mut self) -> u64`
- Consumes, dal **compito 3**: `kernel::wire::ipc::{IpcMessage, BuildStamp, build_stamp, Protection, DegradationReport, PolicyReport, PolicyName, Triple, Access, Call, LayoutState, StepSummary}`
- Consumes, dal **compito 4**: `kernel::ports::custody::{Custody, CustodyKey, CustodyError}`
- Consumes, dal **compito 6**: `kernel::registry::{Registry, Function, Invoker, InvokeError}`, e i due pezzi che questo compito gli **aggiunge** al passo 4
- Consumes, da oggi: `kernel::arbiter::{Arbiter, MakeRoom, VramPolicy, RemotePolicy, LocalPolicy}`; `kernel::client::ClientGrants`; `kernel::degradation::degradation_now`; `kernel::executor::Sleep`; `kernel::permission::{Operation, Permission}`; `kernel::ports::{ipc::{Ipc, ClientId, IpcError}, journal::{Journal, StepId}, reactor::Reactor}`; `kernel::record::{Detail, EffectClass, Record, RecordKind}`; `kernel::time::{Millis, Monotonic}`
- Produces, e i compiti **8**, **9** e **11** li usano con questi nomi esatti:
  - `kernel::serving::Core<I: Ipc, J: Journal, C: Custody>`, con `Core::new(ipc: I, journal: J, custody: C, arbiter: Arbiter, steps: Progressive, parameters: Parameters) -> Core<I, J, C>`
  - `Core::journal(&mut self) -> &mut J` · `Core::arbiter(&mut self) -> &mut Arbiter` · `Core::attending(&self) -> Vec<ClientId>`
  - `kernel::serving::serve<'a, I, J, C, R>(core: &'a RefCell<Core<I, J, C>>, clock: &'a R, sleep: &'a Sleep)` — un `async fn` che **non finisce mai**
  - `kernel::serving::POLICY_FUNCTION: Function` — la **sola** funzione registrata nel 2
  - `kernel::executor::nap(sleep: &Sleep, deadline: Monotonic)` — un `async fn`
  - `kernel::parameters::Parameters::new(executor_turn_limit, total_vram, arbiter_id, gui_tick)` e `Parameters::gui_tick(self) -> Millis`

⛔ **`serve` NON FINISCE, e questo decide come si scrive ogni banco.** L'attività è un `loop` senza uscita: in
produzione il limite di giri è `u64::MAX` (compito **8**, **D21**), nei banchi è finito, quindi
`Executor::run()` rende **`Err(RunError::TurnLimitReached)`** e non `Ok(())`. ⚠️ **Non è un rosso e non si
aggira:** ciò che le sonde guardano è **il pari**, cioè i messaggi che sono arrivati, non il valore di `run`.
Un'attività che si fermasse da sola sarebbe un core che smette di servire.

⛔ **E `Core` NON espone il trasporto oggi.** `Core::ipc` non nasce qui: il suo chiamante è il **rubinetto**
del core finto, compito **11**, e *«un elemento d'API senza chiamante in questo repository si cancella»* —
`crates/kernel/src/boundary.rs`. I tre accessori che nascono hanno un chiamante in **questo** banco.

- [ ] **Passo 1: le misure prima**

```bash
ls crates/kernel/src/serving.rs crates/kernel/tests/serving.rs 2>&1
grep -rnE '\b(fn|struct|enum|mod) (dispatch|session|serve|serving|listen)' crates/ --include='*.rs'
grep -rc 'Parameters::new' crates/ --include='*.rs' | grep -v ':0'
ls crates/kernel/tests/compile_fail/*.rs | while read f; do grep -q 'Parameters::new' "$f" && printf '%s\n' "$f"; done
grep -n 'pub const fn new' crates/kernel/src/parameters.rs
grep -nE '^(pub )?(struct|fn) (Yield|Suspended)|pub async fn' crates/kernel/src/executor.rs
grep -n 'pub fn policy\|pub fn allocated\|pub fn set_policy' crates/kernel/src/arbiter/mod.rs
git ls-files --eol crates/kernel/src/lib.rs crates/kernel/src/parameters.rs crates/kernel/src/executor.rs crates/kernel/src/registry.rs crates/kernel/src/wire/ipc.rs docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
```

Atteso: i due file **non esistono**; l'unico `dispatch` è `pub fn dispatch` di `gateway/mod.rs` (**P-36**);
i siti di `Parameters::new` sono **cinquantasette** in **ventitré** file e **nove** di quei file stanno in
`compile_fail/` (**P-37**); `Parameters::new` prende **tre** argomenti; in `executor.rs` **nessun** `async fn`
e nessun `Yield`; `policy()` rende `&VramPolicy`, `allocated()` rende `Mib`, `set_policy` prende
`(policy, step, journal)`; `registry.rs` e gli altri `i/lf w/crlf`, tranne i due file nuovi che nascono **LF**.
⚠️ **Se una cifra è diversa vale il comando, non questa riga**, ed è una voce d'errata prima di essere un
rimedio.

- [ ] **Passo 2: il tick nei parametri, e i cinquantasette siti**

In `crates/kernel/src/parameters.rs` (**`i/lf w/crlf`**, quindi `replace_unique.py`), l'import in testa:

*Trova* `use crate::arbiter::{ArbiterId, Mib};`
*Sostituisci con*:

```rust
use crate::arbiter::{ArbiterId, Mib};
use crate::time::Millis;
```

Il campo, in coda alla struttura:

*Trova* la riga `    arbiter_id: ArbiterId,` **intera, presa dal file**, e *Sostituisci con*:

```rust
    arbiter_id: ArbiterId,
    gui_tick: Millis,
```

Il costruttore — *Trova* le righe da `    pub const fn new(executor_turn_limit: u64, total_vram: Mib, arbiter_id: ArbiterId) -> Self {` fino al `    }` che lo chiude, **prese dal file**, e *Sostituisci con*:

```rust
    pub const fn new(
        executor_turn_limit: u64,
        total_vram: Mib,
        arbiter_id: ArbiterId,
        gui_tick: Millis,
    ) -> Self {
        Parameters {
            executor_turn_limit,
            total_vram,
            arbiter_id,
            gui_tick,
        }
    }
```

E l'accessore, in coda all'`impl`, **prima** della graffa che lo chiude:

```rust
    /// How long the activity that serves the gui sleeps between two turns.
    ///
    /// ⛔ IT IS DELIVERED AND NOT CHOSEN HERE, and the reason is the one §2.8 gives for every
    /// other field: the kernel's reactor has no I/O readiness, so the serving activity POLLS --
    /// `accept`, then one `receive` per client -- and how often it does so is a trade between how
    /// fast the gui feels and what the machine spends on an idle core. That trade is not the
    /// kernel's to settle, and a constant here would appear in no list and could not be made to
    /// vary in a campaign (gotcha #28).
    ///
    /// ⚠️ AND IT IS WHAT BOUNDS THE COST OF `crate::degradation::degradation_now`, which the
    /// serving activity re-reads once per turn while a gui is attending (D23 of the milestone 2
    /// part 2 plan). That function declares its own cost -- the whole journal is replayed to
    /// answer one question -- and this value is the only dial over it until the checkpoint
    /// `Journal::replay` names arrives.
    pub const fn gui_tick(self) -> Millis {
        self.gui_tick
    }
```

⛔ **Poi i siti, e sono cinquantasette in ventitré file.** Ognuno guadagna un quarto argomento. ⚠️ **Il
valore NON è lo stesso ovunque, e scriverne uno solo sarebbe un default travestito:** ogni banco sceglie il
proprio, come già fa per `TOTAL_VRAM`, e accanto alla costante va la ragione — è la forma che
`executor_determinism.rs` usa parola per parola (*«A LITERAL OF THIS BENCH, and it is inert here on
purpose»*). Per i banchi in cui il tick **non fa nulla** basta `Millis::new(0)` con quella frase; per i due
banchi di questo compito e per la campagna del compito 9 il valore è quello che la sonda esercita.

```bash
grep -rn 'Parameters::new' crates/ --include='*.rs' | grep -v compile_fail
```

⚠️ **`crates/kernel/src/arbiter/mod.rs` ha un sito nei propri banchi interni** e va trattato come gli altri.

- [ ] **Passo 3: i nove `compile_fail`, uno per uno, e i loro `.stderr`**

⛔ **Questo passo è il motivo per cui P-37 esiste, e NON si fa in blocco.** Il vincolo **10** di §11 e il
gotcha **#25** dicono che un `.stderr` si **legge**: una rigenerazione di massa disarma l'oracolo.

Per ciascuno dei nove: si aggiunge il quarto argomento **sulla stessa riga** se la chiamata sta su una riga,
e si **rimisura** se il `.stderr` è cambiato.

```bash
for f in $(ls crates/kernel/tests/compile_fail/*.rs); do grep -q 'Parameters::new' "$f" || continue; printf '=== %s\n' "$f"; grep -n 'Parameters::new' "$f"; done
```

Poi, **una volta sola dopo aver toccato tutti e nove**:

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
git diff --stat -- crates/kernel/tests/compile_fail/
```

⛔ **La lettura che conta, e le due direzioni:**

| Esito | Che cosa significa, e che cosa si fa |
|---|---|
| `git diff` non nomina **nessun** `.stderr` | ✅ il caso continua a cogliere **il proprio** errore, e la chiamata non ha spostato righe. Niente da fare |
| `git diff` nomina un `.stderr` **e** il diff sono soli **numeri di riga** | ✅ la chiamata ha aggiunto una riga sopra l'errore atteso. Si accetta il nuovo `.stderr` **dopo averlo letto**, e si scrive nel commit **quale** file e di quante righe |
| `git diff` nomina un `.stderr` e vi compare **`E0061`** | ⛔ **ROSSO VERO:** quella chiamata non è stata aggiornata. `E0061` è *«this function takes 4 arguments but 3 arguments were supplied»*, che **non è l'errore che quel caso esiste per cogliere** — si corregge la chiamata, non il `.stderr` |
| `git diff` nomina un `.stderr` e l'errore atteso è **sparito** | ⛔ **ROSSO VERO, e il peggiore:** il caso ha smesso di provare ciò per cui esiste. È una voce d'errata, non un aggiornamento |

- [ ] **Passo 4: le correzioni ai compiti 3 e 6 che il pre-controllo impone**

⛔ **Vanno NEI compiti, non nell'errata** — i compiti 3 e 6 sono **scritti e non eseguiti**, ed è la terza
volta che questo piano lo fa (decisione 15 della terza chiusura). Chi esegue trova i compiti già corretti;
questo passo esiste perché il **commit** lo dichiari.

**Nel compito 3** — `crates/kernel/src/wire/ipc.rs`:

| Cosa | Da | A | Perché |
|---|---|---|---|
| il campo di `PolicyReport` | `pub allocatable: Mib` | `pub allocated: Mib` | **P-38**, **D20**: il nome diceva il contrario del contenuto |
| il campo di `StepSummary` | `pub outcome: Option<bool>` | `pub done: bool` | **P-39**: `Some(false)` non ha produttore, e una variante sul filo non si ritira mai |

I due doc che li accompagnano si riscrivono così:

```rust
/// The policy with what the gui shows beside it (G15/G16).
///
/// ⛔ `allocated` IS WHAT THE BOOKS SPEAK FOR, THE TWO PERMANENT QUOTAS INCLUDED, and that is a
/// decision and not an oversight (D20 of the milestone 2 part 2 plan). ADR-0033 holds those two
/// as GRANTS WITH A HOLDER rather than as subtractions -- "the subtraction is not an exemption",
/// gotcha #4 -- so hiding them from this number would commit at the display layer the very
/// mistake the arbiter was built to avoid. `total` is the machine, delivered through
/// `Parameters::total_vram`; the kernel does not hold the two quotas and deliberately does not
/// (the doc of that accessor argues it).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct PolicyReport {
    pub policy: PolicyName,
    pub allocated: Mib,
    pub total: Mib,
}
```

```rust
/// One line of the step list: in milestone 2 these are registry invocations.
///
/// ⛔ A SUMMARY AND NOT THE RECORD. The journal's records must EVOLVE (ADR-0036) and this wire
/// renounces versioning (I4): sending the record itself would tie the two, and a field added to a
/// durable record would change these bytes with nothing going red.
///
/// ⛔ `done` IS A `bool` AND NOT AN `Option<bool>`, AND IT IS THE JOURNAL'S OWN VOCABULARY.
/// `RecordV1::outcome` carries no success flag: an outcome written says the step CLOSED, and
/// there is no "closed badly". A step whose effect failed returns before the outcome is written
/// and stays IN DOUBT (ADR-0007), which is `false` here. A third state would be a variant on the
/// wire with no producer, and an index on the wire never retires.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct StepSummary {
    pub step: u64,
    pub function: String,
    /// `false` while the step is still in doubt -- ADR-0007's intent written and no outcome yet.
    pub done: bool,
}
```

⚠️ **E il `stamp_set()` del compito 3 va riletto**, perché costruisce un `PolicyReport` e uno `StepSummary`
coi nomi vecchi: i due letterali prendono i nomi nuovi, e ⛔ **il valore di `done` resta quello che era**, o
i byte del timbro cambierebbero per un motivo che non è un cambio di schema.

**Nel compito 6** — `crates/kernel/src/registry.rs`, ed è la cura di **P-43**. Sopra `impl Registry`:

```rust
/// Whether the registry must ASK about the permission, or has just been told.
///
/// ⛔ AN ENUM AND NOT A `bool`, which is the lesson `crate::permission::Operation::is_write`
/// already recorded and `crate::wire::ipc::Access` repeats: a `bool` folds every future third
/// case into one of the two, in silence.
///
/// ⛔ AND IT EXISTS BECAUSE THE APPROVAL PATH IS OTHERWISE UNREACHABLE. `permission::grant`
/// writes a NOTE and wants a step somebody else opened -- its own doc -- so decision 21 of the
/// north star puts the grant INSIDE step A, between the invocation note and the effect. The check
/// below runs BEFORE step A is opened, so on the second pass it would answer "not granted" again
/// and the round would never close. The order belongs to the mechanism that owns it, not to the
/// caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approval {
    /// The invocation arrived on its own: the registry asks `is_granted` and refuses if not.
    Checked,
    /// The user has just approved the triple. The registry does not ask, and writes the grant on
    /// step A, between the invocation note and the effect.
    JustGiven,
}
```

`invoke` guadagna l'argomento, e le due righe che cambiano nel corpo:

*Trova* le righe da `        if !permission::is_granted(journal, &function.permission)` fino a `        }`, **intere, prese dal file**, e *Sostituisci con*:

```rust
        if approval == Approval::Checked
            && !permission::is_granted(journal, &function.permission)
                .map_err(InvokeError::Permission)?
        {
            return Err(InvokeError::PermissionRequired(function.permission));
        }
```

e, **subito dopo** la `.note(step, &noted(function, invoker, argument))` col suo `?`:

```rust
        // ⛔ THE GRANT GOES ON STEP A, BETWEEN THE INVOCATION NOTE AND THE EFFECT -- decision 21
        // of the north star, held here rather than by the caller. The permission settles on the
        // step it UNLOCKS, exactly as `sensor::run_the_ring` settles a verdict on the step it
        // judges.
        if approval == Approval::JustGiven {
            permission::grant(journal, step, &function.permission).map_err(InvokeError::Journal)?;
        }
```

⚠️ **La firma prende `approval: Approval` fra `argument` e `effect`**, e il blocco *Interfaces* del compito 6
lo dice. E il registro guadagna il suo **unico** accessore di lettura, che ha un chiamante da oggi:

```rust
    /// The function held under this name, if the registry holds one.
    ///
    /// ⛔ IT EXISTS FOR THE APPROVAL PATH AND HAS A CALLER FROM THE DAY IT IS WRITTEN:
    /// `crate::serving` compares the triple a peer sends back against the triple THE REGISTRY
    /// holds, rather than trusting the strings that came in (ADR-0014, and the doc of
    /// `crate::wire::ipc::Triple` asks that consumer for it by name).
    pub fn held(&self, name: &str) -> Option<Function> {
        self.functions.iter().find(|held| held.name == name).copied()
    }
```

⚠️ **E `invoke` usa `held` invece di ripetere la ricerca**, o sarebbero due copie della stessa riga.

✅ **Il compito 6 è già corretto NEL PIANO — le sue cinque chiamate a `invoke` portano `Approval::Checked`,
e una sonda nuova, `an_approval_just_given_skips_the_question_and_writes_the_grant_on_step_a`, tiene la
seconda variante.** ⛔ **Senza quella sonda il compito 6 consegnerebbe un artefatto provato solo dal banco
di un ALTRO compito**, che è ciò che **D1** vieta: ogni compito finisce con un artefatto provato da sé. La
sua specchio è `a_triple_that_is_not_granted_never_reaches_the_effect` — stesso giornale, stessa tripla non
concessa, e l'unica differenza è la parola.

- [ ] **Passo 5: la sospensione pubblica, in `executor.rs`**

⛔ **Oggi ogni banco si riscrive il proprio `Yield`**, e dal compito 11 servirebbe anche **fuori dal
workspace**, al rubinetto del core finto. In coda a `crates/kernel/src/executor.rs` (**`i/lf w/crlf`**):

```rust
/// Suspends the calling activity until `deadline`, and hands control back to the executor.
///
/// ⛔ IT IS THE ONLY WAY AN ACTIVITY OF THE KERNEL MAY SUSPEND (§2.4.1): the request is written
/// into the one `Sleep` cell and the activity then returns `Pending` exactly once, which is what
/// turns a written request into an actual suspension. Writing to the cell WITHOUT returning
/// `Pending` leaves the request to be read by whoever is polled next -- finding K-1 of the
/// 2026-08-11 audit, and the clearing in `poll_one_turn` is what contains it.
///
/// ⛔ IT IS PUBLIC BECAUSE ACTIVITIES ARE WRITTEN OUTSIDE THIS CRATE TOO. `gui/fake-core` runs
/// `crate::serving::serve` beside a tap of its own (§7 of the milestone 2 design), and every
/// bench since milestone 2 has been carrying a private copy of this two-line future.
///
/// ⚠️ A DEADLINE ALREADY REACHED BEHAVES AS A YIELD, and that is `Sleep::until`'s rule rather than
/// a second one: the executor promotes the activity and polls it again without touching the clock.
pub async fn nap(sleep: &Sleep, deadline: Monotonic) {
    sleep.until(deadline);
    Suspended(false).await;
}

/// The one-shot `Pending`. ⚠️ PRIVATE: what callers need is `nap`, and a bare yield with no
/// deadline would be a second way to suspend -- §2.4.1 allows exactly one.
struct Suspended(bool);

impl Future for Suspended {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            Poll::Pending
        }
    }
}
```

⚠️ **Nessun import nuovo:** `Future`, `Pin`, `Context` e `Poll` sono già in testa al file — si **rilegge** e
non si assume.

- [ ] **Passo 6: il banco che fallisce, da FUORI la crate**

`crates/kernel/tests/serving.rs`, **LF**, nuovo. ⛔ **È la terza domanda del pre-controllo in persona:**
l'attività si prova costruibile **da fuori dalla crate**, che è precisamente ciò che il core finto dovrà
fare al compito 11 (§7 del 2). Un banco dentro `src/` non lo direbbe.

```rust
//! The activity that serves the gui, driven FROM OUTSIDE THE CRATE against an in-memory wire.
//!
//! ⛔ WHAT THIS BENCH IS FOR, AND IT IS NOT ONLY THE DISPATCH: it is the first caller to build
//! `serving::serve` from outside, which is what `gui/fake-core` must do at task 11 (§7 of the
//! milestone 2 design). A signature that could only be assembled inside `kernel` would not be
//! found out any other way.
//!
//! ⛔ `Err(RunError::TurnLimitReached)` IS THE EXPECTED ANSWER OF EVERY ROUND. `serve` is a loop
//! with no exit -- a core that stopped serving would be the defect -- so the run ends when the
//! turns run out. What the probes read is THE WIRE and THE BOOKS, never the value of `run`.
//!
//! ⚠️ AND THE CLOCK IS SHARED THROUGH `SharedClock` rather than handed out twice, which is the
//! shape `crates/simulator/tests/arbiter_campaign.rs` already uses: `VirtualReactor` HOLDS the
//! instant, so two of them would drift and the activity would read an origin the executor had
//! long left. ⛔ THIS IS THE THIRD COPY OF THAT WRAPPER IN THE REPOSITORY -- declared rather than
//! discovered (gotcha #49). Where it should live is REGISTERED AND NOT TAKEN: the fourth caller
//! is task 9's campaign, and moving it into `simulator` is that task's call, with its measure.

use core::cell::{Cell, RefCell};

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, MakeRoom, Mib, Preemption, RemotePolicy,
    ResourceProfile, VramPolicy,
};
use kernel::executor::{nap, Executor, RunError, Sleep};
use kernel::numbering::Progressive;
use kernel::parameters::Parameters;
use kernel::ports::custody::{Custody, CustodyKey};
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use kernel::ports::journal::{Journal, StepId};
use kernel::ports::reactor::Reactor;
use kernel::record::{EffectClass, Record, RecordKind, RecordV1, RoutingDetail, Trust};
use kernel::serving::{serve, Core, POLICY_FUNCTION};
use kernel::time::{Millis, Monotonic, WallTime};
use kernel::wire::ipc::{
    build_stamp, stamp_set, Access, BuildStamp, Call, GrantRequest, IpcMessage, LayoutState,
    PolicyName, Protection, Triple,
};
use simulator::custody::MemoryCustody;
use simulator::journal::MemoryJournal;
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

/// ⚠️ A LITERAL OF THIS BENCH. Nothing here admits anything through the dispatch -- `Request` is
/// not served (D5) -- but `Parameters` carries every delivered value positionally, and §2.8.2
/// rule 2 forbids the kernel to name a default.
const TOTAL: Mib = Mib::new(8_192);

/// The tick this bench delivers. ⛔ IT IS NOT ZERO, AND THAT IS THE POINT: a zero tick makes `nap`
/// behave as a yield (`Sleep::until`'s own rule), and no round would ever move the virtual clock
/// or reach `Reactor::wait_until`.
const TICK: Millis = Millis::new(50);

/// Enough turns for the longest round here, with room to spare. ⛔ FIXED AND VERSIONED WITH THIS
/// FILE (constraint 7 of §11), never drawn from the clock or from the environment.
const TURNS: u64 = 64;

const GUI: ClientId = ClientId::new(1);
const OTHER: ClientId = ClientId::new(2);

/// The gui side of the wire, and the only thing the probes read.
struct Wire {
    waiting: Vec<ClientId>,
    up: Vec<(ClientId, Vec<u8>)>,
    down: Vec<(ClientId, Vec<u8>)>,
    gone: Vec<ClientId>,
}

impl Wire {
    const fn new() -> Self {
        Wire {
            waiting: Vec::new(),
            up: Vec::new(),
            down: Vec::new(),
            gone: Vec::new(),
        }
    }

    /// The gui connects, and then says these things in this order.
    fn arrives(&mut self, client: ClientId, said: &[IpcMessage]) {
        self.waiting.push(client);
        for message in said {
            self.up
                .push((client, message.encode().expect("the bench frames what it sends")));
        }
    }

    /// This client is dead: every `send` and `receive` on it answers `Disconnected`.
    fn dead(&mut self, client: ClientId) {
        self.gone.push(client);
    }

    /// Everything the core has said to this client, in order, decoded and taken off the wire.
    fn heard(&mut self, client: ClientId) -> Vec<IpcMessage> {
        let mut out = Vec::new();
        let mut kept = Vec::new();
        for (id, bytes) in self.down.drain(..) {
            if id == client {
                out.push(IpcMessage::decode(&bytes).expect("the core frames what it sends"));
            } else {
                kept.push((id, bytes));
            }
        }
        self.down = kept;
        out
    }
}

/// The `ipc` port over that wire.
struct FakeIpc<'a> {
    wire: &'a RefCell<Wire>,
}

impl Ipc for FakeIpc<'_> {
    /// ⚠️ IT DOES NOT CONSULT `gone`, deliberately: a listener hands over whoever connected, and
    /// whether that peer is still alive is what the FIRST `receive` finds out. A fake that refused
    /// to accept a dead client would hide the very path `on_disconnect` exists for.
    fn accept(&mut self) -> Option<ClientId> {
        let mut wire = self.wire.borrow_mut();
        if wire.waiting.is_empty() {
            None
        } else {
            Some(wire.waiting.remove(0))
        }
    }

    fn send(&mut self, client: ClientId, message: &[u8]) -> Result<(), IpcError> {
        let mut wire = self.wire.borrow_mut();
        if wire.gone.contains(&client) {
            return Err(IpcError::Disconnected);
        }
        wire.down.push((client, Vec::from(message)));
        Ok(())
    }

    fn receive(&mut self, client: ClientId) -> Result<Option<Vec<u8>>, IpcError> {
        let mut wire = self.wire.borrow_mut();
        if wire.gone.contains(&client) {
            return Err(IpcError::Disconnected);
        }
        let at = wire.up.iter().position(|(id, _)| *id == client);
        Ok(at.map(|index| wire.up.remove(index).1))
    }
}

/// One clock for the executor AND for the activity. ⚠️ See the note at the top of this file.
struct SharedClock<'a> {
    inner: &'a RefCell<VirtualReactor>,
}

impl Reactor for SharedClock<'_> {
    fn now(&self) -> Monotonic {
        self.inner.borrow().now()
    }

    fn wall_time(&self) -> WallTime {
        self.inner.borrow().wall_time()
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        self.inner.borrow_mut().wait_until(deadline)
    }
}

type BenchCore<'b> = Core<FakeIpc<'b>, MemoryJournal, MemoryCustody>;

struct Bench {
    wire: RefCell<Wire>,
    clock: RefCell<VirtualReactor>,
}

impl Bench {
    fn new() -> Self {
        Bench {
            wire: RefCell::new(Wire::new()),
            clock: RefCell::new(VirtualReactor::new()),
        }
    }

    /// One round with the serving activity alone.
    fn round<'b>(
        &'b self,
        before: impl FnOnce(&mut BenchCore<'b>),
        after: impl FnOnce(&mut BenchCore<'b>),
    ) {
        self.round_with_tap(before, false, after);
    }

    /// One round, optionally with a SECOND activity on the same cell.
    ///
    /// ⚠️ THE DECLARATION ORDER IS LOAD-BEARING and swapping two lines does not compile: the
    /// executor borrows `sleep`, `core` and `clock` for its whole life, and locals drop in reverse
    /// order of declaration.
    fn round_with_tap<'b>(
        &'b self,
        before: impl FnOnce(&mut BenchCore<'b>),
        tapped: bool,
        after: impl FnOnce(&mut BenchCore<'b>),
    ) {
        let parameters = Parameters::new(TURNS, TOTAL, ArbiterId::new(1), TICK);
        let mut built = Core::new(
            FakeIpc { wire: &self.wire },
            MemoryJournal::new(),
            MemoryCustody::new(),
            Arbiter::new(parameters, VramPolicy::Remote(RemotePolicy)),
            Progressive::starting_at(1),
            parameters,
        );
        before(&mut built);

        let core = RefCell::new(built);
        let clock = SharedClock { inner: &self.clock };
        let sleep = Sleep::new();
        let tapped_once = Cell::new(false);
        let mut executor = Executor::new(
            SeededRng::new(1),
            SharedClock { inner: &self.clock },
            parameters,
            &sleep,
        );
        executor.spawn(serve(&core, &clock, &sleep));
        if tapped {
            executor.spawn(degrade_once(&core, &clock, &sleep, &tapped_once));
        }

        assert_eq!(
            executor.run(),
            Err(RunError::TurnLimitReached),
            "`serve` is a loop with no exit: the run ends when the turns run out"
        );
        drop(executor);
        if tapped {
            // ⛔ THE NON-VACUITY OF THE TAPPED ROUND. Without it, a tap that never woke up would
            // make `a_degradation_written_by_a_second_activity_reaches_the_gui` prove nothing at
            // all -- it would just be the welcome, arriving as usual.
            assert!(tapped_once.get(), "the second activity must have run");
        }
        after(&mut core.into_inner());
    }

    fn heard(&self, client: ClientId) -> Vec<IpcMessage> {
        self.wire.borrow_mut().heard(client)
    }
}

/// A SECOND activity on the same cell: it sleeps one tick, then writes a degraded routing into the
/// core's journal.
///
/// ⛔ IT IS THE TAP OF `gui/fake-core` IN MINIATURE (§7 of the milestone 2 design, the `degrade`
/// word): the write reaches the journal WITHOUT passing through the dispatch, which is the whole
/// reason D23 re-reads the degradation every turn instead of after its own writes.
async fn degrade_once<'b>(
    core: &RefCell<BenchCore<'b>>,
    clock: &SharedClock<'_>,
    sleep: &Sleep,
    ran: &Cell<bool>,
) {
    nap(sleep, clock.now().saturating_add(Millis::new(120))).await;
    let record = Record::V1(RecordV1::routing(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "a degraded routing, written by somebody who is not the dispatch",
        RoutingDetail::new("a-model", 2, true),
    ))
    .encode();
    // ⚠️ ONE STATEMENT, SO THE BORROW DIES WITH IT. Holding it across a suspension is what the
    // whole shape of `serve` forbids, and this activity plays by the same rule.
    core.borrow_mut()
        .journal()
        .intent(StepId::new(900), &record)
        .expect("the memory journal writes");
    ran.set(true);
}

/// A stamp that is NOT this build's.
///
/// ⛔ IT CANNOT BE MINTED, and that is the whole value of `BuildStamp`: its doc refuses a public
/// constructor, because "a stamp anyone can mint from any number is a stamp that proves nothing".
/// So it is TAKEN from the canonical set, whose `Hello` carries a fixed literal. ⚠️ AND THE
/// ASSERTION IS NOT DECORATION: the day that literal ever coincided with a real stamp, this probe
/// would be testing the ACCEPTING path while claiming to test the refusing one.
fn a_stamp_that_is_not_ours() -> BuildStamp {
    let Some(IpcMessage::Hello(stamp)) = stamp_set().into_iter().next() else {
        panic!("the canonical set opens with `Hello`")
    };
    assert_ne!(
        stamp,
        build_stamp(),
        "the canonical `Hello` must not carry this build's own stamp"
    );
    stamp
}

/// The call the gui sends to change the policy, with the argument the one function reads.
fn switch_to_local() -> Call {
    Call {
        function: String::from(POLICY_FUNCTION.name),
        argument: String::from("local"),
    }
}

/// The triple of the one function, as the core itself would put it on the wire.
fn the_triple() -> Triple {
    Triple {
        tool: String::from(POLICY_FUNCTION.permission.tool),
        resource: String::from(POLICY_FUNCTION.permission.resource),
        operation: Access::Write,
    }
}
```

⛔ **E le sonde, che sono tante perché ciascuna coglie ciò che le altre non colgono.** ⚠️ **Quante siano lo
dice il comando e non questa riga** — è **P-35**, e scrivendo questo compito l'ho sbagliato una volta:

```bash
grep -c '^fn [a-z_]*() {$' crates/kernel/tests/serving.rs
```

```rust
#[test]
fn the_welcome_is_the_five_messages_of_sequence_one() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);

    bench.round(|_| {}, |_| {});

    let heard = bench.heard(GUI);
    // ⛔ THE ORDER IS THE ASSERTION AND NOT A BONUS: sequence 1 of the north star fixes it, and a
    // gui told the policy before it was admitted would be drawing a core it is not attached to.
    assert!(
        matches!(heard.first(), Some(IpcMessage::Accepted(Protection::AsSystemAccount))),
        "the welcome opens with `Accepted`: {heard:?}"
    );
    assert!(
        matches!(heard.get(1), Some(IpcMessage::Degradation(_))),
        "then the degradation: {heard:?}"
    );
    assert!(
        matches!(
            heard.get(2),
            Some(IpcMessage::Policy(report))
                if report.policy == PolicyName::Remote
                    && report.total == TOTAL
                    && report.allocated == Mib::ZERO
        ),
        "then the policy: the default of ADR-0006, the delivered total, empty books: {heard:?}"
    );
    assert!(
        matches!(heard.get(3), Some(IpcMessage::Layout(LayoutState::Nothing))),
        "then the layout, and an archive that opened and holds nothing is `Nothing`: {heard:?}"
    );
    assert!(
        matches!(heard.get(4), Some(IpcMessage::Steps(steps)) if steps.is_empty()),
        "then the step list, empty on a fresh journal: {heard:?}"
    );
    // ⛔ THE SECOND DIRECTION, and without it the five above pass over a core that never stops
    // talking: §6.1.4 says the core sends only what CHANGES, and nothing changed after the welcome.
    assert_eq!(
        heard.len(),
        5,
        "after the welcome the core sends only what changed: {heard:?}"
    );
}

#[test]
fn a_stale_stamp_gets_the_expected_one_and_then_the_core_stops_listening() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        // ⛔ THE SECOND MESSAGE IS THE PROBE. "The core closes" is not an operation of the port
        // (decision 22): what it does is STOP LISTENING, and only a message sent AFTER the refusal
        // tells that apart from "it answered and then had nothing more to say".
        &[
            IpcMessage::Hello(a_stamp_that_is_not_ours()),
            IpcMessage::Invoke(switch_to_local()),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "a refused gui writes nothing, and its later words are not read"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(
        heard,
        vec![IpcMessage::StaleBuild(build_stamp())],
        "a stale gui is told the EXPECTED stamp, and then nothing at all: {heard:?}"
    );
}

#[test]
fn a_request_reaches_neither_the_arbiter_nor_the_journal() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Request(GrantRequest {
                reserved_vram: Mib::new(1_024),
                compute_class: ComputeClass::Batch,
                preemption: Preemption::Never,
            }),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            // ⛔ THE DIRECTION THAT DECIDES, AND IT IS NOT "no `Verdict` came back". D5 says the
            // branch builds NO `ResourceProfile` at all, so what has to be shown is that no value
            // of the peer reached the arbiter: the books are untouched, and untouched means zero
            // because this bench reserves nothing of its own.
            assert_eq!(
                core.arbiter().allocated(),
                Mib::ZERO,
                "an unserved request must not reach the arbiter's books"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "and it must not open a step either"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(heard.len(), 5, "the welcome, and not one word more: {heard:?}");
}

#[test]
fn an_invoke_without_the_permission_asks_for_the_triple_and_writes_nothing() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Invoke(switch_to_local()),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "remote",
                "a refused invocation must not run the effect"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "the registry refuses BEFORE it opens step A: nothing is written"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(
        heard.get(5),
        Some(&IpcMessage::PermissionRequired(the_triple())),
        "the gui is asked for the triple the REGISTRY holds: {heard:?}"
    );
}

#[test]
fn an_approve_whose_triple_does_not_match_the_registry_is_refused() {
    let bench = Bench::new();
    let mut lie = the_triple();
    // ⛔ THE STRINGS CAME BACK FROM THE PEER AND ARE UNTRUSTED (ADR-0014), which is what the doc of
    // `wire::ipc::Triple` asks this consumer for by name. A core that believed them would write a
    // permission for a triple nobody was ever asked about.
    lie.resource = String::from("everything");

    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Approve { triple: lie, call: switch_to_local() },
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "remote",
                "a triple that does not match must not unlock the effect"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "and it must not write a permission either"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(heard.len(), 5, "the welcome, and nothing else: {heard:?}");
}

#[test]
fn an_approve_changes_the_policy_and_the_gui_is_told() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Approve { triple: the_triple(), call: switch_to_local() },
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "local",
                "the whole point of the round: the effect really ran"
            );
            let kinds: Vec<RecordKind> = core
                .journal()
                .replay()
                .expect("the memory journal replays")
                .iter()
                .map(|(_, bytes)| match Record::decode(bytes).expect("this build wrote these") {
                    Record::V1(body) => body.kind(),
                })
                .collect();
            // ⛔ THE ORDER OF DECISION 21, READ BACK OUT OF THE ARCHIVE: step A's intent, the
            // invocation note, the permission note, then step B's own intent and outcome, then A's
            // outcome. A round that wrote the permission FIRST would be putting a note on a step
            // nobody had opened, which is what `permission::grant`'s doc forbids.
            assert_eq!(
                kinds,
                vec![
                    RecordKind::Intent,
                    RecordKind::Invocation,
                    RecordKind::Permission,
                    RecordKind::Intent,
                    RecordKind::Outcome,
                    RecordKind::Outcome,
                ],
                "the journal must read as decision 21 dictates"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert!(
        matches!(heard.get(5), Some(IpcMessage::Policy(report)) if report.policy == PolicyName::Local),
        "the gui is told the NEW policy: {heard:?}"
    );
    assert!(
        matches!(heard.get(6), Some(IpcMessage::Steps(steps)) if steps.len() == 1 && steps[0].done),
        "and the step list, with the invocation closed: {heard:?}"
    );
}

#[test]
fn a_client_that_dies_gives_its_grant_back() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);
    // ⚠️ IT CONNECTS AND SAYS NOTHING, AND IT IS ALREADY DEAD. That is not a contrived case: it is
    // `ClientGrants::on_disconnect`'s own words -- "a gui may die before it ever asked, it may die
    // before it was ever accepted".
    bench.wire.borrow_mut().arrives(OTHER, &[]);
    bench.wire.borrow_mut().dead(OTHER);

    bench.round(
        |core| {
            // ⛔ THE GRANT IS PUT IN BY HAND, AND THE REASON IS D5 ITSELF: nothing in the dispatch
            // of milestone 2 issues one, because `Request` is not served. The wiring still has to
            // be right -- ADR-0033 says the core notices FROM THE IPC DISCONNECTION and
            // reconciles -- and this is the only way to hold it until the 3D pillar brings the
            // writer.
            let profile = ResourceProfile {
                name: "a-client-of-the-bench",
                reserved_vram: Mib::new(1_024),
                compute_class: ComputeClass::Batch,
                preemption: Preemption::Never,
            };
            let Admission::Granted(grant) =
                core.arbiter()
                    .admit(&profile, Millis::new(1_000_000), Monotonic::ORIGIN)
            else {
                panic!("this bench's machine is big enough for one grant")
            };
            core.grants().register(OTHER, grant);
            assert_eq!(
                core.arbiter().allocated(),
                Mib::new(1_024),
                "the books hold it before the round, or the round proves nothing"
            );
        },
        |core| {
            assert_eq!(
                core.arbiter().allocated(),
                Mib::ZERO,
                "the grant of a client that died must come back to the books"
            );
            assert!(
                !core.attending().contains(&OTHER),
                "and the client leaves the table"
            );
        },
    );

    // ⛔ THE OTHER DIRECTION, AND IT IS THE ONE A SWEEPING RECONCILIATION WOULD BREAK: the client
    // that did NOT die is still served. Without it, an `on_disconnect` that gave back every pair
    // it holds would pass the assertion above -- it is `gui_death_campaign.rs`'s standing witness,
    // in miniature.
    let heard = bench.heard(GUI);
    assert_eq!(heard.len(), 5, "the living gui got its whole welcome: {heard:?}");
}

#[test]
fn save_layout_comes_back_with_what_the_port_holds() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::SaveLayout(vec![7, 7, 7]),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.custody().retrieve(CustodyKey::Layout),
                Ok(Some(vec![7, 7, 7])),
                "the package really reached the seventh port"
            );
        },
    );

    let heard = bench.heard(GUI);
    // ⛔ DECISION 13: the core sends back WHAT IT HOLDS after every write, never an error variant,
    // so a failed write comes back as the OLD package and the gui sees it by comparing.
    assert_eq!(
        heard.get(5),
        Some(&IpcMessage::Layout(LayoutState::Package(vec![7, 7, 7]))),
        "and what comes back is what the port holds: {heard:?}"
    );
}

#[test]
fn an_unchanged_degradation_is_not_resent() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);

    bench.round(|_| {}, |_| {});

    let heard = bench.heard(GUI);
    // ⛔ THE FIRST DIRECTION OF D23: one `Degradation`, in the welcome, and the sweep of every one
    // of the turns that follow sends nothing, because nothing changed. Without this the probe
    // below would pass over a core that shouts the same state on every tick.
    assert_eq!(
        heard
            .iter()
            .filter(|message| matches!(message, IpcMessage::Degradation(_)))
            .count(),
        1,
        "an unchanged degradation is not resent: {heard:?}"
    );
}

#[test]
fn a_degradation_written_by_a_second_activity_reaches_the_gui() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);

    bench.round_with_tap(|_| {}, true, |_| {});

    let heard = bench.heard(GUI);
    // ⛔ THE SECOND DIRECTION OF D23, AND IT IS THE ONE THAT DECIDES THE RULE. The degraded routing
    // is written by ANOTHER ACTIVITY -- which is exactly the `degrade` word of `gui/fake-core`'s
    // tap (§7) -- so a core that only re-read the degradation after ITS OWN writes would never see
    // it, and this count would stay at one.
    let reports: Vec<bool> = heard
        .iter()
        .filter_map(|message| match message {
            IpcMessage::Degradation(report) => Some(report.routing_degraded),
            _ => None,
        })
        .collect();
    assert_eq!(
        reports,
        vec![false, true],
        "the welcome says clean, and the sweep then says degraded: {heard:?}"
    );
}
```

⛔ **E che cosa questo file NON prova, detto invece che lasciato dedurre.** La disciplina del prestito —
*«nessun `borrow` attraversa una sospensione»* — **non ha una sonda propria**, e non ne avrà una: è tenuta
da `a_degradation_written_by_a_second_activity_reaches_the_gui`, dove un prestito trattenuto da `serve`
attraverso il suo `.await` farebbe **`already borrowed`** nel `borrow_mut` del rubinetto. ⚠️ **Una seconda
sonda sarebbe la stessa corsa una seconda volta** (gotcha #49), e il modo in cui questa la tiene è scritto
qui perché non venga «aggiunto» domani.

Il rosso atteso del passo:

```bash
cargo test --locked -p kernel --test serving 2>&1 | tail -20
```

Atteso: `error[E0432]: unresolved import`, su `kernel::serving`.

- [ ] **Passo 7: il modulo `serving`**

`crates/kernel/src/serving.rs`, **LF**, nuovo.

```rust
//! The activity that serves the gui over the `ipc` port (§5 of the milestone 2 design).
//!
//! ⛔ IT LIVES IN `kernel` AND NOT IN `daemon`, and that is what makes it checkable at all. A
//! dispatch inside a binary is reachable by no integration test -- `daemon`'s own unit module
//! writes out why -- the DST moves THIS one with `simulator::ipc::DyingGui`, and `gui/fake-core`
//! runs THIS one on in-memory ports instead of writing a second dispatch of its own (§7). A second
//! copy is green on the day the two drift apart.
//!
//! ⛔ NAMED `serving` AND NOT `dispatch`: `crate::gateway::dispatch` already exists and means
//! another thing -- handing a conforming token to a provider. Two `dispatch`es in one kernel is
//! the ambiguity `crate::numbering` refused for `counter`. And not `session` either:
//! `crate::permission` opens by declaring that there is no session in the kernel, and the
//! permission session boundary belongs to a later sub-project.
//!
//! # One borrow per turn, and never across a suspension
//!
//! ⛔ THE WHOLE CORE SITS BEHIND ONE `RefCell` THE CALLER OWNS, which the tap of `gui/fake-core`
//! shares (§7). That is sound here and it is NOT sound by luck: the executor polls ONE activity at
//! a time and nothing inside a poll can reach the executor (§2.4.1), so two activities cannot be
//! inside the cell at once -- PROVIDED no borrow is held across an `.await`. Every borrow in this
//! file is taken and dropped inside one block, and the tapped round of `tests/serving.rs` is what
//! holds it: a borrow that survived the `.await` panics there.
//!
//! # What it does NOT do, and each one has its closer
//!
//! - ⛔ IT DOES NOT SERVE `IpcMessage::Request` -- D5 of the milestone 2 part 2 plan, argued on the
//!   branch itself. Nothing here builds a `ResourceProfile`, so no value the peer chose reaches the
//!   arbiter at all.
//! - ⚠️ NOTHING HERE EVER CALLS `ClientGrants::register`, which follows from the line above: with
//!   no request served, no grant is ever issued to a client. The register and `on_disconnect` are
//!   wired anyway, because ADR-0033 says the core notices a dead gui FROM THE IPC DISCONNECTION and
//!   reconciles; the writer arrives with the 3D pillar, and `tests/serving.rs` holds the wiring
//!   meanwhile by putting a grant in by hand.
//! - ⚠️ IT NEVER STOPS. In production the turn limit is `u64::MAX` (task 8); under a finite limit
//!   the run ends as `RunError::TurnLimitReached`, which is the expected answer and not a failure.

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::arbiter::{Arbiter, LocalPolicy, MakeRoom, RemotePolicy, VramPolicy};
use crate::client::ClientGrants;
use crate::degradation::degradation_now;
use crate::executor::{nap, Sleep};
use crate::numbering::Progressive;
use crate::parameters::Parameters;
use crate::permission::{Operation, Permission};
use crate::ports::custody::{Custody, CustodyError, CustodyKey};
use crate::ports::ipc::{ClientId, Ipc, IpcError};
use crate::ports::journal::{Journal, StepId};
use crate::ports::reactor::Reactor;
use crate::record::{Detail, EffectClass, Record, RecordKind};
use crate::registry::{Approval, Function, InvokeError, Invoker, Registry};
use crate::time::Monotonic;
use crate::wire::ipc::{
    build_stamp, Access, BuildStamp, Call, DegradationReport, IpcMessage, LayoutState, PolicyName,
    PolicyReport, Protection, StepSummary, Triple,
};

/// The ONE function the registry holds in milestone 2: the VRAM policy change.
///
/// ⛔ IT IS REGISTERED HERE AND NOT IN `crate::registry` -- D16, which is rule 1 of ADR-0038: the
/// kernel gives the MECHANISM, and WHICH functions exist is brought by whoever uses them. A
/// registry that named `Arbiter::set_policy` would have to import the arbiter, and the second
/// invoker -- the gesture, with sub-project 12 -- would have to add its effect in there too, which
/// is the "logic for gestures only" that ADR refuses, inside out.
///
/// ⚠️ `Idempotent` AND NOT `Unrepeatable`: setting the policy twice to the same value leaves the
/// same world, which is the argument written beside `Arbiter::set_policy` itself.
pub const POLICY_FUNCTION: Function = Function {
    name: "vram-policy",
    permission: Permission {
        tool: "registry",
        resource: "arbiter",
        operation: Operation::Write,
    },
    effect: EffectClass::Idempotent,
};

/// Where a known client has got to.
enum Stage {
    /// Connected, and it has not introduced itself. Only `Hello` is answered.
    Greeting,
    /// The stamp matched. From here the core sends the piece that CHANGES, and the gui does not
    /// pull (§6.1.4).
    Attending {
        /// The last degradation this client was told about -- D23: the sweep sends only on change.
        told: DegradationReport,
    },
}

struct Client {
    id: ClientId,
    stage: Stage,
}

/// Whether the client the turn is looking at stays on the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Keep,
    Forget,
}

/// Everything the serving activity owns, in one place so that one cell holds it all.
pub struct Core<I: Ipc, J: Journal, C: Custody> {
    ipc: I,
    journal: J,
    custody: C,
    arbiter: Arbiter,
    registry: Registry,
    grants: ClientGrants,
    steps: Progressive,
    clients: Vec<Client>,
    parameters: Parameters,
}

impl<I: Ipc, J: Journal, C: Custody> Core<I, J, C> {
    /// ⛔ THE REGISTRY IS BUILT AND FILLED HERE, with the one function of milestone 2: D16.
    /// A caller cannot forget to register it, and a caller cannot register a second one either --
    /// which is deliberate: the invokers grow (the gesture, the voice), the FUNCTIONS of milestone
    /// 2 do not.
    pub fn new(
        ipc: I,
        journal: J,
        custody: C,
        arbiter: Arbiter,
        steps: Progressive,
        parameters: Parameters,
    ) -> Self {
        let mut registry = Registry::new();
        registry.register(POLICY_FUNCTION);
        Core {
            ipc,
            journal,
            custody,
            arbiter,
            registry,
            grants: ClientGrants::new(),
            steps,
            clients: Vec::new(),
            parameters,
        }
    }

    /// The journal, for whoever shares this cell.
    pub fn journal(&mut self) -> &mut J {
        &mut self.journal
    }

    /// The arbiter, for whoever shares this cell.
    pub fn arbiter(&mut self) -> &mut Arbiter {
        &mut self.arbiter
    }

    /// The seventh port, to read what it holds.
    ///
    /// ⚠️ `&C` AND NOT `&mut C`: `keep` belongs to the dispatch, and a caller that could write the
    /// layout from outside would be a second writer of a value the gui believes the core owns.
    pub fn custody(&self) -> &C {
        &self.custody
    }

    /// The register of who holds what.
    ///
    /// ⚠️ ITS ONLY CALLER TODAY IS A BENCH, and that is stated rather than hidden: milestone 2
    /// issues no grant to a client (D5), so `register` has no production writer yet. Removing it
    /// would leave `on_disconnect`'s wiring held by nothing at all.
    pub fn grants(&mut self) -> &mut ClientGrants {
        &mut self.grants
    }

    /// Who has finished the handshake. ⚠️ A client still in `Greeting`, and a client that has been
    /// refused or has died, is not here.
    pub fn attending(&self) -> Vec<ClientId> {
        self.clients
            .iter()
            .filter(|client| matches!(client.stage, Stage::Attending { .. }))
            .map(|client| client.id)
            .collect()
    }
}
```

⛔ **Poi il giro, e il dispaccio.** Ancora in `crates/kernel/src/serving.rs`:

```rust
/// The activity: one turn, then a nap of the delivered tick, for ever.
///
/// ⛔ IT POLLS, AND THAT IS NOT A CHOICE: the `reactor` port has no I/O readiness, which §5 of the
/// milestone 2 design states as the reason for the tick. What IS a choice is the tick's value, and
/// it is delivered (ADR-0034) rather than picked here.
pub async fn serve<'a, I, J, C, R>(core: &'a RefCell<Core<I, J, C>>, clock: &'a R, sleep: &'a Sleep)
where
    I: Ipc,
    J: Journal,
    C: Custody,
    R: Reactor,
{
    loop {
        // ⛔ ONE BLOCK, AND THE BORROW DIES WITH IT. See the head of this file: the `.await` below
        // must never be reached with the cell borrowed.
        let tick = {
            let core = &mut *core.borrow_mut();
            let now = clock.now();
            core.take_connections();
            core.answer_everyone(now);
            core.sweep_degradation(now);
            core.parameters.gui_tick()
        };
        nap(sleep, clock.now().saturating_add(tick)).await;
    }
}

impl<I: Ipc, J: Journal, C: Custody> Core<I, J, C> {
    /// Everybody the listener has ready, added in the greeting stage.
    fn take_connections(&mut self) {
        while let Some(id) = self.ipc.accept() {
            self.clients.push(Client { id, stage: Stage::Greeting });
        }
    }

    /// One `receive` per known client, and the dispatch of whatever came back.
    fn answer_everyone(&mut self, now: Monotonic) {
        let mut index = 0;
        while index < self.clients.len() {
            let id = self.clients[index].id;
            let outcome = match self.ipc.receive(id) {
                // ⛔ ONE MESSAGE PER CLIENT PER TURN, AND IT IS A CHOICE: draining a client dry
                // would let one peer hold the turn for as long as it keeps talking, and every
                // other client -- and the degradation sweep -- would wait behind it.
                Ok(None) => Outcome::Keep,
                Ok(Some(bytes)) => self.answer(index, &bytes),
                // ⛔ THE ONE SIGNAL THERE IS. `Ipc` has no disconnection event, so an `Err` on
                // `send` or `receive` is what ADR-0033 calls "the core notices from the ipc
                // disconnection" -- `gui_death_campaign.rs` reads it in exactly this place.
                Err(IpcError::Disconnected) => Outcome::Forget,
                // ⚠️ A PEER THAT TALKS NONSENSE IS STILL THERE, which is `IpcError`'s own words:
                // folding this into `Disconnected` would tear down a live gui over one bad frame.
                Err(IpcError::MalformedMessage) => Outcome::Keep,
            };
            match outcome {
                Outcome::Keep => index += 1,
                Outcome::Forget => self.forget(index, now),
            }
        }
    }

    /// One message, dispatched.
    fn answer(&mut self, index: usize, bytes: &[u8]) -> Outcome {
        let Ok(message) = IpcMessage::decode(bytes) else {
            // ⚠️ SAME READING AS `MalformedMessage` ABOVE: bytes that will not decode are a bad
            // frame, not a dead peer.
            return Outcome::Keep;
        };
        let id = self.clients[index].id;
        match message {
            IpcMessage::Hello(stamp) => self.greet(index, stamp),
            IpcMessage::Invoke(call) => self.run(id, call, Approval::Checked),
            IpcMessage::Approve { triple, call } => self.approve(id, triple, call),
            IpcMessage::SaveLayout(package) => self.keep_layout(id, &package),
            // ⛔ NOT SERVED, AND IT IS D5. Serving it would force the core to name a compute lane
            // and a preemption for a consumer that does not exist: `ComputeClass::Batch` is
            // documented "3D render, indexing, background runs" and ADR-0033 describes a VIEWER,
            // and no line anywhere says which is right -- choosing one would be a deduction
            // presented as a design. Refusing it with `Verdict::Refused` would LIE about that
            // type, which means "bigger than the whole machine". Not serving it asserts nothing
            // false, is the shape this repository already uses (`promote` has no caller either --
            // "declared, not pinned", gotcha #73), and takes the privilege away at the root: NO
            // VALUE OF THE PEER REACHES THE ARBITER, because nothing here builds a
            // `ResourceProfile` at all. ⚠️ Entry 27 of milestone 6's open items keeps its trigger
            // intact, and its closer is still the 3D pillar.
            IpcMessage::Request(_) => Outcome::Keep,
            // ⚠️ CORE -> GUI VARIANTS ARRIVING UPWARD. The schema is ONE enum for both directions
            // (I4 renounces versioning, so there is no per-direction type), which makes these
            // expressible and perfectly decodable -- so they are not `MalformedMessage`, and they
            // are IGNORED rather than answered. Tearing the client down for one is the move
            // `IpcError`'s own doc argues against: the gui is sacrificial, and a live one must not
            // be lost over a stray frame.
            IpcMessage::Accepted(_)
            | IpcMessage::StaleBuild(_)
            | IpcMessage::Degradation(_)
            | IpcMessage::Policy(_)
            | IpcMessage::PermissionRequired(_)
            | IpcMessage::Token { .. }
            | IpcMessage::Layout(_)
            | IpcMessage::Steps(_)
            | IpcMessage::Verdict(_) => Outcome::Keep,
        }
    }

    /// The handshake of §6.1.2, and the welcome of sequence 1.
    fn greet(&mut self, index: usize, stamp: BuildStamp) -> Outcome {
        let id = self.clients[index].id;
        if stamp != build_stamp() {
            // ⛔ "THE CORE CLOSES" IS NOT AN OPERATION OF THE PORT (decision 22). What the core
            // does is STOP LISTENING, so this client leaves the table: the gui does not start, says
            // so (§6.1.2), exits by itself, and there is nobody left here to see it go.
            let _ = self.tell(id, &IpcMessage::StaleBuild(build_stamp()));
            return Outcome::Forget;
        }

        let told = match self.degradation() {
            Some(report) => report,
            // ⛔ AN UNKNOWN DEGRADATION IS NOT SENT AS "NOTHING IS DEGRADED", which is the silent
            // degradation ADR-0019 forbids and the argument `DegradationError` spells out. The wire
            // has no third state for it: the gui is simply not told, and the sweep will tell it as
            // soon as the journal reads again. ⚠️ REGISTERED AND NOT TAKEN: whether `Degradation`
            // should gain an "unknown" the way `LayoutState` gained `Unavailable` (decision 35) is
            // the owner's, and it is a variant on a wire that never retires one.
            None => return Outcome::Keep,
        };

        // The welcome, in the order sequence 1 of the north star fixes.
        for message in [
            IpcMessage::Accepted(Protection::AsSystemAccount),
            IpcMessage::Degradation(told),
            IpcMessage::Policy(self.policy_report()),
            IpcMessage::Layout(self.layout()),
        ] {
            if self.tell(id, &message) == Outcome::Forget {
                return Outcome::Forget;
            }
        }
        if let Some(steps) = self.step_list() {
            if self.tell(id, &IpcMessage::Steps(steps)) == Outcome::Forget {
                return Outcome::Forget;
            }
        }

        self.clients[index].stage = Stage::Attending { told };
        Outcome::Keep
    }
}
```

⛔ **E le operazioni, una per riga della §5.** Ancora nello stesso `impl`:

```rust
impl<I: Ipc, J: Journal, C: Custody> Core<I, J, C> {
    /// An invocation, on either of the two roads of sequence 3.
    fn run(&mut self, id: ClientId, call: Call, approval: Approval) -> Outcome {
        let Some(policy) = policy_named(&call.argument) else {
            // ⚠️ AN ARGUMENT THAT NAMES NO POLICY IS REFUSED WITHOUT WRITING ANYTHING, exactly as
            // an unregistered name is: it is text the peer chose, and untrusted content informs,
            // it never authorises (ADR-0014).
            return Outcome::Keep;
        };

        // ⛔ DESTRUCTURED AND NOT REACHED THROUGH THE ACCESSORS, and it is forced rather than
        // tidy: the effect closure needs `&mut arbiter` while `invoke` is holding `&registry` and
        // `&mut journal`. Three methods on `&mut self` cannot be alive at once; three field
        // bindings can.
        let Core { registry, journal, arbiter, steps, .. } = self;
        // ⚠️ BOTH NUMBERS ARE TAKEN UP FRONT, and a refused invocation therefore burns two. That is
        // not a defect: ADR-0036 retires INDICES and never reuses them, and says nothing of the
        // sort about step numbers, which are a progressive and not a schema.
        let step_a = StepId::new(steps.take());
        let step_b = StepId::new(steps.take());
        let outcome = registry.invoke(
            journal,
            step_a,
            &call.function,
            Invoker::Gui(id),
            call.argument.as_bytes(),
            approval,
            |journal| arbiter.set_policy(policy, step_b, journal),
        );

        match outcome {
            Ok(()) => {
                // ⚠️ TWO STATEMENTS AND NOT ONE, deliberately: `self.tell(…, self.policy_report())`
                // asks the compiler for a two-phase borrow, which works and is the kind of line a
                // later edit turns into `E0502` for no reason anybody can see.
                let report = self.policy_report();
                if self.tell(id, &IpcMessage::Policy(report)) == Outcome::Forget {
                    return Outcome::Forget;
                }
                match self.step_list() {
                    Some(steps) => self.tell(id, &IpcMessage::Steps(steps)),
                    None => Outcome::Keep,
                }
            }
            Err(InvokeError::PermissionRequired(permission)) => {
                self.tell(id, &IpcMessage::PermissionRequired(triple_of(permission)))
            }
            // ⚠️ THE OTHER THREE SAY NOTHING, and each for its own reason. `NotRegistered` is a
            // name the peer chose and is refused in silence, like the argument above. `Permission`
            // and `Journal` are the archive failing to answer or to write: there is no variant on
            // this wire that says "the core could not read its own journal", and inventing one
            // here would be a schema decision taken in a dispatch. ⛔ REGISTERED AND NOT TAKEN,
            // with its closer: the first module that has to show a core in trouble -- the Status
            // tile of sub-project 6.
            Err(InvokeError::NotRegistered)
            | Err(InvokeError::Permission(_))
            | Err(InvokeError::Journal(_)) => Outcome::Keep,
        }
    }

    /// The approval road: the triple comes back from the peer, and it is COMPARED, never believed.
    fn approve(&mut self, id: ClientId, triple: Triple, call: Call) -> Outcome {
        // ⛔ WHAT DECIDES IS THE TRIPLE THE REGISTRY HOLDS, and this is the consumer the doc of
        // `crate::wire::ipc::Triple` names: the strings that came back are untrusted (ADR-0014), so
        // a core that converted them into a `Permission` would be granting a permission for a
        // triple nobody was ever asked about. It cannot even be done by accident -- `Permission`
        // wants `&'static str` and a `String` off the wire is not one.
        let Some(function) = self.registry.held(&call.function) else {
            return Outcome::Keep;
        };
        if triple != triple_of(function.permission) {
            return Outcome::Keep;
        }
        self.run(id, call, Approval::JustGiven)
    }

    /// `SaveLayout`, and what comes back is what the port HOLDS (decision 13).
    fn keep_layout(&mut self, id: ClientId, package: &[u8]) -> Outcome {
        // ⛔ THE WRITE'S ERROR IS NOT SENT ON. Decision 13 says the core answers with what it holds
        // after every write, so a failed write comes back as the OLD package and the gui sees it by
        // comparing -- no error variant, and no gui that believes a save stuck when it did not.
        let _ = self.custody.keep(CustodyKey::Layout, package);
        let state = self.layout();
        self.tell(id, &IpcMessage::Layout(state))
    }

    /// D23: every turn, only while somebody is attending, and only when it CHANGED.
    fn sweep_degradation(&mut self, now: Monotonic) {
        if self.attending().is_empty() {
            // ⛔ A BOUND AND NOT AN OPTIMISATION: `degradation_now` re-reads the whole journal, and
            // with no gui attending there is nobody to tell. The cost is the one that function
            // declares of itself, and its remedy is the checkpoint `Journal::replay` names.
            return;
        }
        let Some(state) = self.degradation() else {
            return;
        };

        let mut index = 0;
        while index < self.clients.len() {
            let stale = matches!(self.clients[index].stage, Stage::Attending { told } if told != state);
            if !stale {
                index += 1;
                continue;
            }
            let id = self.clients[index].id;
            match self.tell(id, &IpcMessage::Degradation(state)) {
                Outcome::Keep => {
                    self.clients[index].stage = Stage::Attending { told: state };
                    index += 1;
                }
                Outcome::Forget => self.forget(index, now),
            }
        }
    }

    /// A client leaves the table, and every grant it held goes back to the arbiter.
    ///
    /// ⚠️ THE `Result` OF `on_disconnect` IS DROPPED, and it is said rather than hidden: the only
    /// `Err` it can give is `ReleaseError::UnknownGrant`, which means a grant of ANOTHER arbiter --
    /// this core builds one, so it is unreachable here -- and `ClientGrants` keeps the pairs it has
    /// not released, so a later caller with the right arbiter loses none of them.
    fn forget(&mut self, index: usize, now: Monotonic) {
        let client = self.clients.remove(index);
        let _ = self.grants.on_disconnect(client.id, &mut self.arbiter, now);
    }

    /// One message out, and what a failure on the way means.
    fn tell(&mut self, id: ClientId, message: &IpcMessage) -> Outcome {
        // ⚠️ A MESSAGE THAT WILL NOT ENCODE BECOMES AN EMPTY BODY rather than an error, which is
        // `IpcMessage::encode`'s own containment argument; what can still arrive here is a
        // `WireError` from the ENVELOPE. Nothing is sent, and nothing is said about it: there is no
        // variant that means "the core could not speak", and the gui asks for nothing (§6.1.4).
        let Ok(bytes) = message.encode() else {
            return Outcome::Keep;
        };
        match self.ipc.send(id, &bytes) {
            Ok(()) => Outcome::Keep,
            Err(IpcError::Disconnected) => Outcome::Forget,
            // ⚠️ UNREACHABLE ON THIS OPERATION and written rather than guessed at: the port's own
            // doc says `MalformedMessage` belongs to `receive`, because the bytes handed to `send`
            // were produced by the kernel's schema and a malformed one there is a defect of the
            // kernel, not a failure of the port.
            Err(IpcError::MalformedMessage) => Outcome::Keep,
        }
    }

    /// The degradation, or `None` when the archive cannot say.
    fn degradation(&self) -> Option<DegradationReport> {
        degradation_now(&self.arbiter, &self.journal)
            .ok()
            .map(|state| DegradationReport {
                vram_exhausted: state.vram_exhausted,
                routing_degraded: state.routing_degraded,
            })
    }

    /// The policy with the two numbers the kernel actually holds -- D20.
    fn policy_report(&self) -> PolicyReport {
        PolicyReport {
            // ⛔ AN EXHAUSTIVE `match` AND NOT A STRING COMPARISON ON `name()`: a third policy must
            // stop the compiler here, not arrive on the wire as one of these two.
            policy: match self.arbiter.policy() {
                VramPolicy::Remote(_) => PolicyName::Remote,
                VramPolicy::Local(_) => PolicyName::Local,
            },
            allocated: self.arbiter.allocated(),
            total: self.parameters.total_vram(),
        }
    }

    /// What the seventh port holds under the layout key, in its three states (decision 35).
    fn layout(&self) -> LayoutState {
        match self.custody.retrieve(CustodyKey::Layout) {
            Ok(Some(package)) => LayoutState::Package(package),
            Ok(None) => LayoutState::Nothing,
            Err(CustodyError::Unavailable) => LayoutState::Unavailable,
        }
    }

    /// The step list: in milestone 2 these are the registry's invocations.
    ///
    /// ⛔ `None` WHEN THE ARCHIVE CANNOT BE READ, and never a short list. An incomplete list read
    /// as complete is the same silent partial truth `is_granted` and `degradation_now` both refuse
    /// in their own words, arrived at here by the one road nobody guards: a record this build
    /// cannot decode, skipped, would take a step out of the list the gui shows.
    fn step_list(&self) -> Option<Vec<StepSummary>> {
        let entries = self.journal.replay().ok()?;
        let mut list: Vec<StepSummary> = Vec::new();
        for (step, bytes) in entries {
            let Ok(Record::V1(body)) = Record::decode(&bytes) else {
                return None;
            };
            match body.kind() {
                RecordKind::Invocation => {
                    if let Some(Detail::Invocation(detail)) = body.detail() {
                        list.push(StepSummary {
                            step: step.get(),
                            function: String::from(detail.function()),
                            done: false,
                        });
                    }
                }
                // ⚠️ THE OUTCOME OF STEP A, WHICH IS THE STEP THE INVOCATION NOTE SITS ON. Step B's
                // outcome names a step no line of this list carries, so it finds nothing and
                // changes nothing -- which is right, and is why the search is by step and not a
                // count.
                RecordKind::Outcome => {
                    if let Some(line) = list.iter_mut().find(|line| line.step == step.get()) {
                        line.done = true;
                    }
                }
                _ => {}
            }
        }
        Some(list)
    }
}

/// The triple of a permission, on the wire.
///
/// ⛔ AN EXHAUSTIVE `match` AND NOT `Operation::is_write()`, and that is the lesson this repository
/// has already paid for: `is_write` folds EVERY other variant into `false`, so a third operation
/// would cross the wire as a read. The doc of `crate::wire::ipc::Access` records the measurement.
fn triple_of(permission: Permission) -> Triple {
    Triple {
        tool: String::from(permission.tool),
        resource: String::from(permission.resource),
        operation: match permission.operation {
            Operation::Read => Access::Read,
            Operation::Write => Access::Write,
        },
    }
}

/// The policy an untrusted argument names, if it names one.
///
/// ⛔ THE NAMES ARE THE POLICIES' OWN, read through `MakeRoom::name`, and not two literals here:
/// two houses for one pair of strings is what lets them drift, and the gui shows the same words
/// (G15). ⚠️ AND THERE IS NO EXHAUSTIVE `match` ON THE WAY IN -- the input is a `String` the peer
/// chose -- so the closed set lives in this one function and everything else is `None`.
fn policy_named(argument: &str) -> Option<VramPolicy> {
    let remote = VramPolicy::Remote(RemotePolicy);
    let local = VramPolicy::Local(LocalPolicy);
    if argument == remote.name() {
        Some(remote)
    } else if argument == local.name() {
        Some(local)
    } else {
        None
    }
}
```

E in `crates/kernel/src/lib.rs` (**`i/lf w/crlf`**), accanto agli altri moduli — l'ordine è **di arrivo**:

```
pub mod serving;
```

```bash
cargo test --locked -p kernel --test serving 2>&1 | tail -20
bash scripts/gate-deps.sh
```

Atteso: **tutte** le sonde verdi — quante siano lo dice il comando del passo 6, non questa riga (**P-35**) —
e la lista delle dipendenze **non cresciuta**: `serving` non aggiunge nessuna crate.

- [ ] **Passo 8: i richiami datati**

⛔ **Quattro case, e si toccano nello stesso commit** (quinta riga della disciplina dell'audit). Nella §5 del
[disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) — **`i/lf w/crlf`**, quindi
`replace_unique.py` — tre celle della tabella «Il daemon che ascolta»:

| Riga | Che cosa il richiamo dice |
|---|---|
| `Hello` | ⛔ **RICHIAMO DEL \<data\>, compito 7 del piano della parte 2 (D22):** la protezione **non è consegnata**. `Protection` ha una variante sola, e un parametro che può assumere un valore solo è superficie morta dentro `Parameters` — il doc di `total_vram` lo argomenta — pagata su ogni chiamante. L'attività manda `Protection::AsSystemAccount` e scrive l'**innesco** accanto: il giorno che `Protection` guadagna una seconda variante, il valore diventa consegnato |
| `Request` | ⛔ **RICHIAMO DEL \<data\>, compito 7 (D5):** questa riga dice *«`admit` → `Verdict`»*, e il ramo **non chiama `admit`**. Le tre vie sono state esaminate contro il codice (P-1, P-11, P-12) e il perimetro negativo vive sul ramo stesso, in `crates/kernel/src/serving.rs`; la riga 27 delle voci aperte del Traguardo 6 resta aperta **col suo innesco intatto** |
| il **limite di giri** | ⛔ **RICHIAMO DEL \<data\> (D21):** la riga resta vera e **cambia compito**: `EXECUTOR_TURN_LIMIT` vive in `crates/daemon/src/main.rs`, e la sonda che questa cella detta — *«il grafo con la GUI resta vivo oltre centomila giri»* — vuole il grafo con la GUI, cioè il cablaggio. È il compito **8** |

E la riga **2** della tabella *Stato* della [stella polare](../specs/2026-09-07-direzione-gui-design.md):

> ⛔ **RICHIAMO DEL \<data\>, compito 7 del piano della parte 2 (D20):** `Policy` porta **`allocated`** —
> quanto i libri dell'arbitro impegnano, **le due quote permanenti comprese** — e **`total`**, la macchina
> consegnata. Il denominatore *«tutto meno la quota audio e la quota di presentazione»*, che l'ultima colonna
> di questa riga marca già **dedotto**, **non si costruisce**: il kernel non ha i due addendi e il doc di
> `Parameters::total_vram` argomenta di non consegnarli; e sottrarle nasconderebbe due concessioni **con un
> titolare** (ADR-0033), che è *«la sottrazione non è un'esenzione»*, gotcha **#4**, commesso allo strato che
> l'utente guarda. Se il proprietario vorrà il denominatore netto, i due addendi diventano parametri
> consegnati.

⚠️ **`\<data\>` è la data del giorno in cui il compito si esegue**, non una data scritta oggi: un testo
fissato adesso per una sessione futura invecchia.

✅ **Nella tabella della posizione di QUESTO piano il richiamo di D21 è GIÀ SCRITTO**, alla scrittura del
compito e non all'esecuzione — è ciò che D13 fece con la riga 4: le righe **7** e **8** portano il richiamo
del 2026-09-11, e chi esegue non deve toccarle se non per la spunta finale.

- [ ] **Passo 9: il cancello, la posizione, il commit**

- [ ] `bash scripts/gate.sh` → `GATE GREEN`; `gate-deps.sh` verde e la lista **non cresciuta**;
  `gate-attributes.sh` verde
- [ ] `bash scripts/check-docs.sh` → `OK`
- [ ] i fine-riga: i due file nuovi a **zero** CR, e `git ls-files --eol` **invariato** su tutti i modificati
- [ ] `git diff --stat -- crates/kernel/tests/compile_fail/` riletto, e ciò che è cambiato **nominato nel
  commit** — quale `.stderr`, e perché
- [ ] la riga **7** della tabella della posizione a ✅ con la data
- [ ] commit `gui(compito 7, sotto-progetto 2 parte 2): …`, **senza co-autore**, e push

---

## Come si riprende — il diario di questo piano, coi comandi

### La terza chiusura — 2026-09-11: il piano è SCRITTO FINO AL COMPITO 6 di sedici; nessun compito è eseguito

⛔ **DA SAPERE SUBITO.** Niente è a metà: albero pulito, nessuno stash, nessuna operazione git in corso, nessun
server acceso, **nessun codice di prodotto toccato** — `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/
Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla, e `git diff --name-only 68831c7..HEAD`
rende **un solo file**, questo piano. ⛔ **L'ESECUZIONE NON È COMINCIATA:** la tabella della posizione è tutta ⬜,
l'errata è **vuota**, i compiti **7–16 non esistono**. La sessione nuova **scrive**, non esegue.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline 68831c7..HEAD` — li elenca lui, e sono tutti di documenti |
| Codice di prodotto | **non toccato**, coi due comandi in «Da sapere subito» |
| Quanto è scritto | `grep -c '^## Compito' <questo file>`; la tabella della posizione ne elenca **sedici** |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0**; nasce vuota e non resterà vuota |
| Il pre-controllo e le decisioni | `grep -c '^### P-' <questo file>` e `grep -c '^| \*\*D[0-9]' <questo file>` |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN`, **una volta all'apertura e una prima di ciascun commit**; i log datati nello scratchpad (`gate-2026-09-11-sessione3-*.log`); `bash scripts/check-docs.sh` → `OK` prima di ogni commit |
| Fine-riga | questo piano è **LF** nell'indice e nell'albero: `git ls-files --eol <questo file>` → `i/lf w/lf`, e `tr -cd '\r' < <questo file> \| wc -c` → `0`. ⚠️ L'avviso di `git commit` — *«LF will be replaced by CRLF»* — è **innocuo**, rimisurato dopo ogni commit e invariato tutte e tre le volte |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio |
| File temporanei | nessuno nel repository — `git status --porcelain` vuoto; nello scratchpad restano i log del cancello e gli script d'inserimento |
| Debito lasciato | **nessuno non dichiarato**: i dieci compiti che mancano sono la tabella della posizione; le voci aperte stanno nella sezione omonima; le decisioni sono ribaltabili e ciascuna porta il costo |

#### Le decisioni prese scrivendo, oltre a quelle della tabella

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 11 | il compito 5 **riusa** `platform::journal::OpenError` invece di spostarlo o duplicarlo (**D14**) | spostarlo renderebbe false due righe di doc **del kernel** che ne citano il percorso come precedente di forma; duplicarlo darebbe due tipi da tenere in passo | un percorso che si legge male finché non arriva un terzo archivio |
| 12 | la suite della settima porta prende la forma del **giornale** e non quella della suite `ipc` del compito 2 (**D15**), e la testa del file scrive **perché** le due convivono | la §2 della stella polare dice *«come `journal_contract`»*, e la ragione regge alla lettura: `ipc` usa una macro perché ogni crate porta la propria fabbrica, `custody` non ha un pari | un revisore che «uniforma» le due |
| 13 | il limite di **P-29** si **dichiara e si MISURA** invece di aggiungere una seconda chiave o di scrivere un bugiardo che non muore | un limite dichiarato e non misurato è un'ipotesi, e AUD-019 fu chiuso misurando che il bugiardo passava | il compito 5 cresce di un passo |
| 14 | **P-35 si corregge TOGLIENDO il numerale**, non riallineandolo | un numerale in prosa che conta un artefatto è già marcito una volta e marcirà di nuovo alla prima variante | un comando in più nel criterio di chiusura |
| 15 | una correzione a un compito **scritto e non eseguito** va **NEL compito**, non nell'errata — ⛔ **terza volta**: P-23, P-25, P-35 | un'errata è per ciò che un compito **eseguito** ha smentito; qui il piano è ancora in scrittura, e correggere costa due righe mentre scoprirlo eseguendo costa un compito rifatto | niente |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **Un numerale in prosa che conta un artefatto del piano STESSO non ha nessuna guardia, e si trova solo
  CONTANDO.** *«le undici varianti»* ha attraversato **due sessioni** e **otto case**, e nella stessa pagina il
  criterio di chiusura ne pretendeva quattordici di fixture: nessuno dei due lettori ha mai confrontato le due
  cifre, perché rileggere non è contare. 📌 **Chi scrive un compito che ne consuma un altro rilancia l'`awk` sul
  blocco di codice dettato**, invece di credere alla frase che lo introduce.
- ⛔ **Il difetto di un compito si vede preparando quello DOPO, e la quinta domanda girata all'indietro ha pagato
  DUE volte in questa sessione sola** — P-25 (il compito 2) scrivendo il 5, P-35 (il compito 3) preparando il 7.
  Si chiede sempre: *«i compiti PRIMA del mio lasciano false delle righe nei file, o nei conteggi, che io tocco?»*
- ⛔ **Un commento che DICHIARA DI ESSERE UN COMANDO è la forma peggiore di cifra stantia** — P-24, il doc di
  `crates/platform/src/lib.rs`: dice *«the list is not written here as a fixed set … it comes from `grep …`»*, e
  quel `grep` **enumera sette nomi di tratto**. Chi legge si fida perché vede un comando. **Si rilancia.**
- ⛔ **E la cura sbagliata si misura anche lei.** La prima idea per P-24 era aprire il regex: misurato,
  `^impl [A-Za-z_]+ for ` cattura anche `StorageBackend for FileBackend` e `Default for SequentialRng`, che porte
  non sono. Rumore scambiato per copertura, l'errore opposto e altrettanto silenzioso.
- ⛔ **Uno script che inserisce nel piano deve avere la guardia FUORI dal testo che CITA ciò che corregge.** La
  guardia *««undici» non deve più comparire»* è scattata su sé stessa, perché la voce P-35 cita la parola per
  correggerla. ✅ La scrittura atomica ha retto — `os.replace` su un temporaneo, gotcha **#82** — e il file non è
  stato toccato.
- ⛔ **Un heredoc di Bash MANGIA I BACKSLASH, anche corto:** patchare uno script Python con `<<'PY'` ha
  trasformato `text.index("\n", …)` in un a-capo vero, due volte. Uno script si scrive con **Write**, e se va
  patchato si riscrive intero — non si patcha da heredoc.

#### La lista di lettura della sessione nuova — a compito, non tutto

⛔ **Resta quella della prima chiusura, con queste correzioni misurate scrivendo i compiti 5 e 6:**

| Compito | Che cosa si legge |
|---|---|
| **3**, **4**, **5**, **6** | ✅ **SCRITTI.** Si leggono solo se si esegue |
| **7**, **8** — l'attività e il daemon | ⚠️ **CORRETTA:** oltre a quanto la prima chiusura scrive — §5 del 2 **per intero**, le **tre sequenze** di «La GUI dentro», `crates/daemon/src/main.rs` per intero — servono **`crates/kernel/src/executor.rs`** (la forma di un'attività: `spawn`, il `Future`, `TaskState`, e `Parameters` che porta il limite di giri) e le **tre firme** di `crates/kernel/src/ports/ipc.rs` (`accept` rende un `Option` e **non** ha canale d'errore; `send` e `receive` rendono `Result`), più il blocco *Interfaces* del compito 3 per i nomi dei gemelli. ⛔ **E il compito 6 produce ciò che il 7 consuma:** `Registry::invoke` prende l'**effetto come chiusura** (D16), quindi è il **7** che nomina `Arbiter::set_policy` e registra la funzione vera |
| **9** — la campagna DST | invariata |
| gli altri | come la prima chiusura li ha scritti |

⚠️ **Resta obbligatoria la lettura d'apertura di `CLAUDE.md`** — questo file e il compendio — e la **testa di
questo piano**: vincoli globali, posizione, errata, le voci **P**, le decisioni **D**, le voci aperte. ⛔ **Il peso
non si scrive qui:** lo dà lo snippet `tiktoken` di `CLAUDE.md`, e cresce a ogni compito scritto.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la
   testa è `70155f8` o un commit dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** — non i compiti già scritti, se non per
   i nomi che il compito nuovo consuma: il blocco *Interfaces* di ciascuno li porta.
3. `superpowers:writing-plans`: scrivere i compiti **7, 8, 9 …** nell'ordine della tabella della posizione,
   ciascuno col proprio **pre-controllo delle quattro domande** contro il codice di **adesso**, più la quinta
   girata all'indietro. Ogni difetto trovato è una voce **P** in coda, e la decisione che ne discende una riga **D**.
4. ⛔ **Dopo ogni scrittura su questo file**: il controllo delle tabelle spezzate, `tr -cd '\r'` a zero,
   `bash scripts/check-docs.sh` → `OK`, `bash scripts/gate.sh` → `GATE GREEN`, e il commit — **senza co-autore**.
5. Quando i sedici compiti ci sono: la **revisione del piano intero** — copertura dei disegni, segnaposto,
   coerenza dei nomi fra i blocchi *Interfaces*, ⛔ **e ogni CONTEGGIO rilanciato col comando, non riletto**
   (P-35); poi l'esecuzione in una sessione **nuova**, un subagente fresco per compito.
6. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

---

### La seconda chiusura — 2026-09-11: il piano è SCRITTO FINO AL COMPITO 4 di sedici; nessun compito è eseguito

⛔ **QUESTA INTESTAZIONE DICEVA *«FINO AL COMPITO 3 di sedici, e il pre-controllo del 4 è FATTO»*, ed è
CORRETTA e non cancellata — gotcha #31, un'intestazione è ciò che si legge.** Il verbale fu scritto in `e9e2371`
chiudendo la sessione dopo il compito 3; il **proprietario ha chiesto di proseguire** — *«scrivi il compito 4 in
questa, poi fai session-handoff»* — e il compito 4 è stato scritto in `13cef58`. È lo stesso precedente che
`crates/kernel/src/ports/ipc.rs` porta dal 2026-08-31: il capoverso sotto resta un verbale e vale com'è scritto,
l'intestazione no. **Le cifre delle righe qui sotto sono quelle di ADESSO**, rimisurate coi comandi alla chiusura
vera.

⛔ **DA SAPERE SUBITO.** Niente è a metà: albero pulito, nessuno stash, nessuna operazione git in corso, nessun
server acceso, **nessun codice di prodotto toccato** — `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/
Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla, e `git diff --name-only 1330aca..HEAD`
rende **un solo file**, questo piano. ⛔ **L'ESECUZIONE NON È COMINCIATA:** la tabella della posizione è tutta ⬜,
l'errata è **vuota**, i compiti **5–16 non esistono**. La sessione nuova **scrive**, non esegue.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto; `git stash list` vuoto |
| I commit di questa sessione | `git log --oneline 1330aca..HEAD` — li elenca lui, e sono tutti di documenti |
| Codice di prodotto | **non toccato**, coi due comandi in «Da sapere subito» |
| Quanto è scritto | `grep -c '^## Compito' <questo file>` → **4**; la tabella della posizione ne elenca **sedici** |
| L'errata | `awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' <questo file>` → **0**; nasce vuota e non resterà vuota |
| Il pre-controllo | `grep -c '^### P-' <questo file>` → **23**; le decisioni, `grep -c '^| \*\*D[0-9]' <questo file>` → **13** |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN`, quattro volte: all'apertura, prima di ciascuno dei tre commit (log datati nello scratchpad: `gate-2026-09-11-ripresa-apertura.log`, `-compito3-scritto.log`, `-ripresa-chiusura.log`, `-compito4-scritto.log`); `bash scripts/check-docs.sh` → `OK` prima di ogni commit |
| Fine-riga | questo piano è **LF** nell'indice e nell'albero: `git ls-files --eol <questo file>` → `i/lf w/lf`, e `tr -cd '\r' < <questo file> \| wc -c` → `0`. ⚠️ **`git commit` stampa un avviso** *«LF will be replaced by CRLF the next time Git touches it»*: **innocuo**, rimisurato dopo ogni commit e invariato tutte e tre le volte |
| Tabelle spezzate | `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <questo file>` → **niente** |
| Margine del compendio | **invariato**: questa sessione non ha toccato il compendio |
| File temporanei | nessuno nel repository — `git status --porcelain` vuoto; nello scratchpad restano **solo** i quattro log del cancello, gli attrezzi cancellati |
| Debito lasciato | **nessuno non dichiarato**: i dodici compiti che mancano sono la tabella della posizione; le voci aperte stanno nella sezione omonima; le tredici decisioni sono ribaltabili e ciascuna porta il costo |

#### Le decisioni prese scrivendo, oltre alle tredici della tabella

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 5 | la regola dei gemelli (**D11**) è stata **portata al proprietario in A/B** e non presa da sola | tocca un contratto osservabile — il protocollo core ↔ GUI — e il repo dava **due precedenti opposti** (`Mib` contro `Trust`): `anthropic-skills:dev-communication` dice di fermarsi | una domanda in più |
| 6 | il proprietario ha **delegato** — *«scegli secondo decision-principles»* — e la risposta è una **regola**, non un'opzione | scegliere «sempre A» o «sempre B» avrebbe reso arbitrario uno dei due precedenti; la regola dice **quale** proprietà li separa (un tipo chiuso contro un tipo con campi), e resta vera per il prossimo tipo | una riga D da riscrivere |
| 7 | il pre-controllo del **compito 4** è stato fatto e **registrato** prima di scriverlo | il verbale fu scritto per chiudere; il proprietario ha chiesto di proseguire, e il compito 4 è partito **dal pre-controllo già pagato** invece di rileggere gli stessi file | nessuno: le voci P si aggiungono in coda |
| 8 | il **vincolo globale 1** riceve un richiamo datato invece di essere riscritto | è un vincolo approvato, e qui vale la stessa regola degli ADR: si corregge col richiamo, non in silenzio | nessuno |
| 9 | **P-23 si corregge nel compito 2, non nel 4** dove è stata trovata | il compito 2 è quello che dà a `ipc` il chiamante, cioè la **causa**; correggere nel 4 avrebbe messo il rimedio dove si è visto il **sintomo** — è la terza riga della disciplina dell'audit. ⚠️ E si corregge **adesso** e non con una voce d'errata, perché il compito 2 **non è eseguito**: un'errata è per ciò che un compito eseguito ha smentito | due righe da spostare |
| 10 | due voci del pre-controllo dicono di **NON toccare** una riga | *«all six are named in this milestone»* parla del **Traguardo 1** ed è un fatto datato; la riga 1527 della spec resta vera con sette famiglie. Senza quelle due righe scritte, il prossimo censimento le «corregge» per zelo — ed è metà del valore di un censimento | due correzioni sbagliate |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **Una sonda inventata per verificare un documento può essere sbagliata LEI.** Verificando la prima chiusura,
  `awk '… /^\| *[0-9]+ *\|/ …'` sulla tabella della posizione ha reso **0** e sembrava una divergenza: la tabella
  scrive `| **1** |`, con gli asterischi. **Il documento diceva il vero.** Una sonda che non rende nulla si prova
  su una riga che si è **letta**, prima di dichiarare che il documento mente — e vale doppio quando le altre nove
  tornano tutte verdi, perché è lì che una divergenza isolata sembra un ritrovamento.
- ⛔ **Un difetto del compito N si scopre scrivendo il compito N+2**, ed è P-23: il compito 2 lascia false tre
  righe in due file **che non tocca**, e quei due file li apre il **4**. 📌 **Quindi la quarta domanda del
  pre-controllo va girata all'indietro:** non solo *«ciò che il mio compito produce esiste già?»*, ma *«i compiti
  PRIMA del mio lasciano false delle righe nei file che io apro?»*.
- ⛔ **Un commento può VIETARE ciò che il compito deve fare, per un'altra ragione** — P-21, `ports/mod.rs:79`
  vieta di scrivere «sette famiglie». Chi legge una guardia in un commento si chiede **contro che cosa** è scritta
  prima di obbedirle o di ignorarla: qui i due «sette» sono cose diverse, e la guardia va riscritta **coi numeri
  nuovi**, non cancellata.
- ⛔ **Una frase censita si legge INTERA, e il censimento non finisce alla prima casa.** P-22: «le famiglie
  restano sei» vive in **quattro** posti della spec, di cui **tre** da toccare e **uno** che resta vero.
- ⛔ **Il sorgente di una dipendenza batte il ricordo, e questa volta ha deciso una voce:** `EncodeError` di
  `bincode` 2.0.1 è stato **letto** in `~/.cargo/registry/src/*/bincode-2.0.1/src/error.rs`, ed è ciò che ha reso
  P-17 un fatto invece di un sospetto.
- **Gli inserimenti in un file con tabelle si fanno con Python (`newline=""`)** e si verificano **subito** col
  controllo delle tabelle spezzate: ha retto per tre passate, ma la trappola della prima chiusura resta viva.
- **`$TMPDIR` non è impostata**, confermato: il percorso dello scratchpad si scrive per esteso anche per i log.

#### La lista di lettura della sessione nuova — a compito, non tutto

⛔ **Resta quella della prima chiusura, con queste correzioni misurate scrivendo i compiti 3 e 4:**

| Compito | Che cosa si legge |
|---|---|
| **3**, **4** | ✅ **SCRITTI.** Si leggono solo se si esegue |
| **5** — le due implementazioni della settima porta **e la suite** | ⚠️ **CORRETTA:** `crates/platform/src/journal.rs` **per intero** (632 righe: `FileBackend`, `open(path)`, la forma del modulo `redb`) — **al 4 non serviva, al 5 sì**; la **testa** di `crates/kernel/tests/journal_contract.rs` (32 righe: la forma di una suite `include!`-abile, e il precedente del tempo futuro corretto con un richiamo) e `crates/platform/tests/journal_contract_real.rs` **per intero** (è corto, ed è il modello dell'`include!`); il pezzo 3 della §2 della stella polare; la riga della settima porta della §8 del 2, per le cinque sonde che la suite deve tenere |
| **6** — il registro | invariata: §5 del 2, la prima tabella; ADR-0038; `crates/kernel/src/permission.rs`, `record.rs`, `crates/kernel/tests/frozen_bytes.rs`. ⚠️ **Più `Triple`, `Access` e `Call` del compito 3**, che sono i gemelli sul filo di ciò che il registro decide |
| **7, 8** — l'attività e il daemon | invariata, **più le varianti nuove del compito 3**: il blocco *Interfaces* del 3 le porta coi nomi esatti, e la §5 del 2 va letta **contro quelli** |
| gli altri | come la prima chiusura li ha scritti |

⚠️ **Resta obbligatoria la lettura d'apertura di `CLAUDE.md`** — questo file e il compendio — e la **testa di
questo piano**: vincoli globali, posizione, errata, P-1…P-23, le tredici decisioni, le voci aperte. ⛔ **Il peso
non si scrive qui:** lo dà lo snippet `tiktoken` di `CLAUDE.md`, e cresce a ogni compito scritto.

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la
   testa è `13cef58` o un commit dopo.
2. La lettura d'apertura di `CLAUDE.md`, poi **la testa di questo piano** — non i compiti già scritti, se non per
   i nomi che il compito nuovo consuma: il blocco *Interfaces* di ciascuno li porta.
3. `superpowers:writing-plans`: scrivere i compiti **5, 6, 7 …** nell'ordine della tabella della posizione,
   ciascuno col proprio **pre-controllo delle quattro domande** contro il codice di **adesso**, più la quinta
   girata all'indietro (la seconda trappola qui sopra). Ogni difetto trovato è una voce **P** in coda, e la
   decisione che ne discende una riga **D**.
4. ⛔ **Dopo ogni scrittura su questo file**: il controllo delle tabelle spezzate, `tr -cd '\r'` a zero,
   `bash scripts/check-docs.sh` → `OK`, `bash scripts/gate.sh` → `GATE GREEN`, e il commit — **senza co-autore**.
5. Quando i sedici compiti ci sono: la **revisione del piano intero** — copertura dei disegni, segnaposto,
   coerenza dei nomi fra i blocchi *Interfaces*, ogni *Trova* rilanciato — come la parte 1 fece nella terza
   sessione; poi l'esecuzione in una sessione **nuova**, un subagente fresco per compito.
6. Alla chiusura di ogni sessione: questa sezione come diario, la memoria dell'agente, `session-handoff`.

---

### La prima chiusura — 2026-09-11: il piano è SCRITTO FINO AL COMPITO 2 di sedici; nessun compito è eseguito

⛔ **DA SAPERE SUBITO.** Niente è a metà: albero pulito, nessuno stash, nessuna operazione git in corso,
nessun server acceso, **nessun codice di prodotto toccato** — `git diff --stat 42b50d8..HEAD -- crates/
scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` non rende nulla. ⛔ **E
L'ESECUZIONE NON È COMINCIATA:** la tabella della posizione è tutta ⬜, l'errata è **vuota**, e questo
piano **non è ancora finito** — i compiti **3–16 non esistono**. La sessione nuova **scrive**, non esegue.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto; nessuno stash |
| I commit di questa sessione | `git log --oneline 42b50d8..HEAD` — li elenca lui, e sono tutti di documenti: il piano fino al compito 2, questa chiusura, e la riga del piano nella tabella dei piani di `roadmap.md` (la convenzione di D14 della parte 1: la scrive la sessione che scrive il piano). ⛔ Le righe nella §12 del compendio, in `README.md` e in tracciabilità **non** sono qui: sono il **compito 16** |
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
