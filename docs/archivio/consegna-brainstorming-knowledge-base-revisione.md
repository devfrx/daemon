# La revisione della knowledge base — le chiusure di sessione archiviate

⚠️ **Verbali, veri il giorno in cui furono scritti.** Il documento vivo è la
[consegna della revisione](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md), che tiene solo
l'ultima chiusura di sessione — `CLAUDE.md`, *«Un verbale di correzione non resta nel documento corretto»*. Le chiusure
precedenti stanno qui, parola per parola, coi link riscritti per questa cartella.

## Archiviata il 2026-09-28, alla chiusura della sessione che ha aperto la revisione

La sezione scritta all'apertura del brainstorming, com'era nel commit `9bdbb59`.

## Come si riprende — scritto all'apertura del brainstorming, il 2026-09-28

⛔ **Niente è a metà.** Il commit di questo file porta anche il puntatore della §6 del compendio; albero pulito, tutto pushato.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| codice di prodotto | **non toccato**: `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`, all'apertura sull'albero di `f830cb9` e di nuovo prima del commit di questo file: si rilanciano, non si citano |
| il puntatore | la §6 del compendio: la revisione **prima** del 13 e dei modelli decisionali |
| la guida ARMS | **non** è nel repository: il riassunto sta nella sezione *«La fonte del documento»*, la provenienza in [`riferimenti.md`](../riferimenti.md) |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, e il
   [disegno del 2026-09-04](../superpowers/specs/2026-09-04-knowledge-base-design.md) per intero — la §12 del compendio lo chiede a chi riprende il
   fronte della knowledge base.
3. La prima riga della tabella *«Le risposte del proprietario»* ancora ⏳ è la domanda da porre: si ripone **com'è scritta
   qui**, dopo aver rilanciato i comandi della tabella *«Che cosa esiste oggi»* — il codice può essersi mosso.
4. A ogni risposta: la riga nella tabella, un commit, un push. La domanda successiva si scrive **qui**, nella forma di D1, prima
   di porla.
5. Finite le domande: la chiusura del brainstorming — le decisioni prese, le registrate col chiusore — e il disegno in una
   sessione **nuova**, che scrive i richiami datati al disegno del 2026-09-04 e i rimandi agli ADR.

## Archiviata il 2026-09-29, alla chiusura della seconda sessione della revisione

La sezione scritta alla chiusura della sessione del 2026-09-28, com'era nel commit `5ba2ed5`.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-28

⛔ **Da sapere subito: niente è a metà, ma D3 è senza risposta.** Albero pulito dopo il commit di questa chiusura, tutto
pushato, nessuno stash, nessun codice toccato. Il proprietario ha chiuso la sessione — *«continuiamo l'analisi e le domande
nella prossima sessione»* — sulla forma **riformulata** di D3, prima di leggerla: si ripone quella. La chiusura precedente sta
in [`archivio/consegna-brainstorming-knowledge-base-revisione.md`](consegna-brainstorming-knowledge-base-revisione.md).

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline f830cb9..HEAD`: l'avvio col puntatore della §6; il documento del proprietario col confronto; D2 con D3 posta; questa chiusura, con D3 riformulata |
| codice di prodotto | **non toccato**: `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`, all'apertura sull'albero di `f830cb9` e prima di ogni commit: si rilanciano, non si citano |
| il puntatore | la §6 del compendio: la revisione **prima** del 13 e dei modelli decisionali — non cambia con questa chiusura |
| fine-riga | questo file e l'archivio della consegna **LF**; compendio, archivio dello stato e `riferimenti.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script di questa sessione stanno nello scratchpad |
| la guida ARMS | **non** è nel repository: il riassunto sta nella sezione *«La fonte del documento»*, la provenienza in [`riferimenti.md`](../riferimenti.md) |

**Dove si è arrivati.** Lo stato vive nelle tabelle di questo file; qui c'è solo dove guardare.

| | |
|---|---|
| la forma della knowledge base | il **documento del proprietario**: una root qualsiasi, due attori, il livello strutturale e quello semantico, il riconciliatore meccanico, le esclusioni, il confinamento, il controllo prima di scrivere |
| che cosa cambia del 2026-09-04 | la tabella *«Il documento contro ciò che esiste»*: le risposte 1, 3, 4 e 10 |
| le risposte | D1 respinta, D2 **A** — la tabella *«Le risposte del proprietario»* |
| la domanda aperta | **D3 riformulata**: il modello a due zone, e *«una repo aperta come zona di lavoro entra nel grafo e nella ricerca?»*, col consiglio **A** |
| le domande dopo | D4–D10, nella tabella *«Le domande, una per volta»* |
| i buchi | K1–K34: la tabella *«Lo stato dei buchi dopo il documento»* e quella dei casi nuovi |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, e il
   [disegno del 2026-09-04](../superpowers/specs/2026-09-04-knowledge-base-design.md) per intero — la §12 del compendio lo chiede a chi riprende il
   fronte della knowledge base.
3. Rilanciare i comandi della tabella *«Che cosa esiste oggi»*, e quello di D3 sulla porta `filesystem`: il codice può essersi
   mosso.
4. **Porre D3 com'è scritta qui**, nella forma riformulata. Poi D4–D10, una alla volta: ciascuna si scrive **qui**, nella forma
   di D2 e D3, **prima** di porla — contesto, A/B col costo e ciò che si rifà, i cinque criteri, verificato, dedotto e assunto,
   il consiglio —; in chat a parole semplici, poi `AskUserQuestion` con due opzioni e il consiglio per primo.
5. A ogni risposta: la riga nella tabella delle risposte, lo stato dei K che tocca, il cancello, un commit, un push.
6. Finite le domande: la chiusura del brainstorming e, in una sessione **nuova**, il disegno. Scrive i richiami datati al
   disegno del 2026-09-04 — le risposte 1, 3, 4 e 10, le decisioni 13 e 15 —; i rimandi agli ADR che la revisione tocca,
   ciascuno riletto contro i fratelli, gotcha #59; le righe di `roadmap.md` e di `tracciabilita.md`, fra cui
   `Multi-repo/multi-progetto` e `Mappa del progetto`; e la voce della §5 del compendio per ogni ADR che riceve un rimando.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema. Costo: un `--amend` |
| 2 | le decisioni aperte 2 e 3 del proprietario **accolte** come le propone, senza domanda | coerenti con la risposta 10 del 2026-09-04 e con ADR-0024, e senza un'alternativa da porgli. Costo: una domanda, se le vuole riaprire |
| 3 | le prime forme di D1 e di D3 **non** vanno in archivio | sono domande, non verbali di correzione: stanno nei commit `e714720` e `9bdbb59`, nominati dove servono. Costo: zero |
| 4 | la guida ARMS in `riferimenti.md` come **origine dell'idea**, non come prova | è una guida di pratica, non una norma. Costo: zero |
| 5 | i casi nuovi **continuano** la numerazione, da K24 | un caso tiene il suo numero per tutta la revisione. Costo: zero |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **D1 com'era posta**: tre posti, con la knowledge base scritta dal solo assistente | il proprietario l'ha respinta e ha risposto col suo documento: la forma giusta non era fra le due opzioni. 📌 *Prima di porre una domanda sulla forma, chiedersi se la domanda assume una decisione vecchia — qui la risposta 3 del 2026-09-04 — che il proprietario può voler rovesciare* |
| **D3 nella prima forma**: la root come confine di tutto l'assistente | non reggeva col coding stile Claude Desktop, che apre una cartella qualsiasi — una capacità della roadmap, il 5, che il consiglio ignorava. 📌 *La prova «che cosa arriva» di `CLAUDE.md` si fa anche sul consiglio, non solo sullo schema* |

**Da verificare alla fonte prima del disegno** — le righe **F** delle tabelle: K6 e K34, i file «solo online» di OneDrive; K9,
gli eventi che il sorvegliante di Windows può perdere; e, per K1, le cartelle dati per utente dei due sistemi — quella di
Windows, e la specifica XDG per Linux.
