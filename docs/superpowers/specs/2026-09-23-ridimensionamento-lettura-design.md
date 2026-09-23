# Il ridimensionamento della lettura — la consegna dell'avvio

⚠️ **QUESTO FILE È NATO COME CONSEGNA, il 2026-09-23**, della sessione che ha scritto il
[disegno del design system](2026-09-22-design-system-design.md): chiudendola, il proprietario ha dato un **mandato** per la
sessione successiva, da fare **prima** del piano del design system. La sessione che lo esegue legge questo file **per intero**;
alla propria chiusura lo **riscrive sul posto** come verbale dei tagli, e sposta questo testo **parola per parola** in
`docs/archivio/consegna-avvio-ridimensionamento-lettura.md`, coi soli link riscritti per la cartella — il viaggio delle altre
consegne.

⚠️ **Non è una spec e non è un disegno.** Il prossimo passo sta nella §6 del [compendio](../../COMPENDIO.md), in un posto solo.

## Il mandato, con le parole del proprietario

> *«nella prossima sessione prima del piano, vorrei ridimensionare i file necessari agli handoff, documentazione, e di lettura
> obbligatoria/opzionale perchè attualmente sono molto pesanti e occupano 300/400k token molto velocemente»* — 2026-09-23

| Famiglia | Che cosa vuol dire qui |
|---|---|
| i file **degli handoff** | le consegne e i diari: i *«Come si riprende»*, le chiusure numerate dei piani, le consegne che diventano disegni |
| la **documentazione** | `docs/` in generale: i disegni, i piani, `HANDOFF.md`, `riferimenti.md` |
| la lettura **obbligatoria e opzionale** | `CLAUDE.md`, il compendio e i due pezzi dell'audit; la tabella della §12 del compendio; ciò che ogni chiusura detta di leggere |

## ⛔ È la terza volta: prima di proporre, i precedenti

La regola di `CLAUDE.md`: *un'idea nuova può essere già stata scartata* — si guarda dove un taglio è già stato valutato, e perché.

| Quando | Che cosa è stato fatto | Dove |
|---|---|---|
| 2026-08-28 | lo **sfoltimento del compendio**, e il **tetto in byte** che `check-docs.sh` gli mette | il [disegno](2026-08-28-sfoltimento-compendio-design.md); in `riferimenti.md` la sezione *«Sfoltimento del compendio — 2026-08-28: come si misura il costo di apertura sessione»* |
| 2026-09-09 | il mandato della **decisione 26** della [stella polare](2026-09-07-direzione-gui-design.md) — *«… appena inizia la sessione ne vengono occupati 350k»* — eseguito in **cinque tagli, tutti A** (decisioni 27–31): la cronaca della stella polare, la testa dell'audit, il riquadro delle voci aperte e il puntatore della §6 del compendio, i verbali dentro `CLAUDE.md`; `AVVIO-CHAT.md` resta com'è (decisione 32) | la tabella delle decisioni della stella polare; i testi usciti in [`archivio/lettura-di-apertura-storico.md`](../../archivio/lettura-di-apertura-storico.md) e [`archivio/stato-storico.md`](../../archivio/stato-storico.md) |
| 2026-09-11 | la **lista di lettura** di una chiusura, prezzata **a pezzi**: il diario delle chiusure 1–7 del piano della parte 1 in archivio, e la lista da 182 086 a 136 633 token. I **due disegni della GUI non toccati**, perché sono materia prima del piano (gotcha #119); prezzarli sezione per sezione fu un'opzione **non presa** | in `riferimenti.md`, *«La lista di lettura di una chiusura — 2026-09-11: come si prezza, e il verbale»*; [`archivio/diario-spike-guscio-storico.md`](../../archivio/diario-spike-guscio-storico.md) |

**Il metodo è già scritto nel repository.** La §6 del compendio: *«il metodo — misurare prima, ogni taglio in A/B, niente si
cancella, i puntatori in una casa sola — vale per ogni sfoltimento futuro»*. `CLAUDE.md`: un verbale va in `docs/archivio/`, un
numero misurato si scrive col comando, e i fine-riga sono misti per file. Le due sezioni di `riferimenti.md` qui sopra: si misura
**a pezzi e non a file**, e `tiktoken` è il tokenizzatore di OpenAI, quindi ogni conto è un **limite inferiore**.

## La misura di oggi — la base da cui partire

Il 2026-09-23, con `cl100k_base` di `tiktoken`, limite inferiore. Il comando, e per i due pezzi dell'audit le righe le dà
`grep -n 'NON hanno un numero AUD\|La disciplina, in cinque passi' docs/audit-2026-08-27.md`:

```bash
python -c "import io,tiktoken; e=tiktoken.get_encoding('cl100k_base'); [print(len(e.encode(io.open(p,encoding='utf-8').read())), p) for p in ['CLAUDE.md','docs/COMPENDIO.md','docs/superpowers/specs/2026-09-22-design-system-design.md','docs/archivio/consegna-avvio-brainstorming-design-system.md','docs/superpowers/specs/2026-09-07-direzione-gui-design.md','docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md','docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md','docs/HANDOFF.md','docs/riferimenti.md']]"
```

| Pezzo | Token | Come si legge oggi |
|---|---|---|
| `CLAUDE.md`, il compendio, i due pezzi dell'audit | 4 554 + 33 598 + 2 509 | **obbligatoria**, a ogni sessione |
| il disegno del design system | 26 746 | per intero, dalla sessione del suo piano |
| la sua consegna in archivio | 28 333 | non obbligatoria |
| la stella polare | 60 326 | a pezzi; per intero sul fronte GUI |
| il disegno del 2 | 37 781 | a pezzi |
| il piano della parte 2 | **738 284** | mai intero: a sezioni |
| `HANDOFF.md` | 123 483 | a sezioni |
| `riferimenti.md` | 85 708 | a sezioni |

**Fuori dal repository**, su questa macchina sola: la memoria dell'agente — l'indice `MEMORY.md`, circa 1 600 token a ogni
sessione dopo la compattazione di oggi, e i suoi file, circa 71 000 in tutto, letti quando servono, i due più grandi circa 21 000
e 14 000 — e le skill che una sessione invoca, circa 22 000 per il gruppo di sempre.

⚠️ **La sessione che ha scritto il disegno, misurata su sé stessa** col contatore dell'app alla chiusura: **488 346** token, di
cui **419 699** nei messaggi. La lettura d'apertura, obbligatoria più il diario, ne vale circa 70 000; il resto è **lavoro** — le
verifiche, e un disegno di 82 KB scritto a frammenti e unito, che entra nel contesto più di una volta. 🔶 **Dedotto:** una parte
del peso non è ciò che si legge ma **la taglia di ciò che si scrive**, quindi anche la **forma** dei documenti è materia di
questo mandato.

## I candidati — da misurare e da proporre uno per volta in A/B, non decisi qui

| # | Candidato | Perché |
|---|---|---|
| 1 | la **lista di lettura** che l'ultima chiusura detta, a cominciare da quella della sessione del piano del design system | il 2026-09-11 il peso vero era lei, non la lettura obbligatoria |
| 2 | il **diario delle chiusure** del piano della parte 2 — il sotto-progetto 2 è chiuso | la stessa specie del taglio del 2026-09-11: cronaca di un piano eseguito |
| 3 | il **disegno del design system** e i due della GUI, **sezione per sezione** | l'opzione non presa del 2026-09-11. ⚠️ Sono materia prima dei piani: si taglia solo dove si rilegge **cronaca** |
| 4 | `HANDOFF.md` e `riferimenti.md` | si leggono a sezioni, ma i rimandi li aprono spesso |
| 5 | la **forma** dei documenti: i richiami datati che si accumulano in testa, le citazioni ripetute, le tabelle che ricopiano | il 🔶 della misura qui sopra |
| 6 | la **memoria dell'agente** e le **skill** di ogni sessione | stanno fuori dal repository e su una macchina sola: si misurano e se ne parla col proprietario, non si toccano da qui |

## La prima domanda, per il proprietario

**Il metodo resta quello delle tre volte prima?** **A** — sì: niente si cancella, la cronaca va in archivio parola per parola,
il documento vivo porta lo stato, ogni taglio in A/B. **B** — si possono anche **cancellare** o **fondere** file che non servono
più, ciascuno col suo A/B. Il consiglio: **A**, perché ha retto tre volte, l'archivio non si legge e un file archiviato si
ritrova, uno cancellato no.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-23

⛔ **DA SAPERE SUBITO: niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato, nessun
server acceso, nessun subagente, nessun codice toccato.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| **i commit della sessione** | `git log --oneline ab39f39..HEAD`: il disegno del design system scritto sul posto, la voce del numero scelta **A**, e questa chiusura |
| **la CI** | `386fc5c` verde sui due job alla chiusura; `ee3810e` e questa chiusura erano **in corsa**, e si leggono **per prime**, coi due comandi del punto 3 della quarantunesima chiusura del [piano della parte 2](../plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md) |
| **cancello** | `bash scripts/gate.sh` → `GATE GREEN` all'apertura e prima del commit del disegno; `bash scripts/check-docs.sh` → `OK` a ogni commit |
| **il design system** | disegno **scritto**, rilettura del proprietario **aperta**, il numero scelto **A**: la sua sezione *«Come si riprende»* dice il resto, **dopo** questo mandato |

**Il compito della sessione successiva**, in ordine:

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`; poi la CI, per prima.
2. La lettura obbligatoria di `CLAUDE.md`, finché questo mandato non la cambia; poi **questo file, per intero**. ⛔ **Non** il
   disegno del design system, **non** la sua consegna in archivio, **non** la stella polare: si **misurano**, non si leggono.
3. I precedenti, **a pezzi e col `grep`**: le righe 26–32 della tabella delle decisioni della stella polare; le due sezioni di
   `riferimenti.md` nominate qui sopra; la testa del disegno del 2026-08-28.
4. La **prima domanda** al proprietario; poi le **misure a pezzi** dei candidati, e un taglio per volta in A/B col consiglio.
5. Ogni taglio approvato: uno script nello scratchpad con ancore asserite, i fine-riga di ciascun file conservati e contati dopo,
   il testo tolto in `docs/archivio/` parola per parola e provato con `diff` contro `git show HEAD:<file>`; `check-docs.sh` e
   `gate.sh`; commit e push, senza co-autore.
6. Alla chiusura: questo file riscritto sul posto come verbale, con la misura **prima e dopo**; il puntatore della §6 del
   compendio torna al **design system** — la rilettura del proprietario, poi il piano in una sessione nuova.

Le skill: `anthropic-skills:decision-principles`, `anthropic-skills:session-resume` su questo file,
`anthropic-skills:dev-discipline` e `anthropic-skills:dev-communication`; `superpowers:brainstorming` se la prima domanda
cambia il metodo.

## Punto fermo — le risposte del proprietario, 2026-09-23

⚠️ **Scritto a metà della sessione che esegue il mandato**, perché una sessione che muore non faccia ripresentare
niente. Alla chiusura questo file si riscrive come verbale, e il testo della consegna va in archivio da
`git show 61c8450:docs/superpowers/specs/2026-09-23-ridimensionamento-lettura-design.md`, com'era.

| | La domanda | La risposta | Dove sta |
|---|---|---|---|
| 1 | il metodo resta quello delle tre volte prima? | **A** — niente si cancella, la cronaca in archivio parola per parola, il documento vivo porta lo stato, ogni taglio in A/B | — |
| 2 | taglio 1 — le tre note più grandi della memoria dell'agente diventano regole corte, e la storia va parola per parola in una cartella d'archivio **fuori** dalla memoria, che non si carica | **A** | fuori dal repository, su questa macchina |
| 3 | taglio 2 — la cronaca delle correzioni esce dal compendio: sedici pezzi, fuori dalla §6 e dalla sua tabella delle voci aperte, che sono decisioni del proprietario | **A** | eseguito: `4a7111d` |
| 4 | taglio 3 — i due comandi della CI da leggere da terra passano in `docs/porta-di-qualita.md`, accanto alla riga sulla CI, e il disegno del design system rimanda lì | **A** | eseguito: `f020cd9` |
| 5 | taglio 4 — nei piani futuri resta solo l'ultima chiusura, le vecchie in archivio? | il proprietario ha risposto: *«Rivedi, ottimizza e migliora per intero claude.md (compresa questa cosa) (anche secondo i principi di decision-principles)»*; la revisione, presentata sezione per sezione, approvata **A** | eseguita: il commit che aggiunge questa riga; `CLAUDE.md` com'era in `archivio/lettura-di-apertura-storico.md` |

**La mappa misurata prima di proporre** — `cl100k_base`, limite inferiore: la lettura obbligatoria vale circa 41 000 token
(il compendio circa 33 700); la sessione che ha scritto il disegno del design system ne ha spesi circa 70 000 in letture su 488 346.
Il peso che cresce senza freno sta nelle **note di memoria** (tre file, circa 41 000 token), nel **diario delle chiusure** del
piano della parte 2 (circa 183 000 token su 740 000) e nel **lavoro** di ogni sessione. `HANDOFF.md` (circa 124 000) si legge un
gotcha alla volta, e la mediana di un gotcha è sotto i 500 token: il consiglio è **lasciarlo**. I comandi e il verbale prima e
dopo vanno in `riferimenti.md` alla chiusura, nella sezione *«Sfoltimento del compendio»*.
