# Sotto-progetto 2 — parte 1, lo spike del guscio: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario (punto 7 della §10 del [disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md),
> decisione 48 della stella polare). I passi usano le caselle (`- [ ]`) per il tracciamento.
> ⛔ **Il pre-controllo di ogni compito è fatto nelle sessioni che hanno scritto il piano, il
> 2026-09-09 — i compiti 1–5 contro il repository a `5ad4634`, i compiti 6–8 contro `b50e8b8` — e il piano
> intero è RILETTO il 2026-09-10 contro `4d626ea`, nella terza sessione («Come si riprende», la terza chiusura);
> l'esecuzione va in una sessione NUOVA.**

**Obiettivo.** Eseguire il **pezzo 1** della §3 della [stella polare della GUI](../specs/2026-09-07-direzione-gui-design.md):
lo spike **SP-8** in `spikes/gui-shell/` — la Home finta con `dockview-core` costruita su **due** gusci,
Electron e Tauri, con le misure **M1–M5** e le righe **Q1–Q4** che chiudono [ADR-0029](../../adr/0029-guscio-della-gui.md),
e l'**accettazione di `dockview`** in otto mosse giudicate dal proprietario provandole, la mossa 8 con la
mano di SP-7 — coi criteri **congelati prima** in `spikes/gui-shell/PROTOCOLLO.md`, l'esito in
`spikes/RISULTATI.md`, `.gitignore` e i lockfile, e la chiusura nei documenti di stato. La
Definizione di «fatto» è **copiata** dalla §10 del disegno del 2, com'è prescritto, nel compito che
chiude. ⛔ **La parte 2 — i pezzi 2–9 — si scrive DOPO la misura**, con lo stesso pre-controllo: non è
qui, ed è deliberato (punto 8 della §10 del 2).

**Forma.** Compiti in sequenza, e quanti siano lo dice la tabella della posizione qui sotto.
**Nessun file di `crates/`, `scripts/`, `.github/`, `Cargo.lock` cambia**: lo spike vive in
`spikes/gui-shell/`, fuori dal workspace (`exclude = ["spikes"]` nel manifesto di radice), e il cancello
non lo compila. Le decisioni sono già prese dai due disegni; questo piano le traduce in passi, e le
poche che aggiunge stanno nella tabella *«Le decisioni prese da questo piano»*.

**Strumenti.** `bash`, `awk`, `grep`, `sed -n` in lettura; **Python 3** per ogni scrittura su un file
**CRLF**, con l'aiutante qui sotto; `node` 24.9.0 e `npm` 11.6.0 (misurati il 2026-09-09 su questa
macchina); `cargo` 1.95.0; **PowerShell 5.1** per le misure; `git`. La porta di qualità è
`bash scripts/gate.sh`, e deve stampare `GATE GREEN` **prima di ogni commit**, anche di soli documenti;
`bash scripts/check-docs.sh` deve stampare `OK`. Per la mossa 8: `spikes/gesti/` com'è — la `venv`
`py -3.10`, il worker, il relay, il modello — e la telecamera di questa macchina. Per il compito 7: il
**proprietario**, davanti allo schermo.

⛔ **L'aiutante `replace_unique.py` vive nello scratchpad, mai nel repository.** È lo stesso dei piani
dei gesti e della knowledge base, riportato qui perché un piano porta i propri attrezzi nel testo
(vicolo cieco del piano dei gesti: un aiutante mai tracciato). Sostituisce **una** occorrenza unica,
conserva i fine-riga del file, e rifiuta se il testo vecchio manca o non è unico. I testi *Trova* e
*Sostituisci con* di ogni compito si mettono in due file e si passano a lui:

```python
"""replace_unique.py -- replace ONE unique occurrence of a text, keeping the file's line endings.

Usage: python replace_unique.py <file> <old.txt> <new.txt>

<old.txt> and <new.txt> hold the exact texts (UTF-8). A single trailing newline in each is
dropped, so a file written with an editor works. If <file> is CRLF, the texts are converted to
CRLF before matching and writing. Refuses when the old text is absent or not unique. Builds the
whole content first, writes a temporary file, then os.replace -- a rename cannot fail halfway
(gotcha #82).
"""
import io
import os
import sys

path, old_path, new_path = sys.argv[1:4]
raw = io.open(path, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw


def text(p):
    t = io.open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
    if t.endswith("\n"):
        t = t[:-1]
    return t.replace("\n", "\r\n") if crlf else t


old, new = text(old_path), text(new_path)
n = raw.count(old)
if n != 1:
    sys.exit(f"refused: {n} occurrences of the old text in {path}")
out = raw.replace(old, new)
tmp = path + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print(f"ok: {path} ({'CRLF' if crlf else 'LF'})")
```

Dopo ogni scrittura su un file CRLF si rimisura, e CR deve essere uguale alle righe:

```bash
for f in <i file toccati>; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol <i file toccati>
```

⚠️ **I *Trova* lunghi si prendono DAL FILE, non da questo piano:** dove un passo dice *«la riga che
comincia con …, intera, presa dal file»*, si copia con `grep` o `sed -n` e si mette nel file `old.txt`
così com'è. Un piano che ricopia una riga di tabella di 900 caratteri la ricopia sbagliata (voce E6 del
piano dei gesti).

**Disegni:** la [stella polare](../specs/2026-09-07-direzione-gui-design.md) — le §2, §3 e **§4**, «Il
modello della GUI», i vicoli ciechi — e il [disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md)
— le §2, §9 e **§10**, i vicoli ciechi. Si leggono **prima** dei compiti. La §4 della stella polare è la
casa delle otto mosse: questo piano le **ricopia nel protocollo** perché il protocollo è ciò che si
congela, e lo dice (D3).

---

## Vincoli globali

Valgono per ogni compito, senza che il compito li ripeta.

| # | Vincolo | Da |
|---|---|---|
| 1 | **le due spec non si toccano**: `git diff --name-only 5ad4634..HEAD -- docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` resta vuoto a ogni compito. I due disegni **non sono spec** e si toccano dove i compiti lo dicono, con richiami datati | teste dei disegni |
| 2 | **nessun codice di prodotto, nessun cancello, nessuna CI**: `git diff --name-only 5ad4634..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml` resta vuoto a ogni compito — è la condizione 5 della Definizione di «fatto»; la CI su Windows (X-1) e `cargo audit`/`npm audit` (X-3) sono della **parte 2** (D13) | §10 del 2; decisioni 44 e 45 |
| 3 | **codice in inglese, documenti in italiano**: TypeScript, Vue, JavaScript, Rust, PowerShell, Python e i loro commenti in inglese; il protocollo, l'esito, l'ADR e i verbali in italiano | §1.0 della spec |
| 4 | **nessuna cifra nuova in prosa**: date e comandi; una cifra che sostiene una decisione porta accanto il comando e la data e vive in **una** casa — per M1–M5 e Q1–Q4 la casa è **ADR-0029**, l'esito intero è `spikes/RISULTATI.md`, e nessun altro documento ricopia un numero | `CLAUDE.md` |
| 5 | **i fine-riga si conservano per file** e si rimisurano dopo ogni scrittura: la mappa dei file dice CRLF o LF **oggi**; `git ls-files --eol` prima e dopo, **invariato**; i file nuovi nascono **LF**; un CRLF si tocca con `replace_unique.py` o con Python `newline=""`, **mai** con `sed -i` (che in questa Git Bash toglie i CR) | `CLAUDE.md`; vicoli ciechi della stella polare |
| 6 | **ogni conteggio si rifà col comando** prima di leggere che cosa un documento ne dice; le cifre di questo piano sono istantanee del 2026-09-09 su `5ad4634` | disegni, *Metodo* |
| 7 | **il protocollo si congela al primo commit di codice dello spike**: il compito 1 committa il solo protocollo, il compito 2 il primo codice; una modifica dopo quel commit è una modifica al **metro** e si dichiara nel protocollo con richiamo datato, come E11 dei gesti | `spikes/PROTOCOLLO.md`, *Congelamento*; §4 della stella polare |
| 8 | **le versioni sono appuntate** in `package.json` e `Cargo.toml` come misurate il 2026-09-09 (P-8); il compito che le installa **rilancia** il comando di `riferimenti.md` e scrive nell'esito quelle del suo giorno; una **major** nuova non si prende («novità non è maturità»); una minor o patch nuova solo se l'appuntata non si installa, con voce d'errata | §9 del 2; `CLAUDE.md` |
| 9 | **i lockfile si committano** — `package-lock.json` dell'app, del guscio Electron e del guscio Tauri, `Cargo.lock` del guscio Tauri — e `.gitignore` copre `node_modules/`, `dist/`, `out/`, `target/`, `gen/`: dopo ogni corsa `git status --porcelain` nomina solo file della mappa | condizione 4 della Definizione di «fatto»; decisione 49 |
| 10 | **gli ADR sono append-only**: ADR-0029 si chiude riempiendo la *Decision* e aggiungendo `Negative (accettate)`; la testa, la *Context* e il confronto restano; ciò che diventa falso riceve un richiamo datato accanto | `CLAUDE.md`; precedente di AUD-032 |
| 11 | **il compendio resta sotto il tetto**: margine misurato prima e dopo ogni tocco col comando qui sotto (il 2026-09-09: `10848`); se va rosso si toglie prosa dalla §6, **non si alza il tetto** | §13 del compendio, gotcha #100 |
| 12 | **nessun link `](…)` a un file che non esiste ancora**: si nomina in code span, e il link nasce nel commit che crea il file. I piani sono fuori dal controllo dei link; `spikes/*.md` **no** — il protocollo linka con percorsi relativi a `spikes/gui-shell/` | trappola 5 di `check-docs.sh` |
| 13 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore**; il cancello e `check-docs.sh` girano **prima**; il messaggio comincia con `guscio(compito N…)` | `CLAUDE.md` |
| 14 | **l'accettazione condizionata del proprietario vale finché regge**: se un passo violasse uno dei cinque criteri di `anthropic-skills:decision-principles` — richiedesse una scorciatoia, duplicasse qualcosa che esiste, poggiasse su qualcosa che non è più vero — ci si **ferma e lo si riporta** | teste dei disegni |
| 15 | **il numeratore dei compiti** vive nella tabella della posizione e in nessun altro punto di questo file o del repository | gotcha #68 |
| 16 | **la macchina delle misure è questa**, e si dichiara nell'esito: `Intel(R) Core(TM) i7-14700HX`, 28 processori logici, la GPU di ADR-0002; WebView2 `152.0.4191.66` il 2026-09-09, **riletto** il giorno della misura; niente Linux (decisione C del 2): la condizione di ribaltamento dell'ADR non si misura, si sostituisce con la lettura di Q2 alle fonti e con l'innesco scritto | §2 del 2 |
| 17 | **`spikes/gesti/` non cambia**: worker, relay, pagina, `venv` e modello si usano com'erano (§4 della stella polare); la mossa 8 si prova **nel browser**, e la pagina di `vite` raggiunge il relay da un **proxy** (P-3), non toccando il relay | §4 della stella polare |

Il comando del margine, vincolo 11:

```bash
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
```

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

✅ **IL PIANO È SCRITTO IN DUE SESSIONI IL 2026-09-09 E RILETTO NELLA TERZA, IL 2026-09-10** — la prima si è chiusa per
contesto saturo dopo il compito 5; la seconda ha scritto i compiti **6, 7 e 8** e «Dopo il compito 8»; la terza ha fatto la
revisione del piano intero — copertura dei disegni, segnaposto, nomi fra i compiti, ogni *Trova* rilanciato — coi rimedi
scritti **nei compiti** (la tabella nella sezione *«Come si riprende»*, terza chiusura) e i tre spostamenti di stato.
⛔ **Si esegue in una sessione NUOVA, dal compito 1**, un subagente fresco per compito. Il pre-controllo delle quattro
domande sta nella sezione *«Il pre-controllo del piano»* qui sotto: P-1…P-18 sui compiti 1–5, P-19…P-25 sui compiti 6–8.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | il protocollo di SP-8, congelato — M1–M5, Q1–Q4, le otto mosse — e la riga SP-8 nella tabella degli spike di `roadmap.md` | uno, **solo documenti** | ✅ 2026-09-10 |
| **2** | la Home finta in `spikes/gui-shell/app/`: Vite, Vue 3, `dockview-core` — il nucleo e la striscia bloccati, le tessere, la presa grande, i comandi delle mosse 1–7, salva/ricarica/confronta; `.gitignore` e il lockfile | uno | ⬜ |
| **3** | le tre tessere vive: la chat che rende markdown token per token dal ponte, la scena `three` con fps e API nel titolo, la mano di SP-7 tradotta in eventi del puntatore (mossa 8) | uno | ⬜ |
| **4** | il guscio **Electron**, che legge la pipe con `net` e la passa alla webview; `electron-builder`; lo script di misura dell'albero di processi; `.gitignore` e il lockfile | uno | ⬜ |
| **5** | il guscio **Tauri**, che legge la pipe con `interprocess` e la emette come evento; le icone; `tauri build`; `.gitignore`, `Cargo.lock` e il lockfile | uno | ⬜ |
| **6** | le misure M1–M5 e Q1–Q2 sui due gusci installati, con gli script; la sezione SP-8 di `spikes/RISULTATI.md` coi numeri e le righe del proprietario dichiarate ⏳ | uno | ⬜ |
| **7** | col **proprietario**: le otto mosse nel browser, Q3 e Q4 nei due gusci, la CPU con la chat nascosta e la VRAM a pagina intera, la domanda A/B sul guscio; l'esito completo, la riga SP-8 chiusa | uno | ⬜ |
| **7-bis** | **solo su un no** all'insieme delle mosse: la stessa Home con `interactjs` — il perimetro è qui (D12), il compito si scrive nella sessione che riceve il no, col pre-controllo | — | ⬜ non necessario finché il compito 7 non lo dice |
| **8** | la chiusura: ADR-0029 `Accepted` con la decisione del proprietario, i totali degli ADR, il compendio (§4, §5, §6, §8, l'intestazione), `README.md`, `HANDOFF.md`, `roadmap.md`, l'audit e `GUI-REQUISITI.md` (P-19), i richiami nei due disegni, le fonti in `riferimenti.md`; la Definizione di «fatto» coi comandi; le due app disinstallate | uno | ⬜ |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del
[`COMPENDIO.md`](../../COMPENDIO.md), in un posto solo. Ciò che resta qui è la **posizione** del
piano — la tabella qui sopra, che chi esegue aggiorna nel commit del compito — e **come** si esegue.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro — e i disegni nelle
   sezioni che il compito nomina.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-09.
3. Se il compito dice il falso, **ci si ferma e si riporta**: non si aggira. Una divergenza è una voce
   d'errata prima di essere un rimedio. ⚠️ Vale anche per il codice dettato: un `npm run build` o un
   `cargo build` rosso sul testo dettato è una voce d'errata col testo corretto, **non** un aggiustamento
   silenzioso — è il precedente P-7/E-varie del piano dei gesti sull'API di MediaPipe.
4. Il cancello gira **prima** di ogni commit; il commit dice ciò che il compito ha fatto.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; per i compiti
   2–5 apre l'app nel browser o lancia il guscio e **guarda**, perché un `build` verde non prova che una
   tessera si veda; per il compito 8 rilegge ADR-0029 **contro i suoi fratelli** — 0027, 0030, 0033 — e
   non solo contro l'esito (gotcha #59).
6. Una seconda ondata di **sola prosa** la fa il coordinatore a mano, senza ri-revisione; se tocca un
   fatto o un comando, si ri-rivede. Dopo due ondate di prosa si chiude (gotcha #76 — si **toglie**, non
   si riscrive meglio).
7. Il compito 7 **non si dispaccia** a un subagente: lo esegue il coordinatore col proprietario allo
   schermo, come il compito 8 dei gesti; ciò che ne esce lo rilegge un revisore in sola lettura (E14 dei
   gesti: era l'unico artefatto che nessuno aveva riletto).

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota, e non resterà vuota.** Il pre-controllo ha trovato un difetto reale in **tutti** i
compiti dispacciati finora, senza una sola eccezione: quando ne trovi uno, si scrive **qui**, con il
proprio numero, prima di eseguirlo. Un piano è un'ipotesi. ⚠️ Le tre cose che questo piano **deduce** e
non ha potuto misurare senza eseguire — il permesso `core:default` di Tauri (P-7), il proxy SSE di
`vite` (P-3), la cattura del puntatore sotto eventi sintetici (P-6) — sono i primi candidati.

| # | Voce |
|---|---|
| — | *(vuota alla scrittura del piano, 2026-09-09)* |

---

## Il pre-controllo del piano — che cosa i disegni dicono e il repository (o la fonte) smentisce o precisa

Letto scrivendo, contro il repository a `5ad4634` e alle fonti primarie, il 2026-09-09. Ogni voce è una
delle quattro domande di `CLAUDE.md`, o una delle tre righe che l'elenco non coglie.

### P-1 — `dockview-core` 8.2.0 non spedisce un foglio di stile: il tema è un oggetto, e il pacchetto inietta il proprio `<style>`

Misurato nel pacchetto del registro npm (`dist.tarball` di `dockview-core@8.2.0`, scaricato e aperto in
memoria): **nessun** file `.css` fra i 117 del pacchetto; `dist/package/main.esm.mjs` contiene **una**
`createElement("style")` e i temi sono oggetti — `themeAbyss = { name: "abyss", className: "dockview-theme-abyss", colorScheme: "dark", … }`.
Quindi **nessun `import 'dockview-core/dist/styles/dockview.css'`** nel codice dettato: l'opzione
`theme: themeAbyss` basta. Domanda **1** — un `import` di un file che non esiste sarebbe stato il primo
rosso del compito 2.

### P-2 — La forma del gancio per lo scatto a griglia, e degli altri tipi che il codice usa

La §2 della stella polare nomina *«un gancio `transformFloatingGroupDrag` per lo scatto a griglia»* senza
la firma. Letta nel pacchetto: `transformFloatingGroupDrag?: (context: FloatingGroupDragContext) => { top: number; left: number } | void`,
con `FloatingGroupDragContext { group, proposed: Box, container: {width, height}, others: Box[], modifiers }`.
E con essa: `DockviewDndStrategy = 'auto' | 'pointer' | 'html5'`; `DockviewGroupPanelLocked = boolean | 'no-drop-target'`;
`DockviewGroupMoveParams { group?, position?: Position, index?, skipSetActive? }` con
`Position = 'top' | 'bottom' | 'left' | 'right' | 'center'` (e `Direction = 'left' | 'right' | 'above' | 'below' | 'within'` per `addPanel`);
`FloatingGroupOptions { x?, y?, width?, height?, position?, dragHandle?, disableSmartGuides? }`;
`DockviewPopoutGroupOptions { position?, popoutUrl?, onDidOpen?, onWillClose? }` e `addPopoutGroup(...): Promise<boolean>`;
`CreateComponentOptions { id, name }`; `IContentRenderer`/`ITabRenderer` con `element` e `init(parameters)` obbligatori,
`dispose`, `update`, `layout`, `toJSON`, `focus` facoltativi; `createDockview` esportato da `api/entryPoints.d.ts`;
`group.locked` e `group.header.hidden` esistono. Il codice dei compiti 2 e 3 usa **queste** firme. Domanda **3**
— l'artefatto sbagliato che compila: qui compila (esbuild non controlla i tipi), e sbaglierebbe **in silenzio**.

### P-3 — Il relay di SP-7 serve l'SSE **senza** intestazioni CORS: dalla pagina di `vite` un `EventSource` sarebbe bloccato

`grep -n 'Access-Control' spikes/gesti/relay/src/main.rs` non rende nulla; la risposta di `/stream` porta
solo `Content-Type`, `Cache-Control`, `Connection` (riga 62). La §4 della stella polare dava per dedotto
*«che il relay parli con la pagina di `vite` come parlava con `page.html`»*: `page.html` era **servita dal
relay stesso**, stessa origine. La pagina di `vite` sta su `http://localhost:5173`, altra origine → il
browser blocca. Rimedio senza toccare il relay (vincolo 17): il **proxy** di `vite` — `server.proxy['/stream'] → http://127.0.0.1:7878` —
così la pagina chiede `/stream` alla propria origine. ⚠️ **Dedotto, da provare alla prima corsa della
mossa 8:** che il proxy di `vite` non bufferizzi il flusso `text/event-stream`. Se lo fa, il ripiego è
**una riga** nel relay — `Access-Control-Allow-Origin: *` sulla risposta di `/stream` — con voce d'errata
e richiamo nel protocollo, perché tocca `spikes/gesti/`. Domanda **6** — ciò che smentisce può stare in
un banco di prova, e qui sta in una riga di intestazione HTTP.

### P-4 — Il `popoutUrl` predefinito di `dockview` è `/popout.html`, un percorso assoluto: sotto `file://` non esiste

Il doc di `DockviewPopoutGroupOptions` nel pacchetto: *«The same-origin path at which the popout window
will be created. Defaults to `/popout.html`»*. Il guscio Electron carica la build da `file://…/dist/index.html`
e Tauri da `tauri://localhost/`: `/popout.html` sotto `file://` risolve a `file:///popout.html`, che non
c'è. Il codice dettato passa `popoutUrl: 'popout.html'` (relativo alla pagina) e la build produce
`dist/popout.html` come **seconda pagina** di Vite. Domanda **1**, sul default che nessun disegno nomina.

### P-5 — Il nome della pipe: `interprocess` prepone `\\.\pipe\` ai nomi con spazio dei nomi, e Node lo apre così

Letto nel sorgente della crate `interprocess-2.4.4` (`src/os/windows/local_socket/name_type.rs`):
*«Namespaced strings have `\\.\pipe\` prepended to them»*. L'emettitore di `spikes/gui-ipc` usa
`NOME.to_ns_name::<GenericNamespaced>()` con `NOME = "gui-ipc-spike"` (`src/lib.rs`), quindi la pipe è
**`\\.\pipe\gui-ipc-spike`**: il guscio Electron la apre con `net.connect({ path: '\\\\.\\pipe\\gui-ipc-spike' })`
di Node, e il guscio Tauri con lo stesso `to_ns_name` dello spike (`spikes/gui-ipc/src/bin/gui.rs` è il
precedente del client). Domanda **1**, sull'interfaccia fra due processi che nessun disegno scrive per esteso.

### P-6 — `three`: `./webgpu` è un export del pacchetto, `WebGPURenderer` ricade su WebGL2 da solo, e il backend dice quale API ha ottenuto

Letto il 2026-09-09: il packument di `three` elenca gli export `.`, `./tsl`, `./webgpu`; in
`src/renderers/webgpu/WebGPURenderer.js` l'opzione `forceWebGL` e l'import di `WebGLBackend` dal
ripiego; `WebGPUBackend` pone `this.isWebGPUBackend = true` e `WebGLBackend` pone `this.isWebGLBackend = true`;
`Renderer.js` espone `this.backend` e `async init()`. Quindi M3 *«l'API grafica ottenuta davvero»* si
legge da `renderer.backend.isWebGPUBackend` dopo `await renderer.init()`, senza indovinare dal
`navigator`. ⚠️ **Dedotto, da provare alla mossa 8 e non prima:** che `dockview` in `dndStrategy: 'pointer'`
**non** chiami `setPointerCapture` sull'elemento della linguetta — con un `pointerId` sintetico che nessun
puntatore vero ha generato, quella chiamata solleva `InvalidPointerId` e il trascinamento non parte. Se
succede è il **no tecnico** che il protocollo prevede per la mossa 8, non un difetto da aggirare.

### P-7 — Tauri 2: le chiavi della configurazione sono verificate nello schema, il permesso `core:default` **no**

Lo schema `config.schema.json` del ramo `dev` di `tauri-apps/tauri` porta `frontendDist`, `devUrl`,
`beforeDevCommand`, `beforeBuildCommand`, `windows`, `csp`, `bundle`, `identifier`, `productName`,
`withGlobalTauri`, `targets`, `active`; `tauri::Emitter::emit(&self, event: &str, payload: S)` è su docs.rs.
I tre percorsi tentati per i file dei permessi (`crates/tauri/permissions/{,event/,window/}default.toml`)
hanno reso **404**: che `"permissions": ["core:default"]` in `capabilities/default.json` basti a `listen`
è **dedotto** dal modello di `create-tauri-app`, e lo prova il primo `tauri dev` del compito 5 — se
`listen` fallisce, la voce d'errata porta il nome del permesso letto quel giorno.

### P-8 — Le versioni del 2026-09-09, e quali si appuntano

Rilanciato il comando di `riferimenti.md` (registro npm e crates.io) oggi: `dockview-core` 8.2.0
(2026-08-19, MIT) · `interactjs` 1.10.28 · `vue` 3.5.42 · `vite` 8.2.2 · `@vitejs/plugin-vue` 6.0.8 ·
`vue-tsc` 3.3.11 · `typescript` **7.0.2** · `three` **0.186.0** (2026-09-08) · `markdown-it` 15.0.1 ·
`electron` **44.3.0** (2026-09-08) · `electron-builder` 26.15.3 · `@tauri-apps/cli` 2.11.4 ·
`@tauri-apps/api` 2.11.1 · crate `tauri` 2.11.5 (2026-07-01) · `tauri-build` 2.6.3 · `wry` **0.57.0**
(2026-09-08) · `interprocess` 2.4.4. Si appuntano: `three` **0.185.1** — la 0.186.0 ha un giorno, e la
scena non chiede nulla di nuovo — ed `electron` **44.3.0**, patch della major corrente (M1 e M2 misurano
la major). `typescript` e `vue-tsc` **non entrano**: Vite transpila con esbuild senza controllare i tipi,
e uno spike usa-e-getta non paga un controllo dei tipi su TypeScript 7. Domanda **1** sullo stato dell'arte.

### P-9 — La macchina, misurata: WebView2, i contatori GPU, `node`, il cancello

Il 2026-09-09: WebView2 Runtime `152.0.4191.66` (chiave `EdgeUpdate\Clients\{F3017226-…}`, valore `pv`);
`Get-Counter -ListSet 'GPU Process Memory'` espone `Shared Usage`, `Dedicated Usage`, `Non Local Usage` —
è il contatore che la §2 del 2 nomina per M5; `node` v24.9.0, `npm` 11.6.0; `@tauri-apps/cli` **non** è
installato (`npx --no-install` fallisce): è una `devDependency` del compito 5; `spikes/gui-ipc/target/release/core.exe`
**esiste** (build del 2026-08-06) e si **ricompila** — il suo `Cargo.lock` è ignorato da git, quindi
risolve `interprocess` 2.4.4 quel giorno. ⚠️ **Dedotto:** la forma delle istanze del contatore,
`pid_<n>_luid_…`, che lo script di misura filtra con `^pid_(\d+)_` — ✅ **misurato il 2026-09-10**, alla rilettura: `pid_10364_luid_0x00000000_0x000183A5_phys_0`, il filtro regge; se la VRAM restasse a zero con la
scena in moto, l'indiziato è quel filtro, e la voce d'errata porta il nome d'istanza letto con
`Get-Counter -ListSet 'GPU Process Memory' | Select -Expand PathsWithInstances`.

### P-10 — Le voci aperte del repository, rilette per il chiusore: nessuna ha questo piano

Come la riga di `CLAUDE.md` su `superpowers:writing-plans` prescrive, lette **prima** di scrivere, col
comando: le tabelle *«Le voci aperte del Traguardo 5»* e *«… del Traguardo 6»* di
[`porta-di-qualita.md`](../../porta-di-qualita.md) coi due `awk` della §6 del compendio (29 e 28 righe
non chiuse il 2026-09-09, colonna «Chi la chiude» letta una per una), le voci senza numero AUD
dell'[audit](../../audit-2026-08-27.md), la tabella del 2026-08-10 della §6 del compendio, la §9 del
disegno del 2 e le registrate della stella polare. **Nessuna ha come chiusore questo piano, né «il
proprietario, prima»**: X-1 e X-3 sono **decise** (A, decisioni 44 e 45) e la loro esecuzione sta nella
parte 2 (D13); AUD-004 sbarra il **13**, non il 2 (decisione 43); tre voci del Traguardo 6 (9, 26, 27)
aspettano il primo worker vero, il 12. L'elenco di ciò che si sa e si dichiara è nella sezione *«Le voci
aperte che questo piano SA, e non chiude»*.

### P-11 — Il guardiano dei totali: «38 ADR in stato Accepted» vive in DUE case, e diventa 39 nello stesso commit che chiude ADR-0029

`scripts/check-docs.sh` confronta `<n> ADR in stato …` con il numero di ADR `Accepted` e `<n> ADR` col
totale, in sei file, **togliendo i code span** prima di contare. Rilanciata la sua forma il 2026-09-09:
`docs/HANDOFF.md:1052` e `docs/COMPENDIO.md:160` dichiarano **38 in stato Accepted**; il totale **39**
sta in otto case (`HANDOFF.md` ×3, `roadmap.md`, `COMPENDIO.md` ×2, `AVVIO-CHAT.md`, `CLAUDE.md`) e **non
cambia**. Il compito 8 porta le due a 39 nel commit che scrive `Accepted` in ADR-0029, o il cancello va
rosso (vincolo 11 del piano dei gesti). Domanda **2** — il controllo esiste, e va nominato prima.

### P-12 — `spikes/PROTOCOLLO.md` è congelato per SP-5 e SP-6, e il controllo dei link legge `spikes/*.md`

Come P-8 del piano dei gesti: SP-8 ha il **proprio** protocollo, `spikes/gui-shell/PROTOCOLLO.md`, e la
sezione dei risultati la **propria** in `spikes/RISULTATI.md` — il numero **8** perché l'elenco di quel
file finisce a SP-7 (`grep -n '^## SP-' spikes/RISULTATI.md`), e SP-1…SP-4 vivono nella spec del kernel.
`check-docs.sh` scandisce i `.md` **non ignorati** (`git check-ignore`), quindi anche il protocollo: i
suoi link sono relativi a `spikes/gui-shell/` — `../../docs/…`, `../GUI-REQUISITI.md`, `../gesti/…`.

### P-13 — L'emettitore di `spikes/gui-ipc` emette solo DOPO che un client si collega, e poi esce: da qui l'ordine «riposo, poi flusso» di M1

Letto in `spikes/gui-ipc/src/bin/core.rs`: `listener.incoming().next()` **blocca** finché una GUI non si
collega, poi 2000 messaggi in 10 s e il processo termina. Quindi i due gusci **ritentano** la connessione
ogni due secondi (D8) e lo script di misura lancia il guscio da solo, campiona il **riposo**, avvia
l'emettitore a un secondo fissato, e campiona il **flusso** — un comando solo per M1 e M4 (D7). E il
carico dell'emettitore è `"lorem ipsum dolor sit amet"` su ogni riga: il markdown reso è testo semplice,
**senza blocchi di codice** — dichiarato nel protocollo, perché M4 misura il rendering vero *di quel
flusso*, e inventare un carico diverso cambierebbe il confronto con P3 di SP-5/SP-6.

### P-14 — I fine-riga, misurati oggi

`git ls-files --eol` il 2026-09-09: **CRLF** nell'albero (`i/lf w/crlf`) — `.gitignore`, `docs/COMPENDIO.md`,
`docs/README.md`, `docs/HANDOFF.md`, `docs/roadmap.md`, `docs/riferimenti.md`, `docs/adr/0029-guscio-della-gui.md`,
`spikes/RISULTATI.md`, `spikes/PROTOCOLLO.md`; **LF** — i due disegni, `spikes/gesti/PROTOCOLLO.md`, i
piani. La mappa dei file lo riporta per file; ogni compito rimisura prima di scrivere.

### P-15 — La riga «Ultimo aggiornamento» di `roadmap.md` si riallinea in OGNI commit che tocca il file

La riga 6 di `roadmap.md` porta la data e, sotto, due richiami che raccontano **due** volte in cui è
rimasta indietro. I compiti 1, 7 e 8 toccano il file: ciascuno riscrive quella riga nello stesso commit
(E14 ed E17④ del piano dei gesti). Il *Trova* è la riga intera **presa dal file**.

### P-16 — La tabella dello stato della stella polare dà `spikes/` fra le cose non toccate: la parte 1 la rende stantia

La riga *«codice e spec non toccati»* (riga 54 della stella polare) lancia `git diff --stat 664265a..HEAD -- crates/ scripts/ spikes/ …`
e attende *«nulla»*. Dopo il compito 2 rende `spikes/gui-shell/…`. Il compito 8 vi appende il richiamo
datato; la riga *«Codice di prodotto non toccato»* della §10 del 2 resta vera (`crates/`, `Cargo.lock`,
`Cargo.toml` intatti). Domanda **5** — il contratto cresce sotto il piano.

### P-17 — Il margine del compendio è `10848` byte, e il compito 8 lo consuma cinque volte

Misurato il 2026-09-09 col comando del vincolo 11. Il compito 8 tocca la §4, la §5, la §6 (una riga in
«Chiuso» e il puntatore), la §8 e l'intestazione. La riga 21 — l'intestazione — è oggi un capoverso di
cronaca di **sedici** riprese: si **riscrive corta** e il testo com'era va in
[`archivio/stato-storico.md`](../../archivio/stato-storico.md) parola per parola (D16), che è anche ciò
che libera margine. Se il tetto andasse rosso lo stesso, si toglie prosa dalla §6.

### P-18 — Niente di ciò che il piano detta esiste già (domanda 4), misurato

`ls spikes/gui-shell` → non esiste; `ls gui` → non esiste; `grep -rln 'gui-shell' docs/ spikes/ .gitignore` rende
solo i due disegni e i due archivi delle consegne; `grep -n 'SP-8' spikes/RISULTATI.md docs/roadmap.md` non
rende nulla; ADR-0029 è `Proposed` con `_(da prendere)_` alla riga 66. L'unica cosa che esiste ed è
**riusata** è lo spike `spikes/gui-ipc/` — l'emettitore, la libreria col nome della pipe e la forma del
messaggio — e `spikes/gesti/` per la mossa 8.

### P-19 — Le case della decisione aperta sono TREDICI, non le cinque della mappa scritta a metà

Il 2026-09-09, seconda sessione, contro `b50e8b8`:
`grep -rn 'Proposed\|Tauri o Electron\|Tauri contro Electron\|guscio: aperto\|tranne il guscio' docs/*.md CLAUDE.md docs/design/*.md spikes/*.md | grep -v archivio`
rende `docs/COMPENDIO.md` 104, 160, 454; `docs/HANDOFF.md` 155–156, 208–210, 1001, 1052, 1262; `docs/README.md` 168;
`docs/roadmap.md` 23–26 e 289; `docs/audit-2026-08-27.md` 29 (un verbale datato di come l'audit fu condotto, che resta)
e 351 (una regola al presente, che riceve il richiamo); `spikes/GUI-REQUISITI.md` 63–64; niente in `docs/design/` né in
`tracciabilita.md`. La traccia del compito 8 ne nominava cinque. Il compito 8 le chiude **tutte** e rilancia il `grep`
prima e dopo (D22). Domanda **2** — la sonda che mancava.

### P-20 — Le righe `⏭️` del compendio sono TRE, non due

`grep -n '⏭️' docs/COMPENDIO.md` il 2026-09-09 rende **667** (il puntatore), **675** (il punto 2) e **735** — un code span
nella tabella delle voci aperte, *«ogni riga che porta `⏭️` deve nominare la §6»*. La traccia del compito 8 attendeva
*«due righe come oggi»*: un'attesa mai contata, la specie delle nove voci d'errata dei piani precedenti. Il compito 8
misura il conteggio nel Passo 1 e pretende che **non cambi** (D24); lo spostamento di stato (c) di «Come si riprende» non
aggiunge un marcatore. Domanda **1** — la sonda sbagliata.

### P-21 — L'installazione silenziosa NSIS: le cartelle e l'avvio dopo `/S` sono DEDOTTI

Dedotto, non letto alla fonte: `electron-builder` con `perMachine: false` installa in `%LOCALAPPDATA%\Programs\<productName>`
e `tauri build` per utente in `%LOCALAPPDATA%\<productName>`; e che nessuno dei due **avvii** l'app dopo un `/S`. Il compito 6
**legge** le cartelle con `Get-ChildItem` e le scrive nell'esito, e chiude con `Stop-Process` un'app che fosse partita
(D19): l'artefatto non dipende dalla deduzione. Se una cartella non c'è dove si attende, è una voce d'errata col percorso
letto. Domanda **3** — l'artefatto sbagliato che compila.

### P-22 — Il margine del compendio alla seconda sessione: `10395`

Il richiamo della prima chiusura nel punto 2 della §6 ha consumato 453 byte dei `10848` di P-17; il richiamo di questa
chiusura è della stessa taglia. Il compito 8 lo rimisura (vincolo 11) e l'intestazione corta (D16) libera più di quanto
le righe nuove consumano. Il comando è quello del vincolo 11.

### P-23 — `check-docs.sh` esclude i piani dal controllo dei link e legge `spikes/*.md`

Letto nello script: la riga 33 esclude `./docs/superpowers/plans/*` dal `find`, e il filtro `git check-ignore` toglie
solo ciò che git ignora. Quindi i link dentro **questo** piano — anche nei modelli delle sezioni — sono liberi, e i link che
i compiti 6 e 8 **scrivono** in `spikes/RISULTATI.md`, in ADR-0029 e in `riferimenti.md` devono risolvere dalla cartella
del file: `gui-shell/PROTOCOLLO.md` da `spikes/`, `../../spikes/…` da `docs/adr/`, `../spikes/RISULTATI.md` e
`superpowers/plans/…` da `docs/`. Domanda **2**.

### P-24 — Come `check-docs.sh` conta gli ADR, e che cosa stampa per i `Proposed`

Letto nello script il 2026-09-09: gli `Accepted` si contano con `^- \*\*Status:\*\* Accepted` (riga 287), quindi la riga
dell'ADR deve essere **esattamente** `- **Status:** Accepted`; i `Proposed` con `Status:\*\* Proposed` (riga 305), e sotto
`== ADR still in Proposed ==` lo script stampa `  (none)` quando non ce n'è; la guardia dei conteggi (righe 288–301) legge
**sei** file — `HANDOFF.md`, `roadmap.md`, `README.md`, `COMPENDIO.md`, `AVVIO-CHAT.md`, `CLAUDE.md` — togliendo i code span,
e il totale **39** vive anche in `AVVIO-CHAT.md` 163 e `CLAUDE.md` 8, dove non cambia. Domanda **2**.

### P-25 — La radice dell'albero per `tree.ps1 -AttachPid` sull'app installata

Per Electron la radice è il **processo principale**, il primo partito — `Get-Process sp8-electron | Sort-Object StartTime | Select-Object -First 1`
— dedotto da come Electron avvia gpu, renderer e utility come figli del principale, e coerente con `Get-Tree` del compito
4, che ricostruisce l'albero dal genitore; per Tauri il processo è uno, e i `msedgewebview2.exe` sono figli. Se
`Get-Tree` non prendesse i figli — `procs` fermo a 1 — è la stessa voce d'errata di P-9. Domanda **3**.

---
## Le decisioni prese da questo piano

⛔ **Sono decisioni del piano, non dei disegni, e chi esegue può ribaltarle** portando la misura che le
smentisce — è ciò per cui esiste l'errata.

| | Decisione | Perché |
|---|---|---|
| **D1** | lo spike è **SP-8**, in `spikes/gui-shell/` con `PROTOCOLLO.md`, `app/` (la Home), `electron/`, `tauri/`, `measure/`; l'esito è la sezione **SP-8** di `spikes/RISULTATI.md` nella forma di SP-7 | P-12; §2 del 2 («una sezione nuova, il numero dopo l'ultimo dell'elenco») |
| **D2** | il compito 1 committa il **solo protocollo** e la riga SP-8 in roadmap; il primo codice arriva col compito 2 | vincolo 7; D6 del piano dei gesti: «criteri prima della misura» provato da `git log`, non dichiarato |
| **D3** | il protocollo **ricopia** le otto mosse dalla §4 della stella polare, parola per parola nella colonna «Passa se», e lo dice | ciò che si congela dev'essere il file congelato, non un rimando a un file vivo; la §4 resta la casa del **perché** |
| **D4** | **un** frontend, `app/`, costruito da Vite in `app/dist/`, che **entrambi** i gusci caricano tale e quale; la Home si sviluppa e si prova nel browser a `http://localhost:5173`, i gusci caricano la build | decisione 31 della stella polare: il confronto fra i gusci è equo solo se il frontend è lo stesso |
| **D5** | il **ponte** della Home (`src/bridge.ts`): `window.harness.onLine` se c'è il preload di Electron, l'evento `line` di Tauri se c'è `__TAURI_INTERNALS__`, altrimenti un **generatore locale** con la cadenza dell'emettitore (2000 in 10 s, poi una pausa, poi da capo); le tessere non sanno quale | §6a del 2: la SPA non tocca mai un socket e parla con un ponte piccolo; nel browser le mosse si provano senza un guscio |
| **D6** | i numeri che la pagina misura — fps medi e minimi su 30 s, l'API, i messaggi ricevuti, i buchi (P1), il ritardo massimo e medio (P2), la sorgente, la strategia di trascinamento, il token `Chrome/…` dello user agent — vanno nel **titolo della finestra** una volta al secondo; lo script di misura li legge con `MainWindowTitle` | un secondo canale (file, socket, DevTools) sarebbe un pezzo in più da costruire e da spiegare; il titolo lo leggono tutti e tre i gusci uguale |
| **D7** | le misure M1, M4, M5 con **uno** script PowerShell, `measure/tree.ps1`: lancia il guscio, campiona ogni 250 ms l'**albero** dei processi (RSS sommata, CPU come % di **un** core, VRAM dai contatori `GPU Process Memory` una volta al secondo, il titolo), avvia l'emettitore a un secondo fissato, ferma tutto alla fine; M2 con `measure/size.ps1` sulla cartella installata | P-13; la §2 del 2 sulle trappole di M1 (WebView2 fuori dal PID) e M5 (`nvidia-smi` in WDDM); P3 è «% di un core», quindi nessuna divisione per i processori |
| **D8** | i due gusci **ritentano** la connessione alla pipe ogni due secondi e mostrano «core assente» finché non c'è: Electron nel processo principale con `net`, Tauri in un thread con `interprocess` | P-13; è la fascia «riprova» della §6a del 2, e permette a `tree.ps1` di campionare il riposo prima del flusso |
| **D9** | i pacchetti si costruiscono **NSIS per utente** — `electron-builder --win nsis` con `oneClick` e `perMachine: false`; `tauri build` con `targets: ["nsis"]` — e M2 misura la **cartella installata** più la taglia dell'installatore, dopo un'installazione silenziosa (`/S`) | ADR-0029: «si costruisce l'installatore di ciascuno e si misura la cartella installata»; senza diritti di amministratore |
| **D10** | il guscio Electron carica la build da una **copia** `electron/dist/` fatta da uno script `sync` (`electron-builder` non impacchetta file fuori dalla cartella del progetto) | un vincolo dello strumento, letto nel suo modello di `files`; la copia è ignorata da git |
| **D11** | il guscio Tauri ha il **layout standard** — `tauri/package.json` e `tauri/src-tauri/` con `Cargo.toml`, `tauri.conf.json`, `capabilities/`, `icons/` — e le icone nascono da un PNG generato da `measure/icon.py` (Python puro, `zlib`) con `npx tauri icon` | il CLI cerca `src-tauri/`; un layout diverso è una domanda in più a uno strumento che non serve; `tauri build` senza icone si ferma |
| **D12** | il **compito 7-bis** — la stessa Home con `interactjs`, stesse mosse, stesso protocollo — si scrive **solo su un no** all'insieme delle mosse, nella sessione che lo riceve, col proprio pre-controllo; qui ne sta il **perimetro**: `app/` guadagna una seconda `home-interact.ts` sullo stesso `bridge.ts` e le stesse tessere, un contenitore libero in cui ogni tessera è un `div` assoluto con `interact(...).draggable().resizable()`, lo scatto a griglia scritto noi, «pagina intera» come classe CSS, la finestra a parte con `window.open`, la disposizione in JSON nostro; i gusci non cambiano | decisione 30 della stella polare: la tela libera si costruisce solo su un no; scriverne il codice oggi pagherebbe una strada già scartata sul merito (YAGNI), e il piano lo **dichiara** invece di lasciare un segnaposto |
| **D13** | la CI su Windows (X-1) e `cargo audit` in `gate.sh` più `npm audit` in `gate-gui.sh` (X-3) sono compiti della **parte 2**, accanto al passo `setup-node` | decisione 44 le mette «accanto al passo `setup-node` della §8»; la condizione 5 della Definizione di «fatto» tiene `scripts/` e `.github/` intatti nella parte 1 |
| **D14** | **nessuna riga** per i due disegni e per questo piano nella §12 del compendio e nella tabella «Specifiche» di `README.md` nella parte 1: entrano con la parte 2; la riga di questo piano nella tabella dei **piani** di `roadmap.md` la scrive la sessione che scrive il piano (*«scritto il 2026-09-09»*), e il compito 8 la porta a *«eseguito»* | punto 8 della §10 del 2, letterale; D9 del piano dei gesti per la riga in roadmap |
| **D15** | la decisione sul guscio è del **proprietario** (Deciders di ADR-0029) e si prende al **compito 7**, coi numeri di M1–M5 e Q1–Q4 in mano, come domanda A/B col consiglio derivato dall'asimmetria dei costi scritta nell'ADR; il compito 8 la **scrive** con la sua lettera e le sue parole | un ADR `Accepted` con una decisione presa dall'agente sarebbe il contrario di ciò che ADR-0029 dice di sé («argomenti, non misure», «Deciders: proprietario») |
| **D16** | l'intestazione del compendio (riga 21) si **riscrive corta** al compito 8 e il testo com'era va in `docs/archivio/stato-storico.md` come blocco datato, parola per parola | P-17; la regola di `CLAUDE.md` sui verbali che non restano nel documento corretto, e il mandato del 2026-09-09 (decisione 26) |
| **D17** | la sezione SP-8 di `RISULTATI.md` nasce al compito 6 con M1–M5 e Q1–Q2 **piene** e le righe che aspettano il proprietario — le otto mosse, Q3, Q4, la CPU con la chat nascosta, la VRAM a pagina intera — scritte **⏳ al compito 7** con quelle parole; nessuno slot `<…>` si committa mai | la regola del compito 8 dei gesti («la sezione non si committa con uno slot vuoto») e lo stato «⏳» che i disegni già usano per ciò che aspetta lui |
| **D18** | ogni compito **rimisura** la baseline — cancello, `check-docs.sh`, margine — e non cita quella scritta qui | gotcha #31 |
| **D19** | le due app si **installano** (NSIS, `/S`) e si misurano dalla cartella installata; restano installate dal compito 6 al compito 8, che le disinstalla dopo la corsa della condizione 4 della Definizione di «fatto» | ADR-0029 chiede M2 sulla cartella installata, e M1, M4 e M5 sull'eseguibile installato sono ciò che l'utente avrà; disinstallare al compito 8 lascia al revisore del 7 la possibilità di rilanciare. Costo: due app sulla macchina del proprietario per due compiti |
| **D20** | `tree.ps1 -Seconds 60 -EmitterAt 32`: trentadue secondi di riposo perché la finestra dei 30 s di M3 sia piena **prima** del flusso, e il titolo a riposo si legge dal CSV | il protocollo vuole M3 «alla taglia della Home» a riposo; il `last title` copre il flusso, e la traccia della prima chiusura (`-Seconds 45 -EmitterAt 15`) avrebbe dato un M3 solo sotto flusso. Costo: quattro minuti di corse invece di tre |
| **D21** | i CSV di `tree.ps1` in `$HOME/sp8-measure/`, fuori dal repository **e** fuori dallo scratchpad di sessione; si cancellano al compito 8 | il compito 7 può essere un'altra sessione e il compito 8 può volerli rileggere; lo scratchpad muore con la sessione. Costo: una cartella da ricordarsi di togliere, e il compito 8 lo fa |
| **D22** | il compito 8 chiude la decisione in **tutte** le case che il `grep` del suo Passo 1 rende — tredici il 2026-09-09 (P-19), non le cinque della mappa scritta a metà — e il `grep` sta nel compito, così le case nuove non sfuggono | «un rimedio si chiude su TUTTE le case della frase», decisione dell'audit del 2026-08-27; la mappa dei file è aggiornata di conseguenza. Costo: sette sostituzioni in più |
| **D23** | le fonti di Q2 e le versioni del giorno dello spike in `riferimenti.md` come **sezione nuova** `##` prima di «Cosa NON abbiamo adottato», non come `###` in coda al file né dentro la sezione dei due disegni | in coda cadrebbe sotto «Avvertenza sulla stabilità delle fonti»; dentro la sezione dei disegni ne falsificherebbe il titolo datato *«dal 2026-09-06 al 2026-09-09»*. Costo: una sezione in più in un file che ne ha già molte |
| **D24** | il conteggio delle righe `⏭️` del compendio si **misura** nel Passo 1 del compito 8 e si pretende invariato, invece di attendersene «due» | la traccia diceva due e il `grep` ne rende tre (P-20): un'attesa contata non invecchia, una ricordata sì |
| **D25** | nei compiti 6, 7 e 8 nessuno slot dentro una cella di tabella porta una barra verticale — «<il guscio scelto>», «<resta oppure esce>», non «<Electron \| Tauri>» — e i comandi con `\|` stanno nei blocchi di codice, non nelle celle | una barra in uno slot spezza la tabella e `check-docs.sh` non lo vede (vicolo cieco della tredicesima ripresa della stella polare); una `\|` in una cella copiata dentro un'espressione regolare cambia il comando. Costo: nessuno |

**La baseline di partenza, misurata il 2026-09-09 su `5ad4634` e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `bash scripts/check-docs.sh` → `OK — no inconsistencies.` ·
il comando del vincolo 11 → `10848` · `git status -sb` → `## main...origin/main`, pulito.
**E alla seconda sessione, su `b50e8b8`, lo stesso giorno:** `GATE GREEN` · `OK — no inconsistencies.` · il margine
`10395` (P-22) · albero pulito.

---

## La mappa dei file

| File | Chi lo tocca | Fine-riga il 2026-09-09 | Responsabilità |
|---|---|---|---|
| `spikes/gui-shell/PROTOCOLLO.md` | compito 1, **creato** | LF | i criteri congelati: M1–M5, Q1–Q4, le otto mosse |
| `docs/roadmap.md` | compiti 1, 7, 8; la sessione che scrive il piano | **CRLF** | la riga SP-8 nella tabella degli spike; la riga di questo piano nella tabella dei piani; la riga «Ultimo aggiornamento» a ogni tocco; al compito 8 lo «Stato in una riga» e la riga delle decisioni da prendere (P-19) |
| `spikes/gui-shell/app/package.json` · `vite.config.ts` · `index.html` · `popout.html` · `src/main.ts` · `src/style.css` · `src/vue-bridge.ts` · `src/home.ts` · `src/tiles/Tile.vue` | compito 2, **creati**; `main.ts` e `home.ts` **modificati** dal compito 3 | LF | la Home finta: Vite, Vue 3, `dockview-core`, le mosse 1–7 |
| `spikes/gui-shell/app/package-lock.json` | compito 2, **creato** da `npm install` | LF (come `npm` lo scrive; si misura) | il lockfile, committato |
| `spikes/gui-shell/app/src/bridge.ts` · `src/stats.ts` · `src/hand.ts` · `src/tiles/Chat.vue` · `src/tiles/Scene.vue` · `src/tiles/Hand.vue` | compito 3, **creati** | LF | il ponte, il titolo con le misure, la mano, le tre tessere vive |
| `.gitignore` | compiti 2, 4, 5 | **CRLF** | `node_modules/`, `dist/`, `out/`, `target/`, `gen/` dello spike |
| `spikes/gui-shell/electron/package.json` · `main.js` · `preload.js` · `package-lock.json` | compito 4, **creati** | LF | il guscio Electron e `electron-builder` |
| `spikes/gui-shell/measure/tree.ps1` · `size.ps1` | compito 4, **creati** | LF | M1, M4, M5 sull'albero di processi; M2 |
| `spikes/gui-shell/tauri/package.json` · `package-lock.json` · `src-tauri/Cargo.toml` · `src-tauri/Cargo.lock` · `src-tauri/build.rs` · `src-tauri/src/main.rs` · `src-tauri/tauri.conf.json` · `src-tauri/capabilities/default.json` · `icon.png` · `src-tauri/icons/*` · `measure/icon.py` | compito 5, **creati** | LF (le icone sono binari) | il guscio Tauri |
| `spikes/RISULTATI.md` | compiti 6, 7 | **CRLF** | la sezione SP-8 e la riga della data |
| `docs/adr/0029-guscio-della-gui.md` | compito 8 | **CRLF** | la Decision, Q1–Q4, l'innesco Linux, `Negative (accettate)`, `Accepted`, il richiamo in testa |
| `docs/COMPENDIO.md` | compito 8 | **CRLF** | la riga 160 (i totali), la riga del guscio in §4, il richiamo sulla voce 0029 di §5, la riga in «Chiuso» e il puntatore di §6, la riga SP-8 in §8, l'intestazione (D16) |
| `docs/archivio/stato-storico.md` | compito 8 | **CRLF** | l'intestazione del compendio com'era (D16) |
| `docs/README.md` | compito 8 | **CRLF** | la riga 0029 dell'indice degli ADR |
| `docs/HANDOFF.md` | compito 8 | **CRLF** | le cinque case della decisione aperta (P-19), fra cui «38 ADR in stato `Accepted`» → 39 (P-11); un gotcha nuovo, se c'è |
| `docs/riferimenti.md` | compito 8 | **CRLF** | la sezione SP-8: le versioni del giorno dello spike e le fonti di Q2 (D23) |
| `docs/audit-2026-08-27.md` | compito 8 | **CRLF** | il richiamo sulla riga delle decisioni deliberate, «ADR-0029 fermo a `Proposed`» (P-19) |
| `spikes/GUI-REQUISITI.md` | compito 8 | **CRLF** | il richiamo sulla riga del guscio «ancora aperta» (P-19) |
| `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | compito 8; la sessione che scrive il piano | LF | i richiami datati in §2 e §10 |
| `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` | compito 8 | LF | i richiami datati in §4 e nella tabella dello stato (P-16) |
| `docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md` | ogni compito; le sessioni che lo scrivono | LF | la tabella della posizione, l'errata, «Come si riprende»; al compito 7-bis, su un no |
| `spikes/gui-ipc/` · `spikes/gesti/` | **nessuno**: si usano com'erano | — | l'emettitore; il worker, il relay e la pagina della mano |

---

## Le voci aperte che questo piano SA, e non chiude

⛔ **Lette prima di scrivere, come `CLAUDE.md` prescrive; nessuna ha come chiusore questo piano.** Si
dichiarano perché chi esegue le sappia, non perché le tocchi.

| Voce | Dove vive | Chi la chiude |
|---|---|---|
| **X-1** (la CI anche su Windows) e **X-3** (`cargo audit`, `npm audit`), decise A il 2026-09-09 | la tabella delle voci senza numero AUD dell'[audit](../../audit-2026-08-27.md); voce 13 della §9 del 2 | la **parte 2** di questo piano (D13); **X-2** e **X-4** restano del proprietario |
| le voci 1–4 della §9 del 2 — `markdown-it`, gli attrezzi di prova, la regola `no-raw-text`, `gui/shell/` se vince Tauri | la §9 del disegno del 2 | la parte 2; `markdown-it` entra già nella Home finta (vincolo del carico, P-13) senza decidere nulla per la SPA |
| le voci 5, 6, 7, 8, 9, 10, 12, 14–18 della §9 del 2 | idem | il 3, il 10, il 4, la parte 2, il proprietario — come lì |
| le registrate della stella polare — l'ambito come progetto, «Automazione OS», il grafo del 6, Compatta, i due passi per invocazione, il richiamo in testa ad `AVVIO-CHAT.md`, quanto della stella polare si legga all'apertura | la tabella «Registrate, non prese» della stella polare | il 3, il 6, il 10, il proprietario |
| le tre voci del Traguardo 6 che aspettano il primo worker vero — 9, 26, 27 | la tabella del Traguardo 6 di [`porta-di-qualita.md`](../../porta-di-qualita.md) | il **12** |
| la tabella delle voci aperte del 2026-08-10 della §6 del compendio | §6 del compendio | i traguardi e il proprietario che quella tabella nomina; nessuna tocca la GUI |
| la **metà Linux** di M3 e M5, e la condizione di ribaltamento di ADR-0029 | ADR-0029, dopo il compito 8, come innesco scritto | il **primo Linux vero** |
| AUD-004, l'ADR del proprietario sulle skill | l'audit; voce 11 della §9 del 2 | il proprietario, in parallelo, prima del brainstorming del **13** |

---
## Compito 1: il protocollo di SP-8, congelato — e la riga SP-8 nella tabella degli spike

**Files:**
- Create: `spikes/gui-shell/PROTOCOLLO.md` (LF)
- Modify: `docs/roadmap.md` (**CRLF**) — la riga SP-8 nella tabella «Spike aperti», e la riga «Ultimo aggiornamento» (P-15)
- Read: la §4 della stella polare per intero; la §2 del disegno del 2; [`spikes/PROTOCOLLO.md`](../../../spikes/PROTOCOLLO.md) e [`spikes/gesti/PROTOCOLLO.md`](../../../spikes/gesti/PROTOCOLLO.md) per la forma

**Interfaces:**
- Produces: i criteri M1–M5, Q1–Q4 e le otto mosse che i compiti 6 e 7 misurano e giudicano; il nome `SP-8`

⛔ **Solo documenti in questo commit** (D2): il primo codice arriva col compito 2, così `git log` prova
che i criteri vengono prima della misura.

- [ ] **Passo 1: le misure prima**

```bash
ls spikes/gui-shell 2>&1
grep -n '^## SP-' spikes/RISULTATI.md
grep -n '^| SP-7 |' docs/roadmap.md | cut -c1-40
sed -n '6p' docs/roadmap.md | cut -c1-80
git ls-files --eol docs/roadmap.md
```

Atteso: `spikes/gui-shell` **non esiste** (domanda 4); l'ultima sezione di `RISULTATI.md` è `SP-7`
(P-12); **una** riga `| SP-7 |` in roadmap; la riga 6 comincia con `Ultimo aggiornamento:`; `i/lf w/crlf`.

- [ ] **Passo 2: il protocollo**

`spikes/gui-shell/PROTOCOLLO.md`, LF:

```markdown
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

Ricopiate dalla §4 della stella polare, che ne è la casa del **perché**; qui sono ciò che si congela.
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
```

- [ ] **Passo 3: `roadmap.md`, CRLF — la riga SP-8 e la riga della data**

Con `replace_unique.py`, due sostituzioni:

| Trova | Sostituisci con |
|---|---|
| la riga che comincia con `\| SP-7 \|`, **intera, presa dal file** | la stessa riga, poi a capo la riga qui sotto |
| la riga 6, `Ultimo aggiornamento: …`, **intera, presa dal file** | `Ultimo aggiornamento: **<data>**, con la riga **SP-8** nella tabella degli spike — il compito 1 del [piano della parte 1 del sotto-progetto 2](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md); la riga del piano nella tabella dei piani era arrivata il 2026-09-10, alla rilettura.` |

La riga nuova della tabella degli spike:

```
| SP-8 | il guscio della GUI — Tauri o Electron — con M1–M5 e Q1–Q4 di ADR-0029 sullo stesso frontend, e l'accettazione di `dockview` in otto mosse giudicate dal proprietario | [ADR-0029](adr/0029-guscio-della-gui.md); la SPA del sotto-progetto **2** — §2 del [disegno del 2](superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md), §4 della [stella polare](superpowers/specs/2026-09-07-direzione-gui-design.md) — coi criteri scritti prima in [spikes/gui-shell/PROTOCOLLO.md](../spikes/gui-shell/PROTOCOLLO.md) | ⬜ |
```

- [ ] **Passo 4: le prove, il commit, il push**

```bash
printf 'PROTOCOLLO CR='; tr -cd '\r' < spikes/gui-shell/PROTOCOLLO.md | wc -c
printf 'roadmap CR='; tr -cd '\r' < docs/roadmap.md | wc -c; printf '   righe='; wc -l < docs/roadmap.md
git ls-files --eol docs/roadmap.md
grep -c '^| SP-8 |' docs/roadmap.md
bash scripts/check-docs.sh; bash scripts/gate.sh
git status --porcelain
```

Atteso: `0` CR sul protocollo; CR = righe su roadmap; `i/lf w/crlf` invariato; `1`; `OK` — il controllo
dei link legge anche `spikes/gui-shell/PROTOCOLLO.md`, e i suoi sette link sono relativi a
`spikes/gui-shell/` (P-12) — e `GATE GREEN`; `git status` che nomina il protocollo, roadmap e **questo
piano** (la tabella della posizione), nient'altro.

```bash
git add spikes/gui-shell/PROTOCOLLO.md docs/roadmap.md docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git commit -m "guscio(compito 1, protocollo): i criteri di SP-8 — M1–M5, Q1–Q4 e le otto mosse — scritti prima della misura e congelati al primo commit di codice dello spike; la riga SP-8 nella tabella degli spike"
git push
```

#### Criterio di chiusura del compito 1

- [ ] il protocollo esiste, LF, coi sette link che `check-docs.sh` accetta; nessun file di codice in questo commit
- [ ] la riga SP-8 in roadmap, `⬜`; la riga della data riallineata nello stesso commit
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 2: la Home finta — Vite, Vue 3, `dockview-core`, e le mosse 1–7

**Files:**
- Create (LF): `spikes/gui-shell/app/package.json` · `vite.config.ts` · `index.html` · `popout.html` · `src/main.ts` · `src/style.css` · `src/vue-bridge.ts` · `src/home.ts` · `src/tiles/Tile.vue`
- Create da `npm install`: `spikes/gui-shell/app/package-lock.json`
- Modify: `.gitignore` (**CRLF**)
- Read: la §4 della stella polare; «Il modello della GUI»; P-1, P-2, P-4

**Interfaces:**
- Produces: la mappa `tiles` con i nomi `tile`, `chat`, `scene`, `hand` (il compito 3 sostituisce gli ultimi tre); le funzioni `log`, `floatPanel`, `togglePage`, `popout`, `moveActive`, `saveLayout`, `currentDnd`, `buildHome(dock, bar): DockviewApi`; gli `id` dei pannelli — `nucleus`, `strip`, `stato`, `permessi`, `attivita`, `costi`, `chat`, `scene`, `hand`, `passi`; la build in `app/dist/` con `index.html` e `popout.html`

⛔ **Questo è il primo commit di codice: da qui il protocollo è congelato** (vincolo 7).

- [ ] **Passo 1: le misure prima**

```bash
ls spikes/gui-shell/
node --version; npm --version
git ls-files --eol .gitignore
grep -n '/spikes/gesti/\*\.csv' .gitignore
```

Atteso: solo `PROTOCOLLO.md`; `v24.9.0` e `11.6.0` (P-9: versioni diverse si registrano nell'esito, non
si aggiustano); `i/lf w/crlf`; **una** riga per l'aggancio del Passo 3.

- [ ] **Passo 2: i file dell'app**

`spikes/gui-shell/app/package.json` — le versioni di P-8, appuntate (vincolo 8):

```json
{
  "name": "sp8-home",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "@tauri-apps/api": "2.11.1",
    "dockview-core": "8.2.0",
    "markdown-it": "15.0.1",
    "three": "0.185.1",
    "vue": "3.5.42"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "6.0.8",
    "vite": "8.2.2"
  }
}
```

`spikes/gui-shell/app/vite.config.ts`:

```ts
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

// SP-8 -- the fake Home. `base: './'` because the two shells load the build from a file:// or a
// tauri:// origin, where absolute asset paths break (P-4 of the plan). `popout.html` is a second page:
// dockview opens it for a popout group. `/stream` is proxied to the SP-7 relay, which serves
// Server-Sent Events without CORS headers (P-3): the page stays same-origin and the relay is untouched.
export default defineConfig({
  base: './',
  plugins: [vue()],
  build: {
    rollupOptions: {
      input: {
        index: fileURLToPath(new URL('index.html', import.meta.url)),
        popout: fileURLToPath(new URL('popout.html', import.meta.url)),
      },
    },
  },
  server: {
    proxy: { '/stream': { target: 'http://127.0.0.1:7878', changeOrigin: true } },
  },
});
```

`spikes/gui-shell/app/index.html`:

```html
<!doctype html>
<html lang="it">
  <head>
    <meta charset="utf-8" />
    <title>SP-8</title>
  </head>
  <body>
    <div id="bar"></div>
    <div id="dock"></div>
    <canvas id="hand"></canvas>
    <script type="module" src="./src/main.ts"></script>
  </body>
</html>
```

`spikes/gui-shell/app/popout.html` — la pagina vuota che `dockview` riempie:

```html
<!doctype html>
<html lang="it">
  <head>
    <meta charset="utf-8" />
    <title>SP-8 popout</title>
  </head>
  <body></body>
</html>
```

`spikes/gui-shell/app/src/style.css`:

```css
/* SP-8 -- the fake Home. Colours are conveniences, not the design system (the three moments of §6a). */
html, body { height: 100%; margin: 0; background: #0b0e14; color: #dde3ee; font: 14px system-ui, sans-serif; }
#bar { position: absolute; top: 0; left: 0; right: 0; height: 44px; display: flex; gap: 6px; align-items: center; padding: 0 8px; background: #161b26; box-sizing: border-box; overflow: hidden; }
#bar button, #bar select { font: inherit; padding: 4px 8px; background: #263043; color: inherit; border: 1px solid #3a4661; border-radius: 4px; cursor: pointer; }
#bar #log { margin-left: auto; font-family: ui-monospace, monospace; font-size: 12px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 55vw; }
#dock { position: absolute; top: 44px; left: 0; right: 0; bottom: 0; }
#hand { position: fixed; inset: 0; pointer-events: none; z-index: 9999; }
.bigtab { display: flex; align-items: center; gap: 6px; height: 40px; padding: 0 10px; font-weight: 600; cursor: grab; user-select: none; }
.bigtab-title { flex: 1; }
.bigtab button { font: inherit; width: 26px; height: 26px; background: #263043; color: inherit; border: 1px solid #3a4661; border-radius: 4px; cursor: pointer; }
.tile { height: 100%; box-sizing: border-box; overflow: auto; padding: 8px; }
.tile-who { opacity: .7; font-size: 12px; margin: 0 0 6px; }
.chat .msg { margin: 0 0 8px; }
.chat .msg.untrusted { border-left: 3px solid #e0a34b; padding-left: 8px; }
.chat .prov { opacity: .7; font-size: 12px; }
.scene { height: 100%; }
.scene canvas { display: block; width: 100%; height: 100%; }
```

`spikes/gui-shell/app/src/vue-bridge.ts` — il ponte fra Vue e i pannelli, scritto da noi (decisione 2
del coordinatore della stella polare):

```ts
import { createApp, type App, type Component } from 'vue';
import type { GroupPanelPartInitParameters, IContentRenderer } from 'dockview-core';

/** Mounts a Vue component as the content of one dockview panel -- the bridge between Vue and the
 * panels that the north star (decision 2 of the coordinator) says we write ourselves. One Vue app
 * per panel: a panel that dockview disposes unmounts its app and nothing leaks. */
export class VueContent implements IContentRenderer {
  readonly element = document.createElement('div');
  private app?: App;

  constructor(private readonly component: Component) {
    this.element.className = 'tile';
  }

  init(parameters: GroupPanelPartInitParameters): void {
    this.app = createApp(this.component, {
      title: parameters.title,
      api: parameters.api,
      containerApi: parameters.containerApi,
      params: parameters.params,
    });
    this.app.mount(this.element);
  }

  dispose(): void {
    this.app?.unmount();
    this.app = undefined;
  }
}
```

`spikes/gui-shell/app/src/tiles/Tile.vue` — **un** componente segnaposto per ogni modulo non costruito
(decisione 17 della stella polare):

```vue
<script setup lang="ts">
// A module that nobody has built yet says who fills it, in words (decision 17 of the north star).
defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: { who?: string; text?: string } }>();
</script>

<template>
  <div>
    <p class="tile-who">{{ params?.who ?? '' }}</p>
    <p>{{ params?.text ?? '' }}</p>
  </div>
</template>
```

`spikes/gui-shell/app/src/home.ts` — la Home e le mosse 1–7:

```ts
import {
  createDockview,
  themeAbyss,
  type AddPanelOptions,
  type DockviewApi,
  type DockviewDndStrategy,
  type IDockviewPanel,
  type ITabRenderer,
  type Position,
  type TabPartInitParameters,
} from 'dockview-core';
import { VueContent } from './vue-bridge';
import Tile from './tiles/Tile.vue';

/** The panel types of the fake Home. Task 3 replaces `chat`, `scene` and `hand` with live tiles;
 * the names stay, so a layout saved before task 3 still restores after it. */
export const tiles: Record<string, () => VueContent> = {
  tile: () => new VueContent(Tile),
  chat: () => new VueContent(Tile),
  scene: () => new VueContent(Tile),
  hand: () => new VueContent(Tile),
};

const GRID = 24; // move 2: a floating group snaps to this grid, through dockview's hook
const LAYOUT_KEY = 'sp8-home-layout'; // move 7: the JSON of the last "save"
const DND_KEY = 'sp8-dnd'; // Q4: the drag strategy, kept across reloads

let api: DockviewApi;
let logEl: HTMLElement | undefined;
const lines: string[] = [];

/** Writes into the bar: the last few actions, newest first. Also on the console, for the reviewer. */
export function log(text: string): void {
  lines.unshift(`${new Date().toLocaleTimeString()} ${text}`);
  if (lines.length > 6) lines.pop();
  if (logEl) logEl.textContent = lines.join('  |  ');
  console.log(text);
}

export function currentDnd(): DockviewDndStrategy {
  const v = localStorage.getItem(DND_KEY);
  return v === 'pointer' || v === 'html5' ? v : 'auto';
}

/** The big grab handle: a tall tab with the title and three commands. The tab element is what dockview
 * drags, so a big tab is a big grab (move 5). A click on a command must not start a drag. */
class BigTab implements ITabRenderer {
  readonly element = document.createElement('div');

  init(p: TabPartInitParameters): void {
    this.element.className = 'bigtab';
    const title = document.createElement('span');
    title.className = 'bigtab-title';
    title.textContent = p.title ?? p.api.id;
    this.element.append(title);
    const command = (label: string, hint: string, run: () => void) => {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = label;
      b.title = hint;
      b.addEventListener('pointerdown', (e) => e.stopPropagation());
      b.addEventListener('mousedown', (e) => e.stopPropagation());
      b.addEventListener('click', (e) => {
        e.stopPropagation();
        run();
      });
      this.element.append(b);
    };
    command('⧉', 'move 2: float this tile (drag it back into the grid to re-dock)', () => floatPanel(p.api.id));
    command('⤢', 'move 3: full page, or back', () => togglePage(p.api.id));
    command('↗', 'move 4: open in a separate window, from the command', () => popout(p.api.id));
  }
}

function panelOf(id: string): IDockviewPanel | undefined {
  const panel = api.getPanel(id);
  if (!panel) log(`no panel ${id}`);
  return panel;
}

export function floatPanel(id: string): void {
  const panel = panelOf(id);
  if (!panel) return;
  api.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
  log(`move 2: ${id} floats`);
}

export function togglePage(id: string): void {
  const panel = panelOf(id);
  if (!panel) return;
  if (panel.api.isMaximized()) {
    panel.api.exitMaximized();
    log(`move 3: ${id} back`);
  } else {
    panel.api.maximize();
    log(`move 3: ${id} full page`);
  }
}

export function popout(id: string): void {
  const panel = panelOf(id);
  if (!panel) return;
  api
    .addPopoutGroup(panel, { popoutUrl: 'popout.html' })
    .then((ok) => log(`move 4 / Q3: popout ${ok ? 'OPENED' : 'REFUSED'} for ${id}`))
    .catch((e) => log(`move 4 / Q3: popout threw ${String(e)}`));
}

/** Move 6: the active tile goes into the nearest group in that direction; with no neighbour it splits
 * its own group on that side. Geometry, not dockview's navigation API: it is what a keyboard user sees. */
export function moveActive(dir: 'left' | 'right' | 'up' | 'down'): void {
  const panel = api.activePanel;
  if (!panel) {
    log('move 6: no active tile');
    return;
  }
  const from = panel.group.element.getBoundingClientRect();
  const beyond = (r: DOMRect) =>
    dir === 'left' ? r.right <= from.left + 1
    : dir === 'right' ? r.left >= from.right - 1
    : dir === 'up' ? r.bottom <= from.top + 1
    : r.top >= from.bottom - 1;
  const gap = (r: DOMRect) =>
    dir === 'left' ? from.left - r.right
    : dir === 'right' ? r.left - from.right
    : dir === 'up' ? from.top - r.bottom
    : r.top - from.bottom;
  const target = api.groups
    .filter((g) => g !== panel.group && !g.locked)
    .map((g) => ({ g, r: g.element.getBoundingClientRect() }))
    .filter(({ r }) => beyond(r))
    .sort((a, b) => gap(a.r) - gap(b.r))[0];
  if (target) {
    panel.api.moveTo({ group: target.g, position: 'center' });
    log(`move 6: ${panel.id} -> group ${target.g.id}`);
  } else {
    const side: Position = dir === 'up' ? 'top' : dir === 'down' ? 'bottom' : dir;
    panel.api.moveTo({ group: panel.group, position: side });
    log(`move 6: ${panel.id} splits ${side}`);
  }
}

/** Move 7: save is a string; the comparison after a reload is a string equality -- a measure, not a judgement. */
export function saveLayout(): void {
  const json = JSON.stringify(api.toJSON());
  localStorage.setItem(LAYOUT_KEY, json);
  log(`move 7: saved ${json.length} bytes -- now reload`);
}

function firstDifference(a: string, b: string): number {
  const n = Math.min(a.length, b.length);
  for (let i = 0; i < n; i += 1) if (a[i] !== b[i]) return i;
  return n;
}

/** Move 1: the nucleus and the strip do not move, and take nothing dropped on them. */
function lock(id: string): void {
  const group = api.getPanel(id)?.group;
  if (!group) return;
  group.locked = true;
  group.header.hidden = true;
}

function add(options: AddPanelOptions): void {
  api.addPanel(options);
}

function defaultLayout(): void {
  add({ id: 'nucleus', component: 'tile', title: 'Nucleo', params: { who: 'arriva col 6', text: "niente ancora: la rete viva e l'anello degli artefatti sono del sotto-progetto 6" } });
  add({ id: 'strip', component: 'tile', title: 'Striscia', params: { who: 'sempre visibile', text: 'degrado: — · permessi: — (vivi nel 2 col core vero)' }, position: { referencePanel: 'nucleus', direction: 'below' }, minimumHeight: 56, maximumHeight: 56 });
  add({ id: 'stato', component: 'tile', title: 'Stato', params: { who: 'il 2', text: 'degrado, policy VRAM e budget, «protetto quanto il tuo account»' }, position: { referencePanel: 'nucleus', direction: 'left' } });
  add({ id: 'permessi', component: 'tile', title: 'Permessi', params: { who: 'il 2', text: 'le triple concesse, la richiesta in attesa' }, position: { referencePanel: 'stato', direction: 'below' } });
  add({ id: 'attivita', component: 'tile', title: 'Attività', params: { who: 'arriva col 3', text: "l'indice di tutte le run" }, position: { referencePanel: 'nucleus', direction: 'right' } });
  add({ id: 'costi', component: 'tile', title: 'Costi', params: { who: 'arriva col 3', text: 'costo corrente e distanza dal tetto' }, position: { referencePanel: 'attivita', direction: 'below' } });
  add({ id: 'chat', component: 'chat', title: 'Chat', params: { who: 'il 2 col core finto, il 3 vero', text: 'il flusso di token' }, position: { referencePanel: 'nucleus', direction: 'above' } });
  add({ id: 'scene', component: 'scene', title: 'Scena 3D', params: { who: 'il pilastro degli asset 3D', text: 'la scena three dello spike' }, position: { referencePanel: 'chat', direction: 'right' } });
  add({ id: 'hand', component: 'hand', title: 'Mano', params: { who: 'il 12', text: 'i 21 punti di SP-7' }, position: { referencePanel: 'scene', direction: 'right' } });
  add({ id: 'passi', component: 'tile', title: 'Passi', params: { who: 'il 2', text: 'le invocazioni del registro' }, position: { referencePanel: 'hand', direction: 'right' } });
}

function buildBar(bar: HTMLElement): void {
  const button = (label: string, run: () => void) => {
    const b = document.createElement('button');
    b.type = 'button';
    b.textContent = label;
    b.addEventListener('click', run);
    bar.append(b);
  };
  button('salva (mossa 7)', saveLayout);
  button('ricarica', () => location.reload());
  button('azzera', () => {
    localStorage.removeItem(LAYOUT_KEY);
    location.reload();
  });
  button('stacca (F)', () => api.activePanel && floatPanel(api.activePanel.id));
  button('pagina intera (M)', () => api.activePanel && togglePage(api.activePanel.id));
  button('finestra a parte (P)', () => api.activePanel && popout(api.activePanel.id));
  const select = document.createElement('select');
  for (const s of ['auto', 'pointer', 'html5'] as const) {
    const o = document.createElement('option');
    o.value = s;
    o.textContent = `dnd: ${s}`;
    o.selected = s === currentDnd();
    select.append(o);
  }
  select.addEventListener('change', () => {
    const s = select.value as DockviewDndStrategy;
    localStorage.setItem(DND_KEY, s);
    api.updateOptions({ dndStrategy: s });
    log(`Q4: dndStrategy = ${s}`);
  });
  bar.append(select);
  logEl = document.createElement('span');
  logEl.id = 'log';
  bar.append(logEl);
}

function onKey(e: KeyboardEvent): void {
  const tag = (e.target as HTMLElement | null)?.tagName;
  if (tag === 'INPUT' || tag === 'SELECT' || tag === 'TEXTAREA') return;
  if (e.ctrlKey && e.altKey) {
    const dir =
      e.key === 'ArrowLeft' ? 'left'
      : e.key === 'ArrowRight' ? 'right'
      : e.key === 'ArrowUp' ? 'up'
      : e.key === 'ArrowDown' ? 'down'
      : null;
    if (dir) {
      e.preventDefault();
      moveActive(dir);
    }
    return;
  }
  const active = api.activePanel;
  if (!active) return;
  if (e.key === 'f' || e.key === 'F') floatPanel(active.id);
  else if (e.key === 'm' || e.key === 'M') togglePage(active.id);
  else if (e.key === 'p' || e.key === 'P') popout(active.id);
}

export function buildHome(dock: HTMLElement, bar: HTMLElement): DockviewApi {
  api = createDockview(dock, {
    theme: themeAbyss,
    dndStrategy: currentDnd(),
    floatingGroupBounds: 'boundedWithinViewport',
    popoutUrl: 'popout.html',
    transformFloatingGroupDrag: ({ proposed }) => ({
      left: Math.round(proposed.left / GRID) * GRID,
      top: Math.round(proposed.top / GRID) * GRID,
    }),
    createComponent: ({ name }) => (tiles[name] ?? tiles.tile)(),
    createTabComponent: () => new BigTab(),
  });
  buildBar(bar);
  api.layout(dock.clientWidth, dock.clientHeight);
  const saved = localStorage.getItem(LAYOUT_KEY);
  if (saved) {
    api.fromJSON(JSON.parse(saved));
    const again = JSON.stringify(api.toJSON());
    log(
      again === saved
        ? `move 7: EQUAL, ${saved.length} bytes before and after the reload`
        : `move 7: DIFFERENT at byte ${firstDifference(saved, again)} (${saved.length} vs ${again.length} bytes)`,
    );
  } else {
    defaultLayout();
  }
  lock('nucleus');
  lock('strip');
  window.addEventListener('keydown', onKey);
  window.addEventListener('resize', () => api.layout(dock.clientWidth, dock.clientHeight));
  return api;
}
```

`spikes/gui-shell/app/src/main.ts` — la versione di questo compito; il compito 3 lo **riscrive**:

```ts
import './style.css';
import { buildHome, log } from './home';

buildHome(document.getElementById('dock')!, document.getElementById('bar')!);
log('SP-8 Home: moves 1-7 in this browser; the live tiles arrive with task 3');
```

- [ ] **Passo 3: `.gitignore`, CRLF — l'app**

Con `replace_unique.py`: Trova `/spikes/gesti/*.csv`; Sostituisci con le tre righe:

```
/spikes/gesti/*.csv
/spikes/gui-shell/app/node_modules/
/spikes/gui-shell/app/dist/
```

- [ ] **Passo 4: `npm install`, la build, e ciò che git vede**

```bash
(cd spikes/gui-shell/app && npm install 2>&1 | tail -3 && npm run build 2>&1 | tail -8)
ls spikes/gui-shell/app/dist/ spikes/gui-shell/app/dist/assets/ | head -12
git status --porcelain
for f in spikes/gui-shell/app/package.json spikes/gui-shell/app/vite.config.ts spikes/gui-shell/app/index.html spikes/gui-shell/app/popout.html spikes/gui-shell/app/src/main.ts spikes/gui-shell/app/src/style.css spikes/gui-shell/app/src/vue-bridge.ts spikes/gui-shell/app/src/home.ts spikes/gui-shell/app/src/tiles/Tile.vue spikes/gui-shell/app/package-lock.json; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
printf '.gitignore CR='; tr -cd '\r' < .gitignore | wc -c; printf '   righe='; wc -l < .gitignore; git ls-files --eol .gitignore
```

Atteso: `npm install` senza errori (gli avvisi di `npm audit` si **registrano** nell'esito, non si
sistemano: X-3 è della parte 2); `vite build` che termina con `✓ built` e nomina `dist/index.html`,
`dist/popout.html` e un file `assets/index-*.js`; `git status` che nomina i nove file dell'app, il lockfile,
`.gitignore` e questo piano — **non** `node_modules/` né `dist/`; `0` CR su tutti (se `npm` scrivesse il
lockfile CRLF, si riscrive LF con Python `newline=""` e lo si dice); `.gitignore` CR = righe, `i/lf w/crlf`.
⚠️ **Un rosso di `vite build` sul testo dettato è una voce d'errata** col testo corretto (regola 3 di «Come
si esegue»). ⚠️ Che le mosse **si vedano** — il nucleo bloccato, la presa grande, il galleggiante che scatta
a griglia, la pagina intera, «EQUAL» dopo salva e ricarica — lo verifica il **revisore** aprendo
`npm run dev` nel browser: un `build` verde prova che compila, non che funziona.

- [ ] **Passo 5: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
git add .gitignore spikes/gui-shell/app docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git status --porcelain
git commit -m "guscio(compito 2): la Home finta di SP-8 — Vite, Vue 3 e dockview-core, il ponte fra Vue e i pannelli, il nucleo e la striscia bloccati, la presa grande, i comandi delle mosse 1-7, salva/ricarica/confronta; lockfile committato, node_modules e dist ignorati"
git push
```

Atteso: `OK`, `GATE GREEN` — `spikes/` è fuori dal workspace e il cancello non lo compila, misurato dal
cancello stesso — e `git status --porcelain` dopo l'`add` che mostra solo righe `A`/`M` dei file della mappa.

#### Criterio di chiusura del compito 2

- [ ] `vite build` verde, `dist/index.html` e `dist/popout.html` esistono; `package-lock.json` committato; `node_modules/` e `dist/` ignorati
- [ ] nel browser (il revisore): i due gruppi bloccati non si muovono, una tessera galleggia e scatta a griglia, va a pagina intera e torna, `Ctrl+Alt+frecce` la sposta, «salva» poi «ricarica» scrive `EQUAL`
- [ ] nessun file di `crates/`, `scripts/`, `Cargo.lock` cambia; `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---
## Compito 3: le tre tessere vive — la chat dal ponte, la scena con fps e API, la mano come puntatore

**Files:**
- Create (LF): `spikes/gui-shell/app/src/bridge.ts` · `src/stats.ts` · `src/hand.ts` · `src/tiles/Chat.vue` · `src/tiles/Scene.vue` · `src/tiles/Hand.vue`
- Modify (LF): `spikes/gui-shell/app/src/home.ts` — tre righe della mappa `tiles` e tre `import`; `src/main.ts` — riscritto per intero
- Read: «Il modello della GUI» e la §4 della stella polare (la mossa 8); la §6a del 2 (le regole della chat); P-3, P-5, P-6, P-13; `spikes/gesti/relay/page.html` per la mano

**Interfaces:**
- Consumes: la mappa `tiles`, `log`, `currentDnd`, `buildHome` del compito 2
- Produces: `subscribe(): Promise<Source>` e l'`EventTarget` `wire` con eventi `message` (`detail: Wire`); `stats` e `startTitle()`; il **titolo della finestra** nella forma `SP-8 | fps=<n> min=<n> api=<API> scene=<w>x<h> | msgs=<n> lost=<n> holes=<n> p2max=<ms>ms p2mean=<ms>ms | src=<electron|tauri|browser> dnd=<auto|pointer|html5> | ua=<Chrome/…>`, che il compito 4 legge con `MainWindowTitle`; il contratto del ponte per i gusci — Electron: `window.harness.onLine(cb)` e `window.harness.chrome`; Tauri: l'evento `line` con la riga come `payload`, e il permesso di scrivere il titolo nativo (`core:window:allow-set-title`, compito 5)

- [ ] **Passo 1: le misure prima**

```bash
grep -n "chat: () => new VueContent(Tile)\|scene: () => new VueContent(Tile)\|hand: () => new VueContent(Tile)" spikes/gui-shell/app/src/home.ts
grep -n 'Access-Control' spikes/gesti/relay/src/main.rs; echo "exit $?"
ls spikes/gesti/.venv/Scripts/python.exe spikes/gesti/hand_landmarker.task spikes/gesti/relay/target/release/sp7-relay.exe 2>&1
```

Atteso: **tre** righe, una per nome; `exit 1` (P-3: nessuna intestazione CORS, il proxy resta necessario);
i tre file di SP-7 esistono — se il relay non fosse compilato, `(cd spikes/gesti/relay && cargo build --release)`
lo ricrea, e il suo `Cargo.lock` è ignorato.

- [ ] **Passo 2: il ponte, le misure della pagina, la mano**

`spikes/gui-shell/app/src/bridge.ts`:

```ts
import { stats } from './stats';

/** One line of the emitter, parsed: the shape of `Messaggio` in spikes/gui-ipc/src/lib.rs. */
export interface Wire {
  canale: 'Token' | 'Stato' | 'Metriche';
  seq: number;
  emesso_micros: number;
  carico: string;
}

export type Source = 'electron' | 'tauri' | 'browser';

declare global {
  interface Window {
    harness?: { onLine(cb: (line: string) => void): void; chrome?: string };
    __TAURI_INTERNALS__?: unknown;
  }
}

/** Every parsed message is dispatched here as a CustomEvent 'message' with the Wire in `detail`. */
export const wire = new EventTarget();

/** Where the lines come from -- the Electron preload, the Tauri event, or a local generator with the
 * cadence of spikes/gui-ipc (2000 messages in 10 s). The tiles never know which (decision D5). */
export async function subscribe(): Promise<Source> {
  const onLine = (line: string) => {
    let m: Wire;
    try {
      m = JSON.parse(line) as Wire;
    } catch {
      return; // the emitter writes JSON; a malformed line is nobody's message, and P1 counts holes, not garbage
    }
    stats.onWire(m);
    wire.dispatchEvent(new CustomEvent<Wire>('message', { detail: m }));
  };
  if (window.harness) {
    window.harness.onLine(onLine);
    return 'electron';
  }
  if ('__TAURI_INTERNALS__' in window) {
    const { listen } = await import('@tauri-apps/api/event');
    await listen<string>('line', (event) => onLine(event.payload));
    return 'tauri';
  }
  startGenerator(onLine);
  return 'browser';
}

/** spikes/gui-ipc/src/bin/core.rs, in JavaScript: 2000 messages in 10 s, then a pause, then again. */
function startGenerator(onLine: (line: string) => void): void {
  const TOTAL = 2000;
  const DURATION_MS = 10_000;
  const PAUSE_MS = 5_000;
  const period = DURATION_MS / TOTAL;
  let seq = 0;
  const tick = () => {
    const canale = seq % 10 === 0 ? 'Stato' : seq % 10 === 1 ? 'Metriche' : 'Token';
    onLine(
      JSON.stringify({
        canale,
        seq,
        emesso_micros: Math.round((performance.timeOrigin + performance.now()) * 1000),
        carico: 'lorem ipsum dolor sit amet',
      }),
    );
    seq += 1;
    setTimeout(tick, seq % TOTAL === 0 ? PAUSE_MS : period);
  };
  setTimeout(tick, period);
}
```

`spikes/gui-shell/app/src/stats.ts`:

```ts
import type { Wire } from './bridge';

/** What the page measures by itself, written into the window title once per second (decision D6):
 * fps and API from the scene tile, P1 (holes in the sequence) and P2 (emission -> reception) from
 * the wire. The measuring script reads the title from outside; no second channel exists. */
class Stats {
  msgs = 0;
  lost = 0;
  holes = 0;
  p2MaxMs = 0;
  p2SumMs = 0;
  fps = 0;
  fpsMin = 0;
  api = '-';
  scene = '-';
  source = '-';
  private lastSeq = -1;

  onWire(m: Wire): void {
    if (m.seq < this.lastSeq) this.lastSeq = -1; // a new run of the emitter starts at 0 again
    if (this.lastSeq >= 0 && m.seq > this.lastSeq + 1) {
      this.holes += 1;
      this.lost += m.seq - this.lastSeq - 1;
    }
    this.lastSeq = m.seq;
    this.msgs += 1;
    const delayMs = Date.now() - m.emesso_micros / 1000; // both are the wall clock of this machine
    if (delayMs > this.p2MaxMs) this.p2MaxMs = delayMs;
    this.p2SumMs += delayMs;
  }

  line(): string {
    const mean = this.msgs ? this.p2SumMs / this.msgs : 0;
    const dnd = localStorage.getItem('sp8-dnd') ?? 'auto';
    return (
      `SP-8 | fps=${this.fps.toFixed(0)} min=${this.fpsMin.toFixed(0)} api=${this.api} scene=${this.scene}` +
      ` | msgs=${this.msgs} lost=${this.lost} holes=${this.holes} p2max=${this.p2MaxMs.toFixed(1)}ms p2mean=${mean.toFixed(2)}ms` +
      ` | src=${this.source} dnd=${dnd} | ua=${chromeToken()}`
    );
  }
}

function chromeToken(): string {
  const m = navigator.userAgent.match(/(?:Chrome|Edg)\/[\d.]+/g);
  return m ? m.join(',') : navigator.userAgent.slice(0, 40);
}

export const stats = new Stats();

/** The title: `document.title` everywhere; under Tauri also the native window title, which the webview
 * does not update by itself -- needs `core:window:allow-set-title` in the capability (task 5). */
export async function startTitle(): Promise<void> {
  let native: ((title: string) => Promise<void>) | undefined;
  if ('__TAURI_INTERNALS__' in window) {
    try {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      const w = getCurrentWindow();
      native = (title) => w.setTitle(title);
    } catch (e) {
      console.warn('native title not available', e);
    }
  }
  const write = () => {
    const line = stats.line();
    document.title = line;
    native?.(line).catch((e) => console.warn('setTitle failed', e));
  };
  write();
  setInterval(write, 1000);
}
```

`spikes/gui-shell/app/src/hand.ts`:

```ts
import { ref } from 'vue';
import { log } from './home';

/** SP-7's hand as a pointer (move 8): the 21 points of each hand drawn over the whole page, mirrored as in
 * spikes/gesti/relay/page.html, and the PINCH of the first hand -- thumb tip 4 and index tip 8 closer than
 * PINCH pixels of the 640x480 frame -- translated into pointer events at the index tip: pointerdown when it
 * closes, pointermove while it stays closed, pointerup when it opens. `pointerType: 'mouse'` so that dockview
 * does not wait the long press it reserves for touch. The relay is reached through the vite proxy at
 * `/stream` (P-3), so this works only in the dev browser. Nothing here is product code. */
export const handStatus = ref('not started');

const W = 640;
const H = 480;
const PINCH = 40;
const POINTER_ID = 7;
const CHAINS = [[0, 1, 2, 3, 4], [0, 5, 6, 7, 8], [5, 9, 10, 11, 12], [9, 13, 14, 15, 16], [13, 17, 18, 19, 20], [0, 17]];

type Point = [number, number];

export function startHand(canvas: HTMLCanvasElement): void {
  if (location.protocol !== 'http:') {
    handStatus.value = 'only in the dev browser: the relay is reached through the vite proxy';
    return;
  }
  const ctx = canvas.getContext('2d')!;
  const fit = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
  };
  fit();
  window.addEventListener('resize', fit);
  let pinched = false;
  let last: Point = [0, 0];

  const toScreen = ([x, y]: Point): Point => [((W - x) / W) * window.innerWidth, (y / H) * window.innerHeight];

  const fire = (type: 'pointerdown' | 'pointermove' | 'pointerup', [x, y]: Point) => {
    last = [x, y];
    const target = document.elementFromPoint(x, y) ?? document.body;
    target.dispatchEvent(
      new PointerEvent(type, {
        bubbles: true,
        cancelable: true,
        composed: true,
        clientX: x,
        clientY: y,
        screenX: x,
        screenY: y,
        pointerId: POINTER_ID,
        pointerType: 'mouse',
        isPrimary: true,
        button: type === 'pointermove' ? -1 : 0,
        buttons: type === 'pointerup' ? 0 : 1,
      }),
    );
    if (type !== 'pointermove') {
      const cls = String(target.className).split(' ')[0];
      log(`move 8: ${type} at ${x.toFixed(0)},${y.toFixed(0)} on ${target.tagName.toLowerCase()}.${cls}`);
    }
  };

  const draw = (hands: Point[][], cursor: Point | null) => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.strokeStyle = '#8cf';
    ctx.fillStyle = '#fff';
    ctx.lineWidth = 2;
    for (const hand of hands) {
      for (const chain of CHAINS) {
        ctx.beginPath();
        chain.forEach((i, k) => (k ? ctx.lineTo(hand[i][0], hand[i][1]) : ctx.moveTo(hand[i][0], hand[i][1])));
        ctx.stroke();
      }
      for (const [x, y] of hand) {
        ctx.beginPath();
        ctx.arc(x, y, 4, 0, Math.PI * 2);
        ctx.fill();
      }
    }
    if (cursor) {
      ctx.fillStyle = pinched ? '#f66' : '#6f6';
      ctx.beginPath();
      ctx.arc(cursor[0], cursor[1], 9, 0, Math.PI * 2);
      ctx.fill();
    }
  };

  const source = new EventSource('/stream');
  source.onopen = () => {
    handStatus.value = 'relay connected';
    log('move 8: relay connected');
  };
  source.onerror = () => {
    handStatus.value = 'relay not reachable: start spikes/gesti/relay first';
  };
  source.onmessage = (e) => {
    const raw = (JSON.parse(e.data) as { hands: Point[][] }).hands;
    const hands = raw.map((h) => h.map(toScreen));
    if (!raw.length) {
      if (pinched) {
        pinched = false;
        fire('pointerup', last);
      }
      draw(hands, null);
      return;
    }
    const [tx, ty] = raw[0][4];
    const [ix, iy] = raw[0][8];
    const closed = Math.hypot(tx - ix, ty - iy) <= PINCH; // in frame pixels, as in SP-7
    const tip = toScreen([ix, iy]);
    if (closed && !pinched) {
      pinched = true;
      fire('pointerdown', tip);
    } else if (closed) {
      fire('pointermove', tip);
    } else if (pinched) {
      pinched = false;
      fire('pointerup', tip);
    }
    draw(hands, tip);
  };
}
```

- [ ] **Passo 3: le tre tessere**

`spikes/gui-shell/app/src/tiles/Chat.vue`:

```vue
<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import MarkdownIt from 'markdown-it';
import { wire, type Wire } from '../bridge';

// The chat tile: renders the stream of `Token` messages as markdown, ONE render per animation frame at
// most, and only of the message being streamed; finished messages are frozen as rendered HTML. The
// preset 'default' has html:false and linkify:false (decision 51 of the north star): raw HTML in the text
// is escaped and links do not open by themselves. The text of a model is untrusted, and the tile says so.
defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: unknown }>();

const md = new MarkdownIt();
const FREEZE_AT = 4000; // characters: a message this long is frozen and a new one begins (the tile keeps no state, G1)
const KEEP = 20; // frozen messages kept in the DOM

const frozen = ref<string[]>([]);
const current = ref('');
let text = '';
let raf = 0;

function onMessage(e: Event): void {
  const m = (e as CustomEvent<Wire>).detail;
  if (m.canale !== 'Token') return;
  text += (text ? ' ' : '') + m.carico;
  if (text.length > FREEZE_AT) {
    frozen.value.push(md.render(text));
    if (frozen.value.length > KEEP) frozen.value.shift();
    text = '';
  }
  if (!raf) {
    raf = requestAnimationFrame(() => {
      raf = 0;
      current.value = md.render(text);
    });
  }
}

onMounted(() => wire.addEventListener('message', onMessage));
onUnmounted(() => {
  wire.removeEventListener('message', onMessage);
  if (raf) cancelAnimationFrame(raf);
});
</script>

<template>
  <div class="chat">
    <p class="prov">provenienza: non fidato (un modello) — reso come testo e codice, mai come HTML</p>
    <div v-for="(html, i) in frozen" :key="i" class="msg untrusted" v-html="html"></div>
    <div class="msg untrusted" v-html="current"></div>
  </div>
</template>
```

`spikes/gui-shell/app/src/tiles/Scene.vue`:

```vue
<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { AmbientLight, BoxGeometry, Color, DirectionalLight, InstancedMesh, Matrix4, MeshStandardMaterial, PerspectiveCamera, Scene } from 'three';
import { WebGPURenderer } from 'three/webgpu';
import { stats } from '../stats';

// The 3D tile: a field of instanced cubes under two lights, rendered by WebGPURenderer, which falls back
// to WebGL2 by itself (P-6). M3: fps counted per second, mean and min over the last 30 seconds; the API
// read from the backend the renderer actually obtained, with the adapter or the unmasked renderer string.
defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: unknown }>();

const host = ref<HTMLDivElement>();
let stopped = false;
let observer: ResizeObserver | undefined;

async function describeApi(renderer: WebGPURenderer): Promise<string> {
  const backend = (renderer as unknown as { backend?: { isWebGPUBackend?: boolean; isWebGLBackend?: boolean } }).backend;
  if (backend?.isWebGPUBackend) {
    try {
      const gpu = (navigator as unknown as { gpu?: { requestAdapter(): Promise<unknown> } }).gpu;
      const adapter = await gpu?.requestAdapter();
      const info = (adapter as { info?: { vendor?: string; architecture?: string } } | undefined)?.info;
      return `WebGPU:${info?.vendor ?? '?'}/${info?.architecture ?? '?'}`;
    } catch {
      return 'WebGPU:?';
    }
  }
  if (backend?.isWebGLBackend) {
    const gl = document.createElement('canvas').getContext('webgl2');
    const dbg = gl?.getExtension('WEBGL_debug_renderer_info');
    if (gl && dbg) {
      const name = String(gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL)).replace(/\s+/g, '_').slice(0, 40);
      return `WebGL2:${name}`;
    }
    return 'WebGL2:?';
  }
  return 'unknown';
}

onMounted(async () => {
  const el = host.value!;
  const renderer = new WebGPURenderer({ antialias: true });
  await renderer.init();
  stats.api = await describeApi(renderer);
  el.append(renderer.domElement);

  const scene = new Scene();
  scene.background = new Color(0x0b0e14);
  const camera = new PerspectiveCamera(60, 1, 0.1, 200);
  camera.position.set(0, 0, 40);
  scene.add(new AmbientLight(0xffffff, 0.4));
  const sun = new DirectionalLight(0xffffff, 1.2);
  sun.position.set(10, 20, 30);
  scene.add(sun);
  const COUNT = 4000;
  const cubes = new InstancedMesh(new BoxGeometry(1, 1, 1), new MeshStandardMaterial({ color: 0x4f8fdb, roughness: 0.4, metalness: 0.2 }), COUNT);
  const m = new Matrix4();
  for (let i = 0; i < COUNT; i += 1) {
    m.makeTranslation((Math.random() - 0.5) * 60, (Math.random() - 0.5) * 40, (Math.random() - 0.5) * 40);
    cubes.setMatrixAt(i, m);
  }
  scene.add(cubes);

  const fit = () => {
    const w = Math.max(1, el.clientWidth);
    const h = Math.max(1, el.clientHeight);
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    stats.scene = `${w}x${h}`;
  };
  fit();
  observer = new ResizeObserver(fit);
  observer.observe(el);

  let frames = 0;
  let windowStart = performance.now();
  const seconds: number[] = [];
  const loop = () => {
    if (stopped) return;
    cubes.rotation.y += 0.004;
    cubes.rotation.x += 0.002;
    renderer.render(scene, camera);
    frames += 1;
    const now = performance.now();
    if (now - windowStart >= 1000) {
      seconds.push((frames * 1000) / (now - windowStart));
      if (seconds.length > 30) seconds.shift();
      stats.fps = seconds.reduce((a, b) => a + b, 0) / seconds.length;
      stats.fpsMin = Math.min(...seconds);
      frames = 0;
      windowStart = now;
    }
    requestAnimationFrame(loop);
  };
  requestAnimationFrame(loop);
});

onUnmounted(() => {
  stopped = true;
  observer?.disconnect();
});
</script>

<template>
  <div ref="host" class="scene"></div>
</template>
```

`spikes/gui-shell/app/src/tiles/Hand.vue`:

```vue
<script setup lang="ts">
import { handStatus } from '../hand';

defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: unknown }>();
</script>

<template>
  <div>
    <p class="tile-who">il 12 — la mano di SP-7 come puntatore (mossa 8)</p>
    <p>la mano si disegna sopra tutta la pagina; la pinza afferra ciò che sta sotto la punta dell'indice</p>
    <p>stato: {{ handStatus }}</p>
  </div>
</template>
```

- [ ] **Passo 4: `home.ts` e `main.ts`**

In `spikes/gui-shell/app/src/home.ts`, con `replace_unique.py` (il file è LF), due sostituzioni:

| Trova | Sostituisci con |
|---|---|
| `import Tile from './tiles/Tile.vue';` | `import Tile from './tiles/Tile.vue';` a capo `import Chat from './tiles/Chat.vue';` a capo `import Scene from './tiles/Scene.vue';` a capo `import Hand from './tiles/Hand.vue';` |
| le tre righe `  chat: () => new VueContent(Tile),` · `  scene: () => new VueContent(Tile),` · `  hand: () => new VueContent(Tile),` | `  chat: () => new VueContent(Chat),` · `  scene: () => new VueContent(Scene),` · `  hand: () => new VueContent(Hand),` |

`spikes/gui-shell/app/src/main.ts`, **riscritto per intero**:

```ts
import './style.css';
import { buildHome, log } from './home';
import { subscribe } from './bridge';
import { startTitle, stats } from './stats';
import { startHand } from './hand';

buildHome(document.getElementById('dock')!, document.getElementById('bar')!);
startTitle();
subscribe().then((source) => {
  stats.source = source;
  log(`stream source: ${source}`);
});
startHand(document.getElementById('hand') as HTMLCanvasElement);
```

- [ ] **Passo 5: la build, e ciò che si vede**

```bash
(cd spikes/gui-shell/app && npm run build 2>&1 | tail -8)
grep -c "new VueContent(Tile)" spikes/gui-shell/app/src/home.ts
for f in spikes/gui-shell/app/src/bridge.ts spikes/gui-shell/app/src/stats.ts spikes/gui-shell/app/src/hand.ts spikes/gui-shell/app/src/tiles/Chat.vue spikes/gui-shell/app/src/tiles/Scene.vue spikes/gui-shell/app/src/tiles/Hand.vue spikes/gui-shell/app/src/main.ts spikes/gui-shell/app/src/home.ts; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
git status --porcelain
```

Atteso: `✓ built`, con un chunk in più per `@tauri-apps/api` (l'`import()` dinamico) e nessun errore; `1` —
resta il solo `tile:`; `0` CR ovunque; `git status` che nomina i sei file nuovi, `home.ts`, `main.ts` e
questo piano. ⚠️ Nel browser (`npm run dev`, il **revisore**): il titolo della scheda cambia ogni secondo e
`msgs=` cresce, `src=browser`, `api=WebGPU:…` su questa macchina (P-6: se rende `WebGL2:…` è una **misura**,
si registra); la chat scorre con la provenienza scritta; la scena gira; la tessera Mano dice *«relay not
reachable»* finché il relay non è in ascolto. La mossa 8 stessa è del **compito 7**.

- [ ] **Passo 6: le prove, il commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
git add spikes/gui-shell/app docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git commit -m "guscio(compito 3): le tre tessere vive di SP-8 — il ponte (preload di Electron, evento di Tauri, o il generatore locale con la cadenza dell'emettitore), la chat che rende markdown token per token con la provenienza, la scena three con fps e API ottenuta nel titolo, la mano di SP-7 tradotta in eventi del puntatore"
git push
```

#### Criterio di chiusura del compito 3

- [ ] `vite build` verde; il titolo porta i **dodici** campi `chiave=` dell'interfaccia — `fps min api scene`, `msgs lost holes p2max p2mean`, `src dnd`, `ua`; la chat, la scena e la mano vivono nel browser (il revisore)
- [ ] `spikes/gesti/` intatto: `git diff --name-only 5ad4634..HEAD -- spikes/gesti/` vuoto
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---
## Compito 4: il guscio Electron, e lo script che misura l'albero di processi

**Files:**
- Create (LF): `spikes/gui-shell/electron/package.json` · `main.js` · `preload.js`; `spikes/gui-shell/measure/tree.ps1` · `size.ps1`
- Create da `npm install`: `spikes/gui-shell/electron/package-lock.json`
- Modify: `.gitignore` (**CRLF**)
- Read: la §2 del 2 (M1–M5 e le trappole); P-5, P-9, P-13; `spikes/gui-ipc/src/bin/gui.rs` per il client della pipe

**Interfaces:**
- Consumes: la build `app/dist/`; il contratto del ponte del compito 3 (`window.harness.onLine`, `window.harness.chrome`); il nome della pipe `gui-ipc-spike` di `spikes/gui-ipc/src/lib.rs`
- Produces: `tree.ps1` con i parametri `-Exe`, `-ArgumentList`, `-WorkingDirectory`, `-AttachPid`, `-Seconds`, `-IntervalMs`, `-Emitter`, `-EmitterAt`, `-Csv`, `-KeepRunning` e le righe di uscita `rest: …`, `stream: …`, `last title: …`; `size.ps1 -Path`; l'installatore in `electron/out/`

- [ ] **Passo 1: le misure prima**

```bash
ls spikes/gui-shell/app/dist/index.html
grep -n '/spikes/gui-shell/app/dist/' .gitignore
(cd spikes/gui-ipc && cargo build --release 2>&1 | tail -2) && ls spikes/gui-ipc/target/release/core.exe
```

Atteso: la build esiste (altrimenti `(cd spikes/gui-shell/app && npm run build)`); **una** riga per
l'aggancio del Passo 3; l'emettitore compila — il suo `Cargo.lock` è ignorato e risolve `interprocess`
quel giorno: la versione va nell'esito.

- [ ] **Passo 2: il guscio**

`spikes/gui-shell/electron/package.json`:

```json
{
  "name": "sp8-electron",
  "private": true,
  "version": "0.0.0",
  "description": "SP-8: the Electron shell of the fake Home",
  "author": "harness",
  "main": "main.js",
  "scripts": {
    "sync": "node -e \"require('node:fs').cpSync('../app/dist', 'dist', { recursive: true, force: true })\"",
    "start": "npm run sync && electron .",
    "dist": "npm run sync && electron-builder --win nsis"
  },
  "devDependencies": {
    "electron": "44.3.0",
    "electron-builder": "26.15.3"
  },
  "build": {
    "appId": "harness.sp8.electron",
    "productName": "sp8-electron",
    "directories": { "output": "out" },
    "files": ["main.js", "preload.js", "dist/**"],
    "win": { "target": "nsis" },
    "nsis": { "oneClick": true, "perMachine": false, "allowToChangeInstallationDirectory": false }
  }
}
```

`spikes/gui-shell/electron/main.js`:

```js
// SP-8 -- the Electron shell of the fake Home. The main process reads the emitter's named pipe with
// Node's `net` and forwards every line to the renderer; the preload exposes it as `window.harness`.
// Q1: with this shell, decoding `bincode` would live HERE, in Node (`bincode-ts`, M-11 of ADR-0037).
// Nothing here is product code.
const { app, BrowserWindow } = require('electron');
const net = require('node:net');
const path = require('node:path');

const PIPE = '\\\\.\\pipe\\gui-ipc-spike'; // interprocess prepends \\.\pipe\ to a namespaced name (P-5)
const RETRY_MS = 2000; // the emitter may not be listening yet: retry, like the "riprova" strip of the real GUI

let win;

function connect() {
  const socket = net.connect({ path: PIPE });
  let buffer = '';
  socket.setEncoding('utf8');
  socket.on('connect', () => console.log('core connected'));
  socket.on('data', (chunk) => {
    buffer += chunk;
    let cut;
    while ((cut = buffer.indexOf('\n')) >= 0) {
      const line = buffer.slice(0, cut);
      buffer = buffer.slice(cut + 1);
      if (win && !win.isDestroyed()) win.webContents.send('line', line);
    }
  });
  socket.on('error', () => {}); // ENOENT while the emitter is not there: 'close' below retries
  socket.on('close', () => setTimeout(connect, RETRY_MS));
}

app.whenReady().then(() => {
  win = new BrowserWindow({
    width: 1400,
    height: 900,
    webPreferences: { preload: path.join(__dirname, 'preload.js') },
  });
  // Q3: a popout is a window.open from the page; the shell allows it and Electron opens a child window.
  win.webContents.setWindowOpenHandler(() => ({ action: 'allow' }));
  win.webContents.once('did-finish-load', connect); // no line is sent before the page can receive it (P1)
  win.loadFile(path.join(__dirname, 'dist', 'index.html'));
});

app.on('window-all-closed', () => app.quit());
```

`spikes/gui-shell/electron/preload.js`:

```js
// SP-8 -- the preload: the only door between the page and the shell. The page sees `window.harness`.
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('harness', {
  onLine: (callback) => ipcRenderer.on('line', (_event, line) => callback(line)),
  chrome: process.versions.chrome, // Q2: the Chromium this shell packs
});
```

- [ ] **Passo 3: `.gitignore`, CRLF — il guscio Electron**

Con `replace_unique.py`: Trova `/spikes/gui-shell/app/dist/`; Sostituisci con:

```
/spikes/gui-shell/app/dist/
/spikes/gui-shell/electron/node_modules/
/spikes/gui-shell/electron/dist/
/spikes/gui-shell/electron/out/
```

- [ ] **Passo 4: gli script di misura (D7)**

`spikes/gui-shell/measure/tree.ps1`:

```powershell
<#
SP-8 -- RSS, CPU and VRAM of a whole PROCESS TREE while the shell runs (M1, M4, M5), and the page's own
numbers read from the window title (M3, P1, P2). PowerShell 5.1.

Usage:
  powershell -NoProfile -ExecutionPolicy Bypass -File tree.ps1 -Exe <shell exe> [-ArgumentList "<args>"] [-WorkingDirectory <dir>]
      [-Seconds 40] [-IntervalMs 250] [-Emitter <core.exe>] [-EmitterAt 15] [-Csv <outside the repo>] [-KeepRunning]
  ... or -AttachPid <pid> instead of -Exe, to sample a shell that is already running (task 7).

The tree is rebuilt at every sample from Win32_Process.ParentProcessId, because WebView2 lives OUTSIDE the
Tauri PID (M1). RSS is the sum of WorkingSet64. CPU is the delta of the summed TotalProcessorTime over the
interval, as a percentage of ONE core: P3 is "< 25% of one core". VRAM is the sum of
'\GPU Process Memory(*)\Dedicated Usage' over the tree's PIDs, read once per second (the counter is slow).
The emitter, if given, is started at -EmitterAt seconds: before it the samples are the REST, after it the
STREAM (the shells retry the pipe every two seconds, then 2000 messages take ten seconds, then the emitter
exits: the STREAM phase includes that wait and that tail, and the peak is what P3 judges). At the end the
tree is stopped unless -KeepRunning or -AttachPid.
#>
param(
  [string]$Exe = "",
  [string]$ArgumentList = "",
  [string]$WorkingDirectory = ".",
  [int]$AttachPid = 0,
  [int]$Seconds = 40,
  [int]$IntervalMs = 250,
  [string]$Emitter = "",
  [int]$EmitterAt = 15,
  [string]$Csv = "",
  [switch]$KeepRunning
)

function Get-Tree([int]$root) {
  $all = Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId
  $set = New-Object 'System.Collections.Generic.HashSet[int]'
  [void]$set.Add($root)
  do {
    $grew = $false
    foreach ($p in $all) {
      if ($set.Contains([int]$p.ParentProcessId) -and -not $set.Contains([int]$p.ProcessId)) {
        [void]$set.Add([int]$p.ProcessId)
        $grew = $true
      }
    }
  } while ($grew)
  return @($set)
}

if ($AttachPid -gt 0) {
  $root = $AttachPid
  Write-Output ("attached to pid {0}" -f $root)
} elseif ($Exe -ne "") {
  $startArgs = @{ FilePath = $Exe; WorkingDirectory = $WorkingDirectory; PassThru = $true }
  if ($ArgumentList -ne "") { $startArgs.ArgumentList = $ArgumentList }
  $shell = Start-Process @startArgs
  $root = $shell.Id
  Write-Output ("shell started: pid {0} ({1})" -f $root, $Exe)
} else {
  throw "give -Exe or -AttachPid"
}

$t0 = Get-Date
$end = $t0.AddSeconds($Seconds)
$emitterStarted = $false
$rows = @()
$prevCpu = $null
$prevT = $t0
$vram = 0
$lastVram = $t0.AddSeconds(-10)
Start-Sleep -Milliseconds 500

while ((Get-Date) -lt $end) {
  $now = Get-Date
  if (($Emitter -ne "") -and -not $emitterStarted -and (($now - $t0).TotalSeconds -ge $EmitterAt)) {
    Start-Process -FilePath $Emitter | Out-Null
    $emitterStarted = $true
    Write-Output ("emitter started at {0:N1} s" -f ($now - $t0).TotalSeconds)
  }
  $pids = Get-Tree $root
  $ps = @(Get-Process -Id $pids -ErrorAction SilentlyContinue)
  $rss = ($ps | Measure-Object WorkingSet64 -Sum).Sum
  $cpu = ($ps | ForEach-Object { $_.TotalProcessorTime.TotalMilliseconds } | Measure-Object -Sum).Sum
  $pct = 0
  if ($null -ne $prevCpu) { $pct = ($cpu - $prevCpu) / ($now - $prevT).TotalMilliseconds * 100 }
  if (($now - $lastVram).TotalMilliseconds -ge 1000) {
    $samples = (Get-Counter '\GPU Process Memory(*)\Dedicated Usage' -ErrorAction SilentlyContinue).CounterSamples
    $mine = $samples | Where-Object { ($_.InstanceName -match '^pid_(\d+)_') -and ($pids -contains [int]$Matches[1]) }
    $vram = ($mine | Measure-Object CookedValue -Sum).Sum
    if ($null -eq $vram) { $vram = 0 }
    $lastVram = $now
  }
  $title = ($ps | Where-Object { $_.MainWindowTitle } | Select-Object -First 1).MainWindowTitle
  $rows += [pscustomobject]@{
    t_s = [math]::Round(($now - $t0).TotalSeconds, 2)
    phase = $(if ($emitterStarted) { "stream" } else { "rest" })
    procs = $pids.Count
    rss_mb = [math]::Round($rss / 1MB, 1)
    cpu_pct = [math]::Round($pct, 1)
    vram_mb = [math]::Round($vram / 1MB, 1)
    title = $title
  }
  $prevCpu = $cpu
  $prevT = $now
  Start-Sleep -Milliseconds $IntervalMs
}

$rows | Format-Table t_s, phase, procs, rss_mb, cpu_pct, vram_mb -AutoSize | Out-String -Width 200 | Write-Output
foreach ($phase in @("rest", "stream")) {
  $r = @($rows | Where-Object { $_.phase -eq $phase })
  if ($r.Count -eq 0) { continue }
  Write-Output ("{0}: samples {1} | rss_mb mean {2} max {3} | cpu_pct mean {4} max {5} | vram_mb max {6}" -f $phase, $r.Count,
    [math]::Round(($r.rss_mb | Measure-Object -Average).Average, 1), ($r.rss_mb | Measure-Object -Maximum).Maximum,
    [math]::Round(($r.cpu_pct | Measure-Object -Average).Average, 1), ($r.cpu_pct | Measure-Object -Maximum).Maximum,
    ($r.vram_mb | Measure-Object -Maximum).Maximum)
}
Write-Output ("last title: {0}" -f $rows[-1].title)
if ($Csv -ne "") { $rows | Export-Csv -NoTypeInformation -Path $Csv }
if (-not $KeepRunning -and $AttachPid -eq 0) { Stop-Process -Id (Get-Tree $root) -Force -ErrorAction SilentlyContinue }
```

`spikes/gui-shell/measure/size.ps1`:

```powershell
<# SP-8 -- M2: bytes and files of an installed folder, or the size of an installer. Usage: size.ps1 -Path <folder or file> #>
param([Parameter(Mandatory = $true)][string]$Path)
$item = Get-Item -LiteralPath $Path
if ($item.PSIsContainer) {
  $files = @(Get-ChildItem -LiteralPath $Path -Recurse -File -Force)
  $sum = ($files | Measure-Object Length -Sum).Sum
  Write-Output ("{0}: {1} files, {2} bytes, {3} MB" -f $Path, $files.Count, $sum, [math]::Round($sum / 1MB, 1))
} else {
  Write-Output ("{0}: {1} bytes, {2} MB" -f $Path, $item.Length, [math]::Round($item.Length / 1MB, 1))
}
```

- [ ] **Passo 5: `npm install`, la corsa di prova con l'emettitore, l'installatore**

```bash
(cd spikes/gui-shell/electron && npm install 2>&1 | tail -3 && npm run sync)
ROOT=$(cygpath -w "$PWD")
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$ROOT\spikes\gui-shell\electron\node_modules\electron\dist\electron.exe" -ArgumentList . -WorkingDirectory "$ROOT\spikes\gui-shell\electron" -Seconds 30 -EmitterAt 8 -Emitter "$ROOT\spikes\gui-ipc\target\release\core.exe" | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'
(cd spikes/gui-shell/electron && npm run dist 2>&1 | tail -6) && ls spikes/gui-shell/electron/out/*.exe
```

Atteso: dalla corsa di prova, `emitter started at ~8 s`, due righe `rest:` e `stream:`, e `last title:` che
porta `msgs=2000 lost=0 holes=0` e `src=electron` — la prova che il guscio legge la pipe e la passa alla
pagina (P1 e P4 di SP-5/SP-6 riviste col guscio vero); se `msgs` fosse minore di 2000 o `holes` maggiore di
zero **si registra**, è una misura; da `npm run dist`, un file `sp8-electron Setup 0.0.0.exe` in `out/`.
⚠️ La finestra si apre e si chiude da sola: `tree.ps1` ferma l'albero alla fine.

- [ ] **Passo 6: le prove, il commit, il push**

```bash
for f in spikes/gui-shell/electron/package.json spikes/gui-shell/electron/main.js spikes/gui-shell/electron/preload.js spikes/gui-shell/electron/package-lock.json spikes/gui-shell/measure/tree.ps1 spikes/gui-shell/measure/size.ps1; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
printf '.gitignore CR='; tr -cd '\r' < .gitignore | wc -c; printf '   righe='; wc -l < .gitignore; git ls-files --eol .gitignore
git status --porcelain
bash scripts/check-docs.sh; bash scripts/gate.sh
git add .gitignore spikes/gui-shell/electron spikes/gui-shell/measure docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git commit -m "guscio(compito 4): il guscio Electron di SP-8 — la pipe letta con net nel processo principale e passata alla pagina dal preload, la finestra figlia ammessa per il popout, electron-builder per l'installatore NSIS; tree.ps1 che misura RSS, CPU e VRAM dell'albero di processi e legge il titolo; size.ps1 per M2"
git push
```

Atteso: `0` CR sui sei file (`⚠️` PowerShell legge anche LF; se un editor scrivesse CRLF si riscrive LF);
`.gitignore` CR = righe; `git status` che nomina i file della mappa e **non** `node_modules/`, `dist/`,
`out/`; `OK`, `GATE GREEN`.

#### Criterio di chiusura del compito 4

- [ ] la corsa di prova rende `msgs=2000 lost=0 holes=0 src=electron` nel titolo, o la divergenza è registrata
- [ ] l'installatore esiste in `out/`; il lockfile è committato; `node_modules/`, `dist/` e `out/` ignorati
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 5: il guscio Tauri

**Files:**
- Create (LF): `spikes/gui-shell/tauri/package.json` · `src-tauri/Cargo.toml` · `src-tauri/build.rs` · `src-tauri/src/main.rs` · `src-tauri/tauri.conf.json` · `src-tauri/capabilities/default.json`; `spikes/gui-shell/measure/icon.py`
- Create dagli strumenti: `spikes/gui-shell/tauri/package-lock.json` · `src-tauri/Cargo.lock` · `tauri/icon.png` · `src-tauri/icons/*` (binari)
- Modify: `.gitignore` (**CRLF**)
- Read: P-5, P-7, P-8; `spikes/gui-ipc/src/bin/gui.rs` (il client della pipe con `interprocess`)

**Interfaces:**
- Consumes: la build `app/dist/`; l'evento `line` e il permesso del titolo nativo del compito 3; `tree.ps1` del compito 4
- Produces: l'eseguibile `src-tauri/target/release/sp8-tauri.exe` e l'installatore NSIS in `src-tauri/target/release/bundle/nsis/`

- [ ] **Passo 1: le misure prima**

```bash
grep -n '/spikes/gui-shell/electron/out/' .gitignore
rustc --version; cargo --version
ls spikes/gui-shell/app/dist/index.html
```

Atteso: **una** riga per l'aggancio; `1.95.0` (il `rust-toolchain.toml` di radice vale anche qui, ed
`edition = "2024"` lo richiede); la build esiste.

- [ ] **Passo 2: i file**

`spikes/gui-shell/tauri/package.json`:

```json
{
  "name": "sp8-tauri",
  "private": true,
  "version": "0.0.0",
  "scripts": {
    "dev": "tauri dev",
    "build": "tauri build",
    "icon": "tauri icon icon.png --output src-tauri/icons",
    "tauri": "tauri"
  },
  "devDependencies": {
    "@tauri-apps/cli": "2.11.4"
  }
}
```

`spikes/gui-shell/tauri/src-tauri/Cargo.toml`:

```toml
[package]
name = "sp8-tauri"
version = "0.0.0"
edition = "2024"
publish = false
description = "SP-8: the Tauri shell of the fake Home"

[build-dependencies]
tauri-build = { version = "2.6.3", features = [] }

[dependencies]
tauri = { version = "2.11.5", features = [] }
interprocess = "2.4.4"
```

`spikes/gui-shell/tauri/src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build()
}
```

`spikes/gui-shell/tauri/src-tauri/src/main.rs`:

```rust
//! SP-8 -- the Tauri shell of the fake Home. A thread reads the emitter's named pipe with `interprocess`
//! -- the same client as `spikes/gui-ipc/src/bin/gui.rs` -- and emits every line to the page as the event
//! `line`; the page listens with `@tauri-apps/api/event`. Q1: with this shell, decoding `bincode` would
//! live HERE, in Rust, with the kernel's own decoder. Nothing here is product code.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream, ToNsName};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager};

const NAME: &str = "gui-ipc-spike"; // `NOME` of spikes/gui-ipc/src/lib.rs: \\.\pipe\gui-ipc-spike on Windows (P-5)
const RETRY: Duration = Duration::from_secs(2);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            thread::spawn(move || loop {
                let connected = NAME
                    .to_ns_name::<GenericNamespaced>()
                    .and_then(Stream::connect);
                if let Ok(stream) = connected {
                    eprintln!("core connected");
                    for line in BufReader::new(stream).lines() {
                        let Ok(line) = line else { break };
                        let _ = handle.emit("line", line);
                    }
                    eprintln!("core gone");
                }
                // not listening yet, or gone: retry, like the "riprova" strip of the real GUI
                thread::sleep(RETRY);
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("the Tauri shell did not start");
}
```

`spikes/gui-shell/tauri/src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "sp8-tauri",
  "version": "0.0.0",
  "identifier": "harness.sp8.tauri",
  "build": {
    "frontendDist": "../../app/dist"
  },
  "app": {
    "windows": [
      { "label": "main", "title": "SP-8", "width": 1400, "height": 900 }
    ],
    "security": { "csp": null }
  },
  "bundle": {
    "active": true,
    "targets": ["nsis"],
    "icon": ["icons/icon.ico", "icons/icon.png"]
  }
}
```

`spikes/gui-shell/tauri/src-tauri/capabilities/default.json` — `core:default` per `listen` (P-7, dedotto)
e il permesso del titolo nativo (compito 3):

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "SP-8: the page listens to the event `line` and writes the window title",
  "windows": ["main"],
  "permissions": ["core:default", "core:window:allow-set-title"]
}
```

`spikes/gui-shell/measure/icon.py` — un PNG quadrato per `tauri icon`, con la sola libreria standard:

```python
"""SP-8 -- a square PNG for `tauri icon`, with the standard library only. Usage: python icon.py <out.png> [size]"""
import struct
import sys
import zlib


def chunk(kind: bytes, data: bytes) -> bytes:
    return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)


def png(path: str, size: int, rgb: tuple) -> None:
    row = b"\x00" + bytes(rgb) * size
    raw = row * size
    data = b"\x89PNG\r\n\x1a\n"
    data += chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 2, 0, 0, 0))
    data += chunk(b"IDAT", zlib.compress(raw, 9))
    data += chunk(b"IEND", b"")
    with open(path, "wb") as f:
        f.write(data)


if __name__ == "__main__":
    png(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 1024, (30, 60, 110))
```

- [ ] **Passo 3: `.gitignore`, CRLF — il guscio Tauri**

Con `replace_unique.py`: Trova `/spikes/gui-shell/electron/out/`; Sostituisci con:

```
/spikes/gui-shell/electron/out/
/spikes/gui-shell/tauri/node_modules/
/spikes/gui-shell/tauri/src-tauri/target/
/spikes/gui-shell/tauri/src-tauri/gen/
```

- [ ] **Passo 4: le icone, la build, la corsa di prova**

```bash
python spikes/gui-shell/measure/icon.py spikes/gui-shell/tauri/icon.png
(cd spikes/gui-shell/tauri && npm install 2>&1 | tail -3 && npm run icon 2>&1 | tail -3) && ls spikes/gui-shell/tauri/src-tauri/icons/ | head
(cd spikes/gui-shell/tauri && npm run build 2>&1 | tail -15)
ls spikes/gui-shell/tauri/src-tauri/target/release/sp8-tauri.exe spikes/gui-shell/tauri/src-tauri/target/release/bundle/nsis/*.exe spikes/gui-shell/tauri/src-tauri/Cargo.lock
ROOT=$(cygpath -w "$PWD")
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$ROOT\spikes\gui-shell\tauri\src-tauri\target\release\sp8-tauri.exe" -Seconds 30 -EmitterAt 8 -Emitter "$ROOT\spikes\gui-ipc\target\release\core.exe" | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'
git status --porcelain
```

Atteso: `icon.ico` e `icon.png` fra le icone; `tauri build` che compila la crate (minuti: `tauri` è
grande) e finisce con `Finished` e il percorso dell'installatore; l'eseguibile, l'installatore e il
`Cargo.lock` esistono; `last title:` con `msgs=2000 lost=0 holes=0` e `src=tauri` — se il titolo restasse
`SP-8` fisso, il permesso `core:window:allow-set-title` non ha retto (compito 3) e la voce d'errata porta
il nome letto nel messaggio di `tauri build`; `git status` che **non** nomina `target/`, `gen/`,
`node_modules/`. ⚠️ Un errore di `tauri build` sulla capability (`core:default` sconosciuto, un permesso
con un altro nome) è la voce d'errata di P-7, col nome giusto letto dal messaggio: **non** si toglie il
permesso per far passare la build.

- [ ] **Passo 5: le prove, il commit, il push**

```bash
for f in spikes/gui-shell/tauri/package.json spikes/gui-shell/tauri/package-lock.json spikes/gui-shell/tauri/src-tauri/Cargo.toml spikes/gui-shell/tauri/src-tauri/Cargo.lock spikes/gui-shell/tauri/src-tauri/build.rs spikes/gui-shell/tauri/src-tauri/src/main.rs spikes/gui-shell/tauri/src-tauri/tauri.conf.json spikes/gui-shell/tauri/src-tauri/capabilities/default.json spikes/gui-shell/measure/icon.py; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
printf '.gitignore CR='; tr -cd '\r' < .gitignore | wc -c; printf '   righe='; wc -l < .gitignore; git ls-files --eol .gitignore
bash scripts/check-docs.sh; bash scripts/gate.sh
git add .gitignore spikes/gui-shell/tauri spikes/gui-shell/measure/icon.py docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git status --porcelain
git commit -m "guscio(compito 5): il guscio Tauri di SP-8 — la pipe letta con interprocess in un thread ed emessa alla pagina come evento, la capability con listen e il titolo nativo, le icone generate, tauri build con l'installatore NSIS; Cargo.lock e lockfile committati, target e gen ignorati"
git push
```

Atteso: `0` CR sui nove file (`Cargo.lock` lo scrive `cargo` LF); `.gitignore` CR = righe; `OK`,
`GATE GREEN` — il cancello **non** compila `spikes/`, e `gate-deps.sh` non vede `tauri` né `interprocess`
perché guarda i grafi di `kernel` e `simulator`; `git status` dopo l'`add` con i soli file della mappa.

#### Criterio di chiusura del compito 5

- [ ] `tauri build` verde; l'eseguibile e l'installatore esistono; la corsa di prova rende `msgs=2000 lost=0 holes=0 src=tauri`, o la divergenza è registrata
- [ ] `Cargo.lock`, `package-lock.json` e le icone committati; `target/`, `gen/`, `node_modules/` ignorati
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---
## Compito 6: le misure M1–M5 e Q1–Q2 sui due gusci installati, e la sezione SP-8 di `spikes/RISULTATI.md`

**Files:**
- Modify: `spikes/RISULTATI.md` (**CRLF**) — la sezione **SP-8**, inserita **prima** della riga `## SP-7 — …`, e la riga 3 della data; questo piano (LF) — la posizione, e l'errata se una misura smentisce P-9, P-21 o D19
- Read: `spikes/gui-shell/PROTOCOLLO.md` — le tabelle M1–M5 e Q1–Q4, «La macchina», «Registrazione»; la sezione SP-7 di [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md) per la forma; `spikes/gui-shell/measure/tree.ps1` e `size.ps1` (compito 4); P-9, P-13, P-21, D7, D9, D17, D19, D20, D21; la §2 del disegno del 2

**Interfaces:**
- Consumes: gli installatori dei compiti 4 e 5 — `spikes/gui-shell/electron/out/sp8-electron Setup 0.0.0.exe` e `spikes/gui-shell/tauri/src-tauri/target/release/bundle/nsis/sp8-tauri_0.0.0_x64-setup.exe`; `tree.ps1` coi parametri `-Exe`, `-Seconds`, `-EmitterAt`, `-Emitter`, `-Csv` e le righe `rest: …`, `stream: …`, `last title: …`; `size.ps1 -Path`; il titolo della finestra nella forma del compito 3; l'emettitore `spikes/gui-ipc/target/release/core.exe`
- Produces: la sezione **SP-8** con M1–M5, Q1 e Q2 **piene** e le righe che aspettano il proprietario scritte `⏳ al compito 7, col proprietario` (D17); le due app **installate**, che il compito 7 usa e il compito 8 disinstalla (D19); i CSV in `$HOME/sp8-measure/`, **fuori** dal repository (D21)

⛔ **Ogni numero esce da un comando e va nell'esito testuale**; un criterio non superato resta scritto non superato — il
metro è congelato dal compito 2. M1, M2, M3 e M5 «si riportano» senza soglia; la sola soglia è **P3** su M4, il picco sotto il
25 % di un core (protocollo). ⚠️ Le voci del proprietario **non si riempiono qui** e non si lasciano vuote: si scrivono con le
parole di D17, e il compito 7 le consuma.

- [ ] **Passo 1: le misure prima**

```bash
ls "spikes/gui-shell/electron/out/sp8-electron Setup 0.0.0.exe" spikes/gui-shell/tauri/src-tauri/target/release/bundle/nsis/*-setup.exe spikes/gui-ipc/target/release/core.exe
grep -n '^## SP-' spikes/RISULTATI.md
sed -n '3p' spikes/RISULTATI.md
git ls-files --eol spikes/RISULTATI.md
printf 'RISULTATI CR='; tr -cd '\r' < spikes/RISULTATI.md | wc -c; printf '   righe='; wc -l < spikes/RISULTATI.md
powershell -NoProfile -Command "Get-Process sp8-electron,sp8-tauri,electron -ErrorAction SilentlyContinue | Select-Object -ExpandProperty ProcessName -Unique"
```

Atteso: i tre eseguibili esistono — altrimenti `(cd spikes/gui-shell/electron && npm run dist)`,
`(cd spikes/gui-shell/tauri && npm run build)`, `(cd spikes/gui-ipc && cargo build --release)`; l'ultima sezione di
`RISULTATI.md` è `SP-7` e **nessuna** `SP-8` (domanda 4); la riga 3 è
`Data di esecuzione: **2026-08-06** per SP-5 e SP-6; **SP-7** porta la propria data nella sua sezione`; `i/lf w/crlf`;
CR = righe; **nessun** processo dei gusci in corsa — un'istanza aperta da prima terrebbe la pipe dell'emettitore o
entrerebbe nei conteggi di un altro albero.

- [ ] **Passo 2: le versioni del giorno, e la macchina (vincoli 8 e 16)**

```bash
node --version; npm --version; cargo --version; rustc --version
(cd spikes/gui-shell/app && npm ls --depth=0 2>&1 | tail -9)
(cd spikes/gui-shell/electron && npm ls --depth=0 2>&1 | tail -4)
(cd spikes/gui-shell/tauri && npm ls --depth=0 2>&1 | tail -3)
grep -A1 -E '^name = "(tauri|wry|interprocess)"$' spikes/gui-shell/tauri/src-tauri/Cargo.lock
grep -A1 -E '^name = "interprocess"$' spikes/gui-ipc/Cargo.lock
powershell -NoProfile -Command "(Get-ItemProperty 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}').pv"
powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name; (Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name)"
```

E le ultime versioni al registro npm e a crates.io, **quel giorno** — il comando di `riferimenti.md` ristretto ai pacchetti
di P-8:

```bash
python - <<'EOF'
import json, urllib.request, urllib.parse
def npm(p):
    d = json.load(urllib.request.urlopen("https://registry.npmjs.org/" + urllib.parse.quote(p, safe="@")))
    v = d["dist-tags"]["latest"]
    return v, d["time"][v][:10]
def crate(c):
    req = urllib.request.Request("https://crates.io/api/v1/crates/" + c, headers={"User-Agent": "harness (contatto nel repo)"})
    d = json.load(urllib.request.urlopen(req))["crate"]
    return d["max_stable_version"], d["updated_at"][:10]
for p in ["dockview-core", "vue", "vite", "@vitejs/plugin-vue", "three", "markdown-it", "electron", "electron-builder", "@tauri-apps/cli", "@tauri-apps/api"]:
    print("npm", p, *npm(p))
for c in ["tauri", "tauri-build", "wry", "interprocess"]:
    print("crates.io", c, *crate(c))
EOF
```

Atteso: le versioni **installate** — `npm ls` e i due `Cargo.lock` — sono quelle appuntate in P-8 (`dockview-core` 8.2.0,
`vue` 3.5.42, `vite` 8.2.2, `three` 0.185.1, `markdown-it` 15.0.1, `electron` 44.3.0, `electron-builder` 26.15.3,
`@tauri-apps/cli` 2.11.4, `@tauri-apps/api` 2.11.1, `tauri` 2.11.5, `interprocess` 2.4.4); le ultime al registro **possono**
essere più nuove: si scrivono nell'esito, non si prendono (vincolo 8). WebView2 il 2026-09-09 era `152.0.4191.66` (P-9), e si
scrive quello **letto**.

- [ ] **Passo 3: l'installazione silenziosa di entrambi, e M2**

I due installatori NSIS sono **per utente** (D9) e accettano `/S`; la cartella la sceglie l'installatore e si **legge**
(D19, P-21 — dedotto: `%LOCALAPPDATA%\Programs\sp8-electron` per `electron-builder` con `perMachine: false`, e
`%LOCALAPPDATA%\sp8-tauri` per `tauri build`):

```bash
ROOT=$(cygpath -w "$PWD")
TAURI_SETUP=$(cygpath -w "$(ls spikes/gui-shell/tauri/src-tauri/target/release/bundle/nsis/*-setup.exe | head -1)")
powershell -NoProfile -Command "Start-Process -Wait -FilePath '$ROOT\spikes\gui-shell\electron\out\sp8-electron Setup 0.0.0.exe' -ArgumentList '/S'"
powershell -NoProfile -Command "Start-Process -Wait -FilePath '$TAURI_SETUP' -ArgumentList '/S'"
powershell -NoProfile -Command "Get-ChildItem \$env:LOCALAPPDATA\Programs -Directory -Filter 'sp8-*' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty FullName; Get-ChildItem \$env:LOCALAPPDATA -Directory -Filter 'sp8-*' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty FullName"
powershell -NoProfile -Command "Get-Process sp8-electron,sp8-tauri -ErrorAction SilentlyContinue | Stop-Process -Force"
```

Atteso: **due** cartelle, una per guscio, con dentro `sp8-electron.exe` e `sp8-tauri.exe`; se un installatore avviasse l'app
dopo `/S` (P-21, dedotto che non lo faccia), lo `Stop-Process` la chiude. Le due cartelle, in forma Windows, vanno nelle due
variabili qui sotto **e nell'esito**, riga M2. Poi M2, quattro misure:

```bash
EL_DIR='<la cartella di Electron letta sopra, ad esempio C:\Users\zagor\AppData\Local\Programs\sp8-electron>'
TA_DIR='<la cartella di Tauri letta sopra, ad esempio C:\Users\zagor\AppData\Local\sp8-tauri>'
for p in "$EL_DIR" "$TA_DIR" "$ROOT\spikes\gui-shell\electron\out\sp8-electron Setup 0.0.0.exe" "$TAURI_SETUP"; do powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/size.ps1 -Path "$p"; done
```

Atteso: quattro righe testuali — `<cartella>: <n> files, <n> bytes, <n> MB` per le cartelle, `<file>: <n> bytes, <n> MB` per
gli installatori — nell'esito così come sono. Dedotto e da guardare: la cartella di Electron porta `sp8-electron.exe`,
`resources/app.asar`, i `.pak` e le DLL di Chromium; quella di Tauri il solo `sp8-tauri.exe` e poco altro — è la ragione per
cui M2 «discrimina» (ADR-0029); se non fosse così è una divergenza da scrivere, non da spiegare.

- [ ] **Passo 4: M1, M4 e M5 con `tree.ps1`, due corse per guscio (D7, D20)**

`-Seconds 60 -EmitterAt 32`: trentadue secondi di **riposo**, così la finestra dei 30 s di M3 è piena **prima** del flusso;
poi ventotto secondi di **flusso**, che coprono la riconnessione alla pipe (ogni due secondi, D8), i dieci secondi dei 2000
messaggi e la coda (P-13). Due corse per guscio, perché una corsa sola è un punto in una nuvola (O1 di SP-7). I CSV vanno in
`$HOME/sp8-measure/` (D21):

```bash
mkdir -p "$HOME/sp8-measure"; CSV=$(cygpath -w "$HOME/sp8-measure")
EM="$ROOT\spikes\gui-ipc\target\release\core.exe"
for i in 1 2; do powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$EL_DIR\sp8-electron.exe" -Seconds 60 -EmitterAt 32 -Emitter "$EM" -Csv "$CSV\electron-$i.csv" | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'; done
for i in 1 2; do powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$TA_DIR\sp8-tauri.exe" -Seconds 60 -EmitterAt 32 -Emitter "$EM" -Csv "$CSV\tauri-$i.csv" | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'; done
for f in electron-1 electron-2 tauri-1 tauri-2; do printf '%s rest title: ' "$f"; powershell -NoProfile -Command "Import-Csv '$CSV\\$f.csv' | Where-Object phase -eq rest | Select-Object -Last 1 -ExpandProperty title"; done
for f in electron-1 electron-2 tauri-1 tauri-2; do printf '%s procs: ' "$f"; powershell -NoProfile -Command "Import-Csv '$CSV\\$f.csv' | Measure-Object procs -Minimum -Maximum | Select-Object Minimum,Maximum | Format-Table -HideTableHeaders"; done
```

Atteso, per corsa: `shell started: pid <n>`, `emitter started at ~32 s`, la riga `rest: samples <n> | rss_mb mean <n> max <n> | cpu_pct mean <n> max <n> | vram_mb max <n>`,
la stessa per `stream:`, e `last title: SP-8 | fps=<n> min=<n> api=<API> scene=<w>x<h> | msgs=2000 lost=0 holes=0 p2max=<ms>ms p2mean=<ms>ms | src=electron dnd=auto | ua=Chrome/<…>`
(`src=tauri` per Tauri). Le letture: **M1** = `rss_mb mean` e `max` di `rest` (a riposo) e di `stream` (sotto flusso); **M4** =
`cpu_pct max` di `stream`, il picco che P3 giudica, e `mean`; la CPU di `rest` è la sola scena che gira, e va nell'esito
come lettura di M4 (O2); **M5** a riposo = `vram_mb max` di `rest`; **M3 a riposo** = `fps=`, `min=`, `api=`, `scene=` del
titolo dell'ultima riga `rest` del CSV, **M3 sotto flusso** = gli stessi campi di `last title:`; **P1 e P2** = `msgs=`,
`lost=`, `holes=`, `p2max=`, `p2mean=`; i **processi** dell'albero, minimo e massimo, per corsa.

⛔ **Le divergenze si scrivono, non si aggiustano.** Se per Tauri `procs` resta **1** per tutta la corsa, i processi di
WebView2 stanno **fuori** dall'albero (P-9): la voce d'errata porta `Get-CimInstance Win32_Process -Filter "Name='msedgewebview2.exe'" | Select-Object ProcessId,ParentProcessId`
letto con l'app aperta, e M1, M4 e M5 di Tauri si dichiarano **parziali** finché l'albero non li prende. Se `vram_mb` resta
`0` con la scena in moto, l'indiziato è il filtro sulle istanze (P-9): la voce d'errata porta
`Get-Counter -ListSet 'GPU Process Memory' | Select-Object -ExpandProperty PathsWithInstances | Select-Object -First 5`.
Se `msgs` è sotto 2000 o `holes` sopra zero, è una misura di P1 e si registra. Se `api=` rende `WebGL2:…` su questa
macchina, è una misura di M3 e si registra (P-6).

- [ ] **Passo 5: Q1 dall'architettura, Q2 dal registro e alle fonti**

**Q1** si legge, non si misura (protocollo): con Electron chi decodifica `bincode` è il **processo principale Node** —
`spikes/gui-shell/electron/main.js` legge la pipe, e lo dice il commento in testa; con `bincode-ts` (M-11 di ADR-0037) — e con
Tauri è il **guscio Rust** — `spikes/gui-shell/tauri/src-tauri/src/main.rs`, il commento in testa — col decodificatore del
kernel, cioè una terza compilazione delle crate (decisione 57 della stella polare).

**Q2**: WebView2 dal valore `pv` del Passo 2; il Chromium di Electron dal segmento `ua=` del titolo (`Chrome/<…>`), che deve
coincidere con `process.versions.chrome` del preload; e lo stato di WebGPU su **WebKitGTK** letto **quel giorno** alle fonti
primarie — è la lettura che sostituisce la misura Linux (decisione C del disegno del 2). Tre pagine, con `WebFetch`:

| Pagina | Che cosa si cerca |
|---|---|
| `https://github.com/gpuweb/gpuweb/wiki/Implementation-Status` | la riga di WebKit su Linux / GTK: rilasciato, dietro flag, o assente |
| `https://webkitgtk.org/news.html` | l'ultima versione stabile e se le sue note nominano WebGPU |
| `https://v2.tauri.app/reference/webview-versions/` | la riga su Linux: quale WebKitGTK usa `wry` |

Per ciascuna: l'URL, la data della lettura, la frase letta **breve e fra virgolette**. Vanno nell'esito qui, e al compito 8
in `riferimenti.md` (sezione SP-8) e in ADR-0029.

- [ ] **Passo 6: `spikes/RISULTATI.md`, CRLF — la sezione SP-8 e la riga della data**

Con `replace_unique.py`, due sostituzioni:

| Trova | Sostituisci con |
|---|---|
| la riga `## SP-7 — Riconoscimento gesti: MediaPipe su CPU, e il giro worker → core → GUI — eseguito il 2026-09-04`, **intera, presa dal file** | la sezione qui sotto, una riga vuota, e la stessa riga |
| `Data di esecuzione: **2026-08-06** per SP-5 e SP-6; **SP-7** porta la propria data nella sua sezione` | `Data di esecuzione: **2026-08-06** per SP-5 e SP-6; **SP-7** e **SP-8** portano la propria data nella loro sezione` |

La sezione, nella forma di SP-7. ⛔ **Le celle fra `<…>` sono slot di misura** e si riempiono **tutte** in questo compito
dall'output dei Passi 2–5; le righe con `⏳ al compito 7, col proprietario` restano **così** (D17): sono le sole che il
compito 7 riempie. Dentro una cella di tabella nessuno slot porta una barra verticale (D25). I link sono relativi a
`spikes/`, perché `check-docs.sh` legge questo file (P-23).

```markdown
## SP-8 — Il guscio della GUI, e l'accettazione di `dockview` — misure il <data>; ⏳ il giudizio del proprietario al compito 7

Criteri e soglie: [`gui-shell/PROTOCOLLO.md`](gui-shell/PROTOCOLLO.md), congelato il <la data del commit del compito 2>
al primo commit di codice dello spike, **prima** della misura. Codice in `gui-shell/`: la Home finta in `app/`, i due
gusci in `electron/` e `tauri/`, gli script di misura in `measure/`; i campioni grezzi (CSV) fuori dal repository. Le
due app sono state **installate** dagli installatori NSIS per utente e misurate da lì, col flusso dell'emettitore di
`gui-ipc/` com'è — righe JSON, `"lorem ipsum dolor sit amet"` su ogni riga, quindi markdown senza blocchi di codice.
⏳ **Le otto mosse, Q3, Q4, la CPU con la chat nascosta e la VRAM a pagina intera aspettano il proprietario: compito 7 del piano.**

**La macchina:** CPU `<Win32_Processor>`, GPU `<Win32_VideoController>`, WebView2 `<pv>`, il Chromium di Electron `<Chrome/…, da ua=>`.

| Misura | Criterio | Electron — corsa 1; corsa 2 | Tauri — corsa 1; corsa 2 |
|---|---|---|---|
| M1 — RAM a riposo, media / picco (MB) | si riporta | <n> / <n>; <n> / <n> | <n> / <n>; <n> / <n> |
| M1 — RAM sotto flusso, media / picco (MB) | si riporta | <n> / <n>; <n> / <n> | <n> / <n>; <n> / <n> |
| M2 — cartella installata (file, MB) e installatore (MB) | si riporta | `<cartella>`: <n> file, <n> MB; installatore <n> MB | `<cartella>`: <n> file, <n> MB; installatore <n> MB |
| M3 — fps medi / minimi sugli ultimi 30 s **a riposo**, l'API ottenuta, la taglia della tessera | si riporta, su Windows | <n> / <n>, `api=<…>`, `scene=<w>x<h>`; <n> / <n> | <n> / <n>, `api=<…>`, `scene=<w>x<h>`; <n> / <n> |
| M3 — lo stesso **sotto flusso** (gli ultimi 30 s della corsa) | si riporta | <n> / <n>; <n> / <n> | <n> / <n>; <n> / <n> |
| M4 — CPU dell'albero sotto i 2000 messaggi, picco / media (% di un core) | **picco < 25 %** (P3) | <n> / <n>; <n> / <n> — <✅ `passa` oppure ⚠️ `parziale` oppure ❌ `non passa`> | <n> / <n>; <n> / <n> — <esito> |
| M4 — CPU a riposo, la sola scena che gira, picco / media | senza soglia, per leggere M4 | <n> / <n>; <n> / <n> | <n> / <n>; <n> / <n> |
| M4 — CPU con la chat **nascosta** | senza soglia | ⏳ al compito 7, col proprietario | ⏳ al compito 7, col proprietario |
| M5 — VRAM a riposo, picco (MB) | si riporta | <n>; <n> | <n>; <n> |
| M5 — VRAM con la scena **a pagina intera** | si riporta | ⏳ al compito 7, col proprietario | ⏳ al compito 7, col proprietario |
| P1, P2 dal titolo — `msgs= lost= holes=`, `p2max= p2mean=` | si riporta | <…>; <…> | <…>; <…> |
| processi nell'albero, minimo / massimo per corsa | si riporta | <n> / <n>; <n> / <n> | <n> / <n>; <n> / <n> |
| Q1 — chi decodifica `bincode` lato GUI | letto dall'architettura, non misurato | il processo principale Node — `electron/main.js` — con `bincode-ts` (M-11 di ADR-0037) | il guscio Rust — `tauri/src-tauri/src/main.rs` — col decodificatore del kernel |
| Q2 — le webview in uso | letto | il Chromium impacchettato: `<Chrome/…>` | WebView2 `<pv>` dal registro; **WebKitGTK**, alle fonti il <data>: <lo stato di WebGPU, in una riga, con l'URL> |
| Q3 — la finestra staccata si apre dentro il guscio | ⏳ al compito 7, col proprietario | ⏳ | ⏳ |
| Q4 — `dndStrategy` che basta nella webview | ⏳ al compito 7, col proprietario | ⏳ | ⏳ |

**Le otto mosse — il giudizio del proprietario, con le sue parole:** ⏳ al compito 7, col proprietario.

**La decisione sul guscio, del proprietario:** ⏳ al compito 7, col proprietario.

### SP-8 · Osservazioni registrate — non criteri

| # | Osservazione |
|---|---|
| O1 | <quanto ballano M1, M4 e M5 fra le due corse di ciascun guscio, e se una corsa sola avrebbe cambiato la lettura — O1 di SP-7> |
| O2 | <la CPU a riposo con la sola scena, e quanto pesa sul picco di M4: la scena sta nel frontend di entrambi i gusci, quindi il confronto regge, ma la soglia P3 la paga; e ciò che dice il limite dichiarato «JSON e non `bincode`»> |
| O3 | <M3 sotto flusso contro M3 a riposo, per guscio> |
| O4 | <i processi dell'albero per guscio, e se quelli di WebView2 stavano dentro l'albero di Tauri — la trappola di M1> |
| O5 | <ciò che ha sorpreso, oppure «niente»> |

### SP-8 · Versioni degli strumenti

| Strumento | Comando | Output |
|---|---|---|
| Node, npm | `node --version; npm --version` | `<…>` |
| Rust | `rustc --version; cargo --version` | `<…>` |
| i pacchetti npm installati | `npm ls --depth=0` in `gui-shell/app/`, `gui-shell/electron/`, `gui-shell/tauri/` | `<…>` — le versioni appuntate dal piano (P-8) |
| le crate del guscio Tauri e dell'emettitore | `grep -A1` sui nomi `tauri`, `wry`, `interprocess` nei due `Cargo.lock` | `<…>` |
| le ultime al registro npm e a crates.io, quel giorno | il comando del compito 6 del piano | `<…>` — <uguali alle appuntate, oppure quali sono più nuove e non prese> |
| WebView2 | il valore `pv` della chiave del protocollo | `<…>` |
| CPU, GPU | `Get-CimInstance Win32_Processor`, `Win32_VideoController` | `<…>` |

### SP-8 · Evidenze

| Misura | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| M1, M4, M5 — Electron, corse 1 e 2 | `tree.ps1 -Exe <sp8-electron.exe installato> -Seconds 60 -EmitterAt 32 -Emitter <core.exe> -Csv <fuori dal repository>` | le righe `rest:`, `stream:` e `last title:` di ciascuna corsa, testuali | <…> |
| M1, M4, M5 — Tauri, corse 1 e 2 | lo stesso con `sp8-tauri.exe` | le stesse tre righe per corsa | <…> |
| M2 | `size.ps1 -Path` sulle due cartelle installate e sui due installatori | le quattro righe, testuali | <…> |
| M3 a riposo | il titolo dell'ultima riga `rest` del CSV, letto con `Import-Csv` (il comando nel compito 6 del piano) | testuale, per corsa | <…> |
| Q2, WebKitGTK | le tre pagine lette il <data>: l'*Implementation Status* del wiki di `gpuweb/gpuweb`, le news di `webkitgtk.org`, *Webview Versions* di Tauri | <la frase letta in ciascuna, breve, fra virgolette> | <…> |
```

- [ ] **Passo 7: le prove, il commit, il push**

```bash
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '<[^ ]'
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -n '⏳' | cut -c1-60
grep -n '^## SP-\|^### SP-8' spikes/RISULTATI.md
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' spikes/RISULTATI.md
printf 'RISULTATI CR='; tr -cd '\r' < spikes/RISULTATI.md | wc -c; printf '   righe='; wc -l < spikes/RISULTATI.md; git ls-files --eol spikes/RISULTATI.md
bash scripts/check-docs.sh; bash scripts/gate.sh
git status --porcelain
```

Atteso: `0` — nessuno slot `<…>` nella sezione, e l'oracolo è **delimitato** alla sezione (E13 dei gesti); «picco < 25 %» non
conta, perché dopo `<` c'è uno spazio; le righe con `⏳` dentro l'`awk` sono **sette** — il capoverso d'apertura, M4
nascosta, M5 a pagina intera, Q3, Q4, le mosse, la decisione — perché l'`awk` **salta l'intestazione** (`{s=1;next}`), che
è l'ottava: il compito 7 le consuma tutte e otto (misurato sul modello alla rilettura del 2026-09-10); `## SP-8` **prima** di
`## SP-7`, con le tre sottosezioni `### SP-8 ·`; l'`awk` delle tabelle spezzate non stampa nulla; CR = righe e
`i/lf w/crlf`; `OK` — il controllo dei link legge `spikes/RISULTATI.md`, e i due link della sezione sono relativi a `spikes/`
— e `GATE GREEN`; `git status` che nomina `spikes/RISULTATI.md` e questo piano, **non** i CSV.

```bash
git add spikes/RISULTATI.md docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git commit -m "guscio(compito 6): SP-8 misurato sui due gusci installati — M1, M4 e M5 con due corse per guscio dall'albero di processi, M2 sulle cartelle installate e sugli installatori, M3 con l'API ottenuta a riposo e sotto flusso, Q1 dall'architettura, Q2 dal registro e alle fonti; la sezione SP-8 in RISULTATI.md con le righe del proprietario dichiarate in attesa del compito 7"
git push
```

#### Criterio di chiusura del compito 6

- [ ] la sezione SP-8 esiste prima di SP-7, con le tre sottosezioni; nessuno slot `<…>`; le otto righe ⏳ con le parole di D17
- [ ] due corse per guscio, i CSV in `$HOME/sp8-measure/` e fuori dal repository; le due app restano installate per il compito 7; le divergenze da P-9, P-21 e D19, se ci sono, nell'errata
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata

---

## Compito 7: col proprietario — le otto mosse, Q3 e Q4, le due misure che vogliono un click, la domanda sul guscio

⛔ **Non si dispaccia a un subagente** (regola 7 di «Come si esegue»): lo esegue il coordinatore col proprietario allo
schermo, come il compito 8 dei gesti; ciò che ne esce lo rilegge un revisore in **sola lettura** (E14 dei gesti: era
l'unico artefatto che nessuno aveva riletto). Il proprietario dice sì o no a ogni mossa **con le sue parole**, e le parole
vanno nell'esito così come sono, non riassunte; la lettera sul guscio è sua (D15).

**Files:**
- Modify: `spikes/RISULTATI.md` (**CRLF**) — le otto righe `⏳` del compito 6, una per volta, e l'intestazione della sezione; `docs/roadmap.md` (**CRLF**) — la riga SP-8 della tabella degli spike e la riga «Ultimo aggiornamento» (P-15); questo piano (LF) — la posizione, e l'errata se la mossa 8 smentisce P-3 o P-6; **solo** col ripiego di P-3, `spikes/gesti/relay/src/main.rs` e un richiamo datato nel protocollo
- Read: `spikes/gui-shell/PROTOCOLLO.md` — «Le otto mosse», «La mossa 8, come si fa», «Come si decide»; la sezione SP-8 di [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md); [ADR-0029](../../adr/0029-guscio-della-gui.md) — «Il confronto», «Perché la bilancia pende», «Come si chiude» — per la domanda A/B; il compito 8 del [piano dei gesti](2026-09-03-riconoscimento-gesti.md) per la forma delle parole del proprietario

**Interfaces:**
- Consumes: la Home nel browser (`npm run dev` in `app/`); il relay e il worker di SP-7; le due app installate dal compito 6, con le cartelle scritte nella riga M2 dell'esito; `tree.ps1 -AttachPid` (compito 4); l'emettitore
- Produces: la sezione SP-8 **completa**; la lettera e le parole del proprietario sul guscio e l'esito di `dockview`, che il compito 8 scrive in ADR-0029, nel compendio e nei disegni; su un **no all'insieme** delle mosse, il compito **7-bis** (D12), scritto in questa sessione prima del compito 8

- [ ] **Passo 1: le misure prima**

```bash
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '⏳'
grep -n 'M2 — cartella installata' spikes/RISULTATI.md | cut -c1-200
ls spikes/gesti/.venv/Scripts/python.exe spikes/gesti/hand_landmarker.task spikes/gesti/relay/target/release/sp7-relay.exe
powershell -NoProfile -Command "Get-PnpDevice -Class Camera -Status OK | Select-Object -ExpandProperty FriendlyName"
git ls-files --eol spikes/RISULTATI.md docs/roadmap.md
for f in spikes/RISULTATI.md docs/roadmap.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
```

Atteso: `7` — l'`awk` salta l'intestazione, che è l'ottava riga `⏳` (compito 6, Passo 7); la riga M2 con le **due cartelle** installate, che diventano `EL_DIR` e `TA_DIR` qui sotto; i tre file di SP-7
(il relay si ricompila con `(cd spikes/gesti/relay && cargo build --release)`); una telecamera; `i/lf w/crlf` su entrambi;
CR = righe. Poi le variabili che i passi usano:

```bash
ROOT=$(cygpath -w "$PWD")
EM="$ROOT\spikes\gui-ipc\target\release\core.exe"
CSV=$(cygpath -w "$HOME/sp8-measure")
EL_DIR='<la cartella di Electron, dalla riga M2>'
TA_DIR='<la cartella di Tauri, dalla riga M2>'
ls "$(cygpath "$EL_DIR")/sp8-electron.exe" "$(cygpath "$TA_DIR")/sp8-tauri.exe"
```

- [ ] **Passo 2: il relay di SP-7 e la Home nel browser**

Due comandi, ciascuno in un **terminale suo** che resta aperto (o col tool Bash in background, con percorsi assoluti —
il cwd del tool persiste fra le chiamate, vicolo cieco della stella polare):

```bash
cd spikes/gesti/relay && cargo run --release -- ../.venv/Scripts/python ../s2_worker.py ../hand_landmarker.task
```

```bash
cd spikes/gui-shell/app && npm run dev
```

Il proprietario apre `http://localhost:5173`, preme **«azzera»** (la disposizione di default), e legge nel titolo della
scheda `src=browser` e nella tessera Mano `relay connected`. ⚠️ Se la tessera Mano dice `relay not reachable` col relay in
ascolto, il proxy di `vite` non passa l'SSE: è P-3 — voce d'errata, e il ripiego è **una riga** nel relay,
`Access-Control-Allow-Origin: *` sulla risposta di `/stream`, con `hand.ts` che apre `http://127.0.0.1:7878/stream` invece
di `/stream`; tocca `spikes/gesti/` (vincolo 17) e si dichiara nel protocollo con richiamo datato, come E11 dei gesti.

- [ ] **Passo 3: le mosse 1–7, una per volta, con le sue parole**

Sotto `dnd: auto` (il default del menu). Per ciascuna il proprietario dice **sì o no con le sue parole**; una mossa che passa
solo con un accorgimento è «parziale» (protocollo). Le righe della barra si copiano **testuali**.

| # | Come si prova nella Home | Passa se (protocollo), e che cosa scrive la barra |
|---|---|---|
| 1 | trascinare Stato, poi Chat, dalla presa grande sopra Nucleo e sopra Striscia | i due restano dove sono. ⚠️ Se una tessera si lascia **agganciare** sopra il nucleo, si scrive: è un «parziale» da giudicare col proprietario, e il rimedio — `'no-drop-target'` al posto di `true` in `lock()` di `home.ts` — è una voce d'errata sul compito 2, non un ritocco silenzioso |
| 2 | il comando ⧉ (o il tasto `F`) su Stato: galleggia; trascinarla dalla presa: scatta alla griglia di 24 px; poi trascinarla dentro la griglia | si sgancia, si muove, torna, senza perdersi — `move 2: stato floats` |
| 3 | ⤢ (o `M`) su Scena 3D, poi di nuovo | un gesto per andare, uno per tornare, la disposizione sotto resta — `move 3: scene full page`, `move 3: scene back` |
| 4 | ↗ (o `P`) su Costi: si apre una finestra nuova del browser; chiuderla | chiusa, la tessera torna — `move 4 / Q3: popout OPENED for costi`. Nel browser prova `dockview`; **dentro i gusci** è Q3, al Passo 5 |
| 5 | afferrare la presa grande — la linguetta alta 40 px col titolo — col mouse; col tocco **solo** se lo schermo è touch, altrimenti «tocco: non misurato» | si afferra senza mirare a una linguetta sottile |
| 6 | `Ctrl+Alt+←`, `→`, `↑`, `↓` sulla tessera attiva | la tessera si sposta nelle quattro direzioni — `move 6: <id> -> group <…>` oppure `move 6: <id> splits <lato>` |
| 7 | «salva (mossa 7)», poi «ricarica» | `move 7: EQUAL, <n> bytes before and after the reload` — oppure `move 7: DIFFERENT at byte <n> (<n> vs <n> bytes)`: una misura, non un giudizio (decisione 32), copiata testuale |

- [ ] **Passo 4: la mossa 8, la mano di SP-7 come puntatore**

1. dal menu della barra si sceglie `dnd: pointer` — la barra scrive `Q4: dndStrategy = pointer`, e la scelta sopravvive a
   una ricarica (`localStorage`);
2. la mano davanti alla telecamera: lo scheletro a 21 punti si disegna sopra tutta la pagina, specchiato come in
   `page.html` di SP-7; la **pinza** — pollice e indice a meno di 40 px del fotogramma, o il pugno, che vale come pinza
   (O7 di SP-7) — sulla presa grande di una tessera: la barra scrive `move 8: pointerdown at <x>,<y> on div.bigtab` — o `on span.bigtab-title`, il titolo che riempie la
   presa (`.bigtab-title { flex: 1 }`): `elementFromPoint` rende l'elemento più profondo, e l'evento risale alla linguetta;
   un `on button.` è la pinza su un comando ⧉ ⤢ ↗, che ferma la propagazione e non fa partire nulla: si riprova sul titolo; con la
   pinza chiusa la mano si muove e la tessera **segue** — si sgancia, galleggia, si sposta — e all'apertura la barra scrive
   `move 8: pointerup …` e la tessera si riaggancia dove sta;
3. il sì o no **tecnico** lo dicono quelle righe e la tessera: `pointerdown … on div.bigtab` (o `on span.bigtab-title`) con la tessera che **non** segue
   è il no tecnico che P-6 prevede — `dockview` non segue il puntatore sintetico — e si **scrive**, non si aggira; il
   proprietario dice come si sente **con le sue parole, senza soglia**; il ritardo che si sente è quello di SP-7 — cattura →
   disegno, mediana 114 ms — della pipeline e non di `dockview`.

Poi `Ctrl-C` nei due terminali: il relay e il worker si chiudono, e la pagina di `vite` non serve più.

- [ ] **Passo 5: Q3 e Q4 nei due gusci installati, lanciati a mano, senza emettitore**

Per ciascun guscio, nell'ordine Electron poi Tauri:

```bash
powershell -NoProfile -Command "Start-Process -FilePath '$EL_DIR\sp8-electron.exe'"
```

1. **Q4**: `dnd: auto` dal menu; la mossa 2 col mouse — ⧉ su Stato, poi trascinarla dalla presa dentro la griglia. Se il
   trascinamento funziona, Q4 = `'auto'` basta; se non parte, `dnd: pointer` e di nuovo: Q4 = serve `'pointer'`. Si scrive
   quale, per guscio;
2. **Q3**: il comando ↗ su Costi: la barra scrive `move 4 / Q3: popout OPENED for costi` oppure `REFUSED`, e l'occhio dice se
   la finestra nuova è comparsa **dentro il guscio** — una finestra figlia dell'app, non il browser di sistema; chiusa, la
   tessera torna. Un no in un guscio è un fatto per ADR-0029, non un no a `dockview` (protocollo);
3. si chiude l'app dalla sua finestra.

Poi lo stesso con `Start-Process -FilePath '$TA_DIR\sp8-tauri.exe'`. Il `localStorage` della webview di ciascun guscio è
suo: la strategia scelta nel browser non c'entra.

- [ ] **Passo 6: M4 con la chat nascosta, e M5 con la scena a pagina intera — per guscio (D7, `-AttachPid`)**

Per ciascun guscio, lanciato a mano come al Passo 5 e con la disposizione di default («azzera» se serve):

1. il proprietario trascina **Scena 3D** dentro il gruppo della **Chat**, come seconda linguetta, e attiva Scena: la chat è
   nascosta dietro;
2. il coordinatore prende il PID di radice e lancia `tree.ps1` **agganciato** — con `-AttachPid` lo script non lancia e non
   ferma nulla (compito 4) — con l'emettitore a tre secondi:

```bash
PID=$(powershell -NoProfile -Command "(Get-Process sp8-electron | Sort-Object StartTime | Select-Object -First 1).Id")
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -AttachPid $PID -Seconds 25 -EmitterAt 3 -Emitter "$EM" -Csv "$CSV\electron-hidden.csv" | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'
```

   Per Tauri `PID=$(powershell -NoProfile -Command "(Get-Process sp8-tauri).Id")`, e i CSV `tauri-hidden.csv`. La radice di
   Electron è il processo principale, il primo partito (P-25); la lettura è `cpu_pct max` e `mean` della riga `stream:`, **senza
   soglia**, e `last title:` deve dire `msgs=2000 lost=0 holes=0` — il guscio ha riletto la pipe con la chat nascosta;
3. **M5 a pagina intera**: il proprietario preme ⤢ su Scena 3D; il coordinatore:

```bash
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -AttachPid $PID -Seconds 12 -Csv "$CSV\electron-fullpage.csv" | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'
```

   la lettura è `vram_mb max` della riga `rest:` (nessun emettitore: tutta la corsa è `rest`); poi ⤢ di nuovo, e l'app si
   chiude dalla finestra. Lo stesso per Tauri, `tauri-fullpage.csv`.

- [ ] **Passo 7: la domanda A/B sul guscio (D15), e il verdetto su `dockview`**

Il coordinatore mette davanti al proprietario **una tabella a due colonne**, Electron e Tauri, una riga per M1–M5 e Q1–Q4
coi numeri della sezione SP-8 (entrambe le corse) e con le righe di questo compito, **a parole semplici**, e sotto il
consiglio in **una riga**, derivato dall'asimmetria dei costi scritta in ADR-0029 — un costo di capacità si paga più spesso
di un costo di packaging — **letta contro i numeri**: se M3 dà la stessa API su entrambi, M4 passa su entrambi e Q3 e Q4 sono
uguali, decide l'asimmetria e il consiglio è **A**; se Tauri cade su M4, su Q3 o mostra un'altra API, **A** a maggior
ragione; se è Electron a cadere dove Tauri passa, il consiglio è **B**, e dice perché. La domanda: **«A — Electron; B —
Tauri: quale guscio?»** La lettera e le sue parole vanno nell'esito, e il compito 8 le scrive in ADR-0029.

Il verdetto su `dockview` lo dà il protocollo, «Come si decide»: tutte e otto passano → **resta**; una mossa passata con un
accorgimento → resta, «parziale» scritto; un **no all'insieme** → la tela libera: il compito **7-bis** (D12) si scrive in
questa sessione, col proprio pre-controllo, **prima** del compito 8, e la posizione lo dice. Un no tecnico sulla sola mossa 8
si scrive, e se basti a far cadere `dockview` lo dice il proprietario, sull'insieme.

- [ ] **Passo 8: `spikes/RISULTATI.md`, CRLF — le otto righe ⏳, una per volta**

Con `replace_unique.py`, ogni *Trova* è la riga **intera, presa dal file** (le celle `⏳ al compito 7, col proprietario` da
sole non sono uniche); nessuno slot resta, e nessuna cella porta una barra verticale in uno slot (D25):

| Trova | Sostituisci con |
|---|---|
| l'intestazione `## SP-8 — … — misure il <data>; ⏳ il giudizio del proprietario al compito 7` | `## SP-8 — Il guscio della GUI, e l'accettazione di \`dockview\` — misure il <data>, giudizio del proprietario il <data>` |
| la riga del capoverso d'apertura che comincia con `⏳ **Le otto mosse, Q3, Q4, la CPU con la chat nascosta e la VRAM a pagina intera aspettano il proprietario:`, **intera, presa dal file** — è **una** riga nel modello del compito 6, misurato alla rilettura del 2026-09-10 | `Le otto mosse, Q3, Q4, la CPU con la chat nascosta e la VRAM a pagina intera: **col proprietario il <data>**, qui sotto.` |
| la riga `\| M4 — CPU con la chat **nascosta** \| senza soglia \| ⏳ … \| ⏳ … \|` | `\| M4 — CPU con la chat **nascosta**, picco / media (% di un core) \| senza soglia \| <n> / <n> \| <n> / <n> \|` |
| la riga `\| M5 — VRAM con la scena **a pagina intera** \| si riporta \| ⏳ … \| ⏳ … \|` | `\| M5 — VRAM con la scena **a pagina intera**, picco (MB) \| si riporta \| <n> \| <n> \|` |
| la riga `\| Q3 — la finestra staccata si apre dentro il guscio \| ⏳ … \| ⏳ \| ⏳ \|` | `\| Q3 — la finestra staccata si apre dentro il guscio \| per guscio, dal comando ↗ \| <OPENED oppure REFUSED, e se la finestra è comparsa dentro il guscio> \| <lo stesso> \|` |
| la riga `\| Q4 — \`dndStrategy\` che basta nella webview \| ⏳ … \| ⏳ \| ⏳ \|` | `\| Q4 — \`dndStrategy\` che basta nella webview \| per guscio, la mossa 2 col mouse \| <\`'auto'\` basta, oppure serve \`'pointer'\`> \| <lo stesso> \|` |
| `**Le otto mosse — il giudizio del proprietario, con le sue parole:** ⏳ al compito 7, col proprietario.` | il blocco delle mosse qui sotto |
| `**La decisione sul guscio, del proprietario:** ⏳ al compito 7, col proprietario.` | `**La decisione sul guscio, del proprietario, il <data>: <A — Electron, oppure B — Tauri>.** La tabella a due colonne che ha avuto davanti è quella qui sopra, M1–M5 e Q1–Q4; il consiglio del coordinatore era <A oppure B>, derivato dall'asimmetria dei costi di ADR-0029 letta contro i numeri. Le sue parole: «<…>».` |

Il blocco delle mosse:

```markdown
**Le otto mosse — il giudizio del proprietario, con le sue parole**, nel browser il <data>: `dndStrategy` `auto` per le mosse 1–7 e `pointer` per la 8.

| # | La mossa | Esito | La barra, testuale | Le sue parole |
|---|---|---|---|---|
| 1 | il nucleo e la striscia non si spostano | <✅ `passa` oppure ⚠️ `parziale` oppure ❌ `non passa`> | — | «<…>» |
| 2 | sgancia, galleggia, riaggancia | <esito> | `<move 2: …>` | «<…>» |
| 3 | pagina intera e ritorno | <esito> | `<move 3: …>` | «<…>» |
| 4 | in un'altra finestra, dal comando | <esito> | `<move 4 / Q3: …>` | «<…>» |
| 5 | la presa grande col mouse; col tocco <misurato, oppure non misurato perché lo schermo non è touch> | <esito> | — | «<…>» |
| 6 | con la tastiera, `Ctrl+Alt+frecce` | <esito> | `<move 6: …>` | «<…>» |
| 7 | salva, ricarica, ritrova — una misura | <esito> | `<move 7: EQUAL …, oppure DIFFERENT …>` | «<…>» |
| 8 | la pinza afferra la presa grande e la tessera segue — il sì o no **tecnico** | <esito> | `<move 8: pointerdown … on div.bigtab, oppure on span.bigtab-title>` e ciò che la tessera ha fatto | «<…>», senza soglia |

**`dockview`:** <resta — otto su otto; oppure resta, con le mosse parziali dette; oppure esce — un no all'insieme, la tela libera con `interactjs`, il compito 7-bis del piano>.
```

- [ ] **Passo 9: `roadmap.md`, CRLF — la riga SP-8 e la riga della data**

Con `replace_unique.py`: la riga `| SP-8 | …`, **intera, presa dal file**, con `⬜` in coda → la stessa riga con
`✅ **chiuso il <data>**: il guscio è **<il guscio scelto>**, deciso dal proprietario con M1–M5 e Q1–Q4 in mano; \`dockview\` <resta oppure esce> dopo le otto mosse, <tutte passate, oppure con le parziali dette>; M4 <passa oppure non passa> la soglia P3 <su entrambi, oppure su quale>. Le cifre in [RISULTATI.md](../spikes/RISULTATI.md), sezione SP-8`
al posto di `⬜` — **senza cifre**, che stanno in `RISULTATI.md`; e la riga 6, **intera, presa dal file** →
`Ultimo aggiornamento: **<data>**, con la riga **SP-8** chiusa nella tabella degli spike — il compito 7 del [piano della parte 1 del sotto-progetto 2](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md); ADR-0029 si chiude col compito 8.`

- [ ] **Passo 10: le prove, il commit, il push**

```bash
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '⏳'
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '<[^ ]'
grep -c 'con le sue parole' spikes/RISULTATI.md
grep -c '^| SP-8 |' docs/roadmap.md; grep '^| SP-8 |' docs/roadmap.md | grep -c 'chiuso il'
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' spikes/RISULTATI.md docs/roadmap.md
for f in spikes/RISULTATI.md docs/roadmap.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done; git ls-files --eol spikes/RISULTATI.md docs/roadmap.md
git diff --name-only 5ad4634..HEAD -- spikes/gesti/
bash scripts/check-docs.sh; bash scripts/gate.sh
git status --porcelain
```

Atteso: `0` e `0`; `con le sue parole` almeno **2** (SP-7 ne porta una); `1` e `1`; nessuna tabella spezzata; CR = righe,
`i/lf w/crlf`; `spikes/gesti/` intatto — o, col solo ripiego di P-3, `spikes/gesti/relay/src/main.rs` con la voce d'errata e
il richiamo nel protocollo; `OK`, `GATE GREEN`; `git status` che nomina `spikes/RISULTATI.md`, `docs/roadmap.md` e questo
piano — il compito 7-bis, se c'è, è testo di questo piano.

```bash
git add spikes/RISULTATI.md docs/roadmap.md docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git commit -m "guscio(compito 7): SP-8 col proprietario — le otto mosse giudicate con le sue parole (<esito>), la mossa 8 con la mano di SP-7 (<sì o no tecnico>), Q3 e Q4 nei due gusci installati, la CPU con la chat nascosta e la VRAM a pagina intera; la decisione sul guscio: <A, Electron, oppure B, Tauri>; dockview <resta oppure esce>; la riga SP-8 chiusa in roadmap"
git push
```

#### Criterio di chiusura del compito 7

- [ ] nessuna riga ⏳ né slot nella sezione SP-8; le parole del proprietario per ogni mossa e sul guscio; la mossa 7 come `EQUAL` o `DIFFERENT` testuale; la lettera sul guscio
- [ ] Q3 e Q4 per guscio; M4 con la chat nascosta e M5 a pagina intera per guscio; la riga SP-8 chiusa in roadmap senza cifre
- [ ] su un no all'insieme: il compito **7-bis** scritto in questa sessione (D12), col pre-controllo, prima del compito 8 — e la posizione lo dice
- [ ] `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione aggiornata; il revisore in sola lettura ha riletto l'esito e i comandi accanto ai numeri

---

## Compito 8: la chiusura — ADR-0029 `Accepted`, i documenti di stato in ogni casa, la Definizione di «fatto»

**Files:**
- Modify (**CRLF**, tutti con `replace_unique.py`): `docs/adr/0029-guscio-della-gui.md` · `docs/COMPENDIO.md` · `docs/archivio/stato-storico.md` · `docs/README.md` · `docs/HANDOFF.md` · `docs/roadmap.md` · `docs/riferimenti.md` · `docs/audit-2026-08-27.md` · `spikes/GUI-REQUISITI.md`
- Modify (LF): i due disegni — `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md`, `docs/superpowers/specs/2026-09-07-direzione-gui-design.md`; questo piano — la posizione, «Come si riprende»
- Read: la sezione SP-8 di [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md) — i numeri e le parole; [ADR-0029](../../adr/0029-guscio-della-gui.md) per intero, **e i suoi fratelli** [0027](../../adr/0027-stack-della-gui.md), [0030](../../adr/0030-framework-dell-interfaccia.md), [0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md) (gotcha #59: la *Decision* non deve contraddirli — 0027 «nessun tipo condiviso», 0030 le librerie agnostiche, 0033 la quota di presentazione che M5 misura); la §10 del disegno del 2 — la Definizione di «fatto»; P-11, P-16, P-17, P-19, P-20, P-22, P-23, D14, D15, D16, D22, D23, D24

**Interfaces:**
- Consumes: la lettera e le parole del proprietario, l'esito di `dockview`, i numeri di M1–M5 e le righe Q1–Q4 dal compito 7; le due app installate (D19)
- Produces: ADR-0029 `Accepted`; i documenti di stato allineati in **tutte** le case (D22); la Definizione di «fatto» rilanciata coi comandi; le due app disinstallate e i CSV cancellati

⛔ **Un rimedio si chiude su TUTTE le case della frase, non su quella dove lo si è trovato** (audit, «Le decisioni prese
rimediando»): il Passo 1 le elenca col `grep`, e ogni casa che il `grep` rende quel giorno **in più** rispetto a quelle
nominate qui entra nel compito, con un richiamo datato della stessa forma. ⚠️ Nelle sostituzioni, `<data>` è la data del
compito, `<il guscio scelto>` è la lettera del proprietario scritta per esteso — `Electron` o `Tauri` — e nessuno slot dentro
una cella di tabella porta una barra verticale (D25).

- [ ] **Passo 1: le misure prima — le case, i conteggi, la corsa dello spike**

```bash
grep -rn 'Proposed' docs/*.md CLAUDE.md docs/design/*.md spikes/*.md | grep -v '/archivio/' | cut -c1-120
grep -rn 'Tauri o Electron\|Tauri contro Electron\|guscio: aperto\|tranne il guscio' docs/*.md CLAUDE.md spikes/*.md | grep -v '/archivio/' | cut -c1-120
grep -n '38 ADR in stato' docs/HANDOFF.md docs/COMPENDIO.md
grep -c '⏭️' docs/COMPENDIO.md
grep -c '<[^ ]' docs/adr/0029-guscio-della-gui.md
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '⏳\|<[^ ]'
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
git ls-files --eol docs/adr/0029-guscio-della-gui.md docs/COMPENDIO.md docs/archivio/stato-storico.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/riferimenti.md docs/audit-2026-08-27.md spikes/GUI-REQUISITI.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md
for f in docs/adr/0029-guscio-della-gui.md docs/COMPENDIO.md docs/archivio/stato-storico.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/riferimenti.md docs/audit-2026-08-27.md spikes/GUI-REQUISITI.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
for f in docs/COMPENDIO.md docs/HANDOFF.md docs/roadmap.md docs/README.md docs/riferimenti.md docs/adr/0029-guscio-della-gui.md; do awk -v F="$f" 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print F": "NR} {prev=$0}' "$f"; done
```

Atteso, misurato il 2026-09-09 — i numeri di riga si ritrovano con la **frase**, non col numero (gotcha #70): le case della
decisione aperta sono **tredici** (P-19): `docs/COMPENDIO.md` 104, 160, 454; `docs/HANDOFF.md` 155–156, 208–210, 1001,
1052, 1262; `docs/README.md` 168; `docs/roadmap.md` 23–26 e 289, più la riga SP-8 che il compito 7 ha già chiuso;
`docs/audit-2026-08-27.md` 351 (la 29 è un verbale datato di come l'audit fu condotto, e resta); `spikes/GUI-REQUISITI.md`
63–64; le due righe «38 ADR in stato» sono `COMPENDIO.md` 160 e `HANDOFF.md` 1052; le righe con `⏭️` sono **tre** (P-20 —
il puntatore, il punto 2, e un code span nella tabella delle voci aperte) e il conteggio **non cambia** con questo compito;
gli slot `<` dell'ADR sono **zero** prima, e restano zero dopo; la sezione SP-8 è senza `⏳` né slot; il margine è
positivo; `i/lf w/crlf` sui nove, `w/lf` sui due disegni; CR = righe. L'`awk` delle tabelle spezzate stampa **già oggi** sei righe sui file non
toccati (il 2026-09-10: `docs/COMPENDIO.md: 883`, `docs/HANDOFF.md: 1155`, `docs/riferimenti.md: 105`, `108`, `1740`, `2395` —
quattro sono due tabelle consecutive con una riga vuota in mezzo, due sono tabelle spezzate **preesistenti**, HANDOFF 1153–1155 e
riferimenti 103–105, **del proprietario**): l'output si annota, e al Passo 9 si pretende **lo stesso**.

Poi la **corsa dello spike** per la condizione 4 della Definizione di «fatto», **prima** di toccare i documenti — così
`git status` giudica lo spike e non questo compito:

```bash
ROOT=$(cygpath -w "$PWD")
(cd spikes/gui-shell/electron && npm run sync 2>&1 | tail -1)
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$ROOT\spikes\gui-shell\electron\node_modules\electron\dist\electron.exe" -ArgumentList . -WorkingDirectory "$ROOT\spikes\gui-shell\electron" -Seconds 6 | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$ROOT\spikes\gui-shell\tauri\src-tauri\target\release\sp8-tauri.exe" -Seconds 6 | grep -E '^(shell started|attached to pid|emitter started|rest:|stream:|last title:)'
git status --porcelain
git ls-files spikes/gui-shell | grep -c 'lock'
grep -c '/spikes/gui-shell/' .gitignore
```

Atteso: le due corse partono e si fermano da sole; `git status --porcelain` **vuoto**; `4` lockfile (`app/package-lock.json`,
`electron/package-lock.json`, `tauri/package-lock.json`, `tauri/src-tauri/Cargo.lock`); `8` righe di `.gitignore` per lo spike
(due del compito 2, tre del 4, tre del 5).

- [ ] **Passo 2: ADR-0029, CRLF — otto sostituzioni, append-only (vincolo 10)**

| Trova | Sostituisci con |
|---|---|
| `> sostengono, e forzarla su un argomento sarebbe contro il metodo di questo repository.` | la stessa riga, poi a capo `> ✅ **RICHIAMO DEL <data>:** chiusa con **SP-8** — la *Decision* qui sotto; l'avviso resta com'era, perché è la storia di questa decisione.` |
| `- **Status:** Proposed` | `- **Status:** Accepted` — **esattamente così**: `check-docs.sh` conta `^- \*\*Status:\*\* Accepted` (P-24) |
| `_(da prendere)_` | il blocco della decisione qui sotto |
| `**argomenti, non misure**, ed è per questo che l'ADR resta \`Proposed\`.` | la stessa riga più ` ✅ **RICHIAMO DEL <data>:** le misure sono arrivate — SP-8, la *Decision* qui sopra — e lo stato è \`Accepted\`; la raccomandazione <ha retto, oppure è stata ribaltata dai numeri: quali>.` |
| `### Come si chiude` | la stessa riga, una riga vuota, poi `✅ **Eseguito il <data>: SP-8**, coi criteri congelati prima in [\`spikes/gui-shell/PROTOCOLLO.md\`](../../spikes/gui-shell/PROTOCOLLO.md) — e con **quattro** righe qualitative, Q1–Q4, non le due della §2 del disegno del 2 com'era approvata il 2026-09-06: Q3 e Q4 nascono dalla §4 della stella polare.` |
| `principale a favore di Electron cade e la decisione si ribalta.` | la stessa riga più ` ⚠️ **RICHIAMO DEL <data>:** non misurato — Linux non c'è (decisione C del disegno del 2); è l'**innesco** scritto nella *Decision*.` |
| `    è \`Proposed\`, e \`check-docs.sh\` lo segnala a ogni audit finché non si chiude.` | la stessa riga più ` ✅ **Chiusa il <data>**: non per inerzia, con SP-8.` |
| `    un discriminante nuovo per questo ADR, la misura **M5** qui sopra.` | la stessa riga, una riga vuota, poi il blocco `Negative (accettate)` qui sotto |

Il blocco della decisione — i numeri **come nell'esito**, entrambe le corse, e le parole testuali:

```markdown
✅ **DECISA il <data> dal proprietario, con SP-8 in mano — <il guscio scelto>.** I criteri, congelati **prima** della
misura, in [`spikes/gui-shell/PROTOCOLLO.md`](../../spikes/gui-shell/PROTOCOLLO.md); l'esito intero, con le evidenze e
le osservazioni, in [`spikes/RISULTATI.md`](../../spikes/RISULTATI.md), sezione SP-8; la macchina è quella di ADR-0002,
la data la dice la sezione. Le sue parole: «<le parole del proprietario, testuali>».

| # | Misura, o riga | Electron | Tauri |
|---|---|---|---|
| M1 | RAM a riposo e sotto flusso — media / picco in MB, due corse | riposo <n> / <n>; <n> / <n> — flusso <n> / <n>; <n> / <n> | riposo <n> / <n>; <n> / <n> — flusso <n> / <n>; <n> / <n> |
| M2 | cartella installata e installatore, MB | <n> (<n> file); installatore <n> | <n> (<n> file); installatore <n> |
| M3 | fps medi / minimi su 30 s a riposo, e l'**API ottenuta** — **Windows** | <n> / <n>, `<api>` | <n> / <n>, `<api>` |
| M4 | P3 con rendering vero — picco / media in % di un core, due corse; la soglia è il picco **< 25 %** | <n> / <n>; <n> / <n> — <passa, parziale, oppure non passa>; con la chat nascosta <n> / <n> | <n> / <n>; <n> / <n> — <esito>; con la chat nascosta <n> / <n> |
| M5 | VRAM in MB — a riposo, due corse; con la scena a pagina intera | <n>; <n> — a pagina intera <n> | <n>; <n> — a pagina intera <n> |
| Q1 | chi decodifica `bincode` lato GUI | il processo principale Node, `bincode-ts` (M-11 di ADR-0037) | il guscio Rust, col decodificatore del kernel — una terza compilazione delle crate (decisione 57 della stella polare) |
| Q2 | le webview | il Chromium impacchettato, `<Chrome/…>` | WebView2 `<pv>`; **WebKitGTK**: <lo stato di WebGPU letto il <data> — le fonti in `riferimenti.md`, sezione SP-8> |
| Q3 | la finestra staccata si apre **dentro** il guscio, dal comando | <sì, oppure no> | <sì, oppure no> |
| Q4 | `dndStrategy` che basta nella webview | <`'auto'`, oppure `'pointer'`> | <`'auto'`, oppure `'pointer'`> |

**L'innesco Linux, scritto.** La metà Linux di M3 e M5 **non è misurata** (decisione C del disegno del 2): al primo
Linux vero si rimisurano M3 e M5 su quel sistema, con lo stesso protocollo; se M3 mostra la stessa API grafica su
entrambe le piattaforme con Tauri, questa decisione si **riapre con un ADR nuovo** che la superi (`Superseded by`),
non con una conversazione.

**`dockview`:** <resta — le otto mosse passano, le parole del proprietario in `spikes/RISULTATI.md`; oppure resta, con le mosse parziali dette lì; oppure esce — un no all'insieme, la tela libera con `interactjs`, il compito 7-bis del piano>.
```

Il blocco `Negative (accettate)` — **si scrive il solo elenco del vincitore**:

```markdown
- **Negative (accettate)** — per **<il guscio scelto>**, il guscio scelto:
```

Se **Electron**:

```markdown
  - un **terzo runtime**, Node, accanto a Rust e Python: il processo principale ha accesso pieno all'OS, e il ponte del
    preload è l'unica porta fra la pagina e il guscio — va tenuta stretta, e la webview carica solo contenuto locale
    nostro, come il punto comune qui sopra già impone.
  - Chromium **impacchettato**: la taglia di M2, e gli aggiornamenti del motore a carico nostro, a ogni release.
  - la metà Linux **non misurata**: l'innesco qui sopra; con Electron l'API grafica su Linux dipende da un flag che
    passiamo noi, e resta da provare.
  - chi decodifica `bincode` è Node (Q1): `bincode-ts`, fermo alla 1.0.0 del 2025-07-17 (ADR-0037), è la dipendenza che il
    canale `ipc` paga.
```

Se **Tauri**:

```markdown
  - il rischio di **capacità** su Linux **non misurato**: WebKitGTK e WebGPU, l'innesco qui sopra.
  - la webview del sistema fuori dal nostro controllo: versione e flag sono dell'utente, e su Windows M3 e M5 valgono per
    **questo** WebView2 (Q2), non per quello che l'utente avrà.
  - nessuna familiarità del proprietario, che si somma a Rust da imparare per il core.
  - chi decodifica `bincode` è il guscio Rust (Q1): una terza compilazione delle crate del kernel in `gui/shell/`
    (decisione 57 della stella polare), e il ponte alla webview passa dagli eventi di Tauri.
```

- [ ] **Passo 3: il compendio, CRLF — §4, §5, §6, §8, l'intestazione (D16, P-17)**

| Trova | Sostituisci con |
|---|---|
| `Sono **39 ADR**, di cui **38 ADR in stato Accepted** e uno \`Proposed\` (0029).` | `Sono **39 ADR**, e **39 ADR in stato Accepted** — l'ultimo, 0029, chiuso il <data> con SP-8.` (P-11: la guardia dei conteggi legge 39 e 39) |
| `\| gui — **guscio** \| ⚠️ **APERTO**: Tauri o Electron \| ADR-0029, \`Proposed\` — **non blocca nulla** \|` | `\| gui — **guscio** \| **<il guscio scelto>** — deciso dal proprietario il <data> con SP-8: M1–M5 e Q1–Q4 su Windows, l'innesco Linux nell'ADR \| ADR-0029 \|` |
| `e non tocca la GUI.` (la fine della voce 0029 di §5, unica) | `e non tocca la GUI. ✅ **RICHIAMO DEL <data>: CHIUSA — <il guscio scelto>**, deciso dal proprietario con M1–M5 e Q1–Q4 misurate da SP-8 su Windows, coi criteri congelati prima; l'innesco Linux nell'ADR; \`dockview\` <resta oppure esce> dopo le otto mosse. Il testo sopra resta com'era.` |
| `⏭️ **IL PROSSIMO PASSO: IL PIANO DEL SOTTO-PROGETTO 2, IN DUE PARTI — LA RILETTURA DEL PROPRIETARIO È FATTA IL 2026-09-09.** I due disegni sono` | `⏭️ **IL PROSSIMO PASSO: LA PARTE 2 DEL PIANO DEL SOTTO-PROGETTO 2 — LA PARTE 1, SP-8, È ESEGUITA IL <data>.** I due disegni sono` |
| la riga della tabella «Chiuso» che comincia con `\| la **rilettura del proprietario**`, **intera, presa dal file** | la stessa riga, poi a capo `\| la **parte 1 del piano del 2** — SP-8: il guscio **<il guscio scelto>** deciso dal proprietario, ADR-0029 \`Accepted\`, \`dockview\` <resta oppure esce> dopo le otto mosse \| <data> \| il [piano](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md), «A che punto è» e l'errata; la sezione SP-8 di \`spikes/RISULTATI.md\`; ADR-0029 \|` |
| il **punto 2** del prossimo passo — dalla riga `2. ⏭️ **la prossima sessione — decisione 48 del 2026-09-09:**` fino alla riga del richiamo `✅ **RICHIAMO DEL 2026-09-10, alla chiusura della sessione che l'ha completato:**` compresa, **quattro righe prese dal file** | `2. ✅ **la parte 1 del piano del 2, eseguita il <data>** — [piano](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md): SP-8, ADR-0029 chiuso, \`dockview\` <resta oppure esce>; ⏭️ **la parte 2**: i pezzi 2–9 della §3 della stella polare, scritta **ora** coi numeri in mano — chi decodifica (Q1), \`gui/shell/\` se Tauri (decisione 57), la CI su Windows e \`cargo audit\` più \`npm audit\` (D13 del piano) — con lo stesso pre-controllo, in una sessione nuova; le righe nella §12 di questo file, in \`README.md\`, nella roadmap e in tracciabilità entrano con essa (D14 del piano);` — le quattro righe com'erano vanno nel blocco d'archivio qui sotto |
| `3. ADR-0029 si chiude con M1–M5 all'inizio del 2 (voce 0029 della §5) — **e in parallelo AUD-004**, l'ADR del proprietario` | `3. ✅ ADR-0029 **chiuso il <data>** con SP-8 (voce 0029 della §5) — **e in parallelo AUD-004**, l'ADR del proprietario` |
| la riga di §8 che comincia con `\| ❌ **rifare gli spike SP-5, SP-6 e SP-7**`, **intera, presa dal file** | `\| ❌ **rifare gli spike SP-5, SP-6, SP-7 e SP-8** \| esiti, versioni e comandi in \`spikes/RISULTATI.md\` — coi **seed** per SP-5 e SP-6, che SP-7 e SP-8 non hanno; per SP-7 e SP-8 i protocolli congelati in \`spikes/gesti/PROTOCOLLO.md\` e \`spikes/gui-shell/PROTOCOLLO.md\` \|` |
| `L'unica aperta è **M5** (senza trattino), e richiede una GUI \|` | `L'unica aperta era **M5** (senza trattino) ✅ **misurata da SP-8 il <data>** su Windows; la metà Linux è l'innesco scritto in ADR-0029 \|` |
| l'intestazione — la riga 21, da `**Aggiornato il` a `Manutenzione: §13.`, **intera, presa dal file** | `**Aggiornato il <data>**, con la **parte 1 del piano del 2 eseguita** — SP-8, ADR-0029 chiuso, il guscio **<il guscio scelto>** — e il puntatore della §6 alla parte 2; l'ultimo contenuto di merito è la voce 0029 della §5. Il testo com'era, con le sedici riprese, è in [\`archivio/stato-storico.md\`](archivio/stato-storico.md). Manutenzione: §13.` |

L'**archivio** (`docs/archivio/stato-storico.md`, CRLF): con `replace_unique.py`, *Trova* l'**ultima riga del file**,
presa con `tail -1` e verificata unica con `grep -c`; *Sostituisci* con la stessa riga, una riga vuota, e il blocco:

```markdown
## L'intestazione del compendio e il punto 2 del suo prossimo passo, com'erano — archiviati il <data>

⚠️ **Vero il giorno in cui fu scritto.** Usciti dalla testa e dalla §6 del compendio alla chiusura della parte 1 del piano
del sotto-progetto 2 (compito 8, decisione D16 del piano), parola per parola; i link riscritti per questa cartella.

### L'intestazione, riga 21

<la riga com'era>

### Il punto 2 del prossimo passo, coi suoi richiami

<le quattro righe com'erano>
```

I link del **solo** testo mosso si riscrivono per `archivio/`: `(archivio/stato-storico.md)` → `(stato-storico.md)`,
`(superpowers/plans/…)` → `(../superpowers/plans/…)`, `(superpowers/specs/…)` → `(../superpowers/specs/…)`; poi
`grep -n '\](\s*)' docs/archivio/stato-storico.md` non deve stampare nulla (vicolo cieco della dodicesima ripresa).
Dopo ogni tocco al compendio: `check-docs.sh` e il margine. ⛔ **Se il tetto va rosso si toglie prosa dalla §6, non si alza
il tetto** (vincolo 11): l'intestazione corta libera più di quanto le righe nuove consumano (P-17, P-22).

- [ ] **Passo 4: `README.md`, `HANDOFF.md`, `roadmap.md`, l'audit, `GUI-REQUISITI.md` — CRLF, le altre case (P-19)**

| File | Trova | Sostituisci con |
|---|---|---|
| `docs/README.md` | `\| [0029](adr/0029-guscio-della-gui.md) \| Guscio della GUI: Tauri o Electron \| ⚠️ **Proposed** \|` | `\| [0029](adr/0029-guscio-della-gui.md) \| Guscio della GUI: <il guscio scelto>, deciso con SP-8 \| Accepted \|` |
| `docs/HANDOFF.md` | le **due righe** `> ✅ **Nessuna voce aperta resta nella spec.** L'unica decisione ancora \`Proposed\` del` e `> progetto è **ADR-0029**, il guscio della GUI, che non tocca il sotto-progetto 1.` | le stesse due righe, poi a capo `> ✅ **RICHIAMO DEL <data>:** anche ADR-0029 è \`Accepted\` — <il guscio scelto>, con SP-8.` |
| `docs/HANDOFF.md` | `([ADR-0029](adr/0029-guscio-della-gui.md), \`Proposed\`) e non blocca nulla.` | la stessa riga più ` ✅ **RICHIAMO DEL <data>:** chiuso — **<il guscio scelto>**, SP-8; lo stack è deciso per intero.` |
| `docs/HANDOFF.md` | `\| **0029** \| ⚠️ **guscio: aperto** — Tauri o Electron \| **niente**: sono argomenti, non misure. È il motivo per cui è \`Proposed\` \|` | `\| **0029** \| ⚠️ **guscio: aperto** — Tauri o Electron ✅ **chiuso il <data>: <il guscio scelto>** \| **niente**: sono argomenti, non misure. È il motivo per cui era \`Proposed\`; poi **SP-8**, M1–M5 e Q1–Q4 \|` |
| `docs/HANDOFF.md` | `38 ADR in stato \`Accepted\`` | `39 ADR in stato \`Accepted\`` (P-11: nello **stesso** commit dello `Accepted` nell'ADR, o il cancello va rosso) |
| `docs/HANDOFF.md` | `\| ⚠️ **Guscio: Tauri o Electron** \| ADR-0029 \`Proposed\`, misure **M1–M5** \| no \|` | `\| ~~Guscio: Tauri o Electron~~ \| ✅ **ADR-0029: <il guscio scelto>**, chiuso il <data> con SP-8 — M1–M5 e Q1–Q4 \| — \|` |
| `docs/roadmap.md` | le **quattro righe** dello «Stato in una riga», da `> Spec del kernel **completa e approvata** (§0–§10, 39 ADR). Stack deciso **tranne il` a `> \`Proposed\`) e **non blocca nulla**.`, prese dal file | `> Spec del kernel **completa e approvata** (§0–§10, 39 ADR). Stack deciso **per intero**: core in **Rust**,` a capo `> interfaccia web in **Vue 3**, worker ML in **Python**, guscio **<il guscio scelto>** —` a capo `> [ADR-0029](adr/0029-guscio-della-gui.md), \`Accepted\` il <data> con **SP-8**.` |
| `docs/roadmap.md` | `\| ⚠️ **Guscio della GUI: Tauri o Electron** \| **ADR-0029, \`Proposed\`.** Si chiude con **cinque** misure M1–M5 all'inizio del sotto-progetto 2 \| non blocca il sotto-progetto 1 \|` | `\| ~~Guscio della GUI: Tauri o Electron~~ \| ✅ **[ADR-0029](adr/0029-guscio-della-gui.md): <il guscio scelto>**, chiuso il <data> con **SP-8** — M1–M5 e Q1–Q4 su Windows, l'innesco Linux nell'ADR \| non bloccava il sotto-progetto 1 \|` |
| `docs/roadmap.md` | nella riga di **questo piano** nella tabella dei piani, la cella di stato `✅ **scritto il 2026-09-09 e riletto il 2026-09-10**, in tre sessioni — la prima fino al compito 5, la seconda i compiti 6–8, la terza la revisione; ⏳ **da eseguire in una sessione nuova**, un subagente fresco per compito \|` | `✅ **scritto il 2026-09-09, riletto il 2026-09-10, eseguito il <data>** — \`GATE GREEN\` a ogni compito; SP-8 nella tabella degli spike, ADR-0029 chiuso: **<il guscio scelto>** \|` |
| `docs/roadmap.md` | la riga 6, `Ultimo aggiornamento: …`, **intera, presa dal file** | `Ultimo aggiornamento: **<data>**, con ADR-0029 chiuso — lo «Stato in una riga», la riga delle decisioni da prendere, e la riga del [piano della parte 1 del sotto-progetto 2](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md) nella tabella dei piani, eseguito: il suo compito 8.` |
| `docs/audit-2026-08-27.md` | `[ADR-0029](adr/0029-guscio-della-gui.md) fermo a \`Proposed\`, i due \`#[ignore]\`, il` | `[ADR-0029](adr/0029-guscio-della-gui.md) fermo a \`Proposed\` (✅ chiuso il <data> con SP-8), i due \`#[ignore]\`, il` |
| `spikes/GUI-REQUISITI.md` | `ancora aperta**: [ADR-0029](../docs/adr/0029-guscio-della-gui.md), \`Proposed\`. Il` | `ancora aperta**: [ADR-0029](../docs/adr/0029-guscio-della-gui.md), \`Proposed\` — ✅ **chiusa il <data> con SP-8: <il guscio scelto>**. Il` |

- [ ] **Passo 5: i due disegni, LF — i richiami datati (P-16)**

| File | Trova | Sostituisci con |
|---|---|---|
| il disegno del 2, §2 | `protocollo e come si decide non si ricopiano:** la casa è la **§4 della [stella polare](2026-09-07-direzione-gui-design.md)**.` | la stessa riga, una riga vuota, poi `✅ **RICHIAMO DEL <data>, alla chiusura della parte 1 del piano:** SP-8 eseguito — M1–M5 e Q1–Q4 in ADR-0029, chiuso **<il guscio scelto>**; le otto mosse in \`spikes/RISULTATI.md\`, \`dockview\` <resta oppure esce>; la parte 2 del piano si scrive ora, coi numeri.` |
| il disegno del 2, §10 | `l'esecuzione in una sessione nuova, un subagente fresco per compito (decisione 48).` | la stessa più ` ✅ **PARTE 1 ESEGUITA il <data>:** il verbale nel piano — «A che punto è», l'errata, «Come si riprende»; la parte 2 si scrive ora, dalla §6 del compendio.` |
| la stella polare, §4 | la riga `### §4 — Lo spike di accettazione di \`dockview\` · approvata il 2026-09-08 (A, decisione 24); la mossa 8 il 2026-09-09 (A, decisione 25)`, intera | la stessa riga, una riga vuota, poi `✅ **RICHIAMO DEL <data>, alla chiusura della parte 1 del piano del 2:** le otto mosse provate dal proprietario — <quante passano, quali parziali, il sì o no tecnico della mossa 8>; \`dockview\` <resta oppure esce>; le parole in \`spikes/RISULTATI.md\`, sezione SP-8; il guscio **<il guscio scelto>**, ADR-0029. Le mosse qui sotto restano com'erano approvate.` |
| la stella polare, la tabella dello stato | `è cancello, non codice \|` (la cella «Atteso» della riga «codice e spec non toccati», unica) | `è cancello, non codice ✅ **RICHIAMO DEL <data>:** e \`spikes/gui-shell/\`, \`.gitignore\`, \`spikes/RISULTATI.md\`, \`spikes/GUI-REQUISITI.md\` per SP-8 — la parte 1 del piano del 2; \`crates/\` intatto \|` |

- [ ] **Passo 6: `riferimenti.md`, CRLF — le versioni del giorno dello spike e le fonti di Q2 (D23)**

Con `replace_unique.py`: *Trova* la riga `## Cosa NON abbiamo adottato, e perché` (unica); *Sostituisci* con la sezione qui
sotto, poi `---`, una riga vuota, e la stessa riga. Le tre fonti sono quelle lette al Passo 5 del compito 6, coi loro URL
e la data di lettura; nessun codice `F` — i codici di questo file sono per sezione, e questa ne fa a meno. La sezione
ha dentro una recinzione a tre accenti per il comando: qui è mostrata dentro una recinzione a **quattro**.

````markdown
## SP-8 — il guscio della GUI: le versioni del giorno dello spike, e le fonti su WebKitGTK (Q2) — <data>

Il compito 6 del [piano della parte 1 del sotto-progetto 2](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md)
ha rilanciato il comando qui sotto il <data>; le versioni **installate** stanno nei lockfile di `spikes/gui-shell/` e nella
sezione SP-8 di [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md); i numeri di M1–M5 e le righe Q1–Q4 in
[ADR-0029](adr/0029-guscio-della-gui.md), casa unica. Le ultime al registro quel giorno: <l'uscita del comando, una riga per pacchetto>.

```
<il comando del Passo 2 del compito 6, testuale>
```

**Le fonti di Q2 — lo stato di WebGPU su WebKitGTK, la lettura che sostituisce la misura Linux (decisione C del disegno del 2):**

| Fonte | Letta il | Che cosa dice | Conseguenza |
|---|---|---|---|
| [WebGPU Implementation Status](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status) | <data> | «<la riga su WebKit / Linux, breve>» | Q2 di ADR-0029; da rileggere al primo Linux vero, con l'innesco scritto lì |
| [WebKitGTK — news](https://webkitgtk.org/news.html) | <data> | «<l'ultima versione stabile, e ciò che dice di WebGPU, oppure che non lo nomina>» | idem |
| [Tauri — Webview Versions](https://v2.tauri.app/reference/webview-versions/) | <data> | «<la riga su Linux>» | idem; la stessa pagina è già la prima fonte di ADR-0029, letta il 2026-08-06 |
````

- [ ] **Passo 7: un gotcha nuovo, se c'è — `HANDOFF.md`, CRLF**

```bash
awk '/^## I gotcha/{s=1; next} s&&/^## /{s=0} s&&/^\| [0-9]+ \|/{c++} END{print c}' docs/HANDOFF.md
awk '/^## ⚠️ L.errata di questo piano/{s=1} s&&/^## Il pre-controllo/{s=0} s&&/^\| \*\*E[0-9]+\*\* \|/' docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md | cut -c1-120
```

Se l'errata di questo piano porta un fatto che **nessun documento diceva** e che ha fatto sbagliare un passo — il criterio
della tabella dei gotcha — è un gotcha nuovo: una riga nella sezione «I gotcha» di `HANDOFF.md` col numero successivo a
quello che il primo comando rende, nella forma delle righe vicine, e **niente** nel compendio (§13, casa unica). Altrimenti
niente, e il rapporto del compito lo dice.

- [ ] **Passo 8: la Definizione di «fatto» della parte 1, copiata dalla §10 del disegno del 2, condizione per condizione coi comandi**

| # | Condizione (§10 del disegno del 2) | Chi la verifica, e come |
|---|---|---|
| 1 | il protocollo dello spike congelato in `spikes/gui-shell/PROTOCOLLO.md` **al primo commit di codice**, con M1–M5, Q1–Q4 e le otto mosse | `git log` sul protocollo e su `app/`: la prima riga tocca il **solo** protocollo (compito 1) e viene prima di ogni riga di `app/`; le cinque righe M, le quattro Q e le otto mosse contate — il blocco 1 qui sotto |
| 2 | i numeri di M1–M5 e le righe Q1–Q4 in ADR-0029, lo stato ad `Accepted` con l'innesco Linux scritto; la riga del guscio nella §4 del compendio chiusa | `check-docs.sh` senza ADR in `Proposed`; la riga `Status`, la parola «innesco Linux», le nove righe della tabella, la riga del guscio nel compendio senza `APERTO` — il blocco 2 |
| 3 | le otto mosse giudicate dal proprietario con le sue parole in `spikes/RISULTATI.md`, la mossa 7 come due JSON uguali; l'esito di `dockview` scritto — resta, o la tela libera | sulla sezione SP-8 delimitata con l'`awk`: le parole, le otto righe delle mosse, la riga `move 7`, la riga `dockview`, nessuno slot — il blocco 3 |
| 4 | `.gitignore` con le cartelle di build dello spike; i lockfile dello spike committati | la corsa dello spike del Passo 1: `git status --porcelain` vuoto, `4` lockfile, `8` righe |
| 5 | **nessun codice di prodotto toccato** dalla parte 1 | il `git diff --stat` dei vincoli 1 e 2 — il blocco 5 |
| 6 | fine-riga rimisurati per ogni file toccato | il ciclo dei CR del Passo 9, contro la mappa dei file; `0` sui file LF; `git ls-files --eol` invariato |
| 7 | la parte 2 del piano scritta **dopo** la misura, con lo stesso pre-controllo | non è di questo piano: il puntatore della §6 lo dice — il blocco 7; e la sezione «Dopo il compito 8» |

I comandi, per condizione, con l'esito atteso accanto:

```bash
# 1
git log --format='%h %ad %s' --date=short --reverse -- spikes/gui-shell/PROTOCOLLO.md spikes/gui-shell/app | head -3
grep -c '^| \*\*M[1-5]\*\* |' spikes/gui-shell/PROTOCOLLO.md      # 5
grep -c '^| \*\*Q[1-4]\*\* |' spikes/gui-shell/PROTOCOLLO.md      # 4
grep -c '^| [1-8] |' spikes/gui-shell/PROTOCOLLO.md               # 8
# 2
bash scripts/check-docs.sh | grep -A2 'ADR still in Proposed'    # the heading, then "  (none)" (P-24): no file listed
grep -c '^- \*\*Status:\*\* Accepted' docs/adr/0029-guscio-della-gui.md   # 1
grep -c 'innesco Linux' docs/adr/0029-guscio-della-gui.md                  # at least 1
awk '/^## Decision/{s=1} s&&/^### Come si chiude/{s=0} s' docs/adr/0029-guscio-della-gui.md | grep -c '^| M[1-5] |\|^| Q[1-4] |'   # 9 -- delimited: «Come si chiude» carries four such rows already
grep -n 'gui — \*\*guscio\*\*' docs/COMPENDIO.md                            # the row, without APERTO
# 3
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c 'con le sue parole'            # at least 1
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '^| [1-8] |'                    # 8
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c 'move 7: EQUAL\|move 7: DIFFERENT'   # 1
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '^\*\*`dockview`:\*\*'          # 1
awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '⏳\|<[^ ]'                       # 0
# 5
git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md   # nothing
# 7
grep -c 'la parte 2' docs/COMPENDIO.md                             # at least 1
```

- [ ] **Passo 9: l'ambiente ripulito, le prove, il commit, il push**

Le due app installate si **disinstallano** (D19) — i nomi dei disinstallatori si **leggono** — e i CSV si cancellano:

```bash
EL_DIR='<la cartella di Electron, dalla riga M2 dell'esito>'
TA_DIR='<la cartella di Tauri, dalla riga M2>'
powershell -NoProfile -Command "Get-ChildItem '$EL_DIR','$TA_DIR' -Filter '*nstall*.exe' | Select-Object -ExpandProperty FullName"
powershell -NoProfile -Command "Start-Process -Wait -FilePath '<il disinstallatore di Electron letto sopra>' -ArgumentList '/S'"
powershell -NoProfile -Command "Start-Process -Wait -FilePath '<il disinstallatore di Tauri letto sopra>' -ArgumentList '/S'"
powershell -NoProfile -Command "Get-ChildItem \$env:LOCALAPPDATA\Programs,\$env:LOCALAPPDATA -Directory -Filter 'sp8-*' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty FullName"
rm -r "$HOME/sp8-measure"
```

Atteso: due disinstallatori — dedotto `Uninstall sp8-electron.exe` e `uninstall.exe`; dopo, nessuna cartella `sp8-*` (una
cartella vuota lasciata da NSIS si scrive nel rapporto e si toglie con `Remove-Item`); gli installatori restano in `out/` e in
`bundle/nsis/`, ignorati e rifacibili — è ciò che rende rifacibili i numeri dell'ADR.

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/adr/0029-guscio-della-gui.md docs/COMPENDIO.md docs/archivio/stato-storico.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/riferimenti.md docs/audit-2026-08-27.md spikes/GUI-REQUISITI.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
for f in docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
git ls-files --eol docs/adr/0029-guscio-della-gui.md docs/COMPENDIO.md docs/archivio/stato-storico.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/riferimenti.md docs/audit-2026-08-27.md spikes/GUI-REQUISITI.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md
grep -c '⏭️' docs/COMPENDIO.md
grep -c '<[^ ]' docs/adr/0029-guscio-della-gui.md
grep -n '\](\s*)' docs/archivio/stato-storico.md docs/riferimenti.md
for f in docs/COMPENDIO.md docs/HANDOFF.md docs/roadmap.md docs/README.md docs/riferimenti.md docs/adr/0029-guscio-della-gui.md; do awk -v F="$f" 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print F": "NR} {prev=$0}' "$f"; done
grep -rn 'Proposed' docs/*.md CLAUDE.md docs/design/*.md spikes/*.md | grep -v '/archivio/' | grep -v 'RICHIAMO\|chius\|era `Proposed`\|verbale\|fu \|Come è stato condotto\|DECISIONE APERTA\|Nessuna voce aperta resta' | cut -c1-120
git status --porcelain
```

Atteso: `OK` — la guardia dei conteggi legge 39 e 39 nei sei file (P-11) e nessun ADR in `Proposed`; `GATE GREEN`; margine
positivo; CR = righe sui nove, `0` sui tre LF; `ls-files --eol` invariato; il conteggio di `⏭️` **uguale** a quello del
Passo 1; `0` slot nell'ADR; nessun link vuoto; l'`awk` delle tabelle spezzate rende **lo stesso output del Passo 1** — le sei righe preesistenti — e
nessuna riga nuova sui file toccati; il `grep` di `Proposed` che rende **solo** righe con
un richiamo datato, o la riga 29 dell'audit (il verbale di come fu condotto) — il filtro toglie anche le **due** righe il cui
richiamo sta poche righe **sotto**: la testa della voce 0029 della §5 del compendio («DECISIONE APERTA», chiusa dal Passo 3 in
coda alla voce) e la riga «Nessuna voce aperta resta nella spec» di `HANDOFF.md` (chiusa dal Passo 4 sulla riga nuova sotto),
misurato alla rilettura del 2026-09-10; `git status` che nomina i **dodici** file di
questo compito.

```bash
git add docs/adr/0029-guscio-della-gui.md docs/COMPENDIO.md docs/archivio/stato-storico.md docs/README.md docs/HANDOFF.md docs/roadmap.md docs/riferimenti.md docs/audit-2026-08-27.md spikes/GUI-REQUISITI.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md
git commit -m "guscio(compito 8): la chiusura della parte 1 — ADR-0029 Accepted con la decisione del proprietario (<il guscio scelto>), la tabella M1–M5 e Q1–Q4, l'innesco Linux e Negative (accettate); i totali a 39; il compendio (§4, §5, §6, §8, l'intestazione riscritta corta e archiviata); README, HANDOFF, roadmap, l'audit e GUI-REQUISITI in ogni casa; i richiami nei due disegni; le versioni del giorno e le fonti di Q2 in riferimenti.md; la Definizione di «fatto» rilanciata coi comandi; le due app disinstallate"
git push
```

#### Criterio di chiusura del compito 8

- [ ] le sette condizioni della Definizione di «fatto» verificate **coi comandi** del Passo 8, ed elencate nel rapporto con l'output
- [ ] ADR-0029 `Accepted`, riletto dal revisore contro 0027, 0030 e 0033 (gotcha #59) e non solo contro l'esito; `check-docs.sh` senza ADR in `Proposed`
- [ ] nessuna casa della decisione aperta è rimasta senza richiamo: il `grep` del Passo 1 rilanciato al Passo 9
- [ ] il compendio sotto il tetto, l'intestazione corta col testo com'era in archivio, il puntatore della §6 alla parte 2 in un posto solo
- [ ] le due app disinstallate, i CSV cancellati; `GATE GREEN`, `check-docs.sh` → `OK`, fine-riga rimisurati, commit pushato, posizione del piano a otto ✅ — o nove, col 7-bis

---

## Dopo il compito 8

⛔ **Che cosa venga dopo NON è scritto qui:** è uno stato, e la sua casa unica è la §6 del
[compendio](../../COMPENDIO.md), che il compito 8 scrive: la **parte 2** del piano del 2 — i pezzi 2–9 della §3 della
stella polare — scritta **dopo** la misura, coi numeri in mano e con lo stesso pre-controllo (punto 8 della §10 del
disegno del 2). Le voci che questo piano sa e non chiude stanno nella sezione omonima, e per nessuna il chiusore è questo
piano; il compito 7-bis esiste solo su un no all'insieme delle mosse (D12); la metà Linux di M3 e M5 è l'innesco scritto
in ADR-0029; X-1 e X-3 sono compiti della parte 2 (D13).

## Come si riprende — il diario di questo piano, coi comandi

### La terza chiusura — 2026-09-10: la revisione del piano intero FATTA, i tre spostamenti di stato FATTI — si ESEGUE

⛔ **DA SAPERE SUBITO.** Niente è a metà, **nemmeno questo file**: il piano è scritto per intero e **riletto** — la revisione
del punto 4 della seconda chiusura (copertura dei disegni, segnaposto, nomi fra i compiti, ogni *Trova* rilanciato) e i tre
spostamenti di stato del punto 5 sono **fatti** in questa sessione, con la data vera; ciò che la revisione ha corretto è
scritto **nei compiti** (la tabella qui sotto), l'errata resta vuota perché il piano non è eseguito. Albero pulito, nessuno
stash, nessuna operazione git in corso, **nessun codice toccato**. Lo scratchpad è ripulito: tutto ciò che vale sta qui e nel
commit. ⛔ **L'esecuzione — compito 1 con un subagente fresco — è di una sessione NUOVA** (decisione 48 della stella polare).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit delle tre sessioni del piano | `git log --oneline 5ad4634..HEAD` — **tre**: `b50e8b8`, il piano a metà; `4d626ea`, i compiti 6–8; poi questo, la revisione e gli spostamenti di stato |
| Codice, cancello, CI, spike | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ spikes/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla |
| Documenti toccati da questa sessione | `git diff --stat 4d626ea..HEAD -- docs/` → questo piano, `docs/roadmap.md` (la riga del piano nella tabella dei piani e la riga 6), il disegno del 2 (la §10, il richiamo «PIANO DELLA PARTE 1 SCRITTO») e `docs/COMPENDIO.md` (il richiamo nel punto 2 del prossimo passo della §6, da ⏳ a ✅) — i tre spostamenti di stato |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura (baseline, letto dal log nello scratchpad) e alla chiusura, prima del commit; `bash scripts/check-docs.sh` → `OK`. Si rilanciano, non si citano |
| Fine-riga | questo piano e il disegno del 2 **LF** — `tr -cd '\r' < <file> \| wc -c` → `0`; roadmap e compendio CRLF con CR = righe, `git ls-files --eol` → `i/lf w/crlf`, invariato |
| Margine del compendio | il comando del vincolo 11: `10313` all'apertura di questa sessione — **non** i `10395` di P-22: il richiamo della seconda chiusura era più lungo di quello della prima, non «della stessa taglia» — e il valore alla chiusura lo dà il comando; il compito 8 lo rimisura comunque (D18) |
| Le righe `⏭️` del compendio | `grep -c '⏭️' docs/COMPENDIO.md` → `3` prima e dopo lo spostamento (c): il richiamo riscritto è ✅, senza marcatore (P-20, D24) |
| Debito lasciato | **dichiarato**: le righe **dedotte** e non provate — P-3, P-6, P-7, P-21, P-25 — restano candidate all'errata, non voci; niente altro |

#### Che cosa la revisione ha trovato, e dove è corretto — nei compiti, non nell'errata

Le quattro dimensioni del punto 4 della seconda chiusura, più i tre spostamenti di stato letti contro il giorno in cui si
scrivono. Il metodo: la lettura d'apertura per intero, poi la stella polare («Il modello della GUI», le §2, §3, §4, le decisioni,
le registrate, i vicoli ciechi; la tabella dello stato e delle decisioni a 50 righe), il disegno del 2 (la testa, le §1, §2, §6a,
§8, §9, §10, i vicoli ciechi), poi questo piano per intero a 200 righe per chiamata — 17 blocchi in parallelo, il più grande
26,6 KB (righe 401–600); poi **due revisori in sola lettura in parallelo**, uno sui compiti 1–5 e uno sui compiti 6–8 col
diario, coi prompt salvati in file nello scratchpad, che hanno rilanciato ogni *Trova* col `grep -c` e ogni sonda dei «Passo 1»,
e i cui rapporti il coordinatore ha verificato a campione coi propri `grep` lanciati **prima** di dispacciarli.

| # | Dove | Che cosa diceva | Misurato il 2026-09-10 | Rimedio, nel compito |
|---|---|---|---|---|
| 1 | compito 6, Passo 7; compito 7, Passo 1 | «le righe con `⏳` sono **otto** — l'intestazione, …»; `Atteso: 8` | l'`awk` di delimitazione fa `{s=1;next}` sull'intestazione, quindi **non la stampa**: sul modello estratto dal piano il conteggio rende **7**, e 8 solo col `grep` sul modello intero | attesa **sette** in entrambi, con la ragione scritta; il compito 7 consuma comunque tutte e otto le righe (l'intestazione col proprio *Trova*) |
| 2 | compito 7, Passo 8, seconda riga | «le **due righe** del capoverso d'apertura che portano … e …» | nel modello del compito 6 quel capoverso è **una** riga (riga 2487 del piano) | *Trova* riscritto: la riga che comincia con `⏳ **Le otto mosse, …`, intera, presa dal file |
| 3 | compito 8, Passo 8, blocco 2 | `grep -c '^\| M[1-5] \|\|^\| Q[1-4] \|' … # 9` | «Come si chiude» di ADR-0029 porta **già oggi** quattro righe `\| M1 \|`…`\| M4 \|`: dopo la *Decision* il conteggio farebbe 13 | il `grep` **delimitato** a `## Decision` … `### Come si chiude` con l'`awk`, atteso 9 |
| 4 | «Come si riprende», punto 5 della seconda chiusura; compito 8, Passi 3 e 4; compito 1, Passo 3 | i testi fissati portavano **2026-09-09** per la sessione che completa il piano, e la riga della roadmap diceva «in **due** sessioni … la seconda i compiti 6–8 **e la revisione**»; il compito 1 diceva che la riga del piano in roadmap «era arrivata il 2026-09-09» | la sessione che completa è del **2026-09-10** ed è la **terza**; la riga in roadmap la scrive lei | i tre testi scritti con la data vera e «tre sessioni»; i *Trova* del compito 8 (il richiamo del compendio, la cella della roadmap) e il testo del compito 1 allineati **nello stesso commit**. È il gotcha #57: una data scritta prima del giorno è una previsione |
| 5 | P-22 | «il richiamo riscritto è della stessa taglia» | margine `10313` all'apertura contro i `10395` di P-22: 82 byte in più | nessuno: registrato qui; ogni compito rimisura (D18) |
| 6 | compito 4, Passo 5; compito 5, Passo 4; compito 6, Passo 4; compito 7, Passo 6; compito 8, Passo 1 — otto siti | `tree.ps1 … \| tail -6` (e `-4`, `-3`, `-2`) con l'attesa «`emitter started at …`, `rest:`, `stream:`, `last title:`» | `tree.ps1` scrive `emitter started` **durante** il ciclo e poi stampa la tabella di **tutti** i campioni — uno ogni 250 ms — prima delle righe di sintesi: dopo un `tail` la riga dell'emettitore non c'è mai (revisore 1, letto nel codice dettato) | negli otto siti `\| grep -E '^(shell started\|attached to pid\|emitter started\|rest:\|stream:\|last title:)'`: le sole righe che le attese leggono |
| 7 | compito 3, criterio di chiusura | «il titolo porta le **sette** misure» | i campi `chiave=` dell'interfaccia e di `stats.line()` sono **dodici**; D6 ne elenca otto; nulla ne conta sette (revisore 1) | «i dodici campi `chiave=`», elencati |
| 8 | la mappa dei file, la riga del guscio Tauri | non nomina `tauri/icon.png` | il compito 5 lo crea con `icon.py`, `git add spikes/gui-shell/tauri` lo committa e `.gitignore` non lo ignora: il vincolo 9 («solo file della mappa») sarebbe violato dalla **mappa**, non dal compito (revisore 1) | `icon.png` aggiunto alla riga |
| 9 | compito 7, Passo 4 e Passo 8 — la mossa 8 | «la barra scrive `move 8: pointerdown … on div.bigtab`» | `hand.ts` scrive `tag.classe` dell'elemento di `elementFromPoint`, e `.bigtab-title { flex: 1 }` riempie la presa: sul titolo esce `span.bigtab-title`, sui tre comandi `button.`, che fermano la propagazione (revisore 1, dedotto dal CSS e dal DOM) | l'attesa dice entrambe le forme, e che `on button.` non è il no tecnico di P-6 |
| 10 | P-9, un dedotto | «la forma delle istanze del contatore, `pid_<n>_luid_…`» | **misurato** il 2026-09-10 dal revisore 1 con `Get-Counter -ListSet`: `pid_10364_luid_0x00000000_0x000183A5_phys_0`, il filtro `^pid_(\d+)_` regge | il richiamo in P-9: da dedotto a misurato |
| 11 | compito 8, Passo 9 — il `grep` di `Proposed` filtrato | «rende **solo** righe con un richiamo datato, o la riga 29 dell'audit» | due case tengono `Proposed` su una riga **senza** parola del filtro, col richiamo poche righe sotto: la testa della voce 0029 della §5 del compendio (riga 454 oggi) e «Nessuna voce aperta resta nella spec» di `HANDOFF.md` (155) (revisore 2, rilanciato) | il filtro toglie anche quelle due righe, e l'attesa dice perché |
| 12 | compito 8, Passo 9 — l'`awk` delle tabelle spezzate | «non stampa nulla» | stampa **già oggi** sei righe sui file non toccati: quattro sono due tabelle consecutive con una riga vuota in mezzo (compendio 883; riferimenti 108, 1740, 2395), due sono tabelle **spezzate preesistenti** — `HANDOFF.md` 1153–1155, i gotcha 49 e 50, e `riferimenti.md` 103–105 (revisore 2, rilanciato) | il ciclo si lancia anche al Passo 1 e il Passo 9 pretende **lo stesso output**; le due spezzate sono **del proprietario** — registrate qui e nel rapporto, non toccate |
| 13 | compito 8, Passo 8, blocco 2 — il commento | «the heading alone: no file listed» | `check-docs.sh` stampa `  (none)` sotto l'intestazione quando nessun ADR è `Proposed` (P-24, riga 306 dello script) (revisore 2) | il commento corretto |
| 14 | «Come si riprende», punto 4 (b) della seconda chiusura | «`grep -n 'TBD\|TODO\|da scrivere'` non rende nulla» | rende la riga del comando e la 2411, dove «da scrivere» è prosa («una divergenza da scrivere, non da spiegare»): nessun segnaposto (misurato due volte, coordinatore e revisore 2) | nessuno: il punto (b) è verificato così, e detto qui |

#### Che cosa la sessione nuova fa, nell'ordine — l'ESECUZIONE

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno dopo; se è
   uno dopo, il «prossimo passo» della §6 del compendio si rilegge contro `HEAD` di adesso.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio a blocchi di 200 righe, i due pezzi dell'audit — poi di **questo piano**
   la testa (righe 1–99: strumenti, `replace_unique.py`, la regola sui *Trova* presi dal file), i vincoli globali, la posizione,
   «Come si esegue un compito», l'errata, il pre-controllo P-1…P-25, le decisioni D1…D25, la mappa dei file, le voci aperte;
   **non** i compiti tutti insieme — ogni compito si legge quando si dispaccia (regola 1 di «Come si esegue»), coi disegni nelle
   sezioni che nomina. Le tabelle lunghe: 200 righe per chiamata reggono su tutto il piano (i byte per blocco si misurano prima).
3. `superpowers:subagent-driven-development` dal **compito 1**: un subagente fresco per compito, col prompt salvato in un file
   dello scratchpad **prima** di dispacciarlo (un subagente in background muore col processo); prima di ogni dispaccio il
   pre-controllo del compito si **rilegge** contro il codice di adesso — le quattro domande di `CLAUDE.md` e le tre righe in più —
   e il primo difetto trovato è una voce d'**errata** col proprio numero; il revisore rilancia ogni comando accanto a
   un'affermazione misurabile e li elenca (regola 5), e per i compiti 2–5 apre l'app nel browser o lancia il guscio e guarda.
4. Il compito 7 lo esegue il **coordinatore col proprietario** allo schermo (regola 7); su un no all'insieme delle mosse, il
   compito 7-bis si scrive in quella sessione (D12) prima dell'8; il compito 8 chiude e scrive il puntatore della §6 del compendio.
5. A ogni compito: `check-docs.sh` e `gate.sh` prima del commit, la posizione aggiornata nello stesso commit, i fine-riga
   rimisurati, push, senza co-autore; alla chiusura di ogni sessione «Come si riprende» riscritto come diario, la memoria
   dell'agente, `session-handoff`.

### La seconda chiusura — 2026-09-09: i compiti 6–8 SCRITTI, la revisione e gli spostamenti di stato NO

✅ **RICHIAMO DEL 2026-09-10:** ciò che qui manca è **fatto** — la terza chiusura qui sopra. Il testo resta com'era.

⛔ **DA SAPERE SUBITO.** Niente è a metà **nel repository**: albero pulito, nessuno stash, nessuna operazione git in
corso, nessun codice toccato. A metà è ancora **questo file**, ma meno della prima volta: i compiti **6, 7 e 8** sono
scritti per intero nella forma dei compiti 1–5 — ogni *Trova* rilanciato col `grep` il 2026-09-09 contro `b50e8b8`, ogni
attesa contata — il pre-controllo P-19…P-25 e le decisioni D19–D25 stanno nelle loro tabelle, la mappa dei file è
aggiornata, «Dopo il compito 8» è scritto. ⛔ **Mancano, e li fa la sessione nuova PRIMA di eseguire:** la **revisione del
piano intero** (punto 4 qui sotto) e i **tre spostamenti di stato** (punto 5). Il proprietario ha chiuso con *«fermati, si
continua nella prossima sessione»*. Lo scratchpad è ripulito: tutto ciò che vale sta qui e nel commit.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit delle due sessioni del piano | `git log --oneline 5ad4634..HEAD` — **due**: `b50e8b8`, il piano a metà; poi questo, coi compiti 6–8 |
| Codice, cancello, CI, spike | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ spikes/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla |
| Documenti toccati da questa sessione | `git diff --stat b50e8b8..HEAD -- docs/` → questo piano, e `docs/COMPENDIO.md` per il richiamo nel punto 2 del prossimo passo della §6, riscritto allo stato di questa chiusura |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura (baseline, letto dal log) e alla chiusura, prima del commit; `bash scripts/check-docs.sh` → `OK`. Si rilanciano, non si citano |
| Fine-riga | questo piano **LF** — `tr -cd '\r' < docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md \| wc -c` → `0`; il compendio CRLF con CR = righe, `git ls-files --eol docs/COMPENDIO.md` → `i/lf w/crlf` |
| Margine del compendio | il comando del vincolo 11: `10395` all'apertura di questa sessione — il richiamo della prima chiusura aveva consumato 453 dei `10848` — e il richiamo riscritto è della stessa taglia (P-22) |
| Debito lasciato | **dichiarato**: la revisione finale e i tre spostamenti di stato, qui sotto; le righe **dedotte** e non provate — P-3, P-6, P-7 della prima sessione, P-21 e P-25 di questa — restano candidate all'errata, non voci |

### Che cosa la sessione nuova fa, nell'ordine — ✅ i sei passi della seconda chiusura, ESEGUITI il 2026-09-10

I sei passi com'erano — la lettura, il pre-controllo dei compiti 6–8 riletto, la revisione nelle quattro dimensioni, i tre
spostamenti di stato coi testi fissati, il cancello e il commit — stanno parola per parola in
`git show 4d626ea:docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md`; qui non si ricopiano (gotcha
#68). I tre testi fissati al punto 5 sono **scritti** nei tre file — con la data vera, il 2026-09-10, e «tre sessioni» (rilievo 4
della terza chiusura) — e vivono lì e nei *Trova* del compito 8, non qui. L'ordine per la sessione che **esegue** è nella terza
chiusura, qui sopra.

### La prima chiusura — 2026-09-09, il piano a metà: le sue decisioni e le sue trappole

La prima sessione si è chiusa per contesto saturo dopo il compito 5 e ha committato il piano a metà (`b50e8b8`), col
richiamo nel punto 2 della §6 del compendio. Le sue **tracce** dei compiti 6–8 sono **assorbite** nei compiti e tolte da
qui, perché una seconda casa diverge (gotcha #68): il testo com'era in
`git show b50e8b8:docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md`. Le sue tre decisioni, e i
suoi vicoli ciechi, restano:

| | Decisione | Perché |
|---|---|---|
| a | il piano è **committato a metà**, in un file tracciato, col puntatore della §6 che lo dice | il proprietario ha chiuso all'improvviso; lo scratchpad è effimero e si lavora da due macchine (voce E43 del piano del Traguardo 6); il precedente è il punto fermo del brainstorming del 2. Costo: un commit in più, e la testa che dice «A METÀ» finché la sessione nuova non lo completa |
| b | i tre spostamenti di stato «piano scritto» (roadmap, §10 del 2, §6 del compendio) **non** sono fatti: si fanno a piano completo | uno stato «scritto» su un piano a metà mentirebbe; il richiamo «A METÀ» nella §6 basta a farlo trovare |
| c | nessuna riga nuova in `riferimenti.md` per le verifiche di oggi | sono verifiche di **API** (le firme di `dockview-core`, i sorgenti di `three`, la crate `interprocess`, lo schema di Tauri), non misure di stato dell'arte; stanno nel pre-controllo (P-1…P-9) con la data. Se il proprietario le vuole anche lì, è una riga del compito 8 |

- la testa del piano dei gesti letta col tool Bash a 190 righe **trabocca** (35 KB): si legge 1–83, 84–137, 138–186; questo piano è più grande — misurare i byte per blocco prima.
- `dockview-core` 8.2.0 non spedisce CSS (P-1): un `import` del foglio di stile sarebbe stato il primo rosso; verificato aprendo il tarball del registro in memoria con `tarfile`, che è il metodo che regge anche per le firme `.d.ts`.
- i file dei permessi di Tauri su GitHub (`crates/tauri/permissions/…`) rendono 404 dal ramo `dev`: il permesso `core:default` resta dedotto (P-7); la pagina docs.rs di `GenericNamespaced` è 404, il **sorgente della crate** da `static.crates.io` risponde e dice `\\.\pipe\` (P-5); `cdn.jsdelivr.net` serve i sorgenti di `three` per file (P-6).
- il piano si scrive in **frammenti** col tool `Write` (uno per sezione, sotto i 25 KB) e si unisce con `cat`: un `Write` solo non regge; i frammenti restano nello scratchpad e **muoiono con la sessione** — un punto fermo si committa prima che il contesto sia saturo, non dopo.
- la §10 del 2 chiede al piano di **copiare** la Definizione di «fatto»: il compito 8 la copia per intero, non la rimanda.

### Vicoli ciechi e trappole della seconda sessione

- `bash scripts/gate.sh > "$TMPDIR/gate.log"` dal tool Bash: `$TMPDIR` è **vuoto** in questa Git Bash, la redirezione va in `/gate.log` e fallisce con *Permission denied* — il cancello non gira. Il percorso dello scratchpad si scrive per esteso, assoluto.
- le righe `⏭️` del compendio sono **tre**, non due: la terza è un code span nella tabella delle voci aperte (riga 735 il 2026-09-09). L'attesa «due» delle tracce era **non contata**: ora P-20 e D24.
- la tabella dello stato e delle decisioni della stella polare (righe 44–143) letta col tool Bash a **100** righe trabocca (43,8 KB, salvata su file e persa): 50 righe, o 30 sulla sola tabella delle decisioni, com'era già scritto nei suoi vicoli ciechi.
- questo piano letto a 200 righe per chiamata regge su tutti i blocchi, tredici chiamate in parallelo.
- i frammenti dei compiti scritti col tool `Write` nello scratchpad, sotto i 25 KB l'uno, e uniti con uno script Python che li legge — non con un `Write` solo. Una recinzione a tre accenti che contiene un'altra recinzione a tre accenti si chiude prima del tempo: la esterna va a **quattro** (compito 8, Passo 6).
- un comando con una barra verticale dentro una cella di tabella è una trappola doppia — `\|` per la tabella, che l'esecutore copierebbe dentro un'espressione regolare: i comandi con `|` stanno nei blocchi di codice, non nelle celle (compito 8, Passo 8, riscritto).
- il proprietario ha chiuso la sessione mentre si scriveva la sezione «Dopo il compito 8»: la chiusura è costata più della scrittura, perché il piano riferisce P-19…P-25 e D19–D25 e senza le tabelle i riferimenti sarebbero rimasti appesi. La lezione è quella della prima chiusura: il punto fermo si committa **prima**.

### Vicoli ciechi e trappole della terza sessione

- l'`awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s'` **non stampa l'intestazione** che apre la sezione: un'attesa che la conta è
  sbagliata di uno. Un'attesa su una sezione futura si misura sul **modello** estratto dal piano —
  `awk 'NR>=<riga>{ if(/^```/){exit} print }' <piano> > modello.md`, poi lo stesso comando del compito sul modello — e non a mente.
- un testo **fissato** per una sessione futura non porta la data del giorno in cui si scrive: porta `<data>`, o dice che la
  sessione lo ridata. I tre testi del punto 5 della seconda chiusura portavano il 2026-09-09 e la sessione è arrivata il
  2026-09-10 (gotcha #57: una previsione citata come misura); i *Trova* del compito 8 che li ricopiavano andavano allineati.
- un `\| tail -N` su uno script che stampa una **tabella per campione** prima delle righe di sintesi nasconde tutto ciò che lo
  script ha scritto durante il ciclo: si filtra con `grep -E` sulle righe che l'attesa nomina, non con `tail`.
- una sonda `grep -c` sull'ADR intero conta anche le righe che l'ADR ha **già** nella *Context*: la sonda della Definizione di
  «fatto» si scrive **delimitata** alla sezione che il compito produce, come l'`awk` di `RISULTATI.md`.
- una sonda «il `grep -v` delle parole del richiamo non rende nulla» è **per riga**: una casa il cui richiamo sta poche righe
  sotto resta nell'output; e l'`awk` delle tabelle spezzate ha falsi positivi su due tabelle consecutive e coglie spezzature
  **preesistenti**. Ogni sonda «non rende nulla» si lancia **prima**, sui file di adesso, e la sua baseline si scrive nel Passo 1.
- questo piano letto a 200 righe per chiamata regge su tutti i 17 blocchi, in parallelo, coi byte per blocco misurati prima
  (`awk '{b+=length($0)+1} NR%200==0{print NR, b; b=0}'`): il più grande 26,6 KB. La stella polare a 50 righe sulla tabella
  dello stato e delle decisioni (22–23 KB); le altre sezioni prescritte stanno sotto i 20 KB.
- due revisori in parallelo per perimetro — compiti 1–5; compiti 6–8 e il diario — in sola lettura, coi prompt salvati in file
  nello scratchpad e con l'ordine di **non** fare la lettura d'apertura; il coordinatore lancia **prima** una decina di `grep -c`
  sui *Trova* come baseline, e verifica i rapporti a campione contro quella.
