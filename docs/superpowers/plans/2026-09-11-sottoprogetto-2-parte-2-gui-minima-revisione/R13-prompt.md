# R13 — I moduli riscritti dalla revisione e mai compilati: le sonde del 12 e i moduli del 14

Rapporto: `<scratchpad>/review/R13-report.md`. Vincoli: `constraints.md` **col suo richiamo del 2026-09-16 in testa** (letto prima di
questo). Data del rapporto: 2026-09-16.

## Perimetro — NON una revisione intera: una prova di compilazione ed esecuzione, con ciò che ne esce
- Del **compito 12** (`awk '/^## Compito 12:/{f=1} /^## Compito 13:/{exit} f' <piano>`): il modulo `mod tests` di `gui/fake-core/src/main.rs`,
  riscritto dall'ondata 9 (registro `ledger.md`, sezione «Compito 12»: R5-2, R5-3, R5-4, R5-5, R5-6, R5-7, R5-12 e la «coerenza col 9
  corretto»), e il `main.rs` che lo ospita.
- Del **compito 14** (`awk '/^## Compito 14:/{f=1} /^## Compito 15:/{exit} f' <piano>`): `Frame.vue` (ridettato al Passo 10 dopo D89 del 13),
  `Chat.vue`, `markdown.ts`, `modules.test.ts`, `chat.test.ts` — riscritti dall'ondata 11 (registro, sezione «Compito 14»: R7-1, R7-2, R7-10
  e la cascata di D80/D89) — e con essi tutto ciò che il 14 detta e le sonde importano.
- I compiti 12 e 14 sono GIÀ rivisti (R5, R7): **non rifare la loro revisione**. I rilievi sono ciò che la compilazione e l'esecuzione delle
  sonde mostrano, più ciò che ti smentisce leggendo per compilare (un nome che il 13 non esporta, una firma diversa dal blocco *Interfaces*,
  un import che non esiste).

## Come si compila
- **Il 14** (TypeScript, Vue). In `<scratchpad>/review/probe-R13/gui/` (o in una cartella corta `C:\Users\zagor\AppData\Local\Temp\probe-R13\gui\`
  se `npm` o `vite` inciampano sulla lunghezza del percorso: dillo) ricostruisci il modello dell'**11** dal piano di **oggi** — i file che il
  compito 11 detta: `package.json`, `.npmrc`, `tsconfig.json`, `vite.config.ts`, `index.html`, `src/schema/*`, `src/transport/*`,
  `schema/fixtures/*.json` — poi `npm ci` col lockfile di `<scratchpad>/review/model11-lock/` (copia `package-lock.json` accanto al tuo
  `package.json`: i pacchetti vengono dalla cache, in secondi; se `npm` rifiuta per `engines.node` — `node` è v24.9.0, P-64/P-65 lo sanno —
  togli `.npmrc` dal modello e dillo, non è un rilievo), poi `npm install` dei pacchetti che il Passo 3 del **13** e i Passi del **14**
  aggiungono, alle versioni dettate; poi TUTTI i file del 13 (`awk` sul compito 13, blocco per blocco; `probe-R5/gui-ts5/` della sessione
  vecchia mostra la forma di un modello dell'11, non usare i suoi sorgenti come oracolo), poi quelli del 14 (il `Frame.vue` del 14
  **sostituisce** quello del 13). Poi `npx vue-tsc --noEmit`, `npx vitest run`, e `npx eslint .` solo se la configurazione del 15 è
  ricostruibile in pochi minuti (altrimenti no, e dillo). Ogni rosso: il messaggio **intero**, il file e il Passo che lo detta, verdetto
  `CONFERMATO`, specie `fatto`, `Blocca? sì`. Le fixture `gui/schema/fixtures/*.json` non esistono nel repo (il 3 non è eseguito): il
  generatore del 3 le descrive (scheletro, *Interfaces* del 3; il Passo del 3 che scrive i `.json`): alimenta le sonde che le leggono con
  una forma coerente con quel Passo e **dillo**. Le sonde che montano `dockview` sotto `jsdom`: il Passo 3 del 13 dice che cosa aspettarsi;
  se `dockview` non monta sotto `jsdom`, è un rilievo sul 13 e sul 14 insieme, con l'errore intero.
- **Il 12** (Rust). `gui/fake-core` dipende da codice che non esiste nel repo (`kernel::serving` del 7, `platform::ipc` del 2, le varianti
  del 3): compilare per intero è impossibile. Fai ciò che R5 ha fatto: ricomponi `main.rs` dai blocchi Rust del 12
  (`<scratchpad>/review/probe-R13/model-fake-main.rs`), passalo a `rustfmt --check --edition 2024` (la sintassi), e confronta OGNI nome che il
  modulo delle sonde usa — `SharedClock`, `serve`, `the_faucet`, `take_frame`, `build_stamp`, `stamp_set`, `BuildStamp`, `Call`, `Parameters`,
  `Reactor`, `WallTime`, `Verdict`, `IpcMessage::decode`, `WELCOME`, `WITH_A_PEER`, `Until`, `a_keyboard`, … — coi blocchi *Interfaces* dei
  compiti 2, 3 e 7 nello scheletro e col codice dettato di quei compiti (`grep -n` sul piano): nome, firma, argomenti, tipo di ritorno,
  `&` e `&mut`, e i tipi di **oggi** dove esistono (`grep -rn` in `crates/`). Se un pezzo si può compilare in una crate usa-e-getta contro
  `kernel`, `platform` e `simulator` di oggi per percorso — la `SharedClock` sul `VirtualReactor`, il pari sul tubo — fallo, con
  `CARGO_TARGET_DIR=C:\Users\zagor\AppData\Local\Temp\probe-R13-target` (R5 lo fece in `probe-R5/e0716/`). Un nome che non torna: rilievo
  `fatto`, `sì`.

## Il rapporto
Nel formato di `constraints.md` §5, con in più una tabella «Che cosa ho compilato, e come» (cartella, comandi, versioni installate:
`npm ls --depth=0`, `cargo --version`, `node --version`). Le sezioni «Copertura del disegno» e «Voci P rimisurate» possono restare vuote:
il perimetro è la compilazione. «Numeri di compito ricensiti»: solo quelli che incontri compilando.
