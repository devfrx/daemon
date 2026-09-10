# SP-8 — Il guscio della GUI, e l'accettazione di `dockview`: il protocollo

Criteri scritti **prima** della misura, come vogliono la §2 del
[disegno del 2](../../docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) e la §4 della
[stella polare della GUI](../../docs/superpowers/specs/2026-09-07-direzione-gui-design.md), e **congelati al
primo commit di codice di questo spike**. Un criterio soddisfatto con un accorgimento si registra come
**parziale**, non come passato — la regola di [`PROTOCOLLO.md`](../PROTOCOLLO.md), che vale anche qui, come
valse per [SP-7](../gesti/PROTOCOLLO.md).

**Che cosa misura, e che cosa no.** Le cinque misure **M1–M5** e le quattro righe qualitative **Q1–Q4** con
cui [ADR-0029](../../docs/adr/0029-guscio-della-gui.md) dice di chiudersi — lo stesso frontend minimo sui
due gusci, Electron e Tauri — e le **otto mosse** con cui il proprietario giudica se i pannelli agganciabili
di `dockview` «danno il Jarvis» (decisione 6 della stella polare). ⛔ **Non misura:** la metà Linux di M3 e
M5 (decisione C del disegno del 2: si sostituisce con la lettura di Q2 alle fonti e con l'innesco scritto
nell'ADR); la forma di Compatta (del 10); colori e forme (del design system); i moduli veri; il filo vero
— il flusso viene dall'emettitore di `spikes/gui-ipc/` com'è, righe JSON e non `bincode`, col carico
`"lorem ipsum dolor sit amet"` su ogni riga, quindi **il markdown reso è testo senza blocchi di codice**;
della mano, il worker sotto il core, il porto `process`, il timbro e il riconoscimento di un gesto discreto
(SP-7, F3: il vocabolario è della capacità). Q1 — chi decodifica `bincode` — resta **qualitativa**: si legge
dall'architettura dei due gusci, non si misura, perché questo spike non decodifica `bincode`.

**La macchina.** I criteri valgono **su questa macchina**, e l'esito la dichiara: la CPU la dice
`powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name"`, la GPU è quella di ADR-0002, il
WebView2 lo dice il valore `pv` della chiave `HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}`,
il Chromium di Electron lo dice il titolo della finestra (`ua=`). Le versioni degli attrezzi si rileggono al
registro il giorno dello spike, col comando di `riferimenti.md`, e vanno nell'esito.

**Il frontend, uno solo.** La Home finta di `app/` — Vite, Vue 3, `dockview-core` — con il nucleo bloccato
al centro, la striscia bloccata in basso, le tessere intorno, la presa grande, e **dentro due tessere** la
chat che rende markdown token per token e la scena `three`; la stessa build caricata dai due gusci. Il
guscio legge la pipe dell'emettitore e la passa alla webview: **il salto webview di M4 è vero**. La pagina
scrive nel proprio **titolo**, una volta al secondo, ciò che misura da sé — fps medi e minimi degli ultimi
trenta secondi, l'API grafica ottenuta, i messaggi ricevuti, i buchi nel progressivo (P1), il ritardo
massimo e medio emissione → ricezione (P2), la sorgente, la strategia di trascinamento, lo user agent —
e lo script di misura lo legge da fuori.

## Le misure — M1–M5

| # | Misura | Come | Criterio, o «si riporta» | La trappola, e come si neutralizza |
|---|---|---|---|---|
| **M1** | RAM **a riposo** e **sotto streaming** | `measure/tree.ps1`: la somma di `WorkingSet64` sull'**intero albero di processi** del guscio, ogni 250 ms; il riposo sono i secondi prima che l'emettitore parta, il flusso i dieci secondi dei 2000 messaggi; si riportano **media e picco** di ciascuna finestra | **si riporta**, per confronto fra i due gusci: nessuna soglia | WebView2 tira su processi **fuori** dal PID di Tauri: contare il solo processo principale falsa il confronto. L'albero si ricostruisce da `Win32_Process.ParentProcessId` a ogni campione |
| **M2** | pacchetto **installato** | si costruisce l'installatore NSIS per utente di ciascuno, lo si installa in silenzio (`/S`), e `measure/size.ps1` somma i byte della cartella installata; si riporta anche la taglia dell'installatore | **si riporta** | la cartella la sceglie l'installatore: si scrive nell'esito quale, per ciascuno |
| **M3** | fps del viewer 3D, e l'**API grafica ottenuta davvero** | la scena `three` (`WebGPURenderer` di `three/webgpu`, che ricade su WebGL2 da solo) conta i fotogrammi per secondo; l'API la dichiara `renderer.backend` — `isWebGPUBackend` o `isWebGLBackend` — dopo `init()`, con l'adattatore o il renderer che il browser espone; medi e minimi sugli ultimi **30 s**, con la tessera della scena alla taglia della Home | **si riporta**, su Windows; ⛔ **Linux non ora** (decisione C) | su Windows i due gusci sono entrambi Chromium: l'API sarà probabilmente la stessa, e va **scritto** lo stesso, perché è la misura e non l'attesa |
| **M4** | **P3 con rendering vero**: CPU del guscio sotto i 2000 messaggi col markdown reso davvero, con `dockview` acceso e la chat in una tessera | `tree.ps1`: la CPU dell'**albero** come percentuale di **un** core — la differenza della somma di `TotalProcessorTime` sull'intervallo — ogni 250 ms nei dieci secondi del flusso; si riportano **picco e media** | **picco < 25 % di un core**, che è **P3** di [`GUI-REQUISITI.md`](../GUI-REQUISITI.md); la CPU con la chat **nascosta** dietro un'altra tessera si riporta **senza soglia** | misurato con JSON e non `bincode`: limite dichiarato. Il rendering è per fotogramma (`requestAnimationFrame`), come farà la GUI vera: un rendering per token sarebbe una misura di un difetto, non del guscio |
| **M5** | VRAM **a riposo** e **sotto carico 3D** | `tree.ps1`: la somma di `\GPU Process Memory(*)\Dedicated Usage` sui PID dell'albero, una volta al secondo; a riposo = la Home coi pannelli alla loro taglia; sotto carico = la scena **a pagina intera** (mossa 3) | **si riporta** | `nvidia-smi` in WDDM spesso non dà la VRAM per processo: si usano i contatori di Windows, che esistono su questa macchina (verificato il 2026-09-09) |

## Le righe qualitative — Q1–Q4, per guscio

| # | Domanda | Come si risponde |
|---|---|---|
| **Q1** | chi decodifica `bincode` lato GUI | letto dall'architettura del guscio, non misurato: Electron → il processo principale Node (`bincode-ts`, M-11); Tauri → il guscio Rust col decodificatore del kernel |
| **Q2** | le webview in uso | WebView2: la versione dal registro; Electron: il Chromium impacchettato, dal titolo (`ua=`); lo stato di WebGPU su **WebKitGTK** letto alle fonti primarie **quel giorno**, con URL e data nell'esito — è la lettura che sostituisce la misura Linux |
| **Q3** | la finestra staccata si apre **dentro il guscio** | la mossa 4, **dal comando** (`addPopoutGroup`, che risponde `true` o `false` e la pagina lo scrive nel registro della barra): la finestra nuova si apre, e chiusa, la tessera torna. Un no in un guscio è un fatto per ADR-0029, non un no a `dockview` |
| **Q4** | nella webview basta `dndStrategy: 'auto'` o serve `'pointer'` | la mossa 2 provata **col mouse** in ciascun guscio sotto `'auto'` (il menu `dnd:` della barra); se non trascina, sotto `'pointer'`. La doc di `dockview` consiglia `'pointer'` dove il drag HTML5 è inaffidabile e nomina le webview incorporate |

## Le otto mosse — il giudizio del proprietario, provandole

Ricopiate dalla §4 della stella polare, che ne è la casa del **perché** — la mossa 7 aggiunge solo come la pagina scrive l'esito del confronto, precisato il 2026-09-10 prima del congelamento (errata E1 del piano); qui sono ciò che si congela.
Il proprietario dice **sì o no a ogni mossa con le sue parole**, e le parole vanno nell'esito. Le mosse
1–8 si provano **nel browser** (`npm run dev`, `http://localhost:5173`); le mosse 2 e 4 si riprovano nei
due gusci per Q4 e Q3. La mossa 8 vuole il relay di SP-7 in ascolto e la telecamera.

| # | La mossa | Passa se |
|---|---|---|
| 1 | il nucleo al centro e la striscia in basso non si spostano; le tessere intorno sì | trascinando qualsiasi cosa, i due restano dove sono |
| 2 | una tessera si sgancia, galleggia sopra le altre, si riaggancia | si sgancia, si muove, torna, senza perdersi |
| 3 | una tessera a pagina intera e ritorno | un gesto per andare, uno per tornare, la disposizione sotto resta |
| 4 | una tessera in un'altra finestra | la finestra si apre **dentro il guscio** (riga Q3); chiusa, la tessera torna. Si fa **dal comando**, non trascinando fra due finestre (decisione 34) |
| 5 | la presa grande, disegnata da noi, si afferra col mouse e col tocco | si afferra senza mirare a una linguetta sottile |
| 6 | spostare una tessera con la tastiera | una scorciatoia nostra sposta la tessera attiva nelle quattro direzioni |
| 7 | salva, ricarica, ritrova | i due JSON, prima e dopo, sono **uguali**: una misura, non un giudizio (decisione 32) — la pagina li confronta e scrive «EQUAL» o «DIFFERENT al byte n» |
| 8 | la pinza afferra la presa grande e la tessera segue la mano | la tessera segue la pinza, si sgancia e si riaggancia: un sì o un no **tecnico**; come si sente lo dice il proprietario, con le sue parole, **senza soglia** |

⚠️ **Richiamo del 2026-09-10 — la mossa 7, misurata nel browser durante il compito 2 (errata E4 del piano).** «I due JSON uguali» si legge sul JSON **canonico** — le chiavi degli oggetti ordinate — perché `dockview` 8.2.0 emette `panels` in un ordine diverso dopo `fromJSON` (misurato a 1920×1080: stesso contenuto, stessa lunghezza, primo byte diverso dentro `panels`), e l'ordine delle chiavi non è la disposizione; la pagina scrive **entrambi** i confronti — `canonical EQUAL` o `canonical DIFFERENT at <percorso>: <prima> -> <dopo>`, e `raw EQUAL` o `raw DIFFERENT at byte n` — così il grezzo resta leggibile. Due fatti misurati che il giudizio del proprietario tiene in mano: un gruppo **galleggiante** cresce di **2 px** in larghezza e altezza a ogni salva-e-ricarica (`dv-resize-container`, bordo di 1 px in `content-box`: 462×322 → 464×324 → 466×326), quindi con una tessera staccata il canonico è `DIFFERENT` per costruzione di `dockview`, non per un accorgimento nostro; e su un viewport stretto (1024 px) il ripristino impone i minimi di `dockview` — 100 px per pannello, quattro pannelli sopra il Nucleo — e ridistribuisce le colonne (341 → 400), mentre a 1920 px la griglia torna identica: le prove si fanno alla taglia dello schermo del proprietario, e l'esito la dichiara. È una modifica al metro dopo il congelamento, e sta qui per questo.

**La mossa 8, come si fa.** Il worker `s2_worker.py` e il relay di `spikes/gesti/` **com'è**, righe JSON in
SSE; la Home disegna la mano dai 21 punti — come `page.html` di SP-7 — e traduce la pinza in eventi del
puntatore: `pointerdown` alla chiusura, `pointermove`, `pointerup` all'apertura, con `dockview` in
`'pointer'`. La pinza è il segnaposto e un pugno vale come pinza (O7 di SP-7). Il ritardo che si sente è
quello misurato in SP-7 — cattura → disegno, mediana 114 ms — della pipeline e non di `dockview`. ⚠️ La
pagina di Vite raggiunge il relay da un proxy sulla propria origine (`/stream`): il relay non cambia.

**Come si decide.** Tutte e otto passano: `dockview` resta. Una mossa che passa solo con un trucco è
«parziale». Un no nell'insieme: si rifà la stessa Home con `interactjs`, stesse mosse, stesso protocollo,
così le due si confrontano davvero, **prima** della SPA. Un no sulla sola mossa 4 in un guscio non è un no
a `dockview`: è un fatto per ADR-0029, come Q1. Un no tecnico sulla mossa 8 — `dockview` non segue il
puntatore finto — si sa prima della SPA e su quel punto la tela libera vince; se basti a far cadere
`dockview` lo dice il proprietario, sull'insieme. **Il guscio lo decide il proprietario** con M1–M5 e
Q1–Q4 in mano, in forma A/B: l'ADR ne è il verbale.

## Registrazione

L'esito va in [`RISULTATI.md`](../RISULTATI.md), sezione **SP-8**, nella forma di SP-7: esito per misura e
per mossa, le parole del proprietario, osservazioni che non sono criteri, versioni degli strumenti,
evidenze con comando e output osservato, e le **divergenze** dall'attesa. I numeri di M1–M5 e le righe
Q1–Q4 entrano in **ADR-0029**, che è la loro casa unica. I campioni grezzi (CSV di `tree.ps1`) restano
fuori dal repository.

## Congelamento

Congelato al primo commit di codice di questo spike. Una modifica dopo quel commit è una modifica al
**metro**, e va detta qui con un richiamo datato — come il richiamo del 2026-09-04 di SP-7.
