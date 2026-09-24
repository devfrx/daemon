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
