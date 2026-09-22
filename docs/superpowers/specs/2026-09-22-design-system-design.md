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
**primo tempo** del prossimo passo della §6: manca il brainstorming, che comincia dalla **domanda 1** qui sotto, senza risposta.

## ⛔ Da sapere subito

**Niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato, nessun server acceso,
nessun subagente; il brainstorming **non è cominciato** e nessun file della GUI è stato toccato.

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

- la **domanda 1**, e a seguire le altre, una per messaggio;
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
4. Il visual companion della skill offerto **in un messaggio a sé**: la domanda 2 è visiva, e la consegna d'avvio del 2 già
   diceva che i wireframe si mostrano nel browser.
5. Al proprietario la **domanda 1**, poi le altre, una per messaggio.
6. Alla chiusura: riscrivere questo file sul posto, spostarne il testo in archivio parola per parola, muovere il puntatore della
   §6 se cambia, `bash scripts/check-docs.sh`, commit, push.

## Come tornare operativi

```bash
git fetch --all --prune && git status -sb && git log --oneline -3
bash scripts/check-docs.sh
bash scripts/gate.sh
```
