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
| 9 | **i fine-riga si conservano per file**: i file di `gui/` sono `i/lf` nell'indice e, su questa macchina, CRLF nell'albero (`git ls-files --eol`); i file nuovi nascono **LF**; un CRLF si tocca con Python, **mai** `sed -i` | `CLAUDE.md`; trappola 16 del disegno |
| 10 | **ogni conteggio si rifà col comando**: le cifre di questo piano sono istantanee del 2026-09-23 su `fd2812b` | `CLAUDE.md` |
| 11 | **ogni prova si prova nelle due direzioni**: che scatti dove deve — una violazione messa a mano, poi tolta — e che non scatti dove non deve; e ogni prova del browser porta la **guardia di non-vacuità**, quante cose ha guardato, maggiore di zero | `CLAUDE.md`; (f) |
| 12 | **il kernel non cambia**, e nemmeno il filo: `git diff --stat <base>..HEAD -- crates/ gui/schema/` vuoto a ogni compito. Tutto ciò che è nuovo nel pacchetto della disposizione è un campo **facoltativo** dei byte opachi | (a), (d); I1, I4 |
| 13 | **il compendio resta sotto il tetto**: margine misurato prima e dopo ogni tocco, `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh`; se va rosso si toglie prosa dalla §6, **non si alza il tetto** | §13 del compendio, gotcha #100 |
| 14 | **nessun link `](…)` a un file che non esiste ancora**: si nomina in code span, e il link nasce nel commit che crea il file | trappola 5 di `check-docs.sh` |
| 15 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore**; il cancello e `check-docs.sh` girano **prima**, uno alla volta; il messaggio comincia con `design-system(compito N): …` | `CLAUDE.md` |

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

⏳ **IL PIANO È IN SCRITTURA dal 2026-09-23.** A dirlo non è questa riga ma i due comandi, che coincideranno a piano scritto:
`grep -c '^## Compito' <questo file>` e le righe `| **N** |` della tabella qui sotto. ⛔ **Nessun compito si esegue** prima del
pre-controllo, in una sessione sua.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | **i token**: `base.css` e `themes.css` copiati dalla tavola da uno script, e il test che li tiene uguali; `dock.css` con le regole che oggi stanno in `tokens.css`; `tokens/index.ts` coi due caratteri; il tema sulla radice (`tokens/theme.ts`) e il campo `theme` del pacchetto; i nomi nuovi nei dodici componenti; `tokens.css` esce. Le prove: il contrasto per famiglie, gli stessi ruoli nei due temi, nessuna scala fuori dai token, nessun colore a mano, il tema nelle due direzioni | — | ⬜ |
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
| **P-1** | ⛔ **la regola del contrasto non è «ogni testo su ogni fondo».** La decisione 14 dice che il test accoppia *«ogni `--color-text-*` con ogni `--color-bg-*`»*; alla lettera **fallisce per costruzione** — nello scuro `--color-text-on-ok` e `--color-bg` sono lo stesso `--ref-neutral-5`. Le **176 coppie** approvate con la tavola, *«zero sotto la soglia»*, le ha prodotte la funzione `pairs()` di `palette.py`, per **famiglie**: sei testi su undici fondi a 4,5:1; `text-on-accent` sui tre fondi d'accento e `text-on-X` sul suo `bg-X` a 4,5:1; bordo forte, focus, segno e bordo d'accento sui quattro fondi di base a 3:1 — 88 per tema | `palette.py` nello scratchpad della terza sessione del 2026-09-23, `…\9ae312b8-b35c-470a-90ee-519a47a2f7df\scratchpad\`, righe 92–104 — ⚠️ uno scratchpad non è per sempre: la regola è copiata **qui**, parola per parola, nel compito 1 | il compito 1 scrive le famiglie **dai nomi**, non una lista di coppie, e una guardia che **ogni** ruolo sia in una famiglia o nell'elenco degli esenti: un ruolo nuovo non sfugge in silenzio |
| **P-2** | ⛔ **il linter non legge i `.ts`.** `eslint src` passa i `.vue` e i `.json`; i `.ts` non sono di nessuno (P-101 della parte 2, nel commento di `gui/eslint.config.js`). Le regole di `no-restricted-imports` della (b) sarebbero **cieche** su `icons.ts`, `BigTab.ts`, `dock.ts` | `grep -n 'P-101' gui/eslint.config.js`; `vue/essential/rules` non ha `files`, e così i blocchi nostri — letto nei pacchetti installati | il compito 3 aggiunge un blocco che legge i `.ts` col parser di TypeScript, e prova che il `npm run lint` resti verde sul codice di oggi e diventi rosso su una violazione in un `.ts` |
| **P-3** | ⚠️ **la regola «in `components/` nessun import di `pinia`, `stores/`…» morderebbe `Confirm.vue`**, che la (b) stessa tiene in `components/` come pezzo **composto** e che legge due negozi | `grep -n 'stores' gui/src/components/Confirm.vue` | il compito 3 la applica ai **pezzi di base** — `components/Base*.vue` e `components/icons.ts` — che è il principio della riga *«i pezzi di base … mai lo stato globale»* della (b) |
| **P-4** | **`base.css` uguale alla tavola non può portare la regola della pagina** `html, body, #app { height: 100%; margin: 0; }`, che oggi sta in `tokens.css` | `sed -n '52,57p' gui/src/tokens/tokens.css` | il compito 1 la sposta in uno `<style>` **non** scoped di `App.vue`, la radice che monta la cornice |
| **P-5** | **i caratteri non possono entrare da `base.css`**, per la stessa ragione: la tavola li carica con `<link>` da jsDelivr | `grep -n 'fontsource' docs/superpowers/specs/2026-09-22-design-system-tavole/token.html` | li importa `gui/src/tokens/index.ts`, l'ingresso unico dei token, per la SPA e per la pagina kit |
| **P-6** | **le variabili `--dv-*` del CSS di `dockview` sono 103**, ma quelle d'una tavolozza — `--dv-color-abyss*`, `-gh-`, `-mocha-`, `-monokai-`, `-nord-`, `-sol-` — le legge solo il tema che le porta. *«Ogni variabile che il CSS usa»* si legge come **ogni variabile che il tema di riferimento `.dockview-theme-abyss` imposta**, nei suoi due blocchi | `grep -o 'var(--dv-[a-z0-9-]*' gui/node_modules/dockview/dist/styles/dockview.css \| sort -u \| wc -l`; `awk '/^\.dockview-theme-abyss \{/,/^\}/' …` | il compito 6 lo prova con un test che legge `dockview.css` e `dock.css`: ogni variabile del tema di riferimento è impostata dal nostro, tranne i nove colori dei gruppi di linguette, una funzione che la SPA non accende |
| **P-7** | **lo spazio attorno al dock lo decide chi chiama `layout`**: `updateTheme` di `dockview-core` 8.3.1 applica `gap` come margine della griglia, e nel JavaScript `--dv-spacing-padding` non compare | `grep -n 'spacing-padding' gui/node_modules/dockview-core/dist/package/main.esm.mjs` non rende nulla; `updateTheme` alla riga che `grep -n 'updateTheme() {'` dà sullo stesso file | il compito 6 mette il margine sul contenitore `.dock` — 0 in alto, 12 ai lati, 24 in basso, la risposta 20 — e passa a `api.layout` il **contenuto** del contenitore |
| **P-8** | **il radio di `reka-ui` 2.10.4 lo decide chi lo controlla**: `RadioGroupItem` chiama `changeModelValue`, e con un `modelValue` dato la scelta si vede solo quando il valore torna; l'evento `select` si può fermare con `preventDefault` | `gui/node_modules/reka-ui/dist/RadioGroup/RadioGroupItem.js` e `Radio.js`, letti | il compito 5 lega la policy VRAM al valore del core: il radio si muove quando torna `Policy`, e le righe scritte a mano per E184 non servono più — le prove di `modules.test.ts` si riscrivono su `[role=radio]`, senza perdere ciò che provano (trappola 10) |
| **P-9** | **una carta della Panoramica e il pulsante «moduli» della striscia sono pulsanti**, e la regola del linter vieta `<button>` in `frame/` e `panels/`: servono a `BaseButton` due forme — `variant="card"` e `pill` — la seconda **imposta dalla regola dei raggi**: dentro una pillola va una pillola | (b), (d); la tavola dello stile, `.m-strip .sp` | il compito 3 le mette in `BaseButton`, con la seconda occorrenza scritta accanto: le carte delle viste e *«Salva questa vista»*; il pulsante della striscia e il chip del core |
| **P-10** | **il logo «harness» delle tavole non è nella (d)**, che elenca da sinistra il nome della vista, la ricerca e il chip | la tabella *«La barra»* della (d); `panoramica.html`, `.m-logo` | non si costruisce: il nome del programma non è deciso — «Agentic OS» è il nome del proprietario per il programma, *harness* quello del repository |
| **P-11** | **F3 è libero** in `gui/src`; i `Ctrl+1`, `Ctrl+2` della tavola della Panoramica sono esempi che la (d) non prende | `grep -rn 'F3' gui/src` non rende nulla; la (d): *«con F3 o col clic … le frecce muovono, Invio entra, Esc chiude»* | il compito 8 costruisce F3, frecce, Invio ed Esc, e le carte non mostrano scorciatoie |
| **P-12** | **jsdom non ha `matchMedia`** | `grep -rn 'matchMedia' gui/node_modules/jsdom/lib \| head -1` non rende nulla | il tema si prova con un `matchMedia` finto sotto jsdom (compito 1) e con la query vera nel browser (compito 2) |
| **P-13** | la **provenienza** della chat usa `var(--warn)` come bordo, e il ruolo nuovo per un bordo che deve leggersi a 3:1 non c'è: `--color-border-warn` è una tinta di decoro | `grep -n 'provenance' gui/src/panels/Chat.vue`; la (a), le famiglie di P-1 | il compito 1 usa `--color-text-warn` per quel bordo, detto accanto: supera 4,5:1 sui fondi, quindi 3:1; e sono **le parole** a portare la provenienza (trappola 7) |
| **P-14** | ⚠️ **il pre-controllo va in una sessione sua**: il punto 7 del *«Come si riprende»* del disegno lo metteva *«nella sessione che scrive il piano»*; `CLAUDE.md`, rivisto dopo lo stesso giorno, dice *«brainstorming, disegno, piano, pre-controllo, ogni compito: ciascuno nella sua sessione»* | la riga *«Una fase per sessione»* di `CLAUDE.md` nasce in `eab020d`, dopo il disegno di `386fc5c`: `git show -s --format='%h %ci' eab020d 386fc5c` | vale `CLAUDE.md`: il pre-controllo è la fase dopo |

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
| **D6** | le **icone dei moduli** che le tavole non mostrano le sceglie il piano, una riga ciascuna nella mappa | le tavole danno Stato, Permessi, Passi, Attività, Chat, i moduli, la ricerca, le viste; la presa grande vuole l'icona di **ogni** tipo di modulo. Costo: una riga per cambiarne una |
| **D7** | la pagina kit mostra i due temi **uno alla volta**, con la scelta in cima, e non affiancati | un dialogo di `reka-ui` va in un portale sul `body` e prenderebbe il tema della radice, non quello della colonna. Costo: per confrontarli si cambia la scelta |
| **D8** | le parole della pagina kit sono **esemplari** scritti nel file, con un blocco del linter che lo dice, limitato a `src/kit/` | è una pagina di sviluppo fuori dal pacchetto: le sue parole in `it.json` finirebbero nel pacchetto per niente. Costo: una eccezione in più, in un posto solo |

## Le voci aperte che questo piano SA, e non chiude

Rilette il 2026-09-23 coi due comandi della §6 del compendio e con la tabella dell'audit: nessuna voce aperta dei Traguardi
5 e 6 ha per chiusore la GUI o questo lavoro, e nessuna è del tipo *«prima di questo traguardo»*.

| Voce | Di chi | Che cosa ne fa questo piano |
|---|---|---|
| **X-2** e **X-4** dell'[audit](../../audit-2026-08-27.md) | del proprietario | niente: non toccano la GUI |
| **N-2 di E187**, l'avviso di `vite` sui pezzi sopra i 500 kB | del proprietario | il compito 1 e il 3 **misurano** il pezzo JavaScript dopo il *build* e lo scrivono nel commit: la (e) dice che il design system non lo peggiora, ed è una deduzione |
| **E228**, progress e notifiche | del proprietario | niente: i token ci sono già, `--z-toast` e i colori di stato |
| il **terzo carattere** per il codice | del proprietario | niente: il monospazio resta quello del sistema (decisione 16 del disegno) |
| **AUD-004** | del proprietario | niente: sbarra il sotto-progetto 13, non questo |
| la **finestra** del guscio — `titleBarOverlay`, `setTitleBarOverlay`, gli angoli di Windows | del sotto-progetto **10** | niente: le regole della (d) restano scritte nel disegno per chi farà il guscio |

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
- Modify: `gui/src/main.ts`, `gui/src/App.vue`, `gui/src/stores/layout.ts`, `gui/src/stores/stores.test.ts`,
  `gui/src/a11y.test.ts` (un commento), e gli **undici** componenti che il censimento del passo 1 nomina
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
cd gui
npm install --save-exact @fontsource-variable/geist@5.3.0 @fontsource/barlow@5.3.0
git diff --stat package.json package-lock.json
npm ls @fontsource-variable/geist @fontsource/barlow
```

Atteso: le due righe nuove in `dependencies` di `package.json`, esatte, e il lockfile che le porta. La licenza di tutte e
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

In `gui/src/stores/stores.test.ts`, in coda al file, un `describe` nuovo:

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
cd gui && npx vitest run src/tokens src/stores
```

Atteso: **rosso**, e per la ragione giusta — `board.test.ts` e `contrast.test.ts` senza `base.css` e `themes.css`
(`ENOENT`), `usage.test.ts` senza `tokens/dock.css`, `theme.test.ts` senza `./theme`, e le tre prove del negozio senza
`theme` e `chooseTheme`. Un rosso per un'altra ragione è una voce d'errata.

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
cd gui && npx vitest run src/tokens/board.test.ts src/tokens/contrast.test.ts
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
 * so the caller runs this BEFORE the first paint (`main.ts` does it before `mount`).
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

- [ ] **Passo 12: `main.ts` — i token, e il tema prima della prima pittura**

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

// ⛔ BEFORE THE MOUNT, so the first paint already has its colours: every role of `themes.css` lives under
// `[data-theme]` (design system, section (a)). Until the core's package arrives the choice is `system`.
watchTheme(() => layout.theme);

app.mount("#app");
```

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

Atteso: undici righe `ok:`; poi il censimento non trova più **nessun** nome vecchio né **nessun** colore a mano, e i due
`z-index` dicono `var(--z-overlay)`. ⚠️ Lo script scrive su file CRLF senza toccarne i terminatori, perché legge e scrive
con `newline=""` e le sue sostituzioni non contengono fine-riga: si rimisura lo stesso, al passo 16.

- [ ] **Passo 14: `tokens.css` esce, e ciò che lo nominava**

```bash
git rm gui/src/tokens/tokens.css
```

In `gui/src/a11y.test.ts`, *Trova* `` * `tokens/contrast.test.ts` is what proves AA, on every text colour over every surface. ``
— *Sostituisci con* `` * `tokens/contrast.test.ts` is what proves AA, on the families of the design system's pairs. ``

In `docs/COMPENDIO.md`, *Trova* la riga che comincia con `Lo stile di oggi è un **segnaposto dichiarato**`, **intera, presa
dal file** — *Sostituisci con*:

```markdown
Lo stile dal compito 1 del piano del design system: `gui/src/tokens/base.css` e `themes.css`, copie della tavola dei token.
```

⚠️ Il margine del compendio prima e dopo, col comando del vincolo 13.

- [ ] **Passo 15: tutte le prove, il *build*, il linter**

```bash
cd gui && npm test && npm run build && npm run lint
```

Atteso: **verde** su tutto; nel log del *build*, la riga del pezzo JavaScript — `npm run build 2>&1 | grep 'kB'` — che si
scrive nel commit accanto a quella della baseline (**N-2 di E187**: il design system non dovrebbe peggiorarlo, ed è una
deduzione). Il CSS cresce delle dichiarazioni `@font-face`; i caratteri sono **file a parte**, `dist/assets/*.woff2`.

- [ ] **Passo 16: le due direzioni, e i fine-riga**

Ogni prova si prova **rossa**, poi si torna indietro — `git diff --stat` a zero sui file toccati per la prova:

| La prova | La violazione messa a mano | Atteso |
|---|---|---|
| `board.test.ts` | in `themes.css` un gradino `--ref-bordeaux-28` cambiato di una cifra | rosso: `themes.css is the board's block` |
| `contrast.test.ts` | in `themes.css` `--color-text-muted: var(--ref-neutral-44);` nello scuro | rosso, e il messaggio nomina `color-text-muted on …` |
| `contrast.test.ts`, la guardia | in `themes.css` un ruolo nuovo `--color-bg-test: var(--ref-neutral-5);` nei due temi | rosso: `color-bg-test` non ha giudice — ⚠️ attenzione, `color-bg-test` finisce fra i fondi solo se la regola lo prende: che cosa dice il rosso si **legge** |
| `usage.test.ts`, le scale | in `panels/Strip.vue` `color: var(--ref-neutral-5);` | rosso: `panels/Strip.vue` |
| `usage.test.ts`, i colori | in `panels/Strip.vue` `color: #fff;` | rosso: `panels/Strip.vue:<riga>` |
| `theme.test.ts` | in `theme.ts` `resolveTheme` che ignora `choice` | rosso |

Poi i fine-riga dei file toccati — quelli **nuovi** LF, gli altri come erano:

```bash
git ls-files --eol gui/src gui/package.json gui/package-lock.json docs/COMPENDIO.md | grep -v 'w/lf\|w/crlf'
for f in $(git diff --name-only; git ls-files --others --exclude-standard gui/src); do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
```

- [ ] **Passo 17: guardarlo, nei due temi**

`cd gui && npm run dev`, la pagina nel browser; nella console `harnessFake.deliverAll()`, poi
`document.documentElement.dataset.theme = "light"` e `"dark"`. Atteso: i colori della tavola, i caratteri Geist e Barlow
(DevTools, *Computed*, `font-family`), nessuna scritta illeggibile. ⚠️ Il dock è ancora `themeAbyss` fino al compito 6: nel
tema chiaro stona, ed è **atteso**.

- [ ] **Passo 18: il cancello, il commit, la posizione**

La riga **1** della tabella della posizione diventa `✅ <data>` col commit; poi, uno alla volta:

```bash
bash scripts/gate.sh
bash scripts/check-docs.sh
git add gui/package.json gui/package-lock.json gui/src docs/COMPENDIO.md docs/superpowers/plans/2026-09-23-design-system.md
git commit -m "design-system(compito 1): i token dalla tavola -- base.css e themes.css copiati da uno script e tenuti uguali da board.test.ts; il contrasto per famiglie (P-1), gli stessi ruoli nei due temi, nessuna scala e nessun colore a mano fuori dai token; il tema sulla radice e il campo theme del pacchetto; Geist e Barlow; i nomi nuovi negli undici componenti; tokens.css esce"
git push
```

---
## Come si riprende — punto fermo della scrittura, 2026-09-23

⚠️ **Il piano è A METÀ, e non si esegue.** Scritti: la testa e il **compito 1**. Da scrivere: i compiti **2–9** e la
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
