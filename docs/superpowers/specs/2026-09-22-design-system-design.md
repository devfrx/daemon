# Il design system della GUI — il disegno

✅ **QUESTO DISEGNO È COMPLETO DAL 2026-09-23.** Il brainstorming è durato quattro sessioni, tutte il 2026-09-23: ventidue
risposte del proprietario, la strada — **A**, le variabili CSS sono la verità — e sei sezioni approvate una per volta, in chat:
**(a)** i token, **(b)** il kit, **(c)** il dock, **(d)** la cornice, **(e)** le voci registrate, **(f)** le sonde che
diventano test. La quinta sessione dello stesso giorno lo ha scritto **sul posto**, dal diario. Chi riprende ha un disegno
intero da tradurre in un **piano**, **dopo** che il proprietario lo ha riletto in questa forma: *«Come si riprende»*, in fondo.

⚠️ **RICHIAMO DEL 2026-09-23, alla scrittura del disegno:** questo file è nato il 2026-09-22 come **consegna dell'avvio**
(`2a674cc`) ed è diventato il **diario** del brainstorming, aggiornato e committato a ogni risposta. È riscritto **allo stesso
percorso**, perché il puntatore della §6 del [compendio](../../COMPENDIO.md) non cambi casa — com'è stato per i disegni dei
[gesti](2026-09-03-riconoscimento-gesti-design.md), della [knowledge base](2026-09-04-knowledge-base-design.md) e del
[sotto-progetto 2](2026-09-06-sottoprogetto-2-gui-minima-design.md). Il diario com'era sta **parola per parola** in
[`archivio/consegna-avvio-brainstorming-design-system.md`](../../archivio/consegna-avvio-brainstorming-design-system.md), coi
soli link riscritti per la cartella: lì stanno le **parole testuali** del proprietario, i fatti di ogni risposta col giorno in
cui sono stati letti, le decisioni del metodo del brainstorming e le trappole delle tavole. Ciò che la scrittura ha
**misurato** in più sta in *«Cosa questo disegno ha misurato»*.

⚠️ **Non è una spec.** Come i disegni dei gesti, della knowledge base e del 2, fissa il **perimetro**, le **forme** e, per ogni
artefatto, **il controllo che lo esercita**. ⛔ **I valori non sono qui:** colori, caratteri, spazi, raggi, misure, movimento e
livelli stanno **solo** nella [tavola dei token](2026-09-22-design-system-tavole/token.html), che contiene per intero
`base.css` e `themes.css`; qui stanno le **regole** che li producono e i **nomi**. ⛔ **E la forma della GUI non è qui:**
viste, moduli, disposizione e protocollo core ↔ GUI vivono nella [stella polare](2026-09-07-direzione-gui-design.md), strati e
cartelle nella §6a del [disegno del 2](2026-09-06-sottoprogetto-2-gui-minima-design.md). Questo disegno le **veste**, ne
allarga due punti — la barra delle viste e il pacchetto della disposizione — e **rimanda**, non ricopia.

⚠️ **Come è stato approvato.** Le risposte 1–5 con le parole del proprietario, la 11 nata da una sua richiesta scritta a metà
turno, la 15 con un mandato; tutte le altre con un clic sul consiglio. Questa sessione l'ha aperta il proprietario con
`anthropic-skills:decision-principles`: se scrivendo il piano una sezione viola un criterio — una scorciatoia, una
duplicazione, un fatto che non è più vero — ci si **ferma** e lo si dice, non si esegue.

📌 **Metodo.** Ogni affermazione porta la sua specie — **verificata** (letta nel codice o in un documento, con la data),
🔶 **dedotta**, o **assunta** — e i comandi stanno accanto alle affermazioni e **si rilanciano**, non si citano. Il codice non
è cambiato dalla nascita del diario: `git log --oneline 2a674cc..HEAD -- . ':!docs'` non rende nulla, rilanciato il
2026-09-23. I fatti del codice che il disegno usa sono stati **rilanciati** comunque, e dove il diario sbagliava lo dice
*«Cosa questo disegno ha misurato»*: il merito approvato non è stato toccato.

## Le risposte del proprietario, una per domanda

In breve; il testo intero, con le sue parole e i fatti del giorno, è nella
[consegna in archivio](../../archivio/consegna-avvio-brainstorming-design-system.md).

| # | Che cosa | La risposta |
|---|---|---|
| 1 | che cosa vuol dire *«da Agentic OS»*, e quanto è grande il lavoro | **A, nella sostanza** — letta dal coordinatore e detta come tale: le **fondamenta** più i componenti che **oggi** hanno la seconda occorrenza; ciò che verrà nasce quando arriva, ereditando o creando. Sei criteri: il perimetro, qui sotto |
| 2 | l'idea di stile, a parole | niente **olografico** e niente **vetro**; **due accenti primari**, off-white e **bordeaux**; **angoli morbidi**; un'immagine come **spunto**, non come modello — non è nel repository |
| 3 | la direzione, fra due tavole | **A «Strumento», con le schede morbide della B** |
| 4 | la tavola unita, nei due temi, col kit accanto | **sì**, con la nota sui **raggi concentrici**: *raggio di fuori = raggio di dentro + distanza*, nei token per costruzione |
| 5 | i temi | **A** — due subito, scuro e chiaro, che di base seguono Windows e si scelgono a mano; la domanda sul dock cade |
| 6 | i caratteri | **B** — due caratteri aperti dentro il programma, niente rete a runtime |
| 7 | la stella polare durante il brainstorming | **B** — a pezzi, col `grep`, e per intero una volta prima di scrivere il disegno: l'ha letta la terza sessione, prima degli approcci |
| 8 | la coppia di caratteri | **A — Geist + Barlow** |
| 9 | le icone | **B** — un set aperto, usato da un solo componente |
| 10 | il set | **A — Lucide** |
| 11 | la richiesta del proprietario: *«un componente centralizzato con tutto il set delle icone mappate»*; e che cosa vuol dire «tutto il set» | **A** — un componente e una mappa sola, con le icone che usiamo e i nomi nostri |
| 12 | dove si guardano i componenti | **A** — una **pagina «kit»** dentro l'app, solo in sviluppo |
| 13 | come si cambia vista | **B — la Panoramica** |
| 14 | la strada | **A** — le variabili CSS sono la verità |
| 15 | la prima metà della (a), coi valori della tavola | **un mandato:** *«usa tutte le regole di ui/ux e professionalità che conosci per i token ed i loro valori»* |
| 16 | la (a) rifatta, e dove si conserva il tema | **A** — la (a) com'è nella tavola dei token, e il tema nel pacchetto della disposizione |
| 17 | la (b), il kit | **A** — sette pezzi di base adesso |
| 18 | la (c), il dock | **A** — un tema `dockview` nostro, coi gruppi come schede |
| 19 | la (d), le miniature della Panoramica | **A** — schemi disegnati dalla disposizione salvata |
| 20 | la (d), la striscia e gli angoli della finestra | **A** — la pillola si alza: 12 px dai lati, 24 dal fondo |
| 21 | la (e), la riga «Accessibilità» della tracciabilità | **A** — da ✅ a 🔶, nel piano |
| 22 | la (f), un browser vero nel cancello | **A** — Vitest in modalità browser con Playwright, sul Chrome installato |

## Il perimetro — che cosa fa questo lavoro, e che cosa no

### Che cosa vuol dire «da Agentic OS»

**«Agentic OS» non è un prodotto da imitare:** è il nome che il proprietario dà al **programma intero** — la decisione 3 del
[disegno della chiusura del 1](2026-09-02-sottoprogetto-1-chiusura-design.md) e la decisione 7 della stella polare, la chat
fuori dalla Home. La richiesta è del 2026-09-21, punto 5 della trentasettesima chiusura del
[piano della parte 2](../plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md): lo stile di oggi **non gli piace**, e vuole un
design system a sotto-progetto 2 chiuso, col suo brainstorming. La risposta 1 ne fissa sei criteri:

| # | Criterio | Dove lo prende questo disegno |
|---|---|---|
| 1 | lo stile rispecchia **prima di tutto l'idea del proprietario** e ciò che il progetto ha già scritto | il linguaggio visivo, qui sotto — risposte 2–4 — e la Home approvata della stella polare |
| 2 | **professionale, curato nel minimo dettaglio** | la (a), coi valori costruiti dalle regole del mestiere verificate alle fonti; la (f), con le sonde che diventano test |
| 3 | **temi e token centralizzati** | la (a) |
| 4 | **tutti** i componenti, riutilizzabili e no, ordinati logicamente e architetturalmente | la (b) |
| 5 | le **viste si compongono di componenti** | le regole del linter della (b); la presa grande della (c); la Panoramica della (d) |
| 6 | **modularità** | i pezzi di base che non leggono lo stato globale, e la mappa unica delle icone — la (b) |

🔶 **Dedotto, non smentito:** *«da Agentic OS»* vuol dire un aspetto all'altezza di un sistema operativo per agenti — *«un misto
fra Jarvis e Claude Desktop»*, parole del proprietario del 2026-09-07 che dal diario vivono ora nella consegna in archivio — e
non la copia di un prodotto. L'aspetto che la stella polare aspettava, il «Jarvis», lo danno **colori e forme**: la sua tabella
dei wireframe li rimandava *«al design system»*, e la condizione su `dockview` — *«se provandola non dà il "Jarvis", si passa
alla tela libera»* — l'ha chiusa SP-8, con `dockview` che resta dopo le otto mosse (ADR-0029).

### Che cosa fa — la A della domanda 1

| | Che cosa | Sezione |
|---|---|---|
| 1 | il **linguaggio visivo** — lo stile approvato, coi caratteri e le icone scelti | qui sotto |
| 2 | i **token a strati** e i **due temi** | (a) |
| 3 | il **kit** dei pezzi di base: i componenti che oggi hanno la seconda occorrenza, e la pagina «kit» dove si guardano | (b) |
| 4 | il **dock vestito** coi nostri token: `themeAbyss` esce | (c) |
| 5 | la **cornice**: la barra col nome della vista, la Panoramica, la striscia, e le regole della finestra per il guscio | (d) |
| 6 | le **voci registrate** che il lavoro incrocia | (e) |
| 7 | le **sonde che diventano test**, anche nel browser vero | (f) |

### Che cosa esclude, e chi lo prende

| Escluso | Perché, e chi lo prende |
|---|---|
| il **kit dei pilastri** — viewer 3D, diff, pannello della mappa | nasce col suo pilastro, ereditando o creando: la A della domanda 1, e il terzo dei tre momenti del design system (risposta 3 della [consegna d'avvio del 2](../../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md)) |
| il **nucleo della Home** — l'anello degli artefatti e la rete viva | nasce coi moduli che lo riempiono, come la stella polare li assegna; qui se ne fissa lo **stile**, e la [tavola dello stile](2026-09-22-design-system-tavole/stile-approvato.html) ne è il bersaglio |
| il **guscio vero** — la finestra di Electron, il pacchetto | del sotto-progetto 10; la (d) fissa le **regole** della finestra che il guscio userà |
| **progress e notifiche** per i lavori lunghi — E228 | del proprietario: chi le costruisce, il 3 o il 7 — la (e) |
| il **taglio dei chunk** di `vite` — N-2 di E187 | del proprietario, fuori da questo lavoro — la (e) |
| un **terzo carattere** per il codice, come Geist Mono | una dipendenza nuova, del proprietario: **registrata, non presa** (decisione 16) |
| gli **«spazi»**, le viste una accanto all'altra come i desktop di Windows | 🔶 si possono aggiungere sopra la Panoramica più avanti senza rifare niente (risposta 13) |

### Il linguaggio visivo — lo stile approvato

Le risposte 2, 3 e 4, poi la 8 e la 10 per caratteri e icone. Lo si **guarda** nella
[tavola dello stile](2026-09-22-design-system-tavole/stile-approvato.html), che porta la coppia di caratteri e le icone scelte;
com'era alla risposta 4 lo rende `git show 8c0bbe6:docs/superpowers/specs/2026-09-22-design-system-tavole/stile-approvato.html`.

| Tratto | Che cosa vuol dire |
|---|---|
| **niente olografico, niente vetro** | nessuna trasparenza da vetro, nessuna luce finta: fondi pieni, linee sottili |
| **due accenti primari** | l'**off-white** e il **bordeaux**, col **carbone**: sono le **àncore** della (a), approvate guardandole. Un solo accento forte, usato poco e per ciò che conta — i numeri chiave, il «prossimo», le azioni |
| il **linguaggio della A, «Strumento»** | etichette in **maiuscolo spaziato** con un segno bordeaux e un'icona piccola; **numeri grandi e leggeri** con l'etichetta piccola sotto; la barra a segmenti; la riga «prossima» accesa; **divisori sottili** al posto delle scatole pesanti |
| **dentro schede morbide**, come la B | gli angoli arrotondati; nello scuro un bordo, nel chiaro un'ombra leggera — la (a) |
| il **bordeaux nello scuro** | sul carbone non basta né per un testo né per un segno sottile (contrasto WCAG): lì è un **fondo pieno** col testo off-white sopra, e i testi bordeaux diventano un **rosa antico** — la (a) |
| i **raggi concentrici** | *raggio di fuori = raggio di dentro + distanza*, per costruzione nei token — `--radius-card` e `--radius-frame` sono `calc` — così, se cambia un margine, i raggi lo seguono da soli. Vale per **ogni** annidamento, con la distanza vera: un foglio con 24 di margine attorno a schede da 20 vuole 44. Un elemento che non può stare in un angolo con lo stesso centro **si allontana dall'angolo** |
| il **movimento** | breve e sobrio, a zero con «meno movimento» — la (a) |

Nel repository le tavole approvate sono **quattro**, in [`2026-09-22-design-system-tavole/`](2026-09-22-design-system-tavole/):
lo **stile** (risposte 4, 8, 10), la **Panoramica** (13), i **token** (16) e gli **angoli della finestra** (20); con **tre**
sonde — [`sonda-raggi.js`](2026-09-22-design-system-tavole/sonda-raggi.js),
[`sonda-caratteri.js`](2026-09-22-design-system-tavole/sonda-caratteri.js) e
[`sonda-icone.js`](2026-09-22-design-system-tavole/sonda-icone.js) — che la (f) fa diventare test. ⚠️ L'immagine di
riferimento della risposta 2 **non** è nel repository: è il lavoro di un altro, e il repository è pubblico (decisione 7).

### La strada — le variabili CSS sono la verità

La risposta 14, **A**: i token a strati in `gui/src/tokens/`, il kit in `gui/src/components/`, le regole su **tre livelli** —
il compilatore (i nomi come tipi), il linter, i test. I tre controlli della decisione 18 della stella polare, detti a parole
prima della domanda:

| | |
|---|---|
| **esiste** | `tokens.css`, un file e un tema; dodici componenti, tutti con `<style scoped>` su `var(--…)`; lo strato `components/` della §6a del disegno del 2, oggi con `Confirm.vue`; `contrast.test.ts`, che legge i colori dal CSS invece di ricopiarli; la tavola approvata, già a variabili CSS coi raggi in `calc`; in ESLint `no-restricted-imports` e `vue/no-restricted-html-elements`, senza dipendenze nuove; i test in jsdom, senza layout |
| **arriva** | il grafo del 6, il viewer del 7 e la mano del 12 disegnano su **canvas** e vogliono i colori anche in TypeScript: li legge l'aiutante `readToken` della (c) |
| **regge crescendo** | un componente entra nel kit alla seconda occorrenza, un'icona è una riga della mappa; 🔶 il canvas legge il colore calcolato con l'aiutante, e passare più avanti a una sorgente TypeScript non toccherebbe i componenti, che usano `var(--…)` in tutte e due le strade |

Le strade scartate — la sorgente TypeScript con un generatore, Tailwind — stanno in *«Vicoli ciechi e scelte scartate»*.

## (a) I token — ✅ approvata il 2026-09-23, risposte 15 e 16, con la 5, la 6 e la 8

⛔ **I valori stanno solo nella [tavola dei token](2026-09-22-design-system-tavole/token.html)**, fra i due commenti
`===== proposta/base.css =====` e `===== the board itself, only tokens =====`: il piano li **copia** da lì, e non li riscrive. La
tavola si guarda nel browser; la pagina kit, a grandezza vera, è dove si giudicano le misure assolute — le grandezze, la
spaziatura delle etichette, il movimento (risposta 16).

### I file

| File | Che cosa tiene |
|---|---|
| `gui/src/tokens/base.css` | ciò che **non** cambia col tema: caratteri, spazi, raggi, misure, icone, focus, movimento, livelli |
| `gui/src/tokens/themes.css` | le **scale** `--ref-*` su `:root`, e i **ruoli** `--color-*` e `--shadow-*` sotto `[data-theme="dark"]` e `[data-theme="light"]` |
| `gui/src/tokens/dock.css` | le variabili `--dv-*` di `dockview` legate ai **nostri** ruoli, e le regole della presa grande e di `.panel` — la (c) |
| `gui/src/tokens/tokens.css` | ⛔ **esce**. Oggi è un file solo, un tema solo, coi valori **segnaposto dichiarati** in testa (*«THESE ARE PLACEHOLDER VALUES, and saying so is the point»*); quante variabili porta lo dice `grep -c '^  --' gui/src/tokens/tokens.css`, e l'ultima, `--dv-…`, è di `dockview` |

### Due livelli: le scale e i ruoli

| Livello | Nome | Che cosa |
|---|---|---|
| **scale** | `--ref-<tavolozza>-<tono>` | cinque tavolozze tonali — un grigio caldo dal carbone all'avorio (`neutral`), `bordeaux`, `green`, `amber`, `red` — coi gradini chiamati col **tono**: la L* CIE, che Material chiama *tone* |
| **ruoli** | `--color-<proprietà>-<ruolo>-<stato>`, `--shadow-<ruolo>` | l'ordine è **proprietà, ruolo, stato** — `--color-bg-fill-hover`, `--color-text-muted`, `--color-border-strong` — con gli **stessi nomi nei due temi** |

⛔ **Nessun componente legge una scala**: i componenti leggono i ruoli, e le scale esistono solo per i ruoli. I nomi mettono la
**proprietà prima** (decisione 14): un testo scritto con un fondo si vede dal nome, e il test del contrasto accoppia **ogni**
`--color-text-*` con **ogni** `--color-bg-*` senza una lista a mano — la regola già scritta in `contrast.test.ts`: ogni
coppia, e non una lista tenuta a mano delle coppie che i componenti usano. Quanti ruoli porta ciascun tema lo dice il comando,
sui due temi:

```bash
awk '/^\[data-theme="dark"\] \{/{s=1} s&&/^\}/{exit} s' docs/superpowers/specs/2026-09-22-design-system-tavole/token.html | grep -cE -- '--(color|shadow)-[a-z-]+:'
awk '/^\[data-theme="light"\] \{/{s=1} s&&/^\}/{exit} s' docs/superpowers/specs/2026-09-22-design-system-tavole/token.html | grep -cE -- '--(color|shadow)-[a-z-]+:'
```

⚠️ La risposta 16 scriveva un numero, e contava solo i ruoli presi da una scala: *«Cosa questo disegno ha misurato»*.

### I colori

| Regola | Da dove |
|---|---|
| i colori della tavola approvata sono **àncore esatte**; gli altri gradini si generano al **loro** tono, con tinta e croma interpolate in **OKLCH**; le tinte tenui si **mescolano** nel grigio caldo in **OKLab**, come `color-mix(in oklab, …)` | decisione 15: il proprietario ha approvato quei colori guardandoli, e una scala rigenerata da zero li avrebbe spostati tutti di poco |
| ogni ruolo di **testo** a 4,5:1 su **ogni** fondo, nei due temi; il testo sul bordeaux e sugli stati a 4,5:1 | WCAG 2.2, 1.4.3 |
| **bordo forte, focus e segni** a 3:1 sui fondi | WCAG 2.2, 1.4.11 |
| `--color-text-disabled` è **esente**, e `--color-border` è **decoro** | 1.4.3 esenta i componenti inattivi; un bordo che non serve a riconoscere un controllo non è un segno |
| gli **stati** — sopra, premuto, tenue — sono **colori espliciti**, non strati trasparenti | il test del contrasto sa giudicare un colore, non una trasparenza sopra un fondo che non conosce; gli strati di Material non sono presi |
| il **velo** sotto i dialoghi è un ruolo, `--color-veil` | i due veli di oggi, scritti a mano e già diversi — `rgb(0 0 0 / 50%)` in `Confirm.vue` e `rgb(0 0 0 / 0.45)` in `Drawer.vue` — sono i soli colori fuori dai token: il ruolo li cura |

Le coppie e i loro rapporti li calcola la tavola, che li scrive in chiaro; il test li **ricalcola** leggendo `themes.css` — la (f).
Che cosa è cambiato rispetto alla tavola dello stile — la grandezza vera, i raggi, il focus dello scuro, il rosa antico — fu
detto al proprietario **prima** della risposta 16, ed è scritto lì, in archivio.

### Il resto: `base.css`

| Gruppo | I nomi | La regola |
|---|---|---|
| **caratteri** | `--font-family-text` (Geist), `--font-family-tool` (Barlow), `--font-family-mono`; i pesi `--font-weight-*`; le scorciatoie `font`: `--font-label`, `--font-caption`, `--font-body`, `--font-body-strong`, `--font-title`, `--font-heading`, `--font-numeric`, `--font-display`, `--font-mono` | la scala dei caratteri di Carbon; il minimo è `label-small` di Material, 11/16; l'etichetta in **maiuscolo**, i numeri con le **cifre tabulari** |
| **spazi** | `--space-0-5` … `--space-16` | la griglia da 4, gli spazi di Carbon |
| **raggi** | `--radius-inline`, `--radius-control`, `--radius-card`, `--radius-frame`, `--radius-full` | `card` e `frame` sono **`calc`**: la regola dei raggi della risposta 4; `full` per le pillole — dentro una pillola va una pillola |
| **misure** | `--size-target-min`, `--size-control-sm`/`md`/`lg` | **nulla di cliccabile sotto 24 × 24** (2.5.8); la presa grande è `lg` |
| **icone** | `--size-icon-sm`/`md`/`lg`, `--icon-stroke` | tre misure e un tratto solo |
| **focus** | `--focus-width`, `--focus-offset` | un **contorno** di almeno 2 px a 3:1 (2.4.13), visibile (2.4.7) e non nascosto (2.4.11) |
| **movimento** | `--duration-fast`/`moderate`/`slow`, `--ease-standard`/`enter`/`exit` | le durate e le curve «productive» di Carbon; **a zero** con `prefers-reduced-motion: reduce` (2.3.3) |
| **livelli** | `--z-floating`, `--z-popover`, `--z-overlay`, `--z-toast`, `--z-tooltip` | `--z-floating` è nato con la (c), sotto menu, dialoghi e avvisi — decisione 23 |

### I due temi

| | |
|---|---|
| **quali** | **scuro e chiaro, subito** (risposta 5) |
| **di base** | **seguono Windows**: in Electron `nativeTheme.themeSource` vale di base `system`, e la query CSS `prefers-color-scheme` della pagina lo segue — la pagina di `nativeTheme`, letta alla fonte il 2026-09-23 |
| **a mano** | nelle **Impostazioni**, con `BaseRadioGroup`: sistema, chiaro, scuro. La scelta **non** la tiene la GUI (I1): è un campo **`theme` facoltativo** del `LayoutPack` di `gui/src/stores/layout.ts`, accanto a `view` e `layouts` (risposta 16). Il pacchetto è **opaco al core** — `{ state: "Package"; bytes: number[] }` in `gui/src/schema/messages.ts`, e la riga 1 della §2 della stella polare — quindi il core lo custodisce senza aprirlo, il filo non cambia e il **kernel non cambia**. È fuori dal registro delle funzioni come la disposizione: decisione 14 della stella polare |
| **dove si posa** | come nella tavola, un attributo **`data-theme`** sulla radice; con `system`, o senza il campo, lo decide `prefers-color-scheme` e segue il suo cambio. 🔶 È la forma più corta coerente con le risposte 5 e 16, e la scrive il piano |
| **un pacchetto vecchio** | senza `theme` si apre come `system`: un campo **facoltativo** non rompe ciò che l'ha preceduto |
| **non sono temi** | l'**alto contrasto** di Windows — `inForcedColorsMode` e la query `forced-colors` — e la **riduzione del movimento** sono regole di G20, **sempre** rispettate (risposta 5). ⚠️ La tavola porta la regola del movimento e non una per l'alto contrasto: *«Le trappole»* |
| **chi altro segue il tema** | lo `colorScheme` del dock — la (c) — e i colori dei pulsanti della finestra, `setTitleBarOverlay` — la (d) |

### I caratteri

| | |
|---|---|
| **la coppia** | **Geist** per il testo, **Barlow** per etichette, numeri e orari (risposte 6 e 8): `@fontsource-variable/geist` e `@fontsource/barlow`, licenza OFL-1.1, **dentro il programma** e senza rete a runtime — due dipendenze nuove, approvate dal proprietario |
| **i pesi** | di Barlow **quattro**: 300 per i numeri grandi, 400, 500 per il pulsante della striscia, 600 per le etichette; Geist è variabile, un file per sottoinsieme. Nel programma va **solo** ciò che i token importano: il peso spacchettato dei pacchetti è un limite superiore |
| **perché questa coppia** | **misurata** con [`sonda-caratteri.js`](2026-09-22-design-system-tavole/sonda-caratteri.js): con Geist e Barlow le lettere occupano quasi lo stesso spazio dei caratteri di Windows della tavola approvata — i rapporti alla risposta 8 |
| **il monospazio** | quello del sistema — `ui-monospace`, Cascadia Mono, Consolas — decisione 16 |

### Il passaggio dai nomi di oggi

I nomi di oggi — `--ink`, `--ink-dim`, `--surface`, `--surface-raised`, `--line`, `--accent`, `--warn`, `--stop`, `--radius`,
`--font`, `--font-size`, `--line-height` — passano ai **ruoli** (decisione 14), e il piano ne scrive la **tabella** vecchio →
nuovo prima di rinominare. Gli spazi `--space-1` … `--space-4` tengono **nome e valore**. Quanti usi ci sono da rinominare lo
dice il terzo comando del censimento, in *«Verificato, dedotto, assunto»*. ⚠️ Il rinomino è meccanico **tranne** dove un token
porta un significato: *«Le trappole»*, la provenienza della chat.

## (b) Il kit — ✅ approvata il 2026-09-23, risposta 17, con la 9, la 10, la 11 e la 12; decisioni 18–20

### Dove vive, e la regola dei pezzi di base

| | |
|---|---|
| **la casa** | `gui/src/components/`, lo strato della §6a del [disegno del 2](2026-09-06-sottoprogetto-2-gui-minima-design.md) — oggi con `Confirm.vue` e `markdown.ts` |
| **i pezzi di base** | portano il prefisso **`Base`**; dentro, solo elementi HTML, altri pezzi di base e componenti UI di terzi; **mai lo stato globale**, *«e.g. from a Pinia store»*. È la regola *Base Component Names* della guida di stile di Vue, *strongly recommended*, letta il 2026-09-23 |
| **quando un pezzo entra** | alla **seconda occorrenza**, non alla prima: la regola dei tre momenti, già approvata col disegno del 2. I pezzi qui sotto sono contati coi comandi del censimento, in *«Verificato, dedotto, assunto»* |

### I pezzi di base — otto

Sette dalla risposta 17, e `BaseTextField` dalla (d), decisione 20.

| Pezzo | Che cosa | La seconda occorrenza | Sopra |
|---|---|---|---|
| **`BaseButton`** | il pulsante: variante, misura, spento | cinque `<button` in quattro file — `Confirm.vue` due, `Band.vue`, `ViewBar.vue`, `Placeholder.vue` — più i due comandi della presa grande in `frame/BigTab.ts` | HTML |
| **`BaseIcon`**, con la mappa `icons.ts` | l'icona, **una sola porta** per tutte | la risposta 11: ogni icona del programma passa di qui | i disegni di `lucide` |
| **`BaseDialog`** | la finestra modale col velo | `Confirm.vue` e `Drawer.vue`, coi due veli a mano e diversi, curati da `--color-veil` | `Dialog` di `reka-ui` |
| **`BaseList`** | la lista | il cassetto, Permessi e Passi — `Drawer.vue`, `Permissions.vue`, `Steps.vue` | HTML |
| **`BaseStatus`** | la regione di stato | le tre `role="status"`: `Band.vue`, `Settings.vue`, `Status.vue` | HTML |
| **`BaseLabel`** | l'etichetta in maiuscolo con l'icona | i titoli di Permessi e la presa grande | HTML |
| **`BaseRadioGroup`** | la scelta fra poche opzioni | la policy VRAM delle Impostazioni e la scelta del tema della (a) | `RadioGroup` di `reka-ui` |
| **`BaseTextField`** | il campo di testo | il nome di una vista salvata e la ricerca della barra — la (d) | HTML |

**Fuori, finché non tornano due volte:** i messaggi e le notifiche, e le pillole. `Confirm.vue` resta un pezzo **composto** dai
pezzi di base, e `markdown.ts` resta com'è.

### La forma di un pezzo di base

- `<script setup lang="ts">`, con i **props tipizzati** — variante, misura, spento — e gli **slot**;
- dialog e radio **sopra `reka-ui`**, che dà tastiera e ARIA (domanda 7 del disegno del 2, risposta **B**);
- stile `scoped` coi **soli token**, e gli stati con le pseudo-classi e con gli attributi `data-state` di `reka-ui`;
- **nessuna scritta dentro**: le parole le porta chi lo usa, da `gui/src/locales/it.json` con `vue-i18n`;
- **un test con `axe`** per ciascuno.

### `BaseIcon` e la mappa

| | |
|---|---|
| **l'uso** | `<BaseIcon name="search" />`, e il nome è un **tipo**: un nome sbagliato non compila |
| **la mappa** | `gui/src/components/icons.ts`: **nome nostro → icona di Lucide**, importate **una per una** dal pacchetto `lucide`, quello dei **soli disegni** e non quello per Vue — nello spirito di ADR-0030, che preferisce le librerie agnostiche. In `lucide` 1.47.0 un'icona è un array come `["circle", { cx, cy, r }]`, letto in `dist/esm/icons/search.mjs` il 2026-09-23 |
| **il disegno** | con `h()` di Vue, **senza `v-html`** |
| **la regola** | **nessun'altra parte del codice importa icone**: cambiare set tocca la sola mappa |
| **il costo** | circa mezzo kB a icona; il set intero, la B della risposta 11, costava centinaia di kB in più — le misure alla risposta 11, in archivio |

### `BaseStatus`

Esiste **sempre** nel DOM, vuota e alta zero, e il testo le **entra dentro**: così un lettore di schermo la annuncia, e non
nasce una scatola vuota — la regola *«no empty box»* della §6a del disegno del 2. Chiude **per costruzione** la voce **M-3** di
E187: la (e).

### I pezzi che non si riusano

Stanno in `frame/` e in `panels/`, vicino a chi li usa. Un modulo che cresce tiene i suoi in una **cartella sua**. ⚠️ La risposta
17 dava come esempio `panels/chat/`, che **non esiste**: *«Cosa questo disegno ha misurato»*. La regola resta, e si applicherà
per la prima volta quando un modulo crescerà.

### Le regole, come controlli del linter che c'è già

| Regola | Il controllo |
|---|---|
| in `components/` **nessun import** di `pinia`, `stores/`, `panels/`, `frame/`, `transport/` | `no-restricted-imports` di ESLint, limitato a `components/` |
| `lucide` importato **solo** da `icons.ts` | `no-restricted-imports` |
| in `panels/` e in `frame/` **nessun `<button>`** e **nessuna lista** scritta a mano | `vue/no-restricted-html-elements`, presente in `eslint-plugin-vue` 10.11.0 — letto nel pacchetto installato |
| in `panels/` e in `frame/` **nessun import di `reka-ui`** | `no-restricted-imports` |

Ogni regola si prova nelle **due direzioni**: una violazione messa a mano la fa scattare, e il codice giusto no. ⚠️ Il linter dei
template **non vede** un `document.createElement("button")` in un file `.ts`: *«Le trappole»*.

### La pagina kit

| | |
|---|---|
| **che cos'è** | `gui/kit.html` con `gui/src/kit/`: **ogni** componente in **ogni** stato, nei due temi, a grandezza vera. Solo in sviluppo (risposta 12) |
| **perché regge** | Vite serve ogni pagina HTML in sviluppo, e al *build* prende solo gli ingressi elencati — la guida *Building for Production*, *Multi-Page App*, letta il 2026-09-23. Oggi la pagina è una sola, `gui/index.html`, e `gui/vite.config.ts` non elenca ingressi |
| **che resti fuori dal pacchetto** | **si prova nel piano**, sull'uscita del *build* |
| **a che cosa serve** | è dove si **guardano** i componenti e dove girano i test nel browser della (f) |

## (c) Il dock — ✅ approvata il 2026-09-23, risposta 18

Oggi nella stessa finestra convivono **due linguaggi**: `themeAbyss` di `dockview`, quello su cui il proprietario giudicò le otto
mosse di SP-8, e i nostri token, che vestono solo ciò che disegniamo noi. Con due temi `themeAbyss`, che è solo scuro, non può
restare (risposta 5, e la decisione 8: la domanda sul dock non è stata posta).

| | |
|---|---|
| **il tema** | `themeAbyss` **esce** da `frame/dock.ts`, dove oggi sta sulla riga `theme: themeAbyss`; entra un `DockviewTheme` **nostro**: `className: "dockview-theme-harness"`, `colorScheme` che segue `data-theme`, `gap` letto dal token `--space-3`. Il tipo, in `dockview-core` 8.3.1, porta `name`, `className`, `colorScheme`, `gap`, `edgeGroupCollapsedSize`, `dndOverlayMounting`, `dndPanelOverlay` — letto in `gui/node_modules/dockview-core/dist/cjs/dockview/theme.d.ts` il 2026-09-23 |
| **le variabili** | `tokens/dock.css` dà a **ogni** variabile `--dv-*` che il CSS di `dockview` usa un **nostro ruolo**, senza colori a mano; quante sono lo dice `grep -o 'var(--dv-[a-z0-9-]*' gui/node_modules/dockview/dist/styles/dockview.css \| sort -u \| wc -l`. Le regole della presa grande e di `.panel`, oggi in `tokens.css`, passano lì |
| **i gruppi** | ogni gruppo è una **scheda** — `--color-bg-surface`, `--radius-card`, nello scuro il bordo `--color-border-card` e nel chiaro l'ombra `--shadow-card` — a 12 px dalle altre, sul fondo `--color-bg` |
| **i contenitori galleggianti** | i temi «spaced» di `dockview` scrivono a mano `border-radius: 8px` sui contenitori, `.dv-resize-container:has(> .dv-groupview)`, **sotto la loro classe**: sotto la nostra non vale, e il nostro tema ci scrive il **raggio dei token** — *«Cosa questo disegno ha misurato»* |
| **la presa grande** | alta 40 px, `--size-control-lg`: **`BaseLabel`** con l'icona del modulo, e i suoi due comandi sono **`BaseButton`** con **`BaseIcon`**. `frame/BigTab.ts` monta pezzi Vue come fa già `frame/VueContent.ts`. La linguetta attiva ha il testo pieno e il segno `--color-mark`, l'inattiva il testo `--color-text-muted` |
| **il trascinamento** | la zona d'arrivo è `--color-bg-accent-subtle` col bordo `--color-mark`; i divisori sono **invisibili** finché non ci si passa sopra; i gruppi galleggianti e le finestre staccate sono `--color-bg-raised` con `--shadow-overlay` |
| **i livelli** | `--dv-overlay-z-index` vale **999** nel CSS di `dockview`, sopra i nostri dialoghi a `--z-overlay`: nasce **`--z-floating`**, sotto menu, dialoghi e avvisi — un'aggiunta alla (a) detta al proprietario prima della risposta 18, e scritta ora nella tavola dei token (decisione 23) |
| **`readToken`** | l'aiutante che legge un token dal CSS calcolato, per chi lo vuole in TypeScript: il `gap` del dock oggi, il canvas del 6, del 7 e del 12 domani |
| **l'aspetto vero** | si vede alla **prima prova** del piano; il bersaglio è la Home della [tavola dello stile](2026-09-22-design-system-tavole/stile-approvato.html) |

## (d) La cornice — ✅ approvata il 2026-09-23, risposte 19 e 20, con la 13; decisioni 18–20

### La barra

Oggi `gui/src/frame/ViewBar.vue` porta **tre pulsanti in fila**, uno per vista — le schede che il 2026-09-07 il proprietario
disse di non volere — e il cassetto dei moduli, `<Drawer />`.

| Da sinistra | Che cosa |
|---|---|
| il **nome della vista** in cui sei, con la sua icona | un clic apre la **Panoramica** |
| la **ricerca** | oggi un `<input type="search">` **spento** che dice chi lo riempirà — la decisione 16 della stella polare, scritta accanto nel sorgente; diventa un `BaseTextField`, spento com'è |
| il **chip del core** | lo stato della connessione, com'è oggi |

Il pulsante **«Moduli»** scende nella **striscia**, dov'è nella tavola dello stile e nella [Panoramica](2026-09-22-design-system-tavole/panoramica.html).

### La Panoramica

| | |
|---|---|
| **che cos'è** | tutte le viste **in miniatura**, in una griglia: Home, Lavoro, **Compatta come una finestrella**, e le viste salvate col loro nome; la vista corrente in **bordeaux**; l'ultima scheda è **«Salva questa vista»**. Com'è nella [tavola](2026-09-22-design-system-tavole/panoramica.html) (risposta 13) |
| **le miniature** | **schemi** disegnati dalla disposizione salvata, coi moduli e le loro icone dove stanno: dicono sempre il vero, costano quasi zero e reggono con dieci viste (risposta 19) |
| **come si apre** | con **F3** o col clic sul nome della vista; le **frecce** muovono, **Invio** entra, **Esc** chiude. ⚠️ I tasti sono quelli della tavola: il piano li **controlla** contro quelli che esistono — `Ctrl+Alt+frecce` sono già dei pannelli, `directionOf` in `gui/src/frame/moveActive.ts` |
| **su che cosa è costruita** | su **`BaseDialog`**, a tutta finestra (decisione 18): `reka-ui` dà già Esc, il fuoco chiuso dentro e il fuoco che torna al nome della vista |
| **le frecce nella griglia** | seguono la **geometria**, nelle quattro direzioni — giù va alla riga sotto — con la geometria di `moveActive.ts` estratta in un **aiutante comune** (decisione 19): `RovingFocusGroup` di `reka-ui` 2.10.4 è **lineare**, e nel pacchetto installato `ArrowDown` vale `"next"` (`dist/RovingFocus/utils.js`). È la seconda occorrenza di quella geometria, e si estrae com'è la regola del kit |
| **«Salva questa vista»** | chiede il nome con un **`BaseTextField`** (decisione 20) |

### Le viste salvate col nome

Sono già della §2 della [stella polare](2026-09-07-direzione-gui-design.md) — *«le viste che il proprietario salva con un nome
(domanda 6)»*. Oggi `unpack` in `gui/src/stores/layout.ts` legge solo `home`, `work` e `compact`, e **butta** ogni altra voce
come scritta da un'altra build: è la riga 8 della §2, e resta com'è per le chiavi di `layouts`.

⛔ **Le viste col nome vanno in una lista loro nel pacchetto**, un campo nuovo del `LayoutPack` accanto a `view`, `layouts` e
`theme`: il core la custodisce senza aprirla, e il **kernel non cambia** (risposta 19). Un pacchetto scritto prima, senza la
lista, si apre come prima.

### La finestra

Le regole che la (d) fissa per il **guscio del 10**, che resta suo:

| | |
|---|---|
| **la barra fa da barra del titolo** | `titleBarStyle: 'hidden'` con `titleBarOverlay`; la finestra si trascina dalla barra con `app-region: drag`; lo spazio libero lo danno `env(titlebar-area-x, 0px)` e `env(titlebar-area-width, 100%)` |
| **i pulsanti di Windows** | in alto a destra, coi **colori del tema**: `setTitleBarOverlay({ color, symbolColor, height })` segue il tema, su Windows e su Linux — la guida *Custom Title Bar* di Electron, esempi alla 44.4.5, e l'API di `BrowserWindow`, lette il 2026-09-23 |
| **gli angoli** | li disegna **Windows**: 8 px, e **0** quando la finestra è ingrandita o agganciata — Microsoft Learn, *Geometry in Windows 11* e *Apply rounded corners in desktop apps*. Non sono nostri, e **niente di nostro con un raggio gli sta vicino**: in alto gli angoli sono della barra, piatta; in basso la striscia ne sta lontana |
| **la striscia** | resta una **pillola**, e si alza: a **12 px** dai lati, allineata alle schede, e a **24 px** dal fondo (risposta 20). Costa 12 px d'altezza; la regola dei raggi regge con l'angolo di Windows a 8 e con quello dritto — la [tavola degli angoli](2026-09-22-design-system-tavole/angoli-finestra.html) |

🔶 **Dedotto:** che la finestra di Electron con la barra nascosta prenda gli angoli di Windows lo dice il guscio; se restasse
dritta, la striscia regge lo stesso. ⚠️ La sonda dei raggi **non confronta** un angolo dritto — cerca l'antenato col raggio
maggiore di zero — quindi a finestra ingrandita non ha niente da dire.

## (e) Le voci registrate — ✅ approvata il 2026-09-23, risposta 21; decisione 21

Quattro voci aperte dal [piano della parte 2](../plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md) incrociano questo
lavoro; il loro testo intero sta nell'errata di quel piano.

| Voce | Che cosa decide questo disegno | Chi la chiude |
|---|---|---|
| **N-2 di E235** — in `docs/tracciabilita.md` la riga «Accessibilità» è ✅, e la legenda dice ✅ *«**meccanismo deciso** nel kernel: la funzionalità ha già le sue fondamenta»*, mentre le fondamenta stanno nella GUI | **A:** cambia la **riga**, non la legenda — da ✅ a 🔶, *«meccanismo deciso, **politica o implementazione** nel sotto-progetto indicato»*: il 2 e il design system. «UI/UX della chat» è già 🔶 per la stessa ragione, la GUI sul core finto. Cambiare la legenda avrebbe confuso tutte le righe ✅ | il **piano**, col richiamo datato |
| **M-3 di E187** — le regioni `role="status"` nascono nel DOM **insieme** al loro testo, e molti lettori di schermo non le annunciano | chiusa **per costruzione** da `BaseStatus`, la (b). Le regioni sono **tre**, non due: `gui/src/frame/Band.vue`, `gui/src/panels/Settings.vue` e `gui/src/panels/Status.vue`, tutte con `v-if` — `grep -rn 'role="status"' gui/src --include=*.vue` | il **piano**; e la prova col **lettore di schermo vero** — l'Assistente vocale di Windows — è un passo del piano, **a mano** (decisione 21) |
| **N-2 di E187** — l'avviso di `vite` sui chunk sopra i 500 kB | il design system **non** la peggiora: un'icona di `lucide` pesa circa mezzo kB, e 🔶 i caratteri sono **file a parte** — dedotto da come Vite tratta gli `url()` dei CSS, e si vede al primo *build* del piano. Il pezzo JavaScript di oggi lo dice il log del cancello, con `grep -n 'kB' <log>` | il **proprietario**, fuori da questo lavoro |
| **E228** — *«Progress e notifiche per job lunghi»*, orfana in `tracciabilita.md` | il kit ha già i **token** — `--z-toast` e i colori di stato — e la regola della seconda occorrenza | il **proprietario**: chi le costruisce, il 3 o il 7 |

## (f) Le sonde che diventano test — ✅ approvata il 2026-09-23, risposta 22; decisione 22

`CLAUDE.md` vuole che *«gli invarianti diventano test»*, e la regola dei raggi è saltata **più volte** sulle tavole: la prima
l'ha vista il proprietario a occhio, alla risposta 4; le altre le sonde — le scatole del companion, i difetti della prima
tavola dei token, la striscia della risposta 20. Quindi le tre sonde, e le prove fatte in linea nelle sessioni, diventano
**test**, su due livelli.

| Dove | Che cosa | Da che cosa nasce |
|---|---|---|
| **senza browser**, nel `vitest` di oggi sotto jsdom | il **contrasto** di `gui/src/tokens/contrast.test.ts` allargato a **tutte** le coppie dei token, nei due temi, letti da `themes.css` e non ricopiati | la regola della (a) |
| | la **mappa delle icone**: il nome è un tipo, e ogni voce disegna un'icona | la (b) |
| | le **regole del kit**, nel linter | la (b) |
| **nel browser vero**, sulla pagina kit e nei due temi | i **raggi concentrici** | `sonda-raggi.js` |
| | il **testo tagliato** e **niente che sborda** | le prove in linea delle sessioni |
| | le **icone centrate** | `sonda-icone.js` |
| | i **caratteri caricati**, con le **cifre tabulari** | `sonda-caratteri.js` |
| | **`axe`**, che nel browser sa giudicare il contrasto della pagina disegnata, mentre sotto jsdom lo lascia incompleto | `gui/src/a11y.test.ts` |

⛔ **Ogni prova del browser porta la guardia di non-vacuità** — quante cose ha guardato, **maggiore di zero** — perché una sonda
che non trova niente è **verde**: è successo due volte nella quarta sessione, e con `near: 0` nella terza. ⛔ E
`sonda-raggi.js` diventa un test con le **radici come parametro** — la registrata della terza sessione, perché su una pagina
nuova non renda un verde vuoto — e un angolo **dritto** non lo confronta (risposta 20).

**Il browser dei test:**

| | |
|---|---|
| **le dipendenze** | `@vitest/browser-playwright` **4.1.11**, MIT, che vuole `vitest` 4.1.11 e un `playwright` qualsiasi e porta `@vitest/browser` 4.1.11, MIT; e `playwright` **1.63.0**, Apache-2.0, **senza** script d'installazione — non scarica niente da solo — con `playwright-core` 1.63.0. Solo di sviluppo: due dipendenze nuove, approvate dal proprietario alla risposta 22 |
| **il browser** | il **Chrome installato**, canale `chrome` (decisione 22): la configurazione `playwright({ launchOptions: { channel } })` della pagina *Configuring Playwright* di Vitest; i canali `chrome` e `msedge` usano il browser stabile già installato, senza scaricare — la pagina *Browsers* di Playwright |
| **dove c'è** | su questa macchina, `C:\Program Files\Google\Chrome\Application\chrome.exe`, mentre Edge **non** sta nel percorso solito; sulle due immagini della CI — `ubuntu-latest` e `windows-latest` — ci sono Chrome ed Edge, dai README di `actions/runner-images`. 🔶 Sull'altra macchina serve Chrome, o `npx playwright install chromium`: si verifica là |
| **dove gira** | **nel cancello** — la domanda della risposta 22 era *«un browser vero nel cancello»* — cioè nel passo web, `scripts/gate-gui.sh`; come ci entra lo scrive il piano |
| **`vitest`** | resta alla **4.1.11** del lockfile: al registro c'è la 5.0.1, e salire è un'altra decisione |

## Il prodotto, e il controllo che esercita ciascun artefatto

Un principio che non si può controllare è un'intenzione (`CLAUDE.md`). La colonna *«Da»* dice se il controllo l'ha **approvato**
il proprietario in una sezione, o se lo **propone** chi ha scritto il disegno perché la sezione approvava la regola senza dire
come si controlla: le proposte si **rileggono** col disegno, e il piano ne sceglie la forma.

| # | Artefatto | Dove | Il controllo | Da |
|---|---|---|---|---|
| 1 | i **valori** dei token | `gui/src/tokens/base.css`, `themes.css` | i due blocchi della [tavola dei token](2026-09-22-design-system-tavole/token.html) copiati **byte per byte**, e un `diff` contro la tavola che non rende nulla | proposta |
| 2 | il **contrasto** | `themes.css` | `contrast.test.ts` allargato: ogni `--color-text-*` su ogni `--color-bg-*` a 4,5:1 in ciascun tema; bordo forte, focus e segni a 3:1; le due esenzioni scritte accanto | (f) |
| 3 | gli **stessi ruoli nei due temi** | `themes.css` | un test che confronta i due insiemi di nomi, letti dal file | proposta |
| 4 | **nessun componente legge una scala** | `gui/src/**` fuori da `tokens/` | un test che non trova `var(--ref-` fuori da `gui/src/tokens/`, provato anche rosso | proposta |
| 5 | **nessun colore a mano** fuori dai token | `gui/src/**/*.vue` e `tokens/dock.css` | il secondo comando del censimento diventa un test: nessun `#…`, `rgb(`, `hsl(` fuori dai file dei token — oggi rende i due veli | proposta |
| 6 | la **scelta del tema** | `LayoutPack.theme` in `gui/src/stores/layout.ts` | un test del negozio nelle due direzioni: un pacchetto col tema torna uguale; uno **senza** si apre come `system` | proposta |
| 7 | il **tema sulla radice** | l'attributo `data-theme` | un test con `matchMedia` finto: `system` segue il sistema e il suo cambio; `light` e `dark` vincono | proposta |
| 8 | il **movimento ridotto** | `base.css` | le durate a zero sotto `prefers-reduced-motion: reduce`, nel browser, dove la query si emula | proposta |
| 9 | i **caratteri** | `@fontsource-variable/geist`, `@fontsource/barlow` | nel browser: caricati — lo stato dei `FontFace` e non `document.fonts.check()` — e con le cifre tabulari | (f) |
| 10 | **`BaseIcon`** e la mappa | `components/BaseIcon.vue`, `components/icons.ts` | il nome è un **tipo** (il compilatore, `vue-tsc` nel cancello); ogni voce disegna un'icona (jsdom); `lucide` solo da `icons.ts` (linter); le icone centrate (browser) | (b), (f) |
| 11 | i **pezzi di base** | `gui/src/components/Base*.vue` | un test con `axe` ciascuno; nel browser, sulla pagina kit, raggi, testo tagliato e niente che sborda | (b), (f) |
| 12 | le **regole del kit** | `gui/eslint.config.js` | le quattro regole della (b), ciascuna provata **nelle due direzioni** con una violazione messa a mano | (b) |
| 13 | **`BaseStatus`** | `components/BaseStatus.vue` | un test che la monta **vuota** e trova la regione; poi l'Assistente vocale, a mano, col verbale di ciò che si è sentito | (b), decisione 21 |
| 14 | la **pagina kit** | `gui/kit.html`, `gui/src/kit/` | nel pacchetto **non** c'è: un controllo sull'uscita del *build* | (b) |
| 15 | il **tema del dock** | `frame/dock.ts`, `tokens/dock.css` | `themeAbyss` non compare più nel sorgente; `dock.css` passa il controllo 5; l'aspetto lo giudica il proprietario alla prima prova | (c) |
| 16 | **`readToken`** | un aiutante nuovo | un test che legge un token noto | proposta |
| 17 | la **Panoramica** | `frame/`, su `BaseDialog` | un test con `axe`; l'aiutante della geometria provato coi **rettangoli dati a mano**, come `frame/keys.test.ts`, perché sotto jsdom ogni rettangolo è zero; i tasti contro quelli che esistono | (d), decisione 19 |
| 18 | le **viste col nome** | un campo del `LayoutPack` | un test del negozio nelle due direzioni: la lista torna uguale; un pacchetto **senza** la lista si apre come prima; una chiave sconosciuta dentro `layouts` resta buttata, come dice la riga 8 della §2 | proposta |
| 19 | la **striscia** | la cornice | nel browser, a finestra piena: la pillola a 12 e 24 px, e nessuna scheda vicina a un angolo della pagina | proposta |
| 20 | le **prove del browser** | la configurazione di `vitest` per il browser | ciascuna con la **guardia di non-vacuità**; il passo nel cancello rosso se il browser non parte, non verde | (f) |
| 21 | la **riga «Accessibilità»** | `docs/tracciabilita.md` | 🔶 col richiamo datato; il comando del riquadro in testa al file si rilancia prima e dopo | (e) |

## Verificato, dedotto, assunto

### Verificato nel codice, il 2026-09-23

Il codice è quello della nascita del diario — `git log --oneline 2a674cc..HEAD -- . ':!docs'` non rende nulla — e i fatti sono
stati **rilanciati** lo stesso, da due script nello scratchpad della sessione. I tre comandi del **censimento** portano backslash,
quindi si lanciano **da un file**, non in linea: in linea i backslash spariscono in silenzio e la misura mente — la trappola della
quarantatreesima chiusura del piano della parte 2.

```bash
grep -rl --include='*.vue' -e '<button' gui/src
grep -rn --include='*.vue' -E '#[0-9a-fA-F]{3,8}\b|rgba?\(|hsla?\(' gui/src
grep -rhoE 'var\(--[a-z0-9-]+\)' gui/src --include=*.vue --include=*.css --include=*.ts | sort | uniq -c | sort -rn
```

| Fatto | Il comando |
|---|---|
| `<button` in quattro file: `Confirm.vue`, `Band.vue`, `ViewBar.vue`, `Placeholder.vue` | il primo del censimento |
| i soli colori a mano sono i **due veli**, in `Confirm.vue` e in `Drawer.vue` | il secondo |
| gli usi dei token di oggi, per nome | il terzo |
| dodici componenti con `<style scoped>`, più `App.vue` che non ne ha | `find gui/src -name '*.vue'` e `grep -rL '<style scoped' gui/src --include=*.vue` |
| `reka-ui` è importato solo da `Confirm.vue` e `Drawer.vue` | `grep -rln 'from "reka-ui"' gui/src` |
| le versioni appuntate — `reka-ui` 2.10.4, `dockview-core` 8.3.1, `vite` 8.3.0, `vitest` 4.1.11, `eslint-plugin-vue` 10.11.0 | `gui/package.json` |
| il tema del dock è `themeAbyss` | `grep -n 'theme' gui/src/frame/dock.ts` |
| le liste in `Drawer.vue`, `Permissions.vue`, `Steps.vue` | `grep -rln '<ul\|<ol' gui/src --include=*.vue`, da un file |
| le tre `role="status"`, tutte con `v-if` | `grep -rn 'role="status"' gui/src --include=*.vue` |
| `ViewName` è `"home" \| "work" \| "compact"`, e `unpack` butta il resto | `grep -n 'ViewName\|unpack' gui/src/stores/layout.ts`, da un file |
| il pacchetto è **opaco** al core | `grep -n 'LayoutState' gui/src/schema/messages.ts` |
| la barra: tre pulsanti in fila, la ricerca spenta, il chip, `<Drawer />` | `gui/src/frame/ViewBar.vue`, letto per intero |
| `directionOf` è `Ctrl+Alt+frecce` | `grep -n 'directionOf' gui/src/frame/moveActive.ts` |
| la presa grande crea i suoi `<button>` con `document.createElement` | `grep -n 'createElement' gui/src/frame/BigTab.ts` |
| la policy VRAM è un gruppo di **radio nativi**, con tre sottigliezze scritte accanto | `grep -n 'radio' gui/src/panels/Settings.vue` |
| oggi nessun tema, nessun movimento ridotto, nessun alto contrasto in `gui/src` | `grep -rn 'data-theme\|prefers-color-scheme\|prefers-reduced-motion\|forced-colors' gui/src`, da un file, non rende nulla |
| le cartelle di `gui/src` — nessuna `panels/chat/`; `panels/views/` tiene le disposizioni delle tre viste | `find gui/src -type d` |
| `DockviewTheme`, i suoi sette campi | `gui/node_modules/dockview-core/dist/cjs/dockview/theme.d.ts` |
| `--dv-overlay-z-index` vale 999, e i raggi a mano dei temi «spaced» | `grep -o -- '--dv-overlay-z-index:[^;]*' gui/node_modules/dockview/dist/styles/dockview.css`; `grep -n -B3 'border-radius: 8px' gui/node_modules/dockview/dist/styles/dockview.css` |
| `ArrowDown` è `"next"` in `RovingFocusGroup` | `grep -n 'ArrowDown' gui/node_modules/reka-ui/dist/RovingFocus/utils.js` |
| Chrome c'è, Edge no nel percorso solito | `ls "/c/Program Files/Google/Chrome/Application/chrome.exe"` |
| `lucide`, `playwright` e `@vitest/browser-playwright` non sono installati | `ls gui/node_modules/lucide` |

### Verificato nei documenti

La stella polare — la condizione del «Jarvis» in testa, la riga della Home nella tabella dei wireframe, la §2 con le viste col
nome e la sua riga 8, le decisioni 14, 39 e 48 — coi `grep` sulle frasi; la legenda e le due righe di `docs/tracciabilita.md`;
il compendio, per ADR-0027, 0029, 0030, 0031, 0033, 0034 e 0038 e per le invarianti. La verifica di coerenza è la misura 12 di
*«Cosa questo disegno ha misurato»*.

### Verificato alle fonti

Le regole dei token, le versioni al registro, Microsoft Learn, le pagine di Electron, Vue, Vite, Vitest e Playwright, i README
delle immagini della CI: nella sezione *«Il design system della GUI — le fonti del disegno, 2026-09-23»* di
[`riferimenti.md`](../../riferimenti.md), la casa unica delle fonti, coi comandi.

### Dedotto, e dichiarato tale

| Deduzione | Dove si misura |
|---|---|
| *«da Agentic OS»* è un aspetto, non la copia di un prodotto | è la lettura della risposta 1, detta al proprietario e non smentita |
| il canvas del 6, del 7 e del 12 legge i colori con `readToken` | quando arriva il primo canvas |
| passare a una sorgente TypeScript dei token non toccherebbe i componenti | se mai si passa |
| gli «spazi» si aggiungono sopra la Panoramica senza rifare niente | se mai si aggiungono |
| i caratteri non pesano sul pezzo JavaScript | al primo *build* del piano |
| la finestra di Electron con la barra nascosta prende gli angoli di Windows | al guscio del 10 |
| il tema si posa con `data-theme` e `matchMedia` | quando il piano lo scrive |

### Assunto, e chi lo misura

| Assunzione | Chi la misura |
|---|---|
| il dock vestito dà l'aspetto della Home approvata | il proprietario, alla prima prova del piano |
| l'Assistente vocale annuncia una `BaseStatus` che si riempie | il passo a mano del piano, decisione 21 |
| sull'altra macchina c'è Chrome | la prima corsa là |

## Cosa questo disegno ha misurato, e che non era scritto da nessuna parte

| # | Misurato il 2026-09-23, scrivendo | Che cosa ne segue |
|---|---|---|
| 1 | il **codice non è cambiato** dalla nascita del diario: `git log --oneline 2a674cc..HEAD -- . ':!docs'` non rende nulla | i fatti delle risposte valgono per il codice di oggi; sono stati **rilanciati** comunque, non citati, e le divergenze sono le righe qui sotto |
| 2 | il cancello rilanciato **all'apertura**, prima di toccare un file: `bash scripts/gate.sh` → `GATE GREEN`, e `bash scripts/check-docs.sh` → `OK`. Nel log del cancello il pezzo JavaScript è di **663,26 kB**, 201,23 kB compresso — `grep -n 'kB' <log>` — lo stesso della risposta 21 | la baseline da cui il piano parte è verde, e si **rimisura** all'apertura del piano invece di leggersi qui. ⚠️ La cifra sta qui **una volta**, con la data e il comando, perché è la baseline di questa sessione |
| 3 | la **CI** dei due commit della quarta sessione che alla sua chiusura erano in corsa, `54e53c0` e `ab39f39`: due job per corsa, **entrambi `success`**, letti dall'API coi due comandi del punto 3 della quarantunesima chiusura del [piano della parte 2](../plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md) | nessun rosso ereditato |
| 4 | **i ruoli per tema non sono quelli che la risposta 16 scriveva.** I due comandi della (a) rendono lo **stesso** numero sui due temi, e più alto di quello della risposta; il numero della risposta è quello dei soli ruoli presi da una **scala** — `grep -c 'var(--ref-'` sullo stesso blocco — e gli altri quattro sono `--color-veil`, un colore con la trasparenza, `--color-border-card`, un ruolo che punta a un ruolo, e le due ombre | la tavola, approvata, **non cambia**: sbagliava la frase, non i token. Il disegno scrive il **comando**, non la cifra — `CLAUDE.md`: *un numero misurato non si scrive* — e la consegna in archivio resta com'era, perché è un verbale |
| 5 | **`panels/chat/` non esiste**, e la risposta 17 lo dava come esempio di un modulo con la cartella sua: `find gui/src -type d` rende `panels/views/`, che tiene le disposizioni delle tre viste e il test che le genera | la regola regge **senza precedenti**: si applicherà la prima volta che un modulo cresce. La (b) lo dice |
| 6 | i **raggi a mano** dei temi «spaced» di `dockview` stanno **sotto le loro classi** — sei temi, `grep -n -B3 'border-radius: 8px'` sul CSS — e la risposta 18 diceva che il nostro tema li *«sovrascrive»* | sotto la classe nostra quelle regole **non valgono**: il tema nostro **scrive** il raggio dei token sui contenitori galleggianti, non sovrascrive niente. La (c) lo dice |
| 7 | la **legenda** di `tracciabilita.md` dice 🔶 *«meccanismo deciso, politica o implementazione nel sotto-progetto indicato»*; la risposta 21 la parafrasava come *«fatto nel sotto-progetto indicato»* | il merito non cambia; il piano scrive la riga con le parole della **legenda**. La (e) le cita |
| 8 | **`--z-floating` non stava nella tavola dei token**, nata alla risposta 16, prima della 18 che l'ha approvato; e i *«nostri dialoghi a 200»* della risposta 18 sono `--z-overlay` | aggiunto **nella tavola**, sopra `--z-popover`: una riga che sulla tavola non si vede, e i valori restano in una casa sola — decisione 23 |
| 9 | in `gui/src` oggi **non c'è** né un tema, né il movimento ridotto, né l'alto contrasto; la tavola porta la regola del movimento ridotto e **nessuna** per `forced-colors` | il piano le scrive tutte e tre; per l'alto contrasto la **prova** non è decisa da nessuna sezione: *«Le trappole»* |
| 10 | la presa grande crea i suoi due `<button>` con **`document.createElement`**, in `frame/BigTab.ts`: la regola `vue/no-restricted-html-elements` guarda i **template**, e lì non li vede | la (c) li sostituisce con `BaseButton`; finché resta un `createElement`, il linter non lo coglie: *«Le trappole»* |
| 11 | la policy VRAM delle Impostazioni è un gruppo di **radio nativi** con **tre sottigliezze** scritte nel sorgente — il radio si muove solo quando torna `Policy`, `:checked` da solo ha un buco, e `change` e non `click.prevent` — nel commento sopra il gruppo, in `gui/src/panels/Settings.vue` | `BaseRadioGroup`, sopra `reka-ui`, le deve **tenere**: *«Le trappole»* |
| 12 | **la verifica di coerenza**, il testo approvato riletto contro le invarianti e gli ADR che tocca: **I1** — il tema e le viste col nome li custodisce il core, nel pacchetto opaco; **I4** — nessun messaggio nuovo, il pacchetto viaggia come byte in `Layout`; **ADR-0027** e **0030** — tutto è CSS e Vue, e `lucide` è preso nel pacchetto dei **soli disegni**, agnostico; **ADR-0029** — `nativeTheme` e `titleBarOverlay` sono di Electron, e il guscio resta del 10; **ADR-0031** — nessuna dipendenza del kernel; **ADR-0033** — le miniature sono schemi, non un dock ciascuna; **ADR-0034** — il tema non è un parametro del kernel; **ADR-0038** — la disposizione resta fuori dal registro, decisione 14 della stella polare; **G13** — la provenienza nella chat, *«Le trappole»*; **G20** — la (a), la (e) e la (f) | **regge**, e nessuna decisione cambia. Le precisioni sono le righe 4–11 |
| 13 | le **versioni** al registro npm, rilanciate col comando di [`riferimenti.md`](../../riferimenti.md): `lucide` 1.47.0, i due `@fontsource` 5.3.0 e `playwright` 1.63.0 invariati; `vitest` e `@vitest/browser-playwright` all'ultima 5.0.1, e noi restiamo alla 4.1.11; **`reka-ui` è alla 2.10.5** del 2026-09-21, contro la 2.10.4 installata, in cui è stato letto `RovingFocusGroup` | nessuna scelta cambia: il piano ricontrolla, il giorno che lo scrive, che la 2.10.5 non abbia toccato le frecce — o resta sulla 2.10.4, com'è la regola delle versioni del piano della parte 2 |
| 14 | il **margine del compendio** prima di muovere il puntatore: `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh` — 103 120 byte contro 111 616 | il puntatore della §6 si riscrive **senza crescere**; ciò che è verbale va in archivio |

## Le voci che questo disegno apre per il proprietario

| # | Voce | Perché è sua, e il consiglio |
|---|---|---|
| 1 | **la rilettura di questo disegno** | le sezioni sono approvate nel merito; da leggere come **aggiunte di chi scrive** sono le righe *«proposta»* della tabella dei controlli, le decisioni 23–27 e le precisioni di *«Cosa questo disegno ha misurato»*. Il consiglio: rileggerlo prima del piano, com'è stato per ogni disegno |
| 2 | **il numero** — il design system non ha una riga nella roadmap, e darglielo è suo: decisione 2 del coordinatore: **A** una riga nuova in coda alla roadmap, senza rinumerare, con la sua riga in *«Perché quest'ordine»* — viene prima del 13; **B** nessun numero: resta il primo tempo della §6, con una riga nella tabella delle decisioni della stella polare | **consiglio A**: lo stato per traguardo vive nelle tabelle di `roadmap.md` e di `README.md` (`CLAUDE.md`), e un lavoro con disegno, piano ed esecuzione suoi senza una riga lì ha come sola casa il puntatore della §6, che è una riga. Costo: una riga nella roadmap, una in *«Perché quest'ordine»* e una in `README.md`, scritte dal piano. ✅ **Scelto il 2026-09-23: A**, in chat, subito dopo la scrittura del disegno, con un clic sul consiglio: il piano scrive le tre righe, e il numero è il primo libero in coda alla roadmap |

Restano sue, e **non** le apre questo disegno: N-2 di E187, i chunk; E228, chi costruisce progress e notifiche; il terzo
carattere; e **AUD-004**, che sbarra il secondo tempo della §6, il sotto-progetto 13.

## Decisioni prese dal coordinatore, col perché — il proprietario può ribaltarle

Le righe fino alla 22 sono quelle del brainstorming che restano **vive**, **parola per parola** e coi loro numeri, perché le
sezioni le citano così; le decisioni del **metodo** del brainstorming — 1, 4, 6, 9 e 12: dove nasceva la consegna, il visual
companion nello scratchpad, la cornice delle tavole, il confronto dei caratteri, le scatole del companion nella sonda — stanno
nella [consegna in archivio](../../archivio/consegna-avvio-brainstorming-design-system.md). Le righe 23–27 sono della sessione
che ha scritto il disegno.

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 2 | il nome **senza numero** di sotto-progetto | la roadmap non ne ha uno per il design system, e darglielo è del proprietario. Costo: una rinomina |
| 3 | i commit **senza** `Co-Authored-By` | `CLAUDE.md` dice «senza co-autore»; la direttiva di sistema chiede il contrario, e la divergenza è portata al proprietario, come in ogni sessione di questo repository. Costo: un `--amend` |
| 5 | ogni tavola si **verifica prima di mostrarla**: un'anteprima privata servita in locale, i contrasti WCAG calcolati dal generatore, e sonde nel browser sul testo tagliato, sugli elementi che sbordano e sui raggi concentrici | *«curato nel minimo dettaglio»*: la prima tavola aveva tre difetti visibili che nessuno avrebbe visto leggendo il codice. Costo: qualche minuto per tavola |
| 7 | le parole del proprietario **testuali** nelle risposte, e l'immagine di riferimento **non committata** | è il lavoro di un altro, e il repository è pubblico; gli spunti che se ne prendono stanno a parole nella risposta 2. Costo: se serve di nuovo, la si chiede al proprietario |
| 8 | la domanda 4, il dock, **non posta**: la risposta 5 la decide | con due temi `themeAbyss`, solo scuro, non può restare, e chiedere una cosa già determinata sarebbe una domanda di rito. Costo: se il proprietario voleva `themeAbyss` nel solo tema scuro, lo dice e si riapre |
| 10 | la tavola approvata **si aggiorna** coi caratteri scelti, e non nasce un secondo file | una tavola coi caratteri di Windows mentirebbe sullo stile di oggi, e le tavole nuove partono da lì; com'era sta nella storia di git, col comando nella risposta 8. Costo: chi cerca la tavola della risposta 4 la trova in `8c0bbe6` |
| 11 | la tavola approvata porta anche le icone scelte, e con esse la **licenza di Lucide**, copiata parola per parola in testa al file | i disegni sono **copiati** nel file, e la licenza ISC chiede l'avviso *«in all copies»*; una parte delle icone viene da Feather, sotto MIT, e la licenza lo dice; il repository è pubblico. I caratteri invece non sono copiati: arrivano da jsDelivr. Costo: una cinquantina di righe in testa alla tavola |
| 13 | la Panoramica approvata si salva **a sé**, [`panoramica.html`](2026-09-22-design-system-tavole/panoramica.html), accanto alla tavola dello stile e non dentro | la tavola dello stile mostra lo stile, la panoramica è un meccanismo della cornice; ma una tavola nel repository serve a chi riprende dall'altra macchina, dove lo scratchpad non c'è. Le sue sonde: raggi, testo che entra, cifre e icone, pulite nei due temi e a 1440 e 800 px; i campioni della sonda dei caratteri pensati per la Home **mancano** in questa tavola, e lì la prova vale solo per il testo tagliato, lo sbordare, il caricamento e le cifre. Costo: due file da tenere coerenti |
| 14 | i nomi dei ruoli **proprietà prima** — `--color-bg-…`, `--color-text-…`, `--color-border-…` — invece dei nomi di oggi, `--ink` o `--surface` | un testo scritto con un fondo si vede dal nome, e il test del contrasto accoppia **ogni** `--color-text-*` con **ogni** `--color-bg-*` senza una lista a mano, che è la regola scritta in `contrast.test.ts`. Costo: il piano rinomina a macchina gli usi di oggi nei dodici componenti — quanti, lo dice il terzo comando delle sonde del censimento, più su; gli spazi `--space-1`…`--space-4` tengono nome e valore |
| 15 | i valori della tavola approvata restano **àncore esatte**, e si generano solo i gradini che mancano | il proprietario ha approvato quei colori guardandoli; una scala rigenerata da zero li avrebbe spostati tutti di poco. Costo: le scale non hanno gradini a passo fisso |
| 16 | il **monospazio** è quello del sistema — `ui-monospace`, Cascadia Mono, Consolas | un terzo carattere nel programma, come Geist Mono, sarebbe una dipendenza nuova, ed è del proprietario: registrata e non presa. Costo: il codice nella chat si vede diverso su Linux |
| 17 | i **generatori** restano nello scratchpad; nel repository va la **tavola**, che contiene per intero `base.css` e `themes.css` | è la regola di `CLAUDE.md` sulle misure, e il precedente di `gen_style_v*.py`. Costo: dall'altra macchina un gradino si ritocca a mano nella tavola, non rigenerandolo |
| 18 | la **Panoramica** si costruisce su **`BaseDialog`**, a tutta finestra | `reka-ui` dà già Esc, il fuoco chiuso dentro e il fuoco che torna al nome della vista; una finestra modale fatta a mano sarebbe un secondo modo di fare la stessa cosa. Costo: una variante di `BaseDialog` |
| 19 | nella griglia le frecce seguono la **geometria**, nelle quattro direzioni — giù va alla riga sotto — con la geometria di `gui/src/frame/moveActive.ts` estratta in un aiutante comune | `RovingFocusGroup` di `reka-ui` 2.10.4 è **lineare**: nel pacchetto installato `ArrowDown` → `"next"` e `ArrowUp` → `"prev"`, quindi in una griglia «giù» andrebbe a destra. La geometria delle quattro direzioni esiste già, per i pannelli, e questa è la **seconda occorrenza**: si estrae, com'è la regola del kit; come si tiene il fuoco su un elemento solo lo dice il piano. Costo: l'aiutante e la sua sonda, coi rettangoli dati a mano come in `keys.test.ts`, perché sotto jsdom ogni rettangolo è zero |
| 20 | **`BaseTextField`** entra nel kit | dare il nome a una vista vuole un campo di testo, e con la ricerca della barra i campi sono due: la regola della (b), la seconda occorrenza. Costo: un pezzo di base in più, col suo test `axe` |
| 21 | la prova di M-3 col **lettore di schermo vero** è un passo del piano, a mano, con l'**Assistente vocale** di Windows | è già nel sistema, quindi nessuna dipendenza; la voce la voleva *«con un lettore di schermo vero in mano»*, e nessuna sonda automatica sa se un annuncio si sente. Costo: un minuto del proprietario, o di chi rivede, e il verbale di ciò che si è sentito |
| 22 | il browser dei test è il **Chrome installato**, canale `chrome` | c'è su questa macchina — `C:\Program Files\Google\Chrome\Application\chrome.exe` — e su tutte e due le immagini della CI, mentre Edge qui **non** sta nel percorso solito; il Chromium di Playwright andrebbe scaricato. Costo: 🔶 sull'altra macchina serve Chrome, o `npx playwright install chromium` — si verifica là |
| 23 | `--z-floating: 50` scritto **nella tavola dei token**, nel blocco di `base.css`, sopra `--z-popover` | i valori vivono in una casa sola, la tavola, e il piano li copia da lì: lasciarlo solo qui ne avrebbe fatte due, e una trappola per chi copia. Sulla tavola la riga non si vede. Costo se sbagliato: una riga da togliere |
| 24 | il disegno tiene le risposte **in breve** e le decisioni vive **parola per parola**; il testo intero delle risposte, le decisioni del metodo e le trappole delle tavole restano nella consegna in archivio | una casa sola per ciascuna cosa (gotcha #68): la consegna è un verbale e non si tocca più, il disegno è il documento vivo; le sezioni citano le decisioni per numero, quindi i numeri restano. Costo: chi vuole le parole testuali del proprietario apre l'archivio |
| 25 | le **sviste** del diario — i ruoli per tema, `panels/chat/`, *«si sovrascrive»*, la legenda parafrasata — sono corrette **qui** con la misura, e restano nella consegna come erano | la regola di `CLAUDE.md`: *un'evidenza scritta prima della misura è un'ipotesi*, e dove diverge si registra la divergenza; nessuna tocca il merito approvato. Costo: nessuno |
| 26 | il disegno **non entra** da solo nella §12 del compendio, in `README.md`, in `roadmap.md` né in `tracciabilita.md`; si muovono solo il puntatore della §6 e la riga d'intestazione del compendio | il precedente dei disegni dei gesti e della knowledge base — la misura 6 di quello della knowledge base: li scrive il piano; il puntatore si muove oggi perché il prossimo passo è cambiato. Costo se sbagliato: una riga nel piano in meno |
| 27 | i controlli **proposti** della tabella del prodotto | ogni artefatto vuole un controllo, e alcune sezioni approvavano la regola senza dire come si controlla: la forma più piccola è scritta accanto, e il piano la sceglie. Costo se sbagliato: il piano ne sceglie un'altra |

## Vicoli ciechi e scelte scartate, col perché

| Scartato | Perché |
|---|---|
| **Storybook**, una vetrina a sé — la B della risposta 12 | una dipendenza grande e un secondo mondo da tenere verde; la pagina kit dentro l'app fa lo stesso lavoro coi pezzi veri |
| **Histoire** | fermo a una beta del 2026-01-07 e chiede Vite 7: col nostro Vite 8 non regge |
| una **sorgente TypeScript** dei token con un generatore — la B della risposta 14 — o `style-dictionary` | un generatore, un passo di *build* e un file generato da tenere allineato, per un vantaggio che serve dal 6; 🔶 passarci più avanti non toccherebbe i componenti |
| **Tailwind** | cambierebbe il modo in cui sono scritti tutti e dodici i componenti |
| i **caratteri di sistema** — la A della risposta 6 | su Linux l'aspetto cambierebbe da solo |
| **Inter + Barlow** e **Geist + Barlow Semi Condensed** — la risposta 8 | misurate più lontane dalla tavola approvata, con `sonda-caratteri.js` |
| **Tabler** — la risposta 10 | ha più icone, ma a misura piccola Lucide è più essenziale — l'occhio del coordinatore, detto come tale e non misurato; 1848 bastano anche per i pilastri |
| **Phosphor** e **Heroicons** | nessuna uscita dal 2024 |
| **tutte** le icone di Lucide — la B della risposta 11 | centinaia di kB in più per icone che nessuno usa |
| il pacchetto di Lucide **per Vue** | i soli disegni sopravvivono a un cambio di framework, nello spirito di ADR-0030 |
| gli **strati di stato** di Material, trasparenze sopra il fondo | il test del contrasto sa giudicare un colore, non una trasparenza su un fondo che non conosce |
| `themeAbyss` **nel solo tema scuro** — la domanda 4, non posta | con due temi non può restare (decisione 8) |
| **un tema solo** adesso — la B della risposta 5 | il proprietario ha voluto i due temi subito |
| la scelta del tema come **chiave della settima porta** — la B della risposta 16 | il pacchetto della disposizione viaggia già ed è opaco al core; una chiave nuova avrebbe toccato il kernel |
| le **miniature vive**, un `dockview` per miniatura — la B della risposta 19 | con la scena 3D di SP-8 la GUI sta già sopra P3 sotto flusso — M4 in `spikes/RISULTATI.md`, contro il tetto di un quarto di core di `spikes/GUI-REQUISITI.md`; e l'immagine presa uscendo da una vista vuole il guscio e invecchia |
| la **striscia piatta** attaccata al fondo — la B della risposta 20 | perde la pillola approvata |
| **cambiare la legenda** della tracciabilità — la B della risposta 21 | avrebbe confuso tutte le righe ✅, e il file esiste per dire di quale meccanismo del kernel ha bisogno ogni funzione |
| le sonde che **restano script a mano** — la B della risposta 22 | la regola dei raggi è saltata più volte; gli invarianti diventano test |
| il **Chromium di Playwright**, da scaricare, ed **Edge** | il Chrome installato c'è qui e sulle due immagini della CI; Edge qui non sta nel percorso solito (decisione 22) |
| gli **«spazi»** — la A della risposta 13 | la Panoramica fa vedere tutto prima di entrare e regge con le viste salvate; 🔶 gli spazi si possono aggiungere dopo |
| una **finestra modale fatta a mano** per la Panoramica | sarebbe un secondo modo di fare ciò che fa `BaseDialog` (decisione 18) |
| **`RovingFocusGroup`** per la griglia della Panoramica | è lineare: giù andrebbe a destra (decisione 19) |
| *«sempre le sonde, mai l'occhio»*, detto in chat alla risposta 22 | era falso per la prima volta, che l'ha vista il proprietario; corretto subito dopo la risposta |

## Le trappole che mordono scrivendo il piano

| # | Trappola | Che cosa fare |
|---|---|---|
| 1 | una sonda che **non trova niente** è verde | la guardia di non-vacuità in ogni prova del browser, e si guardano i **conteggi** — `near`, le righe trovate — prima del verdetto: è successo con `near: 0` nella terza sessione, e con le righe `.fp` fuori da un `.ds` nella quarta |
| 2 | un carattere si misura **dopo** averlo caricato | `document.fonts.load` prima della misura; e l'oracolo del caricamento è lo stato `loaded` dei `FontFace`, **non** `document.fonts.check()`, che risponde «sì» anche per un nome che nessun `@font-face` dichiara |
| 3 | sotto jsdom **ogni rettangolo è zero**, e `axe` lascia il contrasto incompleto | la geometria si prova coi rettangoli dati a mano, come `frame/keys.test.ts`; il contrasto della pagina disegnata solo nel browser vero |
| 4 | `sonda-raggi.js` guarda **solo** le sue radici, e **non** confronta un angolo dritto | nel test le radici sono un parametro, e la guardia di non-vacuità conta le coppie confrontate |
| 5 | `vue/no-restricted-html-elements` guarda i **template** | un `document.createElement("button")` in un `.ts` non lo vede: è `frame/BigTab.ts` oggi, finché la (c) non monta `BaseButton` |
| 6 | un comando con i **backslash**, lanciato in linea, li perde in silenzio e la misura mente | si lancia da un file, come i tre del censimento |
| 7 | il rinomino dei token è meccanico **tranne** dove un token porta un significato | la **provenienza** nella chat, G13: `gui/src/panels/Chat.vue` segna un blocco non fidato col bordo sinistro `var(--warn)` **e** con le parole. Il ruolo nuovo tiene la distinzione, e sono le parole a portarla |
| 8 | `unpack` **butta** ciò che non riconosce | le viste col nome in un campo loro; un pacchetto scritto prima, senza `theme` o senza la lista, si apre ancora — le due direzioni, nel test del negozio |
| 9 | `--dv-overlay-z-index` vale **999** | `dock.css` lo lega a `--z-floating`, o i gruppi galleggianti passano sopra i dialoghi |
| 10 | `BaseRadioGroup` al posto dei **radio nativi** delle Impostazioni | le tre sottigliezze scritte in `Settings.vue` restano vere — il radio si muove solo quando torna `Policy`. ⚠️ I test che le tengono, in `gui/src/panels/modules.test.ts`, cercano `input[type=radio]`, e in `reka-ui` 2.10.4 un radio è un **`button`** con `role="radio"` (`dist/RadioGroup/Radio.js`), che le frecce muovono da sé: i test si riscrivono **senza perdere** ciò che provano, e la sottigliezza sul `change` si rilegge contro il componente nuovo |
| 11 | l'**alto contrasto** di Windows | la tavola non ha una regola per `forced-colors`, e G20 lo vuole rispettato: il piano dice come lo prova. 🔶 Playwright sa emulare `forced-colors`, e il contorno del focus, che è un `outline`, resta visibile |
| 12 | la pagina kit **fuori dal pacchetto** | Vite serve ogni pagina HTML in sviluppo ma costruisce solo gli ingressi elencati: si prova sull'uscita del *build*, non si crede |
| 13 | i **tasti** delle tavole sono esempi | F3 e le frecce si controllano contro quelli che esistono: `Ctrl+Alt+frecce` sono dei pannelli |
| 14 | le **licenze** | Lucide è ISC, e MIT per le icone di Feather; i caratteri OFL-1.1: vogliono l'avviso **con le copie**. Nella tavola sta in testa (decisione 11); nel programma impacchettato lo porta il pacchetto del 10 |
| 15 | `@vitest/browser-playwright` va alla **stessa** versione di `vitest` | 4.1.11, non l'ultima; e `playwright` non scarica un browser da solo |
| 16 | i **fine-riga** sono misti per file | i file di `gui/` da toccare sono LF nell'indice e, su questa macchina, CRLF nell'albero — `git ls-files --eol`; uno strumento conserva quelli del file che trova, e li rimisura dopo |
| 17 | i **valori** si copiano dalla tavola | fra i due commenti della (a); un generatore di tavole nuove li legge da `token.html` e non li ricopia, e prende le icone con la loro licenza da `panoramica.html` |
| 18 | `reka-ui` 2.10.5 al registro | se il piano sale, ricontrolla le frecce di `RovingFocusGroup` nel pacchetto nuovo |
| 19 | il pre-controllo di ogni compito ha trovato un difetto in **tutti** i compiti dispacciati finora (`CLAUDE.md`) | ogni compito si rilegge contro il codice di **allora**, non contro questo disegno |
| 20 | gli **attrezzi delle tavole** — il pannello del browser, `http.server` spento per PID, il visual companion | stanno nella consegna in archivio, *«Vicoli ciechi e trappole della prima sessione»*, e servono solo a chi fa una tavola nuova |

## Il prossimo passo

⛔ **Lo dice la §6 del [compendio](../../COMPENDIO.md), in un posto solo.**

### Come si riprende — scritto alla chiusura della sessione del 2026-09-23, coi comandi

⚠️ **È il documento di consegna di questa sessione**, e sta qui perché il repo tiene lo stato in file **tracciati**: chi
riprende legge **questo** file per intero. Ogni riga è stata **riletta coi comandi** prima di essere scritta, non ricordata.

⚠️ **RICHIAMO DEL 2026-09-23, alla chiusura della sessione:** **prima** della rilettura e del piano viene un mandato del
proprietario, il ridimensionamento della lettura, con la sua [consegna](2026-09-23-ridimensionamento-lettura-design.md); l'ordine
lo dice la §6 del compendio. I passi qui sotto valgono **dopo**, e quel mandato può cambiare che cosa dettano di leggere.

⛔ **DA SAPERE SUBITO: niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato, nessun
server acceso, nessun subagente, nessun codice toccato. ⛔ **La rilettura del proprietario non si dà per fatta:** si fa in chat,
non si deduce da una chiusura.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| **i commit di questa sessione** | `git log --oneline ab39f39..HEAD` — il disegno scritto sul posto, la consegna in archivio, le fonti in `riferimenti.md`, `--z-floating` nella tavola dei token, il puntatore della §6 del compendio; poi, in un secondo commit, la voce 2 del proprietario, scelta **A** |
| **codice di prodotto** | **non toccato**: `git diff --stat ab39f39..HEAD -- . ':!docs'` non rende nulla |
| **cancello** | `bash scripts/check-docs.sh` → `OK`; `bash scripts/gate.sh` → `GATE GREEN` all'apertura e alla chiusura. Si rilanciano, non si citano |
| **la CI** | i commit di questa sessione partono col push: la sessione dopo la legge **per prima**, coi due comandi del punto 3 della quarantunesima chiusura del [piano della parte 2](../plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md) — due job per corsa |
| **fine-riga** | `git ls-files --eol` su questo file, sulla consegna in archivio, su `docs/riferimenti.md`, su `docs/COMPENDIO.md` e sulla tavola dei token: la colonna `i/` è LF per tutti; `w/` è di chi misura — su questa macchina CRLF il disegno, `riferimenti.md` e il compendio, LF l'archivio e la tavola |
| **file temporanei** | nessuno nel repository: gli script della sessione stanno nello scratchpad |
| **debito lasciato** | **nessuno non dichiarato**: le voci del proprietario nella loro sezione, i punti aperti del piano nelle trappole, le proposte nella tabella dei controlli |

**Il compito della sessione successiva.** In ordine, e ogni riga è eseguibile:

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md`; poi, **per prima cosa**, la CI dei commit di questa sessione, coi comandi della riga
   «la CI».
3. **Questo file, per intero.** La consegna in archivio **non** è lettura obbligatoria: si apre per le parole testuali del
   proprietario o per le trappole delle tavole.
4. La **rilettura del proprietario**, con la domanda minima: *«il disegno è riletto?»* — **A**, sì: si passa al piano; **B**, no:
   le aggiunte di chi scrive della voce 1 di *«Le voci che questo disegno apre per il proprietario»* dette a parole, una per
   volta, in forma A/B col consiglio scritto — la voce 2 è già scelta. Se il proprietario non dice altro, il piano scrive i
   consigli.
5. Prima di scrivere il piano, la regola di `CLAUDE.md` su `superpowers:writing-plans`: le voci aperte **si sanno prima** — la
   colonna *«Chi la chiude»* di [`porta-di-qualita.md`](../../porta-di-qualita.md), le voci senza numero AUD
   dell'[audit](../../audit-2026-08-27.md), le voci del proprietario qui sopra.
6. `superpowers:writing-plans`, in una **sessione nuova** — la regola del proprietario, una fase per sessione: il piano in
   `docs/superpowers/plans/`, coi compiti che vengono dalle sezioni (a)–(f) e dalla tabella dei controlli, la Definizione di
   «fatto», l'errata in testa e la tabella della posizione — la forma dei piani precedenti, `ls docs/superpowers/plans/`.
7. Il **pre-controllo** delle quattro domande di `CLAUDE.md` su ogni compito, nella sessione che scrive il piano.
8. L'**esecuzione** in un'altra sessione ancora, un subagente fresco per compito, revisione fra uno e l'altro
   (`superpowers:subagent-driven-development`).
9. A piano eseguito: la riga «Accessibilità» di `tracciabilita.md`, questo file nella §12 del compendio e in `README.md`, e la
   riga della roadmap, con la sua riga in *«Perché quest'ordine»*: il numero è **A**, scelto il 2026-09-23.

📌 **Ciò che questo disegno consegna a chi scriverà il piano**, ed è suo e non un puntatore: le sezioni (a)–(f), la tabella dei
controlli per artefatto, le venti trappole, le misure 4–11 — le cose che il piano avrebbe dovuto scoprire da sé — e le due voci
del proprietario, la seconda già scelta.
