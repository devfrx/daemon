# Rapporto R9a — Copertura del DISEGNO DEL 2 (§1–§10, risposte, decisioni, vicoli ciechi) contro il piano della parte 2 — 2026-09-15

Perimetro: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` letto **per intero** (726 righe, LF); del piano le righe `D` (74), le voci `P` a domanda, i Passi dei compiti solo dove un Passo doveva esistere davvero. Repository a `HEAD baf3cde`, albero pulito prima e dopo. Nessun file del repository toccato; le prove in `<scratchpad>/review/probe-R9a/` (i compiti spezzati in `compito-NN.md`, la testa, il pre-controllo, le decisioni, l'undicesima chiusura).

## Esito in tre righe

1. Il disegno è coperto nel merito: le 14 varianti, i 18 tipi di modulo, i 4 messaggi del ponte, i record congelati 6→7→8, le 3 deduzioni della §8, le decisioni 34/35/46–50 e le 18 voci della §9 hanno ciascuna un compito o una `D`/`P` che le dichiara. Ogni cifra del disegno che il piano ripete coincide col comando.
2. **Due rilievi bloccano** — entrambi sonde/richiami che il piano *dice* di produrre e non detta: il compito 12 non ha nessun Passo che scriva il richiamo di D41 nella §7 (il suo criterio cerca una data fissa che oggi nel disegno vale 0), e il compito 9 detta quattro sonde col corpo `{ /* … */ }`, fra cui le due della decisione 35 della §8.
3. Il resto sono contraddizioni **dichiarate in una `D` ma senza richiamo nel disegno** (§3 `send`/`receive` per D9/D10; la suite `ipc` che non gira sulle finte; §8 «decodificata dai byte» per D36; le tre righe che D5 rende false oltre a quella che riceve il richiamo) e **tre promesse del disegno che nessun compito onora e nessuna voce aperta dichiara**: la CPU a riposo del daemon (§9 voce 5, decisione 41 del proprietario), il commento falso di `crates/platform/src/journal.rs` (§9 voce 10, decisione 18 del proprietario, e il compito 5 tocca quel file), i richiami nella §1 della stella polare per i moduli costruiti (§10 punto 9).

## Rilievi

| # | Compito · dove (frase da cercare col grep) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| R9a-1 | Compito 12 · *Files*: «**un** richiamo datato nella §7 (**D41**)»; criterio: «la §7 del disegno porta il richiamo datato di **D41**» | Il compito modifica il disegno con un richiamo di D41 sulla §7 («senza copiarla»), e il criterio di chiusura lo verifica con `grep -c 'RICHIAMO DEL 2026-09-14' <disegno2>` → più di zero | `grep -n 'Passo [0-9]*:' probe-R9a/compito-12.md` → dieci Passi (misure, exclude, `Core::ipc`, manifesto, costanti, rubinetto, console, sonde, direzioni, cancello): **nessuno** scrive nel disegno; `grep -n -i 'richiamo' compito-12.md` → solo la riga *Files* e il criterio; `grep -c 'RICHIAMO DEL 2026-09-14' <disegno2>` → **0** oggi; `grep -c -F 'senza copiarla' <disegno2>` → 1 (la frase c'è, alla riga 374). L'undicesima chiusura elenca questa riga fra le tre «che aspettano il loro compito» e la assegna al 12 | CONFERMATO | fatto | sì — eseguito com'è scritto, il criterio è rosso oppure il richiamo viene improvvisato senza testo dettato; e la data fissa `2026-09-14` è una data di scrittura in un testo per un'altra sessione (tutti gli altri richiami del piano portano `<data>`) | Un Passo nuovo del compito 12, prima del cancello: il testo del richiamo (D41: il finto **riscrive** `build_the_arbiter` coi propri letterali, le tre case che si nominano a vicenda), ancora unica «*diventi raggiungibile dal finto **senza copiarla***» (conteggio 1), Python con `newline=""`; criterio riscritto su `grep -c 'RICHIAMO DEL <data>, compito 12'` → 1 |
| R9a-2 | Compito 9 · Passo 12: «**le tre sonde delle decisioni — l'archivio chiuso, la policy riletta, il secondo core**» | Quattro `#[test]` dettati con firma e doc, e il corpo `{ /* … */ }`; poi: *«I quattro corpi si scrivono al momento … Non sono segnaposto … un corpo vuoto è un segnaposto, che la revisione del piano intero cerca»* | `grep -n '/\* … \*/' probe-R9a/compito-*.md` → **4** righe, tutte nel compito 9 (`a_layout_archive_that_will_not_open_lets_the_core_start`, `a_core_started_on_a_broken_archive_answers_unavailable_to_every_save`, `the_policy_in_the_journal_is_the_one_the_arbiter_starts_on`, `a_second_core_on_the_same_channel_stops_the_start_up`); nessun altro compito ne ha. Le prime due sono **la sonda della decisione 35** che la §8 detta (*«archivio che non si apre → il grafo parte, `Layout` «non disponibile», `SaveLayout` → `Layout` «non disponibile»»*) | CONFERMATO | fatto | sì — un `#[test]` col corpo vuoto compila ed è **verde**: quattro sonde vacue se il testo è eseguito alla lettera; e la seconda direzione di `MaybeCustody` (D30: «la sua sonda deve provare **entrambi** i rami») non è dettata | Dettare i quattro corpi nel Passo 12 sulla forma delle sonde sopra (`run_the_graph` con limite finito e tick nullo, il pari che manda `Hello` + `SaveLayout` e riceve `Layout(Unavailable)`, la directory inesistente come in `a_journal_that_cannot_be_opened_stops_the_start_up`), e nel criterio di chiusura `grep -c '{ /\* … \*/ }' crates/daemon/src/main.rs` → **0** più una mutazione per ciascuna al Passo delle due direzioni |
| R9a-3 | Compito 2 · *Files* (nessun disegno) e Passo 5 «THE CONFORMANCE SUITE OF THE `ipc` PORT»; §3 del 2 righe `send`, `receive`, 🔶 dedotto | D10: il trasporto **non** incornicia, `send` scrive verbatim, `receive` rende la cornice intera busta compresa; D9: `MalformedMessage` solo dal tetto, flusso avvelenato | §3 del 2 dice `send`: *«incornicia con `kernel::framing::frame` e scrive»* e `receive`: *«cornice intera → `Ok(Some(corpo))`; … cornice rotta → `MalformedMessage`»*; il banco dettato lo smentisce alla lettera (`a_whole_frame_comes_back_whole_envelope_included`, `a_body_over_the_cap_is_malformed_and_the_client_stays`). `grep -n 'richiamo\|§3' compito-02.md` → richiami solo su `platform/src/lib.rs` (P-25) e `ports_are_implementable.rs` (P-3): **nessun richiamo nella §3**, e il disegno non è nei *Files*. Anche il 🔶 dedotto della §3 (*«che anche i flussi di `interprocess` leggano senza bloccare»*) è **verificato** al compito 2 («(a) L'API non bloccante … è VERIFICATA, non dedotta») senza che la riga lo riceva | CONFERMATO | prosa | no | Un Passo nel compito 2 (prima del cancello) con tre richiami `<data>` nella §3: cella `send` (D10), cella `receive` (D9 + D10), riga 🔶 dedotto (misurato al Passo 2); ancore uniche misurate: `| \`send\` |` e `| \`receive\` |` sono righe della sola tabella §3; il disegno è LF → Python `newline=""`; *Files* aggiornato |
| R9a-4 | Compito 2 · Passo 5 «WHAT IS DELIBERATELY ABSENT» e Passo 9 «riporta zero test — è voluto»; §3 riga «la suite di conformità», §8 riga «il trasporto `ipc` in `platform`» | La suite è una `macro_rules!` espansa **solo** da `crates/platform/tests/ipc_contract_real.rs`; il banco del kernel riporta zero test | Il disegno dice due volte che la suite *«gira su `FakeGui`, `DyingGui` e il trasporto vero»* (§3) / *«su `FakeGui`, `DyingGui` e il trasporto vero con un pari su un thread»* (§8). `grep -n 'FakeGui\|DyingGui' compito-02.md` → **0** righe; `grep -n 'include!' compito-02.md` → un solo sito, in `platform`. P-27 spiega perché la forma è una macro, **non** dice che le due finte restano fuori; nessuna `D` lo dichiara, nessun richiamo nel disegno | CONFERMATO | fatto | no — le tre sonde della macro girano sul trasporto vero e il contratto è tenuto; ma la §8 promette una prova sulle finte che non esiste | O si espande la macro anche in `crates/kernel/tests/` con la fabbrica di `FakeGui` (e `DyingGui` dal simulatore, già dev-dependency) — tre sonde, costo nullo — oppure una riga `D` che dichiara «la suite gira solo sul trasporto vero, perché le sue promesse vogliono un pari» e il richiamo `<data>` sulle due righe del disegno nello stesso Passo di R9a-3 |
| R9a-5 | §9 del 2, voce 5 «la prontezza I/O del reattore» · consiglio: «il piano riporta la CPU a riposo del daemon col tick, **senza soglia**, così il 3 decide con un numero» · ✅ decisione 41 della stella polare | — (nessun compito la nomina) | `grep -n -i 'riposo' <piano>` → **0**; `grep -n -i '\bcpu\b' compito-07/09/12/15/17.md` → **0**; `grep -n -i 'senza soglia' <piano>` → solo P-78 (la fascia). Decisione 41 della stella polare, riga 111: *«A, … e il piano misura il processore a riposo senza soglia»* — confermata dal proprietario. Non è nella tabella [C] delle voci aperte che il piano SA | CONFERMATO | fatto | no — nessun rosso; ma è una promessa al proprietario che il piano non mantiene né dichiara | Un Passo di misura nel compito 9 (il primo grafo col tick di produzione): il daemon a riposo per un intervallo dichiarato, CPU dell'albero di processi col comando, il numero in `riferimenti.md` al compito 17 con comando e data; oppure una riga in [C] col chiusore («il 3, con la rete») e il richiamo sulla voce 5 |
| R9a-6 | §9 del 2, voce 10 «il commento falso in `crates/platform/src/journal.rs`» · chiusore: «il piano del 2, il primo compito che tocca il file» · consiglio: «si corregge lì, decisione 18 del proprietario»; Compito 5 · *Files*: «`crates/platform/src/journal.rs` — ⛔ **una parola**: `engine` da privata a `pub(crate)`» | Il compito 5 tocca il file per una parola (D14) | `grep -l 'platform/src/journal.rs' compito-*.md` → **solo** il compito 5; `grep -n -i 'commento falso\|false comment\|decisione 18\|boundary.rs' compito-05.md` → 0 (solo la propria `TableDefinition`); il commento è ancora lì oggi: `grep -n 'boundary.rs' crates/platform/src/journal.rs` → riga 63 *«`boundary.rs` writes some that are not a `Record` at all»*; la voce registrata della stella polare (riga 966) lo dice **falso** col chiusore «il piano del 2, nel primo compito che tocca `crates/platform/src/journal.rs`». Non in [C] | CONFERMATO | fatto | no — è un commento; ma il compito 5 è precisamente «il primo che tocca il file» e lascia il falso in un file che riscrive | Nel Passo del compito 5 che rende `engine` `pub(crate)`: sostituire la frase del doc (riga 63) con quella vera della voce registrata (*«`Untrusted::promote` scrive un `Record::V1` normale; byte che non sono un record li scrivono solo i banchi»*), *Trova* col `grep -c -F` (CRLF: `replace_unique.py`), e la voce 10 della §9 riceve il ✅ `<data>`; oppure una riga in [C] «il proprietario, prima» |
| R9a-7 | §8 riga «la SPA, `schema/` (§6a, risposta 10)»: «ogni variante **decodificata dai byte** e confrontata col valore atteso in JSON»; §2 richiamo del 2026-09-10: «la SPA parla `bincode` sull'`ipc` del kernel»; Compito 11 · *Files* (nessun disegno) | D36 (decisione del proprietario, 2026-09-14, B): la prova dello schema confronta i tipi TypeScript col `.json`, **non decodifica i byte**; la decodifica vera è del guscio ([C]) | `grep -c -F 'ogni variante decodificata dai byte' <disegno2>` → 1; `grep -c -F 'la SPA parla \`bincode\`' <disegno2>` → 1; nessun compito modifica il disegno per D36 (il compito 11 non ha il disegno nei *Files*; `grep -n '§8' compito-11.md` → solo letture) | CONFERMATO | prosa | no | Un richiamo `<data>` (D36) sulla riga «la SPA, `schema/`» della §8 e sulla frase della §2, scritto dal compito 11 (che crea la prova) o raccolto dal 17 con gli altri; ancore uniche misurate sopra |
| R9a-8 | §7 del 2, tabella «Pezzo / Forma / La prova che lo esercita», righe «la disposizione» (*«il giro salva → ritrova sul pari, dentro la sonda dell'attività»*) e «la lista dei passi» (*«un `Invoke` dal pari, poi la lista che torna porta l'invocazione»*); Compito 12 · Passo 8 «le sonde, in fondo a `main.rs`» | Sei sonde: `the_welcome_gives_the_sequence_one`, `a_stale_stamp_is_refused_and_then_silence`, `degrade_makes_the_real_code_report_it`, `verdict_reaches_the_peer`, `the_tokens_arrive_untrusted`, `the_faucet_keeps_streaming_with_no_word_waiting` | `grep -n 'SaveLayout\|Layout' compito-12.md` → solo la mappa della sequenza 1 e il criterio «nessun ramo di dispaccio»; `grep -n 'Invoke' compito-12.md` → un solo `Invoke`, dentro `a_stale_stamp…` (col timbro sbagliato: prova il silenzio, non l'invocazione); `grep -n -i 'salva\|ritrova\|invocazione\|invocation' compito-12.md` → **0**. Le due prove che la §7 mette *«dentro la sonda dell'attività»* non sono dettate e la loro assenza non è dichiarata | CONFERMATO | fatto | no — il meccanismo è provato al compito 7 (`save_layout_comes_back_with_what_the_port_holds`, `an_approve_changes_the_policy_and_the_gui_is_told`) sulla **stessa** attività | O due sonde nel Passo 8 del 12 (il pari manda `Hello` + `SaveLayout` e riceve `Layout(Package)`; il pari manda `Hello` + `Invoke` + `Approve` e la lista che torna porta l'invocazione — l'unica prova che la settima porta in memoria e il registro siano **cablati** nel finto), oppure il richiamo `<data>` sulle due righe della §7 («provato al compito 7 sull'attività, non ripetuto nel finto») nello stesso Passo di R9a-1 |
| R9a-9 | §8 riga «la settima porta», sesta prova: «`SaveLayout` su un archivio che rifiuta la scrittura → `Layout` col vecchio»; P-28 la assegna al **7**; Compito 7 · `save_layout_comes_back_with_what_the_port_holds` | P-28: *«`SaveLayout` su un archivio che rifiuta la scrittura → `Layout` col vecchio — 7, l'attività che dispaccia»* | `grep -n -i 'impl Custody\|MemoryCustody\|Refusing' compito-07.md` → il banco monta **solo** `simulator::custody::MemoryCustody` (`type BenchCore<'b> = Core<FakeIpc<'b>, MemoryJournal, MemoryCustody>`); l'unica sonda della disposizione scrive `[7,7,7]` e riceve `[7,7,7]`: **il percorso buono**; il commento dice *«a failed write comes back as the OLD package»* ma nessuna custodia rifiuta; `grep -n -i 'keep' compito-07/09/10.md \| grep -i 'err\|fail\|refus'` → nessuna sonda | CONFERMATO | fatto | no — la seconda direzione (la scrittura rifiutata) è un'affermazione nel commento, non una sonda | Nel banco del compito 7 una custodia locale che rifiuta `keep` dopo il primo (un `bool` in un tipo del banco, come il filo che muore), e la sonda: due `SaveLayout`, il secondo rifiutato → `Layout(Package(primo))`; oppure P-28 corretta e la riga della §8 col richiamo che la manda alla suite del 5 (dove però `Layout` non esiste, come P-28 stessa argomenta) |
| R9a-10 | D5 e il suo richiamo (Compito 7 · Passo 8, solo la riga `Request` della §5); §4 riga «`Request`, `Verdict` \| GUI, core \| come oggi \| il 3D non è nel 2: **il daemon risponde**»; «Le risposte del proprietario», riga «strada»: «il daemon vero … **manda degrado, verdetti e policy**»; §8 riga «il limite di giri e `Disconnected`»: «la GUI che muore con una **concessione ordinaria** → `on_disconnect`» | D5: il ramo `Request` non chiama `admit` e non risponde; il richiamo va alla riga `Request` della §5. Compito 9 Passo 13: *«Nel 2 nessuna concessione ordinaria esiste — D5 … La metà che NON tiene si dichiara accanto alla sonda»* | `grep -n 'RICHIAMO' compito-07.md` → tre celle della §5 (`Hello`, `Request`, «il limite di giri») più la riga 2 di *Stato* della stella polare: **nessun** richiamo sulla riga `Request, Verdict` della §4, sulla risposta «strada», sulla riga della §8; `grep -c -F '| \`Request\` |' <disegno2>` → 1 (la §4 usa `| \`Request\`, \`Verdict\` |`) | CONFERMATO | prosa | no | Nel Passo 8 del compito 7 una quarta cella: la riga `Request, Verdict` della §4 (D5, «il daemon **non** risponde nel 2»); nel Passo 13 del compito 9 il richiamo `<data>` sulla riga della §8 («la metà della concessione ordinaria è del 7, pilastro 3D»); la riga «strada» è una risposta del proprietario: si lascia, e lo dice il richiamo della §4 |
| R9a-11 | §8, capoverso «**Ciò che la §8 non fa:** la CI resta solo Linux (X-1 dell'audit, del proprietario) … la scansione degli avvisi di sicurezza (X-3, del proprietario)»; Compito 16 · *Files* | Il 16 porta la matrice Windows e `cargo audit`/`npm audit`, e chiude X-1 e X-3 nell'audit col richiamo | `grep -n '§8' compito-16.md` → una sola riga (`actions/checkout` resta `v4`); `grep -c -F 'la CI resta solo Linux' <disegno2>` → 1; nessun richiamo sul capoverso. La voce 13 della §9 porta già il ✅ delle decisioni 44/45, quindi il capoverso è **già** in contraddizione con la §9 dal 2026-09-09 e il 16 lo rende falso nel codice | CONFERMATO | prosa | no | Nel compito 16, accanto ai richiami sull'audit: un richiamo `<data>` in coda al capoverso «Ciò che la §8 non fa» (X-1 e X-3 fatte, decisioni 44 e 45), Python `newline=""` (LF) |
| R9a-12 | §9 voce 3: «il piano: `npm run lint` in `gate-gui.sh`, **richiamo alla riga delle scritte della §8**»; §8 riga «le scritte, `locales/it.json` (G21)»: «un controllo che vada rosso … **se** al piano esiste una regola di lint matura; altrimenti revisione, e lo si dice»; Compito 15 · Passo 11 | Il Passo 11 del 15 scrive **un** richiamo, sui 🔶 dedotti della §8 (P-104) | `grep -n 'scritte' compito-15.md` → **0**; il Passo 11 tocca solo il capoverso dei dedotti («*e non riusi quello del workspace.*»). La riga delle scritte resta condizionale mentre il 15 decide (D65: `no-raw-text` a `error`, la rete del 13 muore per metà) | CONFERMATO | prosa | no | Il Passo 11 del 15 scrive anche il richiamo `<data>` sulla riga «le scritte» della §8 (la regola esiste ed è a `error`, D63/D65; la seconda sonda di `copy.test.ts` resta, P-105) |
| R9a-13 | §10 punto 9: «A piano eseguito: … **e i richiami datati nella §1 della stella polare per i moduli che il 2 ha costruito**»; Compito 17 · *Files* (nessuna stella polare); Compito 14 · Passo 15 | Il 14 scrive **un** richiamo (D56) sulla riga 1 di *Passi*; il 17 rilancia `grep -c 'RICHIAMO DEL' <stella>` senza toccarla | `grep -n 'direzione-gui-design' compito-13/14/17.md` → il 13 la legge, il 14 vi scrive solo D56 (`assert b.count(anchor) == 1`, ancora misurata **1** oggi), il 17 la conta. Le tabelle Chat/Stato/Permessi/Passi/Attività e la corta (righe 359–580 della stella) portano le colonne «chi» e «stato» che il §10 chiede di richiamare; nessun compito lo fa e nessuna voce in [C] lo differisce | CONFERMATO | prosa | no | Un Passo nel compito 17: i richiami `<data>` nelle cinque tabelle piene della §1 (le righe con «2» nella colonna «chi» → costruite, col nome del sorgente `gui/src/panels/*.vue`) e nella riga *Impostazioni* della corta (P-85); ancore le righe intere, come il Passo 15 del 14 |
| R9a-14 | Compito 10 · Passo 8 «il richiamo datato nella §5 del disegno del 2»: «in coda alla colonna *«la prova»* della riga *«l'attività»*» | Il testo del richiamo è dettato; **nessuna ancora** e nessuno script (i Passi 15 del 14 e 11 del 15 danno `assert b.count(anchor) == 1` / `grep -n`) | La cella finisce con *«lascia il passo A in dubbio con la sua classe»*: `grep -c -F` → **2** (§5 e la riga «l'attività del daemon» della §8); `grep -c -F "| l'attività |"` → 1 (la testa di riga è unica). Stessa forma senza ancora nei Passi 8 del 7, 9 del 6, 15 del 9: lì le code di cella sono uniche (misurate: «e la lista dei passi (§2 e §3 della stella polare) \|» 1, «dichiarato, non pinzato (gotcha #73) \|» 1, «servirebbero due limiti» 1, «un arresto pulito coi segnali» 1, «\| il giornale \|» 1, «\| l'invocatore \|» 1, «un record congelato in più \|» 1) | CONFERMATO | fatto | no — l'esecutore che ancora sulla coda della cella la trova doppia e `replace_unique.py` si ferma; chi ancora sulla testa di riga passa | Nel Passo 8 del 10 l'ancora esplicita — la riga intera `| l'attività | … con la sua classe |` presa dal file col `grep`, `assert count == 1` — come fanno il 14 e il 15 |
| R9a-15 | P-104 · titolo «la terza deduzione della **§9** è MISURATA»; ultimo capoverso: «il richiamo che la porta da «dedotto» a «misurato» lo scrive il **17**» | Compito 15 Passo 11: «Il richiamo si scrive **qui**»; l'undicesima chiusura: «il **Passo 11 del compito 15**» | `grep -n 'manifest-path. compili nel' <disegno2>` → riga **474**, che sta nel capoverso «🔶 **Dedotto**, da confermare al piano» della **§8** (la §9 vi entra solo come *«l'ambiente scelto in §9»*); `grep -n 'Definizione\|stella' compito-17.md` → il 17 non tocca il disegno del 2 (non è nei suoi *Files*) | CONFERMATO | prosa | no | P-104: «§8» nel titolo e nella prima riga; ultimo capoverso «lo scrive il **15**, Passo 11» |
| R9a-16 | §8 riga «il codice fuori dal perimetro»: «`git diff --stat` **a fine piano** tocca solo ciò che le tabelle nominano; `ports/mod.rs` e la spec solo coi richiami datati»; Compito 17 · Passo 9 «la Definizione di «fatto» della parte 2» | La Definizione è «comandi, non affermazioni», «almeno» dieci comandi elencati | I dieci comandi del Passo 9 (`gate.sh`, i due `grep -c` sulle etichette, `check-docs.sh`, `gate-deps.sh`, `gate-attributes.sh`, `ls frozen`, `grep Custody`, la catena npm, `git status --porcelain`): **nessun `git diff --stat <base>..HEAD`** sull'intero piano; il solo diff del compito è `git diff --stat HEAD~1 -- crates/ gui/ scripts/ .github/` (vuoto, del proprio commit). La riga della §8 è l'unico controllo del perimetro **di tutto il piano** e nessun comando la esegue | CONFERMATO | prosa | no | Nella Definizione di «fatto» (Passo 9 del 17): `git diff --name-only 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml gui/ docs/superpowers/specs/ docs/adr/` confrontato con l'unione delle liste *Files* (ogni nome deve comparire in una), e `git diff --stat 42b50d8..HEAD -- docs/adr/` **vuoto** (vincolo globale 10) |
| R9a-17 | §8 riga «capo a capo: la SPA nel guscio col core finto»: «**dopo lo spike, parte 2 del piano**: la prova del ponte in Node o in Rust secondo il vincitore, e le prove capo a capo con l'attrezzo scelto in §9» | Il guscio non è un compito di questo piano | `grep -n 'capo a capo' <piano>` → P-2 («non si installano `@playwright/test` e `@axe-core/playwright`»), P-53 («il guscio, che questo piano NON costruisce»), D31, [C] (tre voci col guscio come chiusore); la §1 del 2 stessa mette *«cornice nativa, menu, pacchetto, avvio automatico → dopo M1–M5 e il 10»*, quindi la riga della §8 era già in tensione col perimetro. Dichiarato, senza richiamo sulla riga | GIÀ COPERTO da P-53 / D31 / [C] | prosa | no | Nulla di obbligato; al più una parola nel richiamo di R9a-7 sulla stessa tabella |
| R9a-18 | Compito 6 · criterio: «`grep -c 'RICHIAMO DEL 2026-09-11' <disegno2>` → **almeno 2**»; Passo 9: i due richiami P-30 e P-32 portano «RICHIAMO DEL 2026-09-11, dal pre-controllo del compito 6» | Data fissa, non `<data>` | `grep -c 'RICHIAMO DEL 2026-09-11' <disegno2>` → **0** oggi, quindi il criterio non è vacuo; la data è quella del **pre-controllo** che ha trovato il falso (P-30/P-32 sono del 2026-09-11), coerente col testo; ma è una convenzione diversa da quella dei compiti 7, 8, 9, 10, 14, 15 (`<data>`) | NON RIPRODOTTO | prosa | no | Nessuno; al più «RICHIAMO DEL \<data\>, dal pre-controllo del 2026-09-11 (P-30)» per uniformità |
| R9a-19 | P-2 «Le versioni del 2026-09-11» — rimisurate oggi al registro | Tutte le appuntate come misurate l'11; `redb` «4.2.0 al registro» | `npm view <pkg>@<versione> version` per 18 pacchetti → ogni appuntata è **servita**; `latest` invariato per 17; `vitest` latest **5.0.1** (era 5.0.0) e `dist-tags.V4` **4.1.11** invariato → D4 regge; crates.io: `interprocess` max **2.4.4** ✓; `redb` max **4.3.0** (P-2 scriveva 4.2.0) → D6 regge (resta la 4.1.0 del lockfile), la cifra di P-2 è invecchiata | NON RIPRODOTTO | fatto | no | Nulla: le versioni si rilanciano il giorno dell'esecuzione (vincolo globale 8) |

Conteggio: `grep -c '| CONFERMATO |' R9a-report.md` → il numero sta nella risposta finale, derivato dal comando.

## Copertura del disegno per il mio perimetro

Legenda esito: **coperta** · **scoperta** · **contraddetta** (con la `D`/`P` che lo dichiara e se il disegno riceve il richiamo) · **spostata**.

### Testa del disegno e «Le risposte del proprietario»

| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|
| Testa · «ciò che questo disegno vi aggiunge lo dichiara come richiamo datato, compito del piano: la settima porta, sei → sette famiglie» | Compito 4, *Files* + vincolo globale 1: riga 209, §2.3, §3.1 della spec (P-22), `ports/mod.rs` (P-21) | coperta |
| Testa · «Le sezioni sono approvate A CONDIZIONE … ci si ferma e lo si dice» | Vincolo globale 14 del piano | coperta |
| Testa · «Metodo: ogni affermazione porta la sua specie; i comandi si rilanciano» | Vincolo globale 5; testa «Come si esegue», punto 2 | coperta |
| Risposta 1 · chat senza modello vero, flusso dal core finto | Compito 12 (rubinetto `Token`), compito 14 (Chat) | coperta |
| Risposta 2 · stato = degrado, `Refused`/`Queued` distinti, policy e budget, riga G16 | Compito 3 (`DegradationReport`, `PolicyReport`, `Protection`), 14 (`Status.vue`: `lastVerdict`, `status.protectionValue.AsSystemAccount`) | coperta |
| Risposta 3 · ordine: spike, poi trasporto, poi SPA | Parte 1 eseguita; compiti 2 → 13/14 | coperta |
| Risposta 4 · chi decodifica `bincode` lato GUI lo decide la misura (Q1) | D36: nessuno decodifica in questo piano; [C] col guscio come chiusore | contraddetta, dichiarata (D36); il disegno non riceve il richiamo → **R9a-7** |
| Risposta 5 · prima funzione del registro: `Arbiter::set_policy` | Compito 7, `POLICY_FUNCTION` (`tool: "registry"`, `Idempotent`) | coperta |
| Risposta 6 · `gui/` alla radice fuori dal workspace, passo nuovo del cancello | Compiti 11 (D44 al 12: `exclude`), 15 | coperta |
| Risposta 7 / 7-bis · primitive senza stile, Reka UI | Compito 13 (`reka-ui` 2.10.4, `Dialog*` per il cassetto), 14 (`DialogDescription`); D62 (radio nativi in Impostazioni) | coperta; la deroga di D62 è dichiarata |
| Risposta 8 / 8-bis · `vue-i18n`, italiano, tastiera e contrasto da subito | Compito 13 (`locales/it.json`, `i18n.ts`, `:focus-visible`), 14 (`contrast.test.ts`, D53), 15 (`no-raw-text` a `error`) | coperta |
| Risposta 9 · la GUI trova il core; se manca lo dichiara; avvio a mano | Compito 13, `Band.vue` + `useConnection` (D48, «riprova» rimanda `Hello`) | coperta |
| Risposta 10 · i byte del kernel diventano fixture; una variante nuova li rigenera | Compito 3, Passi 6–7 (D35), `ipc_wire.rs` | coperta |
| Risposta 11 · nessuna casella nel 2 | Compito 14, «Che cosa questo compito NON produce»: nessuna casella | coperta |
| Riga «strada» · il daemon «manda degrado, **verdetti** e policy» | Compito 7/9; D5 → il daemon non risponde a `Request`, nessun `Verdict` dal daemon nel 2 | contraddetta a metà, dichiarata (D5), senza richiamo → **R9a-10** |
| Riga «strada» · il finto è un programma Rust in `gui/`, schema vero, filo vero, token a tempo | Compito 12 | coperta |
| Riga «Linux» · niente Linux, innesco scritto in ADR-0029 | Parte 1; [C] «la metà Linux di M3 e M5» | coperta (fuori piano, dichiarata) |
| Riga «trasporto» · `interprocess` 2.4.4 in due passi | Compito 2, Passo 4 (`cargo build -p platform` fuori dal cancello, lockfile insieme); 2.4.4 è ancora `max_version` oggi | coperta |
| Riga «contatore» · tipo del kernel, uno solo, seminato dal giornale | Compito 1 (`numbering::Progressive`, `seeded_from`, D7, D8) | coperta |
| Riga «limite di giri» · `u64::MAX` in produzione | Compito 9 (D21, D28) | coperta |

### «Lo stato dell'arte verificato» e «Ciò che il codice diceva»

| Sezione · riga | Compito · Passo | Esito |
|---|---|---|
| «Le versioni si riverificano … il giorno del piano» | P-2 (2026-09-11), D4 rimisurata il 14; vincolo globale 8; rimisurate oggi (R9a-19) | coperta |
| `IpcMessage` ha due varianti oggi | `grep -c 'Request(GrantRequest)\|Verdict(Verdict)'` nel Passo 1 del 3 → oggi 2 ✓ | coperta, cifra coincide |
| `EXECUTOR_TURN_LIMIT` è `100_000` | `grep -n 'const EXECUTOR_TURN_LIMIT' crates/daemon/src/main.rs` → `100_000` oggi ✓; compito 9 | coperta |
| `IpcError` ha `Disconnected` e `MalformedMessage`; la suite «nasce col canale vero» | oggi ✓ (righe 213, 219 di `ports/ipc.rs`); compito 2 (D9 dà a `MalformedMessage` il suo unico produttore, P-14) | coperta |
| il contatore del giornale non esiste; `ClientId::new(u64)` pubblico | Compito 1, richiami in `ports/journal.rs` e `ports/ipc.rs` | coperta |
| `Reactor` solo tempo, nessuna prontezza I/O | §9 voce 5 → [C] (decisione 41) | coperta come voce aperta; la misura promessa manca → **R9a-5** |
| `framing`: `frame`, `unframe`, `LENGTH_WIDTH = 4` | oggi ✓ (`framing.rs:23`); compito 2 aggiunge `declared_len`/`take_frame` (P-13) | coperta |
| `Degradation` due campi | oggi ✓ (`degradation.rs:28,31`); compito 3 `DegradationReport` | coperta |
| `set_policy` scrive sul passo che riceve; `allocated()`; `VramPolicy::name()` | Compito 8 (specie `Policy`, `policy_now`), 7 (`PolicyReport { allocated, total }`, D20) | coperta |
| `grant`/`is_granted`, nessun confine di sessione | Compito 6 (D24 `Approval`), [C] voce 7 | coperta |
| `ClientGrants::on_disconnect` esiste | Compito 7 `forget` → `on_disconnect`; 9 Passo 13; 10 D33 | coperta |
| `gate.sh` aggiunge un passo con `run`; la CI senza Node; `.gitignore` | Compiti 15, 16 (P-8: oggi le righe `run` sono 6 + il settimo passo, riga 84) | coperta |

### §1 — Il perimetro

| Riga | Compito · Passo | Esito |
|---|---|---|
| i nove pezzi, in ordine (§3 della stella polare) | Tabella della posizione: 1–2 filo, 3 schema, 4–5 settima porta, 6 registro, 7–9 daemon, 10 DST, 11–14 SPA, 12 finto, 15 cancello; l'ordine «lo fissa il piano» (🔶 dedotto) | coperta; ordine spostato (settima porta prima del registro, finto prima della SPA) e dichiarato da D1 |
| «ciò che il 2 non costruisce»: casella, rete OpenRouter, segreti, run/passo, nucleo/ricerca, viewer 3D, cornice nativa/menu/pacchetto/avvio, mano, guide, Compatta vera, kit | Nessun compito li costruisce: casella (14 lo dice), ricerca (13: `<input disabled>` con «arriva col sotto-progetto 6»), guscio (P-53), kit (13: «nessun kit», D62); «OpenRouter» compare solo come etichetta della policy remota (3, 14) | coperta |
| l'ordine di costruzione «lo fissa il piano» | D1 | coperta |
| i debiti del 2: `Token` senza produttore, la Chat che lo dice a parole, **un** segnaposto | 12 (rubinetto), 14 (Chat: «nessuna run»), 13 (`Placeholder.vue`, D47) | coperta |
| Nomi: `custody`, `Custody`, `keep(key, bytes)`, `retrieve(key)`, `CustodyKey::Layout` | Compito 4, *Interfaces* (nomi identici) | coperta |
| Nomi: modulo `registry` | Compito 6, `kernel::registry` | coperta |
| Nomi: i messaggi della §4 fissati, la lista `Steps` | Compito 3: `Steps(Vec<StepSummary>)` e le altre 13 (sotto) | coperta |

### §2 — Il guscio (chiusa dalla parte 1)

| Riga | Compito · Passo | Esito |
|---|---|---|
| SP-8 eseguito, Electron, `dockview` resta | Nessun compito la rifà; il 13 legge lo spike e ne fa salire il **merito** (D49), non il codice | coperta |
| «`dockview-core` 8.2.0 non spedisce il foglio di stile — `dockview` entra per il solo CSS» | Compito 13: due pacchetti a 8.3.1 (D2), criterio `grep -c "dockview/dist/styles/dockview.css" gui/src/main.ts`, `from "dockview"` → 0 | coperta |
| «la SPA parla `bincode` sull'`ipc` del kernel, non questo JSON» | D36: la SPA non decodifica; il guscio sì | contraddetta, dichiarata, senza richiamo → **R9a-7** |
| Q3 «finestra staccata» rifiutata in entrambi i gusci | D58, [C] «finestra a parte» | coperta |

### §3 — Il filo

| Riga | Compito · Passo | Esito |
|---|---|---|
| il trasporto: `crates/platform/src/ipc.rs`, `Ipc` su `interprocess` 2.4.4, listener non bloccante, tabella `Vec`, niente `HashMap` | Compito 2, Passo 7: `ListenerNonblockingMode::Both`, «A `Vec` AND NOT A `HashMap`» | coperta |
| `accept`: nessuno → `None`; numero dal contatore condiviso | Compito 2: `with_nobody_connected_accept_answers_none`, `two_accepts_hand_out_different_and_ascending_numbers`; `bound(name, numbers, max_body)` | coperta |
| `send`: «incornicia con `kernel::framing::frame` e scrive» | D10: **non** incornicia, verbatim | contraddetta, dichiarata (D10), **senza richiamo** → **R9a-3** |
| `receive`: parziale → `Ok(None)`; «cornice intera → `Ok(Some(corpo))`»; «cornice rotta → `MalformedMessage` e il client resta»; fine → `Disconnected` | Compito 2: `half_a_frame_is_not_a_body_yet_and_not_an_error`, `a_whole_frame_comes_back_whole_envelope_included` (busta compresa, D10), `a_body_over_the_cap_is_malformed_and_the_client_stays` (D9: tetto, flusso avvelenato), `a_peer_that_goes_away_is_disconnected_and_leaves_the_table` | contraddetta in due punti, dichiarata (D9, D10), **senza richiamo** → **R9a-3** |
| il contatore: tipo nuovo in `kernel`, il daemon lo costruisce da `replay()` e lo consegna; richiami in `ports/journal.rs` e `ports/ipc.rs` | Compito 1 (`seeded_from`, D7; i due richiami); compito 9 riga 390 `numbering::seeded_from(&journal)` → `LocalSocketIpc::bound` | coperta |
| la prova del contatore: riavvio sopra l'ultimo scritto; due `accept` crescenti | Compito 1 `a_reopened_journal_never_hands_back_a_number_it_already_holds`; compito 2 `two_accepts_hand_out_different_and_ascending_numbers` | coperta |
| la stretta di mano: `Hello` col timbro; accoglie o «segna il client come rifiutato e non lo ascolta più» (decisione 22); la GUI col timbro sbagliato non parte | Compito 7: `the_welcome_is_the_five_messages_of_sequence_one`, `a_stale_stamp_gets_the_expected_one_and_then_the_core_stops_listening`; compito 13 `phase: "stale"`, `Band.vue` | coperta |
| la suite di conformità: `ipc_contract.rs` inclusa da `ipc_contract_real.rs`; «gira su `FakeGui`, `DyingGui` e il trasporto vero» | Compito 2 Passi 5–6: macro espansa **solo** in `platform`; kernel «zero test — è voluto» | contraddetta sulle finte, **non dichiarata**, senza richiamo → **R9a-4** |
| la dipendenza in due passi | Compito 2 Passo 4 (`cargo build -p platform`, `git status --porcelain Cargo.lock` → modificato, commit insieme) | coperta |
| «Perché il contatore è A e non B» | D7 (seminatura in `kernel`), [C] voce 6 (decisione 42) | coperta |
| «Ciò che la §3 non fa»: nessuna prontezza I/O | [C] voce 5 | coperta (misura promessa mancante → **R9a-5**) |
| 🔶 dedotto: flussi non bloccanti «si prova al primo test rosso» | Compito 2, Passo 2: verificata nel sorgente 2.4.4 (`ListenerNonblockingMode`, `Stream::set_nonblocking`) | coperta nel merito; la riga non riceve il richiamo «misurata» → in **R9a-3** |
| 🔶 dedotto: una nota dopo l'esito | Non esercitata dal piano (D24 posa `grant` **dentro** A, prima dell'esito) | coperta per assenza |

### §4 — Lo schema

| Riga | Compito · Passo | Esito |
|---|---|---|
| «un solo enum per le due direzioni; ogni variante dichiara nel doc chi la manda» | Compito 3, righe 241–270 del modello: `gui -> core` / `core -> gui` su tutte e 14 | coperta |
| `Hello` GUI, timbro | `Hello(BuildStamp)`; 13 `stamp.ts` (D52) | coperta |
| `Accepted` core, protezione «come valore» | `Accepted(Protection)`; D22 (mandato, non consegnato — richiamo Passo 8 del 7); 14 `protectionValue.AsSystemAccount` | coperta; D22 richiamata |
| `StaleBuild` core, timbro atteso «poi il core chiude» | `StaleBuild(BuildStamp)`; compito 7 (decisione 22) | coperta |
| `Degradation` core, due campi | `Degradation(DegradationReport)`; D23 (a ogni giro, solo se differisce) | coperta |
| `Policy` core: policy attiva, budget allocato e totale | `Policy(PolicyReport)` con `allocated`/`total` (D20, P-38) | coperta; il richiamo di D20 va alla stella polare (riga 2 *Stato*) |
| `Invoke` GUI: funzione e argomento | `Invoke(Call)` | coperta |
| `PermissionRequired` core: la tripla, a parole nella GUI | `PermissionRequired(Triple)`; 14 `Confirm.vue` (D60) | coperta |
| `Approve` GUI: tripla **e** invocazione (decisione 21) | `Approve { triple: Triple, call: Call }`; 14 `stores/invoke.ts` (D59) | coperta |
| `Token` core: testo + provenienza, enum gemello di `Trust` | `Token { text, provenance: Provenance }`; 12 rubinetto (`the_tokens_arrive_untrusted`) | coperta |
| `Layout` core: pacchetto / niente / non disponibile (decisione 35) | `Layout(LayoutState)` tre varianti; 7 (`Nothing`, `Package`), 9 (`Unavailable`, D30) | coperta (la sonda del 9 → **R9a-2**) |
| `SaveLayout` GUI: pacchetto opaco, fuori dal registro | `SaveLayout(Vec<u8>)`; 7 `keep_layout` senza tripla; 13 `onDidLayoutChange` + `beforeunload` | coperta |
| `Steps` core: le invocazioni con intento ed esito; all'accoglienza e dopo ogni invocazione | `Steps(Vec<StepSummary>)`; 7 (`heard.get(4)`, `heard.get(6)`); D56 (tre campi) | coperta |
| `Request`, `Verdict` «come oggi … il daemon risponde» | D5: il daemon **non** risponde nel 2 | contraddetta, dichiarata (D5), riga della §4 senza richiamo → **R9a-10** |
| «Il core decide quando emettere»: accoglienza `Degradation`+`Policy`+`Layout`+`Steps`; poi il pezzo cambiato; `Layout` dopo `SaveLayout`; `Steps` dopo ogni invocazione | Compito 7 (`the_welcome…five_messages`, `save_layout_comes_back…`, `an_approve_changes_the_policy_and_the_gui_is_told`, `an_unchanged_degradation_is_not_resent`) | coperta |
| le fixture: comando dichiarato, byte + mappa `indice → nome → valore` in `gui/`, committate, rigenerabili | Compito 3 Passi 6–7 (D12, D35: `.bin` + `.json` + `ipc_v1.map`); criterio `ls gui/schema/fixtures/*.bin \| wc -l` → **14** | coperta; cifra coincide con 12 + 2 varianti |
| il controllo: `ipc_wire.rs` ricodifica le fixture → rosso senza rigenerare | Compito 3 (`crates/kernel/tests/ipc_wire.rs`) | coperta |
| il timbro: impronta dell'insieme, ricalcolata dal core, scritta nella mappa; poche righe, **nessuna dipendenza nuova**; non ADR-0018 | Compito 3: `stamp_set()`, `build_stamp()`, doc «IT IS NOT ADR-0018's FINGERPRINT», nessun manifesto nei *Files* | coperta |
| Debiti: `Token` senza produttore; `Request`/`Verdict` senza mittente; impronta identità | 12; D5; 3 | coperta |

### §5 — Il registro, e il daemon che ascolta

| Riga | Compito · Passo | Esito |
|---|---|---|
| dove: modulo `registry`, `no_std`, meccanismo dentro e contenuto fuori | Compito 6 (D16: `grep -cE 'Arbiter\|VramPolicy\|arbiter' registry.rs` → 0) | coperta |
| una funzione registrata: nome `&'static str`, tripla, `EffectClass`; una sola, «registro × arbitro × scrittura», `Idempotent` | Compito 6 `Function`; compito 7 `POLICY_FUNCTION` (`tool: "registry"`, `EffectClass::Idempotent`) | coperta |
| «un nome non registrato → rifiutato, nessun record» | Compito 6 `InvokeError::NotRegistered` e sonda | coperta |
| `invoke`: nome, invocatore, argomento; `is_granted`; no → `PermissionRequired`; sì → l'effetto | Compito 6 (D16: effetto come chiusura; D24 `Approval`); 7 `an_invoke_without_the_permission_asks_for_the_triple_and_writes_nothing`, `an_approve_changes_the_policy_and_the_gui_is_told` | coperta |
| il giornale: `intent` su A, nota `Invocation` (funzione, invocatore, argomento), l'effetto (passo B di `set_policy`), `outcome` su A; decisione 21 (`Permission` su A fra la nota e l'effetto) | Compito 6 (`RecordV1::invocation`, D18: l'argomento nel `payload` — richiamo P-30 al Passo 9; D24) | coperta; D18 richiamata nella cella |
| frozen bytes «sei → sette» | Compito 6 (`the_frozen_records()` sette; i sei `.cbor` identici); oggi `ls crates/kernel/tests/frozen/` → 6 `.cbor` ✓ | coperta, cifra coincide |
| «una sonda legge il dettaglio dopo `replay`» | Compito 6, riga 752 del modello | coperta |
| l'invocatore: enum a una variante col `ClientId`; il 12 aggiunge il gesto «senza cambio di formato (ADR-0036, regola 3)» | Compito 6 `Invoker::Gui(ClientId)`, D17 (`u8`); richiamo P-32 al Passo 9 («la regola citata è quella sbagliata») | coperta; richiamata |
| il permesso: `Approve` → `grant` → la GUI **non** rimanda `Invoke` (decisione 21) | Compito 6 D24 (`Approval::JustGiven`), 7 `approve`, 14 `useInvoke().approve()` | coperta |
| «Limite letto nel codice»: `is_granted` senza confine di sessione → il 3 | [C] voce 7 | coperta |
| l'attività: attività del kernel «così la DST la muove con `DyingGui`»; a ogni giro `accept`, `receive`, dispaccio, tick da `Parameters` | Compito 7 (`serve`, `gui_tick`); D32: **non** `DyingGui` → Passo 8 del 10 col richiamo (testo verificato, ancora assente → **R9a-14**) | contraddetta su `DyingGui`, dichiarata (D32) e richiamata dal 10 |
| la prova: campagna DST breve, due proprietà | Compito 10 (`serving_campaign.rs`: «gui death», «journal crash») | coperta |
| `Hello`: primo messaggio; timbro uguale → `Accepted` «con la protezione (valore che `platform` conosce, **consegnato**)», poi `Degradation`, `Policy`; diverso → `StaleBuild`; decisione 22; poi `Layout` e `Steps` | Compito 7; D22 (non consegnato) col richiamo al Passo 8 | contraddetta su «consegnato», dichiarata (D22) e richiamata |
| `Invoke` / `Approve` al registro; dopo `set_policy` rimanda `Policy` | Compito 7 (`heard.get(5)` `Policy` `Local`) | coperta |
| `SaveLayout`, `Steps`: `keep` sotto `Layout`, `retrieve`, `Layout` con ciò che tiene (il vecchio se fallita; «non disponibile» se non si apre); `Steps` da `replay` dopo ogni invocazione | Compito 7 (`keep_layout`, `layout()`, `step_list()`); 9 (D30) | coperta; la prova «scrittura rifiutata → il vecchio» manca → **R9a-9** |
| `Request`: «`admit` → `Verdict`»; `promote` non si chiama | D5: non servito; richiamo al Passo 8 del 7; [C] righe 27 e E50/E51/E100 | contraddetta, dichiarata (D5), **richiamata** |
| `Disconnected`: `on_disconnect`, il client esce dalla tabella; «una sonda sul cablaggio» | Compito 7 `forget` + `a_client_that_dies_gives_its_grant_back`; 9 Passo 13 (la metà che tiene, dichiarata) | coperta |
| il limite di giri: `u64::MAX`, richiamo sul doc di `EXECUTOR_TURN_LIMIT`; «il grafo con la GUI resta vivo oltre centomila giri» | Compito 9 Passo 10 (D28: limite finito + tick nullo + pari + baseline; `the_graph_with_the_gui_stays_alive_past_a_hundred_thousand_turns`); richiami: D21 al Passo 8 del 7, D28 al Passo 15(a) del 9 | coperta; spostata dal 7 al 9 con richiamo |
| lo spegnimento: nessuno nel 2 | Compito 9 Passo 15(b): richiamo (la guardia del limite finito non esiste più) | coperta, richiamata |
| Debiti: permesso oltre il riavvio (3); `promote` (invariato); watchdog (10) | [C] voci 7, E50, 8 | coperta |

### §6a — La GUI

| Riga | Compito · Passo | Esito |
|---|---|---|
| dove e con che cosa: `gui/`, Node in `package.json`, `vite`, Vue 3, `pinia`, Reka UI, `vue-i18n`, TypeScript, `dockview-core` diretto; lockfile committato, `npm ci` | 11 (D37, D40), 13 (`pinia`, `reka-ui`, `vue-i18n`, `dockview-core`+`dockview`), 15 (`npm ci`) | coperta |
| il ponte: la SPA non tocca socket; riceve decodificati; manda **quattro**: `Hello`, `Invoke`, `Approve`, `SaveLayout`; finta che rilegge le fixture | 11: `OutboundMessage = Extract<IpcMessage, { kind: "Hello" \| "Invoke" \| "Approve" \| "SaveLayout" }>`, `createFakeBridge` | coperta, cifra coincide |
| gli strati: `transport/`, `schema/` (byte **e** JSON), `stores/` (connessione, core, flusso, disposizione), `components/`, `panels/` (tipi registrati uno per riga della §1, un segnaposto; tre viste JSON), `tokens/` (un file), `locales/it.json` | 11 (`transport/`, `schema/`), 13 (`stores/connection.ts`, `core.ts`, `layout.ts`; `panels/registry.ts` con **18** voci = 5 tabelle piene + 13 della corta; `views/*.json`; `tokens.css`; `it.json`), 14 (`stores/stream.ts`, `components/`) | coperta, cifra coincide (`grep -c '^  { name: "'` → 18; corta → 13, misurato oggi) |
| la cornice: barra delle viste, ricerca (dice chi la riempie, il 6), chip del core, fascia, striscia con solo ciò che è vivo, cassetto «+ moduli» col numero; `Layout` all'accoglienza; niente/non disponibile → viste committate; `SaveLayout` quando si ferma e alla chiusura; tipo sparito → lo dice e si chiude | 13: `ViewBar.vue` (`bar.search` disabilitata con `searchHint`, `bar.core` + `coreWaiting/coreConnected/coreStale`), `Band.vue`, `Strip.vue` (degrado, permessi), `Drawer.vue` (`who`), `apply(api, view, pack \| null)`, `onDidLayoutChange` + `beforeunload`, `placeholderParams` («tipo sparito»), D50 | coperta |
| i quattro stati della connessione | 13 `phase: waiting \| connected \| stale` (D48, derivati, senza soglia); «nessuna run» → 14 Chat | coperta (D48 dichiara) |
| il modulo Stato: due campi, policy col budget, riga G16 dal valore, riga di evento solo con un `Verdict` | 14 `Status.vue` (`core.lastVerdict !== null`, `status.protectionValue`) | coperta |
| il cambio di policy in Impostazioni: controllo a due stati → `Invoke`; `PermissionRequired` → finestra → `Approve` con l'invocazione | 14 `Settings.vue` (D62 radio nativi), `stores/invoke.ts` (D59), `Confirm.vue` (D60), `functions.ts` (D55); P-85 | coperta |
| il modulo Chat: scheda di Lavoro, un esemplare per run, non in Home; markdown con codice; provenienza; testo e codice mai HTML; nessun link da solo | 13 `work.json` (`tile(work, "chat")`), 14 `Chat.vue`, `markdown.ts` (D54: link `<span>`, immagini testo), `stream.ts` (D61) | coperta |
| Passi e Permessi | 14 `Steps.vue` (D56), `Permissions.vue` | coperta |
| accessibilità: tastiera dai primitivi Reka; focus visibile; AA dai token; regione del flusso «con moderazione»; trappola di focus; spostare la tessera con scorciatoie sopra `moveTo` | 13 `:focus-visible`; 14 `contrast.test.ts` (D53), `aria-live="polite"` sui blocchi congelati, `Confirm.vue` con `Dialog*` di Reka, `moveActive.ts` + `keys.test.ts` (P-97), `a11y.test.ts`; D62 | coperta |
| testi (G21): tutto in `it.json`, `no-raw-text` nel cancello | 13 (rete `copy.test.ts`, D51), 15 (D65) | coperta |
| la regola del kit: nessun kit nel 2 | 13 («nessun kit») | coperta |
| Debiti: Chat senza produttore; il ponte lato guscio aperto per nome; un segnaposto solo | 14; [C] (guscio); 13 D47 | coperta |
| 🔶 dedotto: `fromJSON` carica le tre viste — la mossa 7 dello spike | Parte 1 (E4: confronto canonico → D49 `canonical`) | coperta |

### §7 — Il core finto

| Riga | Compito · Passo | Esito |
|---|---|---|
| dove: binario in `gui/fake-core/`, fuori dal workspace (`exclude`), per percorso da `kernel`, `platform`, `simulator`; `Cargo.lock` committato | 12 Passi 2, 4 (D44; vincolo 7) | coperta |
| il passo del cancello: `cargo test --locked --manifest-path gui/fake-core/Cargo.toml` | 15 `gate-gui.sh` prima riga | coperta |
| l'attività vera: la stessa del daemon, costruita da fuori, `Executor` col limite del daemon; nessun ramo del dispaccio | 12: `run_the_graph(SOCKET_NAME, u64::MAX, GUI_TICK, …)`, criterio `grep -cE 'IpcMessage::(Hello\|Invoke\|Approve\|SaveLayout) *(\(\|=>)'` → 0 | coperta |
| la sonda da fuori: `Hello → Accepted → Degradation → Policy → Layout → Steps` | 12 `the_welcome_gives_the_sequence_one` | coperta |
| le porte in memoria: `MemoryJournal`, la finta della settima porta, arbitro vero «come lo costruisce il daemon», `SystemReactor` | 12 Passo 5 (`build_the_arbiter` **riscritto**, D41 — decisione del proprietario) | coperta; «senza copiarla» contraddetta e dichiarata, **il richiamo non è dettato** → **R9a-1** |
| il trasporto vero sullo **stesso nome** del daemon; timbro delle fixture | 12 (D45: due letterali confrontati nel criterio; `build_stamp()`) | coperta |
| la sonda contro il trasporto da un thread: timbro giusto → fila; sbagliato → `StaleBuild` e poi niente | 12 `a_peer_that_says`, `a_stale_stamp_is_refused_and_then_silence` | coperta |
| il rubinetto: seconda attività, `RefCell` condivisa; `Token` a tempo «2000 in dieci secondi», non fidati, markdown; `stdin`: `degrade`, `verdict`; niente altro | 12 `the_faucet` (D43: la stessa `RefCell<Core>`; cadenza «§7 asks for 2000 in ten seconds, which is this»; `"degrade"`, `"verdict"`; D42 thread + `try_recv`) | coperta, cifra coincide |
| una sonda per parola, cadenza rapida; i token contati con la provenienza | 12 `degrade_makes_the_real_code_report_it`, `verdict_reaches_the_peer`, `the_tokens_arrive_untrusted` (cadenza 0) | coperta |
| la disposizione: `SaveLayout` → finta → `Layout`; vive finché il finto non riparte; **la prova: il giro salva → ritrova sul pari** | 12: nessuna sonda con `SaveLayout` | scoperta, non dichiarata → **R9a-8** |
| la lista dei passi: `replay`; **la sonda dell'invocazione** | 12: nessuna sonda con `Invoke` + `Approve` → `Steps` | scoperta, non dichiarata → **R9a-8** |
| «Perché A e non B» e «Il costo di A» | D41, D42, D43 | coperta |
| «Ciò che la §7 non fa»: non persiste, non convive col daemon | 12 (`MemoryJournal`, `MemoryCustody`) | coperta |
| Debito «come se ne accorga … lo fissa il piano con la sonda della §8» | D23 (a ogni giro, solo se differisce; 7 `a_degradation_written_by_a_second_activity_reaches_the_gui`) | coperta |
| 🔶 dedotti: la `RefCell`; il valore di `Accepted` consegnato; la forma della `RefCell`; `build_the_arbiter` senza copiarla | D43/P-71 (chiuso dal contratto), D22, D43, D41 | coperta; D41 senza il Passo del richiamo → **R9a-1** |

### §8 — Le prove e il cancello

| Riga | Compito · Passo | Esito |
|---|---|---|
| `scripts/gate-gui.sh`: `cd` alla radice; fake-core, `npm ci`, `npm run build`, `npm test`; si ferma al primo rosso | 15 Passo 7 (`set -euo pipefail`; «LINT LAST, AND THE ORDER IS §8's»); 16 aggiunge `npm audit` in coda | coperta |
| le due direzioni: test reso rosso → `GATE RED`; l'etichetta nell'uscita | 15 (`EXIT=0`, poi «diverso da zero due volte»; `grep -c 'gui: fake core and SPA' /tmp/gate-15.log`) | coperta |
| la riga in `gate.sh` fra «attributes» e «documentation consistency» | 15 (P-8: oggi righe 43–44 di `gate.sh`) | coperta |
| il core finto nel cancello: lockfile, sonde in `main.rs`, «tempo dichiarato e misurato al piano» | 12 (D44, decisione 48), 15 Passo 10 (`<tempo>` ×3 nel commento di `gate.sh`, criterio → 0 segnaposto) | coperta |
| la versione di Node: `engines.node`, `.npmrc` `engine-strict=true`; «nelle due direzioni» | 11 (D37: la riga di `jsdom`; P-64 misurata nelle tre direzioni) | coperta |
| la CI: `actions/setup-node` con `node-version-file: gui/package.json`, `v7`, niente cache | 15 Passo 8 (`setup-node@v7`, `node-version-file`, `package-manager-cache: false` — D66 rende esplicita la 47); `checkout` resta v4 (16) | coperta |
| `.gitignore`: le tre righe di `gui/`; `spikes/gui-shell/` «coi nomi al piano»; i lockfile si committano | 11 (D38), 12, D67 (le righe dello spike esistono da `8fc9696`) | coperta |
| la campagna DST nel settimo passo, nelle due direzioni | 10 Passo 7 (`serving_campaign`, `grep -c 'DST …'`) | coperta |
| il trasporto `ipc` in `platform` (§3) | 2 | coperta salvo le finte → **R9a-4** |
| il contatore condiviso (§3) | 1, 2 | coperta |
| la stretta di mano (§3, decisione 22): `Disconnected` quando la GUI esce | 7 | coperta |
| lo schema (§4): fixture ricodificate, timbro, byte consumati | 3 | coperta |
| il registro (§5): le tre sonde, il giro su `FakeGui`, i byte congelati «quanti lo dice `ls`», il dettaglio dopo `replay` | 6, 7 | coperta |
| l'attività del daemon (§5): la campagna, due proprietà, e la riga nel settimo passo | 10 | coperta |
| il limite di giri e `Disconnected` (§5): «oltre centomila giri»; «la GUI che muore con una concessione ordinaria → `on_disconnect` … più una sonda sul cablaggio» | 9 Passi 10, 13 (D28; la metà dichiarata nel codice) | coperta; la riga non riceve il richiamo di D5 → **R9a-10** |
| la settima porta: sette prove | 4 (finta), 5 (suite, `redb` apri/scrivi/riapri/rileggi, byte non JSON, archivio vuoto), 7 (scrittura rifiutata → **manca**), 9 (non si apre → corpo `{ /* … */ }`) — P-28 | due delle sette scoperte → **R9a-9**, **R9a-2** |
| «salva, riavvia, ritrova» | 9 Passo 11 `a_layout_saved_is_found_again_after_a_restart` | coperta |
| il core finto (§7) | 12 | coperta salvo le due sonde → **R9a-8** |
| la SPA, `schema/`: «ogni variante **decodificata dai byte** … una variante senza fixture → rosso» | 11 `schema.test.ts` su `MESSAGE_KINDS` contro i `.json` (D36) | contraddetta sui byte, dichiarata (D36), senza richiamo → **R9a-7** |
| la SPA, `stores/`, componenti e pannelli: i quattro stati; Stato; la finestra con la trappola; la chat mai HTML; le tre viste; tipo sparito; modulo non costruito; accessibilità con `axe` | 13 (`stores.test.ts`, `frame.test.ts`, `views.test.ts`, `Placeholder.vue`), 14 (`modules.test.ts`, `chat.test.ts`, `markdown.test.ts`, `a11y.test.ts`, `contrast.test.ts`) | coperta |
| le scritte: «**se** esiste una regola matura; altrimenti revisione, e lo si dice» | 13 (D51 rete), 15 (D65 `error`) | coperta; il richiamo chiesto dalla §9 voce 3 manca → **R9a-12** |
| la build della SPA: `npm run build`, `npm ci` gemello di `--locked` | 15 | coperta |
| capo a capo nel guscio: «dopo lo spike, parte 2 del piano» / «fuori dal cancello di oggi» | P-2, P-53, D31, [C] | spostata al guscio, dichiarata → **R9a-17** |
| lo spike del guscio | Parte 1 | coperta |
| le dipendenze nuove: Rust in due passi, `--locked`, `gate-deps.sh`; web `npm ci` | 2, 9, 12 (lockfile insieme), 15 | coperta |
| il registro della porta: una sezione, nessuna riga di catalogo | 17 (D73) | coperta |
| i documenti: §3.1 e `ports/mod.rs` sei → sette, roadmap, `README.md`, §12, tracciabilità, richiami | 4 (P-22, P-21), 17 (D72, P-113, P-116) | coperta |
| il codice fuori dal perimetro: «`git diff --stat` a fine piano tocca solo ciò che le tabelle nominano» | 17 Passo 9: nessun diff dell'intero piano | scoperta → **R9a-16** |
| decisione 35 (l'archivio che non si apre): il core parte, `Layout` «non disponibile», ogni `SaveLayout` idem; la sonda | 9 (D30 `MaybeCustody`), 7 (`Unavailable` → `LayoutState::Unavailable`) | coperta nel meccanismo; la sonda ha il corpo vuoto → **R9a-2** |
| decisione 34 (cancello unico) | 15 («Il cancello resta UNO») | coperta |
| «Ciò che la §8 non fa»: CI solo Linux (X-1), capo a capo, lint se immaturo, X-3, il tempo del cancello | 16 (X-1, X-3), [C] (guscio), 15 (lint c'è; `<tempo>` nel commento con la data) | coperta; il capoverso non riceve il richiamo → **R9a-11** |
| decisioni 46–50 del coordinatore | 11 (46), 15 (47 → D66), 12 (48), 11/12 (49), 3/7 (50) | coperta |
| Debiti: metà Windows senza CI; capo a capo; lint forse assente; doppia compilazione; sonde senza catalogo | 16; [C]; 15; P-104; 17 | coperta |
| 🔶 dedotti: `npm ci` onora `engine-strict`; le prove senza browser in CI; `--manifest-path` nel `target/` del finto | 15 Passo 11 (P-64 al 11; Passo 3 del 13; P-104), ancora «*e non riusi quello del workspace.*» unica (1), `grep -n 'manifest-path. compili nel'` → una riga (474) | coperta; il titolo e la chiusa di P-104 sbagliano sezione e compito → **R9a-15** |

### §9 — Le decisioni aperte col chiusore

| Voce | Chiusore | Compito · `D` · [C] | Esito |
|---|---|---|---|
| 1 renderer markdown | il piano, compito della SPA | 14 (D3 rilettura nel `.tgz` della 15.0.2; D54 due regole nostre; `validateLink`) | coperta |
| 2 attrezzi di prova: `vitest` 4.1.11, `jsdom`, `@vue/test-utils` 2.5.0, `axe-core` diretto, Playwright dopo il guscio, `vue-tsc` 3.3.11 | il piano | 11 (D4 rimisurata: `V4` 4.1.11 ancora oggi; `vue-tsc`), 13 (`jsdom`, `@vue/test-utils`), 14 (`axe-core`); Playwright **non** installato (P-2, dichiarato) | coperta |
| 3 `no-raw-text` in `gate-gui.sh`, «richiamo alla riga delle scritte della §8» | il piano | 15 (D65 a `error`); il richiamo manca | coperta a metà → **R9a-12** |
| 4 `gui/shell/` se vince Tauri | la parte 2 | Electron ha vinto; nessun `gui/shell` nel piano (`grep -n 'gui/shell'` → 0) | coperta per decadenza |
| 5 prontezza I/O; «il piano riporta la CPU a riposo del daemon col tick, senza soglia» | il proprietario (decisione 41: A) | [C] (resta com'è); la misura **manca** | scoperta → **R9a-5** |
| 6 l'allocatore nella porta `journal` | il proprietario (decisione 42: A) | [C]; D7 | coperta |
| 7 confine di sessione | il 3 | [C] | coperta |
| 8 watchdog e spegnimento | il 10 | [C]; 9 Passo 15(b) | coperta |
| 9 la policy riletta all'avvio (decisione 56) | questa sezione, poi il piano | 8 (D25, D26, D27; Passo 12(b): richiamo `<data>` sulla riga 9, ancora «un record congelato in più \|» unica) | coperta, richiamata |
| 10 il commento falso di `crates/platform/src/journal.rs` | il piano del 2, il primo compito che tocca il file (decisione 18) | 5 tocca il file e non lo corregge; non in [C] | scoperta → **R9a-6** |
| 11 AUD-004 | il proprietario (decisione 43) | [C] | coperta |
| 12 il ledger `.superpowers/sdd/` | il proprietario, un comando | fuori piano | coperta (non è del piano) |
| 13 X-1 e X-3 (decisioni 44, 45) | il proprietario → «compito del piano del 2» | 16 (D68–D71) | coperta |
| 14 cache npm | il piano | 15 (D66) | coperta |
| 15 titolo della riga 2 della roadmap; cifre in prosa di `ports/mod.rs` | il piano | 17 Passo 5; 4 (P-5, P-21) | coperta |
| 16 nomi dei messaggi e di `Steps` | il disegno | 3 | coperta |
| 17 Compatta | il 10 | 13 porta la vista `compact.json` (la §2 della stella), non la Compatta vera | coperta |
| 18 preset «auto-approva sicuri» | il 4 | fuori piano | coperta |
| il comando npm «il piano lo rilancia» | — | P-2, D4; rilanciato oggi (R9a-19) | coperta |

### §10 — Come si riprende

| Riga | Compito · Passo | Esito |
|---|---|---|
| punti 1–2: fetch, lettura obbligatoria, «la stella polare per intero e questo file per intero» | Testa del piano, «Come si esegue», punto 1: «il compito — tutto e nient'altro — e i disegni nelle sezioni che il compito nomina» | spostata (dichiarata: chi esegue legge le sezioni nominate, non i disegni interi) |
| punti 3–6: rilettura, voci aperte prima di scrivere, parte 1, pre-controllo | Fatti (parte 1 eseguita; [C]; sezione «Il pre-controllo», 116 `P`) | coperta |
| punto 7: esecuzione in sessione nuova, subagente fresco per compito, revisione fra uno e l'altro; X-1/X-3 al proprietario | «Come si esegue» punti 5–7; `CLAUDE.md`; 16 | coperta |
| punto 8: la parte 2 «con ciò che dipende dal guscio deciso coi numeri»; §12, `README.md`, roadmap (titolo riga 2), tracciabilità «entrano con essa» | 17 Passi 2–6; il guscio → [C] | coperta |
| punto 9: «i richiami datati nella §1 della stella polare per i moduli che il 2 ha costruito» | Nessun compito (14 scrive solo D56 sulla riga 1 di *Passi*; 17 non tocca la stella polare) | scoperta → **R9a-13** |
| «Ciò che i due disegni consegnano»: pezzi, forme, tabella §8, nomi, §9, sequenze, fonti, trappole, Definizione di «fatto» | D1 (taglio sulla §8), 17 D74 (la Definizione della parte 2, coi comandi) | coperta |
| Definizione di «fatto» della parte 1, riga 7: «la parte 2 scritta **dopo** la misura, con lo stesso pre-controllo» | Il piano (116 `P`, undicesima chiusura) | coperta |

### «Decisioni prese dal coordinatore» (2026-09-06) e «Vicoli ciechi e trappole»

| Riga | Compito · Passo | Esito |
|---|---|---|
| 1 Reka UI · 2 le due varianti · 3 contatore A · 4 limite A · 7 senza `Co-Authored-By` | 13; 3; 1; 9; vincolo globale 13 | coperta |
| 5, 6, 8 (punto fermo, misure nel file, wireframe) | storiche | non pertinenti al piano |
| trappola: stato di ADR-0029 con `grep -nE '^(Status\|Stato)'` | ADR chiuso dalla parte 1 | non pertinente |
| trappola: contare la quaterna dei test con un `awk` sul log del cancello (conta due volte i bersagli del passo 7) | Nessun compito conta i test sul log (`grep -n 'test result' compito-*.md` → solo il 1, su un banco); il 10 usa `grep -c 'DST …'` | evitata |
| trappola: sostituzioni in uno script Python con `newline=""`, agganci asseriti, fine-riga conservati | Vincolo globale 4; `replace_unique.py`; i Passi 15 del 14 e 11 del 15 asseriscono l'ancora; i Passi 8 del 10, 8 del 7, 9 del 6, 15 del 9 **no** | evitata a metà → **R9a-14** |
| trappola: `grep -n 'provvisori'` prima di riscrivere | I nomi sono fissati nella §1 | non pertinente |

## Voci P rimisurate

| P | Comando rilanciato → resa | Regge? |
|---|---|---|
| P-2 | `npm view <pkg>@<ver> version` ×18 → tutte servite; `npm view vitest dist-tags --json` → `latest 5.0.1`, `V4 4.1.11`; crates.io `interprocess` max 2.4.4, `redb` max 4.3.0 | regge (D4, D6 invariate); la cifra «`redb` 4.2.0» di P-2 è invecchiata |
| P-4 | `ls crates/kernel/tests/frozen/` → 6 `.cbor` + la mappa | regge |
| P-8 | `grep -n 'run "' scripts/gate.sh` → 6 `run` (39–44) + il settimo passo (84); «attributes» 43, «documentation consistency» 44 | regge |
| P-10 | `gui/` non esiste; `grep -n 'const EXECUTOR_TURN_LIMIT'` → `100_000`; `LENGTH_WIDTH = 4`; `IpcError` due varianti; `Degradation` due campi | regge |
| P-22 | vincolo globale 1: tre posti (209, 575, 906) — non riletti nella spec (fuori perimetro), il compito 4 li nomina | non rimisurata |
| P-28 | la tabella delle sette prove: la sesta al 7 **non** è dettata (R9a-9), la settima al 9 ha il corpo vuoto (R9a-2) | regge come censimento, non come esecuzione |
| P-47 | `git ls-files --eol <disegno2>` → `i/lf w/lf` | regge |
| P-53 | `grep -n 'gui/shell' <piano>` → 0; il guscio fuori dal piano | regge |
| P-56 | i cinque accessori nel blocco *Interfaces* del 7; `Core::custody`, `Core::grants` nel modello (righe 1227, ~1231) | regge |
| P-62 / D35 | §4 «mappa», §6a e §8 «JSON»: il 3 scrive `.bin` + `.json`; il 11 confronta col `.json` | regge |
| P-68 / D41 | `grep -c -F 'senza copiarla' <disegno2>` → 1; il 12 riscrive `build_the_arbiter` | regge; il richiamo non è dettato (R9a-1) |
| P-70, P-71 | il 7 porta D23 e `serve(core: &RefCell<Core>…)`; il 12 riusa la stessa `RefCell` (D43) | reggono; il disegno non riceve richiamo (i dedotti della §7 restano «da confermare») |
| P-84 / D52 | `gui/src/schema/stamp.ts` al 13, dall'ultima riga di `ipc_v1.map` (`tail -3` nel 3) | regge |
| P-85 | `Settings.vue` al 14, `functions.ts` (D55) | regge |
| P-89 / D56 | ancora del Passo 15 del 14 nella stella polare: conteggio **1** oggi; decisa A il 2026-09-15 | regge |
| P-102 / D67 | non rilanciato `git show 8fc9696` (fuori perimetro); il 15 non tocca `.gitignore` | non rimisurata |
| P-104 | `grep -n 'manifest-path. compili nel' <disegno2>` → riga 474, **§8**; la chiusa dice «il 17» ma è il Passo 11 del 15 | regge nel merito; titolo e chiusa da correggere (R9a-15) |
| P-113 | la roadmap non è nel mio perimetro; l'undicesima chiusura la registra corretta in `850137c` | non rimisurata |

## Numeri di compito ricensiti

| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|
| Undicesima chiusura, tabella «La riga / Chi la corregge»: «il Passo 8 del compito 10» | 10, Passo 8 | 10, Passo 8 (`grep -n 'Passo 8' compito-10.md` → «il richiamo datato nella §5») | giusto |
| idem: «il compito 12, richiamo di D41» | 12 | 12 (il core finto) — ma il Passo non esiste | giusto il numero, manca il Passo (R9a-1) |
| idem: «il Passo 11 del compito 15» | 15, Passo 11 | 15, Passo 11 («il richiamo datato sulla riga dei dedotti della §8») | giusto |
| P-104, chiusa: «lo scrive il 17, con le altre case» | 17 | 15 (Passo 11); il 17 non ha il disegno del 2 nei *Files* | **stantio** (R9a-15) |
| Testa, «Come si esegue» punto 5: «per i compiti 13 e 14 apre la SPA nel browser» | 13 e 14 | 13 e 14 (richiamo D25 già scritto) | giusto |
| D3: «il compito 13 rilegge … → RICHIAMO: la rilettura è del compito 14» | 14 | 14 (`markdown-it` al 14, D40) | giusto |
| D21 / Passo 8 del 7: «È il compito 9» per il limite di giri | 9 | 9 | giusto |
| P-28: «`SaveLayout` su un archivio che rifiuta → 7»; «archivio che non si apre → 9» | 7, 9 | 7, 9 | giusti (ma non dettate: R9a-9, R9a-2) |
| §9 voce 10 «il primo compito che tocca `crates/platform/src/journal.rs`» | — | 5 (`grep -l` → solo il compito 5) | il numero è giusto, il compito non lo fa (R9a-6) |
| §1 «cornice nativa … → dopo M1–M5 e il 10»; §8 «capo a capo … parte 2 del piano» | 10 / parte 2 | il guscio non è di questo piano (P-53) | la §8 è la riga stantia (R9a-17) |
| I numeri della §9 e della §1 del disegno («il 3», «il 10», «il 4», «il 12», «il 13») | sotto-progetti | sotto-progetti, non compiti | nessuna collisione: il piano li cita sempre come «il 3», «il 10» con [C] |

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

- `wc -l/-c skeleton.md p-titles.md <disegno2> <piano>` → 678 righe / 82 967 B; 116; 726; 20 129
- `git rev-parse HEAD` → `baf3cde…`; `git status --porcelain | wc -l` → 0 (apertura)
- `sed -n` a blocchi: skeleton 1–678 (5 chiamate), p-titles intero, `<disegno2>` 1–726 (6 chiamate ≤126 righe)
- `grep -n '^## \|^### [^P]' <piano>` → la mappa delle sezioni (17 `## Compito`, 11 chiusure)
- `awk '/^### L.undicesima chiusura/{f=1} /^### La decima chiusura/{exit} f' <piano>` → 128 righe, salvate in `probe-R9a/undicesima.md`
- `awk` per compito → `probe-R9a/compito-01..17.md`; `head.md` (1–168); `precontrollo.md`; `decisioni.md` (`grep -c '^| \*\*D[0-9]'` → 74 = 74 nel piano)
- `sed -n '155,190p' <piano>` → «Come si esegue» e l'errata (vuota)
- `grep -n '^| \*\*D[0-9]' <piano>` in tre blocchi → le 74 righe D lette
- `grep -n -i 'reka' <piano>` → `reka-ui` 2.10.4 al 13, `Dialog*`, D62; `grep -n -i 'playwright'` → P-2 riga 250 (non si installano); `grep -n -i 'senza soglia'` → solo P-78
- compito 2: `grep -n 'richiamo\|§3'`; `grep -n 'FakeGui\|DyingGui'` → 0; `grep -n -i 'hashmap\|nonblocking'`; `grep -n 'interprocess'`; `grep -nE '^\s*(pub )?(const )?(async )?fn '`; `sed -n '200,300p'`; Passo 9 (`riporta zero test`); `grep -n 'include!'` → un sito
- compito 1: `grep -nE 'fn '` → quattro sonde + `starting_at`, `take`, `seeded_from`; compito 9: `grep -n 'seeded_from'` → riga 390
- compito 3: `grep -n 'pub enum IpcMessage'`; `sed -n '240,340p' | grep -vE '///' | grep -nE '^\s*[A-Z]…'` → 14 varianti; `grep -n -i 'gui -> core\|core -> gui…'` → 14 doc; `grep -n 'ipc_v1.map\|ADR-0018\|Cargo.toml\|grep -c'`
- compito 7: `grep -n -i 'richiamo'`; `sed -n '1672,1735p'`; `grep -n 'POLICY_FUNCTION\|"registry"\|Idempotent'`; `grep -n 'on_disconnect'`; `grep -n 'IpcMessage::Steps\|IpcMessage::Policy\|IpcMessage::Layout'`; `grep -nE '^\s*fn [a-z_]+\('` → le dieci sonde; `sed -n '970,1001p'`; `grep -n -i 'impl Custody\|MemoryCustody\|Refusing'`; `grep -n -i 'keep' … | grep -i 'err\|fail\|refus'` → nessuna
- compito 6: `grep -n 'replay'`; `grep -n -i 'richiamo'`; `sed -n '1036,1095p'`
- compito 8: `grep -n 'riga 9\|voce 9\|Trova\|decisione 56'`; `grep -n '2026-09-06-sottoprogetto-2\|§9'`; `sed -n '845,860p'`
- compito 9: `grep -n -i 'spegnimento\|limite di giri'`; `sed -n '760,790p;798,836p;700,760p'`; `grep -n 'Unavailable\|ritrova'`; `grep -nE 'fn '`; `grep -n '/\* … \*/'` → 4 (e su tutti i compiti → solo il 9)
- compito 10: `grep -n 'Passo [0-9]'`; `grep -n 'DyingGui\|§5\|Trova\|DST \|--nocapture'`; `sed -n '767,783p'`
- compito 11: `grep -n 'OutboundMessage'` + `awk` sul tipo → i quattro `kind`; `grep -n 'MESSAGE_KINDS'`
- compito 12: `grep -n 'senza copiarla'` → 0; `grep -n 'Passo [0-9]*:'` → dieci Passi; `grep -n '§7\|RICHIAMO DEL\|richiamo'`; `grep -n 'TURN_LIMIT\|u64::MAX\|"degrade"\|"verdict"\|2000\|cadence\|Steps\|Layout\|Invoke\|SaveLayout'`; `grep -n -i 'salva\|ritrova\|invocazione\|invocation'` → 0; `grep -nE 'fn '`; `sed -n '596,742p'`
- compito 13: `grep -n -i 'search\|ricerca'`; `grep -n 'beforeunload\|onDidLayoutChange'`; `grep -c 'who:'` → 21; `grep -n '{ name: "'` → 18; `sed -n '1740,1760p'`; `grep -n '"bar": {'` + 14 righe; `grep -n -i 'chip\|sparit\|vanish\|unknown\|badge\|pill'`; `grep -n -i 'strip' | grep -i 'degrad\|permiss'`; `grep -n '"chat"\|chat' | grep -i 'home\|work\|compact'`; `grep -n -i 'kit UI\|nessun kit'`
- compito 14: `grep -n 'lastVerdict\|validateLink\|protection\|aria-live\|polite\|focus-visible\|casella'`; `sed -n '2025,2050p'`; `grep -n 'direzione-gui-design'` (13/14/17)
- compito 15: `grep -n 'npm ci\|npm run build\|npm test\|npm run lint\|manifest-path\|npm audit\|GATE RED\|node-version-file\|setup-node@\|package-manager-cache\|Passo 11\|scritte\|gui: fake core and SPA\|porcelain'`; `sed -n '544,566p'`; `grep -n '<tempo>' compito-1*.md` → 15 (3 + criterio)
- compito 16: `grep -n '§8'` → una riga (`checkout` v4)
- compito 17: `grep -n 'git diff --stat\|Passo [0-9]*:\|riga 2\|§1\b\|Definizione'`; `sed -n '60,66p;166,226p'`
- compito 5: `grep -n -i 'commento falso\|false comment\|decisione 18\|voce 10\|TableDefinition\|boundary.rs\|not a .Record'` → solo la propria `TableDefinition`; `grep -l 'platform/src/journal.rs' compito-*.md` → solo il 5; contesto di `pub(crate)`
- `<piano>`: `grep -n -i 'riposo'` → 0; `grep -n -i '\bcpu\b' compito-07/09/12/15/17.md` → 0; `grep -n 'capo a capo'`; `grep -n 'gui/shell'` → 0; `grep -n 'quaterna'` → solo la busta; `grep -il 'openrouter' compito-*.md` → 3, 14 (etichette); `grep -n 'test result' compito-*.md` → solo il 1
- P-27, P-28, P-53, P-104, P-2 (righe 223–253) letti con `awk`/`sed`
- `<disegno2>`: `grep -c 'RICHIAMO DEL 2026-09-14'` → 0; `'RICHIAMO DEL 2026-09-11'` → 0; `'RICHIAMO DEL <data>'` → 0; `'RICHIAMO DEL'` → 16; `grep -c -F` delle ancore: «lascia il passo A in dubbio con la sua classe» 2; «e non riusi quello del workspace.» 1; «senza copiarla» 1 (riga 374); «servirebbero due limiti» 1; «un arresto pulito coi segnali» 1; «\| il giornale \|» 1; «\| l'invocatore \|» 1; «e la lista dei passi (§2 e §3 della stella polare) \|» 1; «dichiarato, non pinzato (gotcha #73) \|» 1; «resta vivo oltre centomila giri» 2; «un record congelato in più \|» 1; «\| \`Hello\` \|» 2; «\| \`Request\` \|» 1; «\| il limite di giri \|» 1; «\| lo spegnimento \|» 1; «\| l'attività \|» 1; «la CI resta solo Linux» 1; «ogni variante decodificata dai byte» 1; «la SPA parla \`bincode\`» 1; `grep -n 'manifest-path. compili nel'` → 474
- stella polare (righe mirate): `grep -n 'commento falso\|journal.rs'` → riga 966 (voce registrata); `sed -n '966p'`; `grep -n '^| 18 |'`; `grep -n '^| 41 |'` → riga 111 («il piano misura il processore a riposo senza soglia»); `grep -c 'RICHIAMO DEL'` → 11; `grep -c -F '<ancora del Passo 15 del 14>'` → 1; `grep -n '^| \*\*Modulo\*\*\|^| Modulo |\|^#### '`; `awk` del criterio del 13 → 13 (la corta); `grep -c '^#### '` → 10; `sed -n '359,364p;411,415p'`; `grep -n '^| 1 | '`
- codice di oggi: `grep -n 'run "' scripts/gate.sh`; `ls crates/kernel/tests/frozen/`; `grep -n 'const EXECUTOR_TURN_LIMIT' crates/daemon/src/main.rs` → `100_000`; `grep -n 'LENGTH_WIDTH: usize' crates/kernel/src/framing.rs` → 4; `grep -n 'Disconnected\|MalformedMessage' crates/kernel/src/ports/ipc.rs`; `grep -n 'pub vram_exhausted\|pub routing_degraded' crates/kernel/src/degradation.rs`; `grep -n 'boundary.rs' crates/platform/src/journal.rs` → riga 63; `git ls-files --eol` dei tre documenti → `i/lf w/lf`
- rete: `npm view <pkg>@<ver> version` e `dist-tags.latest` ×18; `npm view vitest dist-tags --json`; `curl -s https://crates.io/api/v1/crates/{interprocess,redb}` → 2.4.4 / 4.3.0
- chiusura: `git -C /c/Users/zagor/Desktop/harness status --porcelain` → (vuoto); `grep -c '| CONFERMATO |' R9a-report.md`

## Non verificato, e perché

- La **stella polare** non è stata letta per intero (fuori dal mio perimetro): ho letto solo le righe mirate (decisione 41, la voce registrata 966, la decisione 18, le intestazioni della §1, la riga 1 di *Passi*). Il censimento riga per riga delle colonne «chi/stato» della §1 per **R9a-13** è dedotto dalle intestazioni («Costruito dal **2**») e non contato.
- La spec del sotto-progetto 1 (righe 209, 575, 906 — P-22) non riletta: le nomina il vincolo globale 1 e il compito 4, che non sono nel mio perimetro.
- Nessun `cargo`/`npm` eseguito sul repository (solo `npm view` e `curl` verso i registri): le sonde dei compiti le ho lette nel modello, non fatte girare.
- I **download** della §9 (`api.npmjs.org`) non rimisurati: ho rimisurato le versioni e i `dist-tags`, che sono ciò che le `D` decidono.
- La riga «strada» delle risposte del proprietario e il capoverso «Ciò che la §8 non fa» sono **giudizi di prosa**: la loro falsità è derivata da D5 e dal compito 16, non da un comando che possa dirla.
- Il contenuto dei quattro corpi delle sonde del compito 9 (R9a-2) non esiste: non posso dire se, scritti «al momento», sarebbero vacui — solo che il testo dettato lo è.
- P-102 (`git show 8fc9696`) e P-113 (roadmap) non rilanciati: fuori perimetro, e non sostengono nessun rilievo mio.

## Stato finale

`git -C /c/Users/zagor/Desktop/harness status --porcelain` → (vuoto) · `git ls-files --eol` di `<piano>`, `<disegno2>`, `<stella>` → `i/lf w/lf` invariato · nessun file del repository toccato; le prove in `<scratchpad>/review/probe-R9a/`.
