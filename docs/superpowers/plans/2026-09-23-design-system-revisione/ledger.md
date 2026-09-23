# Registro della revisione — il piano del design system, compiti 1–5, 2026-09-23

⛔ **Nessuna correzione è applicata.** Questo registro è la **casa** dei rilievi e del loro stato (⬜ da applicare, ✅
applicato): la sessione che corregge il piano lo segue riga per riga, e spunta **qui**, nello stesso commit.

## Che cosa è stato fatto

- **Chi:** tre revisori Opus in parallelo, in sola lettura sul repository, su `d589d15`, dopo una sessione di scrittura
  chiusa a contesto saturo senza consegna. I prompt sono accanto: `constraints.md` (comune) e `R1-prompt.md`…`R3-prompt.md`.
- **Le fette:** R1 la testa e il compito 1; R2 i compiti 2 e 3; R3 i compiti 4 e 5 e il *«Come si riprende»*. Ciascuno ha
  **applicato** i compiti della sua fetta in una cartella di prova fuori dal repository e ha fatto girare `vue-tsc`, `vitest`
  (anche nel browser, sul Chrome installato), `eslint`, `vite build` e gli script dettati. I rapporti sono accanto:
  `R1-report.md`, `R2-report.md`, `R3-report.md` — lì le prove, i comandi e il testo proposto di ogni correzione.
- **Il costo:** R1 332k token, 101 chiamate, ~2 h; R2 403k, 169, ~2 h 15; R3 496k, 218, ~2 h 15 — in tutto **1,23M token**,
  ~2 h 15 di orologio perché in parallelo. La stima detta al proprietario era 1–1,3M e ~45 minuti: il costo ha retto, il
  tempo no — applicare i compiti e provarli nel browser costa come eseguirli.
- **L'esito:** **59 rilievi** — 3 critici, 17 importanti, 28 minori, 11 di forma. ✅ **Nessuna allucinazione di
  sostanza:** le versioni, le API delle librerie alle versioni appuntate e le fonti citate esistono e dicono ciò che il
  piano fa dire loro; i comandi di P-1…P-14 rendono ciò che le righe affermano. ⛔ **I difetti sono di altre specie:**
  prove che non mordono (vacue, o rosse per la ragione sbagliata), attese scritte senza misurarle, due blocchi che non
  partono come sono dettati (R2-1, R2-9), e le forme dei compiti 6–8 del *«Come si riprende»*, che il compito 6 avrebbe
  seguito sbagliando (R3-17).
- **Verificati dal coordinatore coi suoi comandi**, oltre alla prova del revisore: R1-1, R1-6, R1-7, R1-8, R2-1, R2-9
  (`vue-tsc` rilanciato sulla cartella di prova col cast dettato: uscita 2, `TS2352` a `kit.test.ts(21,14)`), R3-17
  (`dockview.css` riga 1037, `main.esm.mjs` riga 12023), R3-18, R3-20. Gli altri sono accettati sulla prova scritta nel
  rapporto.

## Le decisioni prese su questi rilievi

| Rilievo | Chi | La decisione |
|---|---|---|
| **R1-8**, il controllo 2 del contrasto | il **proprietario**, 2026-09-23 | **A — per famiglie**, com'è il compito 1: le 176 coppie approvate con la tavola. Costo scritto e accettato: il commento di `themes.css` nella tavola (riga 163 di `token.html`, *«Every text role reads 4.5:1 on every background role»*) si riscrive per famiglie, **prima** che il compito 1 la copi; nel disegno un richiamo datato sulla (a), *«Due livelli»*, e sulla riga 2 della tabella dei controlli; P-1 del piano dice la scelta |
| **R2-11**, la regola dei pezzi di base | il **proprietario**, 2026-09-23 | **A — due regole**: i pezzi di base (`components/Base*.vue`, `components/icons.ts`) non importano `pinia` né `stores/`, com'è P-3; **e** tutto `components/**` tranne i `*.test.ts` non importa gli strati sopra — `panels/`, `frame/`, `transport/`. Oggi è verde (`Confirm.vue` importa solo `stores/`, `markdown.ts` solo `markdown-it`). Ciascuna provata nelle due direzioni; nel disegno un richiamo datato sulla tabella *«Le regole»* della (b); P-3 del piano dice la scelta |
| **R1-10**, il tema chiaro illeggibile fino al compito 6 | il **coordinatore** | **A — un ponte nel `dock.css` del compito 1**, che il compito 6 sostituisce: le variabili di `themeAbyss` che dipingono gruppi e linguette legate ai nostri ruoli (il blocco qui sotto). Il foglio viene dopo `dockview.css` in `main.ts`, quindi lo stesso selettore vince; il contrasto è quello delle famiglie per costruzione, e il passo 17 lo **misura** nel browser |
| **R2-3** e **R3-6**, i PNG delle prove rosse | il **coordinatore** | `screenshotFailures: false` nel progetto `browser` del compito 2, con la ragione accanto (il cancello non guarda immagini, e il nome di un file supera i 260 caratteri di Windows); i passi delle due direzioni finiscono con `git status --porcelain` vuoto. ⚠️ Da rimisurare applicandolo: che anche `gui/.vitest-attachments/` non nasca più; se nasce, una riga in `.gitignore` |
| **R3-21**, `colorScheme` che `dockview-core` 8.3.1 non usa per disegnare | il **coordinatore** | resta, perché è testo approvato della (c) e costa un osservatore; il compito 6 lo dice accanto, così nessuno si aspetta che cambi l'aspetto del dock |
| **R2-8** e **R3-22**, due fatti sbagliati nel disegno | il **coordinatore** | si correggono **nel disegno col richiamo datato** — un fatto, non il merito: col canale `chrome` il ripiego è `npx playwright install chrome`, non `chromium`; `DockviewTheme` in 8.3.1 ha **undici** campi, non sette |
| **R1-6**, *«dodici componenti»* nella decisione 14 del disegno | il **coordinatore** | il disegno **non** si tocca: dodici sono i componenti con lo stile `scoped`, undici quelli che usano i token, e il compito 1 segue il censimento, non il numero. Si corregge solo la tabella della posizione del piano |

Il ponte di R1-10, da scrivere nel `dock.css` del compito 1 (i nomi letti in `dockview.css` 8.3.1, secondo blocco
`.dockview-theme-abyss`, e nei due temi della tavola):

```css
/* ⛔ A BRIDGE UNTIL TASK 6 DRESSES THE DOCK: `themeAbyss` paints the groups and the tabs dark, and the light theme's
   text would sit on them at 1.11:1 (measured). These bind the few variables that paint behind text to our roles;
   task 6 replaces this block with a theme of our own. It wins over `dockview.css` because it is loaded after it. */
.dockview-theme-abyss {
  --dv-group-view-background-color: var(--color-bg-surface);
  --dv-tabs-and-actions-container-background-color: var(--color-bg);
  --dv-activegroup-visiblepanel-tab-background-color: var(--color-bg-surface);
  --dv-activegroup-hiddenpanel-tab-background-color: var(--color-bg);
  --dv-inactivegroup-visiblepanel-tab-background-color: var(--color-bg-surface);
  --dv-inactivegroup-hiddenpanel-tab-background-color: var(--color-bg);
  --dv-activegroup-visiblepanel-tab-color: var(--color-text);
  --dv-activegroup-hiddenpanel-tab-color: var(--color-text-muted);
  --dv-inactivegroup-visiblepanel-tab-color: var(--color-text-muted);
  --dv-inactivegroup-hiddenpanel-tab-color: var(--color-text-muted);
  --dv-tab-divider-color: var(--color-border);
  --dv-separator-border: var(--color-border);
  --dv-paneview-header-border-color: var(--color-border);
}
```

⚠️ Il `dock.css` del compito 1 ha già una regola `.dock .dockview-theme-abyss` (l'altezza delle linguette): il ponte le
sta accanto, e la prova `usage.test.ts` del compito 1 lo giudica come il resto del foglio (nessun colore a mano, nessuna
scala).

## I rilievi

Il testo intero, la prova e la correzione proposta stanno nel rapporto. *«Come proposto»* vuol dire il testo della
correzione del rapporto; dove la correzione è un'altra, la dice la riga. **c** è il compito, **p** il passo.

### R1 — la testa e il compito 1

| # | Gravità | Dove | Il rilievo | Correzione | Stato |
|---|---|---|---|---|---|
| R1-1 | Importante | c1 p8, p16 | la terza prova del negozio (*«reads a choice it does not know as absent»*) è verde prima del codice, perché `unpack` di oggi non legge `theme`, e nessun passo la fa rossa | come proposto: il p8 dice «due su tre»; il p16 due righe — `unpack` senza `isThemeChoice`, `settle` senza lo spread | ⬜ |
| R1-2 | Minore | c1 p8 | `usage.test.ts` è rosso anche per i due veli | come proposto | ⬜ |
| R1-3 | Minore | c1 p13 | il censimento trova ancora i nomi di `tokens.css`, che esce al p14 | come proposto | ⬜ |
| R1-4 | Minore | c1 p2 | `npm` riordina `dockview-core`: il diff è `4 +++-` | come proposto | ⬜ |
| R1-5 | Minore | c1 p15 | `grep 'kB'` rende 32 righe; i caratteri sono anche `.woff` | come proposto | ⬜ |
| R1-6 | Minore | testa, riga 1 della posizione | *«dodici componenti»*: sono undici | «undici»; il disegno no (decisioni qui sopra) | ⬜ |
| R1-7 | Minore | vincolo 9 | i file di `gui/` sono misti per file, 41 CRLF e 40 LF | come proposto | ⬜ |
| R1-8 | Importante | P-1, c1 p4 | il controllo 2, approvato nella (f), cambia forma | la decisione **A** del proprietario, qui sopra | ⬜ |
| R1-9 | Nit | vincolo 14 | *«la trappola 5 di `check-docs.sh`»* non esiste | come proposto | ⬜ |
| R1-10 | Importante | c1 p17 | nel tema chiaro il testo dei pannelli sta a 1,11:1 sul dock di `themeAbyss`, non «stona» | il ponte, qui sopra; il p17 riscritto con la misura | ⬜ |
| R1-11 | Minore | c1 p17 | Barlow non lo usa nessuno dopo il compito 1 | come proposto | ⬜ |
| R1-12 | Nit | c1, i blocchi `bash` | il `cd gui` persiste nel Bash del coordinatore | sottoshell `(cd gui && …)` — ⚠️ **specie**: in tutti i compiti, `grep -n 'cd gui' <piano>` | ⬜ |
| R1-13 | Minore, dedotto | c1 p12, p10 | *«first paint»*: vale per il primo disegno di Vue, non per la prima pittura | come proposto; la riga del guscio del 10 fra *«Le voci aperte»* del piano | ⬜ |
| R1-14 | Nit | c1 p7 | *«in coda al file»* su un file CRLF, senza dire con che cosa | come proposto | ⬜ |
| R1-15 | Nit | c1 p12 | `app.mount` attaccato a `connection.attach` | come proposto | ⬜ |
| R1-16 | Nit | c1 p18 | la colonna *«Commit»* non può portare l'hash del commit che la scrive | come proposto — ⚠️ **specie**: vale per il passo finale di ogni compito | ⬜ |

### R2 — il compito 2 e il compito 3

| # | Gravità | Dove | Il rilievo | Correzione | Stato |
|---|---|---|---|---|---|
| R2-1 | **Critico** | c2 p2, p4 | la prova del contorno del focus è rossa com'è dettata: `--color-focus` vive solo sotto `[data-theme]`, e la prova non posa l'attributo | come proposto: `dataset.theme = "dark"` nella prova, tolto nell'`afterEach` | ⬜ |
| R2-2 | Importante | c2 p2, p5 | la stessa prova non prova l'alto contrasto: passa identica senza `forcedColors` | come proposto: la guardia `matchMedia("(forced-colors: active)")` e la seconda direzione col `box-shadow`, una riga nel p5 | ⬜ |
| R2-3 | Importante | c2 p3, p5 | ogni rosso del browser scrive PNG in `gui/src/**/__screenshots__/`, che git non ignora | la decisione del coordinatore, qui sopra | ⬜ |
| R2-4 | Importante | c2 p2 | la sonda dei caratteri non distingue Barlow da Bahnschrift, il suo ripiego | come proposto: la terza asserzione, e il commento riscritto | ⬜ |
| R2-5 | Minore | c2 p5, riga 2 | con `enabled: false` il file non si carica: il messaggio è un altro | come proposto | ⬜ |
| R2-6 | Minore | c2 p2 | `/Chrome\//` prende anche il Chromium di Playwright | come proposto: la marca `"Google Chrome"` in `userAgentData.brands` | ⬜ |
| R2-7 | Minore | c2 p4 | i nomi dei due progetti non stanno nell'uscita di base | come proposto: `--reporter=verbose` coi conteggi | ⬜ |
| R2-8 | Minore | disegno, decisione 22 e (f); c9 | col canale `chrome` il ripiego è `npx playwright install chrome` | la riga 9 del *«Come si riprende»* (già corretta); il richiamo nel disegno | ⬜ |
| R2-9 | **Critico** | c3 p3 | `BaseList as Component` non compila (`TS2352`): il *build* esce 2 | come proposto: `as unknown as Component` | ⬜ |
| R2-10 | Minore | c3 p7 | il commento *«nobody's business»* di `eslint.config.js` resta a dire il contrario | come proposto: una quarta sostituzione | ⬜ |
| R2-11 | Minore | P-3, c3 p7 | P-3 restringe una regola approvata della (b) | la decisione **A** del proprietario, qui sopra: un blocco in più nel p7 e due righe nel p8 | ⬜ |
| R2-12 | Minore | c3 p2 | il `grep` del blocco dell'aiutante lascia fuori il `/**` | come proposto | ⬜ |
| R2-13 | Minore | c3 p6 | al compito 3 il pezzo JavaScript non cambia: il kit non è ancora importato | come proposto; la misura al compito 5 (R3-25) | ⬜ |
| R2-14 | Minore | c3 p5 | `iconOnly` è un `computed` su slot non reattivi | come proposto: nel template | ⬜ |
| R2-15 | Minore | D6, c3 p4 | le tavole non danno l'icona della Chat | come proposto | ⬜ |
| R2-16 | Minore | c3 p3, p8 | le prove del kit non si vedono mai rosse; una senza guardia | come proposto | ⬜ |
| R2-17 | Nit | c3 p7, p8 | un `import()` dinamico passa la regola | detto accanto | ⬜ |
| R2-18 | Nit | c3 p6 | `BaseDialog`: due misure a mano; `aria-describedby` a vuoto senza descrizione | come proposto: la larghezza detta scelta del piano; il `v-bind` | ⬜ |

### R3 — il compito 4, il compito 5 e il «Come si riprende»

| # | Gravità | Dove | Il rilievo | Correzione | Stato |
|---|---|---|---|---|---|
| R3-1 | Importante | c4 p7, riga 1; `Kit.vue` | il rosso viene dalla finestra, e le radici `kit-card` non guardano niente: la griglia stira le schede | come proposto: `align-items: start` in `.kit-grid`, il rosso vero, la violazione su `BaseList` | ⬜ |
| R3-2 | Importante | c4 p7, riga 2 | vacua: `fits` resta verde | come proposto: le due violazioni che mordono | ⬜ |
| R3-3 | Importante | c4 p7, riga 3 | vacua: la sonda salta un genitore non centrato | come proposto: `top: 3px` sull'icona | ⬜ |
| R3-4 | Importante | c4 p7, riga 4 | la prova della pagina resta verde; il rosso è del compito 2 | come proposto: la violazione dentro `Kit.vue` | ⬜ |
| R3-5 | Importante | c4 p7 | `git checkout --` su `Kit.vue`, che non è ancora di git | come proposto: la copia salvata | ⬜ |
| R3-6 | Importante | c2 (la radice), c4 | PNG e allegati nel sorgente a ogni rosso | con R2-3, qui sopra | ⬜ |
| R3-7 | Minore | c4 p2 | le prove `axe` della pagina senza guardia | come proposto | ⬜ |
| R3-8 | Minore | c4 p7, riga 5 | rosso anche `contrast.test.ts` | come proposto | ⬜ |
| R3-9 | Minore | c4 p7, e ogni «indietro» | `git checkout --` riscrive CRLF i file nati LF (`core.autocrlf`) | come proposto — ⚠️ **specie**: `grep -n 'git checkout --' <piano>` | ⬜ |
| R3-10 | Nit | c4 p6 | `rollupOptions` è deprecato in Vite 8.3.0 | `rolldownOptions` | ⬜ |
| R3-11 | Nit | c4 | la pagina kit ha il margine di 8 px | come proposto | ⬜ |
| R3-12 | Importante | c5 p1, p4 | *«M-3 chiusa per costruzione»* non ha una prova: riaprirla lascia tutto verde | come proposto: tre prove, fascia, Stato, Impostazioni | ⬜ |
| R3-13 | Importante | c5 p8 | al caricamento regione e parole nascono insieme; manca la regione di Stato | come proposto | ⬜ |
| R3-14 | Minore | c5 p2 | la chiave costruita del tema senza sonda | come proposto: una prova in `copy.test.ts` | ⬜ |
| R3-15 | Nit | c5, *Files* | `Placeholder.vue` è un *Modify* | come proposto | ⬜ |
| R3-16 | Nit | c5, *Interfaces* | una sola `createElement`, usata due volte | come proposto | ⬜ |
| R3-17 | **Critico** | riga 6 del *«Come si riprende»* | `--dv-overlay-z-index` sul tema non abbassa i gruppi galleggianti: `dockview.css` lo ridefinisce sul contenitore come un ciclo | la riga 6 è **già corretta** nel *«Come si riprende»* di questo commit; il compito 6 lo prova nel browser | ✅ nella riga 6 |
| R3-18 | Importante | riga 7 | la geometria di `moveActive.ts` presa com'è manda «giù» alla prima colonna | la riga 7 è **già corretta**: lo spareggio sull'altro asse | ✅ nella riga 7 |
| R3-19 | Importante | riga 7 | non è scritto dove scrive `settle` con una vista col nome aperta | la riga 7 è **già corretta** | ✅ nella riga 7 |
| R3-20 | Importante | riga 8 | la striscia è un'app Vue sua: il pulsante «moduli» non raggiunge il `ref` di `Drawer.vue` | la riga 8 è **già corretta**: lo stato del cassetto in un negozio | ✅ nella riga 8 |
| R3-21 | Minore | riga 6 | `colorScheme` non cambia ciò che `dockview` disegna | la decisione del coordinatore; la riga 6 è **già corretta** | ✅ nella riga 6 |
| R3-22 | Minore | disegno (c), riga 6 | `DockviewTheme` ha undici campi, non sette | la riga 6 è **già corretta** (`dndOverlayBorder`); il richiamo nel disegno resta | ⬜ il disegno |
| R3-23 | Minore | righe 3, 8, 9 | tre omissioni | le righe 8 e 9 sono **già corrette**; la 3 è archiviata | ✅ |
| R3-24 | Minore | c5 p1 | la via della tastiera sul radio nuovo non ha una prova | come proposto: nel browser | ⬜ |
| R3-25 | Minore | voci aperte; c5 | il pezzo JavaScript cresce al compito 5 (+25,59 kB, +8,44 compressi) | come proposto: la misura ai compiti 5, 6 e 8, e la cifra al proprietario, che ha N-2 | ⬜ |

## Come si applica

1. Uno **script solo**, ad ancore uniche asserite **prima** di scrivere, nella forma di `replace_unique.py` della testa del
   piano; il piano è LF. Ogni correzione nel **compito che la causa**, non in errata: il piano non è ancora eseguito.
2. Le **specie** si cercano in tutto il piano, non solo dove il rapporto le ha viste: R1-12 (`cd gui`), R1-16 (la colonna
   *«Commit»*), R3-9 (`git checkout --`), e i passi *«le due direzioni»* di ogni compito dopo R2-3.
3. Dove il rapporto dà un testo provato nella sua cartella di prova, si copia quello; dove no, il testo nuovo si prova
   **prima** di scriverlo — le cartelle di prova dei revisori restano su questa macchina,
   `C:\Users\zagor\AppData\Local\Temp\probe-ds1` (compito 1 applicato), `probe-ds2` (compiti 2–3), `probe-ds3` (compiti
   1–5, con un suo `git`): si cancellano a correzioni finite.
4. Il disegno e la tavola ricevono i **richiami datati** delle decisioni qui sopra; la tavola **prima** del compito 1.
5. Dopo: `bash scripts/check-docs.sh`, il cancello **da solo**, ogni ⬜ di questo registro diventato ✅, commit e push.
