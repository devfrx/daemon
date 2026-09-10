# Risultati degli spike

Data di esecuzione: **2026-08-06** per SP-5 e SP-6; **SP-7** e **SP-8** portano la propria data nella loro sezione

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

## SP-8 — Il guscio della GUI, e l'accettazione di `dockview` — misure il 2026-09-10, giudizio del proprietario il 2026-09-10

Criteri e soglie: [`gui-shell/PROTOCOLLO.md`](gui-shell/PROTOCOLLO.md), congelato il 2026-09-10
al primo commit di codice dello spike, **prima** della misura. Codice in `gui-shell/`: la Home finta in `app/`, i due
gusci in `electron/` e `tauri/`, gli script di misura in `measure/`; i campioni grezzi (CSV) fuori dal repository. Le
due app sono state **installate** dagli installatori NSIS per utente e misurate da lì, col flusso dell'emettitore di
`gui-ipc/` com'è — righe JSON, `"lorem ipsum dolor sit amet"` su ogni riga, quindi markdown senza blocchi di codice.
Le otto mosse, Q3, Q4, la CPU con la chat nascosta e la VRAM a pagina intera: **col proprietario il 2026-09-10**, qui sotto.

**La macchina:** CPU `Intel(R) Core(TM) i7-14700HX`, GPU `Intel(R) UHD Graphics`, `NVIDIA GeForce RTX 4060 Laptop GPU` — la webview sull'integrata, come dice `api=` nelle righe M3 (richiamo E9 del protocollo: non è la macchina di ADR-0002), WebView2 `152.0.4191.66`, il Chromium di Electron `Chrome/152.0.7977.78`.

| Misura | Criterio | Electron — corsa 1; corsa 2 | Tauri — corsa 1; corsa 2 |
|---|---|---|---|
| M1 — RAM a riposo, media / picco (MB) | si riporta | 501,7 / 558; 439,5 / 470,3 | 485,6 / 519,3; 486,3 / 517,9 |
| M1 — RAM sotto flusso, media / picco (MB) | si riporta | 575 / 585,4; 526,7 / 541,9 | 570,8 / 588,3; 571,1 / 590,4 |
| M2 — cartella installata (file, MB) e installatore (MB) | si riporta | `C:\Users\zagor\AppData\Local\Programs\sp8-electron`: 74 file, 368,9 MB; installatore 106,4 MB | `C:\Users\zagor\AppData\Local\sp8-tauri`: 2 file, 8,6 MB; installatore 2,1 MB |
| M3 — fps medi / minimi sugli ultimi 30 s **a riposo**, l'API ottenuta, la taglia della tessera | si riporta, su Windows | 224 / 16, `api=WebGPU:intel/gen-12lp`, `scene=446x318`; 213 / 7 | 238 / 214, `api=WebGPU:intel/gen-12lp`, `scene=100x349`; 238 / 207 |
| M3 — lo stesso **sotto flusso** (gli ultimi 30 s della corsa) | si riporta | 226 / 140; 200 / 61 | 199 / 58; 200 / 57 |
| M4 — CPU dell'albero sotto i 2000 messaggi, picco / media (% di un core) | **picco < 25 %** (P3) | 196,8 / 116,9; 225,3 / 127,7 — ❌ `non passa` | 209,9 / 115,7; 206,4 / 112,5 — ❌ `non passa` |
| M4 — CPU a riposo, la sola scena che gira, picco / media | senza soglia, per leggere M4 | 129,4 / 77,1; 96 / 22,5 | 120,5 / 73,3; 88,9 / 67,5 |
| M4 — CPU con la chat **nascosta**, picco / media (% di un core) | senza soglia | 146,8 / 111,6 | 138,1 / 97,3 |
| M5 — VRAM a riposo, picco (MB): dedicata / condivisa — su questa macchina la webview gira sull'integrata, dove la dedicata è zero per costruzione (richiamo E10 del protocollo) | si riporta | 0 / 126; 0 / 112,6 | 0 / 85,9; 0 / 85,9 |
| M5 — VRAM con la scena **a pagina intera**, picco (MB): dedicata / condivisa | si riporta | 0 / 167 | 0 / 168,8 |
| P1, P2 dal titolo — `msgs= lost= holes=`, `p2max= p2mean=` | si riporta — `p2mean` con un errore di troncamento fino a 1 ms (`Date.now()` in millisecondi contro `emesso_micros`) | msgs=2000 lost=0 holes=0 p2max=43.9ms p2mean=3.13ms; msgs=2000 lost=0 holes=0 p2max=41.3ms p2mean=11.91ms | msgs=2000 lost=0 holes=0 p2max=33.6ms p2mean=6.03ms; msgs=2000 lost=0 holes=0 p2max=26.9ms p2mean=5.96ms |
| processi nell'albero, minimo / massimo per corsa | si riporta | 4 / 4; 4 / 4 | 7 / 7; 7 / 7 |
| Q1 — chi decodifica `bincode` lato GUI | letto dall'architettura, non misurato | il processo principale Node — `electron/main.js` — con `bincode-ts` (M-11 di ADR-0037) | il guscio Rust — `tauri/src-tauri/src/main.rs` — col decodificatore del kernel |
| Q2 — le webview in uso | letto | il Chromium impacchettato: `Chrome/152.0.7977.78` | WebView2 `152.0.4191.66` dal registro; **WebKitGTK**, alle fonti il 2026-09-10: assente dalla tabella Safari/WebKit di gpuweb (solo macOS, iOS/iPadOS, visionOS — nessuna riga Linux/GTK) — https://github.com/gpuweb/gpuweb/wiki/Implementation-Status |
| Q3 — la finestra staccata si apre dentro il guscio | per guscio, dal comando ↗ | REFUSED, nessuna finestra: dockview 8.2.0 rifiuta prima di aprirla perché la pagina vive su `file://` («popout URL must be same-origin http(s)»); il guscio la ammetterebbe — O7 | REFUSED, nessuna finestra: la pagina vive su `http://tauri.localhost`, dockview chiama `window.open` e WebView2 rende null («perhaps you need to allow pop-ups») — O7 |
| Q4 — `dndStrategy` che basta nella webview | per guscio, la mossa 2 col mouse | `'auto'` basta; provata anche `'pointer'`, va | serve `'pointer'`: con `'auto'` il trascinamento non parte |

**Le otto mosse — il giudizio del proprietario, con le sue parole**, nel browser il 2026-09-10 (il suo Chrome a `http://localhost:5173`, schermo 2048×1280): `dndStrategy` `auto` per le mosse 1–7 e `pointer` per la 8.

| # | La mossa | Esito | La barra, testuale | Le sue parole |
|---|---|---|---|---|
| 1 | il nucleo e la striscia non si spostano | ✅ `passa` | — | «si a tutto» — una risposta sola alle cinque domande sulle mosse 1, 2, 4, 5 e sul sì tecnico della 8 |
| 2 | sgancia, galleggia, riaggancia | ✅ `passa` | `move 2: hand floats` (provata sulla tessera Mano) | «si a tutto» |
| 3 | pagina intera e ritorno | ✅ `passa` | `move 3: hand full page`, `move 3: hand back`; poi `move 3: passi back` | «e sembra funzionare tutto» |
| 4 | in un'altra finestra, dal comando | ✅ `passa` | `move 4 / Q3: popout OPENED for permessi` | «si a tutto»; «move 4 / Q3: popout OPENED for stato da browser» — la seconda finestra, su Stato, l'ha riferita lui in chat e non sta nel file della console |
| 5 | la presa grande col mouse; col tocco non misurato perché lo schermo non è touch | ✅ `passa` | — | «si a tutto» |
| 6 | con la tastiera, `Ctrl+Alt+frecce` | ✅ `passa` | `move 6: passi -> group 9`, `move 6: passi -> group 12`, `move 6: passi -> group 9`, `move 6: passi -> group 7`, `move 6: passi -> group 9`, `move 6: passi -> group 12` | «ctrl alt frecce e h», «e sembra funzionare tutto» |
| 7 | salva, ricarica, ritrova — una misura | ✅ `passa` sul canonico | `move 7: canonical EQUAL; raw DIFFERENT at byte 1798 (3052 vs 3052 bytes)` — dopo «azzera», «salva (mossa 7)», «ricarica»; il grezzo differisce solo per l'ordine delle chiavi (richiamo E4 del protocollo); il caso di una disposizione salvata con una finestra a parte è in O6 | «salva e reload:» |
| 8 | la pinza afferra la presa grande e la tessera segue — il sì o no **tecnico** | ✅ `passa`, sì tecnico | 137 pinze in tutto: 15 sulla presa grande — 10 `on div.bigtab`, 5 `on span.bigtab-title` — 5 sui bordi (`on div.dv-sash`), 117 sul corpo delle tessere o nel vuoto; per esempio `move 8: pointerdown at 270,444 on div.bigtab` … `move 8: pointerup at 843,696 on div.tile`; con la pinza sulla presa grande la tessera ha seguito la mano | «non capisco quando è rosso/verde il pallino che si trova sull'indice cosa sta ad indicare, inoltre il movimento della mano nella vita reale rapportato con quello sulla ui è strano: per arrivare a pinzare una tab verso la fine (in altezza) del monitor esco dall'inquadratura della camera, oltre al fatto che devo essere super preciso (scomodo) per pinzare un bordo di qualcosa o una tab da spostare»; «i pulsanti affianco al titolo della tab di una scheda con la mano non riesco a premerli solo» — senza soglia; il pallino è il cursore, verde a pinza aperta e rosso a pinza chiusa (`hand.ts`), e i comandi accanto al titolo non rispondono a un puntatore sintetico perché fermano la propagazione senza produrre un click |

**`dockview`:** resta — otto su otto.

**La decisione sul guscio, del proprietario, il 2026-09-10: A — Electron.** La tabella a due colonne che ha avuto davanti è quella qui sopra, M1–M5 e Q1–Q4; il consiglio del coordinatore era A, derivato dall'asimmetria dei costi di ADR-0029 letta contro i numeri: dove i due sono pari (M1, M3, M4 bocciati allo stesso modo, M5) decide l'asimmetria, e Q2 su Linux, Q3 senza rimedio e Q4 col trucco sono costi di capacità di Tauri, mentre M2 è un costo di peso di Electron. Le sue parole: «A».

### SP-8 · Osservazioni registrate — non criteri

| # | Osservazione |
|---|---|
| O1 | **M1 balla poco per Tauri e parecchio per Electron; M4 a riposo balla per entrambi; M4 sotto flusso resta sopra soglia in ogni corsa, quindi il verdetto P3 non cambia con una corsa sola.** M1 a riposo, media: Electron 501,7 poi 439,5 MB (−12,4 %), Tauri 485,6 poi 486,3 MB (variazione trascurabile). M1 sotto flusso, media: Electron 575 poi 526,7 MB (−8,4 %), Tauri 570,8 poi 571,1 MB (praticamente identica). M4 a riposo, media: Electron 77,1 poi 22,5 % — un fattore 3,4×: **una corsa sola avrebbe cambiato la lettura**; Tauri 73,3 poi 67,5 % (−7,9 %). M4 sotto flusso, picco: Electron 196,8 poi 225,3 % (+14,5 %), Tauri 209,9 poi 206,4 % (−1,7 %) — ma entrambe le corse di entrambi i gusci restano ben sopra il 25 %, quindi qui il verdetto non balla anche se il numero sì. M5 a riposo, condivisa: Electron 126 poi 112,6 MB (−10,6 %), Tauri 85,9 poi 85,9 MB (identica). ⚠️ **Richiamo del 2026-09-10, alla revisione del compito 6 — una terza corsa di Electron ribalta la lettura di M4, e va capita prima di fidarsi del verdetto.** Il revisore ha rifatto una corsa per guscio dalle app installate, stessi comandi (`-Seconds 60 -EmitterAt 32`): Tauri come le ufficiali (`stream: cpu_pct mean 111,5 max 216,3`, `fps=201`); Electron **no**: `stream: samples 16, rss_mb mean 446,3 max 447,2, cpu_pct mean 4,3 max 17,4, vram_shared_mb max 85,2` e `last title: fps=235 min=211 scene=99x318, msgs=2000 lost=0 holes=0 p2max=17.5ms p2mean=0.44ms` — P3 **passerebbe**. Letto nel CSV dal coordinatore: `procs` 4 costante (nessun processo perso), e i campioni di CPU a riposo `0 93 92 78 97 90 98 90 93 92 2 1 0 2 0 0 11`, sotto flusso `0 2 17 8 9 17 7 6 0 0 0 0 0 0 0 1` — il renderer si è **fermato** a ~20 s dal via e non è più ripartito, mentre il titolo diceva ancora ~235 fps. La stessa forma sta **dentro** le corse ufficiali: la corsa 2 di Electron ha i primi ~25 s di riposo a ~0 % (`0 16 0 0 0 2 1 0 0 2 0 0 2 0 80 84 96 76 68`) — è la ragione della sua media 22,5 % — e la corsa 1 qualche campione a 0 (`0 80 1 0 52 …`); Tauri mai, in cinque corse. **Dedotto, non misurato:** Chromium strozza il renderer di una finestra coperta o in secondo piano (la console dell'emettitore compare sopra il guscio, vista nello screenshot del revisore), e il contatore degli fps nel titolo non lo rivela. Finché non è misurato — al compito 7, con una corsa a finestra coperta apposta — il verdetto `❌ non passa` di P3 per Electron poggia sui picchi presi **a finestra visibile** (196,8 e 225,3 %), e una corsa in cui la finestra non è in primo piano **non conta**: né per passare né per bocciare ✅ **Richiamo del 2026-09-10, compito 7 — MISURATO, in parte.** Due corse dall'app installata (`tree.ps1 -Seconds 30 -EmitterAt 8`, relay e Vite spenti): una con la finestra **coperta apposta** da un Blocco note massimizzato dal sesto secondo alla fine, una in primo piano. Coperta e a riposo la CPU dell'albero cade a **1,8 e 1,7 %** in due campioni e il titolo **si congela** a `fps=228 min=220` — non è più aggiornato, quindi mostra l'ultimo valore alto: è la forma «~0 % col titolo ancora a ~230 fps», riprodotta a riposo. All'arrivo dei messaggi la CPU risale e i picchi sono uguali a quelli in primo piano (`stream: cpu_pct max 215,8` coperta contro `215,9` in primo piano) e il contatore riparte con `min=26`; la forma «tutta la corsa a ~0 %, anche sotto flusso» della terza corsa non si è ripetuta e resta non spiegata. Il verdetto ❌ di P3 per Electron regge; i comandi e le righe delle due corse stanno nelle Evidenze, i CSV sono `electron-covered.csv` ed `electron-front.csv` |
| O2 | **La sola scena `three` a riposo sta già vicino o sopra la soglia di P3, prima di qualunque messaggio.** CPU a riposo, media: Electron 77,1 % e 22,5 %, Tauri 73,3 % e 67,5 %; CPU a riposo, picco: Electron 129,4 % e 96 %, Tauri 120,5 % e 88,9 % — sopra il 25 % di [GUI-REQUISITI.md](GUI-REQUISITI.md) (P3) in sette valori su otto con la sola scena che gira, senza un solo messaggio (l'ottavo, la media a riposo 22,5 % della corsa 2 di Electron, è abbassato dai ~25 s a ~0 % del richiamo in O1; correzione del 2026-09-10 alla revisione: questa frase diceva «già sopra il 25 %» per tutti e otto). Il salto al picco sotto flusso è comunque reale — da 129,4/96 a 196,8/225,3 per Electron, da 120,5/88,9 a 209,9/206,4 per Tauri — quindi le due componenti pesano entrambe: la scena da sola sfonda già il budget, e i 2000 messaggi con markdown reso ne aggiungono altro sopra. Il limite dichiarato dal protocollo — JSON e non `bincode` — non è misurato in questa sessione: nessuna corsa confronta i due decodificatori, quindi quanto del picco sotto flusso verrebbe eroso passando a `bincode` resta una domanda aperta, non un numero |
| O3 | **M3 a riposo contro sotto flusso, per guscio: fps medi stabili, `min` che si muove in direzioni opposte fra i due gusci, e la taglia della tessera che cambia senza che la finestra cambi.** fps medi: Electron 224→226 e 213→200; Tauri 238→199 e 238→200 — nessun crollo medio. fps minimi: Electron **sale** da 16→140 e da 7→61 (compatibile con una finestra mobile di 30 s che, letta a fine flusso, ha lasciato cadere lo scatto d'avvio ancora dentro la lettura `rest` a ~32 s); Tauri **scende** da 214→58 e da 207→57 (qui il calo cade dentro la finestra di flusso, non nell'avvio) — la stessa metrica racconta due storie opposte a seconda del guscio. La taglia della tessera (`scene=`) non è stabile come atteso: la finestra è fissa a 1400×900 in entrambi i sorgenti (`electron/main.js`, `tauri/src-tauri/tauri.conf.json`), eppure Electron mostra `scene=446x318` a riposo e `scene=215x318` sotto flusso nella corsa 1 — cambia **dentro la stessa corsa** — e `scene=99x318` in entrambe le fasi della corsa 2; Tauri resta `scene=100x349` in tutte e quattro le letture. Nessuna misura di questa sessione spiega perché il layout di Electron si muova fra corse e fasi mentre quello di Tauri no: registrato, non spiegato |
| O4 | **I processi dell'albero sono costanti per guscio, e per Tauri la trappola di M1 non ha morso.** Electron: 4 processi in entrambe le corse (minimo = massimo = 4). Tauri: 7 processi in entrambe le corse (minimo = massimo = 7) — sopra 1, quindi WebView2 sta **dentro** l'albero letto da `tree.ps1`, coerente con le corse del compito 5 (anch'esse a 7 costante): la voce d'errata che il piano prevede per `procs` fisso a 1 non si applica, M1/M4/M5 di Tauri non sono parziali per questa ragione |
| O5 | **Il campionatore gira a ~1,7–1,9 s per campione, non ai 250 ms nominali (errata E11): le medie e i picchi di M1 e M4 poggiano su 16–19 campioni per fase, non su ~120–240.** Electron corsa 1: rest 17 campioni in ~32 s (~1,9 s/campione), stream 16 campioni in ~28 s (~1,75 s/campione); corsa 2: rest 19 campioni (~1,7 s/campione), stream 16 (~1,75 s/campione). Tauri corsa 1: rest 18 campioni (~1,78 s/campione), stream 16 (~1,75 s/campione); corsa 2: rest 18 campioni (~1,78 s/campione), stream 16 (~1,75 s/campione). Totale per corsa: 33–35 campioni su 60 s, contro i ~240 attesi a `-IntervalMs 250`: un giro del ciclo paga la lettura di `Win32_Process`/`Get-Process` sull'intero albero e dei due contatori GPU (`Get-Counter` su `Dedicated Usage` e `Shared Usage`), com'è ora registrato nel commento in testa a `tree.ps1` |
| O6 | **Nel browser, una disposizione salvata con una tessera in finestra a parte non si ripristina se il browser blocca i pop-up, e la mossa 7 lo misura come differenza.** Al primo caricamento del 2026-09-10 la pagina ha ripristinato una disposizione salvata in precedenza con la Chat in finestra a parte: `dockview: failed to create popout. perhaps you need to allow pop-ups for this website` — al ricaricamento non c'è un gesto dell'utente, quindi il blocco dei pop-up di Chrome scatta — e `move 7: canonical DIFFERENT at $.popoutGroups: [...] -> undefined; raw DIFFERENT at byte 1350 (2982 vs 2803 bytes)`. La mossa 7 ufficiale è misurata sulla disposizione di default («azzera»): `canonical EQUAL`. Nei gusci la finestra a parte non si apre affatto (Q3), quindi lì il caso non si presenta |
| O7 | **Q3 è rifiutata nei due gusci per due cause diverse, lette dalla console del renderer.** Electron: la pagina vive su `file://` (origine `file://`) e dockview 8.2.0 rifiuta **prima** di chiamare `window.open` — `dockview: failed to create popout. Error: dockview: popout URL must be same-origin http(s); got: popout.html` — mentre il guscio ammette la finestra figlia (`setWindowOpenHandler` → `allow` in `electron/main.js`) e `dist/popout.html` esiste nel pacchetto: **dedotto, non misurato**, un guscio Electron che serve la build da un'origine http(s) — un server locale, o un handler di protocollo — passerebbe il controllo e aprirebbe la finestra figlia. Tauri: la pagina vive su `http://tauri.localhost/`, il controllo dell'origine passa e `window.open` rende null — `dockview: failed to create popout. perhaps you need to allow pop-ups for this website` — perché WebView2 sotto Tauri 2 non apre una finestra figlia con lo stesso contesto: **dedotto, non misurato**, Tauri può gestire la richiesta di nuova finestra, ma dockview vuole il Window che `window.open` restituisce. Il compito 8 lo porta in ADR-0029 come fatto per guscio (protocollo: un no in un guscio non è un no a `dockview`); la lettura è nelle Evidenze |

### SP-8 · Versioni degli strumenti

| Strumento | Comando | Output |
|---|---|---|
| Node, npm | `node --version; npm --version` | `v24.9.0`; `11.6.0` |
| Rust | `rustc --version; cargo --version` | `rustc 1.95.0 (59807616e 2026-04-14)`; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| i pacchetti npm installati | `npm ls --depth=0` in `gui-shell/app/`, `gui-shell/electron/`, `gui-shell/tauri/` | `app/`: `@tauri-apps/api@2.11.1`, `@vitejs/plugin-vue@6.0.8`, `dockview-core@8.2.0`, `dockview@8.2.0`, `markdown-it@15.0.1`, `three@0.185.1`, `vite@8.2.2`, `vue@3.5.42`; `electron/`: `electron-builder@26.15.3`, `electron@44.3.0`; `tauri/`: `@tauri-apps/cli@2.11.4` — tutte le versioni appuntate da P-8 |
| le crate del guscio Tauri e dell'emettitore | `grep -A1` sui nomi `tauri`, `wry`, `interprocess` nei due `Cargo.lock` | `tauri/src-tauri/Cargo.lock`: `interprocess 2.4.4`, `tauri 2.11.5`, `wry 0.55.1`; `gui-ipc/Cargo.lock`: `interprocess 2.4.3` (il lockfile di agosto, più vecchio — E6) |
| le ultime al registro npm e a crates.io, quel giorno | il comando del compito 6 del piano | npm: `dockview-core` 8.3.0 (2026-09-09, più nuova, non presa), `vue` 3.5.42 (2026-08-27, uguale), `vite` 8.3.0 (2026-09-10, più nuova, non presa), `@vitejs/plugin-vue` 6.0.8 (2026-07-14, uguale), `three` 0.186.0 (2026-09-08, più nuova, non presa), `markdown-it` 15.0.1 (2026-08-27, uguale), `electron` 44.3.0 (2026-09-08, uguale), `electron-builder` 26.15.3 (2026-06-09, uguale), `@tauri-apps/cli` 2.11.4 (2026-06-28, uguale), `@tauri-apps/api` 2.11.1 (2026-06-17, uguale); crates.io: `tauri` 2.11.5 (2026-07-01, uguale), `tauri-build` 2.6.3 (2026-06-30, non installata a confronto), `wry` 0.57.0 (2026-09-08, più nuova della 0.55.1 installata — la tira `tauri`, non presa), `interprocess` 2.4.4 (2026-09-03, uguale a quella di `tauri/`, più nuova di quella di `gui-ipc/`) |
| WebView2 | il valore `pv` della chiave del protocollo | `152.0.4191.66` |
| CPU, GPU | `Get-CimInstance Win32_Processor`, `Win32_VideoController` | `Intel(R) Core(TM) i7-14700HX`; `Intel(R) UHD Graphics`, `NVIDIA GeForce RTX 4060 Laptop GPU` |

### SP-8 · Evidenze

| Misura | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| M1, M4, M5 — Electron, corse 1 e 2 | `tree.ps1 -Exe "C:\Users\zagor\AppData\Local\Programs\sp8-electron\sp8-electron.exe" -Seconds 60 -EmitterAt 32 -Emitter "C:\Users\zagor\Desktop\harness\spikes\gui-ipc\target\release\core.exe" -Csv "C:\Users\zagor\sp8-measure\electron-N.csv"` (N=1,2; il CSV fuori dal repository) | **corsa 1:** `shell started: pid 6572 (C:\Users\zagor\AppData\Local\Programs\sp8-electron\sp8-electron.exe)` · `emitter started at 33,3 s` · `rest: samples 17 \| rss_mb mean 501,7 max 558 \| cpu_pct mean 77,1 max 129,4 \| vram_mb max 0 \| vram_shared_mb max 126` · `stream: samples 16 \| rss_mb mean 575 max 585,4 \| cpu_pct mean 116,9 max 196,8 \| vram_mb max 0 \| vram_shared_mb max 119,4` · `last title: SP-8 \| fps=226 min=140 api=WebGPU:intel/gen-12lp scene=215x318 \| msgs=2000 lost=0 holes=0 p2max=43.9ms p2mean=3.13ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78` — **corsa 2:** `shell started: pid 9964 (C:\Users\zagor\AppData\Local\Programs\sp8-electron\sp8-electron.exe)` · `emitter started at 33,4 s` · `rest: samples 19 \| rss_mb mean 439,5 max 470,3 \| cpu_pct mean 22,5 max 96 \| vram_mb max 0 \| vram_shared_mb max 112,6` · `stream: samples 16 \| rss_mb mean 526,7 max 541,9 \| cpu_pct mean 127,7 max 225,3 \| vram_mb max 0 \| vram_shared_mb max 105,6` · `last title: SP-8 \| fps=200 min=61 api=WebGPU:intel/gen-12lp scene=99x318 \| msgs=2000 lost=0 holes=0 p2max=41.3ms p2mean=11.91ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78` | vram_mb max è 0 in entrambe le corse: atteso su questa GPU integrata (richiamo E10), non un errore; vram_shared_mb sopra zero in entrambe. `msgs=2000 lost=0 holes=0` in entrambe: nessuna divergenza su P1. Le altre letture sono in O1–O3 |
| M1, M4, M5 — Tauri, corse 1 e 2 | lo stesso con `sp8-tauri.exe` | **corsa 1:** `shell started: pid 42784 (C:\Users\zagor\AppData\Local\sp8-tauri\sp8-tauri.exe)` · `emitter started at 32,8 s` · `rest: samples 18 \| rss_mb mean 485,6 max 519,3 \| cpu_pct mean 73,3 max 120,5 \| vram_mb max 0 \| vram_shared_mb max 85,9` · `stream: samples 16 \| rss_mb mean 570,8 max 588,3 \| cpu_pct mean 115,7 max 209,9 \| vram_mb max 0 \| vram_shared_mb max 102,7` · `last title: SP-8 \| fps=199 min=58 api=WebGPU:intel/gen-12lp scene=100x349 \| msgs=2000 lost=0 holes=0 p2max=33.6ms p2mean=6.03ms \| src=tauri dnd=auto \| ua=Chrome/152.0.0.0,Edg/152.0.0.0` — **corsa 2:** `shell started: pid 41420 (C:\Users\zagor\AppData\Local\sp8-tauri\sp8-tauri.exe)` · `emitter started at 32,7 s` · `rest: samples 18 \| rss_mb mean 486,3 max 517,9 \| cpu_pct mean 67,5 max 88,9 \| vram_mb max 0 \| vram_shared_mb max 85,9` · `stream: samples 16 \| rss_mb mean 571,1 max 590,4 \| cpu_pct mean 112,5 max 206,4 \| vram_mb max 0 \| vram_shared_mb max 103` · `last title: SP-8 \| fps=200 min=57 api=WebGPU:intel/gen-12lp scene=100x349 \| msgs=2000 lost=0 holes=0 p2max=26.9ms p2mean=5.96ms \| src=tauri dnd=auto \| ua=Chrome/152.0.0.0,Edg/152.0.0.0` | `procs` costante a 7 in entrambe le corse (letto dai CSV): WebView2 dentro l'albero, la trappola P-9 non scatta. `ua=` porta due token, `Chrome/152.0.0.0,Edg/152.0.0.0`: WebView2 si annuncia anche come `Edg/`, non solo `Chrome/` — diverso da Electron, registrato qui perché non c'è una riga dedicata |
| M2 | `size.ps1 -Path` sulle due cartelle installate e sui due installatori | `C:\Users\zagor\AppData\Local\Programs\sp8-electron: 74 files, 386830264 bytes, 368,9 MB` · `C:\Users\zagor\AppData\Local\sp8-tauri: 2 files, 9034590 bytes, 8,6 MB` · `C:\Users\zagor\Desktop\harness\spikes\gui-shell\electron\out\sp8-electron Setup 0.0.0.exe: 111565262 bytes, 106,4 MB` · `spikes\gui-shell\tauri\src-tauri\target\release\bundle\nsis\sp8-tauri_0.0.0_x64-setup.exe: 2162513 bytes, 2,1 MB` | nessuna: M2 «discrimina» come atteso da ADR-0029 (368,9 MB contro 8,6 MB installati). Osservato in più, non richiesto dal protocollo: l'installatore Electron lascia anche una cartella `C:\Users\zagor\AppData\Local\sp8-electron-updater\` con una copia dell'installatore (111565262 byte) — cache dell'auto-updater di `electron-builder`, fuori dalle due cartelle misurate da M2 |
| M3 a riposo | il titolo dell'ultima riga `rest` del CSV, letto con `Import-Csv` (il comando nel compito 6 del piano) | Electron corsa 1: `SP-8 \| fps=224 min=16 api=WebGPU:intel/gen-12lp scene=446x318 \| msgs=0 lost=0 holes=0 p2max=0.0ms p2mean=0.00ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78`; corsa 2: `SP-8 \| fps=213 min=7 api=WebGPU:intel/gen-12lp scene=99x318 \| msgs=0 lost=0 holes=0 p2max=0.0ms p2mean=0.00ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78`. Tauri corsa 1: `SP-8 \| fps=238 min=214 api=WebGPU:intel/gen-12lp scene=100x349 \| msgs=0 lost=0 holes=0 p2max=0.0ms p2mean=0.00ms \| src=tauri dnd=auto \| ua=Chrome/152.0.0.0,Edg/152.0.0.0`; corsa 2: `SP-8 \| fps=238 min=207 api=WebGPU:intel/gen-12lp scene=100x349 \| msgs=0 lost=0 holes=0 p2max=0.0ms p2mean=0.00ms \| src=tauri dnd=auto \| ua=Chrome/152.0.0.0,Edg/152.0.0.0` | la taglia della tessera non è stabile per Electron — vedi O3 |
| Q2, WebKitGTK | le tre pagine lette il 2026-09-10: l'*Implementation Status* del wiki di `gpuweb/gpuweb`, le news di `webkitgtk.org`, *Webview Versions* di Tauri | gpuweb wiki (https://github.com/gpuweb/gpuweb/wiki/Implementation-Status): la tabella per Safari/WebKit ha solo tre colonne, "macOS", "iOS/iPadOS", "visionOS" — nessuna colonna o riga per Linux/GTK; la parola "GTK" non compare nella pagina. webkitgtk.org/news.html (https://webkitgtk.org/news.html): ultima stabile «WebKitGTK 2.52.6 released!» (19 agosto 2026; i minori pari sono le stabili, i dispari le di sviluppo nello schema GNOME — «WebKitGTK 2.53.92 released!» del 2 settembre 2026 è di sviluppo); la parola "WebGPU" non compare in alcun titolo di release. Tauri Webview Versions (https://v2.tauri.app/reference/webview-versions/): nessuna versione WebKitGTK unica per Linux — «The diverse nature of the Linux ecosystem means it is very hard to compile accurate information about WebKitGTK on the various distros» — con un esempio di riga, Ubuntu 20.04 → webkit2gtk 2.28 → WebKit 610.1.1 | nessuna pagina ha richiesto un secondo tentativo; nessuna delle tre nomina WebGPU su WebKitGTK, quindi lo stato letto è «non tracciato/assente», non «dietro flag» |
| M4 con la chat nascosta, M5 a pagina intera — Electron | `tree.ps1 -AttachPid 26532 -Seconds 25 -EmitterAt 3 -Emitter "C:\Users\zagor\Desktop\harness\spikes\gui-ipc\target\release\core.exe" -Csv "C:\Users\zagor\sp8-measure\electron-hidden.csv"` — l'app installata lanciata a mano dal proprietario, «azzera», la Scena 3D trascinata come seconda linguetta del gruppo della Chat e attiva, la finestra riportata in primo piano dal coordinatore prima della corsa; poi ⤢ su Scena e `tree.ps1 -AttachPid 26532 -Seconds 12 -Csv "C:\Users\zagor\sp8-measure\electron-fullpage.csv"` | chat nascosta: `attached to pid 26532` · `emitter started at 4,0 s` · `rest: samples 1 \| rss_mb mean 493,6 max 493,6 \| cpu_pct mean 0 max 0 \| vram_mb max 0 \| vram_shared_mb max 105,2` · `stream: samples 12 \| rss_mb mean 485,9 max 497,3 \| cpu_pct mean 111,6 max 146,8 \| vram_mb max 0 \| vram_shared_mb max 109,4` · `last title: SP-8 \| fps=216 min=15 api=WebGPU:intel/gen-12lp scene=138x318 \| msgs=2000 lost=0 holes=0 p2max=20.3ms p2mean=0.19ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78` — pagina intera: `rest: samples 6 \| rss_mb mean 514,7 max 516,1 \| cpu_pct mean 76,8 max 111,1 \| vram_mb max 0 \| vram_shared_mb max 167` · `last title: SP-8 \| fps=182 min=1 api=WebGPU:intel/gen-12lp scene=1370x743 \| msgs=2000 lost=0 holes=0 p2max=20.3ms p2mean=0.19ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78` | il campione di riposo è uno solo (emettitore a 3 s, circa 2 s per campione, E11): la riga `rest` della corsa con la chat nascosta non si legge, si legge `stream`; il relay di SP-7 e il server di Vite erano ancora accesi durante queste due corse (spenti prima di quelle di Tauri e di O1): un carico fuori dall'albero misurato, dichiarato |
| M4 con la chat nascosta, M5 a pagina intera — Tauri | lo stesso con `-AttachPid 24480`, `tauri-hidden.csv` e `tauri-fullpage.csv`; relay e Vite spenti | chat nascosta: `attached to pid 24480` · `emitter started at 3,3 s` · `rest: samples 1 \| rss_mb mean 558,2 max 558,2 \| cpu_pct mean 0 max 0 \| vram_mb max 0 \| vram_shared_mb max 95,5` · `stream: samples 13 \| rss_mb mean 558,2 max 572 \| cpu_pct mean 97,3 max 138,1 \| vram_mb max 0 \| vram_shared_mb max 95,5` · `last title: SP-8 \| fps=228 min=199 api=WebGPU:intel/gen-12lp scene=250x349 \| msgs=2000 lost=0 holes=0 p2max=12.9ms p2mean=0.50ms \| src=tauri dnd=pointer \| ua=Chrome/152.0.0.0,Edg/152.0.0.0` — pagina intera: `rest: samples 6 \| rss_mb mean 606 max 608,7 \| cpu_pct mean 56 max 81,3 \| vram_mb max 0 \| vram_shared_mb max 168,8` · `last title: SP-8 \| fps=172 min=140 api=WebGPU:intel/gen-12lp scene=1384x805 \| msgs=2000 lost=0 holes=0 p2max=12.9ms p2mean=0.50ms \| src=tauri dnd=pointer \| ua=Chrome/152.0.0.0,Edg/152.0.0.0` | `dnd=pointer` nel titolo perché il proprietario lo aveva scelto per Q4 e la scelta vive nel `localStorage` della webview; non tocca M4 e M5. Anche qui un solo campione di riposo |
| O1 — la finestra coperta contro la finestra in primo piano, Electron | `tree.ps1 -Exe "C:\Users\zagor\AppData\Local\Programs\sp8-electron\sp8-electron.exe" -Seconds 30 -EmitterAt 8 -Emitter "C:\Users\zagor\Desktop\harness\spikes\gui-ipc\target\release\core.exe" -Csv "C:\Users\zagor\sp8-measure\electron-covered.csv"`, con un secondo PowerShell che dopo 5 s apre `notepad` massimizzato, lo attiva e lo tiene davanti per 33 s; poi la stessa corsa con `electron-front.csv` e un secondo PowerShell che riporta davanti la finestra del guscio ogni 5 s | coperta: `emitter started at 8,8 s` · `rest: samples 4 \| rss_mb mean 470,3 max 482,6 \| cpu_pct mean 62 max 128,5 \| vram_mb max 0 \| vram_shared_mb max 114,6` · `stream: samples 13 \| rss_mb mean 522,8 max 548 \| cpu_pct mean 108,8 max 215,8 \| vram_mb max 0 \| vram_shared_mb max 117,9` · `last title: SP-8 \| fps=175 min=26 api=WebGPU:intel/gen-12lp scene=99x318 \| msgs=2000 lost=0 holes=0 p2max=43.9ms p2mean=11.85ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78`; i campioni di `cpu_pct` in ordine, col titolo: `0 128,5 97,7 21,6` a riposo con `fps=220 226 228 228`, poi `1,8 1,7` con il titolo fermo a `fps=228 min=220`, poi `79,2 167,6 215,8 178,7 181,9 115,8 93,3 90,2 91,7 104,5 92,8` con `fps=182 … 175 min=26` — in primo piano: `emitter started at 8,6 s` · `rest: samples 4 \| rss_mb mean 430,3 max 447,2 \| cpu_pct mean 77 max 119,6 \| vram_mb max 0 \| vram_shared_mb max 84,5` · `stream: samples 13 \| rss_mb mean 514,9 max 535,2 \| cpu_pct mean 134,4 max 215,9 \| vram_mb max 0 \| vram_shared_mb max 105,5` · `last title: SP-8 \| fps=194 min=60 api=WebGPU:intel/gen-12lp scene=99x318 \| msgs=2000 lost=0 holes=0 p2max=46.8ms p2mean=12.36ms \| src=electron dnd=auto \| ua=Chrome/152.0.7977.78`; i campioni: `0 119,6 97,3 91,1 107,1 101 215,9 195 191,7 141,7 200,1 134 87,5 88,5 93 107 84,6` | la CPU a ~0 % col titolo congelato si riproduce **a riposo** sotto copertura; sotto flusso la corsa coperta risale agli stessi picchi di quella in primo piano: la forma della terza corsa del revisore (tutta a ~0 %) non si è ripetuta, e perché il renderer riparta all'arrivo dei messaggi pur restando coperto non è misurato — richiamo in O1 |
| Q3 — la causa del rifiuto, per guscio | le due app rilanciate con la porta di debug — `sp8-electron.exe --remote-debugging-port=9222`; `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223` e `sp8-tauri.exe` — e uno script di sola lettura sul Chrome DevTools Protocol (`node cdp-popout.mjs 9222`, poi `9223`, nello scratchpad) che abilita `Runtime` e `Log`, clicca la presa grande di «Costi» e preme `P` (il comando ↗), poi stampa la console e la barra; le corse di M4, M5 e O1 sono state fatte **senza** porta di debug | Electron: `page: file:///C:/Users/zagor/AppData/Local/Programs/sp8-electron/resources/app.asar/dist/index.html \| origin=file://` · `[console.error] dockview: failed to create popout. Error: dockview: popout URL must be same-origin http(s); got: popout.html` · `[console.log] move 4 / Q3: popout REFUSED for costi` — Tauri: `page: http://tauri.localhost/ \| origin=http://tauri.localhost` · `[console.error] dockview: failed to create popout. perhaps you need to allow pop-ups for this website` · `[console.log] move 4 / Q3: popout REFUSED for costi`; prima, col mouse del proprietario e senza porta di debug: `move 4 / Q3: popout REFUSED for costi` e `for scene` (Electron), `for hand` (Tauri) | nessuna: il rifiuto è lo stesso con e senza porta di debug; le cause sono in O7 |
| Le otto mosse — la console del browser | la console di Chrome del proprietario incollata in chat, salvata nello scratchpad della sessione e contata con `grep -c 'pointerdown'`, `grep -c 'pointerdown.*on div.bigtab'`, `grep -c 'pointerdown.*on span.bigtab-title'`, `grep -c 'pointerdown.*on div.dv-sash'` | `137`, `10`, `5`, `5`; le righe `move 2`, `move 3`, `move 4 / Q3`, `move 6`, `move 7` sono quelle della tabella delle mosse; `move 8: relay connected` all'apertura (il proxy di Vite passa l'SSE: P-3 misurato) | lo scratchpad muore con la sessione: il conteggio resta qui, la console intera no |

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

