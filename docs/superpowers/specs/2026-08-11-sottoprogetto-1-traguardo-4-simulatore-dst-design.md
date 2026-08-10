# Traguardo 4 — il simulatore DST: il disegno

- **Data:** 2026-08-11
- **Stato:** disegno approvato, piano da scrivere
- **Sostituisce:** niente. **Precisa:** la [§3 della spec del sotto-progetto 1](2026-08-06-sottoprogetto-1-kernel.md), che descrive il simulatore **finito**

> 🎯 **A cosa serve questo file.** La §3 della spec dice **cosa** è il simulatore DST. Questo
> documento dice **quanto ne costruisce il Traguardo 4, dove vive ciascun pezzo e quale
> controllo lo esercita** — cioè le decisioni che il piano dovrà tradurre in compiti.
>
> ⛔ **Non è una spec nuova e non ne apre una.** La §3 resta la fonte; qui c'è lo
> scaglionamento, che la spec deliberatamente non fissa. Le decisioni che toccano un ADR
> **non vivono qui**: vanno nell'ADR, e la §2.2 dice quale e come.
>
> 📖 **Chi legge questo file:** chi scrive il piano del Traguardo 4, e chi lo esegue.

---

## 0. Il perimetro — cosa consegna il Traguardo 4

### 0.1 La decisione, e cosa l'ha presa

**Il Traguardo 4 costruisce il _motore_ della DST, non tutte le finte della §3.1.**

Il fatto che decide è un conteggio, ed è stato fatto leggendo la §3.3 contro il codice di
oggi invece che contro il piano: delle **dieci** righe di guasto della §3.3, **una sola** ha
oggi il proprio soggetto — la caduta fra intento ed esito sulla porta `journal`, che verifica
`Q5` e la cui riconciliazione esiste dal Traguardo 3. Le altre nove iniettano un guasto dentro
un meccanismo che **non esiste ancora**: l'arbitro è il Traguardo 5, il canale worker e lo
stato di degrado sono il Traguardo 6.

| Criterio | Cosa dice qui |
|---|---|
| **debiti futuri** | quattro finte senza consumatore sono quattro forme **congelate da un'ipotesi**. Il gotcha **#46** lo ha misurato: su una porta senza chiamanti nulla diventa rosso, quindi il difetto non si vede. È il debito peggiore — quello che *sembra* già fatto |
| **stato dell'arte** | la pratica DST costruisce **il banco deterministico** e fa crescere la superficie dei guasti insieme al sistema. Il banco è l'attivo permanente; le finte sono per-meccanismo |
| **coerenza** | è la decisione **D1 del piano del Traguardo 3** applicata di nuovo — *«congelare prima che un consumatore vero abbia esercitato la forma significa congelare la forma sbagliata»* |
| ⛔ **non pigrizia** | **non è la strada corta, è una strada diversa.** Dentro il proprio perimetro costa **di più** dell'alternativa: campagna permanente, oracolo di non-vacuità, iniezione a due livelli con due soggetti distinti — invece di quattro finte vuote che si scrivono in un pomeriggio e non provano nulla |

⚠️ **Il costo, dichiarato subito:** a fine Traguardo 4 la §3 della spec resta **coperta in
parte**. È accettabile **solo** perché la [§7](#7-cosa-non-entra-e-dove-va) dà a ogni riga
scoperta il proprio indirizzo: un arretrato con un indirizzo non è un arretrato, è uno
scaglionamento.

### 0.2 I sette artefatti, e il controllo che esercita ciascuno

⛔ **La colonna di destra è obbligatoria, e non è documentazione.** È la seconda domanda del
pre-controllo — *per ogni artefatto che il compito produce, quale controllo lo esercita?* —
posta **a tempo di disegno** invece che a tempo di dispaccio, perché è la sola classe di
difetto che non si vede rileggendo: non c'è niente da leggere.

| # | Artefatto | Vive in | Chi lo esercita |
|---|---|---|---|
| **1** | `CrashingJournal` — il giornale che cade alla scrittura scelta dal seme | `crates/simulator/src/journal.rs` | la campagna di livello 1, più una sonda che prova che **cade davvero** al punto dichiarato |
| **2** | `CrashingBackend` — il `redb::StorageBackend` che cade all'operazione scelta dal seme | ⛔ `crates/platform/tests/` — **un banco di prova, non `src/`**: vedi il richiamo del 2026-08-11 in §11 | la campagna di livello 2, più il **conteggio dei punti scattati** (§4.2) |
| **3** | la campagna di **livello 1** — riproducibilità e riconciliazione su N semi | `crates/simulator/tests/` | è essa stessa un controllo; la sua non-vacuità la tiene un **giornale che non cade mai** e uno che **cade sempre** |
| **4** | la campagna di **livello 2** — riapertura dell'archivio dopo la caduta | `crates/platform/tests/` | idem, più l'asserzione che `sync_data` sia **scattato** |
| **5** | `C7a` — senza crash, **nessun** passo in dubbio | con la campagna di livello 1 | il bugiardo è un giornale che dichiara dubbi che non ci sono |
| **6** | `C7b` — con crash, i passi in dubbio sono **quelli e solo quelli** | con la campagna di livello 1 | ⛔ confronto sull'**insieme**, mai sulla cardinalità — gotcha **#30** |
| **7** | l'elenco versionato dei semi | `docs/` | ⛔ **niente lo esercita, ed è deliberato** — §6 |

### 0.3 Definizione di «fatto»

⚠️ **Questa lista invecchia**, come tutte quelle prima di lei — è il gotcha **#49** applicato
al metro invece che all'oggetto. Si rilegge **contro il codice** prima di dichiarare chiuso il
traguardo, non contro sé stessa.

| # | Condizione |
|---|---|
| 1 | `bash scripts/gate.sh` → `GATE GREEN` |
| 2 | i **sette** artefatti della §0.2 esistono, ciascuno con il controllo dichiarato nella sua riga |
| 3 | `C7a` e `C7b` girano **a ogni commit**, e il tempo di parete si stampa (vincolo 7 della §11) |
| 4 | il gotcha **#51** è chiuso: togliere la durabilità a `FileJournal` fa **rosso** — provato togliendola davvero |
| 5 | ⛔ ogni campagna è provata **in due direzioni**: che trovi il difetto che c'è, e che **non** ne dichiari uno che non c'è |
| 6 | [ADR-0032](../../adr/0032-motore-di-persistenza.md) porta il proprio **rimando datato** sulla collocazione |
| 7 | l'elenco dei semi esiste e ogni voce **nomina il test permanente** della propria proprietà |
| 8 | il registro [`porta-di-qualita.md`](../../porta-di-qualita.md) ha una riga per ogni controllo nuovo, e i conteggi sono **ricontati**, non incrementati |

---

## 1. Cosa esiste già, misurato e non supposto

La quarta domanda del pre-controllo — *ciò che si detta di produrre esiste già?* — applicata
al traguardo intero. **Ha colto qualcosa**, ed è la ragione per cui questa sezione esiste
prima delle altre.

### 1.1 Quello che il Traguardo 2 ha già reso permanente

| Criterio di M-2 | Dove |
|---|---|
| **C1** — stesso seme, una sola traccia | `c1_the_same_seed_gives_one_single_trace` |
| **C2** — seme diverso, traccia diversa | `c2_a_different_seed_gives_a_different_trace` |
| **C3** — il tempo virtuale non si aspetta | `c3_virtual_time_does_not_wait` |
| **NV** — l'interlacciamento è reale | `non_vacuity_the_interleaving_is_real` |

Tutti e quattro in `crates/kernel/tests/executor_determinism.rs`, dal Traguardo 2. **Non si
riscrivono.**

### 1.2 Quello che manca

| | Stato |
|---|---|
| **C7a** — senza crash, nessun falso positivo | ❌ |
| **C7b** — crash riproducibile **dal seme** | ❌ |
| il giornale cadente | ❌ — e `MemoryJournal` lo dichiara nel proprio doc: *«THIS IS NOT THE FALLING DOUBLE»* |
| il backend cadente | ❌ |
| la campagna | ❌ |
| l'elenco dei semi | ❌ |

⚠️ **Un caso a metà, e la distinzione conta.** `a_crash_leaves_more_than_one_step_in_doubt`
esiste in `crates/kernel/tests/reconciliation.rs` dal Traguardo 3 — ma costruisce lo stato del
giornale **a mano**. Tiene la **proprietà** su un caso scelto da chi l'ha scritto; **non** la
tiene su uno spazio di semi. È esattamente lo scarto che la DST compra, e scriverlo evita di
concludere che C7b sia già coperto.

### 1.3 La riformulazione che ne esce

📌 **Il Traguardo 4 non porta il determinismo: quello c'è dal Traguardo 2. Porta il guasto.**

Non è una sottigliezza di parole: cambia cosa il piano deve dettare. Non *«costruisci il
simulatore»* — che invita a riscrivere ciò che esiste — ma *«dài al simulatore la sola cosa
che gli manca: rompersi»*.

---

## 2. I due livelli di crash sono **due campagne**

### 2.1 Perché hanno soggetti diversi

A parole: *«iniettare un crash»* significa due cose diverse a seconda di **chi è sotto esame**.

| | Livello 1 — alla porta | Livello 2 — dentro il motore |
|---|---|---|
| **Chi è sotto esame** | la **riconciliazione del kernel** | la **coerenza dopo crash di `redb`** attraverso il backend nostro |
| **Domanda** ([§4.6](2026-08-06-sottoprogetto-1-kernel.md)) | *il kernel si riconcilia bene?* | *il motore lascia un archivio recuperabile?* |
| **Cosa gira** | kernel `no_std` su finte pure | `FileJournal` vero, B-tree vero, I/O vero |
| **Costo per corsa** | **25,8 µs** misurati in M-2 → migliaia di semi | I/O vero → **decine** di semi |
| **Dove vive** | `crates/simulator/` | ⛔ `crates/platform/` |

### 2.2 ⛔ Dove vive il backend cadente — la collisione con ADR-0032

[ADR-0032](../../adr/0032-motore-di-persistenza.md) dice, in tabella, che il backend cadente
*«vive in `simulator`»*. **Non è eseguibile**, e la prova è una misura, non una deduzione:

| Guardia | Cosa succede se `redb` entra in `crates/simulator/` |
|---|---|
| `#![no_std]` | `redb` **4.1.0 non ha `no_std`** — le sue sole feature sono `cache_metrics` e `logging` — e tutti e sei i metodi di `StorageBackend` restituiscono `std::result::Result<_, std::io::Error>` |
| `scripts/gate-no-os.sh` | costruisce `simulator` per `x86_64-unknown-none` |
| `scripts/gate-deps.sh`, grafo **spedito** | la lista è `bincode · kernel · minicbor · simulator · unty`. `redb` esce come **«I3 violated»**, e il rimedio scritto dentro lo script è ⛔ *«REMOVE the dependency. Adding it to the list is not a remedy»* |
| `scripts/gate-attributes.sh` | pretende i tre attributi su `crates/simulator/src/lib.rs` |

⚠️ **Non è il gotcha #32, e la verifica è stata fatta prima di scrivere questa riga.**
ADR-0032 è stato aperto: la collocazione **non è una decisione misurata**. La misura di
quell'ADR fu presa con *«un backend scritto da noi, in memoria»* dentro uno **spike**; la riga
`simulator` è una **previsione** di dove sarebbe finito, scritta prima che `simulator` avesse
i propri vincoli. La spec non la ripete mai: la §3.1 assegna a `simulator` il giornale che
cade **alla porta** — livello 1 — e la §4.6 dice del livello 2 solo *«il `StorageBackend` di
`redb` cade»*, senza dire dove abiti.

📌 **La diagnosi è più utile dell'errore:** l'ADR mise entrambi in `simulator` **perché li
trattava come una cosa sola**. Non lo sono — soggetti diversi, costi diversi, cadenze diverse.

⛔ **Rimedio: un rimando datato in ADR-0032, non un `Superseded by`.** La decisione — `redb`
col `StorageBackend` scritto da noi, e il requisito 4 come ragione — **non è toccata**: è
sbagliata **una cella di una tabella**. È esattamente ciò che quell'ADR ha già fatto una volta,
il 2026-08-08, per la riga *«la lista delle dipendenze del kernel resta vuota»*. Gli ADR sono
append-only, e completare o correggere un contorno **non è superare la decisione**.

### 2.3 Le vie scartate, e perché

| Via | Perché cade |
|---|---|
| una **sesta crate** | il vincolo 1 della §11 dice **cinque**, e `platform` è **già fuori** dal perimetro di [ADR-0031](../../adr/0031-dipendenze-del-kernel-parte-del-confine.md): una crate nuova non compra nessun confine che non ci sia già |
| `redb` fra le **dev-dependencies** di `simulator` | passerebbe `gate-deps.sh`, che le esclude — ma **il codice di test di una crate non è raggiungibile da un'altra**, e chi deve usare il backend cadente è `platform`. Sarebbe un vicolo cieco scelto per salvare la lettera di una tabella |
| dietro una **feature** (`#[cfg(feature = "dst")]`) | creerebbe **due configurazioni di compilazione**, e il cancello ne proverebbe una sola. Una superficie che non gira mai è peggio di una dichiarata |
| ⛔ **`pub` in `crates/platform/src/journal.rs`** | è la via che questa sezione **proponeva** fino al richiamo del 2026-08-11 in §11, appoggiandosi al precedente di `abandon_without_commit`. **Il precedente non trasferisce**, ed è la ragione per cui cade: quel metodo è `pub` perché **non è scrivibile da fuori** — gli serve la transazione ancora aperta. Un backend **lo è**, e scriverlo dentro `platform` toglierebbe proprio la proprietà che il Task 8 ha comprato: che il confine sia raggiungibile **da fuori la crate** (gotcha #46) |

✅ **La via giusta ha già il proprio precedente, e stava scritta nel codice.**
`crates/platform/tests/file_journal.rs` porta `CountingBackend` — un `redb::StorageBackend`
scritto **da fuori** `platform`, in un banco di prova — e il suo commento dice testualmente che
*«Milestone 4 will put a FAILING one in the same place»*. `CrashingBackend` va lì: **zero
superficie di produzione**, e la prova che il confine è reale resta quella che vale, cioè
un'implementazione scritta da fuori.

---

## 3. Cosa significa «cadere»

### 3.1 Un errore restituito non è un crash

A parole: se il giornale risponde *«non ho potuto scrivere»*, il kernel lo **vede** e può
reagire. In un crash il processo **non c'è più** a metà operazione, e nessuno reagisce a nulla.
Sono due esperimenti diversi, e confonderli produce un banco che verifica la gestione degli
errori credendo di verificare la ripresa.

⛔ Lo spike modellava la caduta come `Err(Caduto)`, e la funzione chiamante tornava indietro.
**Non sale così**, per la stessa ragione per cui non sale l'aiutante `passo_in_dubbio` —
vincolo 6 della §11, gotcha **#20**.

### 3.2 Il modello di livello 1

| | |
|---|---|
| **prima** | il kernel gira sull'esecutore, scrive intenti ed esiti attraverso la porta |
| **la caduta** | alla scrittura *n* scelta dal seme, il giornale **smette di esistere**: la corsa è **abbandonata**, non gestita |
| **dopo** | si **riapre** sullo stato sopravvissuto e si chiama `kernel::reconcile::steps_in_doubt` |
| **l'asserzione** | i passi in dubbio sono **quell'insieme e non un altro** — ⛔ mai la sola cardinalità, gotcha **#30** |

⚠️ **`[3, 7]` e non `[3]`:** M-2 lo ha già misurato col seme 99, ed è la ragione per cui
`steps_in_doubt` restituisce un **insieme**. Un banco che confronta un solo passo dà un falso
negativo con l'interlacciamento.

### 3.3 Il modello di livello 2

| | |
|---|---|
| **prima** | `FileJournal` scrive attraverso il backend |
| **la caduta** | alla *n*-esima delle **sei** operazioni di `StorageBackend` il backend risponde errore **e non torna mai più utile** |
| **dopo** | il test **riapre il database** sullo stato sopravvissuto |
| **l'asserzione** | o ci sono i soli record confermati prima, **o tutti** — ⛔ mai uno stato parziale |

### 3.4 Il riuso: si avvolge, non si duplica

✅ **Misurato leggendo il sorgente di `redb` 4.1.0**, non ricordato: `StorageBackend` ha
**sei** operazioni — `len`, `read`, `set_len`, `sync_data`, `write`, e **`close`**, che ha
un'implementazione predefinita — e la crate espone già un **`InMemoryBackend`**.

Quindi:

| | |
|---|---|
| `CrashingBackend` | ⛔ **tiene il PROPRIO buffer dietro un `Arc`**, e **non** avvolge `redb::InMemoryBackend` — vedi il richiamo del 2026-08-11 in §11: quel tipo tiene i byte in un campo privato, quindi l'archivio **non si può riaprire** dopo la caduta, che è l'intera domanda del livello 2 |
| `CrashingJournal` | **avvolge `MemoryJournal`**: la sola differenza è il punto di caduta, e duplicare il doppio in memoria creerebbe due verità da tenere allineate |

---

## 4. Il gotcha #51 si chiude, e l'oracolo è **il conteggio**

### 4.1 La promessa che oggi nessuno tiene

`FileJournal` promette che una scrittura sopravviva alla morte del processo. Mettendo
`set_durability(Durability::None)` — cioè **togliendo la garanzia** — tutti e sei i suoi test
restano **verdi**. Non è una lacuna del banco: è la forma del banco, perché i test riaprono il
file dentro un processo **vivo**.

### 4.2 Perché il conteggio dei punti scattati è l'oracolo

⛔ **Il backend cadente la rende visibile, e non nel modo ovvio.** Con `Durability::None`,
`redb` **non chiama `sync_data`**. Quindi un backend che **conta** le chiamate lo dice: una
campagna che pretende *«`sync_data` è scattato almeno una volta»* diventa **rossa** appena la
garanzia sparisce.

📌 È il gotcha **#54** applicato all'iniezione invece che alla mutazione — *«prima di
concludere che qualcosa è invisibile dove lo vuoi invisibile, prova che sia osservabile da
qualche parte»* — e il **#17**: iniettare un guasto dove il codice non arriva è una prova
**vacua che sembra un successo**.

⛔ **Conseguenza per il piano:** il conteggio dei punti scattati non è un dato accessorio della
campagna. **È il suo oracolo di non-vacuità**, e va asserito, non stampato.

---

## 5. La cadenza — e due «livelli» con la stessa parola

### 5.1 La trappola di parola

⛔ **Due «livelli» diversi si incrociano proprio qui**, e chi legge in fretta ne deduce
un'implicazione che non c'è:

| Dove | «Livello 1» | «Livello 2» |
|---|---|---|
| vincolo 8 della §11 | **il compilatore** | **un controllo esterno** |
| ADR-0032 e §4.6 | il crash **alla porta** | il crash **dentro il motore** |

Chi legge *«livello 2 a ogni commit»* nel vincolo 8 e *«crash di livello 2»* in ADR-0032
conclude che la campagna dentro il motore giri a ogni commit **perché è di livello 2**. È un
caso, non un'implicazione. **Candidato gotcha nuovo.**

### 5.2 La campagna breve è un test, e non compra macchinario nuovo

**Decisione:** la campagna breve **è un test**. `cargo test --workspace` la esegue, e
`scripts/gate.sh` esegue già `cargo test` — quindi la cadenza *«a ogni commit»* del vincolo 8
è **già imposta**, senza un settimo controllo del cancello e senza un binario nuovo.

La campagna profonda resta fuori dal giro breve, scelta dal **numero di semi**.

### 5.3 Il tempo di parete

Il vincolo 7 della §11 pretende che **il tempo di parete si stampi a ogni corsa**, e un test
cattura l'uscita. Rimedio: il cancello lancia la campagna con `--nocapture` come **proprio
passo**. Costa poche righe di script e tiene tutto dentro il banco di prova.

---

## 6. I semi — un elenco che non deve mentire

La §3.4 è categorica: **un seme non è un oracolo permanente.** Riproduce un'esecuzione finché
il codice non cambia; modificato il kernel, lo stesso seme esplora un cammino diverso.

| Decisione | |
|---|---|
| l'elenco è **documentazione versionata** | seme → cosa trovò → ⛔ **il nome del test permanente della proprietà** |
| ⛔ **non si rigioca automaticamente** | rigiocare un seme il cui cammino è cambiato **non prova nulla e costa tempo**, ed è precisamente la falsa sicurezza che la §3.4 vieta — la stessa classe di *«cifrato a riposo»* dichiarato più forte di quanto sia |
| cosa protegge domani | la **proprietà**, che entra nella suite. Il seme serve a **indagare oggi** |

⚠️ **È la sola riga della §0.2 senza un controllo che la eserciti, e la casella è vuota di
proposito.** Un elenco di semi che qualcosa *esercita* è un elenco che qualcuno leggerà come
una rete di regressione — cioè l'errore che la §3.4 esiste per impedire.

---

## 7. Cosa NON entra, e dove va

⛔ **Ogni riga scoperta ha un indirizzo.** Un arretrato anonimo è ciò che questa tabella esiste
per non lasciare.

| Riga di guasto della §3.3 | Verifica | Va a |
|---|---|---|
| interlacciamento delle richieste concorrenti | `Q2` — somma delle concessioni ≤ budget | **Traguardo 5** — l'arbitro. ⚠️ L'interlacciamento **c'è già**; manca la proprietà |
| caduta durante la conservazione di un file | `Q22` — l'ambito torna byte-identico | **Traguardo 5/6** — con ambiti e checkpoint |
| uccisione di un worker | `Q4` · I1 · I5 | **Traguardo 6** — col canale worker |
| risposta assente o tardiva | `Q4` | **Traguardo 6** |
| frame malformato | `Q4` | **Traguardo 6** |
| frame non sollecitato | `Q4` · I5 | **Traguardo 6** |
| morte del worker a metà flusso | `Q4` · I1 | **Traguardo 6** |
| morte della gui a metà run | `Q3` · I1 | **Traguardo 6** — con `ipc` implementata |
| perdita della rete | `Q18` — degrado dichiarato prima | **Traguardo 6** — con lo stato di degrado |

⚠️ **Nessuno di questi stati cambia** per effetto di questo documento: `Q4` resta `parziale`,
e resterà tale finché non esiste un worker vero contro cui provare la conformità della finta.

---

## 8. I costi accettati

| Costo | |
|---|---|
| la §3 resta **coperta in parte** a fine traguardo | mitigato dalla §7: ogni riga ha un indirizzo |
| ⚠️ **il banco di livello 2 non è riusabile da un'altra crate** | il codice di test **non attraversa i confini di crate**: `CrashingBackend` serve solo a `platform`, e nessuno lo chiede altrove. ⛔ **Questa riga dichiarava un costo diverso** — *«superficie pubblica di prova in `platform`»* — e quel costo **non si paga più**: vedi il richiamo del 2026-08-11 in §11 |
| la campagna di livello 2 fa **I/O vero** | pochi semi, e il numero è **fissato e versionato** — vincolo 7 |
| [ADR-0032](../../adr/0032-motore-di-persistenza.md) prende un **rimando** | è la seconda volta per quell'ADR, ed è il meccanismo previsto, non una deroga |
| ⛔ **«la finta non è la vera» resta il punto cieco** | la §3.7 lo dichiara già. Il Traguardo 4 ne chiude **una parte** per il giornale — non tutto |
| il numero di semi della campagna breve **frena il commit** | va scelto contro il pavimento misurato (25,8 µs per corsa **minima**), e rimisurato quando gli scenari crescono |

---

## 9. Le misure di questo documento

Eseguite il **2026-08-11** · Windows 11 · toolchain `1.95.0` appuntata da `rust-toolchain.toml`.
Le fonti e i comandi stanno in [`riferimenti.md`](../../riferimenti.md).

| # | Misura | Esito |
|---|---|---|
| **D4-1** | `bash scripts/gate.sh` sul ramo, prima di toccare qualsiasi cosa | `GATE GREEN`, sei controlli su sei |
| **D4-2** | `redb` 4.1.0 supporta `no_std`? | ❌ **no** — nessun `#![no_std]`, e le sole feature sono `cache_metrics` e `logging` |
| **D4-3** | la superficie di `redb::StorageBackend` | **sei** metodi: `len`, `read`, `set_len`, `sync_data`, `write` obbligatori, **`close`** con implementazione predefinita |
| **D4-4** | esiste un backend in memoria già pronto? | ✅ **sì** — `InMemoryBackend`. ⛔ **Ma non serve**, e la ragione è D4-10 |
| **D4-9** | dove il repository dice già che vada il backend cadente | ⛔ `crates/platform/tests/file_journal.rs`, accanto a `CountingBackend`: *«Milestone 4 will put a FAILING one in the same place»*. Era **scritto nel codice** e questo documento non l'aveva letto |
| **D4-10** | `redb::InMemoryBackend` espone il proprio buffer? | ❌ **no** — è `InMemoryBackend(RwLock<Vec<u8>>)` con i guardiani `fn read`/`fn write` **privati**. I byte muoiono con l'oggetto, quindi l'archivio **non si può riaprire**: il cadente tiene il proprio buffer dietro un `Arc`, che è ciò con cui ADR-0032 misurò |
| **D4-5** | quali criteri di M-2 sono già permanenti nel repository | **C1, C2, C3, NV** in `crates/kernel/tests/executor_determinism.rs`. **C7a e C7b no** |
| **D4-6** | quali finte della §3.1 esistono in `crates/simulator/src/` | **tre su sette**: `VirtualReactor`, `SeededRng`, `MemoryJournal` |
| **D4-7** | quante righe di guasto della §3.3 hanno oggi il proprio soggetto | **una su dieci** |

⚠️ **D4-3 corregge una lettura affrettata di chi scrive, non l'ADR.** Una prima passata aveva
contato **cinque** metodi, perché l'estrazione si era fermata prima di `close`; ADR-0032 ne
dichiarava sei ed **aveva ragione**. Registrato invece che taciuto: il conteggio dei punti di
iniezione è l'oracolo della §4.2, quindi un metodo in meno sarebbe stato un oracolo più debole
senza che nulla lo dicesse.

---

## 10. Cosa questo documento lascia aperto

⛔ **Registrate, non prese.** Nessuna è un difetto oggi.

| | Perché non è decisa qui |
|---|---|
| **il numero di semi** della campagna breve e di quella profonda | va scelto **misurando** lo scenario vero, e lo scenario vero lo scrive il piano. Fissarlo adesso sarebbe un'ipotesi travestita da vincolo |
| **una guardia in `check-docs.sh`** che pretenda che ogni voce dell'elenco dei semi nomini un test esistente | è una **riga di catalogo nuova**, e quella è una decisione del proprietario. Un elenco di semi senza proprietà è l'artefatto che marcisce meglio di tutti — la proposta è scritta perché chi la riprende non debba riscoprirla |
| **il checkpoint** | `replay()` carica tutto in memoria e le guardie di `FileJournal` sono scansioni: il rimedio noto è lo stesso per entrambi. ⛔ Resta chiuso dal **primo consumatore che misuri un giornale grande**, non da qui |
| **le quattro finte mancanti** | nascono col meccanismo che le usa — §7. Il piano del Traguardo 4 **non le nomina** |

---

## 11. ⛔ Richiamo — 2026-08-11: due cose sbagliate, e le ha trovate il codice

Trovate **prima di scrivere il piano**, leggendo `crates/platform/tests/file_journal.rs` e il
sorgente di `redb` invece di fidarsi di questo documento. Corrette qui e **non riscritte in
silenzio**, perché il modo in cui sono state trovate vale più delle correzioni.

| | Diceva | È |
|---|---|---|
| **dove vive `CrashingBackend`** | `crates/platform/src/journal.rs`, `pub`, appoggiandosi al precedente di `abandon_without_commit` | ⛔ `crates/platform/tests/`, **un banco di prova**, come `CountingBackend`. Il precedente **non trasferisce**: `abandon_without_commit` è `pub` perché **non è scrivibile da fuori** — gli serve la transazione ancora aperta — mentre un backend lo è, ed è **da fuori** che deve essere scritto o non prova niente (gotcha **#46**). ⚠️ E la risposta era **già scritta nel codice**: il commento di `CountingBackend` dice *«Milestone 4 will put a FAILING one in the same place»* |
| **su cosa poggia `CrashingBackend`** | avvolge `redb::InMemoryBackend` | ⛔ tiene il **proprio buffer** dietro un `Arc`. `InMemoryBackend(RwLock<Vec<u8>>)` ha i guardiani **privati**: i byte muoiono con l'oggetto, quindi **l'archivio non si può riaprire dopo la caduta** — e riaprirlo è l'intera domanda del livello 2. ✅ È anche ciò con cui ADR-0032 misurò: *«un backend scritto da noi, in memoria»* |

📌 **La lezione, ed è la stessa che questo documento predica nella §1.** La §0.1 dice di leggere
la §3.3 *«contro il codice di oggi»*, e chi ha scritto le due righe qui sopra lo ha fatto per la
**spec** e non per il **codice**: le guardie erano state misurate, i banchi di prova no. ⛔ **Un
disegno si legge contro il codice esattamente come un compito**, e i due errori si sono
manifestati in modi diversi — il primo era **scritto** in un commento e bastava aprirlo, il
secondo si vedeva solo **leggendo la libreria**. 📌 Nessuno dei due si vedeva rileggendo questo
documento, che era coerente con sé stesso.

✅ **E il richiamo va a favore del disegno, non contro:** entrambe le correzioni **rafforzano** la
§2.2 — nessuna superficie di produzione, e la prova del confine resta quella scritta **da fuori
la crate**. Il costo dichiarato in §8 è cambiato di conseguenza.
