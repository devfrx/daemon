# R6 — Compito 13: la SPA, la cornice

Rapporto: `<scratchpad>/review/R6-report.md`. Vincoli: `constraints.md` (letto prima di questo).

## Perimetro
`## Compito 13:` del piano — per intero, per intestazione (1 760 righe: a blocchi di 200).

## Sezioni dei disegni assegnate
- `<disegno2>`: `### §6a` (≈281–316).
- `<stella>`: `## Il modello della GUI` (≈181–198), `### §2 — Viste e disposizione` (≈600–674), `### §4 — Lo spike di accettazione di dockview` (≈743–829), la tabella di `### §3` riga 7.
- Lo spike: `ls -R spikes/gui-shell/` e il file che contiene `createDockview` (`grep -rln 'createDockview' spikes/gui-shell/ --include=*.ts --include=*.js --include=*.html`): è il **precedente misurato** di D49 — confronta riga per riga ciò che il compito dice che «sale» (ponte `VueContent`, una `createApp` per pannello con `unmount` su `dispose`, le opzioni di `createDockview`, `lock` in **due** righe, il confronto canonico) e ciò che resta.

## Su che cosa insistere, oltre alle sei dimensioni
- ⛔ **Le versioni si rimisurano oggi**: `npm view dockview-core dockview pinia reka-ui vue-i18n version dist-tags time --json` (nella tua cartella di prova) — la **8.3.1** di `dockview-core` esiste ed è ancora la corrente della v8 (D2)? È uscita una 9? `reka-ui`: gli export `Dialog*` che il compito usa esistono in quella versione (leggi i `.d.ts` nel `.tgz`: `npm pack reka-ui@<v>` nella cartella di prova e `tar -tzf`/`grep`)? `dockview-core` non spedisce il CSS (P-80): il compito installa **due** pacchetti e importa il foglio da quello giusto — verifica nel `.tgz` di entrambi (`npm pack`, `tar -tzf … | grep -i css`).
- `createDockview` e le opzioni che lo spike ha esercitato; `VueContent`; `lock('strip')` → `locked` **e** `header.hidden` (D49, P-82); `PANEL_TYPES` e il **segnaposto** (D47): le tre prove della §8 che vogliono un componente — sono nel 13? Le tre viste come JSON e `title: id` (P-95: chi mette il titolo dalla locale, il 13 o il 14?); il confronto **canonico** della disposizione (`toJSON` e la forma salvata: coincide con ciò che `SaveLayout`/`Layout` del compito 3 trasportano — `String`? byte?); `stores/core.ts`, `stores/layout.ts` (nomi e campi: il 14 li usa con questi nomi? scheletro + `grep -n`); `stamp.ts` legge l'**ultima riga** di `ipc_v1.map` (D52) — la forma della mappa dettata dal compito 3 (modello del generatore, P-62/D35): l'ultima riga è davvero il timbro, in esadecimale, e la conversione a `BigInt` è corretta per un valore sopra `MAX_SAFE_INTEGER`? `tokens.css` e `contrast.test.ts` (D53: nel 13 o nel 14? `--stop` cambia a `#ec5f57` dove?); `locales/it.json` e la rete `copy.test.ts` a **due** `it` (D51, P-105); barra, chip, fascia, striscia, cassetto — fuori o dentro la griglia (D50); `window.harnessFake` (D57: 13 o 14?); `moveActive` è del 14 (D49): il 13 non lo nomina come proprio? `Frame.vue` monta `Confirm.vue` (D60: il 14 lo aggiunge — il 13 lascia il posto?). Il criterio di chiusura chiede al revisore di **aprire la SPA nel browser** contro la finta: con `createFakeBridge` che rigioca solo su richiesta (P-90), che cosa vede il revisore del 13 senza il `deliverAll` del 14? La sonda `axe` (P-86) nel 13? Ogni «Atteso» su file futuri: sul **modello**. Le fixture lette da `gui/schema/fixtures/` (D12) — il percorso relativo da `gui/src/` è quello giusto?
- **P-75…P-85, P-95, P-97** (tutte quelle che nominano il 13): rimisurale. **Numeri di compito**: D3/P-81 (`markdown-it` al 14), D47, P-85 — ogni «il 14» e «il 13» nel compito è quello giusto per la posizione.

## ⛔ Richiamo del 2026-09-16 — la ripresa: che cosa è cambiato nel 13 da quando questo mandato fu scritto

- Il rapporto è `<scratchpad>/review/R6-report.md` con lo scratchpad di **oggi** (il richiamo in testa a `constraints.md`). Il compito 13
  si estrae dal piano di oggi (`awk '/^## Compito 13:/{f=1} /^## Compito 14:/{exit} f' <piano>`), a blocchi di 200 righe; il cammino di
  lettura sopra resta valido; le righe «≈» dei disegni si ritrovano col `grep -n`, non si usano come numeri.
- Il registro `ledger.md`, sezione «Compito 13», elenca le correzioni GIÀ applicate: R9b-11 (**D80**: la disposizione salvata è per vista),
  R9b-12 (**D81**: `beforeunload` salva solo se qualcosa è cambiato), **C13-1 → D89** (il dock segue lo store: `watch([view, arrivals])` in
  `createDock`, lo store conta gli arrivi che non sono l'eco del proprio `SaveLayout`, `Frame.switchTo` è `layout.view = view`), R10-9,
  R10-10, R7-6, D75, D76, e il capoverso sotto il Passo 17. ⛔ **D80, D81 e D89 sono state scritte dal coordinatore SENZA revisione, sul
  modello letto e mai compilato**: sono il primo posto in cui insistere. Rileggi il flusso intero — `main.ts` → `Hello` → il `Layout`
  dell'accoglienza in `layout.receive` → `createDock` → `apply`/`show` → `settle` → `SaveLayout` → l'eco — e di' se il `Layout`
  dell'accoglienza viene mostrato davvero, se l'eco è riconosciuta dai byte, se uno `switchTo` non copia la vista spedita nell'archivio
  (decisione 11 del coordinatore della stella), se `arrivals` non conta due volte, e se le sonde nuove di `stores.test.ts` e
  `frame.test.ts` distinguono davvero il modulo giusto da uno sbagliato (la seconda direzione).
- ⛔ **COMPILA i moduli riscritti** — `layout.ts`, `dock.ts`, `Frame.vue`, `stores.test.ts`, `frame.test.ts` — e con essi tutto ciò che il 13
  detta (`tokens.css`, `registry.ts`, `Placeholder.vue`, `Strip.vue`, le tre viste JSON, `main.ts`, `locales/it.json`, `stamp.ts`,
  `contrast.test.ts`, `copy.test.ts`, …). Come: in `<scratchpad>/review/probe-R6/gui/` (o in una cartella corta
  `C:\Users\zagor\AppData\Local\Temp\probe-R6\gui\` se `npm` o `vite` inciampano sulla lunghezza del percorso: dillo) ricostruisci il modello
  dell'**11** dal piano di **oggi** — i file che il compito 11 detta (`package.json`, `.npmrc`, `tsconfig.json`, `vite.config.ts`,
  `index.html`, `src/schema/*`, `src/transport/*`, `schema/fixtures/*.json`) — poi `npm ci` col lockfile di `<scratchpad>/review/model11-lock/`
  (se `npm` rifiuta per `engines.node`, `node` è v24.9.0 e P-64/P-65 lo sanno: togli `.npmrc` e dillo, non è un rilievo), poi `npm install`
  dei pacchetti che il Passo 3 del 13 aggiunge, alle versioni che detta; poi i file del 13. Lancia `npx vue-tsc --noEmit` e
  `npx vitest run`, e riporta ogni rosso col messaggio **intero**, il file e il Passo che lo detta: rilievo `fatto`, `sì`. Il `Frame.vue`
  del 13 è quello del 13 (il 14 lo ridetta: non è tuo). Le fixture `gui/schema/fixtures/*.json` non esistono nel repo (il 3 non è
  eseguito): il generatore del 3 le descrive (scheletro, *Interfaces* del 3, e il Passo del 3 che scrive i `.json`); se una sonda le
  legge, alimentala con una forma coerente con quel Passo e **dillo**. `dockview` sotto `jsdom`: il Passo 3 del 13 dice che cosa
  aspettarsi; se non monta, è un rilievo con l'errore intero.
- Le versioni si rimisurano oggi come dice il mandato sopra (`npm view …`): `dockview-core` e `dockview` **8.3.1** (D2), `reka-ui`, `pinia`,
  `vue-i18n`, `jsdom` alle versioni del Passo 3. I file del 13 nascono **LF** (vincolo 4).
