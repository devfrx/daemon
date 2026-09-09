# Archivio — la consegna del brainstorming sul sotto-progetto 2, dal 2026-09-06 al 2026-09-09

⛔ **Non è una lettura obbligatoria.** È il **verbale** della consegna con cui il brainstorming del sotto-progetto 2 si è
fermato a metà il **2026-09-06**, poi allargato alla stella polare della GUI il 2026-09-07 e completato sezione per sezione
fino al 2026-09-09, tenuto **parola per parola** com'era a `a539f2a`. Spostato qui il 2026-09-09, quando il disegno è stato
scritto **sul posto** — al percorso che la consegna occupava — con la regola di `CLAUDE.md`: *un documento vivo porta ciò che
è vero adesso, e un verbale va in archivio*. È il viaggio che la consegna stessa prevedeva in testa, e il precedente è
[quella della knowledge base](consegna-brainstorming-knowledge-base.md); la consegna dell'**avvio**, da cui questo
brainstorming è partito, sta in [`consegna-avvio-brainstorming-sottoprogetto-2.md`](consegna-avvio-brainstorming-sottoprogetto-2.md).

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Il disegno vivo è
[`../superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md`](../superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md);
il prossimo passo sta nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md), in un posto solo.

⚠️ **Una sola cosa è cambiata rispetto al testo consegnato:** i percorsi relativi dei collegamenti, riscritti per questa
cartella perché `check-docs.sh` li verifica. Nessuna parola è stata toccata; lo prova `diff` fra questo file, dalla riga sotto
la recinzione in giù, e la consegna a `a539f2a`:
`diff <(git show a539f2a:docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) <(tail -n +22 docs/archivio/consegna-brainstorming-sottoprogetto-2.md)`
— solo righe che contengono `](`.
---

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
[`../../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md`](../archivio/consegna-avvio-brainstorming-sottoprogetto-2.md),
parola per parola. È il viaggio della consegna della knowledge base (`07ab6dc` → `6a7967a`).

⚠️ **Non è una spec e non è ancora il disegno.** Il prossimo passo sta nella §6 del
[compendio](../COMPENDIO.md), in un posto solo.

⚠️ **RICHIAMO DEL 2026-09-07 — il brainstorming è proseguito e si è ALLARGATO, per scelta del
proprietario, alla forma di tutta la GUI.** La consegna di quella sessione è
[`2026-09-07-direzione-gui-design.md`](../superpowers/specs/2026-09-07-direzione-gui-design.md), la **stella polare**, e si
legge **prima** di questo file. Le decisioni prese lì che **ritagliano il 2**, in una riga ciascuna: il 2
costruisce la **cornice** — le viste Home, Lavoro e Compatta, i moduli mobili con `dockview-core`, la Home
col nucleo che a parole dice «niente ancora» — più la sua fetta di oggi; il modulo **Passi** nasce col 2,
con un messaggio IPC per la lista dei passi; l'**archivio della disposizione** — una voce sola, due
messaggi — nasce col 2; la chat **non** sta nella Home di default. I wireframe di Home e Lavoro
sostituiscono la proposta della §6b qui sotto. ⛔ **Le §1 e §6a restano approvate com'erano**, e si
riscrivono col disegno sulla base della stella polare, non prima: ciascuna porta il proprio richiamo.
Il «Prossimo passo» di questo file è **superato** da quello della stella polare.

## Stato in una riga

Brainstorming del 2 a metà, poi allargato alla stella polare il 2026-09-07: undici risposte più la
strada, le sezioni §1–§6a approvate coi richiami datati, nessun codice toccato; mancano le sezioni
scritte della stella polare e le §7–§10 di questo file, poi i due disegni scritti e il piano. ✅ **Richiamo del 2026-09-09:** le **§7 e §8** sono scritte (decisioni 33–35
della stella polare); le §9 e §10 sono **decise** su delega (36 e 37) e si scrivono nella sessione nuova. ✅ **Richiamo della quindicesima ripresa, lo stesso giorno:** le §9 e §10 sono **scritte** — tutte le sezioni §1–§10 sono approvate; mancano i due disegni scritti sul posto, punto 5 del prossimo passo della stella polare.

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
[`riferimenti.md`](../riferimenti.md), e qui resta il rimando — è la regola di `CLAUDE.md` alla
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

⚠️ **RICHIAMO DEL 2026-09-07:** la stella polare aggiunge al perimetro — con richiamo, non con
riscrittura — il motore dei moduli (`dockview-core`, dipendenza nuova, in due passi), le tre viste con
Compatta come segnaposto, il modulo Passi col suo messaggio, e l'archivio della disposizione coi due
messaggi; il pezzo 6 cambia forma. La riscrittura è del disegno: sezione 3 della tabella *«Le sezioni che
mancano»* di [`2026-09-07-direzione-gui-design.md`](../superpowers/specs/2026-09-07-direzione-gui-design.md).

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

⚠️ **RICHIAMO DEL 2026-09-09, undicesima ripresa della stella polare (decisioni 24 e 25):** lo spike del guscio ospita anche lo
**spike di accettazione di `dockview`** — la §4 di
[`2026-09-07-direzione-gui-design.md`](../superpowers/specs/2026-09-07-direzione-gui-design.md). Il frontend minimo porta `dockview-core`; la vista
chat e la scena `three` vivono **dentro due tessere** di una Home finta, così M4 si misura con `dockview` acceso; il protocollo di
`spikes/gui-shell/PROTOCOLLO.md` porta anche le **otto mosse** che il proprietario giudica provandole, l'ottava con la mano di
SP-7 come puntatore; nascono due righe qualitative per guscio, come Q1 e Q2 — **Q3**, la finestra staccata si apre dentro il
guscio; **Q4**, nella webview basta `dndStrategy: 'auto'` o serve `'pointer'` — e i loro esiti entrano in ADR-0029 come fatti.
La riscrittura è del disegno.

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
| la stretta di mano | il primo messaggio della GUI è `Hello` col timbro di build; il core accoglie o rifiuta e chiude; la GUI col timbro sbagliato **non parte e lo dichiara** (§6.1.2). Il timbro nasce dalle fixture, §4. ⚠️ **RICHIAMO DEL 2026-09-08, nona ripresa della stella polare (decisione 22):** «e chiude» non è un'operazione della porta — il tratto `Ipc` ha `accept`, `send` e `receive` e **nessuna chiusura** — quindi il core **segna il client come rifiutato e non lo ascolta più**; la GUI esce da sola, come questa riga già dice, e il core vede `Disconnected`. Trovato disegnando la sequenza dell'accoglienza | una sonda per direzione |
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
| `Approve` | GUI | la tripla concessa dall'utente nella finestra di conferma. ⚠️ **RICHIAMO DEL 2026-09-08, nona ripresa della stella polare (decisione 21): porta anche la funzione e l'argomento dell'invocazione che sblocca** — `permission::grant` scrive una nota e vuole un passo già aperto, e il core apre il passo A solo quando l'invocazione arriva; così `Approve` basta, e la GUI non rimanda `Invoke` | idem |
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
| il giornale | **un'invocazione è un passo suo**: `intent` sul passo A (classe della funzione), poi una **nota** col dettaglio strutturato — funzione, invocatore, argomento — sul precedente di `PermissionDetail`, poi l'effetto, che è il passo B di `set_policy` com'è, poi `outcome` su A. Specie nuova `Invocation`, con la **sua fixture congelata in più**, come fu per `Permission`: i byte vecchi non si toccano. ⚠️ **RICHIAMO DEL 2026-09-08, nona ripresa della stella polare (decisione 21):** dopo un `Approve` la nota `Permission` di `grant` si posa **su A**, fra la nota `Invocation` e l'effetto — il permesso sul passo che sblocca, come `run_the_ring` posa il verdetto sul passo che giudica | frozen bytes: quattro record diventano cinque, i quattro vecchi identici al byte; una sonda legge il dettaglio dopo `replay`. ⚠️ **RICHIAMO DEL 2026-09-08, nona ripresa:** i record congelati sono **sei** (`ls crates/kernel/tests/frozen/`), non quattro — la sesta ripresa lo trovò per la tabella Passi della stella polare — quindi con `Invocation` diventano **sette**, e i sei vecchi restano identici al byte |
| l'invocatore | un enum con una sola variante oggi, il client della GUI col suo `ClientId`; il 12 aggiunge il gesto con un indice nuovo, senza cambio di formato (ADR-0036, regola 3) | — |
| il permesso | `Approve` dalla GUI dopo il sì dell'utente → `permission::grant` → la GUI rimanda `Invoke`. ⚠️ **RICHIAMO DEL 2026-09-08, nona ripresa della stella polare (decisione 21): questa riga non reggeva** — `grant` scrive una **nota** su un passo che qualcun altro ha aperto (il suo doc), e qui nessun passo è aperto quando arriva `Approve`. La forma decisa: `Approve` porta la tripla **e** la funzione con l'argomento; il core apre il passo A, scrive `Invocation`, poi `Permission` con `grant`, poi l'effetto, poi l'esito; **la GUI non rimanda `Invoke`**. Trovato disegnando la sequenza dell'invocazione; scartata la tripla tenuta in memoria dal core fino al prossimo `Invoke` | `permission_triple.rs` già esiste; una sonda sul giro completo |

⚠️ **Limite letto nel codice:** `is_granted` rilegge **tutto** il giornale, quindi una tripla
concessa resta concessa **anche dopo un riavvio**; ADR-0016 dice «per quella sessione», e nel codice
il confine di sessione non esiste. Il 2 non lo costruisce: appartiene a chi porta le run, il 3.

Il daemon che ascolta:

| Pezzo | Forma | La prova |
|---|---|---|
| l'attività | un'attività **del kernel** — così la DST la muove con `DyingGui` — che il daemon lancia sull'esecutore coi porti veri: a ogni giro `accept`, poi `receive` per ogni client, poi dispaccia, poi dorme un **tick** consegnato via `Parameters` (ADR-0034), perché il reattore non ha prontezza I/O | una campagna DST breve: la morte della GUI in un punto scelto dal seme non lascia mai una concessione appesa; un crash del giornale a metà invocazione lascia il passo A in dubbio con la sua classe |
| `Hello` | il primo messaggio deve essere `Hello`; timbro uguale → `Accepted` con la protezione dell'archivio (valore che `platform` conosce, consegnato), poi `Degradation` da `degradation_now` e `Policy` dall'arbitro; diverso → `StaleBuild`, chiusura. ⚠️ **RICHIAMO DEL 2026-09-08, nona ripresa (decisione 22):** «chiusura» si legge *il core non ascolta più quel client* — la porta non ha una chiusura, la GUI esce da sola (§6.1.2) e il core vede `Disconnected`; e dopo `Policy` il core manda anche `Layout` e la lista dei passi (§2 e §3 della stella polare) | una sonda per direzione |
| `Invoke` / `Approve` | al registro; dopo `set_policy` rimanda `Policy` | il giro completo su `FakeGui` |
| `Request` | `admit` → `Verdict`. ⛔ **`promote` non si chiama**: il 2 non costruisce il primo ciclo di orchestrazione, quindi il chiusore di `E50`/`E51`/`E100` non cambia; una richiesta `Queued` resta in coda. Nessuna GUI del 2 manda `Request`: l'innesco è il pilastro 3D | dichiarato, non pinzato (gotcha #73) |
| `Disconnected` | `ClientGrants::on_disconnect(client, arbiter, now)`; il client esce dalla tabella | già provato in `client.rs`; una sonda sul cablaggio |
| il limite di giri | **deciso dal coordinatore su delega, A**: `u64::MAX` in produzione, sul precedente di `FOR_EVER`, richiamo datato sul doc di `EXECUTOR_TURN_LIMIT`; i banchi tengono i loro limiti finiti. Perché non B (contare solo i giri senza attesa): toglierebbe ai test la garanzia che un'attività che dorme su scadenze future per sempre finisca, e servirebbero due limiti | `daemon` guadagna una sonda: il grafo con la GUI resta vivo oltre centomila giri |
| lo spegnimento | nessuno nel 2: si uccide il processo, il giornale write-ahead è fatto per questo (ADR-0007); un arresto pulito coi segnali dell'OS è del 10 | dichiarato |

Debiti dichiarati: il permesso che sopravvive al riavvio (chiusore il 3); `promote` non chiamato
(chiusore invariato); nessuno spegnimento pulito e nessuna guardia contro un'attività che gira a
vuoto in produzione (chiusore il 10, un watchdog dell'OS).

### §6a — La GUI: struttura, strati, regole · approvata il 2026-09-06, a condizione

⚠️ **RICHIAMO DEL 2026-09-07:** «le due schermate» diventano **Home e Lavoro dentro la cornice** di
`dockview-core`; `panels/` ospita i moduli e le tre viste come JSON; la finestra di conferma del permesso
resta; la vista chat non sta nella Home di default (domanda 7 della stella polare). La riscrittura è del
disegno, sulla base di [`2026-09-07-direzione-gui-design.md`](../superpowers/specs/2026-09-07-direzione-gui-design.md).

| Pezzo | Forma |
|---|---|
| dove e con che cosa | `gui/` alla radice, fuori dal workspace Cargo; toolchain web propria: Node LTS con la versione appuntata nel progetto, `vite` 8.2.2, Vue 3.5.42, `pinia` 4.0.3, Reka UI 2.10.4, `vue-i18n` 11.4.10, TypeScript. Il `package-lock.json` si committa: nel cancello gira `npm ci`, gemello di `--locked`, e manifesto e lockfile viaggiano insieme |
| **il ponte** | la SPA **non tocca mai un socket**. Parla con un oggetto ponte piccolo e tipizzato — riceve messaggi già decodificati, ne manda tre: `Hello`, `Invoke`, `Approve`. Ogni guscio lo implementa a modo suo (Electron nel preload, Tauri con eventi e `invoke`): è la cucitura che rende la SPA **indipendente dal guscio**, e che permette di svilupparla e provarla nel browser con un **ponte finto** che rilegge le fixture, prima che il guscio esista. Chi decodifica sta sotto il ponte, e lo decide la misura |
| gli strati, una cartella ciascuno | `transport/` (il ponte e la sua finta) → `schema/` (i tipi TypeScript dei messaggi e le fixture: per ogni variante i byte **e** il valore atteso in JSON, generati dallo stesso comando del kernel) → `stores/` (`pinia`: connessione, stato del core, flusso; **solo presentazione**, I1) → `components/` → `panels/` (le due schermate) → `tokens/` (colori, spazi, caratteri: **un file solo** di variabili CSS) → `locales/it.json` |
| i quattro stati della connessione | **core non in esecuzione** → una fascia che lo dice e un pulsante «riprova», il resto spento (ADR-0019); **timbro sbagliato** → la GUI lo dichiara e non procede; **collegata** → pannello di stato vivo; **nessuna run** → la vista chat lo dice a parole invece di restare vuota: col daemon vero non arriva nessun token fino al 3 |
| il pannello di stato | degrado (due campi oggi), policy attiva con budget allocato su totale, la riga «protetto quanto il tuo account» presa dal valore in `Accepted`, e **una riga di evento** per l'ultimo `Verdict`, che compare **solo quando ne arriva uno**: niente riquadro vuoto, la resa si prova con le fixture |
| il cambio di policy | un controllo a due stati, OpenRouter con VRAM libera oppure locale → `Invoke`; `PermissionRequired` → **finestra di conferma** con la tripla a parole → sì → `Approve` → la GUI rimanda `Invoke`. ⚠️ **RICHIAMO DEL 2026-09-08, nona ripresa (decisione 21):** `Approve` porta anche l'invocazione, e la GUI **non rimanda** `Invoke` — vedi la riga «il permesso» della §5 |
| la vista chat | rende il flusso di `Token` come markdown con blocchi di codice (G4); ogni pezzo porta la **provenienza visibile** (G13); il testo non fidato si rende come **testo e codice, mai come HTML**, e nessun link si apre da solo (ADR-0014 in interfaccia) |
| accessibilità (G20) | tastiera ovunque, dai primitivi di Reka UI; focus visibile; contrasto AA dai token; la regione del flusso annunciata allo screen reader con moderazione; la finestra di conferma con la trappola di focus |
| testi (G21 rimandato) | ogni scritta in `locales/it.json`, italiano solo; **un controllo** che vada rosso su una scritta lasciata nel codice, se esiste una regola di lint matura (si verifica al piano) |
| la regola del kit | un componente si estrae dai pannelli alla **seconda** occorrenza; nel 2 nessun kit |

Debiti dichiarati: la vista chat senza produttore vero fino al 3, e lo dice a parole; due
dipendenze nuove non scelte qui — un renderer di markdown, gli attrezzi di prova — tue, in §9; la
parte del ponte che dipende dal guscio, aperta per nome. 🔶 Dedotto: la forma esatta del ponte nei
due gusci.

### §7 — Il core finto · approvata il 2026-09-09 (A, decisione 33 della stella polare)

Un programma piccolo in `gui/fake-core/` che finge di essere il core, così la GUI si costruisce e si prova
prima che il daemon vero faccia tutto e prima che esista un modello. Parla sul **filo vero** con lo
**schema vero**. Deciso dal proprietario, **A — riusa**: il finto non riscrive il dispaccio, fa girare
l'**attività vera** del kernel che ascolta la GUI (§5, «l'attività») su porte in memoria; finto è soltanto
un **rubinetto**, che produce ciò che nel 2 nessun pezzo vero produce ancora. La riga 7 di «Le sezioni che
mancano» qui sotto era la forma B, e porta il richiamo. Già approvato prima di questa sezione, e qui solo
richiamato: fuori dal workspace Cargo (pezzo 7 della §1), il `Cargo.lock` committato, i token a tempo come
lo spike, `Invoke` → `PermissionRequired` la prima volta, `Layout`/`SaveLayout` e la lista dei passi (§3
della stella polare, pezzo 8).

| Pezzo | Forma | La prova che lo esercita |
|---|---|---|
| dove, e come si costruisce | un binario Rust in `gui/fake-core/` col proprio `Cargo.toml`, **fuori dal workspace** — il manifesto di radice aggiunge `gui` a `exclude` — che dipende **per percorso** da `kernel`, `platform` e `simulator`; il suo `Cargo.lock` **si committa**, perché lo usa il cancello: è un attrezzo, non uno spike | il passo del cancello della §8: `cargo test --locked --manifest-path gui/fake-core/Cargo.toml`, verde |
| l'attività vera | la **stessa** attività del kernel che il daemon lancia sull'esecutore (§5, «l'attività»), costruita **da fuori** con le porte e i parametri consegnati (ADR-0034) — ciò che la campagna DST della §5 pretende già — e fatta girare da `Executor` col limite di giri del daemon; nel finto **non vive nessun ramo del dispaccio** | una sonda **da fuori la crate**, come `ports_are_implementable.rs`: l'attività costruita sulle porte in memoria fa il giro `Hello` → `Accepted` → `Degradation` → `Policy` → `Layout` → la lista dei passi, nell'ordine della sequenza 1 della stella polare |
| le porte in memoria | il giornale è `MemoryJournal` del simulatore; la settima porta è la finta del simulatore (pezzo 5 della §3 della stella polare); l'arbitro è **vero**, con le due quote permanenti di ADR-0033 e la policy di default, costruito **come lo costruisce il daemon** — oggi `build_the_arbiter` in `crates/daemon/src/main.rs`, vedi il dedotto; il reattore è `SystemReactor` di `platform`, come nel daemon, perché i token vanno a tempo di orologio e la GUI è un processo vero | nessuna propria: sono implementazioni già provate dalle suite di conformità; il finto le cabla, e lo prova la sonda del pezzo sopra |
| il trasporto vero | quello di `platform` (§3), sullo **stesso nome** del daemon, così la GUI non sa con chi parla; il timbro di build è quello delle fixture (§4), calcolato dallo stesso codice | la sonda del core finto contro il trasporto vero, **da un thread** (§8): un pari finto sul thread manda `Hello` col timbro giusto e riceve la fila della sequenza 1; col timbro sbagliato riceve `StaleBuild` e poi più niente |
| il rubinetto | l'unica parte finta: una **seconda attività** sullo stesso esecutore, che condivide con la prima il trasporto e la tabella dei client accolti attraverso una `RefCell`, come le attività dei banchi (`executor_determinism.rs`). Fa due cose. Manda `Token` a tempo, come lo spike — 2000 in dieci secondi, testo **non fidato**, markdown con blocchi di codice — con la cadenza consegnata alla costruzione. E legge **una parola da stdin**: `degrade` scrive nel giornale in memoria un passo di comodo con una nota `Detail::Routing` degradata — la stessa che `degradation_now` rilegge — e il resto lo fa il codice vero, che manda `Degradation`; `verdict` chiede all'arbitro vero un'ammissione con una richiesta di comodo e manda alla GUI il `Verdict` che ne esce. Niente altro: una parola nuova si aggiunge quando un modulo la chiede | una sonda per parola, con la cadenza rapida: `degrade` → `degradation_now` sul giornale in memoria risponde `routing_degraded` e il pari riceve `Degradation`; `verdict` → il pari riceve un `Verdict`; e la sonda dei token: il pari li conta e legge la provenienza non fidata su ognuno |
| la disposizione | `SaveLayout` va nella finta in memoria della settima porta e `Layout` torna, come nella sequenza 2; **vive finché il finto non riparte** — la persistenza vera è del daemon, provata dalla sonda «salva, riavvia, ritrova» della §8 | il giro salva → ritrova sul pari, dentro la sonda dell'attività; il riavvio no, dichiarato |
| la lista dei passi | quella vera: `replay` sul giornale in memoria, che porta le invocazioni fatte dalla GUI in questa corsa del finto — il modulo Passi mostra passi veri, non un elenco scritto a mano | la sonda dell'invocazione: un `Invoke` dal pari, poi la lista che torna porta l'invocazione |

**Perché A e non B** (B: un copione a sé che parla lo schema e il filo e scrive a mano ogni risposta del
daemon, la riga 7 com'era). I tre controlli della decisione 18, riletti nel codice il 2026-09-09. *Esiste:*
l'emettitore di `spikes/gui-ipc/src/bin/core.rs` — righe JSON, sopravvive alla GUI che muore; `IpcMessage`
con due varianti e le sonde di `crates/kernel/tests/ipc_wire.rs`; `FakeGui` in `ports_are_implementable.rs`
e `DyingGui` nel simulatore; il dispaccio di `gui_death_campaign.rs` scritto **dentro il banco**, quindi
nessuna attività «servi la GUI» esiste ancora nel kernel: la costruisce il pezzo 6 della §3 della stella
polare; nessun trasporto in `platform`; `gui/` non esiste; il manifesto di radice esclude solo `spikes`.
*Arriva:* col 3 il core vero produce i token e il rubinetto perde quel compito, ma resta per ogni modulo il
cui produttore arriva dopo; col 12 il gesto entra come invocatore nella stessa attività. *Regge crescendo:*
una variante nuova è un ramo del `match` nell'attività vera, e con A il finto la segue gratis; con B è una
risposta in più scritta a mano. Il modo del repo è la logica vera su porte sostituite — è il simulatore
(ADR-0021, §3.1 della spec) — e B sarebbe una **seconda copia del dispaccio**: il giorno che il daemon cambia,
il finto diverge senza che nulla diventi rosso.

**Il costo di A, dichiarato:** il finto dipende anche da `simulator` per percorso; l'attività del kernel deve
potersi costruire da fuori con porte e parametri consegnati — un vincolo che la campagna DST della §5 impone
già, quindi il 2 lo paga una volta sola; il rubinetto condivide trasporto e tabella dei client con l'attività
attraverso una `RefCell`; la disposizione vive finché il finto non riparte.

**Ciò che la §7 non fa:** non persiste nulla oltre la corsa — disposizione, permessi concessi, passi: tutto
nel giornale in memoria; non parla con un modello; non prova il daemon, la cui sonda è quella della §5; e
**non convive col daemon**: uno solo in ascolto per volta, e chi lo avvia lo sa — che cosa faccia il sistema
operativo con due in ascolto sullo stesso nome non è del finto: dichiarato, non pinzato.

Debiti dichiarati: il rubinetto è codice finto fuori dal prodotto, e il suo «degrada» è un meccanismo vero
con una causa finta; i `Token` senza produttore di prodotto fino al 3 (già in §4); perché il `degrade` arrivi
alla GUI, l'attività vera deve **accorgersi** che `degradation_now` è cambiato e mandare `Degradation` — è
la riga «rimanda il pezzo che è cambiato» della §5, e il finto è il primo che la esercita: *come* se ne
accorga, a ogni giro o dopo ogni scrittura, lo fissa il piano del 2 con la sonda della §8, con richiamo alla
§5 se serve. 🔶 **Dedotto**, da confermare da chi costruisce: che un'attività e il rubinetto possano
condividere il trasporto attraverso una `RefCell` — `Executor::spawn` prende future con vita `'a`, e
`executor_determinism.rs` fa già girare più attività su stato condiviso in una `RefCell`, letto il
2026-09-09; che il valore di `Accepted`, la protezione dell'archivio, sia consegnato al finto come al daemon;
la forma esatta di ciò che sta nella `RefCell`; che la costruzione dell'arbitro con le due quote e i
parametri — oggi `build_the_arbiter`, `reserve` e i letterali in `crates/daemon/src/main.rs`, un **binario**,
che il finto non può importare — diventi raggiungibile dal finto **senza copiarla**: dove spostarla lo decide
il piano, e l'attività non ha il problema perché vive nel kernel (pezzo 6 della §3 della stella polare).
**Assunto:** niente.

Controllo sui cinque criteri, il 2026-09-09: **verificato** nel codice — `gui/` non esiste,
`exclude = ["spikes"]` nel manifesto di radice, nessun `impl Ipc for` fuori dai commenti in `crates/`,
nessuna attività che serva la GUI nel kernel, `MemoryJournal` e `DyingGui` nel simulatore, `SystemReactor`
in `platform`, `degradation_now` che rilegge `Detail::Routing`, `build_the_arbiter` in un binario;
**coerenza** — la logica vera su porte sostituite è il modo del repo, e niente si scrive a mano di ciò che il
daemon già sa fare; **debito** — scritto sopra; **stato dell'arte** — nessuna versione scelta qui;
**proporzione** — un rubinetto con due parole e i token, e una parola nuova solo quando un modulo la chiede.

### §8 — Le prove e il cancello · approvata il 2026-09-09 (delegata, «decidi secondo la skill»: A, decisione 34 della stella polare; l'archivio che non si apre: A, decisione 35)

Il 2 costruisce pezzi in due mondi, Rust e web, e il cancello di oggi — `bash scripts/gate.sh`, sei passi più il
settimo che stampa il tempo delle campagne — compila e prova solo il workspace Rust. Questa sezione dice tre cose:
quale prova esercita **ogni** pezzo nuovo, una riga per artefatto, presa dalle colonne «prova» delle sezioni
approvate; come il cancello impara il mondo web — un passo nuovo, `scripts/gate-gui.sh`, chiamato da `gate.sh` con
la stessa riga `run` degli altri, così il cancello resta **uno** (risposta 6, vincolo globale 7); e come la CI
installa Node e che cosa git ignora. Il proprietario ha delegato la sezione e la domanda sull'archivio che non si
apre con «decidi secondo la skill»: le decisioni sono qui, col perché, e restano ribaltabili.

**Il cancello, pezzo per pezzo.**

| Pezzo | Forma | La prova |
|---|---|---|
| `scripts/gate-gui.sh` | `cd` alla radice come `gate.sh`; in ordine: `cargo test --locked --manifest-path gui/fake-core/Cargo.toml`, poi dentro `gui/`: `npm ci`, `npm run build`, `npm test`; si ferma al primo rosso | nelle due direzioni, a mano al piano come per le campagne: un test della SPA reso rosso → `GATE RED`; l'etichetta del passo compare nell'uscita del cancello |
| la riga in `gate.sh` | `run "gui: fake core and SPA" bash scripts/gate-gui.sh`, dopo «attributes of the constrained crates» e prima di «documentation consistency»; etichetta in inglese come le altre | la riga sopra |
| il core finto nel cancello | il suo `Cargo.lock` è committato, quindi `--locked` regge; le sonde vivono in `gui/fake-core/src/main.rs` come nel daemon (decisione 48 del coordinatore). Costo: il finto ricompila `kernel`, `platform` e `simulator` nel proprio `target/`, tempo dichiarato e misurato al piano | il primo passo dello script |
| la versione di Node | in `gui/package.json`, campo `engines.node`, **casa unica**; `gui/.npmrc` con `engine-strict=true`, così un Node sbagliato fa rosso a `npm ci` con la ragione scritta — il gemello del vincolo 4 di §11 del compendio: il prerequisito dell'ambiente si dichiara, o la porta è rossa per il motivo sbagliato (decisione 46) | al piano, nelle due direzioni: con un Node fuori intervallo `npm ci` deve fermarsi |
| la CI | `.github/workflows/quality-gate.yml` guadagna, prima di `bash scripts/gate.sh`, un passo `actions/setup-node` con `node-version-file: gui/package.json`; la versione dell'azione si legge alla fonte il giorno del piano (letta il 2026-09-09: la pagina consiglia `v7`); `checkout` resta com'è. Niente cache npm oggi: una riga, `cache: npm`, che si aggiunge quando la CI misura che serve (decisione 47) | il cancello gira in CI com'è |
| `.gitignore` | `/gui/node_modules/`, `/gui/dist/`, `/gui/fake-core/target/`; per `spikes/gui-shell/`: `node_modules/`, `dist/` e le cartelle di build dei due gusci, coi nomi al piano quando esistono. I lockfile **si committano**: `gui/package-lock.json`, `gui/fake-core/Cargo.lock`, e quelli dello spike, npm e Cargo (§4 della stella polare; decisione 49) | dopo `bash scripts/gate.sh`, `git status --porcelain` vuoto |
| la campagna DST del 2 nel settimo passo | una riga `cargo test --locked -p simulator --test <nome> -- --nocapture` nello stesso commit del banco: il settimo passo nomina i bersagli **uno per uno**, e una campagna assente è silenziosa — scattato due volte, lo dice il commento di `gate.sh` | nelle due direzioni: `grep -c 'DST <nome>'` sull'uscita del cancello, zero senza la riga e più di zero con |

**Il prodotto del 2, e il controllo che esercita ciascun artefatto.** La forma dei disegni dei gesti e della
knowledge base; le righe vengono dalle colonne «prova» delle §3–§7 e delle §2–§4 della stella polare, e qui stanno
in un posto solo perché il piano le tagli per compito.

| Artefatto | Il controllo che lo esercita | Specie |
|---|---|---|
| il trasporto `ipc` in `platform` (§3) | la suite di conformità `crates/kernel/tests/ipc_contract.rs`, inclusa con `include!` da `crates/platform/tests/ipc_contract_real.rs`, su `FakeGui`, `DyingGui` e il trasporto vero con un pari su un thread; un bugiardo per promessa, nelle due direzioni; nessuno collegato → `None` | cancello, `cargo test --locked --workspace` |
| il contatore condiviso (§3) | la sonda del riavvio: riaperto il giornale, il primo numero sta sopra l'ultimo scritto; due `accept` danno numeri diversi e crescenti | cancello |
| la stretta di mano (§3, decisione 22) | una sonda per direzione: timbro giusto → `Accepted`; timbro sbagliato → `StaleBuild`, il client non è più ascoltato, e quando la GUI esce il core vede `Disconnected` | cancello |
| lo schema (§4, più `Layout`, `SaveLayout` e la lista dei passi) | le fixture committate in `gui/`, ricodificate da `crates/kernel/tests/ipc_wire.rs`: schema cambiato senza rigenerare → rosso; il timbro cambia coi byte; `decode` verifica i byte consumati | cancello |
| il registro (§5) | nome non registrato → rifiutato, nessun record; tripla non concessa → `PermissionRequired` e `set_policy` mai chiamato; concessa → l'effetto; `Approve` con l'invocazione → il giro completo su `FakeGui` (decisione 21); i byte congelati: un record in più per `Invocation`, i vecchi identici al byte — quanti lo dice `ls crates/kernel/tests/frozen/`; una sonda legge il dettaglio dopo `replay` | cancello, `frozen_bytes.rs` |
| l'attività del daemon (§5) | la **campagna DST del 2** in `crates/simulator/tests/`: la morte della GUI in un punto scelto dal seme non lascia concessioni appese; un crash del giornale a metà invocazione lascia il passo A in dubbio con la sua classe; e la riga nel settimo passo, tabella sopra | cancello, due volte per costruzione |
| il limite di giri e `Disconnected` (§5) | `daemon`: il grafo con la GUI resta vivo oltre centomila giri; la GUI che muore con una concessione ordinaria → `on_disconnect`, già provato in `client.rs`, più una sonda sul cablaggio | cancello |
| la settima porta (§2 della stella polare) | la finta in `ports_are_implementable.rs`; la suite di conformità sulle due implementazioni coi bugiardi, come `journal_contract`; `redb` in `platform`: apri, scrivi, riapri, rileggi; byte che non sono JSON tornano identici; `SaveLayout` su un archivio che rifiuta la scrittura → `Layout` col vecchio; archivio vuoto → «niente»; **archivio che non si apre → il core parte e `Layout` dice «non disponibile»**, e ogni `SaveLayout` riceve lo stesso (decisione 35, sotto) | cancello |
| «salva, riavvia, ritrova» | sul daemon vero con l'archivio su un file temporaneo, come le sonde di `crates/daemon/src/main.rs`: `SaveLayout`, il grafo si ferma e riparte, `Hello`, `Layout` col pacchetto — la sequenza 2 della stella polare | cancello |
| il core finto (§7) | le sonde in `gui/fake-core/src/main.rs`: il giro della sequenza 1 su porte in memoria; il trasporto vero da un thread, timbro giusto e sbagliato; una sonda per parola del rubinetto; i `Token` contati, con la provenienza non fidata su ognuno | `gate-gui.sh`, primo passo |
| la SPA, `schema/` (§6a, risposta 10) | unit sulle fixture: ogni variante decodificata dai byte e confrontata col valore atteso in JSON; una variante senza fixture → rosso | `npm test` |
| la SPA, `stores/`, componenti e pannelli (§6a; §2 e §3 della stella polare) | unit col ponte finto che rilegge le fixture: i quattro stati della connessione; il modulo Stato coi due campi del degrado, la policy col budget, la riga G16 dal valore in `Accepted`, la riga di evento solo quando arriva un `Verdict`; la finestra di conferma con la trappola di focus; la vista chat che rende testo e codice, mai HTML, con la provenienza su ogni pezzo; la cornice: le tre viste JSON si caricano, un pacchetto con un tipo sparito lo dice a parole e si chiude (riga 8 della §2 della stella polare), un modulo non costruito dice chi lo riempie; la verifica d'accessibilità sui componenti, con l'attrezzo scelto in §9 | `npm test` |
| le scritte, `locales/it.json` (G21) | un controllo che vada rosso su una scritta lasciata nel codice, **se** al piano esiste una regola di lint matura (§6a); altrimenti revisione, e lo si dice | `npm test`, o revisione dichiarata |
| la build della SPA | `npm run build` verde: il compilatore TypeScript è il livello 1 del mondo web, come `rustc` per il kernel; `npm ci` è il gemello di `--locked`: manifesto e lockfile divergenti → rosso | `gate-gui.sh` |
| capo a capo: la SPA nel guscio col core finto | **dopo lo spike**, parte 2 del piano: la prova del ponte in Node o in Rust secondo il vincitore, e le prove capo a capo con l'attrezzo scelto in §9 | fuori dal cancello di oggi, dichiarato |
| lo spike del guscio con l'accettazione di `dockview` (§2; §4 della stella polare) | i criteri congelati in `spikes/gui-shell/PROTOCOLLO.md` al primo commit di codice; le otto mosse giudicate dal proprietario con le sue parole in `spikes/RISULTATI.md`, la mossa 7 come due JSON uguali; M1–M5 e Q1–Q4 in ADR-0029; fuori dal cancello, `spikes` in `exclude` | criterio scritto prima |
| le dipendenze nuove: `interprocess` in `platform`; `dockview-core`, Vue, `pinia`, Reka UI, `vue-i18n` e gli attrezzi di §9 in `gui/` | Rust: in due passi, `--locked` su ogni `cargo` del cancello, `gate-deps.sh` sui grafi di `kernel` e `simulator`, che il finto non tocca — dipende da loro, non il contrario; web: `npm ci` sul `package-lock.json` committato | cancello |
| il registro della porta, `docs/porta-di-qualita.md` | una sezione nuova per il passo web e le sonde del 2, sul precedente della sonda S3; **nessuna riga di catalogo**: la §7.4 è spec, vincolo globale 7, quindi le sonde si registrano e non si prendono | revisione, compito del piano |
| i documenti: la §3.1 della spec e `ports/mod.rs` sei → sette, la roadmap, `README.md`, la §12 del compendio, tracciabilità, i richiami datati | `check-docs.sh`: link, tetto, conteggi ADR; le cifre in prosa nominate nella §3 della stella polare si toccano nel piano, col `grep` sulla frase; il resto è revisione | livello 2 sui link, revisione sul resto |
| il codice fuori dal perimetro | `git diff --stat` a fine piano tocca solo ciò che le tabelle nominano; `ports/mod.rs` e la spec solo coi richiami datati | comando |

**L'archivio che non si apre — decisione 35, delegata: A.** All'avvio il core apre l'archivio della disposizione
dalla settima porta. Se non si apre — file rotto, permessi — il core **parte lo stesso** e lo dichiara: `Layout`
porta un terzo stato, «non disponibile», oltre al pacchetto e a «niente»; la GUI usa le tre viste di default e
ogni `SaveLayout` riceve `Layout` «non disponibile», così il salvataggio fallito si vede senza una variante sua,
come per la scrittura fallita (decisione 13 del coordinatore). Il richiamo datato sta sulla riga 5 della §2 della
stella polare. *Esiste:* il daemon si ferma con `StartupError` quando il giornale non si apre, e la sonda
`a_journal_that_cannot_be_opened_stops_the_start_up` lo prova; `Degradation` ha due campi e `degradation_now` li
ricava dal giornale; la riga 5 della §2 dice «il pacchetto o niente», e le due operazioni della porta possono già
fallire (riga 5: la scrittura rifiutata). *Arriva:* col 10 lo spegnimento pulito e il watchdog; nessun altro
pacchetto all'orizzonte (§2 della stella polare). *Regge crescendo:* un terzo stato è una variante in più dell'enum
di `Layout`, additiva; un archivio mai aperto fallisce entrambe le operazioni, e l'attività traduce la lettura
fallita in «non disponibile»: **nessuna operazione nuova nella porta**. Perché non un campo di `Degradation`
(decisione 50): `Degradation` è una proiezione del giornale (ADR-0019, `degradation_now`), e un archivio cosmetico
che non si apre non merita un record per sempre (ADR-0018); il messaggio che porta la disposizione dice da sé il
proprio stato, «il core manda il pezzo che cambia». Scartata B, fermarsi come per il giornale: la disposizione non
è stato autorevole (I1), e ADR-0019 dice che si dichiara prima, non si fallisce dopo. La sonda: archivio che non si
apre → il grafo parte, `Layout` «non disponibile», `SaveLayout` → `Layout` «non disponibile».

**Perché un cancello unico e non una CI web a parte — decisione 34, delegata: A.** B era un lavoro di CI a parte
che gira solo quando cambia `gui/`: più veloce, ma due verdetti, e un cambio al kernel che rompe le fixture non
farebbe girare le prove della SPA che le leggono; e `CLAUDE.md` dice che la porta si lancia con **un comando solo**.
Costo di A: il passo web gira anche per un commit di sola documentazione, come già i passi Rust.

**Ciò che la §8 non fa:** la CI resta solo Linux (X-1 dell'audit, del proprietario) — e ora pesa di più, perché
il trasporto in `platform` è OS-specifico e la CI prova solo la metà Unix: la metà Windows la prova il cancello
sulla macchina del proprietario; la prova capo a capo nel guscio, parte 2; il lint delle scritte, se immaturo; la
scansione degli avvisi di sicurezza (X-3, del proprietario), che con npm ha un secondo mondo; il tempo del cancello
col passo web, che si misura al piano e si scrive nel commento di `gate.sh` con la data, come per il settimo passo.

**Decisioni del coordinatore in questa sezione**, nella tabella omonima della stella polare: 46, la versione di
Node in `package.json` con `engine-strict`; 47, niente cache npm; 48, le sonde del finto in `main.rs` e la
ricompilazione dichiarata; 49, i lockfile dello spike committati, npm e Cargo, contro `/spikes/rust/Cargo.lock`
ignorato — i numeri dello spike entrano in ADR-0029, e un lockfile ignorato basta a uno spike che si rifà con un
seme; 50, il terzo stato in `Layout` e non in `Degradation`.

Debiti dichiarati: la metà Windows del filo senza CI; il capo a capo dopo lo spike; il lint delle scritte forse
assente; la doppia compilazione delle tre crate nel finto; `porta-di-qualita.md` che riceve le sonde senza righe di
catalogo. 🔶 **Dedotto**, da confermare al piano: che `npm ci` onori `engine-strict` — la pagina di npm letta il
2026-09-09 descrive l'opzione per l'installazione e non nomina `ci`: si prova nelle due direzioni; che le prove
della SPA girino senza browser in CI, con l'ambiente scelto in §9; che `--manifest-path` compili nel `target/` del
finto e non riusi quello del workspace. **Assunto:** niente.

Controllo sui cinque criteri, il 2026-09-09: **verificato** — `scripts/gate.sh` (la riga `run`, `--locked` su
ogni `cargo`, il settimo passo coi bersagli per nome e la lezione scritta), `.github/workflows/quality-gate.yml`
(`ubuntu-latest`, `rustup show`, nessun Node), `.gitignore` (i `target/`, i `Cargo.lock` degli spike, `node_modules`
e `dist` di `spikes/ts/`), «Cosa la porta NON controlla» e la sezione della sonda S3 in `porta-di-qualita.md`,
`journal_contract.rs` e `reactor_contract.rs` in `crates/kernel/tests/`, le sonde del daemon in `main.rs`, la pagina
di `actions/setup-node` (legge `package.json`, consiglia `v7`) e quella di npm (`engine-strict`, default falso);
**coerenza** — stessa riga `run`, `npm ci` gemello di `--locked`, sonde nel binario come nel daemon, campagna per
nome, protocollo congelato come SP-7, sonde registrate e non prese come S3; **debito** — scritto sopra; **stato
dell'arte** — versioni di Node, dell'azione e degli attrezzi al piano, alla fonte; **proporzione** — uno script,
una riga, un passo di CI, tre righe di ignore; niente matrice, niente cache, niente secondo workflow.

### §9 — Le decisioni aperte col chiusore · approvata il 2026-09-09 (delegata, «decidi secondo la skill»: A, decisione 36 della stella polare)

Una tabella: ogni voce aperta, chi la chiude, e un consiglio scritto — così il piano procede anche se il proprietario non
dice altro. Il precedente è «Le voci che questo disegno apre per il proprietario» del
[disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md). Le voci vengono dalla riga 9 di «Le sezioni che
mancano» qui sotto, dai debiti dichiarati delle §3–§8, dalle registrate della stella polare che dicono «il disegno del 2»,
e dalla §8. Delegata dal proprietario alla chiusura della quattordicesima ripresa della stella polare e **scritta alla
quindicesima**, lo stesso giorno: **A**, coi consigli dentro, verificati alla fonte; B era lasciare gli attrezzi web aperti
fino al piano, e avrebbe spostato la stessa verifica di un giro. I numeri delle decisioni sono quelli delle due tabelle della
stella polare, del proprietario (fino alla 38) e del coordinatore (dalla 46 in su). Gli attrezzi web sono **misurati** al
registro npm il 2026-09-09 col comando qui sotto, **due volte** nello stesso giorno — alla proposta e alla scrittura — con lo
stesso esito; il piano lo rilancia e scrive le versioni del **suo** giorno, non queste.

| # | Voce | Chi la chiude | Il consiglio, o la decisione presa |
|---|---|---|---|
| 1 | il renderer di markdown della chat (§6a) | il piano del 2, compito della SPA | **`markdown-it`** — decisione 51 del coordinatore: preset `default`, `html: false` e `linkify: false`, `validateLink` che rifiuta `javascript:` e simili, letto dentro il pacchetto; i link si rendono senza navigare da soli (§6a), con una regola del renderer nostra; scartati `marked` (vuole `dompurify`) e `micromark` |
| 2 | gli attrezzi di prova della GUI (§6a, §8) | il piano | `vitest` 4.1.11 e non la 5 (decisione 52); `jsdom` (53); `@vue/test-utils` 2.5.0; `axe-core` diretto, `vitest-axe` no (54); il capo a capo dopo il guscio con `@playwright/test` e `@axe-core/playwright` (54); `vue-tsc` 3.3.11 per i tipi nella build |
| 3 | la regola di lint per le scritte (§6a, §8) | il piano: `npm run lint` in `gate-gui.sh`, richiamo alla riga delle scritte della §8 | **sì**: `@intlify/eslint-plugin-vue-i18n`, regola `no-raw-text` (decisione 55) |
| 4 | dove va la crate Rust del guscio se vince Tauri (§2) | la parte 2 del piano, dopo M1–M5 | `gui/shell/`, fuori dal workspace come il finto (decisione 57) |
| 5 | la prontezza I/O del reattore: il filo si interroga a ogni giro col tick (§3, §5) | il **proprietario**, registrata; probabilmente il 3, con la rete | resta com'è nel 2; il piano riporta la CPU a riposo del daemon col tick, **senza soglia**, così il 3 decide con un numero |
| 6 | l'allocatore dentro la porta `journal` — la B della §3 | il proprietario, registrata | resta A finché un secondo consumatore non lo chiede |
| 7 | il confine di sessione dei permessi: `is_granted` rilegge tutto il giornale, ADR-0016 dice «per quella sessione» (§5) | il **3**, con le run | nessuna azione nel 2; la §5 lo dichiara |
| 8 | il watchdog e lo spegnimento pulito (§5) | il **10** | nessuna azione nel 2: si uccide il processo, ADR-0007 |
| 9 | come il daemon rilegge la policy all'avvio (decisione 17 del proprietario; registrata alla quarta ripresa della stella polare) | **questa sezione**, poi il piano | il dettaglio tipizzato scritto da `set_policy` sul passo B (decisione 56); un record congelato in più |
| 10 | il commento falso in `crates/platform/src/journal.rs` (registrata all'ottava ripresa della stella polare) | il piano del 2, il primo compito che tocca il file | si corregge lì, decisione 18 del proprietario |
| 11 | AUD-004 — l'ADR del proprietario sulle skill | il **proprietario**, in parallelo al 2, prima del brainstorming del 13 | com'era deciso il 2026-09-04: non sbarra il 2 |
| 12 | il ledger `.superpowers/sdd/` sull'altra macchina | il proprietario, un comando | si lascia: è ignorato da git e non raggiunge nessuno; si toglie quando il piano del 2 apre il suo |
| 13 | la metà Windows della CI, X-1, e la scansione degli avvisi, X-3, con `npm audit` come secondo mondo (§8) | il **proprietario**: passi di CI, vincolo globale 7 | pesano di più col trasporto OS-specifico: si portano come domanda A/B **quando si scrive il piano**; consiglio sì a entrambi |
| 14 | la cache npm in CI (§8) | il piano, quando misura | no, decisione 47 |
| 15 | il titolo della riga 2 della roadmap e le cifre in prosa di `ports/mod.rs` (§3 della stella polare) | il piano, decisione 9 | — |
| 16 | i nomi provvisori dei messaggi e della lista dei passi (§4; §2 e §3 della stella polare) | il disegno del 2 scritto sul posto, nomi inglesi | — |
| 17 | Compatta: popout di `dockview` o finestra rimpicciolita | il **10** | — |
| 18 | se il preset «auto-approva sicuri» approvi le letture ovunque o solo dentro l'ambito (decisione 13 del proprietario) | il **4**, sentito il 5 | — |

**Il comando che rifà la misura** — registro npm e download dell'ultima settimana. Il 2026-09-09 ha reso: `markdown-it`
15.0.1 del 2026-08-27; `marked` 18.0.12 del 2026-09-07; `micromark` 4.0.2 del 2025-02-27; `dompurify` 3.4.15 del 2026-09-06;
`vitest` 5.0.0 del 2026-09-03, col tag `V4` a 4.1.11 del 2026-08-18; `@vue/test-utils` 2.5.0 del 2026-08-27; `jsdom` 30.0.1 del
2026-07-29; `happy-dom` 20.14.0 del 2026-09-03; `@playwright/test` 1.63.0 del 2026-09-04; `axe-core` 4.13.0 del 2026-08-05;
`vitest-axe` 0.1.0 del 2022-10-21; `@axe-core/playwright` 4.13.0 del 2026-08-11; `eslint` 10.10.0 del 2026-09-04;
`eslint-plugin-vue` 10.11.0 del 2026-09-06; `@intlify/eslint-plugin-vue-i18n` 4.5.1 del 2026-06-02; `vue-tsc` 3.3.11 del
2026-08-21. Licenze MIT, tranne Playwright Apache-2.0, `axe-core` e `@axe-core/playwright` MPL-2.0, `dompurify` MPL-2.0 o
Apache-2.0. Il tag `V4` di `vitest` si legge dai `dist-tags` del registro. I download che hanno pesato nelle decisioni 51–55
stanno in quelle righe della tabella del coordinatore della stella polare, con la data, in una casa sola.

```
python - <<'EOF'
import json, urllib.request, urllib.parse
def npm(p):
    d = json.load(urllib.request.urlopen("https://registry.npmjs.org/" + urllib.parse.quote(p, safe="@")))
    v = d["dist-tags"]["latest"]; return v, d["time"][v][:10], d["versions"][v].get("license"), d["dist-tags"].get("V4")
def downloads(p):
    return json.load(urllib.request.urlopen("https://api.npmjs.org/downloads/point/last-week/" + urllib.parse.quote(p, safe="@")))["downloads"]
for p in ["markdown-it", "marked", "micromark", "dompurify", "vitest", "@vue/test-utils", "jsdom", "happy-dom",
          "@playwright/test", "axe-core", "vitest-axe", "@axe-core/playwright", "eslint", "eslint-plugin-vue",
          "@intlify/eslint-plugin-vue-i18n", "vue-tsc"]:
    print(p, *npm(p), downloads(p))
EOF
```

Le opzioni di `markdown-it` si leggono **dentro il pacchetto**, perché il sito e i sorgenti su GitHub non hanno risposto il
2026-09-09: scaricato `markdown-it-15.0.1.tgz` dal registro (`dist.tarball` nei metadati), in `dist/markdown-it.mjs` i tre
preset — `default` e `zero` con `html: false`, `commonmark` con `html: true` — tutti con `linkify: false`, e
`BAD_PROTO_RE = /^(vbscript|javascript|file|data):/` con l'eccezione delle immagini `data:image/(gif|png|jpeg|webp)`. La regola
`no-raw-text`: la pagina `eslint-plugin-vue-i18n.intlify.dev/rules/no-raw-text.html`, letta lo stesso giorno, la descrive come
«disallow to string literal in template or JSX», nel preset `recommended`.

**I tre controlli (decisione 18).** *Esiste:* la riga 9 di «Le sezioni che mancano»; i debiti dichiarati delle §3–§8; le
registrate della stella polare; le fonti di «Lo stato dell'arte verificato» qui sopra (registro npm, il 2026-09-06) e le misure
di questa sezione; il precedente della knowledge base per la forma. *Arriva:* il 3 chiude le voci 5 e 7, il 10 la 8 e la 17, il
4 la 18; il 13 aspetta AUD-004. *Regge crescendo:* una voce nuova è una riga con un chiusore; nessuna voce resta senza.

**Ciò che la §9 non fa:** non prende le decisioni del proprietario — le voci 5, 6, 11 e 13 restano sue, con un consiglio — e non
fissa le versioni del piano, che si rimisurano il giorno in cui si scrive.

**Controllo sui cinque criteri.** Verificato il 2026-09-09, due volte: le versioni e le date al registro, la regola alla sua
pagina, le opzioni nel pacchetto. Coerenza: la forma della knowledge base; le versioni si rimisurano il giorno del piano. Debito:
nessuno nuovo; le voci 5, 6, 11 e 13 restano del proprietario con un consiglio. Stato dell'arte: alle fonti, con la data.
Proporzione: nessuna scelta oltre ciò che il piano deve sapere. 🔶 **Dedotto:** che data e download del registro misurino la
manutenzione, non la qualità; che `axe-core` si chiami sul DOM di `jsdom` senza un adattatore pubblicato. **Assunto:** niente.

**Decisioni del coordinatore in questa sezione**, nella tabella omonima della stella polare: 51–57.

### §10 — Come si riprende · approvata il 2026-09-09 (delegata, «decidi secondo la skill»: A, decisione 37 della stella polare)

Quando il disegno del 2 sarà scritto sul posto (punto 5 del prossimo passo della stella polare), chiude con una sezione
«Come si riprende», nella forma dei disegni dei [gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) e della
[knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md): è il documento di consegna della sessione che scrive i due disegni, e
sta nel disegno del 2 perché da lì parte il piano. **Una sola** — A, decisione 37; B era una in ciascuno dei due file: la
stella polare tiene la propria tabella dello stato e rimanda alla §6 del compendio, in un posto solo (gotcha #68). Questa
sezione dice che cosa conterrà, così chi la scrive non decide da capo.

| Parte | Che cosa porta | Da dove viene la forma |
|---|---|---|
| 1 | la tabella dello stato alla chiusura coi comandi che lo rifanno — ramo, i commit della sessione, codice non toccato, cancello, fine-riga, file temporanei, debito lasciato | la tabella «Stato alla chiusura, e il comando che lo rifà» del disegno della knowledge base |
| 2 | il compito della sessione successiva, in ordine e con ogni riga eseguibile — fetch, la lettura obbligatoria, le voci aperte da sapere prima (la colonna «Chi la chiude» di `porta-di-qualita.md`, le voci senza numero AUD, la §9), `superpowers:writing-plans` per il piano del 2 in **due parti**, la prima fino allo spike compreso; il pre-controllo delle quattro domande di `CLAUDE.md` nella sessione che scrive il piano; l'esecuzione in una sessione nuova, subagent-driven | `CLAUDE.md`, e il modo scelto dal proprietario |
| 3 | ciò che il disegno consegna a chi scrive il piano: i pezzi della §3 della stella polare, la tabella artefatto → controllo della §8, le trappole, le fonti — npm e crates.io con le date — che entrano in `riferimenti.md` | le §3–§9 qui sopra e la §3 della stella polare |
| 4 | la **Definizione di «fatto» della parte 1 del piano**, che il piano copia da lì — la bozza qui sotto | la §5.5 del disegno della knowledge base, da cui la forma della tabella |

La bozza della Definizione di «fatto» della parte 1:

| # | Condizione | Chi la verifica |
|---|---|---|
| 1 | il protocollo dello spike congelato in `spikes/gui-shell/PROTOCOLLO.md` **al primo commit di codice**, con M1–M5, Q1–Q4 e le otto mosse | `git log` sul file: il primo commit precede ogni misura |
| 2 | i numeri di M1–M5 e le righe Q1–Q4 in ADR-0029, lo stato ad `Accepted` con l'innesco Linux scritto; la riga del guscio nella §4 del compendio chiusa | `check-docs.sh`, che non elenca più ADR in `Proposed`; il richiamo datato |
| 3 | le otto mosse giudicate dal proprietario con le sue parole in `spikes/RISULTATI.md`, la mossa 7 come due JSON uguali; l'esito di `dockview` scritto — resta, o la tela libera | la sezione nuova di `RISULTATI.md` |
| 4 | `.gitignore` con le cartelle di build dello spike; i lockfile dello spike committati | `git status --porcelain` vuoto dopo lo spike |
| 5 | **nessun codice di prodotto toccato** dalla parte 1: `git diff --stat <base>..HEAD -- crates/ scripts/ Cargo.lock` vuoto | il comando |
| 6 | fine-riga rimisurati per ogni file toccato | chi esegue |
| 7 | la parte 2 del piano scritta **dopo** la misura, con lo stesso pre-controllo | la sessione che la scrive |

**I tre controlli (decisione 18).** *Esiste:* le sezioni «Come si riprende» dei disegni dei gesti e della knowledge base, lette
il 2026-09-09; la Definizione di «fatto» del piano dei documenti della knowledge base (§5.5 di quel disegno), da cui la forma
della tabella; la §8 qui sopra. *Arriva:* la parte 2 del piano, dopo la misura. *Regge crescendo:* la sezione si riscrive a ogni
chiusura come diario, nello stesso file.

**Ciò che la §10 non fa:** non è ancora la consegna — si scrive alla chiusura della sessione che scrive i due disegni, coi
comandi rilanciati quel giorno, non ricordati.

**Controllo sui cinque criteri.** Verificato: i due precedenti, letti. Coerenza: stessa forma. Debito: nessuno. Stato
dell'arte: non c'entra. Proporzione: una sezione, una tabella. **Dedotto:** niente. **Assunto:** niente.

## Le sezioni che mancano — proposte del coordinatore, non decisioni

| § | Che cosa | La proposta da cui partire |
|---|---|---|
| 6b | la **forma** delle due schermate, coi wireframe a bassa fedeltà mostrati in chat | **schermata 1**: la vista chat a sinistra, larga; il pannello di stato a destra, stretto, con degrado, policy col controllo a due stati, budget, riga G16, e la riga di evento del verdetto sotto quando c'è; in alto la fascia dello stato di connessione, visibile solo se il core manca o il timbro è sbagliato. **Schermata 2**: la finestra di conferma del permesso, sopra la 1, con la tripla a parole («la GUI vuole cambiare la policy della memoria grafica»), due pulsanti, focus nel pulsante che rifiuta. Il proprietario ha chiesto di vederli **nella sessione nuova** |
| 7 | il core finto in `gui/fake-core/` | un binario Rust fuori dal workspace che dipende da `kernel` e `platform` per percorso; ascolta sullo stesso nome del daemon; accetta `Hello` e risponde `Accepted`, poi manda `Degradation` e `Policy`; poi **token a tempo** come lo spike (2000 in dieci secondi, testo non fidato), e su comando da riga di comando un `Verdict` o un cambio di `Degradation` per provare la riga di evento; risponde a `Invoke` come il daemon farebbe, con `PermissionRequired` la prima volta. Il suo `Cargo.lock` **si committa**, perché lo usa il cancello: è un attrezzo, non uno spike ✅ **RICHIAMO DEL 2026-09-09, quattordicesima ripresa della stella polare (decisione 33): la sezione è SCRITTA** — la §7 delle sezioni approvate qui sopra, **A**: l'attività vera del kernel su porte in memoria, più il rubinetto; questa riga era la forma B |
| 8 | le prove e il cancello | `scripts/gate-gui.sh`: `npm ci`, `npm run build`, `npm test`, chiamato da una riga `run` in `gate.sh`; la CI guadagna `actions/setup-node` con la versione appuntata; `.gitignore` guadagna `/gui/node_modules/`, `/gui/dist/`, `/gui/fake-core/target/`; per ogni artefatto il controllo che lo esercita, nella forma dei disegni precedenti: la tabella si compone dalle colonne «prova» delle §3–§6a, più le prove del core finto (una sonda che lo fa girare contro `FakeGui`? no: contro il trasporto vero, da un thread) e della SPA (unit sulle fixture, componenti con verifica di accessibilità, capo a capo **dopo il guscio**, con la prova del ponte in Node o in Rust secondo il vincitore) ✅ **RICHIAMO DEL 2026-09-09, quattordicesima ripresa della stella polare (decisioni 34 e 35, delegate: A e A): la sezione è SCRITTA** — la §8 delle sezioni approvate qui sopra; il passo del cancello, la CI, `.gitignore` e la tabella artefatto → controllo com'erano proposti qui, più la decisione sull'archivio che non si apre |
| 9 | le decisioni aperte del proprietario, col chiusore | il renderer di markdown; gli attrezzi di prova della GUI (`vitest` 5.0.0 di tre giorni contro la 4, `@playwright/test`, uno strumento di verifica dell'accessibilità); la regola di lint per le scritte; dove va la crate Rust del guscio se vince Tauri; la prontezza I/O del reattore (probabilmente il 3); l'allocatore dentro la porta `journal` (registrato); il confine di sessione dei permessi (il 3); il watchdog e lo spegnimento (il 10); AUD-004 in parallelo al 2; il ledger `.superpowers/sdd/` ✅ **RICHIAMO DEL 2026-09-09, chiusura della quattordicesima ripresa della stella polare: DECISA A su delega (decisione 36) e NON scritta** — la proposta parola per parola nel prossimo passo della stella polare, blocco «La proposta per la §9», coi consigli verificati alla fonte quel giorno. ✅ **RICHIAMO DEL 2026-09-09, quindicesima ripresa della stella polare: SCRITTA** — la §9 delle sezioni approvate qui sopra, con le misure rifatte quel giorno; il blocco com'era nella cronaca in archivio della stella polare |
| 10 | come si riprende | la sezione di consegna del disegno, sul precedente dei disegni dei gesti e della knowledge base ✅ **RICHIAMO DEL 2026-09-09: DECISA A su delega (decisione 37) e NON scritta** — una sola «Come si riprende», in questo file quando sarà il disegno; il blocco «La proposta per la §10» nel prossimo passo della stella polare. ✅ **RICHIAMO DEL 2026-09-09, quindicesima ripresa della stella polare: SCRITTA** — la §10 delle sezioni approvate qui sopra |

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

⚠️ **RICHIAMO DEL 2026-09-07: questo elenco è SUPERATO.** Il passo vivo sta nella sezione omonima di
[`2026-09-07-direzione-gui-design.md`](../superpowers/specs/2026-09-07-direzione-gui-design.md), e il puntatore nella §6 del
compendio. L'elenco resta com'era, come verbale di ciò che la sessione del 2026-09-06 lasciava.

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
