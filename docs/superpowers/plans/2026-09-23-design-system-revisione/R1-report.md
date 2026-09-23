# Rapporto R1 — la testa (righe 1–218) e il compito 1 (righe 219–1172)

Revisore R1, 2026-09-23, HEAD `d589d15`, in sola lettura sul repository. Prove in
`C:\Users\zagor\AppData\Local\Temp\probe-ds1` (copia di `gui/` e `scripts/gate-gui.sh`, `npm ci` fatto).

## Rilievi

### R1-1 — Importante — la terza prova del negozio è verde prima del codice, e non la si vede mai rossa
- Dove: piano righe 625–628 (compito 1, passo 7), 656–658 (passo 8), 1137–1144 (passo 16)
- Che cosa dice il piano: passo 8, *«Atteso: rosso … e le tre prove del negozio senza `theme` e `chooseTheme`»*; passo 16,
  la tabella delle violazioni non ha righe per `stores.test.ts`.
- Che cosa è vero: la prova *«reads a choice it does not know as absent, without refusing the package»* è **verde** al
  passo 8: l'`unpack` di oggi non legge `theme` affatto, quindi rende già `{ view: "home", layouts: {} }`. Rosse sono due
  prove su tre. E nessun passo la fa mai diventare rossa: il vincolo 11 (*«ogni prova si prova nelle due direzioni»*) resta
  scoperto proprio sulla regola *«una scelta sconosciuta si legge come assente»*.
- Prova: nella cartella di prova, passi 3–7 applicati alla lettera, `npx vitest run src/tokens src/stores` →
  `× keeps the choice …`, `× sends the choice at once …` e nient'altro del negozio; `npx vitest run src/stores -t "reads a
  choice"` → `Tests 1 passed | 22 skipped`. La violazione che la fa rossa esiste ed è stata provata: in `unpack`
  `return { view: candidate.view, layouts, theme: theme as ThemeChoice };` → `× reads a choice it does not know as absent`,
  `+ "theme": "purple"`. Anche la seconda violazione utile è stata provata: `settle` senza `...(saved.value ?? {})` →
  `× sends the choice at once, and a settle after it keeps it`.
- Correzione proposta: al passo 8, *«… e due delle tre prove del negozio — la prima e la terza — senza `theme` e
  `chooseTheme`; la seconda, la scelta sconosciuta, è verde già oggi perché `unpack` non legge `theme`: la sua direzione
  rossa è al passo 16»*. Al passo 16 due righe: `stores.test.ts`, *«in `unpack` il ramo che rende `theme` senza
  `isThemeChoice`»* → *«rosso: `reads a choice it does not know as absent`, `"theme": "purple"`»*; e *«in `settle` tolto
  `...(saved.value ?? {})`»* → *«rosso: `sends the choice at once, and a settle after it keeps it`»*.

### R1-2 — Minore — al passo 8 `usage.test.ts` è rosso anche per i due veli, e il piano non lo dice
- Dove: piano righe 656–658 (compito 1, passo 8)
- Che cosa dice il piano: *«`usage.test.ts` senza `tokens/dock.css`»*.
- Che cosa è vero: rosse sono due prove — *«sees the files it judges»* (manca `tokens/dock.css`) **e** *«writes no colour by
  hand outside the token files»*, per `components/Confirm.vue:49: background: rgb(0 0 0 / 50%);` e
  `frame/Drawer.vue:38: background: rgb(0 0 0 / 0.45);`. Il secondo rosso resta fino al passo 13. Per la regola del passo 8
  (*«un rosso per un'altra ragione è una voce d'errata»*) un esecutore stretto si fermerebbe.
- Prova: log del passo 8 nella cartella di prova → `AssertionError: expected [ …(2) ] to deeply equal []` con le due righe
  dei veli.
- Correzione proposta: *«`usage.test.ts` senza `tokens/dock.css`, e coi due veli di oggi — `Confirm.vue` e `Drawer.vue` —
  che il passo 13 rinomina»*.

### R1-3 — Minore — al passo 13 il censimento trova ancora dieci nomi vecchi: `tokens.css` esce solo al passo 14
- Dove: piano righe 1096–1101 (compito 1, passo 13)
- Che cosa dice il piano: *«poi il censimento non trova più nessun nome vecchio né nessun colore a mano»*.
- Che cosa è vero: dopo `rename_tokens.py` il censimento rende ancora le dieci righe di `gui/src/tokens/tokens.css`
  (`:145 var(--surface-raised)`, `:147 var(--line)`, … `:70 var(--accent)`), perché il file si toglie al passo 14. Nei
  `.vue` non resta niente, e i due `z-index` dicono `var(--z-overlay)`: quella parte è vera.
- Prova: `bash census-1.sh` nella cartella di prova subito dopo il passo 13 → dieci righe `gui/src/tokens/tokens.css:…`, poi
  `gui/src/frame/Drawer.vue:39:  z-index: var(--z-overlay);` e `:44`.
- Correzione proposta: *«… non trova più nessun nome vecchio **fuori da `tokens.css`**, che esce al passo 14»*; oppure
  rilanciare il censimento dopo il `git rm` del passo 14, dove rende le sole due righe dei `z-index`.

### R1-4 — Minore — al passo 2 `npm` riordina le dipendenze: il diff non è «le due righe nuove»
- Dove: piano righe 267–276 (compito 1, passo 2)
- Che cosa dice il piano: *«Atteso: le due righe nuove in `dependencies` di `package.json`, esatte»*.
- Che cosa è vero: oggi `dockview-core` sta **prima** di `dockview`; `npm install --save-exact` riscrive le dipendenze in
  ordine e lo sposta dopo. Il diff di `package.json` è `4 +++-`: le due righe nuove più `dockview-core` spostato. I CR sono
  conservati (`package.json` 39 → 41 CR su 41 righe; lockfile 4917 CR su 4917 righe), e `npm ls` rende le due versioni.
- Prova: `git diff --stat package.json package-lock.json` → `gui/package-lock.json | 20 ++++`, `gui/package.json | 4 +++-`;
  `git diff package.json` → `-    "dockview-core": "8.3.1",` … `+    "dockview-core": "8.3.1",`.
- Correzione proposta: *«Atteso: le due righe nuove, esatte, e `dockview-core` che `npm` rimette in ordine dopo
  `dockview` (`4 +++-`); il lockfile che le porta (`20 +`)»*.

### R1-5 — Minore — al passo 15 `grep 'kB'` non rende «la riga del pezzo JavaScript», e i caratteri non sono solo `.woff2`
- Dove: piano righe 1129–1131 (compito 1, passo 15)
- Che cosa dice il piano: *«la riga del pezzo JavaScript — `npm run build 2>&1 | grep 'kB'`»*; *«i caratteri sono file a
  parte, `dist/assets/*.woff2`»*.
- Che cosa è vero: `grep 'kB'` rende 32 righe, una per ogni file di carattere; la riga del JavaScript è una fra queste. In
  `dist/assets/` ci sono **17 `.woff2` e 12 `.woff`** (Barlow porta anche il `.woff`). La misura: JS `663.93 kB │ gzip:
  201.53 kB`, contro i 663,26 / 201,23 della baseline scritta nel disegno (misura 2) — la deduzione della (e) regge, +0,67 kB;
  CSS `144.25 kB`.
- Prova: `npm run build 2>&1 | grep 'kB' | grep -E '\.(js|css) '` → le due righe; `ls gui/dist/assets | sed 's/.*\.//' |
  sort | uniq -c` → `1 css`, `1 js`, `12 woff`, `17 woff2`.
- Correzione proposta: `npm run build 2>&1 | grep -E 'assets/index-.*\.js '`; e *«i caratteri sono file a parte,
  `dist/assets/*.woff2` e `*.woff`»*.

### R1-6 — Minore — la tabella della posizione dice «dodici componenti»: sono undici (confermato), e il disegno ha lo stesso errore
- Dove: piano riga 127 (tabella *«A che punto è»*, riga 1); disegno riga 590 (decisione 14)
- Che cosa dice il piano: *«i nomi nuovi nei dodici componenti»*; il compito (righe 230–231, 260–261, 1101, 1168) dice
  **undici**.
- Che cosa è vero: 13 `.vue`; 12 con `<style scoped>` (`App.vue` no); 11 usano `var(--`; il dodicesimo, `Frame.vue`, ha lo
  stile scoped ma nessun token. I nomi vecchi stanno in 11 componenti — il censimento del passo 1 li elenca identici.
- Prova: `grep -rlE 'var\(--' gui/src --include='*.vue' | wc -l` → `11`; `find gui/src -name '*.vue' | wc -l` → 13;
  `grep -rL '<style scoped' gui/src --include=*.vue` → `gui/src/App.vue`; `Frame.vue` è l'unico scoped senza `var(--`.
- Correzione proposta: riga 127, *«i nomi nuovi negli undici componenti che li usano»*. La decisione 14 del disegno (*«il
  piano rinomina a macchina gli usi di oggi nei dodici componenti»*) è un documento approvato: si segnala, non si tocca.
- Altre della stessa specie nella testa: rilette tutte le cifre (otto pezzi, 22 risposte, dieci righe *«proposta»*,
  decisioni 23–27, precisioni 4–11, 176/88/sei/undici di P-1, 103 e nove di P-6, 14 della roadmap, 2.10.5 di `reka-ui`,
  le versioni della *«Pila»*): **nessun'altra sbagliata**. Vedi *«Verificato e giusto»*.

### R1-7 — Minore — vincolo 9: «i file di `gui/` sono CRLF nell'albero» vale per la metà
- Dove: piano riga 109 (vincolo 9)
- Che cosa dice il piano: *«i file di `gui/` sono `i/lf` nell'indice e, su questa macchina, CRLF nell'albero»*.
- Che cosa è vero: su 95 file di `gui/`, **41 `w/crlf`, 40 `w/lf`**, 14 `-text` (i `.bin` di `schema/fixtures`). Dei file
  del compito 1 sono **LF** `contrast.test.ts`, `Confirm.vue`, `Chat.vue`, `Permissions.vue`, `Settings.vue`, `Status.vue`,
  `Steps.vue` (e `a11y.test.ts`); CRLF `App.vue`, `main.ts`, `layout.ts`, `stores.test.ts`, `Band`, `Drawer`, `ViewBar`,
  `Placeholder`, `Strip`, `package.json`, il lockfile. Innocuo per l'esecuzione: `replace_unique.py`, la regola del «file
  riscritto per intero» e `rename_tokens.py` conservano i fine-riga **del file** — provato, CR = righe su ogni CRLF e 0 su
  ogni LF dopo i passi 2–14.
- Prova: `git ls-files --eol gui | awk '{print $2}' | sort | uniq -c` → `14 w/-text`, `41 w/crlf`, `40 w/lf`.
- Correzione proposta: *«i file di `gui/` sono `i/lf` nell'indice e, su questa macchina, **misti nell'albero** — CRLF o LF
  per file (`git ls-files --eol`)»*. La stessa frase sta nei vincoli comuni dei revisori.

### R1-8 — Importante, da segnalare al proprietario — il controllo 2, approvato nella (f), cambia forma senza un sì scritto
- Dove: piano riga 172 (P-1), righe 382–420 (compito 1, passo 4); disegno riga 449 (controllo 2, colonna *«Da»* = **(f)**),
  righe 178–180 e 197; `themes.css` della tavola, riga 163–164 di `token.html`
- Che cosa dice il piano: P-1 — la regola *«ogni testo su ogni fondo»* *«fallisce per costruzione»*, e il compito 1
  giudica per **famiglie** con **nove** esenti.
- Che cosa è vero: P-1 ha ragione sul fatto (nello scuro `--color-text-on-ok` e `--color-bg` sono entrambi
  `var(--ref-neutral-5)`, `token.html` righe 175 e 199), e le famiglie riproducono esattamente le coppie di `palette.py`
  (righe 92–104: 66 + 3 + 3 + 16 = 88 per tema) più una, `--color-text` su `--color-bg-selection`, che passa. **Ma** il
  controllo 2 è marcato **(f)**, approvato dal proprietario, e dice *«ogni `--color-text-*` su ogni `--color-bg-*` … le due
  esenzioni scritte accanto»*; il piano ne porta **nove** (`color-text-disabled`, `color-border`, `color-border-card`,
  `color-border-ok`, `-warn`, `-stop`, `color-veil`, `shadow-card`, `shadow-overlay`). La scelta **A** della voce 1 copre le
  righe *«proposta»*, le decisioni 23–27 e le precisioni 4–11: non un controllo **(f)** riscritto. Il vincolo 1 del piano
  stesso dice *«ci si ferma e lo si riporta»*. In più `themes.css`, copiato byte per byte, porta nel suo commento di testa
  *«Every text role reads 4.5:1 on every background role»*, che P-1 dimostra falso: lo si può correggere solo nella tavola,
  che è del proprietario (D1: *«chi cambia un valore cambia prima la tavola»*).
- Prova: `sed -n '85,110p' …\9ae312b8-…\scratchpad\palette.py` (la funzione `pairs()`); `awk` sui blocchi dei due temi di
  `token.html`: 41 ruoli per tema, e il test del piano, applicato, è verde con 7 prove (`Tests 9 passed` insieme a
  `board.test.ts`); `grep -n 'Every text role' docs/superpowers/specs/2026-09-22-design-system-tavole/token.html` → riga 163.
- Correzione proposta: **da segnalare al proprietario**, in A/B: A — il controllo 2 si scrive per famiglie, com'è nel
  compito (consiglio: sono le 176 coppie che ha approvato con la tavola); B — resta *«ogni testo su ogni fondo»*, e allora la
  tavola cambia (un valore di `text-on-*`). Con A, una riga nella tavola corregge il commento di `themes.css` e la frase della
  (a) righe 178–180 riceve un richiamo datato.

### R1-9 — Nit — vincolo 14: «la trappola 5 di `check-docs.sh`» non si trova
- Dove: piano riga 114 (vincolo 14)
- Che cosa dice il piano: *«nessun link `](…)` a un file che non esiste ancora … | trappola 5 di `check-docs.sh`»*.
- Che cosa è vero: `check-docs.sh` non numera trappole; il controllo dei link è la sua **prima** sezione (`== internal
  links ==`), guarda solo i bersagli **`.md`** e **esclude** `docs/superpowers/plans/` (riga 33: `-not -path
  './docs/superpowers/plans/*'`). La regola resta buona per i documenti vivi che i compiti toccano.
- Prova: `grep -n 'trappola 5\|^echo "== ' scripts/check-docs.sh`; `sed -n '16,46p' scripts/check-docs.sh`.
- Correzione proposta: *«il controllo `== internal links ==` di `check-docs.sh`, sui documenti fuori da `plans/` e sui soli
  bersagli `.md`»*.

### R1-10 — Importante — passo 17: nel tema chiaro il testo dei pannelli è illeggibile, non «stona»; e l'attesa si contraddice
- Dove: piano righe 1153–1158 (compito 1, passo 17); tabella della posizione, riga 6 (il dock vestito solo al compito 6)
- Che cosa dice il piano: *«Atteso: i colori della tavola, … nessuna scritta illeggibile. ⚠️ Il dock è ancora
  `themeAbyss` fino al compito 6: nel tema chiaro stona, ed è atteso»*.
- Che cosa è vero: col tema chiaro i pannelli scrivono coi ruoli del chiaro (testo quasi nero) sul fondo dei gruppi di
  `themeAbyss`, `rgb(0, 12, 24)`: il testo pieno sta a **1,11:1**, il tenue a **2,99:1**, l'avviso a **2,85:1** — è
  **illeggibile**, e le due frasi dell'attesa si escludono. Nello scuro gli stessi elementi stanno fra 7,11 e 15,85:1. Non è
  un caso da laboratorio: il tema di base **segue Windows** (risposta 5), quindi su un Windows chiaro il `main` dei compiti
  1–5 — ciascuno pushato — apre la SPA coi pannelli illeggibili; nessuna prova lo vede (sotto jsdom `axe` ha il contrasto
  spento, e `contrast.test.ts` giudica i token, non `themeAbyss`).
- Prova: cartella di prova dopo i passi 1–14, `npx vite --port 5199`, Chrome del pannello; in console `harnessFake.deliverAll()`
  e poi, per tema, il rapporto WCAG fra `color` di `dt`, `h3`, `p` dentro `.dv-content-container` e il primo fondo opaco
  degli antenati → chiaro: `dt "Degrado" 2.99`, `p "Ultima richiesta d" 1.11`, `p "strumento arbiter," 2.85`, tutti su
  `bg=rgb(0, 12, 24)`; scuro: 7.11, 15.85, 8.76. Lo screenshot a 0,5 lo mostra.
- Correzione proposta: A — (consiglio) il `dock.css` del compito 1 lega **già** ai ruoli le poche variabili di
  `themeAbyss` che dipingono i fondi e i testi dei gruppi (`--dv-group-view-background-color`,
  `--dv-tabs-and-actions-container-background-color`, le quattro `--dv-*-tab-background-color` e le quattro
  `--dv-*-tab-color`), come ponte che il compito 6 riscrive; B — si dichiara il vero (*«nel tema chiaro il testo dei pannelli
  è illeggibile sul fondo di `themeAbyss` — 1,11:1 misurato — fino al compito 6»*), si toglie *«nessuna scritta
  illeggibile»* per l'area del dock e si decide se il compito 6 viene subito dopo l'1. In tutti e due i casi l'attesa del
  passo 17 va riscritta.

### R1-11 — Minore — passo 17: «i caratteri Geist e Barlow» — Barlow, dopo il compito 1, non lo usa nessuno
- Dove: piano righe 1156–1157 (compito 1, passo 17)
- Che cosa dice il piano: *«Atteso: … i caratteri Geist e Barlow (DevTools, *Computed*, `font-family`)»*.
- Che cosa è vero: dopo il compito 1 nessun elemento della SPA porta `--font-family-tool`: i ruoli `--font-label`,
  `--font-numeric`, `--font-display` li usano i pezzi di base dei compiti 3 e 5. Su 215 elementi, 208 calcolano `"Geist
  Variable", system-ui, sans-serif` e 7 `Arial` — i controlli nativi (i tre pulsanti delle viste, la ricerca, `+ moduli`, i
  due radio), che non ereditano il carattere, com'era già prima. Fra i `FontFace` caricati c'è il solo `Geist Variable 100
  900`: Barlow è impacchettato ma non scaricato.
- Prova: in console, conteggio di `getComputedStyle(el).fontFamily` su `body *` → `[["\"Geist Variable\", …", 208],
  ["Arial", 7]]`; `[...document.fonts].filter(f => f.status === 'loaded')` → `Geist Variable 100 900`.
- Correzione proposta: *«Geist nel testo (DevTools, *Computed*, `font-family`); Barlow entra con i pezzi di base, dal
  compito 3; i controlli nativi restano sul carattere del sistema finché il compito 5 non li sostituisce»*.

### R1-12 — Nit — i blocchi `bash` presumono la radice, e il `cd gui` del passo 2 non torna indietro
- Dove: piano righe 267–272 (passo 2), 652–654 (passo 8), 702–705 (passo 9), 1125–1127 (passo 15), 1148–1151 (passo 16)
- Che cosa dice il piano: il passo 2 fa `cd gui` su una riga sua; i passi 8, 9 e 15 cominciano con `cd gui && …`; il passo 16
  usa `git diff --name-only` e `git ls-files --others … gui/src` con percorsi dalla radice.
- Che cosa è vero: dipende dal Bash di chi esegue. Nel coordinatore la cartella **persiste** (la nota di memoria sugli
  attrezzi, *«cwd del Bash che persiste»*): dopo il passo 2 `cd gui && npx vitest …` non lancia niente, e il ciclo del
  passo 16 da `gui/` non trova i file. In un subagente la cartella torna alla radice a ogni chiamata, e i blocchi reggono.
  Nella prova i comandi rendono ciò che il piano dice lanciati dalla radice.
- Correzione proposta: blocchi in una sottoshell — `(cd gui && npx vitest run …)` — o `npm --prefix gui …` / `git -C …`; e
  al passo 16 *«dalla radice del repository»*.

### R1-13 — Minore, dedotto — «il primo disegno ha già i suoi colori» vale per il `mount`, non per la prima pittura
- Dove: piano righe 1023–1025 (passo 12, il commento in `main.ts`); righe 746–747 (il commento di `watchTheme`)
- Che cosa dice il piano: *«⛔ BEFORE THE MOUNT, so the first paint already has its colours»*.
- Che cosa è vero: 🔶 dedotto, non misurato — i ruoli vivono **solo** sotto `[data-theme]`, e l'attributo lo mette lo script
  del modulo, che parte dopo l'analisi del documento; il foglio del *build* è invece un `<link>` che blocca la pittura.
  Una pittura prima dello script vede `body { background: var(--color-bg) }` senza valore: fondo trasparente, tela bianca.
  Prima del compito 1 i colori stavano su `:root` e c'erano dalla prima pittura. Il `watchTheme` prima del `mount` resta
  giusto: evita che **Vue** disegni senza colori.
- Prova: `token.html` righe 173 e 218 (i ruoli solo sotto `[data-theme="dark"]` e `[data-theme="light"]`); `gui/index.html`
  non porta `data-theme`.
- Correzione proposta: il commento dica *«before the mount, so Vue's first render already has its colours»*; e una riga fra
  le trappole per il guscio del 10 (`backgroundColor` della finestra, o mostrarla a `ready-to-show`). Nessun cambio alla
  tavola.

### R1-14 — Nit — passo 7: «in coda al file» su un file CRLF, senza dire con che cosa
- Dove: piano riga 609 (passo 7)
- Che cosa dice il piano: *«In `gui/src/stores/stores.test.ts`, in coda al file, un `describe` nuovo»*.
- Che cosa è vero: il file è CRLF (191 CR su 191 righe dopo l'aggiunta, nella prova) e la testa dà per i CRLF solo
  `replace_unique.py`, che sostituisce e non accoda: serve un'àncora unica, o un'aggiunta in Python con `newline=""` e i
  terminatori convertiti. La prova l'ha fatta così, e il file è rimasto CRLF.
- Correzione proposta: *«in coda al file — Python, `newline=""`, il blocco con `\r\n` se il file è CRLF — come per un file
  riscritto per intero»*.

### R1-15 — Nit — passo 12: `app.mount("#app");` resta attaccato a `connection.attach(bridge);`
- Dove: piano righe 1012–1028 (passo 12)
- Che cosa è vero: il blocco sostituito finisce con `app.mount("#app");`, e la riga dopo nel file di oggi è
  `connection.attach(bridge);` senza riga vuota; prima i negozi e `attach` stavano di seguito, ora il `mount` si incolla
  al blocco degli `attach`. Solo forma: `vue-tsc`, prove e linter verdi.
- Correzione proposta: una riga vuota in coda al testo nuovo.

### R1-16 — Nit — passo 18: la colonna «Commit» non può portare l'hash del commit che la scrive
- Dove: piano riga 1162 (passo 18) e la tabella delle righe 125–135
- Che cosa dice il piano: *«La riga 1 della tabella della posizione diventa `✅ <data>` col commit»*.
- Che cosa è vero: la tabella ha due colonne, *«Commit»* e *«Stato»*; un commit non conosce il proprio hash prima di
  nascere, quindi *«col commit»* non dice che cosa va nella colonna *«Commit»* (resta `—`? lo scrive il compito dopo?).
- Correzione proposta: dirlo — per esempio *«Stato: `✅ <data>`; Commit: lo scrive il compito successivo, o un commit a sé
  subito dopo»*.

## Verificato e giusto

- **P-1..P-14, ogni comando rilanciato** (script `r1-head.sh`): P-1 `palette.py` righe 92–104 sono esattamente
  `NEUTRAL_BGS`…`pairs()` (6 × 11 + 3 + 3 + 16 = 88 per tema, 176 in tutto; la tavola, riga 375, dice *«176 coppie di
  contrasto controllate, zero sotto la soglia»*); P-2 `grep -n 'P-101'` → riga 25, e `npx eslint src/main.ts
  src/tokens/theme.ts` → *«File ignored because no matching configuration was supplied»*; P-3 `Confirm.vue` righe 5–6
  (`../stores/core`, `../stores/invoke`); P-4 `sed -n '52,57p'` → la regola `html, body, #app`; P-5 i cinque `<link>` di
  jsDelivr, righe 54–58; P-6 `103` (anche nella `node_modules` della prova) e i due blocchi `.dockview-theme-abyss`,
  righe 1735 e 1787, coi nove `--dv-tab-group-color-*`; P-7 nessun `spacing-padding`, `updateTheme() {` alla riga 18355;
  P-8 `RadioGroupItem.js` riga 102 (`changeModelValue`), `Radio.js` righe 63–66 (`select`, poi `if (ev?.defaultPrevented)
  return`), `role: "radio"`, `as` di base `"button"`; P-9 `.m-strip .sp` in `stile-approvato.html` riga 129 e in
  `panoramica.html`; P-10 `.m-logo` con `harness` in `panoramica.html`; P-11 nessun `F3` in `gui/src`; P-12 nessun
  `matchMedia` in `jsdom/lib`; P-13 `Chat.vue` riga 92 `border-left: 3px solid var(--warn)`; P-14 `eab020d 2026-09-23
  15:07`, `386fc5c … 13:51` (il disegno), e `git log -S 'Una fase per sessione' -- CLAUDE.md` → solo `eab020d`.
- **La «Pila»** contro `gui/package.json`: tutte e undici le versioni giuste (`dockview` c'è anche lui, 8.3.1). Le nuove al
  registro: `lucide@1.47.0` ISC, `@fontsource-variable/geist@5.3.0` e `@fontsource/barlow@5.3.0` OFL-1.1,
  `@vitest/browser-playwright@4.1.11` MIT, `playwright@1.63.0` Apache-2.0; ultime: `reka-ui` 2.10.5, `vitest` 5.0.1 (il
  vincolo 8 regge).
- **Le cifre della testa**: 14 della roadmap (`grep -n '^| 1[0-9] |' docs/roadmap.md` → fino al 13); otto pezzi; 22
  risposte; dieci righe *«proposta»* (controlli 1, 3–8, 16, 18, 19); decisioni 23–27 e precisioni 4–11; 41 ruoli per tema
  coi due comandi della (a).
- **`replace_unique.py`** identico a quello del piano della parte 2 (`diff` → nessuna differenza).
- **Le voci aperte**: N-2 di E187 (riga 414 del piano della parte 2, l'avviso dei 500 kB), E228 (riga 455, *«Progress e
  notifiche per job lunghi»*), X-2 e X-4 (audit, righe 278–279, non toccano la GUI) esistono dove il piano dice. Il
  vincolo 4 (D65, riga 2892 del piano della parte 2) e l'8 (§9 del disegno del 2, la regola delle versioni rimisurate) hanno
  la fonte che citano.
- **D1–D8**: coerenti col disegno e col codice. D2 = la riga *«a mano»* della (a); D3 aggiunge `openNamed` a `named` dove
  la (d) dice *«un campo nuovo»*, con la ragione scritta — non la contraddice; D4 `.field.is-error` c'è, riga 322 della
  tavola; D6 i diciotto `PANEL_TYPES`; D7 `Teleport` di `reka-ui` va di base su `"body"` (`Teleport.js`, riga 28); D8
  coerente col vincolo 4.
- **Il compito 1 applicato per intero nella cartella di prova, alla lettera** (blocchi estratti dal piano per righe, coi
  recinti controllati): `python -m py_compile` verde sui tre script; `extract_tokens.py` sulla tavola → `base.css` 3554
  caratteri, `themes.css` 5662, LF; i quattro *Trova* di `layout.ts` e i tre di `main.ts` unici, CRLF conservati (144 e 77
  CR); `rename_tokens.py` → undici `ok:`, ogni conteggio atteso esatto (5, 1, 1, 5, 13, 4, 2, 1, 4, 1, 1, 1, 1); la riga del
  compendio unica e sostituita (97 980 → 98 014 byte, tetto 109 568). Poi `npm test` → `17 passed | 1 skipped`, `100
  passed | 1 skipped`; `npm run build` (con `vue-tsc --noEmit`) verde; `npm run lint` verde; `npm audit` 0.
- **I rossi del passo 8** (tranne R1-1 e R1-2) sono quelli detti: `ENOENT` per `base.css`/`themes.css`, `Failed to resolve
  import "./theme"`, `expected undefined to be 'light'`, `layout.chooseTheme is not a function`.
- **Le sei violazioni del passo 16** danno il rosso atteso, col messaggio atteso (`themes.css is the board's block`;
  `color-text-muted on color-bg: 3.34 < 4.5` e altre dieci; `[ 'color-bg-test' ]` nei due temi; `panels/Strip.vue`;
  `panels/Strip.vue:29: .probe { color: #fff; }`; `expected 'dark' to be 'light'`), e tutto torna verde. I fine-riga dopo
  i passi 2–14: CR = righe su ogni CRLF, 0 su ogni LF e su ogni file nuovo; il primo comando del passo 16 non rende nulla.
- **La mappa vecchio → nuovo** (mandato 3): i nomi vecchi vivono solo nei `.vue` e in `tokens.css` — nessun `.ts` li usa,
  nessun `getPropertyValue`/`.style` in `gui/src`; dopo il compito ogni `var(--…)` di `gui/src` è dichiarato in `base.css`,
  `themes.css` o `dock.css` (script `r1_names.py` → *«missing: `--ref-` in `usage.test.ts`»*, la stringa della sonda).
  Le regole di `tokens.css` che non stanno nella tabella hanno casa: `body` e `:focus-visible` nel blocco `base.css` della
  tavola (righe 141–151), `html, body, #app` in `App.vue`, le regole del dock in `dock.css` con `--size-control-lg` = 40px.
- **Nel browser**: `data-theme="dark"` al primo carico col sistema scuro, `color-scheme` che segue, `body` coi colori dei
  due temi, `font: 14px/20px "Geist Variable"`, `margin: 0`, altezza 100%, la presa grande a 40 px.
- **Il pacchetto e il vincolo 12** (mandato 4): `theme` vive dentro i byte di `pack_` (JSON), `LayoutState` e
  `gui/src/schema/messages.ts` non cambiano, `git status -- gui/schema gui/src/schema` vuoto nella prova; il dock guarda
  `view` e `arrivals`, non `saved`, quindi `chooseTheme` non riapplica la disposizione.
- **Il compendio** (mandato 5): *«Lo stile di oggi è un **segnaposto dichiarato**, in testa a `gui/src/tokens/tokens.css`.»*
  esiste, unica, riga 663, nella §6 (`## 6.` alla riga 600, `## 7.` alla 743).
- **Le interfacce** del compito 1 sono quelle che i compiti 2, 4, 5 e il *«Come si riprende»* (compito 6, `shownTheme`)
  consumano, con la stessa forma; il commento di `a11y.test.ts` lo riscrive davvero il compito 3 (riga 1553).
- **Il *«Come si riprende»***, primo capoverso: *«Scritti: la testa e i compiti 1–5»* — `grep -c '^## Compito'` → 5.
- **Codice in inglese**: nessuna parola italiana nei blocchi dettati del compito 1; *«Impostazioni»* nei commenti segue
  l'uso già presente in `Settings.vue` e `Status.vue`.

## Non verificato, e perché

- La costante `0.03928` accanto a *«WCAG 2.2 relative luminance»* in `contrast.test.ts`: credo che la 2.2 scriva `0.04045`
  (senza effetto sui valori a 8 bit), ma la pagina del W3C non ha reso il glossario al `WebFetch`. Non è un rilievo.
- La prima pittura bianca di R1-13: dedotta, non misurata (una pittura prima dello script non si cattura in modo affidabile).
- `bash scripts/gate.sh` e `gate-gui.sh` non girati (regola 1 dei vincoli comuni); al loro posto, nella prova, i passi del
  cancello web uno per uno: `npm test`, `npm run build`, `npm run lint`, `npm audit`.
- I compiti 2–5 letti solo nelle intestazioni e dove usano le interfacce del compito 1: sono fette degli altri revisori.

## Comandi rilanciati

`git status -sb`, `git ls-files --eol` (i file del compito, e `gui` intero); `r1-head.sh` (i comandi di P-1..P-14);
`npm view <p>@<v> version license` per le sei nuove e `npm view <p> version` per le ultime; `cat gui/package.json`; il
censimento del passo 1 (scritto con l'heredoc del piano: i backslash restano) sul repository e nella prova;
`grep -rlE 'var\(--' gui/src --include='*.vue' | wc -l` → 11, `find gui/src -name '*.vue'`, `grep -rL '<style scoped'`;
i due comandi della (a) sui ruoli → 41 e 41; `grep` su roadmap, `check-docs.sh`, audit, piano della parte 2 (a sezioni),
disegno del 2 (§9), compendio (solo la riga 663 e i titoli `## `); nella prova: `npm ci`, `npm install --save-exact` dei
due caratteri, `git diff --stat`, `npm ls`, `python -m py_compile`, `extract_tokens.py`, `replace_unique.py` ×9,
`rename_tokens.py`, `npx vitest run` (passi 8, 9, 10 e le otto violazioni), `npm test`, `npm run build`, `npm run lint`,
`npx eslint src/main.ts src/tokens/theme.ts`, i due comandi dei fine-riga del passo 16, `npx vite --port 5199` con le sonde
in console.

## Stato del repository alla fine

`git -C C:/Users/zagor/Desktop/harness status --porcelain` → **vuoto** (0 righe); HEAD `d589d15`. Nessun `git fetch` (sola
lettura: il coordinatore ha fissato la testa). Il server di prova è spento, il pannello del browser chiuso. La cartella di
prova `C:\Users\zagor\AppData\Local\Temp\probe-ds1` resta com'è, col compito 1 applicato, per chi vuole guardarla: si può
cancellare.
