# Roadmap — sotto-progetti, ordine, stato

Piano generale del progetto. **Da aggiornare a ogni sotto-progetto chiuso**, insieme a
[tracciabilità](tracciabilita.md).

Ultimo aggiornamento: **2026-08-10**.

## Stato in una riga

> Spec del kernel **completa e approvata** (§0–§10, 37 ADR). Stack deciso **tranne il
> guscio della GUI**: core in **Rust**, interfaccia web in **Vue 3**, worker ML in
> **Python**; Tauri contro Electron resta aperto ([ADR-0029](adr/0029-guscio-della-gui.md),
> `Proposed`) e **non blocca nulla**.
>
> **Sotto-progetto 1: §0–§8 approvate**, e la riapertura su sette voci è **tutta chiusa**.
> ✅ **Il Traguardo 1 è eseguito il 2026-08-08**: il workspace alla radice con le cinque
> crate, edition **2024**, `rust-toolchain.toml` che appunta **rustc 1.95.0** e il bersaglio
> **`x86_64-unknown-none`**, e la **porta di qualità in un comando solo** —
> `bash scripts/gate.sh`, sei controlli, **`GATE GREEN`**. ⛔ **Zero logica di prodotto**, ed
> è deliberato. La mappa dei controlli è in [porta-di-qualita.md](porta-di-qualita.md).
> La §7 porta la **porta di qualità**: ogni controllo dichiara il proprio
> livello di forza — compilatore, controllo esterno, lint — e porta la sonda che deve
> scattare *e* la contro-sonda che deve restare verde. **Il livello 3 è vuoto**: nessuna
> invariante del kernel poggia su un lint. La §8 porta la **copertura**: ogni V e ogni Q
> con il proprio stato, e ogni rimandato con il proprio **innesco**, preteso dallo script
> e non dalla buona volontà. **Il livello ⛔ è vuoto**: nulla è lasciato deliberatamente
> senza controllo.
>
> ✅ **Le sette voci della riapertura sono tutte chiuse** (2026-08-08). Erano emerse
> rileggendo [tracciabilita.md](tracciabilita.md) con la domanda *«di quale meccanismo di
> kernel ha bisogno questa funzionalità, e la spec lo nomina?»*, e tre erano di classe
> **B**, cioè non retrofittabili.
>
> | Voce | Chiusa con |
> |---|---|
> | i parametri di decisione consegnati al kernel | [ADR-0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md) · §2.8 |
> | la provenienza del totale di VRAM | §5.1 |
> | l'unico punto di uscita verso la rete | §2.3.1 |
> | **F1a** — la porta verso i worker, che completa la riga di verifica di I4 | [ADR-0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) · §2.3.1 |
> | **F2 con F7** — l'evoluzione del formato durevole del giornale | [ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md) · §4.9 |
> | **F1b** — il progetto della porta `process`, e il formato di filo verso i worker | [ADR-0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) · §6.10 |
> | **F4** — la collocazione dell'anello 3, che scritta si è spaccata in **C + B** | §0.4.3 |
>
> ✅ **La §8 è chiusa** (2026-08-08), toccata una volta sola come previsto, e la spec è
> passata per un **audit sezione-contro-ADR** — quaranta rilievi, **tutti chiusi**: l'ultimo
> era la regola 1 del catalogo (§7.1.1), lasciata aperta perché era una decisione e chiusa
> lo stesso giorno. ✅ **Il piano del Traguardo 1 è scritto ed eseguito**, e la spec non ha
> voci aperte.
>
> ✅ **Il Traguardo 2 — il substrato iniettabile — è eseguito il 2026-08-10**: piano percorso
> **per intero, quattordici compiti su quattordici**, fra il 2026-08-09 e il 2026-08-10,
> `GATE GREEN` a ogni compito. ✅ **Le sei famiglie di porte sono complete** — `reactor` ·
> `journal` · `filesystem` · `network` · `process` · `ipc` — e la §3.1 le dichiara esaustive.
> ✅ **Il Traguardo 3 — giornale e formato durevole — è eseguito il 2026-08-10**, dodici
> compiti in due parti: **dodici su dodici**, `GATE GREEN` a tutti. ⚠️ **Questa riga diceva
> «dieci eseguiti, si riprende dal Task 11»**, e la §6 del compendio era già a undici: è la
> lezione della tredicesima misura: la §6 giusta **nasconde** gli altri documenti di stato.
> ✅ **Il brainstorming del Traguardo 4 è chiuso il 2026-08-11** e il disegno è scritto:
> [Traguardo 4 — il disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md).
> ✅ **E il piano è scritto lo stesso giorno**, dieci compiti in tre parti:
> [Traguardo 4](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md).
> ⏭️ **Il prossimo passo è ESEGUIRLO**, subagent-driven. ⛔ **Il disegno ha
> ristretto il traguardo su una misura**: delle dieci righe di guasto della §3.3 **una sola** ha
> oggi il proprio soggetto, quindi il Traguardo 4 costruisce **il motore** della DST e non tutte
> le finte — le altre nove hanno ciascuna il proprio indirizzo fra i Traguardi 5 e 6. ⚠️ **Questa
> riga diceva *«il brainstorming»***, e prima ancora *«scrivere il piano»* quando il primo passo
> non era stato fatto.
> ✅ **Col Task 9 la porta `journal` ha la conformità contro _entrambe_ le implementazioni**,
> tenuta a ogni commit e non misurata una volta sola.
> ⛔ **E col Task 10 il formato durevole è CONGELATO:** i byte di **tre** record sono nel
> repository, con una mappa che il banco rilegge, e **non si rigenerano mai** — se cambiano si
> apre una versione nuova. Tre record e non uno perché i tre enum del formato hanno **otto**
> varianti fra loro, e ciascuna è stata rinumerata per prova: **otto rossi su otto**.
> ⛔ **Il Task 6 ha riportato indietro una domanda invece di deciderla**, ed è la più importante
> del traguardo: `replay()` non dice quale delle due operazioni abbia scritto una voce, mentre il
> giornale lo sa — decisione **D6**, e chiuderla cambia una porta condivisa.
>
> ✅ **La lacuna su I2 è chiusa**: [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md)
> — quota di presentazione sottratta, con la concessione tenuta dal core. Il kernel non
> ha più lacune aperte.

## Il ciclo che seguiamo

Ogni sotto-progetto attraversa gli stessi passi. Non si salta.

```mermaid
flowchart LR
    B["brainstorming<br/>sezione per sezione"] --> S["spec approvata<br/>+ ADR + diagrammi"]
    S --> R["revisione<br/>umana"]
    R --> P["piano di<br/>implementazione"]
    P --> I["implementazione"]
    I --> V["verifica contro<br/>i requisiti Q"]
    V --> U["aggiornamento di<br/>roadmap e tracciabilita"]
```

## Sotto-progetti

| # | Sotto-progetto | Livello | Stato | Dipende da |
|---|---|---|---|---|
| **0** | **Kernel — arbitri e meccanismi** (§0–§9) | L0 + L1 | ✅ **spec completa** | — |
| **0b** | **Kernel L0 fisico** (§10) — archivi, cifratura, backup, segreti, checkpoint, confinamento | L0 | ✅ **spec completa** | 0 |
| **0c** | **Stack completo** — ADR-0026 core, ADR-0027 GUI, ADR-0028 worker ML | — | ✅ **deciso** | SP-5, SP-6 |
| 1 | Implementazione del kernel + simulatore DST | L0 + L1 | 🔵 **in corso** — spec completa (§0–§8, riapertura chiusa, §8 chiusa, audit passato). **Traguardi 1 e 2 ✅ eseguiti**: scheletro e porta di qualità, poi il substrato iniettabile e le sei famiglie di porte, `GATE GREEN`. ✅ **Traguardo 3 eseguito il 2026-08-10**, **dodici** compiti su dodici: il giornale e il formato durevole, coi **byte congelati**. ⏭️ **Traguardo 4** — il simulatore DST — è il prossimo, e si comincia dal **brainstorming** del suo piano, non dalla scrittura. ⚠️ **Questa cella diceva «sette compiti su dodici»** a dodici eseguiti | 0, 0b, 0c |
| 2 | GUI minima (shell, chat, stato) | — | ⬜ | 1, ADR-0027 |
| 3 | Conversazione | L2 | ⬜ | 1, 2 |
| 4 | Agenti | L2 | ⬜ | 3 |
| 5 | Coding | L2 | ⬜ | 4, 0b |
| 6 | Conoscenza / RAG | L2 | ⬜ | 3 |
| 7 | Generazione asset | L2 | ⬜ | 1 · chiude **SP-1** |
| 8 | Voce | L2 | ⬜ | 7 · chiude **SP-2** |
| 9 | Gestione modelli locali | L1 est. | ⬜ | 1 |
| 10 | Integrazione OS completa | L3 | ⬜ | 2 |
| 11 | **Backup e ripristino** — [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md); chiude **V32 · V33 · Q21** | L0 + L3 | ⬜ | 5, 6, 9 |

## Perché quest'ordine

| Scelta | Motivo |
|---|---|
| **SP-5/SP-6 prima di tutto** | possono **escludere un linguaggio**: farli dopo significa riscrivere |
| Kernel prima di ogni capacità | imposto da ADR-0001: parità, nessun accesso privilegiato |
| Conversazione come prima capacità | è il consumatore più sottile di *tutti* i meccanismi del kernel: lo valida da capo a fondo con il minimo codice |
| Agenti prima di Coding | Coding usa gli anelli e i sensori che Agenti introduce per primo |
| Generazione asset prima di Voce | chiude **SP-1** (il rischio più grande) prima; e **SP-2** richiede che esistano *entrambi* — voce e job GPU pesante |
| L3 completo per ultimo | packaging, i18n e accessibilità non sbloccano nulla a monte |
| **Backup dopo indici e pesi** | non è comodità, è non-vacuità. V32 dice che il backup **esclude indici e pesi perché ricostruibili**: prima che 6 e 9 li producano, l'elenco delle esclusioni è vuoto, e verificare V32 su un elenco vuoto è una prova che non può fallire — gotcha #17. Serve inoltre il filesystem reale, che arriva con 5, e l'interfaccia che dichiara le esclusioni **al momento del backup** (follow-up di ADR-0022) |

Non è un ordine per importanza: i quattro pilastri restano paritari (ADR-0001). È un
ordine per **dipendenza e per rischio**.

## Il primo valore utile

Sotto-progetti 1 + 2 + 3: una chat funzionante con contabilità dei costi, giornale,
ripresa dopo crash e stato di degrado dichiarato. È il momento in cui il progetto
smette di essere solo documenti.

Costo accettato in ADR-0001: arriva più tardi che in un'architettura con baricentro.

## Spike aperti

| ID | Domanda | Blocca | Stato |
|---|---|---|---|
| **SP-5** | iniettabilità di tempo, casualità, I/O, scheduling | ⛔ ADR linguaggio | ✅ **chiuso**: solo Rust passa. Go fallisce C6 (9 e 4 tracce distinte su 100 dentro `synctest`), TypeScript parziale |
| **SP-6** | il sistema di tipi regge il confine dei dati non fidati | ⛔ ADR linguaggio | ✅ **chiuso**: Rust e Go passano, TypeScript parziale su T4 e T6 |
| SP-1 | curva qualità/VRAM di TRELLIS2 su 16 GB | profili di risorsa §2 | ⬜ |
| SP-2 | Q1 (voce < 600 ms) sotto carico GPU | taratura corsie §2 | ⬜ |
| SP-3 | budget della proiezione per modello | taratura §5 | ⬜ |
| SP-4 | provider con annullamento senza addebito | ordine di preferenza §3 | ⬜ |

Protocolli e soglie decisionali: [spec §9](superpowers/specs/2026-08-06-kernel-design.md).

## Piani di implementazione

| Piano | Copre | Stato |
|---|---|---|
| [Spike bloccanti e stack](superpowers/plans/2026-08-06-spike-linguaggio-del-core.md) | SP-5, SP-6, ADR-0026, ADR-0027, ADR-0028 | ✅ **eseguito** il 2026-08-06 |
| [Sotto-progetto 1 · Traguardo 1](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) | il workspace con le cinque crate e la **porta di qualità**, eseguibile e provata in due direzioni. **Zero logica di prodotto** | ✅ **eseguito** il 2026-08-08, con un'**errata in testa**: il piano dettava identificatori italiani, la §1.0 della spec vince |
| [Sotto-progetto 1 · Traguardo 2](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) | tempo, casualità, I/O, scheduling; l'**esecutore** in `kernel`; le **sei famiglie di porte** come tratti; i **parametri consegnati** | ✅ **eseguito** il 2026-08-10, `GATE GREEN` — **quattordici compiti su quattordici**, fra il 2026-08-09 e il 2026-08-10, con un'**errata di quarantanove voci** in sei passate. ⛔ Il difetto del piano è quasi sempre più grave di quello del codice, e cambia forma: prima la **sonda sbagliata** (una fuga della cella `Sleep` dipendente dal seme), poi la **sonda assente** (nessun controllo sul cablaggio di produzione né sulle porte), poi l'**artefatto sbagliato** — al Task 11 il piano dettava una porta che compilava, passava la porta di qualità e **non si poteva implementare** — e infine il **compito stantio**: il Task 13 dettava di aggiungere al registro dei controlli righe che i dodici compiti prima avevano già scritto, gotcha **#49** |
| [Sotto-progetto 1 · Traguardo 3](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) | il **record durevole** come enum di versione con indici espliciti, **due implementazioni** del giornale, la **riconciliazione su un insieme**, e i **byte congelati** | ✅ **eseguito** il 2026-08-10, **dodici compiti su dodici**, `GATE GREEN` a tutti. ⚠️ **Questa cella diceva «dieci compiti, si riprende dal Task 11»**, ed è la terza volta che invecchia qui: la §6 del compendio giusta **nasconde** gli altri documenti di stato. ⛔ **Errata di settantasette voci in nove passate**, di cui **nove decisioni** — ⚠️ **questa cella diceva *«settanta in otto»***, ricontate sul piano il 2026-08-10 — il secondo `intent` rifiutato, `Record::encode` senza `Result`, la porta che guadagna **`note()`** con la variante **`RecordKind::Note`**, il record che guadagna **`reason` all'indice 4** perché l'etichetta di fiducia descriva il **contenuto esterno** invece della ragione scritta dal chiamante, e dal Task 8 il ritorno di **`StepId::get()`** — senza cui la porta non è implementabile fuori da `kernel` — con **`open` che non restituisce `JournalError`**, perché nessuna delle sue tre varianti significa *«non ho potuto aprire il file»*; e dal **Task 10** che i record congelati sono **tre e non uno**, perché i tre enum del formato hanno **otto** varianti fra loro e un record ne fisserebbe tre — sta fra le decisioni perché i byte congelati **non si correggono**; e dal **Task 11** la quarta variante di `JournalError`, **`StepInDoubt`**, che non allarga `OutOfOrder` perché quello è definito da **V6** mentre potare troppo presto viola **ADR-0018**, e il **byte dell'operazione** nella tabella `redb`, perché `FileJournal` non poteva rispondere alla domanda: contare i record è sbagliato, una **nota** non è un esito. ⚠️ **Questa cella diceva «sette decisioni»**, e sono **nove** — ricontate sull'errata invece che dedotte. ✅ **La domanda riportata e non decisa — le due verità sul passo — è chiusa dal proprietario il 2026-08-10**: `replay()` non cambia e il `kind` resta nel record. ⚠️ Questa cella diceva *«da eseguire»* a sei compiti fatti: è la lezione della tredicesima misura, la §6 del compendio giusta **nasconde** gli altri documenti di stato. ⛔ **Dodici compiti in due parti**, e la decisione che ne governa l'ordine è controintuitiva e va letta prima: ⛔ **La decisione che ne governa l'ordine è controintuitiva e va letta prima:** i byte congelati sono **l'ultimo** compito, non il primo. Non si rigenerano mai, quindi congelarli prima che un consumatore vero e **due** implementazioni abbiano esercitato il formato congelerebbe la forma sbagliata — il difetto del Task 11 del traguardo precedente, dove però bastava cambiare una firma. ⚠️ **E sette decisioni sono prese dal piano** perché la spec non le fissa, fra cui che una **promozione non è un passo proprio** e che l'**etichetta di fiducia** entra nel record dal primo giorno: è l'unico campo che, mancando, porta informazione che **si perde per sempre** |

⛔ **Il sotto-progetto 1 si esegue a traguardi, e ciascuno ha il proprio piano.** Scriverne
uno per codice che non esiste ancora significa inventare. I sei traguardi sono elencati nel
piano del primo; i successivi si scrivono quando si arriva — **quelli dei Traguardi 2 e 3 sono
scritti, il 2026-08-09 e il 2026-08-10; quelli dal quarto in poi ancora no**.

| # | Traguardo | Deliverable |
|---|---|---|
| **1** | scheletro e porta di qualità | ✅ **eseguito**: cinque crate, sei controlli in un comando, `GATE GREEN` |
| **2** | il substrato iniettabile | ✅ **eseguito** il 2026-08-10, `GATE GREEN`: i due tempi · la porta `Rng` · i parametri consegnati · la porta `Reactor` · l'**esecutore** · l'orologio virtuale · il reattore reale e la prima **suite di conformità** · il cablaggio di produzione in `daemon` · il **confine dei tipi** `Untrusted`/`Instruction` · le **sei famiglie di porte al completo** · il registro dei controlli e la chiusura |
| **3** | giornale e formato durevole | ✅ **eseguito** il 2026-08-10, **dodici compiti su dodici**, `GATE GREEN` a tutti. [Piano](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) **scritto il 2026-08-10**: la porta `journal` a byte con **due** implementazioni giudicate dalla stessa suite, il record come enum di versione, la **riconciliazione**, ✅ **i byte congelati** — nel repository dal Task 10, e non si rigenerano mai. ⚠️ **Questa cella ha detto *«il prossimo … da eseguire»* a nove compiti fatti e *«dieci su dodici»* a dodici**, ed è la lezione della tredicesima misura ripetuta due volte: la §6 del compendio giusta **nasconde** gli altri documenti di stato |
| **4** | il simulatore DST — **il guasto** | ⏭️ **il prossimo, e resta da ESEGUIRE.** ✅ **Brainstorming, disegno e piano tutti il 2026-08-11** — [il disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) e il [piano](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md), **dieci compiti in tre parti**. ⛔ Costruisce **il motore** della DST — giornale cadente, backend cadente, campagna, semi — e **non** le quattro finte mancanti, che nascono col meccanismo che le usa. ⚠️ Questa cella diceva *«tempo virtuale, iniezione dei guasti, la campagna, i semi»*: il **tempo virtuale è del Traguardo 2**, e con esso `C1`, `C2`, `C3` e la non-vacuità. Il `StorageBackend` su cui iniettare **esiste già**, ed è provato sostituibile da fuori la crate |
| 5 | arbitro GPU | ammissione, corsie, ciclo della concessione, le due policy |
| 6 | gli altri meccanismi | gateway, sensori, permessi, degrado, il canale worker |

**Il codice si scrive in questo repository**, non altrove, e la domanda su *dove* è
**chiusa dall'esecuzione**: il workspace delle cinque crate nasce **alla radice**, con
`spikes/` fra gli `exclude` — `spikes/rust/` è un workspace annidato e porta un
`clippy.toml` che a livello di workspace scatterebbe addosso a `platform`, che *deve*
chiamare l'orologio.

Il prototipo [`spikes/rust/`](../spikes/rust/) resta il punto di partenza del simulatore:
contiene già il confine dei tipi, l'esecutore deterministico, l'esecutore su `Future`
native e il giornale write-ahead iniettabile, tutti con i loro test. ⚠️ **Nel Traguardo 1 non
era salito niente; col Traguardo 2 è salito tutto ciò che la §2.5 assegnava a questo
traguardo** — il confine dei tipi a `crates/kernel/src/boundary.rs`, la porta `Rng` con la
sua implementazione seminata in `simulator`, l'esecutore a `crates/kernel/src/executor.rs`,
la porta `journal`, e i casi di `compile_fail`. Restano negli spike le due righe che la §2.5
dichiara **non** debbano salire, `esegui_thread` e il `World` giocattolo; e il **doppio
cadente** del giornale non è un'omissione, perché la §2.5 lo assegna al **Traguardo 4**:
cadere a una scrittura scelta dal seme **è** iniezione di guasti. ✅ **Il giornale
write-ahead è salito col Traguardo 3**, il 2026-08-10, e in **due** implementazioni invece di
una: il doppio in memoria in `simulator` e `redb` col backend nostro in `platform`.

⛔ **Tutto il codice è in inglese** — §1.0 della spec: crate, moduli, tipi, funzioni,
messaggi d'uscita, commenti nel sorgente. La documentazione resta in italiano. La regola
non stava nei due file di lettura obbligatoria, e questo è costato un traguardo intero da
rifare: gotcha **#40**.

**Toolchain sulla macchina**, verificata il 2026-08-06: `rustc` 1.95.0 · `cargo` 1.95.0
· `clippy` 0.1.95. Go 1.26.5 e Node 24.9 restano installati ma non servono più al core.
✅ **Ora è anche appuntata nel repository**: `rust-toolchain.toml` dichiara il canale
`1.95.0`, i componenti `rustfmt` e `clippy`, e il bersaglio `x86_64-unknown-none` del
cancello senza OS. ⚠️ Che il bersaglio sia dichiarato lì ha una conseguenza misurata sulla
guardia che lo verifica — gotcha **#38**.

## Decisioni ancora da prendere

| Decisione | Quando | Vincolata da |
|---|---|---|
| ~~Linguaggio del core~~ | ✅ **ADR-0026: Rust** | — |
| ~~Interfaccia web o toolkit nativo~~ | ✅ **ADR-0027: interfaccia web** (G7) | — |
| ~~Ecosistema dei worker ML~~ | ✅ **ADR-0028: Python** | — |
| ~~Framework dell'interfaccia~~ | ✅ **ADR-0030: Vue 3** | — |
| ⚠️ **Guscio della GUI: Tauri o Electron** | **ADR-0029, `Proposed`.** Si chiude con **cinque** misure M1–M5 all'inizio del sotto-progetto 2 | non blocca il sotto-progetto 1 |
| ~~La GPU usata dalla GUI non è arbitrata~~ | ✅ **[ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md)**, nella §5 della spec del sotto-progetto 1 | I2 è ora verificato su **tutte e tre** le classi di processo |
| ~~Motore di persistenza~~ | ✅ **[ADR-0032](adr/0032-motore-di-persistenza.md): `redb` 4.1.0** con `StorageBackend` scritto da noi | il requisito 4 di §10.6 è stato misurato: solo `redb` lo espone |
| ~~Serializzatore dello schema IPC~~ | ✅ **`bincode` 2.0.1**, misura M-1 nella §6 — prime voci della lista di [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md), che smette di essere vuota | il criterio non era «`no_std`» ma **il grafo transitivo** |
| ~~Evoluzione del formato durevole del giornale~~ | ✅ **[ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md)**: versione **più** indici espliciti, codifica in `kernel`. Misura M-9, §4.9 | la disciplina solo-append è **eliminata dalla misura**: su un formato posizionale non funziona affatto |
| Livello 3 di confinamento (microVM) | quando servirà eseguire codice di provenienza ignota | ADR-0025 |
| ~~Dove vive backup e ripristino~~ | ✅ **chiusa il 2026-08-07: sotto-progetto 11**, dopo 5, 6 e 9. La lacuna l'aveva trovata la §8, che non riusciva a dare un numero all'innesco di V32, V33 e Q21 | [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) · §8.5.2 della spec del sotto-progetto 1 |

### La lacuna su I2 — ✅ chiusa il 2026-08-07

Trovata durante la revisione di ADR-0027, non cercata. Chiusa da
[ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md) nella §5 della spec del
sotto-progetto 1.

Le tre uscite enumerate **non erano tre opzioni per un problema**: erano tre risposte
parziali per **tre consumatori diversi**, trattati fino ad allora come uno solo.

| # | Consumo GPU della GUI | Governo | Rifiuto esecutivo? |
|---|---|---|---|
| 1 | compositing della webview | quota di presentazione sottratta, **concessione tenuta dal core** | ❌ no |
| 2 | viewer 3D entro la quota | stessa quota | ❌ no |
| 3 | viewer 3D oltre la quota | concessione ordinaria via IPC, prelazionabile | ✅ sì |

Il titolare è il **core** e non la GUI, perché la GUI non può *chiedere*: chi alloca è il
compositor, che non ha un percorso di richiesta. E una quota sottratta senza titolare
lascerebbe I2 falso.

⚠️ **Costo dichiarato:** verso il compositor il rifiuto dell'arbitro **non è esecutivo**.
La quota è una promessa di budget, non un'imposizione, e per la GUI I2 vale in una forma
più debole *in natura*. Il valore della quota è **non misurato**: lo chiude **M5**, e
stringe **RK-1** di una quantità oggi ignota.

## Regola di manutenzione

Alla chiusura di ogni sotto-progetto si aggiornano, **nello stesso passaggio**:

1. la tabella dei sotto-progetti qui sopra;
2. le righe corrispondenti in [tracciabilita.md](tracciabilita.md);
3. lo stato degli spike che quel sotto-progetto ha chiuso;
4. `CLAUDE.md` alla radice, se cambia il «prossimo passo».

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
