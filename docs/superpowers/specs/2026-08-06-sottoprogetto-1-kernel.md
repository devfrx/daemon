# Spec — Sotto-progetto 1: implementazione del kernel + simulatore DST

- **Data:** 2026-08-06
- **Sotto-progetto:** 1. Dipende da 0, 0b, 0c ([roadmap](../../roadmap.md))
- **Stato:** §0–§8 approvate. ⚠️ **Riaperta il 2026-08-07 su sette voci** trovate
  rileggendo la tracciabilità — elenco, ordine e propedeuticità in
  [HANDOFF](../../HANDOFF.md#prima-cosa-da-fare). Chiuse: **F3** (§2.8, ADR-0034), **F6**
  (§5.1), **F5** (§2.3.1), **F1a** — la dichiarazione della porta verso i worker (§2.3.1,
  [ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md)) — e **F2 con F7**,
  l'evoluzione del formato durevole (§4.9,
  [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md)). Restano, in
  ordine: **F1b** (progetto della porta, §5–§6), poi **F4**; poi la §8, una volta sola; poi
  il piano.

Questa spec **non ri-decide l'architettura**: la spec del kernel dice *cosa* il sistema
fa e *perché*, e gli ADR dicono con quali alternative scartate. Qui si dice *quali crate
esistono, quali tratti hanno quali firme, quale controllo prova quale vincolo, e in che
ordine si costruisce*.

Dove mi trovassi a rimettere in discussione una scelta già in un ADR, la strada è
sbagliata — con l'eccezione delle tre decisioni della §0.5, dove qualcosa manca davvero.

## Avanzamento delle sezioni

| § | Sezione | Stato |
|---|---|---|
| 0 | Perimetro e criterio di scaglionamento | **Approvata** |
| 1 | Struttura delle crate e regole di importazione | **Approvata** |
| 2 | Il substrato iniettabile | **Approvata** · ⚠️ **§2.8 aggiunta e §2.3 corretta il 2026-08-07** |
| 3 | Il simulatore DST | **Approvata** · ⚠️ **§3.1 allineata il 2026-08-07** |
| 4 | Giornale, riconciliazione e motore di persistenza | **Approvata** · ⚠️ **§4.9 aggiunta e §4.1 corretta il 2026-08-07** |
| 5 | Arbitro GPU, e la lacuna su I2 | **Approvata** |
| 6 | Gli altri meccanismi: gateway, sensori, permessi, degrado | **Approvata** |
| 7 | La porta di qualità: i controlli automatici | **Approvata** |
| 8 | Copertura V1–V37 e Q1–Q24 | **Approvata** |

---

## 0. Perimetro e criterio di scaglionamento

### 0.1 Cosa costruisce

| Artefatto | |
|---|---|
| **il kernel** | crate proprie, secondo i quattro vincoli di [ADR-0026](../../adr/0026-linguaggio-del-core.md) |
| **il simulatore DST** | l'infrastruttura che rende Q2, Q4 e Q5 verificabili invece che dichiarate |
| **sette ADR** | vedi §0.5 — erano tre quando questa riga fu scritta; **ricontato sulla tabella il 2026-08-08** |
| **la porta di qualità** | controlli automatici, ciascuno **provato in negativo** |

Il punto di partenza è [`spikes/rust/`](../../../spikes/rust/): confine dei tipi,
esecutore deterministico su `Future` native, giornale write-ahead iniettabile, tutti con
i loro test. Sono **prove promosse a fondamenta**, non codice da riscrivere.

### 0.2 Cosa NON costruisce

Il perimetro negativo è l'artefatto più prezioso di questa sezione, come lo era nella
[§0.2 della spec del kernel](2026-08-06-kernel-design.md#02-cosa-il-kernel-non-fa).

| Non costruisce | Perché |
|---|---|
| nessuna capacità L2 | [ADR-0001](../../adr/0001-architettura-a-kernel-con-capacita-paritarie.md): prima il kernel deve esistere |
| nessuna interfaccia grafica | è il sotto-progetto 2, e [ADR-0029](../../adr/0029-guscio-della-gui.md) è ancora `Proposed` |
| nessun adattatore verso provider reali | serve una chiave, una rete e SP-4; e [ADR-0020](../../adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md) garantisce che il kernel sia testabile **senza** chiamare un modello |
| nessun worker Python | [ADR-0028](../../adr/0028-ecosistema-dei-worker-ml.md) ne fissa il linguaggio, non ne chiede l'esistenza ora |
| nessuno spike chiuso | SP-1, SP-2, SP-3 e SP-4 richiedono modelli e GPU reali |

### 0.3 Il criterio di scaglionamento

Il sotto-progetto 1 copre L0 + L1, cioè **tutte** le §0–§10 della spec del kernel. È
molto, e comprende il pezzo che [ADR-0025](../../adr/0025-confinamento-a-livelli.md)
chiama «il più costoso del modulo di piattaforma».

Il criterio **non è «meno lavoro»**: sarebbe una scorciatoia travestita da piano. È
questo, e ha tre regole in ordine di precedenza:

| # | Regola | Esito |
|---|---|---|
| **A** | senza di esso **la DST non prova niente** | entra |
| **B** | è una proprietà **non retrofittabile** — costruirla dopo è una riscrittura, non una patch | entra |
| **C** | ha un **consumatore solo quando esiste una capacità L2** | si scaglia |

A e B vincono su C: se un pezzo ricade sia in B che in C, entra.

**Perché è falsificabile e non un'opinione.** Ogni riga della §0.4 va giustificata
nominando quale regola si applica. Un pezzo scaglionato senza una riga C esplicita è un
errore di questa sezione, non una semplificazione.

### 0.4 Ripartizione, sezione per sezione della spec del kernel

| § del kernel | Entra | Si scaglia | Regola |
|---|---|---|---|
| **§1** architettura di processo | core, ciclo di vita dei worker, **IPC lato core** | il processo `gui` | **A** — Q3 e Q4 sono DST, e senza il confine di processo non c'è nulla da simulare. Lo schema IPC è definito lato core ([ADR-0029](../../adr/0029-guscio-della-gui.md)), quindi non attende il guscio |
| **§2** arbitro GPU | tutta: ammissione, corsie, ciclo della concessione, revoca, due policy | la taratura dei profili reali (SP-1, SP-2) | **A** — Q2 e I2 sono il cuore della DST. I valori dei profili sono parametri, non impianto |
| **§3** gateway | il **decisore**: risoluzione del routing, filtro dei vincoli, catena, contabilità, record risolto | gli adattatori dei provider reali | **A** per il decisore (Q13 è una proprietà su *qualunque* catena) · **C** per gli adattatori |
| **§4** persistenza e run durevoli | giornale write-ahead, riconciliazione, classi di effetto, confini di autonomia, modello dello stato durevole | la **ricomposizione della proiezione** | **A** — Q5 è crash-injection ai confini di persistenza, ed è ciò che giustifica il simulatore · **C** per la ricomposizione: non ha consumatore finché nessuno chiama un modello |
| **§5** harness | il **contratto** del sensore e l'anello di verifica, con sensore finto; e la **dichiarazione delle sorgenti dell'anello 3**, con la porta da cui entrano | i sensori reali, il registro delle guide, l'**anello 4**, e il **registro dei trigger** dell'anello 3 | **A** — Q10 si verifica con un doppio · **B** — le sorgenti dell'anello 3 vanno dichiarate ora, §0.4.3 · **C** — RK-5 dice di rivedere il contratto **dopo** il secondo sensore reale; l'anello 4 legge ricorrenze che esistono solo quando qualcosa gira, e il registro dei trigger non ha consumatore finché nessuna capacità parte da un evento |
| **§6** permessi e confine dei dati | il **confine dei tipi**; la forma del permesso e la sua registrazione nel giornale | il mediatore completo, i preset, il ciclo di approvazione MCP, il canary | **B** — il confine dei tipi è la proprietà n. 1 non aggiungibile dopo (I6, V19, V20) · **C** — un mediatore non ha nulla da mediare finché non esistono strumenti |
| **§7** errori e degrado | tassonomia, stato di degrado osservabile, ritenzione a livelli | la proiezione trace e l'esportazione OTLP | **A** — Q18 è DST: lo stato va dichiarato *prima* del primo fallimento · **C** — l'esportazione ha per consumatore un backend esterno, ed è opt-in |
| **§8** test | tutta: è metà di questo sotto-progetto | — | **A** — è il simulatore |
| **§9** rischi e spike | ⬜ **nessun meccanismo**: non è una sezione di impianto, è il **piano degli spike**. SP-1…SP-4 sono collocati in §0.2, e SP-2 è l'innesco di Q1 in §8.2.1 | — | ⛔ **fuori scala A/B/C, e dichiarato invece che omesso** — vedi sotto |
| **§10** L0 fisico | il **motore di persistenza**; il *lato kernel* di segreti, confinamento e checkpoint: **dichiarare, richiedere, giornalare** | le implementazioni di piattaforma: livello 2 su Windows, cifratura reale, checkpoint su filesystem reale · **backup e ripristino** | **A** per il motore (senza, il giornale non esiste) · **B** per il lato kernel: V34, V35 e V37 costano poco ora e sono strutturali · **C** per la piattaforma — il livello 2 non ha nulla da confinare finché nessuna capacità esegue codice · **C** per il backup — §0.4.1 |

> ⚠️ **La riga §9 è stata aggiunta il 2026-08-08, ed è il quarto caso.** La tabella dichiara
> di coprire *«tutte le §0–§10 della spec del kernel»* e ne saltava una: la §9 non compariva
> **né dentro né fuori**, che è precisamente il terzo stato che §0.4.1, §0.4.2 e §0.4.3 hanno
> già corretto — il backup, la configurazione, l'anello 3. §0.4.3 chiedeva che *«chi troverà
> il quarto caso lo aggiunga»*: eccolo.
>
> ⛔ **Ma la risposta è diversa dalle altre tre, e va detto perché.** Quelle erano pezzi di
> impianto sfuggiti alla classificazione, e sono finiti in «si scaglia» con una regola C. La
> §9 **non è impianto**: è il piano degli spike, cioè un documento su come si decide, non una
> cosa da costruire. Non ha una regola A/B/C perché la scala classifica **meccanismi**, e qui
> non ce n'è uno. La correzione è **dichiararlo**, non collocarlo: un'assenza motivata è
> verificabile, un'assenza muta no.

#### 0.4.1 Il backup mancava da questa tabella, ed è una correzione

> ⚠️ **Aggiunto il 2026-08-07**, trovato scrivendo la §8. La riga §10 non nominava il
> backup **né fra ciò che entra né fra ciò che si scaglia**. Per la §0.3 quello non è
> un'omissione veniale: *«un pezzo scaglionato senza una riga C esplicita è un errore di
> questa sezione, non una semplificazione»*.

**Perché è regola C e non B.** La domanda che separa le due è se costruirlo dopo sia una
riscrittura. Non lo è: un backup **legge** archivi che esistono già e vi aggiunge un
manifesto. Il suo consumatore — un ripristino su macchina nuova — non esiste finché non
esiste qualcosa di irriproducibile da ripristinare.

**E c'è una metà di ADR-0022 che invece è già rispettata qui, non scaglionata.** Il *layout
per natura* — archivi separati con ritenzione, cifratura e backup differenziati — è
strutturale, e sarebbe stato regola B se questo sotto-progetto costruisse un archivio unico
indifferenziato. Non lo fa: giornale e segreti nascono **già come due archivi distinti**,
per V34 e per la struttura delle crate (§1.2). Ciò che si scaglia è il backup, non il
layout.

⚠️ **Il costo dello scaglionamento, che va detto:** fra qui e il sotto-progetto che lo
implementa, l'unico irriproducibile che cresce è il giornale, e l'unica protezione è una
copia manuale del suo file. È un rischio accettato, non un rischio ignorato.

**La riga §10 è quella su cui voglio più attenzione in revisione**, perché è dove il
criterio è più affilato: il kernel *richiede* un livello di confinamento dal primo
giorno — quindi V35 non è rimandabile — ma *chi lo implementa* è la piattaforma, e la
piattaforma non serve finché nessuno esegue nulla. Se questa distinzione non regge,
salta lo scaglionamento più costoso di tutti.

#### 0.4.2 La configurazione mancava da questa tabella, ed è la stessa correzione

> ⚠️ **Aggiunto il 2026-08-07** con [ADR-0034](../../adr/0034-parametri-di-decisione-consegnati-non-letti.md).
> È lo stesso difetto di §0.4.1, trovato subito dopo averlo diagnosticato altrove: la §2.8
> scaglia l'archivio della configurazione e il pannello che lo modifica, ma li scaglia
> **dentro la §2**, e la §8.5.1 ha stabilito che *«vince la §0.4: è l'autorità sul
> perimetro»*. Una riga C dichiarata fuori da qui è una riga C che nessuno trova.

| Entra | Si scaglia | Regola |
|---|---|---|
| il **tipo** dei parametri risolti, e il fatto che il kernel li **riceve** invece di leggerli | l'**archivio** su disco con il suo formato, e il **pannello** che lo modifica | **B** per ciò che entra — consegnarli dopo cambia la firma di ogni decisione che ne legge uno, ed è invisibile a ogni controllo finché non si prova a variarli · **C** per ciò che si scaglia: non c'è chi li cambi finché non esiste un'interfaccia |

**Perché non è regola A.** Senza, la DST prova ancora Q2, Q4 e Q5 a parametri fissi: non
è che «non prova niente». **E perché non è C per intero:** l'arbitro ha bisogno di un
budget *in questo sotto-progetto*, non quando arriverà una capacità L2 — che è esattamente
l'errore di innesco che la §8.3 riga V3 aveva commesso.

#### 0.4.3 L'anello 3 mancava da questa tabella, ed è la terza volta

> ⚠️ **Aggiunto il 2026-08-08.** È la voce **F4** della riapertura, e l'ultima delle sette.
> La riga §5 faceva entrare «il contratto del sensore e l'anello di verifica» e scaglionava
> «i sensori reali, il registro delle guide, l'anello 4». **L'anello 3 — i trigger — non
> compariva né dentro né fuori.** Per la §0.3 quello non è un'omissione veniale: *«un pezzo
> scaglionato senza una riga C esplicita è un errore di questa sezione»*. È lo stesso
> difetto di §0.4.1 e §0.4.2, e **le tre volte hanno la stessa forma**: un pezzo che non è
> né entrato né scaglionato non è invisibile per caso — è invisibile perché la tabella
> chiede «cosa entra» e «cosa si scaglia», e chi la compila non si accorge del terzo stato.

**Cos'è l'anello 3.** [design/04](../../design/04-anelli-e-sensori.md): non è una fase, è
**l'insieme dei modi in cui l'anello 1 può partire** — utente, pianificazione, cambiamento
di file, fine di un'altra run. *«Senza di esso il sistema funziona solo quando qualcuno lo
guarda.»*

**Non è una riga sola: si spacca in due, e le due hanno regole diverse.** È la stessa
forma di §0.4.2.

| Pezzo | Regola | Perché |
|---|---|---|
| il **registro dei trigger**, e l'apertura di una run da un evento | **C** | non ha consumatore finché non esiste una capacità L2 che parta da un evento. E la DST prova Q2, Q4 e Q5 aprendo le run direttamente: senza il registro **non è che non prova niente** |
| che ogni **sorgente di eventi** entri da una porta **dichiarata**, e che si dica **quale** | **B** | costruirlo dopo non è una patch: è una porta aggiunta dopo la campagna |

**Perché la seconda riga è B, e non è pedanteria.** La §3.1 dichiara che le porte del
simulatore *«sono **esattamente** le porte della §2.3, e non esistono altri punti in cui il
mondo tocchi il kernel»*, e il simulatore le sostituisce **tutte**. Una sorgente di eventi
scoperta dopo significherebbe che **C1 era verificato su un mondo più piccolo del reale**,
e **nulla sarebbe diventato rosso**. È il ragionamento di F1a per intero, e il gotcha #17.

**Le due sorgenti che la tracciabilità aveva già promesso**, e da dove entrano:

| Sorgente | Porta | Stato |
|---|---|---|
| **pianificazione** — riga *Scheduling* | `reactor` | ✅ **già coperta**: una scadenza è ciò che la §3.2 modella già, e in simulazione è il seme a decidere quando scatta |
| **cambiamento di file** — riga *File watching* | `reactor` | ⬜ **dichiarata qui, implementazione scaglionata** — la stessa postura di `network` in §2.3 |
| **fine di un'altra run** | nessuna: è **interna** | non tocca il mondo. Lo sa il giornale, che è già in perimetro |
| **utente** | `ipc` | già lì |

**Perché il cambiamento di file sta su `reactor` e non su `filesystem`.** Ciò che deve
essere deterministico non è *quale* percorso, ma **quando arriva la notifica** — ed è
esattamente il contratto del reattore: «cosa è pronto», reale che *«attende gli eventi
dell'OS»*, finto in cui *«cosa è pronto lo decide il seme»* (§3.1). Il percorso è un
argomento della registrazione, come lo è una scadenza; non è una famiglia nuova. Su
`filesystem` sarebbe invece una **direzione nuova** — quella porta modella l'albero, non la
spinta — e la sua finta dovrebbe generare eventi, cioè più macchina per meno determinismo.

> ✅ **Le famiglie di porte restano sei.** L'anello 3 **non** ne aggiunge una, ed è la
> ragione per cui questa voce costa una sezione invece di una riscrittura. Se fosse servita
> una settima famiglia, sarebbe stata regola B per intero e avrebbe dovuto entrare adesso,
> come `process` in §2.3.1.

⛔ **Il limite dichiarato.** Qui si dichiara **da dove** una sorgente entra, non **come**
funziona: il reattore reale non osserva percorsi finché l'anello 3 non si costruisce, e la
sua suite di conformità (§7.4.6) non copre un'operazione che nessuno chiama. Ciò che questa
sezione compra è che il giorno in cui si costruisce **non nasca una porta nuova**.

**Cosa produce per la §8: nessuna riga nuova.** **V29** — *«tempo, casualità, I/O e
scheduling iniettabili»* — copre già le sorgenti di eventi, e lo dice la sua stessa riga di
verifica: *«la campagna DST stessa, il cui criterio C1 fallisce a ogni sorgente nascosta»*.
Una sorgente dichiarata su una porta è dentro quella frase; una non dichiarata è ciò che
C1 fa fallire. **Lo stato di V29 non cambia.**

### 0.5 Le decisioni che questo sotto-progetto deve prendere

Non sono ri-derivazioni: sono buchi, ciascuno già documentato come tale.

| # | Decisione | Dove nasce | Sezione che la ospita |
|---|---|---|---|
| 1 | ✅ **La GPU usata dalla GUI non è arbitrata** — [ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md): quota di presentazione sottratta, concessione tenuta dal **core** | lacuna aperta, [HANDOFF](../../HANDOFF.md) e [roadmap](../../roadmap.md): I2 è verificato solo sui worker | §5 |
| 2 | ✅ **Motore di persistenza** — [ADR-0032](../../adr/0032-motore-di-persistenza.md): `redb`, con backend nostro | [§10.6](2026-08-06-kernel-design.md#106-cosa-resta-a-un-adr-successivo): la roadmap dice che blocca **l'implementazione**. Il discriminante era il requisito 4, **I/O iniettabile**: misurato, e solo `redb` lo espone | §4 |
| 3 | **Dove vive l'esecutore delle attività concorrenti** | conseguenza del vincolo 3 di ADR-0026: la crate del kernel è `#![no_std]`, e va **misurato** se un runtime deterministico di ecosistema può starci dentro o debba stare accanto | §2 |
| 4 | ✅ **Le dipendenze del kernel sono parte del confine I3** — [ADR-0031](../../adr/0031-dipendenze-del-kernel-parte-del-confine.md) | non previsto quando la §0 è stata approvata: emerge da una **misura**, registrata in §1.4.1. `no_std` impedisce di *nominare* `std`, non di *raggiungere* l'OS attraverso una dipendenza | §1 |
| 5 | ✅ **I parametri di decisione sono consegnati, non letti** — [ADR-0034](../../adr/0034-parametri-di-decisione-consegnati-non-letti.md) | **non previsto**: emerge dalla riapertura del 2026-08-07, rileggendo `tracciabilita.md` con la domanda del meccanismo. V29 rende sostituibile ciò che il mondo *risponde*, non i **parametri** con cui il kernel è configurato | §2.8 |
| 6 | ✅ **La porta verso i worker, e la lettura di «singolo» in I4** — [ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) | **non previsto**: è la voce **F1** della stessa riapertura. La §2.3 non aveva nessuna porta per *parlare* con un worker, e `design/01` la descriveva già con un verbo in più | §2.3.1 · §5–§6 |
| 7 | ✅ **L'evoluzione del formato durevole del giornale** — [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md) | **non previsto**: è la voce **F2** della stessa riapertura, con **F7** che vi converge. Il giornale è l'unico archivio irriproducibile, e chi lo rilegge è lo stesso programma mesi dopo, con campi in più: nessuna riga diceva cosa succede in quell'istante | §4.9 |

| 8 | ✅ **Il criterio del pari per il formato dei canali privati** — [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) | **non previsto**: emerge misurando ciò che la voce **F1b** chiedeva. M-1 aveva domandato se il grafo transitivo fosse accettabile — domanda giusta per I3, ma tutta sul **nostro** capo del filo. Un canale privato ne ha due, e il secondo non è Rust | §6.10 · §6.1.1 |

> ⚠️ **La riga 8 è stata aggiunta il 2026-08-08.** Mancava: ADR-0037 è una decisione presa
> **dentro** questo sotto-progetto, con lo stesso criterio di ammissione delle righe 4, 5, 6
> e 7 — non prevista quando la §0 fu approvata, ed emersa da una misura. La catena datata di
> §0.7 si ferma a sei perché è stata scritta prima; ora sono **sette** le decisioni ✅ e
> **otto** le righe, con la n. 3 ancora aperta.

Ciascuna nasce **dentro** la sezione che la richiede, non in coda: una decisione staccata
dal contesto che la motiva è la stessa cosa che ADR-0028 ha dovuto ratificare a
posteriori.

Sulla n. 3 vale la nota di metodo del piano degli spike: **si misura, non si assume.** Un
runtime che risulti incompatibile con `no_std` cambia la firma di ogni tratto del kernel;
uno compatibile renderebbe inutile la complicazione. In entrambi i casi la risposta si
scrive dopo la misura, e dove diverge dall'attesa si registra la divergenza.

### 0.6 I costi dichiarati dello scaglionamento

Un piano che elenca solo ciò che guadagna è incompleto.

| Costo | |
|---|---|
| **Il contratto del sensore resta un'ipotesi** | verificato con un doppio, su tre casi reali di cui **nessuno esiste**. È RK-5 per intero: va rivisto dopo il secondo sensore reale, e se non si adatta si spezza, non si piega |
| **Q6, Q11, Q12, Q16 e Q21 non sono verificati qui** | restano dichiarazioni fino al sotto-progetto che dà loro un consumatore. Vanno nella tabella §8 come *rimandati*, con il sotto-progetto che li chiude. ⚠️ **Q21 è stato spostato qui il 2026-08-07** — vedi sotto |
| **Q17, Q22, Q23 sono verificati solo lato kernel** | la parte di piattaforma resta **verificata ma non validata**: è RK-11, applicato alla §10 invece che al confine OS. Mitigazione di ADR-0002: schizzare *su carta* l'implementazione prima di congelare l'interfaccia |

> ⚠️ **Correzione, 2026-08-07: Q21 era nella riga sbagliata.** La riga «verificati solo
> lato kernel» valeva per **tre** voci, non quattro: Q17 poggia su `secrets`, Q22 sul
> checkpoint, Q23 sul livello di confinamento — e tutte e tre hanno un lato kernel elencato
> in §0.4. **Q21 non ne ha nessuno**, perché il backup non era in perimetro affatto: c'era
> entrato per somiglianza con gli altri tre della §10.
>
> Trovata dalla copertura della §8, che ha dovuto leggere questa sezione con un criterio
> diverso da quello con cui era stata scritta. La correzione è **doppia**, perché la causa
> era a monte: la §0.4 ora colloca il backup esplicitamente (§0.4.1), e Q21 passa alla riga
> dei rimandati. Registrata in §8.5.1 invece che cancellata.
| **Il gateway non parla con nessun provider vero** | il decisore è provato, l'integrazione no. La prima integrazione reale può scoprire che una firma è sbagliata |
| **«Rimandato» tende a diventare «dimenticato»** | mitigazione: la §8 elenca **ogni** V e **ogni** Q con il proprio stato, e `check-docs.sh` può essere esteso per controllarla — come già fa per V30 |

> ✅ **Rimando — la mitigazione è in esercizio (2026-08-07).** La §8 assegna uno stato a
> tutte e sessantuno le voci, e `check-docs.sh` **rifiuta** una tabella incompleta, uno
> stato fuori dall'insieme chiuso, o un `rimandato` senza innesco. Provato in due direzioni,
> §8.6.3. Non era una promessa in senso figurato: era un controllo da scrivere, e c'è.

### 0.7 Definizione di «fatto»

Il sotto-progetto 1 è chiuso quando **tutte** queste sono vere, non quando il codice gira.

| # | Condizione |
|---|---|
| 1 | ogni V in perimetro ha un controllo che gira in automatico |
| 2 | ogni controllo statico **è stato visto fallire** su una violazione deliberata, e poi tornare verde — gotcha #14: un controllo mai visto fallire non è un controllo |
| 3 | ogni Q in perimetro è verificato col metodo che [design/08](../../design/08-strategia-di-test.md) gli assegna, non con un altro |
| 4 | ogni difetto trovato in simulazione conserva il proprio **seed** come caso di regressione permanente (V31) |
| 5 | i **sei** ADR della §0.5 sono scritti, ciascuno con le proprie `Negative (accettate)`. ⚠️ erano tre fino al 2026-08-07, poi quattro con [ADR-0034](../../adr/0034-parametri-di-decisione-consegnati-non-letti.md), cinque con [ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) e sei con [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md). La riga 3 resta l'unica decisione della §0.5 senza ADR — vive in §2.4 |
| 6 | `roadmap.md`, `tracciabilita.md`, lo stato degli spike e `HANDOFF.md` sono aggiornati **nello stesso passaggio** |
| 7 | `bash scripts/check-docs.sh` esce verde |

> 📌 **Rimando — dove si controllano le condizioni 1 e 3 (2026-08-07).** Sono le due che
> parlano di copertura, e il posto in cui si verificano è la tabella della **§8**: la
> colonna del meccanismo nomina il controllo di ogni V (condizione 1) e, per i Q, il metodo
> che `design/08` assegna loro (condizione 3). ⚠️ **Le due condizioni riguardano ciò che è
> «in perimetro»**, quindi si leggono sulle sole righe `verificato qui` e sulla metà
> verificata delle `parziale`: una riga `rimandato` non le viola, la §8 dice perché.

---

## 1. Struttura delle crate e regole di importazione

### 1.0 Convenzione di nomenclatura

| | |
|---|---|
| **Codice** | interamente in **inglese**: nomi di crate, moduli, tipi, funzioni, commenti nel sorgente |
| **Documentazione** | in **italiano** |
| **Riferimenti al codice dentro la documentazione** | in **inglese**, con il nome esatto del sorgente |

Il costo accettato: fra la parola di un ADR («l'arbitro») e il nome nel codice (`arbiter`)
c'è una traduzione, che va tenuta a mente leggendo. Il beneficio: il codice non stona con
un ecosistema che è interamente in inglese, e non nasce un dialetto misto — che è la
condizione peggiore delle due.

### 1.1 Perché la crate è l'unità che conta

In Rust una **crate** è ciò che il compilatore compila in un colpo solo. Qui non è una
scelta organizzativa: i divieti forti — `#![forbid(unsafe_code)]`, `#![no_std]` — si
applicano **a una crate intera**, mai a un file o a una cartella. È la nota strutturale
registrata in [`RISULTATI.md`](../../../spikes/RISULTATI.md) dopo T6, ed è il vincolo 1 di
[ADR-0026](../../adr/0026-linguaggio-del-core.md).

Conseguenza: decidere quali crate esistono significa decidere **dove il compilatore ha il
potere di dire di no**. Ogni confine che non è di crate regge solo finché qualcuno si
ricorda di rispettarlo.

### 1.2 Le crate

| Crate | Libreria standard | Possiede | Cosa il compilatore le vieta |
|---|---|---|---|
| **`kernel`** | **no** — `no_std` + `alloc` | tutta la logica: arbitro, giornale, decisioni di routing, confine dei tipi, macchine a stati, e **la decisione** di quale attività far avanzare | l'OS, l'orologio, `HashMap`, `unsafe` |
| **`platform`** | sì | le implementazioni **reali** dei tratti dichiarati dal kernel: filesystem, orologio, rete, processi, confinamento livello 2 | — |
| **`secrets`** | sì | l'**unico** punto che tocca il portachiavi dell'OS | — |
| **`simulator`** | **no** — `no_std` + `alloc` | le implementazioni **finte** degli stessi tratti: orologio virtuale, RNG seminato, I/O in memoria, guasti scelti dal seed | come `kernel` |
| **`daemon`** (binario) | sì | il cablaggio **di produzione**: monta `platform`, avvia l'esecutore, ospita il server IPC, e **produce i parametri risolti** che consegna al kernel (§2.8) | — |

```mermaid
flowchart BT
    K["kernel — no_std + alloc<br/>logica, decisioni, tratti dichiarati<br/>NON dipende da nessuno"]
    P["platform<br/>implementazioni reali"]
    S["secrets<br/>unico accesso al portachiavi"]
    SIM["simulator — no_std<br/>implementazioni finte"]
    D["daemon — binario<br/>cablaggio, esecutore, server IPC"]
    T["test e campagne DST"]

    P --> K
    S --> K
    SIM --> K
    D --> K
    D --> P
    D --> S
    T --> K
    T --> SIM

    classDef ker fill:#1d4ed8,stroke:#1e3a8a,color:#fff
    classDef ada fill:#0f766e,stroke:#134e4a,color:#fff
    class K ker
    class P,S,SIM ada
```

**`kernel` non dipende da nessuna crate del progetto.** È una riga del suo manifesto, e
rende I3 verificabile guardando quella riga invece di ispezionare il codice.

Cinque crate e non meno: `secrets` è separata da `platform` perché V34 chiede che «un solo
punto legge le credenziali» sia **verificabile staticamente**, e in Rust la granularità
verificabile è la crate. Dentro `platform` sarebbe una regola fra moduli, cioè una
convenzione.

### 1.3 Perché le dipendenze vanno in quel verso

Il kernel **dichiara ciò di cui ha bisogno** e qualcun altro lo fornisce. Non va mai a
prendersi niente da solo.

È questo che rende possibile il simulatore: sostituire il fornitore sostituisce la realtà
con una finta **senza che il kernel se ne accorga**. Col verso opposto, ogni sostituzione
sarebbe una modifica al kernel, e la DST smetterebbe di essere iniettabilità per diventare
riscrittura a ogni test.

Una conseguenza da isolare qui, perché la §2 vi costruisce sopra:

| Chi | Cosa possiede |
|---|---|
| **`kernel`** | la **decisione** di quale attività concorrente far avanzare — è ciò che ADR-0026 ha comprato con lo spareggio #1 |
| **`platform`** | l'**attesa** che qualcosa sia pronto, cioè la chiamata all'OS |

Separarle è ciò che permette al simulatore di essere deterministico senza reimplementare
la logica: rende istantanea l'attesa, mentre la decisione resta quella vera.

### 1.4 Le regole, e con quale forza sono imposte

La colonna che conta è l'ultima. Gotcha #13: **un lint non è il compilatore.**

| Regola | Da | Meccanismo | Forza |
|---|---|---|---|
| il kernel non **nomina** `std` | I3, V28 | `#![no_std]` | **compilatore** — `E0433`, misurato in entrambe le direzioni |
| il kernel non **raggiunge** l'OS attraverso una dipendenza | I3, V28, [ADR-0031](../../adr/0031-dipendenze-del-kernel-parte-del-confine.md) | allow-list sul grafo **transitivo** di `kernel` e `simulator` — §1.4.1; le voci sono in §6.1.1, il meccanismo in §7 | test — **`no_std` non lo copre**, misurato. §6.8.2 aggiunge un controllo strutturale più forte, ma **non sufficiente** |
| niente `unsafe` nel kernel | ADR-0026 | `#![forbid(unsafe_code)]` | **compilatore** — `E0453`, non scavalcabile per riga |
| niente `HashMap` nominato nel kernel e nel simulatore | V29 | conseguenza gratuita di `no_std`: `HashMap` vive in `std`, non in `alloc` | **compilatore**, a costo zero — ma vale la riga 2: una dipendenza può portarne uno |
| niente `HashMap` nelle altre crate | V29 | `clippy.toml` | ⚠️ **lint** — scavalcabile con una riga |
| il kernel non ha un percorso verso il gateway per proprio conto | V28 | grafo delle crate + driver | test |
| solo `secrets` tocca il portachiavi | V34 | grafo delle crate — ⚠️ **nessuno script lo misura oggi**: `gate-deps.sh` guarda i grafi di `kernel` e `simulator`, non quelli di `platform` e `secrets` (2026-08-27, AUD-026) | test — ⏳ **rimandato**, §8.3 |
| un solo punto di uscita verso la rete | V25 | allow-list delle crate autorizzate, **oggi vuota** | test |

> ✅ **Due rimandi dalla §7.4.4**, dove il catalogo dei controlli ha ridotto questa tabella
> invece di ampliarla:
>
> | Riga | Esito |
> |---|---|
> | «niente `HashMap` nelle altre crate», con forza di lint | ⛔ **tolta.** Non difende V29: in una corsa DST `platform` non gira affatto, perché il simulatore sostituisce *tutte* le porte. Un controllo che non protegge niente e scatta su codice legittimo è il gotcha #24 senza contropartita |
> | «il kernel non ha un percorso verso il gateway per proprio conto», con driver proprio | ✅ **corollario, niente driver.** `kernel` non dipende da nessuna crate del progetto: un percorso verso un adattatore comparirebbe nel grafo transitivo e farebbe già scattare il controllo della §7.3.1 |

#### 1.4.1 `no_std` non è tutto il confine — misurato

La riga 2 della tabella non c'era nella prima stesura di questa sezione. È stata aggiunta
dopo una misura che ha smentito un'attesa scritta prima, e la divergenza si registra
invece di allinearsi ad essa.

**Eseguito il 2026-08-06** · `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` ·
`madsim 0.2.34`.

| Sonda | Attesa scritta prima | Misurato |
|---|---|---|
| **controllo** — crate `no_std` che nomina `std::fs` | `E0433` | ✅ `E0433`. Il controllo è attivo, quindi le sonde seguenti non sono vacue |
| **A** — crate `no_std` che dipende da `madsim` | «fallisce: `madsim` richiede `std`» | ❌ **compila.** `Finished dev profile`, 55 crate nel grafo |
| **A3** — crate `no_std` **e** `forbid(unsafe_code)` che chiama una dipendenza la quale usa `std::fs` e `SystemTime` | — | ❌ **compila ed esegue**: legge un file dal disco e stampa l'orologio di sistema, senza mai nominare `std` |

**Causa.** `#![no_std]` toglie `std` dalla portata **dell'unità di compilazione che lo
dichiara**. Non è una proprietà transitiva del grafo: una dipendenza che usa `std`
compila normalmente, e il kernel può chiamarne le funzioni.

**Cosa questo cambia, e cosa no.**

| | |
|---|---|
| **ADR-0026 resta corretto** | afferma che una crate `no_std` rifiuta `std::fs` con `E0433`, e la misura lo riconferma |
| **Ciò che non era mai stato misurato** | che questo bastasse a garantire I3. Non basta |
| **Conseguenza** | `no_std` è **necessario e non sufficiente**. La lista delle dipendenze del kernel è l'altra metà del confine, e va governata da una regola propria, verificata e **provata in negativo** |

La regola è materia di ADR, non di questa sezione: vedi §0.5.

**Perché `simulator` è `no_std`.** Fuori dal kernel il divieto di `HashMap` è solo un lint,
e fuori dal kernel c'è il simulatore — cioè il posto in cui un ordine di iterazione non
riproducibile avvelenerebbe *ogni* traccia, manifestandosi come il gotcha #12: divergenza
inspiegabile che non compare in nessun elenco di «chiamate OS». `no_std` lo rende un errore
del compilatore anche lì.

Costo accettato: il simulatore non fa I/O per conto proprio. Leggere un archivio di seed da
disco appartiene al *runner* dei test, non al simulatore. La scelta è **condizionata a M-2**
(§1.5): se la misura dice che non regge, si torna indietro e lo si dichiara.

**Sull'ultima riga.** Una allow-list vuota passa sempre, quindi sarebbe un controllo vacuo.
Si prova in negativo mettendo deliberatamente una chiamata di rete in `daemon` e verificando
che si accenda. Aggiungere una crate alla allow-list resta un atto esplicito e rivedibile,
non uno scivolamento.

### 1.5 Tre misure prima di congelare questa sezione

Non si assumono. Se una va diversamente, il grafo cambia **prima** che vi si scriva sopra.

| # | Da misurare | Cosa cambia se va male |
|---|---|---|
| **M-1** ✅ | esiste un serializzatore usabile in `no_std` + `alloc`? | se no, lo schema IPC (I4) non può stare in `kernel` e serve una crate accanto: il grafo di §1.2 cambia. **Misurata in §6.8: esito A, il grafo non cambia** |
| **M-2** | ✅ `simulator` regge `no_std` mentre inietta guasti e registra tracce? | **sì** (§3.6). ⚠️ **Ripiego corretto il 2026-08-08:** diceva *«se no, resta appeso a un lint»*, ma il lint su `HashMap` fuori dal kernel è stato **tolto** (§7.4.4 punto 1, «non difende nulla»). Se M-2 fosse fallita, `simulator` sarebbe rimasto **senza nulla**, non degradato — e la misura è passata, quindi il punto è storico |
| **M-3** | ✅ le regole di allow-list si esprimono con la toolchain standard, e si provano in negativo? | **sì**, e con le sonde N1–N4 viste fallire e tornare verdi: §7.2.2 e §7.3.1, `-e normal,no-proc-macro`. ⚠️ **Stato allineato il 2026-08-08:** la riga era ancora scritta al futuro (*«se no, servono driver scritti a mano»*) mentre ADR-0031 la dichiara eseguita e chiusa, e la §1.4 già scrive sopra i suoi risultati |

### 1.6 I costi di questa struttura

| Costo | |
|---|---|
| **cinque crate invece di una** | più manifesti, compilazione più lenta, e spostare codice fra crate costa più che spostarlo fra moduli |
| **`no_std` è scomodo** | niente `HashMap`, niente `std::thread`, nessuna comodità della libreria standard: ogni cosa va sostituita o fatta passare da un tratto. Si paga a ogni riga di `kernel` e di `simulator`, non una volta sola |
| **`kernel` resta una crate grande** | non si spezza: i divieti forti sono per crate, e spezzare moltiplica i posti in cui dimenticare gli attributi. Il costo è che i confini interni sono tenuti dai moduli, cioè più deboli |
| **tre regole restano controllo esterno, non compilatore** | l'**allow-list** di ADR-0031, V25 e V34. Un controllo si cancella, `no_std` no. Detto invece che sperato. ⚠️ **Ricontato sulla colonna «Forza» il 2026-08-08:** diceva «due» e ometteva proprio l'allow-list, cioè quella che §1.4.1 chiama *«l'altra metà del confine»* I3 e che ADR-0031 dichiara **la più debole delle quattro** |

---

## 2. Il substrato iniettabile

È la sezione da cui dipende ogni firma delle successive. **Nessuna delle sue scelte poggia
su una previsione**: le tre misure che la sostengono sono in §2.6, con comandi e versioni.

### 2.0 Cosa vuol dire iniettabile

Il kernel non prende mai niente dal mondo: lo **chiede a un fornitore che gli viene
consegnato**. In produzione il fornitore è vero; in simulazione è finto e governato da un
seme. Il kernel non sa la differenza, ed è per questo che una simulazione può riprodurre un
difetto **a comando**.

V29 elenca quattro cose da consegnare: **tempo, casualità, I/O, scheduling**.

> ⚠️ **Sono quattro, e non bastano — aggiunto il 2026-08-07.** I quattro di V29 sono i
> punti in cui il **non determinismo** entra in una decisione. Ma una decisione dipende
> anche dai **parametri con cui il kernel è stato configurato**, che sono deterministici e
> che nessuna sezione consegnava. Il secondo asse è la **§2.8** e
> [ADR-0034](../../adr/0034-parametri-di-decisione-consegnati-non-letti.md).

### 2.1 Tempo — due concetti distinti

| Concetto | A cosa serve | Chi lo usa |
|---|---|---|
| **monotonic** — non torna mai indietro | scadenze, finestre di validità della concessione, tempi di grazia, timeout | **le decisioni** |
| **wall time** — che ora è nel mondo | Q14, timestamp nel giornale | **solo la registrazione** |

**Nessuna decisione del kernel dipende dal wall time.** L'orologio di sistema torna indietro
— NTP, ora legale, l'utente che lo cambia — e una run che morisse per questo sarebbe un
difetto irriproducibile, cioè esattamente la classe che il sotto-progetto esiste per
eliminare.

Sono **due tipi distinti**, non due funzioni sullo stesso tipo: scambiarli non compila, con
lo stesso meccanismo che separa `Instruction` da `Untrusted`.

### 2.2 Casualità — la porta esiste, i consumatori si contano

Ogni sorgente di casualità è un punto da cui una traccia può divergere. La porta esiste;
**l'elenco di chi la consuma si scrive**, e resta corto per scelta.

| Cosa | Approccio istintivo | Scelto |
|---|---|---|
| identità di run e passi | identificatori casuali | **progressivi, assegnati dal giornale** — deterministici per costruzione, e leggibili in un trace |
| attesa fra due ritentativi | backoff con jitter | **senza jitter**: serve contro la contesa fra molti client, e qui il client è uno |

**Elenco dei consumatori nel kernel: vuoto.** Dichiararlo vuoto è un'informazione; una porta
generica «tanto poi servirà» è il contrario. La casualità serve al `simulator` — per
scegliere l'ordine e iniettare guasti — non alla logica.

### 2.3 I/O — le famiglie di porte

| Famiglia | Progettata in |
|---|---|
| `journal` — scrittura durevole ordinata, rilettura | §4 |
| `filesystem` — ambiti di checkpoint, artefatti | §4 |
| `process` — **ciclo di vita di un worker: avvio, dialogo, uccisione** | §5 · [ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) |
| `ipc` — server verso la gui | §6 |
| `network` — **l'unico punto di uscita verso la rete** (V25) | dichiarata qui, implementazione scaglionata (§0.4) |
| `reactor` — «cosa è pronto», e l'attesa | §2.4 |

#### 2.3.1 Due celle riscritte — F1a e F5

> ⚠️ **Corretto il 2026-08-07**, dopo l'approvazione della §2. Sono le voci **F1** e **F5**
> della riapertura, e stanno nella stessa tabella: si toccano una volta sola. La decisione
> completa, con le quattro alternative e i costi, è
> [ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md).

**`process` — mancava il verbo di mezzo.** La riga diceva «avvio e uccisione»:
non esisteva nessuna porta per **parlare** con un worker già avviato.
[design/01](../../design/01-topologia-dei-processi.md) lo descriveva già con tre verbi —
*«Avvia, **istruisce**, uccide»* — e aggiungeva *«il flusso audio risale al core»*. Il
divario era fra questa tabella e quel diagramma, non fra il progetto e I4.

| | |
|---|---|
| **perché non poteva aspettare la §5** | la §3.1 dichiara questo elenco **esaustivo**, e il simulatore sostituisce *tutte* le porte. Una porta aggiunta dopo significa che **C1 era verificato su un mondo più piccolo del reale**, e nulla sarebbe diventato rosso — gotcha #17 |
| **perché resta una porta sola** | il dialogo entra in `process` invece di nascere accanto: l'oggetto con cui si parla a un worker è quello che restituisce l'avvio, e l'avvio pretende una concessione (§5.6). Spezzare avvio e dialogo su due porte riaprirebbe la chiusura che ha portato I2 al **compilatore** |
| **la lettura di I4** | «singolo» significa un trasporto e uno schema **per canale privato**. Nessun broker, nessuna negoziazione, nessun versionamento — e nessuno dei due canali ha consumatori esterni. ADR-0004 non è superato: riceve un **rimando** |

**`network` — la descrizione era più stretta della promessa.** La riga diceva «uscita verso
i provider»; V25 e Q20 promettono *«un solo punto di uscita **verso la rete**»*. Non è una
sfumatura: [ADR-0017](../../adr/0017-giornale-sorgente-trace-proiezione.md) ha già deciso
un **secondo** consumatore — l'esportazione OTLP opt-in — e con la descrizione precedente
sarebbe nato **fuori** dall'unico punto di uscita, cioè esattamente ciò che V25 vieta.

⛔ Cosa **non** cambia: la porta resta dichiarata e non implementata (§0.4), e la sua
allow-list resta vuota. Il buco di V25 dichiarato in §7.4.2 — sonda sì, contro-sonda no —
resta aperto e resta registrato lì.

### 2.4 Scheduling — dove vive l'esecutore

È la **decisione n. 3** della §0.5.

| | |
|---|---|
| **L'esecutore vive in `kernel`** | sceglie col seme quale attività far avanzare |
| **`platform` implementa la porta `Reactor`** | risponde a «cosa è pronto» e compie l'**attesa** vera sull'OS |
| **`simulator` implementa la stessa porta** | «cosa è pronto» e «quanto tempo passa» li decide il **seme**: è così che il tempo diventa virtuale (C3) |
| **Nessun thread nel percorso decisionale** | `platform` può usarne quanti vuole dietro le porte; la **sequenza delle decisioni** resta una per volta |

#### 2.4.1 La regola che rende possibile il resto

Un risvegliatore su misura — il biglietto «quando è pronto, chiama me» — **non è
costruibile dentro il kernel**: richiede `unsafe`, e `#![forbid(unsafe_code)]` lo rifiuta.
Misurato, §2.6.

Quindi l'esecutore deve sapere da sé chi può avanzare, e può saperlo a una condizione:

> **Un'attività del kernel si sospende solo su una primitiva dell'esecutore o su una porta.**

La prontezza ha due sorgenti, e nessuna richiede un risvegliatore:

| Sorgente | Chi la conosce |
|---|---|
| interna — code, canali, attese fra attività del kernel | l'**esecutore**, che le possiede |
| esterna — I/O, timer, IPC, worker | la porta **`Reactor`** |

La regola è **quasi auto-imposta**: `no_std` toglie dal kernel le primitive di attesa della
libreria standard, e [ADR-0031](../../adr/0031-dipendenze-del-kernel-parte-del-confine.md)
impedisce che rientrino da una dipendenza. Resta la disciplina di non aggirarla quando
sembrerà comodo.

#### 2.4.2 Perché una decisione per volta

Non è una rinuncia al parallelismo: il lavoro pesante sta nei **worker**, che sono processi
separati (ADR-0004), e le operazioni pesanti ma di sistema stanno dietro le porte di
`platform`, che può usare thread propri.

Ciò che si guadagna è la rimozione di una classe di difetti. ADR-0004 descrive l'arbitro
come «un unico processo con **un unico lock**» — ed è la primitiva che ha fatto fallire Go
in C6. Con una decisione per volta **quel lock non esiste**: il problema è rimosso, non
gestito.

E non è la posizione di TypeScript: là il thread singolo era l'**unico** modo di essere
deterministici; qui è una scelta revocabile, perché il linguaggio il parallelismo ce l'ha.
Lo spike lo ha misurato in entrambe le direzioni: `Future` sotto esecutore proprio → **1**
traccia su 100; `std::thread` → **più di 1**. Rust non ha vinto C6 perché i suoi thread
sono deterministici — **non lo sono** — ma perché le `Future` lasciano l'ordine a noi.

**Innesco che riaprirebbe la questione:** una *decisione* del kernel — non un I/O, non una
cifratura — misurata sopra il millisecondo, che tenga fermo l'anello. Anche allora la prima
risposta sarebbe spostare quel calcolo dietro una porta; il multithread nel percorso
decisionale sarebbe un ADR che supera ADR-0021, non una modifica.

#### 2.4.3 Il beneficio di ADR-0026 che non si incassa

ADR-0026 conta fra le conseguenze positive: «esiste un runtime deterministico di ecosistema
— `madsim` — quindi il simulatore non va scritto da zero». **Con questa scelta il beneficio
non si incassa**, e il conto è misurato:

| | Con `madsim` | Con l'esecutore nostro |
|---|---|---|
| crate nel grafo di `kernel` | **55**, fra cui `getrandom` e `rand` | **0** |
| ADR-0031 | la lista nasce con 55 voci da valutare | la lista nasce **vuota** |
| chi decide l'ordine | il runtime, sostituito a compilazione | **il kernel**, sempre, anche fuori dai test |
| codice da scrivere | poco | l'esecutore — nel prototipo misurato ~40 righe |

`getrandom` è la riga decisiva: una sorgente di casualità **seminata dall'OS** dentro il
kernel, cioè il gotcha #12 in una forma che nessun elenco di «chiamate OS» mostrerebbe.

### 2.5 Come sale `spikes/rust/`

| Nello spike | Dove va | Cosa cambia |
|---|---|---|
| `Instruction` / `Untrusted` | `kernel/src/boundary.rs` | sostanza invariata; la conversione diventa **giornalata** (V19) |
| `sched.rs` · `Rng` | porta `Rng` nel kernel + implementazione seminata in `simulator` | la guardia sullo zero (gotcha #10) sale con lei |
| `sched.rs` · `World` | **non sale** | era un esecutore giocattolo: il ruolo lo prende `simulator` |
| `concorrenza.rs` · `esegui_async` | `kernel/src/executor.rs` | è il nucleo, misurato in `no_std` |
| `concorrenza.rs` · `esegui_thread` | **resta nello spike** | non è codice: è l'evidenza che C6 non è vacuo |
| `giornale.rs` | porta `Journal` nel kernel + doppio cadente in `simulator` | §4 la estende |
| `kernel_core/` | assorbito da `kernel` | era la prova di T6 |
| `tests/compile_fail/` | sale e **cresce** | ogni regola nuova porta il suo test negativo |

### 2.6 Le misure che sostengono questa sezione

Eseguite il **2026-08-06** · `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` ·
`madsim 0.2.34` · Windows 11. Prototipi usa-e-getta, fuori dal repository.

#### M-4 — un runtime di ecosistema è utilizzabile sotto `no_std`?

**Sì.** L'ipotesi contraria, scritta prima della misura, era falsa. Esiti e conseguenze in
§1.4.1 e in ADR-0031. Il dato che decide non è la compatibilità ma il **grafo**: 55 crate,
fra cui `tokio`, `mio`, `socket2`, `windows-sys`, `getrandom`, `rand`.

#### M-5 — un esecutore `no_std` senza `unsafe` fa avanzare `Future` reali?

**Sì, con zero dipendenze.**

| Sonda | Esito |
|---|---|
| 100 esecuzioni, seed `20260806` | **1 sola traccia distinta** |
| seme diverso | traccia diversa |
| **non-vacuità** — l'interlacciamento è reale? | **13 cambi di task su 17 transizioni**, contro **2** della controprova sequenziale |
| dipendenze della crate | **0** |
| `unsafe { }` dentro la crate | ❌ `error: usage of an unsafe block` — `forbid` è attivo, non solo dichiarato |
| costruire un `Waker` su misura | ❌ `E0133: call to unsafe function Waker::from_raw is unsafe` |

L'ultima riga **forza** la regola §2.4.1: non è una preferenza di design.

La prima versione di questa sonda usava un controllo di non-vacuità sbagliato («task0 due
volte di fila»), che capita per caso una volta su tre. Registrato perché è il gotcha #14
applicato a sé stessi: è stato corretto contando i **cambi di task**.

#### M-7 — quanto costa una decisione dell'arbitro

Aggiunta per falsificare l'affermazione «sono microsecondi», che era un ragionamento sulla
struttura e non un numero. Arbitro con budget 16 GB meno 1 GB di quota audio **sottratta**,
quattro profili (`realtime` 512 · `interactive` 2048 · `batch` 6144 · `batch` 13312 come
TRELLIS2), corsie, coda e promozione al rilascio. `no_std`, `forbid(unsafe_code)`, zero
dipendenze, `BTreeMap`.

| Coda tenuta a | `request` p99 | `release` p99 — **release** | `release` p99 — **debug** |
|---|---|---|---|
| **2** — realistico per un desktop a utente singolo | ≤ 100 ns | **500 ns** | 2,2 µs |
| 100 | ≤ 100 ns | 3,1 µs | 45,6 µs |
| 500 | ≤ 100 ns | 16,6 µs | 313 µs |
| 2000 | ≤ 100 ns | 86,6 µs | non misurato |

Tradotto nel budget che conta — **Q1, voce sotto i 600 ms** — quante decisioni devono
accodarsi dietro l'anello per mangiarne l'1 % (6 ms):

| Coda | Decisioni in fila per 6 ms |
|---|---|
| 2 | ~12 000 |
| 100 | ~1 900 |
| 500 | ~360 |
| 2000 | ~70 |

**I limiti di questa misura, dichiarati:**

| # | Limite |
|---|---|
| 1 | `request` è **sotto la risoluzione del timer** di Windows (~100 ns): il dato dice «non più di 100 ns», non quanto esattamente |
| 2 | l'implementazione ordina l'intera coda a ogni rilascio — una versione reale la terrebbe ordinata per corsia. I numeri sono quindi un **limite pessimistico** |
| 3 | è misurato **solo l'arbitro**. L'affermazione «le decisioni del kernel costano microsecondi» è più larga di ciò che è stato misurato: giornale, routing e proiezione non lo sono |
| 4 | il `max` per operazione è dominato dal rumore dello scheduler di Windows; il dato è p50/p99 |
| 5 | il primo scenario scritto **è stato buttato**: lasciava crescere la coda a diecimila invece di tenerla al valore obiettivo, e i suoi scaglioni non erano confrontabili. Registrato invece che nascosto |

### 2.7 I costi di questa sezione

| Costo | |
|---|---|
| **l'esecutore lo scriviamo noi** | e va mantenuto. È il beneficio che ADR-0026 aveva contato, e ora si sa quanto vale non incassarlo: 0 crate contro 55 |
| **un'attività si sospende solo su una porta o su una primitiva dell'esecutore** | vincola come si scrive ogni pezzo del kernel. Chi ne inventasse una terza otterrebbe un'attività che non si risveglia più — un blocco che si scopre a runtime, non a compilazione |
| **una decisione per volta** | se una decisione diventasse pesante in CPU terrebbe fermo l'anello. Difesa strutturale: il pesante sta nei worker e dietro le porte. Innesco di riapertura in §2.4.2 |
| **due concetti di tempo** | più attrito a ogni «che ora è». Mitigato dal fatto che sono due tipi: scambiarli non compila |
| **la porta di rete esiste vuota** | dichiarata qui, riempita in un sotto-progetto successivo. Il rischio di dimenticarsene sta nella tabella di copertura §8 |

> 📌 **La §2.8 è stata aggiunta dopo, e porta i propri costi in §2.8.5.** Non sono
> ripetuti qui: questa tabella copre le scelte che la §2 aveva quando è stata approvata.

### 2.8 I parametri di decisione sono consegnati, non letti

> ⚠️ **Sezione aggiunta il 2026-08-07**, dopo l'approvazione della §2. Non corregge una
> scelta di questa sezione: ne aggiunge un **secondo asse** che nessuna sezione copriva.
> La decisione completa, con alternative e costi, è
> [ADR-0034](../../adr/0034-parametri-di-decisione-consegnati-non-letti.md).
>
> ⚠️ **A differenza delle altre scelte della §2, questa non poggia su una misura** — e
> l'apertura della sezione dice che le tre misure di §2.6 la sostengono. Non ne serve una:
> non c'è un discriminante fra due opzioni che un numero possa sciogliere. È una
> **constatazione di coerenza**, verificabile rileggendo gli **otto** — 0005, 0006, 0010,
> 0016, 0018, 0023, 0025, 0033; ⚠️ **ricontato su ADR-0034 il 2026-08-08**, diceva «cinque»,
> che è il numero delle decisioni *che ne leggono uno senza passare da un'interfaccia* e non
> degli ADR che ne nominano uno — seguire la frase alla lettera avrebbe saltato 0010, 0018 e
> 0025, cioè il modo di fallire che il costo di §2.8.5 dichiara — ADR che nominano un
> parametro. ADR-0034 lo dichiara, e dichiara anche cosa resta un giudizio: la stima che la
> correzione tardiva sarebbe *«pervasiva ma meccanica»*.

**A parole.** Il kernel non prende niente dal mondo: gli viene consegnato (§2.0). Ma una
sua decisione dipende da due cose diverse, e finora ne governavamo una sola: *cosa il
mondo risponde*, e *con quali parametri il kernel è stato configurato*. Il budget della
GPU, la quota audio, la quota di presentazione, la policy attiva, i tetti di autonomia:
nessuno di questi gli veniva consegnato, e ciò che non viene consegnato finisce per essere
una **costante scritta dentro il kernel**.

Una costante è la peggiore delle violazioni possibili qui, perché è **invisibile**: non
compare in nessun elenco, non fa scattare nessuna voce del catalogo §7, e si scopre solo
quando qualcuno prova a farla variare in una campagna e non può. È la forma esatta del
gotcha #12.

#### 2.8.1 Perché non è «un quinto iniettabile»

La distinzione va fatta, o la sezione si progetta male.

| | I quattro di V29 | I parametri |
|---|---|---|
| natura | sorgenti di **non determinismo** | valori **deterministici** |
| sostituirli compra | la **riproducibilità** | ⛔ non la riproducibilità |

Consegnare un parametro compra due cose diverse, ed entrambe contano:

| # | | Perché |
|---|---|---|
| 1 | **I3** | un parametro sta su disco; leggerlo è I/O, e il kernel non fa I/O. In linea di principio lo copriva già la porta `filesystem` — ma nessuna riga lo diceva, e l'esito naturale del silenzio è la costante |
| 2 | **variabilità sotto il seme** | la DST varia *ciò che il mondo risponde*; non può variare *come il sistema è configurato*. Un'intera classe di scenari è irraggiungibile |

#### 2.8.2 La regola

> **Nessuna decisione del kernel legge un parametro che non le è stato consegnato.**

| # | |
|---|---|
| 1 | il kernel riceve **alla costruzione** un valore che porta i parametri **risolti** |
| 2 | il kernel **non nomina** un file, una chiave o un default: nessuno dei tre è esprimibile al suo interno |
| 3 | chi produce il valore è **`daemon`**: dall'archivio via `platform` in produzione, dal banco di prova in simulazione |
| 4 | la **sostituzione** di un parametro è un passo giornalato — ✅ già preteso dalla §5.4 per la transizione di policy |

**Non è un'astrazione nuova.** È la mossa di
[ADR-0011](../../adr/0011-routing-risolto-e-giornalato-per-richiesta.md) un livello sopra:
*«il record contiene la decisione risolta, non un riferimento alla configurazione»*. Là il
giornale non vi rimanda, qui il kernel.

**Una conseguenza gratuita:** se il valore consegnato porta **una** policy, «due policy
attive» non è rappresentabile. L'unicità che §5.4 verificava con un test a esempi sale al
compilatore — catalogo §7.4.1, blocco C.

#### 2.8.3 Cosa entra e cosa si scaglia

| | Regola | |
|---|---|---|
| il **tipo** dei parametri risolti, e che il kernel li riceve | **B** | consegnarli dopo cambia la firma di ogni decisione che ne legge uno |
| che il kernel non nomini file, chiave o default | **B** | idem |
| che la sostituzione sia un passo giornalato | — | ✅ già in perimetro per la policy (§5.4) |
| l'**archivio** su disco e il suo formato | **C** | esiste un'interfaccia (2) |
| il **pannello** che li modifica | **C** | idem |
| i **valori** reali | — | SP-1, SP-2, M5: parametri, non impianto |

**Non è A** — senza, la DST prova ancora Q2, Q4 e Q5 a parametri fissi. **Non è C** —
l'arbitro ha bisogno di un budget qui, non quando arriverà una capacità L2.

⚠️ **In questo sotto-progetto i valori di default sono letterali dentro `daemon`.** È il
confine corretto e non una scorciatoia — l'archivio arriva con l'interfaccia — ma va
scritto invece che nascosto.

⛔ **Perimetro negativo.** Non è un sistema di configurazione, non è un registro a chiavi
stringa — che rimetterebbe il kernel in condizione di *chiedere* — e non è una
sostituzione a caldo generalizzata. L'elenco completo è in ADR-0034.

#### 2.8.4 Come si verifica, e il limite dichiarato

| # | Livello | Sonda — *deve scattare* | Contro-sonda — *deve restare verde* |
|---|---|---|---|
| 1 | **1 — compilatore** | costruire l'arbitro **senza** consegnargli i parametri → non compila | consegnandoglieli → compila |
| 2 | **2 — campagna DST** | si fa variare un parametro col seme e le tracce **non cambiano** → c'è una costante nascosta | parametro variato, tracce diverse → verde |

> ⛔ **Il limite, dichiarato prima che qualcuno lo scopra.** Il compilatore **non può**
> vietare una costante scritta dentro il kernel. Il controllo 1 prova che una decisione
> **riceve** i propri parametri, non che non ne abbia altri di nascosto: è il limite del
> gettone (§6.3.2), *prova la provenienza, non l'esclusività*. Il controllo 2 copre quel
> buco solo per i parametri che la campagna fa **davvero** variare. **Non è una prova di
> assenza.**

#### 2.8.5 I costi

| Costo | |
|---|---|
| **attrito su ogni firma che legge un parametro** | si paga a ogni riga, non una volta. Stesso genere di costo che ADR-0021 dichiara per V29 |
| **i default vivono in `daemon` come letterali** | finché non esiste l'archivio. Dichiarato, non nascosto |
| **un tipo in più da tenere allineato** | ogni ADR che introduce un parametro deve comparirvi, o quel parametro rientra come costante da un'altra porta |
| **il compilatore non può vietare una costante** | §2.8.4. Sposta il confine, non lo elimina |
| ⚠️ **il rischio è la crescita, non la pigrizia** | il perimetro negativo di §2.8.3 è la parte da non togliere |

---

## 3. Il simulatore DST

### 3.0 Cosa fa, a parole

Il simulatore prende il posto del mondo. Al kernel vengono consegnate le stesse porte di
sempre, ma dietro non c'è il sistema operativo: c'è un oggetto che **decide col seme** cosa
succede — chi va avanti per primo, quanto tempo passa, quando qualcosa si rompe.

Il kernel non se ne accorge. È questo che permette due cose che senza non esistono:

| | |
|---|---|
| **riprodurre** | stesso seme, stessa identica esecuzione, difetto compreso |
| **accelerare** | un'attesa di cinque secondi non si aspetta: si fa scorrere l'orologio |

### 3.1 Cosa sostituisce

Non è un elenco arbitrario: **sono esattamente le porte della §2.3**, e non esistono altri
punti in cui il mondo tocchi il kernel.

| Porta | Reale (`platform`) | Finta (`simulator`) |
|---|---|---|
| `reactor` | attende gli eventi dell'OS | fa scorrere l'orologio virtuale e sceglie i pronti col seme |
| `journal` | scrive sul motore di persistenza | scrive in memoria, e **cade** a una scrittura scelta dal seme |
| `filesystem` | disco | albero in memoria |
| `process` | avvia, **istruisce** e uccide worker veri | worker finti: risposte, ritardi e uccisione scelti dal seme |
| `network` | HTTPS **verso la rete** — i provider **e** l'esportazione OTLP opt-in | risposte, ritardi e perdite scelti dal seme |
| `ipc` | named pipe verso la gui | client finto, che può **morire** quando il seme decide |
| `rng` | seminato all'avvio | seminato dal seme della campagna |

> ⚠️ **La riga `process` è stata allargata il 2026-08-07**, con la §2.3 e nello stesso
> passaggio: questa tabella dichiara di essere *«esattamente le porte della §2.3»*, quindi
> non poteva restare indietro di un verbo. Il dialogo con un worker è ora una sorgente di
> guasti che il seme governa — risposta sbagliata, tardiva o assente — e non solo
> l'uccisione. **Quali guasti verifichino quale Q è progetto, non dichiarazione:** la §3.3
> si tocca in F1b, con la porta. [ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md).
>
> 📌 Nota di lettura, senza conseguenze: `rng` è dichiarata in **§2.2**, non in §2.3. La
> frase qui sopra resta vera in ciò che afferma — non esistono altri punti in cui il mondo
> tocchi il kernel — ma l'elenco è di **sette** porte e la §2.3 ne enumera sei.

### 3.2 Il tempo virtuale

Regola: **l'orologio avanza solo quando nessuno può lavorare.** Finché esiste un'attività
pronta, il tempo è fermo; quando nessuna lo è, il reattore porta l'orologio alla **prima
scadenza futura**.

#### 3.2.1 La trappola, trovata sbattendoci contro

`advance()` deve considerare **solo le scadenze future**, e deve poter rispondere «non c'è
niente da avanzare».

La prima stesura prendeva il minimo di *tutte* le scadenze registrate. Le voci dei task già
conclusi restano nella mappa con un istante ormai passato: il minimo cadeva lì, l'orologio
non si muoveva, e la funzione dichiarava comunque di aver avanzato. L'esecutore girava a
vuoto per sempre.

Due conseguenze, entrambe adottate:

| | |
|---|---|
| `advance()` filtra le scadenze **strettamente future** e restituisce `false` se non ce ne sono | un avanzamento nullo non deve mai essere dichiarato riuscito |
| l'esecutore ha una **guardia sul numero di giri** | un blocco deve manifestarsi come errore, non come attesa infinita. Un test che non finisce non dice nulla |

#### 3.2.2 Perché qui si ottiene ciò che `synctest` prometteva a metà

Il tempo avanza solo a quiescenza — è la stessa nozione che la documentazione di Go
dichiara per `synctest`. La differenza è che qui l'esecutore **sceglie anche l'ordine** fra
i pronti, col seme. Quiescenza *e* ordine totale, invece di quiescenza soltanto: è
esattamente lo scarto misurato che ha chiuso ADR-0026.

### 3.3 L'iniezione dei guasti

**Si inietta un guasto dove c'è una porta**, e non ci sono altri posti. Ogni riga serve un
requisito preciso:

| Guasto | Porta | Verifica |
|---|---|---|
| caduta fra intento ed esito | `journal` | **Q5** — ripresa senza effetti rieseguiti |
| uccisione di un worker in un istante qualsiasi | `process` | **Q4** · I1 · I5 |
| **risposta assente o tardiva** | `process` | **Q4** — il core non si blocca: l'attesa passa dal `reactor`, e la guardia sui giri dell'esecutore (§3.2.1) trasforma un blocco in errore invece che in attesa infinita |
| **frame malformato** — byte consumati diversi dalla lunghezza dichiarata | `process` | **Q4** — il frame è **rifiutato**, non diventa un valore. Senza questa riga varrebbe il gotcha #34: decodifica riuscita e valore sbagliato |
| **frame non sollecitato** — il worker parla senza ricevuta aperta | `process` | **Q4** · **I5** — è un guasto, non un dato: §6.10.1 |
| **morte del worker a metà flusso** | `process` | **Q4** · I1 — la ricevuta si chiude e la concessione torna alla linea di base |
| morte della gui a metà run | `ipc` | **Q3** · **I1** — la GUI non possiede stato autorevole, quindi ucciderla non perde nulla: è metà esatta della riga di verifica di I1, e l'altra metà è la riga del kill del worker |
| perdita della rete | `network` | **Q18** — il degrado si dichiara *prima* del primo fallimento |
| interlacciamento delle richieste concorrenti | `reactor` | **Q2** — la somma delle concessioni non supera mai il budget · I2 |
| caduta durante la conservazione di un file | `filesystem` | **Q22** — l'ambito torna byte-identico |

> ⚠️ **Quattro righe del dialogo aggiunte il 2026-08-08**, con F1b e la §6.10. La §3.1
> aveva già allargato la riga `process` — «risposta sbagliata, tardiva o assente» — ma la
> mappa **guasto → requisito** non era scritta, e senza di essa il seme governa guasti che
> nessuno ha collegato a un requisito.
>
> 📌 **Tutte e quattro ricadono su Q4**, e non è una scorciatoia: è la regola di §8.2.2 —
> *«un Q della DST eredita lo stato della porta in cui si inietta»* — applicata alla porta
> `process`. **Lo stato di Q4 non cambia** per questo: resta `parziale` finché non esiste
> un worker vero contro cui provare la conformità della finta (§7.4.6).

### 3.4 Il seme come caso di regressione — e cosa **non** è

V31 dice che ogni difetto trovato in simulazione conserva il proprio seme. Adottato: il
seme entra in un elenco versionato, insieme a *cosa* ha trovato.

**Ma un seme non è un oracolo permanente, e va detto adesso.** Un seme riproduce
un'esecuzione **solo finché il codice non cambia**: modificato il kernel, lo stesso seme
esplora un cammino diverso. Quindi:

| Cosa è permanente | Cosa non lo è |
|---|---|
| la **proprietà** verificata — «la somma delle concessioni non supera mai il budget» | il **cammino** che quella volta la violò |
| il valore del seme come punto di ripartenza per indagare *oggi* | la garanzia che domani ritrovi lo stesso difetto |

Il seme serve a **debuggare**; è la proprietà a proteggere. Un elenco di semi presentato
come suite di regressione sarebbe una falsa sicurezza — la stessa classe di errore di
«cifrato a riposo» dichiarato più forte di quanto sia.

### 3.5 La campagna

| Quando | Cosa gira |
|---|---|
| a ogni commit | campagna breve: N semi sugli scenari principali |
| su ciclo lungo | campagna profonda: molti più semi, scenari più grandi |
| su difetto trovato | il seme entra nell'elenco e la **proprietà** entra nella suite |

La misura M-2 sposta questa riga in meglio rispetto a quanto
[design/08](../../design/08-strategia-di-test.md) assumeva («DST su cicli più lunghi»): una
corsa completa dello scenario minimo costa **25,8 µs**, quindi migliaia di semi stanno
dentro un secondo. La DST può girare **a ogni commit**, e i cicli lunghi servono ad andare
più a fondo, non a renderla possibile.

Con una riserva: 25,8 µs è lo scenario **minimo**. Gli scenari reali saranno più pesanti.
La misura dice che il substrato non è il collo di bottiglia — non che le campagne siano
gratis.

### 3.6 La misura — M-2

Eseguita il **2026-08-07** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11, profilo
`release`. Due crate separate, `kernel` e `simulator`, entrambe `#![no_std]` +
`#![forbid(unsafe_code)]`.

Scenario: 3 run concorrenti × 4 passi; ogni passo scrive l'intento, attende **5000 ms
virtuali** la risposta del modello, scrive l'esito.

| # | Criterio | Esito |
|---|---|---|
| **C1** | 100 corse, seme `20260806` | **1 sola traccia distinta** |
| **C2** | seme diverso | traccia diversa |
| **C3** | tempo virtuale | **20 000 ms virtuali in 25,8 µs di parete** |
| **NV** | non-vacuità: interlacciamento reale | **11 cambi di task su 23 transizioni** |
| **C7a** | nessun crash | **nessun** passo in dubbio — niente falsi positivi |
| **C7b** | crash riproducibile | **5 semi su 5**; col seme 99 i passi in dubbio sono `[3, 7]` |
| — | dipendenze esterne di `kernel` e `simulator` | **0 e 0** al momento della misura — §6.1.1 aggiunge poi `bincode` a `kernel`; quella di `simulator` **resta vuota** |
| — | sonda negativa: `std::fs` dentro `simulator` | ❌ `E0433` — `no_std` è attivo, non solo dichiarato |

**M-2 è chiusa: `simulator` regge `no_std`** con iniezione dei guasti, orologio virtuale e
doppio del giornale, a zero dipendenze.

#### 3.6.1 Due cose che la misura ha trovato e che non erano previste

| # | |
|---|---|
| 1 | **Un crash lascia _più_ passi in dubbio, non uno.** Col seme 99: `[3, 7]`. Con esecuzione interlacciata, due run possono avere entrambe l'intento scritto quando il processo cade. [ADR-0007](../../adr/0007-giornale-write-ahead-e-riconciliazione.md) diceva già «per **ogni** passo in dubbio», quindi la semantica regge — ma **l'aiutante `passo_in_dubbio` dello spike non sale così com'è**: restituiva un solo passo perché assumeva esecuzione sequenziale. Con l'interlacciamento dava un falso negativo. Sostituito da una versione che restituisce un insieme |
| 2 | **Il tempo virtuale è 20 000 ms, non 60 000.** Tre task da 4 attese di 5000 ms darebbero 60 000 ms se fossero sequenziali. Il numero misurato è la **controprova che la concorrenza è reale**, non solo il determinismo |

### 3.7 I costi

| Costo | |
|---|---|
| **il simulatore è lavoro prima di ogni valore visibile** | è RK-9, già accettato nella spec del kernel. La contropartita: senza, Q2, Q4 e Q5 restano dichiarazioni |
| **un seme non protegge domani** | §3.4. Il rischio reale è presentare l'elenco dei semi come una rete che non è |
| **la simulazione vede solo ciò che passa da una porta** | è anche la sua forza: qualunque sorgente di non determinismo nascosta si manifesta come **C1 che fallisce**, cioè tracce diverse a parità di seme. Il controllo esiste già ed è il primo |
| **la finta non è la vera** | che `platform` si comporti come `simulator` promette **non è verificato dalla DST**. Serve la quarta tecnica di [design/08](../../design/08-strategia-di-test.md), i **test di contratto**, ed è il vero punto cieco di questa sezione. La §4.6 ne chiude una parte per il giornale |

---

## 4. Giornale, riconciliazione e motore di persistenza

È la sezione che decide se le run lunghe arrivano in fondo. Porta la **seconda decisione**
della §0.5: [ADR-0032](../../adr/0032-motore-di-persistenza.md), motore `redb` con backend
nostro.

### 4.0 A parole

Il giornale è un quaderno su cui il kernel scrive **prima** di fare una cosa, e poi
**dopo** che l'ha fatta. Sembra pedante e invece è tutto: se il computer si spegne fra le
due scritture, riaccendendolo si trova una riga aperta e senza chiusura — e quella riga
dice *«questa cosa forse è successa e forse no»*.

Senza la prima scrittura quel dubbio non sarebbe **rilevabile**: la cosa sarebbe successa e
il quaderno non ne saprebbe nulla. È l'unico caso davvero cattivo, perché alla ripresa il
sistema la rifarebbe credendola mai avvenuta.

### 4.1 La porta `journal`

Il kernel dichiara cosa gli serve; chi lo fornisce sta fuori.

| Operazione | Cosa fa |
|---|---|
| `intent` | rende durevole **l'intenzione** di un passo, prima che l'effetto avvenga. **Uno per passo**: un secondo è rifiutato |
| `outcome` | rende durevole **l'esito**, dopo |
| `note` | appende una **nota** su un passo **già aperto** — né un intento né un esito. Quante se ne vuole; su un passo mai aperto è **rifiutata** |
| `read_back` | rilegge **un** passo **per nome**, alla ripresa, per la riconciliazione |
| `replay` | rilegge **tutto**, in ordine di scrittura, per scoprire l'insieme dei passi in dubbio |
| `prune` | toglie i record di un passo **riconciliato** (ADR-0018) — ⚠️ **due limiti dichiarati**, vedi il richiamo del 2026-08-27 |

> ⛔ **RICHIAMO DEL 2026-08-27 — questa tabella dichiarava CINQUE operazioni e la porta ne ha
> SEI, e la sesta non compariva in nessuna riga di questa spec.** Finding **AUD-003** del secondo
> audit completo, e il **solo finding ALTO che vi resti aperto** — verificabile con un `grep`,
> mentre *«il più grave»* non lo è: la fascia non è ordinata al proprio interno.
> ⛔ **Non è tipografia:** chi implementa `Journal` leggendo questa sezione — cioè la suite di
> conformità della §7.4.6 — scrive cinque metodi e **non compila**. ⚠️ **E qui c'era una seconda
> metà, TOLTA il 2026-08-27 invece che riallineata:** diceva *«e la seconda implementazione
> durevole che il Traguardo 6 chiede a `platform`»*, che nessun documento prevede — la durevole
> **esiste** dal Traguardo 3, e la §7.4.6 di questa stessa spec lo dice. Era l'unica affermazione
> di questo richiamo **ripresa dal rapporto** invece che misurata: gotcha **#65**. ✅ **Misurato il 2026-08-27 da FUORI la crate**, non argomentato: un `impl Journal`
> coi cinque metodi che questa tabella elencava dà
> `` error[E0046]: not all trait items implemented, missing: `note` ``.
>
> ⚠️ **La causa è che questa sezione fu emendata DUE VOLTE lo stesso 2026-08-10** — il riquadro
> su `replay` e quello su `intent` uno-per-passo — **e la terza operazione nata quel giorno non
> fu riportata**. Due richiami datati sulla stessa sezione la fanno leggere come riallineata per
> intero: è il gotcha **#31** nella forma *«una tabella che qualcuno ha appena toccato si legge
> come aggiornata»*.
>
> ⛔ **E con `note` entrano due cose che questa sezione non nominava** — finding **AUD-015**.
> `JournalError` ha **quattro** varianti, e questa sezione non ne nominava nessuna tranne
> `OutOfOrder`: la quarta è **`StepInDoubt`**, che `prune`
> restituisce a un passo ancora aperto, e **non** si riduce a `OutOfOrder` perché quella è
> definita da **V6** mentre potare troppo presto viola **ADR-0018**, che è un invariante diverso
> in un ADR diverso. E le vie di `OutOfOrder` sono **tre** e non due: vedi il richiamo dentro il
> riquadro di `intent`, qui sotto.
>
> ⛔ **E la riga di `prune` prometteva ciò che nessuno fa** — finding **AUD-031**. Diceva
> *«sostituisce un payload con impronta e dimensione»*: **entrambe** le implementazioni
> **tolgono** i record, e nel codice non esiste nessuna impronta. La riga dice ora ciò che
> l'operazione fa, e i **due limiti** di ADR-0018 sono dichiarati per esteso accanto al metodo in
> `crates/kernel/src/ports/journal.rs` e nella §4.5. ⚠️ **Non si chiudono qui**, e il perché è
> scritto lì.

> ⛔ **Un'operazione aggiunta il 2026-08-10, eseguendo il Traguardo 3 — e non è un ripensamento.**
> La §4.3 dice che la ripresa *«raccoglie **tutti** i passi con intento e senza esito»*, ma
> l'unica lettura che questa tabella offriva era `read_back`, che chiede un passo **per nome**.
> ⛔ **Dopo un crash il kernel non sa i nomi: la sua memoria è esattamente ciò che ha perso.**
> Con `read_back` da solo l'insieme non è scopribile, e non era una decisione presa — ADR-0007
> dice *«per ogni passo in dubbio»* senza dire **come si scoprono**. Era una lacuna. ⚠️ La
> descrizione di `read_back` è stata corretta nello stesso passaggio: diceva *«rilegge alla
> ripresa»* senza dire **per nome**, ed è proprio quel dettaglio a rendere visibile la lacuna.
>
> ⚠️ **Perché non è stata trovata prima:** `read_back` non ha **mai avuto un consumatore**. Le
> uniche implementazioni erano finte che ignorano l'argomento, e una firma senza chiamanti non
> si prova. È il gotcha **#46**, e la riconciliazione è il primo consumatore che la mette alla
> prova. ⚠️ **E la firma resta un'ipotesi finché quel consumatore non è scritto:** se scrivendo
> la riconciliazione risultasse scomoda o insufficiente, si cambia **qui**.
>
> ⛔ **Costo dichiarato:** `replay` carica l'intero giornale in memoria. Il rimedio noto è un
> **checkpoint**, e fissarlo ora congelerebbe un meccanismo che nessuna misura ha toccato.

> ⛔ **`intent` è uno per passo — deciso il 2026-08-10, eseguendo il Traguardo 3.** Un secondo
> intento su un passo che ne porta già uno è **rifiutato** con `JournalError::OutOfOrder`.
> Prima non era una decisione: era un comportamento **mai interrogato** — la finta lo accettava
> in silenzio e `read_back` rispondeva col primo. ⛔ **La ragione è la §4.2 stessa:**
> [ADR-0007](../../adr/0007-giornale-write-ahead-e-riconciliazione.md) dice *«l'intento di
> **ogni** passo»*, uno per passo, quindi un secondo è **fuori dal modello** e non un caso da
> disciplinare. Ed è la metà **simmetrica** del rifiuto di un esito senza intento: V6 tenuta
> dalla **porta**, non dalla diligenza del chiamante.
>
> ⚠️ **`OutOfOrder` si è allargata invece di guadagnare una variante vicina**, perché la §4.1
> vuole il tipo d'errore povero: *«un'operazione è arrivata fuori ordine per questo passo»*
> copre entrambe le direzioni, e il kernel non ha nulla da decidere diversamente fra le due.
>
> ⚠️ **RICHIAMO DEL 2026-08-27 — le vie sono TRE e non due**, e la terza è *«una `note` su un
> passo che non ha un intento»*, nata **lo stesso 2026-08-10** di questo riquadro e poche ore
> dopo. ⛔ **Questo capoverso non è riscritto perché è il VERBALE di quella decisione** e dice
> cosa fu deciso allora; l'elenco vivo delle tre vie sta accanto alla variante in
> `crates/kernel/src/ports/journal.rs`. Finding **AUD-015**.
>
> ⛔ **Vincola entrambe le implementazioni, ed è per questo che vive nella suite di conformità**
> (§7.4.6) e non dentro una delle due. Una tabella chiavata sull'identità del passo — la scelta
> naturale per `redb` — divergerebbe altrimenti dalla finta **senza che nulla diventi rosso**.

> ⚠️ **La porta scambia _byte_, non record tipizzati — aggiunto il 2026-08-07** con
> [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md). La codifica
> del record vive in `kernel`, ed è la §4.9 a dirne la regola. Due conseguenze che questa
> tabella non mostrava: il **simulatore scambia byte**, quindi la campagna DST esercita
> davvero codifica e decodifica invece di aggirarle; e la **forma durevole** resta di
> proprietà del kernel, coerente con la §4.4.

Due implementazioni, come per ogni porta:

| Implementazione | Vive in | Sotto c'è |
|---|---|---|
| reale | `platform` | `redb` con backend su file |
| finta | `simulator` | memoria, che cade a una scrittura scelta dal seme |

### 4.2 Il protocollo write-ahead

```mermaid
flowchart LR
    A["intent<br/>durevole"] --> B["l effetto<br/>avviene"] --> C["outcome<br/>durevole"]
    A -.->|"crash qui"| D["passo IN DUBBIO<br/>rilevabile"]
    B -.->|"crash qui"| D
    classDef bad fill:#b45309,stroke:#78350f,color:#fff
    class D bad
```

**Nulla si esegue prima che l'intento sia durevole** (V6). Il costo è due scritture per
passo, accettato in [ADR-0007](../../adr/0007-giornale-write-ahead-e-riconciliazione.md) e
mitigato dalla granularità: un passo è *un'interazione col mondo esterno*, non più fine.

### 4.3 La riconciliazione opera su un **insieme**

Scoperta di M-2, §3.6.1: con esecuzione interlacciata, **un crash lascia più passi in dubbio
insieme**. Col seme 99 lo scenario ne lasciava due, `[3, 7]`.

[ADR-0007](../../adr/0007-giornale-write-ahead-e-riconciliazione.md) diceva già «per **ogni**
passo in dubbio», quindi la semantica del progetto reggeva. Ma:

> **L'aiutante `passo_in_dubbio` dello spike non sale così com'è.** Restituiva *un* passo
> perché assumeva esecuzione sequenziale; con l'interlacciamento dava un **falso negativo**.
> Sostituito da una versione che restituisce un insieme.

La ripresa quindi: rilegge il giornale, raccoglie **tutti** i passi con intento e senza
esito, e li riconcilia uno per uno secondo la classe del proprio effetto — `verificabile`,
`idempotente`, `irripetibile`, e non dichiarata che vale `irripetibile`.

### 4.4 Il modello dello stato durevole

Cosa il giornale contiene, oltre ai passi. È l'elenco di
[ADR-0008](../../adr/0008-contesto-come-proiezione-dello-stato.md), e la regola che lo
governa è una sola: **ciò che deve sopravvivere si scrive**.

| Elemento | Sacrificabile |
|---|---|
| obiettivo · vincoli · piano · stato dei passi | mai |
| decisioni prese, con il motivo | mai |
| fatti acquisiti, con la provenienza | mai |
| artefatti — **riferimenti**, non contenuti | mai |
| trascrizione grezza | **sì**: unica perdita ammessa **nella proiezione** (ADR-0008) |

⚠️ **Due tabelle, due domande diverse — precisato il 2026-08-08.** Questa dice cosa la
**proiezione** può lasciar cadere (ADR-0008); la §4.5 dice cosa la **ritenzione** pota dal
giornale (ADR-0018), e sono prompt, risposte e output degli strumenti oltre alle
trascrizioni. «Unica perdita ammessa» vale qui, non lì: un payload potato **dichiara** di
esserlo, e quella non è una perdita silenziosa.

⚠️ **RICHIAMO DEL 2026-08-27 — quella frase descrive la REGOLA di ADR-0018, non ciò che il
codice fa:** entrambe le implementazioni **tolgono** il record invece di sostituirlo, quindi oggi
un payload potato **non** dichiara di esserlo. La misura, e chi la chiude, stanno nel richiamo
della §4.5. Finding **AUD-031**.

La **ricomposizione della proiezione** non è in questo sotto-progetto (§0.4, regola C): non
ha consumatore finché nessuno chiama un modello. Il *modello dei dati* però sì, perché è la
forma del giornale e la riconciliazione lo rilegge.

### 4.5 La ritenzione, lato kernel

| Livello | Ritenzione |
|---|---|
| struttura — identità, transizioni, esiti, routing, costi, verdetti | lunga |
| payload — prompt, risposte, output degli strumenti | finestra breve, poi **potati**: impronta e dimensione |
| artefatti | riferimenti; il contenuto vive sul filesystem |

Due regole non negoziabili di [ADR-0018](../../adr/0018-ritenzione-a-livelli-del-giornale.md):
un record potato **dichiara di esserlo** — payload assente e payload mai registrato non
devono confondersi — e **un passo in dubbio non è potabile** finché non è riconciliato.

> ⛔ **RICHIAMO DEL 2026-08-27 — NESSUNA DELLE DUE È TENUTA PER INTERO, e le regole restano
> quelle: a cadere è l'affermazione che il codice le rispetti.** Finding **AUD-031** e
> **AUD-006** del secondo audit completo.
>
> | La regola | Cosa fa il codice |
> |---|---|
> | un record potato **dichiara di esserlo** | ⛔ **violata da entrambe le implementazioni.** Nessuna sostituisce niente: `MemoryJournal` toglie le voci, `FileJournal` toglie le righe, e nel codice non esiste nessuna impronta. **Misurato il 2026-08-10:** un passo potato e uno **mai scritto** rispondono entrambi `Err(Missing)` a `read_back`, sono entrambi assenti da `replay`, e una seconda `prune` non li distingue |
> | **un passo in dubbio non è potabile** | ⚠️ **tenuta dalla porta con un'ALTRA nozione di dubbio.** La porta chiede *quale operazione è stata chiamata* — un `intent` senza `outcome`; la §4.3 chiede *cosa dicono i record*, decodificandoli, e un record che questa build non decodifica **entra** nel dubbio. ⛔ **Le due divergono nel verso che AUTORIZZA la potatura**, misurato il 2026-08-27 da **fuori** la crate e su **entrambe**: `steps_in_doubt` risponde `[InDoubt { step: StepId(1), resolution: SuspendAndAsk }]` e `prune` risponde `Ok(())` |
>
> ⛔ **Nessuna delle due si chiude qui, e il perché è scritto invece che taciuto.** La prima
> vuole l'**impronta** che ADR-0018 chiede a un record potato; l'impronta vuole una funzione di
> hash, e nel kernel quella è una **voce nuova nella lista di ADR-0031** — un atto deliberato
> che nessuna misura ha preparato. ⚠️ **E la via che sembrava non costarla è stata cercata e la
> misura la uccide:** svuotare il payload lasciando la voce li rende distinguibili, ma
> `steps_in_doubt` risponde allora `SuspendAndAsk` su **ogni** passo potato, a **ogni** ripresa
> — byte vuoti sono indecifrabili. La seconda **non è chiudibile sulla porta**, che non
> decodifica (ADR-0036): la guardia appartiene a **chi chiama**, e quel chiamante **non esiste
> ancora**. ⚠️ **Quanti ne abbia oggi non è scritto qui:** la misura e chi la chiude stanno nella
> **voce aperta 3** di [`porta-di-qualita.md`](../../porta-di-qualita.md), in una casa sola — una
> frase al presente sui chiamanti di una funzione diventa falsa **nel commit che ne scrive il
> primo**, e questo repository l'ha già pagata una volta con `ask_back`.
>
> ⚠️ **L'obbligo è quindi scritto accanto al metodo**, in `crates/kernel/src/ports/journal.rs`:
> la spazzata di ritenzione vive nel kernel e **può** decodificare, quindi dovrà consultare
> `steps_in_doubt` e saltare ciò che quello restituisce invece di poggiare sulla guardia della
> porta. **Chi le chiude:** il traguardo che porta la ritenzione, insieme alla decisione
> sull'impronta. Le voci aperte stanno in
> [`porta-di-qualita.md`](../../porta-di-qualita.md).

### 4.6 I due livelli di crash

È il contributo di questa sezione alla verifica, e chiude una parte del punto cieco
dichiarato in §3.7.

| Livello | Si inietta | Risponde a | Stato |
|---|---|---|---|
| **1 — alla porta** | il simulatore sostituisce l'intero giornale | *il kernel si riconcilia bene?* | ✅ provato in M-2: crash riproducibile su 5 semi, passi in dubbio rilevati, **nessun falso positivo** senza crash |
| **2 — dentro il motore** | il `StorageBackend` di `redb` cade a una scrittura o a un `sync_data` | *il motore lascia un archivio recuperabile?* | ✅ provato in M-8: 12 punti scattati, **12/12 riaperti, 12/12 coerenti** |

Sostituire la porta **non** copre il livello 2. È la ragione per cui il requisito 4 di
§10.6 esisteva, ed è ciò che ha deciso ADR-0032.

### 4.7 La misura — M-8

Eseguita il **2026-08-07** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11, profilo
`release` · `redb` 4.1.0. `StorageBackend` scritto da noi, in memoria, che fallisce a
un'operazione scelta.

| # | Requisito §10.6 **della spec del kernel** | Esito |
|---|---|---|
| 1 | scrittura durevole e ordinata, confermata | ✅ dopo riapertura i record **confermati** ci sono; quello di una transazione mai confermata **no** |
| 2 | lettura concorrente mentre si scrive | ✅ un lettore aperto **prima** continua a vedere la propria istantanea (5) mentre si scrive; uno nuovo vede lo stato nuovo (10). Nessun blocco |
| 3 | potatura selettiva senza riscrivere | ⚠️ **si stabilizza**: 4 116 → 16 452 → **32 900 KiB a 4, 6 e 8 giri**. Lo spazio è riusato, ma il livello di equilibrio è ~1 ordine di grandezza sopra il dato vivo; `compact()` recupera il 14 % |
| 4 | I/O iniettabile | ✅ 24 punti provati, **12 con fallimento osservabile**; di questi **12/12 riaperti** e **12/12 in stato coerente** — o i soli record confermati, o tutti. Mai uno stato parziale |

**Due errori miei, corretti e registrati:**

| # | |
|---|---|
| 1 | la prima versione di R4 iniettava a operazioni 12, 20 e 33 che la transazione **non raggiungeva mai**: tre prove su cinque erano **vacue** e stavano per essere riportate come successo. Corretta contando prima quante operazioni compie davvero una transazione, e iniettando solo dentro quel numero |
| 2 | la prima versione di R3 guardava **un solo giro** e concludeva «lo spazio non viene riusato». Falso: serviva misurare il **regime**. A regime si stabilizza |

**Limite dell'oracolo, dichiarato:** il controllo dopo il crash conta i record, non ne
verifica il contenuto integrale. La coerenza è *dimostrata su 12 punti*, non provata
esaustivamente.

### 4.8 I costi

| Costo | |
|---|---|
| **due scritture durevoli per passo** | accettato in ADR-0007. Trascurabile per un passo che chiama un modello, **non** per passi molto piccoli: la granularità del passo è una scelta, non un caso |
| **amplificazione dello spazio** | misurata: ~33 MiB contro ~2 MiB di dato vivo nel carico sintetico. Non cresce all'infinito, ma la potatura costa spazio. **Da rimisurare sul carico reale** prima di congelare i parametri di ADR-0018 |
| **`compact()` è esclusivo** | manutenzione da pianificare, non un'operazione da fare mentre il sistema lavora |
| **la riconciliazione è lavoro di progettazione per ogni strumento** | ogni effetto va classificato, e il default `irripetibile` produrrà interruzioni evitabili finché le classi non sono dichiarate bene |
| **il livello 2 è verificato su 12 punti, non esaustivamente** | è molto meglio di zero, e resta un campione |

> 📌 **La §4.9 è stata aggiunta dopo, e porta i propri costi in §4.9.7.** Non sono
> ripetuti qui: questa tabella copre le scelte che la §4 aveva quando è stata approvata.

### 4.9 L'evoluzione del formato durevole

> ⚠️ **Sezione aggiunta il 2026-08-07**, dopo l'approvazione della §4. È la voce **F2**
> della riapertura, e **F7** vi converge. Non corregge una scelta di questa sezione:
> risponde a una domanda che nessuna sezione poneva. La decisione completa, con le quattro
> forme, la misura e i costi, è
> [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md).

**A parole.** Il giornale è l'unico archivio che non si può rifare. E chi lo rilegge non è
chi lo ha scritto: è lo **stesso programma, mesi dopo, con campi in più**. Fino a qui
nessuna riga diceva cosa succede in quell'istante — e il modo di scoprirlo sarebbe stato
la prima evoluzione, cioè quando è troppo tardi.

⚠️ **La scadenza era temporale, non logica:** questa sezione andava scritta **prima della
prima riga di codice che scrive un record**. Alla sua data non ne esiste ancora nessuno, e
per questo la decisione non costa una migrazione. Sarebbe costata quella.

#### 4.9.1 Perché la postura di I4 non è disponibile qui

È [F1a](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) con la risposta rovesciata,
e la tabella lo rende immediato:

| | canale IPC (§6.1) | giornale |
|---|---|---|
| i due capi | **spediscono insieme** | lo **stesso programma in due momenti diversi** |
| se divergono | si rifiuta il pari stantio: **timbro di build** (§6.1.2) | ⛔ **non si può rifiutare il passato** |
| evoluzione dello schema | **rinunciata** esplicitamente | **obbligatoria** |

⛔ **Da cui la scorciatoia che questa sezione rifiuta:** *«usiamo `bincode` anche per il
giornale, tanto è già nella lista di ADR-0031»*. Sarebbe importare in un artefatto che
**deve** evolvere una decisione presa dove l'evoluzione era stata **rinunciata**. La §6.8
aveva scartato `minicbor` per l'IPC proprio perché i suoi indici servono all'evoluzione:
quel giudizio è giusto là e rovesciato qui.

#### 4.9.2 La regola

> **Ogni record durevole dichiara la propria versione, e i suoi campi si identificano per
> indice esplicito.**

| # | Regola | Cosa la sostiene |
|---|---|---|
| 1 | il tipo del record è un **enum di versione**: «un record senza versione» non è esprimibile | **livello 1**, la stessa mossa con cui V5 è salita al compilatore (§7.4.4 punto 3) |
| 2 | ogni campo porta un **indice esplicito**, scritto nel tipo | misurato: è l'indice a comprare il risultato, e costa **un byte su ventisei** |
| 3 | un campo nuovo è **facoltativo** e prende un **indice nuovo** | misurato ✅ in coda, in mezzo e in direzione inversa |
| 4 | un indice **si ritira e non si riusa mai**: il buco resta | misurato: il buco ✅, il riuso ⛔ **silenzio sbagliato** |
| 5 | un cambiamento **non additivo** apre una **versione nuova**; il lettore dispaccia e converte | è la sola cosa che l'indice non compra |
| 6 | la **codifica vive in `kernel`**, e la porta `journal` scambia **byte** | §4.9.3 — ⚠️ **aggiunta all'elenco il 2026-08-08**: la regola c'era, ma solo come prosa nella sottosezione seguente, mentre ADR-0036 la numera. Due documenti che condividono l'indice devono condividerlo per intero, o «regola 6» risolve in uno solo |

**Nessuna delle due metà regge da sola, ed è la misura a dirlo, non un giudizio:** la
disciplina solo-append su un formato posizionale **non funziona affatto** — anche un campo
opzionale in coda rende illeggibili i record vecchi — e il discriminante di versione su un
formato posizionale ha una trappola **non chiudibile**, perché i discriminanti espliciti
sono ignorati.

#### 4.9.3 Dove vive la codifica

**In `kernel`.** La porta `journal` scambia **byte**, non record tipizzati (§4.1).

| # | Perché |
|---|---|
| 1 | **coerenza di proprietà**: la §4.4 mette il modello dei dati in perimetro come cosa del kernel, e [ADR-0032](../../adr/0032-motore-di-persistenza.md) dice che `redb` conserva byte e la codifica è nostra. Con la codifica in `platform` la regola si applicherebbe dove il tipo non vive |
| 2 | **il controllo non resta appeso a un filo**: con la porta a byte il **simulatore scambia byte**, quindi la campagna DST esercita davvero codifica e decodifica, e i crash cadono *dentro* la scrittura |
| 3 | **il costo dove conta è misurato**: una crate spedita in più, senza dipendenze proprie, e il cancello senza OS di §7.3.2 **passa** |

⚠️ **Il costo che non è piccolo, e sta altrove:** il grafo **di build** del kernel passa da
due voci a sette, e per la prima volta porta `syn`. È un «`build graph changed`» ai sensi
di §7.3.1 — ammissibile con giustificazione, mai automatico. Le voci sono in §7.3.1.

#### 4.9.4 Come si verifica, e il limite dichiarato

| # | Livello | Meccanismo | Sonda — *deve scattare* | Contro-sonda — *deve restare verde* |
|---|---|---|---|---|
| 1 | **1 — compilatore** | il tipo del record è un enum di versione | costruire un record **senza versione** → non compila | con la versione → compila |
| 2 | **2 — controllo esterno** | **byte congelati**, con la mappa `indice → nome → valore atteso` | si **riusa** un indice o si rinumera → fallisce e **nomina il campo** | campo facoltativo con indice nuovo → resta verde |

**A parole il secondo.** Si scrive un record vero **oggi**, e i suoi byte entrano nel
repository come file. Ogni build futura deve rileggerli e ritrovare gli stessi valori.

> ⛔ **I byte congelati non si rigenerano.** Se cambiano non è un aggiornamento: è un
> cambio di formato, e va aperta una versione nuova. Rigenerarli in blocco cancella
> l'oracolo — è il **gotcha #25** trasferito dagli `.stderr` di `trybuild` a questo file.

**Un controllo e non due**, nello spirito della §7.4.4: un registro degli indici separato
sarebbe un secondo posto da tenere allineato per la stessa proprietà, e il primo che smette
di essere aggiornato mente in silenzio. Il file dei byte porta la mappa dentro di sé.

> ⛔ **Il limite, dichiarato prima che qualcuno lo scopra.** Il livello 1 prova che un
> record **dichiara** una versione, non che sia quella **giusta** — è il limite del gettone
> (§6.3.2), *prova la provenienza, non l'esattezza*. E la regola 4 resta una **disciplina**:
> nessun meccanismo del compilatore impedisce di riusare un numero. La regge un controllo
> di livello 2, quindi cancellabile.

#### 4.9.5 F7 converge qui, e si vede

Fork e branching sono **un campo in più** sul record: il passo padre, il punto di
diramazione. Sotto la regola sono un **campo facoltativo con un indice nuovo**, ed è
esattamente il caso che la misura dichiara ✅. Senza la regola sarebbero stati la
migrazione che questa sezione esiste per evitare.

⚠️ La tracciabilità afferma che «il giornale lo consente» per fork e branching. Con la §4.9
quell'affermazione ha finalmente un meccanismo sotto; prima era una promessa della sola
tabella. Il resto — quali campi, con quale semantica — è politica della capacità
Conversazione, non di questa sezione.

#### 4.9.6 Le evidenze

Eseguite il **2026-08-07** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11, `release` ·
`bincode` 2.0.1 · `minicbor` 2.3.0 · `serde_json` 1.0.151. Prototipi usa-e-getta fuori dal
repository. La matrice completa, i sei ritrovamenti e le quattro divergenze dalle attese
scritte prima stanno in
[ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md); qui restano le
righe che decidono questa sezione.

| Mutazione | posizionale | per indice | per nome |
|---|---|---|---|
| campo **opzionale** in coda | ⚠️ **errore** | ✅ | ✅ |
| campo **rimosso** | ⛔ **silenzio** | ✅ | ✅ |
| variante **rinumerata** | ⛔ silenzio | ⛔ silenzio | ✅ |
| **direzione inversa** — nuovo → vecchio | ⛔ **silenzio** | ✅ | ✅ |
| indice **riusato** | — | ⛔ silenzio | — |
| byte per record | 26 | **27** *(+ 3 con la versione)* | 76 |

**Non-vacuità, in due direzioni:** il caso di controllo è ✅ su tutte e tre le classi — il
banco funziona — **e** il banco ha prodotto davvero ⛔ su cinque celle, quindi sa
distinguere «errore» da «silenzio sbagliato». Senza la seconda metà la tabella non
proverebbe nulla: è il gotcha #14 applicato al banco invece che al controllo.

#### 4.9.7 I costi

| Costo | |
|---|---|
| **il grafo di build del kernel cresce da due voci a sette** | e porta `syn` per la prima volta. Superficie di supply chain a tempo di compilazione: non può violare V29 a runtime (§7.3.1), ma va rivista invece che subita |
| **il kernel porta due serializzatori** | uno per artefatto, con requisiti opposti. È coerente, e sono due grafi da guardare invece di uno |
| **ogni campo di ogni record durevole porta un indice** | si paga a ogni riga, non una volta. Un byte sul filo, un'annotazione in più nella scrittura |
| **la regola 4 è una disciplina** | «un indice non si riusa mai» non è imponibile dal compilatore. La regge un controllo di livello 2, cancellabile |
| **i byte congelati sono un oracolo rigenerabile** | la difesa è che la rigenerazione **si legge nel diff**, non che sia impossibile. Stessa forza e stessa debolezza del gotcha #25 |
| **la misura è su un record singolo** | che il costo in byte regga a regime non è provato: vale la stessa riserva del requisito 3 di ADR-0032, e le due rimisure si fanno insieme |

---

## 5. Arbitro GPU, e la lacuna su I2

È la sezione che decide se quattro pilastri possono contendersi una sola GPU senza che
nessuno vada in OOM. Porta la **prima decisione** della §0.5:
[ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md).

Non ri-progetta l'arbitro: [ADR-0005](../../adr/0005-arbitrato-gpu-su-due-dimensioni.md),
[ADR-0006](../../adr/0006-due-policy-vram-come-oggetti-distinti.md) e
[design/02](../../design/02-arbitrato-gpu.md) dicono già *cosa* fa. Qui si dice **con
quali tipi**, quale porta lo espone, e quale controllo prova quale vincolo.

### 5.0 A parole

L'arbitro è l'unico che può dire «sì, puoi toccare la GPU». Protegge due cose diverse,
con due meccanismi diversi:

| Protegge | Come | Se cede |
|---|---|---|
| la **memoria** | ammissione: o ci sta, o non parte | OOM — brutale e immediato |
| la **fluidità** | corsie: chi rallenta per chi | balbuzie, latenza che si allunga |

La prima è una capacità che si esaurisce; la seconda una contesa che degrada con
continuità. È il motivo per cui ADR-0005 le arbitra con meccanismi separati invece di
contare solo la VRAM.

### 5.1 Il modello della risorsa nel kernel

| Asse | Rappresentazione | Meccanismo |
|---|---|---|
| **VRAM** | MiB **interi**, in un tipo proprio | ammissione |
| **calcolo** | tre corsie ordinate — **non** un numero | ordinamento + segnale «riduci occupazione» |

Due scelte di rappresentazione, entrambe con un motivo:

| Scelta | Perché |
|---|---|
| MiB interi | la risorsa è quantizzata. Un intero toglie ogni domanda sull'arrotondamento, e le domande sull'arrotondamento in un percorso deterministico sono debito |
| tipo proprio, non un intero nudo | scambiare MiB con millisecondi **non deve compilare**. Stesso meccanismo che separa `Instruction` da `Untrusted` (§2.5) e il tempo monotonic dal wall time (§2.1) |

Il budget allocabile è ciò che resta dopo **due** sottrazioni, non una:

```
budget allocabile = totale − quota audio − quota presentazione
```

La seconda è nuova e viene da ADR-0033. La §5.5 la argomenta.

> ⚠️ **`totale` non aveva provenienza — corretto il 2026-08-07.** Questa formula compare
> identica in tre documenti — qui, in ADR-0033 e in `design/02` — e **nessuno diceva da
> dove venga `totale`**. Interrogare la GPU è una chiamata all'OS, che I3 vieta al kernel,
> e nessuna delle famiglie di porte della §2.3 fornisce la capacità dell'hardware.

**I tre addendi sono parametri consegnati** (§2.8), non numeri che l'arbitro va a
prendere. Vale per tutti e tre lo stesso trattamento che ADR-0005 dà alla riserva —
*dichiarata dal richiedente, picco misurato*:

| | |
|---|---|
| `totale` è **dichiarato** | nessuna porta nuova, nessuna dipendenza dal driver in `platform` adesso |
| l'occupazione reale si **misura e si registra** | con il meccanismo di §5.2.2, che esiste già |
| uno scarto sistematico è **un difetto del parametro** | non un incidente — è la stessa postura di ADR-0005 |

⚠️ **Il costo, dichiarato:** un `totale` sbagliato produce sovra-ammissione, cioè **Q2 che
cede per un errore di configurazione invece che di codice**. La mitigazione è la misura
del picco, non una verifica a priori che qui non esiste.

**Le strutture dell'arbitro sono `BTreeMap` e `Vec`.** Non è una preferenza: `HashMap`
vive in `std`, che la crate `kernel` non nomina — quindi il divieto del gotcha #12 è qui
gratuito e imposto dal compilatore (§1.4). Chiude anche **M-6**, vedi §5.8.

### 5.2 Il profilo di risorsa come tipo

I campi sono quelli di [design/02](../../design/02-arbitrato-gpu.md). Qui si aggiunge
come si esprimono.

| Campo | Tipo | Nota |
|---|---|---|
| `name` | identificatore nominato e versionato | V2: **ogni** lavoro GPU ne ha uno |
| `reserved_vram` | MiB | riserva **dichiarata** dal richiedente |
| `compute_class` | `realtime` \| `interactive` \| `batch` | la corsia |
| `preemptible` | booleano | governa se `InRevoca` è raggiungibile — §5.3 |
| `release_grace` | durata sull'asse **monotonic** | mai wall time |
| `cold_start` | durata | ⚠️ vedi sotto |

#### 5.2.1 `cold_start` non è raggiungibile dal percorso decisionale

design/02 dice di `avvio_a_freddo`: *«usato per avvisare l'utente, non per decidere»*.
È una regola scritta, quindi finora era una raccomandazione.

Si esprime invece nella struttura: `cold_start` **non sta nel profilo che l'arbitro
riceve**. Vive in un descrittore separato, che va alla proiezione di presentazione e non
alla funzione di ammissione. Una decisione che volesse leggerlo non ha come.

Costo: due strutture invece di una, e un punto in più dove tenerle allineate.

#### 5.2.2 Riserva dichiarata, picco misurato

ADR-0005: *«la riserva è dichiarata dal richiedente e verificata dall'arbitro; il picco
reale viene misurato e registrato»*. In pratica:

| | |
|---|---|
| alla concessione | si prenota `reserved_vram` |
| al rilascio | il picco osservato entra nel **giornale**, accanto al passo |
| se il picco supera la riserva | è un **difetto del profilo**, non un incidente: diventa materiale dell'anello 4 |

Il giornale è già il substrato (§4): non serve un secondo posto dove mettere le misure.
È il gotcha #7 applicato qui.

> ⚠️ **Da dove arriva il picco, e sotto quale regola nasce — aggiunto il 2026-08-08.** Il
> numero lo misura il **worker**: risale dalla porta `process`, in un messaggio progettato
> in **§6.10**. Nel record durevole nasce sotto la regola di **§4.9** — campo
> **facoltativo**, **indice nuovo**, e l'indice non si riusa mai.
>
> 📌 È la ragione per cui **F2 doveva precedere F1b**: progettare questo messaggio prima
> che la regola di evoluzione esistesse avrebbe significato aggiungere un campo a un record
> durevole **sotto nessuna regola**, che è esattamente il modo di fallire per cui §4.9
> esiste.

### 5.3 Il ciclo della concessione

La macchina a stati è quella di [design/02](../../design/02-arbitrato-gpu.md) e non si
ripete. Quattro punti che la traduzione in tipi aggiunge:

| # | Punto | Conseguenza |
|---|---|---|
| 1 | `Rifiutata` e `InCoda` sono **esiti distinti** (V4) | l'esito è a **tre vie**, non un «ha funzionato sì/no». Un requisito d'interfaccia diventa una firma: chi chiama è obbligato a distinguerli |
| 2 | la finestra di validità di `Concessa` vive sull'asse **monotonic** | nessuna decisione dell'arbitro legge il wall time. Un orologio che torna indietro non può scadere una concessione |
| 3 | `InRevoca` **non esiste** per i profili non prelazionabili | reso **non rappresentabile** invece che controllato a runtime: la transizione non è costruibile per un profilo con `preemptible = false` |
| 4 | `Forzata` — uccidere è sempre lecito | poggia su I1 e I5: nessun worker possiede stato. Passa dalla porta `process`, §5.6 |

#### 5.3.1 Perché i numeri di M-7 restano validi senza rimisurare

M-7 (§2.6) dichiarava fra i propri limiti: *«l'implementazione ordina l'intera coda a
ogni rilascio — una versione reale la terrebbe ordinata per corsia. I numeri sono quindi
un limite pessimistico.»*

La versione specificata qui tiene l'ordine **per corsia**, cioè è la versione più veloce
delle due. I numeri misurati restano quindi validi **come limite superiore**, e non c'è
niente da rimisurare: una misura che può solo migliorare non va rifatta per confermare
che è migliorata.

### 5.4 Le due policy come due oggetti

[ADR-0006](../../adr/0006-due-policy-vram-come-oggetti-distinti.md) resta invariato: due
oggetti con la stessa interfaccia, uno attivo per volta, determinato dal profilo di
configurazione. Ciò che la spec aggiunge:

| | |
|---|---|
| **la transizione è un passo giornalato** | ha effetti reali sul mondo — eviction, ricarica — e V6 dice che nulla si esegue prima che l'intento sia durevole. Una transizione interrotta a metà lascia un passo **in dubbio**, riconciliabile come tutti gli altri (§4.3) |
| **la policy non cambia per servire una richiesta** | design/05 lo dice già: in policy LOCALE una richiesta può finire su un provider remoto senza che la policy cambi. Il contrario non vale |

⚠️ **Una riga di ADR-0006 diventa incompleta.** La policy REMOTA dichiarava «VRAM
occupata: solo audio riservato». Con ADR-0033 sono audio **più** presentazione. ADR-0006
non è superato — la decisione delle due policy come oggetti distinti regge — e riceve un
rimando; `design/02` è aggiornato nello stesso passaggio.

### 5.5 La lacuna su I2, e come si chiude

Decisione completa, con alternative e costi:
**[ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md)**.

Il consumo GPU della GUI si modella come **tre consumatori distinti**, perché hanno
percorsi di richiesta diversi:

| # | Consumo | Governo | Corsia | Rifiuto esecutivo? |
|---|---|---|---|---|
| 1 | compositing della webview | **quota di presentazione** sottratta | `realtime`, **mai in coda** | ❌ no |
| 2 | viewer 3D **entro** la quota | stessa quota | `realtime`, **mai in coda** | ❌ no |
| 3 | viewer 3D **oltre** la quota | concessione **ordinaria**, richiesta via IPC | `interactive` | ✅ sì |

I consumatori 1 e 2 non entrano mai **in coda** perché una corsia è un **ordinamento che
l'arbitro applica a ciò che schedula**, e il compositor non lo schedula lui.

> ⚠️ **Corretto il 2026-08-08: la colonna diceva «fuori dalle corsie», e non era
> esprimibile.** La corsia è il campo `compute_class` del profilo (§5.2), che ha **tre**
> valori e che V2 rende **obbligatorio per ogni lavoro GPU**; la concessione di presentazione
> è una concessione con un titolare (ADR-0033), non un'esenzione, quindi un profilo ce l'ha
> per forza. «Fuori dalle corsie» sarebbe stato un **quarto valore** che il tipo non ha —
> e aggiungerlo per un consumatore che non viene mai ordinato sarebbe costruire per un caso
> che non esiste. Il profilo dichiara `realtime`; ciò che è vero è che **non entra mai in
> coda**, perché una concessione permanente non torna in ammissione. È un fatto sul ciclo di
> vita, non un valore di tipo.

#### 5.5.1 Perché non basta copiare la quota audio

La quota audio è sottratta **e** ha un titolare: il worker audio detiene una concessione
permanente e non prelazionabile. È il gotcha #4 — *la sottrazione non è un'esenzione*.

Ma per questo consumo **non c'è nessuno che possa chiedere**: chi alloca è il compositor, che non ha un percorso di
richiesta. Una quota sottratta senza titolare lascerebbe I2 falso.

**Il titolare è il core.** Richiede la concessione di presentazione all'avvio, permanente
e non prelazionabile; la GUI la consuma senza mai chiederla.

| Proprietà | |
|---|---|
| coerente con **I1** | la concessione è stato del core; la GUI non tiene nulla |
| **sopravvive alla GUI uccisa in qualsiasi istante** | il titolare ha vita lunga e indipendente. Nessuna concessione perduta, e nessun protocollo di liveness contro un processo progettato per morire (G3) |
| la quota non si libera a GUI chiusa | riallocata a un render, la GUI riaperta andrebbe in OOM |

#### 5.5.2 Cosa resta non imponibile, e come si dichiara

> Verso un worker il rifiuto dell'arbitro è **esecutivo**: il processo non parte.
> Verso il compositor **non lo è**: compone lo stesso.

La quota è una **promessa di budget, non un'imposizione**. Per la GUI I2 è quindi più
debole **in natura** che per i worker, e questo non è mitigabile con la tecnica. Si
dichiara, come si dichiara che «cifrato a riposo» vale quanto l'account OS (gotcha #6).

Cosa resta, e ha valore:

| # | |
|---|---|
| 1 | quella VRAM **non si alloca a nessun altro** |
| 2 | il picco reale si misura e si registra, con il meccanismo di §5.2.2 |
| 3 | il rischio residuo è dichiarato invece che scoperto |

#### 5.5.3 Il valore della quota è non misurato

Non esiste un numero misurato per la VRAM del compositing né per una scena three.js.
Inventarlo violerebbe il metodo. Vale il precedente di
[ADR-0010](../../adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md) con
SP-3: **default conservativo, dichiarato come non misurato**.

La misura è **M5** (§5.8), agganciata a M1–M4 di
[ADR-0029](../../adr/0029-guscio-della-gui.md).

#### 5.5.4 Cosa questa sezione esporta verso ADR-0029

ADR-0033 **non importa nulla dal guscio**: il meccanismo è identico su Tauri e su
Electron, come la lacuna prometteva. Esporta però un discriminante che ADR-0029 non
aveva: **quanto la quota sia governabile dipende da chi possiede il motore di
rendering** — impacchettato dal guscio, o quello di sistema.

È la prima volta che il kernel vincola la scelta del guscio invece del contrario, e per
questo la §5 **non è bloccata** da ADR-0029: la dipendenza va nell'altro verso.

### 5.6 La porta `process`, e I2 imposto dal compilatore

**A parole.** Oggi «nessun worker si avvia senza concessione» è una frase in un
documento, verificata da un test. Un test si può cancellare, e chi lo cancella non
incontra nessun ostacolo.

La porta `process` — dichiarata in §2.3 e progettata qui — **prende una concessione come
argomento** della funzione che avvia un worker. Una concessione può essere emessa solo
dall'arbitro. Chi scrive «avvia il worker» senza averne una **non compila**.

| | Prima | Dopo |
|---|---|---|
| forza di I2 sui worker | test | **compilatore** |
| vie di aggiramento | disciplina | `process` è l'**unica porta** verso i processi (§2.3): questa metà è del compilatore. ⚠️ **Corretto il 2026-08-08:** diceva «nessuna», appoggiandosi a *«`no_std` più ADR-0031»* — ma ADR-0031 esiste **proprio perché** `no_std` non copre il raggiungimento dell'OS attraverso una dipendenza (misura A3), e il suo controllo è dichiarato **il più debole delle quattro regole**. La seconda metà è retta da un controllo di livello 2, cancellabile: §1.4, riga con forza `controllo esterno` |

Il ragionamento sulla chiusura è lo stesso di §2.4.1: **non è la disciplina a reggere il
vincolo, è l'assenza di alternative.** Il kernel non ha un secondo modo di avviare un
processo perché non ha un modo di parlare con l'OS.

Costo, dichiarato: la firma diventa rigida, e spostare codice che avvia worker costa più
di prima. È lo stesso genere di costo della riga 2 di §1.6, e si paga per lo stesso
motivo.

### 5.7 Cosa la DST verifica qui

| Proprietà | Porta dove si inietta | Requisito |
|---|---|---|
| la somma delle concessioni non supera **mai** il budget allocabile | `reactor` — interlacciamento delle richieste concorrenti | **Q2** · I2 |
| nessun processo è `Attiva` senza concessione valida | `process` — kill in istanti arbitrari | **I2** · Q4 |
| **la GUI muore tenendo una concessione discrezionale** → la somma torna alla linea di base | `ipc` | **Q3**, esteso |
| una transizione di policy interrotta lascia un passo riconciliabile | `journal` | Q5 · V6 |
| una concessione scaduta non resta allocata | `reactor` — avanzamento dell'orologio virtuale | V1 |

#### 5.7.1 La non-vacuità, che qui è obbligatoria

Gotcha #14: **un controllo che non si è visto fallire non è un controllo.** Per la
proprietà principale — la somma non supera il budget — la sonda negativa è esplicita:

| Passo | Atteso |
|---|---|
| si rompe deliberatamente l'ammissione, concedendo oltre il budget | la campagna DST **fallisce**, e nomina il seme |
| si ripristina | torna verde |

Senza questo passo, «Q2 verificato» significa solo che la campagna è girata.

⚠️ E vale il gotcha #17 nella sua forma esatta: **iniettare un kill dove il codice non
arriva è una prova vacua che sembra un successo.** Per lo scenario della GUI che muore
tenendo una concessione, si conta prima quante operazioni compie davvero quel percorso,
si inietta dentro quel numero, e si **verifica che il guasto sia scattato** — non solo
che il test sia passato.

### 5.8 Le misure

| # | Domanda | Stato |
|---|---|---|
| **M-7** | quanto costa una decisione dell'arbitro | ✅ **già eseguita** — §2.6, con i cinque limiti dichiarati. **Non si rifà** |
| **M-6** | `BTreeMap`/`Vec` bastano alle strutture del kernel | ✅ **chiusa qui** — vedi sotto |
| **M5** | quanta VRAM prende la presentazione | ⬜ **non misurata, e dichiarata tale** |

#### 5.8.1 M-6 è chiusa dall'esistenza di M-7

Il prototipo di M-7 è una crate `#![no_std]` + `#![forbid(unsafe_code)]`, **zero
dipendenze**, con arbitro a quattro profili, corsie, coda e promozione al rilascio,
costruito interamente su `BTreeMap`. L'arbitro è la struttura dati più complessa che il
kernel abbia finora.

**M-6 è quindi già risposta per l'arbitro**, e non richiede una misura propria. Resta
aperta solo per le strutture che introdurrà la §6.

#### 5.8.2 M5 — quota di presentazione

| | |
|---|---|
| **Domanda** | quanta VRAM prendono compositing e viewer 3D, a riposo e sotto carico |
| **Metodo** | frontend Vue minimo con scena 3D, **sui due gusci** — è lo stesso allestimento che M1–M4 di ADR-0029 richiedono già |
| **Quando** | inizio del sotto-progetto 2. Non prima: richiede una GUI, che qui non esiste |
| **Output** | il valore della quota, **e** un discriminante per ADR-0029 |
| **Soglia** | se quota audio + quota presentazione non lasciano spazio a TRELLIS2 al profilo minimo accettabile → scatta **RK-1**, in una forma più severa di quella prevista: la mutua esclusività non è fra un LLM caldo e un render, ma **fra l'interfaccia e un render** |

#### 5.8.3 Cosa deliberatamente non si misura

| Non misurato | Perché |
|---|---|
| il comportamento dell'arbitro con **due** quote sottratte invece di una | M-7 ne aveva già una. La seconda è **aritmetica sul budget iniziale**, non una scoperta: una misura che non può fallire non è una misura. È il gotcha #17 nella forma «prova vacua che sembra un successo» |
| la taratura dei profili di risorsa reali | è SP-1 e SP-2, esplicitamente scaglionata dalla §0.4. I valori sono parametri, non impianto |
| il costo di disattivare l'accelerazione GPU della webview | è l'**ipotesi E** di ADR-0033, e resta un'ipotesi finché M5 non dice se serve |

### 5.9 I costi di questa sezione

| Costo | |
|---|---|
| **per la GUI, I2 è più debole in natura** | il rifiuto verso il compositor non è esecutivo. Non mitigabile con la tecnica: si dichiara |
| **VRAM sprecata a GUI chiusa** | identico al costo già accettato per la quota audio, e con la stessa mitigazione: un profilo senza interfaccia porta la quota a zero |
| **RK-1 si stringe di una quantità ignota** | la sua soglia era scritta su un budget che non contava la GUI. Di quanto lo dirà M5, ed è l'innesco osservabile del rischio |
| **il consumatore 3 è lavoro reale** | concessione revocabile verso un processo che può morire in qualsiasi istante: riconciliazione sulla disconnessione IPC, più uno scenario DST |
| **due strutture per il profilo invece di una** | il prezzo di rendere `cold_start` irraggiungibile dal percorso decisionale (§5.2.1) |
| **la contesa di calcolo resta indiretta** | ADR-0005 lo dichiarava già: far ridurre l'occupazione ai `batch` non è una garanzia forte. Questa sezione non lo migliora, e SP-2 resta la sua verifica |

---

## 6. Gli altri meccanismi: gateway, sensori, permessi, degrado

Chiude i meccanismi rimasti, ciascuno **al minimo che la §0.4 gli assegna**: quasi tutto
il resto è già scaglionato. Porta la misura **M-1**, che la bloccava.

### 6.0 A parole

Restano quattro macchine, più una. Questa sezione dice come sono fatte **le parti che
entrano ora**, non le macchine intere:

| Macchina | Cosa fa |
|---|---|
| l'**IPC** | come la finestra dell'app parla col programma di sfondo |
| il **gateway** | decide a quale modello mandare una richiesta, e con quali vincoli |
| i **sensori** | controllano la qualità di ciò che è tornato |
| i **permessi** | chi può fare cosa, e su quale risorsa esattamente |
| il **degrado** | dice cosa è rotto **prima** che tu ci sbatta contro |

### 6.1 La porta `ipc` e lo schema

#### 6.1.1 Dove vive lo schema — l'esito di M-1

M-1 chiude con l'esito **A** (§6.8): esiste un serializzatore il cui grafo transitivo è
accettabile, quindi **lo schema IPC vive in `kernel`** e il grafo di §1.2 non cambia.

La lista di [ADR-0031](../../adr/0031-dipendenze-del-kernel-parte-del-confine.md) smette
di essere vuota.

| Crate vincolata | Voci **proprie** | Grafo **transitivo** ammesso |
|---|---|---|
| **`kernel`** | `bincode` 2.0.1 · `unty` 0.0.4 | le stesse |
| **`simulator`** | **nessuna** — non serializza nulla | ⚠️ **quello di `kernel`**, perché vi dipende |

> 📄 **La lista completa vive in §7.3.1**, ed è la sede unica. Quella qui sopra elenca le
> voci **spedite** — quelle che entrano nel prodotto — con la giustificazione che le lega
> allo schema IPC. La §7.3.1 vi aggiunge la colonna **classe** e le voci che girano soltanto
> a tempo di compilazione, che nascono dal meccanismo di verifica e non da questa decisione.

> ⚠️ **La lista non è più solo dello schema IPC — aggiunto il 2026-08-07** con
> [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md). La §4.9
> mette la codifica del **giornale** in `kernel`, e vi aggiunge `minicbor` come **terza**
> voce spedita, con una giustificazione che non ha nulla a che vedere con I4. Le tabelle di
> questa sotto-sezione restano quelle dello schema IPC; **la lista completa è §7.3.1**.
>
> ⛔ E non è la stessa scelta estesa a un secondo artefatto: è una scelta **diversa**, presa
> sul requisito opposto. Il kernel porta due serializzatori perché ha due artefatti con
> requisiti opposti — §4.9.1.

> ⚠️ **La scelta è stata verificata sul _secondo_ capo del filo — aggiunto il 2026-08-08**
> con [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md). M-1 aveva
> chiesto se il **grafo transitivo** fosse accettabile: domanda giusta per I3, ma
> interamente sul **nostro** lato. Il pari di `ipc` è TypeScript
> ([ADR-0030](../../adr/0030-framework-dell-interfaccia.md)), e che sapesse rileggere
> questi byte **non era mai stato misurato** — P1 aveva due binari **Rust** ai due capi.
>
> ✅ **M-11 lo misura, e la risposta è sì**: `bincode-ts` 1.0.0 decodifica i byte veri con i
> valori giusti. **Questa sotto-sezione non cambia**: acquista l'evidenza che le mancava.
> ⚠️ E acquista una **fragilità dichiarata** — quel pacchetto è a una sola versione, con
> entrambi i punti d'ingresso rotti su Node 24. §6.10.6.

⚠️ **La riga di `simulator` è stata corretta da M-3.** La prima stesura diceva «la lista
resta vuota», che confonde due cose: `simulator` **non aggiunge voci proprie**, ma il suo
**grafo transitivo** non è vuoto — contiene `kernel` e quindi `bincode`. La regola di
ADR-0031 è sul grafo transitivo, quindi la lista di `simulator` deve nominarle. Misurato:
`cargo tree -p simulator` restituisce `bincode kernel unty`.

Giustificazione scritta, come la regola 1 di ADR-0031 richiede:

| Voce | Perché serve | Cosa raggiunge |
|---|---|---|
| `bincode` | serializza lo schema IPC (I4). L'alternativa era tipi speculari in `daemon` con uno strato di conversione da tenere allineato | **nulla**: compila per un bersaglio senza OS, e nel grafo non compare nessuna sorgente di casualità |
| `unty` | dipendenza di `bincode`: controllo di tipo alla decodifica | idem |

⚠️ **Il manifesto deve appuntare `2`.** `cargo add bincode` risolve alla **3.0.0**, che è
un segnaposto il cui intero sorgente è `compile_error!`. Vedi §6.8.2 e il gotcha #22.

#### 6.1.2 Il timbro di build — come si rifiuta una GUI stantia senza versionare

[I4](../../adr/0004-topologia-di-processo.md) impone che il protocollo sia **non
versionato**. Ma `core` e `gui` sono due binari, e una GUI vecchia può connettersi a un
core nuovo: parlerebbero due lingue leggermente diverse, e nessuno se ne accorgerebbe —
non fallirebbe, si comporterebbe in modo strano.

La distinzione che scioglie il nodo:

| | Cosa comporta | Ammesso da I4? |
|---|---|---|
| **versionare** | negoziazione, matrice di compatibilità, versioni vecchie da mantenere per sempre | ⛔ **no** |
| **timbro di build** | un solo valore accettato. Diverso → la GUI **non parte** e lo dichiara | ✅ **sì**: non è un contratto, è un'identità |

Non è una deroga a I4: è l'unico modo di **non** versionare senza che il protocollo
diverga in silenzio. Ed è la stessa postura di
[ADR-0012](../../adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md) e
[ADR-0025](../../adr/0025-confinamento-a-livelli.md): meglio rifiutare che funzionare a
metà.

#### 6.1.3 Gli identificativi sono progressivi, non generati

§2.2 dichiara che l'elenco dei consumatori di casualità nel kernel è **vuoto**, e lo
dichiara vuoto apposta.

Lo schema IPC porta identificativi di run e di passo. L'istinto diffuso è generarli
casualmente. Farlo riaprirebbe quella porta **alla prima riga di schema**, e non
comparirebbe in nessun elenco di «chiamate OS» — è il gotcha #12 nella sua forma più
insidiosa.

> **Gli identificativi nello schema IPC sono i progressivi del giornale.** Esistono già
> (§2.2), sono deterministici per costruzione, e sono leggibili in un trace.

#### 6.1.4 Cosa la porta deve permettere dopo, senza costruirlo ora

[ADR-0027](../../adr/0027-stack-della-gui.md) lascia un follow-up esplicito: se P3 con
rendering vero superasse il 25 %, *«la leva non è la GUI ma la frequenza di aggiornamento
decisa dal core: aggregare o campionare è una scelta di kernel.»*

Non si costruisce ora — sarebbe YAGNI. Ma la forma della porta **non deve precluderlo**:
il core decide *quando* emettere, la GUI non tira. È già così per costruzione (design/01:
il core comanda), e va scritto perché non venga eroso.

P1 è passato con margine — 2000 messaggi, zero persi, zero buchi — quindi **non serve un
ADR sulla contropressione**, come ADR-0027 aveva già stabilito.

### 6.2 Il decisore del gateway

Entra il **decisore**; gli adattatori dei provider reali sono scaglionati (§0.4, regola C).

| Entra | Da |
|---|---|
| risoluzione del routing e catena di candidati ordinata | ADR-0011 |
| filtro dei vincoli, con le due classi | ADR-0012 |
| record di routing **risolto**, giornalato col passo | ADR-0011 |
| contabilità gerarchica: token e costo attribuiti al passo | ADR-0011 |
| il rifiuto dell'arbitro come **causa di fallback di prima classe** | ADR-0012 · V1 |
| schema non conforme = **verdetto di sensore**, non eccezione | ADR-0013 |

Due confini che il gateway deve tenere, e che si prestano a essere confusi (design/05):

| Domanda | Discriminante |
|---|---|
| ritentativo o passo nuovo? | **il modello ha prodotto output?** No → stesso passo. Sì ma respinto da un sensore → passo nuovo: quell'output esiste, è stato pagato, e l'anello 4 deve vederlo |
| policy VRAM o destinazione? | la policy dice **cosa risiede in memoria**; il routing dice dove va *questa* richiesta. In policy LOCALE una richiesta può finire su un provider remoto senza che la policy cambi |

**Il decisore è verificabile senza chiamare un modello** ([ADR-0020](../../adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md)):
è ciò che lo rende testabile qui, dove nessun provider esiste ancora.

### 6.3 Q13 come proprietà — e il gettone non falsificabile

#### 6.3.1 Il dispositivo, nominato una volta

C'è una differenza fra due modi di garantire una regola:

| | Cosa succede se qualcuno sbaglia |
|---|---|
| «controlliamo che X prima di fare Y» | il controllo si dimentica, si cancella, o un percorso nuovo lo scavalca |
| «**Y non è scrivibile** se X non è vero» | non compila |

È la differenza fra scrivere «non entrare senza biglietto» sulla porta e installare un
**tornello**. Il progetto lo sta già usando tre volte, e conviene chiamarlo con un nome
invece di riscoprirlo ogni volta:

| Per fare questo… | …va consegnato questo | emesso solo da |
|---|---|---|
| avviare un worker | una **concessione** | l'arbitro (§5.6) |
| mettere testo nel canale delle istruzioni | un `Instruction` | la conversione dichiarata (§6.5) |
| **eseguire una richiesta** | una **prova di conformità** | il filtro dei vincoli |

La terza riga è nuova, ed è ciò che rende **Q13** — *«nessun candidato non conforme viene
mai eseguito, per qualunque catena»* — una proprietà invece di un controllo: un candidato
non filtrato non ha il gettone, quindi non è **esprimibile** come argomento di
un'esecuzione.

> ⚠️ **Gli usi sono quattro dal 2026-08-08, e la tabella completa non è questa.** La §6.10.1
> aggiunge *«leggere da un worker → una **ricevuta**»* con F1b, e la ripresenta per intero
> lì. Questa resta com'era perché era corretta quando è stata scritta; **la versione
> aggiornata è quella di §6.10.1**, e nel catalogo i gettoni sono cinque (§7.4.1 B).

#### 6.3.2 Il limite del dispositivo, dichiarato

> **Un gettone prova la provenienza, non la correttezza.**

Se il filtro dei vincoli ha un difetto, emette gettoni sbagliati e il compilatore non se
ne accorge: ha verificato che il filtro sia *passato*, non che abbia *ragione*.

La logica del filtro resta quindi materia di test e di DST. Il dispositivo elimina una
classe di errori — «ci siamo dimenticati di filtrare» — non due.

### 6.4 Il contratto del sensore, e il sensore finto

Contratto da [ADR-0009](../../adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md),
deliberatamente povero: `(artefatto) → (verdetto, dettaglio, costo)`.

#### 6.4.1 «Costo» sono due cose, e vanno separate

V11 dice: *«ogni sensore dichiara il proprio costo; gli inferenziali restano fuori
dall'anello stretto.»* Ma un costo **restituito** arriva dopo l'esecuzione: troppo tardi
per decidere se eseguire.

| | Dove sta | A cosa serve |
|---|---|---|
| costo **dichiarato** | nel registro, statico | decide se il sensore entra nell'anello stretto — **è V11** |
| costo **speso** | nel verdetto, misurato | entra nel giornale, alimenta l'anello 4 |

È il prezzo sul menù contro il conto: serve quello sul menù per decidere se ordinare.
Non è un ADR nuovo — è un'ambiguità che, non sciolta, rende V11 non implementabile.

#### 6.4.2 Il sensore finto, e cosa prova davvero

Q10 — *«un verdetto negativo rientra nell'anello con il feedback, senza intervento
umano»* — si verifica con un **doppio** che restituisce un verdetto scelto dal test.

Cosa questo prova e cosa no:

| Prova | Non prova |
|---|---|
| che l'**anello** raccoglie il verdetto, apre un passo nuovo (V14) e vi porta il dettaglio | che il **contratto** regga sensori reali |

Il secondo è **RK-5**, già accettato: il contratto va rivisto dopo il secondo sensore
reale in aree diverse, e se non si adatta **si spezza, non si piega**. È il costo
dichiarato in §0.6.

**Un sensore non modifica nulla** (V10): riceve l'artefatto per riferimento immutabile e
restituisce un verdetto. Correggere è compito dell'anello 1.

### 6.5 Il confine dei tipi

§2.5 ha già deciso la destinazione: `Instruction` / `Untrusted` salgono in
`kernel/src/boundary.rs`, sostanza invariata, con **la conversione che diventa
giornalata** (V19).

Quest'ultima ha una conseguenza concreta sulla firma:

> Se la conversione è giornalata, **la funzione di conversione riceve la porta
> `journal`**. Non è una funzione libera: non si può promuovere contenuto non fidato
> senza che qualcuno registri che è successo.

È il gettone di §6.3 applicato al confine: la registrazione non è una cortesia del
chiamante, è un argomento obbligatorio.

Q9 resta un **test di compilazione fallita** (design/08), e la suite `tests/compile_fail/`
cresce con ogni regola nuova (§2.5).

### 6.6 Il permesso come tripla

Entra la **forma** e la sua registrazione; il mediatore completo, i preset, il ciclo di
approvazione MCP e il canary sono scaglionati (§0.4, regola C: non c'è niente da mediare
finché non esistono strumenti).

| | |
|---|---|
| forma | `(strumento × risorsa × operazione)` — mai «lo strumento» |
| estensione | vale per la tripla concessa e per la **sessione corrente** (V21) |
| registrazione | un permesso concesso è un **fatto giornalato** |

Da quest'ultima riga discende una cosa che altrimenti costerebbe un sottosistema: «quali
permessi sono attivi ora» è una **proiezione del giornale**, non un secondo archivio. È
il gotcha #7 applicato ai permessi.

### 6.7 Lo stato di degrado come oggetto derivato

[ADR-0019](../../adr/0019-lo-stato-di-degrado-e-un-oggetto-osservabile.md): il core
mantiene uno stato di degrado corrente, alimentato dagli eventi, **derivato e
ricalcolabile, mai autorevole di per sé**.

| Ingresso | Da |
|---|---|
| connettività | §7 |
| **arbitro GPU** — inclusa la revoca della concessione discrezionale della GUI | §5 · ADR-0033 |
| salute dei provider | §6.2 |
| permessi, strumenti sospesi | §6.6 |

L'arbitro è già un ingresso nominato da ADR-0019; ciò che è **nuovo** è la §5, che ha aggiunto un consumatore
revocabile, e «il viewer 3D è in pausa durante un render» è **esattamente** una
condizione che cambia cosa l'utente può fare — quindi si dichiara (V27).

Il criterio di selezione resta quello di §7.5 della spec del kernel: si mostra ciò che
**cambia cosa l'utente può fare**, non ogni variazione interna. Un'interfaccia che
segnala tutto è indistinguibile da una che non segnala nulla.

### 6.8 La misura — M-1

Eseguita il **2026-08-07** · `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` ·
Windows 11. Prototipi usa-e-getta, fuori dal repository.

**La domanda, nella forma che ADR-0031 le impone:** non «esiste un serializzatore
`no_std`?» ma **«esiste un serializzatore il cui grafo transitivo sia accettabile?»**

Cinque candidati, ciascuno in una crate sonda `#![no_std]` + `alloc` +
`#![forbid(unsafe_code)]`, pilotata da un driver `std` — la stessa forma di `kernel`
guidato da `daemon`. Schema di prova: un messaggio con enum annidati, `String`, `Vec`,
`Option` e interi.

| Candidato | crate di **runtime** | totale | round-trip | bersaglio **senza OS** | casualità nel grafo |
|---|---|---|---|---|---|
| `minicbor` 2.3.0 | **1** | 6 | ✅ 68 B | ✅ | 0 |
| **`bincode` 2.0.1** | **2** | 4 | ✅ 62 B | ✅ | 0 |
| `postcard` 1.1.3 | 5 | 11 | ✅ 60 B | ✅ | 0 |
| `serde_json` 1.0.151 | 6 | 11 | ✅ 141 B | ✅ | 0 |
| `rkyv` 0.8.18 | 8 | 17 | ✅ 96 B | ✅ | 0 |

**Esito: A.** Tutti e cinque passano i criteri di ADR-0031. Lo schema IPC può stare in
`kernel`, e il grafo di §1.2 non cambia. L'esito **B** — tipi in `kernel` e
serializzazione in `daemon` — non è stato necessario.

**Scelto `bincode`**, con la giustificazione di §6.1.1. Le due alternative respinte e
perché:

| Respinto | Motivo |
|---|---|
| `minicbor` | grafo più piccolo di tutti, ma impone `#[n(0)]`, `#[n(1)]`… **su ogni campo di ogni messaggio** — indici che servono all'evoluzione dello schema, cioè a un beneficio **che I4 rinuncia esplicitamente**. Costo permanente per un vantaggio che non vogliamo |
| `postcard` | porta `serde`, che è ciò che leggono i generatori di tipi TypeScript — la mitigazione che ADR-0027 nomina per lo schema speculare della GUI. Ma quella generazione **oggi non è decisa**: tre crate in più per un'opzione ipotetica è YAGNI. Se un giorno si decidesse, è il momento di rivalutare |

> ⚠️ **Rimando — una premessa di questa riga è stata misurata, e cade. La conclusione no
> (2026-08-07).** La misura di [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md)
> ha prezzato quegli indici: nella codifica predefinita — ad **array**, non a mappa — costano
> **un byte su ventisei**. «Costo permanente» resta vero della *scrittura* — un'annotazione
> per campo — ma non dei byte, come questa riga lasciava intendere.
>
> ⛔ **La conclusione regge, e sulla gamba più forte delle due:** I4 rinuncia
> all'evoluzione, e far condividere un formato a due artefatti con requisiti **opposti**
> significa che un cambiamento fatto per uno si propaga sull'altro. È lo stesso
> ragionamento che ADR-0036 usa per rifiutare la scorciatoia inversa — `bincode` sul
> giornale — letto nell'altro verso. **§6.1.1 non si riapre**: il kernel porta due
> serializzatori perché ha due artefatti con requisiti opposti, ed è la coerenza, non la
> duplicazione.

> ⚠️ **La _domanda_ di questa misura era incompleta, e se ne è accorta solo il 2026-08-08**
> — [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md). «Esiste un
> serializzatore il cui grafo transitivo sia accettabile?» riguarda il **nostro** capo del
> filo. Un canale privato ne ha due, e il secondo non è Rust. La seconda metà —
> ***«l'ecosistema del pari ha un lettore conforme e mantenuto?»*** — è stata misurata
> dopo, con **M-10** e **M-11**: no per Python, sì per TypeScript.
>
> ⛔ **La scelta di questa sezione non cambia**, e la tabella dei cinque candidati resta
> valida per ciò che confronta. Cambia il fatto che ora è verificata su entrambi i capi
> invece che su uno.

#### 6.8.1 Le cinque sonde di non-vacuità

Gotcha #14: un controllo mai visto fallire non è un controllo. Ogni affermazione della
tabella poggia su una sonda **vista fallire**.

| Sonda | Esito |
|---|---|
| `std::fs` dentro la crate sonda | ✅ `E0433` — il `no_std` è **in vigore**, non solo dichiarato |
| `unsafe {}` dentro la crate sonda | ✅ `error: usage of an unsafe block` |
| crate `no_std` + `forbid` che raggiunge l'OS **via dipendenza** | ✅ **compila per l'host** — il gotcha #16 riprodotto in modo indipendente |
| la stessa, compilata per il bersaglio **senza OS** | ✅ **non compila** |
| il grep sulla casualità, applicato a quella crate | ✅ **1 riscontro** — quindi gli zeri della tabella sono zeri veri |

Senza la quinta riga, «casualità = 0» avrebbe potuto significare soltanto che il grep era
scritto male.

#### 6.8.2 Due scoperte che valgono oltre M-1

**1 — `cargo add bincode` risolve a una versione che non compile per costruzione.**

La 3.0.0 è l'ultima versione pubblicata, e il suo intero sorgente è
`compile_error!("https://xkcd.com/2347/")`: un segnaposto contro l'occupazione del nome.
È la stessa classe della riga su `sled` in
[ADR-0032](../../adr/0032-motore-di-persistenza.md), ma peggiore — lì la versione utile
era semplicemente più vecchia, qui **la versione più recente esiste ed è inutilizzabile**.
Diventa il **gotcha #22**: verificare che una versione esista non è verificare che
funzioni.

**2 — compilare per un bersaglio senza OS è un controllo strutturale di «raggiunge l'OS».**

Bersaglio usato: **`thumbv7em-none-eabihf`** — `core` + `alloc`, nessun `std`, nessun
sistema operativo sotto. `rustup target add`, poi `cargo build --target`.

È più forte di una allow-list per nome: non enumera le crate, le **prova**. E l'ho visto
respingere esattamente il caso che `no_std` da solo lascia passare — la sonda 4 di sopra,
dove `getrandom` fallisce con `target is not supported`.

⚠️ **Non sostituisce la allow-list, e va detto.** L'unificazione delle feature di cargo
può abilitare, nella build reale per Windows, feature che sul bersaglio bare-metal
restavano spente. È una **condizione necessaria forte, non sufficiente**. Riguarda
direttamente **M-3** (§7).

**3 — ADR-0031 dice «grafo transitivo» senza distinguere runtime da tempo di
compilazione**, e lo scarto misurato è grande: `minicbor` ha 1 crate di runtime e 6 in
totale. Un proc-macro gira sull'host durante la build e non può violare V29 a runtime, ma
**è superficie di supply chain**. La §7 deve decidere quale dei due il controllo misura;
qui si registra che sono due numeri diversi, non uno.

#### 6.8.3 Le divergenze dalle attese scritte prima

Quattro ipotesi su sei sono divergute. Registrate invece che allineate.

| Attesa | Misurato |
|---|---|
| «`bincode` 2.x con `default-features = false`» | ❌ **doppiamente falsa**: `cargo add` dà la 3.0.0, e quella non ha **nessuna feature** né compila |
| «`serde_json` porta `ryu`/`itoa`» | ⚠️ `itoa` sì; `ryu` è stato sostituito da **`zmij`** |
| «`postcard`: `serde` + `cobs`» | ⚠️ anche `serde_core` e `thiserror` |
| «`rkyv` ha molto `unsafe`» | ⚠️ vero, ma la conseguenza è un'altra: **la via sicura di deserializzazione richiede la feature `bytecheck`**. Senza, l'unica via è `unsafe`, che `forbid` rifiuta — cioè il candidato sarebbe stato **inutilizzabile**, non solo scomodo |

### 6.9 I costi di questa sezione

| Costo | |
|---|---|
| **la lista di ADR-0031 non è più vuota** | da qui in poi ogni aggiornamento di `bincode` è un evento da rivedere, non un'operazione automatica. È il costo che l'ADR aveva dichiarato, e ora si paga |
| **il manifesto va appuntato a `2`** | e la ragione va scritta accanto, o il prossimo aggiornamento la ripristina «sistemando» il vincolo |
| **il timbro di build va tenuto allineato** | due binari devono portarlo. Un passo di build in più, e se qualcuno lo dimentica il rifiuto scatta quando non deve |
| **il contratto del sensore resta un'ipotesi** | verificato con un doppio, su tre casi reali di cui **nessuno esiste**. RK-5 per intero |
| **il gettone prova la provenienza, non la correttezza** | §6.3.2. Elimina una classe di errori, non due |
| **il decisore non parla con nessun provider vero** | la prima integrazione reale può scoprire che una firma è sbagliata. Costo già dichiarato in §0.6 |

### 6.10 Il canale verso i worker

> ⚠️ **Aggiunta il 2026-08-08**, dopo l'approvazione della §6. È la voce **F1b** della
> riapertura: [ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) ha
> dichiarato la porta in §2.3.1, e qui la si **progetta**. Il formato di filo lo sceglie
> [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md), con le misure
> **M-10** e **M-11**. Sta in fondo alla sezione, come §2.8 e §4.9: **nessuna
> rinumerazione**.

**A parole.** Il core deve poter dire a un worker *«fai questo»*, e il worker deve poter
rispondere — anche molte volte, come per l'audio. Ma `design/01` dice che *«il worker non
risponde di iniziativa propria»*. Le due frasi sembrano litigare, e non litigano: **lo
streaming è istruito**. Il worker non parla mai da solo, riempie una casella che il core
ha aperto. Il problema è che oggi questo è un commento, e un commento non regge nulla.

#### 6.10.1 La tensione di `design/01`, e come si scioglie

| Riga della tabella dei canali | Cosa dice |
|---|---|
| `core → worker ML` | *«Avvia, istruisce, uccide. **Il worker non risponde di iniziativa propria**»* |
| `core → worker audio` | *«Idem; **il flusso audio risale al core**»* |

La forma della porta la scioglie così:

> **Ogni byte che risale è coperto da una _ricevuta_, e le ricevute le emette solo
> un'istruzione.** Un frame che nessuna ricevuta copre non ha modo di essere nominato:
> non è un dato, è un **guasto**.

È il **quarto uso** del dispositivo di §6.3.1, e conviene vederlo nella stessa tabella:

| Per fare questo… | …va consegnato questo | emesso solo da |
|---|---|---|
| avviare un worker | una **concessione** | l'arbitro (§5.6) |
| mettere testo nel canale delle istruzioni | un `Instruction` | la conversione dichiarata (§6.5) |
| eseguire una richiesta | una **prova di conformità** | il filtro dei vincoli |
| **leggere da un worker** | una **ricevuta** | l'istruzione stessa |

La prima frase di `design/01` resta vera **alla lettera**, e la seconda diventa un caso
della prima: il worker audio tiene aperta una ricevuta di flusso per tutta la propria
vita, aperta da un'istruzione sola all'avvio.

⚠️ **La ricevuta di flusso non è un passo del giornale.** I frammenti che risalgono da una
trascrizione continua sono una **sorgente di eventi**, non passi
([ADR-0011](../../adr/0011-routing-risolto-e-giornalato-per-richiesta.md), gotcha #1):
giornalarli violerebbe Q1. Ciò che si giornala resta la concessione e l'esito. Va scritto,
o qualcuno li giornala per diligenza.

#### 6.10.2 Le firme

| Operazione | Cosa fa | Cosa impone |
|---|---|---|
| `avvia(concessione, descrittore) → Worker` | avvia il processo | senza concessione **non compila** — §5.6, invariata |
| `Worker::istruisci_uno(frame) → RicevutaSingola` | un'istruzione con **una** risposta | l'unico modo di parlare è **l'oggetto che l'avvio ha restituito** |
| `Worker::istruisci_flusso(frame) → RicevutaFlusso` | un'istruzione con un **flusso** di risposte | idem — ed è **l'istruzione** a dichiarare quale delle due |
| `Worker::leggi_uno(RicevutaSingola) → Frammento` | una risposta sola | **consuma** la ricevuta: leggere due volte non compila |
| `Worker::leggi_prossimo(&mut RicevutaFlusso)` | il frame successivo | resta aperta finché il worker dichiara la fine, o il core chiude |
| `Worker::chiudi(RicevutaFlusso)` | chiude il flusso | |
| `Worker::uccidi(self)` | uccide, ed è **sempre lecito** (§5.3, punto 4) | **consuma il `Worker`**: istruire dopo l'uccisione non compila |

**Due tipi di ricevuta e non un enum a due rami.** Costa una funzione di lettura in più, e
compra che *«una risposta singola diventi un flusso»* non sia **esprimibile** — cioè
esattamente la frase di `design/01`. È la stessa mossa del punto 3 di §5.3: reso non
rappresentabile invece che controllato a runtime.

> ⛔ **Corretto il 2026-08-08: le istruzioni sono due, non una.** La tabella aveva una sola
> riga — `istruisci(frame) → Ricevuta` — con un tipo `Ricevuta` che non compare in nessun'altra
> firma, mentre le tre letture prendono `RicevutaSingola` e `RicevutaFlusso`. Le uscite erano
> due, ed erano **entrambe** contro la decisione: o `Ricevuta` è l'enum a due rami che il
> paragrafo qui sopra dichiara di aver comprato con una funzione in più, oppure non esisteva
> **nessun modo di ottenere una `RicevutaFlusso`** — cioè il flusso audio, che è la ragione
> per cui la §6.10 esiste. Con due punti d'ingresso è **l'istruzione** a dichiarare la forma
> della risposta, che è dove la decisione va presa: chi manda un'istruzione sa se si aspetta
> una risposta o un flusso. Costa **due** funzioni in più invece di una — il costo dichiarato
> in §6.10.7 cresce di conseguenza.

**Chi risveglia chi.** Nessuno attende dentro `process`. La prontezza arriva dal
`reactor`, come per ogni altra porta: l'esecutore chiede «cosa è pronto» e un frame
disponibile è una delle risposte. §2.4 resta intatta — **nessun thread nel percorso
decisionale**, e in simulazione è il seme a decidere quando un frame è pronto.

#### 6.10.3 Il formato di filo, e il criterio che lo sceglie

[ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md): il formato di
un canale privato si sceglie **anche** sull'ecosistema del pari, e la risposta si
**misura**. Misurata (§6.10.6): il pari **Python non ha** un lettore per `bincode`, il
pari TypeScript ce l'ha. Quindi i due canali privati ricevono formati diversi, e la
differenza è misurata invece che accidentale.

| | |
|---|---|
| **codificatore** | **`minicbor` 2.3.0** — voce **già spedita** (§7.3.1): zero aggiunte alla lista di ADR-0031 |
| **dove vive** | in **`kernel`**. La porta scambia **byte**, non messaggi tipizzati, come `journal` dopo [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md) |
| **perché lì** | è l'argomento 2 di §4.9.3 applicato qui: con la porta a byte **il simulatore scambia byte**, quindi la campagna DST esercita davvero codifica e decodifica invece di aggirarle |
| **gli schemi restano due** | condividere la crate **non è** condividere lo schema: ADR-0035, regola 2 |
| ⛔ **cosa il canale NON adotta** | la regola di **§4.9**. Usa la crate, **non la disciplina**: nessun enum di versione, nessun registro di indici ritirati, nessun byte congelato. I4 rinuncia al versionamento, e il meccanismo resta il **timbro di build** di §6.1.2, identico sui due canali |

#### 6.10.4 Due regole che escono dalla misura, non da un'opinione

| Regola | L'evidenza |
|---|---|
| **il frame dichiara la propria lunghezza**, e la decodifica verifica che i byte consumati siano **esattamente** quel numero | un decodificatore CBOR si ferma al **primo elemento completo** e ignora la coda. Misurato: dando a `cbor2` i byte di `bincode` di `Esito` restituisce **`1`**, senza sollevare nulla. Senza il controllo, un frame malformato è un **valore sbagliato**, non un errore |
| ogni `Vec<u8>` porta l'**annotazione di stringa di byte** | senza, `minicbor` lo codifica come **array di numeri**. Misurato su un frammento audio da 4096 B: **7813** contro **4101**, cioè **1,91×**. Compila, fa round-trip, ed è corretto: costa solo il doppio del traffico, in silenzio |

**Il campo che questo canale fa entrare nel giornale** è il **picco di VRAM** di §5.2.2:
arriva dal worker, in un messaggio progettato qui. Nasce sotto la regola di §4.9 — campo
**facoltativo**, **indice nuovo** — ed è la ragione per cui F2 doveva precedere F1b.

#### 6.10.5 Come si verifica, e il limite dichiarato

| # | Livello | Meccanismo | Sonda — *deve scattare* | Contro-sonda — *deve restare verde* |
|---|---|---|---|---|
| 1 | **1 — compilatore** | si parla a un worker solo con l'oggetto che l'avvio restituisce | parlargli senza `Worker` → non compila | col `Worker` → compila |
| 2 | **1 — compilatore** | `uccidi` consuma il `Worker` | istruire dopo `uccidi` → non compila | istruire prima → compila |
| 3 | **1 — compilatore** | leggere pretende una ricevuta | leggere senza ricevuta → non compila | con la ricevuta → compila |
| 4 | **1 — compilatore** | una ricevuta singola si consuma | leggere due volte dalla stessa → non compila | leggerne una → compila |
| 5 | **2 — controllo esterno** | i byte consumati sono pari alla lunghezza dichiarata | frame troncato, o con coda dopo l'ultimo elemento → fallisce | frame esatto → verde |

> ⛔ **Il limite, dichiarato prima che qualcuno lo scopra.** Il compilatore prova la forma
> **dalla nostra parte**, non che il pari la rispetti. Un frame non sollecitato viene
> **rifiutato**, non impedito: è un guasto che il simulatore inietta (§3.3), e contro un
> worker vero resta scoperto finché la suite di conformità non esiste — §7.4.6, rimandata
> perché worker non ce ne sono (§0.2).

#### 6.10.6 Le evidenze — M-10 e M-11

Eseguite il **2026-08-08**. `rustc 1.95.0` · Python **3.13.7** · Node **v24.9.0** ·
npm **11.6.0**. Prototipi usa-e-getta, fuori dal repository. Le tabelle complete, con i
candidati respinti e il motivo, stanno in ADR-0037.

| Misura | Domanda | Esito |
|---|---|---|
| **M-10** | il pari **Python** decodifica `bincode` 2.0.1? | ⛔ **no** — nessuna libreria; l'unica che si dichiara compatibile è ferma alla configurazione 1.x e non ha tipi somma. ✅ `minicbor` letto da `cbor2` 6.1.4: valori giusti |
| **M-11** | il pari **TypeScript** ci riesce? | ✅ **sì** — `bincode-ts` 1.0.0, valori giusti e byte tutti consumati. ✅ anche `cbor-x` 1.6.5 su CBOR |

⚠️ **Una fragilità dichiarata sul canale gui, che non cambia la sua decisione.**
`bincode-ts` è a **una sola versione** e ha **entrambi** i punti d'ingresso pubblicati
rotti su Node 24: ha funzionato dietro un bundler. Il sotto-progetto 2 può ancora
specchiare i tipi a mano; si scrive qui perché non venga scoperto allora.

#### 6.10.7 I costi

| Costo | |
|---|---|
| **due istruzioni e due funzioni di lettura invece di una e una** | il prezzo di rendere non rappresentabile ciò che `design/01` vieta a parole. ⚠️ **Ricontato il 2026-08-08:** diceva «due funzioni di lettura invece di una», e dimenticava che se i tipi di ricevuta sono due anche i **punti d'ingresso** devono essere due — altrimenti la firma che li emette è l'enum che la decisione vieta (§6.10.2) |
| ⚠️ **la regola 4 di ADR-0035 poggia su un presupposto ereditato** | *«non versionato»* regge sul **timbro di build**, che pretende un'identità da confrontare. Se l'ambiente Python del worker non è un artefatto **nostro versionato** — follow-up di ADR-0028, oggi solo una raccomandazione per i sotto-progetti 9 e 10 — il timbro non ha nulla da confrontare e la rinuncia al versionamento **cade su questo canale**. Dichiarato qui il 2026-08-08, perché l'ADR lo segnala e la sezione che progetta il canale lo taceva |
| **un `#[n(i)]` per campo** su ogni messaggio del canale | per un beneficio a cui I4 rinuncia. È il costo che M-1 aveva respinto per il canale gui, pagato qui per un'altra ragione — ADR-0037 |
| **un byte in più** sul messaggio piccolo misurato | otto contro sette. Trascurabile, e misurato invece che stimato |
| ⚠️ **`minicbor` serve due artefatti con requisiti opposti** | giornale e canale worker. Un cambiamento fatto per l'uno tocca l'altro: lo contengono il pin nel manifesto e il fatto che gli schemi restano distinti |
| **la lunghezza del frame è un secondo posto da tenere allineato** | col codificatore. È il prezzo per non confondere un frame malformato con un valore |
| **`process` diventa la porta più grande del kernel** | e §7.4.6 acquisisce un'affermazione in più sulla conformità fra la finta e la vera |

---

## 7. La porta di qualità: i controlli automatici

Le §0–§6 hanno deciso **cosa** il sistema garantisce. Questa sezione dice **chi lo verifica
al posto nostro, e con quale forza**. Non aggiunge regole: ogni voce difende una regola già
presa.

### 7.0 A parole

Un vincolo scritto in un documento è un'intenzione. Diventa una garanzia quando esiste un
meccanismo che **fa fallire da solo** chi lo viola, e quando quel meccanismo è stato **visto
fallire**.

I meccanismi però non hanno tutti la stessa forza, e presentarli come «controlli»
indistinti è il modo più semplice di mentire con autorevolezza — la stessa classe di errore
di «cifrato a riposo» dichiarato più forte di quanto sia (gotcha #6). La domanda che li
separa è una sola:

> **Se qualcuno cancella il controllo, la regola resta?**

### 7.1 Il criterio di ammissione

#### 7.1.1 Le tre regole per entrare

| # | Regola | Se manca |
|---|---|---|
| 1 | difende **qualcosa di nominato** — due rami, qui sotto | è un'abitudine, non un controllo: va tolta |
| 2 | **si è visto scattare** su una violazione deliberata | gotcha #14 — un controllo mai visto fallire non è un controllo |
| 3 | **si è visto restare verde** dove la cosa è lecita | gotcha #24 — un controllo che scatta dove non deve insegna a ignorare l'audit |

La terza è quella che si dimentica. È la ragione per cui in M-3 la sonda decisiva è stata
**N4** e non N1.

**I due rami della regola 1.** A separarli non è *cosa* proteggono, ma **di chi è il
verdetto che sostengono**.

| Ramo | Una voce entra se… | Se la cancelli |
|---|---|---|
| **1a** | difende un **`V`**, un'**`I`** o un **`Q`** nominato | quella proprietà smette di essere protetta |
| **1b** | sostiene la **validità del verdetto** di voci nominate del catalogo | quelle voci **restano scritte, e smettono di essere vere** |

⛔ **Il ramo 1b non è «allargare la regola», ed è la distinzione su cui la chiusura poggia.**
Allargarla a *«una proprietà decisa in una sezione nominata»* l'avrebbe resa incapace di
rifiutare, e una regola che non rifiuta mai è decorazione (§8.5.3.1). Il ramo 1b pretende
invece che la casella **nomini le voci del catalogo** di cui sostiene la validità, e questo
rifiuta: cancella `clippy` e non c'è **una sola** riga del catalogo che diventi falsa. La
§7.4.3 regge parola per parola.

> ⚠️ **La regola 1 nominava «V o I», e il catalogo difende anche dei Q — allineata il
> 2026-08-08.** Non è un allargamento: le righe `Q9`, `Q13` e `Q14` erano già lì, e la §8.1.2
> tratta i Q come cittadini di pari grado. Era la regola a essere rimasta indietro.
>
> ⛔ **Resta però un disallineamento vero, e non lo chiudo di mia iniziativa perché è una
> decisione.** Otto righe del catalogo hanno una colonna «Difende» che **non nomina** un V,
> un'I o un Q: `ADR-0026 v.2` · `§5.1` · `§2.1` · `§5.3` · `§5.2.1` · `§2.8 · ADR-0034` ·
> `supply chain` · `§3.7`. Sotto la lettera della regola 1 andrebbero **tolte** — ed è la
> stessa regola con cui questa sezione caccia `clippy` (§7.4.3) e lo stile del codice
> (§7.6.2), quindi non è una lettura severa. Le tre uscite possibili sono: **ri-attribuire**
> ciascuna al V che serve davvero, **allargare** la regola a «una proprietà decisa in una
> sezione nominata» — che però la renderebbe incapace di rifiutare, e una regola che non
> rifiuta mai è decorazione (§8.5.3.1) — o **dichiarare** una classe di eccezione con il suo
> motivo. Trovato il **2026-08-08**; aperto.

> ✅ **Chiusa il 2026-08-08. Il testo sopra resta perché era vero quando è stato scritto**,
> e perché è il verbale di ciò che è stato trovato. Va letto con questo richiamo.
>
> **Le otto righe non erano un problema solo: erano tre.** Guardandole con la domanda del
> ramo 1b — *«se cancello questo controllo, quale altra riga smette di essere vera?»* — si
> separano da sole:
>
> | Righe | Esito | |
> |---|---|---|
> | `§5.2.1` `§2.1` `§5.1` `§5.3` | **1a**, ri-attribuite | `Q8` · `V29` · `Q2` · `I2` |
> | `§2.8 · ADR-0034` | **1a**, ri-attribuita | `V29`, il cui testo si allarga — vedi sotto |
> | `ADR-0026 v.2` `supply chain` `§3.7` | **1b** | non difendono una proprietà del sistema: difendono **il verdetto di altri controlli** |
>
> ⚠️ **Perché `Q8` è la prova che il difetto era di contabilità e non di merito:** la §8.4
> scriveva già *«§5.2.1 rende `cold_start` irraggiungibile dal percorso decisionale, con
> test di compilazione fallita (§7.4.1 C)»*. L'attribuzione **esisteva**, scritta da un lato
> solo. È il gotcha #36 nella sua forma pura — l'**asimmetria** — e per questo la §8 non
> cambia stato: nessuna cella di §8.3 o §8.4 aveva sbagliato giudizio.
>
> ⛔ **La riga `ADR-0026 v.2` è quella che ha imposto il ramo 1b.** `forbid(unsafe_code)`
> non impedisce niente di suo: impedisce che un `unsafe` falsifichi un gettone, transmuti un
> newtype o raggiunga l'OS con un `extern` dichiarato a mano. Toglilo e **nessuna** riga del
> catalogo diventa rossa — diventano tutte **meno vere**. Costringerlo a nominare un `V`
> sarebbe stato più semplice e falso. Stessa natura per il grafo **di build** — se una
> dipendenza di compilazione è compromessa, «non compila» non significa niente, e il kernel
> ora porta `syn` a tempo di compilazione — e per i **test di contratto**, senza i quali ogni
> `Q` della campagna è provata contro una finta.
>
> ✅ **E la regola 1 non è più un'intenzione**, che era il difetto dichiarato in §7.7.1 per
> la regola 3: `check-docs.sh` ha una **sesta asserzione** che la verifica (§8.6.1).
> **Provata in due direzioni sullo stesso giro, con un caso storico invece che costruito:**
> alla prima corsa, sulle trentatré righe del catalogo, ha nominato **le otto e solo le
> otto** — 8 rosse, 25 verdi — e dopo la correzione è verde su tutte e trentatré.

#### 7.1.2 La scala di forza, e ogni controllo dichiara la propria

| Livello | Meccanismo | Se cancelli il controllo | Se lo aggiri |
|---|---|---|---|
| **1 — compilatore** | `no_std` · `forbid(unsafe_code)` · una firma che pretende un gettone | **la regola resta**: la violazione continua a non compilare | non si può — `E0453` rifiuta anche un `#[allow]` locale |
| **2 — controllo esterno** | allow-list sul grafo · cancello senza OS · grafo delle crate | **la regola sparisce**: sotto non c'è nient'altro | si cancella il controllo |
| **3 — lint** | `clippy.toml` | la regola sparisce | **una riga di permesso**, senza cancellare niente |

Il livello non è una qualità del controllo: è una **proprietà dichiarata accanto ad esso**.
[ADR-0031](../../adr/0031-dipendenze-del-kernel-parte-del-confine.md) lo fa già per la
propria regola — *«è un controllo, non il compilatore […] va detto invece che sperato»*.
Questa sezione lo generalizza: **ogni voce del catalogo porta il proprio livello**.

#### 7.1.3 Il test di compilazione fallita è di livello 1, e va detto

Distinzione controintuitiva, e per questo isolata qui.

Quando la regola è del compilatore, il test che l'accompagna **non regge la regola**: la
*dimostra*. Cancellarlo non riapre la violazione — la rende invisibile.

| Se cancelli… | La violazione torna possibile? | Cosa si perde |
|---|---|---|
| un test di compilazione fallita | ❌ **no** | la **visibilità** della regola |
| il controllo della allow-list | ✅ **sì** | la **regola** |

Conseguenza: i test di compilazione fallita entrano nel catalogo con **forza di livello 1 e
visibilità di livello 2**. Senza l'etichetta doppia si sopravvaluta ciò che si perde
cancellandoli, e si sottovaluta ciò che si perde cancellando gli altri.

#### 7.1.4 Come si esclude che un test negativo fallisca per il motivo sbagliato

Il gotcha #9 nasce in Go: un test che verifica «questo non compila» passa **anche quando la
compilazione fallisce per un'altra ragione**. È un falso positivo travestito da successo.

In Rust il meccanismo lo esclude per costruzione: `trybuild` confronta l'errore prodotto con
un **file di riferimento salvato accanto al caso**, quindi verifica il *testo* dell'errore e
non soltanto il suo esito. È già così in
[`spikes/rust/tests/compile_fail/`](../../../spikes/rust/tests/compile_fail/), dove ogni
caso ha il proprio `.stderr`.

> ⚠️ **Ma i file di riferimento si possono rigenerare in blocco.** `trybuild` offre un modo
> di riscrivere tutti gli `.stderr` sull'output corrente. Serve, quando i messaggi cambiano
> legittimamente; usato senza leggerli, trasforma ogni caso in una tautologia — *«l'errore
> atteso è quello che è uscito»* — e da quel momento la suite **passa per sempre**.
>
> La rigenerazione è un **atto deliberato e si legge nel diff**, esattamente come aggiungere
> una voce alla lista di ADR-0031. È il **gotcha #25**.

### 7.2 Le evidenze di M-3

#### 7.2.0 Perché questa misura sta all'inizio e non in fondo

Nelle §4, §5 e §6 la misura chiude la sezione: **conferma** una scelta già argomentata. M-3
ha un ruolo diverso — non conferma la §7, la **abilita**: è la prova che il meccanismo di cui
la sezione parla esiste con la sola toolchain standard. E le due decisioni della §7.3 sono
alla lettera *ciò che M-3 lascia aperto*. Leggerle prima delle evidenze sarebbe leggere una
conclusione senza la premessa.

#### 7.2.1 Esito e strumento

Eseguita il **2026-08-07** · `rustc 1.95.0` · `cargo 1.95.0` · Windows 11. Workspace di
prova che replica il layout reale: `kernel` (`no_std`+`forbid`, con `bincode`) ·
`simulator` (`no_std`+`forbid`, dipende da `kernel`) · `platform` (std) · `daemon`.

**Esito A: esprimibile con la sola toolchain standard.** Nessuno strumento esterno.

| Scoperta | |
|---|---|
| ⛔ **`cargo metadata` non va bene** | le *feature attive* che riporta sono corrette, ma il suo elenco `deps` **le ignora**: mostra anche le dipendenze opzionali spente. Sul caso reale segnalava undici crate esterne invece di due, fra cui `serde` e `syn`, che non vengono compilate |
| ✅ **`cargo tree` sì** | risolve davvero le feature, e `--prefix depth --format {p}` dà un output ricostruibile |
| costo dichiarato | `cargo tree` è un'interfaccia **pensata per gli umani**: nessuna garanzia di stabilità del formato, a differenza di `cargo metadata`. È il prezzo di avere le feature risolte |

#### 7.2.2 Le sonde, tutte viste fallire e poi tornare verdi

| # | Sonda | Atteso | Osservato |
|---|---|---|---|
| N1 | violazione **transitiva** — tolta `unty` dalla lista | fallisce **nominando il rimbalzo** | ✅ `X unty <- kernel -> bincode -> unty` |
| N2 | `getrandom` diretto in `kernel` | fallisce | ✅ `X getrandom <- kernel -> getrandom` |
| N3 | `getrandom` in `simulator` | fallisce | ✅ segnalato **solo** su `simulator` |
| **N4** | **contro-sonda**: `getrandom` in **`platform`**, dove ADR-0031 lo ammette | ⚠️ **non deve scattare** | ✅ `CONFORME`, uscita 0 — e verificato che `platform` lo raggiunga davvero |

**N4 è la sonda che di solito si dimentica.** `platform` **deve** poter toccare l'OS: è il
perimetro esplicito di ADR-0031. Senza quella sonda, una regola troppo larga sarebbe passata
per una regola che funziona.

#### 7.2.3 La correzione trovata riverificando M-3: il filtro era sbagliato

Riverifica eseguita il **2026-08-07**, stesse versioni, su un workspace che replica il
layout reale **con la dipendenza di sviluppo che il kernel avrà davvero** — `trybuild`, che
la §2.5 fa salire insieme a `tests/compile_fail/`.

| Classe | Comando | Crate | Contenuto |
|---|---|---|---|
| **spedita** — entra nel prodotto | `-e normal,no-proc-macro` | **2** | `bincode` · `unty` |
| **di build** — gira sull'host a compilazione | il complemento su `-e no-dev` | **+2** | `bincode_derive` · `virtue` |
| di sviluppo — non esce di qui | il complemento sul comando senza filtri | +24 | `trybuild` e il suo sottoalbero |
| ⛔ **`-e no-proc-macro` da solo** | **il comando nominato in HANDOFF** | **20** | fra cui **`windows-sys`** e **`windows-link`** |

> **HANDOFF affermava che `-e no-proc-macro` separa il grafo di runtime da quello totale.
> Non li separa.** Da solo toglie i generatori di codice ma **lascia dentro l'intero
> sottoalbero di sviluppo**, e con esso le API di Windows. Un controllo costruito su quel
> comando segnalerebbe venti crate invece di due, con dentro `windows-sys`: sarebbe una
> macchina da falsi positivi, cioè il gotcha #24 nella sua forma più pura.
>
> Il comando corretto è **`-e normal,no-proc-macro`**.

**Perché M-3 non poteva accorgersene.** Contro-sonda eseguita: **tolta la dipendenza di
sviluppo, `-e no-proc-macro` da solo restituisce esattamente `bincode unty`** — cioè sembra
corretto. Il workspace di M-3 non ne aveva, quindi la sonda **non poteva falsificare
l'affermazione**. È il gotcha #17 applicato a M-3 stessa: un guasto iniettato dove il codice
non arriva.

I numeri «due contro quattro» restano giusti. Era sbagliato il **comando attribuito a
produrli** — cioè esattamente ciò che finisce nel controllo automatico.

#### 7.2.4 La correzione che M-3 aveva già imposto alla §6.1.1

Registrata al suo posto: `simulator` non aggiunge voci proprie, ma la sua lista **non è
vuota**, perché dipende da `kernel` e la regola 2 di ADR-0031 è sul grafo *transitivo*.
Vedi §6.1.1.

### 7.3 Le due decisioni che questa sezione prende

Nessuna misura le decide al posto suo. M-3 le ha rese concrete con dei numeri; la scelta
resta una scelta.

#### 7.3.1 Il grafo: due classi cancellate, una esclusa e provata esclusa

> **Decisione.** La lista di ADR-0031 dichiara per ogni voce la propria **classe**. Il
> controllo verifica **due** grafi con due comandi distinti, e produce **due errori diversi
> con due rimedi diversi**. Le dipendenze di sviluppo sono **escluse, e l'esclusione è
> provata**.

| Classe | Comando | Errore | **Rimedio** |
|---|---|---|---|
| **spedita** | `cargo tree -p <crate> -e normal,no-proc-macro` | **`I3 violated`** | ⛔ **togliere la dipendenza.** Aggiungerla alla lista *non* è un rimedio |
| **di build** | il complemento fra `-e no-dev` e la riga sopra | **`build graph changed`** | ✅ valutare e **aggiungere alla lista**, con giustificazione |
| di sviluppo | ❌ non cancellata | — | — |

**Perché due e non uno.** Non è completezza: ADR-0031 **ha già deciso questo**, fra le
proprie `Negative` — *«Il grafo cambia sotto di noi. […] aggiornare una dipendenza del
kernel diventa un **evento da rivedere**, non un'operazione automatica.»*

Un controllo che guarda solo ciò che spedisce lascia passare in silenzio proprio l'evento
che l'ADR dice di rivedere. E un controllo che li unifica insegna il rimedio sbagliato —
«aggiungi alla lista» — **anche per una violazione di I3**, che è il modo in cui
un'invariante si degrada in scartoffia.

**Perché le dipendenze di sviluppo si escludono.** Non spediscono e non girano alla
compilazione del prodotto: non possono violare V29 né I3. Ma l'esclusione **va dimostrata**,
o «la lista è corta» potrebbe voler dire soltanto «l'interrogazione era stretta» — è la
quinta sonda di M-1 applicata al comando invece che al grep.

> **Guardia di non-vacuità del controllo.** Se il grafo completo e quello spedito
> **coincidono**, il filtro non sta distinguendo niente — ed è la condizione esatta in cui
> M-3 è stata ingannata (§7.2.3). Il controllo lo **segnala** invece di passare in silenzio.

**La lista, nella forma che questa decisione le dà.** Vive qui, ed è la sede unica: §6.1.1
conserva la giustificazione delle voci spedite e rimanda a questa tabella.

| Crate vincolata | Voce | Classe | Perché serve | Cosa raggiunge |
|---|---|---|---|---|
| **`kernel`** | `bincode` 2.0.1 | **spedita** | serializza lo schema IPC (I4) | **nulla**: compila per un bersaglio senza OS, e nel grafo non compare nessuna sorgente di casualità |
| **`kernel`** | `unty` 0.0.4 | **spedita** | controllo di tipo alla decodifica di `bincode` | idem |
| **`kernel`** | `minicbor` 2.3.0 | **spedita** | codifica il **record durevole** del giornale, per indice esplicito — §4.9 · [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md) — **e lo schema del canale verso i worker**, §6.10 · [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) | **nulla**: misurato il 2026-08-07 su `x86_64-unknown-none`, e nel grafo spedito non compare nessuna sorgente di casualità |
| **`kernel`** | `bincode_derive` 2.0.1 | **di build** | genera il codice di serializzazione; gira sull'host e non entra nel prodotto | **l'host, a tempo di compilazione** |
| **`kernel`** | `virtue` 0.0.18 | **di build** | dipendenza di `bincode_derive` | idem |
| **`kernel`** | `minicbor-derive` 0.19.5 | **di build** | genera codifica e decodifica del record durevole **e dei messaggi del canale worker**, con gli indici dichiarati | idem |
| **`kernel`** | `syn` 2.0.119 | **di build** | dipendenza di `minicbor-derive` ⚠️ | idem |
| **`kernel`** | `quote` 1.0.47 | **di build** | idem | idem |
| **`kernel`** | `proc-macro2` 1.0.107 | **di build** | idem | idem |
| **`kernel`** | `unicode-ident` 1.0.24 | **di build** | dipendenza di `proc-macro2` | idem |
| **`simulator`** | *nessuna propria* | — | — | ⚠️ eredita per intero il grafo di `kernel` |

⚠️ Le voci **di build** non comparivano in §6.1.1, che elenca le sole spedite. Non è una
contraddizione — §6.1.1 rimandava già il meccanismo a questa sezione — ma è la prima volta
che la lista nomina crate che non entrano nel prodotto, e la colonna «classe» esiste per
questo.

> ⚠️ **Cinque voci di build aggiunte il 2026-08-07 con ADR-0036, e una va guardata in
> faccia.** Il grafo di build del kernel passa da **due** voci a **sette**, e per la prima
> volta porta **`syn`**: `bincode_derive` usa `virtue` apposta per evitarlo. Non può violare
> V29 a runtime — è la ragione per cui questa classe esiste — ma **è superficie di supply
> chain**, ed è esattamente l'«evento da rivedere» che ADR-0031 dichiara fra le proprie
> `Negative`. Registrato invece che assorbito.
>
> **L'alternativa che lo evitava è stata valutata e scartata:** scrivere a mano codifica e
> decodifica per ogni tipo di record toglie le cinque voci e sposta il costo **da una volta
> a per sempre**. Il costo dichiarato in ADR-0036.

> ⚠️ **Due giustificazioni allargate il 2026-08-08, e nessuna voce aggiunta** —
> [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md). `minicbor` e
> `minicbor-derive` servono ora **anche** lo schema del canale verso i worker (§6.10). La
> lista **non cresce**: il canale riusa una voce già spedita.
>
> ⛔ **Ed è l'allargamento opposto a quello che F1b si aspettava.** L'istruzione diceva di
> allargare le giustificazioni di `bincode` — «serializza lo schema IPC» — dando per
> scontato che il canale worker ne ereditasse il formato. La misura ha detto altro, e
> quelle righe **restano esatte come sono**: `bincode` continua a servire il solo canale
> gui. Si registra la divergenza invece di allinearsi all'attesa — gotcha #15.

**I costi:**

| Costo | |
|---|---|
| **due invocazioni invece di una** | e due liste da tenere allineate nello stesso file |
| **il grafo di build fa fallire la build** | un aggiornamento di `bincode` che porti un generatore nuovo ferma il lavoro finché non è valutato. È il costo che ADR-0031 aveva dichiarato, e qui si comincia a pagarlo |
| `cargo tree` resta un'interfaccia per umani | un cambio di formato rompe entrambi i controlli in una volta sola |

#### 7.3.2 Il cancello senza OS: si aggiunge, e cambia bersaglio

> **Decisione.** Il cancello entra fra i controlli automatici **accanto alla lista, non al
> suo posto**. Il bersaglio è **`x86_64-unknown-none`**.

**Perché si aggiunge e non sostituisce.** «Necessario ma non sufficiente» spiega solo perché
non *può* sostituire. La ragione per avere entrambi è che **falliscono in modo
complementare**:

| | La lista per nome | Il cancello senza OS |
|---|---|---|
| Come decide | **enumera** | **prova** |
| Coglie | una crate **nuova** che entra, anche innocua | una crate **già in lista** che raggiunge l'OS per una via non prevista — l'unificazione delle feature |
| Messaggio d'errore | ✅ **nomina il rimbalzo**: `X unty <- kernel -> bincode -> unty` | ❌ `target is not supported` — **non dice chi l'ha tirata dentro** |

> **La lista è la diagnosi, il cancello è la prova.** Sostituire la prima con il secondo
> lascia un controllo che dice «no» senza dire perché.

**Perché il bersaglio cambia.** Il criterio era finora implicito, e va scritto:

> **Il bersaglio del cancello deve differire da quello reale in una sola dimensione:
> l'assenza del sistema operativo.** Ogni altra differenza è una sorgente di rossi per il
> motivo sbagliato — il gotcha #9 applicato al bersaglio invece che al test.

| Bersaglio | arch | OS | puntatore | atomici a 64 bit | **scarto dal reale** |
|---|---|---|---|---|---|
| reale — `x86_64-pc-windows-msvc` | x86_64 | windows | 64 | sì | — |
| `thumbv7em-none-eabihf` | **arm** | none | **32** | **no** | **quattro dimensioni** |
| **`x86_64-unknown-none`** | x86_64 | none | 64 | sì | **una** |

**Le sonde, rieseguite il 2026-08-07 su entrambi i bersagli:**

| # | Sonda | `thumbv7em` | **`x86_64-none`** |
|---|---|---|---|
| **B1** | `kernel` e `simulator` compilano | ✅ | ✅ |
| **B2** | `getrandom` in `kernel` | ✅ `target is not supported` | ✅ **stesso messaggio** |
| **B3** | **contro-sonda**: il cancello non si applica a `platform` | — | ✅ con `--workspace` fallisce su `platform` con `can't find crate for std` — **motivo giusto, crate sbagliata** |

**B3 è la sonda che non esisteva.** Chi «migliorasse» il comando aggiungendo `--workspace`
otterrebbe un rosso legittimo sulla crate che **deve** toccare l'OS. Il comando nomina le due
crate vincolate, e non è un dettaglio di comodità.

**I costi:**

| Costo | |
|---|---|
| il bersaglio va **installato** sulla macchina | `rustup target add x86_64-unknown-none` diventa un prerequisito dell'ambiente, o la porta è rossa su una macchina pulita e per il motivo sbagliato |
| **innesco di smantellamento** | se un giorno il kernel avesse bisogno legittimo di qualcosa che non compila senza OS, il cancello diventa un ostacolo. Va tolto con un **ADR**, non con un commento — stessa postura della lista |
| il cancello dice «no» senza dire chi | mitigato dalla lista, che resta accanto. Da soli, nessuno dei due basta |

### 7.4 Il catalogo dei controlli

#### 7.4.0 Come si legge

Il catalogo è organizzato per **livello di forza** (§7.1.2), non per argomento: è la
proprietà che dice quanto vale ogni riga, e raggrupparlo altrimenti la nasconderebbe.

Ogni voce porta **due** sonde, per la regola 3 del criterio di ammissione: quella che
**deve scattare** e quella che **deve restare verde**. Una voce con una sola sonda è
dichiarata incompleta, non tenuta per buona.

⚠️ **La colonna «Difende» ha due specie, e vanno distinte a occhio.** La maggior parte delle
righe nomina un `V`, un'`I` o un `Q`: sono il **ramo 1a** della §7.1.1. Tre righe portano
invece il prefisso **`1b`** e nominano un pezzo del catalogo: sostengono la **validità del
verdetto** di altre righe, non una proprietà del sistema. Cancellarne una non rende rossa
nessuna riga — le rende **meno vere**, ed è per questo che il ramo esiste. ⛔ La colonna
**non è sempre la prima**: nel blocco B dei gettoni è la terza, e chi scrive un controllo su
questa tabella la cerca per **intestazione**, mai per posizione (trappola 3 di
`check-docs.sh`).

Costruendo il catalogo **tre voci si sono ridotte invece di crescere**, e una si è
scaglionata. Sono in §7.4.4 e §7.4.5: sono l'esito più utile di questa sezione, perché una
porta di qualità che cresce a ogni revisione smette di essere letta.

#### 7.4.1 Livello 1 — il compilatore

**A · Attributi delle crate.**

| Difende | Meccanismo | Sonda — *deve scattare* | Contro-sonda — *deve restare verde* |
|---|---|---|---|
| I3 · V28 · V29 | `#![no_std]` su `kernel` e `simulator` | `std::fs` nel kernel → `E0433` | `platform` nomina `std::fs` e **compila** |
| **1b** · validità di §7.4.1 A · B · C | `#![forbid(unsafe_code)]` sulle stesse | `unsafe {}` → errore · un `#[allow]` locale → `E0453` | ⚠️ `platform` usa `unsafe` per la FFI e **compila** |
| V29 · gotcha #12 | `HashMap` non nominabile — conseguenza gratuita di `no_std` | `use std::collections::HashMap` → `E0433` | `BTreeMap` compila |

⚠️ **La contro-sonda sul `forbid` non è teorica.** Cargo permette di dichiarare i divieti a
livello di workspace e farli ereditare da tutte le crate. Fatto così, `platform` smette di
compilare: non può parlare con l'OS senza `unsafe`. La contro-sonda è ciò che intercetta
quella «semplificazione» prima che qualcuno la applichi.

**B · I gettoni — il dispositivo di §6.3.**

| Per fare questo… | …va consegnato questo | Difende | Sonda | Contro-sonda |
|---|---|---|---|---|
| avviare un worker | una **concessione** | **I2** | senza → non compila | con → compila |
| eseguire una richiesta | una **prova di conformità** | **Q13** | candidato non filtrato → non compila | filtrato → compila |
| promuovere testo a istruzione | la porta **`journal`** | **V19** | conversione libera → non compila | conversione giornalata → compila |
| **parlare** a un worker | l'oggetto **`Worker`** che l'avvio ha restituito | **I2** | parlargli senza `Worker` → non compila | col `Worker` → compila |
| **leggere** da un worker | una **ricevuta** | **I5** · **Q4** | leggere senza ricevuta → non compila | con la ricevuta → compila |

**C · Cosa non è esprimibile.**

| Difende | Cosa **non** deve compilare | Contro-sonda |
|---|---|---|
| **Q9** · I6 · V20 | `Untrusted` assegnato a `Instruction` — **regola A** | la promozione dichiarata compila |
| **Q9** · I6 · V20 | una **via di conversione `From`/`Into`** da `Untrusted` a `Instruction` — **regola B**. La direzione è **una sola**, e il richiamo qui sotto dice perché | la promozione dichiarata, che pretende la porta `journal`, compila |
| **Q2** · §5.1 | MiB assegnati a millisecondi | ciascuno con sé stesso |
| **V29** · §2.1 | tempo monotonic assegnato a wall time, **e wall time assegnato a un istante di decisione** — la §2.1 dice *«scambiarli non compila»*, che è simmetrico | ciascuno accettato dal proprio |
| **V29** · §2.1 | una **via di conversione `From`/`Into`** fra i due tempi | i due accessori, nominati entrambi esplicitamente, compilano |
| **V29** · §2.2 | una **riduzione propria** di `below`: vive su un tratto d'estensione con impl a tappeto, e un impl scritto a mano collide | usare la riduzione del kernel compila |
| **I2** · §5.3 | `InRevoca` per un profilo non prelazionabile | costruibile per uno prelazionabile |
| **Q8** · §5.2.1 | l'ammissione legge `cold_start` | la proiezione di presentazione lo legge |
| **V5** | un effetto **senza classe dichiarata** — §7.4.4 | un effetto con la classe compila |
| **V2** | un'ammissione **senza profilo di risorsa** — §5.2.1 dice *«il profilo che l'arbitro riceve»* | con il profilo dichiarato compila |
| **V4** | trattare l'esito dell'arbitro come **due vie** invece di tre — §5.3 punto 1 | distinguere `Concessa`, `Rifiutata` e `InCoda` compila |
| **V10** | un sensore che **modifica** l'artefatto — §6.4.2 lo consegna per riferimento immutabile | osservarlo e restituire un verdetto compila |
| **V29** · §2.8 · ADR-0034 | costruire una decisione **senza i parametri consegnati** — §2.8.2 | riceverli alla costruzione compila |
| **V29** · §2.8 · ADR-0034 | il kernel che **nomina un default**: `Parameters::default()` non esiste — §2.8.2 regola 2 | costruirli con `new`, consegnando ogni campo, compila |
| **V3** | una **seconda policy attiva**: il valore consegnato ne porta una sola | con una policy sola compila, e la transizione resta un passo giornalato (§5.4) |
| **Q14** · §4.9 | un **record durevole senza versione**: il tipo è un enum di versione — §4.9.2 regola 1 | il record che dichiara la propria versione compila |
| **I2** · §6.10 | **istruire un worker dopo `uccidi`**: l'uccisione **consuma** il `Worker` — §6.10.2 | istruirlo prima dell'uccisione compila |
| **I5** · §6.10 | **leggere due volte dalla stessa ricevuta singola**: la lettura la consuma — §6.10.2 | leggerne una compila |
| **Q9** · I6 · V20 · §4.9 | un **payload non fidato scritto senza la propria etichetta**: il campo esiste e non ha default — **regola D4 del piano del Traguardo 3** | un record che dichiara la propria etichetta compila, in entrambi i valori |

> ⛔ **Una riga aggiunta il 2026-08-10, eseguendo il Traguardo 3 — ed è un controllo _nuovo_.**
> Chiude la via **A4** di `crates/kernel/src/boundary.rs`: scrivere testo esterno nel giornale,
> rileggerlo come byte grezzi e ricostruirne un'istruzione. ⛔ **I byte non portano etichette**,
> quindi finché il record non ne ha una il giro **declassa il sospetto in silenzio** — e
> `boundary.rs` ne aveva già scritto il prezzo: *«retrofitted later only by migrating the one
> irreproducible archive»*.
>
> ⚠️ **Cosa compra e cosa no, detto prima che qualcuno lo scopra.** Compra che un lettore non
> possa più **perdere** la distinzione: ciò che risale dalla decodifica di un payload marcato
> è `Untrusted`, non una stringa. **Non** compra che chi scrive etichetti bene — è il limite
> del gettone di §6.3.2, *prova la provenienza, non l'esattezza*.
>
> ⚠️ **Perché entra qui e non solo nel registro:** §8.1.2 ammette come «controllo» solo ciò che
> il catalogo elenca, ed è il gotcha **#36**, che è già successo **tre volte** nello stesso modo.
>
> ⛔ **Due casi per una riga sola, perché le metà sono due** — il piano ne prevedeva zero, e il
> vincolo globale 6 ne pretende. `record_without_trust_label.rs` tiene *«il campo esiste»*
> (`E0063`); `trust_has_no_default.rs` tiene *«e non ha default»* (`E0277`). Entrambi scattano
> come **`error`** e non come `mismatch`, quindi **nessuna rigenerazione in blocco li spegne**.
> Sono complementari e nessuno è di troppo: misurato che un `impl Default for Trust` fa rosso
> **solo** il secondo, e togliere il campo **solo** il primo. La coppia ha il precedente locale
> in `monotonic_as_wall`/`wall_as_monotonic`, quindi **non è una riga in più**. La contro-sonda
> esisteva già: `every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes` di
> `crates/kernel/tests/record_shape.rs`. ⚠️ **L'argomento per intero — la ricetta a due righe,
> e ciò che nessuno dei due casi copre — sta in `trust_has_no_default.rs`**, dove serve a chi ci
> sbatte contro, e non è ripetuto qui.
>
> 📌 **Conteggi ricontati sulla tabella, non dedotti** — gotcha #31: il blocco C passa da
> diciotto a **diciannove** righe, e i test del catalogo da ventitré a **ventiquattro**. La
> §7.4.7 è aggiornata nello stesso passaggio. ⚠️ **E il numeratore del registro era già stantio
> prima di questa riga** — otto, non sette — perché il Task 1 aveva consegnato un caso senza
> scriverne la riga: la divergenza, coi comandi, sta in [`porta-di-qualita.md`](../../porta-di-qualita.md).

> ⛔ **Quattro righe toccate il 2026-08-09, eseguendo il Traguardo 2 — e tre sono controlli
> _nuovi_.** È la differenza dalle note qui sotto, e va detta invece che confusa.
>
> | | |
> |---|---|
> | la riga `V29 · §2.1` è **allargata** | diceva una direzione sola. Misurato: con quel solo caso, aggiungere `impl From<WallTime> for Monotonic` lasciava la porta **verde su sei controlli su sei** — e quella è la direzione **pericolosa**, la decisione che dipende dal wall time. La §2.1 diceva già *«scambiarli non compila»*, simmetrico: il catalogo era più stretto della sezione |
> | **la seconda riga `V29 · §2.1` è nuova** | le regole erano **due** e il catalogo ne registrava una: *«non si passa l'uno per l'altro»* e *«non esiste una via `From`/`Into`»*. La seconda era scritta **in un commento del sorgente**, cioè era un'intenzione |
> | **la riga `V29 · §2.2` è nuova** | `below` viveva come metodo di default, e un metodo di default **si sovrascrive**. Due implementazioni che riducono in modo diverso producono tracce diverse dallo stesso seme, **invisibilmente**. Ora vive su un tratto d'estensione con impl a tappeto, e la collisione è `E0119` |
> | **la seconda riga `V29 · §2.8` è nuova** | la §2.8.2 regola 2 dice che il kernel **non può nominare un default**, e finora lo diceva **soltanto**: nulla impediva a un commit successivo di scrivere `impl Default for Parameters`, e nel momento in cui esiste la regola muore **senza che nulla diventi rosso**. Misurato: con quell'impl la porta resta **verde su sei su sei** — `gate-attributes` legge attributi, `gate-deps` legge il grafo, il cancello senza OS compila per un bersaglio nudo, `check-docs` non legge il codice, e `build`/`test` lo compilano perché è Rust valido. ⚠️ La guardia è **larga esattamente quanto la regola**: ogni `impl Default` incorpora per forza un valore scelto dentro il kernel |
>
> 📌 **E ne esce un test generale, che è il gotcha #42:** `trybuild` distingue nel proprio
> output **`error`** (il caso ha compilato) da **`mismatch`** (l'uscita non combacia con
> l'oracolo). Le due righe nuove scattano come **`error`**, quindi non poggiano sul proprio
> oracolo; la riga allargata scatta come `mismatch` e vi poggia. Una regola guardata **solo**
> da casi `mismatch` è una regola che una rigenerazione in blocco spegne in silenzio.
>
> ⚠️ **Perché entrano qui e non solo nel registro:** §8.1.2 ammette come «controllo» solo ciò
> che il catalogo elenca, e il gotcha **#36** è successo due volte proprio così — una sezione
> decide un meccanismo, lo scrive nella propria tabella, e il catalogo resta indietro.

> ⛔ **Una riga aggiunta il 2026-08-09, chiudendo la voce che il Traguardo 2 aveva lasciato
> aperta: è la _regola B_ della coppia `Untrusted`/`Instruction`.** Il caso esiste dal Task 9 —
> `no_conversion_from_untrusted_to_instruction.rs` — e la riga no. È il gotcha **#36** alla
> **terza** occorrenza, e la prima colta **prima** che il catalogo si sedimentasse: il registro
> [`porta-di-qualita.md`](../../porta-di-qualita.md) ha portato il caso come *implementato e non
> coperto dal catalogo* invece che nel silenzio, ed è quello che ha reso la voce visibile.
>
> ⛔ **E non è una rifinitura della regola A: quella guardia è cieca proprio a questa via, ed è
> misurato.** Il gotcha #42 prevede un `mismatch` — rustc che appende righe di `help: call
> Into::into` che l'oracolo non porta. **Su questa coppia non succede.** Con
> `impl From<Untrusted> for Instruction` presente, `untrusted_as_instruction.rs` resta **`ok`**:
> lì lo scarto è fra **riferimenti** (`&Untrusted` contro `&Instruction`), e quell'impl non
> produce nessun `&Untrusted: Into<&Instruction>`, quindi rustc non ha suggerimenti da
> appendere. Sui due tempi lo scarto è fra **valori posseduti**, e il suggerimento compare.
> Quella guardia non è «disarmabile da una rigenerazione»: è **cieca dall'inizio**, e senza la
> riga B l'`impl From` lascia la porta **verde col confine già caduto**. Il caso B scatta come
> **`error`**, che nessuna rigenerazione di oracoli disarma.
>
> 📌 **Rimisurato il 2026-08-09, _prima_ di scrivere questa riga e non dopo.** Aggiunto
> `impl From<Untrusted> for Instruction` in `crates/kernel/src/boundary.rs` e lanciato
> `cargo test -p kernel --test compile_fail`: `untrusted_as_instruction.rs` → **`ok`**,
> `no_conversion_from_untrusted_to_instruction.rs` → **`error`**. L'esito era già registrato dal
> Task 9; è stato rifatto perché un'evidenza **ereditata** che sta per entrare nel catalogo resta
> un'ipotesi finché qualcuno non la rilancia — gotcha #15.
>
> ⚠️ **Una direzione sola qui, due per i due tempi — scritto perché non venga «sanato».**
> `Untrusted → Instruction` è la direzione **pericolosa**: promuove contenuto esterno nel canale
> delle istruzioni, che è esattamente ciò che I6 vieta, e l'unica strada ammessa pretende la
> porta `journal` (blocco B, `V19`). `Instruction → Untrusted` è un **declassamento**: può solo
> aggiungere sospetto, mai toglierlo, quindi non può violare I6 e non ha bisogno di guardia. Fra
> `Monotonic` e `WallTime` **nessuno dei due è più stretto dell'altro** — un termine di parete e
> un timbro monotono sbagliano ciascuno a modo proprio — ed è per questo che lì le direzioni
> sono due e qui una. L'asimmetria è **misurata, non accidentale**.
>
> 📌 **Conteggi ricontati sulla tabella, non dedotti** — gotcha #31: il blocco C passa da
> diciassette a **diciotto** righe, e i test di compilazione fallita del catalogo da ventidue a
> **ventitré**. La §7.4.7 è aggiornata nello stesso passaggio.

> ⚠️ **Due righe aggiunte il 2026-08-08, e non sono controlli nuovi.** Sono i controlli 2 e
> 4 della §6.10.5, decisi con F1b e
> [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md), che il catalogo
> **non enumerava**. Li ha trovati la §8, con la stessa regola §8.1.2 e nella stessa forma di
> §8.5.3 — registrato in **§8.5.4**. I controlli 1 e 3 della stessa tabella sono **gettoni** e
> stanno nel blocco B; il controllo 5 è di livello 2 e sta in §7.4.2.

> ⚠️ **Una riga aggiunta il 2026-08-07 con [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md).**
> È il controllo di livello 1 della §4.9.4. ⛔ Vale il limite dichiarato lì: il compilatore
> prova che un record **dichiara** una versione, non che sia quella **giusta** — limite del
> gettone, §6.3.2. E la regola «un indice non si riusa mai» **non** è di questo livello:
> vive in §7.4.2, ed è di livello 2.

> ⚠️ **Due righe aggiunte il 2026-08-07 con ADR-0034.** La prima è il controllo di livello
> 1 della §2.8.4; la seconda esisteva già come *comportamento* — la §5.4 verificava
> l'unicità della policy con un test a esempi — e sale al compilatore perché il valore
> consegnato ne porta una sola. ⛔ Vale per entrambe il limite dichiarato in §2.8.4: il
> compilatore prova che una decisione **riceve** i propri parametri, **non** che non ne
> abbia altri scritti dentro come costanti.

> ⚠️ **Tre righe aggiunte il 2026-08-07, e il titolo del blocco corretto.** Le trovò la
> copertura della §8: V2, V4 e V10 sono proprietà **di livello 1** decise nelle §5 e §6 —
> non nuove — che il catalogo non enumerava, quindi la §8 non poteva nominarle senza
> violare la propria regola §8.1.2. Registrato in §8.5.3.
>
> Il titolo diceva «Tipi che non si scambiano», ma **tre delle sei righe originali non erano
> scambi di tipo** — `InRevoca`, `cold_start` e la classe dell'effetto. Il titolo era
> stretto rispetto a ciò che il blocco già conteneva; la colonna, che dice «cosa **non**
> deve compilare», era invece giusta fin dall'inizio.

Tutte le righe dei blocchi B e C sono **test di compilazione fallita**, quindi valgono per
loro la forza di livello 1 e la visibilità di livello 2 (§7.1.3), e il gotcha #25 (§7.1.4).

#### 7.4.2 Livello 2 — controlli esterni

| Difende | Meccanismo | Sonda | Contro-sonda |
|---|---|---|---|
| I3 · **V28** | allow-list, grafo **spedito** (§7.3.1) | N1 · N2 · N3 | **N4** |
| **1b** · validità di §7.4.1 | allow-list, grafo **di build** (§7.3.1) | voce nuova non in lista | voce in lista resta verde |
| I3 | cancello senza OS su `x86_64-unknown-none` (§7.3.2) | **B2** | **B3** |
| V34 · Q24 | solo `secrets` raggiunge il portachiavi | ⚠️ **non esiste** — vedi sotto | ⚠️ **non esiste** — vedi sotto |
| V25 · Q20 | un solo punto di uscita verso la rete | chiamata di rete in `daemon` → scatta | ⚠️ **non esiste ancora** — vedi sotto |
| Q2 · Q3 · Q4 · Q5 · Q18 · Q22 · I1 · I2 · I5 · V1 · V6 | **la campagna DST** (§3.5) | si rompe l'ammissione: la campagna fallisce e **nomina il seme** (§5.7.1) | senza guasto iniettato, **nessun passo in dubbio** — misurato, C7a di M-2 |
| **1b** · validità di §7.4.2, riga della campagna DST | **test di contratto** — §7.4.6 | il doppio diverge dall'implementazione reale → scatta | i due concordi → verde |
| V30 | `check-docs.sh` | un Q senza metodo di verifica → scatta | già in esercizio |
| **Q14** · §4.9 | **byte congelati** del record durevole, con la mappa `indice → nome del campo → valore atteso` (§4.9.4) | si **riusa** un indice o si rinumera → fallisce e **nomina il campo** | si aggiunge un campo facoltativo con un indice nuovo → resta verde |
| V31 | il **seme** entra nell'elenco versionato, la **proprietà** entra nella suite | si reintroduce il difetto che quella proprietà proteggeva → la campagna fallisce e **nomina il seme** | il difetto corretto → la campagna resta verde, e l'elenco dei semi non produce falsi rossi |
| **Q4** · I5 · §6.10 | sul canale verso i worker, i **byte consumati** dalla decodifica sono pari alla **lunghezza dichiarata** dal frame (§6.10.4) | frame troncato, o con una coda dopo l'ultimo elemento → fallisce | frame esatto → resta verde |
| **1b** · validità di §7.4.1 A · B · C | le crate vincolate **dichiarano davvero** i propri attributi — `scripts/gate-attributes.sh` | `#![forbid(unsafe_code)]` tolto, oppure `#![deny(unsafe_code)]` al suo posto → scatta e nomina file e attributo | `platform`, `secrets` e `daemon` non ne dichiarano nessuno e **restano verdi** |
| I3 · **V29** | le crate vincolate **non hanno un build script** — `scripts/gate-attributes.sh` | `crates/kernel/build.rs` o `crates/simulator/build.rs` → scatta e **nomina il file** · `build = "gen.rs"` nel manifesto → scatta | `crates/platform/build.rs` **resta verde** · `build = false`, che il build script lo **disattiva**, resta verde |

> ⛔ **La riga degli attributi è aggiunta il 2026-08-08, e la lacuna che chiude era stata
> misurata eseguendo il Traguardo 1.** Senza di lei si poteva togliere
> `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs`, **scrivere `unsafe` vero nel
> kernel**, e la porta restava **verde su cinque controlli su cinque**.
>
> ⚠️ **Perché era più grave di quanto sembri, e lo dice la sua stessa colonna «Difende».** I
> quattro casi di `tests/compile_fail/` **ridichiarano ciascuno i propri attributi** e non
> nominano mai `kernel::`: provano che il meccanismo **morde dove è dichiarato**, non che sia
> dichiarato nel kernel. E la riga di `forbid` in §7.4.1 A è di **ramo 1b** — sostiene la
> validità dei blocchi A, B e C — quindi toglierla in silenzio non spegneva *una* regola:
> invalidava il **fondamento del livello 1**, e nessun rosso lo diceva.
>
> 📌 **È un controllo di testo, non del compilatore, e il registro non deve promuoverlo:**
> prova che il divieto sia **dichiarato**, non che nel kernel non ci sia `unsafe`. Quella la
> prova il compilatore, ed è proprio ciò che questa riga tiene in piedi.
>
> ⛔ **La sonda che la distingue da un controllo ingenuo è la terza:** `#![deny(unsafe_code)]`
> al posto di `forbid` deve **scattare**. Un `grep -q 'unsafe_code'` resterebbe **verde** —
> misurato — e il vincolo 2 della §11 tornerebbe una preferenza stilistica, perché `deny` è
> scavalcabile da un `#[allow]` locale mentre `forbid` no. **Costo dichiarato:** un attributo
> sepolto in un commento di blocco `/* ... */` sfugge ancora; l'ancora a inizio riga chiude il
> caso `//`, che è quello reale, e chiudere anche l'altro richiederebbe un parser — un rimedio
> più fragile del buco.

> ⛔ **La riga del build script è aggiunta il 2026-08-09, e la lacuna è stata _misurata_, non
> temuta.** Un `crates/kernel/build.rs` che chiama `std::time::SystemTime::now()`,
> `std::fs::metadata()` e `std::env::var()` e inietta il risultato con `cargo:rustc-env` ha
> lasciato la porta **verde su sei controlli su sei**. Ciascuno manca il bersaglio per una
> ragione propria:
>
> | Controllo | Perché non lo vede |
> |---|---|
> | `cargo build` · `cargo test` | un build script è un target **separato**, compilato **per l'host**: usare `std` lì è il suo mestiere |
> | `gate-no-os.sh` | i build script si compilano per l'host **anche** con `--target`. Non solo non lo coglie: **lo esegue** — verificato leggendo `target/debug/build/kernel-*/output` |
> | `gate-deps.sh` | legge il **grafo**, e uno script senza dipendenze proprie non aggiunge nodi. ⚠️ Una `[build-dependencies]` **verrebbe** colta: l'invisibile è lo script *senza* dipendenze |
> | `gate-attributes.sh` | leggeva solo `src/lib.rs`. `build.rs` ha attributi propri, e il `forbid` di `lib.rs` non lo raggiunge — per questo il controllo nuovo vive **lì**: era il suo punto cieco |
> | `check-docs.sh` | non guarda il codice |
>
> ⛔ **Perché la cella «Difende» dice `I3` · `V29` e non `1b`.** Non sostiene la validità di
> un'altra riga: difende **due invarianti direttamente**. `I3` perché sono chiamate all'OS
> dentro la crate del kernel; `V29` perché `cargo:rustc-env` più `env!()` **cuoce nel kernel**
> un valore letto dal mondo al momento della build. È il **gotcha #28** alla lettera — *un
> parametro non consegnato è una costante, e una costante è invisibile* — nella forma più
> difficile da vedere, perché il valore non compare in nessuna firma.
>
> ⛔ **Il rimedio è TOGLIERE, non elencare.** È il rimedio del grafo **spedito** di §7.3.1, non
> quello del grafo di build: un build script nel kernel non si giustifica e non si aggiunge a
> nessuna lista. Il messaggio dello script lo dice, e dice anche **perché** — altrimenti sembra
> pedanteria e la prima persona che ha fretta lo aggira.
>
> ⚠️ **Il perimetro è `kernel` e `simulator` soltanto**, e la directory è **derivata** dalla
> lista dei file vincolati, così non nasce un secondo posto da tenere allineato. `platform`,
> `secrets` e `daemon` **possono** avere un build script: è il posto dove l'I/O deve vivere, e
> un controllo che scattasse anche lì sarebbe rosso per il motivo sbagliato — gotcha #24. È la
> contro-sonda della riga, ed è la direzione che si dimentica.
>
> 📌 **Due vie, non una.** Rinominare `build.rs` in `gen.rs` e dichiarare `build = "gen.rs"` nel
> manifesto è lo stesso oggetto sotto altro nome, e il solo test di esistenza del file non lo
> vedrebbe. Le **virgolette** nel motivo distinguono quel caso da `build = false`, che il build
> script lo **disattiva** e deve restare verde. ⚠️ **Che un manifesto solo basti è misurato, non
> supposto:** `build` **non** è fra le chiavi che `[workspace.package]` può passare in eredità —
> provato su `cargo` 1.95.0, `build.workspace = true` è rifiutato in fase di parsing con
> *«invalid type: map, expected a boolean, string or array»*. Il manifesto della crate è quindi
> l'unico posto da guardare.

> ⚠️ **La riga dei byte consumati è aggiunta il 2026-08-08, e non è un controllo nuovo.** È
> il controllo 5 della §6.10.5, deciso con F1b e
> [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md), che il catalogo
> non enumerava — §8.5.4. Esiste perché la misura ha mostrato che **«ha decodificato» non
> prova nulla**: dando a `cbor2` i byte di `bincode` si ottiene `1`, senza nessuna eccezione
> e con un valore plausibile. È il **gotcha #34**, e su un canale a frame la sola difesa è
> contare i byte.

> ⚠️ **V31 resta debole per natura, e la riga qui sopra non lo nasconde.** Ciò che
> l'automatismo protegge è la **proprietà**, non il seme: §3.4 — un seme non riproduce la
> stessa esecuzione dopo un cambio di codice. Sonda e contro-sonda valgono quindi per la
> proprietà; il seme resta un punto di ripartenza per indagare, non un oracolo.
>
> 📌 La prima stesura di questa riga aveva **tre celle su quattro**: il testo «debole per
> natura» occupava la colonna *Sonda* e la contro-sonda non esisteva. Era l'unica riga del
> catalogo a violare la §7.1.1 regola 3, e l'ha trovata il controllo scritto in §8.6 — alla
> sua prima corsa, prima ancora che la §8 fosse scritta.

> ⚠️ **La riga di V25 ha un buco, e va dichiarato invece che nascosto.** La lista delle
> crate autorizzate a uscire in rete è **vuota**, e una lista vuota passa sempre. La sonda
> esiste — una chiamata di rete deliberata in `daemon` la accende, come §1.4.1 prescrive —
> ma **la contro-sonda no**: non c'è ancora niente di legittimo da lasciar passare.
>
> È quindi l'unica voce del catalogo **provata in una direzione sola**. Si completa nel
> sotto-progetto che accende la rete, e fino ad allora la §8 la registra come tale.

> ⛔ **E LA RIGA DI V34 · Q24 NON HA NÉ SONDA NÉ CONTRO-SONDA — dichiarato il 2026-08-27,
> finding AUD-026 del secondo audit.** Le due celle promettevano *«il portachiavi nel grafo
> di `platform` → scatta»* e *«`secrets` resta verde»*, e **nessuna delle due può scattare**:
> `scripts/gate-deps.sh` cicla su `for crate in kernel simulator` e non misura mai il grafo
> di `platform`; e nessuna crate di portachiavi esiste — un `grep -rni` su `scripts/`,
> `.github/` e `crates/` per *keyring*, *portachiavi* e *credential* restituisce **due righe
> di commento** in `crates/secrets/src/lib.rs` e nient'altro (misurato il 2026-08-27).
>
> ⚠️ **La contro-sonda è vacua per la ragione di §8.5.3.1, non per costo:** senza nessuna
> credenziale nel perimetro, un controllo proverebbe **l'assenza di una cosa che non c'è** —
> gotcha #17. È la stessa lettura che ha declassato **V16**, che parla della stessa sostanza,
> e per questo **V34**, **Q24** e **Q17** passano a ⏳ **rimandato** con innesco **B**.
>
> 📌 **Che le crate siano cinque e non quattro resta una scelta di scrittura giusta**
> (§1.2) — ma una scelta di scrittura non è un controllo, ed è esattamente la distinzione
> che la §7.0 esiste per fare. ⛔ **E gli altri due registri lo dicevano già:**
> [`porta-di-qualita.md`](../../porta-di-qualita.md) conta il portachiavi fra le righe
> **scoperte**, e la §6 del compendio lo porta fra le questioni **da assegnare**. A mentire
> era questa tabella, sola contro due: è la radice **R1** dell'audit del 2026-08-27.

> ⚠️ **La riga dei byte congelati è aggiunta il 2026-08-07 con
> [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md), e porta con sé
> la regola che le impedisce di diventare una tautologia.** L'oracolo è un file generato dal
> codice che il controllo verifica: **rigenerarlo in blocco lo cancella**. Quindi i byte
> congelati **non si rigenerano** — se cambiano non è un aggiornamento, è un cambio di
> formato, e va aperta una versione nuova. È il **gotcha #25** trasferito dagli `.stderr` di
> `trybuild` a questo file, e vale identico: la difesa non è che sia impossibile, è che **si
> legge nel diff**.

#### 7.4.3 Livello 3 — vuoto, e non è una svista

**Nessuna voce del catalogo è di livello 3.** Ogni invariante del kernel in perimetro è
difesa dal compilatore o da un controllo esterno; nessuna da un lint.

`clippy` continua a girare come igiene del codice, ma **non ha voce nella porta**: nessun V
dipende da lui, e la regola 1 del criterio di ammissione (§7.1.1) dice che allora non entra.
Distinguere l'igiene dalla porta tiene il significato della porta affilato — un rosso della
porta è sempre un'invariante violata, mai uno stile discutibile.

#### 7.4.4 Le tre voci che il catalogo ha ridotto invece di aggiungere

**1 · Il divieto di `HashMap` fuori dal kernel si toglie, perché non difende nulla.**

La §1.4 lo elencava come regola di V29, con forza di lint dichiarata. Costruendo il catalogo
è emerso che il vincolo non regge, per due motivi indipendenti:

| | |
|---|---|
| **una porta ordinata non basta** | una porta può restituire un `Vec` — ordinato come *tipo* — costruito però scorrendo una `HashMap`. L'ordine del **contenuto** resta irriproducibile: il tipo non porta la garanzia |
| **e non serve** | in una corsa DST `platform` **non gira affatto**. Il simulatore sostituisce *tutte* le porte (§3.1), quindi ciò che `platform` fa al proprio interno non può rendere irriproducibile una simulazione |

V29 chiede che tempo, casualità, I/O e scheduling siano **sostituibili**, e lo sono. Non
chiede che `platform` sia deterministico in produzione, e non lo è mai stato.

Tenere la regola significherebbe avere un controllo che non protegge niente e che scatta su
codice legittimo: gotcha #24 senza contropartita. **Si toglie**, e §1.4 riceve un rimando.

Costo dichiarato: in produzione l'ordine interno di `platform` non è riproducibile. Se un
giorno servisse — per esempio per un test di contratto instabile — la risposta è rendere
deterministico **quel** punto, non vietare un tipo ovunque.

> ⚠️ **Conseguenza operativa, che la §2.5 non copriva:** il
> [`clippy.toml`](../../../spikes/rust/clippy.toml) di `spikes/rust/` **non sale**. È
> l'evidenza del meccanismo (a) di T6 e resta dov'è, ma nel workspace reale è a livello di
> workspace e scatterebbe addosso a `platform` — che *deve* chiamare l'orologio e il
> filesystem. È lo stesso costo che `RISULTATI.md` ha misurato su un caso reale, quando la
> regola ha bloccato un uso **legittimo** di `Instant::now()` in un test.

**2 · V28 è un corollario dell'allow-list, non un controllo in più.**

La §1.4 prevedeva «grafo delle crate + driver», cioè un controllo da scrivere. Ma `kernel`
**non dipende da nessuna crate del progetto** (§1.2) e la sua allow-list ha **tre** voci
spedite — `bincode`, `unty`, `minicbor`, contate sulla tabella §7.3.1 il 2026-08-08; diceva
«due», ed era il ritratto di prima di ADR-0036: un percorso verso un adattatore di provider comparirebbe nel grafo transitivo e
**farebbe già scattare il controllo della §7.3.1**.

Il catalogo lo registra come riga che **rimanda allo stesso controllo**, senza driver
proprio. Un secondo meccanismo per la stessa proprietà è un secondo posto da tenere
allineato — e il primo che smette di essere aggiornato mente in silenzio.

**3 · V5 sale dal comportamento al compilatore.**

[ADR-0007](../../adr/0007-giornale-write-ahead-e-riconciliazione.md) dice che un effetto
senza classe dichiarata vale `irripetibile`. Oggi è una regola di comportamento.

> **La classe è un campo obbligatorio del tipo dell'effetto.** «Un effetto senza classe»
> non è esprimibile, quindi non compila.

Non contraddice ADR-0007: il default `irripetibile` resta dov'è davvero utile — sui record
riletti da un giornale scritto prima che la classe esistesse. Sposta la difesa dove il
rischio è la dimenticanza di chi scrive, e la lascia dov'è il rischio di un dato vecchio.

> ✅ **Rimando — questo punto presupponeva l'evoluzione, e ora la eredita (2026-08-07).**
> «Un record scritto prima che la classe esistesse» era una regola di **lettura in avanti**
> per **un campo solo**, arrivata di straforo in una sezione che non aveva deciso nulla del
> genere. Con la §4.9 e
> [ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md) quella riga
> **discende** dalla regola generale invece di essere un'eccezione non dichiarata: un campo
> assente in una versione precedente è il caso ordinario, non un caso speciale della classe
> di effetto. Il testo sopra resta com'era, perché era corretto quando è stato scritto.

#### 7.4.5 Il quarto gettone si scaglia, e l'innesco si scrive

> ⚠️ **«Quarto» qui è un nome, non una posizione — precisato il 2026-08-08.** Quando questa
> sezione è stata scritta gli usi del dispositivo erano tre, e quello del confinamento
> sarebbe stato il quarto. Con F1b il catalogo ne ha registrati **due** — l'oggetto `Worker`
> che l'avvio restituisce, e la **ricevuta** — quindi il blocco B ne conta **cinque** e
> questo sarebbe il **sesto**. ⚠️ Il conteggio che vale è quello del **catalogo**, non quello
> di §6.10.1: lì «quarto uso» è vero rispetto alla tabella di §6.3.1, che ha tre righe e non
> registra l'oggetto `Worker`. Un solo registro, ed è §7.4.1 B. **Il titolo resta**: «il quarto
> gettone» è il nome con cui la §8 e [`HANDOFF.md`](../../HANDOFF.md) lo chiamano, e
> rinominarlo romperebbe i rimandi per guadagnare un ordinale. Ciò che conta non cambia:
> **è quello del confinamento, ed è l'unico scaglionato.**

§6.3 nominava il dispositivo del gettone tre volte. Una ricorrenza in più è naturale — *per
eseguire un comando serve la prova del livello di confinamento richiesto* (V35, Q23) — e
[design/08](../../design/08-strategia-di-test.md) chiede già per Q23 una verifica
**statica**, che è esattamente ciò che una firma darebbe.

**Non entra ora, e il motivo è il criterio della §0.3:**

| | |
|---|---|
| il gettone si attacca a una porta | ma **nessuna porta esegue comandi** in questo sotto-progetto: `process` avvia worker, e gli strumenti sono scaglionati per regola C |
| crearne una per ospitarlo | sarebbe costruire per un consumatore che non esiste |
| è retrofittabile? | **sì**: aggiungere un argomento a una firma con zero chiamanti è meccanico. La regola B non si applica, quindi vale la C |

**Cosa entra comunque**, perché §0.4 l'aveva già messo in perimetro: il **tipo** del livello
di confinamento, la sua dichiarazione per azione, e la sua registrazione nel giornale (V37).

> **L'innesco, scritto perché «rimandato» non diventi «dimenticato»:** alla nascita della
> prima porta che esegue un comando, quella porta **prende un livello di confinamento come
> argomento**, e il test di compilazione fallita si scrive lì. La §8 lo registra con il
> sotto-progetto che lo chiude.

#### 7.4.6 I test di contratto — due porte adesso, due dopo

La §3.7 dichiara il punto cieco con parole proprie: **«la finta non è la vera»**. Senza test
di contratto, Q4 e Q5 sono provati contro una finzione — la DST dimostra che il kernel si
riconcilia bene *con il simulatore*.

Entrano per regola A, ma il perimetro si tara da sé: una suite di conformità esiste solo
dove esistono **entrambe** le implementazioni.

| Porta | Implementazione reale in questo sotto-progetto | Suite di conformità |
|---|---|---|
| `journal` | ✅ `redb` in `platform` | ✅ **sì** — e §4.6 ne copre già il livello 2 |
| `reactor` | ✅ l'attesa vera sull'OS | ✅ **sì**, ed è la più importante: la validità della DST poggia lì |
| `process` | ✅ avvio, **dialogo** e uccisione veri | ⚠️ **rimandata**: non esistono worker da avviare (§0.2) — e con il dialogo la suite acquista **un'affermazione in più**, sotto |
| `ipc` | ✅ named pipe | ⚠️ **rimandata**: non esiste una GUI dall'altro capo |
| `filesystem` | ❌ scaglionata (§0.4) | ❌ |
| `network` | ❌ scaglionata | ❌ |

> ⚠️ **L'affermazione in più su `process` — aggiunta il 2026-08-08** con la §6.10. Con il
> solo avvio e la sola uccisione, la conformità riguardava il **ciclo di vita**: il
> processo parte, muore, e la finta si comporta come la vera. Col dialogo riguarda anche il
> **filo** — che i byte prodotti dalla finta siano quelli che un worker vero produrrebbe, e
> che un frame malformato sia **rifiutato allo stesso modo** dalle due. È la parte che
> resta scoperta più a lungo, perché pretende un worker Python vero.

La §8 registra quali porte hanno la suite e quali no, con il sotto-progetto che le chiude.

#### 7.4.7 I costi del catalogo

| Costo | |
|---|---|
| **ogni regola nuova porta due sonde, non una** | e la contro-sonda è la più noiosa da scrivere, perché verifica che *non* succeda niente |
| **i test di compilazione fallita crescono con ogni tipo** | §2.5 lo prevedeva; il catalogo ne conta ormai **ventiquattro** — **cinque** nel blocco B e **diciannove** nel C — e ciascuno ha un `.stderr` da leggere (gotcha #25). ⚠️ **Ricontato una quarta volta il 2026-08-10**, eseguendo il Task 2 del Traguardo 3: diceva «ventitré, cinque e diciotto», ed era il ritratto di prima della riga dell'**etichetta di fiducia**. ⛔ E questa volta il comando è cambiato insieme al numero: si delimita **per intestazione** (`#### 7.4.1` → `#### 7.4.2`) e non per numero di riga, perché un intervallo assoluto che non pesca più nulla darebbe **zero senza sollevare niente** — gotcha #26. I due comandi stanno in [`riferimenti.md`](../../riferimenti.md). ⚠️ **Ricontato sulla tabella il 2026-08-08**: diceva «una dozzina, tre e nove», ed era il ritratto di **prima** di ADR-0034, ADR-0036 e §6.10.5. ⚠️ **Ricontato di nuovo il 2026-08-09**, eseguendo il Traguardo 2: diceva «diciannove, cinque e quattordici», ed era il ritratto di prima delle **tre righe nuove** del blocco C — arrivate una per compito, ai Task 1, 2 e 3. ⚠️ **Ricontato una terza volta il 2026-08-09**, chiudendo la voce della **regola B**: diceva «ventidue, cinque e diciassette», ed era il ritratto di prima che il caso del Task 9 avesse la propria riga. Un ritratto di conteggi si riconta, non si deduce — gotcha #31 |
| **una voce è provata in una direzione sola** | V25, finché la rete non esiste. Dichiarato in §7.4.2 |
| **V31 resta debole per natura** | l'automatismo protegge la proprietà, non il seme: §3.4 |
| **i test di contratto sono lavoro reale** | due suite ora, due rimandate. È il prezzo per non provare Q4 e Q5 contro una finzione |

### 7.5 La cadenza: cosa gira quando

#### 7.5.1 Il livello 1 non ha una cadenza

L'osservazione che accorcia la tabella prima di scriverla: **le voci di livello 1 non
«girano» mai.** Non sono un passo della porta — sono il compilatore. Se il codice compila,
quelle regole valgono, e non esiste un modo di saltarle o di rimandarle a stasera.

È il motivo per cui il livello di forza conta più della cadenza. ⚠️ **Formulazione corretta
il 2026-08-08:** diceva che *«la tabella seguente riguarda solo il livello 2»*, e la sua
prima riga è invece tutta di livello 1. La tabella le porta **entrambi**, ed è giusto così —
ma la prima riga non dice una cadenza: dice che quelle voci **non ne hanno una**, perché
sono il compilatore. Tenerla è ciò che rende visibile la differenza; chiamarla «cadenza» era
l'errore.

| Quando | Cosa gira |
|---|---|
| **a ogni compilazione — cioè: sempre, senza essere un passo** | tutto il **livello 1**: `no_std` · `forbid` · i gettoni · i tipi che non si scambiano |
| **a ogni commit** | allow-list sui due grafi · cancello senza OS · test di compilazione fallita · test a esempi · **campagna DST breve** · test di contratto · `check-docs.sh` |
| **su ciclo lungo** | **campagna DST profonda**: molti più semi, scenari più grandi |

#### 7.5.2 La DST sta a ogni commit, e `design/08` si aggiorna

[design/08](../../design/08-strategia-di-test.md) dice «DST su cicli più lunghi», ed è la
**fonte di verità dichiarata** sulla porta di qualità. Ma la §3.5 ha misurato **25,8 µs**
per una corsa dello scenario minimo: migliaia di semi stanno dentro un secondo.

Non è un capovolgimento ma una precisazione, e la §3.5 ha già la formulazione giusta — *«i
cicli lunghi servono ad andare più a fondo, non a rendere possibile la DST»*. Due cadenze
scritte in due documenti sarebbero **due verità**, quindi `design/08` è aggiornato nello
stesso passaggio di questa sezione.

**Seconda riga aggiornata nello stesso passaggio:** `design/08` assegna i test di contratto
a Q16 e Q21, entrambi fuori dal perimetro di questo sotto-progetto. La §7.4.6 aggiunge un
uso che quel documento non prevedeva — la **conformità fra `platform` e `simulator`** — ed
è quello che chiude il punto cieco dichiarato in §3.7.

#### 7.5.3 Come si dimensiona la campagna breve

Due modi, e la differenza è cosa succede quando gli scenari si appesantiscono.

| | **numero di semi fissato** | budget di tempo |
|---|---|---|
| riproducibilità | ✅ stessa copertura su ogni macchina | ❌ dipende dalla velocità della macchina |
| quando gli scenari si appesantiscono | il commit rallenta, **e si vede** | la copertura **cala in silenzio** |
| rischio residuo | qualcuno abbassa il numero per far tornare veloce il commit | nessuno se ne accorge mai |

> **Il numero di semi della campagna breve è fissato e versionato.** Abbassarlo è una
> modifica che **si legge nel diff**, non un flag di comodità — la stessa postura del
> gotcha #25 sulla rigenerazione degli `.stderr`. Il tempo di parete si stampa a ogni
> corsa, così l'appesantimento diventa visibile **prima** di diventare una tentazione.

Costo dichiarato: la campagna troverà difetti nuovi nei momenti scomodi, perché esplora
semi che nessuno aveva ancora percorso. È il punto della DST, ma va detto — la disciplina è
**registrare il seme e la proprietà** (V31), non abbassare il numero.

### 7.6 Cosa la porta deliberatamente non controlla

Il perimetro negativo, come in §0.2 e nella §0.2 della spec del kernel: è l'artefatto che
impedisce insieme la falsa sicurezza e l'allargamento silenzioso.

#### 7.6.1 Fuori dal kernel per natura

Già elencato in [design/08](../../design/08-strategia-di-test.md) e non si ripete: qualità
delle risposte del modello, valutazione con giudice e dataset curati, correttezza semantica
di un piano agentico, qualità percepita di voce e mesh, ergonomia dell'interfaccia.

#### 7.6.2 Dentro il perimetro, e non controllato per scelta

| Non controllato | Perché | Cosa lo copre invece |
|---|---|---|
| `HashMap` fuori da `kernel` e `simulator` | non difende V29: in una corsa DST `platform` **non gira affatto** (§7.4.4) | niente, ed è corretto |
| il **tempo di parete** dell'arbitro come non-regressione | M-7 dichiara che il massimo per operazione è dominato dal rumore dello scheduler di Windows. Un cancello su un numero rumoroso si impara a ri-lanciare finché non passa | i numeri di M-7 come **limite superiore**; Q1 lo chiude SP-2 |
| la **percentuale di copertura** del codice | il criterio di questo progetto è «ogni V ha un controllo», non «l'X % delle righe». Una copertura alta con invarianti non verificate è la falsa sicurezza peggiore | la tabella della **§8** |
| che una crate ammessa non faccia nulla di indesiderato | ADR-0031 lo dichiara: *«limita la superficie, non la certifica»* | la giustificazione scritta, e chi la legge |
| che `platform` si comporti come `simulator` su **tutte** le porte | solo due ne hanno entrambe le implementazioni qui (§7.4.6) | contratto su `journal` e `reactor`; le altre in §8 come rimandate |
| lo **stile** del codice | non difende nessun V, quindi la regola 1 di §7.1.1 lo esclude | `clippy` come igiene, **fuori** dalla porta |
| Q6 · Q11 · Q12 · Q16 | non hanno consumatore in questo sotto-progetto. ⚠️ dal 2026-08-07 la riga della §0.6 ne elenca **cinque**: vi si è aggiunto **Q21**, per la correzione di §8.5.1 | la §8, con il sotto-progetto che li chiude |

#### 7.6.3 La riga che chiude la sezione

> **La porta non prova che il kernel sia corretto.** Prova che un insieme **nominato** di
> invarianti regge. Un difetto che non viola nessun V passa verde.

È lo stesso limite del gettone (§6.3.2): il dispositivo elimina una classe di errori, non
due. Dichiararlo è ciò che impedisce alla porta verde di diventare un argomento — *«i
controlli passano, quindi va bene»* — che è il modo in cui una porta di qualità smette di
essere utile pur restando in funzione.

### 7.7 I costi di questa sezione

| Costo | |
|---|---|
| **la porta è lavoro prima di ogni valore visibile** | come il simulatore: è lo stesso RK-9, già accettato nella spec del kernel |
| **tredici voci sono di livello 2** | cioè cancellabili. ADR-0031 lo dichiara per una sola; qui vale per tutte, e non è mitigabile — è la natura del livello, non un'omissione. ⚠️ **Ricontato sulla tabella §7.4.2 due volte il 2026-08-08**: diceva «nove» prima delle righe di ADR-0036 e ADR-0037, e «undici» prima della riga degli **attributi**, aggiunta eseguendo il Traguardo 1. ⚠️ **Terzo riconteggio il 2026-08-09**: diceva «dodici» prima della riga del **build script**, aggiunta chiudendo la lacuna che una revisione aveva misurato — sei controlli su sei verdi con un `build.rs` nel kernel |
| **si paga a ogni commit, non una volta** | due grafi, un cancello, una campagna, due suite di contratto |
| **`cargo tree` è un'interfaccia per umani** | un cambio di formato rompe **due** controlli in una volta sola |
| **il bersaglio senza OS è un prerequisito dell'ambiente** | su una macchina pulita la porta è rossa finché non lo si installa, e per il motivo sbagliato |
| **la porta non prova la correttezza** | §7.6.3. Sposta il confine di ciò di cui ci si può fidare, non lo elimina |

#### 7.7.1 Il punto in cui questa sezione viola la propria regola

Va scritto, perché è l'unico posto della §7 che non è a sua volta verificabile.

> La §7.1.1 impone che **ogni** controllo abbia una contro-sonda. Ma niente lo verifica: è
> una regola sul processo, cioè **un'intenzione** — esattamente ciò che il metodo del
> progetto chiama «un principio che non si può controllare».

Il catalogo però è una tabella in un file di testo, e `check-docs.sh` può leggerla e
**fallire se una casella “contro-sonda” è vuota**. Poche righe, e la regola 3 smette di
essere un'intenzione.

La §0.6 aveva già previsto la stessa estensione per la tabella della §8 — *«`check-docs.sh`
può essere esteso per controllarla, come già fa per V30»*. Le due estensioni toccano lo
stesso script e lo stesso genere di tabella.

**Si scrivono insieme, quando si scrive la §8.** Farlo ora significherebbe controllare una
tabella e non l'altra, con due passaggi sullo stesso file.

> ✅ **Fatto, e questa sottosezione ora descrive un passato — richiamo del 2026-08-08.** Le
> due estensioni sono state scritte insieme con la §8: `check-docs.sh` legge il catalogo
> fra `#### 7.4.1` e `#### 7.4.3` e **fallisce se l'ultima casella è vuota** (§8.6.1,
> asserzione 1), con guardia di non-vacuità sui delimitatori (§8.6.2) e sonde in due
> direzioni (§8.6.3). ⛔ **Il testo sopra resta al presente perché era vero quando è stato
> scritto**, ma va letto con questo richiamo: la regola 3 **non è più** un'intenzione, e la
> prova materiale è che alla sua prima corsa il controllo ha trovato la riga di V31 con tre
> celle su quattro — §7.4.2. ⚠️ Ciò che **resta** non verificabile è un'altra cosa, ed è
> dichiarata altrove: che la casella *nomini davvero* una voce del catalogo (§8.6.4), che è
> il buco da cui sono usciti §8.5.3, §8.5.4 e §8.5.5.

---

## 8. Copertura V1–V37 e Q1–Q24

### 8.0 A parole

Le §0–§7 hanno deciso *cosa* il sistema garantisce e *chi lo verifica*. Questa sezione fa
l'unica cosa che nessuna delle precedenti ha fatto: **giudica, una per una, tutte e
sessantuno le voci nominate**, e dice se questo sotto-progetto ha finito con ciascuna.

Non è un riepilogo. Un riepilogo ripete; questa sezione **decide**, e in **cinque** casi ha
trovato che qualcosa non tornava (§8.5).

Ha una seconda natura, meno ovvia. La §7.6.3 chiude così:

> *«La porta non prova che il kernel sia corretto. Prova che un insieme **nominato** di
> invarianti regge.»*

**Questa è la sezione in cui quell'insieme viene nominato.** Senza di essa «insieme
nominato» è una figura retorica: non esiste il posto in cui l'insieme è scritto per intero,
e nessuno può controllare che non si sia ristretto in silenzio.

Ed è la mitigazione che la §0.6 aveva promesso contro il costo peggiore dello
scaglionamento — *«rimandato tende a diventare dimenticato»*. La promessa era in due
pezzi: una tabella che elenca ogni V e ogni Q con il proprio stato, e uno script che la
controlla. Entrambi vivono qui.

### 8.1 Il vocabolario degli stati

Quattro valori, chiusi. Un quinto non si aggiunge senza modificare anche il controllo di
§8.6, che è il punto: l'insieme è chiuso *per costruzione*, non per buona volontà.

| Stato | Significato | Cosa la riga deve portare |
|---|---|---|
| ✅ **verificato qui** | esiste un controllo **in perimetro**, visto scattare e visto restare verde | il meccanismo, con il suo livello di forza |
| ⚠️ **parziale** | una parte è verificata qui, una parte no — e si dice **quale** | ⛔ **innesco obbligatorio** |
| ⏳ **rimandato** | nessun controllo qui | ⛔ **innesco obbligatorio** |
| ⛔ **non controllato** | in perimetro, e **si sceglie** di non controllarlo | il motivo (è la §7.6.2) |

#### 8.1.1 Perché quattro e non tre

I primi tre bastavano quasi. Due classi di righe non ci stanno dentro senza dire il falso,
e sono esattamente le due che le sezioni precedenti avevano già isolato:

| Riga | Con «verificato qui» | Con «rimandato» |
|---|---|---|
| **V25 · Q20** — un solo punto di uscita (§7.4.2) | ⛔ sopravvaluta: la contro-sonda **non esiste**, e la §7.4.2 lo dichiara | ⛔ sottovaluta: il controllo gira *già* a ogni commit, e la sonda scatta |
| **Q17 · Q22 · Q23** — «verificati solo lato kernel» (§0.6) | ⛔ cancella RK-11 | ⛔ cancella il lavoro lato kernel, che è fatto |

**Il costo del quarto stato, dichiarato:** `parziale` è la casella comoda in cui si può
parcheggiare qualunque cosa. La mitigazione è l'**innesco obbligatorio**, ed è il gettone
della §6.3 applicato a una tabella: una riga `parziale` senza innesco non passa lo script,
quindi non è esprimibile. Chi vuole parcheggiare deve prima dire chi verrà a riprendere.

#### 8.1.2 Cosa vale come «controllo» in questa tabella

La colonna del meccanismo nomina **una voce della §7**, mai un'intenzione. Sono ammesse
tre risposte, e non di più:

| Risposta | Da |
|---|---|
| una voce del **catalogo** §7.4.1 o §7.4.2, con il suo livello | §7.4 |
| i **test a esempi**, che la cadenza fa girare a ogni commit | §7.5.1 |
| la **campagna DST**, con la porta in cui si inietta | §3.3 · §7.4.2 |

Un V che non può nominare nessuna delle tre **non è ✅**. È la regola 1 del criterio di
ammissione (§7.1.1) letta al contrario, e serve a impedire che questa tabella diventi il
posto in cui si aggiungono controlli che la §7 non ha esaminato.

#### 8.1.3 I V e i Q si giudicano con criteri diversi, e va detto

Non è un'incoerenza: le due famiglie sono oggetti diversi.

| | Criterio | Autorità |
|---|---|---|
| **V** — vincoli | il vincolo è soddisfatto **in tutte le sue parti** dentro il perimetro? | il testo del vincolo |
| **Q** — requisiti | il **metodo che `design/08` gli assegna** è eseguibile dentro il perimetro? | [`design/08`](../../design/08-strategia-di-test.md) |

Per i Q l'autorità non è discrezionale: è la condizione 3 della §0.7 — *«ogni Q in
perimetro è verificato col metodo che design/08 gli assegna, non con un altro»* — ed è
anche il motivo per cui V30 esiste.

**Conseguenza visibile, e va anticipata perché sembra un'incoerenza e non lo è.** Q7 e Q8
sono ✅ benché la loro metà rivolta all'utente non esista: `design/08` assegna a entrambi
un test a esempi su un **evento emesso dal kernel**, e quel test è eseguibile qui. V9 —
*«ogni ingresso in `AttesaUmano` emette una notifica»* — è invece ⚠️, perché il vincolo
parla della notifica, non dell'evento. Due giudizi diversi sullo stesso confine, ciascuno
dalla propria autorità.

### 8.2 Le due specie di innesco

Un rimando senza innesco è un rimando dimenticato. Le specie sono **due**, e la differenza
è cosa deve accadere perché la voce si chiuda.

| Specie | Si chiude quando | Voci |
|---|---|---|
| **sotto-progetto** | arriva un **consumatore** | tutto il resto |
| **spike** | arriva una **misura** | **Q1**, con SP-2 |

**Ho cercato una terza specie e non l'ho trovata.** I candidati esaminati e perché non
sono inneschi:

| Candidato | Perché no |
|---|---|
| una **misura** — M5, quota di presentazione | tara un numero. Nessun V e nessun Q vi pende: V1 chiede che una concessione esista, non quanto valga |
| un **ADR** — 0029, il guscio | non vincola nessun V né Q di questo sotto-progetto. È il verso opposto: §5.5.4 esporta un discriminante verso di lui |
| **SP-1, SP-3** | tarano i parametri di V2 e Q11; il meccanismo è verificabile senza |

> **L'innesco è ciò che rende il requisito verificabile, non ciò che ne tara un numero.**

È la stessa distinzione della §9.3 della spec del kernel — *«solo due spike bloccano; gli
altri quattro tarano parametri di decisioni già prese»*.

#### 8.2.1 Come si scrive un innesco: la condizione, poi il numero

Un innesco scritto come «sotto-progetto 4» invecchia il giorno in cui la roadmap cambia, e
invecchia **in silenzio**. Quindi:

> L'innesco nomina la **condizione**; il numero del sotto-progetto sta fra parentesi come
> *chi la soddisfa per primo oggi*. Se la roadmap cambia, la condizione resta vera e il
> numero si aggiorna.

Le condizioni che ricorrono, raccolte una volta sola perché la tabella non le ripeta per
esteso:

| Sigla | Condizione | Oggi |
|---|---|---|
| **A** | esiste un'interfaccia | **2** — GUI minima |
| **B** | qualcuno chiama un modello: proiezione, provider reale, rete | **3** — Conversazione |
| **C** | esistono strumenti e permessi da mediare, **e sensori reali da eseguire** | **4** — Agenti |
| **D** | si esegue codice o un comando, e si scrive su file reali | **5** — Coding |
| **E** | esiste un worker reale da avviare e uccidere | **7** — Generazione asset ⚠️ |
| **F** | esistono backup e ripristino | **11** — Backup e ripristino |
| **SP-2** | la misura di Q1 sotto carico GPU | spike, dentro **8** — Voce |

⚠️ **Sulla E.** Il primo worker *certo* è la generazione asset, che dichiara un carico GPU
proprio. Il sotto-progetto 6 potrebbe anticiparla se l'indicizzazione girasse in locale
invece che su un provider remoto: la condizione resta quella, il numero è il candidato
odierno.

📌 **Sulla F, e sul perché questa colonna non è decorativa.** Quel numero **non esisteva**
quando la tabella è stata compilata: la roadmap non collocava il backup da nessuna parte, e
il vuoto è emerso dal solo fatto di dover riempire la casella. Il sotto-progetto 11 è nato
lì (§8.5.2), e il suo posto in coda è **derivato, non comodo**: dipende da 5, 6 e 9 perché
prima l'elenco delle esclusioni di V32 sarebbe vuoto, e verificarlo su un elenco vuoto è
una prova che non può fallire.

#### 8.2.2 Un Q della DST eredita lo stato della porta in cui si inietta

Questa non è una scelta: è la conseguenza di incrociare due tabelle che finora non si
erano mai guardate. La §3.3 dice **in quale porta** si inietta il guasto che verifica ogni
Q; la §7.4.6 dice **quali porte hanno la suite di conformità** fra la finta e la vera.

| Porta | Suite di conformità (§7.4.6) | Q che ne eredita lo stato | Esito | Innesco |
|---|---|---|---|---|
| `journal` | ✅ c'è — e §4.6 ne copre anche il livello 2 | **Q5** | ✅ | — |
| `reactor` | ✅ c'è — ed è la più importante | **Q2** | ✅ | — |
| `process` | ⚠️ rimandata: non ci sono worker da avviare — e col **dialogo** (§6.10) la suite acquista un'affermazione in più, sul **filo** oltre che sul ciclo di vita (§7.4.6) | **Q4** | ⚠️ | E — esiste un worker reale (7) |
| `ipc` | ⚠️ rimandata: non c'è una GUI dall'altro capo | **Q3** | ⚠️ | A — esiste un'interfaccia (2) |
| `filesystem` | ❌ scaglionata (§0.4) | **Q22** | ⚠️ | D — si scrive su file reali (5) |
| `network` | ❌ scaglionata | **Q18** | ⚠️ | B — qualcuno chiama un modello (3) |

L'ultima colonna è ciò che la §7.4.6 chiedeva alla lettera: *«la §8 registra quali porte
hanno la suite e quali no, con il sotto-progetto che le chiude»*.

Senza la suite di conformità, la DST dimostra che il kernel si comporta bene **con il
simulatore**. È il punto cieco che la §3.7 dichiara con parole sue — *«la finta non è la
vera»* — e chiamare ✅ un Q provato contro una finzione sarebbe la falsa sicurezza che
questo progetto combatte ovunque.

⚠️ **Questo aggiunge una condizione che `design/08` non prevedeva**, ed è esattamente ciò
che la §7.5.2 ha registrato: la conformità fra `platform` e `simulator` è un uso dei test
di contratto che quel documento non contemplava. Per Q3, Q4, Q18 e Q22 il metodo di
`design/08` **è** eseguibile qui; ciò che manca è la condizione aggiunta dalla §7.4.6.

Questa tabella è la **derivazione** di sei righe delle due seguenti, non un secondo
registro. È anche la risposta alla richiesta esplicita della §7.4.6 — *«la §8 registra
quali porte hanno la suite e quali no»* — tenuta in un posto solo perché due posti si
disallineano (§7.4.4, caso 2).

### 8.3 I vincoli V1–V37

| ID | Vincolo | Stato | Con quale controllo, e cosa manca | Innesco |
|---|---|---|---|---|
| V1 | nessun lavoro tocca la GPU senza concessione valida | ✅ verificato qui | gettone sulla porta `process`, livello 1 (§7.4.1 B) · campagna DST (§7.4.2) · per la GUI la concessione di presentazione è tenuta dal core (§5.5.1). ⚠️ verso il compositor il rifiuto non è esecutivo: §5.5.2, non mitigabile | — |
| V2 | ogni lavoro GPU ha un profilo di risorsa dichiarato | ✅ verificato qui | §7.4.1 C, riga V2 — un'ammissione senza profilo non compila. La taratura dei valori è SP-1, che è un parametro | — |
| V3 | una sola policy attiva, e proviene dal profilo di configurazione | ⚠️ parziale | §7.4.1 C, riga V3 — una **seconda policy attiva** non è esprimibile: il valore consegnato ne porta una sola (§2.8.2, livello 1) · campagna DST: una transizione interrotta lascia un passo riconciliabile (§5.7). ⚠️ **riscritta il 2026-08-07 con ADR-0034**: la cella diceva *«la configurazione non ha consumatore»*, e non è più vero — la §2.8 la mette in perimetro. Manca l'**archivio su disco e il pannello che lo modifica**, scaglionati per regola C | A — esiste un'interfaccia (2) |
| V4 | `Rifiutata` e `InCoda` sono esiti distinti, anche in interfaccia | ⚠️ parziale | §7.4.1 C, riga V4 — trattare l'esito come due vie non compila; **la distinzione in interfaccia** no | A (2) |
| V5 | nessun effetto senza classe dichiarata; **l'assenza vale `irripetibile`** | ✅ verificato qui | la classe è un campo obbligatorio del tipo: livello 1, con test di compilazione fallita (§7.4.1 C, §7.4.4 punto 3) | — |
| V6 | write-ahead obbligatorio | ✅ verificato qui | campagna DST su `journal`, che ha la suite di conformità (§8.2.2) · due livelli di crash provati, M-2 e M-8 (§4.6) | — |
| V7 | il contesto non è mai sorgente di verità | ⚠️ parziale | test a esempi sul modello dello stato durevole (§4.4): ciò che è dichiarato non sacrificabile si rilegge dal giornale dopo una ripresa. **La proiezione non esiste**, quindi metà del vincolo non ha soggetto | B — qualcuno chiama un modello (3) |
| V8 | ogni run ha un tetto, con default conservativo | ✅ verificato qui | i confini di autonomia entrano (§0.4, §4) · test a esempi sulla transizione ad `AttesaUmano`, che è il metodo di Q7 | — |
| V9 | ogni ingresso in `AttesaUmano` emette una notifica | ⚠️ parziale | test a esempi sull'**evento** emesso e giornalato; **la notifica all'utente** no | A (2) |
| V10 | un sensore osserva e non modifica nulla | ✅ verificato qui | §7.4.1 C, riga V10 — un sensore che modifica l'artefatto non compila: §6.4.2 lo consegna per riferimento immutabile | — |
| V11 | ogni sensore dichiara il proprio costo; gli inferenziali fuori dall'anello stretto | ⚠️ parziale | test a esempi: il **costo dichiarato** decide l'ammissione all'anello stretto (§6.4.1). **Nessun sensore inferenziale esiste**, quindi la seconda metà non ha soggetto. ⚠️ **Innesco allineato il 2026-08-08:** la cella riscriveva la condizione C con parole proprie — «strumenti e sensori reali» — creando una seconda definizione di una sigla che §8.2.1 raccoglie *«una volta sola»*. La condizione C è stata **allargata alla fonte** per comprendere i sensori, invece di essere riscritta qui | C (4) |
| V12 | l'anello 4 propone, non applica | ⏳ rimandato | l'anello 4 è scaglionato per regola C (§0.4, §5): legge ricorrenze che esistono solo quando qualcosa gira | C (4) |
| V13 | la ricomposizione mantiene il **budget**, non evita l'overflow | ⏳ rimandato | la ricomposizione della proiezione è scaglionata per regola C (§0.4, §4) | B (3) |
| V14 | un verdetto negativo che rientra nell'anello è un passo nuovo, giornalato | ✅ verificato qui | test a esempi con sensore finto e verdetto scelto dal test: è il metodo che `design/08` assegna a Q10 (§6.4.2) | — |
| V15 | ogni richiesta dichiara i propri vincoli, anche quando coincidono con i default | ✅ verificato qui | test a esempi sul decisore, che entra per regola A (§0.4, §3) e non richiede nessun provider (ADR-0020) | — |
| V16 | il record di routing non contiene mai credenziali; **nomi di provider e parametri sì** | ⚠️ parziale | ⛔ **Ri-giudicato il 2026-08-08, e lo stato torna a `parziale`.** La metà **positiva** — il record *deve* portare nomi di provider e parametri — è verificata qui: è il record **risolto** di §6.2, con lo stesso test a esempi su giornale sintetico che rende ✅ V15 e Q14. La metà **negativa** resta vacua: nessuna credenziale attraversa il sistema in questo perimetro (`secrets` esiste, nessun adattatore la usa), quindi un controllo proverebbe l'assenza di una cosa che non c'è — gotcha #17, e resta vero. ⚠️ **Il declassamento di §8.5.3.1 era corretto sulla metà che aveva davanti, e la metà positiva non ce l'aveva**: la formulazione in questa colonna era troncata. Vedi §8.5.5 | B (3) |
| V17 | ritentativo e cambio di candidato restano dentro lo stesso passo | ✅ verificato qui | test a esempi sul discriminante di §6.2 — *il modello ha prodotto output?* | — |
| V18 | un errore di vincolo nomina **quale** vincolo | ⚠️ parziale | test a esempi: l'errore prodotto dal filtro porta il nome del vincolo non soddisfatto; **che l'interfaccia lo mostri** no | A (2) |
| V19 | tipo distinto per il contenuto esterno, conversione esplicita e giornalata | ✅ verificato qui | la conversione riceve la porta `journal` come argomento: gettone di livello 1 (§6.5, §7.4.1 B) | — |
| V20 | l'etichetta di non-fidatezza è ereditaria attraverso ogni trasformazione | ✅ verificato qui | test di compilazione fallita, livello 1 (§7.4.1 C). ⚠️ le trasformazioni esistenti oggi sono poche: ogni trasformazione nuova porta il proprio caso in `tests/compile_fail/` (§2.5) | — |
| V21 | un permesso vale per la tripla concessa e per la sessione corrente | ⚠️ parziale | test a esempi sulla **forma** del permesso e sulla sua registrazione nel giornale (§6.6): una tripla concessa non copre una tripla diversa. Il mediatore, i preset e il ciclo di approvazione sono scaglionati per regola C | C (4) |
| V22 | nessuna descrizione di strumento concede permessi | ⏳ rimandato | non esistono strumenti MCP: regola C (§0.4, §6) | C (4) |
| V23 | la provenienza del contenuto è visibile in interfaccia | ⏳ rimandato | vincolo interamente d'interfaccia | A (2) |
| V24 | il giornale è la sorgente; trace, metriche e costi ne sono proiezioni | ⚠️ parziale | test a esempi: il picco di VRAM (§5.2.2) e i permessi attivi (§6.6) si ricavano **rileggendo il giornale**, e non esiste un secondo archivio da cui ricavarli. **La proiezione trace non esiste**, quindi l'altra metà non ha soggetto | A (2) |
| V25 | nessuna telemetria **lascia la macchina** per default; un solo punto di uscita | ⚠️ parziale | il controllo gira a ogni commit e la **sonda scatta** — una chiamata di rete in `daemon` lo accende; **la contro-sonda non esiste**: la lista è vuota e non c'è niente di legittimo da lasciar passare (§7.4.2). È l'unica voce del catalogo provata in una direzione sola | B (3) |
| V26 | la ritenzione pota i payload grezzi, mai i record strutturati | ✅ verificato qui | `prune` è un'operazione della porta `journal` (§4.1) · test a esempi sulle due regole non negoziabili di ADR-0018: un record potato **dichiara** di esserlo, e un passo in dubbio non è potabile (§4.5). ⚠️ **RICHIAMO DEL 2026-08-27:** la §4.5 porta ora il proprio richiamo, e dichiara che **nessuna delle due** regole è tenuta dal codice — la prima è **violata** da entrambe le implementazioni, la seconda è tenuta con un'**altra nozione** di dubbio. Senza questo rimando le due sedi si contraddicono attraverso la citazione che le lega. ⛔ **Ri-giudicare lo stato è un'altra decisione e sta altrove:** è il finding **AUD-005**, radice R7, **aperto** | — |
| V27 | nessuna azione fallisce per una condizione già nota e non dichiarata | ⚠️ parziale | lo stato di degrado è un oggetto derivato in perimetro (§6.7) e Q18 lo verifica in DST; **che l'interfaccia lo dichiari prima** no | A (2) |
| V28 | nessun modello nel percorso decisionale del kernel; **verificabile staticamente** | ✅ verificato qui | §7.4.2, riga I3 · V28 — corollario dell'allow-list: un percorso verso un adattatore comparirebbe nel grafo transitivo (§7.4.4 punto 2). Livello 2, sonde N1–N3 e contro-sonda N4. ⚠️ ADR-0031 dichiara il proprio limite: *«limita la superficie, non la certifica»* | — |
| V29 | tempo, casualità, I/O, scheduling **e i parametri di decisione** iniettabili | ✅ verificato qui | `no_std` e il divieto gratuito di `HashMap`, livello 1 · allow-list e cancello senza OS, livello 2 · §7.4.1 C, righe `V29 · §2.1` e `V29 · §2.8` — scambiare monotonic e wall time non compila, e nemmeno costruire una decisione senza i parametri consegnati · la campagna DST stessa, il cui criterio C1 fallisce a ogni sorgente nascosta di non determinismo (§3.7). ⚠️ `HashMap` fuori dal kernel è **deliberatamente non controllato**: §7.4.4 punto 1. ⚠️ **Formulazione allargata il 2026-08-08 chiudendo la §7.1.1**, e confrontata con la fonte come impone §8.5.5: il testo nominava quattro assi, ADR-0034 ne aveva aggiunto un quinto il 2026-08-07. **Lo stato non cambia** — il quinto asse è difeso da un controllo di livello 1 già in perimetro | — |
| V30 | ogni requisito Q ha un metodo di verifica dichiarato **prima** dell'implementazione | ✅ verificato qui | `check-docs.sh`, livello 2, già in esercizio (§7.4.2) | — |
| V31 | ogni difetto trovato in simulazione conserva il proprio seme **come caso di regressione** | ✅ verificato qui | il seme entra nell'elenco versionato, la **proprietà** entra nella suite (§7.4.2). ⚠️ debole per natura: l'automatismo protegge la proprietà, non il seme (§3.4). Nessun innesco la rafforza — è un limite, non un pezzo mancante | — |
| V32 | il backup contiene solo l'irriproducibile | ⏳ rimandato | il backup si scaglia per regola C (§0.4.1). ⚠️ verificarlo prima che esistano indici e pesi sarebbe **vacuo**: l'elenco delle esclusioni sarebbe vuoto | F — esistono backup e ripristino (11) |
| V33 | i segreti non entrano mai nel backup automatico | ⏳ rimandato | idem (§0.4.1). Il **layout per natura** di ADR-0022 è invece già rispettato: `secrets` è un archivio distinto (§1.2) | F (11) |
| V34 | il **gestore dei segreti** è l'**unico** punto di lettura delle credenziali; verificabile staticamente | ⏳ rimandato | ⛔ **DECLASSATA DA ✅ IL 2026-08-27, finding AUD-026:** la cella diceva *«livello 2, con sonda e contro-sonda (§7.4.2)»* e quel controllo **non esiste** — `gate-deps.sh` misura i grafi di `kernel` e `simulator`, mai quello di `platform`, e in tutto il repository non esiste nessuna crate di portachiavi. Che `secrets` sia una crate separata resta vero ed è il motivo per cui le crate sono cinque e non quattro (§1.2), ma è una **scelta di scrittura** e non un controllo: §8.1.2, e la stessa lettura che declassò **V16** (§8.5.3.1). Il riquadro sotto la tabella di §7.4.2 porta la misura | B (3) |
| V35 | nessuna esecuzione di codice o comando sotto il livello 2 | ⚠️ parziale | **test a esempi**, gli stessi che rendono ✅ V37: il livello di confinamento è un campo obbligatorio dell'azione e finisce nel giornale. ⚠️ **Meccanismo rinominato il 2026-08-08:** la cella diceva «entrano (§7.4.5)», che è una voce di sezione e non una delle tre risposte ammesse da §8.1.2 — la stessa forma dei difetti §8.5.3 e §8.5.4. **Il quarto gettone si scaglia** perché nessuna porta esegue comandi qui, ed è retrofittabile: §7.4.5 | D — si esegue un comando (5) |
| V36 | gli effetti fuori dagli ambiti dichiarati **non sono coperti dal checkpoint** e restano soggetti alle classi di effetto | ⚠️ parziale | §7.4.1 C, riga V5 — un effetto senza classe non compila, **dentro o fuori un ambito indifferentemente**, ed è ciò che V36 chiede. **Che il checkpoint copra davvero gli ambiti** richiede il filesystem reale, scaglionato (§0.4, §10) | D (5) |
| V37 | il livello di confinamento usato entra nel giornale insieme al passo | ✅ verificato qui | test a esempi: è la parte che §7.4.5 fa entrare comunque | — |

### 8.4 I requisiti Q1–Q24

| ID | Requisito | Stato | Con quale controllo, e cosa manca | Innesco |
|---|---|---|---|---|
| Q1 | voce sotto i 600 ms con job GPU pesante | ⏳ rimandato | il metodo di `design/08` è una **misura end-to-end**, che richiede voce e carico reali. §7.6.2 lo dichiara già: il tempo di parete dell'arbitro non è un cancello, perché è un numero rumoroso | **SP-2** (spike, dentro 8) |
| Q2 | zero OOM | ✅ verificato qui | campagna DST su `reactor`, che ha la suite di conformità (§8.2.2) · sonda negativa esplicita: si concede oltre il budget, la campagna fallisce e nomina il seme (§5.7.1) | — |
| Q3 | crash della GUI durante una run | ⚠️ parziale | il metodo di `design/08` — DST con morte del client — **è eseguibile qui** (§5.7); manca la **suite di conformità su `ipc`**, quindi la prova è contro una finta (§8.2.2) | A (2) |
| Q4 | kill di un worker in qualsiasi istante | ⚠️ parziale | idem su `process`: la DST inietta il kill **e i quattro guasti del dialogo** (§3.3) · la vita del worker è al compilatore — parlargli pretende il `Worker` che l'avvio ha restituito, leggere pretende una **ricevuta**, `uccidi` consuma il `Worker` (§7.4.1 B e C) · sul filo, i byte consumati devono pareggiare la lunghezza dichiarata (§7.4.2, gotcha #34). ⚠️ **Riletta il 2026-08-08 con F1b:** non esiste un worker reale contro cui provare la conformità della finta, e col dialogo ciò che manca alla suite è cresciuto — anche il **filo**, non solo il ciclo di vita (§7.4.6). **Lo stato non cambia** | E (7) |
| Q5 | riavvio del core a metà run, nessun effetto rieseguito | ✅ verificato qui | DST con crash-injection su `journal`, suite di conformità presente, **e** il livello 2 dentro il motore: M-8, 12 punti scattati, 12/12 riaperti coerenti (§4.6). ⚠️ **Rafforzato il 2026-08-08 con ADR-0036:** la porta `journal` scambia **byte** e la codifica vive in `kernel` (§4.9.3), quindi il crash cade **dentro** la scrittura e la campagna esercita davvero codifica e decodifica — prima le avrebbe scavalcate | — |
| Q6 | contesto esaurito | ⏳ rimandato | il metodo è una proprietà su ricomposizioni ripetute, e la ricomposizione è scaglionata (§0.6) | B (3) |
| Q7 | tetto di passi, tempo o costo superato | ✅ verificato qui | test a esempi sulla transizione ad `AttesaUmano` — il metodo di `design/08` è interamente lato kernel ed è eseguibile qui (§8.1.3) | — |
| Q8 | avvio a freddo dichiarato | ✅ verificato qui | test a esempi sull'**evento emesso prima dell'attesa** · e §5.2.1 rende `cold_start` irraggiungibile dal percorso decisionale, con test di compilazione fallita (§7.4.1 C) | — |
| Q9 | contenuto non fidato nel canale delle istruzioni | ✅ verificato qui | **non compila**: test negativo di compilazione, livello 1 con visibilità di livello 2 (§7.4.1 C, §7.1.3). ⚠️ **Precisato il 2026-08-09, e la precisazione conta:** le righe di catalogo sono **due** — regola A e regola B — e a reggere questa cella è soprattutto la **seconda**. Misurato: la regola A è **cieca** a `impl From<Untrusted> for Instruction`, perché il suo scarto è fra **riferimenti** e rustc non ha niente da suggerire, quindi il caso resta `ok`. Con la sola regola A catalogata il ✅ poggiava su una guardia che quella via non la vede. **Lo stato non cambia** — il caso esiste dal Task 9 — ma la ragione ora è nominata giusta | — |
| Q10 | verdetto di sensore che rientra nell'anello | ✅ verificato qui | test a esempi con sensore finto, che è il metodo assegnato da `design/08` (§6.4.2). ⚠️ prova che l'**anello** funziona, non che il **contratto** regga sensori reali: è RK-5, già accettato | — |
| Q11 | occupazione della proiezione al budget | ⏳ rimandato | nessuna proiezione da misurare (§0.6). SP-3 ne tarerà la soglia, ma non è ciò che lo rende verificabile | B (3) |
| Q12 | difetto ricorrente che diventa una proposta | ⏳ rimandato | l'**anello 4 non esiste**: è scaglionato per regola C (§0.4, §5), e senza di lui non c'è niente che emetta la proposta che il metodo verifica. ⚠️ **Motivazione corretta il 2026-08-08:** diceva *«legge ricorrenze che esistono solo quando qualcosa gira»*, ma il metodo di `design/08` è un **giornale sintetico con ricorrenza** — la ricorrenza si costruisce senza far girare niente, come per Q14. A mancare è l'anello, non il dato | C (4) |
| Q13 | nessun candidato non conforme viene mai eseguito, **per qualunque catena** | ✅ verificato qui | **gettone di conformità**: un candidato non filtrato non è esprimibile come argomento di un'esecuzione. Livello 1 (§6.3.1, §7.4.1 B). ⚠️ il gettone prova la provenienza, non la correttezza del filtro: §6.3.2. ⛔ **Divergenza da `design/08`, registrata il 2026-08-08 invece che nascosta:** il metodo assegnato è una **verifica di proprietà** su catene generate; qui la proprietà è resa **non esprimibile**, che è più forte — una proprietà provata su N catene lascia scoperta la N+1, un tipo no. È la stessa mossa di §5.3 punto 3. Sostituire un metodo con uno più forte resta una sostituzione, e §8.1.3 pretende che si dica | — |
| Q14 | ricostruire con cosa è stato eseguito un passo di sei mesi fa | ✅ verificato qui | il record di routing è **risolto** e giornalato col passo (§6.2): test a esempi su un giornale sintetico. La proprietà è strutturale — il record non rimanda alla configurazione, quindi non dipende da essa. ⚠️ **Il meccanismo è cresciuto il 2026-08-07 con ADR-0036**, e senza di esso «sei mesi fa» era una promessa: il record **dichiara la propria versione** — enum di versione al compilatore (§7.4.1 C) — e i **byte congelati** con la mappa `indice → nome → valore atteso` provano che un giornale scritto oggi si rilegge domani (§7.4.2, §4.9.4). ⛔ Vale il limite di §4.9.4: il compilatore prova che una versione è **dichiarata**, non che sia quella **giusta**. Un campo aggiunto dopo — il passo padre di un fork, §4.9.5 — è **facoltativo con un indice nuovo**, quindi non rompe la rilettura | — |
| Q15 | un'istruzione trovata nei dati non autorizza | ⚠️ parziale | la metà **statica** è qui: §7.4.1 C, **le due** righe Q9·I6·V20 — la regola A (`Untrusted` dove è attesa un'`Instruction`) e la regola B (nessuna via `From`/`Into`) — più il gettone `journal` sulla conversione (§7.4.1 B, V19). ⚠️ **Riletta il 2026-08-09:** la cella diceva «riga» al singolare, ed era vera finché la riga era una; dal richiamo di §7.4.1 sono due, e la seconda è quella che vede il ponte di conversione. **Lo stato non cambia** · la metà a esempi — *l'obbligo di autorizzazione* — richiede il mediatore e il ciclo di approvazione, scaglionati per regola C | C (4) |
| Q16 | descrizione MCP cambiata dopo l'approvazione | ⏳ rimandato | il metodo è un test di contratto contro un server MCP finto, e non esistono strumenti (§0.6) | C (4) |
| Q17 | un segreto compare in contenuto in uscita | ⏳ rimandato | lato kernel: §7.4.2, riga V34 · Q24 — solo `secrets` raggiunge il portachiavi, livello 2 provato in due direzioni · **il canary è scaglionato** (§0.4, §6) e non c'è contenuto in uscita da controllare. ⛔ **Precisato il 2026-08-08, perché la cella si attribuiva un merito altrui:** il metodo che `design/08` assegna a Q17 è il **canary a esempi**, e **non ne gira niente**; il controllo di livello 2 qui accreditato è quello che `design/08` assegna a **Q24**. Resta ⚠️ e non ⏳ perché è esattamente la classe di §0.6 — *«verificato solo lato kernel»* — che §8.1.1 dichiara essere una delle due ragioni per cui il quarto stato esiste ⛔ **DECLASSATA DA ⚠️ A ⏳ IL 2026-08-27, finding AUD-026:** la metà *«lato kernel»* era **la riga V34 · Q24**, e quel livello 2 **non esiste** — quindi non resta niente di verificato qui, e la ragione scritta sopra (*«resta ⚠️ perché è la classe di §0.6»*) cade con la propria premessa. ⚠️ **Il capoverso del 2026-08-08 NON si riscrive**: è un verbale, e diceva già il vero su `design/08` | B (3) |
| Q18 | perdita della rete | ⚠️ parziale | il metodo di `design/08` — DST con iniezione del guasto — è eseguibile e verifica che il degrado sia dichiarato **prima** del primo fallimento (§3.3); ma `network` non ha implementazione reale, quindi nessuna conformità (§8.2.2) | B (3) |
| Q19 | capire cosa è andato storto in una run di 4 ore | ⏳ rimandato | il metodo poggia sulla **proiezione trace**, scaglionata per regola C (§0.4, §7) | A (2) |
| Q20 | nessun dato lascia la macchina | ⚠️ parziale | la metà statica è §7.4.2, riga V25 · Q20: il controllo gira, la sonda scatta, **la contro-sonda non esiste** · il test «assenza di traffico a default» è eseguibile ma **vacuo** finché nessuno può generarne — gotcha #17 | B (3) |
| Q21 | ripristino da backup su una macchina nuova | ⏳ rimandato | il metodo di `design/08` è *«backup e ripristino su ambiente pulito»*, e qui non esiste né l'uno né l'altro: il backup si scaglia per regola C (§0.4.1). ⚠️ la §0.6 lo elencava fra i «verificati solo lato kernel»: disallineamento trovato e corretto, §8.5.1 | F (11) |
| Q22 | annullare un passo che ha modificato file | ⚠️ parziale | la DST inietta la caduta durante la conservazione (§3.3) e il lato kernel — ambiti dichiarati, riferimento sul passo — entra; `filesystem` non ha implementazione reale (§8.2.2), quindi «l'ambito torna byte-identico» è provato su un albero in memoria | D (5) |
| Q23 | esecuzione sotto il livello 2 di confinamento | ⚠️ parziale | ⚠️ **Riletto il 2026-08-08: `design/08` chiede _due_ cose, non una** — *«statica: nessun percorso di esecuzione senza livello richiesto»* **più** un *«test negativo: con confinamento indisponibile l'azione non parte»*. **Nessuna delle due gira qui**, e per lo stesso motivo: nessuna porta esegue comandi (§7.4.5). Ciò che entra sono i **test a esempi** su tipo, dichiarazione per azione e registrazione nel giornale — che è V37, ed è meno di quanto `design/08` chiede | D (5) |
| Q24 | lettura di credenziali fuori dal gestore dei segreti | ⏳ rimandato | ⛔ **DECLASSATA DA ✅ IL 2026-08-27, finding AUD-026:** questa cella accreditava il livello 2 di §7.4.2, riga V34 · Q24, e quel controllo **non esiste in nessuna delle due direzioni** — vedi la riga V34 di §8.3 e il riquadro sotto la tabella di §7.4.2. Il metodo che `design/08` assegna a Q24 è proprio quella statica sul grafo delle crate, quindi §8.1.3 non lascia un secondo criterio da invocare | B (3) |

### 8.5 Cinque disallineamenti che la copertura ha trovato, e tutti chiusi

Nessuno era cercato. Sono emersi perché **giudicare sessantuno voci obbliga a leggere le
sezioni con criteri diversi** da quelli con cui sono state scritte, ed è il valore
principale di questa sezione oltre alla tabella.

| # | Cosa | Dove |
|---|---|---|
| 1 | il **backup** non era in perimetro, e la §0.6 diceva il contrario | §8.5.1 |
| 2 | **nessun sotto-progetto** della roadmap collocava il backup | §8.5.2 |
| 3 | il **livello 1 del catalogo** non enumerava tre proprietà che le §5 e §6 avevano già deciso | §8.5.3 |
| 4 | il catalogo non enumerava i **cinque controlli della §6.10.5** — ed è la seconda volta, con la stessa forma | §8.5.4 |
| 5 | **otto formulazioni di vincolo erano troncate** rispetto alla fonte, e su una di esse era stato dato un giudizio | §8.5.5 |

I primi due riguardano lo stesso oggetto — il **backup** — e non è un caso: era l'unica
cosa del progetto di cui nessuno era il proprietario, quindi l'unica che nessuna sezione
aveva motivo di nominare. Il terzo e il quarto sono di natura diversa, e li ha trovati la
§8 **contro sé stessa**, con la stessa regola e a distanza di un giorno. Il quinto è l'unico
che la §8 **non** poteva trovare da sola: sta fra questa tabella e la sua fonte, e per
vederlo bisogna aprire l'altro documento. Tutte le correzioni sono registrate al loro posto,
non applicate in silenzio.

#### 8.5.1 Il backup non era in perimetro, e la §0.6 diceva il contrario — ✅ chiuso

| | |
|---|---|
| **Cosa diceva §0.4** | la riga §10 metteva in perimetro *«il motore di persistenza; il lato kernel di segreti, confinamento e checkpoint»*. Il **backup non compariva**, né dentro né fuori |
| **Cosa diceva §0.6** | *«Q17, Q21, Q22, Q23 sono verificati solo lato kernel»* — cioè affermava che Q21 **ha** un lato kernel in perimetro |
| **Perché è successo** | la generalizzazione a quattro voci regge per Q17 (`secrets`), Q22 (checkpoint) e Q23 (confinamento), che hanno tutti un lato kernel elencato in §0.4. **Q21 vi era entrato per somiglianza**, non per un perimetro |

> **Risoluzione: vince la §0.4.** È l'autorità sul perimetro, ed è l'unica delle due resa
> falsificabile riga per riga dal criterio A-B-C della §0.3. Q21, V32 e V33 sono quindi
> **⏳ rimandati per intero**, non parziali.

**Entrambe le sezioni sono state corrette il 2026-08-07**, e la correzione è doppia perché
la causa era a monte:

| Dove | Correzione |
|---|---|
| **§0.4** | il backup entra nella colonna «si scaglia» della riga §10, con **regola C** e la sua giustificazione — §0.4.1. Senza, il perimetro restava incompleto rispetto alla propria regola di §0.3 |
| **§0.6** | Q21 passa dalla riga «verificati solo lato kernel» a quella dei **non verificati qui**, accanto a Q6, Q11, Q12 e Q16 |

Nessuna delle due riscrive la storia: la riga della §0.6 porta il proprio richiamo, e la
§0.4.1 dichiara di essere un'aggiunta. È la stessa postura degli ADR — una decisione
superata si marca, non si cancella.

⚠️ **Cosa questo episodio dice del metodo, e vale più della correzione.** Le §0.4 e §0.6
sono state scritte **nella stessa sessione** e rilette più volte, e la contraddizione è
sopravvissuta. È emersa solo quando qualcosa ha costretto a leggerle con un criterio
diverso — *«dammi lo stato di Q21»* — che è esattamente ciò che questa sezione fa
sessantuno volte. Una tabella di copertura non serve solo a non dimenticare: serve a
**rileggere con un'altra domanda**.

#### 8.5.2 Nessun sotto-progetto della roadmap collocava il backup — ✅ chiuso

V32, V33 e Q21 avevano una **condizione** d'innesco chiara — *esistono backup e
ripristino* — e **nessun numero**: la [roadmap](../../roadmap.md) non nominava il backup in
nessuno dei propri sotto-progetti. Era il caso esatto che la §0.6 temeva, trovato dal solo
fatto di dover riempire una colonna.

> ✅ **Chiuso il 2026-08-07: sotto-progetto 11 — Backup e ripristino**, che dipende da 5, 6
> e 9.

**L'ordine non è comodità, è non-vacuità.** V32 dice che il backup **esclude indici e pesi
perché ricostruibili**. Prima che il sotto-progetto 6 produca gli indici e il 9 i pesi,
l'elenco delle esclusioni è **vuoto**, e una verifica di V32 su un elenco vuoto è una prova
che non può fallire: il gotcha #17 nella forma già vista due volte in questa spec — M-8 sui
punti di crash (§4.7) e M-3 sul proprio grafo (§7.2.3).

Servono inoltre il filesystem reale, che arriva con il sotto-progetto 5, e l'interfaccia
che dichiara le esclusioni **al momento del backup** — il follow-up di
[ADR-0022](../../adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md), che dice di
farlo lì e non al ripristino.

⚠️ **Il costo dello scaglionamento resta, ed è dichiarato in §0.4.1:** fino ad allora
l'unico irriproducibile che cresce è il giornale, e l'unica protezione è una copia manuale
del suo file.

#### 8.5.3 Il catalogo non enumerava tre proprietà di livello 1 — ✅ chiuso

Questo l'ha trovato la §8 **contro sé stessa**, rileggendo la propria tabella con la regola
che si era appena data — la §8.1.2: *«la colonna del meccanismo nomina una voce della §7,
mai un'intenzione»*.

**Diciassette celle su sessantuno non la rispettavano.** Tolti i casi in cui il meccanismo
c'era ma il rimando era indiretto, restavano tre proprietà che il catalogo §7.4.1 **non
enumerava affatto**:

| Voce | La proprietà | Decisa in |
|---|---|---|
| **V2** | l'ammissione dell'arbitro **riceve un profilo**: senza, la chiamata non si scrive | §5.2.1 — *«il profilo che l'arbitro riceve»* |
| **V4** | l'esito è **a tre vie**, e chi chiama è obbligato a distinguerle | §5.3 punto 1 |
| **V10** | il sensore riceve l'artefatto **per riferimento immutabile** | §6.4.2 |

Tutte e tre erano già decise, tutte e tre sono di **livello 1**, e nessuna era nel catalogo.
La §8 poteva quindi solo sopravvalutarle — dichiarando un livello 1 che nessuno aveva
esaminato — o sottovalutarle chiamandole test a esempi, che è falso.

> **Risoluzione: entrano nel catalogo**, blocco C, ciascuna con la propria sonda e la propria
> contro-sonda. Non sono controlli *nuovi*: sono controlli che il catalogo aveva saltato.

**Il titolo del blocco C era stretto, e va corretto insieme.** Diceva «Tipi che non si
scambiano», ma **tre delle sue sei righe originali non erano scambi di tipo** — `InRevoca`
per un profilo non prelazionabile, l'ammissione che legge `cold_start`, l'effetto senza
classe. La colonna, che dice «cosa **non** deve compilare», era invece giusta dall'inizio:
il blocco è diventato **«Cosa non è esprimibile»**.

#### 8.5.3.1 E una voce è stata declassata

**V16** — *«il record di routing non contiene mai credenziali»* — era `parziale`. Rileggendo
la sua cella con la stessa regola, la metà che dichiaravo verificata non lo era: nessuna
credenziale attraversa il sistema in questo perimetro, quindi qualunque controllo
proverebbe **l'assenza di una cosa che non c'è**. È il gotcha #17 nella forma «prova vacua
che sembra un successo», ed è lo stesso motivo per cui V25 non ha una contro-sonda.

> **V16 passa a ⏳ rimandato**, innesco B. Che il tipo del record non abbia un campo per una
> credenziale resta una scelta di scrittura giusta — ma una scelta di scrittura non è un
> controllo, ed è esattamente la distinzione che la §7.0 esiste per fare.

**Cosa questo dice della regola §8.1.2.** Una regola che non rifiuta mai niente è
decorazione. Questa ha rifiutato tre voci del catalogo e una riga della propria tabella
**la prima volta che è stata applicata sul serio**, ed è la ragione per cui vale la pena
scriverla come regola invece che come buona intenzione.

⚠️ **Ciò che resta scoperto, e va detto:** lo script di §8.6 controlla che la casella del
meccanismo sia **piena**, non che nomini davvero una voce della §7. Questa terza scoperta è
venuta da una rilettura, non da un controllo automatico — e resta l'unico punto della §8
che dipende da chi legge.

#### 8.5.4 Il catalogo non enumerava i cinque controlli della §6.10.5 — ✅ chiuso

> ⚠️ **Trovato e chiuso il 2026-08-08**, riallineando la §8 alle sette voci. È la stessa
> forma di §8.5.3, e la §8 lo ha trovato di nuovo **contro sé stessa**, con la stessa regola.

La §6.10.5 — scritta con F1b e
[ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) — decide **cinque
controlli**, quattro di livello 1 e uno di livello 2, ciascuno con la propria sonda e la
propria contro-sonda **già scritte**. Nessuno dei cinque compariva nel catalogo §7.4.

Perché è un problema e non una sfumatura: la §8.1.2 ammette **tre** risposte nella colonna
del meccanismo, e una voce di sezione non è fra queste. La cella di `Q4` poteva quindi
nominare solo la campagna DST — che è vera, ed è ciò che la riga diceva — lasciando quattro
test di compilazione fallita **fuori dall'insieme nominato**. È il difetto che la §7.6.3
descrive al contrario: l'insieme si era ristretto, e nulla sarebbe diventato rosso.

**Perché è successo, e la causa è visibile in una tabella sola.** Tre sezioni della
riapertura hanno la stessa forma — una tabella «come si verifica» con sonde e contro-sonde:

| Sezione | Voce | Righe nel catalogo |
|---|---|---|
| §2.8.4 | F3 · ADR-0034 | ✅ due, aggiunte il 2026-08-07 |
| §4.9.4 | F2 · ADR-0036 | ✅ due, aggiunte il 2026-08-07 |
| §6.10.5 | F1b · ADR-0037 | ⛔ **nessuna** |

Due su tre le hanno aggiunte, una no, e **nessun documento dice perché**. Non è una
decisione presa: è un passaggio saltato.

> **Cercato prima di scrivere, perché una proposta può essere già caduta — gotcha #32.**
> [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) non nomina mai
> il catalogo; la chiusura di F1b in [`HANDOFF.md`](../../HANDOFF.md) elenca sette consegne
> e le colloca in §6.10.2, §6.10.3, §7.3.1, §7.4.6, §3.3, §6.10.1 e §5.2.2 — **mai in
> §7.4.1 o §7.4.2**; la §7.4.4 riduce tre voci, e sono `HashMap`, V28 e V5; la §7.4.5
> scaglia un gettone, ed è quello del **confinamento**. **La questione non era mai stata
> posta**, quindi non si sta rilitigando niente.

> **Risoluzione: i cinque entrano nel catalogo**, dove il blocco li vuole — due gettoni in
> §7.4.1 B, due voci non esprimibili in §7.4.1 C, un controllo esterno in §7.4.2. Come in
> §8.5.3, **non sono controlli nuovi**: sono controlli che il catalogo aveva saltato, e le
> righe **rimandano** a §6.10.5 invece di ridefinirlo — la forma che la §7.4.4 punto 2 ha
> già scelto per V28, proprio per non avere due posti da tenere allineati.

⛔ **Lo stato di `Q4` non cambia**, ed è la parte da non fraintendere. I quattro controlli di
livello 1 provano la forma **dalla nostra parte** del filo; ciò che manca a `Q4` è il worker
vero contro cui provare la conformità della finta, e nessuno dei cinque lo fornisce
(§6.10.5, limite dichiarato). La riga guadagna un meccanismo nominato, non un voto migliore.

**E ha ripagato una previsione, il che è il motivo per cui vale registrarlo.** La §8.5.3.1
chiudeva dicendo che la §8.1.2 *«resta l'unico punto della §8 che dipende da chi legge»*,
perché lo script controlla che la casella sia **piena**, non che nomini davvero una voce
della §7. Il buco previsto si è ripresentato **il giorno dopo**, sulla sezione successiva. Il
rimedio non è irrigidire lo script — §8.6.4 spiega perché non può — ma sapere che questa è
la classe di difetto che ricompare, e cercarla a ogni sezione nuova.

**Le righe nuove sono state provate dentro l'intervallo, non date per dentro.** Una riga
aggiunta **fuori** dai delimitatori `#### 7.4.1` e `#### 7.4.3` passerebbe in silenzio: lo
script non la vedrebbe e uscirebbe verde, che è il gotcha #26 applicato a un'aggiunta invece
che a una rinumerazione. Sonde eseguite il **2026-08-08** sulla spec **reale**, con lo script
**reale** e ripristino verificato:

| # | Sonda — *deve scattare* | Messaggio osservato |
|---|---|---|
| **S7** | si svuota la contro-sonda del gettone «leggere da un worker» (§7.4.1 B) | `row 2624 (**leggere** da un worker): empty counter-probe` |
| **S8** | si svuota la contro-sonda della riga «byte consumati» (§7.4.2) | `row 2692 (**Q4** · I5 · §6.10): empty counter-probe` |

> ✅ **Rimisurate il 2026-08-08, dopo il passaggio di `check-docs.sh` all'inglese** — stesso
> metodo delle diciotto di §8.6.3 e stessa ragione: riscrivere il messaggio a tavolino sarebbe
> stata *un'evidenza scritta prima della misura*, cioè il **gotcha #15**. Entrambe sono passate
> per la guardia di §8.6.3.1 — bersaglio unico e mutazione non vacua — e **C0** è stata
> rieseguita **prima** di esse: spec intatta, `OK — no inconsistencies.`, uscita 0.
>
> ⚠️ **Il primo tentativo era vacuo, e va registrato perché sembrava un successo.** Lanciava
> `bash` senza percorso e prendeva quello di **WSL**, assente su questa macchina: le sonde
> uscivano **1** — cioè «scattato» — per un `execvpe(/bin/bash) failed`. È §8.6.3.1 alla
> seconda occorrenza, ed è l'asserzione **sul messaggio** ad averlo colto, non quella
> sull'uscita: di messaggi non ne arrivava nessuno.

La contro-sonda è **C0** della §8.6.3, che qui vale invariata: la spec intatta esce verde.
Ripristino verificato — **`spec ripristinata byte-identica`**.

⛔ **Il numero di riga nel messaggio è quello del giorno, e invecchia** — basta un richiamo
inserito sopra per spostarlo, ed è successo mentre queste due sonde si scrivevano, e di nuovo
alla rimisurazione: `2566` e `2634` sono diventati `2624` e `2692`. Ciò che identifica la riga
è la **prima cella**, che il messaggio riporta fra parentesi apposta: è quella a non
spostarsi. Vale identico per i numeri della §8.6.3, anch'essi riportati a **oggi**.

⚠️ **Un secondo ritratto è risultato stantio nello stesso passaggio, ed è dichiarato dove
sta:** la §7.4.7 contava i test di compilazione fallita come *«una dozzina — tre nel blocco B
e nove nel C»*, cioè il ritratto di **prima** di ADR-0034, ADR-0036 e §6.10.5. Ricontato sulla
tabella: **cinque** in B e **quattordici** in C.

#### 8.5.5 Otto formulazioni erano troncate, e su una era stato dato un giudizio — ✅ chiuso

> ⚠️ **Trovato e chiuso il 2026-08-08**, in un audit sezione per sezione contro gli ADR e le
> fonti. È l'unico dei cinque che la §8 non poteva trovare da sola: sta **fra** questa
> tabella e il documento da cui copia.

La §8.1.3 nomina l'autorità sui vincoli con una parola sola: **il testo del vincolo**, che
vive nella [spec del kernel](2026-08-06-kernel-design.md). La colonna «Vincolo» di §8.3 è una
**riformulazione abbreviata**, ed è corta per costruzione. Confrontandola con la fonte riga
per riga, **otto** delle trentasette avevano perso un pezzo:

| V | Cosa era caduto | Perché conta |
|---|---|---|
| **V16** | *«nomi di provider e parametri **sì**»* | è la metà **positiva** del vincolo, ed è **verificabile qui** |
| **V5** | *«l'assenza vale `irripetibile`»* | senza, il caso «classe assente a runtime» — un record riletto da una versione precedente — resta senza regola |
| **V36** | *«**non sono coperti dal checkpoint** e…»* | senza, un rollback che lascia intatto un file fuori ambito soddisfa la formulazione |
| **V30** | *«**prima** dell'implementazione»* | è l'unica cosa che impedisce al metodo di essere ritagliato sul risultato |
| **V25** | *«nessuna telemetria **lascia la macchina**»* | senza, vieta anche la **raccolta**, che V24 invece pretende: era un **allargamento**, non un taglio |
| **V34** | *«il **gestore dei segreti** è…»* | perdeva il soggetto proprio nella riga che deve difenderlo |
| **V31** | *«**come caso di regressione**»* | «conservato in un elenco» e «rigira a ogni commit» diventano indistinguibili |
| **V28** | *«verificabile staticamente»* | e la riga V34 accanto lo aveva **tenuto**, quindi non cadeva per brevità |

⛔ **Sette erano innocue. Una no, e aveva già prodotto un giudizio sbagliato.**

> **`V16` torna a ⚠️ `parziale`.** Il declassamento di §8.5.3.1 era **corretto sulla metà che
> aveva davanti** — provare che nessuna credenziale compare, dove nessuna credenziale passa,
> è vacuo (gotcha #17), e resta vero. Ma la metà **positiva** — che il record *porti* nomi di
> provider e parametri — non era nella colonna, quindi non è entrata nel giudizio. Quella
> metà è verificata qui, dallo **stesso test a esempi su giornale sintetico** che rende ✅ V15
> e Q14. Un vincolo con una metà verificata e una vacua è `parziale`: è la definizione di
> §8.1.

⚠️ **Il conteggio torna a «tredici», e non è un ripensamento.** Il 2026-08-08 §8.8 era stata
corretta da «tredici» a «dodici» perché nessuno aveva ricontato dopo il declassamento di
`V16`. Il numero era giusto per la tabella di allora; ora la tabella è cambiata di nuovo, per
una ragione diversa e registrata. **Il ritratto va riletto sulla tabella ogni volta**, che è
esattamente ciò che la regola dice — e questo è il caso in cui obbedirle produce lo stesso
numero di partenza per un motivo nuovo.

**È il gotcha #29, spostato dalle invarianti alla tabella che le giudica.** Quella regola dice
che *«la riga di verifica di un'invariante è il punto in cui l'invariante si restringe in
silenzio»*. Qui non è la riga di verifica ma la **riformulazione**, e l'esito è peggiore: una
riga di verifica troppo stretta lascia scoperto un caso, una riformulazione troppo stretta
**cambia l'oggetto del giudizio**. ⛔ Nessuno script può accorgersene — la colonna è piena e
lo stato è nell'insieme chiuso — ed è la stessa condizione dichiarata in §8.5.3.1 e §8.8: la
§8 ha un punto che dipende da chi rilegge, e ora si sa che sono **due**, non uno.

📌 **Il rimedio, che è il pezzo utile:** quando si tocca la §8, la colonna «Vincolo» si
**confronta con la fonte**, non si rilegge da sola. Costa il tempo di aprire un file di
quarantatré kilobyte, e questa volta ha ribaltato un verdetto.

### 8.6 L'estensione di `check-docs.sh`

#### 8.6.0 A parole

Lo script oggi fa una cosa sola, in cinque modi diversi: **prende un elenco da un posto, un
elenco da un altro, e segnala la differenza.** Quanti file ADR contro quante voci d'indice.
Quali link puntano a file che non esistono. Quali Q non hanno un metodo in `design/08` —
che è V30, ed è la stessa forma di controllo che questa sezione aggiunge.

Le due estensioni non introducono un meccanismo nuovo: applicano quello che c'è a due
tabelle che finora nessuno leggeva. È il motivo per cui costano poche righe, ed è anche il
motivo per cui non c'era una ragione per non farle.

#### 8.6.1 Le sei asserzioni

| # | Asserzione | Chiude |
|---|---|---|
| 1 | ogni riga dei controlli §7.4 ha **l'ultima casella piena** — la contro-sonda | §7.7.1 |
| 2 | **tutti** i V1–V37 e Q1–Q24 compaiono nelle tabelle §8.3 e §8.4, una volta ciascuno | §0.6 |
| 3 | lo **stato** appartiene all'insieme chiuso dei quattro | §8.1 |
| 4 | `parziale` e `rimandato` portano un **innesco** non vuoto | §8.1.1 |
| 5 | la riga ha **cinque colonne** — meno di cinque e le asserzioni 3 e 4 non hanno dove leggere | §8.3 · §8.4 |
| 6 | ogni riga dei controlli §7.4 ha la casella **«Difende»** che nomina un `V`, un'`I`, un `Q` **o** una voce del catalogo | §7.1.1 regola 1 |

> ✅ **La sesta è aggiunta il 2026-08-08, allo script e a questo elenco**, chiudendo la §7.1.1.
> Fa per la **regola 1** ciò che la prima fa per la regola 3: la toglie dalle intenzioni. Tre
> cose vanno dette, perché nessuna è ovvia.
>
> | | |
> |---|---|
> | **gira nella stessa passata della prima** | i delimitatori del catalogo si scrivono **una volta**. Due copie sarebbero due posti da tenere allineati, e il primo che smette mente in silenzio — è l'argomento di §7.4.4 punto 2 applicato allo script |
> | ⛔ **legge la colonna per intestazione, non per posizione** | «Difende» è la **prima** colonna nei blocchi A e C e in §7.4.2, ma la **terza** nel blocco B dei gettoni. Un controllo posizionale avrebbe giudicato la colonna sbagliata su cinque righe. È la trappola 3 di questo script, e qui non c'era ragione di ereditarla |
> | **nel confronto non entra il carattere `§`** | è multibyte, e il byte-matching dipende dal locale: stessa ragione per cui l'asserzione 3 riconosce gli stati da una **parola** e non da un'emoji. Il ramo 1b si riconosce da `7.4.` |
>
> ⚠️ **Vale anche per lei il limite dell'asserzione 1, ed è lo stesso di §8.6.4:** prova che
> la casella **nomini** qualcosa della forma giusta, non che l'attribuzione sia **vera**. Chi
> scrive `§7.4.1` accanto a un lint passa.

> ⚠️ **La quinta è stata aggiunta all'elenco il 2026-08-08, non allo script.** Lo script la
> esegue da sempre — `if (n < 7) { printf "%s: the row does not have the five columns" }` — ma
> questa tabella si presentava come inventario chiuso e ne elencava quattro. Va detto anche
> il suo effetto collaterale, perché non è ovvio: su una riga malformata il controllo **si
> ferma lì**, quindi le asserzioni 3 e 4 su quella riga non girano. Il rosso resta rumoroso,
> non silenzioso, e per questo non è una guardia di non-vacuità mancante.

La 2 è insieme completezza e non-duplicazione: una voce mancante e una voce scritta due
volte con stati diversi sono lo stesso difetto — *la tabella non giudica* — e producono lo
stesso rosso.

#### 8.6.2 Il problema vero, che non è nessuna di quelle

Come fa lo script a sapere **dove finisce** il catalogo e **dove comincia** la §8? Delimita
per intestazione di sezione. E se un giorno qualcuno rinumera, l'intervallo non trova più
niente — e uno script che non trova niente da controllare **esce verde**.

Sarebbe un controllo che smette di controllare senza dirlo: il gotcha #14 applicato allo
script stesso, e la forma più insidiosa perché il segnale è un successo.

> **Guardia di non-vacuità.** Se un'intestazione delimitatrice non si trova, o l'intervallo
> restituisce **zero righe**, è un fallimento. Non si passa in silenzio.

È la stessa postura della §7.3.1 — *«se il grafo completo e quello spedito coincidono, il
filtro non sta distinguendo niente: il controllo lo segnala»* — ed è la decisione più
importante dell'intera estensione.

**Il numero atteso non si scrive.** Un `19` o un `21` messo a guardia diventerebbe rosso il
giorno in cui il catalogo cresce **per un motivo legittimo**: è il gotcha #9 applicato allo
script, cioè un fallimento per la ragione sbagliata. La guardia verifica che i delimitatori
esistano e che le righe siano più di zero; per la §8 la completezza è già garantita
dall'asserzione 2, che è un elenco canonico e quindi non ha questo problema.

> ⚠️ **Un limite dell'intervallo, dichiarato il 2026-08-08 invece di lasciarlo scoprire.**
> Per la §8 l'awk apre su `## 8. ` e **non chiude mai**: l'intervallo reale è «da lì a fine
> file», non «§8.3 e §8.4». Oggi non produce falsi rossi solo per un dettaglio tipografico
> — le altre tabelle della §8 scrivono gli identificativi in **grassetto** (`| **V2** |`) e
> la regex, che pretende `| V2 |`, non li vede. Chi togliesse il grassetto a una di quelle
> celle otterrebbe `V2 appears more than once`, cioè un rosso **per la ragione sbagliata**:
> il gotcha #9 applicato allo script, la stessa cosa che questa sottosezione rifiuta di fare
> col numero atteso. ⛔ **Non si irrigidisce la regex**, che diventerebbe fragile in un altro
> modo: si sa che le tabelle diverse da §8.3 e §8.4 scrivono gli ID in grassetto, ed è una
> convenzione da rispettare, non un caso.

#### 8.6.3 Le sonde, in due direzioni

Eseguite il **2026-08-07**. Ogni sonda muta la spec **reale** e lancia lo script **reale**:
provare una copia del codice invece del codice sarebbe il gotcha #14 nella sua forma più
sottile. Le mutazioni sono transitorie, ripristinate da una copia tenuta nello scratchpad,
e il ripristino è verificato byte per byte alla fine — **`spec ripristinata byte-identica`**.

Ogni riga verifica **due** cose, non una: che l'uscita sia quella attesa, **e** che il
messaggio nomini il colpevole. La seconda è ciò che ha smascherato una sonda vacua, sotto.

> ✅ **Rimisurate tutte il 2026-08-08, dopo il passaggio di `check-docs.sh` all'inglese** —
> la §1.0 impone l'inglese al codice, e il proprietario ha esteso la regola agli script di
> servizio. Ogni messaggio citato qui sarebbe diventato **falso**, e riscriverlo a tavolino
> sarebbe stato *un'evidenza scritta prima della misura*, cioè il **gotcha #15**. Le
> diciotto sonde sono state **rieseguite** con lo stesso metodo e la stessa guardia; le
> colonne sotto riportano ciò che si è visto, non ciò che ci si aspettava.
>
> | | |
> |---|---|
> | **il comportamento non è cambiato, e non è un'affermazione sulla parola** | ogni mutazione è passata per **entrambi** gli script — quello di prima e quello tradotto — confrontati su uscita, numero di fallimenti e payload: numeri di riga, identificativi e valori fra guillemet. **Diciotto su diciotto identici**, il rosso di trentasei righe di S6b compreso |
> | **i numeri di riga sono quelli di oggi** | S1 e S1b dicevano `riga 1778`, che era la posizione della voce il 2026-08-07. La voce è la stessa — `V29 · gotcha #12` — e oggi sta alla **2609**: è l'invecchiamento che §8.5.4 aveva già dichiarato. Le quattro della sesta asserzione ricadono invece sulle **stesse righe** di allora |
> | ⛔ **le parole che lo script _cerca_ restano italiane** | `verificato qui`, `parziale`, `rimandato`, `non controllato` e l'intestazione `Difende` sono **contenuto della documentazione**, non messaggi: tradurle avrebbe reso rosse o, peggio, **vacue** le asserzioni 2, 3, 4 e 6. L'uscita di S5 è mista per costruzione, ed è giusto così |
> | **la guardia di non-vacuità ha fermato una sonda** | il bersaglio di S7c, indicato per prefisso, nella spec compare **tre volte** — anche in §6.3.1 e §6.10.1, che ripetono la stessa riga del dispositivo. La sonda si è **fermata** invece di mutare la tabella sbagliata, e il bersaglio è stato ristretto alla riga intera: è §8.6.3.1 che funziona alla seconda occasione |

| # | Sonda — *deve scattare* | Messaggio osservato |
|---|---|---|
| **S1** | si svuota la casella contro-sonda di una voce | `row 2609 (V29 · gotcha #12): empty counter-probe` |
| **S1b** | si toglie del tutto la colonna | `row 2609 (V29 · gotcha #12): counter-probe column missing` |
| **S2** | si toglie lo stato a una riga della §8 | `V13: state not allowed — «»` |
| **S3** | si cancella la riga `V13` | `missing row for V13` |
| **S3b** | si duplica la riga `V13` | `V13 appears more than once` — ⚠️ **e il richiamo sull'accento decade**: il messaggio italiano scriveva `piu`, e la trascrizione accentata lo rendeva l'unico dei tredici non riproducibile carattere per carattere. In inglese il problema non è stato risolto, è **sparito con la lingua** |
| **S4** | si scrive uno stato fuori dall'insieme | `V13: state not allowed — «🟡 in corso»` |
| **S4b** | si scrive uno stato **ambiguo**, con due parole dell'insieme | `V13: ambiguous state — «⏳ parziale e rimandato»` |
| **S5** | `rimandato` senza innesco | `V13: «⏳ rimandato» with no trigger` |
| **S6** | si rinomina `#### 7.4.1`: l'intervallo del catalogo si svuota | `delimiter «#### 7.4.1» not found`, **e con lui le due guardie**: `no catalogue row in the range: the check would be vacuous` · `no Difende cell read: rule 1 would check nothing`. Tre rossi, non uno |
| **S6b** | si rinomina `#### 7.4.3` | `delimiter «#### 7.4.3» not found` — ⚠️ **ultimo di trentasei**: senza chiusura l'intervallo arriva a fine file, e ogni tabella a valle produce un `catalogue table with no Difende column`. Il rosso è **rumoroso, non silenzioso**, ed è esattamente ciò che la §8.6.2 chiede |
| **S6c** | si rinomina `## 8.` | `delimiter «## 8.» not found` |

**Le quattro della sesta asserzione, eseguite il 2026-08-08** con lo stesso metodo, e con una
guardia in più presa da §8.6.3.1: la mutazione deve applicarsi **esattamente una volta**, o la
sonda si ferma invece di produrre il rosso di un'altra.

| # | Sonda — *deve scattare* | Messaggio osservato |
|---|---|---|
| **S7** | una casella «Difende» che non nomina niente di ammesso | `row 2631: «Difende» = «§5.1» names neither a V, an I or a Q, nor a catalogue entry (rule 1)` |
| **S7b** | la colonna «Difende» sparisce dall'intestazione | `row 2629: catalogue table with no Difende column` |
| **S7c** | ⛔ casella invalida nel **blocco B**, dove «Difende» è la **terza** colonna | `row 2620: «Difende» = «§6.3» names neither a V, an I or a Q, nor a catalogue entry (rule 1)` |
| **S7d** | una riga **1b** perde il riferimento al catalogo | `row 2683: «Difende» = «supply chain» names neither a V, an I or a Q, nor a catalogue entry (rule 1)` |

| # | Contro-sonda — *deve restare verde* | Osservato |
|---|---|---|
| **C0** | la spec intatta — con la casella di V25 che **dichiara** l'assenza, e con tutte le righe ✅ che non hanno innesco | ✅ verde — `OK — no inconsistencies.`, uscita 0 |
| **C5** | `verificato qui` **con** un innesco: è lecito, non obbligatorio | ✅ verde — idem, con l'innesco di V1 portato da `—` ad `A (2)` |
| **C6** | la spec intatta dopo la chiusura di §7.1.1: **trentaquattro** righe di catalogo, comprese le **quattro 1b** e le **cinque del blocco B** | ✅ verde — idem |

> ⚠️ **C6 diceva «trentatré righe e tre 1b», ed è stato ricontato invece che ricopiato.** Oggi
> sono **trentaquattro**, con **quattro** righe di ramo 1b: la §7.4.2 ha guadagnato la riga
> degli attributi — gotcha #36 — dopo la campagna della sesta asserzione. I blocchi sono
> `A 3 · B 5 · C 14 · §7.4.2 12`. ⛔ Il conteggio **non è a guardia dello script**, e non deve
> diventarlo: §8.6.2 dice perché. È il ritratto di cosa C6 stava attraversando.

⛔ **S7c è la sonda decisiva, ed è l'unica che non si sarebbe scritta da sé.** Prova che la
lettura **per intestazione** legge davvero la terza colonna e non la prima. Senza di lei
resterebbe il dubbio che il verde del blocco B venga dal caso: la contro-sonda C6 esclude che
lo script legga la colonna 1, 4 o 5 — su ciascuna di quelle il blocco B sarebbe **tutto
rosso** — ma solo S7c mostra che la colonna letta reagisce.

📌 **E la sonda «deve scattare» della sesta asserzione ha un caso storico, non costruito:**
alla sua **prima** corsa, prima di qualunque correzione, ha nominato **le otto righe e solo
le otto** — 8 rosse su 33, 25 verdi. È la prova nelle due direzioni del gotcha #24 ottenuta
da un giro solo, e sul difetto vero invece che su una mutazione.

**S5 e S6 sono le due che di solito non si scrivono, e sono le più importanti.** S5
impedisce che l'innesco diventi obbligatorio *ovunque*: un ✅ non ne ha bisogno, e un
controllo troppo largo è il gotcha #24 nella sua forma pura — insegna a ignorare l'audit.
S6 è la guardia di §8.6.2, e senza di essa l'intera estensione può spegnersi in silenzio.

#### 8.6.3.1 Una sonda era vacua, e va registrato

> **La prima versione di S1b non provava niente.** Applicava la sua mutazione a una stringa
> che la sonda precedente aveva già consumato, quindi la riga restava com'era: lo script
> usciva rosso, ma **per il guasto di S1**, non per il suo. Un successo apparente.

È il gotcha #17 nella sua forma esatta — *iniettare un guasto dove il codice non arriva è
una prova vacua che sembra un successo* — la stessa classe di errore che M-8 aveva commesso
sui punti di crash di `redb` (§4.7) e M-3 sul proprio grafo di prova (§7.2.3).

**Cosa l'ha intercettata:** non l'uscita, che era giusta, ma l'asserzione **sul messaggio**.
Una sonda che si accontenta del codice di uscita non distingue «è scattato per il mio
guasto» da «è scattato per quello di prima». Riscritta ricostruendo la riga con tre celle
invece di quattro, e verificata mostrando la riga **prima e dopo** la mutazione.

Esito finale: **tredici sonde su tredici**, con il ripristino byte-identico.

> ✅ **Ricontato il 2026-08-08, chiudendo la §7.1.1.** Con le quattro sonde e la contro-sonda
> della sesta asserzione il totale è **diciotto su diciotto**, sempre con ripristino
> byte-identico. «Tredici» resta scritto sopra perché è l'esito della campagna del
> **2026-08-07**, che è ciò che quella riga racconta.

#### 8.6.4 Cosa questa estensione non prova

| Non prova | Perché |
|---|---|
| che la contro-sonda **esista** davvero | verifica che la casella sia **piena**. Chi scrive `n/a` passa. È la stessa classe della riga di ADR-0031 — *«limita la superficie, non la certifica»* |
| che lo **stato dichiarato sia vero** | uno stato è un giudizio nostro; lo script controlla che sia *espresso*, non che sia *giusto* |
| che l'**innesco sia il sotto-progetto giusto** | §8.2.1 mette la condizione prima del numero proprio perché il numero non è verificabile |

⚠️ **La casella di V25 mostra dove passa il confine, ed è il caso da non «correggere».** Il
suo contenuto è `⚠️ non esiste ancora — vedi sotto`, e **deve passare**: è una
dichiarazione, che è esattamente ciò che la regola 3 della §7.1.1 chiede. Un controllo che
pretendesse una contro-sonda *funzionante* trasformerebbe una dichiarazione onesta in un
rosso, e insegnerebbe a cancellarla.

### 8.7 Cosa questa tabella non prova

Il perimetro negativo, come in §0.2 e §7.6.

| # | |
|---|---|
| 1 | **Non prova che il kernel sia corretto.** Prova che ogni V e ogni Q è stato **giudicato**. È la §7.6.3 applicata a sé stessa: un difetto che non viola nessun V passa verde anche qui |
| 2 | **Un ✅ significa «esiste un controllo che è stato visto fallire e visto restare verde»**, non «questo non può andare storto» |
| 3 | **Non sostituisce la percentuale di copertura, e non la vuole.** §7.6.2 lo dichiara già: il criterio è «ogni V ha un controllo», non «l'X % delle righe» — e una copertura alta con invarianti non verificate è la falsa sicurezza peggiore |
| 4 | **Gli stati sono dichiarazioni nostre.** Lo script li controlla come forma; a controllarli come sostanza c'è solo chi rilegge |

#### 8.7.1 Il livello ⛔ è vuoto, e non è una svista

**Nessuna delle sessantuno voci è `non controllato`.** Ogni V e ogni Q in perimetro ha un
controllo, o un innesco che dice chi glielo darà.

⚠️ **Non contraddice la §7.6.2, e la differenza va detta perché sembra una.** Quella
sezione elenca Q6, Q11, Q12 e Q16 sotto il titolo *«dentro il perimetro, e non controllato
per scelta»*. Ma la sua stessa colonna «cosa lo copre invece» dice **«la §8, con il
sotto-progetto che li chiude»**: sta dichiarando che *la porta* non li controlla, non che
nessuno lo farà mai. Qui sono **⏳ rimandati**, che è la traduzione esatta di quella frase.
Il valore ⛔ significa un'altra cosa — *si sceglie di non controllarlo, e nessun innesco lo
riaprirà* — e per questo pretende un motivo invece di un innesco.

È lo stesso esito della §7.4.3, dove il livello 3 del catalogo è risultato vuoto, e ha la
stessa lettura: ciò che la §7.6.2 sceglie di **non** controllare non sono V interi, ma
**pezzi** di V — `HashMap` fuori dal kernel dentro V29, il tempo di parete dell'arbitro
dentro Q1 — e ciascuno è dichiarato nella colonna del meccanismo, dove chi legge la riga lo
incontra per forza.

Il valore resta nell'insieme chiuso apposta: il giorno in cui si sceglierà di non
controllare un V per intero, esiste già la casella in cui dirlo. Toglierlo obbligherebbe a
scrivere ✅ o a tacere, e sono i due modi in cui una rinuncia diventa invisibile.

### 8.8 I costi di questa sezione

| Costo | |
|---|---|
| **sessantuno righe da tenere allineate** | ogni V o Q nuovo, ogni cambio di perimetro, tocca questa tabella. Mitigato dall'asserzione 2: dimenticarla è rosso, non silenzio |
| **tredici V e otto Q sono `parziale`** | è circa un terzo del totale, ed è il ritratto onesto di un sotto-progetto che costruisce il kernel senza nessuno dei suoi consumatori. Il rischio è che `parziale` diventi la casella comoda: l'innesco obbligatorio è l'unica difesa, ed è di livello 2. ⚠️ **Ricontato sulla tabella due volte il 2026-08-08**, non dedotto. Il ritratto pieno è **diciotto ✅ · tredici ⚠️ · sei ⏳** per i V e **nove · otto · sette** per i Q. ⛔ **La storia del numero, perché altrimenti sembra un ripensamento:** diceva «tredici» dal giorno in cui fu scritto; il primo riconteggio lo portò a «dodici», perché nessuno aveva ricontato dopo il declassamento di `V16` in §8.5.3.1; il secondo lo riporta a «tredici», perché `V16` è tornato `parziale` quando si è visto che il declassamento aveva giudicato una **formulazione troncata** (§8.5.5). Stesso numero, tre tabelle diverse: è il motivo per cui **si riconta**, invece di fidarsi di ciò che c'è scritto |
| **gli inneschi invecchiano** | la condizione no, il numero fra parentesi sì. §8.2.1 sceglie quale delle due lo script può controllare — nessuna delle due — e quale un lettore può correggere: il numero |
| **tre sezioni approvate sono state corrette** | §0.4, §0.6 e il **catalogo §7.4** — la riga V31 in §7.4.2, le tre nuove in §7.4.1, e il 2026-08-08 le **cinque** della §6.10.5 più il ritratto ricontato in §7.4.7 (§8.5.4) — per disallineamenti che questa tabella ha trovato. Il costo non è la correzione ma il precedente: una sezione approvata non è congelata, e ogni riapertura va **registrata** invece che applicata, §8.5 |
| **una regola della §8 dipende da chi legge** | §8.1.2 — «il meccanismo nomina una voce della §7» — ha rifiutato tre voci del catalogo e una riga della tabella, e il giorno dopo altre **cinque** voci (§8.5.4), ma **nessuno script la applica**: lo script verifica che la casella sia piena, non che nomini davvero una voce. È l'unico punto della §8 nella condizione in cui era la §7.7.1 prima di questa sezione — ed è **la classe di difetto che ricompare**, non un incidente |
| **lo script controlla la forma, non la sostanza** | §8.6.4. Sposta il confine di ciò di cui ci si può fidare, non lo elimina — è la stessa riga della §7.7 |
