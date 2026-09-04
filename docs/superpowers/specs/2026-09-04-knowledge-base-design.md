# Knowledge base: il disegno

✅ **QUESTO DISEGNO È COMPLETO DAL 2026-09-04.** Le cinque sezioni sono **approvate** dal
proprietario, una per volta, in chat il 2026-09-04, sotto l'**accettazione condizionata** la cui
regola sta qui sotto — e la **§5** fissa dove va la capacità, le dipendenze e l'ordine di ciò che
segue. Chi riprende ha un disegno intero da tradurre in un **piano dei documenti** — **dopo** che il
proprietario lo ha riletto in questa forma. ✅ **Riletto il 2026-09-04**, in chat, sotto accettazione
condizionata: le sei voci della sezione *«Le voci che questo disegno apre per il proprietario»* sono
state poste **una per volta**, in forma A/B, col consiglio scritto, e il proprietario ha scelto il
consiglio **sei volte su sei**. ⚠️ Questa riga diceva *«⏳ La rilettura del proprietario è da fare»*,
ed è riscritta invece di ricevere una riga sotto, sul precedente del disegno dei gesti.

⚠️ **RICHIAMO DEL 2026-09-04, lo stesso giorno:** questo file è nato come **consegna** del
brainstorming — le cinque sezioni approvate, le dodici risposte del proprietario, lo stato del
repository alla chiusura — e il proprietario ha scelto che il disegno lo scrivesse la sessione
**successiva**. Riscritto **sul posto**, allo stesso percorso, perché il puntatore della §6 del
compendio non cambiasse casa, come fu per il
[disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md). Il merito delle cinque sezioni è
quello approvato e **non è stato toccato**; la consegna sta **parola per parola** in
[`archivio/consegna-brainstorming-knowledge-base.md`](../../archivio/consegna-brainstorming-knowledge-base.md);
ciò che la riscrittura ha **misurato** in più sta nella sezione *«Cosa questo disegno ha misurato»*,
e **un'affermazione della consegna vi risulta più debole di come era scritta** — il conteggio delle
funzionalità della sezione 3 di `tracciabilita.md`, §6.2.

⚠️ **Non è una spec.** Come i disegni dei Traguardi 4, 5 e 6 e dei gesti, fissa il **perimetro**, le
**forme** che gli ADR descrivono a parole, e per ogni artefatto **il controllo che lo esercita**. Gli
ADR restano l'autorità — 0009 per le guide, 0008 e 0010 per la proiezione, 0038 per il registro delle
funzioni, 0022 e 0024 per i file — e ciò che questo disegno vi aggiunge lo **dichiara** come rimando
in append, nella §3. ⛔ **E non disegna la capacità:** il compendio (§8) vieta di progettare una
capacità L2 prima del suo sotto-progetto. Qui si decide che cosa la knowledge base **chiede al
kernel**, e dove va.

📌 **Metodo.** Ogni affermazione porta la sua specie — **verificata** (letta nel sorgente o in un
documento del repository, con la data), **dedotta**, o **assunta** — e le tre sono separate nella §6.
Le affermazioni sul sorgente sono state lette il 2026-09-04 contro `07ab6dc`, e il codice non è
cambiato dalla consegna: `git diff --stat c3c7a5d..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml
rust-toolchain.toml docs/adr/` non rende nulla. I comandi stanno accanto alle affermazioni e **si
rilanciano**, non si citano: le cifre invecchiano al primo commit che tocca ciò che misurano, i
comandi no.

**Le regole di questo lavoro, decise dal proprietario**

| Regola | Da dove viene |
|---|---|
| La strada è quella del repo: **brainstorming → disegno scritto → piano → esecuzione** | scelta 5 del 2026-09-02, in testa al [disegno della chiusura](2026-09-02-sottoprogetto-1-chiusura-design.md) |
| ⛔ **Ogni decisione si controlla contro i cinque criteri di `anthropic-skills:decision-principles`**, e i principi governano la decisione senza occuparla. Il proprietario lo ha ripetuto in chat il 2026-09-04: *«lavoriamo e progettiamo sempre secondo i principi di questa per favore»*. È un'**accettazione condizionata**, che vale finché regge: se una scelta li viola, ci si **ferma e lo si dice**, non si tratta come delega in bianco | chat del 2026-09-04 |
| Brainstorming in **una** sessione, disegno nella **successiva**, con la consegna in un file **tracciato** | scelta del proprietario del 2026-09-02 |
| Questo brainstorming è **distinto** da quello dei gesti, e ha **una** domanda da chiudere: *se la knowledge base pretenda un meccanismo di kernel «che non si aggiunge dopo», o sia tutta L2 nel sotto-progetto 6* | voce 2 della §7.8 del disegno della chiusura; la §3 del compendio per «non si aggiunge dopo» |
| **Non si disegna la capacità**: il compendio (§8) vieta di progettare una capacità L2 prima del suo sotto-progetto | compendio §8, riga *«progettare una capacità L2»* |
| Codice in inglese, documenti in italiano; nessun numero senza comando; nessuna fonte senza data | [`CLAUDE.md`](../../../CLAUDE.md) |
| **Prima a parole, poi lo schema**: il proprietario non è operativo in Rust, e ogni scelta gli è stata spiegata a parole prima della tabella | `CLAUDE.md`, *«Ma prima a parole»* |

**Le premesse dette dal proprietario**, che il disegno onora, con le sue parole: *«un archivio unico,
organizzato bene con dei file di routing verso dei gruppi o sottogruppi specifici, ogni file è contenuto
qui dentro a partire dalle skill»*; *«l'agente possa navigarci tranquillamente ed evitando di saturare
contesto, bruciare token»*; *«un puntatore stale è peggio di nessun puntatore»*; *«non riorganizzi le
cartelle, mappi quello che c'è già»*; *«vorrei che la ricerca fosse efficiente, consistente e che
funzioni BENE, non a volte sì a volte no»*; e il test di completamento: *«una sessione nuova di zecca
trova il file giusto attraverso i router al primo salto»*.

**Le dodici domande d'apertura, e le risposte del proprietario — 2026-09-04**

⛔ **Sono sue e non si riaprono senza di lui.** Ogni riga porta anche ciò che è stato **scartato**,
perché chi riprende sappia che cosa non rifare. Le domande com'erano poste stanno nell'archivio.

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

**Le decisioni, numerate come nella consegna** — vivono nella tabella della **§3.4**, che è la loro
**casa unica**, da 1 a 18; i rimandi *«decisione N»* di questo disegno puntano lì. ⛔ **Non sono
ricopiate qui**, deliberatamente: una seconda tabella sarebbe il gotcha **#68** dentro il file che
lo cita. Quante siano lo dice il comando, delimitato per testo e non per numero di riga:
`awk '/^### 3\.4/{s=1} s&&/^## 4\./{s=0} s&&/^[|] [0-9]+(–[0-9]+)? [|]/{n++} END{print n}' docs/superpowers/specs/2026-09-04-knowledge-base-design.md`
— ⚠️ conta le **righe** della tabella, e la prima riga ne raccoglie sette (le decisioni 1–7 del
proprietario), quindi il comando rende **dodici** righe per **diciotto** decisioni; chi aggiunge una
riga sa che cosa muove.

**L'approccio scelto: la strada B, e le due scartate**

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

## 1. Il perimetro — ✅ approvata il 2026-09-04

### 1.1 Che cosa decide ora

| | Decisione |
|---|---|
| **a** | La knowledge base è una **capacità L2 del sotto-progetto 6**: archivio unico di file, a **mappa** (router → gruppi → foglie), skill comprese, **una guida per modello** come foglia, note scritte dall'assistente quando giudica che vale, le catture atterrano qui |
| **b** | La tocca **solo il nostro assistente**, attraverso il kernel. Il modello lo sceglie il proprietario; il routing lo applica e lo annota (ADR-0011) |
| **c** | Navigazione **a piani**: **0** il kernel carica sempre il router centrale e quello dell'ambito, senza giudizio · **1** il modello salta a una foglia, e ogni salto è un passo giornalato con il suo costo · **2** la ricerca per somiglianza, **dopo**, dentro il 6 |
| **d** | ⛔ **Il verdetto sulla domanda della voce 2:** **nessuna sesta proprietà** «che non si aggiunge dopo». C'è un **vincolo d'ordine**: registro delle guide, trigger e proiezione — decisi in ADR-0009, 0008, 0010, **zero codice** — si costruiscono in un **sotto-progetto piccolo di kernel PRIMA del 3**, o la prima capacità si inietta le skill a modo suo e nascono due strade |
| **e** | Il registro delle guide deve rispettare **due pretese** della mappa: la **chiave di contesto** esprime *ambito, run, modello*; ciò che conserva porta **provenienza e impronta** all'approvazione, e «approvato ora» è una **proiezione del giornale** — la forma di `permission.rs` |
| **f** | La pagina visuale è un **pannello della GUI** che disegna un **indice rigenerabile tenuto dal core**, via IPC, aggiornato come la mano di ADR-0039. L'HTML autonomo, se mai, è un'esportazione dello stesso indice |
| **g** | Il pannello agisce **solo attraverso il registro delle funzioni** (ADR-0038). La capacità registra le **CRUD** dei propri file e gruppi — creare, leggere, aggiungere al contesto, aggiornare, spostare (dentro lo spazio, dentro da fuori, fuori), cancellare — come funzioni del registro: ognuna è un effetto con classe dichiarata, giornalato, checkpointato dentro l'ambito. «Aggiungi al contesto» è **una** funzione con **due invocatori**: il click e il modello |

### 1.2 Che cosa rimanda, e a chi

| Voce | A chi |
|---|---|
| **AUD-004**: le skill come contenuto non fidato — se le difese di ADR-0015 (testo integrale, impronta, sospensione se cambia) si estendono alle skill | il **proprietario**, con un ADR suo, **prima** del sotto-progetto 13: il registro deve saperlo per nascere giusto |
| nome e posto del sotto-progetto di kernel nella roadmap | il proprietario, nella §5 di questo disegno — proposto **13** |
| il disegno **del** sotto-progetto 13 (come si costruiscono i tre meccanismi) | il **suo** brainstorming, come ogni traguardo. Qui si fissano solo le pretese di 1.1e |
| il piano 2 — ricerca, embedding, indice, GPU | sotto-progetto 6, seconda metà. Qui **non si disegna** |
| sintassi dei router, forma dell'indice, aspetto del pannello | sotto-progetti 6 e 2: sono la capacità, e il compendio vieta di disegnarla prima |
| se l'«ambito» del piano 0 sia l'**ambito di lavoro** di ADR-0024 | §2 di questo disegno, a parole; la forma la dà il 13 |

### 1.3 Che cosa esclude

- Altri strumenti che leggono o scrivono la cartella (decisione **b**).
- Ingest di documenti (PDF, OCR, web): righe L2 già in `tracciabilita.md`, sotto-progetto 6.
- Un record nuovo del giornale, una porta nuova, la cifratura della knowledge base, regole di backup
  nuove: **niente di tutto questo**, ed è misurabile.
- Riaprire ADR-0009 (le guide sono meccanismo di kernel), ADR-0022 (i file sono artefatti
  dell'utente), ADR-0020 (nessun modello nel kernel).

### 1.4 I «buchi logici», sciolti coi documenti del repo

Il proprietario ha detto che la sua idea era *«grezza»*. Ciascun buco ha una casa già scritta. La
tabella è quella della consegna, portata qui come la sezione approvata prescriveva.

| Il buco | Sciolto da |
|---|---|
| «chi aggiorna il router quando un file si sposta?» | l'assistente, come scrittura giornalata (ADR-0007) dentro un ambito con checkpoint (ADR-0024); e un **sensore** (ADR-0009 — `Sensor::observe(&Untrusted)` esiste in `crates/kernel/src/sensor.rs`) che verifica i router: un puntatore rotto è un **verdetto negativo** che rientra nell'anello |
| «e se il router marcisce lo stesso?» | l'anello di miglioramento (ADR-0009): il kernel vede la ricorrenza e **propone** la voce; il proprietario approva. La regola «il router si aggiorna nello stesso turno» diventa un meccanismo, non una speranza. ⚠️ **È la lezione di questo stesso repository:** la regola «un puntatore si toglie, non si ricorregge» ha ceduto tre volte finché era solo scritta; il freno è nel cancello — `check-docs.sh` — e non nella buona volontà |
| «come non saturo il contesto?» | la proiezione ha un **budget per modello** (ADR-0010), misurato per categoria nel giornale; la mappa è una categoria; le foglie entrano come **riferimenti** e si rileggono su richiesta (ADR-0008) |
| «e se modifico un file a mano?» | trigger su cambiamento di file (ADR-0009) → contenuto da fuori (I6, ADR-0014) finché il proprietario non approva. Una skill cambiata → AUD-004 |
| «come so se funziona BENE?» | ogni salto è un passo con costo (ADR-0011): salti-per-trovare e salti-a-vuoto sono **numeri nel giornale**; il test «al primo salto» si misura. La valutazione probabilistica sta in L2 (ADR-0020) |
| «la foto in che contesto?» | decisione 7: nella knowledge base come artefatto, la run la vede come riferimento (ADR-0018, 0022, 0008) |
| «cosa è privato e resta fuori dagli indici?» — dal concetto del proprietario | ⚠️ **NON sciolto qui.** I segreti stanno nel gestore dei segreti (ADR-0023), mai in file. «Privato ma non segreto» è una regola della capacità: **voce aperta** per il sotto-progetto 6, decisione 13 della §3.4 |
| «come funziona se uso modelli diversi?» | il modello **non tocca mai il disco**: il kernel compone il contesto (ADR-0008) e lo consegna. Ciò che cambia col modello è la **guida** (ADR-0009, una foglia per modello) e il **budget** (ADR-0010); la mappa è una. Se il proprietario tiene sempre un modello, il meccanismo esiste e non scatta mai: costo zero |

### 1.5 Il prodotto di questo disegno, e il controllo che esercita ciascun artefatto

| Artefatto | Controllo |
|---|---|
| questo disegno, riscritto dalla sessione successiva — ✅ **fatto il 2026-09-04, è questo file** | `check-docs.sh` (link, tetto); la sezione verificato/dedotto/assunto; ogni cifra col comando |
| rimando datato in testa ad **ADR-0009**: le due pretese e l'ordine | ADR append-only; `check-docs.sh` accoppia la §5 del compendio |
| rimando in **ADR-0039** («destinazione di una cattura») e decisione 7 del disegno gesti → **chiusa** | `grep` sulla frase; la tabella delle decisioni dei gesti |
| `roadmap.md`: riga nuova del sotto-progetto di kernel; riga 6 aggiornata (mappa poi ricerca; dipende dal nuovo) | «nessuna rinumerazione»; conteggi di `check-docs.sh` |
| `tracciabilita.md`: `Skills` e `Regole e vincoli` → sede assegnata; `Collezioni e knowledge base` → 6, con la forma | `grep -cE 'sede da assegnare'` **scende di due** — provato nelle due direzioni |
| compendio §6 (puntatore) e §12 (riga); `README.md` (riga) | tetto; link |
| **codice: nessuno** | `git diff --stat -- crates/` vuoto, a fine piano |

---

## 2. La forma nel kernel — ✅ approvata il 2026-09-04

### 2.1 In una frase

Il kernel **fornisce tre meccanismi** che mancano e **riusa quattro** che esistono. La capacità (L2,
sotto-progetto 6) li **compone**. **Nessuna porta nuova.**

### 2.2 I pezzi, e dove vive ciascuno

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

### 2.3 Quattro regole di forma

| | Regola | Da |
|---|---|---|
| 1 | il kernel **non legge mai il testo** di una foglia per decidere: il piano 0 usa solo **chiavi** (ambito, run, modello) | ADR-0020 |
| 2 | ogni scrittura nella cartella è un **effetto giornalato con classe** dentro l'ambito: niente scrive «di lato» — né il pannello, né il modello, né la capacità | ADR-0007 · 0024 |
| 3 | una foglia entra **sempre** con provenienza ed etichetta; il registro delle guide **rifiuta** una guida senza impronta | ADR-0014 · 1.1e |
| 4 | **lo spazio designato è l'ambito, e il confine conta.** Dentro (crea, aggiorna, rinomina, sposta fra gruppi, cancella): checkpointato, **reversibile**, il router segue e il sensore verifica. **Da fuori a dentro**: il file entra come `Untrusted` con provenienza *«importato da ‹percorso› al passo ‹S›»*; se è una skill, serve l'approvazione — AUD-004. **Da dentro a fuori**: il router **perde** la voce; il giornale **tiene** il riferimento **con la destinazione** — un file uscito e uno cancellato non devono essere indistinguibili, la forma di ADR-0018; ⚠️ da lì in poi il checkpoint **non lo copre più**, il limite dichiarato di ADR-0024 | ADR-0024 · 0018 · 0014 |

### 2.4 Il costo dichiarato: il primo paga

I tre meccanismi nascono per la knowledge base e per il sotto-progetto 3, e **il 13 li paga** — come
ADR-0039 dice del primo worker. Il 3 e il 6 riusano. Se il 13 li costruisce male, lo scoprono loro: per
questo le due pretese di 1.1e sono scritte **prima**.

### 2.5 Ipotesi che restano tali

La tabella «Assunto» della **§6.5**.

---

## 3. Le decisioni in append — ✅ approvata il 2026-09-04

### 3.1 Nessun ADR nuovo, e perché

Tutti i meccanismi sono **già decisi**: 0009 (guide, sensori, trigger), 0008 e 0010 (proiezione), 0038
(registro delle funzioni), 0022 (artefatti e indici), 0024 (ambiti), 0014 (confine). La knowledge base
è una **capacità L2**, e la forma di una capacità vive nella **roadmap** e nel suo disegno, non in un
ADR. Ciò che è nuovo — l'**ordine**, le **due pretese**, la **decisione 7 chiusa** — sono **rimandi
datati** in testa a ADR esistenti: *«completato → un rimando»*, la regola append-only di `CLAUDE.md`.
Nessuno supera niente.

### 3.2 I rimandi datati

| ADR | Il rimando, in testa | Perché è un rimando e non un superamento |
|---|---|---|
| **0009** | registro delle guide e trigger si costruiscono in un **sotto-progetto di kernel prima della prima capacità che inietta una guida**; le due pretese della mappa: chiave di contesto (ambito, run, modello); provenienza e impronta, con «approvate ora» come proiezione del giornale | «sono meccanismi di kernel» resta; si fissa **quando** e la forma minima |
| **0008 · 0010** | la proiezione si costruisce nello **stesso** sotto-progetto; la mappa è una **categoria** del budget | idem |
| **0038** | la knowledge base registra le **CRUD** dei propri file come funzioni; «aggiungi al contesto» ha **due invocatori** | «molti invocatori» guadagna il caso che l'ADR prevedeva |
| **0039** | la riga *«la destinazione di una cattura»* → **chiusa**: nella knowledge base come artefatto, la run la vede | è il rimando che ADR-0039 stesso lascia aperto |
| **0022**, **0024**, **0014** | **nessuno**: la tabella di 0022 dice già artefatti sì / indici no; la cartella-ambito è un **uso** di 0024; le etichette sono un uso di 0014 | un uso non è un cambiamento |
| **0001** | **nessuno**: «conversazione e **conoscenza**» è già il primo pilastro | i gesti dovettero toccarlo; qui no |

### 3.3 Che cosa NON cambia

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

### 3.4 Le decisioni: prese, registrate, dipendenze

⛔ **Casa unica** del loro stato.

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

---

## 4. La GUI, il sotto-progetto 6 e la ricerca — ✅ approvata il 2026-09-04

### 4.1 Collegamenti, non posizione

Un file ha **due indirizzi**: dove sta sul disco, e chi lo punta. **L'agente naviga la mappa, mai le
cartelle** — al piano 0 il kernel carica il router, al piano 1 il modello segue un collegamento;
nessuno «elenca una cartella». Quindi **il pannello disegna la mappa**, o mostrerebbe una cosa mentre
l'agente ne usa un'altra. È la regola del proprietario — *«non riorganizzi le cartelle, mappi quello
che c'è»* — ed è come funziona questo repository: la §12 del compendio è un router che punta a file in
cinque cartelle, e `check-docs.sh` verifica i **collegamenti**, non le cartelle.

| | Collegamenti (la mappa) | Posizione (la cartella) |
|---|---|---|
| **cosa decide** | la **struttura** del disegno: nodi e frecce, raggruppati per router | un'**etichetta** sul nodo: il percorso, mostrato al click, col bottone «copia» |
| **a cosa serve** | navigare, saltare, «aggiungi al contesto» | il **confine** dello spazio (dentro/fuori), il checkpoint, «apri con l'OS» |
| **un gruppo** | è una **voce di router** — una lista di file — non una cartella. Una cartella *può* coincidere, ma il gruppo lo definisce il router | — |
| **cosa mostra il pannello in più** | gli **orfani**: file nello spazio che **nessun router punta** → per l'agente **non esistono**; i **collegamenti rotti**: un router che punta a un file assente | il sensore di integrità, **nelle due direzioni**, reso visibile |
| **una regola** | i collegamenti puntano a **file**, non ad ancore dentro un file: la trappola 6 di `check-docs.sh` — un'ancora nessun controllo la difende | — |

### 4.2 L'indice

| | |
|---|---|
| **nodi** | router · gruppo · foglia · skill · guida-modello · cattura |
| **attributi** di un nodo | percorso · specie · etichetta (`Untrusted`/`Instruction`) · provenienza · ultima modifica |
| **frecce** | router → gruppo · gruppo → foglia · foglia → skill · skill → router (il ritorno) |
| **segnali** | orfano · rotto |
| **chi lo costruisce** | la capacità, leggendo i router e la cartella; **derivato e rigenerabile** (ADR-0022) |
| **chi lo tiene** | il core, come derivato — non è stato autorevole (I1), come il degrado |
| **come arriva alla GUI** | via `ipc`, spinto dal core quando un trigger dice che la cartella è cambiata — lo stesso giro della mano |

### 4.3 Il pannello

**Fa:** disegna il grafo raggruppato per router; filtra per specie, cartella, etichetta; cerca **sui
nomi** mentre si scrive — non sul significato, quello è il piano 2; al click mostra il nodo (percorso,
provenienza, etichetta) e le **funzioni del registro** su di esso (aggiungi al contesto, apri, sposta,
cancella…). **Non fa:** toccare file; tenere stato (ADR-0004). Se muore, non si perde niente.

### 4.4 Il sotto-progetto 6, in due metà

| Metà | Cosa | Condizione di chiusura |
|---|---|---|
| **prima — la mappa** | router, gruppi, foglie, guide-modello, note, catture; il **sensore** d'integrità; l'**indice**; le funzioni CRUD nel registro; il **pannello**; le **misure** nel giornale (salti-per-trovare, salti-a-vuoto, token per salto) | il test del proprietario: *una run nuova trova il file giusto al piano 0 più un salto*, misurato su un insieme di domande — la valutazione probabilistica di L2 (ADR-0020) |
| **seconda — la ricerca** (piano 2) | embedding sulla GPU via arbitro (ADR-0005; lo swap coordinato è già ✅ in tracciabilità); indice **fuori dal backup** (ADR-0022); i pezzetti entrano come `Untrusted`; la ricerca è una **funzione del registro**; ibrida o no lo decide il 6 | **non si disegna qui** |

### 4.5 Registrate, non prese

L'esportazione HTML dell'indice (decisione 14) · «privato ma non segreto» (13) · ricerca ibrida o solo
vettori (18). Il pannello nasce col 6 (17, presa).

---

## 5. Voci aperte, dipendenze e prossimo passo — ✅ approvata il 2026-09-04

### 5.1 Dove va: una riga nuova, e una cella riscritta

**Decisione 12: una riga nuova, il sotto-progetto 13 «Registro delle guide, trigger e proiezione»**,
strato **L0 + L1** come la riga 1, dipende da **1** e da **AUD-004 deciso**. Si appende **senza
rinumerare**, come 11 e 12. Non un «Traguardo 7» dell'1: l'1 è **chiuso contro la §0.7**, e riaprirlo
direbbe che non lo era. Non dentro il 3: mescolerebbe kernel e capacità in una riga sola.

| Riga | Dipende da | Cosa cambia |
|---|---|---|
| **13** — Registro delle guide, trigger e proiezione, L0 + L1 | 1 · AUD-004 | **nuova**: i tre meccanismi senza codice; le due pretese di 1.1e; l'ambito della cartella |
| **3** — Conversazione | 1, 2, e ora **13** | è la prima capacità che inietta una guida (assunto: se non lo è, il 13 va comunque prima della prima che lo fa) |
| **6** — Conoscenza / RAG → **«Conoscenza — la mappa, poi la ricerca»** | 3 (→ 13), e **2** per il pannello | la cella dice le due metà e rimanda a questo disegno |
| **12** — Gesti | **invariata** | la cattura atterra nello spazio come file + riferimento; finché il 6 non c'è è un **orfano** che il 6 mapperà. Nessuna dipendenza nuova |
| **2** — GUI minima | invariata | costruisce il registro (ADR-0038) e il canale; il pannello della mappa nasce col **6** (decisione 17) |

### 5.2 Le righe di tracciabilità — sei

| Riga | Da | A |
|---|---|---|
| `Skills` · `Regole e vincoli di progetto` | registro delle guide → **sede da assegnare** | → **13** |
| `Collezioni e knowledge base` | 📋 Conoscenza | 📋 **6, la mappa** — col rimando al disegno |
| `Memoria persistente` | politica → Conoscenza | politica → **6**: l'assistente scrive quando giudica (decisione 5) |
| `File watching e awareness del progetto` | trigger anello 3 · politica → Conoscenza | trigger → **13** · politica → **6** |
| `Cattura con un gesto` | «la destinazione la decide il brainstorming» | **decisa**: nello spazio della knowledge base, la run la vede |

`grep -cE 'sede da assegnare' docs/tracciabilita.md` passa da **3 a 1** — resta la cifratura reale.
Si prova nelle due direzioni.

### 5.3 Le dipendenze dichiarate

| Cosa | Da chi dipende |
|---|---|
| il **13** | **AUD-004**: un ADR del proprietario. Senza, il registro non sa cosa conservare all'approvazione di una skill |
| il piano 0 deterministico | la chiave di contesto (1.1e) — la costruisce il 13 |
| il pannello | il registro di ADR-0038 (dal 2) e l'indice (dal 6) |
| il piano 2, la ricerca | l'arbitro (c'è); i modelli locali del **9** se gli embedding sono locali |
| la cattura nella knowledge base | **nessuna nuova**: file + riferimento bastano oggi |

### 5.4 Le voci che restano aperte

Tutte col chiusore nella tabella 3.4. ⛔ **Una sola sbarra qualcosa: AUD-004 sbarra il 13** — non
questo disegno, non il piano dei documenti.

### 5.5 L'ordine di ciò che segue

⛔ **Il prossimo passo vive nella §6 del compendio, in un posto solo.** Qui sta l'ordine approvato:

1. ✅ la §5 approvata **chiude il brainstorming**, 2026-09-04.
2. ✅ la sessione successiva scrive il disegno **sul posto, in questo file** — **fatto il 2026-09-04**,
   lo stesso giorno.
3. ✅ il proprietario rilegge — **fatto il 2026-09-04**, in chat: sei voci in forma A/B, una per
   volta, e il consiglio scritto scelto sei volte su sei.
4. ✅ il **piano dei documenti** con `superpowers:writing-plans` — **scritto il 2026-09-04, pre-controllo
   fatto**, in [`plans/2026-09-04-knowledge-base-documenti.md`](../plans/2026-09-04-knowledge-base-documenti.md);
   l'**esecuzione** va in una sessione nuova, e a che punto sia lo dice la tabella della posizione del piano
   — **nessun codice**: i rimandi datati
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

## 6. Verificato, dedotto, assunto

### 6.1 Verificato nel sorgente, il 2026-09-04

Letto il giorno dell'approvazione contro `c3c7a5d`, e riletto scrivendo il disegno contro `07ab6dc`,
con gli stessi comandi e **gli stessi esiti** — il codice non è cambiato fra i due commit.

| Affermazione | Comando |
|---|---|
| il registro delle guide **non esiste** nel codice; nemmeno i trigger né la proiezione come tipi | `grep -rl --include='*.rs' -E 'Guide\|Trigger\|Projection' crates/` rende **0** file |
| il tratto `Sensor` esiste, con `declared_cost()` e `observe(&self, artefact: &Untrusted) -> Verdict`, e il doc dice *«THE ARTEFACT IS `&`, NEVER `&mut`»* | `sed -n '/^pub trait Sensor/,/^}/p' crates/kernel/src/sensor.rs` |
| `CheckpointId(u64)` esiste nella porta `filesystem`; il tipo «ambito» no — nessun `Scope` in tutto `crates/` | `grep -rnE --include='*.rs' 'pub (struct\|enum) \w*(Scope\|Checkpoint)\w*' crates/` rende **una** riga, in `crates/kernel/src/ports/filesystem.rs` |
| `Untrusted` e `Instruction` esistono ed entrano in molti file | `grep -rl --include='*.rs' 'Untrusted' crates/ \| wc -l` |
| «attivo ora è una proiezione del giornale, non un secondo archivio» è la forma di `permission.rs` e `degradation.rs` — e `degradation.rs` lo dice **citando** `permission`: *«IT IS A PROJECTION AND NOT A SECOND ARCHIVE, exactly as `crate::permission` is»* | `grep -rn --include='*.rs' -i 'projection' crates/kernel/src/` |
| i moduli pubblici di `kernel`, con `sensor`, `gateway`, `permission`, `degradation`; nessuno che si chiami `guide`, `trigger`, `projection` o `knowledge` | `grep -n '^pub mod' crates/kernel/src/lib.rs` |
| il codice **non è cambiato** fra la consegna e questo disegno | `git diff --stat c3c7a5d..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende niente |

### 6.2 Verificato nei documenti, il 2026-09-04

| Affermazione | Dove, e il comando |
|---|---|
| la knowledge base ha già una sede: il sotto-progetto **6** «Conoscenza / RAG», L2, che dipende dal 3; il 3 dipende da 1 e 2; l'ultima riga della tabella è la **12**, e la **8** dipende anche da 12 | [`roadmap.md`](../../roadmap.md), tabella «Sotto-progetti»: `grep -n -E '^\| \*{0,2}[0-9]{1,2}\*{0,2} \|' docs/roadmap.md` |
| la tabella «Perché quest'ordine» della roadmap porta **una riga per ogni scelta d'ordine**, e la riga 12 ne ha ricevuta una — *«Gesti dopo GUI minima e Conversazione, e prima di Voce»* | `grep -n 'Gesti dopo' docs/roadmap.md`. ⚠️ **Conseguenza per il piano:** la riga 13 ne vuole una, e la cella del 3 pure — trappola 6 |
| la tabella «Decisioni ancora da prendere» della roadmap **non nomina** la knowledge base né le guide: non c'è una riga da barrare | `awk '/^## Decisioni ancora da prendere/{s=1} s&&/^## Regola/{s=0} s' docs/roadmap.md \| grep -ci guid`, e lo stesso con `knowledge`: **0** entrambi. ⚠️ Due `grep` e non uno, per la trappola 14 |
| `Skills` e `Regole e vincoli di progetto` poggiano sul registro delle guide con **sede da assegnare**; la terza riga col marcatore è `Storage e cifratura a riposo`, che resta: da **3** a **1** | [`tracciabilita.md`](../../tracciabilita.md): `grep -n 'sede da assegnare' docs/tracciabilita.md` |
| le **sei** righe della §5.2 sono sei **righe di `tracciabilita.md`** — `Skills` e `Regole e vincoli` sono due righe distinte — in cinque righe della tabella della §5.2 | `grep -n -E 'Skills\|Regole e vincoli\|Collezioni e knowledge base\|Memoria persistente\|File watching\|Cattura con un gesto' docs/tracciabilita.md` rende **sei** righe di funzionalità più una di commento |
| ⛔ **RICHIAMO DEL 2026-09-04 — la consegna diceva *«le diciotto funzionalità della sezione 3 «Conoscenza»»*, e sono SEDICI.** Contate delimitando per intestazione, con il comando accanto; la consegna aveva scritto la cifra **senza** comando, ed è la specie di affermazione che il gotcha **#31** produce. Nessuna decisione ne dipende: la cifra è **tolta**, resta il comando | `awk '/^## 3\. Conoscenza/{s=1;next} s&&/^## /{s=0} s&&/^[|] /&&!/^[|] Funzionalità/{n++} END{print n}' docs/tracciabilita.md` |
| **AUD-004**: il **finding** è ✅ chiuso nella tabella dei 73 (`5d66088`) — ha chiuso il *fatto falso*, *«l'unica eccezione»* — e la **decisione** che il rimedio ha registrato, *se le cinque difese si estendano alle skill*, è *«registrata, non presa … vuole un ADR proprio, del proprietario»*. ⚠️ **Quindi «AUD-004 sbarra il 13» si legge: la DECISIONE registrata da AUD-004**, non il finding, che nessuno deve riaprire | [`audit-2026-08-27.md`](../../audit-2026-08-27.md), riga **004** della tabella dei 73 e la riga *«Un rimedio può fermarsi PRIMA di decidere»* di «Le decisioni prese rimediando»; la scheda: `grep -n 'AUD-004' docs/audit-2026-08-27.md` |
| il registro delle guide fa *«archiviazione, versionamento, iniezione nella proiezione»*, e le skill dichiarative *«sono guide»*; i trigger partono da *«pianificazione, cambiamento di file, fine di un'altra run»* | [ADR-0009](../../adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md): `grep -n 'Registro delle guide\|sono guide' docs/adr/0009-*.md` |
| il routing risolve una **configurazione** e la giornala; «modello preferito» è un vincolo dell'utente | [ADR-0011](../../adr/0011-routing-risolto-e-giornalato-per-richiesta.md) riga 21 e 51; [ADR-0012](../../adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md) righe 42 e 60 |
| artefatti: non cifrati, **nel backup**; indici ed embedding: **no**, rigenerabili | [ADR-0022](../../adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) |
| un **ambito di lavoro** è *«un insieme di percorsi dichiarato esplicitamente»*, il checkpoint *«copre quelli e nient'altro»*, e la Conoscenza è nominata fra chi scrive: *«Conoscenza scrive indici e documenti derivati»* | [ADR-0024](../../adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md), «Decision» 1 e 4, e il «Context». ⚠️ Il file si chiama *ad-ambiti*, non *su-ambiti*: `ls docs/adr \| grep 0024` |
| il registro delle funzioni: *«un registro, molti invocatori, lo stesso permesso»*; lo costruisce *«il primo invocatore, il click del sotto-progetto 2»*; la manipolazione della GUI **non** passa dal registro | [ADR-0038](../../adr/0038-registro-delle-funzioni-del-programma.md), e la voce §5 del compendio |
| la decisione 7 dei gesti: la tabella delle decisioni del disegno dei gesti la dà *«⏳ aperta, dipendenza dichiarata — il brainstorming 2, la knowledge base»*; ADR-0039 ha la riga *«la destinazione di una cattura \| il brainstorming della knowledge base — decisione 7»* | [disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md), tabella delle decisioni; [ADR-0039](../../adr/0039-telecamera-come-sorgente-di-percezione.md): `grep -n -i 'destinazione' docs/adr/0039-*.md` |
| la forma del **rimando in testa a un ADR** è già in uso: un blocco citato subito sotto `Deciders`, che comincia con *«⚠️ Rimando del ‹data› — …»* e dichiara *«Nessuna riga di questo ADR è superata»* | [ADR-0001](../../adr/0001-architettura-a-kernel-con-capacita-paritarie.md), righe 7–16: `sed -n '7,16p' docs/adr/0001-*.md`. Le teste di 0008, 0009, 0010 e 0038 **non** portano ancora un rimando: `sed -n '1,8p'` su ciascuno |
| G7 — artefatti con anteprima viva — è nella mappa funzionale, area 2 | `spikes/GUI-REQUISITI.md`, riga G7 |
| «conversazione e **conoscenza**» è già il primo pilastro | [ADR-0001](../../adr/0001-architettura-a-kernel-con-capacita-paritarie.md), e la §1 del compendio |
| la §12 del compendio punta a file in **cinque** cartelle distinte — la frase della §4.1 | `awk '/^## 12\./{s=1;next} s&&/^## /{s=0} s&&/^[|]/' docs/COMPENDIO.md \| grep -oE '\]\(([^)#]*\.md)' \| sed 's/^](//' \| sed 's#/[^/]*$##' \| sed 's#^[^/]*\.md$#.#' \| sort -u \| wc -l` |
| il tetto del compendio | `grep -n '^ceiling=' scripts/check-docs.sh` contro `wc -c docs/COMPENDIO.md` |

### 6.3 Verificato nello stato dell'arte — nessuna fonte esterna, e dichiarato

⚠️ **Né il brainstorming né questo disegno hanno aperto una fonte esterna, deliberatamente:** ogni
decisione poggia su decisioni **del repository**, e sul progetto stesso la fonte più aggiornata è il
progetto. Il concetto di partenza del proprietario — i tre livelli, i router, il *visual second brain* —
è **suo**, e sta nelle premesse con le sue parole. ⛔ **Il piano 2 — embedding, indici, ricerca
ibrida — vorrà fonti primarie con la data** quando il sotto-progetto 6 lo disegnerà: non qui, perché
qui non si disegna.

### 6.4 Dedotto, e dichiarato tale

| Deduzione | Da che cosa |
|---|---|
| che «guida per modello» sia il modo giusto di realizzare il `claude.md`/`deepseek.md` del proprietario | ADR-0009 (iniezione per contesto) + ADR-0010 (budget per modello) + ADR-0011 (il modello risolto nel record) |
| che le **due pretese** sul registro delle guide — chiave di contesto; provenienza e impronta con «approvate ora» come proiezione — siano **tutte** le pretese della mappa | la lettura dei piani 0 e 1 contro ADR-0009 e `permission.rs`. Il disegno le ha messe alla prova sezione per sezione, e il 13 le riprova costruendo |
| che l'«ambito» del piano 0 sia l'**ambito di lavoro** di ADR-0024, riusato | ADR-0024: un insieme di percorsi dichiarato; `CheckpointId` esiste. La forma la dà il sotto-progetto 13 — decisione 15 |
| che nessun ADR nuovo serva, e bastino rimandi datati | la regola append-only di `CLAUDE.md`: *completato → un rimando* |
| che «tenuto dal core» per l'indice non violi I1 | è derivato e rigenerabile, come il degrado: la forma di `degradation.rs` |
| la forma dell'indice (nodi, attributi, frecce, segnali) nella §4.2 | è la proposta minima; il 6 la mette alla prova |

### 6.5 Assunto, e chi lo misura

| Assunzione | Chi la misura |
|---|---|
| che il sotto-progetto **3** sia la prima capacità che inietta una guida — la roadmap non lo dice | la §5.1: se non lo è, il 13 va comunque prima della prima che lo fa |
| che la mappa (router centrale + router dell'ambito) stia nel budget del modello **più piccolo** che il proprietario userà | il sotto-progetto 6, sul primo modello locale |
| che «al primo salto» si ottenga davvero | il 6, coi numeri del giornale |
| che misurare la proiezione per categoria costi poco | il 13, quando la costruisce |

---

## 7. Le fonti

**Nessuna esterna**, dichiarato nella §6.3: ogni affermazione poggia su un documento o un sorgente del
repository, letto il 2026-09-04, col comando accanto. ⚠️ **Quindi il piano dei documenti non porta
niente in `riferimenti.md`** per questo disegno: la regola di `CLAUDE.md` scatta *«se la voce ha
portato una misura o una fonte»*, e qui le misure sono comandi sul repository, che vivono accanto
all'affermazione che sostengono.

---

## Cosa questo disegno ha misurato, e che non era scritto da nessuna parte

| # | Misurato il 2026-09-04 | Che cosa ne segue |
|---|---|---|
| 1 | **la sezione 3 di `tracciabilita.md` ha SEDICI righe, non diciotto** — contate delimitando per intestazione, comando nella §6.2 | l'affermazione della consegna è **più debole di come era scritta**; nessuna decisione ne dipende, la cifra è tolta e resta il comando. È il gotcha **#31** dentro una consegna che dichiara *«ogni cifra col comando»*: la riga non aveva il comando, ed è stata la sola a sbagliare |
| 2 | il codice **non è cambiato** fra la consegna e il disegno: `git diff --stat c3c7a5d..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` è vuoto; e da `c3c7a5d` a `07ab6dc` c'è **un** commit, la consegna | le verifiche della consegna valgono per il codice di oggi; sono state comunque **rilanciate**, non citate, e gli esiti coincidono |
| 3 | il cancello rilanciato **all'apertura**, prima di toccare un file: `bash scripts/gate.sh` → `GATE GREEN`, `bash scripts/check-docs.sh` → `OK`; e `cargo test --locked --workspace --no-fail-fast` → **48 target, 354 passate, 0 fallite, 2 ignorate**, contate con `grep -oE 'test result: ok\. [0-9]+ passed; [0-9]+ failed; [0-9]+ ignored' \| awk '{t++; p+=$4; f+=$6; g+=$8} END{print t, p, f, g}'` | la baseline da cui il piano parte è verde, e si **rimisura** all'apertura del piano invece di leggersi qui. ⚠️ La cifra sta qui **una volta**, con la data e il comando, perché è la baseline di questa sessione e non vive in nessun altro documento |
| 4 | il margine del compendio **prima** di muovere il puntatore: `wc -c docs/COMPENDIO.md` → 175 499 contro `ceiling=188416` in `scripts/check-docs.sh`, cioè **12 917** byte; la consegna ne aveva misurati 13 454, e la differenza è ciò che il puntatore della consegna ha consumato | il piano dei documenti aggiungerà **cinque** rimandi nelle voci di §5 e **una** riga in §12: si misura **prima** di scrivere, e ciò che è verbale va in `archivio/`. Trappola 2 |
| 5 | la §12 del compendio punta a file in **cinque** cartelle distinte — comando nella §6.2 | la frase della §4.1, *«un router che punta a file in cinque cartelle»*, **regge** letta col comando; era scritta a occhio |
| 6 | il disegno dei gesti entrò nella §12 del compendio e in `README.md` con l'**esecuzione del piano** (`92feec3`, compito 9), non quando fu scritto (`c9fcd40`): `git log --format='%h %ad %s' --date=short -S'2026-09-03-riconoscimento-gesti-design' -- docs/COMPENDIO.md docs/README.md` | «questo file nella §12 e in `README.md`» resta un **compito del piano**, come la §5.5 già dice: questo disegno **non** vi si aggiunge da solo. Il puntatore della §6, invece, è mosso oggi, perché il prossimo passo è cambiato — lo stesso precedente |
| 7 | i **fine-riga** dei file che il piano toccherà: `docs/COMPENDIO.md`, `docs/README.md`, `docs/roadmap.md` e `docs/tracciabilita.md` sono LF nell'indice e **CRLF** nell'albero; i cinque ADR che ricevono un rimando (0008, 0009, 0010, 0038, 0039) e il disegno dei gesti sono **LF** in entrambi; e ventuno ADR su trentanove sono CRLF nell'albero: `git ls-files --eol docs/COMPENDIO.md docs/README.md docs/roadmap.md docs/tracciabilita.md docs/adr/` | chi esegue il piano conserva i fine-riga **di ciascun file** e li rimisura dopo — trappola 3. ⚠️ I cinque ADR da toccare sono tutti LF: una scrittura CRLF su di essi sarebbe visibile nel diff come righe cambiate che nessuno ha toccato |
| 8 | la forma del rimando in testa a un ADR **esiste già** e ha un precedente vivo: il blocco citato di ADR-0001 del 2026-09-03, subito sotto `Deciders`, con la frase *«Nessuna riga di questo ADR è superata»*; le teste di 0008, 0009, 0010 e 0038 non ne portano ancora | il piano copia **quella** forma — non ne inventa una — e la frase sulla non-superazione è la parte che rende il rimando un rimando (§3.1) |
| 9 | ADR-0024 si chiama `0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md`, mentre il suo titolo dice *«copre ambiti dichiarati»*: chi lo collega **per nome dedotto dal titolo** scrive un link rotto | `ls docs/adr \| grep 0024` prima di scrivere il link; `check-docs.sh` lo coglierebbe, ma dopo |
| 10 | **la tabella «Perché quest'ordine» della roadmap ha una riga per scelta d'ordine**, e il piano dei gesti ne aggiunse una per la riga 12; la tabella «Decisioni ancora da prendere» non nomina la knowledge base | la riga 13 e la nuova dipendenza del 3 vogliono una riga lì — la §5.5 la nomina come *«riletto»*, e il piano la scrive; nessuna riga da barrare nell'altra tabella |
| 11 | **la verifica di coerenza fatta scrivendo** — il testo approvato riletto contro le sei invarianti, le cinque proprietà della §3 del compendio e contro ADR-0001, 0007, 0008, 0009, 0010, 0011, 0012, 0014, 0018, 0020, 0022, 0023, 0024, 0038, 0039, coi comandi della §6 | **regge**, e nessuna decisione cambia. ⚠️ Con **una precisione** che il testo non faceva: *«AUD-004 sbarra il 13»* nomina la **decisione registrata** dal rimedio di AUD-004, non il finding, che è ✅ chiuso nella tabella dei 73 — §6.2 |
| 12 | **la rilettura del proprietario, il 2026-09-04 in chat, dopo la lettura obbligatoria** — le sei voci della sezione omonima poste una per volta in forma A/B, col consiglio scritto; e i comandi della sezione *«Come si riprende»* rilanciati **prima**, con lo stato dichiarato che regge riga per riga: albero pulito, nessun codice mosso da `07ab6dc`, `GATE GREEN`, `check-docs.sh` `OK`, dodici righe nella 3.4, tre `sede da assegnare`, zero finding aperti, compendio sotto il tetto | il consiglio scelto **sei volte su sei**: nessuna decisione cambia, nessun merito nuovo, e il piano scrive i consigli. ⚠️ **Ciò che la rilettura aggiunge è solo questo:** che le aggiunte dello scrivente sono ora **accettate** e non più «da leggere come sue». Nessuna verifica di coerenza nuova, perché la misura 11 era già scritta e il codice non è cambiato |

---

## Le voci che questo disegno apre per il proprietario

| # | Voce | Perché è sua, e il consiglio |
|---|---|---|
| 1 | **la rilettura di questo disegno**, sotto accettazione condizionata | ✅ **Riletto il 2026-09-04**, in chat: le aggiunte dello scrivente — la §6, la sezione «Cosa ha misurato», le trappole, la correzione della cifra — sono state dette a parole, con la precisione su AUD-004 (misura 11), e **accettate** come sue. Le cinque sezioni restano approvate nel merito. È il passo 3 della §5.5 |
| 2 | la cifra *«diciotto»* diventata **sedici** | nessuna decisione ne dipende. ✅ **Scelto il 2026-09-04: nessuna azione.** La riga vive col comando, il piano non la tocca, e l'archivio della consegna resta com'era perché è un verbale. Il comando della §6.2, rilanciato quel giorno, rende ancora sedici |
| 3 | la decisione **11** — la decisione registrata da AUD-004, se le difese di ADR-0015 si estendano alle skill | è la sola che **sbarra** qualcosa, il 13, e vuole un ADR suo. ✅ **Scelto il 2026-09-04 — il QUANDO, non il merito:** l'ADR si scrive **in parallelo** al sotto-progetto 2 (§5.5, punto 5), perché il 13 viene dopo il 2 (decisione 16) e non deve aspettare; e **prima** del brainstorming del 13, perché il registro *«deve saperlo per nascere giusto»* (§1.2). Il merito resta suo e non è stato deciso |
| 4 | il **nome esatto** della riga 13 | ✅ **Scelto il 2026-09-04:** *«Registro delle guide, trigger e proiezione»*, L0 + L1, com'era proposto. Il piano lo scrive così |
| 5 | la riga di *«Perché quest'ordine»* per il 13 | ✅ **Scelta il 2026-09-04**, la frase com'era consigliata: *«13 dopo la GUI minima e prima di Conversazione: i tre meccanismi senza codice si costruiscono prima della prima capacità che inietta una guida (§1.1d di questo disegno), e il 2 non li usa»*. Il piano la scrive così |
| 6 | le decisioni **13, 14, 15, 18**, aperte con un chiusore scritto | nessuna sbarra il disegno né il piano. ✅ **Scelto il 2026-09-04: restano aperte**, nella tabella 3.4, in una casa sola, ciascuna col proprio chiusore |

---

## Vicoli ciechi e scelte scartate, col perché

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
| «più kernel e meno modello» — router con parole-chiave abbinate dal kernel | più rigido, e più lavoro del proprietario per tenere le chiavi; offerto e non scelto — domanda 4 |
| il pannello **col 2** invece che col 6 | il 2 disegnerebbe un indice che nessuno produce ancora — domanda 12 |
| l'ordine **13, poi 2, poi 3** | la GUI aspetterebbe, e il 13 aspetta comunque AUD-004 — domanda 11 |
| un ADR nuovo per la knowledge base | una capacità L2 vive nella roadmap e nel suo disegno; tutti i meccanismi sono già decisi — §3.1 |
| **una seconda tabella delle decisioni in testa a questo file**, sulla forma del disegno dei gesti | lì la consegna la portava in testa; qui la §3.4 approvata si dichiara *«casa unica»*, e ricopiarla sarebbe il gotcha **#68** dentro il file che lo cita. Il testo in testa **rimanda** |

---

## Le trappole che mordono scrivendo il piano

| # | Trappola | Che cosa fare |
|---|---|---|
| 1 | i **rimandi datati** vanno in testa all'ADR, **append-only**, e le voci §5 del compendio vi rimandano con una riga — non copiano | la forma è il blocco citato di ADR-0001 (misura 8): sotto `Deciders`, *«⚠️ Rimando del ‹data› — …»*, con la frase *«Nessuna riga di questo ADR è superata»*. Nel compendio, la forma delle voci 0001, 0011 e 0023: *«⚠️ Rimando del ‹data›, in testa all'ADR: …»*, una riga |
| 2 | il compendio ha un **tetto in byte**, e il verde non è un margine | prima di aggiungere una riga si misura `wc -c docs/COMPENDIO.md` contro `ceiling=` in `scripts/check-docs.sh`; ciò che è verbale va in `archivio/`, non in §6. Il margine misurato oggi è nella misura 4, e si **rimisura** |
| 3 | i **fine-riga sono misti per file**: compendio, README, roadmap e tracciabilità sono CRLF nell'albero; i cinque ADR da toccare e il disegno dei gesti sono LF | si scrive con Python `newline=""` su un temporaneo e `os.replace` (gotcha #82) — l'aiutante `replace_unique.py` del [piano dei gesti](../plans/2026-09-03-riconoscimento-gesti.md) fa esattamente questo; si **rimisura dopo** con `git ls-files --eol` e `tr -cd '\r' \| wc -c` |
| 4 | i numeri di sezione **duplicati** sono un rosso, letti con `^#{2,6} [0-9]+(\.[0-9]+)*` per file | nessun `## N.` ripetuto; le sotto-sotto-sezioni con `####` |
| 5 | il controllo dei link verifica il **file**, mai il frammento: un'ancora è un rimando che nessuno difende | una sezione si **nomina**, non si collega con un cancelletto; i percorsi sono relativi alla cartella di **ciascun** file — ed è la regola che la §4.1 dà alla knowledge base stessa |
| 6 | la roadmap si appende **senza rinumerare**; la tabella «Perché quest'ordine» ha **una riga per scelta d'ordine** (misura 10) | la riga 13 va in coda alla tabella «Sotto-progetti»; la cella del 3 guadagna la dipendenza da 13; la cella del 6 si riscrive con le due metà e guadagna il 2; «Perché quest'ordine» riceve la riga del 13 — la voce 5 per il proprietario porta il testo consigliato |
| 7 | `tracciabilita.md`: il marcatore `sede da assegnare` è **contato** (`grep -cE`), e la riga `Cattura con un gesto` dice *«la destinazione la decide il brainstorming della knowledge base»* | da **3 a 1**, provato nelle due direzioni; la riga della cattura si riscrive **con la decisione** e non con una riga sotto; il comando del riquadro in testa al file **si rilancia** |
| 8 | ADR-0024 si chiama *ad-ambiti*, non *su-ambiti* (misura 9) | `ls docs/adr \| grep 0024` prima del link |
| 9 | la guardia dei **conteggi** di `check-docs.sh` legge `<cifra> ADR` nei documenti di stato — compendio, README, roadmap fra questi — come **totale** | *«cinque rimandi»* si scrive a parole, mai una cifra in numero seguita da «ADR»; e il totale degli ADR **non cambia**, perché non nasce nessun ADR |
| 10 | il disegno dei gesti chiude la decisione 7 **nella tabella**, con richiamo datato, e il suo comando `awk` che conta le decisioni deve rendere ancora **13** | si riscrive la **cella** della riga 7 — stato e chiusura — senza aggiungere righe; si rilancia il comando della sua testa prima e dopo |
| 11 | `README.md` è un **documento di stato** per la guardia dei conteggi, e la tabella «Specifiche» ha una riga per disegno, sulla forma della riga del disegno dei gesti | `grep -n 'riconoscimento-gesti-design' docs/README.md` dà la forma; la riga nuova la copia, con *«⛔ Non è una spec»* |
| 12 | la **pre-verifica di ogni compito** trova un difetto in tutti i compiti dispacciati finora, senza eccezione (`CLAUDE.md`) | ogni compito si rilegge contro il codice e i documenti di **allora**, non contro questo disegno né contro il piano |
| 13 | `riferimenti.md` **non si tocca** per questo disegno (§7) | il piano lo dichiara, come le passate del Traguardo 5 dichiaravano il contrario; una voce che non porta fonti non ne aggiunge |
| 14 | ⛔ **il `grep` di questa macchina — GNU grep 3.0 in Git Bash — ha due forme che tradiscono, misurate il 2026-09-04 scrivendo questo file:** con `-i` e **più di un** `-e` va in *Aborted* (exit 134); con `-E` e l'alternanza scritta `\|` la barra è **letterale** e il comando rende **0 anche dove la parola c'è** — un verde vacuo, colto solo provando la direzione «deve trovare» su un input che contiene la parola | un'alternanza si scrive in **due** `grep`, o con `-E` e la barra nuda **fuori da una cella di tabella**; e ogni `grep -c` che deve rendere 0 si prova **prima** su un input dove deve rendere 1 — la seconda direzione di `CLAUDE.md` |

---

## Il prossimo passo

⛔ **Lo dice la §6 del [compendio](../../COMPENDIO.md), in un posto solo.** L'ordine approvato, con
le spunte, è nella §5.5 di questo disegno; la prima riga senza spunta è il **piano dei documenti**.
⚠️ **RICHIAMO DEL 2026-09-04:** qui stava *«la rilettura del proprietario, poi il piano dei documenti
in una sessione nuova»*. La rilettura è fatta quel giorno, e il piano si scrive **nella stessa
sessione**, che è la strada B della sezione qui sotto.
✅ **RICHIAMO DEL 2026-09-04, sera:** il piano è scritto col pre-controllo; l'esecuzione va in una sessione nuova, e la §6 del compendio lo dice.

### Come si riprende — scritto alla chiusura della sessione del 2026-09-04, coi comandi

⚠️ **È il documento di consegna di questa sessione**, e sta qui e non in un file a parte perché il
repo ha già la sua convenzione: lo stato vive in file **tracciati**, e chi riprende legge **questo**
file per intero. Ogni riga è stata **riletta coi comandi** prima di essere scritta, non ricordata.

⛔ **DA SAPERE SUBITO: niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà,
tutto pushato, nessun codice toccato.

⚠️ **RICHIAMO DEL 2026-09-04, alla chiusura — la sessione si è chiusa PRIMA della rilettura.** Il
proprietario ha chiuso con l'istruzione *«continuiamo col prossimo step nella prossima sessione, nuovo
agente»*, senza rileggere il disegno in chat: il passo 3 della §5.5 resta **senza spunta**, e la
sessione nuova **apre con esso**. ⛔ **Nessuno lo dà per fatto:** il sì del proprietario è condizionato
e si dà in chat, non si deduce da una chiusura. La **domanda minima** con cui la sessione nuova apre,
dopo la lettura obbligatoria: *«il disegno è riletto?»* — **A**, sì: si scrive il piano dei documenti
(punti 5–7 qui sotto); **B**, no: si presentano le sei voci della sezione *«Le voci che questo disegno
apre per il proprietario»* una per volta, in forma A/B, col consiglio scritto, e **poi** si scrive il
piano nella stessa sessione se il contesto regge — la rilettura è chat, non lettura di file. La
verifica di coerenza che l'accettazione condizionata chiede è **già fatta e scritta** (misura 11); ciò
che la rilettura aggiunge si registra nella stessa sezione, come fece il disegno dei gesti.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin` — zero avanti, zero dietro: `git status -sb` dopo `git fetch --all --prune`. Nessuno stash, nessuna operazione a metà |
| I commit di questa sessione | `git log --oneline 07ab6dc..HEAD` — il primo: il disegno scritto sul posto, l'archivio della consegna, il puntatore della §6; il secondo: la trappola 14; il terzo: questa chiusura |
| Codice di prodotto | **non toccato**: `git diff --stat 07ab6dc..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/adr/` non rende nulla. Sono cambiati tre file di documentazione: questo, il compendio, e l'archivio della consegna |
| Cancello | `bash scripts/check-docs.sh` → `OK` a ogni commit; `bash scripts/gate.sh` → **`GATE GREEN`, rilanciato all'apertura (misura 3) e alla chiusura** — e nessun file che il cancello legga è cambiato fra le due corse. Si rilanciano, non si citano |
| Fine-riga | questo file e l'archivio sono **LF** nell'indice e nell'albero; il compendio è LF nell'indice e **CRLF** nell'albero, con CR = righe: `git ls-files --eol docs/COMPENDIO.md docs/archivio/consegna-brainstorming-knowledge-base.md docs/superpowers/specs/2026-09-04-knowledge-base-design.md`, e `tr -cd '\r' < docs/COMPENDIO.md \| wc -c` contro `wc -l < docs/COMPENDIO.md` |
| File temporanei | nessuno nel repository: gli script di questa sessione stanno nello scratchpad, fuori dall'albero, come `CLAUDE.md` prescrive |
| Debito lasciato | **nessuno non dichiarato**: le voci aperte sono nella tabella 3.4 col loro chiusore, e le sei voci per il proprietario nella sezione omonima |

**Le decisioni prese dal coordinatore, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | **commit senza il trailer `Co-Authored-By`** | `CLAUDE.md` dice *«senza co-autore»*, e `git log --format='%b' -40 \| grep -ci 'co-authored-by'` rende **0**; una direttiva di sistema chiedeva il contrario, e la divergenza è **portata al proprietario** — quarta sessione di fila. Costo se sbagliato: un `--amend` |
| 2 | la tabella delle decisioni **non è ricopiata in testa** al file, contro la forma del disegno dei gesti | la §3.4 approvata si dichiara casa unica; il testo in testa rimanda. Costo se sbagliato: zero — il rimando c'è |
| 3 | la cifra *«diciotto»* **tolta** dalla §6.2 e sostituita dal comando, con richiamo datato | `CLAUDE.md`: una cifra che marcisce si toglie; e la riga era l'unica senza comando. Costo se sbagliato: zero |
| 4 | questo disegno **non entra** in §12 né in `README.md` da solo | il precedente dei gesti (misura 6): è la condizione 5 della Definizione di «fatto», cioè del piano. Costo se sbagliato: una riga nel piano in meno |
| 5 | `riferimenti.md` **non toccato** | nessuna fonte esterna, nessuna misura che non sia un comando sul repo (§7). Costo se sbagliato: una riga da aggiungere |

**Il compito della sessione successiva: la rilettura del proprietario, poi il piano dei documenti.**
In ordine, e ogni riga è eseguibile:

1. `git fetch --all --prune`, poi `git status -sb` e `git log --oneline -3`: si parte da `main`, e la
   testa deve essere il commit di questa chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio per intero, a blocchi, e la testa dell'audit
   del 2026-08-27 — poi **questo file, per intero**. L'archivio della consegna **non** è lettura
   obbligatoria.
3. Il proprietario **rilegge** il disegno sotto la sua accettazione condizionata — passo 3 della §5.5.
   Le sei voci per lui stanno nella sezione omonima, con un consiglio ciascuna; se non dice altro, il
   piano scrive i consigli.
4. Prima di scrivere il piano, la regola di `CLAUDE.md` su `superpowers:writing-plans`: le voci aperte
   **si sanno prima**. Dove stanno: la tabella 3.4 di questo file; le voci senza numero AUD
   dell'[audit](../../audit-2026-08-27.md); la tabella *«Le voci aperte del Traguardo 5, in una tabella
   sola»* di [`porta-di-qualita.md`](../../porta-di-qualita.md). ⚠️ Quali abbiano come chiusore
   **questo piano** o *«il proprietario, prima»* lo decide chi lo scrive leggendo la colonna *«Chi la
   chiude»*, non questa riga: qui la sola che sbarra è la decisione **11**, e sbarra il **13**, non il
   piano.
5. `superpowers:writing-plans`: il piano in `docs/superpowers/plans/<data>-knowledge-base-documenti.md`,
   coi compiti del punto 4 della §5.5, la Definizione di «fatto» della §5.5 **copiata da qui**, e le
   voci 4 e 5 prese scrivendo le righe della roadmap. In testa: modalità subagent-driven, errata,
   pre-controllo — la forma dei piani precedenti, `ls docs/superpowers/plans/`.
6. Il pre-controllo delle quattro domande di `CLAUDE.md` su ciascun compito, **nella sessione che
   scrive il piano**; ogni compito si legge contro i documenti di **allora**, e la sezione *«Le
   trappole»* dice dove guardare.
7. L'esecuzione in una sessione **nuova**, un subagente fresco per compito, revisione fra uno e
   l'altro (`superpowers:subagent-driven-development`): la regola del proprietario.
8. A piano eseguito: la §6 del compendio porta il passo successivo — il sotto-progetto **2**, con
   AUD-004 in parallelo (§5.5, punti 5 e 6) — e questo file entra nella §12 del compendio e in
   `README.md`, come la condizione 5 della Definizione di «fatto» prescrive.

📌 **Ciò che questo disegno consegna a chi scriverà il piano**, ed è suo e non un puntatore: i
compiti del punto 4 della §5.5; la Definizione di «fatto» approvata; la tabella dei controlli per
artefatto della §1.5; le quattordici trappole; le sei voci per il proprietario coi consigli scritti; e le
misure 4, 7, 8, 9 e 10, che sono le cose che il piano avrebbe dovuto scoprire da sé.

📌 **La lezione di questa sessione, e non è un gotcha nuovo.** L'unica riga della consegna scritta
**senza** il comando accanto è l'unica che ha sbagliato — *«diciotto»* per sedici. La regola di
`CLAUDE.md` non è un ornamento: una cifra col comando si rifà, una cifra senza comando si crede.

⛔ **Vicoli ciechi di questa sessione: nessuno nuovo.** L'unica cosa scartata scrivendo è la seconda
tabella delle decisioni in testa al file, ed è registrata nella sezione dei vicoli ciechi col perché.
