# Il design system della GUI — la consegna dell'avvio

⚠️ **QUESTO FILE È NATO COME CONSEGNA, il 2026-09-22, della sessione che ha chiuso il residuo del compito 17 del piano
della parte 2 e NON ha fatto il brainstorming:** il proprietario lo vuole in una sessione nuova — la sua regola *«una fase
per sessione»*, decisioni 39 e 48 della [stella polare](2026-09-07-direzione-gui-design.md) — e riprendibile anche
dall'altra macchina. La sessione del brainstorming legge questo file **per intero**, e alla propria chiusura lo **riscrive
sul posto** — come consegna delle sezioni approvate, o come disegno — e sposta questo testo **parola per parola** in
`docs/archivio/consegna-avvio-brainstorming-design-system.md`, coi soli link riscritti per la cartella: il viaggio delle
consegne della [knowledge base](../../archivio/consegna-brainstorming-knowledge-base.md) e del
[sotto-progetto 2](../../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md).

✅ **RICHIAMO DEL 2026-09-23 — il brainstorming dura più di una sessione, e questo file ne è il diario.** La prima sessione del
brainstorming lo ha aggiornato **sul posto** a ogni risposta del proprietario, e alla chiusura — sua, *«troppo contesto
saturo»* — ci ha scritto la ripresa. La riscrittura come disegno e lo spostamento **parola per parola** in archivio si fanno
quando il **disegno** è scritto, com'è stato per la stella polare; il testo com'era alla nascita lo rende
`git show 2a674cc:docs/superpowers/specs/2026-09-22-design-system-design.md`.

⚠️ **Non è una spec e non è un disegno.** Il prossimo passo sta nella §6 del [compendio](../../COMPENDIO.md), in un posto solo.

## Stato in una riga

Il sotto-progetto 2 è chiuso, e il design system è il **primo tempo** del prossimo passo della §6. Il brainstorming è **a
metà**: cominciato il 2026-09-23, dodici risposte nella tabella *«Le risposte del proprietario, una per domanda»* — il
perimetro, l'idea di stile, la direzione, lo stile approvato coi raggi concentrici, i due temi, i caratteri aperti dentro il
programma, la stella polare letta a pezzi, la coppia **Geist + Barlow**, un set di icone aperto, **Lucide**, dietro un
**componente unico** con la sua mappa, la **pagina «kit»** nell'app — e ⛔ **la prossima è la domanda 8, come si cambia
vista**, l'ultima: è **visiva**, e va nel browser.

## ⛔ Da sapere subito

**Il brainstorming è a metà, e nient'altro.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato; il
visual companion e il server d'anteprima **spenti**, nessun subagente; nessun file della GUI toccato. Le risposte del
proprietario vivono nella tabella *«Le risposte del proprietario, una per domanda»*, e una domanda senza riga lì non ha
risposta: si ripone, non si deduce. Le tavole mostrate vivono nello scratchpad della sessione del 2026-09-23, **su questa
macchina** — `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\db7f9418-81f8-4641-a26c-fdc4ffec03f8\scratchpad\`,
coi generatori `gen_style_v1.py`, `gen_style_v2.py` e `gen_style_v3.py` — e **solo l'approvata** è nel repository, con la sua
sonda, in `docs/superpowers/specs/2026-09-22-design-system-tavole/`: dall'altra macchina si riparte da lì. La **seconda**
sessione, lo stesso giorno, ha lo scratchpad
`C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\68e60615-409a-456a-a16e-8ef420ce4dca\scratchpad\`, con
`gen_style_v5.py` — la tavola approvata coi caratteri e le icone scelti, da cui partono le tavole nuove — e i confronti
`gen_fonts_v1.py` e `gen_icons_v1.py`, più `gen_style_v4.py`, la tavola coi soli caratteri; tutti eseguono `gen_style_v3.py`,
copiato lì accanto, fino alla sua uscita, e `gen_style_v5.py` esegue così anche `gen_icons_v1.py`.

⚠️ **Sull'altra macchina** — il clone con `core.autocrlf` `true`, albero `w/crlf` — tre cose prima di credere a un verde o a
un rosso: un Node che soddisfi `node -p "require('./gui/package.json').engines.node"`; `cargo audit --version` **0.22.2**,
prerequisito del cancello (D68 del [piano della parte 2](../plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md)); e
un'etichetta di fine-riga scritta in un documento è la colonna `w/…` di **chi l'ha misurata**: si rimisura con
`git ls-files --eol`, non si crede (E51, E72, E224 dello stesso piano).

## Stato del repo alla chiusura, coi comandi che lo rifanno

| | Comando | Atteso |
|---|---|---|
| ramo | `git fetch --all --prune`, poi `git status -sb` | `## main...origin/main`, niente sotto |
| i commit della prima sessione del brainstorming | `git log --oneline 2a674cc..HEAD` | le risposte — `c88d831`, `7c83cf9`, `947dcec`, `8c0bbe6`, `35708f5` — e questa chiusura |
| cancello | `bash scripts/gate.sh` | `GATE GREEN` all'apertura del 2026-09-23 e prima del commit di chiusura — si rilancia, non si cita |
| documenti | `bash scripts/check-docs.sh` | `OK` |
| la CI | i due comandi del punto 3 della quarantunesima chiusura del piano della parte 2 | due job per corsa, entrambi `success`: così per `c88d831`, `7c83cf9` e `947dcec`; `8c0bbe6`, `35708f5` e questa chiusura erano **in corsa** alla chiusura, e si leggono **per prime** |
| margine del compendio | `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh` | positivo |

## Che cosa chiede il proprietario, e che cosa vuol dire «da Agentic OS»

**La richiesta, del 2026-09-21** — il punto 5 della trentasettesima chiusura del piano della parte 2, che si ritrova con
`grep -n 'UNA DECISIONE NUOVA, SUL DESIGN SYSTEM' docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md`:
lo stile attuale **non gli piace**, e vuole un design system *«da Agentic OS»* — **stile definito, componenti riutilizzabili,
kit UI, temi e token centralizzati, modularità** — a sotto-progetto 2 chiuso, col suo brainstorming. Se meriti un numero
nella tabella delle decisioni della stella polare lo decide lui.

**«Agentic OS» non è un prodotto da imitare:** è il nome che il proprietario dà al **programma intero** — *«l'Agentic OS è il
programma intero, non una funzione da aggiungere»* (decisione 3 del [disegno della chiusura del 1](2026-09-02-sottoprogetto-1-chiusura-design.md))
e *«Agentic OS, non chatbot»* (decisione 7 della stella polare, la chat fuori dalla Home). 🔶 **Dedotto:** *«da Agentic OS»*
vuol dire all'altezza di un sistema operativo per agenti, non la copia di qualcosa. Si conferma col proprietario alla prima
domanda, non si assume.

**L'aspetto che la stella polare aspetta è il «Jarvis».** I pannelli di `dockview` furono accettati a condizione di una prova —
*«se provandola non dà il "Jarvis", si passa alla tela libera»*, in testa alla stella — e nella tabella dei wireframe sta
*«assunto: l'aspetto Jarvis lo danno colori e forme»*, con *«colori e forme si decidono dopo, nel design system»*.

## Il contesto raccolto — verificato il 2026-09-22

| Pezzo | Dove | Oggi |
|---|---|---|
| i **token** | `gui/src/tokens/tokens.css` | un file solo, **un tema**, scuro; i valori sono **segnaposto dichiarati** in testa al file (*«THESE ARE PLACEHOLDER VALUES, and saying so is the point»*): è fissata la **forma**, ogni colore e ogni misura da una variabile. Quante variabili lo dice `grep -c '^  --' gui/src/tokens/tokens.css`, e l'ultima, `--dv-…`, è di `dockview` e non nostra |
| il **tema del dock** | `gui/src/frame/dock.ts`, sulla riga `theme:` | `themeAbyss` di `dockview`, quello su cui il proprietario giudicò le otto mosse di SP-8: i nostri token vestono **solo** ciò che disegniamo noi — la barra, la fascia, il cassetto, la striscia. Oggi nella stessa finestra convivono **due linguaggi visivi** |
| le **primitive** | `gui/package.json` | `reka-ui`, primitive **senza stile** — domanda 7 del disegno del 2, risposta **B**: tastiera e ARIA da loro, lo stile da noi. Oggi se ne usa il solo `Dialog`, in `components/Confirm.vue` e `frame/Drawer.vue` |
| le **seconde occorrenze**, cioè la regola del kit | le sonde qui sotto | ✅ **oggi ESISTONO**, e al compito 13 no — censite il 2026-09-22: `<button` in quattro componenti più i due comandi della maniglia grande, vestiti in `tokens.css`; le liste in tre; `role="status"` in tre; il `Dialog` in due, coi **due veli scritti a mano e già divergenti** — `rgb(0 0 0 / 50%)` in `Confirm.vue`, `rgb(0 0 0 / 0.45)` in `Drawer.vue`, i soli colori fuori dai token |
| i **tre momenti** del design system | la risposta 3 della [consegna d'avvio del 2](../../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md) | (1) nel 2 le cose care da rifare — cartelle per strato, token in un posto solo, G13 e G20 — ✅ fatto; (2) dopo M1–M5 ciò che dipende dal guscio — cornice della finestra, menu nativi, pacchetto — ✅ sbloccato, Electron dal 2026-09-10; (3) **con ogni pilastro il kit vero** — viewer 3D, diff, pannello della mappa. E la regola: *«il kit UI nel 2 non si fa»*, un componente si estrae alla **seconda** occorrenza |
| l'**accessibilità**, G20 | `spikes/GUI-REQUISITI.md`; `gui/src/a11y.test.ts`; `gui/src/tokens/contrast.test.ts` | `axe-core` sotto jsdom **non sa** giudicare il contrasto, lo segna incompleto: il contrasto AA lo tiene `contrast.test.ts` su coppie scelte, e un valore che fallisce si cambia **nel file dei token**. `--line` su `--surface` è un **sospetto dichiarato**, fuori dalla sonda |
| i **testi** | `gui/src/locales/it.json`, `vue-i18n` | le stringhe in un file di risorse, una lingua sola |

Le sonde del censimento. ⛔ Portano backslash, quindi si lanciano **da un file**, non in linea: in linea i backslash spariscono
in silenzio e la misura mente — la trappola della quarantatreesima chiusura.

```bash
grep -rl --include='*.vue' -e '<button' gui/src
grep -rn --include='*.vue' -E '#[0-9a-fA-F]{3,8}\b|rgba?\(|hsla?\(' gui/src
grep -rhoE 'var\(--[a-z0-9-]+\)' gui/src --include=*.vue --include=*.css --include=*.ts | sort | uniq -c | sort -rn
```

**Cosa si è letto, perché non si rilegga:** `tokens.css` intero; `gui/package.json`; la riga del tema in `dock.ts`; la consegna
d'avvio del 2 intera; nel piano della parte 2 il punto 5 della trentasettesima chiusura e la sezione *«Il design system da
Agentic OS — la risposta, misurata»*; della stella polare la testa e la tabella *«Le decisioni del proprietario, una per
domanda»*, più le righe che rende `grep -n -i 'jarvis\|design system\|kit UI' docs/superpowers/specs/2026-09-07-direzione-gui-design.md`.
✅ **RICHIAMO DEL 2026-09-23: la stella polare è stata letta per intero** dalla prima sessione del brainstorming, e le risposte
1–5 la presuppongono. Se rileggerla **per intero a ogni ripresa** sia obbligatorio è la registrata della tredicesima ripresa
della stella stessa — *«se all'apertura la stella polare si legga per intero»* — ed è del **proprietario**: la prima A/B della
ripresa, **prima** di leggerla.

## Le voci registrate e non prese che toccano la GUI — del proprietario

Il piano della parte 2 è chiuso, e queste restano aperte; stanno qui perché il brainstorming ne incrocia quattro, e perché la
quarantatreesima chiusura non le ha ripetute. Il testo intero sta nella voce, nell'errata di quel piano.

| Voce | Che cosa | Incrocia il design system? |
|---|---|---|
| **M-3** di E187 | le regioni `role="status"` di `Settings.vue` e `Status.vue` nascono nel DOM **insieme** al loro testo, e molti lettori di schermo non le annunciano; la cura urta la regola *«no empty box»* della §6a del disegno del 2 | **sì**: è un componente, la regione di stato, e si decide con un lettore di schermo vero in mano |
| **N-2** di E187 | l'avviso di `vite` sui chunk sopra i 500 kB, per `markdown-it`: tagliare i chunk di un'app locale in Electron è una domanda del proprietario | **sì**: un carattere o un set di icone nel pacchetto pesano sullo stesso numero |
| **E228** | *«Progress e notifiche per job lunghi → GUI minima»*, orfana in `tracciabilita.md`: il 2 non l'ha costruita, e la stella la dà a *«2 il meccanismo, 7 il primo job»* | **sì**: la notifica e l'avanzamento sono componenti del kit, oggi senza casa |
| **N-2** di E235 | «Accessibilità» è ✅ in `tracciabilita.md` con una legenda che dice *«meccanismo deciso nel kernel»*, e le fondamenta stanno nella GUI | **sì**: si decide se cambia la legenda o la riga, e le fondamenta visive sono di questo lavoro |
| **E227** | dove disegnare `DISPOSIZIONE` in `design/10` | no, è del modello dei dati |
| **E209** | i numeri di P-107 invecchiano | no |

## Le risposte del proprietario, una per domanda

| # | Domanda | Risposta |
|---|---|---|
| 1 | **2026-09-23** — che cosa vuol dire *«da Agentic OS»*, e quanto è grande questo lavoro: **A** le fondamenta più i componenti che oggi hanno la seconda occorrenza; **B** anche il kit dei pilastri che verranno | **A, nella sostanza — letta dal coordinatore e detta al proprietario come tale, quindi correggibile.** Il proprietario ha risposto con le sue parole e non con una lettera: *«Vorrei creare un design moderno da AGENTIC OS e che sia ben definito come stile, oltre ad essere ben organizzato come design system (quindi tutti i componenti riutilizzabili e non, kit ui, viste che favoriscono sempre l'uso di componenti etc..) che siano ben organizzati logicamente e architetturalmente e che lo stile prima di tutto rispecchi la mia idea, quanto appuntato nel progetto e sia professionale curato nel minimo dettaglio, con temi e token centralizzati, e modularità»*, e *«i futuri componetni e viste nascerando già ereditando tutto il necessario o creando quello che serve»*. La seconda frase è la **A**: ciò che verrà nasce quando arriva, ereditando o creando, e non si disegna ora. La prima ne fissa i **criteri**: (1) lo stile rispecchia **prima di tutto l'idea del proprietario** e ciò che il progetto ha già scritto; (2) professionale, curato nel minimo dettaglio; (3) temi e token centralizzati; (4) **tutti** i componenti, riutilizzabili e no, ordinati logicamente e architetturalmente; (5) le viste si compongono di componenti; (6) modularità. 🔶 **Dedotto, non smentito:** *«da Agentic OS»* è un aspetto all'altezza di un sistema operativo per agenti — *«un misto fra Jarvis e Claude Desktop»*, parole sue del 2026-09-07 che vivono nella memoria dell'agente e in nessun documento del repository — e non la copia di un prodotto. Sullo stile di oggi, parole sue: *«a me non piace per niente»* |
| 2 | **2026-09-23** — l'idea di stile del proprietario, chiesta a parole prima di ogni tavola (criterio 1 della risposta 1) | **Risposta a parole, con un'immagine di riferimento.** Le parole: *«qualcosa che non richiami roba olografica o vetro, anzi vorrei qualcosa che abbia prima due accenti primari (avevo in mente off-white e bordeaux per uno dei temi chiaro/scuro), angoli morbidi, ti mostro un immagine di quello che ha realizzato un ragazzo, NON deve essere uguale ma deve essere usato come spunto per quello che sarà il mio (Che sarà assolutamente differente non voglio copiare lui, ma prenderà spunto da quanto mostrato)»*. Quindi: (1) **niente olografico e niente vetro**; (2) **due accenti primari**, *off-white* e **bordeaux**, per uno dei due temi, chiaro o scuro — quale, lo dicono le tavole; (3) **angoli morbidi**; (4) l'immagine è **uno spunto e non un modello**. ⚠️ **L'immagine non è nel repository:** è il lavoro di un altro — la schermata di «RUBRIC Agentic OS», firmata «Jay E» e «RoboNuggets» — e il repository è pubblico; se serve di nuovo, la si chiede al proprietario. Gli spunti che il coordinatore ne legge, a parole: un **solo accento forte**, usato poco e solo per ciò che conta — i numeri chiave, il «prossimo», le azioni; **etichette di sezione in maiuscolo spaziato**, con un'icona piccola; **numeri grandi** con un'etichetta piccola sotto, il cruscotto a colpo d'occhio; **divisori sottili** al posto di scatole pesanti, e poche schede arrotondate; **al centro un anello di elementi tondi intorno a una rete di punti**, che è la forma della Home già approvata — l'anello degli artefatti e la rete viva, domande 1–3 della [stella polare](2026-09-07-direzione-gui-design.md); **piccoli grafici densi**, barre a segmenti e griglie di punti; **tabelle coi numeri allineati** e la riga «prossima» accesa; **moduli in colonne** intorno al nucleo. Restano aperti il **movimento** e **quale tema** porta off-white e bordeaux |
| 3 | **2026-09-23** — la domanda 2, le tavole: due direzioni, ciascuna nel tema scuro e in quello chiaro, mostrate nel browser col visual companion — **A, «Strumento»**: denso, da cruscotto, linee sottili al posto delle scatole, etichette in maiuscolo spaziato, numeri grandi; **B, «Scrivania»**: calmo, schede morbide con un'ombra leggera, più spazio | **A, con le schede morbide della B.** Parole sue: *«A, ma con le schede morbide della B (angoli arrotondati)»*. Nel browser aveva cliccato la B, e vale il terminale, com'è la regola del visual companion; il consiglio del coordinatore era la A. Quindi: il **linguaggio della A** — etichette in maiuscolo spaziato col segno bordeaux, numeri grandi e leggeri, la barra a segmenti, l'orario acceso nella riga «prossima», il nucleo col quadrante — **dentro schede arrotondate** come quelle della B. La palette delle tavole, rifinibile: off-white `#EFE9DF`, bordeaux `#7A1F2E`, carbone `#151112`, e un bordeaux acceso `#B0475A` **solo nel tema scuro**, per linee e segni sottili. ⚠️ **Il bordeaux sul carbone non basta né per il testo né per un segno sottile** (contrasto WCAG): nel tema scuro è uno **sfondo pieno** col testo off-white sopra; nel chiaro va bene ovunque. Le tavole e lo script che ne calcola i contrasti stanno nello scratchpad della sessione, **non nel repository**: la tavola unita, una volta approvata, si salva accanto a questo file, sul precedente dei wireframe della [stella polare](2026-09-07-direzione-gui-design.md) (decisione 1 del coordinatore) |
| 4 | **2026-09-23** — la tavola unita: la A con le schede della B, nei due temi, coi pezzi del kit accanto — pulsanti, segni, campo, messaggi di stato, finestra di conferma, lista | **Sì, lo stile è questo**, con una nota: *«sì, passa ai temi, attenzione alla concentricità dei border radius, non voglio casi come come questo che vedi in foto, piccola nota di dettaglio»*. La foto era la finestra di conferma della tavola: una scheda di raggio 14 dentro un riquadro di raggio 10, a dieci pixel. **La regola, decisione del coordinatore dalla nota:** *raggio di fuori = raggio di dentro + distanza*, **per costruzione nei token** e non per convenzione — `--r-card: calc(var(--r-ctl) + var(--pad-card))`, `--r-frame: calc(var(--r-card) + var(--gap))` — così, se cambia un margine, i raggi lo seguono da soli. La scala delle tavole: 6 in linea, 8 i controlli, 18 le schede, 28 la cornice, e le pillole, dove dentro una pillola va una pillola; un elemento che non può stare in un angolo con lo stesso centro **si allontana dall'angolo**: la finestra di conferma sta al centro. **La sonda** è [`sonda-raggi.js`](2026-09-22-design-system-tavole/sonda-raggi.js), accanto alla [tavola approvata](2026-09-22-design-system-tavole/stile-approvato.html), salvata sul precedente dei wireframe della [stella polare](2026-09-07-direzione-gui-design.md): provata nelle **due direzioni**, vuota sulla tavola approvata e rossa su quella precedente, dove coglie proprio la finestra della foto. ⚠️ **La prima forma della sonda NON la coglieva**, perché guardava solo dentro l'arco esterno e la finestra ne stava esattamente sul bordo: il perché sta nel commento della sonda. Il piano la farà diventare un test del kit vero, come `gui/src/tokens/contrast.test.ts` tiene il contrasto. 🔶 **Registrata, non presa:** l'angolo della finestra vera lo disegna Windows, e come si accorda con la cornice da 28 lo decide la cornice della finestra |
| 5 | **2026-09-23** — la domanda 3, i temi: **A** due temi subito, scuro e chiaro, che di base seguono Windows e si possono scegliere a mano; **B** solo lo scuro adesso, coi token pronti per il chiaro | **A.** Parole sue: *«A, due temi subito»*. Verificato alla fonte il 2026-09-23, la pagina di `nativeTheme` della documentazione di Electron: `themeSource` vale di base `system`, e la query CSS `prefers-color-scheme` della pagina lo segue; `inForcedColorsMode` dice se Windows è in alto contrasto, e l'evento `updated` segnala un cambio. La scelta a mano **non** la tiene la GUI (I1): la conserva il core, come la disposizione — dove, lo dice il disegno. L'alto contrasto di Windows e la riduzione del movimento **non sono temi**: sono regole di G20, sempre rispettate. **Conseguenza, decisione del coordinatore: la domanda 4 cade.** Con due temi `themeAbyss`, che è solo scuro, non può restare: il dock si veste coi **nostri** token con un tema suo — il `DockviewTheme` di `dockview-core` 8.3.1 porta `className`, `colorScheme` e `gap`, letto in `gui/node_modules/dockview-core/dist/cjs/dockview/theme.d.ts` il 2026-09-23 — e le sue variabili di raggio, `--dv-border-radius` fra le altre, seguono la regola dei raggi concentrici della risposta 4 |
| 6 | **2026-09-23**, seconda sessione — la domanda 5, i caratteri, riposta uguale coi fatti riverificati quel giorno: **A** quelli di sistema, e su Linux l'aspetto cambia da solo; **B** due caratteri aperti dentro il programma, uno per il testo e uno da strumento per etichette e numeri, stesso aspetto ovunque e niente rete a runtime | **B.** Con un clic, sul consiglio — l'opzione *«B, aperti nel programma»*. I fatti riverificati: la tavola approvata usa `Bahnschrift` per etichette e numeri e `system-ui` per il testo, e la GUI vera i caratteri di sistema — `grep -o "font-family:[^;]*"` sulla tavola, e la variabile `--font` di `gui/src/tokens/tokens.css`; **nessun documento** ha già scelto i caratteri, stella polare compresa — un `grep` su `docs/`, `gui/src` e `spikes/GUI-REQUISITI.md`, archivio escluso, trova «tipografico» solo nel senso di refuso; i quattro candidati e il loro peso col comando della sezione qui sotto, contro i 368,9 MB della cartella installata, riga M2 di `spikes/RISULTATI.md`. È una **dipendenza nuova**, ed è sua. Nessun candidato è scelto: la coppia si sceglie **guardandola** |
| 7 | **2026-09-23**, seconda sessione — la registrata della tredicesima ripresa della [stella polare](2026-09-07-direzione-gui-design.md), in *«Registrate, non prese»*: **A** rileggerla per intero a ogni ripresa di questo brainstorming; **B** i soli pezzi che la domanda del momento tocca, trovati col `grep`, e per intero una volta, prima di scrivere il disegno | **B.** Con un clic, sul consiglio — l'opzione *«B, solo i pezzi utili»*. La stella pesa quanto dicono `wc -c` e lo snippet `tiktoken` di `CLAUDE.md`: il 2026-09-23, 187 481 byte e almeno 60 224 token, limite inferiore. Vale per **questo brainstorming**: la voce nella stella polare e la riga della stella nella §12 del compendio portano il richiamo con la data |
| 8 | **2026-09-23**, seconda sessione — le coppie di caratteri, nel browser: la Home e il kit della tavola approvata in tre coppie, più il riferimento coi caratteri di Windows; **A** Geist + Barlow, **B** Inter + Barlow, **C** Geist + Barlow Semi Condensed — B e C cambiano un carattere solo rispetto ad A | **A, Geist + Barlow.** Con un clic, sul consiglio. Il consiglio era **misurato**, nel pannello browser dell'app, con [`sonda-caratteri.js`](2026-09-22-design-system-tavole/sonda-caratteri.js): un'etichetta di prova in Barlow è 0,992 di Bahnschrift e una frase in Geist 1,019 di Segoe UI Variable Text — Inter 1,075, Barlow Semi Condensed 0,916 — quindi con A le lettere occupano quasi lo stesso spazio della tavola approvata; in tutte e tre le coppie le cifre sono tabulari. La [tavola approvata](2026-09-22-design-system-tavole/stile-approvato.html) ora porta la coppia scelta, e la sonda nuova le sta accanto; com'era alla risposta 4 lo rende `git show 8c0bbe6:docs/superpowers/specs/2026-09-22-design-system-tavole/stile-approvato.html` |
| 9 | **2026-09-23**, seconda sessione — la domanda 6, le icone: **A** nessun set, poche icone disegnate da noi; **B** un set aperto, già pronto, usato da un solo componente del kit | **B.** Con un clic, sul consiglio — l'opzione *«B, un set aperto»*. I fatti verificati quel giorno: in `gui/src` **nessuna** icona, e nessun documento ha già scelto un set, stella polare compresa; le icone **arrivano** — l'immagine di riferimento della risposta 2 mette una piccola icona accanto a ogni etichetta di sezione, e poi pulsanti, cambio di vista, pilastri; i candidati nella sezione qui sotto. Il pacchetto dei **soli disegni** e non quello per Vue è la lettura del coordinatore, nello spirito di ADR-0030: si decide nel disegno. È una **dipendenza nuova**, ed è sua |
| 10 | **2026-09-23**, seconda sessione — il set di icone, nel browser: le stesse venti icone da **A** Lucide e **B** Tabler, sulla Home e sul kit della tavola approvata coi caratteri scelti, e le venti di ciascun set, più grandi, in una striscia | **A, Lucide.** Con un clic, sul consiglio: a misura piccola, accanto alle etichette, è più essenziale — l'occhio del coordinatore, detto al proprietario come tale e **non** misurato; Tabler ne ha di più, ma 1848 bastano anche per i pilastri. La tavola era pulita sulle tre sonde, e [`sonda-icone.js`](2026-09-22-design-system-tavole/sonda-icone.js) è nata qui: icone disegnate, centrate, lo stesso numero per set. La [tavola approvata](2026-09-22-design-system-tavole/stile-approvato.html) ora porta anche le icone, e in testa la licenza di Lucide copiata parola per parola |
| 11 | **2026-09-23**, seconda sessione — la **richiesta del proprietario**, scritta nel mezzo del turno: *«vorrei fosse un componente centralizzato con tutto il set delle icone mappate al suo interno riutilizzato in tutto il software»*; e la domanda che ne è nata, che cosa vuol dire «tutto il set»: **A** le icone che usiamo, coi nomi nostri; **B** tutte le 1848 di Lucide | **A.** Con un clic, sul consiglio. Quindi: **un solo componente** `Icon` e **una sola mappa** — nome nostro, come `stato` o `cerca`, → icona di Lucide, importate una per una — e nessun'altra parte del codice importa icone; cambiare set tocca la sola mappa. La B costava **442 433 byte** in più, `lucide` 1.47.0 intero e minimizzato, contro circa 500 byte a icona — 935 024 byte per 1848 moduli — col comando nella sezione qui sotto. Come la regola diventa **controllata** — il nome come tipo, che non compila se è sbagliato, e una regola del linter che vieta gli import fuori dalla mappa — è una proposta del coordinatore, detta al proprietario: si decide nel disegno |
| 12 | **2026-09-23**, seconda sessione — la domanda 7, dove si guardano i componenti: **A** una pagina «kit» dentro l'app, solo in sviluppo, con ogni componente in ogni stato nei due temi, e le sonde che diventano test su quella pagina; **B** Storybook, una vetrina a sé | **A.** Con un clic, sul consiglio. I fatti verificati quel giorno: oggi nessun posto mostra tutti i componenti — si vedono nelle loro viste, contro il core finto, e si provano con `vitest`; `storybook` 10.6.0 del 2026-09-02, e `@storybook/vue3-vite` accetta Vite `^8.0.0`, quindi la B era possibile davvero — il nucleo pesa 22 168 203 byte spacchettati, con 17 dipendenze dirette; `histoire` è fermo alla 1.0.0-beta.1 del 2026-01-07 e chiede Vite `^7.3.0`: col nostro Vite 8 non regge. Il comando è qui sotto |

```bash
python -c "import json,urllib.request as u; g=lambda n: json.load(u.urlopen(u.Request('https://registry.npmjs.org/'+n.replace('/','%2F'),headers={'User-Agent':'harness'}))); [print(n, (lambda d,v: (v, d['time'][v][:10], d['versions'][v]['dist'].get('unpackedSize'), len(d['versions'][v].get('dependencies',{})), d['versions'][v].get('peerDependencies',{}).get('vite')))(d, d['dist-tags']['latest'])) for n,d in ((n,g(n)) for n in ['storybook','@storybook/vue3-vite','histoire'])]"
```

## I caratteri — i candidati, e il comando che li verifica

La risposta 6 è **B**, e la 8 sceglie la coppia: ✅ **Geist** per il testo, **Barlow** per etichette, numeri e orari. I
candidati confrontati, **verificati il 2026-09-23** col comando qui sotto, che stampa versione, data d'uscita, licenza e peso
spacchettato in byte:

| Pacchetto | Per che cosa | Forma | |
|---|---|---|---|
| `@fontsource-variable/inter` | il testo | variabile | |
| `@fontsource-variable/geist` | il testo | variabile | ✅ scelto |
| `@fontsource/barlow` | etichette e numeri | pesi fissi | ✅ scelto |
| `@fontsource/barlow-semi-condensed` | etichette e numeri, più stretta | pesi fissi | |

I pesi di Barlow che la tavola usa sono **quattro** — 300 per i numeri grandi, 400, 500 per il pulsante della striscia, 600
per le etichette — e il browser ne carica proprio quattro; Geist è variabile, un file per sottoinsieme.

Tutti `5.3.0` del 2026-07-19 e `OFL-1.1`. Il peso spacchettato — da 0,18 a 1,9 MB per pacchetto, con tutti i sottoinsiemi e
tutti i formati — è un **limite superiore**: nel programma va solo ciò che i token importano.

```bash
python -c "import json,urllib.request as u; [print(n, (lambda d: (d['dist-tags']['latest'], d['time'][d['dist-tags']['latest']][:10], d['versions'][d['dist-tags']['latest']].get('license'), d['versions'][d['dist-tags']['latest']]['dist'].get('unpackedSize')))(json.load(u.urlopen(u.Request('https://registry.npmjs.org/'+n.replace('/','%2F'),headers={'User-Agent':'harness'}))))) for n in ['@fontsource-variable/inter','@fontsource/barlow','@fontsource/barlow-semi-condensed','@fontsource-variable/geist']]"
```

## Le icone — i candidati, e il comando che li verifica

La risposta 9 è **B**, la 10 sceglie ✅ **Lucide**, e la 11 lo mette dietro **un componente unico con una mappa sola**. I
candidati confrontati erano i due set **aggiornati nell'ultima settimana**, verificati il 2026-09-23 coi comandi qui sotto — il
primo stampa versione, data d'uscita e licenza, il secondo conta i file d'icona delle due versioni, il terzo pesa Lucide: i
moduli d'icona uno per uno, e il pacchetto intero minimizzato. Nella tavola i disegni di Lucide vengono da `lucide-static`,
stessa versione e stessa licenza.

| Pacchetto | Licenza | Icone | |
|---|---|---|---|
| `lucide` 1.47.0, 2026-09-17 | ISC, e MIT per le icone che vengono da Feather | 1848 moduli d'icona | ✅ scelto |
| `@tabler/icons` 3.48.0, 2026-09-22 | MIT | 5166 a tratto e 1054 piene | confrontato |
| `@phosphor-icons/core` 2.1.1, 2024-03-29 | MIT | — | fuori: nessuna uscita dal 2024 |
| `@heroicons/vue` 2.2.0, 2024-11-18 | MIT | — | fuori: nessuna uscita dal 2024 |

```bash
python -c "import json,urllib.request as u; g=lambda x: json.load(u.urlopen(u.Request(x,headers={'User-Agent':'harness'}))); [print(n, (lambda d: (d['dist-tags']['latest'], d['time'][d['dist-tags']['latest']][:10], d['versions'][d['dist-tags']['latest']].get('license')))(g('https://registry.npmjs.org/'+n.replace('/','%2F')))) for n in ['lucide','@tabler/icons','@phosphor-icons/core','@heroicons/vue']]"
python -c "import json,urllib.request as u; f=lambda p: json.load(u.urlopen(u.Request('https://data.jsdelivr.com/v1/packages/npm/'+p+'?structure=flat',headers={'User-Agent':'harness'})))['files']; print(sum(n['name'].startswith('/dist/esm/icons/') and n['name'].endswith('.mjs') for n in f('lucide@1.47.0')), sum(n['name'].startswith('/icons/outline/') for n in f('@tabler/icons@3.48.0')), sum(n['name'].startswith('/icons/filled/') for n in f('@tabler/icons@3.48.0')))"
python -c "import json,urllib.request as u; f=json.load(u.urlopen(u.Request('https://data.jsdelivr.com/v1/packages/npm/lucide@1.47.0?structure=flat',headers={'User-Agent':'harness'})))['files']; i=[x for x in f if x['name'].startswith('/dist/esm/icons/') and x['name'].endswith('.mjs')]; print(len(i), sum(x['size'] for x in i), [x['size'] for x in f if x['name']=='/dist/umd/lucide.min.js'])"
```

## Le domande del brainstorming, una alla volta — proposte del coordinatore, non decisioni

Ogni domanda a parole di tutti i giorni, due o tre opzioni, il consiglio in una riga: è la forma che il proprietario vuole.
⛔ Versioni, librerie e stato dell'arte si **verificano quel giorno** alle fonti primarie, non si prendono da qui. Le domande
5 e 6 portano una **dipendenza nuova**: sono sue per costruzione.

| # | Domanda | Opzioni | Lettura d'oggi |
|---|---|---|---|
| **1** | che cosa vuol dire *«da Agentic OS»*, e quanto è grande questo lavoro? | **A** le fondamenta: il linguaggio visivo, i token a strati, i temi, il dock vestito coi nostri token, e i componenti che **oggi** hanno la seconda occorrenza. **B** anche il kit dei pilastri che verranno. **C** solo il linguaggio e i token, niente estrazione | **A**: rispetta la regola della seconda occorrenza, già approvata, e rende coerente l'app di oggi; **B** disegna componenti che nessuno usa ancora |
| 2 | la direzione visiva del «Jarvis» | tre tavole a confronto, **nel browser** col visual companion: per esempio un HUD scuro con un accento luminoso e linee sottili, uno strumento professionale sobrio, il vetro traslucido del sistema | è una domanda **visiva**: si mostra, non si descrive |
| 3 | i temi | **A** uno scuro, con i token pronti per un secondo. **B** scuro e chiaro subito. **C** più «pelli» a scelta | **A**, con l'alto contrasto del sistema e il movimento ridotto come **vincoli** di G20, non come temi |
| 4 | il dock | **A** i nostri token vestono anche `dockview`, con un tema nostro sulle sue variabili `--dv-…`. **B** `themeAbyss` resta | **A**: oggi sono due linguaggi nella stessa finestra |
| 5 | i caratteri | **A** quelli di sistema. **B** un carattere incluso nel pacchetto, con licenza aperta e niente rete a runtime | **dipendenza nuova, del proprietario**; incrocia N-2 di E187 |
| 6 | le icone | **A** nessun set, poche SVG nostre. **B** un set aperto | **dipendenza nuova, del proprietario** |
| 7 | dove si guardano i componenti | **A** nella SPA stessa, contro il core finto, come oggi. **B** una vetrina a sé, sul genere di Storybook | **A**: la **B** è una dipendenza e un secondo mondo da tenere verde |
| 8 | come si cambia vista — ✅ **aggiunta alla ripresa del 2026-09-23**, e mancava: oggi la barra cambia vista con tre pulsanti in fila, e il 2026-09-07 il proprietario disse che la navigazione a schede fra gli stati non gli piace e chiese creatività (la memoria dell'agente, in nessun documento del repository) | da mostrare nel browser | è una domanda **visiva**, e nessun documento ha deciso la forma del cambio di vista: la stella polare dice «la barra delle viste», non come |

## Vicoli ciechi e trappole della prima sessione — per chi riprende le tavole

- Il pannello browser dell'app **non fotografa** una pagina `file://`: la rende come istantanea statica e non accetta comandi.
  La pagina si serve con `python -m http.server <porta> --bind 127.0.0.1 --directory <cartella>` e si apre con `preview_start`;
  il server si spegne **per PID**, con `netstat -ano` e `taskkill //F //PID`.
- Nel pannello lo `zoom` di una regione non è supportato, e uno screenshot preso nella stessa chiamata di uno scorrimento può
  mostrare la schermata di prima: si scorre il contenitore `.main` della cornice del companion con JavaScript, si aspetta un
  attimo, e lo screenshot va in una chiamata a sé; la finestra emulata a 800×500 dà la scala uno a uno.
- Lo script del companion su Windows **resta in primo piano**: si lancia in sottofondo, e l'indirizzo con la chiave sta in
  `state/server-info` della sua cartella di sessione. Con `--project-dir` sul repository le tavole diventerebbero file non
  tracciati: `.superpowers/` non è ignorato.
- La **prima** sonda dei raggi non vedeva proprio il caso segnalato dal proprietario — l'angolo interno stava esattamente sul bordo
  dell'arco esterno — e il commento di `sonda-raggi.js` dice la cura: una sonda nuova si prova **anche** sul caso che ha fatto
  nascere la regola.
- Il disegno che illustra una regola la deve rispettare: il riquadro attorno al disegno dei raggi la violava, ed è stato tolto.
- Un `cd` in una sottocartella resta per le chiamate dopo: percorsi assoluti, o `cd` alla radice in testa a ogni comando.
- `document.fonts.check()` **non** dice se un carattere è in uso: risponde «sì» anche per un nome che nessun `@font-face`
  dichiara. L'oracolo è lo stato dei `FontFace` in `document.fonts`, `loaded` — è la prima prova di `sonda-caratteri.js`.
- La pagina del companion non serve i file delle sonde: si lanciano sull'**anteprima privata**, dove stanno accanto alla
  tavola, e il frammento spinto al companion è lo stesso byte per byte. Sulla pagina del companion basta contare i
  `FontFace` caricati.
- La **scatola di scelta** del companion è arrotondata anche lei — 12, con una letterina da 6 — e la regola dei raggi vale
  anche lì: `gen_icons_v1.py` le dà 20 di raggio, perché la letterina sta a 2 + 12 px dall'angolo, e 30 px sotto, perché gli
  angoli da 28 della Home e del kit restino lontani dal suo. La prima forma di `sonda-raggi.js` guardava solo dentro la Home e
  il kit, e non l'ha vista.
- Un disegno **copiato** in un file del repository si porta dietro la sua licenza; uno caricato dalla rete no.

## Decisioni prese dal coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | la consegna nasce **qui**, al percorso del futuro disegno | la decisione 3 della consegna d'avvio del 2: il repo tiene lo stato in file tracciati, si lavora da due macchine, e il puntatore vive nella sola §6. Costo: una rinomina |
| 2 | il nome **senza numero** di sotto-progetto | la roadmap non ne ha uno per il design system, e darglielo è del proprietario. Costo: una rinomina |
| 3 | i commit **senza** `Co-Authored-By` | `CLAUDE.md` dice «senza co-autore»; la direttiva di sistema chiede il contrario, e la divergenza è portata al proprietario, come in ogni sessione di questo repository. Costo: un `--amend` |
| 4 | il visual companion lavora nello **scratchpad** della sessione, con `--project-dir`, e non nel repository | `.superpowers/` non è ignorato da git — `git check-ignore -v .superpowers/brainstorm/x.html` non rende nulla — e le tavole sarebbero file non tracciati nell'albero. Costo: le tavole non viaggiano fra le macchine, e per questo **l'approvata** si salva nel repository |
| 5 | ogni tavola si **verifica prima di mostrarla**: un'anteprima privata servita in locale, i contrasti WCAG calcolati dal generatore, e sonde nel browser sul testo tagliato, sugli elementi che sbordano e sui raggi concentrici | *«curato nel minimo dettaglio»*: la prima tavola aveva tre difetti visibili che nessuno avrebbe visto leggendo il codice. Costo: qualche minuto per tavola |
| 6 | la tavola approvata si salva con una **cornice nostra**, non con quella del plugin del companion | il file resta nostro e si apre senza il plugin. Costo: la cornice minima si tiene coerente a mano |
| 7 | le parole del proprietario **testuali** nelle risposte, e l'immagine di riferimento **non committata** | è il lavoro di un altro, e il repository è pubblico; gli spunti che se ne prendono stanno a parole nella risposta 2. Costo: se serve di nuovo, la si chiede al proprietario |
| 8 | la domanda 4, il dock, **non posta**: la risposta 5 la decide | con due temi `themeAbyss`, solo scuro, non può restare, e chiedere una cosa già determinata sarebbe una domanda di rito. Costo: se il proprietario voleva `themeAbyss` nel solo tema scuro, lo dice e si riapre |
| 9 | nel confronto dei caratteri la coppia **A** è quella **misurata** più vicina alla tavola approvata, e B e C cambiano un carattere solo | la prima forma della tavola aveva come A Inter + Barlow Semi Condensed, scelta a occhio, e Geist + Barlow non c'era: la sonda l'ha smentita **prima** che la tavola si mostrasse. Costo: nessuno |
| 10 | la tavola approvata **si aggiorna** coi caratteri scelti, e non nasce un secondo file | una tavola coi caratteri di Windows mentirebbe sullo stile di oggi, e le tavole nuove partono da lì; com'era sta nella storia di git, col comando nella risposta 8. Costo: chi cerca la tavola della risposta 4 la trova in `8c0bbe6` |
| 11 | la tavola approvata porta anche le icone scelte, e con esse la **licenza di Lucide**, copiata parola per parola in testa al file | i disegni sono **copiati** nel file, e la licenza ISC chiede l'avviso *«in all copies»*; una parte delle icone viene da Feather, sotto MIT, e la licenza lo dice; il repository è pubblico. I caratteri invece non sono copiati: arrivano da jsDelivr. Costo: una cinquantina di righe in testa alla tavola |
| 12 | `sonda-raggi.js` guarda anche le **scatole di scelta** del companion | la tavola dei caratteri si è mostrata con un angolo di 28 accanto a uno di 12, e con la letterina della scatola non concentrica: difetti della scatola del companion e non del nostro stile, ma la regola del proprietario vale per tutta la tavola, e il proprietario l'ha saputo alla tavola dopo. La sonda allargata li ha trovati sulla tavola dei caratteri — sei, due per scatola — e sulla tavola delle icone prima di mostrarla. Costo: nessuno; sulla tavola approvata, che non ha scatole, non cambia nulla |

## Decisioni aperte, del proprietario

- ⛔ **la domanda 8, come si cambia vista — la prossima e l'ultima, da porre nel browser**; la 4 è caduta con la risposta 5;
- se il design system prenda un **numero**: nella tabella delle decisioni della stella polare, o una riga della roadmap;
- **AUD-004**, che sbarra il secondo tempo della §6, il sotto-progetto 13;
- le voci registrate della tabella qui sopra.

## Prossimo passo, eseguibile

1. Aprire la sessione **nella cartella del repo**; `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa
   è il commit di questa chiusura o uno successivo. Sull'altra macchina, prima i comandi di *«Da sapere subito»*.
2. La lettura obbligatoria di `CLAUDE.md`; poi, **per prima cosa**, la CI di `8c0bbe6`, `35708f5` e di questa chiusura, in corsa
   alla chiusura, coi comandi della riga «la CI».
3. **Questo file, per intero**, e la tavola approvata, `2026-09-22-design-system-tavole/stile-approvato.html`, aperta nel
   browser. La stella polare **a pezzi**, col `grep` sulla domanda del momento, e per intero prima di scrivere il disegno —
   la risposta 7. Le skill: `anthropic-skills:decision-principles`,
   `anthropic-skills:session-resume` su questo file, `anthropic-skills:dev-discipline`, `anthropic-skills:dev-communication`, e
   `superpowers:brainstorming` — il percorso è **architetturale**, già detto al proprietario.
4. Al proprietario **la domanda 8**, come si cambia vista: è **visiva** e va nel browser, coi pezzi della stella polare sulle
   viste letti prima — la risposta 7: il visual companion si riavvia con `--project-dir`
   sullo scratchpad della sessione nuova, in sottofondo, e le tavole nuove partono dalla approvata, cioè da `gen_style_v5.py`. Il
   proprietario ha accettato il companion il 2026-09-23 — *«sì, usa il browser»* — e se la skill vuole di nuovo l'offerta, è una
   riga.
5. Dopo l'ultima domanda: gli approcci, poi il **disegno a sezioni**, una approvata per volta — i token a strati coi due temi,
   la regola dei raggi, il kit e dove vive, il dock, la cornice della finestra, le voci registrate della tabella qui sopra, e le
   sonde che rendono verificabile ogni regola, contrasto e raggi. Poi il disegno scritto al posto di questo file, questo testo in
   archivio **parola per parola**, e il piano in una sessione nuova.
6. Alla chiusura di ogni sessione: questo file aggiornato sul posto, `bash scripts/check-docs.sh`, commit, push.

## Come tornare operativi

```bash
git fetch --all --prune && git status -sb && git log --oneline -3
bash scripts/check-docs.sh
bash scripts/gate.sh
bash <cartella della skill brainstorming>/scripts/start-server.sh --project-dir "<scratchpad della sessione>" --open
```

Su questa macchina la cartella della skill è
`C:\Users\zagor\.claude\plugins\cache\claude-plugins-official\superpowers\6.3.0\skills\brainstorming`; sull'altra si
cerca, e la versione del plugin può essere un'altra.
