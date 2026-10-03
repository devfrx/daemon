# ADR-0041: Chi può parlare col core — il solo account che lo esegue

- **Status:** Accepted
- **Date:** 2026-10-02
- **Deciders:** proprietario del progetto
- **Nasce da:** i finding AUD-685 e AUD-688 del [terzo audit](../audit-2026-09-30.md)

> ⚠️ **Rimando del 2026-10-03 — il capo del core è costruito.** `platform::ipc`: `Account::of_this_process`,
> `channel_of_this_account` e `LocalSocketIpc::bound(channel, account, numbers, max_body)`, con le sonde delle due
> direzioni — `this_account_gets_in_three_times_in_a_row` in `crates/platform/tests/ipc_contract_real.rs`, e il rifiuto
> di un altro account in fondo a `crates/platform/src/ipc.rs` —; la metà Linux la prova la CI. Il capo della GUI resta
> della shell. E la misura del 2026-10-02: anche una regola `GRGW` fa entrare tre client di fila, quindi l'«accesso
> pieno» del punto 4 non è l'unica maschera che regge la creazione delle istanze, che il punto 4 porta come la sua
> ragione. Audit del 2026-09-30, AUD-685. **Il testo della decisione non si riscrive.**

## Context

Il core apre un canale locale, la porta `ipc` di [ADR-0035](0035-porta-verso-i-worker-e-lettura-di-i4.md), e la GUI
— zero o una, [ADR-0004](0004-topologia-di-processo.md) — ci manda `Hello`, `Invoke`, `Approve` e `SaveLayout`.
Nessuna decisione diceva **chi** può aprirlo. Il codice, letto il 2026-10-02:

| | Linux | Windows |
|---|---|---|
| l'indirizzo | `LocalSocketIpc::bound` risolve il nome `harness-core` con `to_ns_name::<GenericNamespaced>()`, che su Linux *«resolves to the abstract namespace»* — il sorgente di `interprocess` 2.4.4 | lo stesso nome diventa la pipe `\\.\pipe\harness-core` |
| chi lo apre | **ogni account della macchina**: *«Socket permissions have no meaning for abstract sockets»* — `unix(7)` | nessun descrittore di sicurezza esplicito, quindi quello di base: *«full control to the LocalSystem account, administrators, and the creator owner»*, e lettura a Everyone e all'account anonimo — la documentazione di `CreateNamedPipe` |
| dalla rete | no: la famiglia `AF_UNIX` *«is used to communicate between processes on the same machine»* — `unix(7)` | no: `interprocess` crea ogni istanza con `PIPE_REJECT_REMOTE_CLIENTS` |

Il `Hello` porta il timbro di build, che è un controllo di **versione** e non di identità: a chi ne manda uno sbagliato
il core risponde col timbro giusto, in `StaleBuild` (`crates/kernel/src/serving.rs`). Dopo il `Hello` il core serve
`Invoke`, `Approve` — che scrive un `Permission` nel giornale, `Approval::JustGiven` in `crates/kernel/src/registry.rs`
— e `SaveLayout`. Con i sotto-progetti 3, 4 e 5 da qui passeranno la conversazione e le approvazioni di strumenti veri.

Nel prodotto **non esiste un utente**: la spec del kernel, §0.3 — *«singolo utente, nessuna multi-tenancy, nessuna
autenticazione»*. L'**account** di cui parla questo ADR è quello del **sistema operativo**, che il sistema conosce già
per ogni processo; ed è il confine che il progetto ha già per i dati a riposo —
[ADR-0023](0023-cifratura-a-riposo-e-gestore-dei-segreti.md) e [design/09](../design/09-l0-fisico.md): protegge da un
altro account, non da un programma che gira come te. Il canale ne restava fuori.

| | La risposta del proprietario | Data |
|---|---|---|
| la domanda | *«Chi può parlare col core?»* — **A**: il solo account, controllato dal sistema ai due capi del canale. La B era «chiunque sulla macchina, scritto nell'ADR» | 2026-10-02 |
| il modo | questo disegno, approvato a condizione: niente di inventato né di assunto, il controllo alla radice e non funzione per funzione, secondo i criteri di `decision-principles` | 2026-10-02 |

**Lo stato dell'arte, letto alla fonte il 2026-10-02** e portato in [`riferimenti.md`](../riferimenti.md):

- **.NET**, `PipeOptions.CurrentUserOnly`: il server *«can only be connected to a client created by the same user»*, il
  client *«can only connect to a server created by the same user»*. Nel sorgente di `dotnet/runtime`, su Windows il
  server mette un descrittore con una regola sola — controllo pieno al proprietario del token corrente — e il client
  confronta il proprietario della pipe col proprietario del proprio token; su Unix il server confronta l'utente del pari
  col proprio, e il socket nasce leggibile e scrivibile dal solo proprietario.
- **XDG Base Directory** 0.8: `$XDG_RUNTIME_DIR` è la cartella dei file di runtime *«such as sockets, named pipes»*, e
  *«MUST be owned by the user, and they MUST be the only one having read and write access to it. Its Unix access mode
  MUST be 0700»*.
- **`unix(7)`**: *«pathname sockets honor the permissions of the directory they are in»*.
- **D-Bus**, la specifica 0.43: EXTERNAL, con le credenziali passate dal sistema, è il meccanismo raccomandato su Unix;
  e i socket astratti *«are namespaced according to network namespaces rather than being part of the filesystem»*, che
  *«can lead to a sandbox escape»* quando un confinamento cambia la vista del filesystem e non la rete.
- **VS Code**, `ipc.net.ts`: su Linux i socket in `XDG_RUNTIME_DIR`, e senza di essa nella cartella temporanea.
- **Moby**, `listeners_windows.go`: la pipe col descrittore esplicito, *«Any other user is denied access»*.
- **git**: su Windows `simple-ipc` dà a Everyone lettura e scrittura, perché un demone avviato da amministratore
  risponda ai comandi normali; su Unix `unix-stream-server.c`, sotto un lucchetto, prova a collegarsi al socket che
  trova, si ferma se risponde un server vivo, e altrimenti lo toglie e lega il proprio.

**Misurato il 2026-10-02 su questa macchina** — Windows 11, livello d'integrità medio, `rustc` 1.95.0, `interprocess`
2.4.4, una prova fuori dal repository, il protocollo in [`riferimenti.md`](../riferimenti.md): una pipe col descrittore
`D:P(A;;GA;;;<SID dell'utente>)` accetta tre client di fila; con lo stesso descrittore per un altro SID — `BG`, o un SID
inventato — la connessione dello stesso processo è respinta, `PermissionDenied`, errore 5; e il client legge dalla
propria connessione il proprietario della pipe, che è il SID dell'utente, col descrittore di base e con quello esplicito.

### Alternative considerate

- **Chiunque sulla macchina, scritto nell'ADR** — la B. *Contro:* il canale resta più largo del confine di ADR-0023, e
  coi sotto-progetti 4 e 5 un altro account approverebbe strumenti veri.
- **Un segreto nel `Hello`**, scritto dal core in un file dell'account e letto dalla GUI. *Contro:* il file lo legge il
  solo account, che il sistema controlla già; e il segreto sarebbe un campo nuovo dello schema, che si muove tutto
  insieme (I4).
- **Su Linux il nome resta astratto, e il core controlla l'utente del pari quando accetta.** *Contro:* chiude il capo del
  core e non l'altro — un nome astratto lo prende per primo chiunque, e la GUI dovrebbe controllare il pari da sé —; e un
  confinamento che restringe la vista del filesystem non lo chiude, come avverte D-Bus. La cartella privata chiude i due
  capi col solo sistema, e un confinamento che toglie la cartella toglie anche il canale.
- **Un nome di pipe per account, su Windows**, perché due account della stessa macchina abbiano ciascuno il proprio
  core. *Contro:* è una funzione nuova e non la risposta alla domanda, e il prodotto è a utente singolo (§0.3). Resta un
  limite dichiarato.

## Decision

| # | Decide | Da |
|---|---|---|
| 1 | **Il canale locale del core è del solo account del sistema operativo con cui il core gira.** Un processo di un altro account non lo apre; un processo dello stesso account sì, con tutto ciò che il canale serve | la risposta A · ADR-0023 |
| 2 | **Il controllo sta alla radice, e una volta sola: nel trasporto, prima di ogni messaggio**, in `platform` (I3). Nessun controllo per funzione né per messaggio; il kernel non lo vede; lo schema, il `Hello`, il timbro di build e le fixture non cambiano | I3 · I4 · la condizione del proprietario |
| 3 | **Lo fa il sistema operativo, ai due capi.** Il capo del core: un altro account non apre il canale. Il capo della GUI: la GUI non manda niente su un canale che non sia del suo account | la risposta A · .NET |
| 4 | **Windows.** La pipe nasce con un descrittore di sicurezza esplicito che dà accesso pieno al **solo** SID dell'account; l'accesso pieno comprende la creazione delle istanze dopo la prima, che la documentazione di Microsoft pretende dal server. Restano `PIPE_REJECT_REMOTE_CLIENTS` e `FILE_FLAG_FIRST_PIPE_INSTANCE`, che `interprocess` mette già. La GUI, prima di mandare qualcosa, legge dalla propria connessione il proprietario della pipe e lo confronta col proprio account | la misura del 2026-10-02 · Microsoft · .NET |
| 5 | **Linux.** Il socket è un file nella cartella di runtime dell'account, `$XDG_RUNTIME_DIR`. Prima di legarlo il core verifica che la cartella sia dell'account e chiusa agli altri — nessun permesso per il gruppo né per gli altri —, e se non lo è non parte e dice perché. Il resto lo fa il sistema, `unix(7)`: un altro account non apre il socket e non ne crea uno al suo posto. La GUI fa la stessa verifica sulla cartella prima di collegarsi | XDG 0.8 · `unix(7)` · VS Code |
| 6 | **Linux, il socket rimasto.** Il file del socket sopravvive a un core caduto senza svolgere lo stack — la documentazione di `interprocess` 2.4.4 — e, trovato, farebbe fallire il legame. Il core che lo trova prova a collegarsi: se risponde un core vivo si ferma, come fa oggi un secondo core; se non risponde nessuno toglie il file e lega il proprio; e lo fa sotto un lucchetto, perché l'istanza singola di ADR-0004 resti tale: senza, due avvii insieme possono togliersi il nome a vicenda. È la forma di git | git · `interprocess` 2.4.4 · ADR-0004 |
| 7 | **Senza `$XDG_RUNTIME_DIR` il core non parte**, e dice perché. ⚠️ XDG 0.8 dice che un'applicazione *«should fall back to a replacement directory with similar capabilities and print a warning message»*, e D-Bus e VS Code ne hanno uno, la cartella temporanea: il ripiego **non** si costruisce oggi, e la divergenza è dichiarata | la forma del fail-closed di ADR-0025 · la condizione del proprietario |
| 8 | **L'account atteso è consegnato al trasporto** dalla radice di composizione, che lo chiede al sistema attraverso `platform`: come il nome e il tetto del corpo, che `LocalSocketIpc::bound` già riceve. È ciò che rende provabile il rifiuto con un account solo | ADR-0034 · la misura del 2026-10-02 |
| 9 | **Chi è respinto non diventa un client**: lo respinge il sistema prima dell'ascoltatore, quindi nessun `ClientId`, nessuna riga nel giornale, nessun messaggio | la misura del 2026-10-02 · `unix(7)` |
| 10 | **Il `Hello` resta un controllo di versione**: il timbro di build non è un'identità e non lo diventa. Nessun segreto, nessun login | §0.3 della spec del kernel |
| 11 | **Fuori dal confine**, come in ADR-0023: un processo che gira come l'account | ADR-0023 · design/09 |

## Consequences

- **Positive:**
  - Il canale ha il confine dei dati a riposo: «protetto quanto il tuo account» vale anche per ciò che vi passa.
  - Un controllo solo, all'ingresso, copre ogni funzione di oggi e di domani: ciò che i sotto-progetti 3, 4 e 5 faranno
    passare dal canale nasce già dentro il confine.
  - Kernel, schema e fixture non cambiano.
  - Il rifiuto si prova con un account solo, nelle due direzioni.
- **Negative (accettate):**
  - **Un processo che gira come l'account** fa ciò che fa la GUI, `Approve` compreso: il confine di ADR-0023, accettato
    di nuovo.
  - **Su Windows il nome della pipe è uno per macchina.** Un altro account che lo prende per primo impedisce al core di
    partire — `FILE_FLAG_FIRST_PIPE_INSTANCE` fa fallire la creazione, e il core si ferma con `StartupError::Ipc` —, e
    per la stessa ragione due account non hanno due core insieme. Si perde il servizio, non i dati: la GUI non parla con
    una pipe che non è del suo account. Dalla documentazione di `CreateNamedPipe` e dalla sonda
    `a_second_core_on_the_same_channel_stops_the_start_up`, che lo misura con lo stesso account.
  - **L'elevazione non è misurata.** La misura è a livello medio, core e client da utente normale; un core avviato da
    amministratore con una GUI normale non è stato provato, e ciò che i due controlli faranno in quel caso non si scrive
    finché non si misura. Innesco: il primo avvio del core da amministratore.
  - **Linux non è misurato qui.** La regola è quella di `unix(7)` e di XDG, lette alla fonte; il cancello su
    `ubuntu-latest` proverà la parte nostra — la cartella verificata, il socket rimasto —, non il rifiuto di un altro
    account, che fa il sistema.
  - **Senza `$XDG_RUNTIME_DIR` il core non parte**, il punto 7. Innesco del ripiego: il primo Linux che lo chiede.
  - **La GUI di oggi non ha il suo capo.** La shell non esiste ancora — chi la costruisce lo dice la scelta su AUD-461 —,
    e il core finto e le prove sono client Rust: il controllo del capo della GUI nasce con la shell.
  - **Chiedere al sistema l'account** vuole una chiamata al sistema operativo in `platform`. `interprocess` 2.4.4 porta
    già nel grafo `windows-sys` e `libc` (`Cargo.lock`): se `platform` ne prende una come dipendenza diretta, si
    aggiunge in due passi, la regola di `CLAUDE.md` nata dal finding G-5.
- **Follow-up richiesti:**
  - **Chi costruisce il capo del core:** il pacchetto P06 del terzo audit, che tocca già `crates/platform/src/ipc.rs`,
    `crates/daemon/src/main.rs`, `gui/fake-core/src/main.rs` e `crates/platform/tests/ipc_contract_real.rs`; la sua
    specifica si rilegge contro questo ADR, e i suoi file si ricontano.
  - **Le prove, nelle due direzioni:** l'account entra con una connessione vera; con un account atteso diverso la
    connessione vera è respinta, su Windows, e il core non parte, su Linux; con una cartella aperta agli altri il core
    non parte; un socket rimasto senza core non ferma l'avvio, uno con un core vivo sì.
  - **La shell:** il controllo del capo della GUI, prima del primo messaggio — su Windows il proprietario della pipe, su
    Linux la cartella.
  - **AUD-687**, il limite di decodifica del filo, resta una decisione a sé: questo ADR restringe all'account chi può
    mandare un corpo cattivo, non lo toglie, perché lo può mandare anche una GUI difettosa.
  - I commenti del trasporto e del daemon che descrivono il pari come «local and trusted», o il nome come lo stesso sui
    due sistemi, si riallineano nel compito che costruisce.
