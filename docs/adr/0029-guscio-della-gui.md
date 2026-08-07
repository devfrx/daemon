# ADR-0029: Guscio della GUI — Tauri o Electron

- **Status:** Proposed
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

> ⚠️ **Questa decisione è aperta.** `scripts/check-docs.sh` la segnala a ogni audit,
> come previsto. Non è una svista: è l'unica cosa dello stack che le misure fatte non
> sostengono, e forzarla su un argomento sarebbe contro il metodo di questo repository.

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

_(da prendere)_

**Raccomandazione: Electron**, per i motivi sopra — con la riserva che sono
**argomenti, non misure**, ed è per questo che l'ADR resta `Proposed`.

### Come si chiude

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
principale a favore di Electron cade e la decisione si ribalta.

**Quando:** all'inizio del sotto-progetto 2. Non prima: il sotto-progetto 1 è kernel e
simulatore, interamente Rust, e **non dipende da questa scelta** — lo schema IPC è
definito lato core e ADR-0027 ha già dichiarato che non ci sono tipi condivisi.

## Consequences

- **Della decisione rimandata:**
  - Nulla si blocca: il sotto-progetto 1 non tocca la GUI.
  - Il rischio reale è che «non deciso» diventi «deciso per inerzia», che è
    esattamente ciò che è successo a Python prima di ADR-0028. Mitigazione: questo ADR
    è `Proposed`, e `check-docs.sh` lo segnala a ogni audit finché non si chiude.

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
