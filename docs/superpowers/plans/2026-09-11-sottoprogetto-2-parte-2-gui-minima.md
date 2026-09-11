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
*«`Mib(u64)`, `ComputeClass`, `Preemption`, `Millis(u64)` e varianti unit»*. ⛔ **Le undici varianti del compito 3
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

## Compito 3: lo schema che cresce — le undici varianti, i gemelli del filo, le fixture e il timbro di build

**Files:**
- Modify: `crates/kernel/src/wire/ipc.rs` (**`i/lf w/crlf`**) — i gemelli, le undici varianti, l'insieme canonico, il timbro; i due richiami datati di **P-16** e **P-17**
- Modify: `crates/kernel/tests/ipc_wire.rs` (**`i/lf w/crlf`**) — il controllo delle fixture, il generatore dichiarato, le sonde del timbro e del grafo
- Create: `gui/schema/fixtures/*.bin` — un file per variante, **rigenerabili**
- Create: `gui/schema/fixtures/ipc_v1.map` (**LF**) — la mappa `indice → nome → valore`, e il timbro
- Read: la §4 del [disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md), le due tabelle; la riga «lo schema» della tabella degli artefatti della §8; la **sequenza 1** di «La GUI dentro» nella [stella polare](../specs/2026-09-07-direzione-gui-design.md); `crates/kernel/src/wire/ipc.rs` e `crates/kernel/tests/ipc_wire.rs` **per intero**

**Interfaces:**
- Consumes: `kernel::arbiter::{ComputeClass, Mib, Preemption}`; `kernel::framing::{self, LENGTH_WIDTH, WireError}`; `kernel::time::Millis`
- Produces, e i compiti 7, 8, 10, 11, 12 e 13 li usano con questi nomi esatti:
  - `kernel::wire::ipc::BuildStamp` — `BuildStamp(u64)`, con `BuildStamp::get(&self) -> u64`
  - `kernel::wire::ipc::{Protection, DegradationReport, PolicyReport, PolicyName, Triple, Access, Call, Provenance, LayoutState, StepSummary}`
  - le undici varianti nuove di `kernel::wire::ipc::IpcMessage`
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

⚠️ **Costo dichiarato:** undici varianti portano dieci tipi nuovi in `wire::ipc`, e ogni compito che le riempie
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

- [ ] **Passo 2: i gemelli e le undici varianti**

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct PolicyReport {
    pub policy: PolicyName,
    pub allocatable: Mib,
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
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct StepSummary {
    pub step: u64,
    pub function: String,
    /// `None` while the step is still in doubt -- ADR-0007's intent written and no outcome yet.
    pub outcome: Option<bool>,
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
            allocatable: Mib::new(12288),
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
            outcome: Some(true),
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
git commit -m "gui(compito 3): lo schema che cresce -- le undici varianti di IpcMessage coi gemelli del filo (D11), l'insieme canonico e il timbro di build, le quattordici fixture rigenerabili in gui/schema/fixtures e il controllo che dice rigenera; i richiami datati su P-16 (l'innesco della revoca e' il 7) e P-17 (il grafo di encode riletto)"
git push
```

#### Criterio di chiusura del compito 3

- [ ] `cargo test --locked -p kernel --test ipc_wire` → **dodici passati**, uno ignorato
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

## Come si riprende — il diario di questo piano, coi comandi

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
| **7, 8** — l'attività e il daemon | invariata, **più le undici varianti del compito 3**: il blocco *Interfaces* del 3 le porta coi nomi esatti, e la §5 del 2 va letta **contro quelli** |
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
