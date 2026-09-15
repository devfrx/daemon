# Rapporto R10 — Coerenza FRA i compiti: *Interfaces* ↔ consumatori, *Files*, numeri di compito, conteggi della testa e del diario — 2026-09-15

Repository a `baf3cde`, albero pulito prima e dopo; `bash scripts/gate.sh` → `GATE GREEN` (log in `probe-R10/gate-R10.log`), `bash scripts/check-docs.sh` → `OK — no inconsistencies.`. Sola lettura: nessun file del repository toccato; le prove in `probe-R10/`.

## Esito in tre righe

1. **Due rilievi bloccanti, stessa specie:** i compiti **15** e **16** etichettano **LF** tre file esistenti che oggi sono **`i/lf w/crlf`** (`scripts/gate.sh`, `.github/workflows/quality-gate.yml`, `docs/audit-2026-08-27.md`), lo scrivono nell'**Atteso** del loro Passo 1, e inseriscono righe **LF** con Python dentro un file CRLF (o lo sovrascrivono LF con `cp`): eseguiti come sono scritti, il Passo 1 smentisce l'Atteso e il criterio «fine-riga invariati» va rosso — mentre il compito **10** dice di `gate.sh` la cosa giusta (`i/lf w/crlf`, `replace_unique.py`).
2. **Il contratto e i numeri:** il blocco *Interfaces* del compito 11 dice ancora `interface Fixture { file; kind; value }` che **P-75** dichiara corretto in tre posti e che il Passo 9 detta come `{ file; message: IpcMessage }` — la correzione non è mai stata applicata; quattro numeri di compito sono rimasti a **prima di D25** (riga 7 della posizione «al 8», D2 «compito 12», D4 «al compito 10», D12 «dopo il 10»); il blocco *Files* del 14 non nomina la stella polare che il suo Passo 15 modifica, quello del 13 non nomina `generate-views.test.ts` che detta.
3. **I conteggi reggono tutti** (17/17, 116 P, 74 D, errata 0, CR 0, tabelle intere, margine 11030, numerazione 1–67 continua, 55–67 ↔ D63–D74 riga per riga), con due sonde della chiusura che non reggono **se copiate dalla cella**: `'^\| \*\*D[0-9]'` rende **20129** in BRE, e `(it\|describe)` in ERE rende **0** anche dove deve rendere 1; il `grep -n '<the version'` rende **5** righe perché il diario cita sé stesso.

## Rilievi

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R10-1 | Compito 15 · *Files* «`scripts/gate.sh` (**LF**) — una riga `run`» e «`.github/workflows/quality-gate.yml` (**LF**)»; Passo 1, Atteso «`scripts/gate.sh` e il flusso di lavoro **LF**»; Passo 8, Atteso «**zero** CR; il diff dice **una riga aggiunta**»; Passo 9, Atteso «**zero** CR prima e dopo» | i due file esistenti sono LF; l'inserimento Python `text.replace(anchor, line + anchor, 1)` con `line = '… bash scripts/gate-gui.sh\n'` e `block` LF lascia zero CR | `git ls-files --eol scripts/gate.sh .github/workflows/quality-gate.yml` → **`i/lf w/crlf`** entrambi; `tr -cd '\r' < scripts/gate.sh ¦ wc -c` → **97** = `wc -l` **97**; il flusso → **16** = **16**. Il compito 10 sullo stesso file: *«In `scripts/gate.sh` (**`i/lf w/crlf`**, quindi `replace_unique.py`)»* e il suo Atteso del Passo 1 *«`gate.sh` è `i/lf w/crlf`»*. Con l'inserimento dettato una riga LF finisce fra 97 righe CRLF (`w/mixed`), e l'ultima casella del criterio *«i fine-riga sono invariati … uguale al Passo 1»* non può reggere | CONFERMATO | fatto | sì | *Files*: le due etichette → **`i/lf w/crlf`**; Passo 1, Atteso → *«`scripts/gate.sh` e il flusso di lavoro `i/lf w/crlf`, come il compito 10 li lascia»*; Passo 8: inserire con `replace_unique.py` (converte a CRLF) oppure `if "\r\n" in text: line = line.replace("\n", "\r\n")`, Atteso *«CR = righe»* (97 + 1) invece di «zero CR»; Passo 9: idem sul `block` di `/tmp/setup-node.yml`, Atteso CR = righe |
| R10-2 | Compito 16 · *Files* «`scripts/gate.sh` (**LF**)», «`scripts/gate-gui.sh` (**LF**)», «`.github/workflows/quality-gate.yml` (**LF**)», «`docs/audit-2026-08-27.md` (**LF**)»; Passo 1, Atteso «tutti **LF**»; Passo 2, `text.replace(anchor, block + "\n" + anchor, 1)`, Atteso «CR» a zero; Passo con `cp /tmp/quality-gate.yml .github/workflows/quality-gate.yml`, Atteso «**zero** CR le due volte» | i quattro file sono LF | `git ls-files --eol scripts/gate.sh .github/workflows/quality-gate.yml docs/audit-2026-08-27.md` → **`i/lf w/crlf`** tutti e tre (`gate-gui.sh` nascerà LF al 15, unico vero LF); `tr -cd '\r' < docs/audit-2026-08-27.md ¦ wc -c` → **1885** = `wc -l` **1885**. ⛔ **Il compito si contraddice da solo:** nel passo dell'audit scrive *«Il file dell'audit È CRLF nell'albero di lavoro — `i/lf w/crlf`, misurato il 2026-09-15»* — vero — mentre il suo *Files* e il suo Passo 1 dicono LF. Il `cp` di un file scritto LF sopra un `w/crlf` lo fa passare a `w/lf`: il criterio *«fine-riga invariati rispetto al Passo 1»* non regge | CONFERMATO | fatto | sì | *Files*: `gate.sh`, il flusso e l'audit → **`i/lf w/crlf`**, solo `gate-gui.sh` **LF**; Passo 1, Atteso → *«`gate.sh`, il flusso e l'audit `i/lf w/crlf`; `gate-gui.sh` `i/lf w/lf`»*; Passo 2: blocco convertito a CRLF se `"\r\n" in text`, Atteso CR = righe; il passo del flusso: scrivere `/tmp/quality-gate.yml` con i fine-riga del file (`newline=""` più `.replace("\n","\r\n")`) prima del `cp`, Atteso CR = righe le due volte |
| R10-3 | Testa · tabella della posizione, riga 7: «⛔ **Il limite di giri è al 8 — richiamo del 2026-09-11, D21**» | il limite di giri sta al compito 8 | `grep -n 'Il limite di giri è al 8' <piano>` → **1** (riga 7 della posizione). Ma **D21** dice *«il limite di giri passa al **compito 9**»* e la riga **9** dice *«e il limite di giri — arrivato qui dalla riga 7»*; dal **D25** (2026-09-14) l'8 è la specie `Policy`, che non ha limite di giri. Il *Files* del compito 7 sa che la riga si è mossa (*«era la riga 8 prima della divisione di D25»*) ma la riga 7 non è stata riscritta | CONFERMATO | fatto | no | Riga 7 della posizione: «al 8» → «al **9**» (stesso richiamo, stessa D21). È la specie che D25 promette di aver corretto *«nello stesso commit»* |
| R10-4 | Tabella D · **D2**, «l'evidenza delle otto mosse è sulla 8.2.0, e il **compito 12** lo scrive accanto al primo uso» | il compito 12 scrive la nota sulla 8.2.0 al primo uso di `dockview` | `awk '/^## Compito [0-9]+:/{c=$3} /8\.2\.0/{print c" L"NR}' <piano>` → la nota dettata è nel compito **14** (`gui/src/frame/moveActive.ts`: *«8.2.0: `DockviewApi.getPanel(id)` … `addFloatingGroup`»*), nessuna nel 12 né nel 13. Il 12 è il core finto, Rust: non usa `dockview`. «12» è la cornice **prima di D25** (oggi 13), e l'artefatto che porta la nota è comunque il 14 | CONFERMATO | fatto | no | D2: *«e il compito **14** lo scrive in `gui/src/frame/moveActive.ts`, accanto alle firme che le mosse esercitano»* (o «13» se si vuole la nota in `dock.ts`, ma allora va aggiunta al Passo del 13) |
| R10-5 | Tabella D · **D4**, «⛔ **La misura si rifà al compito 10**, e se il piano si scrivesse fra un mese…» | la rimisura di `vitest` spetta al compito 10 | Contro la posizione il 10 è la campagna DST (Rust, nessun `npm`); chi installa è l'11 — e **la stessa riga lo dice più avanti**: *«chi installa è il compito **11**, e li rilancia quel giorno»*. La prima frase è del 2026-09-11, prima di D25 | CONFERMATO | fatto | no | D4: «al compito 10» → «al compito **11**» (la frase *«RIMISURATA IL 2026-09-14, scrivendo il compito 10»* resta: è un fatto di sessione) |
| R10-6 | Tabella D · **D12**, «spostare il compito 3 dopo **il 10** romperebbe il taglio per artefatto (D1) e lascerebbe lo schema senza controllo per **sette** compiti» | l'alternativa scartata era mettere il 3 dopo il compito che fa nascere `gui/` (allora il 10) | La stessa riga dice *«che però nasce al **compito 11** (P-18)»*: «dopo il 10» è il numero pre-D25, e i compiti fra il 3 e l'11 sono **otto** (`seq 4 11 ¦ wc -l` → 8) | CONFERMATO | fatto | no | D12: «dopo il 10» → «dopo l'**11**», «sette compiti» → «otto compiti» (o togliere il numerale, vincolo globale 3) |
| R10-7 | Compito 11 · *Interfaces*, «`gui/src/schema/fixtures.ts` — `loadFixtures(): Fixture[]` e `interface Fixture { file: string; kind: string; value: unknown }`»; **P-75** «⛔ **Corretto nel compito 11 … Il blocco *Interfaces* prende la forma del Passo 9**»; nona chiusura «Diceva `interface Fixture { file: string; kind: string; value: unknown }`»; compito 13, Read: «il blocco *Interfaces* del compito **11** — ⛔ **come sta ADESSO, P-75 l'ha corretto**» | il blocco *Interfaces* dell'11 è stato corretto alla forma del Passo 9 | `grep -n 'interface Fixture' <piano>` → **11979** (blocco *Interfaces* dell'11) dice ancora `{ file: string; kind: string; value: unknown }`; **12559** (Passo 9, codice dettato) dice `export interface Fixture { file: string; message: IpcMessage; }`; il Passo 12 usa `fixture.message.kind` e `emit(fixture.message)`. La correzione che P-75, la nona chiusura e la lista di lettura del 13 danno per fatta **non è nel file**. Nessun consumatore fuori dall'11 legge `Fixture` (`loadFixtures` è usata solo dentro l'11: `calls.py`), quindi non è un rosso di compilazione — è un blocco-contratto falso e una P che dice il falso | CONFERMATO | fatto | no | Riga del blocco *Interfaces* dell'11: `interface Fixture { file: string; kind: string; value: unknown }` → `interface Fixture { file: string; message: IpcMessage }`; in P-75 aggiungere il richiamo datato *«applicata il <data> in revisione: la riga era rimasta com'era»* |
| R10-8 | Compito 14 · *Files* (Modify: `gui/package.json`, `package-lock.json`, `tokens.css`, `it.json`, `BigTab.ts`, `Frame.vue`, `main.ts`; Create: …); Passo 15 «**il richiamo datato sulle righe di Passi della stella polare**» | la lista *Files* è la mappa di ciò che il compito tocca | `awk 'NR>=15415 && NR<=17518' <piano> ¦ grep -n 'direzione-gui-design'` → il Passo 15 scrive in `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (Python `newline=""`, `grep -c 'RICHIAMO DEL <data>, dal compito 14'`), e il commit fa `git add gui docs/superpowers/specs/2026-09-07-direzione-gui-design.md`; il file **non è nella lista *Files*** del 14 (`git ls-files --eol` → `i/lf w/lf`) | CONFERMATO | fatto | no | *Files* del 14: aggiungere «Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — il richiamo datato sulla riga 1 della tabella Passi (**D56**)» |
| R10-9 | Compito 13 · *Files* «Create: `gui/src/panels/views/index.ts`, `gui/src/panels/views/views.test.ts` (**LF**)»; Passo «`gui/src/panels/views/generate-views.test.ts`, **LF**:» | i file creati sono quelli della lista | `blocks.py 13` → il blocco 14660–14750 detta **`gui/src/panels/views/generate-views.test.ts`** (il generatore delle tre viste, `REGENERATE_VIEWS=1 npx vitest run …`), che il compito crea e committa (`git add gui`) e che il *Files* non nomina | CONFERMATO | fatto | no | *Files* del 13: aggiungere «Create: `gui/src/panels/views/generate-views.test.ts` (**LF**) — il generatore, saltato senza `REGENERATE_VIEWS=1`» |
| R10-10 | Compito 13 · *Interfaces* «Produces, e il compito **14** li usa con questi nomi esatti: …» (dieci voci); Compito 14 · «Consumes, dal **compito 13**: … `VueContent`, **`BigTab`**, `Frame.vue`, `createDock`» | il blocco *Produces* del 13 elenca tutto ciò che il 14 consuma | Conteggio degli `export` dei blocchi dettati del 13 (ancora di riga): `gui/src/frame/BigTab.ts` → `export class BigTab implements ITabRenderer`; `gui/src/schema/stamp.ts` → `export function buildStamp(): U64`; `gui/src/stores/connection.ts` → `export type Phase`. Nessuno dei tre è nel *Produces* del 13, e **`BigTab` è consumato e riscritto dal 14** (*Files*: «Modify: `gui/src/frame/BigTab.ts`») | CONFERMATO | fatto | no | *Produces* del 13: aggiungere «`gui/src/frame/BigTab.ts` — `class BigTab implements ITabRenderer`», «`gui/src/schema/stamp.ts` — `buildStamp(): U64` (**D52**)», «`type Phase`» accanto a `useConnection` |
| R10-11 | Compito 12 · «Consumes, dal **compito 3**: `kernel::wire::ipc::{IpcMessage, Provenance}`»; Compito 10 · «Consumes, da oggi: … `kernel::ports::journal::Journal` … `simulator::journal::CrashingJournal`» | le liste *Consumes* sono i nomi che il codice dettato importa | `uses.py` → il `main.rs` del 12 importa `use kernel::wire::ipc::{IpcMessage, Provenance, Verdict, build_stamp};` — `build_stamp` è del 3 e `Verdict` di oggi, entrambi assenti dal blocco; la campagna del 10 importa `use kernel::rng::RngExt;` (col commento che ne spiega il perché) e `use simulator::journal::{CrashingJournal, MemoryJournal};`, assenti dal blocco, e **non** importa `kernel::ports::journal::Journal` che il blocco elenca (il codice non chiama nessun metodo del tratto: `grep -nE '\.(replay¦intent¦note¦outcome)\('` sul blocco → nessuno) | CONFERMATO | fatto | no | Blocco del 12: `kernel::wire::ipc::{IpcMessage, Provenance, build_stamp}` dal 3 e `kernel::wire::ipc::Verdict` da oggi; blocco del 10: togliere `kernel::ports::journal::Journal`, aggiungere `kernel::rng::RngExt` e `simulator::journal::MemoryJournal` |
| R10-12 | Tabella D · **D29** «è `E25` — due verità indipendenti sullo stesso fatto»; **D26** «una `String` in un `Detail`, che è `E94`»; **P-49** «È **`E25` in persona**»; compito 8, prosa «(lezione `E66`, `E112`)», «con la mutazione `E79` misurata accanto»; diario «`E25` allo strato che sveglia», «`E66`, e `E112` la applicò», «la mutazione `E79`», «**E4** ha misurato» | un numero `E` nudo identifica una voce d'errata | La trappola dell'undicesima chiusura (*«UN NUMERO `D` È UNICO DENTRO IL SUO PIANO, NON FRA I PIANI … chi cita una `D` di un altro piano ne scrive anche il nome del piano»*, P-112) vale identica per le `E`: `grep -l '^¦ \*\*E25\*\* ' docs/superpowers/plans/*.md` → **5** piani (Traguardi 2, 3, 4, 5, 6), `E66` → **4**, `E79`/`E94`/`E112` → **2** ciascuno (Traguardi 5 e 6), con contenuti diversi. Le citazioni **con** il nome reggono (E2/E4/E6 «della parte 1», E12 «del Traguardo 6», E26 «del piano del Traguardo 5», E50/E51/E100 «del Traguardo 5»). ⚠️ I sorgenti di oggi usano già i numeri nudi (`grep -rlE '\bE(25¦41¦50¦66¦79¦94¦112)\b' crates/` → 12 file), quindi i **commenti di codice** dettati seguono una convenzione del repo; la **prosa** del piano no | CONFERMATO | prosa | no | Nelle righe D26, D29, P-49, nella prosa del compito 8 e nel diario: «`E25` del piano del Traguardo N» (il piano giusto lo dice la riga citata: *«due verità indipendenti»*), oppure una riga nella testa: *«una `E` nuda in questo piano è del piano del Traguardo 5 o 6, come nei sorgenti»* |
| R10-13 | Undicesima chiusura · «Segnaposto ¦ ⛔ **uno solo, DICHIARATO**: … — `grep -n '<the version' <questo file>`» | il comando trova il solo segnaposto | `grep -n '<the version' <piano>` → **5** righe: 13015 (il segnaposto) più **18846, 18974, 19125, 19264** — le quattro chiusure che citano il comando. La sonda gemella del `node -e "…"` si ferma al diario con `awk '/^## Come si riprende/{exit} …'`, questa no | CONFERMATO | fatto | no | Nella cella: `awk '/^## Come si riprende/{exit} /<the version/{c++} END{print c+0}' <questo file>` → **1**, oppure `grep -c 'version = "<the version'` → 1 |
| R10-14 | Undicesima chiusura · «`grep -c '^\| \*\*D[0-9]' <questo file>`»; decima chiusura · «`grep -cE '^\s*(it\|describe)\([^)]*\(\) => \{\}\)' <questo file>` → **0**»; «`tr -cd '\r' < <questo file> \| wc -c` → `0`» | i comandi si copiano dalla cella e rendono 74, 0 e 0 | Copiati **così come stanno nel sorgente** (dove `\|` è la convenzione della cella): `grep -c '^\| \*\*D[0-9]' <piano>` → **20129** (in BRE `\|` è alternanza: `^` prende ogni riga); `printf "  it('x', () => {})\n" ¦ grep -cE '^\s*(it\|describe)\(…'` → **0** dove deve rendere **1** (in ERE `\|` è una barra letterale: la sonda è **vacua**, verde su ogni file); la forma resa (`'^| …'`, `(it|describe)`) rende **74** e **1**/**0** come atteso; `awk '…/^\| \*\*E[0-9]/…'` regge in entrambe le forme (provato: 1 su un input con una riga `E`). Il piano non avverte della trappola da nessuna parte (`grep -n 'barra letterale\|copiat[oa] da una cella' <piano>` → niente) | CONFERMATO | fatto | no | Scrivere i comandi in una forma identica resa e grezza: `'^[|] \*\*D[0-9]'`, `'^\s*(it¦describe)…'` con la classe `[|]` al posto della barra, e la pipe della shell fuori dalla cella o in un blocco di codice; una riga fra le trappole del diario |
| R10-15 | Testa · «`grep -c '^## Compito' <questo file>` e le righe `¦ **N** ¦` della tabella qui sotto … **coincidono**» | contando le righe `¦ **N** ¦` si ottiene 17 | `grep -c '^| \*\*[0-9][0-9]*\*\* |' <piano>` su tutto il file → **19**: le due in più sono `¦ **209** ¦` e `¦ **1527** ¦` della tabella di **P-22** (righe della spec), non righe di posizione; la tabella della posizione (righe 133–149) ne ha **17** = `grep -c '^## Compito'` → 17 | NON RIPRODOTTO | fatto | no | — (se il conteggio diventa un comando, delimitarlo alla sezione della posizione con `awk`) |
| R10-16 | Compito 2 · *Files* «Modify: `crates/kernel/tests/framing.rs` — le sonde del lettore di flusso»; compito 6 · «Modify: `crates/kernel/tests/frozen/record_v1.map` — la sezione nuova»; compito 7 · «Modify: `crates/kernel/src/lib.rs`», «`crates/kernel/src/parameters.rs`», «`crates/kernel/src/executor.rs`»; compito 9 · «Modify: `Cargo.lock` — rinfrescato» | vincolo globale 4: *«la mappa dei file dice CRLF o LF **oggi**»* | Le sei voci non portano l'etichetta; `git ls-files --eol` → `i/lf w/crlf` per tutte e sei (i Passi del 7 la dicono per `executor.rs` e `lib.rs`, il *Files* no) | CONFERMATO | prosa | no | Aggiungere «(**`i/lf w/crlf`**)» alle sei voci |
| R10-17 | Compito 13 · Passo 19 «`git add gui .gitignore`» | il commit del 13 tocca `.gitignore` | `awk 'NR>=13655 && NR<=15414' <piano> ¦ grep -n gitignore` → il 13 **misura** soltanto (`grep -c '^/gui/' .gitignore` → due righe dal compito 11) e non scrive nel file: l'`add` è un no-op innocuo | NON RIPRODOTTO | prosa | no | — (togliere `.gitignore` dall'`add` del 13, per non far cercare una modifica che non c'è) |
| R10-18 | Compito 10 · comincia con «- Read: la **§5 del 2**, la riga *«l'attività»*…» senza blocco `**Files:**` | ogni compito apre con *Files* e *Interfaces* | `awk '/^## Compito 10:/{f=1} /^## Compito 11:/{exit} f' <piano> ¦ grep -c '^\*\*Files:\*\*'` → **0**; dai Passi: Passo 2 «`crates/simulator/tests/serving_campaign.rs`, **LF**, nuovo»; Passo 7 «In `scripts/gate.sh` (**`i/lf w/crlf`**, quindi `replace_unique.py`)»; Passo 8 «il richiamo datato nella §5 del disegno del 2» con `git ls-files --eol … 2026-09-06-…design.md` → `i/lf w/lf`; Passo 9 «la riga **10** della tabella della posizione passa a ✅» | CONFERMATO | prosa | no | Inserire prima di «- Read:»: `**Files:**` · «Create: `crates/simulator/tests/serving_campaign.rs` (**LF**) — la campagna, le due proprietà, i due spazi dei mondi» · «Modify: `scripts/gate.sh` (**`i/lf w/crlf`**) — la riga `DST serving` nel settimo passo, con `replace_unique.py`» · «Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**`i/lf w/lf`**) — il richiamo datato nella §5 (**D32**)» · «Modify: **questo piano** — la riga **10** della tabella della posizione» |
| R10-19 | `docs/roadmap.md` · riga del piano della parte 2: «⏳ **in scrittura dal 2026-09-11** — nessuno eseguito. ⛔ **RICHIAMO DEL 2026-09-15: qui stava *«sedici compiti, i primi due scritti»*…»; testa del piano: «✅ **IL PIANO È SCRITTO, dal 2026-09-15**»; D74 «resta da portarla a «eseguito» con la data, e basta» | la roadmap dice lo stato del piano | `grep -n 'in scrittura dal 2026-09-11' docs/roadmap.md` → **1** (riga 231) mentre il piano è **scritto** dal 2026-09-15 e in revisione; il commit `850137c` ha tolto il conteggio e lasciato la parola di stato. Fino al 17 la roadmap dice «in scrittura» di un piano finito: *«un documento di stato disallineato … mente con autorevolezza»* | CONFERMATO | prosa | no | Fuori dai compiti, come `850137c`: «⏳ **scritto il 2026-09-15, in revisione** — nessuno eseguito», e al 17 «eseguito» come D74 prescrive |
| R10-20 | Tutti i rimandi «Passo N del compito M» fra compiti e nel diario (Passo 8 del 10 = §5; Passo 11 del 15 = §8; Passo 15 del 14 = stella polare; Passo 10 del 17 = D56; Passo 6 del 12 = rubinetto; Passi 7 e 11 dell'11 = tipi e ponte; Passi 4 e 7 del 7; Passi 6 e 10 del 14; Passi 6 e 7 del 3) | i Passi citati sono quelli | `awk` sulle intestazioni `- [ ] **Passo N:` dei compiti 3, 7, 10, 11, 12, 14, 15, 17 → tutti coincidono | NON RIPRODOTTO | fatto | no | — |
| R10-21 | Tutti i «Consumes, da oggi» dei blocchi *Interfaces* (63 nomi: `Arbiter`, `MakeRoom`, `ClientGrants`, `degradation_now`, `Sleep`, `RngExt`, `steps_in_doubt`, `CrashingJournal`, `SystemReactor`, `SequentialRng`, `LENGTH_WIDTH`, `WireError`, …) | esistono oggi con quel percorso | `grep -rnE '^\s*pub(\(crate\))? (…)(fn¦struct¦enum¦trait¦const¦type) <nome>\b' crates/*/src` → tutti presenti (`replay` è un metodo del tratto `Journal`, senza `pub`); `kernel::arbiter` riesporta `{LocalPolicy, MakeRoom, RemotePolicy, VramPolicy}` e `{ComputeClass, Mib, Preemption, ResourceProfile, WorkDescriptor}` (`arbiter/mod.rs:30-31`), `pub mod policy` c'è | NON RIPRODOTTO | fatto | no | — |
| R10-22 | **P-113** «la riga della roadmap diceva «sedici compiti, i primi due scritti» … TOLTA» | il conteggio non vive più nella roadmap | `grep -c 'sedici compiti\|diciassette compiti' docs/roadmap.md` → **1**, ma è la citazione **dentro il richiamo datato** (*«qui stava «sedici compiti, i primi due scritti»»*), non un conteggio vivo: P-113 regge | NON RIPRODOTTO | fatto | no | — |

## Copertura del disegno per il mio perimetro

Nessuna sezione dei disegni è assegnata a R10 (il prompt lo dice: *«I compiti non si leggono interi»*, e `constraints.md`: *«Non leggere le sezioni del disegno che non ti sono assegnate»*). Ho verificato soltanto che i **richiami datati ai disegni** che l'undicesima chiusura elenca come *«ancora quelle vecchie»* abbiano davvero il Passo che li produce:

| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|
| §5 del 2 · «`DyingGui` come strumento della campagna» | compito 10 · Passo 8 «il richiamo datato nella §5 del disegno del 2» (`RICHIAMO DEL \<data\>, compito 10 del piano della parte 2 (D32)`) | coperta |
| §7 del 2 · «renderlo raggiungibile dal finto **senza copiarla**» | compito 12 · *Files* «**un** richiamo datato nella §7 (**D41**)», `git add … 2026-09-06-…design.md` | coperta |
| §8 del 2 · la frase dei 🔶 dedotti | compito 15 · Passo 11 «il richiamo datato sulla riga dei dedotti della §8» | coperta |
| stella polare · riga 1 della tabella Passi (D56) | compito 14 · Passo 15 «il richiamo datato sulle righe di Passi della stella polare» — ⚠️ ma il file manca dal *Files* del 14 (**R10-8**) | coperta, con R10-8 |
| stella polare · decisione 56; §9 del 2 · riga 9 | compito 8 · *Files* (Modify dei due disegni, **LF**) | coperta |

## Voci P rimisurate

| P | Comando rilanciato → resa | Regge? |
|---|---|---|
| P-3 | `git ls-files --eol crates/kernel/tests/ports_are_implementable.rs` → `i/crlf w/crlf` | sì |
| P-6 / P-115 | `grep -n '^ceiling=' scripts/check-docs.sh` → `346:ceiling=111616`; `wc -c docs/COMPENDIO.md` → `100586`; margine 11030 | sì |
| P-8 | `grep -n '^run "' scripts/gate.sh` → 39 workspace build · 40 example and compile-fail · 41 no-OS · 42 allow-list · 43 attributes · 44 documentation consistency · 84 DST campaigns; `grep -c 'run "attributes of the constrained crates"'` → 1, `grep -c 'run "documentation consistency"'` → 1 (le ancore del 15 e del 16 sono uniche) | sì |
| P-9 | `grep -c '/gui/' .gitignore` → 0; `grep -c 'matrix\|setup-node' .github/workflows/quality-gate.yml` → 0; il flusso: `runs-on: ubuntu-latest`, `actions/checkout@v4`, `rustup show`, `bash scripts/gate.sh` | sì |
| P-47 | `git ls-files --eol` dei due disegni → `i/lf w/lf`; ⚠️ ma la specie di P-47 (etichetta falsa) è ricomparsa su tre file nei compiti 15 e 16 (**R10-1**, **R10-2**), scritti dopo la sua chiusura: *«l'unica etichetta falsa su ventitré»* era vero al 2026-09-14, non oggi | sì, ma vedi R10-1/2 |
| P-56 | `sed -n '8718,9308p' <piano> ¦ grep -nE '^\s*pub (async )?(const )?(fn¦struct¦enum¦const)'` → `POLICY_FUNCTION`, `Core`, `new`, `journal`, `arbiter`, `custody`, `grants`, `attending`, `serve`: **cinque** accessori, come il blocco | sì |
| P-75 | `grep -n 'interface Fixture' <piano>` → 11979 `{ file; kind; value }` (blocco), 12559 `{ file; message: IpcMessage }` (Passo 9): il difetto è vero, **la correzione dichiarata no** (**R10-7**) | la misura sì, il «Corretto» no |
| P-102 | `grep -c 'gui-shell' .gitignore` → 8; `git ls-files 'spikes/gui-shell/**package-lock.json' 'spikes/gui-shell/**Cargo.lock' ¦ wc -l` → 4 | sì |
| P-112 | `grep -n '^| \*\*D14\*\* |' <piano>` → una riga, `FileCustody::open`; le tre citazioni della D14 altrui portano «della parte 1» / «del piano della parte 1» | sì |
| P-113 | `grep -c 'sedici compiti\|diciassette compiti' docs/roadmap.md` → 1, dentro il richiamo datato; nessun conteggio vivo (**R10-22**); ⚠️ la parola di stato è rimasta «in scrittura» (**R10-19**) | sì |
| P-114 | `git ls-files --eol` delle sette case del 17 → tutte `i/lf w/crlf`; il piano `i/lf w/lf` | sì |
| P-116 | `grep -c '2026-09-06-sottoprogetto-2\|2026-09-07-direzione\|2026-09-09-sottoprogetto-2\|2026-09-11-sottoprogetto-2' docs/README.md` → 0 | sì |
| P-2 (per (d)) | non rilanciato il registro npm: fuori dal mio perimetro (R7/R8) | non verificato |

## Numeri di compito ricensiti

Censiti **625** occorrenze (`numeri2.py`, tollerante al grassetto — la prima passata senza `(\*\*)?` ne perdeva 50, fra cui «compito **9**» e «il **17**») nella testa, nelle voci aperte [C], nella tabella D, nell'undicesima chiusura e nelle 116 voci P: **4** stantie, **477** giuste, **144** non numeri di compito (milestone/disegno 2, sotto-progetti 3/6/7/10/12/13, righe e voci). Le stantie sono tutte **pre-D25** e stanno tutte in righe scritte l'11 e non riscritte il 14:

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| testa, riga 7 della posizione: «Il limite di giri è **al 8** — richiamo del 2026-09-11, D21» | 8 | 9 (daemon; D21 e riga 9 lo dicono) | ✗ stantio — R10-3 |
| D2: «il **compito 12** lo scrive accanto al primo uso» | 12 | 13 (cornice) — e la nota dettata è nel 14 | ✗ stantio — R10-4 |
| D4: «La misura si rifà al **compito 10**» | 10 | 11 (gui nasce; la riga stessa lo dice dopo) | ✗ stantio — R10-5 |
| D12: «spostare il compito 3 dopo **il 10** … per **sette** compiti» | 10, sette | 11, otto | ✗ stantio — R10-6 |
| le voci **P-1…P-61** (scritte prima di D25): «compito 8» = specie Policy, «9» = daemon, «10» = campagna, «11» = gui, «12» = core finto, «13»/«14» = SPA, «15» = cancello, «16» = X-1/X-3, «17» = chiusura — P-6, P-8, P-9, P-40, P-44, P-45, P-46, P-47, P-49, P-51, P-53, P-56, P-59, P-60 portano già i numeri nuovi | — | — | ✓ tutte (nessuna stantia trovata) |
| i numeri che **non** sono compiti: «del 2»/«nel 2»/«il 2» (milestone e disegno 2, 100+), «il 7, il pilastro 3D» (C, D5), «il 3, con le run» / «al 3» (SP 3: C, D19, P-36, P-59, P-93, P-94), «col 12 — il gesto» (D16, P-31, P-32), «il 10» del watchdog (§9 voce 8 del 2: *«il **10**»* = SP 10), «il 13» di AUD-004, «il 12» del primo worker, «3 e 10» della tabella corta | — | — | — classificati, non compiti |

La lista completa, una riga per occorrenza, è la tabella qui sotto (generata da `census_table.py` dalle uscite di `numeri2.py`; la nota dice che cosa è il numero).

### Lista completa, una riga per occorrenza (da `census_table.py`)

| Sezione | Riga | Contesto (…[[numero]]…) | Esito | Nota |
|---|---|---|---|---|
| testa | L5 | proprietario (punto 7 della §10 del [disegno [[del 2]]](../specs/2026-09-06-sottoprogetto-2-gui-min | — | milestone / disegno 2, non un compito |
| testa | L90 | [disegno [[del 2]]](../specs/2026-09-06-sottoprogetto-2-gui-min | — | milestone / disegno 2, non un compito |
| testa | L104 | -progetto 1 passano da sei a sette famiglie ([[compito 4]]). Nessun'altra riga: `git diff --name-only 4 | ✓ | 4 = settima porta (tratto) |
| testa | L104 | CHIAMO DEL 2026-09-11, dal pre-controllo del [[compito 4]] (P-22): i posti sono TRE, non due.** Oltre a | ✓ | 4 = settima porta (tratto) |
| testa | L104 | stano sei»* al presente e in assoluto, e dal [[compito 4]] è falsa alla lettera benché il suo merito —  | ✓ | 4 = settima porta (tratto) |
| testa | L110 | lo gira `npm ci`, gemello di `--locked` ¦ §8 [[del 2]], decisione 49 ¦ | — | milestone / disegno 2, non un compito |
| testa | L111 | le installa **rilancia** il comando della §9 [[del 2]] e scrive nel proprio commit quelle del suo g | — | milestone / disegno 2, non un compito |
| testa | L111 | tata non si installa, con voce d'errata ¦ §9 [[del 2]]; `CLAUDE.md` ¦ | — | milestone / disegno 2, non un compito |
| testa | L136 | la §3.1 (P-22). ⛔ **La suite di conformità è [[al 5]] — richiamo del 2026-09-11, D13** ¦ uno ¦ ⬜ ¦ | ✓ | 5 = custody impl |
| testa | L137 | 6-09-11 (**D13**): una suite ne vuole due, e [[al 4]] ce n'erano zero ¦ uno ¦ ⬜ ¦ | ✓ | 4 = settima porta (tratto) |
| testa | L139 | ick in `Parameters`. ⛔ **Il limite di giri è [[al 8]] — richiamo del 2026-09-11, D21** ¦ uno ¦ ⬜ ¦ | ✗ | ⛔ STANTIO: «al 8» è il daemon PRIMA di D25; oggi il limite di giri è al 9 (D21, riga 9) |
| testa | L141 | 0006 — ⛔ **la specie e la proiezione sono al [[compito 8]], richiamo del 2026-09-14, D25** — «salva, ri | ✓ | 8 = specie Policy |
| testa | L142 | ¦ **10** ¦ la **campagna DST [[del 2]]** in `simulator`, e la sua riga nel settimo  | — | milestone / disegno 2, non un compito |
| testa | L146 | 2026-09-14, P-85**, senza il quale nella SPA [[del 2]] nessuno manda mai un `Invoke` e il registro  | — | milestone / disegno 2, non un compito |
| testa | L146 |  sopra `moveTo`. ⛔ **Il segnaposto è passato [[al 13]] — richiamo del 2026-09-14, D47**: qui restan | ✓ | 13 = SPA cornice |
| testa | L147 | CHIAMO DEL 2026-09-15, dal pre-controllo del [[compito 15]] (P-102): `.gitignore` NON è di questo compit | ✓ | 15 = cancello web |
| testa | L147 | o tracciati; le due righe di `gui/` sono del [[compito 11]] (**D38**) e `/gui/fake-core/target/` del 12. | ✓ | 11 = gui nasce |
| testa | L147 | pito 11 (**D38**) e `/gui/fake-core/target/` [[del 12]]. Al 15 non ne resta nessuna — **D67** ¦ uno  | ✓ | 12 = core finto |
| testa | L166 |    **[[13 e 14]]** apre la SPA nel browser contro il ponte fi | ✓ | 13 = SPA cornice, 14 = SPA moduli |
| testa | L167 | a. ⛔ **RICHIAMO DEL 2026-09-14, scrivendo il [[compito 11]]:** qui stava | ✓ | 11 = gui nasce |
| testa | L168 |    *«[[12 e 13]]»*, numeri di prima che **D25** dividesse la  | ✓ | 12 = core finto, 13 = SPA cornice |
| testa | L169 | nto, che è Rust e non ha browser, e la SPA è [[ai **13]]** e **14**. È la trappola che la | ✓ | 13 = SPA cornice |
| voci aperte [C] | L3151 | -qualita.md), e il doc di `GrantRequest` ¦ **[[il 7]]**, il pilastro 3D — e questo piano **non** l | — | SP 7, pilastro 3D |
| voci aperte [C] | L3152 | un `match` ¦ idem ¦ il primo consumatore; il [[compito 10]] la **asserisce** in una campagna, non la dec | ✓ | 10 = campagna DST |
| voci aperte [C] | L3153 | / E100** — `promote` senza chiamante ¦ righe [[24 e 25]] del Traguardo 5 ¦ chi costruirà il primo cic | — | righe 24 e 25 del Traguardo 5 |
| voci aperte [C] | L3153 | uirà il primo ciclo di orchestrazione; la §5 [[del 2]] lo dichiara già ¦ | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3155 |  rilegge tutto il giornale ¦ voce 7 della §9 [[del 2]] ¦ il **3**, con le run ¦ | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3155 |  tutto il giornale ¦ voce 7 della §9 del 2 ¦ [[il **3]]**, con le run ¦ | — | SP 3 |
| voci aperte [C] | L3156 | '**allocatore** nella porta `journal` ¦ voci [[5 e 6]] della §9 del 2 ¦ il proprietario, confermate | — | voci 5 e 6; decisioni 41 e 42 |
| voci aperte [C] | L3156 |  nella porta `journal` ¦ voci 5 e 6 della §9 [[del 2]] ¦ il proprietario, confermate A il 2026-09-0 | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3156 | tario, confermate A il 2026-09-09 (decisioni [[41 e 42]]): restano com'è ¦ | — | voci 5 e 6; decisioni 41 e 42 |
| voci aperte [C] | L3157 | ** e lo spegnimento pulito ¦ voce 8 della §9 [[del 2]] ¦ il **10** ¦ | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3157 | spegnimento pulito ¦ voce 8 della §9 del 2 ¦ [[il **10]]** ¦ | — | SP 10 (§9 voce 8, watchdog) |
| voci aperte [C] | L3158 | ario sulle skill ¦ l'audit; voce 11 della §9 [[del 2]] ¦ il proprietario, in parallelo; sbarra il * | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3158 | el 2 ¦ il proprietario, in parallelo; sbarra [[il **13]]**, non il 2 ¦ | — | SP 13 (AUD-004) |
| voci aperte [C] | L3158 | ietario, in parallelo; sbarra il **13**, non [[il 2]] ¦ | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3159 | AUD ¦ il **proprietario**; X-1 e X-3 sono il [[compito 16]] ¦ | ✓ | 16 = X-1/X-3 |
| voci aperte [C] | L3161 | o — 9, 26, 27 ¦ la tabella del Traguardo 6 ¦ [[il **12]]** ¦ | — | SP 12 (primo worker) |
| voci aperte [C] | L3162 | to come progetto, «Automazione OS», il grafo [[del 6]], Compatta, i due passi per invocazione ¦ la  | — | SP 3, 6, 10 |
| voci aperte [C] | L3162 | zione ¦ la tabella «Registrate, non prese» ¦ [[il 3]], il 6, il 10, il proprietario ¦ | — | SP 3, 6, 10 |
| voci aperte [C] | L3162 | ¦ la tabella «Registrate, non prese» ¦ il 3, [[il 6]], il 10, il proprietario ¦ | — | SP 3, 6, 10 |
| voci aperte [C] | L3162 | abella «Registrate, non prese» ¦ il 3, il 6, [[il 10]], il proprietario ¦ | — | SP 3, 6, 10 |
| voci aperte [C] | L3163 |  `SOCKET_NAME`** — il daemon lega il nome al [[compito 9]], e in questo piano **nessuno vi si collega** | ✓ | 9 = daemon |
| voci aperte [C] | L3163 | emon/src/main.rs` ¦ il **guscio**, che la §8 [[del 2]] mette *«fuori dal cancello di oggi»*; il gio | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3164 | * entrambe aspettano l'unico pezzo che la §8 [[del 2]] mette *«fuori dal cancello di oggi»* ¦ | — | milestone / disegno 2, non un compito |
| voci aperte [C] | L3165 | ata rifiutata in entrambi i gusci com'erano; [[il 14]] porta «stacca» e «pagina intera» ¦ **P-91**, | ✓ | 14 = SPA moduli |
| voci aperte [C] | L3165 | re voci aspettano lo stesso pezzo, che la §8 [[del 2]] mette *«fuori dal cancello di oggi»* ¦ | — | milestone / disegno 2, non un compito |
| tabella D | L3062 | er **artefatto** sulla tabella *«Il prodotto [[del 2]], e il controllo che esercita ciascun artefat | — | milestone / disegno 2, non un compito |
| tabella D | L3062 | lo che esercita ciascun artefatto»* della §8 [[del 2]], e ogni compito finisce con un artefatto pro | — | milestone / disegno 2, non un compito |
| tabella D | L3063 | videnza delle otto mosse è sulla 8.2.0, e il [[compito 12]] lo scrive accanto al primo uso; se un compor | ✗ | ⛔ STANTIO (D2): il primo uso di dockview è il 13 (cornice), non il 12 (core finto, Rust) |
| tabella D | L3064 | tesso** ¦ è una patch sulla 15.0.1 che la §9 [[del 2]] aveva letto **dentro il pacchetto** (preset  | — | milestone / disegno 2, non un compito |
| tabella D | L3064 | ` con `html: false`, `BAD_PROTO_RE`); ⛔ **il [[compito 13]] rilegge quelle tre proprietà dentro il `.tgz | ✓ | 13 = SPA cornice |
| tabella D | L3064 | CHIAMO DEL 2026-09-14, dal pre-controllo del [[compito 13]] — P-81: la rilettura è del compito 14, non d | ✓ | 13 = SPA cornice |
| tabella D | L3064 | lo del compito 13 — P-81: la rilettura è del [[compito 14]], non del 13.** Questa riga fu scritta prima  | ✓ | 14 = SPA moduli |
| tabella D | L3064 | 3 — P-81: la rilettura è del compito 14, non [[del 13]].** Questa riga fu scritta prima che **D25**  | ✓ | 13 = SPA cornice |
| tabella D | L3064 | iga 8 della posizione; contro quella tabella [[il 13]] è la **cornice** e la Chat col markdown è il | ✓ | 13 = SPA cornice |
| tabella D | L3064 | 13 è la **cornice** e la Chat col markdown è [[il **14]]**, e **D40** mette `markdown-it` fra ciò che | ✓ | 14 = SPA moduli |
| tabella D | L3065 |  non sono maturità. ⛔ **La misura si rifà al [[compito 10**]], e se il piano si scrivesse fra un mese la r | ✗ | ⛔ STANTIO (D4, prima frase): chi installa vitest è l'11; la stessa riga lo dice più avanti |
| tabella D | L3065 | . ✅ **RIMISURATA IL 2026-09-14, scrivendo il [[compito 10]], e la risposta NON cambia:** `npm view vites | ✓ | 10 = campagna DST |
| tabella D | L3065 | , si rifà coi due comandi: chi installa è il [[compito **11**]], e li rilancia quel giorno ¦ | ✓ | 11 = gui nasce |
| tabella D | L3066 | mo scrive perché. La riga `Request` della §5 [[del 2]] riceve un **richiamo datato** al compito 7.  | — | milestone / disegno 2, non un compito |
| tabella D | L3066 | la §5 del 2 riceve un **richiamo datato** al [[compito 7]]. La riga 27 delle voci aperte del Traguardo  | — | SP 7, pilastro 3D |
| tabella D | L3066 | l suo innesco intatto**, e il chiusore resta [[il **7]]**, il pilastro 3D ¦ le tre vie sono state es | — | SP 7, pilastro 3D |
| tabella D | L3066 | ge l'arbitro. ⚠️ **Costo dichiarato:** la §5 [[del 2]] si restringe, e la GUI del 2 vede un `Verdic | — | milestone / disegno 2, non un compito |
| tabella D | L3066 | iarato:** la §5 del 2 si restringe, e la GUI [[del 2]] vede un `Verdict` solo dal rubinetto del cor | — | milestone / disegno 2, non un compito |
| tabella D | L3067 | non serve a nessun passo di questo piano. Il [[compito 5]] usa il `FileBackend` che `platform` già ha ¦ | ✓ | 5 = custody impl |
| tabella D | L3068 | ering::seeded_from`, non in `daemon` ¦ la §7 [[del 2]] costruisce l'attività del kernel **da fuori* | — | milestone / disegno 2, non un compito |
| tabella D | L3069 | entId` dice «the counter», e il richiamo del [[compito 1]] nomina il tipo per esteso ¦ | ✓ | 1 = contatore |
| tabella D | L3070 | , e senza di esso la riga `receive` della §3 [[del 2]] prometterebbe qualcosa che nessuna sonda può | — | milestone / disegno 2, non un compito |
| tabella D | L3070 | n è questo piano:** lo consegnano il daemon ([[compito 9]]) e il core finto (compito 12), e il compito  | ✓ | 9 = daemon |
| tabella D | L3070 | gnano il daemon (compito 9) e il core finto ([[compito 12]]), e il compito 9 dice da dove ¦ | ✓ | 12 = core finto |
| tabella D | L3070 | ompito 9) e il core finto (compito 12), e il [[compito 9]] dice da dove ¦ | ✓ | 9 = daemon |
| tabella D | L3073 | ¦ **D12** ¦ le **fixture nascono al [[compito 3**]], che crea `gui/schema/fixtures/` con **soli  | ✓ | 3 = schema |
| tabella D | L3073 | e.json`, nessun `Cargo.toml` ¦ la §4 e la §8 [[del 2]] le vogliono in `gui/`, che però nasce al **c | — | milestone / disegno 2, non un compito |
| tabella D | L3073 | 2 le vogliono in `gui/`, che però nasce al **[[compito 11**]] (P-18). Le tre vie: spostare il compito 3 do | ✓ | 11 = gui nasce |
| tabella D | L3073 | compito 11** (P-18). Le tre vie: spostare il [[compito 3]] dopo il 10 romperebbe il taglio per artefatt | ✓ | 3 = schema |
| tabella D | L3073 | -18). Le tre vie: spostare il compito 3 dopo [[il 10]] romperebbe il taglio per artefatto (D1) e la | ✗ | ⛔ STANTIO (D12): «dopo il 10» era gui-nasce pre-D25, oggi 11; e i compiti in mezzo sono otto |
| tabella D | L3073 | ifesti `cargo` non la vede. ⚠️ **Costo:** il [[compito 11]] trova la cartella già lì e ci costruisce int | ✓ | 11 = gui nasce |
| tabella D | L3074 | e di conformità della settima porta nasce al [[compito 5]], non al 4**: il 4 porta il tratto, la finta  | ✓ | 5 = custody impl |
| tabella D | L3074 |  della settima porta nasce al compito 5, non [[al 4]]**: il 4 porta il tratto, la finta di `ports_ | ✓ | 4 = settima porta (tratto) |
| tabella D | L3074 | ettima porta nasce al compito 5, non al 4**: [[il 4]] porta il tratto, la finta di `ports_are_impl | ✓ | 4 = settima porta (tratto) |
| tabella D | L3074 | orts_are_implementable.rs` e i tre richiami; [[il 5]] porta le due implementazioni **e** la suite  | ✓ | 5 = custody impl |
| tabella D | L3074 | kernel/src/ports/mod.rs` lo scrive di sé; al [[compito 4]] ce ne sono **zero** (P-20). Le due vie scart | ✓ | 4 = settima porta (tratto) |
| tabella D | L3074 | * (P-20). Le due vie scartate: farla nascere [[al 4]] **col tempo futuro** è ciò che `journal_cont | ✓ | 4 = settima porta (tratto) |
| tabella D | L3074 | otcha #31, lezione già pagata; farla nascere [[al 4]] su una finta minima la renderebbe **vacua**, | ✓ | 4 = settima porta (tratto) |
| tabella D | L3074 |  la prima domanda del pre-controllo. ⚠️ **Il [[compito 4]] resta provato da solo**, che è ciò che D1 ch | ✓ | 4 = settima porta (tratto) |
| tabella D | L3074 |  di `CLAUDE.md` in persona. ⚠️ **Costo:** il [[compito 5]] cresce di un artefatto, e la riga 4 della ta | ✓ | 5 = custody impl |
| tabella D | L3075 | rc/journal.rs` entra nella lista *Files* del [[compito 5]] per **una parola**: `engine` passa da privat | ✓ | 5 = custody impl |
| tabella D | L3076 | ude!` — **e non quella della suite `ipc` del [[compito 2**]], che è `include!` più una `macro_rules!` ¦ l | ✓ | 2 = ipc |
| tabella D | L3077 | io di policy — la registra il **dispaccio**, [[compito 7]] ¦ la regola **1** di ADR-0038 dice che il ke | — | SP 12, il gesto |
| tabella D | L3077 | arbitro, e il secondo invocatore — il gesto, [[col 12]] — dovrebbe aggiungere il proprio effetto lì  | — | SP 12, il gesto |
| tabella D | L3079 | o (**P-30**). ⚠️ **Costo dichiarato:** la §5 [[del 2]] dice *«funzione, invocatore, argomento»* e i | — | milestone / disegno 2, non un compito |
| tabella D | L3079 | a cella riceve il proprio richiamo datato al [[compito 6]] ¦ | ✓ | 6 = registro |
| tabella D | L3080 |  in the kernel»* e la voce aperta 7 della §9 [[del 2]] assegna **al 3** il confine di sessione. `se | — | milestone / disegno 2, non un compito |
| tabella D | L3080 |  e la voce aperta 7 della §9 del 2 assegna **[[al 3]]** il confine di sessione. `serving` è misura | — | SP 3 |
| tabella D | L3082 | ¦ **D21** ¦ ⛔ **il limite di giri passa al [[compito 9**]], e la riga 7 della tabella della posizione p | ✓ | 9 = daemon |
| tabella D | L3082 | le ¦ `EXECUTOR_TURN_LIMIT` vive nel file del [[compito 9]], e la sonda che la §5 del 2 detta per esso — | ✓ | 9 = daemon |
| tabella D | L3082 | nel file del compito 9, e la sonda che la §5 [[del 2]] detta per esso — *«il grafo con la GUI resta | — | milestone / disegno 2, non un compito |
| tabella D | L3083 | osto dichiarato:** è una divergenza dalla §5 [[del 2]], che dice *«consegnato»*; riceve il richiamo | — | milestone / disegno 2, non un compito |
| tabella D | L3084 | *differisce** da quello già ricevuto ¦ la §7 [[del 2]] lascia la scelta al piano fra *«a ogni giro» | — | milestone / disegno 2, non un compito |
| tabella D | L3085 | ive lui il `grant` su A** — correzione **nel [[compito 6**]], non nell'errata ¦ senza di esso la via dell | ✓ | 6 = registro |
| tabella D | L3085 | ila da sé sarebbe la seconda copia che il §7 [[del 2]] rifiuta con le stesse parole. ⚠️ **Un enum e | — | milestone / disegno 2, non un compito |
| tabella D | L3085 |  di un argomento, il blocco *Interfaces* del [[compito 6]] lo dice, e il registro guadagna `held` — che | ✓ | 6 = registro |
| tabella D | L3086 | ¦ **D25** ¦ ⛔ **il [[compito 8]] si SPEZZA in due**: il **8** porta la specie | ✓ | 8 = specie Policy |
| tabella D | L3086 | D25** ¦ ⛔ **il compito 8 si SPEZZA in due**: [[il **8]]** porta la specie `Policy` del giornale e la | ✓ | 8 = specie Policy |
| tabella D | L3086 | e la proiezione che la rilegge, in `kernel`; [[il **9]]** porta il cablaggio del daemon. I compiti s | ✓ | 9 = daemon |
| tabella D | L3086 | a il cablaggio del daemon. I compiti scritti [[dal 9]] al 16 scalano di uno, e **D1 perde il numera | ✓ | 9 = daemon |
| tabella D | L3086 | ablaggio del daemon. I compiti scritti dal 9 [[al 16]] scalano di uno, e **D1 perde il numerale** ¦ | ✓ | 16 = X-1/X-3 |
| tabella D | L3086 | ni riferimento a un numero di compito **dopo [[il 8]]** in questo file si sposta di uno — corretti | ✓ | 8 = specie Policy |
| tabella D | L3088 | , col commento che lo lega ad ADR-0006, e il [[compito **9**]] lo tiene lì. ⚠️ **Costo dichiarato:** il chi | ✓ | 9 = daemon |
| tabella D | L3090 | vece di inventarla:** è quella del banco del [[compito 7]] e di `crates/simulator/tests/arbiter_campaig | ✓ | 7 = serving |
| tabella D | L3090 | ide qui:** la quinta chiusura l'ha assegnata [[al **10]]**, il primo che può misurarne il bisogno con | ✓ | 10 = campagna DST |
| tabella D | L3091 | **La metà che serve esiste già, misurata nel [[compito 7]]:** `Err(CustodyError::Unavailable)` è **già* | ✓ | 7 = serving |
| tabella D | L3091 | ioni nuove nella porta — che è ciò che la §8 [[del 2]] aveva previsto con quelle parole. Le due vie | — | milestone / disegno 2, non un compito |
| tabella D | L3092 | e al core e il core finto ne sceglie uno suo [[al 12]]. ⛔ **E il guscio NON è un compito di questo  | ✓ | 12 = core finto |
| tabella D | L3092 | N è un compito di questo piano** (**P-53**): [[il 12]] lega un nome suo, il 13 e il 14 portano una  | ✓ | 12 = core finto |
| tabella D | L3092 |  piano** (**P-53**): il 12 lega un nome suo, [[il 13]] e il 14 portano una SPA che non tocca socket | ✓ | 13 = SPA cornice |
| tabella D | L3092 |  (**P-53**): il 12 lega un nome suo, il 13 e [[il 14]] portano una SPA che non tocca socket, e il c | ✓ | 14 = SPA moduli |
| tabella D | L3092 | CHIAMO DEL 2026-09-14, dal pre-controllo del [[compito 12]] (P-73): le case sono DUE da quel compito, no | ✓ | 12 = core finto |
| tabella D | L3092 |  letterali è un **criterio di chiusura** del [[compito 12]] (**D45**), non una riga di catalogo — vincol | ✓ | 12 = core finto |
| tabella D | L3093 | ¦ **D32** ¦ ⛔ **la campagna [[del 2]] NON usa `simulator::ipc::DyingGui`**, benché | — | milestone / disegno 2, non un compito |
| tabella D | L3093 | ore dietro un `RefCell`, com'è nel banco del [[compito 7]]. I guasti restano due — la morte sulla porta | ✓ | 7 = serving |
| tabella D | L3093 | cella — e `Core::ipc` è **già assegnato** al [[compito 12]] con un altro significato. ⚠️ **Costo dichiar | ✓ | 12 = core finto |
| tabella D | L3093 | ittura della stessa forma di filo — il banco [[del 7]], il daemon del 9, questa — e le tre case non | ✓ | 7 = serving |
| tabella D | L3093 | sa forma di filo — il banco del 7, il daemon [[del 9]], questa — e le tre case non possono importar | ✓ | 9 = daemon |
| tabella D | L3094 | prima metà confronterebbe **insiemi vuoti**: [[il 2]] non rilascia nessuna concessione a un client | — | milestone / disegno 2, non un compito |
| tabella D | L3095 | a decisione che D29 e la testa del banco del [[compito 7]] avevano REGISTRATO per il 10, e si prende co | ✓ | 7 = serving |
| tabella D | L3095 | l banco del compito 7 avevano REGISTRATO per [[il 10]], e si prende con la misura.** Delle quattro  | ✓ | 10 = campagna DST |
| tabella D | L3096 | 1.map` escono dallo **stesso passaggio** del [[compito 3]], e la mappa resta il foglio che legge un uma | ✓ | 3 = schema |
| tabella D | L3096 |  chiedono *«il valore atteso in JSON»*, e il [[compito 3]] aveva scritto il `Debug` di Rust — che dal c | ✓ | 3 = schema |
| tabella D | L3099 | ` e `/gui/dist/` di `.gitignore` entrano col [[compito 11**]], non col 15; `/gui/fake-core/target/` resta  | ✓ | 11 = gui nasce |
| tabella D | L3099 | i `.gitignore` entrano col compito 11**, non [[col 15]]; `/gui/fake-core/target/` resta al 12 ¦ **P- | ✓ | 15 = cancello web |
| tabella D | L3099 | , non col 15; `/gui/fake-core/target/` resta [[al 12]] ¦ **P-66**: una riga di `.gitignore` apparti | ✓ | 12 = core finto |
| tabella D | L3099 | ito che **crea** ciò che ignora. Raggruppate [[al 15]], il commit del compito 11 elencherebbe l'alb | ✓ | 15 = cancello web |
| tabella D | L3099 | che ignora. Raggruppate al 15, il commit del [[compito 11]] elencherebbe l'albero di `node_modules` e il | ✓ | 11 = gui nasce |
| tabella D | L3099 | ale 13 diventerebbe ineseguibile. ⚠️ **La §8 [[del 2]] non è smentita nel merito** — le tre righe r | — | milestone / disegno 2, non un compito |
| tabella D | L3101 | ¦ **D40** ¦ ⛔ **il [[compito 11]] installa SOLO ciò che usa** — `vue`, `@vitej | ✓ | 11 = gui nasce |
| tabella D | L3101 | `axe-core` e la catena `eslint` arrivano coi [[compiti **13**]], **14** e **15**, che li consumano. ⚠️ **`en | ✓ | 13 = SPA cornice |
| tabella D | L3101 | nes.node` fa ECCEZIONE e si scrive intero al [[compito 11**]] (**D37**): è il prerequisito dell'ambiente d | ✓ | 11 = gui nasce |
| tabella D | L3102 | i — `crates/daemon/src/main.rs`, la campagna [[del **10]]**, `gui/fake-core` — e ciascuna porta accant | ✓ | 10 = campagna DST |
| tabella D | L3104 | RefCell<Core>` di `serve`**, e ciò che nasce [[al 12]] è il solo `Core::ipc` ¦ **P-71**: il blocco  | ✓ | 12 = core finto |
| tabella D | L3104 | :ipc` ¦ **P-71**: il blocco *Interfaces* del [[compito 7]] fissa già la forma — `serve(core: &RefCell<C | ✓ | 7 = serving |
| tabella D | L3104 | o**, non da un meccanismo da inventare; e il [[compito 7]] scrive per esteso che il chiamante di `Core: | ✓ | 7 = serving |
| tabella D | L3106 | on un comando** nel criterio di chiusura del [[compito 12]] ¦ **P-73**: la §7 vuole che *«la GUI non sa  | ✓ | 12 = core finto |
| tabella D | L3107 |  un `IpcMessage` tipizzato**, e la finta del [[compito 11]] **non cresce** ¦ **P-76**: `stamp_set()` è * | ✓ | 11 = gui nasce |
| tabella D | L3108 | ¦ **D47** ¦ ⛔ **il SEGNAPOSTO arriva al [[compito 13**]], e il 14 tiene i moduli veri ¦ **P-77**: `do | ✓ | 13 = SPA cornice |
| tabella D | L3108 |  ⛔ **il SEGNAPOSTO arriva al compito 13**, e [[il 14]] tiene i moduli veri ¦ **P-77**: `dockview` c | ✓ | 14 = SPA moduli |
| tabella D | L3108 | ello spike), quindi senza di esso la cornice [[del 13]] si monta e non mostra nulla — con `npm run b | ✓ | 13 = SPA cornice |
| tabella D | L3108 | ifetto che compila. E le tre prove che la §8 [[del 2]] chiede a questo artefatto — le viste che si  | — | milestone / disegno 2, non un compito |
| tabella D | L3109 | ifica niente — contro la ragione scritta nel [[compito 11]], *«tutto ciò che il guscio sa fare resta dal | ✓ | 11 = gui nasce |
| tabella D | L3109 |  stato della connessione: è del modulo Chat, [[compito 14]] ¦ | ✓ | 14 = SPA moduli |
| tabella D | L3110 | localStorage` come casa della disposizione — [[nel 2]] la casa è la **settima porta** (I1). ⚠️ **`m | — | milestone / disegno 2, non un compito |
| tabella D | L3110 | ettima porta** (I1). ⚠️ **`moveActive` è del [[compito 14**]], con l'accessibilità ¦ | ✓ | 14 = SPA moduli |
| tabella D | L3112 | e scritte le guarda una sonda NOSTRA fino al [[compito 15**]], che la sostituisce con `no-raw-text` ¦ **P- | ✓ | 15 = cancello web |
| tabella D | L3112 | 3**: la catena `eslint` e `gate-gui.sh` sono [[il **15]]** (tabella della posizione, **D40**), quindi | ✓ | 15 = cancello web |
| tabella D | L3112 | abella della posizione, **D40**), quindi fra [[il 13]] e il 15 la riga delle scritte della §8 non a | ✓ | 13 = SPA cornice |
| tabella D | L3112 | ella posizione, **D40**), quindi fra il 13 e [[il 15]] la riga delle scritte della §8 non avrebbe n | ✓ | 15 = cancello web |
| tabella D | L3112 |  la sintassi di Vue. ⛔ **Anticipare `eslint` [[al 13]] costerebbe di più:** la configurazione che i | ✓ | 13 = SPA cornice |
| tabella D | L3112 | 3 costerebbe di più:** la configurazione che [[il 15]] scrive comunque per il cancello vivrebbe in  | ✓ | 15 = cancello web |
| tabella D | L3112 | (gotcha #68), mentre la sonda nostra **muore [[al 15]]** e non lascia niente ¦ | ✓ | 15 = cancello web |
| tabella D | L3113 | Il file vive in `gui/src/schema/`, che è del [[compito 11]], e nasce QUI perché qui nasce il suo unico c | ✓ | 11 = gui nasce |
| tabella D | L3116 | due sorgenti Rust, e il criterio di chiusura [[del 14]] li confronta con un comando** ¦ **P-88**: so | ✓ | 14 = SPA moduli |
| tabella D | L3117 | l passo, funzione, chiuso o in dubbio — e il [[compito 14]] scrive il richiamo datato sulla riga 1 della | ✓ | 14 = SPA moduli |
| tabella D | L3117 | 89**: la riga 1 promette cinque campi, la §4 [[del 2]] (più recente) dice *«con intento ed esito»*  | — | milestone / disegno 2, non un compito |
| tabella D | L3117 | *«con intento ed esito»* e `StepSummary` del [[compito 3]] la segue; nel 2 l'invocatore e la classe del | ✓ | 3 = schema |
| tabella D | L3117 | to»* e `StepSummary` del compito 3 la segue; [[nel 2]] l'invocatore e la classe dell'effetto sono * | — | milestone / disegno 2, non un compito |
| tabella D | L3117 | icy`. ⚠️ **Costo dichiarato di A:** la lista [[del 2]] non dice **quale** policy un'invocazione ha  | — | milestone / disegno 2, non un compito |
| tabella D | L3117 | A/B all'apertura della sessione che scrive i [[compiti 15–17]] invece che alla rilettura, perché B avrebbe  | ✓ | 15 = cancello web, 17 = chiusura |
| tabella D | L3117 | : **nessun compito cambia**, il Passo 15 del [[compito 14]] resta com'è scritto, e il **costo dichiarato | ✓ | 14 = SPA moduli |
| tabella D | L3117 | l **costo dichiarato di A resta** — la lista [[del 2]] non dice quale policy un'invocazione ha chie | — | milestone / disegno 2, non un compito |
| tabella D | L3118 | esta e nessuno gliela fa, quindi il revisore [[del 14]] vedrebbe cinque moduli vuoti. ⛔ **Scartata l | ✓ | 14 = SPA moduli |
| tabella D | L3118 | é all'`Hello`:** falsificherebbe il criterio [[del 13]] (la fascia sparirebbe) e farebbe **decidere* | ✓ | 13 = SPA cornice |
| tabella D | L3120 | vocazione (decisione 21 della stella polare, [[compito 3]]), e nessuno store del 13 ricorda l'ultimo `I | — | SP 3 |
| tabella D | L3120 | a stella polare, compito 3), e nessuno store [[del 13]] ricorda l'ultimo `Invoke`. ⛔ **Non un campo  | — | SP 3 |
| tabella D | L3120 |  messaggio dedotto della riga 1 di Permessi, [[del 3]] ¦ | — | SP 3 |
| tabella D | L3121 | e:** il core non tiene niente in sospeso (§5 [[del 2]]), quindi rifiutare è locale; Escape e il cli | — | milestone / disegno 2, non un compito |
| tabella D | L3122 | e quando la provenienza cambia** ¦ **P-94**: [[nel 2]] il flusso non finisce mai (il rubinetto gira | — | milestone / disegno 2, non un compito |
| tabella D | L3122 | l filo non ha un confine di messaggio, che è [[del 3]]; un testo che cresce senza fine si renderebb | — | SP 3 |
| tabella D | L3125 | pannello **è** il `module` di `PANEL_TYPES` ([[compito 13]]) ed **è** la chiave `modules.*` della locale | ✓ | 13 = SPA cornice |
| tabella D | L3125 | ed **è** la chiave `modules.*` della locale ([[compito 14]]); rinominare per compiacere un lint muovereb | ✓ | 14 = SPA moduli |
| tabella D | L3126 | ` esce **0** sui soli avvisi, quindi la rete [[del 13]] sarebbe sostituita da un controllo **vacuo** | ✓ | 13 = SPA cornice |
| tabella D | L3128 | ¦ **D67** ¦ ⛔ **il [[compito 15]] NON tocca `.gitignore` e NON tocca i lockfil | ✓ | 15 = cancello web |
| tabella D | L3128 | *eseguita**. Le due righe di `gui/` sono del [[compito 11]] (**D38**) e `/gui/fake-core/target/` del 12, | ✓ | 11 = gui nasce |
| tabella D | L3128 | pito 11 (**D38**) e `/gui/fake-core/target/` [[del 12]], quindi al 15 di `.gitignore` non resta **ni | ✓ | 12 = core finto |
| tabella D | L3128 | *) e `/gui/fake-core/target/` del 12, quindi [[al 15]] di `.gitignore` non resta **niente**. ⛔ **Ri | ✓ | 15 = cancello web |
| tabella D | L3129 |  dove chi legge il rosso va a guardare, e il [[compito **17**]] la porta in `porta-di-qualita.md` ¦ | ✓ | 17 = chiusura |
| tabella D | L3132 | ** vulnerabilità, **~6 s** sull'insieme vero [[del 2]]. Un livello scelto oggi sarebbe una manopola | — | milestone / disegno 2, non un compito |
| tabella D | L3134 | nda S3, e NESSUNA riga di catalogo** ¦ la §8 [[del 2]] lo chiede così, e il vincolo globale 7 lo im | — | milestone / disegno 2, non un compito |
| undicesima chiusura | L18807 | ssun manifesto — ma è il prerequisito che il [[compito **16**]] dichiara, quindi su questa | ✓ | 16 = X-1/X-3 |
| undicesima chiusura | L18809 | [[compito 11]] va aggiornato, perché `jsdom` 30.0.1 pretend | ✓ | 11 = gui nasce |
| undicesima chiusura | L18813 | essun compito è cambiato** e il Passo 15 del [[compito 14]] resta com'è; il richiamo datato è nella riga | ✓ | 14 = SPA moduli |
| undicesima chiusura | L18823 | uscita**, perché il Passo 15 del [[compito 14]] la copre e il compito 14 è scritto. | ✓ | 14 = SPA moduli |
| undicesima chiusura | L18823 | ché il Passo 15 del compito 14 la copre e il [[compito 14]] è scritto. | ✓ | 14 = SPA moduli |
| undicesima chiusura | L18827 | ¦ la §5 del disegno [[del 2]] nomina ancora `DyingGui` come strumento dell | — | milestone / disegno 2, non un compito |
| undicesima chiusura | L18827 |  strumento della campagna ¦ il **Passo 8 del [[compito 10**]] ¦ | ✓ | 10 = campagna DST |
| undicesima chiusura | L18828 | ¦ la §7 del disegno [[del 2]] dice *«renderlo raggiungibile dal finto **se | — | milestone / disegno 2, non un compito |
| undicesima chiusura | L18828 | ungibile dal finto **senza copiarla**»* ¦ il [[compito **12**]], richiamo di **D41** ¦ | ✓ | 12 = core finto |
| undicesima chiusura | L18829 | ¦ la frase dei 🔶 **dedotti** della §8 [[del 2]] — tre deduzioni, e tutte e tre sono misurate | — | milestone / disegno 2, non un compito |
| undicesima chiusura | L18829 | utte e tre sono misurate ¦ il **Passo 11 del [[compito 15**]], che scrive un richiamo solo e nomina dove s | ✓ | 15 = cancello web |
| undicesima chiusura | L18846 | e di `interprocess` nel manifesto del finto ([[compito 12]]) — `grep -n '<the version' <questo file>`. ⚠ | ✓ | 12 = core finto |
| undicesima chiusura | L18846 | questo file>`. ⚠️ **`<data>` e `<tempo>` nei [[compiti 15]], 16 e 17 NON sono segnaposto:** sono i valor | ✓ | 15 = cancello web |
| undicesima chiusura | L18846 | `. ⚠️ **`<data>` e `<tempo>` nei compiti 15, [[16 e 17]] NON sono segnaposto:** sono i valori del gio | ✓ | 16 = X-1/X-3, 17 = chiusura |
| undicesima chiusura | L18847 | esta sessione non ha toccato il compendio, e [[il **17]]** è l'unico compito che lo tocca ¦ | ✓ | 17 = chiusura |
| undicesima chiusura | L18848 | l suo perché (**P-113**). Le altre case sono [[il **17]]** ¦ | ✓ | 17 = chiusura |
| undicesima chiusura | L18858 | d avviso lascia `eslint` a **zero**: la rete [[del 13]] sarebbe sostituita da un controllo vacuo ¦ u | ✓ | 13 = SPA cornice |
| undicesima chiusura | L18859 | seconda** sonda di `copy.test.ts` sopravvive [[al 15]] (**D65**, **P-105**) ¦ le diciotto chiavi `m | ✓ | 15 = cancello web |
| undicesima chiusura | L18861 | ¦ 60 ¦ [[il 15]] **non** tocca `.gitignore` (**D67**) ¦ le ot | ✓ | 15 = cancello web |
| undicesima chiusura | L18872 | , NON FRA I PIANI.** La lista di lettura del [[compito 17]] diceva *«le | ✓ | 17 = chiusura |
| undicesima chiusura | L18877 | e l'ha commessa**, scrivendo il Passo 10 del [[compito 17]]: il conteggio è salito di **quattro** con tr | ✓ | 17 = chiusura |
| undicesima chiusura | L18882 |   Inserendo le righe `D` del [[compito 16]] l'ancora `"\n\n**La baseline"` ha prodotto ` | ✓ | 16 = X-1/X-3 |
| undicesima chiusura | L18905 | IL FALSO SI CORREGGE SUBITO, anche se «è del [[compito 17]]».** La riga della | ✓ | 17 = chiusura |
| undicesima chiusura | L18927 |  fra uno e l'altro. ⛔ **Prima di eseguire il [[compito 11]] si aggiorna Node**, e su una | ✓ | 11 = gui nasce |
| undicesima chiusura | L18928 | hina che non l'ha si installa `cargo-audit` ([[compito 16]]). | ✓ | 16 = X-1/X-3 |
| voci P | L196 | le` e il filo porta un `GrantRequest`: la §5 [[del 2]] dice una cosa che il codice non lascia fare  | — | milestone / disegno 2, non un compito |
| voci P | L199 | della §5 [[del 2]] dice *«`admit` → `Verdict`»*. Ma `Arbiter::a | — | milestone / disegno 2, non un compito |
| voci P | L221 | **Conseguenza per questo piano:** il [[compito 7]] **non** costruisce un profilo, e la decision | ✓ | 7 = serving |
| voci P | L225 | Rilanciato il comando della §9 [[del 2]] (registro npm e `crates.io`), più i pacchett | — | milestone / disegno 2, non un compito |
| voci P | L251 | chiarato fuori dal cancello di oggi dalla §8 [[del 2]] («dopo lo spike, parte 2 … fuori dal cancell | — | milestone / disegno 2, non un compito |
| voci P | L258 | misurato. Il [[compito 4]] tocca quel file per aggiungere la finta dell | ✓ | 4 = settima porta (tratto) |
| voci P | L266 | .cbor` — **sei** `.cbor` più la mappa. La §5 [[del 2]] lo dice già col richiamo del | — | milestone / disegno 2, non un compito |
| voci P | L278 | 026-08-28** (finding AUD-054) sul `FIVE`: il [[compito 4]] lo **legge prima** di toccarlo, e il richiam | ✓ | 4 = settima porta (tratto) |
| voci P | L283 | il 2026-09-11 col comando del vincolo 11. Il [[compito 17]] lo consuma: il compendio riceve le | ✓ | 17 = chiusura |
| voci P | L296 | tion consistency · `84` DST campaigns. La §8 [[del 2]] vuole la riga nuova **fra la 43 e la | — | milestone / disegno 2, non un compito |
| voci P | L297 | ; `cargo audit` di X-3 è un'altra riga, e il [[compito 16]] dice dove. | ✓ | 16 = X-1/X-3 |
| voci P | L305 | più — il [[compito 16]] lo **legge prima** di aggiungere la matrice. | ✓ | 16 = X-1/X-3 |
| voci P | L316 | La §7 [[del 2]] dice che alla parola `verdict` il rubinetto  | — | milestone / disegno 2, non un compito |
| voci P | L319 | ragione per cui D5 non toglie nulla alla GUI [[del 2]]**: la riga 4 | — | milestone / disegno 2, non un compito |
| voci P | L321 | [[del 2]] già prescrive. | — | milestone / disegno 2, non un compito |
| voci P | L333 | riga `receive` della §3 [[del 2]] dice *«lettura non bloccante nel buffer; cor | — | milestone / disegno 2, non un compito |
| voci P | L345 | g` sopra `MAX_BODY_LEN`. **Conseguenza:** il [[compito 2]] aggiunge `declared_len` e | ✓ | 2 = ipc |
| voci P | L351 | La §3 [[del 2]] lo pretende — *«cornice rotta → `MalformedMe | — | milestone / disegno 2, non un compito |
| voci P | L357 | [[compito 2]] una ce l'ha. **Conseguenza:** **D9**, il tet | ✓ | 2 = ipc |
| voci P | L370 | svista da correggere qui: il [[compito 2]] lo scrive nel doc del **trasporto**, che è c | ✓ | 2 = ipc |
| voci P | L380 | ettera l'innesco è **questo piano**, e la §4 [[del 2]] non elenca nessuna variante di revoca fra le | — | milestone / disegno 2, non un compito |
| voci P | L383 | stesso:** **D5** non serve `Request`, quindi [[nel 2]] **nessuna concessione | — | milestone / disegno 2, non un compito |
| voci P | L386 | guardo 6 porta già in `porta-di-qualita.md`, [[il **7]]**. | — | SP 7, pilastro 3D |
| voci P | L393 | a variante nuova; il **richiamo datato** del [[compito 3]], passo 9 (a). E | ✓ | 3 = schema |
| voci P | L400 | e varianti unit»*. ⛔ **Le varianti nuove del [[compito 3]] | ✓ | 3 = schema |
| voci P | L412 |  **Conseguenza:** il **richiamo datato** del [[compito 3]], passo 9 (b), **più una sonda** — | ✓ | 3 = schema |
| voci P | L416 | 8 — Le fixture vanno in `gui/`, che nasce al [[compito 11]] | ✓ | 11 = gui nasce |
| voci P | L418 | La §4 [[del 2]] e la riga «lo schema» della tabella degli ar | — | milestone / disegno 2, non un compito |
| voci P | L420 | **[[compito 11**]], otto compiti dopo. Un compito che scrive in | ✓ | 11 = gui nasce |
| voci P | L424 | i i file si committano; e la cartella che il [[compito 3]] crea porta | ✓ | 3 = schema |
| voci P | L430 | tto è sbagliato, e non compilerebbe.** La §4 [[del 2]] dice che `Degradation` porta *«i due | — | milestone / disegno 2, non un compito |
| voci P | L454 | rmità della settima porta NON può nascere al [[compito 4]]: le vuole due, e al 4 ce n'è zero | ✓ | 4 = settima porta (tratto) |
| voci P | L454 | ON può nascere al compito 4: le vuole due, e [[al 4]] ce n'è zero | ✓ | 4 = settima porta (tratto) |
| voci P | L456 | a riga 4 della tabella della posizione dà al [[compito 4]] | ✓ | 4 = settima porta (tratto) |
| voci P | L457 | s`, la suite di conformità»*, e la riga 5 dà [[al 5]] le | ✓ | 5 = custody impl |
| voci P | L461 | H NEEDS TWO IMPLEMENTATIONS TO COMPARE»*. Al [[compito 4]] le implementazioni sono **zero**: la | ✓ | 4 = settima porta (tratto) |
| voci P | L470 | **Conseguenza: D13.** ⚠️ **E il [[compito 4]] resta un artefatto provato da solo**, che è  | ✓ | 4 = settima porta (tratto) |
| voci P | L484 | ⛔ **Chi esegue il [[compito 4]] legge quella riga e ha due modi di sbagliare | ✓ | 4 = settima porta (tratto) |
| voci P | L487 | screpanza non sparisce, si SPOSTA:** dopo il [[compito 4]] il | ✓ | 4 = settima porta (tratto) |
| voci P | L491 | **Conseguenza per il [[compito 4]]:** il richiamo datato su quel capoverso dice | ✓ | 4 = settima porta (tratto) |
| voci P | L498 | -progetto 1 passano da sei a sette famiglie ([[compito 4]]). **Nessun'altra riga**»*. | ✓ | 4 = settima porta (tratto) |
| voci P | L502 | ¦ Riga ¦ Che cosa dice ¦ Dopo il [[compito 4]] ¦ | ✓ | 4 = settima porta (tratto) |
| voci P | L506 | rtura è al **presente e in assoluto**, e dal [[compito 4]] è falsa alla lettera. È la radice **R1** del | ✓ | 4 = settima porta (tratto) |
| voci P | L509 | ichiamo datato** — tre posti, non due — e il [[compito 4]] | ✓ | 4 = settima porta (tratto) |
| voci P | L515 | ### P-23 — Il [[compito 2]] dà a `ipc` un'implementazione vera e lascia  | ✓ | 2 = ipc |
| voci P | L518 | [[compito **2**]], e si vede solo scrivendo il **4**, perché è | ✓ | 2 = ipc |
| voci P | L518 | compito **2**, e si vede solo scrivendo [[il **4]]**, perché è il 4 che apre quei due file. Mis | ✓ | 4 = settima porta (tratto) |
| voci P | L518 |  e si vede solo scrivendo il **4**, perché è [[il 4]] che apre quei due file. Misurato il 2026-09- | ✓ | 4 = settima porta (tratto) |
| voci P | L519 | la lista *Files* del [[compito 2]] non nomina né `crates/kernel/src/ports/mod.r | ✓ | 2 = ipc |
| voci P | L521 | [[compito 2]] non rende **nulla**. Ma il compito 2 crea `p | ✓ | 2 = ipc |
| voci P | L521 | compito 2 non rende **nulla**. Ma il [[compito 2]] crea `platform::ipc::LocalSocketIpc`, e da q | ✓ | 2 = ipc |
| voci P | L523 | ¦ Dove ¦ Che cosa dice ¦ Dopo il [[compito 2]] ¦ | ✓ | 2 = ipc |
| voci P | L527 |  **falsa**: arriva col **sotto-progetto 2**, [[compito 2]] ¦ | ✓ | 2 = ipc |
| voci P | L533 | *Conseguenza: la lista *Files* e i passi del [[compito 2]] crescono di due file e di tre richiami datat | ✓ | 2 = ipc |
| voci P | L534 | nel [[compito 2]] col richiamo che dice **perché** sono arriva | ✓ | 2 = ipc |
| voci P | L535 | na voce d'errata**, e la differenza è che il [[compito 2]] **non è eseguito**: un'errata è per ciò | ✓ | 2 = ipc |
| voci P | L549 | e dopo il [[compito 5]] ne renderà ancora tre mentre le implementazi | ✓ | 5 = custody impl |
| voci P | L570 | **Conseguenza per il [[compito 5]]:** `crates/platform/src/lib.rs` entra nella  | ✓ | 5 = custody impl |
| voci P | L574 | ### P-25 — Il [[compito 2]] lascia FALSA la prosa di `platform/src/lib.r | ✓ | 2 = ipc |
| voci P | L578 | a** del compito che lo rompe: il Passo 9 del [[compito 2]] aggiunge `pub mod ipc;` e **non | ✓ | 2 = ipc |
| voci P | L582 |  — `Ipc` è fra i sette nomi — quindi dopo il [[compito 2]] il comando risponde | ✓ | 2 = ipc |
| voci P | L586 | ⛔ **Corretto nel [[compito 2]] e non con una voce d'errata**, per la stessa | ✓ | 2 = ipc |
| voci P | L586 | ata**, per la stessa ragione di **P-23**: il [[compito 2]] **non è | ✓ | 2 = ipc |
| voci P | L589 | eguenza:** la lista *Files* e il Passo 9 del [[compito 2]] crescono di un **richiamo datato** sulla fra | ✓ | 2 = ipc |
| voci P | L590 | ⚠️ **Il [[compito 5]] NON lo rifà**: quando arriva, la frase nomin | ✓ | 5 = custody impl |
| voci P | L590 | va, la frase nomina già `Ipc`, e il richiamo [[del 5]] si aggiunge | ✓ | 5 = custody impl |
| voci P | L591 | **sotto** quello [[del 2]] senza cancellarlo — due richiami datati nell | — | milestone / disegno 2, non un compito |
| voci P | L621 | , il tutto raggiunto con `include!`. Ma il **[[compito 2**]] ha scelto un'altra | ✓ | 2 = ipc |
| voci P | L625 | coerenza da sanare, e la causa è scritta nel [[compito 2]]:** la suite di `ipc` dichiara *«what is | ✓ | 2 = ipc |
| voci P | L638 | rta»* della tabella degli artefatti della §8 [[del 2]] ne elenca sette in una cella sola, e il tagl | — | milestone / disegno 2, non un compito |
| voci P | L651 | ⛔ **Le ultime due non si POSSONO scrivere [[al 5]], e non è una scelta:** parlano di `SaveLayou | ✓ | 5 = custody impl |
| voci P | L652 | ono **messaggi** — varianti dello schema del [[compito 3]] — e di un core che parte, che è il daemon. A | ✓ | 3 = schema |
| voci P | L652 |  e di un core che parte, che è il daemon. Al [[compito 5]] il | ✓ | 5 = custody impl |
| voci P | L653 |  qui perché un censimento della §8 contro il [[compito 5]] le trovi assenti e sappia che è | ✓ | 5 = custody impl |
| voci P | L670 | , che è ciò che il doc di `CustodyError` del [[compito 4]] rifiuta con le sue stesse parole. E nessun c | ✓ | 4 = settima porta (tratto) |
| voci P | L675 | econda chiave**, chiunque la aggiunga — e il [[compito 5]] porta un **passo di misura**: si scrive la m | ✓ | 5 = custody impl |
| voci P | L686 |  l'artefatto è sbagliato, e compila.** La §5 [[del 2]] dà a `invoke` *«nome, invocatore, argomento» | — | milestone / disegno 2, non un compito |
| voci P | L687 | tace sui tipi. Ma il [[compito 3]] mette sul filo `Call { function: String, arg | ✓ | 3 = schema |
| voci P | L702 | il proprio **richiamo datato** al [[compito 6]], insieme a quello di **P-32**. | ✓ | 6 = registro |
| voci P | L708 | lla forma dei registri di ADR-0009. Ma la §5 [[del 2]] dice *«se sì → l'effetto»* e | — | milestone / disegno 2, non un compito |
| voci P | L712 | amPolicy`. Il secondo invocatore — il gesto, [[col 12]] — | — | SP 12, il gesto |
| voci P | L724 | atore ¦ un enum con una sola variante oggi … [[il 12]] aggiunge il gesto con un indice nuovo, **sen | — | SP 12 (citazione del disegno) |
| voci P | L735 |  il caso. **Richiamo datato sulla riga**, al [[compito 6]]. | ✓ | 6 = registro |
| voci P | L771 | ndici varianti» è FALSO contro l'enum che il [[compito 3]] detta, e vive in otto case dentro questo fil | ✓ | 3 = schema |
| voci P | L774 | compito**. Trovato preparando il [[compito 7]], che consuma quelle varianti per nome. | ✓ | 7 = serving |
| voci P | L781 | ¦ i bracci dell'enum che il **[[compito 3]] detta** ¦ il blocco `pub enum IpcMessage` de | ✓ | 3 = schema |
| voci P | L784 | ⛔ **E il [[compito 3]] si contraddice da solo:** il suo criterio di | ✓ | 3 = schema |
| voci P | L790 | ipeta:** l'intestazione della §4 del disegno [[del 2]] dice *«approvata il | — | milestone / disegno 2, non un compito |
| voci P | L798 | nel criterio di chiusura del [[compito 3]], accanto a quello delle fixture che già c'è. | ✓ | 3 = schema |
| voci P | L800 | ⛔ **Corretto nel [[compito 3]] e non con una voce d'errata**, come **P-23** | ✓ | 3 = schema |
| voci P | L800 | oce d'errata**, come **P-23** e **P-25**: il [[compito 3]] **non è | ✓ | 3 = schema |
| voci P | L801 | uoi quattordici bracci sono quelli che la §4 [[del 2]] e la §2 | — | milestone / disegno 2, non un compito |
| voci P | L821 | in the kernel»*, e la voce aperta 7 della §9 [[del 2]] assegna | — | milestone / disegno 2, non un compito |
| voci P | L822 | **[[al 3]]** il confine di sessione dei permessi. Un `k | — | SP 3 |
| voci P | L823 | occuperebbe il nome che [[il 3]] vuole. | — | SP 3 |
| voci P | L830 | La §5 [[del 2]] dice che l'attività *«dorme un **tick** cons | — | milestone / disegno 2, non un compito |
| voci P | L852 | Il [[compito 3]] detta `pub struct PolicyReport { policy, all | ✓ | 3 = schema |
| voci P | L860 | abella *Stato*, contro il blocco dettato dal [[compito 3]] ¦ | ✓ | 3 = schema |
| voci P | L869 | Il [[compito 3]] detta `pub outcome: Option<bool>`, col doc * | ✓ | 3 = schema |
| voci P | L870 | contro il giornale che il [[compito 6]] scrive: | ✓ | 6 = registro |
| voci P | L877 |  «chiuso male». E dentro `Registry::invoke` ([[compito 6]]) un | ✓ | 6 = registro |
| voci P | L882 | ✅ **Corretto nel [[compito 3]] e non con una voce d'errata**, come P-23, P- | ✓ | 3 = schema |
| voci P | L882 |  voce d'errata**, come P-23, P-25 e P-35: il [[compito 3]] **non è | ✓ | 3 = schema |
| voci P | L886 | ### P-40 — il limite di giri è del [[compito 9]], non di questo | ✓ | 9 = daemon |
| voci P | L889 | rates/daemon/src/main.rs`, che è il file del [[compito **9**]], e la §5 del 2 detta per esso una sonda prec | ✓ | 9 = daemon |
| voci P | L889 | s`, che è il file del compito **9**, e la §5 [[del 2]] detta per esso una sonda precisa: | — | milestone / disegno 2, non un compito |
| voci P | L891 | afo con la GUI**, cioè il cablaggio, cioè il [[compito 9]]: al 7 nulla lancia l'attività in | ✓ | 9 = daemon |
| voci P | L891 | GUI**, cioè il cablaggio, cioè il compito 9: [[al 7]] nulla lancia l'attività in | ✓ | 7 = serving |
| voci P | L894 | lettera**, che spostò la suite di conformità [[dal 4]] al 5 perché una suite ne | ✓ | 4 = settima porta (tratto) |
| voci P | L894 | a**, che spostò la suite di conformità dal 4 [[al 5]] perché una suite ne | ✓ | 5 = custody impl |
| voci P | L895 | vuole due implementazioni e [[al 4]] ce n'erano zero. Stessa forma, stesso rimedi | ✓ | 4 = settima porta (tratto) |
| voci P | L900 | La §5 [[del 2]] dice che `Accepted` porta *«la protezione de | — | milestone / disegno 2, non un compito |
| voci P | L901 | consegnato)»*. Letto contro il [[compito 3]], che detta il tipo: | ✓ | 3 = schema |
| voci P | L919 | La §7 [[del 2]] lascia questo esplicitamente aperto, nei deb | — | milestone / disegno 2, non un compito |
| voci P | L922 | iro o dopo ogni scrittura, lo fissa il piano [[del 2]] con la sonda della §8»*. | — | milestone / disegno 2, non un compito |
| voci P | L939 | ll'indietro, e ha pagato una terza volta: il [[compito **6**]] è scritto, e ciò che | ✓ | 6 = registro |
| voci P | L940 | detta rende il [[compito 7]] **non scrivibile** su una delle due strade d | ✓ | 7 = serving |
| voci P | L942 | `Registry::invoke` ([[compito 6]]) apre così: | ✓ | 6 = registro |
| voci P | L965 | vergono nulla diventa rosso. È ciò che il §7 [[del 2]] rifiuta per il core finto, con le stesse par | — | milestone / disegno 2, non un compito |
| voci P | L967 | ✅ **La cura è nel [[compito 6]], non in un'errata** — il compito 6 **non è e | ✓ | 6 = registro |
| voci P | L967 | ura è nel compito 6, non in un'errata** — il [[compito 6]] **non è eseguito**, come P-23, P-25, P-35 e | ✓ | 6 = registro |
| voci P | L972 | nel [[compito 3]]. | ✓ | 3 = schema |
| voci P | L1025 | Conseguenza: D26**, la specie `Policy`, e il [[compito 8]] la porta col suo lettore. ⚠️ **Non è una ria | ✓ | 8 = specie Policy |
| voci P | L1027 | §9 [[del 2]] — lo dicono con la data. | — | milestone / disegno 2, non un compito |
| voci P | L1029 | i su `RecordKind` sono QUATTRO e non due: il [[compito 6]], così com'è scritto, NON COMPILA | ✓ | 6 = registro |
| voci P | L1032 | nando `reconcile.rs` e `frozen_bytes.rs`; il [[compito **6**]] | ✓ | 6 = registro |
| voci P | L1059 | ✅ **Corretto nel [[compito 6]] e in P-33, non con una voce d'errata** — com | ✓ | 6 = registro |
| voci P | L1063 | ⚠️ **E vale per il [[compito 8]] identico**, che aggiunge la **ottava** varia | ✓ | 8 = specie Policy |
| voci P | L1064 | lui, e la lista *Files* del [[compito 8]] li nomina tutti e quattro. 📌 **La lezione, o | ✓ | 8 = specie Policy |
| voci P | L1068 | ### P-46 — il [[compito 8]] rende ROSSO il banco del compito 7, e la cur | ✓ | 8 = specie Policy |
| voci P | L1068 | P-46 — il compito 8 rende ROSSO il banco del [[compito 7]], e la cura è nel compito 8 | ✓ | 7 = serving |
| voci P | L1068 | OSSO il banco del compito 7, e la cura è nel [[compito 8]] | ✓ | 8 = specie Policy |
| voci P | L1071 | ura ha scritto fra le trappole. Il banco del [[compito 7]], | ✓ | 7 = serving |
| voci P | L1092 | ⚠️ **E non è un difetto del [[compito 7**]], che è **giusto quando viene eseguito**: al  | ✓ | 7 = serving |
| voci P | L1092 | , che è **giusto quando viene eseguito**: al [[compito 7]] `set_policy` scrive | ✓ | 7 = serving |
| voci P | L1093 | o li conta bene. Diventa falso il giorno del [[compito 8]]. ⛔ **Quindi la cura NON è correggere il | ✓ | 8 = specie Policy |
| voci P | L1094 | [[compito 7**]] — sarebbe un'attesa scritta contro un codice | ✓ | 7 = serving |
| voci P | L1095 | meno una scadenza in prosa** dentro il banco [[del 7]], che è gotcha **#77**. La cura è | ✓ | 7 = serving |
| voci P | L1096 | /serving.rs` entri nella lista *Files* del **[[compito 8**]], con un passo che lo aggiorna e il | ✓ | 8 = specie Policy |
| voci P | L1099 | ### P-47 — il disegno [[del 2]] è LF e TRE posti lo dichiarano CRLF: l'unica | — | milestone / disegno 2, non un compito |
| voci P | L1116 | é, e sta nel file che TRE compiti aprono** — [[il 6]] nella lista *Files* e nel | ✓ | 6 = registro |
| voci P | L1117 | suo passo dei richiami, [[il 7]] nel passo delle quattro case. ⚠️ **E il comp | ✓ | 7 = serving |
| voci P | L1117 | il 7 nel passo delle quattro case. ⚠️ **E il [[compito 7]] la riporta anche nell'ATTESA del suo | ✓ | 7 = serving |
| voci P | L1128 | ✅ **Corretto nei [[compiti 6 e 7]] e non con una voce d'errata** — come P-23, P | ✓ | 6 = registro, 7 = serving |
| voci P | L1129 | etta sbagliata** perché nessun compito prima [[del 8]] la | ✓ | 8 = specie Policy |
| voci P | L1130 | apre; il [[compito 8]] la marca **LF**, misurata. | ✓ | 8 = specie Policy |
| voci P | L1185 | ¦ `serve` ([[compito 7]]) ¦ `serve<'a, I, J, C, R>(core: &'a RefCell< | ✓ | 7 = serving |
| voci P | L1195 | i legge invece di inventarla:** il banco del [[compito 7]] dichiara `SharedClock { inner: | ✓ | 7 = serving |
| voci P | L1198 | es/simulator/tests/arbiter_campaign.rs`; col [[compito 7]] e con questo | ✓ | 7 = serving |
| voci P | L1201 | assegnata [[al **10]]**, che è il primo a poterne misurare il biso | ✓ | 10 = campagna DST |
| voci P | L1207 | compilerebbe.** La decisione **35** della §8 [[del 2]] dice: archivio che | — | milestone / disegno 2, non un compito |
| voci P | L1213 | ¦ `FileCustody::open` ([[compito 5]]) ¦ `open(path: &Path) -> Result<FileCustody, | ✓ | 5 = custody impl |
| voci P | L1214 | ¦ `Core::new` ([[compito 7]]) ¦ `new(ipc, journal, custody: C, arbiter, s | ✓ | 7 = serving |
| voci P | L1216 | che RISOLVE il caso esiste già, misurata nel [[compito 7**]] — la traduzione della lettura fallita in | ✓ | 7 = serving |
| voci P | L1225 | Quindi la cura NON è in `kernel` e NON è nel [[compito 5**]], e le due vie scartate lo dicono: | ✓ | 5 = custody impl |
| voci P | L1235 | §8 [[del 2]] aveva già previsto con quelle parole. **Cons | — | milestone / disegno 2, non un compito |
| voci P | L1242 | ue banchi non lo condividono.** Il banco del [[compito 7]] monta | ✓ | 7 = serving |
| voci P | L1259 | [[del 7]] lo conferma dall'altro lato: il suo `TICK` * | ✓ | 7 = serving |
| voci P | L1260 |  tick è un parametro CONSEGNATO** (ADR-0034, [[compito 7]]), quindi una sonda può | ✓ | 7 = serving |
| voci P | L1264 | grafo vero. Quella metà resta del [[compito **10**]], con la campagna, dove il clock è virtuale e | ✓ | 10 = campagna DST |
| voci P | L1272 | ita — lo dichiara il blocco *Interfaces* del [[compito 7**]] — quindi | ✓ | 7 = serving |
| voci P | L1274 | costruzione: il banco [[del 7]] lo asserisce come comportamento atteso e non | ✓ | 7 = serving |
| voci P | L1282 | E il pari non si inventa qui:** il banco del [[compito 2]] lo costruisce già — un thread che si collega | ✓ | 2 = ipc |
| voci P | L1290 | ### P-53 — D9 lascia al [[compito 9]] DUE numeri, e nessun documento li nomina | ✓ | 9 = daemon |
| voci P | L1293 | non è questo piano: lo consegnano il daemon ([[compito 9]]) e | ✓ | 9 = daemon |
| voci P | L1294 | il core finto ([[compito 12]]), e il compito 9 dice da dove»*. Censito il  | ✓ | 12 = core finto |
| voci P | L1294 | il core finto (compito 12), e il [[compito 9]] dice da dove»*. Censito il 2026-09-14 che co | ✓ | 9 = daemon |
| voci P | L1301 | /src/ipc.rs` **non esiste ancora** (nasce al [[compito 2]]), e la sua firma dettata è | ✓ | 2 = ipc |
| voci P | L1303 | tetto**: la §3 [[del 2]] descrive il trasporto, la §6a descrive il po | — | milestone / disegno 2, non un compito |
| voci P | L1311 |  al core, e il core finto ne sceglie uno suo [[al 12]] ¦ | ✓ | 12 = core finto |
| voci P | L1317 | ta il 2026-09-14 la tabella della posizione: [[il **12]]** porta il core finto — che **lega** un nome | ✓ | 12 = core finto |
| voci P | L1318 | suo, non si collega a questo — e [[il **13]]** e il **14** portano la SPA, che *«non tocc | ✓ | 13 = SPA cornice |
| voci P | L1318 | suo, non si collega a questo — e il **13** e [[il **14]]** portano la SPA, che *«non tocca mai un soc | ✓ | 14 = SPA moduli |
| voci P | L1319 | legarsi è il **preload del guscio**, e la §8 [[del 2]] mette il capo a capo nel guscio *«fuori | — | milestone / disegno 2, non un compito |
| voci P | L1333 |  e tre — `interprocess` entra nel grafo al **[[compito 2**]], con `platform` — | ✓ | 2 = ipc |
| voci P | L1334 | quindi al [[compito 9]] sarà in `platform` e **non** in `daemon`. ⛔  | ✓ | 9 = daemon |
| voci P | L1335 | ` è il lato **listener**, e nessuna riga del [[compito 2]] espone un modo di **collegarsi**; farlo | ✓ | 2 = ipc |
| voci P | L1372 | ### P-56 — il blocco *Interfaces* del [[compito 7]] nomina TRE accessori e il codice che detta n | ✓ | 7 = serving |
| voci P | L1374 | a quinta volta.** Il blocco *Interfaces* del [[compito 7]] | ✓ | 7 = serving |
| voci P | L1375 | apre dicendo *«Produces, e i [[compiti **9**]], **10** e **12** li usano con questi nomi es | ✓ | 9 = daemon |
| voci P | L1377 | del [[compito 7]]: | ✓ | 7 = serving |
| voci P | L1393 | non è scrivibile.** Il doc che il [[compito 7]] gli scrive accanto **nomina questo chiamante | ✓ | 7 = serving |
| voci P | L1396 | l'accessore esiste, e il **contratto** che i [[compiti 9]], 10 e 12 | ✓ | 9 = daemon |
| voci P | L1396 |  esiste, e il **contratto** che i compiti 9, [[10 e 12]] | ✓ | 10 = campagna DST, 12 = core finto |
| voci P | L1403 | ✅ **Corretto nel [[compito 7]] e non con una voce d'errata** — come P-23, P | ✓ | 7 = serving |
| voci P | L1404 | [[compito 7]] **non è eseguito**. Un'errata è per ciò che  | ✓ | 7 = serving |
| voci P | L1409 | di **AUD-019** applicata all'attività. La §5 [[del 2]] dice | — | milestone / disegno 2, non un compito |
| voci P | L1411 | isurato il 2026-09-14 contro il codice che i [[compiti 7]] e la finta portano: | ✓ | 7 = serving |
| voci P | L1420 | il dispaccio del [[compito 7]] lo incontra così: `IpcMessage::Request(_) => | ✓ | 7 = serving |
| voci P | L1458 | cisione che **D29** e la testa del banco del [[compito 7]] hanno entrambe REGISTRATO per questo compito | ✓ | 7 = serving |
| voci P | L1469 | ¦ `crates/kernel/tests/serving.rs` ([[compito 7]]) ¦ `VirtualReactor` ¦ ✅ sì — `simulator` è d | ✓ | 7 = serving |
| voci P | L1470 | ¦ questa campagna ([[compito 10]]) ¦ `VirtualReactor` ¦ ✅ sì ¦ | ✓ | 10 = campagna DST |
| voci P | L1471 | ¦ `crates/daemon/src/main.rs` ([[compito 9]]) ¦ `SystemReactor` ¦ ⛔ **no**, e non per una | ✓ | 9 = daemon |
| voci P | L1493 | [[compito 12]]»*. ⛔ **Quindi una finta consegnata al core n | ✓ | 12 = core finto |
| voci P | L1505 |  prestiti mutabili dello stesso valore**. Il [[compito 7]] scioglie la stessa | ✓ | 7 = serving |
| voci P | L1514 |  `Core::ipc` per giunta **è già assegnato al [[compito 12**]], con un altro chiamante e un altro significa | ✓ | 12 = core finto |
| voci P | L1518 | e tiene un prestito — la forma del banco del [[compito 7]], terza scrittura. Da | ✓ | 7 = serving |
| voci P | L1557 | mappa», la §6a e la §8 dicono **JSON**, e il [[compito 3]] ha scritto solo la prima metà | ✓ | 3 = schema |
| voci P | L1559 | tto è sbagliato, e compila.** Il Passo 6 del [[compito 3]] scrive per ogni variante un | ✓ | 3 = schema |
| voci P | L1587 | **, e il **richiamo del 2026-09-14 dentro il [[compito 3**]], che non è eseguito — il precedente | ✓ | 3 = schema |
| voci P | L1623 | direzioni»*. Il [[compito 11]] ci appoggia sopra il **prerequisito dell'amb | ✓ | 11 = gui nasce |
| voci P | L1656 | tto si chiude verde, e la §8 resta com'è; il [[compito 11]] porta la sonda. | ✓ | 11 = gui nasce |
| voci P | L1658 | N soddisfa `jsdom`, e con `engine-strict` il [[compito 11]] nasce rosso | ✓ | 11 = gui nasce |
| voci P | L1682 | n `engine-strict=true` il primo `npm ci` del [[compito 11]] esce **`EXIT=1`** con | ✓ | 11 = gui nasce |
| voci P | L1692 | ### P-66 — `node_modules/` nasce al [[compito 11]] e `.gitignore` lo copre solo al 15: il commi | ✓ | 11 = gui nasce |
| voci P | L1692 | e al compito 11 e `.gitignore` lo copre solo [[al 15]]: il commit dell'11 non può chiudere pulito | ✓ | 15 = cancello web |
| voci P | L1695 | abella della posizione mette `.gitignore` al [[compito **15**]], con | ✓ | 15 = cancello web |
| voci P | L1696 | il passo del cancello; ma è il [[compito **11**]] che lancia `npm install` e `npm run build`,  | ✓ | 11 = gui nasce |
| voci P | L1704 | e è `i/lf w/crlf`. Senza quelle due righe il [[compito 11]] finisce con un | ✓ | 11 = gui nasce |
| voci P | L1709 | `/gui/dist/` nascono col [[compito 11]]; `/gui/fake-core/target/` resta al **12**, c | ✓ | 11 = gui nasce |
| voci P | L1709 | l compito 11; `/gui/fake-core/target/` resta [[al **12]]**, che è quando quella cartella | ✓ | 12 = core finto |
| voci P | L1715 | ie di taglio che **D25** ha già disfatto sul [[compito 8]]. | ✓ | 8 = specie Policy |
| voci P | L1722 | lo non si compila senza il plugin, quindi il [[compito 11]] installerebbe un `vite` che | ✓ | 11 = gui nasce |
| voci P | L1771 | [[del 12]]:** il blocco *Interfaces* del compito **9**  | ✓ | 12 = core finto |
| voci P | L1771 | del 12:** il blocco *Interfaces* del [[compito **9**]] lo dice già — *«la campagna del 10 rifà lo s | ✓ | 9 = daemon |
| voci P | L1771 | el compito **9** lo dice già — *«la campagna [[del 10]] rifà lo stesso cablaggio | ✓ | 10 = campagna DST |
| voci P | L1772 | su porte finte, e il core finto [[del 12]] lo rifà in `gui/fake-core`; entrambi lo risc | ✓ | 12 = core finto |
| voci P | L1776 | *, e il richiamo datato nella §7 del disegno [[del 2]]. | — | milestone / disegno 2, non un compito |
| voci P | L1806 | che la §7 lasciava al piano è GIÀ CHIUSO dal [[compito 7]], e il 12 lo esercita soltanto | ✓ | 7 = serving |
| voci P | L1806 | ciava al piano è GIÀ CHIUSO dal compito 7, e [[il 12]] lo esercita soltanto | ✓ | 12 = core finto |
| voci P | L1810 | iro o dopo ogni scrittura, lo fissa il piano [[del 2]] con la sonda | — | milestone / disegno 2, non un compito |
| voci P | L1811 | della §8»*. **Lo ha già fissato il [[compito 7]].** Misurato il 2026-09-14 sul piano: | ✓ | 7 = serving |
| voci P | L1817 | Il [[compito 7]] rilegge `degradation_now` a ogni giro entro  | ✓ | 7 = serving |
| voci P | L1820 | te da correggere:** senza di essa chi esegue [[il 12]] legge la | ✓ | 12 = core finto |
| voci P | L1825 | **Conseguenza:** nessuna `D` — [[il 12]] **esercita** e non costruisce, e la sonda de | ✓ | 12 = core finto |
| voci P | L1832 | [[compito 7]] la fissa già, ed è **`Core` intero**, non il | ✓ | 7 = serving |
| voci P | L1840 | esiste [[dal 7]], e **`Core::ipc()`, che nasce qui**, come il | ✓ | 7 = serving |
| voci P | L1840 |  e **`Core::ipc()`, che nasce qui**, come il [[compito 7]] scrive per esteso (*«il suo chiamante è il | ✓ | 7 = serving |
| voci P | L1841 | rubinetto del core finto, [[compito 12]]»*). | ✓ | 12 = core finto |
| voci P | L1893 | Il [[compito 9]] detta `const SOCKET_NAME: &str = "harness-co | ✓ | 9 = daemon |
| voci P | L1945 | ### P-75 — Il blocco *Interfaces* del [[compito 11]] dice `{ file, kind, value }` e il suo Passo  | ✓ | 11 = gui nasce |
| voci P | L1945 | il suo Passo 9 detta `{ file, message }`: il [[compito 13]] non compilerebbe | ✓ | 13 = SPA cornice |
| voci P | L1955 | ¦ il blocco *Interfaces* del [[compito 11]] ¦ `interface Fixture { file: string; kind: s | ✓ | 11 = gui nasce |
| voci P | L1961 | nd types neighboring tasks use»* — quindi un [[compito 13]] scritto contro di esso userebbe | ✓ | 13 = SPA cornice |
| voci P | L1965 | :** `kind` e `value` sono i nomi che la **§8 [[del 2]]** usa a parole per la | — | milestone / disegno 2, non un compito |
| voci P | L1970 | ⛔ **Corretto nel [[compito 11]] e non con una voce d'errata**, come **P-35** | ✓ | 11 = gui nasce |
| voci P | L1970 | ta**, come **P-35**, **P-56** e **P-62**: il [[compito 11]] **non | ✓ | 11 = gui nasce |
| voci P | L1975 | ### P-76 — La finta del [[compito 11]] «non inventa nulla», e le fixture portano UN | ✓ | 11 = gui nasce |
| voci P | L1975 | ano UNO dei tre stati di `Layout`: due sonde [[del 13]] non sono raggiungibili con essa | ✓ | 13 = SPA cornice |
| voci P | L1977 | la sonda manca, e non si vede leggendo:** il [[compito 13]] deve provare che *«archivio vuoto → | ✓ | 13 = SPA cornice |
| voci P | L1979 |  disponibile»»* (decisione 35, riga della §8 [[del 2]]). Misurato il 2026-09-14 contro il testo | — | milestone / disegno 2, non un compito |
| voci P | L1980 | dettato dei [[compiti 3 e 11]]: | ✓ | 3 = schema, 11 = gui nasce |
| voci P | L1984 | locco `pub enum LayoutState` del Passo 2 del [[compito 3]] ¦ **tre**: `Package`, `Nothing`, `Unavailabl | ✓ | 3 = schema |
| voci P | L1986 | che cosa consegna la finta ¦ il Passo 12 del [[compito 11]] ¦ *«Delivers the fixture of that kind»* — qu | ✓ | 11 = gui nasce |
| voci P | L2005 | ominano tipi di modulo, e il SEGNAPOSTO è al [[compito 14]]: la cornice del 13 non avrebbe niente da mon | ✓ | 14 = SPA moduli |
| voci P | L2005 |  e il SEGNAPOSTO è al compito 14: la cornice [[del 13]] non avrebbe niente da montare | ✓ | 13 = SPA cornice |
| voci P | L2008 | posizione dà [[al **13]]** *«le tre viste come JSON»* e al **14** *«i | ✓ | 13 = SPA cornice |
| voci P | L2008 | ne dà al **13** *«le tre viste come JSON»* e [[al **14]]** *«il segnaposto»*. Misurato il 2026-09-14  | ✓ | 14 = SPA moduli |
| voci P | L2018 | ⛔ **Quindi [[al 13]] la cornice si monta e non mostra nulla, e il | ✓ | 13 = SPA cornice |
| voci P | L2019 | che compila. E la riga della §8 [[del 2]] che prova questo artefatto pretende **tre co | — | milestone / disegno 2, non un compito |
| voci P | L2019 | ova questo artefatto pretende **tre cose che [[al 13]] non sarebbero | ✓ | 13 = SPA cornice |
| voci P | L2025 | diciotto»* — quindi spostarlo non porta peso [[al 13]]: porta la sola cosa | ✓ | 13 = SPA cornice |
| voci P | L2026 | io giusto è per artefatto, non per parola:** [[il 14]] costruisce i | ✓ | 14 = SPA moduli |
| voci P | L2033 | nza: D47**, e il richiamo datato nelle righe [[13 e 14]] della tabella della posizione. | ✓ | 13 = SPA cornice, 14 = SPA moduli |
| voci P | L2035 | non sono quattro messaggi, e il `Bridge` del [[compito 11]] non ne esprime nessuno | ✓ | 11 = gui nasce |
| voci P | L2037 | de solo scrivendone l'uso da fuori.** La §6a [[del 2]] pretende | — | milestone / disegno 2, non un compito |
| voci P | L2039 | 2026-09-14 contro il contratto del [[compito 11]] e l'enum del compito 3: | ✓ | 11 = gui nasce |
| voci P | L2039 | tro il contratto del compito 11 e l'enum del [[compito 3]]: | ✓ | 3 = schema |
| voci P | L2045 | suna run ¦ **no**: è l'assenza di `Token`, e [[nel 2]] il produttore è il solo core finto ¦ | — | milestone / disegno 2, non un compito |
| voci P | L2051 | e il [[compito 11]] scrive per esteso perché è di due metodi: *« | ✓ | 11 = gui nasce |
| voci P | L2062 | ece di restare vuota»*. Sta nel modulo Chat, [[compito **14**]]; la cornice non lo conosce. | ✓ | 14 = SPA moduli |
| voci P | L2066 | URATO nello spike, e la lista di lettura del [[compito 13]] non lo nominava | ✓ | 13 = SPA cornice |
| voci P | L2069 |  lista di lettura dell'ottava chiusura manda [[il 13]] alla §6a, alla §1 e | ✓ | 13 = SPA cornice |
| voci P | L2070 | a GUI», alla §9 e al blocco *Interfaces* del [[compito 11]]. **Non | ✓ | 11 = gui nasce |
| voci P | L2080 |  Che cosa lo spike ha già ¦ Perché conta per [[il 13]] ¦ | ✓ | 13 = SPA cornice |
| voci P | L2086 | l ripiego che divide il gruppo — ⚠️ ed è del [[compito **14**]], non del 13 ¦ | ✓ | 14 = SPA moduli |
| voci P | L2086 |  il gruppo — ⚠️ ed è del compito **14**, non [[del 13]] ¦ | ✓ | 13 = SPA cornice |
| voci P | L2093 | ([[nel 2]] la casa è la settima porta, I1). | — | milestone / disegno 2, non un compito |
| voci P | L2097 | ew-core` non spedisce il foglio di stile: il [[compito 13]] installa DUE pacchetti, e il richiamo è già  | ✓ | 13 = SPA cornice |
| voci P | L2112 | richiamo è già nella §2 del disegno [[del 2]]** (decisione 37 della settima chiusura della | — | milestone / disegno 2, non un compito |
| voci P | L2122 | ### P-81 — **D3** manda al [[compito 13]] la rilettura di `markdown-it`, e contro la t | ✓ | 13 = SPA cornice |
| voci P | L2122 | tro la tabella della posizione il renderer è [[al 14]] | ✓ | 14 = SPA moduli |
| voci P | L2129 | ¦ la decisione **D3** ¦ *«il [[compito **13**]] rilegge quelle tre proprietà dentro il `.tgz | ✓ | 13 = SPA cornice |
| voci P | L2130 | ¦ la tabella «Che cosa aspetta ora il [[compito 13]]» dell'ottava chiusura ¦ *«il renderer nasce  | ✓ | 13 = SPA cornice |
| voci P | L2130 | » dell'ottava chiusura ¦ *«il renderer nasce [[col **13]]/14**»* — già incerta ¦ | ✓ | 13 = SPA cornice |
| voci P | L2134 | ⛔ **Quindi [[il 13]] rileggerebbe dentro un pacchetto che non ins | ✓ | 13 = SPA cornice |
| voci P | L2135 | `markdown-it` fra ciò che *«arriva coi [[compiti 13]], 14 e 15, che li consumano»* — e il consumat | ✓ | 13 = SPA cornice |
| voci P | L2135 | own-it` fra ciò che *«arriva coi compiti 13, [[14 e 15]], che li consumano»* — e il consumatore è la  | ✓ | 14 = SPA moduli, 15 = cancello web |
| voci P | L2136 | Installarlo [[al 13]] sarebbe la dipendenza senza consumatore che  | ✓ | 13 = SPA cornice |
| voci P | L2155 | ¦ §6a [[del 2]], riga «la cornice» ¦ *«la barra delle viste  | — | milestone / disegno 2, non un compito |
| voci P | L2172 | ### P-83 — `locales/it.json` nasce [[al 13]] e la regola di lint entra al 15: fra i due n | ✓ | 13 = SPA cornice |
| voci P | L2172 | .json` nasce al 13 e la regola di lint entra [[al 15]]: fra i due nulla la fa rispettare | ✓ | 15 = cancello web |
| voci P | L2174 | è «niente»:** la riga delle scritte della §8 [[del 2]] chiede *«un controllo che | — | milestone / disegno 2, non un compito |
| voci P | L2177 | `gate-gui.sh` e la catena `eslint` sono il [[compito **15**]], e **D40** li elenca fra ciò che arriva «coi | ✓ | 15 = cancello web |
| voci P | L2177 |  e **D40** li elenca fra ciò che arriva «coi [[compiti 13]], | ✓ | 13 = SPA cornice |
| voci P | L2178 | [[14 e 15]]». | ✓ | 14 = SPA moduli, 15 = cancello web |
| voci P | L2180 | ⛔ **Quindi i [[compiti 13 e 14]] scrivono ogni scritta e nessun controllo le  | ✓ | 13 = SPA cornice, 14 = SPA moduli |
| voci P | L2180 | e nessun controllo le guarda**, e chi rivede [[il 13]] cercherebbe | ✓ | 13 = SPA cornice |
| voci P | L2183 |  e non è la catena `eslint` anticipata:** il [[compito 13]] porta una | ✓ | 13 = SPA cornice |
| voci P | L2184 |  e va rossa su una scritta nel modello, e il [[compito 15]] la | ✓ | 15 = cancello web |
| voci P | L2186 | dichiarata per ciò che è — una **rete** fino [[al 15]] — o | ✓ | 15 = cancello web |
| voci P | L2189 | int`, `eslint-plugin-vue` e il plugin `i18n` [[al 13]] | ✓ | 13 = SPA cornice |
| voci P | L2190 | significherebbe scrivere [[al 13]] anche la configurazione che il 15 deve scriv | ✓ | 13 = SPA cornice |
| voci P | L2190 | e scrivere al 13 anche la configurazione che [[il 15]] deve scrivere comunque per il cancello — due | ✓ | 15 = cancello web |
| voci P | L2191 |  gotcha **#68** conta. La sonda nostra muore [[al 15]] e non lascia niente. | ✓ | 15 = cancello web |
| voci P | L2198 | on `G->>C: Hello (il timbro di build)`, e il [[compito 3]] detta | ✓ | 3 = schema |
| voci P | L2199 | Stamp)`. Misurato il 2026-09-14 su ciò che i [[compiti 3 e 11]] producono: | ✓ | 3 = schema, 11 = gui nasce |
| voci P | L2203 | ¦ `gui/src/schema/messages.ts` ([[compito 11]]) ¦ i **tipi**, e `U64 = string`. Nessun valo | ✓ | 11 = gui nasce |
| voci P | L2204 | ¦ `loadFixtures()` ([[compito 11]]) ¦ globa i soli `*.json`, e `ipc_v1.map` **n | ✓ | 11 = gui nasce |
| voci P | L2206 | ¦ `interface Bridge` ([[compito 11]]) ¦ `send` e `listen`. Nessun membro che lo p | ✓ | 11 = gui nasce |
| voci P | L2207 | ¦ `ipc_v1.map` ([[compito 3]]) ¦ ✅ **l'ultima riga**, `stamp 0x…` con `{:# | ✓ | 3 = schema |
| voci P | L2226 | e non nomina IMPOSTAZIONI, e con essa la SPA [[del 2]] non manda mai un `Invoke` | — | milestone / disegno 2, non un compito |
| voci P | L2228 | ierlo è il censimento dei tipi di modulo del [[compito 13**]], non una | ✓ | 13 = SPA cornice |
| voci P | L2234 | ¦ §6a [[del 2]], riga «il cambio di policy» ¦ *«nel modulo * | — | milestone / disegno 2, non un compito |
| voci P | L2235 |  tabella corta ¦ *«Impostazioni │ **2**, poi [[3 e 10]] │ nel 2 il cambio di policy VRAM, funzione d | — | SP 3 e 10 (tabella corta) |
| voci P | L2235 | corta ¦ *«Impostazioni │ **2**, poi 3 e 10 │ [[nel 2]] il cambio di policy VRAM, funzione del regis | — | milestone / disegno 2, non un compito |
| voci P | L2240 | delle Impostazioni. Senza di esso, nella SPA [[del 2]] | — | milestone / disegno 2, non un compito |
| voci P | L2241 | rmissionRequired`, e la finestra di conferma [[del 14]] non ha chi la apra: | ✓ | 14 = SPA moduli |
| voci P | L2244 | ollo a due stati e un `Invoke`. ⚠️ **E non è [[del 13]]:** il 13 è la cornice, e | ✓ | 13 = SPA cornice |
| voci P | L2244 |  stati e un `Invoke`. ⚠️ **E non è del 13:** [[il 13]] è la cornice, e | ✓ | 13 = SPA cornice |
| voci P | L2245 | e tipo con `who: 2` come tutti gli altri — è [[il **14]]** che lo costruisce, con | ✓ | 14 = SPA moduli |
| voci P | L2246 | gli altri moduli [[del 2]]. | — | milestone / disegno 2, non un compito |
| voci P | L2255 | trasto AA dei token non lo prova, e un token [[del 13]] lo fallisce | ✓ | 13 = SPA cornice |
| voci P | L2258 | asto AA dei token: i valori del `tokens.css` [[del 13]] sono segnaposto dichiarati, e a provarli è | ✓ | 13 = SPA cornice |
| voci P | L2293 |  la sonda nostra, lanciata a mano sui valori [[del 13]], trova un rosso:** `--stop` `#e5534b` su `-- | ✓ | 13 = SPA cornice |
| voci P | L2294 | dà **4,21** — sotto 4,5 — ed è la coppia che [[il 13]] disegna (`.chip[data-phase="stale"]` sulla b | ✓ | 13 = SPA cornice |
| voci P | L2318 | enza: D53**, il valore nuovo di `--stop` nel [[compito 14]], e il paragrafo in testa a `tokens.css` corr | ✓ | 14 = SPA moduli |
| voci P | L2347 |  girata in avanti, sulla specie di D45.** Il [[compito 7]] detta `POLICY_FUNCTION` con `name: "vram-pol | ✓ | 7 = serving |
| voci P | L2358 | UTO, più che per il nome del canale:** la §5 [[del 2]] dice che *«una funzione non registrata è | — | milestone / disegno 2, non un compito |
| voci P | L2363 | due sorgenti Rust, e il criterio di chiusura [[del 14]] li confronta con un | ✓ | 14 = SPA moduli |
| voci P | L2369 | — La riga 1 della tabella Passi promette per [[il 2]] cinque campi, e `StepSummary` ne porta tre:  | — | milestone / disegno 2, non un compito |
| voci P | L2369 | e campi, e `StepSummary` ne porta tre: la §4 [[del 2]] e il compito 3 dicono «intento ed esito», la | — | milestone / disegno 2, non un compito |
| voci P | L2369 | `StepSummary` ne porta tre: la §4 del 2 e il [[compito 3]] dicono «intento ed esito», la stella polare  | ✓ | 3 = schema |
| voci P | L2376 | lla stella polare, tabella Passi, riga 1 ¦ *«[[nel 2]] le invocazioni del registro — **funzione, in | — | milestone / disegno 2, non un compito |
| voci P | L2377 | ¦ §4 [[del 2]], riga `Steps` (richiamo del 2026-09-09) ¦ *« | — | milestone / disegno 2, non un compito |
| voci P | L2377 | -09-09) ¦ *«la lista dei passi dal giornale: [[nel 2]] le invocazioni del registro **con intento ed | — | milestone / disegno 2, non un compito |
| voci P | L2378 | ¦ il [[compito 3]], `pub struct StepSummary` ¦ `step: u64`, `fu | ✓ | 3 = schema |
| voci P | L2379 | ¦ il [[compito 7]], `step_list` ¦ legge il dettaglio `Invocatio | ✓ | 7 = serving |
| voci P | L2388 | due campi non sono equivalenti nel merito:** [[nel 2]] l'invocatore è **uno** (la GUI) e la | — | milestone / disegno 2, non un compito |
| voci P | L2397 | ¦ **A** ¦ [[il 14]] mostra i tre campi e scrive il **richiamo da | ✓ | 14 = SPA moduli |
| voci P | L2398 | rgument: String` con la sua provenienza, nel [[compito 3]] — non eseguito — e con esso le fixture, il t | ✓ | 3 = schema |
| voci P | L2398 | lo specchio TypeScript dell'11 e `step_list` [[del 7]] ¦ quattro compiti scritti da toccare, e un t | ✓ | 7 = serving |
| voci P | L2402 | ### P-90 — Il revisore [[del 14]] deve GUARDARE con dei dati, e nel browser la | ✓ | 14 = SPA moduli |
| voci P | L2402 | UARDARE con dei dati, e nel browser la finta [[del 13]] non consegna niente: `createFakeBridge` rigi | ✓ | 13 = SPA cornice |
| voci P | L2405 | dei [[compiti 13 e 14]] nel browser. Misurato il 2026-09-15 sul test | ✓ | 13 = SPA cornice, 14 = SPA moduli |
| voci P | L2405 | el browser. Misurato il 2026-09-15 sul testo [[del 13]]: `main.ts` costruisce `createFakeBridge()`, | ✓ | 13 = SPA cornice |
| voci P | L2407 | scia *«il core non ha risposto»*. Quindi con [[il 13]] com'è, il revisore del 14 vedrebbe cinque | ✓ | 13 = SPA cornice |
| voci P | L2407 | posto»*. Quindi con il 13 com'è, il revisore [[del 14]] vedrebbe cinque | ✓ | 14 = SPA moduli |
| voci P | L2414 | **Nessuna riga** nel testo dettato [[del 13]]: le tre righe della sonda degli store sono d | ✓ | 13 = SPA cornice |
| voci P | L2426 | ### P-91 — Il 13 consegna [[al 14]] TRE comandi del menu del modulo, e il terzo  | ✓ | 14 = SPA moduli |
| voci P | L2428 | anda 5 in avanti, contro P-53.** Il Passo 11 [[del 13]] scrive che i comandi *«stacca»*, *«pagina in | ✓ | 13 = SPA cornice |
| voci P | L2429 | spike *«appartengono al menu del modulo, che [[il 14]] disegna con l'accessibilità»*. Misurato | ✓ | 14 = SPA moduli |
| voci P | L2430 | `.d.ts` di `dockview-core` 8.3.1 e nel testo [[del 13]]: | ✓ | 13 = SPA cornice |
| voci P | L2437 | oupOptions` con `popoutUrl`; il `createDock` [[del 13]] **non** ne imposta uno | ✓ | 13 = SPA cornice |
| voci P | L2444 | metà, e lo dice: *«in un pannello libero»* è [[del 14]], | ✓ | 14 = SPA moduli |
| voci P | L2449 | ha provocato (decisione 21), e nessuno store [[del 13]] tiene l'ultimo `Invoke` mandato: `core.pendi | ✓ | 13 = SPA cornice |
| voci P | L2452 | , con la funzione e il suo argomento)`, e il [[compito 3]] detta `Approve { triple, call }`. | ✓ | 3 = schema |
| voci P | L2453 | surato il 2026-09-15 sul blocco *Interfaces* [[del 13]]: `useCore()` porta `pending: Triple ¦ null`  | ✓ | 13 = SPA cornice |
| voci P | L2460 | s`** — non un campo in più in `core.ts`, che [[il 13]] ha scritto come | ✓ | 13 = SPA cornice |
| voci P | L2471 | di focus** (G20); la tabella Permessi dice *«[[nel 2]] la finestra con | — | milestone / disegno 2, non un compito |
| voci P | L2472 | ocus»* (riga 9) e la tabella Chat *«finestra [[nel 2]], in riga dal 3»* (riga 3). Una finestra moda | — | milestone / disegno 2, non un compito |
| voci P | L2472 |  e la tabella Chat *«finestra nel 2, in riga [[dal 3]]»* (riga 3). Una finestra modale | — | SP 3 |
| voci P | L2479 | Reka, verificati per nome [[dal 13]] — più `DialogDescription`, che sta nello ste | ✓ | 13 = SPA cornice |
| voci P | L2490 | ### P-94 — Il flusso non finisce mai [[nel 2]], e il compito 13 non fissa dove un messaggio | — | milestone / disegno 2, non un compito |
| voci P | L2490 | P-94 — Il flusso non finisce mai nel 2, e il [[compito 13]] non fissa dove un messaggio si chiude: i num | ✓ | 13 = SPA cornice |
| voci P | L2492 | nda 2, nella forma di P3.** Il rubinetto del [[compito 12]] gira su `SCRIPT` **per sempre** (Passo 6: `l | ✓ | 12 = core finto |
| voci P | L2493 | `piece % SCRIPT.len()`), e sul filo [[del 2]] non esiste un confine di messaggio — la run  | — | milestone / disegno 2, non un compito |
| voci P | L2493 | on esiste un confine di messaggio — la run è [[del 3]]. Un testo che cresce | — | SP 3 |
| voci P | L2509 | ### P-95 — Il titolo della linguetta [[nel 13]] è l'ID inglese del pannello: le tre viste sc | ✓ | 13 = SPA cornice |
| voci P | L2511 | girata all'indietro, su G21.** Il generatore [[del 13]] detta `api.addPanel({ id, component: id, tit | ✓ | 13 = SPA cornice |
| voci P | L2512 | ers.title ?? parameters.api.id`: il revisore [[del 13]] vedrebbe linguette | ✓ | 13 = SPA cornice |
| voci P | L2513 | knowledge`. Misurato il 2026-09-15 sul testo [[del 13]]: | ✓ | 13 = SPA cornice |
| voci P | L2522 | ✅ **Corretto nel [[compito 13]] e non con una voce d'errata**, come **P-75** | ✓ | 13 = SPA cornice |
| voci P | L2522 |  non con una voce d'errata**, come **P-75**: [[il 13]] non è eseguito. `BigTab` mostra il nome | ✓ | 13 = SPA cornice |
| voci P | L2523 |  — `isModule` e `i18n.global.t` esistono già [[nel 13]] — e il | ✓ | 13 = SPA cornice |
| voci P | L2526 | nseguenza:** nessuna `D` — è una correzione; [[il 14]] riscrive comunque `BigTab` per i comandi e l | ✓ | 14 = SPA moduli |
| voci P | L2528 | vue/no-v-html` è nel preset raccomandato che [[il 15]] accende: il 15 deve scrivere l'eccezione, o  | ✓ | 15 = cancello web |
| voci P | L2528 | è nel preset raccomandato che il 15 accende: [[il 15]] deve scrivere l'eccezione, o la Chat nasce c | ✓ | 15 = cancello web |
| voci P | L2530 | ⛔ **Domanda 5 in avanti, sul [[compito 15]].** L'HTML che la Chat inserisce è **nostro** | ✓ | 15 = cancello web |
| voci P | L2543 | na `D`; una riga in «Che cosa aspetta ora il [[compito 15]]» della chiusura. | ✓ | 15 = cancello web |
| voci P | L2558 | `D`; la forma della sonda `keys.test.ts` nel [[compito 14]]. | ✓ | 14 = SPA moduli |
| voci P | L2560 | con una scritta grezza nel template: la rete [[del 13]] andrebbe sostituita da un controllo VACUO | ✓ | 13 = SPA cornice |
| voci P | L2563 |  **D51** dice che la rete `copy.test.ts` del [[compito 13]] *«muore al 15, sostituita da | ✓ | 13 = SPA cornice |
| voci P | L2563 | a rete `copy.test.ts` del compito 13 *«muore [[al 15]], sostituita da | ✓ | 15 = cancello web |
| voci P | L2581 | ⛔ **Quindi la rete [[del 13]] va rosso e la sua sostituta no**, e il compi | ✓ | 13 = SPA cornice |
| voci P | L2581 | el 13 va rosso e la sua sostituta no**, e il [[compito 15]] così com'è nominato consegnerebbe un | ✓ | 15 = cancello web |
| voci P | L2587 | ` che questo piano scrive sono a una parola: [[il 15]] farebbe ROSSO il codice dei compiti 13 e 14 | ✓ | 15 = cancello web |
| voci P | L2587 | na parola: il 15 farebbe ROSSO il codice dei [[compiti 13 e 14]] | ✓ | 13 = SPA cornice, 14 = SPA moduli |
| voci P | L2618 | `PANEL_TYPES` ([[compito 13]]) ed **è** la chiave `modules.*` della locale | ✓ | 13 = SPA cornice |
| voci P | L2618 | ed **è** la chiave `modules.*` della locale ([[compito 14]]). Un rinomino per compiacere un | ✓ | 14 = SPA moduli |
| voci P | L2676 | [[compito 13]]. | ✓ | 13 = SPA cornice |
| voci P | L2680 | ue dei quattro pezzi che la chiusura assegna [[al 15]] sono eseguiti | ✓ | 15 = cancello web |
| voci P | L2683 | piano la risposta è sì.** La §8 [[del 2]] assegna al cancello *«per `spikes/gui-shell/ | — | milestone / disegno 2, non un compito |
| voci P | L2693 | he 34–41, messe da **`8fc9696`**, cioè dal **[[compito 4]] della parte 1**; e | ✓ | 4 = settima porta (tratto) |
| voci P | L2697 |  nomi al piano quando esistono»*). ⛔ **Ma il [[compito 15]] non deve toccarli**, o riscriverebbe righe e | ✓ | 15 = cancello web |
| voci P | L2698 |  il suo diff direbbe il falso. Ciò che resta [[al 15]] di `.gitignore` è **niente**: le due righe d | ✓ | 15 = cancello web |
| voci P | L2699 | [[compito 11]] (**D38**) e `/gui/fake-core/target/` del 12. | ✓ | 11 = gui nasce |
| voci P | L2699 | pito 11 (**D38**) e `/gui/fake-core/target/` [[del 12]]. | ✓ | 12 = core finto |
| voci P | L2719 | ⚠️ **Il `gui/package.json` del [[compito 11]] non porta NESSUNO dei due campi**, quindi og | ✓ | 11 = gui nasce |
| voci P | L2733 | tta prima della misura è un'ipotesi.** La §9 [[del 2]] dichiara 🔶 **dedotto** | — | milestone / disegno 2, non un compito |
| voci P | L2735 |  già fuori dal workspace, senza aspettare il [[compito 12]] — e senza compilare niente: | ✓ | 12 = core finto |
| voci P | L2747 |  e `/gui/fake-core/target/` in `.gitignore` ([[compito 12]]) è la riga giusta. | ✓ | 12 = core finto |
| voci P | L2749 | la porta da «dedotto» a «misurato» lo scrive [[il **17]]**, con le | ✓ | 17 = chiusura |
| voci P | L2752 | ssuna `D`; una riga nel criterio di chiusura [[del 15]]. | ✓ | 15 = cancello web |
| voci P | L2757 | si accorge di aver perso.** Il Passo 18 del [[compito 13]] scrive in `gui/src/locales/copy.test.ts` **d | ✓ | 13 = SPA cornice |
| voci P | L2759 | ¦ ¦ La sonda ¦ Chi la sostituisce [[al 15]] ¦ | ✓ | 15 = cancello web |
| voci P | L2779 | ✅ **Quindi [[il 15]] uccide la PRIMA sonda e lascia viva la secon | ✓ | 15 = cancello web |
| voci P | L2780 | *«una rete fino al [[compito 15]]»* ma il controllo delle diciotto chiavi cost | ✓ | 15 = cancello web |
| voci P | L2781 |  `gui/src/frame/keys.test.ts` esiste già dal [[compito 14]] e due `keys.test.ts` in due | ✓ | 14 = SPA moduli |
| voci P | L2886 | ### P-110 — `npm audit` sull'insieme VERO [[del 2]]: trecentosessantanove pacchetti, **zero** vu | — | milestone / disegno 2, non un compito |
| voci P | L2889 | lato nello scratchpad l'insieme intero che i [[compiti 11]], 13, | ✓ | 11 = gui nasce |
| voci P | L2890 | [[14 e 15]] mettono in `gui/package.json` — `vue`, `dock | ✓ | 14 = SPA moduli, 15 = cancello web |
| voci P | L2931 | ⛔ **Quindi il criterio di chiusura [[del 16]] manda a GUARDARE la corsa**, con l'indirizzo | ✓ | 16 = X-1/X-3 |
| voci P | L2933 | [[compiti 13 e 14]] manda il revisore nel browser. | ✓ | 13 = SPA cornice, 14 = SPA moduli |
| voci P | L2944 | [[compito 17]], scritta nella decima chiusura, dice *«le ca | ✓ | 17 = chiusura |
| voci P | L2980 | vendo — **D21** ha portato il limite di giri [[dal 7]] al 9, **D25** ha diviso la vecchia riga 8, | ✓ | 7 = serving |
| voci P | L2980 | — **D21** ha portato il limite di giri dal 7 [[al 9]], **D25** ha diviso la vecchia riga 8, | ✓ | 9 = daemon |
| voci P | L2981 | **D47** ha spostato il segnaposto [[dal 14]] al 13 — e la cifra della roadmap era rimasta | ✓ | 14 = SPA moduli |
| voci P | L2981 | **D47** ha spostato il segnaposto dal 14 [[al 13]] — e la cifra della roadmap era rimasta a que | ✓ | 13 = SPA cornice |
| voci P | L3033 | ina NESSUNO dei due disegni né dei due piani [[del 2]], ed è voluto: le righe le deve questo compit | — | milestone / disegno 2, non un compito |
| voci P | L3042 | (**P-112**). Quindi al [[compito 17]] spettano righe **nuove**, non ritocchi: | ✓ | 17 = chiusura |
| voci P | L3046 | ** — la stella polare della GUI e il disegno [[del 2]] — nella forma delle righe che ci sono: *«⛔ N | — | milestone / disegno 2, non un compito |

Righe: 625; ✗: 4; ✓: 477; —: 144

## (b) La tabella dei file — `file → (compito, azione, etichetta)` contro `git ls-files --eol` di oggi

Legenda: **C** = Create, **M** = Modify; l'etichetta è quella scritta nel *Files* del compito (— = assente); la misura è `git ls-files --eol <file>` lanciata in un comando solo su tutti i file esistenti.

| File | Compiti (azione · etichetta) | Misurato oggi | Esito |
|---|---|---|---|
| `crates/kernel/src/numbering.rs`, `crates/kernel/tests/numbering.rs` | 1 C · LF | nuovi | ✓ |
| `crates/kernel/src/lib.rs` | 1 M · CRLF; 6 M · `i/lf w/crlf`; 7 M · — | `i/lf w/crlf` | ✓ (7 senza etichetta, R10-16) |
| `crates/kernel/src/ports/journal.rs`, `crates/kernel/src/ports/ipc.rs` | 1 M · CRLF | `i/lf w/crlf` | ✓ |
| `crates/kernel/src/framing.rs` | 2 M · CRLF | `i/lf w/crlf` | ✓ |
| `crates/kernel/tests/framing.rs` | 2 M · — | `i/lf w/crlf` | etichetta assente (R10-16) |
| `crates/platform/src/ipc.rs`, `crates/kernel/tests/ipc_contract.rs`, `crates/platform/tests/ipc_contract_real.rs` | 2 C · LF | nuovi | ✓ |
| `crates/platform/src/lib.rs` | 2 M · CRLF; 5 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `crates/platform/Cargo.toml` | 2 M · CRLF | `i/lf w/crlf` | ✓ |
| `Cargo.lock` | 2 M · CRLF; 9 M · — | `i/lf w/crlf` | ✓ (9 senza etichetta, R10-16) |
| `crates/kernel/src/ports/mod.rs` | 2 M · `i/lf w/crlf`; 4 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `crates/kernel/tests/ports_are_implementable.rs` | 2 M · `i/crlf w/crlf`; 4 M · `i/crlf w/crlf` | `i/crlf w/crlf` | ✓ (P-3) |
| `crates/kernel/src/wire/ipc.rs` | 3 M · `i/lf w/crlf`; 7 M · — («dal compito 3») | `i/lf w/crlf` | ✓ |
| `crates/kernel/tests/ipc_wire.rs` | 3 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `gui/schema/fixtures/*.bin`, `*.json`, `ipc_v1.map` | 3 C · LF (`.bin` binari) | nuovi | ✓ (11 li legge, 13 legge `ipc_v1.map`) |
| `crates/kernel/src/ports/custody.rs` | 4 C · LF | nuovo | ✓ |
| `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | 4 M · CRLF | `i/lf w/crlf` | ✓ |
| `crates/simulator/src/custody.rs`, `crates/kernel/tests/custody_contract.rs`, `crates/platform/src/custody.rs`, `crates/platform/tests/custody_contract_real.rs`, `crates/platform/tests/file_custody.rs` | 5 C · LF | nuovi | ✓ |
| `crates/simulator/src/lib.rs` | 5 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `crates/platform/src/journal.rs` | 5 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `crates/kernel/src/record.rs`, `crates/kernel/src/reconcile.rs`, `crates/kernel/tests/reconciliation.rs`, `crates/kernel/tests/frozen_bytes.rs` | 6 M · `i/lf w/crlf`; 8 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `crates/kernel/src/registry.rs` | 6 C · LF; 7 M · — («dal compito 6») | nuovo | ✓ |
| `crates/kernel/tests/registry.rs` | 6 C · LF | nuovo | ✓ |
| `crates/kernel/tests/frozen/record_v1_invocation.cbor` | 6 C · (binario) | nuovo | ✓ |
| `crates/kernel/tests/frozen/record_v1.map` | 6 M · —; 8 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ (6 senza etichetta, R10-16) |
| `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | 6 M · LF; 7 M · LF; 8 M · LF; 9 M · LF; 10 M · (`i/lf w/lf` nel Passo, nessun *Files*); 12 M · `i/lf w/lf`; 15 M · LF | `i/lf w/lf` | ✓ |
| `crates/kernel/src/serving.rs` | 7 C · LF; 12 M · LF («dal compito 7», `Core::ipc`) | nuovo | ✓ |
| `crates/kernel/tests/serving.rs` | 7 C · LF; 8 M · LF («dal compito 7», P-46) | nuovo | ✓ |
| `crates/kernel/src/parameters.rs`, `crates/kernel/src/executor.rs` | 7 M · — (i Passi dicono `i/lf w/crlf`) | `i/lf w/crlf` | etichetta assente nel *Files* (R10-16) |
| i ventitré chiamanti di `Parameters::new` e i nove `.stderr` | 7 M | — | non censiti uno per uno (R2/R4) |
| `docs/superpowers/plans/2026-09-11-…-gui-minima.md` (questo piano) | 7 M; 8 M; 9 M; 17 M · LF (e i commit di 1–6, 10–12 lo aggiungono) | `i/lf w/lf` | ✓ |
| `crates/kernel/src/arbiter/mod.rs`, `crates/kernel/tests/arbiter_policy.rs`, `crates/kernel/tests/record_shape.rs`, `crates/simulator/tests/dst_campaign.rs` | 8 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `crates/kernel/tests/frozen/record_v1_policy.cbor` | 8 C · (binario) | nuovo | ✓ |
| `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` | 8 M · LF; **14 M · — (assente dal *Files*, ma Passo 15 e `git add`)** | `i/lf w/lf` | R10-8 |
| `crates/daemon/src/main.rs`, `crates/daemon/Cargo.toml` | 9 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `crates/simulator/tests/serving_campaign.rs` | 10 C · LF (dal Passo 2: nessun *Files*) | nuovo | R10-18 |
| `scripts/gate.sh` | 10 M · `i/lf w/crlf` (Passo 7); **15 M · LF**; **16 M · LF** | `i/lf w/crlf` (CR 97 = righe 97) | ✗ 15 e 16 — R10-1, R10-2 |
| `gui/package.json`, `gui/package-lock.json` | 11 C · LF; 13 M · LF; 14 M · LF; 15 M · LF | nuovi | ✓ |
| `gui/.npmrc`, `gui/tsconfig.json`, `gui/vite.config.ts`, `gui/index.html` | 11 C · LF (`vite.config.ts` 13 M · LF) | nuovi | ✓ |
| `gui/src/main.ts`, `gui/src/App.vue` | 11 C · LF; 13 M · LF (riscritti); 14 M · LF (`main.ts`) | nuovi | ✓ |
| `gui/src/schema/messages.ts`, `parse.ts`, `fixtures.ts`, `schema.test.ts` | 11 C · LF | nuovi | ✓ |
| `gui/src/transport/bridge.ts`, `fakeBridge.ts`, `fakeBridge.test.ts` | 11 C · LF | nuovi | ✓ |
| `.gitignore` | 11 M · `i/lf w/crlf`; 12 M · `i/lf w/crlf`; (13 `git add` senza modifica, R10-17) | `i/lf w/crlf` | ✓ |
| `Cargo.toml` (radice) | 12 M · `i/lf w/crlf` | `i/lf w/crlf` | ✓ |
| `gui/fake-core/Cargo.toml`, `gui/fake-core/Cargo.lock`, `gui/fake-core/src/main.rs` | 12 C · LF | nuovi | ✓ (15 legge `Cargo.toml` e `Cargo.lock`) |
| `gui/src/schema/stamp.ts` | 13 C · LF (nella cartella dell'11, D52) | nuovo | ✓ |
| `gui/src/tokens/tokens.css` | 13 C · LF; 14 M · LF (`--stop`) | nuovo | ✓ |
| `gui/src/locales/it.json`, `gui/src/i18n.ts` | 13 C · LF; 14 M · LF (`it.json`) | nuovi | ✓ |
| `gui/src/stores/connection.ts`, `core.ts`, `layout.ts`, `stores.test.ts` | 13 C · LF | nuovi | ✓ |
| `gui/src/frame/VueContent.ts`, `dock.ts`, `ViewBar.vue`, `Band.vue`, `Drawer.vue`, `frame.test.ts` | 13 C · LF | nuovi | ✓ |
| `gui/src/frame/BigTab.ts`, `gui/src/frame/Frame.vue` | 13 C · LF; 14 M · LF | nuovi | ✓ |
| `gui/src/panels/registry.ts`, `Placeholder.vue`, `Strip.vue` | 13 C · LF | nuovi | ✓ |
| `gui/src/panels/views/home.json`, `work.json`, `compact.json`, `index.ts`, `views.test.ts` | 13 C · LF | nuovi | ✓ |
| `gui/src/panels/views/generate-views.test.ts` | **13 C · LF nel Passo, assente dal *Files*** | nuovo | R10-9 |
| `gui/src/locales/copy.test.ts` | 13 C · LF; 15 M · LF (muore la prima sonda) | nuovo | ✓ |
| `gui/src/panels/functions.ts`, `modules.ts`, `Status.vue`, `Permissions.vue`, `Steps.vue`, `Settings.vue`, `Chat.vue` | 14 C · LF | nuovi | ✓ |
| `gui/src/stores/stream.ts`, `invoke.ts`; `gui/src/components/markdown.ts`, `Confirm.vue`; `gui/src/frame/moveActive.ts`; le nove sonde del 14 | 14 C · LF | nuovi | ✓ |
| `scripts/gate-gui.sh` | 15 C · LF; 16 M · LF (`npm audit` in coda) | nuovo | ✓ |
| `gui/eslint.config.js` | 15 C · LF | nuovo | ✓ |
| `.github/workflows/quality-gate.yml` | **15 M · LF; 16 M · LF** | `i/lf w/crlf` (CR 16 = righe 16) | ✗ — R10-1, R10-2 |
| `docs/audit-2026-08-27.md` | **16 M · LF** (il Passo dice `i/lf w/crlf`) | `i/lf w/crlf` (CR 1885 = righe 1885) | ✗ — R10-2 |
| `docs/COMPENDIO.md`, `docs/README.md`, `docs/roadmap.md`, `docs/tracciabilita.md`, `docs/porta-di-qualita.md`, `docs/riferimenti.md`, `docs/HANDOFF.md` | 17 M · CRLF | `i/lf w/crlf` tutte e sette | ✓ (P-114) |

Nessun file **creato da due compiti**; nessun file **modificato prima di essere creato** (ogni Modify su un file nuovo segue il compito che lo crea: `serving.rs` 7→12, `tests/serving.rs` 7→8, `registry.rs` 6→7, `gate-gui.sh` 15→16, `BigTab.ts`/`Frame.vue`/`tokens.css`/`it.json`/`copy.test.ts` 13→14/15, `main.ts`/`App.vue`/`vite.config.ts`/`package.json` 11→13); nessun **percorso scritto in due modi** (`gui/schema/fixtures/` è la casa unica: l'11 vi arriva con `../../schema/fixtures/*.json`, il 13 con `../../schema/fixtures/ipc_v1.map?raw`). Le sole etichette di fine-riga **false** sono le sei dei compiti 15 e 16 su tre file (R10-1, R10-2); sei voci sono **senza** etichetta (R10-16); il compito 10 non ha il blocco (R10-18).

## (d) I conteggi — ogni comando della tabella «Stato alla chiusura» dell'undicesima chiusura, rilanciato

| Comando | Resa scritta | Resa mia (2026-09-15, `baf3cde`) | Esito |
|---|---|---|---|
| `git status -sb` | `## main...origin/main`, niente sotto | `## main...origin/main`, niente sotto (⚠️ `git fetch --all --prune` **non lanciato**: scrive in `.git`) | ✓ |
| `git stash list` | vuoto | vuoto | ✓ |
| `git log --oneline 83afc58..HEAD` | i commit della sessione | 6 commit: `baf3cde`, `2f546cb`, `850137c`, `3abf7b1`, `ea74387`, `22e5712` | ✓ |
| `git diff --name-only 83afc58..HEAD` | anche `docs/roadmap.md` | `docs/roadmap.md` + il piano | ✓ |
| `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` | non rende nulla | vuoto | ✓ |
| `grep -c '^## Compito' <piano>` / righe della posizione | coincidono | **17** / **17** (righe 133–149); su tutto il file `^\| \*\*N\*\* \|` → 19 per le due righe di P-22 (R10-15) | ✓ |
| `awk '/^## ⚠️ L.errata …/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}'` | 0 | **0**; nell'altra direzione, su un file con una riga `¦ **E1** ¦` sotto la stessa intestazione → **1** | ✓ |
| `grep -c '^### P-' <piano>` | 116 | **116** | ✓ |
| `grep -c '^\| \*\*D[0-9]' <piano>` | 74 | reso (`'^| …'`): **74**; **copiato grezzo dalla cella: 20129** (R10-14) | ✓ / ✗ grezzo |
| `bash scripts/gate.sh` | `GATE GREEN` | `GATE GREEN.` (`probe-R10/gate-R10.log`, EXIT=0) | ✓ |
| `bash scripts/check-docs.sh` | `OK` | `OK — no inconsistencies.` | ✓ |
| `git ls-files --eol <piano>` / `tr -cd '\r' < <piano> ¦ wc -c` | `i/lf w/lf` / 0 | `i/lf w/lf` / **0**; i due disegni `i/lf w/lf` | ✓ |
| tabelle spezzate (`awk 'prev ~ /^\|/ && $0 == "" {getline nxt; …}'`) | niente | niente | ✓ |
| `grep -n '<the version' <piano>` | uno solo | **5** righe: 13015 + le quattro chiusure che citano il comando (R10-13) | ✗ |
| `<data>` / `<tempo>` (non sono segnaposto) | (`constraints.md`: 34 / 4) | `grep -o '<data>' ¦ wc -l` → **34**; `grep -o '<tempo>' ¦ wc -l` → **7** (`grep -c` rende 4 **righe**: il conteggio di `constraints.md` è per righe, non per occorrenze — la trappola che lo stesso file descrive) | ✓ (piano); ⚠️ constraints |
| `awk '/^## Come si riprende/{exit} /node -e "…"/{c++} END{print c+0}'` | 0 | **0** (su tutto il file `grep -c` → 4, tutte nel diario) | ✓ |
| `grep -cE '^\s*(it¦describe)\([^)]*\(\) => \{\}\)'` | 0 | reso: **0**; grezzo dalla cella (`(it\|describe)`): **0 anche dove deve rendere 1** — vacuo (R10-14) | ✓ / ✗ grezzo |
| `todo!()` / `unimplemented!()` | — | `grep -c 'todo!()\|unimplemented!()'` → **0** | ✓ |
| margine del compendio | `11030`, invariato | `ceiling=111616` − `wc -c docs/COMPENDIO.md` 100586 = **11030** | ✓ |
| `git status --porcelain` | vuoto | vuoto | ✓ |
| «Le decisioni prese scrivendo» 55–67 ↔ D63–D74 | riga per riga | 55→D63, 56→D64, 57→D65, 58→D65+P-105, 59→D66, 60→D67, 61→D68, 62→D69, 63→D70, 64→D71, 65→D72, 66→D73, 67→D74: ogni D esiste e dice la stessa cosa della riga | ✓ |
| numerazione 1–67 attraverso le undici chiusure (`grep -n '^\| [0-9]\+ \|'` nel diario) | continua | 1–4 (prima), 5–10, 11–15, 16–19, 20–22, 23–26, 27–30, 31–36, 37–44, 45–54, 55–67 (undicesima): **nessun buco, nessun doppione** | ✓ |

## (e) Citazioni di `D`, `P` ed `E`

| Che cosa | Comando → resa | Esito |
|---|---|---|
| ogni `D<n>` citato esiste | `grep -o '\bD[0-9]\+\b' <piano> ¦ sort -u` → **74** unici, tutti in 1..74, nessuno fuori; tutti citati almeno due volte (D6, D7, D12, D17 due volte) | ✓ |
| ogni `P-<n>` citato esiste | `grep -o 'P-[0-9]\+' <piano> ¦ sort -u` → **116** unici, tutti in 1..116, nessuno fuori | ✓ |
| `D` di altri piani col nome del piano | `grep -n 'D[0-9]\+ della parte 1\|del piano della parte 1\|piano dei gesti\|knowledge base'` → riga 15 «D14 della parte 1», 3041 (P-116) «D14 del piano della parte 1», 20106 (prima chiusura) «D14 della parte 1»; riga 92 «Della parte 1 … (D13 e D14 sono ereditate qui)» nomina la parte nel contesto | ✓ |
| `E` di altri piani col nome | con nome: E2 (×5, «della parte 1»/«of the part-1 plan»), E4 (×5), E6 (×2, «del piano dei gesti»/«part-1 plan»), E12 («del Traguardo 6»), E26 («del piano del Traguardo 5»), E50/E51/E100 («del Traguardo 5», riga 3153 e 20044), E115 (citazione della decisione 56); **nudi**: E25 ×4, E66 ×2, E112 ×2, E79 ×4 (2 in commenti di codice), E94 ×2 (1 in codice), E41 ×1 (*Trova* di un commento esistente), E4 ×1 (riga 19136), E50 ×2 («errata `E50`» a 1055 e 10046, che citano il sorgente) | ✗ nudi — R10-12 |
| `E<n>` nudi: quante case | `grep -l '^\| \*\*E25\*\* ' docs/superpowers/plans/*.md` → 5; E66 → 4; E79, E94, E112 → 2 (Traguardi 5 e 6) | ambigui |
| «piano dei gesti» / «knowledge base» | righe 32, 86 (E6) / 32, 18631, 18634 (link al disegno) | ✓ |

## (f) I blocchi *Interfaces* contro i propri Passi — 3, 7, 11, 13 (conteggio con ancora di riga)

| Compito · file | Comando → elementi pubblici dettati | Nel blocco *Interfaces* | Esito |
|---|---|---|---|
| 7 · `crates/kernel/src/serving.rs` (8718–9308) | `grep -nE '^\s*pub (async )?(const )?(fn¦struct¦enum¦const¦trait¦type)'` → `POLICY_FUNCTION`, `struct Core<I: Ipc, J: Journal, C: Custody>`, `new`, `journal`, `arbiter`, `custody`, `grants`, `attending`, `async fn serve<'a, I, J, C, R>(core: &'a RefCell<Core<I, J, C>>, clock: &'a R, sleep: &'a Sleep)` — **9** | `Core` + `new` (6 argomenti), i **cinque** accessori, `serve`, `POLICY_FUNCTION` — 9 | ✓ (P-56 regge) |
| 7 · `executor.rs` (7951–7986), `parameters.rs` (7735–7770), `registry.rs` (7874–7934) | `pub async fn nap(sleep: &Sleep, deadline: Monotonic)`; `pub const fn new(` + `pub const fn gui_tick(self) -> Millis`; `pub enum Approval` + `pub fn held(&self, name: &str) -> Option<Function>` | `nap`, `Parameters::new(executor_turn_limit, total_vram, arbiter_id, gui_tick)`, `gui_tick`, «i due pezzi … al passo 4» | ✓ |
| 3 · `crates/kernel/src/wire/ipc.rs` (4245–4439, 4451–4544) | `pub struct BuildStamp(u64)` + `pub const fn get`; `Protection`, `DegradationReport`, `PolicyName`, `PolicyReport`, `Triple`, `Access`, `Call`, `Provenance`, `LayoutState`, `StepSummary` (**10**); `pub enum IpcMessage`; `pub fn stamp_set() -> Vec<IpcMessage>`; `pub fn build_stamp() -> BuildStamp` | `BuildStamp`+`get`, i dieci tipi (D11: «dieci tipi nuovi»), le varianti «quante siano lo dice il comando», `stamp_set`, `build_stamp` | ✓ |
| 11 · `messages.ts` (12270–12352) | `^export` → `U64`, `Protection`, `PolicyName`, `Access`, `Provenance`, `ComputeClass`, `DegradationReport`, `PolicyReport`, `Triple`, `Call`, `StepSummary`, `LayoutState`, `Preemption`, `GrantRequest`, `Verdict`, `IpcMessage` — **16** | i sedici nomi, identici | ✓ |
| 11 · `parse.ts` (12362–12551) | `class SchemaError`, `const MESSAGE_KINDS: readonly IpcMessage["kind"][]`, `function parseIpcMessage(raw: unknown): IpcMessage` — 3 | 3, identici | ✓ |
| 11 · `fixtures.ts` (12556–12588) | `interface Fixture { file: string; message: IpcMessage }`, `function loadFixtures(): Fixture[]` — 2 | `interface Fixture { file: string; kind: string; value: unknown }` | ✗ — R10-7 |
| 11 · `bridge.ts` (12669–12701) | `type OutboundMessage = Extract<IpcMessage, { kind: "Hello" ¦ "Invoke" ¦ "Approve" ¦ "SaveLayout" }>`, `type Listener`, `interface Bridge { send(message: OutboundMessage): void; listen(listener: Listener): () => void }` — 3 | 3, identici | ✓ |
| 11 · `fakeBridge.ts` (12706–12756) | `interface FakeBridge extends Bridge { readonly sent; deliver(kind); deliverAll() }`, `function createFakeBridge(): FakeBridge` — 2 | `createFakeBridge(): FakeBridge` con `sent`, `deliver(kind)`, `deliverAll()` | ✓ |
| 13 · `stores/connection.ts`, `core.ts`, `layout.ts` | `export type Phase`, `useConnection` con `{ phase, protection, expected, attach, hello, retry, receive }`; `useCore` con `{ degradation, policy, steps, pending, lastVerdict, receive, settled }`; `export type ViewName`, `interface LayoutPack`, `useLayout` con `{ state, view, attach, receive, settle }`, `pack_(pack: LayoutPack): Uint8Array`, `unpack(state: LayoutState): LayoutPack ¦ null` | gli stessi membri e le stesse firme; manca `Phase` | ✓ (R10-10 per `Phase`) |
| 13 · `panels/registry.ts` | `interface PanelType { name; module; who: number }`, `PANEL_TYPES` (`{ name: "chat", module: "chat", who: 3 }` …), `register(name, component): void`, `isModule(name): boolean`, `componentFor(name): () => IContentRenderer`, `placeholderParams(name): Record<string, unknown>` — 6 | 6, identici (`who` numero, nessun `built`) | ✓ |
| 13 · `views/index.ts`, `frame/VueContent.ts`, `frame/dock.ts`, `i18n.ts` | `VIEWS: Readonly<Record<ViewName, SerializedDockview>>`; `class VueContent implements IContentRenderer`; `canonical(value: unknown): unknown`, `createDock(host: HTMLElement): DockviewApi`, `apply(api: DockviewApi, view: string, pack: LayoutPack ¦ null): void`; `i18n` | identici (`createDock` senza `Bridge`) | ✓ |
| 13 · `frame/BigTab.ts`, `schema/stamp.ts` | `class BigTab implements ITabRenderer`; `buildStamp(): U64` | assenti dal *Produces* | ✗ — R10-10 |
| consumatori (a) — forme rilanciate con `calls.py` | `Core::new` 6 argomenti nello stesso ordine in 7, 9, 10, 12; `Parameters::new` 4 in 7, 9, 10, 12; `LocalSocketIpc::bound(name, Progressive::starting_at(n), tetto)` in 2, 9, 12; `Registry::invoke` 7 argomenti in 6 e 7; `serve(&core, &clock, &sleep)` in 7, 9, 10, 12; `nap(sleep, deadline)` in 7, 12; `RecordV1::invocation`/`policy` 5 argomenti; `keep(CustodyKey::Layout, bytes)`/`retrieve(CustodyKey::Layout)` in 4, 5, 7, 9; `FileCustody::open(&path)` in 5, 9; `policy_now(&journal)` in 8, 9; `POLICY_FUNCTION { name: "vram-policy", tool: "registry", resource: "arbiter", Operation::Write }` contro il 14 (`registry × arbiter × Write`, `"vram-policy"`, `"remote"`/`"local"`); `createDock(host.value)`, `apply(api, view, unpack(layout.state))`, `moveActive(api, direction)`, `directionOf(event)`, `renderMarkdown(text)`, `componentFor`/`isModule`/`placeholderParams(name)`, `deliver("Kind")`, `deliverAll()`, `settled()`, `approve(): boolean` in 13 e 14 | tutte coincidono | ✓ |

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

Tutti da `/c/Users/zagor/Desktop/harness`; `<piano>` = `docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md`; `<SP>` = `probe-R10/`.

- `wc -l -c skeleton.md p-titles.md <piano>` → 678/82967, 116/15009, 20129/1203205
- `awk '/^## Il pre-controllo del piano/{exit} {print}' <piano> > head.md` → 190 righe; `awk '/^## Le decisioni prese da questo piano/{f=1} /^## Le voci aperte che questo piano SA/{exit} f'` → 88 righe (dtable.md); `awk '/^### L.undicesima chiusura/{f=1} /^### La decima chiusura/{exit} f'` → 128 righe (closure11.md)
- `grep -n '^## ' <piano> | head -60` → le intestazioni: compiti 1–17 a 3169…18540, diario a 18801
- `git ls-files --eol -- <49 file esistenti dei blocchi Files>` → tutti `i/lf w/crlf` tranne `ports_are_implementable.rs` `i/crlf w/crlf`, i sei `.cbor` `-text`, il piano e i due disegni `i/lf w/lf`
- `ls crates/kernel/tests/frozen/` → `record_v1.map` + sei `.cbor`
- `python numeri.py` (prima versione) → crash `cp1252` su ⛔; rilanciato con `PYTHONIOENCODING=utf-8` → head 21, P 387, D 130, C 21, cl11 21 righe
- `python numeri2.py` (tollerante al grassetto) → head 22, P 412, D 141, C 25, cl11 25; `grep -v -x -F -f` → le nuove: head 1, C 4, D 23, cl11 6
- `git status -sb` → `## main...origin/main`; `git stash list` → vuoto; `git log --oneline 83afc58..HEAD` → 6 commit; `git diff --name-only 83afc58..HEAD` → roadmap + piano; `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/` → vuoto
- `grep -c '^## Compito' <piano>` → 17; `grep -c '^| \*\*[0-9][0-9]*\*\* |'` → 19; `grep -n` delle 19 → 133–149 e 506–507 (P-22)
- `awk '…errata…'` → 0; su `errata-probe.md` con una riga `| **E1** |` → 1
- `grep -c '^### P-'` → 116; `grep -c '^\| \*\*D[0-9]'` (grezzo, BRE) → 20129; `grep -c '^| \*\*D[0-9]'` → 74; su `printf '| **D1** | x |\nnessuna\n'`: BRE grezzo → 2, ERE grezzo → 1, reso → 1
- `tr -cd '\r' < <piano> | wc -c` → 0; `awk` delle tabelle spezzate → niente; `grep -n '<the version'` → 5 righe; `grep -c 'node -e "…"'` → 4; `awk '/^## Come si riprende/{exit} /node -e "…"/{c++} END{print c+0}'` → 0; `grep -c "it('…'"` → 0; `grep -c 'todo!()\|unimplemented!()'` → 0
- `printf "  it('x', () => {})\n" | grep -cE '^\s*(it\|describe)\(…'` → 0 (vacuo); con `(it|describe)` → 1; sul piano → 0
- `git status --porcelain | wc -l` → 0; `git ls-files --eol <piano> <disegno2> <stella>` → `i/lf w/lf` ×3
- `awk '/^## Compito 10:/{f=1} /^## Compito 11:/{exit} f' <piano> | grep -n 'Create\|Modify\|nuovo\|\.rs\b\|gate\.sh'` → il file nuovo, `gate.sh` `i/lf w/crlf` con `replace_unique.py`, i `git ls-files --eol` dei Passi 1 e 9
- `awk '/^## Compito 15:/{f=1} /^## Compito 17:/{exit} f' <piano> | grep -n 'replace_unique\|newline=""\|sed -i\|cat >>\|ls-files --eol\|tr -cd'` → gli inserimenti Python, i `tr -cd` a zero, i `git ls-files --eol`
- `sed -n '17550,17572p;17948,17962p;17996,18014p;18164,18168p;17962,17972p;18014,18022p' <piano>` → Passo 1/8/9/chiusura del 15 (Atteso «LF», «zero CR»)
- `sed -n '18196,18214p;18262,18284p;18412,18426p;18436,18450p;18468,18488p;18284,18292p;18426,18432p' <piano>` → Passo 1/2/flusso/audit del 16 («tutti LF», «È CRLF … i/lf w/crlf»)
- `grep -o '\bD[0-9]\+\b' <piano> | sort | uniq -c` → D1…D74 tutti; `| sort -u | wc -l` → 74; fuori 1..74 → nessuno; `grep -o 'P-[0-9]\+' | sort -u | wc -l` → 116, fuori 1..116 → nessuno
- `grep -n 'D[0-9]\+ della parte 1\|del piano della parte 1\|piano dei gesti\|knowledge base' <piano>` → righe 15, 32, 86, 3041, 3115, 3133, 18631, 18634, 20041, 20106
- `grep -o '\bE[0-9]\+\b' <piano> | sort | uniq -c` → E2 5, E4 6, E6 2, E12 2, E25 4, E26 1, E41 1, E50 4, E51 2, E66 2, E79 5, E94 2, E100 2, E112 2, E115 2 (+ codici `rustc` E0004…E0502); `grep -n -o ".\{0,90\}\b$e\b.\{0,60\}"` per ciascuno → i contesti
- `grep -l "^| \*\*$e\*\* " docs/superpowers/plans/*.md docs/*.md` per E2…E115 → E25 in 5 piani, E66 in 4, E79/E94/E112/E100/E115 in 2; `grep -H "^| \*\*$e\*\* " …traguardo-*.md | cut -c1-120` → righe diverse per piano
- `grep -rnoE '\bE(25|41|50|66|79|94|112)\b.{0,50}' crates/ --include='*.rs'` → 20 righe; `grep -rlE … | wc -l` → 12 file
- `awk '/^## Come si riprende/{f=1} f{print NR": "$0}' <piano> | grep '^[0-9]*: | [0-9][0-9]* |'` → 67 righe numerate, 55–67 … 1–4; `grep -n '^### L.\|^### La '` → le undici chiusure (+ la sezione «La regola dei gemelli» del 3)
- `python calls.py <piano> <33 nomi Rust>` → `calls-rust.txt` 304 righe; `python calls.py <piano> <38 nomi TS>` → `calls-ts.txt` 396; `python uses.py <piano>` → `uses.txt` 364; `python blocks.py <piano> 3|7|11|13` → 15/25/27/39 recinzioni
- `sed -n '8718,9308p' <piano> | grep -nE '^\s*pub (async )?(const )?(fn|struct|enum|const|trait|type)'` → 9; `sed -n '7951,7986p'` → `nap`; `sed -n '7735,7770p'` → `new`, `gui_tick`; `sed -n '7874,7934p'` → `Approval`, `held`; `sed -n '4245,4439p;4451,4544p'` → `BuildStamp`+`get`, 10 tipi, `IpcMessage`, `stamp_set`, `build_stamp`
- `sed -n "<intervallo>p" <piano> | grep -nE '^export '` per i cinque file dell'11 e i dieci del 13 → i conteggi della tabella (f)
- `sed -n '12669,12701p;12706,12756p;12556,12588p' <piano>` → `bridge.ts`, `fakeBridge.ts`, `fixtures.ts` (`{ file; message }`)
- `sed -n '8772,8778p;8032,8036p;11262,11283p;14170,14176p' <piano>`; `awk 'NR>=10266 && NR<=11125 && /^\s*use /'` → i `use` del 7, del 10, del 12; il 9 li fa dettare a `cargo build`
- `grep -n 'interface Fixture' <piano>` → 1955, 1956, 11979, 12559, 19081; `awk '/^### P-75 /…'` → il testo di P-75
- `awk '/^## Compito [0-9]+:/{c=$3} /^git add /{print c" L"NR": "$0}' <piano>` → gli `add` di 1–5, 11–17; `awk` su 6553–11956 per `git (add|commit)|posizione` → i commit di 6–10 aggiornano la riga della posizione
- `awk 'NR>=15415 && NR<=17518' <piano> | grep -n 'direzione-gui-design'` → 58, 2037, 2048, 2049, 2076, 2100 (Passo 15 e `git add`); `awk 'NR>=13655 && NR<=15414' | grep -n gitignore` → 13710, 13718, 15369
- `sed -n '18125,18128p' <piano>` → l'`add` intero del 15 (con `copy.test.ts`, il flusso, il disegno)
- `grep -n '11030' <piano>` → 114, 281, 3015, 3018, 3139, 18847, 20049; `grep -n -i 'ceiling' scripts/check-docs.sh` → 346 `ceiling=111616`; `wc -c docs/COMPENDIO.md` → 100586
- `awk` sulle intestazioni `- [ ] **Passo N:` dei compiti 3, 7, 10, 11, 12, 14, 15, 17 → le liste dei Passi
- `awk 'NR>=11126 && NR<=11956 && /RICHIAMO DEL|richiamo datato|DyingGui.*§5|…/'` → Passo 8 del 10 = il richiamo §5 (D32)
- `sed -n '11204,11868p' <piano> | grep -nE '\.(replay|intent|note|outcome|append|read_all|len)\('` → solo `.len()`; `grep -n 'Journal\b'` (senza Crashing/Memory) → niente
- `grep -c 'run "attributes of the constrained crates"' scripts/gate.sh` → 1; `grep -c 'run "documentation consistency"'` → 1; `grep -n '^run "'` → 39–44, 84
- `sed -n '8787,8792p' <piano>` → `POLICY_FUNCTION { name: "vram-policy", tool: "registry", resource: "arbiter", Operation::Write }`
- `awk '/^## Compito [0-9]+:/{c=$3} /8\.2\.0/{print c" L"NR}'` → la nota nel 14 (L16732), nessuna nel 12/13; `awk 'NR>=13655 && NR<=15414 && /SP-8|eight moves|otto mosse/'` → 13850, 14382, 14383, 14429 (nessuna versione)
- `grep -n 'watchdog' <disegno2>` → 279, 443, 510 (§9 voce 8: «il **10**» = sotto-progetto)
- `for n in <63 nomi>; do grep -rnE "^\s*pub(\(crate\))? (…)(fn|struct|enum|trait|const|type) $n\b" crates/*/src; done` → tutti presenti tranne `replay` (metodo di tratto)
- `grep -n '^pub mod\|^pub use' crates/kernel/src/arbiter/mod.rs` → `policy`, `resource`, i due `pub use`; `grep -n '^pub mod' crates/kernel/src/lib.rs`, `crates/platform/src/lib.rs`, `crates/simulator/src/lib.rs`, `crates/kernel/src/ports/mod.rs` → i moduli di oggi; `grep -nE '^\s*pub (fn|struct|enum|const)' crates/kernel/src/wire/ipc.rs` → `GrantRequest`, `Verdict`, `IpcMessage`, `encode`, `decode`
- `bash scripts/check-docs.sh` → `OK — no inconsistencies.`; `bash scripts/gate.sh > gate-R10.log` (in background) → `GATE GREEN.`, EXIT=0
- `git ls-files --eol crates/kernel/tests/ports_are_implementable.rs` → `i/crlf w/crlf`; `grep -c '/gui/' .gitignore` → 0; `grep -c 'matrix\|setup-node' .github/workflows/quality-gate.yml` → 0; `grep -n 'runs-on\|checkout@\|rustup show\|gate.sh' …yml` → 10, 12, 15, 16; `grep -c 'gui-shell' .gitignore` → 8; `git ls-files 'spikes/gui-shell/**package-lock.json' 'spikes/gui-shell/**Cargo.lock' | wc -l` → 4; `grep -c 'sedici compiti\|diciassette compiti' docs/roadmap.md` → 1 (nel richiamo); `grep -c '2026-09-06-sottoprogetto-2\|…' docs/README.md` → 0
- `tr -cd '\r' < scripts/gate.sh | wc -c; wc -l < scripts/gate.sh` → 97/97; `.github/workflows/quality-gate.yml` → 16/16; `docs/audit-2026-08-27.md` → 1885/1885
- `sed -n '231p' docs/roadmap.md | grep -o '…'` → «⏳ **in scrittura dal 2026-09-11** — nessuno eseguito. ⛔ **RICHIAMO DEL 2026-09-15: qui stava «sedici compiti, i primi due scritti»…»; `git show 850137c --stat` → `docs/roadmap.md | 4 ++--`
- `grep -o '<data>' <piano> | wc -l` → 34; `grep -o '<tempo>' | wc -l` → 7; `grep -o '\bD[0-9]\+\b' | sort -u | wc -l` → 74
- `python census_table.py <SP>` → 625 righe: ✗ 4, ✓ 477, — 144 (`census.md`)
- (in chiusura) `git -C /c/Users/zagor/Desktop/harness status --porcelain` → vuoto; `git ls-files --eol` di piano e disegni → `i/lf w/lf`

## Non verificato, e perché

- `git fetch --all --prune` della tabella «Stato alla chiusura»: non lanciato perché scrive in `.git` (`FETCH_HEAD`, i ref remoti); `git status -sb` rende già `## main...origin/main`.
- La copertura dei disegni sezione per sezione: nessuna sezione assegnata a R10; ho letto dei disegni la sola riga del watchdog (§9 voce 8 del 2) per classificare «il 10».
- Il passaggio `w/crlf` → `w/mixed`/`w/lf` dopo gli inserimenti dei compiti 15 e 16: non riprodotto su un file del repository (vietato scrivere); dedotto dal codice dettato (`line + anchor` con `line` LF, `cp` di un file LF) e da come `git ls-files --eol` legge l'albero. Il fatto misurato è che i tre file sono `i/lf w/crlf` oggi e che gli Atteso dicono LF.
- I ventitré chiamanti di `Parameters::new` e i nove `.stderr` del compito 7: non censiti (perimetro R2/R4).
- Il registro npm (P-2, D37, D39) e `crates.io`: non interrogati (perimetro R7/R8/R9).
- I consumatori del contratto **dentro** un solo compito (per esempio `Fixture` letta da `fakeBridge.ts` nell'11) sono stati letti; non ho compilato nessun modello: la coincidenza delle firme è per confronto testuale delle chiamate (`calls.py`), non per `tsc`/`cargo`.
- `Progressive::take` e `.send(`/`.receive(` dei consumatori: nomi troppo generici per un `grep` utile; le firme sono state confrontate solo sul blocco *Interfaces* e sulle definizioni.

## Stato finale
`git status --porcelain` → (vuoto: 0 righe) · `git ls-files --eol` di piano e disegni → `i/lf w/lf;` invariato · repository a `baf3cde`, nessun file toccato; le prove in `probe-R10/`.
