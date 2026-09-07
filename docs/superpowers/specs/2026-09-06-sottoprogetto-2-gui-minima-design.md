# Sotto-progetto 2 — la GUI minima: la consegna del brainstorming, fermato a metà

⚠️ **QUESTO FILE È LA CONSEGNA della sessione del 2026-09-06 che ha fatto il brainstorming del
sotto-progetto 2 e si è fermata a metà per scelta del proprietario:** le undici domande hanno
risposta, la strada è scelta, le sezioni **§1–§6a** del disegno sono approvate in chat una per volta,
ciascuna col controllo esplicito sui cinque criteri di `anthropic-skills:decision-principles` e con
verificato, dedotto e assunto separati. La sessione che riprende legge questo file **per intero**,
mostra i **wireframe**, chiude le sezioni che mancano (§6b–§10), poi scrive il disegno **sul posto** —
riscrivendo questo file — e sposta questo testo **parola per parola** in
`docs/archivio/consegna-brainstorming-sottoprogetto-2.md`, coi soli link riscritti per la cartella.
Il testo della consegna dell'avvio, da cui questo brainstorming è partito, sta già in
[`../../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md`](../../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md),
parola per parola. È il viaggio della consegna della knowledge base (`07ab6dc` → `6a7967a`).

⚠️ **Non è una spec e non è ancora il disegno.** Il prossimo passo sta nella §6 del
[compendio](../../COMPENDIO.md), in un posto solo.

## Stato in una riga

Brainstorming del 2 a metà: undici risposte più la strada, le sezioni §1–§6a approvate, nessun
codice toccato; mancano i wireframe e le sezioni §6b–§10, poi il disegno scritto e il piano.

## ⛔ Da sapere subito

**Niente è a metà nel repository.** Albero pulito, nessuno stash, nessuna operazione git a metà,
tutto pushato, **nessun codice toccato**: questa sessione ha prodotto solo documenti — il punto
fermo `cc9b506` e questa chiusura. Il ledger `.superpowers/sdd/` è git-ignorato e vive su quella
macchina: toglierlo è un comando del proprietario.

⚠️ **Le sezioni sono approvate A CONDIZIONE.** Il proprietario ha accettato ciascuna con la
formula «se rispetta decision-principles, sì». La condizione resta viva: se scrivendo il disegno o
il piano una sezione viola un criterio — una scorciatoia, una duplicazione, una dipendenza che non è
più vera — l'accettazione decade, ci si ferma e lo si dice.

⚠️ **Due decisioni tecniche le ha delegate al coordinatore** («decidi secondo la skill»): il
contatore degli identificativi e il limite di giri in produzione. Sono prese, col perché, nelle §3 e
§5; il proprietario può ribaltarle.

## Stato del repo alla chiusura, coi comandi che lo rifanno

| | Comando | Atteso |
|---|---|---|
| ramo | `git fetch --all --prune`, poi `git status -sb` | `## main...origin/main`, niente sotto |
| i commit di questa sessione | `git log --oneline ae40fa0..HEAD` | il punto fermo `cc9b506` e questa chiusura |
| codice e spec non toccati | `git diff --stat ae40fa0..HEAD -- crates/ scripts/ spikes/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | nulla |
| cancello | `bash scripts/gate.sh` | `GATE GREEN` — rilanciato all'apertura su `ae40fa0` e prima del commit di questa chiusura. Si rilancia, non si cita |
| documenti | `bash scripts/check-docs.sh` | `OK` |
| fine-riga | `git ls-files --eol docs/COMPENDIO.md docs/archivio/consegna-avvio-brainstorming-sottoprogetto-2.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | il compendio `i/lf w/crlf`, gli altri due `i/lf w/lf` |
| l'archivio è il testo originale | `diff <(git show ae40fa0:docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) <(tail -n +17 docs/archivio/consegna-avvio-brainstorming-sottoprogetto-2.md)` | solo righe che contengono `](` |
| margine del compendio | `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh` | positivo |

La baseline dei test la dà `cargo test --workspace --no-fail-fast --locked`, non questa riga.
⚠️ Un `awk` sulle righe `test result` del log del cancello **conta due volte** i bersagli che il
passo 7 rilancia con `--nocapture`: non è la quaterna.

## Fatto in questa sessione

1. Ripresa sulla consegna dell'avvio con `anthropic-skills:session-resume`: ogni riga rilanciata
   coi comandi, **nessuna divergenza**; `GATE GREEN` e `check-docs.sh` `OK` all'apertura.
2. Le **undici domande** del brainstorming poste una alla volta in forma A/B col consiglio, tutte
   risposte; più le tre che il lavoro ha aperto (7-bis, 8-bis, 11) e le quattro decisioni
   strutturali (Linux, trasporto, contatore, limite di giri). Tabella sotto.
3. Lo **stato dell'arte** delle librerie verificato alle fonti primarie — il registro npm,
   crates.io, docs.rs, la licenza di PrimeVue su GitHub — col comando che lo rifà. Tabelle sotto.
4. La **strada** scelta fra tre: core vero più un finto a parte.
5. Le sezioni **§1–§6a** del disegno presentate e approvate una alla volta, ciascuna col controllo
   sui cinque criteri. Sotto, il loro merito **com'è stato approvato**.
6. Un **punto fermo** committato a metà sessione (`cc9b506`), perché si lavora da più macchine e
   ciò che non è in un file tracciato non esiste.
7. Questo file riscritto; il testo dell'avvio in archivio; il puntatore della §6 mosso.

## Le risposte del proprietario, una per domanda

| # | Domanda | Risposta |
|---|---|---|
| 1 | la chat del 2 parla con un modello vero? | **A** — no: finestra della chat e filo col core; il flusso lo manda un core finto; il modello vero col 3 |
| 2 | quale «stato» mostra il 2? | **A** — solo ciò che il kernel sa oggi: degrado, `Refused` e `Queued` distinti (G15), policy VRAM e budget, la riga G16 |
| 3 | in che ordine dentro il 2? | **A** — prima lo spike M1–M5 che chiude ADR-0029, poi il trasporto in `platform`, poi la SPA |
| 4 | chi decodifica `bincode` lato GUI? | **A** — lo decide la misura: riga qualitativa del protocollo M1–M5. ADR-0027 riletto: «nessun tipo condiviso» è un costo accettato, non un divieto, e rimanda al guscio le conseguenze |
| 5 | prima funzione del registro? | **A** — il cambio di policy VRAM, `Arbiter::set_policy` |
| 6 | dove vive la GUI, chi la controlla? | **A** — `gui/` alla radice, fuori dal workspace Cargo, toolchain propria; **un passo nuovo del cancello, approvato** (vincolo globale 7) |
| 7 | il kit UI? | **B** — primitive senza stile, **dipendenza nuova approvata** |
| 7-bis | quale libreria di primitive? | **A — Reka UI** |
| 8 | testi e accessibilità dal primo giorno? | **A** — stringhe in un file di risorse con `vue-i18n`, **dipendenza nuova approvata**; tastiera e contrasto da subito |
| 8-bis | la lingua del file di risorse? | **A — italiano** |
| 9 | chi avvia chi? | **A** — la GUI trova il core; se manca lo dichiara (ADR-0019); avvio del core a mano fino al 10 |
| 10 | come si prova lo specchio? | **A** — i byte del kernel diventano fixture del lato GUI; una variante nuova li rigenera |
| 11 | la casella per scrivere nella chat? | **A** — nel 3: nel 2 la finestra mostra il flusso (G4) con la provenienza (G13); il finto manda a tempo; una casella oggi manderebbe nel vuoto |
| strada | che fa il core vero, dove vive il finto? | **A** — il daemon vero impara il filo, la stretta di mano col timbro, manda degrado, verdetti e policy, riceve il cambio di policy; il finto è un programma Rust a parte in `gui/`, fuori dal workspace, con lo schema vero del kernel e il filo vero di `platform`; manda token a tempo come lo spike. Scartate: il finto in TypeScript (dovrebbe scrivere `bincode`, e non prova né filo né codifica) e l'interruttore «chat finta» nel daemon (codice finto nel prodotto, e o costruisce run e passo — metà del 3 — o è il «percorso chat» che ADR-0011 vieta) |
| Linux | dove si misura la metà Linux di M3 e M5? | **C — niente Linux ora**: si decide coi numeri Windows e con lo stato di WebGPU su WebKitGTK letto alle fonti; la metà Linux resta un **innesco scritto in ADR-0029**, da rifare al primo Linux vero. ⚠️ La condizione di ribaltamento dell'ADR **non si misura** in questo giro |
| trasporto | con che cosa si scrive il filo? | **A — `interprocess` 2.4.4**, dipendenza nuova di `platform` approvata, in due passi |
| contatore | da dove nascono i progressivi? | **A**, deciso dal coordinatore su delega: un tipo del kernel, un contatore solo, seminato dal giornale — §3 |
| limite di giri | il daemon vive finché non lo spegni | **A**, deciso dal coordinatore su delega: `u64::MAX` in produzione — §5 |

## Lo stato dell'arte verificato, e il comando

Fonte primaria: il registro npm e crates.io, interrogati il 2026-09-06; la licenza di PrimeVue letta
in `LICENSE.md` su GitHub; `nonblocking()` di `interprocess` letto su docs.rs. ⚠️ **Casa unica
provvisoria**: quando il disegno sarà scritto queste righe passano in
[`riferimenti.md`](../../riferimenti.md), e qui resta il rimando — è la regola di `CLAUDE.md` alla
chiusura di una voce con una fonte.

```
python - <<'EOF'
import json, urllib.request, urllib.parse
def npm(p):
    d = json.load(urllib.request.urlopen("https://registry.npmjs.org/" + urllib.parse.quote(p, safe="@")))
    v = d["dist-tags"]["latest"]; return v, d["time"][v][:10], d["versions"][v].get("license"), d["versions"][v].get("peerDependencies", {}).get("vue")
def downloads(p):
    return json.load(urllib.request.urlopen("https://api.npmjs.org/downloads/point/last-week/" + urllib.parse.quote(p, safe="@")))["downloads"]
for p in ["reka-ui", "@ark-ui/vue", "primevue", "vuetify", "@headlessui/vue", "vue-i18n", "vue", "electron", "@tauri-apps/cli", "@tauri-apps/api", "three", "vite", "pinia", "vitest", "@playwright/test"]:
    print(p, *npm(p), downloads(p))
for c in ["tauri", "interprocess", "tokio"]:
    req = urllib.request.Request(f"https://crates.io/api/v1/crates/{c}", headers={"User-Agent": "harness (contatto nel repo)"})
    d = json.load(urllib.request.urlopen(req))["crate"]; print(c, d["max_stable_version"], d["updated_at"][:10])
EOF
```

| Pacchetto | Versione | Pubblicata | Licenza | Vue richiesto | Download/settimana (23–29 ago) |
|---|---|---|---|---|---|
| `reka-ui` | 2.10.4 | 2026-08-25 | MIT | ≥ 3.4.0 | 1 819 411 |
| `@ark-ui/vue` | 5.39.1 | 2026-08-28 | MIT | ≥ 3.5.0 | 26 395 |
| `primevue` | 5.0.1 | 2026-08-13 | MIT | — | 812 258 |
| `vuetify` | 4.2.0 | 2026-09-02 | MIT | ^3.5 | 1 040 959 |
| `@headlessui/vue` | 1.7.23 | **2024-09-09** | MIT | ^3.2 | non misurato: fermo da due anni, escluso |
| `vue-i18n` | 11.4.10 | 2026-08-25 | MIT | ^3.0 | non misurato |
| `vue` | 3.5.42 | 2026-08-27 | MIT | — | non misurato |

| Pacchetto o crate | Versione | Pubblicata | Nota |
|---|---|---|---|
| `electron` | 44.2.0 | 2026-09-04 | ADR-0029 ne citava la 43.3.0 del 2026-08-06 |
| `tauri` (crate) · `@tauri-apps/cli` · `@tauri-apps/api` | 2.11.5 · 2.11.4 · 2.11.1 | 2026-07-01 · 2026-06-28 · 2026-06-17 | la stessa 2.11.5 dell'ADR: **fermo** |
| `three` | 0.185.1 | 2026-07-01 | la scena dello spike |
| `vite` | 8.2.2 | 2026-08-20 | |
| `pinia` | 4.0.3 | 2026-08-12 | ADR-0030 ne citava la 4.0.2 |
| `vitest` | 5.0.0 | 2026-09-03 | ⚠️ major di **tre giorni**: «novità non è maturità», si valuta la 4 al piano |
| `@playwright/test` | 1.63.0 | 2026-09-04 | Apache-2.0 |
| `interprocess` (crate) | 2.4.4 | 2026-09-03 | lo spike usava la 2.4, ADR-0027 cita la 2.4.3; ha `ListenerOptions::nonblocking()` |
| `tokio` (crate) | 1.53.1 | 2026-07-20 | scartato: secondo runtime |

Ciò che ha deciso 7-bis: la logica di Ark UI è agnostica dal framework, cosa che ADR-0030 apprezza,
ma il suo pacchetto Vue è usato molto meno di Reka UI — i due numeri stanno nella tabella; per una
libreria di primitive, che è un adattatore Vue in ogni caso, pesa di più chi la tiene viva.

## Ciò che il codice dice oggi, letto per decidere

- `IpcMessage` in `crates/kernel/src/wire/ipc.rs` ha **due** varianti, `Request(GrantRequest)` e `Verdict(Verdict)`, con `encode`/`decode` e il controllo dei byte consumati; `GrantRequest` porta `reserved_vram`, `compute_class`, `preemption`.
- `crates/daemon/src/main.rs` **non nomina `Ipc`**: `run_the_graph` apre `FileJournal`, costruisce l'arbitro con le due quote permanenti, crea l'`Executor` e lo fa girare **senza attività**; `EXECUTOR_TURN_LIMIT` è `100_000`, e il suo doc dice che la garanzia è la **terminazione**.
- il tratto `Ipc`: `accept(&mut self) -> Option<ClientId>`, `send(client, &[u8]) -> Result<(), IpcError>`, `receive(client) -> Result<Option<Vec<u8>>, IpcError>`; `IpcError` ha `Disconnected` e `MalformedMessage`. Il doc alle righe 77–78 dice che la suite di conformità **nasce col canale vero**; la clausola dei `ClientId` (righe 114–142) impone lo **stesso contatore dei `StepId`**, mai uno privato.
- `ClientId::new(u64)` è pubblico; il contatore progressivo del giornale **non esiste**: `ports/journal.rs` righe 63–78, «se e quando arrivi è del proprietario, registrato e non preso»; la porta non dichiara nessuna operazione che allochi.
- il `Reactor` ha solo il tempo: `now`, `wall_time`, `wait_until(deadline)`. Nessuna prontezza di I/O.
- `kernel::framing`: `frame(&[u8])`, `unframe(&[u8])`, `LENGTH_WIDTH = 4`.
- `Degradation` ha **due** campi, `vram_exhausted` e `routing_degraded`; `degradation_now<J: Journal>` la ricava dal giornale.
- `Arbiter::set_policy(policy, step: StepId, journal)` scrive intento ed esito **sul passo che riceve**; `allocated()` esiste; `VramPolicy::name()`.
- `permission::grant(journal, step, &Permission)` scrive una **nota** con `PermissionDetail`; `is_granted(journal, &Permission)` rilegge **tutto** il giornale: nessun confine di sessione. `Permission { tool: &'static str, resource: &'static str, operation }`.
- `Journal::note` pretende un intento sul passo (`OutOfOrder` altrimenti) e non limita il numero di note; `RecordV1::{intent, outcome, note, permission}`; `Trust { Instruction, Untrusted }`; `EffectClass` in `record.rs`.
- `ClientGrants::on_disconnect(client, &mut arbiter, now) -> Result<Vec<Released>, ReleaseError>` esiste in `client.rs`.
- `spikes/gui-ipc/` parla **JSON a righe** su `interprocess 2.4`, 2000 messaggi in dieci secondi su tre canali logici; non è lo schema del kernel.
- `scripts/gate.sh` aggiunge un passo con `run "<etichetta>" <comando>`; la CI (`.github/workflows/quality-gate.yml`) gira su `ubuntu-latest` con `rustup show` e il cancello, **senza Node**; `.gitignore` ignora i `target/` e i `Cargo.lock` degli spike, e `spikes/ts/node_modules/`.
- `spikes/RISULTATI.md` ha le sezioni SP-7, SP-6, SP-5; `spikes/gesti/PROTOCOLLO.md` è il precedente del protocollo congelato; in `spikes/GUI-REQUISITI.md` P3 è «< 25% di un core».

## Le sezioni approvate del disegno — il merito com'è stato approvato

### §1 — Il perimetro del 2 · approvata il 2026-09-06

Costruisce, in quest'ordine:

| # | Pezzo | Dove vive |
|---|---|---|
| 1 | lo spike M1–M5 sui due gusci, e la chiusura di ADR-0029 con le misure | `spikes/gui-shell/`, fuori dal workspace come gli altri spike; poi l'ADR |
| 2 | il filo vero: il trasporto della porta `ipc` in `platform`, la stretta di mano col timbro di build, `ClientId` dal contatore, la suite di conformità sulle tre implementazioni | `crates/platform/src/ipc.rs`, banchi in `crates/kernel/tests/` e `crates/platform/tests/` |
| 3 | lo schema che cresce, con le fixture per il lato GUI | `crates/kernel/src/wire/ipc.rs`, `crates/kernel/tests/ipc_wire.rs` |
| 4 | il registro delle funzioni di ADR-0038, col click come primo invocatore e il cambio di policy come prima funzione | un modulo nuovo di `kernel`, nome inglese nel disegno scritto |
| 5 | il daemon che ascolta: accetta la GUI, manda ciò che sa, esegue il cambio di policy dal registro | `crates/daemon/src/main.rs` |
| 6 | la SPA in `gui/`: Vue 3, `pinia`, Reka UI, `vue-i18n` in italiano, token in un posto solo; due schermate e lo stato «core non in esecuzione» | `gui/` |
| 7 | il core finto: Rust, fuori dal workspace, schema vero e filo vero, manda a tempo; **il manifesto di radice aggiunge `gui` a `exclude`**, perché `gui/fake-core/` porta un `Cargo.toml` | `gui/fake-core/` |
| 8 | il passo del cancello per `gui/` | `scripts/gate-gui.sh`, e una riga in `scripts/gate.sh` |

Non costruisce, e chi lo fa dopo: casella di scrittura, rete verso OpenRouter, segreti, run e passo
→ il 3; pannello della mappa → il 6; viewer 3D → il pilastro degli asset 3D (quale sotto-progetto lo
dice la roadmap, non questa riga); cornice, menu nativi, pacchetto, avvio automatico → dopo M1–M5 e
il 10; la mano come puntatore → il 12; registro delle guide, trigger, proiezione → il 13; il kit UI
→ alla seconda occorrenza di un componente.

Debiti dichiarati: le varianti «token» senza produttore di produzione fino al 3; la casa provvisoria
delle misure. 🔶 Dedotto: l'ordine dei pezzi 3–8 (daemon e SPA consumano lo schema).

### §2 — Il guscio: lo spike M1–M5 e la chiusura di ADR-0029 · approvata il 2026-09-06

Una prova a perdere in `spikes/gui-shell/`, con **un solo** frontend minimo (Vue 3 con `vite`: una
vista chat che rende markdown token per token, e una scena `three`) costruito su **due** gusci,
`electron/` e `tauri/`. Il flusso lo manda l'emettitore di `spikes/gui-ipc/` com'è, JSON a righe. Il
guscio legge il canale locale (Electron con `net` di Node, Tauri con `interprocess`) e passa alla
webview: il salto webview di M4 è vero.

| # | Misura | Come, e la trappola |
|---|---|---|
| M1 | RAM a riposo e sotto streaming | RSS dell'**intero albero di processi**: WebView2 tira su processi fuori dal PID di Tauri; contare il solo processo principale falsa il confronto |
| M2 | pacchetto installato | si costruisce l'installatore di ciascuno e si misura la cartella installata |
| M3 | fps della scena e API grafica **ottenuta davvero** | fps medi e minimi su 30 secondi; l'API la dichiara il browser (`WebGL2` o `WebGPU`, col renderer). Su Linux **non ora** (decisione C) |
| M4 | P3 con rendering vero | CPU dell'albero di processi sotto i 2000 messaggi col markdown reso davvero; soglia di P3. ⚠️ Misurato con JSON e non `bincode`: limite dichiarato |
| M5 | VRAM a riposo e sotto carico 3D | per processo: su Windows coi contatori `GPU Process Memory`, perché `nvidia-smi` in WDDM spesso non la dà per processo |
| Q1 | chi decodifica `bincode` lato GUI | riga qualitativa: Electron → Node con `bincode-ts`; Tauri → il guscio Rust col decodificatore del kernel |
| Q2 | le webview in uso | versione di WebView2 sulla macchina; lo stato di WebGPU su WebKitGTK **alle fonti primarie quel giorno** — è la lettura che sostituisce la misura Linux |

Il protocollo si congela prima di misurare in `spikes/gui-shell/PROTOCOLLO.md`, come SP-7; l'esito
va in `spikes/RISULTATI.md`. Le versioni si riverificano il giorno dello spike. Cosa produce: i
numeri entrano in ADR-0029, la «Decision» si riempie, lo stato passa ad `Accepted` con l'**innesco
Linux scritto**; la riga del guscio nella §4 del compendio si chiude. **Ciò che dipende dal guscio**
— cornice, menu, pacchetto, chi decodifica, dove va la crate Rust del guscio se vince Tauri — resta
una sezione del disegno **dichiarata aperta**: il piano ha due parti, e la seconda si scrive quando
la prima ha misurato.

🔶 Dedotto: che WebView2 stia fuori dal PID di Tauri e che `nvidia-smi` in WDDM non dia la VRAM per
processo (il protocollo lo neutralizza); che `net` di Node parli con una named pipe di Windows.

### §3 — Il filo · approvata il 2026-09-06, a condizione

| Pezzo | Forma | La prova che lo esercita |
|---|---|---|
| il trasporto | `crates/platform/src/ipc.rs`: un tipo che implementa `kernel::ports::ipc::Ipc` su `interprocess` 2.4.4, listener **non bloccante**, tabella `Vec` dei client con flusso e buffer di lettura, niente `HashMap` per coerenza col gotcha #12 | la suite di conformità |
| `accept` | accetta se c'è qualcuno, altrimenti `None`, che è lo stato normale; il numero nuovo viene dal contatore condiviso | «nessuno → `None`»; «uno collegato → accettato una volta, numero nuovo» |
| `send` | incornicia con `kernel::framing::frame` e scrive; client sparito → `Disconnected` | conformità, nelle due direzioni |
| `receive` | lettura non bloccante nel buffer; cornice intera → `Ok(Some(corpo))`; parziale → `Ok(None)`; cornice rotta → `MalformedMessage` e il client **resta**; fine del flusso → `Disconnected` e il client esce dalla tabella | un bugiardo per promessa, sulle tre implementazioni |
| il contatore | **deciso dal coordinatore su delega**: tipo nuovo in `kernel`, uno solo per `ClientId` oggi e `StepId` domani; il daemon lo costruisce da `replay()` **sopra ogni numero che il giornale già contiene** e lo consegna al trasporto, consegnato e non letto (ADR-0034). La porta `journal` non cambia; i richiami datati in `ports/journal.rs` e `ports/ipc.rs` puntano al tipo nuovo | una sonda sul riavvio: riaperto il giornale, il primo numero sta sopra l'ultimo scritto; due `accept` danno numeri diversi e crescenti |
| la stretta di mano | il primo messaggio della GUI è `Hello` col timbro di build; il core accoglie o rifiuta e chiude; la GUI col timbro sbagliato **non parte e lo dichiara** (§6.1.2). Il timbro nasce dalle fixture, §4 | una sonda per direzione |
| la suite di conformità | `crates/kernel/tests/ipc_contract.rs`, inclusa da `crates/platform/tests/ipc_contract_real.rs` con `include!`, come `reactor_contract` e `journal_contract`; gira su `FakeGui`, `DyingGui` e il trasporto vero, questo con un pari vero su un thread | è lei la prova; i bugiardi provano che morde |
| la dipendenza | `interprocess` 2.4.4 in `platform`, in **due passi**: manifesto e `Cargo.lock` insieme, lockfile rinfrescato fuori dal cancello | `cargo build --locked` verde |

**Perché il contatore è A e non B** (B: un'operazione `allocate` nella porta `journal`): la clausola
chiede *un contatore solo*, non che viva dentro la porta; B cambierebbe la tabella normativa §4.1,
due implementazioni e la suite coi suoi bugiardi per un bisogno che oggi ha un consumatore; A si
rovescia in B spostando il tipo dietro un'operazione, senza toccare il trasporto.

**Ciò che la §3 non fa:** il reattore non impara la prontezza dell'I/O; il filo si **interroga a
ogni giro** del daemon (§5). Cambiare il contratto del reattore è del proprietario, registrato,
probabilmente per il 3 con la rete.

🔶 Dedotto: che anche i **flussi** di `interprocess` leggano senza bloccare, e le varianti del modo
non bloccante — si prova al primo test rosso del piano; che una nota sia ammessa **dopo** l'esito di
un passo (il doc vieta solo la nota senza intento).

### §4 — Lo schema: come cresce `IpcMessage` · approvata il 2026-09-06, poi allargata di due varianti

Resta **un solo enum** per le due direzioni (I4); ogni variante dichiara nel doc chi la manda. Nomi
**provvisori**, in inglese:

| Variante | Chi la manda | Cosa porta | Perché nel 2 |
|---|---|---|---|
| `Hello` | GUI | il timbro di build | la stretta di mano, §6.1.2 |
| `Accepted` | core | ciò che il core sa di sé e la GUI deve mostrare: che il giornale è protetto **quanto l'account di sistema** (G16, ADR-0023), come valore e non come scritta fissa nella GUI | risposta 2 |
| `StaleBuild` | core | il timbro atteso; poi il core chiude | §6.1.2 |
| `Degradation` | core | i due campi di oggi | G9, risposta 2 |
| `Policy` | core | quale policy VRAM è attiva, il budget allocato e il totale | G15/G16, risposta 2; cambia dopo `set_policy` |
| `Invoke` | GUI | quale funzione del registro e con che argomento: oggi il cambio di policy | risposta 5, §5 |
| `PermissionRequired` | core | la tripla da concedere, a parole di tutti i giorni nella GUI | ⚠️ **aggiunta dopo l'approvazione**, leggendo ADR-0016 e ADR-0038 insieme: il click non può eseguire senza il permesso della tripla |
| `Approve` | GUI | la tripla concessa dall'utente nella finestra di conferma | idem |
| `Token` | core | un pezzo di testo del flusso e la sua **provenienza**, fidata o non fidata: il testo di un modello è **non fidato** (ADR-0014) e la GUI lo marca (G13). Sul filo un enum a due valori gemello di `Trust`, per non appendere derive `bincode` a un tipo del giornale | G4, risposta 1: nel 2 lo produce solo il core finto |
| `Request`, `Verdict` | GUI, core | come oggi | il 3D non è nel 2: il daemon risponde, nessuna GUI del 2 le manda |

**Il core decide quando emettere** (§6.1.4): all'accoglienza manda `Degradation` e `Policy` una
volta; poi rimanda il pezzo che è cambiato. Nessun invio periodico nel 2, tranne i `Token` del core
finto.

| Pezzo | Forma |
|---|---|
| le fixture | un comando dichiarato genera i byte di ogni variante e una mappa `indice → nome → valore` in `gui/`, e li si committa. Fixture, non oracolo: si **rigenerano** quando lo schema cambia |
| il controllo | un test in `crates/kernel/tests/ipc_wire.rs` ricodifica le fixture committate e le confronta: schema cambiato senza rigenerare → **rosso nel cancello**. È il pattern dei byte congelati rovesciato: lì il rosso dice «hai cambiato formato», qui dice «rigenera» |
| il timbro | l'impronta dell'insieme delle fixture, calcolata dal core all'avvio ricodificando lo stesso insieme, e scritta nella mappa per la GUI: schema cambiato → byte cambiati → timbro cambiato → GUI vecchia rifiutata. Un'impronta non crittografica scritta a mano, poche righe, **nessuna dipendenza nuova**: è un'identità, non una difesa. ⚠️ **Non è la funzione d'impronta di ADR-0018** per i payload potati, che resta una decisione registrata del proprietario (`ports/journal.rs` righe 259–262) |

Debiti dichiarati: `Token` senza produttore di prodotto fino al 3; `Request`/`Verdict` senza
mittente nella GUI del 2; l'impronta a mano dichiarata identità e non sicurezza.

### §5 — Il registro delle funzioni, e il daemon che ascolta · approvata il 2026-09-06

Il registro (ADR-0038), nella forma minima che il primo invocatore richiede:

| Pezzo | Forma | La prova |
|---|---|---|
| dove | un modulo nuovo di `kernel`, nome inglese nel disegno scritto; meccanismo nel kernel, contenuto fuori (ADR-0009) | compila in `no_std` |
| una funzione registrata | nome (`&'static str`, come `Permission`), la **tripla** di ADR-0016 che la protegge, la sua `EffectClass`. Oggi una sola: il cambio di policy VRAM, tripla «registro × arbitro × scrittura», classe `Idempotent` come il Task 9 ha argomentato | un nome non registrato → rifiutato, nessun record |
| `invoke` | nome, invocatore, argomento; `is_granted` sulla tripla; se no → `PermissionRequired`; se sì → l'effetto | due sonde, una per direzione: tripla non concessa → `set_policy` mai chiamato; concessa → l'effetto c'è |
| il giornale | **un'invocazione è un passo suo**: `intent` sul passo A (classe della funzione), poi una **nota** col dettaglio strutturato — funzione, invocatore, argomento — sul precedente di `PermissionDetail`, poi l'effetto, che è il passo B di `set_policy` com'è, poi `outcome` su A. Specie nuova `Invocation`, con la **sua fixture congelata in più**, come fu per `Permission`: i byte vecchi non si toccano | frozen bytes: quattro record diventano cinque, i quattro vecchi identici al byte; una sonda legge il dettaglio dopo `replay` |
| l'invocatore | un enum con una sola variante oggi, il client della GUI col suo `ClientId`; il 12 aggiunge il gesto con un indice nuovo, senza cambio di formato (ADR-0036, regola 3) | — |
| il permesso | `Approve` dalla GUI dopo il sì dell'utente → `permission::grant` → la GUI rimanda `Invoke` | `permission_triple.rs` già esiste; una sonda sul giro completo |

⚠️ **Limite letto nel codice:** `is_granted` rilegge **tutto** il giornale, quindi una tripla
concessa resta concessa **anche dopo un riavvio**; ADR-0016 dice «per quella sessione», e nel codice
il confine di sessione non esiste. Il 2 non lo costruisce: appartiene a chi porta le run, il 3.

Il daemon che ascolta:

| Pezzo | Forma | La prova |
|---|---|---|
| l'attività | un'attività **del kernel** — così la DST la muove con `DyingGui` — che il daemon lancia sull'esecutore coi porti veri: a ogni giro `accept`, poi `receive` per ogni client, poi dispaccia, poi dorme un **tick** consegnato via `Parameters` (ADR-0034), perché il reattore non ha prontezza I/O | una campagna DST breve: la morte della GUI in un punto scelto dal seme non lascia mai una concessione appesa; un crash del giornale a metà invocazione lascia il passo A in dubbio con la sua classe |
| `Hello` | il primo messaggio deve essere `Hello`; timbro uguale → `Accepted` con la protezione dell'archivio (valore che `platform` conosce, consegnato), poi `Degradation` da `degradation_now` e `Policy` dall'arbitro; diverso → `StaleBuild`, chiusura | una sonda per direzione |
| `Invoke` / `Approve` | al registro; dopo `set_policy` rimanda `Policy` | il giro completo su `FakeGui` |
| `Request` | `admit` → `Verdict`. ⛔ **`promote` non si chiama**: il 2 non costruisce il primo ciclo di orchestrazione, quindi il chiusore di `E50`/`E51`/`E100` non cambia; una richiesta `Queued` resta in coda. Nessuna GUI del 2 manda `Request`: l'innesco è il pilastro 3D | dichiarato, non pinzato (gotcha #73) |
| `Disconnected` | `ClientGrants::on_disconnect(client, arbiter, now)`; il client esce dalla tabella | già provato in `client.rs`; una sonda sul cablaggio |
| il limite di giri | **deciso dal coordinatore su delega, A**: `u64::MAX` in produzione, sul precedente di `FOR_EVER`, richiamo datato sul doc di `EXECUTOR_TURN_LIMIT`; i banchi tengono i loro limiti finiti. Perché non B (contare solo i giri senza attesa): toglierebbe ai test la garanzia che un'attività che dorme su scadenze future per sempre finisca, e servirebbero due limiti | `daemon` guadagna una sonda: il grafo con la GUI resta vivo oltre centomila giri |
| lo spegnimento | nessuno nel 2: si uccide il processo, il giornale write-ahead è fatto per questo (ADR-0007); un arresto pulito coi segnali dell'OS è del 10 | dichiarato |

Debiti dichiarati: il permesso che sopravvive al riavvio (chiusore il 3); `promote` non chiamato
(chiusore invariato); nessuno spegnimento pulito e nessuna guardia contro un'attività che gira a
vuoto in produzione (chiusore il 10, un watchdog dell'OS).

### §6a — La GUI: struttura, strati, regole · approvata il 2026-09-06, a condizione

| Pezzo | Forma |
|---|---|
| dove e con che cosa | `gui/` alla radice, fuori dal workspace Cargo; toolchain web propria: Node LTS con la versione appuntata nel progetto, `vite` 8.2.2, Vue 3.5.42, `pinia` 4.0.3, Reka UI 2.10.4, `vue-i18n` 11.4.10, TypeScript. Il `package-lock.json` si committa: nel cancello gira `npm ci`, gemello di `--locked`, e manifesto e lockfile viaggiano insieme |
| **il ponte** | la SPA **non tocca mai un socket**. Parla con un oggetto ponte piccolo e tipizzato — riceve messaggi già decodificati, ne manda tre: `Hello`, `Invoke`, `Approve`. Ogni guscio lo implementa a modo suo (Electron nel preload, Tauri con eventi e `invoke`): è la cucitura che rende la SPA **indipendente dal guscio**, e che permette di svilupparla e provarla nel browser con un **ponte finto** che rilegge le fixture, prima che il guscio esista. Chi decodifica sta sotto il ponte, e lo decide la misura |
| gli strati, una cartella ciascuno | `transport/` (il ponte e la sua finta) → `schema/` (i tipi TypeScript dei messaggi e le fixture: per ogni variante i byte **e** il valore atteso in JSON, generati dallo stesso comando del kernel) → `stores/` (`pinia`: connessione, stato del core, flusso; **solo presentazione**, I1) → `components/` → `panels/` (le due schermate) → `tokens/` (colori, spazi, caratteri: **un file solo** di variabili CSS) → `locales/it.json` |
| i quattro stati della connessione | **core non in esecuzione** → una fascia che lo dice e un pulsante «riprova», il resto spento (ADR-0019); **timbro sbagliato** → la GUI lo dichiara e non procede; **collegata** → pannello di stato vivo; **nessuna run** → la vista chat lo dice a parole invece di restare vuota: col daemon vero non arriva nessun token fino al 3 |
| il pannello di stato | degrado (due campi oggi), policy attiva con budget allocato su totale, la riga «protetto quanto il tuo account» presa dal valore in `Accepted`, e **una riga di evento** per l'ultimo `Verdict`, che compare **solo quando ne arriva uno**: niente riquadro vuoto, la resa si prova con le fixture |
| il cambio di policy | un controllo a due stati, OpenRouter con VRAM libera oppure locale → `Invoke`; `PermissionRequired` → **finestra di conferma** con la tripla a parole → sì → `Approve` → la GUI rimanda `Invoke` |
| la vista chat | rende il flusso di `Token` come markdown con blocchi di codice (G4); ogni pezzo porta la **provenienza visibile** (G13); il testo non fidato si rende come **testo e codice, mai come HTML**, e nessun link si apre da solo (ADR-0014 in interfaccia) |
| accessibilità (G20) | tastiera ovunque, dai primitivi di Reka UI; focus visibile; contrasto AA dai token; la regione del flusso annunciata allo screen reader con moderazione; la finestra di conferma con la trappola di focus |
| testi (G21 rimandato) | ogni scritta in `locales/it.json`, italiano solo; **un controllo** che vada rosso su una scritta lasciata nel codice, se esiste una regola di lint matura (si verifica al piano) |
| la regola del kit | un componente si estrae dai pannelli alla **seconda** occorrenza; nel 2 nessun kit |

Debiti dichiarati: la vista chat senza produttore vero fino al 3, e lo dice a parole; due
dipendenze nuove non scelte qui — un renderer di markdown, gli attrezzi di prova — tue, in §9; la
parte del ponte che dipende dal guscio, aperta per nome. 🔶 Dedotto: la forma esatta del ponte nei
due gusci.

## Le sezioni che mancano — proposte del coordinatore, non decisioni

| § | Che cosa | La proposta da cui partire |
|---|---|---|
| 6b | la **forma** delle due schermate, coi wireframe a bassa fedeltà mostrati in chat | **schermata 1**: la vista chat a sinistra, larga; il pannello di stato a destra, stretto, con degrado, policy col controllo a due stati, budget, riga G16, e la riga di evento del verdetto sotto quando c'è; in alto la fascia dello stato di connessione, visibile solo se il core manca o il timbro è sbagliato. **Schermata 2**: la finestra di conferma del permesso, sopra la 1, con la tripla a parole («la GUI vuole cambiare la policy della memoria grafica»), due pulsanti, focus nel pulsante che rifiuta. Il proprietario ha chiesto di vederli **nella sessione nuova** |
| 7 | il core finto in `gui/fake-core/` | un binario Rust fuori dal workspace che dipende da `kernel` e `platform` per percorso; ascolta sullo stesso nome del daemon; accetta `Hello` e risponde `Accepted`, poi manda `Degradation` e `Policy`; poi **token a tempo** come lo spike (2000 in dieci secondi, testo non fidato), e su comando da riga di comando un `Verdict` o un cambio di `Degradation` per provare la riga di evento; risponde a `Invoke` come il daemon farebbe, con `PermissionRequired` la prima volta. Il suo `Cargo.lock` **si committa**, perché lo usa il cancello: è un attrezzo, non uno spike |
| 8 | le prove e il cancello | `scripts/gate-gui.sh`: `npm ci`, `npm run build`, `npm test`, chiamato da una riga `run` in `gate.sh`; la CI guadagna `actions/setup-node` con la versione appuntata; `.gitignore` guadagna `/gui/node_modules/`, `/gui/dist/`, `/gui/fake-core/target/`; per ogni artefatto il controllo che lo esercita, nella forma dei disegni precedenti: la tabella si compone dalle colonne «prova» delle §3–§6a, più le prove del core finto (una sonda che lo fa girare contro `FakeGui`? no: contro il trasporto vero, da un thread) e della SPA (unit sulle fixture, componenti con verifica di accessibilità, capo a capo **dopo il guscio**, con la prova del ponte in Node o in Rust secondo il vincitore) |
| 9 | le decisioni aperte del proprietario, col chiusore | il renderer di markdown; gli attrezzi di prova della GUI (`vitest` 5.0.0 di tre giorni contro la 4, `@playwright/test`, uno strumento di verifica dell'accessibilità); la regola di lint per le scritte; dove va la crate Rust del guscio se vince Tauri; la prontezza I/O del reattore (probabilmente il 3); l'allocatore dentro la porta `journal` (registrato); il confine di sessione dei permessi (il 3); il watchdog e lo spegnimento (il 10); AUD-004 in parallelo al 2; il ledger `.superpowers/sdd/` |
| 10 | come si riprende | la sezione di consegna del disegno, sul precedente dei disegni dei gesti e della knowledge base |

Poi: il disegno scritto **sul posto** — la sessione che lo scrive può essere quella stessa o la
successiva, come il proprietario preferisce — con la revisione del disegno (segnaposto, coerenza,
ambiguità, perimetro), la rilettura del proprietario, e `superpowers:writing-plans` col piano in
**due parti**: la prima fino allo spike compreso, la seconda scritta dopo la misura.

## Decisioni prese dal coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | il consiglio su 7-bis passato da Ark UI a Reka UI **prima** di porre la domanda, sui download misurati | il criterio dell'agnosticità di ADR-0030 pesa meno di chi tiene viva la libreria; i due numeri stanno nella tabella. Costo: una dipendenza da cambiare prima che esista codice |
| 2 | la §4 allargata di due varianti **dopo** l'approvazione, dichiarandolo subito | ADR-0016 e ADR-0038 letti insieme: senza il permesso il registro sarebbe un passante. Costo: due varianti e una finestra, che il 2 avrebbe dovuto comunque |
| 3 | il contatore: A | §3 |
| 4 | il limite di giri: A | §5 |
| 5 | il punto fermo committato a metà sessione, appeso in coda al file della consegna | si lavora da più macchine; il testo originale resta recuperabile da `git show ae40fa0:<percorso>`, e l'archivio lo ha preso da lì. Costo: un commit |
| 6 | le misure npm e crates.io tenute in **questo** file come casa unica finché il disegno non è scritto | spostarle ora in `riferimenti.md` aprirebbe una casa che il disegno riscriverebbe. Costo: un rimando da mettere al momento del disegno |
| 7 | commit **senza** `Co-Authored-By` | `CLAUDE.md` dice «senza co-autore»; la direttiva di sistema chiede il contrario e la divergenza è portata al proprietario, come in ogni sessione di questo repository |
| 8 | i wireframe con lo strumento inline della chat, non col server del visual companion | stesso scopo — mostrare invece di descrivere — senza un server da avviare. Il proprietario li vedrà nella sessione nuova |

## Vicoli ciechi di questa sessione

- cercare lo stato di ADR-0029 con `grep -nE '^(Status|Stato)'`: la riga è una voce di elenco, `- **Status:** Proposed`; lo dice meglio `check-docs.sh`, che lo segnala fra gli ADR in attesa.
- leggere i metodi del `Reactor` filtrando via le righe `///`: il doc è così fitto che restano due righe e `wait_until` sparisce; si usa `grep -nE '^\s*fn '`.
- la condizione di ribaltamento di ADR-0029 **non è misurabile senza un Linux vero**: la scelta C la sostituisce con una lettura alle fonti e un innesco scritto.
- contare la quaterna dei test con un `awk` sul log del cancello: conta due volte i bersagli del passo 7.

## Prossimo passo, eseguibile

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di
   questa chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md`: il compendio per intero, a blocchi, e la testa dell'audit
   del 2026-08-27.
3. **Questo file, per intero.** Poi le skill: `anthropic-skills:decision-principles`,
   `anthropic-skills:session-resume` su questo file, `anthropic-skills:dev-discipline`,
   `anthropic-skills:dev-communication`, e `superpowers:brainstorming` — percorso architetturale,
   si riprende dalla **§6b**.
4. Rilanciare i comandi della tabella dello stato; rileggere le righe **dedotte** delle sezioni
   approvate contro il codice di adesso, prima di costruirci sopra.
5. Al proprietario i **wireframe** delle due schermate, disegnati in chat; poi la §6b da approvare;
   poi le §7–§10, una per volta, ciascuna col controllo esplicito sui cinque criteri e con
   verificato, dedotto e assunto separati, chiedendo il sì dopo ogni sezione.
6. Alla chiusura: riscrivere questo file sul posto — come disegno se lo scrive lo stesso giorno, o
   come consegna delle sezioni approvate — spostare questo testo parola per parola in
   `docs/archivio/consegna-brainstorming-sottoprogetto-2.md` coi link riscritti, muovere il
   puntatore della §6 del compendio, `bash scripts/check-docs.sh`, `bash scripts/gate.sh`, commit,
   push.

## Come tornare operativi

```bash
git fetch --all --prune && git status -sb && git log --oneline -3
bash scripts/check-docs.sh
bash scripts/gate.sh
```
