# Riferimenti esterni

Fonti consultate per le decisioni che poggiano su stato dell'arte esterno, non su
ragionamento interno al progetto. Servono a rendere verificabile la provenienza:
ogni affermazione degli ADR che deriva da qui deve poter essere risalita.

Consultate il **2026-08-06**.

## Harness engineering

Premessa condivisa dalle fonti: `Agent = Model + Harness`, dove *harness* significa
tutto ciò che compone un agente tranne il modello.

| Fonte | Contributo usato | Dove entra |
|---|---|---|
| [Harness engineering for coding agent users — Martin Fowler](https://martinfowler.com/articles/harness-engineering.html) | distinzione **guide** (feedforward) / **sensori** (feedback); sensori computazionali vs inferenziali; *«quando un problema si ripete, si migliorano i controlli»*; principio *keep quality left* | [ADR-0009](adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) |
| [Agentic Harness Engineering (arXiv 2604.25850)](https://arxiv.org/pdf/2604.25850) | enumerazione delle leve del harness: schemi degli strumenti, artefatti di pianificazione, politiche di memoria, recupero, sandbox, sensori di verifica, livelli di permesso, routing, workflow multiagente, gate di revisione umana; evoluzione guidata dall'osservabilità | §0.6 della spec |
| [Skill Issue: Harness Engineering for Coding Agents — HumanLayer](https://www.humanlayer.dev/blog/skill-issue-harness-engineering-for-coding-agents) · [Harness Engineering — Faros AI](https://www.faros.ai/blog/harness-engineering) | i fallimenti degli agenti derivano più da contesto mancante, interfacce fragili, validatori deboli e politiche di ritentativo scadenti che dalla generazione del modello | motivazione di [ADR-0009](adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) |

## Loop engineering

| Fonte | Contributo usato | Dove entra |
|---|---|---|
| [The Art of Loop Engineering — LangChain](https://www.langchain.com/blog/the-art-of-loop-engineering) | i **quattro anelli**: agente → verifica → eventi → miglioramento (*hill climbing*) | [ADR-0009](adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md), [design/04](design/04-anelli-e-sensori.md) |
| [What Is Loop Engineering — Augment Code](https://www.augmentcode.com/guides/what-is-loop-engineering) · [Loop engineering explained — eesel AI](https://www.eesel.ai/blog/loop-engineering) | i problemi centrali si sono spostati dalla formulazione del prompt al disegno dell'anello: trigger, topologia, verificatore, **regole di arresto come requisito di prima classe** | confini di autonomia, §4 |
| [The Agent Loop Decoded — Oracle](https://blogs.oracle.com/developers/the-agent-loop-decoded-three-levels-every-agent-engineer-must-know) | l'anello come ciclo azione → feedback dall'ambiente → decisione successiva, fino a condizione di terminazione | §4, §5 |

## Context engineering

| Fonte | Contributo usato | Dove entra |
|---|---|---|
| [What Is Context Rot in AI Agents — MindStudio](https://www.mindstudio.ai/blog/what-is-context-rot-ai-agents) · [Context Engineering: Agent Reliability Playbook 2026](https://www.digitalapplied.com/blog/context-engineering-agent-reliability-playbook-2026) | **context rot**: la qualità cala al crescere del contesto *prima* del limite tecnico; l'attenzione si distribuisce e l'informazione centrale viene sottopesata; la finestra realmente utile è inferiore a quella dichiarata | [ADR-0010](adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md) |
| [State of Context Engineering in 2026 — Towards AI](https://pub.towardsai.net/state-of-context-engineering-in-2026-cf92d010eab1) · [Context Engineering AI — mem0](https://mem0.ai/blog/context-engineering-ai-agents-guide) | il fattore limitante è la **qualità** del contesto, non il volume; la maggior parte dei team non si avvicina alla dimensione massima della finestra | [ADR-0010](adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md) |
| Definizione attribuita a Phil Schmid (Google DeepMind), ripresa dalle fonti sopra | *«progettare sistemi dinamici che forniscono l'informazione e gli strumenti giusti, nel formato giusto, al momento giusto»* | §0.6 della spec |
| [Agentic Context Engineering (ACE)](https://pub.towardsai.net/state-of-context-engineering-in-2026-cf92d010eab1) — Stanford, SambaNova, UC Berkeley | separazione dei ruoli Generator / Reflector / Curator, con le lezioni ripiegate in un *playbook* strutturato ed evolutivo | **non adottata**: si veda la nota sotto |

## Osservabilità degli agenti

| Fonte | Contributo usato | Dove entra |
|---|---|---|
| [OpenTelemetry GenAI Semantic Conventions — guida all'implementazione](https://hidekazu-konishi.com/entry/opentelemetry_genai_semantic_conventions_guide.html) · [Greptime — come OTel traccia chiamate LLM, ragionamento e strumenti MCP](https://greptime.com/blogs/2026-05-09-opentelemetry-genai-semantic-conventions) | vocabolario `gen_ai.*` per chiamate al modello, passi dell'agente, invocazioni di strumenti, token, costo e metriche di qualità; una run è modellata come pochi tipi di span riconoscibili | [ADR-0017](adr/0017-giornale-sorgente-trace-proiezione.md) |
| [CallSphere — stato dell'arte delle convenzioni GenAI, aprile 2026](https://callsphere.ai/blog/td30-fw-opentelemetry-genai-conventions-april-2026-guide) · [veraexmachina — tracciare agenti in produzione](https://veraexmachina.com/tech/opentelemetry-genai-agent-observability-production/) | **stato di stabilità**: a giugno 2026 (v1.42.0) gli attributi `gen_ai.*` sono in un repository dedicato ma restano **pre-stabili**, senza rilascio 1.0; i nomi possono cambiare | motivo per cui il vocabolario si applica alla **proiezione** e non all'archiviazione |
| [MLflow — guida all'agent observability 2026](https://mlflow.org/articles/what-is-agent-observability-a-2026-developer-guide/) · [Braintrust — guida completa 2026](https://www.braintrust.dev/articles/agent-observability-complete-guide-2026) | quattro pilastri (monitoraggio, tracing, valutazione, governance); trace gerarchico riproducibile con strumenti considerati, argomenti passati, risposte, token e latenza per salto; promozione dei fallimenti confermati a **dataset di regressione** | [ADR-0017](adr/0017-giornale-sorgente-trace-proiezione.md), anello 4 (§5) |
| [Expanso — best practice 2026](https://expanso.io/blog/ai-agent-observability-best-practices/) | telemetria elaborata **localmente** per agenti al margine: l'osservabilità sopravvive alla connettività intermittente e i dati sensibili non lasciano il posto | [ADR-0017](adr/0017-giornale-sorgente-trace-proiezione.md), punto 3 |

## Testing di sistemi agentici

| Fonte | Contributo usato | Dove entra |
|---|---|---|
| [Layer-Isolated Evaluation (arXiv 2606.11686)](https://arxiv.org/pdf/2606.11686) | protezione dello **scaffold deterministico** di un agente in produzione con un harness di test **senza LLM**, bloccato sulle regressioni — separato dallo strato probabilistico | [ADR-0020](adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md) |
| [Cegeka — testing nell'era dell'IA probabilistica](https://www.cegeka.com/en/blogs/testing-in-the-era-of-probabilistic-ai) · [Kunal Ganglani — valutazione a 3 livelli](https://www.kunalganglani.com/blog/evaluate-ai-agents-production) | valutazione a livelli: livello 1 asserzioni deterministiche a ogni commit, livello 2 trace-based con giudice su dataset curati; l'errore diffuso è applicare test tradizionali a sistemi non-deterministici | confine §8 fra kernel e capacità L2 |
| [Confident AI — metriche di valutazione degli agenti 2026](https://www.confident-ai.com/blog/llm-agent-evaluation-complete-guide) · [thinking.inc — valutazione in produzione](https://thinking.inc/en/blue-ocean/agentic/ai-agent-evaluation-production/) | ogni endpoint MCP trattato come dipendenza esterna con contratto: iniezione di dati stantii, risposte malformate e timeout per verificare il degrado sicuro | test di contratto, §8 |
| [FOSDEM 2026 — DST in Rust](https://fosdem.org/2026/schedule/event/GNTZDT-rust-deterministic-simulation-testing/) · [QCon London 2026 — un percorso DST](https://qconlondon.com/presentation/mar2026/deterministic-simulation-testing-dst-journey-wasm-go-state-machines-rust) · [Pierre Zemb — il simulatore di FoundationDB](https://pierrezemb.fr/posts/diving-into-foundationdb-simulation/) | **simulazione deterministica**: esplorazione di molti cammini con guasti iniettati e riproduzione esatta dato il seed; nata in FoundationDB, in uso in TigerBeetle, Resonate, Turso | [ADR-0021](adr/0021-simulazione-deterministica-e-iniettabilita.md) |
| [Testing Storage-System Correctness (arXiv 2602.02614)](https://arxiv.org/pdf/2602.02614) | **crash-consistency testing**: enumerare i confini di persistenza, iniettare un crash in ciascuno, validare lo stato dopo il ripristino; le violazioni emergono solo quando gira la logica di recupero | verifica di Q5 e di [ADR-0007](adr/0007-giornale-write-ahead-e-riconciliazione.md) |

## TRELLIS2 — requisiti hardware (SP-1)

Il documento funzionale originale segnalava un conflitto fra la scheda ufficiale
(≥24 GB) e l'esperienza diretta su 16 GB. **Le due cifre non sono in contraddizione:
misurano configurazioni diverse.**

| Fonte | Dato | Dove entra |
|---|---|---|
| [TRELLIS.2 — requisiti di sistema](https://trellis2.com/blog/trellis2-system-requirements-specs) · [TRELLIS-2 — pagina del modello](https://trellis-2.org/) | ≥24 GB come raccomandazione generale; **512³ → 16 GB minimo**; 1024³ → 40 GB raccomandati; 1536³ → classe H100 | SP-1: il fabbisogno è una **curva**, non un punto |
| [Guida low-VRAM per TRELLIS 2](https://trellis2.app/blog/trellis-2-low-vram) | configurazioni funzionanti fino a 6–8 GB, con tempi di generazione 2–3× | SP-1: esiste un punto di lavoro con margine, a costo di qualità e tempo |
| [ComfyUI-Trellis2 — troubleshooting](https://deepwiki.com/visualbruno/ComfyUI-Trellis2/6-troubleshooting-and-faq) · [guida ComfyUI](https://trellis2.app/blog/trellis-2-comfyui) | leve documentate: `max_num_tokens` (32768 / 49152 / 65536+), `generate_texture_slat=False` per la sola geometria, passi di campionamento | parametri dello spike SP-1 e dei profili di risorsa (§2) |

## Confinamento dell'esecuzione e segreti locali (§10)

| Fonte | Contributo usato | Dove entra |
|---|---|---|
| [Windows Sandbox di Codex CLI: token ristretti, SID sintetici](https://codex.danielvaughan.com/2026/07/18/codex-cli-windows-sandbox-architecture-powershell-ast-safety-elevated-unelevated-appcontainer-restricted-tokens/) · [Lista dei sandbox per agenti di coding, 2026-05](https://gist.github.com/wincent/2752d8d97727577050c043e4ff9e386e) | primitive realmente in uso: su Windows token write-restricted costruiti da SID e ACL più AppContainer; su Linux Landlock per il filesystem e seccomp-BPF per le chiamate di sistema; su macOS Seatbelt | [ADR-0025](adr/0025-confinamento-a-livelli.md), livello 2 |
| [Northflank — come isolare gli agenti nel 2026: microVM e gVisor](https://northflank.com/blog/how-to-sandbox-ai-agents) · [Docker Sandboxes per Codex CLI: isolamento microVM](https://codex.danielvaughan.com/2026/04/13/docker-sandboxes-codex-cli-microvm-isolation/) | con un sandbox del kernel una fuga dà accesso all'host; con una microVM l'attaccante deve superare anche l'hypervisor. Firecracker avvia in ~125 ms con meno di 5 MiB di overhead | [ADR-0025](adr/0025-confinamento-a-livelli.md), livello 3 |
| [Microsoft Learn — cifratura delle chiavi a riposo](https://learn.microsoft.com/en-us/aspnet/core/security/data-protection/implementation/key-encryption-at-rest) · [KomuraSoft — segreti in app Windows con DPAPI](https://comcomponent.com/en/blog/2026/03/16/000-windows-app-secret-storage-best-practices-dpapi/) | le facility dell'OS sono il meccanismo appropriato per dati **mai letti fuori dalla macchina**: cifrano con le credenziali di accesso e non espongono mai la chiave all'applicazione. Distinzione fra ciò che va protetto (token, credenziali) e ciò che resta in chiaro (URL, nomi, flag) | [ADR-0023](adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md) |
| [HackTricks — estrazione di segreti DPAPI](https://hacktricks.wiki/en/windows-hardening/windows-local-privilege-escalation/dpapi-extracting-passwords.html) | i segreti protetti dalle facility dell'OS sono estraibili da chi controlla l'account dell'utente | fonda l'obbligo di **dichiarare in interfaccia** che la protezione equivale a quella dell'account OS |

## Linguaggio del core — SP-5 e SP-6 (ADR-0026)

Fonti consultate il **2026-08-06**, sugli strumenti realmente installati su questa
macchina. Sono verifiche dirette, non articoli: il comando e la sua versione sono la
fonte, e sono riproducibili.

| Verifica | Comando | Dato ottenuto | Dove entra |
|---|---|---|---|
| runtime deterministici Rust | `cargo search madsim` · `cargo search turmoil` | `madsim` 0.2.34 · `madsim-tokio` 0.2.30 (sostituto di tokio) · `turmoil` 0.7.2 | [`spikes/CANDIDATI.md`](../spikes/CANDIDATI.md), SP-5 su Rust |
| semantica di `testing/synctest` | `go doc testing/synctest` su go1.26.5 | orologio finto per bolla; il tempo avanza solo a **quiescenza** (ogni goroutine *durably blocked*); **`sync.Mutex`, `sync.RWMutex`, I/O e chiamate di sistema sono esclusi testualmente**; nessuna promessa di ordine totale deterministico | [`spikes/CANDIDATI.md`](../spikes/CANDIDATI.md) · criterio **C6** del protocollo |

**Correzione tracciata.** La formulazione diffusa «`synctest` dà scheduling
deterministico» è più forte di quanto la documentazione dichiari: il contratto è la
quiescenza, non l'ordine. La differenza non è accademica — l'arbitro GPU di
[ADR-0004](adr/0004-topologia-di-processo.md) è descritto come «un unico lock», cioè
la primitiva che `synctest` esclude. Il criterio C6 di
[`spikes/PROTOCOLLO.md`](../spikes/PROTOCOLLO.md) esiste per misurarlo invece di
assumerlo, in entrambe le direzioni.

## Stack della GUI (ADR-0027)

Consultato il **2026-08-06**.

| Fonte | Contributo usato | Dove entra |
|---|---|---|
| [Tauri — Webview Versions](https://v2.tauri.app/reference/webview-versions/) · [Tauri — Architecture](https://v2.tauri.app/concept/architecture/) · [tauri-apps/wry](https://github.com/tauri-apps/wry) | Tauri **non impacchetta** una webview: usa quella di sistema attraverso WRY. Su Windows è **WebView2**, basata su Chromium, preinstallata su Windows 11 e installata dall'installer sulle versioni precedenti. Su Linux è **WebKitGTK**, con `webkit2gtk 4.1` richiesto da Tauri v2 | [ADR-0027](adr/0027-stack-della-gui.md), costo su G19 |
| idem | La documentazione di Tauri dichiara essa stessa che «la natura diversificata dell'ecosistema Linux» rende difficile raccogliere informazioni accurate su WebKitGTK nelle varie distribuzioni | fonda il costo dichiarato: **due motori di rendering diversi**, non uno portabile |

| [WebGPU — Implementation Status](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status) · [WebGPU nei browser principali](https://web.dev/blog/webgpu-supported-major-browsers) | WebGPU su Chromium/Windows x86-64 è **rilasciato** (Chrome 113+); su Linux è **dietro flag**; per **WebKitGTK non risulta rilasciato**. three.js usa WebGL2 di default, disponibile ovunque | [ADR-0029](adr/0029-guscio-della-gui.md): è il costo principale imputato a Tauri su G6 |
| [Tauri — Frontend](https://v2.tauri.app/start/frontend/) | Tauri è **agnostico rispetto al framework**; Vue è fra quelli documentati via Vite. Vincolo: **SSG, SPA o MPA — niente SSR** | [ADR-0030](adr/0030-framework-dell-interfaccia.md) |

| Verifica | Comando | Dato ottenuto |
|---|---|---|
| versioni degli stack GUI candidati | `cargo search` | `tauri` 2.11.5 · `egui` 0.36.0 · `iced` 0.14.0 · `slint` 1.17.1 · `wry` 0.56.0 · `dioxus` 0.8.0-**alpha**.1 |
| Electron | `npm view` | `electron` 43.3.0 · `electron-builder` 26.15.3 · `electron-vite` 5.0.0 |
| ecosistema Vue | `npm view` | `vue` 3.5.41 · `vue-i18n` 11.4.8 · `pinia` 4.0.2 · `@vueuse/core` 14.4.0 · `@tauri-apps/api` 2.11.1 |
| librerie **agnostiche** per G5 e G6 | `npm view` | `three` 0.185.1 · `codemirror` 6.0.2 — JavaScript puro, sopravvivono a un cambio di framework |
| trasporto IPC locale | `cargo search` | `interprocess` 2.4.3 — named pipe su Windows, socket unix su Linux, stessa API |

## Porta di qualità del kernel (§7)

Verifiche dirette sugli strumenti installati su questa macchina, eseguite il **2026-08-07**
con `rustc 1.95.0` · `cargo 1.95.0`. Il comando e la sua versione **sono** la fonte, e sono
riproducibili. Nessuna di queste è documentazione consultata: sono misure.

| Verifica | Comando | Dato ottenuto | Dove entra |
|---|---|---|---|
| quali classi di dipendenza separa `cargo tree` | `cargo tree --edges` | i valori ammessi sono `all` · `normal` · `build` · `dev` · `features` · `public` · `no-normal` · `no-build` · `no-dev` · `no-proc-macro`. **Non esiste `proc-macro` in positivo** | §7.3.1 |
| il comando che isola le dipendenze **spedite** | `cargo tree -p kernel -e normal,no-proc-macro` | 2 crate. `-e no-proc-macro` **da solo** ne restituisce 20, fra cui `windows-sys`: lascia dentro il sottoalbero **di sviluppo** | §7.2.3 — corregge una riga di HANDOFF |
| bersagli senza sistema operativo installabili | `rustup target list` | esiste **`x86_64-unknown-none`**, stessa architettura e stessa larghezza di puntatore del bersaglio reale | §7.3.2 |
| il cancello respinge una sorgente di casualità | `cargo build --target x86_64-unknown-none -p kernel` con `getrandom` 0.3.4 | `error: target is not supported` — identico su `thumbv7em-none-eabihf` | §7.3.2, sonda B2 |

## Esecuzione del Traguardo 1 — toolchain, versioni risolte, sonde della porta

Misure eseguite il **2026-08-08** e il **2026-08-09** costruendo il workspace e la porta di
qualità. Non sono documentazione consultata: **sono misure**, e il comando con la sua
versione è la fonte. Ambiente: Windows 11 · `rustup` **1.29.0** (28d1352db, 2026-03-05) ·
`rustc` e `cargo` **1.95.0**, appuntati da `rust-toolchain.toml`.

**La toolchain, e cosa serve su una macchina pulita.**

| Verifica | Comando | Dato ottenuto | Dove entra |
|---|---|---|---|
| gira la versione appuntata o quella predefinita? | `rustc --version` contro `rustup run stable rustc --version` | **1.95.0** contro **1.97.1**: `rust-toolchain.toml` vince sul canale `stable`, e la porta gira sulla versione dichiarata anche su una macchina più aggiornata | §4 del compendio |
| il bersaglio senza OS si installa da sé | `targets = ["x86_64-unknown-none"]` nel manifesto della toolchain | su una macchina pulita **non** serve `rustup target add`: il manifesto lo tira giù. È ciò che soddisfa il vincolo 4 della §11 senza chiederlo a nessuno | §7.3.2 · gotcha #38 |
| cosa resta comunque a carico dell'ambiente | build con la toolchain `-msvc` senza **Visual Studio Build Tools** | su Windows il **linker MSVC** è un prerequisito: `rustup` risolve la toolchain, non il linker. Va scritto accanto a `rustup`, o la porta è rossa per il motivo sbagliato | prerequisito d'ambiente in [`AVVIO-CHAT.md`](AVVIO-CHAT.md) |

**Le versioni risolte, e i due grafi del kernel.**

| Verifica | Comando | Dato ottenuto | Dove entra |
|---|---|---|---|
| cosa risolve davvero il manifesto | `Cargo.lock` dopo `cargo build --workspace` | `bincode` **2.0.1** — appuntato a `2`, perché la 3.0.0 è un `compile_error!` · `unty` **0.0.4**, transitiva e deliberatamente **non dichiarata** · `minicbor` **2.3.0** · `trybuild` **1.0.120** fra le dipendenze di sviluppo | §6.1.1 · gotcha #22 · ADR-0031 |
| il grafo **spedito** | `cargo tree -p kernel -e normal,no-proc-macro` | **quattro nodi**: `kernel` → `bincode` → `unty`, più `minicbor` | prima lista di `scripts/gate-deps.sh`, §7.3.1 |
| il grafo **di build** | `cargo tree -p kernel -e no-dev`, per complemento col precedente | **sette voci in più**: `bincode_derive` · `virtue` · `minicbor-derive` · `syn` · `quote` · `proc-macro2` · `unicode-ident` | seconda lista di `gate-deps.sh` — rimedio opposto: si valuta e si **aggiunge** |

**Le sonde, e le tre asimmetrie che nessuno ricostruisce leggendo il codice.**

| Verifica | Comando o banco | Dato ottenuto | Dove entra |
|---|---|---|---|
| un banco `trybuild` **vuoto** è rosso? | `trybuild` **1.0.120**, cartella `compile_fail/` svuotata | ⛔ **no.** Un **glob** che non corrisponde a nulla **non è un errore**: `expand.rs:20` restituisce `Err` solo se è il pattern a essere malformato, e `run.rs:74` stampa un avviso, lascia i fallimenti a zero ed esce 0. Un percorso **letterale** inesistente invece fallisce, perché passa da `check_exists` | guardia di non-vacuità in `crates/kernel/tests/compile_fail.rs` · gotcha #26, seconda occorrenza |
| la guardia sul bersaglio può scattare? | la sola riga `rustup target list --installed`, senza `cargo` | ⛔ `rustup` **1.29.0 riconcilia `rust-toolchain.toml` prima di rispondere**: se il bersaglio manca, **l'atto di chiederlo lo installa**. Isolato con una directory **fuori dal repository** come controllo — lì non c'è manifesto da riconciliare e il bersaglio resta assente 3/3. La guardia scatta solo dove la riconciliazione fallisce, cioè **senza rete** | gotcha #38 · sonda **B4** |
| il cancello senza OS respinge la casualità | `getrandom` **0.2.17** aggiunto a `kernel`, poi `cargo build --target x86_64-unknown-none` | `target is not supported`. ⚠️ La §7 qui sopra registra la stessa sonda con `getrandom` 0.3.4 il 2026-08-07: **versione diversa, esito identico**, e la riga resta valida | sonda **B2** di `scripts/gate-no-os.sh` |
| l'allow-list vede un nome con la **maiuscola**? | `Inflector` **0.11.4** — crate reale, non un nome costruito — aggiunta al grafo **spedito** del kernel | ⛔ **prima no: uscita 0**, cioè un falso negativo su I3. Dopo l'allargamento della classe di caratteri del filtro: **uscita 1 e il nome del colpevole**. ⚠️ Il corteo di dipendenze minuscole veniva segnalato lo stesso: mancava **il capofila**, non l'elenco | gotcha #41 · sonda **N5** |
| basta un manifesto solo per vietare i build script? | `build.workspace = true` in `[workspace.package]` | ⛔ **no**: `cargo` **1.95.0** lo rifiuta in fase di parsing — *«invalid type: map, expected a boolean, string or array»*. La via si chiude **crate per crate**, ed è la ragione per cui il controllo deriva la directory dalla lista dei file vincolati | §7.4.2, riga del build script |

## Evoluzione del formato durevole del giornale (ADR-0036)

Verifiche dirette sugli strumenti installati su questa macchina, eseguite il **2026-08-07**
con `rustc 1.95.0` · `cargo 1.95.0` · Windows 11. Nessuna di queste è documentazione
consultata: **sono misure**, e il comando con la sua versione è la fonte.

| Verifica | Comando | Dato ottenuto | Dove entra |
|---|---|---|---|
| cosa succede rileggendo un record dopo un cambio di tipo | banco su tre classi di formato × nove mutazioni, che confronta i **valori** e non l'esito | **cinque celle su trentasei sono «silenzio sbagliato»**: `Ok` con valori errati. Su formato posizionale anche un campo *opzionale* in coda rende illeggibili i record vecchi | ADR-0036, §4.9 |
| il costo reale degli indici di campo | `minicbor` 2.3.0, codifica predefinita ad **array** | **27 byte contro i 26** di `bincode`, non i 33 della codifica a mappa | ADR-0036, ritrovamento 3 · corregge una premessa di §6.8 |
| i discriminanti espliciti sono onorati? | `bincode` 2.0.1, variante dichiarata `= 20` | **no**: si codifica come l'ordinale, byte per byte. La trappola del riordino **non è chiudibile** appuntando il numero | ADR-0036, ritrovamento 2 |
| il formato dipende dalla configurazione? | `config::standard()` contro `config::legacy()` | byte diversi, e non si leggono a vicenda. **Cambiare configurazione è un cambio di formato**, e nessun byte lo dichiara | ADR-0036, ritrovamento 4 |
| il kernel con **due** serializzatori regge i confini? | `cargo build -p kernel --target x86_64-unknown-none` | ✅ passa. Grafo **spedito** 3 crate; grafo **di build** 7, con `syn` per la prima volta | §7.3.1 · ADR-0031 |

**Correzione tracciata.** La stima corrente prezzava i campi auto-descritti come «costo
permanente su ogni campo di ogni record»: era la codifica a **mappa**. La predefinita della
stessa libreria è ad **array**, e lo scarto è di sette volte — su un numero che stava per
far scartare la forma giusta. Registrato come **gotcha #31**.

## Il formato dei canali privati e i pari non-Rust (ADR-0037)

Consultato e misurato il **2026-08-08**. `rustc 1.95.0` · Python **3.13.7** ·
Node **v24.9.0** con npm **11.6.0**. La domanda che ha aperto la ricerca: *un canale
privato ha due capi, e il secondo non è Rust — il suo ecosistema ha un lettore?*

**Le fonti consultate**, con ciò che affermano:

| Fonte | Cosa dice | Esito |
|---|---|---|
| [`attrs2bin`](https://github.com/fvicent/attrs2bin) — PyPI **0.0.1**, unica release del **2020-03-22** | «compatible with Rust's bincode», e rimanda a `github.com/servo/bincode`, l'URL **pre-1.0** | ⛔ è la configurazione **1.x**, e i serializzatori dichiarati sono `int, float, bytes, str, bool`: **nessun tipo somma** |
| [`serde-generate`](https://docs.rs/serde-generate) | genera Python con «Bincode (**default configuration only**)»; `bincode ^1.3.3` fra le dipendenze | ⛔ **1.x**, e richiederebbe `serde` nel grafo spedito del kernel |
| [`bincode-ts`](https://www.npmjs.com/package/bincode-ts) — **1.0.0**, unica release del **2025-07-17** | espone `BincodeConfig.STANDARD` = `{endian: little, intEncoding: variant}` | ✅ corrisponde a `config::standard()` di bincode 2. ⚠️ README **autodichiarato generato da un LLM** |
| `cbor2` **6.1.4** (Python) · [`cbor-x`](https://www.npmjs.com/package/cbor-x) **1.6.5** (npm, aggiornata il **2026-07-29**) | CBOR conforme a **RFC 8949** | ✅ entrambe leggono i byte di `minicbor` |
| `https://pypi.org/pypi/<nome>/json` · `https://registry.npmjs.org/<nome>` | interrogati per **aprire** i pacchetti invece di fidarsi del nome | ⛔ su PyPI `bincode` installa un modulo **`b64tools`**; su npm `bincode` è una **CLI di sviluppo con l'IA**. Gotcha #33 |

**Le misure**, che non sono documentazione consultata: il comando con la sua versione è
la fonte.

| Verifica | Comando o banco | Dato ottenuto | Dove entra |
|---|---|---|---|
| il pari **Python** decodifica `bincode` 2.0.1? | sonda `no_std` + driver `std`, come in M-1; confronto sui **valori** | ⛔ **no.** `attrs2bin` produce **33 B** dove bincode 2 ne produce **12** — è fixint a otto byte — e sui byte veri solleva `IncompleteOrCorruptedStreamError` | **M-10** · ADR-0037 · §6.10.6 |
| il pari **TypeScript** ci riesce? | `esbuild --bundle` + Node 24, come farebbe Vite | ✅ **sì**, valori giusti e byte tutti consumati. ⚠️ **entrambi** i punti d'ingresso pubblicati sono rotti su Node 24: CJS `exports is not defined`, ESM import senza estensione | **M-11** · ADR-0037 |
| CBOR è leggibile dai due pari? | `minicbor` 2.3.0 → `cbor2` 6.1.4 e `cbor-x` 1.6.5 | ✅ valori giusti su entrambi | ADR-0037, regola 3 |
| un decodificatore CBOR rifiuta byte di un altro formato? | byte di `bincode` dati a `cbor2` | ⛔ **no**: restituisce **`1`** e ignora la coda. Nessuna eccezione | **gotcha #34** · §6.10.4 |
| quanto costa un `Vec<u8>` non annotato? | `minicbor` con e senza `with = "minicbor::bytes"`, frammento audio da 4096 B | **7813 B** contro **4101 B** — **1,91×**, in silenzio | **gotcha #35** · §6.10.4 |

⚠️ **Una nota sulla stabilità di queste fonti in particolare.** Il lettore `bincode` del
pari TypeScript è **un pacchetto a versione unica** con il packaging rotto; i lettori CBOR
sono implementazioni di uno **standard IETF** con più realizzazioni indipendenti. La
decisione non è stata presa su questo scarto — il criterio è *il pari ha un lettore?* — ma
lo scarto è dichiarato in ADR-0037 fra le `Negative`, perché il sotto-progetto 2 lo
incontrerà.

## Esecuzione del Traguardo 2 — le misure del substrato iniettabile

Eseguite il **2026-08-09** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11 · profilo `dev`.
Tutte riproducibili dal repository: non sono prototipi usa-e-getta.

| Misura | Comando | Esito |
|---|---|---|
| **interlacciamento dell'esecutore**, seme `20260806` | `cargo test -p kernel --test executor_determinism -- --nocapture` | **10 cambi di task su 11 transizioni** |
| C1 — stesso seme, stessa traccia | idem, `c1_the_same_seed_gives_one_single_trace` | **una sola traccia** su 100 corse |
| C2 — semi diversi, tracce diverse | idem, `c2_a_different_seed_gives_a_different_trace` | più di una traccia distinta su 200 semi |
| C3 — tempo virtuale | idem, `c3_virtual_time_does_not_wait` | l'orologio arriva **esattamente a 20 000 ms** virtuali |
| la guardia sullo zero di `SeededRng` | calcolo, poi `cargo test -p simulator --test seeded_rng` | il moltiplicatore è **dispari**, quindi la mappa è una **biiezione** modulo 2⁶⁴: **esattamente un seme** finisce a zero, `4_568_919_932_995_229_531` |
| la fuga della cella `Sleep` (difetto E10) | reintroducendo il difetto, sei semi | **quattro semi su sei** perdono — `{2, 3, 5, 6}` |

⛔ **Il 10 su 11 non si legge come «più concorrenza dello spike».** SP-5 misurò **13 su 17**
con un esecutore che sceglieva **una sola** attività a caso per giro; questo ne interroga
**tutte** le pronte in un ordine scelto dal seme (decisione D4 del piano). Con tre attività
che condividono le scadenze, ogni giro è una permutazione completa: **otto cambi sono
forzati** dalla struttura e solo i **tre confini di giro** possono non cambiare. La cifra
è vicina al proprio **massimo strutturale**, e i due numeri **non sono confrontabili**.

⚠️ **La fuga della cella `Sleep` era stata prima misurata a «tre semi su sei — {1,3,5}»**, su
un banco usa-e-getta con un reattore finto **diverso** da quello spedito. Vera, ma di
un'altra cosa: è il gotcha **#15** nella forma più insidiosa, e la cifra che vale è quella
misurata contro `SeededRng` e `VirtualReactor` reali. Il meccanismo è stato poi **calcolato**
— `below(2) == 1` serve alla fuga, e la parità di `xorshift64(seme·M + 1)` per i semi 1–6 è
`0, 1, 1, 0, 1, 1` — e il calcolo combacia col runtime.

## Esecuzione del Traguardo 2 — i Task 7–10: il limite dei giri, e il confine dei tipi

Eseguite il **2026-08-09** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11 · profilo `dev`.
Non sono documentazione consultata: **sono misure**, e il comando col proprio banco è la fonte.
⚠️ **Tutte rifatte** su una copia del repository prima di finire qui, e dove il numero rifatto
si scosta da quello atteso lo scarto è scritto nella riga invece di essere lisciato.

**Il dimensionamento del limite di giri, e lo strumento che lo ottiene senza strumentare nulla.**

⛔ **Il metodo vale più del numero, perché è riusabile e non è ovvio.** L'esecutore **non** è
stato strumentato: si è usato **il limite stesso come strumento**. `Executor::run` fallisce
appena i giri superano il limite consegnato, quindi *il più piccolo limite che restituisce
ancora `Ok(())` **è** il numero di giri*. Il banco è quello di `trace_of` in
`crates/kernel/tests/executor_determinism.rs` — tre attività per quattro passi — con il solo
limite reso variabile.

| Verifica | Banco | Dato ottenuto | Dove entra |
|---|---|---|---|
| quanti giri usa lo scenario di riferimento | il più piccolo limite che passa, cercato per ciascuno dei **duecento** semi | **nove**, e lo **stesso** nove su tutti: minimo uguale al massimo, istogramma `{9: 200}` | `EXECUTOR_TURN_LIMIT` in `crates/daemon/src/main.rs` |
| è un confine o una stima? | limite **otto** e limite **nove**, ciascuno su tutti i semi | con otto la corsa fallisce su **tutti e duecento**, con nove passa su **tutti e duecento**: **confine misurato** | idem · errata **E20** del piano |

⛔ **Smentisce il piano**, che diceva *«meno di quaranta»* senza averlo mai verificato — gotcha
**#15**. Sono nove, e il limite spedito, `100_000`, li supera di **quattro** ordini di
grandezza. Senza questa riga la costante si ri-deriva ogni volta che qualcuno la mette in
dubbio.

⚠️ **Il seme cambia l'ordine _dentro_ un giro, non il _numero_ dei giri**, e non era ovvio a
priori: è ciò che rende la cifra una proprietà **dello scenario** e non di un seme, ed è il
motivo per cui su duecento semi minimo e massimo coincidono invece di formare una banda.

**Il limite è un conteggio di giri, non un tetto sull'orologio.**

Misurato sul **grafo vero** — `SystemReactor` più `SequentialRng`, cioè quello che
`crates/daemon/src/main.rs` cabla davvero, non la coppia finta del simulatore.

| Caso | Banco | Dato ottenuto |
|---|---|---|
| tutto il soffitto speso a interrogare, **nessun giro attende** | un'attività che cede per sempre, limite `100_000`, cinque corse | **≈ 15 ms** — 14,5 · 14,8 · 14,9 · 14,9 · 14,9 — poi `Err(TurnLimitReached)` |
| **una** corsa i cui giri contengono un'attesa da **2000 ms** | un'attività sospesa fino a `Monotonic::from_millis(2_000)`, tre corse | **2,0001 · 2,0001 · 2,0005 s**, poi `Ok(())` |

⚠️ **La seconda riga non ha decimi stabili, e il commento del sorgente ne cita uno.**
`EXECUTOR_TURN_LIMIT` registra **2,0004 s**; la rimisura dà **2,0001–2,0005 s** su tre corse.
Non è una divergenza ma la **granularità dell'overshoot** di `std::thread::sleep`, che nessuna
piattaforma garantisce — la stessa ragione per cui `wait_until` di `platform` dichiara un
residuo invece di scriverci sopra un controllo. La cifra che si cita è **la parte intera**.

📌 **Cosa stabiliscono le due righe insieme.** `EXECUTOR_TURN_LIMIT` copre **in millisecondi** i
blocchi che **non attendono** — un'attività che cede per sempre, una che ri-registra una
scadenza già passata: girano entrambe a vuoto, e finiscono entrambe lì in un quindicesimo di
secondo. E **non limita l'orologio** per un'attività che si riaddormenta su scadenze
**future**: quella non gira a vuoto, aspetta; termina lo stesso — i giri finiscono comunque —
ma al tempo che le sue attese sommano. La garanzia è **terminazione, non prontezza**. ⛔ Una
versione precedente di quel commento affermava il contrario, sostenendo che un giro «non fa
I/O».

**La divergenza dal gotcha #42, sul confine `Untrusted`/`Instruction`.**

Banco: `impl From<Untrusted> for Instruction` scritta in `crates/kernel/src/boundary.rs` — cioè
esattamente il ponte che le due regole di I6 vietano — e poi la porta, nelle due direzioni.

| Verifica | Comando | Dato ottenuto |
|---|---|---|
| il caso della **regola A**, `untrusted_as_instruction.rs`, se ne accorge? | `cargo test -p kernel --test compile_fail` | ⛔ **resta `ok`** — **non** `mismatch`, che era l'attesa. L'uscita `E0308` combacia ancora **esattamente** con l'oracolo, e nulla da nessuna parte diventa rosso |
| il caso della **regola B**, `no_conversion_from_untrusted_to_instruction.rs`, sì? | idem | **`error`**: *«Expected test case to fail to compile, but it succeeded»*, che non passa da nessun oracolo |
| quanto vale quel caso, misurato **dall'altro capo** | con l'`impl` presente **e** quel caso rimosso: `bash scripts/gate.sh` | ⛔ **`GATE GREEN`**: `cargo build`, `cargo test` e i quattro script della porta, **sei controlli su sei verdi mentre I6 è già caduta** |

⛔ **La ragione, ed è il punto.** Lo scarto che il caso della regola A guarda è fra
**riferimenti** — `&Untrusted` contro `&Instruction` — e `impl From<Untrusted> for Instruction`
**non** produce `&Untrusted: Into<&Instruction>`: rustc non ha nessun `help: call Into::into` da
appendere, l'uscita non cambia di una riga, l'oracolo combacia. Sui **due tempi** della §2.1 lo
scarto era invece fra valori **posseduti**, e lì il suggerimento compare — ed è il caso su cui
il #42 fu scritto. ⚠️ La conclusione punta nella stessa direzione, più forte: quella guardia
**non è «disarmabile da una rigenerazione dell'oracolo», è cieca dalla nascita**. E la
differenza cambia il rimedio: non irrigidire l'oracolo, ma tenere il caso **diretto**.

**Le vie che aggirano il confine, contate compilando — e quelle che non compilano.**

⚠️ Ogni riga è stata **compilata**, e quelle che compilano sono anche state **eseguite**:
«compila» e «porta il contenuto oltre il confine» non sono la stessa cosa, e una riga sola
distingue le due.

| Via | Esito |
|---|---|
| **A1** `Instruction::new(u.as_str().into())` · **A2** con `to_owned()` | ✅ compilano · ✅ **portano il testo intatto** |
| **A3** `Instruction::new(format!("{u:?}"))` | ✅ compila · ⛔ **non porta più il contenuto**: esce `Untrusted(<24 bytes>)`. È la via chiusa dal `Debug` scritto a mano |
| **A4** giro attraverso il giornale: `outcome` → `read_back` → `String::from_utf8` | ✅ compila · ✅ testo intatto |
| **A5** `transmute` da una crate che ammette `unsafe` | ✅ compila · ✅ testo intatto |
| **A6** un `Journal` che risponde `Ok(())` senza scrivere un byte | ✅ compila · ✅ `promote` riesce, testo intatto |
| **A7** un modulo **figlio** di `boundary`: `Instruction(u.0)` | ✅ compila · ✅ testo intatto |

📌 **Sette vie compilano, sei portano il contenuto**, che è la forma esatta di ciò che
`Untrusted::promote` e l'errata **E29** scrivono come *«sette compilano, una sola è chiusa»*.

E il contro-insieme, dodici tentativi che **non** compilano:

| Tentativo | Codice |
|---|---|
| `let _i: Instruction = u.into();` | `E0277` |
| `Instruction::from(u)` | ⚠️ **`E0308`, non `E0277`** — vedi sotto |
| `build_prompt(&system, &u)` · `Instruction::new(u)` · `let _r: &Instruction = &u;` · `let _v: Vec<Instruction> = vec![u];` | `E0308` |
| `u.promote()` senza il giornale | `E0061` |
| `u as Instruction` | `E0605` |
| `Instruction("…".into())` da fuori dal modulo, e da un modulo **fratello** dentro `kernel` | `E0423` |
| `u.0` da fuori dal modulo, e da un modulo **fratello** dentro `kernel` | `E0616` |
| `transmute` **senza** blocco `unsafe` | `E0133` |
| `unsafe { transmute }` **dentro `kernel`** | nessun codice: la lint `unsafe_code` di `#![forbid(unsafe_code)]`, *«usage of an `unsafe` block»* |

⛔ **Il risultato che vale più degli altri: la privacy del campo di una tuple-struct è di
modulo, non `pub(crate)`.** Un modulo **fratello** dentro `kernel` **non** può costruire
`Instruction(…)` né leggere `Untrusted.0` — `E0423` e `E0616`, misurati aggiungendo il modulo e
compilando. Un modulo **figlio** di `boundary` invece **sì**, ed è A7. Quindi le sette vie sono
**le** vie: non ce n'è un'ottava nascosta altrove nella crate, ed è un residuo **misurato**
invece che ragionato.

⚠️ **`Instruction::from(u)` e `u.into()` falliscono con codici diversi**, e non era prevedibile
leggendo: `Instruction::from` risolve sull'impl **riflessiva** `From<T> for T` di `core`, quindi
l'errore cade sull'**argomento** (`E0308`) e non sul vincolo di tratto (`E0277`). Le due
sintassi della **stessa** conversione producono uscite diverse: un oracolo scritto per l'una non
copre l'altra.

⚠️ **Un'attesa smentita, registrata invece che allineata.** Ci si aspettava **dieci** vie non
compilanti *«ciascuna col proprio codice d'errore»*. Nessuna delle due metà regge: il **numero**
di vie che non compilano è una proprietà dell'**elenco** che si sceglie di provare, non del
confine — i dodici qui sopra sono i tentativi naturali, e se ne scrivono altrettanti — e i
**codici distinti sono sette**, con `E0308` che da solo ne copre **cinque**. Ciò che non dipende
dall'elenco, e che è dunque la sola parte da citare, sono i codici e il risultato sulla privacy
di modulo.

**Marginale, e però è la ragione per cui una premessa fu smentita.**

| Verifica | Comando | Dato ottenuto |
|---|---|---|
| quante volte `clippy` chiede il `Default` che non c'è | `cargo clippy --workspace --all-targets`, **due volte a distanza** con `cargo clean` in mezzo | **quattro** emissioni su quattro bersagli — `simulator` **lib** e **lib test**, `platform` **lib** e **lib test** — per **due** tipi: `VirtualReactor` in `crates/simulator/src/reactor.rs` e `SystemReactor` in `crates/platform/src/reactor.rs`. Identico alle due misure |

⚠️ **A video se ne leggono due, non quattro**, e chi ricontasse si troverebbe in disaccordo con
questa riga: `cargo` stampa il corpo dell'avviso **una volta per tipo** e riporta gli altri due
come *«1 duplicate»* nelle righe di riepilogo per bersaglio. Le occorrenze sono quattro, i corpi
stampati due. ⛔ Ed è tutto ciò che `clippy` ha da dire sul workspace.

📌 **Perché sta qui.** La premessa con cui era stato chiesto un `impl Default for SystemReactor`
diceva che toglierlo non avrebbe fatto rumore: `clippy` **quel `Default` lo chiede eccome**, e
la decisione di non metterlo regge per **altre** ragioni — nessun chiamante, e `VirtualReactor`
che riceve la stessa identica warning senza aver mai avuto un `Default`, quindi toglierlo rende
i due reattori coerenti invece di isolarne uno. La §7.4.3 scioglie il pareggio: *«clippy non ha
voce nella porta»*. Nessun `#[allow]`, perché sopprimere nasconderebbe anche l'occorrenza
successiva. Errata **E18** del piano.

## Esecuzione del Traguardo 2 — il Task 11: la porta che non era implementabile

Eseguite il **2026-08-09** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11 · profilo `dev`.
Non sono documentazione consultata: **sono misure**, e il comando col proprio banco è la fonte.

**1 — La porta come il piano la dettava non è implementabile fuori da `kernel`.**
Banco: una finta di `Worker` e `Process` in `crates/kernel/tests/ports_are_implementable.rs`,
che è un **test d'integrazione**, quindi una crate a sé — l'unico punto di osservazione da cui
il difetto è visibile.

| Comando | Esito |
|---|---|
| `cargo test -p kernel --test ports_are_implementable`, con `SingleReceipt { pub(crate) id }` e nessun costruttore | `error[E0599]: no function or associated item named 'new' found for struct 'SingleReceipt'`, più quattro errori sulla lettura dell'id — `private field, not a method` |
| lo stesso, dopo aver aggiunto `new` e `id` pubblici | compila |

⛔ **E la stessa misura ha prodotto il gotcha #47.** Provata anche la forma pura — il letterale
`SingleReceipt { id: 7 }` da fuori dalla crate — rustc **non dava nessun errore**, e la lettura
ovvia era che un campo `pub(crate)` fosse scrivibile da fuori. È un **`E0451`**, emesso dalla
passata di **privacy**, che **non gira** perché la compilazione si ferma prima al type-check;
sanati quelli, `E0451` **compare**. L'elenco di errori che si legge è quello della **prima
passata che ha fallito**, non tutti.

**2 — `Grant`: il campo unitario dà la garanzia identica del campo nominato, a costo zero.**

| Forma | Inedificabile da fuori? | Warning | `#[allow]` |
|---|---|---|---|
| `pub(crate) reserved_mib: u64` — dettata dal piano | sì | `field 'reserved_mib' is never read` | **necessario** |
| `pub struct Grant(());` — spedita | sì, `error[E0423]: cannot initialize a tuple struct which contains private fields` | **nessuno** | **nessuno** |

Provate da un test d'integrazione anche `Grant::new()` → `error[E0599]` e `Default::default()`
→ `error[E0277]`. Errata **E39** del piano.

**3 — `Clone`: la contro-sonda è ciò che rende la potatura difendibile.**

| Sonda | Esito |
|---|---|
| `Clone` tolto da `WorkerDescriptor` **e** `Frame` | `cargo test --workspace` **verde**, zero warning |
| `Clone` tolto da `Path` — contro-sonda di non-vacuità | **rosso**: `E0277` · `E0308` · `E0599` |

La differenza non è di gusto: `declare_scope` consegna un **prestito** che l'implementazione
deve trattenere, i due tipi di `process` attraversano **per valore**. Errata **E40**.

**4 — La campagna di mutazione della finta: dodici mutazioni, dodici uccise.**
Comando per ciascuna: applicare la mutazione, **verificare che il file sia cambiato**, poi
`cargo test -p kernel --test ports_are_implementable`. La tabella completa dei carnefici è in
[`porta-di-qualita.md`](porta-di-qualita.md); qui le due righe che decidono qualcosa:

| Mutazione | Esito | |
|---|---|---|
| `read_one` → costante **7** | 1 rosso su 9 | uccide **perché il valore è sbagliato** |
| `read_one` → costante **1** | ⛔ **9 verdi su 9**, prima del rimedio | la correlazione era persa e nessuno se ne accorgeva |
| de-correlazione totale, **zero** `receipt.id()` nella finta | ⛔ **9 verdi su 9**, prima del rimedio | una finta che non correla affatto soddisfaceva il file |
| `kill()` **acquista** una guardia di liveness | ⛔ **9 verdi su 9**, prima del rimedio | l'eccezione «uccidere è sempre lecito» era dichiarata in un commento e **non difesa** |

Dopo `answers_are_correlated_to_the_receipt_that_asked` e il caso del worker già morto, tutte e
quattro sono **rosse**, e la costante `1` è uccisa da **un test solo**.

⛔ **Il gotcha #48 esce da qui, e non dal codice misurato.** Quattro esiti **credibili e falsi**
prodotti dal banco: due `sed` che non agganciavano la riga (mutazione non applicata → verde che
somigliava alla vacuità cercata) · un rilevatore su `^error` che pescava l'`error: test failed`
di `cargo`, dichiarando «non compila» dieci mutazioni che compilavano **e uccidevano** · la
costante di M6a scelta a caso che coincideva col caso fortunato · una sostituzione globale che ha
riscritto il corpo dell'aiutante **dentro sé stesso** (`fn alive() { self.alive()?; }`), colta dal
conteggio dei siti e non dai test. ⚠️ **La prima è ricapitata a chi verificava**, sullo stesso
file, un'ora dopo aver letto la riga che la descrive.

## Esecuzione del Traguardo 2 — il Task 12: la porta che si è progettata sottraendo

Eseguite il **2026-08-10** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11 · profilo `dev`.
Non sono documentazione consultata: **sono misure**, e il comando col proprio banco è la fonte.

**1 — La porta era corretta, e la finta è servita a sottrarre.**
`ipc` è l'unica delle quattro porte dichiarate in anticipo che la finta ha **confermato**
invece di smentire: test scritto **prima** del sorgente → `E0432: could not find ipc in ports`
(rosso per il motivo giusto), poi `ipc.rs` compila **al primo colpo**, nessun `E0599`.

**2 — Quattro item su cinque cadono, misurati uno per uno.**
Metodo: togliere l'item → `cargo build --workspace` **e** `cargo test --workspace`, in **passi
separati**, perché «serve alla crate» e «serve al test» sono esiti diversi.

| Item | Esito togliendolo | Decisione |
|---|---|---|
| `ClientId::get()` | verde, zero warning | **cancellato** |
| `Hash` | verde | **cancellato** — ⛔ abilita `HashMap`, che nel kernel è vietato (gotcha #12) |
| `PartialOrd` / `Ord` | verde | **cancellato** |
| `PartialEq`/`Eq` — **contro-sonda** | **`E0369` × 7** | tenuto |
| `Copy` — **contro-sonda** | **`E0382` × 23** su **otto** legami | tenuto |
| `Debug` — **contro-sonda** | **`E0277` × 5** | tenuto |
| `Clone` con `Copy` in piedi — **contro-sonda** | ⛔ **`kernel` non compila** | tenuto: non è una scelta, lo pretende `Copy` |

⛔ **Le contro-sonde non sono cerimonia.** L'argomento con cui `get()` cade — *«un'implementazione
conserva un `ClientId` `Copy` e lo confronta con `==`»* — poggia interamente su `PartialEq`:
senza la sonda che lo prova rosso, sarebbe la forma esatta in cui `SingleReceipt::id` era
sopravvissuto **senza copertura** al Task 11. Errata **E42** del piano.

**3 — Il numero misurato quattro volte e sbagliato tre, ed è il gotcha #48 in forma pura.**
Il conteggio dei legami mossi togliendo `Copy`:

| # | Strumento | Uscita | |
|---|---|---|---|
| 1 | `head -6` | «sei» | il **tetto del comando** letto come conteggio |
| 2 | `uniq` sulle stringhe di messaggio | 23 errori, «**dieci** legami» | 23 giusto; `doomed` e `survivor` compaiono in **due** forme ciascuno (`borrow of` e `use of moved value`) |
| 3 | parser JSON, ramo *children* | ⛔ **«zero siti», uscita pulita** | cercava `move occurs because` fra i children, ma rustc la porta come **etichetta di span** |
| 4 | parser JSON, etichette di span | **otto** siti, `1+5+6+1+3+3+2+2 = 23` | riconciliati col totale — la misura buona |

⛔ La terza è la più pericolosa: **un numero preciso da uno strumento che sembrava funzionare**.
Il numero non ha mai cambiato l'esito — la contro-sonda era rossa in tutte e quattro — e questo
è il punto: **si conta che il misuratore stia guardando la cosa giusta prima di leggerne
l'uscita**, qui contando le ventitré etichette trovate contro i ventitré errori.

**4 — Campagna di mutazione della finta: quattordici mutazioni, quattordici uccise.**
Comando per ciascuna: applicare, **verificare che il file sia cambiato e che i siti siano
quelli attesi**, poi `cargo test -p kernel --test ports_are_implementable`. La tabella completa
è in [`porta-di-qualita.md`](porta-di-qualita.md); qui le tre che un test solo uccide, che sono
le uniche che dicono qualcosa che la tabella non dice:

| Mutazione | Unico carnefice | Perché conta |
|---|---|---|
| `read_one` → costante `1` (`process`) | `answers_are_correlated_...` | prima del rimedio lasciava **9 verdi su 9** |
| la morte del client è **contagiosa** | `a_dead_client_does_not_take_the_port_with_it` | prima erano **zero** i suoi carnefici esclusivi |
| `accept` **ricicla** l'id di un morto | idem | idem |

⛔ **Il test che porta la proprietà per cui la porta esiste era cancellabile lasciando la porta
verde** — gotcha **#45**, terza occorrenza in due giorni. Le due mutazioni che lo isolano sono
state cercate apposta.

**5 — E il banco ha ingannato altre due volte, con forme nuove.**
⛔ **Due strumenti gemelli, corretto uno solo:** il bug dei fine-riga riparato in `mutate.py` e
non in `mutants.py`, e alla prima corsa successiva il gemello ha **riappiattito il file in LF**
— `git diff` ha dichiarato **seicento righe** cambiate che nessuno aveva toccato. ⚠️ Il
repository ha i fine-riga **misti per file**: non c'è una convenzione da seguire, c'è un file da
non cambiare. ⛔ **E la più insidiosa delle nove: una rifinitura di _leggibilità_ disarma la
campagna senza che nulla diventi rosso.** La rinomina `position` → `row_of`, chiesta da una
revisione di qualità, ha reso **stantie due ancore** e una mutazione è tornata «zero siti»
invece di un esito. L'ha colta **solo** la guardia sul conteggio: le ancore sono **accoppiate ai
nomi del codice**, quindi la campagna si rilancia dopo ogni **rifinitura**, non solo dopo ogni
cambiamento di comportamento.

## Esecuzione del Traguardo 2 — i Task 13–14: i comandi con cui si riconta il catalogo

Eseguiti il **2026-08-10**, chiudendo il traguardo. ⛔ **Non sono misure di codice: sono i
comandi che rispondono a «questo conteggio è ancora vero?»**, e stanno qui perché il gotcha
**#31** dice che un numero a sostegno di una regola giusta non viene mai rimisurato — e chi
volesse rimisurarlo, finora, doveva prima reinventare come.

| Domanda | Comando | Esito il 2026-08-10 |
|---|---|---|
| quante righe ha il **blocco C** del catalogo §7.4.1 | `awk 'NR>=2628 && NR<=2648 && /^\|/ && !/^\|-/ && !/^\| Difende/' <spec> \| wc -l` | **diciotto** |
| quante ne dichiara implementate il registro | `awk 'NR>=41 && NR<=54 && /^\|/ && /blocco C/' docs/porta-di-qualita.md \| wc -l` | **sette** |
| quanti casi di compilazione fallita esistono | `ls crates/kernel/tests/compile_fail/*.rs \| wc -l` | **quattordici** — quattro dal Traguardo 1, dieci dal 2 |
| quante famiglie di porte | `grep -c '^pub mod' crates/kernel/src/ports/mod.rs` | **sei** |

⚠️ **I due `awk` dipendono da numeri di riga, quindi invecchiano**: sono un punto di partenza,
non un controllo. Chi li rilancia verifica prima che l'intervallo peschi ancora la tabella
giusta — un intervallo che non pesca nulla darebbe **zero** senza sollevare niente, che è il
gotcha **#26** applicato a uno strumento usa-e-getta.

📌 **Il conteggio che conta davvero non è nessuno di questi quattro, ed è il quinto:**
**trentanove** righe di catalogo — tre del blocco A, cinque del B, diciotto del C, tredici di
§7.4.2 — verificate **una per una** contro il registro, e **nessuna assente**. È l'unico che
non ha un comando: una riga che manca in un registro non la trova un `grep`, perché non si sa
cosa cercare. La si trova solo enumerando la fonte e cercando ciascuna voce nella destinazione.

## Esecuzione del Traguardo 3 — i Task 1 e 2: i byte del record, e la collisione che ha riscritto due oracoli

Misurate il **2026-08-10** con `rustc 1.95.0` · `cargo 1.95.0` · `minicbor` 2.3.0 ·
`trybuild` 1.0.120 · Windows 11. ⛔ **Sono misure, non documentazione consultata**: il comando
con la sua versione è la fonte. Dove l'attesa scritta prima divergeva dall'esito, la divergenza
è **registrata e non appianata** — sono **cinque**: tre su ciò che il piano del traguardo dava
per scontato, una sul registro, e ⛔ **una su una misura scritta qui in una forma che non si
poteva rifare**, corretta da una revisione che ha provato a rifarla. È il gotcha **#15** rivolto
a chi lo cita, ed è la ragione per cui la colonna **Come** porta ora le precondizioni e non solo
la mutazione.

| Misura | Come | Esito il 2026-08-10 | Dove entra |
|---|---|---|---|
| i **byte** del record durevole | `Record::V1(..).encode()` a payload vuoto | `82 00 81 84 00 01 00 40` — otto byte: array(2), variante `0`, array(1), array(4), `kind`, `effect`, `trust`, stringa di byte vuota. Con un payload da venti byte il record ne fa **ventotto** | doc di modulo di `crates/kernel/src/record.rs` · §4.9.3 |
| `#[cbor(array)]` esplicito **quanto costa** | gli stessi byte, con e senza l'attributo sui due tipi | ⛔ **byte-identici, lunghezza compresa**. La decisione **D3** del piano — *«scriverlo esplicito anche se è il default»* — si onora quindi **a costo zero**, ed è perché il sorgente dettato che non lo portava è stato corretto invece che discusso (errata **E3**) | ADR-0036 · errata E3 del piano |
| ⛔ la **collisione di nomi** fra `record` e `boundary` | aggiunto `pub mod record;` a `crates/kernel/src/lib.rs`, poi isolata commentando la stessa riga | `record::Trust::{Instruction, Untrusted}` collide con `boundary::{Instruction, Untrusted}`: rustc **smette di abbreviare i percorsi**, e due oracoli **pre-esistenti** — `untrusted_as_instruction.stderr` e `no_conversion_from_untrusted_to_instruction.stderr` — passano a **`mismatch`**. Commentato il modulo tornano **`ok`**, che è la prova che la causa è la collisione e non il contenuto dei casi. ⚠️ **È un costo permanente**: ogni oracolo futuro del kernel che nomini quei due tipi porterà i percorsi qualificati per intero | §7.4.1 blocco C · gotcha #25 |
| la **parola** con cui scatta `record_without_version.rs` | aggiunto un `encode` inerente a `RecordV1`, poi rimosso | **`error`**, con `Expected test case to fail to compile, but it succeeded.` — **non** `mismatch`. ⚠️ **Il piano attendeva la direzione opposta** (errata **E2**), e la conclusione che cercava regge lo stesso e in meglio: `TRYBUILD=overwrite` riscrive solo i `.stderr`, quindi non può spegnere un caso che scatta **compilando**, e non serve un secondo caso di forma diversa — gotcha **#42** | §7.4.1 blocco C · registro |
| ⛔ il **giro di andata e ritorno** è cieco, ma in **una direzione sola** | su ciascuno dei tre campi (`kind`, `effect`, `trust`), due mutazioni: `decode` forzato al valore **che il test scrive**, e forzato **all'altro** | forzandolo al valore scritto diventa rossa **una sola** sonda, quella del campo; forzandolo all'altro ne diventano **due**, perché anche il round trip se ne accorge. ⚠️ **La prima stesura del commento del banco affermava «una sola, sempre» ed era falsa**: registrata come divergenza invece che allineata all'attesa | `crates/kernel/tests/record_shape.rs` |
| la **parola** con cui scatta `record_without_trust_label.rs` | rimosso il campo `trust` da `RecordV1`, poi ripristinato | **`error`**, stessa frase. Neanche questa riga poggia sul proprio oracolo, e nessuna rigenerazione in blocco la spegne | §7.4.1 blocco C · registro |
| ⛔ `#[cbor(default)]` **da solo** sul campo | **solo** l'attributo, senza nessun `impl Default for Trust`, poi `cargo build -p kernel` | ⛔ **non compila**: `error[E0277]: the trait bound `Trust: Default` is not satisfied`. Il derive di `minicbor` **pretende** `Default`, quindi la ricetta è di **due righe** e mai di una | §7.4.1 blocco C |
| ⛔ cosa **non** disarma `record_without_trust_label.rs` | la ricetta completa a **due righe** — `impl Default for Trust` **più** `#[cbor(default)]` sul campo — e poi il solo `impl` | **nessuna delle due**: quel caso resta verde. In Rust un `Default` sul **tipo di un campo** non rende quel campo omissibile in un **letterale di struct**. ⚠️ **Il piano prescriveva proprio quella mutazione** come contro-direzione: l'unica che disarma **quel** caso è togliere il campo — ed è la ragione per cui la riga ha un **secondo** caso | §7.4.1 blocco C |
| e cosa disarma `trust_has_no_default.rs` | base **`ok`**; poi `impl Default for Trust` da solo, e poi con l'attributo in aggiunta | **entrambe** lo disarmano: passa a **`error`** con `Expected test case to fail to compile, but it succeeded.`, e tolto l'impl torna verde. ⛔ **Quindi la metà «non ha default» NON è scoperta**: `Trust: Default` è la porta obbligata di ogni via che defaultizzi, e questo caso ci sta dentro. ⚠️ Resta fuori solo un default scritto **a mano** dentro un `Decode` su misura, che non passa da `Default` — stesso limite dichiarato che §2.8.4 porta per `Parameters::new` | §7.4.1 blocco C |

### I fine-riga, misurati per la prima volta invece che assunti

⛔ **La regola dice *«i fine-riga sono misti per file: c'è un file da non cambiare»*, e nessuno
aveva mai misurato quali.** Contati il 2026-08-10 su tutti i file tracciati, leggendo i byte:

| | |
|---|---|
| **solo LF** | **centosessantatré** file |
| **solo CRLF** | **quattro**, e sono questi: `crates/kernel/src/ports/process.rs` · `crates/kernel/tests/ports_are_implementable.rs` · `crates/kernel/tests/reactor_contract.rs` · `crates/platform/src/reactor.rs` |
| ⛔ **misti dentro un file** | **zero**. La regola è vera *fra* i file, non *dentro* uno |

```python
# git ls-files, poi per ciascuno: crlf = d.count(b'\r\n'); lf = d.count(b'\n') - crlf
```

⚠️ **E il modo in cui è saltata fuori vale quanto il numero, perché è il gotcha #48.** Il
controllo usato per tutta la sessione era `grep -cU $'\r' <file>` dentro una sostituzione di
comando, e lì `$'\r'` **collassa a un modello vuoto**, che combacia con **ogni riga**: il
conteggio tornava sempre *«righe con CR = righe totali»*, cioè **CRLF per qualunque file**,
compresi quelli appena creati in LF. ⛔ **Un banco che sbaglia verso l'attesa**: la regola fa
attendere CRLF, e il banco lo confermava. Si è rotto solo quando ha dato **due risposte diverse
sullo stesso file** nella stessa sessione. Il metodo affidabile è leggere i **byte**, mai un
`grep` su un carattere di controllo passato per il quoting della shell.

### I comandi con cui si riconta il catalogo, riscritti perché delimitino per intestazione

⛔ **I due `awk` del 2026-08-10 dipendevano da numeri di riga, e una riga aggiunta li ha
invecchiati lo stesso giorno** — esattamente come la nota che li accompagnava prevedeva. Questi
non dipendono da numeri di riga: delimitano per **intestazione**, così che un delimitatore
rinumerato dia un errore invece di **zero in silenzio** (gotcha **#26**).

```bash
# quante righe ha il blocco C del catalogo §7.4.1
awk '/^#### 7\.4\.1 /{ins=1} /^#### 7\.4\.2 /{ins=0}
     ins&&/^\*\*C · /{c=1}
     ins&&c&&/^\|/&&!/^\|-/&&!/^\| Difende/' \
  docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | wc -l

# quante ne dichiara implementate il registro
grep -cE '^\| \*\*blocco C\*\*' docs/porta-di-qualita.md

# e la controprova che nessun caso resti fuori dal registro
for f in crates/kernel/tests/compile_fail/*.rs; do
  b=$(basename "$f"); grep -qF "$b" docs/porta-di-qualita.md || echo "ORFANO: $b"
done
```

| Domanda | Esito il 2026-08-10 |
|---|---|
| righe del **blocco C** | **diciannove** — erano diciotto, e il piano lo attendeva |
| righe **dichiarate implementate** dal registro | **nove** — il registro ne dichiarava **sette**, ed erano già **otto** prima di questo commit |
| casi **orfani**, cioè non nominati dal registro | **nessuno** — prima di questo commit era uno, `record_without_version.rs` |

⛔ **La seconda riga è la divergenza che vale più delle altre, e il piano non la prevedeva.** Il
denominatore lo muove chi tocca il catalogo, e se ne accorge perché sta scrivendo lì; **il
numeratore lo muove chi scrive un caso di prova**, che il catalogo non lo apre nemmeno. Il Task 1
ha consegnato `record_without_version.rs` senza scriverne la riga nel registro, e nessun
controllo lo ha rilevato: la terza voce qui sopra esiste per questo, ed è l'unica delle tre che
scopre una **mancanza** invece di contare ciò che c'è.

## Esecuzione del Traguardo 3 — il Task 3: il criterio di chiusura che un giornale rotto soddisfa

Misurate il **2026-08-10**, stessa toolchain. Diciotto passate di mutazione su
`crates/simulator/src/journal.rs`, ciascuna **compilata in un passo separato** dall'eseguirla e
provata applicata — lo strumento rifiutava di scrivere se il modello non combaciava **esattamente
una volta**. Tabella completa nel [registro](porta-di-qualita.md).

```bash
cargo build -p simulator                       # la mutazione compila?
cargo test -p simulator --test memory_journal   # e adesso muore?
cargo test -p simulator --test memory_journal -- --exact <nome>   # una per processo
```

| Misura | Come | Esito il 2026-08-10 | Dove entra |
|---|---|---|---|
| ⛔ il **criterio di chiusura** del piano è soddisfatto da un giornale rotto | `outcome` che risponde **sempre** `Err(OutOfOrder)` | ⛔ **tutti e quattro** i test dettati dal piano restano **verdi** — cioè il suo `test result: ok. 4 passed` si ottiene con un giornale che **non registra nessun esito**. È la lacuna di **specie 2** più netta finora: il cammino felice del protocollo write-ahead non era provato. Le tre sonde che la uccidono sono aggiunte eseguendo | `crates/simulator/tests/memory_journal.rs` · specie 2 |
| la **mutazione di controllo**, senza cui la tabella non vale niente | cambiato **solo un commento**, poi l'intera passata | **nessun test rosso**. ⚠️ È la contro-prova del gotcha **#48** applicata al banco stesso: un banco che risponde rosso a tutto conferma qualunque tesi | metodo, ogni passata futura |
| ⛔ **stato globale di processo** dentro un `no_std` | `static AtomicBool` posato da `intent` e letto da `read_back` | **compila** sotto `#![no_std]` **e** `#![forbid(unsafe_code)]`, e rende rosso il solo test sul `drop`. Quindi quel test **non è vacuo**: tiene che il giornale non conservi niente fuori di sé — famiglia del gotcha **#12**, stato seminato **per processo** invece che per istanza. ⚠️ **In esecuzione condivisa l'esito dipende dalla popolazione del file**: il test gemello è sopravvissuto **5 volte su 5** con nove test e caduto **20 su 20** con il decimo, che scrive intenti e ordina prima. Una prova del genere si legge **un test per processo** | `..._does_not_survive_being_dropped` |
| una mutazione **viva e dichiarata** | `has_intent` che ignora il **tipo** della voce | **nessun test rosso, e non è una lacuna**: distingue uno stato **irraggiungibile**, perché il primo record di un passo può essere solo un intento. ⚠️ **L'equivalenza cade** quando `prune` rimuoverà voci selettivamente — compito **11** | registro |
| ⛔ l'ordine di scrittura **è** osservabile, contro l'attesa scritta | intenti scritti **in testa** invece che in coda | ⛔ **Divergenza registrata.** La prima stesura concludeva «invisibile dall'esterno» da una premessa vera — *ogni passo incontra il proprio intento prima del proprio esito* — che regge **solo con al più un intento per passo**. Il testimone è di **tre chiamate senza nessun esito**: `intent(1,"p0"); intent(1,"p1"); read_back(1)` dà `"p0"`, e rovesciato `"p1"`. Ne è uscita una **voce aperta**: se un secondo intento sullo stesso passo debba essere accettato | [registro](porta-di-qualita.md) · conformità |

## Esecuzione del Traguardo 3 — i Task 4 e 5: la conformità, e due promesse vacue contro il proprio bugiardo

Misurate il **2026-08-10**, stessa toolchain. La suite di conformità di `journal` sta in una
copia sola in `crates/kernel/tests/journal_contract.rs`; la tabella per nome è nel
[registro](porta-di-qualita.md).

```bash
cargo test -p kernel --test journal_contract                  # 7 passed
cp crates/platform/tests/journal_contract_real.rs …           # sonda usa-e-getta:
#   include!("../../kernel/tests/journal_contract.rs")         # → 7 passed dentro platform
```

| Misura | Come | Esito il 2026-08-10 | Dove entra |
|---|---|---|---|
| ⛔ una promessa **vacua contro il proprio bugiardo** | neutralizzata una promessa alla volta, commentandone il blocco | ⛔ la promessa sull'ordine di `replay` confrontava le **sole identità** dei passi, e la sequenza dettata `1, 2, 1` **è un palindromo**: `ShuffledJournal`, che rovescia il giornale, la superava e **passava la suite intera**. Chiusa confrontando i **record**, byte compresi | `journal_contract.rs` · specie 1 |
| ⛔ la via **A6** scattava senza saper dire di essere A6 | `SilentJournal` contro la promessa 1 | la rilettura usava `.expect("read_back must find it")`, e un giornale che non scrive risponde `Missing` **prima** dell'asserzione: il payload non nominava nessuna promessa e il test riportava *«ha sparato, ma NON sulla promessa 1»*. Il messaggio della promessa è ora **anche** sull'`expect` | `journal_contract.rs` · specie 1 |
| la corrispondenza **promessa ↔ bugiardo**, misurata e non argomentata | sei neutralizzazioni, una per promessa | **sei su sei**: cade **esattamente** il test del bugiardo di quella promessa, gli altri **sei restano verdi**. Nessuna promessa è decorativa, nessun bugiardo muore sulla promessa di un altro | [registro](porta-di-qualita.md) |
| la **mutazione di controllo** | cambiato **solo un commento** | **nessun test rosso** — `7 passed`. Senza, la tabella qui sopra non prova niente (gotcha **#48**) | metodo |
| ⛔ `assert_eq!` con messaggio **non** produce il messaggio | letto il payload di tutti e sei i panici | il payload è `` assertion `left == right` failed: <messaggio> `` più i due valori, quindi **non è mai uguale** alla costante: i test negativi dettati dal piano (`assert_eq!(caught.as_deref(), Some(MSG))`) sarebbero falliti in **cinque casi su sei**. Si confronta con `contains`, col vincolo che **nessun messaggio sia sottostringa di un altro** | `journal_contract.rs` |
| il file è **`include!`-abile** da `platform` | sonda usa-e-getta in `crates/platform/tests/`, poi rimossa | **7 passed** dentro il binario di `platform`, senza avvisi: `kernel` è dipendenza e `simulator` è già fra le `dev-dependencies` con la ragione scritta. Il Task 9 non incontra ostacoli | Task 9 |
| ⚠️ il **limite dichiarato** dell'hook di panic si è materializzato | esecuzione in parallelo dei sette test | un test è uscito `FAILED` **senza la propria sezione stdout**: il suo panico è caduto nella finestra in cui un altro test aveva silenziato l'hook, che è **globale al processo**. Il fallimento non si perde mai, solo il suo messaggio. Si rilegge con `-- --test-threads=1` | metodo · limite già dichiarato in `reactor_contract.rs` |

## Cosa NON abbiamo adottato, e perché

| Idea | Motivo |
|---|---|
| ACE — tripartizione Generator / Reflector / Curator | è una **politica** della capacità Agenti, non un meccanismo di kernel. Il nostro anello 4 ne copre l'intento (le lezioni ripiegate in guide) con un vincolo in più: la curatela passa dall'approvazione dell'utente ([ADR-0009](adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md)). Resta candidata per la spec della capacità Agenti |
| Evoluzione **automatica** del harness guidata dall'osservabilità (AHE) | ne adottiamo la diagnosi, non l'automatismo: un harness che si auto-modifica in silenzio è indebuggabile. Il sistema propone, l'utente approva |
| Framework e librerie citati dalle fonti | fuori perimetro: qui si progetta il comportamento, non la tecnologia. La selezione tecnica è materia di ADR successivi |
| OpenTelemetry GenAI come **formato di archiviazione** | adottato il vocabolario, rifiutata la dipendenza: la specifica è pre-stabile e legarvi il substrato durevole della ripresa renderebbe un cambio di attributi una migrazione dei dati di ripristino ([ADR-0017](adr/0017-giornale-sorgente-trace-proiezione.md)) |
| Piattaforme di observability gestite (Braintrust, LangSmith, Phoenix, Datadog…) | incompatibili con il local-first per default. Restano raggiungibili come **destinazione opt-in** dell'esportazione OTLP, scelta dall'utente |
| Macchina virtuale leggera (microVM) come confinamento **sempre attivo** | costo e complessità su ogni esecuzione, anche per far girare un linter. Resta il **livello 3**, da attivare quando servirà eseguire codice di provenienza ignota ([ADR-0025](adr/0025-confinamento-a-livelli.md)) |
| Passphrase come cifratura predefinita | romperebbe avvio automatico, daemon e voce always-on: tre requisiti strutturali. Resta il **profilo riservato**, che rinuncia esplicitamente a quei tre ([ADR-0023](adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md)) |

## Avvertenza sulla stabilità delle fonti

L'ecosistema si muove a cadenza mensile e una parte di queste fonti sono articoli
divulgativi, non letteratura sottoposta a revisione. Le tre **diagnosi** riprese qui
— guide vs sensori, i quattro anelli, il context rot — sono convergenti fra fonti
indipendenti e sono quelle su cui il design poggia. Le enumerazioni puntuali di
singoli articoli sono state usate come lista di controllo, non come autorità.
