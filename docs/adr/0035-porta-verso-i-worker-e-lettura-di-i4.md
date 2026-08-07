# ADR-0035: La porta verso i worker, e cosa significa «singolo» in I4

- **Status:** Accepted
- **Date:** 2026-08-07
- **Deciders:** proprietario del progetto

## Context

La §2.3 della [spec del sotto-progetto 1](../superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md)
elenca sei famiglie di porte, e descrive `process` come **«avvio e uccisione dei
worker»**. Non esiste nessuna porta per **parlare** con un worker già avviato.

È la voce **F1** della riapertura del 2026-08-07, emersa rileggendo
[`tracciabilita.md`](../tracciabilita.md) con la domanda *«di quale meccanismo di kernel
ha bisogno questa funzionalità, e la spec lo nomina?»* — gotcha #27.

### Il divario è fra la spec e `design/01`, non fra la spec e I4

[design/01](../design/01-topologia-dei-processi.md) descriveva già il canale con tre
verbi, e la §2.3 ne ha conservato due:

| Riga della tabella dei canali | Cosa dice |
|---|---|
| `core → worker ML` | *«Avvia, **istruisce**, uccide. Il worker non risponde di iniziativa propria»* |
| `core → worker audio` | *«Idem; **il flusso audio risale al core**»* |

Il verbo mancante era già scritto. Quello che mancava è la **porta** che lo regge.

### Perché non poteva aspettare la §5

La §3.1 dichiara che le porte del simulatore *«sono **esattamente** le porte della §2.3,
e non esistono altri punti in cui il mondo tocchi il kernel»*, e il simulatore sostituisce
**tutte** le porte. Una porta aggiunta dopo la campagna significherebbe che il criterio
**C1** era verificato su un mondo più piccolo del reale — e **nulla sarebbe diventato
rosso**. È la classe del gotcha #17: una prova vacua che sembra un successo.

Per il criterio A-B-C della §0.3 è **regola B**: non retrofittabile.

### La porta serve davvero? — sì, e la verifica è stata fatta

Il catalogo della §7.4 ha prodotto il proprio esito migliore **riducendo** tre voci
invece di aggiungerne, quindi la domanda è obbligatoria. La via senza porta sarebbe: il
worker riceve tutto negli argomenti di avvio e restituisce tutto su `filesystem`.

**Non regge**, per tre ragioni indipendenti già scritte altrove:

| # | |
|---|---|
| 1 | `design/01`: *«il flusso audio risale al core»* — esiste un canale di ritorno, dichiarato |
| 2 | gotcha #1: wake word, VAD e trascrizione continua sono **sorgenti di eventi**, che devono comunque raggiungere il core senza essere giornalate |
| 3 | i token in streaming sono un **flusso**, non un file: `filesystem` non li rappresenta |

**I5 però ne limita la portata, e non per disciplina:** nel worker non vivono ritentativi,
code, priorità né decisioni. Il vocabolario da progettare è **piccolo per costruzione**.

### Cosa dice davvero I4

| | |
|---|---|
| **enunciato** | *«Il protocollo IPC è privato, singolo e non versionato: un trasporto, uno schema, nessun broker, nessun service discovery»* |
| **riga di verifica** | *«Nessun consumatore esterno, per [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md)»* |

La riga di verifica parla di **esternalità**, non di quanti dei nostri processi parlino il
protocollo. È la stessa forma della lacuna su I2, dove l'enunciato era corretto e la riga
di verifica copriva una classe di processo su tre.
[ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md) ha fissato il precedente:
**completare non è contraddire**.

### Le prove documentali, verificate una per una

Passate **tutte** le occorrenze di I4 e della sua formulazione nel repository:

| Dove | Cosa afferma | Esito |
|---|---|---|
| [design/01](../design/01-topologia-dei-processi.md) | l'annotazione `(I4)` sta **solo** sulla riga `gui ↔ core`; le righe dei worker si chiamano «comando» e non la portano | prova principale |
| [ADR-0027](0027-stack-della-gui.md) | *«tre canali logici su **un solo trasporto** (I4)»*, misurato in P1 | I4 tollera già il **multiplexing logico** |
| ADR-0027 · [ADR-0030](0030-framework-dell-interfaccia.md) | *«I4 lascia il protocollo libero di cambiare»* | invariato: non versionato |
| [spec del kernel §0.2](../superpowers/specs/2026-08-06-kernel-design.md) | *«non espone un'API pubblica»* | nessuno dei due canali ha consumatori esterni |
| spec sotto-progetto 1, §6.8 | `minicbor` scartato perché gli indici di campo servono all'evoluzione dello schema, *«un beneficio che I4 rinuncia esplicitamente»* | **rafforzato**: due canali non versionati, stesso rifiuto |
| spec sotto-progetto 1, §6.1.1 · §7.3.1 | giustificazione di `bincode`: *«serializza lo schema IPC (I4)»* | ⚠️ **da allargare** — costo dichiarato sotto |
| spec sotto-progetto 1, §6.1.2 | il timbro di build, *«non è una deroga a I4»* | ⚠️ va risposto per il secondo artefatto |

Nessuna rottura. Una riga si rafforza, due si allargano.

## Alternative considerate

Il bivio ne conteneva due, e tenerli separati cambia le risposte: **quanti schemi**, e
**dove passa il dialogo**. Sono assi indipendenti.

| | dialogo su **`ipc`** | dialogo su **`process` allargata** |
|---|---|---|
| **uno schema** | **A** — I4 letterale; il worker tocca *due* porte | **B** — I4 letterale; worker su *una* porta |
| **due schemi** | **C** — worker su *due* porte, e «singolo» da rileggere | **D** — worker su *una* porta, «singolo» da rileggere |

| # | Discriminante | Da | A | B | C | D |
|---|---|---|---|---|---|---|
| 1 | **la catena del gettone**: parlare a un worker che non hai avviato è *inesprimibile*? | §5.6 · §6.3 | ⛔ | ✅ | ⛔ | ✅ |
| 2 | **classe di fiducia come proprietà della porta**: dal worker torna contenuto **non fidato** per nome ([ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md)); dalla gui arriva l'**utente**, cioè il canale fidato | I6 | ⚠️ | ⚠️ | ✅ | ✅ |
| 3 | **§8.2.2 resta 1:1** — «un Q eredita lo stato della porta in cui si inietta» | §3.3 · §8.2.2 | ⛔ | ⛔ | ✅ | ✅ |
| 4 | **i mirror restano minimi**: TypeScript per la gui, Python per il worker | ADR-0027 · [ADR-0028](0028-ecosistema-dei-worker-ml.md) | ⛔ | ⛔ | ✅ | ✅ |
| 5 | la giustificazione di `bincode` resta invariata | [ADR-0031](0031-dipendenze-del-kernel-parte-del-confine.md) r.1 | ✅ | ✅ | ⚠️ | ⚠️ |
| 6 | «singolo» **non** va riletto | I4 | ✅ | ✅ | ⛔ | ⛔ |

**A e C cadono sul discriminante 1**, che è il più pesante: sotto entrambe la vita di un
worker si spezza fra due porte — si avvia da `process` e gli si parla da un'altra. La §5.6
ha comprato I2 dal **compilatore** proprio perché non esiste una seconda via verso un
processo; aggiungerne una accanto rimette in gioco quella chiusura.

**Il discriminante 4 è controintuitivo e va detto:** due schemi costano **meno** in
manutenzione, non di più. Con uno schema solo, il mirror Python deve rispecchiare anche i
messaggi della gui e viceversa — cioè si allarga il costo che ADR-0027 già dichiara
(*«due definizioni da tenere allineate»*).

## Decision

**Il dialogo con un worker vive dentro la porta `process`, che copre l'intero ciclo di
vita; il suo schema è distinto da quello verso la gui; e «singolo» in I4 si legge _per
canale privato_.**

| # | Regola |
|---|---|
| 1 | `process` copre **avvio, dialogo e uccisione**. Non nasce una porta nuova: la §2.3 resta a **sei** famiglie |
| 2 | Gli schemi dei due canali privati sono **distinti**, ed entrambi vivono in `kernel` |
| 3 | **«Singolo» significa: un meccanismo di trasporto e uno schema _per canale privato_** — nessun broker, nessun service discovery, nessuna negoziazione, nessun versionamento. Ciò che I4 compra è che non esista un contratto pubblico da congelare, e **nessuno dei due canali ha consumatori esterni** (ADR-0003) |
| 4 | Il rifiuto di un pari stantio resta il **timbro di build** (§6.1.2), identico sui due canali: non è una macchina nuova |
| 5 | I4 si **completa**, non si riformula. ADR-0004 non è superato e riceve un rimando |

### Perimetro negativo — cosa questa decisione **non** è

| Non è | |
|---|---|
| un **bus, un broker o un service discovery** | è ciò che I4 esclude, e resta escluso su entrambi i canali |
| un **secondo serializzatore in `kernel`** | se il formato non fosse raggiungibile dal pari, vale l'**esito B di M-1** — tipi in `kernel`, serializzazione in `daemon` — già misurato, già prezzato, e **fuori** dal confine di ADR-0031 |
| il **progetto** della porta | firme, messaggi e chi risveglia chi sono §5–§6 |
| la scelta del **formato di filo** | è §6, e ha una domanda aperta dichiarata sotto |
| una deroga al **non versionamento** | il timbro di build è il meccanismo, come per la gui |

## Consequences

- **Positive:**
  - **La catena del gettone si conserva.** L'oggetto con cui si parla a un worker è quello
    che restituisce l'avvio, e l'avvio pretende una concessione (§5.6): «parlare con un
    worker senza concessione» non diventa esprimibile. Nessun gettone nuovo da inventare.
  - **I6 diventa strutturale su questo confine.** Ciò che risale da un worker è contenuto
    non fidato per nome in ADR-0014 — risposte dei provider, trascrizioni; ciò che arriva
    dalla gui è l'utente, cioè il canale fidato. Una porta per classe di fiducia rende la
    distinzione un **tipo**, non un controllo di provenienza a runtime.
  - **La regola di §8.2.2 resta 1:1**: `ipc` → Q3, `process` → Q4. Nessuna porta con due
    inneschi, nessuna nota a piè di pagina su una regola appena scritta.
  - **Costa poco perché non aggiunge niente**: zero porte nuove, zero meccanismi nuovi,
    zero voci nuove nella lista di ADR-0031. Ciò che cresce sono tre giustificazioni e una
    lettura.

- **Negative (accettate):**
  - **La lettura di «singolo» è scritta, non dedotta.** Se un giorno si rivelasse troppo
    larga, si torna a uno schema solo e la §6 si riscrive. È il costo di completare la riga
    di verifica di un'invariante invece di lasciarla ambigua.
  - ⚠️ **Un follow-up di ADR-0028 diventa un presupposto.** *«Trattare l'ambiente Python
    come artefatto da versionare, non come prerequisito dell'utente»* era una
    raccomandazione per i sotto-progetti 9 e 10: ora la regola 4 vi poggia. Se l'ambiente
    del worker non è artefatto nostro, il timbro non ha un'identità da confrontare e «non
    versionato» **cade su questo canale**.
  - **Tre giustificazioni da allargare** in §6.1.1 e §7.3.1, con richiamo datato. Si fanno
    in F1b, quando il formato è scelto: allargarle prima significherebbe scrivere
    un'affermazione non ancora vera.
  - **`process` diventa la porta più grande del kernel**, e §7.4.6 acquisirà
    un'affermazione in più sulla conformità fra la finta e la vera.
  - ⛔ **Una domanda di stato dell'arte resta aperta, e non è risolta qui:** che `bincode`
    sia decodificabile dal pari **Python** non è verificato, e non lo si afferma. Non
    cambia questa decisione — la §2.3 dichiara una porta, non un formato — ma va misurato
    in §6 **prima** di scegliere il formato. Se la risposta è no, scatta l'esito B di M-1.
  - **Una tensione di `design/01` da conciliare in F1b:** *«il worker non risponde di
    iniziativa propria»* contro *«il flusso audio risale al core»*. Non è una
    contraddizione — lo streaming è istruito — ma la forma della porta deve renderlo
    esplicito, o la prima frase diventa falsa in silenzio.

- **Follow-up richiesti:**
  - La §2.3 dichiara la porta (**F1a**, fatto con questo ADR), e la §3.1 vi si allinea:
    quella tabella dichiara di essere *«esattamente le porte della §2.3»*.
  - **F1b** progetta la porta in §5–§6: firme, messaggi, il formato di filo, e
    l'allargamento di §6.1.1 e §7.3.1.
  - §7.4.6 acquisisce la riga della suite di conformità per `process`.
  - La **§8** registra lo stato delle righe toccate — per ultima, e una volta sola.
