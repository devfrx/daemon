# ADR-0032: Motore di persistenza — `redb`, con il backend sotto il nostro controllo

- **Status:** Accepted
- **Date:** 2026-08-07
- **Deciders:** proprietario del progetto

## Context

La [§10.6 della spec del kernel](../superpowers/specs/2026-08-06-kernel-design.md) fissa
**quattro requisiti** del motore di persistenza e rimanda la scelta a dopo il linguaggio.
[ADR-0022](0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) decide la semantica,
non la tecnologia.

| # | Requisito | Da |
|---|---|---|
| 1 | scrittura durevole e ordinata, con conferma prima dell'esecuzione | ADR-0007 · V6 |
| 2 | lettura concorrente mentre si scrive | GUI + core + proiezioni |
| 3 | potatura selettiva senza riscrivere l'archivio | ADR-0018 |
| 4 | ogni operazione di I/O **iniettabile** | V29 · ADR-0021 |

[`RISULTATI.md`](../../spikes/RISULTATI.md) è esplicito su quale decide: *«il discriminante
non è la disponibilità ma il requisito 4 — I/O iniettabile — che va confermato prima di
scegliere.»*

### Perché il requisito 4 non è una formalità

Ci sono **due** livelli di crash, e sostituire la porta ne copre uno solo.

| Livello | Dove si inietta | Risponde a |
|---|---|---|
| **1 — alla porta `journal`** | il simulatore sostituisce l'intero giornale | *il kernel si riconcilia correttamente?* |
| **2 — dentro il motore** | il backend del motore cade a metà scrittura o a metà `sync` | *il motore lascia un archivio recuperabile?* |

Il livello 2 è la *crash-consistency* della fonte citata in
[riferimenti.md](../riferimenti.md), ed è ciò che il requisito 4 rende verificabile. Senza,
resta il punto cieco dichiarato dalla spec del sotto-progetto 1: «la finta non è la vera».

### La misura — 2026-08-07

`rustc 1.95.0` · `cargo 1.95.0` · Windows 11. Versioni invariate rispetto a quelle
registrate in `RISULTATI.md` il 2026-08-06.

| Motore | Requisito 4 | Nota |
|---|---|---|
| **`redb` 4.1.0** | ✅ `pub trait StorageBackend` con `read` · `write` · `set_len` · **`sync_data`** · `len` · `close`, più un `InMemoryBackend` già pronto | Rust puro |
| `fjall` 3.1.8 | ❌ nessun tratto di backend: i tratti pubblici sono di configurazione e transazione | Rust puro; LSM, che `RISULTATI.md` annotava come **adatto alla potatura** |
| `rusqlite` 0.40.1 | ⚠️ si può *selezionare* una VFS per nome (`open_with_flags_and_vfs`), ma **registrarne una propria non ha API sicura**: si passa dalla FFI grezza | dipendenza **C**: `libsqlite3-sys` con `build.rs`, feature `bundled` / `buildtime_bindgen` |
| `sled` | ❌ nessun tratto di backend. E `cargo add sled` risolve a **0.34.7**, non alla `1.0.0-alpha.124` | la 1.0 è **alpha**: escluso dal precedente di [ADR-0027](0027-stack-della-gui.md), «non si fonda un sotto-progetto su una versione alpha» |

`sync_data` è la riga che decide: è **il confine di durabilità**, e poterlo sostituire
significa poter iniettare un crash esattamente lì.

### I quattro requisiti, misurati su `redb`

Backend scritto da noi, in memoria, che fallisce a un'operazione scelta.

| # | Esito |
|---|---|
| **1** | ✅ dopo riapertura i 10 record **confermati** ci sono; quello di una transazione mai confermata **non** c'è |
| **2** | ✅ un lettore aperto **prima** di una scrittura continua a vedere la propria istantanea (5 record) mentre si scrive; un lettore nuovo vede lo stato nuovo (10). Nessun blocco |
| **3** | ⚠️ **si stabilizza, ma in alto** — vedi sotto |
| **4** | ✅ **e dimostrato**: su 24 punti di iniezione, **12 hanno prodotto un fallimento osservabile**; di questi **12/12 hanno riaperto l'archivio** e **12/12 in stato coerente** — o i soli record confermati prima, o tutti, mai uno stato parziale |

#### Il requisito 3, per esteso

Carico sintetico: 2000 record da 1 KiB, poi otto giri di «pota 1500 payload sostituendoli
con un'impronta corta, e scrivine 1500 nuovi» — cioè il regime che
[ADR-0018](0018-ritenzione-a-livelli-del-giornale.md) descrive.

| Momento | Dimensione |
|---|---|
| base, 2000 × 1 KiB | 4 116 KiB |
| dopo 2 giri | 16 452 KiB |
| dopo 4 giri | **32 900 KiB** |
| dopo 6 giri | **32 900 KiB** |
| dopo 8 giri | **32 900 KiB** |
| dopo `compact()` | 28 416 KiB |

**Il dato che conta è che si stabilizza**: lo spazio liberato viene riusato, l'archivio non
cresce indefinitamente. Il costo è che si stabilizza **molto sopra** il dato vivo — in
questo carico circa un ordine di grandezza — e `compact()` ne recupera poco (~14 %).

La prima versione di questa misura guardava un solo giro e concludeva «lo spazio non viene
riusato». Era falso: serviva misurare il **regime**, non il transitorio.

### Alternative considerate

- **`fjall`.** Migliore proprio sul requisito 3, per natura dell'LSM.
  *Contro:* **fallisce il discriminante.** Il livello 2 di crash resterebbe non verificabile,
  e il requisito 4 era stato dichiarato decisivo *prima* di guardare i candidati.
- **`rusqlite`.** Motore maturissimo, e SQLite ha una VFS sostituibile per progetto.
  *Contro:* da Rust non esiste API sicura per registrarne una: servirebbe FFI grezza, cioè
  `unsafe`. E introduce una dipendenza **C** nel packaging.
- **`sled`.** Escluso su due fronti indipendenti: nessun tratto di backend, e la 1.0 è alpha.
- **Giornale scritto da noi**, file append-only.
  *Pro:* nessuna dipendenza, controllo totale.
  *Contro:* la parte difficile della persistenza è la **coerenza dopo un crash**, non
  l'append — esiste letteratura apposta, citata in [riferimenti.md](../riferimenti.md).
  Sarebbe «non pigro» nel senso che il metodo del repository vieta: più costoso *e* più
  rischioso.

## Decision

Il motore di persistenza è **`redb` 4.1.0**, usato con un **`StorageBackend` scritto da
noi** invece di quello su file predefinito.

Il backend nostro non è un dettaglio: è il punto in cui il requisito 4 diventa reale. Ne
esistono due implementazioni, come per ogni altra porta:

| Implementazione | Vive in | Fa |
|---|---|---|
| backend su file | `platform` | l'I/O vero |
| backend cadente in memoria | `simulator` | cade a un'operazione scelta dal seme — è **l'iniezione di livello 2** |

**`redb` vive in `platform`, quindi [ADR-0031](0031-dipendenze-del-kernel-parte-del-confine.md)
non lo vincola**: la lista delle dipendenze del kernel resta vuota. Il kernel conosce solo
la porta `journal`.

## Consequences

- **Positive:**
  - Il requisito 4 non è soddisfatto per dichiarazione ma **dimostrato**: 12 crash iniettati
    dentro il motore, 12 riaperture, 12 stati coerenti.
  - Chiude una parte del punto cieco della DST: il livello 2 di crash diventa verificabile,
    e non solo il livello 1.
  - Rust puro: nessun compilatore C nel packaging, che è un costo di L3 in meno rispetto a
    `rusqlite`.
  - I requisiti 1 e 2 sono soddisfatti e misurati, non assunti.

- **Negative (accettate):**
  - **Amplificazione dello spazio, misurata.** Nel carico sintetico l'archivio si stabilizza
    a ~33 MiB contro ~2 MiB di dato vivo, e `compact()` ne recupera il 14 %. Non cresce
    all'infinito — che è la proprietà che conta — ma la potatura **costa spazio**, e il
    numero va rimisurato sul carico reale del giornale.
  - **Si rinuncia al motore migliore proprio sulla potatura.** `fjall` sarebbe stato più
    adatto al requisito 3. Il discriminante era fissato prima, e ha deciso sul 4.
  - **`compact()` richiede accesso esclusivo** (`&mut Database`): è manutenzione, non
    un'operazione da fare mentre il sistema lavora. Va pianificata, e questo è lavoro in più.
  - **L'oracolo del crash conta i record, non ne verifica il contenuto integrale.** La
    coerenza dopo crash è *dimostrata su 12 punti*, non provata esaustivamente.
  - Una dipendenza in più in `platform`, con il proprio grafo transitivo da guardare — anche
    se ADR-0031 non la vincola.

- **Follow-up richiesti:**
  - **Rimisurare il requisito 3 sul carico reale del giornale** prima di congelare i
    parametri di ritenzione di ADR-0018. Il carico sintetico dice che si stabilizza; non
    dice a che livello lo farà con record veri.
  - Decidere **quando gira `compact()`**, dato che è esclusivo. Candidato naturale: alla
    ripresa, prima che il sistema accetti lavoro.
  - Il backend cadente entra nell'armamentario della DST come **punto di iniezione di
    livello 2**, accanto a quello di livello 1 alla porta.

---

## ✅ Rimando — «la lista delle dipendenze del kernel resta vuota» non è più vero (2026-08-08)

La riga che spiega perché `redb` non è vincolato da ADR-0031 dice: *«`redb` vive in
`platform`, quindi ADR-0031 non lo vincola: la lista delle dipendenze del kernel resta vuota.
Il kernel conosce solo la porta `journal`.»*

**La prima metà regge, la seconda no**, e va marcata invece che lasciata mentire:

| Affermazione | Oggi |
|---|---|
| `redb` vive in `platform` e ADR-0031 non lo vincola | ✅ **invariata**, ed è la sostanza di questo ADR |
| la lista delle dipendenze del kernel **resta vuota** | ⛔ **falsa**. Le voci spedite sono **tre**: `bincode` 2.0.1, `unty` 0.0.4 e `minicbor` 2.3.0 — vedi i due rimandi di [ADR-0031](0031-dipendenze-del-kernel-parte-del-confine.md) |
| il kernel conosce **solo la porta `journal`** | ⚠️ **precisata**: il kernel conosce la porta `journal`, che scambia **byte**, e possiede la **codifica** del record durevole — [ADR-0036](0036-evoluzione-del-formato-durevole-del-giornale.md), §4.9.3 |

⛔ **Nessuna delle tre correzioni tocca la decisione**: il motore resta `redb` 4.1.0 con lo
`StorageBackend` scritto da noi, e il requisito 4 — I/O iniettabile — resta il motivo per cui.
Ciò che è cambiato è il **contorno**, e un contorno stantio dentro un ADR `Accepted` è
esattamente la cosa che si cita invece di riverificare.

📌 Trovato in un audit sezione-contro-ADR il 2026-08-08. La lista era già cresciuta a due
voci **il giorno stesso** in cui questa riga fu scritta (M-1, `bincode` + `unty`): non è
invecchiata, non era mai stata vera. È il gotcha **#31**.
