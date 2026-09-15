| R5-35 | 11 · *Read*, *«**D2**, **D3**, **D4**, **D35**…**D40**»* → riga **D2** del piano, *«l'evidenza delle otto mosse è sulla 8.2.0, e il compito 12 lo scrive accanto al primo uso»* | il primo uso di `dockview` 8.3.1 è al compito 12 | per la tabella della posizione `dockview` entra col **13** (D40: *«`pinia`, `reka-ui`, `vue-i18n`, `dockview` … arrivano coi compiti 13, 14 e 15»*); il 12 è il core finto e non installa nulla di npm (`grep -c 'dockview' c12.md` non serve: il manifesto è Cargo). Numero scritto prima di D25 | CONFERMATO | prosa | no | riga D2: *«e il compito 13 lo scrive accanto al primo uso»*, col richiamo datato nella riga come D3 ha fatto per P-81 |

## Copertura del disegno per il mio perimetro

Legenda: **coperta** = un Passo la produce · **spostata** = a un altro compito, per la tabella della posizione · **contraddetta** = il piano fa altro, con o senza richiamo · **scoperta** = nessun Passo.

| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|
| §6a · «dove e con che cosa»: `gui/` fuori dal workspace, Node in `package.json`, `vite`, Vue 3, TypeScript, lockfile committato, `npm ci` | 11 · Passi 3–4 (`vite`, `vue`, `typescript`, `vue-tsc`, `vitest`, `@vitejs/plugin-vue`, `engines.node`, `package-lock.json`); `pinia`, Reka UI, `vue-i18n`, `dockview-core` → 13 (**D40**); `npm ci` nel cancello → 15 | coperta / spostata (D40, dichiarata nel blocco *Interfaces* dell'11) |
| §6a · «il ponte»: nessun socket, messaggi già decodificati, quattro in uscita (`Hello`, `Invoke`, `Approve`, `SaveLayout`), ponte finto che rilegge le fixture | 11 · Passo 11 (`Bridge { send; listen }`, `OutboundMessage = Extract<IpcMessage, {kind: "Hello" \| "Invoke" \| "Approve" \| "SaveLayout"}>`), Passo 12 (`createFakeBridge` su `loadFixtures`) | coperta |
| §6a · «gli strati»: `transport/` → `schema/` (tipi e fixture: byte **e** JSON) → `stores/` → `components/` → `panels/` → `tokens/` → `locales/it.json` | 11 · Passi 7–12 per `transport/` e `schema/`; le fixture (byte e JSON) dal 3 (**D35**); `stores/`, `panels/`, `tokens/`, `locales/` → 13; `components/` → 14 | coperta / spostata |
| §6a · «la cornice», «i quattro stati della connessione» | 13 (**D48**, **D49**, **D50**) | spostata |
| §6a · «il modulo Stato», «il cambio di policy», «il modulo Chat», «i moduli Passi e Permessi», «accessibilità (G20)», «testi (G21)» | 14 (moduli, finestra, tastiera, `axe-core`); 13 (`locales/it.json`, `i18n.ts`); 15 (`no-raw-text`) | spostata |
| §6a · «la regola del kit»: nessun kit nel 2 | nessun compito — per assenza dichiarata | coperta |
| §6a · debiti: la parte del ponte che dipende dal guscio, aperta per nome | la tabella [C] delle voci aperte (secondo capo di `SOCKET_NAME`, decodifica vera, «finestra a parte») | coperta (dichiarata) |
| §6a · dedotti: la forma del ponte nei due gusci; le tre viste con `fromJSON` | il guscio è fuori piano; le viste → 13 | fuori / spostata |
| §7 · testa: attività vera del kernel, finto è solo il rubinetto; fuori dal workspace, lockfile committato, token a tempo come lo spike, `Invoke` → `PermissionRequired`, `Layout`/`SaveLayout`, lista dei passi | 12 · doc di modulo e Passo 6 (rubinetto), Passo 2 (`exclude`), Passo 4 (lockfile), `GUI_TICK` 5 ms = 2000 in 10 s (Passo 5); `Invoke`/`Layout`/`Steps` sono l'attività del 7 che il finto monta | coperta / spostata al 7 |
| §7 · «dove, e come si costruisce»: binario in `gui/fake-core/`, `exclude` di radice, dipendenze per percorso, lockfile committato; prova `cargo test --locked --manifest-path` | 12 · Passo 2 (**D44**, rosso prima), Passo 4 (manifesto), Passo 10 a mano; nel cancello → 15 | coperta (R5-10 sul lockfile che risolve da sé, R5-19 sullo script mancante) |
| §7 · «l'attività vera»: la stessa del daemon, costruita da fuori, `Executor` col limite del daemon; sonda da fuori `Hello` → `Accepted` → `Degradation` → `Policy` → `Layout` → `Steps` | 12 · Passo 7 (`serve`, `u64::MAX` in `main`), Passo 8 `the_welcome_gives_the_sequence_one` | coperta — ma R5-2 (non compila), R5-3 (il pari panica), R5-7 (G2 vacua) |
| §7 · «le porte in memoria»: `MemoryJournal`, finta della custodia, arbitro vero «costruito come lo costruisce il daemon», `SystemReactor` | 12 · Passo 5 (`build_the_arbiter` riscritto, **D41**), Passo 7 (`MemoryJournal::new()`, `MemoryCustody::new()`, `SystemReactor::new()`) | coperta; «come lo costruisce il daemon» **contraddetta** da D41 col richiamo previsto ma non dettato (R5-8) |
| §7 · «il trasporto vero»: stesso nome del daemon, timbro delle fixture; sonda da un thread, timbro giusto → fila, sbagliato → `StaleBuild` e più niente | 12 · Passo 5 `SOCKET_NAME` (**D45**, `diff` nel criterio), Passo 8 `a_stale_stamp_is_refused_and_then_silence` | coperta — R5-3 |
| §7 · «il rubinetto»: seconda attività sullo stesso esecutore, `RefCell` condivisa; `Token` a tempo, markdown, non fidato; `degrade` → nota `Detail::Routing`; `verdict` → arbitro vero; niente altro | 12 · Passo 6 (`the_faucet`, `SCRIPT`, `note_a_degraded_routing`, `tell_everyone`), Passo 7 (`spawn_the_console`, **D42**) | coperta |
| §7 · prove del rubinetto: `degrade` → `degradation_now` risponde `routing_degraded` e il pari riceve `Degradation`; `verdict` → il pari riceve un `Verdict`; token contati con la provenienza | 12 · Passo 8 `degrade_makes_the_real_code_report_it`, `verdict_reaches_the_peer`, `the_tokens_arrive_untrusted`, `the_faucet_keeps_streaming_with_no_word_waiting` (nessuna sonda interroga `degradation_now` direttamente: lo fa `serve`) | coperta — R5-4 (`verdict` a nessuno), R5-6 (negativa vacua) |
| §7 · «la disposizione»: `SaveLayout` → finta della custodia → `Layout` torna; «il giro salva → ritrova sul pari, dentro la sonda dell'attività» | nessuna sonda del 12 manda `SaveLayout`; il giro vive in `crates/kernel/tests/serving.rs` del **7** | spostata al 7, senza richiamo (R5-16) |
| §7 · «la lista dei passi»: `replay` sul giornale in memoria; «un `Invoke` dal pari, poi la lista porta l'invocazione» | nessuna sonda del 12 manda un `Invoke` accolto; nel **7** (`Steps … len() == 1 && steps[0].done`) | spostata al 7, senza richiamo (R5-16) |
| §7 · «Il costo di A»: dipende da `simulator`; costruzione da fuori; `RefCell` condivisa; disposizione finché il finto non riparte | 12 · manifesto (`simulator` con la ragione), **D43**/P-71, doc di modulo | coperta |
| §7 · «Ciò che la §7 non fa»: non persiste, non parla con un modello, non prova il daemon, non convive col daemon | 12 · doc di modulo (*«IT DOES NOT LIVE ALONGSIDE THE DAEMON»*) | coperta |
| §7 · debito: «come l'attività si accorga che `degradation_now` è cambiato … lo fissa il piano» | chiuso dal **7** (`an_unchanged_degradation_is_not_resent`, **P-70**); il 12 lo esercita | spostata al 7, senza richiamo nella §7 |
| §7 · dedotti: `RefCell` condivisa; «che il valore di `Accepted` sia consegnato al finto come al daemon»; la forma di ciò che sta nella `RefCell`; la costruzione dell'arbitro «senza copiarla» | **D43** (la stessa `RefCell<Core>`); `Accepted` è un letterale di `serve` (**P-41**, del 7: un parametro con un solo valore); `Core` intero; **D41** riscrive | coperta / contraddetta con P-41 (nessun richiamo in §7) / coperta / contraddetta con richiamo previsto e non dettato (R5-8) |
| §8 · `scripts/gate-gui.sh`, la riga in `gate.sh`, la CI (`setup-node`), la campagna DST nel settimo passo | 15, 15, 15 (**D66**), 10 | spostata |
| §8 · «il core finto nel cancello»: lockfile committato, sonde in `main.rs` (decisione 48), costo della ricompilazione «dichiarato e misurato al piano» | 12 · Passo 4 e Passo 8; il tempo: nessun comando | coperta; il tempo **scoperta** (R5-17) |
| §8 · «la versione di Node»: `engines.node` casa unica, `.npmrc` `engine-strict=true`, nelle due direzioni | 11 · Passo 2 (intersezione, **D37**), Passo 3, Passo 4 (due direzioni; la terza in **P-64**) | coperta |
| §8 · `.gitignore`: `/gui/node_modules/`, `/gui/dist/`, `/gui/fake-core/target/`; `spikes/gui-shell/` | 11 · Passo 3 (due righe, **D38**); 12 · Passo 2 (una); le otto di `spikes/gui-shell/` esistono (**P-102**, **D67**) | coperta (taglio D38, dichiarato) |
| §8 · artefatti: trasporto `ipc`, contatore, stretta di mano, schema, registro, attività del daemon, limite di giri e `Disconnected`, settima porta, «salva, riavvia, ritrova» | 2, 1, 7 (+ 12 Passo 8 per lo stantio dal thread), 3, 6, 10, 9, 4–5, 9 | spostata (fuori perimetro) |
| §8 · «il core finto (§7)»: sequenza 1; trasporto vero da un thread, timbro giusto e sbagliato; una sonda per parola; token contati con la provenienza | 12 · Passo 8, le sei sonde | coperta — R5-3/4/6/7 |
| §8 · «la SPA, `schema/`»: «ogni variante decodificata dai byte e confrontata col valore atteso in JSON; una variante senza fixture → rosso» | 11 · Passo 10 (`parses every fixture…`, `has a fixture for every kind … and no fixture for any other`, nelle due direzioni); «dai byte» → **D36** | coperta per il confronto; **contraddetta** per «dai byte», senza richiamo (R5-15) |
| §8 · «la SPA, `stores/`, componenti e pannelli», «le scritte» | 13, 14, 15 | spostata |
| §8 · «la build della SPA»: `npm run build` verde; `npm ci` gemello di `--locked` | 11 · Passo 6 e Passo 14; `npm ci` nel cancello → 15 | coperta — R5-1 la rende rossa oggi |
| §8 · «capo a capo: la SPA nel guscio col core finto» | fuori dal cancello, dichiarato ([C]) | coperta (dichiarata) |
| §8 · «le dipendenze nuove»: Rust in due passi con `--locked` e `gate-deps.sh`; web `npm ci` sul lockfile | 11 · Passi 4, 14; 12 · Passo 3 (`gate-deps.sh`), Passo 4 (manifesto + lockfile nello stesso commit) | coperta — R5-10 (il secondo lockfile risolve da sé, e nessuno lo confronta né lo scandisce) |
| §8 · «il registro della porta», «i documenti», «il codice fuori dal perimetro» | 17 | spostata |
| §8 · decisione 35 (l'archivio che non si apre), decisione 34 (cancello unico) | 7 e 9; 15 | spostata |
| §8 · «Ciò che la §8 non fa»: CI solo Linux; capo a capo; lint delle scritte; X-3; il tempo del cancello col passo web | 16; fuori; 15; 16; 15 (non trovato: R5-17) | spostata |
| §8 · dedotti: `npm ci` onora `engine-strict`; prove senza browser in CI; `--manifest-path` compila nel `target/` del finto | **P-64** e 11 · Passo 4; 11 (`environment: "node"`) e 13 (`jsdom`); 12 · Passo 4 (`git status` senza `target/`), rimisurato qui (R5-30), richiamo al 15 (**P-104**) | coperta |
| §8 · decisioni 46–50 | 46 → 11; 47 → 15 (**D66**); 48 → 12 · Passo 8; 49 → 11/12 (lockfile); 50 → 7 | coperta / spostata |
| §9 · voce 1 (`markdown-it`) | 14 (**D3** col richiamo di **P-81**) | spostata |
| §9 · voce 2 (attrezzi di prova): `vitest` 4.1.11, `jsdom`, `@vue/test-utils` 2.5.0, `axe-core`, `vue-tsc` 3.3.11 | 11 (`vitest`, `vue-tsc`, **D4**); 13 (`jsdom`, `@vue/test-utils`); 14 (`axe-core`); `@playwright/test` non installato (**P-2**) | coperta — R5-1 (`vue-tsc` col `typescript` appuntato) |
| §9 · voce 3 (`no-raw-text`), voce 14 (cache npm), voce 13 (X-1/X-3) | 15 (**D63**–**D66**), 15 (**D66**), 16 | spostata |
| §9 · voce 4 (`gui/shell/` se vince Tauri) | il guscio non è in questo piano; Electron scelto (memoria del proprietario) | fuori |
| §9 · voci 5, 6, 11, 12 (del proprietario) | nessuna azione nel piano, dichiarate in [C] | coperta (dichiarata) |
| §9 · voci 7, 8, 17, 18 (il 3, il 10, il 10, il 4) | fuori dal piano, dichiarate in [C] | coperta (dichiarata) |
| §9 · voce 9 (policy riletta), voce 10 (commento falso in `journal.rs`), voce 15 (roadmap e cifre di `ports/mod.rs`), voce 16 (nomi) | 8; 5 (**D14**, primo compito che tocca il file); 4 (**P-21**) e 17; il disegno | spostata |
| §9 · «Il comando che rifà la misura» (registro npm, `latest`, date, download) | 11 · Passo 2 rifà solo `engines` | coperta a metà (R5-18) |
| §9 · «Le opzioni di `markdown-it` si leggono dentro il pacchetto» | 14 (**D3**) | spostata |
| §9 · decisioni 51–57 | 51 → 14; 52 → 11; 53 → 13; 54 → 14; 55 → 15; 56 → 8; 57 → fuori | coperta / spostata |

## Voci P rimisurate

| P | Comando rilanciato → resa | Regge? |
|---|---|---|
| P-2 | `npm view <pkg>@<v> version engines peerDependencies --json` per `vue`@3.5.42, `vite`@8.3.0, `@vitejs/plugin-vue`@6.0.9, `typescript`@7.0.2, `vue-tsc`@3.3.11, `vitest`@4.1.11, `jsdom`@30.0.1 → tutte esistono; `dist-tags`: `vitest` latest **5.0.1** (2026-09-15), `V4` 4.1.11; gli altri `latest` = appuntata; crates.io `interprocess` `max_version` 2.4.4, `redb` 4.3.0 | sì per le versioni; **no nel merito per `typescript` 7.0.2**, che `vue-tsc` 3.3.11 non sa guidare (R5-1) |
| P-11 | lettura del Passo 6: `asked = ResourceProfile { name: "faucet-probe", … }` costruito nel sorgente, `held.arbiter().admit(&asked, FOR_EVER, now)` → nessun byte in arrivo decodificato in un profilo | sì |
| P-18 | `git check-ignore -v gui/schema/fixtures/x.bin` → nulla, EXIT=1; `ls gui` → non esiste | sì |
| P-62 | `grep -n 'mappa .indice' <disegno2>` → 1 riga (240); `grep -cn 'valore atteso in JSON' <disegno2>` → 2; il Passo 6 del 3 detta `variant_json` a mano | sì |
| P-63 | `grep -n 'non si carica da .nessuno. dei due punti' docs/riferimenti.md` → **0 righe** (i `**` attorno a «nessuno»); `grep -in 'non si carica' docs/riferimenti.md` → 1 riga (1681) | nel merito sì; il comando no (R5-21) |
| P-64 | le tre direzioni rifatte in `probe-R5/engine-probe/` → `EXIT=0` / `EXIT=1` + `npm error code EBADENGINE` / `EXIT=0` + `npm warn EBADENGINE` | sì |
| P-65 | lo script rilanciato (con `@vitejs/plugin-vue` 6.0.9) → `jsdom` 30.0.1 `^22.22.2 \|\| ^24.15.0 \|\| >=26.0.0` è il più stretto su ogni ramo (`eslint` `^20.19.0 \|\| ^22.13.0 \|\| >=24`, `vitest` `^20.0.0 \|\| ^22.0.0 \|\| >=24.0.0`, `vite`/plugin `^20.19.0 \|\| >=22.12.0`, `vue-i18n` `>= 22`); `node --version` → v24.9.0; LTS `v24.21.0` | sì |
| P-66 | `grep -c '^/gui/' .gitignore` → 0; `git ls-files --eol .gitignore` → `i/lf w/crlf`; lo script del Passo 3 dell'11 provato sulla copia → 4 righe CRLF aggiunte | sì |
| P-67 | `npm view @vitejs/plugin-vue@6.0.9` → `engines` `^20.19.0 \|\| >=22.12.0`, pari `vue ^3.2.25`, `vite ^5.0.0 \|\| ^6.0.0 \|\| ^7.0.0 \|\| ^8.0.0`; `npm run build`/`vite build` del modello lo attraversa | sì |
| P-68 | `grep -n 'fn build_the_arbiter\|fn reserve\|const AUDIO_QUOTA\|const PRESENTATION_QUOTA\|const TOTAL_VRAM' crates/daemon/src/main.rs` → 288, 326, 156, 157, 123; `ls crates/daemon/src/lib.rs` → non esiste; `grep -n 'simulator' crates/daemon/Cargo.toml` → il solo commento *«Does NOT depend on `simulator`»* | sì |
| P-69 | `sed -n '18,28p' crates/kernel/src/executor.rs` → *«One decision at a time … THAT LOCK DOES NOT EXIST»*; `grep -n 'pub fn run' -A 12` → il `while` con `turns > self.turn_limit` | sì (ma la sonda negativa del 12 non lo coglie: R5-6) |
| P-70 | `grep -c 'an_unchanged_degradation_is_not_resent\|il pezzo che è cambiato' <piano>` → 5; nel 7 la sonda esiste (c7: `fn an_unchanged_degradation_is_not_resent`) e `serve` rilegge `degradation_now(&self.arbiter, &self.journal)` | sì |
| P-71 | `grep -n 'pub async fn serve\|fn serve<' <piano>` → `pub async fn serve<'a, I, J, C, R>(core: &'a RefCell<Core<I, J, C>>, clock: &'a R, sleep: &'a Sleep)`; il 12 prende in prestito la stessa `RefCell` | sì |
| P-72 | il workspace usa-e-getta rifatto in `probe-R5/p72/` → `EXIT=101` *«current package believes it's in a workspace when it's not»* senza, `EXIT=0` con `exclude = ["spikes", "gui"]` | sì |
| P-73 | `grep -n 'const SOCKET_NAME' <piano>` → 9 e 12 con `"harness-core"`; `grep -c 'vive in una casa sola' <piano>` → 3; il `diff` sui modelli → `EXIT=0` | sì |
| P-74 | `grep -rn 'struct SharedClock' crates/ --include='*.rs'` → oggi 1 (`crates/simulator/tests/arbiter_campaign.rs`); il modello del 12 → 1; il manifesto del 12 porta `simulator = { path = "../../crates/simulator" }` | sì |
| P-75 | `grep -n 'interface Fixture' <piano>` → il blocco *Interfaces* dell'11 dice **ancora** `{ file, kind, value }`; il Passo 9 `{ file, message }` | **no**: la correzione che la voce annuncia non è nel blocco (R5-9) |
| P-76 | `grep -n 'IpcMessage::Layout' c3` → in `stamp_set` solo `Package(alloc::vec![0x7B, 0x7D])`; `LayoutState` a tre varianti; la finta del 12 consegna *«the fixture of that kind»* | sì |

## Numeri di compito ricensiti

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| 11 · *Interfaces*, *«Produces, e i compiti 12, 13 e 14 li usano con questi nomi esatti»* | 12, 13, 14 | 13, 14 (il 12 non consuma nulla di `gui/src/`) | **stantio** (R5-13) |
| riga **D2** letta dall'11, *«il compito 12 lo scrive accanto al primo uso»* (di `dockview` 8.3.1) | 12 | 13 (**D40**) | **stantio** (R5-35) |
| 11 · *Interfaces* *«le fixture del compito 3»*; Passo 1 *«il compito 3 l'ha creata»*; Passo 10 *«lo produce il compito 3»*; Passo 13 G3 | 3 | 3 | giusto |
| 11 · *Files* e Passo 3 *«`/gui/fake-core/target/` … è del compito 12»* | 12 | 12 | giusto |
| 11 · *Interfaces* *«il cancello impara il mondo web al compito 15»*; Passo 14 | 15 | 15 | giusto |
| 11 · *Files* *«la §1 e la §2 della stella polare, che sono dei compiti 13 e 14»*; Passo 5 *«la cornice e i moduli sono i compiti 13 e 14»*; `App.vue` *«TASKS 13 AND 14»* | 13, 14 | 13, 14 | giusto |
| 11 · `vite.config.ts` *«`jsdom` arrives with the components, in the task that has something to render»* | (13) | 13 (**D40**) | giusto |
| 11 · `fixtures.ts` *«written by `regenerate_the_fixtures` in `crates/kernel/tests/ipc_wire.rs`»* | (3) | 3, nome esatto (`fn regenerate_the_fixtures()` nel Passo del 3) | giusto |
| 12 · *Files* e *Interfaces* *«dal compito 1 … 2 … 3 … 5 … 7»*; *«`MAX_BODY` e `SOCKET_NAME` dal compito 9»*; *«P-56 — gli accessori sono CINQUE»* | 1, 2, 3, 5, 7, 9 | idem | giusto |
| 12 · Passo 1 *«due righe `/gui/` in `.gitignore`, dal compito 11»*; criterio → 3 | 11 | 11 | giusto |
| 12 · Passo 5 *«the projection of task 8»*; *«the milestone-2 DST campaign»*; *«the same cure D28 gave task 9»*; P-74 *«`crates/kernel/tests/serving.rs`, the milestone-2 campaign and `crates/daemon/src/main.rs`»* | 8, (10), 9, 7/10/9 | 8, 10, 9 (D28 è del 9), 7/10/9 | giusto |
| 12 · Passo 6 *«task 7, and its probe `an_unchanged_degradation_is_not_resent`»*; Passo 8 *«IF TASK 7 ALREADY EXPOSES SUCH A HELPER»*, *«task 3: a stamp anyone can mint…»*, *«The peer, in the shape task 9 gives it»*, *«The lesson is task 9's»* | 7, 7, 3, 9, 9 | idem | giusto |
| 12 · Passo 8 *«È la stessa voce che il compito 9 aggiunge al daemon (P-54)»*; Passo 10 *«lo impara al compito 15»*; commit *«gui(compito 12)»* | 9, 15, 12 | idem | giusto |

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

- `wc -l -c skeleton.md p-titles.md` → 678 righe / 82 967 byte; 116 / 15 009
- `grep -n '^## Compito 1[0-3]:\|^## Come si riprende' <piano>` → 11126, 11957, 12873, 13655, 18801
- `grep -n '^### §\|^## ' <disegno2>` → §6a 281, §7 317, §8 386, §9 488, §10 570
- `git ls-files --eol` piano e i due disegni → `i/lf w/lf` ×3; `git rev-parse --short HEAD` → `baf3cde`; `git status --porcelain \| wc -l` → 0
- `sed -n` sullo scheletro (4 blocchi), `cat p-titles.md`, `sed -n '281,316p;317,385p;386,436p;437,487p;488,569p' <disegno2>` → letture
- `awk '/^## Compito 11:/{f=1} /^## Compito 12:/{exit} f'` → `c11.md` 916 righe; idem 12 → `c12.md` 782; grep delle intestazioni interne → *Files* 3, *Interfaces* 18/15
- `awk` per P-2, P-11, P-18, P-62…P-76 → `pvoices.md` 518 righe, 18 voci
- `grep -n '^| \*\*D<n>\*\* |'` per D2, D3, D4, D9, D31, D35…D48 → 19 righe
- `git ls-files --eol Cargo.toml .gitignore crates/platform/Cargo.toml crates/daemon/Cargo.toml crates/daemon/src/main.rs rust-toolchain.toml` → tutti `i/lf w/crlf`; `cat -A Cargo.toml`, `cat -A .gitignore`, `cat rust-toolchain.toml`, manifesti di `platform` e `daemon` → letture
- Passo 1 dell'11: `ls gui` → non esiste; `ls gui/schema/fixtures/*.bin \| wc -l` → 0; `*.json` → 0; `grep -c '^/gui/' .gitignore` → 0; `git ls-files --eol .gitignore` → `i/lf w/crlf`; `node --version` → v24.9.0; `npm --version` → 11.6.0
- Passo 1 del 12: `ls gui/fake-core` → non esiste; `grep -n 'exclude' Cargo.toml` → `13:exclude = ["spikes"]`; `grep -n 'pub fn ipc' crates/kernel/src/serving.rs` → file assente (il 7 non è eseguito); `grep -nE '^\s*pub fn (journal\|arbiter\|custody\|grants\|attending\|ipc)' …serving.rs` → assente; `grep -n 'const SOCKET_NAME\|const MAX_BODY\|fn build_the_arbiter\|fn reserve\|const AUDIO_QUOTA\|const PRESENTATION_QUOTA\|const TOTAL_VRAM\|const ARBITER_ID' crates/daemon/src/main.rs` → 123, 137, 156, 157, 288, 326 (`SOCKET_NAME`/`MAX_BODY` assenti: sono del 9)
- `grep -rc 'struct SharedClock' crates/ --include='*.rs' \| grep -v ':0$'` → `crates/simulator/tests/arbiter_campaign.rs:1`; `grep -rl … \| wc -l` → 1
- `grep -c 'RICHIAMO DEL 2026-09-14' <disegno2>` → 0
- `grep -rn 'path = "../../crates\|path = "../../../crates' spikes/ --include=Cargo.toml` → nulla
- python su `.gitignore` con `newline=""`: `count('# Build artefacts of the spikes')` → 1; `count('/gui/node_modules/')` → 0; CR → 55
- `cd spikes && rustup show active-toolchain` → `1.95.0-x86_64-pc-windows-msvc (overridden by 'C:\Users\zagor\Desktop\harness\rust-toolchain.toml')`
- grep di firme in `crates/kernel/src/wire/ipc.rs`, `record.rs`, `ports/journal.rs`, `arbiter/mod.rs`, `arbiter/resource.rs`, `time.rs`, `parameters.rs`, `executor.rs`, `degradation.rs`, `ports/ipc.rs`, `ports/reactor.rs`, `crates/platform/src/reactor.rs`, `rng.rs` → tutte coincidono col codice dettato (R5-31, R5-32)
- `sed -n '1,140p' crates/daemon/src/main.rs`; `sed -n '288,340p'` → `build_the_arbiter`, `reserve`, `FOR_EVER` satura
- `grep -n 'SharedClock(&' <piano>` → 13349, 13353, 13354 (tutte del 12); `grep -n 'let clock = SharedClock…' <piano>` → 8209 (7), 10674 (9)
- `grep -n 'const SOCKET_NAME\|const MAX_BODY' <piano>` → 10378/10393 (9), 13079/13083 (12), entrambi `"harness-core"` e `1024 * 1024`
- `grep -n '^interprocess = \|interprocess = {' <piano>` → 3666 `interprocess = "2.4.4"` (2), 11049 (9), 13015 (12)
- c7: `grep -nE 'pub struct Core\|ipc: I\|pub fn (new\|journal\|arbiter\|custody\|grants\|attending\|ipc)\|pub async fn (serve\|nap)\|an_unchanged_degradation_is_not_resent\|degradation_now\|IpcMessage::(Accepted\|Layout\|Policy\|Degradation)\('` → `Core` 1173, `ipc: I` 1174, `new` 1190, cinque accessori 1214–1242, `serve` 1260, sequenza 1383–1393, sonda 1002, `nap` 316; `grep -n 'fn ipc' c7` → nulla; `sed -n '1188,1212p' c7` → ordine di `Core::new`; `grep -n 'fn steps\|InvocationDetail\|Detail::Invocation' c7` → 1602
- c3: `grep -n 'pub enum IpcMessage'` → 240; tipi 83–231; `sed -n '595,705p'` (`variant_json`), `'297,352p'` (`stamp_set`), `'576,593p'` (`variant_name`), `'707,756p'` (aiutanti JSON); `grep -n 'pub fn decode\|pub fn encode\|fn unframe' c3` → nulla; `regenerate_the_fixtures` → 539
- consumatori: `grep -n 'deliverAll\|\.deliver(\|\.sent\b\|createFakeBridge\|loadFixtures\|Fixture\b\|MESSAGE_KINDS\|parseIpcMessage\|SchemaError\|OutboundMessage\|type Listener\|\.emit(' <piano>` fra 13655 e 18801 → `sent`/`deliver`/`deliverAll`/`FakeBridge` nei 13 e 14; nessun `.emit(`, `loadFixtures`, `Fixture`
- c16: `grep -n 'cargo audit\|--file\|fake-core\|Cargo.lock'` → `run "dependency advisories" cargo audit`; nessun `--file` né `fake-core`
- `npm view <pkg>@<v> version engines peerDependencies --json` ×7 e `npm view <pkg> dist-tags --json` ×7 → R5-34
- python al registro: `latest` e date dei sei; lo script di P-65 con `@vitejs/plugin-vue`; crates.io `interprocess` (max 2.4.4, `features.default = []`); `nodejs.org/dist/index.json` → LTS v24.21.0
- `sed -n '150,200p' crates/kernel/src/wire/ipc.rs` → `encode` incornicia, `decode` comincia con `framing::unframe`; `sed -n '574,582p;624,632p' record.rs`; `sed -n '625,632p' arbiter/mod.rs`; `grep -n 'saturating_add\|valid_for' arbiter/mod.rs`
- `crates/simulator/src/journal.rs`: grep + `sed -n '125,185p'` → `intent` rifiuta solo il doppio intent (`has_intent`), `note` vuole l'intent
- `grep -n -A14 'pub enum JournalError' ports/journal.rs`; `grep -nE 'pub trait Reactor\|fn (now\|wall_time\|wait_until)' ports/reactor.rs` → 35, 37, 51, 142; `grep -n 'pub const fn new\|pub fn new' executor.rs rng.rs` → 163, 214, 32; `grep -n 'SequentialRng::' crates/daemon/src/main.rs` → 264
- `grep -n '^| \*\*D28\*\* |' <piano>` → *«run_the_graph con un limite FINITO e un tick NULLO»* (compito 9)
- `ls ~/.cargo/registry/src/*/ \| grep interprocess` → 2.4.3, 2.4.4; `grep -A8 '^\[features\]'` → `default = []`
- `grep -n 'pub fn take_frame' <piano>` → 3638; `sed -n '3612,3662p'` → rende `(&body[..declared], LENGTH_WIDTH + declared)`
- c2: `grep -n 'fn receive\|take_frame(\|unframe(\|IpcMessage::decode\|verbatim\|VERBATIM'` e `sed -n '624,640p'` → `receive` rende la busta intera
- c9: `grep -n 'fn a_peer_that_says\|struct SharedClock\|fn run_the_graph\|…'` → 467, 173, 367; `sed -n '455,535p'` (il pari e la nota); `sed -n '367,412p'` (ordine di dichiarazione); `grep -n 'run_the_graph(' c9` e `sed -n '547,556p;620,628p;676,684p;692,700p'` → limiti 8, 100 001, 600, 600, tick `Millis::ZERO`
- `grep -n '^```' c11.md` / `c12.md` → mappa dei blocchi; `grep -in 'richiamo' c12.md` → 11, 776; `grep -n 'interface Fixture' <piano>` → 1955, 1956, 11979, 12559, 19081; `sed -n '19076,19090p' <piano>` → la nona chiusura; grep dei numeri di compito in c11/c12
- python: modello `model-fake-main.rs` dai 5 blocchi Rust del 12 (517 righe), `model-core-ipc.rs`, due `.toml`
- sul modello: `grep -cE 'IpcMessage::(Hello\|Invoke\|Approve\|SaveLayout) *(\(\|=>)'` → **7** (400, 430, 431, 453, 471, 483, 509); metà non-test con `awk '/#\[cfg\(test\)\]/{exit} {print}'` → **0**; `grep -c 'struct SharedClock'` → 1; `grep -cE '^\s*fn [a-z_]+\(\) \{\}$'` → 0; `grep -c TOKEN_CADENCE` → 0; `diff <(grep -o 'const SOCKET_NAME…' c9) <(… modello)` → `EXIT=0`; `printf 'exclude = ["spikes", "gui"]' \| grep -c 'exclude = \["spikes", "gui"\]'` → 1; `grep -n 'build_stamp'` → 28 e sonde; `grep -n '#\[cfg(test)\]'` → 309; `grep -c 'allocated'` → 0; `grep -cw 'Grant'` → 0; `grep -cw 'Detail'` → 0; `grep -c 'gui/src' c12.md` → 0
- `probe-R5/e0716/`: `cargo build` con `&SharedClock(&reactor)` → `error[E0716]`, EXIT=101; `let clock` dopo l'esecutore → `E0597`; `let clock` prima → EXIT=0; `cargo test` → `recv_on_a_dropped_sender_returns_at_once … ok`
- `probe-R5/p72/`: `cargo metadata --manifest-path gui/fake-core/Cargo.toml` senza `gui` in `exclude` → EXIT=101 *«current package believes it's in a workspace when it's not»*; con → EXIT=0; `grep -c 'exclude = \["spikes", "gui"\]' Cargo.toml` → 1; dipendenza per percorso dal finto al membro con `edition.workspace = true` → `cargo build` EXIT=0, `gui/fake-core/Cargo.lock` e `gui/fake-core/target/debug/` creati, nessun `target/` di radice
- `probe-R5/engine-probe/`: `npm ci` con engines ok+strict → 0; impossibili+strict → 1 + `EBADENGINE`; impossibili senza `.npmrc` → 0 + `npm warn`
- `git diff --stat` su un file fuori dal repository/non tracciato → 0 righe
- `probe-R5/gui/`: estrazione dei 13 file dai blocchi di c11 + 14 fixture nel formato di `variant_json`; `npm install --no-audit --no-fund` → 118 pacchetti, EXIT=0, versioni installate = appuntate; `npm run build` → `ERR_PACKAGE_PATH_NOT_EXPORTED './lib/tsc'`, EXIT=1; `node -e` sugli `exports`/`bin` di `typescript@7.0.2`; `sed -n '66,80p' node_modules/vue-tsc/index.js` → `resolveTscPath(require.resolve('typescript/lib/tsc'))`; `npx tsc --version` → 7.0.2; `npx vite build` → EXIT=0, `dist/index.html`; `npx vitest run` → 2 file, 9 sonde verdi; senza file di sonda `npx vitest run` → EXIT=1
- `probe-R5/gui-ts5/`: `npm view typescript versions --json` → ultima 5.x **5.9.3**; `npm install` → 98 pacchetti; `npx vue-tsc --noEmit` → EXIT=0; `npm run build` → EXIT=0; `npm test` → 9/9; `npm view typescript@5.9.3 engines` → `>=14.17`
- dry run dello script `.gitignore` sulla copia → `diff` 4 righe `>` CRLF; CR 55 → 59; `grep -c '^/gui/'` → 2; il blocco fra «product» e «spikes»
- P-62: `grep -n 'mappa .indice' <disegno2>` → 240; `grep -cn 'valore atteso in JSON'` → 2 · P-63: `grep -n 'non si carica da .nessuno. dei due punti' docs/riferimenti.md` → 0; `grep -in 'non si carica'` → 1681 · P-68: greps → 288, 326, 123, 156, 157; `ls crates/daemon/src/lib.rs` → non esiste; `grep -n 'simulator' crates/daemon/Cargo.toml` → il commento · P-69: `sed -n '18,28p' executor.rs`, `grep -n 'pub fn run' -A 12` · P-70: `grep -c …` → 5 · P-71: → 8909 · P-73: `grep -c 'vive in una casa sola'` → 3 · P-76: `grep -n 'IpcMessage::Layout' c3` → 335 · P-18: `git check-ignore -v gui/schema/fixtures/x.bin` → nulla, EXIT=1
- `probe-R5/fakecore-probe/` (dipendenze per percorso dalle crate VERE): `cargo metadata --format-version 1` → EXIT=0, 25 pacchetti; risolti `redb 4.3.0`, `interprocess 2.4.4`, `bincode 2.0.1`, `minicbor 2.3.0`; `grep -A1 '^name = "redb"' Cargo.lock` di radice → `4.1.0`; `bincode` 2.0.1; `minicbor` 2.3.0; crates.io `redb` `max_version 4.3.0` (2026-09-15); `git status --porcelain \| wc -l` → 0
- `grep -n 'decodificata dai byte\|risposta 10\|la SPA, .schema/' <piano>` → 1966 (P-63), 11971 (Read dell'11); `grep -n 'la SPA, `schema/`' <disegno2>` → 424; `grep -n 'ricompila\|tempo dichiarato\|secondi' c15` → nulla; `grep -n 'ricompila\|tempo' c12.md` → nulla; `grep -c 'senza copiarla' <disegno2>` → 1; `grep -c 'dove spostarla lo decide' <disegno2>` → 1
- finale: `git status --porcelain \| wc -l` → 0; `git ls-files --eol` di piano e disegni → `i/lf w/lf` ×3; `git rev-parse --short HEAD` → `baf3cde`

## Non verificato, e perché

- **La corsa fra il pari e i giri a cadenza zero** (R5-13): senza i compiti 1–9 eseguiti non si può lanciare; il compito 9 accetta la stessa forma con 600 e 100 001 giri, quindi è un'ipotesi condivisa e non un difetto nuovo del 12.
- **I valori veri delle fixture del 3**: le 14 del probe seguono il formato di `variant_json`, i nomi di `variant_name` e l'ordine di `stamp_set` letti nel piano; il generatore non esiste ancora.
- **`cp Cargo.lock gui/fake-core/Cargo.lock` come semina** (rimedio di R5-10): non provato; è il comportamento documentato di cargo (le voci inutili si potano, quelle che soddisfano restano). L'alternativa `cargo update -p redb --precise 4.1.0` nel finto è un comando di verifica facile.
- **Che `typescript` 5.9.3 resti la scelta il giorno dell'esecuzione**: si rimisura col Passo 2 corretto (R5-18); oggi `typescript@6` è solo `beta` nel registro e `vue-tsc` 3.3.11 nomina l'alias `@typescript/typescript6` nel proprio risolutore.
- **Il tempo di compilazione del finto** (R5-17) e **`cargo audit --file gui/fake-core/Cargo.lock`**: non lanciati, il finto non esiste.
- **`a_stale_stamp_is_refused_and_then_silence` che non si pianta**: regge per l'ordine dei drop solo se `LocalSocketIpc` del 2 chiude i flussi accolti quando cade; letto nel disegno del pari del 9 e del 12, non compilato.
- **Sei sonde in parallelo su sei named pipe** (`cargo test` per difetto): non misurato; il 9 usa la stessa forma con `socket_name_for_line`.
- **Che `serve` mandi i cinque messaggi dell'accoglienza in un solo `poll`** (nessun `Token` in mezzo): letto in c7 (`tell` in sequenza nello stesso metodo), non eseguito.

## Stato finale

`git status --porcelain` → (vuoto) · `git ls-files --eol` di piano e disegni → `i/lf w/lf` invariato · `HEAD` → `baf3cde` · nessun file del
repository letto in scrittura né toccato; tutte le prove in `<scratchpad>/review/probe-R5/` (`gui/`, `gui-ts5/`, `e0716/`, `p72/`,
`fakecore-probe/`, `engine-probe/`, i modelli `model-fake-main.rs`, `model-core-ipc.rs`, `model-toml-*.toml`, `c11.md`, `c12.md`,
`pvoices.md`, `gitignore-copy`).
