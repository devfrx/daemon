# La revisione del compito 3 — il kit: gli otto pezzi di base, la mappa delle icone, le regole del linter

Revisore del compito 3, macchina `zagor`, il 2026-09-25: dalle 13:22 circa; interrotto verso le 14:05 — il processo di
Claude Code si è chiuso — e ripreso col messaggio del coordinatore delle 14:12. Il mandato: `review-3-prompt.md`; il
commit: `c7b7bcd`, sopra `ef3e865`. Il clone delle mutazioni: `C:/Users/zagor/AppData/Local/Temp/rv3`, lasciato dov'è, a
`c7b7bcd` e pulito. I log: nello scratchpad `review3`.

## 1. Il verdetto

**Conforme.** `c7b7bcd` è il compito 3 come il piano lo detta — sedici percorsi uguali al testo del piano, le due celle,
il contratto — e ogni *Atteso* torna, rilanciato. I rilievi sono difetti del **dettato** — voci d'errata candidate
**E19**–**E23** — e dell'**attrezzo** di confronto; nessuno dell'esecuzione.

| Classe | Quanti | Quali |
|---|---|---|
| Critico | 0 | — |
| Importante | 0 | — |
| Minore | 6 | M-1 e M-2, due difetti del foglio dei pezzi; M-3, M-4 e M-5, sonde che mancano; M-6, `compare_task3.py` |
| Nit | 6 | N-1 e N-2, due frasi di `eslint.config.js`; N-3, il confine della regola di `lucide`; N-4, la carta con una misura; N-5 e N-6, il rapporto e l'ordine delle misure |

Le voci candidate, in una riga ciascuna — il testo proposto sta nel rilievo:

| Voce | Rilievo | Che cosa |
|---|---|---|
| **E19** | M-1 | `BaseButton.vue`: il `quiet` spento prende il colore dello spento |
| **E20** | M-2 | `BaseTextField.vue`: l'errore resta rosso sotto il puntatore |
| **E21** | M-3, M-4, M-5 | `kit.test.ts`: quattro sonde che mancano, ciascuna misurata verde sul codice e rossa sulla sua mutazione |
| **E22** | N-1, N-2 | `eslint.config.js`: due frasi, una che il compito smentisce e una che anticipa il compito 5 |
| **E23** | N-3 | il Passo 8 dichiara che un import da `lucide/…` passa il linter, e lo ferma `vue-tsc` |

---

## 2. I rilievi

### Minori

#### M-1 — `BaseButton.vue`: un pulsante `quiet` spento ha il colore di uno acceso — voce candidata **E19**

- **Dove.** `grep -n ':disabled {\|data-variant="quiet"\] {' gui/src/components/BaseButton.vue` → riga 78
  `.base-button:disabled {`, riga 102 `.base-button[data-variant="quiet"] {`. Nel piano, il recinto di `BaseButton.vue`
  del Passo 5 (a `ef3e865` la regola `:hover`/`:active` del `quiet` è alla riga 2311).
- **Perché.** Le due regole pesano uguale — una classe e una pseudo-classe contro una classe e un attributo, più
  l'attributo di `scoped` su entrambe — e quella del `quiet` viene dopo: il suo `color: var(--color-text-muted)` vince
  sullo spento. La tavola lo scrive a parte: `grep -n '^\.btn\.quiet\.is-disabled' docs/superpowers/specs/2026-09-22-design-system-tavole/token.html`
  → riga 320, `.btn.quiet.is-disabled{color:var(--color-text-disabled);background:transparent}`.
- **Evidenza.** Nel Chrome installato — `HeadlessChrome/153.0.0.0`, letto dal `userAgent` —, progetto `browser`, con una
  prova usa-e-getta nel clone (`src/components/zz-review.browser.test.ts`, tolta dopo la corsa, `git status --porcelain`
  vuoto), `getComputedStyle(button).color`:

  | Tema | `--color-text-muted` | `--color-text-disabled` | `quiet` | `quiet` spento | `secondary` spento |
  |---|---|---|---|---|---|
  | scuro | `rgb(163, 154, 143)` | `rgb(111, 102, 96)` | `rgb(163, 154, 143)` | **`rgb(163, 154, 143)`** | `rgb(111, 102, 96)` |
  | chiaro | `rgb(101, 91, 87)` | `rgb(163, 154, 143)` | `rgb(101, 91, 87)` | **`rgb(101, 91, 87)`** | `rgb(163, 154, 143)` |

  Il `primary` e il `card` spenti prendono lo spento; il `quiet` no. La pagina kit del compito 4 lo mette in mostra —
  riga 3363 del piano, `<BaseButton :variant="variant" disabled>Spento</BaseButton>` — e nessuna prova lo vede: jsdom non
  applica il foglio; e `axe` col contrasto, dal compito 4, non guarda un controllo spento — **dedotto** dall'esenzione dei
  componenti inattivi di WCAG 1.4.3, non misurato.
- **Cura proposta**, nel recinto di `BaseButton.vue`, dopo la regola `:hover`/`:active` del `quiet`:

  ```css
  .base-button[data-variant="quiet"]:disabled {
    color: var(--color-text-disabled);
  }
  ```

  Misurata nel clone con la stessa prova: il `quiet` spento → `rgb(111, 102, 96)` nello scuro e `rgb(163, 154, 143)` nel
  chiaro, il ruolo dello spento (`browser-probe-fixed.log`); il file tornato dalla copia, `cmp` uguale. La sonda che la
  tenga è del browser: il pre-controllo del compito 4 può chiedere il colore di ogni variante spenta contro
  `--color-text-disabled`.

#### M-2 — `BaseTextField.vue`: sotto il puntatore, un campo con l'errore perde il bordo rosso — voce candidata **E20**

- **Dove.** `grep -n 'hover:not\|\[data-error\]' gui/src/components/BaseTextField.vue` → riga 55
  `.frame:hover:not([data-disabled]) {`, riga 69 `.frame[data-error] {`; nel piano, riga 2512.
- **Perché.** La prima pesa una classe, una pseudo-classe e l'attributo dentro `:not()`, la seconda una classe e un
  attributo: vince la prima, sempre. Nella tavola `.field.is-error` (riga 326) viene dopo `.field.is-hover` (riga 323) con
  lo stesso peso, e vince l'errore.
- **Evidenza.** Stessa prova usa-e-getta, tema scuro, `userEvent.hover` sulla cornice di un campo con `error`: prima
  `rgb(129, 27, 7)` — `--color-border-stop` —, sotto il puntatore `rgb(163, 154, 143)` — `--color-text-muted`. La
  Panoramica del compito 8 dà l'errore al campo del nome (riga 7556 del piano): sparisce proprio mentre ci si punta.
- **Cura proposta.** `.frame:hover:not([data-disabled], [data-error]) {` — misurata: sotto il puntatore resta
  `rgb(129, 27, 7)`; il file tornato dalla copia, `cmp` uguale.

#### M-3 — `BaseDialog`: la seconda direzione di E16 non ha una prova — voce candidata **E21**, parte (a)

- **La lezione (a), sulla riga stessa di E16.** `grep -n "aria-describedby\|DialogDescription v-if" gui/src/components/BaseDialog.vue`
  → riga 31, `v-bind="description === undefined ? { 'aria-describedby': undefined } : {}"`, e riga 34, la
  `DialogDescription`. E16 prova il ramo **senza** descrizione; il ramo **con** non ha nessuno.
- **Evidenza.** Nel clone, `npx vitest run --project jsdom src/components/kit.test.ts` sotto ciascuna mutazione: **X1**, il
  `v-bind` che spegne sempre — `v-bind="{ 'aria-describedby': undefined }"` — → `Tests  23 passed (23)`; **X2**, la riga
  della `DialogDescription` tolta → `Tests  23 passed (23)`. Una finestra descritta perde il legame, o la descrizione, e
  nessuna prova cade. I compiti 5 e 8 la usano: la tripla della conferma, il suggerimento della Panoramica.
- **Cura proposta**, nella prova «BaseDialog, open, has no violation» (`kit.test.ts`, riga 225), dopo `await nextTick();`:

  ```ts
      // ⛔ THE OTHER DIRECTION OF E16: with a description, the dialog is described by it.
      const dialog = document.querySelector('[role="dialog"]');
      expect(document.getElementById(dialog?.getAttribute("aria-describedby") ?? "")?.textContent).toBe("Vale per questa sessione.");
  ```

  La stessa asserzione, in un file usa-e-getta (`zz-review.test.ts`, P1): verde sul codice; rossa su X1 e su X2,
  `expected undefined to be 'Vale per questa sessione.'`.

#### M-4 — `BaseRadioGroup`: «`null` included» non ha una prova — voce candidata **E21**, parte (b)

- **La lezione (a).** Il commento, righe 8–9: *«`reka-ui` 2.10.4 keeps no state of its own once a value is given, `null`
  included»*; la riga che lo fa, 23, `:model-value="modelValue"`. La prova «checks nothing on null, and names the group
  with its legend» (`kit.test.ts`, riga 174) guarda lo stato iniziale, non un clic.
- **Evidenza.** **X7**, `:model-value="modelValue ?? undefined"` — il `null` dato a `reka-ui` come nessun valore, cioè
  non controllato — → `Tests  23 passed (23)`. Con quella riga, un clic su un gruppo a `null` spunta il radio da sé: il
  caso di P-8 prima che il core risponda, la policy VRAM del compito 5.
- **Cura proposta**, nella prova della riga 174, montata con `attachTo: document.body` e un `"onUpdate:modelValue"` che
  raccoglie: un clic sul secondo radio, poi `expect(asked).toEqual(["local"])` e `aria-checked` ancora
  `["false", "false"]`. Misurata (P4): verde sul codice; rossa su X7, `expected [ 'false', 'true' ] to deeply equal
  [ 'false', 'false' ]`.

#### M-5 — lo slot `trigger` e i tre segni `data-*`: comportamenti del contratto senza una prova — voce candidata **E21**, parti (c) e (d)

- **(c) lo slot `trigger`.** Il commento di `BaseDialog.vue`, riga 9: *«unbound the `trigger` slot opens it»*; riga 21,
  `<DialogTrigger v-if="$slots.trigger" as-child>`. **X3**, `v-if="false"` → `Tests  23 passed (23)`. I compiti 4, 5 e 8
  lo usano: `grep -n '#trigger'` sul piano → righe 3415, 4133, 7525. Prova proposta (P5): montata senza `open`, col
  `trigger` che è un `BaseButton`; nessun `[role="dialog"]`; un clic sul pulsante, `nextTick`, e c'è. Verde sul codice;
  rossa su X3, `Unable to get button within: <!--v-if-->`.
- **(d) i tre `|| undefined`.** `BaseButton.vue` righe 42–43, `:data-pill="pill || undefined"` e
  `:data-icon-only="iconOnly() || undefined"`; `BaseTextField.vue` riga 26, `:data-disabled="disabled || undefined"`.
  Senza, Vue 3.5.42 scrive la stringa `"false"` — `patchAttr` in `node_modules/@vue/runtime-dom/dist/runtime-dom.cjs.js`,
  righe 564–581, toglie l'attributo solo per `null` e `undefined` —, e i selettori `[data-icon-only]`, `[data-pill]`,
  `[data-disabled]` la prendono: ogni pulsante con le parole quadrato e senza padding, ogni pulsante una pillola, ogni
  campo spento. **X4**, **X4b**, **X5** → `Tests  23 passed (23)` ciascuna (X4b e X5 misurate con la sola prova
  proposta). Righe proposte (P2, P3): in «takes `label` as the name of an icon alone» (riga 63), per `worded`,
  `expect(worded.attributes("data-icon-only")).toBeUndefined();` e `expect(worded.attributes("data-pill")).toBeUndefined();`;
  nella prova di `BaseTextField`, prima di `setValue`, `expect(wrapper.get(".frame").attributes("data-disabled")).toBeUndefined();`.
  Verdi sul codice; rosse su X4, X4b e X5, `expected 'false' to be undefined`.

Il file usa-e-getta e le mutazioni: `probes.py` e `zz-review.test.ts` nello scratchpad, log in `probes.log` e
`probes-logs/`; ogni file tornato dalla copia, `cmp` uguale, il file usa-e-getta tolto, `git status --porcelain` vuoto
prima e dopo.

#### M-6 — `compare_task3.py`: il lockfile, oltre l'appuntamento, non è né contato né mostrato

- **Dove.** `grep -n 'the pin; the rest comes from the registry' docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task3.py`
  → riga 156.
- **Evidenza.** Nel clone, un commit sopra `c7b7bcd` che cambia la versione di **un'altra** voce del lockfile —
  `"node_modules/magic-string"` da `0.30.21` a `0.30.22` —, `a1398d4`: `OK             gui/package-lock.json  (the pin;
  the rest comes from the registry -- CHECK BY HAND)`, `--- 16 paths, 0 not matching the plan's text`, uscita **0**, e
  nessuna riga del cambio stampata. Il docstring promette *«what is dictated COUNTS, what is not is SHOWN»*: qui il non
  dettato non si vede, e la riga dice `OK`. Una deriva del lockfile — il vincolo 8 — passerebbe muta.
- **L'altra direzione tiene.** Un carattere cambiato in un commento di `BaseStatus.vue` — *«regiom»* per *«region»* —,
  `3624dbf`: `DIFFERS        gui/src/components/BaseStatus.vue`, il diff di quella riga, gli altri quindici `OK`,
  `--- 16 paths, 1 not matching`, uscita **1**.
- **A mano, sul commit vero.** `git diff --numstat ef3e865 c7b7bcd -- gui/package-lock.json` → `7	0`: la riga fra le
  dipendenze della radice e la voce `node_modules/lucide` — `version` `1.47.0`, `resolved` dal registro, `integrity`,
  `license` `ISC`, nessun `hasInstallScript` —, e nient'altro.
- **Cura proposta**, nello script — un attrezzo della cartella tracciata, non una voce del piano: il lockfile atteso è
  quello della base più le due aggiunte dettate; le voci di `packages` fuori da `node_modules/lucide` si confrontano una
  per una con la base, e una diversa è `CHECK BY HAND` con la sua chiave stampata; la voce di `lucide` è `DIFFERS` se
  `version` non è `1.47.0`, se `license` non è `ISC` o se porta `hasInstallScript`; `OK` solo quando nient'altro cambia.
  Nello stesso modo `compare_task4.py` e i seguenti, se un compito aggiunge un pacchetto.

### Nit

#### N-1 — `eslint.config.js`, riga 36: «WITHOUT IT SIX OF THE THIRTEEN DO NOT PARSE» — voce candidata **E22**, parte (a)

- **La lezione (b).** `grep -n 'SIX OF THE THIRTEEN' gui/eslint.config.js` → riga 36. La frase conta i `.vue` di una
  cartella che il compito fa crescere. Misurato nel clone: tolto il `parser` dal blocco `harness/ts-in-vue`
  (`languageOptions: {}`), `npx eslint src --format json` → 21 `.vue` letti, **13** che non si analizzano — i sei della
  frase, `Chat`, `Confirm`, `Frame`, `Placeholder`, `Settings`, `ViewBar`, e sette `Base*.vue`: tutti tranne
  `BaseStatus.vue`, che nello script ha solo un commento. Il file tornato dalla copia, `cmp` uguale. È la specie di
  **E17**, nello stesso file: E17 ha curato la frase delle parole singole, e questa è rimasta.
- **Cura proposta**: una sesta sostituzione nel Passo 7, sulla sola riga di testa. *Trova*
  `     * ⛔ THE TypeScript PARSER FOR THE `.vue` FILES, AND WITHOUT IT SIX OF THE THIRTEEN DO NOT PARSE.` — *Sostituisci
  con* `     * ⛔ THE TypeScript PARSER FOR THE `.vue` FILES: WITHOUT IT A `<script setup lang="ts">` DOES NOT PARSE.`
  La misura datata delle righe dopo — *«measured on 2026-09-16 on the thirteen `.vue` of this plan»* — resta vera com'è.

#### N-2 — `eslint.config.js`, riga 52: `reka-ui` nominato prima della sua regola — voce candidata **E22**, parte (b)

- Il blocco `harness/ts` dice che le regole di sotto devono vedere i `.ts`, *«or `lucide` and `reka-ui` could come in
  through a `.ts` unseen»*; in questo commit nessuna regola parla di `reka-ui` — `git show c7b7bcd:gui/eslint.config.js |
  grep -c 'name: "reka-ui"'` → `0` —: arriva col compito 5, il blocco `harness/panels-and-frame`, riga 4554 del piano.
  Per due compiti il commento descrive una regola che non c'è (gotcha #58).
- **Cura proposta**, nel recinto della seconda sostituzione del Passo 7 (riga 2833 del piano): *«… or `lucide` -- and,
  from task 5, `reka-ui` -- could come in through a `.ts` unseen.»*

#### N-3 — il confine della regola di `lucide`: un sotto-percorso passa il linter — voce candidata **E23**

- **La lezione (c).** `paths: [LUCIDE]` (righe 120, 130 e 135) prende il nome esatto `lucide`. Misurato nel clone (X6):
  `import search from "lucide/dist/esm/icons/search.mjs";` in `frame/moveActive.ts` → `npm run lint` esce **0**. Lo
  ferma il *build*: `npx vue-tsc --noEmit` → `src/frame/moveActive.ts(2,20): error TS7016: Could not find a declaration
  file for module 'lucide/dist/esm/icons/search.mjs'`, uscita 2 — il pacchetto non ha `exports` e porta un `.d.ts` solo,
  `find node_modules/lucide/dist -name '*.d.ts'` → `dist/lucide.d.ts`. Il file tornato dalla copia, `cmp` uguale. Oggi la
  porta regge per il compilatore, non per il linter: una dichiarazione scritta a mano la aprirebbe.
- **Cura proposta**, in A/B:

  | | Che cosa | Costo |
  |---|---|---|
  | **A** — consigliata | dichiararlo nel ⚠️ del Passo 8, accanto all'`import()` dinamico (R2-17): *«e un import da un sotto-percorso, `lucide/…`, passa la regola: oggi lo ferma `vue-tsc`, TS7016, finché nessuno gli scrive una dichiarazione»* | una frase |
  | **B** | prenderlo nella regola: un `patterns` `{ regex: "^lucide/" }` accanto a `paths: [LUCIDE]` | tre blocchi che ripetono la lista, e una riga rossa in più nel Passo 8 |

#### N-4 — `BaseButton`: una carta con una misura prende l'altezza di un controllo

- Le regole `[data-size="sm"]` e `[data-size="lg"]` (righe 132 e 136) vengono dopo quella del `card` (riga 112), con lo
  stesso peso, e rimettono `height`. Misurato nella prova usa-e-getta, una carta con due righe di testo: alta 74 px col
  contenuto di 72; con `size="lg"` 40 px col contenuto di 48; con `size="sm"` 24 px col contenuto di 48 — il testo esce.
- Nessun compito la usa: `grep -n 'variant="card"'` sul piano → le carte della pagina kit (righe 3429 e 3433) e della
  Panoramica (7534, 7565), senza misura; l'unica combinazione è l'esemplare di `kit.test.ts`, riga 86, che guarda gli
  attributi e non disegna. **Registrata, non presa**: se un compito ne userà una, la cura è sua — le misure con
  `:not([data-variant="card"])`, o una frase nel commento del pezzo.

#### N-5 — il rapporto: il riferimento futuro del Passo 3 che propone è troppo stretto

- Il §8 del rapporto propone *«un `Failed to resolve import` di un `./Base*.vue`»*. Rifatto il rosso tre volte nel clone,
  coi nove file tolti: `./icons`, `./BaseButton.vue`, `./BaseStatus.vue` — il primo non è un `Base*.vue`. Il fatto che
  il rapporto dice, *il file nominato cambia da una corsa all'altra*, torna.
- **Cura**: *«un `Failed to resolve import` di uno dei nove file che il Passo 3 non ha ancora — `./icons` o un
  `./Base*.vue`»*.

#### N-6 — una misura fra il cancello verde e il commit

- `ls -la --time-style=full-iso` nello scratchpad `task3`: il cancello finale finisce alle 13:15:06; `idle-old.txt` e
  `idle-new.txt` nascono alle 13:15:54, `tsc-idle.log` alle 13:16:14; il commit è delle 13:18:10. La misura in più su
  `@ts-expect-error` ha cambiato `kit.test.ts` **dopo** il verde. È tornato dalla copia, `cmp` lo conferma, e
  `compare_task3.py` dice il file committato uguale al recinto: il commit è ciò che il cancello ha visto.
- **Cura**: nessuna sul commit; l'ordine che la regola vuole è misure, cancello, commit.

---

## 3. I comandi rilanciati

Sull'**albero del repository**, in sola lettura, salvo il cancello e `check-docs.sh`; le mutazioni nel **clone**.

| # | Comando | La mia uscita | Il rapporto |
|---|---|---|---|
| 1 | `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline ef3e865..HEAD`; `git status -sb` | `c7b7bcd`; vuoto; una riga; `## main...origin/main [ahead 1]` | — |
| 2 | `git log -1 --format=%ci c7b7bcd`; `ls -la --time-style=full-iso` sui log | `2026-09-25 13:18:10 +0200`; `gate-final-2026-09-25.log` 13:15:06, `gate-opening-2026-09-25.log` 12:55:23 — dell'implementatore —, `gate-baseline.log` del coordinatore 12:37:45 | i due log |
| 3 | `git show --stat c7b7bcd`; `git diff --name-status ef3e865 c7b7bcd` | sedici file, `991 insertions(+), 19 deletions(-)`; 5 `M`, 11 `A` | uguale |
| 4 | `PYTHONIOENCODING=utf-8 python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task3.py ef3e865 c7b7bcd` | sedici `OK`, `--- 16 paths, 0 not matching the plan's text; date 2026-09-25`, uscita 0 | — |
| 5 | le 25 righe della ricetta contro il piano a `ef3e865` | ciascuna apre il recinto giusto, sotto il suo *«Crea …»*, *Trova* o *Sostituisci con* | — |
| 6 | il compito 3 del brief contro `## Compito 3:`…`## Compito 4:` del piano a `ef3e865` | uguali, a meno di una riga vuota in coda | — |
| 7 | `diff` fra il recinto di `replace_unique.py` del brief e la copia dell'implementatore | identici | identico |
| 8 | `git diff ef3e865 c7b7bcd -- <piano>` | due righe: la 2 con `` `23b3134`, con la cura `5f32158` ``, la 3 a `✅ 2026-09-25` con la colonna Commit `—` | uguale |
| 9 | `git diff ef3e865 c7b7bcd -- gui/package.json gui/package-lock.json` | `"lucide": "1.47.0"` fra `dockview-core` e `markdown-it`; nel lockfile la riga della radice e la voce con `license` `ISC`, senza `hasInstallScript` | uguale |
| 10 | nel clone: `npm ls lucide`; `npm view lucide@1.47.0 license scripts.postinstall scripts.install scripts.preinstall` | `` `-- lucide@1.47.0 ``; `ISC`, e nessuno script d'installazione | uguale |
| 11 | `node_modules/lucide/package.json` e `LICENSE`, nel clone | `scripts` di *build* e di prova; nessun `dependencies`; `LICENSE`: riga 1 `ISC License`, riga 19 le icone da Feather, riga 23 `The MIT License (MIT) (for the icons listed above)`. Delle 24 icone di `icons.ts`, la sola `search` sta nell'elenco di Feather (riga 21, 115 nomi) | uguale |
| 12 | `git ls-files --eol` e `tr -cd '\r' \| wc -c` contro `wc -l`, i sedici file, sull'albero | il piano, il manifesto e il lockfile `i/lf w/crlf`, CR uguali alle righe (8656, 44, 5083); gli altri tredici `i/lf w/lf`, zero CR; nell'indice, a `ef3e865` e a `c7b7bcd`, zero CR | uguale, §6 |
| 13 | `git diff --stat ef3e865..c7b7bcd -- crates/ gui/schema/`; lo stesso su `.github/ scripts/ docs/adr/ Cargo.toml Cargo.lock` | vuoto; vuoto | vuoto |
| 14 | `git log -1 --format=%B c7b7bcd \| grep -ci co-authored`; `git log -1 --format=%s c7b7bcd` | `0`; comincia con `design-system(compito 3): il kit` | uguale |
| 15 | il Passo 3 nel clone: tolti `icons.ts` e gli otto `Base*.vue`, copiati prima; `npx vitest run src/components/kit.test.ts`, tre volte | rosso tre volte, `Tests  no tests`: `Failed to resolve import "./icons"`, poi `"./BaseButton.vue"`, poi `"./BaseStatus.vue"`; i nove file tornati, `cmp` uguale, `git status --porcelain` uguale a prima | `./BaseDialog.vue`, e tre corse diverse |
| 16 | il Passo 8, prima tabella, nel clone, una violazione per corsa, `npm run lint` | le sei righe rosse per la ragione scritta, `✖ 1 problem (1 error, 0 warnings)` ciascuna: `4:1` e `28:1` *«a base piece reads no global state …»*; `2:1` e `6:1` *«'lucide' import is restricted … icons pass through BaseIcon …»*; `7:1` e `2:1` *«the kit knows no layer above it …»*; la riga verde esce 0 | uguale, righe e colonne comprese |
| 17 | il Passo 8, seconda tabella, `npx vitest run --project jsdom src/components/kit.test.ts` | `expected [ 'models' ] to deeply equal []` (`kit.test.ts:44:95`); `Unable to get [role="status"] within: <!--v-if-->` (`98:28`); `expected true to be false` a `142:57` (E15) e a `196:54` (E16), con l'avviso di `reka-ui`; `Tests  1 failed \| 22 passed (23)` ciascuna; `v-if="$slots.default"` → `Tests  23 passed (23)`; ogni file tornato, `cmp` uguale; `git status --porcelain` vuoto prima e dopo | uguale |
| 18 | `npx eslint src --format json`, contato per estensione, nel clone a `ef3e865` e a `c7b7bcd`; `find src -name '*.ts' \| wc -l` | 17 file, `{'.json': 4, '.vue': 13}`, 0 errori; poi 73 file, `{'.json': 4, '.ts': 48, '.vue': 21}`, 0 errori, fra i `.ts` anche `src/browser.d.ts`; 45, poi 48 | uguale |
| 19 | il Passo 6 nel clone: `npx vitest run src/components`, `npm run build` | `Test Files  2 passed (2)`, `Tests  28 passed (28)`; `dist/assets/index-DLsd9Y_U.js  663.93 kB`, `index-D7FEpuBn.css  145.10 kB` | uguale |
| 20 | `vue-tsc` sulla riga `@ts-expect-error`, nel clone, le due direzioni | con `"search"`: `kit.test.ts(50,5): error TS2578: Unused '@ts-expect-error' directive.`, uscita 2; senza la direttiva: `kit.test.ts(50,11): error TS2322: Type '"not-an-icon"' is not assignable …`, uscita 2; il file tornato, `cmp` uguale | la prima delle due |
| 21 | i token dei pezzi: `grep -oh 'var(--[a-z0-9-]*' components/Base*.vue`, e per ciascuno `grep -c -- "<nome>:"` su `tokens/base.css` e `tokens/themes.css` | `names: 53 missing: 0` | uguale |
| 22 | `git grep` a `c7b7bcd`: `axe-core`, `testing/axe`, `from "lucide`, `var(--ref-` fuori da `tokens/`, i colori a mano in `components/` e `testing/` | `axe-core` solo in `testing/axe.ts`; `testing/axe` in `a11y.test.ts` e `kit.test.ts`; `lucide` solo in `icons.ts`; nessuno; nessuno — il solo `transparent`, che la tavola usa uguale (righe 307, 312, 317, 320) | — |
| 23 | **il cancello intero**, `bash scripts/gate.sh`, sull'albero, da solo, in background, verso `review3/gate-review-c7b7bcd.log`, 13:26:41 → 13:32:48 | `GATE GREEN.`, uscita 0; `added 341 packages in 1m`; `dist/assets/index-DLsd9Y_U.js  663.93 kB`; `--project jsdom`: `Test Files  18 passed \| 1 skipped (19)`, `Tests  123 passed \| 1 skipped (124)`; `--project browser`: `Test Files  1 passed (1)`, `Tests  5 passed (5)`; `found 0 vulnerabilities` | uguale; la base del coordinatore: 17+1 file, 100+1 prove, 1 e 5, lo stesso pezzo |
| 24 | nessuna prova di prima sparita, **per nome**: `npx vitest run --project jsdom` col rapporto JSON nel clone, a `ef3e865` e a `c7b7bcd`, confrontati file per file e prova per prova (`cmp_json.py`) | 18 file e 101 prove, poi 19 e 124; nessun file sparito, nessuna prova sparita o cambiata di stato; nuovo il solo `src/components/kit.test.ts`, 23 prove | — |
| 25 | `bash scripts/check-docs.sh`, sull'albero, dopo il cancello | `OK — no inconsistencies.`; `git status --porcelain` vuoto | uguale |
| 26 | `compare_task3.py` nelle due direzioni, nel clone | M-6 | — |
| 27 | le sonde mie, X1–X7, e le righe proposte, P1–P5, nel clone | M-3, M-4, M-5, N-3 | — |
| 28 | la prova usa-e-getta nel Chrome installato, e la stessa con le due cure | M-1, M-2, N-4 | — |
| 29 | nel clone, tolto il `parser` di `harness/ts-in-vue`, `npx eslint src --format json` | N-1 | — |
| 30 | il Passo 9 in locale, nel clone: `npm test` alle 14:02; `npm test -- --maxWorkers=8` alle 14:25 e alle 14:29 | rosso, rosso, **verde**: le prime due per l'ambiente (§4); la terza, su Chrome 154, `Test Files  19 passed \| 1 skipped (20)`, `Tests  128 passed \| 1 skipped (129)`, uscita 0 (`step9-npm-test-maxworkers8-b.log`) | `19 passed \| 1 skipped (20)`, `128 passed \| 1 skipped (129)` |

---

## 4. Ciò che non ho potuto verificare, e perché

| Che cosa | Perché |
|---|---|
| `npm test` del Passo 9 **alla lettera**, i due progetti in una corsa coi processi di prova di serie | rilanciato nel clone alle 14:02: **rosso per l'ambiente** — `Error: Failed to connect to the browser session "…" [browser (chromium)] within the timeout`, diciannove `Timeout terminating forks worker`, `Test Files  18 passed \| 1 skipped (20)`, `Tests  123 passed \| 1 skipped (124)`, `Errors  1 error` (`step9-npm-test.log`) — e verso le 14:05 il processo di Claude Code si è chiuso. La causa non l'ho separata: la memoria di P-21 (voce aperta del proprietario) o il Chrome a metà aggiornamento, qui sotto. Non l'ho rilanciato così: con `--maxWorkers=8`, che P-21 misura sopra 2,7 GB liberi, alle 14:29 è **verde** coi numeri del rapporto (riga 30) |
| il Chrome delle prove, adesso | ⚠️ **è cambiato durante la revisione.** La seconda corsa, `npm test -- --maxWorkers=8` alle 14:25, è caduta con `Error: browserType.launch: Target page, context or browser has been closed`: Chrome lanciato (`pid=13408`) e uscito subito con `exitCode=0` (`step9-npm-test-maxworkers8.log`). Nello stesso minuto la cartella `C:\Program Files\Google\Chrome\Application\` mostra l'aggiornamento compiuto — `ls -la` e `VersionInfo.ProductVersion`: la sola cartella `154.0.8037.58`, niente più `new_chrome.exe`, `SetupMetrics` alle 14:25:33, `chrome.exe` `154.0.8037.58`. Il mio cancello (13:26–13:32) e le mie prove nel browser fino alle 13:57 hanno girato su 153 — `HeadlessChrome/153.0.0.0` nel `userAgent` delle 13:50 —; la corsa verde delle 14:29 su 154. Il §0 del mandato — *153, con la 154 scaricata* — da oggi non vale più su `zagor` |
| la CI | è del coordinatore, dopo il push |
| i pezzi a occhio, nei due temi | la pagina kit è del compito 4; qui solo le misure del browser di M-1, M-2 e N-4 |
| l'Assistente vocale su `BaseStatus` | del compito 5, controllo 13 |
| le colonne `w/…` di **prima**, sull'albero | l'albero è a `c7b7bcd`: di `ef3e865` ho l'indice (`i/lf`, zero CR) e il §0 del mandato; il rapporto dell'implementatore e l'albero di adesso coincidono |
| la macchina `Jays` | non l'ho vista |
| le tre corse del rosso del Passo 3 dell'implementatore | nello scratchpad `task3` c'è il log della prima sola; il fatto che dicono — il nome cambia — l'ho rifatto (riga 15) |

⚠️ **Fuori dal compito, per il coordinatore.** Nel mio cancello il passo `dependency advisories` di Rust stampa 39 volte
`error: couldn't check if the package is yanked: registry: request could not be completed in the allotted timeframe`, e
il cancello resta verde; nei tre log di oggi prima del mio — `gate-baseline.log`, `gate-opening-2026-09-25.log`,
`gate-final-2026-09-25.log` — zero (`grep -c`). Il registro non rispondeva: il controllo dei pacchetti ritirati, quando
non raggiunge il registro, non fa rosso. Non tocca `c7b7bcd`.

---

## 5. Come l'ho fatto, e che cosa resta

- Letti per intero: il mandato, il prompt dell'implementatore, il suo rapporto, il brief (1640 righe, a blocchi) e
  `compare_task3.py`; del piano a `ef3e865` solo il *«Come si riprende»* con la ricetta, i recinti che la ricetta nomina,
  le righe dei compiti 4, 5 e 8 che usano i pezzi, e il blocco del linter del compito 5.
- Nel clone, ogni mutazione con una copia salvata prima e la copia rimessa dopo, `cmp` uguale; alla ripresa, i 26 file
  delle copie uguali al clone, e i file del compito — più `Confirm.vue`, `markdown.ts` e `frame/moveActive.ts` — uguali a
  `git show c7b7bcd:<file>` coi CR di questa macchina; `git status --porcelain` vuoto. Nel clone restano due commit miei
  staccati, `3624dbf` e `a1398d4`, le due direzioni di M-6: il clone lo cancella il coordinatore.
- ⚠️ **Un errore mio, fuori dal mandato.** Per leggere la versione di Chrome ho lanciato `chrome.exe --version`, che su
  Windows non stampa la versione ma **apre il browser**: alle 14:26:19 una finestra di Chrome 154, col profilo
  dell'utente, *«New Tab - Google Chrome»*, sette processi. L'ho chiusa con `CloseMainWindow()` sul processo principale
  — nessun'altra istanza di Chrome era aperta, lo dicono le ore d'avvio, tutte 14:26:19 — e dopo `Get-Process chrome`
  ne conta 0. La versione poi l'ho letta dai metadati del file. Nessun file del repository o del clone ne è toccato.
- Gli attrezzi e i log nello scratchpad `review3`: `step8.py`, `probes.py`, `mutate.py`, `cmp_json.py`,
  `zz-review.test.ts`, `zz-review.browser.test.ts`; `gate-review-c7b7bcd.log`, `check-docs-review.log`,
  `step8-tables.log`, `step8-own.log`, `step8-own-x7.log`, `step8-logs/`, `probes.log`, `probes-logs/`,
  `browser-probe.log`, `browser-probe-fixed.log`, `step3-red-{1,2,3}.log`, `jsdom-{ef3e865,c7b7bcd}.{log,json}`,
  `eslint-{ef3e865,c7b7bcd,noparser}.json`, `step6-{tests,build}.log`, `tsc-idle.log`, `tsc-nodirective.log`,
  `step9-npm-test.log`, `step9-npm-test-maxworkers8.log`, `step9-npm-test-maxworkers8-b.log`.
