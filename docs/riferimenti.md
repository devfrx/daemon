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
