# La bozza della mappa del metodo, come mostrata al proprietario — 2026-09-30

⚠️ **Vera il giorno in cui fu scritta.** È il testo della mappa e dei nove ticket sul metodo, parola per parola come mostrato al proprietario il 2026-09-30, prima del censimento delle decisioni in sospeso. Lo stato vivo sta nella [consegna](../superpowers/specs/2026-09-30-metodo-decision-map-design.md) e, quando esisteranno, nei ticket su GitHub.

---

# MAPPA — «Il metodo: si lavora con la decision map» (etichetta decision-map)

## Destinazione

Si lavora con la decision map: `CLAUDE.md` dice come, e ogni decisione in sospeso del proprietario sta in un ticket, non più sparsa nei documenti.

## Note

- **Il progetto**: assistente desktop locale, repository `devfrx/daemon`. Documentazione in italiano, codice in inglese (`CLAUDE.md`).
- **Ogni sessione su questa mappa** fa prima la lettura d'apertura di `CLAUDE.md` — `git fetch`, poi `CLAUDE.md` e `docs/COMPENDIO.md` — e poi carica il corpo di questa mappa, non tutti i ticket. Skill: `anthropic-skills:decision-map` e `anthropic-skills:decision-principles`.
- **Il perimetro**, deciso dal proprietario il 2026-09-30 aprendo la mappa: qui va **solo ciò che è da decidere** — il metodo, e le decisioni in sospeso. ADR, compendio e il piano dei documenti della revisione della knowledge base restano come sono.
- **Una casa sola** (gotcha #68 di `docs/HANDOFF.md`): una voce che diventa ticket si **sposta**, non si copia.
- **Il piano dei documenti** (`docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`) è pre-controllato e tocca `CLAUDE.md` e `docs/COMPENDIO.md`: una modifica del metodo a quei file si prova contro i suoi blocchi col loro `--check`, e il compendio ha un tetto in byte — il comando C del piano.
- **Fuori dal cancello**: `scripts/check-docs.sh` non legge i ticket.
- **Pubblico**: il repository è pubblico, quindi anche mappa e ticket.
- **Il claim**: c'è un solo account GitHub, quindi l'assegnazione non separa due sessioni parallele — si lavora una sessione per volta.
- **Subagenti**: più di uno solo dopo aver detto il costo al proprietario e avuto il sì; modello `opus`, `sonnet` per il lavoro meccanico (`CLAUDE.md`).
- **Alla chiusura della mappa**, una decisione strutturale diventa un ADR, con la sua voce nella §5 del compendio: la pretende `scripts/check-docs.sh`.
- **La consegna di fine sessione**, finché il ticket sulla consegna non decide altro, segue `CLAUDE.md`: la sessione che ha aperto la mappa l'ha scritta in `docs/superpowers/specs/2026-09-30-metodo-decision-map-design.md`.

## Decisioni prese

<!-- una riga per ticket chiuso: il titolo col link, e la risposta in una riga -->

## Non ancora specificato

- **Le decisioni in sospeso, una per una.** Sono citate in 40 file vivi — `grep -rlc 'non pres' docs --include='*.md' | grep -v /archivio/ | wc -l`, il 2026-09-30 — ma quante siano davvero aperte non si sa. Graduano in ticket col censimento.

## Fuori scope

- **Il piano dei documenti della revisione della knowledge base**: è deciso, e si esegue com'è. Quando, lo dice il suo ticket.
- **Riscrivere ADR, compendio o spec attorno ai ticket**: escluso dal perimetro del 2026-09-30.
- **Aprire i fronti nuovi** — il sotto-progetto 13, i modelli decisionali «System One»: questa mappa decide il metodo e trasloca le decisioni in sospeso, non pianifica i fronti.

---

# 1 · decisione · pronto — «Il piano dei documenti si esegue adesso o aspetta il metodo nuovo?»

## Domanda

Il piano dei documenti della revisione della knowledge base è scritto e pre-controllato, e il suo compito 1 era il prossimo passo. Si esegue **adesso**, un compito per sessione, alternando con i ticket di questa mappa — oppure **aspetta** che il metodo nuovo sia deciso?

## Contesto

- Il piano: `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`, sei compiti, nessuno eseguito — la sua tabella *«A che punto è QUESTO PIANO»*.
- Il piano tocca `CLAUDE.md` (la riga «ADR append-only», compito 1) e `docs/COMPENDIO.md` (compiti 1, 2, 3, 6) — la sua *«mappa dei file»*. Le modifiche del metodo nuovo a quei file vanno fatte dopo di lui, o provate contro i suoi blocchi col `--check`.
- Verificato il 2026-09-30: nessuna ancora dei suoi blocchi cade nel puntatore della §6 del compendio, quindi aggiornare il puntatore non li rompe.
- Il proprietario ha aperto questa mappa «prima di iniziare col compito 1».

---

# 2 · decisione · pronto — «Una decisione per sessione, sempre?»

## Domanda

La decision map vuole **una decisione per sessione**, per avere sempre il contesto fresco. Qui spesso una sessione ha portato **più domande A/B piccole** — per esempio la rilettura di un disegno, cinque voci in una sessione. Quando il censimento avrà trovato le decisioni in sospeso, una per sessione è la regola sempre, oppure più A/B piccole possono stare in una sessione — e con quale criterio si dice «piccola»?

## Contesto

- La regola della skill: *«Una sessione, una decisione»* — le ultime decisioni di una sessione lunga sono peggiori della prima.
- La regola del progetto: *«Una fase per sessione»*, e le domande al proprietario *«una per volta, in forma A/B»* (`CLAUDE.md`).

---

# 3 · ricerca · pronto — «Come si tengono insieme, oggi, ticket di decisione, ADR e documenti?»

## Domanda

Alle fonti primarie, con la data: come fanno oggi i progetti a tenere le decisioni **aperte** nei ticket e quelle **prese** negli ADR e nei documenti, senza che la stessa decisione viva in due posti? E quali limiti di GitHub contano per una mappa: quante sub-issue per issue, quanti livelli, blocchi fra issue di mappe diverse, e come si collega un documento a una issue in modo che il collegamento non marcisca.

## Esito atteso

- Fonti primarie con la data: la documentazione di GitHub su sub-issue e dipendenze fra issue; le guide degli ADR, per esempio adr.github.io; se esistono, progetti che usano le issue per le decisioni aperte.
- **Verificato** separato da **dedotto**.
- Nessuna decisione: è un'indagine. Decidono i ticket che questo sblocca.

---

# 4 · decisione · aspetta il 3 — «Come si combinano le cinque fasi con la mappa?»

## Domanda

Oggi ogni lavoro passa da cinque fasi, ciascuna nella sua sessione: brainstorming, disegno, piano, pre-controllo, compiti (`CLAUDE.md`, *«Una fase per sessione»*). Con la decision map le decisioni si prendono nei ticket, una per sessione, e alla fine si **emette** il lavoro. Quali fasi restano come sono, quali diventano ticket della mappa, e dove si scrive il risultato di una decisione — il commento del ticket, un disegno nel repository, un ADR?

## Contesto

- La skill: la mappa **decide e non costruisce**; alla chiusura si emettono issue di lavoro normali, e le decisioni strutturali diventano ADR.
- Il progetto: un disegno si approva **sezione per sezione**, e un piano porta un'errata e un pre-controllo (`CLAUDE.md`, *«Prima di eseguire un compito di un piano»*).

---

# 5 · decisione · aspetta il 3 — «Le voci aperte nei documenti: che cosa diventa ticket, e che cosa resta?»

## Domanda

Oggi le voci aperte vivono in più tabelle: la §6 del compendio, `docs/porta-di-qualita.md`, le voci senza numero AUD di `docs/audit-2026-08-27.md`, i piani, i disegni. Quali diventano ticket — **solo le decisioni del proprietario**, o anche il **lavoro rimandato** che ha già un chiusore, cioè un sotto-progetto futuro? Nel documento che cosa resta: un rimando al ticket, o niente? E un ticket che non riguarda il metodo — una decisione di prodotto — in quale mappa va?

## Contesto

- Le citazioni di «non presa» fuori dall'archivio: `grep -rlc 'non pres' docs --include='*.md' | grep -v /archivio/ | wc -l` — 40 file il 2026-09-30, e molti sono piani già eseguiti.
- La regola del progetto: un puntatore o una cifra che vive in più documenti **si toglie** (`CLAUDE.md`, gotcha #68).
- Alcune tabelle le legge `scripts/check-docs.sh` per posizione (§10 del compendio, trappola 3): spostare una riga può fare rosso.

---

# 6 · decisione · aspetta il 4 — «Dove vive il prossimo passo?»

## Domanda

Oggi il prossimo passo vive **solo** nella §6 del compendio, nel puntatore «⏭️». Con la mappa, la **frontiera** — i ticket aperti, non bloccati e non assegnati — dice quali decisioni sono pronte. La §6 resta la casa unica e rimanda alla mappa, oppure il prossimo passo diventa la frontiera?

## Contesto

- Il puntatore non ha una guardia: è una voce registrata e non presa, nella tabella delle voci aperte della §6 del compendio.
- Nel piano dei documenti il puntatore lo riscrive il compito 6, con `archive_head.py` e `replace_pointer.py`.

---

# 7 · decisione · aspetta il 4 — «Che cosa resta della consegna di fine sessione?»

## Domanda

Oggi a ogni chiusura di sessione si riscrive il *«Come si riprende»* del documento in corso, e quello di prima va in archivio (`CLAUDE.md`, *«Manutenzione della documentazione»*). Con la mappa, la risposta di un ticket è il suo commento di chiusura. Una sessione che lavora un ticket scrive ancora una consegna nel repository — e dove, e con che cosa dentro?

## Contesto

- La skill: la mappa e la consegna di `session-handoff` sono **complementari** — la consegna dice dove si è fermato il lavoro nel repository, la mappa che cosa è deciso.
- Il progetto: la consegna deve arrivare anche sull'altra macchina, quindi sta in un file tracciato, mai in `.handoff/`.

---

# 8 · decisione · aspetta il 5 — «Il cancello deve controllare la mappa?»

## Domanda

I ticket stanno fuori dal cancello: `scripts/check-docs.sh` non li legge, e il progetto dice che *«un principio che non si può controllare è un'intenzione»*. Si accetta così, dichiarato — oppure serve un controllo, per esempio uno script con `gh` in CI che verifica le regole della mappa (ogni ticket chiuso ha la sua riga in *«Decisioni prese»*; nessuna voce vive in due case)? E il cancello locale deve restare senza rete?

## Contesto

- Un passo nuovo di CI è una decisione del proprietario: la §11 del compendio lo dice per la DST profonda.

---

# 9 · lavoro · aspetta il 5 — «Censire le decisioni in sospeso e farne ticket»

## Domanda

Trovare nei documenti vivi **ogni decisione del proprietario ancora in sospeso** — le voci «registrata, non presa», quelle aperte che hanno il proprietario come chiusore, le proposte che lo aspettano —; controllare contro il codice e i documenti di adesso che sia **davvero aperta**; farne un ticket ciascuna, e spostarla secondo la regola del ticket sulle voci aperte.

## Alla chiusura

Il commento dice: quante sono, dove stavano, i comandi usati, e quali voci si sono rivelate già chiuse.
