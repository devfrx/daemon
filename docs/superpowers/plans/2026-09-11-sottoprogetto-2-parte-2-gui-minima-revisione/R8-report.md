# Rapporto R8 — compiti 15, 16 e 17: il passo web del cancello, X-1 e X-3, la chiusura — 2026-09-16

⚠️ **Rapporto scritto A PEZZI.** Questo file cresce un compito alla volta: prima il **15**, poi il **16**, poi il **17**,
e in fondo le sezioni comuni (copertura, P, numeri di compito, comandi, non verificato, stato finale).

Perimetro estratto dal piano di **oggi** (`HEAD` = `5f525c0`), per intestazione:
`awk '/^## Compito 15:/{f=1} /^## Come si riprende/{exit} f' <piano> | wc -l` → **1519** (c15 689, c16 442, c17 388).

---

## Compito 15 — `scripts/gate-gui.sh`, la catena `eslint`, la riga in `gate.sh`

### Esito in tre righe

La catena `eslint` è stata **installata e fatta girare** sui tredici `.vue` che il piano detta, estratti dal piano
stesso, con l'`it.json` vero dei compiti 13 e 14: la configurazione del Passo 3 **non è verde**, e non per poco — sei
file non si **parsano** (nessun analizzatore TypeScript per i blocchi `<script setup lang="ts">`) e sei `error`
`no-raw-text` cadono su `:` e `—` lasciati nudi nei template; `npx eslint src` rende **12 problemi** e `EXIT=1`,
contro l'atteso *«nessuna riga e `EXIT=0`»*. Col solo analizzatore TypeScript aggiunto i sei errori di parsing
spariscono e ne compare un settimo di `no-raw-text`: **sette**, e il verde arriva solo togliendo i sette caratteri
nudi. Altri due passi non possono girare come sono scritti — Python su Windows non risolve il `/tmp` di Git Bash —
e uno dei tre `<tempo>` che il criterio pretende diversi da segnaposto non è misurato da nessun passo.

Tutto il resto del compito **regge**: le tre versioni appuntate sono ancora `latest`, il preset `flat/essential` porta
85 regole e zero avvisi, l'ancora di `gate.sh` è unica e la colonna dei comandi cade a 43 come le altre, le tre ancore
del Passo 11 rendono 1, `package-manager-cache` vale davvero `true` per difetto, `.gitignore` è come **P-102** lo
descrive.

### Rilievi

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R8-1 | 15 · Passo 2, *«Nessun `@typescript-eslint/parser`, e non è una dimenticanza»*; e Passo 4, *«Atteso: **nessuna riga** e `EXIT=0`»* | i `.ts` «questa catena non li guarda affatto», quindi nessun analizzatore TypeScript serve | ⛔ **il difetto non è nei `.ts`, è nei `.vue`**. `vue-eslint-parser` analizza `<script setup lang="ts">` con espree se `parserOptions.parser` non è dato, e la sintassi TypeScript lo ferma. Installata la catena alle versioni del Passo 2 ed estratti dal piano i tredici `.vue`, con l'`it.json` vero: `npx eslint src` → **`EXIT=1`**, `✖ 12 problems`, di cui **sei `Parsing error`** — `Confirm.vue` *Unexpected token `:`*, `Frame.vue` e `ViewBar.vue` *Unexpected token `ViewName`*, `Chat.vue` *Unexpected token `Block`*, `Placeholder.vue` *Unexpected token `{`*, `Settings.vue` *Unexpected token `PolicyArgument`*. ⚠️ **Un file che non si parsa non riceve NESSUNA regola**: l'eccezione di `Chat.vue` non è provabile, e `npx eslint src/panels/Chat.vue; echo $?` rende **1** con *Parsing error*, non lo **0** che la riga 4 della tabella del Passo 4 e il criterio di chiusura pretendono. Aggiunto `@typescript-eslint/parser` in un blocco `files: ["**/*.vue"]` con `languageOptions.parserOptions.parser`: i sei spariscono e la riga 4 rende **`EXIT=0`** | CONFERMATO | fatto | **sì** | Passo 2: una **quarta** dipendenza, `@typescript-eslint/parser` (oggi `8.70.0`, `latest`, da rimisurare il giorno dell'esecuzione) — è un peer **opzionale** di `eslint-plugin-vue`, quindi npm non la tira dentro; Passo 3: un blocco `{ name: "harness/ts-in-vue", files: ["**/*.vue"], languageOptions: { parserOptions: { parser: tsParser } } }` **prima** di `harness/rules`, col commento che dice perché (il preset non lo monta, e senza di esso sei dei tredici `.vue` non si parsano); e il capoverso *«Nessun `@typescript-eslint/parser`»* riscritto: resta vero che i **`.ts`** non li guarda nessuno, diventa falso che l'analizzatore non serva |
| R8-2 | 15 · Passo 4, la riga *«`== verde ==`»* e l'Atteso *«nessuna riga»*; e criterio *«il preset NON stampa avvisi»* | con la configurazione del Passo 3 l'albero pulito dà zero righe | ⛔ **Sette `error` `@intlify/vue-i18n/no-raw-text` sul codice che i compiti 13 e 14 dettano**, misurati con l'analizzatore TypeScript in piedi e l'`it.json` completo (`no-missing-keys` → **zero**): `ViewBar.vue` 32:27 `raw text ':'`; `Status.vue` 17:83 e 18:88 `':'`, 22:69 `'—'`; `Strip.vue` 13:36 e 22:36 `':'`, 14:50 `'—'`. Sono i separatori nudi fra due mustache — `{{ $t("status.vram") }}: {{ … }}`, `{{ $t("strip.degradation") }}:`, `<template v-if="…">—</template>`. ⚠️ **E la rete del 13 NON li vede**, perché la sua regexp pretende `[A-Za-zÀ-ÿ]{2,}`: il 13 e il 14 chiudono verdi e il 15 nasce rosso. Tolti i sette caratteri: `npx eslint src` → **`EXIT=0`**, zero righe | CONFERMATO | fatto | **sì** | due vie, e il piano ne deve scegliere una **dichiarandola**: (a) `"@intlify/vue-i18n/no-raw-text": ["error", { ignoreText: [":", "—"] }]` nel blocco `harness/rules` del Passo 3, col commento che dice che sono **punteggiatura** e non scritte (la regola porta `ignoreText`, `ignorePattern`, `ignoreNodes`, `attributes`: schema letto alla fonte); (b) i sette caratteri escono dai template e rientrano nelle chiavi di `it.json` — ma è una modifica ai **compiti 13 e 14**, non al 15. ⚠️ In entrambi i casi il Passo 4 guadagna la misura, e il criterio *«zero righe»* diventa provato invece che atteso |
| R8-3 | 15 · Passo 9, `block = io.open("/tmp/setup-node.yml", …)`; e Passo 11, `io.open("/tmp/richiamo-8.md", …)` e `io.open("/tmp/richiamo-scritte.md", …)` | il blocco YAML e i due richiami si scrivono «in un file dello scratchpad», nominato `/tmp/…`, e Python lo rilegge | ⛔ **Python su Windows non risolve il `/tmp` di Git Bash**, ed è la trappola che **questo stesso piano** registra nella tredicesima chiusura (*«PYTHON SU WINDOWS NON RISOLVE `/tmp` DI GIT BASH … ciò che Python deve leggere va nello scratchpad, col percorso Windows»*) — e che i tre Passi non hanno recepito. Misurato oggi: `echo ciao > /tmp/r8-test.txt` riesce (bash vede `C:/Users/zagor/AppData/Local/Temp`), e `python -c "io.open('/tmp/r8-test.txt')"` → `FileNotFoundError`, con `os.path.abspath` che rende **`C:\tmp\r8-test.txt`** (cartella inesistente). ⚠️ **E il `tr -cd '\r' < /tmp/… \| wc -c` che precede riesce**, perché è bash: il passo sembra a posto fino alla riga Python | CONFERMATO | fatto | **sì** | nei Passi 9 e 11 i tre file si scrivono in `<scratchpad>/setup-node.yml`, `<scratchpad>/richiamo-8.md`, `<scratchpad>/richiamo-scritte.md` **col percorso Windows completo**, e sia il `tr` sia l'`io.open` lo usano. (Stessa cura ai **cinque** siti gemelli del **16** — `"/tmp/audit-comment.sh"`, `"/tmp/quality-gate.yml"`, `"/tmp/x1.md"`, `"/tmp/x2.md"`, `"/tmp/richiamo-8-non-fa.md"` — vedi R8-13. ✅ **Il compito 17 non ne ha nessuno:** `grep -n '/tmp' c17.md` è vuoto) |
| R8-4 | 15 · Passo 10, *«Il terzo — il cancello **senza** il passo web — è il tempo che l'esecutore ha misurato **prima** di inserire la riga: si prende al Passo 1 e non si ricostruisce dopo»* | il terzo `<tempo>` del commento di `gate.sh` viene dal Passo 1 | ⛔ **Il Passo 1 non misura nessun tempo.** I suoi comandi sono `ls`, sei `grep -c`/`wc -l`, `git ls-files --eol`, `node --version`: nessun `time`. E il Passo 10 gira **dopo** il Passo 8, quindi il suo `time bash scripts/gate.sh` misura il cancello **col** passo web. Il piano chiude anche l'unica via d'uscita, vietando di ricostruirlo. Il criterio `grep -c '<tempo>\|<data>' scripts/gate.sh scripts/gate-gui.sh` → **0** obbliga quindi a scrivere un numero che nessun comando ha prodotto — vincolo globale 3 al contrario | CONFERMATO | fatto | **sì** | una riga nel blocco del **Passo 1**: `time bash scripts/gate.sh > /dev/null 2>&1` con l'Atteso *«si annota: è il terzo `<tempo>` del Passo 10»* (ed è anche la prova che il cancello è verde **prima** di cominciare, che oggi nessun passo del 15 fa) |
| R8-5 | 15 · *Interfaces*, *Consumes*: *«`gui/src/panels/Chat.vue` e le diciotto chiavi `modules.*` di `gui/src/locales/it.json` (compito **13**, Passo 5»* | `Chat.vue` viene dal compito 13 | ⛔ **`Chat.vue` è del compito 14**: `awk '/^## Compito 14:/{f=1} /^## Compito 15:/{exit} f' <piano> \| grep -n 'Create: .*Chat.vue'` lo mette fra i `Create` del 14, e il blocco di codice è alla testa *«`gui/src/panels/Chat.vue`, **LF** — il merito del `tiles/Chat.vue` dello spike»*, dentro il 14. Il Passo 1 dello stesso compito 15 lo dice giusto (*«Se `gui/src/panels/Chat.vue` non esiste, il compito 14 non è eseguito»*), quindi le due frasi si contraddicono. ⚠️ Era la riga **R7-7** del registro (da «14» a «13») e poi **R6-4** (da «Passo 12» a «Passo 5»): la correzione era giusta per le **chiavi** e ha trascinato con sé `Chat.vue` | CONFERMATO | fatto | no | *Consumes*: spezzare in due — *«`gui/src/panels/Chat.vue` (compito **14**); le diciotto chiavi `modules.*` di `gui/src/locales/it.json` (compito **13**, Passo 5 — R6-4)»* |
| R8-6 | 15 · Passo 3, il commento di `vue/multi-word-component-names` in `gui/eslint.config.js`: *«IS the `modules.*` key of the locale (task 14)»* | le chiavi `modules.*` sono del compito 14 | ⛔ **Dal 2026-09-16 (R6-4) le diciotto chiavi `modules.*` sono dettate per esteso al Passo 5 del compito 13**, e il 14 non le tocca: `sed -n '15238,15272p' <piano>` le mostra tutte e diciotto dentro il blocco `it.json` del 13. Il commento entra **nel sorgente** e viene committato, quindi è un commit che dice il falso. ⚠️ Nella stessa riga `PANEL_TYPES (task 13)` è invece giusto | CONFERMATO | fatto | **sì** | Passo 3: *«… and IS the `modules.*` key of the locale (task 13 too, step 5)»*, o `(both task 13)`. Nessun'altra riga del commento cambia |
| R8-7 | 15 · Passo 7, *«§8 lo chiede per nome: «nelle due direzioni … un test della SPA reso rosso → `GATE RED`». Qui si prova lo **script**, non ancora la riga del cancello»* | la seconda direzione è provata sullo script | ⛔ **E nessun passo la prova poi sul cancello.** `grep -n 'GATE RED' <piano>` → **una sola riga**, ed è questa frase del Passo 7. Il Passo 12 lancia `bash scripts/gate.sh` e attende solo `GATE GREEN`; il criterio di chiusura dice *«il passo web va rosso dai DUE mondi, eseguito come al Passo 7»*, cioè di nuovo lo script. La riga di `gate-gui.sh` della §8 chiede letteralmente `GATE RED`, e resta **scoperta** | CONFERMATO | fatto | no | Passo 12: dopo il verde, una mutazione sola — la stessa del mondo web del Passo 7 — con `bash scripts/gate.sh 2>&1 \| tail -3` → **`GATE RED`**, poi la revoca e `git status --porcelain` vuoto; e la riga corrispondente nel criterio di chiusura |
| R8-8 | 15 · Passo 1, Atteso: *«**otto** righe `gui-shell` in `.gitignore` … sono di `8fc9696`, **P-102**»* (e **D67** lo ripete) | le otto righe vengono dal commit `8fc9696` | ⛔ **Vengono da TRE commit**: `git blame -L 34,41 .gitignore` → `01694e3a` le due di `app/` (compito 2 del guscio), `8fc9696` le tre di `electron/` (compito 4), `d5eb0b8` le tre di `tauri/` (compito 5). Il comando che P-102 porta (`git log -1 -S'gui-shell/electron/out'`) trova per costruzione il **solo** commit che ha aggiunto `electron/out`: la risposta è giusta, la conclusione no. ⚠️ Ciò che il passo **asserisce** — otto righe, quattro lockfile tracciati, diff vuoto — regge: `grep -n 'gui-shell' .gitignore \| wc -l` → **8**, `git ls-files 'spikes/gui-shell/**package-lock.json' 'spikes/gui-shell/**Cargo.lock' \| wc -l` → **4** | CONFERMATO | fatto | no | Passo 1, P-102 e D67: *«sono di `01694e3a`, `8fc9696` e `d5eb0b8` — i compiti 2, 4 e 5 del guscio»*, oppure si toglie l'attribuzione e resta il conteggio, che è ciò su cui il passo asserisce |
| R8-9 | 15 · **P-101**, ultima riga: *«**Conseguenza:** nessuna `D`; la forma dello script `lint` nel **Passo 4**»* | la forma di `"lint": "eslint src"` sta al Passo 4 | ⛔ **Sta al Passo 2**, nel blocco `json` degli `scripts` col commento *«la cartella, non un elenco di file, e il perché è P-101»*. Il Passo 4 è il lint nelle quattro direzioni | CONFERMATO | fatto | no | P-101: *«… la forma dello script `lint` nel Passo 2»* |
| R8-10 | 15 · Passo 6, il commento *«LINT LAST, AND THE ORDER IS §8's … §8 fixed "npm ci, npm run build, npm test"; this step appends rather than reordering»* | l'aggiunta è dichiarata nel commento dello script | ⚠️ **La riga `scripts/gate-gui.sh` della §8 resta però a descrivere quattro comandi**, e dopo il 15 (lint) e il 16 (`cargo audit --file`, `npm audit`) ne avrà **sei**. I due richiami datati che il 15 scrive nella §8 stanno sul capoverso dei 🔶 dedotti e sulla riga *«le scritte»*; quello del 16 sul capoverso *«Ciò che la §8 non fa»*. Nessuno tocca la cella che elenca l'ordine, e chi rilegge la §8 ricostruirebbe uno script di quattro comandi | CONFERMATO | prosa | no | Passo 11 del 15: un terzo richiamo datato in coda alla cella *«Forma»* della riga `scripts/gate-gui.sh` della §8 — *«e `npm run lint` in coda dal compito 15; `cargo audit --file gui/fake-core/Cargo.lock` e `npm audit` dal 16»*. In alternativa lo scrive il 16, che è l'ultimo a toccarlo — ma allora va detto in **uno** dei due |

### Comandi lanciati per il compito 15

```
ls scripts/gate-gui.sh gui/eslint.config.js                      -> No such file (entrambi)
grep -c 'gate-gui' scripts/gate.sh                               -> 0
grep -c 'setup-node' .github/workflows/quality-gate.yml          -> 0
grep -n 'gui-shell' .gitignore | wc -l                           -> 8
git ls-files 'spikes/gui-shell/**package-lock.json' 'spikes/gui-shell/**Cargo.lock' | wc -l -> 4
git ls-files --eol scripts/gate.sh .github/workflows/quality-gate.yml -> i/lf w/crlf entrambi
tr -cd '\r' < scripts/gate.sh | wc -c ; wc -l < scripts/gate.sh  -> 97 ; 97  (CR = righe)
tr -cd '\r' < .github/workflows/quality-gate.yml | wc -c ; wc -l -> 16 ; 16 (CR = righe)
node --version                                                   -> v24.9.0
grep -n 'run "' scripts/gate.sh                                  -> 39..44 + 84; anchor unico
grep -c -F 'run "documentation consistency"' scripts/gate.sh     -> 1
awk '/^run "/{…index…}' scripts/gate.sh                          -> colonna 43 per cinque righe su sei (44 per «attributes», 34 per il settimo passo); la riga nuova del piano cade a 43
python (registry.npmjs.org) eslint/eslint-plugin-vue/@intlify   -> 10.10.0, 10.11.0, 4.5.1 sono TUTTE `latest`; engines piu' larghi di D37; `@typescript-eslint/parser` optional peer confermato
npm install eslint@10.10.0 eslint-plugin-vue@10.11.0 @intlify/eslint-plugin-vue-i18n@4.5.1 -> 160 pacchetti, versioni esatte
node -e "…vue.configs['flat/essential']…"                        -> error=85 warn=0, multi-word: error (idem strongly-recommended 85/25 e recommended 85/33)
node -e "…warn di flat/recommended…"                             -> 33, tutte di formattazione
grep -n 'no-v-html' node_modules/eslint-plugin-vue/dist/configs/flat/vue3-recommended.js -> "vue/no-v-html": "warn"
node -e "…essential no-v-html…"                                  -> undefined (non c'e')
grep -n "no-raw-text" node_modules/@intlify/…/dist/configs/flat/recommended.js -> 'warn'
node -e "…i18n.configs['flat/base']…"                            -> 3 blocchi, registra il plugin, parser json/json5/yaml, due regole `off`
python extract_vue.py                                            -> 13 file .vue estratti dal piano (Frame.vue due blocchi: vince quello del 14)
npx eslint src (config del piano, it.json completo)              -> EXIT=1, 12 errori: 6 Parsing error + 6 no-raw-text
npx eslint src (col parser TS)                                   -> EXIT=1, 7 errori: 7 no-raw-text, 0 no-missing-keys
npx eslint src (col parser TS, tolti i 7 caratteri nudi)         -> EXIT=0, zero righe
passo4.sh, config del piano: 0) EXIT=1 con Parsing error · 1) EXIT=1 `raw text 'riprova piu tardi'` · 2) EXIT=1 `'modules.inventato' does not exist` · 3) EXIT=1 `'v-html' directive can lead to XSS attack` · 4) **EXIT=1 Parsing error** (atteso 0)
passo4.sh, col parser TS: 1) 2) 3) identici · 4) **EXIT=0** senza righe
npx eslint src/probe.ts                                          -> `File ignored because no matching configuration was supplied`, EXIT=0 (P-101)
npx eslint src/panels/Built.vue (chiave costruita + letterale)    -> solo il letterale `modules.inventato` -> error (P-105)
node --input-type=module -e "…await import('./eslint.config.js')…" -> stampa i tre blocchi `harness/`, nessun "warn", EXIT=0
curl setup-node v7 action.yml                                    -> package-manager-cache default: true; descrizione = quella di P-103/D66
curl setup-node v7 src/util.ts                                   -> legge volta.node, devEngines.runtime, poi `engines.node` da package.json
api.github.com releases/latest                                   -> setup-node v7.0.0 2026-07-14 · checkout v7.0.1 2026-07-20
grep -n 'manifest-path. compili nel' <disegno2>                   -> 1 riga (474)
python count ancora "finto e non riusi quello del workspace. **Assunto:** niente." -> 1
python, righe che cominciano con "| le scritte, `locales/it.json` (G21) |" -> 1, finisce con " |"
git ls-files --eol dei due disegni e di docs/design/10            -> i/lf w/lf tutti e tre
cargo metadata su spikes/rust, spikes/gui-ipc e il workspace      -> tre `target_directory` distinti (P-104 regge)
git blame -L 34,41 .gitignore                                     -> 01694e3a (2), 8fc9696 (3), d5eb0b8 (3)
python: os.path.abspath('/tmp/x') su Windows                      -> C:\tmp\x ; io.open -> FileNotFoundError
```

---

## Compito 16 — X-1 e X-3: la matrice Windows, `cargo audit`, `npm audit`

### Esito in tre righe

Il **merito** di questo compito regge, e l'ho verificato eseguendolo dove si poteva: `cargo audit` 0.22.2 dà
davvero `1 allowed warning found` / uscita **0** e `1 denied warning found!` / uscita **1** sullo stesso lockfile
senza toccare niente; `npm audit` su un albero con `minimist@0.0.8` dà `1 critical severity vulnerability` e
uscita **1**; il flusso di lavoro dettato è YAML valido con `fail-fast: false`, due sistemi e cinque passi; il
blocco `setup-node` combacia carattere per carattere con quello del 15; simulando **entrambi** gli inserimenti su
una copia di `gate.sh` le otto righe `run` escono nell'ordine che il criterio descrive, incolonnate a 43, e
`bash -n` è verde.

Tre cose non tornano: un criterio di chiusura che va **rosso su un artefatto giusto** (`grep -c 'audit-level'`
rende **1**, perché la parola sta nel commento che il compito stesso scrive), il `/tmp` di Git Bash riletto da
Python in **cinque** siti, e un Atteso del Passo 1 (`grep -c 'gate-gui' scripts/gate.sh` → «uno») che dopo il
Passo 10 del 15 vale **tre**. Più due cifre in prosa che oggi non si riproducono.

### Rilievi

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R8-11 | 16 · criterio di chiusura, *«niente `-n` e niente `--audit-level`, e lo si prova col comando invece di rileggerlo»*, secondo comando: `grep -c 'audit-level' scripts/gate-gui.sh` → **zero** | il criterio rende zero su un `gate-gui.sh` corretto | ⛔ **Rende UNO.** Il commento che il **Passo 4 dello stesso compito** detta contiene la parola: `# ⛔ NO --audit-level, AND THAT IS A DECISION.` Ricostruito `gate-gui.sh` dal piano (Passo 6 del 15 più i due blocchi del Passo 4 del 16): `grep -c 'audit-level' gate-gui.sh` → **1**. Il criterio andrebbe **rosso sull'artefatto giusto**. ⚠️ Il gemello su `gate.sh` invece regge: provato nelle due direzioni su tre file di prova — con `-n` → **1**, con `--no-fetch` → **1**, con la riga vera → **0** | CONFERMATO | fatto | **sì** | criterio: `grep -cE '^[^#]*audit-level' scripts/gate-gui.sh` → **0**, che ignora i commenti. Provato nelle due direzioni sul file ricostruito: **0** com'è, **1** con `npm audit --audit-level=high` al posto di `npm audit` |
| R8-12 | 16 · criterio, *«`npm audit` è l'ULTIMA riga di `gate-gui.sh`, dentro `gui/`: `tail -3 scripts/gate-gui.sh` lo mostra, e sopra c'è `npm run lint` del compito 15»* | tre righe bastano a vedere `npm audit` **e** `npm run lint` sopra | ⛔ **`tail -3` del file ricostruito rende**: l'ultima riga del commento (*«# upgrade, or to declare the exception WITH ITS DATE…»*), `echo "-------- gui: advisories"`, `npm audit`. `npm run lint` è **undici** righe più su — il blocco di commento del Passo 4 ne ha otto. Solo `tail -14` le mostra entrambe | CONFERMATO | fatto | no | criterio: `tail -2 scripts/gate-gui.sh` per l'ultima riga, e `grep -n 'npm run lint\|npm audit' scripts/gate-gui.sh` per l'ordine (due righe, `lint` prima di `audit`) — un numero fisso di righe invecchia col commento |
| R8-13 | 16 · Passo 2 (`io.open("/tmp/audit-comment.sh", …)`), Passo 5 (`src = io.open("/tmp/quality-gate.yml", …)`), Passo 6 (`/tmp/x1.md`, `/tmp/x2.md`, `/tmp/richiamo-8-non-fa.md`) | i cinque file si scrivono in `/tmp/` e Python li rilegge | ⛔ **Stesso difetto di R8-3, in cinque siti**: Python su Windows manda `/tmp/x` a `C:\tmp\x`, che non esiste, mentre bash vede `C:/Users/zagor/AppData/Local/Temp`. Misurato: `os.path.abspath('/tmp/r8-test.txt')` → `C:\tmp\r8-test.txt`, `io.open` → `FileNotFoundError`. ⚠️ Nel Passo 5 il `diff <(git show …) <(grep -A 4 'setup-node@' /tmp/quality-gate.yml)` è **bash** e riesce: a fallire è la riga Python subito dopo | CONFERMATO | fatto | **sì** | i cinque file nello scratchpad col percorso Windows, in `tr` e in `io.open` |
| R8-14 | 16 · Passo 2, il commento dettato di `gate.sh`: *«`-n` gives the SAME verdict in ~1.3s instead of ~10s»* e *«about nine of its ten seconds are [the network]»* | il passo costa ~10 s, nove dei quali di rete | ⛔ **Non si riproduce oggi, e il perché è strutturale: i ~10 s erano la PRIMA clonazione del database degli avvisi.** `ls -la ~/.cargo/advisory-db` → creato il **2026-09-15**, `du -sh` → **45M**; da allora è un fetch incrementale. Misurato tre volte oggi: `cargo audit` **2.81 s / 2.16 s / 2.08 s**, `cargo audit -n` **0.55 s / 0.55 s / 0.56 s** — la rete è ~1,5 s su ~2,1 s (**~70 %**, non il 90 %), e il totale è **un quinto** di quello scritto. ⚠️ Il commento entra nel sorgente committato e nessun passo del 16 lo rimisura: è un numero in prosa che nasce vecchio (gotcha #31) | CONFERMATO | fatto | no | due vie: (a) i due numeri diventano `<tempo>` e il **Passo 3** li misura, come il 15 fa coi tempi del finto; (b) il commento distingue le due situazioni — *«the FIRST run clones the ~45 MB advisory database (~10s, 2026-09-15); afterwards it is an incremental fetch: `<tempo>` against `<tempo>` with `-n`»*. In entrambi i casi **l'argomento di D69 non cambia**: `-n` resta più veloce e resta cieco |
| R8-15 | 16 · Passo 6, la riga **X-1** di `docs/audit-2026-08-27.md` in coda alla quale il compito scrive: *«✅ **Costa poco**: le **due** sonde sui bit di permesso sono già `#[cfg(unix)]` e si autoescludono»* | le sonde `#[cfg(unix)]` sono due | ⛔ **La sonda è UNA.** `grep -rn 'cfg(unix)' crates/ --include='*.rs'` → due attributi, ma solo uno è una sonda: `crates/platform/tests/file_journal.rs:70`, sopra `the_journal_file_is_not_world_readable`; l'altro, `crates/platform/src/journal.rs:201`, è un blocco di **codice di produzione** (`options.mode(0o600)` dentro `FileBackend::open`). Era già così al commit dell'audit: `git grep 'cfg(unix)' d902c40 -- crates/` rende le stesse due righe. ⚠️ Il compito 16 scrive il proprio richiamo **in coda a questa stessa cella** e lascia il numerale dietro di sé | CONFERMATO | fatto | no | Passo 6: il richiamo di X-1 chiude anche il numerale — *«⚠️ e la sonda `#[cfg(unix)]` è **una**, `the_journal_file_is_not_world_readable`; il secondo `cfg(unix)` è la `mode(0o600)` di `FileBackend::open`, codice di produzione: quante siano lo dice `grep -rn 'cfg(unix)' crates/ --include='*.rs'`»* — nella forma che `CLAUDE.md` chiede, il comando invece della cifra |
| R8-16 | 16 · Passo 1, Atteso: *«**uno** per `gate-gui` e `setup-node`, ⛔ e se sono zero il compito 15 non è eseguito»* | `grep -c 'gate-gui' scripts/gate.sh` rende 1 | ⛔ **Rende TRE**, perché il **Passo 10 del compito 15** scrive sopra la riga `run` un commento di sette righe in cui `gate-gui` compare su **due** di esse (*«`gate-gui.sh` rebuilds `kernel`…»* e *«`<data>`: `gate-gui.sh` alone `<tempo>`…»*). Contato sul commento estratto dal piano: 2 righe più la riga `run` = **3**. ⚠️ `setup-node` invece rende davvero **1**: nessun commento del blocco del Passo 9 nomina l'azione | CONFERMATO | fatto | no | Passo 1: *«**più di zero** per `gate-gui` — la riga `run` più il commento del Passo 10 del 15 — e **uno** per `setup-node`»* |
| R8-17 | 16 · criterio, *«niente `-n` … `grep -cE '…' scripts/gate.sh`»* | il divieto di `-n` si prova su `gate.sh` | ⚠️ **Il comando guarda solo `gate.sh`**, ma da questo compito esiste un **secondo** sito `cargo audit`: `cargo audit --file gui/fake-core/Cargo.lock` in `gate-gui.sh` (**D83**). Lì un `-n` futuro non lo vedrebbe nessuno, e l'argomento di D69 vale identico per il lockfile del finto | CONFERMATO | fatto | no | il criterio elenca **due** file nello stesso `grep -cE`: `scripts/gate.sh scripts/gate-gui.sh` → **0** per entrambi (con `-c` e due file l'uscita è `file:conteggio`, che si legge riga per riga) |

### Comandi lanciati per il compito 16

```
grep -c 'cargo audit' scripts/gate.sh                             -> 0
grep -c 'matrix' .github/workflows/quality-gate.yml               -> 0
cargo audit --version                                             -> cargo-audit-audit 0.22.2 (installato)
git ls-files --eol gate.sh / workflow / audit-2026-08-27.md       -> i/lf w/crlf tutti e tre (R10-2 regge)
tr -cd '\r' < docs/audit-2026-08-27.md | wc -c ; wc -l            -> 1885 ; 1885 (CR = righe)
awk -F'|' sulle righe X-1 e X-3 di docs/audit-2026-08-27.md       -> le due righe ci sono
grep -c -F 'la voce resta aperta finche' il passo non esiste'     -> 1
grep -c -F 'la voce resta aperta finche' i passi non esistono'    -> 1
grep -c -F 'run "attributes of the constrained crates"' gate.sh   -> 1 (unica); 'gate-deps.sh' -> 3 occorrenze (il piano dice «tre»: regge)
grep -c 'CHIUSA IL' docs/audit-2026-08-27.md                      -> 0 oggi (dopo il Passo 6: 2 -> «piu' di uno» regge)
grep -rn 'cfg(unix)' crates/ --include='*.rs'                     -> 2 attributi, UNA sola sonda
git grep 'cfg(unix)' d902c40 -- crates/                           -> le stesse due gia' al commit dell'audit
cargo audit                                                       -> `1 allowed warning found`, EXIT=0 (RUSTSEC-2025-0141, bincode)
cargo audit --deny unmaintained                                   -> `1 denied warning found!`, EXIT=1
git status --porcelain dopo le due                                -> vuoto
time cargo audit (x3)                                             -> 2.812s / 2.157s / 2.081s
time cargo audit -n (x3)                                          -> 0.548s / 0.549s / 0.563s ; stesso verdetto, EXIT=0
ls -la ~/.cargo/advisory-db ; du -sh                              -> creato il 2026-09-15 ; 45M
npm audit su un albero con minimist@0.0.8                         -> `1 critical severity vulnerability`, EXIT=1
npm audit su un package.json SENZA lockfile                       -> ENOLOCK, EXIT=1 (in gate-gui.sh `npm ci` viene prima: non morde)
npm audit sul lockfile del modello dell'11 (99 pacchetti)         -> `found 0 vulnerabilities`, EXIT=0 (P-110, direzione confermata su un insieme piu' piccolo)
curl api.github.com/repos/devfrx/daemon                           -> private: False, visibility: public (P-109 / D70 regge)
estratto e parsato il YAML del Passo 5 (53 righe)                 -> fail-fast: False, os: [ubuntu-latest, windows-latest], runs-on: ${{ matrix.os }}, 5 passi
diff del blocco `grep -A 4 'setup-node@'`                         -> IDENTICO a quello del compito 15
simulazione dei DUE inserimenti su una copia di gate.sh           -> CR 122 = righe 122 ; ordine: workspace, tests, no-OS, allow-list, **dependency advisories**, attributes, gui, documentation ; colonna 43 per tutte tranne «attributes» (44, gia' cosi' oggi)
bash -n sulla copia di gate.sh e su gate-gui.sh ricostruito       -> EXIT=0 entrambi
grep -c 'audit-level' gate-gui.sh ricostruito                     -> 1 (il criterio attende 0)
grep -cE '^[^#]*audit-level' ricostruito / mutato                 -> 0 / 1 (il rimedio morde nelle due direzioni)
grep -cE 'cargo audit ((-n|--no-fetch)\b|.*--no-fetch)' su tre file di prova -> 1 (`-n`), 1 (`--no-fetch`), 0 (riga vera)
grep -c 'cargo audit --file gui/fake-core/Cargo.lock' gate-gui.sh -> 1
tail -3 / tail -14 gate-gui.sh                                    -> `npm run lint` NON e' in tail -3
grep -nE 'timeout|nproc|date \+%s%N|/dev/shm|readlink -f|stat -c|realpath|md5sum|ulimit|taskset' scripts/*.sh -> NESSUNA riga
bash su uno script CRLF in questa Git Bash (5.2.37 msys)          -> gira, EXIT=0: i CRLF NON sono il rischio Windows che si temeva
```

---

## Compito 17 — la chiusura: i documenti in ogni casa, e la Definizione di «fatto»

### Esito in tre righe

Le ancore di questo compito **reggono tutte**: le cinque frasi della stella per il Passo 8-bis compaiono una volta
ciascuna, le sette case sono `i/lf w/crlf` e la stella, `design/10` e il piano `i/lf w/lf` (**P-114**), il margine
del compendio è **esattamente** `11030` (**P-115**, **P-6**), `README.md` rende **zero** (**P-116**), la sezione S3
c'è una volta sola, `design/10` ha due `erDiagram` con `INVOCATION_DETAIL` nel **secondo** e `POLICY_DETAIL` da
nessuna parte, e l'ancora di D56 che il 14 scriverà combacia col `grep` che il 17 lancia. Il compito **sta** sotto
il tetto: stimato dai blocchi dettati, aggiunge ~1122 byte di §12 più ~395 della riga nei chiusi e ~519 di ⏭️
nuovo, contro un ⏭️ vecchio che ne pesa di più — margine ancora sopra i diecimila byte.

Tre comandi però non rendono ciò che il compito dice: `awk '/⏭️/` rende **tre** righe e non una,
`grep -n '^**Criterio di chiusura'` ne rende **dieci** e non sedici, e `ls crates/kernel/tests/frozen/ | wc -l`
renderà **nove** e non otto, perché conta anche `record_v1.map`. E un quarto fatto, che non è del 17 ma gli cade
addosso: **il compito 7 non ha nessun criterio di chiusura**, quindi i criteri sono sedici per diciassette compiti.

### Rilievi

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R8-18 | 17 · criterio, *«il ⏭️ della §6 nomina il sotto-progetto 13 e AUD-004, e non è una catena di ✅: `awk '/⏭️/{print}' docs/COMPENDIO.md` rende **una riga sola**»* | dopo la riscrittura del puntatore il simbolo ⏭️ vive su una riga sola | ⛔ **Oggi ne rende TRE**, e due di esse il compito 17 non le tocca: `awk '/⏭️/{print}' docs/COMPENDIO.md \| wc -l` → **3**. La prima è il puntatore vero; la seconda è l'elemento **2** dell'elenco numerato subito sotto, che porta un ⏭️ **a metà riga** (*«… `dockview` resta; ⏭️ **la parte 2**: i pezzi 2–9 …»*); la terza è una riga della **tabella delle voci aperte** della stessa §6, che cita `⏭️` come **letterale** dentro la regola (*«fuori da `COMPENDIO.md`, ogni riga che porta `⏭️`…»*). Quella terza resta lì qualunque cosa faccia il 17, quindi il criterio **non può mai rendere 1**. ⚠️ `awk '/^⏭️/'` invece rende **1** oggi: il puntatore è l'unica riga che **comincia** col simbolo | CONFERMATO | fatto | **sì** | criterio: `grep -c '^⏭️ \*\*IL PROSSIMO PASSO' docs/COMPENDIO.md` → **1** (misurato oggi: 1). E il Passo 3 dica esplicitamente che **anche l'elenco numerato sotto il ⏭️ si riscrive**, o il ⏭️ di metà riga dell'elemento 2 sopravvive alla riscrittura e dice «la parte 2» quando la parte 2 è finita |
| R8-19 | 17 · Passo 9, la riga `ls crates/kernel/tests/frozen/ | wc -l   # i record congelati, otto` | il comando rende otto | ⛔ **Renderà NOVE.** `ls crates/kernel/tests/frozen/` oggi rende **7** voci: **sei** `.cbor` più `record_v1.map`, che non è un record. I compiti 6 e 8 aggiungono due `.cbor` (`record_v1_invocation.cbor`, `record_v1_policy.cbor`), quindi 8 `.cbor` + 1 `.map` = **9**. ⚠️ La riga entra nella **Definizione di «fatto»**, cioè in ciò che una sessione nuova legge per sapere se la parte 2 è fatta: un comando la cui uscita attesa è sbagliata è peggio di nessun comando. E la §8 del 2 la scrive giusta — *«quanti lo dice `ls crates/kernel/tests/frozen/`»*, senza numerale | CONFERMATO | fatto | **sì** | Passo 9: `ls crates/kernel/tests/frozen/*.cbor \| wc -l   # gli otto record congelati` — oppure, nella forma che `CLAUDE.md` preferisce e che la §8 usa già, il comando **senza** la cifra |
| R8-20 | 17 · Passo 1, *«E si rileggono i **sedici** criteri di chiusura, uno per compito»*, col comando `grep -n '^\*\*Criterio di chiusura' <piano>` | il comando mostra i sedici criteri | ⛔ **Ne mostra DIECI.** `grep -c '^\*\*Criterio di chiusura' <piano>` → **10**; `grep -cE '^(#### \|\*\*)Criterio di chiusura' <piano>` → **16**. I compiti **1–6** intestano il proprio criterio con `#### Criterio di chiusura del compito N`, che l'ancora `^\*\*` non vede. ⚠️ **E il Passo 9 costruisce la Definizione di «fatto» da quella rilettura**: seguendo il comando si riassumerebbero dieci criteri su sedici, e il prodotto sembrerebbe più completo di quanto è | CONFERMATO | fatto | **sì** | Passo 1: `grep -nE '^(#### \|\*\*)Criterio di chiusura' <piano>` — provato oggi: **16** righe, con i sei `####` dei compiti 1–6 |
| R8-21 | 17 · Passo 1 e Passo 9, *«i sedici criteri di chiusura, **uno per compito**»* | ogni compito ha il suo criterio | ⛔ **Il compito 7 non ne ha nessuno.** `grep -n '^## Compito ' <piano>` dà al 7 le righe 8061–9938, e `awk` su quel tratto con `grep -niE 'criterio di chiusura'` rende **niente**. I sedici sono i compiti **1–6** (`####`), **8–10** (*«Criterio di chiusura, coi comandi:»*) e **11–17**: sedici criteri per **diciassette** compiti, e il mancante è quello di `kernel::serving` — il dispaccio, il ramo `Request` non servito, i cinque accessori. ⚠️ Il difetto è del **7**, ma cade sul **17**, che è l'unico compito che si è impegnato a riassumerli | CONFERMATO | fatto | no | non è una correzione del 17: **una voce per il compito 7**, che riceve il proprio criterio di chiusura nella forma dei fratelli; e il Passo 1 del 17 dica *«uno per compito, e sono `grep -cE '^(#### \|\*\*)Criterio di chiusura' <piano>`»*, col comando invece del numerale |
| R8-22 | 17 · Passo 1, Atteso *«le **due** righe della roadmap»*, per `grep -n 'parte-2-gui-minima\|^\| 2 \| GUI minima' docs/roadmap.md`; e Passo 5, *«la riga del **piano della parte 2** — trovata col secondo `grep`»* | il primo comando rende due righe, il secondo una | ⛔ **Tre e due.** `grep -c 'parte-2-gui-minima\|^\| 2 \| GUI minima' docs/roadmap.md` → **3**: riga **6** (*«Ultimo aggiornamento: **2026-09-15**, col conteggio dei compiti tolto dalla riga del [piano della parte 2]…»*), riga **163** (`\| 2 \| GUI minima (shell, chat, stato) \| — \| ⬜ \| 1, ADR-0027 \|`) e riga **231** (la riga del piano nella tabella dei piani). E `grep -n 'parte-2-gui-minima' docs/roadmap.md` da solo rende **due** righe, la 6 e la 231: *«la riga del piano»* non è isolata da quel comando. ⚠️ La riga 6 è proprio quella che il Passo 5 dice di **riscrivere** con la data di oggi, quindi sapere che c'è serve | CONFERMATO | fatto | no | Passo 1: *«**tre** righe — l'intestazione «Ultimo aggiornamento», la riga 2 dei sotto-progetti e la riga del piano»*; Passo 5: il secondo `grep` diventa `grep -n '^| \[Sotto-progetto 2 · parte 2' docs/roadmap.md`, che rende **una** riga (misurato: la riga 231 comincia così) |
| R8-23 | 17 · criterio, `grep -c '<data>' docs/COMPENDIO.md` → **zero**, e nient'altro | la rete sui segnaposto di data è quella | ⚠️ **Copre una casa su otto.** Il 17 scrive `<data>` — contato sull'estratto: **19** occorrenze nel testo del compito — in `COMPENDIO.md` (§12 no, §6 sì), `README.md`, `roadmap.md`, `porta-di-qualita.md`, la **stella** (il `DATE = "<data>"` del Passo 8-bis, che è un letterale **dentro lo script** e non ha nessun `assert` che lo fermi, a differenza del Passo 11 del 15), `design/10` e la **Definizione di «fatto»** nel piano. Solo il compendio ha il suo `grep`; l'audit ha il proprio nel 16 | CONFERMATO | fatto | no | Passo 11 e criterio: **un** comando per tutte le case — `grep -rn '<data>\|<tempo>' docs/ --include='*.md'` → **zero** (oggi rende le occorrenze del piano stesso, quindi si esclude il piano o lo si lancia dopo aver sostituito anche lì); e un `assert "<data>" not in DATE` — o meglio `assert DATE != "<data>"` — in testa allo script del Passo 8-bis, nella forma che il Passo 11 del 15 usa già |
| R8-24 | 17 · criterio, *«la tabella della posizione è tutta ✅ … `awk '/^\| \*\*[0-9]+\*\* \|/{print}' <questo file> \| grep -c '⬜'` → **zero**»* | l'`awk` isola le righe della posizione | ⚠️ **L'`awk` rende 19 righe e la posizione ne ha 17**: le due in più sono `\| **209** \|` e `\| **1527** \|`, la tabella di **P-22** sulle righe della spec. Ma **nessuna delle due porta ⬜** (contato: `awk … \| grep -c '⬜'` → **17** oggi, tutte della posizione), quindi il criterio regge | NON RIPRODOTTO | fatto | no | — |

### Comandi lanciati per il compito 17

```
git ls-files --eol delle sette case + stella + design/10 + piano  -> sette i/lf w/crlf ; stella, design/10 e piano i/lf w/lf (P-114 regge)
grep -n '^ceiling=' scripts/check-docs.sh                         -> 346:ceiling=111616
wc -c docs/COMPENDIO.md                                           -> 100586  => margine 111616-100586 = 11030 (P-115 e P-6 reggono ESATTAMENTE)
awk '/^## I gotcha/…' docs/HANDOFF.md                             -> gotcha: 119 (baseline del Passo 8)
grep -c 'sottoprogetto-2|direzione-gui|gui-minima' docs/README.md -> 0 (P-116 regge)
grep -c 'LA SONDA S3' docs/porta-di-qualita.md                    -> 1 (il modello c'e')
grep -c 'PARTE 2' docs/porta-di-qualita.md                        -> 0 oggi (dopo il Passo 7: 1, il criterio regge)
grep -n '^| 2 | GUI minima' docs/roadmap.md                       -> 163, titolo «GUI minima (shell, chat, stato)» (D86 lo cambia); grep -c -> 1
grep -c 'parte-2-gui-minima|^| 2 | GUI minima' docs/roadmap.md    -> 3 (righe 6, 163, 231)
le cinque ancore del Passo 8-bis nella stella                     -> 1,1,1,1,1 (righe 54, 1079, 339, 661, 733); la riga 54 finisce con " |"
grep -c 'confermato dal compito' / 'compito 17 del piano della parte 2' nella stella -> 0 e 0 oggi (dopo: 3 e 2, i criteri reggono)
grep -c 'dal compito 14 del piano della parte 2 (P-89, D56)' stella -> 0 oggi; e il RECALL del Passo 15 del 14 produce ESATTAMENTE quella stringa (R9b-14 regge)
grep -n 'erDiagram|INVOCATION_DETAIL|POLICY_DETAIL|specie [0-9]' design/10 -> primo erDiagram riga 30 (specie 0,1,2), secondo riga 90 con INVOCATION_DETAIL «specie nuova (col 2)» e il blocco a 126; POLICY_DETAIL assente; la riga 160 della tabella c'e'
grep -c '^erDiagram' design/10                                    -> 2 (l'awk del criterio non e' ingannato dalla riga 195, che comincia con `grep`)
grep -n 'enum Detail' -A 8 crates/kernel/src/record.rs            -> varianti 0 Verdict, 1 Routing, 2 Permission (3 e 4 arrivano dal 6 e dall'8)
ls crates/kernel/tests/frozen/ | wc -l                            -> 7 (sei .cbor + record_v1.map) ; ls *.cbor | wc -l -> 6
grep -n 'pub trait Custody' <piano>                               -> 5478 (il compito 4 lo detta: il comando del Passo 9 rendera' 1)
awk '/⏭️/{print}' docs/COMPENDIO.md | wc -l                        -> 3 ; grep -c '^⏭️' -> 1 ; grep -c '^⏭️ **IL PROSSIMO PASSO' -> 1
grep -c '^**00' docs/COMPENDIO.md                                 -> 39 (le voci ADR della §5: la baseline del criterio)
grep -c '<data>' docs/COMPENDIO.md                                -> 0
grep -n '^## 12\.' docs/COMPENDIO.md + forma delle righe          -> 901, tabella «| Se ti serve… | Apri |»: le quattro righe dettate hanno la forma giusta
grep -c '^**Criterio di chiusura' <piano>                         -> 10 ; grep -cE '^(#### |**)Criterio di chiusura' -> 16
awk sul tratto del compito 7 (righe 8061-9938) + grep 'criterio di chiusura' -> NIENTE: il 7 non ha criterio
awk '/^| **[0-9]+** |/{print}' <piano> | wc -l                     -> 19 ; | grep -c '⬜' -> 17
grep -n '^## 2\. Conversazione|^## 8\. Sistema' docs/tracciabilita.md -> 103 e 233 (le due ancore del Passo 6 esistono)
stima del compendio dopo il 17: +1122 (§12) +395 (riga nei chiusi) +519 (⏭️ nuovo) - il ⏭️ vecchio (620 byte il capoverso, 1248 l'elenco) -> margine ancora > 10000 byte
```

### Rilievi del compito 17, seguito — i due disegni

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R8-25 | 17 · Passo 4, *«La tabella «Dove va cosa» NON si tocca se questo piano non ha reso falsa nessuna delle sue righe»*, e **P-116**, che manda le due righe nella tabella *«Specifiche»* | le righe dei due disegni vanno in «Specifiche» | ⚠️ La **§6 della stella polare** dice invece *«la riga nella §12 del compendio e in **«Dove va cosa»** di `README.md` … un compito del piano (decisione 9 del coordinatore), nella forma delle righe dei disegni dei gesti e della knowledge base»*. L'ho cercato: quelle due righe-modello stanno alle righe **205–206** di `docs/README.md`, cioè **dentro `## Specifiche`** (che comincia alla 195), non in *«Dove va cosa»* (107–127). Il piano ha ragione, e la sua fonte è esplicita: **D14 del piano della parte 1** dice *«nella §12 del compendio e nella tabella «Specifiche» di `README.md`»*, ed è il file che il *Read* del 17 nomina (**P-112**). È la §6 della stella a dire la casa sbagliata | NON RIPRODOTTO | fatto | no | — (se si vuole: una riga in **P-116** che dica *«la §6 della stella dice «Dove va cosa»; il precedente vero — gesti e knowledge base — vive in «Specifiche», e D14 della parte 1 dice «Specifiche»»*, così il prossimo censimento non la corregge per zelo) |
| R8-26 | 17 · *Files*, che **non** nomina `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md`; e la §10 del disegno del 2, riga *«Codice di prodotto»* della tabella *«Lo stato alla chiusura, e il comando che lo rifà»* | il 17 tocca la stella (Passo 8-bis) e `design/10` (Passo 8-ter), non il disegno del 2 | ⛔ **La §10 del 2 ha il GEMELLO della riga che il Passo 8-bis corregge nella stella, e nessuno lo tocca.** La riga dice *«Codice di prodotto \| **non toccato**: `git diff --stat 664265a..HEAD -- crates/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende nulla»*: dal **compito 1** quel comando rende, e la riga diventa falsa esattamente come *«codice e spec non toccati»* della stella, che il 17 corregge. ⚠️ **E la §10 dice di sé che cosa aspetta:** *«**Arriva:** la parte 2 del piano, che riscrive questa sezione come diario»* e *«**Regge crescendo:** ogni chiusura riscrive lo stato e le righe fatte, nello stesso file»* — nessun passo di nessun compito lo fa (`grep -c 'sottoprogetto-2-gui-minima-design' c17.md` → **1**, ed è la riga della §12). Il 15 e il 16 toccano quel file, ma solo la §8 | CONFERMATO | fatto | no | un **Passo 8-quater** nel 17, nella forma del 8-bis: il richiamo datato in coda alla riga *«Codice di prodotto»* della §10 (*«✅ **RICHIAMO DEL \<data\>, dal compito 17 del piano della parte 2:** da questo piano il codice è toccato — la Definizione di «fatto» dice che cosa, file per file»*), e una riga che dica che la §10 **resta il verbale della sessione che ha scritto i disegni** e non diventa il diario della parte 2, che vive nel piano. ⚠️ Ancora unica da verificare col `grep -c -F` prima di scrivere; il file è **LF** |

---

## Copertura del disegno per il mio perimetro

| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|
| §8 · riga `scripts/gate-gui.sh` — *«`cd` alla radice … `npm ci`, `npm run build`, `npm test`; si ferma al primo rosso»* | 15 · Passo 6 | **contraddetta, non dichiarata nel disegno**: lo script nasce con **cinque** comandi (più `npm run lint`) e il 16 lo porta a **sette**; il commento del Passo 6 lo dice, la §8 no — R8-10 |
| §8 · riga `scripts/gate-gui.sh`, colonna *«La prova»* — *«nelle due direzioni … un test della SPA reso rosso → `GATE RED`»* | 15 · Passo 7 (lo **script**) e Passo 12 (il cancello, **solo verde**) | **scoperta a metà**: `grep -n 'GATE RED' <piano>` → una riga sola, ed è la frase del Passo 7 — R8-7 |
| §8 · riga `scripts/gate-gui.sh` — *«l'etichetta del passo compare nell'uscita del cancello»* | 15 · Passo 12 e criterio | coperta |
| §8 · riga *«la riga in `gate.sh`»* — posto, etichetta inglese | 15 · Passo 8 | coperta (ancora unica, colonna 43 misurata) |
| §8 · riga *«il core finto nel cancello»* — `--locked`, sonde in `main.rs`, costo dichiarato e misurato al piano | 15 · Passo 6 (il comando), Passo 10 (i tempi) | coperta, ma il terzo `<tempo>` non è prodotto da nessun passo — R8-4 |
| §8 · riga *«la versione di Node»* — casa unica, `engine-strict` | 11 (fuori perimetro); 15 · criterio `grep -c 'engines' gui/package.json` → 1 | coperta |
| §8 · riga *«la CI»* — `setup-node`, `node-version-file`, `checkout` resta, niente cache npm | 15 · Passo 9 | coperta, e `package-manager-cache: false` è **più** di quanto la §8 chiedeva (D66, dichiarato) |
| §8 · riga *«`.gitignore`»* | 11 e 12 (le righe `gui/`); le otto di `spikes/gui-shell/` **esistono già** | **spostata**, dichiarata da **D67** e **P-102**; al 15 resta un'asserzione di diff vuoto |
| §8 · riga *«la campagna DST del 2 nel settimo passo»* | 10 (fuori perimetro) | — |
| §8 · riga *«il registro della porta, `docs/porta-di-qualita.md`»* — una sezione nuova sul precedente di S3, **nessuna riga di catalogo** | 17 · Passo 7 | coperta (S3 esiste una volta sola; `grep -c 'PARTE 2'` oggi 0 → 1) |
| §8 · riga *«i documenti: … la roadmap, `README.md`, la §12 del compendio, tracciabilità, i richiami datati»* | 17 · Passi 2–8 | coperta |
| §8 · riga *«il codice fuori dal perimetro»* — *«`git diff --stat` a fine piano tocca solo ciò che le tabelle nominano»* | 17 · Passo 9, il `--name-only` (R9a-16) | coperta |
| §8 · capoverso *«Ciò che la §8 non fa»* — X-1, X-3, il tempo del cancello, il capo a capo, il lint delle scritte | 16 · Passi 5, 6 e il richiamo (X-1, X-3); 15 · Passo 10 (il tempo) e D63/D65 (il lint); il capo a capo **resta fuori** | coperta |
| §8 · i **tre 🔶 dedotti** — `engine-strict`, le prove senza browser, il `target/` del finto | 15 · Passo 11 | coperta (ancora unica, P-104 rimisurata oggi) |
| §8 · riga *«le scritte, `locales/it.json` (G21)»* — *«se al piano esiste una regola di lint matura»* | 15 · Passo 11, secondo richiamo (R9a-12) | coperta (ancora unica, la riga finisce con " \|") |
| §10 · decisione **10** — *«nessuna riga nuova in `README.md`, `roadmap.md`, `tracciabilita.md` e nella §12 del compendio \| compito del piano»* | 17 · Passi 2, 4, 5, 6 | coperta |
| §10 · punto **8** — *«le righe … nella §12 del compendio, in `README.md`, nella roadmap (il titolo della riga 2) e in tracciabilità entrano con essa»* | 17 · Passi 2, 4, 5, 6; il **titolo** della riga 2 è **D86** | coperta |
| §10 · *«La Definizione di «fatto» della parte 1 … la parte 2 ne scrive la propria dopo la misura»* | 17 · Passo 9 (**D74**) | coperta, ma una riga della Definizione rende il numero sbagliato — R8-19 |
| §10 · tabella dello stato, riga *«Codice di prodotto \| **non toccato**»* | **nessuno** | **scoperta** — R8-26 |
| §10 · *«**Arriva:** la parte 2 del piano, che riscrive questa sezione come diario»* | **nessuno** | **scoperta** — R8-26 |
| §6 della stella · *«le righe nella §12 del compendio e in «Dove va cosa» di `README.md` restano compito del piano»* | 17 · Passi 2 e 4, ma in *«Specifiche»* | **contraddetta, dichiarata altrove**: D14 della parte 1 e **P-112** dicono *«Specifiche»*, ed è dove vivono le righe-modello (gesti, knowledge base) — R8-25 |
| §6 della stella · *«la testa … dice che non è una spec»*, *«chi disegna i moduli»*, *«il rimando dal 2»*, *«l'archivio»* | righe di forma della stella, non artefatti del 2 | fuori dal perimetro del piano |
| audit · **X-1** — la CI solo Linux, decisa A il 2026-09-09 | 16 · Passo 5 (matrice + `fail-fast: false`) e Passo 6 (richiamo) | coperta; il numerale *«le due sonde `#[cfg(unix)]`»* resta falso — R8-15 |
| audit · **X-3** — nessuna scansione degli avvisi, decisa A il 2026-09-09 | 16 · Passi 2, 3, 4 e il richiamo del Passo 6 | coperta |

## Voci P rimisurate

| P | Comando rilanciato → resa | Regge? |
|---|---|---|
| **P-2** (le tre della catena `eslint`) | registro npm: `eslint` 10.10.0, `eslint-plugin-vue` 10.11.0, `@intlify/eslint-plugin-vue-i18n` 4.5.1 — tutte e tre ancora **`latest`**, licenze MIT, `engines.node` più larghi di D37 | ✅ regge |
| **P-8** (dove va la riga `run`) | `grep -n 'run "' scripts/gate.sh` → 39–44 e 84, nell'ordine che P-8 elenca; la riga nuova fra 43 e 44 | ✅ regge |
| **P-96** (`vue/no-v-html` nel preset) | `grep -n 'no-v-html' …/dist/configs/flat/vue3-recommended.js` → `"vue/no-v-html": "warn"`; in `flat/essential` → `undefined` | ✅ regge |
| **P-98** (`no-raw-text` è `warn`) | `grep -n "no-raw-text" …/dist/configs/flat/recommended.js` → `'warn'`; e nel mio albero la regola a `error` morde (mutazione 1 → `EXIT=1`) | ✅ regge |
| **P-99** (`multi-word` è `error` nei tre livelli; 85 regole) | `node -e "…"` → `flat/essential error=85 warn=0 \| multi-word: error`; idem strongly-recommended e recommended | ✅ regge; e i `.vue` sono **tredici**, ViewBar l'unico a più parole (R7-11 regge) |
| **P-100** (33 avvisi in `flat/recommended`, tutti di formattazione) | `node -e "…"` → **33**, e i primi sono `attribute-hyphenation`, `first-attribute-linebreak`, `html-indent`, `html-quotes`, `html-self-closing`, `max-attributes-per-line`… | ✅ regge |
| **P-101** (i `.ts` non li guarda nessuno) | `npx eslint src/probe.ts` → `File ignored because no matching configuration was supplied`, `EXIT=0`; `@typescript-eslint/parser` è `optional: true` fra i peer di `eslint-plugin-vue` | ✅ regge **per i `.ts`**, ⛔ **ma la conclusione «nessun analizzatore serve» non regge per i `.vue`** — R8-1; e il rimando *«Passo 4»* è il Passo 2 — R8-9 |
| **P-102** (`.gitignore` e i lockfile dello spike) | `grep -n 'gui-shell' .gitignore \| wc -l` → **8**; `git ls-files 'spikes/gui-shell/**package-lock.json' 'spikes/gui-shell/**Cargo.lock' \| wc -l` → **4** | ✅ regge nei conteggi; ⛔ **non** nell'attribuzione a `8fc9696` — R8-8 |
| **P-103** (`package-manager-cache` vale `true`) | `curl …/setup-node/v7/action.yml` → `default: true` e la descrizione che P-103 cita; `src/util.ts` legge `engines.node` da `package.json` | ✅ regge, alla fonte |
| **P-104** (i tre `target_directory`) | `cargo metadata` su `spikes/rust`, `spikes/gui-ipc` e il workspace → `…\spikes\rust\target`, `…\spikes\gui-ipc\target`, `…\harness\target` | ✅ regge |
| **P-105** (la seconda sonda sopravvive) | `npx eslint` su un `.vue` con `t(\`modules.${id}\`)` e `t("modules.inventato")` → **solo** il letterale fa `error`; la chiave costruita non rende niente | ✅ regge |
| **P-106** (`cargo audit` non installato, ~3 minuti) | `cargo audit --version` → `cargo-audit-audit 0.22.2`: **oggi è installato** | ⚠️ la premessa non regge più (il Passo 1 del 16 prevede entrambi i casi); il costo d'installazione **non verificato** |
| **P-107** (l'avviso `bincode` è ammesso, le due direzioni) | `cargo audit` → `1 allowed warning found`, `EXIT=0`; `cargo audit --deny unmaintained` → `1 denied warning found!`, `EXIT=1`; `git status --porcelain` vuoto | ✅ regge, alla lettera |
| **P-108** (`-n` toglie nove secondi su dieci) | tre misure: `cargo audit` 2.81/2.16/2.08 s, `-n` 0.55/0.55/0.56 s, stesso verdetto | ⛔ **non regge nei numeri** — il ~10 s era la prima clonazione del db (`~/.cargo/advisory-db`, 45 MB, creato il 2026-09-15) — R8-14. ✅ **L'argomento regge**: `-n` è più veloce e cieco |
| **P-109** (la CI non ha `cargo audit`, i minuti sono gratis) | `curl api.github.com/repos/devfrx/daemon` → `private: False`, `visibility: public` | ✅ regge la premessa; i ~3 minuti per job **non verificati** (solo la corsa li dice) |
| **P-110** (`npm audit`: 369 pacchetti, 0 vulnerabilità) | non riproducibile senza `gui/`; sul lockfile del **modello dell'11** (99 pacchetti) → `found 0 vulnerabilities`, `EXIT=0`; e su un albero con `minimist@0.0.8` → `1 critical severity vulnerability`, `EXIT=1` | ✅ regge nella direzione misurabile; il **369** non verificato |
| **P-111** (metà di X-1 si misura solo guardando la corsa) | `grep -nE 'timeout\|nproc\|date \+%s%N\|…' scripts/*.sh` → **nessuna riga**; uno script CRLF gira in questa Git Bash (`bash 5.2.37 msys`, `EXIT=0`) | ✅ regge: non ho trovato comandi che su Windows falliscano per un motivo sbagliato, e le due righe ⬜ restano tali |
| **P-112** (`D14` è quello della **parte 1**) | `grep -n '^\| \*\*D14\*\*' <piano della parte 1>` → *«nella §12 del compendio e nella tabella «Specifiche» di `README.md` … entrano con la parte 2»*; in **questo** piano `D14` è `FileCustody::open` | ✅ regge |
| **P-113** (il conteggio dei compiti tolto dalla roadmap) | col `sed` → **0**, senza → **1**: il criterio morde nelle due direzioni | ✅ regge (R9b-15 regge) |
| **P-114** (sette case CRLF, i due disegni no) | `git ls-files --eol` sulle dieci → sette `i/lf w/crlf`, stella + `design/10` + piano `i/lf w/lf` | ✅ regge |
| **P-115** / **P-6** (il margine è `11030`) | `grep -n '^ceiling=' scripts/check-docs.sh` → `111616`; `wc -c docs/COMPENDIO.md` → `100586`; **111616 − 100586 = 11030** | ✅ regge, al byte |
| **P-116** (`README.md` non nomina nessuno dei due disegni) | `grep -c 'sottoprogetto-2\|direzione-gui\|gui-minima' docs/README.md` → **0** | ✅ regge |

## Numeri di compito ricensiti

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| 15 · *Files*, *«come il compito 10 lo lascia»* (`scripts/gate.sh`, ×4) | 10 | 10 — il 10 aggiunge la riga `serving_campaign` al settimo passo | ✅ |
| 15 · *Consumes*, *«`gui/src/panels/Chat.vue` … (compito **13**, Passo 5»* | 13 | **14** per `Chat.vue`, **13 Passo 5** per le diciotto chiavi | ⛔ R8-5 |
| 15 · *Consumes*, *«`copy.test.ts` e `registry.ts` (compito 13)»* | 13 | 13 | ✅ |
| 15 · *Consumes*, *«`gui/fake-core/Cargo.toml` … (compito 12)»* | 12 | 12 | ✅ |
| 15 · Passo 1, *«Se `gui/src/panels/Chat.vue` non esiste, il compito 14 non è eseguito»* | 14 | 14 | ✅ |
| 15 · Passo 3, commento: *«`PANEL_TYPES` registry (task 13)»* | 13 | 13 | ✅ |
| 15 · Passo 3, commento: *«the `modules.*` key of the locale (task 14)»* | 14 | **13**, Passo 5 (R6-4, 2026-09-16) | ⛔ R8-6 |
| 15 · Passo 3, commento di `.js`/`type: module`: *«compito 11»* | 11 | 11 | ✅ |
| 15 · Passo 5, commento: *«Task 13 wrote two probes here»*, *«Task 15 replaced the first one»*, *«`keys.test.ts` already exists (task 14)»* | 13, 15, 14 | 13, 15, 14 | ✅ |
| 15 · Passo 6, commento: *«measured by task 12 (step 10)»* | 12, Passo 10 | 12, Passo 10 — che misura davvero freddo e caldo e dice *«che il compito 15 scrive»* | ✅ |
| 15 · Passo 11, *«P-64 … compito 11»*, *«il Passo 3 del compito 13»* | 11, 13 | 11 (`engine-strict`), 13 Passo 3 (*«l'ambiente delle sonde»*) | ✅ |
| 15 · *«il compito 16 gli aggiunge `npm audit` in coda»*, *«nessuna matrice … che sono il 16»*, *«le sonde si registrano al compito 17»* | 16, 16, 17 | 16, 16, 17 | ✅ |
| 16 · *Consumes*, *«(compito 15)»* ×3 e *«i compiti 11, 13, 14 e 15»* | 15; 11,13,14,15 | idem | ✅ |
| 16 · *«il criterio di chiusura del 16 manda a guardarla»*, *«la divergenza … sta nel Passo 9 del compito 15»* | 16, 15 Passo 9 | 16, 15 Passo 9 (`actions/checkout`) | ✅ |
| 16 · *«una divergenza fra i due è lo script di confronto del compito 12»* | 12 | 12, Passo 10 (lo script che confronta i due lockfile, D83) | ✅ |
| 16 · *«nessuna riga di catalogo in `docs/porta-di-qualita.md`, che è il **17**»* | 17 | 17 | ✅ |
| 17 · *«le righe X-1 e X-3 chiuse dal compito **16**»* | 16 | 16 | ✅ |
| 17 · Passo 8-bis, *«compito 7 del piano della parte 2»* (l'accoglienza e il riinvio) | 7 | 7 — `kernel::serving`, D5 | ✅ |
| 17 · Passo 8-bis, *«compiti 5, 7 e 13»* (il `Layout` che torna) | 5, 7, 13 | 5 (le due implementazioni della custodia), 7 (`keep` → `Layout`), 13 (`VIEWS`, `settle`, D80/D89) | ✅ |
| 17 · Passo 8-bis, *«compiti 1–16»* (l'ordine dei pezzi) | 1–16 | 1–16 | ✅ |
| 17 · Passo 8-ter, *«Il 6 ha costruito `InvocationDetail` … e l'8 `PolicyDetail`»* | 6, 8 | 6 (indice 3), 8 (indice 4) — **D25** ha diviso la vecchia riga 8, e i due numeri sono quelli di oggi | ✅ |
| 17 · Passo 10, *«il Passo 15 del compito 14»* (il richiamo di D56) | 14, Passo 15 | 14, Passo 15 — e il `RECALL` di quel passo produce **esattamente** la stringa che il `grep` del 17 cerca | ✅ |
| 17 · Passo 1 e 9, *«i **sedici** criteri di chiusura, uno per compito»* | sedici | sedici **esistono**, ma per **diciassette** compiti: manca quello del **7** | ⛔ R8-21 |

## Non verificato, e perché

- **Il costo d'installazione di `cargo audit`** (**P-106**, *«2m 53s»*, **D68**): l'attrezzo è già installato su questa
  macchina e disinstallarlo per rimisurare avrebbe cambiato lo stato dell'ambiente. Il piano prevede entrambi i casi.
- **I ~3 minuti per job di `cargo install` in CI** (**P-109**, **D70**) e **tutto ciò che solo la corsa può dire** (**P-111**):
  che `windows-latest` onori `rust-toolchain.toml`, che i due job non si annullino, che `bash scripts/gate.sh` arrivi a
  `GATE GREEN` là. È esattamente ciò che P-111 dichiara non misurabile da terra, e il criterio del 16 manda a guardarlo.
- **I 369 pacchetti di `npm audit`** (**P-110**): l'insieme vero nasce coi compiti 11, 13, 14 e 15. Ho misurato la direzione
  (`0 vulnerabilities` sul lockfile del modello dell'11, 99 pacchetti) e la contro-direzione (`minimist@0.0.8`).
- **I tempi del cancello col passo web** (15 · Passo 10): richiedono `gui/` e `gate-gui.sh`, che non esistono.
- **`vitest`, `vue-tsc` e `npm run build` sui file dettati**: fuori dal mio perimetro (sono i compiti 11–14, R6 e R13); io ho
  compilato **solo** la catena `eslint`, che è l'artefatto del 15.
- **Se `actions/checkout@v4` lasci CRLF o LF sul runner Windows**: non misurabile da qui. ⚠️ Il rischio che temevo è
  comunque **escluso a valle**: uno script CRLF gira in questa Git Bash senza errori, quindi anche se il runner lo
  consegnasse CRLF il cancello partirebbe.
- **Il contenuto delle sette case oltre le righe che i *Trova* del 17 nominano**: non l'ho letto, come il mandato prescrive.
  Del compendio ho misurato peso, tetto, la §12 (intestazione e forma delle righe), le tre righe col ⏭️ e i conteggi dei
  criteri; niente altro.

## Stato finale

`git -C /c/Users/zagor/Desktop/harness status --porcelain` → **(vuoto)**, rilanciato a fine revisione.
`git ls-files --eol` del piano e dei due disegni → `i/lf w/lf` **invariato**; `docs/design/10-modello-dei-dati-durevoli.md`
→ `i/lf w/lf`; le sette case del 17 → `i/lf w/crlf`; `scripts/gate.sh`, `.github/workflows/quality-gate.yml` e
`docs/audit-2026-08-27.md` → `i/lf w/crlf`, con CR = righe (97/97, 16/16, 1885/1885).
Nessun file del repository è stato creato, modificato o cancellato: ogni prova vive in
`<scratchpad>/review/probe-R8/` (la catena `eslint` installata, i tredici `.vue` estratti dal piano, `gate-gui.sh` e
`gate.sh` ricostruiti, il flusso di lavoro estratto, i due alberi `npm audit`).

**Conteggio dei rilievi**, contato sulle righe delle tabelle col comando
(`grep -cE '^\| R8-[0-9]+ \|' R8-report.md`) e non dall'elenco che sto scrivendo: **26**.
**CONFERMATI 24**; **NON RIPRODOTTI 2** — `R8-24` e `R8-25`.
**Bloccanti 10**: `R8-1`, `R8-2`, `R8-3`, `R8-4`, `R8-6`, `R8-11`, `R8-13`, `R8-18`, `R8-19`, `R8-20`.
Per compito: **15** → dieci (`R8-1`…`R8-10`), **16** → sette (`R8-11`…`R8-17`), **17** → nove (`R8-18`…`R8-26`).
