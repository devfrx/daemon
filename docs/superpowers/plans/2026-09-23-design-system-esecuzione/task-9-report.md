# Il rapporto dell'implementatore — compito 9 del design system, la chiusura

Macchina `zagor`, 2026-09-28, dalle 10:05 alle 10:56. Il mandato: `task-9-dispatch.md`; il compito: `task-9-brief.md`,
letto per intero a blocchi (le righe oltre 2000 caratteri con `sed -n`).

## 1. Lo stato

**`DONE_WITH_CONCERNS`** — commit **`8c47715`**, non pushato (`## main...origin/main [ahead 1]`), albero pulito dopo.

Le preoccupazioni, nessuna che fermi il compito: **tre voci candidate**, E121–E123, tutte nit del testo del piano, con
l'evidenza nella §8; e il **Passo 9**, che è una scelta di giudizio — sei gotcha su ~96 lezioni lette — e che la revisione
deve giudicare: il criterio e la destinazione di ogni lezione stanno nella §5.

## 2. `git show --stat HEAD`

```
commit 8c47715c6bcd248e0314f9639a7b11104cfce5dc
Author: devfrx <zagor2012@icloud.com>
Date:   Mon Sep 28 10:55:38 2026 +0200

    design-system(compito 9): la chiusura -- il sotto-progetto 14 nei documenti, la riga Accessibilità col richiamo, le case dei documenti, e sei gotcha dalle consegne. […]

 docs/COMPENDIO.md                                  | 13 ++-----
 docs/HANDOFF.md                                    |  8 ++++-
 docs/README.md                                     |  2 +-
 docs/archivio/stato-storico.md                     | 21 ++++++++++++
 docs/porta-di-qualita.md                           | 40 ++++++++++++++++++----
 docs/riferimenti.md                                | 40 ++++++++++++++++++++++
 docs/roadmap.md                                    |  6 ++--
 .../specs/2026-09-07-direzione-gui-design.md       |  2 +-
 .../specs/2026-09-22-design-system-design.md       |  8 ++---
 docs/tracciabilita.md                              |  4 ++-
 10 files changed, 118 insertions(+), 26 deletions(-)
```

Nessun `Co-Authored-By` (`git log -1 --format=%B | grep -ci 'co-authored'` → `0`). Il piano e
`docs/archivio/consegna-piano-design-system.md` non sono nel commit (E120). Messaggio da file, `git commit -F`.

## 3. L'avvio

| Verifica | Uscita |
|---|---|
| `git rev-parse --short HEAD` | `13fea0d` |
| `git status --porcelain` | vuoto |
| `node --version` | `v24.19.0` — `engines`: `^22.22.2 \|\| ^24.15.0 \|\| >=26.0.0` |
| `git config --show-origin --get-all core.autocrlf` | `file:C:/Program Files/Git/etc/gitconfig	true` — come il §0 |
| Chrome, dal nome della cartella | `154.0.8037.58` in `/c/Program Files/Google/Chrome/Application/`; il comando esce 2 perché la cartella di `$LOCALAPPDATA` non c'è, come il dispaccio prevede. Mai `--version` |

Gli attrezzi nello scratchpad `…\scratchpad\task9`: `replace_unique.py`, `word_for_word.py` e `dod_suite.py` estratti dal
brief da uno script, parola per parola (`word_for_word.py` uguale, `cmp`, alla copia della cartella del dispaccio); e miei:
`_block.py` (copia un recinto del brief, `<data>` → `2026-09-28`, e dice quanti), `_append_cell.py` (appende a una cella
**prima di ` |`**, E119), `_step7_texts.py`, `_archive6.py`, `perimeter.py` (quello del pre-controllo, con la cartella del
dispaccio come terza casa, E112). Ogni scrittura è passata da `replace_unique.py` o da Python `newline=""` con `os.replace`.

## 4. I Passi 1–10, coi comandi e le uscite vere

### Passo 1 — le misure prima

| Comando | Uscita |
|---|---|
| `git status --porcelain > prima.txt` | 0 byte |
| `git ls-files --eol` sui dodici | tutti `i/lf w/crlf` |
| `tr -cd '\r' \| wc -c` contro `wc -l` | CR uguali alle righe su tutti e dodici (la tabella della §7) |
| `wc -c docs/COMPENDIO.md`; `grep -n '^ceiling=' scripts/check-docs.sh` | `90735`; `356:ceiling=100352` — margine 9617 |
| il comando del riquadro | `✅ 48`, `🔶 53`, `📋 76`, `⚠️ 0`, `❌ 1` |
| `grep -n '^\| Accessibilità \|'` | `253:\| Accessibilità \| ✅ \|` |
| `grep -n '^\| 1[0-9] \|' docs/roadmap.md` | righe 10–13, nessuna 14 |
| il piano nei tre file | `docs/roadmap.md:235` e `docs/COMPENDIO.md:624`; nessuno in `README.md` |
| la tabella dei passi; `grep -n '^run ' scripts/gate.sh` | `7`; nove `run` |
| i gotcha | `gotcha: 133` |
| i commit dei compiti | ventuno, dai compiti 1–8 |
| `B` | `d10d9a5`; `git diff --stat "$B"..HEAD -- crates/ gui/schema/` vuoto |
| la posizione | righe 1–8 ✅, la 9 ⬜ |
| `bash scripts/gate.sh` (il cancello d'apertura, da solo, in background, 10:06:01–10:10:43) | `GATE GREEN.` — i numeri nella §8 |

Il giudizio del proprietario sul dock (controllo 15, **D24**) è scritto: la cella Stato della riga 6 — *«✅ 2026-09-27 — il
Passo 8 col proprietario … lo sguardo del 6bis le ha approvate, e con lui si chiude questa riga»*. Nessuna domanda.

### Passo 2 — `tracciabilita.md`

`ls` dei dieci percorsi della riga: tutti presenti, uscita 0 — e un controllo in più sulle affermazioni: `axe`/`contrastJudged`
in `kit.browser.test.ts` (5) e `frame.browser.test.ts` (4), `onArrow` in `Overview.vue`, `role="status"` in `BaseStatus.vue`,
`forcedColors`/`reducedMotion` in `tokens.browser.test.ts`. La riga e il capoverso del riquadro sostituiti, testo preso dal file.
Dopo: `✅ 47`, `🔶 54`, `📋 76`, `⚠️ 0`, `❌ 1` — uno in meno e uno in più, gli altri uguali (controllo 21).

### Passo 3 — `roadmap.md`

Quattro `replace_unique.py`, testi vecchi presi dal file. Sonde: `grep -c '^\| 14 \| \*\*Design system\*\*'` → `1`;
`grep -c '^\| \*\*Design system (14)'` → `1`; `grep -c 'eseguito il 2026-09-28'` → `1`; righe 10–14 in fila (174–178).
L'unico diagramma del file è il flusso di lavoro, non i sotto-progetti: nulla da aggiungere.

### Passo 4 — `README.md`

La seconda colonna col sotto-progetto 14, il richiamo prima di ` |`. Sonda `grep -c 'il sotto-progetto 14 |'` → `1`.

### Passo 5 — la §12 del compendio

La riga dopo quella del sotto-progetto 2. `91357` byte, tetto `100352`, margine 8995; `check-docs.sh` → `OK — no inconsistencies.`
(`check-docs.sh` su questa macchina dura oltre due minuti: l'ho sempre lanciato in background).

### Passo 6 — la §6, l'intestazione, l'archivio

- `old-6.txt` con l'`awk` del passo: comincia con `⏭️ **IL PROSSIMO PASSO, IN DUE TEMPI. Uno: il DESIGN SYSTEM`, finisce con
  `in sessioni distinte.`, dieci righe; dentro, una volta ciascuna, `Lo stile dal compito 1 del piano del design system:` e
  `**Due: IL SOTTO-PROGETTO 13**`. **Riletto:** ogni riga parla del design system o del 13 — nessuna voce d'altro da tenere.
- Il nuovo è il secondo tempo di prima **parola per parola**, confrontato a macchina: le righe 7–10 di `old-6.txt` con la sola
  testa cambiata, `cmp` uguale al recinto del passo.
- `old-h.txt`: tre righe, da `**Aggiornato il 2026-09-24**` a `§13.`.
- L'archivio: `_archive6.py` — la cornice del passo, poi `old-h`, una riga vuota, `old-6`; i link riscritti uno per uno e
  contati: `superpowers` 4, `archivio` 2, file di `docs/` 0, lasciati 0, ogni destinazione provata da `docs/archivio/`.
- Sonde: `grep -c 'archiviati il 2026-09-28' docs/archivio/stato-storico.md` → `1`; `grep -c 'IN DUE TEMPI'` → `0`;
  `grep -c '^⏭️ \*\*IL PROSSIMO PASSO: IL SOTTO-PROGETTO 13'` → `1`; `90239` byte (quello del pre-controllo), margine 10113;
  `check-docs.sh` → `OK — no inconsistencies.`
- `python word_for_word.py docs/archivio/stato-storico.md old-h.txt old-6.txt; echo $?` → `word for word` due volte, `0`.
- ⚠️ **E117 rimisurata**, prima di scriverne il gotcha #134 — sull'albero, ogni volta con la copia salvata e `cmp` uguale dopo,
  `git status --porcelain` com'era (`_e117_probe.sh`, `_e117_probe2.sh`, log `e117-probe.log`, `e117-probe2.log`):
  - una `)` tolta dove **un'altra `)` segue** sulla riga (l'intestazione archiviata): `check-docs.sh` → **`1 inconsistencies to fix.`**, `word_for_word.py` → `1`;
  - una `)` tolta dove **nessun'altra `)` segue** (la riga del disegno della knowledge base, nel blocco di oggi): `check-docs.sh` → **`OK — no inconsistencies.`**, `word_for_word.py` → `1`.
  - La prima corsa della sonda aveva rifiutato la mutazione (`refused: 2 occurrences`: la stessa riga sta anche nel puntatore archiviato il 2026-09-22) e non ha misurato niente: rifatta su un'ancora unica, e la sonda ora si ferma se la mutazione non avviene. È la **E121** candidata.

### Passo 7 — `porta-di-qualita.md`

Verificato prima: i sette `gui:` di `gate-gui.sh`, il commento sopra i due `npm test` (E10), `rm -rf dist` (E31), le due
guardie della pagina kit, e in `gui/vite.config.ts` `ignoreDefaultArgs: ["--hide-scrollbars"]` col suo commento.
La sezione prima di `## Le contraddizioni registrate, e non risolte`; le righe 5–7 e il segno sostituiti dalle cinque righe —
`_step7_texts.py` rifiuta se la seconda e la terza colonna delle righe che c'erano (6, 8, 9) non sono quelle del file: uguali —;
C-S0-1 col ✅ prima di ` |`. Sonde:

| Sonda | Uscita |
|---|---|
| le etichette della tabella | `workspace build`, `example and compile-fail tests`, `no-OS gate`, `allow-list on the two graphs`, `dependency advisories`, `attributes of the constrained crates`, `gui: fake core and SPA`, `documentation consistency`, `DST campaigns -- wall time` — l'ordine dei nove `run` (righe 39, 40, 41, 42, 68, 70, 83, 84, 135) |
| `grep -c '⚠️ \*\*\[C-S0-1\]\*\*'` | `0` |
| `grep -c 'IL BROWSER DEI TEST E LA PAGINA KIT'` | `2` |
| `git diff -U0 \| grep '^-[^-]'` | le tre righe vecchie, la riga del segno, la riga C-S0-1 com'era — nient'altro; con `-U1` si vede tolta anche **una** riga vuota, delle due |

### Passo 8 — `riferimenti.md`

La sezione prima di `## Cosa NON abbiamo adottato, e perché` (riga 2849; la seguente ora a 2889). **Ogni fonte riletta contro
la riga che nomina** — le righe P-6, P-7, P-8, P-12, P-15, P-17, P-18, P-19, P-21, P-22, P-23 del piano (lette una per una con
`grep`, non il piano), le fonti del compito 2 (righe 1415–1421 del piano: le otto pagine di Vitest e `index.d.ts`, 2026-09-23),
R2-8, R3-10, R3-22 del registro (datato 2026-09-23 in testa), E4, E7, E10, E24, E43, E46, E85 dell'errata: **nessuna data né
fonte diverge**. Le righe P senza data portano quella della testa della loro tabella, *«una misura del 2026-09-23»*.
Il pezzo JavaScript **prima**: `git diff --quiet 2a30916 "$B" -- gui/; echo $?` → `0`, quindi il valore del 2026-09-24,
`663.26 kB`, compresso `201.23 kB` — letto nel rapporto del compito 1 (`task-1-report.md`, righe 35 e 138, tracciato), nessuna
copia costruita; **dopo**: `698.20 kB`, compresso `213.00 kB`, dal cancello. Nella cella li ho scritti col comando e la provenienza.
⚠️ L'apertura dettata della sezione non dice il vero di nove righe della sua tabella: **E122** candidata, testo lasciato com'è.

### Passo 9 — `HANDOFF.md`: la §5 di questo rapporto

Il comando della §9 del compendio: prima `gotcha: 133`, dopo `gotcha: 139`. L'intestazione *«Aggiornato il»* segue.

### Passo 10 — il disegno e la stella polare

- La riga 5 della posizione, **letta** nel piano: *«alle domande dei punti 3, 4 e 5 — «senti …?» — la risposta «di quello che hai chiesto … funziona tutto»»* → M-3 **chiusa**, il testo del ramo sentito.
- `git diff --name-only "$B"..HEAD -- scripts/` → `scripts/gate-gui.sh`, il solo; e `git diff d10d9a5..HEAD -- scripts/gate-gui.sh` letto: le due corse col commento (E10), `rm -rf dist` (E31), la pagina kit fuori dal pacchetto — ciò che il richiamo dice (E114).
- N-2 di E187 con `663,26` e `698,20` (numeri in prosa, virgola italiana; nella tabella di `riferimenti.md` l'uscita del comando).
- Sonde: `grep -c 'RICHIAMO DEL 2026-09-28, compito 9 del piano del design system'` sulla stella polare → `1`; `grep -c 'dal compito 9 del \[piano\]'` sul disegno → `1`; la riga E228 intatta (nessun `+| **E228**` nel diff).

## 5. Il Passo 9, per esteso (E116)

**Letti:** l'errata intera (E1–E120, nel brief) e le tabelle *«Ciò che questa sessione ha imparato»* che il comando del passo
elenca — **ventitré**, ventidue in `docs/archivio/consegna-piano-design-system.md` e una nel piano (riga 12237): E116 ne contava
ventuno più una, e la ventiduesima è arrivata in archivio con `13fea0d` (`git show HEAD~1:… | grep -c` → `21`). Estratte con
un `awk` in `lessons.txt`, 36 838 byte. E la sezione *«I gotcha»* di `HANDOFF.md`: i titoli dei 133 per intero, e per intero le
righe #101, #112, #115, #127, #131–#133, che le candidate toccano; `grep` delle parole chiave sulle 133 righe (Playwright,
Chrome, `vitest`, `os.replace`, `TaskStop`, `awk`, `check-docs`, `reka-ui`, `dockview`, …) per non scrivere un doppione.

**Il criterio**, deciso coi cinque criteri: una lezione diventa un gotcha se (1) è una trappola — un attrezzo, una libreria,
l'ambiente o un metodo che dà una lettura falsa o un danno in silenzio — per chi lavora **fuori** da questo piano; (2) nessun
gotcha né una regola di `CLAUDE.md` la dice già; (3) oggi vive solo in un verbale d'archivio, non accanto al codice dove ci si
inciampa (un commento nel sorgente, una sezione viva). Lezioni della stessa trappola → **una** riga, come le righe vicine. Il
precedente: la chiusura della parte 2 (`8b0e2e1`) ne scrisse due.

### I sei gotcha scritti, e perché

| # | La trappola | Da dove | Perché è un gotcha |
|---|---|---|---|
| **134** | un rimando che perde la sua `)` esce dal controllo dei link se nessun'altra `)` lo segue sulla riga; un testo archiviato si prova parola per parola | E117; pre-controllo del 9, lezione 1; **rimisurato qui** nelle due direzioni (§4, Passo 6) | ogni chiusura archivia testo parola per parola, e #101 dice *«il cancello lo difende»*: difende le destinazioni che riconosce |
| **135** | un attrezzo fermato lascia un processo o dei file: `TaskStop` e il `node` di `npm run dev`, il compagno visivo acceso e la consegna che lo dice spento, `.superpowers/brainstorm/` non ignorata (**rimisurato qui**: `git check-ignore -v` non risponde su `zagor`), `os.replace` *«Accesso negato»* col server che legge | esecuzione 4 (3), esecuzione 5 (5), Passo 8 del 6 ripreso (5), disegno delle tre voci a metà (5) e scritto (5) | cinque consegne, e nessuna casa viva; il cancello e `npm ci` sono il #133 senza un secondo cancello; il prossimo passo del compendio è un brainstorming, col compagno |
| **136** | gli attrezzi con cui si guarda non vedono ciò che vede l'utente: `--hide-scrollbars` (**rimisurato qui**: `grep -c` → 2 in `playwright-core` 1.63.0), il pannello del browser dell'app nascosto o coperto che non disegna, 0 × 0, densità 1,75, niente `file://`, la pagina coi token aperta come file | esecuzione 1 (3), esecuzione 4 (4), esecuzione 5 (1, 3), tre voci a metà (5), sguardo sulle tre voci (3, 4), pre-controllo 6bis (4), pre-controllo 9 (4); E43, E48, E118 | la trappola più ricorrente del piano, sette consegne; D26 cura le prove, non chi guarda |
| **137** | il Chrome del cancello non è appuntato — si aggiorna da sé fra due corse — e `chrome.exe --version` apre il browser | esecuzione 3 (4, 5) | dal sotto-progetto 14 il cancello dipende dal Chrome installato, fuori da ogni lockfile |
| **138** | `print x > 0` in `awk` scrive un file `0` | pre-controllo 9 (3) | un `awk` di sonda che tace e un file nato nel repository; famiglia del #111 |
| **139** | una violazione provata sul solo file che l'Atteso nomina non vede le cadute altrove | E12, E49, E77, E95; pre-controllo 7 (2), pre-controllo 8 (2) | quattro voci d'errata in un piano solo; ogni piano del repository ha tabelle di violazioni |

In #139 avevo scritto *«la forma di `violations7.py`»*: lo script sta in uno scratchpad della macchina `Jays`, **non tracciato**
(`consegna-piano-design-system.md`, riga 2220), e l'ho tolto — la regola scritta per intero, `npx vitest run` intero.

### Le proposte per `CLAUDE.md` — non scritte: al proprietario, per il «Come si riprende» della chiusura

| | La proposta | Da quali consegne | Dove andrebbe |
|---|---|---|---|
| **P-A** | un passo che solo il proprietario può fare — lo sguardo, il lettore di schermo — **e le celle che ne dipendono** non stanno nel commit di un subagente: il compito si scrive in due commit | pre-controllo del 5 (2), pre-controllo del 6bis (3); **E35, E69, E101, E120** — quattro volte in questo piano | *«Prima di eseguire un compito di un piano»*, una riga dopo la 8 |
| **P-B** | una scelta tecnica reversibile, di poche righe e fuori dal merito approvato, si decide coi cinque criteri e la voce d'errata dice il perché e la via scartata; l'A/B resta al merito | pre-controllo del 4 (4): il proprietario rifiutò l'A/B di **E24** e lo delegò; la forma di E35, E69, E101, E116, E120 | *«Come si lavora qui»*, accanto alla riga delle domande al proprietario |
| **P-C** | il silenzio del proprietario non è un'approvazione: un difetto detto prima dello sguardo e non commentato si scrive aperto, o si dispone col merito che c'è | esecuzione del 6bis (3); E74, E110, **D24** | *«Come si lavora qui»* |
| **P-D** | il pre-controllo rifà il compito **dal testo**, i passi in fila sulla stessa copia — un Atteso si misura sulla sequenza, non su una corsa pulita —, un compito inserito si prova sotto quelli dopo, e il confronto col dettato lo fa uno script provato in due direzioni | pre-controllo del 4 (1), piano della cura (1), pre-controllo del 6bis (1), esecuzione del 6bis (2), pre-controllo dell'8 (4), esecuzione dell'1 (1); `plan_ops.py` nella cartella del dispaccio | *«Prima di eseguire un compito di un piano»* |
| **P-E** | una promessa scritta in una consegna è un contratto per il compito che nomina: il pre-controllo la cerca nelle consegne come nell'errata | pre-controllo del 9 (2); **E116**; allarga il gotcha #132 | la stessa sezione |
| **P-F** | il **curatore** — un subagente sulle decisioni del coordinatore, quando la revisione porta molte cure — è un'opzione da dire al proprietario col suo costo (~0,55 milioni al compito 8) | esecuzione dell'8 (4) | la riga di `superpowers:subagent-driven-development` |

### Dove va ogni altra lezione — resta verbale della sua consegna

Il numero è quello della riga nella sua tabella; «V» è verbale, col motivo in una parola.

| Consegna | Lezioni |
|---|---|
| esecuzione 1, 09-24 | 1 → P-D · 2 V (minore) · 3 → #136 · 4 V (regola 8 di `CLAUDE.md`) · 5 V (E72, la regola dei fine-riga) |
| pre-controllo 2, 09-24 | 1 V (difesa sul posto: il commento di `gate-gui.sh` e la sezione nuova di `porta-di-qualita.md`) · 2 V (#17, #24) · 3 V (#31) · 4 V (regola 8 di *«Come si esegue un compito»*) |
| esecuzione 2, 09-25 | 1 V (la forma già nel modello) · 2 V (#14, #48) · 3 V (#113, trappola 1 del disegno) · 4 V (innocuo) · 5 V (*«Leggere la CI da terra»*) |
| pre-controllo 3, 09-25 | 1 V (domanda 2) · 2 V (#58) · 3 V (#24) |
| esecuzione 3, 09-25 | 1 V (la (f)) · 2 V (#24) · 3 V (#128) · 4, 5 → #137 · 6 V (attrezzi dei subagenti) |
| pre-controllo 4, 09-25 | 1 → P-D · 2 V (difesa sul posto: lo smontaggio in `afterEach`) · 3 V (#54, #74) · 4 → P-B |
| esecuzione 4, 09-26 | 1 V (i commenti delle sonde) · 2 V (#14, #113) · 3 → #135 · 4 → #136 · 5 V (commento in `settings.browser.test.ts`, P-19) |
| pre-controllo 5, 09-26 | 1 V (#55) · 2 → P-A · 3 V (P-19) · 4 V (E37, nel codice) |
| esecuzione 5, 09-26 | 1, 3 → #136 · 2 V (commento della cura E43 nel codice) · 4 V (regola 8) · 5 → #135 |
| pre-controllo 6, 09-26 | 1 V (#75) · 2 V (del piano) · 3 V (regola 5) · 4 V (commento di `probes.ts`, E46) · 5 V (E47, nella prova) |
| esecuzione 6, 09-26 | 1 V (#112) · 2 V (E55, nella prova) · 3 V (minore, di scrittura delle prove) · 4 V (la prova la legge già come parola) · 5 V (del design system) |
| Passo 8 del 6 ripreso, 09-26 | 1 V (#75) · 2 V (la (f), richiamo E61) · 3, 4 V (del design system) · 5 → #135 |
| tre voci, a metà, 09-27 | 1–4 V (#15, #31, E59) · 5 → #135 e #136 |
| tre voci, scritto, 09-27 | 1, 2 V · 3 V (#68) · 4 V (#31) · 5 → #135 |
| sguardo sulle tre voci, 09-27 | 1, 2 V (l'innesco è nella regola, col commento) · 3, 4 → #136 · 5 V (#68) |
| piano della cura 6bis, 09-27 | 1 → P-D · 2, 4 V (del design system) · 3 V (D26) · 5 V (#111; `rev` assente) |
| pre-controllo 6bis, 09-27 | 1 → P-D · 2 V (#68) · 3 → P-A · 4 → #136 · 5 V (E69) |
| esecuzione 6bis, 09-27 | 1 V (#75) · 2 → P-D · 3 → P-C · 4 V (dato per P-20/P-21) |
| pre-controllo 7, 09-27 | 1 V (E75, nel codice) · 2 → #139 · 3 V (regola 5) · 4 V (#69) |
| esecuzione 7, 09-27 | 1 V (#107, la stessa radice dall'altro lato) · 2 V (domanda 3) · 3 V (#17, #55) · 4 V · 5 V (bande di costo: memoria dell'agente) |
| pre-controllo 8, 09-27 | 1 V (#70, #109) · 2 → #139 · 3 V (regola 5, #53) · 4 → P-D |
| esecuzione 8, 09-27 | 1 V (del piano) · 2 V (#15) · 3 V (commento in `dock.ts`, *«SHOWING RESETS THE BASELINE»*) · 4 → P-F |
| pre-controllo 9, 09-28 (nel piano) | 1 → #134 · 2 → P-E · 3 → #138 · 4 → #136 (la prima metà sta nel disegno, E118) |

## 6. La Definizione di «fatto», eseguita blocco per blocco

Sull'albero coi Passi 2–10 applicati, prima del commit; le uscite **non** sono nel piano (E120).

**La base.** `git rev-parse --short "$B"` → `d10d9a5`; `git log --oneline "$B"..HEAD | wc -l` → `49` (a `13fea0d`).

**Blocco 1** (cancello 10:37:56–10:43:26, `gate-dod.log`):

| Comando | Attesa | Uscita |
|---|---|---|
| `tail -1 gate-dod.log` | `GATE GREEN.` | `GATE GREEN.` |
| `grep -c 'gui: fake core and SPA'` | `1` | `1` |
| `grep -E 'assets/index-.*\.js '` | la cifra | `dist/assets/index-DhIQaRiJ.js   698.20 kB │ gzip: 213.00 kB` |
| `bash scripts/check-docs.sh \| tail -1` | `OK — no inconsistencies.` | `OK — no inconsistencies.` |

**Blocco 2** (10:45:35–10:48:14; Chrome `154.0.8037.58` prima e dopo; `--outputFile` in forma `C:/…` e l'uscita di ogni corsa in
`dod-N.out` invece di `/dev/null`): ogni corsa `exit=0`.

```
src/a11y.test.ts: 13 passed, 0 failed, 0 skipped
src/components/kit.test.ts: 31 passed, 0 failed, 0 skipped
src/components/markdown.test.ts: 5 passed, 0 failed, 0 skipped
src/frame/bigtab.test.ts: 3 passed, 0 failed, 0 skipped
src/frame/dock.browser.test.ts: 16 passed, 0 failed, 0 skipped
src/frame/frame.browser.test.ts: 11 passed, 0 failed, 0 skipped
src/frame/frame.test.ts: 24 passed, 0 failed, 0 skipped
src/frame/keys.test.ts: 5 passed, 0 failed, 0 skipped
src/frame/nearest.test.ts: 3 passed, 0 failed, 0 skipped
src/frame/schematic.test.ts: 3 passed, 0 failed, 0 skipped
src/kit/kit.browser.test.ts: 22 passed, 0 failed, 0 skipped
src/locales/copy.test.ts: 4 passed, 0 failed, 0 skipped
src/panels/chat.test.ts: 6 passed, 0 failed, 0 skipped
src/panels/modules.test.ts: 17 passed, 0 failed, 0 skipped
src/panels/settings.browser.test.ts: 1 passed, 0 failed, 0 skipped
src/panels/views/generate-views.test.ts: 0 passed, 0 failed, 1 skipped
src/schema/schema.test.ts: 5 passed, 0 failed, 0 skipped
src/stores/invoke.test.ts: 4 passed, 0 failed, 0 skipped
src/stores/stores.test.ts: 21 passed, 0 failed, 0 skipped
src/stores/stream.test.ts: 5 passed, 0 failed, 0 skipped
src/testing/probes.browser.test.ts: 9 passed, 0 failed, 0 skipped
src/tokens/board.test.ts: 2 passed, 0 failed, 0 skipped
src/tokens/contrast.test.ts: 7 passed, 0 failed, 0 skipped
src/tokens/dock.test.ts: 2 passed, 0 failed, 0 skipped
src/tokens/theme.test.ts: 3 passed, 0 failed, 0 skipped
src/tokens/tokens.browser.test.ts: 7 passed, 0 failed, 0 skipped
src/tokens/usage.test.ts: 3 passed, 0 failed, 0 skipped
src/transport/fakeBridge.test.ts: 4 passed, 0 failed, 0 skipped
dod-1.json: 237 tests, 236 passed, 0 failed, success=True
dod-2.json: 237 tests, 236 passed, 0 failed, success=True
dod-3.json: 237 tests, 236 passed, 0 failed, success=True
dod-4.json: 237 tests, 236 passed, 0 failed, success=True
dod-5.json: 237 tests, 236 passed, 0 failed, success=True
exit=0
```

`dod_suite.py dod-1.json | grep -c ' skipped$'` → `28`; `git ls-files 'gui/src/*.test.ts' | wc -l` → `28`. I dodici file nuovi
del piano ci sono tutti. Nessuna caduta nelle cinque corse. Come il pre-controllo.

**Blocco 3:**

| Riga | Uscita | Attesa |
|---|---|---|
| 1 | `gui/src/tokens/base.css`, `gui/src/tokens/themes.css` | i due file ✓ |
| 4, 5 | `0` | `0` ✓ |
| 6, 18 | `3` | `3` (E111) ✓ |
| 7 | `3` | almeno 1 ✓ |
| 10 | `gui/src/components/icons.ts` | e nient'altro ✓ |
| 11 | `BaseButton`, `BaseDialog`, `BaseIcon`, `BaseLabel`, `BaseList`, `BaseNotice`, `BaseRadioGroup`, `BaseStatus`, `BaseTextField` (`.vue`) | le nove righe della tabella della (b), lette nel disegno: nessuno in più, nessuno in meno ✓ |
| 12 | `6` | `6` ✓ |
| 13 | `✅ 2026-09-26 — il Passo 8 col proprietario, nel suo Chrome con l'Assistente vocale: …` | ✓ |
| 14 | `1`; `ls: cannot access 'dist/kit.html': No such file or directory`; `0` | ✓ |
| 15 | `0` | ✓ |
| 17 | `4` | almeno 1 ✓ |
| 20 | `dock.browser.test.ts 10`, `frame.browser.test.ts 13`, `kit.browser.test.ts 16`, `settings.browser.test.ts 1`, `probes.browser.test.ts 1`, `tokens.browser.test.ts 5` | sei file, ciascuno almeno 1 ✓ |
| 21 | `1` | `1` ✓ |
| 22 | `1` | `1` ✓ |
| 23 | `3` | `3` ✓ |

**Blocco 4:**

| Comando | Uscita |
|---|---|
| `git diff --stat "$B"..HEAD -- crates/ gui/schema/` | niente (prima e dopo il commit) |
| `git ls-files gui/src/tokens/tokens.css` | niente |
| le versioni | `lucide` 1.47.0, geist 5.3.0, barlow 5.3.0, `@vitest/browser-playwright` 4.1.11, `playwright` 1.63.0, `reka-ui` 2.10.4, `vitest` 4.1.11 — esatte |
| il margine | `90239` contro `100352` |
| ADR nuovi | `0` |
| `ls docs/adr/*.md \| wc -l`; `grep -c '^\*\*00' docs/COMPENDIO.md` | `39`; `39` |
| `git diff --name-only "$B"..HEAD` | a `13fea0d` **144** nomi: 83 in una lista *Files*, 61 nella cartella del dispaccio, nessuno per sola coda del nome o sola errata, **0 senza casa**; a `8c47715` **151**: 90 e 61, 0 senza casa |
| `git status --porcelain` | prima del commit i dieci file del compito; dopo, **niente** — **E123** candidata |

**Blocco 5:** l'aspetto non l'ho guardato: non è del compito 9, e il giudizio sul dock è scritto (riga 6, e
`grep -n 'controllo 15'` sul piano: E48, E50, E74, E107, E110, D24, e le righe 6457, 8066, 11585, 11810, 11992, 12179 e
12211 — il compito 6, il 9 e il *«Come si riprende»*); M-3: il
verbale della riga 5 e, da oggi, l'esito nella (e); l'assunzione della barra: il richiamo del 2026-09-28 c'è nel disegno
(`grep -c` → `1`); la CI del commit è del coordinatore, dopo il push; le voci che il piano sa restano nella loro tabella.

## 7. I fine-riga

| File | Prima | Dopo |
|---|---|---|
| `docs/tracciabilita.md` | `i/lf w/crlf`, CR 354 = righe 354 | `i/lf w/crlf`, CR 356 = righe 356 |
| `docs/roadmap.md` | `i/lf w/crlf`, 342 = 342 | `i/lf w/crlf`, 344 = 344 |
| `docs/README.md` | `i/lf w/crlf`, 226 = 226 | `i/lf w/crlf`, 226 = 226 |
| `docs/COMPENDIO.md` | `i/lf w/crlf`, 911 = 911 | `i/lf w/crlf`, 904 = 904 |
| `docs/archivio/stato-storico.md` | `i/lf w/crlf`, 2479 = 2479 | `i/lf w/crlf`, 2500 = 2500 |
| `docs/porta-di-qualita.md` | `i/lf w/crlf`, 2539 = 2539 | `i/lf w/crlf`, 2567 = 2567 |
| `docs/riferimenti.md` | `i/lf w/crlf`, 2870 = 2870 | `i/lf w/crlf`, 2910 = 2910 |
| `docs/HANDOFF.md` | `i/lf w/crlf`, 1400 = 1400 | `i/lf w/crlf`, 1406 = 1406 |
| `…/specs/2026-09-22-design-system-design.md` | `i/lf w/crlf`, 778 = 778 | `i/lf w/crlf`, 778 = 778 |
| `…/specs/2026-09-07-direzione-gui-design.md` | `i/lf w/crlf`, 1081 = 1081 | `i/lf w/crlf`, 1081 = 1081 |
| il piano e l'archivio delle consegne, non toccati | `i/lf w/crlf`, 12253 e 2535 | uguali |

`git ls-files --eol` sui dodici, dopo il commit, identico a quello del Passo 1 (`diff` vuoto); nessun `.tmp` rimasto.

## 8. Il cancello

| Corsa | Esito | jsdom | browser | pezzo JavaScript | avvisi |
|---|---|---|---|---|---|
| d'apertura, `13fea0d`, 10:06–10:10 | `GATE GREEN.` | `Test Files  21 passed \| 1 skipped (22)`, `Tests  170 passed \| 1 skipped (171)` | `Test Files  6 passed (6)`, `Tests  66 passed (66)` | `698.20 kB │ gzip: 213.00 kB` | `found 0 vulnerabilities` |
| blocco 1 della Definizione, 10:37–10:43 | `GATE GREEN.` | uguali | uguali | uguale | uguale |
| Passo 13, prima del commit, 10:48–10:54, dopo `check-docs.sh` → `OK — no inconsistencies.` | `GATE GREEN.` | uguali | uguali | uguale | uguale |

Le verifiche del Passo 13: margine 10113; fine-riga come nella §7; `git diff --stat` coi soli dieci file del punto 2 del
contratto; `git diff --stat -- crates/ gui/ scripts/ .github/ docs/adr/` vuoto; i segnaposto `<data>|<prima>|<dopo>|<Enn>|<i due
valori` fuori dal piano: `0`; `git status --porcelain | diff prima.txt -` → i dieci file e nient'altro.

## 9. Le divergenze — voci candidate, e ciò che non ho misurato

**E121** (Nit) — **Passo 6, e E117: *«una riscrittura che perde la `)` di un rimando passa ogni sonda qui sopra, e anche
`check-docs.sh`»* è vero solo se nessun'altra `)` segue sulla stessa riga.** Il controllo «internal links» (riga 39 di
`scripts/check-docs.sh`) prende un rimando da `](` alla prima `)` che lo segue sulla riga: senza, non lo riconosce e non lo
guarda; con un'altra, la destinazione arriva fino a quella, non esiste, ed è rosso. Misurato il 2026-09-28 sull'albero, nel blocco
appena archiviato, la copia salvata tornata e `cmp` uguale: `)` tolta nell'intestazione archiviata, con un'altra `)` sulla riga →
`1 inconsistencies to fix.`; `)` tolta dopo il rimando al disegno della knowledge base, ultima `)` della riga → `OK — no
inconsistencies.`; `word_for_word.py` → `1` tutte e due le volte. Nessun Atteso cambia: `word_for_word.py` resta la prova. Il
testo proposto: *«…e anche `check-docs.sh`, quando nessun'altra `)` segue sulla riga»*. Il gotcha #134 porta la forma misurata.

**E122** (Nit) — **Passo 8, l'apertura della sezione di `riferimenti.md`**: *«Le fonti che la scrittura del piano e la sua
revisione hanno letto, il 2026-09-23 e il 2026-09-24»* — ma nove righe su ventuno della tabella delle fonti le hanno portate le
voci d'errata dell'**esecuzione**, lette dal 2026-09-24 al 2026-09-27: E4, E7, E10, E24 (due righe), E43 (due), E46, E85. Il
contratto è cresciuto sotto il testo (la riga 5 di `CLAUDE.md`). **Scritto com'è dettato**, non corretto in silenzio. Il testo
proposto: *«…e la sua revisione hanno letto, il 2026-09-23 e il 2026-09-24, quelle che l'esecuzione e le sue voci d'errata hanno
letto fino al 2026-09-27, e le misure…»*.

**E123** (Nit) — **Definizione di «fatto», blocco 4, `git status --porcelain  # niente`, contro il Passo 11 e D20, *«sull'albero
di lavoro con i Passi 2–10 applicati»***: su quell'albero il comando rende i dieci file del compito (misurato, 10 righe
`M`); *«niente»* vale dopo il commit del Passo 13 (misurato: 0 righe). Il testo proposto: *«niente, dopo il commit del compito»*.

**Osservazioni, non voci di questo piano:**
- La riga *«Aggiornato il»* di `HANDOFF.md` diceva `2026-09-22` dopo il gotcha #133 del 2026-09-23 (`3c2aa5d` non la toccò: la
  malattia di AUD-039, fuori da questo piano). La riga del Passo 9 la porta al 2026-09-28; non ho scritto un verbale di quella
  svista nel file (`CLAUDE.md`: i verbali di correzione vanno in archivio), la dico qui.
- E116 contava *«ventuno»* tabelle in archivio: ora ventidue, per `13fea0d` stesso — il contratto che cresce, non un difetto.

**Non misurato da qui:** la CI di `8c47715` (del coordinatore, dopo il push); nessun aspetto guardato (non del compito); il
*«Come si riprende»* e i Passi 11–12 (del coordinatore, E120).

## 10. I tempi e i log

Dalle 10:05:06 alle 10:56:08, ~51 minuti; i tre cancelli ~16 minuti, le cinque corse ~3. Nello scratchpad
`C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\f919d713-f8d4-41b3-a0d4-c94463b505da\scratchpad\task9\`:
`gate-open-2026-09-28.log` (il cancello d'apertura), `gate-dod.log` e `check-docs-dod.log` (blocco 1), `dod-1.json`…`dod-5.json`
e `dod-N.out` (blocco 2), `s13-check-docs.log` e `s13-gate.log` (Passo 13), `e117-probe.log` ed `e117-probe2.log` (E121),
`lessons.txt` e `gotcha-rows.txt` (il Passo 9), `prima.txt`, `old-6.txt`, `old-h.txt`, `eol-prima.txt`, `eol-dopo.txt`,
`cr-prima.txt`, `cr-dopo.txt`, `commit-msg.txt`, e gli attrezzi. Si possono togliere dopo la revisione.
