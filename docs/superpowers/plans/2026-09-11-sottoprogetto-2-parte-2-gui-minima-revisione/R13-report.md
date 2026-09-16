# Rapporto R13 — i moduli riscritti dalla revisione e mai compilati: le sonde del 12 e i moduli del 14 — 2026-09-16

## Esito in tre righe

Il **14 compila**: `npx vue-tsc --noEmit` esce **0** senza una riga, su tutti i file del 13 e del 14 riscritti dal
piano di oggi. Ma le sue **sonde non passano**: `npx vitest run` esce **1** con **tre** `it` rossi in due file —
uno in `markdown.test.ts` (due asserzioni dello **stesso** `it` si contraddicono: `not.toContain("href=")` contro
`toContain('data-href="…"')`) e due in `modules.test.ts` (la finestra di `reka-ui` vuole **tre** `nextTick` per
lasciare il DOM, e le sonde ne aspettano **uno**).
Con i due rimedi misurati la suite va **verde**: `Test Files 14 passed | 1 skipped`, `Tests 81 passed | 1 skipped`.

## Rilievi

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R13-1 | **14**, Passo 7 (`gui/src/components/markdown.test.ts`), l'`it` *«renders a link as text that shows its target, with nothing to follow»* — le due righe `expect(html).not.toContain("href=");` e `expect(html).toContain('data-href="https://example.com/x"');` | lo stesso `it` pretende che l'HTML **non** contenga `href=` **e** che contenga `data-href="https://example.com/x"` | `npx vitest run src/components/markdown.test.ts` → **1 failed**: `AssertionError: expected '<p>vedi <span class="link" data-href=…' not to contain 'href='`; il reso è `<p>vedi <span class="link" data-href="https://example.com/x">qui</span></p>`. ⛔ **`data-href=` CONTIENE la sottostringa `href=`**: le due asserzioni sono mutuamente esclusive e l'`it` è rosso **sempre**, qualunque cosa faccia `renderMarkdown` | CONFERMATO | fatto | **sì** | nel Passo 7, `expect(html).not.toContain("href=");` → `expect(html).not.toContain(" href=");` (con lo **spazio** in testa: `<a href=` ha lo spazio, `data-href=` ha il trattino). Misurato: con questa sola riga cambiata `markdown.test.ts` passa **5/5**. Alternativa equivalente, non misurata: `expect(html).not.toMatch(/(^\|[^-])href=/)` |
| R13-2 | **14**, Passo 13 (`gui/src/panels/modules.test.ts`), l'`it` *«opens only when the core asked AND a call is in flight, and Approve carries the call»* — `(yes as HTMLButtonElement).click();` seguito da **un solo** `await nextTick();` e poi `expect(document.querySelector(".confirm")).toBeNull();` | dopo il click sul «Consenti» e **un** `nextTick`, `.confirm` non è più nel DOM | `npx vitest run src/panels/modules.test.ts` → rosso: `AssertionError: expected <div data-v-e4bfeb46 …(9)>…(3)</div> to be null`. Il nodo reso porta `data-state="closed"` e `data-focus-scope-unmounting=""`: **la logica è giusta** (`core.pending` è `null` e l'`Approve` è partito), è il nodo di `reka-ui` che se ne va **dopo**. Misurato con una sonda mia (`lingers.test.ts` in `probe-R13`, poi cancellata): `after 1 nextTick: present state=closed` · `after 2 nextTick: present state=closed` · `after 3 nextTick: GONE` | CONFERMATO | fatto | **sì** | nel Passo 13, dopo `(yes as HTMLButtonElement).click();` servono **tre** `await nextTick();` invece di uno. Misurato: con tre l'`it` passa. `reka-ui` 2.10.4 smonta `DialogContent` attraverso il proprio strato dismissable, che costa due giri in più |
| R13-3 | **14**, Passo 13 (`gui/src/panels/modules.test.ts`), l'`it` *«sends nothing on a no, and closes»* — `(no as HTMLButtonElement).click();` seguito da **un solo** `await nextTick();` e poi `expect(document.querySelector(".confirm")).toBeNull();` | idem, sul «Rifiuta» | stesso errore e stesso reso (`data-state="closed"`), stessa misura: il nodo sparisce al **terzo** `nextTick` | CONFERMATO | fatto | **sì** | identico a R13-2: tre `await nextTick();` dopo il click. ⚠️ **È lo stesso difetto in due `it`** e si corregge nello stesso Passo; con R13-1 e questi due la suite intera va verde — `Test Files 14 passed \| 1 skipped`, `Tests 81 passed \| 1 skipped` |

## Che cosa ho compilato, e come

| | |
|---|---|
| Cartella | `C:\Users\zagor\AppData\Local\Temp\probe-R13\gui\` (percorso **corto**: `npm` e `vite` inciampano sullo scratchpad, che supera MAX_PATH) |
| Da dove | copia di `C:\Users\zagor\AppData\Local\Temp\probe-R6\gui\` per i soli `node_modules` (l'originale resta intatto); poi `rm -rf src schema dist` e **ogni** file riscritto dal piano di **oggi** |
| Come ho estratto i file | `extract.py` in `probe-R13/`: per ogni recinzione ` ```ts/vue/json/css/html ` risale fino a dieci righe cercando un percorso `` `gui/…` `` — 13 blocchi dal compito 11, 31 dal 13, 27 dal 14; `assemble.py` li scrive nell'ordine **11 → 13 → 14** (il `Frame.vue`, il `BigTab.ts`, il `main.ts` e l'`App.vue` del 14 **sostituiscono** quelli del 13) e applica le toppe (le `dependencies` del 13 e del 14 nel manifesto, i tre innesti in `vite.config.ts`, `"types": ["vite/client", "node"]`, `--stop` e la coda di `tokens.css`, le chiavi del 14 in `it.json`) |
| Prova che i sorgenti sono di **oggi** | `grep -c -F 'INCOMPLETE every time' src/tokens/tokens.css` → **1**; `grep -c 'on the mounted components' src/tokens/tokens.css` → **0** — i due comandi che il Passo 3 del 14 detta. Il modello di `probe-R6` rendeva l'opposto: i suoi sorgenti sono stati **cancellati**, non riusati |
| `.npmrc` | **tolto**, e non è un rilievo: `engine-strict=true` più `engines.node` `^22.22.2 \|\| ^24.15.0 \|\| >=26.0.0` contro `node` **v24.9.0** dà `npm error code EBADENGINE`. È il comportamento che **P-64** vuole; il mandato lo prevede |
| Le fixture | `gui/schema/fixtures/` **non esiste** nel repo (il 3 non è eseguito). Le ho generate io con `fixtures.py`, dai valori **veri** di `stamp_set()` del Passo 5 del 3 resi come li rende `variant_json` del Passo 6: **14** `.json`, 14 `.bin` segnaposto, e `ipc_v1.map` che finisce con `stamp 0x0123456789abcdef`. ⚠️ **Il timbro è una forma, non il valore**: è FNV-1a sulle codifiche `bincode`, che solo Rust produce — di qua nessuno lo ricalcola, `connection.ts` lo spedisce e basta |
| Le tre viste | `REGENERATE_VIEWS=1 npx vitest run src/panels/views/generate-views.test.ts` → **1 passed**, e scrive `home.json`, `work.json`, `compact.json`. Senza di esse `vue-tsc` dà tre `TS2307` — cioè il generatore del 13 **funziona**, `dockview` **monta sotto `jsdom`** (la finta di `ResizeObserver` di `jsdom-setup.ts` basta) e la `setActivePinia` di R6-2 serve davvero |
| Comandi | `npm install --no-audit --no-fund` → `added 33 packages in 3s` · `npx vue-tsc --noEmit` → **EXIT=0**, **zero** righe · `npx vitest run` → **EXIT=1**, `Tests 9 failed` con le mie prime fixture, **`3 failed`** con quelle vere, **`0 failed`** coi due rimedi |
| Versioni installate (`npm ls --depth=0`) | `vue@3.5.42` `vite@8.3.0` `@vitejs/plugin-vue@6.0.9` `typescript@5.9.3` `vue-tsc@3.3.11` `vitest@4.1.11` `jsdom@30.0.1` `@vue/test-utils@2.5.0` `@types/node@24.13.5` `dockview@8.3.1` `dockview-core@8.3.1` `pinia@4.0.3` `reka-ui@2.10.4` `vue-i18n@11.4.10` **`markdown-it@15.0.2`** **`axe-core@4.13.0`** — tutte e sedici alle versioni **appuntate** dal piano |
| `node` / `npm` / `cargo` / `rustfmt` | v24.9.0 · 11.6.0 · 1.95.0 (f2d3ce0bd 2026-03-21) · 1.9.0-stable |

⚠️ **Sei `it` sono falliti nella prima corsa per colpa delle MIE fixture**, non del piano, e li registro perché
nessuno li ricerchi: avevo inventato i valori invece di leggerli. Rifatte dal `stamp_set()` vero del compito 3
(`Policy` 12288/16384, `Token` `Untrusted`, `Triple` `arbiter × policy × Write`, `Steps` passo **42**
`arbiter.set_policy`, `Verdict::Refused` 4096/1024, `Call` `arbiter.set_policy`/`local`) sono passati tutti e
sei. ✅ **È anche una misura a favore del 14:** le sue sonde leggono davvero i valori delle fixture e non
asseriscono due letterali scritti da sé.

---

## Compito 12 — il modulo delle sonde di `gui/fake-core/src/main.rs`

**Esito: nessun rilievo.** `main.rs` ricomposto dai quattro blocchi Rust dei Passi 5, 6, 7 e 8 (**655** righe) si
**parsa** — `rustfmt --check --edition 2024` rende sette `Diff in`, che sono **riavvolgimenti di riga** e non errori
di sintassi; un file che non si parsa fa uscire `rustfmt` con un messaggio di parsing, non con un diff. Ogni nome
che il modulo delle sonde usa torna coi blocchi *Interfaces* dei compiti 1, 2, 3, 5, 7 e 9 e coi tipi di **oggi**.

### Che cosa ho compilato davvero

Una crate usa-e-getta in `C:\Users\zagor\AppData\Local\Temp\probe-R13-rust\`, contro `kernel`, `platform` e
`simulator` di `e851b5d` **per percorso**, con `CARGO_TARGET_DIR=C:\Users\zagor\AppData\Local\Temp\probe-R13-target`.
`cargo run` → **EXIT=0**, zero avvisi. Dentro, **copiati alla lettera** dal piano:

| Pezzo | Resa |
|---|---|
| le costanti del Passo 5 — `TOTAL_VRAM`, `AUDIO_QUOTA`, `PRESENTATION_QUOTA`, `ARBITER_ID`, `FOR_EVER`, `GUI_TICK`, `AUDIO_RESERVATION`, `PRESENTATION_RESERVATION` | compilano; `Mib::new`, `ArbiterId::new`, `Millis::new` sono `const fn` oggi e i campi di `ResourceProfile` sono pubblici |
| `build_the_arbiter` | compila **e gira**: `arbiter.allocated()` rende **`Mib(1792)`** = 1024 + 768 — ⛔ **esattamente il letterale `Mib::new(1_024 + 768)` che la sonda `the_welcome_gives_the_sequence_one` asserisce** (R5-7). Il `match` su `Admission::{Granted, Queued, Refused}` è esaustivo contro l'enum di oggi |
| `SharedClock` e il suo `impl Reactor` | compila **e gira** su `SystemReactor`: le tre firme — `now(&self) -> Monotonic`, `wall_time(&self) -> WallTime`, `wait_until(&mut self, Monotonic) -> Option<Monotonic>` — combaciano riga per riga col tratto di `crates/kernel/src/ports/reactor.rs` |
| `note_a_degraded_routing` | compila **e gira** su `MemoryJournal`: `RecordV1::intent(effect, trust, payload, reason)`, `RecordV1::routing(effect, trust, payload, reason, detail)` e `RoutingDetail::new("fake-core", 1, true)` esistono oggi con quelle firme |
| ⛔ **`a_peer_that_says` — «il pari sul tubo» — VERBATIM**, contro `interprocess` **2.4.4** (la versione che il compito 2 appunta) e l'`IpcMessage` di oggi, con `take_frame` stubbato **con la firma esatta** del blocco *Interfaces* del 2 | compila, **zero avvisi**: `name.to_ns_name::<GenericNamespaced>()`, `Stream::connect(ns.clone())`, `prelude::*`, `write_all`, `read`, `buffer.drain(..next)` e il flusso di prestiti del ciclo reggono tutti |

⚠️ **`buffer.drain(..next)` senza legare il risultato NON dà un avviso `unused_must_use`** su `rustc` 1.95.0 —
provato di proposito, perché sarebbe stato un rosso silenzioso.

### I nomi, uno per uno

| Nome | Da dove | Verificato contro | Esito |
|---|---|---|---|
| `kernel::framing::take_frame(bytes: &[u8]) -> Option<(&[u8], usize)>` | 2 | `while let Some((_, next)) = …` destruttura una coppia e usa il secondo come `usize` | ✓ |
| `IpcMessage::decode` / `encode` | oggi | `crates/kernel/src/wire/ipc.rs`, righe 173 e 191 | ✓ |
| `IpcMessage` derive `Debug, Clone, PartialEq, Eq` | oggi | serve agli `assert_eq!(heard, vec![…])` | ✓ |
| `kernel::wire::ipc::{build_stamp, stamp_set, BuildStamp}` | 3 | `a_stamp_that_is_not_ours` pesca `IpcMessage::StaleBuild(stamp)` da `stamp_set()`, che il Passo 5 del 3 riempie con `BuildStamp(0xFEDC_BA98_7654_3210)` | ✓ |
| `kernel::wire::ipc::Call { function: String, argument: String }` | 3 | campi **pubblici**, `String` e non `&'static str`: `"set-policy".to_string()` compila | ✓ |
| `PolicyReport` — `report.allocated`, `report.total`, e il `*report` che lo **copia** fuori dal messaggio | 3 | `#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]` con `pub policy`, `pub allocated`, `pub total` | ✓ **serve `Copy`, e c'è** |
| `Provenance::Untrusted` e `*provenance == Provenance::Untrusted` | 3 | `#[derive(… Copy, PartialEq …)]` | ✓ |
| `Verdict::{Granted, Queued, Refused { asked, ceiling }}` | **oggi** | `Admission::Refused { asked: Mib, ceiling: Mib }` → `Verdict::Refused { asked: Mib, ceiling: Mib }`: i due campi hanno **lo stesso tipo**, la conversione del rubinetto compila | ✓ |
| `Parameters` passato **tre volte** in `run_the_graph` (a `build_the_arbiter`, a `Core::new`, a `Executor::new`) | oggi | `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` — è `Copy`, quindi **nessun uso-dopo-spostamento** | ✓ |
| `Executor::new(rng, reactor, parameters, sleep)` · `Sleep::new()` · `Monotonic::saturating_add(self, Millis) -> Self` | oggi | firme lette nel sorgente | ✓ |
| `WELCOME: usize = 5` | 12, «rilettura sul dispaccio del 7» | `greet` del 7 manda `Accepted`, `Degradation`, `Policy`, `Layout` **più** `Steps` = **5**; e il compito **9** porta `const WELCOME: usize = 5;` | ✓ **le tre case coincidono** |
| `SOCKET_NAME` e `MAX_BODY` | 9 (**D45**, **D31**) | `grep -o` sui due testi: `"harness-core"` e `1024 * 1024` in **entrambi** — il `diff` del Passo 10 uscirebbe **EXIT=0** | ✓ |
| gli `use` del Passo 5 contro *Interfaces* | R5-14 | `kernel::record::{EffectClass, Record, RecordV1, RoutingDetail, Trust}`: **nessun `Grant`, nessun `Detail`** | ✓ **R5-14 regge** |
| `build_stamp` importato **dentro** `mod tests` | R5-12 | `awk 'NR<318'` sul modello → **nessuna** occorrenza fuori dalle sonde | ✓ **R5-12 regge** |

### I criteri di chiusura meccanici, rilanciati sul modello

| Criterio | Comando → resa | Esito |
|---|---|---|
| nessun ramo di dispaccio nella metà **senza** le sonde | `awk '/#\[cfg\(test\)\]/{exit} {print}' <modello> \| grep -cE 'IpcMessage::(Hello\|Invoke\|Approve\|SaveLayout) *(\(\|=>)'` → **0** | ✓ |
| ⛔ **la seconda direzione, che è ciò che prova il comando** (R5-5) | lo stesso `grep -cE` sul **file intero** → **7** | ✓ **più di zero: il comando morde dove deve** |
| `grep -c 'struct SharedClock' <modello>` → 1 | **1** | ✓ |
| `grep -rl 'struct SharedClock' crates/ --include='*.rs' \| wc -l` → 4 | **1** oggi (`crates/simulator/tests/arbiter_campaign.rs`); i tre che mancano li portano i compiti **7**, **9** e **10**, tutti prima del 12 | ✓ coerente |
| nessuna sonda col corpo vuoto | `grep -cE '^\s*fn [a-z_]+\(\) \{\}$' <modello>` → **0** | ✓ |

⚠️ **Una nota misurata, non un rilievo:** `rustfmt --check --edition 2024` sul modello rende **7** `Diff in` —
`to_ns_name` su tre righe, due `assert_eq!` spezzati, una chiusura su più righe. ⛔ **Nessun cancello di questo
repository lancia `cargo fmt`**: `grep -rn 'cargo fmt\|rustfmt' scripts/ .github/` non rende **niente**, quindi non
sarebbe un rosso. Lo scrivo perché il coordinatore non lo ricerchi.

## Copertura del disegno per il mio perimetro

Vuota per costruzione: il perimetro è la **compilazione**, non la copertura (il mandato lo consente esplicitamente).
I compiti 12 e 14 sono già stati rivisti da R5 e R7.

## Voci P rimisurate

Vuota per costruzione, stessa ragione. Le tre P che ho **incontrato** compilando sono confermate di passaggio:
**P-64** (`.npmrc` più `engines.node` rifiutano `node` v24.9.0 — l'ho visto con `npm error code EBADENGINE`),
**P-86** (il contrasto di `--stop`: `contrast.test.ts` passa col `#ec5f57` del Passo 3 del 14) e **R6-8** (la finta di
`ResizeObserver` di `jsdom-setup.ts` basta a far montare `dockview` sotto `jsdom`).

## Numeri di compito ricensiti

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| 12, Passo 8, *«pinned by task 9's `WELCOME` with the same reading»* | 9 | 9 (il daemon) | ✓ |
| 12, Passo 8, *«MEASURED ON THE DISPATCH OF TASK 7, `greet` in `crates/kernel/src/serving.rs`»* | 7 | 7 (`kernel::serving`) | ✓ |
| 12, Passo 5, *«the other four are … the sub-project 2 campaign (task 10) and `crates/daemon/src/main.rs`»* | 10 e 9 | 10 (la campagna DST) e 9 (il daemon) | ✓ |
| 12, Passo 6, *«task 7, and its probe `an_unchanged_degradation_is_not_resent`»* | 7 | 7 | ✓ |
| 14, Passo 4, *«le chiavi qui sotto entrano accanto a quelle del 13»* | 13 | 13 (la cornice) | ✓ nessuna chiave in comune fra i due blocchi `it.json`, misurato: `overlapping=none` |
| 14, *Interfaces*, *«Consumes, dal compito 13: … `BigTab`»* | 13 | 13 | ✓ il `BigTab.ts` del 14 **riscrive** quello del 13 e `bigtab.test.ts` passa |

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

```
git rev-parse --short HEAD                                              -> e851b5d
wc -l <piano>                                                           -> 23609
awk '/^## Compito 12:/{f=1} /^## Compito 13:/{exit} f' <piano> | wc -l   -> 1016
awk '/^## Compito 13:/{f=1} /^## Compito 14:/{exit} f' <piano> | wc -l   -> 2208
awk '/^## Compito 14:/{f=1} /^## Compito 15:/{exit} f' <piano> | wc -l   -> 2286
grep -c '^### P-' / '^| **D[0-9]' / '^## Compito' <piano>               -> 133 / 91 / 17
cp -r probe-R6/gui probe-R13/gui ; rm -rf probe-R13/gui/{src,schema,dist}
python extract.py c11.md b11 / c13.md b13 / c14.md b14                  -> 13 / 31 / 27 blocchi
python assemble.py                                                      -> 62 file scritti
grep -c -F 'INCOMPLETE every time' src/tokens/tokens.css                -> 1   (Passo 3 del 14)
grep -c 'on the mounted components' src/tokens/tokens.css               -> 0   (Passo 3 del 14)
(con .npmrc) npm install --no-audit --no-fund                           -> npm error code EBADENGINE  (P-64)
(senza .npmrc) npm install --no-audit --no-fund                         -> added 33 packages in 3s
npx vue-tsc --noEmit                                                    -> EXIT=2, tre TS2307 (le viste non generate)
REGENERATE_VIEWS=1 npx vitest run src/panels/views/generate-views.test.ts -> 1 passed; home/work/compact.json scritti
npx vue-tsc --noEmit                                                    -> EXIT=0, ZERO righe
python fixtures.py  (valori inventati da me)                            -> 14 json
npx vitest run                                                          -> EXIT=1, 9 failed  (sei per le MIE fixture)
python fixtures.py  (dallo stamp_set() vero del compito 3)              -> 14 json
npx vitest run                                                          -> EXIT=1, 3 failed  <-- I RILIEVI
npx vitest run src/probe/lingers.test.ts --reporter=verbose             -> 1/2 nextTick: present state=closed; 3: GONE
(coi due rimedi) npx vitest run                                         -> EXIT=0, 81 passed | 1 skipped
npm ls --depth=0                                                        -> le sedici versioni appuntate
awk NR>=213 / 343 / 478 / 557 su c12.md > model-fake-main.rs            -> 655 righe
rustfmt --check --edition 2024 model-fake-main.rs                       -> EXIT=1, 7 "Diff in" (riavvolgimenti)
grep -rn 'cargo fmt|rustfmt' scripts/ .github/                          -> niente
cargo run  (crate usa-e-getta, kernel/platform/simulator per percorso)   -> EXIT=0; allocated = Mib(1792)
cargo build  (col pari VERBATIM su interprocess 2.4.4)                  -> EXIT=0, zero avvisi
awk '/#[cfg(test)]/{exit}{print}' <modello> | grep -cE 'IpcMessage::(Hello|Invoke|Approve|SaveLayout) *(\(|=>)'  -> 0
grep -cE 'IpcMessage::(Hello|Invoke|Approve|SaveLayout) *(\(|=>)' <modello>  -> 7
grep -c 'struct SharedClock' <modello>                                  -> 1
grep -rl 'struct SharedClock' crates/ --include='*.rs' | wc -l          -> 1 (oggi; i 7, 9, 10 portano gli altri tre)
grep -cE '^\s*fn [a-z_]+\(\) \{\}$' <modello>                           -> 0
grep -o 'const SOCKET_NAME: &str = "[^"]*"' c09.md / <modello>          -> "harness-core" in entrambi
grep -o 'const MAX_BODY: usize = [^;]*' c09.md / <modello>              -> 1024 * 1024 in entrambi
grep -n 'WELCOME' c09.md                                                -> const WELCOME: usize = 5;
grep -n 'fn greet' -A 45 c07.md                                         -> Accepted, Degradation, Policy, Layout + Steps = 5
git status --porcelain                                                  -> (vuoto)
git ls-files --eol <piano> <disegno2> <stella>                          -> i/lf w/lf tutti e tre
```

## Non verificato, e perché

| Che cosa | Perché |
|---|---|
| ⛔ **`npx eslint .`, la catena del compito 15** | non ricostruita: `eslint.config.js`, il preset e le quattro regole nostre sono un compito intero, fuori dal mio perimetro — e **R8 l'ha già installata e misurata** il 2026-09-16 (R8-1, R8-2). Ricostruirla qui avrebbe rimisurato il suo lavoro invece del mio |
| il **timbro** vero in `ipc_v1.map` | è FNV-1a sulle codifiche `bincode` del `stamp_set()`, che solo il `kernel` in Rust produce, e il compito 3 non è eseguito. Ne ho messo uno della **forma** giusta (`stamp 0x0123456789abcdef`): di qua nessuno lo ricalcola — `connection.ts` lo spedisce e basta, e nessuna sonda lo confronta con un secondo valore |
| i `.bin` delle fixture | segnaposto di un byte: **nessun file di `gui/src/` li legge** — `fixtures.ts` fa `import.meta.glob("../../schema/fixtures/*.json")`, i `.json` soltanto (**D36**: di qua nessuno decodifica byte) |
| il **12 compilato per intero** | impossibile e dichiarato tale dal mandato: dipende da `kernel::serving` (7), `kernel::numbering` (1), `platform::ipc` (2), `kernel::ports::custody` + `simulator::custody` (4 e 5), le varianti nuove di `wire::ipc` (3) e il `Parameters::new` a **quattro** argomenti (7) — nessuno dei quali esiste a `e851b5d`. Ciò che si poteva compilare l'ho compilato, ed è la tabella qui sopra |
| la SPA **aperta nel browser** | la regola 5 della testa del 13 e del 14 la chiede a chi **esegue**; io sono in sola lettura e il mio perimetro è la compilazione. `npm run dev` non è stato lanciato |

## Stato finale

`git -C /c/Users/zagor/Desktop/harness status --porcelain` → **(vuoto)** · `HEAD` = `e851b5d`, invariato ·
`git ls-files --eol` di piano, disegno del 2 e stella polare → `i/lf w/lf` tutti e tre, **invariato**.
Nessun file del repository è stato creato, toccato o cancellato: ogni prova è vissuta in
`<scratchpad>/review/probe-R13/`, `C:\Users\zagor\AppData\Local\Temp\probe-R13\`,
`C:\Users\zagor\AppData\Local\Temp\probe-R13-rust\` e `…\probe-R13-target\`.
⚠️ `C:\Users\zagor\AppData\Local\Temp\probe-R6\gui\` è stato **letto e copiato**, mai modificato.
