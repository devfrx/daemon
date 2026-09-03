# Tracciabilità — mappa funzionale → dove vive

Ogni funzionalità della mappa funzionale originale, con la sua sede nel progetto.
**Da aggiornare a ogni sotto-progetto chiuso.**

## Come leggere

| Stato | Significato |
|---|---|
| ✅ | **meccanismo deciso** nel kernel: la funzionalità ha già le sue fondamenta |
| 🔶 | meccanismo deciso, **politica o implementazione** nel sotto-progetto indicato |
| 📋 | **pianificata**: sotto-progetto assegnato, non ancora progettata |
| ⚠️ | **lacuna**: nessuna sede assegnata — vedi §Lacune |
| ❌ | **esclusa** per decisione esplicita |

**Il kernel non implementa nessuna funzionalità utente.** Fornisce i meccanismi su cui
tutte poggiano: ✅ significa «le fondamenta esistono», non «è fatto».

> ⛔ **QUESTA TABELLA RISPONDE A UNA DOMANDA SOLA, ed è «dove vive».**
> **Non** risponde a *«di quale meccanismo di kernel ha bisogno questa funzionalità, e la
> spec lo nomina?»*. In particolare `📋` significa **«sotto-progetto assegnato»**, non «non
> richiede un meccanismo di kernel»: sono due cose diverse, e chi legge assume la seconda.
>
> **È la crepa da cui sono uscite le sette voci della riapertura del 2026-08-07**, tre delle
> quali di classe **B**, cioè non retrofittabili. Il rimedio non è riscrivere la legenda —
> gotcha **#27** — è **rileggere con un'altra domanda**, e questo riquadro esiste per
> ricordare che la domanda che manca è quella.
>
> ⛔ **RICHIAMO DEL 2026-08-27, finding AUD-041 — qui c'era una nota del 2026-08-08 che metteva
> a verbale una MISURA SBAGLIATA.** Diceva: *«la §12 del compendio rimandava già al riquadro in
> testa per questo avvertimento, e **il riquadro non lo conteneva**»*. Lo conteneva: il riquadro
> qui sotto, datato **2026-08-07**, **lo portava** parola per parola nella sostanza, a tredici
> righe di distanza — ⚠️ **al passato perché è questa stessa passata a togliercelo**, e scriverlo
> al presente sarebbe stato il gotcha **#78** commesso dentro il rimedio: *una frase che descrive
> lo stato di un altro artefatto che qualcuno sta toccando nello stesso compito*. Il testo che vi
> stava lo dice `git show HEAD:docs/tracciabilita.md`, e un comando non marcisce.
> Chi corresse cercò dove il rimando della §12 mandava — *in testa* — e non lesse il
> riquadro successivo: è il gotcha **#48** nella forma canonica, il banco che sbaglia **verso
> l'attesa**, e l'attesa era che mancasse.
>
> ✅ **E il rimedio applicato allora fu AGGIUNGERE una seconda copia sopra la prima**, cioè creare
> una seconda casa della stessa regola nel documento la cui unica difesa dichiarata è *rileggere
> con un'altra domanda*. Due formulazioni della stessa regola nello stesso posto sono il gotcha
> **#29** in attesa — *la più corta viene letta al posto di quella giusta* — e qui la più corta
> era anche la prima che si incontrava. ⛔ **Ora l'avvertimento vive in questo riquadro e in
> nessun altro:** quello sotto tiene il proprio **stato** e non ripete più la regola.

> ⚠️ **Lo stato della riapertura del 2026-08-07, e non più la regola:** l'avvertimento che questo
> riquadro portava ripeteva quello sopra, ed è **tolto** il 2026-08-27 — finding **AUD-041**, con
> il racconto nel richiamo qui sopra, in una casa sola.
>
> Rileggere la tabella con la seconda domanda ha **riaperto la spec del sotto-progetto 1 su sette
> voci**, ora **tutte chiuse**: elenco, classe e ordine in
> [HANDOFF](HANDOFF.md#prima-cosa-da-fare).
>
> ✅ **F4 chiusa il 2026-08-08**, ed era l'ultima. Le due righe che vi pendevano hanno ora
> un meccanismo collocato **e** una porta: *Scheduling* e *File watching* entrano entrambe
> da `reactor` — la prima è già coperta, la seconda è **dichiarata** con implementazione
> scaglionata (§0.4.3).
>
> ✅ **Riletta il 2026-09-03 con la seconda domanda** — *«quale meccanismo del kernel le serve, e il
> codice lo dà oggi o è scaglionato?»* — contro la §0.4 della spec e gli inneschi della §8. Le
> righe cambiate, con la ragione di ciascuna, stanno nella §7.4 del
> [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md): una
> casa sola. ⚠️ **Le righe per stato non stanno qui**, le conta il comando:
> `for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done`
>
> ✅ **Sotto-progetto 1 chiuso il 2026-09-03 contro la §0.7**: la tabella si riaggiorna a ogni
> sotto-progetto chiuso, come dice la riga sotto il titolo.
>
> ✅ **Aggiornata il 2026-09-03 con le righe del riconoscimento gesti**, dalla §5.2 del [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) approvato dal proprietario — fuori da una chiusura di sotto-progetto, e per questo detto: la sezione 6 diventa «Voce e gesti», e la riga del registro delle funzioni entra nella sezione 2 accanto ai comandi rapidi.

---

## 1. Modelli e risorse

| Funzionalità | | Sede |
|---|---|---|
| Budget VRAM esplicito | ✅ | §2 · ADR-0005 |
| Modalità VRAM per profilo di carico | ✅ | §2 · profili di risorsa |
| Eviction e scarico modelli | ✅ | §2 · revoca della concessione |
| Swap coordinato LLM/embedding ↔ TRELLIS2 | ✅ | §2 · ADR-0006 policy LOCALE |
| Policy differenziata remoto vs locale | ✅ | §2 · ADR-0006 |
| Scarico per inattività (TTL) | 🔶 | meccanismo §2 · politica → Gestione modelli locali |
| Caricamento su richiesta e pre-caricamento | 🔶 | meccanismo §2 · politica → Gestione modelli locali |
| Tetto ai modelli residenti | ✅ | §2 · ammissione |
| Stima di fit prima del caricamento | ✅ | §2 · riserva dichiarata, picco misurato |
| Ecosistema dei worker ML | ✅ | ADR-0028 · Python, con i costi dichiarati |
| Routing locale/remoto con fallback | ✅ | §3 · ADR-0012 |
| Fallback a catena tra modelli | ✅ | §3 · ADR-0012 |
| Preferenze di provider (OpenRouter) | ✅ | §3 · vincoli della richiesta |
| Routing per compito/costo | ✅ | §3 · politica di routing |
| Structured output e constrained decoding | 🔶 | sensore §5 (ADR-0013) + adattatori §3 · provider reale → Conversazione |
| Contabilità token e costi | ✅ | §3 · ADR-0011 |
| Avvisi e tetti di spesa | ✅ | §3 + §4 · V8 |
| Selettore di modello per compito | ✅ | §3 · profili |
| Parametri di generazione configurabili | ✅ | §3 · record di routing |
| Catalogo e download modelli locali | 📋 | Gestione modelli locali |
| Indicatore di stato modello | 📋 | GUI |

## 2. Conversazione

| Funzionalità | | Sede |
|---|---|---|
| UI/UX della chat | 📋 | GUI |
| Streaming delle risposte | 🔶 | trasporto §3 · resa nel processo `gui` → GUI minima |
| Sessioni multiple | 🔶 | gerarchia §3 (ADR-0011) · politica → Conversazione |
| Fork e branching | 📋 | Conversazione — il giornale lo consente |
| Modifica e rigenerazione | 📋 | Conversazione |
| Ricerca nello storico | 📋 | Conversazione |
| System prompt, personas e profili | 🔶 | guide §5 · politica → Conversazione |
| Memoria persistente | 🔶 | stato durevole §4 · politica → Conoscenza |
| Gestione del contesto e compattazione | 🔶 | giornale §4 · ricomposizione della proiezione (ADR-0008, ADR-0010) → Conversazione |
| Indicatore di riempimento contesto | 🔶 | misura §7 · proiezione da misurare → Conversazione |
| Artifacts/canvas | 📋 | GUI |
| Preview renderizzate | 📋 | GUI |
| Allegati in chat | 📋 | Conversazione (+ etichettatura I6) |
| Comandi rapidi e slash-command | 📋 | GUI |
| Registro delle funzioni del programma | 🔶 | registro di [ADR-0038](adr/0038-registro-delle-funzioni-del-programma.md), nella forma dei registri §5 (ADR-0009) · registrazione, invocazione, permesso come tripla, giornale · implementazione → GUI minima, col primo invocatore |
| Template e prompt salvati | 📋 | Conversazione |
| Esportazione conversazioni | 📋 | Conversazione |

## 3. Conoscenza

| Funzionalità | | Sede |
|---|---|---|
| RAG e indicizzazione | 📋 | Conoscenza |
| Modalità piena vs recupero | 📋 | Conoscenza |
| Embedding e reranking | 🔶 | risorsa §2 + gateway §3 · politica → Conoscenza |
| Ricerca ibrida | 📋 | Conoscenza |
| Ingest di documenti (PDF, OCR) | 📋 | Conoscenza (+ I6) |
| Chunking configurabile | 📋 | Conoscenza |
| Collezioni e knowledge base | 📋 | Conoscenza |
| File watching e awareness del progetto | 🔶 | trigger anello 3 §5 · politica → Conoscenza |
| Multi-repo/multi-progetto | 📋 | Conoscenza |
| Mappa del progetto | 📋 | Conoscenza |
| Ricerca web integrata | 📋 | Conoscenza (+ I6) |
| Loop di deep research | 🔶 | anelli §5 + run durevoli §4 · politica → Conoscenza |
| Report strutturato con citazioni | 📋 | Conoscenza |
| Citazioni e verificabilità | 🔶 | sensore §5 · politica → Conoscenza |
| Pianificazione della ricerca rivedibile | 📋 | Conoscenza |
| Gestione del decadimento delle fonti | 📋 | Conoscenza |

## 4. Agenti

| Funzionalità | | Sede |
|---|---|---|
| Orchestrazione e sub-agenti | 🔶 | sub-run §4 · proiezione ristretta (ADR-0008) → Conversazione |
| Planning e decomposizione dei task | 🔶 | piano nello stato durevole §4 · politica → Agenti |
| Modalità piano vs esecuzione | 🔶 | preset §6 · politica → Agenti |
| Tool calling | 🔶 | schema §3 + permessi §6 · mediatore completo → Agenti |
| MCP | 🔶 | ADR-0003 · ciclo di approvazione MCP §6 (ADR-0015) → Agenti |
| Skills | 🔶 | guide §5 (ADR-0003) · registro delle guide → sede da assegnare |
| HITL: approvazioni | 🔶 | §6 · ADR-0016 · ciclo di approvazione → Agenti |
| HITL: interruzione e steering | 🔶 | `AttesaUmano` §4 · politica → Agenti |
| Domande di chiarimento | 📋 | Agenti |
| Checkpoint e rollback | ✅ | giornale §4 + §10 · ADR-0024 |
| Run persistenti, ripresa e cancellazione | ✅ | §4 · ADR-0007 |
| Task in background | ✅ | §4 + ADR-0004 |
| Scheduling | ✅ | trigger anello 3 §5 |
| Coda e priorità delle run | 🔶 | corsie §2 · coda generica → Agenti |
| Replay dei trace | 🔶 | giornale §4 · proiezione trace §7 (ADR-0017) → GUI minima |
| Valutazione degli agenti | 📋 | Agenti — esplicitamente **fuori** dal kernel (§8) |
| Regole e vincoli di progetto | 🔶 | guide §5 · registro delle guide → sede da assegnare |
| Agenti in parallelo isolati | 🔶 | sub-run §4 · isolamento su disco → Coding |
| Limiti di autonomia configurabili | ✅ | §4 · V8 |

## 5. Coding

| Funzionalità | | Sede |
|---|---|---|
| Sandboxing ed esecuzione | 🔶 | permessi §6 + §10 · ADR-0025 — confinamento reale → Coding |
| Edit dei file e diff review | 📋 | Coding |
| Terminale integrato | 📋 | Coding |
| LSP e analisi statica | 🔶 | sensori §5 · implementazione → Coding |
| Git e gestione branch | 📋 | Coding |
| Isolamento per branch di lavoro | 📋 | Coding |
| Undo/checkpoint del filesystem | ✅ | §10 · ADR-0024 — ambiti dichiarati |
| Esecuzione test | 🔶 | sensori §5 · implementazione → Coding |
| Modalità di edit selezionabili | 📋 | Coding |
| Anteprima applicazioni | 📋 | Coding + GUI |
| Approvazione comandi distruttivi | ✅ | §6 + effetti `irripetibili` §4 |

## 6. Voce e gesti

⚠️ **RICHIAMO DEL 2026-09-03:** la sezione si chiamava «Voce»; con [ADR-0039](adr/0039-telecamera-come-sorgente-di-percezione.md) il pilastro si legge «voce e gesti» — rimando datato in testa ad ADR-0001 — e le righe dei gesti stanno qui, con sede **Gesti**, il sotto-progetto 12.

| Funzionalità | | Sede |
|---|---|---|
| Wake word | 📋 | Voce |
| Push-to-talk e dettatura | 📋 | Voce + L3 |
| VAD (rilevazione della voce) | 📋 | Voce |
| STT (voce → testo) | 📋 | Voce |
| TTS (testo → voce) | 📋 | Voce |
| Barge-in e interruzione | 📋 | Voce |
| Interruzione attivabile/disattivabile | 📋 | Voce |
| Gestione dei turni | 📋 | Voce |
| Conversazione continua | 📋 | Voce |
| Stati di ascolto/pensiero/parlato | 📋 | Voce + GUI |
| Ducking dell'audio | 📋 | Voce + L3 |
| Controlli di privacy del microfono | 📋 | Voce + L3 |
| Cancellazione eco / anti auto-attivazione | 📋 | Voce |
| Lettura vocale delle notifiche | 📋 | Voce |
| Convivenza pipeline audio ↔ job GPU | ✅ | §2 · quota sottratta, corsie (ADR-0005) |
| Telecamera come sorgente di percezione always-on | 🔶 | porta `process` §6.10 (ADR-0039) · il worker, le due specie di evento, il campo di degrado · implementazione → Gesti |
| Tracciamento delle mani — 21 punti, stato continuo | 📋 | Gesti |
| Gesti di comando | 📋 | Gesti |
| Manipolazione di pannelli e menu con le mani | 📋 | GUI + Gesti |
| Cattura con un gesto | 📋 | Gesti — la destinazione la decide il brainstorming della knowledge base (decisione 7 del disegno) |
| Indicatore di telecamera accesa | 📋 | GUI |

## 7. Multimodalità e generazione

| Funzionalità | | Sede |
|---|---|---|
| Input immagini e vision | 🔶 | gateway §3 · politica → Conversazione |
| Screenshot e comprensione dello schermo | 📋 | L3 + Conversazione |
| Generazione immagini | 📋 | Generazione asset |
| TRELLIS2 single-image | 📋 | Generazione asset (+ profili §2, SP-1) |
| TRELLIS2 multiview | 📋 | Generazione asset |
| Parametri di qualità 3D configurabili | ✅ | §2 · profili di risorsa (la curva) |
| Post-processing mesh | 📋 | Generazione asset |
| Export mesh (GLB/OBJ/PLY) | 📋 | Generazione asset |
| Viewer 3D integrato | 📋 | GUI |
| Libreria degli asset generati | 📋 | Generazione asset |
| Cronologia e riproducibilità | ✅ | giornale §4 + record di routing §3 |
| Coda dei job di generazione | ✅ | code §2 + run §4 |
| Progress e notifiche per job lunghi | 🔶 | §7 + V9 · notifica all'utente → GUI minima |
| Rimozione dello sfondo/preparazione input | 📋 | Generazione asset |

## 8. Sistema

| Funzionalità | | Sede |
|---|---|---|
| Permessi e sandbox policy | 🔶 | §6 + §10 · ADR-0025 — confinamento reale → Coding |
| Difese da prompt injection | ✅ | §6 · ADR-0014 |
| Difesa da tool poisoning | 🔶 | §6 · ADR-0015 · ciclo di approvazione → Agenti |
| Gestione segreti e credenziali | 🔶 | §10 · ADR-0023 — gestore unico · implementazione → Conversazione |
| Storage e cifratura a riposo | 🔶 | layout §10 (ADR-0022) già rispettato · cifratura reale (ADR-0023) → sede da assegnare |
| Backup ed export dei dati | 🔶 | §10 · ADR-0022 — solo l'irriproducibile · implementazione → Backup e ripristino |
| Osservabilità e tracing locale | 🔶 | giornale §4 · proiezione trace §7 (ADR-0017) → GUI minima |
| Logging | ✅ | §7 |
| Hotkey globale, tray e clipboard | 📋 | L3 |
| Automazione OS | 📋 | L3 |
| Notifiche | 🔶 | V9 §4 · implementazione → L3 |
| Avvio automatico e daemon in background | 🔶 | ADR-0004 · implementazione → L3 |
| Packaging e aggiornamenti | 📋 | L3 |
| Estensibilità e plugin | ✅ | ADR-0003 |
| Accessibilità | 📋 | GUI |
| Internazionalizzazione (i18n) | 📋 | GUI + Voce |
| Comportamento offline | ✅ | §7 · ADR-0019 |
| Impostazioni e profili di configurazione | 🔶 | profili §2/§3 · pannello → GUI |

---

## Funzionalità emerse dal confronto con prodotti comparabili

### Coesistenza sulla stessa GPU

| Funzionalità | | Sede |
|---|---|---|
| Semaforo unico delle risorse GPU | ✅ | §2 · ADR-0005 |
| Precedenza della voce sui job pesanti | ✅ | §2 · quota **sottratta** |
| Avviso di conflitto e stima d'attesa | ✅ | §2 · `InCoda` |
| Passaggio suggerito a OpenRouter durante i render | ✅ | §2 · ADR-0006, transizione esplicita |
| Warm-up e cold-start visibili | ✅ | Q8 |
| Budget di VRAM riservata all'audio | ✅ | §2 · ADR-0005 |

### Ciclo di vita e affidabilità delle run

| Funzionalità | | Sede |
|---|---|---|
| Notifica «l'agente ha bisogno di te» | 🔶 | V9 §4 · notifica all'utente → GUI minima |
| Determinismo/replay riproducibile | ✅ | §4 + §8 (seed) |
| Hook sul ciclo di vita | 🔶 | trigger e anelli §5 · politica → Agenti |
| Classificatore di sicurezza delle azioni | 🔶 | si realizza come **sensore** §5 · politica → Agenti |
| Dataset dai fallimenti | ✅ | §7 promozione + §8 |
| Analisi dei costi per run e per sub-agente | ✅ | §3 · ADR-0011 |

### Conversazione, conoscenza e UX

| Funzionalità | | Sede |
|---|---|---|
| RAG «agentico» su filesystem | 📋 | Conoscenza |
| Sincronizzazione automatica delle cartelle | 🔶 | trigger §5 · politica → Conoscenza |
| Backup della KB indipendente dall'app | 🔶 | §10 · ADR-0022 — documenti nel backup, indice ricostruito · implementazione → Backup e ripristino |
| Memoria selettiva ed episodica | 📋 | Conoscenza |
| Gestione delle contraddizioni nella memoria | 📋 | Conoscenza |
| Full-context mode per singolo allegato | 📋 | Conoscenza |

### Voce e interazione

| Funzionalità | | Sede |
|---|---|---|
| Condivisione visiva in push-to-talk | 📋 | Voce + L3 |
| Instradamento dei comandi semplici in locale | 📋 | Voce |
| Timer, sveglie e annunci | 📋 | Voce |
| Doppia wake word / doppia lingua | 📋 | Voce |
| Metriche di qualità voce | 🔶 | metriche §7 · definizione → Voce |
| Autonomia proattiva a corsie di priorità | 📋 | Voce + Agenti |

### Sistema, sicurezza e governance

| Funzionalità | | Sede |
|---|---|---|
| Zero-Data-Retention selettivo | ✅ | §3 ADR-0012 + §6 ADR-0016 (escalation) |
| Fatturazione a stream interrotto | ✅ | §3 · ADR-0011 |
| Politica di routing come oggetto versionato | ✅ | §3 · ADR-0011 |
| Canary per esfiltrazione dati | 🔶 | §6 · ADR-0016 · canary → Conversazione |
| Modalità di permessi a più livelli | 🔶 | preset §6 · implementazione → Agenti |
| Marketplace/registry di estensioni | ❌ | **escluso** da ADR-0003 (nessun plugin nativo). Resta la scoperta di server MCP → 📋 GUI |
| Overlay/finestra fluttuante sempre in primo piano | 📋 | GUI + L3 |
| Telemetria locale e «no telemetry» garantito | ✅ | §7 · ADR-0017, V25 |

### Generazione asset e pipeline creativa

| Funzionalità | | Sede |
|---|---|---|
| Workflow a nodi/pipeline riusabili | 📋 | Generazione asset |
| Batch e generazione a lotti | 🔶 | code §2/§4 · politica → Generazione asset |
| Validazione della mesh prima dell'export | 🔶 | sensore §5 · ADR-0009 · sensore reale → Generazione asset |
| Confronto varianti | 📋 | Generazione asset + GUI |
| Rendering di anteprima video dell'asset | 📋 | Generazione asset |

### Ergonomia e affidabilità

| Funzionalità | | Sede |
|---|---|---|
| Prompt history ricercabile | 📋 | Conversazione |
| Auto-retry su errori transitori | ✅ | §3 · V17 |
| Modalità «solo lettura»/dry-run | 🔶 | preset §6 + classi di effetto §4 · politica → Agenti |
| Degrado esplicito quando manca la rete | ✅ | §7 · ADR-0019 |

---

## Lacune — chiuse dalla §10

Trovate da questo esercizio di tracciabilità il 2026-08-06, tutte in L0, tutte chiuse
dalla §10 della spec del kernel lo stesso giorno.

| # | Lacuna | Chiusa da |
|---|---|---|
| L-1 | Undo/checkpoint del filesystem | [ADR-0024](adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) — ambiti dichiarati, write-ahead sui file |
| L-2 | Storage e cifratura a riposo | [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) + [ADR-0023](adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md) |
| L-3 | Backup ed export dei dati | [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) — solo l'irriproducibile |
| L-4 | Gestore dei segreti | [ADR-0023](adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md) — punto unico di lettura |
| L-5 | Confinamento reale dell'esecuzione | [ADR-0025](adr/0025-confinamento-a-livelli.md) — quattro livelli, default 2 |

**Nessuna lacuna aperta.** Se un nuovo esercizio di tracciabilità ne trova altre, si
aggiungono qui con lo stesso formato.
