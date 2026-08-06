# ADR-0026: Linguaggio del core

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Il linguaggio del core non era scelto perché due spike bloccanti potevano escluderne
alcuni: SP-5 (iniettabilità, V29) e SP-6 (confine dei tipi, V19). Protocollo e criteri
in [`spikes/PROTOCOLLO.md`](../../spikes/PROTOCOLLO.md), pre-selezione in
[`spikes/CANDIDATI.md`](../../spikes/CANDIDATI.md), risultati completi con le evidenze
in [`spikes/RISULTATI.md`](../../spikes/RISULTATI.md).

Il protocollo è stato scritto e congelato **prima** di vedere qualsiasi codice
funzionare, e copre tutti e cinque i criteri che la [spec §9.4](../superpowers/specs/2026-08-06-kernel-design.md)
aveva fissato per questa decisione. La verifica di copertura è nel protocollo stesso.

I tre candidati competevano per **un solo posto**. Non è una scelta di usare tre
linguaggi: è il costo di non scoprire dopo che il linguaggio scelto non regge una
proprietà non retrofittabile.

### Esiti — SP-6, confine dei dati non fidati e confini statici del kernel

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| T1 non compila | `passa` | `passa` | `passa` |
| T2 percorso unico | `passa` | `passa` | `passa` |
| T3 ereditarietà | `passa` | `passa` | `passa` |
| T4 aggiramento | `passa` | `passa`, ma solo dopo una correzione | **`parziale`** — tre vie, nessuna vietabile dal compilatore |
| T5 rilevabile globalmente | `passa` | `passa` | `passa` |
| T6 importazione vietata, provata in negativo | `passa` | `passa`, con driver scritto a mano | **`parziale`** — regola del compilatore, ma zittibile per riga |

### Esiti — SP-5, iniettabilità e riproducibilità

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| C1 stesso seed → stessa traccia | `passa` | `passa` | `passa` |
| C2 seed diversi → tracce diverse | `passa` | `passa` | `passa` |
| C3 tempo virtuale | `passa` | `passa` | `passa` |
| C4 guasto riproducibile | `passa` | `passa` | `passa`, con seed 4 e non 99 |
| C5 nessun orologio/RNG globale | `passa` | `passa` | `passa` |
| C6 concorrenza nativa ordinabile | `passa` | **`non passa`** | **`parziale`** |
| C7 I/O iniettabile, crash riproducibile | `passa` | `passa` | `passa` |

| Candidato | SP-6 | SP-5 | Passa entrambi? |
|---|---|---|---|
| **Rust** | 6/6 | 7/7 | **sì** |
| Go | 6/6 | 6/7 | no |
| TypeScript | 4/6 | 6/7 | no |

### Versioni degli strumenti e seed usati

| Candidato | Versioni |
|---|---|
| Rust | `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` · `clippy 0.1.95` · `trybuild 1.0.120` |
| Go | `go version go1.26.5 windows/amd64` |
| TypeScript | `tsc 5.9.3` · node `v24.9.0` · npm `11.6.0` · `@types/node` 26.1.2 |

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| C1, C2 | 42, 43 | 42, 43 | 42, 43 |
| C3 | 7 | 7 | 7 |
| C4 | 99 | 99 | **4** — l'RNG qui è a 32 bit, la sequenza differisce |
| C6 | 20260806 | **il seed non entra**: non esiste un punto in cui inserirlo nello scheduler delle goroutine | 20260806, via generatori |
| C7 | 1, 7, 42, 99, 20260806 | idem | idem |
| C7 passo in dubbio | 0 | 0 | 0 |

### Le tre correzioni che gli spike hanno imposto al piano

Registrate perché sono l'esito più utile dell'esercizio: in tutti e tre i casi
un'affermazione scritta prima della misura si è rivelata falsa.

| # | Affermazione | Misura |
|---|---|---|
| 1 | in Go l'aggiramento del confine è possibile «solo dentro il package, perché il campo non è esportato» | **falso.** Con lo stesso nome di campo, i due tipi hanno lo stesso tipo sottostante e `boundary.Instruction(dalWeb)` compila **da qualsiasi punto del progetto**, senza `unsafe` e senza reflection. Eseguito: il contenuto non fidato viene stampato nel canale delle istruzioni. Corretto rinominando il campo, e blindato con un test di compilazione fallita |
| 2 | in TypeScript, togliere il marchio da `Untrusted` fa fallire il controllo di tipo | **falso.** Il typecheck passa comunque, perché `Instruction` resta marchiato. La sonda corretta è togliere il marchio da `Instruction`. La prova di non-vacuità così com'era **non provava nulla** |
| 3 | in TypeScript esiste una via di aggiramento, `as any` | **tre.** `as any`, `as unknown as Instruction`, `<Instruction><unknown>`. Vietare `any` non basta: la doppia asserzione via `unknown` sopravvive |

### La misura che ha chiuso la domanda aperta su Go

L'ipotesi da falsificare era che lo spareggio sul controllo «posseduto vs fornito»
fosse troppo severo verso Go: se il kernel guida le proprie attività con un esecutore
proprio, quanto conta che lo scheduler delle goroutine sia del runtime?

100 esecuzioni della stessa scena, 3 goroutine in contesa, 6 passi ciascuna:

| Prova | Dentro `synctest` | Fuori dalla bolla |
|---|---|---|
| contesa su **canale** della bolla — il caso più favorevole, *durably blocking* | **9** tracce distinte | 13 |
| contesa su **`sync.Mutex`** — escluso testualmente dal *durably blocking* | **4** tracce distinte | 5 |

`synctest` **riduce** il non determinismo, non lo elimina. È coerente con la propria
documentazione, che promette *quiescenza* — il tempo avanza quando ogni goroutine della
bolla è durably blocked — e **non un ordine totale**. La formulazione diffusa
«`synctest` dà scheduling deterministico» è più forte del contratto reale.

La riga sul mutex è la più pesante per questo progetto:
[ADR-0004](0004-topologia-di-processo.md) descrive l'arbitro GPU come «un unico processo
con **un unico lock**», cioè esattamente la primitiva che `synctest` dichiara di non
coprire. Verificato che `synctest` virtualizza davvero il **tempo**: quello funziona, e
non è in discussione.

## Decision

Il core si scrive in **Rust**.

Il criterio che ha deciso il confronto è lo **spareggio #1 — il controllo deterministico
è posseduto o soltanto fornito dai test** — perché è l'unico su cui i tre candidati
divergono in modo non recuperabile, e perché discende da V29 e
[ADR-0021](0021-simulazione-deterministica-e-iniettabilita.md), che dichiarano la
simulazione deterministica **non retrofittabile**.

In Rust l'ordine delle unità concorrenti native è deciso dall'esecutore del kernel, non
dal runtime: `Future` è un oggetto che si sceglie quando far avanzare, e il controllo
vale anche **fuori dai test**. In Go lo scheduler delle goroutine appartiene al runtime
e non è sostituibile: il determinismo è *fornito* da `testing/synctest`, solo dentro i
test, e — misurato — solo parzialmente. In TypeScript il controllo esiste **a
condizione di rinunciare a `async`/`await`** e scrivere il kernel in generatori, e
comunque senza parallelismo reale, che [ADR-0004](0004-topologia-di-processo.md)
richiede.

**Va detto esplicitamente che l'esito non era scontato.** I criteri erano fissati dalla
§9.4 prima che i candidati esistessero, e due delle tre misure decisive potevano andare
diversamente: la verifica su Go è stata eseguita per **falsificare** l'attesa, non per
confermarla, e T6 su TypeScript si è rivelato **più forte** del previsto — un
meccanismo del compilatore e non un lint. Nessuna delle due ha cambiato l'esito, ma
entrambe potevano.

Se **nessuno** dei tre fosse passato, la regola era riaprire la pre-selezione e
valutare C#/.NET. Non è stato necessario.

## Consequences

- **Positive:**
  - I requisiti Q2 (zero OOM), Q4 (kill di un worker in qualsiasi istante) e Q5
    (riavvio a metà run) restano **verificabili** per simulazione deterministica, e
    V31 — ogni difetto conserva il proprio seed — resta applicabile.
  - Il confine dei dati non fidati è imposto dal compilatore, e l'unica via di
    aggiramento residua è vietabile in modo **non scavalcabile per riga**:
    `#![forbid(unsafe_code)]` rifiuta anche un `#[allow]` locale (`E0453`, misurato).
  - I3 e V28 sono verificabili con un meccanismo del compilatore e non un lint: una
    crate `#![no_std]` rifiuta `std::fs` con `E0433`, misurato in entrambe le direzioni.
  - Esiste un runtime deterministico di ecosistema — `madsim` 0.2.34 con
    `madsim-tokio` 0.2.30 come sostituto di tokio, `turmoil` 0.7.2 — quindi il
    simulatore non va scritto da zero.
  - Il packaging resta un binario singolo per il core, senza runtime esterno da
    installare accanto. Rileva rispetto a [ADR-0028](0028-ecosistema-dei-worker-ml.md),
    che ne introduce già uno per i worker.

- **Negative (accettate):**
  - **È il linguaggio su cui il proprietario del progetto non è operativo.** Costo reale
    su tempo di apprendimento e velocità delle prime settimane. È il costo più grande di
    questa decisione, e non è mitigabile con la tecnica.
  - **Il confine forte non è quello di default: va scelto.** Il lint da solo non basta —
    misurato su un caso reale: la regola ha bloccato un uso *legittimo* di
    `Instant::now()` dentro il test che deve misurare il tempo di parete, e ha richiesto
    un `#[allow]` per sito. Chi non sceglie `forbid` e `no_std` ottiene un confine
    disattivabile.
  - **I confini sono a granularità di crate, non di modulo.** Vincola il layout dei
    sorgenti dal primo giorno: il kernel dovrà essere una crate propria e il modulo di
    piattaforma una crate separata. È coerente con I3, ma è un vincolo, non un dettaglio.
  - **`std::collections::HashMap` va vietato nel kernel**: `RandomState` è seminato
    casualmente per processo, quindi l'ordine di iterazione non è riproducibile fra
    esecuzioni. È una violazione di V29 che non compare in nessun elenco di «chiamate
    OS» e che si manifesterebbe come traccia divergente e inspiegabile.
  - Rinunciamo ai vantaggi reali degli esclusi: la curva di apprendimento di Go e il suo
    packaging altrettanto semplice, e la competenza già acquisita su TypeScript.

- **Follow-up richiesti:**
  - La scelta del **motore di persistenza** (§10.6) si valuta ora sull'ecosistema Rust.
    Candidati verificati esistenti: `redb` 4.1.0, `fjall` 3.1.8, `rusqlite` 0.40.1,
    `sled` 1.0.0-alpha.124. Il discriminante non è la disponibilità ma il **requisito 4**
    — I/O iniettabile — che va confermato prima di scegliere.
  - Il prototipo `spikes/rust/` diventa il punto di partenza del simulatore del
    sotto-progetto 1. I prototipi degli esclusi sono rimossi: l'evidenza resta qui e in
    `RISULTATI.md`.
  - Le quattro conseguenze negative tecniche sopra vanno tradotte in **guide del
    progetto** ([ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md)) e
    in controlli automatici, non lasciate alla memoria.
