# La direzione della GUI — la stella polare: la consegna del brainstorming del 2026-09-07

⚠️ **QUESTO FILE È LA CONSEGNA della sessione del 2026-09-07**, che ha ripreso il brainstorming del
sotto-progetto 2 dalla [consegna del 2026-09-06](2026-09-06-sottoprogetto-2-gui-minima-design.md) e,
per scelta del proprietario, lo ha **allargato alla forma di tutta la GUI**: non le due schermate del
2, ma la stella polare — Home, Lavoro, Compatta, il nucleo a pagina intera — di cui il 2 costruisce la
cornice più la propria fetta. Il proprietario ha fermato la sessione dopo il quarto wireframe, prima
delle sezioni scritte. La sessione che riprende legge questo file **per intero**, poi la consegna del 2,
conferma le approvazioni ancora sospese, chiude le sezioni che mancano una per volta, e scrive **sul
posto** i due disegni: questo file come *disegno della direzione della GUI*, l'altro come *disegno del
2*. Il testo di questa consegna va allora **parola per parola** in
`docs/archivio/consegna-brainstorming-direzione-gui.md`, coi soli link riscritti per la cartella — è il
viaggio della knowledge base (`07ab6dc` → `6a7967a`) e del 2 (`ae40fa0` → `664265a`).

⚠️ **Non è una spec e non è ancora il disegno.** Il prossimo passo sta nella §6 del
[compendio](../../COMPENDIO.md), in un posto solo.

## Stato in una riga

Stella polare a metà: sette decisioni del proprietario più la modularità, quattro wireframe disegnati e
salvati — Home approvata; Lavoro e Compatta col grafo **approvati come mappa alla ripresa del 2026-09-07**,
con la decisione 10 — ciò che un wireframe non porta va nel catalogo dei moduli — e la 11, il modello come indicatore; la sezione 1 **cominciata**, tabelle Chat e Stato approvate; nessun codice toccato; mancano il resto della sezione 1, le altre cinque sezioni della stella polare e le §7–§10 del 2,
poi i due disegni e il piano.

## ⛔ Da sapere subito

**Niente è a metà nel repository.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto
pushato, **nessun codice toccato**: questa sessione ha prodotto solo documenti — questo file, i tre
wireframe SVG nella cartella accanto, i richiami datati nella consegna del 2, il puntatore della §6.

⚠️ **Le approvazioni sono A CONDIZIONE**, con la stessa formula del 2: il proprietario ha risposto «A che
rispetti la skill» alla strada, e poi A o B a ogni domanda. Se scrivendo il disegno o il piano una
decisione viola un criterio di `anthropic-skills:decision-principles`, l'accettazione decade: ci si
ferma e lo si dice.

⚠️ **Due approvazioni sono sospese, e non vanno date per prese.** Il wireframe di **Lavoro**: il
proprietario ha scelto A sul modulo Passi e ha fatto una domanda sull'ambito, senza chiedere modifiche e
senza dire «approvato». Il wireframe di **Compatta e del grafo**: presentato con la domanda «va bene il
disegno?», e la sessione si è chiusa prima della risposta. La ripresa apre con queste due conferme, in
forma A/B.

✅ **RICHIAMO DEL 2026-09-07, ALLA RIPRESA: le due conferme sono DATE, entrambe A, «come mappa».** Prima di
rispondere il proprietario ha notato che nella barra della chat di Lavoro mancano il contesto della run, la
modalità di esecuzione e il tasto «+ allegati», e che «se dovessimo mettere tutte le info mancanti ce ne
sarebbero un botto»: da lì la **decisione 10** — un wireframe è una **mappa**, dove sta un modulo e chi lo
costruisce, e tutto ciò che un modulo mostra va nel **catalogo dei moduli**, sezione 1 delle sezioni che
mancano, con la fonte di ogni riga. Il paragrafo qui sopra resta com'era: è un verbale.

⚠️ **La modularità è decisa a condizione di una prova.** Il proprietario ha accettato i pannelli
agganciabili di `dockview` dicendo «deve davvero battere la 3», la tela libera. La prova è nello spike
del guscio: se provandola non dà il «Jarvis», si passa alla tela libera **prima** di scrivere la SPA.

⚠️ **Le §1 e §6a del 2 restano approvate ma portano un richiamo datato**: si riscrivono col disegno,
sulla base di questa stella polare. Il richiamo sta in testa alla consegna del 2 e dentro le due sezioni.

## Stato del repo alla chiusura, coi comandi che lo rifanno

| | Comando | Atteso |
|---|---|---|
| ramo | `git fetch --all --prune`, poi `git status -sb` | `## main...origin/main`, niente sotto |
| i commit di questa sessione | `git log --oneline 664265a..HEAD` | il commit di questa chiusura |
| codice e spec non toccati | `git diff --stat 664265a..HEAD -- crates/ scripts/ spikes/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | nulla |
| cancello | `bash scripts/gate.sh` | `GATE GREEN` — rilanciato su `664265a` prima di scrivere i documenti di questa chiusura, e `check-docs.sh` rilanciato dopo. Si rilancia, non si cita |
| documenti | `bash scripts/check-docs.sh` | `OK` |
| fine-riga | `git ls-files --eol docs/COMPENDIO.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/specs/2026-09-07-direzione-gui-wireframes/*.svg` | il compendio `i/lf w/crlf`, gli altri `i/lf w/lf` |
| i wireframe esistono | `ls docs/superpowers/specs/2026-09-07-direzione-gui-wireframes/` | `compatta-e-grafo.svg  home.svg  lavoro.svg` |
| margine del compendio | `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh` | positivo |

La baseline dei test la dà `cargo test --workspace --no-fail-fast --locked`, non questa riga.

## Fatto in questa sessione

1. Ripresa sulla consegna del 2 con `anthropic-skills:session-resume`: stato riletto coi comandi,
   nessuna divergenza; il cancello non rilanciato all'apertura perché non si toccava codice, rilanciato
   alla chiusura.
2. Prima della progettazione, la domanda del proprietario su tre funzionalità — azioni a un'ora fissa,
   sub-agenti e workflow, il PC «come Cowork» — risposta leggendo i documenti e il codice. L'esito è
   nella tabella *«Ciò che il repo diceva già»* qui sotto.
3. Il percorso classificato **architetturale**: la forma di tutta la GUI, non le due schermate del 2.
4. Le **cinque domande** poste una alla volta in forma A/B col consiglio, tutte risposte; poi le tre
   strade per la modularità, con lo stato dell'arte verificato alle fonti prima di porle.
5. Quattro wireframe disegnati in chat con lo strumento inline (decisione 8 della consegna del 2) e
   **salvati** come SVG autonomi nella cartella accanto a questo file.
6. Due divergenze fra documenti approvati trovate leggendo, e portate al proprietario: i pannelli mobili
   assegnati al 2 dal disegno dei gesti e non nominati dalle sezioni del 2; il replay dei trace
   assegnato alla GUI minima da tracciabilità e non costruito dal 2. Entrambe chiuse a favore del 2.
7. Questo file scritto; i richiami datati nella consegna del 2; il puntatore della §6 mosso.
8. **Alla ripresa, in una sessione nuova lo stesso giorno:** stato riletto coi comandi, nessuna divergenza,
   `GATE GREEN` e `check-docs.sh` OK all'apertura; le due conferme sospese date, entrambe A «come mappa»;
   la decisione 10 sul catalogo dei moduli; questo punto fermo, committato prima di scrivere la sezione 1.
9. **Alla seconda ripresa, lo stesso giorno:** stato riletto coi comandi, nessuna divergenza, `check-docs.sh` OK
   all'apertura, cancello non rilanciato perché non si tocca codice; la domanda del proprietario sul metodo —
   quando e dove si scrivono le informazioni che i wireframe non portano — risposta con la decisione 10; la
   decisione 11 sul modello, indicatore sì, selettore registrato.
10. La **tabella Chat**, prima del catalogo dei moduli: le fonti lette per intero — la §5 del compendio, le
    ventuno righe G di `spikes/GUI-REQUISITI.md`, le righe di tracciabilità sulla conversazione, la §4 e la
    §6a della consegna del 2 — presentata al proprietario con verificato e dedotto separati, e **approvata, A**;
    scritta nella §1 qui sotto, con la riga 24 aggiunta dopo l'approvazione e dichiarata; la riga su allegati
    e «aggiungi al contesto» nelle registrate.
11. La **tabella Stato**, sette righe, approvata (**A**) e scritta, coi due campi di oggi del degrado letti nel
    codice (`crates/kernel/src/degradation.rs`); le registrate sul grafico dell'occupazione GPU e su chi
    costruisce l'esportazione OTLP; la seconda passata su ADR e tracciabilità, chiesta dal proprietario, ha
    trovato altre righe candidate, che si scrivono solo dopo l'approvazione.
12. Le cinque righe candidate dello Stato (8–12) approvate (**A**) e scritte; la riga 18 della Chat guadagna la
    fonte «Run persistenti, ripresa e cancellazione», letta nella spec del kernel come annullamento (SP-4) e non
    come cancellazione dallo storico.

## Le decisioni del proprietario, una per domanda

| # | Domanda | Risposta |
|---|---|---|
| 0 | il perimetro: stella polare di tutta la GUI, o allargare il 2? | **A, «che rispetti la skill»** — il wireframe è la stella polare, in un documento suo; ogni vista porta il numero del sotto-progetto che la costruisce; il 2 costruisce la **cornice** (le viste, i moduli mobili, la Home con l'anello che a parole dice «niente ancora») più la sua fetta di oggi. Scartata B, allargare il 2 fino ad anello, rete e grafo veri: disegna cose che nessuno produce fino al 3, 6 e 7, ed era già caduta come domanda 12 del [disegno della knowledge base](2026-09-04-knowledge-base-design.md) |
| 1 | cosa conta come «artefatto»: le chat e le run stanno nell'anello? | **A — no.** Nell'anello solo **file** prodotti dalle run — documenti, codice, asset 3D, catture, esportazioni — per data (ADR-0008 e ADR-0018: l'artefatto è un file sul disco, riferito dal giornale). Le chat e le run hanno una tessera loro, Attività. La rete al centro ha **tutto**: artefatti e file della knowledge base messi dal proprietario |
| 2 | cosa succede «a fuoco» | **A** — la rete al centro è un modulo come gli altri: «mettere a fuoco» è aprirlo a pagina intera con lo stesso gesto delle tessere, e a pagina intera è il grafo della knowledge base già deciso (§4.3 del disegno della knowledge base). Scartata B, uno zoom speciale della Home: un secondo modo di aprire, fatto per il solo centro |
| 3 | cosa sono le tessere | **A** — ogni tessera è un **modulo in piccolo, vivo**, che si usa sul posto e si apre a pagina intera; quali esistono lo dice la lista G9–G18 di [`spikes/GUI-REQUISITI.md`](../../../spikes/GUI-REQUISITI.md) più i quattro pilastri; ogni tessera porta il numero del sotto-progetto che la riempie; una tessera senza dati lo dice a parole. Scartata B, un pulsante per pilastro: la Home diventa un menu |
| 4 | dove vive la disposizione dei moduli | **A** — nel core, nell'archivio di configurazione di ADR-0022, e **il 2 ne costruisce il pezzo minimo**: un archivio con una voce sola, la disposizione; il core la manda alla GUI al collegamento, la GUI manda «salva disposizione». È la decisione 12 del [disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md), registrata e ora presa. Salvarla nel browser della GUI è escluso da I1. Scartata B, rimandare l'archivio: si riordina e si perde tutto al riavvio |
| 5 | una modalità piccola sempre in primo piano | **A** — entra come terza vista, **Compatta**: si disegna ora, la costruisce il **10**, perché «sempre in primo piano» e la scorciatoia globale sono integrazione con l'OS (riga «Overlay/finestra fluttuante → GUI + L3» di tracciabilità). Zero codice nel 2 |
| 6 | la modularità: griglia, pannelli agganciabili, tela libera | **2, pannelli agganciabili**, a condizione che «batta davvero la 3»: `dockview-core` 8.2.0, usato **diretto** e non con l'adattatore Vue (ADR-0030 preferisce le librerie agnostiche); dipendenza nuova, in due passi. La prova è nello spike del guscio, sezione *«Lo spike di accettazione»*. Scartate: la griglia a tessere, che non sa fare schede e pannelli divisi per Lavoro; la tela libera, che sovrappone, salva in pixel e fa faticare la tastiera — resta la strada di riserva se la prova fallisce |
| 7 | la chat nella Home di default | **B** — no. La chat vive in Lavoro; in Home si aggiunge dal cassetto «+ moduli». «Agentic OS, non chatbot» |
| 8 | il modulo Passi nel 2 | **A** — nasce col 2, minimo: mostra i passi che il giornale ha oggi, cioè le invocazioni del registro con intento ed esito; col 3 cresce ai passi delle run. Chiude la divergenza con le due righe di tracciabilità «Replay dei trace» e «Osservabilità e tracing locale → GUI minima». Costo: un messaggio IPC con la lista dei passi, e un pannello |
| 9 | il PC «come Cowork» | «lavorare sui tuoi file e lanciare comandi va bene, per ora»: deciso nei meccanismi (ADR-0016, ADR-0024, ADR-0025) e visibile col 5. Guidare schermo, mouse e tastiera resta **solo un nome** — righe «Automazione OS → L3» e «Screenshot e comprensione dello schermo → L3 + Conversazione» — senza definizione: registrata, non presa |
| 10 | **alla ripresa, 2026-09-07** — quanto dettaglio porta un wireframe: il proprietario ha notato che nella barra della chat di Lavoro mancano il contesto della run, la modalità di esecuzione e il tasto «+ allegati», e che «se dovessimo mettere tutte le info mancanti ce ne sarebbero un botto» | **A** — un wireframe è una **mappa**: dove sta ogni modulo e chi lo costruisce; Lavoro e Compatta col grafo sono **approvati come mappa**. Tutto ciò che un modulo mostra, e i comandi che ha, va nel **catalogo dei moduli** (sezione 1 delle sezioni che mancano): una tabella per modulo con la **fonte** di ogni riga — G, ADR, riga di tracciabilità — e il sotto-progetto che la costruisce. Le tre cose notate hanno fonte (G11 · ADR-0016 · «Allegati in chat» di tracciabilità) ed entrano nella riga Chat, nella **barra della chat, per run**; la striscia sempre visibile resta il riassunto. Scartata B, ridisegnare i wireframe con tutto dentro: disegna oggi cose che arrivano col 3, 5, 6 e 13 e si rifanno col design system — la stessa ragione della B della domanda 0 |
| 11 | **alla seconda ripresa dello stesso giorno** — il proprietario ha aggiunto il **modello selezionato** alle cose che mancano nella barra della chat, e ha chiesto quando e dove si scrivono le informazioni che i wireframe non portano, per non crescere feature su feature | **A** — il metodo è la decisione 10, e il momento è la sezione 1, prima del disegno e del piano. Il modello entra nella riga Chat come **indicatore** del modello in uso, con fonte: il record di routing di ADR-0011 e la riga «Indicatore di stato modello» di tracciabilità, casa GUI. Un **selettore** a mano per chat non ha fonte, e tracciabilità ha già «Selettore di modello per compito ✅ §3 · profili»: la domanda «posso anche sceglierlo a mano, per run?» è registrata, chiusore il **3** col disegno del modulo Chat. Scartata B, decidere il selettore adesso: una riga G nuova e la riapertura di come si sceglie il routing, decisione strutturale che non si prende a fine giornata |

## Ciò che il repo diceva già, letto per decidere

| Funzionalità chiesta dal proprietario | Decisa? | Dove | Codice oggi | Arriva con |
|---|---|---|---|---|
| azioni automatiche a un'ora fissa | sì, come **trigger**: pianificazione, cambiamento di file, fine di un'altra run | ADR-0009; riga «Scheduling ✅» di [tracciabilità](../../tracciabilita.md) | solo l'orologio del reattore (`now`, `wall_time`, `wait_until`); nessun tipo `Trigger` (`grep -rl --include='*.rs' Trigger crates/` rende zero) | il meccanismo col **13**; la funzione per l'utente col **4** |
| sub-agenti e workflow | sì: un sub-agente è una **sub-run**, «proiezione ristretta della stessa struttura, col proprio segmento di giornale» ([design/03](../../design/03-run-durevoli.md)); la gerarchia run → sub-run → passo è ADR-0011; il piano nello stato durevole è ADR-0008 | righe «Orchestrazione e sub-agenti», «Planning», «Agenti in parallelo isolati» di tracciabilità | run e passo non esistono come tipi: c'è solo `StepId` (`grep -rnE 'RunId' crates/*/src` rende zero) | run e sub-run col **3**, piano e coda col **4**, isolamento su disco col **5** |
| il PC come Cowork: file e comandi | sì: permessi a tripla, checkpoint sugli ambiti dichiarati, nessun comando sotto il livello 2 | ADR-0016, ADR-0024, ADR-0025 | le porte `filesystem` e `process` nel kernel; nessuna capacità sopra | il **5** |
| il PC come Cowork: schermo, mouse, tastiera | **no**: solo il nome «Automazione OS» | tracciabilità, sezioni 7 e 8; nessun documento nomina mouse o tastiera (`grep -rnE 'mouse' docs --include='*.md'` fuori da questo file rende zero) | nulla | il **10**, senza disegno |

E le tre cose del repo che hanno **deciso la forma**, verificate coi comandi:

| Cosa | Dove | Che cosa ha deciso qui |
|---|---|---|
| «il sotto-progetto 2 costruisce pannelli e menu che si muovono con **qualunque puntatore**; la mano è un puntatore in più» | §4.1 del [disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md), e la riga 12 di «Perché quest'ordine» in [roadmap](../../roadmap.md): `grep -n 'pannelli mobili' docs/roadmap.md` | i moduli mobili sono **già del 2**; le sezioni approvate del 2 non li nominavano — divergenza chiusa dal perimetro A |
| il pannello della knowledge base: «disegna il grafo raggruppato per router; filtra per specie, cartella, etichetta; cerca sui nomi; al click mostra il nodo e le funzioni del registro; orfani e collegamenti rotti; non tocca file, non tiene stato» | §4.3 del [disegno della knowledge base](2026-09-04-knowledge-base-design.md), decisioni 6, 9, 10, 17 | il nucleo a pagina intera **è** quel pannello, nato col 6; qui non si ridisegna, si colloca |
| «se la posizione dei pannelli sopravvive a un riavvio: configurazione, archivio di ADR-0022, che non esiste: la chiude chi lo costruisce» | decisione 12 del disegno dei gesti: `grep -n 'posizione dei pannelli' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` | presa oggi: il 2 costruisce l'archivio minimo (domanda 4) |
| «L'interfaccia deve mostrare **cosa è coperto** dall'ambito attivo, prima che l'agente inizi a scrivere»; «un limite di dimensione oltre il quale un file viene escluso dal checkpoint **con avviso**» | follow-up di [ADR-0024](../../adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) | il modulo Ambito di Lavoro, con «fuori ambito: non coperto» e «file grande: escluso, con avviso» |
| «la leva non è la GUI ma la **frequenza di aggiornamento** decisa dal core»; «la webview esegue solo contenuto locale nostro»; G13 dal primo giorno | follow-up di [ADR-0027](../../adr/0027-stack-della-gui.md) | la difesa contro tante chat che scrivono insieme; la provenienza in ogni pezzo di flusso |
| «Replay dei trace 🔶 → GUI minima», «Osservabilità e tracing locale 🔶 → GUI minima» | tracciabilità, sezioni 4 e 8: `grep -n 'GUI minima' docs/tracciabilita.md` | il modulo Passi nasce col 2 (domanda 8) |

## Lo stato dell'arte verificato, e il comando

Fonte primaria: il registro npm interrogato il 2026-09-07 col comando sotto; la documentazione ufficiale
di `dockview` letta lo stesso giorno alle pagine nominate. ⚠️ **Casa unica provvisoria**: quando il
disegno sarà scritto queste righe passano in [`riferimenti.md`](../../riferimenti.md), e qui resta il
rimando — è la regola di `CLAUDE.md` alla chiusura di una voce con una fonte.

```
python - <<'EOF'
import json, urllib.request, urllib.parse
def npm(p):
    d = json.load(urllib.request.urlopen("https://registry.npmjs.org/" + urllib.parse.quote(p, safe="@")))
    v = d["dist-tags"]["latest"]; vv = d["versions"][v]
    return v, d["time"][v][:10], vv.get("license"), (vv.get("peerDependencies") or {}).get("vue", "-")
def dl(p):
    return json.load(urllib.request.urlopen("https://api.npmjs.org/downloads/point/last-week/" + urllib.parse.quote(p, safe="@")))["downloads"]
for p in ["dockview", "dockview-core", "dockview-vue", "splitpanes", "gridstack", "grid-layout-plus", "interactjs", "sigma", "graphology", "d3-force", "cytoscape", "@vue-flow/core", "force-graph", "three", "pixi.js", "markdown-it", "marked", "shiki", "codemirror", "dompurify"]:
    print(p, *npm(p), dl(p))
EOF
```

| Pacchetto | Versione | Pubblicata | Licenza | Vue richiesto | Download/settimana | Per che cosa |
|---|---|---|---|---|---|---|
| `dockview-core` | 8.2.0 | 2026-08-19 | MIT | — | 303 488 | **scelto**: il motore dei moduli, usato diretto |
| `dockview` | 8.2.0 | 2026-08-19 | MIT | — | 215 513 | lo stesso, col pacchetto ombrello |
| `dockview-vue` | 8.2.0 | 2026-08-19 | MIT | ^3.4.0 | 6 105 | l'adattatore Vue: **non scelto**, usato da pochi |
| `splitpanes` | 4.1.2 | 2026-05-26 | MIT | ^3.2.0 | 144 877 | solo pannelli divisi: non basta |
| `gridstack` | 13.2.0 | 2026-08-20 | MIT | — | 499 310 | la griglia a tessere, strada 1, scartata |
| `grid-layout-plus` | 1.1.1 | 2025-10-13 | MIT | ^3.0.0 | 77 895 | idem, versione Vue |
| `interactjs` | 1.10.28 | 2026-08-01 | MIT | — | 591 557 | la tela libera, strada 3, **di riserva** |
| `sigma` | 3.0.3 | 2026-04-30 | MIT | — | 240 975 | grafo su WebGL: candidato per il 6, non scelto qui |
| `graphology` | 0.26.0 | 2025-01-26 | MIT | — | 1 405 123 | la struttura del grafo sotto `sigma` |
| `d3-force` | 3.0.0 | 2021-06-05 | ISC | — | 16 164 124 | la fisica della rete viva: candidato, non scelto qui |
| `cytoscape` | 3.34.2 | 2026-08-25 | MIT | — | 14 670 550 | grafo: candidato per il 6 |
| `@vue-flow/core` | 1.48.2 | 2026-01-28 | MIT | ^3.3.0 | 476 176 | grafo a nodi Vue: candidato per il 6 |
| `force-graph` | 1.51.4 | 2026-04-16 | MIT | — | 595 590 | grafo a forze su canvas: candidato |
| `three` | 0.185.1 | 2026-07-01 | MIT | — | 14 025 392 | il viewer 3D, ADR-0030 |
| `pixi.js` | 8.20.1 | 2026-08-26 | MIT | — | 920 303 | la rete viva su WebGL: candidato, non scelto qui |
| `markdown-it` | 15.0.1 | 2026-08-27 | MIT | — | 27 048 598 | il renderer di markdown: decisione aperta della §9 del 2 |
| `marked` | 18.0.11 | 2026-08-24 | MIT | — | 66 978 414 | idem |
| `shiki` | 4.4.3 | 2026-08-10 | MIT | — | 21 210 699 | colore del codice nei blocchi |
| `codemirror` | 6.0.2 | 2025-06-19 | MIT | — | 7 109 920 | l'editor, ADR-0030 |
| `dompurify` | 3.4.15 | 2026-09-06 | MPL-2.0 OR Apache-2.0 | — | 45 528 659 | **non serve** se il testo non fidato si rende come testo e mai come HTML, §6a del 2 |

**`dockview` 8.x, letto su dockview.dev il 2026-09-07** — pagine `docs/overview/licence`,
`blog/dockview-enterprise`, `docs/core/groups/floatingGroups`, `docs/core/groups/maximizedGroups`,
`docs/core/groups/popoutGroups`, `docs/core/state/save`, `docs/core/locked`, `docs/advanced/accessibility`,
`docs/advanced/keyboard`, `docs/core/panels/move`, `docs/core/panels/tabs`, `docs/core/dnd/thirdParty`:

| Fatto | Conseguenza qui |
|---|---|
| dalla 8.0.0 due forme: il core resta **MIT** e gratis, `dockview-enterprise` è a pagamento; «nothing you rely on today has been taken away» | si usa il core; niente di enterprise entra nel disegno |
| **gratis**: gruppi galleggianti (quanti si vuole, con layout annidato, tenuti dentro la finestra, con un gancio `transformFloatingGroupDrag` per lo scatto a griglia o l'allineamento); massimizza e ripristina; finestre popout; salva e ripristina il layout (`toJSON`); blocco del layout e dei singoli gruppi; tocco e penna; drag esterno e librerie terze; schede e presa personalizzate; gruppi senza intestazione; `moveTo` programmatico; ruoli ARIA, annunci allo screen reader, indicatori di focus, navigazione di focus programmatica; CSP stretta | è ciò con cui la 2 «batte la 3»: pannelli liberi, pagina intera, finestra a parte, mano come puntatore, presa grande disegnata da noi, nucleo bloccato, salvataggio in JSON |
| **a pagamento**: navigazione spaziale da tastiera e aggancio da tastiera; guide e bussola durante il trascinamento; cronologia annulla/ripeti; gruppi ai bordi che si nascondono; schede appuntate, multi-riga, menu contestuali | G20 chiede la tastiera: le scorciatoie per spostare un pannello **si scrivono noi** sopra `moveTo`, dichiarato; lo scatto a griglia si scrive noi sul gancio; annulla/ripeti non è chiesto |
| v8 è additiva: ogni novità è opt-in, un solo cambio di comportamento (le dimensioni riportate ai pannelli escludono l'intestazione) | «novità non è maturità» regge: la 8.x non rompe la 7.x; le versioni si riverificano il giorno dello spike |

## I wireframe, e il loro stato

Disegnati in chat con lo strumento inline, poi salvati come SVG autonomi. Sono a bassa fedeltà: box,
etichette, il numero del sotto-progetto che riempie ogni modulo, e il tratteggio per «modulo mobile».
Colori e forme si decidono dopo, nel design system in tre momenti del 2.

| Wireframe | File | Stato | Verificato · dedotto · assunto |
|---|---|---|---|
| **Home** | [home.svg](2026-09-07-direzione-gui-wireframes/home.svg) | ✅ **approvata**, con la chat nel cassetto (domanda 7) | verificato: le tessere da G9–G18, il pannello knowledge base dal suo disegno, le funzioni di `dockview`; dedotto: la striscia «sempre visibile» come gruppo bloccato di `dockview`, da provare nello spike; assunto: l'aspetto Jarvis lo danno colori e forme |
| **Lavoro** | [lavoro.svg](2026-09-07-direzione-gui-wireframes/lavoro.svg) | ✅ **approvata come mappa alla ripresa del 2026-09-07** — diceva *«⏳ presentata: A su Passi, nessuna modifica chiesta, approvazione da confermare»*; il contesto della run, la modalità di esecuzione e «+ allegati» entrano nella riga Chat del catalogo, nella barra della chat, per run (decisione 10) | verificato: ogni riquadro ha una riga G o un ADR (G4, G5, G7, G13, G14, G18, ADR-0014, ADR-0016, ADR-0017, ADR-0024); dedotto: la richiesta di permesso in riga invece che in finestra — nel 2 resta la finestra della §6a, in riga è la forma del 3; assunto: niente |
| **Compatta e nucleo a pagina intera** | [compatta-e-grafo.svg](2026-09-07-direzione-gui-wireframes/compatta-e-grafo.svg) | ✅ **approvata come mappa alla ripresa del 2026-09-07** — diceva *«⏳ presentata, NON approvata: la sessione si è chiusa prima della risposta»*; ciò che Compatta e il nucleo a pagina intera mostrano va nel catalogo, righe del 10 e del 6 (decisione 10) | verificato: il pannello dalla §4.3 del disegno della knowledge base, decisioni 6, 9, 10; gli stati ascolto/pensiero/parlato da tracciabilità; V9 per la notifica; l'indicatore acceso dal core, ADR-0039; dedotto: trascinare un nodo è la funzione «sposta» del registro, quindi chiede permesso; se Compatta sia una finestra staccata di `dockview` o la finestra principale rimpicciolita lo decide il 10; assunto: niente |

Il quarto disegno, Lavoro, contiene la **quarta vista** implicita: un modulo a pagina intera è la stessa
cosa per tutti i moduli, quindi non ha un wireframe suo.

## Il modello della GUI, com'è stato approvato in chat

| Pezzo | Forma | Da dove |
|---|---|---|
| **modulo** | un **tipo** registrato nella SPA — Chat, Stato, Attività, Ambito, Diff, Anteprima, Terminale, Passi, Sensori, Costi, Permessi, Knowledge base, Asset 3D, Voce e gesti, Nucleo — e i suoi **esemplari**: un pannello di `dockview` con parametri, per esempio il numero della run. Un tipo, tanti esemplari; i tipi sono pochi, gli esemplari quanti si vuole | domanda 3; il modello di `dockview` |
| **regola unica** | ogni modulo si usa sul posto, in piccolo, **oppure** si apre a pagina intera; vale anche per il nucleo (domanda 2) | domande 2 e 3 |
| **vista** | un layout salvato con un nome: JSON di `dockview`; tre arrivano con l'app — **Home**, **Lavoro**, **Compatta** — e il proprietario può salvarne altre; costa un nome e una lista | domande 5 e 6, e la risposta su come cresce |
| **Home** | il **nucleo** al centro, bloccato: l'anello degli artefatti recenti per data, e dentro la rete viva di tutti i file; intorno le tessere agganciate; sopra la barra con le viste, la ricerca, il chip del core, e la fascia che compare solo se il core manca o il timbro è sbagliato; sotto la striscia sempre visibile e il cassetto «+ moduli» | il wireframe Home |
| **Lavoro** | Attività e Ambito a sinistra; la chat a schede al centro, ogni scheda una run, «stacca» per un pannello libero o un'altra finestra; Diff, Anteprima e Terminale a destra; Passi e Sensori in basso | il wireframe Lavoro |
| **Compatta** | una finestrella sempre in primo piano: la rete viva senza anello come presenza (ascolto, pensiero, parlato, fermo), l'ultima notifica, microfono e telecamera accesi dal core, la scorciatoia globale, «grande» per tornare al nucleo | il wireframe Compatta, da confermare |
| **striscia sempre visibile** | in ogni vista: degrado, permessi, contesto, costo e tetto, attese, telecamera, microfono — la lista «deve mostrare sempre» G9–G14 più i due indicatori di percezione | G9–G14; ADR-0039 |
| **ricerca** | sui nomi degli artefatti, dalla barra: colora i trovati nell'anello e nella rete; a pagina intera è la ricerca del pannello knowledge base | domanda 1; §4.3 del disegno della knowledge base |
| **come cresce** | più chat = più esemplari come schede, affiancati o in un'altra finestra; la Home non si allunga: Attività è l'indice di tutte le run, l'anello mostra gli ultimi file e il nucleo a pagina intera li ha tutti; una funzione nuova = un tipo nuovo registrato, che compare nel cassetto col suo numero; le viste sono JSON, niente da ridisegnare | la risposta al proprietario sul come regge |
| **dove si rompe, e cosa lo tiene** | tante chat che scrivono insieme mangiano CPU, e P3 era già stretto: `dockview` disegna solo i pannelli visibili («render modes»), la frequenza la decide il core (ADR-0027), e — dedotto, per il 3 — la GUI dice al core quali run guarda e il core manda solo quelle; un layout salvato che punta a una run sparita: il pannello lo dice a parole e si chiude | ADR-0027; la tabella delle licenze di `dockview` |
| **la disposizione** | vive nel core, archivio di configurazione di ADR-0022, che il 2 costruisce con una voce sola; il core la manda al collegamento, la GUI manda «salva disposizione» | domanda 4 |
| **la mano** | i moduli si muovono con qualunque puntatore; il 12 aggiunge il pinch: `dockview` supporta tocco e penna e le librerie terze di trascinamento | §4.1 del disegno dei gesti; la doc di `dockview` |
| **cosa mostra davvero il 2** | la cornice con `dockview`; Home col nucleo che a parole dice «niente ancora», Stato e Permessi vivi; Lavoro con la chat del core finto, la finestra di permesso, Passi con le invocazioni del registro; le altre tessere dicono a parole chi le riempie; Compatta non esiste ancora | domande 0 e 8 |

## Le sezioni approvate del disegno — il merito com'è stato approvato

### §1 — Il catalogo dei moduli · in corso: la tabella Chat approvata il 2026-09-07

La regola, dalla decisione 10: un wireframe è una **mappa**; tutto ciò che un modulo mostra, e i comandi
che ha, sta qui, **una tabella per modulo**. Ogni riga porta la **fonte** — una riga G di
[`spikes/GUI-REQUISITI.md`](../../../spikes/GUI-REQUISITI.md), un ADR, una riga di
[tracciabilità](../../tracciabilita.md) — e il **sotto-progetto che costruisce la riga**, coi numeri della
[roadmap](../../roadmap.md); il «dove» dentro il modulo quando conta. Una riga **senza fonte non entra**:
prima diventa una riga G. Ciò che non si può decidere oggi va nelle «Registrate, non prese» con chi lo
chiude. Il catalogo dice **cosa** e **chi**, non come appare: ogni sotto-progetto disegna il proprio modulo
quando arriva, e una funzione nuova è una riga nuova, non un wireframe nuovo. Un modulo il cui
sotto-progetto non è chiuso mostra a parole chi lo riempie.

#### Chat · approvata il 2026-09-07

**Tipo** registrato nella SPA; **esemplare**: una scheda per run. Costruito dal **2** — la cornice, il flusso
del core finto, la finestra di permesso — e dal **3**, la chat vera. Messaggi IPC oggi: `Token`,
`PermissionRequired`, `Approve`; col 3 i messaggi della run.

| # | Dove | Cosa mostra o fa | Fonte | Chi | Verificato · dedotto |
|---|---|---|---|---|---|
| 1 | flusso | testo in markdown, blocchi di codice, un token alla volta | G4 · `Token` | 2 col core finto, 3 vero | verificato |
| 2 | flusso | ogni pezzo con la provenienza; il non fidato si rende come testo, mai HTML, e nessun link si apre da solo | G13 · ADR-0014 · §6a del 2 | 2 | verificato |
| 3 | flusso | la richiesta di permesso, la tripla a parole: finestra nel 2, **in riga** dal 3 | ADR-0016 · ADR-0038 · `PermissionRequired`, `Approve` | 2, 3 | verificato; «in riga» dedotto |
| 4 | flusso | «la run aspetta te», con notifica | G14 · V9 | 3 | verificato |
| 5 | flusso | l'esito di un fallback: avviso nel flusso se la catena cede su qualità o costo, errore se cede sui vincoli dei dati | ADR-0012 | 3 | verificato |
| 6 | flusso | `InCoda` e `Rifiutata` come due esiti diversi, resi diversi | G15 · ADR-0012 | 3 | verificato |
| 7 | flusso | un passo in dubbio dopo un crash, effetto irripetibile: domanda all'utente, nessun replay | ADR-0007 | 3 | verificato |
| 8 | flusso | un messaggio potato dalla ritenzione: si vede che c'era, con impronta e dimensione, mai indistinguibile da uno mai registrato | ADR-0018 | 3 | fonte verificata; il «chi» dedotto |
| 9 | flusso | il primo uso di uno strumento MCP: descrizione integrale e impronta all'approvazione; se cambia, sospeso col diff | ADR-0015 · righe «MCP» e «Difesa da tool poisoning» di tracciabilità | 4 | verificato |
| 10 | barra, per run | la casella di scrittura e «invia» | G4 · §1 del 2: «casella di scrittura → il 3» | 3 | verificato |
| 11 | barra, per run | il contesto della run: occupazione per categoria | G11 · ADR-0010 · riga «Indicatore di riempimento contesto» | 3, col dato dal 13 | verificato, decisione 10 |
| 12 | barra, per run | la modalità di esecuzione: i tre preset | ADR-0016 | 3 | verificato, decisione 10; per run o globale nelle registrate |
| 13 | barra, per run | il modello in uso, dal record di routing | ADR-0011 · riga «Indicatore di stato modello» | 3 | verificato, decisione 11; il selettore a mano nelle registrate |
| 14 | barra, per run | «+ allegati»: file e immagini, marcati non fidati | righe «Allegati in chat» e «Input immagini e vision» · ADR-0014 | 3 | verificato; il rapporto con «aggiungi al contesto» nelle registrate |
| 15 | barra, per run | il microfono: dettatura e push-to-talk, gli stati ascolto, pensiero, parlato | righe «Push-to-talk e dettatura» e «Stati di ascolto/pensiero/parlato» · ADR-0011 | 8 | fonte verificata; il «dove» dedotto |
| 16 | barra, per run | le guide attive per questa run: skill e profilo | ADR-0009 · righe «Skills» e «System prompt, personas e profili» | 13 il registro, 3 la mostra | fonte verificata; il «dove» dedotto |
| 17 | barra, per run | gli strumenti di questa run: server MCP attivi e sospesi | ADR-0003 · ADR-0019 · righe «MCP» e «Tool calling» | 4 | fonte verificata; il «dove» dedotto |
| 18 | comando | ferma la risposta; il costo dello stream interrotto resta nel giornale | riga «HITL: interruzione e steering» · riga «Run persistenti, ripresa e cancellazione» — la «cancellazione» è l'annullamento di una run o di una richiesta in corso, SP-4 della spec del kernel · ADR-0011 | 4 | verificato; fonte allargata il 2026-09-07 dopo l'approvazione, contenuto invariato |
| 19 | comando | «+ nuova run»: una chat è una run | ADR-0011, corollario | 3 | verificato; «chiede la cartella» nelle registrate |
| 20 | comando | «stacca» la scheda in un pannello libero o in un'altra finestra; manipolazione della GUI, non passa dal registro | `dockview` · ADR-0038 | 2 | verificato |
| 21 | comando | fork e branching, modifica e rigenerazione, ricerca nello storico, template e prompt salvati, esportazione: una riga ciascuno quando il 3 li disegna | le cinque righe di tracciabilità, casa Conversazione | 3 | verificato |
| 22 | scheda | una scheda è una run, col numero e lo stato; dove si vedono le sub-run lo disegna il 3 | ADR-0011 · righe «Sessioni multiple» e «Orchestrazione e sub-agenti» | 3 | verificato; le sub-run dedotte |
| 23 | scheda | costo e token di questa run | ADR-0011 | 3 | fonte verificata; il «dove» dedotto |
| 24 | flusso | senza una run, la chat lo dice a parole invece di restare vuota | §6a del 2, «i quattro stati della connessione» | 2 | ⚠️ **aggiunta dopo l'approvazione**, scrivendo, dalla §6a già approvata: il proprietario può toglierla |

**Non entra, per decisione già presa:** nessun comando «compatta» — la ricomposizione della proiezione è
continua e proattiva (ADR-0010): il contesto si vede (riga 11), non si comanda; nessuna pulizia del testo
non fidato — si marca, non si sana (ADR-0014).

**Esaminate e senza fonte oggi**, quindi fuori finché non diventano una riga G: rinominare una run;
cancellare una chat dallo storico (il giornale è append-only, ADR-0007: sarebbe una decisione, non una riga; la «cancellazione» di tracciabilità è un'altra cosa, l'annullamento di una run in corso, riga 18); «pensa di
più»; un interruttore «modalità piano» nella barra (il piano è del 4 come capacità, non come interruttore);
mettere in coda un messaggio mentre il modello scrive; fissare o archiviare una chat.

Debiti dichiarati: le righe del 4, dell'8 e del 13 nascono a parole nel 2 e nel 3 — «chi le riempie» — come
dice la regola; la Chat del 2 non ha la casella di scrittura (riga 10, il 3). 🔶 Dedotto, da confermare da
chi costruisce: «in riga» per la richiesta di permesso dal 3 (riga 3); il «chi» della riga 8; il «dove»
delle righe 15, 16, 17 e 23; le sub-run della riga 22.

Controllo sui cinque criteri: fonti lette il 2026-09-07 nel compendio (§5 intera), in
`spikes/GUI-REQUISITI.md` e in tracciabilità, i dedotti marcati; stessa forma della decisione 10 e numeri
della roadmap; le voci aperte nelle registrate; nessuna dipendenza da scegliere qui; solo righe con una
fonte oggi, niente disegno in pixel.

#### Stato · approvata il 2026-09-07

**Tipo** registrato nella SPA; **esemplare**: uno. È la tessera «Stato» della Home, viva nel 2; la striscia
sempre visibile ne è il riassunto (decisione 4 del coordinatore), il modulo è l'intero. Costruito dal **2**.
Messaggi IPC del disegno del 2, nomi provvisori (§4 del 2): `Degradation`, `Policy`, `Accepted`, `Verdict`.

| # | Cosa mostra | Fonte | Chi | Verificato · dedotto |
|---|---|---|---|---|
| 1 | il degrado corrente: i due campi di oggi — `vram_exhausted` e `routing_degraded`, in `crates/kernel/src/degradation.rs` — ognuno con la causa; cresce con chi porta le cause che ADR-0019 nomina: connettività, arbitro GPU, salute dei provider (3), permessi, strumenti sospesi (4); e la telecamera (12, ADR-0039) | G9 · ADR-0019 · `Degradation` · ADR-0039 · righe «Degrado esplicito quando manca la rete» e «Comportamento offline» | 2, poi 3, 4, 12 | verificato, campi letti nel codice |
| 2 | la policy VRAM attiva e il budget allocato sul totale; il totale è il budget allocabile: tutto meno la quota audio e la quota di presentazione | ADR-0006 · ADR-0005 · ADR-0033 · `Policy` · righe «Budget VRAM esplicito» e «Policy differenziata remoto vs locale» | 2 | verificato; le due quote sottratte visibili: dedotto |
| 3 | «protetto quanto il tuo account di sistema», come valore da `Accepted`, non scritta fissa | G16 · ADR-0023 · `Accepted` | 2 | verificato; la cifratura reale è «sede da assegnare» in tracciabilità |
| 4 | una riga di evento per l'ultimo `Verdict`, solo quando ne arriva uno; `InCoda` e `Rifiutata` distinti | G15 · ADR-0012 · `Verdict` · §6a del 2 | 2; il mittente di `Request` arriva col 7 | verificato |
| 5 | il profilo «riservato» attivo e ciò che spegne: avvio automatico, voce always-on, telecamera | ADR-0023 · ADR-0039 | 3, col gestore dei segreti (riga «Gestione segreti e credenziali») | fonte verificata; il «chi» dedotto |
| 6 | la transizione di policy, quando è offerta: gli effetti osservabili prima di accettarla — eviction, ricarica, notifica | ADR-0006 · riga «Swap coordinato» | 9 | fonte verificata; il «chi» dedotto |
| 7 | l'occupazione della GPU nel tempo, come grafico | G8 · ADR-0005 | 9 | fonte verificata; se vive qui o nel modulo Modelli locali: nelle registrate |
| 8 | chi tiene la VRAM adesso: le concessioni attive per titolare — i worker, la quota audio, la quota di presentazione della GUI stessa | ADR-0005 · ADR-0033 · righe «Semaforo unico delle risorse GPU» e «Budget di VRAM riservata all'audio» | 7 o 9, chi porta il primo worker che tiene VRAM | fonte verificata; il «chi» dedotto; serve un messaggio IPC nuovo, dedotto |
| 9 | i livelli di confinamento disponibili su questa macchina; se manca il livello 2, l'esecuzione di codice non parte | ADR-0025 · ADR-0019 · righe «Sandboxing ed esecuzione» e «Permessi e sandbox policy», L-5 | 5 | fonte verificata; se qui o nel modulo Permessi: dedotto |
| 10 | dopo un riavvio: la riconciliazione del giornale, e quanti passi restano in dubbio | ADR-0007 · ADR-0018 · riga «Run persistenti, ripresa e cancellazione» | 3 | fonte verificata; il «dove» dedotto |
| 11 | la telemetria: nessuna lascia la macchina per default; se l'esportazione è accesa, verso dove | ADR-0017 · V25 · riga «Telemetria locale e «no telemetry» garantito» | chi costruisce l'esportazione: nelle registrate, del proprietario | fonte verificata; il «chi» dedotto |
| 12 | la versione del programma, e se c'è un aggiornamento | riga «Packaging e aggiornamenti» | 10 | fonte verificata; se qui o nel modulo Impostazioni: dedotto |

**Nessun comando:** il cambio di policy è una funzione del registro e sta nel modulo Impostazioni (§5 e §6a
del 2, ADR-0038): Stato mostra, non comanda.

**Esaminate e senza fonte oggi:** l'uso di CPU e RAM (M1–M5 sono misure dello spike, non righe della GUI); un
tasto «riavvia il core» (la fascia ha «riprova», che ricollega, §6a del 2); l'elenco dei worker (ADR-0004 e
ADR-0028 li descrivono, nessuna riga dice di mostrarli: entrano solo come titolari di VRAM, riga 8). Il registro
degli eventi non è qui: è il modulo Passi (riga «Osservabilità e tracing locale → GUI minima»).

Debiti dichiarati: le righe del 3, del 5, del 7, del 9, del 10 e del 12 nascono a parole nel 2. 🔶 Dedotto, da
confermare da chi costruisce: le quote sottratte della riga 2; il «chi» delle righe 5, 6, 8 e 11; la casa delle
righe 7, 9, 10 e 12; il messaggio IPC della riga 8. ✅ Le righe 8–12 vengono dalla seconda passata su ADR e
tracciabilità chiesta dal proprietario, approvate il 2026-09-07 prima di essere scritte.

Controllo sui cinque criteri: fonti lette il 2026-09-07 — la §5 del compendio, le righe G, tracciabilità, la
§4 e la §6a del 2, e i due campi nel codice; stessa forma della tabella Chat; le voci aperte nelle registrate;
nessuna dipendenza da scegliere; solo righe con una fonte oggi.

## Le sezioni che mancano — proposte del coordinatore, non decisioni

| § | Che cosa | La proposta da cui partire |
|---|---|---|
| 1 | **il catalogo dei moduli**: tipi, numero del sotto-progetto, messaggi IPC che consumano | una tabella per tipo: Chat (2, 3 · `Token`, e col 3 i messaggi della run), Stato (2 · `Degradation`, `Policy`, `Accepted`, `Verdict`), Permessi (2 · `PermissionRequired`, `Approve`), Passi (2, 3 · un messaggio nuovo con la lista dei passi), Attività (3, 4, 13), Ambito (5), Diff (5), Anteprima (3), Terminale (5), Sensori (4), Costi (3), Knowledge base e Nucleo a pagina intera (6), Asset 3D (7), Voce e gesti (8, 12), Backup (11), Checkpoint (5), Modelli locali (9), Impostazioni (2, il cambio di policy è già una funzione del registro). La regola: un modulo il cui sotto-progetto non è chiuso mostra a parole chi lo riempie. ⚠️ **Allargata alla ripresa del 2026-09-07, decisione 10:** per ogni modulo anche **cosa mostra** e **quali comandi ha**, riga per riga con la **fonte** — G, ADR, riga di tracciabilità — e il sotto-progetto che costruisce la riga; e il «dove» dentro il modulo quando conta: contesto della run, modalità di esecuzione e «+ allegati» nella barra della chat, per run ✅ **RICHIAMO DEL 2026-09-07, seconda ripresa:** la sezione è **cominciata**: le tabelle **Chat** e **Stato** sono approvate e stanno nella §1 delle sezioni approvate qui sopra; restano gli altri moduli, uno per volta |
| 2 | **viste e disposizione**: i layout come JSON, l'archivio minimo nel core, i due messaggi | `Layout` dal core all'accoglienza, dopo `Accepted`; `SaveLayout` dalla GUI; l'archivio in `platform` con una voce, nella forma dell'archivio «configurazione, guide, profili» di ADR-0022, consegnato al daemon e non letto dal kernel (ADR-0034); ⚠️ da decidere lì: se «salva disposizione» sia una **funzione del registro** con la propria tripla (ADR-0038 dice che la manipolazione della GUI non passa dal registro; il salvataggio durevole è un'altra cosa) o una scrittura di configurazione fuori dal registro; le tre viste di default come JSON committati in `gui/` |
| 3 | **la fetta del 2 ritagliata**: che cosa costruisce adesso, e come si riscrivono §1 e §6a | §1 del 2 guadagna: il motore dei moduli (`dockview-core`, dipendenza nuova, in due passi), le tre viste con Compatta come segnaposto, il modulo Passi col suo messaggio, l'archivio della disposizione coi due messaggi; §6a: «le due schermate» diventano «Home e Lavoro nella cornice», la finestra di permesso resta; il pezzo 6 della tabella di §1 cambia forma. Tutto con richiamo datato, non riscrittura silenziosa |
| 4 | **lo spike di accettazione** di `dockview`, dentro lo spike del guscio | in `spikes/gui-shell/`, sul frontend minimo di §2 del 2: una Home finta con `dockview-core` — nucleo bloccato, quattro tessere, una libera, una a pagina intera, presa grande — e la giudica il **proprietario provandola**, come per la mano in SP-7; il criterio scritto **prima** in `spikes/gui-shell/PROTOCOLLO.md`; se non dà il «Jarvis», si passa a `interactjs` prima di scrivere la SPA. In più M4 misura P3 con `dockview` acceso |
| 5 | le sezioni che già mancavano al 2: **core finto, prove e cancello, decisioni aperte, come si riprende** | le proposte stanno nella consegna del 2, tabella «Le sezioni che mancano»; Passi aggiunge al core finto l'invio della lista dei passi; le prove aggiungono la sonda sul giro «salva disposizione, riavvia, ritrova» |
| 6 | **dove vive la stella polare**, e come il disegno del 2 la rimanda | questo file diventa `2026-09-07-direzione-gui-design.md` come disegno, ⛔ **non è una spec e non disegna le capacità**: colloca i moduli, le viste e le regole, e ogni sotto-progetto disegna i propri moduli quando arriva; il disegno del 2 rimanda qui per la forma e non la ricopia; una riga nella §12 del compendio e in «Dove va cosa» di `README.md` è un **compito del piano**, come fu per la knowledge base |

Poi: i due disegni scritti **sul posto**, con la revisione del disegno (segnaposto, coerenza, ambiguità,
perimetro), la rilettura del proprietario, e `superpowers:writing-plans` col piano del 2 in **due
parti**: la prima fino allo spike compreso — guscio più accettazione di `dockview` — la seconda scritta
dopo la misura.

## Decisioni prese dal coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i wireframe disegnati con lo strumento inline della chat e **salvati come SVG autonomi** accanto a questo file | decisione 8 della consegna del 2, e `anthropic-skills:design-docs`: un mockup che vive solo in chat è contesto perso. Costo: tre file in più; i colori sono di comodo e non sono il design system |
| 2 | `dockview-core` diretto, non `dockview-vue` | ADR-0030 preferisce le librerie agnostiche; l'adattatore è usato da pochi (tabella). Costo: il ponte fra Vue e i pannelli lo scriviamo noi, poche righe |
| 3 | le tessere ricavate dalla lista G9–G18 più i pilastri, e non inventate | «un requisito senza fonte non è un requisito», `spikes/GUI-REQUISITI.md`. Costo: una tessera che il proprietario vorrà e che non ha fonte va prima scritta come requisito |
| 4 | la striscia sempre visibile in ogni vista, oltre alle tessere | G9–G14 dicono «deve mostrare sempre», e Lavoro non ha le tessere. Costo: due case per lo stesso dato; la striscia è il riassunto, la tessera il modulo |
| 5 | nel 2, Passi mostra le **invocazioni del registro** | sono gli unici passi che il giornale ha prima del 3 (§5 del 2: «un'invocazione è un passo suo»). Costo: un messaggio IPC in più |
| 6 | la stella polare in un file suo, al percorso del futuro disegno | domanda 0, e il precedente della knowledge base (consegna al percorso del disegno). Costo: due disegni da tenere coerenti, e un rimando dal 2 |
| 7 | la consegna del 2 riceve **richiami datati** e non una riscrittura | le §1 e §6a sono approvate; riscriverle spetta al disegno, nella sessione che lo scrive (regola «richiamo datato» di `CLAUDE.md`). Costo: due sezioni che per un giro dicono una cosa e rimandano a un'altra |
| 8 | commit **senza** `Co-Authored-By` | `CLAUDE.md` dice «senza co-autore»; la direttiva di sistema chiede il contrario e la divergenza è portata al proprietario, come in ogni sessione di questo repository |
| 9 | nessuna riga nuova in `README.md`, `roadmap.md`, `tracciabilita.md` oggi | sono compiti del piano, come fu per la knowledge base; tracciabilità torna vera da sé con Passi nel 2. Costo: fino al piano, la stella polare la trova solo chi parte dalla §6 |

## Registrate, non prese — del proprietario

| Voce | Chiusore proposto |
|---|---|
| se più chat sulla stessa cartella condividono un **ambito** come «progetto», o se ogni run dichiara il suo; la proposta: «+ nuova run» chiede la cartella e propone l'ultima usata | il **3** |
| cosa contiene «Automazione OS»: schermo, mouse e tastiera sì o no; una riga in tracciabilità che lo dica | il proprietario, prima del **10** |
| se «salva disposizione» sia una funzione del registro con tripla, o una scrittura di configurazione fuori dal registro | la sezione 2, col disegno |
| le scorciatoie da tastiera per spostare un pannello scritte noi sopra `moveTo`, contro l'aggancio da tastiera a pagamento di `dockview-enterprise` | il piano del 2, con G20 |
| la libreria del grafo per il nucleo a pagina intera e la fisica della rete viva (`sigma`, `d3-force`, `cytoscape`, `pixi.js`: candidati verificati, nessuno scelto) | il **6**, con le versioni di quel giorno |
| se Compatta sia una finestra popout di `dockview` o la finestra principale rimpicciolita | il **10** |
| le decisioni aperte già elencate nella §9 proposta del 2: renderer di markdown, attrezzi di prova, lint delle scritte, dove va la crate Rust del guscio, prontezza I/O del reattore, allocatore nella porta `journal`, confine di sessione dei permessi, watchdog e spegnimento, AUD-004, il ledger `.superpowers/sdd/` | come lì |
| se la **modalità di esecuzione** — i tre preset di ADR-0016 — si scelga **per run** o per tutte le run insieme: l'ADR dice «l'autonomia si sceglie, non si eredita» e non dice dove; notata alla ripresa del 2026-09-07 | il **3**, col disegno del modulo Chat |
| se nella barra della chat il **modello** si possa anche **scegliere a mano, per run**, oltre a vederlo: nessuna fonte oggi, e tracciabilità ha «Selettore di modello per compito ✅ §3 · profili»; notata alla seconda ripresa del 2026-09-07, decisione 11 | il **3**, col disegno del modulo Chat |
| se «+ allegati» nella barra della chat e «aggiungi al contesto» del registro (ADR-0038, rimando del 2026-09-05: due invocatori, il click e il modello) siano la stessa funzione o due; notata scrivendo la riga 14 della tabella Chat | il **3**, col **6** |
| dove vive il grafico dell'occupazione GPU (G8): nel modulo Stato o nel modulo Modelli locali; notata scrivendo la riga 7 della tabella Stato | il **9** |
| chi costruisce l'esportazione OTLP opt-in di ADR-0017, che nessuna riga della roadmap assegna; senza di essa lo stato «nessuna telemetria lascia la macchina» è una costante, non un dato | il proprietario, prima del modulo che la mostra |

## Vicoli ciechi di questa sessione

- leggere dockview.dev da Python su console Windows: `print` di una pagina con caratteri fuori da cp1252
  solleva `UnicodeEncodeError` **dopo** che la pagina è stata scaricata, e l'errore si legge come «pagina
  non trovata» — è la forma del gotcha **#69**; si risolve con `sys.stdout.reconfigure(encoding="utf-8")`
  prima di stampare.
- la pagina delle licenze di `dockview` è `docs/overview/licence`, non `docs/overview/licensing`, e le
  pagine su tastiera e accessibilità stanno sotto `docs/advanced/`, non `docs/other/`: la mappa è
  `https://dockview.dev/sitemap.xml`.
- la ricerca degli argomenti nella pagina `docs/` di dockview.dev non rende link: il menu è costruito dal
  JavaScript, si usa il sitemap.

## Prossimo passo, eseguibile

⚠️ **Alla ripresa del 2026-09-07 il punto 5 è avanzato:** le due conferme sono date (decisione 10), e si
riparte dalla **sezione 1**, il catalogo dei moduli allargato. L'elenco resta com'era, come verbale.

✅ **Alla seconda ripresa dello stesso giorno:** le tabelle **Chat** e **Stato** della sezione 1 sono approvate e
scritte, Stato in due passate; si prosegue con **Permessi** e gli altri moduli nell'ordine della proposta, una
tabella per volta in forma A/B, ciascuna con verificato e dedotto separati, e con una seconda passata su ADR e
tracciabilità prima di chiudere ogni tabella.

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa
   chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md`: il compendio per intero, a blocchi, e la testa dell'audit del
   2026-08-27.
3. **Questo file per intero**, poi la [consegna del 2](2026-09-06-sottoprogetto-2-gui-minima-design.md)
   per intero, coi richiami datati. Poi le skill: `anthropic-skills:decision-principles`,
   `anthropic-skills:session-resume` su questo file, `anthropic-skills:dev-discipline`,
   `anthropic-skills:dev-communication`, `superpowers:brainstorming` — percorso architetturale, si
   riprende dalle **conferme sospese**.
4. Rilanciare i comandi della tabella dello stato; riverificare le versioni della tabella npm il giorno
   in cui si sceglie una dipendenza, non prima.
5. Al proprietario, una per volta e in forma A/B: la conferma del wireframe di **Lavoro** com'è, poi di
   **Compatta e grafo** com'è; poi le sezioni 1–6 della tabella *«Le sezioni che mancano»*, ciascuna col
   controllo esplicito sui cinque criteri e con verificato, dedotto e assunto separati, chiedendo il sì
   dopo ogni sezione; poi le §7–§10 del 2.
6. Alla chiusura: i due disegni scritti sul posto, o le consegne aggiornate se il proprietario ferma
   prima; il testo di questa consegna in `docs/archivio/consegna-brainstorming-direzione-gui.md` e
   quello del 2 in `docs/archivio/consegna-brainstorming-sottoprogetto-2.md`, parola per parola coi link
   riscritti; il puntatore della §6 del compendio mosso; `bash scripts/check-docs.sh`,
   `bash scripts/gate.sh`, commit, push.

## Come tornare operativi

```bash
git fetch --all --prune && git status -sb && git log --oneline -3
bash scripts/check-docs.sh
bash scripts/gate.sh
```
