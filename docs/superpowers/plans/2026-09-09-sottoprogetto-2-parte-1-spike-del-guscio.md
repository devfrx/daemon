# Sotto-progetto 2 — parte 1, lo spike del guscio: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario (punto 7 della §10 del [disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md),
> decisione 48 della stella polare). I passi usano le caselle (`- [ ]`) per il tracciamento.
> ⛔ **Il pre-controllo di ogni compito è fatto nella sessione che ha scritto il piano, il
> 2026-09-09, contro il repository a `5ad4634`; l'esecuzione va in una sessione NUOVA.**

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

⚠️ **IL PIANO È SCRITTO A METÀ IL 2026-09-09** — la sessione si è chiusa per contesto saturo dopo il
compito 5: i compiti **6, 7 e 8** e la sezione «Dopo il compito 8» **mancano**, e li scrive la sessione
nuova dalla sezione *«Come si riprende»* in coda a questo file. ⛔ **Non si esegue finché non è completo.**
Il pre-controllo delle quattro domande sui compiti 1–5 sta nella sezione *«Il pre-controllo del piano»*
qui sotto, fatto nella sessione che ha scritto il piano; quello dei compiti 6–8 si fa scrivendoli.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | il protocollo di SP-8, congelato — M1–M5, Q1–Q4, le otto mosse — e la riga SP-8 nella tabella degli spike di `roadmap.md` | uno, **solo documenti** | ⬜ |
| **2** | la Home finta in `spikes/gui-shell/app/`: Vite, Vue 3, `dockview-core` — il nucleo e la striscia bloccati, le tessere, la presa grande, i comandi delle mosse 1–7, salva/ricarica/confronta; `.gitignore` e il lockfile | uno | ⬜ |
| **3** | le tre tessere vive: la chat che rende markdown token per token dal ponte, la scena `three` con fps e API nel titolo, la mano di SP-7 tradotta in eventi del puntatore (mossa 8) | uno | ⬜ |
| **4** | il guscio **Electron**, che legge la pipe con `net` e la passa alla webview; `electron-builder`; lo script di misura dell'albero di processi; `.gitignore` e il lockfile | uno | ⬜ |
| **5** | il guscio **Tauri**, che legge la pipe con `interprocess` e la emette come evento; le icone; `tauri build`; `.gitignore`, `Cargo.lock` e il lockfile | uno | ⬜ |
| **6** | le misure M1–M5 e Q1–Q2 sui due gusci installati, con gli script; la sezione SP-8 di `spikes/RISULTATI.md` coi numeri e le righe del proprietario dichiarate ⏳ | uno | ⬜ |
| **7** | col **proprietario**: le otto mosse nel browser, Q3 e Q4 nei due gusci, la CPU con la chat nascosta e la VRAM a pagina intera, la domanda A/B sul guscio; l'esito completo, la riga SP-8 chiusa | uno | ⬜ |
| **7-bis** | **solo su un no** all'insieme delle mosse: la stessa Home con `interactjs` — il perimetro è qui (D12), il compito si scrive nella sessione che riceve il no, col pre-controllo | — | ⬜ non necessario finché il compito 7 non lo dice |
| **8** | la chiusura: ADR-0029 `Accepted` con la decisione del proprietario, i totali degli ADR, il compendio (§4, §5, §6, §8, l'intestazione), `README.md`, `HANDOFF.md`, `roadmap.md`, i richiami nei due disegni, le fonti in `riferimenti.md`; la Definizione di «fatto» coi comandi | uno | ⬜ |

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
`pid_<n>_luid_…`, che lo script di misura filtra con `^pid_(\d+)_`; se la VRAM restasse a zero con la
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

**La baseline di partenza, misurata il 2026-09-09 su `5ad4634` e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `bash scripts/check-docs.sh` → `OK — no inconsistencies.` ·
il comando del vincolo 11 → `10848` · `git status -sb` → `## main...origin/main`, pulito.

---

## La mappa dei file

| File | Chi lo tocca | Fine-riga il 2026-09-09 | Responsabilità |
|---|---|---|---|
| `spikes/gui-shell/PROTOCOLLO.md` | compito 1, **creato** | LF | i criteri congelati: M1–M5, Q1–Q4, le otto mosse |
| `docs/roadmap.md` | compiti 1, 7, 8; la sessione che scrive il piano | **CRLF** | la riga SP-8 nella tabella degli spike; la riga di questo piano nella tabella dei piani; la riga «Ultimo aggiornamento» a ogni tocco |
| `spikes/gui-shell/app/package.json` · `vite.config.ts` · `index.html` · `popout.html` · `src/main.ts` · `src/style.css` · `src/vue-bridge.ts` · `src/home.ts` · `src/tiles/Tile.vue` | compito 2, **creati**; `main.ts` e `home.ts` **modificati** dal compito 3 | LF | la Home finta: Vite, Vue 3, `dockview-core`, le mosse 1–7 |
| `spikes/gui-shell/app/package-lock.json` | compito 2, **creato** da `npm install` | LF (come `npm` lo scrive; si misura) | il lockfile, committato |
| `spikes/gui-shell/app/src/bridge.ts` · `src/stats.ts` · `src/hand.ts` · `src/tiles/Chat.vue` · `src/tiles/Scene.vue` · `src/tiles/Hand.vue` | compito 3, **creati** | LF | il ponte, il titolo con le misure, la mano, le tre tessere vive |
| `.gitignore` | compiti 2, 4, 5 | **CRLF** | `node_modules/`, `dist/`, `out/`, `target/`, `gen/` dello spike |
| `spikes/gui-shell/electron/package.json` · `main.js` · `preload.js` · `package-lock.json` | compito 4, **creati** | LF | il guscio Electron e `electron-builder` |
| `spikes/gui-shell/measure/tree.ps1` · `size.ps1` | compito 4, **creati** | LF | M1, M4, M5 sull'albero di processi; M2 |
| `spikes/gui-shell/tauri/package.json` · `package-lock.json` · `src-tauri/Cargo.toml` · `src-tauri/Cargo.lock` · `src-tauri/build.rs` · `src-tauri/src/main.rs` · `src-tauri/tauri.conf.json` · `src-tauri/capabilities/default.json` · `src-tauri/icons/*` · `measure/icon.py` | compito 5, **creati** | LF (le icone sono binari) | il guscio Tauri |
| `spikes/RISULTATI.md` | compiti 6, 7 | **CRLF** | la sezione SP-8 e la riga della data |
| `docs/adr/0029-guscio-della-gui.md` | compito 8 | **CRLF** | la Decision, Q1–Q4, l'innesco Linux, `Negative (accettate)`, `Accepted`, il richiamo in testa |
| `docs/COMPENDIO.md` | compito 8 | **CRLF** | la riga 160 (i totali), la riga del guscio in §4, il richiamo sulla voce 0029 di §5, la riga in «Chiuso» e il puntatore di §6, la riga SP-8 in §8, l'intestazione (D16) |
| `docs/archivio/stato-storico.md` | compito 8 | **CRLF** | l'intestazione del compendio com'era (D16) |
| `docs/README.md` | compito 8 | **CRLF** | la riga 0029 dell'indice degli ADR |
| `docs/HANDOFF.md` | compito 8 | **CRLF** | «38 ADR in stato `Accepted`» → 39 (P-11) |
| `docs/riferimenti.md` | compito 8 | **CRLF** | le versioni del giorno dello spike e le fonti di Q2 |
| `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | compito 8; la sessione che scrive il piano | LF | i richiami datati in §2 e §10 |
| `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` | compito 8 | LF | i richiami datati in §4 e nella tabella dello stato (P-16) |
| `docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md` | ogni compito | LF | la tabella della posizione, l'errata, «Come si riprende» |
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
| la riga 6, `Ultimo aggiornamento: …`, **intera, presa dal file** | `Ultimo aggiornamento: **<data>**, con la riga **SP-8** nella tabella degli spike — il compito 1 del [piano della parte 1 del sotto-progetto 2](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md); la riga del piano nella tabella dei piani era arrivata il 2026-09-09.` |

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

- [ ] `vite build` verde; il titolo porta le sette misure nella forma dell'interfaccia; la chat, la scena e la mano vivono nel browser (il revisore)
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
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$ROOT\spikes\gui-shell\electron\node_modules\electron\dist\electron.exe" -ArgumentList . -WorkingDirectory "$ROOT\spikes\gui-shell\electron" -Seconds 30 -EmitterAt 8 -Emitter "$ROOT\spikes\gui-ipc\target\release\core.exe" | tail -6
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
powershell -NoProfile -ExecutionPolicy Bypass -File spikes/gui-shell/measure/tree.ps1 -Exe "$ROOT\spikes\gui-shell\tauri\src-tauri\target\release\sp8-tauri.exe" -Seconds 30 -EmitterAt 8 -Emitter "$ROOT\spikes\gui-ipc\target\release\core.exe" | tail -6
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
## Compiti 6, 7 e 8 — ⛔ NON ANCORA SCRITTI

La sessione del 2026-09-09 si è chiusa qui per contesto saturo. Ciò che i tre compiti devono contenere
è scritto per esteso nella sezione *«Come si riprende»* qui sotto, con le decisioni già prese: la sessione
nuova li scrive **nella stessa forma dei compiti 1–5** — Files, Interfaces, passi con le caselle, i comandi
con l'atteso, il criterio di chiusura — poi la sezione *«Dopo il compito 8»*, e infine la revisione del
piano intero (copertura dei disegni, nessun segnaposto, nomi coerenti fra i compiti).

---

## Come si riprende — scritto alla chiusura della sessione del 2026-09-09, coi comandi

⛔ **DA SAPERE SUBITO.** Niente è a metà **nel repository**: albero pulito, nessuno stash, nessuna
operazione git in corso, nessun codice toccato. A metà è **questo file**: il piano è scritto fino al
compito 5 compreso, con tutto il pre-controllo, le decisioni e la mappa dei file; i compiti 6, 7 e 8 non
esistono ancora. Lo scratchpad della sessione è stato ripulito: **tutto ciò che vale sta qui e nel
commit**. Il proprietario ha chiuso con «continuiamo nella prossima sessione, troppo contesto saturo».

### Lo stato alla chiusura, e il comando che lo rifà

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit di questa sessione | `git log --oneline 5ad4634..HEAD` — **uno**: questo piano a metà e il richiamo nel punto 2 del puntatore della §6 del compendio |
| Codice, cancello, CI, spike | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ spikes/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla |
| Documenti toccati | `git diff --stat 5ad4634..HEAD -- docs/` → solo questo piano (nuovo) e `docs/COMPENDIO.md` (il richiamo di poche righe nel punto 2 del prossimo passo della §6) |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura (baseline, log letto) e alla chiusura, prima del commit; `bash scripts/check-docs.sh` → `OK`. Si rilanciano, non si citano |
| Fine-riga | questo piano **LF** (`tr -cd '\r' < docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md \| wc -c` → `0`); il compendio CRLF con CR = righe; `git ls-files --eol docs/COMPENDIO.md` → `i/lf w/crlf` |
| Margine del compendio | il comando del vincolo 11: era `10848` prima del richiamo, e il richiamo ne consuma poche centinaia |
| Debito lasciato | **dichiarato**: i tre compiti mancanti, la sezione «Dopo», la revisione finale del piano, e i tre spostamenti di stato che si fanno **a piano completo** (qui sotto) |

### Che cosa la sessione nuova fa, nell'ordine

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero a blocchi, i due pezzi dell'audit — poi
   la **stella polare** (almeno «Il modello della GUI», le §2, §3, §4, le decisioni, i vicoli ciechi) e il
   **disegno del 2** (almeno le §2, §8, §9, §10, i vicoli ciechi), poi **questo piano per intero** — è
   grande: si misurano prima i byte per blocco con l'`awk` dei vicoli ciechi della stella polare, e col
   tool Bash si legge a blocchi sotto i 30 000 caratteri.
3. Si scrivono i **compiti 6, 7 e 8** e «Dopo il compito 8» dalle tracce qui sotto, nella forma dei
   compiti 1–5; ogni *Trova* si rilancia col `grep` prima di dettarlo, ogni attesa si conta (nove voci
   d'errata su nove dei piani precedenti erano attese mai contate).
4. La revisione del piano intero con gli occhi freschi (`superpowers:writing-plans`, *Self-Review*): ogni
   riga della §4 della stella polare e della §2 e §10 del 2 ha un compito; nessun segnaposto; i nomi fra
   i compiti coincidono (`tiles`, `wire`, `stats`, `tree.ps1` e i suoi parametri, gli `id` dei pannelli,
   `sp8-electron`, `sp8-tauri`, `SP-8`).
5. I tre spostamenti di stato, **a piano completo**: la riga di questo piano nella tabella dei piani di
   `roadmap.md` (*«scritto il 2026-09-09, completato il <data>»*, forma della riga dei gesti — D14) con la
   riga «Ultimo aggiornamento» riallineata (P-15); un richiamo datato nella §10 del disegno del 2, dopo il
   capoverso che finisce con *«nessuna domanda resta pendente.»* (*«✅ PIANO DELLA PARTE 1 SCRITTO il <data>…»*);
   il punto 2 del puntatore della §6 del compendio: il richiamo «A METÀ» diventa «✅ scritto il <data>;
   l'esecuzione in una sessione nuova». La testa di questo file e la tabella della posizione perdono
   l'avviso «A METÀ».
6. `bash scripts/check-docs.sh`, `bash scripts/gate.sh`, i CR rimisurati, **un** commit, push; la memoria
   dell'agente aggiornata; poi `session-handoff`. L'**esecuzione** — compito 1 con un subagente fresco —
   è di un'altra sessione ancora (decisione 48).

### Le tracce dei tre compiti mancanti — le decisioni sono già prese, qui

**Compito 6 — le misure M1–M5 e Q1–Q2 sui due gusci installati; la sezione SP-8 di `spikes/RISULTATI.md`.**
Files: `spikes/RISULTATI.md` (CRLF) e questo piano. Passi: (1) le misure prima — gli installatori esistono
(`ls spikes/gui-shell/electron/out/*.exe spikes/gui-shell/tauri/src-tauri/target/release/bundle/nsis/*.exe`),
altrimenti `npm run dist` / `npm run build`; il comando delle versioni di `riferimenti.md` rilanciato quel
giorno e le versioni scritte nell'esito (vincolo 8). (2) l'installazione silenziosa di entrambi — NSIS accetta
`/S`: `"…/sp8-electron Setup 0.0.0.exe" /S` installa in `%LOCALAPPDATA%\Programs\sp8-electron\`; l'installatore
di Tauri in `…\bundle\nsis\sp8-tauri_0.0.0_x64-setup.exe /S`, per utente, la cartella si **legge** con
`Get-ChildItem $env:LOCALAPPDATA` e va nell'esito; **M2** con `measure/size.ps1 -Path` sulla cartella e
sull'installatore di ciascuno. (3) **M1, M4, M5** con `tree.ps1` sull'eseguibile installato di ciascuno —
`-Seconds 45 -EmitterAt 15 -Emitter <core.exe> -Csv <fuori dal repo>` — **due corse per guscio** (O1 di
SP-7: una corsa sola è un punto in una nuvola), e si riportano entrambe; la fase `rest` dà M1 a riposo e M5
a riposo, la fase `stream` dà M1 sotto flusso e M4 (picco e media, la soglia è il picco < 25 %); **M3** dalla
riga `last title:` — `fps=`, `min=`, `api=`, `scene=` — che dopo 45 s copre la finestra dei 30 s. (4) **Q1**
scritta dall'architettura (i commenti in testa a `main.js` e `main.rs` lo dicono); **Q2**: il WebView2 dal
registro (il comando è nel protocollo), il Chromium di Electron da `ua=` del titolo, e lo **stato di WebGPU
su WebKitGTK letto alle fonti quel giorno** — la pagina *Implementation Status* del wiki di `gpuweb/gpuweb`
e le note di rilascio su `webkitgtk.org` — con URL e data: è la lettura che sostituisce la misura Linux
(decisione C). (5) la sezione **SP-8** in `RISULTATI.md`, inserita **prima** della riga `## SP-7 — …` con
`replace_unique.py` (Trova: quella riga intera, presa dal file; Sostituisci: la sezione, una riga vuota, la
stessa riga), nella forma di SP-7: la tabella per misura con esito e numeri di **entrambi** i gusci, le
righe Q1 e Q2, e le righe che aspettano il proprietario — le otto mosse, Q3, Q4, la CPU con la chat
nascosta, la VRAM a pagina intera — scritte *«⏳ al compito 7, col proprietario»* (D17); poi «Osservazioni
registrate», «Versioni degli strumenti» (node, npm, cargo, le versioni del giorno, WebView2, la CPU, la GPU),
«Evidenze» con comando e output testuale; la riga della data in testa al file: Trova
`Data di esecuzione: **2026-08-06** per SP-5 e SP-6; **SP-7** porta la propria data nella sua sezione`,
Sostituisci con `… **SP-7** e **SP-8** portano la propria data nella loro sezione`. Oracolo, delimitato alla
sezione (E13 dei gesti): `awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md | grep -c '<[^ ]'`
→ `0`. (6) CR = righe, `check-docs.sh`, `gate.sh`, commit `guscio(compito 6): …`, push.

**Compito 7 — col proprietario: le otto mosse, Q3 e Q4, le due misure che vogliono un click, la domanda sul
guscio.** ⛔ **Non si dispaccia**: lo esegue il coordinatore col proprietario allo schermo (regola 7 di «Come
si esegue»); un revisore in sola lettura rilegge l'esito dopo. Files: `spikes/RISULTATI.md` (CRLF, le righe
⏳ riempite una per una con `replace_unique.py`), `docs/roadmap.md` (CRLF: la riga SP-8 → `✅ **chiuso il
<data>**: …` senza cifre, e la riga «Ultimo aggiornamento»), questo piano. Passi: (1) il relay di SP-7 in
ascolto — `(cd spikes/gesti/relay && cargo run --release -- ../.venv/Scripts/python ../s2_worker.py ../hand_landmarker.task)`
— e `npm run dev` in `app/`; il proprietario apre `http://localhost:5173`. (2) le mosse 1–7 con mouse e
tastiera, **le sue parole per ciascuna** nell'esito; la mossa 7 è la riga `move 7: EQUAL …` o `DIFFERENT …`
della barra dopo «salva» e «ricarica», copiata testuale. (3) la mossa 8: `dnd: pointer` dal menu, la pinza
sulla presa grande; il sì o no **tecnico** lo dicono le righe `move 8: pointerdown … on div.bigtab` della
barra e se la tessera si è mossa; le parole del proprietario a parte, senza soglia. (4) **Q3 e Q4** in
ciascun guscio installato, lanciato a mano (senza emettitore): la mossa 2 col mouse sotto `auto` → Q4; il
comando ↗ → Q3, con la riga `move 4 / Q3: popout OPENED|REFUSED` e se la finestra è comparsa **dentro** il
guscio. (5) **M4 con la chat nascosta**: il proprietario trascina Scena 3D nel gruppo della Chat e attiva
Scena; il coordinatore prende il PID (`Get-Process sp8-electron` / `sp8-tauri`) e lancia
`tree.ps1 -AttachPid <pid> -Seconds 25 -EmitterAt 3 -Emitter <core.exe>` — con `-AttachPid` lo script non
lancia né ferma nulla; **M5 a pagina intera**: la scena massimizzata (⤢), `tree.ps1 -AttachPid <pid> -Seconds 12`.
(6) **la domanda A/B sul guscio** (D15), coi numeri di M1–M5 e Q1–Q4 in una tabella a due colonne e il
consiglio derivato dall'asimmetria dei costi scritta in ADR-0029 — A: Electron, B: Tauri — a parole semplici;
la lettera e le parole del proprietario vanno nell'esito e nell'ADR. (7) l'esito completato, la roadmap,
commit `guscio(compito 7): …`, push. Su un **no all'insieme** delle mosse, il compito **7-bis** (D12) si
scrive nella sessione che riceve il no, prima del compito 8.

**Compito 8 — la chiusura.** Files (CRLF salvo i disegni e questo piano): `docs/adr/0029-guscio-della-gui.md`,
`docs/COMPENDIO.md`, `docs/archivio/stato-storico.md`, `docs/README.md`, `docs/HANDOFF.md`, `docs/roadmap.md`,
`docs/riferimenti.md`, i due disegni (LF), questo piano. Passi: (1) la **Definizione di «fatto»** copiata dalla
§10 del 2, condizione per condizione coi comandi — 1: `git log --format='%h %ad %s' --date=short -- spikes/gui-shell/PROTOCOLLO.md spikes/gui-shell/app`
mostra il protocollo **prima** del primo codice; 2: `check-docs.sh` non elenca più ADR in `Proposed`, e la
riga del guscio nella §4 del compendio è chiusa; 3: `grep -c 'giudizio del proprietario\|con le sue parole' spikes/RISULTATI.md`
sulla sezione SP-8 delimitata con l'`awk`, la mossa 7 come due JSON uguali, l'esito di `dockview` scritto;
4: `git status --porcelain` vuoto dopo una corsa dello spike, i lockfile in `git ls-files spikes/gui-shell | grep -c lock`
→ quattro; 5: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ Cargo.lock` vuoto; 6: i CR rimisurati sui
file della mappa; 7: la parte 2 **non** è scritta qui — lo dice il puntatore. (2) **ADR-0029** con
`replace_unique.py`: nel blockquote in testa, dopo la riga *«forzarla su un argomento sarebbe contro il metodo
di questo repository.»*, la riga `> ✅ **RICHIAMO DEL <data>:** chiusa con **SP-8** — la Decision qui sotto; l'avviso resta com'era, perché è la storia di questa decisione.`;
`- **Status:** Proposed` → `- **Status:** Accepted`; `_(da prendere)_` → il blocco della decisione:
*«✅ **DECISA il <data> dal proprietario, con SP-8 in mano — <Electron | Tauri>.**»*, il rimando al protocollo
e alla sezione SP-8, la **tabella** con una riga per M1–M5 e Q1–Q4 e le due colonne Electron/Tauri (i numeri
di entrambe le corse, come nell'esito), le parole del proprietario, l'**innesco Linux** (*la metà Linux di M3
e M5 non è misurata; al primo Linux vero si rimisurano; se M3 mostra la stessa API con Tauri la decisione si
riapre con un ADR nuovo che superi questo, `Superseded by`, non con una conversazione*), e l'esito di
`dockview`; sotto `### Come si chiude` la riga `✅ **Eseguito il <data>: SP-8**, coi criteri congelati prima in spikes/gui-shell/PROTOCOLLO.md`;
in `## Consequences` il punto `- **Negative (accettate):**` per il guscio scelto — Electron: un terzo runtime,
Chromium impacchettato e la taglia di M2, la superficie del processo principale Node, gli aggiornamenti a
carico nostro; Tauri: il rischio di capacità su Linux **non misurato**, la webview del sistema fuori dal
nostro controllo (versione e flag), nessuna familiarità del proprietario — si scrive il solo elenco del
vincitore. (3) **il compendio**: la riga 160 — `di cui **38 ADR in stato Accepted** e uno \`Proposed\` (0029)`
→ `**39 ADR in stato Accepted** — l'ultimo, 0029, chiuso il <data> con SP-8` (P-11, nello stesso commit);
la riga del guscio in §4 (`| gui — **guscio** | ⚠️ **APERTO**: Tauri o Electron | ADR-0029, \`Proposed\` — **non blocca nulla** |`
→ `| gui — **guscio** | **<Electron|Tauri>** — deciso il <data> con SP-8 | ADR-0029 |`); la voce **0029**
della §5 (`grep -n '^\*\*0029'`) riceve in coda ` ✅ **RICHIAMO DEL <data>:** CHIUSA — **<guscio>**, con M1–M5 e Q1–Q4 misurate da SP-8 su Windows; l'innesco Linux nell'ADR; \`dockview\` <resta|esce>. Il testo sopra resta com'era.`;
nella §6 una riga in «Chiuso» (*la parte 1 del piano del 2 — SP-8, ADR-0029 chiuso, `dockview` <esito> | <data> | il piano e la sezione SP-8*),
il punto 2 del prossimo passo → `✅ parte 1 eseguita il <data>` e `⏭️ **la parte 2**: i pezzi 2–9 della §3 della stella polare, scritta ora coi numeri (Q1: chi decodifica; \`gui/shell/\` se Tauri), stesso pre-controllo`,
il punto 3 → `✅ chiuso il <data>`; la riga di §8 `rifare gli spike SP-5, SP-6 e SP-7` → `… SP-7 e SP-8`, e
`per SP-7 il protocollo congelato in \`spikes/gesti/PROTOCOLLO.md\`` → `per SP-7 e SP-8 i protocolli congelati in \`spikes/gesti/PROTOCOLLO.md\` e \`spikes/gui-shell/PROTOCOLLO.md\``;
l'intestazione (riga 21, da `**Aggiornato il` a `Manutenzione: §13.`) **riscritta corta** — *«**Aggiornato il
<data>**, con la **parte 1 del piano del 2 eseguita** — SP-8, ADR-0029 chiuso — e il puntatore della §6 alla
parte 2; l'ultimo contenuto di merito è la voce 0029 della §5. Manutenzione: §13.»* — e il testo com'era
appeso a `docs/archivio/stato-storico.md` come blocco datato, parola per parola (D16); `grep -n '⏭️' docs/COMPENDIO.md`
deve rendere **due** righe come oggi. (4) `README.md` riga dell'indice:
`| [0029](adr/0029-guscio-della-gui.md) | Guscio della GUI: Tauri o Electron | ⚠️ **Proposed** |` →
`| [0029](adr/0029-guscio-della-gui.md) | Guscio della GUI: <Electron|Tauri> | Accepted |`; `HANDOFF.md`:
`38 ADR in stato \`Accepted\`` → `39 ADR in stato \`Accepted\`` (`grep -n '38 ADR in stato' docs/HANDOFF.md`
→ una riga, la 1052 il 2026-09-09). (5) `roadmap.md`: la riga di questo piano nella tabella dei piani →
`✅ **scritto il 2026-09-09, eseguito il <data>** — \`GATE GREEN\` a ogni compito; SP-8 nella tabella degli spike, ADR-0029 chiuso`,
e «Ultimo aggiornamento». (6) i **disegni**: nel disegno del 2, sotto l'intestazione della §2 e il suo
richiamo, `✅ **RICHIAMO DEL <data>, alla chiusura della parte 1 del piano:** SP-8 eseguito — M1–M5 e Q1–Q4 in ADR-0029, chiuso **<guscio>**; le otto mosse in \`spikes/RISULTATI.md\`, \`dockview\` <resta|esce>; la parte 2 del piano si scrive ora, coi numeri.`
e nella §10, dopo il richiamo «PIANO … SCRITTO», `✅ **PARTE 1 ESEGUITA il <data>:** il verbale nel piano, «A che punto è» e «Come si riprende».`;
nella stella polare, sotto l'intestazione della §4, lo stesso richiamo con l'esito delle otto mosse, e nella
riga *«codice e spec non toccati»* della tabella dello stato (riga 54 il 2026-09-09) la cella «Atteso» riceve
` ✅ **RICHIAMO DEL <data>:** e \`spikes/gui-shell/\`, \`.gitignore\`, \`spikes/RISULTATI.md\` per SP-8 — la parte 1 del piano del 2; \`crates/\` intatto`
(P-16). (7) `riferimenti.md`: una sottosezione nuova **in coda al file** (CRLF), `### Il <data> — SP-8: le versioni del giorno dello spike, e le fonti su WebKitGTK (Q2)`,
col comando delle versioni e le righe F con URL e data delle pagine lette per Q2. (8) `check-docs.sh` (i
totali degli ADR!), `gate.sh`, il margine, i CR su ogni file CRLF toccato, `git ls-files --eol` invariato,
commit `guscio(compito 8): …`, push.

**«Dopo il compito 8».** Che cosa venga dopo non è scritto nel piano: la §6 del compendio porta la **parte 2**,
scritta dopo la misura con lo stesso pre-controllo; le voci che il piano sa e non chiude stanno nella sezione
omonima; il compito 7-bis esiste solo su un no.

### Le decisioni prese scrivendo, col perché — già nella tabella D1–D18, e tre da sapere in più

| | Decisione | Perché |
|---|---|---|
| a | il piano è **committato a metà**, in un file tracciato, col puntatore della §6 che lo dice | il proprietario ha chiuso all'improvviso; lo scratchpad è effimero e si lavora da due macchine (voce E43 del piano del Traguardo 6); il precedente è il punto fermo del brainstorming del 2. Costo: un commit in più, e la testa che dice «A METÀ» finché la sessione nuova non lo completa |
| b | i tre spostamenti di stato «piano scritto» (roadmap, §10 del 2, §6 del compendio) **non** sono fatti: si fanno a piano completo | uno stato «scritto» su un piano a metà mentirebbe; il richiamo «A METÀ» nella §6 basta a farlo trovare |
| c | nessuna riga nuova in `riferimenti.md` per le verifiche di oggi | sono verifiche di **API** (le firme di `dockview-core`, i sorgenti di `three`, la crate `interprocess`, lo schema di Tauri), non misure di stato dell'arte; stanno nel pre-controllo (P-1…P-9) con la data. Se il proprietario le vuole anche lì, è una riga del compito 8 |

### Vicoli ciechi e trappole di questa sessione, per chi la riprende

- la testa del piano dei gesti letta col tool Bash a 190 righe **trabocca** (35 KB): si legge 1–83, 84–137, 138–186; questo piano è più grande — misurare i byte per blocco prima.
- `dockview-core` 8.2.0 non spedisce CSS (P-1): un `import` del foglio di stile sarebbe stato il primo rosso; verificato aprendo il tarball del registro in memoria con `tarfile`, che è il metodo che regge anche per le firme `.d.ts`.
- i file dei permessi di Tauri su GitHub (`crates/tauri/permissions/…`) rendono 404 dal ramo `dev`: il permesso `core:default` resta dedotto (P-7); la pagina docs.rs di `GenericNamespaced` è 404, il **sorgente della crate** da `static.crates.io` risponde e dice `\\.\pipe\` (P-5); `cdn.jsdelivr.net` serve i sorgenti di `three` per file (P-6).
- il piano si scrive in **frammenti** col tool `Write` (uno per sezione, sotto i 25 KB) e si unisce con `cat`: un `Write` solo non regge; i frammenti restano nello scratchpad e **muoiono con la sessione** — un punto fermo si committa prima che il contesto sia saturo, non dopo.
- la §10 del 2 chiede al piano di **copiare** la Definizione di «fatto»: il compito 8 la copia per intero, non la rimanda.
