# Il design system della GUI — il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`, un subagente fresco per
> compito, su `model: "opus"`, con revisione fra uno e l'altro: la modalità scelta dal proprietario (`CLAUDE.md`). I passi
> usano le caselle (`- [ ]`) per il tracciamento.
> ⛔ **Una fase per sessione** (`CLAUDE.md`): questo piano è scritto in una sessione sua; il **pre-controllo** delle quattro
> domande si fa in un'altra, **prima** di dispacciare; ogni compito si esegue in un'altra ancora.

**Obiettivo.** Tradurre in codice il [disegno del design system](../specs/2026-09-22-design-system-design.md) — le sezioni
**(a)** i token, **(b)** il kit, **(c)** il dock, **(d)** la cornice, **(e)** le voci registrate, **(f)** le sonde che
diventano test — e le righe che il lavoro deve ai documenti: la riga «Accessibilità» di `docs/tracciabilita.md`, la riga
del **sotto-progetto 14** nella roadmap con la sua riga in *«Perché quest'ordine»*, `docs/README.md`, la §12 del compendio.
⚠️ **Il numero è 14** perché è il primo libero in coda alla roadmap (voce 2 del disegno, scelta **A** il 2026-09-23):
`grep -n '^| 1[0-9] |' docs/roadmap.md` rende fino al 13. La riga la scrive il compito di chiusura, com'è il punto 9 del
*«Come si riprende»* del disegno; finché non c'è, il numero vive solo qui.

**Architettura.** Le **variabili CSS sono la verità** (risposta 14): due fogli — `base.css`, ciò che non cambia col tema, e
`themes.css`, le scale `--ref-*` e i ruoli `--color-*` dei due temi — copiati **byte per byte** dalla
[tavola dei token](../specs/2026-09-22-design-system-tavole/token.html), e un test che li tiene uguali. Il tema si posa come
attributo `data-theme` sulla radice; la scelta vive nel pacchetto della disposizione, che il core custodisce senza aprirlo.
Il kit sono **otto pezzi di base** in `gui/src/components/`, sopra `reka-ui` dove serve tastiera e ARIA, con le icone di
`lucide` dietro **una** porta; le regole stanno nel linter che c'è già; le sonde delle tavole diventano test nel **browser
vero**, Chrome installato, dentro il cancello.

**Pila.** Vue 3.5.42, Pinia 4.0.3, `vue-i18n` 11.4.10, `dockview-core` 8.3.1, `reka-ui` **2.10.4**, Vite 8.3.0, Vitest
4.1.11 con jsdom 30.0.1, `axe-core` 4.13.0, ESLint 10.10.0 con `eslint-plugin-vue` 10.11.0 — tutte da `gui/package.json`.
**Nuove**, approvate dal proprietario nel brainstorming: `lucide` 1.47.0 (risposta 10), `@fontsource-variable/geist` e
`@fontsource/barlow` 5.3.0 (risposte 6 e 8), `@vitest/browser-playwright` 4.1.11 e `playwright` 1.63.0 (risposta 22).

**Disegno:** [`specs/2026-09-22-design-system-design.md`](../specs/2026-09-22-design-system-design.md), **per intero**, prima
del compito; e le tavole approvate, in [`2026-09-22-design-system-tavole/`](../specs/2026-09-22-design-system-tavole/). La
forma della GUI — viste, moduli, disposizione, protocollo — vive nella
[stella polare](../specs/2026-09-07-direzione-gui-design.md): si apre **a pezzi**, dove il compito la nomina.

**Strumenti.** `bash`, `awk`, `grep`, `sed -n` in lettura; **Python 3** per ogni scrittura su un file **CRLF**, con l'aiutante
qui sotto; `node` e `npm` nelle versioni che `gui/package.json` accetta (`engines`); `git`. La porta di qualità è
`bash scripts/gate.sh`, **da sola** (gotcha **#133**), e deve stampare `GATE GREEN` prima di ogni commit; `bash
scripts/check-docs.sh` deve stampare `OK`. Dentro `gui/` il ciclo breve è `npm test`, `npm run build`, `npm run lint`.

⛔ **L'aiutante `replace_unique.py` vive nello scratchpad, mai nel repository** — lo stesso del
[piano della parte 2](2026-09-11-sottoprogetto-2-parte-2-gui-minima.md), riportato qui perché un piano porta i propri
attrezzi. Sostituisce **una** occorrenza unica, conserva i fine-riga del file, e rifiuta se il testo vecchio manca o non è
unico:

```python
"""replace_unique.py -- replace ONE unique occurrence of a text, keeping the file's line endings.

Usage: python replace_unique.py <file> <old.txt> <new.txt>

<old.txt> and <new.txt> hold the exact texts (UTF-8). A single trailing newline in each is
dropped, so a file written with an editor works. If <file> is CRLF, the texts are converted to
CRLF before matching and writing. Refuses when the old text is absent or not unique. Builds the
whole content first, writes a temporary file, then os.replace -- a rename cannot fail halfway
(gotcha #82).
"""
import io
import os
import sys

path, old_path, new_path = sys.argv[1:4]
raw = io.open(path, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw


def text(p):
    t = io.open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
    if t.endswith("\n"):
        t = t[:-1]
    return t.replace("\n", "\r\n") if crlf else t


old, new = text(old_path), text(new_path)
n = raw.count(old)
if n != 1:
    sys.exit(f"refused: {n} occurrences of the old text in {path}")
out = raw.replace(old, new)
tmp = path + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print(f"ok: {path} ({'CRLF' if crlf else 'LF'})")
```

Un file che un compito **riscrive per intero** si scrive con Python `newline=""` e il **terminatore del file com'era**:
`crlf = b"\r\n" in open(path, "rb").read()`, poi il testo nuovo con `\n` sostituito da `\r\n` se `crlf`. Dopo ogni scrittura
si rimisura, e su un file CRLF i CR devono essere uguali alle righe:

```bash
for f in <i file toccati>; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol <i file toccati>
```

---

## Vincoli globali

Valgono per ogni compito, senza che il compito li ripeta.

| # | Vincolo | Da |
|---|---|---|
| 1 | **il merito approvato non si tocca**: le sezioni (a)–(f) del disegno e le 22 risposte. Se un passo lo violerebbe — una scorciatoia, una duplicazione, un fatto che non è più vero — ci si **ferma e lo si riporta** | testa del disegno; `anthropic-skills:decision-principles` |
| 2 | ⛔ **i valori dei token si copiano, non si riscrivono**: stanno solo nella tavola, fra `/* ===== proposta/base.css ===== */`, `/* ===== proposta/themes.css ===== */` e `/* ===== the board itself, only tokens ===== */`; li copia lo script del compito 1 | (a); decisione 23 |
| 3 | **codice in inglese, documenti in italiano**: TypeScript, Vue, CSS, shell e i loro commenti in inglese; un riferimento al codice dentro un documento porta il **nome esatto del sorgente** | §1.0 della spec; gotcha #40 |
| 4 | ⛔ **le parole dell'interfaccia vengono da `gui/src/locales/it.json`**: nessun pezzo di base contiene una scritta; la regola `@intlify/vue-i18n/no-raw-text` resta a `error` fuori dalla pagina kit | (b); D65 del piano della parte 2 |
| 5 | **nessun colore a mano fuori dai file dei token**: nessun `#…`, `rgb(`, `hsl(` in `gui/src/**/*.vue` e in `tokens/dock.css`; il test del compito 1 lo tiene | (a); controllo 5 del disegno |
| 6 | **nessun componente legge una scala**: `var(--ref-` vive solo in `gui/src/tokens/` | (a); controllo 4 |
| 7 | ⛔ **una dipendenza si aggiunge in DUE passi**: `gui/package.json` e `gui/package-lock.json` **insieme**, il lockfile rinfrescato **fuori** dal cancello con `npm install --save-exact`; il cancello gira `npm ci`, che è `--locked` | `CLAUDE.md`, finding G-5 |
| 8 | **le versioni sono appuntate** a quelle rilanciate il 2026-09-23: `reka-ui` **resta 2.10.4** — al registro c'è la 2.10.5, e salire è un'altra decisione (voce 18 delle trappole del disegno); `vitest` resta 4.1.11; una major nuova non si prende | §9 del 2; `riferimenti.md` |
| 9 | **i fine-riga si conservano per file**: i file di `gui/` sono `i/lf` nell'indice, e nell'albero la forma dipende dal `core.autocrlf` di ciascuna macchina: si **misura** per file con `git ls-files --eol`, non si crede a un'etichetta (voce **E72** del piano della parte 2; R1-7 della revisione); i file nuovi nascono **LF**; un CRLF si tocca con Python, **mai** `sed -i` | `CLAUDE.md`; trappola 16 del disegno |
| 10 | **ogni conteggio si rifà col comando**: le cifre di questo piano sono istantanee del 2026-09-23 su `fd2812b` | `CLAUDE.md` |
| 11 | **ogni prova si prova nelle due direzioni**: che scatti dove deve — una violazione messa a mano, poi tolta — e che non scatti dove non deve; e ogni prova del browser porta la **guardia di non-vacuità**, quante cose ha guardato, maggiore di zero. ⛔ **Si torna indietro con la copia salvata, mai con `git checkout --`**: durante un compito i suoi file non sono ancora committati, e `git checkout` non conosce un file nuovo (R3-5), riporta indietro anche il lavoro del compito in un file che il compito ha già cambiato (A-1) e, dove `core.autocrlf` è acceso, riscrive CRLF un file nato LF (R3-9). Prima della prima violazione `git status --porcelain > <scratchpad>/prima.txt` e una copia di ogni file che le violazioni toccano; dopo ciascuna la copia torna, e `cmp` lo conferma; alla fine `git status --porcelain \| diff <scratchpad>/prima.txt -` non rende nulla: nessun file nato dalle prove (R2-3) | `CLAUDE.md`; (f); R2-3, R3-5, R3-9 e A-1 della revisione |
| 12 | **il kernel non cambia**, e nemmeno il filo: `git diff --stat <base>..HEAD -- crates/ gui/schema/` vuoto a ogni compito. Tutto ciò che è nuovo nel pacchetto della disposizione è un campo **facoltativo** dei byte opachi | (a), (d); I1, I4 |
| 13 | **il compendio resta sotto il tetto**: margine misurato prima e dopo ogni tocco, `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh`; se va rosso si toglie prosa dalla §6, **non si alza il tetto** | §13 del compendio, gotcha #100 |
| 14 | **nessun link `](…)` a un file che non esiste ancora**: si nomina in code span, e il link nasce nel commit che crea il file | il controllo `== internal links ==` di `check-docs.sh`, sui documenti fuori da `plans/` e sui soli bersagli `.md`; e la trappola 5 della §10 del compendio: un file nuovo, non ancora aggiunto, è letto (R1-9) |
| 15 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore**; il cancello e `check-docs.sh` girano **prima**, uno alla volta; il messaggio comincia con `design-system(compito N): …` | `CLAUDE.md` |

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

⏳ **IL PIANO È IN SCRITTURA dal 2026-09-23.** A dirlo non è questa riga ma i due comandi, che coincideranno a piano scritto:
`grep -c '^## Compito' <questo file>` e le righe `| **N** |` della tabella qui sotto. ⛔ **Nessun compito si esegue** prima del
pre-controllo, in una sessione sua.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | **i token**: `base.css` e `themes.css` copiati dalla tavola da uno script, e il test che li tiene uguali; `dock.css` con le regole che oggi stanno in `tokens.css`; `tokens/index.ts` coi due caratteri; il tema sulla radice (`tokens/theme.ts`) e il campo `theme` del pacchetto; i nomi nuovi negli undici componenti che li usano; `tokens.css` esce. Le prove: il contrasto per famiglie, gli stessi ruoli nei due temi, nessuna scala fuori dai token, nessun colore a mano, il tema nelle due direzioni | — | ⬜ |
| **2** | **il browser dei test**: `@vitest/browser-playwright` e `playwright`, i due progetti di `vitest` — jsdom e browser, sul Chrome installato e senza finestra — e il comando `emulateMedia`. Le prime prove vere: il browser è vero, i caratteri caricati con le cifre tabulari, il movimento ridotto nelle due direzioni, il contorno del focus sotto l'alto contrasto, il tema che segue il sistema | — | ⬜ |
| **3** | **il kit**: `lucide`; `BaseIcon` con `icons.ts`, `BaseButton`, `BaseLabel`, `BaseList`, `BaseStatus`, `BaseTextField`, `BaseRadioGroup`, `BaseDialog`; un test con `axe` ciascuno; il linter che legge anche i `.ts`, e le due regole sui pezzi di base | — | ⬜ |
| **4** | **la pagina kit**: `gui/kit.html` e `gui/src/kit/`, **fuori** dal pacchetto e provata sull'uscita del *build*; le prove nel browser sulla pagina kit, nei due temi — raggi concentrici, testo tagliato e niente che sborda, icone disegnate e centrate, `axe` col contrasto | — | ⬜ |
| **5** | **il kit al lavoro**: i pezzi di base nei pannelli e nella cornice — la finestra di conferma, il cassetto, la fascia, Stato, Permessi, Passi, Impostazioni con la scelta del tema, il segnaposto, la barra; le due regole del linter su `panels/` e `frame/`; **M-3** chiusa per costruzione, e l'Assistente vocale a mano | — | ⬜ |
| **6** | **il dock vestito**: il tema `dockview-theme-harness`, `dock.css` con ogni variabile del tema di riferimento, i gruppi come schede, `readToken`, `--z-floating`, la presa grande coi pezzi di base; `themeAbyss` esce | — | ⬜ |
| **7** | **le viste col nome, sotto**: `named` e `openNamed` nel pacchetto, il negozio e il dock che le mostrano e le salvano; l'aiutante della geometria estratto da `moveActive.ts`; lo schema di una disposizione, per le miniature | — | ⬜ |
| **8** | **la cornice**: la barra col nome della vista, la **Panoramica** su `BaseDialog` con le miniature e *«Salva questa vista»* — F3, frecce, Invio, Esc —, la **striscia** a pillola coi «moduli»; le prove nel browser della Panoramica e della striscia | — | ⬜ |
| **9** | **la chiusura**: la riga «Accessibilità» di tracciabilità, la riga 14 della roadmap con *«Perché quest'ordine»*, `README.md`, la §12 e la §6 del compendio, `porta-di-qualita.md`, `riferimenti.md`; la **Definizione di «fatto»**, coi comandi | — | ⬜ |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del [`COMPENDIO.md`](../../COMPENDIO.md). Qui resta la
**posizione** — la tabella, che chi esegue aggiorna nel commit del compito — e **come** si esegue.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro — e le sezioni del disegno che nomina.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-23.
3. Se il compito dice il falso, **ci si ferma e si riporta**: una divergenza è una voce d'errata prima di essere un rimedio.
   ⚠️ Vale per il codice dettato: un `npm run build` o un `npm test` rosso sul testo dettato è una voce d'errata col testo
   corretto, **non** un aggiustamento silenzioso.
4. Il cancello gira **prima** di ogni commit, da solo; il commit dice ciò che il compito ha fatto.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; per i compiti **4, 5, 6 e 8** apre
   la SPA — o la pagina kit — nel browser, nei due temi, e **guarda**: un verde non prova che una cosa si veda.
6. Una seconda ondata di **sola prosa** la chiude il coordinatore a mano; dopo due ondate di prosa si chiude (gotcha #76).
7. ⛔ **Le scritture in parallelo non si fanno**: un compito per volta.

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota.** La riempiono il pre-controllo e l'esecuzione: una voce per difetto trovato, col testo corretto, la data e
chi l'ha trovata. Ciò che la **scrittura** del piano ha trovato sta nella sezione dopo, perché è già dentro i compiti.

| # | Voce |
|---|---|

---

## Ciò che la scrittura del piano ha trovato — e i compiti già portano

Ogni riga è una misura del 2026-09-23 contro il codice di `fd2812b`, col comando o il file letto; dove il disegno diceva
altro, la riga lo dice e il compito segue la misura (`CLAUDE.md`: *«un'evidenza scritta prima della misura è un'ipotesi»*).

| # | Che cosa | Il comando o il file | Che cosa fa il piano |
|---|---|---|---|
| **P-1** | ⛔ **la regola del contrasto non è «ogni testo su ogni fondo».** La decisione 14 dice che il test accoppia *«ogni `--color-text-*` con ogni `--color-bg-*`»*; alla lettera **fallisce per costruzione** — nello scuro `--color-text-on-ok` e `--color-bg` sono lo stesso `--ref-neutral-5`. Le **176 coppie** approvate con la tavola, *«zero sotto la soglia»*, le ha prodotte la funzione `pairs()` di `palette.py`, per **famiglie**: sei testi su undici fondi a 4,5:1; `text-on-accent` sui tre fondi d'accento e `text-on-X` sul suo `bg-X` a 4,5:1; bordo forte, focus, segno e bordo d'accento sui quattro fondi di base a 3:1 — 88 per tema | `palette.py` nello scratchpad della terza sessione del 2026-09-23, `…\9ae312b8-b35c-470a-90ee-519a47a2f7df\scratchpad\`, righe 92–104 — ⚠️ uno scratchpad non è per sempre: la regola è copiata **qui**, parola per parola, nel compito 1 | il compito 1 scrive le famiglie **dai nomi**, non una lista di coppie, e una guardia che **ogni** ruolo sia in una famiglia o nell'elenco degli esenti: un ruolo nuovo non sfugge in silenzio. ✅ **Scelto dal proprietario il 2026-09-23 — A, per famiglie** (R1-8 della [revisione](2026-09-23-design-system-revisione/ledger.md)): il controllo 2 della (f) cambia forma col richiamo datato nel disegno, e il commento di `themes.css` nella tavola — *«Every text role reads 4.5:1 on every background role»* — è riscritto per famiglie **prima** che il compito 1 lo copi |
| **P-2** | ⛔ **il linter non legge i `.ts`.** `eslint src` passa i `.vue` e i `.json`; i `.ts` non sono di nessuno (P-101 della parte 2, nel commento di `gui/eslint.config.js`). Le regole di `no-restricted-imports` della (b) sarebbero **cieche** su `icons.ts`, `BigTab.ts`, `dock.ts` | `grep -n 'P-101' gui/eslint.config.js`; `vue/essential/rules` non ha `files`, e così i blocchi nostri — letto nei pacchetti installati | il compito 3 aggiunge un blocco che legge i `.ts` col parser di TypeScript, e prova che il `npm run lint` resti verde sul codice di oggi e diventi rosso su una violazione in un `.ts` |
| **P-3** | ⚠️ **la regola «in `components/` nessun import di `pinia`, `stores/`…» morderebbe `Confirm.vue`**, che la (b) stessa tiene in `components/` come pezzo **composto** e che legge due negozi | `grep -n 'stores' gui/src/components/Confirm.vue` | ✅ **scelto dal proprietario il 2026-09-23 — A, due regole** (R2-11 della revisione): i **pezzi di base** — `components/Base*.vue` e `components/icons.ts` — non importano `pinia`, `stores/` né gli strati sopra, com'è la riga *«i pezzi di base … mai lo stato globale»* della (b); **e** tutto `components/`, tranne i `*.test.ts`, non importa gli strati sopra — `panels/`, `frame/`, `transport/`. Il compito 3 scrive le due e le prova nelle due direzioni; nel disegno il richiamo datato sulla tabella *«Le regole»* della (b) |
| **P-4** | **`base.css` uguale alla tavola non può portare la regola della pagina** `html, body, #app { height: 100%; margin: 0; }`, che oggi sta in `tokens.css` | `sed -n '52,57p' gui/src/tokens/tokens.css` | il compito 1 la sposta in uno `<style>` **non** scoped di `App.vue`, la radice che monta la cornice |
| **P-5** | **i caratteri non possono entrare da `base.css`**, per la stessa ragione: la tavola li carica con `<link>` da jsDelivr | `grep -n 'fontsource' docs/superpowers/specs/2026-09-22-design-system-tavole/token.html` | li importa `gui/src/tokens/index.ts`, l'ingresso unico dei token, per la SPA e per la pagina kit |
| **P-6** | **le variabili `--dv-*` del CSS di `dockview` sono 103**, ma quelle d'una tavolozza — `--dv-color-abyss*`, `-gh-`, `-mocha-`, `-monokai-`, `-nord-`, `-sol-` — le legge solo il tema che le porta. *«Ogni variabile che il CSS usa»* si legge come **ogni variabile che il tema di riferimento `.dockview-theme-abyss` imposta**, nei suoi due blocchi | `grep -o 'var(--dv-[a-z0-9-]*' gui/node_modules/dockview/dist/styles/dockview.css \| sort -u \| wc -l`; `awk '/^\.dockview-theme-abyss \{/,/^\}/' …` | il compito 6 lo prova con un test che legge `dockview.css` e `dock.css`: ogni variabile del tema di riferimento è impostata dal nostro, tranne i nove colori dei gruppi di linguette, una funzione che la SPA non accende. ⚠️ **Richiamo del 2026-09-23, dalla scrittura del compito 6 (P-15):** le famiglie fuori sono due, non una — anche le cinque misure dei gruppi di linguette e la tavolozza `--dv-color-*` |
| **P-7** | **lo spazio attorno al dock lo decide chi chiama `layout`**: `updateTheme` di `dockview-core` 8.3.1 applica `gap` come margine della griglia, e nel JavaScript `--dv-spacing-padding` non compare | `grep -n 'spacing-padding' gui/node_modules/dockview-core/dist/package/main.esm.mjs` non rende nulla; `updateTheme` alla riga che `grep -n 'updateTheme() {'` dà sullo stesso file | il compito 6 mette il margine sul contenitore `.dock` — 0 in alto, 12 ai lati, 24 in basso, la risposta 20 — e passa a `api.layout` il **contenuto** del contenitore |
| **P-8** | **il radio di `reka-ui` 2.10.4 lo decide chi lo controlla**: `RadioGroupItem` chiama `changeModelValue`, e con un `modelValue` dato la scelta si vede solo quando il valore torna; l'evento `select` si può fermare con `preventDefault` | `gui/node_modules/reka-ui/dist/RadioGroup/RadioGroupItem.js` e `Radio.js`, letti | il compito 5 lega la policy VRAM al valore del core: il radio si muove quando torna `Policy`, e le righe scritte a mano per E184 non servono più — le prove di `modules.test.ts` si riscrivono su `[role=radio]`, senza perdere ciò che provano (trappola 10) |
| **P-9** | **una carta della Panoramica e il pulsante «moduli» della striscia sono pulsanti**, e la regola del linter vieta `<button>` in `frame/` e `panels/`: servono a `BaseButton` due forme — `variant="card"` e `pill` — la seconda **imposta dalla regola dei raggi**: dentro una pillola va una pillola | (b), (d); la tavola dello stile, `.m-strip .sp` | il compito 3 le mette in `BaseButton`, con la seconda occorrenza scritta accanto: le carte delle viste e *«Salva questa vista»*; il pulsante della striscia e il chip del core |
| **P-10** | **il logo «harness» delle tavole non è nella (d)**, che elenca da sinistra il nome della vista, la ricerca e il chip | la tabella *«La barra»* della (d); `panoramica.html`, `.m-logo` | non si costruisce: il nome del programma non è deciso — «Agentic OS» è il nome del proprietario per il programma, *harness* quello del repository |
| **P-11** | **F3 è libero** in `gui/src`; i `Ctrl+1`, `Ctrl+2` della tavola della Panoramica sono esempi che la (d) non prende | `grep -rn 'F3' gui/src` non rende nulla; la (d): *«con F3 o col clic … le frecce muovono, Invio entra, Esc chiude»* | il compito 8 costruisce F3, frecce, Invio ed Esc, e le carte non mostrano scorciatoie |
| **P-12** | **jsdom non ha `matchMedia`** | `grep -rn 'matchMedia' gui/node_modules/jsdom/lib \| head -1` non rende nulla | il tema si prova con un `matchMedia` finto sotto jsdom (compito 1) e con la query vera nel browser (compito 2) |
| **P-13** | la **provenienza** della chat usa `var(--warn)` come bordo, e il ruolo nuovo per un bordo che deve leggersi a 3:1 non c'è: `--color-border-warn` è una tinta di decoro | `grep -n 'provenance' gui/src/panels/Chat.vue`; la (a), le famiglie di P-1 | il compito 1 usa `--color-text-warn` per quel bordo, detto accanto: supera 4,5:1 sui fondi, quindi 3:1; e sono **le parole** a portare la provenienza (trappola 7) |
| **P-14** | ⚠️ **il pre-controllo va in una sessione sua**: il punto 7 del *«Come si riprende»* del disegno lo metteva *«nella sessione che scrive il piano»*; `CLAUDE.md`, rivisto dopo lo stesso giorno, dice *«brainstorming, disegno, piano, pre-controllo, ogni compito: ciascuno nella sua sessione»* | la riga *«Una fase per sessione»* di `CLAUDE.md` nasce in `eab020d`, dopo il disegno di `386fc5c`: `git show -s --format='%h %ci' eab020d 386fc5c` | vale `CLAUDE.md`: il pre-controllo è la fase dopo |
| **P-15** | ⛔ **la prova di P-6 lascia fuori due famiglie, non una**: oltre ai nove colori dei gruppi di linguette, le loro **cinque misure** — le leggono soltanto `.dv-tab-group-chip`, la sua continuazione, la linea del gruppo, il campione del menu dei colori e il colore nell'elenco del trabocco: la stessa funzione, spenta —; e la **tavolozza** `--dv-color-*` di ogni tema, che nominano soltanto i blocchi dei temi | da un file, per i backslash: `awk '/\{$/{sel=$0} /--dv-color-/{print sel}' gui/node_modules/dockview/dist/styles/dockview.css \| sort -u \| grep -vc '^\.dockview-theme-'` → **0** blocchi fuori da un tema, e senza l'ultimo filtro i blocchi sono più di zero; `awk '/\{$/{sel=$0} /var\(--dv-tab-group-/{print sel}' gui/node_modules/dockview/dist/styles/dockview.css \| sort -u` → le regole dei gruppi | il compito 6: la prova esclude `--dv-tab-group-*` e `--dv-color-*`, e il suo commento dice perché |
| **P-16** | ⛔ **sotto jsdom un `.css` importato è vuoto, e un `gap` letto da un token assente avvelena la disposizione**: `readToken("--space-3")` vale `""`, `parseFloat` rende `NaN`, e `dockview-core` 8.3.1 lo prende senza errori — poi `toJSON()` dice `null` per larghezza, altezza e ogni misura, e un `settle` lo salverebbe nel pacchetto del core | una sonda di diagnosi del 2026-09-23 sulla cartella di prova, sotto jsdom: il token importato vale `""`, lo stesso da uno `<style>` vale `12px`; `createDockview` con `gap: NaN` → `"width":null,"height":null` in `toJSON()` | il compito 6: `readToken` lancia un errore per un token assente, e `src/jsdom-setup.ts` carica `base.css` letto dal file (D9) |
| **P-17** | ⚠️ **i gruppi galleggianti si impilano per pagina**: `AriaLevelTracker`, un oggetto unico del modulo in `dockview-core` 8.3.1, dà a ogni contenitore galleggiante `calc(var(--dv-overlay-z-index, 999) + 2i)` nell'ordine in cui è stato alzato l'ultima volta, e lo toglie dalla lista solo quando il gruppo è **smontato** | `grep -n 'overlay-z-index' gui/node_modules/dockview-core/dist/package/main.esm.mjs`; nel browser, una prova che non smontava il dock del primo tema ha misurato **52** nel secondo | il compito 6 scrive il limite in `dock.css` — coi valori della tavola, sotto `--z-popover` fino a 25 gruppi aperti e sotto `--z-overlay` fino a 75 — e la sua prova smonta il dock con `api.dispose()` |
| **P-18** | ⚠️ **`axe` sul dock**: il contrasto non lo giudica — nessuna coppia fra i `passes`, le scritte fra gli `incomplete` con *«overlapped by another element»* — e trova **tre difetti che vengono dalla parte 2**: `nested-interactive` su ogni linguetta, perché i due comandi della presa grande sono pulsanti dentro un `role="tab"`; il nome di ogni linguetta, `aria-label`, è il `title` del pannello o il suo id — `permissions`, non «Permessi» —, e le viste portano un `title` uguale all'id; sul contenitore galleggiante, `role="dialog"` con `aria-level`, che un dialogo non ammette, e un `aria-label` uguale all'id | `axe.run` sul dock nel Chrome installato, 2026-09-23; `role`, `aria-*` e `tabindex` di `.dv-tab` e di `.dv-resize-container`, letti sulla SPA; `grep -n '"aria-label"' gui/node_modules/dockview-core/dist/package/main.esm.mjs` | il compito 6 tiene il contrasto del dock col frammento a mano del passo 17 del compito 1; i tre difetti vanno al proprietario, in *«Le voci aperte che questo piano SA»*: toglierli cambia la presa grande di SP-8, mossa 5 |

## Le decisioni prese scrivendo il piano

Il proprietario ha scelto **A** alla domanda minima della rilettura: il piano scrive **i consigli** del disegno — le dieci
righe *«proposta»* della tabella dei controlli, le decisioni 23–27 e le precisioni 4–11. Le decisioni qui sotto sono quelle
che il disegno lasciava al piano; ciascuna si ribalta con una riga.

| # | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| **D1** | il test che tiene **`base.css` e `themes.css` uguali alla tavola** resta nel cancello, non è un controllo d'un giorno | due case per gli stessi valori sono la forma di gotcha #68; il test è ciò che le tiene d'accordo, come la seconda sonda di `copy.test.ts` tiene d'accordo la configurazione e la cartella. Costo: chi cambia un valore cambia **prima** la tavola, poi copia |
| **D2** | la scelta del tema è un campo **`theme`** del pacchetto, e chi la applica è un `watchTheme` in `gui/src/tokens/theme.ts` | la risposta 16. Il tema **mostrato** — `light` o `dark` — sta in un `ref` di quel modulo, che il dock legge per il suo `colorScheme`. Costo: uno stato di modulo, uno solo |
| **D3** | le **viste col nome** sono due campi facoltativi del pacchetto, `named` e `openNamed`; `view` resta un `ViewName` | un pacchetto scritto prima si apre come prima, e uno scritto dopo si apre in una build vecchia sulla vista di sempre: cambiare il tipo di `view` avrebbe rotto la seconda direzione. Costo: due campi da leggere insieme |
| **D4** | due nomi uguali per una vista col nome **non** si accettano: il campo lo dice sotto, in rosso | lo stato d'errore del campo è nella tavola dei token, `.field.is-error`; sovrascrivere in silenzio perderebbe una vista. Costo: rinominare e cancellare una vista col nome non ci sono — non li chiede la (d) |
| **D5** | le **miniature** sono uno schema calcolato dall'albero della disposizione salvata — rettangoli in frazioni e l'icona del modulo visibile — e i gruppi galleggianti non ci sono | la risposta 19 vuole lo schema, non un `dockview` per miniatura; un gruppo galleggiante non ha un posto nella griglia. Costo: una miniatura non mostra ciò che galleggia |
| **D6** | le **icone dei moduli** che le tavole non mostrano le sceglie il piano, una riga ciascuna nella mappa — la **Chat** compresa | le tavole danno Stato, Permessi, Passi, Attività, i moduli, la ricerca, le viste; la Chat no, e `message-square`, che il piano le dà, sulle tavole segna i «Messaggi di stato» (R2-15 della revisione); la presa grande vuole l'icona di **ogni** tipo di modulo. Costo: una riga per cambiarne una |
| **D7** | la pagina kit mostra i due temi **uno alla volta**, con la scelta in cima, e non affiancati | un dialogo di `reka-ui` va in un portale sul `body` e prenderebbe il tema della radice, non quello della colonna. Costo: per confrontarli si cambia la scelta |
| **D8** | le parole della pagina kit sono **esemplari** scritti nel file, con un blocco del linter che lo dice, limitato a `src/kit/` | è una pagina di sviluppo fuori dal pacchetto: le sue parole in `it.json` finirebbero nel pacchetto per niente. Costo: una eccezione in più, in un posto solo |
| **D9** | **`readToken` lancia un errore per un token che la pagina non definisce, e sotto jsdom `base.css` si carica dal file**, in `src/jsdom-setup.ts` | P-16: un token assente in silenzio avvelena la disposizione salvata; leggere il file e non ricopiare i valori tiene la casa unica della tavola. Solo `base.css`: nessuna prova sotto jsdom legge un colore. Costo: ogni prova jsdom ha i token di `base.css` sulla radice — misurato, nessuna delle altre cambia esito |
| **D10** | **il bordo della zona d'arrivo è `--dv-drag-over-border` in `dock.css`**, e il tema TypeScript non porta `dndOverlayBorder` | le due vie erano aperte (R3-22); `updateTheme` di 8.3.1 lascia la variabile al foglio quando il campo manca, e un colore scritto in TypeScript sarebbe una seconda casa. Costo: il bordo non lo vede nessuna prova automatica; si guarda al passo 8 del compito, trascinando una linguetta |
| **D11** | **la faccia della presa grande è un `.vue`**, `frame/BigTabFace.vue`, che `BigTab.ts` monta come `VueContent` monta un pannello | una funzione `h()` dentro `BigTab.ts` avrebbe fatto lo stesso, ma fuori dalla vista del linter dei template — la trappola 5 in un'altra forma; col `.vue` le regole di `harness/panels-and-frame` e di `no-raw-text` la leggono. Costo: un'app Vue per linguetta, smontata in `dispose` e provata |

## Le voci aperte che questo piano SA, e non chiude

Rilette il 2026-09-23 coi due comandi della §6 del compendio e con la tabella dell'audit: nessuna voce aperta dei Traguardi
5 e 6 ha per chiusore la GUI o questo lavoro, e nessuna è del tipo *«prima di questo traguardo»*.

| Voce | Di chi | Che cosa ne fa questo piano |
|---|---|---|
| **X-2** e **X-4** dell'[audit](../../audit-2026-08-27.md) | del proprietario | niente: non toccano la GUI |
| **N-2 di E187**, l'avviso di `vite` sui pezzi sopra i 500 kB | del proprietario | il compito 1, il 5, il 6 e l'8 **misurano** il pezzo JavaScript dopo il *build* — `npm run build 2>&1 \| grep -E 'assets/index-.*\.js '` — e lo scrivono nel commit; il 3 no, perché la SPA non importa ancora il kit (R2-13). ⚠️ La (e) dice che il design system non lo peggiora, ed è una deduzione: la revisione l'ha misurato **crescere al compito 5**, quando i pezzi di base entrano nei pannelli (R3-25), e la cifra si porta al proprietario |
| **E228**, progress e notifiche | del proprietario | niente: i token ci sono già, `--z-toast` e i colori di stato |
| il **terzo carattere** per il codice | del proprietario | niente: il monospazio resta quello del sistema (decisione 16 del disegno) |
| **AUD-004** | del proprietario | niente: sbarra il sotto-progetto 13, non questo |
| la **finestra** del guscio — `titleBarOverlay`, `setTitleBarOverlay`, gli angoli di Windows; e la **prima pittura**: i ruoli vivono solo sotto `[data-theme]`, che lo script mette dopo l'analisi del documento, quindi prima dello script la finestra non ha fondo (R1-13 della revisione, dedotto e non misurato) | del sotto-progetto **10** | niente: le regole della (d) restano scritte nel disegno per chi farà il guscio; per la prima pittura il rimedio è del guscio — il `backgroundColor` della finestra, o mostrarla a `ready-to-show` |
| i **tre difetti di accessibilità** che `axe` trova sul dock — `nested-interactive` sulle linguette, il nome della linguetta uguale all'id del pannello, `aria-level` e `aria-label` sul contenitore galleggiante (P-18) | del proprietario | niente: vengono dalla parte 2 e da `dockview-core`, e toglierli cambia la presa grande giudicata con SP-8 — i comandi fuori dalla linguetta, un `title` in italiano dentro le viste salvate. Il compito 6 li misura e non li nasconde: nessuna prova `axe` sul dock con quelle regole spente |

---
## Compito 1: i token — la tavola copiata, il tema sulla radice, i nomi nuovi

**Da:** la (a) del disegno — *I file*, *Due livelli*, *I colori*, *Il resto: `base.css`*, *I due temi*, *I caratteri*, *Il
passaggio dai nomi di oggi* — e i controlli **1–7** della tabella del prodotto. **P-1, P-4, P-5, P-12, P-13** di questo piano.

**Files:**
- Create: `gui/src/tokens/base.css`, `gui/src/tokens/themes.css` — **dallo script**, mai a mano (vincolo 2)
- Create: `gui/src/tokens/board.test.ts`, `gui/src/tokens/usage.test.ts`, `gui/src/tokens/theme.ts`,
  `gui/src/tokens/theme.test.ts`, `gui/src/tokens/index.ts`, `gui/src/tokens/dock.css`
- Rewrite: `gui/src/tokens/contrast.test.ts`
- Delete: `gui/src/tokens/tokens.css`
- Modify: `gui/src/main.ts`, `gui/src/App.vue`, `gui/src/stores/layout.ts`, `gui/src/stores/stores.test.ts`, e gli
  **undici** componenti che il censimento del passo 1 nomina
- Modify: `gui/package.json`, `gui/package-lock.json` — i due caratteri
- Modify: `docs/COMPENDIO.md` — la riga della §6 che nomina `tokens.css`

**Interfaces:**
- Produces: `gui/src/tokens/theme.ts` —
  `type ThemeChoice = "system" | "light" | "dark"`, `type Theme = "light" | "dark"`,
  `THEME_CHOICES: readonly ThemeChoice[]`, `isThemeChoice(value: unknown): value is ThemeChoice`, `DARK_QUERY: string`,
  `resolveTheme(choice: ThemeChoice, systemIsDark: boolean): Theme`, `shownTheme: Ref<Theme>`,
  `watchTheme(choice: () => ThemeChoice, root?: HTMLElement, matchMedia?: (query: string) => MediaQueryList): () => void`.
- Produces: in `useLayout()` il getter `theme: ThemeChoice` e l'azione `chooseTheme(choice: ThemeChoice): void`; in
  `LayoutPack` il campo `theme?: ThemeChoice`.
- Produces: `gui/src/tokens/index.ts`, l'ingresso unico dei token — lo importano `main.ts` e, dal compito 4, la pagina kit.
- Produces: i nomi dei ruoli `--color-*` e `--shadow-*` e di `base.css`, per tutti i compiti dopo.

- [ ] **Passo 1: rimisura il punto di partenza**

Da un file, non in linea (trappola 6 del disegno): i backslash dei comandi si perdono.

```bash
cat > /tmp/census-1.sh <<'EOF'
cd "$(git rev-parse --show-toplevel)" || exit 1
grep -rnoE 'var\(--(ink|ink-dim|surface|surface-raised|line|accent|warn|stop|radius|font|font-size|line-height)\)' gui/src --include=*.vue --include=*.css --include=*.ts | sort
grep -rn --include='*.vue' -E '#[0-9a-fA-F]{3,8}\b|rgba?\(|hsla?\(' gui/src
grep -rn 'z-index' gui/src --include=*.vue
EOF
bash /tmp/census-1.sh
```

Atteso, il 2026-09-23: i nomi vecchi in `tokens.css` e in undici componenti — `Confirm`, `Band`, `Drawer`, `ViewBar`,
`Chat`, `Permissions`, `Placeholder`, `Settings`, `Status`, `Steps`, `Strip`; i due veli, in `Confirm.vue` e `Drawer.vue`;
i due `z-index` di `Drawer.vue`. Se il censimento dice altro, è una voce d'errata **prima** di andare avanti. Poi, da solo,
`bash scripts/gate.sh` → `GATE GREEN`: la baseline.

- [ ] **Passo 2: i due caratteri, in due passi**

```bash
(cd gui &&
  npm install --save-exact @fontsource-variable/geist@5.3.0 @fontsource/barlow@5.3.0 &&
  git diff --stat package.json package-lock.json &&
  npm ls @fontsource-variable/geist @fontsource/barlow)
```

Atteso: le due righe nuove in `dependencies` di `package.json`, esatte, e `dockview-core` che `npm` rimette in ordine dopo
`dockview` — il diff di `package.json` è `4 +++-` (R1-4) —; e il lockfile che le porta. La licenza di tutte e
due è `OFL-1.1` (`npm view @fontsource/barlow@5.3.0 license`). Manifesto e lockfile vanno nello **stesso** commit
(vincolo 7).

- [ ] **Passo 3: le prove, prima del codice — la tavola**

Crea `gui/src/tokens/board.test.ts` (LF):

```ts
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const HERE = dirname(fileURLToPath(import.meta.url));
const BOARD = join(HERE, "..", "..", "..", "docs", "superpowers", "specs", "2026-09-22-design-system-tavole", "token.html");

/** The three markers of the approved board, verbatim: the plan's script cut the two files between them. */
const BASE = "/* ===== proposta/base.css ===== */";
const THEMES = "/* ===== proposta/themes.css ===== */";
const END = "/* ===== the board itself, only tokens ===== */";

/** LF, no blank line at the start, no whitespace at the end, one LF after: the terminator belongs to the
 * checkout, not to the tokens -- the files are CRLF on one machine and LF on another. */
function normal(text: string): string {
  return `${text.replace(/\r\n/g, "\n").replace(/^\n+/, "").replace(/\s+$/, "")}\n`;
}

function between(text: string, start: string, end: string): string {
  const from = text.indexOf(start);
  const to = text.indexOf(end);
  if (from < 0 || to < from) throw new Error(`the board lost a marker: ${start} … ${end}`);
  return text.slice(from + start.length, to);
}

/**
 * ⛔ TWO HOUSES FOR THE SAME VALUES, AND THIS PROBE IS WHAT KEEPS THEM AGREEING (D1 of the design-system plan):
 * the values live in the approved board -- decision 23 of the design -- and the SPA ships a copy. Whoever
 * changes a colour changes the board FIRST, then copies; a copy edited by hand turns this red.
 */
describe("the token files", () => {
  const board = readFileSync(BOARD, "utf8");

  it.each([
    ["base.css", BASE, THEMES],
    ["themes.css", THEMES, END],
  ])("%s is the board's block, byte for byte", (file, start, end) => {
    const block = normal(between(board, start, end));
    // ⛔ NON-VACUITY: two empty cuts would agree with two empty files.
    expect(block.length).toBeGreaterThan(1000);
    expect(normal(readFileSync(join(HERE, file), "utf8"))).toBe(block);
  });
});
```

- [ ] **Passo 4: le prove — il contrasto per famiglie, e gli stessi ruoli nei due temi**

Riscrivi `gui/src/tokens/contrast.test.ts` per intero, col terminatore che il file ha oggi (la testa del piano dice come):

```ts
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const css = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "themes.css"), "utf8");

/** Every `--name: value;` of the block that opens with `selector {`. ⛔ READ FROM THE FILE AND NOT RETYPED:
 * a probe that carried its own copy of the palette would go on passing after someone edits the real one. */
function block(selector: string): Record<string, string> {
  const start = css.indexOf(`${selector} {`);
  if (start < 0) throw new Error(`themes.css has no block ${selector}`);
  const body = css.slice(start, css.indexOf("}", start));
  return Object.fromEntries([...body.matchAll(/--([a-z0-9-]+):\s*([^;]+);/g)].map((found) => [found[1] ?? "", (found[2] ?? "").trim()]));
}

const SCALES = block(":root");
const THEMES: Record<string, Record<string, string>> = {
  dark: block('[data-theme="dark"]'),
  light: block('[data-theme="light"]'),
};

/** A role down to its `#rrggbb`, through the scales and through other roles; `null` for what is not a plain
 * colour -- a veil with alpha, `transparent`, a shadow. */
function colour(roles: Record<string, string>, value: string, depth = 0): string | null {
  if (/^#[0-9a-fA-F]{6}$/.test(value)) return value;
  const name = /^var\(--([a-z0-9-]+)\)$/.exec(value)?.[1];
  if (name === undefined || depth > 4) return null;
  const next = SCALES[name] ?? roles[name];
  return next === undefined ? null : colour(roles, next, depth + 1);
}

/** WCAG 2.2 relative luminance, and the contrast ratio built on it. */
function luminance(hex: string): number {
  const channel = (index: number): number => {
    const value = Number.parseInt(hex.slice(index, index + 2), 16) / 255;
    return value <= 0.03928 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5);
}

export function contrast(foreground: string, background: string): number {
  const [high, low] = [luminance(foreground), luminance(background)].sort((a, b) => b - a) as [number, number];
  return (high + 0.05) / (low + 0.05);
}

/**
 * ⛔ THE FAMILIES OF THE 176 PAIRS APPROVED WITH THE BOARD (P-1 of the design-system plan), as rules on NAMES
 * and not as a list of pairs. `pairs()` of the board's generator, `palette.py`, was:
 *   text 4.5:1     every text role but the disabled one and the `on-*` ones, on the base backgrounds, their
 *                  fills and every subtle tint
 *   on 4.5:1       `text-on-X` on `bg-X` and on its hover and active states
 *   non-text 3:1   border-strong, focus, mark, border-accent -- on bg, bg-surface, bg-raised, bg-fill
 * "Every text on every background" -- the words of decision 14 of the design -- fails by construction: in
 * the dark theme `--color-text-on-ok` IS `--color-bg`. One pair is added here, and it passes: `--color-text`
 * on `--color-bg-selection`, the pair `::selection` draws in `base.css`.
 */
function families(names: string[]) {
  const texts = names.filter((n) => n.startsWith("color-text") && n !== "color-text-disabled" && !n.startsWith("color-text-on-"));
  const grounds = names.filter((n) => /^color-bg(-surface|-raised|-fill(-hover|-active)?|-[a-z]+-subtle(-hover)?)?$/.test(n));
  const onPairs = names
    .filter((n) => n.startsWith("color-text-on-"))
    .flatMap((on) => {
      const state = on.slice("color-text-on-".length);
      return names
        .filter((n) => n === `color-bg-${state}` || n === `color-bg-${state}-hover` || n === `color-bg-${state}-active`)
        .map((bg): [string, string] => [on, bg]);
    });
  const marks = ["color-border-strong", "color-focus", "color-mark", "color-border-accent"].filter((n) => names.includes(n));
  const surfaces = ["color-bg", "color-bg-surface", "color-bg-raised", "color-bg-fill"].filter((n) => names.includes(n));
  return { texts, grounds, onPairs, marks, surfaces };
}

/** The roles no pair judges, each with its reason -- the design's (a), "I colori". */
const EXEMPT: Record<string, string> = {
  "color-text-disabled": "WCAG 1.4.3 exempts an inactive component",
  "color-border": "decoration: it is not what tells a control apart",
  "color-border-card": "decoration, and `transparent` in the light theme",
  "color-border-ok": "decoration of a message, whose icon and words carry the state",
  "color-border-warn": "decoration of a message, whose icon and words carry the state",
  "color-border-stop": "decoration of a message, whose icon and words carry the state",
  "color-veil": "a translucent veil, not a surface a text sits on",
  "shadow-card": "a shadow, not a colour",
  "shadow-overlay": "a shadow, not a colour",
};

describe("the two themes", () => {
  it("carry the same roles, by name", () => {
    expect(Object.keys(THEMES.dark ?? {}).sort()).toEqual(Object.keys(THEMES.light ?? {}).sort());
  });

  for (const [theme, roles] of Object.entries(THEMES)) {
    const names = Object.keys(roles);
    const f = families(names);
    const pairs: [string, string, number][] = [
      ...f.texts.flatMap((text) => f.grounds.map((ground): [string, string, number] => [text, ground, 4.5])),
      ...f.onPairs.map(([text, ground]): [string, string, number] => [text, ground, 4.5]),
      ["color-text", "color-bg-selection", 4.5],
      ...f.marks.flatMap((mark) => f.surfaces.map((surface): [string, string, number] => [mark, surface, 3])),
    ];

    it(`${theme}: every family is there -- so that a green below means something`, () => {
      expect(f.texts.length).toBeGreaterThan(0);
      expect(f.grounds.length).toBeGreaterThan(0);
      expect(f.onPairs.length).toBeGreaterThan(0);
      expect(f.marks).toHaveLength(4);
      expect(f.surfaces).toHaveLength(4);
    });

    it(`${theme}: every pair reads at its threshold (WCAG 2.2, 1.4.3 and 1.4.11)`, () => {
      const failing: string[] = [];
      for (const [foreground, background, need] of pairs) {
        const a = colour(roles, roles[foreground] ?? "");
        const b = colour(roles, roles[background] ?? "");
        if (a === null || b === null) {
          failing.push(`${foreground} on ${background}: not a plain colour`);
          continue;
        }
        const ratio = contrast(a, b);
        if (ratio < need) failing.push(`${foreground} on ${background}: ${ratio.toFixed(2)} < ${need}`);
      }
      expect(failing).toEqual([]);
    });

    it(`${theme}: every role is judged by a family, or exempt with its reason`, () => {
      // ⛔ THE GUARD AGAINST A SILENT ESCAPE: a role added tomorrow in no family would never be judged.
      const judged = new Set([...f.texts, ...f.grounds, ...f.onPairs.flat(), ...f.marks, ...f.surfaces, "color-bg-selection"]);
      expect(names.filter((name) => !judged.has(name) && !(name in EXEMPT))).toEqual([]);
    });
  }
});
```

⚠️ **La regola delle famiglie è copiata da `palette.py`**, righe 92–104, lette il 2026-09-23 nello scratchpad della terza
sessione (P-1): è questo commento a portarla da qui in avanti.

- [ ] **Passo 5: le prove — nessuna scala fuori dai token, nessun colore a mano**

Crea `gui/src/tokens/usage.test.ts` (LF):

```ts
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const SRC = dirname(dirname(fileURLToPath(import.meta.url)));

/** Every file under `gui/src` ending in one of the extensions, relative to `gui/src`, with `/`. */
function sources(extensions: readonly string[]): string[] {
  const found: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) walk(path);
      else if (extensions.some((extension) => entry.name.endsWith(extension))) found.push(relative(SRC, path).split(sep).join("/"));
    }
  };
  walk(SRC);
  return found.sort();
}

const read = (file: string): string => readFileSync(join(SRC, file), "utf8");

/** The design-system rules on how the tokens are USED -- controls 4 and 5 of the design. */
describe("the token discipline", () => {
  const all = sources([".vue", ".ts", ".css"]);

  it("sees the files it judges", () => {
    // ⛔ NON-VACUITY: a walk that found nothing would pass both probes below.
    expect(all.filter((file) => file.endsWith(".vue")).length).toBeGreaterThan(10);
    expect(all).toContain("tokens/dock.css");
  });

  it("keeps the scales to `tokens/`: no component reads a `--ref-*`", () => {
    expect(all.filter((file) => !file.startsWith("tokens/") && read(file).includes("var(--ref-"))).toEqual([]);
  });

  it("writes no colour by hand outside the token files", () => {
    const HAND = /#[0-9a-fA-F]{3,8}\b|rgba?\(|hsla?\(/;
    const judged = all.filter((file) => file.endsWith(".vue") || file === "tokens/dock.css");
    const offenders = judged.flatMap((file) =>
      read(file)
        .split(/\r?\n/)
        .flatMap((line, index) => (HAND.test(line) ? [`${file}:${index + 1}: ${line.trim()}`] : [])),
    );
    expect(offenders).toEqual([]);
  });
});
```

- [ ] **Passo 6: le prove — il tema sulla radice**

Crea `gui/src/tokens/theme.test.ts` (LF):

```ts
import { describe, expect, it } from "vitest";
import { nextTick, ref } from "vue";

import { DARK_QUERY, resolveTheme, shownTheme, watchTheme, type ThemeChoice } from "./theme";

/** A system that says dark or light and can change its mind. jsdom has no `matchMedia` (P-12 of the plan);
 * the real query is proven in the browser, task 2. */
function system(dark: boolean) {
  const listeners = new Set<() => void>();
  const queries: string[] = [];
  const list = {
    matches: dark,
    addEventListener: (_type: string, listener: () => void) => listeners.add(listener),
    removeEventListener: (_type: string, listener: () => void) => listeners.delete(listener),
  };
  return {
    queries,
    listeners,
    matchMedia: (query: string): MediaQueryList => {
      queries.push(query);
      return list as unknown as MediaQueryList;
    },
    turn(darkNow: boolean): void {
      list.matches = darkNow;
      for (const listener of [...listeners]) listener();
    },
  };
}

describe("the theme on the root (design system, section (a))", () => {
  it("follows the system while the choice is `system`, and stops following when stopped", () => {
    const os = system(true);
    const root = document.createElement("div");
    const stop = watchTheme((): ThemeChoice => "system", root, os.matchMedia);
    expect(os.queries).toEqual([DARK_QUERY]);
    expect(root.dataset.theme).toBe("dark");
    expect(shownTheme.value).toBe("dark");
    os.turn(false);
    expect(root.dataset.theme).toBe("light");
    expect(shownTheme.value).toBe("light");
    stop();
    // ⛔ A stopped watch that still listened would fight the next one.
    expect(os.listeners.size).toBe(0);
  });

  it("lets `light` and `dark` win over the system, and moves when the choice does", async () => {
    const os = system(true);
    const root = document.createElement("div");
    const choice = ref<ThemeChoice>("light");
    const stop = watchTheme(() => choice.value, root, os.matchMedia);
    expect(root.dataset.theme).toBe("light");
    os.turn(false);
    os.turn(true);
    // ⛔ THE SECOND DIRECTION: the system changed twice, and the owner's choice held.
    expect(root.dataset.theme).toBe("light");
    choice.value = "system";
    await nextTick();
    expect(root.dataset.theme).toBe("dark");
    choice.value = "dark";
    await nextTick();
    os.turn(false);
    expect(root.dataset.theme).toBe("dark");
    stop();
  });

  it("resolves the three choices", () => {
    expect(resolveTheme("system", true)).toBe("dark");
    expect(resolveTheme("system", false)).toBe("light");
    expect(resolveTheme("light", true)).toBe("light");
    expect(resolveTheme("dark", false)).toBe("dark");
  });
});
```

- [ ] **Passo 7: le prove — il tema nel pacchetto, nelle due direzioni**

In `gui/src/stores/stores.test.ts`, in coda al file — con Python, `newline=""`, e il blocco con `\r\n` se il file è
CRLF, come un file riscritto per intero (R1-14) —, un `describe` nuovo:

```ts
describe("the theme in the package (design system, section (a))", () => {
  it("keeps the choice a package carries, and opens one without it as `system`", () => {
    const layout = useLayout();
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "home", layouts: {}, theme: "light" })] } });
    expect(layout.theme).toBe("light");
    // ⛔ THE SECOND DIRECTION: a package written before the field existed still opens -- as `system`.
    setActivePinia(createPinia());
    const older = useLayout();
    older.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "work", layouts: {} })] } });
    expect(older.view).toBe("work");
    expect(older.theme).toBe("system");
  });

  it("reads a choice it does not know as absent, without refusing the package", () => {
    const bytes = [...new TextEncoder().encode('{"view":"home","layouts":{},"theme":"purple"}')];
    expect(unpack({ state: "Package", bytes })).toEqual({ view: "home", layouts: {} });
  });

  it("sends the choice at once, and a settle after it keeps it", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    layout.chooseTheme("dark");
    const first = bridge.sent[0];
    const chosen = first?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: first.value }) : null;
    expect(chosen).toEqual({ view: "home", layouts: {}, theme: "dark" });
    expect(layout.theme).toBe("dark");
    const home = { marker: "home, as the owner left it" } as never;
    layout.settle(home);
    const second = bridge.sent[1];
    const settled = second?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: second.value }) : null;
    // ⛔ NOT "something was sent": a settle that rebuilt the package from `view` and `layouts` alone would drop
    // the choice at the first move of a panel.
    expect(settled).toEqual({ view: "home", layouts: { home }, theme: "dark" });
  });
});
```

- [ ] **Passo 8: lancia le prove, e guardale fallire**

```bash
(cd gui && npx vitest run src/tokens src/stores)
```

Atteso: **rosso**, e per la ragione giusta — `board.test.ts` e `contrast.test.ts` senza `base.css` e `themes.css`
(`ENOENT`), `usage.test.ts` senza `tokens/dock.css` e coi due veli di oggi, in `Confirm.vue` e `Drawer.vue`, che il passo
13 rinomina (R1-2), `theme.test.ts` senza `./theme`, e **due** delle tre prove del negozio — la prima e la terza — senza
`theme` e `chooseTheme`. La seconda, la scelta sconosciuta, è **verde** già oggi, perché l'`unpack` di oggi non legge
`theme`: la sua direzione rossa è al passo 16 (R1-1). Un rosso per un'altra ragione è una voce d'errata.

- [ ] **Passo 9: i due fogli, copiati dallo script**

Scrivi `extract_tokens.py` nello **scratchpad**, non nel repository:

```python
"""extract_tokens.py -- cut base.css and themes.css out of the approved token board (design system, task 1).

Usage: python extract_tokens.py <repository root>

Writes gui/src/tokens/base.css and gui/src/tokens/themes.css, LF: each is the text between two markers of
docs/superpowers/specs/2026-09-22-design-system-tavole/token.html, with the leading blank lines and the
trailing whitespace removed and one LF at the end -- the very cut board.test.ts makes.
"""
import io
import os
import sys

root = sys.argv[1]
board_path = os.path.join(root, "docs", "superpowers", "specs", "2026-09-22-design-system-tavole", "token.html")
board = io.open(board_path, encoding="utf-8", newline="").read().replace("\r\n", "\n")
MARKS = [
    "/* ===== proposta/base.css ===== */",
    "/* ===== proposta/themes.css ===== */",
    "/* ===== the board itself, only tokens ===== */",
]
for mark in MARKS:
    if board.count(mark) != 1:
        sys.exit(f"refused: {mark!r} is in the board {board.count(mark)} times")


def cut(start, end):
    text = board[board.index(start) + len(start):board.index(end)]
    return text.lstrip("\n").rstrip() + "\n"


for name, (start, end) in {"base.css": MARKS[0:2], "themes.css": MARKS[1:3]}.items():
    out = os.path.join(root, "gui", "src", "tokens", name)
    with io.open(out, "w", encoding="utf-8", newline="") as f:
        f.write(cut(start, end))
    print(f"ok: {out} ({len(cut(start, end))} characters)")
```

```bash
python <scratchpad>/extract_tokens.py "$(git rev-parse --show-toplevel)"
(cd gui && npx vitest run src/tokens/board.test.ts src/tokens/contrast.test.ts)
```

Atteso: `board.test.ts` **verde**, due prove; `contrast.test.ts` **verde** — gli stessi ruoli, e per ciascun tema le
famiglie, le coppie e nessun ruolo senza giudice. ⛔ Se una coppia fallisce, **non si ritocca un valore**: è la tavola
approvata che sbaglia, o la regola copiata qui, e si porta al proprietario (vincolo 1).

- [ ] **Passo 10: il tema — `theme.ts`, e il campo del pacchetto**

Crea `gui/src/tokens/theme.ts` (LF):

```ts
import { ref, watch, type Ref } from "vue";

/** What the owner can choose in Impostazioni (answer 5): the system's theme, or one of the two. */
export type ThemeChoice = "system" | "light" | "dark";
/** What is shown: the value of `data-theme` on the root, the attribute `themes.css` reads. */
export type Theme = "light" | "dark";

export const THEME_CHOICES: readonly ThemeChoice[] = ["system", "light", "dark"];

export function isThemeChoice(value: unknown): value is ThemeChoice {
  return typeof value === "string" && (THEME_CHOICES as readonly string[]).includes(value);
}

/** The query Electron's page follows while `nativeTheme.themeSource` is `system`, its default -- the (a),
 * row "di base", read at the source on 2026-09-23. */
export const DARK_QUERY = "(prefers-color-scheme: dark)";

export function resolveTheme(choice: ThemeChoice, systemIsDark: boolean): Theme {
  if (choice === "system") return systemIsDark ? "dark" : "light";
  return choice;
}

/** The theme on screen, for whoever draws outside CSS: the dock's `colorScheme` today (task 6), a canvas
 * tomorrow (D2 of the design-system plan). ONE module state, written only by `watchTheme`. */
export const shownTheme: Ref<Theme> = ref<Theme>("dark");

/**
 * Puts `data-theme` on the root and keeps it there: when the choice changes, and -- while the choice is
 * `system` -- when the system changes. Returns the stop.
 *
 * ⛔ EVERY ROLE OF `themes.css` LIVES UNDER `[data-theme]`: a root without the attribute has no colour at all,
 * so the caller runs this BEFORE THE MOUNT, and Vue's first render has its colours (`main.ts` does). The paint
 * before any script runs is the shell's to cure (R1-13 of the design-system review).
 */
export function watchTheme(
  choice: () => ThemeChoice,
  root: HTMLElement = document.documentElement,
  matchMedia: (query: string) => MediaQueryList = (query) => window.matchMedia(query),
): () => void {
  const system = matchMedia(DARK_QUERY);
  const apply = (): void => {
    const theme = resolveTheme(choice(), system.matches);
    root.dataset.theme = theme;
    shownTheme.value = theme;
  };
  system.addEventListener("change", apply);
  const stop = watch(choice, apply, { immediate: true });
  return () => {
    stop();
    system.removeEventListener("change", apply);
  };
}
```

In `gui/src/stores/layout.ts` (CRLF su questa macchina: `replace_unique.py`), quattro sostituzioni.

*Trova:*

```ts
import type { IpcMessage, LayoutState } from "../schema/messages";
import type { Bridge } from "../transport/bridge";
```

*Sostituisci con:*

```ts
import type { IpcMessage, LayoutState } from "../schema/messages";
import { isThemeChoice, type ThemeChoice } from "../tokens/theme";
import type { Bridge } from "../transport/bridge";
```

*Trova:*

```ts
export interface LayoutPack {
  view: ViewName;
  layouts: Partial<Record<ViewName, SerializedDockview>>;
}
```

*Sostituisci con:*

```ts
export interface LayoutPack {
  view: ViewName;
  layouts: Partial<Record<ViewName, SerializedDockview>>;
  /** ⛔ OPTIONAL, AND THAT IS THE COMPATIBILITY (answer 16 of the design system): a package written before
   * the field existed opens as `system`, and the core keeps the bytes without opening them -- the kernel
   * does not change. */
  theme?: ThemeChoice;
}
```

*Trova* (la funzione `settle` e il `return` del negozio, interi):

```ts
  function settle(layout: SerializedDockview): void {
    const pack: LayoutPack = {
      view: view.value,
      layouts: { ...(saved.value?.layouts ?? {}), [view.value]: layout },
    };
    saved.value = pack;
    const bytes = [...pack_(pack)];
    sent = bytes;
    wire?.send({ kind: "SaveLayout", value: bytes });
  }

  return { state, view, saved, arrivals, attach, receive, settle };
```

*Sostituisci con:*

```ts
  function settle(layout: SerializedDockview): void {
    // ⛔ THE REST OF THE PACKAGE IS KEPT: a settle that rebuilt it from `view` and `layouts` alone would drop
    // the theme -- and, from task 7, the named views -- at the first move of a panel.
    keep({
      ...(saved.value ?? {}),
      view: view.value,
      layouts: { ...(saved.value?.layouts ?? {}), [view.value]: layout },
    });
  }

  /** The theme of the package, and `system` when it has none (answer 16). */
  const theme = computed<ThemeChoice>(() => saved.value?.theme ?? "system");

  /** ⛔ SAVED AT ONCE, NOT AT THE NEXT SETTLE: a choice made in Impostazioni is a decision, not a movement of
   * panels, and closing the window right after it must not lose it. */
  function chooseTheme(choice: ThemeChoice): void {
    keep({ layouts: {}, ...(saved.value ?? {}), view: view.value, theme: choice });
  }

  function keep(pack: LayoutPack): void {
    saved.value = pack;
    const bytes = [...pack_(pack)];
    sent = bytes;
    wire?.send({ kind: "SaveLayout", value: bytes });
  }

  return { state, view, saved, arrivals, theme, attach, receive, settle, chooseTheme };
```

*Trova:*

```ts
    return { view: candidate.view, layouts };
```

*Sostituisci con:*

```ts
    // ⛔ A CHOICE THIS BUILD DOES NOT KNOW IS READ AS ABSENT, and the package still opens: the layouts in it are
    // worth more than a word we cannot read.
    const theme = (candidate as { theme?: unknown }).theme;
    return isThemeChoice(theme) ? { view: candidate.view, layouts, theme } : { view: candidate.view, layouts };
```

E `computed` entra nell'import di `vue` della stessa riga: *Trova* `import { ref } from "vue";` — *Sostituisci con*
`import { computed, ref } from "vue";`.

- [ ] **Passo 11: l'ingresso dei token, il foglio del dock, la pagina**

Crea `gui/src/tokens/index.ts` (LF):

```ts
// The one entry of the design tokens (design system, section (a)), for the SPA and for the kit page.
//
// ⛔ THE FONTS ARE IMPORTED HERE AND NOT FROM `base.css` (P-5 of the plan): `base.css` is the board's block
// byte for byte, and the board loads its fonts from jsDelivr with <link>. Only what the tokens name ships:
// Barlow 300 (large numbers), 400, 500 (the strip's button), 600 (labels); Geist is variable, one file per
// subset. Nothing is fetched at run time (answer 6).
import "@fontsource-variable/geist";
import "@fontsource/barlow/300.css";
import "@fontsource/barlow/400.css";
import "@fontsource/barlow/500.css";
import "@fontsource/barlow/600.css";

import "./base.css";
import "./themes.css";
import "./dock.css";
```

Crea `gui/src/tokens/dock.css` (LF) — le regole che `tokens.css` porta per la cornice, coi nomi della (a); il compito 6
lo riscrive come il tema nostro:

```css
/* The frame's rules that no SFC dresses -- design system, task 1: moved here from `tokens.css`, which
   leaves, with the names of the (a). ⛔ Task 6 rewrites this file as OUR `dockview` theme; until then
   `themeAbyss` stays, and these rules keep the grab of SP-8 on it.

   ⛔ `frame/BigTab.ts` and `frame/VueContent.ts` build plain DOM, because `dockview` asks for a RENDERER and
   not a component: the classes they name -- `bigtab`, `bigtab-title`, `panel` -- live here and not in a
   scoped <style> (E165, E166 of the part-2 plan). */

/* ⛔ THE GRAB IS THE CONTAINER'S, NOT THE TAB'S (E170): `dockview` sizes the whole tab strip from
   `--dv-tabs-and-actions-container-height` and clips `.dv-tabs-container`, so the VARIABLE moves and the tab
   fills it. 40 px is the grab move 5 of SP-8 was judged on, and the (a) now names it. */
.dock .dockview-theme-abyss {
  --dv-tabs-and-actions-container-height: var(--size-control-lg);
}

/* ⛔ A BRIDGE UNTIL TASK 6 DRESSES THE DOCK: `themeAbyss` paints the groups and the tabs dark, and the light theme's
   text would sit on them at 1.11:1, the big grab's buttons at 1.06:1 (measured). These bind the few variables that
   paint behind text to our roles; task 6 replaces this block with a theme of our own. It wins over `dockview.css`
   because it is loaded after it. */
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

/* ⛔ No vertical padding on the strip's tabs, so the grab is the whole height; and the CHILD combinator,
   because the overflow dropdown renders `.dv-tab` rows inside the dock too (E173). */
.dock .dv-tabs-container > .dv-tab {
  padding-top: 0;
  padding-bottom: 0;
}

.bigtab {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 100%;
  padding: 0 10px;
  font-weight: 600;
  cursor: grab;
  user-select: none;
}

.bigtab-title {
  flex: 1;
}

/* The element `VueContent` hands to `dockview`: without a height of its own, every `height: 100%` inside a
   panel resolves against nothing. */
.panel {
  height: 100%;
}

/* The two commands of the big grab handle. */
.bigtab button {
  font: inherit;
  width: 26px;
  height: 26px;
  background: var(--color-bg-raised);
  color: inherit;
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-control);
  cursor: pointer;
}
```

Riscrivi `gui/src/App.vue` per intero, col terminatore che ha oggi:

```vue
<script setup lang="ts">
import Frame from "./frame/Frame.vue";
</script>

<template>
  <Frame />
</template>

<!-- ⛔ NOT SCOPED, AND THE ONE PAGE RULE OUTSIDE THE TOKENS (P-4 of the design-system plan): the page is the
     frame's, and `base.css` is the board's block byte for byte, so this rule cannot live there. -->
<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
}
</style>
```

- [ ] **Passo 12: `main.ts` — i token, e il tema prima del *mount***

In `gui/src/main.ts`, *Trova* `import "./tokens/tokens.css";` — *Sostituisci con* `import "./tokens";`. Poi *Trova*:

```ts
import { useStream } from "./stores/stream";
```

*Sostituisci con:*

```ts
import { useStream } from "./stores/stream";
import { watchTheme } from "./tokens/theme";
```

Poi *Trova*:

```ts
const app = createApp(App);
app.use(createPinia());
app.use(i18n);
app.mount("#app");

const connection = useConnection();
const core = useCore();
const layout = useLayout();
const invoke = useInvoke();
const stream = useStream();
```

*Sostituisci con:*

```ts
const app = createApp(App);
app.use(createPinia());
app.use(i18n);

const connection = useConnection();
const core = useCore();
const layout = useLayout();
const invoke = useInvoke();
const stream = useStream();

// ⛔ BEFORE THE MOUNT, so Vue's first render already has its colours: every role of `themes.css` lives under
// `[data-theme]` (design system, section (a)). Until the core's package arrives the choice is `system`.
watchTheme(() => layout.theme);

app.mount("#app");

```

⚠️ Il testo nuovo finisce con una riga vuota, così il `mount` non si attacca al blocco degli `attach` che lo segue (R1-15).

- [ ] **Passo 13: i nomi nuovi nei componenti, da uno script**

Il passaggio della (a), *«Il passaggio dai nomi di oggi»*, come tabella — ⛔ **meccanico tranne dove un token porta un
significato** (trappola 7):

| Vecchio | Nuovo | Dove, e perché |
|---|---|---|
| `1px solid var(--line)` | `var(--border-width) solid var(--color-border)` | cinque bordi |
| `border-bottom: 1px solid var(--warn)` | `border-bottom: var(--border-width) solid var(--color-border-warn)` | la fascia: un bordo di decoro, le parole dicono l'avviso |
| `3px solid var(--warn)` | `3px solid var(--color-text-warn)` | ⛔ la **provenienza** della chat (G13): un bordo che deve leggersi a 3:1, e `--color-border-warn` è decoro — **P-13** |
| `var(--warn)` | `var(--color-text-warn)` | ogni altro uso è un testo |
| `var(--ink-dim)` | `var(--color-text-muted)` | |
| `var(--surface-raised)` | `var(--color-bg-raised)` | |
| `var(--accent)` | `var(--color-text-accent)` | due testi: il chip collegato e il «sì» della conferma |
| `var(--stop)` | `var(--color-text-stop)` | il chip del timbro diverso |
| `var(--radius)` | `var(--radius-control)` | |
| `rgb(0 0 0 / 50%)`, `rgb(0 0 0 / 0.45)` | `var(--color-veil)` | i due veli, già diversi fra loro: il ruolo li cura |
| `z-index: 100;`, `z-index: 101;` | `z-index: var(--z-overlay);` | il cassetto: velo e contenuto, nell'ordine del DOM |

Scrivi `rename_tokens.py` nello **scratchpad**:

```python
"""rename_tokens.py -- the design-system renames of task 1, with the count each one must find.

Usage: python rename_tokens.py <repository root>

Applies the table below IN ORDER to every .vue file under gui/src, keeping each file's line endings, and
refuses -- writing nothing at all -- when a rename does not find exactly the count it expects.
"""
import io
import os
import sys

root = sys.argv[1]
src = os.path.join(root, "gui", "src")
RENAMES = [
    ("1px solid var(--line)", "var(--border-width) solid var(--color-border)", 5),
    ("border-bottom: 1px solid var(--warn)", "border-bottom: var(--border-width) solid var(--color-border-warn)", 1),
    ("3px solid var(--warn)", "3px solid var(--color-text-warn)", 1),
    ("var(--warn)", "var(--color-text-warn)", 5),
    ("var(--ink-dim)", "var(--color-text-muted)", 13),
    ("var(--surface-raised)", "var(--color-bg-raised)", 4),
    ("var(--accent)", "var(--color-text-accent)", 2),
    ("var(--stop)", "var(--color-text-stop)", 1),
    ("var(--radius)", "var(--radius-control)", 4),
    ("rgb(0 0 0 / 50%)", "var(--color-veil)", 1),
    ("rgb(0 0 0 / 0.45)", "var(--color-veil)", 1),
    ("z-index: 100;", "z-index: var(--z-overlay);", 1),
    ("z-index: 101;", "z-index: var(--z-overlay);", 1),
]
files = sorted(
    os.path.join(d, name) for d, _, names in os.walk(src) for name in names if name.endswith(".vue")
)
texts = {path: io.open(path, encoding="utf-8", newline="").read() for path in files}
for old, new, expected in RENAMES:
    found = sum(text.count(old) for text in texts.values())
    if found != expected:
        sys.exit(f"refused: {old!r} found {found} times, expected {expected} -- nothing written")
    texts = {path: text.replace(old, new) for path, text in texts.items()}
for path, text in texts.items():
    if text != io.open(path, encoding="utf-8", newline="").read():
        with io.open(path, "w", encoding="utf-8", newline="") as f:
            f.write(text)
        print(f"ok: {os.path.relpath(path, root)}")
```

```bash
python <scratchpad>/rename_tokens.py "$(git rev-parse --show-toplevel)"
bash /tmp/census-1.sh
```

Atteso: undici righe `ok:`; poi il censimento non trova più **nessun** nome vecchio fuori da `tokens.css` — che esce al
passo 14, e fino ad allora ne porta dieci righe (R1-3) — né **nessun** colore a mano, e i due `z-index` dicono
`var(--z-overlay)`. ⚠️ Lo script scrive su file CRLF senza toccarne i terminatori, perché legge e scrive
con `newline=""` e le sue sostituzioni non contengono fine-riga: si rimisura lo stesso, al passo 16.

- [ ] **Passo 14: `tokens.css` esce, e ciò che lo nominava**

```bash
git rm gui/src/tokens/tokens.css
```

⚠️ Il commento di `gui/src/a11y.test.ts` che nomina `contrast.test.ts` resta com'è: lo toglie il compito 3, che porta
quell'aiutante in un modulo suo.

In `docs/COMPENDIO.md`, *Trova* la riga che comincia con `Lo stile di oggi è un **segnaposto dichiarato**`, **intera, presa
dal file** — *Sostituisci con*:

```markdown
Lo stile dal compito 1 del piano del design system: `gui/src/tokens/base.css` e `themes.css`, copie della tavola dei token.
```

⚠️ Il margine del compendio prima e dopo, col comando del vincolo 13.

- [ ] **Passo 15: tutte le prove, il *build*, il linter**

```bash
(cd gui && npm test && npm run build && npm run lint)
```

Atteso: **verde** su tutto; nel log del *build*, la riga del pezzo JavaScript — `npm run build 2>&1 | grep -E
'assets/index-.*\.js '`: `grep 'kB'` ne rende una per ogni file di carattere (R1-5) — che si scrive nel commit accanto a
quella della baseline (**N-2 di E187**: il design system non dovrebbe peggiorarlo, ed è una deduzione). Il CSS cresce
delle dichiarazioni `@font-face`; i caratteri sono **file a parte**, `dist/assets/*.woff2` e `*.woff`.

- [ ] **Passo 16: le due direzioni, e i fine-riga**

Ogni prova si prova **rossa**, poi si torna indietro con la **copia salvata**, e alla fine `git status --porcelain` è
quello di prima (vincolo 11): `themes.css` e `theme.ts` sono nati in questo compito, e `Strip.vue` ne porta già i nomi
nuovi — `git checkout` non conosce i primi e toglierebbe al secondo il lavoro del passo 13 (A-1):

| La prova | La violazione messa a mano | Atteso |
|---|---|---|
| `board.test.ts` | in `themes.css` un gradino `--ref-bordeaux-28` cambiato di una cifra | rosso: `themes.css is the board's block` |
| `contrast.test.ts` | in `themes.css` `--color-text-muted: var(--ref-neutral-44);` nello scuro | rosso, e il messaggio nomina `color-text-muted on …` |
| `contrast.test.ts`, la guardia | in `themes.css` un ruolo nuovo `--color-bg-test: var(--ref-neutral-5);` nei due temi | rosso: `color-bg-test` non ha giudice — ⚠️ attenzione, `color-bg-test` finisce fra i fondi solo se la regola lo prende: che cosa dice il rosso si **legge** |
| `usage.test.ts`, le scale | in `panels/Strip.vue` `color: var(--ref-neutral-5);` | rosso: `panels/Strip.vue` |
| `usage.test.ts`, i colori | in `panels/Strip.vue` `color: #fff;` | rosso: `panels/Strip.vue:<riga>` |
| `theme.test.ts` | in `theme.ts` `resolveTheme` che ignora `choice` | rosso |
| `stores.test.ts`, la scelta sconosciuta | in `unpack` il ramo che rende `theme` senza `isThemeChoice` | rosso: `reads a choice it does not know as absent`, con `"theme": "purple"` (R1-1) |
| `stores.test.ts`, la scelta che resta | in `settle` tolto `...(saved.value ?? {})` | rosso: `sends the choice at once, and a settle after it keeps it` (R1-1) |

Poi, dalla radice del repository (R1-12), i fine-riga dei file toccati — quelli **nuovi** LF, gli altri come erano:

```bash
git ls-files --eol gui/src gui/package.json gui/package-lock.json docs/COMPENDIO.md | grep -v 'w/lf\|w/crlf'
for f in $(git diff --name-only; git ls-files --others --exclude-standard gui/src); do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
```

- [ ] **Passo 17: guardarlo, nei due temi**

`(cd gui && npm run dev)`, la pagina nel browser; nella console `harnessFake.deliverAll()`, poi, per ciascun tema —
`document.documentElement.dataset.theme = "light"` e `"dark"` —, il contrasto di ogni scritta del dock sul primo fondo
pieno dei suoi antenati:

```js
(() => {
  const rgb = (c) => c.match(/[\d.]+/g).map(Number);
  const lum = ([r, g, b]) => [r, g, b].map((v) => ((v /= 255) <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4)).reduce((s, v, i) => s + v * [0.2126, 0.7152, 0.0722][i], 0);
  const ratio = (a, b) => (Math.max(lum(a), lum(b)) + 0.05) / (Math.min(lum(a), lum(b)) + 0.05);
  const back = (el) => { for (let e = el; e; e = e.parentElement) { const c = rgb(getComputedStyle(e).backgroundColor); if (c.length === 3 || c[3] === 1) return c; } return [255, 255, 255]; };
  const own = (el) => [...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim() !== "");
  const seen = [...document.querySelectorAll(".dock *")].filter((el) => own(el) && el.getBoundingClientRect().width > 0);
  const worst = seen.map((el) => [Math.round(ratio(rgb(getComputedStyle(el).color), back(el)) * 100) / 100, el.textContent.trim().slice(0, 20)]).sort((a, b) => a[0] - b[0]);
  return { theme: document.documentElement.dataset.theme, seen: seen.length, worst: worst.slice(0, 3) };
})()
```

Atteso: in tutti e due i temi `seen` sopra zero e il primo di `worst` **sopra 4,5** — pannelli e linguette, i pulsanti
della presa grande compresi. ⚠️ Il dock è ancora `themeAbyss` fino al compito 6, e a tenerlo leggibile è il **ponte** di
`dock.css`: senza, nel tema chiaro il testo dei pannelli sta a 1,11:1 e i pulsanti della presa grande a 1,06:1 (misurato
dalla revisione, R1-10). Poi i caratteri: **Geist** nel testo (DevTools, *Computed*, `font-family`); Barlow entra coi
pezzi di base, dal compito 3, e i controlli nativi restano sul carattere del sistema finché il compito 5 non li
sostituisce (R1-11). ⛔ Una scritta sotto la soglia è una voce d'errata, non un valore da ritoccare.

- [ ] **Passo 18: il cancello, il commit, la posizione**

La riga **1** della tabella della posizione: **Stato** `✅ <data>`; la sua colonna **Commit** la scrive il compito 2, perché
un commit non conosce il proprio hash prima di nascere (R1-16). Poi, uno alla volta:

```bash
bash scripts/gate.sh
bash scripts/check-docs.sh
git add gui/package.json gui/package-lock.json gui/src docs/COMPENDIO.md docs/superpowers/plans/2026-09-23-design-system.md
git commit -m "design-system(compito 1): i token dalla tavola -- base.css e themes.css copiati da uno script e tenuti uguali da board.test.ts; il contrasto per famiglie (P-1), gli stessi ruoli nei due temi, nessuna scala e nessun colore a mano fuori dai token; il tema sulla radice e il campo theme del pacchetto; Geist e Barlow; i nomi nuovi negli undici componenti; tokens.css esce"
git push
```

---
## Compito 2: il browser dei test — due progetti, Chrome installato, le prime prove vere

**Da:** la (f) del disegno — *«Il browser dei test»* — e i controlli **8**, **9** e **20**; la trappola **2** (il carattere
si misura dopo averlo caricato) e la **11** (l'alto contrasto); **P-12**.

**Files:**
- Modify: `gui/package.json`, `gui/package-lock.json` — `@vitest/browser-playwright` 4.1.11 e `playwright` 1.63.0, di sviluppo
- Modify: `gui/vite.config.ts` — i due progetti e il comando `emulateMedia`
- Create: `gui/src/browser.d.ts` — il tipo del comando, per le prove
- Create: `gui/src/tokens/tokens.browser.test.ts`
- Modify: `scripts/gate-gui.sh` — il commento del passo delle prove

**Interfaces:**
- Consumes: `gui/src/tokens/index.ts` e `watchTheme` del compito 1.
- Produces: il progetto `browser` di `vitest`, che prende ogni `src/**/*.browser.test.ts`; il comando
  `commands.emulateMedia({ colorScheme?, reducedMotion?, forcedColors? })` da `vitest/browser`, dove `null` restituisce la
  caratteristica al browser; la finestra delle prove a **1440 × 900**.

⚠️ **Fonti lette il 2026-09-23 alla v4.1.11 di `vitest-dev/vitest`, su GitHub:** `docs/guide/browser/index.md` (il
fornitore, i progetti), `docs/config/browser/playwright.md` (`launchOptions`), `docs/api/browser/commands.md` (un comando
riceve `page`, *«the full page that contains the test iframe»*), `docs/api/browser/context.md` (`page.viewport`),
`docs/guide/projects.md` (`extends: true` per ereditare `plugins`), `docs/config/css.md` (*«This option is not applied to
browser tests»*: nel browser il CSS vale), `docs/config/browser/headless.md` (di base `process.env.CI`: **fuori** dalla CI
si aprirebbe una finestra), `docs/config/browser/viewport.md` (di base 414 × 896). E il tipo di `ctx.page` sta in
`@vitest/browser-playwright/dist/index.d.ts`, che aumenta `BrowserCommandContext` di `vitest/node`.

- [ ] **Passo 1: le due dipendenze, in due passi**

```bash
(cd gui &&
  npm install --save-dev --save-exact @vitest/browser-playwright@4.1.11 playwright@1.63.0 &&
  npm ls @vitest/browser-playwright playwright vitest &&
  npm view playwright@1.63.0 scripts.install scripts.postinstall)
```

Atteso: `@vitest/browser-playwright@4.1.11` che porta `@vitest/browser@4.1.11`, `vitest@4.1.11` **invariato**, e
`playwright@1.63.0` **senza** script d'installazione — l'ultimo comando non stampa nulla: **non scarica un browser**
(trappola 15). Licenze: MIT e Apache-2.0.

- [ ] **Passo 2: la prova, prima della configurazione**

Crea `gui/src/tokens/tokens.browser.test.ts` (LF):

```ts
import { commands, userEvent } from "vitest/browser";
import { afterEach, describe, expect, it } from "vitest";

import "./index";
import { watchTheme, type ThemeChoice } from "./theme";

// ⛔ THIS FILE RUNS IN THE INSTALLED CHROME (design system, section (f)): the tokens and the fonts are the
// SPA's own, imported above, and every probe asks the layout engine rather than a copy of the values.

/** Every emulated feature goes back to the browser, so a probe never inherits the previous one's. */
afterEach(async () => {
  await commands.emulateMedia({ colorScheme: null, reducedMotion: null, forcedColors: null });
  document.body.replaceChildren();
  delete document.documentElement.dataset.theme;
});

const rootStyle = (): CSSStyleDeclaration => getComputedStyle(document.documentElement);

/** `navigator.userAgentData` is not in TypeScript's `DOM` library: the shape the probe reads, written here. */
type BrandedNavigator = Navigator & { userAgentData?: { brands: readonly { brand: string }[] } };

describe("the browser the probes run in", () => {
  it("is a real one: it lays out, and it is Chrome", () => {
    // ⛔ THE NON-VACUITY OF THE WHOLE PROJECT (control 20 of the design): under jsdom every rectangle is zero,
    // so a box with a size proves a layout engine; and the channel is the installed Chrome (decision 22).
    const box = document.createElement("div");
    box.style.cssText = "width:120px;height:40px";
    document.body.append(box);
    expect(box.getBoundingClientRect().width).toBe(120);
    // ⛔ THE BRAND, NOT THE USER AGENT: `HeadlessChrome/` ends in `Chrome/`, and so would the Chromium Playwright
    // downloads; the installed Chrome is the one whose brands say "Google Chrome" (R2-6 of the design-system review).
    const brands = (navigator as BrandedNavigator).userAgentData?.brands.map((entry) => entry.brand) ?? [];
    expect(brands).toContain("Google Chrome");
  });
});

describe("the tokens, in a real browser (design system, sections (a) and (f))", () => {
  it("load both fonts, and draw the tool font's digits at one width", async () => {
    // ⛔ LOAD BEFORE MEASURING (trap 2): a face loads only when some text needs it. And the oracle is the
    // FontFace's status, not `document.fonts.check()`, which says yes for a family no @font-face declares.
    await document.fonts.load('400 14px "Geist Variable"');
    for (const weight of [300, 400, 500, 600]) await document.fonts.load(`${weight} 32px "Barlow"`);
    const loaded = (family: string): number =>
      [...document.fonts].filter((face) => face.family.replace(/"/g, "") === family && face.status === "loaded").length;
    expect(loaded("Geist Variable")).toBeGreaterThan(0);
    expect(loaded("Barlow")).toBeGreaterThanOrEqual(4);

    const width = (text: string, variant: string, family?: string): number => {
      const span = document.createElement("span");
      span.textContent = text;
      span.style.cssText = `position:absolute;white-space:nowrap;font:var(--font-display);font-variant-numeric:${variant}`;
      if (family !== undefined) span.style.fontFamily = family;
      document.body.append(span);
      return span.getBoundingClientRect().width;
    };
    expect(Math.abs(width("111111", "tabular-nums") - width("000000", "tabular-nums"))).toBeLessThan(0.5);
    // ⛔ THE SECOND DIRECTION: without `tabular-nums` the digits are proportional -- measured on the token board on
    // 2026-09-23, 67.34 against 108.10 px. It proves that `tabular-nums` acts, NOT which font draws: Bahnschrift, the
    // chain's first fallback on Windows, has proportional digits too (R2-4 of the design-system review).
    expect(Math.abs(width("111111", "normal") - width("000000", "normal"))).toBeGreaterThan(10);
    // ⛔ WHICH FONT DRAWS: the same text in the token's chain WITHOUT Barlow measures otherwise -- 67.33 against 63.84 px
    // on the plain ones, 100.42 against 102.19 on the tabular zeros, in the installed Chrome on 2026-09-23. A token
    // that drew with the fallback would measure the same in both.
    const chain = rootStyle().getPropertyValue("--font-family-tool").trim();
    const fallback = chain.replace(/^"Barlow",\s*/, "");
    expect(fallback, "the chain starts with Barlow").not.toBe(chain);
    expect(Math.abs(width("111111", "normal") - width("111111", "normal", fallback))).toBeGreaterThan(0.5);
    expect(Math.abs(width("000000", "tabular-nums") - width("000000", "tabular-nums", fallback))).toBeGreaterThan(0.5);
  });

  it("put the motion to zero when the system asks for less, and only then (WCAG 2.3.3)", async () => {
    await commands.emulateMedia({ reducedMotion: "reduce" });
    for (const name of ["--duration-fast", "--duration-moderate", "--duration-slow"]) {
      expect(rootStyle().getPropertyValue(name).trim(), name).toBe("0ms");
    }
    // ⛔ THE SECOND DIRECTION: without the request, the durations are the board's and not zero.
    await commands.emulateMedia({ reducedMotion: "no-preference" });
    expect(rootStyle().getPropertyValue("--duration-fast").trim()).not.toBe("0ms");
  });

  it("keep the focus ring under Windows' high contrast: an outline, which forced colours keep (G20, trap 11)", async () => {
    document.documentElement.dataset.theme = "dark";
    await commands.emulateMedia({ forcedColors: "active" });
    // ⛔ THE EMULATION IS IN FORCE, or nothing below is about high contrast (R2-2 of the design-system review).
    expect(matchMedia("(forced-colors: active)").matches).toBe(true);
    const button = document.createElement("button");
    button.textContent = "focus";
    document.body.append(button);
    // A KEY and not `focus()`: `:focus-visible` is the keyboard's ring, and the probe must reach it the same way.
    await userEvent.tab();
    expect(document.activeElement).toBe(button);
    const ring = (element: HTMLElement): boolean => {
      const style = getComputedStyle(element);
      return style.outlineStyle !== "none" && Number.parseFloat(style.outlineWidth) >= 2;
    };
    expect(ring(button)).toBe(true);
    // ⛔ THE SECOND DIRECTION IS WHAT HIGH CONTRAST DOES: forced colours erase a ring drawn with `box-shadow` -- the
    // reason the (a) draws it with an outline. The same button, ringed that way, has no ring left.
    button.style.outline = "none";
    button.style.boxShadow = "0 0 0 2px currentColor";
    expect(getComputedStyle(button).boxShadow).toBe("none");
  });

  it("follow the system's scheme through the real query while the choice is `system`", async () => {
    const root = document.createElement("div");
    const stop = watchTheme((): ThemeChoice => "system", root);
    await commands.emulateMedia({ colorScheme: "dark" });
    await expect.poll(() => root.dataset.theme).toBe("dark");
    await commands.emulateMedia({ colorScheme: "light" });
    await expect.poll(() => root.dataset.theme).toBe("light");
    stop();
  });
});
```

Crea `gui/src/browser.d.ts` (LF):

```ts
// The custom command of `vite.config.ts`, as the probes in the browser see it ("Custom Commands", Vitest 4.1).
// `null` gives a feature back to the browser.
export {};

declare module "vitest/browser" {
  interface BrowserCommands {
    emulateMedia: (media: {
      colorScheme?: "light" | "dark" | null;
      reducedMotion?: "reduce" | "no-preference" | null;
      forcedColors?: "active" | "none" | null;
    }) => Promise<void>;
  }
}
```

```bash
(cd gui && npx vitest run src/tokens/tokens.browser.test.ts)
```

Atteso: **rosso** — sotto jsdom, com'è la configurazione di oggi, `vitest/browser` non c'è e il file non si carica. È il
rosso giusto: la prova chiede un browser che la configurazione non ha ancora.

- [ ] **Passo 3: i due progetti, e il comando**

In `gui/vite.config.ts` (`replace_unique.py`), tre sostituzioni.

*Trova:*

```ts
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vitest/config";
```

*Sostituisci con:*

```ts
import { playwright } from "@vitest/browser-playwright";
import vue from "@vitejs/plugin-vue";
import { configDefaults, defineConfig } from "vitest/config";
import type { BrowserCommand } from "vitest/node";
```

*Trova:*

```ts
export default defineConfig({
```

*Sostituisci con:*

```ts
/** The media features a probe may emulate -- the three the design system reads. `null` gives one back. */
interface Media {
  colorScheme?: "light" | "dark" | null;
  reducedMotion?: "reduce" | "no-preference" | null;
  forcedColors?: "active" | "none" | null;
}

/**
 * ⛔ A COMMAND AND NOT A CONTEXT OPTION: `contextOptions` would fix one value for a whole file, while a probe
 * must see BOTH directions -- "reduce" and "no-preference" -- in one test. `page` is the page that holds the
 * test iframe (Vitest 4.1, "Custom Commands"), and `emulateMedia` applies to its frames.
 */
const emulateMedia: BrowserCommand<[media: Media]> = async ({ page }, media) => {
  await page.emulateMedia(media);
};

export default defineConfig({
```

*Trova:*

```ts
  test: {
    // ⛔ `jsdom` FROM THIS TASK ON: task 11 ran on `node` because nothing it built touched a DOM,
    // and said so. The frame mounts components, so it needs one.
    environment: "jsdom",
    include: ["src/**/*.test.ts"],
    // ⛔ `jsdom` 30.0.1 has no `ResizeObserver`, and `dockview-core` wants one the moment a grid is
    // created: the fake in this file is what lets a probe mount a grid at all (R6-8).
    setupFiles: ["src/jsdom-setup.ts"],
  },
```

*Sostituisci con:*

```ts
  test: {
    /**
     * ⛔ TWO PROJECTS, ONE COMMAND (design system, section (f)): `npm test` runs both, so the gate's "probes"
     * step keeps its shape. `extends: true` hands each one the `plugins` and the `define` above; everything else
     * is written per project, because an inline project inherits nothing it does not ask for.
     */
    projects: [
      {
        extends: true,
        test: {
          name: "jsdom",
          // ⛔ `jsdom` since task 11 of part 2: the frame mounts components, so it needs a DOM.
          environment: "jsdom",
          include: ["src/**/*.test.ts"],
          exclude: [...configDefaults.exclude, "src/**/*.browser.test.ts"],
          // ⛔ `jsdom` 30.0.1 has no `ResizeObserver`, and `dockview-core` wants one the moment a grid is
          // created: the fake in this file is what lets a probe mount a grid at all (R6-8).
          setupFiles: ["src/jsdom-setup.ts"],
        },
      },
      {
        extends: true,
        test: {
          name: "browser",
          include: ["src/**/*.browser.test.ts"],
          browser: {
            enabled: true,
            // ⛔ EXPLICIT: the default is `process.env.CI`, which would open a window on every local gate.
            headless: true,
            // ⛔ NO PICTURES OF A RED (R2-3 of the design-system review): by default every failing probe leaves a PNG in
            // `__screenshots__` next to its file -- inside `src/`, which git does not ignore, on a path Windows' git cannot
            // add past 260 characters. Nobody looks at them: the gate reads the words of the failure.
            screenshotFailures: false,
            // ⛔ THE INSTALLED CHROME (decision 22 of the design): nothing is downloaded, and a machine without
            // it goes red at this step with Playwright's own message -- the prerequisite, declared.
            provider: playwright({ launchOptions: { channel: "chrome" } }),
            instances: [{ browser: "chromium" }],
            // The width the approved boards were probed at; the default is a phone's, 414 x 896.
            viewport: { width: 1440, height: 900 },
            commands: { emulateMedia },
          },
        },
      },
    ],
  },
```

- [ ] **Passo 4: le prove, verdi**

```bash
(cd gui && npx vitest run --project browser && npm test && npm run build)
```

Atteso: il progetto `browser` **verde**, cinque prove; il conto dei file e delle prove di `npm test` più alto di quello
della baseline del compito 1 di **un** file e **cinque** prove; `npm run build` verde — `vue-tsc` legge anche
`vite.config.ts` e `browser.d.ts`. E i **due** progetti, che il reporter di base non nomina quando tutto è verde (R2-7) —
da un file, per i backslash (trappola 6 del disegno):

```bash
cat > /tmp/projects-2.sh <<'EOF'
cd "$(git rev-parse --show-toplevel)/gui" || exit 1
npx vitest run --reporter=verbose | grep -oE '\|(jsdom|browser \(chromium\))\|' | sort | uniq -c
EOF
bash /tmp/projects-2.sh
```

Atteso: due righe, `|browser (chromium)|` e `|jsdom|`, ciascuna col suo conto sopra zero.

- [ ] **Passo 5: le due direzioni del cancello**

| La prova | La violazione | Atteso |
|---|---|---|
| il browser **non parte** → il passo è **rosso**, non verde (controllo 20) | in `vite.config.ts` `channel: "chrome-that-does-not-exist"` | `npm test` esce **diverso da zero**, col messaggio di Playwright |
| il browser è vero | in `vite.config.ts` il progetto `browser` con `enabled: false` | rosso: `vitest/browser can be imported only inside the Browser Mode. Your test is running in forks pool.` — fuori dal browser il file non si carica, ed è l'import a fare da guardia per questa riga, non il rettangolo (R2-5) |
| i caratteri | in `tokens/index.ts` tolta la riga di Barlow 300 | rosso: `loaded("Barlow")` sotto quattro, o le cifre |
| il movimento | in `base.css` tolto il blocco `prefers-reduced-motion` | rosso — ⛔ poi `base.css` torna dalla **copia salvata**, e `board.test.ts` lo conferma verde |
| l'alto contrasto | nella prova del contorno del focus, tolta la riga `await commands.emulateMedia({ forcedColors: "active" });` | rosso alla guardia, `expected false to be true`: senza l'emulazione la prova non parla dell'alto contrasto (R2-2) |
| il carattere che disegna | in `base.css` `--font-display` col solo ripiego, `"Bahnschrift", system-ui, sans-serif` al posto di `var(--font-family-tool)` | rosso al confronto del token con la sua catena senza Barlow, `expected 0 to be greater than 0.5`, mentre le due direzioni delle larghezze restano verdi: anche Bahnschrift ha le cifre proporzionali, e uguali con `tabular-nums` (R2-4) — ⛔ e `board.test.ts`, che vuole il foglio della tavola: poi `base.css` torna dalla **copia salvata** |

Ogni violazione torna indietro con la copia salvata; poi, dalla radice del repository, `git status --porcelain` è quello
di prima della prima violazione (vincolo 11): nessun file nato dai rossi del browser (R2-3).

- [ ] **Passo 6: il cancello, e il suo commento**

In `scripts/gate-gui.sh`, *Trova*:

```bash
echo "-------- gui: probes"
npm test
```

*Sostituisci con:*

```bash
echo "-------- gui: probes"
# ⛔ TWO PROJECTS IN ONE COMMAND (design system, task 2): jsdom, and the INSTALLED Chrome for what only a layout
# engine can judge -- fonts, motion, radii, clipping, the contrast of the drawn page. Nothing is downloaded
# (decision 22 of the design): a machine without Google Chrome goes red here with Playwright's message, and that
# is a prerequisite of the environment, like the `rustup` target and `cargo audit`.
npm test
```

- [ ] **Passo 7: il cancello, il commit, la CI**

La riga **2** della tabella della posizione: **Stato** `✅ <data>`; e nella riga **1** la colonna **Commit** con l'hash del
compito 1 (R1-16). Poi `bash scripts/gate.sh`, da solo, e
`bash scripts/check-docs.sh`; il commit coi file del compito e il piano; `git push`. ⛔ **La CI si legge**, coi due comandi
di `docs/porta-di-qualita.md`, *«Leggere la CI da terra»*: è la prima corsa col browser su `ubuntu-latest` e
`windows-latest`, e un rosso là è una voce d'errata di questo compito, non del prossimo.

---
## Compito 3: il kit — gli otto pezzi di base, la mappa delle icone, le regole del linter

**Da:** la (b) del disegno — *Dove vive*, *I pezzi di base — otto*, *La forma di un pezzo di base*, *`BaseIcon` e la mappa*,
*`BaseStatus`*, *Le regole, come controlli del linter* — i controlli **10–13**; le decisioni **18–20** del disegno; **P-2,
P-3, P-8, P-9** e **D6** di questo piano. L'aspetto dei pezzi viene dalla tavola dei token, dalle regole `.btn`, `.field`,
`.row`, `.dlg` del suo foglio (`grep -n '^\.btn\|^\.field\|^\.row\|^\.dlg' docs/superpowers/specs/2026-09-22-design-system-tavole/token.html`),
riscritte coi **soli** token.

**Files:**
- Modify: `gui/package.json`, `gui/package-lock.json` — `lucide` 1.47.0
- Create: `gui/src/components/icons.ts`, `BaseIcon.vue`, `BaseButton.vue`, `BaseLabel.vue`, `BaseList.vue`,
  `BaseStatus.vue`, `BaseTextField.vue`, `BaseRadioGroup.vue`, `BaseDialog.vue` — tutti in `gui/src/components/`
- Create: `gui/src/components/kit.test.ts`, `gui/src/testing/axe.ts`
- Modify: `gui/src/a11y.test.ts` — l'aiutante di `axe` passa in `testing/axe.ts`, seconda occorrenza
- Modify: `gui/eslint.config.js` — il blocco dei `.ts` e le regole sugli import

**Interfaces:**
- Consumes: i token del compito 1.
- Produces, per i compiti 4–8:
  - `ICONS`, `type IconName = keyof typeof ICONS`, `isIconName(name: string): name is IconName` da `components/icons.ts`;
  - `BaseIcon` — props `name: IconName`, `size?: "sm" | "md" | "lg"`;
  - `BaseButton` — props `variant?: "primary" | "secondary" | "quiet" | "card"`, `size?: "sm" | "md" | "lg"`, `pill?`,
    `disabled?`, `icon?: IconName`, `label?: string` (il nome del pulsante che è **solo** un'icona); slot `default`; gli
    attributi e gli ascoltatori di chi lo usa vanno sul `<button>`, che è la radice;
  - `BaseLabel` — props `icon?: IconName`, `as?: "span" | "h2" | "h3" | "h4"`; slot `default`;
  - `BaseList<T>` — props `items: readonly T[]`, `keyOf: (item: T, index: number) => string | number`, `ordered?`; slot
    `item({ item, index })`;
  - `BaseStatus` — slot `default`; la radice è un `div role="status"` sempre presente;
  - `BaseTextField` — `v-model: string`, props `label: string`, `icon?: IconName`, `disabled?`, `error?: string`,
    `type?: "text" | "search"`; gli attributi di chi lo usa vanno sull'`input`;
  - `BaseRadioGroup` — props `modelValue: string | null`, `options: readonly { value: string; label: string }[]`,
    `legend: string`, `disabled?`; emette `update:modelValue(value: string)` e **non cambia da sé**;
  - `BaseDialog` — `v-model:open?: boolean`, props `title: string`, `description?: string`,
    `variant?: "center" | "sheet" | "full"`; slot `default`, `trigger`, `actions`;
  - `violations(node: Element, options?: { contrast?: boolean }): Promise<string[]>` da `testing/axe.ts`.

- [ ] **Passo 1: `lucide`, in due passi**

```bash
(cd gui &&
  npm install --save-exact lucide@1.47.0 &&
  npm ls lucide && npm view lucide@1.47.0 license)
```

Atteso: `lucide@1.47.0` in `dependencies`, licenza `ISC` — e MIT per le icone che vengono da Feather, lo dice la sua
licenza; il pacchetto dei **soli disegni**, non quello per Vue (la (b), nello spirito di ADR-0030).

- [ ] **Passo 2: l'aiutante di `axe`, seconda occorrenza**

Crea `gui/src/testing/axe.ts` (LF):

```ts
import axe from "axe-core";

/**
 * Every violation axe finds under a node, as `rule: targets`, and nothing else. Two users since the design system's
 * task 3 -- `a11y.test.ts` and `components/kit.test.ts` -- so it lives here once.
 *
 * ⛔ `color-contrast` IS OFF UNLESS ASKED, AND NOT IGNORED: under jsdom axe files it under `incomplete` every time --
 * there is no layout to read a background from (measured on 2026-09-15, P-86 of part 2) -- so a green from it would
 * prove nothing there. `tokens/contrast.test.ts` holds the families of the pairs; the probes of the kit page, in the
 * real browser, turn it ON with `contrast: true` (task 4).
 */
export async function violations(node: Element, options: { contrast?: boolean } = {}): Promise<string[]> {
  const results = await axe.run(node, { rules: { "color-contrast": { enabled: options.contrast === true } } });
  return results.violations.map((violation) => `${violation.id}: ${violation.nodes.map((n) => n.target.join(" ")).join(", ")}`);
}
```

In `gui/src/a11y.test.ts` (`replace_unique.py`), tre sostituzioni — ⚠️ ciascuna col testo **intorno**, perché
l'aiutante toglie l'ultimo fine-riga dei due testi e una riga tolta da sola lascerebbe la sua riga vuota. *Trova*:

```ts
import { mount } from "@vue/test-utils";
import axe from "axe-core";
```

*Sostituisci con:*

```ts
import { mount } from "@vue/test-utils";
```

*Trova:*

```ts
import { createFakeBridge } from "./transport/fakeBridge";
```

*Sostituisci con:*

```ts
import { violations } from "./testing/axe";
import { createFakeBridge } from "./transport/fakeBridge";
```

*Trova* il blocco che comincia con il `/**` sopra `async function violations(node: Element): Promise<string[]> {`, finisce
con la `}` che chiude la funzione, e prosegue con la riga vuota e con la riga
`/** Fills the stores the way a welcome does, so every component has something to draw. */` — **intero, preso dal file**
(`grep -n 'Every violation axe finds\|Fills the stores' gui/src/a11y.test.ts`, e il blocco va dalla riga **sopra** la prima
— il `/**` — alla seconda: il `grep` di `async function violations` ne lascerebbe fuori il commento, R2-12) —
*Sostituisci con* la sola riga `/** Fills the stores the way a welcome does, so every component has something to draw. */`.

```bash
(cd gui && npx vitest run src/a11y.test.ts)
```

Atteso: **verde**, come prima: le prove sono le stesse, l'aiutante è lo stesso.

- [ ] **Passo 3: le prove, prima dei pezzi**

Crea `gui/src/components/kit.test.ts` (LF):

```ts
import { mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it } from "vitest";
import { h, nextTick, ref, type Component } from "vue";

import { PANEL_TYPES } from "../panels/registry";
import { violations } from "../testing/axe";

import BaseButton from "./BaseButton.vue";
import BaseDialog from "./BaseDialog.vue";
import BaseIcon from "./BaseIcon.vue";
import BaseLabel from "./BaseLabel.vue";
import BaseList from "./BaseList.vue";
import BaseRadioGroup from "./BaseRadioGroup.vue";
import BaseStatus from "./BaseStatus.vue";
import BaseTextField from "./BaseTextField.vue";
import { ICONS, isIconName, type IconName } from "./icons";

// ⛔ THE WORDS BELOW ARE SPECIMENS: a base piece carries none of its own (section (b)), so a probe hands them in.

/** `BaseList` is generic for its callers' templates; a probe hands it plain props, so it sees a plain component --
 * through `unknown`, because to `vue-tsc` a generic component is a function, and the direct cast is TS2352. */
const List = BaseList as unknown as Component;

beforeEach(() => {
  document.body.replaceChildren();
});

describe("BaseIcon and the one map (design system, section (b); control 10)", () => {
  it("draws every icon of the map, with `currentColor`, hidden from the reader", () => {
    const names = Object.keys(ICONS) as IconName[];
    expect(names.length).toBeGreaterThan(0);
    for (const name of names) {
      const svg = mount(BaseIcon, { props: { name } }).get("svg");
      expect(svg.attributes("data-icon")).toBe(name);
      expect(svg.attributes("stroke")).toBe("currentColor");
      expect(svg.attributes("aria-hidden")).toBe("true");
      expect(svg.element.children.length, name).toBeGreaterThan(0);
    }
  });

  it("has an icon for every module type, by the same name (D6 of the plan)", () => {
    // ⛔ NON-VACUITY: with no module types, the filter below would be empty and green.
    expect(PANEL_TYPES.length).toBeGreaterThan(0);
    expect(PANEL_TYPES.filter((type) => !isIconName(type.module)).map((type) => type.module)).toEqual([]);
  });

  it("takes only a name of the map, and the compiler is the check", () => {
    // ⛔ `vue-tsc` in the build reads this file, and an UNUSED `@ts-expect-error` is an error too: the line below is
    // proven in both directions -- a name outside the map does not compile, and the directive is not idle.
    // @ts-expect-error -- "not-an-icon" is not an IconName
    const wrong: IconName = "not-an-icon";
    expect(isIconName(wrong)).toBe(false);
  });
});

describe("BaseButton", () => {
  it("is a button that never submits, with the words of whoever uses it", () => {
    const button = mount(BaseButton, { slots: { default: () => "Consenti" } }).get("button");
    expect(button.attributes("type")).toBe("button");
    expect(button.text()).toBe("Consenti");
  });

  it("takes `label` as the name of an icon alone, and only then (WCAG 2.5.3)", () => {
    const alone = mount(BaseButton, { props: { icon: "float", label: "Stacca" } }).get("button");
    expect(alone.attributes("aria-label")).toBe("Stacca");
    expect(alone.attributes("data-icon-only")).toBeDefined();
    const worded = mount(BaseButton, { props: { icon: "float", label: "Stacca" }, slots: { default: () => "Stacca la tessera" } }).get("button");
    // ⛔ THE SECOND DIRECTION: with visible words, the words are the name.
    expect(worded.attributes("aria-label")).toBeUndefined();
  });

  it("names an icon alone again when the words go away: the slot is read at every render", async () => {
    const words = ref(true);
    const wrapper = mount({
      render: () => h(BaseButton, { icon: "float", label: "Stacca" }, words.value ? { default: () => "Stacca la tessera" } : {}),
    });
    expect(wrapper.get("button").attributes("aria-label")).toBeUndefined();
    words.value = false;
    await nextTick();
    // ⛔ `useSlots()` is not reactive: a `computed` over it keeps its first answer, and this button stays without a name.
    expect(wrapper.get("button").attributes("aria-label")).toBe("Stacca");
    expect(wrapper.get("button").attributes("data-icon-only")).toBeDefined();
  });

  it("carries its shape to the element, and is off when disabled", () => {
    const button = mount(BaseButton, { props: { variant: "card", size: "lg", pill: true, disabled: true }, slots: { default: () => "Home" } }).get("button");
    expect(button.attributes("data-variant")).toBe("card");
    expect(button.attributes("data-size")).toBe("lg");
    expect(button.attributes("data-pill")).toBeDefined();
    expect((button.element as HTMLButtonElement).disabled).toBe(true);
  });
});

describe("BaseStatus -- M-3 of E187, closed by construction", () => {
  it("is in the DOM while empty, and the words enter the same region", async () => {
    const shown = ref(false);
    const wrapper = mount(() => h(BaseStatus, null, { default: () => (shown.value ? h("p", "Il core non ha risposto.") : null) }));
    const region = wrapper.get('[role="status"]');
    expect(region.text()).toBe("");
    shown.value = true;
    await nextTick();
    // ⛔ THE SAME ELEMENT, NOW WITH WORDS: a region born with its text is the case many readers do not announce.
    expect(wrapper.get('[role="status"]').element).toBe(region.element);
    expect(region.text()).toBe("Il core non ha risposto.");
  });
});

describe("BaseList and BaseLabel", () => {
  it("draws one row per item through the slot, as a list or an ordered one", () => {
    const items = ["uno", "due"];
    const slots = { item: ({ item }: { item: string }) => h("span", item) };
    const plain = mount(List, { props: { items, keyOf: (item: string) => item }, slots });
    expect(plain.get("ul").findAll("li").map((row) => row.text())).toEqual(items);
    const ordered = mount(List, { props: { items, keyOf: (item: string) => item, ordered: true }, slots });
    expect(ordered.find("ol").exists()).toBe(true);
  });

  it("puts the label in the element asked for, with its icon", () => {
    const label = mount(BaseLabel, { props: { icon: "permissions", as: "h3" }, slots: { default: () => "Permessi" } });
    expect(label.get("h3").text()).toBe("Permessi");
    expect(label.find('svg[data-icon="permissions"]').exists()).toBe(true);
  });
});

describe("BaseTextField", () => {
  it("binds its text, puts the caller's attributes on the input, and says an error under it", async () => {
    const text = ref("");
    const wrapper = mount(() =>
      h(BaseTextField, {
        label: "Nome della vista",
        placeholder: "Revisione",
        modelValue: text.value,
        "onUpdate:modelValue": (value: string) => (text.value = value),
        error: text.value === "Home" ? "esiste già" : undefined,
      }),
    );
    const input = wrapper.get("input");
    expect(input.attributes("aria-label")).toBe("Nome della vista");
    expect(input.attributes("placeholder")).toBe("Revisione");
    await input.setValue("Home");
    expect(text.value).toBe("Home");
    await nextTick();
    expect(wrapper.get("input").attributes("aria-invalid")).toBe("true");
    const describedBy = wrapper.get("input").attributes("aria-describedby");
    expect(wrapper.get(`[id="${describedBy}"]`).text()).toBe("esiste già");
  });
});

describe("BaseRadioGroup -- controlled (P-8 of the plan)", () => {
  const options = [
    { value: "remote", label: "OpenRouter, VRAM libera" },
    { value: "local", label: "Locale" },
  ];
  it("checks what it is given, asks on a click, and does not move by itself", async () => {
    const asked: string[] = [];
    const wrapper = mount(BaseRadioGroup, {
      attachTo: document.body,
      props: { modelValue: "remote", options, legend: "Policy VRAM", "onUpdate:modelValue": (value: string) => asked.push(value) },
    });
    const checked = (): (string | undefined)[] => wrapper.findAll('[role="radio"]').map((radio) => radio.attributes("aria-checked"));
    expect(checked()).toEqual(["true", "false"]);
    await wrapper.findAll('[role="radio"]')[1]?.trigger("click");
    expect(asked).toEqual(["local"]);
    // ⛔ WHOEVER HOLDS THE VALUE DECIDES: the prop did not change, so the check did not move.
    expect(checked()).toEqual(["true", "false"]);
    await wrapper.setProps({ modelValue: "local" });
    expect(checked()).toEqual(["false", "true"]);
    wrapper.unmount();
  });

  it("checks nothing on null, and names the group with its legend", () => {
    const wrapper = mount(BaseRadioGroup, { props: { modelValue: null, options, legend: "Policy VRAM" } });
    expect(wrapper.findAll('[role="radio"]').map((radio) => radio.attributes("aria-checked"))).toEqual(["false", "false"]);
    const group = wrapper.get('[role="radiogroup"]');
    expect(wrapper.get(`[id="${group.attributes("aria-labelledby")}"]`).text()).toBe("Policy VRAM");
  });
});

describe("BaseDialog", () => {
  it("opens bound, names itself with its title, and asks to close on Esc", async () => {
    const asked: boolean[] = [];
    const wrapper = mount(BaseDialog, {
      attachTo: document.body,
      props: { open: true, title: "Serve un permesso", "onUpdate:open": (value: boolean | undefined) => asked.push(value === true) },
      slots: { actions: () => h(BaseButton, null, () => "Rifiuta") },
    });
    await nextTick();
    const dialog = document.querySelector('[role="dialog"]');
    expect(dialog).not.toBeNull();
    expect(document.getElementById(dialog?.getAttribute("aria-labelledby") ?? "")?.textContent).toBe("Serve un permesso");
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    await nextTick();
    expect(asked).toEqual([false]);
    wrapper.unmount();
  });
});

describe("the eight pieces, under axe", () => {
  // ⛔ ONE PROBE PER PIECE (the (b), "un test con axe per ciascuno"), each in the state a user meets.
  const pieces: [string, () => ReturnType<typeof h>][] = [
    ["BaseButton", () => h(BaseButton, null, () => "Consenti")],
    ["BaseButton, an icon alone", () => h(BaseButton, { icon: "fullPage", label: "A pagina intera" })],
    ["BaseIcon, inside a named button", () => h("button", { type: "button" }, [h(BaseIcon, { name: "search" }), "Cerca"])],
    ["BaseLabel", () => h(BaseLabel, { icon: "status", as: "h3" }, () => "Stato")],
    ["BaseList", () => h(List, { items: ["uno"], keyOf: (item: string) => item }, { item: ({ item }: { item: string }) => item })],
    ["BaseStatus", () => h(BaseStatus, null, () => "Richiesta inviata.")],
    ["BaseTextField", () => h(BaseTextField, { label: "Cerca", icon: "search", modelValue: "" })],
    ["BaseRadioGroup", () => h(BaseRadioGroup, { modelValue: "system", options: [{ value: "system", label: "Sistema" }, { value: "dark", label: "Scuro" }], legend: "Tema" })],
  ];
  for (const [name, render] of pieces) {
    it(`${name} has no violation`, async () => {
      const wrapper = mount(render, { attachTo: document.body });
      await nextTick();
      expect(await violations(wrapper.element)).toEqual([]);
      wrapper.unmount();
    });
  }

  it("BaseDialog, open, has no violation", async () => {
    const wrapper = mount(BaseDialog, { attachTo: document.body, props: { open: true, title: "Serve un permesso", description: "Vale per questa sessione." } });
    await nextTick();
    // The portal renders into `body`, so the whole document is the node under probe.
    expect(await violations(document.body)).toEqual([]);
    wrapper.unmount();
  });
});
```

```bash
(cd gui && npx vitest run src/components/kit.test.ts)
```

Atteso: **rosso** — i pezzi non esistono ancora.

- [ ] **Passo 4: la mappa e `BaseIcon`**

Crea `gui/src/components/icons.ts` (LF):

```ts
import {
  Activity,
  AppWindow,
  Archive,
  BookmarkPlus,
  Box,
  Coins,
  Cpu,
  Eye,
  FolderTree,
  Gauge,
  GitCompare,
  Layers,
  LayoutGrid,
  ListChecks,
  Maximize2,
  MessageSquare,
  Mic,
  Network,
  Radar,
  RotateCcw,
  Search,
  Settings,
  ShieldCheck,
  SquareTerminal,
  type IconNode,
} from "lucide";

/**
 * ⛔ THE ONE MAP OF THE PROGRAM'S ICONS (answer 11 of the design system): OUR name -> a Lucide drawing, and no other
 * file imports `lucide` -- the linter says so. Changing the set touches this file alone. Only the icons we use are
 * here: one is about half a kB, the whole set hundreds of kB (the measures of answer 11).
 *
 * Lucide 1.47.0 is ISC, and MIT for the icons that come from Feather: the licences travel with the package the
 * shell of sub-project 10 will ship (trap 14 of the design).
 */
export const ICONS = {
  // the frame
  views: Layers,
  modules: LayoutGrid,
  search: Search,
  saveView: BookmarkPlus,
  float: AppWindow,
  fullPage: Maximize2,
  // one per module type of `panels/registry.ts`, BY THE SAME NAME: the big grab and the overview draw them. The
  // boards gave Stato, Permessi, Passi and Attività; the rest are the plan's choice, one line each (D6) -- Chat too:
  // on the boards `message-square` marks the status messages, and they would need another drawing if they entered
  // the kit (R2-15 of the review).
  chat: MessageSquare,
  status: Gauge,
  permissions: ShieldCheck,
  steps: ListChecks,
  activity: Activity,
  settings: Settings,
  scope: FolderTree,
  diff: GitCompare,
  preview: Eye,
  terminal: SquareTerminal,
  sensors: Radar,
  costs: Coins,
  knowledge: Network,
  assets3d: Box,
  voice: Mic,
  backup: Archive,
  checkpoint: RotateCcw,
  models: Cpu,
} satisfies Record<string, IconNode>;

export type IconName = keyof typeof ICONS;

export function isIconName(name: string): name is IconName {
  return Object.hasOwn(ICONS, name);
}
```

Crea `gui/src/components/BaseIcon.vue` (LF):

```vue
<script setup lang="ts">
import { h } from "vue";

import { ICONS, type IconName } from "./icons";

/**
 * The one door of the program's icons (answer 11 of the design system): a Lucide drawing of `icons.ts`, drawn with
 * `h()` and never with `v-html`. ⛔ DECORATIVE BY CONSTRUCTION -- `aria-hidden` -- because the name belongs to the
 * control around it: a button that is an icon alone carries `label` (BaseButton).
 */
const props = withDefaults(defineProps<{ name: IconName; size?: "sm" | "md" | "lg" }>(), { size: "md" });

function Drawing() {
  return h(
    "svg",
    {
      class: "base-icon",
      "data-icon": props.name,
      "data-size": props.size,
      xmlns: "http://www.w3.org/2000/svg",
      viewBox: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      "stroke-linecap": "round",
      "stroke-linejoin": "round",
      "aria-hidden": "true",
      focusable: "false",
    },
    ICONS[props.name].map(([tag, attributes]) => h(tag, attributes)),
  );
}
</script>

<template>
  <Drawing />
</template>

<style scoped>
.base-icon {
  flex: none;
  stroke-width: var(--icon-stroke);
}
.base-icon[data-size="sm"] {
  width: var(--size-icon-sm);
  height: var(--size-icon-sm);
}
.base-icon[data-size="md"] {
  width: var(--size-icon-md);
  height: var(--size-icon-md);
}
.base-icon[data-size="lg"] {
  width: var(--size-icon-lg);
  height: var(--size-icon-lg);
}
</style>
```

- [ ] **Passo 5: `BaseButton`, `BaseLabel`, `BaseList`, `BaseStatus`**

Crea `gui/src/components/BaseButton.vue` (LF):

```vue
<script setup lang="ts">
import { useSlots } from "vue";

import BaseIcon from "./BaseIcon.vue";
import type { IconName } from "./icons";

/**
 * The button of the kit (design system, section (b)): the board's `.btn`, with three variants for the controls and a
 * fourth, `card`, for a whole card that is one button -- the views of the overview and "Salva questa vista" (P-9 of the
 * plan). `pill` is imposed by the radius rule: inside a pill goes a pill -- the strip's "moduli".
 *
 * ⛔ NO WORDS INSIDE: they come from whoever uses it, in the slot -- or in `label` when the button is an icon alone,
 * and only then, because with visible words the words are the name (WCAG 2.5.3).
 */
const props = withDefaults(
  defineProps<{
    variant?: "primary" | "secondary" | "quiet" | "card";
    size?: "sm" | "md" | "lg";
    pill?: boolean;
    disabled?: boolean;
    icon?: IconName;
    label?: string;
  }>(),
  { variant: "secondary", size: "md", pill: false, disabled: false, icon: undefined, label: undefined },
);
const slots = useSlots();
/**
 * A function the template calls, NOT a `computed`: `useSlots()` is not reactive, so a `computed` would keep its first
 * answer, and a button whose words go away would stay without a name. A render reads the slots afresh.
 */
function iconOnly(): boolean {
  return props.icon !== undefined && slots.default === undefined;
}
</script>

<template>
  <button
    type="button"
    class="base-button"
    :data-variant="variant"
    :data-size="size"
    :data-pill="pill || undefined"
    :data-icon-only="iconOnly() || undefined"
    :disabled="disabled"
    :aria-label="iconOnly() ? label : undefined"
    :title="iconOnly() ? label : undefined"
  >
    <BaseIcon v-if="icon !== undefined" :name="icon" :size="size === 'lg' ? 'lg' : 'md'" />
    <slot />
  </button>
</template>

<style scoped>
.base-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  height: var(--size-control-md);
  padding: 0 var(--space-3);
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-control);
  background: transparent;
  color: var(--color-text);
  font: var(--font-body-strong);
  white-space: nowrap;
  cursor: pointer;
  transition:
    background-color var(--duration-fast) var(--ease-standard),
    color var(--duration-fast) var(--ease-standard);
}
.base-button:hover:not(:disabled) {
  background: var(--color-bg-fill-hover);
}
.base-button:active:not(:disabled) {
  background: var(--color-bg-fill-active);
}
.base-button:disabled {
  color: var(--color-text-disabled);
  cursor: default;
}

.base-button[data-variant="primary"] {
  background: var(--color-bg-accent);
  border-color: var(--color-bg-accent);
  color: var(--color-text-on-accent);
}
.base-button[data-variant="primary"]:hover:not(:disabled) {
  background: var(--color-bg-accent-hover);
  border-color: var(--color-bg-accent-hover);
}
.base-button[data-variant="primary"]:active:not(:disabled) {
  background: var(--color-bg-accent-active);
  border-color: var(--color-bg-accent-active);
}
.base-button[data-variant="primary"]:disabled {
  background: var(--color-bg-fill);
  border-color: var(--color-bg-fill);
  color: var(--color-text-disabled);
}

.base-button[data-variant="quiet"] {
  border-color: transparent;
  color: var(--color-text-muted);
}
.base-button[data-variant="quiet"]:hover:not(:disabled),
.base-button[data-variant="quiet"]:active:not(:disabled) {
  color: var(--color-text);
}

/* A whole card that is one button: the card of the (a), a column, the words where they fall. */
.base-button[data-variant="card"] {
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
  height: auto;
  padding: var(--space-3);
  border-color: var(--color-border-card);
  border-radius: var(--radius-card);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
  font: var(--font-body);
  text-align: start;
  white-space: normal;
}
/* The current card: the mark all around -- the overview's view in bordeaux (the (d)). */
.base-button[data-variant="card"][aria-current]:not([aria-current="false"]) {
  border-color: var(--color-mark);
  box-shadow: 0 0 0 var(--border-width) var(--color-mark);
}

.base-button[data-size="sm"] {
  height: var(--size-control-sm);
  padding: 0 var(--space-2);
}
.base-button[data-size="lg"] {
  height: var(--size-control-lg);
  padding: 0 var(--space-4);
}
.base-button[data-icon-only] {
  padding: 0;
  aspect-ratio: 1;
}
.base-button[data-pill] {
  border-radius: var(--radius-full);
}
</style>
```

Crea `gui/src/components/BaseLabel.vue` (LF):

```vue
<script setup lang="ts">
import BaseIcon from "./BaseIcon.vue";
import type { IconName } from "./icons";

/**
 * The label of the kit (design system, section (b)): the board's `.lab` -- small capitals, spaced, with an icon in the
 * mark's colour. `as` keeps the meaning where the label is a heading: the titles of Permessi are `h3`.
 */
withDefaults(defineProps<{ icon?: IconName; as?: "span" | "h2" | "h3" | "h4" }>(), { icon: undefined, as: "span" });
</script>

<template>
  <component :is="as" class="base-label">
    <BaseIcon v-if="icon !== undefined" :name="icon" size="sm" />
    <slot />
  </component>
</template>

<style scoped>
.base-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: 0;
  font: var(--font-label);
  letter-spacing: var(--tracking-label);
  text-transform: uppercase;
  color: var(--color-text-muted);
}
.base-label :deep(.base-icon) {
  color: var(--color-mark);
}
</style>
```

Crea `gui/src/components/BaseList.vue` (LF):

```vue
<script setup lang="ts" generic="T">
/**
 * The list of the kit (design system, section (b)): the rows of the drawer, of Permessi and of Passi. Static rows --
 * no hover, because nothing in them is clickable -- in the board's `.row` shape; the words come in the `item` slot.
 */
withDefaults(defineProps<{ items: readonly T[]; keyOf: (item: T, index: number) => string | number; ordered?: boolean }>(), {
  ordered: false,
});
defineSlots<{ item(props: { item: T; index: number }): unknown }>();
</script>

<template>
  <component :is="ordered ? 'ol' : 'ul'" class="base-list">
    <li v-for="(item, index) in items" :key="keyOf(item, index)" class="base-list-row">
      <slot name="item" :item="item" :index="index" />
    </li>
  </component>
</template>

<style scoped>
.base-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-0-5);
  margin: 0;
  padding: 0;
  list-style: none;
}
.base-list-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--space-2);
  min-height: var(--size-control-md);
  padding: 0 var(--space-3);
  border-radius: var(--radius-control);
  color: var(--color-text);
}
</style>
```

Crea `gui/src/components/BaseStatus.vue` (LF):

```vue
<script setup lang="ts">
/**
 * The status region of the kit (design system, section (b)) -- and the cure of M-3 of E187 BY CONSTRUCTION: the region
 * is ALWAYS in the DOM, empty and zero high, and the words ENTER it. Many screen readers announce a live region only
 * when its content changes, never one that is born with its text; and an empty `div` draws nothing, which is the
 * "no empty box" rule of §6a of the sub-project 2 design.
 */
</script>

<template>
  <div class="base-status" role="status"><slot /></div>
</template>
```

- [ ] **Passo 6: `BaseTextField`, `BaseRadioGroup`, `BaseDialog`**

Crea `gui/src/components/BaseTextField.vue` (LF):

```vue
<script setup lang="ts">
import { useId } from "vue";

import BaseIcon from "./BaseIcon.vue";
import type { IconName } from "./icons";

/**
 * The text field of the kit (design system, section (b); decision 20): the name of a saved view and the bar's search
 * are its two occurrences. `label` is the accessible name -- the board draws no visible label, the icon and the
 * placeholder speak to the eye; `error` says what is wrong under the field and marks it invalid.
 * ⛔ The caller's attributes -- `placeholder`, `@keydown` -- land on the INPUT, not on the frame around it.
 */
defineOptions({ inheritAttrs: false });
withDefaults(defineProps<{ label: string; icon?: IconName; disabled?: boolean; error?: string; type?: "text" | "search" }>(), {
  icon: undefined,
  disabled: false,
  error: undefined,
  type: "text",
});
const model = defineModel<string>({ default: "" });
const errorId = useId();
</script>

<template>
  <div class="base-text-field">
    <div class="frame" :data-error="error !== undefined || undefined" :data-disabled="disabled || undefined">
      <BaseIcon v-if="icon !== undefined" :name="icon" />
      <input
        v-model="model"
        v-bind="$attrs"
        :type="type"
        :disabled="disabled"
        :aria-label="label"
        :aria-invalid="error !== undefined || undefined"
        :aria-describedby="error !== undefined ? errorId : undefined"
      />
    </div>
    <p v-if="error !== undefined" :id="errorId" class="error">{{ error }}</p>
  </div>
</template>

<style scoped>
.frame {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  height: var(--size-control-md);
  padding: 0 var(--space-3);
  border: var(--border-width) solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: var(--color-bg);
  color: var(--color-text-muted);
  font: var(--font-body);
}
.frame:hover:not([data-disabled]) {
  border-color: var(--color-text-muted);
}
/* ⛔ THE RING IS THE FRAME'S, as on the board: the input inside gives its own away. */
.frame:has(input:focus-visible) {
  outline: var(--focus-width) solid var(--color-focus);
  outline-offset: var(--focus-offset);
  color: var(--color-text);
}
.frame[data-disabled] {
  background: var(--color-bg-fill);
  border-color: var(--color-border);
  color: var(--color-text-disabled);
}
.frame[data-error] {
  border-color: var(--color-border-stop);
  color: var(--color-text);
}
input {
  flex: 1;
  min-width: 0;
  height: 100%;
  padding: 0;
  border: 0;
  background: transparent;
  color: var(--color-text);
  font: inherit;
}
input:focus-visible {
  outline: none;
}
input:disabled {
  color: var(--color-text-disabled);
}
input::placeholder {
  color: var(--color-text-muted);
}
.error {
  margin: var(--space-1) 0 0;
  font: var(--font-caption);
  color: var(--color-text-stop);
}
</style>
```

Crea `gui/src/components/BaseRadioGroup.vue` (LF):

```vue
<script setup lang="ts">
import { RadioGroupIndicator, RadioGroupItem, RadioGroupRoot } from "reka-ui";
import { useId } from "vue";

/**
 * The choice among a few options (design system, section (b)), on `reka-ui`'s radio group, which gives the arrows and
 * the roles. ⛔ CONTROLLED, AND THAT IS THE POINT (P-8 of the plan): the group SHOWS `modelValue` and only ASKS for a
 * change with `update:modelValue` -- `reka-ui` 2.10.4 keeps no state of its own once a value is given, `null`
 * included -- so a choice the core decides moves when the core answers, not on the click. `null` checks nothing.
 */
defineProps<{ modelValue: string | null; options: readonly { value: string; label: string }[]; legend: string; disabled?: boolean }>();
const emit = defineEmits<{ "update:modelValue": [value: string] }>();
const id = useId();

function ask(value: unknown): void {
  if (typeof value === "string") emit("update:modelValue", value);
}
</script>

<template>
  <div class="base-radio-group">
    <span :id="`${id}-legend`" class="legend">{{ legend }}</span>
    <RadioGroupRoot :model-value="modelValue" :disabled="disabled" :aria-labelledby="`${id}-legend`" class="options" @update:model-value="ask">
      <div v-for="option in options" :key="option.value" class="option">
        <RadioGroupItem :id="`${id}-${option.value}`" :value="option.value" class="radio">
          <RadioGroupIndicator class="dot" />
        </RadioGroupItem>
        <label :for="`${id}-${option.value}`">{{ option.label }}</label>
      </div>
    </RadioGroupRoot>
  </div>
</template>

<style scoped>
.base-radio-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.legend {
  font: var(--font-label);
  letter-spacing: var(--tracking-label);
  text-transform: uppercase;
  color: var(--color-text-muted);
}
.base-radio-group :deep(.options) {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}
.option {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-height: var(--size-target-min);
}
/* The radio is a `button` in `reka-ui` (trap 10): a 24 x 24 target (WCAG 2.5.8) around a 16 px circle. */
.base-radio-group :deep(.radio) {
  display: grid;
  place-items: center;
  width: var(--size-target-min);
  height: var(--size-target-min);
  padding: 0;
  border: 0;
  border-radius: var(--radius-full);
  background: transparent;
  cursor: pointer;
}
.base-radio-group :deep(.radio)::before {
  content: "";
  grid-area: 1 / 1;
  width: var(--size-icon-md);
  height: var(--size-icon-md);
  box-sizing: border-box;
  border: var(--border-width) solid var(--color-border-strong);
  border-radius: var(--radius-full);
  background: var(--color-bg);
}
.base-radio-group :deep(.radio[data-state="checked"])::before {
  border-color: var(--color-mark);
}
.base-radio-group :deep(.dot) {
  grid-area: 1 / 1;
  width: var(--space-2);
  height: var(--space-2);
  border-radius: var(--radius-full);
  background: var(--color-mark);
}
.base-radio-group :deep(.radio[data-disabled]) {
  cursor: default;
}
.base-radio-group :deep(.radio[data-disabled])::before {
  border-color: var(--color-border);
  background: var(--color-bg-fill);
}
label {
  font: var(--font-body);
  color: var(--color-text);
  cursor: pointer;
}
.option:has([data-disabled]) label {
  color: var(--color-text-disabled);
  cursor: default;
}
</style>
```

Crea `gui/src/components/BaseDialog.vue` (LF):

```vue
<script setup lang="ts">
import { DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogRoot, DialogTitle, DialogTrigger } from "reka-ui";

/**
 * The modal window of the kit (design system, section (b)), on `reka-ui`'s dialog, which gives Esc, the focus kept inside
 * and given back to the control that opened it (decision 18). Three shapes: `center`, a question with its answers;
 * `sheet`, from the bottom -- the drawer; `full`, the whole window -- the overview.
 *
 * `open` is optional: bound with `v-model:open` the window is the caller's, unbound the `trigger` slot opens it.
 * ⛔ NO WORDS INSIDE: the title and the description are props, the body and the `actions` are slots.
 */
withDefaults(defineProps<{ title: string; description?: string; variant?: "center" | "sheet" | "full" }>(), {
  description: undefined,
  variant: "center",
});
const open = defineModel<boolean | undefined>("open", { default: undefined });
</script>

<template>
  <DialogRoot v-model:open="open">
    <DialogTrigger v-if="$slots.trigger" as-child>
      <slot name="trigger" />
    </DialogTrigger>
    <DialogPortal>
      <DialogOverlay class="base-dialog-veil" />
      <!-- ⛔ WITHOUT A DESCRIPTION, NO `aria-describedby`: reka-ui 2.10.4 would point it at a description that is not
           there, and warn. Its `$attrs` merge after its own, so this `undefined` wins (R2-18 of the review). -->
      <DialogContent
        class="base-dialog"
        :data-variant="variant"
        v-bind="description === undefined ? { 'aria-describedby': undefined } : {}"
      >
        <DialogTitle class="title">{{ title }}</DialogTitle>
        <DialogDescription v-if="description !== undefined" class="description">{{ description }}</DialogDescription>
        <slot />
        <div v-if="$slots.actions" class="actions">
          <slot name="actions" />
        </div>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>

<style scoped>
.base-dialog-veil {
  position: fixed;
  inset: 0;
  z-index: var(--z-overlay);
  background: var(--color-veil);
}
.base-dialog {
  position: fixed;
  z-index: var(--z-overlay);
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3);
  border: var(--border-width) solid var(--color-border-card);
  background: var(--color-bg-raised);
  box-shadow: var(--shadow-overlay);
  color: var(--color-text);
}
/* The radii follow the rule of answer 4: a card of 20 with 12 of margin around controls of 8.
   ⚠️ The width here and the sheet's `max-height` below are the PLAN's choice, written by hand: the board has no token
   for a window's size -- its `.dlg` is 74% of its frame -- and a token is the board's to add (R2-18 of the review). */
.base-dialog[data-variant="center"] {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: min(30rem, calc(100vw - var(--space-12)));
  border-radius: var(--radius-card);
}
.base-dialog[data-variant="sheet"] {
  inset: auto 0 0 0;
  max-height: 60vh;
  overflow: auto;
  border-radius: var(--radius-card) var(--radius-card) 0 0;
}
/* The whole window: its corners are Windows' (the (d)), so ours are square. */
.base-dialog[data-variant="full"] {
  inset: 0;
  overflow: auto;
  padding: var(--space-6);
  border: 0;
  border-radius: 0;
  background: var(--color-bg);
}
.title {
  margin: 0;
  font: var(--font-title);
}
.base-dialog[data-variant="sheet"] .title,
.base-dialog[data-variant="full"] .title {
  font: var(--font-label);
  letter-spacing: var(--tracking-label);
  text-transform: uppercase;
  color: var(--color-text-muted);
}
.description {
  margin: 0;
  color: var(--color-text-muted);
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-top: var(--space-1);
}
</style>
```

```bash
(cd gui && npx vitest run src/components && npm run build)
```

Atteso: **verde** — le prove di `kit.test.ts`, `markdown.test.ts` com'era; il *build* verde, con `vue-tsc` che ha letto la
riga `@ts-expect-error`. Il pezzo JavaScript **non cambia** a questo compito: nessun file dell'app importa ancora il kit, e
`vite` lascia fuori `icons.ts` e `lucide` — la misura, e le ventiquattro icone, sono del compito 5 (R2-13).

- [ ] **Passo 7: il linter — i `.ts`, e le regole del kit**

In `gui/eslint.config.js` (`replace_unique.py`), quattro sostituzioni. *Trova*:

```js
import vue from "eslint-plugin-vue";
```

*Sostituisci con:*

```js
import vue from "eslint-plugin-vue";

/** The one door of the icons (answer 11 of the design system). */
const LUCIDE = {
  name: "lucide",
  message: "icons pass through BaseIcon and the one map, src/components/icons.ts (answer 11 of the design system)",
};
/** What a base piece may not reach: the global state, and every layer above the kit (section (b); P-3 of its plan). */
const UPWARD = {
  regex: "^pinia$|(^|/)(stores|panels|frame|transport)(/|$)",
  message: "a base piece reads no global state and knows no layer above it (design system, section (b))",
};
/** What no file of the kit may reach: the layers above it (section (b); R2-11 of the design-system review). */
const ABOVE = {
  regex: "(^|/)(panels|frame|transport)(/|$)",
  message: "the kit knows no layer above it: panels, frame and transport use the kit, not the other way (design system, section (b))",
};
```

*Trova:*

```js
  {
    name: "harness/settings",
```

*Sostituisci con:*

```js
  {
    /**
     * ⛔ THE `.ts` FILES ARE READ FROM THE DESIGN SYSTEM ON (P-2 of its plan). P-101 of part 2 left them to `vue-tsc`,
     * and their TYPES stay there; but the import rules below must see `icons.ts`, `BigTab.ts`, `dock.ts`, or `lucide` and
     * `reka-ui` could come in through a `.ts` unseen. The unscoped blocks now reach the `.ts` too, and the gate says
     * whether any of them objects.
     */
    name: "harness/ts",
    files: ["**/*.ts"],
    languageOptions: { parser: tsParser },
  },
  {
    name: "harness/settings",
```

*Trova* — la chiusura del file, dopo il blocco di `Chat.vue`:

```js
    name: "harness/chat-renders-our-own-html",
    files: ["src/panels/Chat.vue"],
    rules: { "vue/no-v-html": "off" },
  },
];
```

*Sostituisci con:*

```js
    name: "harness/chat-renders-our-own-html",
    files: ["src/panels/Chat.vue"],
    rules: { "vue/no-v-html": "off" },
  },
  /**
   * ⛔ THE IMPORT RULES OF THE KIT (design system, section (b)). One rule, a scope per block, and the ORDER MATTERS: in a flat
   * config a later block REPLACES an earlier one's options for the same rule, it does not merge them -- so every block
   * says the whole list for its files.
   */
  {
    name: "harness/imports",
    files: ["**/*.vue", "**/*.ts"],
    rules: { "no-restricted-imports": ["error", { paths: [LUCIDE] }] },
  },
  {
    // ⛔ THE WHOLE KIT KNOWS NO LAYER ABOVE IT (section (b); R2-11 of the design-system review, the owner's choice A). A
    // composed piece such as `Confirm.vue` may read the stores, which the base pieces may not -- their block below says so,
    // and comes AFTER this one because it replaces it for their files. The tests are out: `kit.test.ts` reads
    // `PANEL_TYPES`, to check that every module type has an icon.
    name: "harness/imports/components",
    files: ["src/components/**/*.{vue,ts}"],
    ignores: ["src/components/**/*.test.ts"],
    rules: { "no-restricted-imports": ["error", { paths: [LUCIDE], patterns: [ABOVE] }] },
  },
  {
    name: "harness/imports/base-pieces",
    files: ["src/components/Base*.vue"],
    rules: { "no-restricted-imports": ["error", { paths: [LUCIDE], patterns: [UPWARD] }] },
  },
  {
    // The one file that may import `lucide`, and the only rule it keeps is the base pieces' one.
    name: "harness/imports/the-icon-map",
    files: ["src/components/icons.ts"],
    rules: { "no-restricted-imports": ["error", { patterns: [UPWARD] }] },
  },
];
```

*Trova* — nel commento del blocco `harness/ts-in-vue`, la frase che il blocco `harness/ts` smentisce (R2-10; gotcha #58):

```js
     * proven either (R8-1). The `.ts` files are still nobody's business here (P-101): `vue-tsc` inside
     * `npm run build` is the level 1 of the web world.
```

*Sostituisci con:*

```js
     * proven either (R8-1). The `.ts` files are read from the design system on (block `harness/ts`, P-2 of its plan);
     * their TYPES stay with `vue-tsc` inside `npm run build`, the level 1 of the web world (R2-10 of its review).
```

```bash
(cd gui && npm run lint)
```

Atteso: **verde** sul codice di oggi. ⛔ Se una regola già attiva scatta su un `.ts` ora letto — le regole di
`vue/essential` e dei due blocchi nostri non hanno `files` — è una voce d'errata col suo esito, **non** un blocco del
linter spento per farla tacere.

- [ ] **Passo 8: le due direzioni del linter**

Una alla volta, `npm run lint` dopo ciascuna, poi indietro con la **copia salvata** (vincolo 11): `BaseLabel.vue` e
`BaseButton.vue` sono nati in questo compito, e `git checkout` non li conosce ancora (R3-5, A-1):

| La violazione messa a mano | Atteso |
|---|---|
| in `src/components/BaseLabel.vue` la riga `import { useCore } from "../stores/core";` | **rosso**: *«a base piece reads no global state»* |
| in `src/frame/moveActive.ts` la riga `import { Search } from "lucide";` | **rosso**: *«icons pass through BaseIcon»* — ⛔ è la prova che i `.ts` ora si leggono |
| in `src/components/BaseButton.vue` la riga `import { Search } from "lucide";` | **rosso**: lo stesso messaggio, dal blocco dei pezzi di base |
| in `src/components/Confirm.vue` la riga `import { PANEL_TYPES } from "../panels/registry";` | **rosso**: *«the kit knows no layer above it»* — un pezzo composto legge i negozi, non gli strati sopra (R2-11) |
| in `src/components/markdown.ts` la riga `import type { Bridge } from "../transport/bridge";` | **rosso**: lo stesso messaggio — ⛔ la regola arriva anche ai `.ts` di `components/`, e a un `import type` |
| niente: `src/components/icons.ts` importa `lucide`, `src/components/Confirm.vue` legge due negozi, `src/components/kit.test.ts` legge `PANEL_TYPES` | **verde**: la prima è la porta unica, il secondo è un pezzo composto (**P-3**), il terzo è una prova (R2-11) |

E le prove del kit nella loro direzione rossa (R2-16) — una alla volta, `npx vitest run --project jsdom
src/components/kit.test.ts` dopo ciascuna, poi indietro con la copia salvata:

| La violazione messa a mano | Atteso |
|---|---|
| in `src/components/icons.ts` tolta la riga `models: Cpu,` | **rosso**: `expected [ 'models' ] to deeply equal []` |
| in `src/components/BaseStatus.vue` la regione dentro un `v-if` che la fa nascere solo quando lo slot disegna qualcosa che non è un commento — `(slots.default?.() ?? []).some((node) => node.type !== Comment)` —: il difetto di M-3 | **rosso**: `Unable to get [role="status"] within: <!--v-if-->`. ⚠️ Un `v-if="$slots.default"` qualunque resta **verde**, e a ragione: chi usa `BaseStatus` passa sempre lo slot, anche quando disegna `null` |

⚠️ **Il linter non ha una guardia di non-vacuità**: se un `files` smettesse di trovare i suoi file, le regole tacerebbero
col verde. La prova delle due direzioni si rifà a mano in ogni compito che tocca `eslint.config.js`; una guardia statica è
un controllo nuovo, del proprietario (vincolo globale 7 della parte 2) — **registrata, non presa**. ⚠️ E un `import()`
**dinamico** passa le regole: `no-restricted-imports` guarda gli import statici (R2-17). `no-restricted-syntax` su
`ImportExpression` lo coprirebbe; oggi nessun file del kit ne ha uno, e la regola non la prende nessun compito.

- [ ] **Passo 9: tutte le prove, il cancello, il commit**

```bash
(cd gui && npm test && npm run build && npm run lint)
```

Poi la riga **3** della tabella della posizione — **Stato** `✅ <data>`, e nella riga **2** la colonna **Commit** con l'hash
del compito 2 (R1-16) —, `bash scripts/gate.sh` da solo, `bash scripts/check-docs.sh`,
il commit — `design-system(compito 3): il kit …` — coi fine-riga rimisurati, e `git push`.

---
## Compito 4: la pagina kit — fuori dal pacchetto, e le sonde diventano prove nel browser

**Da:** la (b), *La pagina kit*; la (f), la tabella *«nel browser vero»*; i controlli **9, 11, 14, 20**; le trappole **1,
3, 4, 12**; **D7** e **D8** di questo piano. Le tre sonde delle tavole, lette per intero:
`docs/superpowers/specs/2026-09-22-design-system-tavole/sonda-raggi.js`, `sonda-icone.js`, `sonda-caratteri.js`.

**Files:**
- Create: `gui/kit.html`, `gui/src/kit/main.ts`, `gui/src/kit/Kit.vue`, `gui/src/kit/kit.browser.test.ts`
- Create: `gui/src/testing/probes.ts` — le tre sonde come funzioni, per questo compito e per il compito 8
- Modify: `gui/eslint.config.js` — il blocco delle parole esemplari (D8)
- Modify: `scripts/gate-gui.sh` — la pagina kit **fuori** dal pacchetto, provato sull'uscita del *build*

**Interfaces:**
- Consumes: gli otto pezzi e `ICONS` del compito 3; `watchTheme` e `isThemeChoice` del compito 1; `violations` di
  `testing/axe.ts`; il progetto `browser` del compito 2.
- Produces, da `gui/src/testing/probes.ts`:
  `concentricRadii(roots: Element[]): { near: number; bad: string[] }`,
  `fits(roots: Element[], boxes: string): { seen: number; boxed: number; problems: string[] }`,
  `iconsCentred(roots: Element[]): { icons: number; centred: number; problems: string[] }`,
  `firstFamily(element: Element): string`.
- Produces: `Kit.vue`, con la prop `initialTheme?: ThemeChoice`; le classi `kit-card`, `kit-frame`, `kit-strip` sono le
  **radici** delle prove.

⚠️ **La regola dei raggi decide anche come è fatta la pagina kit**: un elemento che non può stare in un angolo con lo stesso
centro *«si allontana dall'angolo»* (la (a), il linguaggio visivo). Quindi le carte che sono pulsanti stanno in una cornice
`kit-frame` da `--radius-frame` — 20 + 12 = 32 —, il pulsante a pillola sta nella pillola `kit-strip` e non in una scheda, e
nessuna scheda finisce con un radio nel suo angolo; e la griglia **non stira** le schede (`align-items: start`), così ciò che
chiude una scheda — una nota, o l'ultima riga della lista — sta nel suo angolo, e la sonda dei raggi lo giudica (R3-1). Una
prova rossa sulla pagina kit si legge
**prima** di toccare un pezzo: può dire che è la pagina a violare la regola.

- [ ] **Passo 1: le sonde, come funzioni**

Crea `gui/src/testing/probes.ts` (LF):

```ts
/**
 * The probes of the design-system boards, as functions over the real DOM (design system, section (f)). They were
 * scripts pasted into a console on 2026-09-23 -- `sonda-raggi.js`, `sonda-icone.js`, `sonda-caratteri.js`, next to the
 * approved boards -- and each was proven in both directions there. Here they take their ROOTS as a parameter (trap 4),
 * and each returns how much it looked at, so that a caller can refuse a green that looked at nothing (trap 1).
 *
 * ⛔ THEY NEED A LAYOUT ENGINE: under jsdom every rectangle is zero, so they run in the browser project alone.
 */

function describe(element: Element): string {
  const classes = (element.getAttribute("class") ?? "").trim().split(/\s+/)[0];
  return classes !== undefined && classes !== "" ? classes : element.tagName.toLowerCase();
}

/**
 * The owner's rule of answer 4 -- OUTER radius = INNER radius + distance. For every element with a radius, the nearest
 * rounded ancestor inside a root, and its four corners: where the inner corner sits close to the outer one (within the
 * larger radius, plus 2 px), an element IN the corner must share the centre, and one OFF the corner must not be rounder
 * than the outer radius minus the smaller distance. A straight corner is never compared (answer 20): an element or an
 * ancestor with no radius is skipped. SVG content is a drawing, not a surface.
 */
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
  const radius = (element: Element): number => Number.parseFloat(getComputedStyle(element).borderTopLeftRadius) || 0;
  const effective = (element: Element): number => {
    const box = element.getBoundingClientRect();
    return Math.min(radius(element), box.height / 2, box.width / 2);
  };
  const bad: string[] = [];
  let near = 0;
  for (const root of roots) {
    for (const element of [root, ...root.querySelectorAll("*")]) {
      if (element.closest("svg") !== null) continue;
      const inner = effective(element);
      if (inner === 0) continue;
      let ancestor = element.parentElement;
      while (ancestor !== null && !(radius(ancestor) > 0)) ancestor = ancestor.parentElement;
      if (ancestor === null || (!root.contains(ancestor) && ancestor !== root)) continue;
      const outer = effective(ancestor);
      const b = element.getBoundingClientRect();
      const B = ancestor.getBoundingClientRect();
      const corners: [string, number, number][] = [
        ["top-left", b.left - B.left, b.top - B.top],
        ["top-right", B.right - b.right, b.top - B.top],
        ["bottom-left", b.left - B.left, B.bottom - b.bottom],
        ["bottom-right", B.right - b.right, B.bottom - b.bottom],
      ];
      for (const [corner, dx, dy] of corners) {
        const reach = Math.max(outer, inner) + 2;
        if (!(dx < reach && dy < reach)) continue;
        near += 1;
        const inTheCorner = Math.abs(dx - dy) <= 1.5;
        const ok = inTheCorner ? Math.abs(inner - (outer - dx)) <= 1.5 : inner <= outer - Math.min(dx, dy) + 1.5;
        if (!ok) {
          bad.push(`${describe(element)} in ${describe(ancestor)}, ${corner}: radius ${inner.toFixed(1)}, outer ${outer.toFixed(1)}, distance ${dx.toFixed(1)}/${dy.toFixed(1)}`);
        }
      }
    }
  }
  return { near, bad };
}

/**
 * No text is cut -- an element's content is never wider than the element -- and nothing sticks out of the nearest
 * box that holds it: the fourth check of `sonda-caratteri.js`, with the boxes as a parameter.
 */
export function fits(roots: Element[], boxes: string): { seen: number; boxed: number; problems: string[] } {
  const problems: string[] = [];
  let seen = 0;
  let boxed = 0;
  for (const root of roots) {
    for (const element of root.querySelectorAll<HTMLElement>("*")) {
      if (element.closest("svg") !== null) continue;
      seen += 1;
      if (element.clientWidth > 0 && element.scrollWidth > element.clientWidth + 1) {
        problems.push(`cut: ${describe(element)} "${(element.textContent ?? "").trim().slice(0, 24)}" ${element.scrollWidth}>${element.clientWidth}`);
      }
      const box = element.parentElement?.closest(boxes);
      if (box === null || box === undefined) continue;
      const b = element.getBoundingClientRect();
      const B = box.getBoundingClientRect();
      if (b.width === 0 && b.height === 0) continue;
      boxed += 1;
      if (b.left < B.left - 1 || b.right > B.right + 1 || b.top < B.top - 1 || b.bottom > B.bottom + 1) {
        problems.push(`sticks out: ${describe(element)} of ${describe(box)}`);
      }
    }
  }
  return { seen, boxed, problems };
}

/**
 * Every icon is drawn, strokes with `currentColor`, and -- in a flex row that centres -- sits within 0.75 px of the
 * centre of its parent's content box: `sonda-icone.js`, on the icons of `BaseIcon`. ⚠️ An icon whose parent is NOT a
 * centring flex row is counted and not judged: a violation that un-centres the PARENT falls through here, so the red
 * direction moves the icon inside a centred row (R3-3 of the review).
 */
export function iconsCentred(roots: Element[]): { icons: number; centred: number; problems: string[] } {
  const problems: string[] = [];
  let icons = 0;
  let centred = 0;
  for (const root of roots) {
    for (const svg of root.querySelectorAll("svg.base-icon")) {
      icons += 1;
      const name = svg.getAttribute("data-icon") ?? "?";
      const b = svg.getBoundingClientRect();
      if (!(b.width > 0 && b.height > 0)) problems.push(`not drawn: ${name}`);
      if (svg.getAttribute("stroke") !== "currentColor") problems.push(`stroke is not currentColor: ${name}`);
      const parent = svg.parentElement;
      if (parent === null) continue;
      const style = getComputedStyle(parent);
      if (!style.display.includes("flex") || style.alignItems !== "center" || style.flexDirection.startsWith("column")) continue;
      const P = parent.getBoundingClientRect();
      const top = P.top + Number.parseFloat(style.borderTopWidth) + Number.parseFloat(style.paddingTop);
      const bottom = P.bottom - Number.parseFloat(style.borderBottomWidth) - Number.parseFloat(style.paddingBottom);
      const off = (b.top + b.bottom) / 2 - (top + bottom) / 2;
      centred += 1;
      if (Math.abs(off) > 0.75) problems.push(`off centre by ${off.toFixed(2)} px: ${name} in ${describe(parent)}`);
    }
  }
  return { icons, centred, problems };
}

/** The first family an element computes -- the check "applied" of `sonda-caratteri.js`. */
export function firstFamily(element: Element): string {
  return (getComputedStyle(element).fontFamily.split(",")[0] ?? "").trim().replace(/^["']|["']$/g, "");
}
```

- [ ] **Passo 2: la prova, prima della pagina**

Crea `gui/src/kit/kit.browser.test.ts` (LF):

```ts
import { mount } from "@vue/test-utils";
import axe from "axe-core";
import { afterEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import { violations } from "../testing/axe";
import { concentricRadii, firstFamily, fits, iconsCentred } from "../testing/probes";

import Kit from "./Kit.vue";

// ⛔ THE KIT PAGE IN THE INSTALLED CHROME (design system, section (f)): the probes of the boards, on the real pieces.

const ROOTS = ".kit-card, .kit-frame, .kit-strip";
const BOXES = ".kit-card, .kit-frame, .kit-strip, .base-button, .base-list-row, .base-text-field > .frame, .option, .base-dialog";

afterEach(() => {
  document.body.replaceChildren();
});

async function kit(theme: "light" | "dark") {
  const wrapper = mount(Kit, { attachTo: document.body, props: { initialTheme: theme } });
  await nextTick();
  await document.fonts.ready;
  return wrapper;
}

const roots = (selector: string): Element[] => [...document.querySelectorAll(selector)];

/**
 * ⛔ THE NON-VACUITY OF `axe` (R3-7 of the review): an empty list of violations says something only if the contrast was
 * JUDGED -- nodes among the passes, none left incomplete. Under jsdom axe files every contrast as incomplete; here, in
 * the browser, it must not.
 */
async function contrastJudged(node: Element): Promise<{ passes: number; incomplete: number }> {
  const results = await axe.run(node, { runOnly: ["color-contrast"] });
  const count = (list: axe.Result[]): number => list.find((rule) => rule.id === "color-contrast")?.nodes.length ?? 0;
  return { passes: count(results.passes), incomplete: count(results.incomplete) };
}

for (const theme of ["light", "dark"] as const) {
  describe(`the kit page, ${theme} theme`, () => {
    it("puts its theme on the root", async () => {
      const wrapper = await kit(theme);
      expect(document.documentElement.dataset.theme).toBe(theme);
      wrapper.unmount();
    });

    it("keeps every radius concentric (answer 4)", async () => {
      const wrapper = await kit(theme);
      const report = concentricRadii(roots(ROOTS));
      // ⛔ NON-VACUITY (trap 1): a probe that met no corner near another is green for nothing.
      expect(report.near).toBeGreaterThan(0);
      expect(report.bad).toEqual([]);
      wrapper.unmount();
    });

    it("cuts no text, and lets nothing stick out of its box", async () => {
      const wrapper = await kit(theme);
      const report = fits(roots(ROOTS), BOXES);
      expect(report.seen).toBeGreaterThan(0);
      expect(report.boxed).toBeGreaterThan(0);
      expect(report.problems).toEqual([]);
      wrapper.unmount();
    });

    it("draws every icon in currentColor, and centres it", async () => {
      const wrapper = await kit(theme);
      const report = iconsCentred(roots(".kit"));
      expect(report.icons).toBeGreaterThan(0);
      expect(report.centred).toBeGreaterThan(0);
      expect(report.problems).toEqual([]);
      wrapper.unmount();
    });

    it("dresses labels and numbers in Barlow, and the text in Geist", async () => {
      const wrapper = await kit(theme);
      const label = document.querySelector(".base-label");
      const text = document.querySelector(".base-list-row span");
      const number = document.querySelector(".kit em");
      expect(label !== null && text !== null && number !== null).toBe(true);
      expect(firstFamily(label as Element)).toBe("Barlow");
      expect(firstFamily(text as Element)).toBe("Geist Variable");
      expect(firstFamily(number as Element)).toBe("Barlow");
      wrapper.unmount();
    });

    it("has no axe violation -- contrast included, on the drawn page", async () => {
      const wrapper = await kit(theme);
      expect(await violations(wrapper.element, { contrast: true })).toEqual([]);
      const judged = await contrastJudged(wrapper.element);
      expect(judged.passes).toBeGreaterThan(0);
      expect(judged.incomplete).toBe(0);
      wrapper.unmount();
    });

    it("opens its window with the radii concentric, and no axe violation", async () => {
      const wrapper = await kit(theme);
      document.querySelector<HTMLButtonElement>('[data-kit="open-dialog"]')?.click();
      await nextTick();
      await nextTick();
      const dialog = roots(".base-dialog");
      expect(dialog).toHaveLength(1);
      const report = concentricRadii(dialog);
      expect(report.near).toBeGreaterThan(0);
      expect(report.bad).toEqual([]);
      expect(await violations(dialog[0] as Element, { contrast: true })).toEqual([]);
      const judged = await contrastJudged(dialog[0] as Element);
      expect(judged.passes).toBeGreaterThan(0);
      expect(judged.incomplete).toBe(0);
      wrapper.unmount();
    });
  });
}
```

```bash
(cd gui && npx vitest run --project browser src/kit)
```

Atteso: **rosso** — `Kit.vue` non c'è.

- [ ] **Passo 3: la pagina**

Crea `gui/kit.html` (LF):

```html
<!doctype html>
<html lang="it">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Harness — il kit</title>
  </head>
  <body>
    <div id="kit"></div>
    <script type="module" src="/src/kit/main.ts"></script>
  </body>
</html>
```

Crea `gui/src/kit/main.ts` (LF):

```ts
import { createApp } from "vue";

import "../tokens";
import Kit from "./Kit.vue";

// The kit page (design system, section (b); answer 12): `npm run dev`, then /kit.html. A development page: `vite build`
// takes `index.html` alone, and the gate proves it on the output (task 4 of the plan).
createApp(Kit).mount("#kit");
```

Crea `gui/src/kit/Kit.vue` (LF):

```vue
<script setup lang="ts">
import { onUnmounted, ref } from "vue";

import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseIcon from "../components/BaseIcon.vue";
import BaseLabel from "../components/BaseLabel.vue";
import BaseList from "../components/BaseList.vue";
import BaseRadioGroup from "../components/BaseRadioGroup.vue";
import BaseStatus from "../components/BaseStatus.vue";
import BaseTextField from "../components/BaseTextField.vue";
import { ICONS, type IconName } from "../components/icons";
import { isThemeChoice, watchTheme, type ThemeChoice } from "../tokens/theme";

/**
 * The kit page (design system, section (b); answer 12): every piece in the states it can be shown in, at full size, in
 * the theme chosen at the top -- ONE THEME AT A TIME (D7 of the plan), because a dialog goes to a portal on `body` and
 * would take the root's theme, not a column's. ⛔ The words and the values are SPECIMENS (D8).
 *
 * ⛔ THE RADIUS RULE SHAPES THIS PAGE TOO: the card buttons sit in a frame of `--radius-frame`, the pill button in a
 * pill, and no card ends with a small round control in its corner. The grid does NOT stretch the cards, so what closes
 * one -- a note, or the list's last row -- sits in its corner, where the radius probe judges it (R3-1 of the review).
 */
const props = withDefaults(defineProps<{ initialTheme?: ThemeChoice }>(), { initialTheme: "system" });
const theme = ref<ThemeChoice>(props.initialTheme);
onUnmounted(watchTheme(() => theme.value));

function chooseTheme(value: string): void {
  if (isThemeChoice(value)) theme.value = value;
}

const themes = [
  { value: "system", label: "Sistema" },
  { value: "light", label: "Chiaro" },
  { value: "dark", label: "Scuro" },
];
const variants = ["primary", "secondary", "quiet"] as const;
const sizes = ["sm", "md", "lg"] as const;
const icons = Object.keys(ICONS) as IconName[];
const text = ref("");
const policies = [
  { value: "remote", label: "OpenRouter, VRAM libera" },
  { value: "local", label: "Locale" },
];
const policy = ref<string | null>("remote");
const rows = [
  { what: "policy · VRAM · cambia", when: "14:02" },
  { what: "file · ~/note · leggi", when: "13:58" },
  { what: "rete · openrouter · usa", when: "13:41" },
];
</script>

<template>
  <main class="kit">
    <header class="kit-head">
      <h1>Il kit</h1>
      <BaseRadioGroup :model-value="theme" :options="themes" legend="Tema" @update:model-value="chooseTheme" />
    </header>

    <div class="kit-grid">
      <section class="kit-card">
        <BaseLabel icon="modules" as="h2">Pulsanti</BaseLabel>
        <div v-for="variant in variants" :key="variant" class="kit-row">
          <BaseButton v-for="size in sizes" :key="size" :variant="variant" :size="size">Consenti</BaseButton>
          <BaseButton :variant="variant" disabled>Spento</BaseButton>
          <BaseButton :variant="variant" icon="float" label="Stacca la tessera" />
        </div>
        <div class="kit-row">
          <BaseButton icon="search">Cerca</BaseButton>
          <BaseButton size="lg" icon="fullPage" label="A pagina intera" />
        </div>
        <p class="kit-note">Principale, normale, discreto; tre misure; spento; solo un'icona, col nome per chi non vede.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="status" as="h2">Etichette</BaseLabel>
        <BaseLabel icon="status">Stato</BaseLabel>
        <BaseLabel icon="permissions">Permessi</BaseLabel>
        <BaseLabel icon="steps">Passi</BaseLabel>
        <p class="kit-note">Maiuscolo spaziato, con l'icona nel colore del segno.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="search" as="h2">Campo</BaseLabel>
        <BaseTextField v-model="text" label="Cerca negli artefatti" icon="search" placeholder="Cerca negli artefatti" />
        <BaseTextField model-value="" label="Cerca" icon="search" placeholder="la ricerca arriva col sotto-progetto 6" disabled />
        <BaseTextField model-value="Home" label="Nome della vista" error="Esiste già una vista con questo nome." />
        <p class="kit-note">Normale, spento, con un errore sotto.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="settings" as="h2">Scelta</BaseLabel>
        <BaseRadioGroup :model-value="policy" :options="policies" legend="Policy VRAM" @update:model-value="policy = $event" />
        <BaseRadioGroup :model-value="null" :options="policies" legend="Spenta, finché il core non parla" disabled />
        <p class="kit-note">Il radio mostra il valore che riceve: si muove quando il valore cambia.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="steps" as="h2">Lista</BaseLabel>
        <BaseList :items="rows" :key-of="(row) => row.when">
          <template #item="{ item }">
            <span>{{ item.what }}</span>
            <em>{{ item.when }}</em>
          </template>
        </BaseList>
      </section>

      <section class="kit-card">
        <BaseLabel icon="status" as="h2">Stato</BaseLabel>
        <BaseStatus><p>Richiesta inviata: in attesa del core.</p></BaseStatus>
        <p class="kit-note">La regione c'è sempre, anche vuota: le parole ci entrano.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="float" as="h2">Finestra</BaseLabel>
        <BaseDialog title="Serve un permesso" description="Vale per questa tripla e per questa sessione.">
          <template #trigger>
            <BaseButton data-kit="open-dialog">Apri la finestra</BaseButton>
          </template>
          <template #actions>
            <BaseButton variant="quiet">Rifiuta</BaseButton>
            <BaseButton variant="primary">Consenti</BaseButton>
          </template>
        </BaseDialog>
        <p class="kit-note">Esc chiude, il fuoco resta dentro e torna al pulsante.</p>
      </section>

      <section class="kit-section kit-wide">
        <BaseLabel icon="views" as="h2">Carte</BaseLabel>
        <div class="kit-frame">
          <BaseButton variant="card" aria-current="page">
            <BaseLabel icon="views">Home</BaseLabel>
            <span>La vista di adesso, col segno tutto intorno.</span>
          </BaseButton>
          <BaseButton variant="card">
            <BaseLabel icon="saveView">Salva questa vista</BaseLabel>
            <span>Una carta intera che è un pulsante.</span>
          </BaseButton>
        </div>
      </section>

      <section class="kit-section kit-wide">
        <BaseLabel icon="modules" as="h2">Pillola</BaseLabel>
        <div class="kit-strip">
          <span>Degrado nessuno</span>
          <BaseButton size="lg" pill icon="modules">moduli</BaseButton>
        </div>
      </section>

      <section class="kit-section kit-wide">
        <BaseLabel icon="views" as="h2">Icone</BaseLabel>
        <div class="kit-icons">
          <span v-for="name in icons" :key="name" class="kit-icon"><BaseIcon :name="name" /><code>{{ name }}</code></span>
        </div>
      </section>
    </div>
  </main>
</template>

<!-- ⛔ NOT SCOPED, like `App.vue`'s (P-4 of the design-system plan): this page does not mount `App.vue`, so without this
     rule the browser's 8 px stay around it, and `min-height: 100vh` scrolls 16 px for nothing (R3-11 of the review). -->
<style>
body {
  margin: 0;
}
</style>

<style scoped>
.kit {
  min-height: 100vh;
  box-sizing: border-box;
  padding: var(--space-8) var(--space-6);
  background: var(--color-bg);
  color: var(--color-text);
  font: var(--font-body);
}
.kit-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-6);
  margin-bottom: var(--space-6);
}
h1 {
  margin: 0;
  font: var(--font-heading);
}
.kit-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(22rem, 1fr));
  gap: var(--space-6);
  /* ⛔ NOT STRETCHED: a card stretched to the tallest of its row moves what closes it away from its corner, and the
     radius probe would judge nothing there (R3-1). */
  align-items: start;
}
.kit-card {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-3);
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-card);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
}
.kit-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.kit-wide {
  grid-column: 1 / -1;
}
/* A frame of 32 with 12 of margin around cards of 20: answer 4, and the `calc` of the tokens. */
.kit-frame {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
  padding: var(--space-3);
  border-radius: var(--radius-frame);
  background: var(--color-bg-fill);
}
/* The strip's shape: a pill with 8 of margin around a pill button of 40 -- inside a pill goes a pill. */
.kit-strip {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-2) var(--space-2) var(--space-2) var(--space-4);
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-full);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
}
.kit-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-2);
}
.kit-icons {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3) var(--space-6);
}
.kit-icon {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.kit-note {
  margin: 0;
  font: var(--font-caption);
  color: var(--color-text-muted);
}
code {
  font: var(--font-mono);
  color: var(--color-text-muted);
}
em {
  font: var(--font-numeric);
  font-style: normal;
  font-variant-numeric: tabular-nums;
  color: var(--color-text-muted);
}
</style>
```

- [ ] **Passo 4: il linter — le parole esemplari**

In `gui/eslint.config.js`, *Trova*:

```js
  {
    name: "harness/imports",
```

*Sostituisci con:*

```js
  {
    // ⛔ THE KIT PAGE'S WORDS ARE SPECIMENS (D8 of the design-system plan): a development page outside the package, whose
    // words in `it.json` would ship for nothing. The one exception to the raw-text rule, in one place, like Chat's above.
    name: "harness/kit-page-specimens",
    files: ["src/kit/**"],
    rules: { "@intlify/vue-i18n/no-raw-text": "off" },
  },
  {
    name: "harness/imports",
```

- [ ] **Passo 5: le prove, verdi — e che cosa dice un rosso**

```bash
(cd gui && npx vitest run --project browser && npm run lint && npm run build)
```

Atteso: il progetto `browser` **verde**, le prove della pagina kit nei due temi. ⛔ Un rosso di `concentricRadii` o di `fits`
si **legge** prima di correggere: nomina l'elemento, l'antenato, l'angolo e le distanze; dice se è la pagina a mettere un
pezzo nell'angolo sbagliato — si corregge la pagina, con la regola scritta nel commento di `Kit.vue` — o se è il pezzo a
sbagliare il raggio — si corregge il pezzo, **mai** il token.

- [ ] **Passo 6: la pagina fuori dal pacchetto, provata sull'uscita**

In `scripts/gate-gui.sh`, *Trova*:

```bash
echo "-------- gui: build"
npm run build
```

*Sostituisci con:*

```bash
echo "-------- gui: build"
npm run build
# ⛔ THE KIT PAGE STAYS OUT OF THE PACKAGE (design system, section (b)): `vite build` takes the inputs it is given, and with
# none it takes `index.html` alone. Proven on the output, not believed (trap 12 of the design); the first line is the
# non-vacuity guard -- a build that produced nothing would pass the second.
test -f dist/index.html || { echo "dist/index.html is missing: the build produced nothing to check"; exit 1; }
if [ -e dist/kit.html ] || grep -rlq 'kit-card' dist/assets; then echo "the kit page is in the package"; exit 1; fi
```

Le due direzioni: in `gui/vite.config.ts` si aggiunge per prova
`build: { rolldownOptions: { input: { index: "index.html", kit: "kit.html" } } },` accanto a `define` — `rolldownOptions`, il
nome di Vite 8.3.0: `rollupOptions` vi è deprecato (R3-10) —, e
`bash scripts/gate-gui.sh` è **rosso** con *«the kit page is in the package»*; si toglie, ed è **verde**.

- [ ] **Passo 7: le due direzioni delle sonde**

Una alla volta, nel progetto `browser`, poi indietro con la **copia salvata** (vincolo 11): `Kit.vue` è nato in questo
compito e git non lo conosce ancora (R3-5); e alla fine `git status --porcelain` è quello di prima, cioè nessuna immagine
né allegato nati dai rossi del browser (R3-6).

| La violazione | Atteso |
|---|---|
| in `BaseButton.vue` `border-radius: var(--radius-card)` al posto di `var(--radius-control)` | rosso: la prova della **finestra**, nei due temi — `base-button in base-dialog, bottom-right: …`; nessun pulsante della pagina sta nell'angolo di una scheda (R3-1) |
| in `BaseList.vue` il raggio delle righe, `var(--radius-card)` al posto di `var(--radius-control)` | rosso: la prova della **pagina**, nei due temi — `base-list-row in kit-card, bottom-left: …`: è l'ultima riga della lista a chiudere la sua scheda (R3-1) |
| in `Kit.vue` `.kit-note` con `white-space: nowrap; overflow: hidden;` | rosso: `fits`, nei due temi — `cut: kit-note …` (R3-2) |
| in `Kit.vue` `.kit-strip > .base-button { margin-right: -40px; }` | rosso: `fits`, nei due temi — `sticks out: base-button of kit-strip` (R3-2); ⚠️ e anche la prova `axe` della pagina, nei due temi, alla guardia di R3-7 — `expected 1 to be +0`: il pulsante che sborda è in parte coperto, e `axe` ne lascia il contrasto fra gli `incomplete` (`elmPartiallyObscured`). È la direzione rossa di quella guardia |
| in `BaseLabel.vue` l'icona spostata dentro la sua riga centrata, `.base-label :deep(.base-icon) { position: relative; top: 3px; }` | rosso: `iconsCentred`, nei due temi — `off centre by 3.00 px: … in base-label` (R3-3) |
| in `Kit.vue` `.kit` con `font: 400 0.875rem/1.25rem serif` al posto di `font: var(--font-body)` | rosso: `expected 'serif' to be 'Geist Variable'`, nei due temi — la famiglia **dichiarata**; un carattere che non si carica lo vede la prova del compito 2 (R3-4) |
| in `themes.css` `--color-text-muted` dello scuro portato a `var(--ref-neutral-39)` | rosso: `axe`, `color-contrast` — ⛔ e anche `board.test.ts` e `contrast.test.ts`, che la vogliono (R3-8) |

- [ ] **Passo 8: guardarla, e il commit**

`(cd gui && npm run dev)`, poi `/kit.html` nel browser: i tre temi dalla scelta in cima, a grandezza vera; la tastiera sui
radio e sulla finestra. Poi la riga **4** della tabella della posizione — **Stato** `✅ <data>`, e nella riga **3** la colonna
**Commit** con l'hash del compito 3 (R1-16) —, `bash scripts/gate.sh` da solo,
`bash scripts/check-docs.sh`, il commit — `design-system(compito 4): la pagina kit …` — e `git push`.

---
## Compito 5: il kit al lavoro — i pezzi di base nei pannelli e nella cornice

**Da:** la (b), la tabella *«I pezzi di base»* — la colonna *«La seconda occorrenza»* dice dove va ciascuno — e *«Le regole,
come controlli del linter»*, le ultime due; la (e), la voce **M-3 di E187**; la (a), *«I due temi»*, la riga *«a mano»*; i
controlli **11–13**; la trappola **10**; **P-8** di questo piano; la decisione **21** del disegno.

**Files:**
- Rewrite: `gui/src/components/Confirm.vue`, `gui/src/frame/Drawer.vue`, `gui/src/frame/Band.vue`,
  `gui/src/panels/Settings.vue`, `gui/src/panels/Permissions.vue`, `gui/src/panels/Steps.vue`, `gui/src/frame/ViewBar.vue`
  — ciascuno **per intero**, col terminatore che ha oggi
- Modify: `gui/src/panels/Placeholder.vue` — due *Trova/Sostituisci*, il pulsante e il suo import (R3-15)
- Modify: `gui/src/panels/Status.vue` — la riga dell'evento dentro `BaseStatus`
- Modify: `gui/src/locales/it.json` — la scelta del tema; `gui/src/locales/copy.test.ts` — la sua prova (R3-14)
- Modify: `gui/src/panels/modules.test.ts`, `gui/src/a11y.test.ts` — le prove su `[role=radio]` e su `[role=dialog]`; e, con
  `gui/src/frame/frame.test.ts`, le tre prove di M-3 (R3-12)
- Create: `gui/src/panels/settings.browser.test.ts` — la via della tastiera sul radio, nel browser (R3-24)
- Modify: `gui/eslint.config.js` — le due regole su `panels/` e `frame/`

**Interfaces:**
- Consumes: gli otto pezzi del compito 3; `useLayout().theme`, `chooseTheme`, `THEME_CHOICES`, `isThemeChoice` del
  compito 1.
- Produces: nessun `<button>`, nessun `<ul>` e nessun `<ol>` scritti a mano nei template di `panels/` e `frame/`, e nessun
  import di `reka-ui` o di `lucide` in quelle due cartelle — ⚠️ **tranne** i due pulsanti che
  `frame/BigTab.ts` crea con `document.createElement` (R3-16), che il linter dei template non vede (trappola 5): li toglie il
  compito 6.
- Produces: in `it.json` le chiavi `settings.themeTitle` e `settings.theme.system`, `.light`, `.dark`.

- [ ] **Passo 1: le prove che cambiano, prima del codice**

Le prove della finestra di conferma cercano `.confirm`, una classe che il `DialogContent` di `BaseDialog` non porta: si
cerca il **ruolo**, che è ciò che un lettore di schermo vede. Scrivi `retarget_confirm.py` nello scratchpad:

```python
"""retarget_confirm.py -- the confirmation-window probes look for the dialog's ROLE, not a class (design system, task 5).

Usage: python retarget_confirm.py <repository root>
Refuses, writing nothing, when a count is not the one measured on 2026-09-23.
"""
import io
import os
import sys

root = sys.argv[1]
PLAN = {
    "gui/src/panels/modules.test.ts": [('".confirm button"', """'[role="dialog"] button'""", 2), ('".confirm"', """'[role="dialog"]'""", 4)],
    "gui/src/a11y.test.ts": [('".confirm"', """'[role="dialog"]'""", 1)],
}
out = {}
for rel, renames in PLAN.items():
    path = os.path.join(root, rel)
    text = io.open(path, encoding="utf-8", newline="").read()
    for old, new, expected in renames:
        found = text.count(old)
        if found != expected:
            sys.exit(f"refused: {old} found {found} times in {rel}, expected {expected} -- nothing written")
        text = text.replace(old, new)
    out[path] = text
for path, text in out.items():
    with io.open(path, "w", encoding="utf-8", newline="") as f:
        f.write(text)
    print(f"ok: {path}")
```

```bash
python <scratchpad>/retarget_confirm.py "$(git rev-parse --show-toplevel)"
```

Poi in `gui/src/panels/modules.test.ts` il blocco `describe("Impostazioni", () => {` — dalla sua riga fino alla riga prima di
`describe("the confirmation window", () => {`, **preso dal file** con i due ancoraggi — si sostituisce con:

```ts
describe("Impostazioni", () => {
  // The policy is the FIRST radio group of the panel; the theme is the second.

  it("is off until the core has said which policy is active", () => {
    wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    const radios = wrapper.findAll('[role="radiogroup"]')[0]?.findAll('[role="radio"]') ?? [];
    expect(radios).toHaveLength(2);
    for (const radio of radios) expect((radio.element as HTMLButtonElement).disabled).toBe(true);
  });

  it("sends Invoke with the registry's literals on a change, and nothing on the current value", async () => {
    const { bridge, invoke } = wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    bridge.deliver("Policy");
    await nextTick();
    const [remote, local] = wrapper.findAll('[role="radiogroup"]')[0]?.findAll('[role="radio"]') ?? [];
    expect(remote?.attributes("aria-checked")).toBe("true");
    await remote?.trigger("click");
    expect(bridge.sent).toEqual([]);
    await local?.trigger("click");
    expect(bridge.sent).toEqual([{ kind: "Invoke", value: { function: "vram-policy", argument: "local" } }]);
    expect(invoke.inFlight).not.toBeNull();
    expect(wrapper.text()).toContain(t("settings.inFlight"));
  });

  it("keeps its status region before an Invoke, and the words enter that same region (M-3 of E187)", async () => {
    const { bridge } = wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    bridge.deliver("Policy");
    await nextTick();
    const region = wrapper.get('[role="status"]');
    expect(region.text()).toBe("");
    await wrapper.findAll('[role="radiogroup"]')[0]?.findAll('[role="radio"]')[1]?.trigger("click");
    await nextTick();
    expect(wrapper.get('[role="status"]').element).toBe(region.element);
    expect(region.text()).toContain(t("settings.inFlight"));
  });

  it("leaves the control on the core's policy until the core answers, then moves with it", async () => {
    // ⛔ E184, AND THE TRAP 10 OF THE DESIGN SYSTEM: the radio is a `button` with `role="radio"` in `reka-ui` 2.10.4 now,
    // and the group is CONTROLLED (P-8) -- the probe still asks what the CONTROL shows, not only what went on the wire.
    const { bridge, core } = wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    bridge.deliver("Policy");
    await nextTick();
    const policyRadios = () => wrapper.findAll('[role="radiogroup"]')[0]?.findAll('[role="radio"]') ?? [];
    const checked = (): (string | undefined)[] => policyRadios().map((radio) => radio.attributes("aria-checked"));
    await policyRadios()[1]?.trigger("click");
    await nextTick();
    expect(checked()).toEqual(["true", "false"]);
    // ⛔ THE SECOND DIRECTION: the core answers, and the control DOES move.
    core.receive({ kind: "Policy", value: { policy: "Local", allocated: "12288", total: "16384" } });
    await nextTick();
    expect(checked()).toEqual(["false", "true"]);
  });

  it("keeps saying a call is in flight after the yes, until the core answers with Policy", async () => {
    // ⛔ I-1 OF THE REVIEW (E186): the line must not go silent between the yes and `Policy`.
    const { bridge, invoke } = wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    bridge.deliver("Policy");
    await nextTick();
    await wrapper.findAll('[role="radiogroup"]')[0]?.findAll('[role="radio"]')[1]?.trigger("click");
    bridge.deliver("PermissionRequired");
    expect(invoke.approve()).toBe(true);
    await nextTick();
    expect(wrapper.text()).toContain(t("settings.inFlight"));
    bridge.deliver("Policy");
    await nextTick();
    expect(wrapper.text()).not.toContain(t("settings.inFlight"));
  });

  it("chooses the theme, which the layout package keeps at once (design system, section (a))", async () => {
    wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    const layout = useLayout();
    expect(layout.theme).toBe("system");
    const themeRadios = wrapper.findAll('[role="radiogroup"]')[1]?.findAll('[role="radio"]') ?? [];
    expect(themeRadios.map((radio) => radio.attributes("aria-checked"))).toEqual(["true", "false", "false"]);
    await themeRadios[2]?.trigger("click");
    await nextTick();
    expect(layout.theme).toBe("dark");
    expect(wrapper.findAll('[role="radiogroup"]')[1]?.findAll('[role="radio"]').map((radio) => radio.attributes("aria-checked"))).toEqual(["false", "false", "true"]);
  });
});

```

E `useLayout` entra negli import di `modules.test.ts`: *Trova* `import { useInvoke } from "../stores/invoke";` — *Sostituisci
con* le due righe `import { useInvoke } from "../stores/invoke";` e `import { useLayout } from "../stores/layout";`.

⛔ **M-3 chiusa per costruzione, e una prova per ciascuna delle tre regioni** (R3-12): la prova di `BaseStatus` del compito
3 prova il pezzo, non chi lo usa, e rimettere `BaseStatus` dentro il `v-if` lasciava tutto verde. Quella di Impostazioni
sta nel blocco qui sopra; in `gui/src/panels/modules.test.ts`, *Trova* — la fine del `describe("Stato", …)`:

```ts
    expect(event.text()).toContain(t("status.refusedDetail", { asked: "4096", ceiling: "1024" }));
  });
});
```

*Sostituisci con:*

```ts
    expect(event.text()).toContain(t("status.refusedDetail", { asked: "4096", ceiling: "1024" }));
  });

  it("keeps the event's status region before a Verdict, and the row enters that same region (M-3 of E187)", async () => {
    const { bridge } = wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    const region = wrapper.get('[role="status"]');
    expect(region.text()).toBe("");
    bridge.deliver("Verdict");
    await nextTick();
    expect(wrapper.get('[role="status"]').element).toBe(region.element);
    expect(region.text()).toContain(t("status.verdict.Refused"));
  });
});
```

In `gui/src/frame/frame.test.ts`, *Trova* — la fine del `describe("the band", …)`:

```ts
    expect(wrapper.text()).toBe("");
  });
});
```

*Sostituisci con:*

```ts
    expect(wrapper.text()).toBe("");
  });

  it("keeps its status region while connected, and the words enter that same region (M-3 of E187)", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    const region = wrapper.get('[role="status"]');
    expect(region.text()).toBe("");
    connection.receive({ kind: "StaleBuild", value: "81985529216486895" });
    await nextTick();
    // ⛔ THE SAME ELEMENT, NOW WITH WORDS: a region born with its text is the case many readers do not announce.
    expect(wrapper.get('[role="status"]').element).toBe(region.element);
    expect(region.text()).toContain(i18n.global.t("band.stale"));
  });
});
```

E la via della tastiera sul radio nuovo, che le prove qui sopra non fanno — usano il clic (R3-24). Crea
`gui/src/panels/settings.browser.test.ts` (LF):

```ts
import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { userEvent } from "vitest/browser";
import { afterEach, beforeEach, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import { i18n } from "../i18n";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";
import { createFakeBridge } from "../transport/fakeBridge";

import Settings from "./Settings.vue";

// ⛔ THE ARROW KEY'S WAY, IN THE INSTALLED CHROME (R3-24 of the design-system review): in `reka-ui` 2.10.4 an arrow reaches
// the radio through `RovingFocusGroup` and a `setTimeout` that clicks it -- another road than the click the jsdom probes
// take, and one an update of `reka-ui` could change with nothing going red. Under jsdom that focus and that timer are
// fragile, so the probe lives here.

beforeEach(() => {
  setActivePinia(createPinia());
});

afterEach(() => {
  document.body.replaceChildren();
});

it("asks the core on an arrow key as on a click, and keeps the check on the core's value", async () => {
  const bridge = createFakeBridge();
  const connection = useConnection();
  const core = useCore();
  const invoke = useInvoke();
  invoke.attach(bridge);
  bridge.listen((message) => {
    connection.receive(message);
    core.receive(message);
    invoke.receive(message);
  });
  const wrapper = mount(Settings, { attachTo: document.body, global: { plugins: [i18n] } });
  bridge.deliver("Policy");
  await nextTick();
  const radios = (): HTMLElement[] => [...(document.querySelector('[role="radiogroup"]')?.querySelectorAll<HTMLElement>('[role="radio"]') ?? [])];
  expect(radios()).toHaveLength(2);
  await userEvent.click(radios()[0] as HTMLElement);
  // ⛔ THE SECOND DIRECTION: a click on the current value asks nothing.
  expect(bridge.sent).toEqual([]);
  await userEvent.keyboard("{ArrowDown}");
  await expect.poll(() => bridge.sent.length).toBe(1);
  expect(bridge.sent).toEqual([{ kind: "Invoke", value: { function: "vram-policy", argument: "local" } }]);
  // The focus sits on the radio the arrow reached; the check stays on the core's value until `Policy` comes back (P-8).
  expect(radios().map((radio) => radio.getAttribute("aria-checked"))).toEqual(["true", "false"]);
  expect(document.activeElement).toBe(radios()[1]);
  wrapper.unmount();
});
```

```bash
(cd gui && npx vitest run --project jsdom src/panels/modules.test.ts src/a11y.test.ts src/frame/frame.test.ts)
(cd gui && npx vitest run --project browser src/panels)
```

Atteso: **rosso** sulle prove di Impostazioni — i radio nativi non hanno `role="radio"` né `aria-checked`, e non c'è il
secondo gruppo —; **rosse** le tre prove di M-3, perché i `role="status"` di oggi nascono col `v-if` e la prova non trova
la regione vuota (R3-12); **rossa** la prova della tastiera nel browser, che non trova i due radio; e **verde** sulla
finestra di conferma: un `DialogContent` di `reka-ui` ha già `role="dialog"`.

- [ ] **Passo 2: le parole del tema**

Prima la prova, perché la chiave `settings.theme.${choice}` la **costruisce** `Settings.vue`, e `no-missing-keys` è cieca
alle chiavi costruite: `copy.test.ts` le sonda, e sondava solo `modules.*` (R3-14). In `gui/src/locales/copy.test.ts`,
*Trova*:

```ts
 * the drawer and `modules.${parameters.api.id}` in `BigTab.ts`, and `no-missing-keys` is blind
 * to a built key -- measured on 2026-09-15, both directions in one file (P-105).
```

*Sostituisci con:*

```ts
 * the drawer and `modules.${parameters.api.id}` in `BigTab.ts` -- and, from the design system on,
 * `settings.theme.${choice}` in `Settings.vue` -- and `no-missing-keys` is blind to a built key
 * -- measured on 2026-09-15, both directions in one file (P-105).
```

*Trova:*

```ts
    for (const type of PANEL_TYPES) expect(Object.keys(modules), type.module).toContain(type.module);
  });
```

*Sostituisci con:*

```ts
    for (const type of PANEL_TYPES) expect(Object.keys(modules), type.module).toContain(type.module);
  });

  it("has a word for every theme choice", async () => {
    const { THEME_CHOICES } = await import("../tokens/theme");
    const words = (it_ as { settings?: { theme?: Record<string, string> } }).settings?.theme ?? {};
    // ⛔ NON-VACUITY: no choices would leave nothing to check.
    expect(THEME_CHOICES.length).toBeGreaterThan(0);
    for (const choice of THEME_CHOICES) expect(Object.keys(words), choice).toContain(choice);
  });
```

```bash
(cd gui && npx vitest run --project jsdom src/locales)
```

Atteso: **rosso**, `system: expected [] to include 'system'` — le parole non ci sono ancora. Poi in
`gui/src/locales/it.json`, *Trova*:

```json
    "who": "Le altre preferenze arrivano coi sotto-progetti 3 e 10."
  },
```

*Sostituisci con:*

```json
    "who": "Le altre preferenze arrivano coi sotto-progetti 3 e 10.",
    "themeTitle": "Tema",
    "theme": {
      "system": "Sistema",
      "light": "Chiaro",
      "dark": "Scuro"
    }
  },
```

E la stessa corsa è **verde**; la sua direzione rossa, con la copia salvata dopo (vincolo 11): in `tokens/theme.ts` una
scelta in più senza parola, `["system", "light", "dark", "sepia" as ThemeChoice]` → `sepia: expected [ 'system', 'light',
'dark' ] to include 'sepia'`.

- [ ] **Passo 3: la finestra di conferma e il cassetto, su `BaseDialog`**

Riscrivi `gui/src/components/Confirm.vue`:

```vue
<script setup lang="ts">
import { computed } from "vue";

import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

import BaseButton from "./BaseButton.vue";
import BaseDialog from "./BaseDialog.vue";

// A COMPOSED piece, and that is why it may read two stores (P-3 of the design-system plan): the base pieces below do not.
const core = useCore();
const invoke = useInvoke();

// ⛔ OPEN ONLY WHEN THE CORE ASKED AND A CALL OF OURS IS IN FLIGHT (D59): a `PermissionRequired` following no `Invoke` is a
// shape the core never produces -- the registry only ever answers one -- and a window that opened on it would offer a
// "yes" with nothing to send. The Permessi panel shows such a request; this window does not ask about it.
const open = computed(() => core.pending !== null && invoke.inFlight !== null);

// Escape and the veil are the "no" (ADR-0016: nothing is granted by silence).
function onOpenChange(value: boolean | undefined): void {
  if (value !== true) invoke.refuse();
}
</script>

<template>
  <!-- THE TRIPLE IN EVERYDAY WORDS (sequence 3 of the north star, G20): tool, resource, operation -- what the core asked,
       not what the click meant. -->
  <BaseDialog
    :open="open"
    :title="$t('confirm.title')"
    :description="core.pending === null ? undefined : $t('permissions.triple', { tool: core.pending.tool, resource: core.pending.resource, operation: $t(`permissions.operation.${core.pending.operation}`) })"
    @update:open="onOpenChange"
  >
    <p v-if="invoke.inFlight !== null">{{ $t("confirm.call", { function: invoke.inFlight.function, argument: invoke.inFlight.argument }) }}</p>
    <p class="scope">{{ $t("confirm.scope") }}</p>
    <template #actions>
      <BaseButton variant="quiet" @click="invoke.refuse()">{{ $t("confirm.no") }}</BaseButton>
      <BaseButton variant="primary" @click="invoke.approve()">{{ $t("confirm.yes") }}</BaseButton>
    </template>
  </BaseDialog>
</template>

<style scoped>
p {
  margin: 0;
}
.scope {
  color: var(--color-text-muted);
}
</style>
```

Riscrivi `gui/src/frame/Drawer.vue`:

```vue
<script setup lang="ts">
import { ref } from "vue";

import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseList from "../components/BaseList.vue";
import { PANEL_TYPES } from "../panels/registry";

// ⛔ THE DRAWER IS WHERE "WHO FILLS WHAT" LIVES (decision 16 of the north star): every type with its number, so the strip
// can stay thin. On `BaseDialog` since the design system -- its second occurrence, with the confirmation window: the two
// veils written by hand, already different, are one role now, `--color-veil`.
const open = ref(false);
</script>

<template>
  <BaseDialog v-model:open="open" :title="$t('drawer.title')" variant="sheet">
    <template #trigger>
      <BaseButton>{{ $t("drawer.open") }}</BaseButton>
    </template>
    <BaseList :items="PANEL_TYPES" :key-of="(type) => type.name">
      <template #item="{ item }">
        <span>{{ $t(`modules.${item.module}`) }}</span>
        <span class="who">{{ $t("drawer.who", { number: item.who }) }}</span>
      </template>
    </BaseList>
    <template #actions>
      <BaseButton variant="quiet" @click="open = false">{{ $t("drawer.close") }}</BaseButton>
    </template>
  </BaseDialog>
</template>

<style scoped>
.who {
  color: var(--color-text-muted);
}
</style>
```

- [ ] **Passo 4: le regioni di stato — M-3 chiusa per costruzione**

Riscrivi `gui/src/frame/Band.vue`:

```vue
<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useConnection } from "../stores/connection";

// ⛔ THE BAND APPEARS ONLY WHEN THE CORE IS MISSING OR THE STAMP IS WRONG (§6a), and it is NOT a panel (D50): a panel that
// came and went would rewrite the saved layout on every disconnection. ⛔ AND THERE IS NO THRESHOLD (D48): "not running"
// and "slow" are one state here, because the gui does the same thing in both -- offer `retry`.
// ⛔ THE REGION IS ALWAYS THERE AND THE BAND ENTERS IT (M-3 of E187, closed by `BaseStatus`): a status region born with
// its text is the one many screen readers never announce.
const connection = useConnection();
</script>

<template>
  <BaseStatus>
    <div v-if="connection.phase !== 'connected'" class="band">
      <span v-if="connection.phase === 'stale'">
        {{ $t("band.stale") }}
        <template v-if="connection.expected !== null">
          {{ $t("band.expected", { stamp: connection.expected }) }}
        </template>
      </span>
      <template v-else>
        <span>{{ $t("band.waiting") }}</span>
        <BaseButton size="sm" @click="connection.retry()">{{ $t("band.retry") }}</BaseButton>
      </template>
    </div>
  </BaseStatus>
</template>

<style scoped>
/* The warning message of the (a): the subtle tint, the warning's text, a border of decoration -- the words carry it. */
.band {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-warn-subtle);
  border-bottom: var(--border-width) solid var(--color-border-warn);
  color: var(--color-text-warn);
}
</style>
```

In `gui/src/panels/Status.vue`, *Trova*:

```vue
    <!-- ONE EVENT ROW FOR THE LAST Verdict, AND ONLY WHEN ONE HAS ARRIVED (§6a): no empty box. -->
    <p v-if="core.lastVerdict !== null" class="event" role="status">
      {{ $t(`status.verdict.${core.lastVerdict.verdict}`) }}
      <template v-if="core.lastVerdict.verdict === 'Refused'">{{ $t("status.refusedDetail", { asked: core.lastVerdict.asked, ceiling: core.lastVerdict.ceiling }) }}</template>
    </p>
```

*Sostituisci con:*

```vue
    <!-- ONE EVENT ROW FOR THE LAST Verdict, AND ONLY WHEN ONE HAS ARRIVED (§6a): no empty box -- and the row ENTERS a region
         that is always there (M-3 of E187, `BaseStatus`). -->
    <BaseStatus>
      <p v-if="core.lastVerdict !== null" class="event">
        {{ $t(`status.verdict.${core.lastVerdict.verdict}`) }}
        <template v-if="core.lastVerdict.verdict === 'Refused'">{{ $t("status.refusedDetail", { asked: core.lastVerdict.asked, ceiling: core.lastVerdict.ceiling }) }}</template>
      </p>
    </BaseStatus>
```

E l'import: *Trova* `import { useConnection } from "../stores/connection";` in `Status.vue` — *Sostituisci con* le due righe
`import BaseStatus from "../components/BaseStatus.vue";` e `import { useConnection } from "../stores/connection";`.

- [ ] **Passo 5: Impostazioni — la policy e il tema, su `BaseRadioGroup`**

Riscrivi `gui/src/panels/Settings.vue`:

```vue
<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

import BaseRadioGroup from "../components/BaseRadioGroup.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";
import { useLayout } from "../stores/layout";
import { THEME_CHOICES, isThemeChoice } from "../tokens/theme";

import { VRAM_POLICY, type PolicyArgument } from "./functions";

// The Impostazioni row of the short table of §1: in sub-project 2 the VRAM policy change, a registry function with its triple
// (ADR-0038) -- the FIRST INVOKER of the registry, the click; and since the design system the theme (answer 5).
const { t } = useI18n();
const core = useCore();
const invoke = useInvoke();
const layout = useLayout();

const current = computed<PolicyArgument | null>(() =>
  core.policy === null ? null
  : core.policy.policy === "Local" ? VRAM_POLICY.argument.local
  : VRAM_POLICY.argument.remote,
);
const policies = computed(() => [
  { value: VRAM_POLICY.argument.remote, label: t("settings.remote") },
  { value: VRAM_POLICY.argument.local, label: t("settings.local") },
]);
const themes = computed(() => THEME_CHOICES.map((choice) => ({ value: choice, label: t(`settings.theme.${choice}`) })));

/**
 * ⛔ THE CONTROL SHOWS THE CORE'S POLICY, NOT THE LAST CLICK (I1): a choice sends `Invoke`, and the radio moves when
 * `Policy` comes back -- after the confirmation window, if the triple is not yet granted; meanwhile the line below says a
 * call is in flight. Since the design system the group is CONTROLLED (P-8 of its plan): it shows `current` and only asks,
 * so the hand re-sync E184 needed on native radios went away with them, and an arrow key asks the way a click does.
 * ⚠️ M-2 of E187 stays true, and declared: after an arrow key the focus sits on the radio the arrow reached, while the
 * check stays on the core's value.
 */
function choosePolicy(argument: string): void {
  if (argument !== current.value) invoke.send({ function: VRAM_POLICY.name, argument });
}

/** The theme is the SPA's and not the core's (answer 16): saved at once, in the layout package. */
function chooseTheme(choice: string): void {
  if (isThemeChoice(choice)) layout.chooseTheme(choice);
}
</script>

<template>
  <section class="settings">
    <!-- Off while the core has not said which policy is active: "the rest off" (§6a). -->
    <BaseRadioGroup
      :model-value="current"
      :options="policies"
      :legend="$t('settings.policy')"
      :disabled="core.policy === null"
      @update:model-value="choosePolicy"
    />
    <BaseStatus>
      <p v-if="invoke.inFlight !== null">{{ $t("settings.inFlight") }}</p>
    </BaseStatus>
    <BaseRadioGroup :model-value="layout.theme" :options="themes" :legend="$t('settings.themeTitle')" @update:model-value="chooseTheme" />
    <p class="who">{{ $t("settings.who") }}</p>
  </section>
</template>

<style scoped>
.settings {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
p {
  margin: 0;
}
.who {
  color: var(--color-text-muted);
}
</style>
```

- [ ] **Passo 6: Permessi, Passi, il segnaposto, la barra**

Riscrivi `gui/src/panels/Permissions.vue`:

```vue
<script setup lang="ts">
import BaseLabel from "../components/BaseLabel.vue";
import BaseList from "../components/BaseList.vue";
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

// The Permessi table of §1 of the north star, rows 1-3, 7-9 -- what sub-project 2 builds: the request in flight, the triples
// THIS session approved, and the rule on duration. The window that answers is the frame's (`components/Confirm.vue`, D60).
const core = useCore();
const invoke = useInvoke();
</script>

<template>
  <section class="permissions">
    <BaseLabel icon="permissions" as="h3">{{ $t("permissions.pendingTitle") }}</BaseLabel>
    <p v-if="core.pending === null">{{ $t("permissions.none") }}</p>
    <p v-else class="pending">{{ $t("permissions.triple", { tool: core.pending.tool, resource: core.pending.resource, operation: $t(`permissions.operation.${core.pending.operation}`) }) }}</p>
    <BaseLabel icon="permissions" as="h3">{{ $t("permissions.approvedTitle") }}</BaseLabel>
    <p v-if="invoke.approved.length === 0">{{ $t("permissions.noneApproved") }}</p>
    <BaseList v-else :items="invoke.approved" :key-of="(_triple, index) => index">
      <template #item="{ item }">{{ $t("permissions.triple", { tool: item.tool, resource: item.resource, operation: $t(`permissions.operation.${item.operation}`) }) }}</template>
    </BaseList>
    <p class="rule">{{ $t("permissions.duration") }}</p>
  </section>
</template>

<style scoped>
.permissions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
p {
  margin: 0;
}
.pending {
  color: var(--color-text-warn);
}
.rule {
  color: var(--color-text-muted);
  margin-top: var(--space-2);
}
</style>
```

Riscrivi `gui/src/panels/Steps.vue`:

```vue
<script setup lang="ts">
import BaseList from "../components/BaseList.vue";
import { useCore } from "../stores/core";

// The Passi table of §1 of the north star, rows 1-3 and 14: a PROJECTION of the journal, re-read from the core (`Steps`
// replaces, it never appends). ⛔ THREE FIELDS AND NOT FIVE (P-89, D56): the wire's `StepSummary` carries the step, the
// function and whether it closed; the row says so in words.
const core = useCore();
</script>

<template>
  <section class="steps">
    <p v-if="core.steps.length === 0">{{ $t("steps.none") }}</p>
    <BaseList v-else :items="core.steps" :key-of="(step) => step.step" ordered>
      <template #item="{ item }">
        <span>{{ $t("steps.row", { step: item.step, function: item.function }) }}</span>
        <span class="outcome" :data-done="item.done">{{ item.done ? $t("steps.done") : $t("steps.inDoubt") }}</span>
      </template>
    </BaseList>
    <p class="who">{{ $t("steps.who") }}</p>
  </section>
</template>

<style scoped>
.steps {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
p {
  margin: 0;
}
.outcome {
  color: var(--color-text-muted);
}
.outcome[data-done="false"] {
  color: var(--color-text-warn);
}
.who {
  color: var(--color-text-muted);
}
</style>
```

In `gui/src/panels/Placeholder.vue`, *Trova*
`      <button type="button" @click="api?.close()">{{ $t("placeholder.closeMissing") }}</button>` — *Sostituisci con*
`      <BaseButton @click="api?.close()">{{ $t("placeholder.closeMissing") }}</BaseButton>`; e *Trova*
`import type { DockviewPanelApi } from "dockview-core";` — *Sostituisci con* le due righe
`import type { DockviewPanelApi } from "dockview-core";` e `import BaseButton from "../components/BaseButton.vue";`.

Riscrivi `gui/src/frame/ViewBar.vue` — ⚠️ la forma di questo compito è **di passaggio**: il compito 8 porta la barra della
(d), col nome della vista e la Panoramica; qui i pulsanti diventano `BaseButton` perché la regola del linter entra ora:

```vue
<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseTextField from "../components/BaseTextField.vue";
import { useConnection } from "../stores/connection";
import { useLayout, type ViewName } from "../stores/layout";

import Drawer from "./Drawer.vue";

const connection = useConnection();
const layout = useLayout();
const views: ViewName[] = ["home", "work", "compact"];
const emit = defineEmits<{ (event: "switch", view: ViewName): void }>();
</script>

<template>
  <header class="bar">
    <nav :aria-label="$t('bar.views')">
      <BaseButton
        v-for="name in views"
        :key="name"
        variant="quiet"
        :aria-current="layout.view === name ? 'page' : undefined"
        @click="emit('switch', name)"
      >
        {{ $t(`views.${name}`) }}
      </BaseButton>
    </nav>

    <!-- ⚠️ DISABLED AND SAYING WHO FILLS IT, not hidden: decision 16 of the north star wants the search box to say who fills
         it, and a control that is simply absent teaches nothing. -->
    <div class="search">
      <BaseTextField model-value="" type="search" icon="search" :label="$t('bar.search')" :placeholder="$t('bar.searchHint')" disabled />
    </div>

    <span class="chip" :data-phase="connection.phase">
      {{ $t("bar.core") }}:
      {{
        connection.phase === "connected"
          ? $t("bar.coreConnected")
          : connection.phase === "stale"
            ? $t("bar.coreStale")
            : $t("bar.coreWaiting")
      }}
    </span>

    <Drawer />
  </header>
</template>

<style scoped>
.bar {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-raised);
  border-bottom: var(--border-width) solid var(--color-border);
}
.search {
  flex: 1;
  max-width: 320px;
}
.chip[data-phase="connected"] {
  color: var(--color-text-accent);
}
.chip[data-phase="stale"] {
  color: var(--color-text-stop);
}
</style>
```

```bash
(cd gui && npm test && npm run build)
```

Atteso: **verde** — le prove di Impostazioni coi radio di `reka-ui`, la finestra di conferma per ruolo, la fascia che se ne
va (la prova di `frame.test.ts`: *«is there while waiting, and gone once connected»*), le tre prove di M-3, la prova della
tastiera nel browser, le prove di `axe`. E il pezzo JavaScript — `npm run build 2>&1 | grep -E 'assets/index-.*\.js '` — si
scrive nel commit accanto a quello del compito 1: qui **cresce**, perché i pezzi di base, le ventiquattro icone e il radio
di `reka-ui` entrano nei pannelli, e la cifra si porta al proprietario, che ha N-2 (R3-25, R2-13). ⛔ La direzione rossa
delle tre prove di M-3: `BaseStatus` rimesso dentro il `v-if` nei tre pannelli → tre rossi, `Unable to get [role="status"]`;
poi la copia salvata.

- [ ] **Passo 7: le due regole del linter su `panels/` e `frame/`**

In `gui/eslint.config.js`, *Trova* la chiusura del file:

```js
  {
    // The one file that may import `lucide`, and the only rule it keeps is the base pieces' one.
    name: "harness/imports/the-icon-map",
    files: ["src/components/icons.ts"],
    rules: { "no-restricted-imports": ["error", { patterns: [UPWARD] }] },
  },
];
```

*Sostituisci con:*

```js
  {
    // The one file that may import `lucide`, and the only rule it keeps is the base pieces' one.
    name: "harness/imports/the-icon-map",
    files: ["src/components/icons.ts"],
    rules: { "no-restricted-imports": ["error", { patterns: [UPWARD] }] },
  },
  {
    /**
     * ⛔ THE VIEWS ARE MADE OF PIECES (design system, section (b); criterion 5 of its perimeter): in the panels and in the
     * frame a button is `BaseButton` and a list is `BaseList`, and `reka-ui` is reached through the base pieces only.
     * ⚠️ `vue/no-restricted-html-elements` READS TEMPLATES: a `document.createElement("button")` in a `.ts` is invisible
     * to it (trap 5) -- `frame/BigTab.ts` until task 6.
     */
    name: "harness/panels-and-frame",
    files: ["src/panels/**/*.{vue,ts}", "src/frame/**/*.{vue,ts}"],
    rules: {
      "no-restricted-imports": [
        "error",
        { paths: [LUCIDE, { name: "reka-ui", message: "panels and frame use the base pieces, which sit on reka-ui (design system, section (b))" }] },
      ],
      "vue/no-restricted-html-elements": [
        "error",
        { element: ["button"], message: "a button is BaseButton (design system, section (b))" },
        { element: ["ul", "ol"], message: "a list is BaseList (design system, section (b))" },
      ],
    },
  },
];
```

```bash
(cd gui && npm run lint)
```

Atteso: **verde**. Poi le due direzioni, una alla volta, indietro con la **copia salvata** dopo ciascuna (vincolo 11):
`ViewBar.vue` l'ha riscritto questo compito, e `git checkout` gliene toglierebbe il lavoro (A-1):

| La violazione | Atteso |
|---|---|
| in `panels/Strip.vue` un `<button type="button">x</button>` nel template | rosso: *«a button is BaseButton»* |
| in `frame/ViewBar.vue` un `<ul><li>x</li></ul>` nel template | rosso: *«a list is BaseList»* |
| in `frame/moveActive.ts` la riga `import { DialogRoot } from "reka-ui";` | rosso: *«panels and frame use the base pieces»* |
| niente: `components/Confirm.vue` usa `BaseDialog`, e `components/BaseDialog.vue` importa `reka-ui` | verde |

- [ ] **Passo 8: M-3 col lettore di schermo vero — a mano (decisione 21)**

Un passo per il proprietario o per chi rivede, non per un subagente: `(cd gui && npm run dev)`, la pagina nel browser,
l'**Assistente vocale** di Windows acceso (`Win + Ctrl + Invio`). Poi, nella console:

1. al caricamento la fascia dice *«Il core non ha risposto.»*: si **annota** che cosa si sente, e non decide M-3 —
   `connection.phase` vale già `"waiting"`, quindi regione e parole nascono nello stesso montaggio, il caso che la
   costruzione non cura perché non c'è un «prima» vuoto (R3-13);
2. `harnessFake.deliver("Accepted")`: la fascia se ne va;
3. `harnessFake.deliver("StaleBuild")`: la fascia **rientra** nella regione che c'era già — si sente *«Il core parla una
   versione diversa del protocollo…»*?
4. nelle Impostazioni, `harnessFake.deliver("Policy")`, poi un clic su «Locale»: si sente *«Richiesta inviata: in attesa
   del core.»*?
5. `harnessFake.deliver("Verdict")`: si sente *«Ultima richiesta di VRAM: rifiutata…»*? — la terza regione, quella di
   Stato, nella vista Home accanto a Impostazioni (R3-13).

Il verbale — che cosa si è sentito, a ogni punto, con la data — va nella cella *Stato* della riga **5** della tabella della
posizione. ⛔ Se un annuncio dei punti 3, 4 o 5 **non** si sente, è una voce d'errata: M-3 non è chiusa, e lo si dice.

- [ ] **Passo 9: il cancello e il commit**

La riga **5** della tabella della posizione — **Stato** `✅ <data>` col verbale, e nella riga **4** la colonna **Commit** con
l'hash del compito 4 (R1-16) —; `bash scripts/gate.sh` da solo,
`bash scripts/check-docs.sh`, il commit — `design-system(compito 5): il kit al lavoro …` — coi fine-riga rimisurati, e
`git push`.

---
## Compito 6: il dock vestito — il tema nostro, le schede, la presa grande coi pezzi del kit

**Da:** la (c) del disegno, per intero; i controlli **15** e **16**; le trappole **5** e **9**; **P-6**, **P-7** e
**P-15**…**P-18** di questo piano; **D9**…**D11**; R3-17, R3-21, R3-22, R1-10 e A-2 della revisione.

**Files:**
- Create: `gui/src/tokens/readToken.ts`, `gui/src/tokens/dock.test.ts`, `gui/src/frame/BigTabFace.vue`,
  `gui/src/frame/dock.browser.test.ts`
- Rewrite: `gui/src/tokens/dock.css`, `gui/src/frame/BigTab.ts`, `gui/src/frame/bigtab.test.ts`, `gui/src/jsdom-setup.ts` —
  ciascuno **per intero**, col terminatore che ha oggi
- Modify: `gui/src/frame/dock.ts`, `gui/src/frame/Frame.vue`, `gui/src/frame/frame.test.ts`,
  `gui/src/tokens/tokens.browser.test.ts`; `gui/eslint.config.js` — un commento

**Interfaces:**
- Consumes: `shownTheme` di `tokens/theme.ts` (compito 1); il progetto `browser` (compito 2); `BaseLabel`, `BaseButton`,
  `isIconName` e `type IconName` (compito 3); `concentricRadii` di `testing/probes.ts` (compito 4).
- Produces: `readToken(name: string, element?: Element): string` da `tokens/readToken.ts` — il valore calcolato di un token,
  e un **errore** per un token che la pagina non definisce; `harnessTheme(): DockviewTheme` da `frame/dock.ts`; la classe
  `dockview-theme-harness`, che `tokens/dock.css` veste; `BigTabFace.vue` — props `title: string`, `icon?: IconName`, eventi
  `float` e `page`. Sotto jsdom, `base.css` caricato da `src/jsdom-setup.ts` (D9).

⚠️ **Che cosa la scrittura di questo compito ha misurato**, il 2026-09-23 nel Chrome installato 154 e sotto jsdom, sulla
cartella di prova coi compiti 1–5 applicati; le righe intere sono **P-15**…**P-18**, in testa al piano:

| | Il fatto | Che cosa ne fa il compito |
|---|---|---|
| 1 | la classe del tema va sul `.dv-shell`, figlio diretto del contenitore; il gruppo galleggiante è un `.dv-resize-container` con `role="dialog"`, e il suo `z-index` è **in linea**: `calc(var(--dv-overlay-z-index, 999) + 2i)` | la regola del livello sul contenitore (R3-17), e una prova con un gruppo galleggiante vero |
| 2 | P-6 misurato di nuovo: oltre ai nove colori dei gruppi di linguette, fuori le **cinque misure** della stessa funzione e la **tavolozza** `--dv-color-*`, che leggono solo le regole dei temi (P-15) | la prova le vuole tutte le altre nel tema nostro, e il suo rosso le elenca |
| 3 | sotto jsdom un `.css` importato è vuoto, e un `gap` letto da un token assente vale `NaN`: `dockview-core` lo prende, e `toJSON()` dice `null` per ogni misura (P-16) | `readToken` rifiuta un token assente; `src/jsdom-setup.ts` carica `base.css` dal file (D9) |
| 4 | i gruppi galleggianti si impilano in una lista **della pagina**, `+ 2` l'uno sull'altro (P-17) | il limite scritto in `dock.css`; la prova del browser smonta il suo dock |
| 5 | con un **margine** su `.dock`, il guscio di `dockview` occupa esattamente il contenuto: `clientWidth` è già la misura giusta, e la chiamata a `api.layout` resta com'è (P-7) | il margine in `Frame.vue`; `dock.ts` lo dice accanto alla chiamata |
| 6 | `axe` sul dock non giudica il contrasto — nessuna coppia fra i `passes`, le scritte fra gli `incomplete`, *«overlapped by another element»* — e trova tre difetti della parte 2 (P-18) | il contrasto resta il frammento a mano del passo 17 del compito 1, ripetuto al passo 8; i tre difetti sono del proprietario |

⚠️ **Tre forme della (c) che questo compito lascia com'erano**, dette qui perché chi confronta non le cerchi: `tabGroupIndicator`
di `themeAbyss` (`"none"`) non si porta, perché governa solo la linea dei gruppi di linguette, spenti (P-15); le **finestre
staccate** non si vestono, perché nella SPA non ci sono — vogliono `popoutUrl`, che è del guscio (D58 della parte 2); e
`dndOverlayMounting`, `dndPanelOverlay`, `dndTabIndicator` restano quelli di `themeAbyss`, i predefiniti, su cui le otto
mosse di SP-8 sono state giudicate.

- [ ] **Passo 1: rimisura il punto di partenza**

```bash
git status --porcelain > <scratchpad>/prima.txt
grep -n 'themeAbyss' gui/src/frame/dock.ts
grep -n -- '--dv-overlay-z-index: var' gui/node_modules/dockview/dist/styles/dockview.css
grep -n 'overlay-z-index' gui/node_modules/dockview-core/dist/package/main.esm.mjs
(cd gui && npm run build 2>&1 | grep -E 'assets/index-.*\.js ')
```

Atteso: `themeAbyss` nell'import e nell'opzione `theme` di `dock.ts`; in `dockview.css` 8.3.1 le due ridefinizioni **su sé
stesse** — `.dv-resize-container` e `.dv-render-overlay` —, `--dv-overlay-z-index: var(--dv-overlay-z-index, 999);`; in
`dockview-core` la riga di `AriaLevelTracker`, `` calc(var(--dv-overlay-z-index, 999) + ${i * 2}) ``. La riga del pezzo
JavaScript è la baseline del compito: va nel commit accanto a quella nuova (**N-2**). Poi, da solo, `bash scripts/gate.sh` →
`GATE GREEN`.

- [ ] **Passo 2: `readToken` — la prova, poi il modulo**

In `gui/src/tokens/tokens.browser.test.ts` (`replace_unique.py`), due sostituzioni. *Trova*:

```ts
import "./index";
import { watchTheme, type ThemeChoice } from "./theme";
```

*Sostituisci con:*

```ts
import "./index";
import { readToken } from "./readToken";
import { watchTheme, type ThemeChoice } from "./theme";
```

*Trova* — la fine del file:

```ts
    await expect.poll(() => root.dataset.theme).toBe("light");
    stop();
  });
});
```

*Sostituisci con:*

```ts
    await expect.poll(() => root.dataset.theme).toBe("light");
    stop();
  });

  it("reach TypeScript through `readToken` as the layout engine draws them, and a missing one is refused (control 16)", () => {
    const box = document.createElement("div");
    box.style.cssText = "position:absolute;width:var(--space-3)";
    document.body.append(box);
    // ⛔ TWO ORACLES THAT SHARE NO COPY: the token's text, and the width the engine draws from it.
    expect(readToken("--space-3")).toBe(`${box.getBoundingClientRect().width}px`);
    // ⛔ THE SECOND DIRECTION: a token the page does not define is an error, not "" -- a `gap` of NaN is what "" became.
    expect(() => readToken("--space-that-is-not")).toThrow(/is not defined here/);
  });
});
```

```bash
(cd gui && npx vitest run --project browser src/tokens/tokens.browser.test.ts)
```

Atteso: **rosso**, `Failed to import test file …/tokens.browser.test.ts` — il modulo non c'è, e il file intero non si
carica. Poi crea `gui/src/tokens/readToken.ts` (LF):

```ts
/**
 * A design token as the page computes it, for whoever needs it outside CSS (design system, section (c)): the dock's `gap`
 * today, the canvases of sub-projects 6, 7 and 12 tomorrow. The CSS variables are the truth (answer 14): this reads them,
 * it keeps no copy.
 *
 * ⛔ A TOKEN THE PAGE DOES NOT DEFINE IS AN ERROR, NOT AN EMPTY STRING. Measured under jsdom on 2026-09-23: a `gap`
 * parsed from "" is NaN, `dockview-core` 8.3.1 takes it without a word, and `toJSON()` then says `null` for every size --
 * a settle would have saved that into the core's package.
 */
export function readToken(name: string, element: Element = document.documentElement): string {
  const value = getComputedStyle(element).getPropertyValue(name).trim();
  if (value === "") throw new Error(`the token ${name} is not defined here: are the token sheets loaded?`);
  return value;
}
```

E la stessa corsa è **verde**, sei prove.

- [ ] **Passo 3: le prove del dock, prima del tema**

Crea `gui/src/tokens/dock.test.ts` (LF) — **P-6** e **P-15**:

```ts
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const HERE = dirname(fileURLToPath(import.meta.url));
const DOCKVIEW = join(HERE, "..", "..", "node_modules", "dockview", "dist", "styles", "dockview.css");

/** Every `--dv-*` a stylesheet SETS in the blocks that open with `selector {`, in the order they come. */
function setIn(css: string, selector: string): string[] {
  const found: string[] = [];
  let from = css.indexOf(`${selector} {`);
  while (from >= 0) {
    const body = css.slice(from, css.indexOf("}", from));
    for (const match of body.matchAll(/(--dv-[a-z0-9-]+)\s*:/g)) found.push(match[1] ?? "");
    from = css.indexOf(`${selector} {`, from + selector.length);
  }
  return found;
}

/**
 * ⛔ OUR THEME REPLACES `themeAbyss`, SO IT SETS WHAT `themeAbyss` SETS (design system, section (c); P-6 of the plan).
 * The reference is `.dockview-theme-abyss`, in its two blocks of `dockview.css` 8.3.1: a variable it sets and ours
 * forgot would fall back to `dockview`'s own default -- a colour by hand from a stylesheet we do not own. Two families
 * are out, each with its reason, measured in `dockview.css` 8.3.1 on 2026-09-23:
 *   `--dv-tab-group-*`  the TAB GROUPS, a feature the SPA does not turn on: only `.dv-tab-group-chip` and the
 *                       group's underline read them -- the nine colours and the five sizes alike;
 *   `--dv-color-*`      a theme's own PALETTE (`--dv-color-abyss-dark`, ...): every rule that reads one sits under a
 *                       `.dockview-theme-*` selector, the one that carries it.
 */
describe("our dockview theme", () => {
  const reference = setIn(readFileSync(DOCKVIEW, "utf8"), ".dockview-theme-abyss");
  const ours = new Set(setIn(readFileSync(join(HERE, "dock.css"), "utf8"), ".dockview-theme-harness"));

  it("sees the reference theme it replaces", () => {
    // ⛔ NON-VACUITY: a `dockview` that renamed its theme would leave nothing to compare, and a green.
    expect(reference.length).toBeGreaterThan(40);
  });

  it("sets every variable the reference theme sets, but the tab groups' and the palette's", () => {
    const needed = reference.filter((name) => !name.startsWith("--dv-tab-group-") && !name.startsWith("--dv-color-"));
    expect(needed.filter((name) => !ours.has(name))).toEqual([]);
  });
});
```

Crea `gui/src/frame/dock.browser.test.ts` (LF) — le schede, i raggi e il livello, nei due temi:

```ts
import "dockview/dist/styles/dockview.css";
import "../tokens";

import type { DockviewApi } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

import { registerModules } from "../panels/modules";
import { concentricRadii } from "../testing/probes";
import { readToken } from "../tokens/readToken";

import { createDock } from "./dock";

// ⛔ THE DRESSED DOCK IN THE INSTALLED CHROME (design system, section (c)): what only a layout engine can judge -- the
// space between the cards, their radius and surface, the level of a floating group. The stylesheets are the SPA's own,
// in the order `main.ts` loads them: `dockview.css` first, our tokens after it.

const docks: DockviewApi[] = [];

beforeEach(() => {
  setActivePinia(createPinia());
  registerModules();
});

afterEach(() => {
  // ⛔ DISPOSED, NOT ONLY DETACHED: `dockview-core` 8.3.1 stacks the floating groups of the whole PAGE in one module-level
  // list, `+ 2` per group, and a group left there lifts the next test's to 52 (measured on 2026-09-23).
  for (const api of docks.splice(0)) api.dispose();
  document.body.replaceChildren();
  delete document.documentElement.dataset.theme;
});

/** The dock of the Home view in one theme, on a host of the boards' size, once `dockview` has laid it out. */
async function dock(theme: "light" | "dark") {
  document.documentElement.dataset.theme = theme;
  const host = document.createElement("div");
  host.style.cssText = "width:1400px;height:800px";
  document.body.append(host);
  const api = createDock(host);
  docks.push(api);
  await new Promise((resolve) => setTimeout(resolve, 50));
  return { host, api };
}

/** What a declaration of `property: var(token)` computes to here: the oracle of a role, with no copy of its value. */
function computed(property: string, token: string): string {
  const probe = document.createElement("div");
  probe.style.setProperty(property, `var(${token})`);
  document.body.append(probe);
  const value = getComputedStyle(probe).getPropertyValue(property);
  probe.remove();
  return value;
}

for (const theme of ["light", "dark"] as const) {
  describe(`the dressed dock, ${theme} theme`, () => {
    it("draws every group as a card, `--space-3` from its neighbours", async () => {
      const { host } = await dock(theme);
      const groups = [...host.querySelectorAll(".dv-groupview")];
      // ⛔ NON-VACUITY: the Home view has groups side by side and one above the other.
      expect(groups.length).toBeGreaterThan(2);
      for (const group of groups) {
        const style = getComputedStyle(group);
        expect(style.borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
        expect(style.backgroundColor).toBe(computed("background-color", "--color-bg-surface"));
      }
      // The nearest neighbour on the right and below, where the two overlap: the distance between the facing edges.
      const boxes = groups.map((group) => group.getBoundingClientRect());
      const gaps: number[] = [];
      for (const a of boxes) {
        const right = boxes.filter((b) => b.left >= a.right && Math.min(a.bottom, b.bottom) > Math.max(a.top, b.top));
        const below = boxes.filter((b) => b.top >= a.bottom && Math.min(a.right, b.right) > Math.max(a.left, b.left));
        if (right.length > 0) gaps.push(Math.min(...right.map((b) => b.left - a.right)));
        if (below.length > 0) gaps.push(Math.min(...below.map((b) => b.top - a.bottom)));
      }
      expect(gaps.length).toBeGreaterThan(1);
      const space = Number.parseFloat(computed("width", "--space-3"));
      expect(gaps.filter((gap) => Math.abs(gap - space) > 0.5)).toEqual([]);
    });

    it("keeps every radius in it concentric (answer 4)", async () => {
      const { host } = await dock(theme);
      const report = concentricRadii([host]);
      // ⛔ NON-VACUITY (trap 1): a probe that met no corner near another is green for nothing.
      expect(report.near).toBeGreaterThan(0);
      expect(report.bad).toEqual([]);
    });

    it("floats a group BELOW the dialogs: at `--z-floating`, under `--z-overlay` (trap 9, R3-17)", async () => {
      const { host, api } = await dock(theme);
      const panel = api.getPanel("status");
      expect(panel).toBeDefined();
      if (panel !== undefined) api.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
      await new Promise((resolve) => setTimeout(resolve, 50));
      const floating = [...host.querySelectorAll(".dv-resize-container")];
      expect(floating).toHaveLength(1);
      const level = Number(getComputedStyle(floating[0] as Element).zIndex);
      expect(level).toBe(Number(readToken("--z-floating")));
      expect(level).toBeLessThan(Number(readToken("--z-overlay")));
    });
  });
}
```

In `gui/src/frame/frame.test.ts` (`replace_unique.py`), due sostituzioni. *Trova*:

```ts
import { pack_, useLayout, type LayoutPack } from "../stores/layout";
import { createFakeBridge, type FakeBridge } from "../transport/fakeBridge";

import { createDock } from "./dock";
```

*Sostituisci con:*

```ts
import { pack_, useLayout, type LayoutPack } from "../stores/layout";
import { shownTheme } from "../tokens/theme";
import { createFakeBridge, type FakeBridge } from "../transport/fakeBridge";

import { createDock, harnessTheme } from "./dock";
```

*Trova* — la fine del `describe("the dock", …)`:

```ts
    expect(api.getPanel("strip")?.params ?? {}).toEqual({});
    // And a module type nobody built still carries its own, from the registry.
    expect(api.getPanel("knowledge")?.params).toEqual(placeholderParams("knowledge"));
  });
});
```

*Sostituisci con:*

```ts
    expect(api.getPanel("strip")?.params ?? {}).toEqual({});
    // And a module type nobody built still carries its own, from the registry.
    expect(api.getPanel("knowledge")?.params).toEqual(placeholderParams("knowledge"));
  });

  it("wears our theme, and hands it to dockview again when the theme on screen changes (design system, section (c))", async () => {
    useLayout().attach(createFakeBridge());
    const where = host();
    const api = createDock(where);
    // ⛔ THE SHELL WEARS OUR CLASS, the one `tokens/dock.css` dresses -- and `themeAbyss`'s is gone (control 15).
    expect(where.querySelector(".dv-shell")?.classList.contains("dockview-theme-harness")).toBe(true);
    expect(where.querySelector(".dockview-theme-abyss")).toBeNull();
    const handed: unknown[] = [];
    const update = api.updateOptions.bind(api);
    api.updateOptions = (options) => {
      handed.push(options.theme?.colorScheme);
      update(options);
    };
    shownTheme.value = "light";
    await flush();
    shownTheme.value = "dark";
    await flush();
    // ⛔ WHAT THE DOCK WAS HANDED, NOT WHAT IT DRAWS: `colorScheme` draws nothing in `dockview-core` 8.3.1 (R3-21).
    expect(handed).toEqual(["light", "dark"]);
  });

  it("refuses a gap that is not a length in px, and builds the theme from the token that is", () => {
    document.documentElement.style.setProperty("--space-3", "0.75rem");
    try {
      // ⛔ `parseFloat` would read 0.75 and shrink the gap without a word.
      expect(() => harnessTheme()).toThrow(/not a length in px/);
    } finally {
      document.documentElement.style.removeProperty("--space-3");
    }
    // ⛔ THE SECOND DIRECTION: the token as the board has it, read from `base.css` (`src/jsdom-setup.ts`).
    expect(harnessTheme()).toMatchObject({ name: "harness", className: "dockview-theme-harness" });
    expect(harnessTheme().gap).toBeGreaterThan(0);
  });
});
```

```bash
(cd gui && npx vitest run --project jsdom src/tokens/dock.test.ts src/frame/frame.test.ts)
(cd gui && npx vitest run --project browser src/frame/dock.browser.test.ts)
```

Atteso: **rosso**, e per le ragioni giuste. `dock.test.ts`: la guardia verde, e la prova che elenca **tutte** le variabili che
il tema nostro non imposta ancora — `expected [ …(43) ] to deeply equal []` il 2026-09-23. `frame.test.ts`: `expected false to
be true` sul guscio, che porta ancora `dockview-theme-abyss`, e `harnessTheme` che non esiste — `expected [Function] to throw
error matching /not a length in px/ but got '(0 , __vite_ssr_import_13__.harnessTh…'`. `dock.browser.test.ts`, nei due temi:
`expected '0px' to be '20px'` sulle schede, ed `expected 999 to be 50` sul gruppo galleggiante — il difetto di R3-17 **già
oggi**; verdi, a questo punto, le due prove dei raggi concentrici. Un rosso per un'altra ragione è una voce d'errata.

- [ ] **Passo 4: il tema nostro — `dock.css`, `dock.ts`, il margine, i token sotto jsdom**

Riscrivi `gui/src/tokens/dock.css` per intero — il **ponte** del compito 1 esce (R1-10), le regole della presa grande
passano nella sua faccia (passo 5):

```css
/* OUR `dockview` THEME, `dockview-theme-harness` (design system, section (c); task 6 of its plan): `frame/dock.ts` puts the
   class on the dock's shell, and this sheet dresses it with our roles and tokens only -- no colour by hand (control 5,
   `usage.test.ts`). It sets every variable `themeAbyss` set, but the tab groups' and the palette's (`dock.test.ts`).

   ⛔ `frame/VueContent.ts` and `frame/BigTab.ts` hand `dockview` plain elements, because it asks for RENDERERS and not
   components: the classes they name -- `panel`, `bigtab` -- live here and not in a scoped <style> (E165, E166 of the
   part-2 plan). */

.dockview-theme-harness {
  /* ⛔ THE GRAB IS THE CONTAINER'S, NOT THE TAB'S (E170): `dockview` sizes the whole tab strip from this variable and clips
     `.dv-tabs-container`, so the variable moves and the tab fills it -- 40 px, the grab move 5 of SP-8 was judged on. */
  --dv-tabs-and-actions-container-height: var(--size-control-lg);
  --dv-tabs-and-actions-container-font-size: inherit;
  --dv-tab-font-size: inherit;
  --dv-tab-margin: 0;
  --dv-tab-border-radius: 0;
  --dv-tab-close-icon-size: var(--size-icon-sm);
  --dv-tabs-container-scrollbar-color: var(--color-border-strong);
  --dv-icon-hover-background-color: var(--color-bg-fill-hover);
  --dv-dropdown-border-radius: var(--radius-control);

  /* Every group is a CARD, header included (the (c)). The space AROUND the dock is the margin of `.dock` in `Frame.vue`, the
     space BETWEEN groups is the theme's `gap`: nothing is padding here (P-7 of the plan). */
  --dv-group-view-background-color: var(--color-bg-surface);
  --dv-tabs-and-actions-container-background-color: var(--color-bg-surface);
  --dv-border-radius: var(--radius-card);
  --dv-spacing-padding: 0;

  /* The visible tab of a group in full, the hidden ones muted (the (c)); the mark follows, below. */
  --dv-activegroup-visiblepanel-tab-background-color: var(--color-bg-surface);
  --dv-activegroup-hiddenpanel-tab-background-color: var(--color-bg-surface);
  --dv-inactivegroup-visiblepanel-tab-background-color: var(--color-bg-surface);
  --dv-inactivegroup-hiddenpanel-tab-background-color: var(--color-bg-surface);
  --dv-activegroup-visiblepanel-tab-color: var(--color-text);
  --dv-activegroup-hiddenpanel-tab-color: var(--color-text-muted);
  --dv-inactivegroup-visiblepanel-tab-color: var(--color-text);
  --dv-inactivegroup-hiddenpanel-tab-color: var(--color-text-muted);
  --dv-tab-divider-color: var(--color-border);
  --dv-paneview-header-border-color: var(--color-border);
  --dv-paneview-active-outline-color: var(--color-focus);

  /* The dividers are invisible until the pointer is on one (the (c)). */
  --dv-separator-border: transparent;
  --dv-sash-color: transparent;
  --dv-active-sash-color: var(--color-border-strong);
  --dv-sash-border-radius: var(--radius-full);
  --dv-active-sash-transition-duration: var(--duration-fast);
  --dv-active-sash-transition-delay: var(--duration-slow);

  /* The drop zone: the accent's subtle tint, the mark around it (the (c)). ⚠️ `--dv-drag-over-border` is this sheet's:
     `updateTheme` writes it inline only when the theme carries `dndOverlayBorder`, and ours does not -- one house. */
  --dv-drag-over-background-color: var(--color-bg-accent-subtle);
  --dv-drag-over-border-color: var(--color-mark);
  --dv-drag-over-border: var(--border-width) solid var(--color-mark);
  --dv-edge-dock-indicator-color: var(--color-mark);

  /* A floating group is raised, with the overlay's shadow (the (c)); its level is set on its container, below. */
  --dv-floating-box-shadow: var(--shadow-overlay);
  --dv-floating-border: var(--border-width) solid var(--color-border-card);
  --dv-floating-group-border: none;
  --dv-floating-group-dragging-opacity: 0.5;
  --dv-floating-titlebar-height: var(--size-target-min);
  --dv-floating-titlebar-background-color: var(--color-bg-raised);
  --dv-floating-titlebar-border-bottom: var(--border-width) solid var(--color-border);
  --dv-overlay-z-index: var(--z-floating);
}

/* The card: the radius, the border the dark theme draws and the light one leaves transparent, the shadow the light one
   casts. `border-box`, or the border would add to the size `dockview` gives the group. */
.dockview-theme-harness .dv-groupview {
  box-sizing: border-box;
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
}

/* The mark of the visible tab (the (c)): the icon of its label in `--color-mark`, the hidden tabs' in the tab's own colour.
   ⛔ Over `BaseLabel`'s scoped colour, which is muted everywhere else: here the tab's colour, above, decides. */
.dockview-theme-harness .dv-tab .base-label {
  color: inherit;
}
.dockview-theme-harness .dv-tab.dv-inactive-tab .base-label .base-icon {
  color: inherit;
}

/* ⛔ THE FLOATING CONTAINER RE-DECLARES THE LEVEL ON ITSELF, AS A CYCLE: `.dv-resize-container { --dv-overlay-z-index:
   var(--dv-overlay-z-index, 999) }` in `dockview.css` 8.3.1, so a value set on the shell never reaches it and the group
   floats at 999, over our dialogs at `--z-overlay` (R3-17 of the design-system review). The value goes on the container.
   ⚠️ DECLARED LIMIT: `dockview-core` lifts each open floating group 2 above the one before -- `+ 2i` inline, in the order
   they were last raised -- so with n of them the top one sits at `--z-floating` + 2(n - 1): under `--z-popover` up to 25,
   under `--z-overlay` up to 75, with the board's values of 2026-09-23. */
.dockview-theme-harness .dv-resize-container {
  --dv-overlay-z-index: var(--z-floating);
  --dv-group-view-background-color: var(--color-bg-raised);
  --dv-tabs-and-actions-container-background-color: var(--color-bg-raised);
  border-radius: var(--radius-card);
}
.dockview-theme-harness .dv-resize-container > .dv-floating-titlebar {
  border-radius: var(--radius-card) var(--radius-card) 0 0;
}
/* Inside a floating container the group is not a second card: the container is. */
.dockview-theme-harness .dv-resize-container .dv-groupview {
  border: 0;
  border-radius: 0 0 var(--radius-card) var(--radius-card);
  box-shadow: none;
}

/* The element `BigTab` hands to `dockview`, where it mounts the face of the grab: without a height of its own the face's
   `height: 100%` resolves against nothing. */
.bigtab {
  height: 100%;
}

/* The element `VueContent` hands to `dockview`: the same, for every `height: 100%` inside a panel. */
.panel {
  height: 100%;
}
```

In `gui/src/frame/dock.ts` (`replace_unique.py`), tre sostituzioni. *Trova*:

```ts
import { createDockview, themeAbyss, type DockviewApi, type SerializedDockview } from "dockview-core";
import { watch } from "vue";

import { componentFor, isBuilt, placeholderParams } from "../panels/registry";
import { useLayout, type LayoutPack, type ViewName } from "../stores/layout";
import { VIEWS } from "../panels/views";

import { BigTab } from "./BigTab";

const GRID = 24;
```

*Sostituisci con:*

```ts
import { createDockview, type DockviewApi, type DockviewTheme, type SerializedDockview } from "dockview-core";
import { watch } from "vue";

import { componentFor, isBuilt, placeholderParams } from "../panels/registry";
import { useLayout, type LayoutPack, type ViewName } from "../stores/layout";
import { VIEWS } from "../panels/views";
import { readToken } from "../tokens/readToken";
import { shownTheme } from "../tokens/theme";

import { BigTab } from "./BigTab";

const GRID = 24;

/** A token that is a length in px, as the number `dockview` wants. ⛔ THE UNIT IS CHECKED, NOT
 * DROPPED: `parseFloat("0.75rem")` is 0.75, and a board that moved a space to `rem` would shrink
 * the gap to nothing without a word. */
function pixels(name: string): number {
  const value = readToken(name);
  const found = /^(\d+(?:\.\d+)?)px$/.exec(value);
  if (found === null) throw new Error(`the token ${name} is "${value}", not a length in px`);
  return Number(found[1]);
}

/**
 * OUR `dockview` THEME (design system, section (c)): `tokens/dock.css` dresses its class, and the
 * space between the groups is the token `--space-3`, read and not retyped.
 *
 * ⚠️ `colorScheme` FOLLOWS THE THEME ON SCREEN, AS THE (c) WANTS, AND DRAWS NOTHING: in
 * `dockview-core` 8.3.1 `updateTheme` reads the class, the gap, the edge groups' size, the drop
 * border, the overlay's mounting and the tab groups' indicator -- not `colorScheme`, which the
 * library keeps for whoever reads its options (R3-21 of the design-system review).
 * ⚠️ NO `dndOverlayBorder`: the drop zone's border is `--dv-drag-over-border` in `dock.css`, which
 * `updateTheme` leaves to the sheet when the field is absent -- one house for a colour.
 */
export function harnessTheme(): DockviewTheme {
  return { name: "harness", className: "dockview-theme-harness", colorScheme: shownTheme.value, gap: pixels("--space-3") };
}
```

*Trova:*

```ts
  const api = createDockview(host, {
    // The theme the eight moves were judged on. Our own tokens dress what WE draw -- the bar,
    // the band, the drawer, the strip, the placeholder -- and the design system is decided in
    // its own three moments, none of which is this task.
    theme: themeAbyss,
```

*Sostituisci con:*

```ts
  const api = createDockview(host, {
    // ⛔ OUR THEME AND NOT `themeAbyss`, on which the eight moves were judged: that one is dark
    // only, and the design system has two themes (section (c), answers 5 and 18).
    theme: harnessTheme(),
```

*Trova:*

```ts
  api.layout(host.clientWidth, host.clientHeight);

  function show(view: ViewName): SerializedDockview {
```

*Sostituisci con:*

```ts
  // ⚠️ `clientWidth` IS THE CONTENT: the space around the dock is a MARGIN of `.dock` (`Frame.vue`),
  // outside the box, so the grid gets exactly the room it has (P-7 of the design-system plan).
  api.layout(host.clientWidth, host.clientHeight);

  // The theme follows the one on screen (the (c)): `updateOptions` hands it to `updateTheme` again.
  watch(shownTheme, () => api.updateOptions({ theme: harnessTheme() }));

  function show(view: ViewName): SerializedDockview {
```

In `gui/src/frame/Frame.vue`, *Trova*:

```css
.dock {
  flex: 1;
  min-height: 0;
}
```

*Sostituisci con:*

```css
/* Answer 20 of the design system: the dock -- the strip is its last row -- 12 px from the sides and 24 from the bottom,
   away from the window's corners; the bar above has none. A MARGIN and not a padding, so `clientWidth` is the room the
   grid gets (P-7 of its plan). */
.dock {
  flex: 1;
  min-height: 0;
  margin: 0 var(--space-3) var(--space-6);
}
```

Riscrivi `gui/src/jsdom-setup.ts` per intero — il falso di `ResizeObserver` resta com'è, e sotto arriva `base.css` (D9):

```ts
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// ⛔ `jsdom` 30.0.1 DOES NOT IMPLEMENT `ResizeObserver`, and `dockview-core` calls it the moment
// a grid is created (`watchElementResize`): without this, every probe that mounts a grid dies on
// `ReferenceError: ResizeObserver is not defined` before any layout question is asked --
// measured on 2026-09-16 (R6-8 of the in-depth review of task 13). Mounted by `test.setupFiles`.
//
// ⚠️ THIS FAKES AN ABSENT API, NOT LAYOUT: it observes nothing, and `getBoundingClientRect` still
// answers zeros under jsdom, which is why the geometry of `moveActive` is probed with rectangles
// of our own (P-97). The day jsdom ships a `ResizeObserver`, `??=` leaves it alone.
globalThis.ResizeObserver ??= class {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
};

// ⛔ THE TOKENS THAT DO NOT CHANGE WITH THE THEME, AS THE BROWSER HAS THEM (design system, task 6):
// under jsdom Vitest does not process an imported `.css` -- the import is empty -- so `readToken`
// would find no token, and `createDock` no `gap` (measured on 2026-09-23: `--space-3` reads "" when
// imported, "12px" from a <style>). `base.css` is READ, not retyped: the values stay in the board's
// copy. `themes.css` stays out: no probe under jsdom reads a colour.
const sheet = document.createElement("style");
sheet.textContent = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "tokens", "base.css"), "utf8");
document.head.append(sheet);
```

```bash
(cd gui && npx vitest run --project jsdom src/tokens src/frame && npx vitest run --project browser src/frame src/tokens)
```

Atteso: **verde** — P-6, le prove del dock sotto jsdom, le schede a `--space-3` col raggio della scheda, i raggi concentrici
e il gruppo galleggiante a `--z-floating`, nei due temi; e ancora verdi le prove di `bigtab.test.ts`, che il passo 5
riscrive. Il 2026-09-23, sulla cartella di prova: jsdom, otto file e 37 prove; browser, due file e dodici prove.

- [ ] **Passo 5: la presa grande coi pezzi del kit — la prova, poi la faccia**

Riscrivi `gui/src/frame/bigtab.test.ts` per intero:

```ts
import type { TabPartInitParameters } from "dockview-core";
import { describe, expect, it } from "vitest";

import { i18n } from "../i18n";

import { BigTab } from "./BigTab";

const t = i18n.global.t;

function parameters(id: string, title?: string) {
  const calls: string[] = [];
  const api = {
    id,
    isMaximized: () => false,
    maximize: () => calls.push("maximize"),
    exitMaximized: () => calls.push("exit"),
  };
  const containerApi = {
    getPanel: (wanted: string) => (wanted === id ? { id } : undefined),
    addFloatingGroup: () => calls.push("float"),
  };
  return { calls, init: { api, containerApi, title, params: {} } as unknown as TabPartInitParameters };
}

describe("the big tab", () => {
  it("shows a module's Italian name with the module's icon, and a plain panel's own title without one", () => {
    const status = new BigTab();
    status.init(parameters("status").init);
    expect(status.element.querySelector(".base-label")?.textContent?.trim()).toBe(t("modules.status"));
    expect(status.element.querySelector('.base-label svg[data-icon="status"]')).not.toBeNull();
    const other = new BigTab();
    other.init(parameters("not-a-module", "Titolo dato").init);
    expect(other.element.querySelector(".base-label")?.textContent?.trim()).toBe("Titolo dato");
    expect(other.element.querySelector(".base-label svg")).toBeNull();
  });

  it("carries two named commands of the kit, that run and do not start a drag", () => {
    const { calls, init } = parameters("status");
    const tab = new BigTab();
    tab.init(init);
    const buttons = [...tab.element.querySelectorAll("button")];
    expect(buttons.map((b) => b.getAttribute("aria-label"))).toEqual([t("menu.float"), t("menu.page")]);
    // ⛔ THE KIT'S PIECES, NOT GLYPHS IN A BUTTON BUILT BY HAND (section (c); trap 5 of the design system).
    expect(buttons.map((b) => b.querySelector("svg.base-icon")?.getAttribute("data-icon"))).toEqual(["float", "fullPage"]);
    let reachedTheTab = 0;
    tab.element.addEventListener("pointerdown", () => {
      reachedTheTab += 1;
    });
    buttons[0]?.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
    // ⛔ THE SECOND DIRECTION: the same event on the label DOES reach the tab -- so the count below
    // is one because of the stop on the button, not because nothing bubbles.
    tab.element.querySelector(".base-label")?.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
    expect(reachedTheTab).toBe(1);
    buttons[0]?.click();
    buttons[1]?.click();
    expect(calls).toEqual(["float", "maximize"]);
  });

  it("takes its face away when dockview disposes of the tab", () => {
    const tab = new BigTab();
    tab.init(parameters("status").init);
    expect(tab.element.childElementCount).toBeGreaterThan(0);
    tab.dispose();
    // ⛔ ONE VUE APP PER TAB: a tab `dockview` throws away that kept its app would leak a reactive tree.
    expect(tab.element.childElementCount).toBe(0);
  });
});
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/bigtab.test.ts)
```

Atteso: **rosso**, tre prove — `expected undefined to be 'Stato'`, `expected [ undefined, undefined ] to deeply equal [ 'float',
'fullPage' ]`, `TypeError: tab.dispose is not a function`. Poi crea `gui/src/frame/BigTabFace.vue` (LF) — D11:

```vue
<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseLabel from "../components/BaseLabel.vue";
import type { IconName } from "../components/icons";

/**
 * The face of the big grab handle (design system, section (c)): the module's label with its icon, and the two commands as
 * base pieces -- `BigTab.ts` mounts it in the element `dockview` drags.
 *
 * ⛔ A PRESS ON A COMMAND MUST NOT START A DRAG, and stopping `click` alone is not enough: `dockview` begins the drag on
 * `pointerdown`/`mousedown`, so both stop on the button. Measured in SP-8; without it, every press of a command drags the
 * tile a few pixels first.
 */
defineProps<{ title: string; icon?: IconName }>();
const emit = defineEmits<{ float: []; page: [] }>();
</script>

<template>
  <div class="face">
    <BaseLabel :icon="icon" class="title">{{ title }}</BaseLabel>
    <BaseButton variant="quiet" size="sm" icon="float" :label="$t('menu.float')" @pointerdown.stop @mousedown.stop @click.stop="emit('float')" />
    <BaseButton variant="quiet" size="sm" icon="fullPage" :label="$t('menu.page')" @pointerdown.stop @mousedown.stop @click.stop="emit('page')" />
  </div>
</template>

<style scoped>
.face {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  height: 100%;
  padding: 0 var(--space-2) 0 var(--space-3);
  cursor: grab;
  user-select: none;
}
.title {
  flex: 1;
  min-width: 0;
}
</style>
```

Riscrivi `gui/src/frame/BigTab.ts` per intero:

```ts
import type { ITabRenderer, TabPartInitParameters } from "dockview-core";
import { createApp, type App } from "vue";

import { isIconName } from "../components/icons";
import { i18n } from "../i18n";
import { isModule } from "../panels/registry";

import BigTabFace from "./BigTabFace.vue";

/**
 * The big grab handle (move 5): the tab element is what `dockview` drags, so a big tab is a big
 * grab -- which is what makes a pointer that is a HAND able to take it (move 8, ADR-0039).
 *
 * ⛔ THE FACE IS THE KIT'S (design system, section (c)): `BigTabFace.vue`, a `BaseLabel` with the
 * module's icon and two `BaseButton`s, mounted here as `VueContent` mounts a panel -- ONE VUE APP
 * PER TAB, AND `unmount` ON `dispose`. Before, the two commands were `<button>`s built with
 * `document.createElement`, which the template linter cannot see (trap 5 of the design).
 *
 * ⛔ TWO COMMANDS AND NOT THREE (D58): "float" and "full page" are the library's; "in a separate
 * window" needs `popoutUrl` and a page served from an http(s) origin, which is the shell's (Q3 of
 * SP-8, P-91), and the shell is outside this plan. Each command has a name from the locale:
 * reachable with the tab key, read by a screen reader (G20).
 *
 * ⚠️ DECLARED LIMIT (M-1 of the review, E187): on a FLOATING group `maximize()` is a no-op in
 * `dockview-core` 8.3.1 -- measured on 2026-09-22, rectangle unchanged, no error -- so the second
 * command does nothing on a tile the first one detached, and is not disabled there. A button that
 * follows `api.location` is a second occurrence of the same kit question, and waits for it.
 */
export class BigTab implements ITabRenderer {
  readonly element = document.createElement("div");
  private app?: App;

  init(parameters: TabPartInitParameters): void {
    this.element.className = "bigtab";
    const id = parameters.api.id;
    this.app = createApp(BigTabFace, {
      // ⛔ THE MODULE'S ITALIAN NAME AND NOT THE PANEL'S ID (G21, P-95): the views carry `title: id`,
      // and an id is code. A panel that is not a module type keeps the title it was given, and no icon.
      title: isModule(id) ? i18n.global.t(`modules.${parameters.api.id}`) : (parameters.title ?? id),
      icon: isModule(id) && isIconName(id) ? id : undefined,
      onFloat: () => {
        const panel = parameters.containerApi.getPanel(id);
        if (panel !== undefined) parameters.containerApi.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
      },
      onPage: () => {
        if (parameters.api.isMaximized()) parameters.api.exitMaximized();
        else parameters.api.maximize();
      },
    });
    this.app.use(i18n);
    this.app.mount(this.element);
  }

  dispose(): void {
    this.app?.unmount();
    this.app = undefined;
  }
}
```

E in `gui/eslint.config.js` il commento del blocco `harness/panels-and-frame`, che il compito rende falso (gotcha #58).
*Trova*:

```js
     * to it (trap 5) -- `frame/BigTab.ts` until task 6.
```

*Sostituisci con:*

```js
     * to it (trap 5). `frame/BigTab.ts` built its two commands so until task 6, and mounts the kit's pieces since.
```

⚠️ Il commento di `gui/src/locales/copy.test.ts` che nomina `` `modules.${parameters.api.id}` in `BigTab.ts` `` resta vero:
la chiave si costruisce ancora lì, con la stessa espressione.

```bash
(cd gui && npx vitest run --project jsdom src/frame/bigtab.test.ts && npm run lint)
```

Atteso: **verde**, tre prove, e il linter verde — la faccia è un `.vue`, e le regole di `harness/panels-and-frame` la
leggono: niente `<button>` scritto a mano, niente scritta fuori da `it.json`.

- [ ] **Passo 6: tutte le prove, il *build*, il linter**

```bash
(cd gui && npm test && npm run build && npm run lint)
```

Atteso: **verde** su tutto; il conto dei file di prova più alto di quello del compito 5 di **due**, `dock.test.ts` e
`dock.browser.test.ts`. La riga del pezzo JavaScript — `npm run build 2>&1 | grep -E 'assets/index-.*\.js '` — va nel commit
accanto a quella del passo 1: il 2026-09-23, sulla cartella di prova, da 689,58 kB a **690,50 kB**, compressi da 209,99 a
210,37 — `themeAbyss` esce, la faccia entra (**N-2**, la cifra per il proprietario).

- [ ] **Passo 7: le due direzioni**

Una violazione alla volta, poi indietro con la **copia salvata** e `cmp` (vincolo 11): `dock.css`, `BigTab.ts` e
`jsdom-setup.ts` il compito li ha riscritti, e `readToken.ts`, `dock.test.ts` e `BigTabFace.vue` sono nati qui — `git
checkout` non conosce i secondi e toglierebbe ai primi il lavoro del compito (A-1).

| La prova | La violazione messa a mano | Atteso, misurato il 2026-09-23 |
|---|---|---|
| `dock.test.ts`, P-6 | in `dock.css` tolta, dal blocco `.dockview-theme-harness`, la riga `--dv-overlay-z-index: var(--z-floating);` — l'ultima del blocco, non quella del contenitore galleggiante | rosso: `expected [ '--dv-overlay-z-index' ] to deeply equal []` |
| `dock.test.ts`, la guardia | nella prova, `".dockview-theme-abyss"` → `".dockview-theme-abyss-that-is-not"` | rosso: `expected 0 to be greater than 40` — ⛔ e la prova di P-6, senza niente da confrontare, **verde**: è la ragione della guardia |
| `bigtab.test.ts`, l'icona | in `BigTab.ts` `icon: undefined,` al posto della riga che la sceglie | rosso: `expected null not to be null` |
| `bigtab.test.ts`, il trascinamento | in `BigTabFace.vue` tolto `@pointerdown.stop` dal primo pulsante | rosso: `expected 2 to be 1` |
| `bigtab.test.ts`, lo smontaggio | in `BigTab.ts` tolta la riga `this.app?.unmount();` | rosso: `expected 1 to be +0` |
| `frame.test.ts`, il tema che segue | in `dock.ts` tolta la riga del `watch(shownTheme, …)` | rosso: `expected [] to deeply equal [ 'light', 'dark' ]` |
| `frame.test.ts`, il `gap` in px | in `dock.ts` le due righe dopo `exec(value)` sostituite da `return Number.parseFloat(value);` | rosso: `expected [Function] to throw an error` |
| i token sotto jsdom | in `src/jsdom-setup.ts` tolta la riga `document.head.append(sheet);` | rosso, **cinque** prove di `frame.test.ts` — le quattro che montano il dock e quella del `gap` —, tutte con `Error: the token --space-3 is not defined here: are the token sheets loaded?` |
| `tokens.browser.test.ts`, `readToken` | in `readToken.ts` tolta la riga dell'`if` | rosso: `expected [Function] to throw an error` |
| `dock.browser.test.ts`, le schede | in `dock.css`, nel blocco `.dockview-theme-harness .dv-groupview`, tolta `border-radius: var(--radius-card);` | rosso, nei due temi: `expected '0px' to be '20px'` |
| `dock.browser.test.ts`, la distanza | in `dock.ts` `gap: 0 };` al posto di `` gap: pixels("--space-3") }; `` | rosso, nei due temi: `expected [ Array(10) ] to deeply equal []`, le distanze a 0 — ⚠️ e anche i raggi concentrici: con le altezze cambiate un radio di Impostazioni finisce a 13 px dall'angolo della sua scheda, `radio in dv-groupview, bottom-left: radius 12.0, outer 20.0, distance 13.0/13.0` |
| `dock.browser.test.ts`, i raggi | in `dock.css`, prima del commento *«The mark of the visible tab»*, `.dockview-theme-harness .dv-tabs-and-actions-container { border-radius: var(--radius-control); }` | rosso, nei due temi: `dv-tabs-and-actions-container in dv-groupview, top-left: radius 8.0, outer 20.0, distance 1.0/1.0` |
| `dock.browser.test.ts`, il livello | in `dock.css` tolta, dal blocco `.dockview-theme-harness .dv-resize-container`, la riga `--dv-overlay-z-index: var(--z-floating);` | rosso, nei due temi: `expected 999 to be 50` — R3-17 |

Alla fine, dalla radice del repository, `git status --porcelain | diff <scratchpad>/prima.txt -` rende soltanto i file del
compito: nessun file nato dai rossi del browser (R2-3).

- [ ] **Passo 8: guardarlo, nei due temi**

`(cd gui && npm run dev)`, la pagina nel browser a 1440 × 900, e nella console `harnessFake.deliverAll()`. Poi, per ciascun
tema — `document.documentElement.dataset.theme = "light"` e `"dark"` —, il frammento del passo 17 del compito 1: il contrasto
di ogni scritta del dock sul primo fondo pieno dei suoi antenati.

```js
(() => {
  const rgb = (c) => c.match(/[\d.]+/g).map(Number);
  const lum = ([r, g, b]) => [r, g, b].map((v) => ((v /= 255) <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4)).reduce((s, v, i) => s + v * [0.2126, 0.7152, 0.0722][i], 0);
  const ratio = (a, b) => (Math.max(lum(a), lum(b)) + 0.05) / (Math.min(lum(a), lum(b)) + 0.05);
  const back = (el) => { for (let e = el; e; e = e.parentElement) { const c = rgb(getComputedStyle(e).backgroundColor); if (c.length === 3 || c[3] === 1) return c; } return [255, 255, 255]; };
  const own = (el) => [...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim() !== "");
  const seen = [...document.querySelectorAll(".dock *")].filter((el) => own(el) && el.getBoundingClientRect().width > 0);
  const worst = seen.map((el) => [Math.round(ratio(rgb(getComputedStyle(el).color), back(el)) * 100) / 100, el.textContent.trim().slice(0, 20)]).sort((a, b) => a[0] - b[0]);
  return { theme: document.documentElement.dataset.theme, seen: seen.length, worst: worst.slice(0, 3) };
})()
```

Atteso: `seen` sopra zero e il primo di `worst` **sopra 4,5** — il 2026-09-23, sulla cartella di prova, 6,22 nel chiaro e 6,41
nello scuro, su 33 scritte, le etichette delle linguette comprese. ⚠️ Il **ponte** del compito 1 non c'è più: a tenere
leggibili pannelli e linguette è ora il tema nostro, e i due comandi della presa grande sono icone `currentColor` di un
`BaseButton` discreto — `--color-text-muted` sulla superficie, una coppia che `contrast.test.ts` già giudica (A-2).

Poi il livello, sulla SPA vera: «Stacca la tessera» su Stato, poi «+ moduli»; nella console

```js
(() => { const f = document.querySelector(".dv-resize-container").getBoundingClientRect(); return document.elementFromPoint(f.left + f.width / 2, f.top + f.height / 2)?.className; })()
```

Atteso: `base-dialog-veil` — il velo del cassetto **sopra** il gruppo galleggiante. Poi si **guarda**, nei due temi: le
schede, le linguette col segno, i divisori che compaiono al passaggio, la zona d'arrivo trascinando una linguetta. ⛔ Il
bersaglio è la Home della tavola dello stile, e l'aspetto lo **giudica il proprietario** alla prima prova (controllo 15): ciò
che non gli piace è una voce d'errata col suo *«perché»*, non un ritocco di chi esegue.

- [ ] **Passo 9: il cancello, il commit, la posizione**

La riga **6** della tabella della posizione — **Stato** `✅ <data>`, e nella riga **5** la colonna **Commit** con l'hash del
compito 5 (R1-16) —; `bash scripts/gate.sh` da solo, `bash scripts/check-docs.sh`, il commit — `design-system(compito 6): il
dock vestito …`, con le due righe del pezzo JavaScript — coi fine-riga rimisurati, e `git push`.

---
## Come si riprende — il registro applicato, 2026-09-23

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e i **compiti 1–5**, rivisti e **corretti** — le 73 correzioni
del [registro](2026-09-23-design-system-revisione/ledger.md) sono nel piano, nel disegno e nella tavola dal commit
`7c42748`, e il registro ha ogni riga a ✅. Da scrivere: i compiti **6–9** e la Definizione di «fatto». La consegna
precedente sta parola per parola in
[`archivio/consegna-piano-design-system.md`](../../archivio/consegna-piano-design-system.md).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **cancello** | `GATE GREEN` all'apertura e prima di ogni commit della sessione: si rilancia, non si cita — `bash scripts/gate.sh`, **da solo** |
| **la CI** | le corse di `9ca93c8`, `7c42748` e `a4dfd81` sono verdi su tutti e due i sistemi; quella del commit che scrive questa riga era **in corso** alla chiusura, e la sessione dopo la legge per prima, coi comandi di [`porta-di-qualita.md`](../../porta-di-qualita.md), *«Leggere la CI da terra»*. Il rosso `windows-latest` di `833e6b2`, di causa **ignota** — il log rende 403 —, non è tornato |
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
