# Rapporto R3 — il compito 4 (la pagina kit), il compito 5 (il kit al lavoro) e il «Come si riprende»

Piano `docs/superpowers/plans/2026-09-23-design-system.md` a `d589d15`; fetta: righe 2661–3298 (compito 4), 3299–4012
(compito 5), 4013–4028 (*«Come si riprende»*). Cartella di prova: `C:\Users\zagor\AppData\Local\Temp\probe-ds3`.

**Come ho provato.** `gui/` e `scripts/gate-gui.sh` copiati da `d589d15`, `npm ci`, poi le quattro dipendenze nuove alle
versioni appuntate (`lucide@1.47.0`, i due `@fontsource` 5.3.0, `@vitest/browser-playwright@4.1.11`, `playwright@1.63.0`).
I compiti 1–3 applicati **dal testo del piano** con uno script che prende i blocchi per numero di riga (i blocchi *Crea*, le
*Trova/Sostituisci* con la logica di `replace_unique.py`, `extract_tokens.py` e `rename_tokens.py` lanciati come dettati):
tutte le sostituzioni hanno trovato il testo una volta sola. Poi i compiti 4 e 5, passo per passo, con le prove dettate
lanciate davvero (Chrome installato, progetto `browser`). Una sola toppa fuori fetta per andare avanti: il cast di
`kit.test.ts` del compito 3 (vedi *«Fuori fetta»*). La cartella di prova ha un suo `git` per tornare indietro.

## Rilievi

### R3-1 — Importante — la violazione 1 del passo 7 non fa rosso dove il piano dice, e le radici `kit-card` sono cieche
- Dove: piano righe 3284–3286, compito 4, passo 7, riga 1; e la pagina di righe 3143–3157.
- Che cosa dice il piano: «in `BaseButton.vue` `border-radius: var(--radius-card)` … | rosso: `concentricRadii`, un
  `base-button` in `kit-card`».
- Che cosa è vero: la prova della pagina, *«keeps every radius concentric (answer 4)»*, resta **verde** nei due temi; il
  rosso viene solo dalla prova della finestra, e dice `base-button in base-dialog`. Nessun `BaseButton` della pagina sta
  vicino a un angolo di una `kit-card`: ogni scheda chiude con una nota, e la griglia **stira** le schede di una riga
  all'altezza della più alta (`display: grid` senza `align-items`), così nemmeno la scheda «Lista», che non ha la nota,
  porta la sua ultima riga nell'angolo. Nessun angolo di una `kit-card` ha qualcosa vicino: `near = 10` è 4 da
  `kit-frame`, 2 da `kit-strip` e 4 dalla scheda «Scelta» — il pallino **dentro** il radio, non l'angolo della scheda.
  La guardia `near > 0` è verde lo stesso, perché conta su tutte le radici insieme. Prova a parte: il raggio sbagliato
  sulle righe di `BaseList` (`--radius-card` al posto di `--radius-control`) resta **verde** su tutta la pagina.
- Prova: nella cartella di prova, violazione 1 → `FAIL … > opens its window with the radii concentric, and no axe violation`
  (light e dark), `+ "base-button in base-dialog, bottom-right: radius 16.0, outer 20.0, distance 13.0/13.0"`,
  `Tests 3 failed | 16 passed (19)` — il terzo rosso è la prova del compito 2 in *«Fuori fetta»*. Il rapporto delle sonde
  sulla pagina com'è dettata: `{"rootsCount":9,"near":10,"bad":[],"seen":87,"boxed":87,"icons":47,"centred":47}`, e sulla
  finestra `{"dialogs":1,"dnear":1}`; per radice: `Pulsanti=0, Etichette=0, Campo=0, Scelta=4, Lista=0, Stato=0,
  Finestra=0, kit-frame=4, kit-strip=2`. Raggio sbagliato in `BaseList.vue` → `Tests 1 failed | 18 passed` (solo il rosso
  del compito 2). Con `align-items: start;` in `.kit-grid` la pagina resta verde (`Tests 14 passed`) e lo stesso raggio
  sbagliato fa rosso: `"base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 13.0/13.0"` e
  `bottom-right`. E la frase del piano *«ciascuna chiude con una nota»* (riga 2687, e il commento di `Kit.vue`) è falsa
  per la scheda «Lista» (righe 3061–3069), che chiude con la lista: la verità del verde, oggi, è lo stiramento della
  griglia. Con la griglia non stirata le righe della lista e i pulsanti della finestra passano per **tolleranza**, non
  per costruzione: raggio 8 dentro 20 a distanza **13** (12 di margine più 1 di bordo) vorrebbe 7, e la sonda accetta
  1,5 px.
- Correzione proposta: (1) la riga 1 della tabella dice il rosso vero — *«rosso: la prova della finestra, nei due temi,
  `base-button in base-dialog, bottom-right: radius 16.0, outer 20.0, distance 13.0/13.0`»*; (2) `Kit.vue` porta
  `align-items: start;` in `.kit-grid`, con una riga di commento (una scheda stirata allontana dall'angolo ciò che la
  chiude), e il passo 7 aggiunge la violazione su `BaseList.vue` col rosso `base-list-row in kit-card`. Così le radici
  `kit-card` guardano davvero qualcosa; la guardia per radice resta una scelta del piano.

### R3-2 — Importante — la violazione 2 del passo 7 è vacua: `fits` resta verde
- Dove: piano riga 3287, compito 4, passo 7, riga 2.
- Che cosa dice il piano: «in `Kit.vue` `.kit-card` con `width: 12rem` e `overflow: hidden` | rosso: `fits`, un testo
  tagliato».
- Che cosa è vero: la prova resta **verde** nei due temi. A 12rem tutto va a capo — le righe dei pulsanti hanno
  `flex-wrap`, i campi si stringono (`min-width: 0`), le note e le etichette vanno a capo — e niente è più largo della
  scheda.
- Prova: violazione 2 → `Tests 1 failed | 18 passed (19)`, e l'unico rosso è la prova del compito 2. Due violazioni che
  mordono davvero, provate: `.kit-note { … white-space: nowrap; overflow: hidden; }` → `+ "cut: kit-note \"Principale,
  normale, dis\" 490>422"` nei due temi; `.kit-strip > .base-button { margin-right: -40px; }` → `+ "sticks out:
  base-button of kit-strip"` nei due temi.
- Correzione proposta: al posto della riga 2 le due righe qui sopra, una per metà di `fits` — il testo tagliato e ciò che
  sborda —, coi rossi scritti come sopra.

### R3-3 — Importante — la violazione 3 del passo 7 è vacua: `iconsCentred` salta le icone che la violazione sposta
- Dove: piano riga 3288, compito 4, passo 7, riga 3.
- Che cosa dice il piano: «in `BaseLabel.vue` `.base-label` con `align-items: flex-start` e `min-height:
  var(--size-control-lg)` | rosso: `iconsCentred`, un'icona fuori centro — o nessuna centrata: si **legge** quale».
- Che cosa è vero: nessuno dei due. La sonda — com'è in `sonda-icone.js` — misura il centro **solo** se il genitore è una
  riga flex con `align-items: center`; con `flex-start` le icone delle etichette escono dal conteggio, e le altre
  (pulsanti, campi, elenco delle icone) restano centrate: `centred > 0`, `problems` vuoto, **verde**.
- Prova: violazione 3 → `Tests 1 failed | 18 passed (19)` (solo il compito 2). Una violazione che morde, provata: lasciare
  il genitore centrato e spostare l'icona, `.base-label :deep(.base-icon) { … position: relative; top: 3px; }` →
  `+ "off centre by 3.00 px: modules in base-label"` e altre dodici righe uguali, nei due temi.
- Correzione proposta: la riga 3 con la violazione `position: relative; top: 3px` sull'icona di `BaseLabel`, rosso
  *«off centre by 3.00 px: … in base-label»*; e una parola nel commento di `iconsCentred` che dica ciò che la sonda salta
  (un genitore non centrato non è giudicato), perché è lì che la prima forma della violazione è caduta.

### R3-4 — Importante — la violazione 4 del passo 7 non tocca la prova della pagina: il rosso è del compito 2
- Dove: piano riga 3289, compito 4, passo 7, riga 4; prova di righe 2889–2899.
- Che cosa dice il piano: «in `tokens/index.ts` tolta la riga di Geist | rosso: la famiglia del testo non è `Geist
  Variable`».
- Che cosa è vero: `firstFamily` legge la famiglia **dichiarata** dello stile calcolato, che resta `"Geist Variable",
  system-ui, sans-serif` anche senza nessun `@font-face` — è il controllo *«applied»* di `sonda-caratteri.js`, non il
  *«loaded»*. La prova della pagina resta **verde**; va rossa quella del compito 2, *«load both fonts…»*.
- Prova: violazione 4 → `FAIL |browser (chromium)| src/tokens/tokens.browser.test.ts > … > load both fonts, and draw the
  tool font's digits at one width` — `AssertionError: expected 0 to be greater than 0`; nessun rosso in
  `src/kit/kit.browser.test.ts`. Una violazione che la prova della pagina vede: la famiglia tolta dalla pila dichiarata
  (in prova, `--font-family-text: system-ui, sans-serif;` in `base.css`) → `expected 'system-ui' to be 'Geist Variable'`
  nei due temi.
- Correzione proposta: la riga 4 dice il rosso vero (la prova del compito 2, `expected 0 to be greater than 0`) — o, se
  la riga vuole provare la prova della pagina, una violazione **dentro `Kit.vue`**, per esempio `.kit { font: 400
  0.875rem/1.25rem serif; }`, che non tocca i fogli copiati dalla tavola (`base.css` cambiato manda rosso anche
  `board.test.ts`).

### R3-5 — Importante — `git checkout -- Kit.vue` al passo 7 non può funzionare: il file non è ancora di git
- Dove: piano righe 3282 e 3287, compito 4, passo 7.
- Che cosa dice il piano: «Una alla volta, nel progetto `browser`, poi indietro con `git checkout -- <file>`», e la riga 2
  tocca `Kit.vue`.
- Che cosa è vero: `Kit.vue` nasce al passo 3 e si committa al passo 8; al passo 7 è **non tracciato**, e `git checkout
  -- <file>` lo rifiuta. Le due violazioni che propongo in R3-2 sono anch'esse in `Kit.vue`.
- Prova: `echo x > gui/src/kit/Untracked.vue && git checkout -- gui/src/kit/Untracked.vue` → `error: pathspec
  'gui/src/kit/Untracked.vue' did not match any file(s) known to git`, uscita 1.
- Correzione proposta: per `Kit.vue` «indietro» è la copia salvata prima (`cp Kit.vue <scratchpad>/Kit.vue.bak`, poi il
  ritorno) — o `git add gui/src/kit` prima del passo 7 e `git checkout -- <file>` dopo. Detto nel passo.

### R3-6 — Importante — ogni rosso del browser lascia file nel sorgente, e il piano non lo sa
- Dove: compito 4, passi 2, 5 e 7 (cinque rossi voluti al passo 7); la radice è la configurazione del compito 2 (righe
  1421–1439).
- Che cosa dice il piano: niente — `grep -n 'screenshot\|attachments' docs/superpowers/plans/2026-09-23-design-system.md`
  non rende nulla.
- Che cosa è vero: in `vitest` 4.1.11 `browser.screenshotFailures` vale di base `!browser.ui`, cioè **sì** con `vitest
  run`: a ogni prova rossa nel browser nascono `gui/src/<cartella>/__screenshots__/<file>/<prova>-1.png` e
  `gui/.vitest-attachments/<hash>.png`. Nessuno dei due percorsi è in `.gitignore`; `git checkout -- <file>` non li
  toglie; un `git add gui/src` li porta nel commit, e un cancello rosso nel repository li lascia nell'albero.
- Prova: `node_modules/vitest/dist/chunks/reporters.d.DtoKVV2s.d.ts:1705–1709` — «Should Vitest take screenshots if the
  test fails / @default !browser.ui»; dopo la prima corsa rossa, nella prova, `git status --porcelain` → `?? gui/.vitest-
  attachments/` e `?? gui/src/tokens/__screenshots__/`; `find` → `src/tokens/__screenshots__/tokens.browser.test.ts/the-
  tokens--in-a-real-browser--…--1.png` e `.vitest-attachments/7b34becf….png`. `.gitignore` della radice letto: ignora
  `/gui/node_modules/`, `/gui/dist/`, `/gui/fake-core/target/` e basta.
- Correzione proposta: nel compito 2 `screenshotFailures: false` nel progetto `browser`, detto accanto (il cancello non
  guarda immagini) — o le due righe in `.gitignore`; e nel passo 7 del compito 4 una riga che controlla `git status
  --porcelain` pulito dopo il ritorno. È del compito 2 — lo segnalo qui perché morde il 4 cinque volte.

### R3-7 — Minore — la prova `axe` della pagina non ha guardia di non-vacuità
- Dove: piano righe 2901–2905 e 2917, compito 4, passo 2; vincolo globale 11 (riga 111).
- Che cosa dice il piano: il vincolo 11 vuole che *«ogni prova del browser porta la guardia di non-vacuità, quante cose ha
  guardato, maggiore di zero»*; le due prove `axe` asseriscono solo `toEqual([])` su `violations(…)`, che rende le sole
  violazioni.
- Che cosa è vero: oggi la prova guarda davvero — `color-contrast` giudica **79** nodi per tema, **0** `incomplete` — e la
  violazione 5 la manda rossa; ma un giorno in cui `axe` mettesse il contrasto fra gli `incomplete` (è ciò che fa sotto
  jsdom) la prova resterebbe verde senza aver guardato niente.
- Prova: `axe.run(wrapper.element, { rules: { "color-contrast": { enabled: true } } })` nella prova →
  `{"theme":"light","pass":79,"incN":0}` e `{"theme":"dark","pass":79,"incN":0}`.
- Correzione proposta: accanto alle due prove `axe`, un conteggio — i nodi di `color-contrast` fra i `passes` maggiori di
  zero e nessuno fra gli `incomplete` — con un `axe.run` diretto nella prova o un'opzione in `testing/axe.ts`.

### R3-8 — Minore — la violazione 5 del passo 7 manda rosso anche `contrast.test.ts`, non solo `board.test.ts`
- Dove: piano riga 3290.
- Che cosa dice il piano: «rosso: `axe`, `color-contrast` — ⛔ e anche `board.test.ts`, che lo vuole».
- Che cosa è vero: vero, e in più `contrast.test.ts`, *«dark: every pair reads at its threshold»*. Nel browser il rosso è
  in due prove per il tema scuro: la pagina e la finestra (`#reka-dialog-description-…`, il pulsante `quiet`).
- Prova: `npx vitest run --project jsdom src/tokens` con la violazione → `× themes.css is the board's block, byte for
  byte`, `× dark: every pair reads at its threshold (WCAG 2.2, 1.4.3 and 1.4.11)`, `Tests 2 failed | 13 passed (15)`.
- Correzione proposta: *«e anche `board.test.ts` e `contrast.test.ts`, che la vogliono»*.

### R3-9 — Minore — `git checkout --` su questa macchina riscrive in CRLF i file nati LF
- Dove: compito 4, passo 7 (e ogni passo «indietro con `git checkout`» dei compiti 1–5); vincolo globale 9.
- Che cosa dice il piano: «i file nuovi nascono **LF**».
- Che cosa è vero: `core.autocrlf=true` sta nella configurazione di sistema di Git per Windows; un `git checkout -- <file>`
  riscrive il file in CRLF. L'indice resta LF — `git diff` non vede niente — ma la rimisura dei CR che il piano chiede
  dopo ogni compito li trova.
- Prova: `git config --show-origin --get core.autocrlf` nel repository → `file:C:/Program Files/Git/etc/gitconfig true`;
  nella prova, dopo i ritorni del passo 7, `tr -cd '\r' < gui/src/components/BaseButton.vue | wc -c` → `141` (era 0).
- Correzione proposta: una riga nel passo 7: dopo i ritorni i file toccati si rimisurano, e un file che l'indice tiene LF
  e l'albero CRLF è l'effetto di `core.autocrlf`, non un difetto — oppure il ritorno con la copia salvata (R3-5), che non
  cambia i fine-riga.

### R3-10 — Nit — la prova delle due direzioni del cancello usa il nome deprecato `rollupOptions`
- Dove: piano righe 3276–3278, compito 4, passo 6.
- Che cosa dice il piano: «`build: { rollupOptions: { input: { index: "index.html", kit: "kit.html" } } },`».
- Che cosa è vero: in Vite 8.3.0 `build.rollupOptions` è `@deprecated Use rolldownOptions instead`, ma vale ancora
  (`buildConfig.rolldownOptions ??= buildConfig.rollupOptions`): la direzione funziona.
- Prova: `grep -n 'rollupOptions\|rolldownOptions' node_modules/vite/dist/node/index.d.ts` → righe 947–964;
  `node_modules/vite/dist/node/chunks/node.js:2727`. Con la riga dettata: `dist/kit.html`, `dist/assets/kit-*.js` e
  `kit-*.css`, e il controllo del cancello stampa `the kit page is in the package`, uscita 1.
- Correzione proposta: `rolldownOptions`, il nome della versione appuntata.

### R3-11 — Nit — la pagina kit ha il margine di 8 px del browser
- Dove: `gui/kit.html` (righe 2934–2947) e `Kit.vue` (`.kit { min-height: 100vh; … }`).
- Che cosa è vero: la regola `html, body, #app { height: 100%; margin: 0; }` vive in `App.vue` (P-4), che la pagina kit
  non monta: nel browser `getComputedStyle(document.body).margin` → `8px`, e con `min-height: 100vh` la pagina scorre
  sempre di 16 px. Il fondo è giusto (lo porta `base.css` sul `body`), quindi si vede appena. *«A grandezza vera»* resta
  vero.
- Prova: `npm run dev` nella prova, `/kit.html`, dalla console → `{"bodyMargin":"8px","scrollH":1478,"innerH":768}`.
- Correzione proposta: `body { margin: 0; }` in uno `<style>` non scoped di `Kit.vue`, come `App.vue` — o niente, detto.

### R3-12 — Importante — «M-3 chiusa per costruzione» nei tre pannelli non ha una prova: rimetterla aperta lascia tutto verde
- Dove: piano righe 3572–3645 (passo 4) e 3711–3713 (Impostazioni), compito 5; *Interfaces* e *Da* (controlli 11–13).
- Che cosa dice il piano: il passo 4 si chiama *«M-3 chiusa per costruzione»*; l'unica prova di M-3 nel compito è il passo
  8, a mano. La prova di `BaseStatus` del compito 3 (righe 1683–1695) prova il **pezzo**, non chi lo usa.
- Che cosa è vero: la costruzione regge solo se ciascun chiamante mette `BaseStatus` **fuori** dal `v-if`. Rimettendolo
  dentro il `v-if` in tutti e tre i pannelli — cioè riaprendo M-3 — nessuna prova se ne accorge: `frame.test.ts` guarda il
  testo, `a11y.test.ts` le violazioni, `modules.test.ts` la classe `.event`.
- Prova: nella prova, dopo i passi 1–6, `<BaseStatus v-if="connection.phase !== 'connected'">` in `Band.vue`, `<BaseStatus
  v-if="core.lastVerdict !== null">` in `Status.vue`, `<BaseStatus v-if="invoke.inFlight !== null">` in `Settings.vue` →
  `npx vitest run --project jsdom` → `Test Files 18 passed | 1 skipped (19)`, `Tests 123 passed | 1 skipped (124)`.
- Correzione proposta: nel passo 1 del compito 5 tre prove in forma di quella del compito 3 — per la fascia, collegati →
  `[role="status"]` c'è ed è vuota; `StaleBuild` → **lo stesso elemento** (`toBe(region.element)`) porta le parole; per
  Stato prima e dopo un `Verdict`; per Impostazioni prima e dopo un `Invoke` — rosse sul codice di oggi (i `role="status"`
  nascono col `v-if`), verdi dopo il passo 4, e rosse con la mutazione qui sopra.

### R3-13 — Importante — il passo 8 chiede di sentire un annuncio che la costruzione non promette, e salta la terza regione
- Dove: piano righe 3991–4004, compito 5, passo 8.
- Che cosa dice il piano: «1. al caricamento la fascia dice *«Il core non ha risposto.»*: la si sente?» e «⛔ Se un
  annuncio **non** si sente, è una voce d'errata: M-3 non è chiusa».
- Che cosa è vero: (1) al caricamento `connection.phase` vale già `"waiting"` (`const phase = ref<Phase>("waiting")` in
  `gui/src/stores/connection.ts`), quindi la regione di `BaseStatus` e le parole della fascia nascono **nello stesso
  montaggio**: è proprio il caso di M-3 — *«le regioni nascono nel DOM insieme al loro testo»* — che la costruzione non
  cura, perché non c'è un «prima» vuoto. Un «non si sente» al punto 1 è atteso dal meccanismo, e la regola del piano lo
  farebbe diventare una voce d'errata che dice M-3 aperta. I punti 2–3 (la fascia se ne va, `StaleBuild` la fa rientrare)
  e il 4 sono le prove vere. (2) Le regioni sono **tre** — la (e): `Band.vue`, `Settings.vue`, `Status.vue` — e il passo
  prova la fascia e Impostazioni, non la riga dell'evento di Stato; Stato è nella vista Home, accanto a Impostazioni.
- Prova: `gui/src/stores/connection.ts` riga 22 (`tr -d '\r' < … | grep -n 'phase'`); la (e) del disegno, riga 403; le
  viste di oggi, `home: status | permissions | settings | knowledge | activity | costs | strip`; il `Verdict` c'è fra le
  fixture del ponte finto (`bridge.deliver("Verdict")` in `modules.test.ts`); il testo *«Ultima richiesta di VRAM:
  rifiutata»* in `it.json`.
- Correzione proposta: il punto 1 diventa un'osservazione e non un criterio (*«al caricamento regione e parole nascono
  insieme: si annota che cosa si sente, e non decide M-3»*); si aggiunge un punto 5, *«`harnessFake.deliver("Verdict")`:
  si sente «Ultima richiesta di VRAM: rifiutata…»?»*; la regola dell'errata vale per i punti 3, 4 e 5. La decisione 21
  resta com'è: il passo è a mano, con l'Assistente vocale, col verbale.

### R3-14 — Minore — la chiave costruita `settings.theme.${choice}` non ha la sonda che le chiavi costruite hanno
- Dove: piano righe 3681 (`t(\`settings.theme.${choice}\`)`) e 3452–3472 (le parole), compito 5.
- Che cosa è vero: `@intlify/vue-i18n/no-missing-keys` è cieca alle chiavi costruite (lo dice `copy.test.ts`, P-105), e
  `copy.test.ts` sonda solo `modules.*`. Una scelta nuova in `THEME_CHOICES` senza la sua parola mostrerebbe la chiave
  cruda nelle Impostazioni, a lint e prove verdi.
- Prova: `tr -d '\r' < gui/src/locales/copy.test.ts` — due prove, *«has a name for every module type»* e la cartella del
  linter; nient'altro.
- Correzione proposta: nel passo 2 una prova in `copy.test.ts`, *«has a word for every theme choice»*, su `THEME_CHOICES`,
  come quella dei moduli su `PANEL_TYPES`.

### R3-15 — Nit — `Placeholder.vue` fra i file «riscritti per intero», ma il passo cambia due righe
- Dove: piano righe 3306–3308 (*Files*) e 3843–3847 (passo 6).
- Che cosa dice il piano: «Rewrite: … `gui/src/panels/Placeholder.vue`, … — ciascuno **per intero**»; il passo 6 fa due
  *Trova/Sostituisci*.
- Correzione proposta: `Placeholder.vue` sotto *Modify*.

### R3-16 — Nit — «i due `document.createElement("button")`» di `BigTab.ts` sono una chiamata sola, usata due volte
- Dove: piano righe 3318–3319.
- Che cosa è vero: una sola `document.createElement("button")`, dentro l'aiutante `command`, chiamato per `float` e per
  `page`: due pulsanti. `grep -n 'createElement' gui/src/frame/BigTab.ts` → righe 25, 29, 39.
- Correzione proposta: *«i due pulsanti che `frame/BigTab.ts` crea con `document.createElement`»*.

### R3-17 — Critico — «Come si riprende», riga 6: `--dv-overlay-z-index: var(--z-floating)` non abbassa i gruppi galleggianti
- Dove: piano riga 4025; la (c) del disegno, riga *«i livelli»*; trappola 9; decisione 23.
- Che cosa dice il piano: «`--dv-overlay-z-index: var(--z-floating)`» nel tema nostro, perché i gruppi galleggianti
  stiano sotto i dialoghi.
- Che cosa è vero: in `dockview` 8.3.1 il contenitore galleggiante ridefinisce la variabile **su sé stesso**,
  `.dv-resize-container { --dv-overlay-z-index: var(--dv-overlay-z-index, 999); }` (riga 1037 di `dockview.css`): è un
  ciclo, e la variabile vale «invalida» su quell'elemento qualunque cosa imposti un antenato. `dockview-core` poi scrive
  in linea `z-index: calc(var(--dv-overlay-z-index, 999) + 2i)`, che cade sul ripiego: **999**. Con la variabile del
  tema a 50 sul guscio del nostro tema, il gruppo galleggiante resta a 999, sopra `--z-overlay` (200).
- Prova: nel browser dei test della prova, `createDockview` col tema `{ name: "harness", className:
  "dockview-theme-harness" }`, `--dv-overlay-z-index: 50` sul guscio (`.dv-shell.dockview-theme-harness`), un
  `addFloatingGroup` → `{"inline":["calc(var(--dv-overlay-z-index, 999) + 0)"],"computed":["999"],"varOnFloating":[""]}`.
  Con la regola sul contenitore stesso, `.dockview-theme-harness .dv-resize-container { --dv-overlay-z-index: 50; }` →
  `{"computed":["50"],"handles":["50","50"]}`. Le righe: `grep -n -- '--dv-overlay-z-index'
  gui/node_modules/dockview/dist/styles/dockview.css` (1037, 1203) e `grep -n 'overlay-z-index'
  gui/node_modules/dockview-core/dist/package/main.esm.mjs` (12023). **E nella SPA vera**, sulla prova dopo il compito
  5 (`npm run dev`, `harnessFake.deliverAll()`, «Stacca la tessera» su Stato, poi «+ moduli»): il gruppo galleggiante
  `z-index` **999**, il velo e il foglio del cassetto **200**, e lo schermo mostra la tessera Stato **sopra** il velo e
  sopra il foglio; con `--dv-overlay-z-index: 50` sul guscio (`.dv-shell`) resta **999**; con la regola sul contenitore
  va a **50** e la tessera finisce sotto il velo e sotto il foglio. È il difetto che la trappola 9 teme, **già oggi**, e la
  cura scritta non lo toglie.
- Perché Critico: il *«Come si riprende»* dice che queste forme sono *«già decise … e da non ridecidere senza una misura
  nuova»*; scritto così, il compito 6 produrrebbe un `dock.css` che compila e passa una prova che legge il file, e i
  gruppi galleggianti resterebbero sopra ogni dialogo — la Panoramica del compito 8 compresa.
- Correzione proposta: la riga 6 dice *«`--dv-overlay-z-index: var(--z-floating)` **sul contenitore galleggiante**,
  `.dockview-theme-harness .dv-resize-container`, perché `dockview.css` lo ridefinisce lì come un ciclo»*, e il compito 6
  lo prova nel browser con un gruppo galleggiante vero (z-index calcolato = `--z-floating`, e la regola tolta → 999).
  **Da segnalare al proprietario**: la trappola 9 e la decisione 23 restano giuste nello scopo, ma il meccanismo scritto
  nel disegno non funziona com'è scritto. Nota a margine per chi scrive i compiti 6–8: in 8.3.1 il contenitore
  galleggiante porta **`role="dialog"`** (`dv-resize-container dv-resize-container-with-titlebar`), quindi con il dock
  montato `[role="dialog"]` — il selettore che il compito 5 dà alle prove della conferma — non è più univoco.

### R3-18 — Importante — «Come si riprende», riga 7: la geometria di `moveActive.ts` estratta com'è manda «giù» alla prima colonna
- Dove: piano riga 4026; la (d), *«le frecce nella griglia»*, e la decisione 19.
- Che cosa dice il piano: «l'aiutante `nearest(from, candidates, direction)` estratto dalla geometria di `moveActive.ts`».
- Che cosa è vero: la geometria di oggi ordina i candidati **solo** per la distanza sull'asse della direzione (`gap`), e
  a parità tiene il primo in ordine. In una griglia di carte tutte le carte della riga sotto hanno la stessa distanza:
  «giù» da qualunque colonna va alla **prima** carta della riga sotto, «su» dalla terza colonna va alla prima. La (d) vuole
  che *«giù va alla riga sotto»*, e la ragione della decisione 19 è proprio che *«in una griglia «giù» andrebbe a
  destra»* con `RovingFocusGroup`.
- Prova: la geometria di `moveActive.ts` (`beyond`, `gap`, `sort`) copiata in `nearest-check.mjs` su una griglia 3 × 2 →
  `r0c0 down -> r1c0`, `r0c1 down -> r1c0`, `r0c2 down -> r1c0`, `r1c2 up -> r0c0`.
- Correzione proposta: la riga 7 dice che `nearest` aggiunge, a parità di distanza, la vicinanza sull'**altro** asse (la
  distanza fra i centri, o la sovrapposizione), e che la prova coi rettangoli a mano include una griglia di tre colonne con
  «giù» dalla colonna di mezzo. Se `moveActive` passa a usare `nearest`, anche i pannelli ne guadagnano.

### R3-19 — Importante — «Come si riprende», riga 7: che cosa fa `settle` mentre è aperta una vista col nome non è scritto
- Dove: piano riga 4026; D3 (riga 197).
- Che cosa dice il piano: «`LayoutPack.named?` … e `openNamed?: string` (D3) … nel negozio `openNamed` e `saveNamed(name,
  layout)` … il dock guarda anche `openNamed`».
- Che cosa è vero: oggi — e dopo il compito 1 — `settle(layout)` scrive **sempre** in `layouts[view.value]`
  (`layouts: { ...(saved.value?.layouts ?? {}), [view.value]: layout }`, righe 828–836 del piano); con D3 `view` resta un
  `ViewName` anche mentre è aperta una vista col nome. Se il compito 7 non instrada `settle` su `named`, il primo
  movimento di un pannello in una vista col nome **sovrascrive la disposizione della vista di base** — Home o Lavoro —
  senza errori. Lo stesso per `receive`, che oggi riporta solo `view`.
- Prova: `tr -d '\r' < gui/src/stores/layout.ts` (la `settle` di oggi); il testo del compito 1, righe 827–855 del piano.
- Correzione proposta: la riga 7 aggiunge *«`settle` scrive nella vista col nome aperta quando `openNamed` c'è, e in
  `layouts[view]` altrimenti; una prova del negozio lo tiene: una mossa in una vista col nome lascia `layouts.home`
  com'era»*. E una parola su D4: lo specimen della pagina kit (`BaseTextField model-value="Home"` con *«Esiste già una vista
  con questo nome.»*, riga 3050) dice che anche i nomi delle tre viste di sempre sono «già usati»; la riga 7 dice solo *«un
  nome già usato»*.

### R3-20 — Importante — «Come si riprende», riga 8: il pulsante «moduli» della striscia non può aprire il cassetto com'è oggi
- Dove: piano riga 4027; compito 5, `Drawer.vue` (righe 3533–3570).
- Che cosa dice il piano: «la striscia a pillola con i «moduli» `BaseButton pill` che apre il cassetto».
- Che cosa è vero: la striscia è un **pannello del dock** (`strip` in tutte e tre le viste), e ogni pannello è **un'app
  Vue sua** (`VueContent`: *«ONE VUE APP PER PANEL»*, e solo Pinia è condivisa); il cassetto del compito 5 tiene `open` in
  un `ref` **locale** di `Drawer.vue`, montato nella barra. Un `emit`, un `provide/inject` o un `v-model` dalla striscia
  non attraversano due app: serve uno stato in un negozio, o il cassetto spostato.
- Prova: `tr -d '\r' < gui/src/frame/VueContent.ts` (la testa e `createApp` in `init`); `node -e` sulle viste →
  `home: … | strip`, `work: … | strip`, `compact: knowledge | strip`; il `Drawer.vue` del piano, `const open = ref(false)`.
- Correzione proposta: la riga 8 dice dove vive lo stato del cassetto — per esempio un campo in un negozio di `stores/`,
  letto da `Drawer.vue` e scritto dalla striscia — e che la barra perde il suo pulsante del cassetto quando «moduli» scende
  nella striscia (la (d): *«Il pulsante «Moduli» scende nella striscia»*).

### R3-21 — Minore — «Come si riprende», riga 6: `colorScheme` che segue il tema non fa niente in `dockview-core` 8.3.1
- Dove: piano riga 4025 (*«`colorScheme` da `shownTheme` … aggiornato con `api.updateOptions({ theme })`»*); la (c).
- Che cosa è vero: `api.updateOptions({ theme })` esiste e chiama `updateTheme()` (`if ("theme" in options)
  this.updateTheme();`), ma `updateTheme()` usa `className`, `gap`, `edgeGroupCollapsedSize`, `dndOverlayBorder`,
  `dndOverlayMounting`, `tabGroupIndicator` — **non** `colorScheme`, che nel JavaScript compare solo dentro i temi già fatti.
  Il tipo lo dice: *«Useful for adapting panel content colors»*. Seguire il tema costa un osservatore e una chiamata, e non
  cambia niente di ciò che `dockview` disegna.
- Prova: `grep -n 'colorScheme' gui/node_modules/dockview-core/dist/package/main.esm.mjs` → solo le righe 11413–11478 (i
  temi); `updateTheme() {` alla riga 18355, letto.
- Correzione proposta: la riga 6 lo dice, e il compito 6 decide se tenere il campo come dato per chi legge `api` (un
  `readToken` nostro basta per il canvas) o fissarlo: **da segnalare al proprietario**, perché *«`colorScheme` che segue
  `data-theme`»* è testo approvato della (c).

### R3-22 — Minore — il disegno conta «sette campi» di `DockviewTheme`; la 8.3.1 ne ha undici
- Dove: la (c) del disegno (riga 334) e *«Verificato nel codice»* (riga 504); la riga 6 del *«Come si riprende»* ci si
  appoggia.
- Che cosa è vero: oltre ai sette, `dndTabIndicator`, `dndOverlayBorder`, `tabGroupIndicator`, `tabAnimation`.
  `dndOverlayBorder` è il campo che il tema usa per `--dv-drag-over-border` — il bordo della zona d'arrivo che la (c) vuole
  `--color-mark` —, e `tabGroupIndicator` governa i gruppi di linguette che P-6 lascia fuori.
- Prova: `awk '/^export interface DockviewTheme \{/,/^\}/' gui/node_modules/dockview-core/dist/cjs/dockview/theme.d.ts
  | grep -E '^\s+[a-zA-Z]+\??:'` → undici righe; `"version": "8.3.1"`.
- Correzione proposta: la riga 6 nomina `dndOverlayBorder` (o dice che il bordo passa dalla variabile CSS, che
  `updateTheme` lascia al foglio quando il campo manca); il disegno, col richiamo datato, **da segnalare al proprietario**.

### R3-23 — Minore — «Come si riprende», righe 3, 8 e 9: tre omissioni rispetto ai compiti e al disegno
- Dove: righe 4022, 4027, 4028.
- Che cosa è vero: (1) riga 3, `BaseTextField` senza `type` — l'interfaccia del compito 3 (riga 1522) lo ha, e il compito
  5 lo usa (`type="search"`); (2) riga 8, le prove del browser della striscia senza *«nessuna scheda vicina a un angolo
  della pagina»* (controllo 19) e la Panoramica senza la prova `axe` (controllo 17); e nessuna parola su come disegnare
  nella miniatura il foglio `strip`, che è in tutte le viste e non ha un'icona in `ICONS`; (3) riga 9, la riga
  «Accessibilità» senza il **richiamo datato** e senza il comando del riquadro rilanciato prima e dopo (la (e) e il
  controllo 21).
- Correzione proposta: le tre aggiunte, una riga ciascuna.

### R3-24 — Minore — la via della tastiera sul radio nuovo è vera, ma nessuna prova la tiene
- Dove: piano righe 3683–3693 (il commento di `choosePolicy`), compito 5; la trappola 10 del disegno (*«la sottigliezza
  sul `change` si rilegge contro il componente nuovo»*).
- Che cosa dice il piano: *«an arrow key asks the way a click does»* e *«after an arrow key the focus sits on the radio
  the arrow reached, while the check stays on the core's value»*.
- Che cosa è vero: vero, misurato — ma le prove riscritte del passo 1 usano solo il clic, come le vecchie usavano solo
  `setValue`. In `reka-ui` 2.10.4 la freccia passa da `RovingFocusGroup` e da un `setTimeout` che clicca l'elemento
  raggiunto (`handleFocus` in `RadioGroupItem.js`): una via diversa dal clic, che un aggiornamento di `reka-ui` può
  cambiare senza che niente diventi rosso.
- Prova: nel progetto `browser` della prova, Impostazioni montato, `Policy` consegnato, clic sul radio corrente,
  `userEvent.keyboard("{ArrowDown}")` → `{"sentAfterClick":0,"sent":[{"kind":"Invoke","value":{"function":"vram-policy",
  "argument":"local"}}],"checked":["true","false"],"focusedIsSecond":true}`.
- Correzione proposta: questa prova, nel browser (sotto jsdom il fuoco e il `setTimeout` di `reka-ui` sono fragili),
  accanto a quelle del passo 1 — con la seconda direzione: il clic sul valore corrente non manda niente.

### R3-25 — Minore — il pezzo JavaScript cresce al compito 5, e il piano lo misura solo ai compiti 1 e 3
- Dove: *«Le voci aperte che questo piano SA»*, riga 212 (*«il compito 1 e il 3 **misurano** il pezzo JavaScript»*);
  compito 5, passo 6.
- Che cosa è vero: ai compiti 1–4 i pezzi di base non sono ancora usati dalla SPA, e il pezzo non si muove; al compito 5
  entrano (e con loro le ventiquattro icone della mappa e il radio di `reka-ui`), e il pezzo cresce di **25,59 kB**, **8,44
  kB** compressi. La (e) dice che il design system non peggiora N-2 di E187, *«ed è una deduzione»*: la misura la smentisce
  di poco, proprio dove il piano non misura.
- Prova: nei log di `npm run build` della prova — dopo i compiti 1–3 `index-DeW1hRQt.js 663.93 kB │ gzip: 201.53 kB`,
  uguale dopo il 4; dopo il 5 `index-m28g1T6-.js 689.52 kB │ gzip: 209.97 kB`.
- Correzione proposta: la misura anche nel commit del compito 5 (e del 6 e dell'8); e una riga al proprietario, che N-2
  è sua: **da segnalare**, con la cifra e il comando (`npm run build 2>&1 | grep 'kB'`).

## Fuori fetta, e perché li scrivo

Due difetti dei compiti 2 e 3 che la mia prova ha dovuto aggirare, perché fermano i comandi della mia fetta.

- **Compito 3, `kit.test.ts` riga 21** (piano riga ~1624): `const List = BaseList as Component;` → `vue-tsc` rosso,
  `error TS2352: Conversion of type '<T>(__VLS_props: …' to type 'Component' may be a mistake … Property 'item' is missing
  in type 'Readonly<InternalSlots>' but required in type '__VLS_Slots'`. `npm run build` del compito 3 — e quindi dei
  compiti 4 e 5 — non parte. In prova: `as unknown as Component`, poi verde.
- **Compito 2, la prova *«keep the focus ring under Windows' high contrast»*** (righe 1281–1297): rossa, `expected false
  to be true` alla riga 77 del file. La ragione, misurata: nessuno posa `data-theme` sulla radice in quel file, e tutti i
  ruoli di `themes.css` vivono sotto `[data-theme]` — `--color-focus` non esiste, e la scorciatoia `outline:
  var(--focus-width) solid var(--color-focus)` è invalida al valore calcolato, cioè niente contorno. Con
  `document.documentElement.dataset.theme = "dark"` dentro quella prova, il file passa (`Tests 5 passed (5)`). Ferma il
  passo 5 del compito 4 (`npx vitest run --project browser && …`) e il passo 6 del 5 (`npm test && …`).
- **Compito 2, `screenshotFailures`**: vedi R3-6. In più, il nome del file della prova qui sopra, nel repository, fa un
  percorso di **261** caratteri (`C:/Users/zagor/Desktop/harness/` + 230): in prova `git add -A gui` è morto con
  `error: open("gui/src/tokens/__screenshots__/…"): Filename too long` e `fatal: adding files failed`; `core.longpaths` non
  è impostato.

## Verificato e giusto

- **Compito 4, `probes.ts`**: `concentricRadii` è `sonda-raggi.js` riga per riga — `radius`, `effective`, la salita
  all'antenato col raggio, `reach = max(R, r) + 2`, l'angolo «dentro» a 1,5 px, l'SVG saltato — con le radici come
  parametro; `fits` è il quarto controllo di `sonda-caratteri.js` (tagliato e sbordare, `seen`/`boxed`); `iconsCentred` è
  `sonda-icone.js` senza la parità (qui c'è un set solo) e senza il salto `.ic` (qui non ci sono messaggi); `firstFamily` è
  il controllo *«applied»*. Le guardie `near`, `seen`, `boxed`, `icons`, `centred` ci sono (manca solo quella di `axe`,
  R3-7). Letti i tre file delle tavole.
- **Compito 4, la pagina e le prove, sul testo dettato**: `vue-tsc` e `vite build` verdi; `npm run lint` verde; il passo
  2 rosso per la ragione giusta (`Failed to resolve import "./Kit.vue"`); `src/kit/kit.browser.test.ts` **14 prove
  verdi** nei due temi, e **non vacue**: 9 radici, `near` 10, `bad` vuoto, `seen` 87, `boxed` 87, 47 icone tutte
  centrate, famiglie `Barlow` / `Geist Variable` / `Barlow`, `axe` col contrasto su 79 nodi e 0 incompleti, la finestra
  aperta nel `body` col tema della radice (`rgb(251, 248, 243)` chiaro, `rgb(36, 31, 32)` scuro — D7 regge).
- **Compito 4, il cancello**: dopo `npm run build` `dist/` ha solo `index.html` e `assets/`; le due righe dettate sono
  verdi; con `kit.html` fra gli ingressi nascono `dist/kit.html`, `kit-*.js`, `kit-*.css` e il controllo esce 1 con *«the
  kit page is in the package»*. `/kit.html` servita da `npm run dev` (HTTP 200, il titolo *«Harness — il kit»*); la
  scelta del tema in cima cambia `data-theme` sulla radice — guardata nel browser, chiaro e scuro.
- **D8**: senza il blocco `harness/kit-page-specimens` `eslint src` dà **33** errori `no-raw-text` in `Kit.vue`; col
  blocco è verde.
- **Violazione 5 del passo 7**: rossa come dice il piano — `color-contrast` nella pagina e nella finestra, tema scuro.
- **Compito 5, i testi da sostituire esistono com'è citato**: `retarget_confirm.py` trova 2, 4 e 1 (`".confirm button"`,
  `".confirm"` in `modules.test.ts`; `".confirm"` in `a11y.test.ts`); i due ancoraggi del blocco *Impostazioni*; le
  *Trova* di `it.json`, `Status.vue` (template e import), `Placeholder.vue` (pulsante e import), `eslint.config.js` —
  tutte una volta sola, dopo i compiti 1–3 applicati. Le riscritture per intero conservano il terminatore del file.
- **Compito 5, le prove**: dopo il passo 1 rosse le **5** di Impostazioni e verdi le altre 19 (la conferma per ruolo
  passa già col `Confirm.vue` di oggi); dopo i passi 2–6 jsdom tutto verde (141 prove passate nei due progetti, l'unico
  rosso è la prova del compito 2 di *«Fuori fetta»*), `vue-tsc` e `vite build` verdi, nessun avviso di Vue o di i18n nel
  log. `modules.test.ts`: `it` da 12 a 13, *Impostazioni* da 4 a 5; ciò che le quattro vecchie asserivano c'è ancora
  (spento per ciascun radio invece che per il `fieldset`; `aria-checked` invece di `.checked`; clic invece di
  `setValue`), più la prova del tema. `a11y.test.ts`: stesse prove, un selettore.
- **P-8** letto nei file spediti di `reka-ui` 2.10.4: `RadioGroupRoot` usa `useVModel(…, { passive: props.modelValue ===
  void 0 })` — con un valore dato, `null` compreso, non tiene stato suo; `RadioGroupItem` passa `onUpdate:checked` a
  `changeModelValue`; `Radio.handleClick` emette `select` e si ferma se `defaultPrevented`. E nel browser il gruppo
  controllato fa ciò che il commento di `Settings.vue` dice (R3-24).
- **`layout.chooseTheme`** lo crea il compito 1 — *Interfaces* riga 241, codice righe 841–845 — e `THEME_CHOICES`,
  `isThemeChoice` righe 723–727.
- **Le regole del linter del passo 7**: verde sul codice; le tre violazioni rosse col messaggio dettato (`a button is
  BaseButton`, `a list is BaseList`, `'reka-ui' import is restricted … panels and frame use the base pieces`); il blocco
  ripete `LUCIDE` e quindi non toglie la regola delle icone a `panels/` e `frame/`.
- **Tutti i `var(--…)`** dei file dei compiti 4 e 5 esistono in `base.css` o `themes.css`; la coppia della fascia
  (`--color-text-warn` su `--color-bg-warn-subtle`) è fra quelle che `contrast.test.ts` giudica.
- **Passo 8, i comandi**: `harnessFake` c'è in sviluppo (`main.ts`), le fixture `Accepted`, `StaleBuild`, `Policy`,
  `Verdict` ci sono; `Win + Ctrl + Invio` è la scorciatoia dell'Assistente vocale; il passo è a mano, del proprietario o di
  chi rivede, col verbale nella riga 5 — la forma della decisione 21.
- **«Come si riprende», righe 2–5**: dicono ciò che i compiti 2–5 fanno (le omissioni in R3-23). **Righe 6–9**, i fatti
  che reggono: `DockviewTheme` ha `name`, `className`, `colorScheme`, `gap`; `api.updateOptions` esiste e con `theme` chiama
  `updateTheme()`, che mette `gap` come margine della griglia (P-7); `api.layout(width, height)`; `SerializedDockview` ha
  `grid.{root,width,height,orientation}`, `panels`, `floatingGroups?` (e `popoutGroups?`, `edgeGroups?`); l'orientamento si
  alterna a ogni livello (`_deserializeNode` passa `orthogonal(orientation)` ai figli); P-6 rilanciato — 103 variabili, due
  blocchi `.dockview-theme-abyss {` (righe 1735 e 1787), 63 variabili in quei blocchi, 9 `--dv-tab-group-color-*`;
  `PANEL_TYPES` ha 18 tipi, e `ICONS` ha `float` e `fullPage` per i due comandi della presa grande; `BigTab.ts` oggi crea
  i due pulsanti col testo `⧉`/`⤢` e il nome da `menu.float`/`menu.page`; la riga 9 è coerente con l'obiettivo della testa.

## Non verificato, e perché

- **La CI** (`ubuntu-latest`, `windows-latest`) e l'altra macchina: non si toccano da qui.
- **L'Assistente vocale**: è il passo a mano della decisione 21; ho verificato solo che i comandi del passo esistano e che
  cosa la costruzione promette (R3-13).
- **Il cancello intero** `scripts/gate-gui.sh` nella prova: il suo primo passo compila il core finto contro le crate del
  repository, che la prova non ha; ho lanciato le due righe nuove del compito 4 da sole, dopo `npm run build`.
- **Le fonti di `vitest` citate nella riga 2** del *«Come si riprende»* (i file `docs/…` alla v4.1.11): sono del compito
  2; ho controllato solo che la riga dica ciò che il compito 2 fa.
- **I compiti 1–3** li ho applicati per poter provare i miei, non rivisti: i difetti che mi hanno fermato stanno in
  *«Fuori fetta»*.
- **Le prove in linea delle sessioni** che la (f) nomina accanto a *«testo tagliato e niente che sborda»*: non sono nel
  repository; ho confrontato `fits` con `sonda-caratteri.js`, che le porta.

## Comandi rilanciati

Nella cartella di prova `C:\Users\zagor\AppData\Local\Temp\probe-ds3`, salvo dove è detto il repository (solo letture).

- `tar … gui scripts/gate-gui.sh | tar -x`, `npm ci`, `npm install --save-exact @fontsource-variable/geist@5.3.0
  @fontsource/barlow@5.3.0 lucide@1.47.0`, `npm install --save-dev --save-exact @vitest/browser-playwright@4.1.11
  playwright@1.63.0`.
- `python apply123.py <piano> .` (compiti 1–3 dal testo), `extract_tokens.py`, `rename_tokens.py` (undici `ok:`), e la
  terza sostituzione di `a11y.test.ts` presa dal file.
- `npm run build` (dopo 1–3: `TS2352` in `kit.test.ts`; con la toppa verde), `npx vitest run` (dopo 1–3: `1 failed | 126
  passed | 1 skipped`).
- Compito 4: `npx vitest run --project browser src/kit` (passo 2, rosso), `npx vitest run --project browser` (passo 5),
  `npm run lint`, `npm run build`, le due righe del cancello (verde, poi rosso con `rollupOptions.input`), `npx eslint src`
  senza il blocco D8, `npx vite --port 5199` + `curl /kit.html`, il pannello del browser su `/kit.html`.
- Compito 4, passo 7: le cinque violazioni dettate e cinque mie (`run-violations4.sh 1 … 10`), `npx vitest run --project
  jsdom src/tokens` con la violazione 5; `axe.run` coi conteggi di `color-contrast`; la griglia con `align-items: start`.
- Compito 5: `python apply5.py <piano> . 1…7`, `npx vitest run src/panels/modules.test.ts src/a11y.test.ts` (passo 1),
  `npx vitest run`, `npm run build`, `npx eslint src`, `lint5.sh` (le tre violazioni), la mutazione di M-3 con `npx
  vitest run --project jsdom`, la prova della tastiera nel browser.
- «Come si riprende»: `node nearest-check.mjs`; la prova di `dockview` nel browser (z-index del gruppo galleggiante,
  prima e dopo la cura); la SPA della prova con `npm run dev`, un gruppo staccato e il cassetto aperto, con le due
  varianti della regola e lo schermo guardato.
- Sul repository, in sola lettura: `git status -sb`, `git ls-files --eol …`, `git config --show-origin --get
  core.autocrlf`, `grep`/`awk`/`sed -n` su `gui/src/**`, su `gui/node_modules/{dockview,dockview-core,reka-ui}` e sul
  piano; `grep -c '^## Compito'` → 5 e le righe `| **N** |` della tabella → 9.

## Stato del repository alla fine

`git -C C:/Users/zagor/Desktop/harness status --porcelain` → **vuoto** (0 righe).

La cartella di prova resta in `C:\Users\zagor\AppData\Local\Temp\probe-ds3`, fuori dal repository, col suo `git` (i
commit `baseline`, `tasks 1-3 applied`, `task 4 applied`, `task5 …`) per chi vuole rifare una prova; si cancella a mano.



