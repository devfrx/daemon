# Archivio — la consegna del brainstorming sul riconoscimento gesti, 2026-09-03

⛔ **Non è una lettura obbligatoria.** È il **verbale** della consegna con cui il brainstorming del
riconoscimento gesti si è chiuso il **2026-09-03**, tenuto **parola per parola**. Spostato qui lo
stesso giorno, quando il disegno è stato scritto **sul posto** — al percorso che la consegna
occupava — con la regola di `CLAUDE.md`: *un documento vivo porta ciò che è vero adesso, e un
verbale va in archivio*.

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Il disegno vivo è
[`../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md`](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md);
il prossimo passo sta nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md), in un posto solo.

⚠️ **Una sola cosa è cambiata rispetto al testo consegnato:** i percorsi relativi dei collegamenti,
riscritti per questa cartella perché `check-docs.sh` li verifica. Nessuna parola è stata toccata;
lo prova `diff` fra questo file, dalla riga sotto la recinzione in giù, e la consegna a `066008a`:
`git show 066008a:docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md`.

---

# Riconoscimento gesti dalla telecamera — la consegna del brainstorming

⚠️ **QUESTO FILE È LA CONSEGNA DEL BRAINSTORMING, NON ANCORA IL DISEGNO.** Il brainstorming è
**CHIUSO il 2026-09-03**: le cinque sezioni sono approvate, il testo di ciascuna sta nella §6 e lo stato
nella tabella della §2, in una casa sola. Chi riprende legge questo file **per intero** dopo la lettura
obbligatoria di [`CLAUDE.md`](../../CLAUDE.md) e del [compendio](../COMPENDIO.md), e **scrive il
disegno sul posto**, come dice la §11.
Quando il disegno sarà scritto, sarà scritto **sul posto, a questo stesso percorso**, come fu per
il [disegno della chiusura](../superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md): il puntatore della §6
del compendio non cambia casa. **Il prossimo passo lo dice la §6 del compendio**, in un posto solo.

📌 **Metodo.** Ogni affermazione porta la sua specie: **verificata** (letta nel sorgente o in una
fonte primaria, oggi), **dedotta**, o **assunta**. Le tre sono separate nella §5. Ogni cifra porta
accanto il comando che la rifà, e le fonti portano la data.

---

## 0. Lo stato del repository — aggiornato alla chiusura del brainstorming, 2026-09-03, coi comandi

| | |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb` |
| Il sotto-progetto 1 | **mergiato su `main` il 2026-09-03**: commit di merge `e77329a` (due genitori, albero identico alla testa del ramo), poi `a38d898` che ha portato su `main` il punto di ripresa di `HANDOFF.md` e la riga `branch` di `AVVIO-CHAT.md`. Il ramo `spec/sottoprogetto-1-kernel` è **cancellato**, in locale e su `origin` |
| Albero | pulito, nessuno stash, nessuna operazione a metà |
| Cancello | `bash scripts/check-docs.sh` → `OK` a ogni commit della sessione che ha chiuso il brainstorming. `bash scripts/gate.sh` **non rilanciato** in quella sessione, e dichiarato: nessun file di codice è stato toccato, lo prova il comando della riga sotto; l'ultimo `GATE GREEN` è quello sull'albero mergiato, **prima** del push del merge. Si **rilanciano**, non si citano |
| Codice di prodotto | **non toccato** dal brainstorming, dall'apertura alla chiusura: `git diff --stat c8e234e..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla |
| Questo file | è entrato col commit di consegna `c8e234e`, poi un commit per sezione approvata e uno per la chiusura: `git log --oneline c8e234e~1..HEAD -- docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` li elenca. Tutti su `main`, tutti pushati |
| Fine-riga | questo file è **LF** nell'indice e nell'albero di lavoro; il compendio è LF nell'indice e **CRLF** nell'albero: `git ls-files --eol docs/COMPENDIO.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md`. Chi li riscrive **conserva i fine-riga di ciascuno** e li rimisura dopo col comando `tr -cd` di `CLAUDE.md`; nella sessione del brainstorming ha funzionato Python con `newline=""`, scrittura su un temporaneo e `os.replace` (gotcha #82) |

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
| 6 | La **sezione 2** del disegno, la forma nel kernel | ✅ **approvata il 2026-09-03** sotto accettazione condizionata, letta contro il sorgente di quel giorno | il testo approvato è in §6 |
| 7 | Dove vivono i **worker Python** (decisione 8 della §8) | **`workers/` alla radice**, fuori da `crates/`, con un lockfile Python per worker — sotto accettazione condizionata, raccomandazione accolta | il lockfile non è cosmesi: §6.10.7 della spec fa reggere il timbro di build su un ambiente Python **nostro e versionato** |
| 8 | La **sezione 3** del disegno, le decisioni in append | ✅ **approvata il 2026-09-03** sotto accettazione condizionata; con essa le decisioni 1, 3, 4 e 6 della §8 | il testo approvato è in §6 |
| 9 | La **sezione 4** del disegno, la GUI e lo spike SP-7 | ✅ **approvata il 2026-09-03** dal proprietario; con essa la decisione 5 della §8 | il testo approvato è in §6 |
| 10 | La **sezione 5** del disegno, voci aperte, dipendenze e prossimo passo | ✅ **approvata il 2026-09-03** sotto accettazione condizionata; con essa la decisione 11 della §8, e il brainstorming è **chiuso** | il testo approvato è in §6 |

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
| le righe di [`tracciabilita.md`](../tracciabilita.md) vicine: *Comandi rapidi e slash-command → GUI*, *Wake word → Voce*, *Screenshot e comprensione dello schermo → L3 + Conversazione*, *Hotkey globale, tray e clipboard → L3*, *Overlay/finestra fluttuante → GUI + L3*, *Convivenza pipeline audio ↔ job GPU ✅ quota sottratta (ADR-0005)*. Le sezioni: 6 Voce, 7 Multimodalità e generazione, 8 Sistema | `grep -n '^## ' docs/tracciabilita.md`, e `grep -n -i -E 'comand\|wake\|screenshot\|hotkey\|overlay\|audio' docs/tracciabilita.md` |
| la [roadmap](../roadmap.md): sotto-progetto **2** GUI minima (dipende da 1 e ADR-0027), **8** Voce (L2, dipende da 7, chiude lo spike SP-2), **10** Integrazione OS completa (L3, dipende da 2) | `grep -n -E '^\| \*{0,2}[0-9]{1,2}\*{0,2} \|' docs/roadmap.md` |
| gli spike esistenti sono SP-1 … SP-6 in [`spikes/RISULTATI.md`](../../spikes/RISULTATI.md): il prossimo si chiama **SP-7** | `grep -n -E '^## ' spikes/RISULTATI.md` |
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

## 6. Il testo approvato, sezione per sezione

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

#### Sezione 2 — La forma nel kernel, approvata il 2026-09-03

Sotto accettazione condizionata, letta contro il sorgente di `3ec1ac2`; i comandi stanno accanto alle
affermazioni, e si rilanciano.

| Pezzo | Forma | Dove vive |
|---|---|---|
| il worker | Python, possiede la telecamera, MediaPipe su CPU; senza stato, uccidibile in ogni istante (ADR-0028, I5); i fotogrammi non escono mai | `workers/` alla radice, fuori da `crates/` (decisione 8 della §8), con un lockfile Python |
| il profilo | `ResourceProfile { name: <letterale>, reserved_vram: Mib::ZERO, compute_class: ComputeClass::Realtime, preemption: Preemption::Never }`, finestra `FOR_EVER` — la forma di `AUDIO_RESERVATION` | letterale in `crates/daemon/src/main.rs` |
| la concessione | segue la vita del worker: entra con `Process::start(grant, descriptor)`, torna con `Killed.grant` ad `Arbiter::release`. Con la telecamera opt-in si chiede all'accensione, non all'avvio del core | già così nel porto `process` |
| il canale in su | `FromWorker` guadagna due varianti — lo stato della mano (21 punti per mano, coordinate **intere**) e il gesto (`kind`: enum chiuso `#[cbor(index_only)]`, `confidence`: intero) — a indici nuovi `#[n(2)]` e `#[n(3)]`, sotto le regole di §6.10 | `crates/kernel/src/wire/worker.rs` |
| il canale in giù | la prima istruzione vera del canale, «traccia le mani», mandata una volta con `instruct_stream`; oggi la direzione core → worker non ha nessun messaggio | stesso file |
| il core | legge `read_next`, campiona alla frequenza che riceve come **parametro consegnato** (ADR-0034: un campo nuovo di `Parameters`, letterale in `daemon`), manda alla GUI con `Ipc::send`; un gesto di **comando** prende la strada della wake word (ADR-0011) | il primo lettore di produzione di una porta: `grep -rn read_next crates/kernel/src crates/daemon/src crates/platform/src` trova solo commenti, e lo stesso vale per `.receive(` e `.accept(` |
| verso la GUI | una variante nuova di `IpcMessage` con la mano campionata; **si definisce quando la GUI esiste** (sotto-progetto 2), perché prima non ha destinatario — la regola già scritta per la revoca in testa a `crates/kernel/src/wire/ipc.rs` | `crates/kernel/src/wire/ipc.rs` |
| «riservato» | nel codice non esiste (`grep -rni riservato crates/` → niente); spegnere la telecamera è un richiamo ad ADR-0023, il meccanismo arriva col profilo | ADR-0023 |
| degrado | `Degradation` guadagna «telecamera assente o spenta» **solo quando il worker esiste**: la regola *«un campo sempre `false` si legge come "va bene" e non come "non so"»* è già scritta in testa al tipo | `crates/kernel/src/degradation.rs` |

**Tre regole di forma, verificate nel sorgente:**

| Regola | Perché, e il comando |
|---|---|
| **niente decimali nel kernel**: sul filo viaggiano interi, la conversione la fa il worker | `grep -rnw f32 crates/kernel/src` e `grep -rnw f64 crates/kernel/src` → niente; MediaPipe dà coordinate normalizzate fra 0 e 1 (F4) |
| **nessun testo dal worker arriva a una decisione**: il gesto è un enum chiuso, non una stringa | l'argomento di `GrantRequest`, che rifiuta `name` perché testo scelto dal pari è contenuto non fidato (ADR-0014). Il vocabolario dei gesti resta della capacità; il **tipo** si fissa ora |
| **le regole del canale restano quelle di §6.10**: un `#[n(i)]` per campo, niente enum di versione, niente byte congelati, stringhe di byte annotate | testa di `crates/kernel/src/wire/worker.rs` |

**Ciò che il codice dice e la consegna non diceva: la telecamera sarebbe il PRIMO worker vero, e il
primo paga.** Verificato il 2026-09-03:

| Che cosa manca | Dove lo dice |
|---|---|
| nessuna implementazione di `Process` o `Worker` fuori dai banchi: la piattaforma deve imparare ad avviare un processo e parlargli su una pipe | `grep -rn 'impl Process for' crates/` e `grep -rn 'impl Worker for' crates/` → solo `tests/` |
| il canale worker ha una direzione sola, in su; in giù nessun messaggio, e il grilletto dichiarato è *«il primo processo worker vero»* | testa di `crates/kernel/src/wire/worker.rs` |
| il timbro di build non esiste: niente rifiuta un worker stantio; e §6.10.7 della spec fa reggere il timbro su un ambiente Python **nostro e versionato** — quindi il lockfile in `workers/` non è cosmesi | stessa testa; spec §6.10.7 |
| il reattore conosce solo il tempo (`now`, `wall_time`, `wait_until`): «pronto da leggere» per una pipe non c'è, e allargarlo è dichiarato meccanico nel file | `crates/kernel/src/ports/reactor.rs` |
| nessun codice di produzione legge una porta: il ciclo che legge lo stream nasce con questo | il comando della riga «il core» |

Non cambia l'approccio: è il prezzo, lo paga chi arriva primo fra voce e gesti — come la strada della
wake word — e va scritto nell'ADR B come **costo dichiarato**.

**Ipotesi che restano tali:** una riserva da `Mib::ZERO` passa l'ammissione — sonda nel kernel, §5.

#### Sezione 3 — Le decisioni in append, approvata il 2026-09-03

Sotto accettazione condizionata. Le righe degli ADR citate sono state lette quel giorno, coi comandi
`grep -n -i quattro docs/adr/0001-*.md`, `grep -n -i percettiv docs/adr/0011-*.md`,
`grep -n -i riservato docs/adr/0023-*.md`, `grep -n -i 'strumenti interni' docs/adr/0025-*.md`,
`grep -n -i irripetibil docs/adr/0016-*.md` e `grep -n -i registro docs/adr/0009-*.md`.

| ADR nuovo | Decide | Negative (accettate) |
|---|---|---|
| **A — il registro delle funzioni del programma** | un registro unico di **strumenti interni** (il livello 1 di ADR-0025), meccanismo di kernel nella forma di ADR-0009: il kernel dà registrazione, invocazione, il permesso come tripla di ADR-0016 e il giornale; le capacità e la GUI portano le funzioni. **Molti invocatori** — agente, gesto, voce, click — **con lo stesso permesso**, e nessuna logica «solo per gesti». Un gesto è un evento di percezione: **informa, mai autorizza** (ADR-0014 per analogia). Un effetto irripetibile chiede conferma a qualunque invocatore (ADR-0016, già così), e **per default la conferma non è gestuale**. La manipolazione della GUI — pannelli, menu virtuali — è presentazione e **non passa dal registro**. Quali funzioni siano gestuali lo decide la capacità (decisione 2 della §8) | un meccanismo di kernel in più prima di ogni capacità; ogni funzione con effetto va dichiarata come tripla; lo stesso permesso pesa anche sulle funzioni banali invocate dalla GUI |
| **B — la telecamera come sorgente di percezione always-on sotto il core** | la forma della sezione 2 per intero. Più: telecamera **spenta per default**, e accenderla è una funzione del registro; **solo la wake word apre una run**, un gesto di comando entra come passo in una run aperta; «riservato» la spegne; il campo di `Degradation` nasce col worker. **Non costruzioni dichiarate:** il messaggio alla GUI aspetta il sotto-progetto 2; la terza quota aspetta un tracciatore su GPU | un processo Python su CPU finché la telecamera è accesa; un salto in più sulla manipolazione, misurato da SP-7; il conto del **primo worker** (sezione 2); niente GPU su Windows per il tracciatore (F2) |

**I tre richiami datati:**

| ADR | La riga | Il richiamo |
|---|---|---|
| ADR-0001 | *«quattro aree — conversazione, agenti/coding, voce, …»* (riga 9) e i consumatori paritari (riga 30) | «voce» si legge **«voce e gesti»**; i pilastri restano quattro, nessun ADR superato |
| ADR-0011 | la tabella *«inferenza percettiva always-on»*, esempi *«wake word, VAD, trascrizione continua»* (righe 57–64) | entra **il tracciamento delle mani**; un gesto di comando fa come la trascrizione che diventa messaggio — il gesto apre un passo, i fotogrammi no |
| ADR-0023 | punto 5, *«disattiva avvio automatico e voce always-on»* (riga 65) | entra **«e la telecamera»** |

**ADR-0005 e ADR-0033 non cambiano**: la terza quota resta voce registrata (decisione 9 della §8).

**Le decisioni della §8 prese qui**, sotto accettazione condizionata: **1** → (a) dentro Voce; **3** → (a) solo
la wake word sveglia; **4** → (a) opt-in; **6** → (a) mai da solo, e la conferma non è gestuale per default.

**Registrate, non prese:** dove si salva l'interruttore della telecamera fra un avvio e l'altro — è l'archivio
dei parametri (ADR-0034, ADR-0022), che non esiste, e lo chiude chi lo costruisce; e la posizione della
capacità nella roadmap — con la scelta 1 il sotto-progetto 8 si allarga a «voce e gesti», e se i gesti
vengano prima della voce si decide nella sezione 5.

#### Sezione 4 — La GUI, il sotto-progetto 2 e lo spike SP-7, approvata il 2026-09-03

Letta contro [`spikes/GUI-REQUISITI.md`](../../spikes/GUI-REQUISITI.md) (G6, G20, P1–P3), le righe
M1–M5 di ADR-0029, il consumatore 1 della tabella di ADR-0033 e il formato di
[`spikes/RISULTATI.md`](../../spikes/RISULTATI.md).

| Pezzo | Forma | Da dove |
|---|---|---|
| la mano sullo schermo | la GUI **disegna la mano dai 21 punti**, in un livello sopra i pannelli; **niente video** (decisione 5 della §8) | il disegno è compositing della webview: sta **dentro la quota di presentazione**, nessuna concessione da chiedere — ADR-0033, consumatore 1 |
| l'indicatore | un segno **sempre visibile** quando la telecamera è accesa; lo accende il **core** con un messaggio, la GUI non lo indovina | lo spirito di ADR-0023: *una falsa sicurezza è peggio di nessuna sicurezza* |
| pannelli e menu | il sotto-progetto 2 costruisce pannelli e menu che si muovono con **qualunque puntatore**; la mano è un puntatore in più, e la aggiunge la capacità (pinch che trascina, menu virtuali). Solo stato di presentazione | ADR-0004: la GUI possiede solo presentazione. Il messaggio IPC con la mano si definisce da lì in poi, non prima (sezione 2) |
| accessibilità | un gesto **non è mai l'unica strada**: ogni funzione gestuale si raggiunge anche da tastiera e click | G20; segue da «un registro, molti invocatori» (ADR A) |
| ADR-0029 | **nessuna misura in più**; ma **M4** — «P3 con rendering vero» — deve includere la mano disegnata a 30 Hz quando la si lancia: P3 è già stretto (21,43 % su 25) | ADR-0029 righe M1–M5; GUI-REQUISITI P3 |

**Lo spike SP-7** — usa e getta, in `spikes/`, fuori dal workspace (`exclude = ["spikes"]` in `Cargo.toml`).
I criteri si scrivono **prima** di misurare; l'esito va in `spikes/RISULTATI.md` con le versioni degli
strumenti, la CPU della macchina e le evidenze, nella forma di SP-5 e SP-6.

| # | Domanda | Criterio, scritto prima | Come |
|---|---|---|---|
| S1 | MediaPipe Hand Landmarker su CPU regge 30 Hz su questa macchina? | tempo per fotogramma **< 33 ms** — mediana e p95 riportati — a due mani, 640×480, modo LIVE_STREAM. Il margine si **riporta**, non si promette | Python, `mediapipe` 1.0.1 (F1), telecamera vera |
| S2 | quanto costa il giro worker → core → GUI a 30 Hz? | latenza da cattura a disegno, mediana e p95; il solo salto core → GUI contro il numero che il repo ha già, **P2 < 100 ms**. L'accettabilità della mano sul pannello la giudica il **proprietario provandola**: nessuna soglia inventata | Python + un relay Rust usa e getta + una pagina che disegna i 21 punti |
| S3 | una riserva da zero MiB passa l'ammissione? | `Admission::Granted` con `reserved_vram: Mib::ZERO`, **anche a macchina piena**; e la contro-sonda: una riserva vera a macchina piena resta `Queued` | **sonda nel kernel**, `crates/kernel/tests/arbiter_admission.rs` — non spike |

**Decisione della §8 presa qui**, sotto accettazione condizionata: **5** → (a) nessuna anteprima video.

**Registrata, non presa:** se la posizione dei pannelli sopravvive a un riavvio — è configurazione
(archivio di ADR-0022), che non esiste; la chiude chi costruisce l'archivio.

#### Sezione 5 — Voci aperte, dipendenze e prossimo passo, approvata il 2026-09-03

Sotto accettazione condizionata. Letta contro la tabella «Sotto-progetti» e il «Perché quest'ordine» di
[`roadmap.md`](../roadmap.md), e la sezione 6 di [`tracciabilita.md`](../tracciabilita.md).

**Dove va la capacità — decisione 11 della §8: una riga nuova, il sotto-progetto 12 «Gesti».** Non dentro
«Voce»: la roadmap mette Voce dopo Generazione asset perché SP-2 vuole *voce e job GPU pesante insieme*,
e i gesti non usano la GPU — quella ragione non li riguarda. Si appende **senza rinumerare**, come 0b, 0c
e 11. Il pilastro resta «voce e gesti» (richiamo ad ADR-0001), costruito in due sotto-progetti come il
kernel lo è in quattro.

| Riga della roadmap | Dipende da | Perché |
|---|---|---|
| **12 — Gesti**, L2 | 2 e 3 | dal 2 i pannelli mobili e il registro delle funzioni (ADR A); dal 3 la run che un gesto di comando comanda |
| **8 — Voce** | 7, e ora anche **12** | riusa ciò che il primo worker paga (sezione 2): trasporto di `process`, messaggio in giù, timbro di build, prontezza del reattore, ciclo di lettura |
| **2 — GUI minima** | invariata | ma è il **primo invocatore** del registro di ADR A, col click: chi arriva primo lo costruisce |

**Le righe di tracciabilità:** nella sezione 6, che si intitola «Voce e gesti», con sede «Gesti» —
tracciamento delle mani, gesti di comando, manipolazione di pannelli e menu con le mani (sede GUI +
Gesti), cattura con un gesto (Gesti + brainstorming 2), indicatore di telecamera accesa (GUI). Il registro
delle funzioni va accanto a «Comandi rapidi e slash-command», sede GUI, come meccanismo deciso.

**Le dipendenze dichiarate:**

| Cosa | Da chi dipende |
|---|---|
| dove finisce la **cattura** (decisione 7) | il brainstorming 2, la knowledge base |
| la strada «un evento di percezione apre un passo» (ADR-0011) | la costruisce il **12**, primo con una sorgente di percezione; la voce la riusa |
| l'interruttore della telecamera e la posizione dei pannelli (decisioni 10 e 12) | l'archivio dei parametri, che nessun sotto-progetto colloca ancora — registrato |
| il timbro di build sui due canali (§6.1.2) | GUI col **2**, worker col **12**: i due grilletti già scritti in testa a `crates/kernel/src/wire/ipc.rs` e `crates/kernel/src/wire/worker.rs` |

**Le voci che restano aperte**, tutte con un chiusore scritto nella §8: **2** (quali funzioni sono
gestuali → la capacità), **7** (la cattura → brainstorming 2), **9** (la terza quota → un tracciatore su
GPU), **10** e **12** (→ l'archivio dei parametri). Nessuna sbarra il disegno.

**Il prossimo passo, in ordine:**

1. ✅ questa sezione approvata **chiude il brainstorming** il 2026-09-03.
2. la **sessione successiva** scrive il disegno **sul posto, in questo file** (regola del proprietario del 2026-09-02).
3. poi il piano con `superpowers:writing-plans`. Compiti attesi: i due ADR — ciascuno con la propria voce nella §5 del compendio, che `check-docs.sh` pretende — e i tre richiami datati; la riga 12 e le dipendenze in `roadmap.md`; le righe di `tracciabilita.md`; le fonti F1–F9 in `riferimenti.md`, con F8 risalita a OpenMMLab e il motivo della chiusura di F9 letto; lo spike SP-7 in `spikes/` con l'esito in `spikes/RISULTATI.md`; la sonda S3 nel kernel; questo file nella §12 del compendio.
4. poi il brainstorming **distinto** della knowledge base; poi il sotto-progetto 2.

---

## 7. Le sezioni che restano, con ciò che ciascuna porta al proprietario

⛔ **Sono il contenuto previsto, non deciso.** Ogni sezione si presenta, si discute, si approva; e ogni decisione si controlla contro i cinque criteri prima di proporla.

⚠️ **RICHIAMO DEL 2026-09-03: le cinque sezioni sono tutte approvate.** Il testo che vale è quello della §6; questa tabella resta come verbale di ciò che era previsto, e dove i due divergono vince la §6.

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
| 1 | il pilastro (sezione 3) | ✅ **DECISA il 2026-09-03: (a) dentro Voce**, con richiamo ad ADR-0001 — sotto accettazione condizionata. Scartato (b), il quinto pilastro: vorrebbe un ADR che supera 0001 per un contendente della GPU che oggi non la usa | — |
| 2 | **quali** funzioni del programma sono gestuali | tutte · una lista | si decide nel sotto-progetto della capacità, non ora |
| 3 | un gesto può **svegliare** l'agente dormiente, cioè aprire una run senza wake word? | ✅ **DECISA il 2026-09-03: (a)** — solo la wake word sveglia, i gesti di comando valgono a run aperta; (b), il gesto di attenzione, resta aggiungibile dopo senza rifare niente | — |
| 4 | la telecamera è accesa **per default**? | ✅ **DECISA il 2026-09-03: (a) opt-in** — spenta finché l'utente non la accende, e accenderla è una funzione del registro; dove l'interruttore si salva è la riga 10 | — |
| 5 | l'anteprima video nella GUI | ✅ **DECISA il 2026-09-03: (a) nessuna** — la mano si disegna dai punti; il video costerebbe una misura in più per ADR-0029 e una telecamera divisa fra due processi | — |
| 6 | un gesto può invocare un'azione con effetto **irripetibile** (ADR-0007)? | ✅ **DECISA il 2026-09-03: (a)** — chiede conferma a qualunque invocatore (ADR-0016), e per default la conferma **non è gestuale**: un gesto letto male non deve poter confermare sé stesso | — |
| 7 | dove finisce la **cattura** con un gesto | run corrente · knowledge base · entrambe | **brainstorming 2**, dipendenza dichiarata |
| 8 | dove vivono i **worker Python** nel repo | ✅ **DECISA il 2026-09-03: `workers/` alla radice**, con un lockfile per worker — sotto accettazione condizionata (§2, riga 7). Scartato *«dentro `crates/`»*: Cargo tratta `crates/` come workspace, e un pacchetto non Rust lì confonde il cancello e ADR-0031 | — |
| 9 | la **terza quota** di ADR-0005 | si apre quando esiste un tracciatore su GPU | registrata, non presa |
| 10 | dove si **salva l'interruttore** della telecamera fra un avvio e l'altro | l'archivio dei parametri (ADR-0034, ADR-0022), che non esiste | registrata, non presa: la chiude chi costruisce l'archivio |
| 11 | la **posizione nella roadmap** della capacità | ✅ **DECISA il 2026-09-03: una riga nuova, il sotto-progetto 12 «Gesti»**, che dipende da 2 e 3; Voce dipende anche da 12 — sotto accettazione condizionata. Scartato *«dentro il sotto-progetto 8»*: rimandava i gesti per una ragione, SP-2, che non li riguarda | — |
| 12 | se la **posizione dei pannelli** sopravvive a un riavvio | configurazione, archivio di ADR-0022, che non esiste | registrata, non presa: la chiude chi costruisce l'archivio |

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

⚠️ **Da portare in [`riferimenti.md`](../riferimenti.md) quando si scrive il disegno**, che è il momento in cui `CLAUDE.md` le vuole tracciate lì. Qui stanno con la data perché non vadano perse.

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
2. La lettura obbligatoria che `CLAUDE.md` prescrive — il compendio per intero, in più letture e non in una, e la testa dell'audit del 2026-08-27 — poi questo file, per intero.
3. Si dichiara al proprietario, in poche righe: il brainstorming è **chiuso** (§2), il compito è **scrivere il disegno**, e l'accettazione condizionata (§1) è in vigore.
4. Si scrive il disegno **sul posto, in questo file**, al passo «Write design doc» di `superpowers:brainstorming`: il testo approvato della §6 diventa le sezioni del disegno, le decisioni della §8 e le fonti della §10 entrano dove il disegno le ospita. ⚠️ Dove finisca il verbale di questa consegna lo decide chi scrive, con la regola di `CLAUDE.md`: un documento vivo porta ciò che è vero adesso, e un verbale va in [`archivio/`](../archivio/). ⚠️ E ogni affermazione verificata qui si rilegge contro il codice di **allora**: i comandi stanno accanto.
5. Poi la revisione del disegno dal proprietario, poi `superpowers:writing-plans`, poi l'esecuzione con `superpowers:subagent-driven-development`.
6. Poi il brainstorming **distinto** della knowledge base, prima di aprire il sotto-progetto 2.

## Che cosa questa consegna NON ha fatto

| Non fatto | Dove sta il residuo |
|---|---|
| nessun ADR scritto, nessun richiamo datato, nessuna riga di roadmap o tracciabilità | è il prodotto del disegno, §6 e §7 |
| `riferimenti.md` **non toccato**: le fonti stanno nella §10 | si portano lì col disegno |
| nessuno spike lanciato, nessuna sonda scritta | SP-7 e la sonda della riserva zero, §5 |
| questo file **non è nella §12 del compendio** | il passaggio sui documenti di stato del disegno lo aggiunge, come fu per i disegni dei traguardi |
| il motivo della chiusura dell'issue F9 non è stato letto | si legge quando si scrive il disegno |
