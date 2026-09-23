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
