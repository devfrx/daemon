# Riconoscimento gesti dalla telecamera — la consegna del brainstorming

⚠️ **QUESTO FILE È LA CONSEGNA DEL BRAINSTORMING, NON IL DISEGNO.** Il brainstorming è **in
corso**: le decisioni prese in chat stanno qui, e **lo stato di ogni sezione del disegno sta nella
tabella della §2**, in una casa sola. Chi riprende legge questo file **per intero** dopo la lettura
obbligatoria di [`CLAUDE.md`](../../../CLAUDE.md) e del [compendio](../../COMPENDIO.md), e continua
dalla prima sezione che la §2 non dà per approvata.
Quando il disegno sarà scritto, sarà scritto **sul posto, a questo stesso percorso**, come fu per
il [disegno della chiusura](2026-09-02-sottoprogetto-1-chiusura-design.md): il puntatore della §6
del compendio non cambia casa. **Il prossimo passo lo dice la §6 del compendio**, in un posto solo.

📌 **Metodo.** Ogni affermazione porta la sua specie: **verificata** (letta nel sorgente o in una
fonte primaria, oggi), **dedotta**, o **assunta**. Le tre sono separate nella §5. Ogni cifra porta
accanto il comando che la rifà, e le fonti portano la data.

---

## 0. Lo stato del repository alla consegna — 2026-09-03, coi comandi

| | |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb` |
| Il sotto-progetto 1 | **mergiato su `main` il 2026-09-03**: commit di merge `e77329a` (due genitori, albero identico alla testa del ramo), poi `a38d898` che ha portato su `main` il punto di ripresa di `HANDOFF.md` e la riga `branch` di `AVVIO-CHAT.md`. Il ramo `spec/sottoprogetto-1-kernel` è **cancellato**, in locale e su `origin` |
| Albero | pulito, nessuno stash, nessuna operazione a metà |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` sull'albero mergiato, **prima** del push; `bash scripts/check-docs.sh` → `OK`. Si **rilanciano**, non si citano |
| Codice di prodotto | **non toccato** da questo brainstorming: nessun file in `crates/`, `scripts/`, `Cargo.lock` |
| Questo file | entra col commit di consegna, insieme al puntatore nuovo della §6 del compendio |

---

## 1. Le regole di questo brainstorming, decise dal proprietario

| Regola | Da dove viene |
|---|---|
| La strada è quella del repo: **brainstorming → disegno scritto → piano → esecuzione**, come ogni traguardo | scelta 5 del 2026-09-02, in testa al disegno della chiusura |
| `anthropic-skills:decision-map` **non si usa**: proposta e motivata il 2026-09-03, il proprietario non ha obiettato. Tre ragioni: il perimetro è già piccolo e noto (le quattro decisioni di kernel in testa al disegno della chiusura), quella skill mette le decisioni aperte in GitHub Issues che sarebbero una **seconda casa** (gotcha #68), e da questa macchina non c'è né `gh` né il connettore GitHub | chat del 2026-09-03 |
| Percorso **architetturale** di `superpowers:brainstorming`: domande una per volta, approcci, disegno a sezioni approvate una per una | classificazione dichiarata in chat |
| **Ogni decisione si controlla contro i cinque criteri di `anthropic-skills:decision-principles`**, e il proprietario ha chiesto il controllo **esplicito** («sicuro che rispetti tutti i principi?»). Ha poi dato un'**accettazione condizionata**: *«decidi e progettiamo sempre secondo questa skill, se regge contro i principi procediamo»*. Vale finché regge: se una scelta li viola, ci si **ferma e lo si dice**, non si tratta come delega in bianco | chat del 2026-09-03 |
| Brainstorming in **una** sessione, disegno nella **successiva**, con la consegna in un file **tracciato** | scelta del proprietario del 2026-09-02, scartata *«scrivere il disegno nella stessa sessione»* nel disegno della chiusura |
| Codice in inglese, documenti in italiano (§1.0 della spec); nessun numero senza comando; nessuna fonte senza data | `CLAUDE.md` |

---

## 2. Le decisioni prese in chat il 2026-09-03

⛔ **Sono del proprietario e non si riaprono senza di lui.** La domanda è riportata com'era, con le
opzioni, perché chi riprende sappia che cosa è stato scartato e non solo che cosa è stato scelto.

| # | La domanda | La risposta | Che cosa ne segue |
|---|---|---|---|
| 1 | Nel primo giro la telecamera è **A** solo una **sorgente di eventi** (un gesto riconosciuto = un evento, come la wake word; i fotogrammi non escono dal worker), o **B** anche un **occhio** per l'agente (immagini nel contesto di un modello: contenuto non fidato, gateway)? | **A**, *«ma conta che deve avere del potenziale»*: predisposta a **menu virtuali** e a **spostare finestre direttamente con le mani** (pinch e simili). Su B: con un gesto specifico si vorrebbe fare una **cattura** legata al contesto dell'agente, ma *«nascono un sacco di problemi: che contesto? conversazionale? un posto nella knowledge base?»* | i fotogrammi **non escono mai** dal worker. Lo **stato continuo della mano** è un evento necessario per la manipolazione diretta, non un lusso. La cattura resta un **caso d'uso**, con la destinazione **aperta** (§8) |
| 2 | Le finestre e i menu da muovere con le mani sono **A** quelle **del programma** (la sua GUI: tutta presentazione, nessuna porta nuova) o **B** anche quelle **del sistema operativo** (effetto OS via modulo di piattaforma, azione con permesso ADR-0016, una **settima famiglia di porte** dove oggi sono sei per decisione)? | **A** | nessuna porta nuova nel kernel. B è **esclusa** da questo brainstorming: se mai, nel sotto-progetto 10 con un ADR suo |
| 3 | Che cosa significa *«self-use dell'agente sulle funzioni del programma»*? **A** l'agente invoca **le stesse azioni** che un gesto invoca; il gesto è solo uno degli invocatori, come la voce o il click. **B** l'agente usa la telecamera per sé | **«A mutuale»**: *l'agente può richiamare nativamente **tutte** le funzioni esistenti del programma; una **fetta** di quelle (tutte o solo alcune, si deciderà) le uso anch'io gestualmente* | **un registro unico** delle funzioni del programma, **molti invocatori**, **lo stesso permesso** per tutti. Nessuna logica «solo per gesti». È un ADR nuovo (§7, sezione 3) |
| 4 | Approccio **1**, sorgente di percezione **sotto il core**, o **2**, tracciamento **dentro la GUI**? | **1**, dopo la sfida *«sicuro che rispetti tutti i principi?»* e le **tre correzioni** della §4, sotto accettazione condizionata | la §4 è la forma vincolante |
| 5 | La **sezione 1** del disegno, il perimetro | ✅ **approvata il 2026-09-03**, ripresentata nella sessione successiva. ⚠️ Prima diceva *«presentata, NON approvata»*: il proprietario aveva aperto questa consegna invece di rispondere | il testo approvato è in §6, con la clausola aggiunta rileggendolo |

**Le premesse dette dal proprietario**, che il disegno deve onorare: l'agente è **dormiente e risvegliabile con la wake word**; i gesti sono *«stile Jarvis»*; vuole *«basarmi sulla reale architettura del progetto ed integrare le cose in modo professionale, seguendo prima lo stato dell'arte e i principi di decision-principles»*; e ha *«molti buchi tecnici e soprattutto logici»* in testa, che la §3 scioglie coi documenti del repo.

---

## 3. I «buchi logici», sciolti coi documenti del repo

**Il punto che scioglie quasi tutti: il core non dorme mai. Dorme la run.** Il core vive a lungo, anche senza GUI (ADR-0004); i worker always-on vivono sotto di lui. La wake word non «sveglia il programma»: **apre una run** (ADR-0011, corollario). Un gesto fa lo stesso se è un gesto di **comando**; un gesto di **manipolazione** (un pinch che sposta un pannello) non apre nulla: è presentazione, lo consuma la GUI, e non tocca mai il giornale, come i frammenti audio.

| Buco | Che cosa dice già l'architettura | Che cosa resta da decidere |
|---|---|---|
| «Se l'agente dorme, quando succede?» | il core è sempre sveglio; un evento di comando apre una run | quali gesti sono **comando** e quali **manipolazione**: la lista è del disegno |
| «La foto va in che contesto?» | una cattura è un **artefatto**: file su disco, nel giornale solo il **riferimento** (ADR-0018, ADR-0022); entra nel contesto come **proiezione** (ADR-0008), nella run aperta dal gesto o già in corso | se vada **anche** nella knowledge base: è il **brainstorming 2**, dipendenza dichiarata |
| «Una foto può dare ordini?» | no: è contenuto **non fidato**, informa e non autorizza (ADR-0014, I6). Un gesto non concede permessi (ADR-0016): un'azione invocata a gesti chiede lo stesso permesso che chiederebbe da tastiera | niente |
| «Chi tiene i fotogrammi?» | il worker, in Python, senza stato, uccidibile in ogni istante (ADR-0028, I5). Al core arrivano **eventi** | la forma dell'evento continuo sul canale `process` (§7, sezione 2) |
| «E la GPU?» | audio e presentazione sono **concessioni permanenti tenute dal core** (ADR-0033, montate dal Task 10 del Traguardo 5). Il porto `process` **pretende una concessione per avviare qualunque worker** (§5.6 della spec, `Process::start(grant, ..)`) | niente di nuovo oggi: il worker telecamera chiede una concessione da **zero MiB**, non prelazionabile (§4) |
| «Profilo riservato» | spegne la voce always-on (ADR-0023) | spegne **anche** la telecamera: un richiamo datato |
| «Telecamera assente o spenta» | si dichiara **prima**, non si fallisce dopo (ADR-0019) | niente |

---

## 4. L'approccio scelto, nella forma corretta

**Approccio 1 — Sorgente di percezione sotto il core.**

| Pezzo | Forma |
|---|---|
| il worker | Python, always-on, sotto il core come sarà il microfono (ADR-0028). **Possiede la telecamera**; i fotogrammi non escono mai. MediaPipe su **CPU** (§5: su Windows in Python la GPU non c'è) |
| il canale | il porto `process` (ADR-0035): **una** `instruct_stream` all'avvio, poi `read_next` per tutta la vita, come il worker audio descritto nella doc del porto. Ogni frame dichiara la propria lunghezza, `minicbor` (§6.10 della spec, ADR-0037) |
| le due specie di evento | lo **stato continuo della mano** (21 punti per mano, pinch derivato, a N Hz) e il **gesto discreto** (nome, confidenza). Sono **eventi, non passi** (ADR-0011): niente giornale |
| il core | li **smista**: la manipolazione va alla GUI con `Ipc::send`, transitoria, **campionata alla frequenza che il core decide** (§6.1.4 della spec); un gesto di **comando** prende **la strada della wake word** (ADR-0011) e solo lì nasce un passo |
| la concessione | da **zero MiB**, `Preemption::Never`, chiesta all'avvio come le due permanenti di ADR-0033. **La formula di ADR-0005 non cambia** |
| «riservato» | spegne anche la telecamera (richiamo ad ADR-0023) |
| la GUI | **disegna la mano dai 21 punti**: niente video nella webview, quindi **nessuna misura in più** per ADR-0029 |

**Le tre correzioni**, nate dalla sfida del proprietario e dalla verifica nel sorgente (§5):

| Prima diceva | Ora dice | Perché |
|---|---|---|
| «terza concessione permanente, oggi zero» come **terza quota** nella formula | **niente terza quota**: concessione da zero MiB come ogni worker; la formula resta | aggiungere oggi una quota che vale zero è **sfoggio** (criterio 5); il porto pretende comunque una concessione, quindi la forma minima c'è già. La terza quota si apre **quando esiste un tracciatore su GPU**: voce registrata |
| «il gesto di comando diventa un **trigger** (ADR-0009)» | prende **la strada della wake word** (ADR-0011) | il trigger di ADR-0009 è l'innesco dell'**anello di verifica**; il posto giusto per «un evento apre un passo» è il corollario di ADR-0011. **E quella strada non esiste ancora nel codice**: la costruisce chi arriva primo, la voce o i gesti — dipendenza dichiarata, non buco dell'approccio |
| «il core smista alla GUI» (dedotto) | **verificato**: `Ipc::send` è chiamato dal core quando decide, e campionare è una leva del kernel | la **latenza** resta **non misurata**: spike |

**Scartati, col perché** (§9): l'approccio 2 e l'ibrido.

| | 1 · worker sotto il core | 2 · dentro la GUI |
|---|---|---|
| always-on senza GUI | sì | no |
| coerenza con ADR-0011 e ADR-0004 | piena | rotta: la telecamera diventa stato della GUI, che è sacrificabile |
| latenza della manipolazione | un salto in più, **da misurare** | minima |
| costo di partenza | un worker Python in più | solo JS nella webview |
| se poi serve l'altro | niente da rifare | si riscrive tutto |

---

## 5. Verificato, dedotto, assunto

### Verificato nel sorgente, il 2026-09-03

| Che cosa | Dove, e il comando |
|---|---|
| il canale `process` **regge lo stream per costruzione**: *«The audio worker keeps a stream receipt open for its whole life, opened by a single instruction at start-up»*; *«A STREAM RECEIPT IS NOT A JOURNAL STEP … a SOURCE OF EVENTS, not steps (ADR-0011)»*; *«EVERY BYTE THAT FLOWS BACK IS COVERED BY A RECEIPT»* | doc di modulo di `crates/kernel/src/ports/process.rs`; il tratto `Worker` ha `instruct_one`, `instruct_stream`, `read_one`, `read_next`, `close`, `kill`; `Process::start(grant, descriptor)` — `grep -n -E '^\s*(pub trait|fn )' crates/kernel/src/ports/process.rs` |
| il canale `ipc` ha il verso **core → GUI**, e *«the core decides WHEN to emit, and the gui does not pull»*; *«aggregating, sampling or coalescing updates is a KERNEL choice»* | doc di modulo di `crates/kernel/src/ports/ipc.rs`; il tratto `Ipc` ha `accept`, `send`, `receive` |
| **nessun meccanismo «un evento apre una run» esiste nel codice**: la decisione c'è (ADR-0009 trigger, ADR-0011 sorgenti di eventi), il meccanismo no | `grep -rn -E 'fn (open\|start\|begin)_?run\|pub struct Run\b\|enum Trigger\|struct Trigger\|trait Trigger' crates/kernel/src` non rende niente |
| `Mib::ZERO` esiste ed è usato nelle somme dell'arbitro; **nessun divieto trovato** su una riserva da zero | `crates/kernel/src/arbiter/resource.rs:66`; `grep -n -i -E 'Mib\(0\)\|zero' crates/kernel/src/arbiter/*.rs`. ⚠️ *Non trovato* non è *provato*: sonda |
| **non esiste un registro delle funzioni del programma**: l'unica riga vicina è ADR-0025, *«il livello 1 resta ammesso solo per strumenti interni che non eseguono codice»* | `grep -n -i -E 'strument[oi] intern\|registro degli strumenti\|palette\|scorciatoi' docs/superpowers/specs/2026-08-06-kernel-design.md docs/adr/*.md` |
| le righe di [`tracciabilita.md`](../../tracciabilita.md) vicine: *Comandi rapidi e slash-command → GUI*, *Wake word → Voce*, *Screenshot e comprensione dello schermo → L3 + Conversazione*, *Hotkey globale, tray e clipboard → L3*, *Overlay/finestra fluttuante → GUI + L3*, *Convivenza pipeline audio ↔ job GPU ✅ quota sottratta (ADR-0005)*. Le sezioni: 6 Voce, 7 Multimodalità e generazione, 8 Sistema | `grep -n '^## ' docs/tracciabilita.md`, e `grep -n -i -E 'comand\|wake\|screenshot\|hotkey\|overlay\|audio' docs/tracciabilita.md` |
| la [roadmap](../../roadmap.md): sotto-progetto **2** GUI minima (dipende da 1 e ADR-0027), **8** Voce (L2, dipende da 7, chiude lo spike SP-2), **10** Integrazione OS completa (L3, dipende da 2) | `grep -n -E '^\| \*{0,2}[0-9]{1,2}\*{0,2} \|' docs/roadmap.md` |
| gli spike esistenti sono SP-1 … SP-6 in [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md): il prossimo si chiama **SP-7** | `grep -n -E '^## ' spikes/RISULTATI.md` |
| la telecamera **non era mai stata valutata** nel repo | scoperta 4 del disegno della chiusura, col suo `grep` |

### Verificato nello stato dell'arte, il 2026-09-03 — fonti in §10

| Fatto | Fonte |
|---|---|
| MediaPipe è **mantenuto**: `mediapipe` 1.0.1 su PyPI, rilasciato il **2026-08-14**; Python 3.9–3.12; ruote per Windows x86-64 e ARM64; Apache 2.0. Su GitHub `v1.0.0` del 2026-07-28 | F1, F7 |
| in Python la GPU **non c'è su Windows**: *«GPU support is currently limited to Ubuntu platforms»* | F2 |
| Hand Landmarker: **21 punti** per mano, coordinate normalizzate e in metri; su Pixel 6 **17,12 ms** su CPU e **12,27 ms** su GPU; opzioni `num_hands`, tre soglie di confidenza | F4 |
| Gesture Recognizer: modi IMAGE, VIDEO, **LIVE_STREAM** (risultati **per callback**, non bloccante); **otto** gesti pronti: `None`, `Closed_Fist`, `Open_Palm`, `Pointing_Up`, `Thumb_Down`, `Thumb_Up`, `Victory`, `ILoveYou`; gesti propri con `custom_gestures_classifier_options` | F3 |
| nel browser: JS su **WebGL**; **WebGPU non supportato** (issue aperta dal 2025-01-15, in attesa di Google); `detectForVideo` **blocca il thread della UI**, servono web worker | F5, F6 |
| alternativa mantenuta: **RTMPose** (OpenMMLab, Apache 2.0, esporta ONNX e TensorRT); se un giorno serve la GPU su Windows la via è **ONNX Runtime**. Ultraleap Gemini e OpenXR sono hardware dedicato, fuori perimetro | F8 |

### Dedotto, e dichiarato tale

- **Il pinch non è fra i gesti pronti**: si ricava dalla distanza fra la punta del pollice e quella dell'indice. È la pratica corrente nei progetti che usano i 21 punti; **non è una fonte primaria**.
- Il carico dello stato continuo sul canale è **piccolo**: 21 punti per tre coordinate per due mani sono nell'ordine del mezzo kilobyte a fotogramma, cioè decine di kilobyte al secondo a 30 Hz. Ordine di grandezza, non misura.

### Assunto, e lo dice lo spike SP-7 (§7, sezione 4)

1. Il giro **worker → core → GUI a 30 Hz** su questa macchina ha una latenza accettabile per spostare un pannello con la mano. **Non misurato.**
2. MediaPipe su CPU su questa macchina regge **30 Hz** con margine (i 17 ms del Pixel 6 lo fanno sperare, non lo provano). **Non misurato.**
3. Una riserva da **zero MiB** passa l'ammissione dell'arbitro. **Sonda nel kernel**, non spike.

---

## 6. La sezione 1 del disegno, il testo presentato

> **Sezione 1 — Il perimetro**
>
> Cosa decide **ora**:
>
> 1. La forma della telecamera nel kernel: worker sotto il core, stream aperto una volta, due specie di evento, concessione da zero MiB, «riservato» che la spegne.
> 2. Il principio delle funzioni del programma: **un registro**, molti invocatori (agente, gesto, voce, click), **lo stesso permesso** per tutti.
> 3. Dove va la capacità: una riga in roadmap e le righe in tracciabilità.
> 4. Lo spike che misura le tre ipotesi, prima di scrivere codice di prodotto.
>
> Cosa **rimanda**, e a chi:
>
> - il vocabolario dei gesti, e quali funzioni sono gestuali → il sotto-progetto della capacità;
> - menu virtuali e pannelli mossi con le mani → forme nel sotto-progetto 2, logica nella capacità;
> - dove finisce la foto catturata → brainstorming 2, la knowledge base;
> - la strada «un evento apre una run» → chi la costruisce per primo, la voce o i gesti.
>
> Cosa **esclude**: le finestre dell'OS (B della seconda domanda) → sotto-progetto 10, con un ADR suo; la telecamera come occhio (B della prima) → capacità futura; la GPU per il tracciamento → voce registrata, si apre quando esiste un tracciatore su GPU.
>
> Il prodotto finale, dopo il disegno: **due ADR nuovi** (funzioni del programma; telecamera come sorgente di percezione), **tre richiami datati** (ADR-0001, ADR-0011, ADR-0023), le righe di roadmap e tracciabilità, e lo spike. I numeri degli ADR si danno quando si scrivono, non oggi.

📌 **Clausola aggiunta il 2026-09-03, ripresentandola:** il richiamo datato su ADR-0001 vale se i gesti
entrano nel pilastro Voce; con un quinto pilastro al suo posto serve un ADR che **supera** ADR-0001. La
scelta è la decisione 1 della §8, e si prende nella sezione 3.

---

## 7. Le sezioni che restano, con ciò che ciascuna porta al proprietario

⛔ **Sono il contenuto previsto, non deciso.** Ogni sezione si presenta, si discute, si approva; e ogni decisione si controlla contro i cinque criteri prima di proporla.

| Sezione | Contenuto previsto | Decisioni che porta al proprietario |
|---|---|---|
| **2 — La forma nel kernel** | il worker e il suo profilo (zero MiB, `Preemption::Never`, always-on); la `instruct_stream` unica e le due specie di evento con lo **schema `minicbor`** sul canale `process` (§6.10); il core che campiona e manda alla GUI con `Ipc::send`; il gesto di comando sulla strada della wake word; dove vive ogni pezzo: lo schema in `crates/kernel/src/wire/worker.rs`, il cablaggio in `daemon`, il worker in Python **fuori da `crates/`** | ⚠️ **dove vivono i worker Python**: nel repo non ne esiste ancora nessuno, quindi una cartella (per esempio `workers/`) è una **decisione nuova**, e vale anche per il futuro worker audio |
| **3 — Le decisioni in append** | **ADR nuovo A**: le funzioni del programma sono **strumenti interni** in un registro unico (livello 1 di ADR-0025), invocabili dall'agente, dai gesti, dalla voce e dalla GUI **con lo stesso permesso** (ADR-0016); una fetta è gestuale. **ADR nuovo B**: la telecamera è una **sorgente di percezione always-on sotto il core** (la forma della §4). **Richiami datati**: ADR-0011 (la riga *inferenza percettiva always-on* guadagna il tracciamento delle mani accanto a wake word, VAD e trascrizione), ADR-0023 («riservato» spegne anche la telecamera), ADR-0001 (vedi la decisione a destra). ADR-0005 e ADR-0033 **non cambiano**: voce registrata per il tracciatore su GPU | ⚠️ **il pilastro**: **(a)** i gesti stanno nel pilastro **Voce**, che diventa «voce e gesti» con un richiamo, e il sotto-progetto 8 si allarga; **(b)** un **quinto pilastro**, che richiede un ADR che **superi** ADR-0001 sulla parola «quattro» — la §7 del compendio la dice non rilitigabile senza. **Raccomandazione (a)**: ADR-0011 tratta già voce e percezione insieme, la strada della wake word è condivisa, e il quinto contendente della GPU oggi non usa GPU |
| **4 — La GUI, il sotto-progetto 2 e lo spike SP-7** | la GUI disegna la mano dai punti, **niente video**; un **indicatore sempre visibile** di telecamera accesa (lo spirito di ADR-0023: *una falsa sicurezza è peggio di nessuna sicurezza*); la mappa dei gesti di manipolazione vive nella GUI; **nessuna misura in più** per ADR-0029. Lo spike **SP-7** misura le tre assunzioni della §5, e la sonda della riserva zero è un test del kernel | ⚠️ se il proprietario vuole comunque l'**anteprima video** nella GUI: allora torna la misura in più per il sotto-progetto 2 e il problema della telecamera condivisa fra due processi |
| **5 — Voci aperte, dipendenze, prossimo passo** | la tabella della §8; le dipendenze col brainstorming 2 e con la strada della wake word; poi disegno → piano → esecuzione (due ADR, tre richiami, le righe, lo spike) | — |

---

## 8. Le decisioni aperte per il proprietario

⛔ **Restano aperte: nessuna è risolta d'ufficio qui.** Ognuna porta le opzioni e ciò che consiglierei.

| # | Decisione | Opzioni | Consiglio |
|---|---|---|---|
| 1 | il pilastro (sezione 3) | (a) dentro Voce, con richiamo · (b) quinto pilastro, con ADR che supera 0001 | (a) |
| 2 | **quali** funzioni del programma sono gestuali | tutte · una lista | si decide nel sotto-progetto della capacità, non ora |
| 3 | un gesto può **svegliare** l'agente dormiente, cioè aprire una run senza wake word? | (a) solo la wake word sveglia, i gesti di comando valgono a run aperta · (b) un **gesto di attenzione** deliberato, gemello della wake word | la premessa del proprietario dice (a); (b) è coerente con ADR-0011 e si aggiunge dopo senza rifare niente |
| 4 | la telecamera è accesa **per default**? | (a) opt-in, spenta finché l'utente non la accende · (b) always-on come il microfono | (a): una telecamera è più sensibile di un microfono, e il default conservativo è la forma di ADR-0006 e ADR-0023 |
| 5 | l'anteprima video nella GUI | (a) nessuna, la mano si disegna dai punti · (b) anteprima | (a) |
| 6 | un gesto può invocare un'azione con effetto **irripetibile** (ADR-0007)? | (a) mai da solo: chiede conferma · (b) sì, se la confidenza supera una soglia | (a): un gesto mal riconosciuto non deve poter autorizzare ciò che non si disfa |
| 7 | dove finisce la **cattura** con un gesto | run corrente · knowledge base · entrambe | **brainstorming 2**, dipendenza dichiarata |
| 8 | dove vivono i **worker Python** nel repo | una cartella nuova, per esempio `workers/` · dentro `crates/` | fuori da `crates/`: non sono Rust, e il cancello misura `crates/` |
| 9 | la **terza quota** di ADR-0005 | si apre quando esiste un tracciatore su GPU | registrata, non presa |

---

## 9. Vicoli ciechi e scelte scartate, col perché

| Scartata | Perché |
|---|---|
| `decision-map` | perimetro già noto; GitHub Issues sarebbero una seconda casa delle decisioni (gotcha #68); nessuno strumento GitHub da questa macchina |
| approccio 2, tracciamento nella GUI | muore con la GUI (niente Jarvis a GUI chiusa), contraddice lo slot di ADR-0011, WebGL dentro la quota di presentazione, blocca il thread della UI, la telecamera diventa stato di un processo sacrificabile |
| ibrido: worker per l'always-on e GUI per la manipolazione | due tracciatori su una telecamera, due modelli, due codici: sfoggio |
| la **terza quota** nella formula di ADR-0005, oggi | vale zero; il porto pretende già una concessione, quindi la forma minima esiste. Si apre con un tracciatore su GPU |
| il gesto di comando come **trigger di ADR-0009** | quel trigger innesca l'anello di verifica; il posto giusto è il corollario di ADR-0011 |
| il **video nella webview** | i 21 punti bastano a disegnare la mano; il video costerebbe una misura in più per ADR-0029 e una telecamera condivisa fra due processi |
| le finestre dell'**OS** | effetto OS, settima famiglia di porte: sotto-progetto 10, ADR suo |
| la telecamera come **occhio** | B della prima domanda: i fotogrammi come contenuto non fidato nel gateway sono un'altra cosa, e si aggiungono dopo come capacità |

---

## 10. Le fonti — verificate il 2026-09-03

⚠️ **Da portare in [`riferimenti.md`](../../riferimenti.md) quando si scrive il disegno**, che è il momento in cui `CLAUDE.md` le vuole tracciate lì. Qui stanno con la data perché non vadano perse.

| | Fonte | Che cosa sostiene |
|---|---|---|
| F1 | PyPI, `mediapipe` — https://pypi.org/project/mediapipe/ | versione 1.0.1 del 2026-08-14, Python 3.9–3.12, ruote Windows, Apache 2.0 |
| F2 | Google AI Edge, `BaseOptions` — https://ai.google.dev/edge/api/mediapipe/python/mp/tasks/BaseOptions | *«GPU support is currently limited to Ubuntu platforms»* |
| F3 | Google AI Edge, Gesture Recognizer per Python — https://developers.google.com/edge/mediapipe/solutions/vision/gesture_recognizer/python | modi, callback in LIVE_STREAM, gli otto gesti, i gesti propri |
| F4 | Google AI Edge, Hand Landmarker — https://developers.google.com/edge/mediapipe/solutions/vision/hand_landmarker | 21 punti, coordinate, latenze su Pixel 6, opzioni |
| F5 | Google AI Edge, Hand Landmarker per il Web — https://developers.google.com/edge/mediapipe/solutions/vision/hand_landmarker/web_js | `detectForVideo` sincrono, web worker |
| F6 | GitHub, issue WebGPU support for Vision Tasks — https://github.com/google-ai-edge/mediapipe/issues/5826 | aperta dal 2025-01-15, WebGL oggi |
| F7 | GitHub, release — https://github.com/google-ai-edge/mediapipe/releases | `v1.0.0` del 2026-07-28, `v0.10.35`, `v0.10.33` |
| F8 | Forasoft, la pila del pose tracking nel 2026 — https://www.forasoft.com/learn/ai-for-video-engineering/articles-ai/openpose-mediapipe-rtmpose-pose-tracking | RTMPose come alternativa mantenuta, esporta ONNX e TensorRT. ⚠️ Fonte **di terza mano**: prima di citarla nel disegno si risale a OpenMMLab |
| F9 | GitHub, issue *GPU Delegate is not yet supported for Windows* — https://github.com/google-ai-edge/mediapipe/issues/5126 | aperta il 2024-02-08, **chiusa**; il motivo della chiusura **non è stato letto** |

---

## 11. Come si riprende

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -3`: si parte da `main`, e questo file deve esserci.
2. La lettura obbligatoria di `CLAUDE.md`, per intero; poi questo file, per intero.
3. Si dichiara al proprietario, in poche righe: dove siamo (§0 e §2), quale sezione è la prossima, e che l'accettazione condizionata (§1) è in vigore.
4. Si presenta la prossima sezione della §7, una per volta, ciascuna controllata contro i cinque criteri prima di proporla; le decisioni della §8 si portano nella sezione che le ospita, con le opzioni. ⚠️ Le sezioni già approvate stanno nella §2 con la data, e non si ripresentano.
5. A brainstorming chiuso: il disegno si scrive **sul posto, in questo file**, nella sessione che il proprietario sceglie (la regola del 2026-09-02 dice *la successiva*); poi `superpowers:writing-plans`, poi l'esecuzione con `superpowers:subagent-driven-development`.
6. Poi il brainstorming **distinto** della knowledge base, prima di aprire il sotto-progetto 2.

## Che cosa questa consegna NON ha fatto

| Non fatto | Dove sta il residuo |
|---|---|
| nessun ADR scritto, nessun richiamo datato, nessuna riga di roadmap o tracciabilità | è il prodotto del disegno, §6 e §7 |
| `riferimenti.md` **non toccato**: le fonti stanno nella §10 | si portano lì col disegno |
| nessuno spike lanciato, nessuna sonda scritta | SP-7 e la sonda della riserva zero, §5 |
| questo file **non è nella §12 del compendio** | il passaggio sui documenti di stato del disegno lo aggiunge, come fu per i disegni dei traguardi |
| il motivo della chiusura dell'issue F9 non è stato letto | si legge quando si scrive il disegno |
