# Il piano del design system — le consegne passate

> ⛔ **Non è lettura obbligatoria.** Qui stanno, **parola per parola**, le chiusure di sessione che il
> [piano](../superpowers/plans/2026-09-23-design-system.md) non tiene più: un documento vivo tiene solo l'ultima
> (`CLAUDE.md`). La più vecchia per prima.

## Il punto fermo della scrittura, del 2026-09-23

Tolto dal piano il 2026-09-23, quando la sessione di ripresa ha scritto *«Come si riprende — dopo la revisione»*. Le
forme dei compiti 2–5 sono diventate i compiti; quelle dei compiti 6–9 le ha **corrette** la revisione — R3-17…R3-23
del [registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md) — e vivono corrette nel piano. Il
testo com'era, dal commit `d589d15`:

## Come si riprende — punto fermo della scrittura, 2026-09-23

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–5**. Da scrivere: i compiti **6–9** e la
Definizione di «fatto». Chi riprende la scrittura legge questo file per intero, poi il disegno, e scrive i compiti che
mancano con le forme qui sotto — già decise scrivendo, e da non ridecidere senza una misura nuova.

| Compito | Le forme già decise |
|---|---|
| 2 | `vite.config.ts` con `test.projects`: `jsdom` — `environment: "jsdom"`, `include: ["src/**/*.test.ts"]`, `exclude: [...configDefaults.exclude, "src/**/*.browser.test.ts"]`, `setupFiles` — e `browser` — `include: ["src/**/*.browser.test.ts"]`, `browser: { enabled: true, headless: true, provider: playwright({ launchOptions: { channel: "chrome" } }), instances: [{ browser: "chromium" }], viewport, commands: { emulateMedia } }` — entrambi con `extends: true` per `plugins` e `define`. Il comando `emulateMedia(ctx, options)` chiama `ctx.page.emulateMedia(options)`; il tipo in `gui/src/browser.d.ts` con `declare module "vitest/browser" { interface BrowserCommands … }`. Fonti lette il 2026-09-23 alla v4.1.11 di `vitest-dev/vitest`: `docs/guide/browser/index.md`, `docs/config/browser/playwright.md`, `docs/api/browser/commands.md`, `docs/api/browser/context.md`, `docs/guide/projects.md`, `docs/config/css.md` (*«This option is not applied to browser tests»*: il CSS vale nel browser) |
| 3 | `BaseButton` — `variant: "primary" \| "secondary" \| "quiet" \| "card"`, `size: "sm" \| "md" \| "lg"`, `pill`, `disabled`, `icon`, `label` (per il pulsante che è solo un'icona), `type="button"` sempre; `BaseIcon` — `name: IconName`, `size`, disegno con `h()` dagli `IconNode` di `lucide` (`[tag, attrs][]`, `export type { IconNode }` in `dist/lucide.d.ts`), `stroke-width: var(--icon-stroke)`, `aria-hidden`; `icons.ts` — `ICONS = { views: Layers, modules: LayoutGrid, search: Search, saveView: BookmarkPlus, float: AppWindow, fullPage: Maximize2, e un'icona per ciascuno dei diciotto tipi di `PANEL_TYPES` } satisfies Record<string, IconNode>`, `type IconName = keyof typeof ICONS`; `BaseStatus` — un `div role="status"` sempre presente, il contenuto nello slot; `BaseList` — generico, `items`, `keyOf`, `ordered`, slot `item`; `BaseLabel` — `icon`, `as`; `BaseTextField` — `modelValue`, `label` (nome accessibile), `icon`, `disabled`, `error`, `inheritAttrs: false` con gli attributi sull'`input`; `BaseRadioGroup` — **controllato**: `modelValue: string \| null`, `options: { value, label }[]`, `legend`, `disabled`, emette `update:modelValue` e non cambia da sé (P-8); `BaseDialog` — `open` (facoltativo, `v-model:open`), `title`, `description`, `variant: "center" \| "sheet" \| "full"`, slot `trigger` e `actions`, velo `--color-veil`, livello `--z-overlay`. Il linter: un blocco `**/*.ts` col parser di TypeScript (P-2); `no-restricted-imports` su `src/components/Base*.vue` e `src/components/icons.ts` (P-3); `lucide` solo da `icons.ts` |
| 4 | `gui/kit.html` + `src/kit/main.ts` + `src/kit/Kit.vue`: ogni pezzo in ogni stato, il tema scelto in cima con `BaseRadioGroup` (D7), le parole esemplari scritte nel file e un blocco del linter su `src/kit/**` (D8); nel cancello, dopo `npm run build`, `dist/kit.html` non deve esistere e `dist/index.html` sì. Le prove del browser montano `Kit.vue`: raggi (l'algoritmo di `sonda-raggi.js` con le radici come parametro, guardia `near > 0`), testo tagliato e sbordare (quello di `sonda-caratteri.js`), icone disegnate, `currentColor`, centrate entro 0,75 px (`sonda-icone.js`), `axe` col contrasto acceso; nei due temi |
| 5 | `Band`: `<BaseStatus><div v-if=… class="band">…</div></BaseStatus>`; Stato e Impostazioni allo stesso modo; `Confirm` e `Drawer` su `BaseDialog`; Permessi, Passi e il cassetto su `BaseList`; i titoli con `BaseLabel`; la policy VRAM e il tema su `BaseRadioGroup` (il tema chiama `layout.chooseTheme`); le prove di `modules.test.ts` su `[role=radio]`; le regole `vue/no-restricted-html-elements` (`button`, `ul`, `ol`) e `no-restricted-imports` (`reka-ui`) su `src/panels/**` e `src/frame/**`; l'Assistente vocale a mano, col verbale |
| 6 | `DockviewTheme` nostro: `name: "harness"`, `className: "dockview-theme-harness"`, `colorScheme` da `shownTheme`, `gap` da `readToken("--space-3")`, aggiornato con `api.updateOptions({ theme })`; il margine sul contenitore `.dock` — 0, 12, 24 — e `api.layout` col **contenuto** (P-7); `dock.css` imposta ogni variabile dei due blocchi `.dockview-theme-abyss` tranne `--dv-tab-group-color-*` (P-6), con un test che lo prova leggendo `dockview.css`; `--dv-overlay-z-index: var(--z-floating)`; `BigTab` monta un pezzo Vue con `BaseLabel` e due `BaseButton` a sola icona |
| 7 | `LayoutPack.named?: { name: string; layout: SerializedDockview }[]` e `openNamed?: string` (D3); `unpack` scarta le voci invalide e i doppioni; nel negozio `openNamed` e `saveNamed(name, layout)`, che rifiuta un nome già usato (D4); il dock guarda anche `openNamed`; l'aiutante `nearest(from, candidates, direction)` estratto dalla geometria di `moveActive.ts`; `schematic(layout)` → rettangoli in frazioni dall'albero della griglia, con l'orientamento che si alterna a ogni livello a partire da `grid.orientation`, e senza i gruppi galleggianti (D5) |
| 8 | la barra: `BaseButton` col nome della vista e l'icona `views`, la ricerca `BaseTextField` spenta, il chip; la Panoramica in `frame/Overview.vue` su `BaseDialog variant="full"`: carte `BaseButton variant="card"` con la miniatura, la vista corrente in bordeaux, l'ultima carta *«Salva questa vista»*; F3 in `Frame.vue`, ignorato mentre la finestra di conferma è aperta; le frecce con `nearest`; la striscia a pillola con i «moduli» `BaseButton pill` che apre il cassetto; le prove del browser: la striscia a 12 e 24 px, le frecce e Invio nella Panoramica |
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda; la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` (il browser nel cancello, Chrome come prerequisito); `riferimenti.md` (le fonti della scrittura del piano, qui sopra e nella testa); la Definizione di «fatto» coi comandi |

## Dopo la revisione, del 2026-09-23

Tolto dal piano il 2026-09-23, quando la sessione che doveva applicare il
[registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md) si è chiusa **senza scriverlo**, con le
correzioni provate in una cartella di prova e raccolte in uno script, e ha scritto *«Come si riprende — l'applicazione del
registro, a metà»*. Il testo com'era, dal commit `82e64a2`, parola per parola; i tre rimandi sono riscritti per questa
cartella.

## Come si riprende — dopo la revisione, 2026-09-23

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–5**, **rivisti ma non ancora corretti**. Da
scrivere: i compiti **6–9** e la Definizione di «fatto». La consegna precedente — il punto fermo della scrittura, con le
forme dei compiti 2–9 — sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

⛔ **DA SAPERE SUBITO.** La sessione di scrittura si è chiusa a contesto saturo **senza** consegna: il proprietario l'ha
fermata mentre cominciava il compito 6, che **non esiste** — né qui né nel suo scratchpad, dove i frammenti `f00`…`f05`
coincidono col piano committato (`cmp` sui testi senza CR). La sessione di ripresa ha fatto **rivedere** i compiti 1–5 da
tre revisori Opus, ciascuno coi compiti della sua fetta applicati in una cartella di prova: **59 rilievi, 3 critici**. Il
registro, le decisioni e i rapporti stanno in
[`2026-09-23-design-system-revisione/`](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md). ⛔ **Nessuna correzione è applicata
ai compiti 1–5**, e si applicano **prima** di scrivere il compito 6: i compiti 6–8 costruiscono sui pezzi dei compiti 1–5.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura e alla chiusura della ripresa: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | la corsa di `833e6b2` ha il job `windows-latest` **rosso** al passo di `gate.sh`, e il log non si legge senza accesso (l'API rende 403); lo stesso albero — `git diff --stat 833e6b2 d589d15 -- . ':!docs'` non rende nulla, e il compendio è lo stesso — è **verde** sui due sistemi a `d589d15`: non deterministico, da guardare se torna. ⏳ Le corse dei commit di questa ripresa erano **in corso** alla chiusura: la sessione dopo le legge **per prima**. I comandi stanno in [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato |
| **le cartelle di prova** | su **questa** macchina soltanto, fuori dal repository: `C:\Users\zagor\AppData\Local\Temp\probe-ds1` (compito 1 applicato), `probe-ds2` (compiti 2 e 3), `probe-ds3` (compiti 1–5, con un suo `git`). Servono a provare le correzioni, e si cancellano dopo |

**Il prossimo passo, in ordine** — ciascuno nella sua sessione (`CLAUDE.md`, una fase per sessione):

1. **Applicare il registro**, `2026-09-23-design-system-revisione/ledger.md`, com'è scritto nella sua sezione *«Come si
   applica»*: uno script solo ad ancore uniche, ogni correzione nel compito che la causa, le specie cercate in tutto il
   piano, i richiami datati nel disegno e nella tavola, ogni ⬜ che diventa ✅; poi `check-docs.sh`, il cancello, commit e
   push.
2. **Scrivere i compiti 6–9** e la Definizione di «fatto», con le forme qui sotto — ⚠️ **già corrette dalla revisione**
   (R3-17…R3-23 del registro), e da non ridecidere senza una misura nuova. Chi scrive legge questo file per intero e il
   disegno, e **prima** di ogni compito rilegge le *Interfaces* dei compiti 1–5 **corretti**.
3. Il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito, in una sessione sua; poi l'esecuzione.

| Compito | Le forme già decise, corrette dalla revisione |
|---|---|
| 6 | `DockviewTheme` nostro: `name: "harness"`, `className: "dockview-theme-harness"`, `gap` da `readToken("--space-3")`; il bordo della zona d'arrivo dal campo `dndOverlayBorder`, o dalla variabile `--dv-drag-over-border` che `updateTheme` lascia al foglio quando il campo manca — il tipo in `dockview-core` 8.3.1 ha **undici** campi, non i sette del disegno (R3-22); aggiornato con `api.updateOptions({ theme })`. `colorScheme` da `shownTheme` **resta**, testo della (c), con detto accanto che `dockview-core` 8.3.1 non lo usa per disegnare (R3-21). Il margine sul contenitore `.dock` — 0, 12, 24 — e `api.layout` col **contenuto** (P-7). `dock.css` imposta ogni variabile dei due blocchi `.dockview-theme-abyss` tranne `--dv-tab-group-color-*` (P-6), con un test che lo prova leggendo `dockview.css`, e **toglie il ponte del compito 1** (R1-10). ⛔ `--dv-overlay-z-index: var(--z-floating)` **sul contenitore galleggiante**, `.dockview-theme-harness .dv-resize-container`: `dockview.css` 8.3.1 lo ridefinisce lì come un ciclo (riga 1037), e sul solo tema il gruppo resterebbe a 999, sopra i dialoghi (R3-17) — provato nel browser con un gruppo galleggiante vero: `z-index` calcolato uguale a `--z-floating`, e 999 con la regola tolta. ⚠️ Quel contenitore porta `role="dialog"`: col dock montato e un gruppo galleggiante, una prova che cerca `[role="dialog"]` ne trova due. `BigTab` monta un pezzo Vue con `BaseLabel` e due `BaseButton` a sola icona |
| 7 | `LayoutPack.named?: { name: string; layout: SerializedDockview }[]` e `openNamed?: string` (D3); `unpack` scarta le voci invalide e i doppioni; nel negozio `openNamed` e `saveNamed(name, layout)`, che rifiuta un nome già usato (D4) — anche i nomi delle tre viste di sempre, com'è lo specimen della pagina kit. ⛔ **`settle` scrive nella vista col nome quando `openNamed` c'è, e in `layouts[view]` altrimenti**, e `receive` riporta anche `openNamed`: una prova del negozio tiene che una mossa in una vista col nome lasci `layouts.home` com'era (R3-19). Il dock guarda anche `openNamed`. L'aiutante `nearest(from, candidates, direction)` estratto dalla geometria di `moveActive.ts`, che lo usa anche lui, **con lo spareggio sull'altro asse** — a parità di distanza vince il centro più vicino sull'asse perpendicolare — provato coi rettangoli a mano anche su una griglia di tre colonne, «giù» dalla colonna di mezzo (R3-18). `schematic(layout)` → rettangoli in frazioni dall'albero della griglia, con l'orientamento che si alterna a ogni livello a partire da `grid.orientation`, e senza i gruppi galleggianti (D5) |
| 8 | la barra: `BaseButton` col nome della vista e l'icona `views`, la ricerca `BaseTextField type="search"` spenta, il chip; il pulsante del cassetto **esce** dalla barra, perché «Moduli» scende nella striscia (la (d)). La Panoramica in `frame/Overview.vue` su `BaseDialog variant="full"`: carte `BaseButton variant="card"` con la miniatura, la vista corrente in bordeaux, l'ultima carta *«Salva questa vista»*; F3 in `Frame.vue`, ignorato mentre la finestra di conferma è aperta; le frecce con `nearest`. La striscia a pillola coi «moduli» `BaseButton pill` che apre il cassetto — ⛔ **lo stato aperto del cassetto vive in un negozio**, letto da `Drawer.vue` e scritto dalla striscia: la striscia è un pannello, cioè un'app Vue sua (`VueContent`), e non raggiunge un `ref` di `Drawer.vue` (R3-20). Nella miniatura il foglio `strip`, che sta in tutte le viste e non ha un'icona in `ICONS`: come si disegna lo dice la tavola della Panoramica, o lo decide il compito. Le prove: `axe` sulla Panoramica (controllo 17); nel browser la striscia a 12 e 24 px e **nessuna scheda vicina a un angolo della pagina** (controllo 19), le frecce e Invio nella Panoramica (R3-23) |
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda, **col richiamo datato**, e il comando del riquadro in testa al file rilanciato prima e dopo (controllo 21, R3-23); la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` — il browser nel cancello, e il prerequisito **Google Chrome, o `npx playwright install chrome`**: col canale `chrome` il Chromium scaricato non vale (R2-8); `riferimenti.md`, le fonti della scrittura del piano e della revisione; il pezzo JavaScript misurato, con la cifra per il proprietario, che ha N-2 (R3-25); la Definizione di «fatto» coi comandi |

## L'applicazione a metà, del 2026-09-23

Tolto dal piano il 2026-09-23, quando la sessione dopo ha fatto girare nel Chrome installato le correzioni che lo
aspettavano, ha corretto due righe dello script e lo ha lanciato — il commit `7c42748` —, e ha scritto *«Come si riprende
— il registro applicato»*. Il testo com'era, dal commit `9ca93c8`, parola per parola; i due rimandi sono riscritti per
questa cartella.

## Come si riprende — l'applicazione del registro, a metà, 2026-09-23

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–5**, rivisti; ⛔ **le correzioni della
revisione non sono ancora scritte** — in questa sessione nessun file del repository è cambiato, salvo questa consegna,
l'archivio e il registro. Da scrivere dopo: i compiti **6–9** e la Definizione di «fatto». La consegna precedente sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

⛔ **DA SAPERE SUBITO: il lavoro di questa sessione vive su UNA macchina, fuori dal repository** — quella dell'account
`Jays`, col clone in `E:\ALL\DEV\MY_REPOS\daemon`. Le cartelle di prova dei revisori stanno invece sull'altra, dell'account
`zagor`, e qui non servono più.

| | Dove, e che cosa |
|---|---|
| **la cartella di prova** | `C:\Users\Jays\AppData\Local\Temp\pds`, col suo `git`: la base a `82e64a2`, i compiti 1–5 applicati dal testo del piano, un commit per compito, poi un commit per ciascuna correzione provata. Sta in `%TEMP%` e non nello scratchpad perché qui i percorsi lunghi di Windows sono spenti (`LongPathsEnabled = 0`), e un revisore è già caduto su un nome oltre i 260 caratteri |
| **lo script delle correzioni** | `C:\Users\Jays\AppData\Local\Temp\pds\tools\ledger\apply_ledger.py`, coi pezzi `ledger_part1.py`…`ledger_part4.py`: le correzioni del piano, del disegno e della tavola, ad ancore uniche **tutte** controllate prima di scrivere. `python apply_ledger.py <radice> --dry` le conta senza scrivere; senza `--dry` le scrive. Le sostituzioni a blocco intero prendono il testo dai file della cartella di prova, che sono quelli provati |
| **Google Chrome** | installato su questa macchina dal proprietario il 2026-09-23, alla chiusura: prima non c'era |

Se quei file non ci sono più — un'altra macchina, `%TEMP%` ripulito — il lavoro si rifà dai rapporti e dal registro: le
decisioni prese applicando e i rilievi nuovi stanno nella sua sezione *«L'applicazione, a metà»*.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura e alla chiusura: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | le corse di `5b90eae` e `82e64a2` sono verdi su tutti e due i sistemi, e il rosso `windows-latest` di `833e6b2` non è tornato: la sua causa resta **ignota** — il log rende 403 — e si guarda se torna; quella del commit di chiusura si legge coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato |

**Il prossimo passo, in ordine** — la stessa fase, nella sessione dopo, su questa macchina:

1. **Le prove nel browser**, nella cartella di prova, ora che Chrome c'è: `(cd /c/Users/Jays/AppData/Local/Temp/pds/gui &&
   npx vitest run --project browser)`, poi le direzioni rosse. Le aspettano R2-2, R2-3 — se `gui/.vitest-attachments/`
   nasce ancora con `screenshotFailures: false` —, R2-4, R2-6 (la parte di tipi è provata), R2-7, le righe del passo 7 del
   compito 4 (R3-1…R3-4 e R3-8), R3-7 e R3-24. ⛔ Dove una prova dice altro, si corregge il file della cartella di prova o
   il pezzo dello script **prima** di scrivere, e la divergenza va nel registro.
2. **Lo script**: `python apply_ledger.py E:/ALL/DEV/MY_REPOS/daemon --dry`, poi senza `--dry`; poi il `git diff`
   riletto, e i fine-riga dei tre file rimisurati (sono LF: `tr -cd '\r' < <file> | wc -c` → 0).
3. **Il registro**: ogni ⬜ a ✅, e la sezione *«L'applicazione, a metà»* chiusa col commit che scrive.
4. `bash scripts/check-docs.sh`, il cancello da solo, commit e push.

Poi, ciascuno nella sua sessione (`CLAUDE.md`, una fase per sessione): i compiti **6–9** e la Definizione di «fatto», con
le forme qui sotto — ⚠️ già corrette dalla revisione (R3-17…R3-23 del registro), e da non ridecidere senza una misura
nuova; il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito; poi l'esecuzione. Chi scrive i
compiti 6–9 legge questo file per intero e il disegno, **prima** di ogni compito rilegge le *Interfaces* dei compiti 1–5
**corretti**, e porta nei compiti nuovi due forme che la correzione dà ai compiti 1–5: la colonna **Commit** di R1-16 —
ogni compito scrive l'hash del precedente — e il ritorno delle violazioni del **vincolo 11**, con la copia salvata.

| Compito | Le forme già decise, corrette dalla revisione |
|---|---|
| 6 | `DockviewTheme` nostro: `name: "harness"`, `className: "dockview-theme-harness"`, `gap` da `readToken("--space-3")`; il bordo della zona d'arrivo dal campo `dndOverlayBorder`, o dalla variabile `--dv-drag-over-border` che `updateTheme` lascia al foglio quando il campo manca — il tipo in `dockview-core` 8.3.1 ha **undici** campi, non i sette del disegno (R3-22); aggiornato con `api.updateOptions({ theme })`. `colorScheme` da `shownTheme` **resta**, testo della (c), con detto accanto che `dockview-core` 8.3.1 non lo usa per disegnare (R3-21). Il margine sul contenitore `.dock` — 0, 12, 24 — e `api.layout` col **contenuto** (P-7). `dock.css` imposta ogni variabile dei due blocchi `.dockview-theme-abyss` tranne `--dv-tab-group-color-*` (P-6), con un test che lo prova leggendo `dockview.css`, e **toglie il ponte del compito 1** (R1-10). ⛔ `--dv-overlay-z-index: var(--z-floating)` **sul contenitore galleggiante**, `.dockview-theme-harness .dv-resize-container`: `dockview.css` 8.3.1 lo ridefinisce lì come un ciclo (riga 1037), e sul solo tema il gruppo resterebbe a 999, sopra i dialoghi (R3-17) — provato nel browser con un gruppo galleggiante vero: `z-index` calcolato uguale a `--z-floating`, e 999 con la regola tolta. ⚠️ Quel contenitore porta `role="dialog"`: col dock montato e un gruppo galleggiante, una prova che cerca `[role="dialog"]` ne trova due. `BigTab` monta un pezzo Vue con `BaseLabel` e due `BaseButton` a sola icona. ⚠️ Il tema nostro sostituisce il **ponte** del compito 1, che oggi tiene leggibili anche i pulsanti della presa grande (senza, nel chiaro, 1,06:1 — A-2 del registro): il frammento del passo 17 del compito 1 si rilancia nei due temi, e pannelli, linguette e pulsanti stanno sopra 4,5 |
| 7 | `LayoutPack.named?: { name: string; layout: SerializedDockview }[]` e `openNamed?: string` (D3); `unpack` scarta le voci invalide e i doppioni; nel negozio `openNamed` e `saveNamed(name, layout)`, che rifiuta un nome già usato (D4) — anche i nomi delle tre viste di sempre, com'è lo specimen della pagina kit. ⛔ **`settle` scrive nella vista col nome quando `openNamed` c'è, e in `layouts[view]` altrimenti**, e `receive` riporta anche `openNamed`: una prova del negozio tiene che una mossa in una vista col nome lasci `layouts.home` com'era (R3-19). Il dock guarda anche `openNamed`. L'aiutante `nearest(from, candidates, direction)` estratto dalla geometria di `moveActive.ts`, che lo usa anche lui, **con lo spareggio sull'altro asse** — a parità di distanza vince il centro più vicino sull'asse perpendicolare — provato coi rettangoli a mano anche su una griglia di tre colonne, «giù» dalla colonna di mezzo (R3-18). `schematic(layout)` → rettangoli in frazioni dall'albero della griglia, con l'orientamento che si alterna a ogni livello a partire da `grid.orientation`, e senza i gruppi galleggianti (D5) |
| 8 | la barra: `BaseButton` col nome della vista e l'icona `views`, la ricerca `BaseTextField type="search"` spenta, il chip; il pulsante del cassetto **esce** dalla barra, perché «Moduli» scende nella striscia (la (d)). La Panoramica in `frame/Overview.vue` su `BaseDialog variant="full"`: carte `BaseButton variant="card"` con la miniatura, la vista corrente in bordeaux, l'ultima carta *«Salva questa vista»*; F3 in `Frame.vue`, ignorato mentre la finestra di conferma è aperta; le frecce con `nearest`. La striscia a pillola coi «moduli» `BaseButton pill` che apre il cassetto — ⛔ **lo stato aperto del cassetto vive in un negozio**, letto da `Drawer.vue` e scritto dalla striscia: la striscia è un pannello, cioè un'app Vue sua (`VueContent`), e non raggiunge un `ref` di `Drawer.vue` (R3-20). Nella miniatura il foglio `strip`, che sta in tutte le viste e non ha un'icona in `ICONS`: come si disegna lo dice la tavola della Panoramica, o lo decide il compito. Le prove: `axe` sulla Panoramica (controllo 17); nel browser la striscia a 12 e 24 px e **nessuna scheda vicina a un angolo della pagina** (controllo 19), le frecce e Invio nella Panoramica (R3-23) |
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda, **col richiamo datato**, e il comando del riquadro in testa al file rilanciato prima e dopo (controllo 21, R3-23); la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` — il browser nel cancello, e il prerequisito **Google Chrome, o `npx playwright install chrome`**: col canale `chrome` il Chromium scaricato non vale (R2-8); `riferimenti.md`, le fonti della scrittura del piano e della revisione; il pezzo JavaScript misurato, con la cifra per il proprietario, che ha N-2 (R3-25); la Definizione di «fatto» coi comandi |

## Il registro applicato, del 2026-09-23

Tolto dal piano il 2026-09-23, quando la ripresa della sessione che aveva scritto i compiti 6 e 7 — fermata dal
proprietario a contesto saturo, col compito 7 scritto e non committato, senza consegna — ha verificato i due compiti
contro la cartella di prova, ha corretto la ragione di D12, ha committato il 7 — `5ed1fa2` — e ha scritto *«Come si
riprende — il compito 7 scritto e verificato alla ripresa»*. Il testo com'era, dal commit `5d32147`, parola per
parola; i tre rimandi sono riscritti per questa cartella.

## Come si riprende — il registro applicato, 2026-09-23

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–5**, rivisti e **corretti** — le 73 correzioni
del [registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md) sono nel piano, nel disegno e nella tavola dal commit
`7c42748`, e il registro ha ogni riga a ✅. Da scrivere: i compiti **6–9** e la Definizione di «fatto». La consegna
precedente sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura e prima di ogni commit della sessione: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | le corse di `9ca93c8`, `7c42748` e `a4dfd81` sono verdi su tutti e due i sistemi; quella del commit che scrive questa riga era **in corso** alla chiusura, e la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»*. Il rosso `windows-latest` di `833e6b2`, di causa **ignota** — il log rende 403 —, non è tornato |
| **codice di prodotto** | non toccato |
| **Google Chrome** | 154, sulla macchina dell'account `Jays`; le prove nel browser non scaricano niente (decisione 22 del disegno) |

⚠️ **Una cartella di prova, su una macchina sola.** `C:\Users\Jays\AppData\Local\Temp\pds`, col suo `git`, `HEAD` a
`4637ac5`: i compiti 1–5 applicati dal testo del piano, con le correzioni di codice e di prova — non quelle di sola prosa.
È il banco dove provare il codice dei compiti 6–9 mentre si scrivono; **non è una fonte**, e si rifà dal piano. Sta in
`%TEMP%` e non nello scratchpad perché su quella macchina i percorsi lunghi di Windows sono spenti. Le cartelle dei
revisori, sull'altra macchina, si cancellano: *«L'applicazione»* del registro.

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`): **scrivere i compiti 6–9 e la Definizione di
«fatto»**, con le forme qui sotto — ⚠️ già corrette dalla revisione (R3-17…R3-23 del registro), e da non ridecidere senza
una misura nuova. Chi li scrive legge questo file per intero e il disegno, **prima** di ogni compito rilegge le
*Interfaces* dei compiti 1–5 corretti, e porta nei compiti nuovi le forme che la correzione ha dato a quelli:

1. la colonna **Commit** di R1-16 — ogni compito scrive l'hash del precedente;
2. il ritorno delle violazioni del **vincolo 11** — la copia salvata, `cmp`, e `git status --porcelain` confrontato con
   quello di prima;
3. i comandi in una sottoshell, `(cd gui && …)` (R1-12);
4. per **ogni** prova nuova, la violazione che la fa rossa e il messaggio **misurato** del rosso: applicando il registro, su
   tredici violazioni lanciate nel browser una prova nuova non aveva la sua (A-4) e una ne faceva cadere due, non una (A-5).

Poi, ciascuno nella sua sessione: il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito; poi
l'esecuzione.

| Compito | Le forme già decise, corrette dalla revisione |
|---|---|
| 6 | `DockviewTheme` nostro: `name: "harness"`, `className: "dockview-theme-harness"`, `gap` da `readToken("--space-3")`; il bordo della zona d'arrivo dal campo `dndOverlayBorder`, o dalla variabile `--dv-drag-over-border` che `updateTheme` lascia al foglio quando il campo manca — il tipo in `dockview-core` 8.3.1 ha **undici** campi, non i sette del disegno (R3-22); aggiornato con `api.updateOptions({ theme })`. `colorScheme` da `shownTheme` **resta**, testo della (c), con detto accanto che `dockview-core` 8.3.1 non lo usa per disegnare (R3-21). Il margine sul contenitore `.dock` — 0, 12, 24 — e `api.layout` col **contenuto** (P-7). `dock.css` imposta ogni variabile dei due blocchi `.dockview-theme-abyss` tranne `--dv-tab-group-color-*` (P-6), con un test che lo prova leggendo `dockview.css`, e **toglie il ponte del compito 1** (R1-10). ⛔ `--dv-overlay-z-index: var(--z-floating)` **sul contenitore galleggiante**, `.dockview-theme-harness .dv-resize-container`: `dockview.css` 8.3.1 lo ridefinisce lì come un ciclo (riga 1037), e sul solo tema il gruppo resterebbe a 999, sopra i dialoghi (R3-17) — provato nel browser con un gruppo galleggiante vero: `z-index` calcolato uguale a `--z-floating`, e 999 con la regola tolta. ⚠️ Quel contenitore porta `role="dialog"`: col dock montato e un gruppo galleggiante, una prova che cerca `[role="dialog"]` ne trova due. `BigTab` monta un pezzo Vue con `BaseLabel` e due `BaseButton` a sola icona. ⚠️ Il tema nostro sostituisce il **ponte** del compito 1, che oggi tiene leggibili anche i pulsanti della presa grande (senza, nel chiaro, 1,06:1 — A-2 del registro): il frammento del passo 17 del compito 1 si rilancia nei due temi, e pannelli, linguette e pulsanti stanno sopra 4,5 |
| 7 | `LayoutPack.named?: { name: string; layout: SerializedDockview }[]` e `openNamed?: string` (D3); `unpack` scarta le voci invalide e i doppioni; nel negozio `openNamed` e `saveNamed(name, layout)`, che rifiuta un nome già usato (D4) — anche i nomi delle tre viste di sempre, com'è lo specimen della pagina kit. ⛔ **`settle` scrive nella vista col nome quando `openNamed` c'è, e in `layouts[view]` altrimenti**, e `receive` riporta anche `openNamed`: una prova del negozio tiene che una mossa in una vista col nome lasci `layouts.home` com'era (R3-19). Il dock guarda anche `openNamed`. L'aiutante `nearest(from, candidates, direction)` estratto dalla geometria di `moveActive.ts`, che lo usa anche lui, **con lo spareggio sull'altro asse** — a parità di distanza vince il centro più vicino sull'asse perpendicolare — provato coi rettangoli a mano anche su una griglia di tre colonne, «giù» dalla colonna di mezzo (R3-18). `schematic(layout)` → rettangoli in frazioni dall'albero della griglia, con l'orientamento che si alterna a ogni livello a partire da `grid.orientation`, e senza i gruppi galleggianti (D5) |
| 8 | la barra: `BaseButton` col nome della vista e l'icona `views`, la ricerca `BaseTextField type="search"` spenta, il chip; il pulsante del cassetto **esce** dalla barra, perché «Moduli» scende nella striscia (la (d)). La Panoramica in `frame/Overview.vue` su `BaseDialog variant="full"`: carte `BaseButton variant="card"` con la miniatura, la vista corrente in bordeaux, l'ultima carta *«Salva questa vista»*; F3 in `Frame.vue`, ignorato mentre la finestra di conferma è aperta; le frecce con `nearest`. La striscia a pillola coi «moduli» `BaseButton pill` che apre il cassetto — ⛔ **lo stato aperto del cassetto vive in un negozio**, letto da `Drawer.vue` e scritto dalla striscia: la striscia è un pannello, cioè un'app Vue sua (`VueContent`), e non raggiunge un `ref` di `Drawer.vue` (R3-20). Nella miniatura il foglio `strip`, che sta in tutte le viste e non ha un'icona in `ICONS`: come si disegna lo dice la tavola della Panoramica, o lo decide il compito. Le prove: `axe` sulla Panoramica (controllo 17); nel browser la striscia a 12 e 24 px e **nessuna scheda vicina a un angolo della pagina** (controllo 19), le frecce e Invio nella Panoramica (R3-23) |
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda, **col richiamo datato**, e il comando del riquadro in testa al file rilanciato prima e dopo (controllo 21, R3-23); la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` — il browser nel cancello, e il prerequisito **Google Chrome, o `npx playwright install chrome`**: col canale `chrome` il Chromium scaricato non vale (R2-8); `riferimenti.md`, le fonti della scrittura del piano e della revisione; il pezzo JavaScript misurato, con la cifra per il proprietario, che ha N-2 (R3-25); la Definizione di «fatto» coi comandi |

## Il compito 7 scritto e verificato alla ripresa, del 2026-09-23

Tolto dal piano il 2026-09-24, quando la sessione che doveva scrivere i compiti 8 e 9 — su un'altra macchina, dove la
cartella di prova non c'era — l'ha rifatta dal testo del piano, ha trovato due cadute della suite e ha scritto *«Come si
riprende — la cartella di prova rifatta su un'altra macchina, e due cadute della suite»*. Il testo com'era, dal commit
`70500c0`, parola per parola; i rimandi sono riscritti per questa cartella.

## Come si riprende — il compito 7 scritto e verificato alla ripresa, 2026-09-23

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–7**. I compiti 1–5 li ha letti la revisione e
corretti il [registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md) (`7c42748`); il **6** (`a3d5509`) e il **7**
(`5ed1fa2`) sono nati dopo, e **nessun revisore li ha letti**: li legge il pre-controllo. Da scrivere: i compiti **8** e
**9** e la **Definizione di «fatto»**. La consegna precedente sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura della ripresa e prima di ogni suo commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi su tutti e due i sistemi le corse fino ad `a3d5509`; alla chiusura, quella di `5ed1fa2` era verde su `ubuntu-latest` e **in corso** su `windows-latest`, e quella del commit che scrive questa riga **in corso**: la sessione dopo le legge per prime, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato |
| **Google Chrome** | 154, sulla macchina dell'account `Jays`; le prove nel browser non scaricano niente (decisione 22 del disegno) |

⚠️ **La cartella di prova, su una macchina sola.** `C:\Users\Jays\AppData\Local\Temp\pds`, col suo `git`: `master` a
`4637ac5`, i compiti 1–5; `task6` a `9b825f4`; `task7` a `df7768f`. Ogni blocco di codice dei compiti 6 e 7 è **identico** a
un file di quei rami — confrontato alla ripresa, tranne i due frammenti della console —, e al compito 7 la cartella è verde:
*build*, linter, e la suite intera sei volte di fila, 26 file e 171 prove. È il banco dei compiti 8 e 9 mentre si scrivono;
**non è una fonte**, e si rifà dal piano. Sta in `%TEMP%` e non nello scratchpad perché su quella macchina i percorsi lunghi
di Windows sono spenti (`LongPathsEnabled` a 0). Le cartelle dei revisori, sull'altra macchina, si cancellano come dice il
[registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md), *«L'applicazione»*: da questa macchina non si verifica.

📌 **Il metodo dei compiti 6 e 7, che vale per l'8 e il 9.** Il codice si applica sul banco, in un ramo suo — `task8` da
`task7` —; le prove e i rossi girano lì, ciascuno col suo messaggio misurato; poi uno script compone il testo del compito
prendendo ogni file con `git show <ramo>:<file>`, e rifiuta se un blocco *Trova* non è unico nei file del ramo di prima, o
un blocco *Sostituisci con* in quelli del ramo nuovo. Così il codice del piano **è** il codice provato. Gli script della
sessione che li ha scritti — `insert6.py` e `insert7.py`, coi rossi in `reds6.py` e `reds7.py` — stanno nel suo scratchpad
su questa macchina, `…\E--ALL-DEV-MY-REPOS-daemon\4def4062-1a15-49d6-95bd-98976859816d\scratchpad\`: un esempio, non una
fonte.

✅ **D12 è del proprietario, dal 2026-09-23 — A:** due nomi di vista che differiscono solo per le maiuscole o per gli
spazi intorno sono lo stesso nome; la B era il confronto esatto. La sessione che l'ha scritta gli attribuiva la ragione
senza fonte; la ripresa gliel'ha chiesta, e la riga D12 e il commento di `sameName` dicono la sua scelta. Non si riapre.

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`): **scrivere i compiti 8 e 9 e la Definizione di
«fatto»**, con le forme qui sotto — ⚠️ già corrette dalla revisione (R3-20, R3-23, R3-25 e R2-8 del registro), e da non
ridecidere senza una misura nuova. Chi li scrive legge questo file per intero e il disegno, **prima** di ogni compito rilegge
le *Interfaces* dei compiti 1–7, e porta nei compiti nuovi le forme che hanno i compiti di prima:

1. la colonna **Commit** di R1-16 — ogni compito scrive l'hash del precedente;
2. il ritorno delle violazioni del **vincolo 11** — la copia salvata, `cmp`, e `git status --porcelain` confrontato con
   quello di prima;
3. i comandi in una sottoshell, `(cd gui && …)` (R1-12);
4. per **ogni** prova nuova, la violazione che la fa rossa e il messaggio **misurato** del rosso (A-4 e A-5 del registro).

Poi, ciascuno nella sua sessione: il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito — il 6 e
il 7 compresi —; poi l'esecuzione.

| Compito | Le forme già decise |
|---|---|
| 8 | la barra: `BaseButton` col nome della vista e l'icona `views`, la ricerca `BaseTextField type="search"` spenta, il chip; il pulsante del cassetto **esce** dalla barra, perché «Moduli» scende nella striscia (la (d)). La Panoramica in `frame/Overview.vue` su `BaseDialog variant="full"`: carte `BaseButton variant="card"` con la miniatura, la vista corrente in bordeaux, l'ultima carta *«Salva questa vista»*; F3 in `Frame.vue`, ignorato mentre la finestra di conferma è aperta; le frecce con `nearest`. La striscia a pillola coi «moduli» `BaseButton pill` che apre il cassetto — ⛔ **lo stato aperto del cassetto vive in un negozio**, letto da `Drawer.vue` e scritto dalla striscia: la striscia è un pannello, cioè un'app Vue sua (`VueContent`), e non raggiunge un `ref` di `Drawer.vue` (R3-20). Nella miniatura il foglio `strip`, che sta in tutte le viste e non ha un'icona in `ICONS`: come si disegna lo dice la tavola della Panoramica, o lo decide il compito. Le prove: `axe` sulla Panoramica (controllo 17); nel browser la striscia a 12 e 24 px e **nessuna scheda vicina a un angolo della pagina** (controllo 19), le frecce e Invio nella Panoramica (R3-23). ⚠️ **Dalle *Interfaces* del compito 7:** la miniatura è `schematic(layout)`; una vista col nome si apre scrivendo `openNamed`, una delle tre con `showView`; *«Salva questa vista»* chiama `saveNamed(name, layout, shown)` — `shown`, i nomi che la cornice mostra per le tre viste, da `it.json` (D12) — e dice sotto il campo, in rosso, `"empty"` e `"taken"` (D4). ⚠️ **Dal compito 6:** il gruppo galleggiante è un `.dv-resize-container` con `role="dialog"`: col dock montato e un gruppo galleggiante, una prova che cerca `[role="dialog"]` ne trova due |
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda, **col richiamo datato**, e il comando del riquadro in testa al file rilanciato prima e dopo (controllo 21, R3-23); la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` — il browser nel cancello, e il prerequisito **Google Chrome, o `npx playwright install chrome`**: col canale `chrome` il Chromium scaricato non vale (R2-8); `riferimenti.md`, le fonti della scrittura del piano e della revisione; il pezzo JavaScript misurato, con la cifra per il proprietario, che ha N-2 (R3-25); la Definizione di «fatto» coi comandi |

## La cartella di prova rifatta sulla macchina `zagor`, del 2026-09-24

Tolto dal piano il 2026-09-24, quando la sessione dopo ha chiuso **P-20** — la cura nel passo 2 del compito 5 — e
**P-21** — la causa non trovata, l'attesa di `expect.poll` a 5 s e la prova del tema da un tema noto, scelta A del
proprietario — e ha scritto *«Come si riprende — P-20 e P-21 chiuse»*. Il testo com'era, dal commit `71f6a92`, parola
per parola; i rimandi sono riscritti per questa cartella.

## Come si riprende — la cartella di prova rifatta su un'altra macchina, e due cadute della suite, 2026-09-24

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–7**; da scrivere: i compiti **8** e **9** e la
**Definizione di «fatto»**. Il 6 e il 7 non li ha letti nessun revisore: li legge il pre-controllo. La consegna precedente
sta parola per parola in [`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

⛔ **DA SAPERE SUBITO: la suite dei compiti 1–7 è caduta due volte, e il piano non lo porta ancora.** Due prove rosse nelle
prime sei corse della suite coi compiti 1–7, sulla macchina dell'account `zagor`, ciascuna una volta sola; nessuna nelle
sedici corse intere dopo. È ciò che il passo 7 del compito 7 prevede — *«una
sua caduta qui è una voce d'errata, non una corsa da ripetere finché passa»* —, trovato dalla ricostruzione prima di quel
passo. La prima ha la causa e la cura provata, **P-20**; la seconda **no**, **P-21**. ⛔ **Si chiudono tutte e due prima dei
compiti 8 e 9**: si scrivono su un banco che non cade da solo.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura della sessione e prima del suo commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi su tutti e due i sistemi le corse fino a `70500c0`; quella del commit che scrive questa riga **in corso** alla chiusura: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: la cura di P-20 vive solo nella cartella di prova |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL-DEV\MY-REPOS\daemon`, Chrome 154 e la cartella di prova della consegna in archivio; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness` — il portatile con l'i7-14700HX e la RTX 4060 —, Chrome **153**, dove questa sessione ha lavorato e le prove nel browser sono verdi. Su tutte e due `LongPathsEnabled` è 0: le cartelle di prova stanno in `%TEMP%`, non nello scratchpad |

📌 **La cartella di prova sulla macchina `zagor`:** `C:\Users\zagor\AppData\Local\Temp\pds`, col suo `git` senza `origin` — la
base a `70500c0`, `main` coi compiti 1–5 un commit l'uno, `task6`, `task7`, e `p20-copy-imports`, un commit sopra `task7` con
la cura di P-20. **Non è una fonte**: si rifà dal piano, ed è ciò che questa sessione ha fatto — ogni blocco *Trova* dei sette
compiti trovato **una volta** nel suo file, nessun rifiuto; al compito 7 la suite rende 26 file e 171 prove, come sull'altra
macchina, e il pezzo JavaScript 663,93 kB ai compiti 1–4, 689,58 al 5, 690,50 al 6 — le cifre del compito 6 — e 691,70 al 7,
`npm run build 2>&1 | grep -E 'assets/index-.*\.js '` (N-2, la cifra per il proprietario). Gli attrezzi stanno accanto, in
`pds\tools\`, fuori dal suo `git` per `.git/info/exclude`: `rebuild\apply_plan.py` con le ricette `recipe-1.txt`…`recipe-7.txt`,
che nominano i blocchi per **numero di riga del piano a `70500c0`** — si lanciano su un file preso con `git show
70500c0:docs/superpowers/plans/2026-09-23-design-system.md`, non sul piano di dopo —; e `flakes\`, gli attrezzi delle misure
qui sotto. Una sessione sulla macchina `Jays` ha la sua cartella, ma non la cura di P-20: la prende dal diff qui sotto.

**P-20 — la prova delle parole dei moduli, rossa per il tempo: causa trovata, cura provata, non ancora nel piano.**

| | |
|---|---|
| **che cosa** | `has a name for every module type` di `gui/src/locales/copy.test.ts` — della parte 2 — rossa **una** volta, `5177ms`, oltre il limite di 5 s di una prova: la prima delle cinque corse |
| **la causa, verificata** | la prova fa `await import("../panels/registry")` **nel suo corpo**, quindi il grafo del registro — `vue`, `vue-i18n`, il segnaposto, la striscia e, dal compito 5, i pezzi di base — si carica **dentro** i suoi 5 s. Una sonda provvisoria per strato, sotto jsdom, tre corse: `BaseIcon.vue` 751–826 ms — il primo `.vue` del file porta `vue` e la prima trasformazione —, `Placeholder.vue` ~295, `i18n` ~100, `lucide` ~100, `Strip.vue` ~90, `BaseButton.vue` ~85 |
| **le misure** | la sua durata nei rapporti JSON: sul codice di `70500c0` 1,3–1,4 s, 2,5 s a freddo; coi compiti 1–7 **2,5–2,8 s** in cinque corse su sei, e 5,2 s la volta che è caduta. Le due `await import("./Band.vue")` di `frame.test.ts` costano al massimo 95 ms: gli import in cima al file hanno già caricato il resto. A mano: `(cd gui && npx vitest run --reporter=verbose) \| grep 'has a name for every module type'` — la durata compare sopra i 300 ms |
| **la cura, provata** | `PANEL_TYPES` e `THEME_CHOICES` importati **in cima** a `copy.test.ts`, e le due prove senza `await`: dopo, **6–24 ms** in sei corse e **7–8 ms** in quattro cicli come il cancello, `npm ci` e poi la suite. È la regola che lo stesso file scrive per il linter — *«A guard that can go red for being slow guards nothing»* — e la forma di tutti gli altri file di prova. Il testo: `git -C C:/Users/zagor/AppData/Local/Temp/pds diff task7 p20-copy-imports` |
| **che cosa resta** | scriverla **nel piano**: la riga **P-20** in *«Ciò che la scrittura del piano ha trovato»*; nel passo 2 del compito 5, che tocca già quel file, un *Trova/Sostituisci* per gli import in cima, uno per la prima riga della prova del registro, e la prova del tema senza `await import`, col commento che dice perché. La direzione rossa si **misura**: con l'import rimesso nel corpo la durata torna sopra i 2 s |

**P-21 — la prova del tema che segue il sistema, rossa una volta: causa NON trovata.**

| | |
|---|---|
| **che cosa** | `follow the system's scheme through the real query while the choice is system` di `gui/src/tokens/tokens.browser.test.ts` — dal compito 2 — rossa **una** volta, in 1331 ms, nella seconda delle cinque corse. ⚠️ Il messaggio d'errore **non è stato preso**: quella corsa filtrava le righe col `grep`. 🔶 Dedotto: è scaduto un `expect.poll`, che in `vitest` 4.1.11 aspetta **1000 ms** di base — `defaults.timeout ?? 1e3`, letto nel pacchetto installato |
| **escluso, misurato** | (1) **che l'avviso del sistema arrivi tardi**: una sonda provvisoria, in un file suo, misura quanto tarda il cambio sulla radice dopo `emulateMedia` — **7–21 ms** al peggio in tredici corse, da solo, sotto la suite e dopo `npm ci`. (2) **che un file del browser blocchi gli altri**: un file che tiene occupato il suo filo per 2 s non ritarda i timer di un altro file nello stesso momento — 3–7 ms, tre corse coi tempi assoluti; ogni file ha contesto e pagina suoi, `openBrowserPage` di `@vitest/browser-playwright` 4.1.11. La prova vera, nelle dieci corse dopo: 70–540 ms, mai rossa |
| **la pista aperta** | durante la suite, dei processi di pagina del Chrome di prova **6 su 11** girano a priorità di base **4** — la classe *idle* di Windows — contro 8 dei 24 processi `node.exe`, benché Playwright 1.63.0 passi già `--disable-renderer-backgrounding`: un campione, `pds\tools\flakes\priority_sample.ps1`. 🔶 Sotto la CPU piena un processo *idle* può restare fermo a lungo. ⚠️ **Non si sa se sono le pagine delle prove**: `priority_detail.ps1`, che le distingue col tempo di CPU, è scritto e **non** è stato lanciato |
| **come si chiude** | prima la causa, poi la cura (`superpowers:systematic-debugging`): il messaggio della caduta, con `pds\tools\flakes\flake_runs.py` — un rapporto JSON per corsa, e la prima riga di ogni caduta — sulla suite intera, più corse, anche a freddo con `cold_runs.sh`; e quali processi sono *idle*, con `priority_detail.ps1` mentre la suite gira. Le cure dipendono dalla causa e restano **aperte** — un'opzione di Chrome nel `launchOptions` del compito 2, un tempo per `expect.poll` detto una volta nella configurazione del progetto `browser`, o altro —: nessuna si prende prima della causa |

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI della chiusura, per prima.
2. Questa sezione, la testa del piano e le *Interfaces* dei compiti 1–7.
3. **P-20** nel piano, con la cura di `p20-copy-imports`.
4. **P-21**: la causa, poi la cura, nel piano. Poi la cartella di prova riportata al testo del piano — `task7` con le due cure —,
   così che il codice del piano **sia** il codice provato; e la suite intera cinque volte di fila, verde.
5. I compiti **8** e **9** e la **Definizione di «fatto»**, con le forme qui sotto — già corrette dalla revisione (R3-20, R3-23,
   R3-25 e R2-8 del [registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md)), da non ridecidere senza una misura nuova —, e con
   le forme che hanno i compiti di prima:
   1. la colonna **Commit** di R1-16 — ogni compito scrive l'hash del precedente;
   2. il ritorno delle violazioni del **vincolo 11** — la copia salvata, `cmp`, e `git status --porcelain` confrontato con
      quello di prima;
   3. i comandi in una sottoshell, `(cd gui && …)` (R1-12);
   4. per **ogni** prova nuova, la violazione che la fa rossa e il messaggio **misurato** del rosso (A-4 e A-5 del registro);
   5. il metodo dei compiti 6 e 7: il codice si applica sul banco, in un ramo suo — `task8` da `task7` —, le prove e i rossi
      girano lì, e uno script compone il testo del compito dai file del ramo con `git show`, e rifiuta se un blocco *Trova*
      non è unico nei file del ramo di prima, o un blocco *Sostituisci con* in quelli del ramo nuovo.
6. Poi, ciascuno nella sua sessione: il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito — il 6 e il 7
   compresi —; poi l'esecuzione.

| Compito | Le forme già decise |
|---|---|
| 8 | la barra: `BaseButton` col nome della vista e l'icona `views`, la ricerca `BaseTextField type="search"` spenta, il chip; il pulsante del cassetto **esce** dalla barra, perché «Moduli» scende nella striscia (la (d)). La Panoramica in `frame/Overview.vue` su `BaseDialog variant="full"`: carte `BaseButton variant="card"` con la miniatura, la vista corrente in bordeaux, l'ultima carta *«Salva questa vista»*; F3 in `Frame.vue`, ignorato mentre la finestra di conferma è aperta; le frecce con `nearest`. La striscia a pillola coi «moduli» `BaseButton pill` che apre il cassetto — ⛔ **lo stato aperto del cassetto vive in un negozio**, letto da `Drawer.vue` e scritto dalla striscia: la striscia è un pannello, cioè un'app Vue sua (`VueContent`), e non raggiunge un `ref` di `Drawer.vue` (R3-20). Nella miniatura il foglio `strip`, che sta in tutte le viste e non ha un'icona in `ICONS`: come si disegna lo dice la tavola della Panoramica, o lo decide il compito. Le prove: `axe` sulla Panoramica (controllo 17); nel browser la striscia a 12 e 24 px e **nessuna scheda vicina a un angolo della pagina** (controllo 19), le frecce e Invio nella Panoramica (R3-23). ⚠️ **Dalle *Interfaces* del compito 7:** la miniatura è `schematic(layout)`; una vista col nome si apre scrivendo `openNamed`, una delle tre con `showView`; *«Salva questa vista»* chiama `saveNamed(name, layout, shown)` — `shown`, i nomi che la cornice mostra per le tre viste, da `it.json` (D12) — e dice sotto il campo, in rosso, `"empty"` e `"taken"` (D4). ⚠️ **Dal compito 6:** il gruppo galleggiante è un `.dv-resize-container` con `role="dialog"`: col dock montato e un gruppo galleggiante, una prova che cerca `[role="dialog"]` ne trova due |
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda, **col richiamo datato**, e il comando del riquadro in testa al file rilanciato prima e dopo (controllo 21, R3-23); la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` — il browser nel cancello, e il prerequisito **Google Chrome, o `npx playwright install chrome`**: col canale `chrome` il Chromium scaricato non vale (R2-8); `riferimenti.md`, le fonti della scrittura del piano e della revisione; il pezzo JavaScript misurato, con la cifra per il proprietario, che ha N-2 (R3-25); la Definizione di «fatto» coi comandi |

## Il compito 8 scritto, del 2026-09-24

Tolto dal piano il 2026-09-24, quando la sessione dopo ha scritto il **compito 8** — la cornice, applicata e provata prima
sulla cartella di prova — e *«Come si riprende — il compito 8 scritto»*. Il testo com'era, dal commit `7b9e338`, parola
per parola; i rimandi sono riscritti per questa cartella.

## Come si riprende — P-20 e P-21 chiuse, 2026-09-24

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–7**; da scrivere: i compiti **8** e **9** e la
**Definizione di «fatto»**. Il 6 e il 7 non li ha letti nessun revisore: li legge il pre-controllo. La consegna precedente
sta parola per parola in [`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

✅ **Le due cadute della suite sono chiuse, e il banco non cade da solo.** **P-20**, la causa trovata, è una cura nel
passo 2 del compito 5. **P-21**, la causa **non** trovata dopo una caccia che ha escluso priorità, CPU e memoria, ha
l'attesa di `expect.poll` a 5 s nel progetto `browser` del compito 2 — scelta **A** del proprietario il 2026-09-24 — e la
prova del tema che parte da un tema noto, con due righe nuove nelle direzioni rosse del suo passo 5. Le righe intere
sono in testa, in *«Ciò che la scrittura del piano ha trovato»*; la memoria della macchina, che la caccia ha misurato,
è una voce del proprietario in *«Le voci aperte che questo piano SA»*.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura della sessione e prima del suo commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi su tutti e due i sistemi le corse fino a `71f6a92`; quella del commit che scrive questa riga **in corso** alla chiusura: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: le cure vivono nel piano e nella cartella di prova |
| **il banco** | la cartella di prova della macchina `zagor`, ramo **`cures`**: sette corse intere verdi, una dopo `npm ci` e l'ultima sul commit finale del ramo, coi rapporti JSON |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL-DEV\MY-REPOS\daemon`, Chrome 154 e la sua cartella di prova, **senza** le cure; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness` — il portatile con l'i7-14700HX, la RTX 4060 e 15,7 GB di memoria —, Chrome 153, dove questa sessione ha lavorato. Su tutte e due `LongPathsEnabled` è 0: le cartelle di prova stanno in `%TEMP%`, non nello scratchpad |

📌 **La cartella di prova sulla macchina `zagor`:** `C:\Users\zagor\AppData\Local\Temp\pds`, col suo `git` senza
`origin` — la base a `70500c0`, `main` coi compiti 1–5, `task6`, `task7`, e **`cures`**, due commit sopra `task7`: i tre
file che P-20 e P-21 toccano, rifatti dal testo del piano con `pds\tools\rebuild\apply_plan.py` e la ricetta qui
sotto — `vite.config.ts` e `copy.test.ts` prima riportati alla base, `git show 70500c0:<file>` —; `git diff task7 cures`
rende solo le righe delle due cure. La ricetta vale per il piano del commit che scrive questa riga, e sulla macchina
`Jays` si lancia uguale. **Il compito 8 parte da `cures`.** Gli attrezzi della caccia stanno in
`pds\tools\flakes\`, fuori dal suo `git`: `flake_runs.py` — un rapporto JSON per corsa —; `renderer_watch.ps1` e
`chrome_dump.ps1` con `dump_diff.py`, le priorità dei processi di Chrome; `sys_sample.ps1`, memoria e disco; `hog.py`,
la CPU; e tre sonde usa-e-getta da copiare in `gui/src/`: `zz-probe` — evento, fotogramma e timer a ogni cambio di
tema —, `zz-amplified` — la sequenza del file quaranta volte — e `zz-late`, l'attesa del progetto.

```text
R gui/vite.config.ts 1436 1443
R gui/vite.config.ts 1452 1458
R gui/vite.config.ts 1480 1494
W gui/src/tokens/tokens.browser.test.ts 1284
R gui/src/tokens/tokens.browser.test.ts 4566 4573
R gui/src/tokens/tokens.browser.test.ts 4581 4590
R gui/src/locales/copy.test.ts 3858 3865
R gui/src/locales/copy.test.ts 3873 3881
R gui/src/locales/copy.test.ts 3896 3903
R gui/src/locales/copy.test.ts 3909 3916
```

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI della chiusura, per prima.
2. Questa sezione, la testa del piano e le *Interfaces* dei compiti 1–7.
3. I compiti **8** e **9** e la **Definizione di «fatto»**, con le forme qui sotto — già corrette dalla revisione (R3-20,
   R3-23, R3-25 e R2-8 del [registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md)), da non ridecidere senza una misura
   nuova —, e con le forme che hanno i compiti di prima:
   1. la colonna **Commit** di R1-16 — ogni compito scrive l'hash del precedente;
   2. il ritorno delle violazioni del **vincolo 11** — la copia salvata, `cmp`, e `git status --porcelain` confrontato con
      quello di prima;
   3. i comandi in una sottoshell, `(cd gui && …)` (R1-12);
   4. per **ogni** prova nuova, la violazione che la fa rossa e il messaggio **misurato** del rosso (A-4 e A-5 del registro);
   5. il metodo dei compiti 6 e 7: il codice si applica sul banco, in un ramo suo — `task8` da `cures` —, le prove e i
      rossi girano lì, e uno script compone il testo del compito dai file del ramo con `git show`, e rifiuta se un blocco
      *Trova* non è unico nei file del ramo di prima, o un blocco *Sostituisci con* in quelli del ramo nuovo;
   6. ⚠️ ogni corsa della suite col suo **rapporto JSON** — `flake_runs.py` —, e una caduta è una voce del piano, non una
      corsa da ripetere: P-20 e P-21 si sono viste così, e il messaggio della seconda si era perso in un `grep`.
4. Poi, ciascuno nella sua sessione: il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito — il 6
   e il 7 compresi —; poi l'esecuzione.

| Compito | Le forme già decise |
|---|---|
| 8 | la barra: `BaseButton` col nome della vista e l'icona `views`, la ricerca `BaseTextField type="search"` spenta, il chip; il pulsante del cassetto **esce** dalla barra, perché «Moduli» scende nella striscia (la (d)). La Panoramica in `frame/Overview.vue` su `BaseDialog variant="full"`: carte `BaseButton variant="card"` con la miniatura, la vista corrente in bordeaux, l'ultima carta *«Salva questa vista»*; F3 in `Frame.vue`, ignorato mentre la finestra di conferma è aperta; le frecce con `nearest`. La striscia a pillola coi «moduli» `BaseButton pill` che apre il cassetto — ⛔ **lo stato aperto del cassetto vive in un negozio**, letto da `Drawer.vue` e scritto dalla striscia: la striscia è un pannello, cioè un'app Vue sua (`VueContent`), e non raggiunge un `ref` di `Drawer.vue` (R3-20). Nella miniatura il foglio `strip`, che sta in tutte le viste e non ha un'icona in `ICONS`: come si disegna lo dice la tavola della Panoramica, o lo decide il compito. Le prove: `axe` sulla Panoramica (controllo 17); nel browser la striscia a 12 e 24 px e **nessuna scheda vicina a un angolo della pagina** (controllo 19), le frecce e Invio nella Panoramica (R3-23). ⚠️ **Dalle *Interfaces* del compito 7:** la miniatura è `schematic(layout)`; una vista col nome si apre scrivendo `openNamed`, una delle tre con `showView`; *«Salva questa vista»* chiama `saveNamed(name, layout, shown)` — `shown`, i nomi che la cornice mostra per le tre viste, da `it.json` (D12) — e dice sotto il campo, in rosso, `"empty"` e `"taken"` (D4). ⚠️ **Dal compito 6:** il gruppo galleggiante è un `.dv-resize-container` con `role="dialog"`: col dock montato e un gruppo galleggiante, una prova che cerca `[role="dialog"]` ne trova due |
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda, **col richiamo datato**, e il comando del riquadro in testa al file rilanciato prima e dopo (controllo 21, R3-23); la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` — il browser nel cancello, e il prerequisito **Google Chrome, o `npx playwright install chrome`**: col canale `chrome` il Chromium scaricato non vale (R2-8); `riferimenti.md`, le fonti della scrittura del piano e della revisione — fra queste le due righe di `vitest` 4.1.11 lette per P-21, l'attesa di base di `expect.poll` e il tempo di una prova nel browser; il pezzo JavaScript misurato, con la cifra per il proprietario, che ha N-2 (R3-25); la Definizione di «fatto» coi comandi |

## Il compito 9 e la Definizione di «fatto» scritti, del 2026-09-24

Tolto dal piano il 2026-09-24, quando la sessione dopo ha scritto il **compito 9** — la chiusura — e la **Definizione di
«fatto»**, e *«Come si riprende — il piano scritto»*. Il testo com'era, dal commit `79d33b1`, parola per parola; i rimandi
sono riscritti per questa cartella, e i numeri della ricetta sono quelli del piano di quel commit.

## Come si riprende — il compito 8 scritto, 2026-09-24

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–8**; da scrivere: il compito **9** e la
**Definizione di «fatto»**. Il 6, il 7 e l'8 non li ha letti nessun revisore: li legge il pre-controllo. La consegna
precedente sta parola per parola in [`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

✅ **Il compito 8 è scritto, e provato sulla cartella di prova prima di esserlo.** Il codice è stato applicato nel ramo
`task8`, le prove e i venti rossi del suo passo 9 sono girati lì, e il testo è composto dai file del ramo da uno script
che rifiuta un *Trova* non unico e rifà ogni file nuovo byte per byte. Scrivendolo sono venuti **P-22**…**P-25** — due
fatti della libreria di prova e della striscia, uno di `reka-ui`, e un commento del compito 3 corretto — e le decisioni
**D14**…**D18**. ⚠️ **La D18 è da rileggere dal proprietario**: la miniatura di Compatta è il suo schema, non la
*«finestrella»* della (d). E una voce 🔶 **dedotta** è in *«Le voci aperte che questo piano SA»*: Ctrl+Alt+frecce sotto
una finestra modale.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura della sessione e prima del suo commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi su tutti e due i sistemi le corse fino a `7b9e338`; quella del commit che scrive questa riga **in corso** alla chiusura: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: il compito 8 vive nel piano e nella cartella di prova |
| **il banco** | la cartella di prova della macchina `zagor`, ramo **`task8`**, due commit sopra `cures`: cinque corse intere verdi di fila, coi rapporti JSON, e i venti rossi del passo 9 del compito 8 fatti e tolti uno alla volta |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL-DEV\MY-REPOS\daemon`, Chrome 154 e la sua cartella di prova, **senza** le cure né il compito 8; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, Chrome 153, dove questa sessione ha lavorato. Su tutte e due `LongPathsEnabled` è 0: le cartelle di prova stanno in `%TEMP%`, non nello scratchpad |

📌 **La cartella di prova sulla macchina `zagor`:** `C:\Users\zagor\AppData\Local\Temp\pds`, col suo `git` senza
`origin` — `cures` com'era, `task8` sopra, e `t8-tests`, le sole prove del compito 8 sopra `cures`, da cui vengono i
rossi del passo 3. Gli attrezzi della scrittura stanno in `pds\tools\t8\`, fuori dal suo `git`: `compose8.py` compone il
testo del compito dai due rami, `c8-template.md` ne è la prosa, `reds.py` misura le venti violazioni una alla volta, e
`one_red.py` ne rifà una. Il ramo si rifà dal testo del piano con `pds\tools\rebuild\apply_plan.py` e la ricetta qui sotto,
**da `cures`** — la ricetta vale per il piano del commit che scrive questa riga, e sulla macchina `Jays` si lancia uguale,
dopo la ricetta delle cure:

```text
R gui/src/testing/axe.ts 6379 6386
R gui/src/kit/kit.browser.test.ts 6408 6420
R gui/src/kit/kit.browser.test.ts 6431 6449
R gui/src/frame/dock.browser.test.ts 6459 6472
R gui/src/frame/frame.test.ts 6492 6511
R gui/src/frame/frame.test.ts 6535 6544
R gui/src/a11y.test.ts 6702 6709
R gui/src/a11y.test.ts 6717 6726
R gui/src/a11y.test.ts 6737 6744
R gui/src/a11y.test.ts 6751 6776
R gui/src/locales/copy.test.ts 6823 6830
R gui/src/locales/copy.test.ts 6838 6845
R gui/src/locales/copy.test.ts 6853 6860
W gui/src/frame/frame.browser.test.ts 6875
W gui/src/stores/drawer.ts 7068
R gui/src/stores/invoke.ts 7089 7095
R gui/src/stores/invoke.ts 7101 7108
R gui/src/stores/invoke.ts 7124 7130
R gui/src/components/Confirm.vue 7138 7161
R gui/src/components/Confirm.vue 7178 7184
W gui/src/frame/Drawer.vue 7190
R gui/src/locales/it.json 7234 7242
R gui/src/locales/it.json 7249 7259
W gui/src/frame/Overview.vue 7280
W gui/src/frame/ViewBar.vue 7550
W gui/src/frame/Frame.vue 7617
W gui/src/panels/Strip.vue 7703
R gui/src/tokens/dock.css 7765 7772
```

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI della chiusura, per prima.
2. Questa sezione, la testa del piano e le *Interfaces* dei compiti 1–8.
3. Il compito **9** e la **Definizione di «fatto»**, con la forma qui sotto — già corretta dalla revisione (R3-23, R3-25
   e R2-8 del [registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md)), da non ridecidere senza una misura nuova —, e con
   le forme che hanno i compiti di prima: la colonna **Commit** di R1-16; i comandi in una sottoshell, `(cd gui && …)`
   (R1-12); per **ogni** controllo nuovo la violazione che lo fa rosso e il messaggio **misurato** del rosso (A-4 e A-5
   del registro), e il ritorno con la copia salvata, `cmp` e `git status --porcelain` confrontato con quello di prima
   (vincolo 11).
4. Poi, ciascuno nella sua sessione: il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito — il
   6, il 7 e l'8 compresi —; poi l'esecuzione.

| Compito | La forma già decisa |
|---|---|
| 9 | la riga «Accessibilità» da ✅ a 🔶 con le parole della legenda, **col richiamo datato**, e il comando del riquadro in testa al file rilanciato prima e dopo (controllo 21, R3-23); la riga **14** in coda alla roadmap e in *«Perché quest'ordine»* (prima del 13); `README.md`; la §12 e la §6 del compendio; `porta-di-qualita.md` — il browser nel cancello, e il prerequisito **Google Chrome, o `npx playwright install chrome`**: col canale `chrome` il Chromium scaricato non vale (R2-8); `riferimenti.md`, le fonti della scrittura del piano e della revisione — fra queste le due righe di `vitest` 4.1.11 lette per P-21, l'attesa di base di `expect.poll` e il tempo di una prova nel browser, e i due file di `reka-ui` 2.10.4 letti per P-22; il pezzo JavaScript misurato, con la cifra per il proprietario, che ha N-2 (R3-25) — sulla cartella di prova 696,60 kB coi compiti 1–8, contro i 663,26 del `main` senza il design system; la Definizione di «fatto» coi comandi |

## Il pre-controllo del compito 1, del 2026-09-24

Tolto dal piano il 2026-09-24, quando la sessione dopo ha fatto il **pre-controllo del compito 1**, e scritto
*«Come si riprende — il pre-controllo del compito 1»*. Il testo com'era, dal commit `923ee52`, parola per parola; i
rimandi sono riscritti per questa cartella, e i numeri della ricetta sono quelli del piano di quel commit — e del
commit dopo, che non ha mosso nessuna riga prima di *«Come si riprende»*.

## Come si riprende — il piano scritto, 2026-09-24

✅ **Il piano è SCRITTO**: la testa, i compiti **1–9** e la **Definizione di «fatto»**. ⛔ **Non si esegue ancora:** viene prima
il **pre-controllo** delle quattro domande di `CLAUDE.md`, compito per compito, ciascuno in una sessione sua. I compiti 6, 7, 8 e 9
non li ha letti nessun revisore: li legge il pre-controllo. La consegna precedente sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

✅ **Che cosa ha fatto questa sessione.** Il compito **9** e la **Definizione di «fatto»**, nella forma che il *«Come si riprende»*
di prima dava — R2-8, R3-23 e R3-25 del [registro](../superpowers/plans/2026-09-23-design-system-revisione/ledger.md) — e sul modello del compito 17
della parte 2; scrivendoli sono venuti **P-26**…**P-30** e **D19**…**D24**. Nello stesso commit, per **D19**: la riga del disegno
in `docs/README.md`, falsa dal 2026-09-23, riscritta (P-26); la riga di questo piano nella tabella dei piani della roadmap, con
l'intestazione (P-27); e il puntatore della §6 del compendio, che dice il piano scritto. ✅ **Il proprietario ha scelto,
all'apertura:** prima il compito 9, e le contraddizioni di `porta-di-qualita.md` dopo, in una sessione loro che **non ha ancora
collocato** — tranne **C-S0-1**, che il compito 9 corregge dove scrive il browser (P-29). ✅ **D18** l'ha scelta il
proprietario alla chiusura, **A**, lo schema — col richiamo datato nella (d) del disegno —; la voce 🔶 **dedotta** di Ctrl+Alt+frecce sotto una finestra modale resta fra le voci che il piano sa.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura della sessione e prima del suo commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi su tutti e due i sistemi le corse fino a `03d1abd`, il commit che ha scritto il piano; quella del commit che scrive questa riga — la scelta di D18 — **in corso** alla chiusura: la sessione dopo le legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: questa sessione ha scritto documenti, e `dod_suite.py` solo nello scratchpad |
| **il banco** | la cartella di prova della macchina `zagor`, ramo **`task8`**, com'era: questa sessione non l'ha toccata |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL-DEV\MY-REPOS\daemon`, Chrome 154 e la sua cartella di prova, **senza** le cure né il compito 8; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, Chrome 153, dove questa sessione ha lavorato. Su tutte e due `LongPathsEnabled` è 0: le cartelle di prova stanno in `%TEMP%`, non nello scratchpad |

📌 **La cartella di prova sulla macchina `zagor`:** `C:\Users\zagor\AppData\Local\Temp\pds`, col suo `git` senza `origin` —
`cures`, `task8` sopra, `t8-tests` —, e gli attrezzi in `pds\tools\`. Il ramo del compito 8 si rifà dal testo del piano con
`pds\tools\rebuild\apply_plan.py` e la ricetta qui sotto, **da `cures`**. ⚠️ **La ricetta vale per il piano del commit che scrive
questa riga:** le righe della testa aggiunte da questa sessione hanno spostato i numeri, e quelli qui sotto sono spostati e
**provati** — ogni riga nomina un'apertura di blocco uguale a quella della ricetta di prima. Le ricette dei compiti 1–7 e delle
cure, in `pds\tools\rebuild\`, valgono per il piano del loro commit: `git show <commit>:docs/superpowers/plans/2026-09-23-design-system.md`
dà il file da passare ad `apply_plan.py`.

```text
R gui/src/testing/axe.ts 6390 6397
R gui/src/kit/kit.browser.test.ts 6419 6431
R gui/src/kit/kit.browser.test.ts 6442 6460
R gui/src/frame/dock.browser.test.ts 6470 6483
R gui/src/frame/frame.test.ts 6503 6522
R gui/src/frame/frame.test.ts 6546 6555
R gui/src/a11y.test.ts 6713 6720
R gui/src/a11y.test.ts 6728 6737
R gui/src/a11y.test.ts 6748 6755
R gui/src/a11y.test.ts 6762 6787
R gui/src/locales/copy.test.ts 6834 6841
R gui/src/locales/copy.test.ts 6849 6856
R gui/src/locales/copy.test.ts 6864 6871
W gui/src/frame/frame.browser.test.ts 6886
W gui/src/stores/drawer.ts 7079
R gui/src/stores/invoke.ts 7100 7106
R gui/src/stores/invoke.ts 7112 7119
R gui/src/stores/invoke.ts 7135 7141
R gui/src/components/Confirm.vue 7149 7172
R gui/src/components/Confirm.vue 7189 7195
W gui/src/frame/Drawer.vue 7201
R gui/src/locales/it.json 7245 7253
R gui/src/locales/it.json 7260 7270
W gui/src/frame/Overview.vue 7291
W gui/src/frame/ViewBar.vue 7561
W gui/src/frame/Frame.vue 7628
W gui/src/panels/Strip.vue 7714
R gui/src/tokens/dock.css 7776 7783
```

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI della chiusura, per prima.
2. Questa sezione; poi la testa del piano — i vincoli, *«Come si esegue un compito»*, l'errata, *«Ciò che la scrittura del piano
   ha trovato»*, le decisioni e le voci che il piano sa.
3. Il **pre-controllo del compito 1**, con le quattro domande di `CLAUDE.md` e le righe 5–8 di ciò che non colgono — ⛔ la 5
   soprattutto: il compito si legge contro il codice di **adesso** —; le voci che trova vanno nell'errata, e il dispaccio resta
   pronto su file. Poi l'**esecuzione** del compito 1, in un'altra sessione; e così compito per compito, fino al 9.
