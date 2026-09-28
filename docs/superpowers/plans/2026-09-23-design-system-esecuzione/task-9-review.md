# La revisione del compito 9 del design system — la chiusura

Revisore: un subagente fresco, macchina `zagor`, 2026-09-28, dalle 11:01 alle 11:50 circa. Il mandato:
`review-9-prompt.md`. Rivisto **un** commit, `8c47715`, sopra `13fea0d`.

All'avvio: `git rev-parse --short HEAD` → `8c47715`; `git status --porcelain` → vuoto; `git log --oneline 13fea0d..HEAD`
→ una riga. Chrome `154.0.8037.58` dal nome della cartella, Node `v24.19.0`, `core.autocrlf` `true` da
`C:/Program Files/Git/etc/gitconfig`. I miei file e log: `…\scratchpad\review9\`. Il clone: `C:\Users\zagor\AppData\Local\Temp\rv9`,
lasciato a `8c47715`, pulito (i due commit usa-e-getta vi restano come oggetti sciolti). L'albero del repository non l'ho
toccato: `git status --porcelain` → vuoto anche alla fine.

## 1. Il verdetto

**Conforme al dettato** — `compare_task9.py 13fea0d 8c47715` rende OK sui nove documenti ed esce 0, la Definizione di
«fatto» torna blocco per blocco, il contratto è rispettato —, con **0 critici, 0 importanti, 4 minori, 6 nit**. Le tre
candidate del rapporto reggono, e **E121** va corretta prima di entrare nell'errata (M-3). Cinque candidate nuove,
**E124**…**E128**: tre minori del dettato, due nit.

## 2. I rilievi

### Minori

**M-1 — Dettato (E120), candidata E124. Fra il commit dell'implementatore e la chiusura, sei case dicono chiuso ed eseguito
ciò che il piano dice ancora da chiudere: E120 ha curato il piano, non le altre case.**

| Dove (`grep -n` sulla frase) | Che cosa dice a `8c47715` |
|---|---|
| `docs/roadmap.md:178` | la riga 14, *«✅ **chiuso il 2026-09-28** — la Definizione di «fatto» del piano, coi comandi»* |
| `docs/roadmap.md:237`, `:6` | *«eseguito il 2026-09-28 — … la Definizione di «fatto» coi comandi sta nel piano»*; l'intestazione, *«chiusura del sotto-progetto 14»* |
| `docs/README.md:212` | *«il piano è **eseguito**»* |
| `docs/COMPENDIO.md:20`, `:618`, `:830` | l'intestazione, *«chiusura del sotto-progetto 14»*; il ⏭️ al 13; la §12, *«come si è **eseguito**»* |
| `docs/tracciabilita.md:77`, `docs/HANDOFF.md:3` | *«alla chiusura del sotto-progetto 14»*, *«con la chiusura del sotto-progetto 14»* |
| `docs/riferimenti.md:2886` | le cinque corse: *«la Definizione di «fatto» del piano, blocco 2 \| lì, con la data»* |

Contro il piano allo stesso commit: `grep -n 'IL PIANO È SCRITTO il 2026-09-24' <piano>` → `121`;
`awk -F'|' '/^\| \*\*9\*\* \|/{print $5}' <piano>` → ` ⬜ `; `grep -n '^## La Definizione di «fatto»' <piano>` →
`12034:## La Definizione di «fatto» — i comandi, e le uscite attese` — le uscite del giorno non ci sono ancora (E120). È la
malattia che E120 nomina — *«fra i due commit il piano direbbe eseguito un compito non ancora rivisto»* —, spostata nelle
altre case; e il dispaccio dice il push del coordinatore *«dopo la revisione»*: pushato da solo, `8c47715` la pubblica.
L'implementatore ha scritto il dettato: non è un suo difetto.

Testo proposto per **E124**: *«⚠️ Compito 9 — E120 cura il piano e non le altre case: il commit dell'implementatore scrive
«chiuso il <data>» e «eseguito il <data>» (roadmap, `README.md`), «la chiusura del sotto-progetto 14» (le intestazioni del
compendio, di `tracciabilita.md` e di `HANDOFF.md`), il ⏭️ al 13 e, in `riferimenti.md`, «lì, con la data» per le cinque
corse, mentre il piano, fino alla chiusura, dice «IL PIANO È SCRITTO», la riga 9 ⬜ e la Definizione con le sole attese.
La cura, la più leggera: i due commit — quello dell'implementatore e la chiusura — si pushano **insieme**, mai il primo
da solo; se la revisione riapre il compito, la chiusura riscrive quelle celle. Scartato portare quelle scritture nella
chiusura: svuoterebbe il compito 9 e toglierebbe a `compare_task9.py` il testo che prova.»* La scelta è del coordinatore,
coi cinque criteri: è la forma di E120.

**M-2 — Dettato (P-28), candidata E125. Il richiamo sulla riga «codice di prodotto» del disegno dà al piano ciò che il
comando della riga rende anche da prima del piano.**

`docs/superpowers/specs/2026-09-22-design-system-design.md:746`: *«**non toccato**: `git diff --stat ab39f39..HEAD -- . ':!docs'`
non rende nulla ✅ **RICHIAMO DEL 2026-09-28:** toccato dal piano, i compiti 1–8 — che cosa, file per file, lo dice la
Definizione di «fatto» del piano»*. Ma:

```
git diff --stat ab39f39..d10d9a5 -- . ':!docs'      # d10d9a5 = B, prima del primo commit del compito 1
 CLAUDE.md             | 157 ++++++++++++++++++++------------------------------
 scripts/check-docs.sh |  14 ++++-
git log --format='%h %ad' --date=short ab39f39..8c47715 -- CLAUDE.md scripts/check-docs.sh
c4ec042 2026-09-24 · 3c2aa5d 2026-09-23 · eab020d 2026-09-23 · 4a7111d 2026-09-23     # il ridimensionamento e la compressione
git diff --stat ab39f39..2a30916 -- . ':!docs'      # 2a30916, la misura di P-28: gli stessi due file
```

La riga era falsa dal 2026-09-23, non *«dal compito 1»* come dice P-28 (`<piano>:327`), e la Definizione di «fatto» — il suo
perimetro parte da `B` — quei due file non li nomina. Dopo il commit, chi lancia il comando della riga vede `CLAUDE.md` e
`scripts/check-docs.sh`, e il richiamo li dà al piano. Testo proposto per **E125**: in coda al richiamo, *«…; e, prima del
piano, `CLAUDE.md` e `scripts/check-docs.sh`, dal ridimensionamento della lettura del 2026-09-23 e dalla compressione del
compendio del 2026-09-24 — `git log --format='%h %ad' ab39f39..d10d9a5 -- CLAUDE.md scripts/check-docs.sh`»*; e in P-28
*«falsa dal compito 1»* → *«falsa dal 2026-09-23, prima del piano»*. Nessun'altra casa cambia: la riga della stella polare
(`scripts/`) nomina già `scripts/check-docs.sh`.

**M-3 — Testo dell'implementatore (#134) e candidata E121. Un rimando con un'ancora che perde la sua `)` resta verde anche
quando un'altra `)` lo segue sulla riga.**

`docs/HANDOFF.md:1243`, #134: *«… e rosso, `1 inconsistencies to fix.`, dove un'altra `)` la segue — il controllo «internal
links» prende un rimando da `](` alla prima `)` che lo segue sulla riga: senza, il rimando per lui non c'è; con un'altra, la
destinazione arriva fino a quella, e non esiste.»* E il rapporto, E121: *«è vero solo se nessun'altra `)` segue sulla stessa
riga»*. Misurato nel clone, `bash scripts/check-docs.sh`, il file tornato con `git checkout --` dopo ogni caso:

| Caso | Mutazione | Uscita |
|---|---|---|
| A | `)` tolta dopo `](lettura-di-apertura-storico.md` nell'intestazione archiviata; un'altra `)` segue | `1 inconsistencies to fix.` — come il rapporto |
| B | `)` tolta dopo `](../superpowers/specs/2026-09-22-design-system-design.md` nel puntatore archiviato; nessun'altra `)` | `OK — no inconsistencies.` — come il rapporto |
| C | in coda a `docs/roadmap.md`, `[topologia](design/01-topologia-dei-processi.md#canali e poi (una parentesi) qui.` — l'ancora, e un'altra `)` dopo | `OK — no inconsistencies.` (nella stessa corsa di B) |
| C′ | la stessa riga senza `#canali` — la controprova che la riga si legge | `1 inconsistencies to fix.`, `-> design/01-topologia-dei-processi.md e poi (una parentesi` |

Il meccanismo, `scripts/check-docs.sh:39-40`: dopo `grep -o '](\([^)#]*\.md\)[^)]*)'` viene `cut -d'#' -f1`, e ciò che precede
il `#` è il file vero. La lezione del gotcha — la prova è `word_for_word.py` — non cambia; cambia la frase che dice quando il
cancello morde. Testo proposto: in #134, *«con un'altra, la destinazione arriva fino a quella e, tagliata al primo `#`, di
solito non esiste — ma un rimando con un'ancora resta verde anche lì (misurato il 2026-09-28 dalla revisione del compito 9, su
un clone)»*; in **E121**, *«…e anche `check-docs.sh`, quando nessun'altra `)` segue sulla riga, o quando il rimando porta
un'ancora `#…`»*. Il messaggio di `8c47715` dice *«solo se»*: è immutabile, lo dice la chiusura.

**M-4 — Sonda del pre-controllo (E112), candidata E126. Il perimetro del blocco 4 dà casa a ogni file sotto `crates/`,
`gui/`, `scripts/` e `.github/` per la riga del compito 9 che vieta di toccarli.**

`perimeter.py` (del pre-controllo, copiato dall'implementatore) prende i token fra backtick di **ogni** riga delle liste
*Files*, e un token che finisce con `/` è una cartella-casa: anche *«- ⛔ **NON si tocca il codice** — `crates/`, `gui/`,
`scripts/`, `.github/`»* del compito 9. Con la casa stampata per ciascun nome (`perimeter_verbose.py` nello scratchpad), un
nome solo ha per casa quel divieto e nient'altro: `gui/src/panels/Chat.vue <- Files Compito 9 [gui/]`. La sua casa vera è la
riga *Files* del compito 1, *«e gli **undici** componenti che il censimento del passo 1 nomina»* (`<piano>:398-399`), con
`Chat` nell'Atteso del passo (`<piano>:428-429`) — proprio il caso che il ⚠️ del blocco 4 nomina. La conclusione del
rapporto, **0 senza casa**, regge, riletta a mano; ma la sonda è vuota su quattro cartelle, e la chiusura (E120) la rilancia.
E il testo di E112 — *«gli altri 83 hanno ciascuno il proprio percorso in una lista Files»* — ne conta uno di troppo.
Testo proposto per **E126**: la sonda salta le righe *Files* che cominciano con `- ⛔ **NON si tocca` e conosce gli undici
nomi del censimento del compito 1; in E112, *«…gli altri 83 hanno casa in una lista Files: 82 per il percorso, e
`gui/src/panels/Chat.vue` per il censimento del compito 1, che la lista chiama «gli undici componenti»»*.

### Nit

**N-1 — Dettato, candidata E127. La riga `gui: build` della sezione nuova non dice che `dist/` si vuota prima del *build*.**
`docs/porta-di-qualita.md:2469`: *«dopo `npm run build`, rosso se `dist/index.html` manca — la guardia di non-vacuità — …»*;
`grep -n 'rm -rf dist' scripts/gate-gui.sh` → `37`, col commento *«NO OUTPUT OF A PREVIOUS BUILD … (E31…)»*; nella sezione
nuova `E31`, `rm -rf` e *vuotat* → `0`. La specie di **E114** — il testo scritto prima di E31 —, che ha corretto la stella
polare e non questa riga. Testo proposto: *«`dist/` vuotato, poi `npm run build`: rosso se `dist/index.html` manca — la
guardia di non-vacuità, che senza lo svuotamento leggeva un `dist/` rimasto (**E31**) — e rosso se …»*, e nella colonna *Da*
*«compito 4, e la sua cura (**E31**)»*.

**N-2 — Dettato, candidata E128. La riga N-2 di E187 della (e) data al 2026-09-28 un valore del 2026-09-24, e porta le due
cifre in una seconda casa.** `docs/superpowers/specs/2026-09-22-design-system-design.md:454`: *«📌 **Misurato il 2026-09-28**:
il pezzo JavaScript da 663,26 a 698,20 kB»* — il primo è del 2026-09-24 su `B`, come dice `docs/riferimenti.md:2884` e la
regola del Passo 8; e le due cifre vivono anche lì, col comando (`CLAUDE.md`: *«vive in una casa sola»*). Testo proposto:
*«📌 **Misurato**: il pezzo JavaScript **cresce** — i valori, del 2026-09-24 su `B` e del 2026-09-28 su `HEAD`, e il comando in
[`riferimenti.md`](../../riferimenti.md), la sezione del piano —: la deduzione qui a fianco, «non la peggiora», **non regge**;
resta del proprietario»*.

**N-3 — Testo dell'implementatore. Il titolo del gotcha #135 copre due dei suoi quattro casi.** `docs/HANDOFF.md:1244`:
*«UN ATTREZZO FERMATO PUÒ LASCIARE DIETRO DI SÉ UN PROCESSO O DEI FILE»*; ma `.superpowers/brainstorm/` la scrive il compagno
**acceso** (e `git check-ignore -v .superpowers/brainstorm/x.html` → nessuna uscita, `exit=1`, rimisurato), e il temporaneo
di `os.replace` lo lascia una scrittura fallita col server che legge. Titolo proposto: *«UN ATTREZZO — FERMATO A METÀ, RIMASTO
ACCESO, O CON UNA SCRITTURA FALLITA — PUÒ LASCIARE DIETRO DI SÉ UN PROCESSO O DEI FILE, E LI TROVANO DOPO IL CANCELLO O IL
COMMIT.»*

**N-4 — Testo dell'implementatore. Il gotcha #136 dà per misurato ciò che E118 dice dedotto.** `docs/HANDOFF.md:1245`:
*«una pagina di prova coi token aperta come file non carica i moduli di Vite»*; E118: *«aperta come file la pagina non carica
`/src/tokens/index.ts`, dedotto»*. Cura: *«… non carica i moduli di Vite — dedotto (E118) — …»*.

**N-5 — Rapporto. Due cifre che non tornano.** §1, *«sei gotcha su ~96 lezioni lette»*: le lezioni sono **106** —
`grep -cE '^\| [0-9]+ \|'` sul `lessons.txt` dell'implementatore → `106`, sul mio → `106` (102 nelle ventidue tabelle
d'archivio, 4 nel piano); la tabella della §5 le classifica tutte e 106, quindi è la cifra a sbagliare, non la lettura. §4,
Passo 5: *«check-docs.sh su questa macchina dura oltre due minuti»*; i suoi log dicono ~41 s (`s13.start` 10:48:49, `s13-check-docs.log`
10:49:30) e ~56 s (`gate-dod.end` 10:43:26, `check-docs-dod.log` 10:44:22); nel clone 47 s.

**N-6 — Rapporto, §5. Tre «V» senza il perché, e due perché che non sono del criterio.** Il rapporto scrive *«"V" è verbale,
col motivo in una parola»*, e ne mancano tre: *tre voci, scritto* 1 e 2, *esecuzione 7* 4. E *«minore»* non è una ragione
del criterio del rapporto: la lezione 2 dell'*esecuzione 1* vive nella nota di memoria dell'agente (`harness-attrezzi-sdd-e-heredoc.md`,
il server fermato per PID), la 3 dell'*esecuzione 6* nel commento di `gui/src/frame/dock.browser.test.ts:148` — la classe
regge, il perché è un altro. La 2 del *pre-controllo 9* va a **P-E**, mentre è la trappola del gotcha **#132** in un'altra
casa: il criterio del rapporto, *«stessa trappola → una riga»*, la darebbe a quella famiglia; lo dico, non lo curo — le due
strade sono difendibili.

### Le candidate del rapporto

| Voce | Esito |
|---|---|
| **E121** (Nit) | i due casi misurati tornano (A e B qui sopra); il testo proposto è **incompleto**: M-3 |
| **E122** (Nit) | **confermata**: la sezione nuova di `riferimenti.md` ha 21 righe di fonti, e nove — E4, E7, E10, E24 (due), E43 (due), E46, E85 — le ha portate l'esecuzione, lette fino al 2026-09-27; l'apertura dettata le dà alla scrittura e alla revisione |
| **E123** (Nit) | **confermata**: sull'albero coi Passi 2–10 applicati `git status --porcelain` rende i dieci file (il rapporto: 10 righe `M`); *«niente»* vale dopo il commit (`git status --porcelain \| wc -l` → `0`, rimisurato) |

## 3. I comandi rilanciati, con le uscite

| Comando | La mia uscita | Il rapporto |
|---|---|---|
| `bash scripts/gate.sh` sull'albero, da solo, in background, 11:03:20–11:08:25, `gate-review.log` | `GATE GREEN.`; jsdom `Test Files  21 passed \| 1 skipped (22)`, `Tests  170 passed \| 1 skipped (171)`; browser `Test Files  6 passed (6)`, `Tests  66 passed (66)`; `dist/assets/index-DhIQaRiJ.js 698.20 kB │ gzip: 213.00 kB`; `found 0 vulnerabilities`; `OK — no inconsistencies.` | uguali, e uguali al cancello d'apertura del coordinatore, `gate-open-13fea0d.log` |
| `git diff --stat 13fea0d..8c47715 -- gui/ crates/ scripts/` (e da `f35d3c2`) | vuoto | — |
| blocco 2, una corsa: `npx vitest run --reporter=json`, poi `dod_suite.py` copiato dal brief (`cmp` uguale a quello dell'implementatore) | 28 righe, nessun `failed`; `237 tests, 236 passed, 0 failed, success=True`; `exit=0`; `28` e `28`; l'elenco per file **identico** a quello di `dod-1.json` (`diff` vuoto) — nessun file sparito, per nome | uguale |
| `dod_suite.py` sulle cinque corse dell'implementatore | cinque volte `237 tests, 236 passed, 0 failed, success=True`, `exit=0`; i JSON fra 10:46:06 e 10:48:13 | uguale |
| la base: `B`; `git log --oneline "$B"..HEAD \| wc -l` | `d10d9a5`; `50` a `8c47715` (`49` a `13fea0d`) | `d10d9a5`; `49` a `13fea0d` |
| blocco 3, riga per riga (`dod34.sh`) | 1 i due file; 4-5 `0`; 6-18 `3` (e `6` col comando di prima di E111); 7 `3`; 10 `icons.ts` solo; 11 i nove `Base*.vue` nome per nome; 12 `6`; 13 `✅ 2026-09-26 — …`; 14 `1`, `No such file or directory`, `0`; 15 `0`; 17 `4`; 20 dock 10, frame 13, kit 16, settings 1, probes 1, tokens 5; 21 `1`; 22 `1`; 23 `3` | uguale |
| blocco 4 | `crates/`, `gui/schema/` vuoto; `tokens.css` niente; le sette versioni esatte; `90239` contro `100352`; ADR nuovi `0`; `39` e `39`; 151 nomi; `git status` `0` | uguale |
| `perimeter.py <piano> d10d9a5`, a `8c47715` e, nel clone, a `13fea0d` | `151`: Files 90, dispatch 61, homeless 0; `144`: 83, 61, 0 | uguale — ma M-4 |
| il Passo 1 sui file di `13fea0d` (`git show`) | ✅ 48, 🔶 53, 📋 76, ⚠️ 0, ❌ 1; `253:\| Accessibilità \| ✅`; righe 10–13; il piano a `roadmap.md:235` e `COMPENDIO.md:624`; tabella 7 righe e 9 `run`; `gotcha: 133`; 21 commit dei compiti (3, 2, 2, 2, 3, 2, 2 per 6bis, 2, 3); righe 1–8 ✅, la 9 ⬜ | uguale |
| le sonde dei Passi 2–10 su `8c47715` | ✅ 47, 🔶 54; le righe 10–14 a 174–178; Passo 3 `1`, `1`, `1`; Passo 4 `1`; Passo 6 `1`, `0`, `1`; Passo 7 le nove etichette nell'ordine dei nove `run`, `0`, `2`, le cinque righe tolte e nient'altro; Passo 10 `1` e `1`; `git diff --name-only "$B"..HEAD -- scripts/` → `scripts/gate-gui.sh`; `git diff --quiet 2a30916 "$B" -- gui/` → `0` | uguale |
| il compendio: `wc -c`, e il Passo 5 ricostruito in Python dalla base | `90239`, margine `10113`; dopo il solo Passo 5 `91357`, margine `8995` | uguale |
| `task-1-report.md`, righe 35 e 138 | `663.26 kB │ gzip: 201.23 kB` al Passo 1 del compito 1 | uguale |
| `bash scripts/check-docs.sh`, da solo | `OK — no inconsistencies.` | uguale |
| il comando della §9 del compendio; la serie | `139`; numeri da 1 a 139 contigui | `139` |
| fine-riga: `git ls-files --eol` e CR contro righe sui dodici file, a `13fea0d` (nel clone) e a `8c47715` (sull'albero) | tutti `i/lf w/crlf` prima e dopo; CR uguali alle righe: 354→356, 342→344, 226, 911→904, 2479→2500, 2539→2567, 2870→2910, 1400→1406, 778, 1081; piano 12253 e archivio 2535 invariati | uguale |
| le tabelle, i due `awk` di 7(c) su ogni file a `13fea0d` e a `8c47715` | le sole occorrenze di prima (compendio 798→790, HANDOFF 1159, riferimenti sei), spostate e non nuove; nessuna riga aggiunta con due spazi o testo attaccato prima di ` \|` (E119) | — |
| i rimandi nelle righe aggiunte (`newlinks.py`) | 42 rimandi, ogni file esiste; nessuna ancora; il solo «mancante» è `](…)` dentro un code span di #134 | — |
| i segnaposto fuori dal piano | `0` | `0` |
| il perimetro del contratto | `git diff --stat 13fea0d..8c47715 -- crates/ gui/ scripts/ .github/ Cargo.toml Cargo.lock` vuoto; nulla in `docs/adr/`, nella spec del sotto-progetto 1, in `docs/superpowers/plans/`, in `CLAUDE.md`; la §5 del compendio intatta — a `13fea0d` va dalla riga 135 alla 580, e gli hunk del commit stanno alle righe 20, 620 e 837; nessun `mode change` | uguale |
| il contratto | dieci file; il messaggio comincia con `design-system(compito 9): `; `co-authored` → `0`; `## main...origin/main [ahead 1]` | uguale |
| i log citati, `ls -la --time-style=full-iso` contro `git log -1 --format=%ci 8c47715` (10:55:38) | `gate-open-2026-09-28.log` 10:10:43, `gate-dod.log` 10:43:26, `check-docs-dod.log` 10:44:22, `s13-check-docs.log` 10:49:30, `s13-gate.log` 10:54:43, `e117-probe*.log` 10:31–10:32: dell'implementatore, dopo le 10:05 e prima del commit; nessuna sovrapposizione fra cancelli e corse | — |
| le fonti nuove di `riferimenti.md`, i comandi scritti accanto | `unexpectedly reloaded` → 1; `passWithNoTests` in `vitest/dist`; `@experimental` sopra `force?: boolean`; `--hide-scrollbars` → 2 in `playwright-core` 1.63.0; `maximizedNode` 17 in `main.esm.mjs` e 0 in `dockviewComponent.d.ts`; le date di P-12, P-21, P-22, P-23, R2-8, R3-10, R3-22 uguali a quelle del piano e del registro | — |
| le affermazioni dei gotcha | #135 `git check-ignore -v .superpowers/brainstorm/x.html` → niente, `exit=1`; #137 `(Get-Item …\chrome.exe).VersionInfo.ProductVersion` → `154.0.8037.58`, nessun processo `chrome`; #138 `echo 5 \| awk '{print $1 > 0}'` → niente, e nasce un file `0` con `5`; #139 E12, E49, E77, E95 riletti | — |

## 4. Le due direzioni

**`compare_task9.py`, letto.** Ricostruisce dal testo del piano a `13fea0d` i nove documenti: i ventidue recinti
```` ```markdown ```` del compito 9, ciascuno col suo prefisso asserito; la data dalla riga 14 della roadmap del commit,
uguale ovunque; poi, file per file, ciò che i Passi 2–8 e 10 dettano — la riga e il riquadro di tracciabilità, i quattro
tocchi della roadmap, la seconda colonna e il richiamo del README, la riga della §12, il puntatore e l'intestazione presi dal
file con la regola dell'`awk` del Passo 6, l'archivio coi rimandi riscritti come il passo dice, la sezione e la tabella di
`porta-di-qualita.md` col segno e **una** riga vuota tolti, la cella di C-S0-1 e le quattro della (e) e della stella polare
**prima di ` |`** (E119, con l'asserzione che la riga finisca così) —, e lascia liberi, e li mostra, i soli valori del pezzo
JavaScript e `<Enn>`. Fuori dal confronto, dichiarati nel suo testo: i fine-riga, una fila di righe vuote contata come una, e
la data se è sbagliata ma uguale ovunque; li ho misurati a parte — i fine-riga nella §3, la riga vuota tolta con `git diff`
(una sola), la data contro quella del commit. Il secondo tempo del puntatore **parola per parola** lo prova il recinto, non il
vecchio testo: il `git diff` della §6 cambia la sola riga di testa.

| Nel clone | Che cosa ho cambiato | Uscita |
|---|---|---|
| `27963cd`, sopra `8c47715` | nella riga 14 della roadmap, dettata, *«livelli»* → *«livelle»* | `DIFFERS    docs/roadmap.md` e basta — gli altri otto `OK`, `HANDOFF.md` `SHOWN` —, `exit=1` |
| `f1d11e9`, sopra `8c47715` | nel piano, *«la chiusura»* → *«la chiusurA»* nella riga 9 della posizione | `UNEXPECTED docs/superpowers/plans/2026-09-23-design-system.md` (E120), `exit=1` |
| `8c47715`, dall'albero del repository | — | nove `OK`, `SHOWN docs/HANDOFF.md`, `exit=0`; liberi `663.26`/`201.23` su `B` e `698.20`/`213.00` su `HEAD` in `riferimenti.md`, `663,26` e `698,20` nella (e), e M-3 chiusa |

Il clone è tornato a `8c47715`, pulito. I valori liberi: il *dopo* è quello del mio cancello; il *prima* è del `gui/` di `B`,
uguale a `2a30916`, misurato il 2026-09-24 al Passo 1 del compito 1 — la provenienza scritta è vera (N-2 per la data).

**`word_for_word.py`**, copiato dal brief (`cmp` uguale a quello dell'implementatore e a quello della cartella del dispaccio),
coi testi di prima estratti da me da `git show 13fea0d:docs/COMPENDIO.md` con l'`awk` del Passo 6 (uguali, CR a parte, a
`old-6.txt` e `old-h.txt` dell'implementatore): sull'archivio vero `word for word` due volte, `exit=0`; su una copia con
*«Lo stile»* → *«Lo stilo»* nel puntatore archiviato, `NOT FOUND: …old-6.txt`, `exit=1`. E `check-docs.sh` sulle stesse
specie, nel clone: la tabella di M-3.

## 5. Il Passo 9

Il comando del passo, rilanciato: **23** tabelle — ventidue in `docs/archivio/consegna-piano-design-system.md` (righe 697…2519)
e una nel piano (`12237`) —, l'elenco del rapporto; a `f35d3c2` l'archivio ne aveva 21, e la ventiduesima l'ha portata
`13fea0d`. Le lezioni: 106 (N-5). «=» vuol dire che la classe del rapporto regge.

| Consegna | Lezione | Rapporto | La mia |
|---|---|---|---|
| esecuzione 1 | 1 `compare_task1.py` come modello | P-D | = |
| | 2 `Local:` di Vite spezzato dai colori | V (minore) | = V: la nota di memoria dell'agente (N-6) |
| | 3 pannello nascosto, `requestAnimationFrame` fermo | #136 | = |
| | 4 il rapporto dà per vero un commento | V (regola 8) | = |
| | 5 E1, i fine-riga fra macchine | V (E72) | = |
| pre-controllo 2 | 1 una guardia implicita sparisce cambiando la forma della corsa | V (sul posto) | = |
| | 2 la violazione sul file del progetto (E11) | V (#17, #24) | = |
| | 3 il costo sul comando che girerà | V (#31) | = (#31 è la stima sulla variante sbagliata) |
| | 4 il dispaccio non viaggiava | V (regola 8 del piano) | = |
| esecuzione 2 | 1 la forma dei fine-riga del modello | V | = |
| | 2 un confronto che mostra senza contare | V (#14, #48) | = |
| | 3 una negazione verde sul vuoto (E14) | V (#113, trappola 1) | = |
| | 4 il profilo di Playwright in `%TEMP%` | V (innocuo) | = |
| | 5 il log della CI a 403 | V (*«Leggere la CI da terra»*) | = |
| pre-controllo 3 | 1 una riga col perché senza prova | V (domanda 2) | = |
| | 2 un commento che quantifica su una cartella | V (#58) | = |
| | 3 un blocco del linter sul confine | V (#24) | = |
| esecuzione 3 | 1 l'aspetto non ha prove in jsdom | V (la (f)) | = |
| | 2 la cura di una direzione lascia l'altra | V (#24) | = |
| | 3 una frase ripetuta, tutte le case | V (#128) | = |
| | 4, 5 `chrome.exe --version`; Chrome che si aggiorna | #137 | = |
| | 6 il subagente interrotto, e `tail` sugli elenchi | V (attrezzi dei subagenti) | = (nota di memoria, righe 289–291) |
| pre-controllo 4 | 1 l'Atteso sulla sequenza dei passi | P-D | = |
| | 2 la pulizia in `afterEach` | V (sul posto) | = |
| | 3 il ramo di una sonda non percorso | V (#54, #74) | = |
| | 4 la scelta tecnica coi cinque criteri | P-B | = |
| esecuzione 4 | 1 una sonda copiata ne eredita i limiti | V (commenti) | = |
| | 2 una prova a mano con `bad: []` solo | V (#14, #113) | = |
| | 3 `TaskStop` e il `node` | #135 | = |
| | 4 il fotogramma vecchio del pannello | #136 | = |
| | 5 le frecce sui radio di `reka-ui` | V (commento, P-19) | = |
| pre-controllo 5 | 1 una violazione del linter con parole | V (#55) | = |
| | 2 un passo a mano fuori dal commit di un subagente | P-A | = |
| | 3 la lezione non riprodotta | V (P-19) | = |
| | 4 il segno `NON-VACUITY` | V (E37) | = |
| esecuzione 5 | 1, 3 le barre nascoste; il pannello che non disegna | #136 | = |
| | 2 `dockview` un fotogramma dopo | V (commento di E43) | = |
| | 4 la cura proposta è un'ipotesi | V (regola 8) | = |
| | 5 `os.replace` e *«Accesso negato»* | #135 | = (ma N-3) |
| pre-controllo 6 | 1 una sonda sulla pagina intera | V (#75) | = |
| | 2 la cura di una sonda comune | V (del piano) | = |
| | 3 l'Atteso che diverge e il contratto | V (regola 5) | = |
| | 4 il raggio del CSS | V (commento, E46) | = |
| | 5 il chiaro che coincide | V (E47) | = |
| esecuzione 6 | 1 un elemento messo a mano | V (#112) | = |
| | 2 `dragAndDrop` coi passi | V (E55) | = |
| | 3 un bordo senza stile calcola 0 | V (minore) | = V: il commento di `dock.browser.test.ts:148` (N-6) |
| | 4 la classe come parola | V | = |
| | 5 la radice nella tavola | V (del design system) | = |
| Passo 8 del 6 ripreso | 1 una misura che esclude | V (#75) | = |
| | 2 lo scarto che nasconde la regola | V (la (f)) | = |
| | 3, 4 la tavola e l'immagine del proprietario | V (del design system) | = |
| | 5 `.superpowers/brainstorm/` non ignorata | #135 | = (ma N-3) |
| tre voci, a metà | 1–4 il valore che segue; misurare prima; il metodo; i due assi | V (#15, #31, E59) | = |
| | 5 `file://` e la cartella del compagno | #135, #136 | = |
| tre voci, scritto | 1 un nome contro il codice | V, senza perché | = V: metodo del disegno (N-6) |
| | 2 una regola approvata dove il pezzo va | V, senza perché | = V: del design system (N-6) |
| | 3 una cifra copiata | V (#68) | = |
| | 4 una misura senza metodo | V (#31) | = |
| | 5 una consegna che mente su un processo | #135 | = |
| sguardo sulle tre voci | 1, 2 l'innesco; gli stili cambiati a pagina caricata | V (nella regola) | = |
| | 3, 4 le barre nascoste; 0 × 0 e 1,75 | #136 | = |
| | 5 le cifre nella prosa di una tavola | V (#68) | = |
| piano della cura 6bis | 1 scritto una volta, provato due | P-D | = |
| | 2, 4 `.msg span`; `BaseDialog` storto | V (del design system) | = |
| | 3 `ignoreDefaultArgs` | V (D26) | = |
| | 5 `rev` assente, `awk` e i backslash | V (#111) | = |
| pre-controllo 6bis | 1 un compito inserito, sotto quelli dopo | P-D | = |
| | 2 una cifra che il `grep` sulle parole non trova | V (#68) | = |
| | 3 lo sguardo fuori dal commit | P-A | = |
| | 4 la finestra coperta | #136 | = |
| | 5 `deliverAll()` e `StaleBuild` | V (E69) | = |
| esecuzione 6bis | 1 il costo cieco al confine | V (#75) | = |
| | 2 i rossi rifatti dal testo | P-D | = |
| | 3 il silenzio non è approvazione | P-C | = |
| | 4 6 GB liberi | V (dato) | = |
| pre-controllo 7 | 1 lo stato derivato | V (nel codice) | = |
| | 2 la violazione sulla suite intera | #139 | = |
| | 3 un fatto smentito da un compito inserito | V (regola 5) | = |
| | 4 il ripristino in `finally` | V (#69) | = |
| esecuzione 7 | 1 `git diff <commit>` e il file nuovo | V (#107) | = (la stessa radice) |
| | 2 il consumatore da fuori | V (domanda 3) | = |
| | 3 un rosso si legge prima | V (#17, #55) | = |
| | 4 una direzione in una prova che c'è | V, senza perché | = V: metodo delle prove (N-6) |
| | 5 la banda di costo | V (memoria) | = |
| pre-controllo 8 | 1 `F3` dentro un colore | V (#70, #109) | = |
| | 2 una riga di JSON tolta | #139 | = |
| | 3 i conti dedotti | V (regola 5, #53) | = |
| | 4 il testo rigenerato dalla copia | P-D | = |
| esecuzione 8 | 1 la prova nel compito, il rimedio fuori | V (del piano) | = |
| | 2 una dichiarazione è un'affermazione | V (#15) | = |
| | 3 `onDidMaximizedGroupChange` dentro `fromJSON` | V (commento in `dock.ts`) | = |
| | 4 il curatore | P-F | = |
| pre-controllo 9 | 1 `check-docs.sh` e il rimando rotto | #134 | = (ma M-3) |
| | 2 una promessa in una consegna | P-E | ≈ trappola della famiglia #132 (N-6) |
| | 3 `print x > 0` | #138 | = |
| | 4 la preferenza di Windows, la pagina come file | #136 | = (ma N-4) |

**I sei gotcha.** `grep -c` sulle 133 righe di `13fea0d` per le parole di ciascuno — `hide-scrollbars`, `TaskStop`,
`check-ignore`, `superpowers/brainstorm`, `Accesso negato`, `densità`, `chrome.exe`, `Playwright`, `si aggiorna`, `print x`,
`suite intera`, `parola per parola`, `compagno`, `requestAnimationFrame` → `0` ciascuna; `file://`, `violazion`, `parentesi`,
`.tmp` rendono righe d'altro (#13, #68, #112, #116, #127). Tutti e sei **nuovi** e **trappole vere**, ciascuno ritrovato
nelle lezioni che cita, coi numeri **134–139** — la serie contigua da 1 a 139 — e nella forma di #131 e #132 (#138 col ⚠️,
come #101, #122, #126). Le proposte P-A…P-F sono regole di lavoro, restano al proprietario nel *«Come si riprende»* della
chiusura (E116).

**Le promesse al compito 9** (7(f)), cercate con `grep -n 'compito 9\|la chiusura'` nell'archivio delle consegne e
nell'errata: E7, E10, E24, E43, E46, E85 e l'archivio a 1350 e 2314 chiedono le fonti nella tabella del Passo 8 — ci sono,
una riga ciascuna; E10 ed E64 chiedono la riga `gui: probes` con le due corse e le barre — c'è; E50 il `0` di `themeAbyss` —
blocco 3, riga 15; E118 la misura nel Passo 8 — c'è; C-S0-1 (P-29) — corretta; le ventitré tabelle (E116) — lette e
classificate. Mantenute tutte.

## 6. Ciò che non ho potuto verificare

- **La CI di `8c47715`**: non è pushato. Quella di `13fea0d` la dà verde il coordinatore.
- **`git fetch`**: non l'ho lanciato — il mandato vuole nessuna scrittura —, quindi `[ahead 1]` è misurato contro l'ultimo
  `origin/main` noto a questo clone.
- **Le pagine web** delle fonti (G18, *Dep Optimization Options*, CSS Backgrounds 3): non rilette; ho rilanciato i soli
  comandi locali scritti accanto alle righe.
- **L'aspetto**: la regola 5 non nomina il compito 9, e il compito non tocca il codice.
- **Il Passo 5 intermedio** (`91357`) l'ho ricostruito in Python dalla base, non misurato sull'albero di quel momento.
- **Le misure del pre-controllo** che il compito cita — la barra sotto i colori forzati, la preferenza di Windows (E118) —
  non le ho rifatte: sono del 2026-09-28 e hanno il loro metodo nell'errata.
