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
