# ADR-0031: Le dipendenze del kernel sono parte del confine I3

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

[I3](0004-topologia-di-processo.md) stabilisce che il kernel non contiene codice
OS-specifico. [ADR-0026](0026-linguaggio-del-core.md) ne fa un errore del compilatore
invece che un lint: la crate del kernel è `#![no_std]`, quindi `std::fs` produce `E0433`.

Quell'affermazione è vera ed è stata misurata in entrambe le direzioni. **Ciò che non era
mai stato misurato è se bastasse.**

### La misura

Eseguita il **2026-08-06** · `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` ·
`madsim 0.2.34`. Prototipi usa-e-getta, fuori dal repository.

| Sonda | Attesa scritta prima | Misurato |
|---|---|---|
| **controllo** — crate `no_std` che nomina `std::fs` | `E0433` | ✅ `E0433`. Il controllo è attivo: le sonde seguenti non sono vacue |
| **A** — crate `no_std` che dipende da `madsim` | «fallisce, `madsim` richiede `std`» | ❌ **compila.** `Finished dev profile`, **55 crate** nel grafo |
| **A3** — crate `no_std` **e** `#![forbid(unsafe_code)]` che chiama una dipendenza la quale usa `std::fs` e `SystemTime` | — | ❌ **compila ed esegue**: legge un file dal disco e stampa l'orologio di sistema, **senza mai nominare `std`** |

**Causa.** `#![no_std]` toglie `std` dalla portata **dell'unità di compilazione che lo
dichiara**. Non è una proprietà transitiva del grafo: una dipendenza che usa `std` compila
normalmente, e il kernel può chiamarne le funzioni.

### Perché è emerso proprio ora

Valutando se adottare `madsim` come runtime deterministico. Aggiungerlo al kernel porta nel
grafo `tokio`, `mio`, `socket2`, `windows-sys` e — la peggiore — `getrandom` con `rand`:
una **sorgente di casualità seminata dall'OS** dentro il kernel. È una violazione di V29
nella forma esatta del gotcha #12: non compare in nessun elenco di «chiamate OS» e si
manifesterebbe come traccia divergente e inspiegabile.

Nessuno avrebbe scritto `getrandom` nel manifesto del kernel. Ci sarebbe arrivato
attraverso una dipendenza, che è il punto.

### Alternative considerate

- **A — zero dipendenze:** il kernel vede solo `core` e `alloc`.
  *Pro:* garanzia massima, nessun controllo da mantenere.
  *Contro:* serializzazione e strutture dati scritte da noi. È la trappola che il metodo
  del repository nomina esplicitamente — «non pigro» **non** significa «più costoso» — e
  nessuna invariante richiede questo prezzo.
- **B — allow-list esplicita, verificata sul grafo transitivo.**
  *Contro:* è un controllo, non il compilatore; e attrito su ogni dipendenza nuova.
- **C — nessuna regola:** `no_std` più disciplina.
  *Contro:* **misurato insufficiente.** È l'ipotesi che la sonda A3 ha falsificato.

## Decision

**Le crate che devono essere deterministiche e prive di OS — `kernel` e `simulator` —
hanno una lista nominata delle dipendenze ammesse, e la lista è verificata sul grafo
_transitivo_.**

| # | Regola |
|---|---|
| 1 | Ogni voce della lista porta la propria **giustificazione scritta**: perché serve, e cosa quella crate raggiunge |
| 2 | Il controllo è sul grafo **transitivo**, non sulle dipendenze dirette. È la lezione della misura: il pericolo arriva di rimbalzo |
| 3 | Il controllo è **provato in negativo** — introdotta una dipendenza non in lista, deve fallire; rimossa, deve passare |
| 4 | Aggiungere una voce è un **atto deliberato e rivedibile**, non uno scivolamento |
| 5 | La lista nasce **vuota** |

> **Dove vive la lista, e cosa contiene oggi.** Nata vuota, si è riempita alla misura
> **M-1** (§6.1.1 e §6.8 della spec del sotto-progetto 1): `kernel` ammette **`bincode`
> 2.0.1** e la sua dipendenza `unty`, per lo schema IPC.
>
> ⚠️ **`simulator` non aggiunge voci proprie, ma la sua lista non è vuota**: dipende da
> `kernel`, e la regola 2 è sul grafo **transitivo**. Misurato in M-3. Confondere «zero
> voci proprie» con «grafo vuoto» rende la regola 2 non applicata proprio dove serve.
>
> ✅ **Rimando — la §7.3.1 ha chiuso il meccanismo di verifica** (2026-08-07). La lista
> completa vive lì, con una colonna **classe** che questo ADR non prevedeva: il controllo
> verifica **due** grafi — le crate *spedite* e quelle *di build* — con due comandi, due
> errori e **due rimedi opposti**. Una violazione fra le spedite si ripara **togliendo** la
> dipendenza, non aggiungendola alla lista. Le dipendenze di **sviluppo** sono escluse, e
> l'esclusione è provata. Evidenze e sonde in §7.2.

**Perimetro, ed è ciò che rende la regola economica.** `platform`, `secrets` e `daemon`
**non** sono vincolati: è lì che l'I/O deve vivere, per I3. Il motore di persistenza, il
client di rete e le facility dell'OS stanno tutti fuori dal kernel, dietro una porta. La
lista vincola dove il costo è basso proprio perché l'architettura ha già spinto fuori ciò
che sarebbe caro.

## Consequences

- **Positive:**
  - I3 e V29 tornano verificabili **per intero** invece che a metà. Prima il confine era
    controllato su un lato solo e nessuno lo sapeva.
  - Il costo di una dipendenza diventa visibile **prima** di pagarlo, non dopo.
  - La scelta su `madsim` smette di essere un'opinione e diventa un conto: 55 crate di
    superficie non verificabile contro un esecutore che nel prototipo è ~30 righe.
  - Il criterio si applica da solo alle scelte future — serializzatore per lo schema IPC,
    e qualunque comodità dell'ecosistema si sia tentati di far entrare nel kernel.

- **Negative (accettate):**
  - **È un controllo, non il compilatore.** Un test si può cancellare; `#![no_std]` e
    `#![forbid(unsafe_code)]` no. Questa regola è **più debole** delle altre tre di
    ADR-0026, e va detto invece che sperato.
  - **Attrito su ogni dipendenza nuova del kernel:** valutare non la crate, ma il suo
    grafo transitivo. È lavoro reale, e ricade su chi propone.
  - **Il grafo cambia sotto di noi.** Un aggiornamento minore di una dipendenza ammessa
    può introdurne una nuova. Il controllo lo rileva, ma **dopo**: aggiornare una
    dipendenza del kernel diventa un evento da rivedere, non un'operazione automatica.
  - **Rischio di irrigidimento.** Una lista tenuta troppo stretta spinge a riscrivere cose
    che l'ecosistema ha già — cioè a pagare il costo dell'opzione A per gradi. Mitigazione
    dichiarata: la giustificazione si scrive per **ammettere**, non per escludere; il
    default è valutare, non rifiutare.
  - **Limita la superficie, non la certifica.** Una crate ammessa può comunque fare
    qualcosa di indesiderato. La lista riduce il numero di cose da guardare; non le guarda
    al posto nostro.

- **Follow-up richiesti:**
  - ~~Il controllo entra nella **porta di qualità** del sotto-progetto 1~~ ✅ **fatto**:
    §7.3.1, con le sonde N1–N4 viste fallire e tornare verdi (§7.2.2). ⚠️ La riverifica ha
    trovato che il comando indicato per separare i due grafi era **sbagliato**: `cargo tree
    -e no-proc-macro` da solo lascia dentro le dipendenze di *sviluppo*, e con esse
    `windows-sys`. Il comando corretto è `-e normal,no-proc-macro`.
  - ~~La ricerca di un serializzatore per lo schema IPC (misura M-1)~~ ✅ **chiusa**: la
    domanda riformulata — «esiste un serializzatore il cui grafo transitivo è
    accettabile?» — ha risposta **sì**, per tutti e cinque i candidati provati. Scelto
    `bincode`. Due cose emerse dalla misura ricadono su questo ADR: (a) compilare per un
    bersaglio **senza OS** è un controllo strutturale più forte della lista per nome, ma
    **non sufficiente** per via dell'unificazione delle feature; (b) «grafo transitivo»
    non distingue **runtime** da **tempo di compilazione**, e lo scarto è grande — la §7
    deve dichiarare quale dei due il controllo misura.
    ✅ **Entrambe chiuse dalla §7.3** (2026-08-07): (a) il cancello **si aggiunge** alla
    lista invece di sostituirla, perché i due falliscono in modo complementare — la lista
    nomina il colpevole, il cancello prova invece di enumerare; (b) il controllo misura
    **entrambi** i grafi, con rimedi opposti.
  - La scelta del **motore di persistenza** non è vincolata da questo ADR — vive in
    `platform` — ma il criterio va comunque dichiarato quando si scriverà quell'ADR, per
    evitare che qualcuno lo applichi dove non serve.
