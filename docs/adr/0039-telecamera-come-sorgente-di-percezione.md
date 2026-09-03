# ADR-0039: La telecamera come sorgente di percezione always-on sotto il core

- **Status:** Accepted
- **Date:** 2026-09-03
- **Deciders:** proprietario del progetto

## Context

Il proprietario vuole l'agente **dormiente e risvegliabile con la wake word**, e gesti *«stile
Jarvis»*: menu virtuali, pannelli spostati con le mani, una cattura con un gesto. Il
[disegno del 2026-09-03](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) ha posto
quattro domande d'apertura e il proprietario ha risposto: la telecamera è **una sorgente di
eventi** — non un occhio che mette immagini nel contesto di un modello — con i fotogrammi che
**non escono mai** dal processo che la possiede; i pannelli mossi con le mani sono quelli **del
programma**, non le finestre dell'OS; e la forma è l'**approccio 1**, la sorgente di percezione
**sotto il core**, scelta dopo la sfida *«sicuro che rispetti tutti i principi?»* e tre
correzioni.

Ciò che l'architettura dice già, e che questa decisione onora:

| Il buco | Ciò che decide già |
|---|---|
| «se l'agente dorme, quando succede?» | **il core non dorme mai: dorme la run.** Il core vive a lungo, anche senza GUI ([ADR-0004](0004-topologia-di-processo.md)); la wake word non «sveglia il programma», **apre una run** ([ADR-0011](0011-routing-risolto-e-giornalato-per-richiesta.md), corollario) |
| «chi tiene i fotogrammi?» | un **worker**, in Python, senza stato, uccidibile in ogni istante ([ADR-0028](0028-ecosistema-dei-worker-ml.md), I5); al core arrivano **eventi** |
| «e la GPU?» | il porto `process` **pretende una concessione** per avviare qualunque worker — `Process::start(grant, descriptor)`, §5.6 della spec; le due concessioni permanenti di [ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md) sono il precedente |
| «il kernel decide sui gesti?» | no: un gesto è un **dato opaco** ([ADR-0020](0020-nessun-modello-nel-percorso-decisionale-del-kernel.md)), smistato come la trascrizione che diventa messaggio; il kernel resta testabile senza modello, perché l'evento si inietta a copione ([ADR-0021](0021-simulazione-deterministica-e-iniettabilita.md)) |
| «una foto può dare ordini?» | no: contenuto **non fidato**, informa e non autorizza ([ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md), I6); un gesto non concede permessi ([ADR-0038](0038-registro-delle-funzioni-del-programma.md)) |
| «telecamera assente o spenta» | si **dichiara prima**, non si fallisce dopo ([ADR-0019](0019-lo-stato-di-degrado-e-un-oggetto-osservabile.md)) |
| il profilo «riservato» | spegne la voce always-on ([ADR-0023](0023-cifratura-a-riposo-e-gestore-dei-segreti.md)): spegne **anche** la telecamera |

**Lo stato dell'arte, verificato il 2026-09-03** (le fonti F1–F9 in [`riferimenti.md`](../riferimenti.md)):
MediaPipe è mantenuto — `mediapipe` 1.0.1 su PyPI, rilasciato il 2026-08-14 — e il suo Hand
Landmarker dà **21 punti** per mano, con un modo `LIVE_STREAM` a callback; in Python **la GPU non
c'è su Windows** (F2, e F9 lo conferma), quindi il tracciatore gira su **CPU**. L'alternativa con
GPU su Windows esiste — RTMPose, via ONNX Runtime — e la sua manutenzione si rimisura il giorno
che serve: oggi l'ultimo rilascio è del 2024-07-12.

### Alternative considerate

| | Perché no |
|---|---|
| **il tracciamento dentro la GUI** — MediaPipe in JavaScript sulla webview, l'approccio 2 | muore con la GUI: niente Jarvis a GUI chiusa; contraddice lo slot di ADR-0011 — la sorgente di percezione diventerebbe stato di un processo **sacrificabile** (ADR-0004); WebGL dentro la quota di presentazione; e `detectForVideo` **blocca il thread della UI** (F5). Vince sulla latenza della manipolazione — un salto in meno — e per questo la latenza dell'approccio 1 **si misura** (SP-7, S2) invece di darsi per buona |
| **l'ibrido** — worker per l'always-on, GUI per la manipolazione | due tracciatori su una telecamera, due modelli, due codici: sfoggio |
| **una terza quota permanente** nella formula di [ADR-0005](0005-arbitrato-gpu-su-due-dimensioni.md), oggi | varrebbe **zero**, e il porto pretende già una concessione: la forma minima esiste. Si apre quando esiste un tracciatore su GPU — decisione 9 del disegno, registrata |
| il gesto di comando come **trigger** di [ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) | quel trigger innesca l'**anello di verifica**; il posto di «un evento apre un passo» è il corollario di ADR-0011 |
| il **video** nella webview | i 21 punti bastano a disegnare la mano; il video costerebbe una misura in più per [ADR-0029](0029-guscio-della-gui.md) e una telecamera condivisa fra due processi — decisione 5 |
| un **quinto pilastro** | vorrebbe un ADR che **supera** [ADR-0001](0001-architettura-a-kernel-con-capacita-paritarie.md) sulla parola «quattro», per un contendente della GPU che oggi non la usa — decisione 1 |

## Decision

> **La telecamera è una sorgente di percezione always-on sotto il core: un worker Python la
> possiede, i fotogrammi non escono mai, al core arrivano eventi — lo stato continuo della mano e
> il gesto discreto — che non sono passi. Spenta per default; solo la wake word apre una run;
> «riservato» la spegne.**

| Pezzo | Forma |
|---|---|
| il worker | Python, always-on, sotto il core come sarà il microfono (ADR-0028). **Possiede la telecamera**; i fotogrammi **non escono mai**. MediaPipe Hand Landmarker su **CPU**. Senza stato e **uccidibile in ogni istante** (I5): ciò che ADR-0004 chiama *«vita breve»* si legge, per un worker always-on, come per il worker audio — la ricevuta di stream resta aperta per tutta la vita — e a reggere sono *senza stato* e *uccidibile* |
| il canale | il porto `process` ([ADR-0035](0035-porta-verso-i-worker-e-lettura-di-i4.md)): **una** `instruct_stream` all'accensione — «traccia le mani», la prima istruzione vera della direzione core → worker — poi `read_next` per tutta la vita. `minicbor`, ogni frame dichiara la propria lunghezza ([ADR-0037](0037-criterio-del-pari-per-il-formato-dei-canali.md), §6.10 della spec) |
| le due specie di evento | lo **stato continuo della mano** — 21 punti per mano, coordinate **intere**, a N Hz — e il **gesto discreto** — `kind`: un enum chiuso, `confidence`: un intero. Due varianti nuove di `FromWorker` a indici nuovi, sotto le regole di §6.10. **Eventi, non passi** (ADR-0011): niente giornale per fotogramma |
| il core | li **smista**: la manipolazione va alla GUI con `Ipc::send`, **campionata alla frequenza che il core riceve come parametro consegnato** ([ADR-0034](0034-parametri-di-decisione-consegnati-non-letti.md)); un gesto di **comando** prende la strada della wake word: apre un **passo** nella run aperta, e passa dal registro di ADR-0038 con lo stesso permesso di ogni invocatore |
| la run | **solo la wake word apre una run** (decisione 3); un gesto di comando vale a run aperta. Il gesto di attenzione resta aggiungibile dopo senza rifare niente |
| la concessione | da **zero MiB**, `Preemption::Never`, corsia `Realtime`, chiesta come le due permanenti di ADR-0033 — **all'accensione**, non all'avvio del core; torna con `Killed.grant` ad `Arbiter::release`. **La formula di ADR-0005 non cambia** |
| l'interruttore | **spenta per default** (decisione 4): accenderla è una **funzione del registro** di ADR-0038, quindi con permesso e giornale. Dove l'interruttore si salvi fra un avvio e l'altro è l'archivio dei parametri, che non esiste: decisione 10, registrata |
| «riservato» | spegne anche la telecamera — rimando datato in testa ad ADR-0023 |
| il degrado | `Degradation` guadagna «telecamera assente o spenta» **solo quando il worker esiste** (ADR-0019): un campo sempre `false` si legge come «tutto bene» invece che come «ignoto» |
| la GUI | **disegna la mano dai 21 punti**, in un livello sopra i pannelli; **niente video** (decisione 5): compositing della webview, dentro la quota di presentazione di ADR-0033, nessuna misura in più per ADR-0029. Un **indicatore sempre visibile** quando la telecamera è accesa, acceso dal **core** con un messaggio — la GUI non lo indovina: una falsa sicurezza è peggio di nessuna sicurezza (ADR-0023) |
| il pilastro | «voce» si legge «voce e gesti»; i pilastri restano quattro — rimando datato in testa ad ADR-0001 |

### Regole di forma, verificate nel sorgente il 2026-09-03

| Regola | Perché |
|---|---|
| **niente decimali nel kernel**: sul filo viaggiano interi, la conversione la fa il worker | `grep -rnw f32 crates/kernel/src` e `grep -rnw f64 crates/kernel/src` non rendono niente; MediaPipe dà coordinate normalizzate fra 0 e 1, e il worker le scala |
| **nessun testo dal worker arriva a una decisione**: il gesto è un enum chiuso, non una stringa | il precedente è `GrantRequest` in `crates/kernel/src/wire/ipc.rs`, che attraversa il filo **senza `name`**: un testo scelto dal pari è contenuto non fidato (ADR-0014). Il **vocabolario** dei gesti resta della capacità; il **tipo** si fissa qui |
| **le regole del canale restano quelle di §6.10**: un indice per campo, niente enum di versione, niente byte congelati | testa di `crates/kernel/src/wire/worker.rs` |

### Costo dichiarato: la telecamera sarebbe il PRIMO worker vero, e il primo paga

Verificato il 2026-09-03: nessuna implementazione di `Process` o `Worker` esiste fuori dai banchi — `git grep -l -e 'impl Process for' -e 'impl Worker for' 4d16f33 -- crates/` rende quattro banchi e un solo file di `src/`, `crates/kernel/src/ports/process.rs`, dove la frase sta in un **commento** che nomina questo stesso comando; il
canale worker ha una direzione sola, in su, e in giù nessun messaggio; il **timbro di build** che
rifiuta un worker stantio non esiste, e la §6.10.7 della spec lo fa reggere su un ambiente Python
**nostro e versionato** — quindi il lockfile del worker non è cosmesi; il reattore conosce solo
il tempo, e «pronto da leggere» per una pipe non c'è; nessun codice di produzione legge ancora
una porta. Non cambia l'approccio: è il **prezzo**, lo paga chi arriva primo fra voce e gesti — e
per decisione 11 del disegno è il sotto-progetto **12**. La Voce lo **riusa**.

### Perimetro negativo — cosa questa decisione **non** è

| Non è | |
|---|---|
| il **vocabolario** dei gesti, e quali funzioni siano gestuali | la capacità, sotto-progetto 12 — decisione 2 |
| la **destinazione** di una cattura | il brainstorming della knowledge base — decisione 7 |
| il messaggio IPC verso la GUI con la mano campionata | **si definisce quando la GUI esiste** (sotto-progetto 2): prima non ha destinatario, come la revoca dichiarata in testa a `crates/kernel/src/wire/ipc.rs` |
| una terza quota nella formula di ADR-0005 | decisione 9: si apre con un tracciatore su GPU |
| il **confinamento** del processo del worker — se, oltre a separato (ADR-0028), sia **ristretto** al livello 2 di [ADR-0025](0025-confinamento-a-livelli.md) | **nessun ADR lo decide** e nel codice non c'è dove dirlo: `WorkerDescriptor` è byte opachi. Decisione 13 del disegno, registrata: la chiude il sotto-progetto 12 col proprietario, quando avvia il primo worker vero. Consiglio scritto: processo **ristretto**, perché la telecamera è un dispositivo di privacy e ADR-0025 dice che un confinamento più debole non è un ripiego |
| la telecamera come **occhio** dell'agente | un'altra cosa — fotogrammi come contenuto non fidato nel gateway — che si aggiunge dopo come capacità |
| le finestre dell'**OS** | sotto-progetto 10, con un ADR suo |
| `workers/` alla radice | decisione 8: la cartella nasce col primo worker di prodotto, cioè col sotto-progetto 12, non con questo ADR |

## Consequences

- **Positive:**
  - **Jarvis a GUI chiusa**: la sorgente di percezione vive sotto il core, che è l'unico
    processo a vita lunga.
  - **Coerenza piena con ADR-0011 e ADR-0004**: eventi e non passi, e nessuno stato in un
    processo sacrificabile.
  - **Se poi serve l'altro approccio, niente da rifare**: la GUI può aggiungere un tracciatore
    proprio senza toccare il kernel; il contrario non è vero.
  - **Nessuna misura in più per ADR-0029**, e nessuna quota in più nella formula del budget.

- **Negative (accettate):**
  - **Un processo Python su CPU finché la telecamera è accesa**, e la CPU che consuma la paga
    l'utente: ecco perché è opt-in.
  - **Un salto in più sulla manipolazione** — worker → core → GUI — e la latenza **non è nota**:
    la misura **SP-7** (S2), e il giudizio sulla mano che muove un pannello è del proprietario
    che la prova.
  - **Il conto del primo worker** (sopra): trasporto di `process`, messaggio in giù, timbro di
    build, prontezza del reattore, ciclo di lettura. Lo paga il sotto-progetto 12.
  - **Niente GPU su Windows per il tracciatore** (F2, F9): se un giorno serve, la via è ONNX
    Runtime con RTMPose, e quel giorno la manutenzione di RTMPose si rimisura.
  - **Tre ipotesi restano ipotesi fino a SP-7**: che MediaPipe su CPU regga 30 Hz su questa
    macchina (S1), che il giro a 30 Hz sia accettabile (S2), e che una riserva da zero passi
    l'ammissione (S3, sonda nel kernel).

- **Follow-up richiesti:**
  - **SP-7** in `spikes/`, coi criteri scritti prima della misura, e la sonda **S3** in
    `crates/kernel/tests/arbiter_admission.rs`.
  - I tre **rimandi datati** in testa ad ADR-0001, ADR-0011 e ADR-0023.
  - La riga **12 «Gesti»** in [`roadmap.md`](../roadmap.md), che dipende da 2 e 3, e la Voce che
    dipende anche da 12; le righe in [`tracciabilita.md`](../tracciabilita.md), sezione «Voce e
    gesti».
  - Il **sotto-progetto 12** costruisce il worker, le due varianti di `FromWorker`, il messaggio
    in giù, il parametro della frequenza, il campo di `Degradation`, e decide la 13.
