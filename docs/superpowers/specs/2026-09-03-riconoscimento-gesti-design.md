# Riconoscimento gesti dalla telecamera: il disegno

✅ **QUESTO DISEGNO È COMPLETO DAL 2026-09-03.** Le cinque sezioni sono **approvate** dal
proprietario, una per volta, in chat il 2026-09-03 — le sezioni 2, 3 e 5 sotto **accettazione
condizionata**, la cui regola sta qui sotto — e la **§5** fissa dove va la capacità, le dipendenze e
l'ordine di ciò che segue. Chi riprende ha un disegno intero da tradurre in piano — **dopo** che il
proprietario lo ha riletto in questa forma. ✅ **Riletto il 2026-09-03**, sotto accettazione
condizionata: la verifica di coerenza che quella condizione chiedeva è scritta nella sezione *«Cosa
questo disegno ha misurato»*, con ciò che ha aggiunto.

⚠️ **RICHIAMO DEL 2026-09-03, lo stesso giorno:** questo file è nato come **consegna** del
brainstorming — le cinque sezioni approvate, le scelte del proprietario, lo stato del repository alla
chiusura — e il proprietario ha scelto che il disegno lo scrivesse la sessione **successiva**.
Riscritto **sul posto**, allo stesso percorso, perché il puntatore della §6 del
[compendio](../../COMPENDIO.md) non cambiasse casa, come fu per il
[disegno della chiusura](2026-09-02-sottoprogetto-1-chiusura-design.md). Il merito delle cinque
sezioni è quello approvato e **non è stato toccato**; la consegna sta **parola per parola** in
[`archivio/consegna-brainstorming-gesti.md`](../../archivio/consegna-brainstorming-gesti.md); ciò che
la riscrittura ha **misurato** in più sta nella sezione *«Cosa questo disegno ha misurato»*, e
**un'affermazione della consegna vi risulta più debole di come era scritta** — la fonte F8, §7.

⚠️ **Non è una spec.** Come i disegni dei Traguardi 4, 5 e 6, fissa il **perimetro**, le **forme**
che gli ADR e la [spec del sotto-progetto 1](2026-08-06-sottoprogetto-1-kernel.md) descrivono a
parole, e per ogni artefatto **il controllo che lo esercita**. La spec resta l'autorità — la §5 per
l'arbitro, la §6.10 per il canale worker — e ciò che questo disegno vi aggiunge lo **dichiara** come
decisione in append, nella §3.

📌 **Metodo.** Ogni affermazione porta la sua specie — **verificata** (letta nel sorgente o in una
fonte primaria, con la data), **dedotta**, o **assunta** — e le tre sono separate nella §6. Le
affermazioni sul sorgente sono state lette il 2026-09-03 contro `066008a`, e il codice non è
cambiato dalla consegna: `git diff --stat c8e234e..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml
rust-toolchain.toml docs/adr/` non rende nulla. I comandi stanno accanto alle affermazioni e **si
rilanciano**, non si citano: le cifre invecchiano al primo commit che tocca ciò che misurano, i
comandi no.

**Le regole di questo lavoro, decise dal proprietario**

| Regola | Da dove viene |
|---|---|
| La strada è quella del repo: **brainstorming → disegno scritto → piano → esecuzione**, come ogni traguardo | scelta 5 del 2026-09-02, in testa al disegno della chiusura |
| `anthropic-skills:decision-map` **non si usa**: il perimetro è piccolo e noto, GitHub Issues sarebbe una **seconda casa** delle decisioni (gotcha #68), e da questa macchina non c'è né `gh` né il connettore GitHub | proposta e motivata il 2026-09-03, senza obiezione |
| **Ogni decisione si controlla contro i cinque criteri di `anthropic-skills:decision-principles`**, in modo **esplicito**. Il proprietario ha dato un'**accettazione condizionata** — *«decidi e progettiamo sempre secondo questa skill, se regge contro i principi procediamo»* — che vale finché regge: se una scelta li viola, ci si **ferma e lo si dice**, non si tratta come delega in bianco | chat del 2026-09-03 |
| Brainstorming in **una** sessione, disegno nella **successiva**, con la consegna in un file **tracciato** | scelta del proprietario del 2026-09-02 |
| Codice in inglese, documenti in italiano (§1.0 della spec); nessun numero senza comando; nessuna fonte senza data | `CLAUDE.md` |

**Le premesse dette dal proprietario**, che il disegno onora: l'agente è **dormiente e risvegliabile
con la wake word**; i gesti sono *«stile Jarvis»*; vuole *«basarmi sulla reale architettura del
progetto ed integrare le cose in modo professionale, seguendo prima lo stato dell'arte e i principi di
decision-principles»*; e aveva *«molti buchi tecnici e soprattutto logici»* in testa, che la §1.5
scioglie coi documenti del repo.

**Le quattro domande d'apertura, e le risposte del proprietario — 2026-09-03**

⛔ **Sono sue e non si riaprono senza di lui.** Ogni riga porta anche ciò che è stato **scartato**,
perché chi riprende sappia che cosa non rifare. Le domande com'erano poste stanno nell'archivio.

| # | La risposta | Scartato, e perché |
|---|---|---|
| 1 | la telecamera è **una sorgente di eventi**: un gesto riconosciuto è un evento come la wake word, e i fotogrammi **non escono mai** dal worker. *«Ma conta che deve avere del potenziale»*: predisposta a **menu virtuali** e a **spostare pannelli con le mani** (pinch e simili), quindi lo **stato continuo della mano** è un evento necessario per la manipolazione diretta, non un lusso | la telecamera come **occhio** dell'agente — immagini nel contesto di un modello: contenuto non fidato, gateway — è un'altra cosa, e si aggiunge dopo come capacità. La **cattura** con un gesto resta un caso d'uso, con la destinazione **aperta**: *«che contesto? conversazionale? un posto nella knowledge base?»* — decisione 7, ✅ **decisa il 2026-09-04** (riga 7 della tabella) |
| 2 | i pannelli e i menu mossi con le mani sono quelli **del programma**, la sua GUI: tutta presentazione, **nessuna porta nuova** nel kernel | le finestre dell'**OS**: effetto OS via modulo di piattaforma, azione con permesso (ADR-0016), una **settima famiglia di porte** dove oggi sono sei per decisione → se mai, nel sotto-progetto 10 con un ADR suo |
| 3 | *«self-use dell'agente sulle funzioni del programma»* significa **A mutuale**: l'agente può richiamare nativamente **tutte** le funzioni esistenti del programma; una **fetta** di quelle (tutte o solo alcune, si deciderà) è anche gestuale. Ne segue **un registro unico, molti invocatori, lo stesso permesso** — l'ADR A della §3 | una logica «solo per gesti»; l'agente che usa la telecamera per sé |
| 4 | **approccio 1**, la sorgente di percezione **sotto il core**, dopo la sfida *«sicuro che rispetti tutti i principi?»* e le **tre correzioni** della §2.1, sotto accettazione condizionata | l'approccio 2 — tracciamento **dentro la GUI** — e l'ibrido: sezione *«Vicoli ciechi»* |

**Le decisioni, numerate come nella consegna** — i rimandi *«decisione N»* di questo disegno
puntano qui.

⛔ **RICHIAMO DEL 2026-09-04 — questa riga diceva *«Le DODICI decisioni»* e la tabella ne porta
TREDICI.** La cifra è **TOLTA e non riallineata** dalle case **vive**: qui, nella §5.4, nella
voce 3 delle voci aperte, e nei **due indici** che il compito 9 aveva appena scritto — la §12 del
compendio e la tabella *«Specifiche»* di [`README.md`](../../README.md). Un cardinale scritto sopra
una tabella che cresce invecchia al primo rigo aggiunto, ed è quello che è successo: la decisione
**13** fu aggiunta il **2026-09-03**, lo stesso giorno dell'etichetta, e lo dichiara di sé. La casa
unica è **la tabella**, e il conto lo dà il comando, delimitato per **testo** e non per numero di riga:
`awk '/^\*\*Le decisioni, numerate/{s=1} s&&/^## 1\./{s=0} s&&/^\| [0-9]+ \|/{n++} END{print n}' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md`.
⛔ **La prima stesura di questo richiamo ancorava il comando ai NUMERI DI RIGA, e il richiamo stesso
lo ha reso falso prima del commit:** il richiamo stesso ha spostato la tabella e il comando
rendeva **5**. ✅ **Provato nelle due direzioni:** con cinquanta righe inserite sopra rende ancora
**13**, e con una riga di tabella in più rende **14**.
⚠️ **Che cosa NON è toccato, e la ragione è una sola:** la riga della sezione *«Come si riprende»*
qui sotto e le occorrenze del [piano](../plans/2026-09-03-riconoscimento-gesti.md) dicono
*«tredici»*, cioè il valore **vero**, dentro **verbali datati** di un piano **chiuso** — e un
verbale invecchia onestamente. Trovato dalla **revisione** del compito 9; il censimento delle case
allargato dalla **ri-revisione** dell'ondata, che ne aveva contate sei su dieci.

| # | Decisione | Stato al 2026-09-03 | Scelta, o chi la chiude | Scartato, e perché |
|---|---|---|---|---|
| 1 | il pilastro | ✅ decisa, sotto accettazione condizionata | **(a) dentro Voce**, che diventa «voce e gesti» con un richiamo datato ad ADR-0001; i pilastri restano quattro | (b) un **quinto pilastro**: vorrebbe un ADR che **supera** ADR-0001 sulla parola «quattro» per un contendente della GPU che oggi non la usa |
| 2 | **quali** funzioni del programma sono gestuali | ⏳ aperta | la capacità, cioè il sotto-progetto 12 — non ora | — |
| 3 | un gesto può **svegliare** l'agente dormiente, aprendo una run senza wake word? | ✅ decisa | **(a) solo la wake word sveglia**; i gesti di comando valgono a run aperta | (b) il gesto di attenzione: resta aggiungibile dopo senza rifare niente |
| 4 | la telecamera è accesa **per default**? | ✅ decisa | **(a) opt-in**: spenta finché l'utente non la accende, e accenderla è una funzione del registro | accesa per default |
| 5 | l'anteprima video nella GUI | ✅ decisa | **(a) nessuna**: la mano si disegna dai 21 punti | il video costerebbe una misura in più per ADR-0029 e una telecamera divisa fra due processi |
| 6 | un gesto può invocare un'azione con effetto **irripetibile** (ADR-0007)? | ✅ decisa | **(a)**: chiede conferma a qualunque invocatore (ADR-0016, già così), e per default la conferma **non è gestuale** — un gesto letto male non deve poter confermare sé stesso | la conferma a gesti |
| 7 | dove finisce la **cattura** con un gesto | ✅ **presa il 2026-09-04** dal brainstorming della knowledge base, sotto accettazione condizionata. ⚠️ Questa cella diceva *«⏳ aperta, dipendenza dichiarata»* | **nella knowledge base**: la cattura atterra come file in un gruppo dello spazio, il router segue, la run riceve il **riferimento** — [disegno della knowledge base](2026-09-04-knowledge-base-design.md), risposta 7 e regola 4 della §2.3 | *solo nella run, poi decide l'assistente*: due posti per un file, e una foto dimenticata non è nella mappa; *entrambe*: idem |
| 8 | dove vivono i **worker Python** nel repo | ✅ decisa, sotto accettazione condizionata | **`workers/` alla radice**, fuori da `crates/`, con un lockfile Python per worker | dentro `crates/`: Cargo tratta `crates/` come workspace, e un pacchetto non Rust lì confonde il cancello e ADR-0031 |
| 9 | la **terza quota** nella formula di ADR-0005 | ⏳ registrata, non presa | si apre quando esiste un **tracciatore su GPU** | aggiungerla oggi, a zero: sfoggio (§2.1) |
| 10 | dove si **salva l'interruttore** della telecamera fra un avvio e l'altro | ⏳ registrata, non presa | l'**archivio dei parametri** (ADR-0034, ADR-0022), che non esiste: la chiude chi lo costruisce | — |
| 11 | la **posizione nella roadmap** della capacità | ✅ decisa, sotto accettazione condizionata | una riga nuova, il **sotto-progetto 12 «Gesti»**, che dipende da 2 e 3; Voce dipende anche da 12 (§5.1) | dentro il sotto-progetto 8, Voce: lo rimandava per una ragione — SP-2 — che non riguarda i gesti |
| 12 | se la **posizione dei pannelli** sopravvive a un riavvio | ⏳ registrata, non presa | configurazione, archivio di ADR-0022, che non esiste: la chiude chi lo costruisce | — |
| 13 | il **confinamento** del worker telecamera — ⚠️ **non era nella consegna**: trovata il 2026-09-03 verificando la coerenza del disegno contro ADR-0025 | ⏳ registrata, non presa | ADR-0028 rende **obbligatorio il confine di processo**; se quel processo debba essere anche **ristretto** — il livello 2 di ADR-0025 — nessun ADR lo decide, e nel codice non c'è dove dirlo: `WorkerDescriptor` è byte opachi e nel kernel non esiste un tipo di confinamento (`grep -rn -i confinement crates/kernel/src`). La chiude il sotto-progetto 12, col proprietario, quando avvia il primo worker vero | — |

---

## 1. Il perimetro — ✅ approvata il 2026-09-03

### 1.1 Che cosa decide ora

1. **La forma della telecamera nel kernel**: worker sotto il core, stream aperto una volta, due specie
   di evento, concessione da zero MiB, «riservato» che la spegne — la §2.
2. **Il principio delle funzioni del programma**: **un registro**, molti invocatori (agente, gesto,
   voce, click), **lo stesso permesso** per tutti — l'ADR A della §3.
3. **Dove va la capacità**: una riga in roadmap e le righe in tracciabilità — la §5.
4. **Lo spike che misura le tre ipotesi**, prima di scrivere codice di prodotto — la §4.2.

### 1.2 Che cosa rimanda, e a chi

| Rimandato | A chi |
|---|---|
| il vocabolario dei gesti, e quali funzioni sono gestuali | il sotto-progetto della capacità, il 12 — decisione 2 |
| menu virtuali e pannelli mossi con le mani | le **forme** nel sotto-progetto 2, la **logica** nella capacità |
| dove finisce la foto catturata | il brainstorming 2, la knowledge base — decisione 7. ✅ **Decisa il 2026-09-04**: nella knowledge base, la run la vede — riga 7 della tabella delle decisioni |
| la strada «un evento apre una run» | chi la costruisce per primo, la voce o i gesti — con la decisione 11 è il **12** (§5.3) |

### 1.3 Che cosa esclude

| Escluso | Dove andrebbe, se mai |
|---|---|
| le finestre dell'**OS** — la B della seconda domanda | il sotto-progetto 10, con un ADR suo |
| la telecamera come **occhio** — la B della prima domanda | una capacità futura |
| la **GPU** per il tracciamento | voce registrata — decisione 9 — che si apre quando esiste un tracciatore su GPU |

### 1.4 Il prodotto, e il controllo che esercita ciascun artefatto

Il prodotto finale, dopo questo disegno: **due ADR nuovi** — le funzioni del programma; la telecamera
come sorgente di percezione — **tre richiami datati** (ADR-0001, ADR-0011, ADR-0023), le righe di
roadmap e tracciabilità, le fonti in `riferimenti.md`, e lo spike con la sua sonda. **I numeri degli
ADR si danno quando si scrivono**, non oggi: l'ultimo lo dice `ls docs/adr | tail -1`.

⚠️ **La tabella qui sotto è un'aggiunta dello scrivente**, non un testo approvato: il *prodotto* viene
dalla sezione approvata, la colonna *«il controllo che lo esercita»* è la forma che ogni disegno di
questo repository porta, riempita leggendo `scripts/check-docs.sh` e `scripts/gate.sh` il 2026-09-03.
✅ **Approvata il 2026-09-03 con la rilettura del disegno**, sotto accettazione condizionata.

| Artefatto | Il controllo che lo esercita | Specie |
|---|---|---|
| gli ADR A e B, in `docs/adr/` | `check-docs.sh` pretende **una voce in §5 del compendio per ogni file** di `docs/adr/`, accoppiata per numero: un ADR senza voce è rosso. ⚠️ E lo stesso script confronta con la realtà i **totali** degli ADR scritti nei documenti di stato: si aggiornano **nello stesso commit** degli ADR, e quali siano lo dice il comando `grep -n -o -E '[0-9]+ ADR( in stato [A-Za-z]+)?\|[0-9]+ decisioni architetturali' docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md` | livello 2, cancello |
| i tre richiami datati su ADR-0001, ADR-0011, ADR-0023 | **nessuno script**: li difende chi rilegge l'ADR e la revisione del compito che li scrive. ⚠️ Dichiarato invece che taciuto: è la specie di affermazione della radice R1 dell'[audit](../../audit-2026-08-27.md), e la forma che regge è quella di AUD-032 — il richiamo **in testa all'ADR**, e il compendio che vi **rimanda** invece di ricopiarlo. ⚠️ **RICHIAMO DEL 2026-09-04:** l'attribuzione ad AUD-032 è **stata corretta** nella §3.2 — la posizione «in testa» è una scelta di questo disegno, non la forma di AUD-032 | revisione |
| la riga 12 e le dipendenze in [`roadmap.md`](../../roadmap.md) | il controllo dei link di `check-docs.sh`; *«senza rinumerare»* lo tiene chi rilegge la tabella «Sotto-progetti» intera, perché nessuno script la legge per posizione | livello 2 sui link, revisione sul resto |
| le righe di [`tracciabilita.md`](../../tracciabilita.md) | il riquadro in testa conta le funzionalità **col comando** e non con una cifra, quindi le righe nuove non falsificano niente; il controllo dei link sul resto | comando |
| F1–F9 in [`riferimenti.md`](../../riferimenti.md) | il controllo dei link; la data accanto a ogni fonte, come il resto del file | livello 2 sui link |
| lo spike SP-7 in `spikes/`, con l'esito in [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md) | i **criteri scritti prima** della misura (§4.2); `spikes/` è fuori dal workspace — `exclude = ["spikes"]` in `Cargo.toml` — quindi il cancello non lo compila, e non deve | criterio scritto prima |
| la sonda S3 in `crates/kernel/tests/arbiter_admission.rs` | `cargo test --locked` dentro `bash scripts/gate.sh`, **nelle due direzioni** (§4.2) | livello 2, cancello |
| questo file nella §12 del compendio e nella tabella «Dove va cosa» di [`README.md`](../../README.md), come i disegni del Traguardo 6 e della chiusura | il controllo dei link; e il **tetto** del compendio, che `check-docs.sh` misura in byte — il margine si legge con `wc -c docs/COMPENDIO.md` contro il valore che vive **nello script**, in una casa sola | livello 2 |

### 1.5 I «buchi logici», sciolti coi documenti del repo

**Il punto che scioglie quasi tutti: il core non dorme mai. Dorme la run.** Il core vive a lungo,
anche senza GUI (ADR-0004); i worker always-on vivono sotto di lui. La wake word non «sveglia il
programma»: **apre una run** (ADR-0011, corollario). Un gesto fa lo stesso se è un gesto di
**comando**; un gesto di **manipolazione** — un pinch che sposta un pannello — non apre nulla: è
presentazione, lo consuma la GUI, e non tocca mai il giornale, come i frammenti audio.

| Buco | Che cosa dice già l'architettura | Che cosa resta da decidere |
|---|---|---|
| «Se l'agente dorme, quando succede?» | il core è sempre sveglio; un evento di comando apre un passo in una run — e con la decisione 3 la run la apre **solo la wake word** | quali gesti sono **comando** e quali **manipolazione**: la lista è della capacità |
| «La foto va in che contesto?» | una cattura è un **artefatto**: file su disco, nel giornale solo il **riferimento** (ADR-0018, ADR-0022); entra nel contesto come **proiezione** (ADR-0008), nella run in corso | se vada **anche** nella knowledge base: brainstorming 2, decisione 7 — ✅ **sì, decisa il 2026-09-04**: la cartella della knowledge base **è** l'archivio, la foto atterra in un gruppo, la run riceve il riferimento (riga 7 della tabella delle decisioni) |
| «Una foto può dare ordini?» | no: è contenuto **non fidato**, informa e non autorizza (ADR-0014, I6). Un gesto non concede permessi (ADR-0016): un'azione invocata a gesti chiede lo stesso permesso che chiederebbe da tastiera | niente |
| «Il kernel decide sui gesti?» | no: il gesto è un evento, **dato opaco** (ADR-0020), smistato come la trascrizione che diventa messaggio (ADR-0011); passa dal registro **con lo stesso permesso** di ogni invocatore (ADR A). Il kernel resta **testabile senza modello**: l'evento si inietta a copione, come ogni porta (ADR-0021) | niente |
| «Chi tiene i fotogrammi?» | il worker, in Python, senza stato, uccidibile in ogni istante (ADR-0028, I5). Al core arrivano **eventi** | la forma dell'evento continuo sul canale `process`: §2.2 |
| «E la GPU?» | audio e presentazione sono **concessioni permanenti tenute dal core** (ADR-0033, montate dal Task 10 del Traguardo 5). Il porto `process` **pretende una concessione per avviare qualunque worker**: `Process::start(grant, descriptor)` (§5.6 della spec) | niente di nuovo oggi: il worker telecamera chiede una concessione da **zero MiB**, non prelazionabile (§2.2) |
| «Profilo riservato» | spegne la voce always-on (ADR-0023) | spegne **anche** la telecamera: un richiamo datato, §3.2 |
| «Telecamera assente o spenta» | si dichiara **prima**, non si fallisce dopo (ADR-0019) | niente: il campo di `Degradation` nasce col worker, §2.2 |

---

## 2. La forma nel kernel — ✅ approvata il 2026-09-03

Sotto accettazione condizionata, letta contro il sorgente di `3ec1ac2` il giorno dell'approvazione e
riletta contro `066008a` scrivendo il disegno — lo stesso codice, misurato: vedi il *Metodo* in testa.
I comandi stanno accanto alle affermazioni, e si rilanciano.

### 2.1 L'approccio, e le tre correzioni

**Approccio 1 — Sorgente di percezione sotto il core.**

| Pezzo | Forma |
|---|---|
| il worker | Python, always-on, sotto il core come sarà il microfono (ADR-0028). **Possiede la telecamera**; i fotogrammi non escono mai. MediaPipe su **CPU**: su Windows, in Python, la GPU non c'è (F2, e F9 lo conferma — §7) |
| il canale | il porto `process` (ADR-0035): **una** `instruct_stream` all'avvio, poi `read_next` per tutta la vita, come il worker audio descritto nella doc del porto. Ogni frame dichiara la propria lunghezza, `minicbor` (§6.10 della spec, ADR-0037) |
| le due specie di evento | lo **stato continuo della mano** (21 punti per mano, pinch derivato, a N Hz) e il **gesto discreto** (nome, confidenza). Sono **eventi, non passi** (ADR-0011): niente giornale |
| il core | li **smista**: la manipolazione va alla GUI con `Ipc::send`, transitoria, **campionata alla frequenza che il core decide** (§6.1.4 della spec); un gesto di **comando** prende **la strada della wake word** (ADR-0011) e solo lì nasce un passo |
| la concessione | da **zero MiB**, `Preemption::Never`, chiesta come le due permanenti di ADR-0033. **La formula di ADR-0005 non cambia** |
| «riservato» | spegne anche la telecamera (richiamo ad ADR-0023) |
| la GUI | **disegna la mano dai 21 punti**: niente video nella webview, quindi **nessuna misura in più** per ADR-0029 |

**Le tre correzioni**, nate dalla sfida del proprietario e dalla verifica nel sorgente:

| Prima diceva | Ora dice | Perché |
|---|---|---|
| «terza concessione permanente, oggi zero» come **terza quota** nella formula | **niente terza quota**: concessione da zero MiB come ogni worker; la formula resta | aggiungere oggi una quota che vale zero è **sfoggio** (criterio 5); il porto pretende comunque una concessione, quindi la forma minima c'è già. La terza quota si apre **quando esiste un tracciatore su GPU**: decisione 9 |
| «il gesto di comando diventa un **trigger** (ADR-0009)» | prende **la strada della wake word** (ADR-0011) | il trigger di ADR-0009 è l'innesco dell'**anello di verifica**; il posto giusto per «un evento apre un passo» è il corollario di ADR-0011. **E quella strada non esiste ancora nel codice**: la costruisce chi arriva primo, e con la decisione 11 è il sotto-progetto 12 (§5.3) — dipendenza dichiarata, non buco dell'approccio |
| «il core smista alla GUI» (dedotto) | **verificato**: `Ipc::send` è chiamato dal core quando decide, e campionare è una leva del kernel — doc di modulo di `crates/kernel/src/ports/ipc.rs` | la **latenza** resta **non misurata**: SP-7, domanda S2 |

**I due scartati, a confronto** — il perché per esteso nella sezione *«Vicoli ciechi»*:

| | 1 · worker sotto il core | 2 · dentro la GUI |
|---|---|---|
| always-on senza GUI | sì | no |
| coerenza con ADR-0011 e ADR-0004 | piena | rotta: la telecamera diventa stato della GUI, che è sacrificabile |
| latenza della manipolazione | un salto in più, **da misurare** | minima |
| costo di partenza | un worker Python in più | solo JS nella webview |
| se poi serve l'altro | niente da rifare | si riscrive tutto |

### 2.2 I pezzi, e dove vive ciascuno

| Pezzo | Forma | Dove vive |
|---|---|---|
| il worker | Python, possiede la telecamera, MediaPipe su CPU; senza stato, uccidibile in ogni istante (ADR-0028, I5); i fotogrammi non escono mai. ⚠️ ADR-0004 dice dei worker *«vita breve»*: il precedente dell'always-on è il worker **audio**, la cui ricevuta di stream resta aperta *«for its whole life»* (doc di modulo di `crates/kernel/src/ports/process.rs`), e ADR-0004 stesso elenca fra i requisiti strutturali *«Voce always-on con daemon in background»*. Ciò che regge sono **senza stato** e **uccidibile**, e il worker telecamera le ha entrambe | `workers/` alla radice, fuori da `crates/` (decisione 8), con un lockfile Python. ⚠️ La cartella **non esiste ancora**, e non la crea questo disegno: nasce col primo worker di prodotto, cioè col sotto-progetto 12 |
| il profilo | `ResourceProfile { name: <letterale>, reserved_vram: Mib::ZERO, compute_class: ComputeClass::Realtime, preemption: Preemption::Never }`, finestra `FOR_EVER` — la forma di `AUDIO_RESERVATION`: `grep -n -A5 'const AUDIO_RESERVATION' crates/daemon/src/main.rs` | letterale in `crates/daemon/src/main.rs` |
| la concessione | segue la vita del worker: entra con `Process::start(grant, descriptor)`, torna con `Killed.grant` ad `Arbiter::release`. Con la telecamera opt-in (decisione 4) si chiede all'**accensione**, non all'avvio del core | già così nel porto `process`: `grep -n -E '^\s*(pub trait\|fn )' crates/kernel/src/ports/process.rs` |
| il canale in su | `FromWorker` guadagna due varianti — lo stato della mano (21 punti per mano, coordinate **intere**) e il gesto (`kind`: enum chiuso `#[cbor(index_only)]`, `confidence`: intero) — a indici nuovi `#[n(2)]` e `#[n(3)]`, sotto le regole di §6.10 | `crates/kernel/src/wire/worker.rs`, che oggi ha `Fragment` a `#[n(0)]` e `VramPeak` a `#[n(1)]` |
| il canale in giù | la prima istruzione vera del canale, «traccia le mani», mandata una volta con `instruct_stream`; oggi la direzione core → worker **non ha nessun messaggio**, e la testa del file lo dichiara come non-costruzione col grilletto *«il primo processo worker vero»* | stesso file |
| il core | legge `read_next`, campiona alla frequenza che riceve come **parametro consegnato** (ADR-0034: un campo nuovo di `Parameters`, che oggi ne ha tre — `grep -n -A4 'pub struct Parameters' crates/kernel/src/parameters.rs` — letterale in `daemon`), manda alla GUI con `Ipc::send`; un gesto di **comando** prende la strada della wake word (ADR-0011) | il **primo lettore di produzione di una porta**: `grep -rn 'read_next\|\.receive(\|\.accept(' crates/kernel/src crates/daemon/src crates/platform/src` trova solo commenti e la firma del tratto |
| verso la GUI | una variante nuova di `IpcMessage` con la mano campionata; **si definisce quando la GUI esiste** (sotto-progetto 2), perché prima non ha destinatario — la regola già scritta per la revoca in testa a `crates/kernel/src/wire/ipc.rs`: *«a revocation needs an ADDRESSEE, and until milestone 2 of the subproject there is nobody to tell»* | `crates/kernel/src/wire/ipc.rs` |
| «riservato» | nel codice non esiste: `grep -rni riservato crates/` non rende niente. Spegnere la telecamera è un richiamo ad ADR-0023, e il meccanismo arriva col profilo | ADR-0023 |
| degrado | `Degradation` guadagna «telecamera assente o spenta» **solo quando il worker esiste**: la regola *«a field that is always `false` reads as "fine" rather than as "unknown"»* è già scritta in testa al tipo | `crates/kernel/src/degradation.rs` |

### 2.3 Tre regole di forma, verificate nel sorgente

| Regola | Perché, e il comando |
|---|---|
| **niente decimali nel kernel**: sul filo viaggiano interi, la conversione la fa il worker | `grep -rnw f32 crates/kernel/src` e `grep -rnw f64 crates/kernel/src` non rendono niente; MediaPipe dà coordinate normalizzate fra 0 e 1 (F4), e il worker le scala a interi |
| **nessun testo dal worker arriva a una decisione**: il gesto è un enum chiuso, non una stringa | il precedente è `GrantRequest` in `crates/kernel/src/wire/ipc.rs`, che attraversa il filo con **tre campi e nessun `name`** — `grep -n -A4 'pub struct GrantRequest' crates/kernel/src/wire/ipc.rs` — e il suo doc dice perché: *«`name` is the one field NO ARBITER DECISION READS»*, *«the name this type refuses»* — un testo scelto dal pari è contenuto non fidato (ADR-0014). Il **vocabolario** dei gesti resta della capacità (decisione 2); il **tipo** si fissa ora |
| **le regole del canale restano quelle di §6.10**: un `#[n(i)]` per campo, niente enum di versione, niente byte congelati, stringhe di byte annotate | testa di `crates/kernel/src/wire/worker.rs`: *«no version enum, no register of retired indices, NO FROZEN BYTES»* |

### 2.4 La telecamera sarebbe il PRIMO worker vero, e il primo paga

Verificato il 2026-09-03, ed è ciò che il codice dice e la consegna del brainstorming non diceva
prima di leggerlo:

| Che cosa manca | Dove lo dice |
|---|---|
| nessuna implementazione di `Process` o `Worker` fuori dai banchi: la piattaforma deve imparare ad avviare un processo e parlargli su una pipe | `grep -rln 'impl Process for\|impl Worker for' crates/` rende solo file di `tests/` — e un commento di `crates/kernel/src/ports/process.rs`, che nomina proprio questo comando |
| il canale worker ha una direzione sola, in su; in giù nessun messaggio, e il grilletto dichiarato è *«il primo processo worker vero»* | testa di `crates/kernel/src/wire/worker.rs` |
| il timbro di build non esiste: niente rifiuta un worker stantio; e la §6.10.7 della spec fa reggere il timbro su un ambiente Python **nostro e versionato** — quindi il lockfile in `workers/` non è cosmesi | stessa testa; spec, §6.10.7 |
| il reattore conosce solo il tempo (`now`, `wall_time`, `wait_until`): «pronto da leggere» per una pipe non c'è, e allargarlo è dichiarato meccanico nel file | `crates/kernel/src/ports/reactor.rs` |
| nessun codice di produzione legge una porta: il ciclo che legge lo stream nasce con questo | il comando della riga «il core» in §2.2 |

Non cambia l'approccio: è il **prezzo**, lo paga chi arriva primo fra voce e gesti — come la strada
della wake word — e con la decisione 11 chi arriva primo è il sotto-progetto 12. Va scritto nell'ADR B
come **costo dichiarato**, e la Voce lo **riusa** (§5.1).

### 2.5 Ipotesi che restano tali

Una riserva da `Mib::ZERO` passa l'ammissione: `Mib::ZERO` esiste — `crates/kernel/src/arbiter/resource.rs`,
`grep -n 'pub const ZERO' crates/kernel/src/arbiter/resource.rs` — ed è usato nelle somme dell'arbitro, e
nessun divieto su una riserva da zero è stato trovato. ⚠️ *Non trovato* non è *provato*: è la sonda S3
della §4.2, nel kernel e non nello spike.

---

## 3. Le decisioni in append — ✅ approvata il 2026-09-03

Sotto accettazione condizionata. Le righe degli ADR citate sono state lette il giorno
dell'approvazione e rilette oggi, coi comandi `grep -n -i quattro docs/adr/0001-*.md`,
`grep -n -i percettiv docs/adr/0011-*.md`, `grep -n -i riservato docs/adr/0023-*.md`,
`grep -n -i 'strumenti interni' docs/adr/0025-*.md`, `grep -n -i irripetibil docs/adr/0016-*.md` e
`grep -n -i registro docs/adr/0009-*.md`; gli ADR non sono cambiati dalla consegna (il *Metodo*).

### 3.1 I due ADR nuovi

| ADR nuovo | Decide | Negative (accettate) |
|---|---|---|
| **A — il registro delle funzioni del programma** | un registro unico di **strumenti interni** (il livello 1 di ADR-0025), meccanismo di kernel nella forma di ADR-0009: il kernel dà registrazione, invocazione, il permesso come tripla di ADR-0016 e il giornale; le capacità e la GUI portano le funzioni. **Molti invocatori** — agente, gesto, voce, click — **con lo stesso permesso**, e nessuna logica «solo per gesti». Un gesto è un evento di percezione: **informa, mai autorizza** (ADR-0014 per analogia). Un effetto irripetibile chiede conferma a qualunque invocatore (ADR-0016, già così), e **per default la conferma non è gestuale** (decisione 6). La manipolazione della GUI — pannelli, menu virtuali — è presentazione e **non passa dal registro**. Quali funzioni siano gestuali lo decide la capacità (decisione 2) | un meccanismo di kernel in più prima di ogni capacità; ogni funzione con effetto va dichiarata come tripla; lo stesso permesso pesa anche sulle funzioni banali invocate dalla GUI |
| **B — la telecamera come sorgente di percezione always-on sotto il core** | la forma della §2 per intero. Più: telecamera **spenta per default**, e accenderla è una funzione del registro (decisione 4); **solo la wake word apre una run**, un gesto di comando entra come passo in una run aperta (decisione 3); «riservato» la spegne; il campo di `Degradation` nasce col worker. **Non-costruzioni dichiarate:** il messaggio alla GUI aspetta il sotto-progetto 2; la terza quota aspetta un tracciatore su GPU (decisione 9) | un processo Python su CPU finché la telecamera è accesa; un salto in più sulla manipolazione, misurato da SP-7; il conto del **primo worker** (§2.4); niente GPU su Windows per il tracciatore (F2, F9) |

⛔ **Un ADR senza `Negative (accettate)` è incompleto** (`CLAUDE.md`): le due colonne di destra
entrano negli ADR come sono, e il piano non le accorcia.

### 3.2 I tre richiami datati

| ADR | La riga, letta oggi | Il richiamo |
|---|---|---|
| ADR-0001 | *«quattro aree — conversazione, agenti/coding, voce, …»* (riga 9) e i consumatori paritari (riga 30) | «voce» si legge **«voce e gesti»**; i pilastri restano quattro, **nessun ADR superato** — è la decisione 1, e la clausola approvata con la sezione 1: con un quinto pilastro sarebbe servito un ADR che **supera** ADR-0001 |
| ADR-0011 | la tabella *«inferenza percettiva always-on»*, esempi *«wake word, VAD, trascrizione continua»* (righe 57–64) | entra **il tracciamento delle mani**; un gesto di comando fa come la trascrizione che diventa messaggio — il gesto apre un passo, i fotogrammi no |
| ADR-0023 | punto 5, *«disattiva avvio automatico e voce always-on»* (riga 65) | entra **«e la telecamera»** |

📌 **La forma è quella di AUD-032:** il richiamo va **in testa all'ADR**, append-only, con la data; la
voce dell'ADR nella §5 del compendio vi **rimanda** invece di ricopiarlo. I numeri di riga qui sopra sono
quelli del 2026-09-03 e servono a ritrovare la frase col `grep`, non a citarla per posizione.

⚠️ **RICHIAMO DEL 2026-09-04:** *«la forma è quella di AUD-032»* è falso **sull'attribuzione**, non sulla
scelta. Il rimando di AUD-032 non sta in testa: vive **dentro la Decision** di ADR-0005, accanto al paragrafo
che corregge — `grep -n 'Rimando del 2026-08-27' docs/adr/0005-arbitrato-gpu-su-due-dimensioni.md` letto contro
`grep -n '^## ' docs/adr/0005-arbitrato-gpu-su-due-dimensioni.md` — come il rimando di AUD-004 in ADR-0015, che
sta nella Context, e quello di AUD-033 in ADR-0036, che sta nelle Consequences: **un richiamo si legge dove si
legge la frase che corregge**, in qualunque sezione viva. La posizione **in testa** resta quella giusta per i
tre richiami di questo disegno, perché correggono l'**ADR intero** e non una frase — ma è una scelta di questo
lavoro, non «la forma di AUD-032». La stessa attribuzione sta nella tabella della §1.4 e nella trappola 8, e
tutte e due rimandano qui.

### 3.3 Che cosa NON cambia

**ADR-0005 e ADR-0033 non cambiano**: la formula del budget resta a due quote sottratte, e la terza
resta voce registrata — decisione 9. **ADR-0009 non cambia**: il trigger dell'anello di verifica non è
la strada di un gesto di comando (§2.1, seconda correzione).

### 3.4 Le decisioni prese qui, e quelle registrate

**Prese con questa sezione**, sotto accettazione condizionata: la **1** → (a) dentro Voce; la **3** →
(a) solo la wake word sveglia; la **4** → (a) opt-in; la **6** → (a) mai da solo, e la conferma non è
gestuale per default.

**Registrate, non prese:** dove si salva l'interruttore della telecamera fra un avvio e l'altro — la
decisione **10**: è l'archivio dei parametri (ADR-0034, ADR-0022), che non esiste, e lo chiude chi lo
costruisce; e la posizione della capacità nella roadmap, presa poi nella §5 come decisione **11**.

---

## 4. La GUI, il sotto-progetto 2 e lo spike SP-7 — ✅ approvata il 2026-09-03

Letta contro [`spikes/GUI-REQUISITI.md`](../../../spikes/GUI-REQUISITI.md) (G6, G20, P1–P3), le
righe M1–M5 di ADR-0029, il consumatore 1 della tabella di ADR-0033 e il formato di
[`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md).

### 4.1 La mano sullo schermo

| Pezzo | Forma | Da dove |
|---|---|---|
| la mano sullo schermo | la GUI **disegna la mano dai 21 punti**, in un livello sopra i pannelli; **niente video** (decisione 5) | il disegno è compositing della webview: sta **dentro la quota di presentazione**, nessuna concessione da chiedere — ADR-0033, consumatore 1 |
| l'indicatore | un segno **sempre visibile** quando la telecamera è accesa; lo accende il **core** con un messaggio, la GUI non lo indovina | lo spirito di ADR-0023: *una falsa sicurezza è peggio di nessuna sicurezza* |
| pannelli e menu | il sotto-progetto 2 costruisce pannelli e menu che si muovono con **qualunque puntatore**; la mano è un puntatore in più, e la aggiunge la capacità (pinch che trascina, menu virtuali). Solo stato di presentazione | ADR-0004: la GUI possiede solo presentazione. Il messaggio IPC con la mano si definisce da lì in poi, non prima (§2.2) |
| accessibilità | un gesto **non è mai l'unica strada**: ogni funzione gestuale si raggiunge anche da tastiera e click | G20; segue da «un registro, molti invocatori» (ADR A) |
| ADR-0029 | **nessuna misura in più**; ma **M4** — «P3 con rendering vero» — deve includere la mano disegnata a 30 Hz quando la si lancia: P3 è già stretto, e quanto lo dice la tabella degli esiti di `spikes/GUI-REQUISITI.md`, riga P3 | ADR-0029 righe M1–M5; GUI-REQUISITI P3 |

### 4.2 Lo spike SP-7, e la sonda S3

**Lo spike SP-7** — usa e getta, in `spikes/`, fuori dal workspace (`exclude = ["spikes"]` in
`Cargo.toml`). I criteri si scrivono **prima** di misurare; l'esito va in `spikes/RISULTATI.md` con le
versioni degli strumenti, la CPU della macchina e le evidenze, nella forma di SP-5 e SP-6 — le sezioni
*Esito*, *Osservazioni registrate — non criteri*, *Versioni degli strumenti*, *Evidenze*:
`grep -n -E '^## ' spikes/RISULTATI.md`. Il nome è il prossimo libero: gli spike esistenti sono SP-1
… SP-6.

| # | Domanda | Criterio, scritto prima | Come |
|---|---|---|---|
| S1 | MediaPipe Hand Landmarker su CPU regge 30 Hz su questa macchina? | tempo per fotogramma **< 33 ms** — mediana e p95 riportati — a due mani, 640×480, modo LIVE_STREAM. Il margine si **riporta**, non si promette | Python, `mediapipe` 1.0.1 (F1), telecamera vera |
| S2 | quanto costa il giro worker → core → GUI a 30 Hz? | latenza da cattura a disegno, mediana e p95; il solo salto core → GUI contro il numero che il repo ha già, **P2 < 100 ms**. L'accettabilità della mano sul pannello la giudica il **proprietario provandola**: nessuna soglia inventata | Python + un relay Rust usa e getta + una pagina che disegna i 21 punti |
| S3 | una riserva da zero MiB passa l'ammissione? | `Admission::Granted` con `reserved_vram: Mib::ZERO`, **anche a macchina piena**; e la contro-sonda: una riserva vera a macchina piena resta `Queued` | **sonda nel kernel**, `crates/kernel/tests/arbiter_admission.rs` — non spike. ⚠️ `Admission` non deriva `Debug` né `PartialEq`, perché `Grant` non li ha e non deve averli: le asserzioni sono `matches!` e `let … else`, come ogni sonda dell'arbitro (disegno del Traguardo 5). ⚠️ La sonda **non ha una riga di catalogo**: la §7.4 è spec (vincolo globale 7), quindi si **registra** e non si prende — stesso trattamento di PL-1 e di K-1/B-1, stessa ragione (gotcha #36) |

### 4.3 Registrata, non presa

Se la posizione dei pannelli sopravvive a un riavvio — la decisione **12**: è configurazione, archivio
di ADR-0022, che non esiste; la chiude chi costruisce l'archivio.

---

## 5. Voci aperte, dipendenze e prossimo passo — ✅ approvata il 2026-09-03

Sotto accettazione condizionata. Letta contro la tabella «Sotto-progetti» e il «Perché quest'ordine» di
[`roadmap.md`](../../roadmap.md), e la sezione 6 di [`tracciabilita.md`](../../tracciabilita.md).

### 5.1 Dove va la capacità: il sotto-progetto 12 «Gesti»

**Decisione 11: una riga nuova, il sotto-progetto 12 «Gesti».** Non dentro «Voce»: la roadmap mette
Voce dopo Generazione asset perché SP-2 vuole *voce e job GPU pesante insieme*, e i gesti non usano la
GPU — quella ragione non li riguarda. Si appende **senza rinumerare**, come 0b, 0c e 11. Il pilastro
resta «voce e gesti» (richiamo ad ADR-0001), costruito in due sotto-progetti come il kernel lo è in
quattro.

| Riga della roadmap | Dipende da | Perché |
|---|---|---|
| **12 — Gesti**, L2 | 2 e 3 | dal 2 i pannelli mobili e il registro delle funzioni (ADR A); dal 3 la run che un gesto di comando comanda |
| **8 — Voce** | 7, e ora anche **12** | riusa ciò che il primo worker paga (§2.4): trasporto di `process`, messaggio in giù, timbro di build, prontezza del reattore, ciclo di lettura |
| **2 — GUI minima** | invariata | ma è il **primo invocatore** del registro di ADR A, col click: chi arriva primo lo costruisce |

### 5.2 Le righe di tracciabilità

Nella sezione 6, che si intitola «Voce e gesti», con sede «Gesti»: tracciamento delle mani, gesti di
comando, manipolazione di pannelli e menu con le mani (sede GUI + Gesti), cattura con un gesto (Gesti +
brainstorming 2 — ✅ **decisa il 2026-09-04**, riga 7 della tabella), indicatore di telecamera accesa (GUI). Il registro delle funzioni va accanto a
«Comandi rapidi e slash-command», sede GUI, come meccanismo deciso.

### 5.3 Le dipendenze dichiarate

| Cosa | Da chi dipende |
|---|---|
| dove finisce la **cattura** (decisione 7) | il brainstorming 2, la knowledge base — ✅ **sciolta il 2026-09-04**: nella knowledge base, la run la vede (riga 7 della tabella delle decisioni) |
| la strada «un evento di percezione apre un passo» (ADR-0011) | la costruisce il **12**, primo con una sorgente di percezione; la voce la riusa |
| l'interruttore della telecamera e la posizione dei pannelli (decisioni 10 e 12) | l'archivio dei parametri, che nessun sotto-progetto colloca ancora — registrato |
| il timbro di build sui due canali (§6.1.2) | GUI col **2**, worker col **12**: i due grilletti già scritti in testa a `crates/kernel/src/wire/ipc.rs` e `crates/kernel/src/wire/worker.rs` |

### 5.4 Le voci che restano aperte

Tutte con un chiusore scritto nella tabella delle decisioni: la **2** (quali funzioni sono
gestuali → la capacità), la **7** (la cattura → brainstorming 2 — ✅ **chiusa il 2026-09-04**, riga 7 della tabella), la **9** (la terza quota → un
tracciatore su GPU), la **10** e la **12** (→ l'archivio dei parametri). **Nessuna sbarra il
disegno**, né il piano.

### 5.5 L'ordine di ciò che segue

⛔ **Il prossimo passo vive nella §6 del compendio, in un posto solo.** Qui sta l'ordine che la sezione
ha approvato, con le spunte di oggi:

1. ✅ la sezione 5 approvata **chiude il brainstorming** il 2026-09-03.
2. ✅ la sessione successiva scrive il disegno **sul posto, in questo file** — fatto il 2026-09-03
   (regola del proprietario del 2026-09-02).
3. ✅ il **proprietario rilegge** questo disegno — fatto il 2026-09-03, sotto **accettazione
   condizionata**: *«se è coerente col progetto e col codice e si allinea a decision-principles»*.
   La verifica è scritta nella sezione *«Cosa questo disegno ha misurato»*, e regge.
4. ✅ **il piano è scritto il 2026-09-03** con `superpowers:writing-plans` — [`plans/2026-09-03-riconoscimento-gesti.md`](../plans/2026-09-03-riconoscimento-gesti.md), nove compiti,
   pre-controllo fatto; **eseguito il 2026-09-04**, nove compiti su nove; l'esecuzione è chiusa, e il verbale
   per compito sta nella tabella della posizione del piano. Compiti attesi: i due ADR — ciascuno con la propria
   voce nella §5 del compendio, che `check-docs.sh` pretende — e i tre richiami datati; la riga 12 e le
   dipendenze in `roadmap.md`; le righe di `tracciabilita.md`; le fonti F1–F9 in `riferimenti.md`, con
   F8 risalita a OpenMMLab e il motivo della chiusura di F9 — **entrambi letti scrivendo questo
   disegno**, §7; lo spike SP-7 in `spikes/` con l'esito in `spikes/RISULTATI.md`; la sonda S3 nel
   kernel; questo file nella §12 del compendio. Esecuzione con `superpowers:subagent-driven-development`.
5. poi il brainstorming **distinto** della knowledge base; poi il sotto-progetto 2.

✅ **La Definizione di «fatto» dell'esecuzione — proposta dallo scrivente, APPROVATA il 2026-09-03 con
la rilettura del disegno**, sotto la stessa accettazione condizionata. È l'elenco del punto 4 letto come
condizioni, nella forma che i disegni dei Traguardi 5 e 6 danno alla propria chiusura; il piano la copia
**da qui**. ⚠️ Questo capoverso diceva *«proposta, non approvata»*: la rilettura l'ha approvata, e la
riga si riscrive invece di ricevere una nota sotto.

| # | Condizione | Chi la verifica |
|---|---|---|
| 1 | i due ADR esistono in `docs/adr/`, `Accepted`, ciascuno con `Negative (accettate)` e con la propria voce in §5 del compendio; i totali degli ADR nei documenti di stato sono aggiornati nello stesso commit (§1.4) | `bash scripts/check-docs.sh` → `OK` |
| 2 | i tre richiami datati stanno in testa ad ADR-0001, ADR-0011 e ADR-0023, append-only, e le voci di §5 del compendio vi rimandano | la revisione, leggendo i tre ADR |
| 3 | `roadmap.md` porta la riga 12 e la dipendenza nuova della riga 8, **senza rinumerare**; il «Perché quest'ordine» è riletto contro la riga nuova | la revisione, leggendo la tabella intera |
| 4 | `tracciabilita.md` porta le righe della §5.2 e il titolo nuovo della sezione 6; il comando del riquadro in testa gira e conta | il comando del riquadro |
| 5 | `riferimenti.md` porta F1–F9 con le date, F8 risalita a OpenMMLab e F9 col motivo della chiusura, presi dalla §7 di questo disegno | il controllo dei link, e la revisione |
| 6 | SP-7: i criteri S1 e S2 scritti in `spikes/` **prima** della misura; l'esito in `spikes/RISULTATI.md` nella forma di SP-5 e SP-6, con versioni, CPU e evidenze; il giudizio sulla mano che muove un pannello è del **proprietario che la prova** | il proprietario, e la forma di `RISULTATI.md` |
| 7 | S3: la sonda in `crates/kernel/tests/arbiter_admission.rs`, nelle **due direzioni**, dentro il cancello | `bash scripts/gate.sh` → `GATE GREEN` |
| 8 | questo file nella §12 del compendio e nella tabella «Dove va cosa» di `README.md`; il compendio sotto il tetto; il puntatore della §6 mosso al passo successivo | `check-docs.sh`, e `wc -c docs/COMPENDIO.md` contro il tetto nello script |
| 9 | i fine-riga di ogni file toccato **rimisurati** dopo la scrittura, con `git ls-files --eol` e `tr -cd '\r' \| wc -c` | chi esegue, a ogni compito |

---

## 6. Verificato, dedotto, assunto

### 6.1 Verificato nel sorgente, il 2026-09-03

Letto il giorno dell'approvazione e riletto scrivendo il disegno contro `066008a`, con gli stessi
comandi e gli stessi esiti.

| Che cosa | Dove, e il comando |
|---|---|
| il canale `process` **regge lo stream per costruzione**: *«The audio worker keeps a stream receipt open for its whole life, opened by a single instruction at start-up»*; *«A STREAM RECEIPT IS NOT A JOURNAL STEP … a SOURCE OF EVENTS, not steps (ADR-0011)»*; *«EVERY BYTE THAT FLOWS BACK IS COVERED BY A RECEIPT»* | doc di modulo di `crates/kernel/src/ports/process.rs`; il tratto `Worker` ha `instruct_one`, `instruct_stream`, `read_one`, `read_next`, `close`, `kill`; `Process::start(grant, descriptor)` — `grep -n -E '^\s*(pub trait\|fn )' crates/kernel/src/ports/process.rs` |
| il canale `ipc` ha il verso **core → GUI**, e *«the core decides WHEN to emit, and the gui does not pull»*; *«aggregating, sampling or coalescing updates is a KERNEL choice»* | doc di modulo di `crates/kernel/src/ports/ipc.rs`; il tratto `Ipc` ha `accept`, `send`, `receive` |
| **nessun meccanismo «un evento apre una run» esiste nel codice**: la decisione c'è (ADR-0009 trigger, ADR-0011 sorgenti di eventi), il meccanismo no | `grep -rn -E 'fn (open\|start\|begin)_?run\|pub struct Run\b\|enum Trigger\|struct Trigger\|trait Trigger' crates/kernel/src` non rende niente |
| `Mib::ZERO` esiste ed è usato nelle somme dell'arbitro; **nessun divieto trovato** su una riserva da zero | `grep -n 'pub const ZERO' crates/kernel/src/arbiter/resource.rs`; `grep -n -i -E 'Mib\(0\)\|zero' crates/kernel/src/arbiter/*.rs`. ⚠️ *Non trovato* non è *provato*: S3 |
| **non esiste un registro delle funzioni del programma**: l'unica riga vicina è ADR-0025, *«il livello 1 resta ammesso solo per strumenti interni che non eseguono codice»* | `grep -n -i -E 'strument[oi] intern\|registro degli strumenti\|palette\|scorciatoi' docs/superpowers/specs/2026-08-06-kernel-design.md docs/adr/*.md` |
| le righe di `tracciabilita.md` vicine: *Comandi rapidi e slash-command → GUI*, *Wake word → Voce*, *Screenshot e comprensione dello schermo → L3 + Conversazione*, *Hotkey globale, tray e clipboard → L3*, *Overlay/finestra fluttuante → GUI + L3*, *Convivenza pipeline audio ↔ job GPU ✅ quota sottratta (ADR-0005)*. Le sezioni: 6 Voce, 7 Multimodalità e generazione, 8 Sistema | `grep -n '^## ' docs/tracciabilita.md`, e `grep -n -i -E 'comand\|wake\|screenshot\|hotkey\|overlay\|audio' docs/tracciabilita.md` |
| la roadmap: sotto-progetto **2** GUI minima (dipende da 1 e ADR-0027), **8** Voce (L2, dipende da 7, chiude lo spike SP-2), **10** Integrazione OS completa (L3, dipende da 2); l'ultima riga è la **11**, e il sotto-progetto 1 vi è ✅ chiuso | `grep -n -E '^\| \*{0,2}[0-9a-z]{1,3}\*{0,2} \|' docs/roadmap.md` |
| gli spike esistenti sono SP-1 … SP-6 in `spikes/RISULTATI.md`: il prossimo si chiama **SP-7** | `grep -n -E '^## ' spikes/RISULTATI.md` |
| la telecamera **non era mai stata valutata** nel repo | scoperta 4 del disegno della chiusura, col suo `grep` |
| il codice **non è cambiato** fra la consegna e questo disegno | `git diff --stat c8e234e..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende niente |

### 6.2 Verificato nello stato dell'arte, il 2026-09-03 — le fonti in §7

| Fatto | Fonte |
|---|---|
| MediaPipe è **mantenuto**: `mediapipe` 1.0.1 su PyPI, rilasciato il **2026-08-14**; Python 3.9–3.12; ruote per Windows x86-64 e ARM64; Apache 2.0. Su GitHub `v1.0.0` del 2026-07-28 | F1, F7 |
| in Python la GPU **non c'è su Windows**: *«GPU support is currently limited to Ubuntu platforms»*. E l'issue che lo chiedeva è stata chiusa **per inattività**, non perché risolta: un collaboratore scrisse il 2024-02-09 *«Windows support is not yet available»* | F2, F9 |
| Hand Landmarker: **21 punti** per mano, coordinate normalizzate e in metri; su Pixel 6 **17,12 ms** su CPU e **12,27 ms** su GPU; opzioni `num_hands`, tre soglie di confidenza | F4 |
| Gesture Recognizer: modi IMAGE, VIDEO, **LIVE_STREAM** (risultati **per callback**, non bloccante); **otto** gesti pronti: `None`, `Closed_Fist`, `Open_Palm`, `Pointing_Up`, `Thumb_Down`, `Thumb_Up`, `Victory`, `ILoveYou`; gesti propri con `custom_gestures_classifier_options` | F3 |
| nel browser: JS su **WebGL**; **WebGPU non supportato** (issue aperta dal 2025-01-15, in attesa di Google); `detectForVideo` **blocca il thread della UI**, servono web worker | F5, F6 |
| alternativa **esistente e non archiviata**: **RTMPose**, progetto dentro `open-mmlab/mmpose`, Apache 2.0, con modelli per la mano a **21 punti** su cinque dataset, esportazione **ONNX e TensorRT** via MMDeploy, istruzioni per Windows e per ONNX Runtime. ⚠️ **La sua manutenzione è più debole di come la consegna la diceva:** l'ultimo rilascio di mmpose è `v1.3.2` del **2024-07-12**, l'ultimo push del **2025-08-04**, e le novità della pagina di RTMPose si fermano a **dicembre 2023**. Se un giorno serve la GPU su Windows la via è **ONNX Runtime**, e quel giorno la manutenzione si **rimisura**. Ultraleap Gemini e OpenXR sono hardware dedicato, fuori perimetro | F8 |

### 6.3 Dedotto, e dichiarato tale

- **Il pinch non è fra i gesti pronti**: si ricava dalla distanza fra la punta del pollice e quella
  dell'indice. È la pratica corrente nei progetti che usano i 21 punti; **non è una fonte primaria**.
- Il carico dello stato continuo sul canale è **piccolo**: 21 punti per tre coordinate per due mani
  sono nell'ordine del mezzo kilobyte a fotogramma, cioè decine di kilobyte al secondo a 30 Hz. Ordine
  di grandezza, non misura.
- Le coordinate viaggiano **intere** perché il kernel non ha decimali (§2.3); che la scala scelta dal
  worker basti al disegno della mano è una deduzione dal fatto che lo schermo è a pixel interi: la
  scala la fissa il piano del sotto-progetto 12, non questo disegno.

### 6.4 Assunto, e lo misura SP-7

1. Il giro **worker → core → GUI a 30 Hz** su questa macchina ha una latenza accettabile per spostare
   un pannello con la mano. **Non misurato** → S2.
   ⚠️ **Misurato il 2026-09-04:** l'assunto **regge sul giudizio del proprietario**, e il criterio passa — ma misura il salto relay → pagina, che non era il rischio: il costo vive a monte, e una parte non ha una spiegazione misurata — l'esito in [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md), sezione SP-7.
2. MediaPipe su CPU su questa macchina regge **30 Hz** con margine: i 17 ms del Pixel 6 lo fanno
   sperare, non lo provano. **Non misurato** → S1.
   ⚠️ **Misurato il 2026-09-04:** l'assunto **cade** — su questa CPU il tracciatore a due mani non ha margine sui 30 Hz, e il criterio è bocciato su due corse indipendenti; la speranza del Pixel 6 non regge — l'esito in [`spikes/RISULTATI.md`](../../../spikes/RISULTATI.md), sezione SP-7.
3. Una riserva da **zero MiB** passa l'ammissione. **Sonda nel kernel**, non spike → S3.
   ⚠️ **Misurato il 2026-09-04:** l'assunto **regge** — la sonda S3 è nel cancello dal compito 6 del piano, nelle due direzioni.

---

## 7. Le fonti — verificate il 2026-09-03

⚠️ **Da portare in [`riferimenti.md`](../../riferimenti.md) col piano**, che è il momento in cui
`CLAUDE.md` le vuole tracciate lì (§5.5, punto 4). Qui stanno con la data perché non vadano perse, e
perché **F8 e F9 sono state rilette oggi** — le due righe che la consegna lasciava da leggere.

| | Fonte | Che cosa sostiene |
|---|---|---|
| F1 | PyPI, `mediapipe` — https://pypi.org/project/mediapipe/ | versione 1.0.1 del 2026-08-14, Python 3.9–3.12, ruote Windows, Apache 2.0 |
| F2 | Google AI Edge, `BaseOptions` — https://ai.google.dev/edge/api/mediapipe/python/mp/tasks/BaseOptions | *«GPU support is currently limited to Ubuntu platforms»* |
| F3 | Google AI Edge, Gesture Recognizer per Python — https://developers.google.com/edge/mediapipe/solutions/vision/gesture_recognizer/python | modi, callback in LIVE_STREAM, gli otto gesti, i gesti propri |
| F4 | Google AI Edge, Hand Landmarker — https://developers.google.com/edge/mediapipe/solutions/vision/hand_landmarker | 21 punti, coordinate, latenze su Pixel 6, opzioni. ⚠️ **RICHIAMO DEL 2026-09-04:** questa riga non diceva **quale bundle**, e lo spike SP-7 ha usato il `float16`: l'URL, riletto alla fonte il giorno della misura, sta nella tabella delle versioni di SP-7 in `spikes/RISULTATI.md`, in una casa sola. È la proposizione che la riga F4 di [`riferimenti.md`](../../riferimenti.md) porta già |
| F5 | Google AI Edge, Hand Landmarker per il Web — https://developers.google.com/edge/mediapipe/solutions/vision/hand_landmarker/web_js | `detectForVideo` sincrono, web worker |
| F6 | GitHub, issue WebGPU support for Vision Tasks — https://github.com/google-ai-edge/mediapipe/issues/5826 | aperta dal 2025-01-15, WebGL oggi |
| F7 | GitHub, release — https://github.com/google-ai-edge/mediapipe/releases | `v1.0.0` del 2026-07-28, `v0.10.35`, `v0.10.33` |
| F8 | **Risalita alla fonte primaria il 2026-09-03:** OpenMMLab, RTMPose in `mmpose` — https://github.com/open-mmlab/mmpose/tree/main/projects/rtmpose — e il repository https://api.github.com/repos/open-mmlab/mmpose con l'ultimo rilascio https://api.github.com/repos/open-mmlab/mmpose/releases/latest | Apache 2.0, non archiviato; sezione *«Hand 2d (21 Keypoints)»* su COCO-Wholebody-Hand, OneHand10K, FreiHand2d, RHD2d e Halpe; *«How to Deploy»* con ONNX e TensorRT via MMDeploy; sezione Windows e `mmdeploy_runtime` su onnxruntime. ⚠️ Ultimo rilascio `v1.3.2` del 2024-07-12, ultimo push 2025-08-04, novità della pagina ferme a dicembre 2023. 📌 La consegna citava una fonte **di terza mano** — Forasoft, https://www.forasoft.com/learn/ai-for-video-engineering/articles-ai/openpose-mediapipe-rtmpose-pose-tracking — che diceva *«mantenuta»*: la parola **non regge** letta alla fonte, e qui è sostituita |
| F9 | GitHub, issue *GPU Delegate is not yet supported for Windows* — https://github.com/google-ai-edge/mediapipe/issues/5126, letta il 2026-09-03 via https://api.github.com/repos/google-ai-edge/mediapipe/issues/5126 e i suoi commenti | aperta il 2024-02-08, **chiusa il 2024-02-25 dal bot per inattività** — *«This issue was closed due to lack of activity after being marked stale for past 7 days»* — con `state_reason: completed` che è la parola del bot, non una risoluzione. L'unico commento di un collaboratore, del 2024-02-09: *«GPU support in Python is available for Regular Linux and macOS from version 0.10.8 onwards. Unfortunately, Windows support is not yet available.»* — F9 **rafforza** F2 |

---

## Cosa questo disegno ha misurato, e che non era scritto da nessuna parte

| # | Misurato il 2026-09-03 | Che cosa ne segue |
|---|---|---|
| 1 | **F9 è stata chiusa dal bot per inattività**, non perché la GPU su Windows fosse arrivata; il collaboratore lo disse in chiaro il 2024-02-09 | la frase *«MediaPipe su CPU: su Windows in Python la GPU non c'è»* ha ora **due** fonti concordi invece di una. Nessuna decisione cambia |
| 2 | **F8 risalita a OpenMMLab:** RTMPose esiste, è Apache 2.0, ha i modelli della mano a 21 punti e l'esportazione ONNX/TensorRT — ma l'ultimo rilascio è del 2024-07-12 e la pagina è ferma a dicembre 2023 | *«alternativa mantenuta»* diventa *«alternativa esistente, la cui manutenzione si rimisura il giorno che serve»*. Oggi nessuna decisione ne dipende: la GPU per il tracciamento è la decisione 9, registrata |
| 3 | il codice **non è cambiato** fra la consegna e il disegno: `git diff --stat c8e234e..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` è vuoto | le verifiche della consegna valgono per il codice di oggi; sono state comunque **rilanciate**, non citate |
| 4 | il cancello era **dichiarato non rilanciato** dalla consegna; rilanciato oggi, `bash scripts/gate.sh` → `GATE GREEN`, `bash scripts/check-docs.sh` → `OK`, **prima** di toccare un file | la baseline da cui il piano parte è verde, e si **rimisura** all'apertura del piano invece di leggersi qui |
| 5 | il disegno del Traguardo 6 entrò nella §12 del compendio e in `README.md` **alla chiusura del sotto-progetto**, non quando fu scritto: `git log --format='%h %ad' --date=short -S'traguardo-6-altri-meccanismi-design' -- docs/COMPENDIO.md docs/README.md` dà il primo commit al 2026-09-03, contro un file nato il 2026-08-29 | «questo file nella §12» resta un **compito del piano**, come la consegna prevedeva: questo disegno **non** vi si aggiunge da solo. Il puntatore della §6 del compendio, invece, è mosso oggi, perché il prossimo passo è cambiato |
| 6 | il compendio è **uniformemente CRLF** nell'albero di lavoro e LF nell'indice; ventuno file di `docs/adr/` su trentasette sono CRLF nell'albero e tutti LF nell'indice: `git ls-files --eol docs/COMPENDIO.md docs/adr/` | chi scrive gli ADR nuovi li scrive **LF**, e chi tocca il compendio conserva i CR e li rimisura: oggi `tr -cd '\r' < docs/COMPENDIO.md \| wc -c` è uguale a `wc -l`, prima e dopo la modifica |
| 7 | `GrantRequest` attraversa il canale `ipc` con **tre campi e nessun `name`**: `grep -n -A4 'pub struct GrantRequest' crates/kernel/src/wire/ipc.rs` | è il precedente vivo della regola *«nessun testo dal worker arriva a una decisione»* (§2.3), col file esatto invece di un nome nudo |
| 8 | **La verifica di coerenza chiesta dal proprietario rileggendo** — *«se è coerente col progetto e col codice e si allinea a decision-principles»* — fatta il 2026-09-03 contro le sei invarianti e contro ADR-0004, 0009, 0011, 0014, 0016, 0019, 0020, 0021, 0025, 0028, 0031, 0033, 0034, 0035, 0037, coi comandi di questa sezione e della §6. **Regge**, con tre letture che il disegno non scriveva e ora scrive: la *«vita breve»* di ADR-0004 (§2.2, worker), il gesto come dato opaco di ADR-0020 (§1.5), la sonda S3 senza riga di catalogo (§4.2) | nessuna decisione cambia; le tre letture sono scritte dove il lettore le cerca |
| 9 | **una lacuna trovata dalla stessa verifica:** nessun ADR decide se il processo del worker sia **ristretto** (livello 2 di ADR-0025) oltre che separato (ADR-0028), e `WorkerDescriptor` è byte opachi: `grep -n -A2 'pub struct WorkerDescriptor' crates/kernel/src/ports/process.rs` | la decisione **13**, registrata col suo chiusore; non sbarra il piano |
| 10 | la roadmap segna l'**ulteriore strato** quando un sotto-progetto tocca il kernel — `grep -n -E 'L1 est\|L0 \+ L3' docs/roadmap.md` rende le righe 9 e 11 — e la riga 12 approvata dice **L2** mentre paga lavoro di kernel (§2.4) | la voce **4** per il proprietario: una parola, col consiglio scritto |

---

## Le voci che questo disegno apre per il proprietario

| # | Voce | Perché è sua, e il consiglio |
|---|---|---|
| 1 | la tabella dei controlli per artefatto (§1.4) e la Definizione di «fatto» (§5.5) sono **aggiunte dello scrivente** | ✅ **Approvate il 2026-09-03 con la rilettura del disegno**, sotto accettazione condizionata: il piano le copia da qui. Il merito viene dalle sezioni approvate, la forma no, ed è per questo che erano portate al proprietario invece di essere date per approvate |
| 2 | F8 più debole di come era scritta | nessuna decisione ne dipende oggi. Consiglio: **non** cercare ora una seconda alternativa a MediaPipe — sarebbe lavoro per una voce registrata (la 9), cioè sfoggio; si rimisura quando la voce si apre |
| 3 | le decisioni **2, 7, 9, 10, 12**, aperte con un chiusore scritto | nessuna sbarra il disegno né il piano; restano nella tabella delle decisioni, in una casa sola. ✅ **Richiamo del 2026-09-05: la 7 è chiusa** — decisa il 2026-09-04 dal brainstorming della knowledge base, riga 7 della tabella delle decisioni |
| 4 | l'**etichetta di strato** della riga 12 della roadmap: la sezione approvata dice **L2**, ma la riga paga anche lavoro di kernel (§2.4), e la roadmap segna l'ulteriore strato quando c'è — la riga 9 è *«L1 est.»*, la 11 *«L0 + L3»* | una parola, presa **scrivendo la riga** nel piano. Consiglio: **«L2 + L1 est.»**, sulla forma delle righe 9 e 11 — se il proprietario non dice altro, il piano la scrive così |
| 5 | la decisione **13**, il confinamento del worker telecamera | registrata; la chiude il sotto-progetto 12 col proprietario. Consiglio: **processo ristretto**, perché la telecamera è un dispositivo di privacy e ADR-0025 dice che un confinamento più debole non è un ripiego; ma non si decide prima che esista il primo worker, o sarebbe una previsione (gotcha #57) |
| 6 | la sonda S3 **senza riga di catalogo** | registrata, come PL-1 e K-1/B-1: una riga nuova in §7.4 è spec, vincolo globale 7 |

---

## Vicoli ciechi e scelte scartate, col perché

| Scartata | Perché |
|---|---|
| `decision-map` | perimetro già noto; GitHub Issues sarebbero una seconda casa delle decisioni (gotcha #68); nessuno strumento GitHub da questa macchina |
| approccio 2, tracciamento nella GUI | muore con la GUI (niente Jarvis a GUI chiusa), contraddice lo slot di ADR-0011, WebGL dentro la quota di presentazione, blocca il thread della UI (F5), la telecamera diventa stato di un processo sacrificabile |
| ibrido: worker per l'always-on e GUI per la manipolazione | due tracciatori su una telecamera, due modelli, due codici: sfoggio |
| la **terza quota** nella formula di ADR-0005, oggi | vale zero; il porto pretende già una concessione, quindi la forma minima esiste. Si apre con un tracciatore su GPU — decisione 9 |
| il gesto di comando come **trigger di ADR-0009** | quel trigger innesca l'anello di verifica; il posto giusto è il corollario di ADR-0011 |
| il **video nella webview** | i 21 punti bastano a disegnare la mano; il video costerebbe una misura in più per ADR-0029 e una telecamera condivisa fra due processi — decisione 5 |
| le finestre dell'**OS** | effetto OS, settima famiglia di porte: sotto-progetto 10, ADR suo |
| la telecamera come **occhio** | B della prima domanda: i fotogrammi come contenuto non fidato nel gateway sono un'altra cosa, e si aggiungono dopo come capacità |
| un **quinto pilastro** | vorrebbe un ADR che supera ADR-0001 per un contendente della GPU che oggi non la usa — decisione 1 |
| i gesti **dentro il sotto-progetto 8** | li rimandava per una ragione, SP-2, che non li riguarda — decisione 11 |
| la conferma **a gesti** di un effetto irripetibile | un gesto letto male non deve poter confermare sé stesso — decisione 6 |
| i worker **dentro `crates/`** | Cargo tratta `crates/` come workspace, e un pacchetto non Rust lì confonde il cancello e ADR-0031 — decisione 8 |

---

## Le trappole che mordono scrivendo il piano

| # | Trappola | Che cosa fare |
|---|---|---|
| 1 | **un ADR senza voce in §5 del compendio è rosso**, e i **totali** degli ADR nei documenti di stato sono confrontati con la realtà da `check-docs.sh` | l'ADR, la sua voce in §5 e i totali si scrivono **nello stesso commit**; le case dei totali le dà il comando in §1.4 |
| 2 | il compendio ha un **tetto in byte**, e il verde non è un margine | prima di aggiungere una riga si misura `wc -c docs/COMPENDIO.md` contro il valore in `scripts/check-docs.sh`; ciò che è verbale va in `archivio/`, non in §6 |
| 3 | i **fine-riga sono misti per file**: il compendio è CRLF nell'albero, gli ADR metà e metà, tutto LF nell'indice | si scrive con Python `newline=""` su un temporaneo e `os.replace` (gotcha #82); gli ADR nuovi nascono LF; si **rimisura dopo** con `git ls-files --eol` e `tr -cd '\r' \| wc -c` |
| 4 | i numeri di sezione **duplicati** sono un rosso, letti con `^#{2,6} [0-9]+(\.[0-9]+)*` per file | nessun `## N.` ripetuto; le sotto-sotto-sezioni con `####` |
| 5 | il controllo dei link verifica il **file**, mai il frammento: un'ancora è un rimando che nessuno difende | una sezione si **nomina**, non si collega con un cancelletto; i percorsi sono relativi alla cartella di **ciascun** file |
| 6 | la roadmap si appende **senza rinumerare**, e il «Perché quest'ordine» parla dell'ordine di Voce | la riga 12 va in coda alla tabella «Sotto-progetti»; la riga 8 guadagna la dipendenza da 12; il «Perché quest'ordine» si rilegge e, se dice il falso sulla Voce, riceve un richiamo datato |
| 7 | la sezione 6 di `tracciabilita.md` cambia titolo — «Voce» → «Voce e gesti» | si controlla che nessun comando o rimando la cerchi per titolo: `grep -rn 'Voce' docs/tracciabilita.md scripts/check-docs.sh` prima di rinominarla |
| 8 | i richiami datati vivono **in testa all'ADR**, e il compendio **rimanda** | forma di AUD-032: nessuna copia del richiamo nella voce di §5, solo il rinvio. ⚠️ **RICHIAMO DEL 2026-09-04:** l'attribuzione ad AUD-032 è **stata corretta** nella §3.2 — la posizione «in testa» è una scelta di questo disegno, non la forma di AUD-032 |
| 9 | SP-7 ha bisogno della **telecamera vera** e di **questa macchina**, e il relay Rust è usa e getta | tutto in `spikes/`, fuori dal workspace; nulla in `crates/` né in `workers/`, che non esiste ancora e non nasce da questo piano |
| 10 | S3 su un `Admission` che non deriva `Debug` né `PartialEq` | `matches!` e `let … else`, come ogni sonda di `crates/kernel/tests/arbiter_admission.rs`; e la **contro-sonda** — una riserva vera a macchina piena resta `Queued` — è la seconda direzione, non un lusso |
| 11 | **i numeri degli ADR** si danno quando si scrivono | `ls docs/adr \| tail -1` dice l'ultimo; l'accoppiamento per numero con la voce di §5 è ciò che lo script controlla |
| 12 | la **pre-verifica di ogni compito** trova un difetto in tutti i compiti dispacciati finora, senza eccezione (`CLAUDE.md`) | ogni compito si rilegge contro il codice di **allora**, non contro questo disegno né contro il piano |

---

## Il prossimo passo

⛔ **Lo dice la §6 del [compendio](../../COMPENDIO.md), in un posto solo.** L'ordine approvato, con
le spunte di oggi, è nella §5.5 di questo disegno; la prima riga senza spunta è il **piano**, in una
sessione nuova.

⚠️ **RICHIAMO DEL 2026-09-03, la stessa sera:** il piano è scritto; la riga 4 della §5.5 porta la spunta della
**scrittura**, e l'esecuzione — che è il passo che la §6 del compendio nomina ora — non ha una spunta finché il
compito 9 del piano non la scrive.

✅ **RICHIAMO DEL 2026-09-04:** il piano è eseguito; la §6 del compendio porta il passo successivo, il brainstorming della knowledge base.

### Come si riprende — scritto alla chiusura della sessione del 2026-09-03, coi comandi

⚠️ **È il documento di consegna di questa sessione**, e sta qui e non in un file a parte perché il
repo ha già la sua convenzione: lo stato vive in file **tracciati**, e chi riprende legge **questo**
file per intero. Ogni riga è stata **riletta coi comandi** prima di essere scritta, non ricordata.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb`. Nessuno stash, nessuna operazione a metà |
| I commit di questa sessione | `git log --oneline 066008a..HEAD`: il disegno scritto sul posto (`c9fcd40`), la rilettura del proprietario con la verifica di coerenza (`b28a038`), e questa chiusura |
| Codice di prodotto | **non toccato**: `git diff --stat 066008a..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla. Sono cambiati tre file di documentazione: questo, il compendio, e l'archivio della consegna |
| Cancello | `bash scripts/check-docs.sh` → `OK` a ogni commit; `bash scripts/gate.sh` → **`GATE GREEN`, rilanciato alla chiusura** — e all'apertura, dove la consegna lo dichiarava non rilanciato. Si rilanciano, non si citano |
| Fine-riga | questo file e l'archivio sono **LF** nell'indice e nell'albero; il compendio è LF nell'indice e **CRLF** nell'albero, uniformemente: `git ls-files --eol docs/COMPENDIO.md docs/archivio/consegna-brainstorming-gesti.md docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md`. Chi li riscrive conserva i fine-riga di ciascuno e li rimisura dopo (trappola 3) |
| File temporanei | nessuno nel repository: le misure e gli script di questa sessione stanno nello scratchpad, fuori dall'albero, come `CLAUDE.md` prescrive |
| Debito lasciato | **nessuno non dichiarato**: le voci aperte sono le decisioni 2, 7, 9, 10, 12 e 13 e la voce 4 — l'etichetta di strato — tutte nelle tabelle di questo file col loro chiusore |

**Il compito della sessione successiva: scrivere il piano.** In ordine, e ogni riga è eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -3`: si parte da `main`, e la
   testa deve essere il commit di questa chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e la testa dell'audit
   del 2026-08-27 — poi **questo file, per intero**. L'archivio della consegna **non** è lettura
   obbligatoria.
3. Prima di scrivere, la regola di `CLAUDE.md` su `superpowers:writing-plans`: le voci aperte **si
   sanno prima**. Dove stanno: la tabella *«Le voci aperte del Traguardo 5, in una tabella sola»* di
   [`porta-di-qualita.md`](../../porta-di-qualita.md), col comando che vive lì; le voci senza numero
   AUD dell'[audit](../../audit-2026-08-27.md); la tabella delle tredici decisioni di questo file.
   ⚠️ Quali abbiano come chiusore **questo piano** o *«il proprietario, prima»* lo decide chi lo
   scrive leggendo la colonna *«Chi la chiude»*, non questa riga: quelle si chiudono o si portano nel
   piano, le altre si conoscono e si dichiarano nell'errata in testa.
4. `superpowers:writing-plans`: il piano in `docs/superpowers/plans/<data>-riconoscimento-gesti.md`,
   coi compiti del punto 4 della §5.5, la Definizione di «fatto» della §5.5 **copiata da qui**, e la
   voce 4 presa scrivendo la riga della roadmap. In testa: modalità subagent-driven, errata,
   pre-controllo — la forma dei piani precedenti, `ls docs/superpowers/plans/`.
5. Il pre-controllo delle quattro domande di `CLAUDE.md` su ciascun compito, **nella sessione che
   scrive il piano**; ogni compito si legge contro il codice di **allora**, e la sezione *«Le
   trappole»* dice dove guardare.
6. L'esecuzione in una sessione **nuova**, un subagente fresco per compito, revisione fra uno e
   l'altro (`superpowers:subagent-driven-development`): la regola del proprietario.
7. A piano eseguito: la §6 del compendio porta il passo successivo — il brainstorming **distinto**
   della knowledge base — e questo file entra nella §12 del compendio e in `README.md`, come la
   condizione 8 della Definizione di «fatto» prescrive.

📌 **Ciò che questo disegno consegna a chi scriverà il piano**, ed è suo e non un puntatore: i
compiti del punto 4 della §5.5; la Definizione di «fatto» approvata; la tabella dei controlli per
artefatto della §1.4; le trappole; le fonti F1–F9 della §7, già rilette alla radice, che entrano in
`riferimenti.md` così come sono; e il consiglio scritto sulla voce 4.

⛔ **Vicoli ciechi di questa sessione: nessuno nuovo.** L'unico tentativo caduto è una fonte —
Forasoft, di terza mano, che diceva *«mantenuta»* di RTMPose — ed è registrato nella riga F8 della
§7, perché nessuno la citi di nuovo senza risalire a OpenMMLab.
