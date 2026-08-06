# Candidati e pre-selezione

Il proprietario del progetto è operativo su Python, TypeScript/web e un linguaggio
compilato. La pre-selezione applica i vincoli globali della spec, non le preferenze.

**I tre candidati competono per un solo posto: il linguaggio del `core`.** Due dei tre
prototipi vengono cancellati alla chiusura di ADR-0026. Non è una scelta di usare tre
linguaggi — è il costo di non scoprire dopo che il linguaggio scelto non regge una
proprietà non retrofittabile (V29, V19).

## Ammessi allo spike

| Candidato | Perché è ammesso | Verificato |
|---|---|---|
| **Rust** | tipi nominali nativi (`newtype`); esistono runtime deterministici di ecosistema — `madsim` 0.2.34 con `madsim-tokio` 0.2.30 come sostituto di tokio, `turmoil` 0.7.2; adatto a un daemon a vita lunga con concorrenza reale | `cargo search`, 2026-08-06 |
| **Go** | tipi denominati nominali; `testing/synctest` fornisce **tempo virtuale e quiescenza deterministica** nei test — vedi la nota sotto, perché non è ciò che si legge in giro; daemon nativo | `go doc testing/synctest`, go1.26.5, 2026-08-06 |
| **TypeScript** | competenza dell'utente; i branded types emulano la nominalità; ciclo di eventi controllabile | — |

### Nota su `testing/synctest` — cosa promette davvero

Va scritto qui perché la formulazione corrente e diffusa — «scheduling deterministico»
— è **più forte di quanto la documentazione dichiari**, e su questa differenza poggia
lo spareggio #1 dell'ADR.

| `synctest` fa | `synctest` **non** dichiara di fare |
|---|---|
| orologio finto per bolla; il tempo avanza solo quando **ogni** goroutine della bolla è *durably blocked* | imporre un **ordine totale deterministico** all'interlacciamento delle goroutine |
| tratta come durably blocking: send/receive su canale della bolla, `select` sui soli canali della bolla, `sync.Cond.Wait`, `WaitGroup.Wait`, `time.Sleep` | controllare `sync.Mutex` e `sync.RWMutex`, l'I/O di rete e le chiamate di sistema — **esclusi testualmente** |

La seconda riga della colonna destra ha un peso specifico su questo progetto:
[ADR-0004](../docs/adr/0004-topologia-di-processo.md) descrive l'arbitro GPU come «un
unico processo con **un unico lock**: la forma più semplice possibile del pezzo che
deve essere infallibile». Il criterio **C6** del protocollo misura esattamente questo,
su entrambe le primitive, invece di assumerlo.

## Esclusi, con motivo

| Candidato | Motivo dell'esclusione |
|---|---|
| **Python** | V28 e V19 richiedono verifica **statica**: l'annotazione di tipo è opzionale e non impedisce l'assegnazione a runtime. Inoltre il GIL rende difficile la concorrenza reale richiesta da ADR-0004 per un daemon a vita lunga. **Resta il linguaggio dei worker ML**, dove è insostituibile: l'esclusione riguarda il core, non il progetto — vedi ADR-0028 |
| **C# / .NET** | non fra gli ecosistemi su cui l'utente è operativo. Sarebbe un candidato tecnicamente valido: se i tre ammessi fallissero, va riconsiderato |
| **C++** | nessun vantaggio sui tre ammessi per questo carico, e costo di sicurezza della memoria non giustificato |

## Cosa la pre-selezione non decide

Il linguaggio dei **worker ML** non è in gara: è Python per assenza di alternative
reali (TRELLIS2, embedding, STT e TTS hanno implementazioni Python). ADR-0028 lo
**ratifica** e ne dichiara i costi; non lo sceglie.

Il linguaggio della **GUI** non è in gara qui: dipende dal core e si valuta in coppia
con esso in ADR-0027, perché ergonomia dell'IPC, tipi condivisi e packaging cambiano a
seconda della coppia.

## Regola

Se **nessuno** dei tre passa entrambi gli spike, non si sceglie il meno peggio: si
riapre la pre-selezione e si valuta C#/.NET, registrandolo qui.
