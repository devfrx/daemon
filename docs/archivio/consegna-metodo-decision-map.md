# Il metodo con la decision map — le consegne di prima

⚠️ **Vere il giorno in cui furono scritte, salvo dove una correzione dice altro.** Uscite dalla [consegna viva](../superpowers/specs/2026-09-30-metodo-decision-map-design.md) parola per parola, coi link riscritti per questa cartella.

## Il punto fermo di `a9e8265` — 2026-09-30, prima del censimento

⛔ **Correzione del 2026-09-30, detta dal proprietario.** Due frasi di questa consegna non erano sostenute da nulla: *«La modalità ultracode il proprietario l'ha offerta; il coordinatore l'ha lasciata spenta»* e la decisione 5 del coordinatore, *«ultracode resta spento»*, che dava la richiesta di parallelizzare per consenso al workflow. Il proprietario ha detto che l'effort della sessione l'aveva impostato lui su *ultracode*, e che senza non si sarebbe potuto usare un workflow come per il censimento. È una sua dichiarazione: da qui non si verifica, perché i metadati della sessione mostrano l'effort di adesso, «max», e non quello di allora.

# Il metodo con la decision map — la consegna

> ⚠️ **Non è un disegno.** È la **consegna** del fronte aperto dal proprietario il 2026-09-30, tenuta al percorso del suo
> futuro disegno come vuole `CLAUDE.md` (*«Manutenzione della documentazione»*), finché il ticket sulla consegna della
> mappa non decide altro. Il merito delle decisioni vivrà nei ticket della mappa, su GitHub; qui restano lo stato e il
> modo di riprendere.

## Che cosa è stato deciso, il 2026-09-30

**Prima del compito 1** del [piano dei documenti della revisione della knowledge base](../superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md),
il proprietario ha chiesto di passare la documentazione e il metodo di pianificazione e di lavoro dalla skill
`anthropic-skills:decision-map`: le decisioni aperte diventano **ticket** su GitHub Issues, una **mappa** fa da indice, e
ogni sessione ne chiude una. Quattro domande, una alla volta, col consiglio:

| # | Domanda | Risposta del proprietario |
|---|---|---|
| 1 | Che cosa passa dalla mappa? | **A** — solo ciò che è **da decidere**: il metodo nuovo e le sue decisioni in sospeso. ADR, compendio e il piano dei documenti restano come sono |
| 2 | Qual è la meta? | **A** — *«Si lavora con la decision map: `CLAUDE.md` dice come, e ogni decisione in sospeso del proprietario sta in un ticket, non più sparsa nei documenti.»* |
| 3 | Il censimento delle decisioni in sospeso: adesso o dopo? | **adesso**, prima di creare la mappa, con tanti aiutanti `sonnet` in parallelo, ognuno su pochi file e con poco contesto — *«non 3 agenti per 40 file.. sono troppo pochi»* |
| 4 | Quanto largo? | **A** — tutto: i file vivi coi segni, il racconto delle voci aperte nell'archivio, il codice; il costo stimato detto prima, 5-8 milioni di token quasi tutti `sonnet` |

La modalità *ultracode* il proprietario l'ha offerta; il coordinatore l'ha lasciata spenta — la decisione 5 qui sotto.

## Che cosa esiste adesso

| | Stato |
|---|---|
| GitHub | `devfrx/daemon` è **pubblico** e ha le issue attive — il comando A. ⏳ **Mappa e ticket NON ancora creati**: aspettano il censimento, e poi il sì del proprietario sulla lista |
| `gh` | la 2.101.0, collegata; `--parent` e `--blocked-by` alla creazione, `--add-blocked-by` alla modifica — il comando B. La guida della skill dava incerto `--set-parent`: nel binario c'è `--parent` |
| un tracker valutato prima? | **no**: *tracker*, *GitHub Issues*, *backlog*, *sub-issue* e *decision-map* non compaiono in `docs/` né in `CLAUDE.md` — il comando C, una parola alla volta |
| le citazioni di «non presa» | 40 file vivi, il 2026-09-30 — il comando D |
| il piano dei documenti | **pre-controllato, nessun compito eseguito**. Tocca `CLAUDE.md` e il compendio; nessuna ancora dei suoi blocchi cade nel puntatore della §6, e il margine del compendio era 8366 byte prima di questa sessione — il comando C del piano |
| il censimento | ⏳ il workflow `censimento-decisioni-in-sospeso`, corsa `wf_c901e460-e99`, lanciato in questa sessione: la sezione qui sotto |

```bash
gh repo view --json nameWithOwner,visibility,hasIssuesEnabled
gh issue create --help | grep -E 'parent|blocked'; gh issue edit --help | grep -E 'parent|blocked'
for w in 'GitHub Issues' 'tracker' 'backlog' 'sub-issue' 'decision-map'; do grep -rn -i -F "$w" docs CLAUDE.md --include='*.md' | head -3; done
grep -rlc 'non pres' docs --include='*.md' | grep -v /archivio/ | wc -l
```

## Il censimento

**Che cosa legge.** Ogni riga che contiene un segno — `non pres`, `registrat`, `⏳`, `proprietari` — nei file vivi di
`docs/` (fuori da `docs/archivio/`), in `spikes/*.md` e in `docs/archivio/stato-storico.md`, che dal 2026-09-09 tiene il
racconto delle voci ancora aperte: **146 file e 2564 righe-segno**, il 2026-09-30. Più il codice, dove *owner* quasi
sempre è il proprietario della GPU e non conta.

**Come.** Quattro passi, tutti in sola lettura, con agenti di tipo `Explore`:

| Passo | Chi | Che cosa |
|---|---|---|
| Leggere | 85 agenti `sonnet`, uno per pezzo di 1-3 file — i file enormi divisi per intervalli di righe —, più uno per il codice | ogni riga-segno letta con almeno otto righe intorno; ogni decisione in sospeso citata parola per parola, col file e la riga |
| Unire | un agente `opus` | la stessa decisione scritta in più file diventa una sola |
| Verificare | un agente `sonnet` per decisione | prova a dimostrare che è già chiusa, o che non è del proprietario; se non ci riesce, resta aperta |
| Cosa manca | un agente `opus` | cerca le decisioni scritte senza i segni, rilegge gli indici delle voci aperte, e manda a verificare ciò che trova |

**Dove sta.** Lo script: `C:\Users\zagor\.claude\projects\C--EVERYTHING-DEV-MY-REPOS-daemon\332372f1-c420-4da0-9bbd-cd7ed5ea2418\workflows\scripts\censimento-decisioni-in-sospeso-wf_c901e460-e99.js`;
i pezzi, argomento del workflow, nello scratchpad della sessione, `census-args.json`. ⚠️ **Tutti e due vivono solo su
questa macchina**, e il risultato di ogni agente sta nel `journal.jsonl` della corsa:
`C:\Users\zagor\.claude\projects\C--EVERYTHING-DEV-MY-REPOS-daemon\332372f1-c420-4da0-9bbd-cd7ed5ea2418\subagents\workflows\wf_c901e460-e99\`.

⏳ **L'esito non c'è ancora.** Quando torna, la lista verificata si scrive qui — o, se la mappa nasce, nei ticket, e qui
resta il rimando.

## La bozza della mappa — nove ticket sul metodo

La meta è la risposta 2. I ticket qui sotto sono le domande **sul metodo** che si sanno porre oggi; le decisioni in
sospeso del proprietario si aggiungono dopo il censimento, e se passano la ventina vanno in una **seconda mappa** — la
skill: una mappa con più di una ventina di ticket aperti ha la meta troppo larga.

| # | Ticket | Tipo | Aspetta |
|---|---|---|---|
| 1 | Il piano dei documenti si esegue adesso o aspetta il metodo nuovo? | decisione | — |
| 2 | Una decisione per sessione, sempre? | decisione | — |
| 3 | Come si tengono insieme, oggi, ticket di decisione, ADR e documenti? | ricerca | — |
| 4 | Come si combinano le cinque fasi con la mappa? | decisione | 3 |
| 5 | Le voci aperte nei documenti: che cosa diventa ticket, e che cosa resta? | decisione | 3 |
| 6 | Dove vive il prossimo passo? | decisione | 4 |
| 7 | Che cosa resta della consegna di fine sessione? | decisione | 4 |
| 8 | Il cancello deve controllare la mappa? | decisione | 5 |
| 9 | Censire le decisioni in sospeso e farne ticket | lavoro | 5 |

Il testo intero della mappa e dei nove ticket, parola per parola come mostrato al proprietario il 2026-09-30, sta in
[`archivio/bozza-mappa-del-metodo.md`](bozza-mappa-del-metodo.md). ⚠️ Il ticket 9 cambia col censimento:
il censimento è fatto adesso, quindi resta lo **spostamento** delle voci secondo la regola del ticket 5.

## Le decisioni del coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | nove ticket sul metodo, e i blocchi della tabella qui sopra | sono le domande che si sanno porre oggi (la skill: *«si tickettano solo le domande che sai formulare»*); il 3 viene prima del 4 e del 5 perché il proprietario vuole lo stato dell'arte **prima** delle domande. Costo: un ticket in più o in meno si aggiunge o si chiude |
| 2 | il censimento oltre i 40 file di «non presa»: tutti i file vivi coi quattro segni, il racconto nell'archivio, il codice | una decisione in sospeso si scrive anche *«aspettano il proprietario»* o in una colonna *«Chi la chiude»*, senza la parola «presa». Costo: circa 45 agenti in più sui file piccoli |
| 3 | gli agenti del censimento sono di tipo `Explore` | non possono scrivere, e non ricevono `CLAUDE.md`: meno contesto a testa, come voleva il proprietario. Costo: le regole del progetto che servono stanno nel prompt |
| 4 | il puntatore della §6 cambia in una frase, e il piano dei documenti riceve la voce **ER-8** | il prossimo passo è cambiato; il compito 6 del piano riscrive il puntatore intero con `replace_pointer.py`, e senza la voce cancellerebbe la mappa senza che nulla diventi rosso. Costo: una riga nell'errata |
| 5 | *ultracode* resta spento | la richiesta di parallelizzare vale già per questo workflow; ultracode renderebbe i workflow la regola per ogni lavoro, col costo che smette di essere un limite, e il proprietario il costo lo vuole detto prima (`CLAUDE.md`). Costo: per un workflow futuro serve di nuovo il suo sì |

## Come si riprende

⛔ **Da sapere subito:** mappa e ticket **non esistono ancora** su GitHub. Niente è a metà nel repository.

1. La lettura d'apertura di `CLAUDE.md`; poi questa consegna.
2. **Se il censimento è tornato**, la sua lista sta nella sezione *«Il censimento»*. Se la sessione che l'ha lanciato è
   morta prima, il risultato di ogni agente sta nel `journal.jsonl` della corsa — solo su questa macchina —; da un'altra
   macchina si rilancia lo script coi pezzi, e i pezzi si rifanno col comando qui sotto.
3. Il coordinatore controlla un campione della lista contro il repository, toglie i doppioni rimasti, e la mostra al
   proprietario: **una mappa o due**, e il testo di ciascun ticket.
4. Col sì del proprietario — il repository è pubblico: mappa e ticket si vedono da fuori —, le quattro etichette, la mappa,
   i ticket e i loro blocchi, coi comandi di `references/gh-operations.md` della skill; poi qui i link al posto delle bozze.
5. Il prossimo passo, dopo: il ticket *«Il piano dei documenti si esegue adesso o aspetta il metodo nuovo?»*.

```bash
RE='non pres|registrat|⏳|proprietari'; { find docs -name '*.md' -not -path '*/archivio/*'; ls spikes/*.md spikes/*/*.md; echo docs/archivio/stato-storico.md; } | sort -u | while read f; do n=$(grep -c -E "$RE" "$f"); [ "$n" -gt 0 ] && echo "$n $f"; done | wc -l
```
