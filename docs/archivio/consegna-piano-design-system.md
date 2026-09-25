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

## L'esecuzione del compito 1, del 2026-09-24

Tolto dal piano il 2026-09-24, quando la sessione dopo ha **eseguito il compito 1** e scritto *«Come si riprende —
l'esecuzione del compito 1»*. Il testo com'era, dal commit `e2cd7df`, parola per parola; i rimandi sono riscritti per
questa cartella. ⚠️ I numeri della sua ricetta sono quelli del piano di `d10d9a5`, come dice il suo richiamo di E1: le
righe E2–E9 dell'errata, scritte da `69d10fa`, li hanno spostati di altre otto.

## Come si riprende — il pre-controllo del compito 1, 2026-09-24

✅ **Il pre-controllo del compito 1 è fatto, e non ha trovato nessun difetto**: l'errata resta vuota, e il compito si esegue
com'è scritto. La consegna precedente sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

⚠️ **Perché «nessun difetto» è una misura e non un'impressione.** Il compito è stato **rifatto per intero dal testo del
piano** su una copia pulita di `923ee52` — `git clone` in `C:\Users\zagor\AppData\Local\Temp\pc1`, `npm ci`, poi
`pds\tools\rebuild\apply_plan.py` con la ricetta qui sotto e gli script dei Passi 9 e 13 — e ogni *Atteso* è tornato. Il testo
del compito è lo stesso, riga per riga, che la cartella di prova aveva già provato dopo la revisione:
`diff <(git show 5ed1fa2:<piano> | sed -n '230,1237p') <(sed -n '254,1261p' <piano>)` non rende nulla.

| Domanda | Esito, e il comando o la misura |
|---|---|
| 1 — la sonda è sbagliata? | no: le **otto** violazioni del Passo 16, una per volta sulla copia, rosse tutte **per la ragione scritta** — il contrasto nomina `color-text-muted on color-bg: 3.34 < 4.5`, la guardia dice `color-bg-test` nei due temi, i colori `panels/Strip.vue:30` —; ogni file tornato dalla copia salvata, `cmp` uguale, e `git status --porcelain` alla fine uguale a quello di prima |
| 2 — manca una sonda? | no: ogni artefatto ha la sua. Senza prova automatica restano quelli che il piano già dice — la regola della pagina in `App.vue` (P-23, compito 8); il tema posato prima del *mount* in `main.ts` e la presa grande, che il Passo 17 guarda (misurata **40** px); i caratteri caricati, che prova il compito 2 |
| 3 — l'artefatto è sbagliato? | no, sulla copia: Passo 2, il diff di `package.json` `4 +++-` e la licenza `OFL-1.1`; Passo 8, **6** prove rosse su 28 e la scelta sconosciuta verde; Passo 9, **9** verdi; Passo 13, undici `ok:` e il censimento pulito; Passo 15, **100** prove passate e una saltata, *build* e linter verdi, il pezzo JavaScript `663.93 kB` contro i `663.26 kB` del `main`; `npm audit`, 0; `check-docs.sh`, `OK`; i fine-riga, i file nuovi LF e gli altri coi CR uguali alle righe; Passo 17, il testo peggiore del dock a **5,86** nello scuro e **6,22** nel chiaro, `seen` 39, Geist caricato |
| 4 — è già eseguito? | no: `gui/src/tokens/tokens.css` c'è, `base.css` no |
| 5 — il contratto è cresciuto sotto il piano? | no: `git log fd2812b..HEAD -- gui/` non rende nulla; sotto `scripts/` è cambiato solo il tetto di `check-docs.sh` |
| 6 — un commento o un banco lo smentisce? | no: sulla copia, dopo il compito, `tokens.css` è nominato solo da `tokens/dock.css`, che dice che esce, e il commento di `a11y.test.ts` sul contrasto è quello che il Passo 14 lascia al compito 3, detto |
| 7, 8 | non si applicano: il compito non tocca un ADR e non è un rapporto |

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura — sotto `gui/` 84 prove passate e una saltata — e prima del commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi su tutti e due i sistemi le corse fino a `923ee52`; quella del commit che scrive questa riga, **in corso** alla chiusura: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: questa sessione ha scritto documenti |
| **la copia del pre-controllo** | `C:\Users\zagor\AppData\Local\Temp\pc1`, ramo **`t1`**, **senza** `origin`: il compito 1 rifatto dal testo del piano, in un commit locale. Serve all'esecuzione e alla revisione come **confronto** — i file del compito devono esserle uguali a meno dei fine-riga —, e si cancella dopo il compito 1. Solo sulla macchina `zagor` |
| **il dispaccio** | pronto sulla macchina `zagor`, nella cartella git-ignorata `.superpowers/sdd/2026-09-23-design-system/`: `dispatch-task-1.md`, e `task-1-brief.md` fatto per **ancore** da `_extract_brief_1.py`. Su un'altra macchina non c'è, e si rifà: il brief è la testa del piano — obiettivo, pila, strumenti, vincoli, *«Come si esegue un compito»*, l'errata —, le voci **P-1, P-4, P-5, P-12, P-13**, le decisioni **D1** e **D2**, le voci che il piano sa, il compito 1, la **(a)** del disegno e i suoi controlli **1–7** |
| **il banco** | la cartella di prova della macchina `zagor`, `C:\Users\zagor\AppData\Local\Temp\pds`, ramo **`task8`**, com'era: questa sessione ne ha usato solo gli attrezzi |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL-DEV\MY-REPOS\daemon`, Chrome 154 e la sua cartella di prova, **senza** le cure né il compito 8; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, Chrome 153, dove questa sessione ha lavorato. Su tutte e due `LongPathsEnabled` è 0: le cartelle di prova stanno in `%TEMP%`, non nello scratchpad |

📌 **La ricetta del compito 1**, per `apply_plan.py`, vale per il piano del commit che scrive questa riga — le righe prima di
*«Come si riprende»* non si sono mosse, quindi vale anche la ricetta del compito 8 della consegna precedente, in archivio. I
due caratteri si installano **a mano prima**, col Passo 2; le righe `S` salvano gli script dei Passi 9 e 13, da lanciare a mano.

⚠️ **Richiamo del 2026-09-24, E1:** la riga di **E1** nell'errata sposta di **una** riga tutto ciò che la segue. I numeri qui sotto sono quelli di prima **più uno**, riletti uno per uno sul recinto che aprono; la ricetta del compito 8 in archivio vale col suo più uno, e il `diff` del pre-controllo, più su, si legge a `6ae9b8b`. E1 cambia la prosa del Passo 10, non un recinto.

```text
W gui/src/tokens/board.test.ts 319
W gui/src/tokens/contrast.test.ts 371
W gui/src/tokens/usage.test.ts 513
W gui/src/tokens/theme.test.ts 569
A gui/src/stores/stores.test.ts 649
S 704 <cartella>/extract_tokens.py
W gui/src/tokens/theme.ts 755
R gui/src/stores/layout.ts 814 821
R gui/src/stores/layout.ts 829 838
R gui/src/stores/layout.ts 851 868
R gui/src/stores/layout.ts 900 906
RL gui/src/stores/layout.ts
< import { ref } from "vue";
> import { computed, ref } from "vue";
W gui/src/tokens/index.ts 920
W gui/src/tokens/dock.css 941
W gui/src/App.vue 1020
RL gui/src/main.ts
< import "./tokens/tokens.css";
> import "./tokens";
R gui/src/main.ts 1045 1051
R gui/src/main.ts 1058 1073
S 1115 <cartella>/rename_tokens.py
```

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI della chiusura, per prima.
2. Questa sezione; poi il dispaccio, con la testa e la data riallineate e il brief rigenerato.
3. ⛔ **Il costo, prima di dispacciare, e il sì del proprietario**: un compito eseguito per intero sono da quattro a cinque
   dispacci su Opus — implementatore, revisore, un giro di correzioni, una ri-revisione —, misurati sui compiti della
   parte 2 fra circa 0,9 e 1,5 milioni di token e fra una e due ore.
4. L'**esecuzione del compito 1**, con `superpowers:subagent-driven-development`; il revisore rilancia ogni comando e
   confronta i file con la copia `t1`. Poi il **pre-controllo del compito 2**, in un'altra sessione; e così compito per
   compito, fino al 9.

## Il pre-controllo del compito 2, del 2026-09-24

Tolto dal piano il 2026-09-24, quando la sessione dopo ha fatto il **pre-controllo del compito 2** e scritto
*«Come si riprende — il pre-controllo del compito 2»*. Il testo com'era, dal commit `b66aee8`, parola per parola; i
rimandi sono riscritti per questa cartella.

## Come si riprende — l'esecuzione del compito 1, 2026-09-24

✅ **Il compito 1 è eseguito, rivisto e curato.** La consegna precedente — il pre-controllo — sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

| Commit | Che cosa |
|---|---|
| `d10d9a5` | **E1**, dal coordinatore prima del dispaccio: l'etichetta *«CRLF su questa macchina»* del Passo 10 tolta, perché sulla macchina dell'esecuzione era falsa |
| `95068bb` | **il compito 1**, dall'implementatore — conforme al dettato byte per byte, dice la revisione |
| `69d10fa` | le cure della revisione, dal coordinatore (scelta **A** del proprietario, senza ri-revisione): **E2**, **E3**, **E4**, **E7**, **E8**, **E9**, e il commento di `Drawer.vue` sui gruppi galleggianti, falso da prima del compito |
| `e2cd7df` | **E5** ed **E6**, le due scelte **A** del proprietario: la frase della tavola sui ruoli non-testo, e la prova dei colori a mano allargata alle funzioni di CSS Color 4 |

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` a `e2cd7df`, sotto `gui/` **100** prove passate e **una** saltata — si rilancia, non si cita: `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi sui due sistemi `d10d9a5`, `95068bb` e `69d10fa`; `e2cd7df` e il commit che scrive questa riga **in corso** alla chiusura: la sessione dopo li legge per primi, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **la posizione** | la riga **1** a `✅ 2026-09-24`; la sua colonna **Commit** la scrive il compito 2 (R1-16): `95068bb`, con le cure `69d10fa` ed `e2cd7df` |
| **il pezzo JavaScript** | da `663.26 kB` a `663.93 kB` col compito 1, e le cure non lo muovono: è la cifra che **N-2 di E187** porta al proprietario, in *«Le voci aperte che questo piano SA»* |
| **il dispaccio** | sulla macchina dell'account `Jays`, nella cartella git-ignorata `.superpowers/sdd/2026-09-23-design-system/`: il prompt e il brief dell'implementatore (`dispatch-task-1.md`, e `task-1-brief.md` da `_extract_brief_1.py`, per ancore), il suo rapporto, il prompt e il rapporto del revisore, e `compare_task1.py`. ⚠️ **Non viaggiano**: su un'altra macchina si rifanno dal piano |
| **le copie** | il clone della revisione è cancellato; la copia `t1` del pre-controllo, sulla macchina `zagor`, non serve più e si può cancellare |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL\DEV\MY_REPOS\daemon`, dove questa sessione ha lavorato — la consegna di prima scriveva `E:\ALL-DEV\MY-REPOS\daemon`, ed era sbagliato —, con la sua cartella di prova `%TEMP%\pds` al ramo `task7`; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness` e la cartella di prova al ramo `task8` |

**La revisione** — un revisore Opus fresco: **conforme**; **0** critici, **2** importanti, **4** minori, **3** nit, tutti chiusi
dai due commit di cura. Il costo misurato: l'implementatore **~267k** token, 115 chiamate, **~16** minuti; il revisore
**~430k**, 158 chiamate, **~35** minuti. L'implementatore è sotto la banda detta al proprietario (0,4–0,7 milioni): il compito
era quasi tutto dettato.

📌 **Ciò che questa sessione ha imparato, e che non era scritto** — nessuna voce è ancora un gotcha: le raccoglie la chiusura
del sotto-progetto.

| | Che cosa | Che cosa se ne fa |
|---|---|---|
| 1 | **senza la copia `t1`**, il confronto col dettato lo fa `compare_task1.py`: ricostruisce ogni file dal testo del piano — la ricetta, i tagli della tavola, le rinomine, la riga del compendio — e lo confronta col commit. Il revisore l'ha provato nelle due direzioni, con quattordici commit mutanti | è il modello per i compiti dopo, su qualunque macchina: basta la ricetta del compito |
| 2 | nel log di Vite `Local:` è spezzato dai codici colore | il server si aspetta su `netstat -ano \| grep ':5173' \| grep LISTENING`, e si spegne per PID |
| 3 | col pannello del browser nascosto `requestAnimationFrame` non scatta, e una schermata può scadere | il cambio di tema e la misura in due chiamate; la schermata si ritenta |
| 4 | il **rapporto** dell'implementatore ha dato per vero un commento senza misurarlo — *«floating groups are at 99»* —, e la revisione l'ha misurato falso (I-1) | un rapporto è una dichiarazione: le sue frasi di merito si rimisurano |
| 5 | **E1** è la settima volta dei fine-riga fra le due macchine | un compito pre-controllato su una macchina ed eseguito sull'altra rifà gli Attesi di forma prima del dispaccio |

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI di `e2cd7df` e del commit che scrive questa riga, per prima.
2. Il **pre-controllo del compito 2**, con le quattro domande di `CLAUDE.md` e le righe 5–8. ⛔ La 5 soprattutto: il compito
   si legge contro il codice di **adesso**, e le cure hanno toccato dieci file che il compito 1 aveva scritto o toccato —
   `git diff --stat 95068bb..HEAD -- gui/ docs/superpowers/specs/` li elenca. Le voci che trova vanno nell'errata, e la
   prossima libera è **E10**; il dispaccio resta pronto su file.
3. L'**esecuzione del compito 2**, in un'altra sessione, col costo detto prima e il sì del proprietario; e così compito per
   compito, fino al 9.

## L'esecuzione del compito 2, del 2026-09-25

Tolto dal piano il 2026-09-25, quando la sessione dopo ha **eseguito il compito 2** e scritto *«Come si riprende —
l'esecuzione del compito 2»*. Il testo com'era, dal commit `5f32158`, parola per parola; i rimandi sono riscritti per
questa cartella. ⚠️ I numeri della sua ricetta sono quelli del piano di `f00d5b7`: la riga **E14** dell'errata e la cura
del recinto del Passo 2, scritte da `5f32158`, spostano di una riga il primo recinto e di quattro gli altri.

## Come si riprende — il pre-controllo del compito 2, 2026-09-24

✅ **Il pre-controllo del compito 2 è fatto, e ha trovato quattro difetti**: **E10**, del proprietario — scelta **A** —,
**E11**, **E12** ed **E13**, scritti nell'errata e **già applicati** al testo del compito, che si esegue com'è scritto adesso.
La consegna precedente sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

⚠️ **Perché è una misura e non un'impressione.** Il compito è stato **rifatto per intero dal testo del piano** su una copia
pulita di `b66aee8` — `git clone -c core.autocrlf=false` in `%TEMP%\pc2`, sulla macchina `Jays`, `npm ci`, poi i Passi 1–7 coi
recinti del piano, estratti con `blocks.py` — oggi nella cartella tracciata del dispaccio — e applicati con `replace_unique.py` —, e ogni *Atteso* è tornato. Scritte
le voci, i Passi 3 e 6 sono stati **rifatti dal testo corretto**, con la ricetta qui sotto, e il passo web del cancello,
`bash scripts/gate-gui.sh`, è girato sulla copia: verde com'è scritto, rosso con l'uno o l'altro progetto vuoto.

| Domanda | Esito, e il comando o la misura |
|---|---|
| 1 — la sonda è sbagliata? | no: le **otto** violazioni del Passo 5, una per volta sulla copia, rosse tutte **per la ragione scritta** — `Unsupported chromium channel`; `vitest/browser can be imported only inside the Browser Mode`; `expected 3 to be greater than or equal to 4`; `--duration-fast: expected '110ms' to be '0ms'`; la guardia dell'alto contrasto a `expected false to be true`; `expected 0 to be greater than 0.5` al primo confronto col ripiego; il tema rosso a **5057 ms** al primo cambio; l'attesa di 5 s verde con la riga e rossa senza, a **1057 ms** —; ogni file tornato dalla copia salvata, `cmp` uguale, e `git status --porcelain` alla fine uguale a quello di prima, dopo sei rossi del browser |
| 2 — manca una sonda? | **sì, due**: **E10** e **E11**. Le altre ci sono, e misurate: `browser.d.ts` tolto rende rosso `vue-tsc`, sette `TS2339`; un canale valido ma non installato, `chrome-beta`, dà `Chromium distribution 'chrome-beta' is not found`, uscita 1. ⚠️ La finestra delle prove è **1440 × 900**, `devicePixelRatio` 1, misurata con una prova usa-e-getta che falliva dicendo i valori; nessuna prova la tiene, e nessun controllo del disegno lo chiede — **registrato, non preso** |
| 3 — l'artefatto è sbagliato? | no, sulla copia: Passo 1, undici pacchetti, MIT e Apache-2.0, nessuno con script d'installazione, `npm audit` 0; Passo 2, il rosso giusto; Passo 4, il progetto `browser` con **5** prove verdi, la suite da 17 file e 100 prove a **18** e **105**, il *build* verde e il pezzo JavaScript `663.93 kB`, com'era; i due progetti, **5** e **101** righe; Passi 6–7, il passo web verde e `check-docs.sh` `OK`; i fine-riga, tutti LF sulla copia — la macchina `Jays`, dove `core.autocrlf` è `false`; `git diff --stat HEAD -- crates/ gui/schema/` vuoto |
| 4 — è già eseguito? | no: `gui/package.json` non ha `@vitest/browser-playwright`, e `tokens.browser.test.ts` non c'è |
| 5 — il contratto è cresciuto sotto il piano? | no: `git log 95068bb..HEAD` non rende nulla sui file che il compito tocca o legge — `scripts/`, `gui/vite.config.ts`, `package.json`, il lockfile, `tokens/index.ts`, `base.css`, il disegno del 2 —, tranne `theme.ts`, dove la cura E8 cambia un commento e `watchTheme` resta com'era; i quattro *Trova* tornano unici a `b66aee8`, e la base del Passo 4, 100 prove e una saltata, è quella del cancello d'apertura |
| 6 — un commento o un banco lo smentisce? | **sì**: il commento di `gate-gui.sh`, *«§8 fixed "npm ci, npm run build, npm test"»*, dice **approvata** la forma del passo, ed è la ragione del richiamo che **E10** scrive nella §8 del disegno del 2; il commento di `theme.test.ts`, *«the real query is proven in the browser, task 2»*, il compito lo rende vero |
| 7, 8 | non si applicano: il compito non tocca un ADR e non è un rapporto |

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura, a `b66aee8` — sotto `gui/` 100 prove passate e una saltata — e prima del commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi sui due sistemi le corse fino a `ae79a97`, lette alla chiusura; quella del commit che scrive questa riga **in corso**: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: questa sessione ha scritto il piano e la cartella del dispaccio, e il codice del compito 2 vive solo nella copia |
| **la copia del pre-controllo** | ⚠️ **solo sulla macchina `Jays`**: `%TEMP%\pc2`, ramo **`t2`**, **senza** `origin`, col compito rifatto dal testo **corretto** nel commit locale `bead315`. Altrove non serve: il confronto col testo del piano lo fa `compare_task2.py` su qualunque clone. Si cancella dopo il compito 2 |
| **il dispaccio** | ✅ **viaggia con git**, per il punto 8 di *«Come si esegue un compito»*: nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/` stanno il **modello** del prompt, `dispatch-task-2.md` — coi campi `<repo>`, `<HEAD>`, `<data>`, `<scratchpad>` e la tabella delle due macchine —; `_extract_brief_2.py`, che scrive il brief nella cartella di lavoro ignorata e la crea su un clone nuovo; `compare_task2.py`, che confronta un commit col testo del piano, **provato nelle due direzioni** sulla copia — un commit fedele al piano sopra `ebfc255` esce **0**, e quattro mutanti, un valore di `vite.config.ts`, il file usa-e-getta lasciato, un file in più e `playwright` non esatto, escono **1** ciascuno —; `blocks.py`, che estrae i recinti del piano; e il dispaccio del compito 1 intero — prompt, rapporto, prompt del revisore, revisione, script —, dove `review-1-prompt.md` è il modello del prompt del revisore del compito 2. Il brief **non** si committa: è una copia del piano |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL\DEV\MY_REPOS\daemon`, `core.autocrlf` `false` in `.git/config` e l'albero `w/lf`, Chrome **154**, Node 24.19.0 e la cartella di prova `%TEMP%\pds` al ramo `task7`, dove questa sessione ha lavorato; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, `core.autocrlf` `true` dal file di sistema e l'albero `w/crlf`, e Chrome 153. ⛔ Sulla macchina che esegue gli Attesi di **forma** si misurano, non si copiano (E72): il §4 del prompt lo dice |

📌 **La ricetta del compito 2**, per rifarlo o confrontarlo dal testo del piano, vale per il piano del commit che scrive questa
riga: `W` è il file intero dal recinto aperto a quella riga; `R` sostituisce l'occorrenza unica del primo recinto col secondo;
`S` è il file usa-e-getta del Passo 5, che nasce e si cancella. Le due dipendenze si installano **prima**, a mano, col Passo 1;
il `<data>` dell'ultima riga è il giorno del commit. ⚠️ I numeri sono quelli di `ebfc255` **più sette**: il punto 8
di *«Come si esegue un compito»* è nato dopo, e ognuno è stato riletto sul recinto che apre.

```text
W gui/src/tokens/tokens.browser.test.ts 1327
W gui/src/browser.d.ts 1450
R gui/vite.config.ts 1479 1486
R gui/vite.config.ts 1495 1501
R gui/vite.config.ts 1523 1537
S 1630 gui/src/late.browser.test.ts
R scripts/gate-gui.sh 1651 1658
R docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md 1676 1682
```

📌 **Ciò che questa sessione ha imparato, e che non era scritto** — nessuna voce è ancora un gotcha: le raccoglie la chiusura
del sotto-progetto.

| | Che cosa | Che cosa se ne fa |
|---|---|---|
| 1 | **una guardia che nessuno ha scritto sparisce quando cambia la forma della corsa**: con un progetto solo `vitest` è rosso su un `include` vuoto, con due un progetto vuoto è verde, e nessun controllo lo diceva — E10 | quando un compito divide o raggruppa le prove, si rimisura il caso **vuoto** |
| 2 | **una sonda prova il meccanismo del progetto solo se una violazione tocca il file del progetto**: la guardia dell'alto contrasto e il `box-shadow` provavano il browser, e la regola `:focus-visible` di `base.css` nessuna riga — E11 | per ogni prova, una violazione sul file che la prova difende |
| 3 | il costo di E10 misurato con la forma del cancello, `npm test -- --project`, è 1,5–2 s in più; con `npx vitest run`, misurato per primo, erano 2,5 | un costo si misura sul comando che girerà |
| 4 | **ciò che il dispaccio usa non viaggiava**: `.superpowers/sdd/` è ignorata da un suo `.gitignore` con `*`, e il prompt portava i valori di una macchina — `core.autocrlf` `false`, *«CR 0»* —, falsi sull'altra | dal 2026-09-24 il dispaccio sta nella cartella tracciata, e il prompt è un modello coi campi della macchina: il punto 8 |

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI del commit che scrive questa riga, per prima.
2. Questa sezione; poi il dispaccio: dalla radice del repository il brief, con
   `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_2.py`, e il prompt dal modello
   `dispatch-task-2.md` della stessa cartella, coi campi e i valori della macchina che esegue — il riquadro in testa al
   modello dice come.
3. ⛔ **Il costo, prima di dispacciare, e il sì del proprietario**: la banda è quella dei dispacci recenti — il compito 1,
   nella consegna precedente in archivio: l'implementatore ~267k token, il revisore ~430k.
4. L'**esecuzione del compito 2**, con `superpowers:subagent-driven-development`; il revisore rilancia ogni comando e
   confronta con `compare_task2.py`, e il suo prompt si scrive sul modello di `review-1-prompt.md`; alla chiusura del
   compito, i file del dispaccio nella cartella tracciata (punto 8). Poi il pre-controllo del compito 3, in un'altra sessione;
   e così compito per compito, fino al 9.

## Il pre-controllo del compito 3, del 2026-09-25

Tolto dal piano il 2026-09-25, quando la sessione dopo ha fatto il **pre-controllo del compito 3** e scritto *«Come si
riprende — il pre-controllo del compito 3»*. Il testo com'era, dal commit `fa4b3f4`, parola per parola; i rimandi sono
riscritti per questa cartella.

## Come si riprende — l'esecuzione del compito 2, 2026-09-25

✅ **Il compito 2 è eseguito, rivisto e curato.** La consegna precedente — il pre-controllo — sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

| Commit | Che cosa |
|---|---|
| `23b3134` | **il compito 2**, dall'implementatore — conforme al dettato: `compare_task2.py f00d5b7 23b3134` esce 0, e la revisione l'ha provato nelle due direzioni |
| `5f32158` | le cure della revisione, dal coordinatore (scelta **A** del proprietario, senza ri-revisione): **E14**, e **M-1** in `compare_task2.py` |
| il commit che scrive questa riga | il dispaccio del compito 2 nella cartella tracciata, e questa consegna |

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` a `5f32158`, sulla macchina `zagor`: sotto `gui/` il progetto `jsdom` con 17 file passati e uno saltato, 100 prove passate e una saltata, e il progetto `browser` con un file e **5** prove; `found 0 vulnerabilities` — si rilancia, non si cita: `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi sui due sistemi `f00d5b7`, `23b3134` — la **prima corsa col browser**: il flusso di lavoro non installa nessun browser, e il Chrome delle immagini di `ubuntu-latest` e `windows-latest` basta — e `5f32158`, lette alla chiusura; quella del commit che scrive questa riga **in corso**: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»*. ⚠️ Il log di un job vuole credenziali — l'API risponde 403 —: che il progetto `browser` abbia girato lo prova la guardia di **E10**, perché un progetto vuoto o un Chrome che non parte fanno rosso il passo |
| **la posizione** | la riga **2** a `✅ 2026-09-25`; la sua colonna **Commit** la scrive il compito 3 (R1-16): `23b3134`, con la cura `5f32158` |
| **il pezzo JavaScript** | `663.93 kB`, invariato: il compito 2 non tocca il pacchetto della SPA |
| **il dispaccio** | nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: il prompt **spedito**, `dispatch-task-2.md`, che prende il posto del modello — gli stessi testi coi valori della macchina `zagor` —, il rapporto dell'implementatore, il prompt del revisore, `review-2-prompt.md`, e la revisione; e `compare_task2.py` curato (M-1), il modello di `compare_task3.py` |
| **le copie** | il clone della revisione, `%TEMP%\rv2` sulla macchina `zagor`, è cancellato; la copia `%TEMP%\pc2` del pre-controllo, sulla macchina `Jays`, non serve più e si può cancellare |
| **le voci registrate, non prese** | ⚠️ **la finestra delle prove, 1440 × 900**: nessuna prova la tiene e nessun controllo del disegno lo chiede — misurato dal pre-controllo, e dalla revisione con la riga che la terrebbe, verde sul commit e rossa senza `viewport`, `expected [ 414, 896 ] to deeply equal [ 1440, 900 ]`. Il fatto nuovo della revisione: le prove della cornice del compito 8 sono scritte alla finestra delle prove, e senza sarebbero rosse, non verdi |
| **le due macchine** | quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, dove questa sessione ha lavorato: `core.autocrlf` `true` dal file di sistema e l'albero `w/crlf`, Node v24.19.0, Chrome `153.0.8010.53` con la `154.0.8037.58` già scaricata come `new_chrome.exe`; e quella dell'account `Jays`, col repository in `E:\ALL\DEV\MY_REPOS\daemon`, `core.autocrlf` `false` e l'albero `w/lf`, Chrome 154. ⛔ Sulla macchina che esegue, gli Attesi di **forma** si misurano, non si copiano (E72) |

**La revisione** — un revisore Opus fresco: **conforme**; **0** critici, **0** importanti, **2** minori, **0** nit, chiusi dal
commit di cura. Il costo misurato: l'implementatore **~223k** token, 104 chiamate, **~29** minuti; il revisore **~375k**, 164
chiamate, **~56** minuti — ~0,6 milioni in tutto, dentro la banda detta al proprietario (~0,7).

📌 **Ciò che questa sessione ha imparato, e che non era scritto** — nessuna voce è ancora un gotcha: le raccoglie la chiusura
del sotto-progetto.

| | Che cosa | Che cosa se ne fa |
|---|---|---|
| 1 | **la regola dei fine-riga del modello del dispaccio era falsa per un file che cresce**: *«CR uguale a prima»*, mentre `gui/vite.config.ts` passa da 53 a 114 CR, uguali alle righe — la nota N1 del rapporto, confermata dalla revisione | il modello del prompt del compito 3 dice la **forma**: CR uguali alle righe su un file CRLF, zero su un file LF, e la colonna `w/…` uguale |
| 2 | **uno script di confronto che mostra una differenza senza contarla esce verde**: `compare_task2.py` lasciava al lettore anche le due celle dettate, e prendeva la data dal commit stesso che rivedeva — M-1 | `compare_task3.py` nasce dal `compare_task2.py` curato: ciò che è dettato conta, ciò che non lo è si mostra |
| 3 | **una negazione è verde su un valore vuoto anche dentro un testo pre-controllato**: `not.toBe("0ms")`, la forma della trappola 1 del disegno; le violazioni del Passo 5 provavano la prima direzione della prova del movimento, e nessuna la seconda — E14 | nel pre-controllo, per ogni asserzione negativa: che cosa dice su un valore vuoto? |
| 4 | **Playwright lascia in `%TEMP%` un profilo vuoto, `playwright_chromiumdev_profile-*`, quando il LANCIO fallisce**: non quando le prove passano, né quando una è rossa | innocuo: si sa, e si toglie a mano se serve |
| 5 | **il log di un job della CI non si legge senza credenziali** (403) | il verdetto si legge job per job; che una parte del passo abbia girato lo deve provare una guardia del cancello, come E10 per il progetto `browser` |

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI del commit che scrive questa riga, per prima.
2. Il **pre-controllo del compito 3**, con le quattro domande di `CLAUDE.md` e le righe 5–8, contro il codice di **adesso**:
   il compito 2 ha cambiato `gui/package.json` e il lockfile, che il compito 3 tocca per `lucide` — `git diff --stat
   95068bb..HEAD -- gui/` e la lista *Files* del compito 3. Le voci che trova vanno nell'errata, e la prossima libera è
   **E15**; il dispaccio nasce nella cartella tracciata come **modello**, con la regola dei fine-riga della lezione 1, e
   `compare_task3.py` dal `compare_task2.py` curato, con la lezione 2.
3. L'**esecuzione del compito 3**, in un'altra sessione, col costo detto prima e il sì del proprietario; e così compito per
   compito, fino al 9.

## L'esecuzione del compito 3, del 2026-09-25

Tolto dal piano il 2026-09-25, quando la sessione dopo ha **eseguito il compito 3** e scritto *«Come si riprende —
l'esecuzione del compito 3»*. Il testo com'era, dal commit `9d2ffb0`, parola per parola; i rimandi sono riscritti per
questa cartella. ⚠️ I numeri della sua ricetta sono quelli del piano di `ef3e865`: le voci **E19**–**E23** dell'errata e
le cure dei recinti dei Passi 3, 5, 6 e 7, scritte da `9d2ffb0`, spostano i recinti che la ricetta nomina, e il Passo 7
ha una sesta sostituzione.

## Come si riprende — il pre-controllo del compito 3, 2026-09-25

✅ **Il pre-controllo del compito 3 è fatto, e ha trovato quattro difetti**: **E15**, **E16**, **E17** ed **E18**, scritti
nell'errata e **già applicati** al testo del compito, che si esegue com'è scritto adesso. Nessuno tocca il merito approvato,
quindi nessuno è del proprietario. La consegna precedente — l'esecuzione del compito 2 — sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

⚠️ **Perché è una misura e non un'impressione.** Il compito è stato **rifatto per intero dal testo del piano** su una copia
pulita di `fa4b3f4` — `git clone` in `%TEMP%\pc3`, sulla macchina `zagor`, senza `origin`, `npm ci`, poi i Passi 1–7 coi
recinti del piano, applicati con `apply_plan.py` della cartella di prova `%TEMP%\pds`, e la terza sostituzione del Passo 2
da uno script che prende il blocco dal file —, e ogni *Atteso* è tornato: Passo 1, `lucide@1.47.0` in `dependencies`, un
pacchetto nuovo, `ISC`, e il suo `LICENSE` dice MIT per le icone di Feather; Passo 2, `a11y.test.ts` verde con le sue 11
prove; Passo 3, rosso, `Failed to resolve import "./BaseIcon.vue"`; Passi 4–6, 2 file e 28 prove verdi, il *build* verde e il
pezzo JavaScript `663.93 kB` col nome di prima, `index-DLsd9Y_U.js`; Passo 7, il lint verde; Passo 8, ogni riga rossa per la
ragione scritta e il codice giusto verde; Passo 9, il passo web del cancello verde — `--project jsdom` 18 file passati e uno
saltato, 123 prove passate e una saltata; `--project browser` 1 file e 5 prove; `found 0 vulnerabilities`. Scritte le voci,
il compito è stato **rifatto dal testo corretto**, con la ricetta qui sotto, e `compare_task3.py` è stato provato su quel
commit: il commit fedele esce **0**, sedici percorsi `OK`; cinque mutanti escono **1** ciascuno — un token cambiato
in `BaseButton.vue`, un file in più, `lucide` non esatto, la riga 2 senza la cura, l'aiutante di `axe` lasciato in
`a11y.test.ts` —; e due cambi non dettati escono **0** con `CHECK BY HAND` — una voce d'errata in più, una data che non
è il giorno del commit. Su quel commit il passo web del cancello è verde con gli stessi numeri del Passo 9.

| Domanda | Esito, e il comando o la misura |
|---|---|
| 1 — la sonda è sbagliata? | no: le righe del Passo 8, una per volta sulla copia, rosse **per la ragione scritta** — i cinque messaggi del linter, un problema per corsa; `expected [ 'models' ] to deeply equal []`; `Unable to get [role="status"] within: <!--v-if-->` —, verde il codice giusto e verde il `v-if="$slots.default"` che il Passo 8 dice verde. E mordono anche le prove che il Passo 8 non muta: `iconOnly` fatto `computed` → `expected undefined to be 'Stacca'`; la riga `@ts-expect-error` tolta → `TS2322` nel *build*, e con un nome della mappa → `TS2578`; il gruppo di radio lasciato a sé → `expected [ 'false', 'true' ] to deeply equal [ 'true', 'false' ]`. Ogni file tornato dalla copia salvata, confrontato byte per byte, e `git status --porcelain` alla fine uguale a quello di prima |
| 2 — manca una sonda? | **sì, tre**: **E15**, **E16** ed **E18**. Le prime due hanno la stessa forma: una riga di un pezzo che porta il suo perché nel commento, tolta, lascia verdi tutte le prove |
| 3 — l'artefatto è sbagliato? | no, sulla copia dal testo; e **da fuori**: nella cartella di prova `%TEMP%\pds`, ramo `task8`, i compiti 4–8 sono costruiti sopra questi pezzi, e gli otto `Base*.vue`, `icons.ts` e `kit.test.ts` sono **uguali** a quelli rifatti oggi, a meno dei CR — nessun compito dopo li cambia; `testing/axe.ts` differisce per il commento di P-25 e per `contrastJudged` del compito 8, `eslint.config.js` per i blocchi dei compiti 4 e 5. `git diff --stat` fra la base e il compito, su `crates/` e `gui/schema/`, vuoto |
| 4 — è già eseguito? | no: `gui/package.json` non ha `lucide`, `gui/src/testing/` non esiste, e in `gui/src/components/` ci sono i soli `Confirm.vue`, `markdown.ts` e `markdown.test.ts` |
| 5 — il contratto è cresciuto sotto il piano? | sì, e regge. Il compito 2 ha portato due `.ts` nuovi, `browser.d.ts` e `tokens.browser.test.ts`, che il blocco `harness/ts` fa leggere al linter: verde; `eslint src` legge **48** `.ts`, prima nessuno, contati col rapporto JSON; il lint, a cache calda, da 10,5 a 11,5 s. I due progetti di `vitest`: i comandi dei Passi 2, 3 e 6 filtrano per file e girano, il Passo 8 dice `--project jsdom`. La colonna **Commit** della riga 2 è `` `23b3134`, con la cura `5f32158` ``: il Passo 9 dice *«l'hash del compito 2»*, e il prompt detta la cella intera, come fece per la riga 1 nel compito 2 |
| 6 — un commento o un banco lo smentisce? | **sì**: **E17**, la frase di `eslint.config.js` sulle parole singole. Le altre frasi dei file che il compito tocca reggono: la quarta sostituzione del Passo 7 corregge già quella sui `.ts`, e il commento che la terza sostituzione del Passo 2 toglie — *«on every text colour over every surface»*, falso dal compito 1, che prova il contrasto per le famiglie di P-1 — se ne va col suo aiutante |
| 7, 8 | non si applicano: il compito non tocca un ADR e non è un rapporto |

Lo stato alla chiusura, riga per riga col comando che la rifà:

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura, a `fa4b3f4`, sulla macchina `zagor` — sotto `gui/` il progetto `jsdom` con 17 file passati e uno saltato, 100 prove passate e una saltata, il progetto `browser` con un file e 5 prove, il pezzo JavaScript `663.93 kB`, `found 0 vulnerabilities` — e prima del commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verdi sui due sistemi le corse fino a `ee084d7`, il commit del pre-controllo, lette alla chiusura; quella del commit che scrive questa riga **in corso**: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: questa sessione ha scritto il piano e la cartella del dispaccio, e il codice del compito 3 vive solo nella copia |
| **la copia del pre-controllo** | ⚠️ **solo sulla macchina `zagor`**: `%TEMP%\pc3`, senza `origin`, coi rami `task3` — il compito rifatto dal testo di `fa4b3f4` —, `task3-cured` — con le correzioni di E15–E17 —, `base3` e `t3` — il piano di questa sessione e il compito rifatto dal suo testo, il confronto di `compare_task3.py` — e i sette mutanti `m1`…`m7`, e gli attrezzi in `%TEMP%\pc3-tools\`: le ricette, lo script della terza sostituzione, quello delle violazioni, quello delle correzioni, quello dei mutanti, e i log delle misure in `logs\`. Altrove non serve: il confronto col testo del piano lo fa `compare_task3.py` su qualunque clone. Si cancella dopo il compito 3 |
| **il dispaccio** | ✅ **viaggia con git**, per il punto 8 di *«Come si esegue un compito»*: nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/` il **modello** del prompt, `dispatch-task-3.md` — coi campi `<repo>`, `<HEAD>`, `<data>`, `<scratchpad>` e la tabella delle due macchine, e i fine-riga detti per **forma** (la lezione 1 della consegna precedente) —; `_extract_brief_3.py`, che scrive il brief nella cartella di lavoro ignorata; `compare_task3.py`, nato da `compare_task2.py` curato: ciò che è dettato conta, ciò che non lo è si mostra (la lezione 2). Il prompt del revisore si scrive sul modello di `review-2-prompt.md`. Il brief **non** si committa: è una copia del piano |
| **le voci registrate, non prese** | ⚠️ **la finestra delle prove, 1440 × 900**, com'era registrata dalla consegna precedente, in archivio: nessuna prova la tiene e nessun controllo del disegno lo chiede; le prove della cornice del compito 8 sono scritte a quella finestra |
| **le copie delle sessioni prima** | la copia `%TEMP%\pc2` del pre-controllo del compito 2, sulla macchina `Jays`, non serve più e si può cancellare |
| **le due macchine** | quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, dove questa sessione ha lavorato: `core.autocrlf` `true` dal file di sistema, l'albero `w/crlf` per i file che git ha scritto e `w/lf` per `gui/eslint.config.js` e `gui/src/a11y.test.ts`, Node v24.19.0, Chrome `153.0.8010.53` con la `154.0.8037.58` già scaricata come `new_chrome.exe`; e quella dell'account `Jays`, col repository in `E:\ALL\DEV\MY_REPOS\daemon`, `core.autocrlf` `false` e l'albero `w/lf`, Chrome 154. ⛔ Sulla macchina che esegue gli Attesi di **forma** si misurano, non si copiano (E72): il §4 del prompt lo dice |

📌 **La ricetta del compito 3**, per rifarlo o confrontarlo dal testo del piano, vale per il piano del commit che scrive questa
riga: `W` è il file intero dal recinto aperto a quella riga; `R` sostituisce l'occorrenza unica del primo recinto col secondo;
`B` è la terza sostituzione del Passo 2, il blocco preso **dal file** — `apply_plan.py` della cartella di prova non la
conosce, e sulla copia l'ha fatta uno script a parte. `lucide` si installa **prima**, a mano, col Passo 1.

```text
W gui/src/testing/axe.ts 1757
R gui/src/a11y.test.ts 1778 1785
R gui/src/a11y.test.ts 1791 1797
B gui/src/a11y.test.ts
W gui/src/components/kit.test.ts 1819
W gui/src/components/icons.ts 2064
W gui/src/components/BaseIcon.vue 2142
W gui/src/components/BaseButton.vue 2204
W gui/src/components/BaseLabel.vue 2356
W gui/src/components/BaseList.vue 2394
W gui/src/components/BaseStatus.vue 2438
W gui/src/components/BaseTextField.vue 2457
W gui/src/components/BaseRadioGroup.vue 2559
W gui/src/components/BaseDialog.vue 2669
R gui/eslint.config.js 2793 2799
R gui/eslint.config.js 2821 2828
R gui/eslint.config.js 2846 2856
R gui/eslint.config.js 2897 2904
R gui/eslint.config.js 2911 2917
```

📌 **Ciò che questa sessione ha imparato, e che non era scritto** — nessuna voce è ancora un gotcha: le raccoglie la chiusura
del sotto-progetto.

| | Che cosa | Che cosa se ne fa |
|---|---|---|
| 1 | **una riga che porta il suo perché nel commento può non avere una prova che cada senza di lei**: `inheritAttrs: false` di `BaseTextField` e il `v-bind` di R2-18 in `BaseDialog` — tolte, le 23 prove restavano verdi — E15, E16 | nel pre-controllo, per ogni riga di un artefatto che dichiara il suo perché: tolta, quale prova cade? |
| 2 | **un commento che quantifica su una cartella diventa falso quando il compito ci aggiunge file**: *«every `.vue` file here but `ViewBar` is single-word»* e gli otto `Base*.vue` — E17 | si rilegge ogni commento che dice *«ogni»* o *«tutti»* sui file che il compito fa crescere |
| 3 | **un blocco del linter si prova sul suo confine**, non solo sul caso che lo motiva: la mappa delle icone toglie `lucide` dal divieto e ci tiene l'altra regola, e la seconda metà non aveva la sua riga — E18 | il pre-controllo del **compito 4** guarda lo stesso sul blocco `harness/kit-page-specimens`, che spegne `no-raw-text` su `src/kit/**` |

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI del commit che scrive questa riga, per prima.
2. Questa sezione; poi il dispaccio: dalla radice del repository il brief, con
   `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_3.py`, e il prompt dal modello
   `dispatch-task-3.md` della stessa cartella, coi campi e i valori della macchina che esegue — il riquadro in testa al
   modello dice come.
3. ⛔ **Il costo, prima di dispacciare, e il sì del proprietario**: la banda è quella del compito 2, nella consegna
   precedente in archivio — l'implementatore ~223k token e ~29 minuti, il revisore ~375k e ~56 minuti.
4. L'**esecuzione del compito 3**, con `superpowers:subagent-driven-development`; il revisore rilancia ogni comando e
   confronta con `compare_task3.py`; alla chiusura del compito, i file del dispaccio nella cartella tracciata (punto 8).
   Poi il pre-controllo del compito 4, in un'altra sessione, con la prossima voce d'errata libera, **E19**; e così compito
   per compito, fino al 9.

## Il pre-controllo del compito 4, del 2026-09-25

Tolto dal piano il 2026-09-25, quando la sessione dopo ha fatto il **pre-controllo del compito 4** e scritto
*«Come si riprende — il pre-controllo del compito 4»*. Il testo com'era, dal commit `71a9f84`, parola per parola; i
rimandi sono riscritti per questa cartella.

## Come si riprende — l'esecuzione del compito 3, 2026-09-25

✅ **Il compito 3 è eseguito, rivisto e curato.** La consegna precedente — il pre-controllo — sta parola per parola in
[`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

| Commit | Che cosa |
|---|---|
| `c7b7bcd` | **il compito 3**, dall'implementatore — conforme al dettato: `compare_task3.py ef3e865 c7b7bcd` esce 0, sedici percorsi `OK`, e la revisione l'ha provato nelle due direzioni |
| `9d2ffb0` | le cure della revisione, dal coordinatore (scelta **A** del proprietario, senza ri-revisione): **E19**–**E23** nell'errata e nel codice, i recinti dei Passi 3, 5, 6 e 7 allineati — il piano rifà i quattro file curati, confrontati a macchina —, e **M-6** in `compare_task3.py` |
| `c79eaeb` | il dispaccio del compito 3 nella cartella tracciata, e questa consegna |
| il commit che scrive questa riga | la consegna completata nella stessa chiusura: la CI letta, e la riga su come si riprende dall'altra macchina |

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` a `9d2ffb0`, sulla macchina `zagor`: sotto `gui/` il progetto `jsdom` con 18 file passati e uno saltato, **125** prove passate e una saltata — le cento di prima e le 25 di `kit.test.ts` —, il progetto `browser` con un file e 5 prove; il pezzo JavaScript `663.93 kB`, invariato: il kit non entra ancora nel pacchetto della SPA; `found 0 vulnerabilities` — si rilancia, non si cita: `bash scripts/gate.sh`, **da solo** |
| **la CI** | verde sui due sistemi `c7b7bcd`, `9d2ffb0` e `c79eaeb`, lette alla chiusura; quella del commit che scrive questa riga **in corso**: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **la posizione** | la riga **3** a `✅ 2026-09-25`; la sua colonna **Commit** la scrive il compito 4 (R1-16): `` `c7b7bcd`, con la cura `9d2ffb0` `` |
| **il dispaccio** | nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: il prompt **spedito**, `dispatch-task-3.md`, al posto del modello — gli stessi testi coi valori della macchina `zagor` —; il rapporto dell'implementatore; il prompt del revisore, `review-3-prompt.md`, col messaggio della ripresa in coda; la revisione; e `compare_task3.py` curato (M-6), il modello di `compare_task4.py` |
| **le copie** | sulla macchina `zagor` restano il clone della revisione, `%TEMP%\rv3` — coi commit sonda della revisione e del coordinatore, staccati —, la copia del pre-controllo, `%TEMP%\pc3`, e i suoi attrezzi, `%TEMP%\pc3-tools`: non servono più e si possono cancellare, e questa sessione non l'ha fatto. Il banco `%TEMP%\pds` porta i pezzi del compito 3 di **prima** delle cure |
| **le voci registrate, non prese** | ⚠️ **la finestra delle prove, 1440 × 900**, com'era registrata dalle consegne precedenti; ⚠️ **N-4 della revisione**: un `BaseButton` `card` con una misura prende l'altezza di un controllo, e il testo esce — nessun compito del piano la usa, e se uno la userà la cura è sua; ⚠️ la strada **B** di **E23**, `lucide/…` preso nella regola; ⚠️ **il controllo dei pacchetti ritirati di `cargo audit` non fa rosso senza il registro**: nel cancello della revisione, alle 13:26–13:32, 39 righe `error: couldn't check if the package is yanked: registry: request could not be completed in the allotted timeframe`, e il cancello verde; in quello delle cure, zero. Tocca **X-3** dell'audit, ed è del proprietario |
| **le due macchine** | quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, dove questa sessione ha lavorato: `core.autocrlf` `true` dal file di sistema, Node v24.19.0, e Chrome **`154.0.8037.58`** — aggiornato **da sé** il 2026-09-25 alle 14:25, durante la revisione: la riga del §0 del modello del dispaccio che dice 153 non vale più —; e quella dell'account `Jays`, col repository in `E:\ALL\DEV\MY_REPOS\daemon`, `core.autocrlf` `false` e l'albero `w/lf`, Chrome 154. ⛔ Sulla macchina che esegue, gli Attesi di **forma** si misurano, non si copiano (E72) |
| **dall'altra macchina** | si riprende da `origin`, perché tutto ciò che serve è tracciato: questa sezione, la cartella del dispaccio e il piano. Restano **solo** su `zagor`, e non servono per riprendere: il registro git-ignorato `.superpowers/sdd/2026-09-23-design-system/progress.md` con gli script delle cure, le copie in `%TEMP%` della riga **le copie**, e le note di memoria dell'agente, che stanno fuori dal repository — le lezioni che contano sono nella tabella qui sotto. Sull'altra macchina l'albero è `w/lf`: le forme dei fine-riga si rimisurano lì (E72) |

**La revisione** — un revisore Opus fresco, **interrotto** a metà dalla chiusura del processo di Claude Code verso le 14:05
e **ripreso** alle 14:12 con `SendMessage`: **conforme**; **0** critici, **0** importanti, **6** minori, **6** nit. M-1…M-5 e
N-1…N-3 sono curati o dichiarati da **E19**–**E23**, M-6 da `compare_task3.py`; N-4 è registrato; N-5 e N-6 non chiedono
cure — una frase del rapporto dell'implementatore, e l'ordine *misure, cancello, commit*. Il costo misurato:
l'implementatore **~238k** token, 97 chiamate, **~33** minuti; il revisore **~385k** al rapporto finale — 29 chiamate e ~19
minuti dopo la ripresa, ~43 minuti prima —; ~0,6 milioni in tutto, dentro la banda detta al proprietario (0,6–0,9).

📌 **Ciò che questa sessione ha imparato, e che non era scritto** — nessuna voce è ancora un gotcha: le raccoglie la chiusura
del sotto-progetto.

| | Che cosa | Che cosa se ne fa |
|---|---|---|
| 1 | **l'aspetto di un pezzo non ha prove in jsdom**, che non applica il foglio: M-1 e M-2 si vedono solo nel browser vero | il pre-controllo del compito 4 chiede, fra le prove nel browser della pagina kit, il colore di ogni variante spenta contro `--color-text-disabled` e il bordo dell'errore sotto il puntatore (**E19**, **E20**) |
| 2 | **la cura di una direzione lascia scoperta l'altra**: E16 provava la finestra **senza** descrizione, e con la descrizione nessuna prova cadeva (M-3) | per ogni voce d'errata che aggiunge una prova: e l'altra direzione? |
| 3 | **una frase che si ripete si cura in tutte le sue case**: i segni `data-*` con `\|\| undefined` erano quattro, non tre — `data-error` l'ha trovato la cura | prima di curare, il `grep` della frase nel file e nei suoi fratelli |
| 4 | **`chrome.exe --version`, su Windows, apre il browser** col profilo dell'utente, e non stampa niente: il revisore l'ha fatto alle 14:26 e l'ha richiuso | la versione si legge dal file — `(Get-Item '<cartella>\chrome.exe').VersionInfo.ProductVersion` — o dal nome della cartella della versione; il modello del prompt del revisore del compito 4 lo dice |
| 5 | **Chrome si aggiorna da sé fra una corsa e l'altra**: alle 14:25 la 154 ha preso il posto della 153, e una corsa di Playwright in quel minuto è caduta con `browserType.launch: Target page, context or browser has been closed`; una alle 14:02, `Failed to connect to the browser session … within the timeout`, ha la causa non separata fra la memoria di P-21 e l'aggiornamento in attesa | un rosso del progetto `browser` si rilancia dopo aver letto la versione, prima di cercarne la causa nel codice |
| 6 | **un subagente interrotto dalla chiusura del processo non è perso**: il suo trascritto resta, e `SendMessage` lo riprende. ⚠️ E lo stato si legge **intero**: il messaggio della ripresa diceva *«nessun log del cancello intero»*, perché un `ls \| tail -20` aveva tagliato `gate-review-c7b7bcd.log` | prima della ripresa si misurano albero, clone, processi e log, senza `tail` sugli elenchi, e il messaggio dice lo stato |

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI del commit che scrive questa riga, per prima.
2. Il **pre-controllo del compito 4**, con le quattro domande di `CLAUDE.md` e le righe 5–8, contro il codice di **adesso**:
   il compito 3 e le sue cure hanno cambiato `kit.test.ts` — **25** prove, non 23 —, `BaseButton.vue`, `BaseTextField.vue`
   ed `eslint.config.js`, a cui il compito 4 aggiunge un blocco: i suoi *Trova* si rilanciano sul file di oggi. Le voci che
   trova vanno nell'errata, e la prossima libera è **E24**. Porta con sé tre cose: la sonda del browser per **E19** ed
   **E20** (lezione 1); il blocco `harness/kit-page-specimens`, che spegne `no-raw-text` su `src/kit/**`, provato sul suo
   **confine** (lezione 3 della consegna del pre-controllo del compito 3, in archivio); e il modello del dispaccio con la
   riga di Chrome del §0 rimisurata e la lezione 4. Il banco `%TEMP%\pds`, se serve, si rifà dal piano di oggi.
3. L'**esecuzione del compito 4**, in un'altra sessione, col costo detto prima e il sì del proprietario; e così compito
   per compito, fino al 9.

## L'esecuzione del compito 4, del 2026-09-26

Tolto dal piano il 2026-09-26, quando la sessione dopo ha eseguito il **compito 4** e scritto
*«Come si riprende — l'esecuzione del compito 4»*. Il testo com'era, dal commit `1b531e9`, parola per parola; i
rimandi sono riscritti per questa cartella.

## Come si riprende — il pre-controllo del compito 4, 2026-09-25

✅ **Il pre-controllo del compito 4 è fatto, e ha trovato quattro difetti**: **E24**, **E25**, **E26** ed **E27**, scritti
nell'errata e **già applicati** al testo del compito, che si esegue com'è scritto adesso. **E24** chiedeva una scelta fra due
cure: portata al proprietario, l'ha delegata — *«decidi secondo la skill decision-principles»* —, e la voce dice la scelta,
la via scartata e il costo. Nessuna tocca il merito approvato. La consegna precedente — l'esecuzione del compito 3 — sta
parola per parola in [`archivio/consegna-piano-design-system.md`](consegna-piano-design-system.md).

⚠️ **Perché è una misura e non un'impressione.** Il compito è stato **rifatto per intero dal testo del piano** su una copia
pulita di `71a9f84` — `git clone` in `%TEMP%\pc4`, sulla macchina `Jays`, senza `origin`, `npm ci`, poi i Passi 1–7 coi
recinti del piano, applicati da uno script —, e ogni *Atteso* è tornato **tranne uno**: la prima corsa del Passo 5, rossa
quattordici su quattordici per la cache delle dipendenze (**E24**). Passo 2, rosso, `Failed to resolve import "./Kit.vue"`;
Passo 5, alla corsa dopo, il progetto `browser` verde, il lint e il *build* verdi, il pezzo JavaScript `663.93 kB` col nome
del `main` di questa macchina, `index-CiZv4zPX.js`; Passo 6, rosso *«the kit page is in the package»* con la pagina fra gli
ingressi, e verde senza; Passo 7, ogni riga rossa per la ragione scritta, nei temi detti, e `git status --porcelain` alla fine
uguale a quello di prima. La pagina, aperta dal server di sviluppo della copia: nessun errore in console, e la scelta in cima
porta la radice da `dark` a `light`. Scritte le voci, il compito è stato **rifatto dal testo corretto** su una seconda copia,
`%TEMP%\pc4b`, con la ricetta qui sotto: la prima corsa del Passo 5 dopo il rosso del Passo 2 è **verde**, 23 prove; il passo
web intero verde — `--project jsdom` 18 file passati e uno saltato, 125 prove passate e una saltata; `--project browser` 2
file, 23 prove; `found 0 vulnerabilities` —; e ogni riga del Passo 7, le quattro nuove comprese, rossa sulle **sole** prove
che nomina. È lì che **E25** ha preso la sua seconda metà, lo smontaggio in `afterEach`: prima, la finestra rossa trascinava
a cascata la prova del puntatore. E `compare_task4.py` è stato provato su una terza copia, `%TEMP%\pc4c`, con questo piano
committato in locale come base: il commit fedele alla ricetta esce **0**, nove percorsi `OK`, e i suoi otto file sono
**uguali**, byte per byte, a quelli della copia `pc4b`; sei mutanti escono **1** ciascuno — un token cambiato in `Kit.vue`, un
file in più, la cella della riga 3 non dettata, la riga di **E24** tolta, lo smontaggio di **E25** tolto, `package.json`
toccato —; e due cambi non dettati escono **0** con `CHECK BY HAND` — una voce d'errata in più, una data che non è il giorno
del commit.

| Domanda | Esito, e il comando o la misura |
|---|---|
| 1 — la sonda è sbagliata? | **sì, a metà**: **E27**, il ramo *fuori dall'angolo* di `concentricRadii` che la pagina non esercitava mai — con un `throw` su quel ramo, le 23 prove verdi. Le altre righe del Passo 7 mordono per la ragione scritta |
| 2 — manca una sonda? | **sì, due**: **E25**, le prove nel browser di **E19** ed **E20**, che il compito 3 lasciava a questo pre-controllo; ed **E26**, il blocco `harness/kit-page-specimens` senza righe rosse, né sul bisogno né sul confine |
| 3 — l'artefatto è sbagliato? | **sì**: **E24**, la prima corsa del Passo 5 che cade per la cache delle dipendenze e non per la pagina; e la cascata che la seconda metà di **E25** cura. **Da fuori**: le due sostituzioni del compito 8 su `kit.browser.test.ts` si applicano una volta ciascuna al file corretto, e quello che ne esce tiene `colourOf` e `userEvent`; `concentricRadii`, `fits` e `iconsCentred` hanno le firme che i compiti 6 e 8 importano. `git diff --stat` fra la base e il compito, su `crates/` e `gui/schema/`, vuoto |
| 4 — è già eseguito? | no: `gui/kit.html`, `gui/src/kit/` e `gui/src/testing/probes.ts` non esistono |
| 5 — il contratto è cresciuto sotto il piano? | sì, e regge: il compito 3 e le sue cure hanno cambiato `BaseButton.vue`, `BaseTextField.vue`, `kit.test.ts` ed `eslint.config.js`, e i *Trova* dei Passi 4 e 6 e ogni violazione del Passo 7 si applicano una volta sul file di oggi. La colonna **Commit** della riga 3 è `` `c7b7bcd`, con la cura `9d2ffb0` ``: il Passo 8 dice *«l'hash del compito 3»*, e il prompt detta la cella intera. Il compito 8 contava le prove della pagina: allineato da **E25** |
| 6 — un commento o un banco lo smentisce? | no: il commento di `testing/axe.ts` promette già il contrasto acceso dalla pagina kit; quello di `harness/chat-renders-our-own-html` parla dell'unica eccezione a `vue/no-v-html`, il blocco nuovo dell'unica a `no-raw-text`; e quello di `vite.config.ts` sui progetti — *«everything else is written per project»* — regge con la riga di **E24** |
| 7, 8 | non si applicano: il compito non tocca un ADR e non è un rapporto |

Lo stato alla chiusura, riga per riga col comando che la rifà:

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura, a `71a9f84`, sulla macchina `Jays` — sotto `gui/` il progetto `jsdom` con 18 file passati e uno saltato, 125 prove passate e una saltata, il progetto `browser` con un file e 5 prove, il pezzo JavaScript `663.93 kB`, `found 0 vulnerabilities` — e prima del commit: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | verde sui due sistemi fino a `71a9f84`, letta in questa sessione; quella del commit che scrive questa riga **in corso**: la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **codice di prodotto** | non toccato: il codice del compito 4 vive solo nelle copie |
| **le copie del pre-controllo** | solo sulla macchina `Jays`, e non servono altrove — il confronto col testo del piano lo fa `compare_task4.py` su qualunque clone: `%TEMP%\pc4`, il compito dal testo di `71a9f84`; `%TEMP%\pc4b`, dal testo corretto; `%TEMP%\pc4c`, la prova di `compare_task4.py`. Si cancellano dopo il compito 4, con `%TEMP%\pc2` e il banco `%TEMP%\pds` di questa macchina, del 2026-09-23, vecchi |
| **il dispaccio** | nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: il **modello** del prompt, `dispatch-task-4.md`, coi campi della macchina da riempire; `_extract_brief_4.py`, che scrive il brief nella cartella di lavoro ignorata — le sonde delle tavole comprese, che il compito vuole lette per intero —; `compare_task4.py`, nato da `compare_task3.py`: ciò che è dettato conta, ciò che non lo è si mostra. Il prompt del revisore si scrive sul modello di `review-3-prompt.md`, con la lezione 4 della consegna precedente, in archivio: la versione di Chrome si legge dal file, mai con `chrome.exe --version` |
| **le voci registrate, non prese** | quelle della consegna precedente, in archivio: la finestra delle prove, 1440 × 900; **N-4** della revisione del compito 3; la strada B di **E23**; il controllo dei pacchetti ritirati di `cargo audit` senza il registro, che tocca **X-3** ed è del proprietario. ⚠️ **E due nuove**: la pagina kit mostra la finestra nella sola forma `center` — `sheet` e `full` le mostrano il cassetto del compito 5 e la Panoramica del compito 8 —, mentre la (b) dice *«ogni componente in ogni stato»*; e la guardia di non-vacuità del Passo 6, `test -f dist/index.html`, che in locale una `dist/` lasciata da un *build* precedente soddisfa anche se il *build* scrivesse altrove — non nella CI, che parte da un clone pulito |
| **le due macchine** | quella dell'account `Jays`, col repository in `E:\ALL\DEV\MY_REPOS\daemon`, dove questa sessione ha lavorato: `core.autocrlf` `false` in `.git/config` e l'albero `w/lf`, Node v24.19.0, Chrome `154.0.8037.58` letto dal file; e quella dell'account `zagor`, col repository in `C:\Users\zagor\Desktop\harness`, `core.autocrlf` `true` dal file di sistema, Node v24.19.0, Chrome `154.0.8037.58` dalla consegna precedente. ⛔ Sulla macchina che esegue, gli Attesi di **forma** si misurano, non si copiano (E72) |

📌 **La ricetta del compito 4**, per rifarlo o confrontarlo dal testo del piano, vale per il piano del commit che scrive questa
riga: `W` è il file intero dal recinto aperto a quella riga; `R` sostituisce l'occorrenza unica del primo recinto col secondo.

```text
W gui/src/testing/probes.ts 3082
R gui/vite.config.ts 3217 3226
W gui/src/kit/kit.browser.test.ts 3243
W gui/kit.html 3407
W gui/src/kit/main.ts 3424
W gui/src/kit/Kit.vue 3437
R gui/eslint.config.js 3710 3717
R scripts/gate-gui.sh 3746 3753
```

📌 **Ciò che questa sessione ha imparato, e che non era scritto** — nessuna voce è ancora un gotcha: le raccoglie la chiusura
del sotto-progetto.

| | Che cosa | Che cosa se ne fa |
|---|---|---|
| 1 | **un Atteso si misura sulla sequenza dei passi, non su una corsa pulita**: la corsa rossa del Passo 2 lascia una cache che il Passo 5 eredita, e il cancello — che svuota `node_modules/` — non la vede mai (**E24**) | nel pre-controllo, i passi si rifanno **in fila** sulla stessa copia, come li farà chi esegue, e un rosso che il cancello non vede si cerca nello stato che un passo lascia al successivo |
| 2 | **una prova che muove il puntatore vero vuole che ogni prova prima si pulisca anche quando cade**: una finestra di `reka-ui` rimasta montata lascia `pointer-events: none` sul `body` (**E25**) | le app montate da una prova si smontano in `afterEach`, mai sull'ultima riga della prova — la forma dei compiti 6 e 8 |
| 3 | **un ramo di una sonda che la pagina non percorre è una sonda vuota a metà**, e la guardia di non-vacuità non lo vede, perché conta i casi di **tutti** i rami insieme (**E27**) | per ogni ramo di una sonda: un `throw` sul ramo dice se è percorso; poi un caso verde e uno rosso |
| 4 | **una scelta tecnica reversibile e di poche righe, fuori dal merito approvato, si decide coi cinque criteri**: il proprietario ha rifiutato l'A/B di **E24** e l'ha delegata | l'A/B resta per il merito approvato — com'è stato per **E5**, **E6** ed **E10** —; il resto si decide, e la voce d'errata dice il perché e la via scartata col suo costo |

**Il prossimo passo** — una fase nuova, nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI del commit che scrive questa riga, per prima.
2. Questa sezione; poi il dispaccio: dalla radice del repository il brief, con
   `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_4.py`, e il prompt dal modello
   `dispatch-task-4.md` della stessa cartella, coi campi e i valori della macchina che esegue — il riquadro in testa al
   modello dice come.
3. ⛔ **Il costo, prima di dispacciare, e il sì del proprietario**: la banda è quella del compito 3, nella consegna
   precedente in archivio — l'implementatore ~238k token e ~33 minuti, il revisore ~385k.
4. L'**esecuzione del compito 4**, con `superpowers:subagent-driven-development`; il revisore rilancia ogni comando,
   confronta con `compare_task4.py` e **guarda** la pagina kit nel browser, nei due temi (punto 5 di *«Come si esegue un
   compito»*); alla chiusura del compito, i file del dispaccio nella cartella tracciata (punto 8). Poi il pre-controllo del
   compito 5, in un'altra sessione, con la prossima voce d'errata libera, **E28**; e così compito per compito, fino al 9.
