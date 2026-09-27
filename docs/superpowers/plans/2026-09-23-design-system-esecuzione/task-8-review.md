# La revisione del compito 8 — la cornice: la barra col nome della vista, la Panoramica, la striscia a pillola

> Revisore: subagente fresco, nessuna riga scritta da me fra quelle rivedute. Commit rivisto: `ac56b11`, sopra `bc6e94d`.
> Macchina `Jays`, il 2026-09-27, dalle 20:48 alle 21:24; Node `v24.19.0`; Chrome `154.0.8037.58`, letto dal nome della
> cartella (`ls "/c/Program Files/Google/Chrome/Application/"`). Clone di lavoro: `C:\Users\Jays\AppData\Local\Temp\rv8`,
> lasciato a `ac56b11`, `git status --porcelain` vuoto. Log, script e schermate nello scratchpad
> `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d19536a1-7be4-437b-8e11-39fed58af059\scratchpad\review8\`
> (sotto: `<review8>`). Nell'albero del repository ho scritto solo questo file, che `.superpowers/sdd/.gitignore` ignora —
> il cancello, com'è chiesto, rifà `gui/node_modules` e `gui/dist`, ignorati; gli script Python del dispaccio li ho lanciati
> con `PYTHONDONTWRITEBYTECODE=1` —; `git status --porcelain` vuoto dopo.

## 1. Il verdetto

**Conforme al dettato** — `ac56b11` è il testo del piano byte per byte (`compare_task8.py` e `plan_ops.py`, ciascuno
provato nelle due direzioni), ogni Atteso rifatto torna, le trentadue righe del Passo 9 cadono come dettano, il cancello è
verde; ma **guardando la SPA** ho trovato due difetti che nessuna prova coglie e che il Passo 10 del proprietario incontrerà
alla lettera — la miniatura di Compatta dice il falso (**E103**), e l'ingrandimento di un gruppo arriva al pacchetto solo per
caso (**E104**) —, e quattro minori del dettato.

| Classe | Quanti | Voci candidate |
|---|---|---|
| Critico | 0 | — |
| Importante | 2 | **E103**, **E104** |
| Minore | 4 | **E105**, **E106**, **E107**, **E108** |
| Nit | 2 | **E109**, **E110** |
| la candidata del rapporto | 1, confermata | **E102** (Nit), §2.3 |

## 2. I rilievi

⚠️ Il codice è dettato: nessun rilievo è una cura per l'implementatore. Ciascuno è una **voce d'errata candidata**, col
testo proposto; la prossima libera dopo la **E102** del rapporto è la **E103**. Le cure non le ho applicate.

### 2.1 Importanti

#### I-1 — E103 (candidata): la miniatura di Compatta dice il falso

- **Dove.** `grep -n '"size"' gui/src/panels/views/compact.json` → righe **20** e **33**, `"size": 500` ciascuna: la
  knowledge base e la striscia a metà del quadrato. `grep -n 'minimumHeight: 56, maximumHeight: 56'
  gui/src/panels/views/generate-views.test.ts` → riga **95**, la striscia di Compatta. `grep -n 'tiles = schematic(saved)'
  gui/src/frame/Overview.vue` → riga **59**. Nel piano, `grep -n 'la knowledge base a tutta pagina'` → riga **337**, **D18**.
- **Il fatto, misurato** sul server di sviluppo del clone a `ac56b11`, Chrome installato, 1440 × 900, dopo
  `harnessFake.deliverAll()` (`<review8>/probe8d.mjs`): la miniatura di Compatta ha **una** tessera,
  `height: calc(50% - 2 * var(--space-0-5))`; sullo schermo, scelta Compatta dalla Panoramica, il gruppo della knowledge
  base è alto **690 px, il 91,8 %** del dock, la striscia **50, il 6,6 %**. Home e Lavoro disegnano la riga della striscia al
  5,6 % (le tessere arrivano al 94,4 % in tutte e due). La schermata `<review8>/shots/light-24-compatta-on-screen.png` contro
  `light-06-overview-after-delivery.png`: mezza miniatura vuota.
- **Perché.** `dockview` stringe la striscia ai suoi 56 px quando dispone la vista; `schematic` legge le misure **scritte**,
  e il generatore scrive Compatta prima che una disposizione le corregga. Misurato sotto jsdom nel clone
  (`<review8>/consumer/zz-review-compact.test.ts`, `<review8>/consumer-compact.log`): com'è il generatore, le foglie `[500,
  500]`; con `api.layout(1600, 1000, true)` dopo la striscia, `[944, 56]`.
- **La regola rotta.** Il vincolo 1: la risposta 19 vuole miniature che *«dicono sempre il vero»*, e **D18** ne dichiara il
  costo come *«la miniatura di Compatta mostra la knowledge base a tutta pagina»* — misurato, a metà pagina. Nessuna prova lo
  vede: la prova delle miniature (*«draws each view in miniature from its layout…»*) guarda la sola Home.
- **Testo proposto.** ⚠️ **Compito 8 — la miniatura di Compatta disegna la knowledge base a metà: `compact.json` porta le
  due foglie a 500 su 1000, perché il generatore scrive la vista prima che `dockview` applichi alla striscia i suoi 56 px; lo
  schermo la stringe, lo schema no.** Due strade: **A**, nel generatore `api.layout(1600, 1000, true)` prima di `write` —
  misurato: Compatta `[944, 56]` —, poi `REGENERATE_VIEWS=1` e, nella prova delle miniature, una seconda attesa per **ogni**
  vista spedita — le tessere senza la striscia arrivano in basso almeno fin dove comincia la riga della striscia —, rossa sul
  file di oggi per Compatta; **B**, `schematic` che rispetta i vincoli di una foglia. Consiglio la **A**: il file spedito dice
  il falso anche a chi lo legge, e la **B** complica lo schema per un caso solo. Il costo della A: i tre JSON rigenerati, da
  guardare riga per riga (Home e Lavoro dovrebbero restare uguali: **dedotto**, non misurato). Il richiamo datato in **D18**.

#### I-2 — E104 (candidata): ingrandire e ripristinare un gruppo non arriva al pacchetto, e la miniatura della vista sullo schermo lo contraddice

- **Dove.** `grep -n 'api.onDidLayoutChange' gui/src/frame/dock.ts` → riga **117**: il `settle` parte solo da lì.
  `grep -n 'exitMaximized\|maximize()' gui/src/frame/BigTab.ts` → righe **46**–**47**, la presa grande. Di prima del compito
  8: il `settle` e i due comandi della presa vengono dalla parte 2; la Panoramica lo rende visibile, e il Passo 10 lo guarda
  (**E85**, **E101**).
- **Il fatto, misurato** sotto jsdom nel clone, `dockview-core` 8.3.1 (`<review8>/consumer/zz-review-maximize.test.ts`,
  `<review8>/consumer-maximize.log`): reso attivo il gruppo di Stato → eventi `active`, `layout`, un `SaveLayout`; poi
  `maximize()` → il solo evento `maximized:true`, **nessun** `SaveLayout` in più; poi `exitMaximized()` → il solo
  `maximized:false`, **nessun** `SaveLayout`. E nel browser (`<review8>/probe8.mjs`, `probe8b.mjs`), 1440 × 900:
  - ingrandito un gruppo **non** attivo, il pacchetto ha `maximizedNode` (la mossa passa per il cambio del gruppo attivo); poi
    ripristinato, **nessun** salvataggio: il pacchetto resta ingrandito. Lavoro, poi di nuovo Home dalla Panoramica → Home si
    riapre **ingrandita**, la striscia alta 2 px a y 124;
  - ingrandito il gruppo **già** attivo (un clic dentro Stato, poi la presa), nessun salvataggio: lo schermo mostra Stato solo,
    e la miniatura di Home nella Panoramica ne disegna **sei** tessere — `<review8>/shots/light-21-group-maximized.png` contro
    `light-22-overview-with-maximized-group.png`, e lo stesso nello scuro.
- **Perché.** In `main.esm.mjs` di `dockview-core` 8.3.1 `maximizeGroup` chiama `doSetGroupActive`, e l'evento del cambio
  del gruppo attivo è ciò che fa salvare; `exitMaximizedGroup` no; `onDidMaximizedGroupChange` esiste e scatta nei due versi
  (misurato qui sopra), ma `createDock` non lo ascolta.
- **La regola rotta.** Il vincolo 1: la risposta 19 e la cura **E85** — *«una vista con un gruppo ingrandito»* disegnata sola
  — valgono solo quando l'ingrandimento cambia il gruppo attivo; e la decisione 12, il salvataggio quando la disposizione si
  assesta: il ripristino si perde al primo cambio di vista. ⚠️ **Il Passo 10** chiede al proprietario proprio *«ingrandito un
  gruppo con la sua presa, la miniatura che lo disegna solo»*: l'esito dipende da quale gruppo era attivo.
- **Testo proposto.** ⚠️ **Di prima del compito 8 — l'ingrandimento non è un `onDidLayoutChange` in `dockview-core`
  8.3.1: si salva solo quando cambia il gruppo attivo, e il ripristino mai.** In `createDock` il corpo del `settle` diventa
  una funzione, e l'ascolta anche `api.onDidMaximizedGroupChange`; una prova sotto jsdom accanto a quelle del dock di
  `frame.test.ts`: reso attivo un gruppo, `maximize()` spedisce un `SaveLayout` col `maximizedNode`, `exitMaximized()` uno
  senza — rossa oggi sul secondo e sul terzo conto (misurato qui sopra). Il costo: un ascoltatore in più, e un `settle` a ogni
  ingrandimento, che la decisione 12 vuole. Da decidere **prima** del Passo 10, o da dire al proprietario prima dello sguardo.

### 2.2 Minori

#### M-1 — E105 (candidata): «Salva questa vista» spenta si vede accesa

- **Dove.** `grep -n ':disabled="layout.arrivals === 0"' gui/src/frame/Overview.vue` → riga **208**;
  `grep -n 'color: var(--color-mark);\|color: var(--color-text-muted);' gui/src/components/BaseLabel.vue` → righe **28**, **31**.
- **Il fatto, misurato** (`<review8>/probe8c.mjs`), prima e dopo `harnessFake.deliverAll()`: il pulsante passa da
  `rgb(163, 154, 143)` — `--color-text-disabled`, nel chiaro — a `rgb(27, 23, 24)`, ma le parole restano
  `rgb(101, 91, 87)` (`--color-text-muted` di `BaseLabel`) e l'icona `rgb(122, 31, 46)` (`--color-mark`) **nei due stati**;
  nello scuro lo stesso (`rgb(163, 154, 143)` e `rgb(191, 85, 103)` fermi). Cambia il solo cursore. Le schermate
  `light-03-overview-F3-before-delivery.png` e `light-06-overview-after-delivery.png` sono uguali a vista.
- **La regola rotta.** La (b): un pulsante spento prende `--color-text-disabled` — la specie di **E19**, dove una regola più
  pesante vinceva sullo spento; qui vince il colore che `BaseLabel` scrive sul proprio elemento. Il Passo 10 chiede di
  vederla spenta prima e accesa dopo (**E98**): oggi non si vede.
- **Testo proposto.** Nel kit, non nella Panoramica, perché vale per ogni `BaseLabel` dentro un `BaseButton` spento: in
  `BaseButton.vue`, dopo `.base-button:disabled` (`grep -n '.base-button:disabled {'` → riga **78**),
  `.base-button:disabled :deep(.base-label), .base-button:disabled :deep(.base-icon) { color: inherit; }` col perché nel
  commento; e nel browser — la prova *«draws the overview…»* consegna la risposta prima di aprire (**E98**), quindi una sua
  gemella, o la stessa aperta anche **prima** di `receive` — le parole della carta `[data-card="save"]` contro
  `computed("color", "--color-text-disabled")`: rossa oggi, misurato che il colore è quello del `muted`. ⚠️ Il peso del selettore contro `.base-label :deep(.base-icon)` è **dedotto**, da misurare con la prova.

#### M-2 — E106 (candidata): il campo del nome non dice all'occhio che cosa vuole

- **Dove.** `grep -n "overview.name" gui/src/frame/Overview.vue` → riga **196**, il solo `:label`; `grep -n 'the icon and
  the' gui/src/components/BaseTextField.vue` → riga **9**: *«the board draws no visible label, the icon and the placeholder
  speak to the eye»*.
- **Il fatto, visto** in `<review8>/shots/light-10-naming-open.png` e `dark-10-naming-open.png`: un campo vuoto con
  l'icona del segnalibro, «Annulla» e «Salva»; il nome accessibile c'è (`aria-label`, e `axe` è verde), una parola visibile
  no — nessun segnaposto.
- **La regola rotta.** Il contratto del pezzo stesso — senza segnaposto all'occhio parla la sola icona — e WCAG 3.3.2
  (*Labels or Instructions*), che vuole le istruzioni presentate a chi vede.
- **Testo proposto.** In `Overview.vue` il `BaseTextField` del nome prende anche `:placeholder="$t('overview.name')"` — la
  chiave c'è, *«Nome della vista»* —, e nella prova *«says under the field…»* un'attesa sul `placeholder` dell'`input`;
  rossa oggi (`null`). Il costo: nessuna parola nuova.

#### M-3 — E107 (candidata): undici righe col loro perché che nessuna prova tiene, e che la tabella del Passo 9 non nomina

Ogni mutazione da sola, sulla **suite intera** nel clone, poi la copia salvata e `cmp` uguale (`<review8>/violations8.py`
con `ROWS_FILE=<review8>/extra_rows.py`, log `<review8>/extra8.log`). ✅ Tenute: tolto `@focus="roving = index"` →
rossa la prova delle frecce, `expected [ -1, -1, -1, +0 ] to deeply equal [ +0, -1, -1, -1 ]`; `asking` senza il volo →
tre prove di `modules.test.ts`. **Verdi** (234 passate, nessuna caduta) sotto ciascuna di queste:

| # | La riga, ritrovata col `grep -n` | Che cosa promette il suo commento |
|---|---|---|
| X2 | `Overview.vue:210` `@focus="roving = cards.length"` | **D17**: nel giro del Tab la carta che le frecce hanno raggiunto — la prova guarda il giro del Tab solo dopo essere tornata su Home |
| X3 | `Overview.vue:116` `roving.value = Math.max(0, …)` | *«Every opening starts from the view on screen»*, e **D17** *«il fuoco apre sulla vista che si vede»*: le prove aprono sempre con Home, l'indice 0 di partenza |
| X4 | `Overview.vue:113` `naming.value = false;` (con `name.value = "";`) | *«…with no name half written»* |
| X5 | `Overview.vue:130` `!from.hasAttribute("data-card")` | *«ONLY FROM A CARD: in the name's field the arrows move the caret»* — senza, le frecce nel campo portano via il fuoco |
| X6 | `Frame.vue:27` `event.preventDefault();` di F3 | *«its default, the browser's "find next", is not ours to keep»* |
| X7 | `Overview.vue:84` `layout.openNamed === null &&` | una sola carta in bordeaux: senza, con una vista col nome aperta ne sono due |
| X8 | `Overview.vue:143`–`144` `cancel()` | «Annulla» chiude il campo e rende il fuoco alla carta |
| X9 | `Drawer.vue:24` `@click="drawer.open = false"` | «Chiudi» chiude il cassetto dal negozio (R3-20) — la prova del browser lo chiude con Esc |
| X10 | `Overview.vue:140` il fuoco al campo | il nome si scrive subito |
| X12 | `Strip.vue:45` `padding: var(--space-1) var(--space-1) …` | **P-24**: il pulsante *«the same distance from its edge all round»* — con `var(--space-3)` a destra la sonda dei raggi lo giudica «fuori dall'angolo» e lo assolve |
| X14 | `Overview.vue:221` `.view-name :deep(.base-label)` | le parole del nome della vista nel colore del testo (aspetto) |

X11 — il filtro `card !== from` di `onArrow` — è verde anch'esso, ma è **dichiarato** da **E99 (c)**.

- **Testo proposto.** Minore — **Compito 8 — righe col loro perché senza prova**. Poche attese, in quattro prove che ci sono
  già, tengono le promesse della (d) e di **D17**: nella prova delle frecce, dopo `ArrowDown` il giro del Tab `[-1, -1, -1, 0]` (X2), e dopo
  `Enter` su Lavoro di nuovo F3 col fuoco su Lavoro (X3); nella stessa, Invio su «Salva questa vista» e il fuoco nel campo (X10),
  `ArrowLeft` nel campo e il fuoco che resta lì (X5), «Annulla» e il fuoco sulla carta (X8); sotto jsdom, nella prova della vista
  col nome, `aria-current` su quella sola (X7), un `keydown` di F3 `cancelable` con `defaultPrevented` vero (X6), e nella prova
  del cassetto «Chiudi» che lo chiude (X9). X4, X12 e X14 **dichiarati**, come **E89**: aspetto o ripristino che nessun caso
  d'uso oggi produce. Classificata Minore perché **D17** è decisione del piano e la sua prova la dichiara tenuta
  (*«One card in the tab order: the one the arrows reached»*, `frame.browser.test.ts:230`).

#### M-4 — E108 (candidata): l'oracolo `computed` risponde per un token che la pagina non ha, e porta il nome di quello di Vue

- **Dove.** `grep -n 'export function computed' gui/src/testing/probes.ts` → riga **165**; `grep -n 'A TOKEN THE PAGE DOES
  NOT DEFINE' gui/src/tokens/readToken.ts` → riga **6**: *«A TOKEN THE PAGE DOES NOT DEFINE IS AN ERROR, NOT AN EMPTY
  STRING»*.
- **Il fatto, misurato** dal consumatore nel browser (§4): `readToken("--color-txet")` lancia; `computed("color",
  "--color-txet")` rende il colore del `body`, `computed("border-top-left-radius", "--radius-crad")` rende `"0px"`,
  `computed("background-color", "--color-bg-srface")` rende `"rgba(0, 0, 0, 0)"` — proprio il valore che una prova *«non
  dipinge nulla»* attende. E un file che vuole `computed` di Vue e l'oracolo senza rinominarne uno non si carica: `PARSE_ERROR
  Identifier 'computed' has already been declared` (`<review8>/consumer-clash.log`).
- **Perché conta ora.** **E92** l'ha portato in *una casa*, da aiutante locale a interfaccia comune di tre file: un nome di
  token sbagliato dà un oracolo plausibile, e una prova verde falsa.
- **Testo proposto.** In `testing/probes.ts`, `computed` chiama prima `readToken(token)`, che lancia sul token che manca, col
  perché nel commento (**E73**, la regola di `readToken`); una prova a mano in `probes.browser.test.ts` che un token
  inesistente lancia. Il nome: facoltativo, `tokenValue` — tre file da toccare.

### 2.3 La candidata del rapporto: E102 — confermata

- **Il fatto regge, per la ragione detta.** `grep -n 'npx vitest run --project jsdom src/frame/frame.test.ts'` sul piano →
  riga **10124** (Passo 3, quattro file) e **10928** (Passo 7, tre file); **E93** dice *«e `stores.test.ts` nei comandi dei
  Passi 3 e 7»* (riga **262** del piano), e il recinto del Passo 7 non l'ha ricevuto. Misurato nel clone, **sulla sequenza**
  dei passi (applicati fino al 2, rosso del Passo 3, poi i Passi 4–7 sopra, senza ripulire): com'è scritto `Test Files  3
  passed (3)`, `Tests  40 passed (40)`; con `src/stores/stores.test.ts` `Test Files  4 passed (4)`, `Tests  61 passed (61)`.
- **La cura del rapporto è giusta, e l'alternativa no.** Il comando coi quattro file, come al Passo 3: la prova di
  `showNamed` vista rossa al Passo 3 va vista verde al Passo 7, com'è il ciclo del piano, e il testo torna a dire ciò che
  **E93** afferma già fatto. Portare l'Atteso a tre file e a 40 lascerebbe falsa **E93** e senza verde il codice del Passo 5.
  Da aggiungere al testo: che **E93** dichiarava corretto anche il Passo 7; e che la riga cambia senza spostare righe, quindi
  i numeri della ricetta (`W`/`R`, fino alla 10905) restano validi. Nit, come la classifica il rapporto.

### 2.4 Nit

#### N-1 — E109 (candidata): quattro commenti che il compito lascia a metà

- `grep -n 'each returns how much it looked at' gui/src/testing/probes.ts` → riga **5**: la testa del file dice che le sonde
  vengono dalle tavole e che *ciascuna* rende quanto ha guardato; `computed` non viene da una tavola e rende un valore
  (come già `firstFamily`).
- `grep -n 'turn it ON' gui/src/testing/axe.ts` → riga **10**: *«the probes of the kit page … turn it ON with `contrast:
  true` (task 4)»* — dal compito 8 anche la prova della Panoramica (`frame.browser.test.ts:200`).
- `grep -n 'task 8 adds the strip and the overview' gui/src/frame/frame.browser.test.ts` → riga **20**: il compito aggiunge
  anche la barra (**E57**, **E39**), il cassetto dalla tastiera (**E38**), la fascia `stop` (**E70**) e l'anello (**E100**).
- `grep -n 'it the store holds no package' gui/src/frame/Overview.vue` → riga **152**: prima della risposta il negozio può
  tenerne uno — un `settle` del dock prima del benvenuto lo costruisce, la domanda di classe di **E40**, registrata.
- **Testo proposto.** Le quattro frasi riscritte senza conto né elenco chiuso: *«the probes, and the oracles they share»*;
  *«the probes in the real browser turn it ON»*; *«born with task 6bis …; task 8 extends it»*; *«before it the store holds no
  package of the core's»*. Nessuna prova cambia.

#### N-2 — E110 (candidata, d'aspetto, per lo sguardo): «Salva questa vista» non è la carta della tavola

- `grep -n 'data-card="save"' gui/src/frame/Overview.vue` → riga **207**; nella tavola della Panoramica `grep -n
  'ov-card.add{'` → riga **237**: `align-items:center; justify-content:center; border-style:dashed; min-height:108px`.
- **Visto:** da sola sulla sua riga è una carta piena alta 42 px con le parole in alto a sinistra; con una vista col nome
  accanto si allunga all'altezza di una carta con la miniatura, vuota — `<review8>/shots/light-14-overview-with-named-view.png`
  e `dark-14-overview-with-named-view.png`. È la variante `card` del kit (**P-9**, compito 3), non un errore del compito; ma
  la (d) dice *«com'è nella tavola»* (risposta 13).
- **Testo proposto.** Nessuna cura scritta: la forma la decide il Passo 10 (controllo 15). Se il proprietario vuole la
  tavola: `align-self: start` sulla carta e il tratteggio al centro, da misurare.

## 3. I comandi rilanciati, con le uscite

Ogni comando accanto alla sua uscita; dove il rapporto ne dice una, è la stessa, salvo dove è scritto.

| # | Comando (sotto `/e/ALL/DEV/MY_REPOS/daemon` o nel clone) | Uscita mia | Il rapporto |
|---|---|---|---|
| 1 | `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline bc6e94d..HEAD` | `ac56b11`; vuoto; una riga | — |
| 2 | `git status -sb` | `## main...origin/main [ahead 1]` | niente push ✅ |
| 3 | `git log -1 --format=%B ac56b11 \| grep -ci co-authored` | `0` | `0` ✅ |
| 4 | `git show --stat ac56b11` | 20 file, `902 insertions(+), 123 deletions(-)`, due `create mode 100644` | uguale ✅ |
| 5 | `PYTHONIOENCODING=utf-8 python …/compare_task8.py bc6e94d ac56b11` | 20 `OK`, `--- 20 paths, 0 mode changes, 0 not matching the plan's text`, uscita **0**; né il piano né il disegno | uguale ✅ |
| 6 | lo stesso nel clone su un commit usa-e-getta, un carattere in un commento di `Overview.vue` (`W`) | `DIFFERS` su `gui/src/frame/Overview.vue` **e nessun altro**, `1 not matching`, uscita **1** | — |
| 7 | lo stesso, un carattere nel commento di testa di `invoke.ts`, fuori dal testo sostituito (`R`) | `DIFFERS` su `gui/src/stores/invoke.ts` e nessun altro, uscita **1**; il clone tornato a `ac56b11`, pulito | — |
| 8 | `python plan_ops.py <review8>/plan-base.md 8 list` | `39 operations`, le righe della ricetta | 39 ✅ |
| 9 | `<review8>/recipe_vs_ops.py`: la ricetta di `compare_task8.py` contro `plan_ops.parse` | `recipe lines: 39  plan_ops ops: 39`, `identical, in order: True` | — |
| 10 | `plan_ops.parse` sul brief e sul piano base | 39 e 39, stessi testi, tipi, percorsi, passi | uguale ✅ |
| 11 | clone a `bc6e94d` pulito, `plan_ops … apply <clone>`, `git add -A`, `git diff --cached --stat ac56b11 -- gui/` | `39 operations, 0 refused`; diff **vuoto**; `git diff --cached ac56b11 \| wc -c` → `0`, `--summary` vuoto | — |
| 12 | clone pulito, `apply --upto 2`, poi `npx vitest run --project jsdom src/frame/frame.test.ts src/stores/stores.test.ts src/a11y.test.ts src/locales/copy.test.ts` | `23 operations, 0 refused`; `Failed to resolve import "./stores/drawer"` in `a11y.test.ts`, `"../stores/drawer"` in `frame.test.ts`, `TypeError: layout.showNamed is not a function`, `Test Files  3 failed \| 1 passed (4)`, `Tests  1 failed \| 24 passed (25)` | uguale ✅ |
| 13 | `npx vitest run --project browser src/frame/frame.browser.test.ts src/frame/dock.browser.test.ts src/kit/kit.browser.test.ts` | `Tests  10 failed \| 39 passed (49)`: il dock `expected '21px' to be '9999px'` ×2; la pillola `expected 21 to be greater than or equal to 25` ×2; il cassetto e la Panoramica `expected null not to be null` ×2 ciascuno; le frecce `expected null not to be null`; l'anello `expected 9 to be greater than or equal to 13` | uguale ✅ |
| 14 | sopra, senza ripulire, `apply --only-step 4`, `5`, `6`, `7`; `git add -A` e `diff --cached ac56b11` | `7`, `5`, `2`, `2 operations, 0 refused`; **0 byte** di differenza | — |
| 15 | il comando jsdom del Passo 7 com'è scritto; poi coi quattro file | `Tests  40 passed (40)`; `Tests  61 passed (61)` | uguale ✅ (E102) |
| 16 | il comando browser del Passo 7 | `Test Files  3 passed (3)`, `Tests  49 passed (49)` | uguale ✅ |
| 17 | tre corse di `npx vitest run --reporter=json` nel clone a `ac56b11`, una alla volta | 28 file, **234** passate, 0 cadute, 1 saltata, ciascuna | cinque corse, 234 ✅ — i suoi cinque JSON riletti: 28, 234, 0, 1 |
| 18 | la suite intera a `bc6e94d` nel clone, poi i nomi delle prove contro la corsa 1 | base **213** passate; spariti **4** nomi, le due rinominate che il compito detta (*«draws every group as a card, `--space-3`…»* e *«lays the band on the page…»*), due temi; nuovi 25 = 21 prove + 4 rinominate | «ventuno in più» ✅ |
| 19 | le trentadue righe del Passo 9, `<review8>/violations8.py`, sulla suite intera, una alla volta | **32 su 32** rosse coi messaggi della tabella e sulle **sole** prove che nominano (le tre di **E95** anche sulle prove in più); `cmp` uguale dopo ciascuna, anche `BaseDialog.vue` e `Band.vue`; `git status` come prima (vuoto). Log `<review8>/violations8.log`, JSON in `<review8>/viol/` | uguale ✅ |
| 20 | le mutazioni in più, §2.2 M-3 | `<review8>/extra8.log` | — |
| 21 | `bash scripts/gate.sh`, da solo, in background, sull'albero del repository | `GATE GREEN.`, uscita 0; jsdom `Test Files  21 passed \| 1 skipped (22)`, `Tests  169 passed \| 1 skipped (170)`; browser `Test Files  6 passed (6)`, `Tests  65 passed (65)`; `dist/assets/index-D1K6_br5.js  698.14 kB │ gzip: 212.98 kB`; `found 0 vulnerabilities` — log `<review8>/gate-review.log` | uguale ✅ |
| 22 | `bash scripts/check-docs.sh` | `OK — no inconsistencies.`, uscita 0 | uguale ✅ |
| 23 | `diff` dei nomi dei file di prova, `git ls-tree -r --name-only <base\|HEAD> -- gui/src \| grep '\.test\.ts$'` | uguali, 28 file, 6 del browser | «gli stessi» ✅ |
| 24 | `git ls-files --eol` e `tr -cd '\r' \| wc -c` sui venti file, e sui blob di `bc6e94d` | tutti `i/lf w/lf`, 0 CR, prima e dopo; righe come nella tabella del rapporto (275, 488, 287…); i due nuovi finiscono con `\n` | uguale ✅ |
| 25 | `git diff --stat bc6e94d..ac56b11 -- crates/ gui/schema/`; `-- gui/package.json gui/package-lock.json` | vuoti (vincoli 12 e 7); `reka-ui` `2.10.4`, `vitest` `4.1.11` | ✅ |
| 26 | il Passo 1 a `bc6e94d`: `git grep -nw F3`, `git cat-file -e …drawer.ts`, `git grep -n 'bar\.views'` | nessun `F3` con `-w` (senza, `#F3EEE6` di `themes.css`, **E91**); `drawer.ts` assente; `bar.views` nel solo `ViewBar.vue:17` | uguale ✅ |
| 27 | i log del cancello del rapporto: `ls -la --time-style=full-iso` contro `git log -1 --format=%ci ac56b11` (`20:41:46`) | `gate-passo1-…-202731.log` scritto 20:28:49 (`693.02 kB`, 157+56, verde), `gate-commit-…-203929.log` 20:40:49 (`698.14 kB`, 169+65, verde), `check-docs.log` 20:41:07: dell'implementatore, e prima del commit; `.path` 20:27:31 e 20:39:29 → 78 e 80 s | uguale ✅ |

⚠️ **Una nota sulla tabella del Passo 9, senza voce:** la riga delle frecce scrive `expected <button …(8)>…(2)</button> to
be <button …(7)>…(2)</button>`, e `vitest` stampa `<button data-v-2d3a2488 …(8)>…`: il primo attributo, l'identità dello
stile con ambito, è tolto. È l'abbreviazione giusta — quell'identità cambia col file —; lo dico perché chi confronta il
messaggio con una sottostringa esatta lo trova diverso, come è successo al mio script (`message=False`, riga 19).

## 4. Il consumatore da fuori — 8(c)

Tre file usa-e-getta nel clone, fuori da ciò che il compito ha scritto, lanciati e tolti (`git status --porcelain` vuoto
dopo), più tre diagnostiche scritte per fallire — `zz-review-diag.browser.test.ts`, `zz-review-maximize.test.ts` (**E104**),
`zz-review-compact.test.ts` (**E103**) —; le copie in `<review8>/consumer/`, i log in `<review8>/consumer-*.log`.

| File | Che cosa fa | Che cosa ha reso |
|---|---|---|
| `zz-review-consumer.test.ts` (jsdom) | un componente **terzo** apre il cassetto con `useDrawer()`, F3 non apre la Panoramica, «Chiudi» lo chiude dal negozio e F3 allora apre; `asking` letto da fuori — falso con la richiesta del core senza un volo nostro (**D59**), vero col volo, falso dopo `refuse()`; `showNamed` prima di ogni pacchetto, con un nome ignoto, con `" revisione "`, col nome esatto, e `showView` che la chiude; `Overview.vue` montata da una cornice futura con la sua `snapshot` e `v-model:open`, salvata con l'invio del modulo; `ViewBar.vue` col clic sul nome; le chiavi di `it.json` | **6 su 6 verdi** (`<review8>/consumer-jsdom.log`): `showNamed` rende `false` prima del pacchetto, per `"Assente"` e per `" revisione "` — il confronto è esatto, coerente con `unpack` (`named.some((entry) => entry.name === open)`), mentre `saveNamed` usa `sameName`: una chiamante futura che passi un nome scritto a mano lo deve prendere dalla lista —; la `snapshot` chiamata **una** volta, `update:open` a `false`, `openNamed` `"Mia"`; il clic sul nome emette `update:overview` `[true]`, `aria-keyshortcuts` `F3` |
| `zz-review-consumer.browser.test.ts` | `computed` su un token che c'è e su tre che non ci sono; `contrastJudged` e `violations` su `.bar`, `.band`, il gruppo della striscia e quello di Stato; le due in `Promise.all` | `computed("border-top-left-radius", "--radius-card")` → `21px` ✅ (il mio confronto con `readToken` era sbagliato: `readToken` rende il `calc(…)` scritto, non il valore); i tre token inesistenti → il colore del `body`, `0px`, `rgba(0, 0, 0, 0)`, e `readToken` lancia — **E108**; `.bar` e `.band` giudicate, `passes` > 0 e `incomplete` 0, nessuna violazione; il gruppo della striscia `passes` **0** — vedi sotto; `Promise.all` → rifiutata, *«Axe is already running»*: un limite di `axe`, che nessun file usa in parallelo, e le due funzioni non lo dicono |
| `zz-review-clash.browser.test.ts` | `computed` di Vue e l'oracolo nello stesso file, senza rinominare | non si carica, `PARSE_ERROR Identifier 'computed' has already been declared` — e, messo accanto agli altri, fa fallire la scansione delle dipendenze di **tutto** il progetto `browser` (`Failed to scan for dependencies`, `Vite unexpectedly reloaded a test`): un file rotto ne trascina altri, da sapere |

⚠️ **`contrastJudged` sul dock della cornice non giudica niente** — diagnostica scritta per fallire,
`<review8>/consumer-diag.log` e `consumer-diag2.log`: su `.dock`, sul gruppo di Stato e su quello della striscia, nei due
temi, `passes` **0** e `incomplete` **25**, tutti *«Element's background color could not be determined because it is
overlapped by another element»*; ma `document.elementsFromPoint` al centro della prima scritta della striscia mette in cima
lo `span` stesso, poi `.strip`, `.panel`, `.dv-content-container`, `.dv-groupview`; e con `.dock` a `overflow: visible` o
`hidden` invece di `clip` il conto non cambia. Quindi non è **E43**: è `axe` 4.13 che non sa leggere lo sfondo dentro
`dockview`. L'aiutante fa ciò che il suo nome promette — dice che il contrasto **non** è stato giudicato —; oggi nessuna prova
lo chiede sul dock (il contrasto del dock lo tiene il frammento del Passo 8 del compito 6, sotto), ma chi ce lo porterà lo
troverà rosso per questa ragione, non per un colore.

## 5. Che cosa ho visto guardando la SPA

Il server di sviluppo del clone (`npm run dev -- --port 5199 --strictPort`, PID 34344), il **Chrome installato** senza
finestra, **con le barre di scorrimento accese** (`ignoreDefaultArgs: ["--hide-scrollbars"]`), 1440 × 900, uno script
Playwright nello scratchpad (`<review8>/look8.mjs`, `playwright` dal `node_modules` del clone con `createRequire`; nessun
download). Il tema: `colorScheme` del contesto **e** `document.documentElement.dataset.theme`, riletto dopo la consegna.
Poi il server fermato per PID (`taskkill //PID 34344 //T //F`), e nessun `node.exe` né Chrome senza finestra rimasto
(`Get-CimInstance Win32_Process`: 0 e 0). Le schermate in `<review8>/shots/`, `light-NN-…` e `dark-NN-…`; i numeri in
`<review8>/shots/look8.json` e `<review8>/look8.log`. Ho guardato ogni schermata; l'aspetto lo giudica il proprietario.

| Che cosa | Chiaro | Scuro |
|---|---|---|
| **La barra** (`01`, `02`) | a sinistra «HOME» in maiuscoletto con l'icona `views` in bordeaux, dentro un pulsante col bordo; a destra la ricerca spenta col segnaposto **intero** (314,9 px nei 334 del campo) e il chip «Core: in attesa»; nessun fondo né riga sotto | uguale; il pulsante del nome col bordo scuro |
| **F3 prima della consegna** (`03`) | la Panoramica a tutta finestra: «LE VISTE», la riga di aiuto, tre carte in fila con le miniature — Home, Lavoro, Compatta — e «Salva questa vista» sotto Home; Home corrente col bordo bordeaux e, col fuoco, l'anello fuori; `scrollHeight` 900 su 900, niente sborda; «Salva questa vista» **spenta** (`disabled`) ma **uguale a vista** a quella accesa (**E105**) | uguale |
| **Esc** (`04`) | la Panoramica chiusa, il fuoco sul nome, con l'anello | uguale |
| **Il clic sul nome** | apre la Panoramica, quattro carte | uguale |
| **Dopo `harnessFake.deliverAll()`** (`05`, `06`) | la fascia `stop` *«Il core parla una versione diversa del protocollo…»* col timbro, il chip «Core: timbro diverso» in rosso; la Panoramica uguale a prima, «Salva questa vista» ora `disabled: false` | uguale |
| **Le frecce** (`07`, `08`) | → Lavoro, ↓ «Salva questa vista» sotto Home, ↑ Home, ← resta su Home; il giro del Tab `["0","-1","-1","-1"]`, dopo → `["-1","0","-1","-1"]`; ⚠️ il bordo della carta corrente e l'anello del fuoco sono **due anelli bordeaux** che si distinguono per i 2 px di distanza — per lo sguardo | uguale, in rosa |
| **Invio su Lavoro** (`09`) | la vista Lavoro, com'è la sua miniatura; «LAVORO» nella barra, il fuoco sul nome | uguale |
| **Il nome di una vista nuova** (`10`–`13`) | il campo col fuoco e l'icona, **nessuna parola visibile** (**E106**), «Annulla» e «Salva»; vuoto → *«Scrivi un nome.»* in rosso sotto, bordo d'errore; *«home»* → *«C'è già una vista con questo nome.»*, e il bordo resta sotto il puntatore (**E20**); *«Revisione»* con Invio → la Panoramica si chiude, «REVISIONE» nella barra col fuoco | uguale; l'errore in rosso-arancio |
| **La vista salvata** (`14`, `15`) | quattro carte e «Salva questa vista»; «Revisione» corrente, con *«salvata»* a destra del nome; il fuoco apre su di lei; «Salva questa vista» accanto si allunga all'altezza della carta, vuota (**E110**); Home, poi «Revisione» riaperta dalla Panoramica | uguale |
| **La striscia** (`16`) | la pillola a 12 px dai lati e 24 dal fondo, alta 50, `9999px`; «moduli» a pillola, alto 40, a **5 px** sopra, sotto e a destra | uguale |
| **Il cassetto** (`17`, `18`) | col clic si apre il foglio «I MODULI»; Esc → il fuoco torna a «moduli» (**P-22**); Invio → di nuovo aperto, il fuoco su «Chiudi» a y 855–887 dentro il foglio 334–900 (**E38**), la barra del foglio visibile a destra | uguale |
| **Il Tab su Stato** (`19`, `20`) | `section.status` col fuoco, `outline` `solid 2px`, `outline-offset: -4px`: l'anello **dentro** la scheda sui quattro lati, gli angoli di sotto tondi come la scheda (**E100**) | uguale, in rosa |
| **Un gruppo ingrandito** (`21`–`23`) | Stato a tutto il dock; ⚠️ la **striscia sparisce** sotto l'ingrandimento (è `dockview`: una vista ingrandita è un gruppo solo), e con lei «moduli»; la miniatura di Home **non** lo disegna solo — sei tessere —, perché qui Stato era già attivo dal Tab (**E104**); in `probe8.mjs`, da un gruppo non attivo, la miniatura lo disegna solo sul quadrato intero (`width: calc(100% …)`, `height: calc(100% …)`) | uguale |
| **Compatta** (`light-24`, `probe8d.mjs`) | sullo schermo la knowledge base al 91,8 % del dock; la miniatura la disegna al 50 % (**E103**) | — |
| **La console** | nel chiaro un `404` di una risorsa, al caricamento — non indagato: quale risorsa non l'ho letto, e a vista non manca nulla; nello scuro nulla | — |

**Il frammento del contrasto** del Passo 8 del compito 6, dopo la consegna, com'è scritto (`.dock *`) e con la Panoramica
aperta (`.base-dialog[data-variant="full"] *`, la radice che `Overview.vue` disegna con `BaseDialog variant="full"`):

| | `.dock *` | la Panoramica |
|---|---|---|
| chiaro | `seen` **35**; `worst` **5,39** *«chiesti 4096 MiB, te»*, 5,7, 5,7 | `seen` **6**; `worst` **5,7** *«Le viste»*, 5,7 *«F3 apre e chiude. Le»*, 6,22 *«Home»* |
| scuro | `seen` **35**; `worst` **5,4** *«chiesti 4096 MiB, te»*, 6,41, 6,41 | `seen` **6**; `worst` **6,41** *«Home»*, 6,41, 6,41 |

Il primo di `worst` sopra 4,5 e `seen` sopra zero in tutti e quattro.

## 6. Ciò che non ho potuto verificare, e perché

- **L'aspetto** — i caratteri, il maiuscoletto del nome della vista, i due anelli bordeaux, la carta «Salva questa vista»,
  le tessere scure nello scuro: lo giudica il proprietario al Passo 10 (controllo 15); qui è descritto, non giudicato.
- **Le cure proposte** di **E103**…**E108** non sono scritte né provate per intero: di **E103** ho misurato il solo
  generatore di Compatta (`[944, 56]` con la disposizione in più), non la rigenerazione dei tre file; di **E104** l'evento
  (`maximized:true`/`false` senza `layout`), non l'ascoltatore nuovo; il peso del selettore di **E105** è dedotto.
- **Ctrl+Alt+frecce con la Panoramica aperta:** `onArrow` guarda il solo `event.key`, e `Frame.onKey` non guarda le finestre
  aperte — **dedotto** dal codice, non misurato —: la freccia sposterebbe il fuoco fra le carte **e** la tessera del dock
  sotto il velo. È la voce 🔶 che il piano sa, del proprietario; il compito 8 vi aggiunge solo che la Panoramica non filtra i
  modificatori.
- Nella prima corsa dello sguardo, dopo un ingrandimento ripristinato, il salvataggio di una vista col nome lasciò il fuoco
  sul `body`; nella sequenza pulita (`probe8.mjs`, con Invio e col clic) torna al nome. Non riprodotto, e non inseguito.
- **La macchina `zagor`**, la **CI** dopo il push, e il **lettore di schermo**: non da qui.
