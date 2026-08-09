# Handoff — ripresa del progetto

Aggiornato il **2026-08-09**, alla **chiusura del Traguardo 1** del sotto-progetto 1 — lo
scheletro delle cinque crate e la porta di qualità, eseguibile in un comando solo e provata
in due direzioni. Serve a riprendere senza rifare, e senza rilitigare ciò che è già deciso.

> 📍 **Punto di ripresa: la testa del ramo `spec/sottoprogetto-1-kernel`** — la spec del
> sotto-progetto 1 è **completa e senza voci aperte**, e il **Traguardo 1 è eseguito**.
> Il piano è stato percorso compito per compito, **subagent-driven**, con revisione fra uno
> e l'altro; poi quattro compiti di riallineamento hanno riportato in **inglese** tutto il
> codice, che il piano dettava in italiano contro la §1.0 della spec.
>
> ✅ **Il codice del prodotto non è più zero righe.** Esiste il workspace alla radice con le
> cinque crate — `kernel` · `platform` · `secrets` · `simulator` · `daemon` — edition
> **2024**, `rust-toolchain.toml` che appunta **rustc 1.95.0** e il bersaglio
> **`x86_64-unknown-none`**. `kernel` e `simulator` sono `#![no_std]` + `alloc` +
> `#![forbid(unsafe_code)]` e **non contengono nessuna logica di prodotto**: è deliberato,
> non un lavoro lasciato a metà.
>
> ✅ **La porta di qualità gira in un comando solo** — `bash scripts/gate.sh` — con **sei**
> controlli: build del workspace · test · cancello senza OS · allow-list sui due grafi ·
> attributi delle crate vincolate · coerenza della documentazione. **`GATE GREEN`.** La CI
> lancia lo stesso identico comando, `.github/workflows/quality-gate.yml`. La mappa
> riga-per-riga sul catalogo §7.4 è in [`porta-di-qualita.md`](porta-di-qualita.md), e dice
> anche ciò che **non** è coperto.
>
> ⛔ **Il quinto controllo copre due righe del catalogo, non una — dal 2026-08-09.** Oltre
> agli attributi, `gate-attributes.sh` verifica che le crate vincolate **non abbiano un
> build script**: una lacuna che una revisione ha **misurato**, non temuto — un
> `crates/kernel/build.rs` lasciava la porta verde su **sei su sei**. Il catalogo §7.4.2
> passa da **dodici a tredici** voci di livello 2. Dettagli in `porta-di-qualita.md`.
>
> ⚠️ **Quattro gotcha nuovi, tutti misurati eseguendo** — **#38**, **#39**, **#40**, **#41** —
> più una **seconda occorrenza** di #26 e una di #25. Il #39 e il #41 hanno prodotto codice:
> `scripts/gate-attributes.sh` e la classe di caratteri di `gate-deps.sh`. Il testo integrale
> è nella tabella dei gotcha.
>
> ✅ **Nessuna voce aperta resta nella spec.** L'unica decisione ancora `Proposed` del
> progetto è **ADR-0029**, il guscio della GUI, che non tocca il sotto-progetto 1.
>
> ⏭️ **Il prossimo passo è il Traguardo 2 — il substrato iniettabile**, e il suo piano
> ✅ **è scritto** (2026-08-09):
> [Traguardo 2](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md).
> Quattordici compiti, in due parti: il substrato che la §2 progetta per intero, poi le
> altre cinque famiglie di porte come tratti. **Resta da eseguirlo.**
>
> ⚠️ Deliberatamente **senza SHA**: un hash scritto dentro il file che quel commit contiene
> nasce già vecchio di uno. Lo SHA sta nel messaggio di delega, dove è vero nel momento in
> cui si legge.

## In trenta secondi

Assistente desktop locale, utente singolo, GPU singola RTX 5080 16 GB. **Piattaforma a
quattro pilastri paritari** su kernel comune. Spec del kernel **§0–§10 completa, 37 ADR**.
Stack deciso **tranne il guscio della GUI**: core in **Rust**, interfaccia web in **Vue 3**,
worker ML in **Python**; Tauri contro Electron è ancora aperto
([ADR-0029](adr/0029-guscio-della-gui.md), `Proposed`) e non blocca nulla.

**La spec del sotto-progetto 1 ha §0–§8 approvate**, è stata **riaperta su sette voci**
trovate rileggendo `tracciabilita.md` con una domanda che nessuno le aveva posto, e le sette
sono **tutte chiuse**. La **§8 è stata riallineata e chiusa** il 2026-08-08. Il **Traguardo 1
è eseguito** lo stesso giorno: workspace, cinque crate, porta di qualità verde, **zero logica
di prodotto**. Il **piano del Traguardo 2 è scritto** il 2026-08-09: tocca a eseguirlo.

✅ **La lacuna su I2 è chiusa.** La GPU usata dalla GUI è governata da
[ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md): **quota di presentazione
sottratta, con la concessione tenuta dal core.** Il kernel non ha più lacune aperte.

Il vincolo che governa tutto non è funzionale ma di risorsa: quattro aree che si contendono
una sola GPU.

⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
scrive **qui**, e vive in [`../crates/`](../crates/): cinque crate, con `kernel` e
`simulator` in `no_std`. Gli spike in [`../spikes/rust/`](../spikes/rust/) restano
**prove**, **fuori dal workspace** — e la §2.5 della spec dice riga per riga quali pezzi
saliranno a `crates/kernel/` e quali restano dove sono. Nel Traguardo 1 **non è ancora
salito niente**: sale col substrato.

## Prima cosa da fare

⏭️ **Eseguire il piano del Traguardo 2 — il substrato iniettabile**, che ✅ **è scritto**:
[`2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md`](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md).
Tempo, casualità, I/O, scheduling, l'esecutore nel `kernel`, e le sei porte come tratti.
Vale «spec prima del codice», e la spec c'era già: **§2** e **§3** del sotto-progetto 1.

> ⛔ **Non si riusa il piano del Traguardo 1**, e non si scrive un piano unico per i
> traguardi rimasti: scriverne uno per codice che non esiste ancora significa inventare.
> È la regola della §6 del compendio, e ha già retto alla prova due volte.
>
> **Come**: `superpowers:subagent-driven-development` — **un subagente fresco per
> compito**, con revisione fra uno e l'altro. La scelta è del proprietario ed è già presa.

> ⛔ **Il piano porta in testa il perimetro, e la voce che è stata cercata prima di
> fissarlo.** L'idea di far dichiarare al Traguardo 2 le sole porte con un consumatore
> immediato è stata cercata dove era già stata decisa — quattro documenti concordi — e la
> prova nuova ha giocato **contro**: la §3.3 inietta guasti su tutte e sei le porte, e la
> campagna è il Traguardo 4. Un tratto inesistente a quel punto significa che **C1 sarebbe
> verificato su un mondo più piccolo del reale**, gotcha #17. È registrato nel piano invece
> che dimenticato — gotcha #32.
>
> ⛔ **Leggere prima l'errata del piano del Traguardo 1**, che sta in testa al piano
> stesso. Quattro voci, e due valgono anche per i piani futuri: **E1** — il piano dettava
> identificatori **italiani**, e la §1.0 della spec vince; **E4** — la Definizione di
> «fatto» dava per verificata una condizione che i test non verificavano (gotcha #39).
>
> ⛔ **Cosa NON si fa ancora, e non è una dimenticanza:** non si scrive **nessun record del
> giornale**. Il vincolo 14 della §11 fa entrare i **byte congelati** nel repository *al
> primo record scritto*, e quel record appartiene al **Traguardo 3**. Scriverne uno ora
> congelerebbe un formato che la §4.9 non ha ancora messo alla prova, e ⛔ **i byte
> congelati non si rigenerano**.
>
> **Nessuna misura blocca**: l'unica aperta è **M5**, e richiede una GUI.

### Le sette voci, e come sono state trovate

`tracciabilita.md` risponde a *«dove vive questa funzionalità»*. Rileggendola con un'altra
domanda — ***«di quale meccanismo di kernel ha bisogno, e la spec lo nomina?»*** — sono
emerse sette voci che non reggono. È la stessa mossa della §8, applicata alla tracciabilità
invece che ai V/Q, ed è la terza volta che «rileggere con un'altra domanda» trova qualcosa.

⚠️ **La crepa sta nella legenda.** Un `📋` significa *«pianificata: sotto-progetto
assegnato»*, e **non** significa «non richiede un meccanismo di kernel». Tutto ciò che è
`📋` non era verificato su quel fronte.

| # | Voce | Voluta? | Classe §0.3 | Stato |
|---|---|---|---|---|
| **F3** | i **parametri di decisione** non erano consegnati al kernel | esplicita in §8.3 V3, ma con l'innesco della specie sbagliata | **B** | ✅ **chiusa** — [ADR-0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md), spec §2.8 |
| **F6** | la **VRAM totale** non aveva provenienza | implicita | conseguenza di F3 | ✅ **chiusa** con F3, spec §5.1 |
| **F5** | la porta `network` era descritta «verso i **provider**», V25 promette «un solo punto di uscita **verso la rete**» | implicita | una riga | ✅ **chiusa** — spec §2.3.1 |
| **F1** | nessuna porta per **parlare** con un worker: `process` era «avvio e uccisione» | implicita | **B** | ✅ **chiusa** — F1a con [ADR-0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) e §2.3.1; **F1b** con [ADR-0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) e §6.10 |
| **F2** | l'**evoluzione del formato durevole** del giornale non è decisa | implicita | **B** | ✅ **chiusa** — [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md), spec §4.9 |
| **F4** | l'**anello 3** non è collocato in §0.4, né dentro né fuori | implicita | **C** *e* **B** — scritta invece che assunta, si è **spaccata** | ✅ **chiusa** — §0.4.3 |
| **F7** | «il giornale lo consente» per **fork e branching** è un'affermazione della sola tracciabilità | implicita | converge in F2 | ✅ **chiusa con F2** — §4.9.5 |

### L'ordine, e perché

Il criterio **non** è «prima la scrittura»: non esiste ancora costruzione, sono tutte
scrittura. È:

> **Si decide prima ciò che vincola una firma, poi ciò che la descrive.** Una descrizione
> scritta prima della decisione che descrive va riscritta comunque.

```
✅ F3 ─▶ F6         F3 chiusa: F6 è caduta con lei
✅ F1a ─▶ F5        stessa tabella, toccata una volta sola. Il bivio su I4
                    è deciso: ADR-0035, «singolo» = per canale privato
✅ F2 ─▶ F7         chiuse insieme: F7 è un campo facoltativo con un
                    indice nuovo, e la regola di F2 lo rende meccanico
✅ F1b (§5–§6)      firme, messaggi, formato di filo. La propedeuticità su F2
                    era soddisfatta, e il formato l'ha deciso una misura
✅ F4               scritta invece che assunta, si è spaccata: C + B
✅ §8               per ultima, e UNA VOLTA SOLA — rispettato: chiusa 2026-08-08
```

Il test che ha deciso l'ordine: **quale, decisa per ultima, costringerebbe a riaprire le
altre?** F3 per ultima riapriva F1 e F2; F2 per ultima non riapre niente. Ma F2 ha una
scadenza **temporale** e non logica: va chiusa **prima della prima riga di codice che
scrive un record**, perché aggiungere un discriminante a record già su disco è una
migrazione dell'unico archivio irriproducibile.

> ✅ **Entrambe le scadenze sono state onorate (2026-08-07).** F2 è chiusa **prima** che
> esistesse un solo record durevole, quindi la decisione non è costata una migrazione —
> sarebbe costata quella. E F1b riparte con la regola già in vigore.

⚠️ **F2 precede F1b anche per una ragione logica, scoperta il 2026-08-07 sbagliando
l'ordine.** Il diagramma era stato letto come una preferenza, e non lo è: **F1b può creare
campi durevoli**. Il picco di VRAM che §5.2.2 fa entrare nel giornale *«accanto al passo»*
arriva dal worker, cioè da un messaggio che F1b progetta. Progettare quei messaggi prima
che F2 abbia dato la regola di evoluzione significa aggiungere campi a record **sotto
nessuna regola** — che è esattamente il modo di fallire per cui F2 esiste.

⚠️ **Tre propedeuticità di processo, che non sono fra le sette:** la §8 si tocca per
ultima e una volta sola, perché ognuna delle sette cambia una sua riga; **nessuna
rinumerazione** di sezioni, perché lo script legge §7.4 e §8 per posizione (gotcha #26);
e ogni correzione a una sezione approvata porta il proprio **richiamo datato**, come §8.5.

### F1b — ✅ chiusa. Cosa ha deciso, e cosa non rifare

✅ **Chiusa il 2026-08-08** con
[ADR-0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) e la **§6.10**. Le
sette cose che le erano state consegnate sono tutte scritte; qui resta ciò che serve a
**non ri-derivarle**.

| # | Cosa era | Dove è finita |
|---|---|---|
| 1 | firme e messaggi di `process` | **§6.10.2** — l'avvio restituisce il `Worker`, ed è l'unico modo di parlargli; `uccidi` lo **consuma**. Due tipi di ricevuta, non un enum |
| 2 | il formato di filo, e la misura che lo decide | **§6.10.3** — `minicbor`, codifica in `kernel`, porta a **byte**. ⛔ **L'esito B di M-1 non è scattato**: vedi sotto |
| 3 | l'allargamento delle giustificazioni | **§7.3.1** — ma **al contrario di come era istruito**: vedi sotto |
| 4 | la suite di conformità | **§7.4.6** — resta rimandata, e acquista l'affermazione sul **filo** oltre a quella sul ciclo di vita |
| 5 | quali guasti del dialogo verificano quale Q | **§3.3** — quattro righe, tutte su **Q4** per la regola di §8.2.2. Lo stato di Q4 **non cambia** |
| 6 | la tensione di `design/01` | **§6.10.1** — la scioglie il gettone della **ricevuta**: un frame non coperto è un guasto, non un dato |
| 7 | il picco di VRAM nel giornale | **§5.2.2** — campo facoltativo, indice nuovo, sotto la regola di §4.9 |

⛔ **Due cose sono andate diversamente da come erano istruite, e vanno lette.**

| | |
|---|---|
| **l'esito B di M-1 non è scattato, pur essendo «no» la risposta** | ADR-0035 prevedeva: se il pari Python non legge `bincode`, tipi in `kernel` e serializzazione in `daemon`. Il ripiego era stato prezzato **prima** che `minicbor` entrasse in `kernel` con ADR-0036, cioè il giorno prima. Con quella voce già spedita esiste una terza via che non c'era: il canale usa `minicbor`, la codifica **resta in `kernel`**, e la lista di ADR-0031 non cresce |
| **le giustificazioni allargate sono le altre** | l'istruzione diceva di allargare quelle di `bincode` — «serializza lo schema IPC» — dando per scontato che il canale worker ne ereditasse il formato. **Non si allargano**: `bincode` serve il solo canale gui, ed è la riga di `minicbor` a crescere. Divergenza registrata invece che allineata — gotcha #15 |

⚠️ **E una domanda che nessuno aveva posto è stata posta, con esito opposto all'atteso.**
Lo stesso criterio applicato all'**altro** canale privato — *il pari TypeScript sa leggere
`bincode`?* — ha risposto **sì** (M-11). Quindi §6.1.1 **non si tocca**, e i due canali
privati hanno formati diversi per una ragione **misurata**. Il tentativo di uniformarli è
il **gotcha #32**.

⚠️ **Un presupposto ereditato, che non era tale:** il follow-up di
[ADR-0028](adr/0028-ecosistema-dei-worker-ml.md) — *«trattare l'ambiente Python come
artefatto da versionare, non come prerequisito dell'utente»* — era una raccomandazione per
i sotto-progetti 9 e 10. Ora la regola 4 di ADR-0035 vi poggia: se l'ambiente del worker
non è artefatto nostro, il **timbro di build** non ha un'identità da confrontare e «non
versionato» **cade su quel canale**.

### F4 — ✅ chiusa. E «va scritta, non assunta» ha ripagato

✅ **Chiusa il 2026-08-08** con la **§0.4.3**, ed era l'ultima delle sette.

La classe attesa era **C**, e l'istruzione avvertiva di **scriverla invece di assumerla**.
Scrivendola si è spaccata in due, e **una metà è B**:

| Pezzo | Regola | Perché |
|---|---|---|
| il **registro dei trigger**, e l'apertura di una run da un evento | **C** | nessun consumatore finché non esiste una capacità L2 che parta da un evento. E la DST prova Q2, Q4 e Q5 aprendo le run direttamente: **non è che senza non prova niente** |
| che ogni **sorgente di eventi** entri da una porta **dichiarata**, e che si dica **quale** | **B** | §3.1 dichiara le porte *«esattamente quelle della §2.3»* e il simulatore le sostituisce **tutte**. Una sorgente scoperta dopo la campagna è una **porta aggiunta dopo la campagna**: C1 sarebbe stato verificato su un mondo più piccolo del reale, e **nulla sarebbe diventato rosso**. È F1a per intero, e il gotcha #17 |

**Le due righe della tracciabilità hanno ora una porta:**

| Sorgente | Porta | |
|---|---|---|
| *Scheduling* | `reactor` | ✅ già coperta: una scadenza è ciò che §3.2 modella, e in simulazione la decide il seme |
| *File watching* | `reactor` | ⬜ **dichiarata**, implementazione scaglionata — la postura di `network` |
| fine di un'altra run | nessuna | è **interna**: lo sa il giornale |
| utente | `ipc` | già lì |

**Perché su `reactor` e non su `filesystem`:** ciò che deve essere deterministico non è
*quale percorso*, ma **quando arriva la notifica** — ed è il contratto del reattore. Su
`filesystem` sarebbe una **direzione nuova**, e la sua finta dovrebbe generare eventi: più
macchina per meno determinismo.

✅ **Le famiglie restano sei.** Se ne fosse servita una settima, sarebbe stata regola B per
intero e sarebbe dovuta entrare adesso, come `process` in §2.3.1.

⚠️ **La lezione, che vale oltre F4.** È la **terza volta** che un pezzo sfugge alla §0.4
non essendo né entrato né scaglionato — dopo il backup (§0.4.1) e la configurazione
(§0.4.2). Non è distrazione: la tabella chiede «cosa entra» e «cosa si scaglia», e chi la
compila **non vede il terzo stato**. Chi troverà il quarto caso lo aggiunga qui.

### ✅ Cosa la §8 ha incassato — chiusa il 2026-08-08, toccata una volta sola

⛔ La §8 si toccava **per ultima e una volta sola**: ognuna delle sette voci le cambia una
riga, e toccarla sette volte sarebbero state sette occasioni di disallinearla. L'elenco si è
accumulato qui mano a mano, ed è stato incassato tutto insieme. **Resta come consuntivo**, e
la colonna «Stato» dice com'è finita — non si cancella, o la prossima riapertura riparte
senza sapere cosa era già stato deciso.

| Voce | Righe della §8 da rivedere | Stato |
|---|---|---|
| **F3** | **V3** — la metà rimandata non è più «la configurazione non ha consumatore» ma «l'archivio e il pannello non esistono»; l'innesco A resta valido per quella metà | ✅ già scritta |
| **F6** | nessuna: la provenienza del totale è un parametro, e la riga V3 la copre | — |
| **F5** | **V25** e **Q20** — la descrizione di `network` si è allargata, ma il buco della contro-sonda resta e lo **stato non cambia**. Da **rileggere**, non da riscrivere | ✅ **riletta, nessuna scrittura**: le due celle non dicevano «verso i provider», quindi erano già giuste. L'istruzione «rileggere, non riscrivere» ha retto alla lettera |
| **F1a** | **§8.2.2** — la riga `process` → Q4 resta 1:1, ed è ciò che il discriminante 3 di ADR-0035 verificava. Da **rileggere** | ✅ **1:1 confermato.** La cella `process` guadagna una riga sola: col dialogo, ciò che manca alla suite di conformità è cresciuto — anche il **filo**, non solo il ciclo di vita (§7.4.6) |
| **F2 + F7** | **Q14** — il meccanismo cresce: livello 1 (§7.4.1 blocco C) e byte congelati (§7.4.2). **Q5** — la porta `journal` scambia byte, quindi la campagna esercita davvero la codifica | ✅ **scritte entrambe.** `Q14` porta ora l'enum di versione e i byte congelati, e dice che un fork è un campo facoltativo con indice nuovo (§4.9.5); `Q5` dice che il crash cade **dentro** la scrittura. Nessuno dei due stato cambia: erano ✅ e restano ✅ |
| **F1b** | **Q4** — la §3.3 aggiunge **quattro** guasti del dialogo, tutti sulla porta `process`, e §7.4.6 acquista l'affermazione sul **filo** oltre a quella sul ciclo di vita. ⚠️ Lo **stato non cambia**: Q4 resta `parziale` con innesco **E (7)**, perché manca ancora il worker vero. Da **rileggere**, non da riscrivere | ⚠️ **riletta — ed era anche da scrivere.** Lo stato di `Q4` non cambia, come previsto. Ma la rilettura ha trovato che il **catalogo §7.4 non enumerava i cinque controlli della §6.10.5**: la cella non poteva nominarli senza violare §8.1.2. I cinque sono entrati, e il fatto è registrato in **§8.5.4** — gotcha **#36** |
| **F4** | ✅ **nessuna riga nuova.** **V29** copre già le sorgenti di eventi, e lo dice la sua stessa riga di verifica — *«C1 fallisce a ogni sorgente nascosta»*: una sorgente **dichiarata su una porta** è dentro quella frase, una non dichiarata è ciò che C1 fa fallire. Lo stato di V29 **non cambia** | ✅ **stabilito** |

✅ **Il ritratto è stato ricontato sulla tabella il 2026-08-08, non dedotto** — e **due
volte**, perché fra il primo e il secondo riconteggio l'audit ha cambiato uno stato.

| Momento | V | Q | Perché |
|---|---|---|---|
| prima | 13 ⚠️ dichiarati | 8 ⚠️ | numero scritto quando `V16` era ancora `parziale` |
| **1° riconteggio** | 18 ✅ · **12** ⚠️ · 7 ⏳ | 9 · 8 · 7 | nessuno aveva ricontato dopo il declassamento di `V16` in §8.5.3.1 |
| **2° riconteggio — vale questo** | 18 ✅ · **13** ⚠️ · 6 ⏳ | 9 · 8 · 7 | `V16` torna `parziale`: era stato giudicato su una **formulazione troncata**, §8.5.5 |

⛔ **Stesso numero di partenza, tre tabelle diverse.** Non è un ripensamento: è la
dimostrazione del perché la regola dice **ricontare sulla tabella** invece di fidarsi di ciò
che c'è scritto — se al primo riconteggio ci si fosse fermati al «tredici» già presente,
sarebbe stato giusto per il motivo sbagliato, e il secondo non sarebbe mai avvenuto.

⚠️ **E un secondo ritratto era stantio nella stessa famiglia:** la §7.4.7 contava i test di
compilazione fallita come «una dozzina — tre nel blocco B e nove nel C», il ritratto di prima
di ADR-0034, ADR-0036 e §6.10.5. Ricontato: **cinque** in B e **quattordici** in C. Due
conteggi stantii trovati nello stesso passaggio non sono una coincidenza — è la ragione per
cui la regola dice **ricontare**, non dedurre.

### ✅ L'audit sezione-contro-ADR — 2026-08-08. Quaranta rilievi

Dopo la chiusura della §8, la spec è stata passata al setaccio da **undici revisori in sola
lettura**: uno per sezione §0–§8, più due sulle **formulazioni** di V1–V37 e Q1–Q24 contro le
loro fonti. Ciascuno con l'obbligo di **citare entrambe le parti**, e con l'elenco dei falsi
positivi noti — un revisore che scatta dove non deve è il gotcha #24.

⛔ **Nessuna decisione è stata riaperta e nessun ADR superato.** Tre hanno ricevuto un
**rimando** (0021, 0032, 0035); tutte le correzioni portano il proprio richiamo datato.

| Cosa | Dove |
|---|---|
| ⛔ **`V16` ri-giudicato: torna a ⚠️ `parziale`** | la formulazione era troncata, e il declassamento aveva giudicato la metà rimasta — **§8.5.5**, gotcha #29 terza occorrenza |
| **otto formulazioni di vincolo troncate** | `V5` `V16` `V25` `V28` `V30` `V31` `V34` `V36` — §8.3, tutte ripristinate dalla fonte |
| ⛔ **due firme impossibili in §6.10.2** | `istruisci → Ricevuta` restituiva un tipo inesistente altrove: o l'enum vietato, o **nessun modo di ottenere un flusso**. Ora le istruzioni sono due |
| **`daemon` non monta il simulatore** | tabella §1.2 contro il grafo accanto; ADR-0034 dà ragione al grafo. **È la riga che il piano traduce in `Cargo.toml`** |
| **`network` stretto in §3.1** | F5 aveva allargato la gemella in §2.3 e non questa, nella tabella che si dichiara *«esattamente le porte della §2.3»* |
| **§9 mancava dalla tabella §0.4** | **quarto** caso del terzo stato, dopo backup, configurazione e anello 3. Risposta diversa: la §9 non è impianto, è il piano degli spike — si **dichiara**, non si colloca |
| **sei conteggi stantii** | §0.1 «tre ADR» → sette · §1.6 «due regole test» → tre · §2.8 «cinque ADR» → otto · §4.9.2 «cinque regole» → sei · §7.4.4 «due voci spedite» → tre · §7.7 «nove voci liv. 2» → undici |
| **§7.7.1 descriveva un'intenzione già realizzata** | la contro-sonda **è** verificata dallo script da due corse; la sottosezione lo diceva al futuro |
| **cadenza di `check-docs.sh` scritta in due modi** | la porta diceva «a ogni commit», lo script «a ogni chiusura». Allineata allo script, che è l'unico dei due eseguibile |
| ✅ **la voce lasciata aperta è stata chiusa il 2026-08-08** | **otto righe del catalogo** avevano «Difende» che non nominava un V, un'I o un Q. Non erano un problema solo: **cinque** ri-attribuite (`Q8` · `V29` · `Q2` · `I2` · `V29`, e `Q8` era **già scritta** in §8.4), **tre** difendono il **verdetto di altri controlli** invece di una proprietà — è il **ramo 1b** della regola 1. `V29` si allarga ai parametri, come ADR-0034 aveva già deciso. La regola non è più un'intenzione: **sesta asserzione** di `check-docs.sh`, rossa sulle otto e verde sulle altre venticinque alla **prima** corsa. Gotcha **#37** · **§7.1.1** |

📌 **Un falso positivo respinto, e vale registrarlo:** un revisore segnalava che «grafo delle
crate» in §7.1.2 fosse un meccanismo cancellato da §7.4.4 punto 2. **No**: quel punto toglie
il *driver di V28*, mentre il grafo delle crate resta il meccanismo reale di V34 · Q24. La
regola dei due lati citati ha retto — chi legge il rilievo può verificarlo in trenta secondi.

### Cosa ha chiuso F3, in tre righe

| | |
|---|---|
| **la regola** | nessuna decisione del kernel legge un parametro che non le è stato consegnato |
| **non è un quinto iniettabile** | i quattro di V29 sono sorgenti di *non determinismo*; un parametro è deterministico. Consegnarlo compra I3 **e** la variabilità sotto il seme, non la riproducibilità |
| **il guadagno che non c'era** | la DST può ora far variare i parametri col seme, quindi lo scenario di **RK-1** — quota audio + quota presentazione contro TRELLIS2 — diventa esplorabile *prima* che M5 lo misuri |

⛔ **Il limite di F3, dichiarato:** il compilatore **non può** vietare una costante scritta
dentro il kernel. Prova che una decisione **riceve** i propri parametri, non che non ne
abbia altri di nascosto — è il limite del gettone (§6.3.2). Lo copre solo la campagna, e
solo per i parametri che fa davvero variare.

### Cosa hanno chiuso F1a e F5, in quattro righe

| | |
|---|---|
| **il divario non era con I4** | era fra la §2.3 e `design/01`, che descriveva già il canale con tre verbi — *«Avvia, **istruisce**, uccide»* — e aggiungeva *«il flusso audio risale al core»*. La §2.3 ne aveva conservati due |
| **una porta sola, non due** | il dialogo entra in `process` invece di nascere accanto: l'oggetto con cui si parla a un worker è quello che restituisce l'avvio, e l'avvio pretende una concessione (§5.6). Spezzarli riaprirebbe la chiusura che ha portato I2 al **compilatore** |
| **la lettura di «singolo»** | un trasporto e uno schema **per canale privato**. I4 si **completa**, non si riformula: ADR-0004 riceve un rimando, come per I2 con ADR-0033 |
| **F5 non era una sfumatura** | `network` «verso i provider» avrebbe lasciato l'**esportazione OTLP** di ADR-0017 *fuori* dall'unico punto di uscita — cioè esattamente ciò che V25 vieta. Un secondo consumatore già deciso, e già escluso da una descrizione |

⛔ **Il limite di F1a, dichiarato:** resta aperta una domanda di stato dell'arte che **non
è stata misurata** — se `bincode` sia decodificabile dal pari **Python**. Non tocca la
dichiarazione della porta; tocca il **formato di filo**, che è F1b. Se la risposta è no,
vale l'esito **B** di M-1, già misurato e già prezzato: il confine di ADR-0031 non cresce.

⚠️ **E un follow-up di ADR-0028 è diventato un presupposto:** *«trattare l'ambiente Python
come artefatto da versionare»*. Il timbro di build regge sul secondo canale **solo se** il
worker è artefatto nostro; se non lo è, «non versionato» cade lì.

### F2 (+F7) — ✅ chiusa. Cosa ha deciso, e cosa non rifare

✅ **Stato: chiusa il 2026-08-07** —
[ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md) e spec **§4.9**. La
misura è fatta e le evidenze complete vivono nell'ADR; qui resta ciò che serve a **non
ri-derivarlo**.

**Il nodo era.** Il giornale è l'unico archivio irriproducibile, e chi lo rilegge non è chi
lo ha scritto: è lo stesso programma mesi dopo, con campi in più. Nessuna riga diceva cosa
succede in quell'istante.

**Perché la postura di I4 qui non è disponibile** — è F1a con la risposta rovesciata:

| | canale IPC | giornale |
|---|---|---|
| i due capi | **spediscono insieme** | lo **stesso programma in due momenti diversi** |
| se divergono | si rifiuta il pari stantio: **timbro di build** (§6.1.2) | ⛔ non si può rifiutare il passato |
| evoluzione dello schema | **rinunciata esplicitamente** | **obbligatoria** |

**Quattro cose già decise che la vincolano:**

| # | | Cosa impone |
|---|---|---|
| 1 | [ADR-0011](adr/0011-routing-risolto-e-giornalato-per-richiesta.md): il record porta la decisione **risolta**, non un rimando alla configurazione | i record sono già **auto-contenuti**: metà del problema è risolta, e per un altro motivo |
| 2 | [ADR-0018](adr/0018-ritenzione-a-livelli-del-giornale.md): un record potato **dichiara** di esserlo | esiste già un precedente di auto-descrizione **dentro** il record, e già una mutazione dopo la scrittura — la potatura riscrive |
| 3 | **§7.4.4 punto 3** | ⚠️ la spec **presuppone già l'evoluzione senza averla decisa**: il default `irripetibile` *«resta dov'è davvero utile — sui record riletti da un giornale scritto **prima che la classe esistesse**»*. È una regola di lettura in avanti per **un campo solo**, arrivata di straforo |
| 4 | [ADR-0032](adr/0032-motore-di-persistenza.md): `redb` conserva byte | la **codifica del record è nostra**: la decisione è interamente qui |

⚠️ **Il ritrovamento, ed è il pezzo da non perdere.** La §6.8 ha scartato `minicbor` perché
i suoi indici di campo *«servono all'evoluzione dello schema, cioè a un beneficio che I4
rinuncia esplicitamente»*. Quel giudizio è **giusto per lo schema IPC e rovesciato per il
giornale**, dove l'evoluzione non è un beneficio di cui fare a meno ma il requisito.

> ⛔ **La scorciatoia da rifiutare, e ha un nome:** *«usiamo `bincode` anche per il
> giornale, tanto è già nella lista di ADR-0031»* — cioè importare in un artefatto che
> **deve** evolvere una decisione presa dove l'evoluzione era stata **rinunciata**.

**Le quattro forme, e cosa la misura ne ha fatto.** Erano l'inventario aperto; ora due sono
cadute e la decisione è la **somma** delle altre due.

| | Regola | Esito |
|---|---|---|
| **A** | discriminante di **versione** nel record, e il lettore dispaccia | ✅ **adottata, ma non da sola** — sopra un formato posizionale la sua stessa enumerazione è posizionale, e la trappola **non si chiude**: misurato che `bincode` ignora i discriminanti espliciti |
| **B** | campi **auto-descritti** per indice | ✅ **adottata** — e costa **un byte su ventisei**, non «permanente su ogni campo»: la stima prezzava la codifica a mappa, la predefinita è ad **array** |
| **C** | disciplina **solo-append** | ⛔ **eliminata dalla misura, non da un giudizio.** Su un formato posizionale anche un campo *opzionale* in coda rende illeggibili i record vecchi: non compra nemmeno la compatibilità che promette. Dove funziona, funziona perché sotto c'è un indice — cioè **è B** |
| **D** | migrazione al riavvio | ⛔ resta l'uscita d'emergenza, con un ADR che ne dichiari il rischio. Non è la regola |

> ✅ **La decisione, in una riga:** *ogni record durevole dichiara la propria versione, e i
> suoi campi si identificano per indice esplicito.* La codifica vive in **`kernel`**, e la
> porta `journal` scambia **byte**.

**Le tre cose da non riscoprire:**

| # | |
|---|---|
| 1 | **Il modo di fallire peggiore non è l'errore: è il record che si rilegge e restituisce il numero sbagliato.** Cinque celle su trentasei sono ⛔ *silenzio*, e un banco che guarda solo `Ok`/`Err` le legge come successi — gotcha #30 |
| 2 | **Nessun formato che identifica per numero sopravvive alla rinumerazione.** Nemmeno quello per indice. Cambia la **visibilità**: in `bincode` rinumerare è *spostare una riga*; in `minicbor` è *scrivere un numero diverso*, e si legge nel diff |
| 3 | ⚠️ **Il costo che si paga non è dove sembrava.** Il grafo *spedito* cresce di una voce; quello **di build** passa da due a **sette**, e il kernel porta `syn` per la prima volta. È l'«evento da rivedere» di ADR-0031, e questa è la prima volta che si paga su quella classe |

**F7 è chiusa con F2**, §4.9.5: fork e branching sono un campo facoltativo con un indice
nuovo, ed è esattamente il caso che la misura dichiara ✅. Quali campi e con quale semantica
resta politica della capacità Conversazione, non del kernel.

⚠️ **Il piano deve decidere anche _dove nasce il workspace_.** Alla radice non c'è nessun
`Cargo.toml`: il workspace delle cinque crate nasce alla radice escludendo gli spike,
oppure accanto ad essi. È l'unica domanda strutturale che la spec ha deliberatamente
lasciato al piano — con **un fatto in più che nessun documento nominava**: sotto `spikes/`
i progetti Cargo sono **due**, non uno — `spikes/rust/`, che è anche un **workspace
annidato**, e `spikes/gui-ipc/`.

✅ **Nessuna misura blocca il piano.** L'unica ancora aperta — M5 — richiede una GUI, cioè
il sotto-progetto 2.

⛔ **Due cose che il piano _non_ deve rifare**, perché la §8 le ha già chiuse: la copertura
V/Q e l'estensione di `check-docs.sh`. Entrambe sono in esercizio e provate in due
direzioni.

#### Cosa la spec consegna al piano — già deciso, e sparso

Nessuna di queste righe è una decisione da prendere: sono decisioni **prese**, che il piano
deve tradurre in passi. Raccolte qui perché cercarle una per una è il modo in cui se ne
perde qualcuna.

| # | Vincolo sul primo commit di codice | Da |
|---|---|---|
| 1 | **cinque crate**: `kernel` · `platform` · `secrets` · `simulator` · `daemon`. `kernel` non dipende da nessuna crate del progetto — è una riga del suo manifesto | §1.2 |
| 2 | `kernel` e `simulator`: `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]`. **`forbid`, non `deny`** | §1.4 · ADR-0026 |
| 3 | il manifesto **appunta `bincode` a `2`**, con la ragione scritta accanto: la `3.0.0` è un `compile_error!` | §6.1.1 · gotcha #22 |
| 4 | `rustup target add x86_64-unknown-none` è un **prerequisito dell'ambiente**, o la porta è rossa per il motivo sbagliato | §7.3.2 |
| 5 | il [`clippy.toml`](../spikes/rust/clippy.toml) di `spikes/rust/` **non sale**: a livello di workspace scatterebbe addosso a `platform` | §7.4.4 |
| 6 | l'aiutante `passo_in_dubbio` dello spike **non sale così com'è**: restituisce un passo, ne servono un insieme | §4.3 · gotcha #20 |
| 7 | il numero di semi della campagna breve è **fissato e versionato**, e il tempo di parete si stampa a ogni corsa | §7.5.3 |
| 8 | la cadenza: livello 1 a ogni compilazione (non «gira»), livello 2 a ogni commit, DST profonda su ciclo lungo | §7.5.1 |
| 9 | riga per riga, **cosa sale da `spikes/rust/` e cosa resta** | §2.5 |
| 10 | ogni regola nuova porta **due** sonde e un caso in `tests/compile_fail/` con il suo `.stderr` — da **leggere**, non da rigenerare in blocco | §7.1.4 · gotcha #25 |
| 11 | **nessuna decisione legge un parametro che non le è stato consegnato**: budget, quote, policy attiva, tetti. In sotto-progetto 1 i default sono letterali in `daemon` | §2.8 · ADR-0034 |
| 12 | il record durevole è un **enum di versione**, e ogni campo porta un **indice esplicito**. Un campo nuovo è facoltativo con un indice nuovo; un indice **si ritira e non si riusa mai** | §4.9 · ADR-0036 |
| 13 | la porta `journal` scambia **byte**, non record tipizzati: la codifica vive in `kernel`, e `minicbor` entra nella lista di ADR-0031 con la sua classe | §4.1 · §4.9.3 · §7.3.1 |
| 14 | ⛔ al **primo record scritto**, i suoi byte entrano nel repository come oracolo, con la mappa `indice → nome → valore atteso`. **Non si rigenerano**: se cambiano è un cambio di formato | §4.9.4 · gotcha #25 |

📌 **Cosa la §8 ha deciso, e che vale la pena non riscoprire:**

| | |
|---|---|
| **quattro stati, non tre** | `verificato qui` · `parziale` · `rimandato` · `non controllato`. Il quarto — `parziale` — esiste perché V25 e i «solo lato kernel» con tre stati si possono solo sopravvalutare o sottovalutare |
| **`parziale` e `rimandato` pretendono l'innesco** | ed è lo script a pretenderlo, non la buona volontà: è il gettone della §6.3 applicato a una tabella |
| **l'innesco è la _condizione_, il numero sta fra parentesi** | «esiste un'interfaccia (2)», non «sotto-progetto 2». Se la roadmap cambia, la condizione resta vera |
| **un Q della DST eredita lo stato della porta in cui si inietta** | incrociando §3.3 e §7.4.6: Q2 e Q5 sono ✅ perché `reactor` e `journal` hanno la suite di conformità; Q3, Q4, Q18, Q22 sono `parziale` perché le loro porte no |
| **il livello ⛔ è vuoto**, come il livello 3 del catalogo | nessun V e nessun Q è lasciato deliberatamente senza controllo. Ciò che §7.6.2 non controlla sono **pezzi** di V, dichiarati dentro la riga |
| **la porta non prova la correttezza, e la §8 non prova la verità** | prova che ogni V e ogni Q è stato **giudicato**. Lo script controlla che lo stato sia *espresso*, non che sia *giusto* |

✅ **Cinque disallineamenti trovati dalla copertura, tutti chiusi** — §8.5. **Tre sezioni
approvate sono state corrette**, ciascuna con il proprio richiamo datato: §0.4, §0.6 e il
**catalogo §7.4** — la riga V31 in §7.4.2, che era l'unica priva di contro-sonda, le tre
nuove in §7.4.1, e il 2026-08-08 le **cinque** della §6.10.5 più il ritratto ricontato in
§7.4.7.

| # | Trovato | Chiuso come |
|---|---|---|
| 1 | la **§0.6 elencava Q21** fra i «verificati solo lato kernel», ma la **§0.4 non metteva il backup in perimetro**, né dentro né fuori | correzione **doppia**, perché la causa era a monte: §0.4 colloca il backup in «si scaglia» con **regola C** (nuova §0.4.1), e Q21 passa alla riga dei rimandati — §8.5.1 |
| 2 | **nessun sotto-progetto della roadmap collocava il backup** | **sotto-progetto 11 — Backup e ripristino**, dipendente da 5, 6 e 9. L'ordine è derivato: prima che 6 e 9 producano indici e pesi, l'elenco delle esclusioni di V32 è **vuoto**, e verificarlo sarebbe vacuo — gotcha #17. §8.5.2 |
| 3 | il **livello 1 del catalogo non enumerava** tre proprietà che le §5 e §6 avevano già deciso: **V2** (l'ammissione riceve un profilo), **V4** (esito a tre vie), **V10** (artefatto per riferimento immutabile) | entrano nel blocco C di §7.4.1, con sonda e contro-sonda. Il titolo del blocco è passato da «Tipi che non si scambiano» a **«Cosa non è esprimibile»**, perché **tre delle sue sei righe originali non erano scambi di tipo**. E **V16 è stato declassato** da `parziale` a `rimandato`: la metà che dichiarava verificata era vacua. §8.5.3 |
| 4 | il catalogo **non enumerava i cinque controlli della §6.10.5**, decisi con F1b il giorno prima: quattro di livello 1 e uno di livello 2, con le sonde già scritte | entrano dove il blocco li vuole — due gettoni in §7.4.1 B, due voci in §7.4.1 C, uno in §7.4.2 — e **rimandano** a §6.10.5 invece di ridefinirlo, la forma già scelta per V28 in §7.4.4 punto 2. Provato che lo script le **veda**: sonde S7 e S8, ripristino byte-identico. Lo stato di `Q4` **non cambia**. §8.5.4 — è il gotcha **#36** |

I primi due riguardano lo stesso oggetto — il **backup**, l'unica cosa del progetto di cui
nessuno era proprietario, quindi l'unica che nessuna sezione aveva motivo di nominare. Il
terzo e il quarto li ha trovati la §8 **contro sé stessa**, applicando la regola §8.1.2 alla
propria tabella: la prima volta diciassette celle su sessantuno non la rispettavano, e la
seconda — **il giorno dopo, sulla sezione successiva** — è ciò che ha reso il difetto una
classe invece che un incidente.

📌 **La lezione, che vale più delle tre correzioni.** §0.4 e §0.6 erano state scritte nella
**stessa sessione** e rilette più volte: la contraddizione è sopravvissuta. È emersa solo
quando qualcosa ha costretto a rileggerle con una domanda diversa — *«dammi lo stato di
Q21»* — che è ciò che la §8 fa sessantuno volte. **Una tabella di copertura non serve solo
a non dimenticare: serve a rileggere con un'altra domanda.** E una regola che non rifiuta
mai niente è decorazione: la §8.1.2 ha rifiutato tre voci del catalogo e una riga della
propria tabella la prima volta che è stata applicata sul serio.

📌 **Cosa la §7 ha deciso, e che vale la pena non riscoprire:**

| | |
|---|---|
| **il catalogo ha ridotto tre voci invece di aggiungerne** | `HashMap` fuori dal kernel è **tolto** (non difende V29: in DST `platform` non gira affatto) · V28 è un **corollario** dell'allow-list, niente driver · V5 **sale al compilatore** |
| **il livello 3 è vuoto** | nessuna invariante del kernel poggia su un lint. `clippy` resta come igiene, senza voce nella porta |
| **il livello 1 non ha cadenza** | non «gira»: *è* il compilatore. Solo il livello 2 ha una cadenza |
| **la porta non prova la correttezza** | prova che un insieme **nominato** di invarianti regge. Un difetto che non viola nessun V passa verde — §7.6.3 |
| **il quarto gettone si scaglia** | V35/Q23: nessuna porta esegue comandi qui, ed è retrofittabile. L'innesco è scritto in §7.4.5 |

✅ **Le due cose che la §7 aveva lasciato alla §8 sono fatte:**

| # | | Esito |
|---|---|---|
| 1 | **estendere `check-docs.sh`** | ✅ due blocchi nuovi, **cinque asserzioni**: contro-sonda piena nel catalogo §7.4 · completezza e non-duplicazione delle voci in §8 · stato dentro l'insieme chiuso · innesco obbligatorio per `parziale` e `rimandato` · la riga ha cinque colonne. Con una **guardia di non-vacuità** che è la parte importante — §8.6.2. ⚠️ La quinta è stata **aggiunta all'elenco** il 2026-08-08, non allo script: la eseguiva già, e §8.6.1 si presentava come inventario chiuso di quattro |
| 2 | **registrare i rimandati con l'innesco** | ✅ due specie distinte, e ne ho cercata una terza senza trovarla: una **misura** e un **ADR** tarano, non abilitano. §8.2 |

#### Il conteggio delle ventuno voci — ✅ chiuso, e il grep dà zero

Le ventuno voci che nessuna sezione nominava sono ora tutte giudicate. Rieseguito alla
chiusura della §8, il `grep` su `V<n>` e `Q<n>` restituisce **zero mancanti**.

Il ritratto che ne esce, contato il **2026-08-07** sulla tabella stessa:

| | ✅ verificato qui | ⚠️ parziale | ⏳ rimandato | ⛔ non controllato |
|---|---|---|---|---|
| **V** (37) | 18 | 12 | 7 | **0** |
| **Q** (24) | 9 | 8 | 7 | **0** |

⚠️ **Un terzo delle voci è `parziale`, ed è il ritratto onesto** di un sotto-progetto che
costruisce il kernel senza nessuno dei suoi consumatori. Chi legge la tabella cercando
conforto la leggerà male.

⛔ **Il livello `non controllato` è vuoto**, come il livello 3 del catalogo. Non è una
svista e non contraddice la §7.6.2: quella sezione dice che *la porta* non controlla Q6,
Q11, Q12 e Q16 — e la sua stessa colonna li rimanda alla §8. Qui sono `rimandati`, che è la
traduzione esatta. Il valore ⛔ significa un'altra cosa: *si sceglie di non controllarlo, e
nessun innesco lo riaprirà*. §8.7.1.

### ✅ Le due domande della §7 sono decise

Le aveva sollevate M-1 (§6.8.2) e M-3 le aveva rese concrete con dei numeri. Decise nella
**§7.3** il 2026-08-07.

| # | Domanda | Decisione |
|---|---|---|
| **1** | il controllo della allow-list misura il grafo di **runtime** o quello **totale**? | **entrambi, con due comandi e due rimedi distinti** (§7.3.1). Una violazione fra le crate *spedite* è `I3 violated` e si ripara **togliendo** la dipendenza; un cambiamento fra quelle *di build* è un evento da rivedere e si ripara **aggiungendola alla lista**. Le dipendenze di **sviluppo** sono escluse, e l'esclusione è provata |
| **2** | il **cancello bare-metal** entra fra i controlli automatici? | **si aggiunge alla lista, non la sostituisce** (§7.3.2), e il bersaglio passa a **`x86_64-unknown-none`**. I due falliscono in modo complementare: la lista **nomina il colpevole**, il cancello **prova** invece di enumerare |

📄 [`superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md)

## Stato del sotto-progetto 1

| § | Sezione | Stato | Cosa ha deciso |
|---|---|---|---|
| 0 | Perimetro e criterio di scaglionamento | ✅ | cosa entra e cosa si scaglia, con un criterio falsificabile a tre regole (A/B/C) |
| 1 | Struttura delle crate e regole di importazione | ✅ | cinque crate: `kernel` · `platform` · `secrets` · `simulator` · `daemon`. Più [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md), nato da una misura |
| 2 | Il substrato iniettabile | ✅ | esecutore nel `kernel`, `Reactor` come porta, nessun thread nel percorso decisionale |
| 3 | Il simulatore DST | ✅ | cosa sostituisce, tempo virtuale, iniezione dei guasti, il seme e cosa **non** è |
| 4 | Giornale, riconciliazione, persistenza | ✅ | write-ahead, riconciliazione su un **insieme**, [ADR-0032](adr/0032-motore-di-persistenza.md) `redb`. Più la **§4.9**: versione + indici espliciti, [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md) |
| 5 | Arbitro GPU, e la lacuna su I2 | ✅ | tre consumatori GPU nella GUI, quota di presentazione, I2 sui worker imposto dal **compilatore**. Più [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md) |
| 6 | Gateway, sensori, permessi, degrado | ✅ | schema IPC in `kernel` con **`bincode`**, **timbro di build** contro la GUI stantia, il **gettone non falsificabile** nominato una volta, «costo» del sensore separato in due |
| **7** | **La porta di qualità: i controlli automatici** | ✅ | **scala di forza a tre livelli**, evidenze di M-3, le due decisioni sulle dipendenze, il **catalogo** con sonda e contro-sonda, la cadenza, il perimetro negativo. **Il livello 3 è vuoto** |
| **8** | **Copertura V1–V37 e Q1–Q24** | ✅ | **quattro stati** con innesco obbligatorio, **due specie di innesco**, la regola che un Q della DST eredita lo stato della propria porta, l'estensione di `check-docs.sh` provata in due direzioni. **Il livello ⛔ è vuoto**. ⚠️ **Riallineata e chiusa il 2026-08-08** alle sette voci, poi passata per l'**audit sezione-contro-ADR**: **cinque** disallineamenti in tutto, l'ultimo è §8.5.5 — otto formulazioni di vincolo troncate, e su una (`V16`) era stato dato un giudizio |

**§0–§8 approvate, e la riapertura è chiusa.** Le sezioni toccate portano ciascuna il proprio
richiamo datato: §0.4.1, §0.4.2, **§0.4.3 (nuova)**, §0.5, §0.7, §1.2, §2.0, §2.3.1 (nuova),
§2.8 (nuova), §3.1, **§3.3**, **§4.1**, **§4.9 (nuova)**, §5.1, §5.2.2, **§6.1.1**, **§6.8**,
**§6.10 (nuova)**, **§7.3.1**, **§7.4.1**, **§7.4.2**, §7.4.4, **§7.4.6**, **§7.4.7**, e in
§8 le celle `Q4`, `Q5`, `Q14`, la riga V3 di §8.3, la riga `process` di §8.2.2, **§8.5.4
(nuova)** e il ritratto di §8.8.

### I sei traguardi, e dove siamo

Il sotto-progetto 1 si esegue a traguardi, e **ciascuno ha il proprio piano** — scritto
quando si arriva, perché scriverne uno per codice che non esiste significa inventare.

| # | Traguardo | Stato |
|---|---|---|
| **1** | scheletro e porta di qualità — le cinque crate e i controlli, **zero logica** | ✅ **eseguito il 2026-08-08**, `GATE GREEN` |
| **2** | il substrato iniettabile — tempo, casualità, I/O, scheduling, l'esecutore, le sei porte | ⏭️ **prossimo**. ✅ Il [piano](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) è **scritto** il 2026-08-09, quattordici compiti in due parti. **Da eseguire** |
| 3 | giornale e formato durevole — la porta a byte, l'enum di versione, **i byte congelati** | ⬜ |
| 4 | il simulatore DST — tempo virtuale, guasti, campagna, semi | ⬜ |
| 5 | arbitro GPU — ammissione, corsie, concessione, le due policy | ⬜ |
| 6 | gli altri meccanismi — gateway, sensori, permessi, degrado, canale worker | ⬜ |

**Cosa il Traguardo 1 ha lasciato dietro di sé**, oltre al codice:

| | |
|---|---|
| **quattro gotcha nuovi** | **#38** la guardia che installa ciò che verifica · **#39** i test negativi che ridichiarano le proprie precondizioni · **#40** una decisione fuori da un ADR non arriva al compendio · **#41** il filtro che decide cosa il controllo può vedere |
| **due seconde occorrenze** | **#26** — il glob vuoto di `trybuild` esce verde, il percorso letterale no · **#25** — gli oracoli sono accoppiati al **grafo linkato**, e due diventano rossi insieme per un motivo estraneo alla regola |
| **tre controlli nati dopo il piano** | `scripts/gate-attributes.sh` (da #39) · la classe di caratteri allargata di `gate-deps.sh` (da #41) · e, il **2026-08-09**, la riga sul **build script** dentro `gate-attributes.sh`, trovata da una revisione **misurando**, non leggendo. Nessuno dei tre era previsto |
| ⛔ **un'errata in testa al piano** | quattro voci, e la prima è la §1.0: il piano dettava identificatori **italiani**, e il codice è in **inglese** perché lo impone la spec |

### Le decisioni aperte dalla §0.5 — tre previste, quattro emerse

| # | Decisione | Esito |
|---|---|---|
| 1 | GPU della GUI non arbitrata | ✅ [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md): quota di presentazione sottratta, concessione tenuta dal **core** |
| 2 | Motore di persistenza | ✅ [ADR-0032](adr/0032-motore-di-persistenza.md): `redb` 4.1.0 con backend nostro |
| 3 | Dove vive l'esecutore | ✅ nel `kernel`, con `Reactor` come porta (§2.4) |
| 4 | Dipendenze del kernel nel confine I3 | ✅ [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md) — **non prevista**, emersa da una misura |
| 5 | I parametri di decisione sono consegnati, non letti | ✅ [ADR-0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md) — **non prevista**, emersa dalla riapertura |
| 6 | La porta verso i worker, e la lettura di «singolo» in I4 | ✅ [ADR-0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) — **non prevista**, è la voce F1 della stessa riapertura |
| 7 | L'evoluzione del formato durevole del giornale | ✅ [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md) — **non prevista**, è la voce F2, con F7 che vi converge |
| 8 | Il criterio del pari per il formato dei canali privati | ✅ [ADR-0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) — **non prevista**, emersa misurando ciò che F1b chiedeva |

### Misure eseguite, e quelle ancora aperte

Tutte con `rustc 1.95.0` · `cargo 1.95.0` · Windows 11. Evidenze complete nella spec.
M-10 e M-11 hanno un secondo capo: Python **3.13.7** e Node **v24.9.0** con npm **11.6.0**.

| # | Domanda | Esito |
|---|---|---|
| M-4 | un runtime di ecosistema è usabile sotto `no_std`? | ✅ **sì** — l'attesa contraria era falsa. 55 crate nel grafo, fra cui `getrandom` |
| M-5 | un esecutore `no_std` senza `unsafe` fa avanzare `Future` reali? | ✅ **sì, con zero dipendenze**. Un `Waker` su misura invece **non** è costruibile: `E0133` |
| M-7 | quanto costa una decisione dell'arbitro? | `request` ≤ 100 ns · `release` p99 **500 ns** a coda realistica, 86,6 µs a coda 2000 |
| M-2 | `simulator` regge `no_std`? | ✅ **sì**. 100 corse → 1 traccia · 20 000 ms virtuali in **25,8 µs** · crash riproducibile 5/5 |
| M-8 | i quattro requisiti di §10.6 su `redb` | 1 ✅ · 2 ✅ · 3 ⚠️ si stabilizza in alto · 4 ✅ **12/12 crash recuperati** |
| M-6 | `BTreeMap`/`Vec` bastano alle strutture del kernel | ✅ **chiusa dall'esistenza di M-7**: il suo prototipo è `no_std`, zero dipendenze, tutto su `BTreeMap`, e l'arbitro è la struttura più complessa del kernel finora. Resta aperta solo per ciò che introdurrà la §6 |
| M-1 | serializzatore per lo schema IPC con **grafo transitivo** accettabile | ✅ **sì, tutti e cinque i candidati provati.** Scelto `bincode` 2.0.1 (2 crate di runtime). Esito **A**: lo schema sta in `kernel`, il grafo di §1.2 non cambia |
| M-3 | allow-list di ADR-0031 esprimibile con la toolchain standard, provata in negativo | ✅ **sì, esito A** — con `cargo tree`, **non** con `cargo metadata`. Sonde N1–N4 e B1–B3, entrambe le direzioni dell'errore. **Evidenze nella §7.2 della spec**; qui sotto resta solo la correzione al comando |
| **M-9** | cosa succede rileggendo un record durevole dopo che il tipo è cambiato | ✅ **tre classi di formato × nove mutazioni.** Cinque celle sono **silenzio sbagliato**, non errore. Elimina la forma **C** e prezza l'indice a **un byte su ventisei**. Evidenze in [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md) |
| **M-10** | il pari **Python** decodifica `bincode` 2.0.1? | ⛔ **no.** L'unica libreria che si dichiara compatibile è ferma alla configurazione **1.x** (33 B contro 12) e **non ha tipi somma**; il pacchetto PyPI omonimo è un helper base64. ✅ `minicbor` letto da `cbor2` 6.1.4: valori giusti. Evidenze in [ADR-0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) |
| **M-11** | e il pari **TypeScript**? | ✅ **sì** — `bincode-ts` 1.0.0 decodifica con i valori giusti e i byte tutti consumati. ⚠️ pacchetto a **una sola versione**, con entrambi i punti d'ingresso rotti su Node 24. Anche `cbor-x` 1.6.5 ✅ |
| **M5** | quanta VRAM prende la presentazione della GUI | ⬜ **aperta e dichiarata tale** — richiede una GUI: sotto-progetto 2, accanto a M1–M4 di ADR-0029 |

#### M-3 — ✅ evidenze trasferite nella spec

Le evidenze complete — esito A, lo scarto fra `cargo metadata` e `cargo tree`, le sonde
N1–N4 e B1–B3 — **vivono ora nella §7.2 della spec del sotto-progetto 1**, che è la loro
sede unica. Qui restano solo le due cose che non stanno lì.

**1 · Una riga di questo documento era sbagliata, e va saputo.** HANDOFF affermava che
`cargo tree -e no-proc-macro` separa il grafo di runtime da quello totale.

> ⛔ **Non li separa.** Da solo toglie i generatori di codice ma lascia dentro l'intero
> sottoalbero delle dipendenze **di sviluppo**, e con esso `windows-sys`. Su un workspace
> con `trybuild` — che il kernel avrà, §2.5 — restituisce **venti** crate invece di due.
> Il comando corretto è **`-e normal,no-proc-macro`**.

**Perché M-3 non poteva accorgersene:** il suo workspace di prova non aveva dipendenze di
sviluppo, e senza quelle i due comandi danno la stessa risposta. La sonda **non poteva
falsificare l'affermazione** — gotcha #17 applicato a M-3 stessa. Riverificato il
2026-08-07, con la contro-sonda che lo dimostra.

**2 · La correzione che M-3 aveva imposto alla §6.1.1**, già applicata: `simulator` non
aggiunge voci proprie, ma la sua lista **non è vuota** — dipende da `kernel`, e la regola 2
è sul grafo *transitivo*. Scritto «resta vuota», misurato `bincode kernel unty`.

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

36 ADR in stato `Accepted`. Rimetterne in discussione uno **richiede un ADR
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
| Il controllo delle dipendenze misura **due grafi con rimedi opposti** (§7.3.1) | unificarli sembra una semplificazione e non lo è: insegna il riflesso «aggiungi alla lista» **anche per una violazione di I3**, dove aggiungere alla lista non è un rimedio ma la violazione scritta in un modulo. È così che un'invariante si degrada in scartoffia |
| Il **cancello senza OS si aggiunge**, non sostituisce la lista (§7.3.2) | sembrano ridondanti e non lo sono: la lista coglie una crate **nuova**, il cancello una crate **già ammessa** che raggiunge l'OS per una via non prevista. E quando falliscono, **solo la lista dice il nome del colpevole** |
| Il **livello 3 del catalogo è vuoto** (§7.4.3) | la tentazione è aggiungere un lint «tanto non costa niente». Costa: un rosso della porta deve significare sempre «invariante violata», mai «stile discutibile», o si impara a ignorarlo |
| L'**innesco è obbligatorio** per `parziale` e `rimandato` (§8.1) | sembra burocrazia e non lo è: è l'unica cosa che impedisce a `parziale` di diventare la casella comoda in cui parcheggiare tutto. Toglierla riporta alla situazione che la §0.6 chiamava «rimandato tende a diventare dimenticato» — con in più una tabella che sembra dire il contrario |
| La **guardia di non-vacuità** dei controlli nuovi (§8.6.2) | è il pezzo che sembra più togliibile e il solo che non si può togliere. Senza, basta rinumerare una sotto-sezione perché due controlli smettano di controllare **uscendo verdi**: gotcha #26. E il «miglioramento» sbagliato è metterci un numero atteso di righe, che diventa rosso quando la tabella cresce per un motivo legittimo |
| I **parametri di decisione sono consegnati**, non letti (ADR-0034) | la scorciatoia tentante è «tanto il budget è 16 GB, scrivilo e basta». Una costante nel kernel **non fa scattare nessun controllo del catalogo §7**: si scopre solo quando qualcuno prova a farla variare in campagna e non può. È il gotcha #12 su un altro asse — e toglie alla DST l'unico modo di esplorare lo scenario di RK-1 |
| Il **dialogo con un worker vive dentro `process`** (ADR-0035) | la scorciatoia tentante è «è tutto IPC, mettiamolo su `ipc`». Sembra un accorpamento e invece **spezza la vita di un worker fra due porte**: l'avvio pretende una concessione (§5.6), e se il dialogo passa da un'altra porta quella catena non copre più il parlare. Si perde il meccanismo che ha portato I2 dal test al **compilatore**, e lo si perde senza che nulla diventi rosso |
| Il **formato del giornale è deciso a sé**, versione più indici (ADR-0036) | la scorciatoia tentante è «usiamo `bincode` anche per il giornale, tanto è già nella lista». Sarebbe importare in un artefatto che **deve** evolvere una decisione presa dove l'evoluzione era stata **rinunciata** (I4). E non è un'opinione: la misura dice che su un formato posizionale un campo *opzionale* aggiunto in coda **rende illeggibili i record vecchi**, e che togliere un campo li rilegge **in silenzio con byte non consumati**. La correzione tardiva non è una patch: è la migrazione dell'unico archivio irriproducibile |

## Le quattro proprietà che non si aggiungono dopo

Se le trascuri, la correzione non è una patch: è una riscrittura — o, per la quarta, una
**migrazione**.

| # | Proprietà | Da |
|---|---|---|
| 1 | Confine dei dati non fidati **nel sistema di tipi** | I6 · ADR-0014 |
| 2 | Nessuna chiamata OS-specifica nel kernel | I3 · ADR-0002 |
| 3 | **Iniettabilità** di tempo, casualità, I/O e scheduling — e dei **parametri di decisione**, che sono l'altro asse | V29 · ADR-0021 · **ADR-0034** |
| 4 | Il **record durevole dichiara la propria versione**, e i suoi campi si identificano per **indice esplicito** | §4.9 · **ADR-0036** |

⚠️ **La quarta è entrata il 2026-08-07**, ed è la sola il cui costo tardivo non è una
riscrittura ma la **migrazione dell'unico archivio irriproducibile**. La sua finestra si
chiude alla prima riga di codice che scrive un record — non a un traguardo di progetto.

Più una quinta, di natura diversa ma altrettanto vincolante: **nessuna esecuzione di
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
| 25 | **Rigenerare in blocco le evidenze di un test negativo lo trasforma in una tautologia** | un test di compilazione fallita in Rust confronta l'errore prodotto con un file `.stderr` salvato accanto al caso: è **ciò che gli impedisce di fallire per il motivo sbagliato** (gotcha #9 in forma Rust). Ma `trybuild` offre un modo di riscrivere **tutti** gli `.stderr` sull'output corrente. Serve quando i messaggi cambiano legittimamente; usato senza leggerli, ogni caso diventa «l'errore atteso è quello che è uscito» e la suite **passa per sempre**, restando verde. La rigenerazione è un atto deliberato e **si legge nel diff**, come aggiungere una voce alla lista di ADR-0031. Corollario che vale oltre `trybuild`: ogni volta che l'oracolo di un test è un file generato dal test stesso, aggiornarlo automaticamente **cancella l'oracolo**. 📌 **Seconda occorrenza, 2026-08-08, e aggiunge la ragione per cui la tentazione arriva proprio nel momento peggiore.** Gli `.stderr` non sono accoppiati al **sorgente del caso**: sono accoppiati al **grafo linkato**. Misurato togliendo `#![no_std]` da `crates/kernel/src/lib.rs`: il caso continua a non compilare — ridichiara il proprio `#![no_std]` — ma dall'output reale **sparisce la riga dell'allocatore**, *«no global memory allocator found»*, e **due oracoli diventano rossi insieme**: `std_in_kernel.rs` e `hashmap_in_kernel.rs`, cioè i due che quella riga la portavano. ⛔ Il punto che fa male: quei due rossi non dicono nulla sulla regola che i due casi mettono alla prova — parlano della **forma del kernel**. Chi li vede rossi «per un motivo che non c'entra» è esattamente chi rigenererebbe in blocco, e in quel momento la rigenerazione cancella l'oracolo di **entrambi** |
| 26 | **Un controllo che delimita il proprio bersaglio per intestazione si spegne quando qualcuno rinumera — e si spegne _verde_** | è il gotcha #14 in una forma che #14 non copre: quel controllo **è stato visto fallire**, quindi era un controllo vero. Poi qualcuno rinomina `#### 7.4.1`, l'intervallo non trova più righe, e uno script che non ha niente da controllare **esce con successo**. Il segnale è indistinguibile da «tutto a posto». Rimedio, applicato in §8.6.2: **se un delimitatore non si trova, o l'intervallo è vuoto, è un fallimento**. Sonde S6, S6b e S6c. ⚠️ E il rimedio sbagliato è mettere a guardia un **numero atteso** di righe: diventerebbe rosso il giorno in cui la tabella cresce per un motivo legittimo, cioè il gotcha #9 applicato allo script. Si verifica che i delimitatori esistano, non quante righe ci siano — a meno che l'elenco non sia canonico, come i V1–V37, dove la completezza *è* il controllo. 📌 **Seconda occorrenza, 2026-08-08, fuori dagli script e dentro un banco di test.** Il piano prevedeva che, senza la cartella `tests/compile_fail/`, il banco fosse **rosso**. Misurato su `trybuild` 1.0.120: **un glob che non pesca niente non è un errore** — stampa un avviso giallo, lascia i fallimenti a zero ed **esce 0**. Un percorso **letterale** inesistente invece diventa rosso. ⚠️ **L'asimmetria è la parte da ricordare**, perché non si ricostruisce leggendo `t.compile_fail(...)`: la stessa chiamata è severa o indulgente a seconda che l'argomento contenga un `*`. Rimedio in esercizio: `crates/kernel/tests/compile_fail.rs` conta i `.rs` **prima** di chiamare `trybuild` e fallisce se sono zero — **senza numero atteso**, che è il rimedio sbagliato di questa stessa riga |
| 27 | **La legenda di una tabella risponde a una domanda sola, e chi la legge ne assume un'altra** | `tracciabilita.md` risponde a *«dove vive questa funzionalità»*. Il suo `📋` significa «sotto-progetto assegnato» — **non** «non richiede un meccanismo di kernel». Nessuno l'aveva mai letta con la seconda domanda, e leggerla così ha **riaperto la spec su sette voci**, tre di classe B. È il gotcha #26 spostato dai controlli ai documenti: una tabella che non ha mai rifiutato niente non sta verificando niente. ⚠️ Il rimedio non è riscrivere la legenda: è **rileggere con un'altra domanda**, che è ciò che la §8 ha fatto sessantuno volte e questa volta ha fatto la tracciabilità |
| 28 | **Un parametro non consegnato è una costante, e una costante è invisibile** | V29 rende sostituibile ciò che il mondo *risponde*. Non dice nulla sui **parametri con cui il kernel è configurato** — budget della GPU, quote sottratte, policy attiva, tetti di autonomia — e ciò che non viene consegnato finisce scritto dentro. Non compare in nessun elenco, non fa scattare nessuna voce del catalogo, e **si manifesta solo come uno scenario che la campagna non può esplorare**: con le quote fisse, RK-1 è irraggiungibile. Chiuso da ADR-0034. ⛔ E il limite resta dichiarato: il compilatore prova che una decisione *riceve* i propri parametri, **non** che non ne abbia altri di nascosto |
| 29 | **La riga di _verifica_ di un'invariante è il punto in cui l'invariante si restringe in silenzio** | Le sei invarianti di ADR-0004 hanno due colonne: l'**enunciato** e **come si verifica**. La seconda è più corta, più concreta, e viene letta al posto della prima — ma è scritta guardando i casi che esistevano quel giorno. È già successo **due volte**: I2 diceva «nessun **worker** si avvia senza concessione», e copriva una classe di processo su tre (chiuso da ADR-0033); I4 dice «nessun **consumatore esterno**», che parla di esternalità e non di quanti dei nostri processi parlino il protocollo (chiuso da ADR-0035). In entrambi i casi l'enunciato era giusto e nessun controllo poteva accorgersene, perché **la riga di verifica _è_ il controllo**. ⚠️ Il rimedio non è riscrivere le invarianti: è che **completare una riga di verifica non è superarla** — ADR-0004 non è mai stato superato, ha ricevuto due rimandi. Chi trova il terzo caso lo aggiunga qui invece di aprire un dibattito sull'invariante. 📌 **Terza occorrenza, 2026-08-08, e in una forma peggiore delle prime due.** Non una riga di verifica ma una **riformulazione**: la colonna «Vincolo» di §8.3 riassume in poche parole un vincolo che vive nella spec del kernel, e **otto** delle trentasette avevano perso un pezzo. Sette erano innocue; su `V16` la metà caduta era quella **positiva** — *«nomi di provider e parametri sì»* — e il giudizio era stato dato sulla metà rimasta, declassando a `rimandato` un vincolo che è `parziale`. ⛔ La differenza che rende questa forma peggiore: una riga di verifica troppo stretta **lascia scoperto un caso**, una riformulazione troppo stretta **cambia l'oggetto del giudizio**, e nessuno script può accorgersene perché la casella è piena e lo stato è ammesso. Rimedio in §8.5.5: quando si tocca la §8, la colonna «Vincolo» **si confronta con la fonte**, non si rilegge da sola |
| 30 | **Un banco che guarda solo l'esito non vede la risposta sbagliata** | È il gotcha #17 spostato dall'**iniezione** all'**oracolo**: non «il guasto non è scattato», ma «il guasto *è* scattato e il banco non sa distinguerlo da un successo». **Misurato in M-9**: rileggendo un record durevole dopo un cambio di tipo, cinque celle su trentasei restituiscono `Ok` **con valori sbagliati**. Un banco che confronta solo `Ok`/`Err` le legge tutte come successi — e la forma che *sembrava* più economica avrebbe superato il vaglio. Il rimedio è confrontare i **valori**, non l'esito, e vale ovunque l'oracolo sia un codice di ritorno: decodifica, parsing, riconciliazione, conversione di tipi. ⚠️ **Corollario che vale oltre F2:** in un archivio durevole il modo di fallire peggiore **non è l'errore** — un errore lo vedi — ma il record che si rilegge e ti restituisce il numero sbagliato |
| 31 | **Una stima di costo prezzata sulla variante sbagliata sopravvive, perché viene citata invece che rifatta** | Questo documento prezzava la forma B dell'evoluzione come *«permanente su ogni campo di ogni record»*. **Misurato: un byte su ventisei.** La stima guardava la codifica a **mappa**; la predefinita della stessa libreria è ad **array**, e lo scarto è di sette volte — su un numero che stava per far scartare la forma giusta. Non è un errore di calcolo: è che una stima scritta una volta **suona più certa più invecchia**, e nessuno la rimisura perché «c'è già scritto». È il gotcha #15 — un'evidenza scritta prima della misura è un'ipotesi — applicato ai **costi** invece che agli esiti. ⚠️ Il rimedio non è diffidare delle stime: è che una stima che sta per **decidere** va rimisurata, e quella che decide soltanto di rimandare no. 📌 **Seconda occorrenza, 2026-08-08**, e in una forma peggiore perché il numero **sostiene una regola giusta**: `CLAUDE.md`, il compendio e `AVVIO-CHAT.md` dicevano che HANDOFF + spec del sotto-progetto 1 + `adr/` pesano *«oltre settecento kilobyte»*. Misurati: **cinquecentosedici**, cioè un terzo in meno, e i documenti da allora sono solo **cresciuti** — quindi il numero non era invecchiato, **non era mai stato vero**. Nessuno l'aveva verificato perché la regola che giustifica — «non aprirli per farsi un'idea» — è corretta, e un numero che argomenta bene non sembra un'affermazione. ⛔ Un numero gonfiato a sostegno di una regola giusta resta un numero falso, e il giorno in cui qualcuno lo verifica **è la regola a perdere credito** |
| 32 | ⛔ **Un'idea che sembra nuova può essere già stata scartata, e il compendio non lo dice** | Il compendio comprime le **decisioni**, non le alternative respinte: il *perché* lungo — misure comprese — vive nell'ADR o nella sezione. Conseguenza: una proposta perfettamente ragionevole può essere già caduta, **con la misura fatta**, e chi la ripropone non incontra nessun ostacolo. **Successo il 2026-08-08**: si è proposto di sostituire `bincode` con `minicbor` sul canale `ipc` per uniformare i codificatori. Era già stato valutato e respinto in §6.8 e riaffermato il giorno prima, e il compendio lo vietava per nome. ⚠️ Il rimedio **non** è leggere tutto prima di parlare — sarebbero cinquecento kilobyte. È: **prima di proporre qualcosa che sostituisce una decisione presa, cercare dove era già stata valutata e perché era caduta**, e riaprire **solo con una prova nuova**. ⛔ E se la prova nuova gioca **contro** la propria idea, si registra e si chiude: qui a smontare la proposta è stata **la stessa misura che l'aveva motivata** (M-11). La regola sta in `CLAUDE.md` |
| 33 | **Il nome del formato è occupato da un'altra cosa, e in due ecosistemi** | Su PyPI il pacchetto `bincode` installa un modulo **`b64tools`**: funzioni base64, nessun rapporto col formato. Su npm `bincode` è una **CLI di sviluppo con l'IA**. Cercare per nome trova pacchetti che non c'entrano, e in un elenco di risultati sembrano conferme. È il gotcha #22 nella forma più larga: che una versione esista non dice che funzioni, e che un **nome** esista non dice **cosa contiene**. Il rimedio è aprire il pacchetto: `pip download`/`npm pack` e guardare i file |
| 34 | ⛔ **Un decodificatore CBOR si ferma al primo elemento completo e ignora la coda** | Misurato: dando a `cbor2` i byte prodotti da `bincode` per `Esito`, restituisce **`1`** — legge il primo byte come intero senza segno e si ferma. **Nessuna eccezione, un valore plausibile.** Vale per la famiglia: il formato è auto-delimitante per *elemento*, non per *buffer*. ⚠️ Conseguenza operativa su un canale a frame: un controllo «ha decodificato senza errori» **non prova nulla**. Il frame deve dichiarare la propria lunghezza, e la decodifica deve verificare che i **byte consumati** siano esattamente quel numero. È il gotcha #30 spostato dall'archivio durevole al filo |
| 35 | **Un `Vec<u8>` non annotato raddoppia il traffico, in silenzio** | In `minicbor`, scritto nel modo naturale, un `Vec<u8>` si codifica come **array di numeri** e non come stringa di byte: ogni byte ≥ 24 ne occupa due. Serve l'annotazione esplicita (`with = "minicbor::bytes"`). **Misurato** su un frammento audio da 4096 B: **7813** contro **4101**, cioè **1,91×**. ⚠️ La trappola è che non somiglia a un difetto: compila, fa round-trip, e i valori sono **corretti**. Costa soltanto il doppio della banda, su un canale — quello audio — dove la banda è il vincolo |
| 36 | ⛔ **La tabella «come si verifica» di una sezione non è il catalogo, e il passaggio si salta** | Quando una sezione decide un meccanismo nuovo, scrive una propria tabella *«come si verifica»* con sonda e contro-sonda — ed è giusto che lo faccia, perché il controllo si progetta dove si progetta la cosa. Ma il **catalogo §7.4** è l'unico posto che la §8.1.2 ammette come meccanismo nominabile, ed è la lista che il piano tradurrà in lavoro. Chi scrive la sezione considera il controllo «scritto» e non lo riporta. **Successo due volte**: la prima con V2, V4 e V10 — proprietà di livello 1 decise nelle §5 e §6 e mai enumerate (§8.5.3); la seconda il giorno dopo con i **cinque controlli della §6.10.5** (§8.5.4). ⚠️ **La diagnosi che rende il gotcha utile è l'asimmetria**: nella stessa riapertura, §2.8.4 e §4.9.4 le loro righe le hanno aggiunte, §6.10.5 no, e **nessun documento dice perché** — cioè non era una decisione, era un passaggio saltato. Prima di concludere che sia stato deliberato si cerca dove sarebbe stato deciso: in ADR-0037 la parola «catalogo» non compare, e la §7.4.4 riduce tre voci che sono altre. ⛔ **Il rimedio non è irrigidire `check-docs.sh`**: lo script verifica che la casella del meccanismo sia *piena*, non che nomini davvero una voce della §7 (§8.6.4), e pretendere di più trasformerebbe le dichiarazioni oneste in rossi. Il rimedio è sapere che **questa è la classe di difetto che ricompare**, e cercarla a ogni sezione che decide un controllo. È il gotcha #29 spostato dalle invarianti ai controlli: il posto dove una cosa si **verifica** è il posto dove si restringe in silenzio |
| 37 | ⛔ **Un controllo può difendere _un altro controllo_ invece di una proprietà, e la regola d'ammissione lo scambia per un'abitudine** | La §7.1.1 ammette una voce nel catalogo solo se difende un `V`, un'`I` o un `Q` nominato; se non ne difende nessuno *«è un'abitudine, non un controllo: va tolta»*. È la regola con cui la §7.4.3 caccia `clippy`, ed è giusta. **Misurato il 2026-08-08: otto righe su trentatré non la rispettavano**, e sotto la sua lettera andavano tolte tutte e otto. Cinque erano contabilità — `Q8` era perfino **già scritta** in §8.4 e mai riportata nel catalogo, gotcha #36 nella forma pura. ⛔ Le altre tre no, e sono la trappola: `forbid(unsafe_code)`, l'allow-list sul grafo **di build** e i **test di contratto** non difendono una proprietà del sistema — difendono il **verdetto** di altri controlli. Cancella `forbid` e **nessuna** riga del catalogo diventa rossa: diventano tutte **meno vere**, perché un `unsafe` falsifica un gettone, transmuta un newtype o raggiunge l'OS con un `extern` scritto a mano. Un catalogo senza una casella per dirlo costringe a scrivere il numero di una sezione nella colonna del vincolo, e lì resta finché qualcuno applica la regola alla lettera. ⚠️ **Il rimedio sbagliato è allargare la regola** a «una proprietà decisa in una sezione nominata»: così non rifiuta più niente, e una regola che non rifiuta mai è decorazione — la §8.5.3.1 esiste perché quella famiglia di regole ha **rifiutato** alla prima applicazione seria. Il rimedio è un **secondo ramo** con criterio proprio: *nomina le voci del catalogo di cui sostieni la validità*. Rifiuta ancora `clippy`, perché cancellare `clippy` non rende falsa nessuna riga. 📌 **E la prova nelle due direzioni è arrivata gratis**: la sesta asserzione di `check-docs.sh`, scritta **prima** della correzione, alla prima corsa ha nominato **le otto e solo le otto** — 8 rosse, 25 verdi — con un caso storico invece che con una mutazione costruita |
| 38 | **Un controllo che interroga uno stato può _modificarlo_ mentre lo interroga, e allora la sua condizione non può essere vera** | `scripts/gate-no-os.sh` ha una guardia in testa: chiede a `rustup` l'elenco dei bersagli installati e, se `x86_64-unknown-none` non c'è, esce 1 con il comando da lanciare — perché senza il bersaglio il cancello sarebbe rosso **per il motivo sbagliato**, che è il vincolo 4 della §11. **Misurato**: `rustup target list --installed` **riconcilia `rust-toolchain.toml` prima di rispondere**, e il manifesto dichiara `targets = ["x86_64-unknown-none"]`. Quindi se il bersaglio manca, **l'atto stesso di chiederlo lo installa**, e la risposta è «c'è». Isolato eseguendo la sola riga della guardia, senza `cargo`, con una directory **fuori dal repository** come controllo: lì il manifesto non esiste, non c'è niente da riconciliare, e il bersaglio resta assente **tre volte su tre**. ⚠️ La guardia **può** scattare, ma solo dove la riconciliazione fallisce, cioè **senza rete**: verificato, uscita 1 e messaggio corretto. È dunque **la via offline, non una rete di sicurezza** — rende usabile una macchina isolata, non sorveglia una macchina connessa, e la differenza sta scritta accanto alla guardia perché chi la legge non le attribuisca un potere che non ha. Sonda **B4** in [`porta-di-qualita.md`](porta-di-qualita.md) |
| 39 | ⛔ **Un test negativo che _ridichiara le proprie precondizioni_ prova il meccanismo, non che il sistema sia configurato così** | I quattro casi di `crates/kernel/tests/compile_fail/` **ridichiarano ciascuno** `#![no_std]` e `#![forbid(unsafe_code)]`, e **non nominano mai `kernel::`**. Provano che il divieto **morde dove è dichiarato** — cosa vera e utile — non che il kernel lo dichiari. **Misurato**: tolto `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs` **e scritto un `unsafe` vero** dentro la crate, la porta restava **verde su cinque controlli su cinque**. La Definizione di «fatto» del piano dava quella condizione — *«i quattro casi passano»* — per una verifica del divieto sul kernel, e non lo era: è la voce **E4** dell'errata. Rimedio: `scripts/gate-attributes.sh`, che cerca i tre attributi ancorati a inizio riga nei due file vincolati e rifiuta anche `deny` al posto di `forbid`. È **livello 2**, quindi cancellabile, e questo è dichiarato. ⚠️ **Peggiore di come sembra**, e la ragione sta nel catalogo: la riga di `forbid(unsafe_code)` è di **ramo 1b** — sostiene la validità dei blocchi A, B e C, gotcha #37 — quindi toglierla non spegneva **una** regola, invalidava **il fondamento del livello 1** senza che niente diventasse rosso |
| 40 | ⛔ **Il compendio dichiara di contenere tutte le decisioni, ma il controllo ne pretende una voce solo per gli _ADR_: una decisione che vive in una _sezione di spec_ può mancare, e allora per chi legge non esiste** | `CLAUDE.md` ordina di leggere due file e **fermarsi**, e il compendio si presenta come *«una compressione, non una selezione — ci sono dentro tutte le decisioni»*. La promessa è mantenuta **dove è controllata**: `check-docs.sh` pretende una voce di §5 per ogni file in `docs/adr/`, in due direzioni. Ma la **§1.0** della spec del sotto-progetto 1 — *codice interamente in inglese, documentazione in italiano, riferimenti al codice in inglese col nome esatto del sorgente* — **non è un ADR**: è una sezione, quindi **nessun controllo ne pretendeva la presenza**, e infatti non c'era né nel compendio né in `CLAUDE.md`. **Misurato il 2026-08-08**: un agente ha letto per intero **entrambi** i file obbligatori, ha fatto esattamente ciò che gli era stato detto, e ha scritto **un traguardo intero** con gli identificatori in italiano — perché la regola non era in nessuno dei due e il piano dettava nomi italiani. La correzione è costata **sei rinomini di file, undici di funzione**, la traduzione di quattro script e la **rigenerazione dei quattro oracoli** `.stderr`, che è l'atto più delicato del repository (gotcha #25). ⚠️ Il controllo di §13 accoppia le voci ai **file** in `docs/adr/`: è **esatto per ciò che misura**, e cieco a tutto il resto — non è un difetto dello script, è il suo perimetro. ⛔ **Il rimedio non è irrigidirlo**: nessun elenco di «sezioni che contano» resterebbe vero più di una sessione, e uno script che pretende voci per sezioni inventate produce rossi per il motivo sbagliato. Il rimedio è che **una decisione scritta fuori da un ADR va portata a mano nel compendio**, e chi la scrive è **l'unico** che può saperlo. Registrato in §4 del compendio e nella tabella «Come si lavora qui» di `CLAUDE.md`, cioè nei due posti dove chi legge inciampa |
| 41 | **Un filtro che _normalizza l'ingresso_ di un controllo decide anche che cosa il controllo può vedere** | `scripts/gate-deps.sh` legge l'uscita di `cargo tree` e ne estrae i nomi di crate; per scartare le righe che nomi non sono, filtrava con `grep -E '^[a-z0-9_-]+$'`. **Misurato**: una crate col nome che contiene una **maiuscola** non passa il filtro, quindi non entra nell'insieme confrontato con la allow-list, quindi **non compare fra gli intrusi** — e il cancello esce **verde**. È un **falso negativo su I3**, cioè il modo di fallire peggiore per quel controllo: un rosso mancato su un'invariante non si nota mai, mentre un rosso di troppo lo segnala qualcuno. Provato con `Inflector`, crate reale e non un nome costruito, aggiunta al grafo **spedito** del kernel: **uscita 0 prima** della correzione, **uscita 1 e il nome del colpevole dopo**. Rimedio: allargare la classe di caratteri, con la ragione scritta accanto alla classe. Sonda **N5**. ⚠️ **E c'era un secondo strato, che la sola uscita 0 nascondeva**: `Inflector` porta con sé un corteo di dipendenze dal nome tutto minuscolo, che il filtro **lasciava passare**. Il controllo quindi non taceva del tutto — segnalava **il corteo e non il capofila**, pur stampando il capofila dentro ogni catena. Un elenco di colpevoli in cui manca l'unico nome che spiega gli altri è peggio di un elenco vuoto: sembra un rapporto completo |

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
| ❌ ri-derivare l'architettura | è in **37 ADR**, ciascuno con alternative scartate e motivo |
| ❌ riscrivere `tracciabilita.md` da zero | 170 funzionalità già mappate: si **aggiorna**, non si rigenera |
| ❌ ri-cercare lo stato dell'arte già tracciato | è in `riferimenti.md` con le fonti. Verificane semmai l'invecchiamento |
| ❌ rifare gli spike SP-5 e SP-6 | esiti, seed, versioni e comandi sono in [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md). I prototipi esclusi sono recuperabili dalla storia git, lo SHA è lì |
| ❌ rifare le misure **M-1 · M-2 · M-3 · M-4 · M-5 · M-6 · M-7 · M-8 · M-9** | tutte chiuse, con comandi, versioni e sonde: M-1 §6.8 · M-2 §3.6 · M-3 §7.2 · M-4 e M-5 §2.6 · M-6 §5.8.1 · M-7 §2.6 · M-8 §4.7 · **M-9 in [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md)**, con la matrice, i sei ritrovamenti e le quattro divergenze. L'unica aperta è **M5** (senza trattino), e richiede una GUI |
| ❌ riaprire le **due decisioni della §7.3** | erano le uniche domande che la §7 doveva prendere, e sono state prese dopo aver misurato. Riaprirle richiede una misura nuova, non un'opinione |
| ❌ progettare una capacità L2 | prima il kernel deve esistere (ADR-0001) |
| ⛔ **riscrivere il piano del Traguardo 1** | è il registro di ciò che fu osservato eseguendolo, e riscriverlo falsificherebbe la storia. Dove detta una cosa e il repository ne contiene un'altra, parla l'**errata in testa** — quattro voci, e la prima sono gli identificatori italiani |
| ⛔ **«tradurre in italiano» qualcosa in `crates/` o in `scripts/`** | la §1.0 lo vieta, ed è già costato un traguardo intero da rifare — gotcha #40. Restano italiane **solo** le parole che `check-docs.sh` cerca dentro i documenti: `verificato qui`, `parziale`, `rimandato`, `non controllato`, e l'intestazione «Difende» del catalogo. Sono dati confrontati, non identificatori |
| ⛔ **rigenerare in blocco gli `.stderr` di `compile_fail/`** | sono accoppiati al **grafo linkato**, non al solo sorgente del caso: due possono diventare rossi insieme per un motivo estraneo alla regola sotto test, ed è proprio il momento in cui rigenerare è più tentante. Gotcha #25, seconda occorrenza |
| ❌ promuovere l'aiutante `passo_in_dubbio` dello spike | assume esecuzione sequenziale: con l'interlacciamento dà un **falso negativo**. Gotcha #20 |
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
| [`roadmap.md`](roadmap.md) | dodici sotto-progetti, ordine, dipendenze, decisioni aperte |
| [`tracciabilita.md`](tracciabilita.md) | 170 funzionalità → dove vive ciascuna |
| [`README.md`](README.md) | indice di ADR e diagrammi |
| [`porta-di-qualita.md`](porta-di-qualita.md) | **dove vive ogni controllo**: ogni riga del catalogo §7.4 → il file che la implementa, le sonde per nome, e ciò che la porta **non** controlla ancora |
| [`adr/`](adr/) | **37 decisioni architetturali**. Leggi **0001** e **0004** per primi: tutto il resto ne discende. Poi **0026** (linguaggio) se devi scrivere codice |
| [`design/`](design/) | 9 diagrammi Mermaid della struttura corrente |
| [`superpowers/specs/`](superpowers/specs/) | la spec del kernel §0–§10, **e quella del sotto-progetto 1** — §0–§8 complete, con tutte le evidenze delle misure |
| [`superpowers/plans/`](superpowers/plans/) | i due piani scritti finora — lo **stack** e il **Traguardo 1** — entrambi **eseguiti**, ed entrambi con un'**errata in testa** che documenta dove il piano sbagliava. ⛔ Un piano non si riscrive: è il registro di ciò che fu osservato eseguendolo |
| [`../crates/`](../crates/) | **il codice del prodotto.** Cinque crate: `kernel` e `simulator` in `no_std` + `alloc` + `forbid(unsafe_code)`; `platform`, `secrets` e `daemon` sono il posto dove l'I/O deve vivere |
| [`riferimenti.md`](riferimenti.md) | fonti esterne, con data e con **cosa non abbiamo adottato** |
| [`../spikes/`](../spikes/) | **prove, non kernel.** `PROTOCOLLO.md` criteri e soglie · `CANDIDATI.md` pre-selezione · `RISULTATI.md` esiti, seed, versioni, evidenze · `GUI-REQUISITI.md` G1–G21 e P1–P4 |
| [`../spikes/rust/`](../spikes/rust/) | il prototipo vincente: confine dei tipi, esecutore deterministico, `Future` native, giornale write-ahead. **Punto di partenza del simulatore** |
| [`../spikes/gui-ipc/`](../spikes/gui-ipc/) | prototipo IPC con P1–P4 misurati |
| `../scripts/gate.sh` | **la porta di qualità, in un comando solo.** Sei controlli, e gli altri quattro script sono i suoi pezzi: `gate-no-os.sh` · `gate-deps.sh` · `gate-attributes.sh` · `check-docs.sh` |
| `../scripts/check-docs.sh` | controllo di coerenza, verificato anche in negativo. **Da eseguire prima di ogni commit di documentazione** — o `gate.sh`, che lo comprende |

## Come si aggiorna questo handoff

Alla chiusura di ogni sotto-progetto, **nello stesso passaggio**: `roadmap.md`,
`tracciabilita.md`, lo stato degli spike, `CLAUDE.md` se cambia il prossimo passo, e
questo file se emergono gotcha nuovi.

Di [`tracciabilita.md`](tracciabilita.md) è stato toccato **solo il riquadro in testa**,
che conta le voci della riapertura: nessuna funzionalità ha cambiato sede, e la regola dice
di aggiornare la tabella alla *chiusura* del sotto-progetto. Resta quindi da aggiornare
**quando il sotto-progetto 1 chiude**, non alla fine della §8.

### Quattro trappole di `check-docs.sh`, da sapere prima di scrivere

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
verificato sulle §5, §6, §7 e §8, che ne hanno una decina ciascuna.

**3 · Due tabelle della spec sono lette _per posizione_.** I controlli aggiunti dalla §8.6
non fanno analisi del testo: contano le celle.

| Tabella | Cosa lo script pretende |
|---|---|
| il **catalogo** §7.4.1 e §7.4.2 | l'**ultima** colonna è la contro-sonda, e la casella non è vuota. Una riga con meno celle dell'intestazione è un errore, non una scorciatoia tipografica — è così che è stata trovata la riga di V31 |
| le tabelle **§8.3 e §8.4** | **cinque** colonne, con l'ID in prima, lo **stato** in terza e l'**innesco** in quinta. Una colonna aggiunta o spostata rompe entrambe le asserzioni |

⚠️ **E i delimitatori sono intestazioni.** Il catalogo è delimitato da `#### 7.4.1` e
`#### 7.4.3`, la copertura da `## 8.`. Rinumerarle non è un ritocco: senza la guardia di
non-vacuità spegnerebbe i controlli **in verde** — gotcha #26. Con la guardia diventa un
rosso che nomina il delimitatore mancante, ed è il comportamento voluto.

**4 · Un falso positivo in attesa, se qualcuno allarga la lista dei file.** La guardia dei
conteggi gira oggi su sei documenti di stato — `HANDOFF.md`, `roadmap.md`, `README.md`,
`COMPENDIO.md`, `AVVIO-CHAT.md` e `CLAUDE.md`. In [`tracciabilita.md`](tracciabilita.md)
esistono righe come `§4 ADR-0008`, dove il regex leggerebbe `4 ADR` e pretenderebbe il
totale: rosso per il motivo sbagliato, cioè gotcha #24. **Oggi non scatta**, perché quel
file non è nella lista. Se un giorno servisse aggiungerlo, il rimedio è il **regex** — che
non deve accettare una cifra preceduta da `§` — non il documento.

> 📌 **Provata sul campo, per sbaglio, due volte.** Scrivendo questa riga l'esempio è finito
> prima **fuori da un code span**, e poi dentro un code span **spezzato su due righe**: la
> guardia è scattata entrambe le volte su `HANDOFF.md` stesso. È la conferma nella direzione
> che conta — il controllo vede davvero il pattern — e insieme la dimostrazione della
> trappola 1.
>
> ⚠️ **Il sotto-caso che non era scritto:** lo spogliamento dei code span è `sed` **riga per
> riga**. Un code span che va a capo non protegge la parte sulla prima riga, perché il
> delimitatore di chiusura sta sulla seconda. Gli esempi con una cifra **stanno su una riga
> sola**, o si riformulano senza la cifra.

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
