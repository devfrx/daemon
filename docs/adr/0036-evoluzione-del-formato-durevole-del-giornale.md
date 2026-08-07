# ADR-0036: L'evoluzione del formato durevole del giornale

- **Status:** Accepted
- **Date:** 2026-08-07
- **Deciders:** proprietario del progetto

## Context

Il giornale ([ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md)) è l'**unico
archivio irriproducibile** del sistema. E chi lo rilegge non è chi lo ha scritto: è lo
**stesso programma, mesi dopo, con campi in più**. Nessuna riga del progetto diceva cosa
succede in quell'istante.

È la voce **F2** della riapertura del 2026-08-07, emersa rileggendo
[`tracciabilita.md`](../tracciabilita.md) con la domanda *«di quale meccanismo di kernel
ha bisogno questa funzionalità, e la spec lo nomina?»* — gotcha #27. La voce **F7** — fork
e branching — vi converge: sono un campo in più sul record, e sotto una regola di
evoluzione aggiungerlo è meccanico.

### Perché la postura di I4 non è disponibile qui

È [F1a](0035-porta-verso-i-worker-e-lettura-di-i4.md) con la risposta rovesciata:

| | canale IPC | giornale |
|---|---|---|
| i due capi | **spediscono insieme** | lo **stesso programma in due momenti diversi** |
| se divergono | si rifiuta il pari stantio: **timbro di build** (§6.1.2) | ⛔ **non si può rifiutare il passato** |
| evoluzione dello schema | **rinunciata** esplicitamente | **obbligatoria** |

### La scorciatoia da rifiutare, e ha un nome

> *«Usiamo `bincode` anche per il giornale, tanto è già nella lista di
> [ADR-0031](0031-dipendenze-del-kernel-parte-del-confine.md)»* — cioè importare in un
> artefatto che **deve** evolvere una decisione presa dove l'evoluzione era stata
> **rinunciata**.

La §6.8 della [spec del sotto-progetto 1](../superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md)
ha scartato `minicbor` per lo schema IPC perché i suoi indici di campo *«servono
all'evoluzione dello schema, cioè a un beneficio che I4 rinuncia esplicitamente»*. Quel
giudizio è **giusto per l'IPC e rovesciato per il giornale**, dove l'evoluzione non è un
beneficio di cui fare a meno ma il requisito.

### Quattro cose già decise che vincolano questa

| # | | Cosa impone |
|---|---|---|
| 1 | [ADR-0011](0011-routing-risolto-e-giornalato-per-richiesta.md): il record porta la decisione **risolta**, non un rimando alla configurazione | i record sono già **auto-contenuti**: metà del problema era risolta, e per un altro motivo |
| 2 | [ADR-0018](0018-ritenzione-a-livelli-del-giornale.md): un record potato **dichiara** di esserlo | esiste già un precedente di auto-descrizione **dentro** il record, e già una mutazione dopo la scrittura |
| 3 | §7.4.4 punto 3 della spec | ⚠️ la spec **presupponeva già l'evoluzione senza averla decisa**: il default `irripetibile` *«resta dov'è davvero utile — sui record riletti da un giornale scritto prima che la classe esistesse»*. Una regola di lettura in avanti per **un campo solo**, arrivata di straforo |
| 4 | [ADR-0032](0032-motore-di-persistenza.md): `redb` conserva **byte** | la codifica del record è **nostra**: la decisione è interamente qui |

## La misura

Eseguita il **2026-08-07** · `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` ·
Windows 11, profilo `release`. Prototipi usa-e-getta fuori dal repository. Versioni:
`bincode` **2.0.1** · `minicbor` **2.3.0** · `serde_json` **1.0.151** — le stesse di M-1.

**La domanda:** si scrive un record, si cambia il tipo, si rileggono **gli stessi byte**.
Il banco confronta i **valori**, non l'esito: senza quel confronto non si separa «errore»
da «silenzio sbagliato», che è l'unica distinzione che conta.

| | |
|---|---|
| ✅ **letto giusto** | i byte vecchi si rileggono con i valori corretti |
| ⚠️ **errore** | il lettore fallisce, rumorosamente. Accettabile: lo sai |
| ⛔ **silenzio sbagliato** | il lettore **riesce** e restituisce valori errati |

**Non-vacuità, in due direzioni:** il caso di controllo è ✅ su tutte e tre le classi — il
banco funziona — e il banco ha **prodotto davvero** ⛔ su cinque celle: sa distinguere.

### La matrice — byte vecchi riletti col tipo nuovo

| Mutazione | posizionale `bincode` | per indice `minicbor` | per nome `serde_json` |
|---|---|---|---|
| controllo, nessuna mutazione | ✅ | ✅ | ✅ |
| campo **obbligatorio** in coda | ⚠️ errore | ⚠️ errore | ⚠️ errore |
| campo **opzionale** in coda | ⚠️ **errore** | ✅ | ✅ |
| campo opzionale **in mezzo** | — | ✅ | ✅ |
| campo **rimosso** | ⛔ **silenzio** ¹ | ✅ | ✅ |
| campi **riordinati** | ⚠️ errore ² | ✅ | ✅ |
| variante di enum in coda | ✅ | ✅ | ✅ |
| variante **rinumerata** | ⛔ silenzio | ⛔ silenzio | ✅ |
| `u32` → `u64` | ✅ ³ | ✅ | ✅ |
| **direzione inversa** — nuovo → vecchio | ⛔ **silenzio** ¹ | ✅ | ✅ |
| indice ritirato, **buco** mai riusato | — | ✅ | — |
| indice **riusato** per un campo diverso | — | ⛔ silenzio | — |

¹ Il lettore restituisce i valori giusti e lascia **byte non letti**. Lo rileva **solo**
chi confronta i byte consumati con la lunghezza; senza quel confronto è un ✅ apparente.

² Errore **per fortuna, non per costruzione**: lo scambio fra un testo e un intero ha
fatto leggere una lunghezza assurda. Con due campi dello stesso tipo sarebbe stato
silenzio.

³ ✅ **solo per effetto del varint**. Con `config::legacy()` — larghezza fissa — lo stesso
caso dà `UnexpectedEnd`. Vedi il ritrovamento 4.

### Le dimensioni, sullo stesso record

| Formato | byte | contro `bincode` |
|---|---|---|
| `bincode` 2.0.1, `config::standard()` | **26** | — |
| `minicbor`, codifica ad **array** | **27** | **+4 %** |
| `minicbor` ad array **+ versione** | 30 | +15 % |
| `minicbor`, codifica a **mappa** | 33 | +27 % |
| `serde_json` | 76 | **+192 %** |

### I sei ritrovamenti

| # | |
|---|---|
| **1** | ⛔ **La forma C non è «una disciplina senza niente sotto»: non funziona affatto.** Su un formato posizionale **anche un campo opzionale in coda rende illeggibili i record vecchi** — l'opzionale scrive comunque un byte di presenza, che nei byte vecchi non c'è. C non compra nemmeno la compatibilità che promette. Dove funziona non è C: **è B** |
| **2** | ⛔ **La forma A da sola eredita la fragilità del formato sotto, e la trappola non si può chiudere.** Misurato: `bincode` **ignora i discriminanti espliciti** — una variante dichiarata `= 20` si codifica byte per byte come l'ordinale. Non si può appuntare il numero nel tipo |
| **3** | ✅ **L'indice compra quasi tutto, e costa un byte, non sette.** La stima corrente prezzava la codifica a **mappa**; la **predefinita è ad array**, e costa 27 byte contro 26 |
| **4** | ⚠️ **Una quarta fragilità che non era nell'elenco delle quattro forme:** il formato di `bincode` dipende da una **configurazione che non sta nel record**. `standard()` e `legacy()` producono byte diversi e non si leggono a vicenda. Cambiare configurazione **è** un cambio di formato, e nessun byte lo dichiara |
| **5** | **Nessun formato che identifica per numero sopravvive alla rinumerazione. Cambia la _visibilità_.** In `bincode` rinumerare significa **spostare una riga**, e non si vede; in `minicbor` significa **scrivere un numero diverso**, e si legge nel diff — stessa postura del gotcha #25 |
| **6** | **Solo il nome sopravvive alla rinumerazione, e costa +192 %.** Su un archivio che M-8 ha misurato stabilizzarsi a ~33 MiB contro ~2 MiB di dato vivo, moltiplicare per quasi tre il dato vivo non è gratis |

### Le divergenze dalle attese scritte prima

Quattro su nove. Registrate invece che allineate, come prescrive il metodo.

| Attesa | Misurato |
|---|---|
| campo opzionale in coda su posizionale: «forse errore», incerta | **errore, e strutturale** — è ciò che elimina la forma C |
| campo in mezzo su posizionale: «silenzio o errore» | **errore, ma accidentale**: dipende dai tipi in gioco, non dal formato |
| `u32` → `u64`: «⛔ o ⚠️» | ✅, per una ragione non prevista — il **varint**. Con `legacy()` è ⚠️ |
| costo della forma B: «permanente su ogni campo di ogni record» | **un byte su ventisei**: era prezzata la mappa invece dell'array |

### Le misure sul grafo, che decidono la collocazione

Su una crate `kernel` che replica il layout reale — `#![no_std]` + `#![forbid(unsafe_code)]`,
con **entrambi** i serializzatori:

| Verifica | Comando | Esito |
|---|---|---|
| compila | `cargo build -p kernel` | ✅ |
| **cancello senza OS** | `cargo build -p kernel --target x86_64-unknown-none` | ✅ **passa**, sul bersaglio di §7.3.2 |
| grafo **spedito** | `cargo tree -p kernel -e normal,no-proc-macro` | **3** crate: `bincode` · `minicbor` · `unty`. Erano due |
| grafo **di build** | il complemento su `-e no-dev` | **7** crate. Erano due: `minicbor-derive`, `syn`, `quote`, `proc-macro2`, `unicode-ident` in più |
| sorgenti di casualità nel grafo spedito | ispezione del grafo | **0** |

⚠️ Per la prima volta `kernel` porta **`syn`** a tempo di compilazione: `bincode_derive`
usa `virtue` apposta per evitarlo. È un «grafo di build cambiato» ai sensi di §7.3.1 —
ammissibile, ma **evento da rivedere**, non operazione automatica.

## Alternative considerate

Le quattro forme che la regola poteva prendere. La misura ne elimina una e ne indebolisce
un'altra: è ciò per cui è stata fatta.

- **A — discriminante di versione nel record**, e il lettore dispaccia.
  *Pro:* **livello 1** se il campo è obbligatorio nel tipo; ed è la sola forma che copre un
  cambiamento **non additivo** — spezzare un campo, cambiare unità, ristrutturare.
  *Contro misurato:* ⛔ **da sola non basta**. Sopra un formato posizionale la sua stessa
  enumerazione di versioni è posizionale, e la trappola **non si può chiudere** perché i
  discriminanti espliciti sono ignorati.

- **B — campi auto-descritti**, un indice accanto a ciascuno.
  *Pro misurato:* copre campo aggiunto, rimosso, riordinato, il buco e la direzione
  inversa, a **+4 %** di byte.
  *Contro:* non copre un cambiamento non additivo, e non impedisce di **riusare** un
  indice — misurato ⛔ silenzio.

- **C — disciplina solo-append:** campi nuovi facoltativi, mai rimossi, mai riordinati.
  ⛔ **Eliminata dalla misura, non da un giudizio.** Su un formato posizionale un campo
  opzionale in coda **rende illeggibili i record vecchi**: non compra la compatibilità che
  promette. Dove funziona, funziona perché sotto c'è un indice — cioè è B.

- **D — migrazione al riavvio:** rileggi vecchio, riscrivi nuovo.
  *Contro:* riscrive l'**unico archivio irriproducibile**, e un crash a metà migrazione è
  il caso peggiore del progetto. Resta l'uscita di emergenza se un giorno servisse un
  cambiamento che nemmeno A copre; non è la regola.

**Sulla collocazione**, due alternative:

- **la codifica in `platform`.** *Pro:* la lista di ADR-0031 non cresce di una riga.
  *Contro:* il simulatore tiene record tipizzati e **non esercita mai la codifica**, quindi
  la regola resta appesa a un solo controllo di livello 2; e la mappa tipo→byte — cioè la
  **forma durevole** — finisce fuori dal kernel, mentre §4.4 mette il modello dei dati in
  perimetro come cosa del kernel.
- **impl `Encode`/`Decode` scritte a mano**, per evitare le cinque crate di build.
  *Contro:* sposta il costo da una volta a **per sempre**, su ogni tipo di record.

## Decision

> **Ogni record durevole dichiara la propria versione, e i suoi campi si identificano per
> indice esplicito.** È la forma **A sopra B**: nessuna delle due da sola regge.

| # | Regola | Cosa la sostiene |
|---|---|---|
| 1 | il tipo del record è un **enum di versione**: «un record senza versione» non è esprimibile | **livello 1** — la stessa mossa con cui V5 è salita al compilatore (§7.4.4 punto 3) |
| 2 | ogni campo porta un **indice esplicito**, scritto nel tipo e leggibile nel diff | misurato: è l'indice a comprare il risultato, e costa un byte |
| 3 | un campo nuovo è **facoltativo** e prende un **indice nuovo** | misurato ✅ in coda, in mezzo e in direzione inversa |
| 4 | un indice **si ritira e non si riusa mai**: il buco resta | misurato: il buco ✅, il riuso ⛔ silenzio |
| 5 | un cambiamento **non additivo** apre una **versione nuova**; il lettore dispaccia e converte | è la sola cosa che l'indice non compra |
| 6 | la **codifica vive in `kernel`**, e la porta `journal` scambia **byte** | sotto |

**Perché la codifica sta in `kernel`:**

| # | |
|---|---|
| 1 | **coerenza di proprietà.** §4.4 mette il *modello dei dati* del giornale in perimetro come cosa del kernel, e ADR-0032 dice che `redb` conserva byte e la codifica è nostra. Se codificasse `platform`, la regola di evoluzione si applicherebbe dove il tipo non vive |
| 2 | **il controllo non resta appeso a un filo.** Con la porta a byte il **simulatore scambia byte**, quindi la campagna DST esercita davvero codifica e decodifica, e i crash si iniettano *dentro* la scrittura |
| 3 | **il costo dove conta è misurato e piccolo:** una crate spedita in più, senza dipendenze proprie, e il cancello senza OS passa |

### Il controllo — uno solo, e perché non due

| Livello | Difende | Meccanismo | Sonda — *deve scattare* | Contro-sonda — *deve restare verde* |
|---|---|---|---|---|
| **1 — compilatore** | regola 1 | il tipo del record è un enum di versione | costruire un record **senza versione** → non compila | con la versione → compila |
| **2 — controllo esterno** | regole 2–5 | **byte congelati**: i byte di un record scritto oggi entrano nel repository, insieme alla mappa `indice → nome del campo → valore atteso` | si **riusa** un indice o si rinumera → fallisce e **nomina il campo** | si aggiunge un campo facoltativo con indice nuovo → resta verde |

⛔ **La regola che impedisce al controllo di diventare una tautologia**, ed è il gotcha #25
applicato qui: **i byte congelati non si rigenerano**. Se cambiano non è un aggiornamento,
è un cambio di formato, e va aperta una versione nuova. Rigenerarli in blocco cancella
l'oracolo, esattamente come per gli `.stderr` di `trybuild`.

**Un controllo e non due.** Un registro degli indici separato dai byte congelati sarebbe un
secondo posto da tenere allineato per la stessa proprietà, e il primo che smette di essere
aggiornato mente in silenzio — §7.4.4 caso 2. Il file dei byte porta la mappa dentro di sé.

### Non serve un vincolo nuovo, e va detto

La regola difende **Q14** — *«ricostruire con cosa è stato eseguito un passo di sei mesi
fa»* — la cui cella in §8.4 dichiara che la proprietà è *strutturale*. Lo è rispetto alla
**configurazione**; non lo era rispetto al **formato**. Inventare un V38 gonfierebbe
l'insieme nominato della §8 senza aggiungere copertura, e costringerebbe a modificare anche
`check-docs.sh`.

### Perimetro negativo — cosa questa decisione **non** è

| Non è | |
|---|---|
| una **migrazione** dei record esistenti | non ce ne sono: alla data di questo ADR il prodotto non ha ancora scritto un record. È la ragione per cui la scadenza di F2 era **temporale** |
| un **formato di scambio** | il giornale non ha consumatori esterni. Nessuna negoziazione, nessuna compatibilità pubblica da mantenere |
| una **riapertura** della scelta per lo schema IPC | §6.8 resta valida, e per il motivo migliore dei due che dava — vedi le conseguenze |
| un **sistema di migrazione generico** | la regola 5 dice che il lettore dispaccia e converte; non esiste un motore di migrazioni, e non serve a nessuno adesso |

## Consequences

- **Positive:**
  - **La domanda «cosa succede rileggendo un record vecchio» ha una risposta scritta**, e
    non è più un'omissione che si scopre alla prima evoluzione.
  - **F7 diventa meccanico.** Fork e branching sono un campo facoltativo con un indice
    nuovo: la misura dice ✅ su quel caso esatto.
  - **La §7.4.4 punto 3 smette di presupporre.** La regola di lettura in avanti che era
    arrivata di straforo per un campo solo — il default `irripetibile` — ora **discende**
    da qui invece di essere un'eccezione non dichiarata.
  - **La DST guadagna un bersaglio.** Con la porta a byte, un crash iniettato al giornale
    cade dentro la scrittura di byte veri, non di una struttura in memoria.
  - **§6.8 esce rafforzata su una gamba e corretta su un'altra.** La sua premessa sul
    «costo permanente» degli indici è **misurata a un byte**, quindi cade; la sua
    conclusione regge sull'altra gamba, che è più forte: I4 non vuole l'evoluzione, e far
    condividere un formato a due artefatti con requisiti opposti significa che un
    cambiamento fatto per uno si propaga sull'altro.

- **Negative (accettate):**
  - **Il grafo di build del kernel passa da due voci a sette**, e per la prima volta porta
    `syn`. È superficie di supply chain a tempo di compilazione — non può violare V29 a
    runtime (§7.3.1) — ma va rivista, non subita.
  - **Il kernel porta due serializzatori**, uno per artefatto. È coerente — requisiti
    opposti, strumenti diversi — ma sono due grafi da guardare invece di uno, e due
    aggiornamenti che diventano eventi da rivedere invece di uno.
  - **Ogni campo di ogni record durevole porta un indice.** Si paga a ogni riga, non una
    volta. È il costo che la misura ha prezzato a un byte sul filo; sulla scrittura resta
    un'annotazione in più per campo.
  - **La regola 4 è una disciplina** — un indice non si riusa mai — e il controllo che la
    regge è di **livello 2**, quindi cancellabile. Non c'è un meccanismo del compilatore
    che impedisca di riusare un numero.
  - **I byte congelati sono un oracolo che qualcuno può rigenerare.** La difesa è che la
    rigenerazione **si legge nel diff**, non che sia impossibile. Stessa forza, e stessa
    debolezza, del gotcha #25.
  - ⛔ **Il livello 1 copre la presenza della versione, non la sua correttezza.** Il
    compilatore prova che un record **dichiara** una versione, non che sia quella giusta.
    È il limite del gettone (§6.3.2) applicato qui: prova la provenienza, non l'esattezza.
  - **La misura è su un record singolo.** Che il costo in byte regga a regime, su un carico
    reale, non è provato — vale la stessa riserva che ADR-0032 dichiara per il requisito 3.

- **Follow-up richiesti:**
  - La sezione che lo specifica è la **§4.9** della spec del sotto-progetto 1, con il
    controllo e le sue due sonde.
  - La **§4.1** dichiara che la porta `journal` scambia byte; **§6.1.1** e **§7.3.1**
    acquisiscono `minicbor` con la propria classe e giustificazione.
  - Il catalogo **§7.4** acquisisce le due voci del controllo, ciascuna con contro-sonda.
  - La **§8** registra lo stato di **Q14** e **Q5** — per ultima, e una volta sola.
  - **Rimisurare il costo in byte sul carico reale** del giornale prima di congelare i
    parametri di ritenzione di [ADR-0018](0018-ritenzione-a-livelli-del-giornale.md): è lo
    stesso follow-up che ADR-0032 chiede per l'amplificazione dello spazio, e le due misure
    si fanno insieme.
  - Se un giorno servisse un cambiamento che nemmeno la regola 5 copre, l'uscita è la forma
    **D** — migrazione al riavvio — e va presa con un ADR che ne dichiari il rischio, non
    come manutenzione.
