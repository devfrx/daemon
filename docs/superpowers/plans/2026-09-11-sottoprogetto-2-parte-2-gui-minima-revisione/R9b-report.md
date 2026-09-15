# Rapporto R9b — Copertura della STELLA POLARE della GUI contro il piano — 2026-09-15

Perimetro: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (`<stella>`, `i/lf w/lf`, `tr -cd '\r' | wc -c` → 0) — la tabella delle decisioni del proprietario (0–48), «Il modello della GUI», le tre sequenze, §2, §3, §6, «Le sezioni, e dove vive ciascuna», le decisioni del coordinatore (1–57), «Registrate, non prese», «Vicoli ciechi e trappole», «Il prossimo passo» — contro il piano a `HEAD` = `baf3cde`, letto via `skeleton.md`, `p-titles.md`, le righe `D`, le voci `P` e i Passi nominati sotto. Repository non toccato: `git status --porcelain` vuoto all'apertura e alla chiusura.

## Esito in tre righe

1. La stella polare è **coperta nel merito**: i dodici messaggi delle tre sequenze sono tutti varianti del compito 3, rami del 7 e store del 13/14 coi nomi identici; l'ordine dell'accoglienza è asserito nelle due direzioni; la settima porta, le tre viste, il salvataggio automatico, «non disponibile», X-1/X-3, il cancello unico, `engines.node`, i lockfile, `markdown-it`, `vitest`, `jsdom`, `axe-core`, la specie `Policy` hanno tutti un compito e, dove il piano precisa, una `D`.
2. **Due rilievi bloccano**: la disposizione del compito 13 tiene **una sola** vista (`LayoutPack { view; layout }`) e `apply` la applica a **qualunque** vista si scelga — la §2 dice *«per ogni vista dove stanno i pannelli»* e la riga 6 *«con lo stesso nome»*, e nessuna sonda cambia vista con un pacchetto salvato di un'altra (R9b-11); il criterio di chiusura del 17 su P-113 legge **rosso** com'è scritto, perché il richiamo del 2026-09-15 sulla riga della roadmap cita *«sedici compiti»* (R9b-15).
3. Il resto è **prosa e richiami che nessun Passo scrive**: la stella riceve solo due richiami (compito 8 sulla decisione 56, compito 14 sulla riga 1 di Passi); restano senza richiamo la riga dello stato della testa che dopo la parte 2 dice *«`crates/` intatto»*, i moduli costruiti (regola della §6), le registrate che questo piano chiude (`moveTo`, la forma della transizione) o doveva chiudere (il commento falso di `journal.rs`, compito 5), i 🔶 dedotti, design/10 (decisione 20) e il titolo della riga 2 della roadmap (debito della §3); la misura del processore a riposo promessa dalla decisione 41 non è in nessun compito.

## Rilievi

| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R9b-1 | Compito 5 · *Files*: «`crates/platform/src/journal.rs` … una parola: `engine` da privata a `pub(crate)`»; criterio «nessun altro tocco a quel file: `git diff --stat -- crates/platform/src/journal.rs` mostra una sola regione» | il 5 è il **primo** compito che tocca `journal.rs`, e vieta ogni altro tocco | la registrata della stella *«il commento di `crates/platform/src/journal.rs` … è falso … si corregge come compito del piano»* ha chiusore **«il piano del 2, nel primo compito che tocca `crates/platform/src/journal.rs`; o il proprietario, prima»**. `grep -n 'not a \`Record\`' crates/platform/src/journal.rs` → riga 63, il commento c'è; `grep -n promote crates/kernel/src/boundary.rs` → `Untrusted::promote` c'è; `grep -rln 'note(.*, b"' crates/*/tests` → 5 banchi (i soli che scrivono byte grezzi): la registrata è **vera oggi**. `grep -n 'boundary.rs' <piano>` → 5 righe, nessuna sul commento; la tabella [C] non la porta | CONFERMATO | fatto | no | Passo del compito 5 che tocca `journal.rs` (quello di D14): un secondo *Trova* sulla frase *«`boundary.rs` writes some that are not a `Record` at all»* riscritta al vero (*«only the benches write bytes that are not a `Record`»*), col richiamo datato; il criterio passa a **due** regioni. In alternativa una riga nuova in [C] col chiusore «il proprietario» — ma la stella dice che il chiusore è **questo piano** |
| R9b-2 | Compito 6 · *Files* (nessuna riga su `docs/design/`); compito 8 idem; compito 17 «⛔ NON si tocca nessun ADR» | nessun compito tocca `docs/design/10-modello-dei-dati-durevoli.md` | decisione 20 del proprietario: *«un'entità costruita passa dal secondo diagramma al primo con richiamo datato, regola scritta nel file»*; `grep -n 'col 2' docs/design/10-modello-dei-dati-durevoli.md` → riga 98 `DETAIL ||--o| INVOCATION_DETAIL : "specie nuova (col 2)"` e riga 160 la sua riga di tabella; `grep -n 'design/10\|10-modello\|docs/design' <piano>` → **nessuna** riga. Il 6 costruisce `InvocationDetail` (`Detail::Invocation` indice 3) e l'8 aggiunge `PolicyDetail` (`Detail::Policy` indice 4), che design/10 non nomina affatto | CONFERMATO | fatto | no | un Passo nel 6 (e uno nell'8) — o uno solo nel 17, «documenti in ogni casa» — che sposta `INVOCATION_DETAIL` nel primo `erDiagram` col richiamo datato e aggiunge `POLICY_DETAIL` (specie 4); `git ls-files --eol` prima (il vicolo della quinta ripresa dice che `docs/design/` è misto) |
| R9b-3 | Compito 17 · Passo 5 «`roadmap.md` — la riga 2 e la riga del piano»: «la riga 2 della tabella dei sotto-progetti passa da ⬜ a ✅ con la data» | il Passo cambia **solo lo stato** della riga 2 | «Debiti dichiarati» della §3: *«la roadmap dà il 2 come «GUI minima (shell, chat, stato)», titolo più stretto del perimetro di oggi — aggiornarlo è un compito del piano (decisione 9 del coordinatore)»*; `grep -n 'GUI minima' docs/roadmap.md` → riga 163 `| 2 | GUI minima (shell, chat, stato) | — | ⬜ |`; `grep -n -F 'GUI minima (shell' <piano>` → **nessuna** riga: il titolo non è nominato da nessun compito | CONFERMATO | prosa | no | Passo 5 del 17: la cella del titolo della riga 2 riscritta sul perimetro della §3 (per esempio *«GUI minima — la cornice con `dockview`, il filo, la settima porta, il registro, Stato/Permessi/Passi/Impostazioni»*), col `grep -c 'GUI minima (shell, chat, stato)' docs/roadmap.md` → 0 nel criterio; oppure un richiamo datato nella §3 della stella che dichiara il debito **non** pagato |
| R9b-4 | Compito 14 · Passo 15 «il richiamo datato sulle righe di Passi della stella polare» — «⛔ Solo la riga 1 riceve il richiamo» | un solo richiamo nella stella, sulla riga 1 della tabella *Passi*, per P-89/D56 | la testa della stella e la riga «chi disegna i moduli» della §6: *«quando **costruisce** un modulo, mette un richiamo datato nella riga di quel modulo nella §1: la regola di design/10»*. Il 14 costruisce Stato, Permessi, Passi, Chat (in parte) e Impostazioni (tabella corta); `grep -n 'direzione-gui-design' <piano>` → la stella è toccata solo dal compito 8 (decisione 56) e dal 14 (riga 1 di Passi, che parla dei **campi**, non del «costruito») | CONFERMATO | prosa | no | Passo 15 del 14: un richiamo per modulo costruito — sull'intestazione `#### Stato`, `#### Permessi`, `#### Passi`, `#### Chat` (con l'elenco delle righe costruite) e sulla riga *Impostazioni* della corta — *«✅ costruito dal compito 14 della parte 2, <data>: `gui/src/panels/<Nome>.vue`»*; il `grep -c` nel criterio. ⚠️ Le righe della §1 sono di R7: qui si segnala la **regola** della §6 |
| R9b-5 | Compito 14 · *Interfaces* «`gui/src/frame/moveActive.ts` — … `moveActive(api, direction)`»; compito 8 · Passo 12 «(a) … la riga 56 della tabella *Decisioni prese dal coordinatore*» | il 14 costruisce le scorciatoie sopra `moveTo`; l'8 scrive il richiamo sulla decisione 56 | tabella «Registrate, non prese»: *«le scorciatoie da tastiera per spostare un pannello scritte noi sopra `moveTo` … → **il piano del 2, con G20**»* e *«🔶 nata alla quarta ripresa — la forma con cui la transizione di policy si rilegge … → il disegno del 2 …; il piano ne fa un compito»*: entrambe hanno chiusore **questo piano**, entrambe restano senza «✅ chiusa» — `grep -n 'registrat' <piano>` → nessuna riga che tocchi quella tabella | CONFERMATO | prosa | no | nel Passo 15 del 14 e nel Passo 12(a) dell'8: un *Trova* sulla cella «Chiusore» di ciascuna riga e «✅ **chiusa il <data>, compito N del piano della parte 2**» in coda, come le righe già chiuse della stessa tabella |
| R9b-6 | Compito 17 · *Files* (la stella non c'è); Passo 10 «si **verifica** che ci sia, col `grep`» | il 17 non tocca la stella | testa della stella, tabella «Lo stato alla scrittura del disegno», riga «codice e spec non toccati»: *«… `crates/` intatto»* col richiamo del 2026-09-10; la §6 dice *«questo file tiene la tabella dello stato»*. Dopo i compiti 1–10 `git diff --stat 664265a..HEAD -- crates/` **non** rende nulla di vuoto: la riga mente. E «Il prossimo passo» chiude con *«Le skill della sessione che scrive il piano: …»*, frase stantia da quando il piano è scritto (2026-09-15). La regola del 17: *«si tocca ciò che questo piano ha reso falso»* | CONFERMATO | prosa | no — ma viola la regola scritta del 17 | Passo nuovo del 17: un richiamo datato in coda alla cella «Atteso» della riga «codice e spec non toccati» (*«✅ RICHIAMO DEL <data>: e `crates/`, `scripts/`, `.github/`, `gui/` per la parte 2 del piano del 2 — la Definizione di «fatto» del piano dice che cosa»*), e una riga in «Il prossimo passo» che dica che il piano è **eseguito** |
| R9b-7 | Compito 9 · «Passo 1: le misure prima, e la BASELINE …»; compito 12 · criterio di chiusura | nessuna misura del processore a riposo in nessun compito | decisione 41 del proprietario: *«nel 2 il daemon guarda il filo a ogni giro e dorme un tick consegnato, **e il piano misura il processore a riposo senza soglia**»*. `grep -in 'idle\|riposo\| cpu' <piano>` → 3 righe, tutte commenti di codice (`gui_tick` doc, «an idle client», «every idle tick»); `grep -n 'Get-Process\|typeperf\|%CPU\|cpu_pct\|processore' <piano>` → **nessuna** | CONFERMATO | fatto | no | nel criterio del 9 (o del 12, dove il core gira davvero con una GUI finta): il comando che misura il processore del `daemon` a riposo per 60 s (`Get-Process daemon \| Select-Object CPU` due volte a 60 s, o `typeperf`), la resa scritta in `riferimenti.md` dal 17 con data e comando, **senza soglia** |
| R9b-8 | Riga **D2**: «l'evidenza delle otto mosse è sulla 8.2.0, e il **compito 12** lo scrive accanto al primo uso» | il compito 12 scrive la nota accanto al primo uso di `dockview` | il primo uso di `dockview` è il **13** (tabella della posizione, D25 ha spostato 9–16 di uno; il 12 è il core finto in Rust). `awk 'NR>=13655 && NR<=15415' <piano> \| grep -n '8\.3\.1\|otto mosse\|eight moves'` → il 13 nomina «le otto mosse» per le opzioni e il tema, **mai** «misurate sulla 8.2.0, appuntata 8.3.1»; la sola frase in quei termini è nel **14** (*«letti nei `.d.ts` di `dockview-core` 8.3.1 il 2026-09-15, non ricordati dallo spike sulla 8.2.0»*) | CONFERMATO | prosa | no | richiamo datato in D2 (come D3 ha ricevuto per P-81): «il compito **13**»; e nel Passo di `dock.ts` del 13 la riga di commento promessa: *«the eight moves were judged on 8.2.0; pinned 8.3.1 (D2)»* |
| R9b-9 | Righe **D66**, **D71**, **D63–D65**, **D3**, **D2**, **D4**; compiti 16, 15, 14, 13, 11 · *Files* | il piano **precisa** sei decisioni della tabella del coordinatore — 47 (`package-manager-cache: false` scritto), 45 (niente `--audit-level`), 55 (`no-raw-text` è `warn` nel preset e sale a `error`; preset `flat/essential`; `multi-word-component-names` spenta), 51 (15.0.2 e rilettura al 14), 6 (8.3.1 e non 8.2.0), 52 (rimisurata) — e nessun Passo scrive un richiamo nella stella | `grep -n 'direzione-gui-design' <piano>` → la stella è modificata solo dal compito 8 (decisione 56) e dal 14 (riga 1 di Passi). Il precedente **dentro questo piano** è proprio l'8: una decisione del coordinatore precisata riceve il richiamo dove vive | CONFERMATO | prosa | no | un richiamo datato in coda alla cella di ciascuna decisione, dal compito che la esegue: 47 e 45 dal 16, 55 dal 15, 51 dal 14, 6 dal 13, 52 dall'11 — una riga l'uno, col numero `D` |
| R9b-10 | Riga **D4**: «⛔ La misura si rifà al compito 10» | la rimisura di `vitest` è «al compito 10» | D4 fu scritta prima di D25; il 10 di oggi è la campagna DST (tabella della posizione), e la riga stessa finisce con *«chi installa è il compito 11»*: il primo numero è stantio, il secondo giusto. `grep -c '^## Compito' <piano>` → 17 | CONFERMATO | prosa | no | richiamo datato in D4, come D3: «il compito **11**» |
| R9b-11 | Compito 13 · *Interfaces* «`interface LayoutPack { view: ViewName; layout: SerializedDockview }`»; Passo di `dock.ts` «A SAVED VIEW WINS OVER THE DEFAULT»; Passo di `Frame.vue` «function switchTo(view: ViewName)» | il pacchetto tiene **una** vista; `apply(api, view, pack)` fa `api.fromJSON(pack?.layout ?? shipped)` senza confrontare `pack.view` con `view`; `switchTo` chiama `apply(api, view, unpack(layout.state))` | la §2: la disposizione è *«quale vista è aperta, **per ogni vista** dove stanno i pannelli, e le viste che il proprietario salva con un nome»*; riga 6: *«una vista salvata **con lo stesso nome** vince sul default»*. Col codice dettato: pacchetto salvato `{view:"home", layout:H}` → `switchTo("work")` → `fromJSON(H)`: la disposizione di Home compare sotto la linguetta Lavoro; poi `onDidLayoutChange` → `settle({view:"work", layout:H})` e Home è **persa**. `awk 'NR>=13655 && NR<=15415' <piano> \| grep -n 'it("'` → nessuna sonda cambia vista con un pacchetto salvato di un'altra vista («takes a Package and reads the view out of it» legge solo `view`). Compila, e passa tutte le sonde del 13 | CONFERMATO | fatto | **sì** | `LayoutPack { view: ViewName; layouts: Partial<Record<ViewName, SerializedDockview>> }`; `apply` prende `pack?.layouts[view] ?? shipped`; `settle` fonde la vista corrente nelle `layouts` esistenti; blocco *Interfaces* del 13 e riga **D** nuova; due sonde in `frame.test.ts`: salva sotto `home`, `switchTo("work")` → la vista spedita di Lavoro; torna a `home` → la salvata. Il 14 consuma `useLayout` (solo `view`): non cambia |
| R9b-12 | Compito 13 · Passo di `dock.ts`: «window.addEventListener("beforeunload", () => { … layout.settle({ view: layout.view, layout: api.toJSON() }) })» | alla chiusura della finestra il pacchetto parte **sempre**, anche se `same(last, now)` | decisione 11 del coordinatore: *«le tre viste di default restano in `gui/`, non si copiano nell'archivio al primo avvio — copiate, un aggiornamento che migliora una vista non arriverebbe mai a chi non l'ha toccata»*. All'avvio il piano la rispetta (ascoltatore registrato **dopo** `apply`, evento bufferizzato, confronto canonico — misurato: `grep -n '_bufferOnDidLayoutChange' spikes/gui-shell/app/node_modules/dockview-core/dist/dockview-core.js` → `AsapEvent`); alla **prima chiusura** di una GUI mai toccata il default viene copiato nell'archivio, e la ragione della 11 cade. In più il bundle 8.2.0 spara `onDidLayoutChange` anche su `onDidActiveChange` (stessa riga del `grep`): un click su un pannello cambia `activeGroup` in `toJSON()` e basta a salvare | CONFERMATO | fatto | no | `beforeunload` salva solo se `!same(last, api.toJSON())`; una riga nel doc di `createDock` che dichiara che il pannello attivo **è** disposizione (o lo filtra dal confronto); sonda nelle due direzioni in `frame.test.ts` |
| R9b-13 | Compiti 3, 5, 7, 12, 13 · nessun Passo nomina i «🔶 dedotti» della stella | nessun richiamo di conferma | la stella marca *«🔶 Dedotto, da confermare da chi costruisce»*: §2 — il pacchetto opaco, i default in `gui/`, il salvataggio automatico, `Layout` come conferma, la forma della tabella `redb`; §3 — l'ordine dei pezzi 2–9, la SPA contro il ponte finto prima del daemon, i passi all'accoglienza e dopo ogni invocazione; sequenze — la lista dei passi all'accoglienza e dopo ogni invocazione, il client rifiutato in tabella fino al `Disconnected`. Il piano li **conferma tutti** nel codice (5: promessa 1; 13: `VIEWS`, `settle`; 7: `keep` → `retrieve` → `Layout`, l'ordine dell'accoglienza asserito; 5: una tabella e una chiave) e li **precisa** in uno (7: `Core::attending` doc *«a client that has been refused or has died, is not here»* — il rifiutato resta in `clients` con un altro `Stage`, non in `attending`), ma chi costruisce non lo scrive nella stella | CONFERMATO | prosa | no | Passo nuovo nel 17 (documenti in ogni casa): per ciascun 🔶 un «✅ confermato dal compito N, <data>» in coda al capoverso, e per il client rifiutato la precisazione (`Stage`, non tabella) |
| R9b-14 | Compito 17 · Passo 10 «la voce D56 … si **verifica** che ci sia, col `grep`» → «grep -c 'RICHIAMO DEL' docs/superpowers/specs/2026-09-07-direzione-gui-design.md» | il conteggio di **tutti** i richiami prova che c'è quello del 14 | `grep -c 'RICHIAMO DEL' <stella>` → **11** oggi; dopo l'8 e il 14 sarà 13 anche se il richiamo del 14 fosse stato scritto altrove o riscritto: il comando non isola ciò che dice di verificare, e il Passo non scrive l'atteso. Il 14 stesso usa il comando giusto: `grep -c 'RICHIAMO DEL <data>, dal compito 14'` | CONFERMATO | fatto | no | Passo 10 del 17: `grep -c 'dal compito 14 del piano della parte 2 (P-89, D56)' <stella>` → 1 |
| R9b-15 | Compito 17 · criterio «⛔ il conteggio dei compiti NON è tornato nella roadmap: `grep -cE 'sedici\|diciassette\|[0-9]+ compiti' docs/roadmap.md` → **zero** sulla riga del piano della parte 2 (P-113)» | il comando rende zero | `grep -cE 'sedici\|diciassette\|[0-9]+ compiti' docs/roadmap.md` → **2** oggi (righe 87 e 231); `sed -n '231p' docs/roadmap.md \| grep -oE '.{40}sedici.{40}'` → *«RICHIAMO DEL 2026-09-15: qui stava «sedici compiti, i primi due scritti», falso»* — è il richiamo stesso di P-113, sulla riga del piano, che cita il testo tolto; e il comando conta **righe del file intero**, non la riga. Com'è scritto, il criterio del 17 è **rosso** anche a lavoro fatto, e la via facile per farlo verde sarebbe cancellare il richiamo — la storia che `CLAUDE.md` vieta di togliere | CONFERMATO | fatto | **sì** | il comando anchorato alla riga e depurato delle citazioni: `grep -F 'parte-2-gui-minima.md' docs/roadmap.md \| sed 's/«[^»]*»//g' \| grep -cE 'sedici\|diciassette\|[0-9]+ compiti'` → **0**, provato oggi nelle due direzioni (senza il `sed` rende 1) |
| R9b-16 | Vincoli del revisore §1 («un heredoc Bash con backslash o righe lunghe rompe») contro i 23 `python - <<'EOF'` del piano | i Passi usano heredoc Python | `python probe-R9b/heredocs.py` → 23 blocchi, il più grande **1 273** byte, riga più lunga **316**, 12 blocchi con backslash; una prova nella cartella di prova — heredoc `<<'EOF'` con 2 backslash e una riga da 360 caratteri — → `EXIT=0`, testo intatto | NON RIPRODOTTO | fatto | no | — (il vicolo della stella parla di ~9 KB con righe di tabella: i blocchi del piano stanno sotto di sette volte) |
| R9b-17 | §2 «nessun tetto di dimensione … il giorno che serve è un parametro consegnato, non una costante» contro **D9**/**D31** (`max_body`, `MAX_BODY = 1024 * 1024` nel daemon e nel finto) | il tetto è del **trasporto** (unico produttore di `MalformedMessage`, P-14), consegnato a `LocalSocketIpc::bound`, e i due numeri sono letterali della radice di composizione | `grep -n 'const MAX_BODY' <piano>` → due definizioni (compiti 9 e 12), stesso valore; il pacchetto della disposizione ne è limitato **di riflesso**. È «un parametro consegnato», com'è scritto in §2 | NON RIPRODOTTO | prosa | no | (facoltativo) una riga nella §2 che dica che la cornice del filo lo limita a `MAX_BODY` |
| R9b-18 | Compito 8 · Passo 12 «(a) … la riga 56 … In coda alla cella, senza toccare il testo che c'è»; «(b) … la riga 9 della §9. In coda alla cella» | il *Trova* non è scritto, solo il *Sostituisci con* | Python `text.count`: la coda della riga 56 (*«… si rileggono al piano \|»*) → **1** nella stella; la coda della riga 9 della §9 (*«(decisione 56); un record congelato in più \|»*) → **1** nel disegno del 2 (⚠️ nel 2 tre righe cominciano con `\| 9 \|`: la §9 è la seconda). Chi esegue ha un aggancio unico | NON RIPRODOTTO | fatto | no | — (scrivere i due *Trova* nel Passo eviterebbe la scelta) |
| R9b-19 | §6 riga «la riga nella §12 del compendio e in «Dove va cosa» di `README.md`»: la forma *«⛔ Non è una spec e non disegna le capacità: …»* — contro il compito 17 Passo delle «Quattro righe nella forma che la §12 ha già» | il 17 scrive la riga nella forma delle righe vicine (`⛔ **il perimetro …** \| [\`specs/…\`](…) — ⚠️ **non è una spec**`) e due righe nella tabella «Specifiche» del README | `grep -n 'Non è una spec e non disegna' docs/COMPENDIO.md docs/README.md` → **nessuna**: la forma citata dalla §6 non esiste letteralmente; le righe vere dei due disegni sono `docs/COMPENDIO.md:917,919` (§12) e `docs/README.md:205,206` — nella tabella **«Specifiche»** (riga 195), non in «Dove va cosa» (riga 107). Il piano copia la forma **vera** | NON RIPRODOTTO | prosa | no | — |
| R9b-20 | Le tre sequenze contro il compito 3 (`pub enum IpcMessage`), il 7 (`IpcMessage::…` nel dispaccio), il 13/14 (`case "…"` / `kind: "…"`) | ogni messaggio ha un nome identico nei tre posti | enum del 3: `Hello, Accepted, StaleBuild, Degradation, Policy, Invoke, PermissionRequired, Approve { triple, call }, Token, Layout, SaveLayout, Steps, Request, Verdict` (14, come il piano conta); nel 7 tutte e 14 compaiono (`awk … \| grep -o 'IpcMessage::[A-Za-z]*' \| sort \| uniq -c`); nel 13/14 gli store trattano `Hello, Accepted, StaleBuild, Degradation, Policy, Layout, SaveLayout, Steps, Invoke, PermissionRequired, Approve, Token, Verdict` — `Request` mai mandato (D5, com'è scritto). L'accoglienza: `Accepted, Degradation, Policy, Layout, Steps` e **`heard.len() == 5`** (la seconda direzione). «La lista dei passi» = `Steps` (decisione 46) | NON RIPRODOTTO | fatto | no | — |
| R9b-21 | §2 riga 8 e nota della sequenza 2: «un pannello che punta a un tipo … sparito lo dice a parole **e si chiude**» contro il compito 13 | il registro dà un segnaposto a un tipo non costruito (D47) | `awk 'NR>=13655 && NR<=15415' <piano> \| grep -n 'it("'` → `it("says a type that is GONE is gone, does not promise a sub-project, and closes")` e `it("refuses a package it cannot read, instead of half-applying it")`: le due prove della riga 8 ci sono | NON RIPRODOTTO | fatto | no | — |
| R9b-22 | Decisione 11 del coordinatore al **primo avvio** contro `createDock` del 13 («let last = api.toJSON(); api.onDidLayoutChange(…)») | al primo avvio il default non parte verso l'archivio | l'ascoltatore è registrato dopo `apply`, l'evento di `dockview` è bufferizzato (`AsapEvent`, letto nel bundle 8.2.0 dello spike) e il confronto è canonico (E4): il primo `fromJSON` non salva. Il caso della chiusura è R9b-12 | NON RIPRODOTTO | fatto | no | — |
| R9b-23 | Vincoli globali · «L'aiutante `replace_unique.py` vive nello scratchpad» contro il vicolo «il percorso dello scratchpad supera i 259 caratteri di `MAX_PATH`» | `python replace_unique.py <file> <old.txt> <new.txt>` dallo scratchpad | il percorso dello scratchpad di questa sessione è ~120 caratteri e `python "<scratchpad>/review/probe-R9b/heredocs.py"` è partito; i `python - <<'EOF'` del piano non passano per un percorso | NON RIPRODOTTO | fatto | no | — (chi esegue tiene l'aiutante **in cima** allo scratchpad, non in una sottocartella profonda) |
| R9b-24 | Tabella [C] · le righe «`**E50 / E51 / E100**`» e «`**E12**`» contro il comando del diario «L'errata … `awk '/^## ⚠️ L.errata …/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++}'` → 0» | l'errata è vuota | `grep -c '^\| \*\*E[0-9]' <piano>` → **2** (le due righe di [C]), ma l'`awk` del diario è **scopato** alla sezione dell'errata e rende 0: le due righe non lo falsificano | NON RIPRODOTTO | fatto | no | — |
| R9b-25 | Decisione 35 del proprietario («il core parte e `Layout` dice «non disponibile»») | **D30**: `MaybeCustody` nel daemon, `Unavailable` tradotto in `LayoutState::Unavailable` dal 7 | `grep -n 'impl Custody for MaybeCustody' <piano>` → 1; la sonda del 13 `it("keeps the default view on Nothing and on Unavailable")` | GIÀ COPERTO da D30 | fatto | no | — |
| R9b-26 | Decisione 22 (nessuna chiusura nella porta; il rifiutato non si ascolta più) | enum del 3: doc di `StaleBuild` *«the core stops listening to that client -- the port has no close (decision 22)»*; 7: `fn a_stale_stamp_gets_the_expected_one_and_then_the_core_stops_listening` | `awk 'NR>=7650 && NR<=9367' <piano> \| grep -n 'refused'` → *«a refused gui writes nothing, and its later words are not read»* | GIÀ COPERTO da compito 3 Passo 2 e compito 7 | fatto | no | — |
| R9b-27 | Decisione 21 (`Approve` porta anche funzione e argomento) | enum del 3: `Approve { triple: Triple, call: Call }` con la decisione nel doc; **D59** lo store `invoke.ts` accoppia `core.pending` col `Call` in volo | `sed -n '4400,4475p' <piano>` | GIÀ COPERTO da D59 | fatto | no | — |
| R9b-28 | Decisione 8 del proprietario e 5 del coordinatore (Passi mostra le invocazioni del registro) contro la riga 1 della tabella *Passi* | **D56**: tre campi, richiamo dal 14, **decisa A dal proprietario il 2026-09-15** | Python `count` dell'ancora del Passo 15 del 14 nella stella → **1** (il *Trova* regge); `grep -c 'RICHIAMO DEL <data>, dal compito 14'` → 0 oggi, 1 dopo | GIÀ COPERTO da D56 / P-89 | fatto | no | — |

Conteggio dalla tabella (righe che cominciano con `| R9b-`): `grep -c '^| R9b-[0-9]* |.*| CONFERMATO |' R9b-report.md` → **15**; con `NON RIPRODOTTO` → 9; con `GIÀ COPERTO` → 4; righe totali `grep -c '^| R9b-'` → 28. Bloccano: **R9b-11** e **R9b-15**.

## Copertura del disegno per il mio perimetro

Legenda dell'esito: **coperta** (compito/Passo/D la produce) · **precisata** (il piano fa una cosa più stretta o diversa e lo dichiara con una `D`; «senza richiamo» se nessun Passo lo scrive nella stella) · **scoperta** (nessun compito) · **contraddetta** · **n/a** (documenti già scritti prima del piano, o fuori dal 2).

### A. Le decisioni del proprietario (tabella «una per domanda»)

| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|
| 0 — la cornice più la fetta del 2; ogni vista col numero di chi la costruisce | 13 (cornice, tre viste, `Placeholder.vue` con nome e numero — D47), 14 (moduli) | coperta |
| 1, 2, 3 — anello/rete/tessere «vive», nucleo che a parole dice «niente ancora» | 13 · `home.json` con `knowledge` bloccato al centro; `placeholderParams` | coperta (il nucleo è segnaposto del 6, com'è scritto) |
| 4 — la disposizione nel core, archivio con una voce sola; il core la manda al collegamento, la GUI «salva disposizione» | 4/5 (`CustodyKey::Layout`, una chiave), 7 (`Layout` all'accoglienza; `SaveLayout` → `keep` → `retrieve` → `Layout`), 13 (`settle`) | coperta |
| 5 — Compatta: terza vista, si disegna ora, la costruisce il 10, zero codice nel 2 | 13 · `compact.json` col commento *«IS A PLACEHOLDER AND SAYS SO … closer is sub-project 10»* | coperta |
| 6 — pannelli agganciabili, `dockview-core` **8.2.0** diretto, dipendenza in due passi | 13 (`createDockview` diretto, `VueContent` di poche righe); **D2** appunta **8.3.1** | precisata, senza richiamo — R9b-8, R9b-9 |
| 7 — la chat non in Home, vive in Lavoro | 13 · generatore delle viste: *«The chat is NOT here -- it is a tab of Lavoro (question 7)»*; `tile(work, "chat")` | coperta |
| 8 — Passi nasce col 2: invocazioni del registro con intento ed esito; un messaggio IPC | 3 (`Steps(Vec<StepSummary>)`), 7 (all'accoglienza e dopo ogni invocazione), 14 (`Steps.vue`); D56 sui tre campi, decisa A il 2026-09-15 | coperta (precisata con richiamo dal 14) |
| 9 — «Automazione OS» solo un nome | [C] «le registrate della stella polare … il proprietario» | n/a, dichiarata |
| 10, 11, 12, 13 — catalogo: wireframe come mappa, indicatore del modello, tabella corta, Ambito | §1 (R7) | n/a qui |
| 14 — «salva disposizione» fuori dal registro | 7 · `SaveLayout` gestito nel dispaccio con `self.custody.keep`, non con `Registry::invoke`; `POLICY_FUNCTION` unica funzione registrata | coperta |
| 15 — la settima porta; la §3.1 da sei a sette con richiamo | 4 (tratto, finta, tre richiami nella spec — P-22), 5 (`FileCustody`, `MemoryCustody`, suite) | coperta |
| 16 — il mandato dei diagrammi; una correzione a una sezione approvata torna in A/B | D56 portata in A/B e decisa; design/10 **non** aggiornato | in parte — R9b-2 |
| 17 — la policy corrente è la proiezione del giornale; il daemon rilegge all'avvio; `build_the_arbiter` riceve la policy | 8 (`policy_now`, D26/D27), 9 (`build_the_arbiter` riceve la policy, `unwrap_or` sul default) | coperta |
| 18 — il metodo esiste/arriva/regge | ogni riga `D` porta il «Perché» e il costo | coperta |
| 19 — la lettera E della §8.2 (12) | fuori dal 2 | n/a |
| 20 — design/10: «un'entità costruita passa dal secondo diagramma al primo con richiamo datato» | nessun compito tocca `docs/design/10-modello-dei-dati-durevoli.md` | **scoperta** — R9b-2 |
| 21 — `Approve` porta funzione e argomento; il core apre il passo A | 3 (`Approve { triple, call }`), 6 (`Approval::JustGiven`, D24), 7, 14 (D59) | coperta |
| 22 — nessuna chiusura nella porta; il rifiutato non si ascolta più | 3 (doc di `StaleBuild`), 7 (sonda `…the_core_stops_listening`) | coperta |
| 23 — le sequenze vivono nella stella | — | n/a |
| 24, 25 — lo spike, la mossa 8 | §4, parte 1 (R6) | n/a |
| 26–32 — i tagli della lettura d'apertura | eseguiti il 2026-09-09 | n/a |
| 33 — la §7 del 2, il finto **riusa** l'attività vera | 12 · *«Il dispaccio è l'attività vera del kernel (compito 7)»*, D41–D45 | coperta |
| 34 — un cancello unico col passo web dentro | 15 · `run "gui: fake core and SPA" bash scripts/gate-gui.sh` in `gate.sh`, *«Il cancello resta UNO, ed è la decisione 34 in persona»* | coperta |
| 35 — l'archivio che non si apre: il core parte, `Layout` «non disponibile» | 3 (`LayoutState::Unavailable`), 7 (traduzione di `CustodyError::Unavailable`), 9 (**D30** `MaybeCustody`, sonda sui due rami) | coperta |
| 36, 37, 38, 39, 40 — §9, §10, §6, sessioni | scritte il 2026-09-09 | n/a |
| 41 — prontezza I/O: A; «il piano misura il processore a riposo senza soglia» | 7 (`gui_tick` in `Parameters`, `nap`), 9 (`GUI_TICK`); **la misura non c'è** | in parte — R9b-7 |
| 42 — il contatore: un tipo del kernel seminato con `replay` | 1 · `kernel::numbering::{Progressive, seeded_from}` (D7, D8) | coperta |
| 43 — AUD-004 in parallelo, sbarra il 13 | [C] | n/a, dichiarata |
| 44 — X-1: la matrice Windows; da provare che il runner onori `rust-toolchain.toml` e che Git Bash lanci `gate.sh` | 16 · `strategy.matrix.os`, `fail-fast: false`; criterio: nel job Windows `rustup show` e `bash scripts/gate.sh` arriva a `GATE GREEN` (P-111: metà si guarda nella corsa) | coperta |
| 45 — X-3: `cargo audit` in `gate.sh`, `npm audit` in `gate-gui.sh`, niente `cargo deny`; `--audit-level` come manopola | 16 · D68–D71; **D71** non usa `--audit-level` | coperta, precisata senza richiamo — R9b-9 |
| 46 — i sette nomi inglesi (`custody`, `Custody`, `keep`, `retrieve`, `CustodyKey::Layout`, `registry`, `Steps`) | 4, 5, 6, 3 — tutti nei blocchi *Interfaces* e nell'enum | coperta |
| 47 — la §1 del 2 per rimando | — | n/a |
| 48 — la parte 1 in una sessione nuova | — | n/a |

### B. «Il modello della GUI»

| Riga | Compito · Passo | Esito |
|---|---|---|
| modulo = tipo registrato, esemplari = pannelli con parametri | 13 · `panels/registry.ts` (`PANEL_TYPES`, `register`, `componentFor`) | coperta |
| regola unica: sul posto o a pagina intera | 14 · menu del modulo «stacca» e «pagina intera» (D58; «finestra a parte» aperta, P-91) | coperta (precisata in [C]) |
| vista = layout salvato con un nome; tre con l'app | 13 · `VIEWS`, `home/work/compact.json` **generati** | coperta |
| Home, Lavoro, Compatta | 13 · generatore delle viste (nucleo bloccato, striscia bloccata; Lavoro con chat/attività/ambito/diff/anteprima/terminale/passi/sensori; Compatta segnaposto) | coperta |
| striscia sempre visibile in ogni vista | 13 · `Strip.vue`, gruppo bloccato in tutte e tre (D50) | coperta |
| ricerca dalla barra | 13 · `<input class="search" … disabled :placeholder="$t('bar.searchHint')">` — «la ricerca sugli artefatti arriva col sotto-progetto 6» | coperta (dice chi la riempie) |
| come cresce (schede, tipi nuovi nel cassetto) | 13 · `Drawer.vue` su `PANEL_TYPES` con `who` | coperta |
| dove si rompe: «render modes», la frequenza la decide il core | 7 · `gui_tick`, D23 (il degrado solo quando differisce) | coperta |
| la disposizione nel core | 4/5/7/13 | coperta — ma **per vista** no: R9b-11 |
| la mano: `dndStrategy` | 13 · `dndStrategy: "pointer"` (D49, dallo spike) | coperta |
| cosa mostra davvero il 2 | 13 + 14 | coperta |

### C. Le tre sequenze del protocollo core ↔ GUI

| Freccia / nota | Compito 3 (variante) | Compito 7 (ramo) | 13/14 (store) | Esito |
|---|---|---|---|---|
| 1 · `Hello (il timbro di build)` | `Hello(BuildStamp)` | `Hello` letto per primo | `connection.hello()` col timbro da `ipc_v1.map` (D52) | coperta |
| 1 · `Accepted (la protezione come valore)` | `Accepted(Protection)` | primo dell'accoglienza | `phase: "connected"` | coperta (D22: mandato, non consegnato — richiamo nel 2, non nella stella) |
| 1 · `degradation_now` → `Degradation` | `Degradation(DegradationReport)` | secondo; poi solo se cambia (D23) | `core.degradation` | coperta |
| 1 · arbitro → `Policy` | `Policy(PolicyReport { policy, allocated, total })` (D20) | terzo | `core.policy` | coperta |
| 1 · settima porta → `Layout (pacchetto o niente)` | `Layout(LayoutState)` con `Nothing / Package / Unavailable` | quarto | `layout.receive` | coperta |
| 1 · `replay` → «la lista dei passi» | `Steps(Vec<StepSummary>)` | quinto, e `heard.len() == 5` | `core.steps` (sostituisce, non appende) | coperta |
| 1 · «da qui solo il pezzo che cambia» | — | seconda direzione della sonda dell'accoglienza | — | coperta |
| 1 · `StaleBuild (il timbro atteso)`; il core non ascolta più; la GUI esce | `StaleBuild(BuildStamp)` | sonda *stops listening* | `phase: "stale"`, `expected` | coperta |
| ogni sequenza · `Err(Disconnected)` → `on_disconnect` | — | `Core::grants`, `Disconnected` (P-56, D33) | — | coperta |
| 2 · `SaveLayout (toJSON e la vista attiva)` | `SaveLayout(Vec<u8>)` | `let _ = self.custody.keep(CustodyKey::Layout, package)` | `layout.settle` → `pack_` | coperta — una vista sola: R9b-11 |
| 2 · scrittura riuscita/fallita → «ridammi i byte» → `Layout` | — | `retrieve` dopo `keep`, sempre | — | coperta; la sonda con la scrittura **rifiutata** non è in nessun compito (le finte del 5 provano la suite, `MaybeCustody` prova «non disponibile»): riga 5 della §2, «Non verificato» |
| 2 · riavvio → `Hello` … `Layout (il pacchetto)` / `(niente)` | — | 9 · «salva, riavvia, ritrova» | 13 · `apply` con default se `null` | coperta |
| 2 · «un pannello che punta a un tipo sparito lo dice e si chiude» | — | — | 13 · sonda *GONE … and closes* | coperta |
| 3 · `Invoke (funzione, argomento, invocatore)` | `Invoke(Call { function, argument })`; l'invocatore è il `ClientId` del filo | `Registry::invoke(…, Invoker::Gui(id), …)` | 14 · `invoke.send` (D55: `functions.ts`) | coperta |
| 3 · funzione non registrata → rifiutata senza scrivere | — | 6 · `InvokeError::NotRegistered`; sonda *the registry refuses BEFORE it opens step A* | — | coperta |
| 3 · `is_granted` → `PermissionRequired (la tripla)` | `PermissionRequired(Triple)` | ramo `PermissionRequired` | 14 · `Confirm.vue` (D60) | coperta |
| 3 · `Approve (tripla, funzione, argomento)` | `Approve { triple, call }` | `Approval::JustGiven` scrive il `grant` su A (D24) | 14 · `invoke.approve()` (D59) | coperta |
| 3 · intent su A, «note su A dettaglio Invocation (funzione, invocatore, argomento)» | — | 6 · `RecordV1::invocation` (specie, non nota); argomento nel `payload` (D18) | — | precisata nel disegno del 2 (P-30, P-32 dal 6); la sequenza resta «nota» a parole — prosa, nessun richiamo (R9b-13) |
| 3 · `set_policy` su B, esito su A | — | 7 · sequenza dei record asserita; 8 · da sei a sette record (P-46) | — | coperta |
| 3 · `Policy (la nuova)` e la lista aggiornata | — | dopo ogni invocazione | — | coperta |
| 🔶 dedotti delle sequenze (lista all'accoglienza; rifiutato in tabella fino al `Disconnected`) | — | confermato / precisato (`Stage`) | — | nessun richiamo — R9b-13 |

### D. §2 — Viste e disposizione

| Riga | Compito · Passo | Esito |
|---|---|---|
| capoverso 1: la disposizione = vista aperta + **per ogni vista** i pannelli + le viste salvate con un nome | 13 · `LayoutPack { view, layout }` | **contraddetta senza D** — R9b-11 |
| 1 · pacchetto opaco; byte non JSON tornano identici | 5 · promessa 1 della suite (`custody_contract.rs`), `TextCustody` fra i bugiardi | coperta |
| 2 · la settima porta: due operazioni, una chiave; non è configurazione; `ports/mod.rs` e §3.1 sei → sette | 4 · `Custody::{keep, retrieve}`, `CustodyKey::Layout`, tre richiami nella spec (P-22), le cifre di `ports/mod.rs` (P-5/P-21: SIX→SEVEN, FIVE→SIX fakes, SEVEN→EIGHT) | coperta |
| 3 · `redb` sul `FileBackend`, un file, una tabella, una chiave; apri-scrivi-riapri-rileggi | 5 · `FileCustody::open(&Path)` (D14, `engine` `pub(crate)`), `what_was_kept_survives_reopening_the_file` | coperta |
| 4 · la finta del simulatore; la campagna del 2 gira con questa porta | 5 · `MemoryCustody`; 10 · `simulator::custody::MemoryCustody` nella campagna | coperta |
| 5 · `Layout` e `SaveLayout`; all'accoglienza dopo `Accepted`, di nuovo dopo ogni `SaveLayout`; fixture; **sonda: archivio che rifiuta la scrittura → `Layout` col vecchio**; richiamo 2026-09-09 «non disponibile» | 3 (varianti e fixture), 7 (ordine, `keep`→`retrieve`), 9 (D30) | coperta, **tranne la sonda della scrittura rifiutata**: nessuna finta del piano rifiuta `keep` tenendo il vecchio (`grep -n -B8 'fn keep(' <piano>` → 10 implementazioni, nessuna così) — «Non verificato» |
| 6 · le tre viste come JSON committati; una vista salvata con lo stesso nome vince sul default | 13 · `VIEWS`, `apply` («A SAVED VIEW WINS…») | **contraddetta nel «con lo stesso nome»** — R9b-11 |
| 7 · si salva da solo quando si ferma e alla chiusura; la cadenza è della GUI | 13 · `onDidLayoutChange` + `same` canonico, `beforeunload` | coperta; il salvataggio alla chiusura copia il default — R9b-12 |
| 8 · se il salvato non torna: lo dice e si chiude, controlla la GUI | 13 · `unpack` → `null` → default; sonda *GONE … closes*; `updateParameters(placeholderParams)` dopo `fromJSON` | coperta |
| 9 · il percorso dell'archivio argomento del daemon; ogni banco passa il suo | 9 · `LAYOUT_PATH`, percorso come argomento (tabella della posizione) | coperta |
| «Perché fuori dal registro — decisione 14» | 7 (vedi A·14) | coperta |
| «Perché una settima porta e non il giornale — decisione 15»; «una porta con due operazioni, tre implementazioni» | 4, 5 (finta di `ports_are_implementable.rs`, `MemoryCustody`, `FileCustody`) | coperta |
| «Ciò che la §2 non fa»: niente sistema di configurazione; una chiave; **nessun tetto** (un parametro consegnato quando serve); nessun secondo client | 4 (una chiave); D9/D31 (`max_body` consegnato al trasporto) | coperta — R9b-17 |
| «Esaminate e senza fonte»: ripristina / esporta / importa | `grep -n 'ripristina\|esporta\|importa' <piano>` → nessuna riga di prodotto | coperta (non costruite) |
| Debiti: C1 più largo; una chiave; la GUI sa della scrittura fallita solo confrontando | 4 (P-21, la guardia); 13 (`same`, `canonical`) | coperta |
| 🔶 dedotti: pacchetto opaco, default in `gui/`, salvataggio automatico, `Layout` come conferma, forma della tabella `redb` | tutti confermati nel codice dei compiti 5, 7, 13 | nessun richiamo — R9b-13 |
| Assunto `toJSON` si rimette → richiamo 2026-09-10 (E4, canonico) | 13 · `canonical`, `same`, il commento *«MEASURED ON 2026-09-10 BY SP-8»* | coperta |

### E. §3 — La fetta del 2 ritagliata

| Riga | Compito · Passo | Esito |
|---|---|---|
| 1 · spike + accettazione + ADR-0029 | parte 1 | n/a (eseguito) |
| 2 · il filo — invariato | 2 · `platform::ipc::LocalSocketIpc`, `ipc_contract.rs` | coperta |
| 3 · lo schema **più tre** (`Layout`, `SaveLayout`, la lista dei passi), fixture e timbro | 3 · enum a 14 (11 della §4 + 3), `stamp_set`, `build_stamp`, `.bin/.json/.map` (D35) | coperta |
| 4 · il registro col click e il cambio di policy — invariato | 6 (`kernel::registry`), 7 (`POLICY_FUNCTION`), 14 (Impostazioni, P-85) | coperta |
| 5 · la settima porta e i richiami a §2.3, §3.1, testa di `ports/mod.rs` | 4, 5 | coperta |
| 6 · il daemon che ascolta, ora anche `Layout` e i passi, scrive la disposizione; percorso argomento | 7 (dispaccio), 8 (policy riletta), 9 (cablaggio) | coperta |
| 7 · la SPA: Vue 3, `pinia`, Reka UI, `vue-i18n`, token, cornice `dockview-core`, tre viste, moduli, finestra di permesso, scorciatoie sopra `moveTo` | 11 (base), 13 (cornice; Reka UI per il dialogo — D62), 14 (moduli, `Confirm.vue`, `moveActive.ts`) | coperta |
| 8 · il core finto più `Layout`/`SaveLayout` e i passi | 12 · l'attività vera su porte in memoria: tutto ciò che il 7 fa, gratis | coperta |
| 9 · il passo del cancello | 15 (+16) | coperta |
| «Le sezioni del 2 che cambiano» — §1, §2, §6a riscritte; §4, §5 col richiamo; §3 niente | `grep -n 'RICHIAMO DEL 2026-09-0[89]\|riscritta il 2026-09-09' <disegno2>` → righe 100–102, 134–136, 209, 227, 236, 253, 258, 271, 281–283: **tutti esistono già**; nessuna riga della §3 del 2 è toccata dal piano | coperta |
| «Cosa il 2 NON costruisce»: Compatta vera · nucleo, ricerca, casella → 6 · casella di scrittura e run → 3 · tetto → parametro · navigazione spaziale e guide a pagamento → mai · ripristina/esporta/importa · kit UI | 13 (`compact.json` segnaposto; `search` disabilitata con la scritta del 6), 14 («nessuna casella di scrittura nella Chat»); `grep` di `kit UI`, `ripristina`, `navigazione spaziale`, `esporta` → niente di prodotto | coperta (niente lo costruisce) |
| Debito: il titolo della roadmap | 17 · Passo 5 cambia solo lo stato | **scoperta** — R9b-3 |
| Debito: le cifre di `ports/mod.rs` (sette, sei, otto) | 4 · tabella delle sei righe (P-5, P-21) | coperta |
| Debito: i richiami a §2.3 e §3.1 | 4 · tre posti (P-22) | coperta |
| 🔶 dedotti: ordine dei pezzi 2–9; SPA contro il ponte finto prima del daemon; passi all'accoglienza e dopo ogni invocazione | ordine 2,3,4/5,6,7/8/9,10,11/12,13/14,15/16 = quello; 11 `fakeBridge.ts`, 13/14 nel browser contro la finta (D57); 7 | confermati, **nessun richiamo** — R9b-13 |
| «Cosa è già studiato, e dove» — i richiami 2026-09-08 (sequenze, design/10) | — | n/a; design/10 non aggiornato dal piano — R9b-2 |
| le due divergenze (policy in design/09; guide) | chiuse il 2026-09-08 (decisioni 17 e 20) | n/a |

### F. §6 — Dove vive la stella polare, e «Le sezioni, e dove vive ciascuna»

| Riga | Compito · Passo | Esito |
|---|---|---|
| la testa: «non è una spec» | — | n/a |
| chi disegna i moduli → **richiamo datato nella riga del modulo costruito** | 14 · Passo 15 solo riga 1 di Passi | **scoperta** — R9b-4 |
| il rimando dal 2 | — | n/a |
| la riga nella §12 del compendio e in «Dove va cosa» del README, nella forma dei due disegni | 17 · quattro righe in §12 (D72) nella forma di `COMPENDIO.md:917,919`; due righe in «Specifiche» del README (P-116), dove stanno le righe dei due disegni (`README.md:205,206`) | coperta — R9b-19 |
| l'archivio (la consegna in coda a `consegna-brainstorming-direzione-gui.md`) | fatto il 2026-09-09 | n/a |
| la consegna della sessione: §10 del 2 | — | n/a |
| «questo file tiene la tabella dello stato e rimanda alla §6 del compendio» | 17 · riscrive il puntatore della §6 (D72); **la tabella dello stato della stella non è toccata** | **scoperta** — R9b-6 |
| «Le sezioni, e dove vive ciascuna» (7 righe) | nulla cambia casa | coperta (invariata, giusto così) |

### G. Le decisioni del coordinatore (1–57)

| # | Compito · Passo | Esito |
|---|---|---|
| 1, 3, 6, 7 — wireframe SVG, tessere da G9–G18, file suo, richiami e non riscritture | — | n/a |
| 2 — `dockview-core` diretto, non `dockview-vue` | 13 · `createDockview`, `VueContent implements IContentRenderer`; `dockview` **solo per il CSS** (E2 della parte 1, richiamo nel 2 alla §2) | coperta |
| 4 — la striscia in ogni vista | 13 · `strip` nelle tre viste, gruppo bloccato (D50) | coperta |
| 5 — Passi mostra le invocazioni | 3/7/14, D56 | coperta |
| 8 — commit senza co-autore | vincolo globale 13 | coperta |
| 9 — le righe di README/roadmap/tracciabilità sono compiti del piano | 17 · README (P-116), tracciabilità (Passo 6); roadmap: **solo lo stato** | in parte — R9b-3 |
| 10 — `redb` sul `FileBackend` esistente | 5 · `FileCustody` su `FileBackend::open`, `engine` `pub(crate)` (D14) | coperta |
| 11 — i default restano in `gui/`, non si copiano all'avvio | 13 · al primo avvio no; alla chiusura sì | precisata di fatto, senza D — R9b-12 |
| 12 — salvataggio automatico quando si ferma, non un pulsante; la cadenza dichiarata | 13 · `settle` con `same` canonico (il commento cita la decisione 12) | coperta |
| 13 — `Layout` rimandato dopo ogni `SaveLayout` | 7 · `retrieve` dopo `keep`, sempre; doc di `Layout` nell'enum | coperta |
| 14, 15, 20, 25–29 — le delegate | vedi A | coperta |
| 16 — la striscia mostra solo il vivo; la casella dice chi la riempie; il cassetto elenca i tipi col numero | 13 · `Strip.vue` (degrado, permessi), `search` disabilitata con la scritta, `Drawer.vue` su `PANEL_TYPES.who` | coperta |
| 17 — **un** segnaposto | 13 · `Placeholder.vue` (D47) | coperta |
| 18 — la settima porta pezzo suo | compiti 4 e 5 | coperta |
| 19 — nessun ADR | vincolo globale 10 | coperta |
| 21, 22, 24 — metodo dei diagrammi | — | n/a |
| 23 — i due passi per invocazione registrati | [C] | coperta (dichiarata) |
| 30–34 — lo spike | §4 (R6), parte 1 | n/a |
| 35–45 — i tagli | eseguiti | n/a |
| 46 — `engines.node` + `engine-strict=true` in `.npmrc`, non `.nvmrc` | 11 · D37 (la riga di `jsdom` tale e quale), `.npmrc` di una riga; P-65: **Node di questa macchina `v24.9.0` non soddisfa `^24.15.0`** — prerequisito dell'ambiente dichiarato, si aggiorna Node prima del Passo 1 | coperta |
| 47 — niente cache npm in CI | 15 · D66 `package-manager-cache: false` scritto | precisata (resa esplicita), senza richiamo — R9b-9 |
| 48 — le sonde del finto in `main.rs`; ricompilazione dichiarata | 12 · «le sonde in fondo (decisione 48)» | coperta |
| 49 — i lockfile si committano | vincolo globale 7; 11, 12, 13, 14, 15 | coperta |
| 50 — «non disponibile» in `Layout`, non in `Degradation` | 3 · `LayoutState::Unavailable`; 7 | coperta |
| 51 — `markdown-it` 15.0.1, preset `default`, `html: false` | 14 · D3 (15.0.2, rilettura nel `.tgz` al 14 — P-81), D54 (link come `<span>`, immagini come testo — P-87) | precisata, senza richiamo — R9b-9 |
| 52 — `vitest` 4.1.11, non la 5 | 11 · D4 (rimisurata il 2026-09-14) | coperta — R9b-10 sul numero |
| 53 — `jsdom` 30.0.1 | 13 · `vite.config.ts` → `jsdom` | coperta |
| 54 — `axe-core` 4.13.0 diretto, `@vue/test-utils` 2.5.0, niente `vitest-axe` | 14 · `a11y.test.ts`; D53 (`color-contrast` fra gli incompleti: sonda nostra sui token, P-86) | coperta, precisata con D53 |
| 55 — `no-raw-text` nel preset `recommended` di `@intlify/eslint-plugin-vue-i18n` 4.5.1, `eslint` 10.10.0, `eslint-plugin-vue` 10.11.0, `npm run lint` nel cancello | 15 · D63 (`flat/essential`), D64, D65 (`no-raw-text` da `warn` a `error`, P-98) | precisata, senza richiamo — R9b-9 |
| 56 — il dettaglio tipizzato della policy | 8 · D26 (specie, non nota — P-44), richiamo sulla riga 56 dal Passo 12(a) | coperta, **con** richiamo |
| 57 — se vince Tauri, `gui/shell/` | Electron scelto (richiamo del 2026-09-10 nella §4) | n/a |

### H. «Registrate, non prese» contro la tabella [C]

| Registrata (stella) | Chiusore (stella) | In [C]? | Esito |
|---|---|---|---|
| l'ambito come «progetto» | il 3 | sì (riga «le registrate…») | stesso chiusore |
| «Automazione OS» | il proprietario, prima del 10 | sì | stesso chiusore |
| «salva disposizione» fuori dal registro | ✅ chiusa (decisione 14) | — | n/a |
| le scorciatoie sopra `moveTo` | **il piano del 2, con G20** | no | eseguita dal 14, **non segnata chiusa** — R9b-5 |
| la libreria del grafo | il 6 | sì | stesso chiusore |
| Compatta popout o finestra | il 10 | sì | stesso chiusore |
| le aperte della §9 del 2 «come lì» (renderer, prove, lint, guscio, prontezza I/O, allocatore, confine di sessione, watchdog, AUD-004, il ledger `.superpowers/sdd/`) | come lì | in parte: prontezza I/O e allocatore («il proprietario, decisioni 41 e 42»), confine di sessione (il 3), watchdog (il 10), AUD-004 (il proprietario); renderer/prove/lint/guscio chiuse dalle decisioni 51–57 e da D3/D4/D63–D65; **il ledger** (riga 12 della §9 del 2: «il proprietario, un comando») non è in [C] | coerente; il ledger manca in [C] ma il suo chiusore non è questo piano |
| modalità di esecuzione per run | il 3 | no (riassunta) | stesso chiusore |
| modello scelto a mano per run | il 3 | no | idem |
| «+ allegati» e «aggiungi al contesto» | il 3, col 6 | no | idem |
| il grafico GPU | il 9 | no | idem |
| l'esportazione OTLP | il proprietario | no | idem |
| «auto-approva sicuri» ovunque o nell'ambito | il 4 | no | idem |
| ✅ policy VRAM (decisione 17), ✅ guide (design/09), ✅ lettera E, ✅ archivio che non si apre, ✅ AVVIO-CHAT | chiuse | — | n/a |
| 🔶 la forma con cui la transizione di policy si rilegge | il disegno del 2 …; **il piano ne fa un compito** | no | eseguita dall'8, **non segnata chiusa** — R9b-5 |
| 🔶 i due passi per invocazione | il 3 | sì | stesso chiusore |
| 🔶 il commento falso di `crates/platform/src/journal.rs` | **il piano del 2, nel primo compito che tocca il file; o il proprietario, prima** | no | **il 5 tocca il file e non lo corregge** — R9b-1 |
| 🔶 l'innesco B (3) di Q6 e Q11 | il proprietario | no | fuori dal 2 |
| 🔶 le storie nelle celle della tabella del 2026-08-10 | il proprietario | no | fuori dal 2 |
| 🔶 la stella polare per intero all'apertura | il proprietario | no | la riga §12 del 17 scrive «si legge per intero», com'è già in testa alla stella | 
| 🔶 il richiamo in testa ad `AVVIO-CHAT.md` | il proprietario | no | fuori dal 2 |

### I. «Vicoli ciechi e trappole» — la frase del piano che li evita

| Vicolo | Nel piano | Esito |
|---|---|---|
| dockview.dev da Python (cp1252, 403, sitemap, `docs/` senza link) | n/a: il piano legge i `.d.ts` nel `.tgz` (P-80, compito 14 «letti nei `.d.ts` di `dockview-core` 8.3.1») | evitato |
| heredoc ~9 KB con righe lunghe | i 23 heredoc del piano ≤ 1 273 byte, riga ≤ 316 (R9b-16) | evitato |
| `MAX_PATH` dello scratchpad → `python - < percorso` | `python - <<'EOF'` da stdin; `replace_unique.py` in cima allo scratchpad (R9b-23) | evitato |
| `read_me` del widget; mermaid; `erDiagram`/`sequenceDiagram` | il piano non disegna | n/a |
| `docs/design/` ha nove file → il nuovo è il 10 | il piano non crea file in `docs/design/` (ma dovrebbe toccare il 10: R9b-2) | n/a |
| script con agganci asseriti che scrive solo alla fine; `git ls-files --eol` prima; agganci multiriga al fine-riga del file | `replace_unique.py` («refuses when the old text is absent or not unique … os.replace»); vincolo globale 4; il Passo 15 del 14 `assert b.count(anchor) == 1` | evitato |
| il cwd del tool Bash persiste | i comandi del piano sono `cd gui && … ; cd ..` in una chiamata sola; «Aprirla **nella cartella del repo**» nel diario | evitato |
| tetto di 30 000 caratteri per chiamata; tetti per tabella | il diario («questo file a … righe trabocca su 920–1081 … lì 20–60») | evitato |
| `grep '8\.2' scripts/check-docs.sh` non trova | n/a | n/a |
| il cancello in background, la quaterna non si ricava dal log | i criteri chiedono `GATE GREEN` e la baseline a `cargo test --workspace --no-fail-fast --locked` | evitato |
| «LF will be replaced by CRLF» per i file `w/lf` | vincolo globale 4 e le etichette per file (P-47 ha corretto le tre false) | evitato |
| il puntatore della §6 comincia a metà riga | 17 · D72 «il puntatore della §6 si RISCRIVE» | evitato |
| `tiktoken`; i 350k del proprietario | n/a | n/a |
| aggancio non unico → il primo dopo l'indice noto | `replace_unique.py` rifiuta i non unici; R9b-18 (le code delle celle dell'8 sono uniche, misurato) | evitato |
| riscrittura dei link solo sul testo mosso; `grep -n '\](\s*)'` | il piano non muove testo in archivio | n/a |
| `printf` con backtick fallisce | il piano scrive coi `python - <<'EOF'` | evitato |
| quattordici blocchi in parallelo reggono | n/a | n/a |
| un blocco `####` con la domanda A/B come citazione | D56 in A/B nel piano, riga `D` | n/a |
| `sed -i` toglie i CR; Python `newline=""`; `tr -cd '\r'` dopo | vincolo globale 4; ogni Passo sui CRLF nomina `replace_unique.py`; il 14 e il 16 rimisurano con `tr -cd '\r'` | evitato |
| togliere una riga di tabella spezza la tabella; l'`awk` che deve non stampare nulla | 14 · criterio «più `awk 'prev ~ /^\|/ …'`»; diario «Tabelle spezzate → niente» | evitato |
| una stima prima della misura sbaglia di tre volte | vincolo globale 5; R9b-15 è un caso in cui il comando **non** è stato provato prima di scriverne l'atteso | **caduto una volta** — R9b-15 |
| `python -` con `print` fuori da cp1252 | i Passi stampano solo ASCII o niente; il 14 stampa dal `grep` | evitato |
| la tabella delle decisioni trabocca a 65 righe → 30 | il diario lo dice | evitato |
| il repo si muove mentre si legge: `git fetch` prima di scrivere | diario «Come si riprende»: `git fetch --all --prune`, `git status -sb`, `git log --oneline -3` | evitato |

### J. «Il prossimo passo»

| Frase | Oggi | Chi lo aggiorna |
|---|---|---|
| «Lo dice la §6 del compendio, in un posto solo» | vero | 17 · D72 riscrive il puntatore |
| «la consegna … è la §10 del disegno del 2 … e da lì parte il piano» | vero (il piano è scritto) | — |
| «Le skill della sessione che scrive il piano: …» | stantia: il piano è scritto dal 2026-09-15 | nessuno — R9b-6 |
| la testa: «QUESTO DISEGNO È COMPLETO DAL 2026-09-09 … il prossimo passo lo dice la §6» | vero | — |
| la tabella dello stato: «codice e spec non toccati … `crates/` intatto» | vero oggi, **falso dopo il compito 1** | nessuno — R9b-6 |

## Voci P rimisurate

| P | Comando rilanciato → resa | Regge? |
|---|---|---|
| P-10 | `awk '/^pub enum IpcMessage/{s=1} s&&/^}/{exit} s&&/^    [A-Z]/{c++} END{print c}' crates/kernel/src/wire/ipc.rs` → **2** | sì |
| P-5 / P-21 | `grep -n 'SIX\|FIVE\|SEVEN' crates/kernel/src/ports/mod.rs` → righe 1, 22, 37, 43, 77 — le cifre in prosa ci sono ancora, e il compito 4 le tocca una per una (tabella delle sei righe) | sì |
| P-22 | `grep -n 'restano sei' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` → riga 209; `sed -n '575p;906p'` → le intestazioni «### 2.3 I/O — le famiglie di porte» e «### 3.1 Cosa sostituisce» (le frasi da toccare stanno sotto, il compito 4 le cerca col `grep` sulla frase) | sì |
| P-44 / D26 | Python `count` della coda della riga 56 della tabella del coordinatore nella stella → **1**; `tr -cd '\r' \| wc -c` sulla stella → 0 (LF) | sì |
| P-47 | `git ls-files --eol` dei due disegni e del piano → `i/lf w/lf` tutti e tre | sì |
| P-65 | `node --version` → **v24.9.0**; non sta in `^24.15.0` di `jsdom` 30.0.1: il prerequisito dichiarato dal piano (aggiornare Node prima del Passo 1 dell'11) vale ancora oggi | sì |
| P-80 | `grep -n 'non spedisce' <disegno2>` → riga 141 (richiamo del 2026-09-10 nella §2): «il richiamo è già scritto» è vero; `spikes/gui-shell/app/node_modules/dockview-core/package.json` → `"version": "8.2.0"` | sì |
| P-89 / D56 | Python `count` dell'ancora del Passo 15 del 14 nella stella → **1**; `grep -c 'RICHIAMO DEL <data>, dal compito 14' <stella>` → 0 (non eseguito, com'è giusto) | sì |
| P-102 / D67 | `ls scripts/` → `check-docs.sh gate-attributes.sh gate-deps.sh gate-no-os.sh gate.sh` (nessun `gate-gui.sh`, nessun `replace_unique.py` nel repo — vive nello scratchpad, com'è scritto) | sì |
| P-113 | `grep -cE 'sedici\|diciassette\|[0-9]+ compiti' docs/roadmap.md` → **2**; sulla riga del piano il match è dentro il richiamo del 2026-09-15 che cita il testo tolto (R9b-15) | **la voce sì, il criterio del 17 no** |
| P-116 | `grep -c 'sottoprogetto-2\|direzione-gui\|gui-minima' docs/README.md` → **0** | sì |
| [B] | `grep -c '^### P-' <piano>` → 116; `grep -c '^| \*\*D[0-9]' <piano>` → 74; `grep -c '^## Compito' <piano>` → 17; `grep -c '<the version' <piano>` → 5 (1 nel manifesto del 12, 4 righe del diario che citano il `grep`); `grep -c '^| \*\*E[0-9]' <piano>` → 2 (le due righe di [C], fuori dall'errata: l'`awk` del diario è scopato e rende 0) | sì |

## Numeri di compito ricensiti

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| D2 «il compito 12 lo scrive accanto al primo uso» | 12 | **13** (il primo uso di `dockview`; il 12 è il core finto) | stantio — R9b-8 |
| D4 «La misura si rifà al compito 10» | 10 | **11** (chi installa); la riga stessa lo dice in coda | stantio, autocorretto in coda — R9b-10 |
| D3 «il compito 13 rilegge … ✅ RICHIAMO … la rilettura è del compito 14» | 13 → 14 | 14 | corretto col richiamo (P-81) |
| D9 «lo consegnano il daemon (compito 9) e il core finto (compito 12), e il compito 9 dice da dove» | 9, 12 | 9, 12 | giusto |
| D13, D21, D25, D47 (i movimenti dichiarati) | 4→5, 7→9, 8 diviso, 14→13 | come la posizione | giusti |
| D56 «il compito 14 scrive il richiamo … Il 17 ne registra l'esito» | 14, 17 | 14, 17 | giusto |
| D66 (CI) — «compito 15» nella tabella della posizione; D68–D71 «compito 16» | 15, 16 | 15, 16 | giusti |
| [C] «X-1 e X-3 sono il compito 16» | 16 | 16 | giusto |
| [C] «sbarra il 13, non il 2», «il 3, con le run», «il 10», «il 12», «il 7», «il 6», «il 3, il 6, il 10» | sotto-progetti | sotto-progetti | giusti (non sono compiti) |
| Vincolo globale 1 «passano da sei a sette famiglie (compito 4)» | 4 | 4 | giusto |
| Compito 8 · Passo 12 «(b) … la riga 9 della §9» del 2 | riga 9 della §9 | è la seconda delle tre righe `\| 9 \|` del file; la coda è unica | giusto, ambiguo a occhio (R9b-18) |
| Compito 14 · Passo 15 «dal compito 14 del piano della parte 2» | 14 | 14 | giusto |
| Compito 17 · Passo 10 «l'ha già scritto il Passo 15 del compito 14» | 14 | 14 | giusto |
| Compito 17 · «D14 del piano della PARTE 1» (P-112) | D14 parte 1 | — | giusto, col nome del piano |
| Stella §6 «lo toccheranno il 3, il 6, il 10 e il 12» | sotto-progetti | — | non sono compiti; e il **2** lo tocca (compiti 8 e 14) senza essere nominato — prosa, dentro R9b-4 |

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

- `cat <scratchpad>/review/R9b-prompt.md` → il mandato
- `mkdir -p <scratchpad>/review/probe-R9b; wc -l -c skeleton.md p-titles.md` → 678 righe / 82 967 byte; 116 righe / 15 009 byte
- `grep -n '^#' skeleton.md | head -80` → le sezioni [A]–[D] e i 17 compiti
- Read `skeleton.md` (1–180, 181–360, 361–520, 521–678) → letto per intero
- Read `p-titles.md` → 116 titoli
- `grep -n '^#' <stella>` → le intestazioni (44, 66, 120, …, 1074)
- `wc -c <stella>` → 177 838; `sed -n '<range>p' | wc -c` per 15 intervalli → tutti sotto 30 000 tranne 66–119 (37 276, letto in due Read)
- `git ls-files --eol` dei tre file → `i/lf w/lf`; `git rev-parse --short HEAD` → `baf3cde`; `git status --porcelain | wc -l` → 0
- Read `<stella>` 66–92 e 93–119 → le decisioni 0–48
- `sed -n '181,344p' <stella>` → il modello e le tre sequenze
- `sed -n '600,742p' <stella>` → §2 e §3
- `sed -n '830,881p' <stella>` → §6 e la mappa delle sezioni
- `sed -n '882,943p' <stella>` → le decisioni del coordinatore 1–57
- `sed -n '944,1081p' <stella>` → registrate, vicoli, prossimo passo
- `grep -n '^## ' <piano>` → le sezioni del piano e le righe dei 17 compiti
- `grep -n '^| \*\*D[0-9]' <piano> | cut -c1-230` → le 74 righe D
- `grep -o 'decisione [0-9]\+' <piano> | sort | uniq -c` → 24 numeri citati (nessuna 11, 12, 13, 14, 22)
- `grep -o 'decisioni [0-9]\+ e [0-9]\+' <piano> | sort | uniq -c` → «41 e 42», «44 e 45»
- `grep -n 'direzione-gui-design' <piano>` → 20 righe; la stella è **modificata** solo ai compiti 8 (riga 9381) e 14 (riga 17451)
- `grep -n 'Electron' <stella>` → riga 745 (richiamo del 2026-09-10 nella §4)
- `grep -n 'RICHIAMO DEL 2026-09-0[89]\|riscritta il 2026-09-09' <disegno2>` → 16 righe (100–283)
- `grep -n -F '<k>' <piano>` per `boundary.rs`, `moveTo`, `LayoutPack`, `tetto di dimensione`, `MAX_BODY`, `Compatta`, `ledger`, `pub enum IpcMessage`, `ripristina`, `kit UI`, `esporta`, `navigazione spaziale`, `GUI minima (shell`, `registrat` → vedi rilievi (niente per `tetto di dimensione`, `ledger`, `ripristina`, `kit UI`, `navigazione spaziale`, `GUI minima (shell`)
- `sed -n '1,65p' <stella>` → la testa e la tabella dello stato (7 756 byte)
- `sed -n '4400,4475p' <piano>` → l'enum del compito 3 (14 varianti)
- `awk 'NR>=7650 && NR<=9367' <piano> | grep -o 'IpcMessage::[A-Za-z]*' | sort | uniq -c` → 14 varianti + `decode`/`encode`
- `awk 'NR>=7650 && NR<=9367' <piano> | grep -n 'refused\|rifiutat\|Refused'` → 9 righe
- `awk 'NR>=13655 && NR<=17519' <piano> | grep -o 'case "…"\|kind === "…"\|kind: "…"' | sort | uniq -c` → i generi trattati dagli store
- `sed -n '14160,14235p' <piano>` → `stores/layout.ts`; `sed -n '14870,14910p'` → `apply`
- `grep -n '^| \*\*D\(2\|3\|4\|9\|30\|37\|56\|66\|71\)\*\* |' <piano>` → le nove righe intere
- `grep -n 'not a \`Record\`\|boundary.rs' crates/platform/src/journal.rs` → riga 63
- `awk 'NR>=5477 && NR<=6553' <piano> | grep -n 'journal.rs'` → 15 righe: «una parola», «nessun altro tocco»
- `grep -n 'design/10\|10-modello\|design/09\|design/01\|design/02\|design/05\|docs/design' <piano>` → **niente**
- `grep -n 'GUI minima' docs/roadmap.md` → righe 163, 185, 186, 231
- `awk 'NR>=18540 && NR<=18801' <piano> | grep -n 'roadmap'` → Passo 5 e criterio
- `grep -n 'Non è una spec e non disegna' docs/COMPENDIO.md docs/README.md` → niente
- `sed -n '18596,18612p' <piano>` → le quattro righe della §12 dettate dal 17
- `grep -n '^## \|^### ' docs/README.md` → «Dove va cosa» 107, «Specifiche» 195
- `grep -n 'git fetch\|MAX_PATH\|python - <\|cd /c/Users\|heredoc\|run_in_background\|30 000\|prev ~ /' <piano>` → 23 heredoc, `git fetch` nel diario, l'`awk` delle tabelle
- `grep -n -o '.\{90\}decisione \(2\|4\|5\|9\|16\|19\|30\|34\|37\)\b.\{50\}' <piano>` → 3 righe (19 e 4 del coordinatore, 5 nell'ancora del 14)
- `grep -n 'beforeunload\|pagehide\|visibilitychange' <piano>` → riga 14861
- `grep -n 'a riposo\|senza soglia' <piano>` → riga 2056 (la fascia), nessuna misura
- `grep -n '8\.3\.1\|8\.2\.0' <stella>` → solo la riga 76 (decisione 6, 8.2.0)
- `sed -n '17436,17470p' <piano>` → Passo 15 del 14
- `sed -n '18662,18700p' <piano>` → Passi 5–7 del 17
- `grep -n 'col 2\|(col \|richiamo datato\|RICHIAMO\|erDiagram' docs/design/10-modello-dei-dati-durevoli.md` → righe 19–20 (la regola), 98 (`INVOCATION_DETAIL (col 2)`)
- `grep -n 'gesti-design\|knowledge-base-design' docs/COMPENDIO.md` → 655, 656, 917, 919
- `grep -in 'idle\|riposo\| cpu' <piano>` → 3 commenti di codice
- `awk 'NR>=18540 && NR<=18801' <piano> | grep -n 'direzione-gui\|stella polare'` → 7 righe, nessuna modifica alla stella
- `awk 'NR>=13655 && NR<=15415' <piano> | grep -n '8\.2\.0\|apply(\|settle(\|onDidLayoutChange\|canonical(\|search\|ricerca\|"chat"\|beforeunload'` → 26 righe (nessun 8.2.0)
- `sed -n '8884,8898p' <piano>` → doc di `Core::attending`
- `awk 'NR>=7650 && NR<=9367' <piano> | grep -n 'keep(\|retrieve(\|rejects\|the old\|refuses'` → `keep` a 1488, `retrieve` a 984 e 1580
- `awk 'NR>=5477 && NR<=6553' <piano> | grep -n 'JSON\|reopen\|atomic\|not json'` → promessa 1, `what_was_kept_survives_reopening_the_file`
- `awk '/^### P-65 /…' <piano>` → la voce intera
- `awk 'NR>=18169 && NR<=18540' <piano> | grep -n 'rust-toolchain\|shell:\|Git Bash\|bash scripts'` → le due direzioni della decisione 44 nel criterio del 16
- `awk … heredoc` (fallito: regexp non terminata) → rifatto in Python
- `ls scripts/` → 5 script; `grep -c 'replace_unique' <piano>` → 11
- `grep -n 'promote\|Record::V1\|fn note' crates/kernel/src/boundary.rs` → `Untrusted::promote`; `grep -rln 'note(.*, b"' crates/*/tests` → 5 banchi
- `node --version` → v24.9.0
- Write `probe-R9b/heredocs.py`; `python …/heredocs.py` → 23 blocchi, max 1 273 byte, riga max 316, 12 con backslash
- `grep -c 'RICHIAMO DEL' <stella>` → 11
- `awk 'NR>=18540 && NR<=18801' <piano> | grep -n "RICHIAMO DEL"` → solo il `grep -c`, senza atteso
- Python: `count` dell'ancora del 14 → 1; della riga 56 → 1; CR → 0
- `sed -n '10186,10200p' <piano>` → il richiamo nel banco del 7 (compito 8)
- `grep -n -B8 'fn keep(' <piano> | grep -E 'impl .* for|fn keep'` → 10 implementazioni di `Custody`
- `awk 'NR>=13655 && NR<=15415' <piano> | grep -n 'it("\|test("'` → 18 sonde del 13
- `sed -n '14706,14745p' <piano>` → il generatore delle viste
- `sed -n '14815,14830p' <piano>` → `same` e il commento sulla decisione 12
- `sed -n '16726,16736p' <piano>` → la nota «8.3.1 … non ricordati dallo spike sulla 8.2.0» nel 14
- `awk 'NR>=10266 && NR<=11126' <piano> | grep -n 'misur\|a riposo\|dorme'` → le misure del 9: nessuna sul processore
- `grep -n 'archivio\|configurazione\|custodia\|CONFIG' docs/design/10-modello-dei-dati-durevoli.md` → riga 5 (prosa): la disposizione non è nel modello
- `ls spikes/gui-shell/app/node_modules/dockview-core/dist` → il bundle 8.2.0 c'è
- `sed -n '18738,18749p' <piano>` → il blocco dei comandi della Definizione di «fatto»
- `grep -n 'IpcMessage::Accepted(' <piano> | head -1` → 4466 (era lo `stamp_set` del 3: ripetuto sul 7)
- Python: coda della riga 9 della §9 nel 2 → 1; CR nel 2 → 0; righe `| 9 |` → 3
- `grep -o '"version": *"[^"]*"' …/dockview-core/package.json` → 8.2.0
- `grep -n '_onDidLayoutChange\|onDidLayoutChange\b' …/dist/dockview-core.js` → `_bufferOnDidLayoutChange.onEvent` (2508), `AsapEvent` (2507)
- `sed -n '2520,2540p' …/dist/dockview-core.js` → gli inneschi: `gridview.onDidChange`, `onDidAdd/onDidRemove/onDidActiveChange`, `setVisible`
- `awk 'NR>=7650 && NR<=9367 && /IpcMessage::Accepted\(/{print NR}' <piano> | head -1` → 8329; `sed -n '8323,8369p'` → l'ordine dell'accoglienza e `heard.len() == 5`
- `sed -n '28,46p' <piano>` → `replace_unique.py` vive nello scratchpad, sorgente nel piano
- `grep -n 'foglio di stile\|non spedisce\|dockview/dist' <stella> <disegno2>` → `<disegno2>:141`
- `python - <<'EOF'` nella cartella di prova con backslash e riga da 360 → `EXIT=0`
- `awk 'NR>=5114 && NR<=5477' <piano> | grep -n 'SIX\|FIVE\|SEVEN\|EIGHT'` → la tabella delle sei righe e il doc nuovo
- `awk 'NR>=13655 && NR<=15415' <piano> | grep -n '8\.3\.1\|otto mosse\|eight moves'` → «le otto mosse» sì, «8.2.0» no
- `grep -n 'SIX\|FIVE\|SEVEN' crates/kernel/src/ports/mod.rs` → 1, 22, 37, 43, 77
- `awk '/^pub enum IpcMessage/…' crates/kernel/src/wire/ipc.rs` → 2
- `grep -n 'restano sei\|sei famiglie\|sei porte' <spec SP1>` → 209; `sed -n '575p;906p'` → le due intestazioni
- `grep -cE 'sedici|diciassette|[0-9]+ compiti' docs/roadmap.md` → 2; `grep -nE …` → 87, 231; `sed -n '231p' | grep -oE '.{40}sedici.{40}'` → il richiamo del 2026-09-15
- `grep -n 'in scrittura dal 2026-09-11' docs/roadmap.md` → 231
- `grep -c 'sottoprogetto-2\|direzione-gui\|gui-minima' docs/README.md` → 0
- `grep -c '^### P-'`, `'^| \*\*D[0-9]'`, `'^## Compito'`, `'^| \*\*E[0-9]'`, `'<the version'` su `<piano>` → 116, 74, 17, 2, 5
- `grep -n '^| \*\*E[0-9]' <piano>` → le due righe di [C]; `grep -n "E\[0-9\]" <piano>` → gli `awk` scopati del diario
- `git status --porcelain | wc -l` → 0 (all'apertura); `git -C /c/Users/zagor/Desktop/harness status --porcelain` → vuoto (alla chiusura, sotto)

## Non verificato, e perché

- **La sonda «`SaveLayout` su un archivio che rifiuta la scrittura → `Layout` col vecchio» (§2 riga 5, sequenza 2, ramo «scrittura fallita»).** Le dieci implementazioni di `Custody` nel piano (`grep -n -B8 'fn keep(' <piano>`) sono la finta del 4, `MemoryCustody`, i cinque bugiardi della suite del 5, `FileCustody`, `MaybeCustody`; nessuna rifiuta `keep` **tenendo** il vecchio, e i banchi del 7 non ne montano una (il `grep` su `rejects\|the old\|refuses` nel 7 rende solo il registro). Non ho letto i Passi del 7 e del 10 per intero (sono di altri revisori): lo dichiaro come **possibile sonda mancante** per la riga 5 della §2, da confermare da R-7/R-10 col `grep -n 'Err(CustodyError' <piano>` nei loro perimetri.
- **`package-manager-cache` in `actions/setup-node` v7 (P-103, D66)** non rimisurato alla fonte (rete non usata per GitHub); riportato com'è nel piano.
- **La resa nel browser** delle sonde del 13 (`frame.test.ts`, `views.test.ts`) non è stata eseguita: `gui/` non esiste e il repository non si tocca. R9b-11 è letto **sul codice dettato**, non fatto girare; la sequenza di chiamate (`switchTo` → `apply(api, view, unpack(layout.state))` → `fromJSON(pack.layout)`) è testuale nel piano.
- **Il comportamento di `onDidLayoutChange` su `fromJSON` e sul pannello attivo** è letto nel bundle **8.2.0** dello spike, non nella 8.3.1 appuntata da D2 (non installata da nessuna parte): la lettura vale per la versione delle otto mosse.
- **La §1 della stella** (le righe da marcare «costruito», R9b-4) e la **§4** non sono state lette, per mandato: R9b-4 riguarda la **regola** della §6 e della testa, non le righe.

## Stato finale

`git -C /c/Users/zagor/Desktop/harness status --porcelain` → (vuoto) · `git ls-files --eol` di piano e disegni → `i/lf w/lf` invariato (`docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md`, `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md`, `docs/superpowers/specs/2026-09-07-direzione-gui-design.md`) · le prove in `<scratchpad>/review/probe-R9b/` (`heredocs.py`, `heredoc-probe.txt`, `part2.md`, `part3.md`).
