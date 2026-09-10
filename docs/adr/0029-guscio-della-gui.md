# ADR-0029: Guscio della GUI — Tauri o Electron

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

> ⚠️ **Questa decisione è aperta.** `scripts/check-docs.sh` la segnala a ogni audit,
> come previsto. Non è una svista: è l'unica cosa dello stack che le misure fatte non
> sostengono, e forzarla su un argomento sarebbe contro il metodo di questo repository.
> ✅ **RICHIAMO DEL 2026-09-10:** chiusa con **SP-8** — la *Decision* qui sotto; l'avviso resta com'era, perché è la storia di questa decisione.

## Context

[ADR-0027](0027-stack-della-gui.md) decide che la GUI è un'**interfaccia web**, per G7.
Resta da decidere il **guscio** che la ospita e che parla col core.

Il core è Rust ([ADR-0026](0026-linguaggio-del-core.md)), i worker ML sono Python
([ADR-0028](0028-ecosistema-dei-worker-ml.md)). Questa scelta determina se ci sarà un
**terzo runtime**.

### Fatti verificati il 2026-08-06

| Fatto | Fonte |
|---|---|
| Tauri **non impacchetta** una webview: usa quella di sistema tramite WRY. Su Windows **WebView2** (Chromium), su Linux **WebKitGTK** (`webkit2gtk 4.1`) | [Tauri — Webview Versions](https://v2.tauri.app/reference/webview-versions/) |
| La documentazione di Tauri dichiara essa stessa che «la natura diversificata dell'ecosistema Linux» rende difficile avere informazioni accurate su WebKitGTK nelle varie distribuzioni | idem |
| Electron **impacchetta** Chromium e Node.js con l'applicazione | architettura del progetto |
| WebGPU su Chromium/Windows x86-64: **rilasciato** (Chrome 113+). Su Linux: **dietro flag**. Per WebKitGTK **non risulta rilasciato** | [WebGPU Implementation Status](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status) |
| Tauri è agnostico rispetto al framework; vincolo: **SSG, SPA o MPA — niente SSR** | [Tauri — Frontend](https://v2.tauri.app/start/frontend/) |
| Versioni | `tauri` 2.11.5 · `wry` 0.56.0 · `electron` 43.3.0 · `electron-builder` 26.15.3 |

### Il confronto

| | **Tauri** | **Electron** |
|---|---|---|
| runtime aggiunti | nessuno: il guscio è **Rust**, come il core | **Node.js**, il cui unico compito è ospitare la finestra |
| motore di rendering | quello **del sistema**: due diversi fra Windows e Linux | **uno solo**, impacchettato, identico ovunque |
| controllo su versione e flag del motore | **nessuno**: è quello dell'utente | **totale**: il browser è tuo |
| WebGPU per G6 | Windows sì · **Linux incerto** | Windows sì · Linux dietro un flag **che passi tu** |
| dimensione del pacchetto | piccola | + Chromium |
| superficie di sicurezza | guscio Rust con capacità esplicite | main process Node con accesso pieno all'OS |
| maturità desktop (auto-update, tray, crash report) | buona | maggiore |
| familiarità per il proprietario | **nulla**, e si somma a Rust da imparare | **alta**, isola il nuovo nel solo core |

### Perché la bilancia pende, e da che parte

Il profilo di **questa** GUI non è una finestra di impostazioni: viewer 3D interattivo
(G6), artifacts con anteprima viva (G7), streaming token per token (G4), diff (G5),
grafici (G8). È un frontend pesante, ed è esattamente dove «il browser è tuo» smette di
essere un dettaglio.

A questo si somma una condizione del progetto, non della tecnologia: **il proprietario
sta già imparando Rust per il core.** Prendere anche Tauri concentrerebbe due incognite
nello stesso momento; Electron isolerebbe il nuovo nel solo core.

L'asimmetria dei costi:

| | Natura del costo | Quando si paga |
|---|---|---|
| Electron | **packaging** — terzo runtime, browser impacchettato, superficie di sicurezza | all'installazione e agli aggiornamenti |
| Tauri | **capacità** — API grafiche non garantite, due motori da testare, comunità minore | ogni giorno, su un frontend esigente |

Un costo di capacità su un frontend pesante si paga più spesso di un costo di packaging.

## Decision

✅ **DECISA il 2026-09-10 dal proprietario, con SP-8 in mano — Electron.** I criteri, congelati **prima** della
misura, in [`spikes/gui-shell/PROTOCOLLO.md`](../../spikes/gui-shell/PROTOCOLLO.md); l'esito intero, con le evidenze e
le osservazioni, in [`spikes/RISULTATI.md`](../../spikes/RISULTATI.md), sezione SP-8, che dà la data. La macchina è quella
dell'esito — il portatile con `Intel(R) Core(TM) i7-14700HX`, `Intel(R) UHD Graphics` e `NVIDIA GeForce RTX 4060 Laptop GPU`,
la webview sull'**integrata** (`api=WebGPU:intel/gen-12lp`) — e **non** quella di ADR-0002 (E9 del piano della parte 1): i
numeri assoluti di M3 e M5 non sono quelli della RTX 5080; il confronto fra i gusci regge, perché macchina e adattatore sono
gli stessi per entrambi. Le sue parole: «A».

| # | Misura, o riga | Electron | Tauri |
|---|---|---|---|
| M1 | RAM a riposo e sotto flusso — media / picco in MB, due corse | riposo 501,7 / 558; 439,5 / 470,3 — flusso 575 / 585,4; 526,7 / 541,9 | riposo 485,6 / 519,3; 486,3 / 517,9 — flusso 570,8 / 588,3; 571,1 / 590,4 |
| M2 | cartella installata e installatore, MB | 368,9 (74 file); installatore 106,4 | 8,6 (2 file); installatore 2,1 |
| M3 | fps medi / minimi sugli ultimi 30 s a riposo, due corse, e l'**API ottenuta** — **Windows** | 224 / 16; 213 / 7, `api=WebGPU:intel/gen-12lp` — il minimo è lo scatto d'avvio caduto nella finestra (O3 dell'esito) | 238 / 214; 238 / 207, `api=WebGPU:intel/gen-12lp` |
| M4 | P3 con rendering vero — picco / media in % di un core, due corse; la soglia è il picco **< 25 %** | 196,8 / 116,9; 225,3 / 127,7 — **non passa**; con la chat nascosta 146,8 / 111,6 | 209,9 / 115,7; 206,4 / 112,5 — **non passa**; con la chat nascosta 138,1 / 97,3 |
| M5 | VRAM in MB, picco, dedicata / condivisa — a riposo, due corse; con la scena a pagina intera | 0 / 126; 0 / 112,6 — a pagina intera 0 / 167 | 0 / 85,9; 0 / 85,9 — a pagina intera 0 / 168,8 |
| Q1 | chi decodifica `bincode` lato GUI | il processo principale Node, `bincode-ts` (M-11 di ADR-0037) | il guscio Rust, col decodificatore del kernel — una terza compilazione delle crate (decisione 57 della stella polare) |
| Q2 | le webview | il Chromium impacchettato, `Chrome/152.0.7977.78` | WebView2 `152.0.4191.66`; **WebKitGTK**: WebGPU **non tracciato** alle fonti lette il 2026-09-10 — assente dalla tabella Safari/WebKit di gpuweb, mai nominato nelle news di WebKitGTK, nessuna versione unica per Linux nella pagina di Tauri — le fonti in `riferimenti.md`, sezione SP-8 |
| Q3 | la finestra staccata si apre **dentro** il guscio, dal comando | **no**: `dockview` 8.2.0 rifiuta prima di `window.open` perché la pagina vive su `file://` («popout URL must be same-origin http(s)») e il guscio la ammetterebbe — servire la build da un'origine http(s) è il rimedio, **dedotto**, da misurare nella parte 2 (E14 del piano; O7 dell'esito) | **no**: la pagina vive su `http://tauri.localhost/`, il controllo dell'origine passa e `window.open` rende null in WebView2 (O7 dell'esito) |
| Q4 | `dndStrategy` che basta nella webview | `'auto'` | `'pointer'` — con `'auto'` il trascinamento non parte |

**P3 non passa su nessuno dei due, e non discrimina fra i gusci:** la scena `three` senza tetto di fps sta vicino o sopra la
soglia già **a riposo**, prima di qualunque messaggio (O2 dell'esito), e con la chat nascosta il picco resta sopra la soglia su
entrambi. È un fatto per la **SPA** — un tetto di fps, o il rendering a richiesta — che la parte 2 del piano misura. E M5 su
questa macchina è la memoria **condivisa** dell'integrata: un proxy della VRAM dedicata della macchina di ADR-0002 (E10 del
piano), da rimisurare là.

**L'innesco Linux, scritto.** La metà Linux di M3 e M5 **non è misurata** (decisione C del disegno del 2): al primo
Linux vero si rimisurano M3 e M5 su quel sistema, con lo stesso protocollo; se M3 mostra la stessa API grafica su
entrambe le piattaforme con Tauri, questa decisione si **riapre con un ADR nuovo** che la superi (`Superseded by`),
non con una conversazione.

**`dockview`:** resta — le otto mosse passano, otto su otto, le parole del proprietario in `spikes/RISULTATI.md`.

**Raccomandazione: Electron**, per i motivi sopra — con la riserva che sono
**argomenti, non misure**, ed è per questo che l'ADR resta `Proposed`. ✅ **RICHIAMO DEL 2026-09-10:** le misure sono arrivate — SP-8, la *Decision* qui sopra — e lo stato è `Accepted`; la raccomandazione **ha retto**: dove i due gusci sono pari (M1, M3, M4 bocciata allo stesso modo, M5) ha deciso l'asimmetria dei costi qui sopra — Q2 su Linux, Q3 senza rimedio nel guscio e Q4 col trucco sono costi di **capacità** di Tauri, M2 un costo di **peso** di Electron.

### Come si chiude

✅ **Eseguito il 2026-09-10: SP-8**, coi criteri congelati prima in [`spikes/gui-shell/PROTOCOLLO.md`](../../spikes/gui-shell/PROTOCOLLO.md) — e con **quattro** righe qualitative, Q1–Q4, non le due della §2 del disegno del 2 com'era approvata il 2026-09-06: Q3 e Q4 nascono dalla §4 della stella polare.

Con una misura, non con una discussione. Stesso frontend minimo — chat in streaming
più una scena three.js — costruito sui due gusci, e quattro numeri:

| # | Misura | Perché discrimina |
|---|---|---|
| M1 | RAM a riposo e sotto streaming | è il costo principale imputato a Electron |
| M2 | dimensione del pacchetto installato | idem |
| M3 | fps del viewer 3D e API grafica realmente ottenuta, **su Windows e su Linux** | è il costo principale imputato a Tauri |
| M4 | **P3 con rendering vero**, salto webview compreso | chiude anche il margine stretto lasciato da ADR-0027 |
| **M5** | **VRAM a riposo e sotto carico 3D**, sui due gusci | aggiunta da [ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md): è il valore della **quota di presentazione**, e quanto sia governabile dipende da chi possiede il motore di rendering — impacchettato dal guscio o del sistema |

Se M3 mostra la stessa API grafica su entrambe le piattaforme con Tauri, l'argomento
principale a favore di Electron cade e la decisione si ribalta. ⚠️ **RICHIAMO DEL 2026-09-10:** non misurato — Linux non c'è (decisione C del disegno del 2); è l'**innesco** scritto nella *Decision*.

**Quando:** all'inizio del sotto-progetto 2. Non prima: il sotto-progetto 1 è kernel e
simulatore, interamente Rust, e **non dipende da questa scelta** — lo schema IPC è
definito lato core e ADR-0027 ha già dichiarato che non ci sono tipi condivisi.

## Consequences

- **Della decisione rimandata:**
  - Nulla si blocca: il sotto-progetto 1 non tocca la GUI.
  - Il rischio reale è che «non deciso» diventi «deciso per inerzia», che è
    esattamente ciò che è successo a Python prima di ADR-0028. Mitigazione: questo ADR
    è `Proposed`, e `check-docs.sh` lo segnala a ogni audit finché non si chiude. ✅ **Chiusa il 2026-09-10**: non per inerzia, con SP-8.

- **Comune a entrambe le opzioni, da progettare comunque:**
  - La webview esegue **solo contenuto locale nostro**. Caricarvi contenuto remoto
    riaprirebbe la superficie che [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md)
    ha chiuso: va vietato esplicitamente.
  - Il vincolo **niente SSR** vale per Tauri ed è naturale per Electron: la GUI è una
    SPA locale che parla col core via IPC. Nessuna delle due opzioni ne soffre.
  - ~~**La GPU usata dalla GUI non è arbitrata da nessuno.**~~ ✅ **Chiusa** da
    [ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md): quota di presentazione
    sottratta, con la concessione tenuta dal core. Era un problema di kernel e non di
    guscio, come previsto — il meccanismo è identico sulle due opzioni. Ne **esce** però
    un discriminante nuovo per questo ADR, la misura **M5** qui sopra.

- **Negative (accettate)** — per **Electron**, il guscio scelto:
  - un **terzo runtime**, Node, accanto a Rust e Python: il processo principale ha accesso pieno all'OS, e il ponte del
    preload è l'unica porta fra la pagina e il guscio — va tenuta stretta, e la webview carica solo contenuto locale
    nostro, come il punto comune qui sopra già impone.
  - Chromium **impacchettato**: la taglia di M2, e gli aggiornamenti del motore a carico nostro, a ogni release.
  - la metà Linux **non misurata**: l'innesco qui sopra; con Electron l'API grafica su Linux dipende da un flag che
    passiamo noi, e resta da provare.
  - chi decodifica `bincode` è Node (Q1): `bincode-ts`, fermo alla 1.0.0 del 2025-07-17 (ADR-0037), è la dipendenza che il
    canale `ipc` paga.
  - la finestra a parte **non si apre** sotto `file://` (Q3): il guscio della SPA serve la build da un'origine http(s) — un
    server locale, o un handler di protocollo — ed è **dedotto**, da misurare nella parte 2.
