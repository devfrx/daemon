# ADR-0033: La GPU della GUI — quota di presentazione sottratta, concessione tenuta dal core

- **Status:** Accepted
- **Date:** 2026-08-07
- **Deciders:** proprietario del progetto

## Context

[I2](0004-topologia-di-processo.md#invarianti) stabilisce che la GPU ha un solo
proprietario: nessun processo la tocca senza concessione dell'arbitro. La riga di
verifica scritta accanto all'invariante dice però una cosa più stretta di ciò che
l'invariante afferma:

> «Nessun **worker** si avvia senza una concessione valida.»

[ADR-0005](0005-arbitrato-gpu-su-due-dimensioni.md) e
[design/02](../design/02-arbitrato-gpu.md) **non menzionano mai la GUI**. Ma il processo
`gui` tocca la GPU, e lo fa in due modi distinti:

| | Consumo | Quando |
|---|---|---|
| 1 | **compositing** della webview | sempre, anche su una schermata di solo testo |
| 2 | **viewer 3D interattivo** (G6) | quando l'utente guarda una mesh |

Durante un render TRELLIS2 che vuole 13–14 GB su 16 ([SP-1](../superpowers/specs/2026-08-06-kernel-design.md#sp-1--curva-qualitàvram-di-trellis2-su-16-gb),
RK-1) quella VRAM è contesa. La lacuna era registrata in
[HANDOFF](../HANDOFF.md) e in [roadmap](../roadmap.md) con tre uscite enumerate.

### L'asimmetria che le tre uscite non nominavano

Un worker è **roba nostra**: lo avviamo noi, dichiara un profilo di risorsa, e se
l'arbitro rifiuta **non parte**. Il rifiuto è *esecutivo*.

Il compositor no. Chi alloca VRAM per la GUI non è il nostro codice: è il motore di
rendering — quello di sistema o quello impacchettato dal guscio, secondo come si
chiuderà [ADR-0029](0029-guscio-della-gui.md).

| | Worker | Compositor |
|---|---|---|
| chi lo avvia | noi | il guscio |
| dichiara un profilo di risorsa | sì | **no**, e non sa quanto userà |
| esiste un percorso di richiesta | sì | **no** |
| uccidibile per liberare | sì | ucciderlo = chiudere l'interfaccia |
| **il rifiuto dell'arbitro è esecutivo** | **sì** | **no**: compone lo stesso |

Questa differenza è il contenuto del problema, ed è il motivo per cui nessuna delle tre
uscite enumerate funziona da sola.

### Le alternative considerate — sono le tre uscite, e sono tre risposte parziali

- **A — il viewer chiede una concessione come tutti.**
  *Dove è giusta:* per il consumo discrezionale del viewer 3D, che **è** nostro codice e
  **ha** un percorso di richiesta (IPC verso il core).
  *Dove non regge:* non è applicabile al compositing. Non esiste un punto in cui il
  compositor possa chiedere. **A non rende I2 vero: lo rende vero per una parte del
  problema**, lasciando fuori proprio quella che è sempre attiva.

- **B — il viewer è esente.**
  *Dove è giusta:* è l'etichetta onesta della **non-esecutività** del rifiuto verso il
  compositor, che resta vera qualunque cosa si decida.
  *Dove non regge:* come design è una ritirata. Lascia scoperto l'OOM che si verifica in
  pratica, e indebolisce Q2 — «zero OOM, per costruzione (I2)» — senza dirlo.

- **C — quota sottratta, come per l'audio.**
  *Dove è giusta:* è il meccanismo corretto per compositing e viewer di base.
  *Dove non basta:* copiata pari pari **viola il vincolo che ADR-0005 si era dato**. La
  sottrazione della quota audio non è un'esenzione: il worker audio detiene una
  concessione *permanente e non prelazionabile*. Una quota sottratta **senza titolare**
  lascia I2 falso per la GUI.

- **D — degrado cooperativo:** durante un job `batch` pesante l'arbitro istruisce la GUI
  a smettere di renderizzare il 3D.
  *Esito:* **non è una quarta alternativa.** È la specificazione di cosa significhi
  `InRevoca` per il consumo discrezionale, ed è già nella macchina a stati di design/02.
  Confluisce nel punto 3 della decisione.

- **E — disattivare l'accelerazione GPU della webview.** Eliminerebbe il consumo 1.
  *Contro:* il compositing cadrebbe sulla CPU, e **P3 è già a 21,43 % su una soglia del
  25 %** senza rendering reale ([ADR-0027](0027-stack-della-gui.md)). L'ipotesi è che lo
  sfondi. È un'**ipotesi non misurata**: entra fra le misure, non fra gli argomenti.

### Una divergenza da un'attesa scritta, registrata

[HANDOFF](../HANDOFF.md) annotava che l'uscita A «incrina I1 — la GUI smette di essere
solo stato di presentazione». **Alla verifica non regge.** Un worker detiene una
concessione ed è dichiarato `possiede: nulla` in
[ADR-0004](0004-topologia-di-processo.md#decision): tenere un token non è tenere stato
autorevole. A va scartata, ma per un motivo **diverso e più forte** — il compositor non
ha un percorso di richiesta.

Registrata invece di allineata all'attesa, come prescrive il metodo.

## Decision

### 1. Tre consumatori, non uno

Il consumo GPU della GUI si modella come **tre consumatori distinti**, governati
diversamente perché hanno percorsi di richiesta diversi.

| # | Consumo | Governo | Rifiuto esecutivo? |
|---|---|---|---|
| 1 | compositing della webview | dentro la **quota di presentazione** | ❌ no |
| 2 | viewer 3D **entro** la quota | dentro la stessa quota | ❌ no |
| 3 | viewer 3D **oltre** la quota | **concessione ordinaria**, richiesta via IPC | ✅ sì |

Il consumatore 2 sta dentro la quota per scelta e non per necessità: un viewer che si
rifiuta di renderizzare durante un render sarebbe il «degrado silenzioso» che ADR-0005
vieta, e G6 è un requisito di prima classe.

### 2. La quota di presentazione è sottratta, e la concessione la tiene il **core**

```
budget allocabile = totale − quota audio − quota presentazione
```

Il core richiede all'avvio una concessione di presentazione **permanente e non
prelazionabile**, sottratta dal budget. La GUI la consuma **senza mai chiederla**.

| Proprietà | Perché regge |
|---|---|
| coerente con **I1** | la concessione è stato del core; la GUI non tiene nulla |
| coerente con la regola di ADR-0005 | la sottrazione **non è esenzione**: la concessione esiste, ha un titolare |
| **sopravvive alla GUI uccisa in qualsiasi istante** | il titolare è il core, la cui vita è lunga e indipendente (ADR-0004). Nessuna concessione perduta, e **nessun protocollo di liveness contro un processo progettato per morire** |
| la quota non si libera a GUI chiusa | se fosse riallocata a un render, la GUI riaperta andrebbe in OOM |

### 3. Il consumo discrezionale è una concessione ordinaria

Il consumatore 3 passa dall'arbitro come qualunque altro: profilo dichiarato, esito a
tre vie (`Concessa` · `InCoda` · `Rifiutata`), **prelazionabile con tempo di grazia**.

| Evento | Comportamento |
|---|---|
| revoca | la GUI smette di renderizzare il 3D e lo **dichiara** (ADR-0019): nessun degrado silenzioso |
| **la GUI muore tenendola** | il core se ne accorge dalla disconnessione IPC — misurato in P4, se ne accorge al messaggio 606 — e **riconcilia la concessione** |

### 4. I2 si **completa**, non si riformula

L'enunciato di I2 resta vero: ogni processo che tocca la GPU è coperto da una
concessione. Ciò che era incompleto è la **riga di verifica**, che nominava solo i
worker. La verifica completa è:

| Consumatore | Verifica |
|---|---|
| worker | non si avvia senza una concessione valida — **imposto dal compilatore**: la porta `process` richiede una concessione come argomento (§5.6 della spec del sotto-progetto 1) |
| GUI, consumi 1 e 2 | coperti dalla concessione di presentazione tenuta dal core |
| GUI, consumo 3 | concessione propria, con riconciliazione alla disconnessione |

Completare non è contraddire: [ADR-0004](0004-topologia-di-processo.md) **non è
superato**, riceve un rimando.

### 5. Il valore della quota è dichiarato **non misurato**

Non esiste un numero misurato per la VRAM del compositing né per una scena three.js, e
inventarlo violerebbe il metodo. Si applica il precedente di
[ADR-0010](0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md) con SP-3: un
default conservativo, **dichiarato come tale e non spacciato per misurato**.

La misura è **M5**, e si aggancia a M1–M4 di [ADR-0029](0029-guscio-della-gui.md), che
richiedono già lo stesso allestimento — frontend Vue minimo con scena 3D sui due gusci.
Costo marginale prossimo a zero.

## Consequences

- **Positive:**
  - I2 torna verificabile **per intero** invece che sui soli worker. Prima era controllato
    su una classe di processo su tre, e la riga di verifica lo nascondeva.
  - Per i worker I2 sale da **test a compilatore**: la porta `process` non è chiamabile
    senza una concessione. Un test si cancella, una firma no.
  - La GUI resta **sacrificabile senza eccezioni** (ADR-0004, G3): nessuna concessione si
    perde uccidendola, perché quella permanente non è sua e quella discrezionale si
    riconcilia sulla disconnessione.
  - Il meccanismo è **agnostico rispetto al guscio**: ADR-0033 non importa nulla da
    ADR-0029. Vale identico per Tauri e per Electron, come la lacuna prometteva.
  - Esporta invece un **discriminante nuovo verso ADR-0029**: quanto la quota di
    presentazione sia governabile dipende da chi possiede il motore di rendering. È la
    prima volta che il kernel vincola la scelta del guscio, e non il contrario.

- **Negative (accettate):**
  - **Per la GUI, I2 è più debole in natura che per i worker.** Verso un worker il rifiuto
    dell'arbitro è esecutivo; verso il compositor no. La quota è una **promessa di budget,
    non un'imposizione**. Non è mitigabile con la tecnica, ed è la stessa classe di onestà
    di [ADR-0023](0023-cifratura-a-riposo-e-gestore-dei-segreti.md): dichiarare la forza
    reale invece della forza che la parola suggerisce.
  - **VRAM sprecata quando la GUI è chiusa.** Identico al costo già accettato in ADR-0005
    per la quota audio, e con la stessa mitigazione: un profilo di configurazione senza
    interfaccia porta la quota a zero.
  - **RK-1 si stringe di una quantità non ancora nota.** La soglia di RK-1 — picco oltre
    ~13–14 GB su 16 — è stata scritta su un budget che non contava la GUI. Con due quote
    sottratte il margine è più piccolo, e di quanto lo dirà M5. È l'innesco osservabile
    del rischio.
  - **Una riga di ADR-0006 diventa incompleta.** La policy REMOTA dichiarava «VRAM
    occupata: solo audio riservato». Sono audio **più** presentazione. ADR-0006 non è
    superato — la decisione delle due policy come oggetti distinti resta valida — ma
    riceve un rimando, e [design/02](../design/02-arbitrato-gpu.md) è aggiornato.
  - **Il consumatore 3 aggiunge lavoro reale**: una concessione revocabile verso un
    processo che può morire in qualsiasi istante, quindi riconciliazione sulla
    disconnessione IPC e uno scenario DST in più.
  - **La quota si tara su un numero che oggi non esiste.** Fino a M5 il default è
    conservativo e dichiarato tale; un default troppo generoso spreca VRAM, uno troppo
    stretto lascia il compositor a sforare in silenzio — che è la modalità peggiore,
    perché non produce un errore ma un OOM altrove.

- **Follow-up richiesti:**
  - **M5 entra nel gruppo di misure di ADR-0029**: VRAM a riposo e sotto carico 3D, sui
    due gusci. Chiude il parametro di questo ADR **e** aggiunge un discriminante a quello.
  - **L'ipotesi E va misurata, non argomentata**: disattivare l'accelerazione GPU della
    webview sposta il costo sulla CPU, e P3 ha 3,57 punti di margine. Se M5 dicesse che la
    quota è insostenibile, E è la leva successiva — ma va misurata prima di essere
    proposta.
  - **Il picco reale della presentazione si misura e si registra**, con lo stesso
    meccanismo «riserva dichiarata, picco misurato» di ADR-0005: una quota
    sistematicamente sbagliata è un difetto del profilo, non un incidente.
  - Se M5 mostrasse che quota audio + quota presentazione non lasciano spazio a TRELLIS2
    al profilo minimo accettabile, scatta **RK-1** e la mutua esclusività va dichiarata in
    interfaccia — non fra un LLM caldo e un render, ma **fra l'interfaccia e un render**,
    che è una conseguenza più severa di quella che RK-1 prevedeva.
