# Handoff — ripresa del progetto

Aggiornato il **2026-08-07**, alla chiusura della sessione che ha scritto le **§5 e §6**
della spec del sotto-progetto 1 e chiuso le misure **M-1**, **M-3** e **M-6**. Serve a
riprendere senza rifare, e senza rilitigare ciò che è già deciso.

## In trenta secondi

Assistente desktop locale, utente singolo, GPU singola RTX 5080 16 GB. **Piattaforma a
quattro pilastri paritari** su kernel comune. Spec del kernel **§0–§10 completa, 33 ADR**.
Stack deciso **tranne il guscio della GUI**: core in **Rust**, interfaccia web in **Vue 3**,
worker ML in **Python**; Tauri contro Electron è ancora aperto
([ADR-0029](adr/0029-guscio-della-gui.md), `Proposed`) e non blocca nulla.

**È in corso la spec del sotto-progetto 1** — implementazione del kernel + simulatore DST.
Sezioni **§0–§6 approvate**, §7–§8 da scrivere. Il codice non è ancora iniziato: vale
«spec prima del codice».

✅ **La lacuna su I2 è chiusa.** La GPU usata dalla GUI è governata da
[ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md): **quota di presentazione
sottratta, con la concessione tenuta dal core.** Il kernel non ha più lacune aperte.

Il vincolo che governa tutto non è funzionale ma di risorsa: quattro aree che si contendono
una sola GPU.

L'unico codice nel repository è in [`../spikes/rust/`](../spikes/rust/): sono **prove**,
non il kernel.

## Prima cosa da fare

**Riprendere la spec del sotto-progetto 1 dalla §7** — la porta di qualità, cioè i
controlli automatici. Si **presenta**, si discute, si approva, si scrive: mai tutto
insieme. Poi §8. Poi il piano. Poi il codice.

✅ **Nessuna misura la blocca.** M-3 è chiusa (evidenze più sotto, e vanno **trasferite
nella §7**). Le misure ancora aperte — M5 — richiedono una GUI, cioè il sotto-progetto 2.

### ⚠️ Le due domande che la §7 deve decidere, e che nessuna misura decide al posto sua

Le ha sollevate M-1 (§6.8.2) e M-3 le ha rese concrete con dei numeri.

| # | Domanda | Cosa si sa già |
|---|---|---|
| **1** | il controllo della allow-list misura il grafo di **runtime** o quello **totale**? | lo scarto è reale e misurato: `kernel` ha **2** crate esterne a runtime e **4** in totale. Un proc-macro gira sull'host durante la build: **non può violare V29 a runtime**, ma **è superficie di supply chain**. `cargo tree -e no-proc-macro` separa i due, quindi entrambe le scelte sono implementabili — è una decisione, non un vincolo |
| **2** | il **cancello bare-metal** entra fra i controlli automatici? | è provato che funziona (sonde B1/B2) e che è più forte della lista per nome — *prova* le crate invece di enumerarle. ⚠️ Ma l'unificazione delle feature di cargo può accendere, nella build reale per Windows, ciò che sul bare-metal restava spento: **condizione necessaria forte, non sufficiente**. Va deciso se si aggiunge alla lista o la sostituisce — la raccomandazione di chi ha misurato è **aggiungere, non sostituire** |

📄 [`superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md)

## Stato del sotto-progetto 1

| § | Sezione | Stato | Cosa ha deciso |
|---|---|---|---|
| 0 | Perimetro e criterio di scaglionamento | ✅ | cosa entra e cosa si scaglia, con un criterio falsificabile a tre regole (A/B/C) |
| 1 | Struttura delle crate e regole di importazione | ✅ | cinque crate: `kernel` · `platform` · `secrets` · `simulator` · `daemon`. Più [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md), nato da una misura |
| 2 | Il substrato iniettabile | ✅ | esecutore nel `kernel`, `Reactor` come porta, nessun thread nel percorso decisionale |
| 3 | Il simulatore DST | ✅ | cosa sostituisce, tempo virtuale, iniezione dei guasti, il seme e cosa **non** è |
| 4 | Giornale, riconciliazione, persistenza | ✅ | write-ahead, riconciliazione su un **insieme**, [ADR-0032](adr/0032-motore-di-persistenza.md) `redb` |
| 5 | Arbitro GPU, e la lacuna su I2 | ✅ | tre consumatori GPU nella GUI, quota di presentazione, I2 sui worker imposto dal **compilatore**. Più [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md) |
| 6 | Gateway, sensori, permessi, degrado | ✅ | schema IPC in `kernel` con **`bincode`**, **timbro di build** contro la GUI stantia, il **gettone non falsificabile** nominato una volta, «costo» del sensore separato in due |
| **7** | **La porta di qualità: i controlli automatici** | ⏭️ **da presentare** | **non bloccata**: M-3 è chiusa. Deve assorbire le evidenze di M-3 e decidere le **due domande** qui sopra |
| 8 | Copertura V1–V37 e Q1–Q24 | ⬜ | |

### Le decisioni aperte dalla §0.5 — tre previste, una emersa

| # | Decisione | Esito |
|---|---|---|
| 1 | GPU della GUI non arbitrata | ✅ [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md): quota di presentazione sottratta, concessione tenuta dal **core** |
| 2 | Motore di persistenza | ✅ [ADR-0032](adr/0032-motore-di-persistenza.md): `redb` 4.1.0 con backend nostro |
| 3 | Dove vive l'esecutore | ✅ nel `kernel`, con `Reactor` come porta (§2.4) |
| 4 | Dipendenze del kernel nel confine I3 | ✅ [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md) — **non prevista**, emersa da una misura |

### Misure eseguite, e quelle ancora aperte

Tutte con `rustc 1.95.0` · `cargo 1.95.0` · Windows 11. Evidenze complete nella spec.

| # | Domanda | Esito |
|---|---|---|
| M-4 | un runtime di ecosistema è usabile sotto `no_std`? | ✅ **sì** — l'attesa contraria era falsa. 55 crate nel grafo, fra cui `getrandom` |
| M-5 | un esecutore `no_std` senza `unsafe` fa avanzare `Future` reali? | ✅ **sì, con zero dipendenze**. Un `Waker` su misura invece **non** è costruibile: `E0133` |
| M-7 | quanto costa una decisione dell'arbitro? | `request` ≤ 100 ns · `release` p99 **500 ns** a coda realistica, 86,6 µs a coda 2000 |
| M-2 | `simulator` regge `no_std`? | ✅ **sì**. 100 corse → 1 traccia · 20 000 ms virtuali in **25,8 µs** · crash riproducibile 5/5 |
| M-8 | i quattro requisiti di §10.6 su `redb` | 1 ✅ · 2 ✅ · 3 ⚠️ si stabilizza in alto · 4 ✅ **12/12 crash recuperati** |
| M-6 | `BTreeMap`/`Vec` bastano alle strutture del kernel | ✅ **chiusa dall'esistenza di M-7**: il suo prototipo è `no_std`, zero dipendenze, tutto su `BTreeMap`, e l'arbitro è la struttura più complessa del kernel finora. Resta aperta solo per ciò che introdurrà la §6 |
| M-1 | serializzatore per lo schema IPC con **grafo transitivo** accettabile | ✅ **sì, tutti e cinque i candidati provati.** Scelto `bincode` 2.0.1 (2 crate di runtime). Esito **A**: lo schema sta in `kernel`, il grafo di §1.2 non cambia |
| M-3 | allow-list di ADR-0031 esprimibile con la toolchain standard, provata in negativo | ✅ **sì, esito A** — con `cargo tree`, **non** con `cargo metadata`. Quattro sonde in negativo, entrambe le direzioni dell'errore. Evidenze qui sotto |
| **M5** | quanta VRAM prende la presentazione della GUI | ⬜ **aperta e dichiarata tale** — richiede una GUI: sotto-progetto 2, accanto a M1–M4 di ADR-0029 |

#### M-3, per esteso — ⚠️ **da trasferire nella §7 quando la si scrive**

È l'unica misura le cui evidenze **non stanno ancora nella spec**, perché la sezione che
le ospita non è scritta. Trasferirle è parte del lavoro della §7, non un extra.

Eseguita il **2026-08-07** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11. Workspace di
prova che replica il layout reale: `kernel` (`no_std`+`forbid`, con `bincode`) ·
`simulator` (`no_std`+`forbid`, dipende da `kernel`) · `platform` (std) · `daemon`.

**Esito A: esprimibile con la sola toolchain standard.** Nessuno strumento esterno.

| Scoperta | |
|---|---|
| ⛔ **`cargo metadata` non va bene** | le *feature attive* che riporta sono corrette, ma il suo elenco `deps` **ignora le feature**: mostra anche le dipendenze opzionali spente. Sul caso reale segnalava 11 crate esterne invece di 2, fra cui `serde` e `syn` che **non vengono compilate** |
| ✅ **`cargo tree` sì** | risolve davvero le feature. `--prefix depth --format {p}` dà un output ricostruibile, e `-e no-proc-macro` separa il grafo di runtime da quello totale |
| costo dichiarato | `cargo tree` è un'interfaccia **pensata per gli umani**: nessuna garanzia di stabilità del formato, a differenza di `cargo metadata`. È il prezzo di avere le feature risolte |

**Le sonde, tutte viste fallire e poi tornare verdi:**

| # | Sonda | Atteso | Osservato |
|---|---|---|---|
| N1 | violazione **transitiva** — tolta `unty` dalla lista | fallisce **nominando il rimbalzo** | ✅ `X unty <- kernel -> bincode -> unty` |
| N2 | `getrandom` diretto in `kernel` | fallisce | ✅ `X getrandom <- kernel -> getrandom` |
| N3 | `getrandom` in `simulator` | fallisce | ✅ segnalato solo su `simulator`, non su `kernel` |
| **N4** | **guardia contro il falso positivo**: `getrandom` in **`platform`** | ⚠️ **non deve scattare** | ✅ `CONFORME`, exit 0 — e verificato che `platform` lo raggiunga davvero |
| B1 | cancello bare-metal su `kernel` e `simulator` | passano | ✅ entrambi |
| B2 | idem con `getrandom` in `kernel` | fallisce | ✅ `target is not supported` |

**N4 è la sonda che di solito si dimentica.** Un controllo che scatta dove non deve è
peggio di uno assente: insegna a ignorare l'audit. `platform` **deve** poter toccare
l'OS — è il perimetro esplicito di ADR-0031.

**La correzione che M-3 ha imposto a una riga della §6.1.1:** `simulator` non aggiunge
voci proprie, ma la sua lista **non è vuota** — dipende da `kernel`, e la regola 2 è sul
grafo *transitivo*. Scritto «resta vuota», misurato `bincode kernel unty`.

**Cosa la §7 deve ancora decidere**, e che M-3 non decide al posto suo: le due domande in
fondo a questo documento.

Lo stack non è più una domanda aperta:

| ADR | Decisione | Misurata da |
|---|---|---|
| **0026** | core in **Rust** | SP-5 e SP-6 su tre candidati. Rust è l'unico che passa entrambi |
| **0027** | GUI a **interfaccia web**, non toolkit nativo | G7, con P1–P4 misurati su un prototipo IPC |
| **0028** | worker ML in **Python** | non una scelta: i modelli hanno implementazioni Python |
| **0029** | ⚠️ **guscio: aperto** — Tauri o Electron | **niente**: sono argomenti, non misure. È il motivo per cui è `Proposed` |
| **0030** | interfaccia in **Vue 3** | merito + competenza del proprietario, criterio legittimo qui e non in ADR-0026 |

### L'unica cosa aperta, e perché non blocca

| Aperta | Si chiude con | Blocca il sotto-progetto 1? |
|---|---|---|
| **guscio della GUI** (ADR-0029) | cinque misure **M1–M5** su un frontend Vue minimo con scena 3D, sui due gusci | **no**: il sotto-progetto 1 è interamente Rust e non tocca la GUI |

#### Come la lacuna su I2 è stata chiusa — [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md)

Il problema: [ADR-0005](adr/0005-arbitrato-gpu-su-due-dimensioni.md) e
[design/02](design/02-arbitrato-gpu.md) non menzionavano mai la GUI, e la verifica di I2
era scritta sui soli worker.

**Le tre uscite enumerate non erano tre opzioni per un problema: erano tre risposte
parziali per tre consumatori diversi**, che erano stati trattati come uno solo.

| # | Consumo GPU della GUI | Governo | Rifiuto esecutivo? |
|---|---|---|---|
| 1 | compositing della webview | quota di presentazione sottratta, **concessione tenuta dal core** | ❌ no |
| 2 | viewer 3D entro la quota | stessa quota | ❌ no |
| 3 | viewer 3D oltre la quota | concessione ordinaria via IPC, prelazionabile | ✅ sì |

Tre cose da sapere, se qualcuno riapre il tema:

| | |
|---|---|
| **il titolare è il core, non la GUI** | la GUI non può *chiedere*: chi alloca è il compositor, che non ha un percorso di richiesta. Una quota sottratta **senza titolare** lascerebbe I2 falso — gotcha #4 |
| **I2 per la GUI è più debole in natura** | verso un worker il rifiuto è esecutivo, verso il compositor no. La quota è una promessa di budget, non un'imposizione. Dichiarato, non nascosto |
| **una divergenza registrata** | HANDOFF affermava che l'uscita A «incrina I1». **Non regge**: un worker tiene una concessione ed è dichiarato `possiede: nulla`. A è stata scartata per un motivo diverso e più forte |

Vale identico per Tauri e per Electron: **è una questione di kernel, non di guscio** — e
la §5 lo ha confermato con un motivo, esportando verso ADR-0029 il discriminante **M5**
invece di importarne uno.

Toolchain verificata il 2026-08-06: `rustc` 1.95.0 · `cargo` 1.95.0 · `clippy` 0.1.95.

### I quattro vincoli che ADR-0026 impone alla prima riga di codice

Conseguenze **misurate**, non raccomandazioni. Vanno tradotte in controlli automatici.

| # | Vincolo | Perché |
|---|---|---|
| 1 | il kernel è una **crate propria**, la piattaforma un'altra | i confini sono a granularità di crate, non di modulo |
| 2 | `#![forbid(unsafe_code)]`, **non** `deny` | `forbid` non è scavalcabile da un `#[allow]` locale (`E0453`) |
| 3 | la crate del kernel è `#![no_std]` + `alloc` | è ciò che rende `E0433` un errore del **compilatore** e non un lint |
| 4 | **`std::collections::HashMap` vietato** | vedi gotcha #12 |

## Non rilitigabile

32 ADR in stato `Accepted`. Rimetterne in discussione uno **richiede un ADR
nuovo che lo superi** (`Superseded by`), non una conversazione. Le decisioni che
è più probabile qualcuno voglia riaprire per comodità, e la ragione per cui non si fa:

| Decisione | Se la riapri |
|---|---|
| I quattro pilastri sono **paritari**; nessuno ha accesso privilegiato al kernel (ADR-0001) | il kernel diventa il servo di un pilastro e gli altri tre restano cittadini di seconda classe per sempre |
| **Tre** classi di processo, non quattro (ADR-0004) | la quarta si giustifica contro la tabella, o non si fa |
| **Nessun codice di terzi in-process** (ADR-0003) | rientrano contratto pubblico da congelare e superficie d'attacco |
| Default **OpenRouter, VRAM libera** (ADR-0006) | lo swap coordinato passa da eccezione a caso normale e cambia tutta la UX di attesa |
| **Fail-closed** sui vincoli dei dati (ADR-0012) | la protezione torna a essere una preferenza |
| Il **contesto è una proiezione**, non lo stato (ADR-0008) | le run lunghe tornano a perdere informazione in modo irreversibile |
| **Nessun modello** nel percorso decisionale del kernel (ADR-0020) | un fallimento del kernel smette di essere sempre un difetto, e la DST diventa impossibile |
| L'anello 4 **propone**, l'utente approva (ADR-0009) | il harness si auto-modifica in silenzio e diventa indebuggabile |
| Il core è **Rust** (ADR-0026) | riaprirlo significa rifare SP-5 e SP-6, i cui esiti sono misurati e registrati con seed e versioni. Il criterio che ha deciso è lo **spareggio #1**, e discende da V29 e ADR-0021: **rimettere in discussione il linguaggio significa rimettere in discussione la DST**, non il linguaggio |
| Le **dipendenze del kernel** sono parte del confine I3 (ADR-0031) | `no_std` blocca solo il *nominare* `std`. Misurato: una crate con `no_std` **e** `forbid(unsafe_code)` legge un file dal disco attraverso una dipendenza. Senza la lista, I3 è controllato su un lato solo |
| Il motore è **`redb` con backend nostro** (ADR-0032) | il backend nostro non è un dettaglio: è il punto in cui il **livello 2** di crash diventa iniettabile. Prenderne uno con l'I/O non sostituibile rinuncia a metà della verifica |
| L'**esecutore vive nel kernel** (§2.4) | prendere un runtime di ecosistema restituisce a lui l'ordine delle attività — cioè esattamente il controllo che lo spareggio #1 aveva comprato escludendo Go |
| La **concessione di presentazione la tiene il core** (ADR-0033) | la scorciatoia tentante è «esentiamo la GUI e amen». Esentarla rende **I2 falso** e indebolisce Q2 in silenzio; darla in mano alla GUI crea una concessione che si perde ogni volta che la GUI muore — cioè in qualsiasi istante, per G3. Il titolare deve avere vita lunga, e l'unico che ce l'ha è il core |

## Le tre proprietà che non si aggiungono dopo

Se le trascuri, la correzione non è una patch: è una riscrittura.

| # | Proprietà | Da |
|---|---|---|
| 1 | Confine dei dati non fidati **nel sistema di tipi** | I6 · ADR-0014 |
| 2 | Nessuna chiamata OS-specifica nel kernel | I3 · ADR-0002 |
| 3 | **Iniettabilità** di tempo, casualità, I/O e scheduling | V29 · ADR-0021 |

Più una quarta, di natura diversa ma altrettanto vincolante: **nessuna esecuzione di
codice o comando sotto il livello 2 di confinamento** (V35 · ADR-0025).

## I gotcha

Trappole reali, alcune trovate correggendo errori già commessi in questo progetto.

| # | Trappola | Perché fa male |
|---|---|---|
| 1 | **«Tutto è una run» vale solo per l'inferenza _generativa_** | applicarlo a wake word, VAD e trascrizione continua giornalerebbe migliaia di frammenti: viola Q1 e riempie il giornale di rumore. Quelle sono **sorgenti di eventi**, mai passi |
| 2 | **Ritentativo o passo nuovo?** Il discriminante è: *il modello ha prodotto output?* | no (trasporto, 5xx, rifiuto dell'arbitro) → stesso passo. Sì ma respinto da un sensore → passo nuovo, perché quell'output esiste, è stato pagato e deve restare visibile all'anello 4 |
| 3 | **Policy VRAM ≠ destinazione della richiesta** | V3 riguarda *cosa risiede in memoria*. In policy LOCALE una singola richiesta può finire su un provider remoto senza che la policy cambi |
| 4 | **La quota audio sottratta non esenta da I2** | il worker audio ha una concessione *permanente e non prelazionabile*, non l'assenza di concessione |
| 5 | **I permessi applicativi non sono un confine contro codice eseguito** | un processo figlio non passa dal mediatore: apre ciò che l'utente può aprire. Serve il livello 2 |
| 6 | **«Cifrato a riposo» qui vale quanto l'account OS** | va detto *in interfaccia*, non solo nell'ADR. Una falsa sicurezza è peggio di nessuna sicurezza |
| 7 | **Il giornale è la sorgente; trace, contesto, costi e metriche sono proiezioni** | non costruire un secondo sistema di osservabilità: esiste già, ed è il giornale |
| 8 | **Ogni requisito Q deve avere un metodo di verifica** (V30) | la §10 ha violato questa regola appena scritta, aggiungendo Q21–Q24 senza metodo. `scripts/check-docs.sh` ora lo rileva |
| 9 | **Go non ha test di compilazione fallita di serie** | un driver che compila un file *fuori* dal modulo fallisce per il motivo sbagliato: falso positivo. Va tenuto dentro il modulo, dietro un build tag |
| 10 | **xorshift resta bloccato su zero** | senza guardia sullo stato iniziale, certi seed producono una traccia vuota e lo spike sembra passare |
| 11 | **Il contesto degrada _prima_ che la finestra si riempia** (context rot) | compattare all'overflow significa lavorare degradati per gran parte di una run lunga. Si tiene un **budget target**, non una soglia |
| 12 | **`std::collections::HashMap` viola V29** | `RandomState` è seminato casualmente **per processo**: l'ordine di iterazione non è riproducibile fra esecuzioni. Non compare in nessun elenco di «chiamate OS» e si manifesta come traccia divergente e inspiegabile. Usare `BTreeMap`, o un hasher fissato. *(Vale anche altrove: in Go la randomizzazione delle `map` è deliberata — misurate 8 sequenze distinte su 200 iterazioni della stessa map, e lì non c'è alternativa ordinata nella libreria standard.)* |
| 13 | **Un lint non è il compilatore** | `clippy` ferma la violazione ma `cargo build` no, e un `#[allow]` per riga la annulla. Solo `forbid` e `no_std` producono un divieto non scavalcabile. **Misurato**: la regola clippy ha bloccato un uso *legittimo* di `Instant::now()` in un test, e ha richiesto un `allow` — cioè ha dimostrato di essere aggirabile mentre faceva il proprio lavoro |
| 14 | **Un test negativo va provato _in negativo_** | il piano degli spike conteneva **due sonde di non-vacuità sbagliate su tre**: quella di TypeScript modificava il tipo sbagliato e il controllo passava comunque, quindi non provava nulla. Un controllo che non si è visto fallire **non è un controllo**. Vale per ogni test di compilazione fallita, per ogni regola di importazione, per ogni `grep` di conformità |
| 15 | **Un'evidenza scritta prima della misura è un'ipotesi, non un risultato** | il piano dettava il testo delle evidenze da riportare. Tre di quelle affermazioni sono risultate **false** alla misura — inclusa una che nascondeva un buco reale nel confine dei tipi. Si esegue, si misura, si registra ciò che si è visto; dove diverge, si registra la divergenza |
| 16 | **`no_std` impedisce di _nominare_ `std`, non di _raggiungere_ l'OS** | non è transitivo sul grafo delle dipendenze. **Misurato**: una crate con `#![no_std]` **e** `#![forbid(unsafe_code)]` — gli attributi esatti che ADR-0026 impone al kernel — legge un file dal disco e l'orologio di sistema chiamando una dipendenza, e *compila ed esegue*. ADR-0026 resta corretto: dice che `E0433` blocca `std::fs`, e lo blocca. Ciò che non era mai stato misurato è che bastasse a garantire I3. **Non basta**: la lista delle dipendenze del kernel è l'altra metà del confine. Evidenze e comandi in [`specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) §1.4.1 |
| 17 | **Iniettare un guasto dove il codice non arriva è una prova _vacua_** | è il gotcha #14 travestito da successo: la prima misura sui crash di `redb` iniettava alle operazioni 12, 20 e 33, che quella transazione non raggiungeva mai. Tre prove su cinque non provavano nulla, e stavano per essere riportate come `5/5`. **Si conta prima quante operazioni compie davvero il codice**, e si inietta dentro quel numero — poi si verifica che il guasto sia *scattato*, non solo che il test sia passato |
| 18 | **Misurare il transitorio invece del regime** | la prima misura sulla potatura di `redb` guardava **un solo giro** e concludeva «lo spazio non viene riusato». Falso: a regime si stabilizza (32 900 KiB identici a 4, 6 e 8 giri). Per qualunque proprietà di stato stazionario, un solo giro non è una misura |
| 19 | **Un avanzamento nullo dichiarato riuscito è un ciclo infinito** | nel reattore a orologio virtuale, `advance()` prendeva il minimo di *tutte* le scadenze. Quelle dei task già conclusi sono nel passato: l'orologio non si muoveva e la funzione diceva di aver avanzato. Va filtrato alle scadenze **strettamente future**, e l'esecutore deve avere una **guardia sui giri** — un blocco va visto come errore, non come test che non finisce |
| 20 | **Un crash lascia _più_ passi in dubbio, non uno** | con esecuzione interlacciata due run possono avere entrambe l'intento scritto quando il processo cade. **Misurato**: seme 99 → passi `[3, 7]`. ADR-0007 diceva già «per *ogni* passo in dubbio», quindi la semantica reggeva — ma **l'aiutante `passo_in_dubbio` dello spike non sale così com'è**: restituiva un solo passo perché assumeva sequenzialità, e con l'interlacciamento dà un **falso negativo** |
| 21 | **Il rifiuto dell'arbitro è esecutivo solo verso ciò che avviamo noi** | un worker che riceve `Rifiutata` non parte: il rifiuto *è* il meccanismo. Il compositor della webview **compone lo stesso**, perché non lo avviamo noi, non dichiara un profilo e non ha un percorso di richiesta. Verso di lui una quota è una **promessa di budget, non un'imposizione**, e I2 vale in una forma più debole *in natura* — non per implementazione mancante. Il corollario che quasi sfugge: una quota sottratta **senza titolare della concessione** non salva I2 affatto (è il gotcha #4 letto al contrario). Il titolare dev'essere un processo a vita lunga, e l'unico è il core — ADR-0033 |
| 22 | **Che una versione esista non vuol dire che funzioni** | `cargo add bincode` risolve alla **3.0.0**, che è l'ultima pubblicata e il cui **intero sorgente** è `compile_error!("https://xkcd.com/2347/")`: un segnaposto contro l'occupazione del nome. La versione utile è la `2.0.1`, e il manifesto va **appuntato a `2`** con la ragione scritta accanto, o il prossimo aggiornamento «sistema» il vincolo e rompe la build. È la stessa classe della riga su `sled` in ADR-0032, ma peggiore: lì la versione utile era solo più vecchia, qui **la più recente esiste ed è inutilizzabile**. Corollario: in una misura sui candidati, `cargo add --dry-run` dice che il nome si risolve — **non** che il codice compili |
| 23 | **`cargo metadata` non risolve le feature; `cargo tree` sì** | i due strumenti danno grafi diversi sullo stesso workspace, e la differenza è grande. `cargo metadata` riporta correttamente le *feature attive* di ogni nodo, ma il suo elenco `deps` **le ignora**: elenca anche le dipendenze opzionali **spente**. **Misurato**: sul kernel con `bincode` senza la feature `serde`, `cargo metadata` segnalava **11** crate esterne — fra cui `serde` e `syn`, che non vengono compilate — contro le **2** reali di `cargo tree`. Un controllo di allow-list costruito sull'interfaccia macchina «giusta» sovra-segnala di 5×. Costo dell'alternativa, dichiarato: `cargo tree` è pensato per gli umani e non garantisce la stabilità del formato |
| 24 | **Un controllo si prova in _due_ direzioni, non una** | il gotcha #14 copre metà del problema: un controllo mai visto fallire non è un controllo. L'altra metà è che **un controllo che scatta dove non deve è peggio di uno assente**, perché insegna a ignorare l'audit. In M-3 la sonda decisiva è stata **N4**: mettere `getrandom` dentro `platform` — dove ADR-0031 lo **ammette** — e verificare che il controllo **resti verde**. Senza quella sonda, una regola troppo larga sarebbe passata per una regola che funziona. È la stessa ragione per cui `check-docs.sh` conta le sezioni duplicate **per file** e non sull'insieme |

## Il metodo di lavoro

Non è preferenza estetica: ha prodotto quattro incoerenze reali intercettate prima che
diventassero codice.

| Regola | |
|---|---|
| **Spec prima del codice** | nessun sotto-progetto si implementa senza spec approvata |
| **Sezione per sezione** | si propone, si discute, si approva, si scrive. Mai tutto insieme |
| **Decidere sul merito** | né scorciatoie né sovra-ingegnerizzazione. «Non pigro» **non** significa «più costoso»: la topologia a micro-servizi è stata scartata *perché* più costosa e peggiore |
| **Rendere verificabile** | un principio che non si può controllare è un'intenzione. Le invarianti diventano test |
| **Dichiarare i costi** | ogni ADR elenca cosa peggiora, non solo cosa migliora. Un ADR senza `Negative (accettate)` è incompleto |
| **Stato dell'arte verificato** | se una nozione non è certa si cerca **prima** di scrivere, e la fonte si traccia in [`riferimenti.md`](riferimenti.md). Mai inventare |
| **Schema-first** | tabelle, diagrammi, elenchi numerati. Niente muri di testo |
| **Audit a ogni chiusura** | `bash scripts/check-docs.sh` — link, indici, numerazioni, V30, ADR pendenti |

## Cosa NON rifare

| | |
|---|---|
| ❌ ri-derivare l'architettura | è in **33 ADR**, ciascuno con alternative scartate e motivo |
| ❌ riscrivere `tracciabilita.md` da zero | 170 funzionalità già mappate: si **aggiorna**, non si rigenera |
| ❌ ri-cercare lo stato dell'arte già tracciato | è in `riferimenti.md` con le fonti. Verificane semmai l'invecchiamento |
| ❌ rifare gli spike SP-5 e SP-6 | esiti, seed, versioni e comandi sono in [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md). I prototipi esclusi sono recuperabili dalla storia git, lo SHA è lì |
| ❌ progettare una capacità L2 | prima il kernel deve esistere (ADR-0001) |
| ⚠️ fidarsi delle fonti senza data | l'ecosistema si muove a cadenza mensile; `riferimenti.md` riporta la data di consultazione |

## Domande legittimamente aperte

Non sono lacune: sono decisioni **rimandate con criterio**, e ciascuna ha già il metodo
per chiudersi.

| Domanda | Si chiude con | Blocca? |
|---|---|---|
| ~~Linguaggio del core~~ | ✅ **ADR-0026: Rust** | — |
| ~~Interfaccia web o toolkit nativo~~ | ✅ **ADR-0027: web** | — |
| ~~Ecosistema dei worker ML~~ | ✅ **ADR-0028: Python** | — |
| ~~Framework dell'interfaccia~~ | ✅ **ADR-0030: Vue 3** | — |
| ⚠️ **Guscio: Tauri o Electron** | ADR-0029 `Proposed`, misure **M1–M5** | no |
| ~~GPU della GUI non arbitrata~~ | ✅ **ADR-0033**: quota di presentazione, concessione tenuta dal core | — |
| **Quanto vale la quota di presentazione** | **M5**, insieme a M1–M4 | no: default conservativo dichiarato non misurato |
| ~~Motore di persistenza~~ | ✅ **ADR-0032: `redb`** con backend nostro | — |
| CPU della GUI con rendering reale (P3) | rimisura nel sotto-progetto 2 | no: il margine misurato è 21,4 % su 25 %, **stretto** |
| Curva qualità/VRAM di TRELLIS2 | SP-1 | no: tara i profili di risorsa |
| Voce < 600 ms sotto carico | SP-2 | no |
| Budget della proiezione per modello | SP-3 | no: vale un default conservativo, dichiarato |
| Provider con annullamento senza addebito | SP-4 | no |

## Mappa dei documenti

| File | Cosa contiene |
|---|---|
| [`../CLAUDE.md`](../CLAUDE.md) | istruzioni operative per l'agente |
| [`roadmap.md`](roadmap.md) | 11 sotto-progetti, ordine, dipendenze, decisioni aperte |
| [`tracciabilita.md`](tracciabilita.md) | 170 funzionalità → dove vive ciascuna |
| [`README.md`](README.md) | indice di ADR e diagrammi |
| [`adr/`](adr/) | **33 decisioni**. Leggi **0001** e **0004** per primi: tutto il resto ne discende. Poi **0026** (linguaggio) se devi scrivere codice |
| [`design/`](design/) | 9 diagrammi Mermaid della struttura corrente |
| [`superpowers/specs/`](superpowers/specs/) | la spec del kernel §0–§10, **e quella del sotto-progetto 1** — §0–§6 approvate, con tutte le evidenze delle misure |
| [`superpowers/plans/`](superpowers/plans/) | il piano dello stack — **eseguito**, con l'errata in testa che documenta cosa il piano sbagliava |
| [`riferimenti.md`](riferimenti.md) | fonti esterne, con data e con **cosa non abbiamo adottato** |
| [`../spikes/`](../spikes/) | **prove, non kernel.** `PROTOCOLLO.md` criteri e soglie · `CANDIDATI.md` pre-selezione · `RISULTATI.md` esiti, seed, versioni, evidenze · `GUI-REQUISITI.md` G1–G21 e P1–P4 |
| [`../spikes/rust/`](../spikes/rust/) | il prototipo vincente: confine dei tipi, esecutore deterministico, `Future` native, giornale write-ahead. **Punto di partenza del simulatore** |
| [`../spikes/gui-ipc/`](../spikes/gui-ipc/) | prototipo IPC con P1–P4 misurati |
| `../scripts/check-docs.sh` | controllo di coerenza, verificato anche in negativo. **Da eseguire prima di ogni commit di documentazione** |

## Come si aggiorna questo handoff

Alla chiusura di ogni sotto-progetto, **nello stesso passaggio**: `roadmap.md`,
`tracciabilita.md`, lo stato degli spike, `CLAUDE.md` se cambia il prossimo passo, e
questo file se emergono gotcha nuovi.

`tracciabilita.md` **non** è stato toccato in questa sessione: nessuna funzionalità ha
cambiato sede, e la regola dice di aggiornarlo alla *chiusura* del sotto-progetto.

### Due trappole di `check-docs.sh`, da sapere prima di scrivere

**1 · I conteggi.** La guardia confronta con la realtà **ogni** occorrenza di
`<cifra> ADR`, `<cifra> ADR in stato ...` e `<cifra> decisioni architetturali` nei
documenti di stato. Scrivere `2 ADR nuovi` la fa scattare, perché legge `2` come il
totale. **Per i numeri piccoli si usano le parole** — «due ADR nuovi» — e le cifre si
riservano ai conteggi veri. Gli esempi vanno nei code span, che la guardia ignora.

⚠️ La guardia ha due punti ciechi **dichiarati**: un numero scritto a parole le è
invisibile, e così pure `<cifra> decisioni` senza «architetturali». Entrambi hanno già
prodotto conteggi stantii in questo repository.

**2 · La numerazione delle sezioni.** Il controllo sui duplicati è **per file**, e il suo
regex cattura `^#{2,3} <numero>`. Quindi `### 7.4.1` verrebbe letto come un duplicato di
`### 7.4`. **Le sotto-sotto-sezioni si scrivono con `####`**, che il regex non cattura —
verificato in questa sessione sulle §5 e §6, che ne hanno una decina ciascuna.

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
