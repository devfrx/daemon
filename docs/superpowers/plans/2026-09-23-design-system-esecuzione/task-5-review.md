# La revisione del compito 5 — `545f500`, il kit al lavoro

Revisore del compito 5, macchina `Jays`, il 2026-09-26 dalle 14:19 alle 15:00 circa. Rivisto **un** commit, `545f500`
sopra `39827e8`. Le mutazioni nel clone `C:\Users\Jays\AppData\Local\Temp\rv5`, lasciato al suo posto per il
coordinatore: alla fine è a `545f500`, staccato, `git status --porcelain` vuoto — i commit usa-e-getta delle sonde di
`compare_task5.py` restano solo nel suo reflog, non referenziati. Log, script e immagini nello scratchpad
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\b62a978b-28b5-4b69-963c-b58323e34338\scratchpad\review5\`
(qui sotto: `review5\`). Nell'albero del repository non ho scritto nulla fuori da questo file, che è ignorato: alla fine
`git status --porcelain` è vuoto e `## main...origin/main [ahead 1]`. Chrome `154.0.8037.58` dal nome della cartella,
all'inizio e alla fine; Node v24.19.0. Nessun `node.exe` e nessun Chrome senza finestra lasciati accesi.

## 1. Il verdetto

✅ **Conforme**: il commit è il dettato del piano byte per byte — `compare_task5.py`, provato nelle due direzioni, dà OK
sui diciotto percorsi —, ogni Atteso dei Passi 1, 2, 6 e 7 torna quando lo rilancio nella sequenza dei passi, e ogni cifra
del rapporto torna. I rilievi sono **uno Importante**, **tre Minori** e **tre Nit**, nessun Critico. Sei su sette
riguardano il **dettato**: cinque sono voci d'errata candidate, **E38**–**E42**, e una, N-2, si può registrare; il
settimo è una frase del rapporto. I due più
pesanti li ho trovati **guardando** la SPA, e nessuna prova li vede: il cassetto che si apre col fuoco fuori dalla vista, e
il segnaposto della ricerca che non dice più chi la riempie. Tutti e due sono regressioni rispetto a `39827e8`, misurate, e
il compito 8 le eredita col suo stesso testo.

## 2. I rilievi

| # | Classe | Dove | In una riga | Voce |
|---|---|---|---|---|
| I-1 | Importante | `gui/src/frame/Drawer.vue:16-27`, `gui/src/components/BaseDialog.vue:73-76` | il cassetto aperto dalla tastiera dà il fuoco a «Chiudi», che sta sotto il bordo del foglio e non si vede: la regola «focus» della (a) — visibile e non nascosto — rotta, e a `39827e8` si vedeva | **E38** |
| M-1 | Minore | `gui/src/frame/ViewBar.vue:61`, `:29` | il segnaposto della ricerca è tagliato a «…arriva col sotto-p»: il numero di chi la riempie (decisione 16) sparisce, e a `39827e8` ci stava | **E39** |
| M-2 | Minore | `gui/src/panels/Settings.vue:63`, `gui/src/stores/layout.ts:99-100` | un tema scelto prima che arrivi il pacchetto del core manda `layouts: {}`: la premessa di **E2** non la tiene il codice | **E40** |
| M-3 | Minore | `gui/src/components/Confirm.vue:19-21` | «Escape e il velo sono il no» (ADR-0016) non lo tiene nessuna prova — da prima del compito, ma la strada è nuova | **E41** |
| N-1 | Nit | `gui/src/panels/Settings.vue:60-62`, `:72` | la `BaseStatus` vuota fra i due gruppi raddoppia il `gap`: 32 px dove altrove sono 16 | **E42**, o registrata |
| N-2 | Nit | `gui/src/frame/Band.vue:25`, `gui/src/frame/ViewBar.vue:32` | altre due righe col perché e senza prova — «Riprova» (D48) e la ricerca spenta (decisione 16) —, tutte e due già così a `39827e8` | registrabili |
| N-3 | Nit | `task-5-report.md`, §7 punto 1 | *«Le tre `.base-status` nel DOM, vuote»* al caricamento: quella della fascia porta la fascia | — |

### I-1 — Importante — il cassetto si apre col fuoco su un «Chiudi» che non si vede — voce candidata E38

**Dove.** `grep -n 'variant="sheet"\|BaseList :items\|#actions' gui/src/frame/Drawer.vue` → `16:`, `20:`, `26:`;
`grep -n 'max-height: 60vh\|overflow: auto' gui/src/components/BaseDialog.vue` → `75:`, `76:`;
`grep -n preventScroll gui/node_modules/reka-ui/dist/FocusScope/utils.js` → `72: element.focus({ preventScroll: true });`.
Nel piano a `545f500`: il recinto del Passo 3, `grep -n 'variant="sheet">' <piano>` → `4389:`, e quello del compito 8,
`7574:`, con la stessa forma.

**Evidenza.** `review5\sheet.cjs` e `review5\sheet_base.cjs`: Chrome installato, senza finestra, 1440 × 900, la SPA del
clone su `npm run dev`, `Accepted`, il fuoco su «+ moduli» e **Invio**; poi, per la base, la sola `Drawer.vue` rimessa com'era
a `39827e8`, e ricopiata dopo, `cmp` uguale:

| `Drawer.vue` | `scrollHeight` / `clientHeight` del foglio | il foglio, y | «Chiudi», col fuoco, y | si vede? |
|---|---|---|---|---|
| di `545f500` | 702 / 564 | 334–900 | 993–1025 | **no** — `scrollTop` 0, anche dopo un Tab |
| di `39827e8` | 482 / 482 | 417–900 | 863–884 | sì, col contorno |

Le diciotto righe di `BaseList` sono più alte degli `<li>` di prima e non stanno più nei `60vh` del foglio; `reka-ui`
2.10.4 dà il fuoco al primo elemento raggiungibile — «Chiudi», l'ultimo — con `preventScroll: true`, e il foglio non
scorre. Chi usa la tastiera apre il cassetto e non vede dove sta il fuoco. È la riga «focus» della (a), merito approvato:
*«un contorno … visibile (2.4.7) e non nascosto (2.4.11)»* — per questo Importante e non Minore, benché **E19** ed **E20**,
difetti di colore, fossero Minori. 🔶 Dedotto: i 678 px di contenuto stanno nei `60vh` solo con la finestra alta almeno
1130 px (678 / 0,6), quindi su uno schermo da 1080 righe il difetto c'è. Esc, il fuoco che torna a «+ moduli» e
«Chiudi» col mouse funzionano, visti nei due temi. Il compito 8 riscrive `Drawer.vue` con la stessa forma, e la sua prova
nel browser apre il cassetto col **clic** e guarda solo il ritorno del fuoco: il difetto sopravvive al compito 8.
Immagini: `review5\shots\dark-04-drawer-open.png`, `light-04-drawer-open.png` (col fuoco su «Chiudi», che non c'è);
`dark-04f-sheet-focus-BASE-drawer.png` (la base).

**Cura proposta, in testo.** È di `BaseDialog`, non del cassetto: la forma `sheet` tiene in vista le sue azioni. Provata
nel clone, poi tolta, `cmp` uguale: in `gui/src/components/BaseDialog.vue`, dopo la regola `.actions`,

```css
.base-dialog[data-variant="sheet"] .actions {
  position: sticky;
  bottom: 0;
  background: var(--color-bg-raised);
}
```

→ «Chiudi», col fuoco, a y 855–887, visibile, nei due temi (`review5\shots\dark-12-sheet-CURE-sticky.png`). ⚠️ Così com'è,
sotto la barra resta il `padding` del foglio, 12 px, dove si vede passare la riga dopo: la forma finale si misura — per
esempio il corpo in uno scorrimento suo, titolo e azioni fuori. Dove la cura atterra — una cura di questo compito sul
file del compito 3, o il compito 8 — lo decide il coordinatore. Come chiede il dispaccio della revisione, **non** propongo
una prova del cassetto: il compito 8 ne porta una.

**Voce candidata E38.** *⚠️ Compito 5, Passo 3 — il cassetto aperto dalla tastiera dà il fuoco a «Chiudi» fuori dalla
vista: con `BaseList` le diciotto righe non stanno nei `60vh` del foglio `sheet`, e `reka-ui` 2.10.4 mette il fuoco con
`preventScroll: true`. Misurato dalla revisione il 2026-09-26, Chrome 154, 1440 × 900: «Chiudi» a y 993–1025 col foglio a
334–900 e `scrollTop` 0; con la `Drawer.vue` di `39827e8` a 863–884, visibile. Rompe la riga «focus» della (a). Il compito 8
ne eredita la forma. Cura: la forma `sheet` di `BaseDialog.vue` tiene in vista le azioni — provata con `position: sticky`,
«Chiudi» a 855–887 —, nella forma che la misura sceglie.*

### M-1 — Minore — il segnaposto della ricerca non dice più chi la riempie — voce candidata E39

**Dove.** `grep -n 'max-width: 320px\|SAYING WHO FILLS IT' gui/src/frame/ViewBar.vue` → `29:` il commento *«⚠️ DISABLED AND
SAYING WHO FILLS IT, not hidden: decision 16 of the north star wants the search box to say who fills it»*, `61:  max-width:
320px;`. Nel piano: `grep -n 'max-width: 320px' <piano>` → `4769:` (compito 5) e `7970:` (compito 8).

**Evidenza.** `review5\bar_measure.cjs`, 1440 × 900: la larghezza del testo dal carattere calcolato dell'`input`, contro lo
spazio dell'`input` meno `padding` e bordi; la base con `ViewBar.vue` e `Settings.vue` di `39827e8`, ricopiate dopo, `cmp`
uguale:

| `ViewBar.vue` | carattere | serve | c'è | a video |
|---|---|---|---|---|
| di `545f500` | `14px "Geist Variable"` | 315 px | 270 px | *«la ricerca sugli artefatti arriva col sotto-p»* |
| di `39827e8` | `13.3333px Arial` | 288 px | 312 px | intero, col 6 |

Il campo di `BaseTextField` ha il carattere del kit e l'icona, e nei 320 px il numero del sotto-progetto non entra più: il
commento della riga 29 dice ancora che la casella dice chi la riempie (gotcha **#58**, nella sua forma visiva). Nei due temi
uguale: il carattere non cambia col tema. Immagini: `review5\shots\dark-11-bar-committed.png`, `dark-11b-bar-BASE.png`, e
ogni `*-01-load.png`. Il compito 8 tiene `max-width: 320px` (riga 7970 del piano).

**Cura proposta, in testo.** **A** — `max-width: 24rem;` al posto di `max-width: 320px;` nella regola `.search`, nel
recinto del compito 5 (riga 4769) e in quello del compito 8 (riga 7970): provata nel clone, 334 px contro 315, nei due
temi, poi tolta, `cmp` uguale. **B** — una `bar.searchHint` più corta in `it.json`. Consiglio **A**: tocca un numero scritto
a mano dal piano, come il 320; **B** tocca le parole approvate della parte 2.

**Voce candidata E39.** *Compito 5, Passo 6 — il segnaposto della ricerca tagliato: in `BaseTextField` (14 px, Geist, e
l'icona) servono 315 px e ce ne sono 270, e a video resta «…arriva col sotto-p»: la decisione 16 della stella polare vuole
che la casella dica chi la riempie, e a `39827e8` ci stava (288 su 312). Misurato dalla revisione il 2026-09-26. Cura:
`max-width: 24rem` nei recinti dei compiti 5 e 8 — 334 px, misurato.*

### M-2 — Minore — un tema scelto prima del pacchetto del core manda `layouts: {}` — voce candidata E40

**Dove.** `grep -n 'BaseRadioGroup :model-value="layout.theme"' gui/src/panels/Settings.vue` → `63:`, senza `:disabled`,
mentre la policy ce l'ha, `57:`; `grep -n 'keep({ layouts: {}' gui/src/stores/layout.ts` → `100:`. Nel piano, **E2**, riga
170: *«Dal compito 5 la scelta si fa in Impostazioni **dopo** l'arrivo del pacchetto»*.

**Evidenza, misurata.** Una prova usa-e-getta nel clone, `review5\zz-before-package.test.ts`, scritta per fallire così da
stampare ciò che misura, poi cancellata, `git status --porcelain` com'era: col negozio in `state: "Nothing"` e `saved`
`null`, i tre radio del tema sono accesi, `[false, false, false]`; un clic su «Scuro» manda `SaveLayout` col pacchetto
`{ layouts: {}, view: "home", theme: "dark" }`; poi `deliver("Layout")`, e il tema torna `system`. Nella SPA, al
caricamento, il gruppo è acceso mentre la fascia dice *«Il core non ha risposto.»* (`review5\shots\dark-01-load.png`).

**Dedotto, non misurato — qui non c'è un core vero.** Con un core lento (D48: lento e assente sono uno stato), il core
risponde a `Hello` col pacchetto che tiene, e poi custodisce il `SaveLayout` che segue: le disposizioni salvate diventano
`layouts: {}` — il male di **E2**, per il tempo invece che per lo *spread*; e la sua risposta sono i byte che la SPA ha
mandato, che `receive` prende per l'eco (`sameBytes(message.value.bytes, sent)`) e non mostra. ⚠️ Il `settle` del dock ha
la stessa finestra dalla parte 2 — una mossa prima del benvenuto manda la sola vista aperta, `gui/src/frame/dock.ts:88` —:
la domanda è di classe, e del proprietario.

**Cura proposta, in testo.** **A** — il gruppo del tema spento finché non è arrivato un `Layout` (il benvenuto conta in
`layout.arrivals`), come la policy, *«the rest off»* (§6a), con una prova nelle due direzioni; ⚠️ la prova *«chooses the
theme…»* dovrà consegnare prima un `Layout`. **B** — il negozio tiene la scelta e la fonde col pacchetto quando arriva. E
la stessa domanda sul `settle`, per il proprietario.

**Voce candidata E40.** *⚠️ Compito 5, Passo 5 — la premessa di E2 non la tiene il codice: il gruppo del tema è acceso
prima del benvenuto, e `chooseTheme` con `saved` nullo manda `{"layouts":{},…}` — misurato dalla revisione il 2026-09-26
sotto jsdom; che il core perda così le disposizioni è dedotto. La stessa finestra per il `settle` del dock, dalla parte 2.
Del proprietario: il gruppo spento fino al benvenuto, o la scelta tenuta e fusa all'arrivo.*

### M-3 — Minore — «Escape e il velo sono il no» senza una prova — voce candidata E41

**Dove.** `grep -n 'Escape and the veil\|if (value !== true)\|@update:open' gui/src/components/Confirm.vue` → `19:`, `21:`,
`32:`; la prova del no, `grep -n 'sends nothing on a no' gui/src/panels/modules.test.ts` → `267:`, clicca «Rifiuta» (`274:`).

**Evidenza.** Con `void value;` al posto di `if (value !== true) invoke.refuse();`, `npm test` nel clone → `Tests  158
passed | 1 skipped (159)` (`review5\why-2.log`). Già così a `39827e8`: la `Confirm.vue` di prima aveva la stessa riga e
nessuna prova le mandava Escape. Ma la strada è nuova — il `defineModel` di `BaseDialog`, `update:open` con
`boolean | undefined` —, ed è la riga di ADR-0016: *nulla si concede col silenzio*. Nella SPA funziona: Esc chiude e non
manda niente, nei due temi (`sentAfterEsc: 0` in `review5\shots\walk.json`).

**Cura proposta, in testo.** Una terza prova in `describe("the confirmation window", …)` di `modules.test.ts`, provata nel
clone (`review5\zz-escape.test.ts`, poi cancellata): verde sul commit, `Tests  1 passed (1)`; rossa con `void value;`,
`AssertionError: expected { tool: 'arbiter', …(2) } to be null`.

```ts
  it("takes Escape for a no: sends nothing, and closes (ADR-0016)", async () => {
    const { bridge, core, invoke } = wire();
    const wrapper = mount(Confirm, { global: { plugins: [i18n] }, attachTo: document.body });
    invoke.send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    bridge.deliver("PermissionRequired");
    await nextTick();
    await nextTick();
    // ⛔ NON-VACUITY: the window is open, or Escape would have nothing to refuse.
    expect(document.querySelector('[role="dialog"]')).not.toBeNull();
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    await nextTick();
    await nextTick();
    await nextTick();
    expect(bridge.sent.map((message) => message.kind)).toEqual(["Invoke"]);
    expect(core.pending).toBeNull();
    expect(document.querySelector('[role="dialog"]')).toBeNull();
    wrapper.unmount();
  });
```

### N-1 — Nit — la regione vuota di Impostazioni raddoppia lo spazio — E42, o registrata

**Dove.** `gui/src/panels/Settings.vue:60-62`, la `BaseStatus` fra i due `BaseRadioGroup`; `:72`, `gap: var(--space-4);`.

**Evidenza.** `review5\bar_measure.cjs`, sulla colonna di `section.settings`: `base-radio-group->base-status: 16px`,
`base-status->base-radio-group: 16px`, `base-radio-group->who: 16px` — 32 px fra i due gruppi finché nessuna chiamata è in
volo, 16 altrove (`review5\shots\dark-05-policy.png` contro `dark-05b-policy-arrow.png`). È l'osservazione 1
dell'implementatore, confermata. Non è una scatola — la regione è alta zero —, ma occupa un posto nella colonna.

**Cura proposta, in testo.** Il gruppo della policy e la sua regione in un contenitore solo, col suo spazio, così la
sezione ha un `gap` solo fra i gruppi. ⛔ Non `display: none` sulla regione vuota, né `display: contents`: M-3 vuole la
regione nell'albero dell'accessibilità **prima** delle parole.

### N-2 — Nit — due righe col perché e senza prova, di prima — registrabili

Tolta una alla volta nel clone (`review5\why_lines.py`), `npm test` resta `Tests  158 passed | 1 skipped (159)`:
`gui/src/frame/Band.vue:25`, l'`@click="connection.retry()"` di «Riprova» (D48, commento alle righe 6-8), e
`gui/src/frame/ViewBar.vue:32`, il `disabled` della ricerca (decisione 16, commento alla riga 29). Già senza prova a
`39827e8`: nei file di prova di allora il solo `retry` è quello del negozio, `stores.test.ts:40`. Il compito 8 riscrive la
barra e tiene la ricerca spenta (riga 7942 del piano). Le altre righe col perché dei componenti riscritti fanno cadere la
loro prova: la tabella in §3, riga 15.

### N-3 — Nit — una frase del rapporto

§7, punto 1: *«Le tre `.base-status` nel DOM, vuote»*, detto del caricamento. Misurato (`review5\shots\walk.json`, passo
`01-load`, nei due temi): quella della fascia porta *«Il core non ha risposto.Riprova»*, alta 41 px; le altre due sono
vuote e alte zero. Nessuna conclusione cambia: le tre prove di M-3 e la loro direzione rossa tornano.

## 3. I comandi rilanciati

| # | Che cosa | Comando, in breve | La mia uscita | Il rapporto |
|---|---|---|---|---|
| 1 | l'avvio | `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline 39827e8..HEAD` | `545f500`; vuoto; una riga | — |
| 2 | il cancello, sull'albero, da solo | `bash scripts/gate.sh` → `review5\gate-review.log`, 14:20:16–14:21:29, `real 1m13.540s` | `GATE GREEN.`; jsdom `Test Files  18 passed \| 1 skipped (19)`, `Tests  130 passed \| 1 skipped (131)`; browser `Test Files  4 passed (4)`, `Tests  28 passed (28)`; `index-BS90oxvQ.js 689.58 kB │ gzip: 209.99 kB`; `index-BIBdFfwk.css 151.78 kB`; `found 0 vulnerabilities`; `OK — no inconsistencies.` | uguale, riga per riga |
| 3 | la base | `grep` su `gate-baseline.log` del coordinatore | `Tests  125 passed \| 1 skipped (126)`, `Tests  27 passed (27)`, `index-CiZv4zPX.js 663.93 kB`, `index-Bx6RaF8P.css 145.10 kB` | uguale |
| 4 | i log dell'implementatore sono suoi | `ls -la --time-style=full-iso` contro `git log -1 --format=%ci 545f500` = `2026-09-26 14:15:47 +0200` | `gate-open-…` scritto 14:02:10, `gate-commit-…` 14:15:16, `check-docs.log` 14:15:34: tutti prima del commit, e le loro righe `Tests` sono quelle del rapporto | — |
| 5 | nessuna prova sparita, per nome | `git ls-tree -r --name-only` dei `*.test.ts` alle due basi, e i titoli `it(`/`describe(` dei quattro file toccati | un solo file in più, `settings.browser.test.ts`; nessun titolo perso; cinque nuovi sotto jsdom — tre in `modules.test.ts`, uno in `frame.test.ts`, uno in `copy.test.ts` — e uno nel browser: 125 → 130, 27 → 28 | *«cinque prove nuove sotto jsdom … e una nel browser»* |
| 6 | `compare_task5.py`, verde | `PYTHONIOENCODING=utf-8 python …/compare_task5.py 39827e8 545f500` | 18 × `OK`, il piano `OK (the one cell of step 9)`, `--- 18 paths, 0 mode changes, 0 not matching the plan's text`, uscita 0 | — |
| 7 | `compare_task5.py`, rosso | nel clone, un carattere di un commento di `Confirm.vue` (`piece` → `piecE`), commit `c890f0e` | `DIFFERS  gui/src/components/Confirm.vue` e nessun altro, `1 not matching`, uscita 1 | — |
| 8 | le altre forme dello script | `review5\probe_compare.py`, sette commit usa-e-getta sopra `545f500` | `X`: `DIFFERS` solo `modules.test.ts`; `S`: solo `a11y.test.ts`; `RL`: solo `Placeholder.vue`; la riga 5 a ✅: `DIFFERS` sul piano; una riga in più nel piano: `CHECK BY HAND`, uscita 0; `theme.ts` toccato: `UNEXPECTED`; un cambio di modo: `UNEXPECTED mode change 100644 => 100755`; uscita 1 in tutti gli altri | — |
| 9 | il Passo 1, nella sequenza | nel clone i tredici file non di prova com'erano a `39827e8`, copie salvate prima; `npx vitest run --project jsdom --reporter=verbose src/panels/modules.test.ts src/a11y.test.ts src/frame/frame.test.ts` | `Test Files  2 failed \| 1 passed (3)`, `Tests  8 failed \| 29 passed (37)`: le sei di Impostazioni — `expected [] to have a length of 2 but got +0`, `expected undefined to be 'true'`, `Unable to get [role="status"] within: <section … class="settings">`, `expected [] to deeply equal [ 'true', 'false' ]`, `expected false to be true`, `expected [] to deeply equal [ 'true', 'false', 'false' ]` —, M-3 di Stato `Unable to get [role="status"] within: <section … class="status">`, M-3 della fascia `… within: <!--v-if-->`; verdi le due della finestra di conferma e le undici di `a11y.test.ts` | uguale, prova per prova |
| 10 | il Passo 1, nel browser | `npx vitest run --project browser src/panels` | `Tests  1 failed (1)`, `AssertionError: expected [] to have a length of 2 but got +0` | uguale |
| 11 | il Passo 2 | `copy.test.ts` del commit, `it.json` di prima; poi `it.json`; poi `sepia` in `theme.ts`, copia e `cmp` | `system: expected [] to include 'system'`, `Tests  1 failed \| 2 passed (3)`; `Tests  3 passed (3)`; `sepia: expected [ 'system', 'light', 'dark' ] to include 'sepia'`; `cmp equal` | uguale |
| 12 | il Passo 6 | i file tornati dalle copie, `cmp` uguale a `HEAD` per tutti e tredici; `npm test && npm run build` | `Test Files  22 passed \| 1 skipped (23)`, `Tests  158 passed \| 1 skipped (159)`, `index-BS90oxvQ.js 689.58 kB` | uguale |
| 13 | la direzione rossa di M-3 | `review5\m3_red.py`, un pannello per volta, `npm test` intero | ciascuna `Tests  1 failed \| 157 passed \| 1 skipped (159)`, la sola prova di M-3 di quel pannello, `Unable to get [role="status"] within: <!--v-if-->` / `<section … class="settings">` / `<section … class="status">`; `cmp equal to HEAD` dopo ognuna | uguale |
| 14 | il Passo 7 e il confine | `review5\lint_red.py`, `npm run lint`, una violazione per volta | righe 1–3: uscita 1, **un errore solo**: `12:5 error a button is BaseButton…`, `17:5 error a list is BaseList…`, `2:1 error 'reka-ui' import is restricted from being used. panels and frame use the base pieces…`; riga 4: uscita 0. Il confine: `lucide` in `panels/Strip.vue` e in `frame/moveActive.ts` → un errore, `'lucide' import is restricted … icons pass through BaseIcon and the one map…` (il messaggio di prima); `<ol><li></li></ol>` in `panels/Strip.vue` → `a list is BaseList`; in `components/Confirm.vue` un `<button>`, un `<ul>` e un import di `reka-ui` insieme → uscita 0. Senza il blocco (`eslint.config.js` di `39827e8`) le righe 1–3 → uscita 0 ciascuna (**E34**). `cmp equal to HEAD` dopo ognuna, `git status --porcelain` com'era | le righe 1–4 uguali |
| 15 | le righe col perché | `review5\why_lines.py`, una riga tolta per volta, `npm test` | cadono: D59 in `Confirm.vue` (le due della finestra), la tripla nella descrizione (le due), la fascia solo senza il core (*«is there while waiting…»* e M-3), I1 in `Settings.vue` (una sotto jsdom e quella del browser), la policy spenta (una), il tema salvato subito (una), *«and closes»* del segnaposto (una); **non** cade nulla per Escape in `Confirm.vue` (M-3), per «Riprova» e per la ricerca spenta (N-2) | — |
| 16 | la stabilità della tastiera | nel clone, `npx vitest run --project browser src/panels` × 10; `npm test` × 5 | 10 su 10 `Tests  1 passed (1)`; 5 su 5 `Tests  158 passed \| 1 skipped (159)` | uguale |
| 17 | la prova della tastiera morde la strada della freccia | nel clone, `if (isArrowKeyPressed.value) currentElement.value?.click();` spento in `node_modules/reka-ui/dist/RadioGroup/RadioGroupItem.js`, poi ricopiato | spento: `Tests  1 failed (1)`, `AssertionError: expected +0 to be 1`, `Matcher did not succeed in time`; ricopiato: `Tests  1 passed (1)`; `sha256` uguale prima e dopo | — |
| 18 | i fine-riga | `git show <c>:<f> \| tr -cd '\r' \| wc -c` e `wc -l` alle due basi; `git ls-files --eol` | tutti `i/lf w/lf`, zero CR prima e dopo; righe come nella tabella del rapporto, file per file; `settings.browser.test.ts` 61 righe, finisce con `\n` | uguale |
| 19 | i vincoli 5, 6, 7, 12 | `git diff` delle righe aggiunte in `.vue` e `dock.css` contro `#…`, `rgb(`, `hsl(`, `oklch(`…; `git grep 'var(--ref-' 545f500 -- gui/src ':!gui/src/tokens/'`; `git diff --stat` di `gui/package.json`, del lockfile, di `crates/` e `gui/schema/` | niente, niente, vuoto, vuoto | — |
| 20 | il contratto | `git log -1 --format=%B 545f500`; `… \| grep -ci co-authored`; `git status -sb`; `git rev-list --count 39827e8..545f500` | comincia con `design-system(compito 5): `, porta le due righe del pezzo, `663.93 kB` e `689.58 kB`, come il messaggio di `95068bb`; 0; `[ahead 1]`; 1. Il piano: la sola colonna Commit della riga 4, `` `841dc54`, con la cura `8d098d0` ``, e la riga 5 a `—` / `⬜` | — |
| 21 | le chiavi e il segno | `grep` in `it.json` e in `settings.browser.test.ts`; `grep -o "'\[role=\"dialog\"\]"` | `themeTitle` e `theme.{system,light,dark}`; `44:  // ⛔ NON-VACUITY (E37 …)`; sei in `modules.test.ts`, uno in `a11y.test.ts`; il blocco di Impostazioni dalle righe 137–199 a 87 righe | uguale |
| 22 | nessun `<button>`, `<ul>`, `<ol>` a mano, nessun `reka-ui`/`lucide` | `grep -rn -E '<(button\|ul\|ol)\b'` nei `.vue`, `grep -rn 'createElement'`, `grep -rn -E 'from "(reka-ui\|lucide)'` fuori da `components/` | il solo `<button` di `BaseButton.vue:37`; il solo `createElement("button")` di `frame/BigTab.ts:39` (R3-16, atteso); nessun import | — |
| 23 | i commenti | `grep` su *«native radio»*, `.confirm`, `drawer-overlay`, `E184`, *«veil»*, `color-scheme` in `gui/src` | veri tutti: il ponte di `dock.css` (**E36**) parla delle sole barre di scorrimento, e nel dock non c'è più un controllo nativo (`grep '<(input\|textarea\|select)'` → il solo `BaseTextField.vue`, nella barra); `harness/ts` promette `reka-ui` *«from task 5»* ed è vero (riga 3 del Passo 7 su un `.ts`) | — |
| 24 | `check-docs.sh` da solo | `bash scripts/check-docs.sh` | `OK — no inconsistencies.`, uscita 0 | uguale |

## 4. La SPA, guardata

`review5\walk.cjs`: Chrome installato (`chromium.launch({ channel: "chrome" })`, `playwright` 1.63.0 del clone, nessun
download), 1440 × 900, `prefers-color-scheme` scuro e poi chiaro, la SPA del clone su `npm run dev`; il core finto guidato
con `harnessFake.deliver(…)`; uno screenshot dopo ogni passo in `review5\shots\<tema>-NN-….png`, e le misure in
`review5\shots\walk.json`. Poi `sheet.cjs`, `sheet_base.cjs` e `bar_measure.cjs` per I-1, M-1 e N-1. Il server fermato per
PID due volte (`Stop-Process` sul `vite` e sul suo `npm`), e alla fine nessun `node.exe` e nessun Chrome senza finestra.
Le immagini le ho guardate una per una.

**Nei due temi, uguale:**

| Passo | Che cosa ho visto |
|---|---|
| `01-load` | la barra coi tre pulsanti `quiet`, la ricerca spenta con l'icona — il segnaposto tagliato, M-1 —, *«Core: in attesa»*, «+ moduli» col bordo; la fascia *«Il core non ha risposto.»* col «Riprova» piccolo, nei colori dell'avviso; Stato, Permessi coi titoli di `BaseLabel` e l'icona, Impostazioni con la policy spenta e il tema su «Sistema»; i segnaposto *«arriva col sotto-progetto N»* e *«niente ancora»*. La radice ha `data-theme` uguale al sistema; nessuno scorrimento orizzontale; nessun testo tagliato dentro un elemento, misurato, tranne il segnaposto dell'`input`, che la misura non vede e l'occhio sì |
| `02-accepted` | la fascia se ne va, la **stessa** regione resta, vuota, alta zero (`sameBandRegion: true`); il dock sale; *«Core: collegato»* nel colore dell'accento |
| `03-bar-*` | il contorno del focus, visibile, su «Home» dopo un Tab e su «+ moduli» — rosa nello scuro, rosso scuro nel chiaro |
| `04-drawer-*` | il foglio dal basso col velo, *«I MODULI»* e le righe di `BaseList`, il nome a sinistra e *«arriva col sotto-progetto N»* all'estremo destro, 1400 px più in là; l'ultima riga tagliata dal bordo del foglio, e **«Chiudi», che ha il fuoco, fuori dalla vista** (I-1); Esc chiude, e il fuoco torna a «+ moduli» col contorno; col mouse lo stesso, anche con «Chiudi» |
| `05-policy` | i due gruppi, «POLICY VRAM» e «TEMA»; fra i due, lo spazio doppio (N-1) |
| `05b-policy-arrow` | la freccia, col tasto tenuto: il fuoco su «Locale» col contorno, la spunta ferma su «OpenRouter» (P-8), *«Richiesta inviata: in attesa del core.»* nella regione, e sul filo `Invoke(local)` |
| `06-confirm-open` | la finestra al centro, *«Serve un permesso»*, la tripla in parole, *«Per: vram-policy con local»*, *«Vale per questa tripla…»*, «Rifiuta» `quiet` col fuoco e «Consenti» `primary`; il velo; la tripla in attesa in Permessi, nel colore dell'avviso |
| `06b-confirm-esc` | Esc: la finestra chiusa, niente mandato (`sentAfterEsc: 0`), il fuoco di nuovo su «Locale», la regione di Impostazioni vuota |
| `07*` | un altro «Locale», la finestra, «Consenti»: `Approve`, la finestra chiusa, la riga ancora in volo (I-1 di E186); Permessi con la tripla concessa in `BaseList`; `Verdict`: in Stato *«Ultima richiesta di VRAM: rifiutata chiesti 4096 MiB, tetto 1024»* dentro la sua regione |
| `08-work`, `08b-steps` | Lavoro: Passi con *«passo 42: arbiter.set_policy»* e *«chiuso»*, il resto nei segnaposto; nulla che sborda |
| `09*` | nel gruppo del tema, la freccia tenuta: «Chiaro» → la radice `light`, poi «Scuro» → `dark`, poi «Sistema» → il tema del sistema; ogni scelta un `SaveLayout` col tema; tutta la SPA cambia colori insieme, e il contorno resta visibile nei due |
| `10-stale` | `StaleBuild`: la fascia rientra nella **stessa** regione (`sameBandRegion: true`), *«Il core parla una versione diversa del protocollo. La finestra non procede. Timbro atteso: …»*; *«Core: timbro diverso»* nel colore dello stop |

Il contrasto a occhio regge nei due temi: i testi deboli e le etichette in maiuscolo si leggono, e i soli elementi pallidi
sono i radio spenti della policy, che WCAG esenta. In console, nello scuro, un `404`: è `/favicon.ico` — `gui/public/` non
c'è e `index.html` non nomina un'icona —, la voce già registrata. Il segnaposto nello stato *«non esiste più»* non si
raggiunge coi fixture del finto: lo tiene la prova `a11y` sotto jsdom.

## 5. Ciò che non ho potuto verificare

- **Il Passo 8**, l'Assistente vocale: è del proprietario col coordinatore (**E35**). Ho visto che le parole entrano
  nella **stessa** regione, che c'era già, nei tre punti; che si **sentano**, no.
- Il **lato del core** di M-2: qui non c'è un core vero, e l'ordine dei messaggi l'ho dedotto dal codice di
  `stores/layout.ts` e dalla decisione 13 che il suo commento cita.
- La **CI**, del coordinatore dopo il push; e la macchina **`zagor`**.
- La **finestra vera** di Electron: ho guardato nel Chrome installato, senza finestra, a 1440 × 900. La soglia di 1130 px
  di I-1 è un conto, non una misura.
