# Risultati degli spike

Data di esecuzione: **2026-08-06** per SP-5 e SP-6; **SP-7** porta la propria data nella sua sezione

Criteri e soglie: [PROTOCOLLO.md](PROTOCOLLO.md) — congelato al primo commit di
codice di spike.

> ⚠️ **Aggiornato il 2026-08-08 — gli spike restano FUORI dal workspace.** Il piano del
> Traguardo 1 crea il workspace alla radice con `crates/{kernel,platform,secrets,simulator,daemon}`
> e mette `spikes` fra gli `exclude`. Due ragioni, entrambe misurate: `spikes/rust/` è a sua
> volta un **workspace annidato**, e porta un `clippy.toml` che a livello di workspace
> scatterebbe addosso a `platform` — che *deve* chiamare l'orologio e il filesystem
> (vincolo 5 della §11, §7.4.4).
>
> ⛔ **«Punto di partenza» significa che si copia, non che sale.** La
> [§2.5 della spec](../docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) dice
> riga per riga cosa entra in `crates/kernel/` e cosa **resta qui**: `esegui_thread` resta,
> perché non è codice ma l'evidenza che C6 non è vacuo; e l'aiutante `passo_in_dubbio`
> **non sale così com'è**, perché assume esecuzione sequenziale e con l'interlacciamento dà
> un falso negativo (gotcha #20).

**Dove sono i prototipi.** Quello del candidato vincente resta in `spikes/rust/` e
diventa il punto di partenza del simulatore del sotto-progetto 1. Quelli di Go e
TypeScript sono stati rimossi dopo ADR-0026, ma restano nella storia: l'ultimo commit
che li contiene è **`da653a1`**. Un ADR che cita misure deve lasciarle rifacibili —
`git show da653a1:spikes/go/sched/c6_test.go` e simili.

## SP-7 — Riconoscimento gesti: MediaPipe su CPU, e il giro worker → core → GUI — eseguito il 2026-09-04

Criteri e soglie: [`gesti/PROTOCOLLO.md`](gesti/PROTOCOLLO.md), congelato il 2026-09-04
al primo commit di codice dello spike, **prima** della misura — e col richiamo datato del 2026-09-04 che vi dichiara l'unica modifica al metro, il **massimo** aggiunto a «Che cosa si riporta» di S2 (voce E11 dell'errata del piano). Codice in `gesti/`; dati grezzi
fuori dal repository. La terza ipotesi del disegno, S3, non è qui: è una sonda nel kernel,
registrata in `docs/porta-di-qualita.md`.

| Criterio | Esito | Misura |
|---|---|---|
| S1 — su CPU, due mani, 640×480, LIVE_STREAM: mediana e p95 < 33 ms su ≥ 600 risultati a due mani | ❌ **`non passa`** | mediana **31,67 ms**, p95 **35,54 ms**, massimo 54,98 ms; 895 risultati, 28,5 al secondo, 897 inviati, 2 scartati; 895 risultati a due mani su 895. ⛔ La mediana regge, il **p95 no**. Ripetuto sulla stessa configurazione: **bocciato una seconda volta e peggio** — vedi O1 |
| S2 — relay → pagina p95 < 100 ms (P2); cattura → disegno **riportato** | ✅ **`passa`** | relay → disegno: mediana **1 ms**, p95 **1 ms**, massimo 38 ms; cattura → disegno: mediana **114 ms**, p95 **144 ms**, massimo 203 ms; 627 campioni. ⚠️ Il criterio passa di cento volte perché misura un salto su `localhost`: vedi O5 |

**Il giudizio del proprietario sulla mano che muove il pannello, con le sue parole:** «il pinch lo prende e la mano si vede, solo che non si vede a schermo intero ma in una piccola area della pagina (non funzionale al successo del test, solo un'accortezza), lo scheletro della mano si vede e pinchando riesco a spostare la forma verde, unica cosa, sembra che io non debba obbligatoriamente pinchare la forma verder per poterla muovere (anche se pincho dove l'area è vuota riesco a muovere il rettangolo nella posizione dove si trova, inoltre anche facendo il pungo (va bene non fa niente, è anche comodo ma bisogna ponderare quali azioni servono per fare altri comandi come il click etc..), di base funziona».

### SP-7 · Osservazioni registrate — non criteri

| # | Osservazione |
|---|---|
| O1 | ⛔ **S1 è stato misurato due volte sulla stessa configurazione, e il verdetto regge mentre il numero balla.** Seconda corsa valida (898 risultati a due mani su 898): mediana **33,18 ms**, p95 **42,26 ms**, massimo 60,32 ms — cioè **peggio** della prima. Il p95 si muove del 19% fra due corse consecutive, quindi 35,54 non è un valore, è un punto in una nuvola. 📌 **Ciò che entrambe dicono:** il tracciatore costa **~33 ms per fotogramma a due mani** su questa CPU, che è **esattamente** il budget dei 30 Hz, con margine **zero**. I 17,12 ms del Pixel 6 della pagina F4 sono la metà |
| O2 | **Nessuna coda: la pipeline sta dietro alla telecamera.** In entrambe le corse valide i fotogrammi inviati senza risultato sono **2 su ~900**, ed è l'unico dato che il banco produce sul punto: `s1_bench.py` non legge mai `CAP_PROP_FPS`. Quindi la latenza misurata è **costo di calcolo**, non attesa in coda — l'ipotesi della coda è stata considerata e **cade sulla misura** |
| O3 | **Su questa macchina le due `cap.set()` di `s1_bench.py` sono una no-op:** il default della webcam è **già** 640×480. Misurato con una copia usa-e-getta dello script identica tranne quelle due righe, che stampa `camera frame: 640x480` lo stesso. Non cambia nulla per il criterio; conta per chi rileggesse il banco chiedendosi che cosa forzi davvero |
| O4 | ⚠️ **Perdere il tracciamento costa caro, ed è la ragione per cui il criterio pretende risultati a due mani.** In una corsa in cui le mani sono state fuori campo per più di metà del tempo — **359 risultati a due mani su 866** — la mediana sale a **37,93 ms** e il p95 a **63,95 ms**, coi fotogrammi scartati da 2 a **34**. ⛔ **Perché costi di più non è misurato da questa corsa, e non si inventa** — lo stesso metro che O5 applica agli ~80 ms. Quella corsa **non è confrontabile** col criterio: non raggiunge nemmeno la soglia dei 600 |
| O5 | ⛔ **Il criterio di S2 passa di cento volte perché guarda il salto che non era il rischio, e il costo vero non ha un nome.** `relay → pagina` è una scrittura su `localhost`: **1 ms** contro un budget di 100. Il numero che si sente addosso è l'altro, quello che il protocollo riporta **senza soglia**: **114 ms** mediani da cattura a disegno. La scomposizione, coi numeri di oggi: 1 ms è relay → pagina, **~33 ms** sono l'inferenza secondo S1, e **~80 ms restano senza spiegazione**. 📌 **Registrato come divergenza e NON spiegato:** nessuna misura di questa sessione li attribuisce, e inventare una causa plausibile la renderebbe più difficile da trovare |
| O6 | **Il pannello di `page.html` non ha nessun controllo di collisione: una pinza in qualunque punto lo afferra.** Notato dal proprietario provandolo, poi verificato nel sorgente: `pinch()` memorizza lo scarto fra dito e angolo del pannello quando la pinza si chiude e non confronta **mai** il dito col rettangolo. ⚠️ **Non è un difetto dello spike ma una scorciatoia dichiarata:** qui si misura il **giro**, non l'interazione. Per il sotto-progetto 12 è una delle cose da progettare |
| O7 | ⛔ **Un pugno chiuso vale come pinza, e conferma il confine che il protocollo dichiara.** La «pinza» di `page.html` è **solo** questo: punta del pollice e punta dell'indice più vicine di **40 pixel**, e un pugno soddisfa la condizione. Il proprietario l'ha notato provandolo e ne ha tratto la conseguenza giusta — *«bisogna ponderare quali azioni servono per fare altri comandi»*. 📌 È esattamente ciò che il protocollo mette **fuori** perimetro: lo spike non misura il riconoscimento di un gesto discreto, e il **vocabolario è della capacità** (F3, ADR-0038, ADR-0039) |
| O8 | **La scena della pagina è fissa a 640×480**, e la mano si disegna in quell'area invece che a schermo intero. È voluto: il worker manda i punti in **pixel interi** di un fotogramma 640×480 — la regola del disegno — e la pagina li disegna 1:1 senza scalarli. Cosmetico, notato dal proprietario |

### SP-7 · Versioni degli strumenti

| Strumento | Comando | Output |
|---|---|---|
| Python | `spikes/gesti/.venv/Scripts/python --version` | `Python 3.10.6` — **`py -3.10`**, perché `mediapipe` 1.0.1 vuole 3.9–3.12 (F1) |
| MediaPipe | `pip show mediapipe` | `Version: 1.0.1`; il resto dell'ambiente in `gesti/requirements.lock` |
| Rust, il relay | `rustc --version` | `rustc 1.95.0 (59807616e 2026-04-14)` |
| CPU | `Get-CimInstance Win32_Processor` | `Intel(R) Core(TM) i7-14700HX` |
| Telecamera | `Get-PnpDevice -Class Camera` | `HP True Vision FHD Camera` — la macchina ne espone anche una `HP IR Camera`, non usata |
| Modello | l'URL letto su F4 il 2026-09-04 | `hand_landmarker.task`, float16, 7819105 byte. ✅ **L'URL è la stessa letta il 2026-09-03**, riletta alla fonte il giorno della misura: `https://storage.googleapis.com/mediapipe-models/hand_landmarker/hand_landmarker/float16/latest/hand_landmarker.task` |

### SP-7 · Evidenze

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| S1, la corsa del criterio | `cd spikes/gesti && .venv/Scripts/python s1_bench.py --model hand_landmarker.task --seconds 30 --csv …` con il CSV **fuori dal repository** | `camera frame: 640x480` · `results 895  sent 897  dropped 2  results/s 28.5` · `latency ms: median 31.67  p95 35.54  max 54.98` · `results with two hands: 895 of 895` | ⛔ **L'attesa erano i 17,12 ms del Pixel 6 di F4, come speranza e non come prova, e la speranza cade: il costo è il doppio.** Il disegno lo dichiarava — *«lo fanno sperare, non lo provano»*. E ciò che cade è il **p95**, non la mediana: il tracciatore sta sulla riga dei 30 Hz e la supera nella coda |
| S1, la ripetizione | la copia usa-e-getta di O3 — `s1_bench.py` **senza le due `cap.set()` delle righe 59–60**, identica per il resto — stessa configurazione, cancellata dopo la corsa | `results 898  sent 900  dropped 2  results/s 28.9` · `latency ms: median 33.18  p95 42.26  max 60.32` · `results with two hands: 898 of 898` | ⛔ **Non era attesa nessuna ripetizione, e ha cambiato la lettura:** la seconda corsa è **peggiore** della prima e boccia anche la **mediana**. Una corsa sola avrebbe fatto leggere 35,54 come un valore |
| S1, la corsa con le mani fuori campo | la stessa copia usa-e-getta | `results 866  sent 900  dropped 34  results/s 27.8` · `latency ms: median 37.93  p95 63.95  max 70.33` · `results with two hands: 359 of 866` | **Fuori criterio per costruzione** (meno di 600 a due mani). Registrata perché misura il costo della perdita di tracciamento, che nessun criterio chiedeva — O4 |
| S2 | `cd spikes/gesti/relay && cargo run --release -- ../.venv/Scripts/python ../s2_worker.py ../hand_landmarker.task`, poi *dump stats* | `{ "samples": 627, "capture_to_draw_ms": { "median": 114, "p95": 144, "max": 203 }, "relay_to_draw_ms": { "median": 1, "p95": 1, "max": 38 } }` | ⛔ **Il criterio è passato di due ordini di grandezza, e questa è la divergenza:** ci si attendeva che il giro fosse il rischio, e il salto misurato dal criterio non lo è. Il rischio sta **a monte** del relay, dove **~80 ms su 114 non hanno una spiegazione misurata** — O5 |

## SP-6 — Confine dei dati non fidati, e confini statici del kernel

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| T1 non compila | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| T2 percorso unico | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| T3 ereditarietà | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| T4 aggiramento | ✅ `passa` | ✅ `passa` **ma solo dopo una correzione**, vedi sotto | ⚠️ **`parziale`** — tre vie, nessuna vietabile dal compilatore |
| T5 rilevabile globalmente | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| T6 importazione vietata, provata in negativo | ✅ `passa` | ✅ `passa`, con driver scritto a mano | ⚠️ **`parziale`** — regola del compilatore, ma zittibile per riga |

## SP-5 — Iniettabilità e riproducibilità

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| C1 stesso seed → stessa traccia | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| C2 seed diversi → tracce diverse | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| C3 tempo virtuale | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| C4 guasto riproducibile | ✅ `passa` | ✅ `passa` | ✅ `passa`, con **seed 4** e non 99 |
| C5 nessun orologio/RNG globale | ✅ `passa` | ✅ `passa` | ✅ `passa` |
| C6 concorrenza nativa ordinabile | ✅ `passa` | ❌ **`non passa`** | ⚠️ **`parziale`** |
| C7 I/O iniettabile, crash riproducibile | ✅ `passa` | ✅ `passa` | ✅ `passa` |

## Esito

| Candidato | SP-6 | SP-5 | Passa? |
|---|---|---|---|
| **Rust** | 6/6 `passa` | 7/7 `passa` | ✅ **sì, entrambi** |
| **Go** | 6/6 `passa` | 6/7 — **C6 `non passa`** | ❌ no |
| **TypeScript** | 4/6 — T4 e T6 `parziale` | 6/7 — **C6 `parziale`** | ❌ no |

Il protocollo è esplicito: *«un candidato passa solo se soddisfa **tutti** i criteri»*,
e *«un criterio soddisfatto con un accorgimento va registrato come parziale, non come
passato»*. **Rust è l'unico candidato che passa entrambi gli spike.**

Per la regola di applicazione di C6:

| Candidato | C6 | Lo spareggio #1 dell'ADR… |
|---|---|---|
| Rust | `passa` | **non si applica**: possiede il controllo |
| Go | `non passa` | si applica in pieno, con una misura |
| TypeScript | `parziale` | si applica, e l'evidenza dice **in quali condizioni** il controllo si perde |

## Osservazioni registrate — non criteri

| # | Rust | Go | TypeScript |
|---|---|---|---|
| O1 motore di persistenza conforme a §10.6 | candidati esistono: `redb` 4.1.0 · `fjall` 3.1.8 (LSM, adatto alla potatura selettiva) · `rusqlite` 0.40.1 · `sled` 1.0.0-alpha.124. **Requisito 4 (I/O iniettabile) da confermare** nell'ADR sulla persistenza: è il discriminante, non la disponibilità | candidati esistono: `go.etcd.io/bbolt` v1.5.0 · `github.com/dgraph-io/badger/v4` v4.9.6 · `github.com/cockroachdb/pebble` v1.1.5. Stessa riserva sul requisito 4 | non verificato: il candidato non passa nessuno dei due spike |
| O2 daemon a vita lunga, istanza singola | via consolidata; nessun runtime esterno da impacchettare, binario singolo | via consolidata; binario singolo. È il caso d'uso per cui il linguaggio è nato | richiede il runtime Node accanto all'eseguibile: il packaging non è un binario singolo |

## Versioni degli strumenti

| Candidato | Comando | Output |
|---|---|---|
| Rust | `rustc --version` | `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` · `clippy 0.1.95` · `trybuild 1.0.120` |
| Go | `go version` | `go version go1.26.5 windows/amd64` |
| TypeScript | `npx tsc --version` | `Version 5.9.3` · node `v24.9.0` · npm `11.6.0` · `@types/node` 26.1.2 |

## Seed usati

Un risultato senza seed non è valido.

| Criterio | Candidato | Seed | Note |
|---|---|---|---|
| C1, C2 | Rust | `42`, `43` | tracce identiche a parità di seed, diverse fra seed |
| C3 | Rust | `7` | orologio virtuale a 5000 ms, tempo di parete < 1 s |
| C4 | Rust | `99` | il seed inietta almeno un `GUASTO`; riprodotto identico |
| C6 | Rust | `20260806` | 100 esecuzioni → **1 sola** traccia distinta; con `20260807` l'interlacciamento cambia |
| C7 | Rust | `1, 7, 42, 99, 20260806` | tracce identiche a parità di seed, caduta inclusa |
| C7 dubbio | Rust | **`0`** | primo seed su 200 che cade *fra* intento ed esito: passo 0 resta `InDubbio`, rilevabile |
| C1, C2 | Go | `42`, `43` | come Rust |
| C3 | Go | `7` | orologio virtuale a 5000 ms, tempo di parete < 1 s |
| C4 | Go | `99` | il seed inietta un guasto; `TestC4` non è stato saltato |
| C6 | Go | — | **il seed non entra**: non c'è alcun punto in cui inserirlo nello scheduler delle goroutine. È il risultato, non un'omissione |
| C7 | Go | `1, 7, 42, 99, 20260806` | tracce identiche a parità di seed, caduta inclusa |
| C7 dubbio | Go | **`0`** | stesso esito di Rust |
| C1, C2 | TypeScript | `42`, `43` | come gli altri due |
| C3 | TypeScript | `7` | orologio virtuale a 5000 ms, tempo di parete < 1 s |
| C4 | TypeScript | **`4`**, non 99 | RNG a 32 bit: sequenza diversa. Primi seed validi: 1, 4, 6, 10, 11, 12 |
| C6 (a) | TypeScript | `20260806` | 100 esecuzioni con generatori sotto esecutore proprio -> 1 traccia |
| C7 | TypeScript | `1, 7, 42, 99, 20260806` | tracce identiche a parita di seed, caduta inclusa |
| C7 dubbio | TypeScript | **`0`** | passo 4 resta `InDubbio` |

## Evidenze

Una riga per criterio e candidato: comando eseguito, output osservato, e **le
divergenze** rispetto a ciò che ci si aspettava. Una divergenza non registrata è un
risultato perso.

### SP-6 · Rust — eseguito il 2026-08-06, rustc 1.95.0

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **T1** | `cargo test --test compile_fail` | `error[E0308]: mismatched types … expected &Instruction, found &Untrusted`. **Provato non vacuo**: rendendo compilabile la violazione il test passa a `FAILED`, ripristinandola torna `ok` | il piano prevedeva questo esito; confermato |
| **T2** | ricerca testuale su `src/` | una sola funzione, `Untrusted::promote_to_instruction`. I campi delle due struct non sono pubblici: nessun'altra via di costruzione dall'esterno | nessuna |
| **T3** | `cargo test --test boundary` | `summarize(&Untrusted) -> Untrusted`: la firma **impone** l'ereditarietà, non la raccomanda | nessuna |
| **T4** | `cargo build` con `#![forbid(unsafe_code)]` + `#[allow(unsafe_code)]` locale | `error[E0453]: allow(unsafe_code) incompatible with previous forbid`. **`forbid` non è scavalcabile per riga**, a differenza di `deny` | nessuna. Per la regola di decisione del protocollo è `passa`, non `parziale`: il divieto è del compilatore |
| **T5** | `cargo build` | la compilazione dell'intero progetto è essa stessa il controllo: non esiste un sito d'uso che si possa dimenticare di controllare | nessuna |
| **T6 (a)** lint | `cargo clippy -- -D clippy::disallowed_methods -D clippy::disallowed_types` | ferma `SystemTime::now`; **`cargo build` da solo NON la ferma**. Il divieto vive in `clippy.toml`, è configurabile e disattivabile con `#[allow]` | — |
| **T6 (b)** compilatore | `cargo build -p kernel_core` su una crate `#![no_std]` | `error[E0433]: cannot find module or crate 'std'`. Non è un lint: è ciò che il compilatore ha caricato. Provato in **entrambe** le direzioni | — |

**Nota strutturale su T6.** In Rust entrambi i meccanismi sono a **granularità di
crate**, non di modulo. Conseguenza architetturale, non dettaglio: il kernel dovrebbe
essere una crate propria, e il modulo di piattaforma una crate separata. È coerente con
I3, ma va detto perché vincola il layout dei sorgenti dal primo giorno.

**Scoperta collaterale, non cercata.** `std::collections::HashMap` è stata inserita fra
i tipi vietati: `RandomState` è seminato casualmente **per processo**, quindi l'ordine
di iterazione non è riproducibile fra esecuzioni. È una violazione di V29 che non
compare in nessun elenco di «chiamate OS» e che C1 scoprirebbe solo come traccia
divergente e inspiegabile. Vale per ogni candidato: va verificata anche su Go e
TypeScript.

### SP-5 · Rust — eseguito il 2026-08-06, rustc 1.95.0

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **C1–C4** | `cargo test --test sched` | 4/4. Il seed 99 inietta un guasto **senza doverne cercare un altro**: il piano prevedeva di doverlo sostituire | il piano prevedeva un possibile skip; non è servito |
| **C5** | `grep -rnE "Instant::now\|SystemTime\|rand::\|thread_rng\|std::fs\|std::net\|std::env\|HashMap" src/ kernel_core/src/` escludendo i commenti | nessun riscontro. **Provato non vacuo**: inserendo `SystemTime::now()` in `sched.rs` il grep lo trova | il grep iniziale, senza escludere i commenti, dava un falso positivo su un `//!` |
| **C6 (a)** | `cargo test --test c6` | `Future` native guidate da un esecutore proprio: **100 esecuzioni, seed 20260806, 1 sola traccia distinta**. Interlacciamento reale verificato, altrimenti il determinismo sarebbe vacuo | nessuna |
| **C6 (b)** controprova | idem | `std::thread` dell'OS: **> 1 traccia distinta su 100**. Stabilisce il confine di C6 — non è un criterio che tutti superano per costruzione | nessuna |
| **C7** | `cargo test --test c7` | 6/6. Crash riproducibile su 5 seed; l'ordine è write-ahead (`I,E,I,E,I,E`); il passo `InDubbio` è rilevabile e **senza falsi positivi** quando non c'è crash; il giornale è sostituibile con un secondo doppio senza toccare il codice sotto test | nessuna |
| ecosistema | `cargo add --dry-run madsim` | `Adding madsim v0.2.34`. Esiste un runtime deterministico di ecosistema che **sostituisce tokio**; `turmoil` 0.7.2 è l'alternativa | nessuna |

**Il dato che distingue Rust, in una riga.** L'ordine delle unità concorrenti native è
deciso dal **nostro** esecutore, non dal runtime: `Future` è un oggetto che si sceglie
quando far avanzare. Non serve uno strumento di test per ottenerlo, e vale anche fuori
dai test — che è la differenza fra controllo *posseduto* e *fornito*.

**Il costo, misurato e non stimato.** La regola di T6 (a) è a granularità di **crate**
e ha bloccato un uso **legittimo** di `Instant::now()` dentro il test C3, che deve
misurare il tempo di parete proprio per provare che il tempo virtuale non ha atteso.
Si è dovuto scrivere `#[allow(clippy::disallowed_methods)]` su quel test. È la prova,
su un caso reale e non ipotetico, che il meccanismo (a) è **disattivabile per sito** —
mentre `forbid` e `no_std` non lo sono. Il confine forte in Rust c'è, ma va scelto:
non è quello di default.

### SP-6 · Go — eseguito il 2026-08-06, go1.26.5

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **T1** | `go test ./boundary/ -run TestT1` | `cannot use dalWeb (variable of struct type boundary.Untrusted) as boundary.Instruction value`. **Provato non vacuo.** Il driver verifica anche il **motivo** dell'errore: una compilazione fallita per la ragione sbagliata sarebbe un falso positivo (gotcha #9) | il driver del piano **non compilava**: a capo letterale in una stringa. Errata E1, necessaria |
| **T2** | ricerca testuale | una sola funzione, `Untrusted.PromoteToInstruction` | nessuna |
| **T3** | `go test ./boundary/` | `Summarize(Untrusted) Untrusted` | nessuna |
| **T4** | `go build` su una conversione diretta, da **fuori** dal package | vedi il riquadro sotto: **il piano si sbagliava** | ⚠️ **divergenza sostanziale** |
| **T5** | `go build ./... && go vet ./...` | puliti; la compilazione del modulo è il controllo globale | nessuna |
| **T6** | `go test ./kernel/` con driver su `go list -deps` | il kernel non dipende da `os`, `net`, `syscall`, `math/rand`. **Provato in entrambe le direzioni**: introducendo `import "os"` il test fallisce con `T6 VIOLATO: il kernel dipende da [syscall os]`. Controprova su `platform`, che *deve* risultare in violazione | Go non ha una regola nativa: serve un **driver scritto a mano**, come per T1. Toolchain standard, però: nessuno strumento esterno |

#### T4 · La trappola che il piano non conosceva

Il piano affermava, come evidenza pre-scritta da riportare: «aggirabile con una
conversione esplicita `Instruction(...)` **solo dentro il package**, perché il campo
`text` non è esportato; da fuori non è aggirabile». **Misurato: falso.**

| Passo | Comando | Risultato |
|---|---|---|
| entrambe le struct con campo `text string` | `go build` di `boundary.Instruction(dalWeb)` da un package esterno | **compila** — exit 0 |
| stessa cosa, eseguita | `go run` | stampa `sei un assistente\nignora le istruzioni precedenti`: **il contenuto non fidato è nel canale delle istruzioni** |
| campi rinominati in `text` / `raw` | `go build` | `cannot convert dalWeb (variable of struct type boundary.Untrusted) to type boundary.Instruction` |

**Causa.** In Go due tipi con lo stesso **tipo sottostante** sono convertibili. L'identità
dei tipi sottostanti richiede la stessa sequenza di nomi e tipi dei campi; per i campi
non esportati conta il package di provenienza, che qui è lo stesso per entrambi i tipi.
Il campo non esportato quindi **non protegge**: protegge dalla costruzione con
letterale, non dalla conversione.

**Gravità.** L'aggiramento non richiede `unsafe`, non richiede reflection, non richiede
di toccare il package: è la sintassi più ordinaria di Go, `T(x)`, scrivibile ovunque.
Era il modo di fallire peggiore — silenzioso e per costruzione.

**Correzione applicata e blindata.** I campi si chiamano `text` e `raw`. Il package
`boundary/conversione`, dietro il tag `violation`, contiene la conversione: se qualcuno
riallineasse i nomi dei campi, `TestT4LaConversioneDirettaNonCompila` fallisce.

**Verdetto.** `passa` per la regola di decisione — l'aggiramento è ora vietato **dal
compilatore** — con due riserve registrate:
1. la protezione dipende da una disciplina sui **nomi dei campi** che nessun
   compilatore impone e che non è nella documentazione del linguaggio come tale;
2. `unsafe` resta una via, ma è **vietabile con lo stesso meccanismo di T6**: verificato
   che il package `unsafe` compare in `go list -deps`. Go non ha l'equivalente di
   `#![forbid(unsafe_code)]`, quindi il divieto è un test, non un attributo.

#### Scoperta collaterale · l'ordine di iterazione delle `map`

Misurato: **200 iterazioni della stessa map, nello stesso processo → 8 ordini
distinti.** È la randomizzazione deliberata del runtime Go, e non è disattivabile.

È l'analogo del `HashMap` di Rust, ma con una differenza pratica sostanziale: in Rust
si sostituisce `HashMap` con `BTreeMap` e il problema sparisce; in Go `map` è il tipo
**incorporato** e non esiste un'alternativa ordinata nella libreria standard — le
chiavi vanno estratte e ordinate a ogni iterazione, ogni volta, per sempre.

Per V29 è una fonte di non determinismo che non compare in nessun elenco di «chiamate
OS» e che C1 rivelerebbe solo come traccia divergente e inspiegabile.

### SP-5 · Go — eseguito il 2026-08-06, go1.26.5

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **C1–C4** | `go test ./sched/` | 4/4. Il seed 99 inietta un guasto: `TestC4` **non** è stato saltato | il piano prevedeva un possibile skip; non è servito, come in Rust |
| **C5** | `grep -rnE "time\.Now\|math/rand\|os\.\|net\." sched/ giornale/ kernel/` escludendo i commenti | nessun riscontro | nessuna |
| tempo virtuale | `go test ./sched/ -run TestGoroutineReali` | PASS in < 1 s di tempo di parete: **`synctest` virtualizza davvero il tempo** anche per goroutine e timer reali. La firma `synctest.Test(t, func(*testing.T))` è quella del piano | nessuna: su questo il piano era corretto |
| **C6** | `go test ./sched/ -run TestC6 -count=1` | vedi il riquadro sotto | ⚠️ **il criterio non è soddisfatto** |
| **C7** | `go test ./giornale/` | 6/6, esattamente come Rust. Crash riproducibile su 5 seed, ordine `I,E,I,E,I,E`, passo `InDubbio` rilevabile senza falsi positivi, giornale sostituibile con un secondo doppio | nessuna |

#### C6 · La misura che chiude la domanda aperta

L'ipotesi da falsificare era: *«lo spareggio #1 è troppo severo verso Go, perché se il
kernel guida le proprie attività con un esecutore proprio, che lo scheduler delle
goroutine sia del runtime conta poco.»*

**Falsificata.** 100 esecuzioni della stessa scena, 3 goroutine in contesa, 6 passi
ciascuna:

| Prova | Dentro `synctest` | Fuori dalla bolla |
|---|---|---|
| contesa su **canale** della bolla — il caso più favorevole, durably blocking | **9** tracce distinte | 13 |
| contesa su **`sync.Mutex`** — escluso testualmente dal durably blocking | **4** tracce distinte | 5 |

`synctest` **riduce** il non determinismo, non lo elimina. È coerente con la propria
documentazione: promette quiescenza — il tempo avanza quando ogni goroutine della bolla
è durably blocked — e **non promette un ordine totale**. La formulazione diffusa
«synctest dà scheduling deterministico» è più forte del contratto reale.

**Perché non basta un esecutore proprio.** L'esecutore ordina le attività *che gestisce
lui*. Ma il kernel di ADR-0004 è un daemon che riceve eventi concorrenti da IPC, da
worker e dalla rete: quelle goroutine esistono, e il loro interlacciamento resta del
runtime. La riga della tabella sul `sync.Mutex` è la più pesante: ADR-0004 descrive
l'arbitro GPU come «un unico processo con **un unico lock**», cioè esattamente la
primitiva che `synctest` dichiara di non coprire.

**Conseguenza sui requisiti.** Q2 (zero OOM), Q4 (kill di un worker in qualsiasi
istante) e Q5 (riavvio a metà run) sono verificabili **solo** per simulazione
deterministica (ADR-0021, design/08). Con l'interlacciamento non riproducibile, un
difetto trovato in simulazione non conserva il proprio seed — e V31 cade con lui.

**I test restano nel repository come guardie**, non come fallimenti: asseriscono il non
determinismo misurato. Se una versione futura di Go lo eliminasse, falliscono e C6 va
rimisurato.

### SP-6 e SP-5 · TypeScript — eseguito il 2026-08-06, tsc 5.9.3 su node 24.9

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **T1–T3** | `npm run typecheck` | i branded types reggono: `@ts-expect-error` è *usato* su entrambe le violazioni | nessuna |
| **T1 non-vacuità** | rimozione del marchio | ⚠️ **il piano indicava la sonda sbagliata.** Togliendo il marchio da `Untrusted` il typecheck **passa comunque**, perché `Instruction` resta marchiato e una `string` semplice non gli è assegnabile. La sonda corretta è togliere il marchio da **`Instruction`**: allora escono i due `TS2578: Unused '@ts-expect-error' directive` che il piano si aspettava | ⚠️ **divergenza** |
| **T4** | `tsc` su tre vie di aggiramento | **tutte e tre compilano**: `dalWeb as any` · `dalWeb as unknown as Instruction` · `<Instruction><unknown>dalWeb`. Nessun flag del compilatore le vieta | il piano ne citava **una**; sono tre, e la seconda sopravvive al divieto di `any` |
| **T5** | `tsc --noEmit` | è il controllo globale del progetto | nessuna |
| **T6** | `tsc -p tsconfig.kernel.json` con `"types": []` | ✅ meccanismo **del compilatore**, non un lint: `TS2307: Cannot find module 'node:fs'`. Provato in entrambe le direzioni, con controprova su `platform` che *deve* fallire. **Ma**: `// @ts-ignore` sopra l'import lo zittisce, exit 0 | il piano dava per scontato che in TS servisse un lint. È **meglio** di così — ma resta zittibile per riga |
| **C1–C3, C5, C7** | `npm test` | come Rust e Go. C7 6/6: crash riproducibile, ordine write-ahead, passo `InDubbio` rilevabile | nessuna |
| **C4** | `npm test` | ⚠️ il **seed 99 non inietta guasti**: l'RNG qui è a 32 bit (`>>> 0` a ogni passo), in Rust e Go a 64. Sequenza diversa a parità di seed. Primi seed validi misurati: **1, 4, 6, 10, 11, 12**. Usato **4**, registrato | il piano lo prevedeva e chiedeva di registrarlo: fatto |
| **C6** | `npm test` | vedi il riquadro sotto | — |

#### T4 · Tre vie, non una

| Via | Sopravvive al divieto di `any`? | Vietabile dal compilatore? |
|---|---|---|
| `dalWeb as any` | no | no — serve un lint |
| `dalWeb as unknown as Instruction` | **sì** | no |
| `<Instruction><unknown>dalWeb` | **sì** | no |

Il piano prevedeva `parziale` citando solo `as any`. Il verdetto regge, ma la motivazione
va corretta: vietare `any` **non basta**, perché la doppia asserzione via `unknown`
resta. Servono almeno due regole di lint, entrambe esterne e disattivabili per riga.

#### C6 · Il controllo esiste, ma a un prezzo che tocca ADR-0004

| Via | Tracce distinte su 100 | Cosa dimostra |
|---|---|---|
| **(a)** generatori guidati da un esecutore proprio, seed 20260806 | **1** | il controllo c'è, ed è ordinabile dal seed |
| **(b)** funzioni `async` sul ciclo di eventi | **1** | determinismo **per assenza di concorrenza**, non per controllo |

La riga (b) è la più importante e va letta con attenzione: è deterministica, ma il
**seed non entra da nessuna parte**. Il ciclo di eventi è a thread singolo e le microtask
si accodano in ordine di creazione: non c'è un ordine *scelto*, c'è l'assenza di scelta.

Verdetto **`parziale`**, con le condizioni in cui il controllo si perde:

1. in JavaScript una `Promise` **non è ispezionabile** — non esiste un `poll` che
   permetta a un esecutore di decidere quando farla avanzare. Per riprendere il
   controllo bisogna **rinunciare a `async`/`await`** e scrivere il kernel in
   generatori, che è l'unica primitiva del linguaggio in cui il punto di sospensione
   torna al chiamante;
2. il parallelismo reale richiede `worker_threads`, il cui ordinamento **non è
   controllabile dall'applicazione**;
3. ADR-0004 richiede un daemon a **concorrenza reale**. La via (a) la ottiene solo
   restando a thread singolo, cioè rinunciando al requisito.

