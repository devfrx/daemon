# Requisiti della GUI, e criteri di passaggio del prototipo IPC

La GUI non ha incognite bloccanti: [ADR-0004](../docs/adr/0004-topologia-di-processo.md)
la rende **sacrificabile**, quindi la scelta è reversibile. Ma **non è indipendente dal
core**: ergonomia dell'IPC, tipi condivisi e packaging cambiano a seconda della coppia.
Si valuta la coppia, non i due elementi separati.

Ogni riga cita la sua fonte. **Se un requisito non ha fonte nella spec, non è un
requisito.**

## Vincoli strutturali

| # | Vincolo | Fonte |
|---|---|---|
| G1 | 0..1 istanze, effimera, **solo stato di presentazione** | ADR-0004, I1 |
| G2 | Parla **solo** col core, via IPC privato non versionato | ADR-0004, I4 |
| G3 | Può essere uccisa in qualsiasi istante senza perdite | I1, Q3 |

## Deve rendere

| # | Cosa | Fonte |
|---|---|---|
| G4 | chat markdown con blocchi di codice, streaming token per token | mappa funzionale, area 2 |
| G5 | diff dei file, approvabili o rifiutabili | capacità Coding |
| G6 | viewer 3D interattivo (rotazione, zoom, materiali) | mappa funzionale, area 7 |
| G7 | artifacts o canvas con anteprima viva | mappa funzionale, area 2 |
| G8 | grafici di costo e di occupazione | sezioni 3 e 7 |

## Deve mostrare sempre

| # | Cosa | Fonte |
|---|---|---|
| G9 | stato di degrado corrente | V27, ADR-0019 |
| G10 | permessi attivi nella sessione | V21, sezione 6 |
| G11 | occupazione del contesto **per categoria** | ADR-0010, sezione 7 |
| G12 | costo corrente e distanza dal tetto | V8, sezione 3 |
| G13 | provenienza del contenuto: fidato o non fidato | V23, ADR-0014 |
| G14 | run in `AttesaUmano`, con notifica | V9 |
| G15 | `Rifiutata` e `InCoda` come esiti **distinti** | V4, sezione 2 |
| G16 | che la cifratura vale quanto l'account OS | ADR-0023 |
| G17 | cosa il backup **non** contiene, al momento del backup | ADR-0022 |
| G18 | cosa copre l'ambito di checkpoint, **prima** che l'agente scriva | ADR-0024 |

## Vincoli di piattaforma

| # | Vincolo | Fonte |
|---|---|---|
| G19 | Windows primario, Linux successivo senza riscrivere | ADR-0002 |
| G20 | navigazione da tastiera, screen reader, contrasto | mappa funzionale, area 8 |
| G21 | interfaccia multilingua | mappa funzionale, area 8 |

## Matrice delle coppie

Il core è **Rust** ([ADR-0026](../docs/adr/0026-linguaggio-del-core.md)). Le righe
relative a Go e TypeScript sono state cancellate: righe non pertinenti lasciate lì
sono rumore che il prossimo lettore dovrà filtrare a mano.

| Core | Opzione GUI | Packaging | Tipi condivisi | Costo del viewer 3D (G6) | Esito |
|---|---|---|---|---|---|
| Rust | **interfaccia web** in un guscio nativo | dipende dal guscio | no, schema duplicato | libreria web matura | ✅ **scelta** — ADR-0027, per G7 |
| Rust | toolkit nativo del linguaggio | binario singolo | sì | da scrivere | ❌ scartata: non regge G7 |

**Il guscio che ospita la webview — Tauri o Electron — è una decisione separata e
ancora aperta**: [ADR-0029](../docs/adr/0029-guscio-della-gui.md), `Proposed`. Il
framework è **Vue 3**, [ADR-0030](../docs/adr/0030-framework-dell-interfaccia.md).

## Criteri di passaggio del prototipo IPC

**Fissati prima di guardare i numeri.** I requisiti da G4 a G18 sono lavoro noto:
l'unica incognita reale è se l'IPC regga **aggiornamenti ad alta frequenza** senza
ingolfare l'interfaccia — token in streaming, stato di degrado e occupazione del
contesto, tutti insieme.

| # | Criterio | Soglia |
|---|---|---|
| P1 | messaggi persi | **zero** |
| P2 | ritardo massimo fra emissione e rendering | < 100 ms |
| P3 | CPU della gui durante lo streaming | < 25% di un core |
| P4 | uccidere la gui a metà streaming e riaprirla | il core non se ne accorge (G3) |

**Se P1 fallisce, l'IPC ha bisogno di contropressione**: è una decisione di design del
**kernel**, non un dettaglio della GUI, e va scritta come ADR a sé.

### Scenario

| | |
|---|---|
| Carico | **2000 messaggi in 10 secondi**, su tre canali logici distinti |
| Canali | `token` (streaming), `stato` (degrado), `metriche` (contesto e costo) |
| Misure | messaggi persi · ritardo massimo emissione → rendering · CPU del processo gui |

## Risultati misurati

Eseguito il **2026-08-06** su rustc 1.95.0, Windows 11. Prototipo in
[`gui-ipc/`](gui-ipc/), trasporto `interprocess` 2.4.3 — named pipe su Windows,
socket unix su Linux, stesso codice (G19).

### Corsa 1 — carico nominale

| # | Soglia | Misurato | Esito |
|---|---|---|---|
| P1 | zero persi | **0 persi, 0 buchi** su 2000 attesi / 2000 ricevuti | ✅ |
| P2 | < 100 ms | **10,21 ms** di picco · 0,363 ms medio | ✅ con margine ~10× |
| P3 | < 25% di un core | **21,43 %** di picco · 8,05 % medio, su 46 campioni | ✅ **ma stretto** |
| P4 | — | corsa 2 | — |

Ripartizione osservata: `token` 1600 · `stato` 200 · `metriche` 200, in 10 001 ms.

### Corsa 2 — P4, la gui muore a metà e si riapre

La gui termina con `abort()`, non con un'uscita pulita: è il caso che I1 e G3
promettono di reggere.

| Evento | Osservato |
|---|---|
| gui abortita | dopo 600 messaggi ricevuti |
| il core se ne accorge | al messaggio **606**, e prosegue |
| gui riaperta | al messaggio **606** |
| esito del core | **2000 messaggi emessi in 10 000 ms** — nessun rallentamento |
| gui #2 | 1393 messaggi ricevuti, **0 buchi nel progressivo** |

**P4 ✅.** I «607 persi» della gui #2 sono contati rispetto al totale di 2000 e non
sono perdita di messaggi: sono i messaggi emessi mentre la gui era morta. La
distinzione conta, ed è il motivo per cui si misurano **i buchi nel progressivo** e non
solo il conteggio: i buchi sono zero.

### Cosa questo prototipo NON ha misurato

| Non misurato | Perché conta |
|---|---|
| il salto **webview ↔ shell** dentro il processo gui | il prototipo misura `core → processo gui`. Una GUI a interfaccia web ha un **salto in più**, non incluso nei 10,21 ms |
| il costo di rendering reale (DOM, layout, 3D) | qui il «rendering» è una lista in memoria: il vero costo di P3 sarà più alto |
| il comportamento su Linux | `interprocess` promette lo stesso codice; **non verificato** |

Il margine su P2 (10 ms contro 100) lascia spazio al salto mancante. Il margine su P3
(21,43 % contro 25 %) **non ne lascia**: va rimisurato con un rendering vero, ed è la
prima cosa da controllare nel sotto-progetto 2.
