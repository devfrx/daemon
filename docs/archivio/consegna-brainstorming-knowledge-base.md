# Archivio — la consegna del brainstorming sulla knowledge base, 2026-09-04

⛔ **Non è una lettura obbligatoria.** È il **verbale** della consegna con cui il brainstorming
della knowledge base si è chiuso il **2026-09-04**, tenuto **parola per parola**. Spostato qui il
giorno dopo, quando il disegno è stato scritto **sul posto** — al percorso che la consegna occupava —
con la regola di `CLAUDE.md`: *un documento vivo porta ciò che è vero adesso, e un verbale va in
archivio*. È il viaggio che la consegna stessa prevedeva, e il precedente è
[quella dei gesti](consegna-brainstorming-gesti.md).

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Il disegno vivo è
[`../superpowers/specs/2026-09-04-knowledge-base-design.md`](../superpowers/specs/2026-09-04-knowledge-base-design.md);
il prossimo passo sta nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md), in un posto solo.

⚠️ **Una sola cosa è cambiata rispetto al testo consegnato:** i percorsi relativi dei collegamenti,
riscritti per questa cartella perché `check-docs.sh` li verifica. Nessuna parola è stata toccata;
lo prova `diff` fra questo file, dalla riga sotto la recinzione in giù, e la consegna a `07ab6dc`:
`git show 07ab6dc:docs/superpowers/specs/2026-09-04-knowledge-base-design.md`.

---

# Knowledge base: il disegno

⚠️ **QUESTO FILE È NATO COME CONSEGNA del brainstorming del 2026-09-04**, e la sessione successiva lo
**riscrive sul posto**, allo stesso percorso, come disegno intero — la regola del proprietario del
2026-09-02: brainstorming in **una** sessione, disegno nella **successiva**, con la consegna in un file
**tracciato**. È la forma che ha retto per il
[disegno dei gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md), nato allo stesso modo e riscritto il
giorno dopo; la sua consegna vive parola per parola in
[`archivio/consegna-brainstorming-gesti.md`](consegna-brainstorming-gesti.md), e questa
farà lo stesso viaggio. Il percorso non cambia perché il puntatore della §6 del
[compendio](../COMPENDIO.md) non cambi casa.

Le **cinque sezioni** sono **approvate dal proprietario, una per volta, in chat il 2026-09-04**, e il
loro testo sta nella sezione 6 di questo file **così com'è stato approvato**, con le modifiche che il
proprietario ha chiesto durante l'approvazione già dentro. Chi riscrive **non ne tocca il merito**:
aggiunge ciò che misura, e dichiara ciò che risulta più debole di come era detto.

⚠️ **Non è una spec.** Come i disegni dei Traguardi 4, 5 e 6 e dei gesti, fissa il **perimetro**, le
**forme** che gli ADR descrivono a parole, e per ogni artefatto **il controllo che lo esercita**. Gli
ADR restano l'autorità, e ciò che questo disegno vi aggiunge lo **dichiara** come rimando in append,
nella sezione 3 del testo approvato.

📌 **Metodo.** Ogni affermazione porta la sua specie — **verificata** (letta nel sorgente o in un
documento del repository, con la data), **dedotta**, o **assunta** — e le tre sono separate nella
sezione 5. Le affermazioni sul sorgente sono state lette il 2026-09-04 contro `c3c7a5d`. I comandi
stanno accanto alle affermazioni e **si rilanciano**, non si citano.

---

## 0. Lo stato del repository alla chiusura del brainstorming — 2026-09-04, coi comandi

| | Stato, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git fetch --all --prune && git status -sb`. Nessuno stash, nessuna operazione a metà |
| Il commit da cui questa consegna parte | `c3c7a5d` — la chiusura della sola voce aperta della sessione del compito 9 dei gesti. I commit di questa sessione: `git log --oneline c3c7a5d..HEAD` |
| Codice di prodotto | **non toccato**: `git diff --stat c3c7a5d..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende nulla |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura della sessione, exit 0; `bash scripts/check-docs.sh` → `OK` a ogni commit. Si rilanciano, non si citano |
| Il compendio e il suo tetto | `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh`. Misurato il 2026-09-04 prima di questa consegna: **188 416 − 174 962 = 13 454** byte di margine; la consegna ne consuma una parte, e il piano dei documenti lo **rimisura** |
| Fine-riga | il compendio è LF nell'indice e **CRLF** nell'albero, CR = righe: `tr -cd '\r' < docs/COMPENDIO.md \| wc -c` contro `wc -l`. Ogni scrittura è passata da `replace_unique.py` — l'aiutante che vive nel [piano dei gesti](../superpowers/plans/2026-09-03-riconoscimento-gesti.md), `sed -n '35,70p'` — e i CR sono stati rimisurati dopo. Questo file: `git ls-files --eol docs/superpowers/specs/2026-09-04-knowledge-base-design.md` |
| File temporanei | nessuno nel repository: i testi di sostituzione stanno nello scratchpad, fuori dall'albero |

---

## 1. Le regole di questo brainstorming, decise dal proprietario

| Regola | Da dove viene |
|---|---|
| La strada è quella del repo: **brainstorming → disegno scritto → piano → esecuzione** | scelta 5 del 2026-09-02, in testa al [disegno della chiusura](../superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md) |
| ⛔ **Ogni decisione si controlla contro i cinque criteri di `anthropic-skills:decision-principles`**, e i principi governano la decisione senza occuparla. Il proprietario lo ha ripetuto in chat il 2026-09-04: *«lavoriamo e progettiamo sempre secondo i principi di questa per favore»* | accettazione condizionata del proprietario, confermata oggi |
| Brainstorming in **una** sessione, disegno nella **successiva**, con la consegna in un file **tracciato** | scelta del proprietario del 2026-09-02 |
| Questo brainstorming è **distinto** da quello dei gesti, e ha **una** domanda da chiudere: *se la knowledge base pretenda un meccanismo di kernel «che non si aggiunge dopo», o sia tutta L2 nel sotto-progetto 6* | voce 2 della §7.8 del disegno della chiusura; la §3 del compendio per «non si aggiunge dopo» |
| **Non si disegna la capacità**: il compendio (§8) vieta di progettare una capacità L2 prima del suo sotto-progetto. Qui si decide che cosa la knowledge base **chiede al kernel**, e dove va | compendio §8, riga *«progettare una capacità L2»* |
| Codice in inglese, documenti in italiano; nessun numero senza comando; nessuna fonte senza data | [`CLAUDE.md`](../../CLAUDE.md) |
| **Prima a parole, poi lo schema**: il proprietario non è operativo in Rust, e ogni scelta gli è stata spiegata a parole prima della tabella | `CLAUDE.md`, *«Ma prima a parole»* |

**Le premesse dette dal proprietario**, che il disegno onora, con le sue parole: *«un archivio unico,
organizzato bene con dei file di routing verso dei gruppi o sottogruppi specifici, ogni file è contenuto
qui dentro a partire dalle skill»*; *«l'agente possa navigarci tranquillamente ed evitando di saturare
contesto, bruciare token»*; *«un puntatore stale è peggio di nessun puntatore»*; *«non riorganizzi le
cartelle, mappi quello che c'è già»*; *«vorrei che la ricerca fosse efficiente, consistente e che
funzioni BENE, non a volte sì a volte no»*; e il test di completamento: *«una sessione nuova di zecca
trova il file giusto attraverso i router al primo salto»*.

---

## 2. Le domande d'apertura e le risposte del proprietario — 2026-09-04

⛔ **Sono sue e non si riaprono senza di lui.** Ogni riga porta anche ciò che è stato **scartato**,
perché chi riprende sappia che cosa non rifare.

| # | La domanda | La risposta | Scartato, e perché |
|---|---|---|---|
| 1 | a che cosa serve | **un archivio unico a mappa**: un file centrale → file *router* → gruppi e sottogruppi di file → skill, e ogni skill rimanda al suo router. Progetti, note, tutto dentro. L'agente **salta**, non fruga. Più una pagina visuale della mappa | le quattro opzioni proposte (roba da fuori / memoria del lavoro / separate / mescolate): il proprietario ha risposto **con la propria forma**, che le contiene tutte in un archivio solo |
| 2 | mappa o motore di ricerca | **mappa prima, ricerca sopra dopo**: la mappa dei router è la struttura e funziona senza GPU; la ricerca per somiglianza si aggiunge dopo, sulla stessa cartella | *solo la mappa* (ciò che non è nella mappa non esiste); *solo ricerca* (il RAG classico: niente struttura leggibile, e non è ciò che ha descritto) |
| 3 | chi legge e chi scrive la cartella | **solo il nostro assistente**, attraverso il kernel. ⚠️ **Con una correzione del proprietario:** *non è il routing a scegliere il modello* — OpenRouter li mette a disposizione, **lui** sceglie, e può tenere sempre GLM o DeepSeek. Verificato: ADR-0011 riga 21 dice che le scelte *«derivano da una configurazione»*, ADR-0012 riga 60 che *«un vincolo posto dall'utente resta un vincolo»*. Il routing **applica** la scelta a ogni richiesta e la **annota**; è un impiegato, non un capo. Il suo `claude.md`/`deepseek.md` è una **guida per modello**, una foglia della mappa che il registro delle guide inietta per quel modello; la mappa è **una** | *anche altri agenti da fuori* (Claude Code, Codex, Cursor): file con struttura convenzionale, router che marciscono per mano altrui, e una skill riscritta da un altro strumento che entra nel contesto — AUD-004 per forza; *altri leggono, solo noi scriviamo*: una convenzione, non un controllo. Se un giorno altri strumenti dovranno vederla, sarà un'**esportazione** a parte |
| 4 | chi naviga: l'agente o il kernel | **a piani**: **0** il kernel carica sempre il router centrale e quello dell'ambito, da chiavi meccaniche, senza giudizio — deterministico, si testa nel simulatore; **1** il modello legge il router (poche voci) e chiede una foglia — ogni salto è un passo giornalato col suo costo, **misurato**, e un sensore lo controlla; **2** la ricerca, deterministica dato l'indice, dopo. Il piano 1 non si può rendere deterministico (ADR-0020), ma si tiene piccolo, si misura, e la mappa si migliora con l'anello di ADR-0009 invece del prompt | *più kernel e meno modello* (router con parole-chiave abbinate dal kernel): più rigido, e più lavoro del proprietario per tenere le chiavi — offerto e non scelto |
| 5 | chi scrive le note di memoria | **l'assistente, quando giudica che vale**, come un umano: una scrittura di file giornalata come ogni altra; il giornale resta la verità, la nota è ciò che ha scelto di ricordare | *il sistema in automatico dal giornale* (un meccanismo nuovo di proiezione giornale → file, e file non modificabili); *tutti e due* (due specie di file nella stessa mappa) |
| 6 | la pagina visuale | **un pannello della GUI** che disegna un **indice rigenerabile tenuto dal core**, via IPC, aggiornato come la mano di ADR-0039. ⚠️ **Il consiglio iniziale era l'artefatto HTML autonomo**, e il proprietario ha chiesto *perché non la GUI*: rispondendo è caduto il motivo «arriva prima» — la GUI (sotto-progetto 2) esiste **prima** del 6, letto in roadmap — e il consiglio è cambiato **sul merito**, non per insistenza | *l'artefatto HTML autonomo* (portabile, ma niente click-per-aprire né aggiornamento vivo): se mai, è un'**esportazione** dello stesso indice, dopo; *tutti e due* (due disegnatori della stessa mappa) |
| 7 | la foto catturata con un gesto — decisione 7 del disegno dei gesti | **nella knowledge base, e la run la vede**: la cartella È l'archivio, la foto atterra in un gruppo, il router segue, la run riceve il riferimento | *solo nella run, poi decide l'assistente*: due posti per un file, e una foto dimenticata non è nella mappa |
| 8 | la strada | **B** — i meccanismi già decisi si costruiscono nel kernel **prima** della prima capacità che li usa; la knowledge base resta L2 nel 6 | **A** (tutto nel 6: due strade nel sotto-progetto 3, e un ADR che supera ADR-0009); **C** (la knowledge base come porta di kernel: il kernel saprebbe cosa sono «gruppi» e «router», vietato da ADR-0001 — sfoggio) |
| 9 | le azioni dal pannello: aggiungere file al contesto, creare cartelle, aprire, **le CRUD**, e lo spostamento **dentro e fuori dallo spazio** | **tutte funzioni del registro di ADR-0038**, con la propria tripla di permesso, giornalate, checkpointate dentro l'ambito; «aggiungi al contesto» è **una** funzione con **due invocatori**, il click e il modello. Lo spazio designato **è** l'ambito di ADR-0024 | — (non cambia nulla di quanto deciso prima: atterra su una decisione del giorno precedente) |
| 10 | collegamenti o posizione fisica: su cosa si basa la mappa e il pannello | **i collegamenti decidono la struttura; la posizione è un attributo del nodo.** L'agente naviga la mappa, mai le cartelle; il pannello disegna la mappa; un gruppo è una **voce di router**, non una cartella; il pannello mostra **orfani** e **collegamenti rotti** perché il sensore li misura; i collegamenti vanno a **file**, non ad ancore | *le cartelle come struttura*: il pannello mostrerebbe una cosa e l'agente ne userebbe un'altra |
| 11 | l'ordine dopo il piano dei documenti | **2, poi 13, poi 3** | *13, poi 2, poi 3*: la GUI aspetterebbe, e il 13 aspetta comunque AUD-004 |
| 12 | il pannello della mappa nasce col 6 o col 2 | **col 6**: è la UI di una capacità, nasce con la capacità | *col 2*: il 2 disegnerebbe un indice che nessuno produce ancora |

---

## 3. I «buchi logici», sciolti coi documenti del repo

Il proprietario ha detto che la sua idea era *«grezza»*. Ciascun buco ha una casa già scritta.

| Il buco | Sciolto da |
|---|---|
| «chi aggiorna il router quando un file si sposta?» | l'assistente, come scrittura giornalata (ADR-0007) dentro un ambito con checkpoint (ADR-0024); e un **sensore** (ADR-0009 — `Sensor::observe(&Untrusted)` esiste in `crates/kernel/src/sensor.rs`) che verifica i router: un puntatore rotto è un **verdetto negativo** che rientra nell'anello |
| «e se il router marcisce lo stesso?» | l'anello di miglioramento (ADR-0009): il kernel vede la ricorrenza e **propone** la voce; il proprietario approva. La regola «il router si aggiorna nello stesso turno» diventa un meccanismo, non una speranza. ⚠️ **È la lezione di questo stesso repository:** la regola «un puntatore si toglie, non si ricorregge» ha ceduto tre volte finché era solo scritta; il freno è nel cancello — `check-docs.sh` — e non nella buona volontà |
| «come non saturo il contesto?» | la proiezione ha un **budget per modello** (ADR-0010), misurato per categoria nel giornale; la mappa è una categoria; le foglie entrano come **riferimenti** e si rileggono su richiesta (ADR-0008) |
| «e se modifico un file a mano?» | trigger su cambiamento di file (ADR-0009) → contenuto da fuori (I6, ADR-0014) finché il proprietario non approva. Una skill cambiata → AUD-004 |
| «come so se funziona BENE?» | ogni salto è un passo con costo (ADR-0011): salti-per-trovare e salti-a-vuoto sono **numeri nel giornale**; il test «al primo salto» si misura. La valutazione probabilistica sta in L2 (ADR-0020) |
| «la foto in che contesto?» | decisione 7: nella knowledge base come artefatto, la run la vede come riferimento (ADR-0018, 0022, 0008) |
| «cosa è privato e resta fuori dagli indici?» — dal concetto del proprietario | ⚠️ **NON sciolto qui.** I segreti stanno nel gestore dei segreti (ADR-0023), mai in file. «Privato ma non segreto» è una regola della capacità: **voce aperta** per il sotto-progetto 6, sezione 7 |
| «come funziona se uso modelli diversi?» | il modello **non tocca mai il disco**: il kernel compone il contesto (ADR-0008) e lo consegna. Ciò che cambia col modello è la **guida** (ADR-0009, una foglia per modello) e il **budget** (ADR-0010); la mappa è una. Se il proprietario tiene sempre un modello, il meccanismo esiste e non scatta mai: costo zero |

---

## 4. L'approccio scelto: la strada B, e le due scartate

**La risposta alla domanda della voce 2:** **nessuna sesta proprietà** «che non si aggiunge dopo». Tutto
ciò che la mappa chiede è già deciso in un ADR. C'è però un **vincolo d'ordine**, ed è questo che non
si aggiunge dopo: registro delle guide, trigger e proiezione — decisi in ADR-0009, 0008 e 0010, **zero
codice** al 2026-09-04 — si costruiscono in un sotto-progetto piccolo di kernel **prima** della prima
capacità che inietta una guida; altrimenti quella capacità si inietta le skill a modo suo, nascono due
strade, e una skill installata senza impronta all'approvazione non si può più verificare dopo — il buco
di AUD-004 reso permanente.

| | Strada | Cosa compra | Cosa costa |
|---|---|---|---|
| **A** | tutto L2 nel sotto-progetto 6, kernel intatto | niente da decidere oggi | il sotto-progetto 3 arriva prima del 6 e ha bisogno delle skill: o aspetta, o si inietta le guide a modo suo → **due strade**. Un registro delle guide in L2 contraddice ADR-0009: servirebbe un ADR che lo supera |
| **B** ✅ | i meccanismi già decisi si costruiscono nel kernel **prima** della prima capacità che li usa; la knowledge base resta L2 nel 6, appoggiata sopra | **una** strada; ADR-0009 onorato senza ADR nuovi; il piano 0 deterministico e testabile nel simulatore | un sotto-progetto di kernel prima del 3; e **AUD-004 va decisa prima** |
| **C** | la knowledge base come meccanismo di kernel: una porta `knowledge` nuova, i router gestiti dal core | massimo determinismo | il kernel saprebbe cosa sono «gruppi» e «router» — una **funzionalità utente**, vietata da ADR-0001. E i router sono file `.md`: ADR-0022 dice già che sono artefatti dell'utente. **Sfoggio** |

Un precedente nel codice da riusare, verificato: `crates/kernel/src/permission.rs` dice che *«quali
permessi sono attivi ora è una PROIEZIONE del giornale, non un secondo archivio»*. «Quali guide sono
approvate ora» è la stessa forma.

---

## 5. Verificato, dedotto, assunto

### Verificato nel sorgente, il 2026-09-04, contro `c3c7a5d`

| Affermazione | Comando |
|---|---|
| il registro delle guide **non esiste** nel codice; nemmeno i trigger né la proiezione come tipi | `grep -rl --include='*.rs' -E 'Guide|Trigger|Projection' crates/` rende **0** file |
| il tratto `Sensor` esiste, con `declared_cost()` e `observe(&self, artefact: &Untrusted) -> Verdict` | `sed -n '/^pub trait Sensor/,/^}/p' crates/kernel/src/sensor.rs` |
| `CheckpointId` esiste nella porta `filesystem`; il tipo «ambito» no | `grep -rnE --include='*.rs' 'pub (struct|enum) \w*(Scope|Checkpoint)\w*' crates/` |
| `Untrusted` e `Instruction` esistono ed entrano in molti file | `grep -rl --include='*.rs' 'Untrusted' crates/ \| wc -l` |
| «attivo ora è una proiezione del giornale, non un secondo archivio» è la forma di `permission.rs` e `degradation.rs` | `grep -rn --include='*.rs' -i 'projection' crates/kernel/src/` |
| i moduli pubblici di `kernel`, con `sensor`, `gateway`, `permission`, `degradation` | `grep -n '^pub mod' crates/kernel/src/lib.rs` |

### Verificato nei documenti, il 2026-09-04

| Affermazione | Dove |
|---|---|
| la knowledge base ha già una sede: il sotto-progetto **6** «Conoscenza / RAG», che dipende dal 3; il 3 dipende da 1 e 2 | [`roadmap.md`](../roadmap.md), tabella «Sotto-progetti» |
| `Skills` e `Regole e vincoli di progetto` poggiano sul registro delle guide con **sede da assegnare**; il conteggio: `grep -cE 'sede da assegnare' docs/tracciabilita.md` rende **3** | [`tracciabilita.md`](../tracciabilita.md) |
| le diciotto funzionalità della sezione 3 «Conoscenza» | idem, sezione 3 |
| **AUD-004**: se le difese di ADR-0015 si estendano alle skill *«NON è deciso — vuole un ADR proprio, del proprietario»* | [`audit-2026-08-27.md`](../audit-2026-08-27.md), tabella «Stato dei rimedi» e scheda AUD-004 |
| il registro delle guide fa *«archiviazione, versionamento, iniezione nella proiezione»*, e le skill dichiarative *«sono guide»*; i trigger partono da *«pianificazione, cambiamento di file, fine di un'altra run»* | [ADR-0009](../adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) |
| il routing risolve una **configurazione** e la giornala; «modello preferito» è un vincolo dell'utente | [ADR-0011](../adr/0011-routing-risolto-e-giornalato-per-richiesta.md) riga 21 e 51; [ADR-0012](../adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md) righe 42 e 60 |
| artefatti: non cifrati, **nel backup**; indici ed embedding: **no**, rigenerabili | [ADR-0022](../adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) |
| il registro delle funzioni: *«un registro, molti invocatori, lo stesso permesso»*; lo costruisce *«il primo invocatore, il click del sotto-progetto 2»*; la manipolazione della GUI **non** passa dal registro | [ADR-0038](../adr/0038-registro-delle-funzioni-del-programma.md), e la voce §5 del compendio |
| la decisione 7 dei gesti: *«dove finisce la cattura → il brainstorming 2, la knowledge base: run corrente · knowledge base · entrambe»*; una cattura è un artefatto, riferimento nel giornale | [disegno dei gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md), tabella delle decisioni e §1.5; [ADR-0039](../adr/0039-telecamera-come-sorgente-di-percezione.md), riga *«la destinazione di una cattura»* |
| G7 — artefatti con anteprima viva — è nella mappa funzionale, area 2 | `spikes/GUI-REQUISITI.md`, riga G7 |
| «conversazione e **conoscenza**» è già il primo pilastro | [ADR-0001](../adr/0001-architettura-a-kernel-con-capacita-paritarie.md), e la §1 del compendio |
| il tetto del compendio | `grep -n '^ceiling=' scripts/check-docs.sh` |

### Verificato nello stato dell'arte — nessuna fonte esterna, e dichiarato

⚠️ **Questo brainstorming non ha aperto nessuna fonte esterna, deliberatamente:** ogni decisione presa
oggi poggia su decisioni **del repository**, e sul progetto stesso la fonte più aggiornata è il
progetto. Il concetto di partenza del proprietario — i tre livelli, i router, il *visual second brain* —
è **suo**, e sta nella sezione 1 con le sue parole. ⛔ **Il piano 2 — embedding, indici, ricerca
ibrida — vorrà fonti primarie con la data** quando il sotto-progetto 6 lo disegnerà: non qui, perché
qui non si disegna.

### Dedotto, e dichiarato tale

| Deduzione | Da che cosa |
|---|---|
| che «guida per modello» sia il modo giusto di realizzare il `claude.md`/`deepseek.md` del proprietario | ADR-0009 (iniezione per contesto) + ADR-0010 (budget per modello) + ADR-0011 (il modello risolto nel record) |
| che le **due pretese** sul registro delle guide — chiave di contesto; provenienza e impronta con «approvate ora» come proiezione — siano **tutte** le pretese della mappa | la lettura dei piani 0 e 1 contro ADR-0009 e `permission.rs`. Il disegno le mette alla prova sezione per sezione |
| che l'«ambito» del piano 0 sia l'**ambito di lavoro** di ADR-0024, riusato | ADR-0024: un insieme di percorsi dichiarato; `CheckpointId` esiste. La forma la dà il sotto-progetto 13 |
| che nessun ADR nuovo serva, e bastino rimandi datati | la regola append-only di `CLAUDE.md`: *completato → un rimando* |
| che «tenuto dal core» per l'indice non violi I1 | è derivato e rigenerabile, come il degrado: la forma di `degradation.rs` |
| la forma dell'indice (nodi, attributi, frecce, segnali) nella sezione 4 del testo approvato | è la proposta minima; il 6 la mette alla prova |

### Assunto, e chi lo misura

| Assunzione | Chi la misura |
|---|---|
| che il sotto-progetto **3** sia la prima capacità che inietta una guida — la roadmap non lo dice | la §5 del testo approvato: se non lo è, il 13 va comunque prima della prima che lo fa |
| che la mappa (router centrale + router dell'ambito) stia nel budget del modello **più piccolo** che il proprietario userà | il sotto-progetto 6, sul primo modello locale |
| che «al primo salto» si ottenga davvero | il 6, coi numeri del giornale |
| che misurare la proiezione per categoria costi poco | il 13, quando la costruisce |

---

## 6. Il testo approvato, sezione per sezione

Le sezioni sono qui **come approvate in chat il 2026-09-04**, con le modifiche chieste dal proprietario
durante l'approvazione già dentro (la riga **g** della §1; la riga del registro e la regola **4** della
§2; l'ordine e il pannello nella §5). Chi riscrive il disegno ne conserva il **merito**.

#### Sezione 1 — Il perimetro, approvata il 2026-09-04

**1.1 Che cosa decide ora**

| | Decisione |
|---|---|
| **a** | La knowledge base è una **capacità L2 del sotto-progetto 6**: archivio unico di file, a **mappa** (router → gruppi → foglie), skill comprese, **una guida per modello** come foglia, note scritte dall'assistente quando giudica che vale, le catture atterrano qui |
| **b** | La tocca **solo il nostro assistente**, attraverso il kernel. Il modello lo sceglie il proprietario; il routing lo applica e lo annota (ADR-0011) |
| **c** | Navigazione **a piani**: **0** il kernel carica sempre il router centrale e quello dell'ambito, senza giudizio · **1** il modello salta a una foglia, e ogni salto è un passo giornalato con il suo costo · **2** la ricerca per somiglianza, **dopo**, dentro il 6 |
| **d** | ⛔ **Il verdetto sulla domanda della voce 2:** **nessuna sesta proprietà** «che non si aggiunge dopo». C'è un **vincolo d'ordine**: registro delle guide, trigger e proiezione — decisi in ADR-0009, 0008, 0010, **zero codice** — si costruiscono in un **sotto-progetto piccolo di kernel PRIMA del 3**, o la prima capacità si inietta le skill a modo suo e nascono due strade |
| **e** | Il registro delle guide deve rispettare **due pretese** della mappa: la **chiave di contesto** esprime *ambito, run, modello*; ciò che conserva porta **provenienza e impronta** all'approvazione, e «approvato ora» è una **proiezione del giornale** — la forma di `permission.rs` |
| **f** | La pagina visuale è un **pannello della GUI** che disegna un **indice rigenerabile tenuto dal core**, via IPC, aggiornato come la mano di ADR-0039. L'HTML autonomo, se mai, è un'esportazione dello stesso indice |
| **g** | Il pannello agisce **solo attraverso il registro delle funzioni** (ADR-0038). La capacità registra le **CRUD** dei propri file e gruppi — creare, leggere, aggiungere al contesto, aggiornare, spostare (dentro lo spazio, dentro da fuori, fuori), cancellare — come funzioni del registro: ognuna è un effetto con classe dichiarata, giornalato, checkpointato dentro l'ambito. «Aggiungi al contesto» è **una** funzione con **due invocatori**: il click e il modello |

**1.2 Che cosa rimanda, e a chi**

| Voce | A chi |
|---|---|
| **AUD-004**: le skill come contenuto non fidato — se le difese di ADR-0015 (testo integrale, impronta, sospensione se cambia) si estendono alle skill | il **proprietario**, con un ADR suo, **prima** del sotto-progetto 13: il registro deve saperlo per nascere giusto |
| nome e posto del sotto-progetto di kernel nella roadmap | il proprietario, nella §5 di questo disegno — proposto **13** |
| il disegno **del** sotto-progetto 13 (come si costruiscono i tre meccanismi) | il **suo** brainstorming, come ogni traguardo. Qui si fissano solo le pretese di 1.1e |
| il piano 2 — ricerca, embedding, indice, GPU | sotto-progetto 6, seconda metà. Qui **non si disegna** |
| sintassi dei router, forma dell'indice, aspetto del pannello | sotto-progetti 6 e 2: sono la capacità, e il compendio vieta di disegnarla prima |
| se l'«ambito» del piano 0 sia l'**ambito di lavoro** di ADR-0024 | §2 di questo disegno, a parole; la forma la dà il 13 |

**1.3 Che cosa esclude**

- Altri strumenti che leggono o scrivono la cartella (decisione **b**).
- Ingest di documenti (PDF, OCR, web): righe L2 già in `tracciabilita.md`, sotto-progetto 6.
- Un record nuovo del giornale, una porta nuova, la cifratura della knowledge base, regole di backup
  nuove: **niente di tutto questo**, ed è misurabile.
- Riaprire ADR-0009 (le guide sono meccanismo di kernel), ADR-0022 (i file sono artefatti
  dell'utente), ADR-0020 (nessun modello nel kernel).

**1.4 I «buchi logici», sciolti coi documenti del repo** — la tabella è la sezione 3 di questa
consegna, e il disegno la porta qui.

**1.5 Il prodotto di questo disegno, e il controllo che esercita ciascun artefatto**

| Artefatto | Controllo |
|---|---|
| questo disegno, riscritto dalla sessione successiva | `check-docs.sh` (link, tetto); la sezione verificato/dedotto/assunto; ogni cifra col comando |
| rimando datato in testa ad **ADR-0009**: le due pretese e l'ordine | ADR append-only; `check-docs.sh` accoppia la §5 del compendio |
| rimando in **ADR-0039** («destinazione di una cattura») e decisione 7 del disegno gesti → **chiusa** | `grep` sulla frase; la tabella delle decisioni dei gesti |
| `roadmap.md`: riga nuova del sotto-progetto di kernel; riga 6 aggiornata (mappa poi ricerca; dipende dal nuovo) | «nessuna rinumerazione»; conteggi di `check-docs.sh` |
| `tracciabilita.md`: `Skills` e `Regole e vincoli` → sede assegnata; `Collezioni e knowledge base` → 6, con la forma | `grep -cE 'sede da assegnare'` **scende di due** — provato nelle due direzioni |
| compendio §6 (puntatore) e §12 (riga); `README.md` (riga) | tetto; link |
| **codice: nessuno** | `git diff --stat -- crates/` vuoto, a fine piano |

#### Sezione 2 — La forma nel kernel, approvata il 2026-09-04

**2.1 In una frase.** Il kernel **fornisce tre meccanismi** che mancano e **riusa quattro** che esistono.
La capacità (L2, sotto-progetto 6) li **compone**. **Nessuna porta nuova.**

**2.2 I pezzi, e dove vive ciascuno**

| Pezzo | Vive in | Esiste al 2026-09-04? | Cosa fa per la mappa |
|---|---|---|---|
| **registro delle guide** (ADR-0009) | `kernel` | ❌ → sotto-progetto 13 | conserva skill e guide-modello con **provenienza e impronta**; «approvate ora» è una proiezione del giornale (forma di `permission.rs`); **inietta per chiave** — ambito, run, modello |
| **trigger** (ADR-0009) | `kernel` il meccanismo, `platform` la sorveglianza dei file | ❌ → 13 | un file della cartella cambia → evento → anello di verifica. Una modifica a mano diventa «da fuori» |
| **proiezione** (ADR-0008, 0010) | `kernel` | ❌ → 13 | compone il contesto a ogni passo: la mappa è una **categoria con budget per modello**; le foglie sono **riferimenti** |
| **sensore** di integrità dei router | capacità (6), sul tratto `Sensor` | tratto ✅, sensore ❌ | osserva i router come `Untrusted`, verdetto negativo su un puntatore rotto → rientra nell'anello |
| **ambito** della cartella (ADR-0024) | `kernel` | `CheckpointId` ✅, il tipo «ambito» ❌ | la cartella è un ambito dichiarato: ogni scrittura è checkpointata |
| **registro delle funzioni** (ADR-0038) | `kernel`, lo costruisce il sotto-progetto 2 | ❌ | le azioni del pannello e i salti del modello sono **invocatori** delle stesse funzioni; **la capacità vi registra le CRUD dei propri file e gruppi, spostamenti compresi** |
| **indice** della mappa | derivato dalla capacità, tenuto dal core, **rigenerabile** (ADR-0022: fuori dal backup) | ❌ | ciò che il pannello disegna, via `ipc` |
| **la cartella** su disco | artefatti dell'utente (ADR-0022): non cifrata, **nel backup** | — | router, foglie, guide, catture |
| **confine dei tipi** (ADR-0014) | `crates/kernel/src/boundary.rs` | ✅ | ogni foglia è `Untrusted` o `Instruction`, e l'etichetta è ereditaria |

**2.3 Quattro regole di forma**

| | Regola | Da |
|---|---|---|
| 1 | il kernel **non legge mai il testo** di una foglia per decidere: il piano 0 usa solo **chiavi** (ambito, run, modello) | ADR-0020 |
| 2 | ogni scrittura nella cartella è un **effetto giornalato con classe** dentro l'ambito: niente scrive «di lato» — né il pannello, né il modello, né la capacità | ADR-0007 · 0024 |
| 3 | una foglia entra **sempre** con provenienza ed etichetta; il registro delle guide **rifiuta** una guida senza impronta | ADR-0014 · 1.1e |
| 4 | **lo spazio designato è l'ambito, e il confine conta.** Dentro (crea, aggiorna, rinomina, sposta fra gruppi, cancella): checkpointato, **reversibile**, il router segue e il sensore verifica. **Da fuori a dentro**: il file entra come `Untrusted` con provenienza *«importato da ‹percorso› al passo ‹S›»*; se è una skill, serve l'approvazione — AUD-004. **Da dentro a fuori**: il router **perde** la voce; il giornale **tiene** il riferimento **con la destinazione** — un file uscito e uno cancellato non devono essere indistinguibili, la forma di ADR-0018; ⚠️ da lì in poi il checkpoint **non lo copre più**, il limite dichiarato di ADR-0024 | ADR-0024 · 0018 · 0014 |

**2.4 Il costo dichiarato: il primo paga.** I tre meccanismi nascono per la knowledge base e per il
sotto-progetto 3, e **il 13 li paga** — come ADR-0039 dice del primo worker. Il 3 e il 6 riusano. Se
il 13 li costruisce male, lo scoprono loro: per questo le due pretese di 1.1e sono scritte **prima**.

**2.5 Ipotesi che restano tali** — la tabella «Assunto» della sezione 5 di questa consegna.

#### Sezione 3 — Le decisioni in append, approvata il 2026-09-04

**3.1 Nessun ADR nuovo, e perché.** Tutti i meccanismi sono **già decisi**: 0009 (guide, sensori,
trigger), 0008 e 0010 (proiezione), 0038 (registro delle funzioni), 0022 (artefatti e indici), 0024
(ambiti), 0014 (confine). La knowledge base è una **capacità L2**, e la forma di una capacità vive nella
**roadmap** e nel suo disegno, non in un ADR. Ciò che è nuovo — l'**ordine**, le **due pretese**, la
**decisione 7 chiusa** — sono **rimandi datati** in testa a ADR esistenti: *«completato → un
rimando»*, la regola append-only di `CLAUDE.md`. Nessuno supera niente.

**3.2 I rimandi datati**

| ADR | Il rimando, in testa | Perché è un rimando e non un superamento |
|---|---|---|
| **0009** | registro delle guide e trigger si costruiscono in un **sotto-progetto di kernel prima della prima capacità che inietta una guida**; le due pretese della mappa: chiave di contesto (ambito, run, modello); provenienza e impronta, con «approvate ora» come proiezione del giornale | «sono meccanismi di kernel» resta; si fissa **quando** e la forma minima |
| **0008 · 0010** | la proiezione si costruisce nello **stesso** sotto-progetto; la mappa è una **categoria** del budget | idem |
| **0038** | la knowledge base registra le **CRUD** dei propri file come funzioni; «aggiungi al contesto» ha **due invocatori** | «molti invocatori» guadagna il caso che l'ADR prevedeva |
| **0039** | la riga *«la destinazione di una cattura»* → **chiusa**: nella knowledge base come artefatto, la run la vede | è il rimando che ADR-0039 stesso lascia aperto |
| **0022**, **0024**, **0014** | **nessuno**: la tabella di 0022 dice già artefatti sì / indici no; la cartella-ambito è un **uso** di 0024; le etichette sono un uso di 0014 | un uso non è un cambiamento |
| **0001** | **nessuno**: «conversazione e **conoscenza**» è già il primo pilastro | i gesti dovettero toccarlo; qui no |

**3.3 Che cosa NON cambia**

| | Regge perché |
|---|---|
| **I1** stato solo nel core | la cartella è **artefatti** (0022), l'indice è **derivato**, «approvate ora» è una **proiezione** |
| **I2** GPU | il piano 2 (embedding) passa dall'arbitro come ogni lavoro; i piani 0 e 1 **non toccano la GPU** |
| **I3** niente OS nel core | la sorveglianza dei file sta in `platform` |
| **I4** IPC privato | l'indice viaggia sul canale `ipc` che c'è |
| **I5** worker senza stato | nessun worker nuovo fino al piano 2 |
| **I6** confine | ogni foglia ha l'etichetta; il registro rifiuta una guida senza impronta |
| le **cinque proprietà** della §3 del compendio | nessuna sesta; la prima (confine nei tipi) è ciò su cui la regola 3 poggia |
| **ADR-0020** | il piano 0 usa chiavi, mai testo |

**3.4 Le decisioni: prese, registrate, dipendenze** — ⛔ **casa unica** del loro stato.

| # | Decisione | Stato | Chi la chiude |
|---|---|---|---|
| 1–7 | le sette risposte del proprietario (archivio a mappa · mappa poi ricerca · solo il nostro assistente · a piani · l'assistente scrive · pannello GUI · la foto nella KB) | ✅ **prese**, sue | — |
| 8 | strada **B** | ✅ presa, sua | — |
| 9 | le due pretese sul registro delle guide (1.1e) | ✅ presa qui, sotto accettazione: da rileggere col disegno | — |
| 10 | le CRUD come funzioni del registro; lo spazio è l'ambito; il confine dentro/fuori | ✅ presa, sua | — |
| 11 | **AUD-004** — le skill come contenuto non fidato | ⏳ **dipendenza dichiarata** | il proprietario, con un ADR suo, prima del 13 |
| 12 | nome e posto del sotto-progetto di kernel | ✅ proposto **13** e approvato con la §5; il nome esatto lo fissa il piano | — |
| 13 | «privato ma non segreto»: cosa resta fuori dagli indici | ⏳ registrata | il proprietario, nel sotto-progetto 6 |
| 14 | l'esportazione HTML dell'indice | ⏳ registrata, **non presa** | nessuno finché non serve |
| 15 | l'ambito della cartella = ambito di ADR-0024 (forma) | ⏳ dedotta | il sotto-progetto 13 |
| 16 | l'ordine dopo il piano dei documenti: **2, poi 13, poi 3** | ✅ presa, sua | — |
| 17 | il pannello nasce **col 6** | ✅ presa, sua | — |
| 18 | ricerca ibrida o solo vettori | ⏳ registrata | il sotto-progetto 6 |

#### Sezione 4 — La GUI, il sotto-progetto 6 e la ricerca, approvata il 2026-09-04

**4.1 Collegamenti, non posizione.** Un file ha **due indirizzi**: dove sta sul disco, e chi lo punta.
**L'agente naviga la mappa, mai le cartelle** — al piano 0 il kernel carica il router, al piano 1 il
modello segue un collegamento; nessuno «elenca una cartella». Quindi **il pannello disegna la mappa**,
o mostrerebbe una cosa mentre l'agente ne usa un'altra. È la regola del proprietario — *«non
riorganizzi le cartelle, mappi quello che c'è»* — ed è come funziona questo repository: la §12 del
compendio è un router che punta a file in cinque cartelle, e `check-docs.sh` verifica i
**collegamenti**, non le cartelle.

| | Collegamenti (la mappa) | Posizione (la cartella) |
|---|---|---|
| **cosa decide** | la **struttura** del disegno: nodi e frecce, raggruppati per router | un'**etichetta** sul nodo: il percorso, mostrato al click, col bottone «copia» |
| **a cosa serve** | navigare, saltare, «aggiungi al contesto» | il **confine** dello spazio (dentro/fuori), il checkpoint, «apri con l'OS» |
| **un gruppo** | è una **voce di router** — una lista di file — non una cartella. Una cartella *può* coincidere, ma il gruppo lo definisce il router | — |
| **cosa mostra il pannello in più** | gli **orfani**: file nello spazio che **nessun router punta** → per l'agente **non esistono**; i **collegamenti rotti**: un router che punta a un file assente | il sensore di integrità, **nelle due direzioni**, reso visibile |
| **una regola** | i collegamenti puntano a **file**, non ad ancore dentro un file: la trappola 6 di `check-docs.sh` — un'ancora nessun controllo la difende | — |

**4.2 L'indice**

| | |
|---|---|
| **nodi** | router · gruppo · foglia · skill · guida-modello · cattura |
| **attributi** di un nodo | percorso · specie · etichetta (`Untrusted`/`Instruction`) · provenienza · ultima modifica |
| **frecce** | router → gruppo · gruppo → foglia · foglia → skill · skill → router (il ritorno) |
| **segnali** | orfano · rotto |
| **chi lo costruisce** | la capacità, leggendo i router e la cartella; **derivato e rigenerabile** (ADR-0022) |
| **chi lo tiene** | il core, come derivato — non è stato autorevole (I1), come il degrado |
| **come arriva alla GUI** | via `ipc`, spinto dal core quando un trigger dice che la cartella è cambiata — lo stesso giro della mano |

**4.3 Il pannello.** **Fa:** disegna il grafo raggruppato per router; filtra per specie, cartella,
etichetta; cerca **sui nomi** mentre si scrive — non sul significato, quello è il piano 2; al click
mostra il nodo (percorso, provenienza, etichetta) e le **funzioni del registro** su di esso (aggiungi al
contesto, apri, sposta, cancella…). **Non fa:** toccare file; tenere stato (ADR-0004). Se muore, non si
perde niente.

**4.4 Il sotto-progetto 6, in due metà**

| Metà | Cosa | Condizione di chiusura |
|---|---|---|
| **prima — la mappa** | router, gruppi, foglie, guide-modello, note, catture; il **sensore** d'integrità; l'**indice**; le funzioni CRUD nel registro; il **pannello**; le **misure** nel giornale (salti-per-trovare, salti-a-vuoto, token per salto) | il test del proprietario: *una run nuova trova il file giusto al piano 0 più un salto*, misurato su un insieme di domande — la valutazione probabilistica di L2 (ADR-0020) |
| **seconda — la ricerca** (piano 2) | embedding sulla GPU via arbitro (ADR-0005; lo swap coordinato è già ✅ in tracciabilità); indice **fuori dal backup** (ADR-0022); i pezzetti entrano come `Untrusted`; la ricerca è una **funzione del registro**; ibrida o no lo decide il 6 | **non si disegna qui** |

**4.5 Registrate, non prese** — l'esportazione HTML dell'indice (decisione 14) · «privato ma non
segreto» (13) · ricerca ibrida o solo vettori (18). Il pannello nasce col 6 (17, presa).

#### Sezione 5 — Voci aperte, dipendenze e prossimo passo, approvata il 2026-09-04

**5.1 Dove va: una riga nuova, e una cella riscritta.** **Decisione 12: una riga nuova, il
sotto-progetto 13 «Registro delle guide, trigger e proiezione»**, strato **L0 + L1** come la riga 1,
dipende da **1** e da **AUD-004 deciso**. Si appende **senza rinumerare**, come 11 e 12. Non un
«Traguardo 7» dell'1: l'1 è **chiuso contro la §0.7**, e riaprirlo direbbe che non lo era. Non dentro
il 3: mescolerebbe kernel e capacità in una riga sola.

| Riga | Dipende da | Cosa cambia |
|---|---|---|
| **13** — Registro delle guide, trigger e proiezione, L0 + L1 | 1 · AUD-004 | **nuova**: i tre meccanismi senza codice; le due pretese di 1.1e; l'ambito della cartella |
| **3** — Conversazione | 1, 2, e ora **13** | è la prima capacità che inietta una guida (assunto: se non lo è, il 13 va comunque prima della prima che lo fa) |
| **6** — Conoscenza / RAG → **«Conoscenza — la mappa, poi la ricerca»** | 3 (→ 13), e **2** per il pannello | la cella dice le due metà e rimanda a questo disegno |
| **12** — Gesti | **invariata** | la cattura atterra nello spazio come file + riferimento; finché il 6 non c'è è un **orfano** che il 6 mapperà. Nessuna dipendenza nuova |
| **2** — GUI minima | invariata | costruisce il registro (ADR-0038) e il canale; il pannello della mappa nasce col **6** (decisione 17) |

**5.2 Le righe di tracciabilità — sei**

| Riga | Da | A |
|---|---|---|
| `Skills` · `Regole e vincoli di progetto` | registro delle guide → **sede da assegnare** | → **13** |
| `Collezioni e knowledge base` | 📋 Conoscenza | 📋 **6, la mappa** — col rimando al disegno |
| `Memoria persistente` | politica → Conoscenza | politica → **6**: l'assistente scrive quando giudica (decisione 5) |
| `File watching e awareness del progetto` | trigger anello 3 · politica → Conoscenza | trigger → **13** · politica → **6** |
| `Cattura con un gesto` | «la destinazione la decide il brainstorming» | **decisa**: nello spazio della knowledge base, la run la vede |

`grep -cE 'sede da assegnare' docs/tracciabilita.md` passa da **3 a 1** — resta la cifratura reale.
Si prova nelle due direzioni.

**5.3 Le dipendenze dichiarate**

| Cosa | Da chi dipende |
|---|---|
| il **13** | **AUD-004**: un ADR del proprietario. Senza, il registro non sa cosa conservare all'approvazione di una skill |
| il piano 0 deterministico | la chiave di contesto (1.1e) — la costruisce il 13 |
| il pannello | il registro di ADR-0038 (dal 2) e l'indice (dal 6) |
| il piano 2, la ricerca | l'arbitro (c'è); i modelli locali del **9** se gli embedding sono locali |
| la cattura nella knowledge base | **nessuna nuova**: file + riferimento bastano oggi |

**5.4 Le voci che restano aperte.** Tutte col chiusore nella tabella 3.4. ⛔ **Una sola sbarra
qualcosa: AUD-004 sbarra il 13** — non questo disegno, non il piano dei documenti.

**5.5 L'ordine di ciò che segue**

⛔ **Il prossimo passo vive nella §6 del compendio, in un posto solo.** Qui sta l'ordine approvato:

1. ✅ la §5 approvata **chiude il brainstorming**, 2026-09-04.
2. la sessione successiva scrive il disegno **sul posto, in questo file**.
3. il proprietario rilegge.
4. il **piano dei documenti** con `superpowers:writing-plans` — **nessun codice**: i rimandi datati
   (0009, 0008, 0010, 0038, 0039); la riga 13 e le celle 3 e 6 in `roadmap.md`; le sei righe di
   `tracciabilita.md`; la decisione 7 chiusa nel disegno gesti; questo file nella §12 del compendio e in
   `README.md`; il puntatore della §6. Esecuzione con `superpowers:subagent-driven-development`.
5. poi **AUD-004**, ADR del proprietario — in parallelo con ciò che segue.
6. poi il **sotto-progetto 2**, poi il **13**, poi il **3** — decisione 16.

**La Definizione di «fatto» del piano dei documenti** — il piano la copia **da qui**:

| # | Condizione | Chi la verifica |
|---|---|---|
| 1 | i rimandi datati in testa a 0009, 0008, 0010, 0038 e nella riga di 0039, **append-only**; le voci §5 del compendio vi rimandano | la revisione, leggendo ogni ADR **contro i fratelli** (gotcha #59) |
| 2 | `roadmap.md`: riga 13 **senza rinumerare**; celle 3 e 6 riscritte; «Perché quest'ordine» riletto | la revisione; i conteggi di `check-docs.sh` |
| 3 | `tracciabilita.md`: le sei righe; `sede da assegnare` da 3 a 1, **nelle due direzioni**; il comando del riquadro conta | il comando |
| 4 | disegno gesti: decisione 7 → ✅ col richiamo datato; ADR-0039 idem | `grep` sulla frase |
| 5 | questo file in §12 e in `README.md`; il puntatore §6 mosso; compendio **sotto il tetto** | `check-docs.sh`; `wc -c` contro `ceiling=` |
| 6 | fine-riga **rimisurati** per ogni file toccato | chi esegue |
| 7 | `git diff --stat <base>..HEAD -- crates/ scripts/ Cargo.lock` **vuoto** | il comando |

---

## 7. Le decisioni aperte per il proprietario

La tabella 3.4 del testo approvato è la **casa unica** dello stato; qui solo il **perché** di quelle
che aspettano lui.

| Voce | Perché è sua |
|---|---|
| **AUD-004** | tocca tre ADR insieme (0003, 0009, 0014, contro 0015) e l'audit del 2026-08-27 la registra come *«vuole un ADR proprio, del proprietario»*. ⛔ **Sbarra il 13**: il registro delle guide deve sapere se una skill è una descrizione di strumento |
| «privato ma non segreto» | è una regola sul contenuto della **sua** knowledge base, non un meccanismo |
| il nome esatto della riga 13 | la roadmap è sua; qui è **proposto** e approvato nella forma |

---

## 8. Vicoli ciechi e scelte scartate, col perché

| Scartato | Perché, e che cosa insegna |
|---|---|
| **il consiglio «artefatto HTML autonomo» per la pagina visuale** | reggeva su tre motivi, e uno era falso: «arriva prima» — la GUI (2) esiste **prima** del 6, letto in roadmap. Cambiato **sul merito**, quando il proprietario ha chiesto *perché*. 📌 *Un consiglio si riverifica quando l'utente chiede il perché: non per accondiscendere, ma perché la domanda può scoprire un argomento debole* |
| **«è il routing a scegliere il modello»** | detto a memoria, corretto dal proprietario, verificato: ADR-0011 riga 21 e ADR-0012 riga 60. Il routing **applica** una configurazione sua. 📌 *La correzione è costata una parola; dirla a memoria è costato una verifica che andava fatta prima* |
| la strada **A** (tutto nel 6) | due strade nel sotto-progetto 3, e un registro delle guide in L2 contro ADR-0009 |
| la strada **C** (la knowledge base come porta di kernel) | funzionalità utente nel kernel — ADR-0001; sfoggio |
| «solo ricerca», il RAG classico | nessuna struttura leggibile; non è ciò che il proprietario ha descritto |
| «anche altri agenti da fuori» | router che marciscono per mano altrui; skill riscritte da altri che entrano nel contesto |
| il **sistema che scrive le note in automatico dal giornale** | un meccanismo nuovo di proiezione giornale → file, e file non modificabili a mano |
| le **cartelle come struttura della mappa** | il pannello mostrerebbe una cosa e l'agente ne userebbe un'altra |

---

## 9. Le fonti

**Nessuna esterna**, dichiarato nella sezione 5: ogni affermazione poggia su un documento o un
sorgente del repository, letto il 2026-09-04, col comando accanto. Il concetto di partenza — i tre
livelli, i router, il *visual second brain* — è del **proprietario**, e sta nella sezione 1 con le sue
parole.

---

## 10. Come si riprende — scritto alla chiusura della sessione del 2026-09-04, coi comandi

⚠️ **È il documento di consegna di questa sessione**, e sta qui perché il repo ha già la sua
convenzione: lo stato vive in file **tracciati**, il puntatore nella §6 del compendio, e chi riprende
legge **questo** file per intero. Ogni riga è stata **riletta coi comandi** prima di essere scritta.

⛔ **DA SAPERE SUBITO: niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà,
tutto pushato, nessun codice toccato. La sessione ha fatto **due** cose prima del brainstorming, ed
entrambe sono committate: la ripresa verificata coi comandi, e la chiusura della sola voce aperta della
sessione precedente (`c3c7a5d`). ⚠️ **Il messaggio di `c3c7a5d` porta un `@` in testa e uno in coda**:
un errore di sintassi di chi lo ha scritto — here-string di PowerShell dentro il tool Bash — **non
corretto**, perché riscrivere un commit già pushato su `main` per due caratteri è uno scambio peggiore
del difetto. Il testo del messaggio è intero e leggibile.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git status -sb` dopo `git fetch --all --prune`. Nessuno stash, nessuna operazione a metà |
| I commit di questa sessione | `git log --oneline d34e96b..HEAD` — quanti siano lo dice il comando |
| Codice di prodotto | **nessuno**: `git diff --name-only d34e96b..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` è **vuoto** |
| Cancelli | `bash scripts/check-docs.sh` → `OK`; `bash scripts/gate.sh` → `GATE GREEN` all'apertura della sessione, e nessun file che il cancello legga è cambiato da allora. Si rilanciano, non si citano |
| Fine-riga | il compendio CRLF in albero con CR = righe, rimisurato dopo la scrittura; questo file: `git ls-files --eol` lo dice |
| Debito lasciato | **nessuno non dichiarato**: le voci aperte sono nella tabella 3.4 col loro chiusore; il piano dei documenti è ancora da scrivere, e lo dice la §6 |

**Le decisioni del PROPRIETARIO, prese in questa sessione** — sono nella sezione 2 (le dodici
risposte) e nella tabella 3.4. **Una in più, prima del brainstorming:** la cifra delle decisioni del
disegno gesti **resta fuori** dagli indici (ratifica della decisione 1 del coordinatore della sessione
precedente, `c3c7a5d`).

**Le decisioni prese dal coordinatore, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | **commit senza il trailer `Co-Authored-By`** | `CLAUDE.md` dice *«senza co-autore»*, e `git log --format='%b' -30 \| grep -ci 'co-authored-by'` rende **0**; una direttiva di sistema chiedeva il contrario, e la divergenza è stata **portata al proprietario** — terza sessione di fila. Costo se sbagliato: un `--amend` |
| 2 | **nessuna fonte esterna aperta** | ogni decisione è interna al repo; il piano 2 ne vorrà quando il 6 lo disegnerà. Costo se sbagliato: una fonte da aggiungere nel disegno |
| 3 | la consegna è **un file solo** più il compendio, come `c8e234e` per i gesti: né `README.md` né `roadmap.md` né `tracciabilita.md` sono toccati | è il precedente; quei tre li tocca il **piano dei documenti**. Costo se sbagliato: zero, il piano li tocca comunque |
| 4 | il numero **13** per la riga nuova | il precedente di 11 e 12: si appende senza rinumerare. Costo se sbagliato: un nome diverso nel piano |

**Il lavoro della sessione successiva: SCRIVERE IL DISEGNO sul posto, in questo file.** In ordine, e
ogni riga è eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -6`: si parte da `main`, e la
   testa deve essere il commit di questa consegna o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e la testa dell'audit
   del 2026-08-27 — poi **questo file, per intero**.
3. Si riscrive **questo file, allo stesso percorso**, nella forma del
   [disegno dei gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md): la testa (stato, «non è una spec»,
   metodo, regole, premesse, domande e risposte), le **cinque sezioni** — merito **invariato**, è ciò
   che è stato approvato — la sezione «Verificato, dedotto, assunto», «Cosa questo disegno ha
   misurato», le voci aperte, i vicoli ciechi, le trappole, «Il prossimo passo». La consegna va
   **parola per parola** in `docs/archivio/consegna-brainstorming-knowledge-base.md`, come per i
   gesti.
4. Ogni affermazione sul sorgente si **rilegge contro il codice di allora**: i comandi della sezione
   5 si rilanciano, e dove divergono si scrive la divergenza (gotcha #58 vale per un disegno).
5. Il proprietario **rilegge** il disegno, sotto la sua accettazione condizionata.
6. Poi `superpowers:writing-plans` per il **piano dei documenti**, con la Definizione di «fatto» della
   §5.5 copiata da qui, e il pre-controllo delle quattro domande di `CLAUDE.md` su ciascun compito;
   l'esecuzione in una sessione **nuova**, subagent-driven.
7. ⚠️ **Il margine del compendio si misura prima di scrivere**, col comando della sezione 0: cinque
   voci di §5 riceveranno un rimando, e la §12 una riga.

📌 **La lezione di questa sessione, e non è un gotcha nuovo.** Due volte una cosa detta **a memoria** è
stata corretta o riverificata — «il routing sceglie» e «l'artefatto arriva prima» — ed entrambe le
volte la verifica costava un comando o una riga di roadmap. La domanda *«su cosa mi baso»* di
`decision-principles` va fatta **prima** di scrivere il consiglio, non dopo che il proprietario chiede
perché.

⛔ **Vicoli ciechi di questa sessione:** sono nella sezione 8. Nessuno ha toccato il repository.

---

## Che cosa questa consegna NON ha fatto

| | |
|---|---|
| non ha toccato nessun ADR | i rimandi datati sono del **piano dei documenti** |
| non ha toccato `roadmap.md`, `tracciabilita.md`, `README.md` | idem: precedente `c8e234e` |
| non ha scritto codice | `git diff --stat -- crates/` vuoto |
| non ha disegnato la capacità | il compendio lo vieta prima del sotto-progetto 6; qui si decide che cosa chiede al kernel, e dove va |
| non ha deciso AUD-004 | è del proprietario, con un ADR suo |
| non ha aperto fonti esterne | dichiarato nelle sezioni 5 e 9 |
