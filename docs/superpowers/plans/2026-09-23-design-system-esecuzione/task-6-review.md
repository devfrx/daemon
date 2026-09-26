# La revisione del compito 6 — `c1102fc`, il dock vestito

Revisore del compito 6, macchina `Jays`, il 2026-09-26 dalle 18:23 alle 19:05 circa. Rivisto **un** commit, `c1102fc`
sopra `1a83208`. Le mutazioni nel clone `C:\Users\Jays\AppData\Local\Temp\rv6`, lasciato al suo posto per il
coordinatore: alla fine è a `c1102fc`, staccato, `git status --porcelain` vuoto — i cinque commit usa-e-getta della prova
di `compare_task6.py` restano solo nel suo reflog, e `gui/dist/` è ignorato. Log, script e immagini nello scratchpad
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d5484876-d48f-457d-8568-4ed6a1b75b37\scratchpad\review6\`
(qui sotto: `review6\`). Nell'albero del repository non ho scritto nulla fuori da questo file, che è ignorato
(`git check-ignore -v` → `.superpowers/sdd/.gitignore:1:*`): alla fine `git status --porcelain` è vuoto e
`## main...origin/main [ahead 1]`. Chrome `154.0.8037.58` dal nome della cartella, all'inizio e alla fine; Node v24.19.0.
Nessun `node.exe` lasciato acceso: il server di sviluppo fermato per PID (`taskkill //PID 40868 //T //F`, che ha chiuso
anche 37396 e 16152, quello in ascolto su 5174), poi `Get-Process node` → 0 e nessuno in ascolto su 5170–5179.

Una cosa per volta, misurata dai tempi dei log: le prove nel clone fino alle 18:31, il cancello sull'albero da solo dalle
18:33:24 alle 18:34:32, poi di nuovo il clone — le venti violazioni, le sonde, il server di sviluppo, fermato prima della
stabilità e delle ultime sonde.

## 1. Il verdetto

**Conforme.** Il commit è il dettato del piano byte per byte — `compare_task6.py`, provato nelle due direzioni, dà `OK` sui
sedici percorsi —; ogni Atteso dei Passi 2, 3, 4, 5 e 6 torna quando lo rilancio **sulla sequenza dei passi**; le venti
righe del Passo 7 sono rosse per la ragione scritta, coi messaggi e le prove che il rapporto dice; il cancello è verde; ogni
cifra del rapporto che ho rilanciato torna. I rilievi sono **uno Importante**, **due Minori** e **tre Nit**, nessun
Critico, e **tutti riguardano il dettato**: cinque voci d'errata candidate, **E50**–**E54**, e la conferma di **E49**,
la candidata dell'implementatore. Il più pesante è quello che il coordinatore aveva già misurato — `themeAbyss` in quattro
commenti dettati, contro il controllo 15 e la Definizione di «fatto» —; il più nuovo l'ho misurato io: il **secondo**
ciclo di `--dv-overlay-z-index`, quello di `.dv-render-overlay` che il Passo 1 misura e nessuna regola cura, alza a
**1000** il contenuto di un pannello `renderer: "always"` in un gruppo staccato, sopra i nostri dialoghi — oggi nessun
pannello lo usa.

## 2. I rilievi

| # | Classe | Dove | In una riga | Voce |
|---|---|---|---|---|
| I-1 | Importante | `gui/src/frame/dock.ts:80`, `gui/src/frame/frame.test.ts:195`, `gui/src/tokens/dock.css:3`, `gui/src/tokens/dock.test.ts:23` | quattro commenti dettati nominano `themeAbyss`: il controllo 15 del disegno (*«non compare più nel sorgente»*) è falso alla lettera, e la riga 15 del blocco 3 della Definizione di «fatto» renderà 4 dove attende 0 | **E50** |
| M-1 | Minore | `gui/src/tokens/dock.css:94-113` | la seconda ridefinizione su sé stessa, `.dv-render-overlay`, misurata al Passo 1 e non curata: un pannello `renderer: "always"` in un gruppo staccato si disegna a z-index **1000**, sopra `--z-overlay` | **E51** |
| M-2 | Minore | `gui/src/frame/dock.browser.test.ts:68-77`, `gui/src/tokens/dock.css:76-83` | *«draws every group as a card»* guarda raggio e fondo, non il bordo (scuro) né l'ombra (chiaro) che la (c) nomina: tolti, la suite resta verde | **E52** |
| N-1 | Nit | `task-6-report.md` §8 | **E49** dell'implementatore: confermata, le prove in più cadono davvero e per la ragione detta; il testo proposto è giusto e basta | E49 |
| N-2 | Nit | `gui/src/testing/probes.ts:27-29`, `:45-53` | la regola di ciò che scorre guarda le scatole **fra** elemento e antenato: un antenato tondo che scorre **lui** è giudicato lo stesso, e il commento non lo dice | **E53** |
| N-3 | Nit | `gui/src/frame/BigTabFace.vue:10-13`, `:21-22` | `@mousedown.stop` e `@click.stop` senza una prova, e il commento dice che `dockview` comincia il trascinamento anche su `mousedown`: `dockview-core` 8.3.1 non ascolta `mousedown` — ereditato da `BigTab.ts` di `1a83208` | **E54** |

### I-1 — Importante — `themeAbyss` in quattro commenti dettati — voce candidata E50

**Dove.** `grep -rn 'themeAbyss' gui/src`, a `c1102fc`:

```
gui/src/frame/dock.ts:80:    // ⛔ OUR THEME AND NOT `themeAbyss`, on which the eight moves were judged: that one is dark
gui/src/frame/frame.test.ts:195:    // ⛔ THE SHELL WEARS OUR CLASS, the one `tokens/dock.css` dresses -- and `themeAbyss`'s is gone (control 15).
gui/src/tokens/dock.css:3:   `usage.test.ts`). It sets every variable `themeAbyss` set, but the tab groups' and the palette's (`dock.test.ts`).
gui/src/tokens/dock.test.ts:23: * ⛔ OUR THEME REPLACES `themeAbyss`, SO IT SETS WHAT `themeAbyss` SETS (design system, section (c); P-6 of the plan).
```

`grep -rn 'themeAbyss' gui/src | wc -l` → **4**. Nel piano a `c1102fc` la riga della Definizione di «fatto»,
`grep -n "grep -rn 'themeAbyss' gui/src | wc -l" <piano>` → `9166:`, con l'attesa `# 0`; nel disegno la riga del
controllo, `grep -n '^| 15 |' docs/superpowers/specs/2026-09-22-design-system-design.md` → `465:`, *«`themeAbyss` non
compare più nel sorgente»*. Le quattro righe nascono dai recinti del compito 6: `grep -n "THE SHELL WEARS OUR CLASS\|OUR
THEME REPLACES\|It sets every variable \`themeAbyss\` set\|OUR THEME AND NOT" <piano>` → `5179:`, `5383:`, `5515:`,
`5716:`, e **nessun** recinto dopo il compito 6 le contiene — una cura non tocca i *Trova* dei compiti 7 e 8.

**Che cosa non è.** Nessun `import` e nessun uso: `themeAbyss` esce dal codice, e la prova del DOM lo tiene —
*«wears our theme…»* di `frame.test.ts` pretende il guscio con `dockview-theme-harness` e nessun `.dockview-theme-abyss`
(la riga 6 del Passo 7 e la mia corsa: rossa senza il `watch`, verde sul commit). Non è Critico per questo: il
comportamento che il controllo vuole c'è ed è provato; cade la sua traduzione in un `grep`, alla lettera, e cadrà al
compito 9, che la esegue.

**Le strade, col costo:**

| | Che cosa | Che cosa peggiora |
|---|---|---|
| **A** | riscrivere i quattro commenti senza l'identificatore, col senso di adesso — per esempio `dock.css:3` *«It sets every variable the abyss theme set, …»*; `dock.test.ts:23` *«⛔ OUR THEME REPLACES `dockview`'S ABYSS THEME, SO IT SETS WHAT THAT THEME SETS …»* (la riga dopo nomina già `.dockview-theme-abyss`); `frame.test.ts:195` *«… -- and the abyss theme's class is gone (control 15).»*; `dock.ts:80` *«⛔ OUR THEME AND NOT `dockview`'s abyss theme, on which …»*, riflowed se passa la larghezza del file. Nel codice e nei quattro recinti del piano, con la voce d'errata; `compare_task6.py` sul commit di cura, come per E38–E42 | quattro righe di commento in quattro file, e i recinti; chi cerca il nome dell'oggetto trova la classe. Nessuna prova cambia, la Definizione e il disegno restano come sono, e il `grep` della riga 15 resta una guardia vera: un `import` tornerebbe rosso |
| **B** | correggere la riga 15 della Definizione perché guardi il codice e non i commenti — `grep -rnE "import .*themeAbyss\|theme: *themeAbyss" gui/src \| wc -l # 0` — o mandarla alla prova del DOM | il controllo 15 del disegno resta falso alla lettera: serve un richiamo datato nel disegno, merito approvato (vincolo 1), quindi decisione del proprietario; e un filtro sui commenti è fragile — `dock.css:3` è la continuazione di un commento, e non comincia con `*` né con `//` |
| **C** | tenere tutto, e scrivere nella Definizione *«4, tutti commenti»* | un numero misurato scritto, che invecchia al primo commento (gotcha #31), e il controllo 15 resta falso alla lettera |

**Consiglio: A** — la più piccola che rimette vere tutte e tre le case (il disegno, il piano, il codice) senza toccare il
merito. La scelta è del coordinatore; se si prende la B, tocca il disegno ed è del proprietario.

### M-1 — Minore — `.dv-render-overlay`: il secondo ciclo di `--dv-overlay-z-index` — voce candidata E51

**Dove.** `dockview.css` 8.3.1, `grep -n -- '--dv-overlay-z-index: var' gui/node_modules/dockview/dist/styles/dockview.css`
→ `1037:` (`.dv-resize-container`) e `1203:` (`.dv-render-overlay`) — le due righe che il Passo 1 fa misurare. In
`dockview-core` 8.3.1, `grep -n 'dv-render-overlay' …/main.esm.mjs` → `14028: element.className = "dv-render-overlay";`,
`14144:` (la classe `dv-render-overlay-float` quando il pannello è staccato), e a `14184:`
`` focusContainer.style.zIndex = `calc(var(--dv-overlay-z-index, 999) + ${level * 2 + 1})` ``: il contenitore del
contenuto di un pannello `renderer: "always"` in un gruppo staccato prende il livello **da sé stesso**, dove la variabile è
ciclica e cade sul 999. In `dock.css` la cura sta sul solo contenitore, `grep -n 'DECLARED LIMIT' gui/src/tokens/dock.css`
→ `97:`, e il limite dichiarato parla dei soli gruppi.

**Misurato**, nel clone, con una prova usa-e-getta del progetto `browser` scritta per fallire (`review6\diag-overlay2.log`,
`diag-overlay3.log`), nei due temi: il dock della Home su 1400 × 800, poi `api.addPanel({ id: "review-always", component:
"knowledge", renderer: "always", floating: { x: 100, y: 100, width: 400, height: 300 } })`, e sopra la pagina una scatola
`position:fixed; inset:0; z-index: var(--z-overlay)` — il livello dei nostri dialoghi.

| | contenitore | `.dv-render-overlay` | `elementFromPoint` al centro del gruppo |
|---|---|---|---|
| il commit | `50` | `1000` (in linea `calc(var(--dv-overlay-z-index, 999) + 1)`) | `panel` — il contenuto **sopra** la scatola dei dialoghi |
| con la regola qui sotto | `50` | `51` | `review-veil` — la scatola sopra |

`dock.css` tornato dalla copia, `cmp` uguale; la prova tolta, `git status --porcelain` vuoto. **Oggi latente:**
`grep -rn '"always"' gui/src | wc -l` → 0, e nessuna vista lo usa. Un pannello che tiene vivo un contesto WebGL — gli asset
3D, *«arriva col sotto-progetto 7»* nel cassetto — è il candidato naturale a un `renderer: "always"`: dedotto, non letto in
nessun disegno.

**Le strade:**

| | Che cosa | Costo |
|---|---|---|
| **A** | in `dock.css`, dopo il blocco del contenitore: `/* ⛔ AND THE RENDER OVERLAY, THE SAME CYCLE: .dv-render-overlay { --dv-overlay-z-index: var(--dv-overlay-z-index, 999) } in dockview.css 8.3.1, and dockview-core lifts the content of a panel with renderer "always" in a floating group to that level + 2 * level + 1 -- measured at 1000, over our dialogs, with its container at 50. */ .dockview-theme-harness .dv-render-overlay { --dv-overlay-z-index: var(--z-floating); }`; in `dock.browser.test.ts`, nella prova del livello o in una sua, il pannello `renderer: "always"` staccato e le due attese — il livello del suo contenuto sotto `--z-overlay`, e `--z-floating` + 1 —; una riga nel Passo 7 (la regola tolta → `1000`) | cinque righe di CSS, una quindicina di prova, una riga di tabella; la prova porta un pannello che la SPA oggi non ha |
| **B** | registrarla: una frase nel limite dichiarato di `dock.css` e una riga in *«Le voci aperte che questo piano SA»*, chiusore il primo sotto-progetto con un pannello `renderer: "always"` | debito dichiarato: chi arriva deve ritrovare la regola, e il difetto è della stessa specie che il compito cura (R3-17, trappola 9) |

**Consiglio: A**: la cura è quella che il compito già applica al contenitore, misurata qui sopra nelle due direzioni, e la
prova non aspetta un consumatore perché il pannello lo aggiunge lei.

### M-2 — Minore — la scheda senza il suo bordo né la sua ombra — voce candidata E52

**Dove.** `grep -n 'draws every group as a card' gui/src/frame/dock.browser.test.ts` → `68:`; la prova guarda, per
ogni gruppo, `borderTopLeftRadius` e `backgroundColor` (righe 75-76) e le distanze. `grep -n 'The card: the radius'
gui/src/tokens/dock.css` → `76:`, il commento del perché: *«the border the dark theme draws and the light one leaves
transparent, the shadow the light one casts»*. La (c): *«ogni gruppo è una scheda — `--color-bg-surface`,
`--radius-card`, nello scuro il bordo `--color-border-card` e nel chiaro l'ombra `--shadow-card`»*.

**Misurato**, nel clone, una mutazione per volta con la copia salvata (`review6\violations.py`, `viol\viol-X10.log`,
`viol-X11.log`): tolta la riga `border: …` → `Tests  177 passed | 1 skipped (178)`; tolta `box-shadow: …` → lo stesso.
Nel chiaro, senza l'ombra, la scheda si distingue dal fondo per un soffio: è ciò che la rende una scheda.

**Il testo proposto**, provato nel clone (`review6\e52.py`): nella prova, dopo l'attesa del fondo,

```ts
        expect(style.borderTopColor).toBe(computed("border-top-color", "--color-border-card"));
        expect(style.boxShadow).toBe(computed("box-shadow", "--shadow-card"));
```

verde sul codice, `Tests  8 passed (8)`; senza il bordo rossa **nei due temi**, `expected 'rgb(27, 23, 24)' to be
'rgba(0, 0, 0, 0)'` e `expected 'rgb(236, 230, 218)' to be 'rgb(47, 40, 41)'` — tolto il bordo, il colore torna
`currentColor`, e morde anche nel chiaro —; senza l'ombra rossa **nel solo chiaro**, `expected 'none' to be 'rgba(29, 23,
24, 0.06) 0px 1px 2px 0p…'` — nello scuro `--shadow-card` è `none`, e quella metà non può mordere, come E47. Due righe nel
Passo 7. I file tornati dalle copie, `cmp` uguali, `git status --porcelain` vuoto. Il *Trova* del compito 8 su questa prova
finisce sulla riga del raggio, prima di queste due: non lo tocca. L'alternativa è rinominare la prova su ciò che guarda.

⚠️ **Dello stesso genere, e lasciati al Passo 8 com'è scritto** (*«le schede, le linguette col segno … il gruppo
staccato»*): tolte le due regole del segno della linguetta (`.dockview-theme-harness .dv-tab .base-label` e quella
dell'icona della linguetta inattiva) o quella del gruppo dentro il contenitore galleggiante, la suite resta verde
(`viol-X2.log`, `viol-X3.log`, `viol-X5.log`, `177 passed`). Guardati, sono giusti: le cinque linguette della Home hanno
l'etichetta `rgb(27, 23, 24)` e l'icona `rgb(122, 31, 46)` nel chiaro, `rgb(236, 230, 218)` e `rgb(191, 85, 103)` nello
scuro (`look\look-result.json`); una linguetta inattiva nella Home non c'è, perché ogni gruppo ha un pannello solo. Non li
conto come rilievo: il piano li affida all'occhio del proprietario (controllo 15), come D10 per il bordo della zona d'arrivo.

### N-1 — Nit — E49 dell'implementatore: confermata

Rilanciate le due righe (`viol\viol-11.log`, `viol-19.log`), coi due progetti interi:

- **(a)** *«la distanza»*, `gap: 0 };` → `Tests  3 failed | 174 passed | 1 skipped (178)`: le due del browser,
  `expected [ Array(10) ] to deeply equal []`, **e** sotto jsdom *«refuses a gap that is not a length in px…»*,
  `expected [Function] to throw an error`. La ragione è quella del rapporto: il `gap` scritto a mano non passa da
  `pixels`, e con `--space-3` a `0.75rem` `harnessTheme()` non lancia più.
- **(b)** *«la riduzione»*, `return radius(element, corner);` → `Tests  6 failed | 171 passed | 1 skipped (178)`: la
  pillola, ciò che scorre, il dock nei due temi, **e** *«keeps every radius concentric (answer 4)»* della pagina kit nei due
  temi, a `kit.browser.test.ts:72`, `expected [ …(24) ] to deeply equal []` — i radi letti `9999.0` in `kit-card` e nel
  puntino, i `base-button` a pillola in `kit-strip`.

Il testo proposto per i due Atteso è giusto e basta. La frase di E46 *«sulla pagina kit … nessun esito cambia»* resta
vera nel suo senso — la cura contro la riduzione di prima: coi quattro angoli uguali `r · min(1, w/2r, h/2r)` è
`min(r, w/2, h/2)` —, e la mia sonda lo conferma (qui sotto, *«Il resto del punto 7»*, (f)).

### N-2 — Nit — la regola di ciò che scorre e l'antenato che scorre lui — voce candidata E53

**Dove.** `grep -n 'WHAT SCROLLS IS NOT PLACED' gui/src/testing/probes.ts` → `27:`; `scrolled` alle righe 45-53 cammina
da `element.parentElement` e si ferma **prima** di `ancestor`.

**Misurato**, con la prova usa-e-getta (`review6\diag-scroll.log`): una scatola tonda di 20 px, `overflow:auto`, con 300 px
di contenuto, e dentro un pezzo 60 × 30 a pillola nel suo angolo in basso a sinistra → `{ scrolls: true, probe: { near: 1,
bad: ["piece in div, bottom-left: radius 15.0, outer 20.0, distance 0.0/0.0"] } }`: giudicato, benché stia dove lo
scorrimento lo lascia — il caso di E44 con la scatola che scorre spostata sull'antenato. **Oggi latente**: nelle radici che
le prove giudicano non c'è un antenato tondo che scorre — il foglio di `BaseDialog.vue` scorre, ma ha gli angoli di sotto
dritti e quelli di sopra, a scorrimento zero, sono disegnati dal suo `padding`.

**Il testo proposto**, nel commento della sonda: *«⚠️ Only a box BETWEEN the two: an ancestor that scrolls ITSELF is
still judged, at the scroll it has -- at 0, what its padding puts in its top corners is drawn, and a sheet's `r r 0 0` is
judged there.»* E una riga per il pre-controllo del compito 8, accanto alla domanda di E44 sulla Panoramica.

### N-3 — Nit — `@mousedown.stop` e `@click.stop` della presa — voce candidata E54

**Dove.** `grep -n 'A PRESS ON A COMMAND' gui/src/frame/BigTabFace.vue` → `10:`: *«`dockview` begins the drag on
`pointerdown`/`mousedown`, so both stop on the button»*.

**Misurato:** tolto `@mousedown.stop` dal primo pulsante → `Tests  177 passed | 1 skipped (178)` (`viol\viol-X1.log`);
`@click.stop` → `@click`, lo stesso (`viol-X8.log`). E `grep -c '"mousedown"'
gui/node_modules/dockview-core/dist/package/main.esm.mjs` → **0**: la linguetta ascolta `pointerdown` (riga 5940),
`click`, `contextmenu` e la pressione lunga, e la mossa `pointer` di `dock.ts` comincia su `pointerdown` — la metà provata
dalla riga 4 del Passo 7. **Ereditato:** la stessa frase e lo stesso ascoltatore senza prova stavano in `BigTab.ts` a
`1a83208` (`git show 1a83208:gui/src/frame/BigTab.ts`).

**Il testo proposto**, nel commento: *«… `dockview` 8.3.1 begins its pointer drag on `pointerdown`, stopped here and held
by `bigtab.test.ts`; `mousedown` and `click` are stopped too, as SP-8 measured them, and no test holds those two.»* —
dichiarare, non togliere: la mossa 5 di SP-8 è stata giudicata con loro.

### Il resto del punto 7 — ciò che ho guardato e che non è un rilievo

- **(a) Le righe col loro perché, tolte** — oltre alle venti del Passo 7, undici mie nel clone, una per volta con la copia
  salvata (`review6\violations.py`, `viol\viol-X*.log`): cadono le prove giuste per `box-sizing: border-box` (le distanze,
  `expected [ Array(10) ] to deeply equal []`), per `.bigtab { height: 100%; }` (la presa, `expected [ 24, 24, 24, 24, 24 ]
  to deeply equal []`) e per `--dv-spacing-padding: 0` (P-6, `expected [ '--dv-spacing-padding' ] to deeply equal []`);
  restano verdi, `177 passed`, le righe di M-2 e N-3, e il margine di `.dock` in `Frame.vue` — la sua sonda è del compito 8,
  *«floats the strip as a pill 12 px from the sides and 24 from the bottom»*, con la sua riga nel Passo 7 di quel compito.
- **(b) I commenti che dicono «every», «all», «only»** su ciò che il compito fa crescere: veri a `c1102fc` — *«It sets every
  variable … but the tab groups' and the palette's»* lo tiene P-6; *«Every group is a CARD»*; il limite dichiarato, 50 + 2(n −
  1) sotto 100 fino a 25 e sotto 200 fino a 75, coi valori di `base.css` (`grep -n -- '--z-' gui/src/tokens/base.css` →
  50, 100, 200); il commento di `eslint.config.js`, *«built its two commands so until task 6, and mounts the kit's pieces
  since»*, vero. Il commento di `copy.test.ts:24` su `` `modules.${parameters.api.id}` in `BigTab.ts` `` resta vero
  (`BigTab.ts:39`).
- **(c) Le interfacce** della lista *Produces*: `readToken(name, element = document.documentElement)` con l'errore — la riga 9
  del Passo 7; `harnessTheme()` esportata, `{ name: "harness", className: "dockview-theme-harness", colorScheme, gap }`; la
  classe sul guscio (la prova del DOM); `BigTabFace.vue` con `defineProps<{ title: string; icon?: IconName }>()` e
  `defineEmits<{ float: []; page: [] }>()`, montata come `VueContent` (`VueContent.ts:28-42`: stessa forma, `i18n` sì e
  Pinia no, `unmount` in `dispose`); `base.css` sotto jsdom — la riga 8 del Passo 7, cinque prove; `concentricRadii` — le righe 18-20. Nessun
  pulsante nato con `document.createElement` in `panels/` e `frame/`.
- **(d) Nessun commento che dica il falso** dopo il commit, fuori da I-1 e N-3: il censimento `grep -rniE
  "bridge|ponte|bigtab-title|until task 6|task 6|compito 6|\.bigtab button|glyph|⧉|⤢" gui` (fuori da `node_modules` e
  `dist`) rende, oltre al trasporto (`Bridge`, `fakeBridge`) e a `VueContent.ts` (*«the bridge between Vue and…»*, un'altra
  cosa), cinque righe col compito 6 **al passato** o come attribuzione — `eslint.config.js:155`, `jsdom-setup.ts:19`,
  `probes.browser.test.ts:7`, `dock.css:1`, `theme.ts:23` (*«the dock's `colorScheme` from task 6»*, ora vero: `dock.ts` lo
  legge) —, e il ponte (E36) non c'è più. Lette intere.
- **(e) I vincoli 4, 5 e 6**: nella tabella dei comandi — nessuna scritta fuori da `it.json`, nessun colore a mano, nessuna
  scala fuori da `tokens/`.
- **(f) La sonda corretta, per chi la usava prima.** Le prove della pagina kit verdi in ogni corsa intera (il cancello, il
  Passo 6, la stabilità). Con una sonda usa-e-getta (`review6\diag-scroll.log`) che conta le coppie vicine con la regola
  dello scorrimento e senza: sulla pagina kit **12 e 12** nei due temi, **nulla tolto**; sul dock della Home **4 contro 6**,
  tolti i due radi di Impostazioni che scorrono (`radio in dv-groupview bl 12.0/20.0 @13.0/19.0` e `@13.0/-9.0`, il secondo
  già sotto il bordo), e le quattro che restano sono il puntino dentro un radio — nessuna del dock, com'è scritto in E44;
  col gruppo staccato **8 e 8**, nulla tolto, e nel contenitore **4**, esattamente la guardia `>= 4`. E46 coi quattro angoli
  uguali dà lo stesso numero di prima, quindi sulla pagina kit, dove ogni pezzo li ha uguali, nulla cambia.

## 3. I comandi rilanciati

| Che cosa | Il comando | La mia uscita | Il rapporto |
|---|---|---|---|
| avvio | `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline 1a83208..HEAD` | `c1102fc`; vuoto; una riga | — |
| i log del cancello dell'implementatore | `ls -la --time-style=full-iso` sui due log, contro `git log -1 --format=%ci c1102fc` | `gate-apertura-…` 18:03:40, `gate-chiusura-…` 18:15:50; il commit 18:17:15 | suoi, prima del commit; diversi da `gate-baseline.log` (17:48:19) |
| Passo 1 | `git show 1a83208:gui/src/frame/dock.ts \| grep -n themeAbyss`; i due `grep` di `overlay-z-index` | `1:` e `56:`; `dockview.css` `1037:` e `1203:`; `main.esm.mjs` `12023:` dentro `AriaLevelTracker` (`12008:`) | uguali |
| Passo 1, il pezzo | `grep -nE 'assets/index-.*\.js ' gate-baseline.log` | `index-COUloyjC.js 689.65 kB │ gzip: 210.02 kB` | uguale |
| la ricetta | `review6\recipe_fences.py 1a83208`: per ogni riga `W`/`R` il recinto, la sua etichetta, la prima e l'ultima riga | ventidue righe, ciascuna sul recinto giusto; fuori dalla ricetta solo recinti `bash` e i due `js` del Passo 8 | — |
| `compare_task6.py` | `PYTHONIOENCODING=utf-8 python …/compare_task6.py 1a83208 c1102fc` | sedici `OK`, `0 not matching`, `date 2026-09-26`, uscita 0 | uguale |
| le due direzioni dello script | nel clone, un commit per mutazione (`review6\cmpmut\run.sh`, `run.log`) | un carattere in un commento di `dock.css` (`W`) → `DIFFERS` sul solo `dock.css`, uscita 1; in `dock.ts` dentro una sostituzione (`R`) → `DIFFERS` sul solo `dock.ts`, 1; la colonna Commit della riga 5 → `DIFFERS` sul piano, 1; una parola dell'errata → `CHECK BY HAND`, 0; `gui/package.json` → `UNEXPECTED`, 1 | — |
| Passo 2 | nel clone, `readToken.ts` tolto, `npx vitest run --project browser src/tokens/tokens.browser.test.ts` | `Error: Failed to import test file …/tokens.browser.test.ts` (`Failed to resolve import "./readToken"`); tornato, `cmp` uguale, `Tests  6 passed (6)` | uguale |
| Passo 3 | lo stato del Passo 3 (le quattro prove e i due file del Passo 2 del commit, gli otto file dei Passi 4 e 5 da `1a83208`, `BigTabFace.vue` tolto), i due comandi | jsdom `Tests  3 failed \| 12 passed (15)`: `expected [ …(43) ] to deeply equal []`, `expected false to be true`, `… but got '(0 , __vite_ssr_import_13__.harnessTh…'`; browser `Tests  10 failed \| 5 passed (15)`: nei due temi schede e contenitore `'0px' to be '20px'`, presa `[ 27, 27, 27, 27, 27 ]`, livello `999 to be 50`; nella sonda la barra e ciò che scorre; **verde la pillola**; nove file tornati, `cmp` uguali | uguale |
| Passo 4, sulla sequenza | commit meno il Passo 5 (`BigTab.ts`, `bigtab.test.ts`, `eslint.config.js` da `1a83208`, `BigTabFace.vue` tolto) | jsdom `Test Files  8 passed (8)`, `Tests  37 passed (37)`; browser `3 passed (3)`, `21 passed (21)` — sul commit intero jsdom dà 38: `bigtab.test.ts` passa da due prove a tre | uguale |
| Passo 5 | `BigTab.ts` da `1a83208`, `BigTabFace.vue` tolto | `Tests  3 failed (3)`: `expected undefined to be 'Stato'`, `expected [ undefined, undefined ] to deeply equal [ 'float', 'fullPage' ]`, `TypeError: tab.dispose is not a function`; tornati, `cmp` uguali | uguale |
| Passo 6 | nel clone `npm test`, `npm run build`, `npm run lint` | `Test Files  24 passed \| 1 skipped (25)`, `Tests  177 passed \| 1 skipped (178)`; `index-DX-fK1-d.js 690.57 kB │ gzip: 210.40 kB`; lint uscita 0 | uguale |
| Passo 7 | `review6\violations.py 1..20`, una per volta, `npx vitest run --reporter=verbose`, la copia indietro | le venti righe rosse col messaggio e le prove della tabella del rapporto (`viol\viol-1.log`…`viol-20.log`, riassunto `review6\summarize.py`); le due in più di E49 (qui sopra); venti `cmp` uguali, `git status --porcelain` vuoto alla fine | uguale |
| stabilità | nel clone, 10 volte `npx vitest run --project browser src/frame/dock.browser.test.ts src/testing/probes.browser.test.ts`, 5 volte `npx vitest run` | 10 su 10 `Tests  15 passed (15)`; 5 su 5 `Tests  177 passed \| 1 skipped (178)`; e in 31 corse intere mutate nessuna prova è caduta fuori da quelle della mutazione | uguale |
| il cancello | `bash scripts/gate.sh`, da solo, in background (`review6\gate-review6-20260926-183324.log`) | `GATE GREEN.`; jsdom `Test Files  19 passed \| 1 skipped (20)`, `Tests  137 passed \| 1 skipped (138)`; browser `5 passed (5)`, `40 passed (40)`; `index-DX-fK1-d.js 690.57 kB │ gzip: 210.40 kB`; `found 0 vulnerabilities` | uguale |
| contro la base | `gate-baseline.log` del coordinatore | jsdom 18 + 1 file e 132 + 1 prove, browser 4 file e 28 prove, 689.65 kB: il pezzo JavaScript **cresce di 0,92 kB**, compresso di 0,38 — la cifra al proprietario (N-2) | — |
| nessuna prova sparita | `git diff --name-status --diff-filter=D 1a83208 c1102fc` → nulla; i titoli `it("…")` dei quattro file di prova toccati, base contro commit | nessun file tolto; nessun titolo tolto fuori da `bigtab.test.ts`, riscritto per intero dal dettato (due prove rinominate e allargate, una nuova); il conto torna: jsdom +5 (2 + 2 + 1), browser +12 (8 + 1 + 3) | — |
| `check-docs` | `bash scripts/check-docs.sh` | `OK — no inconsistencies.`, uscita 0 | uguale |
| fine-riga | `git ls-files --eol` e `tr -cd '\r' \| wc -c` contro `wc -l`, sui sedici file, e sui blob di `1a83208` | tutti `i/lf w/lf`, zero CR prima e dopo; i quattro nuovi LF; le righe come nella tabella del rapporto | uguale |
| vincolo 7 | `git diff --stat 1a83208..c1102fc -- gui/package.json gui/package-lock.json` | vuoto | uguale |
| vincolo 8 | `grep -E '"(reka-ui\|vitest\|…)"' gui/package.json` | `reka-ui` 2.10.4, `vitest` 4.1.11, `@vitest/browser-playwright` 4.1.11, `playwright` 1.63.0, `dockview`/`dockview-core` 8.3.1 | — |
| vincolo 12 | `git diff --stat 1a83208..c1102fc -- crates/ gui/schema/` | vuoto | uguale |
| vincoli 4, 5, 6 | `grep` dei colori a mano su `dock.css`, `BigTabFace.vue`, `Frame.vue`; `grep -rln 'var(--ref-' gui/src \| grep -v '^gui/src/tokens/' \| wc -l`; il template di `BigTabFace.vue` | nessun colore; `0`; nessuna scritta, solo `$t('menu.float')`, `$t('menu.page')` e `{{ title }}`; `usage.test.ts` legge `tokens/dock.css` (`grep -n 'dock.css' gui/src/tokens/usage.test.ts` → `32:`, `44:`) |
| D11, il linter legge la faccia | nel clone, in `BigTabFace.vue` un `<button type="button"></button>` e `<span>testo</span>`, `npx eslint src/frame/BigTabFace.vue` | `a button is BaseButton (design system, section (b))` e `raw text 'testo' is used`; tornato, `cmp` uguale, uscita 0 | — |
| trappola 5 | `grep -rn 'createElement' gui/src/panels gui/src/frame` | solo `div` e un `input` (nelle prove), il `div` della linguetta in `BigTab.ts:30` e quello di `VueContent.ts:21`: nessun pulsante | — |
| controllo 20 | per ogni `*.browser.test.ts`, `grep -cE 'NON-VACUITY\|toBeGreaterThan\(0'` | `dock.browser.test.ts 4`, `kit 13`, `settings 1`, `probes 1`, `tokens 4` | — |
| il contratto | `git rev-list --count 1a83208..c1102fc`; `git status -sb`; `git log -1 --format=%B c1102fc \| grep -ci co-authored`; `git log -1 --format=%s` | `1`; `[ahead 1]`, `origin/main` = `1a83208`; `0`; comincia con `design-system(compito 6): `, e il corpo chiude con le due righe del pezzo, 689.65 e 690.57 kB | uguale |
| la riga 5 del piano | `git log -1 --format='%cs %s'` di `545f500`, `7e25d03`, `9e6657b` | i tre commit del compito 5, del 2026-09-26 | — |
| `updateTheme` non disegna `colorScheme` (R3-21) | `sed -n` su `updateTheme() {` di `main.esm.mjs` (riga 18355) | legge `className`, `gap`, `edgeGroupCollapsedSize`, `dndOverlayBorder`, `dndOverlayMounting`, `tabGroupIndicator`: il commento di `dock.ts` è vero | — |

## 4. Che cosa ho visto, tema per tema

Il server di sviluppo del clone (`npm run dev -- --port 5174 --strictPort`), e uno script Playwright dello scratchpad
(`review6\look\look.cjs`, `drag.cjs`), `playwright` preso dal `node_modules` del clone con `createRequire`,
`chromium.launch({ channel: "chrome", headless: true, ignoreDefaultArgs: ["--hide-scrollbars"] })`, 1440 × 900 a densità
2, `harnessFake.deliverAll()`, poi `document.documentElement.dataset.theme`. I due frammenti del Passo 8 sono i recinti
del piano, confrontati con `diff` (`look\contrast.js`, `look\level.js`). I numeri in `look\look-result.json` e
`look\drag.log`; le immagini in `review6\look\`. ⛔ **L'aspetto non lo giudico io**: descrivo, e dico ciò che è rotto —
niente, qui.

**Le due misure del Passo 8**

| | Chiaro | Scuro | Atteso |
|---|---|---|---|
| il contrasto | `seen` **33**, il peggiore **6,22** (*«Degrado»*, *«Policy VRAM»*, *«Giornale»*) | `seen` **33**, **6,41** (le stesse) | sopra 4,5; 6,22 e 6,41 su 33 |
| il livello, «Stacca la tessera» su Stato, poi «+ moduli» | `base-dialog-veil` | `base-dialog-veil` | `base-dialog-veil` |

E, a lato: le cinque linguette visibili (Stato, Permessi, Impostazioni, Attività, Costi) alte **40**, e la presa 40; il
gruppo staccato a `z-index` **50**, raggio **20px**, barra, intestazione, linguetta e gruppo `rgb(251, 248, 243)` nel
chiaro e `rgb(36, 31, 32)` nello scuro (E47); nessun testo tagliato fra ciò che non scorre, e la pagina senza sbordo,
`[1440, 1440, 900, 900]`. Nella console un solo errore, `Failed to load resource: … 404` — come nel rapporto.

**Chiaro**

- `light-01-home.png` — la Home: schede chiare sul fondo appena più scuro, distinte quasi solo dall'ombra morbida; 12 px
  fra loro; la striscia in fondo, a 12 px dai lati e 24 dal fondo; in alto il dock tocca la fascia, com'è la risposta 20 (*«the
  bar above has none»*). Le linguette: icona bordeaux, etichetta in maiuscoletto piena, i due comandi discreti accanto.
  Stato e Impostazioni scorrono, e la loro ultima riga è tagliata dal bordo della scheda: è lo scorrimento.
- `light-02-grab.png`, `light-03-grab-focus.png` — la presa di Stato; col fuoco dalla tastiera sul primo comando l'anello
  bordeaux di 2 px a 2 px dal pulsante (`:focus-visible` vero), e intorno all'intera linguetta un riquadro di 1 px
  `rgb(220, 210, 196)` che `dockview` disegna sul `::after` della linguetta (`light-10-focus-corner.png`: l'arco della
  scheda lo taglia, non sborda).
- `light-04-divider-hover.png` — il divisorio compare solo sotto il puntatore, una barra di 4 px `rgb(132, 122, 115)` nel
  mezzo dei 12 px.
- `light-05-corner-Stato.png`, `light-05-corner-Impostazioni.png` — le barre di scorrimento accese: il binario, più
  chiaro della scheda, arriva all'angolo tondo e l'arco ne taglia il fondo; la freccia resta intera (E48).
- `light-06-floating.png`, `light-07-floating-top.png` — il gruppo staccato: scheda con l'ombra della sovrapposizione,
  raggio 20 in alto, la barra del titolo vuota con la sua riga sotto, la linguetta sullo stesso fondo dell'intestazione.
- `light-08-drawer-over-floating.png` — il cassetto: il velo copre anche il gruppo staccato.
- `light-11-drop-zone.png` — trascinando «Permessi» su Attività: la zona d'arrivo `rgb(239, 225, 221)` col bordo di 1 px
  `rgb(122, 31, 46)`, un rettangolo dagli angoli dritti dentro la scheda tonda, il cui arco ne taglia il bordo in basso.
  (`light-09-drop-zone.png`, il primo tentativo sul gruppo della chat, non mostra la zona: quel gruppo è `locked` nella Home,
  `grep -o '"locked":[^,}]*' gui/src/panels/views/home.json` → due volte `true` — dalla parte 2, non dal compito.)

**Scuro**

- `dark-01-home.png` — le schede col bordo `--color-border-card` e senza ombra; icone del segno `rgb(191, 85, 103)`,
  etichette `rgb(236, 230, 218)`; barre di scorrimento scure (`color-scheme` dal tema, `themes.css:20`).
- `dark-02-grab.png`, `dark-03-grab-focus.png`, `dark-10-focus-corner.png` — come nel chiaro, l'anello `rgb(191, 85, 103)`,
  il riquadro di `dockview` `rgb(47, 40, 41)`.
- `dark-04-divider-hover.png` — il divisorio `rgb(122, 112, 106)`.
- `dark-05-corner-Stato.png`, `dark-05-corner-Impostazioni.png` — il binario scuro tagliato dall'arco, la freccia intera, il
  bordo della scheda che segue l'arco.
- `dark-06-floating.png`, `dark-07-floating-top.png` — il gruppo staccato più chiaro delle schede, e **nessun** rettangolo
  più scuro dietro «STATO»: E47 curata.
- `dark-08-drawer-over-floating.png` — il velo sopra il gruppo staccato.
- `dark-11-drop-zone.png` — la zona `rgb(47, 27, 29)` col bordo `rgb(191, 85, 103)`.

Fuori dal compito 6, per il pre-controllo del compito 8 che riscrive il cassetto: nel foglio del cassetto il titolo
*«I MODULI»* comincia a 13 px dal bordo e le righe a 25 (`light-08`, `dark-08`) — da giudicare con la tavola, non misurato
oltre.

## 5. Che cosa non ho potuto verificare, e perché

- **L'aspetto**: del proprietario (controllo 15, D24). Le immagini sono qui sopra.
- **Il lampo della fascia (E43), fotogramma per fotogramma, alle quattro misure del rapporto**: non l'ho rifatto — non è
  nel Passo 8 del compito 6, e la sonda è del compito 8. Ho guardato lo stato fermo: nessuno sbordo della pagina a
  1440 × 900.
- **Il `padding` `0px 8px` delle prese**: non l'ho misurato; ho misurato le altezze, 40 e 40. Dedotto: il `padding` verticale
  lo toglie la regola di E45, l'orizzontale è il `0.5rem` di `dockview.css`.
- **Gli stati intermedi dell'implementatore** (`prima-violazioni.txt`, i quindici file prima del commit): passati; al loro
  posto ho rifatto la sequenza dei passi nel clone.
- **Gli sfondi a comparsa di `dockview`** — il menu del trabocco delle linguette e quello contestuale: `openPopover` li mette
  a `var(--dv-overlay-z-index)`, cioè `--z-floating` da un gruppo agganciato — sotto qualunque gruppo staccato che li copra —
  e al doppio del livello del gruppo da uno staccato (`popoverZIndexFor`, righe 13636 e 9697 di `main.esm.mjs`). Dedotto dal
  sorgente, **non misurato**: nella Home nessun gruppo ha più linguette, e nessun menu contestuale è configurato. Non è una
  regressione: con `themeAbyss` era 999 contro 999 + 2i, lo stesso ordine; e ora stanno sotto i nostri dialoghi, prima sopra.
- **La CI**: del coordinatore, dopo il push.
- **Il lettore di schermo**: fuori dal compito; i tre difetti di `axe` sul dock restano del proprietario (P-18).
