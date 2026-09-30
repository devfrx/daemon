# Il metodo con la decision map — la consegna

> ⚠️ **Non è un disegno.** È la **consegna** del fronte aperto dal proprietario il 2026-09-30, tenuta al percorso del suo
> futuro disegno come vuole `CLAUDE.md` (*«Manutenzione della documentazione»*), finché il ticket sulla consegna della
> mappa non decide altro. Il merito delle decisioni vive nei ticket della [mappa](https://github.com/devfrx/daemon/issues/1),
> su GitHub; qui restano lo stato e il modo di riprendere. Le versioni di prima, con le loro correzioni, stanno in
> [`archivio/consegna-metodo-decision-map.md`](../../archivio/consegna-metodo-decision-map.md).

## Che cosa è stato deciso, il 2026-09-30

**Prima del compito 1** del [piano dei documenti della revisione della knowledge base](../plans/2026-09-29-knowledge-base-revisione-documenti.md),
il proprietario ha chiesto di passare la documentazione e il metodo di pianificazione e di lavoro dalla skill
`anthropic-skills:decision-map`: le decisioni aperte diventano **ticket** su GitHub Issues, una **mappa** fa da indice, e
ogni sessione ne chiude una. Cinque domande, una alla volta, col consiglio:

| # | Domanda | Risposta del proprietario |
|---|---|---|
| 1 | Che cosa passa dalla mappa? | **A** — solo ciò che è **da decidere**: il metodo nuovo e le sue decisioni in sospeso. ADR, compendio e il piano dei documenti restano come sono |
| 2 | Qual è la meta? | **A** — *«Si lavora con la decision map: `CLAUDE.md` dice come, e ogni decisione in sospeso del proprietario sta in un ticket, non più sparsa nei documenti.»* |
| 3 | Il censimento delle decisioni in sospeso: adesso o dopo? | **adesso**, prima di creare la mappa, con tanti aiutanti `sonnet` in parallelo, ognuno su pochi file e con poco contesto — *«non 3 agenti per 40 file.. sono troppo pochi»* |
| 4 | Quanto largo? | **A** — tutto: i file vivi coi segni, il racconto delle voci aperte nell'archivio, il codice |
| 5 | Che cosa si crea su GitHub, visto l'esito? | **A** — **solo la mappa del metodo** coi suoi nove ticket; le 114 decisioni trovate restano nella lista datata, e le sposta il ticket del trasloco con la regola del ticket sulle voci aperte |

Il censimento è girato come **workflow**. Il proprietario dice di aver impostato lui l'effort della sessione su *ultracode*, e che senza non si sarebbe potuto usare un workflow così — parole sue, 2026-09-30. Da qui non si verifica: i metadati della sessione mostrano l'effort di adesso, «max», e non quello di allora.

## Che cosa esiste adesso

| | Stato |
|---|---|
| la mappa | ✅ [*Il metodo: si lavora con la decision map*](https://github.com/devfrx/daemon/issues/1), creata il 2026-09-30, coi nove ticket come sub-issue e i blocchi; le quattro etichette della skill — il comando A |
| la frontiera | la dà il comando B, e non si ricopia qui: cambia a ogni ticket chiuso |
| GitHub | `devfrx/daemon` è **pubblico**: mappa e ticket si vedono da fuori |
| `gh` | la 2.101.0; `--parent` e `--blocked-by` alla creazione, `--add-blocked-by` alla modifica. La guida della skill dava incerto `--set-parent`: nel binario c'è `--parent` |
| un tracker valutato prima? | **no**: *tracker*, *GitHub Issues*, *backlog*, *sub-issue* e *decision-map* non comparivano in `docs/` né in `CLAUDE.md`, il 2026-09-30 |
| il censimento | ✅ fatto: **114** decisioni del proprietario aperte — la sezione qui sotto |
| la ricerca | ✅ fatta il 2026-09-30, nel [suo ticket](https://github.com/devfrx/daemon/issues/4): la risposta in breve nel commento, le fonti verificate in [`riferimenti.md`](../../riferimenti.md), nella sezione *«Il metodo con la decision map — ticket di decisione, ADR e documenti: la ricerca, 2026-09-30»* |
| il piano dei documenti | **pre-controllato, nessun compito eseguito**; si esegue **adesso**, un compito per sessione dal compito 1 — deciso dal proprietario il 2026-09-30 nel [suo ticket](https://github.com/devfrx/daemon/issues/2); il puntatore della §6 del compendio, e la voce **ER-8** del piano col suo richiamo |

```bash
# A — il corpo della mappa
gh issue view 1 --json title,body,subIssuesSummary
# B — la frontiera: aperti, non assegnati, senza blocchi aperti
for n in $(gh issue view 1 --json subIssues --jq '.subIssues.nodes[] | select(.state=="OPEN") | .number'); do gh issue view $n --json number,title,assignees,blockedBy --jq '"\(.number) assegnati=\(.assignees|length) bloccato-da=\([.blockedBy.nodes[]? | select(.state=="OPEN") | .number]|join(",")) \(.title)"'; done
```

## Il censimento

**La lista** sta in [`archivio/censimento-decisioni-2026-09-30.md`](../../archivio/censimento-decisioni-2026-09-30.md):
una fotografia datata, col modo in cui è stata fatta. Ogni voce si rilegge contro il repository prima di spostarla.

| | |
|---|---|
| che cosa ha letto | ogni riga coi segni `non pres`, `registrat`, `⏳`, `proprietari` nei 146 file vivi che li portano, nel racconto delle voci aperte in `docs/archivio/stato-storico.md` e nel codice: 2665 righe-segno |
| come | workflow `censimento-decisioni-in-sospeso`, corsa `wf_c901e460-e99`, sola lettura con agenti `Explore`: 86 lettori `sonnet`, un'unione `opus`, un verificatore `sonnet` per voce, un critico `opus` |
| esito | 162 voci: **114 aperte** del proprietario — 36 metodo e cancello, 39 spec e documenti, 29 prodotto e codice, 10 roadmap e fronti —; 36 già chiuse; 4 non sue; 8 erano i ticket del metodo, letti nella consegna scritta mentre il censimento girava. **46** delle 114 vengono da `docs/porta-di-qualita.md` |
| costo | ⚠️ **16,1 milioni di token**, 250 agenti, 39 minuti. La stima detta prima era **5-8 milioni**: ha sbagliato di due volte, perché le voci da verificare sono state 162 e non la sessantina attesa, e ogni verificatore ha cercato in tutto il repository |
| controllo a campione | il coordinatore, 8 voci su 8 vere; il numero di riga a volte spostato di qualche riga — la voce si ritrova cercando la frase |
| il segno su C009 | il controllo del harness ha segnalato nel suo testo una forma da istruzione, perché cita `settings.json`: letto, è una decisione del proprietario — quali plugin spegnere —, e nessuna istruzione è stata eseguita |

Lo script e il `journal.jsonl` della corsa vivono **solo su questa macchina**:
`C:\Users\zagor\.claude\projects\C--EVERYTHING-DEV-MY-REPOS-daemon\332372f1-c420-4da0-9bbd-cd7ed5ea2418\`, nelle cartelle
`workflows\scripts\` e `subagents\workflows\wf_c901e460-e99\`.

## La mappa, com'è nata

| Ticket | Tipo | Aspetta |
|---|---|---|
| [Il piano dei documenti si esegue adesso o aspetta il metodo nuovo?](https://github.com/devfrx/daemon/issues/2) | decisione | — |
| [Una decisione per sessione, sempre?](https://github.com/devfrx/daemon/issues/3) | decisione | — |
| [Come si tengono insieme, oggi, ticket di decisione, ADR e documenti?](https://github.com/devfrx/daemon/issues/4) | ricerca | — |
| [Come si combinano le cinque fasi con la mappa?](https://github.com/devfrx/daemon/issues/5) | decisione | la ricerca |
| [Le voci aperte nei documenti: che cosa diventa ticket, e che cosa resta?](https://github.com/devfrx/daemon/issues/6) | decisione | la ricerca |
| [Dove vive il prossimo passo?](https://github.com/devfrx/daemon/issues/7) | decisione | le cinque fasi |
| [Che cosa resta della consegna di fine sessione?](https://github.com/devfrx/daemon/issues/8) | decisione | le cinque fasi |
| [Il cancello deve controllare la mappa?](https://github.com/devfrx/daemon/issues/9) | decisione | le voci aperte |
| [Spostare nei ticket le decisioni in sospeso](https://github.com/devfrx/daemon/issues/10) | lavoro | le voci aperte |

Il testo mostrato al proprietario **prima** del censimento sta in
[`archivio/bozza-mappa-del-metodo.md`](../../archivio/bozza-mappa-del-metodo.md). Dopo il censimento sono cambiati tre
testi: il ticket del trasloco — prima «censire», ora «spostare» —, e il contesto dei ticket sulla sessione e sulle voci
aperte, che citano la lista.

## Le decisioni del coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | nove ticket sul metodo, e i blocchi della tabella qui sopra | sono le domande che si sanno porre oggi (la skill: *«si tickettano solo le domande che sai formulare»*); la ricerca viene prima delle cinque fasi e delle voci aperte perché il proprietario vuole lo stato dell'arte **prima** delle domande. Costo: un ticket in più o in meno si aggiunge o si chiude |
| 2 | il censimento oltre i 40 file di «non presa»: tutti i file vivi coi quattro segni, il racconto nell'archivio, il codice | una decisione in sospeso si scrive anche *«aspettano il proprietario»* o in una colonna *«Chi la chiude»*, senza la parola «presa». Costo: circa 45 agenti in più sui file piccoli |
| 3 | gli agenti del censimento sono di tipo `Explore` | non possono scrivere, e non ricevono `CLAUDE.md`: meno contesto a testa, come voleva il proprietario. Costo: le regole del progetto che servono stanno nel prompt |
| 4 | il puntatore della §6 cambia in una frase, e il piano dei documenti riceve la voce **ER-8** | il prossimo passo è cambiato; il compito 6 del piano riscrive il puntatore intero con `replace_pointer.py`, e senza la voce cancellerebbe il rimando senza che nulla diventi rosso. Costo: una riga nell'errata |
| 5 | la lista del censimento sta in **archivio**, e le otto voci C001…C008 non contano fra le 114 | è una fotografia datata, che non si aggiorna: la casa di ogni voce resta il suo documento finché il trasloco non la sposta; C001…C008 sono i ticket del metodo, già sulla mappa. Costo: nessuno |
| 6 | il 2026-09-30, la ricerca **intera** e verificata in una sezione datata di `riferimenti.md`; nel ticket la risposta in breve, col **permalink** al commit | `CLAUDE.md` vuole le fonti in `riferimenti.md`, e la mappa è un indice che non archivia; il permalink fissa il testo che il ticket riassume — la fonte [15] della ricerca. Costo: una sezione lunga in `riferimenti.md`, che non è lettura d'apertura |
| 7 | il 2026-09-30, la voce **ER-8** del piano riceve un **richiamo datato** invece di essere riscritta, e il puntatore nuovo della §6 **conserva** il rimando a questa consegna | ER-8 resta vera al compito 6, e un richiamo datato è la forma del repository per una voce che cambia a metà. Costo: una frase in coda alla cella |

## Come si riprende

⛔ **Da sapere subito:** niente è a metà, e tutto è pushato. Il 2026-09-30 si sono chiusi due ticket della mappa: il piano
dei documenti si esegue **adesso**, e la ricerca su ticket, ADR e documenti è fatta. Nessun ticket è assegnato. La
consegna di prima — le due sezioni riscritte — sta in [archivio](../../archivio/consegna-metodo-decision-map.md),
parola per parola.

1. La lettura d'apertura di `CLAUDE.md`; poi il **corpo della mappa** — il comando A —, dove *«Decisioni prese»* ha una
   riga per ticket chiuso.
2. ▶️ **Il prossimo passo proposto: il compito 1 del piano dei documenti** — il puntatore della §6 del compendio. Come si
   comincia lo dice il *«Come si riprende»* del [piano](../plans/2026-09-29-knowledge-base-revisione-documenti.md): la
   cartella del dispaccio, poi il costo detto al proprietario, e il suo sì. La sessione del compito legge anche la voce
   **ER-8** dell'errata, col richiamo del 2026-09-30.
3. **Oppure un ticket della mappa**, a scelta del proprietario: la frontiera è il comando B. Alla chiusura del 2026-09-30
   erano *«Una decisione per sessione, sempre?»*, *«Come si combinano le cinque fasi con la mappa?»* e *«Le voci aperte
   nei documenti: che cosa diventa ticket, e che cosa resta?»*; gli ultimi due partono dalla ricerca — il suo commento
   nel ticket, e le fonti in `riferimenti.md`. Chi prende un ticket se lo **assegna prima** di cominciare:
   `gh issue edit <numero> --add-assignee "@me"`.
4. Finché il piano non finisce, un ticket che cambia `CLAUDE.md` o `docs/COMPENDIO.md` si prova contro i blocchi che
   restano: `extract.py` copiato a mano dal piano, il `--check` di ciascun blocco col comando del *«Come si riprende»*
   del piano, e il comando C per il margine del compendio.
5. La chiusura di un ticket, nell'ordine della skill: il commento con la risposta, il ticket chiuso, una riga in
   *«Decisioni prese»* nel corpo della mappa, riletto subito prima di riscriverlo. Una risposta che rimanda a un file del
   repository si scrive **dopo** il push, col permalink al commit. Poi la chiusura della sessione secondo `CLAUDE.md`:
   questa sezione riscritta, quella di prima in archivio, il puntatore della §6 se il prossimo passo cambia, commit e push.

Il conteggio dei file coi segni del censimento — per il trasloco, che rilegge ogni voce contro il repository:

```bash
RE='non pres|registrat|⏳|proprietari'; { find docs -name '*.md' -not -path '*/archivio/*'; ls spikes/*.md spikes/*/*.md; echo docs/archivio/stato-storico.md; } | sort -u | while read f; do n=$(grep -c -E "$RE" "$f"); [ "$n" -gt 0 ] && echo "$n $f"; done | wc -l
```
