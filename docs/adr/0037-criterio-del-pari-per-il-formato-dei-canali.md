# ADR-0037: Il criterio del pari — il formato di un canale privato si sceglie anche sull'ecosistema di chi lo legge

- **Status:** Accepted
- **Date:** 2026-08-08
- **Deciders:** proprietario del progetto

## Context

[ADR-0035](0035-porta-verso-i-worker-e-lettura-di-i4.md) lascia una domanda aperta e
dichiarata: *«che `bincode` sia decodificabile dal pari **Python** non è verificato, e non
lo si afferma […] va misurato in §6 prima di scegliere il formato»*. È il punto 2 di F1b.

Misurarla ha trovato che la domanda non era mai stata posta **per nessuno dei due canali
privati**, e che le risposte sono diverse.

### Il criterio che decideva, e cosa non guardava

M-1 (§6.8) ha scelto `bincode` chiedendo — nella forma che
[ADR-0031](0031-dipendenze-del-kernel-parte-del-confine.md) impone — *«esiste un
serializzatore il cui **grafo transitivo** sia accettabile?»*. È la domanda giusta per I3:
riguarda ciò che la crate raggiunge, cioè **il nostro lato del filo**.

Un canale privato però ha due capi, e il secondo non è Rust:
[ADR-0030](0030-framework-dell-interfaccia.md) mette TypeScript alla gui,
[ADR-0028](0028-ecosistema-dei-worker-ml.md) mette Python ai worker. Per quel capo il
criterio è un altro, e non era scritto da nessuna parte:

> **L'ecosistema del pari ha un lettore conforme e mantenuto?**

⚠️ **P1 non vi risponde, e sembra di sì.** Lo spike ha misurato duemila messaggi senza
perdite, ma con **due binari Rust** ai due capi
([`spikes/gui-ipc/src/bin/gui.rs`](../../spikes/gui-ipc/src/bin/gui.rs)): misura la
contropressione del trasporto, non la leggibilità del formato.

### Le due misure

**M-10 — il pari Python.** 2026-08-08 · `rustc 1.95.0` · Python 3.13.7. Sonda `#![no_std]`
+ `alloc` + `#![forbid(unsafe_code)]` pilotata da un driver `std`, come in M-1;
vocabolario rappresentativo del dialogo con un worker — enum a più varianti, `String`,
`Vec<u8>`, `Option`, interi di due larghezze, `bool`.

| Candidato | Esito |
|---|---|
| `attrs2bin` 0.0.1 — unica release, **2020-03-22** | ⛔ **33 B** dove `bincode` 2 ne produce **12**: è la configurazione **1.x**, fixint a otto byte. Sui byte veri solleva `IncompleteOrCorruptedStreamError`. E non ha **tipi somma**: l'enum, che è la forma di ogni messaggio, non è esprimibile |
| `bincode` 0.1 su PyPI | ⛔ **non è bincode**: il wheel installa un modulo `b64tools`, funzioni base64 |
| `serde-generate` | ⛔ genera Python per «Bincode (default configuration only)» con `bincode ^1.3.3`: è la **1.x**. E richiederebbe `serde` nel grafo spedito — ciò che M-1 ha respinto per `postcard` |
| decodificatore scritto da noi | ✅ **130 righe**, tutti e otto i casi con i valori giusti |
| **`minicbor` 2.3.0 letto da `cbor2` 6.1.4** | ✅ tutti i casi con i valori giusti |

**M-11 — il pari TypeScript.** 2026-08-08 · Node `v24.9.0` · npm `11.6.0`, le stesse
versioni di [ADR-0026](0026-linguaggio-del-core.md). Stessi byte, stesso confronto.

| Candidato | Esito |
|---|---|
| **`bincode-ts` 1.0.0** | ✅ **tutti i casi con i valori giusti**, byte tutti consumati. Dichiara `{endian: little, intEncoding: variant}`, cioè `config::standard()` di `bincode` 2 |
| `ts-rust-bridge-bincode` 0.3.0 (2019) · `ts-binary` (2020) | ⛔ epoca bincode 1.x, fermi |
| `@inversealtruism/csd-codec` | ⛔ dichiara **fixint-LE**: configurazione 1.x |
| **`cbor-x` 1.6.5** (2026-07-29) | ✅ tutti i casi con i valori giusti |

**Le sonde di non-vacuità, in due direzioni** (gotcha #14, #24): `attrs2bin` fa round-trip
con sé stesso, quindi il «no» su bincode è reale · byte CBOR dati a `bincode-ts` →
`BincodeError: Invalid enum variant index: 130`, quindi il «sì» non è compiacenza.

⚠️ **Due esiti negativi della prima corsa erano del banco.** Le varianti di `bincode-ts`
portano nome e valore su chiavi **simbolo**, invisibili a `JSON.stringify`. Corretto e
rimisurato. È il gotcha #17 nella direzione opposta: una prova che *sembra* un fallimento.

### La risposta, e perché non è simmetrica

| Canale | Pari | Lettore conforme e mantenuto per `bincode`? |
|---|---|---|
| `ipc` | **TypeScript** | ✅ sì |
| `process` | **Python** | ⛔ no |

I due canali hanno lo stesso requisito — privato, non versionato, nessun consumatore
esterno — e **ricevono risposte diverse**, perché i loro pari sono diversi.

## Alternative considerate

| # | Discriminante | **1** — un codificatore solo, CBOR ovunque | **2** — `bincode` sui due canali | **3** — un formato **per pari** |
|---|---|---|---|---|
| 1 | il pari ha un lettore conforme e mantenuto | ✅ | ⛔ Python no | ✅ |
| 2 | nessun decodificatore scritto e mantenuto da noi | ✅ | ⛔ 130 righe, con la trappola del varint | ✅ |
| 3 | gli indici di campo non si pagano dove non comprano nulla | ⛔ anche sul canale gui | ✅ | ✅ |
| 4 | nessun codificatore condiviso fra requisiti opposti | ⛔ | ✅ | ⛔ |
| 5 | non riapre una decisione presa, misurata e protetta (§6.1.1) | ⛔ | ✅ | ✅ |

**La 1 è stata proposta, e ritirata.** L'argomento era che la ripartizione dei
codificatori non seguisse quella dei requisiti. Regge solo se i due canali privati sono lo
stesso problema — e **M-11 misura che non lo sono**. Ritirarla è il caso d'uso di
`bincode`-su-`ipc` che questa ADR conferma, non un ripensamento.

**La 2 cade sul discriminante 2**, che è il più pesante: un decodificatore scritto da noi è
una **seconda definizione dello schema** in un altro linguaggio, allineata da nulla. E la
misura ne mostra il modo di fallire: un lettore che tratta il varint come un byte semplice
restituisce `('Esito', 7, 251, False)` invece di `('Esito', 7, 4096, True)`, **senza
sollevare nessuna eccezione**. È il gotcha #30 trasferito dall'archivio durevole al filo.

## Decision

> **Il formato di un canale privato si sceglie _anche_ sull'ecosistema del pari che lo
> legge, e la risposta si _misura per pari_ invece di dedurla.**

| # | Regola |
|---|---|
| 1 | prima di fissare il formato di un canale privato si **misura** se il pari ha un lettore **conforme e mantenuto**. La misura confronta i **valori**, non l'assenza di eccezioni |
| 2 | se ce l'ha, il criterio di ADR-0031 sul **grafo transitivo** resta l'unico a decidere |
| 3 | se non ce l'ha, il canale prende un formato che il pari **legge già** |
| 4 | due canali privati possono quindi avere formati **diversi**: la differenza è misurata, non accidentale, e **non è un'incoerenza da sanare** |
| 5 | un decodificatore scritto e mantenuto **da noi** nel linguaggio del pari **non è una via**: è una seconda definizione dello schema |

### Cosa la decisione fissa, canale per canale

| Artefatto | Cosa decide | Formato |
|---|---|---|
| `ipc` — canale gui | il pari **ha** il lettore (M-11) → decide ADR-0031 | **`bincode` 2.0.1**, invariato: §6.1.1 è **confermata**, non riaperta |
| `process` — canale worker | il pari **non** ha il lettore (M-10) → regola 3 | **`minicbor` 2.3.0**, già voce spedita: nessuna aggiunta ad ADR-0031 |
| giornale | deve **evolvere** → [ADR-0036](0036-evoluzione-del-formato-durevole-del-giornale.md) | `minicbor` 2.3.0, invariato |

### Perimetro negativo — cosa questa decisione **non** è

| Non è | |
|---|---|
| una **riapertura di §6.1.1** | la **conferma**, con l'evidenza che le mancava. `bincode` resta, e resta appuntato a `2` |
| un **terzo serializzatore** | le voci spedite restano `bincode`, `unty`, `minicbor` |
| una regola sui **canali pubblici** | non ne esistono, e [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md) fa in modo che non nascano |
| la decisione di **come la GUI decodificherà** | è sotto-progetto 2. Qui si dichiara solo che una via esiste, e a che condizioni |
| un criterio che **sostituisce** ADR-0031 | vi si aggiunge: il grafo transitivo continua a decidere ciò che entra nel kernel |

## Consequences

- **Positive:**
  - **L'asimmetria diventa leggibile.** Due canali privati con due formati sono una
    stranezza che invita a essere «sistemata»: senza la ragione scritta, il primo lettore
    competente riapre §6.1.1. **È successo mentre si scriveva questa ADR**, ed è la prova
    del suo valore.
  - **La domanda aperta di ADR-0035 si chiude**, e si chiude per entrambi i pari invece
    che per quello che l'aveva sollevata.
  - **§6.1.1 acquista l'evidenza che non aveva.** La scelta di `bincode` era corretta;
    finora era corretta **per una ragione sola su due**, e la seconda non era stata
    guardata.
  - **Costa poco perché non aggiunge niente**: zero voci nuove nella lista di ADR-0031,
    zero porte, zero meccanismi. Ciò che cresce è un criterio e due misure.

- **Negative (accettate):**
  - ⚠️ **`minicbor` serve due artefatti con requisiti opposti** — il giornale, che *deve*
    evolvere, e il canale worker, che vi rinuncia. Un cambiamento fatto per l'uno tocca
    l'altro. È esattamente l'obiezione che il richiamo di §6.8 chiama «la gamba più forte»,
    e qui si paga invece di negarla: la contiene il **pin nel manifesto**, che rende ogni
    movimento un atto deliberato, e il fatto che gli **schemi restano distinti**
    (ADR-0035, regola 2).
  - **Il canale worker paga `#[n(i)]` su ogni campo** per un beneficio a cui I4 rinuncia.
    È il costo che M-1 aveva respinto per il canale gui; qui si paga per un'altra ragione,
    e la ragione va scritta o la riga sembra una contraddizione.
  - ⚠️ **Il pari TypeScript regge su un pacchetto solo, ed è fragile.** `bincode-ts` 1.0.0
    è a **una sola versione**; **entrambi** i punti d'ingresso pubblicati sono rotti su
    Node 24 — CJS `exports is not defined`, ESM import senza estensione — e ha funzionato
    solo dietro un bundler; il README si dichiara generato da un LLM; i tipi documentano
    `EnumVariant` mentre a runtime esiste `Variant`. **Non cambia la decisione** — il
    sotto-progetto 2 può ancora specchiare i tipi a mano — ma è dichiarato qui perché non
    venga **scoperto** allora.
  - **Il criterio è di livello 0: va applicato, e niente lo impone.** Un canale nuovo
    aggiunto senza misurare il pari non fa diventare rosso nulla. È la stessa forma del
    limite dichiarato in §2.8.4.

- **Follow-up richiesti:**
  - La **§6.10** progetta il canale worker sotto questo criterio, e ne porta le due regole
    che escono dalla misura: lunghezza del frame verificata, e annotazione di stringa di
    byte obbligatoria.
  - **§6.1.1** riceve il richiamo datato con M-11, e §6.8 la riga del pari.
  - Il **compendio** corregge la riga che difende i due serializzatori: gli artefatti sono
    tre, e a decidere non è solo il requisito.
  - La regola di lavoro — **cercare se un'idea è già stata scartata** — entra in
    `CLAUDE.md` e fra i gotcha.
