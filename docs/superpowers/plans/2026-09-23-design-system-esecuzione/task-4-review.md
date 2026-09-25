# La revisione del compito 4 — `841dc54`, la pagina kit fuori dal pacchetto e le sonde nel browser

Revisore del compito 4, macchina `Jays`, il 2026-09-26 dalle 00:20 alle 01:00 circa. Rivisto **un** commit, `841dc54`
sopra `1b531e9`. Le mutazioni nel clone `C:\Users\Jays\AppData\Local\Temp\rv4`, lasciato al suo posto per il
coordinatore: alla fine è a `841dc54`, staccato, `git status --porcelain` vuoto. I log, gli script e le immagini nello
scratchpad `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46\scratchpad\review4\`
(qui sotto: `review4\`). Nell'albero del repository non ho scritto nulla: alla fine `git status --porcelain` è vuoto,
`## main...origin/main [ahead 1]`. Chrome `154.0.8037.58`, dal nome della cartella, all'inizio e alla fine.

## 1. Il verdetto

✅ **Conforme**: il commit è il dettato del piano byte per byte, e ogni Atteso e ogni cifra del rapporto tornano quando
li rilancio. I sette rilievi sono tre **Minore** e quattro **Nit**, nessuno **Critico** né **Importante**. Tutti tranne
uno sono difetti del **dettato**, quindi voci d'errata candidate, **E29**–**E33**, più la conferma di **E28**; il
restante riguarda lo script del pre-controllo.

## 2. I rilievi

| # | Classe | Dove | In una riga | Voce |
|---|---|---|---|---|
| M-1 | Minore | `gui/src/kit/Kit.vue:194`, `gui/src/kit/kit.browser.test.ts:67-72` | la prova dei raggi non tiene l'unico caso che giudica `BaseList`, e la riga della griglia che lo crea non la tiene nessuna prova | **E29** |
| M-2 | Minore | `gui/src/testing/probes.ts:19`, `:23` | `concentricRadii` legge solo l'angolo in alto a sinistra, e il suo commento promette che un angolo dritto non si confronta mai | **E30** |
| M-3 | Minore | `scripts/gate-gui.sh:36-40` | la guardia di non-vacuità si accorge di un `dist/` che manca, non di un `dist/` rimasto da un *build* precedente | **E31** |
| N-1 | Nit | `gui/src/testing/probes.ts:107` | `iconsCentred` guarda l'attributo `stroke`, non il colore disegnato | **E32** |
| N-2 | Nit | `gui/src/testing/probes.ts:52`, `kit.browser.test.ts:67-72` | il limite di **E27** non è scritto nel codice | **E33** |
| N-3 | Nit, fuori dal commit | `…-esecuzione/compare_task4.py:64-68`, `:143-144`, `:39-41` | lo script non vede la data del dispaccio né il modo dei file, e non lo dichiara | — |
| N-4 | Nit | `gui/eslint.config.js:112-123` | confermo **E28**, la candidata dell'implementatore | **E28** |

### M-1 — Minore — la prova dei raggi non tiene il suo caso di `BaseList` — voce candidata E29

**Dove.** `grep -n "align-items: start\|NOT STRETCHED" gui/src/kit/Kit.vue` → `192:` il commento *«⛔ NOT STRETCHED: a card
stretched to the tallest of its row moves what closes it away from its corner, and the radius probe would judge nothing
there (R3-1).»*, `194:  align-items: start;`; `grep -n "keeps every radius concentric\|expect(report.near)"
gui/src/kit/kit.browser.test.ts` → `67:`, `71:`. Nel piano: il recinto del Passo 3, riga 3631, e quello del Passo 2, riga
3310.

**Evidenza.** La riga ha il suo perché nel commento, e se la togli non cade nessuna prova (lezione (c)). Anzi: senza di
lei la riga 2 del Passo 7, che il piano vuole rossa, diventa verde.

| Nel clone, una cosa per volta, indietro con la copia salvata | `npx vitest run --project browser` |
|---|---|
| il codice del commit | `Tests  23 passed (23)` |
| riga 2 del Passo 7, `BaseList.vue` a `var(--radius-card)` | `2 failed \| 21 passed (23)`, `base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 13.0/13.0` (`review4\logs\02.log`) |
| tolta `align-items: start;` col suo commento | `Tests  23 passed (23)` (`logs\C1.log`) |
| tolta la riga **e** riga 2 del Passo 7 | **`Tests  23 passed (23)`**: la violazione passa (`logs\C1v2.log`) |

Il conto l'ho misurato con una prova usa-e-getta scritta per fallire, `review4\near-count.browser.test.ts`, messa in
`src/kit/` del clone, lanciata da sola e tolta dopo. Col codice del commit `near: 12`, e l'ultima riga della lista sta a
`13.0` px dal fondo della sua scheda (`p8-near-committed.log`). Senza la riga `near: 10`, e l'ultima riga sta a `105.0` px
(`p8-near-stretched.log`). Le due coppie `base-list-row in kit-card` sono le sole che giudicano il raggio di `BaseList`, e
la guardia `near > 0` non vede che sono sparite: conta i casi di tutta la pagina. È la lezione 3 del pre-controllo, qui su
un **caso** invece che su un ramo. Succede lo stesso con una nota dopo la lista, che è la forma di tutte le altre schede
(misurato qui sotto, con la cura).

**Cura proposta (E29).** Nel recinto del Passo 2, nella prova *«keeps every radius concentric (answer 4)»*, dopo
`expect(report.bad).toEqual([]);`:

```ts
      // ⛔ AND THE ONE CASE THAT JUDGES `BaseList` (R3-1): its last row closes its card, in the corner. `near` counts the
      // whole page, so a stretched grid, or a note after the list, would take this case away and stay above zero.
      const list = document.querySelector(".kit-card:has(> .base-list)");
      expect(list).not.toBeNull();
      expect(concentricRadii([list as Element]).near).toBeGreaterThan(0);
```

Poi una riga nella tabella del Passo 7: *in `Kit.vue` tolta `align-items: start;` di `.kit-grid` → rosso: la prova dei
raggi, nei due temi, `expected 0 to be greater than 0`*. L'ho misurata nel clone (`logs\F1cure.log`,
`logs\F1cure+C1.log`, `logs\F1cure+note.log`):

- verde sul codice del commit, `Tests  23 passed (23)`;
- rossa con la riga tolta, `2 failed | 21 passed (23)`, le sole due prove dei raggi, `AssertionError: expected 0 to be greater than 0`;
- rossa uguale con `<p class="kit-note">…</p>` dopo `</BaseList>`.

Il numero di prove non cambia, perché è un'attesa in più dentro una prova che c'è già: gli Atteso dei compiti 6 e 8
restano quelli.

### M-2 — Minore — `concentricRadii` legge un angolo solo — voce candidata E30

**Dove.** `grep -n "A straight corner is never compared\|borderTopLeftRadius" gui/src/testing/probes.ts` →
`19: * … A straight corner is never compared (answer 20): an element or an …` e `23:  const radius = (element: Element):
number => Number.parseFloat(getComputedStyle(element).borderTopLeftRadius) || 0;`. Nel piano, riga 3101, recinto del
Passo 1.

**Evidenza.** Un elemento con angoli diversi viene giudicato con il raggio in alto a sinistra su tutti e quattro. L'ho
misurato con una prova usa-e-getta scritta per fallire (`review4\probe-limits.browser.test.ts`, in `src/testing/` del
clone, da sola, tolta dopo; `p8-mixed-radii.log`):

| Il caso, dentro un esterno tondo 20 | Ciò che promette il commento | Ciò che rende la sonda |
|---|---|---|
| controllo: un interno tondo 30 su tutti gli angoli, nell'angolo in basso a sinistra a 0/0 | preso | preso: `inner in outer, bottom-left: radius 30.0, outer 20.0, distance 0.0/0.0` |
| lo stesso difetto con il **solo** angolo in basso a sinistra tondo, `0 0 0 30px` | preso | `{ near: 0, bad: [] }`: **non lo giudica mai** |
| la forma di un `sheet`, `16px 16px 0 0`, con gli angoli dritti in basso a 12/12 dagli angoli dell'esterno | nessun confronto (risposta 20) | **due** voci in `bad`: `sheet in outer, bottom-left: radius 16.0, outer 20.0, distance 12.0/12.0` e `bottom-right` |

Oggi la pagina kit non ne risente: tutti i suoi elementi hanno i quattro angoli uguali, e la finestra che mostra è la
`center`. Ma angoli diversi ci sono già: `gui/src/components/BaseDialog.vue:77` (`sheet`, `var(--radius-card)
var(--radius-card) 0 0`) e `gui/src/frame/Drawer.vue:52`. E il piano, al compito 6, scrive `.dv-floating-titlebar` con
`20 20 0 0` e `.dv-groupview` con `0 0 20 20` dentro un contenitore galleggiante (righe 5234 e 5240). I compiti 6 e 8
importano questa sonda. La lettura del solo angolo in alto a sinistra viene fedele da `sonda-raggi.js`; la frase *«un
angolo dritto non lo confronta»* viene dalla (f) e dalla trappola 4.

**Cura proposta (E30).** Due strade:

- **A**: un raggio per angolo, all'interno e all'esterno, e un angolo dritto da una delle due parti non si confronta. È
  il testo qui sotto, che sostituisce la funzione nel recinto del Passo 1. L'ho misurata nel clone (`logs\F2cure*.log`):
  - la pagina e le tre prove usa-e-getta verdi, `Test Files  3 passed (3)`, `Tests  26 passed (26)`;
  - `npx vue-tsc --noEmit` esce 0, zero `error TS`;
  - le righe 1, 2 e 8 del Passo 7 rosse con gli stessi messaggi di oggi, la riga 8b verde;
  - la cornice e la striscia a raggio sbagliato (C3, C4 del §3.8) rosse come oggi.

  Il costo: una decina di righe. E se le tre prove sugli esempi a mano entrano nel cancello, come file
  `gui/src/testing/probes.browser.test.ts`, crescono i conti del progetto `browser` negli Atteso dei compiti 6 e 8, da
  riallineare come per **E25**.
- **B**: dichiarare il limite nel commento: *«It reads the TOP-LEFT radius for all four corners: an element whose
  corners differ -- a sheet, `r r 0 0` -- is judged as if all four were that one, and one whose top-left is straight
  is skipped whole.»*

Consiglio **A**: la risposta 20 è la regola approvata, e B lascia ai compiti che importano la sonda un verde e un rosso
falsi che si conoscono già. Tocca però una sonda approvata dal disegno, la trappola 4: se sia merito (vincolo 1) lo decide
il coordinatore.

```ts
/**
 * The owner's rule of answer 4 -- OUTER radius = INNER radius + distance. For every element with a rounded corner, the
 * nearest rounded ancestor inside a root, and their four corners, EACH WITH ITS OWN RADIUS: where the inner corner sits
 * close to the outer one (within the larger radius, plus 2 px), an element IN the corner must share the centre, and one
 * OFF the corner must not be rounder than the outer radius minus the smaller distance. A straight corner, inside or
 * outside, is never compared (answer 20) -- a sheet's `r r 0 0` included. SVG content is a drawing, not a surface.
 */
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
  const CORNERS = ["TopLeft", "TopRight", "BottomLeft", "BottomRight"] as const;
  type Corner = (typeof CORNERS)[number];
  const radius = (element: Element, corner: Corner): number =>
    Number.parseFloat(getComputedStyle(element)[`border${corner}Radius` as const]) || 0;
  const rounded = (element: Element): boolean => CORNERS.some((corner) => radius(element, corner) > 0);
  const effective = (element: Element, corner: Corner): number => {
    const box = element.getBoundingClientRect();
    return Math.min(radius(element, corner), box.height / 2, box.width / 2);
  };
  const bad: string[] = [];
  let near = 0;
  for (const root of roots) {
    for (const element of [root, ...root.querySelectorAll("*")]) {
      if (element.closest("svg") !== null) continue;
      if (!rounded(element)) continue;
      let ancestor = element.parentElement;
      while (ancestor !== null && !rounded(ancestor)) ancestor = ancestor.parentElement;
      if (ancestor === null || !root.contains(ancestor)) continue;
      const b = element.getBoundingClientRect();
      const B = ancestor.getBoundingClientRect();
      const corners: [string, Corner, number, number][] = [
        ["top-left", "TopLeft", b.left - B.left, b.top - B.top],
        ["top-right", "TopRight", B.right - b.right, b.top - B.top],
        ["bottom-left", "BottomLeft", b.left - B.left, B.bottom - b.bottom],
        ["bottom-right", "BottomRight", B.right - b.right, B.bottom - b.bottom],
      ];
      for (const [name, corner, dx, dy] of corners) {
        const inner = effective(element, corner);
        const outer = effective(ancestor, corner);
        if (inner === 0 || outer === 0) continue;
        const reach = Math.max(outer, inner) + 2;
        if (!(dx < reach && dy < reach)) continue;
        near += 1;
        const inTheCorner = Math.abs(dx - dy) <= 1.5;
        const ok = inTheCorner ? Math.abs(inner - (outer - dx)) <= 1.5 : inner <= outer - Math.min(dx, dy) + 1.5;
        if (!ok) {
          bad.push(`${describe(element)} in ${describe(ancestor)}, ${name}: radius ${inner.toFixed(1)}, outer ${outer.toFixed(1)}, distance ${dx.toFixed(1)}/${dy.toFixed(1)}`);
        }
      }
    }
  }
  return { near, bad };
}
```

### M-3 — Minore — la guardia di `scripts/gate-gui.sh` non vede un `dist/` vecchio — voce candidata E31

**Dove.** `grep -n "non-vacuity guard\|test -f dist/index.html\|^npm run build" scripts/gate-gui.sh` → `35:npm run build`,
`38:# non-vacuity guard -- a build that produced nothing would pass the second.`, `39:test -f dist/index.html || …`. Nel
piano, il recinto *Sostituisci con* del Passo 6, righe 3753-3760.

**Evidenza.** Ho lanciato nel clone le righe dello script così come sono: `review4\mirror_build_check.sh` le prende da
`scripts/gate-gui.sh` con `sed`, da `echo "-------- gui: build"` fino alla riga della pagina kit, e le esegue sotto
`set -euo pipefail`.

| Il caso | L'uscita |
|---|---|
| `build: { outDir: "out", rolldownOptions: { input: { index: "index.html", kit: "kit.html" } } }`, senza `dist/` | `dist/index.html is missing: the build produced nothing to check`, exit 1: la guardia scatta (`p4-guard-caseA-nodist.log`) |
| la stessa configurazione, con il `dist/` di un *build* precedente ancora lì | `mirror: both checks passed`, exit 0, mentre `out/kit.html` è nato (`p4-guard-caseB-stale.log`) |
| la sola seconda riga, senza `dist/` | `grep: dist/assets: No such file or directory`, e passa: la guardia serve davvero |

`npm ci` non tocca `dist/`, che sta in `.gitignore` (`/gui/dist/`), e il cancello gira su macchine che lo conservano da
una corsa all'altra. Il commento *«a build that produced nothing would pass the second»* è vero solo dove un `dist/`
vecchio non c'è: in CI sì, in locale no. Quando potrebbe spostarsi l'uscita: per esempio col guscio del sotto-progetto 10.
È una deduzione, il piano non lo dice.

**Cura proposta (E31).** Nel recinto *Sostituisci con* del Passo 6, fra `echo "-------- gui: build"` e `npm run build`:

```bash
# ⛔ NO OUTPUT OF A PREVIOUS BUILD: `npm ci` keeps `dist/`, and a build that wrote elsewhere -- or nothing -- would leave
# the checks below reading the old one (E31 of the design-system plan).
rm -rf dist
```

L'ho misurata nel clone: con la cura, con l'uscita spostata e il `dist/` vecchio, la guardia scatta,
`dist/index.html is missing…`, exit 1 (`p4-cure-red.log`); con la cura e la configurazione del commit passano tutte e due,
`dist/assets/index-CiZv4zPX.js  663.93 kB` (`p4-cure-green.log`). Un costo misurabile non c'è: con `outDir` al posto
solito, Vite vuota già la cartella.

### N-1 — Nit — `iconsCentred` guarda l'attributo, non il colore — voce candidata E32

**Dove.** `grep -n 'getAttribute("stroke")' gui/src/testing/probes.ts` →
`107:      if (svg.getAttribute("stroke") !== "currentColor") problems.push(…)`. La prova si chiama *«draws every icon in
currentColor, and centres it»* (`kit.browser.test.ts:83`). Nel piano, riga 3189.

**Evidenza.** Ho aggiunto `stroke: var(--color-text-stop);` a `.base-label :deep(.base-icon)` in `BaseLabel.vue`: le icone
delle etichette si disegnano nel colore dell'errore, e il progetto resta `Tests  23 passed (23)` (`logs\T6.log`). Una
regola CSS vince sull'attributo di presentazione. Oggi in `gui/src` nessuna regola scrive `stroke`: `grep -rn "stroke"`
rende solo `stroke-width` e `--icon-stroke`. Anche la sonda delle tavole guarda l'attributo.

**Cura proposta (E32).** Al posto della riga 107:

```ts
      const drawn = getComputedStyle(svg);
      if (drawn.stroke !== drawn.color) problems.push(`stroke is not currentColor: ${name}`);
```

L'ho misurata nel clone (`logs\F3cure.log`, `logs\F3cure+T6.log`):

- verde sul codice del commit, `Tests  23 passed (23)`;
- rossa con la regola aggiunta, `2 failed | 21 passed (23)`, le sole due prove delle icone, `stroke is not currentColor: modules` e altre quattordici righe, nei due temi.

### N-2 — Nit — il limite di E27 non sta scritto nel codice — voce candidata E33

**Dove.** `probes.ts:52`, il ramo *fuori dall'angolo*; la prova dei raggi, `kit.browser.test.ts:67-72`.

**Evidenza.** Con un `throw` sul ramo, sul codice del commit, `Tests  23 passed (23)` (`logs\T1.log`), come dice **E27**:
la pagina non lo percorre mai. Con lo stesso `throw` e lo spostamento di **E27** diventa rosso, `Error: off the corner:
base-list-row bottom-left 17.0/13.0` (`logs\T1b.log`): il ramo si raggiunge, e lo esercita la riga del Passo 7, mai il
cancello. Né il commento della sonda né quello della prova lo dicono, e la guardia `near > 0` si legge come se le due
regole fossero tenute entrambe.

**Cura proposta (E33).** Se **E30** prende la strada A con gli esempi a mano, una coppia fuori dall'angolo, una giusta e una
sbagliata, tiene il ramo nel cancello. Altrimenti una frase nel commento della prova dei raggi:
`// ⚠️ The page puts every piece IN its corner: the rule off the corner is proven by the E27 row of step 7, not here.`

### N-3 — Nit, fuori dal commit — `compare_task4.py` non vede la data del dispaccio né il modo dei file

**Dove.** `grep -n "DATE = row4\|COMMIT_DAY\|def show" …/compare_task4.py` → `67`, `68`, `143-144`, `39`.

**Evidenza.** Nel clone, due commit miei, poi di nuovo a `841dc54`:

- la riga 4 a `✅ 2026-09-26`, il giorno del commit, invece del `2026-09-25` fissato dal dispaccio → tutto `OK`, nessun `CHECK BY HAND`, exit 0;
- `git update-index --chmod=+x gui/kit.html` (`mode change 100644 => 100755`) → tutto `OK`, exit 0.

La docstring dichiara solo i fine-riga. Sul commit rivisto non cambia niente: la data è quella dettata e il modo non
cambia (`git diff --summary 1b531e9 841dc54` → cinque `create mode 100644`).

**Cura proposta.** Due aggiunte, oppure le due frasi nella docstring:

- la data del dispaccio come terzo argomento facoltativo, confrontata con la riga 4, e `DIFFERS` se diversa;
- `git diff --summary <base> <target>`, con un `mode change` su un percorso che diventa `UNEXPECTED`.

### N-4 — Nit — confermo E28

**Dove.** `gui/eslint.config.js:112-123`: la testa *«⛔ THE IMPORT RULES OF THE KIT … One rule, a scope per block, and the
ORDER MATTERS …»* ora introduce `harness/kit-page-specimens`, che spegne `@intlify/vue-i18n/no-raw-text` e non è una regola
d'import. È la specie del gotcha **#58**, come **E17** ed **E22** nello stesso file. Sul comportamento non pesa: L1, L2 e
L3 sono rosse come dettato (§3.5) e `npm run lint` è verde. La cura dell'implementatore è giusta: il *Trova* ancorato
alla fine del blocco di `Chat.vue`, `    rules: { "vue/no-v-html": "off" },\n  },`, è unico
(`grep -c 'rules: { "vue/no-v-html": "off" },' gui/eslint.config.js` → `1`).

### Guardato, e non è un rilievo

- **Tre righe col loro perché, tenute ciascuna da una prova.** Ho tolto ciascuna, una per volta, sul codice del commit:
  - `.kit-frame` con `--radius-card` al posto di `--radius-frame` → 4 voci `base-button in kit-frame, …: radius 20.0, outer 20.0, distance 12.0/12.0` (`logs\C3.log`);
  - `.kit-strip` con `--radius-card` → `base-button in kit-strip, top-right/bottom-right: … distance 9.0/9.0` (`logs\C4.log`);
  - lo smontaggio in `afterEach` di **E25** → la prova di **E20** cade nei due temi, `TimeoutError: locator.hover:
    Timeout 14886ms exceeded.` (`logs\C5.log`). La cura di **E25** la tiene il cancello anche sulla corsa verde, non solo
    nelle cascate.
- **`body { margin: 0; }` di `Kit.vue`**: tolto, nessuna prova cade (`logs\C2.log`). Il perché è l'aspetto di una
  pagina di sviluppo, e una prova ripeterebbe il CSS: non la propongo.
- **I rami di salto delle sonde.** Con un `throw` su ciascuno resta `Tests  23 passed (23)`, quindi la pagina non li
  prende mai (`logs\T2.log`–`T5.log`):
  - la scatola vuota di `fits`;
  - il `box` nullo di `fits`: le radici stanno fra le scatole;
  - l'icona in una riga che non centra, in `iconsCentred`: `centred` è 47 su 47.

  Invece ogni elemento con del testo ha `clientWidth > 0`, quindi la regola del taglio li giudica tutti. I rami che
  **giudicano** li esercita il Passo 7: nell'angolo, il taglio, lo sbordare, fuori centro, disegnata e `stroke`. Resta
  scoperto solo quello di N-2.
- **Il 404 al caricamento** della pagina nel server di sviluppo è `/favicon.ico` (`curl` → 404). Né `kit.html` né
  `index.html` dichiarano un'icona: vale anche per la SPA, ed è fuori dal compito.
- **Le frecce sui radio di `reka-ui`**: la nota dell'implementatore è giusta, l'ho vista nel sorgente
  (`node_modules/reka-ui/dist/RadioGroup/RadioGroupItem.js:61-81`, `isArrowKeyPressed` e `setTimeout`) e l'ho misurata:
  - `keyboard.press("ArrowDown", { delay: 120 })` sceglie;
  - `keyboard.press("ArrowDown")` senza attesa sposta solo il fuoco (`review4\shots\look.log`, righe `22`–`24`).

  Conta per le prove con la tastiera del compito 5.
- **La ragione `elmPartiallyObscured` della riga 4 del Passo 7**, che l'implementatore non aveva misurato, l'ho misurata
  (`logs\04why-base.log`, `logs\04why.log`):
  - sul commit, `incomplete` di `color-contrast` vuoto nello scuro;
  - con la violazione, `button[data-pill="true"] :: elmPartiallyObscured`.

## 3. I comandi rilanciati, con le uscite

### 3.1 L'avvio, e ogni affermazione misurabile del rapporto

| Comando | Uscita |
|---|---|
| `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline 1b531e9..HEAD` | `841dc54`; vuoto; **una** riga |
| `git show --stat 841dc54` | i nove file del rapporto, `9 files changed, 585 insertions(+), 2 deletions(-)` |
| `git log -1 --format=%B 841dc54 \| grep -ci co-authored`; il messaggio | `0`; comincia con `design-system(compito 4): ` |
| `git status -sb` | `## main...origin/main [ahead 1]` |
| `git diff --stat 1b531e9..841dc54 -- crates/ gui/schema/` e `-- gui/package.json gui/package-lock.json` | vuoti |
| `git diff --summary 1b531e9 841dc54` | cinque `create mode 100644`, nessun cambio di modo (`scripts/gate-gui.sh` resta `100644`) |
| i log dell'implementatore: `ls -la --time-style=full-iso …\scratchpad\task4\*.log` contro `git log -1 --format=%ci 841dc54` | commit `2026-09-26 00:14:32 +0200`; `gate-final-20260926-0020.log` `00:14:01`, `check-docs-20260926.log` `00:14:25`, `gate-open-20260925-2354.log` `2026-09-25 23:55:12`: sono suoi e **prima** del commit. Il nome del finale porta un'ora stimata, e il rapporto lo dichiara |
| le loro righe | apertura: `18 passed \| 1 skipped (19)`, `125 passed \| 1 skipped (126)`, `1 passed (1)`, `5 passed (5)`, `index-CiZv4zPX.js 663.93 kB`, `found 0 vulnerabilities`, `GATE GREEN.`, uguali a `gate-baseline.log` del coordinatore; finale: le stesse col `browser` a `2 passed (2)`, `23 passed (23)`; `check-docs`: `OK — no inconsistencies.` |
| i conteggi delle guardie del rapporto (`roots=9 near=12 bad=0 seen=87 boxed=87 problems=0 icons=47 centred=47 iconProblems=0 off=… dialogNear=1 dialogBad=0`), rimisurati con `review4\counts.browser.test.ts`, usa-e-getta, scritta per fallire | `light: roots=9 near=12 bad=0 seen=87 boxed=87 problems=0 icons=47 centred=47 iconProblems=0 off=primary/quiet/secondary dialogNear=1 dialogBad=0`, e lo stesso nello `dark` (`p1-counts.log`): **uguali** |
| fine-riga: `git ls-files --eol` e `tr -cd '\r' \| wc -c` contro `wc -l`, dopo; e i blob di `1b531e9` prima | i nove file `i/lf w/lf`, CR 0; righe 8807, 150, 12, 266, 151, 8, 126, 122, 83; prima `eslint.config.js` 143, `vite.config.ts` 114, `gate-gui.sh` 78, il piano 8807, CR 0; tutti finiscono con un LF. **Uguali** al §5 del rapporto |

### 3.2 La conformità al dettato, e lo script provato nelle due direzioni

**Letto.** La ricetta del piano, `grep -n "La ricetta del compito 4"` → riga 8771, porta otto righe: `W probes.ts 3082`,
`R vite.config.ts 3217 3226`, `W kit.browser.test.ts 3243`, `W kit.html 3407`, `W main.ts 3424`, `W Kit.vue 3437`,
`R eslint.config.js 3710 3717`, `R gate-gui.sh 3746 3753`. Ciascuna apre il recinto giusto: letto con `sed -n` sul piano
a `1b531e9`. Sono tutti i file e tutte le sostituzioni del compito, e lo script ricostruisce le due celle a parte.

**Lanciato.** `PYTHONIOENCODING=utf-8 python …/compare_task4.py 1b531e9 841dc54`:

- nove `OK`, il piano *«(the two cells of step 8)»*;
- `CHECK BY HAND  the date 2026-09-25 of row 4 is not the commit's day 2026-09-26`, che è atteso;
- `--- 9 paths, 0 not matching the plan's text`, exit **0**.

**La direzione rossa, nel clone.**

- Un carattere in un commento di `Kit.vue`, *«column's»* → *«column'x»*, in un commit (`e866893`) → `DIFFERS  gui/src/kit/Kit.vue` col diff di quella riga, nessun altro `DIFFERS`, `1 not matching`, exit **1**.
- Una cella del piano sbagliata (`c7b7bce`) più un file non dettato (`BaseStatus.vue`) → `DIFFERS` sul piano, `UNEXPECTED` su `BaseStatus.vue`, exit **1**.
- I due punti ciechi di N-3.

Il clone torna ogni volta a `841dc54`, con lo stato vuoto.

**Le celle, a mano.** Il diff del piano cambia le sole righe 3 e 4 della tabella della posizione:

- riga 3, colonna Commit `` `c7b7bcd`, con la cura `9d2ffb0` ``;
- riga 4, `— | ✅ 2026-09-25`.

### 3.3 Il rosso del Passo 2, e la sequenza di E24 nelle due direzioni

Nel clone a `841dc54`, dopo `npm ci`. Ho salvato e poi tolto `gui/kit.html`, `gui/src/kit/main.ts` e
`gui/src/kit/Kit.vue`; sono rimasti `kit.browser.test.ts`, `probes.ts` e la riga di **E24**.

| Passo | Comando | Uscita |
|---|---|---|
| 2 | `(cd gui && npx vitest run --project browser src/kit)` | `Forced re-optimization of dependencies`; `(!) Failed to run dependency scan. Skipping dependency pre-bundling.`; `Internal server error: Failed to resolve import "./Kit.vue" from "src/kit/kit.browser.test.ts". Does the file exist?`; `Test Files  1 failed (1)`, `Tests  no tests`, exit 1 (`p3-step2-red.log`) — l'Atteso e il rapporto. La cache lasciata: `@vue/test-utils` e tre pezzi di `vitest`, **senza** `vue`, `reka-ui` né `lucide` (`_metadata.json`) |
| 5, subito dopo | i tre file dalle copie, `cmp` uguali, `git status --porcelain` com'era; `npx vitest run --project browser` | `Forced re-optimization…`, `Test Files  2 passed (2)`, `Tests  23 passed (23)`, 3.27 s: **verde alla prima corsa** (`p3-step5-green.log`) |
| 5 | `npm run lint`; `npm run build` | exit 0 senza righe; `✓ 746 modules transformed.`, `index-Bx6RaF8P.css 145.10 kB`, `index-CiZv4zPX.js 663.93 kB │ gzip: 201.53 kB`, `dist/` con il solo `index.html` |
| **E24 rossa**: tolta la sola `optimizeDeps: { force: true },`, `node_modules/.vite` svuotata (come la misura di E24: *«la cache rifatta da una corsa senza `Kit.vue`»*), la stessa sequenza | Passo 2 | rosso uguale, cache senza `reka-ui` (`p3-noforce-step2-red.log`) |
| | Passo 5, prima corsa | `dependencies optimized: axe-core, lucide, reka-ui, vue`; `optimized dependencies changed. reloading`; `[vitest] Vite unexpectedly reloaded a test.`; **`Tests  18 failed \| 5 passed (23)`**, tutte le prove della pagina, un errore solo `TypeError: 'set' on proxy: trap returned falsish for property 'style'` dentro `reka-ui.js` via `@vue_test-utils.js` (`p3-noforce-step5.log`). È ciò che **E24** scrive. I diciotto e non i quattordici di E24 sono le quattro prove di **E25**, venute dopo quella misura |
| | la corsa dopo | `Tests  23 passed (23)` (`p3-noforce-rerun.log`); `vite.config.ts` dalla copia, `cmp` uguale, stato com'era |

### 3.4 Il Passo 6 nelle due direzioni, e la guardia

`bash scripts/gate-gui.sh` nel clone, da solo, in background.

**Rosso.** Con `build: { rolldownOptions: { input: { index: "index.html", kit: "kit.html" } } },` accanto a `define`
(`p4-gate-gui-red.log`, 34 s):

- `dist/kit.html 0.54 kB`;
- `dist/assets/kit-CP1QMCqd.css 9.67 kB`, `kit-Bw1CQ6GG.js 31.78 kB`, `index-Dwx_PUol.js 521.80 kB`, gli stessi nomi del rapporto;
- `the kit page is in the package`, exit 1.

**Verde.** Senza la riga, `vite.config.ts` dalla copia e `cmp` uguale (`p4-gate-gui-green.log`, 50 s):

- `dist/assets/index-CiZv4zPX.js 663.93 kB`;
- `jsdom` `Test Files  18 passed | 1 skipped (19)`, `Tests  125 passed | 1 skipped (126)`;
- `browser` `Test Files  2 passed (2)`, `Tests  23 passed (23)`;
- `found 0 vulnerabilities`, exit 0.

**La guardia.** Scatta quando `dist/index.html` manca dopo il *build*, e il `dist/` vecchio la inganna: vedi M-3.

### 3.5 Il Passo 7, una violazione per volta, nel clone

`review4\violations.py`, con `prima7.txt` preso prima (vuoto). Per ogni violazione il file deve essere uguale alla sua
copia salvata **prima**. La violazione sostituisce **un'**occorrenza unica, lo script lancia il comando e rimette la
copia. Dopo ciascuna `cmp` rende *«bytes equal: True»*, e alla fine `git status --porcelain | diff prima7.txt -` non rende
nulla. I log sono `review4\logs\NN.log`.

| # | Violazione | Esito | Prove che cadono | Messaggio vero |
|---|---|---|---|---|
| 1 | `BaseButton.vue` `var(--radius-card)` | `2 failed \| 21 passed (23)` | la finestra, nei due temi | `base-button in base-dialog, bottom-right: radius 16.0, outer 20.0, distance 13.0/13.0` |
| 2 | `BaseList.vue` righe a `var(--radius-card)` | `2 failed \| 21 passed (23)` | i raggi, nei due temi | `base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 13.0/13.0` e `bottom-right` |
| 3 | `.kit-note` `white-space: nowrap; overflow: hidden;` | `2 failed \| 21 passed (23)` | `fits`, nei due temi | `cut: kit-note "Principale, normale, dis" 490>422` |
| 4 | `.kit-strip > .base-button { margin-right: -40px; }` | `4 failed \| 19 passed (23)` | `fits` e `axe` della pagina, nei due temi | `sticks out: base-button of kit-strip`; `AssertionError: expected 1 to be +0` |
| 5 | `BaseLabel.vue` icona `top: 3px` | `2 failed \| 21 passed (23)` | le icone, nei due temi | quindici righe da `off centre by 3.00 px: modules in base-label` |
| 6 | `.kit` `font: 400 0.875rem/1.25rem serif` | `2 failed \| 21 passed (23)` | i caratteri, nei due temi | `expected 'serif' to be 'Geist Variable'` |
| 7 | `themes.css` muted dello scuro a `var(--ref-neutral-39)` | `2 failed \| 21 passed (23)`; jsdom `src/tokens` `2 failed \| 13 passed (15)` | le due `axe` dello **scuro**: pagina e finestra; `board.test.ts` *«themes.css is the board's block, byte for byte»* e `contrast.test.ts` *«dark: every pair reads at its threshold…»* | `color-contrast: #v-0-legend, .kit-card:nth-child(1) > h2, …`; `color-contrast: #reka-dialog-description-v-15, .actions > .base-button[data-variant="quiet"]…` |
| 8 | righe a `var(--radius-card)` **e** `margin-inline: var(--space-1)` | `2 failed \| 21 passed (23)` | i raggi, nei due temi | `… bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0` e `bottom-right` |
| 8b | la sola `margin-inline`, raggio giusto | **`Tests  23 passed (23)`** | nessuna | — la metà verde di **E27** |
| 9 | tolta la regola del `quiet` spento | `2 failed \| 21 passed (23)` | i pulsanti spenti, nei due temi | scuro: atteso `quiet: rgb(111, 102, 96)`, reso `quiet: rgb(163, 154, 143)`; chiaro: atteso `rgb(163, 154, 143)`, reso `rgb(101, 91, 87)` |
| 10 | `.frame:hover:not([data-disabled])` | `2 failed \| 21 passed (23)` | il bordo dell'errore, nei due temi | scuro `expected 'rgb(163, 154, 143)' to be 'rgb(129, 27, 7)'`; chiaro `expected 'rgb(101, 91, 87)' to be 'rgb(251, 182, 168)'` |
| 11 | tolta `await userEvent.hover(frame as HTMLElement);` | `2 failed \| 21 passed (23)` | la stessa prova, alla guardia | `AssertionError: expected false to be true` |
| L1 | tolto `harness/kit-page-specimens` | exit 1 | — | `✖ 33 problems (33 errors, 0 warnings)`, un file solo, `src/kit/Kit.vue`: da `56:11 error raw text 'Il kit' is used` a `151:41 error raw text 'Icone' is used` |
| L2 | `role="status">ciao<slot />` in `BaseStatus.vue` | exit 1 | — | `11:42 error raw text 'ciao' is used @intlify/vue-i18n/no-raw-text`, `✖ 1 problem` |
| L3 | `import { Search } from "lucide";` in `Kit.vue` | exit 1 | — | `2:1 error 'lucide' import is restricted from being used. icons pass through BaseIcon and the one map, …`, `✖ 1 problem` |

Ogni riga cade per la ragione scritta, soltanto sulle prove che nomina e da sola: sono i messaggi del rapporto, uno per
uno.

### 3.6 Guardarla

Il server di sviluppo gira nel clone con `npm run dev -- --port 5199 --strictPort`; `/kit.html` risponde `HTTP 200`. Gli
script sono `review4\look.mjs`, `look2.mjs` e `look3.mjs`: prendono `playwright` dal `node_modules` del clone con
`createRequire` e aprono `chromium.launch({ channel: "chrome" })`, senza finestra, e il browser dice `154.0.8037.58`. Poi
ho fermato il server per PID, dopo averne letto la riga di comando (`vite.js --port 5199`, più `npm run dev`). Dopo non
restava nessun `node.exe` e nessun Chrome di Playwright. Il §4 dice che cosa ho visto.

### 3.7 Il cancello, sull'albero del repository, da solo

`bash scripts/gate.sh`, a `841dc54`, in background (`review4\gate-review-20260926.log`, 78 s, exit 0):

- `GATE GREEN.`;
- `jsdom` `Test Files  18 passed | 1 skipped (19)`, `Tests  125 passed | 1 skipped (126)`;
- `browser` `Test Files  2 passed (2)`, `Tests  23 passed (23)`;
- `dist/assets/index-CiZv4zPX.js  663.93 kB │ gzip: 201.53 kB`: uguale alla base, la pagina kit non lo muove;
- `found 0 vulnerabilities`, e la coerenza dei documenti `OK — no inconsistencies.`

Poi `bash scripts/check-docs.sh` rende exit 0, `OK — no inconsistencies.`, e `git status --porcelain` è vuoto.

**Le prove per nome di file.** `git ls-tree -r --name-only <c> gui/src | grep '\.test\.ts$'` a `1b531e9` e a `841dc54`:

- lato `jsdom` 19 e 19, con gli stessi nomi;
- lato `browser` 1 e 2: si aggiunge `gui/src/kit/kit.browser.test.ts`;
- `git diff --stat 1b531e9 841dc54 -- 'gui/src/**/*.test.ts'` → il solo file nuovo.

Nessuna prova di prima sparisce, e nessuna cambia.

### 3.8 Il codice, oltre il dettato

- **(a) I rami delle sonde**: tabella del §2 e M-1, N-2. Oltre ai log `T1`–`T6`, `C1`–`C5`, `F1cure*`, `F2cure*`,
  `F3cure*`, ho usato `near-count`, `probe-limits` e `counts`, tutti in `review4\`.
- **(b) Lo smontaggio.** Ogni `mount` passa da `kit()` e finisce in `kits`; `afterEach` smonta tutto e svuota il `body`, e
  nessuna prova si smonta da sé. Lo tiene la prova di **E20** (`C5`).
- **(c) Le righe col perché**: M-1, C2, C3, C4, C5 e il §3.3. La riga di **E24** non la vede il cancello, ed è dichiarato
  nel suo commento; la sequenza la prova.
- **(d) Commenti e confini.** `grep` su *task 4*, *kit page*, `index.html`, `testing/`: nessun commento diventa falso.
  `testing/axe.ts:9-10` ora è vero e P-25 resta vero, perché dice chi è arrivato per secondo. Il blocco del linter è
  provato sul confine con L2 e L3. La specie del gotcha **#58** c'è una volta sola: **E28**, N-4.
- **Le interfacce.** Le firme di `probes.ts` sono quelle della lista *Interfaces*: `concentricRadii` → `{ near; bad }`,
  `fits` → `{ seen; boxed; problems }`, `iconsCentred` → `{ icons; centred; problems }`, e `firstFamily`. Così le usano i
  compiti 6 (riga 5024) e 8 (righe 7244-7252). `Kit.vue` ha la prop `initialTheme?: ThemeChoice`, e ci sono le radici
  `kit-card`, `kit-frame`, `kit-strip`. Le due sostituzioni del compito 8 su `kit.browser.test.ts` (piano, *Trova* alle
  righe 6652 e 6677) si applicano **una volta ciascuna** al file committato, in fila (`review4\task8_subs.py`: `1`, `1`).
  Dopo, `contrastJudged` arriva da `testing/axe` e non resta nessun `import axe`.
- **I vincoli 4, 5 e 6.** Nei pezzi di base non c'è nessuna scritta fra i tag, e L2 prova che la regola resta rossa fuori
  da `src/kit/`. Il `grep` dei colori a mano in `gui/src/**/*.vue` è vuoto, e la prova `usage.test.ts` percorre tutto
  `src`, `Kit.vue` compresa. Nessun `var(--ref-` fuori da `tokens/`.

### 3.9 I vincoli globali

- **7**: `package.json` e il lockfile non cambiano.
- **8**: nessuna versione tocca.
- **9**: il §3.1: i file nuovi sono LF.
- **11**: le due direzioni, §3.5. Ogni prova del browser porta la sua guardia, `near`, `seen`/`boxed`,
  `icons`/`centred`, la lista delle tre varianti, `:hover`, `passes`/`incomplete` e `dialog` di lunghezza 1; il limite è
  M-1.
- **12**: vuoto.

### 3.10 Il contratto del prompt dell'implementatore

- un commit, coi soli file del suo punto 2 e le due celle;
- il messaggio con il prefisso giusto;
- nessun push (`ahead 1`);
- nessun co-autore (`0`);
- nulla in `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package*.json`, `docs/adr/`.

## 4. Che cosa ho visto guardando la pagina

Le immagini stanno in `review4\shots\`, a 1440 × 900 (la misura delle prove) e a grandezza vera; lo stato di ogni passo è
in `review4\shots\look.log`. La pagina misura `1440x1254 in 1440x900`: nessuno scorrimento orizzontale.

| Tema | Immagini | Che cosa vedo |
|---|---|---|
| **Chiaro** | `02-chiaro.png`, `10-chiaro-part-01…11.png` | la radice `data-theme="light"`. I raggi concentrici dove la pagina li costruisce: le carte da 20 a 12 px dentro la cornice da 32, il pulsante a pillola nella striscia a pillola, la riga finale della lista nell'angolo della sua scheda. Nessun testo tagliato: la nota lunga dei pulsanti va a capo. Niente sborda, e le icone sono centrate nei pulsanti, nelle etichette, nei campi e nella fila delle 24. I tre «Spento» nel grigio dello spento, il `quiet` compreso (**E19**, a vista). Il campo con l'errore ha il bordo **pallido**: `--color-border-stop` è un ruolo di decoro nel disegno dei token (**E5**), e l'errore lo portano la frase rossa sotto e `aria-invalid`. Non è della pagina |
| **Scuro** | `03-scuro.png`, `10-scuro-part-01…11.png` | la radice `dark`, tutta la pagina scura. Il bordo dell'errore è rosso e ben visibile, «Home» ha il segno tutto intorno, il contrasto del testo si legge bene; il `<p>` di `BaseStatus` tiene i margini del browser, la nota dell'implementatore |
| **Sistema** | `01-start-sistema-system-light.png` e `04-sistema-system-light.png`, con il sistema chiaro; `05-sistema-system-dark.png`, con il sistema scuro emulato | segue il sistema: `light`, poi `dark` senza toccare la scelta |
| **La tastiera** | `21`…`24` (radio), `26` (il pulsante), `27`…`31` (la finestra), `32`, `33` (finestra nello scuro), `35`, `36` (finestra nel chiaro) | Tab porta sul radio scelto, col contorno visibile: `solid 2px rgb(122, 31, 46)` nel chiaro, `rgb(191, 85, 103)` nello scuro. Con la freccia tenuta 120 ms si sceglie *Chiaro* e poi *Scuro*, e la radice cambia. Con la freccia istantanea si sposta solo il fuoco, e *Scuro* resta scelto. 18 Tab fino ad *Apri la finestra*. Invio apre la finestra: un `dialog`, il fuoco su *Rifiuta*, il `body` a `pointer-events: none`. Tab porta su *Consenti* e un altro Tab torna su *Rifiuta*: il fuoco resta dentro. Esc chiude, il fuoco torna al pulsante, e `pointer-events` torna senza valore. `aria-describedby` punta alla descrizione. Nelle immagini a pagina intera con la finestra aperta, il velo copre solo i primi 900 px: è fisso, ed è un artefatto della cattura |
| **800 px** | `40-800px-chiaro.png`, `40-800px-scuro.png` | due colonne, i pulsanti vanno a capo, `800x1632 in 800`: nessuno scorrimento orizzontale, e la regola del taglio, lanciata lì, non rende niente |

## 5. Ciò che non ho potuto verificare

- **La CI**: il push è del coordinatore.
- **L'altra macchina**, `zagor`, con `core.autocrlf` a `true`: i fine-riga e il Chrome di là non li ho misurati.
- **Il modo di lavorare dell'implementatore**: l'ordine della prova prima della pagina e `git status` prima del commit li
  vedo solo nelle ore dei suoi log. `step2-red.log` è delle 23:57:22, prima di `step5.log` delle 23:57:56.
- **`userEvent.keyboard` di Vitest sulle frecce di un radio di `reka-ui`**: non misurato. Ho misurato solo `keyboard.press`
  di Playwright, istantaneo e tenuto.
- **La cura A di E30 sulla sonda dei compiti 6 e 8**: il dock vestito e la Panoramica non esistono ancora. Che l'uscita
  del *build* si sposti (M-3) è una deduzione.
- **Il lettore di schermo**: fuori da questa revisione.
