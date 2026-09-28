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
Il kit sono i **pezzi di base** in `gui/src/components/`, sopra `reka-ui` dove serve tastiera e ARIA, con le icone di
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

✅ **IL PIANO È SCRITTO il 2026-09-24**, e i due comandi coincidono: `grep -c '^## Compito' <questo file>` e le righe `| **N** |`
della tabella qui sotto. ⏳ **Viene il pre-controllo**, compito per compito, ciascuno in una sessione sua. ⛔ **Nessun compito si
esegue** prima del proprio pre-controllo.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | **i token**: `base.css` e `themes.css` copiati dalla tavola da uno script, e il test che li tiene uguali; `dock.css` con le regole che oggi stanno in `tokens.css`; `tokens/index.ts` coi due caratteri; il tema sulla radice (`tokens/theme.ts`) e il campo `theme` del pacchetto; i nomi nuovi negli undici componenti che li usano; `tokens.css` esce. Le prove: il contrasto per famiglie, gli stessi ruoli nei due temi, nessuna scala fuori dai token, nessun colore a mano, il tema nelle due direzioni | `95068bb`, con le cure `69d10fa` ed `e2cd7df` | ✅ 2026-09-24 |
| **2** | **il browser dei test**: `@vitest/browser-playwright` e `playwright`, i due progetti di `vitest` — jsdom e browser, sul Chrome installato e senza finestra — e il comando `emulateMedia`. Le prime prove vere: il browser è vero, i caratteri caricati con le cifre tabulari, il movimento ridotto nelle due direzioni, il contorno del focus sotto l'alto contrasto, il tema che segue il sistema | `23b3134`, con la cura `5f32158` | ✅ 2026-09-25 |
| **3** | **il kit**: `lucide`; `BaseIcon` con `icons.ts`, `BaseButton`, `BaseLabel`, `BaseList`, `BaseStatus`, `BaseTextField`, `BaseRadioGroup`, `BaseDialog`; un test con `axe` ciascuno; il linter che legge anche i `.ts`, e le due regole sui pezzi di base | `c7b7bcd`, con la cura `9d2ffb0` | ✅ 2026-09-25 |
| **4** | **la pagina kit**: `gui/kit.html` e `gui/src/kit/`, **fuori** dal pacchetto e provata sull'uscita del *build*; le prove nel browser sulla pagina kit, nei due temi — raggi concentrici, testo tagliato e niente che sborda, icone disegnate e centrate, `axe` col contrasto | `841dc54`, con la cura `8d098d0` | ✅ 2026-09-25 |
| **5** | **il kit al lavoro**: i pezzi di base nei pannelli e nella cornice — la finestra di conferma, il cassetto, la fascia, Stato, Permessi, Passi, Impostazioni con la scelta del tema, il segnaposto, la barra; le due regole del linter su `panels/` e `frame/`; **M-3** chiusa per costruzione, e l'Assistente vocale a mano | `545f500`, con le cure `7e25d03` e `9e6657b` | ✅ 2026-09-26 — il Passo 8 col proprietario, nel suo Chrome con l'Assistente vocale: alle domande dei punti 3, 4 e 5 — *«senti …?»* — la risposta *«di quello che hai chiesto … funziona tutto»*, e il punto 1 non riportato a parte; e *«a parte il piccolo problema citato poco fa»*, il lampo delle barre di scorrimento con la fascia, curato da **E43** |
| **6** | **il dock vestito**: il tema `dockview-theme-harness`, `dock.css` con ogni variabile del tema di riferimento, i gruppi come schede, `readToken`, `--z-floating`, la presa grande coi pezzi di base; `themeAbyss` esce | `c1102fc`, con la cura `ca83dac` | ✅ 2026-09-27 — il Passo 8 col proprietario, nel suo Chrome: il 2026-09-26 quattro foto — la zona d'arrivo ad angoli dritti, due sfondi, la barra con un fondo suo, il nucleo vestito da scheda —, curate (**E55**–**E58**); ripreso lo stesso giorno, tre voci **non approvate** — la barra di scorrimento, i messaggi con la fascia, il bordo nel raggio (**E59**–**E61**) —, disegnate e approvate il 2026-09-27 e curate dal compito **6bis**; lo sguardo del 6bis le ha approvate, e con lui si chiude questa riga |
| **6bis** | **la cura delle tre voci del Passo 8 del 6**: nella tavola il bordo nei raggi, la barra di scorrimento col suo innesco e il messaggio, e la copia; la sonda a mezzo pixel; le barre mostrate al progetto browser; `BaseNotice` con le icone dei toni, nella pagina kit, nella fascia sulla pagina, in Impostazioni e in Stato; `frame.browser.test.ts` | `da7a522`, con la cura `5105ad0` | ✅ 2026-09-27 — rivisto **conforme**: 0 critici, 0 importanti, 2 minori e 2 nit, tutti del dettato e curati (**E70**–**E73**). Il Passo 16 col proprietario, nel suo Chrome col mouse vero — la fascia prima e dopo la consegna, i messaggi, la barra col cursore che compare sotto il puntatore, i raggi —: *«quello che mi hai chiesto funziona»*, A, a condizione che la chiusura segua `anthropic-skills:decision-principles`; il frammento del contrasto sulla SPA a 1440 × 900, misurato dal coordinatore, 5,39 nel chiaro e 5,4 nello scuro. Due difetti di prima, trovati dalla revisione e detti prima dello sguardo, che non li ha commentati: il bordo di «Riprova» sulla fascia, 1,22 e 1,00, è il bordo di decoro che la (a) esenta (**E5**), e resta; l'anello del fuoco tagliato è **E74**, aperta |
| **7** | **le viste col nome, sotto**: `named` e `openNamed` nel pacchetto, il negozio e il dock che le mostrano e le salvano; l'aiutante della geometria estratto da `moveActive.ts`; lo schema di una disposizione, per le miniature | `0417c9c`, con la cura `a606f33` | ✅ 2026-09-27 — rivisto **conforme**: 0 critici, 0 importanti, 4 minori e 5 nit, tutti del dettato e curati (**E82**–**E89**), e **E90** al pre-controllo del compito 8. Nessuno sguardo del proprietario: la regola 5 non nomina il compito 7. La colonna Commit la scrive il compito 8 (**E79**): `0417c9c`, con la cura `a606f33` |
| **8** | **la cornice**: la barra col nome della vista, la **Panoramica** su `BaseDialog` con le miniature e *«Salva questa vista»* — F3, frecce, Invio, Esc —, la **striscia** a pillola coi «moduli»; le prove nel browser della Panoramica e della striscia | `ac56b11`, con le cure `77c4062` e `7f04cdf` | ✅ 2026-09-27 — rivisto **conforme** al dettato, con **2 importanti** di prima del compito che la Panoramica rende visibili — la miniatura di Compatta falsa (**E103**), l'ingrandimento non salvato (**E104**) —, 4 minori e 2 nit, e la candidata **E102** del rapporto confermata; curati da un **curatore**, un subagente col sì del proprietario sulla banda, sulle decisioni del coordinatore (**E102**…**E110**), e **X4** di **E107** dal coordinatore, perché la dichiarazione decisa per lei la misura l'ha smentita. Il Passo 10 col proprietario, nel suo Chrome, sulla lista del passo e sulle cure — Compatta, l'ingrandimento, *«Salva questa vista»* spenta e accesa, il segnaposto del nome, l'anello dentro —: *«quello che mi hai chiesto funziona»*. Detti prima dello sguardo e non commentati, e restano come sono: la carta *«Salva questa vista»* contro la tavola (**E110**), i due anelli bordeaux della carta corrente col fuoco, la striscia che sparisce sotto un gruppo ingrandito — in `dockview` una vista ingrandita è un gruppo solo |
| **9** | **la chiusura**: la riga «Accessibilità» di tracciabilità, la riga 14 della roadmap con *«Perché quest'ordine»*, `README.md`, la §12 e la §6 del compendio, `porta-di-qualita.md` con C-S0-1 (P-29), `riferimenti.md`; i richiami nel disegno e nella stella polare (P-28); la **Definizione di «fatto»**, eseguita, con le uscite del giorno | — | ⬜ |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del [`COMPENDIO.md`](../../COMPENDIO.md). Qui resta la
**posizione** — la tabella, che chi esegue aggiorna nel commit del compito — e **come** si esegue.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro — e le sezioni del disegno che nomina.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-23.
3. Se il compito dice il falso, **ci si ferma e si riporta**: una divergenza è una voce d'errata prima di essere un rimedio.
   ⚠️ Vale per il codice dettato: un `npm run build` o un `npm test` rosso sul testo dettato è una voce d'errata col testo
   corretto, **non** un aggiustamento silenzioso.
4. Il cancello gira **prima** di ogni commit, da solo; il commit dice ciò che il compito ha fatto.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; per i compiti **4, 5, 6, 6bis e 8** apre
   la SPA — o la pagina kit — nel browser, nei due temi, e **guarda**: un verde non prova che una cosa si veda.
6. Una seconda ondata di **sola prosa** la chiude il coordinatore a mano; dopo due ondate di prosa si chiude (gotcha #76).
7. ⛔ **Le scritture in parallelo non si fanno**: un compito per volta.
8. ⛔ **Il dispaccio viaggia con git**, perché un compito si riprende anche dall'altra macchina — richiesta del
   proprietario, 2026-09-24. Prompt, script, rapporti e revisioni stanno in
   `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`, tracciata, e il prompt di un compito vi nasce come
   **modello**, coi campi della macchina da riempire. Nella cartella di lavoro `.superpowers/sdd/2026-09-23-design-system/`,
   ignorata, nascono i brief — copie del piano, che non si committano — e scrivono implementatore e revisore; alla
   chiusura del compito il coordinatore copia nella cartella tracciata il prompt spedito, il rapporto, il prompt del
   revisore e la revisione, e li committa.

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota.** La riempiono il pre-controllo e l'esecuzione: una voce per difetto trovato, col testo corretto, la data e
chi l'ha trovata. Ciò che la **scrittura** del piano ha trovato sta nella sezione dopo, perché è già dentro i compiti.

| # | Voce |
|---|---|
| **E1** | ⚠️ **Compito 1, Passo 10 — *«CRLF su questa macchina»*, detto di `gui/src/stores/layout.ts`, è la colonna `w/…` della macchina `zagor`, dove il piano è stato scritto e pre-controllato: sulla macchina dell'account `Jays` è falso.** Misurato il 2026-09-24 a `6ae9b8b`, preparando il dispaccio del compito 1: `git config --show-origin core.autocrlf` → `false` in `.git/config` (E51 del piano della parte 2); `git ls-files --eol gui/src/stores/layout.ts` → `i/lf w/lf`. Il pre-controllo non poteva vederlo, perché su `zagor` è vero. È la malattia di **E51** per la **settima** volta: E224 del piano della parte 2 si conta sesta. ✅ **Nessun Atteso cambia:** `replace_unique.py` conserva il fine-riga che trova, e il Passo 16 lo misura come invariante. ✅ **Corretto con la regola di E72** del piano della parte 2, decisione del proprietario del 2026-09-19: l'etichetta si **toglie**, non si rovescia. Trovato dal coordinatore dell'esecuzione del compito 1 |
| **E2** | ⛔ **Compito 1, Passo 7 — la terza prova del negozio non vede un `chooseTheme` che butti il pacchetto:** chiama la scelta su un negozio **vuoto**, dove `saved` è `null`, e lo *spread* non si vede. Trovata dalla revisione del compito 1 (I-2) e rifatta dal coordinatore il 2026-09-24, con uno script suo, nel clone a `95068bb`: tolto `...(saved.value ?? {})` da `chooseTheme`, `npx vitest run src/tokens src/stores` → `Tests  38 passed (38)`. Dal compito 5 la scelta si fa in Impostazioni **dopo** l'arrivo del pacchetto, e un `chooseTheme` così manderebbe al core `layouts: {}`: ogni disposizione salvata persa in silenzio. ✅ **Curata** nel commit che scrive questa riga: prima della scelta il negozio riceve un pacchetto con `layouts: { work }`, e le due attese diventano `layouts: { work }` e `layouts: { work, home }`; verde sul codice, rossa con la mutazione su *«sends the choice at once, and a settle after it keeps it»*, e rossa ancora con la violazione 8 del Passo 16. ⚠️ **La seconda *Trova* del compito 7, la fine di `stores.test.ts`, è allineata nello stesso commit**: cercava `layouts: { home }` |
| **E3** | ⚠️ **Compito 1, Passo 6 — lo *stop* di `watchTheme` è provato a metà:** la prima prova conta gli ascoltatori del sistema, e nessuna guarda il `watch` sulla scelta, mentre il commento della prova promette l'intero. Trovata dalla revisione (M-1) e rifatta dal coordinatore: tolto `stop();` dalla funzione resa, `Tests  38 passed (38)`. ✅ **Curata** nel commit che scrive questa riga: in coda alla seconda prova, dopo `stop();`, la scelta passa a `light` e la radice resta `dark`; rossa con la mutazione, `expected 'light' to be 'dark'` |
| **E4** | ⚠️ **Compito 1, Passo 11 — nel tema chiaro il dock disegna scuri i controlli nativi:** `dockview.css` 8.3.1 dà `color-scheme: dark` a `.dockview-theme-abyss`, lo stesso selettore del ponte, che non lo rimette: le barre di scorrimento e i radio di Impostazioni si disegnano scuri, e il radio **non** scelto sembra quello scelto. La causa letta dal coordinatore in `gui/node_modules/dockview/dist/styles/dockview.css`; l'effetto visto dalla revisione (M-2) nel browser. Lo *snippet* del Passo 17 misura il testo, e non lo vede. ✅ **Curata** nel commit che scrive questa riga: `color-scheme: inherit;` in testa al ponte di `dock.css`, col perché nel commento; misurata dal coordinatore nel browser, `getComputedStyle(document.querySelector(".dock input[type=radio]")).colorScheme` uguale al tema nei due temi. Transitoria: il radio esce col compito 5, `themeAbyss` col 6 |
| **E5** | ✅ **Del proprietario — il commento di `themes.css`, copiato dalla tavola, dice il falso sui ruoli non-testo:** *«every non-text role 3:1»*, mentre i bordi di decoro stanno fra 1,13 e 1,88 e la prova li esenta per nome (`EXEMPT`). Trovata dalla revisione del compito 1 (M-3). La copia non si ritocca (vincolo 2, D1): la frase si corregge **nella tavola**, poi si ricopia con lo script del Passo 9, e `board.test.ts` le tiene uguali. La tavola è approvata, quindi decide il proprietario, come per la riscrittura di P-1. Il testo proposto dalla revisione: *«Every text role reads 4.5:1 on the backgrounds of its FAMILY, and the non-text roles that tell a control apart -- strong border, focus, mark, accent border -- 3:1 on the base backgrounds; the decoration borders and the veil are exempt, each with its reason in the contrast test.»* ✅ **Decisa dal proprietario il 2026-09-24 — A**, e curata nel commit che lo scrive: la frase riscritta **nella tavola** con quel senso, e `themes.css` ricopiato con lo script del Passo 9; `base.css` e i valori non cambiano |
| **E6** | ✅ **Del proprietario — la prova dei colori a mano non vede un colore per nome:** `color: white;` in `panels/Strip.vue` lascia verde *«writes no colour by hand outside the token files»* (revisione del compito 1, M-4); per `oklch(`, `lab(` e `color-mix(` lo stesso, **dedotto** dall'espressione regolare. La prova fa alla lettera il controllo 5 del disegno — *«nessun `#…`, `rgb(`, `hsl(`»* —, e allargarla tocca il merito approvato (vincolo 1). Oggi in `gui/src` non c'è nessun colore per nome. Le due strade: tenere il controllo e dichiararne il limite nella prova, o allargarlo ai nomi di CSS e alle funzioni di CSS Color 4. ✅ **Decisa dal proprietario il 2026-09-24 — A**, e curata nel commit che lo scrive: la prova vede anche `hwb(`, `lab(`, `lch(`, `oklab(`, `oklch(` e `color-mix(`, e il suo commento dichiara che un colore **per nome** non lo vede — misurato: `color: white;` in `panels/Strip.vue` la lascia verde, `oklch(` e `color-mix(` la fanno rossa |
| **E7** | Nit — **Compito 1, Passo 4:** `contrast.test.ts` citava la luminanza di WCAG 2.2 con la soglia di prima del maggio 2021, `0.03928`; WCAG 2.2 usa `0.04045` — tecnica G18, letta dal coordinatore il 2026-09-24, che dice anche che la differenza non ha effetti pratici —, e nessun canale a 8 bit cade fra le due (10/255 = 0,039216; 11/255 = 0,043137). Trovata dalla revisione (N-1). ✅ **Curata** nel commit che scrive questa riga: `0.04045`, come lo *snippet* del Passo 17, e la fonte nella tabella del Passo 8 del compito 9 |
| **E8** | Nit — **Compito 1, Passo 10:** il commento di `shownTheme` in `theme.ts` diceva *«the dock's `colorScheme` today (task 6)»*, e oggi nessuno lo legge. Trovata dalla revisione (N-2). ✅ **Curata**: *«from task 6»* |
| **E9** | Nit — **Compito 1, Passo 13:** in `panels/Chat.vue` il bordo della provenienza usa un ruolo di **testo**, `--color-text-warn`, e il perché — P-13 — viveva solo nel piano: chi rilegge il file lo «correggerebbe» in `--color-border-warn`, che è decoro. Trovata dalla revisione (N-3). ✅ **Curata**: un commento accanto alla regola, che lo script dei nomi non poteva scrivere |
| **E10** | ⛔ **Del proprietario — Compito 2, Passi 3, 5 e 6: due progetti in una corsa sola nascondono un progetto VUOTO.** Col `jsdom` e il `browser` nello stesso `npm test`, un progetto che non trova nessun file è **verde**: `vitest` 4.1.11 dice `No test files found` solo quando è vuota la corsa **intera** — `passWithNoTests` sta fra le `NonProjectOptions`, un'opzione solo globale, letto nel pacchetto installato. Misurato il 2026-09-24 dal pre-controllo sulla copia `%TEMP%\pc2`, col compito rifatto dal testo del piano: con l'`include` del progetto `browser` che non trova niente, `npm test` esce **0** con 17 file e 100 prove, e Chrome non parte — proprio il caso che il controllo 20 del disegno vuole rosso, *«il passo nel cancello rosso se il browser non parte, non verde»*; con quello del `jsdom` vuoto esce **0** con un file solo, e le cento prove sotto jsdom spariscono. ⚠️ **E oggi la guardia c'è:** con un progetto solo, un `include` vuoto dà `No test files found, exiting with code 1`, misurato sul `vite.config.ts` di `b66aee8` — il compito, com'era scritto, la **toglieva**. Lanciati uno alla volta, `npm test -- --project jsdom` e `npm test -- --project browser` escono **1** ciascuno sul proprio vuoto, e **0** sul codice del compito. ✅ **Decisa dal proprietario il 2026-09-24 — A**, contro B, tenere `npm test` e dichiarare il limite: il cancello lancia i due progetti **uno alla volta**, e il Passo 6 scrive il richiamo datato sulla riga di `scripts/gate-gui.sh` della §8 del disegno del sotto-progetto 2, la cui forma approvata è *«`npm ci`, `npm run build`, `npm test`»* — com'è stato per il lint (R8-10); il commento del Passo 3 lo dice, e il Passo 5 prende la violazione. Il costo, dentro `gui/`, `time npm test` contro `time (npm test -- --project jsdom && npm test -- --project browser)`, tre corse per parte sulla copia, macchina `Jays`, il 2026-09-24: da 3,9–4,4 s a 5,6–5,9 s. `npm test` in locale resta una corsa sola, col limite scritto accanto; la riga `gui: probes` che il compito 9 scrive in `porta-di-qualita.md` è corretta sul posto, e la fonte sta nella tabella del suo Passo 8. Scartato lo script `test` di `package.json` a due corse: terrebbe alla lettera la forma della §8, ma sdoppierebbe il riepilogo che gli Atteso dei compiti 6, 7 e 8 leggono. Trovata dal pre-controllo del compito 2 |
| **E11** | ⚠️ **Compito 2, Passo 5 — manca la violazione che prova che la sonda del contorno del focus guarda la NOSTRA regola.** Le righe del Passo 5 provano che la sonda parla dell'alto contrasto — la guardia — e il corpo della prova che un `box-shadow` sparisce sotto i colori forzati; nessuna toglie `:focus-visible` da `base.css`, e nessuna diceva se il contorno di base di Chrome passasse la sonda. Misurato il 2026-09-24 dal pre-controllo, sulla copia: senza la regola `npm test` esce 1, rossa `expect(ring(button)).toBe(true)` con `expected false to be true` — e con lei `board.test.ts`, che vuole il foglio della tavola —: la sonda morde sulla regola del progetto, non sul browser. ✅ **Corretta** nel commit che la scrive: una riga nel Passo 5, e `base.css` torna dalla copia salvata |
| **E12** | Nit — **Compito 2, Passo 5, la riga del tema che segue il sistema:** tolta `system.addEventListener("change", apply);`, va rossa **anche** la prova del compito 1 sotto jsdom, *«follows the system while the choice is `system`, and stops following when stopped»* di `theme.test.ts`, `expected 'dark' to be 'light'` — la stessa riga, tenuta dal `matchMedia` finto e da quello vero. L'Atteso nominava la sola prova del browser, e chi esegue si sarebbe fermato su una divergenza che non c'è. Misurato il 2026-09-24 dal pre-controllo, sulla copia. ✅ **Corretta** nel commit che la scrive: l'Atteso nomina le due |
| **E13** | Nit — **Compito 2, Passo 5:** *«Il file usa-e-getta della penultima riga»* — il file sta nell'**ultima** riga della tabella, e le righe di **E10** e **E11** la spostano ancora. Trovata dal pre-controllo il 2026-09-24. ✅ **Corretta** nel commit che la scrive: la riga si **nomina**, *«l'attesa di 5 s»*, invece di contarla |
| **E14** | ⚠️ **Compito 2, Passo 2 — la seconda direzione della sonda del movimento è verde su un valore vuoto:** guardava **un** nome solo, `--duration-fast`, e chiedeva che non fosse la stringa `0ms`, mentre il commento prometteva *«the durations are the board's»*: una stringa vuota passa — la trappola 1 del disegno, una negazione verde quando non trova niente. Una durata tolta **solo** dal blocco di base di `base.css`, e rimasta in quello del movimento ridotto, la sonda del browser non la vedeva: la prendeva `board.test.ts`, che vuole il foglio della tavola. Trovata dalla revisione del compito 2 (M-2), che l'ha misurata nel clone a `23b3134`: tolta la riga `--duration-fast: 110ms;` dal blocco di base, la prova del movimento resta verde e cade la sola *«base.css is the board's block, byte for byte»*. ✅ **Curata** nel commit che scrive questa riga, col testo proposto dalla revisione: la seconda direzione chiede a **ciascuna** delle tre durate un valore, `toMatch(/^[1-9]\d*ms$/)`, senza ricopiare un valore della tavola (vincolo 2), e il commento lo dice. Misurata dal coordinatore il 2026-09-25 sull'albero, macchina `zagor`: `npm test -- --project browser` → `Tests  5 passed (5)`; con la stessa mutazione → `AssertionError: --duration-fast: expected '' to match /^[1-9]\d*ms$/`, `Tests  1 failed \| 4 passed (5)`; `base.css` tornato dalla copia salvata, `cmp` uguale. Il recinto del Passo 2 è allineato nello stesso commit |
| **E15** | ⚠️ **Compito 3, Passo 3 — la seconda direzione di *«gli attributi di chi lo usa vanno sull'`input`»* non ha una prova:** la prova di `BaseTextField` guarda che il `placeholder` arrivi all'`input`, e nessuna che **non** arrivi anche alla radice intorno. Tolta `defineOptions({ inheritAttrs: false });` da `BaseTextField.vue`, gli attributi vanno sull'`input` **e** sulla radice — un `@keydown` di chi lo usa girerebbe due volte, sull'`input` e sulla sua risalita —, e `npx vitest run --project jsdom src/components/kit.test.ts` → `Tests  23 passed (23)`. Misurato il 2026-09-25 dal pre-controllo sulla copia `%TEMP%\pc3`, macchina `zagor`, col compito rifatto dal testo del piano. ✅ **Corretta** nel commit che la scrive: nella prova di `BaseTextField`, dopo il `placeholder` dell'`input`, `expect(wrapper.element.hasAttribute("placeholder")).toBe(false);` col perché nel commento — verde sul codice del compito, rossa con la riga tolta, `expected true to be false`, la sola prova del campo —; e una riga nella seconda tabella del Passo 8 |
| **E16** | ⚠️ **Compito 3, Passo 3 — la cura di R2-18 in `BaseDialog.vue`, nessun `aria-describedby` senza una descrizione, non ha una prova:** tolta la riga del `v-bind` che lo spegne, `reka-ui` 2.10.4 scrive `aria-describedby="reka-dialog-description-v-1"`, che non punta a nessun elemento, e avvisa *«Missing `Description` or `aria-describedby="undefined"` for DialogContent.»* — e le 23 prove restano verdi. Misurato il 2026-09-25 dal pre-controllo sulla copia, con una prova usa-e-getta che falliva dicendo i valori: con la riga, nessun attributo e nessun avviso. ✅ **Corretta** nel commit che la scrive: nella prova di `BaseDialog`, che la apre senza descrizione, `expect(dialog?.hasAttribute("aria-describedby")).toBe(false);` — verde sul codice, rossa con la riga tolta, `expected true to be false`, la sola prova della finestra —; e una riga nella seconda tabella del Passo 8 |
| **E17** | Nit — **Compito 3, Passo 7 — un commento di `gui/eslint.config.js` che il compito rende falso** (gotcha **#58**): accanto a `vue/multi-word-component-names`, *«Every `.vue` file here but `ViewBar` is single-word (P-99; recounted at the review, R7-11).»* — gli otto `Base*.vue` del compito hanno due parole ciascuno, e nessun compito del piano toccava la frase: `grep -c 'single-word'` sul piano rendeva **0** prima di questa voce. È la specie della quarta sostituzione dello stesso Passo, R2-10. Trovata dal pre-controllo il 2026-09-25. ✅ **Corretta** nel commit che la scrive: una **quinta** sostituzione nel Passo 7, che data il censimento e nomina i pezzi di base; `npm run lint` resta verde, misurato sulla copia |
| **E18** | Nit — **Compito 3, Passo 8 — il blocco `harness/imports/the-icon-map` non aveva la sua riga rossa:** toglie `lucide` dal divieto per `icons.ts` e ci tiene la regola dei pezzi di base, e la prima tabella provava la prima metà — `icons.ts` importa `lucide`, verde — e non la seconda. Misurato il 2026-09-25 dal pre-controllo sulla copia: con `import { useCore } from "../stores/core";` in `icons.ts`, `npm run lint` → rosso, *«a base piece reads no global state»*. ✅ **Corretta** nel commit che la scrive: una riga nella prima tabella del Passo 8 |
| **E19** | ⚠️ **Compito 3, Passo 5 — un `BaseButton` `quiet` spento ha il colore di uno acceso:** `.base-button[data-variant="quiet"]` pesa quanto `.base-button:disabled` e viene dopo, quindi il suo `--color-text-muted` vince sullo spento; la tavola scrive la regola a parte, `.btn.quiet.is-disabled`. Trovata dalla revisione del compito 3 (M-1), misurata nel Chrome installato; jsdom non applica il foglio, e nessuna prova la vedeva. ✅ **Curata** nel commit che scrive questa riga, col testo della revisione: `.base-button[data-variant="quiet"]:disabled { color: var(--color-text-disabled); }` dopo la regola `:hover`/`:active` del `quiet`, col perché nel commento; il recinto del Passo 5 è allineato. Misurata dal coordinatore il 2026-09-25, macchina `zagor`, Chrome 154, con una prova usa-e-getta del progetto `browser` nei due temi: col codice curato il `quiet` spento prende `--color-text-disabled`, verde; senza la cura, rossa, `dark: expected 'rgb(163, 154, 143)' to be 'rgb(111, 102, 96)'`; la prova tolta dopo, `git status --porcelain` com'era. ⚠️ **La sonda che la tiene nel cancello non c'è ancora**: è del browser, e la chiede il pre-controllo del **compito 4**, che porta le prove nel browser sulla pagina kit nei due temi |
| **E20** | ⚠️ **Compito 3, Passo 6 — sotto il puntatore, un `BaseTextField` con l'errore perde il bordo rosso:** `.frame:hover:not([data-disabled])` pesa più di `.frame[data-error]`, e vince sempre; nella tavola `.field.is-error` viene dopo `.field.is-hover` con lo stesso peso, e vince l'errore. La Panoramica del compito 8 dà l'errore al campo del nome: il bordo sparirebbe proprio mentre ci si punta. Trovata dalla revisione del compito 3 (M-2), misurata nel Chrome installato. ✅ **Curata** nel commit che scrive questa riga: `.frame:hover:not([data-disabled], [data-error])`, col perché nel commento; il recinto del Passo 6 è allineato. Misurata dal coordinatore con la stessa prova usa-e-getta, tema scuro: col codice curato il bordo resta `--color-border-stop` prima e sotto il puntatore, verde; senza la cura, rossa, `expected 'rgb(163, 154, 143)' to be 'rgb(129, 27, 7)'`. ⚠️ La sonda nel cancello, come per **E19**, la chiede il pre-controllo del compito 4 |
| **E21** | ⚠️ **Compito 3, Passo 3 — quattro comportamenti del contratto dei pezzi senza una prova**, ciascuno misurato dalla revisione del compito 3 (M-3, M-4, M-5) con `Tests  23 passed (23)` sotto la sua mutazione: **(a)** la seconda direzione di **E16** — con una descrizione, la finestra è descritta da lei —; **(b)** il `null` di `BaseRadioGroup` anche dopo un clic, il caso di P-8 prima che il core risponda; **(c)** lo slot `trigger` che apre la finestra non legata; **(d)** i segni `data-*` scritti con `\|\| undefined`, senza i quali Vue scrive la stringa `"false"` e i selettori `[data-icon-only]`, `[data-pill]`, `[data-disabled]` la prendono — e un **quarto**, `[data-error]` di `BaseTextField`, trovato dal coordinatore applicando la cura: la stessa frase nello stesso file. ✅ **Curate** nel commit che scrive questa riga, nel file e nel recinto del Passo 3: due prove nuove nel blocco di `BaseDialog` — *«is described by its description, when it has one»* e *«opens from its `trigger` slot when unbound»* —, la prova del `null` con un clic, e quattro righe sui segni; `kit.test.ts` passa da 23 a **25** prove. Misurate dal coordinatore il 2026-09-25 sull'albero, una mutazione per volta con la copia salvata e `cmp` uguale dopo: verdi sul codice, `Tests  25 passed (25)`; rosse ciascuna da sola, `Tests  1 failed \| 24 passed (25)`, coi messaggi delle quattro righe nuove della seconda tabella del Passo 8 |
| **E22** | Nit — **Compito 3, Passo 7 — due frasi di `gui/eslint.config.js` che il compito smentisce o anticipa** (gotcha **#58**): **(a)** la riga di testa del blocco `harness/ts-in-vue`, *«WITHOUT IT SIX OF THE THIRTEEN DO NOT PARSE»*, conta i `.vue` di una cartella che il compito fa crescere — misurato dalla revisione del compito 3 (N-1): senza il `parser`, tredici `.vue` su ventuno non si analizzano —, la specie di **E17** nello stesso file; **(b)** il blocco `harness/ts` nomina `reka-ui` fra ciò che le regole devono vedere, e nessuna regola ne parla prima del blocco `harness/panels-and-frame` del compito 5 (N-2). ✅ **Corrette** nel commit che scrive questa riga: per **(a)** una **sesta** sostituzione nel Passo 7, col testo della revisione — la misura datata delle righe sotto resta vera com'è —; per **(b)** *«… or `lucide` -- and, from task 5, `reka-ui` -- could come in …»* nel recinto della seconda sostituzione. `npm run lint` verde, misurato dal coordinatore |
| **E23** | Nit — **Compito 3, Passo 8 — il confine della regola di `lucide`:** `paths: [LUCIDE]` prende il nome esatto, e un import da un sotto-percorso passa il linter. Trovata dalla revisione del compito 3 (N-3) e rimisurata dal coordinatore il 2026-09-25: con `import search from "lucide/dist/esm/icons/search.mjs";` in `frame/moveActive.ts`, `npm run lint` esce 0, e `npx vue-tsc --noEmit` esce 2, `TS7016: Could not find a declaration file for module 'lucide/dist/esm/icons/search.mjs'` — il pacchetto porta un `.d.ts` solo —; il file tornato dalla copia, `cmp` uguale. ✅ **Dichiarata**, com'è dichiarato l'`import()` dinamico (R2-17): una frase nel ⚠️ del Passo 8. Prenderla nella regola — un `patterns` accanto a `paths` nei tre blocchi — è la strada B della revisione, **registrata, non presa** |
| **E24** | ⛔ **Compito 4, Passo 5 — la prima corsa del progetto `browser` dopo il Passo 2 cade quattordici su quattordici, e non per la pagina: la cache delle dipendenze è stantia.** La corsa rossa del Passo 2 — `Kit.vue` non c'è ancora — lascia in `gui/node_modules/.vite/` un pacchetto senza `reka-ui` né `lucide`; al Passo 5 Vite li trova a corsa avviata, rifà il pacchetto e ricarica — *«dependencies optimized: axe-core, lucide, reka-ui, vue»*, *«optimized dependencies changed. reloading»* —, e la pagina si monta con **due copie di Vue**: ogni sua prova cade con `TypeError: 'set' on proxy: trap returned falsish for property 'style'`, dentro `reka-ui`. Vitest lo prevede e lo stampa, *«Vite unexpectedly reloaded a test. This may cause tests to fail…»*, in `@vitest/browser` 4.1.11 installato. Misurato il 2026-09-25 dal pre-controllo sulla copia `%TEMP%\pc4`, macchina `Jays`, col compito rifatto dal testo del piano: la cache rifatta da una corsa senza `Kit.vue`, poi la corsa intera, **2 su 2** `Tests  14 failed \| 5 passed (19)`; la corsa dopo, verde. ⚠️ **Il cancello non lo vede mai**: `npm ci` svuota `node_modules/`, e a cache vuota la corsa è verde, 3 su 3 — il rosso è di chi esegue e di chi rivede, e torna a ogni prova del browser che raggiunge per prima una dipendenza, nei compiti 6 e 8. ✅ **Decisa dal coordinatore il 2026-09-25 con `anthropic-skills:decision-principles`, su delega del proprietario — A**: `optimizeDeps: { force: true }` sul solo progetto `browser`, che rifà il pacchetto a ogni corsa dalla scansione dei file di prova. Misurata: lo scenario rosso diventa verde, 2 su 2, e la corsa costa uguale — 3,8–3,9 s a freddo, 3,6–3,9 s con la cache. Scartata **B**, la lista `optimizeDeps.include` che l'avviso di Vitest consiglia, verde anch'essa 2 su 2: va allungata a ogni dipendenza che una prova del browser raggiunge per la prima volta, e un nome dimenticato riporta il rosso solo in locale, dove il cancello non guarda. Il costo di A, detto nel commento accanto: si scosta da quel consiglio, e i tipi di Vite 8.3.0 marcano l'opzione `@experimental`, mentre la pagina *Dep Optimization Options* la documenta senza riserve; se sparisse, la via è B. ✅ **Corretta** nel commit che la scrive: il Passo 2 scrive la riga in `gui/vite.config.ts` prima della prova, *Files* la nomina, l'Atteso del Passo 5 lo dice; le due fonti nella tabella del Passo 8 del compito 9 |
| **E25** | ⚠️ **Compito 4, Passo 2 — le prove nel browser di E19 ed E20, che la revisione del compito 3 lasciava a questo pre-controllo: jsdom non applica il foglio, e i due difetti si vedono solo nel Chrome.** Nella prova della pagina kit, per ciascun tema, *«draws every button that is off in the disabled colour, whatever its variant (E19)»* — i tre pulsanti spenti della pagina, uno per variante, contro `--color-text-disabled` calcolato dalla pagina e non ricopiato — e *«keeps the error's border under the pointer (E20)»* — il bordo `--color-border-stop` del campo con l'errore, prima e sotto il puntatore di `userEvent.hover`, con la guardia che il puntatore sia arrivato. Misurate il 2026-09-25 dal pre-controllo sulla copia, macchina `Jays`, Chrome 154: verdi sul codice, 23 prove nel progetto `browser`; rossa la prima senza la regola del `quiet` spento — nello scuro `quiet: rgb(163, 154, 143)` dove la pagina vuole `rgb(111, 102, 96)`, i valori della revisione —; rossa la seconda con `.frame:hover:not([data-disabled])` — nello scuro `expected 'rgb(163, 154, 143)' to be 'rgb(129, 27, 7)'` —; e rossa la sua guardia senza la riga del `hover`, `expected false to be true`; ogni file tornato dalla copia salvata, `git status --porcelain` com'era. ⚠️ **E la prova del puntatore ha chiesto una cura al file.** Rifatto il compito dal testo corretto sulla copia `%TEMP%\pc4b`, la violazione della riga 1 del Passo 7 faceva cadere **anche** la prova di E20, `TimeoutError`, e la violazione del contrasto la faceva cadere nello scuro: la prova della finestra, rossa, si ferma prima del suo `wrapper.unmount()`, e la finestra rimasta montata lascia `pointer-events: none` sul `body` — misurato con una diagnostica scritta per fallire, `body pointer-events: "none"` —, dove il puntatore vero non si posa più. Curata alla radice, nella forma delle prove del dock e della cornice (compiti 6 e 8): le pagine montate in una lista, smontate in `afterEach`, e nessuna prova si smonta da sé; rimisurato, la riga 1 fa cadere le sole due prove della finestra, e la riga del contrasto le sole due di `axe` nello scuro. ✅ **Corretta** nel commit che la scrive: le due prove, l'aiutante `colourOf` e lo smontaggio in `afterEach` nel recinto del Passo 2, `userEvent` fra ciò che il compito consuma, tre righe nella tabella del Passo 7. ⚠️ **E il compito 8 allineato nello stesso commit**: il suo primo *Trova* su `kit.browser.test.ts` porta l'import di `userEvent`, e i suoi Atteso contano le quattro prove in più — *«nove rosse su trentuno»*, *«le diciotto della pagina kit»*, *«trentuno su trentuno»*; dedotto, perché nessun compito dopo il 4 tocca i pezzi che le due prove guardano |
| **E26** | Nit — **Compito 4, Passo 4 — il blocco `harness/kit-page-specimens` non aveva righe rosse, né sul bisogno né sul confine** (lezione 3 della consegna del pre-controllo del compito 3, in archivio: un blocco del linter si prova sul suo confine); il Passo 5 prova il solo verde. Misurato il 2026-09-25 dal pre-controllo sulla copia, una riga per volta e indietro con la copia salvata: tolto il blocco, `npm run lint` è rosso con trentatré `raw text … is used` in `src/kit/Kit.vue` — è il blocco a lasciar passare le parole della pagina —; col blocco, una parola in `BaseStatus.vue` resta rossa, `raw text 'ciao' is used` — il blocco si ferma alla sua cartella —; e `import { Search } from "lucide";` in `Kit.vue` resta rosso, *«'lucide' import is restricted…»* — il blocco spegne la sua regola e nessun'altra. ✅ **Corretta** nel commit che la scrive: una seconda tabella nel Passo 7, con le tre righe |
| **E27** | Nit — **Compito 4, Passo 7 — il ramo *fuori dall'angolo* di `concentricRadii` non è mai valutato sulla pagina kit.** La regola che il commento della sonda promette — un elemento lontano dall'angolo non è più tondo del raggio di fuori meno la distanza minore — non ha una riga che la faccia cadere, e nemmeno un caso verde da giudicare: la pagina mette ogni pezzo *nell'*angolo. Misurato il 2026-09-25 dal pre-controllo sulla copia: con un `throw` su ogni coppia fuori dall'angolo, le 23 prove restano verdi. ✅ **Corretta** nel commit che la scrive: una riga nel Passo 7, le righe della lista più tonde **e** spostate dall'angolo — rossa nei due temi con `base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0`; col ramo reso cieco, `: true`, la stessa riga è verde, cioè la prende quel ramo; e con lo stesso spostamento e il raggio giusto è verde, dove il ramo giudica e assolve — col `throw`, rossa `off the corner: base-list-row bottom-left 17.0/13.0`. ⚠️ E la riga *Files* della sonda nomina anche il compito 6, che usa `concentricRadii` |
| **E28** | Nit — **Compito 4, Passo 4 — il blocco `harness/kit-page-specimens` stava sotto la testa delle regole d'import** (gotcha **#58**): il *Trova* del Passo 4, `  {` e `    name: "harness/imports",`, lo metteva dopo il commento *«⛔ THE IMPORT RULES OF THE KIT … One rule, a scope per block»*, che così introduceva un blocco che non è una regola d'import, e il suo *«like Chat's above»* rimandava sopra quella testa. La specie di **E17** ed **E22** nello stesso file; il comportamento non cambiava. Trovata dall'implementatore del compito 4 e confermata dalla revisione (N-4). ✅ **Curata** nel commit che scrive questa riga: il *Trova* è la fine del blocco di `Chat.vue`, `    rules: { "vue/no-v-html": "off" },` e `  },`, unica nel file, e il blocco sta fra le due eccezioni, sopra la testa. Misurata dal coordinatore il 2026-09-26, macchina `Jays`: `npm run lint` verde; le tre righe della seconda tabella del Passo 7 rosse come prima — trentatré errori, il primo `raw text 'Il kit' is used`, poi `raw text 'ciao' is used`, poi *«'lucide' import is restricted from being used»* |
| **E29** | ⚠️ **Compito 4, Passo 2 — la prova dei raggi non teneva l'unico caso che giudica `BaseList`:** la riga `align-items: start;` di `.kit-grid`, col suo perché nel commento, non la teneva nessuna prova, e senza di lei la riga 2 del Passo 7 diventava **verde** — la griglia stira la scheda della lista, la sua ultima riga esce dall'angolo, e la guardia `near > 0`, che conta i casi di tutta la pagina, non vede sparire i due che giudicano il raggio di `BaseList` (`near` da 12 a 10). La lezione 3 del pre-controllo del compito 4, su un **caso** invece che su un ramo. Trovata dalla revisione del compito 4 (M-1), misurata nel suo clone. ✅ **Curata** nel commit che scrive questa riga, col testo della revisione: nella prova *«keeps every radius concentric (answer 4)»* un'attesa sulla sola scheda della lista, `concentricRadii([list]).near` maggiore di zero; e una riga nel Passo 7. Misurata dal coordinatore il 2026-09-26: verde sul codice curato; tolta `align-items: start;`, rosse le sole due prove dei raggi, `expected 0 to be greater than 0` |
| **E30** | ⚠️ **Compito 4, Passo 1 — `concentricRadii` leggeva il solo angolo in alto a sinistra**, fedele a `sonda-raggi.js`, mentre la (f), la trappola 4 e il suo stesso commento promettono che *«un angolo dritto non lo confronta»* (risposta 20): un pezzo con l'angolo in alto a sinistra dritto e un altro tondo non era **mai** giudicato, e gli angoli dritti di una forma `r r 0 0` finivano in `bad`. Sulla pagina kit non succedeva, perché ogni pezzo ha i quattro angoli uguali; ma le forme ci sono già — `BaseDialog.vue` nella forma `sheet`, `frame/Drawer.vue`, e nel compito 6 `.dv-floating-titlebar` e `.dv-groupview` —, e la sonda la importano i compiti 6 e 8. Trovata dalla revisione del compito 4 (M-2), con due strade: **A**, un raggio per angolo; **B**, dichiarare il limite nel commento. ✅ **Presa la A**, decisa dal coordinatore coi cinque criteri: la B scriveva nel codice il contrario della (f), che è merito approvato, e lasciava ai compiti 6 e 8 un verde e un rosso falsi già noti, mentre la A la realizza; la sonda delle tavole resta com'è, e la prova se ne scosta dove la (f) lo chiede. Il costo: una decina di righe, e un file di prove nuovo, `gui/src/testing/probes.browser.test.ts`, perché senza pezzi ad angoli diversi sulla pagina il ramo nuovo sarebbe una sonda vuota a metà (lezione 3). ✅ **Curata** nel commit che scrive questa riga, con la funzione della revisione e il rimando a questa voce. Misurata dal coordinatore il 2026-09-26: le prove a mano scritte **prima**, rosse sulla sonda di prima — `expected { near: +0, bad: [] } to deeply equal { near: 1, …(1) }` e `expected { near: 3, bad: [ …(2) ] } to deeply equal { near: 1, bad: [] }` —, verdi dopo; il progetto `browser` verde, 27 prove in tre file; le righe 1, 2 e 8 del Passo 7 rosse coi messaggi di prima, la 8b verde; e una riga nel Passo 7 |
| **E31** | ⚠️ **Compito 4, Passo 6 — la guardia di non-vacuità si accorgeva di un `dist/` che manca, non di un `dist/` rimasto da un *build* precedente:** `npm ci` non tocca `dist/`, e in locale un *build* che scrivesse altrove lasciava le due righe a leggere quello vecchio. In CI no, perché parte da un clone pulito. Registrata e non presa dalla consegna del pre-controllo del compito 4; la revisione (M-3) l'ha misurata — con `outDir: "out"` e la pagina fra gli ingressi le due righe passavano, e `out/kit.html` nasceva — e ne ha provato la cura. ✅ **Curata** nel commit che scrive questa riga: `rm -rf dist` prima del *build*, col perché nel commento; nessun costo, perché Vite vuota già la cartella quando l'uscita è al suo posto. Misurata dal coordinatore il 2026-09-26, con le righe del *build* prese dallo script al momento della corsa: con la cura, l'uscita spostata e il `dist/` vecchio, rosso *«dist/index.html is missing: the build produced nothing to check»*, e così `bash scripts/gate-gui.sh` intero; con lo script di prima della cura, nella stessa situazione, le due righe passano; con la configurazione del commit, verdi; e la pagina fra gli ingressi, rosso *«the kit page is in the package»* come prima |
| **E32** | Nit — **Compito 4, Passo 1 — `iconsCentred` guardava l'attributo `stroke`, non il colore disegnato:** una regola CSS vince sull'attributo di presentazione, e con `stroke: var(--color-text-stop);` sulle icone delle etichette la prova *«draws every icon in currentColor»* restava verde (revisione del compito 4, N-1). Fedele a `sonda-icone.js`, come la sonda di **E30** a `sonda-raggi.js`, e la prova se ne scosta per la stessa ragione: il suo nome dice ciò che si disegna. ✅ **Curata** nel commit che scrive questa riga, col testo della revisione: lo `stroke` calcolato contro il `color` calcolato. Misurata dal coordinatore il 2026-09-26: verde sul codice curato; con quella regola, rosse le sole due prove delle icone, `stroke is not currentColor: modules` e le altre icone delle etichette; e una riga nel Passo 7 |
| **E33** | Nit — **Compito 4, Passo 7 — il limite di E27 non stava scritto da nessuna parte:** il ramo *fuori dall'angolo* lo esercitava la sola riga del Passo 7, mai il cancello, e né il commento della sonda né quello della prova lo dicevano (revisione del compito 4, N-2). ✅ **Curata** nel commit che scrive questa riga, dalle prove a mano di **E30**: una coppia a 17/13 dall'angolo, una più tonda della regola e una no, tiene il ramo nel cancello. Misurata dal coordinatore il 2026-09-26: col ramo reso cieco, `: true`, rossa la sola prova che rifiuta, `expected { near: 1, bad: [] } to deeply equal { near: 1, …(1) }`; e una riga nel Passo 7 |
| **E34** | ⚠️ **Compito 5, Passo 7 — le righe 1 e 2 della tabella rompono DUE regole, e il loro rosso non prova quella nuova:** la `x` scritta dentro il `<button>` di `panels/Strip.vue` e dentro la lista di `frame/ViewBar.vue` è testo nudo, che `@intlify/vue-i18n/no-raw-text` prende. Ciascuna riga dava due errori — *«a button is BaseButton»* o *«a list is BaseList»*, e *«raw text 'x' is used»* —, e **senza** il blocco `harness/panels-and-frame` il linter restava rosso lo stesso, `raw text 'x' is used`: la mutazione troppo larga del vicolo cieco dell'[audit](../../audit-2026-08-27.md), che fa cadere più promesse di quella che prova. Misurato il 2026-09-26 dal pre-controllo sulla copia `%TEMP%\pc5`, macchina `Jays`, col compito rifatto dal testo del piano, una riga per volta e indietro con la copia salvata, `git status --porcelain` com'era. ✅ **Corretta** nel commit che la scrive: le due violazioni senza parole, `<button type="button"></button>` e `<ul><li></li></ul>`, e l'Atteso dice **un errore solo**, quello della regola — misurato: un errore ciascuna col blocco, uscita 0 senza. La riga 3, `import { DialogRoot } from "reka-ui";`, dava già un errore solo |
| **E35** | ⚠️ **Compito 5, Passi 8 e 9 — il verbale del lettore di schermo non lo può scrivere chi fa il commit:** il Passo 8 è *«un passo per il proprietario o per chi rivede, non per un subagente»*, e il Passo 9 dettava la riga 5 della tabella della posizione — *«Stato `✅ <data>` col verbale»* — nel commit dell'implementatore, che è un subagente e il verbale non l'ha; né l'ha il revisore, subagente anche lui: l'Assistente vocale lo sente solo il proprietario. Trovata dal pre-controllo il 2026-09-26. ✅ **Decisa dal coordinatore con `anthropic-skills:decision-principles`** — reversibile, poche righe, e la decisione 21 del disegno non cambia: la prova resta a mano, col verbale —: l'implementatore **salta** il Passo 8 e lascia la riga 5 a `⬜`; il Passo 8 lo fa il proprietario col coordinatore, dopo la revisione e le sue cure, sul codice che resta; il coordinatore porta la riga 5 a `✅ <data>` col verbale nel commit della chiusura, e se un annuncio dei punti 3, 4 o 5 non si sente lo dicono una voce d'errata e la cella. Scartata la via che lascia la cella all'implementatore, `✅` col verbale scritto dopo: fra i due commit la riga direbbe fatto un passo che non lo è. ✅ **Corretta** nel commit che la scrive: il testo dei Passi 8 e 9. ✅ **E le parole del Passo 8 tornano**, rifatto il 2026-09-26 sul server di sviluppo della copia fino al punto 5, **senza** l'Assistente vocale: la fascia al caricamento, via con `Accepted`, di nuovo con `StaleBuild`; dopo `Policy` e un clic vero su «Locale», *«Richiesta inviata: in attesa del core.»* nella regione di Impostazioni; col `Verdict`, *«Ultima richiesta di VRAM: rifiutata chiesti 4096 MiB, tetto 1024»* in quella di Stato. Che si **sentano** resta al proprietario |
| **E36** | Nit — **Compito 5 — un commento di `gui/src/tokens/dock.css` che il compito rende falso** (gotcha **#58**): il ponte spiega `color-scheme: inherit;` coi controlli nativi del dock — *«scrollbars, the radios of Impostazioni»* — e con *«the radio NOT chosen looks chosen»* (**E4**), e dal Passo 5 i radio di Impostazioni sono i `button` di `reka-ui` (trappola 10). Nessun compito toccava la frase, e il compito 6 riscrive `dock.css` per intero: sarebbe rimasta falsa fra il 5 e il 6. La specie di **E17**, **E22** ed **E28**. Trovata dal pre-controllo il 2026-09-26. ✅ **Corretta** nel commit che la scrive: una sostituzione nel Passo 5, che tiene le barre di scorrimento e data i radio, e il file nella riga *Files*; si applica una volta sul file di oggi, e il linter e le prove restano verdi, misurato sulla copia |
| **E37** | Nit — **Compito 5, Passo 1 — la guardia di non-vacuità di `settings.browser.test.ts` non porta il segno che la Definizione di «fatto» cerca:** la riga del controllo 20 conta, per ogni file del browser, `NON-VACUITY\|toBeGreaterThan\(0`, e sul file di questo compito rendeva **0** — misurato il 2026-09-26 sulla copia: `kit.browser.test.ts` 13, `probes.browser.test.ts` 1, `tokens.browser.test.ts` 4, `settings.browser.test.ts` 0. La guardia c'è, in un'altra forma, `expect(radios()).toHaveLength(2);`, e la Definizione dice che un file a 0 si legge; ma ogni altro file del browser la segna (vincolo 11), e il compito 9 sarebbe dovuto tornare qui. ✅ **Corretta** nel commit che la scrive: un commento `⛔ NON-VACUITY` sopra quella riga, nel recinto del Passo 1 |
| **E38** | ⚠️ **Compito 5, Passo 3 — il cassetto aperto dalla tastiera dà il fuoco a «Chiudi» fuori dalla vista:** con `BaseList` le diciotto righe non stanno più nei `60vh` del foglio `sheet`, e `reka-ui` 2.10.4 dà il fuoco al primo controllo — «Chiudi», l'ultimo — con `preventScroll`: il foglio non scorre, e chi usa la tastiera non vede dove sta il fuoco. Rompe la riga «focus» della (a), *«visibile … e non nascosto»*; a `39827e8`, con la `Drawer.vue` di prima, il pulsante si vedeva. Trovata dalla revisione del compito 5 (I-1), guardando la SPA — nessuna prova apre il cassetto —, Chrome 154, 1440 × 900: «Chiudi» a y 993–1025 col foglio a 334–900 e `scrollTop` 0. ✅ **Curata** nel commit che scrive questa riga, in `BaseDialog.vue` e non nel cassetto, perché è la forma `sheet` che deve tenere in vista le sue azioni: la barra delle azioni `sticky` sul bordo del foglio, lo scostamento e il margine negativi che coprono il `padding` del foglio — dove, con la sola `bottom: 0` della revisione, le righe si vedevano passare —, e sopra i pulsanti lo spazio dell'anello del fuoco, che altrimenti si disegna sulla riga che passa sotto; una sostituzione nel Passo 3, e il file nella riga *Files*. Misurata dal coordinatore il 2026-09-26 nel clone della revisione, Chrome installato, 1440 × 900, nei due temi, col foglio aperto, a metà e in fondo: la barra a y 851–899, «Chiudi» col fuoco a 855–887, e sul bordo del foglio la barra, non una riga; lo scorrimento invariato, 702 su 564. ⛔ **La sonda resta al compito 8**, che riscrive il cassetto e ne porta la prova nel browser, oggi aperta col clic e attenta al solo ritorno del fuoco (P-22): il suo pre-controllo vi aggiunge l'apertura **dalla tastiera** e il fuoco **dentro la vista** |
| **E39** | ⚠️ **Compito 5, Passo 6 — il segnaposto della ricerca non dice più chi la riempie:** in `BaseTextField` — 14 px di Geist, e l'icona — servono 315 px, e i `max-width: 320px` ne lasciano 270: a video resta *«…arriva col sotto-p»*. La decisione 16 della stella polare vuole che la casella dica chi la riempie, e il commento della riga lo dice ancora (gotcha **#58**, nella sua forma visiva); a `39827e8` ci stava, 288 su 312. Trovata dalla revisione del compito 5 (M-1), guardando la SPA: nessuna prova del browser guarda la barra. ✅ **Curata** nel commit che scrive questa riga: `max-width: 24rem;` nel recinto del Passo 6 e in quello del compito 8, che riscrive la barra con la stessa riga — un numero scritto a mano dal piano, come il 320, e in `rem` come la larghezza della finestra di `BaseDialog.vue`; le parole approvate della parte 2 non si toccano. Misurata dal coordinatore il 2026-09-26 nel clone, nei due temi: 315 px su 334, e il segnaposto intero a video. ⛔ **La sonda resta al compito 8**, che porta la prima prova del browser sulla barra: il suo pre-controllo vi misura il segnaposto contro la casella, come ha fatto la revisione |
| **E40** | ⚠️ **Compito 5, Passo 5 — la premessa di E2 non la teneva il codice:** il gruppo del tema era acceso **prima** del benvenuto del core, e `chooseTheme` con `saved` nullo manda `{"layouts":{},…}` — misurato dalla revisione del compito 5 (M-2) sotto jsdom. Con un core lento il `SaveLayout` arriva dopo il benvenuto, e il core custodisce il pacchetto senza le disposizioni: perse in silenzio — dedotto dal codice di `stores/layout.ts`, non misurato, perché qui un core vero non c'è. ✅ **Curata** nel commit che scrive questa riga con la strada **A** della revisione, decisa coi cinque criteri: il gruppo spento finché `layout.arrivals` è zero, come la policy finché il core non l'ha detta, *«the rest off»* (§6a); la strada **B**, la scelta tenuta e fusa all'arrivo, cambiava il negozio della parte 2 per una finestra che la **A** chiude nel componente. Nel blocco di Impostazioni del Passo 1 una prova nuova, scritta prima e vista rossa sul codice del compito, `expected false to be true`, verde con la cura; e la prova *«chooses the theme…»* consegna prima il benvenuto. ⚠️ **La stessa finestra vale per il `settle` del dock, dalla parte 2** — una mossa prima del benvenuto manda la sola vista aperta —: la domanda è di classe e del proprietario, **registrata e non presa** |
| **E41** | ⚠️ **Compito 5, Passo 1 — «Escape e il velo sono il no» (ADR-0016) senza una prova:** tolta la riga `if (value !== true) invoke.refuse();` da `Confirm.vue`, `npm test` restava verde. Già così a `39827e8`, ma la strada è nuova — il `defineModel` di `BaseDialog.vue`. Trovata dalla revisione del compito 5 (M-3), che ha scritto la prova e l'ha provata nel suo clone. ✅ **Curata** nel commit che scrive questa riga, col testo della revisione: una terza prova in `describe("the confirmation window", …)`, una sostituzione nel Passo 1. Misurata dal coordinatore il 2026-09-26: verde col `Confirm.vue` di `39827e8`, quindi nel Passo 1 è verde come le altre due della finestra; verde con quello del compito; con `void value;` al posto della riga, rossa, `expected { tool: 'arbiter', …(2) } to be null`, e la copia salvata tornata, `cmp` uguale |
| **E42** | Nit — **Compito 5, Passo 5 — la regione vuota di Impostazioni raddoppiava lo spazio fra i due gruppi:** alta zero, ma nella colonna col `gap` prendeva due spazi, 32 px dove altrove sono 16. Trovata dall'implementatore del compito 5 e confermata dalla revisione (N-1). ✅ **Curata** nel commit che scrive questa riga: il gruppo della policy e la sua regione in un contenitore solo, e le parole della regione a `--space-2` dal loro gruppo — non `display: none` né `display: contents`, perché M-3 vuole la regione nell'albero **prima** delle parole. Misurata dal coordinatore il 2026-09-26 nel clone, nei due temi: vuota, 16 px fra i gruppi; con una chiamata in volo, le parole a 8 px dal loro gruppo e a 16 dal tema |
| **E43** | ⚠️ **Compito 5, Passo 8 — la fascia che entra fa lampeggiare le due barre di scorrimento della pagina:** `dockview` 8.3.1 si ridimensiona un fotogramma dopo — `watchElementResize` passa la misura nuova a un `requestAnimationFrame` —, e per quel fotogramma la griglia del dock resta alta di prima e sborda dal suo contenitore, `.dock` di `Frame.vue`, che non la taglia: lo sbordo arriva alla pagina, che mostra le sue due barre, in basso e di lato. Visto dal proprietario al Passo 8, nel suo Chrome: *«sembra flickerare quando scompare la fascia come se comparissero e scomparissero continuamente le barre di scorrimento in basso ed al lato di essa»*. Misurato dal coordinatore il 2026-09-26 in Chrome senza finestra **con le barre accese** — Playwright le spegne per difetto, `--hide-scrollbars` —, un fotogramma per volta, a 1920 × 950, 1400 × 960, 1000 × 700 e altre misure: all'ingresso della fascia, `StaleBuild`, la pagina con le due barre per un fotogramma; all'uscita, nulla. La differenza col momento che il proprietario ha riportato resta; la cura l'ha chiusa nel suo Chrome. **Di prima del compito 5**: lo stesso lampo a `39827e8`, dalla parte 2; la revisione del compito 5 ha guardato la SPA con Playwright, che spegne le barre. ✅ **Curata** nel commit che scrive questa riga: `overflow: clip` su `.dock`, col perché nel commento — `clip` e non `hidden`, perché niente deve poter scorrere quella scatola —; i contenitori interni di `dockview` tagliano già sullo stesso bordo, quindi a regime non cambia nulla di ciò che si vede. Misurata: con la cura nessun fotogramma con le barre della pagina, alle stesse misure; e il proprietario, di nuovo nel suo Chrome: *«non tremola più»*. Una sostituzione nel Passo 4, il file nella riga *Files*, e la riga allineata nel *Trova* e nel *Sostituisci* del compito 6 e nel `Frame.vue` del compito 8. ⛔ **La sonda resta al compito 8**, che monta la cornice intera nel browser: il suo pre-controllo vi porta la pagina che non sborda mai quando la fascia entra, fotogramma per fotogramma — lo sbordo si legge su `scrollHeight`, che le barre spente non nascondono |
| **E44** | ⚠️ **Compito 6, Passi 3, 4 e 7 — la prova dei raggi del dock giudicava ciò che il dock non disegna, e il contenitore galleggiante della (c) non lo giudicava nessuno.** Misurato il 2026-09-26 dal pre-controllo sulla copia `%TEMP%\pc6`, macchina `Jays`, col compito rifatto dal testo del piano su `3a8cfdd`: al Passo 4 l'Atteso *«verde»* è falso — *«keeps every radius in it concentric (answer 4)»* rossa nei due temi, `radio in dv-groupview, bottom-left: radius 12.0, outer 20.0, distance 13.0/19.0`. È un radio di Impostazioni: nella Home, su un ospite di 1400 × 800, il pannello ha 198 px per 256 di contenuto e scorre, e il radio «Chiaro» cade a 19 px dal fondo della scheda, dove la regola *fuori dall'angolo* lo rifiuta; con la `Settings.vue` di `545f500`, prima di **E42**, il contenuto era alto 272, il radio a 3 px e le sei prove verdi — il compito era scritto contro il compito 5 dettato, non contro quello curato (la riga 5 di `CLAUDE.md`). ⛔ **E la radice è più larga del rosso:** nella Home la sonda incontrava sei coppie vicine, e nessuna del dock — quattro erano il puntino dentro un radio, un pezzo del kit, e due i radio che scorrono sotto il bordo —: la guardia `near > 0` la teneva ciò che il dock non disegna, la malattia di **E29**; e il contenitore galleggiante, che la (c) vuole col raggio dei token e per cui **E30** era nata, non lo giudicava nessuna prova. ✅ **Corretta** nel commit che la scrive: la prova stacca un gruppo, come quella del livello, con un aiutante comune; il contenitore ha `--radius-card`; nei suoi angoli la sonda incontra almeno quattro coppie — la barra del titolo sopra, il gruppo sotto — e nessuna storta; e nel dock intero nessuna storta. La sonda non giudica più ciò che arriva al suo antenato **attraverso una scatola che scorre**: sta dove l'ha messo lo scorrimento, non a una distanza che qualcuno ha disegnato — con una prova a mano nelle due direzioni in `probes.browser.test.ts`. Scartato un `skip` sul contenuto dei pannelli, provato sulla copia: toglieva al giudizio anche il pulsante a pillola della striscia, che non scorre, e che la riga *«la pillola»* del Passo 7 del compito 8 vuole giudicato proprio dalla prova dei raggi del dock. Nella riga *«la distanza»* del Passo 7 il ⚠️ dei raggi esce, e sette righe nuove provano le cure di **E44**…**E47**. ⚠️ **Col gruppo staccato Stato lascia la colonna, e Impostazioni non scorre più:** senza la regola dello scorrimento la prova del dock resta verde, misurato — la regola c'è perché un pannello che cresce non faccia cadere la prova del dock, e la tiene la sua prova a mano |
| **E45** | ⚠️ **Compito 6, Passo 4 — la riscrittura di `dock.css` toglieva la regola di E170 ed E173 della parte 2, e la presa grande era alta 32 px, non 40.** `.dock .dv-tabs-container > .dv-tab { padding-top: 0; padding-bottom: 0; }` non c'era più, e tornava il `padding: 0.25rem 0.5rem` di `.dv-tab` in `dockview.css` 8.3.1: misurato sulla copia il 2026-09-26, ogni linguetta visibile alta 40 e il suo `.bigtab` 32, `padding` `4px`/`4px`. La (c) vuole la presa alta 40 px, `--size-control-lg`; la mossa 5 di SP-8 fu giudicata su 40; e il commento del blocco del tema lo dice ancora, *«the tab fills it»* (gotcha **#58**). Nessuna prova la vedeva: sotto jsdom non c'è disposizione, e quelle del browser guardavano schede, raggi e livello. ✅ **Corretta** nel commit che la scrive: la regola sotto la nostra classe, `.dockview-theme-harness .dv-tabs-container > .dv-tab`, col combinatore di figlio di **E173** e il perché nel commento; e una prova nel browser, *«keeps the grab as tall as its strip»*, contro `--size-control-lg` calcolato. Misurata: verde con la regola; senza, rossa nei due temi, `expected [ 32, 32, 32, 32, 32 ] to deeply equal []`; e al Passo 8 le cinque prese a 40, `padding` `0px 8px` |
| **E46** | ⚠️ **Compito 6, dalla sonda del compito 4 — `concentricRadii` dimezzava ogni angolo da solo, e il CSS no.** `effective` prendeva `min(r, h/2, w/2)`, giusto solo per quattro angoli uguali: il CSS riduce **tutti** i raggi di un fattore solo, e solo se i due raggi di un lato superano il lato — CSS Backgrounds 3, §4.5 *«Overlapping Curves»*, letta alla fonte il 2026-09-26. La barra del titolo del gruppo galleggiante, `20px 20px 0 0` in 24 px, si disegna a 20, perché ogni lato tiene 20 + 0 in 24; la sonda la leggeva 12, e con la cura di **E44** dava un rosso falso nei due temi, `dv-floating-titlebar in dv-resize-container, top-left: radius 12.0, outer 20.0, distance 1.0/1.0`. **E30** aveva letto un raggio per angolo e tenuto la riduzione del caso simmetrico. Trovata dal pre-controllo il 2026-09-26, curando E44. ✅ **Corretta** nel commit che la scrive: il fattore della regola in `effective`, col perché e la fonte nel commento; due prove a mano in `probes.browser.test.ts` — la barra `20px 20px 0 0` alta 24 a 1/1, `{ near: 2, bad: [] }`, e la stessa coi quattro angoli a 20, disegnati a 12, rifiutata nei due angoli di sopra —; e la fonte nella tabella del compito 9. Misurata: con la riduzione di prima, rosse la prova della barra e quella dei raggi del dock nei due temi; senza nessuna riduzione, rosse la prova della pillola, quella di ciò che scorre, il cui pezzo è una pillola, e quella del dock; sulla pagina kit, dove ogni pezzo ha i quattro angoli uguali, nessun esito cambia |
| **E47** | ⚠️ **Compito 6, Passo 4 — nel gruppo galleggiante la linguetta restava sul fondo della superficie:** il blocco del contenitore ridichiarava a `--color-bg-raised` il fondo del gruppo e quello dell'intestazione, e non i quattro delle linguette, che il blocco del tema lega a `--color-bg-surface`. Visto dal pre-controllo al Passo 8, nello scuro: dietro «STATO» e i suoi due comandi un rettangolo più scuro sull'intestazione rialzata — misurato, la linguetta `rgb(27, 23, 24)` e l'intestazione `rgb(36, 31, 32)`; nel chiaro i due ruoli hanno lo stesso valore, e non si vede. La (c) vuole i gruppi galleggianti `--color-bg-raised`. ✅ **Corretta** nel commit che la scrive: i quattro fondi delle linguette a `--color-bg-raised` nel blocco del contenitore, col perché nel commento; e nella prova del livello, che stacca già un gruppo, l'intestazione e la linguetta contro `--color-bg-raised` calcolato. Misurata: verde con la cura; senza, rossa la sola prova dello scuro, `expected 'rgb(27, 23, 24)' to be 'rgb(36, 31, 32)'` — ⚠️ la metà chiara non può mordere, perché nella tavola i due ruoli coincidono |
| **E48** | Nit — **Compito 6, Passo 8 — il guardare non nominava ciò che solo lì si vede:** **(a)** con Playwright senza finestra le barre di scorrimento non si vedono (la lezione 1 della consegna dell'esecuzione del compito 5, in archivio), e le barre dei pannelli che scorrono — Stato e Impostazioni nella Home a 1440 × 900 — arrivano ora all'angolo tondo della scheda, che ne taglia il binario: la freccia resta intera, misurato ingrandendo i due angoli nei due temi, ed è aspetto, che giudica il proprietario (controllo 15); **(b)** **E47** si vedeva solo nello scuro, e col gruppo staccato. ✅ **Corretta** nel commit che la scrive: il Passo 8 nomina il gruppo staccato e le barre all'angolo, e dice di guardare con le barre accese |
| **E49** | Nit — **Compito 6, Passo 7 — due righe facevano cadere più prove di quelle che l'Atteso nomina**, la specie di **E12**: dal rapporto dell'implementatore, confermata dalla revisione. **(a)** *«la distanza»*, `gap: 0 };`, fa cadere anche sotto jsdom la prova del `gap` in px di `frame.test.ts`, `expected [Function] to throw an error` — il `gap` scritto a mano non legge il token —, `Tests  3 failed \| 174 passed \| 1 skipped (178)`; **(b)** *«la riduzione»*, `return radius(element, corner);`, fa cadere anche nei due temi *«keeps every radius concentric (answer 4)»* della pagina kit, `expected [ …(24) ] to deeply equal []` — radio e pillole letti 9999 —, `Tests  6 failed \| 171 passed \| 1 skipped (178)`. Nessun codice cambia. ✅ **Corretta** nel commit che la scrive: le due righe del Passo 7 le nominano |
| **E50** | ⚠️ **Compito 6 — quattro commenti dettati nominavano `themeAbyss`**, e il controllo 15 del disegno dice *«`themeAbyss` non compare più nel sorgente»*: la sua riga nella *Definizione di «fatto»*, `grep -rn 'themeAbyss' gui/src \| wc -l` con l'attesa `0`, avrebbe reso **4** al compito 9 — misurato dal coordinatore e dalla revisione (I-1) a `c1102fc`, in `frame/dock.ts`, `frame/frame.test.ts`, `tokens/dock.css` e `tokens/dock.test.ts`, nessun `import` e nessun uso. Scartate la strada B, un comando che guardi il solo codice, che lascia falso alla lettera il controllo del disegno — merito approvato, e un filtro sui commenti è fragile —, e la C, un `4` scritto, che invecchia (gotcha #31). ✅ **Corretta** nel commit che la scrive, la strada A: i quattro commenti dicono *«the abyss theme»* col senso di prima, nei sorgenti e nei recinti; nessuna prova cambia, e il comando torna a rendere 0 |
| **E51** | ⚠️ **Compito 6, Passo 4 — il secondo ciclo di `--dv-overlay-z-index` non era curato:** `.dv-render-overlay { --dv-overlay-z-index: var(--dv-overlay-z-index, 999) }` in `dockview.css` 8.3.1, che il Passo 1 misura accanto a quello del contenitore, e `dockview-core` alza il contenuto di un pannello `renderer: "always"` in un gruppo staccato a quel livello + 2i + 1: misurato dalla revisione (M-1) a **1000**, sopra una scatola a `--z-overlay`, col contenitore a 50. Oggi latente — nessun pannello è `always` —, e della specie di R3-17. ✅ **Corretta** nel commit che la scrive: la regola su `.dv-render-overlay`, nel tema; in `dock.browser.test.ts` la prova *«keeps an always-rendered panel's content…»*, che porta lei il pannello, rossa senza la regola nei due temi, `expected 1000 to be 51`; una riga nel Passo 7 |
| **E52** | Minore — **Compito 6, Passo 3 — la prova delle schede non guardava il bordo né l'ombra** che la (c) nomina: tolti, la suite restava verde (revisione, M-2). ✅ **Corretta** nel commit che la scrive: due attese, `borderTopColor` a `--color-border-card` e `boxShadow` a `--shadow-card`; senza il bordo rossa nei due temi, senza l'ombra nel solo chiaro — nello scuro `--shadow-card` è `none`, come per **E47**; due righe nel Passo 7 |
| **E53** | Nit — **Compito 6, Passo 4 — la regola di ciò che scorre guarda le sole scatole FRA l'elemento e l'antenato:** un antenato tondo che scorre lui stesso è giudicato lo stesso, alla posizione di scorrimento che ha — misurato dalla revisione (N-2) con un caso a mano; oggi latente. ✅ **Corretta** nel commit che la scrive: il commento della sonda lo dichiara. ⚠️ **Al pre-controllo del compito 8**, accanto alla domanda di **E44** sulla Panoramica |
| **E54** | Nit — **Compito 6, Passo 5 — il commento della presa diceva che `dockview` comincia il trascinamento anche su `mousedown`:** 8.3.1 non lo ascolta — `grep -c '"mousedown"' gui/node_modules/dockview-core/dist/package/main.esm.mjs` → 0, il 2026-09-26 —, e `@mousedown.stop` e `@click.stop` non hanno una prova (revisione, N-3); ereditato da `BigTab.ts` di `1a83208`. ✅ **Corretta** nel commit che la scrive: il commento dice che la prova tiene `pointerdown`, e che gli altri due restano, come SP-8 li ha giudicati, senza prova — dichiarati, non tolti |
| **E55** | ⚠️ **Compito 6, Passo 8 — la zona d'arrivo aveva gli angoli dritti dentro la scheda tonda:** il proprietario, nel suo Chrome, il 2026-09-26. Col montaggio predefinito di 8.3.1 la zona è `.dv-drop-target-selection`, dentro il `.dv-content-container` del gruppo — misurato trascinando con un mouse vero, Playwright sul Chrome installato —, e **non** il `.dv-drop-target-anchor` che i temi «spaced» arrotondano, che è del montaggio `absolute`: una prima cura su quello, provata su un elemento messo a mano, era verde e non toccava la zona vera. ✅ **Corretta** nel commit che la scrive: nel tema `.dv-drop-target-selection` col raggio della scheda meno il bordo (risposta 4); in `dock.browser.test.ts` una prova che **trascina davvero** una linguetta — `userEvent.dragAndDrop` con `{ steps: 12 }`, perché con `dndStrategy: "pointer"` un salto solo non disegna la zona, misurato: la guardia rossa — e la legge con un osservatore; rossa senza la regola nei due temi, `expected [ '0px' ] to deeply equal [ '19px' ]`; stabile, 5 corse su 5; una riga nel Passo 7. Il richiamo nella (c) del disegno |
| **E56** | ⚠️ **Compito 6, Passo 8 — due sfondi, e nel chiaro le schede sprofondavano:** il proprietario, il 2026-09-26. `.dv-dockview { background-color: var(--dv-group-view-background-color) }` in `dockview.css` 8.3.1, e il nostro tema dà a quella variabile la superficie delle schede: la griglia intera si dipingeva del loro colore, sotto e intorno fino al margine di `.dock`, contro la (c), *«sul fondo `--color-bg`»* — misurato risalendo gli strati: l'unico dipinto è `dv-grid-view dv-dockview`, `rgb(251, 248, 243)` nel chiaro e `rgb(27, 23, 24)` nello scuro. ✅ **Corretta** nel commit che la scrive: nel tema la griglia trasparente; in `dock.browser.test.ts` la prova *«paints nothing between the cards…»*, che risale da ogni scheda all'ospite, rossa senza la regola nei due temi, `expected [ Array(1) ] to deeply equal []`; una riga nel Passo 7 |
| **E57** | ⚠️ **Compito 6, Passo 8 — la barra aveva un fondo suo e una riga sotto:** il proprietario, il 2026-09-26: *«eviterei di colorarla diversamente dallo sfondo (anche il border bottom) e renderla un tutt'uno con esso»*. È ciò che la tavola disegna — `.m-bar`, né fondo né bordo —: il fondo `--color-bg-raised` col bordo veniva dalla parte 2, e i compiti 5 e 8 lo portavano avanti. ✅ **Corretta** nel commit che la scrive: `frame/ViewBar.vue` senza le due righe, col perché nel commento, al Passo 4; il recinto di `ViewBar.vue` del compito 8 allineato; il richiamo in *«La barra»* del disegno. ⚠️ **La sonda al pre-controllo del compito 8**, che monta la cornice intera nel browser (`frame.browser.test.ts`), come per **E43** |
| **E58** | ⚠️ **Compito 6, Passo 8 — il nucleo era vestito da scheda:** il proprietario, il 2026-09-26: *«la scheda del grafo … deve fare eccezione, ed essere un tutt'uno con lo sfondo»*. È la stella polare — *«il nucleo al centro … intorno le tessere»* — e la tavola, `.m-core` fra le schede `.m-mod`. ✅ **Corretta** nel commit che la scrive, sullo schema della striscia del compito 8 (*«Found by what it holds»*): la radice del nucleo porta `.nucleus` — oggi quella di `Placeholder.vue`, per il modulo che lo interpreta —, e il tema toglie al suo gruppo fondo, bordo e ombra con `:has(.nucleus)`; in `dock.browser.test.ts` la prova *«draws the nucleus on the page…»*, e la prova delle schede lo salta; rossa senza la regola nei due temi e, senza il segno, `expected [] to have a length of 1 but got +0`; due righe nel Passo 7. La prova di `modules.test.ts` riconosceva il segnaposto dal testo `class="placeholder"`, e con la seconda classe cadeva: legge ora la classe come parola. Il richiamo nella (c) del disegno |
| **E59** | ⚠️ **Compito 6, Passo 8 — la barra di scorrimento è quella di Windows, e nessuna tavola la disegna:** il proprietario, il 2026-09-26, sul pannello Impostazioni: *«va creato un componente ui universale anche per la scroll bar personalizzata, é orrenda cosí»*. È l'aspetto che **E48** gli lasciava: l'arco della scheda ne taglia il binario. Nessuna delle quattro tavole approvate disegna una barra — `grep -c -i scrollbar` rende 0 su ciascuna —: un buco del disegno, non una divergenza. Nel codice nessuna regola nostra; le scatole che scorrono le elenca `grep -rn -E 'overflow(-[xy])?: auto' gui/src --include='*.vue' --include='*.css'`, e `dockview` 8.3.1 veste da sé le sole barre delle linguette. ✅ **Decisa dal proprietario il 2026-09-26 — A**: una regola sola nella tavola dei token, ricopiata in `base.css` accanto a `:focus-visible` e `::selection`, che vale per ogni scatola che scorre — la nostra, quelle di `dockview` e di `reka-ui`, e quelle che arriveranno —; contro **B**, un pezzo `BaseScrollArea` sopra `ScrollArea` di `reka-ui` 2.10.4, universale solo dove lo si avvolge, che per ciò che non è nostro avrebbe chiesto anche la A. ✅ **E l'aspetto, la C**, scelto guardando quattro varianti vive nel suo Chrome, nei due temi, dentro una scheda come Impostazioni: coi pseudo-elementi `::-webkit-scrollbar`, larga 10 px col cursore a 6 — un bordo trasparente di 2 px e `background-clip: padding-box` —, arrotondato a `--radius-full`, binario trasparente, **senza frecce** (`::-webkit-scrollbar-button`), staccata dalle estremità di `--space-3` (il `margin-block` del binario), col colore `--color-border-strong` e `--color-text-muted` sotto il puntatore; il cursore **si vede solo col puntatore sopra la scatola o col fuoco dentro** (`:hover` e `:focus-within` della scatola). Scartate la A delle varianti, le sole proprietà standard `scrollbar-width: thin` e `scrollbar-color` — misurato nel pannello del browser dell'app: le frecce restano, piccole —, e la B, la stessa C sempre visibile. Le fonti, lette il 2026-09-26: *Scrollbar styling* di Chrome for Developers, aggiornata il 2024-01-17 — le proprietà standard da Chrome 121, solo colore e spessore: niente raggio, margini o frecce —; e `::-webkit-scrollbar` su MDN, aggiornata il 2026-08-27 — **non standard**, e spenta dove `scrollbar-color` o `scrollbar-width` valgono altro che `auto`, con `scrollbar-color` ereditata. ⚠️ **I costi, dichiarati:** pseudo-elementi non standard, ma di Chromium, cioè di Electron e del Chrome delle prove; la preferenza di Windows che tiene sempre visibili le barre non la vediamo — dedotto, da misurare —, e così il comportamento sotto i colori forzati; una scatola che imposta `scrollbar-width` o `scrollbar-color` resta fuori dalla regola — `.dv-tabs-container` di `dockview` ha `scrollbar-width: thin`. ⏳ **La cura non è scritta:** il richiamo datato nella (a) del disegno, la regola nella tavola dei token e le fonti in [`riferimenti.md`](../../riferimenti.md), poi il piano con la prova vista rossa — la larghezza misurata, `offsetWidth − clientWidth`, a 10 — e lo sguardo del proprietario. ⚠️ **RICHIAMO DEL 2026-09-27, al disegno delle tre voci:** la parte del **disegno** è scritta — la riga della barra nella (a), il controllo 22 e l'assunzione del [disegno](../specs/2026-09-22-design-system-design.md), e le fonti con due misure in [`riferimenti.md`](../../riferimenti.md) —, con tre precisioni che l'aspetto approvato vuole e la voce non diceva: il distacco **per asse** — col solo `margin-block` l'orizzontale arriva agli angoli, misurato in Chromium 152 —; l'**angolo** fra le due barre trasparente — Chromium lo disegna chiaro, misurato —; e un token per lo spessore, `--size-scrollbar`, che la prova legge invece di ricopiare il numero. ⏳ **Resta:** la regola nella tavola e la sua copia, il piano con la prova vista rossa, lo sguardo del proprietario. ⚠️ **RICHIAMO DEL 2026-09-27, allo sguardo del proprietario:** le tre voci **approvate — A**, in chat — guardandole insieme su una copia della tavola dei token nello scratchpad. Verificandola prima di mostrarla, **la C com'è scritta non si accende col puntatore**: Chromium ridisegna una barra personalizzata solo quando cambia lo stile della scatola stessa, e una regola sul solo pseudo-elemento non lo cambia — misurato nel Chrome 154 installato e nel Chromium 152 del pannello dell'app, con varianti ferme dal caricamento: col fuoco il cursore compare, perché il contorno cambia la scatola; col puntatore no; e compare se al passaggio la scatola cambia anche una sola variabile. La cura, decisa dal coordinatore come scelta tecnica contenuta e mostrata al proprietario con la pagina e col confronto senza: l'**innesco**, una proprietà registrata che **non si eredita**, `--scrollbar-trigger`, a 1 su `:hover` e `:focus-within`. ⚠️ Il puntatore delle prove era simulato: la conferma col mouse vero non è registrata, e la dà il secondo sguardo sul dock. ⛔ **E il controllo 22, com'è scritto, misura il niente:** senza finestra Playwright lancia Chrome con `--hide-scrollbars`, e `gui/vite.config.ts` lo lancia così — lo spessore vale 0 con la regola e senza, 10 togliendo il solo flag. Le misure e i metodi in [`riferimenti.md`](../../riferimenti.md). ⏳ **Resta:** la cura ⚠️ **RICHIAMO DEL 2026-09-27, al piano della cura:** la cura è il **compito 6bis**, scritto, col codice provato su una cartella di prova (**E62**). |
| **E60** | ⚠️ **Compito 6, Passo 8 — la fascia non segue la tavola, e i messaggi non hanno un pezzo:** il proprietario, il 2026-09-26, su *«Il core non ha risposto. Riprova»*: *«va reso coerente col sistema e oltre a spaziarlo correttamente dal resto come si deve, si migliora lo stile con icone … e approfitterei per renderlo universale per tutti warning, messaggi normali di sistema, errori o qualsiasi altra cosa»*. **Non è un gusto nuovo:** la tavola dei token, *«Gli stati»*, disegna già il messaggio `.msg` in tre toni — `ok`, `warn` e `stop`, con `circle-check`, `triangle-alert` e `circle-x` a `--size-icon-lg` —, un titolo `--font-body-strong` col testo `--font-caption` sotto, fondo e bordo del tono, `--radius-control` e `--space-3` dentro; e il suo esempio è il nostro, *«Il core non risponde»*. `frame/Band.vue` è una striscia da bordo a bordo con la sola riga sotto, senza icona né titolo: una **divergenza dalla tavola approvata**, come **E57** ed **E58** (la lezione 5 della consegna dell'esecuzione del compito 6, in archivio). E lo spazio: il dock ha 0 in alto (risposta 20), e con la fascia in mezzo le schede la toccano. ⚠️ **Già deciso, e detto prima della domanda:** la (b) del disegno, *«Fuori, finché non tornano due volte: i messaggi e le notifiche»* — le righe di Impostazioni e di Stato vi erano contate come regioni (`BaseStatus`), non come messaggi —; e **E228**, le notifiche dei lavori lunghi, resta del proprietario. ✅ **Decisa dal proprietario il 2026-09-26 — A**: un pezzo di base per i messaggi, **adesso** — `BaseMessage`, il nome da confermare nel disegno: il `.msg` della tavola coi tre toni più uno **neutro**, l'icona del tono dalla mappa, titolo, testo e un'azione facoltativa —; lo usano la fascia, la riga *«Richiesta inviata»* di Impostazioni e quella del verdetto di Stato; la (b) cambia col richiamo datato. Contro **B**, la fascia come nella tavola dentro `frame/`, con la (b) com'è. Il consiglio del coordinatore era la A: la tavola ha già deciso l'asse che varia, il tono. ✅ **Il tono neutro: N2**, deciso guardandolo nei due temi — fondo `--color-bg-accent-subtle`, bordo `--color-border-accent`, icona `info` a `--color-text-accent` —, contro **N1**, fondo `--color-bg-raised`, bordo `--color-border` e icona `--color-text-muted`, che il coordinatore consigliava: la tavola vuole *«un solo accento forte, usato poco»*, e nello scuro il bordeaux di N2 si avvicina al rosso di `stop`. Misurato sulla pagina di prova, nel pannello del browser dell'app: il testo sul fondo del messaggio fra 11,6 e 16,8 in ogni tono, le icone fra 5,4 e 8,0. ✅ **La fascia**, approvata guardandola: lo stesso pezzo **sulla pagina**, col raggio della scheda (`--radius-card`, con **E61**), a `--space-3` dai lati e sopra le schede, sotto la barra che le lascia i suoi 8 px; «Riprova» dentro, a `--space-3` più il bordo, con `--radius-control`: concentrico per costruzione. Un messaggio **in una scheda** ha `--radius-control`. ✅ **I toni**, con le parole di oggi (**E39**): *«Il core non ha risposto.»* `warn`, con «Riprova»; *«Il core parla una versione diversa del protocollo…»* **`stop`** — oggi `warn` —, col *«Timbro atteso»* come testo; *«Richiesta inviata: in attesa del core.»* e il verdetto `Queued` neutri; `Granted` `ok`; `Refused` `stop`, col dettaglio come testo. ⏳ **La cura non è scritta:** i richiami datati nella (b) e nella (d), il tono neutro nella tavola dei token, poi il piano e lo sguardo del proprietario. ⚠️ **RICHIAMO DEL 2026-09-27, al disegno delle tre voci:** la parte del **disegno** è scritta — nella (b) il pezzo, **`BaseNotice`** e non `BaseMessage`, col nome deciso dal coordinatore su delega del proprietario: nel codice della GUI *message* è già il messaggio del filo (decisione 28 del [disegno](../specs/2026-09-22-design-system-design.md), col tono neutro `info` e le icone col nome del tono); la sua forma — `title` e `description` come `BaseDialog`, l'azione in uno slot, nessuna chiusura a mano (decisione 29); i toni con le chiavi di oggi; e l'azione **solo sulla pagina**, che la regola dei raggi vuole: in una scheda starebbe a 13 px da un angolo di 8. Nella (d) la fascia; il controllo 23; le trappole 21–24, fra cui le cifre degli *«otto pezzi»* che la testa del piano e il compito 9 portano. E la **misura del contrasto**, rifatta da `themes.css`, **diverge** da quella scritta qui sopra: il testo sotto il titolo non stava nella gamma, e il 16,8 non si riproduce; tutte le coppie passano, e `contrast.test.ts` le giudica già — i numeri e il comando in [`riferimenti.md`](../../riferimenti.md), *«Il messaggio — E60»*. ⏳ **Resta:** il tono neutro e il messaggio sulla pagina nella tavola, col piano della cura; lo sguardo del proprietario sulle tre voci; il piano. ⚠️ **RICHIAMO DEL 2026-09-27, allo sguardo del proprietario — approvata, A:** sulla copia della tavola il tono `info` primo in «Gli stati», coi ruoli dell'accento, e in cima a ciascun tema il messaggio sulla pagina con «Riprova», allineato alle schede e col loro raggio. Misurato: «Riprova» sta **nell'angolo**, a 13 px dai due lati, perché il pulsante, alto 24, dà l'altezza alla riga — concentrico esatto —; e per la stessa ragione il titolo sta **2 px più in alto** del testo di «Riprova», com'era nella v3 approvata: detto al proprietario, che non l'ha commentato; lo decide il piano della cura, con lui. Il contrasto rilanciato col comando di [`riferimenti.md`](../../riferimenti.md): gli stessi numeri. ⏳ **Resta:** la cura ⚠️ **RICHIAMO DEL 2026-09-27, al piano della cura:** la cura è il **compito 6bis**, scritto, col codice provato su una cartella di prova (**E62**). E i **2 px** del titolo sopra «Riprova»: **B** del proprietario, con un'azione la riga sta al centro (**D27**). |
| **E61** | ⚠️ **Compito 6, Passo 8 — i raggi non contano il bordo:** il proprietario, il 2026-09-26, sul messaggio dentro la scheda della pagina di prova di **E60**: *«non sono concentrici»*. **Misurato:** il messaggio sta a 12 px di margine più **1 di bordo** della scheda, 13 dal bordo esterno, con `--radius-control`, 8: il concentrico è 20 − 13 = 7. `--radius-card: calc(var(--radius-control) + var(--space-3))` conta il solo margine, e la regola dei raggi del disegno vuole *«la distanza vera»*. **Non è della sola pagina di prova**, misurato il 2026-09-26 sul server di sviluppo a 1440 × 900, con la geometria di `concentricRadii`: nella pagina kit le righe di `BaseList` nelle schede, `r 8.0 outer 20.0 dist 13.0 -> off 1.0`; nella SPA, col gruppo di Stato staccato, `.dv-floating-titlebar` e `.dv-groupview` in `.dv-resize-container`, `r 20.0 outer 20.0 dist 1.0 -> off 1.0`. E la tavola stessa lo porta: `.card` di `token.html` ha il bordo, `--space-3` dentro e `--radius-card`. ⛔ **La sonda non l'ha mai visto:** `concentricRadii` accetta `Math.abs(inner - (outer - dx)) <= 1.5`, e un pixel passa verde. ✅ **Decisa dal proprietario il 2026-09-26 — A, il bordo entra nel raggio:** nella tavola dei token `--radius-card: calc(var(--radius-control) + var(--space-3) + var(--border-width))`, 21, ricopiato in `base.css`, e `--radius-frame` segue, 33; ciò che sta subito dentro il bordo prende `calc(var(--radius-card) - var(--border-width))`, come la zona d'arrivo di **E55** — la barra del titolo e il gruppo del contenitore galleggiante, in `tokens/dock.css` —; e la sonda scende a **mezzo pixel**, rossa prima sulla pagina kit e sul gruppo staccato coi token di oggi, verde dopo. Contro **B**, il bordo tolto dal margine — 11 px in ogni scatola col bordo che tiene qualcosa nell'angolo —: la stessa correzione in dieci posti. Il costo: due valori approvati cambiano di 1 px, e ogni superficie col raggio della scheda porta il suo bordo da 1 px, trasparente nel chiaro, com'è già. Misurato sulla pagina di prova con la A: venti coppie di angoli giudicate a mezzo pixel, nessuna storta, nei due temi; e il proprietario, guardando la scheda ingrandita coi cerchi dei raggi disegnati sopra: *«perfetto»*. ⚠️ **Un errore del coordinatore, detto al proprietario:** la prima misura della pagina di prova escludeva la cornice finta attorno alla fetta di pagina — angoli da 8 px, e dentro le schede a 13 con raggio 21 — e fu riportata come *«ogni coppia esatta»*; ora la fetta ha gli angoli dritti. ⏳ **La cura non è scritta:** il richiamo datato nella regola dei raggi del disegno e nella (f), il valore nella tavola, la sonda, e le cure che il suo rosso nomina. ⚠️ **RICHIAMO DEL 2026-09-27, al disegno delle tre voci — la cornice conta il suo bordo, A del proprietario del 2026-09-26:** qui sopra sta *«`--radius-frame` segue, 33»*, con la formula di oggi, che non conta il bordo **della cornice**. Misurato nel browser sulla tavola dei token: nella dimostrazione *«Raggi concentrici»* `.nest-frame` ha il bordo e `.nest-card` no — oggi la scheda sta a 13 px dal bordo esterno della cornice da 32, che ne vorrebbe 33; coi token di questa voce le coppie storte sarebbero **due**, la cornice 33 contro 34 e la scheda 21 contro 20; ciascuna delle due strade qui sotto le rende esatte tutte e due. Decisa **A**: `--radius-frame: calc(var(--radius-card) + var(--space-3) + var(--border-width))`, 34, e ogni superficie nostra arrotondata che contiene qualcosa di arrotondato porta il suo bordo da 1 px, disegnato o trasparente — `.kit-frame` della pagina kit uno trasparente, e resta com'è; la scheda della dimostrazione quello delle schede —; contro **B**, le cornici senza bordo, 33, con la convenzione scritta nella (a) e la dimostrazione senza il bordo che la tavola disegna. ✅ **E la curva resta il quarto di cerchio**, il 2026-09-27: alla domanda del proprietario sulla concentricità lo squircle è stato guardato su una pagina di prova e scartato — *«Vicoli ciechi e scelte scartate»* del [disegno](../specs/2026-09-22-design-system-design.md). ⚠️ **RICHIAMO DEL 2026-09-27, alla chiusura della sessione:** la parte del **disegno** è scritta, in `06dc5d5` — i richiami nel linguaggio visivo, nella (a) e nella (f), e lo squircle fra le scelte scartate. ⏳ **Resta:** i valori nella tavola — la scheda 21, la cornice 34, la scheda della dimostrazione e il foglio col loro bordo — e la loro copia, la sonda a mezzo pixel, e le cure che il suo rosso nomina. ⚠️ **RICHIAMO DEL 2026-09-27, allo sguardo del proprietario — approvata, A:** sulla copia della tavola la scheda 21, la cornice 34, il foglio 46 col suo bordo trasparente, e la scheda della dimostrazione col bordo delle schede. La sonda a mezzo pixel, portata da `concentricRadii`: sulla copia **nessuna** coppia storta su 28; sulla tavola di oggi **10 su 24** — la finestra di dialogo, lo stage, la dimostrazione — e **nessuna** con lo scarto di oggi, 1,5. ⚠️ **Sullo schermo a 175 % un bordo da 1 px si disegna 0,57 px**: le distanze misurate valgono 12,57 dove la regola ne conta 13, e mezzo pixel separa una coppia esatta, a 0,43, da una storta di un pixel, a 0,57 — a densità 1, quella delle prove, 0 contro 1. ⚠️ E la **prosa** della tavola porta le cifre vecchie — *«i raggi diventano 20 e 32»*, *«cornice 32 = scheda 20 + 12»* —: le riscrive la cura. ⏳ **Resta:** la cura ⚠️ **RICHIAMO DEL 2026-09-27, al piano della cura:** la cura è il **compito 6bis**, scritto, col codice provato su una cartella di prova (**E62**). |
| **E62** | ⚠️ **Compito 6, Passo 8 — la cura di E59–E61 è il compito 6bis, e con un'azione la riga del messaggio sta al centro:** il proprietario, il 2026-09-27, alle due domande della consegna. **A**, un compito nuovo fra il 6 e il 7, contro **B**, la cura dentro il Passo 8 del 6: la cura tocca il perimetro di cinque compiti — la tavola e i token del 1, il kit del 3, la pagina kit del 4, la cornice e i pannelli del 5, il dock del 6 — con la taglia di un compito, mentre le cure del 6 (`ca83dac`) erano 153 righe dentro il dock. E **B**, con un'azione la riga al centro, contro **A**, com'era la v3, guardando una pagina di confronto nei due temi: in alto titolo e icona stavano 2 px sopra il testo di «Riprova», a densità 1 e a 1,75 (**D27**). ✅ **Scritto** nel commit che scrive questa voce: il compito 6bis, col codice **provato** su una cartella di prova — un `git worktree` di `950bba1` nello scratchpad —, ogni rosso e ogni verde dei suoi passi misurato, le quindici violazioni del Passo 15 comprese, e il testo del compito riletto a macchina contro quel codice: applicato a una copia pulita, rende i diciotto file byte per byte. ⚠️ **Trovato scrivendolo:** a mezzo pixel coi token di oggi è storto di un pixel anche il pulsante nell'angolo della finestra di `BaseDialog`, che nessuno aveva visto — i token nuovi lo curano —; e due prove della sonda si reggevano sullo scarto di 1,5. La riga 6 della posizione resta `⏳` e si chiude con lo sguardo del 6bis |
| **E63** | ⚠️ **Compito 8 — `frame.browser.test.ts` lo crea il compito 6bis, e il compito 8 lo ESTENDE** (**D29**): il suo Passo 2 dice *«Crea»* e porta l'intestazione, gli import, `frame(theme)` e `px(token)`, che dal 6bis ci sono già, con la prova della fascia; restano da aggiungere gli import che mancano e le sue prove. ⏳ **Al pre-controllo del compito 8**, che si rilegge contro il codice di allora (`CLAUDE.md`, domanda 5) |
| **E64** | ⚠️ **Compiti 7, 8 e la Definizione di «fatto» — ciò che il 6bis rende falso fuori da sé, trovato applicando i compiti 7 e 8 SOPRA il 6bis.** Misurato il 2026-09-27 dal pre-controllo del 6bis, macchina `Jays`: il 6bis applicato dal testo del piano a un `git worktree` di `dc77fc8`, poi i *Trova*, i *Crea* e i *Riscrivi* dei compiti 7 e 8 estratti dal testo e applicati in fila — il 7 passa, tredici operazioni su tredici; l'8 ne rifiuta **due**: la *Crea* di `frame.browser.test.ts`, che è **E63**, e il **primo *Trova* del Passo 2 su `frame.test.ts`**, gli import, *«0 occurrences»*: il Passo 12 del 6bis vi aggiunge `vi`. E il suo *Sostituisci*, com'era scritto, toglieva `vi`, che la prova della fascia del 6bis usa (`vi.spyOn`). Il resto è di conto, la specie di **E25**: nel compito 8 le prove della pagina kit sono **ventidue** e non diciotto — il 6bis ne aggiunge due per tema —, quelle dei tre file del browser del Passo 3 **trentasette** e non trentuno — con le due della fascia —, il raggio della scheda nei rossi della pillola è **21** e non 20, e `frame.browser.test.ts` non è più un file nuovo del compito; nel compito 7 la base dei file di prova è il 6bis, non il 6; nella Definizione di «fatto» `frame.browser.test.ts` nasce col 6bis, e ⛔ **`ls gui/src/components/Base*.vue \| wc -l` attendeva `8`, e col 6bis rende `9`**, misurato sulla copia: la trappola **23** del disegno cercava *«otto pezzi»* scritto in parole, e una cifra non la vedeva. ✅ **Corretta** nel commit che scrive questa voce: il *Trova* e il *Sostituisci* degli import del Passo 2 del compito 8 con `vi`; gli Atteso dei Passi 3, 7 e 8 e la riga *«la pillola»* del Passo 9 del compito 8, e il Passo 7 del compito 7, coi conti nuovi — ⚠️ **dedotti**, non misurati: il compito 8 estende il file (**D29**) e le due prove della fascia sono verdi prima del suo codice; dopo, lo dice il suo pre-controllo, con **E63** —; nella Definizione il file col suo compito, e i pezzi di base **nome per nome** al posto della cifra, che vive nella (b) (gotcha **#68**); e nella riga `gui: probes` che il compito 9 scrive in `porta-di-qualita.md`, le barre mostrate (**D26**). Rifatti i compiti 7 e 8 sopra il 6bis col testo corretto: resta il solo rifiuto di **E63** |
| **E65** | Minore — **Compito 6bis, Passo 10 — la prova dei ruoli del messaggio non guarda le sue parole:** il titolo sta nel colore del testo e il testo sotto nel `muted`, come `.msg b` e `.msg span` della tavola, mentre il colore del tono lo prende la radice, che l'icona disegna; tolto `color` da `.title` e da `.description` in `BaseNotice.vue`, il titolo prende il colore del tono e le prove restano verdi — misurato il 2026-09-27 dal pre-controllo sulla copia: `kit.browser.test.ts` e `frame.browser.test.ts` `Tests  24 passed (24)`, `kit.test.ts` `Tests  31 passed (31)`. La specie di **E52**. ✅ **Corretta** nel commit che la scrive: nella prova *«dresses each tone of a message in its roles»* il titolo contro `--color-text` e ogni testo contro `--color-text-muted`, e una guardia che un testo ci sia — quello del rifiuto, nella scheda dei messaggi. Misurata: verde sul codice; senza il colore del titolo rossa nei due temi, nel chiaro `expected 'rgb(122, 31, 46)' to be 'rgb(27, 23, 24)'` — il titolo del tono neutro nel colore dell'accento —; senza quello del testo, `expected 'rgb(168, 42, 19)' to be 'rgb(101, 91, 87)'`; senza il testo del rifiuto nella pagina kit, `expected 0 to be greater than 0`; ogni file tornato dalla copia salvata, `cmp` uguale. Una riga nel Passo 15. ⚠️ I **caratteri** del titolo e del testo restano dello sguardo, come negli altri pezzi del kit |
| **E66** | Nit — **Compito 6bis — un commento di `gui/src/components/BaseDialog.vue` che il compito rende falso** (gotcha **#58**): *«The radii follow the rule of answer 4: a card of 20 with 12 of margin around controls of 8.»* — col 6bis la scheda è 21, e i 12 di margine stanno dentro un bordo da 1 px: è il pulsante nell'angolo della finestra, storto di un pixel coi token di prima (la riga 1 della tabella in testa al compito). La specie di **E17**, **E22**, **E28**, **E36** ed **E50**; il compito non toccava il file. Trovata dal pre-controllo il 2026-09-27, cercando sulla copia col 6bis le cifre dei raggi nei commenti: `grep -rn 'card of 20' gui/src` rende la sola riga di `BaseDialog.vue`. ✅ **Corretta** nel commit che la scrive: una terza sostituzione nel Passo 7, e il file nella riga *Files*; il linter e le prove non cambiano, misurato sulla copia |
| **E67** | Nit — **Il controllo 23 del disegno giudica i ruoli *«col colore calcolato contro `readToken`»*, e il compito col colore calcolato di un token, `colourOf`:** è **D28**, col perché — `readToken` rende il testo del token, non un colore. La decisione c'era, e il disegno non la diceva: due case per una regola (gotcha **#68**), e la riga 6 delle regole di `CLAUDE.md`, *«vale anche per un DISEGNO»*. Trovata dal pre-controllo del 6bis il 2026-09-27. ✅ **Corretta** nel commit che la scrive: il richiamo datato nella riga 23 della tabella del prodotto del disegno, che rimanda a **D28** — e a **E65**, le parole nei loro ruoli |
| **E68** | Nit — **«Come si esegue un compito», regola 5 — lo sguardo del revisore nomina i compiti 4, 5, 6 e 8, e non il 6bis**, che cambia ciò che si vede: la fascia sulla pagina, i messaggi, le barre di scorrimento, i raggi. Trovata dal pre-controllo il 2026-09-27. ✅ **Corretta** nel commit che la scrive: *«4, 5, 6, 6bis e 8»* |
| **E69** | ⚠️ **Compito 6bis, Passi 16 e 17 — lo sguardo del proprietario e le celle della posizione non li ha chi fa il commit del codice:** il Passo 17 cominciava con *«Approvato lo sguardo»*, e l'implementatore, un subagente, committa **prima** della revisione e lo sguardo non l'ha; né l'ha il revisore. La specie di **E35** del compito 5 — e il compito 6 ha mostrato il costo del contrario: la riga 6 portata a `✅` nel commit del codice, e riportata a `⏳` dopo uno sguardo che non approvava. E il Passo 16 mandava a guardare la fascia *«a core spento»* **dopo** `harnessFake.deliverAll()`, quando non c'è più: misurato il 2026-09-27 dal pre-controllo sul server di sviluppo della copia, prima della consegna la fascia è `warn` con «Riprova», dopo è il timbro diverso, `stop` — le fixture consegnano anche `StaleBuild`. ✅ **Decisa dal coordinatore con `anthropic-skills:decision-principles`**, la forma di **E35**, reversibile e fuori dal merito: l'implementatore fa i Passi 1–15 e il commit del codice, senza le celle né il disegno; il Passo 16 lo fanno il proprietario e il coordinatore dopo la revisione e le sue cure, sul codice che resta; approvato, il coordinatore porta le righe 6 e 6bis a `✅` col verbale, e scrive nel disegno ciò che lo sguardo ha visto della barra, nel commit della chiusura. ✅ **Corretta** nel commit che la scrive: il testo dei Passi 16 e 17 |
| **E70** | ⚠️ **Compito 6bis, Passo 12 — la fascia del timbro diverso *sulla pagina* non la teneva nessuna prova:** la prova del ramo `stale`, *«stops the window when the core speaks another protocol…»* di `frame.test.ts`, guarda tono, titolo, timbro e l'assenza del pulsante, non il posto; e `frame.browser.test.ts` misura la sola fascia che aspetta. Tolti `on-page` o `class="band"` dal ramo `stale` di `Band.vue`, `frame.test.ts` resta `Tests  14 passed (14)` e `frame.browser.test.ts` `Tests  2 passed (2)` — misurato dalla revisione del 6bis (M-1) nel suo clone a `da7a522`, e dal coordinatore sull'albero per `on-page`. La (d) e la (b) del disegno mettono **tutte e due** le fasce sulla pagina; il controllo 23, alla lettera, misura nel browser *«la fascia»*, ed è tenuto: per questo è minore. ✅ **Curata** nel commit che scrive questa riga, col testo della revisione: nella prova *«stops the window…»* due attese, `band` fra le classi e `data-on-page` presente, col perché nel commento; il *Sostituisci* della prova nel Passo 12 allineato, e due righe nel Passo 15. Misurata dal coordinatore il 2026-09-27, macchina `Jays`: verde sul codice, `Tests  31 passed (31)` coi due file jsdom del Passo 12; senza `on-page` rossa la sola prova, `AssertionError: expected undefined to be defined`, `Tests  1 failed \| 13 passed (14)`; senza `class="band"`, `AssertionError: expected [ 'base-notice' ] to include 'band'`; e senza la cura la prima mutazione resta verde, `Tests  14 passed (14)`; `Band.vue` tornato ogni volta dalla copia salvata, `cmp` uguale. ⚠️ La strada più larga — la fascia `stop` misurata nel browser, consegnando `StaleBuild` in `frame.browser.test.ts` — è **registrata, non presa**: la può prendere il compito 8, che estende il file (**E63**) |
| **E71** | Minore — **D25 diceva *«Costo: nessuno»*, e un costo c'è:** nell'angolo il giudizio è a due lati, `Math.abs(inner - (outer - dx)) <= SLACK`, fuori dall'angolo a uno, il solo troppo tondo; con la soglia di *«nell'angolo»* a mezzo pixel un pezzo a **un** pixel dalla diagonale passa alla regola a un lato, e lì un raggio troppo piccolo non cade più. Misurato dalla revisione del 6bis (M-2) nel suo clone, con una prova usa-e-getta: un pezzo a 13/14 dall'angolo in basso a sinistra di un angolo di 20, raggio 4 — il concentrico sarebbe 7 —, accettato dalla sonda del commit, `{"near":1,"bad":[]}`, e rifiutato con la sola soglia dell'angolo a 1,5. **Latente**: le coppie che le prove incontrano — pagina kit, finestra, dock, gruppo staccato, fascia — sono 25 per tema, tutte con le due distanze uguali e scarto 0,00, e con la soglia dell'angolo a 1,5 le 56 prove del browser restano verdi. ✅ **Corretta** nel commit che scrive questa riga: la colonna del costo di **D25** lo dichiara; la regola e il codice non cambiano. Scartate, dal coordinatore coi cinque criteri, due costanti — la soglia dell'angolo di nuovo a 1,5 e lo scarto del raggio a 0,5 —: il giudizio a due lati misura il raggio contro `outer - dx` soltanto, e a mezzo pixel rifiuterebbe anche il raggio concentrico sull'altro asse, `outer - dy` — **dedotto** dalla formula, non misurato —; tenerlo chiederebbe una regola nuova per un caso che oggi non c'è. Scartata anche una prova che fissi il verdetto nuovo, come la revisione: terrebbe un'indulgenza, non una regola |
| **E72** | Nit — **Compito 6bis, Passo 12 — il titolo di una prova di Stato nominava la forma di prima:** *«says the core has not spoken, and shows no event row, before anything arrives»* in `modules.test.ts`, mentre la prova cerca ora il messaggio, `.base-notice` — la trappola 21 del disegno: le prove si riscrivono sul pezzo —; non falso alla lettera, perché il messaggio porta la classe `event`. Trovata dalla revisione del 6bis (N-1). ✅ **Curata** nel commit che scrive questa riga: *«… and shows no message, before anything arrives»*; il *Trova* e il *Sostituisci* della prima sostituzione di `modules.test.ts` nel Passo 12 prendono anche il titolo. Nessun esito cambia: `Tests  31 passed (31)` coi due file jsdom del Passo 12 |
| **E73** | Nit — **Compito 8, al suo pre-controllo — due cose che `frame.browser.test.ts` porta dal 6bis:** **(a)** l'aiutante `computed(property, token)` è identico, riga per riga, a quello di `dock.browser.test.ts`, e `colourOf` di `kit.browser.test.ts` è lo stesso oracolo per `color`: la ragione di **D29**, *«un secondo modo di fare la stessa cosa»*, vale anche per un aiutante; **(b)** il suo `afterEach` smonta l'app, ma `Frame.vue` non dispone il dock — il suo `onUnmounted` toglie solo l'ascoltatore della tastiera —, mentre `dock.browser.test.ts` dispone ogni dock e dice perché: `dockview-core` 8.3.1 tiene i gruppi galleggianti della pagina in una lista di modulo. Oggi innocuo, perché in quel file nessun gruppo galleggia — **dedotto** leggendo il codice, non misurato. Trovate dalla revisione del 6bis (N-2), e rilette dal coordinatore sul codice. ⏳ **Al pre-controllo del compito 8**, che estende il file e ne riusa gli aiutanti (**E63**): portarli in `gui/src/testing/` o dichiarare la copia, e disporre il dock se il compito vi stacca un gruppo |
| **E74** | ⚠️ **Di prima del 6bis — l'anello del fuoco di una scatola che scorre lo taglia la sua scheda:** nella Home a 1440 × 900 la scatola di Stato scorre, e Chrome la mette nella sequenza del Tab — uno *scroller* senza nulla di focalizzabile dentro —; col Tab ci si arriva, e `:focus-visible` disegna il contorno `solid 2px` a `2px` di distanza, ma la scheda ne taglia i lati e il fondo, e resta la sola riga in alto, dopo la linguetta. La riga «focus» della (a) del disegno lo vuole *«visibile (2.4.7) e non nascosto (2.4.11)»*. Trovato dalla revisione del 6bis (P-1) e misurato **uguale** a `9579ca0`, nei due temi: non è del 6bis; la schermata riguardata dal coordinatore. Detto al proprietario prima dello sguardo del Passo 16, che non l'ha commentato: lo sguardo non l'ha approvato. ⏳ **Aperta**: la forma della cura è aspetto — il contorno **dentro** la scatola che scorre, o uno spazio attorno che la scheda non tagli — e la decide il proprietario (controllo 15); la sonda nel browser la può portare il compito 8, che monta la cornice intera ✅ **RICHIAMO DEL 2026-09-27 — chiusa dal compito 8:** la forma l'ha decisa il proprietario, **A**, l'anello **dentro** la scatola che scorre (**E100**), curata in `ac56b11` con la sua prova nel browser, e lo sguardo del Passo 10 l'ha approvata |
| **E75** | ⚠️ **Compito 7, Passo 5 — dopo un pacchetto che non si legge, la vista col nome restava aperta, e la mossa dopo non andava da nessuna parte:** `receive` rimetteva `openNamed` solo dentro `if (saved.value !== null)`. Quando il core risponde `Nothing` o `Unavailable` — o con un pacchetto che `unpack` rifiuta — `saved` diventa `null` e il nome resta: il dock mostra la vista di sempre, perché `apply` ricade su `VIEWS[view]`; il negozio dice ancora «Revisione», e la barra del compito 8 ne mostrerebbe il nome; e il `settle` dopo scrive nella vista col nome, che il pacchetto non ha più, quindi la mossa non va in `layouts` né in `named` — persa in silenzio. È la strada di una scrittura che non attecchisce al primo avvio — il core risponde con ciò che tiene (decisione 13), cioè niente — o di un archivio che non risponde. Misurato il 2026-09-27 dal pre-controllo sulla copia, un `git worktree` di `1c168a1` nello scratchpad, macchina `Jays`, col compito rifatto dal testo del piano, con una prova usa-e-getta: dopo `Unavailable`, e dopo `Nothing`, `openNamed` resta `'Revisione'`, e la mossa spedisce `{"layouts":{},"view":"home","openNamed":"Revisione","named":[]}`; con la cura, `null` e `{"layouts":{"home":{…}},"view":"home"}`. ✅ **Corretta** nel commit che la scrive, decisa coi cinque criteri — la radice è lo stato del negozio, non il dock, che ricade già —: in `receive` il nome si rilegge dal pacchetto **sempre**, `openNamed.value = saved.value?.openNamed ?? null;`, e la riga di `view` resta quella di oggi; nel Passo 2 una sesta prova del negozio, *«closes the named view when the core holds no package it can read…»*, rossa sul codice dettato prima, `expected 'Revisione' to be null`; gli Atteso dei Passi 2 e 5, e una riga nel Passo 8 |
| **E76** | Nit — **Compito 7, la riga 3 della tabella in testa e D13 — *«nessuna prova monta `Frame.vue`»* è falso dal 6bis:** `frame.browser.test.ts` monta `App.vue`, e con lei la cornice; il `grep -rln 'Frame.vue'` del Passo 1 resta vuoto perché il file importa `App.vue`. Nessuna prova però preme la barra: con `layout.view = view;` al posto di `layout.showView(view);` la suite intera resta verde, `Tests  212 passed \| 1 skipped (213)` — misurato il 2026-09-27 dal pre-controllo sulla copia, col testo corretto. Lo stesso vale per `aria-current` in `ViewBar.vue`, che guarda il solo `layout.view` — dedotto leggendo il codice, non misurato. ✅ **Dichiarata, non curata**, decisa coi cinque criteri: fra il compito 7 e l'8 nessuna vista col nome si apre dall'interfaccia, e il compito 8 riscrive `Frame.vue` e `ViewBar.vue` per intero — la riga vive un compito solo, e una sonda nel browser per lei sarebbe sfoggio. ✅ **Corretta** nel commit che la scrive: la riga 3 della tabella del compito 7, e il richiamo datato in **D13**. ⏳ **Al pre-controllo del compito 8**: la sua prova *«shows the view of the card chosen and closes…»* sceglie Lavoro **prima** di aprire la vista col nome, e non tiene che scegliere una delle tre chiuda la vista col nome — nella Panoramica, il ruolo che qui ha `Frame.switchTo` |
| **E77** | Nit — **Compito 7, Passo 8 — tre righe facevano cadere più prove di quelle che l'Atteso nomina**, la specie di **E12** ed **E49**. Misurato il 2026-09-27 dal pre-controllo sulla copia, ogni violazione sulla suite intera, i due progetti: *«ciò che sta oltre»* anche *«splits its own group on that side when nothing lies there»* di `keys.test.ts`, `expected 'moved' to be 'split'` — dal Passo 3 `moveActive` passa da `nearest` —, quattro prove; *«il nome aperto che arriva»* anche la prova del dock di `frame.test.ts`, `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]`, tre — quattro con la prova di **E75** —; *«`showView`»* anche il dock, `expected [ 'status' ] to deeply equal [ 'activity', 'costs', …(5) ]`, due. Le altre undici righe cadono come dettano, e ogni file è tornato uguale dalla copia salvata. Nessun codice cambia. ✅ **Corretta** nel commit che la scrive: le tre righe nominano le prove in più, e l'intestazione della colonna la data della misura |
| **E78** | Nit — **Compito 7, Passo 6 — la testa di `createDock` in `gui/src/frame/dock.ts`, che il compito rende falsa** (gotcha **#58**): *«in the two ways the store changes under it: the bar writes `layout.view`, and the core sends a package…»* — col compito i modi sono tre, con la vista col nome che si apre o si chiude, e il `watch` subito sotto lo dice; e dal compito 8 non è più la barra a scegliere la vista, ma la Panoramica. Nessun compito toccava la frase: sulla copia coi compiti 7 e 8 applicati, `grep -n 'two ways' gui/src/frame/dock.ts` rende la riga. La specie di **E17**, **E22**, **E28**, **E36**, **E50** ed **E66**. Trovata dal pre-controllo il 2026-09-27. ✅ **Corretta** nel commit che la scrive: una terza sostituzione nel Passo 6, *«in the three ways … one of the three views is shown (`showView`), a named view opens or closes (`openNamed` …)»*, che non nomina chi mostra la vista e resta vera dopo l'8; le prove e il linter non cambiano, misurato sulla copia |
| **E79** | Nit — **Compito 7, Passo 9 — la colonna Commit della riga 6 c'è già:** l'ha scritta la chiusura del 6bis, `1c168a1` — `c1102fc` con la cura `ca83dac` —, insieme a quella della riga 6bis, perché il 6bis non toccava le celle (**E69**); e il Passo 9 la faceva scrivere con *«l'hash del compito 6»* (R1-16). È la quarta domanda del pre-controllo — una parte del passo è già eseguita —, e riscriverla col solo hash perderebbe la cura. Trovata dal pre-controllo il 2026-09-27. ✅ **Corretta** nel commit che la scrive: il Passo 9 dice che la riga 6 porta già la colonna, e che non si tocca; quella della riga 7 la scrive il compito 8, com'è il suo Passo 11 |
| **E80** | Nit — **Compito 7 — il pezzo JavaScript non lo misurava:** la riga **N-2 di E187** delle voci aperte nominava *«il compito 1, il 5, il 6 e l'8»*, e il compito 7 cambia la SPA — `layout.ts`, `dock.ts`, `Frame.vue`, `moveActive.ts` con `nearest.ts` —: misurato il 2026-09-27 dal pre-controllo sulla copia, da `691.82 kB`, compresso `210.77 kB`, a `693.02 kB`, `211.20 kB`. Anche il 6bis lo misura, e la riga non lo diceva. ✅ **Corretta** nel commit che la scrive: la baseline nel Passo 1, la riga nel commit dai Passi 7 e 9, e nella riga di N-2 la regola — ogni compito che cambia la SPA —, coi compiti che la seguono |
| **E81** | ⚠️ **Compito 8, al suo pre-controllo — *«Salva questa vista»* prima del benvenuto del core perde le disposizioni salvate:** su un negozio che non ha ancora ricevuto il pacchetto, `saveNamed` costruisce il suo da `layouts: {}`, e il core custodisce quello — le tre disposizioni, il tema e le altre viste col nome, persi in silenzio. È la finestra di **E40**, che il compito 5 ha chiuso per il tema spegnendone il gruppo finché `layout.arrivals` è zero; la Panoramica del compito 8 non guarda `arrivals` — sul testo del compito `grep -c arrivals` rende 0 —, e F3 la apre quando non c'è una conferma né il cassetto, anche prima del benvenuto — dedotto dal testo del suo `Frame.vue`. Misurato il 2026-09-27 dal pre-controllo del compito 7 sulla copia, con una prova usa-e-getta: con `arrivals` a 0, `saveNamed` rende `"saved"` e spedisce `{"layouts":{},"view":"home","openNamed":"Revisione","named":[…]}`. ⏳ **Al pre-controllo del compito 8**: la scheda *«Salva questa vista»* spenta finché `layout.arrivals` è zero, come il gruppo del tema (**E40**, strada A), con una prova vista rossa; la domanda di classe — ogni scrittura prima del benvenuto, compreso il `settle` del dock — resta del proprietario, **registrata e non presa** (**E40**) |
| **E82** | ⚠️ **Compito 7, Passo 2 — le prove delle viste col nome ne usavano UNA sola, e tre righe che scelgono QUALE non le teneva nessuna:** con una vista col nome sola, *«quella»* non si distingue da *«tutte»* né da *«la prima»*, e ciascuna delle tre righe perderebbe una vista in silenzio, la perdita che **D4** vuole evitare. Misurato dalla revisione del compito 7 (M-1) nel suo clone a `0417c9c`, la suite intera verde, `Tests  212 passed \| 1 skipped (213)`, sotto ciascuna mutazione: in `settle` la mossa scritta in **ogni** vista col nome; in `saveNamed` la lista ridotta alla nuova; in `apply` la **prima** della lista, qualunque sia il nome. La specie di **E29**. ✅ **Curata** nel commit che scrive questa riga dentro le prove che c'erano, così i conti e i rossi del Passo 2 non cambiano: nella prova della mossa due viste col nome, l'aperta **seconda**; in quella di `saveNamed` una vista salvata prima, che resta; nella prova del dock due viste, aperta la seconda. Misurata dal coordinatore il 2026-09-27, macchina `Jays`, ogni mutazione da sola sulla suite intera, e il file tornato dalla copia, `cmp` uguale: verdi sul codice; rossa una prova ciascuna — la mossa in tutte e la mossa nella prima `expected { view: 'home', …(3) } to deeply equal { view: 'home', …(3) }`, la lista ridotta `expected { view: 'home', layouts: {}, …(2) } to deeply equal { view: 'home', layouts: {}, …(2) }`, il dock sulla prima `expected [ 'costs' ] to deeply equal [ 'status' ]`. Quattro righe nel Passo 8 |
| **E83** | Minore — **Compito 7, Passo 2 — la cura di E75 era tenuta sul solo ramo `null`:** il caso che la regola nomina per primo, la decisione 13, è il pacchetto **vecchio**, leggibile e senza il nome, e la prova di **E75** consegnava `Nothing`. Misurato dalla revisione (M-2): con `openNamed.value = saved.value === null ? null : (saved.value.openNamed ?? openNamed.value);` — il nome tenuto quando il pacchetto tace — la suite intera resta verde, e la mossa dopo finisce nel nulla, com'era **E75**. ✅ **Curata** nel commit che scrive questa riga: la prova di **E75** rifà la scrittura che non attecchisce con le due risposte del core, il pacchetto senza il nome e nessun pacchetto, e il titolo lo dice. Misurata dal coordinatore: verde sul codice; rossa con quella mutazione, `expected 'Revisione' to be null`, e rossa ancora con la riga *«ciò che arriva illeggibile»* del Passo 8, lo stesso messaggio. Una riga nel Passo 8 |
| **E84** | Minore — **Compito 7, Passo 2 — *«tiles each view that ships without a gap or an overlap»* non guardava le sovrapposizioni:** la somma delle aree e i bordi non escludono una sovrapposizione con un buco della stessa area. Misurato dalla revisione (M-3): in `schematic.ts` l'origine del ramo orizzontale dimenticata, `x: offset * box.width`, lascia verde la suite intera, e in Lavoro `steps` e `sensors` finiscono sopra `scope`. ✅ **Curata** nel commit che scrive questa riga col testo della revisione: nella stessa prova, per ogni coppia di tessere l'area dell'intersezione sotto `1e-9`, col perché nel commento. Misurata dal coordinatore: verde sul codice; rossa con la mutazione, `work: scope over steps: expected 0.07846999999999998 to be less than 1e-9`. Una riga nel Passo 8 |
| **E85** | ⚠️ **Compito 7, Passo 4 — lo schema non vedeva un gruppo INGRANDITO, e la miniatura diceva il falso:** la presa grande ha *ingrandisci* (`BigTab.ts`), e `dockview-core` 8.3.1 scrive la griglia com'è senza l'ingrandimento, più `grid.maximizedNode`, gli indici dalla radice fino a quel gruppo — `serialize()` in `main.esm.mjs`, letto dal coordinatore il 2026-09-27 —, che il tipo pubblico `SerializedDockview` non dichiara; `fromJSON` riapre la disposizione col gruppo ingrandito. Misurato dalla revisione (M-4) sotto jsdom: dopo `maximize()` il pacchetto porta il nodo, e `schematic` rendeva sette tessere dove la vista si apre con una. La risposta 19 vuole miniature che *«dicono sempre il vero»*, e **D5** pensava ai galleggianti. Due strade: **A**, lo schema segue gli indici e disegna quel gruppo solo, sul quadrato intero; **B**, dichiararlo come i galleggianti, cioè scrivere nel codice il contrario della risposta 19. ✅ **Presa la A**, decisa dal coordinatore coi cinque criteri, come **E30**: realizza il merito approvato in poche righe, e come si **disegni** la miniatura resta dello sguardo del compito 8. ✅ **Curata** nel commit che scrive questa riga: in `schematic.ts` il percorso di `maximizedNode`, vuoto quando non c'è — nessun ramo nuovo —, col perché nel commento e un limite dichiarato: un nodo nascosto con `setVisible` si scrive `visible: false` con la misura che aveva, e nessun sorgente lo chiama (`grep -rn 'setVisible(' gui/src` vuoto); in `schematic.test.ts` una terza prova da un `toJSON()` vero dopo `maximize()`, con la Pinia attiva: i pannelli veri leggono i negozi, e senza di lei `fromJSON` fallisce e torna indietro — la prima corsa rossa della prova lo era per **questa** ragione, e l'ha rifatta —; e il richiamo in **D5**. Misurata dal coordinatore: rossa sullo `schematic.ts` del compito, `expected [ { x: +0, y: +0, …(4) }, …(6) ] to deeply equal [ { x: +0, y: +0, width: 1, …(3) } ]`, verde con la cura; la suite intera `Tests  213 passed \| 1 skipped (214)`. L'Atteso del Passo 4 dice tre prove; una riga nel Passo 8 |
| **E86** | Nit — **Compito 7, Passo 6 — un commento di `gui/src/frame/Frame.vue` che il compito rende falso** (gotcha **#58**): *«The mapping and the geometry live in `moveActive.ts`; this is only the wire.»* — dal Passo 3 la geometria vive in `nearest.ts`. La specie di **E78**. Trovata dalla revisione (N-1). ✅ **Curata** nel commit che scrive questa riga: una seconda sostituzione in `Frame.vue` nel Passo 6; nessuna prova cambia |
| **E87** | Nit — **Compito 7, Passo 3 — due commenti davano per presente il chiamante del compito 8:** in `moveActive.ts` *«shared with the overview's grid since the design system»* e in `nearest.ts` *«and of the arrows in the overview's grid»*, mentre `nearest` lo chiama il solo `moveActive.ts` fino al compito 8. La specie di **E22 (b)**. Trovata dalla revisione (N-2). ✅ **Curata** nel commit che scrive questa riga: *«from task 8»* nei due commenti, nei recinti del Passo 3 |
| **E88** | Nit — **Compito 7, Passo 2 — le guardie di `readNamed` su una voce `null` e su una disposizione `null` non le teneva nessuna prova:** tolta la prima, una voce `null` fa lanciare la destrutturazione dentro il `try` di `unpack`, che rende `null` — **tutto** il pacchetto perso, disposizioni e tema —; tolta la seconda, resta una vista col nome senza disposizione. Misurato dalla revisione (N-3): la suite verde sotto ciascuna. ✅ **Curata** nel commit che scrive questa riga: nella lista di *«drops what it cannot read…»* anche `null` e `{ name: "Nulla", layout: null }`, col perché nel commento. Misurata dal coordinatore: verde sul codice; rossa senza la prima guardia, `expected null to deeply equal { view: 'home', …(2) }`, e senza la seconda, `expected { view: 'home', …(2) } to deeply equal { view: 'home', …(2) }`. Due righe nel Passo 8 |
| **E89** | Nit — **Compito 7 — due rami difensivi senza sonda:** in `schematic.ts` la divisione in parti uguali quando le misure di un ramo sommano a zero — nessuna disposizione vera le ha a zero —, e in `apply` il ritorno alla vista di sempre per un nome che il pacchetto non ha — dopo **E75** la strada c'è solo scrivendo in `openNamed` un nome che la lista non ha, e il compito 8 scrive solo nomi della lista. Misurato dalla revisione (N-4): verde senza l'uno o l'altro. ✅ **Dichiarati, non sondati**, decisi coi cinque criteri come **E76**: una prova terrebbe un caso che nessuno produce. Due righe dopo la tabella del Passo 8; la porta di `openNamed` è di **E90** |
| **E90** | ⚠️ **Compito 8, al suo pre-controllo — tre cose che la revisione del compito 7 ha trovato usando da fuori le interfacce che il compito 8 consuma:** **(a)** una disposizione malformata rompe **tutta** la Panoramica: `unpack` tiene come disposizione ogni oggetto, `schematic({})` lancia `TypeError: Cannot read properties of undefined (reading 'root')` — e, dopo **E85**, così fa un `maximizedNode` i cui indici non portano a una foglia —, e il `computed` delle schede chiama `schematic` su ogni vista insieme; il dock lancia già sulla stessa, `root must be of type branch`, ma solo quando la mostra: la classe è della parte 2; **(b)** `openNamed` si scrive senza guardia: con un nome che la lista non ha il dock ricade sulla vista di sempre, e il `settle` dopo perde la mossa in silenzio — la malattia di **E75** da un'altra porta; **(c)** `nearest` esclude la scheda di partenza per geometria, non per identità: con rettangoli nulli, come sotto jsdom, rende la scheda stessa, e il titolo *«never the card it starts from»* vale per rettangoli più larghi di un pixel. **(a)** misurato dal consumatore della revisione; **(b)** e **(c)** **dedotti** dal testo del compito 8 — che scrive solo nomi della lista, e il cui `onArrow` toglie `from` dai candidati come `moveActive` —, non misurati sul suo codice, che non esiste ancora. ⏳ **Al pre-controllo del compito 8**: per **(a)** una miniatura che non si legge disegnata vuota, con la sua prova; per **(b)** una prova che lo tenga, o un `showNamed(name)` che rifiuti un nome ignoto, simmetrico a `showView`; per **(c)** le sue prove sotto jsdom |
| **E91** | Nit — **Compito 8, Passo 1 — `grep -rn 'F3' gui/src` non è vuoto dal compito 1:** `themes.css` copia la tavola, e `--ref-neutral-94: #F3EEE6` contiene `F3`. La sonda cerca un tasto e trova un colore — l'Atteso *«`F3` in nessun file»* era falso a ogni esecuzione. Misurato il 2026-09-27 dal pre-controllo del compito 8 sulla copia, un `git worktree` di `f3206c7` nello scratchpad, macchina `Jays`. E il Passo 1 non misurava il pezzo JavaScript, che il Passo 8 porta nel commit accanto alla baseline: la regola di **N-2**, dal compito 7 (**E80**). ✅ **Corretta** nel commit che la scrive: `grep -rnw`, che non prende `F3` dentro `#F3EEE6` — vuoto, misurato —; e la riga del *build* nel Passo 1, `693.02 kB`, compresso `211.20 kB`, a `f3206c7`, gli stessi del cancello d'apertura |
| **E92** | ⚠️ **Compito 8, Passo 2 — E63 ed E73.** La *Crea* di `frame.browser.test.ts` rifiutava, *«exists already»*: il file c'è dal 6bis (**D29**). ✅ **Corretta** nel commit che la scrive: il compito lo **estende** con quattro sostituzioni — gli import; l'aiutante `computed` che esce, e al suo posto le due porte della Panoramica; la prova della fascia seguita da quelle del compito —, che applicate al file di `f3206c7` rendono quello della copia. **E73 (a)**, decisa dal coordinatore coi cinque criteri — reversibile, poche righe, solo prove, fuori dal merito —: l'oracolo `computed(property, token)`, identico riga per riga in `dock.browser.test.ts` e in `frame.browser.test.ts`, e `colourOf` di `kit.browser.test.ts`, lo stesso per `color`, vivono **una volta** in `testing/probes.ts`, com'è per `contrastJudged` in `testing/axe.ts` nello stesso compito; `colourOf` resta, e passa da lì. Scartata la copia dichiarata: un secondo modo della stessa cosa, la ragione di **D29**. **E73 (b)**: in quel file il compito non stacca un gruppo — `grep -c 'floating'` rende 0 sulla copia —, e il dock resta com'è. Misurato: le prove del dock, della pagina kit e della cornice verdi con l'oracolo spostato |
| **E93** | ⚠️ **Compiti 8 — i conti di E64, dedotti, misurati.** Il compito com'era scritto, rifatto sulla copia: al Passo 3 le prove dei tre file del browser sono **quarantasette**, non trentasette — il dock ne ha **sedici** dalle cure del compito 6 (**E44**…**E58**), che **E64** non contava —, e ne cadono **nove**, coi messaggi scritti; al Passo 7 quarantasette su quarantasette; al Passo 8 `Tests  229 passed \| 1 skipped (230)`, sedici in più, e cinque corse uguali. Col testo corretto: al Passo 3 sotto jsdom i due import e `layout.showNamed is not a function`, nel browser **dieci** rosse su **quarantanove** — la nuova è **E100** —; al Passo 7 `Tests  61 passed (61)` nei quattro file jsdom e `Tests  49 passed (49)`; al Passo 8 `Tests  234 passed \| 1 skipped (235)`, **ventuno** in più, e cinque corse su cinque, 234 passate ciascuna; il pezzo `698.14 kB`, compresso `212.98 kB`. ✅ **Corretta** nel commit che la scrive: gli Atteso dei Passi 3, 7 e 8 coi numeri misurati, e `stores.test.ts` nei comandi dei Passi 3 e 7 |
| **E94** | Nit — **Compito 8, Passo 6 — la riscrittura di `Frame.vue` rimetteva il commento che E86 ha corretto:** *«The mapping and the geometry live in `moveActive.ts`»*, falso dal Passo 3 del compito 7, che ha portato la geometria in `nearest.ts` (gotcha **#58**; la riga 5 di `CLAUDE.md`: il compito, scritto prima, si legge contro il codice di adesso). Trovato dal pre-controllo, confrontando la riscrittura col file di `f3206c7`. ✅ **Corretta** nel commit che la scrive: nel recinto il commento di oggi |
| **E95** | Nit — **Compito 8, Passo 9 — sulla suite intera tre righe fanno cadere più prove di quelle nominate, e una non dava il suo rosso**, la specie di **E12**, **E49** ed **E77**. Misurato il 2026-09-27, una violazione per volta sulla copia, ogni file tornato dalla copia salvata, `cmp` uguale: *«la carta scelta»* anche la prova delle frecce — Invio lascia aperta la Panoramica —; *«il pulsante della striscia»* anche la prova del cassetto nei due temi, `expected null not to be null`; *«la Panoramica aperta»* anche la Panoramica disegnata nei due temi, lo stesso messaggio di `axe`. E *«le parole delle viste»*: tolta la riga di `"compact"`, quella di `"work"` resta con la virgola, il JSON non si legge — `trailing comma at line 5 column 3` — e **dodici** file di prova non si caricano. Le altre sedici cadono come dettano. ✅ **Corretta** nel commit che la scrive: le tre righe nominano le prove in più; la violazione toglie anche la virgola, e dà il rosso scritto; l'intestazione della colonna la data della misura |
| **E96** | ⚠️ **Compito 8, al suo pre-controllo — le sonde che l'errata gli lasciava: E38, E39, E43, E57 ed E70, e la domanda di E44 con E53.** Le cure c'erano tutte, e le sonde le tengono. Misurato con una diagnostica nel Chrome installato, 1440 × 900: il cassetto aperto dalla tastiera dà il fuoco a «Chiudi» a y 855–887, dentro il foglio a 334–900 (**E38**); il segnaposto della ricerca 315 px nei 334 della casella (**E39**); col core salutato la fascia esce, e con un altro timbro la fascia `stop` entra sulla pagina a 12 px dai lati, e la pagina non sborda — senza `overflow: clip` sborda di 50 px al primo fotogramma (**E43**, **E70**); la barra senza fondo né riga (**E57**). ✅ **Corretta** nel commit che la scrive, in `frame.browser.test.ts`: la prova della fascia prende l'altra fascia e la pagina fotogramma per fotogramma — una direzione nuova in una prova che c'è —; quella del cassetto l'apertura dalla tastiera, col fuoco dentro il foglio; una prova nuova la barra e il segnaposto; e cinque righe nel Passo 9, ciascuna rossa. **E44** ed **E53**: la Panoramica **non scorre** a 1440 × 900 con le tre viste e *«Salva questa vista»* — `scrollHeight` 900 su 900 — e le sue carte non hanno un antenato tondo: la finestra intera ha gli angoli dritti (la (d)). Scorra o no, lo scorrimento non toglie nulla al giudizio: si giudicano la miniatura nella carta, la tessera nella miniatura e il campo nel riquadro del nome, e fra loro nulla scorre. Nessuna cura |
| **E97** | Nit — **Compito 8, Passo 2 — E76 ed E82.** La prova *«shows the view of the card chosen…»* sceglieva Lavoro **prima** di aprire la vista col nome, e non teneva che una delle tre la chiudesse — il ruolo che aveva `Frame.switchTo`, ora della Panoramica —; e usava **una** vista col nome, dove *«quella scelta»* e *«la prima»* sono la stessa carta. ✅ **Corretta** nel commit che la scrive: la prova porta **due** viste col nome, sceglie la **seconda**, poi riapre la Panoramica e sceglie Home, e la vista col nome si chiude; due righe nel Passo 9 — con `layout.view = view` al posto di `showView`, `expected 'Lettura' to be null`; con la prima della lista, `expected 'Revisione' to be 'Lettura'` |
| **E98** | ⚠️ **Compito 8 — E81, la finestra prima del benvenuto.** ✅ **Corretta** nel commit che la scrive, la strada **A** di **E40**, decisa dal coordinatore coi cinque criteri: *«Salva questa vista»* è **spenta finché `layout.arrivals` è zero**, col perché nel commento di `save`; una prova sotto jsdom, vista rossa sul codice del compito, `expected false to be true`, verde con la cura; e le tre prove che arrivano a quella carta — il nome sotto jsdom, la Panoramica disegnata e le frecce nel browser — consegnano prima una risposta del core, `{ state: "Nothing" }`. Una riga nel Passo 9. La domanda di classe — ogni scrittura prima del benvenuto, compreso il `settle` del dock — resta del proprietario, **registrata e non presa** (**E40**) |
| **E99** | ⚠️ **Compito 8 — E90.** ✅ **(a) Corretta**: una disposizione che non si legge la Panoramica la disegna **vuota**, e non cade tutta — `drawn` prende ciò che `schematic` lancia; la prova, vista rossa sul codice del compito, `TypeError: Cannot destructure property 'maximizedNode' of 'layout.grid' as it is undefined.`, verde con la cura. La classe, `unpack` che tiene come disposizione ogni oggetto, resta della parte 2. ✅ **(b) Corretta**, decisa dal coordinatore coi cinque criteri: `showNamed(name)` nel negozio, la gemella di `showView`, apre una vista col nome solo se la lista la tiene, e altrimenti rifiuta e non cambia nulla; la Panoramica le apre solo da lì. La regola vive dov'è lo stato, come **D13**, e una prova della Panoramica non la terrebbe: la Panoramica scrive solo nomi della lista. Una prova del negozio, vista rossa, `layout.showNamed is not a function`, e una riga nel Passo 9. **(c) Dichiarata**: `onArrow` toglie la carta di partenza **per identità** prima di `nearest`, e nessuna prova sotto jsdom preme le frecce — letto nel testo; il limite di `nearest` coi rettangoli nulli resta quello che **E90** dice. Due righe nel Passo 9 |
| **E100** | ⚠️ **Compito 8 — E74, decisa dal proprietario il 2026-09-27 — A: l'anello del fuoco di un pannello che scorre si disegna DENTRO la scatola**, nel padding che ogni pannello ha, contro **B**, uno spazio attorno che la scheda non tagli. Misurato prima di chiedere, con una diagnostica nel Chrome installato: nella Home a 1440 × 900 Stato e Impostazioni scorrono, il Tab li raggiunge — `section.status`, `section.settings` — e stanno a 1 px dal bordo della scheda, che ha `overflow: hidden` e il raggio 21: l'anello, 2 px a 2 px fuori, restava solo in alto. ✅ **Corretta** nel commit che la scrive: nel Passo 7 una regola in `tokens/dock.css`, `.dockview-theme-harness .panel > :focus-visible`, con l'`outline-offset` a meno la distanza e lo spessore dell'anello — lo stesso anello, rovesciato dentro — e gli angoli di sotto della scheda meno il suo bordo, concentrici (risposta 4), perché un anello dritto in un angolo tondo lì si taglia; una prova nel browser che col Tab arriva al pannello che scorre e misura l'anello dentro la scheda e i suoi angoli, vista rossa, `expected 9 to be greater than or equal to 13`; due righe nel Passo 9. Il costo, detto al proprietario prima della scelta: una seconda forma dell'anello, dentro, nei soli pannelli del dock. Il richiamo datato nella riga «focus» della (a) del [disegno](../specs/2026-09-22-design-system-design.md) |
| **E101** | ⚠️ **Compito 8, Passi 10 e 11 — lo sguardo e le celle, la specie di E35 ed E69:** il Passo 10 non diceva chi guarda, e il Passo 11 portava la riga 8 a `✅` nel commit dell'implementatore, che committa **prima** della revisione e dello sguardo; e la colonna Commit della riga 7 la voleva *«con l'hash del compito 7»*, che perderebbe la cura (**E79**). ✅ **Decisa dal coordinatore** con `anthropic-skills:decision-principles`, la forma di **E35** ed **E69**, reversibile e fuori dal merito, e **corretta** nel commit che la scrive: l'implementatore fa i Passi 1–9 e il commit del codice, **senza** celle; il Passo 10 lo fanno il proprietario e il coordinatore dopo la revisione e le sue cure, e nomina ciò che solo lì si vede — una vista con un gruppo ingrandito (**E85**: *come* si disegni la miniatura resta di questo sguardo), *«Salva questa vista»* spenta prima della risposta del core, il cassetto dalla tastiera, l'anello dentro —; nel commit della chiusura il coordinatore porta la riga 8 a `✅` col verbale e scrive la colonna Commit della riga 7, `0417c9c` con la cura `a606f33` |
| **E102** | Nit — **Compito 8, Passo 7 — il comando sotto jsdom non nominava `src/stores/stores.test.ts`, e l'Atteso lo contava:** **E93** lo dà fatto nei comandi dei Passi 3 e 7, e il recinto del Passo 7 non l'aveva ricevuto. Trovata dall'implementatore del compito 8 (il suo rapporto, §7) e confermata dalla revisione (§2.3), che l'hanno misurata tutti e due: com'era scritto `Test Files  3 passed (3)` e `Tests  40 passed (40)`, coi quattro file `Test Files  4 passed (4)` e `Tests  61 passed (61)`, la prova di `showNamed` compresa. ✅ **Decisa dal coordinatore** coi cinque criteri: il comando coi quattro file, nell'ordine del Passo 3 — la prova vista rossa al Passo 3 si vede verde al Passo 7, e **E93** torna vera —, contro l'Atteso portato a tre file e a 40, che lascerebbe falsa **E93** e senza verde il codice del Passo 5. ✅ **Curata** nel commit che scrive questa riga: la riga del comando cambia senza spostare righe; e con la prova di **E104** l'Atteso è `Tests  62 passed (62)`, misurato il 2026-09-27, macchina `Jays`, col testo curato applicato passo per passo a un clone di `bc6e94d` |
| **E103** | ⚠️ **Compito 8, Passo 5 — la miniatura di Compatta diceva il falso, e il costo scritto in D18 anche:** `compact.json` portava le due foglie a 500 su 1000, perché il generatore scriveva la vista prima che `dockview` applicasse alla striscia i suoi 56 px — lo schermo la stringe, lo schema no. Trovata dalla revisione del compito 8 (I-1): sullo schermo la knowledge base al 91,8 % del dock, nella miniatura al 50 %, e **D18** dichiarava *«a tutta pagina»*. ✅ **Decisa dal coordinatore** coi cinque criteri: la strada **A** della revisione, per **tutte e tre** le viste — la radice è il generatore, non il file —, contro **B**, `schematic` che rispetta i vincoli di una foglia, che complica lo schema per un caso solo e lascia il file spedito a dire il falso a chi lo legge. ✅ **Curata** nel commit che scrive questa riga: in `generate-views.test.ts` la `write` ridispone la griglia alla sua misura, `api.layout(api.width, api.height, true)`, prima di scriverla, col perché nel commento, e `compact.json` rigenerato. Misurato il 2026-09-27, macchina `Jays`: il generatore di prima riscrive i tre file uguali byte per byte; quello curato, `(cd gui && REGENERATE_VIEWS=1 npx vitest run src/panels/views/generate-views.test.ts)`, cambia la sola Compatta, a `[944, 56]` — Home e Lavoro **uguali**, la deduzione della revisione ora misurata. La prova, dentro quella delle miniature: per **ogni** vista spedita le tessere arrivano in basso almeno dove comincia la riga della striscia, la sua altezza sulla griglia; rossa col file di prima, `compact: expected 50 to be greater than or equal to 94.399999`, verde col file rigenerato; tolta la riga del generatore e rigenerato, `compact.json` torna a 500 e 500 e la prova di nuovo rossa, lo stesso messaggio, e i file tornati dalla copia, `cmp` uguale. Nel testo le due operazioni nel Passo 5 — il `JSON` come *Riscrivi*, la copia di ciò che il generatore scrive —; il richiamo datato in **D18**; una riga nel Passo 9 |
| **E104** | ⚠️ **Di prima del compito 8 — ingrandire e ripristinare un gruppo non arrivavano al pacchetto:** in `dockview-core` 8.3.1 l'ingrandimento non è un `onDidLayoutChange`: `maximizeGroup` salvava solo attraverso il cambio del gruppo attivo che fa, quando lo fa, ed `exitMaximizedGroup` mai — ripristinato, il gruppo restava ingrandito nel pacchetto, e la miniatura della vista sullo schermo lo contraddiceva. Trovata dalla revisione del compito 8 (I-2), misurata sotto jsdom e nel browser; la Panoramica la rende visibile, e il Passo 10 la guarda (**E85**). ✅ **Decisa dal coordinatore** coi cinque criteri: in `createDock` il corpo del `settle` diventa una funzione, `settleIfMoved`, che ascoltano `onDidLayoutChange`, `onDidMaximizedGroupChange` e la chiusura della finestra, che ne aveva una copia; scartato dichiararlo, che lascerebbe falsa la risposta 19 e perso il ripristino alla prima vista cambiata (decisione 12). ⚠️ **La regola che c'è, *«SHOWING RESETS THE BASELINE»*, misurata prima e dopo la cura** con una diagnostica sotto jsdom nel clone: mostrare una vista col nome che si apre ingrandita, poi Home, spediva **0** e **0** `SaveLayout`; con l'ascoltatore sentito subito **1** e **2**, e il secondo scriveva in `layouts.home` la vista col nome a metà smontata, la sola scheda `costs`; e lasciare una Home ingrandita per Lavoro scriveva in `layouts.work` le schede di Home: `fromJSON` lancia l'evento dentro `apply` — l'uscita in `clear`, l'ingrandimento in `deserialize` — prima che `show` rimetta la base. Quindi l'ascoltatore è **differito a un microtask**, come `dockview` differisce `onDidLayoutChange` (`AsapEvent`, `queueMicrotask`): lo stesso meccanismo del `fromJSON` di oggi, l'evento confronta con la base nuova e non salva — di nuovo **0** e **0** —, detto nel commento e nella testa di `createDock`. ✅ **Curata** nel commit che scrive questa riga. La prova, sotto jsdom accanto a quelle del dock: `maximize()` spedisce un `SaveLayout` col `maximizedNode`, Lavoro e di nuovo Home non spediscono nulla e Home si riapre ingrandita, `exitMaximized()` ne spedisce uno senza; rossa sul codice di prima, `expected 1 to be 2`, verde con la cura; senza l'ascoltatore `expected 1 to be 2`, sentito subito `expected 4 to be 2`. Misurato il 2026-09-27, macchina `Jays`. L'operazione nel Passo 6, `dock.ts`; due righe nel Passo 9 |
| **E105** | Minore — **Compito 8, Passo 5 — «Salva questa vista» spenta si vedeva accesa:** il pulsante passava a `--color-text-disabled`, ma le parole restavano nel `--color-text-muted` di `BaseLabel` e l'icona nel `--color-mark`, nei due stati: cambiava il solo cursore. La specie di **E19**. Trovata dalla revisione del compito 8 (M-1). ✅ **Decisa dal coordinatore** coi cinque criteri: nel **kit**, non nella Panoramica, perché vale per ogni `BaseLabel` e `BaseIcon` dentro un `BaseButton` spento — oggi la sola carta di «Salva questa vista» —: in `BaseButton.vue`, accanto a `.base-button:disabled`, le parole e l'icona prendono il colore dello spento del pulsante, `color: inherit`, col perché nel commento. ✅ **Curata** nel commit che scrive questa riga. Il peso del selettore, dedotto dalla revisione, **misurato**: con `:disabled` la regola vince su `.base-label :deep(.base-icon)`; senza, l'icona resta bordeaux — misurato —, perché la pareggia in peso e viene prima nel foglio — dedotto. La prova, nel browser, dentro quella della Panoramica disegnata, nei due temi: F3 prima della risposta del core, le parole e l'icona della carta contro `computed("color", "--color-text-disabled")`; poi la risposta, e le parole di nuovo nel `--color-text-muted`; rossa sul codice di prima, `expected 'rgb(101, 91, 87)' to be 'rgb(163, 154, 143)'` nel chiaro e `expected 'rgb(163, 154, 143)' to be 'rgb(111, 102, 96)'` nello scuro, verde con la cura; le prove del kit e i contrasti invariati, la suite intera verde. Misurato il 2026-09-27, macchina `Jays`. Tre righe nel Passo 9, ciascuna rossa col suo messaggio |
| **E106** | Minore — **Compito 8, Passo 5 — il campo del nome non diceva all'occhio che cosa vuole:** il nome accessibile c'era, una parola visibile no — la tavola non disegna un'etichetta, e il contratto di `BaseTextField` dà la parola al segnaposto (WCAG 3.3.2). Trovata dalla revisione del compito 8 (M-2). ✅ **Decisa dal coordinatore**: in `Overview.vue` il `BaseTextField` del nome prende `:placeholder="$t('overview.name')"` — la chiave c'è, e `placeholder` arriva all'`input` —, nessuna parola nuova. ✅ **Curata** nel commit che scrive questa riga: nella prova *«says under the field…»* un'attesa sul `placeholder` dell'`input`, rossa sul codice di prima, `expected null to be 'Nome della vista'`, verde con la cura; le prove della Panoramica nel browser, col campo aperto, verdi. Misurato il 2026-09-27, macchina `Jays`. Una riga nel Passo 9 |
| **E107** | Minore — **Compito 8 — righe col loro perché che nessuna prova teneva:** la revisione del compito 8 (M-3) le ha tolte una per una, ciascuna sulla suite intera, e la suite è rimasta verde. ✅ **Decisa dal coordinatore** coi cinque criteri: attese dentro le prove che ci sono. ✅ **Curata** nel commit che scrive questa riga: nella prova delle frecce, nel browser, il giro del Tab su «Salva questa vista» (**X2**, **D17**), Invio che apre il campo col fuoco dentro (**X10**), la freccia a destra nel campo che muove il cursore e non il fuoco (**X5**), «Annulla» che lo chiude e rende il fuoco alla carta (**X8**), e l'apertura dalla vista sullo schermo — giù fino a «Salva questa vista», Esc, e F3 di nuovo su Lavoro (**X3**); sotto jsdom F3 tolto al browser, `defaultPrevented` (**X6**), una sola carta in bordeaux con una vista col nome aperta (**X7**), «Chiudi» che chiude il cassetto dal negozio (**X9**). Ciascuna rossa con la sua riga tolta — la mutazione della revisione, `extra_rows.py` —, sulla suite intera: **X2** `expected [ -1, +0, -1, -1 ] to deeply equal [ -1, -1, -1, +0 ]`, **X3** `expected <button …(7)>…(1)</button> to be <button …(8)>…(2)</button>`, **X5**, **X6** e **X10** `expected false to be true`, **X7** `expected [ null, 'page', null, null, …(2) ] to deeply equal [ Array(6) ]`, **X8** `expected <form …(1)>…(2)</form> to be null`, **X9** `expected true to be false`; verdi sul codice. ⚠️ Due attese del testo della revisione non avrebbero preso la loro riga, **misurato**: `ArrowLeft` nel campo — a sinistra non c'è una carta, e il fuoco resta anche senza la guardia — e F3 subito dopo Invio su Lavoro — l'ultima carta raggiunta è già Lavoro —; di qui la freccia a destra, e la discesa con Esc prima di riaprire. **X12** e **X14** **dichiarate**, come **E89**: aspetto — il pulsante della pillola a destra, il colore del nome nella barra —, che giudica lo sguardo del Passo 10 (controllo 15); verdi sotto la loro mutazione, misurato. **X11** è di **E99 (c)**. ✅ **X4 curata dal coordinatore** nel commit che scrive questa frase, dopo `77c4062`: la dichiarazione decisa prima, *«ripristino che nessun caso d'uso oggi produce»*, non reggeva — misurato dal curatore con una diagnostica sotto jsdom: un nome scritto a metà, la Panoramica chiusa con F3 e riaperta, e senza le due righe del `watch` il campo tornava aperto con *«Revis»* —; ora, nella prova *«says under the field…»*, dopo il rifiuto di *«home»* F3 chiude e F3 riapre, e il campo non c'è; riaperto, è vuoto e senza rifiuto. Rossa, sulla suite intera, senza le due righe del `watch`, `expected <form …(1)>…(2)</form> to be null`, e sotto jsdom senza la sola `name.value = "";`, `expected '  home ' to be ''`; verde sul codice. ⚠️ Il `refusal.value = null;` dello stesso `watch`, tolto, lascia verde anche questa prova, misurato: `startNaming` azzera il rifiuto a ogni apertura del campo, e la riga del `watch` è una seconda cintura che nessun uso osserva — **dichiarata**, come **E89**. Misurato il 2026-09-27, macchina `Jays`. Nove righe nel Passo 9 |
| **E108** | Minore — **Compito 8, Passo 2 — l'oracolo `computed` rispondeva per un token che la pagina non ha:** `computed("color", "--color-txet")` rendeva il colore del `body`, `computed("border-top-left-radius", "--radius-crad")` `0px`, `computed("background-color", "--color-bg-srface")` `rgba(0, 0, 0, 0)` — proprio ciò che una prova *«non dipinge nulla»* attende —, e **E92** l'ha portato da aiutante locale a interfaccia di tre file. Trovata dalla revisione del compito 8 (M-4). ✅ **Decisa dal coordinatore**: `computed` chiama prima `readToken(token)`, che lancia sul token che manca — la regola di `readToken` (**E73**) —, col perché nel commento. ⚠️ **Il nome resta `computed`**, dichiarato: lo scontro con quello di Vue fallisce forte, al caricamento (`Identifier 'computed' has already been declared`), e si risolve con un alias dove capita; rinominarlo toccherebbe tre file di prova per un caso che nessun file ha oggi. ✅ **Curata** nel commit che scrive questa riga: una prova nuova in `probes.browser.test.ts`, un token inesistente lancia e uno definito si calcola; rossa sul codice di prima, `expected [Function] to throw an error`, verde con la guardia, e di nuovo rossa senza. Nel testo la guardia sta dentro la sostituzione di `probes.ts` del Passo 2, con l'import in testa, e la prova nuova nel Passo 2: nella sequenza del piano la prova nasce verde, e il suo rosso è questo. Misurato il 2026-09-27, macchina `Jays`. Una riga nel Passo 9 |
| **E109** | Nit — **Compito 8, Passi 2 e 5 — quattro commenti che il compito lasciava a metà** (gotcha **#58**): la testa di `testing/probes.ts` diceva che ogni funzione viene da una tavola e rende quanto ha guardato, e `computed` no; `testing/axe.ts` nominava le sole sonde della pagina kit fra chi accende il contrasto, e dal compito 8 lo accende anche la prova della Panoramica; `frame.browser.test.ts` diceva che il compito 8 aggiunge la striscia e la Panoramica, e aggiunge anche la barra, il cassetto dalla tastiera, la fascia `stop` e l'anello; `Overview.vue` diceva che prima della risposta il negozio non tiene un pacchetto, e un `settle` del dock prima del benvenuto ne costruisce uno (**E40**). Trovata dalla revisione del compito 8 (N-1). ✅ **Curata** nel commit che scrive questa riga: le quattro frasi senza conto né elenco chiuso, come la revisione propone — *«the probes, and the oracles they share»*, *«the probes in the real browser turn it ON»*, *«task 8 extends it»*, *«no package of the core's»* —; nessuna prova cambia |
| **E110** | Nit, d'aspetto — **Compito 8, Passo 10 — «Salva questa vista» non è la carta della tavola:** da sola sulla sua riga è una carta piena alta 42 px con le parole in alto a sinistra, e accanto a una vista col nome si allunga all'altezza di una carta con la miniatura, vuota; la tavola della Panoramica la disegna al centro, tratteggiata, alta almeno 108 px (`.ov-card.add`). È la variante `card` del kit (**P-9**), non un errore del compito. Vista dalla revisione del compito 8 (N-2), nelle schermate dei due temi. ✅ **Decisa dal coordinatore: nessuna cura** — la forma la giudica il proprietario al Passo 10 (controllo 15); se vuole la tavola, la misura che la revisione proporrebbe è `align-self: start` sulla carta e il tratteggio al centro, da misurare ✅ **RICHIAMO DEL 2026-09-27 — il Passo 10:** detta al proprietario prima dello sguardo, che non l'ha commentata: la carta resta la variante `card` del kit |
| **E111** | Nit — **Definizione di «fatto», blocco 3, riga *«6, 18»* — il `grep` contava anche i tre *cast* di `unpack`:** `grep -cE '(theme\|named\|openNamed)\?:' gui/src/stores/layout.ts` attendeva **3**, i campi facoltativi di `LayoutPack` (**D2**, **D3**), e rende **6**: prende anche `{ theme?: unknown }`, `{ named?: unknown }` e `{ openNamed?: unknown }` dentro `unpack`, il primo dal compito 1 e gli altri due dal 7. Misurato il 2026-09-28 dal pre-controllo del compito 9 su `f35d3c2`, macchina `zagor`, eseguendo la Definizione blocco per blocco sull'albero. ✅ **Corretta** nel commit che la scrive: il comando guarda le sole dichiarazioni del tipo, `grep -cE '^  (theme\|named\|openNamed)\?: '` → **3**, misurato |
| **E112** | ⚠️ **Definizione di «fatto», blocco 4 — il perimetro non conosceva la cartella del dispaccio:** la regola voleva ogni nome di `git diff --name-only "$B"..HEAD` *«in una lista Files dei compiti 1–9, o in una voce d'errata»*, e dei **141** nomi a `f35d3c2` **58** non stanno in nessuna delle due: sono i prompt, gli script, i rapporti e le revisioni di `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`, la cartella tracciata della regola 8 di *«Come si esegue un compito»*, voluta dal proprietario il 2026-09-24; gli altri **83** hanno ciascuno il proprio percorso in una lista *Files*. Misurato il 2026-09-28 dal pre-controllo del compito 9, con uno script che cerca ogni nome fra i percorsi delle liste *Files* e dell'errata. Chi esegue avrebbe aperto una voce per cinquantotto nomi che una casa ce l'hanno. ✅ **Corretta** nel commit che la scrive: il blocco 4 nomina la terza casa, la cartella del dispaccio, nel commento del comando e nel ⚠️ sotto ⚠️ **Richiamo del 2026-09-28, E126:** degli **83**, **82** hanno il percorso in una lista *Files*, e `gui/src/panels/Chat.vue` la casa nel censimento del compito 1, che la lista chiama *«gli undici componenti»*: la sonda del pre-controllo prendeva per casa anche la riga del compito 9 che vieta di toccare il codice |
| **E113** | Nit — **Compito 9, Passo 10 — la riga N-2 di E187 della (e) riceveva i numeri e non la divergenza:** la (e) dice che il design system *«non la peggiora»*, dedotto dalle icone e dai caratteri, e il pezzo JavaScript va da `663.26 kB` a `698.20 kB` — il valore di prima dal Passo 8, quello di dopo dal cancello d'apertura a `f35d3c2` —; il piano lo sa, nelle voci che sa e non chiude (R3-25 della revisione). Il testo dettato scriveva solo i due numeri, e `CLAUDE.md` vuole che dove una misura diverge da ciò che era scritto **si registri la divergenza**, invece di allinearsi. Trovata dal pre-controllo il 2026-09-28. ✅ **Corretta** nel commit che la scrive: il testo del Passo 10 dice che la deduzione non regge |
| **E114** | Nit — **Compito 9, Passo 10 — il richiamo della stella polare diceva meno di ciò che i compiti 2 e 4 hanno fatto a `scripts/gate-gui.sh`:** *«il commento del passo delle prove e la pagina kit fuori dal pacchetto»*, mentre il compito 2 ha diviso `npm test` in due corse, un progetto alla volta (**E10**), e la cura del 4 vuota `dist/` prima del *build* (**E31**) — `git diff d10d9a5..HEAD -- scripts/gate-gui.sh`, letto il 2026-09-28. **E10** corresse la riga `gui: probes` del Passo 7, non questa: la riga 5 delle regole di `CLAUDE.md`, un compito scritto prima si legge contro il codice di adesso. ✅ **Corretta** nel commit che la scrive: il richiamo nomina le due corse ed **E31**; la sua sonda non cambia |
| **E115** | Nit — **Compito 9, Passo 12 — la colonna Commit della riga 8 c'è già:** l'ha scritta la chiusura del compito 8, `f35d3c2` — `ac56b11`, con le cure `77c4062` e `7f04cdf` (**E101**) —, e il passo la faceva scrivere *«con l'hash del compito 8»*: riscriverla perderebbe le cure. La specie di **E79**, la quarta domanda del pre-controllo. Trovata il 2026-09-28. ✅ **Corretta** nel commit che la scrive: il passo dice che la riga 8 porta già la colonna, e che non si tocca |
| **E116** | ⚠️ **Compito 9, Passo 9 — le consegne promettono alla chiusura ciò che il passo non leggeva:** ogni tabella *«Ciò che questa sessione ha imparato, e che non era scritto»* delle consegne di questo piano dice *«le raccoglie la chiusura del sotto-progetto»* — misurato il 2026-09-28, ventuno sezioni in `docs/archivio/consegna-piano-design-system.md` e una nel *«Come si riprende»* del piano, **102** righe e ~31 kB —, e il Passo 9 rileggeva la sola errata. La promessa nasce con `b66aee8`, la chiusura del compito 1, dopo la scrittura di questo compito: il contratto è cresciuto sotto il piano, la riga 5 delle regole di `CLAUDE.md`. E *«Manutenzione della documentazione»* di `CLAUDE.md` vuole, alla chiusura di un sotto-progetto, anche *«questo file se cambia il modo di lavorare»*, che il compito non nominava. ✅ **Decisa dal coordinatore** con `anthropic-skills:decision-principles` — reversibile, solo documenti, la regola del passo e quella di `CLAUDE.md` — e **corretta** nel commit che la scrive: il Passo 9 rilegge l'errata **e** quelle tabelle, che un comando elenca; una voce che è una trappola per chi viene dopo diventa un gotcha, con la regola di prima; una che cambierebbe il **modo di lavorare** va nel *«Come si riprende»* della chiusura come **proposta al proprietario**, perché `CLAUDE.md` è suo; le altre restano verbali delle loro consegne. Scartato lasciar cadere la promessa: le consegne direbbero il falso, e nessuno lo saprebbe. Il costo: una lettura di ~31 kB in più per chi esegue |
| **E117** | Nit — **Compito 9, Passi 6 e 12 — nessuna sonda diceva che un testo archiviato è parola per parola:** le sonde del Passo 6 contano il titolo nell'archivio e il puntatore nel compendio, e `check-docs.sh` controlla che un rimando porti a un file che c'è: un rimando riscritto male ci passa. Misurato il 2026-09-28 dal pre-controllo, col compito rifatto da uno script del coordinatore su un `git worktree` di `f35d3c2`: la sua prima riscrittura **perdeva la `)` di chiusura** di ogni rimando, e `check-docs.sh` usciva `OK — no inconsistencies.` sull'archivio sbagliato. ✅ **Corretta** nel commit che la scrive: `word_for_word.py`, un aiutante dello scratchpad dettato nel Passo 6, toglie la destinazione di ogni rimando, `](…)` → `]()`, dalle due parti, e cerca i testi di prima nell'archivio, in ordine — provato il 2026-09-28 in tre direzioni: **0** sull'archivio giusto, **1** con una `)` persa, **1** coi due testi in ordine rovesciato; e lo stesso aiutante nel Passo 12, per la consegna che va in archivio ⚠️ **Richiamo del 2026-09-28, E121:** *«ci passa»* vale quando nessun'altra `)` segue sulla riga, o quando il rimando porta un'ancora `#…`; con un'altra `)` e senza ancora `check-docs.sh` è rosso |
| **E118** | ⚠️ **L'assunzione della barra di scorrimento, misurata al pre-controllo del compito 9 — la A del proprietario, il 2026-09-28:** la riga del [disegno](../specs/2026-09-22-design-system-design.md), *«la barra di scorrimento regge con la preferenza di Windows che tiene sempre visibili le barre … e sotto i colori forzati»*, era rimasta senza un compito che la misurasse (la chiusura del 6bis). **I colori forzati**, emulati nel Chrome installato, 154.0.8037.58, con `commands.emulateMedia({ forcedColors: "active" })`, in una prova usa-e-getta del progetto `browser` su un `git worktree` di `f35d3c2` nello `%TEMP%`, macchina `zagor`, nei due temi, su una scatola che scorre nei due versi: la barra resta di **10** px, `offsetWidth − clientWidth` e `offsetHeight − clientHeight`, come senza emulazione; e il cursore si disegna coi colori di sistema, un contorno, **visibile anche senza il puntatore** — le foto della scatola a riposo e sotto il puntatore, nello scratchpad. Regge: sotto l'alto contrasto il cursore non si nasconde. **La preferenza di Windows**, *«Mostra sempre le barre di scorrimento»*, accesa dal proprietario, nel suo Chrome, su una pagina di prova coi token servita da Vite sulla copia: *«su 5199 non le vedo finche non passo il mouse, sul file aperto dalla dir invece le vedo sempre»* — aperta come file la pagina non carica `/src/tokens/index.ts`, dedotto, e mostra le barre di Chrome. La preferenza **non arriva** alla pagina: con la nostra regola il cursore resta nascosto finché il puntatore non entra. È il costo della **C**, dichiarato in **E59** e ora misurato; la cura sarebbe la **B**, la barra sempre visibile, scartata dal proprietario il 2026-09-26 — **registrata, non presa**. ⚠️ Nessuna prova del cancello tiene la barra sotto i colori forzati: la misura è di questo giorno, **registrata, non presa**. Il richiamo datato nella riga dell'assunzione del disegno, nel commit che scrive questa voce; la misura in una riga del Passo 8 del compito 9 |
| **E119** | Nit — **Compito 9, Passi 4, 7 e 10 — *«prima del `\|` finale»* non diceva dove va lo spazio:** ogni testo che il compito appende a una cella comincia con uno spazio, e la riga finisce con ` \|`; messo davanti alla sola barra lascia due spazi prima e nessuno dopo. La forma delle celle del repository è ` <testo> \|` — quella di `append_cell.py` nelle voci d'errata, e dei richiami nella riga della stella polare che il Passo 10 tocca. Trovata il 2026-09-28 dal pre-controllo, scrivendo `compare_task9.py`, che ricostruisce le righe dal testo del piano. ✅ **Corretta** nel commit che la scrive: le cinque frasi dicono *«prima di ` \|` che chiude la riga»* |
| **E120** | ⚠️ **Compito 9, Passi 11–13 — chi scrive il piano, la specie di E35, E69 ed E101:** i Passi 11 e 12 scrivono il piano — le uscite della Definizione di «fatto», la testa *«IL PIANO È ESEGUITO»*, la riga 9 a `✅`, il *«Come si riprende»* con la chiusura, e la consegna di prima in archivio — nel commit dell'implementatore, che committa **prima** della revisione; e la chiusura della sessione riscrive lo stesso *«Come si riprende»*: due scrittori per una sezione, e fra i due commit il piano direbbe eseguito un compito non ancora rivisto. E il Passo 13 finiva con `git push`, che nel contratto del dispaccio è del coordinatore. Trovata dal pre-controllo il 2026-09-28, scrivendo il dispaccio. ✅ **Decisa dal coordinatore** con `anthropic-skills:decision-principles`, la forma di **E101**, reversibile e fuori dal merito, e **corretta** nel commit che la scrive: l'implementatore fa i Passi 1–10, **esegue** la Definizione di «fatto» e ne riporta le uscite nel **rapporto**, e fa il commit del Passo 13 **senza** il piano né l'archivio delle consegne; i Passi 11 e 12 li fa il coordinatore nella chiusura, dopo la revisione e le sue cure, sull'albero che resta — le uscite si rilanciano lì —; il push è suo. `compare_task9.py` lo tiene: senza `--closure` un commit che tocca il piano o quell'archivio è `UNEXPECTED` |
| **E121** | Nit — **Compito 9, Passo 6 ed E117 — *«una `)` persa passa `check-docs.sh`»* vale a metà:** il controllo «internal links» prende un rimando da `](` alla prima `)` che lo segue sulla riga, e ne taglia la destinazione al primo `#`: una `)` persa passa quando nessun'altra `)` segue sulla riga, o quando il rimando porta un'ancora; con un'altra `)` e senza ancora il cancello è rosso, `1 inconsistencies to fix.`. Trovata dall'implementatore il 2026-09-28, misurata nelle due direzioni sull'albero, e completata dalla revisione su un clone: il rimando con l'ancora resta verde anche con un'altra `)` dopo. `word_for_word.py` resta la prova, e nessun Atteso cambia. ✅ **Corretta** nel commit che la scrive: la frase del Passo 6, un richiamo in **E117**, e il gotcha **#134** |
| **E122** | Nit — **Compito 9, Passo 8 — l'apertura della sezione di `riferimenti.md` dava tutte le fonti alla scrittura e alla revisione:** nove righe su ventuno della tabella delle fonti le hanno portate le voci d'errata dell'esecuzione, lette fino al 2026-09-27 — **E4**, **E7**, **E10**, **E24** (due), **E43** (due), **E46**, **E85**: il contratto cresciuto sotto il testo, la regola 5 di `CLAUDE.md`. Trovata dall'implementatore il 2026-09-28, scritta com'era dettata, e confermata dalla revisione. ✅ **Corretta** nel commit che la scrive: la frase nel Passo 8 e in `riferimenti.md` |
| **E123** | Nit — **Definizione di «fatto», blocco 4 — `git status --porcelain` → *«niente»* vale dopo il commit:** sull'albero coi Passi 2–10 applicati, dove il Passo 11 e **D20** eseguono la Definizione, il comando rende i dieci file del compito. Trovata dall'implementatore il 2026-09-28 e confermata dalla revisione, con le due uscite misurate. ✅ **Corretta** nel commit che la scrive: il commento dice *«niente, dopo il commit del compito»* |
| **E124** | Minore — **Compito 9 — E120 cura il piano e non le altre case:** il commit dell'implementatore scrive *«chiuso il 2026-09-28»* ed *«eseguito il 2026-09-28»* nella roadmap e in `README.md`, *«la chiusura del sotto-progetto 14»* nelle intestazioni del compendio, di `tracciabilita.md` e di `HANDOFF.md`, il ⏭️ al 13 e, in `riferimenti.md`, *«lì, con la data»* per le cinque corse, mentre il piano, fino alla chiusura, dice *«IL PIANO È SCRITTO»*, la riga 9 a ⬜ e la Definizione con le sole attese. Trovata dalla revisione il 2026-09-28. ✅ **Decisa dal coordinatore** coi cinque criteri, la forma di **E120**: i commit del compito — dell'implementatore, delle cure e della chiusura — si pushano **insieme**, mai il primo da solo; se una revisione riapre il compito, la chiusura riscrive quelle celle. Scartato portare quelle scritture nella chiusura: svuoterebbe il compito, e toglierebbe a `compare_task9.py` il testo che prova |
| **E125** | Minore — **Compito 9, Passo 10, e P-28 — il richiamo sulla riga «codice di prodotto» del disegno dava al piano ciò che il comando della riga rende da prima del piano:** `git diff --stat ab39f39..d10d9a5 -- . ':!docs'` — `d10d9a5` è `B`, il genitore del primo commit del compito 1 — rende `CLAUDE.md` e `scripts/check-docs.sh`, toccati dal ridimensionamento della lettura del 2026-09-23 e dalla compressione del compendio del 2026-09-24: la riga era falsa dal 2026-09-23, non dal compito 1. Trovata dalla revisione il 2026-09-28, e rimisurata dal coordinatore lo stesso giorno: `git log --format='%h %ad' --date=short ab39f39..d10d9a5 -- CLAUDE.md scripts/check-docs.sh` rende `c4ec042`, `3c2aa5d`, `eab020d` e `4a7111d`. ✅ **Corretta** nel commit che la scrive: il richiamo nel Passo 10 e nel disegno nomina i due file col comando, e **P-28** dice *«falsa dal 2026-09-23, prima del piano»* |
| **E126** | Minore — **Definizione di «fatto», blocco 4, ed E112 — la sonda del perimetro dava casa a ogni file sotto `crates/`, `gui/`, `scripts/` e `.github/`:** prendeva i percorsi fra backtick di ogni riga delle liste *Files*, e un percorso che finisce con `/` era una cartella-casa, anche nella riga del compito 9 che vieta di toccare il codice. Oggi salva il solo `gui/src/panels/Chat.vue`, che una casa vera ce l'ha — il censimento del compito 1, che la lista *Files* chiama *«gli undici componenti»* —: la conclusione *«0 senza casa»* regge. Trovata dalla revisione il 2026-09-28, con la casa stampata per ogni nome. ✅ **Corretta** nel commit che la scrive: la regola del blocco 4 dice che una riga che **vieta** una cartella non ne è la casa, e un richiamo in **E112**; la chiusura rilancia il perimetro nome per nome, con la sonda corretta |
| **E127** | Nit — **Compito 9, Passo 7 — la riga `gui: build` della sezione nuova non diceva che `dist/` si vuota prima del *build*:** `scripts/gate-gui.sh` lo fa, `rm -rf dist`, col commento *«NO OUTPUT OF A PREVIOUS BUILD»* (**E31**), e senza lo svuotamento la guardia di non-vacuità leggeva un `dist/` rimasto. La specie di **E114**: un testo scritto prima di **E31**, che ha corretto la stella polare e non questa riga. Trovata dalla revisione il 2026-09-28. ✅ **Corretta** nel commit che la scrive: la riga nel Passo 7 e in `porta-di-qualita.md` |
| **E128** | Nit — **Compito 9, Passo 10 — la riga N-2 di E187 della (e) datava al giorno della chiusura un valore del 2026-09-24, e portava le due cifre in una seconda casa:** il pezzo di prima, `663.26 kB`, è del 2026-09-24 su `B`, e i due valori vivono col loro comando in `riferimenti.md`, dove li scrive il Passo 8 — *«vive in una casa sola»*, `CLAUDE.md`. Trovata dalla revisione il 2026-09-28. ✅ **Corretta** nel commit che la scrive: la riga dice che il pezzo **cresce**, con la data di ciascun valore, e rimanda a `riferimenti.md` per i valori e il comando; la sua testa, *«Misurato il <data>»*, resta, perché `compare_task9.py` riconosce il blocco da lì |

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
| **P-19** | ⛔ **la prova della tastiera del compito 5 era instabile**: `reka-ui` 2.10.4 clicca il radio in un `setTimeout(0)` dopo il fuoco, e solo se una freccia è ancora premuta — il `keydown` alza il segno, il `keyup` lo toglie —; `userEvent.keyboard("{ArrowDown}")` preme e rilascia subito, e un `keyup` arrivato prima del timer non lascia niente di cliccato | `handleFocus` in `gui/node_modules/reka-ui/dist/RadioGroup/RadioGroupItem.js`, letto; sulla cartella di prova, dieci corse della suite intera: la prova rossa **una** volta sul codice del compito 5 e **due** col compito 7, `expected +0 to be 1` con *«Matcher did not succeed in time»*; col tasto tenuto, **nessuna** su dodici | il passo 1 del compito 5 corretto: `{ArrowDown>}`, l'`Invoke` atteso, `{/ArrowDown}` — il tasto tenuto come lo tiene una mano; e il passo 7 del compito 7 fa girare la suite cinque volte |
| **P-20** | ⛔ **la prova delle parole dei moduli può andare rossa per il tempo**: `has a name for every module type` di `gui/src/locales/copy.test.ts` — della parte 2 — fa `await import("../panels/registry")` **nel suo corpo**, quindi il grafo del registro — `vue`, `vue-i18n`, il segnaposto, la striscia e, dal compito 5, i pezzi di base — si carica **dentro** i 5 s della prova: rossa **due** volte il 2026-09-24, `5177ms` e `5027ms`, in 67 corse della suite intera coi compiti 1–7 | la durata della prova nel rapporto JSON di `vitest`, sulla cartella di prova: sul codice di `70500c0` 1,3–1,4 s, 2,5 s a freddo; coi compiti 1–7 da 0,5 a 5,2 s, secondo quando il file gira nella suite — e 2,4–3,5 s anche con `--maxWorkers=8`, quindi non è la memoria — 🔶 dedotto, la coda delle trasformazioni di `vite`, che serve tutti i processi; una sonda per strato, sotto jsdom, dà il primo `.vue` — `BaseIcon.vue`, che porta `vue` e la prima trasformazione — a 751–826 ms | il passo 2 del compito 5, che tocca già quel file: `PANEL_TYPES` e `THEME_CHOICES` importati **in cima**, e le due prove senza `await` — dopo, 6–24 ms. È la regola che lo stesso file scrive per il linter, *«A guard that can go red for being slow guards nothing»*, e la forma di tutti gli altri file di prova. Le due `await import("./Band.vue")` di `frame.test.ts` restano: costano al massimo 95 ms, perché gli import in cima al file hanno già caricato il resto |
| **P-21** | ⛔ **la prova del tema che segue il sistema è caduta una volta, e la causa NON si è trovata**: `follow the system's scheme through the real query while the choice is system` di `gui/src/tokens/tokens.browser.test.ts`, rossa **una** volta in 1331 ms — nessun evento entro il secondo che `expect.poll` aspetta di base in `vitest` 4.1.11, mentre una prova nel browser ne ha **15**. ⚠️ E la misura ha trovato un secondo difetto, nella prova: su un Windows in tema scuro il primo cambio, verso lo scuro, **non cambia nulla**, e passa anche con `watchTheme` sordo al sistema | `defaults.timeout ?? 1e3` e `resolved.testTimeout ??= resolved.browser.enabled ? 15e3 : 5e3`, letti nel pacchetto installato. Escluso, misurato il 2026-09-24 sulla cartella di prova: **la priorità** — le pagine delle prove girano a 8, e a 4 stanno i quattro processi con `--top-chrome-webui`, l'interfaccia di Chrome, e uno da 0,16 s di CPU; **la CPU piena** — 28 processi occupati a priorità normale, la prova a 35–98 ms in cinque corse; **la memoria piena** — all'avvio di ogni corsa la memoria disponibile scende da 3–5 GB a 68–150 MB, e una sonda accanto non ha mai visto l'evento oltre 12 ms, un fotogramma oltre 8, un timer oltre 21, in 34 corse; **la sequenza del file** — caratteri, movimento, alto contrasto col Tab, tema — ripetuta 1320 volte nella suite, mai oltre il secondo. Il tema scuro: `AppsUseLightTheme` vale 0, e con `watchTheme` sordo il rosso cade al **secondo** cambio, `expected 'dark' to be 'light'` | ✅ **scelto dal proprietario il 2026-09-24 — A**: il compito 2 dà al progetto `browser` `expect: { poll: { timeout: 5000 } }`, una volta per le tre attese del piano — un valore in ritardo passa, uno che non arriva resta rosso, un'attesa verde finisce subito —, con la prova del suo effetto al passo 5; e la prova del tema parte da un tema **noto**, così i due cambi vogliono l'evento su ogni macchina. La causa resta **non trovata**: se la prova ricade si riparte dagli attrezzi di `pds\tools\flakes\` e dal rapporto JSON, mai da un `grep` |
| **P-22** | **il cassetto rende il fuoco al pulsante della striscia anche senza un `DialogTrigger`**: aperto dal negozio (R3-20) e chiuso con Esc, il fuoco torna al pulsante *«moduli»*, che sta in un'altra app di Vue. Torna all'elemento che aveva il fuoco **prima** di aprire: col pulsante tolto dal fuoco prima del clic, torna al `body`. ⚠️ La lettura del codice lasciava attendere il contrario — la chiusura di `DialogContentModal` ferma il ritorno di base e dà il fuoco al `triggerElement`, qui assente — e la causa non è letta fino in fondo | una sonda nel Chrome installato, il 2026-09-24 sulla cartella di prova: dopo Esc `document.activeElement` è il pulsante; la riga *«il fuoco che torna»* del passo 9 del compito 8, il `blur()` prima di aprire; `onCloseAutoFocus` in `gui/node_modules/reka-ui/dist/Dialog/DialogContentModal.js` e lo smontaggio in `dist/FocusScope/FocusScope.js`, letti | nessuna cura; una prova del compito 8 nel browser tiene il fatto, perché un aggiornamento di `reka-ui` lo potrebbe cambiare senza che niente diventi rosso |
| **P-23** | ⛔ **`mount` di `@vue/test-utils` 2.5.0 non monta sull'elemento che riceve**: crea un `div` suo dentro `attachTo` e monta lì, e quel `div` non ha altezza — la regola di `App.vue` su `html, body, #app` non lo raggiunge. La cornice esce alta **116** px, il dock **0**, e la striscia a metà pagina, dove la fascia della connessione le ruba il clic | una sonda nel browser il 2026-09-24: `.frame` alto 116 dentro un `#app` di 900; montata con `createApp(App).mount(host)`, 900, e il dock 784 | la prova del compito 8 nel browser monta `App.vue` come la monta `main.ts` |
| **P-24** | **la pillola della striscia è alta 50 px, non 56**: le viste spedite danno alla sua riga 56 — `minimumHeight` e `maximumHeight` del pannello `strip` in `panels/views/*.json` — e il `gap` del dock, `--space-3`, se ne prende metà | la stessa sonda: il gruppo della striscia da 826 a 876, in una finestra alta 900 | il compito 8 mette il pulsante grande, 40, a `--space-1` più il bordo dal filo della pillola, sopra, sotto e a destra: la regola della risposta 4, *«dentro una pillola va una pillola»* |
| **P-25** | **il commento di `testing/axe.ts` contava i suoi utenti** — *«Two users since the design system's task 3»* — e il compito 4 ne aggiunge un terzo, `kit/kit.browser.test.ts`: dal compito 4 il commento è falso (gotcha #58) | `grep -rln 'testing/axe' gui/src` sulla cartella di prova coi compiti 1–4: tre file | ✅ **corretto nel compito 3** il 2026-09-24: il commento dice chi è arrivato per secondo, e un conto non c'è più; `contrastJudged`, che il compito 8 porta lì, nasce nella stessa forma |
| **P-26** | **la riga del disegno in `docs/README.md` dice il falso dal 2026-09-23**: *«Design system — la consegna dell'avvio … **Non è ancora un disegno** … il disegno la riscriverà sul posto»*, e il disegno l'ha riscritta sul posto quel giorno. Misurato il 2026-09-24 su `2a30916` | `grep -n 'design-system-design' docs/README.md` | la riga è **riscritta** nel commit che scrive il compito 9 (**D19**): è falsa oggi, e resterebbe falsa per tutte le sessioni del pre-controllo e dell'esecuzione; il compito 9 le aggiunge soltanto l'esecuzione |
| **P-27** | **la tabella dei piani di `docs/roadmap.md` non ha la riga di questo piano**, mentre quello della parte 2 l'ebbe all'inizio della scrittura — `f0f8fab`, 2026-09-11, *«in scrittura»* — e il suo compito 17 la portò a *«eseguito»* (P-113 di quel piano). Misurato il 2026-09-24 su `2a30916` | `grep -c 'design-system' docs/roadmap.md` → **0**; `git log --format='%h %ad %s' --date=short -S'parte-2-gui-minima.md' -- docs/roadmap.md` | la riga nasce nel commit che scrive il compito 9, *«scritto il 2026-09-24»*, con l'intestazione *«Ultimo aggiornamento»* (**D19**); il compito 9 la porta a *«eseguito»* |
| **P-28** | **tre case che la forma del compito 9 non nominava**, rese false da questo piano o dovute per precedente: la riga *«codice e spec non toccati»* della tabella dello stato della [stella polare](../specs/2026-09-07-direzione-gui-design.md), il cui *pathspec* comprende `scripts/`, che i compiti 2 e 4 toccano in `scripts/gate-gui.sh`; la riga *«codice di prodotto»* del *«Come si riprende»* del disegno, *«non toccato»*, falsa dal 2026-09-23, prima del piano (**E125**); e le righe della (e) del disegno, che danno per chiusore *«il piano»*. Il compito 17 della parte 2 curò le prime due specie nei suoi Passi 8-bis e 8-quater (R8-26 di quella revisione). Misurato il 2026-09-24 su `2a30916` | `sed -n '54p' docs/superpowers/specs/2026-09-07-direzione-gui-design.md`; `grep -n 'codice di prodotto' docs/superpowers/specs/2026-09-22-design-system-design.md`; la tabella della (e) | il Passo 10 del compito 9: un richiamo datato su ciascuna, e nella (e) l'esito di ogni voce — M-3 compresa, che dipende dal verbale della riga 5 |
| **P-29** | **la tabella dei passi di `gate.sh` in `docs/porta-di-qualita.md` ne conta sette, e `gate.sh` ne lancia nove**: mancano `dependency advisories` e `gui: fake core and SPA` — la contraddizione **C-S0-1**, registrata il 2026-09-24 e non corretta (decisione 11 del [verbale degli sfoltimenti](../specs/2026-09-23-ridimensionamento-lettura-design.md)). Il browser del compito 2 vive proprio nel passo che manca | `grep -n '^run ' scripts/gate.sh`; la tabella dopo *«Un comando solo»* in `docs/porta-di-qualita.md` | ✅ **scelto dal proprietario il 2026-09-24**, all'apertura della sessione che scrive il compito 9: *«Compito 9»*, col compito che corregge C-S0-1 dove scrive il browser. Il Passo 7 porta la tabella a nove righe e la riga C-S0-1 a ✅ (**D21**); le altre contraddizioni restano alla loro sessione |
| **P-30** | **il rapporto JSON di `vitest` 4.1.11** porta `testResults[].name` col percorso intero, `assertionResults[].status` per ogni prova e, in testa, `numTotalTests`, `numPassedTests`, `numFailedTests`, `success`; e un file saltato vi compare lo stesso. ⚠️ **E Python non legge un percorso `/c/…` scritto dentro `python -c "…"`**: Git Bash converte solo gli argomenti interi — `FileNotFoundError` da dentro la stringa, lettura riuscita dallo stesso percorso passato come argomento. Misurato il 2026-09-24 su `2a30916` | `(cd gui && npx vitest run --reporter=json --outputFile=<scratchpad>/probe.json)` sul `main` di quel giorno: 85 prove, 84 passate, una saltata, `generate-views.test.ts` | la Definizione di «fatto» legge i rapporti con `dod_suite.py`, che prende i percorsi come argomenti e ha la guardia di non-vacuità (**D23**) |

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
| **D5** | le **miniature** sono uno schema calcolato dall'albero della disposizione salvata — rettangoli in frazioni e l'icona del modulo visibile — e i gruppi galleggianti non ci sono | la risposta 19 vuole lo schema, non un `dockview` per miniatura; un gruppo galleggiante non ha un posto nella griglia. Costo: una miniatura non mostra ciò che galleggia. ⚠️ **Richiamo del 2026-09-27 (E85):** un gruppo **ingrandito** si disegna **solo**, sul quadrato intero — è ciò che la disposizione apre |
| **D6** | le **icone dei moduli** che le tavole non mostrano le sceglie il piano, una riga ciascuna nella mappa — la **Chat** compresa | le tavole danno Stato, Permessi, Passi, Attività, i moduli, la ricerca, le viste; la Chat no, e `message-square`, che il piano le dà, sulle tavole segna i «Messaggi di stato» (R2-15 della revisione); la presa grande vuole l'icona di **ogni** tipo di modulo. Costo: una riga per cambiarne una |
| **D7** | la pagina kit mostra i due temi **uno alla volta**, con la scelta in cima, e non affiancati | un dialogo di `reka-ui` va in un portale sul `body` e prenderebbe il tema della radice, non quello della colonna. Costo: per confrontarli si cambia la scelta |
| **D8** | le parole della pagina kit sono **esemplari** scritti nel file, con un blocco del linter che lo dice, limitato a `src/kit/` | è una pagina di sviluppo fuori dal pacchetto: le sue parole in `it.json` finirebbero nel pacchetto per niente. Costo: una eccezione in più, in un posto solo |
| **D9** | **`readToken` lancia un errore per un token che la pagina non definisce, e sotto jsdom `base.css` si carica dal file**, in `src/jsdom-setup.ts` | P-16: un token assente in silenzio avvelena la disposizione salvata; leggere il file e non ricopiare i valori tiene la casa unica della tavola. Solo `base.css`: nessuna prova sotto jsdom legge un colore. Costo: ogni prova jsdom ha i token di `base.css` sulla radice — misurato, nessuna delle altre cambia esito |
| **D10** | **il bordo della zona d'arrivo è `--dv-drag-over-border` in `dock.css`**, e il tema TypeScript non porta `dndOverlayBorder` | le due vie erano aperte (R3-22); `updateTheme` di 8.3.1 lascia la variabile al foglio quando il campo manca, e un colore scritto in TypeScript sarebbe una seconda casa. Costo: il bordo non lo vede nessuna prova automatica; si guarda al passo 8 del compito, trascinando una linguetta |
| **D11** | **la faccia della presa grande è un `.vue`**, `frame/BigTabFace.vue`, che `BigTab.ts` monta come `VueContent` monta un pannello | una funzione `h()` dentro `BigTab.ts` avrebbe fatto lo stesso, ma fuori dalla vista del linter dei template — la trappola 5 in un'altra forma; col `.vue` le regole di `harness/panels-and-frame` e di `no-raw-text` la leggono. Costo: un'app Vue per linguetta, smontata in `dispose` e provata |
| **D12** | **`saveNamed` riceve da chi la chiama i nomi che la cornice mostra per le tre viste, e due nomi sono lo stesso nome a meno degli spazi intorno e delle maiuscole** | nessun negozio legge le parole di `it.json`, e farlo ne farebbe il primo; «Home» e «home», affiancate nella Panoramica, si leggerebbero come una vista sola. ✅ **Scelto dal proprietario il 2026-09-23 — A**, il confronto senza le maiuscole e gli spazi intorno, contro B, il confronto esatto. Costo: un parametro in più, che la Panoramica del compito 8 passa; e «Revisione» e «revisione» non possono essere due viste |
| **D13** | **`showView(view)` nel negozio: aprire una delle tre viste chiude quella col nome** | la regola vive dov'è lo stato, e una prova del negozio la raggiunge: nessuna prova monta `Frame.vue`. Costo: `Frame.switchTo` chiama `showView` invece di scrivere `view`; le prove che scrivono `view` restano valide, perché lì nessuna vista col nome è aperta. ⚠️ **RICHIAMO DEL 2026-09-27, dal pre-controllo del compito 7 (E76):** dal 6bis `frame.browser.test.ts` monta la cornice, attraverso `App.vue`; la regola resta nel negozio, e la riga di `Frame.switchTo` non la tiene nessuna prova — misurato, e dichiarato: il compito 8 riscrive `Frame.vue` |
| **D14** | **la miniatura non disegna la striscia** | sta in ogni vista, uguale, senza un'icona in `ICONS`, e le miniature della tavola della Panoramica non la disegnano: il *«come si disegna»* che il *«Come si riprende»* lasciava alla tavola o al compito. Costo: in fondo a ogni miniatura resta una fascia vuota, alta quanto la riga della striscia |
| **D15** | **la domanda della conferma vive nel negozio, `useInvoke().asking`**, e `Confirm.vue` la legge da lì | la leggono in due — la finestra e F3 della cornice — e due copie della regola di D59 divergerebbero (gotcha #68). Costo: un getter in più nel negozio |
| **D16** | **F3 tace anche mentre il cassetto è aperto**, non solo sotto la conferma | la ragione del *«Come si riprende»* — una finestra modale sopra un'altra ne copre la domanda — vale per il cassetto allo stesso modo. Costo: col cassetto aperto F3 non fa niente, ed Esc lo chiude |
| **D17** | **una carta sola nel giro del Tab**, l'ultima che le frecce hanno raggiunto, e il fuoco apre sulla vista che si vede | la decisione 19 del disegno lasciava al piano *«come si tiene il fuoco su un elemento solo»*; col Tab su ogni carta la griglia costerebbe un tasto per carta. Costo: a *«Salva questa vista»* si arriva con le frecce |
| **D18** | **la miniatura di Compatta è il suo schema, non una «finestrella»** | la (d) scrive *«Compatta come una finestrella»*, dalla tavola della risposta 13; la risposta 19 ha deciso poi gli schemi *«dalla disposizione salvata»*, che *«dicono sempre il vero»* — e la Compatta di oggi è una disposizione a finestra piena, un segnaposto che dice di esserlo: la sua forma vera è del sotto-progetto 10. Una finestrella disegnata direbbe ciò che non è. Costo: fino al 10 la miniatura di Compatta mostra la knowledge base a tutta pagina. ✅ **Scelto dal proprietario il 2026-09-24 — A**, lo schema, alla chiusura della sessione che ha scritto il compito 9. ⚠️ **Richiamo del 2026-09-27 (E103):** il costo non era vero — misurato dalla revisione del compito 8, la miniatura di Compatta disegnava la knowledge base a **metà**, perché il generatore scriveva la vista prima che `dockview` applicasse alla striscia i suoi 56 px, mentre lo schermo gliela dava quasi tutta; dal generatore curato la miniatura la disegna com'è sullo schermo, a tutta pagina meno la riga della striscia |
| **D19** | **la riga del disegno in `README.md` e la riga di questo piano nella roadmap si scrivono nel commit che scrive il compito 9**, non dal compito 9 | P-26 e P-27: la prima è falsa oggi, e la seconda è il precedente della parte 2 — un piano scritto sta nella tabella dei piani prima di essere eseguito. Costo: il compito 9 tocca le due righe una seconda volta, per l'esecuzione |
| **D20** | **la Definizione di «fatto» si scrive ORA, coi comandi e le uscite attese dai compiti; il compito 9 la esegue riga per riga e scrive accanto a ogni comando l'uscita vista, con la data** | unisce la forma dei Traguardi 1–4, la definizione scritta col piano, e quella di D74 della parte 2, comandi con le uscite vere. Un'uscita che diverge dall'attesa è una voce d'errata, non un'attesa nuova. Costo: un'attesa di oggi può essere sbagliata al compito 9 — ed è il punto: *«un'evidenza scritta prima della misura è un'ipotesi»* (`CLAUDE.md`) |
| **D21** | **C-S0-1, corretta dal compito 9, resta nella tabella delle contraddizioni con un ✅ datato**, e il segno `⚠️ **[C-S0-1]**` sotto la tabella dei passi esce | è la forma di X-1 e X-3 nella tabella dell'audit, che resta la casa unica anche delle voci chiuse; togliere la riga lascerebbe la sessione delle contraddizioni senza sapere che una è già fatta. Costo: una tabella intitolata *«non risolte»* porta una riga risolta, finché quella sessione non ne decide la forma |
| **D22** | **una riga sola nella §12 del compendio, per il disegno e il piano insieme** | è la forma delle righe dei gesti e della knowledge base; le quattro righe della parte 2 erano quattro file con quattro modi di leggerli. Costo: una riga più lunga |
| **D23** | **la suite, nella Definizione di «fatto», sono cinque corse coi rapporti JSON**, lette da `dod_suite.py`, che stampa i conti file per file ed esce 1 su una caduta o su un rapporto vuoto | è la regola di P-19, P-20 e P-21 — una caduta è una voce d'errata, non una corsa da ripetere finché passa —, e un rapporto dà i conti di ogni file senza avviare `vitest` una volta per file. Costo: cinque corse della suite, e un aiutante che il piano porta nel proprio testo |
| **D24** | **se il giudizio del proprietario sull'aspetto del dock (controllo 15) non è scritto da nessuna parte al compito 9, il coordinatore glielo chiede prima del commit** | il compito 6 lo vuole *«alla prima prova»*, e ciò che non gli piace è una voce d'errata: un controllo che è *«il proprietario giudica»* non si chiude per silenzio. Costo: una domanda in più alla chiusura |
| **D25** | **uno scarto solo, `SLACK`, di mezzo pixel, per il raggio e per *«nell'angolo»*** — il compito 6bis, 2026-09-27 | **E61** vuole mezzo pixel sul raggio e lascia al piano la soglia dell'angolo: due distanze più lontane di mezzo pixel non possono avere lo stesso centro, e il giudizio fuori dall'angolo le prende; misurato, abbassarla non cambia nessun verdetto delle prove. Costo: un pezzo a **un** pixel dalla diagonale dell'angolo cade nella regola fuori dall'angolo, che rifiuta il solo troppo tondo, e lì un raggio troppo piccolo passa, dove la soglia di 1,5 lo rifiutava — latente: ogni coppia che le prove incontrano ha le due distanze uguali (**E71**). In cambio, una costante al posto di tre cifre |
| **D26** | **le barre di scorrimento mostrate a TUTTO il progetto browser**, con `ignoreDefaultArgs: ["--hide-scrollbars"]`, e non un progetto o un'istanza a parte per il controllo 22 | le prove girano come la SPA si vede — Electron le barre le mostra —, e misurato il 2026-09-27 nessun'altra prova cambia; un progetto in più sarebbe un altro verde da tenere non vuoto (**E10**). Costo: una barra che comparisse dove prima non c'era sposterebbe di 10 px ciò che una prova misura — e la prova lo direbbe |
| **D27** | **con un'azione la riga del messaggio è al centro**: `data-action` su `BaseNotice`, e nella tavola `.msg:has(>.btn){align-items:center}` al posto di `.msg>.btn{align-self:center}` della copia approvata | la risposta **B** del proprietario del 2026-09-27, guardando la pagina di confronto (**E62**). Senza azione la riga resta in alto, con l'icona accanto al titolo. Costo: un messaggio con testo **e** azione — oggi nessuno — avrebbe l'icona a metà delle due righe |
| **D28** | **le forme di `BaseNotice` che la (b) non nomina**: il prop `onPage`, sull'elemento `data-on-page`, come `pill` di `BaseButton`; il colore del tono sull'elemento, che l'icona disegna con `currentColor`; e nel browser i ruoli giudicati col colore **calcolato** di un token — `colourOf` della pagina kit —, non con `readToken`, che rende il testo del token e non un colore | la forma più corta coerente coi pezzi che ci sono. Costo se sbagliato: un rinomino, o una prova che legge in un altro modo |
| **D29** | **`frame.browser.test.ts` nasce col 6bis**, con gli aiutanti del compito 8 — `frame(theme)` e `px(token)` —, e il compito 8 lo **estende** | il controllo 23 vuole la fascia misurata fra la barra e le schede, cioè la cornice intera, e un secondo file con lo stesso montaggio sarebbe un secondo modo di fare la stessa cosa. Costo: il compito 8 cambia da *«Crea»* ad *«aggiungi»* — **E63** |

## Le voci aperte che questo piano SA, e non chiude

Rilette il 2026-09-23 coi due comandi della §6 del compendio e con la tabella dell'audit: nessuna voce aperta dei Traguardi
5 e 6 ha per chiusore la GUI o questo lavoro, e nessuna è del tipo *«prima di questo traguardo»*.

| Voce | Di chi | Che cosa ne fa questo piano |
|---|---|---|
| **X-2** e **X-4** dell'[audit](../../audit-2026-08-27.md) | del proprietario | niente: non toccano la GUI |
| **N-2 di E187**, l'avviso di `vite` sui pezzi sopra i 500 kB | del proprietario | ogni compito che cambia la SPA **misura** il pezzo JavaScript dopo il *build* — `npm run build 2>&1 \| grep -E 'assets/index-.*\.js '` — e lo scrive nel commit: l'1, il 5, il 6, il 6bis, il 7 (**E80**) e l'8; il 3 no, perché la SPA non importa ancora il kit (R2-13). ⚠️ La (e) dice che il design system non lo peggiora, ed è una deduzione: la revisione l'ha misurato **crescere al compito 5**, quando i pezzi di base entrano nei pannelli (R3-25), e la cifra si porta al proprietario |
| la **memoria della macchina `zagor`** all'avvio della suite coi compiti 1–7: 24 processi di prova, circa uno per core, e il Chrome delle prove — la memoria disponibile scende da 3–5 GB a 68–150 MB, e Windows scrive su disco; con `--maxWorkers=8` resta sopra 2,7 GB, a parità di tempo, misurato il 2026-09-24 (P-21) | del proprietario | niente: non è la causa né di P-20 né di P-21, e limitare i processi cambia il cancello su ogni macchina |
| **E228**, progress e notifiche | del proprietario | niente: i token ci sono già, `--z-toast` e i colori di stato |
| il **terzo carattere** per il codice | del proprietario | niente: il monospazio resta quello del sistema (decisione 16 del disegno) |
| **AUD-004** | del proprietario | niente: sbarra il sotto-progetto 13, non questo |
| la **finestra** del guscio — `titleBarOverlay`, `setTitleBarOverlay`, gli angoli di Windows; e la **prima pittura**: i ruoli vivono solo sotto `[data-theme]`, che lo script mette dopo l'analisi del documento, quindi prima dello script la finestra non ha fondo (R1-13 della revisione, dedotto e non misurato) | del sotto-progetto **10** | niente: le regole della (d) restano scritte nel disegno per chi farà il guscio; per la prima pittura il rimedio è del guscio — il `backgroundColor` della finestra, o mostrarla a `ready-to-show` |
| i **tre difetti di accessibilità** che `axe` trova sul dock — `nested-interactive` sulle linguette, il nome della linguetta uguale all'id del pannello, `aria-level` e `aria-label` sul contenitore galleggiante (P-18) | del proprietario | niente: vengono dalla parte 2 e da `dockview-core`, e toglierli cambia la presa grande giudicata con SP-8 — i comandi fuori dalla linguetta, un `title` in italiano dentro le viste salvate. Il compito 6 li misura e non li nasconde: nessuna prova `axe` sul dock con quelle regole spente |
| 🔶 **con una finestra modale aperta — la conferma, il cassetto e, dal compito 8, la Panoramica — Ctrl+Alt+frecce muovono ancora le tessere del dock sotto il velo**: `onKey` di `Frame.vue` non guarda le finestre aperte, da prima del design system. Dedotto leggendo il codice il 2026-09-24, **non misurato** | del proprietario | niente: non è nella (d); la cura sarebbe una riga in `onKey`, come il silenzio di F3 (D16), con una prova che conti i `SaveLayout` |

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

In `gui/src/stores/layout.ts` (`replace_unique.py`, che conserva il fine-riga del file: lo rimisura il Passo 16 — **E1**), quattro sostituzioni.

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
- Modify: `scripts/gate-gui.sh` — il passo delle prove, i due progetti **uno alla volta**, e il loro commento (**E10**)
- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` — il richiamo datato sulla riga di `scripts/gate-gui.sh` della §8 (**E10**)

**Interfaces:**
- Consumes: `gui/src/tokens/index.ts` e `watchTheme` del compito 1.
- Produces: il progetto `browser` di `vitest`, che prende ogni `src/**/*.browser.test.ts`, e nel cancello i due progetti
  lanciati uno alla volta (**E10**); il comando
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
    // ⛔ THE SECOND DIRECTION: without the request, every duration is a duration and not zero -- a VALUE, so a
    // name the base block lost is red here too, and not only in `board.test.ts` (E14 of the design-system plan).
    await commands.emulateMedia({ reducedMotion: "no-preference" });
    for (const name of ["--duration-fast", "--duration-moderate", "--duration-slow"]) {
      expect(rootStyle().getPropertyValue(name).trim(), name).toMatch(/^[1-9]\d*ms$/);
    }
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
    // ⛔ A KNOWN START (P-21 of the design-system plan): `null` gives back the machine's own scheme -- dark on a
    // Windows set to dark, where a first flip to dark changed nothing and passed with `watchTheme` deaf to the
    // system. Light first, and BOTH flips need the event.
    await commands.emulateMedia({ colorScheme: "light" });
    const root = document.createElement("div");
    const stop = watchTheme((): ThemeChoice => "system", root);
    expect(root.dataset.theme).toBe("light");
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
     * ⛔ TWO PROJECTS (design system, section (f)): `npm test` runs both in one run, for the local loop. The gate
     * runs them ONE AT A TIME (E10 of the design-system plan): inside one run a project that finds no file is GREEN
     * -- `vitest` answers "No test files found" only when the whole run is empty -- so a renamed file or a wrong
     * glob would drop a whole project in silence. `extends: true` hands each one the `plugins` and the `define`
     * above; everything else is written per project, because an inline project inherits nothing it does not ask for.
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
          // ⛔ FIVE SECONDS FOR `expect.poll`, NOT VITEST'S ONE (P-21 of the design-system plan): the scheme probe went
          // red once, no event within 1 s, and no cause was found -- not the renderers' priority, not a full CPU,
          // not a full memory, measured on 2026-09-24. A probe here guards THAT a value comes, not how fast: a late one
          // passes, one that never comes stays red, and a green poll ends at once -- well inside a browser test's 15 s.
          expect: { poll: { timeout: 5000 } },
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
| un progetto **vuoto** è rosso nel cancello (**E10**, controllo 20) | in `vite.config.ts` l'`include` del progetto `browser` a `["src/**/*.nothing.test.ts"]`; poi, tornato il file dalla copia, quello del progetto `jsdom` allo stesso modo | lanciati come li lancia il cancello, `npm test -- --project browser` e poi `npm test -- --project jsdom`: ciascuno esce **1** sul proprio vuoto, `No test files found, exiting with code 1`. ⚠️ Nelle stesse due violazioni `npm test`, una corsa sola, esce **0**: è la ragione di E10, e il limite che il commento del Passo 3 dichiara |
| i caratteri | in `tokens/index.ts` tolta la riga di Barlow 300 | rosso: `loaded("Barlow")` sotto quattro, o le cifre |
| il movimento | in `base.css` tolto il blocco `prefers-reduced-motion` | rosso — ⛔ poi `base.css` torna dalla **copia salvata**, e `board.test.ts` lo conferma verde |
| l'alto contrasto | nella prova del contorno del focus, tolta la riga `await commands.emulateMedia({ forcedColors: "active" });` | rosso alla guardia, `expected false to be true`: senza l'emulazione la prova non parla dell'alto contrasto (R2-2) |
| il contorno è la **nostra** regola (**E11**) | in `base.css` tolto il blocco `:focus-visible` | rosso a `expect(ring(button)).toBe(true)`, `expected false to be true`: il contorno di base di Chrome non passa la sonda — ⛔ e `board.test.ts`, che vuole il foglio della tavola: poi `base.css` torna dalla **copia salvata** |
| il carattere che disegna | in `base.css` `--font-display` col solo ripiego, `"Bahnschrift", system-ui, sans-serif` al posto di `var(--font-family-tool)` | rosso al confronto del token con la sua catena senza Barlow, `expected 0 to be greater than 0.5`, mentre le due direzioni delle larghezze restano verdi: anche Bahnschrift ha le cifre proporzionali, e uguali con `tabular-nums` (R2-4) — ⛔ e `board.test.ts`, che vuole il foglio della tavola: poi `base.css` torna dalla **copia salvata** |
| il tema che segue il sistema (P-21) | in `tokens/theme.ts` tolta la riga `system.addEventListener("change", apply);` | rosso dopo cinque secondi al **primo** cambio, `expected 'light' to be 'dark'`: l'evento non arriva, e l'attesa lo aspetta cinque secondi, non uno — e rossa **anche** la prova del compito 1 sotto jsdom, *«follows the system while the choice is `system`…»* di `theme.test.ts`, `expected 'dark' to be 'light'`: la stessa riga, tenuta dal `matchMedia` finto (**E12**) |
| l'attesa di 5 s arriva al progetto `browser` (P-21) | il file usa-e-getta qui sotto, `gui/src/late.browser.test.ts`; poi, col file ancora lì, la riga `expect:` tolta da `vite.config.ts` | verde con la riga; senza, rosso dopo un secondo, `expected 'before' to be 'after'` — poi il file si **cancella** |

Il file usa-e-getta della riga *«l'attesa di 5 s»*, che il passo cancella (**E13**):

```ts
// A THROWAWAY PROBE (task 2, step 5): a value that turns right two seconds late -- green only when the project's
// `expect.poll` budget is longer than 2 s, the proof that the setting reaches the browser project.
import { expect, it } from "vitest";

it("waits for a value that arrives two seconds late", async () => {
  let value = "before";
  setTimeout(() => {
    value = "after";
  }, 2000);
  await expect.poll(() => value).toBe("after");
});
```

Ogni violazione torna indietro con la copia salvata; poi, dalla radice del repository, `git status --porcelain` è quello
di prima della prima violazione (vincolo 11): nessun file nato dai rossi del browser (R2-3).

- [ ] **Passo 6: il cancello coi due progetti uno alla volta, e il richiamo nella §8 del 2 (E10)**

In `scripts/gate-gui.sh`, *Trova*:

```bash
echo "-------- gui: probes"
npm test
```

*Sostituisci con:*

```bash
echo "-------- gui: probes"
# ⛔ TWO PROJECTS, ONE AT A TIME (design system, task 2; E10 of its plan): jsdom, and the INSTALLED Chrome for
# what only a layout engine can judge -- fonts, motion, radii, clipping, the contrast of the drawn page. One at a
# time because inside ONE run a project that finds no file is GREEN -- `vitest` 4.1.11 says "No test files
# found" only when the whole run is empty, measured on 2026-09-24 -- while alone each one goes red on its own
# emptiness: a renamed file or a wrong glob must not drop a project in silence (control 20 of the design). Still
# `npm test`, the command of §8, twice. Nothing is downloaded (decision 22 of the design): a machine without
# Google Chrome goes red here with Playwright's message, and that is a prerequisite of the environment, like the
# `rustup` target and `cargo audit`.
npm test -- --project jsdom
npm test -- --project browser
```

⚠️ **E10 — il richiamo datato.** La forma della riga di `scripts/gate-gui.sh` nella §8 del
[disegno del sotto-progetto 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) è approvata — *«`npm ci`,
`npm run build`, `npm test`»* —, e ora `npm test` gira due volte. In quel file (`replace_unique.py`), *Trova*:

```markdown
l'ordine che quella cella fissa **non cambia**, la catena si allunga |
```

*Sostituisci con* — `<data>` è il giorno del commit:

```markdown
l'ordine che quella cella fissa **non cambia**, la catena si allunga ✅ **RICHIAMO DEL <data>, dal compito 2 del [piano del design system](../plans/2026-09-23-design-system.md) (E10), sulla stessa cella:** `npm test` gira **due volte**, `npm test -- --project jsdom` e poi `npm test -- --project browser` — in una corsa sola un progetto che non trova file è verde, da solo è rosso —: l'ordine non cambia |
```

Atteso: `grep -c 'dal compito 2 del \[piano del design system\]' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **1**; il link lo legge `check-docs.sh` al
Passo 7, perché quel file sta fuori da `plans/`.

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
 * Every violation axe finds under a node, as `rule: targets`, and nothing else. Its second user came with the design
 * system's task 3 -- `components/kit.test.ts`, after `a11y.test.ts` -- so it lives here once.
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
    // ⛔ AND NO MARK WRITTEN "false": Vue writes a `false` attribute as that string, and `[data-icon-only]` and
    // `[data-pill]` would take it -- every button square, every button a pill (`|| undefined`, E21 of the plan).
    expect(worded.attributes("data-icon-only")).toBeUndefined();
    expect(worded.attributes("data-pill")).toBeUndefined();
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
    // ⛔ THE SECOND DIRECTION: and not on the root around it as well -- a caller's `@keydown` there would run twice, on
    // the input and on its bubble (E15 of the plan).
    expect(wrapper.element.hasAttribute("placeholder")).toBe(false);
    // ⛔ NOR ON THE FRAME: `[data-disabled]` and `[data-error]` would take a mark written "false" -- every field off,
    // every field wrong (`|| undefined`, E21 of the plan).
    expect(wrapper.get(".frame").attributes("data-disabled")).toBeUndefined();
    expect(wrapper.get(".frame").attributes("data-error")).toBeUndefined();
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

  it("checks nothing on null, not even on a click, and names the group with its legend", async () => {
    const asked: string[] = [];
    const wrapper = mount(BaseRadioGroup, {
      attachTo: document.body,
      props: { modelValue: null, options, legend: "Policy VRAM", "onUpdate:modelValue": (value: string) => asked.push(value) },
    });
    const checked = (): (string | undefined)[] => wrapper.findAll('[role="radio"]').map((radio) => radio.attributes("aria-checked"));
    expect(checked()).toEqual(["false", "false"]);
    const group = wrapper.get('[role="radiogroup"]');
    expect(wrapper.get(`[id="${group.attributes("aria-labelledby")}"]`).text()).toBe("Policy VRAM");
    await wrapper.findAll('[role="radio"]')[1]?.trigger("click");
    expect(asked).toEqual(["local"]);
    // ⛔ `null` INCLUDED (E21 of the plan): given to reka-ui as no value at all, the group would hold a state of its
    // own, and the click would check the radio before whoever holds the value answers (P-8).
    expect(checked()).toEqual(["false", "false"]);
    wrapper.unmount();
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
    // ⛔ NO DESCRIPTION, NO `aria-describedby` (R2-18 of the review; E16 of the plan): without the component's own
    // `undefined`, reka-ui 2.10.4 points it at a description that is not there, and warns.
    expect(dialog?.hasAttribute("aria-describedby")).toBe(false);
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    await nextTick();
    expect(asked).toEqual([false]);
    wrapper.unmount();
  });

  it("is described by its description, when it has one: the other direction of E16", async () => {
    const wrapper = mount(BaseDialog, { attachTo: document.body, props: { open: true, title: "Serve un permesso", description: "Vale per questa sessione." } });
    await nextTick();
    const dialog = document.querySelector('[role="dialog"]');
    expect(dialog).not.toBeNull();
    // ⛔ BOTH HALVES (E21 of the plan): the `v-bind` that turns `aria-describedby` off must not do it here, and the
    // description must be there for it to point at.
    expect(document.getElementById(dialog?.getAttribute("aria-describedby") ?? "")?.textContent).toBe("Vale per questa sessione.");
    wrapper.unmount();
  });

  it("opens from its `trigger` slot when unbound", async () => {
    const wrapper = mount(BaseDialog, { attachTo: document.body, props: { title: "Serve un permesso" }, slots: { trigger: () => h(BaseButton, null, () => "Apri") } });
    // ⛔ CLOSED FIRST: a dialog open from the start would pass the probe below (E21 of the plan).
    expect(document.querySelector('[role="dialog"]')).toBeNull();
    await wrapper.get("button").trigger("click");
    await nextTick();
    expect(document.querySelector('[role="dialog"]')).not.toBeNull();
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
/* Off, the quiet one too: its muted colour weighs as much as `:disabled` and comes later (E19 of the plan). */
.base-button[data-variant="quiet"]:disabled {
  color: var(--color-text-disabled);
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
/* The hover gives way to the error, as on the board, where `.is-error` comes after `.is-hover` (E20 of the plan). */
.frame:hover:not([data-disabled], [data-error]) {
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

In `gui/eslint.config.js` (`replace_unique.py`), sei sostituzioni. *Trova*:

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
     * and their TYPES stay there; but the import rules below must see `icons.ts`, `BigTab.ts`, `dock.ts`, or `lucide` --
     * and, from task 5, `reka-ui` -- could come in through a `.ts` unseen. The unscoped blocks now reach the `.ts` too,
     * and the gate says whether any of them objects.
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

*Trova* — nel commento di `vue/multi-word-component-names`, la frase che gli otto pezzi di base smentiscono (**E17**; gotcha #58):

```js
       * Every `.vue` file here but `ViewBar` is single-word (P-99; recounted at the review, R7-11).
```

*Sostituisci con:*

```js
       * When it was turned off, every `.vue` file here but `ViewBar` was single-word (P-99;
       * recounted at the review, R7-11); from the design system's task 3 on, the kit's base
       * pieces are `Base*`, two words by name.
```

*Trova* — nella riga di testa del blocco `harness/ts-in-vue`, il conto dei `.vue` che gli otto pezzi di base smentiscono (**E22**; gotcha #58):

```js
     * ⛔ THE TypeScript PARSER FOR THE `.vue` FILES, AND WITHOUT IT SIX OF THE THIRTEEN DO NOT PARSE.
```

*Sostituisci con:*

```js
     * ⛔ THE TypeScript PARSER FOR THE `.vue` FILES: WITHOUT IT A `<script setup lang="ts">` DOES NOT PARSE.
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
| in `src/components/icons.ts` la riga `import { useCore } from "../stores/core";` | **rosso**: lo stesso messaggio, dal blocco della mappa, che toglie `lucide` dal divieto e tiene la regola dei pezzi di base (**E18**) |
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
| in `src/components/BaseTextField.vue` tolta la riga `defineOptions({ inheritAttrs: false });` | **rosso**: `expected true to be false`, alla prova di `BaseTextField` (**E15**) |
| in `src/components/BaseDialog.vue` tolta la riga `v-bind="description === undefined ? { 'aria-describedby': undefined } : {}"` | **rosso**: `expected true to be false`, alla prova di `BaseDialog` che la apre senza descrizione (**E16**) |
| in `src/components/BaseDialog.vue` il `v-bind` che spegne sempre, `v-bind="{ 'aria-describedby': undefined }"`; e, a parte, la riga della `DialogDescription` tolta | **rosso**, ciascuna: `expected undefined to be 'Vale per questa sessione.'`, alla prova della finestra con la descrizione (**E21**) |
| in `src/components/BaseRadioGroup.vue` `:model-value="modelValue ?? undefined"` — il `null` dato a `reka-ui` come nessun valore | **rosso**: `expected [ 'false', 'true' ] to deeply equal [ 'false', 'false' ]`, alla prova del `null` (**E21**) |
| in `src/components/BaseDialog.vue` `v-if="false"` sul `DialogTrigger` | **rosso**: `Unable to get button within: <!--v-if-->`, alla prova dello slot `trigger` (**E21**) |
| in `src/components/BaseButton.vue` `:data-icon-only="iconOnly()"`, e a parte `:data-pill="pill"`; in `src/components/BaseTextField.vue` `:data-disabled="disabled"`, e a parte `:data-error="error !== undefined"` | **rosso**, ciascuna: `expected 'false' to be undefined` (**E21**) |

⚠️ **Il linter non ha una guardia di non-vacuità**: se un `files` smettesse di trovare i suoi file, le regole tacerebbero
col verde. La prova delle due direzioni si rifà a mano in ogni compito che tocca `eslint.config.js`; una guardia statica è
un controllo nuovo, del proprietario (vincolo globale 7 della parte 2) — **registrata, non presa**. ⚠️ E un `import()`
**dinamico** passa le regole: `no-restricted-imports` guarda gli import statici (R2-17). `no-restricted-syntax` su
`ImportExpression` lo coprirebbe; oggi nessun file del kit ne ha uno, e la regola non la prende nessun compito.
⚠️ E un import da un **sotto-percorso**, `lucide/…`, passa le regole, che prendono il nome esatto `lucide` (**E23**):
oggi lo ferma `vue-tsc` nel *build* — `TS7016`: il pacchetto porta un `.d.ts` solo —, e una dichiarazione scritta a mano
lo aprirebbe.

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
- Create: `gui/src/testing/probes.ts` — le tre sonde come funzioni, per questo compito e per i compiti 6 e 8
- Create: `gui/src/testing/probes.browser.test.ts` — la sonda dei raggi su scatole fatte a mano, dove la pagina kit non
  arriva: gli angoli diversi (**E30**) e il ramo fuori dall'angolo (**E33**)
- Modify: `gui/eslint.config.js` — il blocco delle parole esemplari (D8)
- Modify: `gui/vite.config.ts` — il progetto `browser` rifà il pacchetto delle dipendenze a ogni corsa (**E24**)
- Modify: `scripts/gate-gui.sh` — la pagina kit **fuori** dal pacchetto, provato sull'uscita del *build*

**Interfaces:**
- Consumes: gli otto pezzi e `ICONS` del compito 3; `watchTheme` e `isThemeChoice` del compito 1; `violations` di
  `testing/axe.ts`; il progetto `browser` e `userEvent` del compito 2.
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
 * The owner's rule of answer 4 -- OUTER radius = INNER radius + distance. For every element with a rounded corner, the
 * nearest rounded ancestor inside a root, and their four corners, EACH WITH ITS OWN RADIUS (E30 of the design-system
 * plan): where the inner corner sits close to the outer one (within the larger radius, plus 2 px), an element IN the
 * corner must share the centre, and one OFF the corner must not be rounder than the outer radius minus the smaller
 * distance. A straight corner, inside or outside, is never compared (answer 20) -- a sheet's `r r 0 0` included. SVG
 * content is a drawing, not a surface.
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
 * Every icon is drawn, strokes with `currentColor` -- read on the COMPUTED stroke, since a CSS rule beats the
 * presentation attribute (E32 of the design-system plan) -- and, in a flex row that centres, sits within 0.75 px of the
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
      const drawn = getComputedStyle(svg);
      if (drawn.stroke !== drawn.color) problems.push(`stroke is not currentColor: ${name}`);
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

E le prove a mano della sonda dei raggi (**E30**, **E33**): la pagina kit dà a ogni pezzo quattro angoli uguali e lo
mette *nel* suo angolo, quindi qui si prova ciò che lei non mostra — un raggio per angolo, e la regola fuori dall'angolo.
Crea `gui/src/testing/probes.browser.test.ts` (LF):

```ts
import { afterEach, expect, it } from "vitest";

import { concentricRadii } from "./probes";

// ⛔ THE RADIUS PROBE ON BOXES DRAWN BY HAND (E30 and E33 of the design-system plan): the kit page gives every piece four
// equal corners and puts it IN its corner, so two things would stay unproven there -- that each corner is read with its
// own radius, and the rule OFF the corner. ⛔ NON-VACUITY (trap 1): every case says the WHOLE report, `near` included,
// so a probe that met no corner cannot pass.

afterEach(() => document.body.replaceChildren());

function box(className: string, css: string, parent: Element): HTMLElement {
  const element = document.createElement("div");
  element.className = className;
  element.style.cssText = css;
  parent.append(element);
  return element;
}

/** The outer box: 300 × 120, every corner rounded 20. */
const outer = (): HTMLElement =>
  box("outer", "position:absolute;left:0;top:0;width:300px;height:120px;border-radius:20px", document.body);

it("judges the rounded corner of a piece whose top-left is straight (E30)", () => {
  const root = outer();
  // Only the bottom-left is rounded, 30, in the outer corner of 20 at 0/0, where it should be 20.
  box("piece", "position:absolute;left:0;bottom:0;width:120px;height:60px;border-radius:0 0 0 30px", root);
  expect(concentricRadii([root])).toEqual({
    near: 1,
    bad: ["piece in outer, bottom-left: radius 30.0, outer 20.0, distance 0.0/0.0"],
  });
});

it("never compares a straight corner -- a sheet's r r 0 0 (answer 20, E30)", () => {
  const root = outer();
  // The sheet's straight bottom corners sit 12/12 from the outer ones; the piece in the top-left corner is the one judged.
  box("sheet", "position:absolute;left:12px;right:12px;bottom:12px;height:40px;border-radius:16px 16px 0 0", root);
  box("piece", "position:absolute;left:12px;top:12px;width:60px;height:30px;border-radius:8px", root);
  expect(concentricRadii([root])).toEqual({ near: 1, bad: [] });
});

it("off the corner, refuses a piece rounder than the outer radius minus the smaller distance (E33)", () => {
  const root = outer();
  // 17/13 from the bottom-left corner: at most 20 - 13, with the probe's 1.5 px of slack.
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:16px", root);
  expect(concentricRadii([root])).toEqual({
    near: 1,
    bad: ["piece in outer, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0"],
  });
});

it("off the corner, lets a smaller radius be (E33)", () => {
  const root = outer();
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:8px", root);
  expect(concentricRadii([root])).toEqual({ near: 1, bad: [] });
});
```

- [ ] **Passo 2: la prova, prima della pagina**

In `gui/vite.config.ts` il progetto `browser` rifà il pacchetto delle dipendenze a ogni corsa (**E24**) — **prima** della
prova, perché la sua corsa rossa lascerebbe la cache senza `reka-ui`, e la prima corsa del Passo 5 cadrebbe per quello.
*Trova*:

```ts
      {
        extends: true,
        test: {
          name: "browser",
```

*Sostituisci con:*

```ts
      {
        extends: true,
        // ⛔ THE DEPENDENCIES ARE PRE-BUNDLED AFRESH ON EVERY RUN (E24 of the design-system plan). A cache left by a run
        // that never reached a library makes Vite re-bundle MID-RUN and reload, and the probes end up with two copies of
        // Vue: every one red with `'set' on proxy: trap returned falsish`, measured on 2026-09-25. `npm ci` empties the
        // cache, so the gate never sees it -- it bites whoever runs the probes by hand. Not `optimizeDeps.include`, the
        // list Vitest's warning suggests: it must grow with every library a probe reaches, and a forgotten name is red
        // only where the gate does not look. The cost, measured: none -- a run takes under 4 s, cold or warm; and Vite
        // 8.3.0's types mark the option `@experimental`, while its page documents it without reserve.
        optimizeDeps: { force: true },
        test: {
          name: "browser",
```

Crea `gui/src/kit/kit.browser.test.ts` (LF):

```ts
import { mount } from "@vue/test-utils";
import axe from "axe-core";
import { userEvent } from "vitest/browser";
import { afterEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import { violations } from "../testing/axe";
import { concentricRadii, firstFamily, fits, iconsCentred } from "../testing/probes";

import Kit from "./Kit.vue";

// ⛔ THE KIT PAGE IN THE INSTALLED CHROME (design system, section (f)): the probes of the boards, on the real pieces.

const ROOTS = ".kit-card, .kit-frame, .kit-strip";
const BOXES = ".kit-card, .kit-frame, .kit-strip, .base-button, .base-list-row, .base-text-field > .frame, .option, .base-dialog";

/** The kits the probes mounted: every one unmounted after its probe, red or green. */
const kits: { unmount(): void }[] = [];

afterEach(() => {
  // ⛔ UNMOUNTED HERE, NOT ON A PROBE'S LAST LINE (E25 of the plan): a probe that goes red stops before it, and the app
  // it leaves alive keeps an open window's `pointer-events: none` on the body -- the next probe that moves the real
  // pointer then times out, measured on 2026-09-25. The shape of the dock's and the frame's probes (tasks 6 and 8).
  for (const wrapper of kits.splice(0)) wrapper.unmount();
  document.body.replaceChildren();
});

async function kit(theme: "light" | "dark") {
  const wrapper = mount(Kit, { attachTo: document.body, props: { initialTheme: theme } });
  kits.push(wrapper);
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

/** A colour token as the page computes it -- `rgb(…)`, in the theme on the root -- never copied from the board. */
function colourOf(token: string): string {
  const probe = document.createElement("span");
  probe.style.color = `var(${token})`;
  document.body.append(probe);
  const colour = getComputedStyle(probe).color;
  probe.remove();
  return colour;
}

for (const theme of ["light", "dark"] as const) {
  describe(`the kit page, ${theme} theme`, () => {
    it("puts its theme on the root", async () => {
      await kit(theme);
      expect(document.documentElement.dataset.theme).toBe(theme);
    });

    it("keeps every radius concentric (answer 4)", async () => {
      await kit(theme);
      const report = concentricRadii(roots(ROOTS));
      // ⛔ NON-VACUITY (trap 1): a probe that met no corner near another is green for nothing.
      expect(report.near).toBeGreaterThan(0);
      expect(report.bad).toEqual([]);
      // ⛔ AND THE ONE CASE THAT JUDGES `BaseList` (E29 of the plan): its last row closes its card, in the corner. `near`
      // counts the whole page, so a stretched grid, or a note after the list, would take this case away and stay above 0.
      const list = document.querySelector(".kit-card:has(> .base-list)");
      expect(list).not.toBeNull();
      expect(concentricRadii([list as Element]).near).toBeGreaterThan(0);
    });

    it("cuts no text, and lets nothing stick out of its box", async () => {
      await kit(theme);
      const report = fits(roots(ROOTS), BOXES);
      expect(report.seen).toBeGreaterThan(0);
      expect(report.boxed).toBeGreaterThan(0);
      expect(report.problems).toEqual([]);
    });

    it("draws every icon in currentColor, and centres it", async () => {
      await kit(theme);
      const report = iconsCentred(roots(".kit"));
      expect(report.icons).toBeGreaterThan(0);
      expect(report.centred).toBeGreaterThan(0);
      expect(report.problems).toEqual([]);
    });

    it("dresses labels and numbers in Barlow, and the text in Geist", async () => {
      await kit(theme);
      const label = document.querySelector(".base-label");
      const text = document.querySelector(".base-list-row span");
      const number = document.querySelector(".kit em");
      expect(label !== null && text !== null && number !== null).toBe(true);
      expect(firstFamily(label as Element)).toBe("Barlow");
      expect(firstFamily(text as Element)).toBe("Geist Variable");
      expect(firstFamily(number as Element)).toBe("Barlow");
    });

    it("has no axe violation -- contrast included, on the drawn page", async () => {
      const wrapper = await kit(theme);
      expect(await violations(wrapper.element, { contrast: true })).toEqual([]);
      const judged = await contrastJudged(wrapper.element);
      expect(judged.passes).toBeGreaterThan(0);
      expect(judged.incomplete).toBe(0);
    });

    it("opens its window with the radii concentric, and no axe violation", async () => {
      await kit(theme);
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
    });

    it("draws every button that is off in the disabled colour, whatever its variant (E19)", async () => {
      await kit(theme);
      const off = [...document.querySelectorAll<HTMLElement>(".kit .base-button:disabled")];
      // ⛔ NON-VACUITY (trap 1): one button off per variant, and the probe meets all three. jsdom applies no sheet, so
      // only here can a rule that weighs more than `:disabled` show -- the quiet one did (E19 of the plan).
      expect(off.map((button) => button.dataset.variant).sort()).toEqual(["primary", "quiet", "secondary"]);
      const disabled = colourOf("--color-text-disabled");
      expect(off.map((button) => `${button.dataset.variant}: ${getComputedStyle(button).color}`)).toEqual(
        off.map((button) => `${button.dataset.variant}: ${disabled}`),
      );
    });

    it("keeps the error's border under the pointer (E20)", async () => {
      await kit(theme);
      const frame = document.querySelector<HTMLElement>(".kit .base-text-field > .frame[data-error]");
      expect(frame).not.toBeNull();
      const stop = colourOf("--color-border-stop");
      expect(getComputedStyle(frame as HTMLElement).borderTopColor).toBe(stop);
      await userEvent.hover(frame as HTMLElement);
      // ⛔ NON-VACUITY: a pointer that never arrived would leave the border as it was, and the probe green for nothing.
      expect((frame as HTMLElement).matches(":hover")).toBe(true);
      expect(getComputedStyle(frame as HTMLElement).borderTopColor).toBe(stop);
      await userEvent.unhover(frame as HTMLElement);
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

In `gui/eslint.config.js`, *Trova* — la fine del blocco di `Chat.vue`, così il blocco nuovo sta accanto all'altra
eccezione e sopra la testa delle regole d'import (**E28**):

```js
    rules: { "vue/no-v-html": "off" },
  },
```

*Sostituisci con:*

```js
    rules: { "vue/no-v-html": "off" },
  },
  {
    // ⛔ THE KIT PAGE'S WORDS ARE SPECIMENS (D8 of the design-system plan): a development page outside the package, whose
    // words in `it.json` would ship for nothing. The one exception to the raw-text rule, in one place, like Chat's above.
    name: "harness/kit-page-specimens",
    files: ["src/kit/**"],
    rules: { "@intlify/vue-i18n/no-raw-text": "off" },
  },
```

- [ ] **Passo 5: le prove, verdi — e che cosa dice un rosso**

```bash
(cd gui && npx vitest run --project browser && npm run lint && npm run build)
```

Atteso: il progetto `browser` **verde** — 27 prove in tre file: le cinque del compito 2, le diciotto della pagina kit, nove
per tema, e le quattro della sonda dei raggi a mano (**E30**, **E33**) —,
anche se la corsa rossa del Passo 2 ha lasciato la cache delle dipendenze senza `reka-ui` (**E24**). ⛔ Un rosso di
`concentricRadii` o di `fits`
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
# ⛔ NO OUTPUT OF A PREVIOUS BUILD: `npm ci` keeps `dist/`, and a build that wrote elsewhere -- or nothing -- would leave
# the checks below reading the old one (E31 of the design-system plan).
rm -rf dist
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
`bash scripts/gate-gui.sh` è **rosso** con *«the kit page is in the package»*; si toglie, ed è **verde**. E la guardia della
prima riga (**E31**): con `build: { outDir: "out", rolldownOptions: { input: { index: "index.html", kit: "kit.html" } } },`
e il `dist/` di un *build* precedente lasciato lì, `bash scripts/gate-gui.sh` è **rosso** con *«dist/index.html is missing:
the build produced nothing to check»* — senza il `rm -rf dist` le due righe leggerebbero il `dist/` vecchio e passerebbero
—; si toglie, e `gui/out/`, nato dalla prova, si cancella.

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
| in `BaseList.vue` il raggio delle righe `var(--radius-card)` al posto di `var(--radius-control)`, **e** `margin-inline: var(--space-1);` accanto | rosso: la prova della **pagina**, nei due temi — `base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0`: la riga è **fuori** dall'angolo, e la giudica l'altro ramo della sonda (**E27**). Con la sola `margin-inline` e il raggio giusto, **verde**: fuori dall'angolo un raggio più piccolo è ammesso |
| in `BaseButton.vue` tolta la regola del `quiet` spento, quella di **E19** | rosso: la prova dei pulsanti spenti, nei due temi — nello scuro `quiet: rgb(163, 154, 143)` dove la pagina vuole `rgb(111, 102, 96)` (**E25**) |
| in `BaseTextField.vue` `.frame:hover:not([data-disabled])`, senza la cura di **E20** | rosso: la prova del bordo dell'errore, nei due temi — nello scuro `expected 'rgb(163, 154, 143)' to be 'rgb(129, 27, 7)'` (**E25**) |
| in `kit.browser.test.ts` tolta la riga `await userEvent.hover(frame as HTMLElement);` | rosso: la guardia della stessa prova, nei due temi — `expected false to be true`: un puntatore che non arriva non lascia la prova verde per niente (**E25**) |
| in `Kit.vue` tolta `align-items: start;` di `.kit-grid` | rosso: la prova dei **raggi**, nei due temi — `expected 0 to be greater than 0`: la griglia stira le schede, l'ultima riga della lista esce dal suo angolo, e l'attesa sulla scheda della lista se ne accorge (**E29**) |
| in `probes.ts`, dentro `effective`, `radius(element, "TopLeft")` al posto di `radius(element, corner)` | rosso: le due prove a mano degli angoli diversi, e solo quelle — `expected { near: +0, bad: [] } to deeply equal { near: 1, …(1) }` e `expected { near: 3, bad: [ …(2) ] } to deeply equal { near: 1, bad: [] }` (**E30**) |
| in `BaseLabel.vue`, nella regola `.base-label :deep(.base-icon)`, `stroke: var(--color-text-stop);` accanto al `color` | rosso: la prova delle **icone**, nei due temi — `stroke is not currentColor: modules` e le altre icone delle etichette: l'attributo dice `currentColor`, il disegno no (**E32**) |
| in `probes.ts` il ramo fuori dall'angolo reso cieco, `: true;` al posto di `: inner <= outer - Math.min(dx, dy) + 1.5;` | rosso: la sola prova a mano che rifiuta fuori dall'angolo — `expected { near: 1, bad: [] } to deeply equal { near: 1, …(1) }` (**E33**) |

E il blocco del Passo 4, con `(cd gui && npm run lint)`, nelle due direzioni e sul suo confine (**E26**) — una riga per volta,
indietro con la copia salvata:

| La violazione | Atteso |
|---|---|
| in `eslint.config.js` tolto il blocco `harness/kit-page-specimens` | rosso: trentatré `@intlify/vue-i18n/no-raw-text` in `src/kit/Kit.vue`, da `raw text 'Il kit' is used` in giù — è il blocco a lasciar passare le parole della pagina |
| col blocco, in `BaseStatus.vue` una parola dentro la regione, `role="status">ciao<slot />` | rosso: `raw text 'ciao' is used` — il blocco si ferma alla sua cartella |
| col blocco, in cima allo `<script>` di `Kit.vue` `import { Search } from "lucide";` | rosso: *«'lucide' import is restricted from being used»* — il blocco spegne la sua regola, e nessun'altra |

- [ ] **Passo 8: guardarla, e il commit**

`(cd gui && npm run dev)`, poi `/kit.html` nel browser: i tre temi dalla scelta in cima, a grandezza vera; la tastiera sui
radio e sulla finestra. Poi la riga **4** della tabella della posizione — **Stato** `✅ <data>`, e nella riga **3** la colonna
**Commit** con l'hash del compito 3 (R1-16) —, `bash scripts/gate.sh` da solo,
`bash scripts/check-docs.sh`, il commit — `design-system(compito 4): la pagina kit …` — e `git push`.

---
## Compito 5: il kit al lavoro — i pezzi di base nei pannelli e nella cornice

**Da:** la (b), la tabella *«I pezzi di base»* — la colonna *«La seconda occorrenza»* dice dove va ciascuno — e *«Le regole,
come controlli del linter»*, le ultime due; la (e), la voce **M-3 di E187**; la (a), *«I due temi»*, la riga *«a mano»*; i
controlli **11–13**; la trappola **10**; **P-8** e **P-20** di questo piano; la decisione **21** del disegno.

**Files:**
- Rewrite: `gui/src/components/Confirm.vue`, `gui/src/frame/Drawer.vue`, `gui/src/frame/Band.vue`,
  `gui/src/panels/Settings.vue`, `gui/src/panels/Permissions.vue`, `gui/src/panels/Steps.vue`, `gui/src/frame/ViewBar.vue`
  — ciascuno **per intero**, col terminatore che ha oggi
- Modify: `gui/src/panels/Placeholder.vue` — due *Trova/Sostituisci*, il pulsante e il suo import (R3-15)
- Modify: `gui/src/panels/Status.vue` — la riga dell'evento dentro `BaseStatus`
- Modify: `gui/src/locales/it.json` — la scelta del tema; `gui/src/locales/copy.test.ts` — la sua prova (R3-14), e i due import in cima (P-20)
- Modify: `gui/src/panels/modules.test.ts`, `gui/src/a11y.test.ts` — le prove su `[role=radio]` e su `[role=dialog]`; e, con
  `gui/src/frame/frame.test.ts`, le tre prove di M-3 (R3-12)
- Create: `gui/src/panels/settings.browser.test.ts` — la via della tastiera sul radio, nel browser (R3-24)
- Modify: `gui/eslint.config.js` — le due regole su `panels/` e `frame/`
- Modify: `gui/src/tokens/dock.css` — il commento del ponte, che nominava i radio nativi di Impostazioni (**E36**)
- Modify: `gui/src/components/BaseDialog.vue` — la forma `sheet` tiene in vista le sue azioni (**E38**)
- Modify: `gui/src/frame/Frame.vue` — il contenitore del dock tiene per sé ciò che sborda (**E43**)

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

E Escape, che nessuna prova manda alla finestra (**E41**): in `gui/src/panels/modules.test.ts`, *Trova* — la fine del `describe("the confirmation window", …)`, col ruolo che lo script ha appena scritto:

```ts
    expect(core.pending).toBeNull();
    expect(document.querySelector('[role="dialog"]')).toBeNull();
    wrapper.unmount();
  });
});
```

*Sostituisci con:*

```ts
    expect(core.pending).toBeNull();
    expect(document.querySelector('[role="dialog"]')).toBeNull();
    wrapper.unmount();
  });

  it("takes Escape for a no: sends nothing, and closes (ADR-0016, E41)", async () => {
    const { bridge, core, invoke } = wire();
    const wrapper = mount(Confirm, { global: { plugins: [i18n] }, attachTo: document.body });
    invoke.send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    bridge.deliver("PermissionRequired");
    await nextTick();
    await nextTick();
    // ⛔ NON-VACUITY: the window is open, or Escape would have nothing to refuse.
    expect(document.querySelector('[role="dialog"]')).not.toBeNull();
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    await nextTick();
    await nextTick();
    await nextTick();
    expect(bridge.sent.map((message) => message.kind)).toEqual(["Invoke"]);
    expect(core.pending).toBeNull();
    expect(document.querySelector('[role="dialog"]')).toBeNull();
    wrapper.unmount();
  });
});
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
    // The core's welcome first: before it the group is off (E40).
    layout.receive({ kind: "Layout", value: { state: "Nothing" } });
    await nextTick();
    expect(layout.theme).toBe("system");
    const themeRadios = wrapper.findAll('[role="radiogroup"]')[1]?.findAll('[role="radio"]') ?? [];
    expect(themeRadios.map((radio) => radio.attributes("aria-checked"))).toEqual(["true", "false", "false"]);
    await themeRadios[2]?.trigger("click");
    await nextTick();
    expect(layout.theme).toBe("dark");
    expect(wrapper.findAll('[role="radiogroup"]')[1]?.findAll('[role="radio"]').map((radio) => radio.attributes("aria-checked"))).toEqual(["false", "false", "true"]);
  });

  it("keeps the theme off until the core's welcome: before it a choice would save a package without its layouts (E40)", async () => {
    wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    const layout = useLayout();
    const themeRadios = () => wrapper.findAll('[role="radiogroup"]')[1]?.findAll('[role="radio"]') ?? [];
    // ⛔ NON-VACUITY: the three choices are there, or "off" would hold of nothing.
    expect(themeRadios()).toHaveLength(3);
    for (const radio of themeRadios()) expect((radio.element as HTMLButtonElement).disabled).toBe(true);
    // ⛔ THE SECOND DIRECTION: the welcome -- a package, or none on a first run -- turns the group on.
    layout.receive({ kind: "Layout", value: { state: "Nothing" } });
    await nextTick();
    for (const radio of themeRadios()) expect((radio.element as HTMLButtonElement).disabled).toBe(false);
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
  // ⛔ NON-VACUITY (E37 of the design-system plan): the policy's two radios, or the arrow would have nowhere to go.
  expect(radios()).toHaveLength(2);
  await userEvent.click(radios()[0] as HTMLElement);
  // ⛔ THE SECOND DIRECTION: a click on the current value asks nothing.
  expect(bridge.sent).toEqual([]);
  // ⛔ THE KEY IS HELD, AS A HAND HOLDS IT: `reka-ui` 2.10.4 clicks the radio in a `setTimeout(0)` after the focus, and
  // only while an arrow is still down -- a `keydown` sets the flag, a `keyup` clears it. `{ArrowDown}` presses and
  // releases at once, and a `keyup` that arrived before the timer left nothing clicked: red in 1 run in 10 of the whole
  // suite, measured on 2026-09-23.
  await userEvent.keyboard("{ArrowDown>}");
  await expect.poll(() => bridge.sent.length).toBe(1);
  await userEvent.keyboard("{/ArrowDown}");
  expect(bridge.sent).toEqual([{ kind: "Invoke", value: { function: "vram-policy", argument: "local" } }]);
  // The focus sits on the radio the arrow reached; the check stays on the core's value until `Policy` comes back (P-8).
  expect(radios().map((radio) => radio.getAttribute("aria-checked"))).toEqual(["true", "false"]);
  expect(document.activeElement).toBe(radios()[1]);
  wrapper.unmount();
});
```

⚠️ **Richiamo del 2026-09-23, dalla scrittura del compito 7 (P-19):** il tasto si tiene premuto — `{ArrowDown>}`, l'`Invoke`
atteso, `{/ArrowDown}` —: premuto e rilasciato insieme, la prova cadeva una volta su dieci corse della suite intera.

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

Poi i due import, **in cima** e non dentro le prove (**P-20**) — *Trova*:

```ts
import { describe, expect, it } from "vitest";

import it_ from "./it.json";
```

*Sostituisci con:*

```ts
import { describe, expect, it } from "vitest";

// ⛔ AT THE TOP, NOT INSIDE A PROBE (P-20 of the design-system plan): an `await import` in a probe's body loads the
// module's whole graph inside the probe's 5 s -- the registry's is `vue`, `vue-i18n` and, from the design system on,
// the base pieces: from 0.5 to 5.2 s on 2026-09-24, and red twice for it. Here the file pays the load and each probe
// takes milliseconds: a guard that can go red for being slow guards nothing.
import { PANEL_TYPES } from "../panels/registry";
import { THEME_CHOICES } from "../tokens/theme";

import it_ from "./it.json";
```

*Trova:*

```ts
  it("has a name for every module type", async () => {
    const { PANEL_TYPES } = await import("../panels/registry");
```

*Sostituisci con:*

```ts
  it("has a name for every module type", () => {
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

  it("has a word for every theme choice", () => {
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

E la forma `sheet` di `gui/src/components/BaseDialog.vue` tiene in vista le sue azioni (**E38**): il cassetto aperto dalla tastiera dà il fuoco a «Chiudi», l'ultimo controllo, e `reka-ui` 2.10.4 lo dà con `preventScroll` — con le righe di `BaseList` il pulsante restava sotto il bordo del foglio. *Trova*:

```css
.actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-top: var(--space-1);
}
```

*Sostituisci con:*

```css
.actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-top: var(--space-1);
}
/* ⛔ THE SHEET KEEPS ITS ACTIONS IN VIEW (E38): `reka-ui` 2.10.4 gives the first control the focus with `preventScroll`, and
   in the drawer that control is "Chiudi", below a list longer than the sheet -- focused and out of sight. Stuck to the
   sheet's bottom edge: the negative offset and margin cover the sheet's own padding, where the rows would show, and the
   padding on top holds the focus ring, which would otherwise be drawn over the row passing under the bar. */
.base-dialog[data-variant="sheet"] .actions {
  position: sticky;
  bottom: calc(-1 * var(--space-3));
  margin: 0 0 calc(-1 * var(--space-3));
  padding: calc(var(--focus-width) + var(--focus-offset)) 0 var(--space-3);
  background: var(--color-bg-raised);
}
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

E in `gui/src/frame/Frame.vue` il contenitore del dock tiene per sé ciò che sborda (**E43**): quando la fascia entra, `dockview` 8.3.1 si ridimensiona un fotogramma dopo, e la sua griglia, alta di prima, arrivava alla pagina. *Trova*:

```css
.dock {
  flex: 1;
  min-height: 0;
}
```

*Sostituisci con:*

```css
.dock {
  flex: 1;
  min-height: 0;
  /* ⛔ THE DOCK KEEPS ITS SPILL TO ITSELF (E43): `dockview` 8.3.1 resizes one frame late -- its ResizeObserver hands the
     new size to a `requestAnimationFrame` -- so for a frame after the band comes in, the grid is as tall as before and
     spills out of this box; unclipped, the spill reached the page and flashed both its scrollbars. `clip` and not
     `hidden`: nothing may scroll this box either. */
  overflow: clip;
}
```

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
    <!-- The group and its region together: the empty region takes no room between the groups (E42). -->
    <div class="policy">
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
    </div>
    <!-- Off until the core's welcome (E40): before it the package is not ours to write, and a choice would save one without
         the layouts the core holds. -->
    <BaseRadioGroup
      :model-value="layout.theme"
      :options="themes"
      :legend="$t('settings.themeTitle')"
      :disabled="layout.arrivals === 0"
      @update:model-value="chooseTheme"
    />
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
.policy p {
  margin-top: var(--space-2);
}
.who {
  color: var(--color-text-muted);
}
</style>
```

E il commento del ponte in `gui/src/tokens/dock.css`, che spiega `color-scheme: inherit;` anche coi radio nativi di
Impostazioni, che da questo passo non ci sono più (**E36**, gotcha #58): la riga resta, per le barre di scorrimento. *Trova*:

```css
   line below, the native controls inside the dock -- scrollbars, the radios of Impostazioni -- are drawn dark under
   the light theme, and the radio NOT chosen looks chosen (E4 of the design-system plan). */
```

*Sostituisci con:*

```css
   line below, the native controls inside the dock -- the scrollbars -- are drawn dark under the light theme (E4 of the
   design-system plan; until its task 5 the radios of Impostazioni were native too, and the one NOT chosen looked
   chosen). */
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
  max-width: 24rem;
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
| in `panels/Strip.vue` un `<button type="button"></button>` nel template, **senza parole** (**E34**) | rosso, **un errore solo**: *«a button is BaseButton»* |
| in `frame/ViewBar.vue` un `<ul><li></li></ul>` nel template, **senza parole** (**E34**) | rosso, **un errore solo**: *«a list is BaseList»* |
| in `frame/moveActive.ts` la riga `import { DialogRoot } from "reka-ui";` | rosso: *«panels and frame use the base pieces»* |
| niente: `components/Confirm.vue` usa `BaseDialog`, e `components/BaseDialog.vue` importa `reka-ui` | verde |

- [ ] **Passo 8: M-3 col lettore di schermo vero — a mano (decisione 21)**

Un passo per il **proprietario**, col coordinatore, **dopo** la revisione e le sue cure, sul codice che resta (**E35**): né
l'implementatore né il revisore, due subagenti, sentono l'Assistente vocale, e l'implementatore **salta** questo passo.
`(cd gui && npm run dev)`, la pagina nel browser, l'**Assistente vocale** di Windows acceso (`Win + Ctrl + Invio`). Poi,
nella console:

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
posizione, che il coordinatore porta a `✅ <data>` nel commit della chiusura (**E35**). ⛔ Se un annuncio dei punti 3, 4 o 5
**non** si sente, è una voce d'errata: M-3 non è chiusa, e lo si dice, nella voce e nella cella.

- [ ] **Passo 9: il cancello e il commit**

Nella riga **4** della tabella della posizione la colonna **Commit** con l'hash del compito 4 (R1-16); la riga **5** resta
`⬜`, perché il Passo 8 viene dopo (**E35**); `bash scripts/gate.sh` da solo,
`bash scripts/check-docs.sh`, il commit — `design-system(compito 5): il kit al lavoro …` — coi fine-riga rimisurati, e
`git push`.

---
## Compito 6: il dock vestito — il tema nostro, le schede, la presa grande coi pezzi del kit

**Da:** la (c) del disegno, per intero; i controlli **15** e **16**; le trappole **5** e **9**; **P-6**, **P-7** e
**P-15**…**P-18** di questo piano; **D9**…**D11**; R3-17, R3-21, R3-22, R1-10 e A-2 della revisione; **E44**…**E48**
dell'errata, dal pre-controllo.

**Files:**
- Create: `gui/src/tokens/readToken.ts`, `gui/src/tokens/dock.test.ts`, `gui/src/frame/BigTabFace.vue`,
  `gui/src/frame/dock.browser.test.ts`
- Rewrite: `gui/src/tokens/dock.css`, `gui/src/frame/BigTab.ts`, `gui/src/frame/bigtab.test.ts`, `gui/src/jsdom-setup.ts` —
  ciascuno **per intero**, col terminatore che ha oggi
- Modify: `gui/src/frame/dock.ts`, `gui/src/frame/Frame.vue`, `gui/src/frame/frame.test.ts`,
  `gui/src/tokens/tokens.browser.test.ts`, `gui/src/testing/probes.ts` e `gui/src/testing/probes.browser.test.ts` (**E44**,
  **E46**); `gui/eslint.config.js` — un commento; dal Passo 8, `gui/src/panels/Placeholder.vue` e
  `gui/src/panels/modules.test.ts` (**E58**) e `gui/src/frame/ViewBar.vue` (**E57**)

**Interfaces:**
- Consumes: `shownTheme` di `tokens/theme.ts` (compito 1); il progetto `browser` (compito 2); `BaseLabel`, `BaseButton`,
  `isIconName` e `type IconName` (compito 3); `concentricRadii` di `testing/probes.ts` (compito 4), che il compito corregge
  (**E44**, **E46**).
- Produces: `readToken(name: string, element?: Element): string` da `tokens/readToken.ts` — il valore calcolato di un token,
  e un **errore** per un token che la pagina non definisce; `harnessTheme(): DockviewTheme` da `frame/dock.ts`; la classe
  `dockview-theme-harness`, che `tokens/dock.css` veste; `BigTabFace.vue` — props `title: string`, `icon?: IconName`, eventi
  `float` e `page`. Sotto jsdom, `base.css` caricato da `src/jsdom-setup.ts` (D9). `concentricRadii` legge il raggio che il
  CSS disegna, e non giudica ciò che arriva a un angolo attraverso una scatola che scorre (**E44**, **E46**).

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
 * ⛔ OUR THEME REPLACES THE ABYSS THEME, SO IT SETS WHAT THAT ONE SETS (design system, section (c); P-6 of the plan).
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

Crea `gui/src/frame/dock.browser.test.ts` (LF) — le schede, i raggi, la presa e il livello, nei due temi:

```ts
import "dockview/dist/styles/dockview.css";
import "../tokens";

import type { DockviewApi } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { userEvent } from "vitest/browser";

import { registerModules } from "../panels/modules";
import { concentricRadii } from "../testing/probes";
import { readToken } from "../tokens/readToken";

import { createDock } from "./dock";

// ⛔ THE DRESSED DOCK IN THE INSTALLED CHROME (design system, section (c)): what only a layout engine can judge -- the
// space between the cards, their radius and surface, the grab's height, the level and the surface of a floating group.
// The stylesheets are the SPA's own, in the order `main.ts` loads them: `dockview.css` first, our tokens after it.

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

/** The Status panel floated where the grab's first command puts it, and its container once laid out. */
async function floatStatus(host: HTMLElement, api: DockviewApi): Promise<Element> {
  const panel = api.getPanel("status");
  expect(panel).toBeDefined();
  if (panel !== undefined) api.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
  await new Promise((resolve) => setTimeout(resolve, 50));
  const floating = [...host.querySelectorAll(".dv-resize-container")];
  expect(floating).toHaveLength(1);
  return floating[0] as Element;
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
        // ⛔ THE NUCLEUS IS NOT A CARD (E58): the page shows through it, and its test is its own, below.
        if (group.querySelector(".nucleus") !== null) continue;
        expect(style.backgroundColor).toBe(computed("background-color", "--color-bg-surface"));
        // The border the dark theme draws, the shadow the light one casts (E52): tokens that are transparent or `none`
        // in the other theme, so each bites in its own.
        expect(style.borderTopColor).toBe(computed("border-top-color", "--color-border-card"));
        expect(style.boxShadow).toBe(computed("box-shadow", "--shadow-card"));
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

    it("paints nothing between the cards: the page shows through (E56)", async () => {
      const { host } = await dock(theme);
      // ⛔ FROM EVERY CARD UP TO THE HOST, NOTHING PAINTS (the owner at step 8 of task 6): `dockview.css` 8.3.1 paints the
      // whole grid in the cards' surface, a second background under them and around them, where the light cards sink.
      const painted = new Set<string>();
      let seen = 0;
      for (const group of [...host.querySelectorAll(".dv-groupview")]) {
        for (let element = group.parentElement; element !== null && element !== host; element = element.parentElement) {
          seen += 1;
          const { backgroundColor } = getComputedStyle(element);
          if (backgroundColor !== "rgba(0, 0, 0, 0)") painted.add(`${element.className}: ${backgroundColor}`);
        }
      }
      // ⛔ NON-VACUITY: the layers between the cards and the host were walked.
      expect(seen).toBeGreaterThan(0);
      expect([...painted]).toEqual([]);
    });

    it("draws the nucleus on the page, not as a card (E58)", async () => {
      const { host } = await dock(theme);
      const groups = [...host.querySelectorAll(".dv-groupview")];
      const nuclei = groups.filter((group) => group.querySelector(".nucleus") !== null);
      // ⛔ NON-VACUITY: the Home view has its nucleus, and one.
      expect(nuclei).toHaveLength(1);
      const style = getComputedStyle(nuclei[0] as Element);
      expect(style.backgroundColor).toBe("rgba(0, 0, 0, 0)");
      expect(style.borderTopColor).toBe("rgba(0, 0, 0, 0)");
      expect(style.boxShadow).toBe("none");
    });

    it("rounds the drop zone like the cards, a tab dragged for real (E55)", async () => {
      const { host, api } = await dock(theme);
      // ⛔ THE DROP ZONE LIVES ONLY WHILE A TAB IS DRAGGED, so an observer reads it the moment `dockview` draws it: with
      // the default mounting of 8.3.1 it is `.dv-drop-target-selection`, inside the target group's content container --
      // measured with a real mouse on 2026-09-26, and NOT the `.dv-drop-target-anchor` of the absolute mounting.
      const radii = new Set<string>();
      const observer = new MutationObserver(() => {
        for (const selection of host.querySelectorAll(".dv-drop-target-selection")) {
          radii.add(getComputedStyle(selection).borderTopLeftRadius);
        }
      });
      observer.observe(host, { childList: true, subtree: true, attributes: true });
      const tab = api.getPanel("permissions")?.group.element.querySelector(".dv-tabs-container > .dv-tab");
      const target = api.getPanel("activity")?.group.element.querySelector(".dv-content-container");
      expect(tab).toBeInstanceOf(HTMLElement);
      expect(target).toBeInstanceOf(HTMLElement);
      // ⛔ IN STEPS: with `dndStrategy: "pointer"` a drag starts past a threshold, and one jump from tab to target never
      // draws the zone -- measured, the guard below red. Playwright 1.63 interpolates the moves, and `vitest` hands it
      // the option.
      await userEvent.dragAndDrop(tab as HTMLElement, target as HTMLElement, { steps: 12 });
      observer.disconnect();
      // ⛔ NON-VACUITY: the drag drew a drop zone.
      expect(radii.size).toBeGreaterThan(0);
      // Concentric with the card it sits in: one border inside the card's edge (answer 4).
      const card = Number.parseFloat(computed("border-top-left-radius", "--radius-card"));
      // `width`, not `border-top-width`: a border with no style computes to 0, whatever its width says.
      const border = Number.parseFloat(computed("width", "--border-width"));
      expect([...radii]).toEqual([`${card - border}px`]);
    });

    it("keeps every radius of its own concentric, a floating group's too (answer 4)", async () => {
      const { host, api } = await dock(theme);
      const floating = await floatStatus(host, api);
      // A card too, with the tokens' radius -- not the 8px the spaced themes write under their own class (the (c)).
      expect(getComputedStyle(floating).borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
      // ⛔ NON-VACUITY ON THE CASE THAT JUDGES (trap 1, E29): in the Home view nothing the dock draws sits near a corner,
      // and what did -- the dot in a radio, a radio scrolled to the card's edge -- is not the dock's (E44). The floating
      // container is: its title bar in the two top corners, its group in the two bottom ones (E30, E46).
      const own = concentricRadii([floating]);
      expect(own.near).toBeGreaterThanOrEqual(4);
      expect(own.bad).toEqual([]);
      expect(concentricRadii([host]).bad).toEqual([]);
    });

    it("keeps the grab as tall as its strip, `--size-control-lg` (move 5 of SP-8, E45)", async () => {
      const { host } = await dock(theme);
      const grabs = [...host.querySelectorAll(".dv-tabs-container > .dv-tab .bigtab")].filter(
        (grab) => grab.getBoundingClientRect().width > 0,
      );
      // ⛔ NON-VACUITY: the Home view shows the tabs of five of its groups.
      expect(grabs.length).toBeGreaterThan(0);
      const tall = Number.parseFloat(computed("height", "--size-control-lg"));
      const heights = grabs.map((grab) => grab.getBoundingClientRect().height);
      expect(heights.filter((height) => Math.abs(height - tall) > 0.5)).toEqual([]);
    });

    it("floats a group BELOW the dialogs: at `--z-floating`, under `--z-overlay` (trap 9, R3-17)", async () => {
      const { host, api } = await dock(theme);
      const floating = await floatStatus(host, api);
      const level = Number(getComputedStyle(floating).zIndex);
      expect(level).toBe(Number(readToken("--z-floating")));
      expect(level).toBeLessThan(Number(readToken("--z-overlay")));
      // And raised, its tab too (the (c), E47): a tab left on the surface draws a darker block on the raised header.
      const raised = computed("background-color", "--color-bg-raised");
      for (const part of [".dv-tabs-and-actions-container", ".dv-tab"]) {
        const element = floating.querySelector(part);
        expect(element).not.toBeNull();
        expect(getComputedStyle(element as Element).backgroundColor).toBe(raised);
      }
    });

    it("keeps an always-rendered panel's content with its floating group, under the dialogs (E51)", async () => {
      const { host, api } = await dock(theme);
      // ⛔ A PANEL THE SPA DOES NOT HAVE YET, BROUGHT BY THE TEST: `renderer: "always"` keeps its content in an overlay of
      // its own, which `dockview.css` 8.3.1 gives the floating container's cycle (E51).
      api.addPanel({
        id: "always",
        component: "knowledge",
        renderer: "always",
        floating: { x: 100, y: 100, width: 400, height: 300 },
      });
      await new Promise((resolve) => setTimeout(resolve, 50));
      const overlays = [...host.querySelectorAll(".dv-render-overlay")];
      // ⛔ NON-VACUITY: the one panel that is rendered always.
      expect(overlays).toHaveLength(1);
      const level = Number(getComputedStyle(overlays[0] as Element).zIndex);
      expect(level).toBe(Number(readToken("--z-floating")) + 1);
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
    // ⛔ THE SHELL WEARS OUR CLASS, the one `tokens/dock.css` dresses -- and the abyss theme's is gone (control 15).
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

In `gui/src/testing/probes.browser.test.ts` (`replace_unique.py`), due sostituzioni — **E44** ed **E46**: la prova dei
raggi del dock giudica il contenitore galleggiante soltanto se la sonda legge il raggio che il CSS disegna e lascia stare
ciò che scorre, e ciascuna delle due cose ha qui la sua prova a mano. *Trova*:

```ts
// ⛔ THE RADIUS PROBE ON BOXES DRAWN BY HAND (E30 and E33 of the design-system plan): the kit page gives every piece four
// equal corners and puts it IN its corner, so two things would stay unproven there -- that each corner is read with its
// own radius, and the rule OFF the corner. ⛔ NON-VACUITY (trap 1): every case says the WHOLE report, `near` included,
// so a probe that met no corner cannot pass.
```

*Sostituisci con:*

```ts
// ⛔ THE RADIUS PROBE ON BOXES DRAWN BY HAND (E30 and E33 of the design-system plan): the kit page gives every piece four
// equal corners and puts it IN its corner, so two things would stay unproven there -- that each corner is read with its
// own radius, and the rule OFF the corner; and, from task 6, two more -- the radius a corner is DRAWN with (E46), and
// what reaches a corner through a box that scrolls (E44). ⛔ NON-VACUITY (trap 1): every case says the WHOLE report,
// `near` included, so a probe that met no corner cannot pass.
```

*Trova* — la fine del file:

```ts
it("off the corner, lets a smaller radius be (E33)", () => {
  const root = outer();
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:8px", root);
  expect(concentricRadii([root])).toEqual({ near: 1, bad: [] });
});
```

*Sostituisci con:*

```ts
it("off the corner, lets a smaller radius be (E33)", () => {
  const root = outer();
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:8px", root);
  expect(concentricRadii([root])).toEqual({ near: 1, bad: [] });
});

it("reads the radius a corner is DRAWN with: a short bar's r r 0 0 keeps it (E46)", () => {
  const root = outer();
  // 24 px tall, `20px 20px 0 0`, 1/1 inside the top corners: each side's two radii fit in the side, so CSS draws 20,
  // and the outer corner wants 19 -- in the probe's slack. Halving each corner on its own read 12.
  box("bar", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:20px 20px 0 0", root);
  expect(concentricRadii([root])).toEqual({ near: 2, bad: [] });
});

it("still shrinks the radii where CSS does: the same bar with four corners (E46)", () => {
  const root = outer();
  // Four corners of 20 in 24 px: each side would hold 40, CSS draws 12 -- and 12 is not 19.
  box("pill", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:20px", root);
  expect(concentricRadii([root])).toEqual({
    near: 2,
    bad: [
      "pill in outer, top-left: radius 12.0, outer 20.0, distance 1.0/1.0",
      "pill in outer, top-right: radius 12.0, outer 20.0, distance 1.0/1.0",
    ],
  });
});

it("does not judge what reaches the corner through a box that scrolls, and judges it while the box does not (E44)", () => {
  const root = outer();
  const scroller = box("scroller", "position:absolute;inset:0;overflow:auto", root);
  // Too round for the corner it sits in: 15 drawn, 20 wanted.
  box("piece", "position:absolute;left:0;bottom:0;width:60px;height:30px;border-radius:30px", scroller);
  expect(concentricRadii([root])).toEqual({
    near: 1,
    bad: ["piece in outer, bottom-left: radius 15.0, outer 20.0, distance 0.0/0.0"],
  });
  // The same piece in the same place, once the box holds more than it shows: where it lands is the scroll's.
  box("filler", "position:absolute;left:0;top:0;width:1px;height:400px", scroller);
  expect(concentricRadii([root])).toEqual({ near: 0, bad: [] });
});
```

```bash
(cd gui && npx vitest run --project jsdom src/tokens/dock.test.ts src/frame/frame.test.ts)
(cd gui && npx vitest run --project browser src/frame/dock.browser.test.ts src/testing/probes.browser.test.ts)
```

Atteso: **rosso**, e per le ragioni giuste. `dock.test.ts`: la guardia verde, e la prova che elenca **tutte** le variabili che
il tema nostro non imposta ancora — `expected [ …(43) ] to deeply equal []` il 2026-09-23. `frame.test.ts`: `expected false to
be true` sul guscio, che porta ancora `dockview-theme-abyss`, e `harnessTheme` che non esiste — `expected [Function] to throw
error matching /not a length in px/ but got '(0 , __vite_ssr_import_13__.harnessTh…'`. `dock.browser.test.ts`, nei due temi:
`expected '0px' to be '20px'` sulle schede e sul contenitore galleggiante, `expected [ 27, 27, 27, 27, 27 ] to deeply equal
[]` sulla presa — la striscia di `themeAbyss` —, ed `expected 999 to be 50` sul gruppo galleggiante — il difetto di R3-17
**già oggi**. `probes.browser.test.ts`: rosse le due prove che la sonda di oggi non sa, la barra — `expected { near: 2, bad:
[ …(2) ] } to deeply equal { near: 2, bad: [] }` (**E46**) — e ciò che scorre — `expected { near: 1, …(1) } to deeply
equal { near: +0, bad: [] }` (**E44**) —, e verde la pillola, che la sonda di oggi già riduce. Il 2026-09-26, sulla copia
del pre-controllo, dieci rosse su quindici nel browser. Un rosso per un'altra ragione è una voce d'errata. ⚠️ **Col
compito curato** — le prove di **E51**, **E55**, **E56** ed **E58**, sulla sequenza dei passi nel clone della revisione il
2026-09-26 —: nel browser `Tests  18 failed | 5 passed (23)`, in più per tema `expected 1000 to be 51`, `expected [
Array(1) ] to deeply equal []`, `expected [] to have a length of 1 but got +0` ed `expected [ '0px' ] to deeply equal [
'19px' ]`; sotto jsdom com'era.

- [ ] **Passo 4: il tema nostro — `dock.css`, `dock.ts`, il margine, i token sotto jsdom**

Riscrivi `gui/src/tokens/dock.css` per intero — il **ponte** del compito 1 esce (R1-10), le regole della presa grande
passano nella sua faccia (passo 5):

```css
/* OUR `dockview` THEME, `dockview-theme-harness` (design system, section (c); task 6 of its plan): `frame/dock.ts` puts the
   class on the dock's shell, and this sheet dresses it with our roles and tokens only -- no colour by hand (control 5,
   `usage.test.ts`). It sets every variable `dockview`'s abyss theme set, but the tab groups' and the palette's
   (`dock.test.ts`).

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

/* ⛔ AND THE TAB FILLS IT (E170, E173 of the part-2 plan; E45 of the design-system plan): no vertical padding on the
   strip's tabs, or the 0.25rem of `dockview.css` leaves a grab of 32 px in the strip of 40. The CHILD combinator, because
   the overflow dropdown renders `.dv-tab` rows inside the dock too. */
.dockview-theme-harness .dv-tabs-container > .dv-tab {
  padding-top: 0;
  padding-bottom: 0;
}

/* The card: the radius, the border the dark theme draws and the light one leaves transparent, the shadow the light one
   casts. `border-box`, or the border would add to the size `dockview` gives the group. */
.dockview-theme-harness .dv-groupview {
  box-sizing: border-box;
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
}

/* ⛔ ONLY THE CARDS PAINT (the owner at step 8 of task 6, E56): `.dv-dockview { background-color:
   var(--dv-group-view-background-color) }` in `dockview.css` 8.3.1 paints the whole grid in the cards' surface -- a
   second background under the cards and around them, up to the margin of `.dock`, where the light cards sink. Between
   the cards and around them, the page. */
.dockview-theme-harness .dv-dockview {
  background-color: transparent;
}

/* ⛔ THE NUCLEUS IS NOT A CARD (the owner at step 8 of task 6, E58): the north star's Home is the nucleus at the centre
   and the tiles around it, and the board draws it on the page, `.m-core` among the `.m-mod` cards. Its group paints
   nothing -- the border stays, transparent, so its content sits where a card's would. Found by what it holds, like the
   strip: `.nucleus` is the root of the nucleus, today `Placeholder.vue`'s for the module that plays it. */
.dockview-theme-harness .dv-groupview:has(.nucleus) {
  --dv-group-view-background-color: transparent;
  border-color: transparent;
  box-shadow: none;
}

/* ⛔ THE DROP ZONE IS ROUNDED LIKE THE CARDS (the owner at step 8 of task 6, E55): with the default mounting of 8.3.1 it
   is `.dv-drop-target-selection`, filling the target group's content -- or its half, over an edge --, one border inside
   the card's edge; so concentric, the card's radius less the border (answer 4). The `.dv-drop-target-anchor` that
   `dockview`'s spaced themes round belongs to `dndOverlayMounting: "absolute"`, which the SPA does not use. */
.dockview-theme-harness .dv-drop-target-selection {
  border-radius: calc(var(--radius-card) - var(--border-width));
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
  /* Raised, as the (c) wants -- its tabs too, or each draws a block of surface on the raised header (E47). */
  --dv-group-view-background-color: var(--color-bg-raised);
  --dv-tabs-and-actions-container-background-color: var(--color-bg-raised);
  --dv-activegroup-visiblepanel-tab-background-color: var(--color-bg-raised);
  --dv-activegroup-hiddenpanel-tab-background-color: var(--color-bg-raised);
  --dv-inactivegroup-visiblepanel-tab-background-color: var(--color-bg-raised);
  --dv-inactivegroup-hiddenpanel-tab-background-color: var(--color-bg-raised);
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
/* ⛔ AND THE RENDER OVERLAY, THE SAME CYCLE (E51): `.dv-render-overlay { --dv-overlay-z-index: var(--dv-overlay-z-index,
   999) }` in `dockview.css` 8.3.1, and `dockview-core` lifts the content of a `renderer: "always"` panel in a floating
   group to that level + 2i + 1 -- measured at 1000, over our dialogs, with its container at 50. No panel of the SPA is
   `always` today; the first that keeps a context alive, a 3D view for one, would float over the dialogs unseen. */
.dockview-theme-harness .dv-render-overlay {
  --dv-overlay-z-index: var(--z-floating);
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
    // ⛔ OUR THEME AND NOT `dockview`'s abyss theme, on which the eight moves were judged: that one
    // is dark only, and the design system has two themes (section (c), answers 5 and 18).
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
  /* ⛔ THE DOCK KEEPS ITS SPILL TO ITSELF (E43): `dockview` 8.3.1 resizes one frame late -- its ResizeObserver hands the
     new size to a `requestAnimationFrame` -- so for a frame after the band comes in, the grid is as tall as before and
     spills out of this box; unclipped, the spill reached the page and flashed both its scrollbars. `clip` and not
     `hidden`: nothing may scroll this box either. */
  overflow: clip;
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
  /* ⛔ THE DOCK KEEPS ITS SPILL TO ITSELF (E43): `dockview` 8.3.1 resizes one frame late -- its ResizeObserver hands the
     new size to a `requestAnimationFrame` -- so for a frame after the band comes in, the grid is as tall as before and
     spills out of this box; unclipped, the spill reached the page and flashed both its scrollbars. `clip` and not
     `hidden`: nothing may scroll this box either. */
  overflow: clip;
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

In `gui/src/testing/probes.ts` (`replace_unique.py`), tre sostituzioni — **E46** ed **E44**. *Trova*:

```ts
 * distance. A straight corner, inside or outside, is never compared (answer 20) -- a sheet's `r r 0 0` included. SVG
 * content is a drawing, not a surface.
 */
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
```

*Sostituisci con:*

```ts
 * distance. A straight corner, inside or outside, is never compared (answer 20) -- a sheet's `r r 0 0` included. SVG
 * content is a drawing, not a surface.
 *
 * ⛔ THE RADIUS A CORNER IS DRAWN WITH, NOT THE ONE WRITTEN (E46 of the design-system plan): where the two radii of a side
 * add up to more than the side, CSS shrinks ALL the radii of the box by one factor (css-backgrounds-3, section 4.5,
 * "Overlapping Curves") -- so a pill's 9999px is half its height, and a 24 px bar's `20px 20px 0 0` stays 20. Halving
 * each corner on its own read that bar, the dock's floating title bar, as 12.
 * ⛔ WHAT SCROLLS IS NOT PLACED (E44): an element that reaches its ancestor through a box whose content scrolls sits where
 * the scroll put it, not at a distance anyone drew -- the dock's Impostazioni, taller than its card, put a radio 19 px
 * from a corner. It is not judged; what does not scroll still is. ⚠️ ONLY A BOX BETWEEN THE TWO (E53): an ancestor
 * that scrolls ITSELF is still judged, at the scroll it has -- today no root the tests judge holds one scrolled, and the
 * dialog's sheet, which scrolls, has its top corners placed by its padding at 0.
 */
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
```

*Trova*:

```ts
  const effective = (element: Element, corner: Corner): number => {
    const box = element.getBoundingClientRect();
    return Math.min(radius(element, corner), box.height / 2, box.width / 2);
  };
```

*Sostituisci con:*

```ts
  const effective = (element: Element, corner: Corner): number => {
    const box = element.getBoundingClientRect();
    const [tl, tr, bl, br] = CORNERS.map((c) => radius(element, c)) as [number, number, number, number];
    const room = (side: number, radii: number): number => (radii > 0 ? side / radii : Number.POSITIVE_INFINITY);
    const across = Math.min(room(box.width, tl + tr), room(box.width, bl + br));
    const down = Math.min(room(box.height, tl + bl), room(box.height, tr + br));
    return radius(element, corner) * Math.min(1, across, down);
  };
  const scrolled = (element: Element, ancestor: Element): boolean => {
    for (let box = element.parentElement; box !== null && box !== ancestor; box = box.parentElement) {
      const style = getComputedStyle(box);
      const acrossScrolls = /auto|scroll/.test(style.overflowX) && box.scrollWidth > box.clientWidth + 1;
      const downScrolls = /auto|scroll/.test(style.overflowY) && box.scrollHeight > box.clientHeight + 1;
      if (acrossScrolls || downScrolls) return true;
    }
    return false;
  };
```

*Trova*:

```ts
      if (ancestor === null || !root.contains(ancestor)) continue;
```

*Sostituisci con:*

```ts
      if (ancestor === null || !root.contains(ancestor)) continue;
      if (scrolled(element, ancestor)) continue;
```

In `gui/src/panels/Placeholder.vue` (`replace_unique.py`) il nucleo lo dice alla sua radice, `.nucleus`, e il tema
non gli disegna la scheda (**E58**, dal Passo 8):

*Trova*:

```vue
const NUCLEUS = "knowledge";
</script>

<template>
  <section class="placeholder">
```

*Sostituisci con:*

```vue
// ⛔ AND ITS ROOT SAYS SO, `.nucleus`: the dock's theme draws no card around it (`tokens/dock.css`, E58).
const NUCLEUS = "knowledge";
</script>

<template>
  <section class="placeholder" :class="{ nucleus: params?.module === NUCLEUS }">
```

In `gui/src/panels/modules.test.ts` (`replace_unique.py`) la prova riconosce il segnaposto dalla classe letta come
parola, perché la radice del nucleo ne porta due (**E58**):

*Trova*:

```ts
    // fills it -- otherwise plugging five in would have to be checked in the browser.
    const unbuilt = PANEL_TYPES.filter((type) => !(type.name in MODULES));
    expect(unbuilt.length).toBeGreaterThan(0);
    for (const type of unbuilt) expect(drawn(type.name), type.name).toContain('class="placeholder"');
```

*Sostituisci con:*

```ts
    // fills it -- otherwise plugging five in would have to be checked in the browser. Its root carries the CLASS, and
    // the nucleus's carries `nucleus` too (E58): the class is read as a word, not as the attribute's whole text.
    const unbuilt = PANEL_TYPES.filter((type) => !(type.name in MODULES));
    expect(unbuilt.length).toBeGreaterThan(0);
    for (const type of unbuilt) expect(drawn(type.name), type.name).toMatch(/\bclass="placeholder\b/);
```

In `gui/src/frame/ViewBar.vue` (`replace_unique.py`) la barra è parte della pagina, com'è `.m-bar` nella tavola
(**E57**, dal Passo 8):

*Trova*:

```css
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-raised);
  border-bottom: var(--border-width) solid var(--color-border);
}
```

*Sostituisci con:*

```css
  padding: var(--space-2) var(--space-3);
  /* ⛔ NO SURFACE AND NO LINE UNDER IT (the owner at step 8 of task 6, E57): the bar is part of the page, as the board's
     `.m-bar` is. */
}
```

```bash
(cd gui && npx vitest run --project jsdom src/tokens src/frame && npx vitest run --project browser src/frame src/tokens src/testing)
```

Atteso: **verde** — P-6, le prove del dock sotto jsdom, le schede a `--space-3` col raggio della scheda, la presa alta
quanto la sua striscia (**E45**), i raggi concentrici del dock e del gruppo staccato (**E44**, **E46**), il gruppo
galleggiante a `--z-floating` e rialzato con la sua linguetta (**E47**), nei due temi, e le prove a mano della sonda; e
ancora verdi le prove di `bigtab.test.ts`, che il passo 5 riscrive. Il 2026-09-26, sulla copia del pre-controllo: jsdom,
otto file e 37 prove; browser, tre file e 21 prove. ⚠️ Col compito curato, il 2026-09-26: browser, tre file e **29**
prove (**E51**, **E55**, **E56**, **E58**); jsdom com'era.

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
 * ⛔ A PRESS ON A COMMAND MUST NOT START A DRAG, and stopping `click` alone is not enough: `dockview-core` 8.3.1 begins
 * its pointer drag on `pointerdown`, stopped here and held by `bigtab.test.ts` -- without it, every press of a command
 * drags the tile a few pixels first (SP-8). ⚠️ `mousedown` and `click` stop too, as SP-8 judged the grab with them,
 * and no test holds those two (E54): 8.3.1 does not listen to `mousedown` at all.
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
210,37 — `themeAbyss` esce, la faccia entra (**N-2**, la cifra per il proprietario). ⚠️ Col compito curato, il
2026-09-26: `Tests  185 passed | 1 skipped (186)`, e il pezzo `690.61 kB`, compresso `210.41 kB`.

- [ ] **Passo 7: le due direzioni**

Una violazione alla volta, poi indietro con la **copia salvata** e `cmp` (vincolo 11): `dock.css`, `BigTab.ts` e
`jsdom-setup.ts` il compito li ha riscritti, `probes.ts` e `Placeholder.vue` li ha cambiati, e `readToken.ts`, `dock.test.ts` e `BigTabFace.vue`
sono nati qui — `git
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
| `dock.browser.test.ts`, la distanza | in `dock.ts` `gap: 0 };` al posto di `` gap: pixels("--space-3") }; `` | rosso, nei due temi: `expected [ Array(10) ] to deeply equal []`, le distanze a 0; e sotto jsdom la prova del `gap` in px di `frame.test.ts`, `expected [Function] to throw an error`: il `gap` scritto a mano non legge il token (**E49**) |
| `dock.browser.test.ts`, i raggi | in `dock.css`, prima del commento *«The mark of the visible tab»*, `.dockview-theme-harness .dv-tabs-and-actions-container { border-radius: var(--radius-control); }` | rosso, nei due temi, l'ultima attesa: `expected [ …(8) ] to deeply equal []`, fra le voci `dv-tabs-and-actions-container in dv-groupview, top-left: radius 8.0, outer 20.0, distance 1.0/1.0` e il suo `top-right` |
| `dock.browser.test.ts`, il livello | in `dock.css` tolta, dal blocco `.dockview-theme-harness .dv-resize-container`, la riga `--dv-overlay-z-index: var(--z-floating);` | rosso, nei due temi: `expected 999 to be 50` — R3-17 |
| `dock.browser.test.ts`, la presa | in `dock.css`, nella regola `.dockview-theme-harness .dv-tabs-container > .dv-tab`, tolte le due righe del `padding` | rosso, nei due temi: `expected [ 32, 32, 32, 32, 32 ] to deeply equal []` — **E45** |
| `dock.browser.test.ts`, il raggio del contenitore | in `dock.css`, dal blocco `.dockview-theme-harness .dv-resize-container`, tolta `border-radius: var(--radius-card);` | rosso, nei due temi: `expected '0px' to be '20px'` — **E44** |
| `dock.browser.test.ts`, la barra del titolo | in `dock.css` tolta la regola `.dockview-theme-harness .dv-resize-container > .dv-floating-titlebar` | rosso, nei due temi: `expected 2 to be greater than or equal to 4` — dei quattro angoli del contenitore ne restano due, e la guardia lo vede (**E44**) |
| `dock.browser.test.ts`, il fondo delle linguette | in `dock.css`, dal blocco `.dockview-theme-harness .dv-resize-container`, tolte le quattro righe `--dv-…-tab-background-color` | rosso, nello scuro: `expected 'rgb(27, 23, 24)' to be 'rgb(36, 31, 32)'` — ⚠️ nel chiaro i due ruoli hanno lo stesso valore, e quella metà non può mordere (**E47**) |
| `probes.browser.test.ts`, il raggio disegnato | in `probes.ts` `return Math.min(radius(element, corner), box.height / 2, box.width / 2);` al posto di `return radius(element, corner) * Math.min(1, across, down);` | rosso: la prova della barra, `expected { near: 2, bad: [ …(2) ] } to deeply equal { near: 2, bad: [] }`, e nei due temi quella dei raggi del dock, `expected [ …(2) ] to deeply equal []` — la barra del titolo letta 12 (**E46**) |
| `probes.browser.test.ts`, la riduzione | la stessa riga sostituita da `return radius(element, corner);` | rosso: la prova della pillola, `expected { near: 2, bad: [] } to deeply equal { near: 2, bad: [ …(2) ] }`, quella di ciò che scorre, il cui pezzo è una pillola, e nei due temi quella dei raggi del dock, `expected [ …(24) ] to deeply equal []` (**E46**); e nei due temi quella dei raggi della pagina kit, `expected [ …(24) ] to deeply equal []`, i radio e le pillole letti 9999 (**E49**) |
| `probes.browser.test.ts`, ciò che scorre | in `probes.ts` tolta la riga `if (scrolled(element, ancestor)) continue;` | rosso: la sola prova di ciò che scorre, `expected { near: 1, …(1) } to deeply equal { near: +0, bad: [] }` — ⚠️ quella del dock resta verde: col gruppo staccato Impostazioni non scorre più (**E44**) |
| `dock.browser.test.ts`, il livello del contenuto | in `dock.css` tolta la regola `.dockview-theme-harness .dv-render-overlay` | rosso, nei due temi, il 2026-09-26: `expected 1000 to be 51` (**E51**) |
| `dock.browser.test.ts`, il bordo della scheda | in `dock.css`, dalla regola `.dockview-theme-harness .dv-groupview`, tolta la riga `border: …` | rosso, nei due temi, il 2026-09-26: `expected 'rgb(27, 23, 24)' to be 'rgba(0, 0, 0, 0)'` nel chiaro ed `expected 'rgb(236, 230, 218)' to be 'rgb(47, 40, 41)'` nello scuro — senza il bordo il colore torna `currentColor` (**E52**) |
| `dock.browser.test.ts`, l'ombra della scheda | dalla stessa regola tolta `box-shadow: var(--shadow-card);` | rosso nel solo chiaro, il 2026-09-26: `expected 'none' to be 'rgba(29, 23, 24, 0.06) 0px 1px 2px 0p…'` — ⚠️ nello scuro `--shadow-card` è `none`, e quella metà non può mordere (**E52**) |
| `dock.browser.test.ts`, la zona d'arrivo | in `dock.css` tolta la regola `.dockview-theme-harness .dv-drop-target-selection` | rosso, nei due temi, il 2026-09-26: `expected [ '0px' ] to deeply equal [ '19px' ]` (**E55**) |
| `dock.browser.test.ts`, la pagina fra le schede | in `dock.css` tolta la regola `.dockview-theme-harness .dv-dockview` | rosso, nei due temi, il 2026-09-26: `expected [ Array(1) ] to deeply equal []`, la voce `dv-grid-view dv-dockview` (**E56**) |
| `dock.browser.test.ts`, il nucleo | in `dock.css` tolta la regola `.dockview-theme-harness .dv-groupview:has(.nucleus)` | rosso, nei due temi, il 2026-09-26: `expected 'rgb(251, 248, 243)' to be 'rgba(0, 0, 0, 0)'` ed `expected 'rgb(27, 23, 24)' to be 'rgba(0, 0, 0, 0)'` (**E58**) |
| `dock.browser.test.ts`, il segno del nucleo | in `Placeholder.vue` tolto `:class="{ nucleus: params?.module === NUCLEUS }"` | rosso, nei due temi, il 2026-09-26: `expected [] to have a length of 1 but got +0` (**E58**) |

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
schede, le linguette col segno, i divisori che compaiono al passaggio, la zona d'arrivo trascinando una linguetta; il gruppo
staccato, raggio e fondo della sua linguetta compresi — **E47** si vedeva solo nello scuro —; e le barre di scorrimento dei
pannelli che scorrono, Stato e Impostazioni nella Home, che arrivano all'angolo tondo della scheda, e l'arco ne taglia il
binario (**E48**). ⚠️ Chi guarda con Playwright senza finestra lancia Chrome con `ignoreDefaultArgs: ["--hide-scrollbars"]`:
senza, le barre non si vedono. ⛔ Il
bersaglio è la Home della tavola dello stile, e l'aspetto lo **giudica il proprietario** alla prima prova (controllo 15): ciò
che non gli piace è una voce d'errata col suo *«perché»*, non un ritocco di chi esegue.

✅ **Il 2026-09-26 il proprietario, nel suo Chrome, con quattro foto:** la zona d'arrivo ad angoli dritti (**E55**), due
sfondi — la griglia di `dockview` del colore delle schede, e nel chiaro le schede che vi sprofondavano (**E56**) —, la barra
con un fondo suo e una riga sotto (**E57**), e il nucleo vestito da scheda (**E58**): curati nel commit delle cure, e il
secondo sguardo dopo. Le radio del tema erano spente perché il benvenuto del core finto non era consegnato:
`harnessFake.deliverAll()`, com'è scritto qui sopra — **E40** del compito 5, non un difetto.

- [ ] **Passo 9: il cancello, il commit, la posizione**

La riga **6** della tabella della posizione — **Stato** `✅ <data>`, e nella riga **5** la colonna **Commit** con l'hash del
compito 5 (R1-16) —; `bash scripts/gate.sh` da solo, `bash scripts/check-docs.sh`, il commit — `design-system(compito 6): il
dock vestito …`, con le due righe del pezzo JavaScript — coi fine-riga rimisurati, e `git push`.

---

## Compito 6bis: la cura delle tre voci del Passo 8 — il bordo nei raggi, la barra di scorrimento, i messaggi

**Da:** **E59**, **E60** ed **E61** dell'errata, coi loro richiami; la (a), la (b) — *«`BaseNotice`»* —, la (d) — *«La fascia»* —
e la (f) del disegno, coi richiami del 2026-09-27; i controlli **22** e **23**; le trappole **21**–**28**; le due risposte del
proprietario del 2026-09-27, alla scrittura di questo compito — la cura è un **compito nuovo**, A, e con un'azione la riga del
messaggio sta **al centro**, B —; **D25**…**D29**; **E62** e **E63**.

⛔ **È il Passo 8 del compito 6 che continua**: il proprietario non l'ha approvato, e le tre voci che ha deciso le cura questo
compito. La riga **6** della posizione si chiude **con lo sguardo di questo compito** (Passo 16), non prima.

**Files:**
- Create: `gui/src/components/BaseNotice.vue`, `gui/src/frame/frame.browser.test.ts` (**E63**: il compito 8 lo estende)
- Rewrite: `gui/src/frame/Band.vue`, **per intero**, col terminatore che ha oggi
- Modify: `docs/superpowers/specs/2026-09-22-design-system-tavole/token.html` — da `cure_board.py` —, e `gui/src/tokens/base.css`
  — dalla tavola, con `extract_tokens.py`; `gui/src/testing/probes.ts` e `gui/src/testing/probes.browser.test.ts`;
  `gui/vite.config.ts`; `gui/src/tokens/tokens.browser.test.ts`; `gui/src/tokens/dock.css`; `gui/src/components/BaseDialog.vue`, un commento (**E66**); `gui/src/components/icons.ts` e
  `gui/src/components/kit.test.ts`; `gui/src/kit/Kit.vue` e `gui/src/kit/kit.browser.test.ts`; `gui/src/panels/Settings.vue`,
  `gui/src/panels/Status.vue`, `gui/src/panels/modules.test.ts` e `gui/src/frame/frame.test.ts`

**Interfaces:**
- Consumes: `readToken` (compito 6); `concentricRadii` e `fits` di `testing/probes.ts` (compiti 4 e 6); `BaseIcon`,
  `BaseButton`, `BaseStatus` e la mappa `ICONS` (compito 3); la pagina kit e le sue prove (compito 4); la fascia, Impostazioni e
  Stato sui pezzi di base (compito 5); il tema del dock (compito 6); `extract_tokens.py` (compito 1, qui per intero).
- Produces: `BaseNotice.vue` — props `tone: "info" | "ok" | "warn" | "stop"`, `title: string`, `description?: string`,
  `onPage?: boolean`; lo slot `action`; sull'elemento `data-tone`, `data-on-page` e `data-action`, e le classi `.title` e
  `.description`. In `ICONS` le voci `info`, `ok`, `warn` e `stop`. I token `--size-scrollbar` e `--scrollbar-trigger`, e i
  raggi `--radius-card` 21 e `--radius-frame` 34. In `concentricRadii` uno scarto di **mezzo pixel**. Il progetto browser coi
  bordi delle barre **mostrati**. `frame.browser.test.ts` con gli aiutanti `frame(theme)`, `px(token)` e `computed(property,
  token)`, che il compito 8 riusa.

⚠️ **Che cosa la scrittura di questo compito ha misurato**, il 2026-09-27 nel Chrome installato 154 e sotto jsdom, su una
cartella di prova — un `git worktree` di `950bba1` nello scratchpad, col codice di questo compito applicato passo per passo:

| | Il fatto | Che cosa ne fa il compito |
|---|---|---|
| 1 | coi token di oggi e la sonda a mezzo pixel le coppie storte sono **tre**: le righe di `BaseList` nelle schede della pagina kit (8 a 13 px da 20), **il pulsante nell'angolo della finestra di `BaseDialog`** (lo stesso pixel, e nessuno l'aveva visto) e la barra del titolo e il gruppo del contenitore staccato del dock (20 a 1 px da 20) | il Passo 3 le fa vedere rosse; i token nuovi curano le prime due, e il Passo 7 la terza |
| 2 | coi token nuovi le coppie storte sono di nuovo due: la cornice della pagina kit — 34, senza bordo, attorno a carte da 21 a 12 px — e il dock | il Passo 7: il bordo trasparente di `.kit-frame`, e la barra e il gruppo a `--radius-card` meno il bordo |
| 3 | due prove della sonda **si reggevano sullo scarto di 1,5**: *«off the corner, lets a smaller radius be»* — un raggio di 8 dove il limite è 7 — e *«reads the radius a corner is DRAWN with»* — una barra da 20 a 1 px dentro un angolo da 20 | il Passo 2: 7 e 19, e una prova nuova per il pixel del bordo, rossa a 1,5 |
| 4 | abbassare a mezzo pixel **anche** la soglia di *«nell'angolo»* non cambia nessun verdetto delle prove | **D25**: una costante sola, `SLACK` |
| 5 | `ignoreDefaultArgs: ["--hide-scrollbars"]` nelle `launchOptions` di `playwright()` **arriva** al Chrome delle prove: lo spessore di una barra è 0 con le barre nascoste, 15 senza la regola, 10 con la regola; e **nessun'altra prova cambia** mostrandole | **D26**: le barre mostrate a tutto il progetto |
| 6 | lo script della tavola rende la **copia approvata** del 2026-09-27 — `diff` contro `look/tavola-tre-voci-innesco.html` dello scratchpad della macchina `Jays` —, tranne il titolo, il foglio che presentava le tre voci e la riga che la seconda risposta cambia: `.msg:has(>.btn){align-items:center}` al posto di `.msg>.btn{align-self:center}` | la tavola del repository porta ciò che il proprietario ha guardato, e la sua scelta del 2026-09-27 |
| 7 | col messaggio in alto, com'era la v3, titolo e icona stanno **2 px** sopra il centro di «Riprova», a densità 1 e a 1,75 — lo stesso carattere, 600 14/20, in una riga da 24; con la riga al centro, **0** | **D27**; la prova del Passo 10 |
| 8 | la suite coi Passi 2–13: `Test Files  25 passed \| 1 skipped (26)`, `Tests  200 passed \| 1 skipped (201)` — erano 24 e 185; il pezzo JavaScript `691.82 kB`, compresso `210.77 kB` — era `690.61 kB` e `210.41 kB`; *build* e linter puliti | le attese del Passo 14 |

- [ ] **Passo 1: rimisura il punto di partenza**

```bash
git status --porcelain > <scratchpad>/prima.txt
grep -n -e 'radius-card:' -e 'radius-frame:' gui/src/tokens/base.css
grep -c -i 'scrollbar' gui/src/tokens/base.css docs/superpowers/specs/2026-09-22-design-system-tavole/token.html
grep -n 'launchOptions' gui/vite.config.ts
grep -c '1.5' gui/src/testing/probes.ts
ls gui/src/components/BaseNotice.vue gui/src/frame/frame.browser.test.ts
(cd gui && npm run build 2>&1 | grep -E 'assets/index-.*\.js ')
```

Atteso: i due raggi `/* 20 */` e `/* 32 */`, senza `--border-width`; **0** e **0** per la barra; `launchOptions: { channel:
"chrome" }` e basta; **2** righe con `1.5` in `probes.ts`, le due soglie; `No such file or directory` per i due file nuovi — la
quarta domanda del pre-controllo —; il pezzo JavaScript, la baseline del compito: `690.61 kB` il 2026-09-27. Poi, da solo, `bash
scripts/gate.sh` → `GATE GREEN`. ⚠️ Se il compito 7 o l'8 sono già eseguiti, `frame.browser.test.ts` esiste: il compito si
rilegge contro il codice di allora (**E63**).

- [ ] **Passo 2: la sonda — le sue prove, a mezzo pixel (E61)**

In `gui/src/testing/probes.browser.test.ts` (`replace_unique.py`), tre sostituzioni.
*Trova* — il commento della prova fuori dall'angolo:

```ts
it("off the corner, refuses a piece rounder than the outer radius minus the smaller distance (E33)", () => {
  const root = outer();
  // 17/13 from the bottom-left corner: at most 20 - 13, with the probe's 1.5 px of slack.
```

*Sostituisci con:*

```ts
it("off the corner, refuses a piece rounder than the outer radius minus the smaller distance (E33)", () => {
  const root = outer();
  // 17/13 from the bottom-left corner: at most 20 - 13, with the probe's half pixel of slack (E61).
```

*Trova* — il raggio più piccolo, che era 8 dentro un limite di 7:

```ts
it("off the corner, lets a smaller radius be (E33)", () => {
  const root = outer();
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:8px", root);
```

*Sostituisci con:*

```ts
it("off the corner, lets a smaller radius be (E33)", () => {
  const root = outer();
  // At most 20 - 13, and the bound itself passes. It was 8 here, green only by the old slack of 1.5 (E61).
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:7px", root);
```

*Trova* — la barra corta, e il pixel del bordo:

```ts
it("reads the radius a corner is DRAWN with: a short bar's r r 0 0 keeps it (E46)", () => {
  const root = outer();
  // 24 px tall, `20px 20px 0 0`, 1/1 inside the top corners: each side's two radii fit in the side, so CSS draws 20,
  // and the outer corner wants 19 -- in the probe's slack. Halving each corner on its own read 12.
  box("bar", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:20px 20px 0 0", root);
  expect(concentricRadii([root])).toEqual({ near: 2, bad: [] });
});
```

*Sostituisci con:*

```ts
it("reads the radius a corner is DRAWN with: a short bar's r r 0 0 keeps it (E46)", () => {
  const root = outer();
  // 24 px tall, `19px 19px 0 0`, 1/1 inside the top corners: each side's two radii fit in the side, so CSS draws 19,
  // which is what the outer corner wants. Halving each corner on its own read 12.
  box("bar", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:19px 19px 0 0", root);
  expect(concentricRadii([root])).toEqual({ near: 2, bad: [] });
});

it("sees the pixel of a border: the same bar at the outer radius is one pixel too round (E61)", () => {
  const root = outer();
  // What the dock's floating title bar was: the card's radius, 1 px inside the card's edge. The old slack of 1.5 kept it
  // green; half a pixel is under the smallest defect the rule wants to see.
  box("bar", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:20px 20px 0 0", root);
  expect(concentricRadii([root])).toEqual({
    near: 2,
    bad: [
      "bar in outer, top-left: radius 20.0, outer 20.0, distance 1.0/1.0",
      "bar in outer, top-right: radius 20.0, outer 20.0, distance 1.0/1.0",
    ],
  });
});
```

```bash
(cd gui && npx vitest run --project browser src/testing/probes.browser.test.ts)
```

Atteso: **rossa** la sola prova nuova, `expected { near: 2, bad: [] } to deeply equal { near: 2, bad: [ …(2) ] }`: con lo scarto
di 1,5 il pixel del bordo passa; `Tests  1 failed | 7 passed (8)`.

- [ ] **Passo 3: la sonda a mezzo pixel**

In `gui/src/testing/probes.ts` (`replace_unique.py`), due sostituzioni.
*Trova* — la testa di `concentricRadii`:

```ts
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
  const CORNERS = ["TopLeft", "TopRight", "BottomLeft", "BottomRight"] as const;
```

*Sostituisci con:*

```ts
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
  // ⛔ HALF A PIXEL OF SLACK (E61 of the design-system plan): a slack is chosen tighter than the smallest defect the rule
  // wants to see, and that defect is the pixel of a border -- 1.5 kept green a card's radius 1 px inside the card's
  // edge, on the kit page and in the dock's floating group. The same half pixel decides "in the corner": two distances
  // further apart cannot share a centre, and the rule off the corner judges them (D25).
  const SLACK = 0.5;
  const CORNERS = ["TopLeft", "TopRight", "BottomLeft", "BottomRight"] as const;
```

*Trova* — le tre soglie:

```ts
        const inTheCorner = Math.abs(dx - dy) <= 1.5;
        const ok = inTheCorner ? Math.abs(inner - (outer - dx)) <= 1.5 : inner <= outer - Math.min(dx, dy) + 1.5;
```

*Sostituisci con:*

```ts
        const inTheCorner = Math.abs(dx - dy) <= SLACK;
        const ok = inTheCorner ? Math.abs(inner - (outer - dx)) <= SLACK : inner <= outer - Math.min(dx, dy) + SLACK;
```

```bash
(cd gui && npx vitest run --project browser src/testing/probes.browser.test.ts src/kit/kit.browser.test.ts src/frame/dock.browser.test.ts)
```

Atteso: le prove della sonda **verdi**; **rosse**, nei due temi, le tre coppie che i token di oggi storcono di un pixel (la riga 1
della tabella qui sopra) — nella pagina kit *«keeps every radius concentric»*, le voci `base-list-row in kit-card, bottom-left:
radius 8.0, outer 20.0, distance 13.0/13.0` e il suo `bottom-right`, e *«opens its window with the radii concentric»*, la voce
`base-button in base-dialog, bottom-right: radius 8.0, outer 20.0, distance 13.0/13.0`; nel dock *«keeps every radius of its own
concentric, a floating group's too»*, quattro voci, `dv-floating-titlebar in dv-resize-container, top-left: radius 20.0, outer
20.0, distance 1.0/1.0` coi suoi tre compagni — `Tests  6 failed | 36 passed (42)`. È il rosso che **E61** chiedeva di vedere prima
della cura.

- [ ] **Passo 4: la barra — la prova (E59, controllo 22)**

In `gui/src/tokens/tokens.browser.test.ts` (`replace_unique.py`), una sostituzione.
*Trova* — la fine del file:

```ts
    // ⛔ THE SECOND DIRECTION: a token the page does not define is an error, not "" -- a `gap` of NaN is what "" became.
    expect(() => readToken("--space-that-is-not")).toThrow(/is not defined here/);
  });
});
```

*Sostituisci con:*

```ts
    // ⛔ THE SECOND DIRECTION: a token the page does not define is an error, not "" -- a `gap` of NaN is what "" became.
    expect(() => readToken("--space-that-is-not")).toThrow(/is not defined here/);
  });

  it("draw every scroll bar `--size-scrollbar` thick, and light its trigger under the pointer and with the focus inside (control 22, E59)", async () => {
    const box = document.createElement("div");
    box.style.cssText = "width:200px;height:100px;overflow:auto";
    box.tabIndex = 0;
    const content = document.createElement("div");
    content.style.cssText = "width:400px;height:400px";
    box.append(content);
    document.body.append(box);
    // ⛔ NON-VACUITY: the box scrolls both ways, so there are two bars to measure.
    expect(box.scrollHeight).toBeGreaterThan(box.clientHeight);
    expect(box.scrollWidth).toBeGreaterThan(box.clientWidth);
    // ⛔ THE THICKNESS THE ENGINE DRAWS, against the token: 0 while the project hid the bars (trap 25 of the design), 15
    // without our rule -- Chrome's own -- measured on 2026-09-27.
    const thick = Number.parseFloat(readToken("--size-scrollbar"));
    expect({ vertical: box.offsetWidth - box.clientWidth, horizontal: box.offsetHeight - box.clientHeight }).toEqual({
      vertical: thick,
      horizontal: thick,
    });
    // ⛔ THE TRIGGER (E59): Chromium redraws a custom bar only when the box's OWN style changes, so the rule sets a
    // registered property on it. The thumb that lights up is the owner's to see; this reads that the box's style moves.
    const trigger = (): string => getComputedStyle(box).getPropertyValue("--scrollbar-trigger").trim();
    expect(trigger()).toBe("0");
    await userEvent.hover(box);
    expect(trigger()).toBe("1");
    await userEvent.unhover(box);
    expect(trigger()).toBe("0");
    box.focus();
    expect(trigger()).toBe("1");
  });
});
```

```bash
(cd gui && npx vitest run --project browser src/tokens/tokens.browser.test.ts)
```

Atteso: **rossa**, `Error: the token --size-scrollbar is not defined here: are the token sheets loaded?`.

- [ ] **Passo 5: la tavola, poi la sua copia (E59, E60, E61)**

⛔ **La tavola prima, la copia dopo** (vincolo 2): `board.test.ts` vuole `base.css` e `themes.css` uguali byte per byte ai blocchi
della tavola. Ciò che lo script scrive è la copia che il proprietario ha approvato il 2026-09-27 — la tabella di *«Come si
riprende»* della chiusura di quel giorno, in archivio —, con la riga al centro della sua seconda risposta; la riga 6 della tabella
qui in testa dice come è stato verificato.

Scrivi `cure_board.py` nello **scratchpad**, non nel repository:

```python
"""cure_board.py -- the three items of step 8 of task 6 in the approved token board (design system, task 6bis).

Usage: python cure_board.py <repository root>

E61, the border in the radii; E59, the scroll bar with its trigger; E60, the neutral tone and the message on the page,
its row centred when it has an action. What the owner approved on 2026-09-27 on a copy of the board -- in the
scratchpad of the machine `Jays`, not tracked -- and the second question of the cure, the same day. Every hunk is
refused unless its anchor is there exactly as many times as said; the board keeps its line endings.
"""
import io
import os
import sys

root = sys.argv[1]
path = os.path.join(root, "docs", "superpowers", "specs", "2026-09-22-design-system-tavole", "token.html")
raw = io.open(path, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw
board = raw.replace("\r\n", "\n")


def once(old, new, times=1):
    global board
    n = board.count(old)
    if n != times:
        sys.exit(f"refused: {n} occurrences, not {times}, of {old[:70]!r}")
    board = board.replace(old, new)


def after_lines(test, insert, times):
    global board
    lines = board.split("\n")
    hits = [i for i, line in enumerate(lines) if test(line)]
    if len(hits) != times:
        sys.exit(f"refused: {len(hits)} anchor lines, not {times}, for {insert[:50]!r}")
    for i in reversed(hits):
        lines[i + 1:i + 1] = insert.split("\n")
    board = "\n".join(lines)


SCROLLBAR = """/* E59: ONE rule for every box that scrolls -- ours, dockview's, reka-ui's, and those to come. Chromium's
   pseudo-elements, because the standard `scrollbar-width` and `scrollbar-color` say only colour and thickness, and the
   arrows stay. Where a box sets one of the two, the pseudo-elements turn off there (MDN): dockview's
   `.dv-tabs-container` has `scrollbar-width: thin`. Away from the ends on each axis -- `margin-block` alone leaves the
   horizontal bar at the sides -- and the corner between the two bars transparent, where Chromium draws it light. The
   thumb shows only with the pointer over the box or the focus inside it, and lights up under the pointer. */
::-webkit-scrollbar {
  width: var(--size-scrollbar);
  height: var(--size-scrollbar);
}

::-webkit-scrollbar-button {
  display: none;
}

::-webkit-scrollbar-track,
::-webkit-scrollbar-corner {
  background: transparent;
}

::-webkit-scrollbar-track:vertical {
  margin-block: var(--space-3);
}

::-webkit-scrollbar-track:horizontal {
  margin-inline: var(--space-3);
}

::-webkit-scrollbar-thumb {
  border: var(--space-0-5) solid transparent;
  border-radius: var(--radius-full);
  background-clip: padding-box;
  background-color: transparent;
}

:hover::-webkit-scrollbar-thumb,
:focus-within::-webkit-scrollbar-thumb {
  background-color: var(--color-border-strong);
}

::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-text-muted);
}

/* the trigger: Chromium restyles a custom scroll bar only when the scroller's OWN style changes, and a rule on the
   pseudo-element alone does not change it -- measured in Chrome 154 and Chromium 152. A registered property that does
   not inherit, so that nothing below the box restyles with it. */
@property --scrollbar-trigger {
  syntax: "<number>";
  inherits: false;
  initial-value: 0;
}

:hover,
:focus-within {
  --scrollbar-trigger: 1;
}

"""

ON_PAGE = ('  <div class="msg warn on-page"><svg class="i" data-icon="triangle-alert" aria-hidden="true" '
           'xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" '
           'stroke-linecap="round" stroke-linejoin="round" > <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 '
           '21h16a2 2 0 0 0 1.73-3" /> <path d="M12 9v4" /> <path d="M12 17h.01" /> </svg><div><b>Il core non ha '
           'risposto.</b></div><button class="btn sm" type="button">Riprova</button></div>')

INFO = ('      <div class="msg info"><svg class="i" data-icon="info" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" '
        'viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" '
        'stroke-linejoin="round" > <circle cx="12" cy="12" r="10" /> <path d="M12 16v-4" /> <path d="M12 8h.01" /> '
        '</svg><div><b>Richiesta inviata</b><span>In attesa del core.</span></div></div>')

ACTS_END = ('        <div class="acts"><span class="btn quiet">Annulla</span><span class="btn pri">Conferma</span></div></div>\n'
            '    </div>\n'
            '  </div>\n')

SCROLLING_CARD = """  <div class="card sc"><h2>Una scheda che scorre — passa sopra, o Tab dentro</h2>
    <div class="sc-body" tabindex="0" role="region" aria-label="Una scheda che scorre"><div class="rows">
      <div class="row">Sintesi della riunione <em>09:12</em></div>
      <div class="row">Report dei costi <em>10:40</em></div>
      <div class="row">Indice della KB <em>12:48</em></div>
      <div class="row">Esportazione mesh <em>14:05</em></div>
      <div class="row">Trascrizione della nota vocale <em>14:31</em></div>
      <div class="row">Confronto dei preventivi <em>15:02</em></div>
      <div class="row">Revisione del diff <em>15:47</em></div>
      <div class="row">Anteprima del viewer 3D <em>16:20</em></div>
      <div class="row">Riepilogo della settimana <em>17:05</em></div>
      <div class="row">Pulizia degli artefatti <em>17:40</em></div>
    </div><pre>2026-09-27 12:48:03  core  giornale  passo 42  intento scritto, esito atteso  ·  una riga di registro non va a capo, e la scatola scorre anche di lato</pre></div>
  </div>
"""

# ---- the block of base.css: E61, the border in the radii; E59, the scroll bar
once("  /* radii: OUTER = INNER + DISTANCE, by construction (answer 4) */\n",
     "  /* radii: OUTER = INNER + DISTANCE, by construction (answer 4). The distance runs from outer edge to outer edge, so\n"
     "     the border of the one that holds counts: every surface of ours that holds something rounded has its 1 px border,\n"
     "     drawn or transparent (E61) */\n")
once("  --radius-card: calc(var(--radius-control) + var(--space-3));   /* 20 */\n"
     "  --radius-frame: calc(var(--radius-card) + var(--space-3));     /* 32 */\n",
     "  --radius-card: calc(var(--radius-control) + var(--space-3) + var(--border-width));   /* 21 */\n"
     "  --radius-frame: calc(var(--radius-card) + var(--space-3) + var(--border-width));     /* 34 */\n")
once("  --focus-offset: 2px;\n\n",
     "  --focus-offset: 2px;\n\n"
     "  /* the scroll bar, thin: a 6 px thumb inside a 2 px transparent border (E59) */\n"
     "  --size-scrollbar: 10px;\n\n")
once("::selection {\n  background: var(--color-bg-selection);\n  color: var(--color-text);\n}\n\n",
     "::selection {\n  background: var(--color-bg-selection);\n  color: var(--color-text);\n}\n\n" + SCROLLBAR)

# ---- the board's own rules: the sheet and the demonstration with their border (E61); the message (E60); the card that
# scrolls (E59). With an action the message's row is centred: the owner's B of 2026-09-27, the 2 px of E60.
once(".sheet{background:var(--color-bg);color:var(--color-text);border-radius:calc(var(--radius-card) + var(--space-6));"
     "padding:var(--space-6)}\n",
     ".sheet{background:var(--color-bg);color:var(--color-text);border:var(--border-width) solid transparent;"
     "border-radius:calc(var(--radius-card) + var(--space-6) + var(--border-width));padding:var(--space-6)}\n")
once(".btn.quiet.is-disabled{color:var(--color-text-disabled);background:transparent}\n",
     ".btn.quiet.is-disabled{color:var(--color-text-disabled);background:transparent}\n"
     ".btn.sm{height:var(--size-control-sm);padding:0 var(--space-2)}\n")
once(".msg span{font:var(--font-caption);color:var(--color-text-muted)}\n",
     ".msg span{font:var(--font-caption);color:var(--color-text-muted)}\n"
     ".msg>div{flex:1;min-width:0}\n"
     ".msg:has(>.btn){align-items:center}\n"
     ".msg.on-page{border-radius:var(--radius-card)}\n"
     ".msg.info{background:var(--color-bg-accent-subtle);border-color:var(--color-border-accent)}"
     ".msg.info svg{color:var(--color-text-accent)}\n")
once(".nest-card{border-radius:var(--radius-card);padding:var(--space-3);background:var(--color-bg-surface)}\n",
     ".nest-card{border-radius:var(--radius-card);padding:var(--space-3);background:var(--color-bg-surface);"
     "border:var(--border-width) solid var(--color-border-card)}\n")
once(".rules li{margin:0 0 var(--space-2)}\n",
     ".rules li{margin:0 0 var(--space-2)}\n"
     ".card.sc{display:flex;flex-direction:column;height:240px;padding:0;overflow:hidden}\n"
     ".sc h2{padding:var(--space-3) var(--space-3) 0}\n"
     ".sc-body{flex:1;min-height:0;overflow:auto;padding:0 var(--space-3) var(--space-3)}\n"
     ".sc-body pre{font:var(--font-mono);color:var(--color-text-muted);margin:var(--space-3) 0 0}\n")

# ---- the words: the radii of the prose (trap 27 of the design), and what each theme shows
once("Schede e cornice con 12 di margine: i raggi diventano 20 e 32 (erano 18 e 28). La regola «fuori = dentro + "
     "distanza» resta, scritta nei token.</li>",
     "Schede e cornice con 12 di margine e il loro bordo: i raggi diventano 21 e 34 (erano 18 e 28). La regola «fuori = "
     "dentro + distanza» resta, scritta nei token, con la distanza da bordo esterno a bordo esterno.</li>")
once("cornice 32 = scheda 20 + 12 · scheda 20 = controllo 8 + 12 · in linea 6 · pillola tonda",
     "cornice 34 = scheda 21 + 12 + 1 · scheda 21 = controllo 8 + 12 + 1 · in linea 6 · pillola tonda")
# at the top of each theme, under its label: the message on the page with «Riprova», as the band
after_lines(lambda line: line.startswith('  <div class="lab">') and 'data-icon="layout-grid"' in line, ON_PAGE, 2)
# first in "Gli stati" of each theme: the neutral tone
after_lines(lambda line: line == '    <div class="msgs" style="margin-top:var(--space-4)">', INFO, 2)
# at the end of each theme, after the card of the elevation: a card that scrolls both ways
once(ACTS_END, ACTS_END + SCROLLING_CARD, 2)

out = board.replace("\n", "\r\n") if crlf else board
tmp = path + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print(f"ok: {path} ({'CRLF' if crlf else 'LF'})")
```

```bash
python <scratchpad>/cure_board.py "$(git rev-parse --show-toplevel)"
```

Poi `extract_tokens.py`, lo stesso del Passo 9 del compito 1, riportato qui per intero:

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
```

```bash
(cd gui && npx vitest run --project jsdom src/tokens/)
(cd gui && npx vitest run --project browser src/tokens/tokens.browser.test.ts src/kit/kit.browser.test.ts src/frame/dock.browser.test.ts)
grep -c 'scrollbar' gui/src/tokens/base.css
git diff --stat -- gui/src/tokens/themes.css
```

Atteso: sotto jsdom **verde**, `Tests  17 passed (17)` — `board.test.ts` e il contrasto: i ruoli non cambiano; nel browser
**rosse** tre cose — la barra, `expected { vertical: +0, horizontal: +0 } to deeply equal { vertical: 10, horizontal: 10 }`,
perché il progetto nasconde le barre (trappola 25); nella pagina kit la cornice, `base-button in kit-frame, top-left: radius 21.0,
outer 34.0, distance 12.0/12.0` coi suoi tre compagni; il dock com'era al Passo 3, a 21 — e **verdi** le righe di `BaseList` e la
finestra, curate dai token. `scrollbar` più volte in `base.css`; `themes.css` **non** cambia.

- [ ] **Passo 6: le barre mostrate al progetto browser (D26)**

In `gui/vite.config.ts` (`replace_unique.py`), una sostituzione.
*Trova*:

```ts
            // ⛔ THE INSTALLED CHROME (decision 22 of the design): nothing is downloaded, and a machine without
            // it goes red at this step with Playwright's own message -- the prerequisite, declared.
            provider: playwright({ launchOptions: { channel: "chrome" } }),
```

*Sostituisci con:*

```ts
            // ⛔ THE INSTALLED CHROME (decision 22 of the design): nothing is downloaded, and a machine without
            // it goes red at this step with Playwright's own message -- the prerequisite, declared.
            // ⛔ WITH ITS SCROLL BARS (control 22, E59): headless, Playwright launches Chrome with `--hide-scrollbars`, and a
            // bar is then 0 px thick, with our rule and without it. Shown, it is 15 without the rule and `--size-scrollbar`
            // with it -- and no other probe moved, measured on 2026-09-27 (D26 of the design-system plan).
            provider: playwright({ launchOptions: { channel: "chrome", ignoreDefaultArgs: ["--hide-scrollbars"] } }),
```

```bash
(cd gui && npx vitest run --project browser src/tokens/tokens.browser.test.ts)
```

Atteso: **verde**, `Tests  7 passed (7)` — lo spessore a 10, l'innesco a `0`, a `1` col puntatore sopra, di nuovo a `0` quando se
ne va, e a `1` col fuoco dentro.

- [ ] **Passo 7: ciò che sta subito dentro un bordo (E61)**

In `gui/src/tokens/dock.css` (`replace_unique.py`), una sostituzione.
*Trova*:

```css
.dockview-theme-harness .dv-resize-container > .dv-floating-titlebar {
  border-radius: var(--radius-card) var(--radius-card) 0 0;
}
/* Inside a floating container the group is not a second card: the container is. */
.dockview-theme-harness .dv-resize-container .dv-groupview {
  border: 0;
  border-radius: 0 0 var(--radius-card) var(--radius-card);
```

*Sostituisci con:*

```css
/* ⛔ ONE BORDER INSIDE THE CONTAINER'S EDGE, SO THE CARD'S RADIUS LESS THE BORDER (E61): the title bar and the group sit
   right inside the container's 1 px border, as the drop zone inside a card's (E55). */
.dockview-theme-harness .dv-resize-container > .dv-floating-titlebar {
  border-radius: calc(var(--radius-card) - var(--border-width)) calc(var(--radius-card) - var(--border-width)) 0 0;
}
/* Inside a floating container the group is not a second card: the container is. */
.dockview-theme-harness .dv-resize-container .dv-groupview {
  border: 0;
  border-radius: 0 0 calc(var(--radius-card) - var(--border-width)) calc(var(--radius-card) - var(--border-width));
```

In `gui/src/kit/Kit.vue` (`replace_unique.py`), una sostituzione.
*Trova*:

```vue
/* A frame of 32 with 12 of margin around cards of 20: answer 4, and the `calc` of the tokens. */
.kit-frame {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
  padding: var(--space-3);
  border-radius: var(--radius-frame);
```

*Sostituisci con:*

```vue
/* A frame of 34 with 12 of margin and its own border around cards of 21: answer 4, and the `calc` of the tokens. ⛔ THE
   BORDER IS TRANSPARENT AND IT COUNTS (E61): the distance runs from outer edge to outer edge, so every surface of ours
   that holds something rounded carries its 1 px border, drawn or not. */
.kit-frame {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
  padding: var(--space-3);
  border: var(--border-width) solid transparent;
  border-radius: var(--radius-frame);
```

In `gui/src/components/BaseDialog.vue` (`replace_unique.py`), una sostituzione — il commento dei raggi, che i token nuovi
rendono falso (gotcha **#58**, **E66**).
*Trova*:

```vue
/* The radii follow the rule of answer 4: a card of 20 with 12 of margin around controls of 8.
```

*Sostituisci con:*

```vue
/* The radii follow the rule of answer 4: a card of 21, 12 of margin and its 1 px border, around controls of 8 (E61).
```

```bash
(cd gui && npx vitest run --project browser src/kit/kit.browser.test.ts src/frame/dock.browser.test.ts src/testing/probes.browser.test.ts)
```

Atteso: **verde**, `Tests  42 passed (42)`.

- [ ] **Passo 8: `BaseNotice` — le prove (E60, controllo 23)**

In `gui/src/components/kit.test.ts` (`replace_unique.py`), quattro sostituzioni.
*Trova* — gli import:

```ts
import BaseList from "./BaseList.vue";
```

*Sostituisci con:*

```ts
import BaseList from "./BaseList.vue";
import BaseNotice from "./BaseNotice.vue";
```

*Trova* — prima di `BaseList and BaseLabel`:

```ts
describe("BaseList and BaseLabel", () => {
```

*Sostituisci con:*

```ts
describe("BaseNotice -- the message of the kit (E60 of the plan, control 23)", () => {
  const tones = ["info", "ok", "warn", "stop"] as const;

  it("draws the icon of its tone, by the tone's name, in each of the four (decision 28 of the design)", () => {
    for (const tone of tones) {
      const notice = mount(BaseNotice, { props: { tone, title: "Il core non ha risposto." } });
      expect(notice.attributes("data-tone")).toBe(tone);
      expect(notice.get("svg").attributes("data-icon")).toBe(tone);
    }
  });

  it("says its title always, its description only when it has one, and has no button of its own (decision 29)", () => {
    const bare = mount(BaseNotice, { props: { tone: "info", title: "Richiesta inviata: in attesa del core." } });
    expect(bare.get(".title").text()).toBe("Richiesta inviata: in attesa del core.");
    // ⛔ NO EMPTY BOX, AND NO CLOSING BY HAND: without a description there is no line for it, and nothing to click.
    expect(bare.find(".description").exists()).toBe(false);
    expect(bare.find("button").exists()).toBe(false);
    const told = mount(BaseNotice, {
      props: { tone: "stop", title: "Ultima richiesta di VRAM: rifiutata", description: "chiesti 4096 MiB, tetto 1024" },
    });
    expect(told.get(".description").text()).toBe("chiesti 4096 MiB, tetto 1024");
  });

  it("carries the action it is given, and only then centres its row (the owner, 2026-09-27: the 2 px of E60)", () => {
    const bare = mount(BaseNotice, { props: { tone: "warn", title: "Il core non ha risposto." } });
    expect(bare.attributes("data-action")).toBeUndefined();
    const acting = mount(BaseNotice, {
      props: { tone: "warn", title: "Il core non ha risposto.", onPage: true },
      slots: { action: () => h(BaseButton, { size: "sm" }, () => "Riprova") },
    });
    expect(acting.attributes("data-action")).toBeDefined();
    expect(acting.get("button").text()).toBe("Riprova");
  });

  it("takes the card's radius on the page, and only when asked (the (b) of the design)", () => {
    expect(mount(BaseNotice, { props: { tone: "ok", title: "Concessa" } }).attributes("data-on-page")).toBeUndefined();
    const onPage = mount(BaseNotice, { props: { tone: "warn", title: "Il core non ha risposto.", onPage: true } });
    expect(onPage.attributes("data-on-page")).toBeDefined();
  });
});

describe("BaseList and BaseLabel", () => {
```

*Trova* — la lista dei pezzi sotto `axe`:

```ts
    ["BaseStatus", () => h(BaseStatus, null, () => "Richiesta inviata.")],
```

*Sostituisci con:*

```ts
    ["BaseStatus", () => h(BaseStatus, null, () => "Richiesta inviata.")],
    ["BaseNotice", () => h(BaseNotice, { tone: "stop", title: "Ultima richiesta di VRAM: rifiutata", description: "chiesti 4096 MiB, tetto 1024" })],
    ["BaseNotice, on the page with its action", () =>
      h(BaseNotice, { tone: "warn", title: "Il core non ha risposto.", onPage: true }, { action: () => h(BaseButton, { size: "sm" }, () => "Riprova") })],
```

*Trova* — il titolo che contava i pezzi:

```ts
describe("the eight pieces, under axe", () => {
```

*Sostituisci con:*

```ts
// ⛔ NO COUNT IN THE TITLE (trap 23 of the design): it said "the eight pieces", and E60 made them nine.
describe("the pieces of the kit, under axe", () => {
```

```bash
(cd gui && npx vitest run --project jsdom src/components/kit.test.ts)
```

Atteso: **rosso**, `Error: Failed to resolve import "./BaseNotice.vue" from "src/components/kit.test.ts". Does the file exist?`.

- [ ] **Passo 9: le icone dei toni, e il pezzo**

In `gui/src/components/icons.ts` (`replace_unique.py`), quattro sostituzioni.
*Trova* — gli import, il primo:

```ts
  Box,
  Coins,
```

*Sostituisci con:*

```ts
  Box,
  CircleCheck,
  CircleX,
  Coins,
```

*Trova* — il secondo:

```ts
  GitCompare,
  Layers,
```

*Sostituisci con:*

```ts
  GitCompare,
  Info,
  Layers,
```

*Trova* — il terzo:

```ts
  SquareTerminal,
  type IconNode,
```

*Sostituisci con:*

```ts
  SquareTerminal,
  TriangleAlert,
  type IconNode,
```

*Trova* — la mappa, e il commento di `chat` (trappola 24 del disegno):

```ts
  fullPage: Maximize2,
  // one per module type of `panels/registry.ts`, BY THE SAME NAME: the big grab and the overview draw them. The
  // boards gave Stato, Permessi, Passi and Attività; the rest are the plan's choice, one line each (D6) -- Chat too:
  // on the boards `message-square` marks the status messages, and they would need another drawing if they entered
  // the kit (R2-15 of the review).
  chat: MessageSquare,
```

*Sostituisci con:*

```ts
  fullPage: Maximize2,
  // the tones of a message, BY THE TONE'S NAME (decision 28 of the design): the three drawings of the board's `.msg`,
  // and `info` for the neutral tone -- `BaseNotice` draws them (E60).
  info: Info,
  ok: CircleCheck,
  warn: TriangleAlert,
  stop: CircleX,
  // one per module type of `panels/registry.ts`, BY THE SAME NAME: the big grab and the overview draw them. The
  // boards gave Stato, Permessi, Passi and Attività; the rest are the plan's choice, one line each (D6) -- Chat too:
  // on the boards `message-square` marked the status messages, which entered the kit with the drawings of their
  // tones, above (R2-15 of the review, E60).
  chat: MessageSquare,
```

Crea `gui/src/components/BaseNotice.vue` (LF):

```vue
<script setup lang="ts">
import BaseIcon from "./BaseIcon.vue";

/**
 * The message of the kit (design system, section (b); E60 of its plan): the token board's `.msg` made a piece, for every
 * message of the system -- a tone among four with its icon, a title always, a description and an action when there are.
 * `title` and `description` are props, as in `BaseDialog`, and the action is a slot (decision 29 of the design).
 *
 * ⛔ NO WORDS INSIDE, AND NO CLOSING BY HAND (decision 29): it follows a state and goes when the state does -- a message
 * closed by hand hides a state that is still there, and one that closes itself is a notification, E228, the owner's.
 * ⛔ THE RADIUS IS THE CALLER'S TO SAY: in a card `--radius-control`, as the board's `.msg`; `onPage`, the card's -- the
 * band. The action lives on the page only: there it sits `--space-3` plus the border from the edge with
 * `--radius-control`, concentric by construction (E61); in a card it would sit 13 px from a corner of 8.
 * ⛔ WITH AN ACTION THE ROW IS CENTRED (the owner, 2026-09-27): the action is `--size-control-sm` high and the title's
 * line 20, and at the top the title and its icon sat 2 px above the action's words.
 * It enters a `BaseStatus` and takes no role of its own: the region is always there (M-3 of E187), and the space around
 * the message is the caller's class on it, never the region's (E42).
 */
withDefaults(defineProps<{ tone: "info" | "ok" | "warn" | "stop"; title: string; description?: string; onPage?: boolean }>(), {
  description: undefined,
  onPage: false,
});
</script>

<template>
  <div class="base-notice" :data-tone="tone" :data-on-page="onPage || undefined" :data-action="$slots.action ? '' : undefined">
    <BaseIcon :name="tone" size="lg" />
    <div class="words">
      <p class="title">{{ title }}</p>
      <p v-if="description !== undefined" class="description">{{ description }}</p>
    </div>
    <slot name="action" />
  </div>
</template>

<style scoped>
/* The board's `.msg` (the (a), "Gli stati"): the tone's subtle ground and border, the icon in the tone's text colour --
   the root's colour, which `BaseIcon` draws with --, the title in the text's own and the description muted under it. */
.base-notice {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  padding: var(--space-3);
  border: var(--border-width) solid;
  border-radius: var(--radius-control);
}
.base-notice[data-action] {
  align-items: center;
}
.base-notice[data-on-page] {
  border-radius: var(--radius-card);
}
.words {
  flex: 1;
  min-width: 0;
}
.title {
  margin: 0;
  font: var(--font-body-strong);
  color: var(--color-text);
}
.description {
  margin: 0;
  font: var(--font-caption);
  color: var(--color-text-muted);
}

/* The neutral tone is the accent's (N2 of E60): `info` in the code, because `neutral` is the warm grey's scale. */
.base-notice[data-tone="info"] {
  background: var(--color-bg-accent-subtle);
  border-color: var(--color-border-accent);
  color: var(--color-text-accent);
}
.base-notice[data-tone="ok"] {
  background: var(--color-bg-ok-subtle);
  border-color: var(--color-border-ok);
  color: var(--color-text-ok);
}
.base-notice[data-tone="warn"] {
  background: var(--color-bg-warn-subtle);
  border-color: var(--color-border-warn);
  color: var(--color-text-warn);
}
.base-notice[data-tone="stop"] {
  background: var(--color-bg-stop-subtle);
  border-color: var(--color-border-stop);
  color: var(--color-text-stop);
}
</style>
```

```bash
(cd gui && npx vitest run --project jsdom src/components/kit.test.ts)
```

Atteso: **verde**, `Tests  31 passed (31)` — le quattro prove del pezzo, i due pezzi nuovi sotto `axe`, e la mappa: ogni voce
disegna un'icona, le quattro nuove comprese.

- [ ] **Passo 10: la pagina kit — le prove**

In `gui/src/kit/kit.browser.test.ts` (`replace_unique.py`), tre sostituzioni.
*Trova* — le radici e le scatole:

```ts
const ROOTS = ".kit-card, .kit-frame, .kit-strip";
const BOXES = ".kit-card, .kit-frame, .kit-strip, .base-button, .base-list-row, .base-text-field > .frame, .option, .base-dialog";
```

*Sostituisci con:*

```ts
const ROOTS = ".kit-card, .kit-frame, .kit-strip, .base-notice";
const BOXES =
  ".kit-card, .kit-frame, .kit-strip, .base-button, .base-list-row, .base-text-field > .frame, .option, .base-dialog, .base-notice";
```

*Trova* — la guardia dei raggi:

```ts
      const list = document.querySelector(".kit-card:has(> .base-list)");
      expect(list).not.toBeNull();
      expect(concentricRadii([list as Element]).near).toBeGreaterThan(0);
    });
```

*Sostituisci con:*

```ts
      const list = document.querySelector(".kit-card:has(> .base-list)");
      expect(list).not.toBeNull();
      expect(concentricRadii([list as Element]).near).toBeGreaterThan(0);
      // ⛔ AND THE ONE CASE THAT JUDGES A MESSAGE'S ACTION (E60, E61): «Riprova», on the page, in the two right corners of
      // the message that holds it.
      const onPage = document.querySelector(".base-notice[data-on-page]");
      expect(onPage).not.toBeNull();
      expect(concentricRadii([onPage as Element]).near).toBeGreaterThan(0);
    });
```

*Trova* — la fine del file:

```ts
      expect(getComputedStyle(frame as HTMLElement).borderTopColor).toBe(stop);
      await userEvent.unhover(frame as HTMLElement);
    });
  });
}
```

*Sostituisci con:*

```ts
      expect(getComputedStyle(frame as HTMLElement).borderTopColor).toBe(stop);
      await userEvent.unhover(frame as HTMLElement);
    });

    it("dresses each tone of a message in its roles (control 23, E60)", async () => {
      await kit(theme);
      // The neutral tone is the accent's (N2 of E60); the three others are the states' own.
      const ROLE = { info: "accent", ok: "ok", warn: "warn", stop: "stop" } as const;
      const notices = [...document.querySelectorAll<HTMLElement>(".kit-card .base-notice")];
      // ⛔ NON-VACUITY: the card of the messages shows the four tones.
      expect(notices.map((notice) => notice.dataset.tone).sort()).toEqual(["info", "ok", "stop", "warn"]);
      for (const notice of notices) {
        const role = ROLE[notice.dataset.tone as keyof typeof ROLE];
        const icon = notice.querySelector("svg.base-icon");
        expect(icon).not.toBeNull();
        const style = getComputedStyle(notice);
        expect([style.backgroundColor, style.borderTopColor, getComputedStyle(icon as Element).color]).toEqual([
          colourOf(`--color-bg-${role}-subtle`),
          colourOf(`--color-border-${role}`),
          colourOf(`--color-text-${role}`),
        ]);
        // ⛔ AND THE WORDS IN THE TEXT'S OWN ROLES, WHATEVER THE TONE (the board's `.msg b` and `.msg span`; E65 of the
        // plan): without them the title takes the tone's colour from the root, which the icon draws with.
        expect(getComputedStyle(notice.querySelector(".title") as Element).color).toBe(colourOf("--color-text"));
        for (const description of notice.querySelectorAll(".description")) {
          expect(getComputedStyle(description).color).toBe(colourOf("--color-text-muted"));
        }
      }
      // ⛔ NON-VACUITY: a description was judged -- the refusal's, in the card of the messages.
      expect(document.querySelectorAll(".kit-card .base-notice .description").length).toBeGreaterThan(0);
    });

    it("sets a message's title and icon on the line of its action (the owner, 2026-09-27: the 2 px of E60)", async () => {
      await kit(theme);
      const notice = document.querySelector<HTMLElement>(".base-notice[data-action]");
      expect(notice).not.toBeNull();
      const centre = (selector: string): number => {
        const found = (notice as HTMLElement).querySelector(selector);
        expect(found).not.toBeNull();
        const box = (found as Element).getBoundingClientRect();
        return (box.top + box.bottom) / 2;
      };
      const action = centre("button");
      // ⛔ AT THE TOP, AS THE BOARD'S `.msg` WAS, THE TITLE SAT 2 PX ABOVE: the action is 24 high, the title's line 20.
      expect(Math.abs(centre(".title") - action)).toBeLessThanOrEqual(0.5);
      expect(Math.abs(centre("svg.base-icon") - action)).toBeLessThanOrEqual(0.5);
    });
  });
}
```

```bash
(cd gui && npx vitest run --project browser src/kit/kit.browser.test.ts)
```

Atteso: **rosse**, nei due temi, tre prove — *«keeps every radius concentric»*, `expected null not to be null`: non c'è un
messaggio sulla pagina; *«dresses each tone of a message in its roles»*, `expected [] to deeply equal [ 'info', 'ok', 'stop',
'warn' ]`; *«sets a message's title and icon on the line of its action»*, `expected null not to be null` — `Tests  6 failed | 16
passed (22)`.

- [ ] **Passo 11: la pagina kit — i messaggi**

In `gui/src/kit/Kit.vue` (`replace_unique.py`), tre sostituzioni.
*Trova* — gli import:

```vue
import BaseList from "../components/BaseList.vue";
```

*Sostituisci con:*

```vue
import BaseList from "../components/BaseList.vue";
import BaseNotice from "../components/BaseNotice.vue";
```

*Trova* — dopo la scheda di Stato:

```vue
        <p class="kit-note">La regione c'è sempre, anche vuota: le parole ci entrano.</p>
      </section>
```

*Sostituisci con:*

```vue
        <p class="kit-note">La regione c'è sempre, anche vuota: le parole ci entrano.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="info" as="h2">Messaggi</BaseLabel>
        <BaseNotice tone="info" title="Richiesta inviata: in attesa del core." />
        <BaseNotice tone="ok" title="Ultima richiesta di VRAM: concessa" />
        <BaseNotice tone="warn" title="Il core non ha risposto." />
        <BaseNotice tone="stop" title="Ultima richiesta di VRAM: rifiutata" description="chiesti 4096 MiB, tetto 1024" />
        <p class="kit-note">Quattro toni, ciascuno con la sua icona; il testo sotto il titolo quando c'è; nessuno si chiude a mano.</p>
      </section>
```

*Trova* — prima delle carte:

```vue
      <section class="kit-section kit-wide">
        <BaseLabel icon="views" as="h2">Carte</BaseLabel>
```

*Sostituisci con:*

```vue
      <section class="kit-section kit-wide">
        <BaseLabel icon="warn" as="h2">Messaggio sulla pagina</BaseLabel>
        <!-- ⛔ ON THE PAGE AND NOT IN A CARD (trap 22 of the design): an action in a card would sit 13 px from a corner of 8. -->
        <BaseNotice tone="warn" title="Il core non ha risposto." on-page>
          <template #action><BaseButton size="sm">Riprova</BaseButton></template>
        </BaseNotice>
      </section>

      <section class="kit-section kit-wide">
        <BaseLabel icon="views" as="h2">Carte</BaseLabel>
```

```bash
(cd gui && npx vitest run --project browser src/kit/kit.browser.test.ts)
```

Atteso: **verde**, `Tests  22 passed (22)` — i raggi con «Riprova» nei due angoli di destra del messaggio sulla pagina, i
ruoli dei quattro toni e delle loro parole (**E65**), la riga al centro, il testo che non si taglia e `axe` col contrasto.

- [ ] **Passo 12: i tre usi — le prove**

In `gui/src/frame/frame.test.ts` (`replace_unique.py`), tre sostituzioni.
*Trova* — gli import di `vitest`:

```ts
import { beforeEach, describe, expect, it } from "vitest";
```

*Sostituisci con:*

```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
```

*Trova* — la prima prova della fascia:

```ts
describe("the band", () => {
  it("is there while waiting, and gone once connected", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(i18n.global.t("band.waiting"));
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
```

*Sostituisci con:*

```ts
describe("the band", () => {
  it("is a warning with «Riprova» while waiting, «Riprova» retries, and it is gone once connected (control 23)", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    const retry = vi.spyOn(connection, "retry");
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    const notice = wrapper.get(".base-notice");
    expect(notice.attributes("data-tone")).toBe("warn");
    expect(notice.get(".title").text()).toBe(i18n.global.t("band.waiting"));
    await notice.get("button").trigger("click");
    expect(retry).toHaveBeenCalledOnce();
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
```

*Trova* — prima della prova della regione:

```ts
  it("keeps its status region while connected, and the words enter that same region (M-3 of E187)", async () => {
    const Band = (await import("./Band.vue")).default;
```

*Sostituisci con:*

```ts
  it("stops the window when the core speaks another protocol: no action, and the stamp the core expects (E60)", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    connection.receive({ kind: "StaleBuild", value: "81985529216486895" });
    await nextTick();
    const notice = wrapper.get(".base-notice");
    expect(notice.attributes("data-tone")).toBe("stop");
    // ON THE PAGE, LIKE THE WAITING BAND (E70 of the design-system plan): the browser measures the waiting one's place
    // only, so this one's class and mark are held here.
    expect(notice.classes()).toContain("band");
    expect(notice.attributes("data-on-page")).toBeDefined();
    expect(notice.get(".title").text()).toBe(i18n.global.t("band.stale"));
    expect(notice.get(".description").text()).toBe(i18n.global.t("band.expected", { stamp: "81985529216486895" }));
    // ⛔ NOTHING TO RETRY: the core stopped listening to this build.
    expect(notice.find("button").exists()).toBe(false);
  });

  it("keeps its status region while connected, and the words enter that same region (M-3 of E187)", async () => {
    const Band = (await import("./Band.vue")).default;
```

In `gui/src/panels/modules.test.ts` (`replace_unique.py`), tre sostituzioni.
*Trova* — Stato, prima di ogni verdetto, col titolo della prova (**E72**):

```ts
  it("says the core has not spoken, and shows no event row, before anything arrives", () => {
    wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(t("status.unknown"));
    expect(wrapper.find(".event").exists()).toBe(false);
```

*Sostituisci con:*

```ts
  it("says the core has not spoken, and shows no message, before anything arrives", () => {
    wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(t("status.unknown"));
    expect(wrapper.find(".base-notice").exists()).toBe(false);
```

*Trova* — Stato, il verdetto nei suoi toni:

```ts
  it("shows one event row only once a Verdict has arrived, with Refused told apart", async () => {
    const { bridge } = wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    bridge.deliver("Verdict");
    await nextTick();
    const event = wrapper.find(".event");
    expect(event.exists()).toBe(true);
    expect(event.text()).toContain(t("status.verdict.Refused"));
    expect(event.text()).toContain(t("status.refusedDetail", { asked: "4096", ceiling: "1024" }));
  });
```

*Sostituisci con:*

```ts
  it("shows the last Verdict as a message in its tone once one has arrived, Refused with its detail (control 23, E60)", async () => {
    const { bridge, core } = wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    bridge.deliver("Verdict");
    await nextTick();
    const notice = () => wrapper.get(".base-notice");
    expect(notice().attributes("data-tone")).toBe("stop");
    expect(notice().get(".title").text()).toBe(t("status.verdict.Refused"));
    expect(notice().get(".description").text()).toBe(t("status.refusedDetail", { asked: "4096", ceiling: "1024" }));
    core.receive({ kind: "Verdict", value: { verdict: "Granted" } });
    await nextTick();
    expect(notice().attributes("data-tone")).toBe("ok");
    expect(notice().get(".title").text()).toBe(t("status.verdict.Granted"));
    expect(notice().find(".description").exists()).toBe(false);
    core.receive({ kind: "Verdict", value: { verdict: "Queued" } });
    await nextTick();
    expect(notice().attributes("data-tone")).toBe("info");
    expect(notice().get(".title").text()).toBe(t("status.verdict.Queued"));
  });
```

*Trova* — Impostazioni, la richiesta in volo:

```ts
    expect(bridge.sent).toEqual([{ kind: "Invoke", value: { function: "vram-policy", argument: "local" } }]);
    expect(invoke.inFlight).not.toBeNull();
    expect(wrapper.text()).toContain(t("settings.inFlight"));
  });
```

*Sostituisci con:*

```ts
    expect(bridge.sent).toEqual([{ kind: "Invoke", value: { function: "vram-policy", argument: "local" } }]);
    expect(invoke.inFlight).not.toBeNull();
    // A message in the neutral tone (E60): the call is news, not a warning.
    const notice = wrapper.get(".base-notice");
    expect(notice.attributes("data-tone")).toBe("info");
    expect(notice.get(".title").text()).toBe(t("settings.inFlight"));
  });
```

Crea `gui/src/frame/frame.browser.test.ts` (LF):

```ts
import "dockview/dist/styles/dockview.css";
import "../tokens";

import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { createApp, type App as VueApp } from "vue";

import App from "../App.vue";
import { i18n } from "../i18n";
import { registerModules } from "../panels/modules";
import { concentricRadii } from "../testing/probes";
import { readToken } from "../tokens/readToken";

// ⛔ THE WHOLE FRAME IN THE INSTALLED CHROME (the (d) of the design system): the page of the SPA -- `App.vue`, whose rule
// gives `html`, `body` and `#app` the whole window -- at the probes' 1440 x 900, with the stylesheets in the order
// `main.ts` loads them. Born with task 6bis for the band on the page (E60); task 8 adds the strip and the overview.

const frames: VueApp[] = [];

beforeEach(() => {
  registerModules();
});

afterEach(() => {
  for (const frame of frames.splice(0)) frame.unmount();
  document.body.replaceChildren();
  delete document.documentElement.dataset.theme;
});

/**
 * The frame in one theme, once `dockview` has laid out the Home view. ⛔ MOUNTED AS `main.ts` MOUNTS IT, on `#app`
 * itself: `mount` of `@vue/test-utils` puts the app in a `div` of its own inside the element it is given, and that `div`
 * has no height -- the frame came out 116 px high, the dock 0, and the strip in the middle of the page, measured on
 * 2026-09-24 (P-23 of the plan).
 */
async function frame(theme: "light" | "dark"): Promise<void> {
  document.documentElement.dataset.theme = theme;
  const pinia = createPinia();
  setActivePinia(pinia);
  const host = document.createElement("div");
  host.id = "app";
  document.body.append(host);
  const app = createApp(App).use(pinia).use(i18n);
  app.mount(host);
  frames.push(app);
  await new Promise((resolve) => setTimeout(resolve, 50));
}

/** A length token, in px, as the page computes it. */
function px(token: string): number {
  return Number.parseFloat(readToken(token));
}

/** What a declaration of `property: var(token)` computes to here: the oracle of a token, with no copy of its value. */
function computed(property: string, token: string): string {
  const probe = document.createElement("div");
  probe.style.setProperty(property, `var(${token})`);
  document.body.append(probe);
  const value = getComputedStyle(probe).getPropertyValue(property);
  probe.remove();
  return value;
}

for (const theme of ["light", "dark"] as const) {
  describe(`the frame, ${theme} theme`, () => {
    it("lays the band on the page: under the bar, 12 px from the sides and from the cards, rounded like a card (control 23, E60)", async () => {
      await frame(theme);
      // With no core the connection waits (§6a): the band is there, a warning with «Riprova».
      const band = document.querySelector<HTMLElement>(".band");
      const bar = document.querySelector<HTMLElement>(".bar");
      expect(band).not.toBeNull();
      expect(bar).not.toBeNull();
      const box = (band as HTMLElement).getBoundingClientRect();
      // Aligned with the cards: the dock's own margin, `--space-3`, on each side.
      expect(box.left).toBeCloseTo(px("--space-3"), 1);
      expect(window.innerWidth - box.right).toBeCloseTo(px("--space-3"), 1);
      // Right under the bar, which leaves it its own 8 px: the band adds nothing above itself.
      expect(box.top).toBeCloseTo((bar as HTMLElement).getBoundingClientRect().bottom, 1);
      // `--space-3` above the highest card: the dock has none on top (answer 20).
      const cards = [...document.querySelectorAll(".dv-groupview")]
        .map((group) => group.getBoundingClientRect())
        .filter((card) => card.width > 0);
      expect(cards.length).toBeGreaterThan(0);
      expect(Math.min(...cards.map((card) => card.top)) - box.bottom).toBeCloseTo(px("--space-3"), 1);
      // Rounded like a card, and «Riprova» concentric in its corners at half a pixel (E61).
      expect(getComputedStyle(band as HTMLElement).borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
      const radii = concentricRadii([band as HTMLElement]);
      expect(radii.near).toBeGreaterThan(0);
      expect(radii.bad).toEqual([]);
    });
  });
}
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts src/panels/modules.test.ts)
(cd gui && npx vitest run --project browser src/frame/frame.browser.test.ts)
```

Atteso: sotto jsdom **rosse** quattro prove, `Unable to get .base-notice within: …` — le due della fascia, quella del verdetto e
quella della richiesta in volo —, `Tests  4 failed | 27 passed (31)`; nel browser **rosse** le due della fascia, `expected +0 to
be close to 12, received difference is 12, but expected 0.05`: oggi la fascia va da bordo a bordo.

- [ ] **Passo 13: i tre usi — il pezzo sulla pagina e nelle schede**

Riscrivi `gui/src/frame/Band.vue` **per intero**, col terminatore che ha oggi:

```vue
<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseNotice from "../components/BaseNotice.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useConnection } from "../stores/connection";

// ⛔ THE BAND APPEARS ONLY WHEN THE CORE IS MISSING OR THE STAMP IS WRONG (§6a), and it is NOT a panel (D50): a panel that
// came and went would rewrite the saved layout on every disconnection. ⛔ AND THERE IS NO THRESHOLD (D48): "not running"
// and "slow" are one state here, because the gui does the same thing in both -- offer `retry`.
// ⛔ THE REGION IS ALWAYS THERE AND THE BAND ENTERS IT (M-3 of E187, closed by `BaseStatus`): a status region born with
// its text is the one many screen readers never announce.
// ⛔ A MESSAGE ON THE PAGE (E60 of the design-system plan): `BaseNotice` with the card's radius, aligned with the cards --
// `--space-3` from the sides and above them, the dock having none on top (answer 20) -- and under the bar, which leaves it
// its own 8 px. The space is the message's, never the region's, which stays zero high when empty (E42). Waiting is a
// warning with «Riprova»; another stamp STOPS the window -- nothing to retry, and the bar's chip is `--color-text-stop`.
const connection = useConnection();
</script>

<template>
  <BaseStatus>
    <BaseNotice
      v-if="connection.phase === 'stale'"
      class="band"
      tone="stop"
      :title="$t('band.stale')"
      :description="connection.expected !== null ? $t('band.expected', { stamp: connection.expected }) : undefined"
      on-page
    />
    <BaseNotice v-else-if="connection.phase !== 'connected'" class="band" tone="warn" :title="$t('band.waiting')" on-page>
      <template #action>
        <BaseButton size="sm" @click="connection.retry()">{{ $t("band.retry") }}</BaseButton>
      </template>
    </BaseNotice>
  </BaseStatus>
</template>

<style scoped>
.band {
  margin: 0 var(--space-3) var(--space-3);
}
</style>
```

In `gui/src/panels/Settings.vue` (`replace_unique.py`), tre sostituzioni.
*Trova* — gli import:

```vue
import BaseRadioGroup from "../components/BaseRadioGroup.vue";
```

*Sostituisci con:*

```vue
import BaseNotice from "../components/BaseNotice.vue";
import BaseRadioGroup from "../components/BaseRadioGroup.vue";
```

*Trova* — la riga della richiesta in volo:

```vue
        <p v-if="invoke.inFlight !== null">{{ $t("settings.inFlight") }}</p>
```

*Sostituisci con:*

```vue
        <BaseNotice v-if="invoke.inFlight !== null" class="in-flight" tone="info" :title="$t('settings.inFlight')" />
```

*Trova* — lo spazio sopra il messaggio:

```vue
.policy p {
  margin-top: var(--space-2);
}
```

*Sostituisci con:*

```vue
/* The space is the message's, never the region's (E42). */
.in-flight {
  margin-top: var(--space-2);
}
```

In `gui/src/panels/Status.vue` (`replace_unique.py`), tre sostituzioni.
*Trova* — lo script:

```vue
import BaseStatus from "../components/BaseStatus.vue";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";

// The Stato table of §1 of the north star, rows 1-4 -- what sub-project 2 builds. Stato SHOWS and
// does not command: the policy change is a registry function and lives in Impostazioni.
const connection = useConnection();
const core = useCore();
```

*Sostituisci con:*

```vue
import { computed } from "vue";
import { useI18n } from "vue-i18n";

import BaseNotice from "../components/BaseNotice.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";

// The Stato table of §1 of the north star, rows 1-4 -- what sub-project 2 builds. Stato SHOWS and
// does not command: the policy change is a registry function and lives in Impostazioni.
const { t } = useI18n();
const connection = useConnection();
const core = useCore();

/** The tone of each verdict (E60 of the design-system plan): queued is news, granted is good, refused stops the call. */
const TONE = { Queued: "info", Granted: "ok", Refused: "stop" } as const;

/** The last verdict as a message: its tone, its words, and the detail of a refusal. */
const verdict = computed(() => {
  const last = core.lastVerdict;
  if (last === null) return null;
  return {
    tone: TONE[last.verdict],
    title: t(`status.verdict.${last.verdict}`),
    description: last.verdict === "Refused" ? t("status.refusedDetail", { asked: last.asked, ceiling: last.ceiling }) : undefined,
  };
});
```

*Trova* — la riga del verdetto:

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

*Sostituisci con:*

```vue
    <!-- ONE MESSAGE FOR THE LAST Verdict, AND ONLY WHEN ONE HAS ARRIVED (§6a): no empty box -- and the message ENTERS a
         region that is always there (M-3 of E187, `BaseStatus`), in the verdict's tone (E60). -->
    <BaseStatus>
      <BaseNotice v-if="verdict !== null" class="event" :tone="verdict.tone" :title="verdict.title" :description="verdict.description" />
    </BaseStatus>
```

*Trova* — lo spazio sopra il messaggio:

```vue
.event {
  margin-top: var(--space-3);
  border-top: var(--border-width) solid var(--color-border);
  padding-top: var(--space-2);
}
```

*Sostituisci con:*

```vue
/* The space is the message's, never the region's (E42); the line above the old row went with it -- the message has a
   border of its own. */
.event {
  margin-top: var(--space-3);
}
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts src/panels/modules.test.ts)
(cd gui && npx vitest run --project browser src/frame/frame.browser.test.ts)
```

Atteso: **verde**, `Tests  31 passed (31)` e `Tests  2 passed (2)`. ⛔ Le prove della regione — *«keeps its status region…»*, in
tutti e tre gli usi — restano **com'erano** e verdi: la regione è la stessa prima e dopo (M-3).

- [ ] **Passo 14: tutte le prove, il *build*, il linter**

```bash
(cd gui && npm test && npm run build && npm run lint)
```

Atteso: **verde** su tutto; `Test Files  25 passed | 1 skipped (26)` e `Tests  200 passed | 1 skipped (201)`, il 2026-09-27 sulla
cartella di prova — i file uno in più, `frame.browser.test.ts`. La riga del pezzo JavaScript, `npm run build 2>&1 | grep -E
'assets/index-.*\.js '`, va nel commit accanto a quella del Passo 1: `691.82 kB`, compresso `210.77 kB` (**N-2**, la cifra per il
proprietario).

- [ ] **Passo 15: le due direzioni**

Una violazione alla volta, poi indietro con la **copia salvata** e `cmp` (vincolo 11): `BaseNotice.vue` e `frame.browser.test.ts`
sono nati qui, e ogni altro file il compito l'ha cambiato — `git checkout` non conosce i primi e toglierebbe ai secondi il lavoro
del compito (A-1). Ciascuna si prova col file che la deve cogliere, da solo; misurate il 2026-09-27 sulla cartella di prova:

| La prova | La violazione messa a mano | Atteso |
|---|---|---|
| `probes.browser.test.ts`, il pixel del bordo | in `probes.ts` `const SLACK = 1.5;` | rosso: `expected { near: 2, bad: [] } to deeply equal { near: 2, bad: [ …(2) ] }` |
| `tokens.browser.test.ts`, lo spessore | in `base.css` tolta la regola `::-webkit-scrollbar { … }` — e `board.test.ts` rosso anche lui, che è giusto | rosso: `expected { vertical: 15, horizontal: 15 } to deeply equal { vertical: 10, horizontal: 10 }` — lo spessore di Chrome |
| `tokens.browser.test.ts`, l'innesco | in `base.css` tolta la regola `:hover, :focus-within { --scrollbar-trigger: 1; }` | rosso: `expected '0' to be '1'` |
| `tokens.browser.test.ts`, le barre mostrate | in `vite.config.ts` tolto `, ignoreDefaultArgs: ["--hide-scrollbars"]` | rosso: `expected { vertical: +0, horizontal: +0 } to deeply equal { vertical: 10, horizontal: 10 }` |
| `dock.browser.test.ts`, la barra del titolo staccata | in `dock.css` il suo raggio di nuovo `var(--radius-card) var(--radius-card) 0 0` | rosso, nei due temi: `dv-floating-titlebar in dv-resize-container, top-left: radius 21.0, outer 21.0, distance 1.0/1.0` e il suo `top-right` |
| `kit.browser.test.ts`, la cornice | in `Kit.vue` tolto il bordo trasparente di `.kit-frame` | rosso, nei due temi: `base-button in kit-frame, top-left: radius 21.0, outer 34.0, distance 12.0/12.0` coi suoi tre compagni |
| `kit.browser.test.ts`, la riga al centro | in `BaseNotice.vue` tolta la regola `.base-notice[data-action]` | rosso, nei due temi: `expected 2 to be less than or equal to 0.5` — i 2 px |
| `kit.browser.test.ts`, i ruoli del tono | in `BaseNotice.vue` il selettore `[data-tone="info"]` → `[data-tone="info-not"]` | rosso, nei due temi: nel chiaro `expected [ 'rgba(0, 0, 0, 0)', …(2) ] to deeply equal [ 'rgb(239, 225, 221)', …(2) ]` |
| `kit.browser.test.ts`, le parole nei loro ruoli | in `BaseNotice.vue` tolto `color: var(--color-text);` da `.title` | rosso, nei due temi: nel chiaro `expected 'rgb(122, 31, 46)' to be 'rgb(27, 23, 24)'` — il titolo del tono neutro nel colore dell'accento (**E65**) |
| `kit.test.ts`, l'icona col nome del tono | in `BaseNotice.vue` `name="info"` al posto di `:name="tone"` | rosso: `expected 'info' to be 'ok'` |
| `frame.browser.test.ts`, il raggio sulla pagina | in `BaseNotice.vue` tolta la regola `.base-notice[data-on-page]` | rosso, nei due temi: `expected '8px' to be '21px'` |
| `frame.browser.test.ts`, lo spazio della fascia | in `Band.vue` la regola `.band` vuota | rosso, nei due temi: `expected +0 to be close to 12, received difference is 12, but expected 0.05` |
| `frame.test.ts`, «Riprova» che riprova | in `Band.vue` tolto ` @click="connection.retry()"` | rosso: `expected "wrappedAction" to be called once, but got 0 times` |
| `frame.test.ts`, il timbro diverso | in `Band.vue` il `tone="stop"` → `tone="warn"` | rosso: `expected 'warn' to be 'stop'` |
| `frame.test.ts`, il timbro diverso sulla pagina | in `Band.vue` tolto `on-page` dal ramo `stale` | rosso: `expected undefined to be defined` (**E70**) |
| `frame.test.ts`, il timbro diverso come la fascia | in `Band.vue` tolto `class="band"` dal ramo `stale` | rosso: `expected [ 'base-notice' ] to include 'band'` (**E70**) |
| `modules.test.ts`, il rifiuto | in `Status.vue` `Refused: "warn"` | rosso: `expected 'warn' to be 'stop'` |
| `modules.test.ts`, la richiesta in volo | in `Settings.vue` `tone="warn"` | rosso: `expected 'warn' to be 'info'` |

Alla fine, dalla radice del repository, `git status --porcelain | diff <scratchpad>/prima.txt -` rende soltanto i file del
compito: nessun file nato dai rossi (R2-3).

- [ ] **Passo 16: guardarlo — il secondo sguardo del proprietario, nel suo Chrome col mouse vero**

⛔ **Del proprietario col coordinatore, dopo la revisione e le sue cure**, sul codice che resta — non di un subagente
(**E69**). `(cd gui && npm run dev)`, la SPA nel browser del proprietario a 1440 × 900: ⚠️ **prima** di
`harnessFake.deliverAll()` la fascia è quella del core che non ha risposto, `warn` con «Riprova»; **dopo**, è il timbro
diverso, `stop`, perché le fixture consegnano anche `StaleBuild` — misurato il 2026-09-27 (**E69**). Poi la pagina kit,
`/kit.html`, e la tavola, servita con `python -m http.server` da un'altra porta — mai `file://`. Nei due temi:

1. **la barra (E59)** — Stato e Impostazioni nella Home, e la scheda che scorre in fondo alla tavola: sottile, senza frecce,
   staccata dalle estremità, l'angolo trasparente; il cursore che **compare col puntatore sopra** — la conferma col mouse vero
   che la sessione del 2026-09-27 non ha registrato — e col fuoco dentro, e che si accende sotto il puntatore;
2. **i raggi (E61)** — le schede, il gruppo staccato con «Stacca la tessera», la zona d'arrivo trascinando una linguetta, la
   finestra della pagina kit;
3. **i messaggi (E60)** — la fascia **a core spento**, prima della consegna, sulla pagina, con «Riprova» concentrico e la riga al
   centro; dopo `harnessFake.deliverAll()`, il timbro diverso in `stop`, *«Richiesta inviata: in attesa del core.»* scegliendo
   l'altra policy in Impostazioni, e il verdetto in Stato.

Poi il frammento del contrasto del Passo 8 del compito 6, sulla SPA e nei due temi: il primo di `worst` **sopra 4,5**. ⛔ L'aspetto
lo **giudica il proprietario** (controllo 15): ciò che non gli piace è una voce d'errata col suo *«perché»*, non un ritocco di chi
esegue. Se lo squircle torna in mente, è la condizione scritta in *«Vicoli ciechi e scelte scartate»* del disegno.

- [ ] **Passo 17: il cancello, il commit, la posizione**

⛔ **Due commit, e il secondo è del coordinatore** (**E69**, la forma di **E35**). **Il codice**, dopo il Passo 15: `bash
scripts/gate.sh` da solo, `bash scripts/check-docs.sh`, il commit dei **soli file del compito** — `design-system(compito 6bis):
la cura delle tre voci del Passo 8 …`, con le due righe del pezzo JavaScript — coi fine-riga rimisurati; le righe **6** e **6bis**
della posizione restano come sono, perché il Passo 16 viene dopo la revisione. **La chiusura**, approvato lo sguardo del Passo
16: la riga **6bis** della tabella della posizione a `✅ <data>`, e la riga **6** a `✅ <data>` col verbale del Passo 8 — le
quattro foto del 2026-09-26, le tre voci, lo sguardo di questo compito —; la riga **6** della colonna **Commit** con gli hash
del compito 6 e delle sue cure; in *«Verificato, dedotto, assunto»* del disegno, l'assunzione della barra sotto la preferenza
di Windows e i colori forzati, con ciò che lo sguardo ha visto — o resta assunta, e lo dice —; il cancello da solo,
`check-docs.sh`, il commit e `git push`. Se lo sguardo non approva, ciò che non piace è una voce d'errata col suo *«perché»*,
e le due righe restano aperte.

---
## Compito 7: le viste col nome, sotto — il pacchetto, il negozio, il dock, la geometria comune, lo schema

**Da:** la (d) del disegno, *«Le viste salvate col nome»* e le righe *«le miniature»* e *«le frecce nella griglia»* della
Panoramica; il controllo **18**, e dal **17** l'aiutante della geometria provato coi rettangoli dati a mano; le decisioni
**19** e **20** del disegno; **D3**, **D4**, **D5**, **D12** e **D13** di questo piano; R3-18 e R3-19 della revisione.

**Files:**
- Create: `gui/src/frame/nearest.ts`, `gui/src/frame/nearest.test.ts`, `gui/src/frame/schematic.ts`,
  `gui/src/frame/schematic.test.ts`
- Rewrite: `gui/src/stores/layout.ts` — **per intero**, col terminatore che ha oggi
- Modify: `gui/src/stores/stores.test.ts`, `gui/src/frame/moveActive.ts`, `gui/src/frame/dock.ts`,
  `gui/src/frame/Frame.vue`, `gui/src/frame/frame.test.ts`

**Interfaces:**
- Consumes: `LayoutPack`, `useLayout`, `pack_` e `unpack` col campo `theme` (compito 1); `createDock` e `apply` col tema
  nostro (compito 6).
- Produces, per il compito 8:
  - in `LayoutPack` i campi `named?: NamedView[]` e `openNamed?: string`, con `interface NamedView { name: string; layout:
    SerializedDockview }` da `stores/layout.ts`;
  - in `useLayout()` lo stato `openNamed: string | null`, `showView(view: ViewName): void` — una delle tre, e la vista col
    nome si chiude —, e `saveNamed(name: string, layout: SerializedDockview, shown: readonly string[]): "saved" | "empty" |
    "taken"`, dove `shown` sono i nomi che la cornice mostra per le tre viste (D12); per aprire una vista col nome si
    scrive `openNamed`, come `view` si scriveva;
  - `apply(api, view, pack, named?)` da `frame/dock.ts`;
  - `nearest<T extends { rect: Box }>(from: Box, candidates: readonly T[], direction: Direction): T | undefined`, con `type
    Box` e `type Direction`, da `frame/nearest.ts`;
  - `schematic(layout: SerializedDockview): Tile[]`, con `interface Tile { x; y; width; height; views: string[]; active?:
    string }` in frazioni del quadrato unitario, da `frame/schematic.ts`; un gruppo ingrandito solo, sul quadrato
    intero (**E85**).

⚠️ **Che cosa la scrittura di questo compito ha misurato**, il 2026-09-23 sulla cartella di prova coi compiti 1–6 applicati:

| | Il fatto | Che cosa ne fa il compito |
|---|---|---|
| 1 | l'albero di una disposizione salvata: la radice stende i figli lungo `grid.orientation`, ogni ramo sotto sull'altro asse, e la `size` di un nodo è la sua estensione lungo l'asse del genitore — nella Home spedita la radice `HORIZONTAL` ha un figlio solo, e sotto di lui l'area dei pannelli e la striscia si dividono l'altezza | `schematic` segue la regola; la prova la tiene su una disposizione scritta a mano e sulle tre viste spedite |
| 2 | nessun altro file importa `Direction` | il tipo passa in `nearest.ts` |
| 3 | nessun negozio legge le parole di `it.json`; e nessuna prova **jsdom** monta `Frame.vue` — ⚠️ dal 6bis la monta `frame.browser.test.ts`, attraverso `App.vue`, e nessuna prova ne preme la barra (**E76**) | `saveNamed` riceve i nomi mostrati da chi la chiama (D12); la regola *«una delle tre chiude la vista col nome»* vive nel negozio, in `showView`, dove una prova la raggiunge (D13). ⚠️ **La riga di `Frame.vue` che la chiama non la tiene nessuna prova**, misurato, ed è dichiarato (**E76**): il compito 8 riscrive `Frame.vue` e `ViewBar.vue` |
| 4 | la prova della tastiera del compito 5 cadeva una volta su dieci corse della suite intera (**P-19**) | è corretta nel compito 5; qui la si ritrova verde a ogni corsa |

- [ ] **Passo 1: rimisura il punto di partenza**

```bash
git status --porcelain > <scratchpad>/prima.txt
grep -rnw 'Direction' gui/src --include=*.ts --include=*.vue
grep -rln 'i18n' gui/src/stores
grep -rln 'Frame.vue' gui/src --include=*.test.ts
(cd gui && npm run build 2>&1 | grep -E 'assets/index-.*\.js ')
```

Atteso: `Direction` nel solo `frame/moveActive.ts` — ⚠️ `-w`, la parola intera: senza, il comando prende anche `flexDirection`
in `testing/probes.ts`, misurato —; i due `grep -rln` **non rendono nulla**; e la riga del pezzo JavaScript è la baseline del
compito (**E80**) — `691.82 kB`, compresso `210.77 kB`, il 2026-09-27 a `1c168a1`. Poi, da solo, `bash scripts/gate.sh` →
`GATE GREEN`.

- [ ] **Passo 2: le prove, prima del codice**

Crea `gui/src/frame/nearest.test.ts` (LF):

```ts
import { describe, expect, it } from "vitest";

import { nearest, type Box, type Direction } from "./nearest";

/** A card of a grid, as `nearest` sees it: a name for the probe, and a rectangle written by hand -- under jsdom every
 * rectangle is zero (P-97 of part 2). */
function card(name: string, left: number, top: number, width = 200, height = 120): { name: string; rect: Box } {
  return { name, rect: { left, top, right: left + width, bottom: top + height } };
}

/** Three columns and two rows, 24 px apart: the overview's grid, and the case R3-18 of the review measured. */
const GRID = [0, 1].flatMap((row) => [0, 1, 2].map((column) => card(`r${row}c${column}`, column * 224, row * 144)));

function from(name: string): Box {
  const found = GRID.find((candidate) => candidate.name === name);
  if (found === undefined) throw new Error(`no card ${name}`);
  return found.rect;
}

describe("nearest (decision 19 of the design system)", () => {
  it("goes to the card beyond, in each of the four directions", () => {
    const moves: [string, Direction, string][] = [
      ["r0c1", "right", "r0c2"],
      ["r0c1", "left", "r0c0"],
      ["r1c2", "up", "r0c2"],
      ["r0c0", "down", "r1c0"],
    ];
    for (const [start, direction, end] of moves) expect(nearest(from(start), GRID, direction)?.name, `${start} ${direction}`).toBe(end);
  });

  it("breaks a tie on the other axis: down from the middle column is the middle card below (R3-18)", () => {
    // ⛔ EVERY CARD OF THE ROW BELOW IS EQUALLY FAR on the vertical axis: the gap alone sends "down" to the first column.
    expect(nearest(from("r0c1"), GRID, "down")?.name).toBe("r1c1");
    expect(nearest(from("r0c2"), GRID, "down")?.name).toBe("r1c2");
    expect(nearest(from("r1c2"), GRID, "up")?.name).toBe("r0c2");
  });

  it("finds nothing beyond an edge, and never the card it starts from", () => {
    expect(nearest(from("r0c0"), GRID, "up")).toBeUndefined();
    expect(nearest(from("r0c2"), GRID, "right")).toBeUndefined();
  });
});
```

Crea `gui/src/frame/schematic.test.ts` (LF):

```ts
import { createDockview, type SerializedDockview } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";

import { componentFor } from "../panels/registry";
import { VIEWS } from "../panels/views";

import { schematic } from "./schematic";

// The probe of E85 mounts the real panels, and they read the stores.
beforeEach(() => {
  setActivePinia(createPinia());
});

/** A leaf of the serialized grid: one group with its panels. */
function leaf(size: number, ...views: string[]) {
  return { type: "leaf" as const, size, data: { views, activeView: views[0], id: views.join("+") } };
}

/**
 * ⛔ A LAYOUT WRITTEN BY HAND, with sizes chosen so the fractions are exact: the root is VERTICAL -- `a` above, a branch
 * below -- and the level under it is HORIZONTAL, `b` a quarter and `c` three quarters. And a floating group, which a
 * miniature does not draw (D5 of the plan).
 */
const HAND = {
  grid: {
    orientation: "VERTICAL",
    width: 800,
    height: 600,
    root: { type: "branch", size: 800, data: [leaf(30, "a"), { type: "branch", size: 70, data: [leaf(1, "b"), leaf(3, "c", "d")] }] },
  },
  panels: {},
  floatingGroups: [{ data: { views: ["floating"], id: "f" }, position: { left: 0, top: 0, width: 100, height: 100 } }],
} as unknown as SerializedDockview;

describe("schematic (answer 19 of the design system)", () => {
  it("cuts the unit square along the tree, the orientation alternating at every level, and leaves the floating out", () => {
    expect(schematic(HAND)).toEqual([
      { x: 0, y: 0, width: 1, height: 0.3, views: ["a"], active: "a" },
      { x: 0, y: 0.3, width: 0.25, height: 0.7, views: ["b"], active: "b" },
      { x: 0.25, y: 0.3, width: 0.75, height: 0.7, views: ["c", "d"], active: "c" },
    ]);
  });

  it("tiles each view that ships without a gap or an overlap, the strip across the bottom", () => {
    for (const [name, view] of Object.entries(VIEWS)) {
      const tiles = schematic(view);
      // ⛔ NON-VACUITY: every view has panels, so every view has tiles.
      expect(tiles.length, name).toBeGreaterThan(1);
      const area = tiles.reduce((sum, tile) => sum + tile.width * tile.height, 0);
      expect(Math.abs(area - 1), `${name}: the areas add up to the square`).toBeLessThan(1e-9);
      for (const tile of tiles) {
        expect(tile.x >= 0 && tile.y >= 0 && tile.x + tile.width <= 1 + 1e-9 && tile.y + tile.height <= 1 + 1e-9, `${name}: ${tile.views}`).toBe(true);
      }
      // ⛔ THE AREAS ADDING UP DO NOT RULE OUT AN OVERLAP (E84 of the plan): an overlap and a gap of the same size add up
      // too. No two tiles overlap, the sum is the square and every tile is inside it -- so the tiles cover it.
      tiles.forEach((a, index) => {
        for (const b of tiles.slice(index + 1)) {
          const across = Math.min(a.x + a.width, b.x + b.width) - Math.max(a.x, b.x);
          const down = Math.min(a.y + a.height, b.y + b.height) - Math.max(a.y, b.y);
          expect(Math.max(0, across) * Math.max(0, down), `${name}: ${a.views} over ${b.views}`).toBeLessThan(1e-9);
        }
      });
      const strip = tiles.find((tile) => tile.views.includes("strip"));
      expect(strip && strip.x === 0 && Math.abs(strip.width - 1) < 1e-9 && Math.abs(strip.y + strip.height - 1) < 1e-9, `${name}: the strip`).toBe(true);
    }
  });

  it("draws a maximized group alone, on the whole square: the layout opens with it (E85 of the plan)", () => {
    const host = document.createElement("div");
    document.body.append(host);
    const api = createDockview(host, { createComponent: ({ name }) => componentFor(name)() });
    api.layout(1600, 1000);
    api.fromJSON(VIEWS.home);
    // The group the big grab maximizes: a real `toJSON()`, not a node written by hand -- the shape is dockview's.
    const group = api.getPanel("status")?.group;
    group?.api.maximize();
    const saved = api.toJSON();
    // ⛔ NON-VACUITY: dockview wrote the group maximized.
    expect((saved.grid as { maximizedNode?: unknown }).maximizedNode).toBeDefined();
    expect(schematic(saved)).toEqual([
      { x: 0, y: 0, width: 1, height: 1, views: group?.panels.map((panel) => panel.id), active: group?.activePanel?.id },
    ]);
  });
});
```

In `gui/src/stores/stores.test.ts` (`replace_unique.py`), due sostituzioni. *Trova*:

```ts
import { pack_, unpack, useLayout } from "./layout";
```

*Sostituisci con:*

```ts
import { pack_, unpack, useLayout, type LayoutPack } from "./layout";
```

*Trova* — la fine del file (allineata con **E2** il 2026-09-24):

```ts
    // ⛔ NOT "something was sent": a settle that rebuilt the package from `view` and `layouts` alone would drop
    // the choice at the first move of a panel.
    expect(settled).toEqual({ view: "home", layouts: { work, home }, theme: "dark" });
  });
});
```

*Sostituisci con:*

```ts
    // ⛔ NOT "something was sent": a settle that rebuilt the package from `view` and `layouts` alone would drop
    // the choice at the first move of a panel.
    expect(settled).toEqual({ view: "home", layouts: { work, home }, theme: "dark" });
  });
});

describe("the named views in the package (design system, section (d))", () => {
  /** A package from the core, as `receive` gets it. */
  function fromTheCore(pack: LayoutPack): IpcMessage {
    return { kind: "Layout", value: { state: "Package", bytes: [...pack_(pack)] } };
  }

  /** What the store sent, read back: the n-th `SaveLayout`. */
  function sentPack(bridge: ReturnType<typeof createFakeBridge>, index: number): LayoutPack | null {
    const message = bridge.sent[index];
    return message?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: message.value }) : null;
  }

  it("keeps the named views and the open one a package carries, and opens one without them as before (control 18)", () => {
    const layout = useLayout();
    const review = { marker: "the owner's review" } as never;
    layout.receive(fromTheCore({ view: "work", layouts: {}, named: [{ name: "Revisione", layout: review }], openNamed: "Revisione" }));
    expect(layout.saved?.named).toEqual([{ name: "Revisione", layout: review }]);
    expect(layout.openNamed).toBe("Revisione");
    expect(layout.view).toBe("work");
    // ⛔ THE SECOND DIRECTION: a package written before the list existed opens as before, no named view open.
    setActivePinia(createPinia());
    const older = useLayout();
    older.receive(fromTheCore({ view: "compact", layouts: {} }));
    expect(older.saved).toEqual({ view: "compact", layouts: {} });
    expect(older.openNamed).toBeNull();
  });

  it("drops what it cannot read -- and a key it does not know inside `layouts` stays dropped (row 8 of §2)", () => {
    const text = JSON.stringify({
      view: "home",
      layouts: { home: {}, mine: {} },
      named: [{ name: "Revisione", layout: {} }, { name: "", layout: {} }, { name: "Senza" }, { layout: {} }, null, { name: "Nulla", layout: null }, { name: " revisione ", layout: { second: true } }],
      openNamed: "Sparita",
    });
    // ⛔ A NAME ALREADY READ IS A SECOND VIEW UNDER IT (D4): the first stays. An open name the list does not hold is
    // read as absent, and the view of always opens. And an entry that is `null`, or holds a `null` layout, is dropped
    // (E88 of the plan): without its guard the first throws inside `unpack`'s `try`, and the whole package is lost.
    expect(unpack({ state: "Package", bytes: [...new TextEncoder().encode(text)] })).toEqual({
      view: "home",
      layouts: { home: {} },
      named: [{ name: "Revisione", layout: {} }],
    });
  });

  it("writes a move in an open named view into THAT view, and leaves the three and the other named views as they were (R3-19)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const home = { marker: "home, as the owner left it" } as never;
    // ⛔ TWO NAMED VIEWS, THE OPEN ONE SECOND (E82 of the plan): with one alone, "that view" reads the same as "every
    // named view" or "the first", and a move written into the wrong one loses it in silence (D4).
    const other = { marker: "another named view" } as never;
    layout.receive(fromTheCore({ view: "home", layouts: { home }, named: [{ name: "Altra", layout: other }, { name: "Revisione", layout: { marker: "before" } as never }], openNamed: "Revisione" }));
    const moved = { marker: "the review, one panel moved" } as never;
    layout.settle(moved);
    // ⛔ `layouts.home` AS IT WAS: before R3-19 a settle wrote `layouts[view]` whatever was on screen.
    expect(sentPack(bridge, 0)).toEqual({ view: "home", layouts: { home }, named: [{ name: "Altra", layout: other }, { name: "Revisione", layout: moved }], openNamed: "Revisione" });
    layout.chooseTheme("light");
    // And a theme chosen meanwhile keeps the named view open.
    expect(sentPack(bridge, 1)).toEqual({ view: "home", layouts: { home }, named: [{ name: "Altra", layout: other }, { name: "Revisione", layout: moved }], openNamed: "Revisione", theme: "light" });
  });

  it("shows one of the three by closing the named view, saves nothing for showing, and settles into the three after", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const review = { marker: "review" } as never;
    layout.receive(fromTheCore({ view: "home", layouts: {}, named: [{ name: "Revisione", layout: review }], openNamed: "Revisione" }));
    layout.showView("compact");
    expect(layout.openNamed).toBeNull();
    expect(layout.view).toBe("compact");
    // ⛔ SHOWING IS NOT SAVING (decision 11).
    expect(bridge.sent).toEqual([]);
    const compact = { marker: "compact, moved" } as never;
    layout.settle(compact);
    expect(sentPack(bridge, 0)).toEqual({ view: "compact", layouts: { compact }, named: [{ name: "Revisione", layout: review }] });
  });

  it("saves the layout on screen under a new name at once and opens it, keeps those saved before, and refuses an empty or a taken name (D4)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    // ⛔ A NAMED VIEW SAVED BEFORE (E82 of the plan): the new name joins the list, and the one before stays.
    const before = { marker: "saved before" } as never;
    layout.receive(fromTheCore({ view: "home", layouts: {}, named: [{ name: "Prima", layout: before }] }));
    // The names the frame shows for the three views: the words are the locale's, and the store reads none.
    const shown = ["Home", "Lavoro", "Compatta"];
    const now = { marker: "on screen" } as never;
    expect(layout.saveNamed("   ", now, shown)).toBe("empty");
    expect(layout.saveNamed("home", now, shown)).toBe("taken");
    expect(bridge.sent).toEqual([]);
    expect(layout.saveNamed(" Revisione ", now, shown)).toBe("saved");
    expect(layout.openNamed).toBe("Revisione");
    expect(sentPack(bridge, 0)).toEqual({ view: "home", layouts: {}, named: [{ name: "Prima", layout: before }, { name: "Revisione", layout: now }], openNamed: "Revisione" });
    // ⛔ NOT OVERWRITTEN: the same name again, in another case, is refused and nothing more is sent.
    expect(layout.saveNamed("REVISIONE", { marker: "another" } as never, shown)).toBe("taken");
    expect(bridge.sent).toHaveLength(1);
  });

  it("closes the named view when the core answers without it -- an old package, or none it can read -- and the next move goes into the three (E75)", () => {
    // ⛔ A WRITE THAT DID NOT STICK (decision 13): the core answers with what it holds, the same before and after the
    // save -- a package without the name, the case the rule is for (E83 of the plan), or nothing, where E75 was found.
    const answers: IpcMessage[] = [fromTheCore({ view: "home", layouts: {} }), { kind: "Layout", value: { state: "Nothing" } }];
    for (const held of answers) {
      setActivePinia(createPinia());
      const bridge = createFakeBridge();
      const layout = useLayout();
      layout.attach(bridge);
      layout.receive(held);
      expect(layout.saveNamed("Revisione", { marker: "on screen" } as never, ["Home", "Lavoro", "Compatta"])).toBe("saved");
      // The dock shows the view of always, and a name left open would send the next move nowhere: not into
      // `layouts`, and not into `named`, which no longer holds it.
      layout.receive(held);
      expect(layout.openNamed).toBeNull();
      const moved = { marker: "home, moved" } as never;
      layout.settle(moved);
      expect(sentPack(bridge, 1)).toEqual({ view: "home", layouts: { home: moved } });
    }
  });
});
```

In `gui/src/frame/frame.test.ts`, *Trova* — la fine del `describe("the dock", …)`:

```ts
    expect(harnessTheme()).toMatchObject({ name: "harness", className: "dockview-theme-harness" });
    expect(harnessTheme().gap).toBeGreaterThan(0);
  });
});
```

*Sostituisci con:*

```ts
    expect(harnessTheme()).toMatchObject({ name: "harness", className: "dockview-theme-harness" });
    expect(harnessTheme().gap).toBeGreaterThan(0);
  });

  it("shows the named view that is open, and the view of always once it closes -- saving neither (the (d))", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const api = createDock(host());
    api.layout(1600, 1000);
    // ⛔ TWO NAMED VIEWS, THE OPEN ONE SECOND (E82 of the plan): with one alone, "the one open" reads the same as
    // "the first".
    const first = grid();
    first.addPanel({ id: "costs", component: "costs", title: "costs", params: placeholderParams("costs") });
    layout.receive(packageFromTheCore({ view: "home", layouts: {}, named: [{ name: "Prima", layout: first.toJSON() }, { name: "Revisione", layout: ownersHome() }], openNamed: "Revisione" }));
    await flush();
    expect(showing(api)).toEqual(["status"]);
    layout.showView("home");
    await flush();
    // ⛔ THE SHIPPED HOME, NOT THE REVIEW UNDER ITS NAME: the dock watches `openNamed` as it watches `view`.
    expect(showing(api)).toEqual(Object.keys(VIEWS.home.panels ?? {}).sort());
    expect(saves(bridge)).toBe(0);
  });
});
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/nearest.test.ts src/frame/schematic.test.ts src/stores/stores.test.ts src/frame/frame.test.ts)
```

Atteso: **rosso**, e per le ragioni giuste — misurato il 2026-09-23: `Failed to resolve import "./nearest"` e `Failed to
resolve import "./schematic"`; le **sei** prove nuove del negozio, `expected undefined to deeply equal [ { name: 'Revisione',
…(1) } ]`, `expected { view: 'home', layouts: { home: {} } } to deeply equal { view: 'home', …(2) }`, `expected { view:
'home', …(1) } to deeply equal { view: 'home', …(3) }`, `TypeError: layout.showView is not a function` e, due volte — la seconda è la prova di **E75** —, `TypeError:
layout.saveNamed is not a function`; e il dock, `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]` — la
Home spedita al posto della vista col nome. Verdi tutte le prove che c'erano. ✅ **Rimisurato il 2026-09-27** dal pre-controllo,
sulla copia a `1c168a1` col testo corretto: `Tests  7 failed | 28 passed (35)`, coi messaggi di qui sopra.

- [ ] **Passo 3: la geometria comune — `nearest`, e `moveActive` che la usa**

Crea `gui/src/frame/nearest.ts` (LF):

```ts
/** A rectangle, as `getBoundingClientRect` gives it and as a probe writes it by hand. */
export interface Box {
  left: number;
  top: number;
  right: number;
  bottom: number;
}

export type Direction = "left" | "right" | "up" | "down";

/**
 * The candidate nearest to `from` in a direction: the geometry of move 6 of SP-8, the tiles moved with the keyboard, and,
 * from task 8, of the arrows in the overview's grid (decision 19 of the design system) -- its second occurrence, so it
 * lives here once.
 * Only what lies BEYOND `from` in that direction counts, and the nearest is the smallest gap on that axis.
 *
 * ⛔ A TIE IS BROKEN ON THE OTHER AXIS, by the centre nearest to `from`'s (R3-18 of the design-system review): in a grid
 * every card of the row below is equally far, and the gap alone sent "down" to the first column from any column.
 *
 * ⚠️ UNDER jsdom EVERY RECT IS ZERO: the probes hand rectangles of their own (`nearest.test.ts`, `keys.test.ts`), and the
 * browser is where the real ones are seen.
 */
export function nearest<T extends { rect: Box }>(from: Box, candidates: readonly T[], direction: Direction): T | undefined {
  const beyond = (rect: Box): boolean =>
    direction === "left" ? rect.right <= from.left + 1
    : direction === "right" ? rect.left >= from.right - 1
    : direction === "up" ? rect.bottom <= from.top + 1
    : rect.top >= from.bottom - 1;
  const gap = (rect: Box): number =>
    direction === "left" ? from.left - rect.right
    : direction === "right" ? rect.left - from.right
    : direction === "up" ? from.top - rect.bottom
    : rect.top - from.bottom;
  const across = (rect: Box): number =>
    direction === "left" || direction === "right"
      ? Math.abs(rect.top + rect.bottom - from.top - from.bottom) / 2
      : Math.abs(rect.left + rect.right - from.left - from.right) / 2;
  return candidates
    .filter(({ rect }) => beyond(rect))
    .sort((a, b) => gap(a.rect) - gap(b.rect) || across(a.rect) - across(b.rect))[0];
}
```

In `gui/src/frame/moveActive.ts` (`replace_unique.py`), due sostituzioni. *Trova*:

```ts
import type { DockviewApi, Position } from "dockview-core";

export type Direction = "left" | "right" | "up" | "down";
export type Moved = "moved" | "split" | "none";
```

*Sostituisci con:*

```ts
import type { DockviewApi, Position } from "dockview-core";

import { nearest, type Direction } from "./nearest";

export type Moved = "moved" | "split" | "none";
```

*Trova:*

```ts
 * instead, and the browser is where the reviewer sees the real thing (rule 5 of the head).
 */
export function moveActive(api: DockviewApi, direction: Direction): Moved {
  const panel = api.activePanel;
  if (panel === undefined) return "none";
  const from = panel.group.element.getBoundingClientRect();
  const beyond = (rect: DOMRect): boolean =>
    direction === "left" ? rect.right <= from.left + 1
    : direction === "right" ? rect.left >= from.right - 1
    : direction === "up" ? rect.bottom <= from.top + 1
    : rect.top >= from.bottom - 1;
  const gap = (rect: DOMRect): number =>
    direction === "left" ? from.left - rect.right
    : direction === "right" ? rect.left - from.right
    : direction === "up" ? from.top - rect.bottom
    : rect.top - from.bottom;
  const target = api.groups
    .filter((group) => group !== panel.group && !group.locked)
    .map((group) => ({ group, rect: group.element.getBoundingClientRect() }))
    .filter(({ rect }) => beyond(rect))
    .sort((a, b) => gap(a.rect) - gap(b.rect))[0];
```

*Sostituisci con:*

```ts
 * instead, and the browser is where the reviewer sees the real thing (rule 5 of the head).
 * The geometry itself is `nearest`, shared with the overview's grid from task 8 of the design system.
 */
export function moveActive(api: DockviewApi, direction: Direction): Moved {
  const panel = api.activePanel;
  if (panel === undefined) return "none";
  const candidates = api.groups
    .filter((group) => group !== panel.group && !group.locked)
    .map((group) => ({ group, rect: group.element.getBoundingClientRect() }));
  const target = nearest(panel.group.element.getBoundingClientRect(), candidates, direction);
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/nearest.test.ts src/frame/keys.test.ts)
```

Atteso: **verde** — le tre prove di `nearest`, e le cinque di `keys.test.ts` com'erano: la geometria dei pannelli è la stessa,
più lo spareggio, che fra i gruppi di una disposizione vera cambia soltanto i pareggi esatti.

- [ ] **Passo 4: lo schema di una disposizione**

Crea `gui/src/frame/schematic.ts` (LF) — D5:

```ts
import { Orientation, type SerializedDockview } from "dockview-core";

/** One group of a layout, as a miniature draws it: where it sits, in fractions of the whole, and its panels. */
export interface Tile {
  x: number;
  y: number;
  width: number;
  height: number;
  views: string[];
  active?: string;
}

/** A node of the serialized grid, as `toJSON()` writes it: a branch holds nodes, a leaf holds a group. */
interface GridNode {
  type: "branch" | "leaf";
  data: GridNode[] | { views: string[]; activeView?: string };
  size?: number;
}

/**
 * The miniature of a saved layout (answer 19 of the design system): the groups of its grid as rectangles in fractions of
 * the unit square, drawn from the tree and not from a `dockview` of their own -- they always say what the layout holds,
 * cost almost nothing and hold with ten views.
 *
 * ⛔ THE ORIENTATION ALTERNATES AT EVERY LEVEL, starting from `grid.orientation`: the root lays its children along it and
 * each branch below along the other axis -- how `dockview-core` 8.3.1 reads the tree back (`_deserializeNode` hands
 * `orthogonal(orientation)` to the children). A node's `size` is its extent along its parent's axis.
 * ⛔ THE FLOATING GROUPS ARE NOT DRAWN (D5 of the plan): they have no place in the grid.
 * ⛔ A MAXIMIZED GROUP IS DRAWN ALONE, ON THE WHOLE SQUARE (E85 of the plan): `dockview-core` 8.3.1 writes the grid as it
 * is with nothing maximized, plus `grid.maximizedNode` -- the indices from the root down to that group, which the public
 * type `SerializedDockview` does not declare -- and `fromJSON` opens the layout with the group maximized: what opens is
 * what is drawn. ⚠️ A node hidden with `setVisible` is written with `visible: false` and the size it had: nothing here
 * hides one, and the miniature would draw it.
 */
export function schematic(layout: SerializedDockview): Tile[] {
  const tiles: Tile[] = [];
  const place = (node: GridNode, box: Omit<Tile, "views" | "active">, orientation: Orientation): void => {
    if (node.type === "leaf") {
      const group = node.data as { views: string[]; activeView?: string };
      tiles.push(group.activeView === undefined ? { ...box, views: group.views } : { ...box, views: group.views, active: group.activeView });
      return;
    }
    const children = node.data as GridNode[];
    const total = children.reduce((sum, child) => sum + (child.size ?? 0), 0);
    const next = orientation === Orientation.HORIZONTAL ? Orientation.VERTICAL : Orientation.HORIZONTAL;
    let offset = 0;
    for (const child of children) {
      const share = total > 0 ? (child.size ?? 0) / total : 1 / children.length;
      place(
        child,
        orientation === Orientation.HORIZONTAL
          ? { x: box.x + offset * box.width, y: box.y, width: share * box.width, height: box.height }
          : { x: box.x, y: box.y + offset * box.height, width: box.width, height: share * box.height },
        next,
      );
      offset += share;
    }
  };
  const { maximizedNode } = layout.grid as { maximizedNode?: { location: number[] } };
  const shown = (maximizedNode?.location ?? []).reduce<GridNode>(
    (node, index) => (node.data as GridNode[])[index] as GridNode,
    layout.grid.root as unknown as GridNode,
  );
  place(shown, { x: 0, y: 0, width: 1, height: 1 }, layout.grid.orientation);
  return tiles;
}
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/schematic.test.ts)
```

Atteso: **verde**, tre prove — la terza, del gruppo ingrandito, è della cura della revisione (**E85**).

- [ ] **Passo 5: il negozio — le viste col nome, `showView`, `saveNamed`**

Riscrivi `gui/src/stores/layout.ts` per intero, col terminatore che ha oggi — D3, D4, D12, D13 e R3-19:

```ts
import type { SerializedDockview } from "dockview-core";
import { defineStore } from "pinia";
import { computed, ref } from "vue";

import type { IpcMessage, LayoutState } from "../schema/messages";
import { isThemeChoice, type ThemeChoice } from "../tokens/theme";
import type { Bridge } from "../transport/bridge";

export type ViewName = "home" | "work" | "compact";

/**
 * ⛔ ONE LAYOUT PER VIEW, AND THE PACKAGE CARRIES THEM ALL (D80). The opening paragraph of §2 of the
 * north star makes the layout "which view is open, FOR EVERY VIEW where the panels are", and row 6 has a
 * saved view win over the default BY NAME. A package with a single `layout` put Home's layout
 * under the Lavoro tab and lost Home at the next settle -- and it compiled and passed every probe.
 *
 * `layouts` IS PARTIAL ON PURPOSE: a view the owner never touched has NO entry and falls back to
 * the shipped one, which is what keeps decision 11 true -- the shipped views stay in `gui/`, and
 * an update that improves one still reaches whoever has not touched it.
 */
export interface LayoutPack {
  view: ViewName;
  layouts: Partial<Record<ViewName, SerializedDockview>>;
  /** ⛔ OPTIONAL, AND THAT IS THE COMPATIBILITY (answer 16 of the design system): a package written before
   * the field existed opens as `system`, and the core keeps the bytes without opening them -- the kernel
   * does not change. */
  theme?: ThemeChoice;
  /** The views the owner saved under a name (the (d); §2 of the north star). ⛔ OPTIONAL, AND A LIST OF THEIR
   * OWN, not keys of `layouts` (D3 of the design-system plan): `view` stays one of the three, so a package
   * written by this build opens in an older one on the view of always. */
  named?: NamedView[];
  /** The named view that is open, by name; absent while one of the three is. */
  openNamed?: string;
}

export interface NamedView {
  name: string;
  layout: SerializedDockview;
}

const VIEWS: readonly ViewName[] = ["home", "work", "compact"];

function isViewName(value: unknown): value is ViewName {
  return typeof value === "string" && (VIEWS as readonly string[]).includes(value);
}

/** Two names are the same name when they differ only in the spaces around them or in case: side by side in the overview
 * they would read as one (D12 of the design-system plan, chosen by the owner on 2026-09-23). */
function sameName(a: string, b: string): boolean {
  return a.trim().toLocaleLowerCase("it") === b.trim().toLocaleLowerCase("it");
}

/** The named views of a package. ⛔ AN ENTRY WITHOUT A NAME OR A LAYOUT IS DROPPED, AND SO IS A SECOND ENTRY UNDER A NAME
 * ALREADY READ -- the first stays: two views never share a name (D4). */
function readNamed(value: unknown): NamedView[] {
  if (!Array.isArray(value)) return [];
  const read: NamedView[] = [];
  for (const entry of value) {
    if (typeof entry !== "object" || entry === null) continue;
    const { name, layout } = entry as { name?: unknown; layout?: unknown };
    if (typeof name !== "string" || name.trim() === "" || typeof layout !== "object" || layout === null) continue;
    if (read.some((kept) => sameName(kept.name, name))) continue;
    read.push({ name, layout: layout as SerializedDockview });
  }
  return read;
}

function sameBytes(a: readonly number[], b: readonly number[]): boolean {
  return a.length === b.length && a.every((byte, index) => byte === b[index]);
}

/**
 * ⛔ THE PACKAGE IS OPAQUE TO THE CORE AND STRUCTURED ONLY HERE (row 1 of §2 of the north star):
 * the core keeps bytes and hands them back, and if `dockview` changes format the core does not
 * change. So the shape above is the gui's business alone, and the wire carries `number[]`.
 *
 * ⛔ AND WHAT COMES BACK IS NOT TRUSTED TO BE OURS: an archive can hold a package written by an
 * older build. `unpack` returns `null` on anything it does not recognise, and the caller falls
 * back to the committed views -- the same shape as row 8 of §2, where a panel pointing at a type
 * that is gone says so and closes.
 */
export const useLayout = defineStore("layout", () => {
  const state = ref<LayoutState>({ state: "Nothing" });
  const view = ref<ViewName>("home");
  /** The named view on screen, or `null` while one of the three is: the dock watches it as it watches `view`. */
  const openNamed = ref<string | null>(null);
  /** The package we hold: the last one the core sent, or our own last save while its echo is in
   * flight. What `apply` reads. */
  const saved = ref<LayoutPack | null>(null);
  /** ⛔ HOW MANY PACKAGES ARRIVED THAT ARE NOT THE ECHO OF OUR OWN SAVE (D89): the dock watches
   * it and shows what arrived. A counter and not an event, because the dock is a Vue watcher. */
  const arrivals = ref(0);
  let wire: Bridge | null = null;
  let sent: readonly number[] | null = null;

  function attach(bridge: Bridge): void {
    wire = bridge;
  }

  function receive(message: IpcMessage): void {
    if (message.kind !== "Layout") return;
    state.value = message.value;
    // ⛔ THE CORE ANSWERS EVERY `SaveLayout` WITH WHAT IT HOLDS (decision 13, task 7), so a package
    // equal to the bytes we last sent is our own save coming back: nothing new, nothing to show.
    // Anything else -- the welcome, the OLD package after a write that did not stick, a package
    // another build wrote -- replaces what we hold, and the dock shows it (D89).
    if (message.value.state === "Package" && sent !== null && sameBytes(message.value.bytes, sent)) return;
    saved.value = unpack(message.value);
    if (saved.value !== null) view.value = saved.value.view;
    // ⛔ AND THE NAMED VIEW THAT IS OPEN (R3-19 of the design-system review), or none -- ALSO WHEN NOTHING
    // READABLE ARRIVED (E75 of the design-system plan): the dock then shows the view of always, and a name left
    // open would send the next move nowhere.
    openNamed.value = saved.value?.openNamed ?? null;
    arrivals.value += 1;
  }

  /** The package we hold, with what is on screen: the view of always, and the named view if one is open. */
  function onScreen(): LayoutPack {
    const pack: LayoutPack = { ...(saved.value ?? { layouts: {} }), view: view.value };
    if (openNamed.value === null) delete pack.openNamed;
    else pack.openNamed = openNamed.value;
    return pack;
  }

  /** ⛔ AUTOMATIC, NOT A BUTTON (decision 12): when the layout settles, and when the window
   * closes. The cadence is the gui's -- it is presentation, not a kernel decision.
   *
   * ⛔ AND IT MERGES (D80): the open view's entry is replaced and the other views keep theirs. A
   * `settle` that replaced the whole package lost every view but the open one. The rest of the package
   * -- the theme, the named views -- is kept whole. */
  function settle(layout: SerializedDockview): void {
    const pack = onScreen();
    const open = pack.openNamed;
    if (open === undefined) {
      keep({ ...pack, layouts: { ...pack.layouts, [view.value]: layout } });
      return;
    }
    // ⛔ A MOVE IN A NAMED VIEW GOES TO THAT VIEW (R3-19): writing `layouts[view]` here would overwrite the view of
    // always -- Home or Lavoro -- with the named one, without an error.
    keep({ ...pack, named: (pack.named ?? []).map((entry) => (entry.name === open ? { name: open, layout } : entry)) });
  }

  /** One of the three views on screen, which closes the named one. ⛔ SHOWING IS NOT SAVING (decision 11): the choice
   * reaches the package at the next settle. */
  function showView(next: ViewName): void {
    openNamed.value = null;
    view.value = next;
  }

  /**
   * The layout on screen saved under a name, and opened ("Salva questa vista", the (d)). ⛔ SAVED AT ONCE, like a theme:
   * it is a decision. ⛔ A NAME ALREADY TAKEN IS REFUSED, NOT OVERWRITTEN (D4): overwriting would lose a view in silence.
   * Taken are the named views' names and `shown` -- the names the frame shows for the three views, which are the locale's
   * words, and a store reads none.
   */
  function saveNamed(name: string, layout: SerializedDockview, shown: readonly string[]): "saved" | "empty" | "taken" {
    const wanted = name.trim();
    if (wanted === "") return "empty";
    const taken = [...shown, ...(saved.value?.named ?? []).map((entry) => entry.name)];
    if (taken.some((other) => sameName(other, wanted))) return "taken";
    openNamed.value = wanted;
    keep({ ...onScreen(), named: [...(saved.value?.named ?? []), { name: wanted, layout }] });
    return "saved";
  }

  /** The theme of the package, and `system` when it has none (answer 16). */
  const theme = computed<ThemeChoice>(() => saved.value?.theme ?? "system");

  /** ⛔ SAVED AT ONCE, NOT AT THE NEXT SETTLE: a choice made in Impostazioni is a decision, not a movement of
   * panels, and closing the window right after it must not lose it. */
  function chooseTheme(choice: ThemeChoice): void {
    keep({ ...onScreen(), theme: choice });
  }

  function keep(pack: LayoutPack): void {
    saved.value = pack;
    const bytes = [...pack_(pack)];
    sent = bytes;
    wire?.send({ kind: "SaveLayout", value: bytes });
  }

  return { state, view, openNamed, saved, arrivals, theme, attach, receive, settle, showView, saveNamed, chooseTheme };
});

/** The package as bytes: UTF-8 of the JSON. ⚠️ Exported for the probes, which must be able to
 * build one without a store and a bridge. */
export function pack_(pack: LayoutPack): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(pack));
}

export function unpack(state: LayoutState): LayoutPack | null {
  if (state.state !== "Package") return null;
  try {
    const value: unknown = JSON.parse(new TextDecoder().decode(Uint8Array.from(state.bytes)));
    if (typeof value !== "object" || value === null) return null;
    const candidate = value as { view?: unknown; layouts?: unknown };
    if (!isViewName(candidate.view)) return null;
    if (typeof candidate.layouts !== "object" || candidate.layouts === null) return null;
    const held = candidate.layouts as Record<string, unknown>;
    const layouts: LayoutPack["layouts"] = {};
    // ⛔ ONLY THE THREE VIEWS THIS BUILD KNOWS ARE READ (decision 11): an entry under another
    // name is another build's, and is neither shown nor kept -- row 8 of §2, for views.
    for (const name of VIEWS) {
      const layout = held[name];
      if (typeof layout === "object" && layout !== null) layouts[name] = layout as SerializedDockview;
    }
    const pack: LayoutPack = { view: candidate.view, layouts };
    // ⛔ A CHOICE THIS BUILD DOES NOT KNOW IS READ AS ABSENT, and the package still opens: the layouts in it are
    // worth more than a word we cannot read.
    const theme = (candidate as { theme?: unknown }).theme;
    if (isThemeChoice(theme)) pack.theme = theme;
    const named = readNamed((candidate as { named?: unknown }).named);
    if (named.length > 0) pack.named = named;
    // ⛔ AN OPEN NAME THE LIST DOES NOT HOLD IS READ AS ABSENT, and the view of always opens.
    const open = (candidate as { openNamed?: unknown }).openNamed;
    if (typeof open === "string" && named.some((entry) => entry.name === open)) pack.openNamed = open;
    return pack;
  } catch {
    // ⛔ A PACKAGE THAT DOES NOT PARSE IS NOT AN ERROR TO SHOW: it is an old build's layout, and
    // the answer is the committed views. Row 8 of §2 asks the gui to cope, not to complain.
    return null;
  }
}
```

```bash
(cd gui && npx vitest run --project jsdom src/stores/stores.test.ts)
```

Atteso: **verde**, le prove di oggi e le **sei** nuove — `Tests  20 passed (20)`, rimisurato il 2026-09-27.

- [ ] **Passo 6: il dock guarda anche la vista col nome, e la barra passa da `showView`**

In `gui/src/frame/dock.ts` (`replace_unique.py`), tre sostituzioni. *Trova* — la testa di `createDock`, che il compito
rende falsa (**E78**):

```ts
 * ⛔ THE DOCK FOLLOWS THE STORE (D89), in the two ways the store changes under it: the bar writes
 * `layout.view`, and the core sends a package that is NOT the echo of our own save -- the welcome
 * after `Hello`, or the OLD package after a write that did not stick (decision 13). Both are shown
 * here and nowhere else: `Frame.vue` does not call `apply`, and a `Layout` that arrives after the
 * dock is up is not left in the store. Before D89 it was: `apply` ran once, before `Hello`, and
```

*Sostituisci con:*

```ts
 * ⛔ THE DOCK FOLLOWS THE STORE (D89), in the three ways the store changes under it: one of the
 * three views is shown (`showView`), a named view opens or closes (`openNamed`, the (d) of the
 * design system), and the core sends a package that is NOT the echo of our own save -- the welcome
 * after `Hello`, or the OLD package after a write that did not stick (decision 13). All three are
 * shown here and nowhere else: `Frame.vue` does not call `apply`, and a `Layout` that arrives after
 * the dock is up is not left in the store. Before D89 it was: `apply` ran once, before `Hello`, and
```

*Trova:*

```ts
  function show(view: ViewName): SerializedDockview {
    apply(api, view, layout.saved);
    return api.toJSON();
  }

  let last = show(layout.view);
  watch([() => layout.view, () => layout.arrivals], () => {
    last = show(layout.view);
  });
```

*Sostituisci con:*

```ts
  function show(): SerializedDockview {
    apply(api, layout.view, layout.saved, layout.openNamed);
    return api.toJSON();
  }

  // ⛔ AND THE NAMED VIEW THAT IS OPEN (the (d)): opening one, or closing it for one of the three, shows it here too.
  let last = show();
  watch([() => layout.view, () => layout.openNamed, () => layout.arrivals], () => {
    last = show();
  });
```

*Trova:*

```ts
 * Before D80 the one saved layout was applied under every tab.
 */
export function apply(api: DockviewApi, view: ViewName, pack: LayoutPack | null): void {
  api.fromJSON(pack?.layouts[view] ?? VIEWS[view]);
```

*Sostituisci con:*

```ts
 * Before D80 the one saved layout was applied under every tab.
 * ⛔ A NAMED VIEW WINS WHILE IT IS OPEN (the (d) of the design system), and a name the package no
 * longer holds falls back to the view of always.
 */
export function apply(api: DockviewApi, view: ViewName, pack: LayoutPack | null, named: string | null = null): void {
  const chosen = named === null ? undefined : pack?.named?.find((entry) => entry.name === named)?.layout;
  api.fromJSON(chosen ?? pack?.layouts[view] ?? VIEWS[view]);
```

In `gui/src/frame/Frame.vue` (`replace_unique.py`), due sostituzioni. *Trova*:

```ts
  // the same path -- and none of them saves a view for merely showing it (decision 11).
  layout.view = view;
```

*Sostituisci con:*

```ts
  // the same path -- and none of them saves a view for merely showing it (decision 11). Through
  // `showView` since the design system: one of the three closes the named view (the (d)).
  layout.showView(view);
```

*Trova* — il commento della tastiera, che il Passo 3 rende falso (**E86**):

```ts
// G20, move 6 of SP-8: the active tile moves in the four directions from the keyboard. The
// mapping and the geometry live in `moveActive.ts`; this is only the wire.
```

*Sostituisci con:*

```ts
// G20, move 6 of SP-8: the active tile moves in the four directions from the keyboard. The
// mapping lives in `moveActive.ts` and the geometry in `nearest.ts`; this is only the wire.
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts)
```

Atteso: **verde** — la prova nuova del dock, e quelle di D80, D81 e D89 com'erano.

- [ ] **Passo 7: tutte le prove, il *build*, il linter**

```bash
(cd gui && npm test && npm run build && npm run lint)
```

Atteso: **verde** su tutto; i file di prova più alti di quelli del compito **6bis** di **due** (**E64**), `nearest.test.ts` e
`schematic.test.ts`. ⛔ E la suite intera **più volte** — cinque corse di `npm test` — perché la prova della tastiera del
compito 5 cadeva una volta su dieci (P-19): una sua caduta qui è una voce d'errata, non una corsa da ripetere finché passa.
E la riga del pezzo JavaScript, `npm run build 2>&1 | grep -E 'assets/index-.*\.js '`, va nel commit accanto a quella del
Passo 1 (**E80**): il compito cambia la SPA. ✅ **Rimisurato il 2026-09-27** dal pre-controllo, sulla copia col testo
corretto: cinque corse su cinque `Test Files  27 passed | 1 skipped (28)` e `Tests  212 passed | 1 skipped (213)`; *build*
e linter puliti; il pezzo `693.02 kB`, compresso `211.20 kB`. ⚠️ **Richiamo del 2026-09-27, dalle cure della revisione
(E82–E88):** la prova del gruppo ingrandito è nuova, e le altre direzioni stanno in prove che c'erano — `Test Files  27
passed | 1 skipped (28)` e `Tests  213 passed | 1 skipped (214)`; il pezzo resta `693.02 kB`, compresso `211.20 kB`.

- [ ] **Passo 8: le due direzioni**

Una violazione alla volta, poi indietro con la **copia salvata** e `cmp` (vincolo 11): `layout.ts` il compito l'ha
riscritto, e `nearest.ts` e `schematic.ts` sono nati qui (A-1).

| La prova | La violazione messa a mano | Atteso, misurato il 2026-09-23 e rimisurato il 2026-09-27 (**E77**) |
|---|---|---|
| `nearest.test.ts`, lo spareggio | in `nearest.ts` tolto `\|\| across(a.rect) - across(b.rect)` dal `sort` | rosso, due prove: `r1c2 up: expected 'r0c0' to be 'r0c2'` e `expected 'r1c0' to be 'r1c1'` — il difetto di R3-18 |
| `nearest.test.ts`, ciò che sta oltre | in `nearest.ts` tolta la riga `.filter(({ rect }) => beyond(rect))` | rosso, **quattro** prove: `r0c1 right: expected 'r0c0' to be 'r0c2'`, `expected 'r0c1' to be 'r1c1'`, `expected { name: 'r1c0', …(1) } to be undefined`; e in `keys.test.ts` *«splits its own group on that side when nothing lies there»*, `expected 'moved' to be 'split'` — dal Passo 3 `moveActive` passa da `nearest` (**E77**) |
| `schematic.test.ts`, l'alternanza | in `schematic.ts` `const next = orientation;` | rosso, due prove: `expected [ …(3) ] to deeply equal [ …(3) ]` e `home: the strip: expected false to be true` |
| `schematic.test.ts`, i galleggianti | in `schematic.ts`, dopo la chiamata a `place`, una riga che aggiunge a `tiles` ogni `layout.floatingGroups` | rosso: `expected [ …(4) ] to deeply equal [ …(3) ]` |
| `stores.test.ts`, i doppioni | in `layout.ts` tolta la riga `if (read.some((kept) => sameName(kept.name, name))) continue;` | rosso: `expected { view: 'home', …(2) } to deeply equal { view: 'home', …(2) }` — due viste sotto un nome |
| `stores.test.ts`, il nome aperto che non c'è | in `unpack`, `if (typeof open === "string") pack.openNamed = open;` | rosso: `expected { view: 'home', …(3) } to deeply equal { view: 'home', …(2) }` |
| `stores.test.ts`, la mossa nella vista col nome | in `settle`, `if (true) {` al posto di `if (open === undefined) {` | rosso: `expected { view: 'home', …(3) } to deeply equal { view: 'home', …(3) }` — `layouts.home` sovrascritta, R3-19 |
| `stores.test.ts`, il nome aperto che arriva | in `receive` tolta la riga di `openNamed` | rosso, **quattro** prove: `expected null to be 'Revisione'`; la mossa che finisce nelle tre, `expected { view: 'home', …(2) } to deeply equal { view: 'home', …(3) }`; la prova di **E75**, `expected 'Revisione' to be null`; e in `frame.test.ts` il dock, `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]` (**E77**) |
| `stores.test.ts`, ciò che arriva illeggibile | in `receive`, `if (saved.value !== null) openNamed.value = saved.value.openNamed ?? null;` al posto della riga di `openNamed` — la forma di prima di **E75** | rosso: `expected 'Revisione' to be null` — il nome resta aperto su una vista che il pacchetto non ha più |
| `stores.test.ts`, `showView` | in `showView` tolta la riga `openNamed.value = null;` | rosso, **due** prove: `expected 'Revisione' to be null`; e in `frame.test.ts` il dock, `expected [ 'status' ] to deeply equal [ 'activity', 'costs', …(5) ]` (**E77**) |
| `stores.test.ts`, i nomi delle tre viste | in `saveNamed` tolto `...shown,` da `taken` | rosso: `expected 'saved' to be 'taken'` |
| `stores.test.ts`, le maiuscole | in `sameName` `return a === b;` | rosso, due prove: i doppioni e `expected 'saved' to be 'taken'` |
| `stores.test.ts`, lo schermo | in `onScreen` le due righe dell'`if` sostituite da `if (openNamed.value !== null) pack.openNamed = openNamed.value;` | rosso: `expected { view: 'compact', layouts: {}, …(2) } to deeply equal { view: 'compact', …(2) }` — il nome chiuso restava nel pacchetto |
| `frame.test.ts`, il dock che guarda | in `dock.ts` tolto `() => layout.openNamed,` dal `watch` | rosso: `expected [ 'status' ] to deeply equal [ 'activity', 'costs', …(5) ]` |
| `frame.test.ts`, `apply` | in `apply` `const chosen = undefined;` | rosso: `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]` |
| `stores.test.ts`, la mossa nelle altre viste col nome | in `settle`, `.map((entry) => ({ name: entry.name, layout }))` al posto del `.map` che sceglie per nome | rosso: `expected { view: 'home', …(3) } to deeply equal { view: 'home', …(3) }` — la mossa scritta in **ogni** vista col nome (**E82**) |
| `stores.test.ts`, la mossa nella prima | in `settle`, `.map((entry, index) => (index === 0 ? { name: open, layout } : entry))` | rosso, lo stesso messaggio: l'aperta è la **seconda** (**E82**) |
| `stores.test.ts`, le viste col nome di prima | in `saveNamed`, `named: [{ name: wanted, layout }]` | rosso: `expected { view: 'home', layouts: {}, …(2) } to deeply equal { view: 'home', layouts: {}, …(2) }` — la vista salvata prima, persa (**E82**) |
| `stores.test.ts`, il pacchetto vecchio | in `receive`, `openNamed.value = saved.value === null ? null : (saved.value.openNamed ?? openNamed.value);` al posto della riga di `openNamed` | rosso: `expected 'Revisione' to be null` — il nome tenuto dopo un pacchetto che non lo ha (**E83**) |
| `stores.test.ts`, la voce nulla | in `readNamed` tolta la riga `if (typeof entry !== "object" \|\| entry === null) continue;` | rosso: `expected null to deeply equal { view: 'home', …(2) }` — il pacchetto perso intero (**E88**) |
| `stores.test.ts`, la disposizione nulla | in `readNamed` tolto `\|\| layout === null` | rosso: `expected { view: 'home', …(2) } to deeply equal { view: 'home', …(2) }` (**E88**) |
| `schematic.test.ts`, le sovrapposizioni | in `schematic.ts` `x: offset * box.width` al posto di `x: box.x + offset * box.width` | rosso: `work: scope over steps: expected 0.07846999999999998 to be less than 1e-9` (**E84**) |
| `schematic.test.ts`, il gruppo ingrandito | in `schematic.ts` `([] as number[])` al posto di `(maximizedNode?.location ?? [])` | rosso: `expected [ { x: +0, y: +0, …(4) }, …(6) ] to deeply equal [ { x: +0, y: +0, width: 1, …(3) } ]` (**E85**) |
| `frame.test.ts`, quale vista col nome | in `apply` `pack?.named?.[0]?.layout` al posto della ricerca per nome | rosso: `expected [ 'costs' ] to deeply equal [ 'status' ]` — il dock mostra la prima (**E82**) |

⚠️ **Due rami difensivi restano senza sonda, dichiarati (E89):** in `schematic.ts` la divisione in parti uguali quando
le misure di un ramo sommano a zero, e in `apply` il ritorno alla vista di sempre per un nome che il pacchetto non ha —
tolto l'uno o l'altro, la suite intera resta verde, misurato dalla revisione del compito.

Alla fine `git status --porcelain | diff <scratchpad>/prima.txt -` rende soltanto i file del compito.

- [ ] **Passo 9: il cancello, il commit, la posizione**

La riga **7** della tabella della posizione — **Stato** `✅ <data>`; la riga **6** porta già la colonna **Commit**, scritta
dalla chiusura del 6bis con quella della riga **6bis**, e non si tocca (**E79**) —; `bash scripts/gate.sh` da solo, `bash
scripts/check-docs.sh`, il commit — `design-system(compito 7): le viste col nome, sotto …`, con le due righe del pezzo
JavaScript (**E80**) — coi fine-riga rimisurati, e `git push`.

---
## Compito 8: la cornice — la barra col nome della vista, la Panoramica, la striscia a pillola

**Da:** la (d) del disegno, *«La barra»*, *«La Panoramica»* e la riga *«la striscia»* di *«La finestra»*; i controlli **17**
e **19**; le decisioni **18**–**20** del disegno; **P-11**, **P-22**…**P-25**, **D4**, **D12**, **D14**…**D18** di questo
piano; R3-20, R3-23 e R3-25 della revisione. La tavola della [Panoramica](../specs/2026-09-22-design-system-tavole/panoramica.html)
dà la griglia a tre colonne, la carta corrente e *«Salva questa vista»* in fondo.

**Files:**
- Create: `gui/src/stores/drawer.ts`, `gui/src/frame/Overview.vue`
- Rewrite: `gui/src/frame/ViewBar.vue`, `gui/src/frame/Frame.vue`, `gui/src/frame/Drawer.vue`, `gui/src/panels/Strip.vue`,
  `gui/src/panels/views/compact.json` (**E103**) — ciascuno **per intero**, col terminatore che ha oggi
- Modify: `gui/src/stores/invoke.ts`, `gui/src/stores/layout.ts` (**E99**), `gui/src/components/Confirm.vue`,
  `gui/src/locales/it.json`, `gui/src/tokens/dock.css`, `gui/src/frame/dock.ts` (**E104**), `gui/src/components/BaseButton.vue`
  (**E105**), `gui/src/panels/views/generate-views.test.ts` (**E103**)
- Modify: `gui/src/frame/frame.test.ts`, `gui/src/frame/frame.browser.test.ts` — nato col 6bis (**D29**, **E63**) —,
  `gui/src/stores/stores.test.ts`, `gui/src/a11y.test.ts`, `gui/src/locales/copy.test.ts`, `gui/src/frame/dock.browser.test.ts`,
  `gui/src/kit/kit.browser.test.ts`, `gui/src/testing/axe.ts`, `gui/src/testing/probes.ts` (**E92**),
  `gui/src/testing/probes.browser.test.ts` (**E108**)

**Interfaces:**
- Consumes: `BaseButton` — `variant="card"`, `pill`, `size="lg"`, `icon` —, `BaseDialog` — `variant="full"` e `"sheet"`,
  `v-model:open`, lo slot `trigger` —, `BaseLabel`, `BaseIcon`, `BaseTextField` con `error`, `isIconName` e `type IconName`
  (compito 3); `violations` di `testing/axe.ts` (compito 3); `concentricRadii`, `fits`, `iconsCentred` di
  `testing/probes.ts` (compito 4); il progetto `browser` e `userEvent` (compito 2); `readToken` (compito 6); in `useLayout()`
  `showView`, `openNamed`, `saved.named` e `saveNamed(name, layout, shown)`, e `nearest` con `type Direction`, e
  `schematic` (compito 7).
- Produces: `useDrawer()` da `stores/drawer.ts`, con `open: boolean`; in `useInvoke()` il getter `asking: boolean`; in
  `useLayout()` `showNamed(name: string): boolean` (**E99**); `computed(property, token)` da `testing/probes.ts` (**E92**),
  che lancia su un token che la pagina non definisce (**E108**); `Overview.vue` — la prop `snapshot: () => SerializedDockview`
  e `v-model:open` —; in `ViewBar.vue` la prop `snapshot` e
  `v-model:overview`; `contrastJudged(node: Element): Promise<{ passes: number; incomplete: number }>` da
  `testing/axe.ts`; in `it.json` le chiavi `overview.*`, e `drawer.open` che dice *«moduli»*; la classe `view-name` sul nome
  della vista e l'attributo `data-card` — `view`, `save` — sulle carte, che le prove leggono.

⚠️ **Che cosa la scrittura di questo compito ha misurato**, il 2026-09-24 sulla cartella di prova coi compiti 1–7 e le
cure di P-20 e P-21, nel Chrome installato 153 e sotto jsdom; le righe intere sono **P-22**…**P-25**, in testa al piano:

| | Il fatto | Che cosa ne fa il compito |
|---|---|---|
| 1 | chiuso con Esc, il cassetto aperto dal negozio rende il fuoco al pulsante *«moduli»* della striscia, benché nessun `DialogTrigger` lo registri: il pulsante sta in un'altra app di Vue (**P-22**) | nessuna cura; una prova nel browser tiene il fatto, se un aggiornamento di `reka-ui` lo cambiasse |
| 2 | `mount` di `@vue/test-utils` monta l'app in un `div` suo dentro l'elemento che riceve, e quel `div` non ha altezza: la cornice alta 116 px, il dock 0, la striscia a metà pagina (**P-23**) | la prova nel browser monta `App.vue` come la monta `main.ts`, con `createApp` su `#app` |
| 3 | la pillola della striscia è alta **50** px: le viste spedite danno alla sua riga 56, e lo spazio fra le schede se ne prende metà di `--space-3` (**P-24**) | il pulsante grande, 40, sta a `--space-1` più il bordo dal filo della pillola, tutto intorno |
| 4 | senza un core la fascia *«Il core non ha risposto»* sta fra la barra e il dock, e il dock prende il resto: la striscia resta a 24 px dal fondo | niente: le prove misurano la cornice com'è |
| 5 | aperta la finestra, `reka-ui` dà il fuoco al primo elemento raggiungibile col Tab: con una carta sola nel giro del Tab (**D17**) è la vista che si vede | la prova delle frecce parte da lì |
| 6 | il commento di `testing/axe.ts` contava i suoi utenti, e il compito 4 ne aggiunge un terzo (**P-25**) | il commento è corretto nel compito 3; qui `contrastJudged` nasce senza conto |

- [ ] **Passo 1: rimisura il punto di partenza**

```bash
git status --porcelain > <scratchpad>/prima.txt
grep -rnw 'F3' gui/src
ls gui/src/stores/drawer.ts
grep -rn 'bar\.views' gui/src
grep -n '"reka-ui"' gui/package.json
(cd gui && npm run build 2>&1 | grep -E 'assets/index-.*\.js ')
```

Atteso: `F3` **in nessun file** (P-11) — con `-w`, perché `#F3EEE6` di `themes.css` è un colore (**E91**); `drawer.ts` **non
c'è**; `bar.views` solo in `frame/ViewBar.vue`, che il compito riscrive — e la chiave esce da `it.json`; `reka-ui` a
**2.10.4**, la versione su cui P-22 è misurato; il pezzo JavaScript, la baseline che il Passo 8 porta nel commit (**E80**):
`693.02 kB`, compresso `211.20 kB`, a `f3206c7`. Poi, da solo, `bash scripts/gate.sh` → `GATE GREEN`.

- [ ] **Passo 2: le prove, prima del codice**

In `gui/src/testing/axe.ts` (`replace_unique.py`) l'aiutante di R3-7 arriva dalla pagina kit — la sua seconda occorrenza è
la cornice:

*Trova*:

```ts
}

```

*Sostituisci con:*

```ts
}

/**
 * ⛔ THE NON-VACUITY OF `axe` (R3-7 of the review): an empty list of violations says something only if the contrast was
 * JUDGED -- nodes among the passes, none left incomplete. Under jsdom axe files every contrast as incomplete; in the
 * browser it must not. Its second user came with the design system's task 8 -- the frame, after the kit page -- so it
 * lives here once.
 */
export async function contrastJudged(node: Element): Promise<{ passes: number; incomplete: number }> {
  const results = await axe.run(node, { runOnly: ["color-contrast"] });
  const count = (list: axe.Result[]): number => list.find((rule) => rule.id === "color-contrast")?.nodes.length ?? 0;
  return { passes: count(results.passes), incomplete: count(results.incomplete) };
}

```

E il commento di testa: il contrasto lo accendono le sonde nel browser, senza nominarne una — dal compito 8 anche la
prova della Panoramica (**E109**):

*Trova:*

```ts
 * prove nothing there. `tokens/contrast.test.ts` holds the families of the pairs; the probes of the kit page, in the
 * real browser, turn it ON with `contrast: true` (task 4).
```

*Sostituisci con:*

```ts
 * prove nothing there. `tokens/contrast.test.ts` holds the families of the pairs; the probes in the real browser turn it
 * ON with `contrast: true`, since task 4.
```

In `gui/src/testing/probes.ts` (`replace_unique.py`), due sostituzioni — la testa del file importa `readToken`, e dice
che qui vivono anche gli oracoli che le sonde condividono (**E108**, **E109**):

*Trova*:

```ts
/**
 * The probes of the design-system boards, as functions over the real DOM (design system, section (f)). They were
 * scripts pasted into a console on 2026-09-23 -- `sonda-raggi.js`, `sonda-icone.js`, `sonda-caratteri.js`, next to the
 * approved boards -- and each was proven in both directions there. Here they take their ROOTS as a parameter (trap 4),
 * and each returns how much it looked at, so that a caller can refuse a green that looked at nothing (trap 1).
```

*Sostituisci con:*

```ts
import { readToken } from "../tokens/readToken";

/**
 * The probes of the design-system boards, and the oracles they share, as functions over the real DOM (design system,
 * section (f)). The probes were scripts pasted into a console on 2026-09-23 -- `sonda-raggi.js`, `sonda-icone.js`,
 * `sonda-caratteri.js`, next to the approved boards -- and each was proven in both directions there. Here they take their
 * ROOTS as a parameter (trap 4), and a probe returns how much it looked at, so that a caller can refuse a green that
 * looked at nothing (trap 1); an oracle returns a value.
```

E **E92**: l'oracolo di un token, che la prova del dock e quella della cornice portavano parola per parola, e la pagina
kit per il solo `color`, in una casa sola; e **E108**: risponde solo per un token che la pagina definisce, la regola di
`readToken`:

*Trova:*

```ts
/** The first family an element computes -- the check "applied" of `sonda-caratteri.js`. */
```

*Sostituisci con:*

```ts
/**
 * What a declaration of `property: var(token)` computes to here: the oracle of a token, with no copy of its value.
 * ⛔ ONE HOUSE (E73 of the design-system plan): the dock's probes and the frame's had it word for word, and the kit
 * page's for `color` alone.
 * ⛔ A TOKEN THE PAGE DOES NOT DEFINE IS AN ERROR (E108), the rule of `readToken`: a misspelt one computed the property's
 * initial value -- `0px`, `rgba(0, 0, 0, 0)`, the very value a probe of "nothing drawn" waits for -- and a green that
 * looked at nothing. ⚠️ THE NAME IS VUE'S TOO: a file that wants both imports one under an alias, and without one it
 * fails at load (`Identifier 'computed' has already been declared`).
 */
export function computed(property: string, token: string): string {
  readToken(token);
  const probe = document.createElement("div");
  probe.style.setProperty(property, `var(${token})`);
  document.body.append(probe);
  const value = getComputedStyle(probe).getPropertyValue(property);
  probe.remove();
  return value;
}

/** The first family an element computes -- the check "applied" of `sonda-caratteri.js`. */
```

In `gui/src/testing/probes.browser.test.ts` (`replace_unique.py`), due sostituzioni — **E108**: l'oracolo rifiuta un
token che la pagina non definisce, e ne calcola uno che c'è:

*Trova*:

```ts
import { concentricRadii } from "./probes";
```

*Sostituisci con:*

```ts
import { computed, concentricRadii } from "./probes";
```

*Trova:*

```ts
  box("filler", "position:absolute;left:0;top:0;width:1px;height:400px", scroller);
  expect(concentricRadii([root])).toEqual({ near: 0, bad: [] });
});
```

*Sostituisci con:*

```ts
  box("filler", "position:absolute;left:0;top:0;width:1px;height:400px", scroller);
  expect(concentricRadii([root])).toEqual({ near: 0, bad: [] });
});

// ⛔ THE ORACLE OF A TOKEN ANSWERS ONLY FOR A TOKEN THE PAGE DEFINES (E108 of the design-system plan), as `readToken` does:
// a misspelt one computed the property's initial value -- a plausible oracle, and a green that looked at nothing.
it("refuses a token the page does not define, and computes one it does (E108)", () => {
  expect(() => computed("color", "--color-txet")).toThrow(/not defined/);
  document.documentElement.style.setProperty("--probe-colour", "rgb(1, 2, 3)");
  try {
    expect(computed("color", "--probe-colour")).toBe("rgb(1, 2, 3)");
  } finally {
    document.documentElement.style.removeProperty("--probe-colour");
  }
});
```

In `gui/src/kit/kit.browser.test.ts` (`replace_unique.py`), due sostituzioni — l'aiutante esce,
e l'import lo prende da `testing/axe.ts`:

*Trova*:

```ts
import { mount } from "@vue/test-utils";
import axe from "axe-core";
import { userEvent } from "vitest/browser";
import { afterEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import { violations } from "../testing/axe";
```

*Sostituisci con:*

```ts
import { mount } from "@vue/test-utils";
import { userEvent } from "vitest/browser";
import { afterEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import { contrastJudged, violations } from "../testing/axe";
```

*Trova:*

```ts
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

```

*Sostituisci con:*

```ts
const roots = (selector: string): Element[] => [...document.querySelectorAll(selector)];

```

E le due di **E92** — `colourOf` passa dall'oracolo di `testing/probes.ts`:

*Trova:*

```ts
import { concentricRadii, firstFamily, fits, iconsCentred } from "../testing/probes";
```

*Sostituisci con:*

```ts
import { computed, concentricRadii, firstFamily, fits, iconsCentred } from "../testing/probes";
```

*Trova:*

```ts
/** A colour token as the page computes it -- `rgb(…)`, in the theme on the root -- never copied from the board. */
function colourOf(token: string): string {
  const probe = document.createElement("span");
  probe.style.color = `var(${token})`;
  document.body.append(probe);
  const colour = getComputedStyle(probe).color;
  probe.remove();
  return colour;
}
```

*Sostituisci con:*

```ts
/** A colour token as the page computes it -- `rgb(…)`, in the theme on the root -- never copied from the board. */
function colourOf(token: string): string {
  return computed("color", token);
}
```

In `gui/src/frame/dock.browser.test.ts` (`replace_unique.py`) la prova delle schede sa della striscia — dal compito 8 è una
pillola, e la prova del compito 6 diventerebbe rossa:

*Trova*:

```ts
    it("draws every group as a card, `--space-3` from its neighbours", async () => {
      const { host } = await dock(theme);
      const groups = [...host.querySelectorAll(".dv-groupview")];
      // ⛔ NON-VACUITY: the Home view has groups side by side and one above the other.
      expect(groups.length).toBeGreaterThan(2);
      for (const group of groups) {
        const style = getComputedStyle(group);
        expect(style.borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
```

*Sostituisci con:*

```ts
    it("draws every group as a card, and the strip as a pill, `--space-3` from its neighbours", async () => {
      const { host } = await dock(theme);
      const groups = [...host.querySelectorAll(".dv-groupview")];
      // ⛔ NON-VACUITY: the Home view has groups side by side and one above the other.
      expect(groups.length).toBeGreaterThan(2);
      // ⛔ THE STRIP IS THE ONE PILL (the (d), task 8 of the plan): its group holds `Strip.vue`, and every other is a card.
      const strips = groups.filter((group) => group.querySelector(".strip") !== null);
      expect(strips).toHaveLength(1);
      for (const group of groups) {
        const style = getComputedStyle(group);
        const radius = strips.includes(group) ? "--radius-full" : "--radius-card";
        expect(style.borderTopLeftRadius).toBe(computed("border-top-left-radius", radius));
```

E le due di **E92** — `computed` si prende da `testing/probes.ts`:

*Trova:*

```ts
import { concentricRadii } from "../testing/probes";
```

*Sostituisci con:*

```ts
import { computed, concentricRadii } from "../testing/probes";
```

*Trova:*

```ts
  return floating[0] as Element;
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
```

*Sostituisci con:*

```ts
  return floating[0] as Element;
}
```

In `gui/src/frame/frame.test.ts` (`replace_unique.py`), tre sostituzioni — gli import, la prova dell'ingrandimento nel
dock (**E104**), e in fondo la cornice intera sotto jsdom:

*Trova* — con `vi`, che il Passo 12 del 6bis vi aggiunge (**E64**):

```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
import { nextTick } from "vue";

import { i18n } from "../i18n";
import { PANEL_TYPES, componentFor, isModule, placeholderParams } from "../panels/registry";
import { VIEWS } from "../panels/views";
import Placeholder from "../panels/Placeholder.vue";
import type { IpcMessage } from "../schema/messages";
import { useConnection } from "../stores/connection";
import { pack_, useLayout, type LayoutPack } from "../stores/layout";
import { shownTheme } from "../tokens/theme";
import { createFakeBridge, type FakeBridge } from "../transport/fakeBridge";

import { createDock, harnessTheme } from "./dock";
```

*Sostituisci con:*

```ts
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { nextTick } from "vue";

import { i18n } from "../i18n";
import { VRAM_POLICY } from "../panels/functions";
import { PANEL_TYPES, componentFor, isModule, placeholderParams } from "../panels/registry";
import { VIEWS } from "../panels/views";
import Placeholder from "../panels/Placeholder.vue";
import type { IpcMessage } from "../schema/messages";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";
import { useDrawer } from "../stores/drawer";
import { useInvoke } from "../stores/invoke";
import { pack_, unpack, useLayout, type LayoutPack } from "../stores/layout";
import { shownTheme } from "../tokens/theme";
import { createFakeBridge, type FakeBridge } from "../transport/fakeBridge";

import Frame from "./Frame.vue";
import { createDock, harnessTheme } from "./dock";
```

La prova dell'ingrandimento, in fondo a quelle del dock — **E104**: una mossa che si salva, e mostrare che resta non
salvare:

*Trova:*

```ts
    expect(showing(api)).toEqual(Object.keys(VIEWS.home.panels ?? {}).sort());
    expect(saves(bridge)).toBe(0);
  });
});
```

*Sostituisci con:*

```ts
    expect(showing(api)).toEqual(Object.keys(VIEWS.home.panels ?? {}).sort());
    expect(saves(bridge)).toBe(0);
  });

  it("saves a group maximized and a group restored, and nothing for showing a view left or opened maximized (E104)", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const api = createDock(host());
    api.layout(1600, 1000);
    await flush();
    /** Whether the last package sent opens Home with a group maximized: `dockview-core` 8.3.1 writes `grid.maximizedNode`,
     * which its public type does not declare (E85 of the plan). */
    const homeMaximized = (): boolean => {
      const last = bridge.sent.filter((message) => message.kind === "SaveLayout").at(-1);
      const pack = last?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: last.value }) : null;
      return (pack?.layouts.home?.grid as { maximizedNode?: unknown } | undefined)?.maximizedNode !== undefined;
    };
    api.getPanel("status")?.group.api.setActive();
    await flush();
    const before = saves(bridge);
    // ⛔ A MAXIMIZE IS A MOVE, AND NOT AN `onDidLayoutChange` (E104 of the plan): `dockview-core` 8.3.1 saved it only
    // through the change of the active group it makes -- and here there is none, Stato is active already.
    api.getPanel("status")?.group.api.maximize();
    await flush();
    expect(saves(bridge)).toBe(before + 1);
    expect(homeMaximized()).toBe(true);
    // ⛔ SHOWING STAYS NOT SAVING (decision 11): leaving a view with a group maximized, and coming back to it, fire the
    // same event inside `fromJSON` -- and Home opens as it was left.
    layout.showView("work");
    await flush();
    layout.showView("home");
    await flush();
    expect(saves(bridge)).toBe(before + 1);
    expect(api.hasMaximizedGroup()).toBe(true);
    // ⛔ AND THE RESTORE IS A MOVE TOO: before, it never reached the package, and Home came back maximized.
    api.getPanel("status")?.group.api.exitMaximized();
    await flush();
    expect(saves(bridge)).toBe(before + 2);
    expect(homeMaximized()).toBe(false);
  });
});
```

In fondo, la cornice intera sotto jsdom:

*Trova:*

```ts
    expect(region.text()).toContain(i18n.global.t("band.stale"));
  });
});

```

*Sostituisci con:*

```ts
    expect(region.text()).toContain(i18n.global.t("band.stale"));
  });
});

/** The overview is open: the whole-window variant of `BaseDialog`, rendered in its portal on `body`. */
function overviewOpen(): boolean {
  return document.querySelector('.base-dialog[data-variant="full"]') !== null;
}

/** The overview's cards, views first and «Salva questa vista» last. */
function cards(): HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>("[data-card]")];
}

/** A key pressed on the window, where the frame listens: the event says whether a listener took it from the browser. */
function press(key: string): KeyboardEvent {
  const event = new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true });
  window.dispatchEvent(event);
  return event;
}

/** Writes in the field of a new view's name, as a keyboard does: the value, and the `input` event `v-model` listens to. */
function write(text: string): void {
  const field = document.querySelector<HTMLInputElement>(".naming input");
  if (field === null) throw new Error("no field for the name");
  field.value = text;
  field.dispatchEvent(new Event("input", { bubbles: true }));
}

/** ⛔ THE WHOLE FRAME, WITH ITS DOCK (the (d) of the design system): the bar, the windows, the strip inside the grid. The
 * frames are UNMOUNTED after each probe -- a frame listens on `window`, and one left mounted would answer the next F3. */
describe("the frame (the (d) of the design system)", () => {
  const frames: { unmount: () => void }[] = [];

  afterEach(() => {
    for (const frame of frames.splice(0)) frame.unmount();
    document.body.replaceChildren();
  });

  async function frame(): Promise<void> {
    frames.push(mount(Frame, { attachTo: document.body, global: { plugins: [i18n] } }));
    await flush();
  }

  it("names the view on screen in the bar, and opens the overview from the name", async () => {
    await frame();
    const name = document.querySelector<HTMLElement>(".view-name");
    expect(name?.textContent).toContain(i18n.global.t("views.home"));
    name?.click();
    await flush();
    expect(overviewOpen()).toBe(true);
    // The three views and «Salva questa vista»; the view on screen is the current card, in bordeaux.
    expect(cards().map((card) => card.getAttribute("data-card"))).toEqual(["view", "view", "view", "save"]);
    expect(cards().map((card) => card.getAttribute("aria-current"))).toEqual(["page", null, null, null]);
  });

  it("opens and closes the overview with F3, and keeps quiet while the confirmation or the drawer is open", async () => {
    await frame();
    // ⛔ F3 IS OURS (P-11): its default, the browser's "find next", is taken away.
    expect(press("F3").defaultPrevented).toBe(true);
    await flush();
    expect(overviewOpen()).toBe(true);
    press("F3");
    await flush();
    expect(overviewOpen()).toBe(false);
    // ⛔ THE SECOND DIRECTION: a window that asks something is not covered by the views.
    useDrawer().open = true;
    await flush();
    press("F3");
    await flush();
    expect(overviewOpen()).toBe(false);
    useDrawer().open = false;
    useInvoke().send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    useCore().receive({ kind: "PermissionRequired", value: { tool: "registry", resource: "arbiter", operation: "Write" } });
    await flush();
    expect(useInvoke().asking).toBe(true);
    press("F3");
    await flush();
    expect(overviewOpen()).toBe(false);
  });

  it("shows the view of the card chosen and closes, opens a named view by its name, and closes it with one of the three (E76, E82)", async () => {
    const layout = useLayout();
    await frame();
    press("F3");
    await flush();
    cards()[1]?.click();
    await flush();
    expect(layout.view).toBe("work");
    expect(layout.openNamed).toBeNull();
    expect(overviewOpen()).toBe(false);
    // ⛔ TWO NAMED VIEWS (E82): with one, "the one chosen" and "the first" are the same card.
    const named = [
      { name: "Revisione", layout: ownersHome() },
      { name: "Lettura", layout: ownersHome() },
    ];
    layout.receive(packageFromTheCore({ view: "work", layouts: {}, named }));
    await flush();
    press("F3");
    await flush();
    // The named views sit after the three, before «Salva questa vista», with a word that says they were saved.
    expect(cards()).toHaveLength(6);
    expect(cards()[4]?.textContent).toContain(i18n.global.t("overview.saved"));
    cards()[4]?.click();
    await flush();
    expect(layout.openNamed).toBe("Lettura");
    expect(document.querySelector(".view-name")?.textContent).toContain("Lettura");
    press("F3");
    await flush();
    // ⛔ ONE CARD IN BORDEAUX (the (d)): the named view on screen, and not Lavoro too -- the view of always under it.
    expect(cards().map((card) => card.getAttribute("aria-current"))).toEqual([null, null, null, null, "page", null]);
    // ⛔ ONE OF THE THREE CLOSES THE NAMED VIEW (E76, D13): the role `Frame.switchTo` had, the overview's now.
    cards()[0]?.click();
    await flush();
    expect(layout.openNamed).toBeNull();
    expect(layout.view).toBe("home");
  });

  it("says under the field a name that is empty or taken, and saves a new one and opens it (D4, D12)", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    // The core has answered, with nothing: before it «Salva questa vista» is off (E81).
    layout.receive({ kind: "Layout", value: { state: "Nothing" } });
    await frame();
    press("F3");
    await flush();
    document.querySelector<HTMLElement>('[data-card="save"]')?.click();
    await flush();
    // ⛔ THE FIELD SAYS TO THE EYE WHAT IT WANTS (E106): the board draws no visible label, and the placeholder speaks.
    expect(document.querySelector(".naming input")?.getAttribute("placeholder")).toBe(i18n.global.t("overview.name"));
    const confirm = (): void => document.querySelectorAll<HTMLElement>(".naming .base-button")[1]?.click();
    confirm();
    await flush();
    expect(document.querySelector(".naming")?.textContent).toContain(i18n.global.t("overview.empty"));
    expect(document.querySelector(".naming input")?.getAttribute("aria-invalid")).toBe("true");
    // ⛔ THE THREE VIEWS' NAMES ARE THE FRAME'S WORDS (D12): «Home» is taken in any case, spaces around or not.
    write("  home ");
    confirm();
    await flush();
    expect(overviewOpen(), "a name that is taken keeps the overview open").toBe(true);
    expect(document.querySelector(".naming")?.textContent).toContain(i18n.global.t("overview.taken"));
    expect(saves(bridge)).toBe(0);
    // ⛔ EVERY OPENING STARTS WITH NO NAME HALF WRITTEN (X4 of E107): closed with F3 on a refused name, the overview opens
    // again with no field, and the field opens empty, with no refusal under it.
    press("F3");
    await flush();
    press("F3");
    await flush();
    expect(document.querySelector(".naming")).toBeNull();
    document.querySelector<HTMLElement>('[data-card="save"]')?.click();
    await flush();
    expect(document.querySelector<HTMLInputElement>(".naming input")?.value).toBe("");
    expect(document.querySelector(".naming")?.textContent).not.toContain(i18n.global.t("overview.taken"));
    write("Revisione");
    confirm();
    await flush();
    expect(overviewOpen()).toBe(false);
    expect(layout.openNamed).toBe("Revisione");
    expect(layout.saved?.named?.map((entry) => entry.name)).toEqual(["Revisione"]);
    expect(saves(bridge)).toBe(1);
  });

  it("keeps «Salva questa vista» off until the core has answered (E81)", async () => {
    const layout = useLayout();
    await frame();
    press("F3");
    await flush();
    const save = (): HTMLButtonElement | null => document.querySelector<HTMLButtonElement>('[data-card="save"]');
    // ⛔ BEFORE THE ANSWER THE STORE HOLDS NO PACKAGE: a view saved now would send `layouts: {}`, and the core would keep
    // that -- every saved layout lost in silence, the window of E40.
    expect(save()?.disabled).toBe(true);
    layout.receive({ kind: "Layout", value: { state: "Nothing" } });
    await flush();
    expect(save()?.disabled).toBe(false);
  });

  it("draws a layout it cannot read as an empty miniature, and every other as before (E90)", async () => {
    const layout = useLayout();
    // ⛔ `unpack` keeps any object as a layout, and `schematic` throws on one it cannot read: that card goes empty, not the
    // whole overview.
    const unreadable = {} as SerializedDockview;
    layout.receive(packageFromTheCore({ view: "home", layouts: {}, named: [{ name: "Rotta", layout: unreadable }] }));
    await frame();
    press("F3");
    await flush();
    expect(cards()).toHaveLength(5);
    expect(cards()[3]?.querySelectorAll(".tile")).toHaveLength(0);
    expect(cards()[0]?.querySelectorAll(".tile").length).toBeGreaterThan(0);
  });

  it("draws each view in miniature from its layout: a tile per group, with its module's icon, and not the strip (D14)", async () => {
    await frame();
    press("F3");
    await flush();
    const icons = [...(cards()[0]?.querySelectorAll(".tile") ?? [])].map((tile) => tile.querySelector("svg")?.getAttribute("data-icon"));
    // ⛔ FROM THE LAYOUT, NOT A PICTURE (answer 19): Home ships one panel per group, and the strip is one of them.
    expect(icons.sort()).toEqual(Object.keys(VIEWS.home.panels ?? {}).filter((id) => id !== "strip").sort());
    // ⛔ AND EVERY VIEW THAT SHIPS SAYS THE TRUTH ABOUT ITS STRIP (E103): the strip's row is the strip's own height, and
    // the tiles reach down to it, as on screen -- a view written before `dockview` applied that height drew Compatta's
    // knowledge base in half the miniature.
    const percent = (value: string): number => Number(/^calc\(([^%]+)%/.exec(value)?.[1]);
    for (const [index, view] of (["home", "work", "compact"] as const).entries()) {
      const shipped = VIEWS[view];
      const strip = (shipped.panels.strip?.maximumHeight ?? 0) / shipped.grid.height;
      const tiles = [...(cards()[index]?.querySelectorAll<HTMLElement>(".tile") ?? [])];
      expect(strip, view).toBeGreaterThan(0);
      expect(tiles.length, view).toBeGreaterThan(0);
      const reach = Math.max(...tiles.map((tile) => percent(tile.style.top) + percent(tile.style.height)));
      expect(reach, view).toBeGreaterThanOrEqual(100 * (1 - strip) - 1e-6);
    }
  });

  it("opens the drawer from the strip's button", async () => {
    await frame();
    const button = document.querySelector<HTMLElement>(".strip .base-button");
    expect(button?.textContent).toContain(i18n.global.t("drawer.open"));
    button?.click();
    await flush();
    expect(useDrawer().open).toBe(true);
    expect(document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
    // ⛔ AND «CHIUDI» CLOSES IT THROUGH THE STORE (R3-20): the sheet's own button, not only Esc.
    const close = document.querySelector<HTMLElement>('.base-dialog[data-variant="sheet"] .base-button');
    expect(close?.textContent).toContain(i18n.global.t("drawer.close"));
    close?.click();
    await flush();
    expect(useDrawer().open).toBe(false);
    expect(document.querySelector('.base-dialog[data-variant="sheet"]')).toBeNull();
  });
});

```

In `gui/src/stores/stores.test.ts` (`replace_unique.py`), una sostituzione — **E99**: una vista col nome si apre solo per
un nome che la lista tiene, accanto alla prova di `showView`:

*Trova*:

```ts
  it("saves the layout on screen under a new name at once and opens it, keeps those saved before, and refuses an empty or a taken name (D4)", () => {
```

*Sostituisci con:*

```ts
  it("shows a named view only by a name the list holds, and refuses any other (E90)", () => {
    const layout = useLayout();
    const review = { marker: "review" } as never;
    layout.receive(fromTheCore({ view: "home", layouts: {}, named: [{ name: "Revisione", layout: review }] }));
    // ⛔ AN UNKNOWN NAME WOULD SEND THE NEXT MOVE NOWHERE -- the dock falls back to the view of always, and `settle` writes
    // into a view the package does not have: the loss of E75, by another door.
    expect(layout.showNamed("Assente")).toBe(false);
    expect(layout.openNamed).toBeNull();
    expect(layout.showNamed("Revisione")).toBe(true);
    expect(layout.openNamed).toBe("Revisione");
  });

  it("saves the layout on screen under a new name at once and opens it, keeps those saved before, and refuses an empty or a taken name (D4)", () => {
```

In `gui/src/a11y.test.ts` (`replace_unique.py`), quattro sostituzioni — la barra ha una prop e un modello, e la
Panoramica e il cassetto aperti hanno la loro prova:

*Trova*:

```ts
import Band from "./frame/Band.vue";
import ViewBar from "./frame/ViewBar.vue";
```

*Sostituisci con:*

```ts
import Band from "./frame/Band.vue";
import Drawer from "./frame/Drawer.vue";
import ViewBar from "./frame/ViewBar.vue";
```

*Trova:*

```ts
import Strip from "./panels/Strip.vue";
import { useConnection } from "./stores/connection";
import { useCore } from "./stores/core";
import { useInvoke } from "./stores/invoke";
```

*Sostituisci con:*

```ts
import Strip from "./panels/Strip.vue";
import { VIEWS } from "./panels/views";
import { useConnection } from "./stores/connection";
import { useCore } from "./stores/core";
import { useDrawer } from "./stores/drawer";
import { useInvoke } from "./stores/invoke";
```

*Trova:*

```ts
async function mounted(component: Component): Promise<{ element: Element; unmount: () => void }> {
  const wrapper = mount(component, { global: { plugins: [i18n] }, attachTo: document.body });
```

*Sostituisci con:*

```ts
async function mounted(component: Component, props: Record<string, unknown> = {}): Promise<{ element: Element; unmount: () => void }> {
  const wrapper = mount(component, { global: { plugins: [i18n] }, attachTo: document.body, props });
```

*Trova:*

```ts
  const modules: [string, Component][] = [
    ["Stato", Status],
    ["Permessi", Permissions],
    ["Passi", Steps],
    ["Impostazioni", Settings],
    ["Chat", Chat],
    ["la striscia", Strip],
    ["la barra", ViewBar],
  ];

  for (const [name, component] of modules) {
    it(`${name} has no violation`, async () => {
      welcome();
      const { element, unmount } = await mounted(component);
      expect(await violations(element)).toEqual([]);
      unmount();
    });
  }

  it("the band, while waiting, has no violation", async () => {
```

*Sostituisci con:*

```ts
  // The bar holds the overview's trigger, which saves the layout on screen: a snapshot of Home stands for the dock.
  const bar = { snapshot: () => VIEWS.home, overview: false };
  const modules: [string, Component, Record<string, unknown>?][] = [
    ["Stato", Status],
    ["Permessi", Permissions],
    ["Passi", Steps],
    ["Impostazioni", Settings],
    ["Chat", Chat],
    ["la striscia", Strip],
    ["la barra", ViewBar, bar],
  ];

  for (const [name, component, props] of modules) {
    it(`${name} has no violation`, async () => {
      welcome();
      const { element, unmount } = await mounted(component, props);
      expect(await violations(element)).toEqual([]);
      unmount();
    });
  }

  it("the overview, open, has no violation", async () => {
    welcome();
    const { unmount } = await mounted(ViewBar, { ...bar, overview: true });
    expect(document.querySelector('.base-dialog[data-variant="full"]')).not.toBeNull();
    // The portal renders into `body`, so the whole document is the node under probe.
    expect(await violations(document.body)).toEqual([]);
    unmount();
  });

  it("the drawer, open, has no violation", async () => {
    useDrawer().open = true;
    const { unmount } = await mounted(Drawer);
    expect(document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
    expect(await violations(document.body)).toEqual([]);
    unmount();
  });

  it("the band, while waiting, has no violation", async () => {
```

In `gui/src/locales/copy.test.ts` (`replace_unique.py`), tre sostituzioni — la Panoramica costruisce
`views.${view}`, e `no-missing-keys` non vede una chiave costruita:

*Trova*:

```ts
import { PANEL_TYPES } from "../panels/registry";
import { THEME_CHOICES } from "../tokens/theme";
```

*Sostituisci con:*

```ts
import { PANEL_TYPES } from "../panels/registry";
import { VIEWS } from "../panels/views";
import { THEME_CHOICES } from "../tokens/theme";
```

*Trova:*

```ts
 * `settings.theme.${choice}` in `Settings.vue` -- and `no-missing-keys` is blind to a built key
 * -- measured on 2026-09-15, both directions in one file (P-105).
```

*Sostituisci con:*

```ts
 * `settings.theme.${choice}` in `Settings.vue` and `views.${view}` in the overview -- and
 * `no-missing-keys` is blind to a built key -- measured on 2026-09-15, both directions in one file
 * (P-105).
```

*Trova:*

```ts

  it("has a word for every theme choice", () => {
```

*Sostituisci con:*

```ts

  it("has a name for every view that ships (the overview and the bar build `views.${view}`)", () => {
    const names = (it_ as { views?: Record<string, string> }).views ?? {};
    // ⛔ NON-VACUITY: no views would leave nothing to check.
    expect(Object.keys(VIEWS).length).toBeGreaterThan(0);
    for (const view of Object.keys(VIEWS)) expect(Object.keys(names), view).toContain(view);
  });

  it("has a word for every theme choice", () => {
```

In `gui/src/frame/frame.browser.test.ts` (`replace_unique.py`), cinque sostituzioni — il file **c'è dal 6bis**, con la
prova della fascia e gli aiutanti `frame(theme)` e `px(token)`, e il compito lo **estende** (**D29**, **E63**): la cornice
intera nel Chrome installato, montata come la monta `main.ts` (P-23) — le due fasce sulla pagina e la pagina che non
sborda (**E70**, **E43**), il controllo **19**, il cassetto dalla striscia anche dalla tastiera (**E38**), la Panoramica
disegnata, le frecce (R3-23), la barra (**E57**, **E39**) e l'anello di un pannello che scorre (**E100**); `computed` esce, e
si prende da `testing/probes.ts` (**E92**). Gli import:

*Trova*:

```ts
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
```

*Sostituisci con:*

```ts
import { createPinia, setActivePinia } from "pinia";
import { userEvent } from "vitest/browser";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
```

*Trova:*

```ts
import { registerModules } from "../panels/modules";
import { concentricRadii } from "../testing/probes";
```

*Sostituisci con:*

```ts
import { registerModules } from "../panels/modules";
import { useConnection } from "../stores/connection";
import { useLayout } from "../stores/layout";
import { contrastJudged, violations } from "../testing/axe";
import { computed, concentricRadii, fits, iconsCentred } from "../testing/probes";
```

Il commento di testa dice che il compito lo estende, senza elenco (**E109**):

*Trova:*

```ts
// `main.ts` loads them. Born with task 6bis for the band on the page (E60); task 8 adds the strip and the overview.
```

*Sostituisci con:*

```ts
// `main.ts` loads them. Born with task 6bis for the band on the page (E60); task 8 extends it.
```

L'aiutante `computed` esce, e al suo posto le due porte della Panoramica:

*Trova:*

```ts
/** What a declaration of `property: var(token)` computes to here: the oracle of a token, with no copy of its value. */
function computed(property: string, token: string): string {
  const probe = document.createElement("div");
  probe.style.setProperty(property, `var(${token})`);
  document.body.append(probe);
  const value = getComputedStyle(probe).getPropertyValue(property);
  probe.remove();
  return value;
}
```

*Sostituisci con:*

```ts
const overview = (): HTMLElement | null => document.querySelector('.base-dialog[data-variant="full"]');
const cards = (): HTMLElement[] => [...document.querySelectorAll<HTMLElement>("[data-card]")];
```

La prova della fascia prende l'altra fascia e la pagina che non sborda, e dopo di lei le prove del compito:

*Trova:*

```ts
    it("lays the band on the page: under the bar, 12 px from the sides and from the cards, rounded like a card (control 23, E60)", async () => {
      await frame(theme);
      // With no core the connection waits (§6a): the band is there, a warning with «Riprova».
      const band = document.querySelector<HTMLElement>(".band");
      const bar = document.querySelector<HTMLElement>(".bar");
      expect(band).not.toBeNull();
      expect(bar).not.toBeNull();
      const box = (band as HTMLElement).getBoundingClientRect();
      // Aligned with the cards: the dock's own margin, `--space-3`, on each side.
      expect(box.left).toBeCloseTo(px("--space-3"), 1);
      expect(window.innerWidth - box.right).toBeCloseTo(px("--space-3"), 1);
      // Right under the bar, which leaves it its own 8 px: the band adds nothing above itself.
      expect(box.top).toBeCloseTo((bar as HTMLElement).getBoundingClientRect().bottom, 1);
      // `--space-3` above the highest card: the dock has none on top (answer 20).
      const cards = [...document.querySelectorAll(".dv-groupview")]
        .map((group) => group.getBoundingClientRect())
        .filter((card) => card.width > 0);
      expect(cards.length).toBeGreaterThan(0);
      expect(Math.min(...cards.map((card) => card.top)) - box.bottom).toBeCloseTo(px("--space-3"), 1);
      // Rounded like a card, and «Riprova» concentric in its corners at half a pixel (E61).
      expect(getComputedStyle(band as HTMLElement).borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
      const radii = concentricRadii([band as HTMLElement]);
      expect(radii.near).toBeGreaterThan(0);
      expect(radii.bad).toEqual([]);
    });
  });
}
```

*Sostituisci con:*

```ts
    it("lays both bands on the page -- under the bar, 12 px from the sides and from the cards, rounded like a card -- and the page never spills while one comes in (control 23, E60, E70, E43)", async () => {
      await frame(theme);
      const bar = document.querySelector<HTMLElement>(".bar");
      expect(bar).not.toBeNull();
      const onThePage = (band: HTMLElement): void => {
        const box = band.getBoundingClientRect();
        // Aligned with the cards: the dock's own margin, `--space-3`, on each side.
        expect(box.left).toBeCloseTo(px("--space-3"), 1);
        expect(window.innerWidth - box.right).toBeCloseTo(px("--space-3"), 1);
        // Right under the bar, which leaves it its own 8 px: the band adds nothing above itself.
        expect(box.top).toBeCloseTo((bar as HTMLElement).getBoundingClientRect().bottom, 1);
        // `--space-3` above the highest card: the dock has none on top (answer 20).
        const cards = [...document.querySelectorAll(".dv-groupview")]
          .map((group) => group.getBoundingClientRect())
          .filter((card) => card.width > 0);
        expect(cards.length).toBeGreaterThan(0);
        expect(Math.min(...cards.map((card) => card.top)) - box.bottom).toBeCloseTo(px("--space-3"), 1);
        // Rounded like a card.
        expect(getComputedStyle(band).borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
      };
      // With no core the connection waits (§6a): the band is there, a warning with «Riprova».
      const waiting = document.querySelector<HTMLElement>(".band");
      expect(waiting).not.toBeNull();
      onThePage(waiting as HTMLElement);
      // «Riprova» concentric in its corners at half a pixel (E61).
      const radii = concentricRadii([waiting as HTMLElement]);
      expect(radii.near).toBeGreaterThan(0);
      expect(radii.bad).toEqual([]);
      // ⛔ THE BAND THAT STOPS THE WINDOW IS ON THE PAGE TOO (E70), AND THE PAGE NEVER SPILLS WHILE IT COMES IN (E43): the
      // core welcomed, the band leaves; another stamp brings it back. `dockview` resizes one frame late, and for that
      // frame the grid spilled out of the dock and the page showed its scrollbars -- read on `scrollHeight`, which the
      // hidden scrollbars do not hide: 50 px on the first frame without the dock's `overflow: clip`, measured on
      // 2026-09-27.
      const connection = useConnection();
      connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
      await expect.poll(() => document.querySelector(".band")).toBeNull();
      const frames = async (count: number): Promise<number[]> => {
        const spill: number[] = [];
        for (let step = 0; step < count; step += 1) {
          await new Promise((resolve) => requestAnimationFrame(resolve));
          const page = document.documentElement;
          spill.push(page.scrollHeight - page.clientHeight + page.scrollWidth - page.clientWidth);
        }
        return spill;
      };
      await frames(6);
      connection.receive({ kind: "StaleBuild", value: "81985529216486895" });
      expect(await frames(8)).toEqual([0, 0, 0, 0, 0, 0, 0, 0]);
      const stop = document.querySelector<HTMLElement>(".band");
      expect(stop?.getAttribute("data-tone")).toBe("stop");
      onThePage(stop as HTMLElement);
    });

    it("floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)", async () => {
      await frame(theme);
      const strips = [...document.querySelectorAll(".dv-groupview")].filter((group) => group.querySelector(".strip") !== null);
      expect(strips).toHaveLength(1);
      const strip = strips[0] as Element;
      const box = strip.getBoundingClientRect();
      // Answer 20: aligned with the cards, `--space-3` from the sides, and `--space-6` from the bottom.
      expect(box.left).toBeCloseTo(px("--space-3"), 1);
      expect(window.innerWidth - box.right).toBeCloseTo(px("--space-3"), 1);
      expect(window.innerHeight - box.bottom).toBeCloseTo(px("--space-6"), 1);
      // A pill: the radius reaches half the height.
      expect(Number.parseFloat(getComputedStyle(strip).borderTopLeftRadius)).toBeGreaterThanOrEqual(box.height / 2);
      // ⛔ THE CORNERS ARE WINDOWS' (the (d)): no rounded box of ours comes within `--space-3` of a corner of the page on
      // both axes -- the square where the window's arc is drawn, 8 px in Windows 11.
      const reach = px("--space-3");
      let rounded = 0;
      const near: string[] = [];
      for (const element of document.body.querySelectorAll("*")) {
        if (element.closest("svg") !== null) continue;
        if (!(Number.parseFloat(getComputedStyle(element).borderTopLeftRadius) > 0)) continue;
        const b = element.getBoundingClientRect();
        if (b.width === 0 || b.height === 0) continue;
        rounded += 1;
        const gaps = [
          [b.left, b.top],
          [window.innerWidth - b.right, b.top],
          [b.left, window.innerHeight - b.bottom],
          [window.innerWidth - b.right, window.innerHeight - b.bottom],
        ];
        if (gaps.some(([dx, dy]) => (dx ?? 0) < reach && (dy ?? 0) < reach)) near.push(element.getAttribute("class") ?? element.tagName);
      }
      // ⛔ NON-VACUITY (trap 1 of the design): the cards, the strip and the controls are rounded.
      expect(rounded).toBeGreaterThan(0);
      expect(near).toEqual([]);
    });

    it("opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight", async () => {
      await frame(theme);
      const button = document.querySelector<HTMLElement>(".strip .base-button");
      expect(button).not.toBeNull();
      await userEvent.click(button as HTMLElement);
      await expect.poll(() => document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
      await userEvent.keyboard("{Escape}");
      await expect.poll(() => document.querySelector('.base-dialog[data-variant="sheet"]')).toBeNull();
      // ⛔ THE BUTTON IS IN ANOTHER VUE APP, AND NO `DialogTrigger`: the focus comes back all the same, measured on
      // 2026-09-24 with `reka-ui` 2.10.4 (P-22 of the plan) -- this probe is what keeps it true across an update.
      await expect.poll(() => document.activeElement).toBe(button);
      // ⛔ FROM THE KEYBOARD, THE FOCUS IN SIGHT (E38): `reka-ui` gives the first control the focus with `preventScroll`
      // -- "Chiudi", below a list longer than the sheet -- and the sheet keeps its actions stuck to its bottom edge.
      await userEvent.keyboard("{Enter}");
      await expect.poll(() => document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
      const sheet = document.querySelector('.base-dialog[data-variant="sheet"]') as HTMLElement;
      const focused = document.activeElement as HTMLElement;
      expect(sheet.contains(focused)).toBe(true);
      const seen = focused.getBoundingClientRect();
      expect(seen.top).toBeGreaterThanOrEqual(sheet.getBoundingClientRect().top);
      expect(seen.bottom).toBeLessThanOrEqual(Math.min(sheet.getBoundingClientRect().bottom, window.innerHeight));
    });

    it("draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation", async () => {
      await frame(theme);
      await userEvent.keyboard("{F3}");
      await expect.poll(overview).not.toBeNull();
      // ⛔ OFF, AND IT LOOKS OFF (E105): until the core answers «Salva questa vista» is off (E81), and its words and its icon
      // take the button's colour for off -- the colour `BaseLabel` writes on its own element won over it.
      const save = document.querySelector('[data-card="save"]') as HTMLButtonElement;
      const words = save.querySelector(".base-label") as Element;
      expect(save.disabled).toBe(true);
      expect(getComputedStyle(words).color).toBe(computed("color", "--color-text-disabled"));
      expect(getComputedStyle(save.querySelector(".base-icon") as Element).color).toBe(computed("color", "--color-text-disabled"));
      // ⛔ THE CORE HAS ANSWERED (E81): «Salva questa vista» is on, and its words are the label's own again.
      useLayout().receive({ kind: "Layout", value: { state: "Nothing" } });
      await expect.poll(() => save.disabled).toBe(false);
      expect(getComputedStyle(words).color).toBe(computed("color", "--color-text-muted"));
      // And with the name of a new view asked, the field and its two buttons in the place of «Salva questa vista».
      for (const naming of [false, true]) {
        if (naming) {
          await userEvent.click(document.querySelector('[data-card="save"]') as HTMLElement);
          await expect.poll(() => document.querySelector(".naming")).not.toBeNull();
        }
        const dialog = overview() as HTMLElement;
        const radii = concentricRadii([dialog]);
        // ⛔ NON-VACUITY (trap 1): the miniatures sit in the cards, the tiles in the miniatures.
        expect(radii.near).toBeGreaterThan(0);
        expect(radii.bad).toEqual([]);
        const fit = fits([dialog], ".base-dialog, .base-button, .mini, .naming, .base-text-field > .frame");
        expect(fit.seen).toBeGreaterThan(0);
        expect(fit.boxed).toBeGreaterThan(0);
        expect(fit.problems).toEqual([]);
        const icons = iconsCentred([dialog]);
        expect(icons.icons).toBeGreaterThan(0);
        expect(icons.centred).toBeGreaterThan(0);
        expect(icons.problems).toEqual([]);
        expect(await violations(dialog, { contrast: true })).toEqual([]);
        const judged = await contrastJudged(dialog);
        expect(judged.passes).toBeGreaterThan(0);
        expect(judged.incomplete).toBe(0);
      }
    });
  });
}

describe("the overview's grid, under the keys (R3-23 of the review)", () => {
  it("moves by geometry with the arrows, enters a view with Enter, and gives the focus back to the view's name", async () => {
    await frame("light");
    // ⛔ THE CORE HAS ANSWERED (E81): before it, «Salva questa vista» is off.
    useLayout().receive({ kind: "Layout", value: { state: "Nothing" } });
    await userEvent.keyboard("{F3}");
    await expect.poll(overview).not.toBeNull();
    // Three columns: Home, Lavoro, Compatta above, «Salva questa vista» alone below, under Home. The focus opens on the
    // view on screen.
    await expect.poll(() => document.activeElement).toBe(cards()[0]);
    await userEvent.keyboard("{ArrowRight}");
    expect(document.activeElement).toBe(cards()[1]);
    // ⛔ DOWN GOES TO THE ROW BELOW, NOT TO THE NEXT CARD: `RovingFocusGroup` would have gone right (decision 19).
    await userEvent.keyboard("{ArrowDown}");
    expect(document.activeElement).toBe(cards()[3]);
    // One card in the tab order, the one the arrows reached -- «Salva questa vista» too (D17).
    expect(cards().map((card) => card.tabIndex)).toEqual([-1, -1, -1, 0]);
    // ⛔ THE NAME OF A NEW VIEW (the (d)): Enter opens the field with the focus in it; there the arrows move the caret and
    // not the focus; «Annulla» closes it, and the focus is back on the card.
    await userEvent.keyboard("{Enter}");
    await expect.poll(() => document.activeElement?.matches(".naming input")).toBe(true);
    await userEvent.keyboard("{ArrowRight}");
    expect(document.activeElement?.matches(".naming input")).toBe(true);
    await userEvent.click(document.querySelector(".naming .base-button") as HTMLElement);
    await expect.poll(() => document.querySelector(".naming")).toBeNull();
    await expect.poll(() => document.activeElement?.getAttribute("data-card")).toBe("save");
    // Up again: three cards are as far, and the nearest centre wins -- Home, the column of the card below (R3-18).
    await userEvent.keyboard("{ArrowUp}");
    expect(document.activeElement).toBe(cards()[0]);
    // Nothing beyond: the focus stays.
    await userEvent.keyboard("{ArrowLeft}");
    expect(document.activeElement).toBe(cards()[0]);
    // One card in the tab order: the one the arrows reached.
    expect(cards().map((card) => card.tabIndex)).toEqual([0, -1, -1, -1]);
    await userEvent.keyboard("{ArrowRight}");
    await userEvent.keyboard("{Enter}");
    await expect.poll(() => useLayout().view).toBe("work");
    await expect.poll(overview).toBeNull();
    await expect.poll(() => document.activeElement?.classList.contains("view-name")).toBe(true);
    // ⛔ EVERY OPENING STARTS FROM THE VIEW ON SCREEN (D17), not from the card the arrows left: down to «Salva questa
    // vista», out with Esc, and F3 opens on Lavoro again.
    await userEvent.keyboard("{F3}");
    await expect.poll(overview).not.toBeNull();
    await expect.poll(() => document.activeElement).toBe(cards()[1]);
    await userEvent.keyboard("{ArrowDown}");
    expect(document.activeElement).toBe(cards()[3]);
    await userEvent.keyboard("{Escape}");
    await expect.poll(overview).toBeNull();
    await userEvent.keyboard("{F3}");
    await expect.poll(overview).not.toBeNull();
    await expect.poll(() => document.activeElement).toBe(cards()[1]);
  });
});

describe("the bar, and the focus in a panel that scrolls", () => {
  it("draws the bar as part of the page, and the search's whole placeholder in its field (E57, E39)", async () => {
    await frame("light");
    const bar = document.querySelector(".bar") as HTMLElement;
    // ⛔ NO SURFACE AND NO LINE UNDER IT (E57): the bar is part of the page, as the board's `.m-bar` is.
    expect(getComputedStyle(bar).backgroundColor).toBe("rgba(0, 0, 0, 0)");
    expect(getComputedStyle(bar).borderBottomStyle).toBe("none");
    // ⛔ THE BOX SAYS WHO FILLS IT (E39, decision 16 of the north star): the whole placeholder fits the field, measured
    // in the field's own font -- 315 px in 334 on 2026-09-27; in the 320 px of task 5 it was cut.
    const input = document.querySelector(".search input") as HTMLInputElement;
    const style = getComputedStyle(input);
    const pen = document.createElement("canvas").getContext("2d") as CanvasRenderingContext2D;
    pen.font = style.font;
    expect(input.placeholder.length).toBeGreaterThan(0);
    const room = input.clientWidth - Number.parseFloat(style.paddingLeft) - Number.parseFloat(style.paddingRight);
    expect(pen.measureText(input.placeholder).width).toBeLessThanOrEqual(room);
  });

  it("draws the focus ring of a panel that scrolls inside it, clear of its card, and its corners below follow the card's (E74)", async () => {
    await frame("light");
    // ⛔ CHROME PUTS A BOX THAT SCROLLS IN THE TAB ORDER, and in Home at 1440 x 900 Stato scrolls: the Tab reaches it.
    let panel: HTMLElement | null = null;
    for (let step = 0; step < 30 && panel === null; step += 1) {
      await userEvent.keyboard("{Tab}");
      const focused = document.activeElement;
      if (focused instanceof HTMLElement && focused.closest(".panel") !== null && focused.scrollHeight > focused.clientHeight) panel = focused;
    }
    // ⛔ NON-VACUITY: the Tab reached a panel that scrolls, and its ring is drawn.
    expect(panel).not.toBeNull();
    const box = panel as HTMLElement;
    const style = getComputedStyle(box);
    expect(style.outlineStyle).toBe("solid");
    const card = box.closest(".dv-groupview") as HTMLElement;
    const edge = Number.parseFloat(getComputedStyle(card).borderTopWidth);
    const inner = card.getBoundingClientRect();
    // How far the ring reaches out of the box -- its offset and its width; below zero it is inside. The card clips what
    // passes its border: on the three sides the panel touches, the ring stays within.
    const reach = Number.parseFloat(style.outlineOffset) + Number.parseFloat(style.outlineWidth);
    const ring = box.getBoundingClientRect();
    expect(ring.left - reach).toBeGreaterThanOrEqual(inner.left + edge);
    expect(ring.right + reach).toBeLessThanOrEqual(inner.right - edge);
    expect(ring.bottom + reach).toBeLessThanOrEqual(inner.bottom - edge);
    // ⛔ AND THE CORNERS BELOW ARE THE CARD'S LESS ITS BORDER (answer 4): a square ring in a round corner is cut there.
    const corner = Number.parseFloat(getComputedStyle(card).borderBottomLeftRadius) - edge;
    expect(Number.parseFloat(style.borderBottomLeftRadius)).toBeCloseTo(corner, 1);
    expect(Number.parseFloat(style.borderBottomRightRadius)).toBeCloseTo(corner, 1);
  });
});
```

- [ ] **Passo 3: lancia le prove, e guardale fallire**

```bash
(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts src/stores/stores.test.ts src/a11y.test.ts src/locales/copy.test.ts)
(cd gui && npx vitest run --project browser src/frame/frame.browser.test.ts src/frame/dock.browser.test.ts src/kit/kit.browser.test.ts)
```

Atteso: **rosso**, e per le ragioni giuste — misurato il 2026-09-24, e rifatto il 2026-09-27 dal pre-controllo (**E93**). Sotto jsdom `Failed to resolve import
"../stores/drawer"` in `frame.test.ts` e `Failed to resolve import "./stores/drawer"` in `a11y.test.ts`; `copy.test.ts`
**verde**, perché le tre viste hanno già le loro parole: la sua prova nuova è una guardia, e il suo rosso è al passo 9;
`stores.test.ts` rossa la sola prova di **E99**, `layout.showNamed is not a function`. Nel browser **dieci rosse su
quarantanove** (**E93**): il dock, `expected '21px' to be '9999px'` nei due temi — la striscia è ancora una scheda, e dal 6bis
la scheda è 21 —; la cornice, `expected 21 to be greater than or equal to 25` — il raggio di una scheda, non di una pillola —
e `expected null not to be null` per il pulsante della striscia e per la Panoramica, che F3 non apre, nei due temi; la prova
delle frecce, `expected null not to be null`; e l'anello del pannello che scorre, `expected 9 to be greater than or equal to
13` — disegnato fuori, e la scheda lo taglia (**E100**). Verdi le ventidue della pagina kit, le sedici del dock meno le due
della pillola, la prova delle fasce nei due temi — le cure di **E70** ed **E43** ci sono dal 6bis e dal compito 5 — e quella
della barra, che il compito 6 ha già curato (**E57**, **E39**). Le prove delle cure della revisione (**E103**…**E108**) non
cambiano i conti: quelle di `frame.test.ts` stanno nel file che non si carica, quelle di `frame.browser.test.ts` in prove
che cadono già — la Panoramica che F3 non apre —, e quella di `probes.browser.test.ts`, che questi comandi non nominano,
nasce verde: la guardia di `computed` arriva nello stesso Passo 2 (**E108**).

- [ ] **Passo 4: il cassetto da un negozio, e la domanda della conferma in un posto solo**

Crea `gui/src/stores/drawer.ts` (LF) — R3-20:

```ts
import { defineStore } from "pinia";
import { ref } from "vue";

/**
 * Whether the drawer of the modules is open (the (d) of the design system). ⛔ IN A STORE AND NOT IN A `ref` OF
 * `Drawer.vue` (R3-20 of the design-system review): the button that opens it sits in the strip, and the strip is a
 * panel -- a Vue app of its own (`frame/VueContent.ts`), which reaches the stores of the page and not the refs of
 * another app.
 */
export const useDrawer = defineStore("drawer", () => {
  const open = ref(false);
  return { open };
});
```

In `gui/src/stores/invoke.ts` (`replace_unique.py`), tre sostituzioni — la regola di D59 entra nel
negozio, perché ora la leggono in due (**D15**):

*Trova*:

```ts
import { ref } from "vue";
```

*Sostituisci con:*

```ts
import { computed, ref } from "vue";
```

*Trova:*

```ts
  let wire: Bridge | null = null;

```

*Sostituisci con:*

```ts
  let wire: Bridge | null = null;

  /**
   * ⛔ THE CORE ASKED, AND ABOUT A CALL OF OURS (D59): the one condition under which the confirmation window is open. A
   * `PermissionRequired` following no `Invoke` is a shape the core never produces -- the registry only ever answers one --
   * and a window that opened on it would offer a "yes" with nothing to send; the Permessi panel shows such a request.
   * ⛔ HERE AND NOT IN `Confirm.vue` since the design system: the frame reads it too, to keep F3 quiet while the question
   * is open -- two readers of one rule, one house (gotcha #68).
   */
  const asking = computed(() => core.pending !== null && inFlight.value !== null);

```

*Trova:*

```ts
  return { inFlight, approved, attach, send, approve, refuse, receive };
```

*Sostituisci con:*

```ts
  return { inFlight, approved, asking, attach, send, approve, refuse, receive };
```

In `gui/src/components/Confirm.vue` (`replace_unique.py`), due sostituzioni:

*Trova*:

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

```

*Sostituisci con:*

```vue
<script setup lang="ts">
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

import BaseButton from "./BaseButton.vue";
import BaseDialog from "./BaseDialog.vue";

// A COMPOSED piece, and that is why it may read two stores (P-3 of the design-system plan): the base pieces below do not.
// ⛔ OPEN ONLY WHILE `invoke.asking` (D59): the core asked, and about a call of ours -- the rule lives in the store.
const core = useCore();
const invoke = useInvoke();

```

*Trova:*

```vue
    :open="open"
```

*Sostituisci con:*

```vue
    :open="invoke.asking"
```

Riscrivi `gui/src/frame/Drawer.vue` per intero — il pulsante esce, e la finestra si apre dal negozio:

```vue
<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseList from "../components/BaseList.vue";
import { PANEL_TYPES } from "../panels/registry";
import { useDrawer } from "../stores/drawer";

// ⛔ THE DRAWER IS WHERE "WHO FILLS WHAT" LIVES (decision 16 of the north star): every type with its number, so the strip
// can stay thin. On `BaseDialog` since the design system -- its second occurrence, with the confirmation window: the two
// veils written by hand, already different, are one role now, `--color-veil`.
// ⛔ OPENED FROM THE STRIP, THROUGH A STORE (the (d); R3-20 of the review): the button lives in another Vue app.
const drawer = useDrawer();
</script>

<template>
  <BaseDialog v-model:open="drawer.open" :title="$t('drawer.title')" variant="sheet">
    <BaseList :items="PANEL_TYPES" :key-of="(type) => type.name">
      <template #item="{ item }">
        <span>{{ $t(`modules.${item.module}`) }}</span>
        <span class="who">{{ $t("drawer.who", { number: item.who }) }}</span>
      </template>
    </BaseList>
    <template #actions>
      <BaseButton variant="quiet" @click="drawer.open = false">{{ $t("drawer.close") }}</BaseButton>
    </template>
  </BaseDialog>
</template>

<style scoped>
.who {
  color: var(--color-text-muted);
}
</style>
```

- [ ] **Passo 5: le parole, e la Panoramica**

In `gui/src/locales/it.json` (`replace_unique.py`), due sostituzioni — `bar.views` esce, perché la sola
`nav` che lo leggeva esce con la barra vecchia; il pulsante del cassetto dice *«moduli»*, com'è nella tavola; le parole
della Panoramica:

*Trova*:

```json
  "bar": {
    "views": "Viste",
    "search": "Cerca",
```

*Sostituisci con:*

```json
  "bar": {
    "search": "Cerca",
```

*Trova:*

```json
    "open": "+ moduli",
    "title": "I moduli",
    "close": "Chiudi",
    "who": "arriva col sotto-progetto {number}"
  },
```

*Sostituisci con:*

```json
    "open": "moduli",
    "title": "I moduli",
    "close": "Chiudi",
    "who": "arriva col sotto-progetto {number}"
  },
  "overview": {
    "title": "Le viste",
    "hint": "F3 apre e chiude. Le frecce scelgono una vista, Invio la apre, Esc chiude.",
    "saved": "salvata",
    "save": "Salva questa vista",
    "name": "Nome della vista",
    "confirm": "Salva",
    "cancel": "Annulla",
    "empty": "Scrivi un nome.",
    "taken": "C'è già una vista con questo nome."
  },
```

In `gui/src/stores/layout.ts` (`replace_unique.py`), due sostituzioni — **E99**: `showNamed`, la gemella di `showView`,
che rifiuta un nome che la lista non tiene; la Panoramica apre una vista col nome solo da lì:

*Trova*:

```ts
  /**
   * The layout on screen saved under a name, and opened ("Salva questa vista", the (d)).
```

*Sostituisci con:*

```ts
  /**
   * A named view shown (the (d)). ⛔ ONLY BY A NAME THE LIST HOLDS (E90 of the design-system plan): with an unknown name
   * the dock falls back to the view of always, and `settle` writes the next move into a view the package does not have
   * -- the loss of E75, by another door. Refused, and nothing changes: the twin of `showView`.
   */
  function showNamed(name: string): boolean {
    if (!(saved.value?.named ?? []).some((entry) => entry.name === name)) return false;
    openNamed.value = name;
    return true;
  }

  /**
   * The layout on screen saved under a name, and opened ("Salva questa vista", the (d)).
```

*Trova:*

```ts
attach, receive, settle, showView, saveNamed, chooseTheme };
```

*Sostituisci con:*

```ts
attach, receive, settle, showView, showNamed, saveNamed, chooseTheme };
```

Crea `gui/src/frame/Overview.vue` (LF) — la (d), le decisioni 18 e 19, **D4**, **D12**, **D14**, **D17** e **D18**:

```vue
<script setup lang="ts">
import type { SerializedDockview } from "dockview-core";
import { computed, nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseIcon from "../components/BaseIcon.vue";
import BaseLabel from "../components/BaseLabel.vue";
import BaseTextField from "../components/BaseTextField.vue";
import { isIconName, type IconName } from "../components/icons";
import { VIEWS } from "../panels/views";
import { useLayout, type ViewName } from "../stores/layout";

import { nearest, type Direction } from "./nearest";
import { schematic } from "./schematic";

/**
 * THE OVERVIEW OF THE VIEWS (the (d) of the design system, answer 13): every view in miniature, in a grid -- Home, Lavoro,
 * Compatta and the views saved under a name -- the one on screen in bordeaux, and last «Salva questa vista». It opens from
 * the view's name in the bar, which this file draws as its trigger, or with F3 (`Frame.vue`).
 *
 * ⛔ ON `BaseDialog`, THE WHOLE WINDOW (decision 18): `reka-ui` gives Esc, the focus kept inside and given back to the
 * view's name. ⛔ THE ARROWS FOLLOW THE GEOMETRY (decision 19): `nearest`, the helper of the tiles moved with the keyboard,
 * in the four directions -- `RovingFocusGroup` is linear, and "down" would go right. ⛔ ONE CARD IN THE TAB ORDER, the one
 * the arrows reached (a roving tabindex): Tab leaves the grid instead of walking every card.
 */
const props = defineProps<{ snapshot: () => SerializedDockview }>();
const open = defineModel<boolean>("open", { required: true });

const { t } = useI18n();
const layout = useLayout();
const THREE: readonly ViewName[] = ["home", "work", "compact"];

/** A tile of a miniature: where it sits, in percent of the miniature, and the icon of the module it shows. */
interface Drawn {
  style: Record<string, string>;
  icon?: IconName;
}

interface Card {
  key: string;
  name: string;
  tiles: Drawn[];
  saved: boolean;
  current: boolean;
  show: () => void;
}

/** The miniature of a layout (answer 19): the schema of `schematic`, with a gap of `--space-0-5` around every tile.
 * ⛔ THE STRIP IS NOT DRAWN (D14 of the plan): it is in every view, the same, with no icon -- and the board's miniatures
 * leave it out. A module type that is gone is drawn without an icon: it is there, and the miniature says so.
 * ⛔ A LAYOUT IT CANNOT READ IS DRAWN EMPTY (E90 of the plan): `unpack` keeps any object as a layout, and `schematic`
 * throws on one it cannot read -- the dock only when that view is shown, while the cards draw every view at once, and one
 * would take the whole overview down. */
function drawn(saved: SerializedDockview): Drawn[] {
  let tiles: ReturnType<typeof schematic>;
  try {
    tiles = schematic(saved);
  } catch {
    return [];
  }
  return tiles
    .filter((tile) => !tile.views.includes("strip"))
    .map((tile) => {
      const style = {
        left: `calc(${tile.x * 100}% + var(--space-0-5))`,
        top: `calc(${tile.y * 100}% + var(--space-0-5))`,
        width: `calc(${tile.width * 100}% - 2 * var(--space-0-5))`,
        height: `calc(${tile.height * 100}% - 2 * var(--space-0-5))`,
      };
      const shown = tile.active ?? tile.views[0] ?? "";
      return isIconName(shown) ? { style, icon: shown } : { style };
    });
}

// ⛔ A SAVED VIEW WINS OVER THE SHIPPED ONE BY NAME (row 6 of §2 of the north star), in the miniature as on screen.
const cards = computed<Card[]>(() => [
  ...THREE.map((view) => ({
    key: view,
    name: t(`views.${view}`),
    tiles: drawn(layout.saved?.layouts[view] ?? VIEWS[view]),
    saved: false,
    current: layout.openNamed === null && layout.view === view,
    // ⛔ ONE LINE, AND THE DOCK FOLLOWS (D89 of part 2): the store is where the open view lives, and `createDock` watches it.
    // Showing is not saving (decision 11).
    show: () => layout.showView(view),
  })),
  ...(layout.saved?.named ?? []).map((entry) => ({
    key: `named:${entry.name}`,
    name: entry.name,
    tiles: drawn(entry.layout),
    saved: true,
    current: layout.openNamed === entry.name,
    show: () => {
      layout.showNamed(entry.name);
    },
  })),
]);

/** The name of the view on screen: the owner's words for a named view, the locale's for the three. */
const current = computed(() => layout.openNamed ?? t(`views.${layout.view}`));

const grid = ref<HTMLElement | null>(null);
const roving = ref(0);
const naming = ref(false);
const name = ref("");
const refusal = ref<"empty" | "taken" | null>(null);

// Every opening starts from the view on screen, with no name half written.
watch(open, (now) => {
  if (!now) return;
  naming.value = false;
  name.value = "";
  refusal.value = null;
  roving.value = Math.max(0, cards.value.findIndex((card) => card.current));
});

function choose(card: Card): void {
  card.show();
  open.value = false;
}

const ARROWS: Readonly<Record<string, Direction>> = { ArrowLeft: "left", ArrowRight: "right", ArrowUp: "up", ArrowDown: "down" };

/** ⛔ ONLY FROM A CARD: in the name's field the arrows move the caret. Nothing beyond, nothing moves. */
function onArrow(event: KeyboardEvent): void {
  const direction = ARROWS[event.key];
  const from = event.target;
  if (direction === undefined || !(from instanceof HTMLElement) || !from.hasAttribute("data-card")) return;
  event.preventDefault();
  const others = [...(grid.value?.querySelectorAll<HTMLElement>("[data-card]") ?? [])].filter((card) => card !== from);
  const target = nearest(from.getBoundingClientRect(), others.map((element) => ({ element, rect: element.getBoundingClientRect() })), direction);
  target?.element.focus();
}

function startNaming(): void {
  naming.value = true;
  refusal.value = null;
  void nextTick(() => grid.value?.querySelector<HTMLInputElement>(".naming input")?.focus());
}

function cancel(): void {
  naming.value = false;
  refusal.value = null;
  roving.value = cards.value.length;
  void nextTick(() => grid.value?.querySelector<HTMLElement>('[data-card="save"]')?.focus());
}

/** «Salva questa vista»: the layout on screen under a name, and opened. ⛔ THE NAMES OF THE THREE ARE THE FRAME'S WORDS
 * (D12 of the plan): the store reads no locale, so it is handed them. ⛔ OFF UNTIL THE CORE HAS ANSWERED (E81): before
 * it the store holds no package of the core's, and a view saved then would send one without the core's layouts -- every
 * saved layout lost in silence, the window E40 closed for the theme. */
function save(): void {
  const result = layout.saveNamed(name.value, props.snapshot(), THREE.map((view) => t(`views.${view}`)));
  if (result === "saved") {
    open.value = false;
    return;
  }
  refusal.value = result;
}
</script>

<template>
  <BaseDialog v-model:open="open" :title="$t('overview.title')" :description="$t('overview.hint')" variant="full">
    <template #trigger>
      <BaseButton class="view-name" aria-keyshortcuts="F3">
        <BaseLabel icon="views">{{ current }}</BaseLabel>
      </BaseButton>
    </template>
    <div ref="grid" class="grid" @keydown="onArrow">
      <BaseButton
        v-for="(card, index) in cards"
        :key="card.key"
        variant="card"
        data-card="view"
        :tabindex="index === roving ? 0 : -1"
        :aria-current="card.current ? 'page' : undefined"
        @focus="roving = index"
        @click="choose(card)"
      >
        <span class="mini">
          <span v-for="(tile, at) in card.tiles" :key="at" class="tile" :style="tile.style">
            <BaseIcon v-if="tile.icon !== undefined" :name="tile.icon" size="sm" />
          </span>
        </span>
        <span class="caption">
          <BaseLabel>{{ card.name }}</BaseLabel>
          <span v-if="card.saved" class="saved">{{ $t("overview.saved") }}</span>
        </span>
      </BaseButton>
      <form v-if="naming" class="naming" @submit.prevent="save">
        <BaseTextField
          v-model="name"
          icon="saveView"
          :label="$t('overview.name')"
          :placeholder="$t('overview.name')"
          :error="refusal === 'empty' ? $t('overview.empty') : refusal === 'taken' ? $t('overview.taken') : undefined"
        />
        <span class="actions">
          <BaseButton variant="quiet" @click="cancel">{{ $t("overview.cancel") }}</BaseButton>
          <BaseButton variant="primary" @click="save">{{ $t("overview.confirm") }}</BaseButton>
        </span>
      </form>
      <BaseButton
        v-else
        variant="card"
        data-card="save"
        :disabled="layout.arrivals === 0"
        :tabindex="roving === cards.length ? 0 : -1"
        @focus="roving = cards.length"
        @click="startNaming"
      >
        <BaseLabel icon="saveView">{{ $t("overview.save") }}</BaseLabel>
      </BaseButton>
    </div>
  </BaseDialog>
</template>

<style scoped>
/* The view's name in the bar: the words in full, the icon in the mark's colour (the board's `.vt`). */
.view-name :deep(.base-label) {
  color: var(--color-text);
}
.grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--space-3);
  margin-top: var(--space-4);
}
/* ⛔ THE RADII ARE CONCENTRIC (answer 4): the miniature sits in a card of `--radius-card` at `--space-3` from its edge, so it
   takes `--radius-control`; a tile sits `--space-0-5` inside the miniature's border, so it takes what is left. */
.mini {
  position: relative;
  display: block;
  aspect-ratio: 16 / 10;
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-bg);
}
.tile {
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--radius-control) - var(--space-0-5) - var(--border-width));
  background: var(--color-bg-surface);
  color: var(--color-text-muted);
}
.caption {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--space-2);
}
.saved {
  font: var(--font-caption);
  color: var(--color-text-muted);
}
/* The name of a new view, where «Salva questa vista» was: a card that is not a button. */
.naming {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-3);
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-card);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
}
</style>
```

In `gui/src/components/BaseButton.vue` (`replace_unique.py`), una sostituzione — **E105**: nel kit, non nella Panoramica,
perché vale per ogni `BaseLabel` e `BaseIcon` dentro un `BaseButton` spento — le parole e l'icona prendono il colore dello
spento del pulsante:

*Trova*:

```css
.base-button:disabled {
  color: var(--color-text-disabled);
  cursor: default;
}
```

*Sostituisci con:*

```css
.base-button:disabled {
  color: var(--color-text-disabled);
  cursor: default;
}
/* ⛔ OFF INSIDE TOO (E105 of the plan): a `BaseLabel` writes its own colour on its element, and the mark's on its icon, so
   a button that was off still showed its words and icon lit -- they take the button's colour for off (the kind of E19). */
.base-button:disabled :deep(.base-label),
.base-button:disabled :deep(.base-icon) {
  color: inherit;
}
```

In `gui/src/panels/views/generate-views.test.ts` (`replace_unique.py`), una sostituzione — **E103**: il generatore scrive
ogni vista dopo che `dockview` l'ha disposta, cioè coi vincoli dei pannelli applicati:

*Trova*:

```ts
function write(name: string, api: DockviewApi): void {
  mkdirSync(OUT, { recursive: true });
```

*Sostituisci con:*

```ts
function write(name: string, api: DockviewApi): void {
  // ⛔ LAID OUT AGAIN BEFORE IT IS WRITTEN (E103 of the design-system plan): `dockview` applies a panel's limits -- the
  // strip's 56 px -- when it lays the grid out, not when the panel is added. Written at once, Compatta kept the strip at
  // half the grid, and its miniature drew the knowledge base in half the square, while the screen gave it all but 56 px.
  api.layout(api.width, api.height, true);
  mkdirSync(OUT, { recursive: true });
```

Riscrivi `gui/src/panels/views/compact.json` per intero — **E103**: è ciò che il generatore curato scrive, la knowledge
base a 944 e la striscia a 56, e `home.json` e `work.json` li riscrive uguali byte per byte, misurato il 2026-09-27 col
comando `(cd gui && REGENERATE_VIEWS=1 npx vitest run src/panels/views/generate-views.test.ts)`:

```json
{
  "grid": {
    "root": {
      "type": "branch",
      "data": [
        {
          "type": "branch",
          "data": [
            {
              "type": "leaf",
              "data": {
                "views": [
                  "knowledge"
                ],
                "activeView": "knowledge",
                "id": "1",
                "locked": true,
                "hideHeader": true
              },
              "size": 944
            },
            {
              "type": "leaf",
              "data": {
                "views": [
                  "strip"
                ],
                "activeView": "strip",
                "id": "2",
                "locked": true,
                "hideHeader": true
              },
              "size": 56
            }
          ],
          "size": 1600
        }
      ],
      "size": 1000
    },
    "width": 1600,
    "height": 1000,
    "orientation": "HORIZONTAL"
  },
  "panels": {
    "knowledge": {
      "id": "knowledge",
      "contentComponent": "knowledge",
      "params": {
        "module": "knowledge",
        "who": 6
      },
      "title": "knowledge"
    },
    "strip": {
      "id": "strip",
      "contentComponent": "strip",
      "title": "strip",
      "minimumHeight": 56,
      "maximumHeight": 56
    }
  },
  "activeGroup": "2"
}
```

- [ ] **Passo 6: la barra, e la cornice**

Riscrivi `gui/src/frame/ViewBar.vue` per intero — la forma **di passaggio** del compito 5 finisce qui:

```vue
<script setup lang="ts">
import type { SerializedDockview } from "dockview-core";

import BaseTextField from "../components/BaseTextField.vue";
import { useConnection } from "../stores/connection";

import Overview from "./Overview.vue";

// THE BAR, FROM THE LEFT (the (d) of the design system): the name of the view on screen, which opens the overview; the
// search; the core's chip. The three buttons in a row -- the tabs the owner did not want, on 2026-09-07 -- are gone, and so
// is the drawer's button: "moduli" came down into the strip.
defineProps<{ snapshot: () => SerializedDockview }>();
const overview = defineModel<boolean>("overview", { required: true });
const connection = useConnection();
</script>

<template>
  <header class="bar">
    <Overview v-model:open="overview" :snapshot="snapshot" />

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
  </header>
</template>

<style scoped>
.bar {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-2) var(--space-3);
  /* ⛔ NO SURFACE AND NO LINE UNDER IT (the owner at step 8 of task 6, E57): the bar is part of the page, as the board's
     `.m-bar` is. */
}
/* The search and the chip on the right, as on the board. */
.search {
  flex: 1;
  max-width: 24rem;
  margin-left: auto;
}
.chip[data-phase="connected"] {
  color: var(--color-text-accent);
}
.chip[data-phase="stale"] {
  color: var(--color-text-stop);
}
</style>
```

Riscrivi `gui/src/frame/Frame.vue` per intero — F3, il cassetto montato qui, la fotografia del dock per *«Salva questa
vista»* (**D16**):

```vue
<script setup lang="ts">
import type { SerializedDockview } from "dockview-core";
import { onMounted, onUnmounted, ref } from "vue";

import Confirm from "../components/Confirm.vue";
import { useDrawer } from "../stores/drawer";
import { useInvoke } from "../stores/invoke";

import Band from "./Band.vue";
import Drawer from "./Drawer.vue";
import ViewBar from "./ViewBar.vue";
import { createDock } from "./dock";
import { directionOf, moveActive } from "./moveActive";

const host = ref<HTMLElement | null>(null);
const drawer = useDrawer();
const invoke = useInvoke();
/** The overview of the views: opened from the view's name in the bar, or with F3 here. */
const overview = ref(false);
let api: ReturnType<typeof createDock> | null = null;

function onKey(event: KeyboardEvent): void {
  // ⛔ F3 OPENS AND CLOSES THE OVERVIEW (the (d) of the design system), AND IS QUIET WHILE ANOTHER WINDOW IS OPEN -- the
  // confirmation or the drawer: a second modal window over the first would hide its question under the views. F3 is free
  // in `gui/src` (P-11 of the plan); its default, the browser's "find next", is not ours to keep.
  if (event.key === "F3") {
    event.preventDefault();
    if (!invoke.asking && !drawer.open) overview.value = !overview.value;
    return;
  }
  // G20, move 6 of SP-8: the active tile moves in the four directions from the keyboard. The
  // mapping lives in `moveActive.ts` and the geometry in `nearest.ts`; this is only the wire.
  const direction = directionOf(event);
  if (direction === null || api === null) return;
  event.preventDefault();
  moveActive(api, direction);
}

onMounted(() => {
  if (host.value !== null) api = createDock(host.value);
  window.addEventListener("keydown", onKey);
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKey);
});

/** The layout on screen, for «Salva questa vista»: the dock's own serialisation -- what `settle` saves. */
function snapshot(): SerializedDockview {
  if (api === null) throw new Error("the dock is not mounted");
  return api.toJSON();
}
</script>

<template>
  <div class="frame">
    <ViewBar v-model:overview="overview" :snapshot="snapshot" />
    <Band />
    <Confirm />
    <Drawer />
    <div ref="host" class="dock"></div>
  </div>
</template>

<style scoped>
.frame {
  display: flex;
  flex-direction: column;
  height: 100%;
}
/* Answer 20 of the design system: the dock -- the strip is its last row -- 12 px from the sides and 24 from the bottom,
   away from the window's corners; the bar above has none. A MARGIN and not a padding, so `clientWidth` is the room the
   grid gets (P-7 of its plan). */
.dock {
  flex: 1;
  min-height: 0;
  margin: 0 var(--space-3) var(--space-6);
  /* ⛔ THE DOCK KEEPS ITS SPILL TO ITSELF (E43): `dockview` 8.3.1 resizes one frame late -- its ResizeObserver hands the
     new size to a `requestAnimationFrame` -- so for a frame after the band comes in, the grid is as tall as before and
     spills out of this box; unclipped, the spill reached the page and flashed both its scrollbars. `clip` and not
     `hidden`: nothing may scroll this box either. */
  overflow: clip;
}
</style>
```

In `gui/src/frame/dock.ts` (`replace_unique.py`), due sostituzioni — **E104**, di prima del compito 8, che la Panoramica
rende visibile: ingrandire e ripristinare un gruppo sono mosse, e `dockview-core` 8.3.1 non le annuncia con
`onDidLayoutChange`. Il corpo del `settle` diventa una funzione, che ascoltano `onDidLayoutChange`, la chiusura della
finestra e `onDidMaximizedGroupChange` — questo **differito a un microtask**, come `dockview` differisce
`onDidLayoutChange`, perché `fromJSON` lo lancia dentro `apply`, prima che `show` rimetta la base, e mostrare resti non
salvare:

*Trova*:

```ts
 * ⛔ SHOWING RESETS THE BASELINE: the buffered `onDidLayoutChange` that follows a `fromJSON`
 * compares equal and does not settle, so LOOKING at a view is not SAVING it -- decision 11, the
 * shipped views stay in `gui/` until the owner changes one.
```

*Sostituisci con:*

```ts
 * ⛔ SHOWING RESETS THE BASELINE: the buffered `onDidLayoutChange` that follows a `fromJSON`, and
 * a maximize deferred like it (E104), compare equal and do not settle, so LOOKING at a view is not
 * SAVING it -- decision 11, the shipped views stay in `gui/` until the owner changes one.
```

*Trova:*

```ts
  api.onDidLayoutChange(() => {
    const now = api.toJSON();
    if (same(last, now)) return;
    last = now;
    layout.settle(now);
  });

  window.addEventListener("beforeunload", () => {
    // Decision 12: and when the window closes. ⛔ ONLY IF SOMETHING CHANGED SINCE THE LAST SETTLE
    // (D81): before, an untouched gui copied the shipped Home into the archive at its first close.
    const now = api.toJSON();
    if (same(last, now)) return;
    last = now;
    layout.settle(now);
  });
```

*Sostituisci con:*

```ts
  /** What is on screen, saved only if it moved since the last save or the last showing. */
  function settleIfMoved(): void {
    const now = api.toJSON();
    if (same(last, now)) return;
    last = now;
    layout.settle(now);
  }

  api.onDidLayoutChange(settleIfMoved);

  // ⛔ A MAXIMIZE IS A MOVE, AND NOT AN `onDidLayoutChange` (E104 of the design-system plan): in `dockview-core` 8.3.1
  // `maximizeGroup` saved only through the change of the active group it makes, when it makes one, and
  // `exitMaximizedGroup` never -- a group restored stayed maximized in the package. ⛔ DEFERRED TO A MICROTASK, AS
  // `dockview` DEFERS `onDidLayoutChange`, SO THAT SHOWING STAYS NOT SAVING: `fromJSON` fires this event inside `apply` --
  // restoring the group of the view it leaves, maximizing the one of the view it opens -- before `show` resets the
  // baseline; heard at once, it saved the view left under the name of the view opened (measured on 2026-09-27).
  api.onDidMaximizedGroupChange(() => queueMicrotask(settleIfMoved));

  // Decision 12: and when the window closes. ⛔ ONLY IF SOMETHING CHANGED SINCE THE LAST SETTLE
  // (D81): before, an untouched gui copied the shipped Home into the archive at its first close.
  window.addEventListener("beforeunload", settleIfMoved);
```

- [ ] **Passo 7: la striscia a pillola**

Riscrivi `gui/src/panels/Strip.vue` per intero — **P-24**:

```vue
<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import { useCore } from "../stores/core";
import { useDrawer } from "../stores/drawer";

// ⚠️ ONLY WHAT IS ALIVE IN SUB-PROJECT 2 (decision 16 of the north star): degradation and
// permissions. Seven "arrives with N" in a thin strip is noise, and the drawer is where "who
// fills what" belongs.
// ⛔ AND THE DRAWER'S BUTTON, "moduli", came down from the bar (the (d) of the design system): the strip is a panel, a Vue
// app of its own, so it opens the drawer through the store and not through a ref (R3-20 of the review).
const core = useCore();
const drawer = useDrawer();
</script>

<template>
  <div class="strip">
    <span>
      {{ $t("strip.degradation") }}:
      <template v-if="core.degradation === null">—</template>
      <template v-else-if="core.degradation.vram_exhausted || core.degradation.routing_degraded">
        <span v-if="core.degradation.vram_exhausted" class="warn">{{ $t("strip.vram") }}</span>
        <span v-if="core.degradation.routing_degraded" class="warn">{{ $t("strip.routing") }}</span>
      </template>
      <template v-else>{{ $t("strip.none") }}</template>
    </span>
    <span>
      {{ $t("strip.permissions") }}:
      {{ core.pending === null ? $t("strip.quiet") : $t("strip.pending") }}
    </span>
    <BaseButton class="modules" size="lg" pill icon="modules" @click="drawer.open = true">{{ $t("drawer.open") }}</BaseButton>
  </div>
</template>

<style scoped>
/* ⛔ THE PILL'S RULE (answer 4 of the design system): inside a pill goes a pill, the same distance from its edge all round.
   The shipped views give the strip's row 56 px and the dock's gap takes half of `--space-3` above it, so the pill is 50
   high, measured on 2026-09-24 (P-24 of the plan): the large button, 40, sits `--space-1` plus the group's border from the
   pill's edge -- above and below by this padding, on the right by the same. */
.strip {
  display: flex;
  gap: var(--space-4);
  align-items: center;
  height: 100%;
  box-sizing: border-box;
  padding: var(--space-1) var(--space-1) var(--space-1) var(--space-4);
  color: var(--color-text-muted);
}
.warn {
  color: var(--color-text-warn);
  margin-left: var(--space-1);
}
.modules {
  margin-left: auto;
}
</style>
```

In `gui/src/tokens/dock.css` (`replace_unique.py`), una sostituzione — la pillola, e l'anello del fuoco di un pannello che scorre, disegnato dentro (**E100**, A del proprietario del 2026-09-27):

*Trova*:

```css

/* The mark of the visible tab (the (c)): the icon of its label in `--color-mark`, the hidden tabs' in the tab's own colour.
```

*Sostituisci con:*

```css

/* ⛔ THE STRIP IS A PILL, NOT A CARD (the (d) of the design system, answer 20): its group -- the last row of every view,
   12 px from the sides and 24 from the bottom -- takes the full radius, and the pill button inside it keeps the rule of
   answer 4. Found by what it holds, because a group carries no name of its own: `.strip` is the root of `Strip.vue`. */
.dockview-theme-harness .dv-groupview:has(.strip) {
  border-radius: var(--radius-full);
}

/* ⛔ THE RING OF A PANEL THAT SCROLLS IS DRAWN INSIDE IT (E74, the owner's A of 2026-09-27): Chrome puts a box that scrolls
   in the Tab order, and a panel's root fills its card, 1 px from the edge -- the card clips, and the ring drawn outside
   kept only its top line. Inside, in the padding every panel has: the ring of `base.css` mirrored, its gap and its width
   within the edge; and the corners below are the card's less its border (answer 4), or a square ring is cut there. */
.dockview-theme-harness .panel > :focus-visible {
  outline-offset: calc(-1 * (var(--focus-width) + var(--focus-offset)));
  border-bottom-left-radius: calc(var(--radius-card) - var(--border-width));
  border-bottom-right-radius: calc(var(--radius-card) - var(--border-width));
}

/* The mark of the visible tab (the (c)): the icon of its label in `--color-mark`, the hidden tabs' in the tab's own colour.
```

```bash
(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts src/stores/stores.test.ts src/a11y.test.ts src/locales/copy.test.ts)
(cd gui && npx vitest run --project browser src/frame/frame.browser.test.ts src/frame/dock.browser.test.ts src/kit/kit.browser.test.ts)
```

Atteso: **verde** — sotto jsdom `Tests  62 passed (62)` nei quattro file: le otto prove nuove della cornice, le due della
Panoramica e del cassetto aperti, quella delle parole delle viste, quella di `showNamed` e quella dell'ingrandimento nel
dock (**E104**), con le altre; nel browser `Tests  49 passed (49)` — misurati il 2026-09-27 (**E93**, **E102**).

- [ ] **Passo 8: tutte le prove, il *build*, il linter, e il pezzo JavaScript**

```bash
(cd gui && npm test && npm run build && npm run lint)
(cd gui && npm run build 2>&1 | grep -E 'assets/index-.*\.js ')
```

Atteso: **verde** su tutto; i file di prova **gli stessi** del compito 7 — `frame.browser.test.ts` c'è dal 6bis, e il compito lo estende (**D29**, **E64**) —, e le
prove più alte di **ventitré** — tredici sotto jsdom e dieci nel browser: `Tests  236 passed | 1 skipped (237)`, misurato il
2026-09-27 (**E93**; le due in più sono la prova dell'ingrandimento, **E104**, e quella dell'oracolo, **E108**). ⛔ E la suite intera **cinque volte**, ciascuna col suo rapporto JSON — `npx vitest run
--reporter=json --outputFile=<scratchpad>/corsa-N.json` —: una caduta è una voce d'errata, non una corsa da ripetere
finché passa (P-19, P-20, P-21). Il pezzo JavaScript va nel messaggio del commit, accanto alla baseline del Passo 1: sulla copia del
pre-controllo, il 2026-09-27, `dist/assets/index-….js` a `698.14 kB`, compresso `212.98 kB`, e con le cure della revisione
(**E103**…**E108**) a `698.20 kB`, compresso `213.00 kB`, contro `693.02 kB` e `211.20 kB` (**E80**, **E91**) — la cifra è per il proprietario, che ha N-2 (R3-25).

- [ ] **Passo 9: le due direzioni**

Una violazione alla volta, poi indietro con la **copia salvata** e `cmp` (vincolo 11): i file nuovi sono nati qui, e gli
altri il compito li ha già cambiati (A-1).

| La prova | La violazione messa a mano | Atteso, misurato il 2026-09-24 e rifatto il 2026-09-27 sulla suite intera (**E95**) |
|---|---|---|
| `frame.test.ts`, la carta corrente | in `Overview.vue` `current: false,` per le tre viste | rosso: `expected [ null, null, null, null ] to deeply equal [ 'page', null, null, null ]` |
| `frame.test.ts`, F3 sotto la conferma (D15) | in `Frame.vue` `if (!drawer.open) overview.value = …`, senza `!invoke.asking` | rosso: `expected true to be false` |
| `frame.test.ts`, F3 sotto il cassetto (D16) | in `Frame.vue` `if (!invoke.asking) overview.value = …`, senza `!drawer.open` | rosso: `expected true to be false` |
| `frame.test.ts`, la carta scelta | in `choose` tolta la riga `open.value = false;` | rosso: `expected true to be false` — la Panoramica resta aperta; e la prova delle frecce, `expected <div …> to be null` (**E95**) |
| `frame.test.ts`, il rifiuto detto (D4) | in `save` tolta la riga `refusal.value = result;` | rosso: `expected 'AnnullaSalva' to contain 'Scrivi un nome.'` |
| `frame.test.ts`, i nomi delle tre (D12) | in `save` `props.snapshot(), []);` al posto dei nomi | rosso: `a name that is taken keeps the overview open: expected false to be true` — *«home»* salvata |
| `frame.test.ts`, la miniatura (D14) | in `drawn` tolta la riga del `.filter` sulla striscia | rosso: `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'activity', 'costs', …(4) ]` |
| `frame.test.ts`, il pulsante della striscia | in `Strip.vue` tolto ` @click="drawer.open = true"` | rosso: `expected false to be true`; e nel browser la prova del cassetto nei due temi, `expected null not to be null` (**E95**) |
| `a11y.test.ts`, la Panoramica aperta | in `Overview.vue` tolta la riga `<BaseLabel>{{ card.name }}</BaseLabel>`: carte senza nome | rosso: `expected [ Array(1) ] to deeply equal []`; e nel browser la Panoramica disegnata nei due temi, lo stesso messaggio (**E95**) |
| `copy.test.ts`, le parole delle viste | in `it.json` tolta la riga di `"compact"`, **con la virgola** di `"work"` — senza, il JSON non si legge e dodici file non si caricano, `trailing comma` (**E95**) | rosso: `compact: expected [ 'home', 'work' ] to include 'compact'` |
| `frame.browser.test.ts` e `dock.browser.test.ts`, la pillola | in `dock.css` tolta la regola `:has(.strip)` | rosso, sei prove nei due temi: `expected 21 to be greater than or equal to 25`, `expected '21px' to be '9999px'` — la scheda del 6bis, **E64** — e i raggi del dock, `expected [ …(2) ] to deeply equal []` — il pulsante a pillola in una scheda |
| `frame.browser.test.ts`, i 24 px dal fondo | in `Frame.vue` il margine di `.dock` a `0 var(--space-3) var(--space-3)` | rosso, nei due temi: `expected 12 to be close to 24, received difference is 12` |
| `frame.browser.test.ts`, gli angoli della pagina | in `ViewBar.vue` il padding di `.bar` a `var(--space-2) 0` | rosso, nei due temi: `expected [ 'base-button view-name' ] to deeply equal []` |
| `frame.browser.test.ts`, il fuoco che torna (P-22) | in `Strip.vue` `@click="($event.currentTarget as HTMLElement).blur(); drawer.open = true"` | rosso, nei due temi: `expected <body style>…(1)</body> to be <button …>` — il fuoco torna a chi l'aveva prima di aprire |
| `frame.browser.test.ts`, i raggi della Panoramica | in `Overview.vue` il raggio di `.mini` a `var(--radius-card)` | rosso, nei due temi: `expected [ …(12) ] to deeply equal []` |
| `frame.browser.test.ts`, il testo tagliato | in `Overview.vue` `.caption` con `width: 40px;` e `overflow: hidden;` | rosso, nei due temi: `expected [ 'cut: caption "Lavoro" 49>40', …(1) ] to deeply equal []` |
| `frame.browser.test.ts`, le icone centrate | in `Overview.vue`, prima di `.caption {`, la regola `.tile :deep(.base-icon) { margin-top: 4px; }` | rosso, nei due temi: `expected [ …(15) ] to deeply equal []` |
| `frame.browser.test.ts`, il contrasto | in `Overview.vue`, prima di `.caption {`, la regola `.caption :deep(.base-label) { color: var(--color-border); }` | rosso, nei due temi: `expected [ Array(1) ] to deeply equal []` |
| `frame.browser.test.ts`, le frecce (decisione 19) | in `Overview.vue` tolto ` @keydown="onArrow"` | rosso: `expected <button …(8)>…(2)</button> to be <button …(7)>…(2)</button>` — il fuoco resta su Home |
| `frame.browser.test.ts`, il giro del Tab (D17) | in `Overview.vue` `:tabindex="0"` su ogni carta delle viste | rosso: `expected [ +0, +0, +0, -1 ] to deeply equal [ +0, -1, -1, -1 ]` |
| `frame.test.ts`, una delle tre chiude la vista col nome (**E97**) | in `Overview.vue` `show: () => { layout.view = view; },` al posto di `showView` | rosso: `expected 'Lettura' to be null` |
| `frame.test.ts`, la vista col nome scelta (**E97**) | in `Overview.vue` `layout.showNamed((layout.saved?.named ?? [])[0]?.name ?? entry.name);` | rosso: `expected 'Revisione' to be 'Lettura'` |
| `frame.test.ts`, «Salva questa vista» spenta (**E98**) | in `Overview.vue` tolta la riga `:disabled="layout.arrivals === 0"` | rosso: `expected false to be true` |
| `frame.test.ts`, la miniatura che non si legge (**E99**) | in `drawn` il `try`/`catch` tolto: `tiles = schematic(saved);` | rosso: `TypeError: Cannot destructure property 'maximizedNode' of 'layout.grid' as it is undefined.` |
| `stores.test.ts`, un nome che la lista non tiene (**E99**) | in `layout.ts` tolta la riga del `return false` di `showNamed` | rosso: `expected true to be false` |
| `frame.browser.test.ts`, l'anello dentro (**E100**) | in `dock.css` tolta la riga `outline-offset` della regola `.panel > :focus-visible` | rosso: `expected 9 to be greater than or equal to 13` |
| `frame.browser.test.ts`, gli angoli dell'anello (**E100**) | in `dock.css` tolte le due righe `border-bottom-*-radius` della stessa regola | rosso: `expected +0 to be close to 20, received difference is 20` |
| `frame.browser.test.ts`, il cassetto dalla tastiera (**E96**, **E38**) | in `BaseDialog.vue` tolta la riga `position: sticky;` delle azioni del foglio | rosso, nei due temi: `expected 1025 to be less than or equal to 900` — «Chiudi» col fuoco, sotto il fondo del foglio |
| `frame.browser.test.ts`, il segnaposto intero (**E96**, **E39**) | in `ViewBar.vue` `max-width: 320px;` al posto di `24rem` | rosso: `expected 314.8876953125 to be less than or equal to 270` |
| `frame.browser.test.ts`, la barra senza fondo (**E96**, **E57**) | in `ViewBar.vue`, dopo il `padding` di `.bar`, `background: var(--color-bg-raised);` | rosso: `expected 'rgb(251, 248, 243)' to be 'rgba(0, 0, 0, 0)'` |
| `frame.browser.test.ts`, la pagina che non sborda (**E96**, **E43**) | in `Frame.vue` tolta la riga `overflow: clip;` di `.dock` | rosso, nei due temi: `expected [ 50, +0, +0, +0, +0, +0, +0, +0 ] to deeply equal [ +0, +0, +0, +0, +0, +0, +0, +0 ]` — lo sbordo del primo fotogramma |
| `frame.browser.test.ts`, la fascia `stop` sulla pagina (**E96**, **E70**) | in `Band.vue` tolto `on-page` dal ramo `stale` | rosso, nei due temi: `expected '8px' to be '21px'`; e sotto jsdom la prova del 6bis, *«stops the window…»*, `expected undefined to be defined` |
| `frame.test.ts`, la miniatura di ogni vista spedita (**E103**) | in `compact.json` le due `"size"` delle foglie, `944` e `56`, a `500`, com'erano | rosso: `compact: expected 50 to be greater than or equal to 94.399999` |
| `frame.test.ts`, l'ingrandimento che si salva (**E104**) | in `dock.ts` tolta la riga `api.onDidMaximizedGroupChange(() => queueMicrotask(settleIfMoved));` | rosso: `expected 1 to be 2` |
| `frame.test.ts`, mostrare che non salva (**E104**) | in `dock.ts` `api.onDidMaximizedGroupChange(settleIfMoved);`, sentito subito e non differito | rosso: `expected 4 to be 2` — la vista lasciata, salvata col nome di quella aperta |
| `frame.browser.test.ts`, «Salva questa vista» spenta (**E105**) | in `BaseButton.vue` tolta la regola `.base-button:disabled :deep(.base-label), .base-button:disabled :deep(.base-icon)` | rosso, nei due temi: `expected 'rgb(101, 91, 87)' to be 'rgb(163, 154, 143)'` nel chiaro, `expected 'rgb(163, 154, 143)' to be 'rgb(111, 102, 96)'` nello scuro |
| `frame.browser.test.ts`, l'icona spenta (**E105**) | nella stessa regola tolto il selettore `.base-button:disabled :deep(.base-icon)` | rosso, nei due temi: `expected 'rgb(122, 31, 46)' to be 'rgb(163, 154, 143)'` nel chiaro, `expected 'rgb(191, 85, 103)' to be 'rgb(111, 102, 96)'` nello scuro |
| `frame.browser.test.ts`, le parole accese dopo la risposta (**E105**) | nella stessa regola `.base-button :deep(.base-label)`, senza `:disabled` | rosso, nei due temi: `expected 'rgb(163, 154, 143)' to be 'rgb(101, 91, 87)'` nel chiaro, `expected 'rgb(111, 102, 96)' to be 'rgb(163, 154, 143)'` nello scuro — le parole ereditano il colore del pulsante, all'inizio della sua transizione dallo spento |
| `frame.test.ts`, il segnaposto del nome (**E106**) | in `Overview.vue` tolta la riga `:placeholder="$t('overview.name')"` | rosso: `expected null to be 'Nome della vista'` |
| `frame.browser.test.ts`, il giro del Tab su «Salva questa vista» (**E107**, **D17**) | in `Overview.vue` tolto `@focus="roving = cards.length"` | rosso: `expected [ -1, +0, -1, -1 ] to deeply equal [ -1, -1, -1, +0 ]` |
| `frame.browser.test.ts`, l'apertura dalla vista sullo schermo (**E107**, **D17**) | nel `watch` di `Overview.vue` tolta la riga `roving.value = Math.max(0, …)` | rosso: `expected <button …(7)>…(1)</button> to be <button …(8)>…(2)</button>` — il fuoco apre su «Salva questa vista» |
| `frame.browser.test.ts`, le frecce nel campo del nome (**E107**) | in `onArrow` tolto ` \|\| !from.hasAttribute("data-card")` | rosso: `expected false to be true` — la freccia a destra porta il fuoco su Lavoro |
| `frame.test.ts`, F3 tolto al browser (**E107**, **P-11**) | in `Frame.vue` tolta la riga `event.preventDefault();` del ramo di F3 | rosso: `expected false to be true` |
| `frame.test.ts`, una sola carta in bordeaux (**E107**) | in `Overview.vue` `current: layout.view === view,` per le tre viste | rosso: `expected [ null, 'page', null, null, …(2) ] to deeply equal [ Array(6) ]` |
| `frame.browser.test.ts`, «Annulla» (**E107**) | in `cancel` tolta la riga `naming.value = false;` | rosso: `expected <form …(1)>…(2)</form> to be null` |
| `frame.test.ts`, «Chiudi» del cassetto (**E107**, R3-20) | in `Drawer.vue` `@click="() => {}"` al posto di `@click="drawer.open = false"` | rosso: `expected true to be false` |
| `frame.browser.test.ts`, il fuoco nel campo del nome (**E107**) | in `startNaming` tolta la riga del `focus()` | rosso: `expected false to be true` |
| `frame.test.ts`, il nome scritto a metà (**E107**) | nel `watch` di `Overview.vue` tolte le due righe `naming.value = false;` e `name.value = "";` | rosso: `expected <form …(1)>…(2)</form> to be null` — il campo torna aperto; tolta la sola `name.value = "";`, `expected '  home ' to be ''` |
| `probes.browser.test.ts`, un token che la pagina non definisce (**E108**) | in `probes.ts` tolta la riga `readToken(token);` di `computed` | rosso: `expected [Function] to throw an error` |

Alla fine `git status --porcelain | diff <scratchpad>/prima.txt -` rende soltanto i file del compito.

- [ ] **Passo 10: guardarlo, nei due temi**

⛔ **Un passo per il proprietario col coordinatore, non per l'implementatore** (**E101**, la forma di **E35** ed **E69**): si fa
dopo la revisione e le sue cure, sul codice che resta, e l'implementatore lo **salta**; il revisore guarda la SPA per conto
suo, com'è la regola 5 di *«Come si esegue un compito»*.

`(cd gui && npm run dev)`, e nel browser, a finestra piena, nel tema chiaro e nello scuro — la scelta in Impostazioni: la
barra col **nome della vista** e l'icona, parte della pagina, la ricerca spenta col segnaposto intero e il chip a destra;
**F3** e il clic sul nome aprono la Panoramica, con le **miniature** delle tre viste — i moduli dove stanno, con le loro
icone — e, ingrandito un gruppo con la sua presa, la miniatura che lo disegna solo, sul quadrato intero (**E85**): *come* si
disegni una miniatura lo decide questo sguardo; la vista corrente in **bordeaux** e *«Salva questa vista»* sotto, spenta
finché il core finto non ha risposto — `harnessFake.deliverAll()` nella console (**E98**); le frecce che girano la griglia,
Invio che entra, Esc che chiude e rende il fuoco al nome; il nome di una vista nuova, rifiutato vuoto e rifiutato *«home»*,
poi salvato; in fondo la **striscia a pillola**, a 12 px dai lati e a 24 dal fondo, col pulsante *«moduli»* che apre il
cassetto, anche dalla tastiera; e col Tab, nella Home, il fuoco su Stato, che scorre, con l'anello **dentro** la scheda
(**E100**). Un difetto che si vede e che nessuna prova ha colto è una voce d'errata (la regola 5 di *«Come si esegue un
compito»*).

- [ ] **Passo 11: il cancello, il commit**

`bash scripts/gate.sh` da solo, `bash scripts/check-docs.sh`, il commit — `design-system(compito 8): la cornice …`, col pezzo
JavaScript misurato accanto alla baseline del Passo 1 — coi fine-riga rimisurati, e `git push`. ⛔ **Senza le celle della
posizione** (**E101**): le scrive il coordinatore nel commit della chiusura, dopo il Passo 10 approvato — la riga **8** a
`✅ <data>` col verbale dello sguardo, e nella riga **7** la colonna **Commit**, `0417c9c`, **con la cura `a606f33`**
(**E79**, R1-16).

---

## Compito 9: la chiusura — i documenti in ogni casa, e la Definizione di «fatto» coi comandi

**Da:** la riga **9** della tabella della posizione e la forma del *«Come si riprende»* del 2026-09-24 — R2-8, R3-23 e R3-25
della revisione —; il controllo **21** e la (e) del disegno; il punto 9 del *«Come si riprende»* del disegno; **P-26**…**P-30** e
**D19**…**D24** di questo piano. Il modello è il compito 17 del [piano della parte 2](2026-09-11-sottoprogetto-2-parte-2-gui-minima.md):
i suoi Passi 1–11, e le voci **E216**…**E224** della sua errata, che sono ciò che quel compito ha sbagliato — si leggono **prima**.

**Files:**
- Modify: `docs/tracciabilita.md` — la riga «Accessibilità», da ✅ a 🔶 col richiamo datato e le sedi di oggi; una riga
  *«Aggiornata il»* nel riquadro in testa
- Modify: `docs/roadmap.md` — la riga **14** in coda ai sotto-progetti, la sua riga in *«Perché quest'ordine»*, la riga di questo
  piano nella tabella dei piani a *«eseguito»*, l'intestazione *«Ultimo aggiornamento»*
- Modify: `docs/README.md` — la riga del disegno nella tabella «Specifiche»: il numero del sotto-progetto e l'esecuzione
- Modify: `docs/COMPENDIO.md` — una riga in **§12** (D22), il puntatore ⏭️ della **§6** riscritto, l'intestazione
- Modify: `docs/archivio/stato-storico.md` — il puntatore ⏭️ e l'intestazione del compendio com'erano, parola per parola
- Modify: `docs/porta-di-qualita.md` — **una** sezione nuova, il browser dei test e la pagina kit nel passo web; la tabella dei passi
  di `gate.sh` a nove righe, e la riga **C-S0-1** delle contraddizioni col suo ✅ (P-29, D21)
- Modify: `docs/riferimenti.md` — una sezione: le fonti della scrittura e della revisione del piano, le misure dell'esecuzione
- Modify: `docs/HANDOFF.md` — **solo** se l'esecuzione ha portato un gotcha nuovo: la sua riga nella sezione *«I gotcha»*, e
  l'intestazione *«Aggiornato il»*
- Modify: `docs/superpowers/specs/2026-09-22-design-system-design.md` — l'esito di ogni riga della (e), e la riga *«codice di
  prodotto»* del *«Come si riprende»* col richiamo (P-28)
- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` — la riga *«codice e spec non toccati»* della tabella dello
  stato, col richiamo (P-28)
- Modify: `docs/superpowers/plans/2026-09-23-design-system.md` — la Definizione di «fatto» con le uscite del giorno, la testa
  *«A che punto è»*, la riga **9** della posizione — la **8** porta già la colonna Commit (**E115**) —, *«Come si riprende»*:
  ⛔ **nella chiusura, dal coordinatore** (**E120**)
- Modify: `docs/archivio/consegna-piano-design-system.md` — la chiusura precedente di questo piano, parola per parola:
  ⛔ **nella chiusura, dal coordinatore** (**E120**)
- ⛔ **NON si tocca il codice** — `crates/`, `gui/`, `scripts/`, `.github/` —, nessun ADR, la §5 del compendio, la spec del
  sotto-progetto 1, e **nessun piano eseguito**: né `E187` né `E235` del piano della parte 2, che sono verbali — se un verbale
  che afferma il falso vada corretto è la voce **X-4** dell'audit, del proprietario
- ⚠️ **I fine-riga non si scrivono qui:** li misura `git ls-files --eol` al Passo 1 sulla macchina che esegue, e il Passo 13 li
  vuole **invariati** (E224 della parte 2); ogni scrittura passa da `replace_unique.py` o da Python con `newline=""`
- Read: la **§12** e la **§13** del compendio; il riquadro in testa a `tracciabilita.md`; la sezione *«IL PASSO WEB E LE SONDE
  DELLA PARTE 2»* di `porta-di-qualita.md`, che è il modello della sezione nuova; la (e), la (f) e la tabella dei controlli del
  disegno; l'**errata** di questo piano per intero; le celle **Stato** della tabella della posizione — la **5** porta il verbale
  dell'Assistente vocale; le tabelle *«Ciò che questa sessione ha imparato»* delle consegne (**E116**)
- ⛔ **NON si leggono** i compiti 1–8 per intero: di ciascuno servono il passo del cancello e le prove che nomina, che la
  Definizione di «fatto» qui sotto già raccoglie

**Interfaces:**
- Consumes: ciò che i compiti 1–8 hanno prodotto — le righe *Produces* di ciascuno — e le **uscite attese** che la Definizione
  di «fatto» raccoglie; `replace_unique.py` della testa; `dod_suite.py` della Definizione di «fatto»
- Produces: nessun artefatto di codice. ⛔ **Produce ciò che una sessione nuova legge per sapere dov'è:** il puntatore della
  §6 e la riga 14 della roadmap

⛔ **Questo è il compito in cui si sbaglia per ZELO** — il compito 17 della parte 2 lo dice per esteso: riallineare una cifra
invece di toglierla (gotcha #68); ricopiare in un secondo documento ciò che ha già una casa; allungare il puntatore della §6 con
ciò che si è chiuso; correggere una riga che questo piano non ha reso falsa, **tranne** C-S0-1, scelta dal proprietario
(P-29). ⛔ **La regola: si tocca ciò che questo piano ha reso falso o ha lasciato da scrivere, e nient'altro.** E `<data>`, in
ogni testo dettato qui sotto, è la data del giorno dell'esecuzione: il Passo 13 vuole che nessun `<data>` sopravviva fuori da
questo piano.

- [ ] **Passo 1: le misure prima — la baseline di ogni casa**

```bash
git status --porcelain > <scratchpad>/prima.txt
git ls-files --eol docs/tracciabilita.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/archivio/stato-storico.md \
    docs/porta-di-qualita.md docs/riferimenti.md docs/HANDOFF.md docs/superpowers/specs/2026-09-22-design-system-design.md \
    docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/plans/2026-09-23-design-system.md \
    docs/archivio/consegna-piano-design-system.md
wc -c docs/COMPENDIO.md; grep -n '^ceiling=' scripts/check-docs.sh
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
grep -n '^| Accessibilità |' docs/tracciabilita.md | cut -c1-40
grep -n '^| 1[0-9] |' docs/roadmap.md | cut -c1-40
grep -n 'plans/2026-09-23-design-system.md' docs/roadmap.md docs/README.md docs/COMPENDIO.md | cut -c1-60
awk '/^\*\*Un comando solo:\*\*/{s=1; next} s&&/^\|/{t=1} s&&t&&!/^\|/{exit} s&&/^\| [0-9]+ \|/{c++} END{print c}' docs/porta-di-qualita.md
grep -n '^run ' scripts/gate.sh
awk '/^## I gotcha/{s=1; next} s&&/^## /{s=0} s&&/^\| [0-9]+ \|/{c++} END{print "gotcha: "c}' docs/HANDOFF.md
git log --format='%h %s' --grep='^design-system(compito' | cut -c1-72
B=$(git log --format=%H --grep='^design-system(compito 1):' | tail -1)^; git rev-parse --short "$B"
git diff --stat "$B"..HEAD -- crates/ gui/schema/
awk -F'|' '/^\| \*\*[0-9]\*\* \|/{print $2 "|" $5}' docs/superpowers/plans/2026-09-23-design-system.md | cut -c1-120
bash scripts/gate.sh 2>&1 | tail -1
```

Atteso: **dodici** file, tutti `i/lf`; la colonna `w/…` è **della sessione** — la decide `core.autocrlf`, che non è versionato
— e si annota per il Passo 13. Il tetto e il peso del compendio, da cui il **margine**. Il conto per stato di tracciabilità,
che il Passo 2 rifà (controllo 21), e la riga «Accessibilità» ancora ✅. Nella roadmap le righe fino alla **13**, e **nessuna**
14. Il percorso del piano **una** volta nella roadmap — la riga della tabella dei piani, **D19** — e **una** nel compendio, nella
§6; **nessuna** in `README.md`, dove sta il disegno. La tabella dei passi a **sette** righe, e `gate.sh` a **nove** `run`
(P-29). Il numero dei gotcha, la baseline del Passo 9. I commit dei compiti 1–8, uno o più ciascuno; `B` è il genitore del
**primo** commit del compito 1, e il `git diff --stat` su `crates/` e `gui/schema/` è **vuoto** (vincolo 12). Le righe 1–8
della posizione ✅, la 9 ⬜. `GATE GREEN.`

⛔ **Se la riga 14 c'è già, o la riga «Accessibilità» è già 🔶, il compito è già in parte eseguito:** ci si ferma e si riporta,
invece di scrivere righe doppie. ⚠️ **Se la tabella dei passi ha già nove righe**, C-S0-1 l'ha corretta la sessione delle
contraddizioni: la parte del Passo 7 che la riguarda **non** si fa, e lo si dice nel commit.

⚠️ **Una cosa del proprietario, che si guarda adesso e non alla fine** — ✅ la seconda, **D18**, l'ha scelta il 2026-09-24, **A**:

| | Dove si guarda | Se manca |
|---|---|---|
| il **giudizio sull'aspetto del dock** (controllo 15, passo 8 del compito 6) | la cella Stato della riga 6, o una voce d'errata che nomina il controllo 15: `grep -n 'controllo 15' docs/superpowers/plans/2026-09-23-design-system.md` | il coordinatore lo chiede prima del commit (**D24**), in A/B — *«approvato»* o *«non approvato, e perché»* —, e la risposta va nella Definizione di «fatto», blocco 5 |

- [ ] **Passo 2: `tracciabilita.md` — la riga «Accessibilità», e il riquadro**

In `docs/tracciabilita.md`, *Trova* la riga che comincia con `| Accessibilità | ✅ |`, **intera, presa dal file** —
*Sostituisci con*:

```markdown
| Accessibilità | 🔶 | nel **2** e nel **14**, il design system — ⚠️ **RICHIAMO DEL <data>:** era ✅, e la legenda dà a ✅ le fondamenta *nel kernel*, mentre queste stanno nella GUI: cambia la riga, non la legenda (N-2 di E235 del piano della parte 2; la (e) del [disegno del design system](superpowers/specs/2026-09-22-design-system-design.md)) · `axe-core` sui pannelli montati (`gui/src/a11y.test.ts`, l'aiutante in `gui/src/testing/axe.ts`) e, nel browser vero col contrasto della pagina disegnata, sulla pagina kit e sulla cornice (`gui/src/kit/kit.browser.test.ts`, `gui/src/frame/frame.browser.test.ts`); il contrasto AA dei token per famiglie, nei due temi (`gui/src/tokens/contrast.test.ts`); il movimento ridotto e l'alto contrasto (`gui/src/tokens/tokens.browser.test.ts`); le regioni `role="status"` presenti prima del loro testo (`gui/src/components/BaseStatus.vue`); la tastiera (`gui/src/frame/moveActive.ts`, `gui/src/frame/keys.test.ts`, e le frecce della Panoramica in `gui/src/frame/Overview.vue`) — nel passo web del cancello |
```

Ogni percorso della riga si prova: `ls gui/src/a11y.test.ts gui/src/testing/axe.ts gui/src/kit/kit.browser.test.ts
gui/src/frame/frame.browser.test.ts gui/src/tokens/contrast.test.ts gui/src/tokens/tokens.browser.test.ts
gui/src/components/BaseStatus.vue gui/src/frame/moveActive.ts gui/src/frame/keys.test.ts gui/src/frame/Overview.vue` — un
file che manca è una voce d'errata: la riga dice dove vive, e non può nominare ciò che non c'è.

Poi, nel riquadro in testa, *Trova* il capoverso che comincia con `> ✅ **Aggiornata il 2026-09-22 con le sedi dei pezzi della
GUI`, **intero, preso dal file**, e *Sostituisci con* lo stesso capoverso seguito da:

```markdown
>
> ✅ **Aggiornata il <data> con la riga «Accessibilità»**, dal compito 9 del [piano del design system](superpowers/plans/2026-09-23-design-system.md) — alla chiusura del sotto-progetto 14, come dice la riga sotto il titolo.
```

Il comando del riquadro, **dopo**: ✅ **uno in meno** e 🔶 **uno in più** del Passo 1, gli altri uguali (controllo 21).

- [ ] **Passo 3: `roadmap.md` — la riga 14, la sua ragione, la riga del piano, l'intestazione**

Quattro tocchi, ciascuno con `replace_unique.py` e il testo vecchio **preso dal file**.

**Uno.** Dopo la riga che comincia con `| 13 |`, **intera**, la riga:

```markdown
| 14 | **Design system** — i token a due livelli e i due temi, il kit dei pezzi di base con la pagina kit, il dock vestito, la cornice con la Panoramica e le viste col nome, le prove nel browser vero — [disegno](superpowers/specs/2026-09-22-design-system-design.md) | — | ✅ **chiuso il <data>** — la Definizione di «fatto» del [piano](superpowers/plans/2026-09-23-design-system.md), coi comandi | 2 |
```

**Due.** In *«Perché quest'ordine»*, dopo la riga che comincia con `| **Registro delle guide (13)`, **intera**, la riga:

```markdown
| **Design system (14) dopo la GUI minima, e prima del registro delle guide (13)** | la richiesta del proprietario del 2026-09-21, punto 5 della trentasettesima chiusura del [piano della parte 2](superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md): lo stile del 2 era un segnaposto dichiarato, e il design system si fa a 2 chiuso. Il numero è il primo libero in coda, senza rinumerare — la voce 2 del [disegno](superpowers/specs/2026-09-22-design-system-design.md), scelta **A** il 2026-09-23. Non tocca il kernel né il filo (vincolo 12 del piano), quindi non cambia niente per il 13, che resta sbarrato da AUD-004 |
```

**Tre.** Nella riga di questo piano nella tabella dei piani (**D19**), la cella *Stato* — dal `⏳` alla fine della riga —
diventa:

```markdown
✅ **scritto il 2026-09-24, eseguito il <data>** — `GATE GREEN` a ogni compito; la Definizione di «fatto» coi comandi sta nel piano |
```

**Quattro.** La riga che comincia con `Ultimo aggiornamento:`, **intera**, diventa:

```markdown
Ultimo aggiornamento: **<data>**, con la **chiusura del sotto-progetto 14**, il design system — la riga 14 dei sotto-progetti con la sua riga in *«Perché quest'ordine»*, e la riga del [piano del design system](superpowers/plans/2026-09-23-design-system.md) nella tabella dei piani portata a «eseguito» dal compito 9 di quel piano.
```

Sonde: `grep -c '^| 14 | \*\*Design system\*\*' docs/roadmap.md` → **1**; `grep -c '^| \*\*Design system (14)' docs/roadmap.md`
→ **1**; `grep -c 'eseguito il <data>' docs/roadmap.md` → **1**, con la data vera al posto di `<data>`;
`grep -n '^| 1[0-9] |' docs/roadmap.md` → le righe 10–14, in fila.

- [ ] **Passo 4: `README.md` — la riga del disegno**

La riga del disegno nella tabella «Specifiche», riscritta nel commit che ha scritto questo compito (**D19**), riceve il numero
e l'esecuzione. *Trova* la riga che comincia con `| [Design system — il disegno]`, **intera, presa dal file** — e nella
seconda colonna `il design system della GUI` diventa `il design system della GUI, il sotto-progetto 14`; in coda alla terza,
prima di ` |` che chiude la riga:

```markdown
 ✅ **RICHIAMO DEL <data>:** il piano è **eseguito** — la Definizione di «fatto» del [piano](superpowers/plans/2026-09-23-design-system.md), coi comandi
```

Sonda: `grep -c 'il sotto-progetto 14 |' docs/README.md` → **1**.

- [ ] **Passo 5: la §12 del compendio — una riga, e il margine rimisurato**

Dopo la riga che comincia con `| ⛔ **come si è ESEGUITO il sotto-progetto 2**`, **intera, presa dal file**, la riga (**D22**):

```markdown
| ⛔ **il DESIGN SYSTEM**, il sotto-progetto 14 — i token a due livelli e i due temi, il kit dei pezzi di base con la pagina kit, il dock vestito, la cornice con la Panoramica e le viste col nome, le sonde che diventano prove nel browser vero, e per ogni artefatto il controllo che lo esercita; e come si è **eseguito**, con la Definizione di «fatto» coi comandi | il [disegno](superpowers/specs/2026-09-22-design-system-design.md) — ⚠️ **non è una spec** · il [piano](superpowers/plans/2026-09-23-design-system.md), con l'errata in testa e la tabella della posizione — ⚠️ **a compiti, mai intero** |
```

```bash
wc -c docs/COMPENDIO.md; grep -n '^ceiling=' scripts/check-docs.sh
bash scripts/check-docs.sh 2>&1 | tail -1
```

Atteso: il margine **positivo**, `OK — no inconsistencies.` ⛔ Se il margine va sotto zero si toglie prosa dalla §6, **non si
alza il tetto** (vincolo 13).

- [ ] **Passo 6: la §6 del compendio — il puntatore RISCRITTO, non allungato; l'intestazione; l'archivio**

⛔ **Ciò che è chiuso non si ripete nella §6**: lo dice la colonna «Stato» della roadmap, e la §6 lo scrive in testa al suo
*«Il prossimo passo»*. Il ⏭️ perde il primo tempo — il design system — e il secondo diventa **il** prossimo passo.

**Uno — il vecchio, preso dal file.** Il blocco del ⏭️ è il capoverso che comincia con `⏭️ **IL PROSSIMO PASSO` e finisce alla
prima riga vuota:

```bash
awk '/^⏭️ \*\*IL PROSSIMO PASSO/{s=1} s&&/^\r?$/{exit} s' docs/COMPENDIO.md > <scratchpad>/old-6.txt
head -c 80 <scratchpad>/old-6.txt; echo; tail -c 120 <scratchpad>/old-6.txt
```

Atteso: comincia con `⏭️ **IL PROSSIMO PASSO, IN DUE TEMPI. Uno: il DESIGN SYSTEM` e finisce con `in sessioni distinte.`; dentro
ci sono la riga che il compito 1 ha scritto — `Lo stile dal compito 1 del piano del design system:` — e il secondo tempo,
**Due: IL SOTTO-PROGETTO 13**. ⛔ **Se comincia o finisce altrimenti, la §6 è cambiata dopo la scrittura di questo passo:** ci si
ferma e si riporta. ⚠️ **E si rilegge prima di cancellarlo:** una riga che parla d'altro che del design system o del 13 — una
voce aperta che nessuno ha chiuso — **resta**, e va nel testo nuovo.

**Due — il nuovo**, in `<scratchpad>/new-6.txt`, e `python <scratchpad>/replace_unique.py docs/COMPENDIO.md
<scratchpad>/old-6.txt <scratchpad>/new-6.txt`:

```markdown
⏭️ **IL PROSSIMO PASSO: IL SOTTO-PROGETTO 13** — i tre meccanismi che la knowledge base chiede al kernel, che la **decisione 16** mette **prima**
del 3 — il verdetto della knowledge base del 2026-09-05: **nessuna sesta proprietà** della §3, ma questo **vincolo d'ordine**. ⛔ **Lo sbarra AUD-004**, l'ADR del proprietario sulle skill dichiarative: è una sua decisione e non del piano. Il perimetro
sta nel [disegno della knowledge base](superpowers/specs/2026-09-04-knowledge-base-design.md), che chi riprende quel fronte legge
**per intero**; brainstorming e disegno del 13 vengono prima del piano, in sessioni distinte.
```

⚠️ È il secondo tempo del vecchio, **parola per parola**: niente di nuovo si scrive nel puntatore.

**Tre — l'intestazione.** Il capoverso che comincia con `**Aggiornato il` e finisce con `§13.`, **preso dal file**, va in
`<scratchpad>/old-h.txt`, e diventa:

```markdown
**Aggiornato il <data>**, con la **chiusura del sotto-progetto 14**, il design system — una riga nella §12 e il puntatore della §6 al 13; il puntatore e questa riga com'erano sono in [`archivio/stato-storico.md`](archivio/stato-storico.md). L'ultimo contenuto di merito resta la voce di ADR-0029. Manutenzione, e perché questa riga è la più facile da lasciare indietro: §13.
```

**Quattro — l'archivio.** In coda a `docs/archivio/stato-storico.md`, con Python `newline=""` e il terminatore del file:

```markdown

## Il puntatore «Il prossimo passo» e l'intestazione del compendio, com'erano — archiviati il <data>

⚠️ **Vero il giorno in cui fu scritto.** Usciti dal compendio alla chiusura del sotto-progetto 14, il design system (compito 9
del [piano](../superpowers/plans/2026-09-23-design-system.md)), parola per parola: l'intestazione, e il ⏭️ della §6 dal suo
inizio alla riga vuota; i link riscritti per questa cartella.

<il testo di old-h.txt, una riga vuota, il testo di old-6.txt — coi link riscritti>
```

I link si riscrivono in Python, uno per uno e contati: `](superpowers/` → `](../superpowers/`, `](archivio/` → `](`, e ogni altro
`](<file>.md` della cartella `docs/` → `](../<file>.md`. Sonde: `grep -c 'archiviati il <data>' docs/archivio/stato-storico.md`
→ **1**, con la data vera; `grep -c 'IN DUE TEMPI' docs/COMPENDIO.md` → **0**; `grep -c '^⏭️ \*\*IL PROSSIMO PASSO: IL
SOTTO-PROGETTO 13' docs/COMPENDIO.md` → **1**; e il margine, di nuovo, coi due comandi del Passo 5.

⛔ **E parola per parola** (**E117**): una riscrittura che perde la `)` di un rimando passa ogni sonda qui sopra, e anche
`check-docs.sh` quando nessun'altra `)` segue sulla riga, o quando il rimando porta un'ancora `#…` (**E121**). L'aiutante
vive nello scratchpad, come `replace_unique.py`:

```python
"""word_for_word.py -- is a text archived word for word, but for its link targets? (design system, task 9: E117)

Usage: python word_for_word.py <archive.md> <original.txt> [<original.txt> ...]

Each <original.txt> holds a text as it was (UTF-8, any line ending). The link TARGETS are stripped on both sides,
`](...)` -> `]()`, and every original must be found in the archive, in the order given: a rewrite that changed
anything but a target -- a lost `)`, a word, a line -- is not found. Exits 1 when one is not. The paths come in as
ARGUMENTS, so Git Bash turns `/c/...` into `C:/...` for Python (P-30).
"""
import io
import re
import sys

if len(sys.argv) < 3:
    sys.exit("usage: python word_for_word.py <archive.md> <original.txt> [<original.txt> ...]")


def read(path):
    return io.open(path, encoding="utf-8", newline="").read().replace("\r\n", "\n")


def strip(text):
    return re.sub(r"\]\([^)]*\)", "]()", text)


archive = strip(read(sys.argv[1]))
at = 0
for path in sys.argv[2:]:
    original = strip(read(path).strip("\n"))
    if not original:
        sys.exit(f"{path}: empty -- an empty original is found everywhere, and proves nothing")
    found = archive.find(original, at)
    if found < 0:
        print(f"NOT FOUND: {path} -- the archived text differs from it")
        sys.exit(1)
    at = found + len(original)
    print(f"word for word: {path}")
sys.exit(0)
```

```bash
python <scratchpad>/word_for_word.py docs/archivio/stato-storico.md <scratchpad>/old-h.txt <scratchpad>/old-6.txt; echo $?
```

Atteso: `word for word` per i due file, e **0**. Provato il 2026-09-28 dal pre-controllo in tre direzioni: **0** sull'archivio
giusto, **1** con la `)` di un rimando persa, **1** coi due file in ordine rovesciato.

- [ ] **Passo 7: `porta-di-qualita.md` — UNA sezione nuova, e la tabella dei passi a nove righe**

**Uno — la sezione**, sul modello di *«IL PASSO WEB E LE SONDE DELLA PARTE 2»*: si inserisce **prima** della riga
`## Le contraddizioni registrate, e non risolte`, così le sezioni datate restano in ordine di data e le due di servizio in
fondo.

```markdown
## ⛔ IL BROWSER DEI TEST E LA PAGINA KIT NEL PASSO WEB — <data>

Dal [piano del design system](superpowers/plans/2026-09-23-design-system.md), il sotto-progetto 14, per la (f) del suo
[disegno](superpowers/specs/2026-09-22-design-system-design.md). Due cose entrano nel passo web, `gui: fake core and SPA`, e
**nessun passo nuovo**: i sotto-passi di `scripts/gate-gui.sh` restano quelli che `grep -n 'gui:' scripts/gate-gui.sh` elenca.

| Dove | Che cosa | Da |
|---|---|---|
| `gui: probes` | `npm test` gira i **due progetti** di `vitest` **uno alla volta** — `npm test -- --project jsdom`, poi `npm test -- --project browser`: in una corsa sola un progetto che non trova file è verde, da solo è rosso (**E10**) —: `jsdom`, e `browser` sul **Chrome installato**, canale `chrome`, senza finestra ma **con le barre di scorrimento**, che Playwright altrimenti nasconde (**D26**) — ogni `src/**/*.browser.test.ts`, per ciò che solo un motore d'impaginazione giudica: i caratteri, il movimento ridotto, l'alto contrasto, i raggi, lo spessore della barra di scorrimento, il testo tagliato, le icone centrate, `axe` col contrasto della pagina disegnata. Ogni prova porta la guardia di non-vacuità | compito 2, e il commento sopra i due `npm test` in `scripts/gate-gui.sh`; le barre, compito 6bis (**E64**), e il commento in `gui/vite.config.ts` |
| `gui: build` | la **pagina kit fuori dal pacchetto**: `dist/` vuotato, poi `npm run build`: rosso se `dist/index.html` manca — la guardia di non-vacuità, che senza lo svuotamento leggeva un `dist/` rimasto (**E31**) — e rosso se `dist/kit.html` c'è o un file di `dist/assets` porta `kit-card`, *«the kit page is in the package»* | compito 4, e la sua cura (**E31**) |

⛔ **Un prerequisito dell'ambiente, come il bersaglio di `rustup` e `cargo audit`:** **Google Chrome** stabile, o
`npx playwright install chrome`. Col canale `chrome` il Chromium che Playwright scarica **non** vale (R2-8 della revisione del
piano): una macchina senza Chrome va **rossa** a `gui: probes`, col messaggio di Playwright, e non verde. Le due immagini della
CI, `ubuntu-latest` e `windows-latest`, portano Chrome — la fonte in [`riferimenti.md`](riferimenti.md), *«Il browser dei
test — la risposta 22»*.

⚠️ **Non hanno una riga di catalogo**, come il passo web della parte 2 e per la stessa ragione (gotcha #36): la §7.4 è spec, e
una riga nuova è decisione del proprietario — le prove si **registrano** e non si prendono. Le due direzioni di ciascuna, coi
rossi misurati, stanno nei compiti del piano; che cosa c'è alla chiusura lo dice la sua **Definizione di «fatto»**, coi comandi.

| Che cosa la porta NON controlla, di questo lavoro | Perché |
|---|---|
| l'**aspetto** — la SPA e la pagina kit nei due temi, il dock | si **guarda**: i passi «guardarlo» dei compiti e la regola 5 di *«Come si esegue un compito»* del piano; il dock lo giudica il proprietario (controllo 15 del disegno) |
| il **lettore di schermo vero** sulle regioni `role="status"` | a mano, con l'Assistente vocale di Windows (decisione 21 del disegno): il verbale nella riga 5 della tabella della posizione del piano |
| `axe` sul **dock** | trova tre difetti che vengono dalla parte 2 e da `dockview-core`, del proprietario (P-18 del piano): nessuna prova li spegne per andare verde, e nessuna li guarda |
| la **prima pittura** della finestra del guscio | del sotto-progetto 10: il piano la registra fra le voci che sa e non chiude |
```

**Due — la tabella dei passi** (P-29, **D21**; si salta se il Passo 1 l'ha trovata già a nove righe). Le righe `| 5 |`, `| 6 |`
e `| 7 |` della tabella che segue `**Un comando solo:**`, **prese dal file**, diventano:

```markdown
| 5 | `dependency advisories` | `cargo audit` sul `Cargo.lock` — la voce **X-3** dell'[audit](audit-2026-08-27.md) |
| 6 | `attributes of the constrained crates` | livello 2 — `scripts/gate-attributes.sh` |
| 7 | `gui: fake core and SPA` | il **passo web** — `scripts/gate-gui.sh`: il core finto, la SPA, le prove sotto jsdom e nel browser vero, il linter, gli avvisi; le sezioni *«IL PASSO WEB E LE SONDE DELLA PARTE 2»* e *«IL BROWSER DEI TEST E LA PAGINA KIT NEL PASSO WEB»* |
| 8 | `documentation consistency` | livello 2 — `scripts/check-docs.sh` |
| 9 | `DST campaigns -- wall time` | il **tempo di parete** delle campagne, ristampato con `--nocapture` |
```

⚠️ La terza colonna delle righe che c'erano si **copia dal file**, non da qui, se è cambiata dopo la scrittura di questo passo.
Poi la riga `⚠️ **[C-S0-1]**` sotto la tabella esce, con **una** delle due righe vuote che la circondano; e nella tabella
*«Le contraddizioni registrate, e non risolte»*, in coda alla terza colonna della riga **C-S0-1**, prima di ` |` che chiude la riga:

```markdown
 ✅ **Corretta il <data>** dal compito 9 del [piano del design system](superpowers/plans/2026-09-23-design-system.md), dove scrive il browser: la tabella ha i nove passi (P-29, D21)
```

Sonde:

```bash
awk -F'|' '/^\*\*Un comando solo:\*\*/{s=1; next} s&&/^\|/{t=1} s&&t&&!/^\|/{exit} s&&/^\| [0-9]+ \|/{print $3}' docs/porta-di-qualita.md
grep -n '^run ' scripts/gate.sh
grep -c '⚠️ \*\*\[C-S0-1\]\*\*' docs/porta-di-qualita.md
grep -c 'IL BROWSER DEI TEST E LA PAGINA KIT' docs/porta-di-qualita.md
git diff -U0 -- docs/porta-di-qualita.md | grep '^-[^-]'
```

Atteso: le **nove** etichette nell'ordine dei nove `run` — la stessa sequenza, riga per riga; **0** segni; **2** — il titolo
della sezione e la riga 7 della tabella che la nomina; e fra le righe tolte **soltanto** le tre righe vecchie della tabella, la
riga del segno e la riga C-S0-1 com'era: nessun'altra riga del file è toccata.

- [ ] **Passo 8: `riferimenti.md` — le fonti della scrittura e della revisione, e le misure**

Una sezione, dopo *«Il design system della GUI — le fonti del disegno, 2026-09-23»* e le sue sottosezioni, **prima** della riga
`## Cosa NON abbiamo adottato, e perché`:

```markdown
## Il design system della GUI — il piano e la sua esecuzione, <data>

Le fonti che la **scrittura** del [piano](superpowers/plans/2026-09-23-design-system.md) e la sua **revisione** hanno letto, il
2026-09-23 e il 2026-09-24, quelle che le voci d'errata dell'**esecuzione** hanno letto fino al 2026-09-27, e le misure
dell'esecuzione, coi comandi. Il fatto intero sta nella riga del piano che la
tabella nomina: qui la provenienza, lì il merito — una casa ciascuno.

### Le fonti

| Fonte | Letta | Per |
|---|---|---|
| `vitest-dev/vitest` alla v4.1.11, su GitHub: `docs/guide/browser/index.md`, `docs/config/browser/playwright.md`, `docs/api/browser/commands.md`, `docs/api/browser/context.md`, `docs/guide/projects.md`, `docs/config/css.md`, `docs/config/browser/headless.md`, `docs/config/browser/viewport.md` | 2026-09-23 | i due progetti, il comando `emulateMedia`, la finestra delle prove — il compito 2 |
| `@vitest/browser-playwright/dist/index.d.ts`, nel pacchetto installato | 2026-09-23 | il tipo di `ctx.page` — il compito 2 |
| `vitest` 4.1.11 installato: `defaults.timeout ?? 1e3` e `resolved.testTimeout ??= resolved.browser.enabled ? 15e3 : 5e3` | 2026-09-24 | l'attesa di base di `expect.poll` e il tempo di una prova nel browser — P-21 |
| `reka-ui` 2.10.4 installato: `dist/RadioGroup/RadioGroupItem.js` e `Radio.js` | 2026-09-23 | il radio che decide chi lo controlla, e `handleFocus` — P-8, P-19 |
| `reka-ui` 2.10.4 installato: `dist/Dialog/DialogContentModal.js` e `dist/FocusScope/FocusScope.js` | 2026-09-24 | il fuoco che torna senza un `DialogTrigger` — P-22 |
| `dockview` 8.3.1 installato: `dist/styles/dockview.css` | 2026-09-23 | le variabili del tema di riferimento, e le due famiglie che ne restano fuori — P-6, P-15 |
| `dockview-core` 8.3.1 installato: `dist/package/main.esm.mjs` — `updateTheme`, `AriaLevelTracker`, `aria-label` | 2026-09-23 | P-7, P-17, P-18 |
| `@vue/test-utils` 2.5.0 installato: `mount` | 2026-09-24 | P-23 |
| `jsdom` 30.0.1 installato: nessun `matchMedia` | 2026-09-23 | P-12 |
| Playwright 1.63.0: il messaggio per un Chrome che manca, `Run "npx playwright install chrome"` | 2026-09-23 | il prerequisito — R2-8 del [registro della revisione](superpowers/plans/2026-09-23-design-system-revisione/ledger.md) |
| Vite 8.3.0: `build.rolldownOptions`, con `rollupOptions` deprecato | 2026-09-23 | la prova della pagina kit fuori dal pacchetto — R3-10 |
| `dockview-core` 8.3.1: `DockviewTheme` con undici campi | 2026-09-23 | il tema del dock — R3-22 |
| `dockview` 8.3.1 installato: `dist/styles/dockview.css`, `color-scheme: dark` su `.dockview-theme-abyss` | 2026-09-24 | i controlli nativi scuri nel tema chiaro — E4 |
| WCAG 2.2, tecnica G18, `https://www.w3.org/WAI/WCAG22/Techniques/general/G18`, aggiornata il 2026-08-10: la soglia `0.04045`, e `0.03928` prima del maggio 2021, *«no practical effect»* | 2026-09-24 | la luminanza di `contrast.test.ts` — E7 |
| `vitest` 4.1.11 installato: `passWithNoTests` fra le `NonProjectOptions` dei suoi tipi, un'opzione solo globale — `grep -rn 'passWithNoTests' gui/node_modules/vitest/dist` | 2026-09-24 | un progetto vuoto dentro una corsa a due è verde, da solo è rosso — E10 |
| `@vitest/browser` 4.1.11 installato: su *«optimized dependencies changed. reloading»* stampa *«Vite unexpectedly reloaded a test. This may cause tests to fail…»* e consiglia `optimizeDeps.include` — `grep -n 'unexpectedly reloaded' gui/node_modules/@vitest/browser/dist/index.js` | 2026-09-25 | la cache stantia del progetto `browser`, e la via scartata — E24 |
| Vite, *Dep Optimization Options*, `https://vite.dev/config/dep-optimization-options`: `optimizeDeps.force`, *«Set to `true` to force dependency pre-bundling, ignoring previously cached optimized dependencies»*, senza marca di sperimentale; nei tipi di Vite 8.3.0 installato è `@experimental` — `grep -n -B3 'force?: boolean' gui/node_modules/vite/dist/node/index.d.ts` | 2026-09-25 | la cura di E24 |
| `dockview-core` 8.3.1 installato: `dist/package/main.esm.mjs` — `watchElementResize` passa la misura nuova a un `requestAnimationFrame` | 2026-09-26 | il dock che si ridimensiona un fotogramma dopo, e sborda — E43 |
| CSS Backgrounds and Borders Module Level 3, W3C Candidate Recommendation Draft dell'11 marzo 2024, §4.5 *«Overlapping Curves»*, `https://www.w3.org/TR/css-backgrounds-3/#corner-overlap`: `f = min(Li/Si)`, e se `f < 1` **tutti** i raggi si moltiplicano per `f` | 2026-09-26 | il raggio che un angolo disegna, nella sonda dei raggi — E46 |
| Playwright 1.63.0 installato: `lib/coreBundle.js`, senza finestra aggiunge `--hide-scrollbars` — `grep -n -- '--hide-scrollbars' gui/node_modules/playwright-core/lib/coreBundle.js` | 2026-09-26 | un'occhiata senza finestra non vede le barre di scorrimento: si lancia con `ignoreDefaultArgs: ["--hide-scrollbars"]` — E43 |
| `dockview-core` 8.3.1 installato: `dist/package/main.esm.mjs` — `serialize()` della griglia scrive `maximizedNode: { location }`, gli indici dalla radice fino al gruppo ingrandito; e `dist/cjs/dockview/dockviewComponent.d.ts`, dove `SerializedDockview` non lo dichiara | 2026-09-27 | la miniatura di un gruppo ingrandito — E85 |

### Le misure dell'esecuzione

| Che cosa | Il comando | Il valore, con la data | Che cosa sostiene |
|---|---|---|---|
| il pezzo JavaScript della SPA, prima e dopo | `(cd gui && npm run build 2>&1 \| grep -E 'assets/index-.*\.js ')`, sul `main` di `B` — il blocco 1 della Definizione di «fatto» — e su `HEAD` | <i due valori del giorno> | N-2 di E187, del proprietario (R3-25) |
| la suite, file per file, in cinque corse | la Definizione di «fatto» del piano, blocco 2 | lì, con la data | la stabilità, dopo P-19, P-20 e P-21 |
| la barra di scorrimento sotto i colori forzati, e con la preferenza di Windows che tiene sempre visibili le barre | una prova usa-e-getta del progetto `browser` con `commands.emulateMedia({ forcedColors })`, nei due temi, e una pagina di prova coi token davanti al proprietario — **E118** del piano | 2026-09-28, al pre-controllo del compito 9: 10 px e il cursore visibile a riposo sotto i colori forzati; la preferenza di Windows non arriva alla pagina | l'assunzione della barra nel disegno |
```

⚠️ **Prima di scriverla, ogni riga si rilegge contro la riga del piano che nomina** — la P, la R del registro —: la fonte e la data
vengono da lì, non da qui, e dove divergono vince la riga del piano. Il pezzo JavaScript **prima**: se `gui/` su `B` è quello
del 2026-09-24 — `git diff --quiet 2a30916 "$B" -- gui/; echo $?` → **0** — il valore è quello misurato quel giorno, **663,26 kB**,
201,23 compressi, dal log del cancello; se no, si misura su una copia — `git worktree add` in una cartella **corta** di `%TEMP%`,
come la cartella di prova (`LongPathsEnabled` è 0 sulle due macchine), `npm ci --no-audit --no-fund` e `npm run build` lì dentro,
poi `git worktree remove` —, mai con un `checkout` sull'albero di lavoro.

- [ ] **Passo 9: `HANDOFF.md` — soltanto un gotcha nuovo, se c'è**

Si rileggono l'**errata** di questo piano **e** ogni tabella *«Ciò che questa sessione ha imparato, e che non era scritto»*
delle consegne, che promettono *«le raccoglie la chiusura del sotto-progetto»* (**E116**) — le elenca
`grep -n '^📌 \*\*Ciò che questa sessione ha imparato' docs/archivio/consegna-piano-design-system.md docs/superpowers/plans/2026-09-23-design-system.md`,
ancorato all'inizio della riga: la frase sta anche in questo passo e in **E116**.
Una voce che è una trappola **per chi viene dopo**, fuori da questo piano — non un difetto del suo testo —, diventa una riga
nella sezione *«I gotcha»* di `docs/HANDOFF.md`, la sua **unica** casa, nella forma delle righe vicine, e la riga *«Aggiornato
il»* in testa al file segue (finding AUD-039). Una voce che cambierebbe il **modo di lavorare** non si scrive: va nel *«Come si
riprende»* del Passo 12 come **proposta al proprietario**, perché `CLAUDE.md` è suo (*«Manutenzione della documentazione»*); le
altre restano verbali delle loro consegne. ⛔ **Se non ce n'è nessuna, il file non si tocca**, e il commit lo dice. Il comando
della §9 del compendio, prima e dopo: il numero sale di quante righe si sono scritte.

- [ ] **Passo 10: il disegno e la stella polare — gli esiti e i richiami (P-28)**

Nel [disegno](../specs/2026-09-22-design-system-design.md), la tabella della (e): in coda alla colonna *«Chi la chiude»* di tre
righe, prima di ` |` che chiude la riga, l'esito — **letto**, non dedotto; la quarta, **E228**, resta del proprietario e non si tocca.

La riga **N-2 di E235** — l'esito lo dà il Passo 2:

```markdown
 ✅ **Chiusa il <data>** dal compito 9 del [piano](../plans/2026-09-23-design-system.md): la riga di tracciabilità è 🔶, col richiamo datato
```

La riga **M-3 di E187** — l'esito lo dà la cella Stato della riga 5 della posizione: se il verbale dice che gli annunci dei punti
3, 4 e 5 si sono sentiti,

```markdown
 ✅ **Chiusa il <data>**: `BaseStatus` dal compito 5 del [piano](../plans/2026-09-23-design-system.md), e il verbale dell'Assistente vocale nella riga 5 della sua tabella della posizione
```

e se uno non si è sentito, con la voce d'errata che il passo 8 del compito 5 ha aperto:

```markdown
 ⛔ **Ancora aperta il <data>**: la voce <Enn> dell'errata del [piano](../plans/2026-09-23-design-system.md)
```

La riga **N-2 di E187** — i due valori stanno in `riferimenti.md`, dove li scrive il Passo 8 col comando, e qui non si ricopiano (**E128**):

```markdown
 📌 **Misurato il <data>**: il pezzo JavaScript **cresce** — i due valori, del 2026-09-24 su `B` e del <data> su `HEAD`, stanno col comando in [`riferimenti.md`](../../riferimenti.md), la sezione del piano —: la deduzione qui a fianco, *«non la peggiora»*, **non regge**; resta del proprietario
```

E nel *«Come si riprende»* del disegno, in coda alla cella della riga **codice di prodotto**, prima di ` |` che chiude la riga:

```markdown
 ✅ **RICHIAMO DEL <data>:** toccato dal piano, i compiti 1–8 — che cosa, file per file, lo dice la Definizione di «fatto» del [piano](../plans/2026-09-23-design-system.md); e, prima del piano, `CLAUDE.md` e `scripts/check-docs.sh`, dal ridimensionamento della lettura del 2026-09-23 e dalla compressione del compendio del 2026-09-24 — `git log --format='%h %ad' --date=short ab39f39..d10d9a5 -- CLAUDE.md scripts/check-docs.sh`
```

Nella [stella polare](../specs/2026-09-07-direzione-gui-design.md), in coda alla cella *Atteso* della riga **codice e spec non
toccati**, prima di ` |` che chiude la riga — dopo avere provato che il solo file di `scripts/` toccato dal piano è quello:
`git diff --name-only "$B"..HEAD -- scripts/` → `scripts/gate-gui.sh`:

```markdown
 ✅ **RICHIAMO DEL <data>, compito 9 del piano del design system:** e `scripts/gate-gui.sh`, dai compiti 2 e 4 di quel [piano](../plans/2026-09-23-design-system.md) — le prove in due corse, un progetto alla volta, col commento che dice perché (E10 di quel piano), e la pagina kit fuori dal pacchetto, con `dist/` vuotato prima del *build* (E31)
```

Sonde: `grep -c 'RICHIAMO DEL <data>, compito 9 del piano del design system' docs/superpowers/specs/2026-09-07-direzione-gui-design.md`
→ **1**, e `grep -c 'dal compito 9 del \[piano\]' docs/superpowers/specs/2026-09-22-design-system-design.md` → **1**, con la data
vera al posto di `<data>`.

- [ ] **Passo 11: la Definizione di «fatto» — ogni riga eseguita, e l'uscita vista accanto**

⛔ **Lo scrive il coordinatore nella chiusura** (**E120**), dopo la revisione e le sue cure, sull'albero che resta:
l'implementatore esegue i blocchi e ne riporta le uscite nel **rapporto**, e non tocca il piano.

La sezione *«La Definizione di «fatto»»* qui sotto si **esegue** blocco per blocco, sull'albero di lavoro con i Passi 2–10
applicati (**D20**). Accanto a ogni comando, al posto dell'attesa, si scrive **l'uscita vista**; il titolo diventa
*«… — i comandi, con le uscite del <data>»*, e il capoverso in testa dice che le attese sono diventate uscite. ⛔ **Un'uscita che
diverge dall'attesa è una voce d'errata**, col comando, le due uscite e il perché: non si scrive la nuova come se fosse l'attesa,
e non si chiude il compito finché la voce non ha un esito. ⚠️ Dove l'uscita è un conto, resta **accanto al proprio comando**
(vincolo 10): chi rilancia confronta due uscite, non una cifra con la memoria.

- [ ] **Passo 12: il piano — la testa, la posizione, la chiusura**

⛔ **Lo fa il coordinatore nella chiusura** (**E120**): l'implementatore lo salta.

**Uno.** Il capoverso di *«A che punto è QUESTO PIANO»* che comincia con `✅ **IL PIANO È SCRITTO`, **preso dal file**, diventa:

```markdown
✅ **IL PIANO È ESEGUITO, il <data>.** A dirlo non è questa riga ma la tabella qui sotto — ogni riga ✅, e la colonna **Commit**
piena — e la **Definizione di «fatto»**, coi comandi e le uscite del giorno. Il passo dopo lo dice la §6 del compendio.
```

**Due.** Nella tabella: la riga **9**, **Stato** `✅ <data>`; la riga **8** porta già la colonna **Commit** — `ac56b11`, con
le cure `77c4062` e `7f04cdf`, scritta dalla chiusura del compito 8 (**E101**) — e **non si tocca** (**E115**); e nella riga **9** la colonna **Commit** con il comando, perché il commit che la scrive non può portare il proprio hash:
`` `git log -1 --format=%h --grep='^design-system(compito 9):'` ``.

**Tre.** *«Come si riprende»* si riscrive con la chiusura del piano — ciò che resta aperto e dove vive: le voci che il piano sa e
non chiude, le voci d'errata senza esito, il giudizio del controllo 15 se è del proprietario, le proposte per `CLAUDE.md` del
Passo 9 (**E116**), la CI del commit da leggere per prima —; la chiusura di prima va **parola per parola** in coda a
`docs/archivio/consegna-piano-design-system.md`, nella forma delle sezioni che vi stanno — un'intestazione con la data, il
capoverso *«Tolto dal piano il …»*, i rimandi riscritti per quella cartella. ⛔ **Parola per parola lo dice `word_for_word.py`**
del Passo 6 (**E117**), col testo di prima preso dal piano del commit precedente:
`git show HEAD:docs/superpowers/plans/2026-09-23-design-system.md | awk '/^## Come si riprende/{s=1} s' > <scratchpad>/old-resume.txt`,
poi `python <scratchpad>/word_for_word.py docs/archivio/consegna-piano-design-system.md <scratchpad>/old-resume.txt` → **0**.

- [ ] **Passo 13: il cancello, il margine, i fine-riga, il commit**

```bash
bash scripts/check-docs.sh 2>&1 | tail -1
bash scripts/gate.sh 2>&1 | tail -1
wc -c docs/COMPENDIO.md; grep -n '^ceiling=' scripts/check-docs.sh
git ls-files --eol docs/tracciabilita.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/archivio/stato-storico.md \
    docs/porta-di-qualita.md docs/riferimenti.md docs/HANDOFF.md docs/superpowers/specs/2026-09-22-design-system-design.md \
    docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/plans/2026-09-23-design-system.md \
    docs/archivio/consegna-piano-design-system.md
git diff --stat
git diff --stat -- crates/ gui/ scripts/ .github/ docs/adr/
git diff -- docs/ ':!docs/superpowers/plans/' | grep '^+' | grep -cE '<data>|<prima>|<dopo>|<Enn>|<i due valori'
git status --porcelain | diff <scratchpad>/prima.txt -
```

Atteso: `OK — no inconsistencies.`, `GATE GREEN.`, il margine **positivo**, i fine-riga **identici** al Passo 1; il
`git diff --stat` nomina **solo** i file della lista *Files* — ⛔ se ne nomina altri, lo zelo ha vinto: si revoca ciò che questo
piano non ha reso falso —; il secondo `--stat` **vuoto**; **0** segnaposto — `<data>`, `<prima>`, `<dopo>`, `<Enn>` — scritti fuori da questo piano, dove i
compiti li dettano;
e il confronto con lo stato di prima rende solo i file della lista.

```bash
git add docs/
git commit -m "design-system(compito 9): la chiusura -- il sotto-progetto 14 nei documenti, la riga Accessibilita' col richiamo, le case dei documenti"
```

⛔ **Senza co-autore** (vincolo 15). ⛔ **Il piano e l'archivio delle consegne non stanno in questo commit**, e il push è
del coordinatore, dopo la revisione (**E120**). ⛔ **La CI si legge**, coi due comandi di `docs/porta-di-qualita.md`, *«Leggere la CI da
terra»*: i due job del commit — `gate (ubuntu-latest)` e `gate (windows-latest)` — vanno nel blocco 5 della Definizione di «fatto»
alla sessione dopo, che li legge **per prima**.

---

## La Definizione di «fatto» — i comandi, e le uscite attese

⛔ **Comandi, non affermazioni** (D74 della parte 2; **D20** qui). Scritta il **2026-09-24**, con le **attese** che i compiti 1–8
dettano; il Passo 11 del compito 9 la esegue sull'albero del proprio commit e scrive accanto a ogni comando **l'uscita vista**, con
la data. ⛔ **Un'uscita che diverge dall'attesa è una voce d'errata**, non un'attesa nuova. Dove un compito dopo ha cambiato ciò che
uno prima misurava, vale lo stato **finale**, e la riga nomina chi l'ha cambiato. Un conto sta **accanto al proprio comando**
(vincolo 10): chi rilancia confronta due uscite, non una cifra con la memoria.

**La base.** `B` è il genitore del **primo** commit del compito 1 — i messaggi cominciano con `design-system(compito N):`
(vincolo 15):

```bash
B=$(git log --format=%H --grep='^design-system(compito 1):' | tail -1)^
git rev-parse --short "$B"; git log --oneline "$B"..HEAD | wc -l    # l'hash, e i commit del piano
```

**Blocco 1 — il cancello e i documenti.** Una corsa sola del cancello, e i `grep` sul suo log:

```bash
bash scripts/gate.sh > <scratchpad>/gate-dod.log 2>&1; tail -1 <scratchpad>/gate-dod.log   # GATE GREEN.
grep -c 'gui: fake core and SPA' <scratchpad>/gate-dod.log                                   # 1: il passo web, e dentro il browser
grep -E 'assets/index-.*\.js ' <scratchpad>/gate-dod.log                                     # il pezzo JavaScript: la cifra per il proprietario (N-2 di E187, R3-25)
bash scripts/check-docs.sh 2>&1 | tail -1                                                    # OK — no inconsistencies.
```

**Blocco 2 — la suite, file per file, in cinque corse** (**D23**). L'aiutante vive nello scratchpad, **mai** nel repository,
come `replace_unique.py`:

```python
"""dod_suite.py -- the suite's JSON reports, file by file (design system, task 9: the Definition of Done).

Usage: python dod_suite.py <report.json> [<report.json> ...]

Prints, for the FIRST report, one line per test file -- passed, failed, skipped -- named from `src/`; then, for
every report, its totals. Exits 1 when any report has a failed test or `success` false: a fall is an errata
entry, not a run to repeat until it passes (P-19, P-20, P-21 of the plan). The paths come in as ARGUMENTS, so
Git Bash turns `/c/...` into `C:/...` for Python -- a path written inside `python -c "..."` is not converted.
"""
import json
import os
import sys

if len(sys.argv) < 2:
    sys.exit("usage: python dod_suite.py <report.json> [<report.json> ...]")
bad = False
for i, path in enumerate(sys.argv[1:]):
    with open(path, encoding="utf-8") as f:
        d = json.load(f)
    if not d["testResults"]:
        sys.exit(f"{path}: no test file at all -- a report that saw nothing proves nothing")
    if i == 0:
        for r in sorted(d["testResults"], key=lambda r: r["name"]):
            name = r["name"].replace("\\", "/")
            name = name[name.find("/src/") + 1:] if "/src/" in name else name
            s = [a["status"] for a in r["assertionResults"]]
            p, x = s.count("passed"), s.count("failed")
            print(f"{name}: {p} passed, {x} failed, {len(s) - p - x} skipped")
    print(f"{os.path.basename(path)}: {d['numTotalTests']} tests, {d['numPassedTests']} passed, "
          f"{d['numFailedTests']} failed, success={d['success']}")
    bad = bad or d["numFailedTests"] > 0 or not d["success"]
sys.exit(1 if bad else 0)
```

Provato il 2026-09-24 nelle due direzioni e sul vuoto, sul rapporto del `main` di quel giorno (P-30): esce **0** sul rapporto
vero, **1** con un rapporto a cui è stata messa a mano una caduta, **1** su un rapporto senza file.

```bash
for i in 1 2 3 4 5; do (cd gui && npx vitest run --reporter=json --outputFile=<scratchpad>/dod-$i.json > /dev/null 2>&1); done
python <scratchpad>/dod_suite.py <scratchpad>/dod-1.json <scratchpad>/dod-2.json <scratchpad>/dod-3.json <scratchpad>/dod-4.json <scratchpad>/dod-5.json; echo $?
python <scratchpad>/dod_suite.py <scratchpad>/dod-1.json | grep -c ' skipped$'; git ls-files 'gui/src/*.test.ts' | wc -l
```

Attese: una riga per file di prova, **nessuna** con un `failed` diverso da zero — fra loro i dodici file nuovi del piano:
`tokens/board.test.ts`, `tokens/usage.test.ts`, `tokens/theme.test.ts` (compito 1), `tokens/tokens.browser.test.ts` (2),
`components/kit.test.ts` (3), `kit/kit.browser.test.ts` (4), `panels/settings.browser.test.ts` (5), `tokens/dock.test.ts` e
`frame/dock.browser.test.ts` (6), `frame/nearest.test.ts` e `frame/schematic.test.ts` (7), `frame/frame.browser.test.ts` (6bis, e l'8 lo estende: **D29**, **E64**);
cinque totali **uguali** fra loro, `0 failed` e `success=True`; l'uscita **0**; e le ultime due cifre **uguali**: ogni file di
prova del repository è girato, e nessuno fuori dal repository.

**Blocco 3 — i controlli del disegno, uno per riga** — la tabella *«Il prodotto, e il controllo che esercita ciascun
artefatto»*. Dove il controllo è un file di prova, la sua riga del blocco 2 è la prova; qui il comando dice che l'artefatto c'è:

```bash
# 1 -- i valori dei token, copie della tavola (D1): board.test.ts nel blocco 2
ls gui/src/tokens/base.css gui/src/tokens/themes.css                                   # i due file
# 2, 3 -- il contrasto per famiglie, gli stessi ruoli nei due temi (P-1): contrast.test.ts nel blocco 2
# 4, 5 -- nessuna scala e nessun colore a mano fuori dai token: usage.test.ts nel blocco 2
grep -rln 'var(--ref-' gui/src | grep -v '^gui/src/tokens/' | wc -l                    # 0
# 6, 18 -- il tema e le viste col nome, campi facoltativi del pacchetto (D2, D3): stores.test.ts nel blocco 2
grep -cE '^  (theme|named|openNamed)\?: ' gui/src/stores/layout.ts                      # 3: le dichiarazioni di LayoutPack, non i cast di unpack (E111)
# 7 -- il tema sulla radice: theme.test.ts nel blocco 2
grep -c 'data-theme' gui/src/tokens/theme.ts                                            # almeno 1
# 8, 9, 16 -- il movimento ridotto, i caratteri, readToken: tokens.browser.test.ts nel blocco 2
# 10 -- la mappa delle icone, e lucide da un posto solo
grep -rlE "from ['\"]lucide" gui/src                                                    # gui/src/components/icons.ts, e nient'altro
# 11 -- i pezzi di base: kit.test.ts e kit.browser.test.ts nel blocco 2
ls gui/src/components/Base*.vue                                                         # i pezzi della tabella della (b), nome per nome: nessuno in più, nessuno in meno -- non una cifra (E64)
# 12 -- le regole del kit nel linter, coi loro messaggi
grep -c 'message: "' gui/eslint.config.js                                               # 6: i tre messaggi del compito 3 e i tre del 5 -- il 2026-09-24 erano 0
# 13 -- BaseStatus: la regione vuota c'è (kit.test.ts); il lettore di schermo vero, a mano
awk -F'|' '/^\| \*\*5\*\* \|/{print $5}' docs/superpowers/plans/2026-09-23-design-system.md   # ✅ <data>, col verbale dell'Assistente vocale
# 14 -- la pagina kit fuori dal pacchetto
grep -c 'the kit page is in the package' scripts/gate-gui.sh                           # 1
(cd gui && ls dist/kit.html 2>&1; grep -rl 'kit-card' dist/assets | wc -l)             # «No such file or directory», e 0 -- dopo il build del blocco 1
# 15 -- il tema del dock: dock.test.ts e dock.browser.test.ts nel blocco 2
grep -rn 'themeAbyss' gui/src | wc -l                                                   # 0
# 17 -- la Panoramica: a11y.test.ts, nearest.test.ts, frame.test.ts, frame.browser.test.ts nel blocco 2
grep -rl 'F3' gui/src/frame | wc -l                                                      # almeno 1
# 19 -- la striscia a pillola: frame.browser.test.ts nel blocco 2
# 20 -- le prove del browser, ciascuna con la sua guardia
for f in $(git ls-files 'gui/src/*.browser.test.ts'); do printf '%s ' "$f"; grep -cE 'NON-VACUITY|toBeGreaterThan\(0' "$f"; done   # sei file, ciascuno almeno 1
# 21 -- la riga «Accessibilità»
grep -c '^| Accessibilità | 🔶 |' docs/tracciabilita.md                                 # 1
# 22 -- la barra di scorrimento: tokens.browser.test.ts nel blocco 2, con le barre mostrate al progetto browser (D26)
grep -c 'ignoreDefaultArgs' gui/vite.config.ts                                          # 1
# 23 -- BaseNotice: kit.test.ts, kit.browser.test.ts, frame.test.ts, modules.test.ts, frame.browser.test.ts nel blocco 2
grep -l 'BaseNotice' gui/src/frame/Band.vue gui/src/panels/Settings.vue gui/src/panels/Status.vue | wc -l   # 3
```

⚠️ **Il 20 non si prova con un `grep`:** un commento o una guardia trovati dicono che la guardia **è scritta**, non che morda. Che
morda l'hanno provato le due direzioni di ciascun compito, al loro giorno — e il passo **rosso** se il browser non parte, il
passo 5 del compito 2 (controllo 20). Il `grep` qui dice solo che nessun file del browser è nato **senza**: un file con **0** si
legge, e o la guardia ha un'altra forma, o manca — e allora è una voce d'errata.

**Blocco 4 — i vincoli globali e il perimetro:**

```bash
git diff --stat "$B"..HEAD -- crates/ gui/schema/                                       # niente (vincolo 12)
git ls-files gui/src/tokens/tokens.css                                                  # niente: esce al compito 1
grep -E '"(lucide|@fontsource-variable/geist|@fontsource/barlow|@vitest/browser-playwright|playwright|reka-ui|vitest)"' gui/package.json   # 1.47.0, 5.3.0, 5.3.0, 4.1.11, 1.63.0, 2.10.4, 4.1.11 -- versioni esatte (vincoli 7 e 8)
wc -c docs/COMPENDIO.md; grep -n '^ceiling=' scripts/check-docs.sh                     # il margine positivo (vincolo 13)
git diff --name-status "$B"..HEAD -- docs/adr/ | grep -c '^A'                            # 0: nessun ADR nuovo
ls docs/adr/*.md | wc -l; grep -c '^\*\*00' docs/COMPENDIO.md                          # uguali fra loro
git diff --name-only "$B"..HEAD                                                          # ogni nome in una lista Files dei compiti 1–9, in una voce d'errata, o nella cartella del dispaccio (E112)
git status --porcelain                                                                   # niente, dopo il commit del compito (E123)
```

⚠️ **Il perimetro si legge nome per nome, non col numero** (R9a-16 ed E217 della parte 2): ogni nome che il penultimo comando
elenca sta in una lista *Files* — per il percorso, o per la cartella che la lista nomina, come gli **undici** componenti del
compito 1, e mai per una riga che **vieta** di toccarla (**E126**) —, in una voce d'errata, o nella cartella del dispaccio, `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`,
la regola 8 di *«Come si esegue un compito»* (**E112**). Un nome senza casa è una voce d'errata nuova.

**Blocco 5 — ciò che un comando non dice**, e dove si guarda:

- ⛔ **l'aspetto si GUARDA**, nei due temi: i passi «guardarlo» dei compiti e la regola 5 di *«Come si esegue un compito»*; e il
  **giudizio del proprietario sul dock** (controllo 15), con la data e dove è scritto — o, se non era scritto, la sua risposta
  alla domanda del compito 9 (**D24**);
- ⛔ **M-3 col lettore di schermo vero**: il verbale nella cella Stato della riga 5 della posizione, e l'esito nella (e) del
  disegno (Passo 10 del compito 9);
- ⛔ **l'assunzione della barra di scorrimento**, sotto i colori forzati e con la preferenza di Windows: misurata al pre-controllo
  del compito 9 (**E118**), l'esito nella riga dell'assunzione del disegno;
- ⛔ **la CI del commit del compito 9**: i due job, `gate (ubuntu-latest)` e `gate (windows-latest)`, letti coi due comandi di
  `docs/porta-di-qualita.md`, *«Leggere la CI da terra»* — la casa unica di quei comandi, che qui non si ricopiano;
- ⚠️ **le voci che questo piano sa e non chiude** restano nella loro tabella, in testa al piano, col loro chiusore.

---

## Come si riprende — il pre-controllo del compito 9, 2026-09-28

✅ **Il pre-controllo del compito 9 è fatto: dieci voci, E111…E120**, scritte nell'errata e applicate al testo. **E118** è la
domanda del proprietario — **A**, la barra di scorrimento misurata —; le altre le ha decise il coordinatore coi cinque
criteri, fuori dal merito approvato. Il compito rifatto dal testo del piano su un `git worktree` di `f35d3c2` in
`%TEMP%\pc9`, macchina `zagor`: i Passi 2–8 e 10 applicati da uno script, nessun rifiuto, ogni sonda dei passi come detta,
`check-docs.sh` verde, i fine-riga conservati — CR uguali alle righe in ogni file —, nessuna tabella spezzata in più; e la
Definizione di «fatto» eseguita sull'albero a `f35d3c2`: blocco 1 `GATE GREEN`, blocco 2 cinque corse uguali — `237 tests,
236 passed, 0 failed`, ventotto file su ventotto —, blocchi 3 e 4 coi difetti di **E111** ed **E112**. La riga 9 della
posizione resta a `⬜`, pre-controllata. La consegna precedente — l'esecuzione del compito 8 — sta parola per parola in
[`archivio/consegna-piano-design-system.md`](../../archivio/consegna-piano-design-system.md).

⛔ **Da sapere subito.**

1. **Chi scrive il piano** (**E120**): l'implementatore fa i Passi 1–10 e il commit del Passo 13 **senza** il piano né
   l'archivio delle consegne, ed **esegue** la Definizione di «fatto» riportandone le uscite nel rapporto; i Passi 11 e 12 —
   le uscite nel piano, la testa, la riga 9, la chiusura del piano e la consegna in archivio — li fa il coordinatore nella
   chiusura, dopo la revisione e le sue cure; il push è suo. Il compito **non ha uno sguardo** del proprietario.
2. **Il Passo 9 legge anche le lezioni** (**E116**): le tabelle *«Ciò che questa sessione ha imparato»* delle consegne,
   ~31 kB; una trappola diventa un gotcha, una regola di lavoro una **proposta al proprietario** per `CLAUDE.md`.
3. **Il giudizio sul dock** (controllo 15, **D24**) **è scritto**: le celle 6 e 6bis della posizione portano lo sguardo del
   proprietario, e la domanda del Passo 1 non serve.
4. **La barra di scorrimento** (**E118**): sotto i colori forzati regge; la preferenza di Windows che tiene sempre visibili
   le barre non arriva alla pagina — il costo della C, ora misurato —, e la B resta **registrata, non presa**.
5. **In `%TEMP%` della macchina `Jays` restano le cartelle di prova di sessioni passate** — `pc2`…`pc6c`, `rv4`, `rv5`,
   `rv6` —: toglierle è del proprietario; `ls -d /c/Users/Jays/AppData/Local/Temp/pc* /c/Users/Jays/AppData/Local/Temp/rv*`
   le elenca. Su `zagor` la copia di questa sessione, `%TEMP%\pc9`, è tolta.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin` dopo il push: `git fetch --all --prune`, poi `git status -sb` |
| **i commit** | `git log --oneline f35d3c2..HEAD`: uno, quello che scrive questa riga — l'errata, il compito 9 corretto, il richiamo di **E118** nel disegno, questa sezione e la precedente in archivio, i tre file del dispaccio. Nessun codice cambia: `git diff --stat f35d3c2..HEAD -- gui/ crates/ scripts/` non rende nulla |
| **cancello** | `GATE GREEN` all'apertura su `f35d3c2` — jsdom 170 prove passate e una saltata, browser 66, il pezzo `698.20 kB`, compresso `213.00 kB`, `found 0 vulnerabilities` — e prima del commit che scrive questa riga. Si rilancia, non si cita: `bash scripts/gate.sh`, **da solo** |
| **la CI** | `f35d3c2` verde su `ubuntu-latest` e `windows-latest`; quella del commit che scrive questa riga la legge per prima la sessione dopo, coi comandi di [`porta-di-qualita.md`](../../porta-di-qualita.md), *«Leggere la CI da terra»* |
| **la posizione** | le righe dalla 1 alla 8 a `✅`; la 9, pre-controllata, a `⬜` |
| **il dispaccio** | nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: `_extract_brief_9.py`, il modello `dispatch-task-9.md`, e `compare_task9.py`, che ricostruisce dal testo del piano i nove documenti dettati e li confronta col commit — provato in due direzioni su una copia |
| **lo scratchpad** | sulla macchina `zagor`, **non tracciato**: in `.superpowers/sdd/2026-09-23-design-system/` gli attrezzi del pre-controllo — `_precheck_9_perimeter.py`, `_precheck_9_apply.py`, `_precheck_9_write.py`, `word_for_word.py` —; nello scratchpad della sessione i cinque rapporti JSON della suite, le foto della barra e i log. Non servono a riprendere: il piano basta |
| **server e cartelle di prova** | nessun server acceso; `git worktree list` rende la sola cartella del repository |

**Che cosa ha fatto questa sessione.** La ripresa coi comandi — trentadue commit arrivati, il fast-forward —, la CI di `f35d3c2`
e il cancello d'apertura; la Definizione di «fatto» eseguita blocco per blocco sull'albero, le cinque corse comprese; il
perimetro nome per nome con uno script; il compito rifatto dal testo su una copia, con ogni sonda; la domanda del
proprietario sulla barra, **A**, e la misura: i colori forzati emulati nel Chrome installato, e la preferenza di Windows
guardata dal proprietario su una pagina di prova; le dieci voci; il dispaccio, con `compare_task9.py` provato nelle due
direzioni; la chiusura.

📌 **Ciò che questa sessione ha imparato, e che non era scritto** — le raccoglie la chiusura del sotto-progetto (**E116**).

| | Che cosa | Che cosa se ne fa |
|---|---|---|
| 1 | **`check-docs.sh` non vede un rimando rotto dentro il testo**: una riscrittura che perde la `)` lascia un rimando che porta ancora a un file vero, e il cancello resta verde | un testo archiviato si prova **parola per parola**, con le destinazioni tolte dalle due parti (**E117**) |
| 2 | **una promessa scritta in una consegna è un contratto per il compito che nomina**: ventidue consegne dicevano *«le raccoglie la chiusura»*, e il compito della chiusura leggeva la sola errata | il pre-controllo di un compito cerca nelle consegne ciò che gli è stato promesso, non solo nell'errata |
| 3 | **in `awk`, `print x > 0` scrive un file che si chiama `0`**: il confronto dentro un `print` è una redirezione | un confronto si scrive fra parentesi, `print (x > 0)`, o in una condizione |
| 4 | **la preferenza di Windows che tiene visibili le barre non arriva a una pagina con `::-webkit-scrollbar`**, e una pagina di prova aperta come file non carica i moduli di Vite, e mostra le barre di Chrome | una pagina di prova coi token si serve da Vite, mai da un file |

**Il prossimo passo** — ciascuno nella sua sessione (`CLAUDE.md`):

1. `git fetch --all --prune`, `git status -sb`; la CI del commit che scrive questa riga, per prima.
2. **L'esecuzione del compito 9**: il brief da `_extract_brief_9.py`, il prompt dal modello `dispatch-task-9.md`, col sì del
   proprietario sulla banda dei costi detta prima — un compito di **soli documenti**, senza sguardo, che fa girare il
   cancello e cinque corse della suite: 🔶 dedotta dalle bande dei compiti 7 e 8, 0,6–0,9 milioni per implementatore e
   revisore —; la revisione con `compare_task9.py`, le cure; poi la chiusura del coordinatore coi Passi 11 e 12 (**E120**),
   e il push. Dopo, la §6 del compendio punta al sotto-progetto 13.
