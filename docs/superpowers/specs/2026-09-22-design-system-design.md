# Il design system della GUI — la consegna dell'avvio

⚠️ **QUESTO FILE È NATO COME CONSEGNA, il 2026-09-22, della sessione che ha chiuso il residuo del compito 17 del piano
della parte 2 e NON ha fatto il brainstorming:** il proprietario lo vuole in una sessione nuova — la sua regola *«una fase
per sessione»*, decisioni 39 e 48 della [stella polare](2026-09-07-direzione-gui-design.md) — e riprendibile anche
dall'altra macchina. La sessione del brainstorming legge questo file **per intero**, e alla propria chiusura lo **riscrive
sul posto** — come consegna delle sezioni approvate, o come disegno — e sposta questo testo **parola per parola** in
`docs/archivio/consegna-avvio-brainstorming-design-system.md`, coi soli link riscritti per la cartella: il viaggio delle
consegne della [knowledge base](../../archivio/consegna-brainstorming-knowledge-base.md) e del
[sotto-progetto 2](../../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md).

⚠️ **Non è una spec e non è un disegno.** Il prossimo passo sta nella §6 del [compendio](../../COMPENDIO.md), in un posto solo.

## Stato in una riga

Il sotto-progetto 2 è chiuso — il piano della parte 2 per intero, residuo del compito 17 compreso — e il design system è il
**primo tempo** del prossimo passo della §6: il brainstorming è **cominciato il 2026-09-23**, e le risposte avute stanno in
*«Le risposte del proprietario, una per domanda»* qui sotto; si riprende dalla prima domanda senza risposta.

## ⛔ Da sapere subito

**Il brainstorming è a metà, e nient'altro.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato,
nessun server acceso, nessun subagente; nessun file della GUI è stato toccato. Le risposte del proprietario vivono nella tabella
*«Le risposte del proprietario, una per domanda»*, e una domanda senza riga lì non ha risposta: si ripone, non si deduce.

⚠️ **Sull'altra macchina** — il clone con `core.autocrlf` `true`, albero `w/crlf` — tre cose prima di credere a un verde o a
un rosso: un Node che soddisfi `node -p "require('./gui/package.json').engines.node"`; `cargo audit --version` **0.22.2**,
prerequisito del cancello (D68 del [piano della parte 2](../plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md)); e
un'etichetta di fine-riga scritta in un documento è la colonna `w/…` di **chi l'ha misurata**: si rimisura con
`git ls-files --eol`, non si crede (E51, E72, E224 dello stesso piano).

## Stato del repo alla chiusura, coi comandi che lo rifanno

| | Comando | Atteso |
|---|---|---|
| ramo | `git fetch --all --prune`, poi `git status -sb` | `## main...origin/main`, niente sotto |
| i commit di questa sessione | `git log --oneline c828f2d..HEAD` | E237 (`c5b1b96`), E236 (`e00cf5e`), la quarantatreesima chiusura (`abd4d13`) e questa consegna |
| cancello | `bash scripts/gate.sh` | `GATE GREEN`, rilanciato prima di ogni commit di oggi — si rilancia, non si cita |
| documenti | `bash scripts/check-docs.sh` | `OK` |
| la CI | i due comandi del punto 3 della quarantunesima chiusura del piano della parte 2 | due job per corsa, entrambi `success`: così alla chiusura per `c828f2d`, `c5b1b96`, `e00cf5e` e `abd4d13` |
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
⛔ **Il resto della stella polare NON è letto**, e la §12 del compendio la vuole **per intero** da chi riprende il fronte GUI:
è la prima lettura della sessione nuova, dopo questo file.

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

## Decisioni prese dal coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | la consegna nasce **qui**, al percorso del futuro disegno | la decisione 3 della consegna d'avvio del 2: il repo tiene lo stato in file tracciati, si lavora da due macchine, e il puntatore vive nella sola §6. Costo: una rinomina |
| 2 | il nome **senza numero** di sotto-progetto | la roadmap non ne ha uno per il design system, e darglielo è del proprietario. Costo: una rinomina |
| 3 | i commit **senza** `Co-Authored-By` | `CLAUDE.md` dice «senza co-autore»; la direttiva di sistema chiede il contrario, e la divergenza è portata al proprietario, come in ogni sessione di questo repository. Costo: un `--amend` |

## Decisioni aperte, del proprietario

- le domande dalla **3** in poi, una per messaggio: la 3 sono **i temi**;
- se il design system prenda un **numero**: nella tabella delle decisioni della stella polare, o una riga della roadmap;
- **AUD-004**, che sbarra il secondo tempo della §6, il sotto-progetto 13;
- le voci registrate della tabella qui sopra.

## Prossimo passo, eseguibile

1. Aprire la sessione **nella cartella del repo**; `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa
   è il commit di questa consegna o uno successivo. Sull'altra macchina, prima i comandi di *«Da sapere subito»*.
2. La lettura obbligatoria di `CLAUDE.md`, poi la CI dei commit del 2026-09-22, coi comandi della riga «la CI» qui sopra.
3. **Questo file, per intero**, poi la **stella polare per intero**. Le skill: `anthropic-skills:decision-principles`,
   `anthropic-skills:session-resume` su questo file, `anthropic-skills:dev-discipline`, `anthropic-skills:dev-communication`,
   e `superpowers:brainstorming`.
4. Il visual companion della skill offerto **in un messaggio a sé**, alla **prima domanda che si capisce meglio vedendo** e non
   in apertura: così dice `superpowers:brainstorming` letta il 2026-09-23 (*just-in-time*); la consegna d'avvio del 2 già diceva
   che i wireframe si mostrano nel browser.
5. Al proprietario la prima domanda senza risposta in *«Le risposte del proprietario»*, poi le altre, una per messaggio.
6. Alla chiusura: riscrivere questo file sul posto, spostarne il testo in archivio parola per parola, muovere il puntatore della
   §6 se cambia, `bash scripts/check-docs.sh`, commit, push.

## Come tornare operativi

```bash
git fetch --all --prune && git status -sb && git log --oneline -3
bash scripts/check-docs.sh
bash scripts/gate.sh
```
