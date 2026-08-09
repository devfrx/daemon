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
