# SP-7 — Riconoscimento gesti: il protocollo

Criteri scritti **prima** della misura, come vuole la §4.2 del
[disegno](../../docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md), e
**congelati al primo commit di codice di questo spike**. Un criterio soddisfatto con un
accorgimento si registra come **parziale**, non come passato — la regola di
[`PROTOCOLLO.md`](../PROTOCOLLO.md), che vale anche qui.

**Che cosa misura, e che cosa no.** Le due ipotesi **assunte** dal disegno (§6.4): S1 e S2. La
terza, S3, non è uno spike: è una sonda nel kernel, `crates/kernel/tests/arbiter_admission.rs`.
Lo spike **non** misura il riconoscimento di un gesto discreto (F3): il vocabolario è della
capacità. E non misura niente sulla GPU: su Windows, in Python, non c'è (F2, F9).

**La macchina.** Il criterio vale **su questa macchina**: la CPU la dice
`powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name"` e va nell'esito con le
versioni degli strumenti. La telecamera è quella integrata o collegata il giorno della misura, e
si dichiara.

## S1 — MediaPipe Hand Landmarker su CPU regge 30 Hz?

| | |
|---|---|
| Domanda | il tempo per fotogramma del tracciatore su CPU, a **due mani**, **640×480**, modo **LIVE_STREAM** |
| Criterio | **mediana < 33 ms e p95 < 33 ms** su almeno **600** risultati consecutivi con **entrambe le mani in campo**; il margine si **riporta**, non si promette |
| Che cosa si riporta | mediana, p95, massimo, numero di risultati, risultati al secondo, fotogrammi inviati e **scartati** (inviati senza risultato), quota di risultati a due mani |
| Come | `s1_bench.py`: la latenza è il tempo fra `detect_async` e la callback, misurato con `time.perf_counter_ns` nello stesso processo |
| Parziale | se il criterio regge solo a una mano, o solo sotto 640×480, o solo con `num_hands=1` |

## S2 — Quanto costa il giro worker → core → GUI a 30 Hz?

| | |
|---|---|
| Domanda | la latenza da **cattura** a **disegno** della mano su una pagina, passando per un relay Rust che sta al posto del core |
| Criterio | il solo salto **relay → pagina** ha **p95 < 100 ms**, che è **P2** di [`GUI-REQUISITI.md`](../GUI-REQUISITI.md); la latenza **totale** da cattura a disegno si **riporta**, mediana e p95, **senza soglia**: l'accettabilità della mano sul pannello la giudica il **proprietario provandola** — il pannello che segue il pinch — e il giudizio va nell'esito con le sue parole |
| Che cosa si riporta | mediana, p95 **e massimo** di cattura → disegno e di relay → disegno su almeno **600** campioni; il giudizio del proprietario |
| Come | `s2_worker.py` emette una riga JSON per risultato; `relay/` lo spawna come processo figlio, legge lo stdout, timbra, e serve la riga in Server-Sent Events a `page.html`, che disegna i 21 punti e misura col proprio orologio. Tre orologi di parete della stessa macchina, in millisecondi |
| Parziale | se regge solo sotto i 30 Hz, o solo a una mano |
| Che cosa non prova | niente sul canale `process` vero (`minicbor`, la busta): il relay usa righe JSON su una pipe, perché qui si misura il **giro**, non il formato |

⛔ **RICHIAMO DEL 2026-09-04 — è una modifica al METRO, e la clausola *Congelamento* di
questo file pretende che sia detta.** La riga *«Che cosa si riporta»* di S2 diceva *«mediana
e p95»*, e ora dice *«mediana, p95 e massimo»*. ⚠️ **Il criterio NON cambia:** resta
`p95 < 100 ms` sul salto relay → pagina, che è ciò che la §4.2 del disegno approva —
*«mediana e p95»*, *«nessuna soglia inventata»*. ⚠️ **Ciò che va detto è che P2 è un
MASSIMO:** la riga 77 di [`GUI-REQUISITI.md`](../GUI-REQUISITI.md) lo enuncia come *«ritardo
massimo fra emissione e rendering»*, e la riga 103 lo misurò come picco. Provarlo su un p95
è una lettura **più debole** del metro citato, e su seicento campioni lascia passare fino
a trenta campioni sopra i 100 ms senza che il criterio cada. Riportare **anche** il massimo
rende l'esito di SP-7 **confrontabile** con quella riga invece che soltanto accostabile — ed
è la forma che **S1 di questo stesso file già usa**. Trovato dalla revisione del compito 7
il 2026-09-04, deciso dal **proprietario** lo stesso giorno; voce **E11** dell'errata del piano.

## Registrazione

L'esito va in [`RISULTATI.md`](../RISULTATI.md), sezione **SP-7**, nella forma di SP-5 e SP-6:
esito per criterio, osservazioni che non sono criteri, versioni degli strumenti, evidenze con
comando e output osservato, e le **divergenze** dall'attesa. I dati grezzi (CSV) restano fuori dal
repository.

## Congelamento

Congelato al primo commit di codice di questo spike. Una modifica dopo quel commit è una
modifica al **metro**, e va detta.
