# Knowledge base, la revisione: la consegna del brainstorming

⏳ **BRAINSTORMING IN CORSO, aperto il 2026-09-28.** Questo file è la **consegna** del brainstorming di revisione della
knowledge base, al percorso del suo futuro disegno — il precedente è la consegna dei
[modelli decisionali](2026-09-28-modelli-decisionali-design.md). Si aggiorna a **ogni risposta** del proprietario, nella
tabella *«Le risposte del proprietario»*, e si committa ogni volta: una sessione che muore non fa ripresentare niente.

✅ **Il 2026-09-28 il proprietario ha risposto a D1 col proprio documento**, con la guida ARMS allegata: è riportato
**parola per parola** nella sezione *«Il documento del proprietario»*, e la forma della knowledge base ora è **quella**. Le
sezioni dopo lo confrontano con ciò che esiste: che cosa cambia del 2026-09-04, quali buchi chiude, dove urta decisioni già
prese, e i casi limite nuovi.

⚠️ **Non è un disegno, e da solo non cambia niente.** Il disegno approvato resta il
[disegno della knowledge base](2026-09-04-knowledge-base-design.md) del 2026-09-04. Ciò che questa revisione deciderà di diverso
andrà là come **richiamo datato**, negli ADR come **rimando**, in roadmap e in tracciabilità come righe riscritte: mai in
silenzio — `CLAUDE.md`, *«Uno schema è una verifica, e corregge ciò che esiste»*.

## La domanda del proprietario, parola per parola — 2026-09-28

> vorrei (prima di continuare col brainstorming dell'ia decisionale ed il sotto progetto 13, riprendere l'architettura e lo
> studio della knowledge base. Perchè ho dei dubbi sulle decisioni prese, dove vivono i file, chi ce li mette, chi ci accede,
> come, quando, perchè, se in mezzo si trovano anche le repo dei progetti sulla quale lavorerò etc... più tutti i casi limite
> e ciò alla quale non abbiamo pensato.

Con `anthropic-skills:decision-principles` invocata nella stessa richiesta.

## Il documento del proprietario, parola per parola — 2026-09-28

Mandato in chat al posto della risposta a D1, col riferimento al PDF della guida ARMS. ⛔ **È la sua proposta, e le sue
parole non si ritoccano:** le frasi affermative sono sue decisioni; i punti *«Proposta»* della sezione finale sono le sue
decisioni aperte, col suo consiglio.

> # Knowledge base locale navigabile dall'agente
>
> ## Presupposti
>
> La knowledge base è una directory qualsiasi del filesystem locale, che il programma riceve come percorso di root nella configurazione a runtime. Non deve essere dedicata al programma: può essere la cartella in cui tengo già tutto quello che ho sul PC.
>
> L'unica fonte di verità è il filesystem. Il programma è una finestra su quella directory: tutto ciò che costruisce sopra (grafo in UI, indici, router) è stato derivato e si può ricostruire in qualsiasi momento dal contenuto reale.
>
> La directory viene modificata da due attori indipendenti. Io posso aggiungere, spostare, rinominare e cancellare file e cartelle dall'esterno, con qualsiasi strumento. L'agente crea, modifica e naviga file e cartelle dall'interno. Il programma quindi non può assumere di essere l'unico a scrivere, e deve riallineare lo stato derivato con quello reale in due momenti:
>
> - mentre è in esecuzione, con un watcher del filesystem;
> - all'avvio, con una scansione di riconciliazione per le modifiche fatte mentre era chiuso.
>
> Il watcher da solo non basta: a programma chiuso non vede nulla, e anche in esecuzione può perdere eventi.
>
> Non serve uno storico delle modifiche. La KB è locale, non è condivisa né sincronizzata con cloud o altre macchine, e ogni installazione ha la sua: niente versioning, audit log o sync. Serve però rilevare le modifiche, per tenere coerente lo stato derivato.
>
> ## Struttura di navigazione: due livelli
>
> **Livello strutturale.** È l'indice completo della directory: albero, metadati e testo per la ricerca. Si genera con una scansione, senza modello e quindi senza token. È sempre esaustivo e sempre riallineato. Alimenta il grafo in UI, la ricerca da UI e la ricerca dell'agente.
>
> **Livello semantico.** Un router master descrive le aree di lavoro e punta ciascuna al proprio indice d'area. Ogni indice d'area elenca i file chiave di quell'area, una riga ciascuno. È parziale per scelta: sono puntatori, non un inventario, e ogni indice sta sotto una pagina. Le aree sono aree di lavoro, non cartelle: un'area può coprire più cartelle, e una cartella può non appartenere a nessuna area. L'idea viene dalla sezione Memory della guida ARMS: a un agente serve una mappa, non cartelle ordinate.
>
> ## Setup
>
> Il setup funziona su qualsiasi macchina a partire dal solo percorso di root. È idempotente: rilanciarlo su una KB già inizializzata non duplica e non corrompe nulla. Usa solo percorsi relativi alla root e non fa assunzioni su sistema operativo o struttura preesistente. Ha due fasi:
>
> 1. **Scansione.** Genera il livello strutturale in automatico.
> 2. **Setup semantico guidato.** L'agente mi chiede in un'unica volta:
>    - quali sono le mie aree principali di lavoro;
>    - quali file o cartelle uso più spesso;
>    - cosa è privato e deve restare fuori.
>
>    Poi definisce le aree (vedi decisioni aperte, punto 2) e scrive il router master e gli indici d'area.
>
> ## Agente
>
> L'agente può leggere, cercare, creare e modificare file e cartelle. Per la cancellazione vedi le decisioni aperte.
>
> A una richiesta come "dove mi ero appuntato X" risponde così:
>
> 1. Legge il router master e sceglie l'area pertinente.
> 2. Legge l'indice di quell'area. Se X è un file chiave, lo trova al primo passaggio.
> 3. Se non lo è, cerca nel livello strutturale limitandosi alle cartelle di quell'area.
> 4. La ricerca su tutta la KB è l'ultimo ripiego.
>
> Il costo in token dipende quindi dal percorso fatto, non dalla dimensione della KB.
>
> Quando l'agente crea, sposta o modifica file, aggiorna i router interessati nello stesso turno.
>
> ## Coerenza dei router
>
> Un puntatore vecchio è peggio di nessun puntatore. Siccome io modifico la KB anche dall'esterno, la coerenza non può dipendere solo dalla disciplina dell'agente. Il riconciliatore corregge i router in modo meccanico, senza modello:
>
> - rimuove la voce di un file cancellato;
> - aggiorna il percorso di un file spostato, che riconosce dallo stesso hash.
>
> Un file nuovo aggiunto dall'esterno entra subito nel livello strutturale. Entra nei router solo se io o l'agente lo promuoviamo a file chiave.
>
> ## UI
>
> Dalla UI posso fare tutto senza passare dall'agente:
>
> - navigare la KB come grafo zoomabile o come griglia, raggruppati per cartella e per area;
> - cercare mentre digito, con filtri per tipo e per cartella;
> - cliccando un file, vederne l'anteprima e il percorso completo, con un pulsante per copiarlo;
> - creare, rinominare, spostare, modificare e cancellare file e cartelle.
>
> La vista si aggiorna da sola quando la KB cambia, anche dall'esterno.
>
> ## Vincoli
>
> **Esclusioni e privacy.** Alla root c'è un file di regole di esclusione, in stile `.gitignore`. Copre il rumore (`.git`, `node_modules`, binari, media pesanti) e ciò che è privato. Le esclusioni valgono sia per l'indicizzazione sia per ciò che l'agente può leggere, perché leggere un file significa mandarne il contenuto al modello.
>
> **Confinamento.** L'agente scrive solo dentro la root: percorsi normalizzati, niente `..`, niente symlink che puntano fuori.
>
> **Scritture concorrenti.** Prima di scrivere, l'agente verifica che il file non sia cambiato dall'ultima lettura; se è cambiato, si ferma. Senza versioning, una modifica sovrascritta è persa.
>
> ## Decisioni aperte
>
> 1. **Dove vivono indici e router.** Le opzioni sono tre:
>    - dentro ogni cartella: leggibili a mano, ma invasivi su una cartella non dedicata;
>    - in un'unica cartella nascosta `.<nomeapp>/` alla root, che contiene sia l'indice strutturale sia i router;
>    - nella directory dati dell'applicazione, fuori dalla KB: nessun impatto sulla cartella, ma gli indici non la seguono se la sposto.
>
>    Proposta: la cartella nascosta. Segue la cartella se la sposto, è invisibile di default ed è una sola cosa da ignorare o cancellare. I router restano modificabili dalla UI.
>
> 2. **Chi decide le aree.** Le opzioni sono due:
>    - aree uguali alle cartelle di primo livello: deterministico e senza costo, ma contraddice l'idea della mappa e su una cartella disordinata produce aree inutili;
>    - aree proposte dall'agente a partire dalla scansione e confermate da me: costa token una volta sola e riflette come lavoro davvero.
>
>    Proposta: la seconda.
>
> 3. **Cancellazione da parte dell'agente.** Il sistema deve gestire le cancellazioni comunque, perché posso cancellare dall'esterno. La domanda è solo se l'agente possa farlo. Proposta: sì, ma morbida (spostamento nel cestino di sistema) e con conferma. La cartella non è dedicata e non c'è versioning per recuperare.
>
> 4. **Collegamenti nel grafo.** Il contenimento in cartella e l'appartenenza a un'area si ricavano da soli. Resta da decidere se servono anche archi per i link espliciti tra file, come i link markdown.

## La fonte del documento: la guida ARMS

Il proprietario la indica come origine del livello semantico. È una **guida di pratica** di RoboNuggets, non una norma né
una documentazione di prodotto: vale come **origine dell'idea**, non come prova. La provenienza sta in
[`riferimenti.md`](../../riferimenti.md), nella sezione datata di questa revisione.

| | |
|---|---|
| il file | `C:\Users\zagor\Desktop\ARMS-Agentic-OS-Guide.pdf`, sul Desktop del proprietario e **fuori** dal repository; il testo si estrae con `pdftotext -layout`, che su questa macchina viene con Git per Windows |
| la sezione | *Memory*, pagine 6 e 7: tre livelli — una cartella, poi i **router** (un router master che nomina le aree e punta ciascuna al suo indice; un indice per area coi file chiave, una riga l'uno, sotto una pagina), poi un *visual second brain* — e il criterio di fatto: una sessione nuova trova il file giusto **al primo salto** |
| che cosa il documento aggiunge alla guida | i **due attori** e il riallineamento — sorvegliante più scansione all'avvio —; il **livello strutturale** senza modello; il riconciliatore **meccanico**; le **esclusioni** valide anche per ciò che l'agente legge; il confinamento; il controllo prima di scrivere. La guida rifà l'indice rilanciando uno script a mano |
| la data | il file porta la data del 2026-09-04, il giorno del primo brainstorming della knowledge base — `ls -la` sul file — mentre il disegno di quel giorno dichiara *«nessuna fonte esterna»* (§6.3) |

## Le regole di questo lavoro

| Regola | Da dove viene |
|---|---|
| ogni scelta si controlla **esplicitamente** sui cinque criteri di `anthropic-skills:decision-principles`, con verificato, dedotto e assunto separati | la richiesta del proprietario, qui sopra |
| una domanda per volta, in forma A/B, col costo di ciascuna opzione, ciò che si rifà dopo e il consiglio; prima a parole semplici | `CLAUDE.md`, *«Schema-first, ma prima a parole»* |
| le dodici risposte del 2026-09-04 sono del proprietario: si riaprono **perché è lui a chiederlo**, e ogni domanda dice che cosa cambierebbe rispetto alla risposta di allora | `CLAUDE.md`, *«Un'idea nuova può essere già stata scartata»*; la testa del disegno del 2026-09-04 |
| la revisione viene **prima** del sotto-progetto 13 e del brainstorming dei modelli decisionali | la richiesta del proprietario |
| la strada: questo brainstorming → il disegno, in una sessione sua → le correzioni ai documenti → poi il 13 | `CLAUDE.md`, *«Una fase per sessione»* |
| ⛔ il 13 costruisce **registro delle guide, trigger e proiezione**: ogni buco che li tocca si chiude **qui**, prima che il 13 li costruisca | §2.4 del disegno del 2026-09-04, *«il primo paga»* |
| dove i software di oggi hanno una risposta, la si **legge alla fonte e la si adotta**; al proprietario resta ciò che **urta** una decisione del progetto, o dove le fonti divergono | la consegna del proprietario del 2026-09-29, a D11 e a D9 |

## Che cosa esiste oggi — verificato il 2026-09-28 sull'albero di `f830cb9`

Ogni riga si rifà col comando accanto; il codice può muoversi, e il comando si rilancia prima di fidarsi della riga.

✅ **Rilanciati il 2026-09-29 dalla revisione, su `561140e`:** ogni comando della tabella rende come scritto, e il codice
non si è mosso da `f830cb9` — `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende
nulla. La tabella resta la fotografia di **prima** delle risposte: ciò che è deciso dopo sta nelle tabelle dei buchi.

| Domanda | Che cosa è deciso oggi | Dove, e il comando | Stato |
|---|---|---|---|
| **dove** vivono i dati del programma — giornale, disposizione dei pannelli | **niente**: il daemon scrive `journal.redb` e `layout.redb` nella **cartella da cui parte**, e il sorgente lo dichiara — *«where a per-user data directory belongs is a decision no ADR has taken»* | `grep -n -e 'JOURNAL_PATH: ' -e 'LAYOUT_PATH: ' -e 'no ADR has taken' crates/daemon/src/main.rs` | ❌ buco |
| **dove** vive la cartella della knowledge base | un «archivio unico» di file dell'utente, in chiaro e nel backup — ma **nessun percorso**, e nessuna regola per sceglierlo | §2.2 del disegno del 2026-09-04; ADR-0022; [design/09](../../design/09-l0-fisico.md) | ❌ buco |
| **dove** vivono gli altri file prodotti dalle run — documenti, codice, asset 3D, esportazioni | l'archivio «artefatti» di ADR-0022; le **catture** nella knowledge base (decisione 7); la rete della Home li mostra **tutti** (decisione 1 della [stella polare della GUI](2026-09-07-direzione-gui-design.md)) — ma nessun posto su disco | `grep -n 'Nell.anello solo' docs/superpowers/specs/2026-09-07-direzione-gui-design.md` | ⚠️ a metà |
| le **repo** dei progetti: dentro o fuori | **mai discusse**. La risposta 1 del 2026-09-04 dice *«Progetti, note, tutto dentro»*, e non dice se «progetti» sono le note **sui** progetti o le repo **stesse**. In tracciabilità `Multi-repo/multi-progetto` e `Mappa del progetto` stanno in Conoscenza come 📋, non disegnate, e `Git e gestione branch` in Coding | `grep -n -i 'repo' docs/superpowers/specs/2026-09-04-knowledge-base-design.md` rende solo «repository» nel senso di **questo** repository; `grep -n -e 'Multi-repo' -e 'Mappa del progetto' -e 'Git e gestione' docs/tracciabilita.md` | ❌ buco |
| **chi scrive** | solo il nostro assistente, attraverso il kernel (decisione 3); le note quando l'assistente giudica che vale (5); il proprietario dal pannello, con le funzioni del registro (10); le catture (7); una modifica a mano diventa **«da fuori»** finché non è approvata | le risposte 3, 5, 7 e 9, e la §1.4 del disegno | ✅ deciso — con i casi K5–K9 e K15 |
| **chi legge** | il kernel al piano 0, per chiavi; il modello al piano 1, un salto giornalato alla volta; la ricerca al piano 2, dopo; il pannello **dall'indice**, via `ipc`, mai dai file | §1.1c, §4.2 e §4.3 del disegno | ✅ deciso — i server MCP non considerati, K12 |
| **come** | ogni scrittura è un effetto giornalato con classe, checkpointato nell'ambito; i collegamenti puntano a **file**; le CRUD sono funzioni del registro | §2.3 e §4.1 del disegno | ✅ deciso |
| **quando** | il piano 0 a ogni passo; le note quando l'assistente giudica; i trigger quando un file cambia | §1.1c e §2.2 del disegno | ⚠️ i cambi a programma **spento** non li vede nessuno, K8 |
| **perché** | l'agente **salta** invece di frugare: meno contesto, meno token, e funziona senza GPU | le premesse del disegno | ✅ |
| **chi costruisce** la porta `filesystem` vera | design/09 dice *«l'implementazione vera col 5»*, Coding; ma il 6, Conoscenza, **non dipende** dal 5 e può arrivare prima | `grep -n 'implementazione vera col 5' docs/design/09-l0-fisico.md`; le righe 5 e 6 di [`roadmap.md`](../../roadmap.md) | ⚠️ incoerenza d'ordine, K23 |

## Che cosa arriva, e se regge crescendo

| Arriva | Che cosa porta alla knowledge base |
|---|---|
| **13** | registro delle guide, trigger e proiezione, **nel kernel**: li paga lui |
| **3** e **4** | la prima capacità che inietta una guida; poi gli agenti, anche **più run insieme**, e i server MCP — `grep -n 'approvazione MCP' docs/tracciabilita.md` li dà agli Agenti |
| **5** | Coding: **repo e git**, e la porta `filesystem` vera secondo design/09 |
| **6** | la mappa, poi la ricerca |
| **7** | gli asset 3D: **file grandi** |
| **11** | backup e ripristino |
| **12** | le catture della telecamera, che atterrano nella knowledge base |

⛔ **Regge crescendo? No, su tre punti.** Quando il 5 apre una repo, la decisione 3 — *«scrive solo il nostro assistente»* —
incontra una cartella scritta da git, dagli editor e da altri agenti. Quando il 7 scrive un asset da qualche GB, checkpoint e
backup della cartella si gonfiano, e il limite di dimensione che ADR-0024 chiede non è mai stato fissato. Quando il
proprietario usa una seconda macchina, niente dice se la knowledge base lo segue. **Se non si decide qui, lo decidono il 5 o il
6 da soli: due strade.**

✅ **Riletto il 2026-09-29, dopo le risposte:** il primo punto lo chiudono i due attori del documento e le zone di lavoro
di D3; il secondo è a metà — i file grandi restano nodi del grafo senza che lo scanner li apra, D5; la copia costa solo sui
file che l'agente tocca; il limite di ADR-0024 resta da fissare, K21; e che cosa salva il backup lo decide D12 —; il terzo lo
chiude K5, una knowledge base per installazione.

## I buchi e i casi limite

Specie: **V** verificato col comando; **D** dedotto; **F** da verificare alla fonte primaria prima di deciderci sopra. La
colonna *13?* dice se il buco tocca ciò che il 13 costruisce: quelli con ✅ si chiudono **prima** del 13.

| # | Il caso | Specie | 13? | Chi lo chiude, proposto |
|---|---|---|---|---|
| **K1** | **i dati del programma non hanno una casa**: il daemon scrive nella cartella da cui parte | V | — | questa revisione decide la regola; la costruisce il primo sotto-progetto che installa il programma |
| **K2** | **la cartella della knowledge base non ha un posto**, né una regola per sceglierlo | V | — | questa revisione |
| **K3** | **le repo: dentro o fuori** la knowledge base | V | ✅ la chiave «ambito» del piano 0 e il «router dell'ambito» dipendono da qui | questa revisione, **D1** |
| **K4** | **gli altri file delle run** — codice, documenti, asset 3D, esportazioni: dove stanno, e se sono nodi della mappa | V | — | questa revisione, con la decisione 1 della stella polare |
| **K5** | **più macchine**: niente dice se la knowledge base segue il proprietario. Il giornale è cifrato con le chiavi dell'OS (ADR-0023), e «approvate ora» è una proiezione del giornale: una skill approvata su una macchina **non** lo è sull'altra. `grep -rn -i 'più macchine' docs/adr/` e `grep -rn -i 'sincronizz' docs/adr/` rendono niente | V + D | ✅ lo stato delle approvazioni | questa revisione, **D2** |
| **K6** | **cartelle sincronizzate da terzi**: su Windows, Documenti e Desktop possono stare sotto OneDrive — un terzo che scrive la cartella, copie di conflitto, file «solo online» che non si leggono | F | — | questa revisione, con K2 |
| **K7** | **programmi che scrivono lo stesso** — git, un editor, l'antivirus: la decisione 3 è una **convenzione**, non un lucchetto, e l'unica difesa è **vedere** il cambio e marcarlo «da fuori» | D | ✅ i trigger | questa revisione |
| **K8** | **cambi a programma spento**: il sorvegliante non c'era. All'avvio serve un **confronto delle impronte** con l'ultimo stato noto — la riconciliazione di ADR-0007, applicata alla cartella | D | ✅ i trigger | questa revisione, poi il 13 |
| **K9** | **eventi persi dal sorvegliante**: un sorvegliante di file può perdere eventi — buffer pieno, dischi di rete — e allora serve una scansione intera | F | ✅ i trigger | il 13, alla fonte |
| **K10** | **due run che scrivono lo stesso file** — gli agenti del 4: nessuna regola. Una forma nota: si scrive **solo se** il file è ancora la versione letta, altrimenti è un conflitto | D | — | chi costruisce la porta `filesystem` vera |
| **K11** | **un segreto incollato in una nota**: la knowledge base è in chiaro e nel backup, cioè il vettore di fuga che ADR-0022 vieta per i segreti; il sensore chiesto dal seguito di ADR-0016 guarda solo l'input dell'utente | V + D | — | il 6: un sensore sulle scritture, col canary che esiste già |
| **K12** | **i server MCP** hanno un accesso ai file tutto loro: uno puntato sulla cartella legge fuori dalla mappa e fuori dal budget | D | — | il 4, che porta MCP: la tripla di ADR-0016 sui percorsi della knowledge base |
| **K13** | **collegamenti che escono dalla cartella** — un router che punta a un file di una repo, o a un percorso qualunque: ammessi o no? Se sì, il modello legge file arbitrari attraverso la mappa | D | ✅ il piano 1 | questa revisione, con D1 |
| **K14** | **collegamenti simbolici e junction** dentro la cartella che puntano fuori: si esce dall'ambito. Il kernel confronta **byte**, non file — lo prova `two_spellings_of_one_file_are_two_paths`, in `crates/kernel/tests/ports_are_implementable.rs` — quindi «dentro l'ambito» va deciso sul **percorso vero** da chi implementa la porta | V + D | — | chi costruisce la porta `filesystem` vera |
| **K15** | **i file-guida dentro le repo** — `CLAUDE.md`, `AGENTS.md`, `.cursorrules`, le cartelle di skill di una repo clonata: sembrano guide. Iniettarli da soli vorrebbe dire che la repo di uno sconosciuto dà ordini. Sono **«da fuori»**: informano, non autorizzano; diventano guide solo importati e approvati con l'impronta — è la sorella di AUD-004 | D | ✅ l'ingresso del registro delle guide | questa revisione, con AUD-004 |
| **K16** | **il modello cambia per un fallback**: la proiezione — budget e guida del modello — è composta per il modello chiesto; se il routing ripiega su un altro, e ADR-0012 elenca il *«contesto eccessivo»* fra le cause, va ricomposta per il modello **risolto** | V + D | ✅ la proiezione | il 13 |
| **K17** | **cancellare «davvero»**: una nota cancellata resta nei checkpoint, nei payload del giornale e nei backup finché non sono potati; «dimentica questo» oggi non esiste | D | — | il traguardo della ritenzione, e l'11 |
| **K18** | **rinomine fatte fuori dal programma**: un collegamento rotto più un orfano. Con l'impronta si riconoscono come lo stesso file, e l'anello di miglioramento **propone** la correzione | D | — | il 6 |
| **K19** | **percorsi fra Windows e Linux**: maiuscole, separatori, nomi riservati come `CON` e `NUL`, lunghezza massima — la lunghezza ha già morso questo repository, gotcha #117. Spostare la cartella su Linux può rompere i collegamenti che differiscono per una maiuscola | V + D | — | chi costruisce la porta `filesystem` vera, e il 10 |
| **K20** | **importare da fuori: copia, sposta o collega?** La regola 4 della §2.3 del disegno dice che il file entra «da fuori», non quale delle tre: un file da 2 GB copiato raddoppia il disco, spostato lascia il suo posto, collegato resta fuori da ambito, checkpoint e backup | D | — | il 6 |
| **K21** | **file grandi** — asset 3D, video, dataset: il costo del checkpoint e del backup. ADR-0024 chiede un limite di dimensione **con avviso**, mai fissato | V | — | il primo che scrive file grandi, il 5 o il 7 |
| **K22** | **la knowledge base cresce**: il router centrale, caricato **sempre** al piano 0, cresce coi gruppi; la rete della Home con migliaia di nodi | assunto nella §6.5 del disegno | — | il 6, misurando |
| **K23** | **chi paga la porta `filesystem` vera**: design/09 dice il 5, ma il 6 può arrivare prima | V | — | questa revisione, in roadmap |

## Il documento contro ciò che esiste

Ogni punto del documento del proprietario, letto contro il disegno del 2026-09-04, gli ADR e il codice. **Cambia** vuol dire
che una risposta del 2026-09-04 va corretta, col richiamo datato, quando si scriverà il disegno.
⚠️ **Riletta il 2026-09-29 dalla revisione:** una riga che una risposta ha chiuso porta ora la risposta in coda alla cella;
la parte di prima resta, perché dice che cosa la risposta ha sciolto.

| Punto del documento | Contro che cosa | Esito |
|---|---|---|
| la knowledge base è **una cartella qualsiasi**, e la root arriva dalla configurazione | la risposta 1 del 2026-09-04, *«un archivio unico»*; ADR-0034, i parametri li legge il daemon e li consegna | **cambia** la risposta 1: non più un archivio dedicato. Coerente con ADR-0034: la root è un parametro che il daemon consegna |
| **due attori**: il proprietario da fuori, con qualsiasi strumento, e l'agente da dentro | la risposta 3, *«solo il nostro assistente»* | **cambia** la risposta 3. Chiude K7 |
| il **sorvegliante** più la **scansione all'avvio** | i trigger del 13 (ADR-0009); K8 e K9 | chiude K8 e K9 nel principio; al 13 resta un requisito: il meccanismo deve saper dire *«ho perso eventi, riscansiona»* |
| **niente storico**, versioning, audit o sync; una knowledge base per installazione | ADR-0007, il giornale delle azioni; ADR-0024, la copia prima che l'agente tocchi un file | ⚠️ **da conciliare — D2**. Chiude K5: niente sync — ✅ **D2, A**, 2026-09-28: il giornale e la copia restano, per le sole azioni dell'agente |
| indici **e router** «derivati, ricostruibili» | ADR-0022, gli indici fuori dal backup perché ricostruibili | ⚠️ vale per l'indice strutturale, **non per i router**, che portano le scelte del proprietario — K32 — ✅ **D4, A**, 2026-09-29: i router in `.<nomeapp>/` alla root, l'indice nella cartella dati del programma; che cosa salva il backup, **D12** |
| il **livello strutturale**: l'indice completo, testo compreso, senza modello | il disegno del 2026-09-04: l'indice della mappa (§4.2), e la ricerca per somiglianza come **seconda metà** del 6 (§4.4) | **allarga**: una ricerca testuale senza GPU nasce con la prima metà; la seconda resta per la somiglianza |
| il **livello semantico**: router master → indici d'area → file chiave; le aree non sono cartelle | la risposta 1 e la §4.1: router → gruppi → foglie, e *«un gruppo è una voce di router, non una cartella»* | **uguale** nella sostanza: l'«area» è il «gruppo» del 2026-09-04 |
| l'agente cerca anche nel **livello strutturale**, prima nell'area e poi dappertutto | la risposta 4 e la §4.1: *«l'agente naviga la mappa, mai le cartelle»*, e gli orfani per l'agente *«non esistono»* | **cambia** le risposte 4 e 10: un file fuori dai router si trova lo stesso, con un costo in più |
| il **riconciliatore** corregge i router da solo, senza modello | la §1.4 del 2026-09-04: un sensore trova il puntatore rotto, l'anello **propone**, il proprietario approva | **cambia** per i due casi meccanici — cancellato, spostato con lo stesso hash — e resta deterministico, quindi fuori dal divieto di ADR-0020. Chiude K18 nel principio; apre K24 e K25 — ✅ **D6, A**, 2026-09-29: nel dubbio, rotto e una domanda; chiusi K24 e K25 |
| un file nuovo entra nei router solo se **promosso** | la §4.1: gli orfani mostrati nel pannello | coerente |
| la **UI**: grafo o griglia per cartella e per area, ricerca, anteprima, e tutte le CRUD **senza passare dall'agente** | la §4.3 del 2026-09-04; ADR-0038, un registro e molti invocatori | **allarga** il pannello. «Senza l'agente» vuol dire senza il **modello**: il click invoca la stessa funzione del registro, e il core la esegue, la giornala e ne controlla il permesso |
| le **esclusioni** in stile `.gitignore` alla root, per l'indice **e** per ciò che l'agente legge | la decisione 13 del 2026-09-04, *«privato ma non segreto»*, aperta; K11 e K21 | chiude la 13 nel principio; apre K26 e K27 — ✅ **D5, A**, 2026-09-29: due specie, rumore e privato; chiusi K26 e K27 |
| il **confinamento**: l'agente scrive solo dentro la root, niente `..` né collegamenti simbolici fuori | ADR-0024, l'ambito; K13 e K14 | chiude K14; K13 a metà, perché dice *scrive* e non *legge* — K28 — ✅ **D3, A**, 2026-09-29: il confine è ogni zona aperta, e confina lo scrivere **e** il leggere; chiusi K13 e K28 |
| le **scritture concorrenti**: prima di scrivere si controlla che il file non sia cambiato | K10 | chiude K10 |
| il **setup** in due fasi, idempotente, coi soli percorsi relativi | la guida ARMS, il livello dei router | nuovo: è la capacità, il 6 |
| la decisione aperta 1: **dove vivono** indici e router | ADR-0022; K30, K31, K32 | **D4** — ✅ **A**, 2026-09-29 |
| la decisione aperta 2: le **aree** proposte dall'agente e confermate dal proprietario | la risposta 10 del 2026-09-04 | coerente: la sua proposta si accoglie com'è |
| la decisione aperta 3: la **cancellazione** dell'agente, morbida e con conferma | ADR-0024, la copia prima; ADR-0016, le scritture che chiedono | coerente, e la copia del kernel è una rete in più: la sua proposta si accoglie com'è |
| la decisione aperta 4: i **link markdown** come archi del grafo | la §4.2 del 2026-09-04, le frecce della mappa | **D8** — ✅ **A**, 2026-09-29 |

## Lo stato dei buchi dopo il documento

| # | Stato |
|---|---|
| K1 | **chiuso**: i dati del programma nella cartella dati per utente del sistema — `%LOCALAPPDATA%\<nomeapp>\` su Windows, le cartelle XDG su Linux —, in sottocartelle per natura, e la configurazione porta il percorso della root. D4, 2026-09-29; lo costruisce il primo sotto-progetto che installa il programma |
| K2 | **chiuso**: la root arriva dalla configurazione |
| K3 | **chiuso**: una repo dentro la root sta nel livello strutturale, meno ciò che le esclusioni tolgono; una repo fuori è una zona di lavoro con la sua scheda progetto — D3, 2026-09-29 |
| K4 | **chiuso**: i file delle run stanno nella root o in una zona di lavoro aperta, gli unici posti dove l'agente scrive — D3; i file pesanti restano nodi del grafo — D5; quelli di una zona fuori dalla root stanno nell'anello, e nella rete c'è la scheda della zona — D13. ⚠️ **Richiamo del 2026-09-29, revisione:** diceva *«dentro la root, perché l'agente scrive solo lì»*, vero per il documento e non più dopo D3, e *«resta la visibilità dei file pesanti, K27»*, chiusa da D5 |
| K5 | **chiuso**: una knowledge base per installazione, niente sync |
| K6 | **a metà**: chi scrive da fuori è coperto dai due attori; restano i file «solo online», K34 |
| K7 | **chiuso**: due attori, e il riallineamento |
| K8 · K9 | **chiusi nel principio**: il sorvegliante più la scansione; al 13 resta l'evento «riscansiona» |
| K10 | **chiuso**: il controllo prima di scrivere |
| K11 | **a metà**: il privato si esclude, ed è un confine — D5; un segreto in una nota **non** esclusa resta — il sensore sulle scritture, al 6 |
| K12 | **aperto**, registrato: al 4 |
| K13 | **chiuso**: un collegamento che esce dalla root punta a una zona, la scheda progetto, e il modello la legge solo a zona aperta — fuori da ogni zona la porta risponde `OutsideScope`. D3, 2026-09-29 |
| K14 | **chiuso**: niente collegamenti simbolici fuori |
| K15 | **chiuso**: fiducia una volta per zona, modalità ristretta per la zona non fidata, impronta della versione caricata nel giornale — D9, 2026-09-29; le skill della knowledge base restano ad AUD-004 |
| K16 | **chiuso**: la proiezione si compone per ogni candidato e si controlla prima di mandarla; il candidato che non la tiene si salta — D10, 2026-09-29 |
| K17 | **a metà**: la cancellazione dell'agente è morbida; «dimentica davvero» resta registrato |
| K18 | **chiuso**: l'hash per il caso certo, e il dubbio segnato rotto coi candidati — K24 e K25, D6 |
| K19 | **a metà**: i percorsi relativi alla root; maiuscole e nomi riservati restano, alla porta vera |
| K20 | **chiuso**: dentro la root i file li porta il proprietario da fuori; l'agente, da una zona aperta, **copia** — legge nella zona e scrive nella root, e il file entra da fuori con la provenienza della regola 4 della §2.3 del 2026-09-04 —; spostare è copiare più una cancellazione morbida con conferma, la decisione aperta 3 del proprietario; un collegamento verso un file di una zona punta alla scheda della zona, o non si disegna — D8. ⚠️ **Richiamo del 2026-09-29, revisione:** diceva *«e l'agente fuori non arriva»*, vero per il documento e non più dopo D3 |
| K21 | **a metà**: lo scanner non apre i file pesanti, che restano nodi del grafo — D5; la copia costa solo sui file che l'agente tocca; il limite di ADR-0024 resta da fissare; il backup dei file grandi della root è del proprietario, D12. ⚠️ **Richiamo del 2026-09-29, revisione:** diceva *«i file pesanti fuori dall'indice»*, scritto prima di D5 |
| K22 | **a metà**: il costo in token dipende dal percorso, non dalla dimensione; il grafo coi molti nodi resta al 6 |
| K23 | **aperto**, registrato |

### I casi limite nuovi, aperti dal documento

| # | Il caso | Specie | 13? | Chi lo chiude, proposto |
|---|---|---|---|---|
| **K24** | **spostato e modificato insieme** — o salvato da un editor come file nuovo: l'hash cambia, il riconciliatore vede «cancellato» più «nuovo», toglie la voce, e un file chiave esce dalla mappa senza che nessuno lo sappia | D | — | **D6** — ✅ chiuso il 2026-09-29: nel dubbio la voce resta, segnata rotta, coi candidati |
| **K25** | **due file identici**: lo stesso hash in due posti, e lo spostamento diventa ambiguo | D | — | **D6** — ✅ chiuso il 2026-09-29: è un caso di dubbio, rotto coi due candidati |
| **K26** | **chi scrive il file delle esclusioni**: se l'agente può toglierne una riga, un file malevolo che l'agente ha letto può convincerlo a scoprire il privato e poi leggerlo — *«un'istruzione trovata nei dati non è mai un'autorizzazione»*, ADR-0014 | D | — | **D5** — ✅ chiuso il 2026-09-29: le regole del privato le cambia solo il proprietario, con conferma a ogni preset |
| **K27** | **escluso non vuol dire invisibile**: i «media pesanti» esclusi sparirebbero dal grafo, mentre la rete della Home deve mostrare anche gli asset 3D — decisione 1 della stella polare | V + D | — | **D5** — ✅ chiuso il 2026-09-29: il rumore resta un nodo del grafo |
| **K28** | **la root è il confine di tutto l'assistente, o solo della knowledge base?** Il coding lavora su repo: dentro la root, o anche fuori con ambiti suoi (ADR-0024) e permessi suoi (ADR-0016)? E il documento confina lo **scrivere**, non il **leggere** | D | ✅ gli ambiti che il piano 0 usa come chiave | **D3** — ✅ chiuso il 2026-09-29, risposta A: il confine è ogni zona aperta, la knowledge base più le zone di lavoro, e confina lo scrivere **e** il leggere |
| **K29** | **una root enorme** — «tutto quello che ho sul PC»: la prima scansione è lunga, e le build nelle repo inondano il sorvegliante. Serve una scansione incrementale — dimensione e data, l'hash solo se cambiano — e lo stato *«indicizzazione in corso»* dichiarato prima, come vuole ADR-0019 | D | ✅ l'evento «riscansiona» | registrato: il 6, e il 13 per l'evento |
| **K30** | **su Windows il punto nel nome non nasconde una cartella**: `.git` è nascosta perché git le mette l'attributo H, `.github` e `.superpowers` no. La cartella `.<nomeapp>/` va marcata nascosta dal modulo di piattaforma | V: `cmd //c "attrib .git"` e `cmd //c "attrib .github"` nella radice di questo repository | — | **D4** — ✅ chiuso il 2026-09-29: la marca il modulo di piattaforma |
| **K31** | **il backup**: se la root è il PC intero, il programma non può salvarla tutta, mentre ADR-0022 metteva la cartella della knowledge base nel suo backup. Il programma salva ciò che è **suo** — i router —, e il resto è dei backup del proprietario | V + D | — | **D4** — ✅ chiuso il 2026-09-29: il programma salva i suoi dati e i router, il resto è del proprietario; lo costruisce l'11 ⚠️ **Riaperto il 2026-09-29 dalla revisione:** la frase urta ADR-0022 sugli artefatti e sulle guide — **D12**, K42 — richiuso: ✅ **D12, A**, 2026-09-29, delegata allo stato dell'arte |
| **K32** | **i router non si ricostruiscono**: portano le scelte del proprietario — le aree, i file chiave, la riga di descrizione. Rifarli è rifare il setup guidato, coi suoi token e le sue domande | D | — | **D4** — ✅ chiuso il 2026-09-29: i router stanno in `.<nomeapp>/`, lontani dall'indice che si rifà, e il programma li salva |
| **K33** | **l'agente che scrive senza chiedere**: col preset di default di ADR-0016 ogni scrittura chiede conferma, mentre il documento vuole i router aggiornati nello stesso turno | V: ADR-0016, punto 2 | — | **D7** — ✅ chiuso il 2026-09-29: un sì per la sessione, la tripla sulla cartella dei router |
| **K34** | **i file «solo online» di OneDrive** dentro la root: leggerli scarica il file, o fallisce senza rete | F | — | registrato: il 6, alla fonte |
| **K35** | **una zona di lavoro si apre e non si chiude**: la porta `filesystem` ha `declare_scope` e nessuna chiusura, mentre la zona dura la sessione; e gli ambiti sono **della porta**, non della run — due run con due zone diverse, gli agenti del 4, alla porta vedrebbero l'una la zona dell'altra, e il confine per run lo dà solo il permesso di ADR-0016 | V: `grep -n '^    fn ' crates/kernel/src/ports/filesystem.rs` rende i cinque metodi del tratto, nessuno che chiuda; D: una porta sola nel daemon | — | registrato: chi costruisce la porta `filesystem` vera, con K23 |
| **K36** | **il privato escluso dalla porta non lo è per i comandi**: uno script che l'agente esegue apre i file da sé, e la porta non lo vede. Le documentazioni di Claude Code e di Cursor lo dicono dei loro prodotti; da noi il livello 1 di ADR-0025, per costruzione, non regge contro codice eseguito | V alla fonte, il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md); D per il nostro caso | — | registrato: il 5, col confinamento di livello 2 che nega i percorsi privati; il 4 per MCP, con K12 |
| **K37** | **la sessione non è definita**: ADR-0016 dice che un sì vale *«per la sessione corrente»* e *«non vale domani»*, ma nessun documento dice che cos'è una sessione; il kernel lo dichiara nel sorgente, e un permesso concesso resta concesso **per sempre**, anche dopo un riavvio; il disegno del 2 ha dato il confine a chi porta le run, il 3, senza definirlo. Trovato dal proprietario, rispondendo a D7 | V: `grep -n 'SCOPED TO A SESSION' crates/kernel/src/permission.rs`, `grep -n 'triple therefore survives' crates/kernel/src/registry.rs`, `grep -n 'il confine di sessione dei permessi' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | — | **D11** — ✅ chiuso il 2026-09-29: la run coi sotto-agenti, con la chiusura a mano e le due scadenze; la costruisce il 3 — e al riavvio del core finisce, RR3 |
| **K38** | **il sì oltre la sessione**: Claude Code e VS Code offrono anche un sì per lo spazio di lavoro o per sempre, con un comando che li azzera, e Android azzera da solo i permessi non usati; ADR-0016 dice *«un'approvazione non si estende»*, e fra le sue alternative non ha mai valutato la durata | V alla fonte, il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md); `grep -n 'Alternative considerate per i permessi' docs/adr/0016-*.md` | — | registrato: il **proprietario**, con un ADR nuovo se vorrà riaprire il punto 3 di ADR-0016 |
| **K39** | **OpenRouter comprime da solo il prompt**: la compressione *middle-out* toglie il centro sulle destinazioni con finestra fino a 8 192 token, accesa per default — proprio ciò che ADR-0008 vuole mai sacrificabile | V alla fonte, il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) | ✅ la proiezione | **D10** — ✅ chiuso il 2026-09-29: il gateway fa la catena e manda un modello per richiesta, la compressione spenta, come Claude Desktop |
| **K40** | **il fallback dentro OpenRouter**: con la lista di modelli il ripiego avviene dentro di lui, e il gateway non valuta i vincoli di ogni candidato né ricompone la proiezione; il modello usato lo sa dalla risposta | V alla fonte, il 2026-09-29; D per il gateway | ✅ la proiezione | **D10** — ✅ chiuso il 2026-09-29: il gateway fa la catena e manda un modello per richiesta, la compressione spenta, come Claude Desktop |
| **K41** | **le regole di privacy di una zona di lavoro: dove stanno, e chi le scrive.** D3 dà a una zona *«la stessa lista di base, più le regole della zona»*, e D5 vuole il privato cambiato dal **solo** proprietario; ma le esclusioni che una repo porta con sé le ha scritte chi ha scritto la repo. Lo stato dell'arte risponde: in Claude Code le regole di una repo che **negano** valgono anche prima della fiducia, perché restringono soltanto, e quelle che **concedono** solo dopo | V alla fonte il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md); D per il nostro caso | — | registrato, e **chiuso nel principio dallo stato dell'arte**: una zona porta la lista di base, che non si toglie, le regole del proprietario per quella zona, e le esclusioni della repo, che possono solo **aggiungere** privato; nessuna regola della repo rende leggibile qualcosa. Dove stanno le regole del proprietario per una zona lo decide il 5, che costruisce le zone |
| **K42** | **il backup contro ADR-0022**: D4 scrive, come cosa comune alle due risposte, che il programma salva nel suo backup *«i suoi dati e i router»* e che il resto della root è dei backup del proprietario; ma nella root stanno anche gli **artefatti** delle run — K4 — e le **guide**, che ADR-0022 mette nel backup del programma: la riga degli artefatti, e il rimando del 2026-09-08, per cui la politica delle guide — nel backup — non cambia. E *«i suoi dati»*, alla lettera, comprende i segreti e l'indice, che ADR-0022 tiene fuori | V: `grep -n -e '^. artefatti' -e 'nel backup, permanente' docs/adr/0022-*.md`; `grep -n 'Backup della KB' docs/tracciabilita.md` | — | **D12** — ✅ **D12, A**, 2026-09-29, delegata allo stato dell'arte: la root, e il suo backup, sono del proprietario |
| **K43** | **gli artefatti di una zona fuori dalla root, e la rete della Home**: la decisione 1 della [stella polare della GUI](2026-09-07-direzione-gui-design.md) mette nella rete al centro *«tutto: artefatti e file della knowledge base»*, e D3 tiene fuori dal grafo una zona esterna alla root. Il codice che l'agente scrive in una repo aperta come zona è un artefatto: per la decisione 1 sta nella rete, per D3 no | V: `grep -n 'Nell.anello solo' docs/superpowers/specs/2026-09-07-direzione-gui-design.md` | — | **D13** — ✅ A, 2026-09-29: nell'anello, e nella rete la scheda; la decisione 1 si legge «ciò che sta nella root» |
| **K44** | **la risorsa di un permesso su un percorso scelto a runtime**: la risorsa del kernel è un `&'static str`, per I6, confrontato carattere per carattere; le zone di D3 e la cartella dei router di D7 sono percorsi scelti a runtime, e D7 vuole che una tripla su una cartella copra i file dentro | V: `grep -n 'pub resource' crates/kernel/src/permission.rs` | — | registrato: chi porta le zone — il 5, o chi costruisce prima la porta vera, K23 —, con la forma di RR5: un identificativo coniato dal kernel per l'ambito, e l'appartenenza decisa da chi implementa la porta |
| **K45** | **il contratto della porta `filesystem` cresce**: chiudere un ambito, K35; le esclusioni del privato, D5; spostare e cancellare, le CRUD della knowledge base. Oggi la porta dichiara, conserva, ripristina, legge e scrive | V: `grep -n '^    fn ' crates/kernel/src/ports/filesystem.rs` | — | registrato: chi costruisce la porta vera, K23; è una porta del kernel, quindi un richiamo datato alla spec del sotto-progetto 1, del proprietario |
| **K46** | **l'autorità del riconciliatore**: scrive i router da solo nei casi certi, D6, ma non è un invocatore del registro e lavora fuori da ogni sessione, mentre ADR-0016 fa chiedere le scritture e il sì di D7 vale dentro una sessione | V: `grep -n 'pub enum Invoker' crates/kernel/src/registry.rs` | — | **D14** — ✅ A, 2026-09-29: un'impostazione — da solo, chiedi, mai — che parte da «da solo» |
| **K47** | **il candidato del gateway non sa dire un modello scelto a runtime, né la sua finestra**: il nome è un `&'static str`, per I6, e la finestra non c'è — RR12, RR13 | V: `grep -n -A10 '^pub struct Candidate' crates/kernel/src/gateway/mod.rs` | ✅ la proiezione per candidato | registrato: il **3**, che costruisce il gateway vero e il selettore, con la forma di K44 — un identificativo coniato dal kernel da un catalogo consegnato —; la finestra la legge il 13 |
| **K48** | **quali errori fanno scattare la catena**: Claude Code **non** ripiega sui limiti di frequenza, perché ritenta; il contesto di ADR-0012 li nomina fra ciò da cui la catena protegge, ed è contesto, non decisione; su OpenRouter un limite può essere di un solo modello, e allora un ripiego servirebbe | V alla fonte il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md); D per OpenRouter | — | registrato: il **3**, partendo dalla regola di Claude Code, come chiede D10, e misurando i limiti di OpenRouter |

## La revisione di coerenza e correttezza — 2026-09-29

Chiesta dal proprietario alla chiusura della sessione di prima — *«si continua nella prossima, con una revisione iniziale
della coerenza e correttezza di quanto scritto»* — e fatta **prima** di D10, su `561140e`. Riletti per intero questo file e
il [disegno del 2026-09-04](2026-09-04-knowledge-base-design.md), e i testi di ADR-0016, ADR-0022 e ADR-0024.

| Che cosa si è controllato | Come | Esito |
|---|---|---|
| ogni risposta contro la sua sezione, e contro i K che dice di chiudere | le due tabelle dei buchi, riga per riga | reggono, salvo K4, K20 e K21, che D3 e D5 hanno reso vecchi, e K31, riaperto |
| le risposte fra loro | D3 con D11; D7 con D11; D5 con K36 e con la lista di base di D3; D4 con ADR-0022; D9 con la regola 3 e la pretesa 1.1e del 2026-09-04, e con AUD-004 | due punti di **merito**, D12 e D13; gli altri di forma |
| ogni riga *«verificato»* | ogni comando del file rilanciato, uno per uno | rendono tutti come scritto; il codice non si è mosso da `f830cb9` |
| le fonti | ciò che questo file fa dire a ciascuna, contro la sua riga di [`riferimenti.md`](../../riferimenti.md); dove il file dice di più, riletta alla fonte | tre frasi su Claude Code senza la loro riga: rilette, reggono, con una precisione |
| le tabelle e le sezioni del 2026-09-28 | riga per riga, contro le risposte | le righe superate portano la risposta in coda; *«Che cosa esiste oggi»* e *«Che cosa arriva»* una riga datata |
| che cosa le risposte cambiano | ogni risposta contro il disegno del 2026-09-04, gli ADR, la roadmap, la tracciabilità e la stella polare della GUI | l'elenco qui sotto |

**Che cosa ha trovato.**

| # | Il punto | Specie | Dove sta ora |
|---|---|---|---|
| **RC1** | D4 scrive come **comune** alle due risposte che il programma salva nel suo backup i suoi dati e i router, e che il resto della root è del proprietario; ma nella root stanno anche gli artefatti delle run e le guide, che ADR-0022 mette nel backup del programma, e la sezione di D4 dava la frase per coerente con ADR-0022. La decisione 4 del coordinatore della sessione di prima ha sbagliato qui: ciò che era scritto come comune era una scelta | **merito** | **D12** — ✅ A —, K42; un richiamo nella sezione di D4, nella risposta D4 e in K31 |
| **RC2** | la decisione 1 della stella polare della GUI mette nella rete della Home *«tutto»*, artefatti compresi; D3 tiene fuori dal grafo le zone esterne alla root, e con loro il codice che l'agente vi scrive | **merito** | **D13** — ✅ A —, K43 |
| **RC3** | K4 e K20 dicevano che l'agente scrive solo nella root: vero per il documento, non più dopo D3; K4 lasciava aperta la visibilità dei file pesanti, e K21 li diceva *«fuori dall'indice»*: D5 li tiene come nodi | forma | le tre righe riscritte, col richiamo |
| **RC4** | nella sezione di D3 il permesso della knowledge base, *«concesso per la root»*, si poteva leggere come un sì per sempre, contro il punto 3 di ADR-0016, che D11 riempie | forma | la cella riscritta, col richiamo |
| **RC5** | D9 dice che la fiducia a una zona resta; D11 che alla fine della sessione *«cadono i suoi sì»*. Non si contraddicono — la fiducia non concede triple, e Claude Code la tiene su disco —, ma nessuna riga lo diceva | forma | una riga nuova in *«Che cosa ne segue»* di D9 |
| **RC6** | D9 faceva dire a Claude Code che prima della fiducia *«le regole di permesso … non valgono»*: alla fonte non valgono quelle che **concedono**, e quelle che negano valgono sempre | forma | la riga della fonte in D9, e [`riferimenti.md`](../../riferimenti.md) |
| **RC7** | D3 dà a una zona *«le regole della zona»* senza dire dove stanno né chi le scrive, e D5 vuole il privato cambiato dal solo proprietario | caso nuovo | **K41**, chiuso nel principio dallo stato dell'arte |
| **RC8** | tre frasi su Claude Code usate da D9 e da D11 non avevano la loro riga in [`riferimenti.md`](../../riferimenti.md): la sessione come conversazione legata a una cartella, il sì per sempre ai comandi e ai domini, la fiducia alla cartella | forma | rilette alla fonte il 2026-09-29, e aggiunte |
| **RC9** | la testa di D10 e K16 dicono *«posta il 2026-09-29»*, e D10 non era ancora posta | forma | vero da quando si pone, lo stesso giorno: nessuna correzione |

### Che cosa le risposte cambiano — l'elenco per il disegno

Il disegno di questa revisione, nella sua sessione, lo scrive così: richiami datati al disegno del 2026-09-04; rimandi in
testa agli ADR, ciascuno riletto contro i fratelli — gotcha #59 —, con la voce della §5 del compendio per ogni ADR che ne
riceve uno; righe riscritte nella roadmap, nella tracciabilità e nella stella polare della GUI. ⚠️ **È l'elenco del
2026-09-29:** le risposte a D10, D12 e D13 lo allungano.

| Dove | Che cosa cambia | Da |
|---|---|---|
| il disegno del 2026-09-04: le premesse, la risposta 1, §1.1a | non più *«un archivio unico»*: una cartella qualsiasi, e la root arriva dalla configurazione — ADR-0034 | il documento |
| la risposta 3, §1.1b, §1.3 | non più *«solo il nostro assistente»*: due attori, e il proprietario scrive da fuori con qualunque strumento; cade l'esclusione degli *«altri strumenti che leggono o scrivono la cartella»* | il documento |
| le risposte 4 e 10, §4.1 | l'agente cerca anche nel livello strutturale: un file che nessun router punta si trova lo stesso, con un costo in più | il documento |
| §1.1c e la decisione 15 | il «router dell'ambito» del piano 0: per una zona di lavoro è la sua scheda progetto; per la knowledge base resta il router master, e l'area la sceglie l'agente leggendolo — il primo passo del documento —, salvo che la run nasca già da un'area. ⚠️ La sezione di D3 dice *«per la knowledge base la chiave è l'area»*: vale solo in quel caso | il documento, D3 |
| la risposta 9, §2.3 regola 4 | lo spazio designato non è più uno: la knowledge base e le zone di lavoro aperte, D3; da una zona alla root l'agente copia, e il file entra con la provenienza della regola 4 — K20 | D3 |
| §1.4 | il riconciliatore corregge da solo i due casi certi, e nel dubbio segna rotto e chiede — D6; una modifica del proprietario da fuori non si approva, perché è sua: il riconciliatore riallinea lo stato derivato, e una skill cambiata resta ad AUD-004 | il documento, D6 |
| §1.4, la riga del privato, e la decisione 13 | chiusa: rumore e privato, D5; le zone, K41 | D5 |
| §2.2, la cartella su disco | non più *«nel backup»* per intero: lo decide D12 | D4, D12 |
| §2.2, i trigger | il sorvegliante più la scansione all'avvio; il meccanismo sa dire *«ho perso eventi, riscansiona»*, e l'indicizzazione in corso si dichiara prima — ADR-0019, K29 | il documento, K8, K9 |
| §2.3 regola 3, §1.1e | per il file-guida di una zona l'approvazione è la fiducia alla cartella, e l'impronta si scrive a ogni caricamento | D9 |
| §4.2, l'indice | ogni file è un nodo; le frecce di cartella, di area e dei link, D8; il segnale rotto anche per il file chiave perso, D6, e per il link verso un file che non c'è, D8; il rumore resta nodo, D5 | il documento, D5, D6, D8 |
| §4.3, il pannello | la ricerca testuale mentre si scrive, coi filtri per tipo e per cartella; la griglia accanto al grafo | il documento |
| §4.4, la prima metà del 6 | il livello strutturale, con la ricerca testuale senza modello; la seconda metà resta per la somiglianza | il documento |
| §6.3 e §7 | *«nessuna fonte esterna»* non regge più: l'idea dei router viene dalla guida ARMS, datata il giorno del disegno | la sezione sulla guida ARMS |
| ADR-0009 | i file-guida di una zona si caricano per fiducia alla cartella, con l'impronta di ogni caricamento nel giornale; i trigger col «riscansiona» | D9, K8, K9 |
| ADR-0011 | la «sessione» della contabilità è quella di D11 | D11 |
| ADR-0014 | da rileggere con D9: il passaggio esplicito e giornalato, per il file-guida di una zona, è la fiducia alla cartella | D9 |
| ADR-0016 | la «sessione» del punto 3 è quella di D11, e al riavvio del core finisce — RR3; un rimando: il permesso lo chiede chi agisce per un modello o invoca una funzione, e la manutenzione deterministica del programma sulla sua cartella segue un'impostazione — D14 | D11, D14 |
| ADR-0022 | un **ADR nuovo** supera, per i file della root, le righe degli artefatti e delle guide e la conseguenza sulla base di conoscenza: il programma salva il giornale, la configurazione e i router, e dice al backup che cosa resta fuori; la root la salva il proprietario. La voce nuova nella §5 del compendio, e il rimando in testa ad ADR-0022 | D12 |
| ADR-0024 | l'ambito di una zona si chiude con la sessione — K35; il limite di dimensione resta da fissare — K21 | D3, D11 |
| ADR-0025 | il livello 2 nega i percorsi privati — K36 | D5 |
| la stella polare della GUI, decisione 1 | un richiamo datato: la rete ha tutto ciò che sta **nella root**; un artefatto di una zona esterna sta nell'anello, e nella rete attraverso la scheda della sua zona | D13 |
| la stella polare della GUI, la voce registrata del **selettore del modello** e la riga 13 della barra | decisa: il modello si sceglie a mano accanto al pulsante di invio, per la sessione o come default, come Claude Desktop, e ogni risposta porta il nome del modello che l'ha data | D10 |
| [design/09](../../design/09-l0-fisico.md), la riga della cartella della knowledge base | la cartella della knowledge base non è più nel backup del programma: ci sono i router | D12 |
| la spec del sotto-progetto 1: §4, la porta `filesystem`, e §6.6, il permesso | la porta cresce — chiudere, escludere, spostare, cancellare —, K45; la risorsa di un permesso su un percorso scelto a runtime, K44; il permesso porta la sessione, RR2 | RR2, RR5, RR6 |
| il codice: `permission.rs`, `record.rs`, `parameters.rs`, `filesystem.rs` | la sessione nel record del permesso e il record di fine sessione, RR2; i due tempi, RR4; la risorsa, K44; la porta, K45 | il 3 e il 5 |
| il codice: `gateway/mod.rs` | il candidato con un nome scelto a runtime e la sua finestra, K47 | il 3 |
| le funzioni della knowledge base nel registro | la funzione che rende leggibile è irripetibile — RR8 | D5 |
| `roadmap.md` | il **6**: la cella dice ancora *«archivio unico»*, e la prima metà guadagna il livello strutturale e il riconciliatore con la sua impostazione, D14; il **5**: la porta `filesystem` vera, le zone, il livello 2 che nega il privato — K23, K35, K36, K44, K45; il **3**: la run nel giornale e la sessione di D11, con la fine come record e il riavvio — RR1–RR4; il **3** anche il selettore del modello, la catena nel gateway con un modello per richiesta e la compressione spenta, D10; il **13**: il «riscansiona», la fiducia di D9 e la proiezione per candidato, D10; l'**11**: il backup di D12; il **10**: le cartelle dati e la cartella nascosta — K1, K30 | le risposte |
| `tracciabilita.md` | `Multi-repo/multi-progetto` e `Mappa del progetto`: le zone e la scheda progetto, D3; `Git e gestione branch`: la zona; `Collezioni e knowledge base`: la forma del documento; `File watching e awareness del progetto`: sorvegliante e scansione; `Sessioni multiple`: la sessione di D11; `Selettore di modello per compito`: il selettore della sessione e il modello nella definizione di un sotto-agente, D10; `Backup della KB indipendente dall'app`: la root nei backup del proprietario, i router in quello del programma, D12 | le risposte |

### La prova alla radice — chiesta dal proprietario, 2026-09-29

Il proprietario, prima che la revisione ponesse D12: *«hai controllato che quanto scritto nella documentazione combaci e
rientri nell'architettura del programma al cuore e non come feature aggiunta? idem vale per la sessione […] è integrata in
modo coerente con quanto è scritto nei permessi e si integra con quanto già fatto o si farà alla radice?»*. La revisione aveva
confrontato le risposte coi testi degli ADR, **non col codice** del kernel: questa è la prova che mancava, letta su `c698f37`.

| # | Il punto | Che cosa c'è nel codice | Esito, e che cosa ne segue |
|---|---|---|---|
| **RR1** | la sessione è la run radice coi suoi discendenti, D11 | nessuna run nel kernel: il giornale conosce solo `StepId`, e il filo lo dichiara — `grep -n 'RunId' crates/kernel/src/wire/ipc.rs` | ✅ **al cuore**: ADR-0011 decide già la gerarchia passo → run → run padre, e la sessione ne è la cima, non un oggetto accanto. Il 3 la scrive nel giornale |
| **RR2** | un sì vale per la sessione | `permission::is_granted` scorre tutto il giornale, e il record del permesso porta strumento, risorsa e operazione, nient'altro — `grep -n -A6 '^pub struct PermissionDetail' crates/kernel/src/record.rs` | ✅ **al cuore**, nella forma che il kernel usa già: il record del permesso guadagna la sessione su un **indice nuovo e facoltativo** — ADR-0036, regola 3: i byte congelati restano uguali finché è vuoto —; la fine della sessione è **un record del giornale**, scritto dal core con la sua causa; e *«quali sì valgono ora»* resta una **proiezione del giornale**, come dice `permission.rs` |
| **RR3** | il riavvio del core | `time.rs`: le decisioni usano solo il tempo **monotono**, *«grant validity windows»* comprese, e mai l'ora del mondo — `grep -n 'grant validity windows' crates/kernel/src/time.rs` | ⚠️ **D11 non lo diceva, e la radice lo decide**: il tempo monotono non attraversa un riavvio, quindi il core non può misurare un'inattività che lo attraversa; **una sessione aperta finisce al riavvio**, e la riconciliazione all'avvio ne scrive la fine con la causa «riavvio». È anche ADR-0016, *«non vale domani»*. Scritto in D11 |
| **RR4** | i due tempi di D11 | `Parameters` ha quattro campi, e aggiungerne cambia la firma di `new` — `grep -n -A6 '^pub struct Parameters' crates/kernel/src/parameters.rs` | ✅ **al cuore**: due parametri consegnati, ADR-0034; l'attrito della firma è voluto |
| **RR5** | un permesso su una zona, e su `.<nomeapp>/` | la risorsa di `Permission` è un `&'static str`, confrontato carattere per carattere, e il motivo scritto è I6: un nome arrivato da fuori, dentro una decisione, è testo non fidato — `grep -n 'pub resource' crates/kernel/src/permission.rs` | ❌ **oggi non si può esprimere**: D3 apre zone su percorsi scelti a runtime, e D7 vuole una tripla su una **cartella** che copra i file dentro. ⚠️ La deduzione di D7, *«una tripla su una cartella copre i file che contiene»*, **il codice non la fa**. La forma coerente con la radice: la risorsa diventa un **identificativo coniato dal kernel** quando apre un ambito, come `StepId`, e se un file sta nell'ambito lo decide chi implementa la porta, come `filesystem.rs` dice già. Resta I6, resta la tripla. **K44** |
| **RR6** | chiudere una zona, escludere il privato, spostare, cancellare | la porta `filesystem` ha cinque metodi: dichiarare un ambito, conservare, ripristinare, leggere, scrivere — `grep -n '^    fn ' crates/kernel/src/ports/filesystem.rs` | ❌ **oggi non si può**: niente chiusura — K35 —, niente esclusioni — il privato di D5, che *«la porta rifiuta»* —, niente spostare né cancellare — le CRUD della knowledge base, K20, la cancellazione morbida. E il confronto col privato — la sintassi di `.gitignore`, le maiuscole di Windows — lo fa chi implementa la porta, perché il kernel non interpreta i percorsi. Il contratto della porta cresce: è una porta del kernel, quindi un richiamo datato alla spec del sotto-progetto 1, del proprietario quando si fa. **K45** |
| **RR7** | il riconciliatore scrive i router da solo, D6 | `registry::invoke` chiede `is_granted` a chi **invoca** una funzione, e la porta dei file non chiede niente: il riconciliatore non è né l'uno né l'altra | ❌ **nessuna risposta dice con quale autorità scrive**: lavora all'avvio e sugli eventi del sorvegliante, fuori da ogni sessione, mentre il sì di D7 vale dentro una sessione e ADR-0016 fa chiedere le scritture. **K46**, **D14** — ✅ A |
| **RR8** | un cambio che rende leggibile qualcosa chiede conferma con ogni preset, D5 | `EffectClass`, e il preset `autonomo` di ADR-0016, che chiede per gli effetti **irripetibili** | ✅ **al cuore, senza regole nuove**: la funzione che rende leggibile è **irripetibile** — un contenuto letto può essere già andato al modello, e non si disfa —, quindi chiede con ogni preset. L'elenco qui sopra non chiede più un rimando in ADR-0016 per questo |
| **RR9** | la fiducia di D9 | niente ancora; e `filesystem.rs`: il kernel non sa dire se due percorsi sono lo stesso file | ✅ **al cuore**: un record del giornale e una proiezione, come i permessi; la chiave è il percorso **come lo dà la piattaforma**, e se due grafie mancano la stessa zona la fiducia si richiede — l'errore cade dal lato chiuso |
| **RR10** | il fallback di D10 | `gateway::resolve` risolve la catena **per chiamata**, coi vincoli di ADR-0012 — `grep -n 'THE CHAIN IS DELIVERED PER CALL' crates/kernel/src/gateway/mod.rs` | ✅ **al cuore**: la catena la fa già il kernel, e la risposta A di D10 ci si appoggia |
| **RR11** | la knowledge base intera | router, indice, scansione e riconciliatore nel 6; sorvegliante, registro delle guide e proiezione nel 13; zone, checkpoint, permessi e sessione nei meccanismi del kernel | ✅ **nessuna funzione a parte**: ogni pezzo sta su un meccanismo deciso; dove il meccanismo non basta ancora, lo dicono RR5, RR6 e RR7 |
| **RR12** | il modello scelto col selettore, D10 | il nome di un `Candidate` del gateway è un `&'static str`, per I6 — `grep -n 'pub model' crates/kernel/src/gateway/mod.rs` | ❌ **oggi non si può esprimere**: il catalogo di OpenRouter non è noto a tempo di compilazione, e un modello scelto a runtime non è un `&'static str`. La forma coerente con la radice è quella di RR5: un identificativo coniato dal kernel da un catalogo consegnato, ADR-0034. **K47**. ⚠️ Aggiunta del 2026-09-29, alla risposta di D10 |
| **RR13** | il controllo prima di mandare, D10 | `Candidate` porta modello, locale, ritenzione e prezzo, e nessuna finestra — `grep -n -A10 '^pub struct Candidate' crates/kernel/src/gateway/mod.rs` | ❌ **manca un campo**: senza la finestra il gateway non sa se la proiezione entra. **K47**. ⚠️ Aggiunta del 2026-09-29, alla risposta di D10 |

## Le domande, una per volta

Ogni domanda si pone **sola**, col contesto, A/B, i cinque criteri e il consiglio; la risposta va nella tabella *«Le risposte
del proprietario»*, e si committa. ⚠️ **L'elenco è stato rifatto il 2026-09-28** dopo il documento del proprietario, che
risponde a D1 e a buona parte delle domande di prima; l'elenco di prima sta nel commit `e714720`.

| # | Domanda | Chiude |
|---|---|---|
| **D1** | la forma: tre posti o uno solo | ⛔ **respinta**: la risposta è il documento del proprietario |
| **D2** | **lo storico**: il giornale e la copia valgono solo per le azioni dell'agente? | la riga *«niente storico»* del confronto |
| **D3** | **le zone**: il coding su una repo fuori dalla knowledge base, e se una zona di lavoro entra nel grafo e nella ricerca | K28, K13, K3 |
| **D4** | **dove vivono** indici e router — la decisione aperta 1 del proprietario — e i dati del programma | K1, K30, K31, K32 |
| **D5** | **le esclusioni**: quante specie, e chi scrive il file delle regole | K26, K27 |
| **D6** | **il file chiave che il riconciliatore non ritrova** | K24, K25 |
| **D7** | **l'agente che scrive senza chiedere** | K33 |
| **D8** | **i link markdown come archi** — la decisione aperta 4 del proprietario | — |
| **D9** | **i file-guida delle repo**: mai iniettati da soli; guida solo se importati e approvati | K15, con AUD-004 |
| **D10** | **la proiezione quando il modello cambia** per un fallback | K16, K39, K40 |
| **D11** | **la sessione**: che cos'è, e se scade col tempo — posta **prima** di D7, che ne dipende | K37 |
| **D12** | **il backup**: il programma salva anche gli artefatti delle run e le guide che stanno nella root, come ADR-0022, o solo i suoi dati e i router, come D4 — trovata dalla revisione, posta **prima** di D10 | K42, e riapre K31 |
| **D13** | **gli artefatti di una zona esterna**: anche nella rete della Home, o solo nell'anello — trovata dalla revisione, posta **prima** di D10 | K43 |
| **D14** | **il riconciliatore**: le correzioni certe si scrivono da sole, come manutenzione del programma, o aspettano il sì del proprietario — trovata dalla prova alla radice, posta **prima** di D10 | K46 |
| — | registrati col chiusore, senza domanda salvo che il proprietario la chieda: K11, K12, K17, K19, K21, K22, K23, K29, K34, K35, K36, K38, K41, K44, K45, K47, K48 | |

### D1, posta il 2026-09-28

⛔ **Respinta dal proprietario lo stesso giorno**, con la risposta nel suo documento. Resta qui com'era posta, perché la
tabella delle risposte la nomina.

**Che cos'è, a parole semplici.** Oggi non è deciso dove stanno i file. Il programma scrive i suoi dati nella cartella da cui
parte, e le repo dei progetti non sono mai state discusse. Il 4 settembre la risposta 1 diceva *«Progetti, note, tutto
dentro»*: non era chiaro se «progetti» volesse dire le note **sui** progetti o le repo **stesse**.

| | **A — tre posti, ognuno col suo compito** | **B — un posto solo, tutto dentro** |
|---|---|---|
| com'è | i **dati del programma** — giornale, disposizione, indici, pesi, segreti — in una cartella che gestisce il programma · la **knowledge base** in **una** cartella del proprietario, che scrivono solo l'assistente e il proprietario dal pannello · le **repo** restano dove sono, e nella knowledge base c'è una **scheda progetto** per ciascuna — che cos'è, dov'è, note, decisioni. Quando l'assistente lavora su una repo, la repo è un **ambito di lavoro** a sé (ADR-0024), e la scheda è il suo router | una cartella sola, con dentro la knowledge base **e** le repo |
| «progetti dentro» | **sì, la conoscenza** del progetto: scheda, note, decisioni. I file della repo stanno fuori | sì, anche i file della repo |
| costo | tre posti da capire; nella mappa un nodo «progetto» che punta **fuori** — e K13 si decide così: un collegamento esce solo verso un ambito dichiarato | la decisione 3 è falsa dal primo giorno — git, gli editor e altri agenti scrivono nelle repo; il sorvegliante scatta a ogni build; indice, backup e checkpoint vogliono **regole di esclusione** in quattro posti da tenere allineati; il `CLAUDE.md` di una repo clonata sembra una skill del proprietario |
| che cosa si rifà dopo | niente: il formato della scheda si disegna col 6 | spostare le repo fuori, un giorno, vuol dire riscrivere tutti i collegamenti |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | poggia su fatti letti oggi: le repo mai discusse, la decisione 3, ADR-0024 che nasce per gli ambiti e fa convivere git, il daemon senza casa | contraddice la decisione 3, verificata |
| coerenza | riusa l'ambito di ADR-0024, la tripla di ADR-0016 — il cui esempio è proprio `(file, ~/progetti/x, lettura)` —, il confine di ADR-0014 e la separazione per natura di ADR-0022 | un secondo modo di trattare file scritti da altri, dentro l'archivio che doveva essere solo nostro |
| debito | dichiarato: il formato della scheda progetto, al 6 | quattro liste di esclusione che devono restare uguali — la forma del gotcha #68 |
| stato dell'arte | non serve: la scelta poggia sulle decisioni del repository | idem |
| proporzione | nessun meccanismo nuovo: un tipo di nodo e un attributo, il percorso | un macchinario di esclusione per un problema che A non ha |
| di chi è | **del proprietario**: rilegge la sua risposta 1 | idem |

**Verificato, dedotto, assunto.** **Verificati** coi comandi della tabella *«Che cosa esiste oggi»*: le repo mai discusse, la
decisione 3, ADR-0024 e ADR-0016, il daemon senza casa. **Dedotti**, non misurati: i costi di B — il sorvegliante che scatta a
ogni build, il `CLAUDE.md` che sembra una skill. **Assunto**: che il proprietario lavorerà sulle repo **anche** con altri
strumenti — git, un editor, Claude Code —, come fa su questo repository.

**Il consiglio: A.** È l'unica forma in cui la decisione 3 resta vera, e riusa ciò che è già deciso invece di aggiungere regole.

### D2, posta il 2026-09-28

**Che cos'è, a parole semplici.** Il documento dice *«non serve uno storico»*. Oggi il kernel ne tiene già uno, ma **solo di
ciò che fa l'agente**: prima di ogni azione ne scrive l'intenzione e dopo l'esito, nel giornale (ADR-0007) — è ciò che fa
ripartire il programma dopo un crash, costruito dal sotto-progetto 1 —; e prima che l'agente cambi un file dentro un ambito,
ne tiene una **copia** (ADR-0024). I cambi del proprietario da fuori non li traccia nessuno.

| | **A — giornale e copia restano, solo per l'agente** | **B — niente copia nella knowledge base** |
|---|---|---|
| com'è | nessuno storico dei cambi del proprietario, nessun versioning, nessun sync; per le azioni dell'agente il giornale e la copia già decisi, e un suo errore si annulla | come A, ma l'agente cambia e cancella i file senza copia |
| costo | lo spazio delle copie dei soli file che l'agente tocca — non della root, anche se la root è il PC intero —, potate con la logica di ADR-0018 | un ADR nuovo che superi ADR-0024 per questo caso; un errore dell'agente che sovrascrive o cancella è perso |
| che cosa si rifà dopo | niente | rimettere la copia vorrebbe dire costruire il pezzo che oggi è già deciso |

⛔ **Il giornale resta in ogni caso**: senza, il programma non riparte dopo un crash, e ADR-0007 sta fra le decisioni che la
§7 del compendio dice non rilitigabili senza un ADR nuovo. La domanda riguarda solo la **copia**.

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | ADR-0007 e ADR-0024 letti; il giornale esiste nel codice, `FileJournal` in `crates/platform/src/journal.rs` | toglie una difesa decisa |
| coerenza | nessun ADR cambia; la rete che il documento chiede per la cancellazione copre ogni modifica dell'agente | un ambito senza copia, diverso da tutti gli altri |
| debito | nessuno | un buco che si scopre al primo errore dell'agente |
| stato dell'arte | non serve: sono decisioni del repository | idem |
| proporzione | la copia costa solo dove l'agente scrive | risparmia poco spazio |
| di chi è | **del proprietario** | idem |

**Verificato, dedotto, assunto.** **Verificati**: ADR-0007 e ADR-0024, e il giornale nel codice. **Dedotto**: che *«niente
storico»* nel documento parli dei cambi del proprietario e non delle azioni dell'agente — per questo è una domanda e non una
correzione.

**Il consiglio: A.** *«Niente storico»* resta vero per i file del proprietario; per l'agente c'è l'annulla.

### D3, posta il 2026-09-28 e riformulata lo stesso giorno

⚠️ **La prima forma** chiedeva se la root fosse il confine di tutto l'assistente, e consigliava di sì. **Il proprietario ha
risposto con una domanda:** *«e quando si fa coding su una repo esterna alla kb? ricordiamo che possiede una parte di coding
agentico sitle claude desktop»*. Ha colto una lacuna vera: col confine unico una repo fuori dalla root sarebbe
irraggiungibile, e il coding stile Claude Desktop — si apre una cartella qualsiasi e l'agente lavora lì — non funzionerebbe.
La prima forma sta nel commit `9bdbb59`.

**Il modello che ne segue: due specie di zona.**

| | La knowledge base | Una zona di lavoro |
|---|---|---|
| che cos'è | la root del documento: **una** per installazione | una cartella che il proprietario apre per lavorarci, come in Claude Desktop — una repo, anche **fuori** dalla knowledge base |
| quanto dura | sempre | la sessione |
| mappata e indicizzata | sì: router, livello strutturale, grafo | la domanda qui sotto |
| l'agente legge e scrive | solo dentro | solo dentro |
| la copia prima delle modifiche dell'agente | sì, D2 | sì, D2 |
| il permesso | l'ambito della root c'è sempre, dalla configurazione; dentro, il preset di ADR-0016 — col default le letture procedono e le scritture chiedono, e un sì vale la sessione, D11. ⚠️ **Richiamo del 2026-09-29, revisione:** diceva *«concesso per la root»*, che si poteva leggere come un sì per sempre, contro il punto 3 di ADR-0016 | concesso all'apertura, per la sessione — ADR-0016, e la sessione è quella di D11 |
| le regole di privacy | una **lista di base comune** a tutte le zone — `.env`, chiavi, `.ssh` —, più il file della root | la stessa lista di base, più le regole della zona — dove stanno, e chi le scrive: K41 |

Fuori da tutte le zone l'agente non legge e non scrive: la porta risponde `OutsideScope`. E la lista di base comune toglie il
costo che la B della prima forma aveva: una zona fuori dalla root **non** resta senza regole.

**Che cosa esiste già.** La porta `filesystem` accetta più percorsi in `declare_scope`; ADR-0024 nasce proprio per gli ambiti
di lavoro, e nomina il coding fra chi scrive; ADR-0016 dà il permesso per percorso e per sessione. È anche il modo in cui
lavora Claude Code, in questa stessa sessione: una cartella di lavoro, e i permessi chiesti.

✅ **Verificato alla fonte il 2026-09-29**, nella documentazione di Claude Code — la provenienza in
[`riferimenti.md`](../../riferimenti.md): l'agente vede la cartella di lavoro e le sottocartelle, e i file altrove **col
permesso**; trova il codice **cercando sul posto**, file per nome e contenuto per espressione regolare, e la pagina **non
nomina alcun indice**; prima di modificare un file ne tiene una copia, separata da git — la forma di ADR-0024 e di D2. ⚠️ La
porta invece sa **aprire** una zona e non chiuderla: K35, registrato.

⚠️ **Una repo DENTRO la root non cambia con nessuna delle due risposte**: è già nel livello strutturale, meno ciò che le
esclusioni tolgono — K3. La domanda riguarda una zona **fuori** dalla root.

**Una conseguenza per il 13 — dedotta.** Il router di una zona di lavoro è la sua **scheda progetto** nella knowledge base:
il «router dell'ambito» del piano 0 del 2026-09-04 torna, per le zone di lavoro, nella forma della decisione 15. Per la
knowledge base la chiave è l'**area**. Il kernel le riceve entrambe come chiavi **opache**. Se il `CLAUDE.md` di una repo si
legga da solo, come fa Claude Desktop, lo decide D9.

**La domanda: una repo aperta come zona di lavoro entra nella knowledge base — nel grafo e nella ricerca?**

| | **A — no: ci si lavora come in Claude Desktop** | **B — sì: diventa una seconda root della knowledge base** |
|---|---|---|
| com'è | l'agente cerca nei file della repo quando serve, senza un indice fisso; nella knowledge base c'è una **scheda progetto** che punta alla cartella — che cos'è, dov'è, note, decisioni — e fa da router della zona | la repo si indicizza e si sorveglia finché è aperta; i suoi file compaiono nel grafo e nella ricerca |
| costo | nel grafo la repo è un nodo, la scheda, e non i suoi file | scansioni e sorveglianza su più alberi, build comprese; la knowledge base diventa «a più root», contro la root unica del documento; chiusa la zona, i suoi nodi spariscono dal grafo |
| che cosa si rifà dopo | niente: indicizzare una zona si può aggiungere un giorno | tornare a una root sola vuol dire togliere un pezzo costruito |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | la porta accetta più zone; il modello è quello di Claude Code, in uso in questa sessione | l'indice a più root non esiste, ed è dedotto |
| coerenza | la knowledge base resta a root unica, come nel documento | la cambia |
| debito | il formato della scheda progetto, al 6 | l'indice e la sorveglianza a più root |
| stato dell'arte | ✅ **verificato alla fonte il 2026-09-29**: è il modo in cui la documentazione di Claude Code descrive il suo lavoro — la ricerca sul posto, nessun indice nominato | non cercato alla fonte: quella letta descrive A |
| proporzione | il minimo | un indice per un caso che la ricerca sul posto già copre |
| di chi è | **del proprietario** | idem |

**Verificato, dedotto, assunto.** **Verificati**: la porta `filesystem`, ADR-0024 e ADR-0016; e, alla fonte, come lavora
Claude Code — la cartella di lavoro, la ricerca sul posto, la copia prima di modificare. **Dedotti**: la scheda come router
della zona, e i costi di B. **Assunto**: che la ricerca sul posto **basti** per lavorare su una repo del proprietario — la
fonte dice come lavora Claude Code, non che basti su ogni repo: lo misura il 5.

**Il consiglio: A.** Il coding funziona come in Claude Desktop, la knowledge base resta una, e il progetto ci entra con la sua
scheda.

### D4, posta il 2026-09-29

**Che cos'è, a parole semplici.** Il programma costruisce due cose sopra la cartella: l'**indice** — l'elenco completo dei
file, coi metadati e il testo per la ricerca, che una scansione rifà da sola — e i **router** — le aree e i file chiave, che
portano **le scelte del proprietario** e non si rifanno senza rifare il setup guidato, K32. Il proprietario propone di
metterli **insieme** in una cartella nascosta `.<nomeapp>/` alla root. E ci sono i dati **del programma** — il giornale, la
disposizione dei pannelli, la configurazione che dice dov'è la root —, che oggi finiscono nella cartella da cui parte il
daemon, K1.

**Che cosa esiste già.**

| | Che cosa dice | Dove, e il comando |
|---|---|---|
| ADR-0022 | la separazione **per natura**: gli indici **fuori dal backup**, perché rigenerabili; configurazione e guide **nel** backup | `grep -n 'indici ed embedding' docs/adr/0022-*.md` |
| il disegno del 2026-09-04 | l'indice della mappa è *«derivato dalla capacità, tenuto dal core, rigenerabile (ADR-0022: fuori dal backup)»* | la §2.2 |
| il daemon | scrive `journal.redb` e `layout.redb` nella cartella da cui parte, e lo dichiara | K1, il comando della tabella *«Che cosa esiste oggi»* |
| Windows | il punto nel nome **non** nasconde: `.git` porta l'attributo H, `.github` no | K30, `cmd //c "attrib .git"` e `cmd //c "attrib .github"`, rilanciati il 2026-09-29 |
| le cartelle dati per utente | Windows: `%LOCALAPPDATA%`, per utente. Linux, XDG 0.8: `~/.local/share` per i dati, `~/.config` per la configurazione, `~/.local/state` per lo stato che sopravvive al riavvio — il *layout* fra gli esempi —, `~/.cache` per i dati **non essenziali** | lette alla fonte il 2026-09-29, la provenienza in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa arriva.** Il **6** costruisce indice e router; l'**11** il backup; il **10** l'integrazione con l'OS, e con lei
l'installazione.

**Regge crescendo?** La root può essere il PC intero: l'**indice** cresce con lei e cambia a ogni file che cambia; i
**router** restano piccoli.

**In tutte e due le risposte**, e non è una domanda: i dati del programma vanno nella cartella dati per utente del sistema —
`%LOCALAPPDATA%\<nomeapp>\` su Windows, le cartelle XDG su Linux —, in sottocartelle per natura come vuole ADR-0022, e la
configurazione porta il percorso della root: chiude K1. La cartella `.<nomeapp>/` la marca nascosta il modulo di
piattaforma, perché su Windows il punto non basta: chiude K30. Il programma salva nel proprio backup ciò che è **suo** — i
suoi dati e i router —, e il resto della root è dei backup del proprietario: chiude K31.

⚠️ **Richiamo del 2026-09-29, revisione:** l'ultima frase **non** era comune alle due risposte, né coerente con ADR-0022 come dice la
tabella dei criteri qui sotto: nella root stanno anche gli artefatti delle run e le guide, che ADR-0022 mette nel backup del
programma, e *«i suoi dati»* comprende alla lettera i segreti e l'indice, che ADR-0022 tiene fuori. La coppia indice–router
resta decisa; il backup torna al proprietario come **D12** — K42. ✅ **D12, A**, 2026-09-29, delegata allo stato dell'arte: la root è del proprietario, e il suo backup pure.

**La domanda: l'indice sta coi router nella cartella nascosta della root, o nella cartella dati del programma?**

| | **A — separati per natura** | **B — insieme, la proposta com'è** |
|---|---|---|
| com'è | i **router** in `.<nomeapp>/` alla root: seguono la cartella, li salva il backup del proprietario, sono suoi · l'**indice** nella cartella dati del programma, fra i dati rigenerabili, uno per root | router **e** indice in `.<nomeapp>/` alla root |
| costo | se la root si sposta, l'indice si rifà con **una** scansione all'avvio — quella che il documento chiede già —, senza modello e senza token; due posti invece di uno | l'indice, grande e sempre in movimento, sta **dentro la cartella del proprietario**: finisce nei suoi backup e in ogni sincronizzazione che la copre, contro la riga di ADR-0022; e cancellare `.<nomeapp>/` per rifare l'indice cancella anche i **router**, che non si rifanno |
| che cosa si rifà dopo | niente | separarli dopo vuol dire spostare l'indice e cambiare la regola del backup |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | ADR-0022 e la §2.2 del 2026-09-04 letti; le cartelle per utente lette alla fonte; K30 misurato | mette un indice rigenerabile dentro una cartella che sta nei backup, contro la riga *«indici ed embedding»* di ADR-0022 |
| coerenza | separa per natura come ADR-0022, e l'indice resta *«fuori dal backup»* come il 2026-09-04 | una cartella che mescola ciò che si rifà e ciò che non si rifà |
| debito | nessuno: la scansione all'avvio esiste comunque | la regola «di `.<nomeapp>/` si salvano i router, non l'indice», da tenere a mano nel backup dell'11 e in quello del proprietario |
| stato dell'arte | XDG mette i dati **non essenziali** in una cartella a sé, la cache | la stessa fonte: B mette dati non essenziali fra i dati dell'utente |
| proporzione | nessun meccanismo nuovo | risparmia una scansione dopo uno spostamento |
| di chi è | **del proprietario**: è la sua decisione aperta 1 | idem |

**Verificato, dedotto, assunto.** **Verificati**: ADR-0022, la §2.2 del 2026-09-04, il daemon senza casa, il punto che su
Windows non nasconde, e alla fonte le cartelle dati per utente dei due sistemi. **Dedotti**: che l'indice di una root grande
sia grande e cambi spesso — non misurato, lo misura il 6 —; che cancellare la cartella nascosta sia il modo naturale di
«rifare l'indice». **Assunto**: che spostare la root sia raro.

**Il consiglio: A.** I router sono del proprietario e seguono la cartella; l'indice si rifà da solo e non ha motivo di stare
nei suoi backup.

### D5, posta il 2026-09-29

**Che cos'è, a parole semplici.** Il documento mette alla root **un** file di esclusioni, in stile `.gitignore`, che tiene
insieme il **rumore** — `.git`, `node_modules`, i binari, i media pesanti — e il **privato**; ciò che vi sta non entra
nell'indice **e** l'agente non lo legge. Tenerli insieme dà due problemi: un asset 3D escluso come «media pesante» sparisce
dal grafo, mentre la Home deve mostrarlo — K27; e se l'agente può cambiare il file, un file letto con dentro un'istruzione
malevola può convincerlo a togliere un'esclusione e poi a leggere il privato — K26.

**Che cosa esiste già.**

| | Che cosa dice | Dove |
|---|---|---|
| ADR-0014 | il contenuto non fidato **informa, mai autorizza**; un'azione la cui decisione dipende da esso chiede la stessa autorizzazione che chiederebbe se l'utente non l'avesse chiesta | la voce della §5 del compendio |
| ADR-0025 | il livello 1 — i permessi applicativi — **non regge** contro codice eseguito; per ogni comando il minimo è il livello 2, un processo ristretto dall'OS | idem |
| D3 | una **lista di base** comune a tutte le zone — `.env`, chiavi, `.ssh` —, più le regole di ciascuna | la risposta D3 |
| la stella polare della GUI, decisione 1 | nell'anello della Home stanno i file prodotti dalle run, **asset 3D** compresi | `grep -n 'Nell.anello solo' docs/superpowers/specs/2026-09-07-direzione-gui-design.md` |
| Claude Code | la lista «non leggere» sta nelle regole `Read` di negazione dei permessi, con la sintassi di `.gitignore`; valgono per gli strumenti dei file e per i comandi che nominano il file, **non** per uno script che apre i file da sé, e per un blocco valido per ogni processo la pagina rimanda alla sandbox | letta alla fonte il 2026-09-29, la provenienza in [`riferimenti.md`](../../riferimenti.md) |
| Cursor | la lista sta in un file dedicato, `.cursorignore`: blocca l'agente e le menzioni, **non** il terminale né gli strumenti MCP, e la pagina dice che la protezione completa non è garantita | idem |

**Che cosa arriva.** Il **5** esegue comandi, al livello 2; il **4** porta i server MCP, K12; il **7** scrive asset 3D; il
**6** costruisce l'indice.

**Regge crescendo?** Con una specie sola, no. Quando arriva il 7, escludere i media li toglie dal grafo, e non escluderli fa
lavorare l'indice su file enormi. E quando arriva il 5, un'esclusione tenuta **solo** dalla porta dei file non ferma un
comando che legge da sé — lo dicono le due fonti: K36.

**In tutte e due le risposte**, e non è una domanda: le regole del **privato** le cambia **solo il proprietario**, dal
pannello o a mano; l'agente può proporre, e un cambio che rende leggibile qualcosa chiede conferma **con qualunque preset**,
anche `autonomo`, perché la decisione può venire da contenuto non fidato — ADR-0014; e la lista di base non si toglie. Chiude
K26.

**La domanda: il file delle esclusioni distingue due specie — rumore e privato — o una sola, come nel documento?**

| | **A — due specie** | **B — una specie, com'è nel documento** |
|---|---|---|
| com'è | **rumore**: lo scanner non ci entra — niente testo, niente impronta, una cartella non si apre —, ma il file o la cartella restano un **nodo** del grafo, e l'agente lo apre se serve · **privato**: fuori dall'indice e da ciò che l'agente vede, la porta **rifiuta** la lettura, e il confinamento dei comandi nega quei percorsi | ogni riga del file toglie dall'indice **e** dalle letture dell'agente |
| costo | due sezioni da capire nel file; il privato vuole la sua regola anche nel confinamento del 5 | un asset 3D escluso sparisce dal grafo, K27; la `node_modules` di una repo dentro la root diventa illeggibile anche per il debug; e ogni riga, anche di rumore, è un confine di sicurezza: o ogni cambio chiede conferma, o il privato non è protetto |
| che cosa si rifà dopo | niente | dividere dopo vuol dire riclassificare ogni riga del file del proprietario |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | ADR-0014 e ADR-0025 letti; la decisione 1 della GUI; le due fonti lette oggi | contraddice la decisione 1 della GUI appena i media si escludono |
| coerenza | il privato è un confine come gli altri — la porta e il livello 2 —; il rumore è un'ottimizzazione, e non chiede conferme | una lista che è insieme un'ottimizzazione e un confine di sicurezza |
| debito | la regola del privato nel confinamento del 5, registrata: K36 | come mostrare i media, rimandato al primo asset |
| stato dell'arte | Claude Code e Cursor tengono la lista «non leggere» in un posto **dedicato** | contro le due fonti |
| proporzione | due sezioni in un file | un file più semplice, e un problema in più a ogni asset |
| di chi è | **del proprietario**: il file è suo | idem |

**Verificato, dedotto, assunto.** **Verificati**: ADR-0014, ADR-0025, la decisione 1 della GUI, e alla fonte le due
documentazioni. **Dedotti**: che un'esclusione tenuta dalla sola porta non fermi i comandi **da noi** — le fonti lo dicono dei
loro prodotti, e il nostro livello 1 per costruzione non regge contro codice eseguito —; che un binario non abbia testo da
indicizzare. **Assunto**: che due sezioni siano chiare per il proprietario.

**Il consiglio: A.** Il rumore è una questione di ordine, il privato di sicurezza: tenerli insieme rende la sicurezza o
fastidiosa o finta.

### D6, posta il 2026-09-29

**Che cos'è, a parole semplici.** Il riconciliatore del documento ripara i router da solo: toglie la voce di un file
cancellato, e aggiorna il percorso di un file spostato, che riconosce dallo **stesso hash**, l'impronta del contenuto. Ma un
file spostato **e** modificato — cosa normale se il programma era chiuso mentre il proprietario riordinava — ha un'impronta
nuova: il riconciliatore vede «cancellato» più «nuovo», toglie la voce, e un file chiave esce dalla mappa senza che nessuno
lo sappia — K24. E due file identici hanno la stessa impronta, quindi uno spostamento diventa ambiguo — K25.

**Che cosa esiste già.**

| | Che cosa dice | Dove |
|---|---|---|
| il documento del proprietario | *«Un puntatore vecchio è peggio di nessun puntatore»*; il riconciliatore corregge in modo meccanico, senza modello | la sezione *«Coerenza dei router»* |
| ADR-0007 | davanti a un dubbio non risolvibile il sistema si ferma, non indovina | la voce della §5 del compendio |
| ADR-0009 | l'anello di miglioramento: il kernel **propone**, l'utente **approva** | idem |
| ADR-0019 | *«si dichiara prima, non si fallisce dopo»*: il «nessun degrado silenzioso» di ADR-0005 diventa una proprietà del kernel | idem |
| il disegno del 2026-09-04 | il pannello mostra gli **orfani** e i **collegamenti rotti**, perché il sensore li misura | la §4.1 |
| git | riconosce uno spostamento **anche** con modifiche, per somiglianza: di default un «cancellato più nuovo» è uno spostamento se almeno metà del file è rimasta uguale; la sola impronta esatta è il caso `-M100%` | la documentazione di `git diff`, 2.56.0, letta alla fonte il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa arriva.** Il **6** costruisce il riconciliatore; il **13** i trigger, cioè gli eventi della sorveglianza.

**Regge crescendo?** I file chiave sono pochi per costruzione — un indice d'area sta sotto una pagina —, quindi una domanda
per un file chiave perso resta rara; una perdita silenziosa, invece, cresce a ogni riordino.

**In tutte e due le risposte**, e non è una domanda: il riconciliatore resta **deterministico**, senza modello — ADR-0020 —, e
il caso **certo** — la stessa impronta, un solo candidato — si applica da solo, come vuole il documento.

**La domanda: un file chiave che il riconciliatore non ritrova con certezza si toglie dalla mappa, o si segna rotto e si
chiede?**

| | **A — nel dubbio, rotto e una domanda** | **B — si toglie, com'è nel documento** |
|---|---|---|
| com'è | la voce **resta**, marcata **rotta**: l'agente non la segue, il pannello la mostra; il riconciliatore propone i candidati — lo stesso nome altrove, o un contenuto simile, come fa git — e il proprietario sceglie, o conferma che il file non c'è più. Due file identici: rotto, coi due candidati | la voce sparisce; il file resta nel livello strutturale e si trova cercando, ma non è più un file chiave. Per due file identici serve una regola di spareggio |
| costo | una domanda ogni tanto, per un file chiave che si è mosso **e** cambiato; la ricerca dei candidati, nel 6 | un file chiave esce dalla mappa **senza che nessuno lo sappia**, e il primo salto non lo trova più |
| che cosa si rifà dopo | niente | aggiungere la domanda dopo; e i file chiave già persi non si ritrovano |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | ADR-0007, ADR-0009 e la §4.1 del 2026-09-04 letti; git letto alla fonte | l'impronta esatta manca lo spostamento con modifica: è il caso `-M100%` di git |
| coerenza | è la forma già decisa: il sensore trova il rotto, l'anello propone, il proprietario approva | una perdita silenziosa, contro il «nessun degrado silenzioso» di ADR-0019, per analogia |
| debito | la ricerca dei candidati, al 6 | la regola di spareggio per i file identici, e le perdite che nessuno vede |
| stato dell'arte | git riconosce lo spostamento con modifica per somiglianza | contro la stessa fonte |
| proporzione | una domanda rara, su pochi file | nessun lavoro in più, e un difetto che non si vede |
| di chi è | **del proprietario**: corregge il suo documento | idem |

**Verificato, dedotto, assunto.** **Verificati**: il documento del proprietario, ADR-0007, ADR-0009, ADR-0019 e la §4.1 del
2026-09-04; alla fonte, il riconoscimento degli spostamenti di git. **Dedotti**: che lo spostamento con modifica sia comune
quando il programma era chiuso; che le domande restino rare perché i file chiave sono pochi. **Assunto**: che il proprietario
preferisca una domanda a una perdita.

**Il consiglio: A.** È la regola del documento stesso — un puntatore sbagliato è peggio di nessuno —, applicata anche al
puntatore che si perde: un file chiave non esce dalla mappa senza che il proprietario lo sappia.

### D7, posta il 2026-09-29

⚠️ **Il proprietario ha risposto con una domanda, lo stesso giorno:** *«da cosa è definita una sessione? se non c'è una
definizione di sessione nel software anche se ti dicessi "un si per sessione" ci sarebbe un buco. Cosa rappresenta una
sessione nel software? è differente o uguale a quella del lato di coding?»*. La lacuna è vera — K37: la sessione non è
definita da nessuna parte, e D7 poggiava su di lei. **D7 si ripone dopo D11**, che la definisce.

**Che cos'è, a parole semplici.** Il documento vuole che l'agente, quando crea, sposta o modifica un file, aggiorni i router
**nello stesso turno**. Ma col preset di default di ADR-0016, `auto-approva sicuri`, ogni scrittura **chiede**: creare un
file e poi aggiornare il router sono due conferme — K33.

**Che cosa esiste già.**

| | Che cosa dice | Dove, e il comando |
|---|---|---|
| ADR-0016, i preset | col default le letture procedono, le scritture chiedono; con `autonomo` chiedono solo gli effetti irripetibili e ciò che un sensore ferma | `grep -n -e 'auto-approva' -e 'autonomo' docs/adr/0016-*.md` |
| ADR-0016, il punto 3 | un'approvazione vale per la tripla concessa **e per la sessione corrente**: approvare `(file, ~/progetti/x, scrittura)` non concede `~/progetti/y`, e non vale domani | `grep -n 'non si estende' docs/adr/0016-*.md` |
| D2 e D4 | ogni modifica dell'agente ha la sua copia, e si annulla; i router stanno tutti in `.<nomeapp>/` | le risposte |
| ADR-0014 e il disegno del 2026-09-04 | una riga scritta dall'agente dopo aver letto contenuto non fidato porta la provenienza e l'etichetta: **informa, non autorizza** | la regola 3 della §2.3 |
| Claude Code | per le modifiche ai file, la scelta *Yes, and don't ask again* vale **fino alla fine della sessione** | la tabella dei permessi di *Configure permissions*, letta alla fonte il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa arriva.** Il **6** costruisce i router e la promozione dei file chiave; il **4** gli agenti, anche più run insieme.

**Regge crescendo?** Con una conferma per ogni riga dei router, no: più file l'agente tocca, più conferme, e la tentazione è
passare ad `autonomo` per tutto — che allarga il permesso a **ogni** scrittura, non solo ai router.

**In tutte e due le risposte**, e non è una domanda: ogni cambio di un router è giornalato, ha la sua copia — D2 — e si
mostra nel turno; le righe portano la loro provenienza — ADR-0014.

**La domanda: quando l'agente aggiorna un router, chiede ogni volta, o basta un sì per tutta la sessione?**

| | **A — un sì per la sessione** | **B — ogni volta** |
|---|---|---|
| com'è | la tripla `(file, .<nomeapp>/, scrittura)` si concede alla prima modifica di un router, e vale fino alla fine della sessione, com'è già il punto 3 di ADR-0016; dopo, l'agente aggiorna i router nello stesso turno | ogni modifica di un router chiede conferma, anche dopo la prima: i router diventano l'unica risorsa per cui il sì non vale per la sessione |
| costo | una conferma per sessione; le modifiche dopo si vedono nel turno e si annullano, ma non si approvano una per una | due conferme per ogni file che l'agente crea o sposta e promuove; una regola speciale, più stretta di ADR-0016 |
| che cosa si rifà dopo | niente | togliere la regola speciale |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | ADR-0016 letto: il punto 3 lo prevede già; la fonte di Claude Code letta | ADR-0016 letto: serve un'eccezione al punto 3 |
| coerenza | nessuna regola nuova: una tripla come le altre | una risorsa trattata diversamente da tutte le altre |
| debito | nessuno | l'eccezione da scrivere e da mantenere |
| stato dell'arte | è il modo di Claude Code per le modifiche ai file | più stretto delle fonti lette |
| proporzione | il minimo che dà l'aggiornamento nello stesso turno | una conferma in più per ogni file |
| di chi è | **del proprietario**: i router sono le sue scelte | idem |

**Verificato, dedotto, assunto.** **Verificati**: i preset e il punto 3 di ADR-0016, D2 e D4, la regola 3 del 2026-09-04, e
alla fonte la tabella dei permessi di Claude Code. **Dedotto**: che una tripla su una cartella copra i file che contiene — è la
forma dell'esempio di ADR-0016, `~/progetti/x`. **Assunto**: che il proprietario voglia vedere le modifiche ai router, non
approvarle una per una.

**Il consiglio: A.** Dà l'aggiornamento nello stesso turno che il documento chiede, con la regola che esiste già, e ogni
modifica resta visibile e annullabile.

### D11, posta il 2026-09-29 e riformulata lo stesso giorno — prima di D7, che ne dipende

⚠️ **Il proprietario ha risposto alla prima forma — «finisce solo con la run, o scade anche col tempo?» — con una consegna:**
*«come le sessioni moderne delle app moderne stato dell'arte, decision-principles devi seguire»*. È un'**accettazione
condizionata**: la risposta la dà lo stato dell'arte letto alla fonte, coi cinque criteri, e dove lo stato dell'arte urta una
decisione del progetto si **segnala** invece di applicarlo. La prima forma sta nel commit `516dc72`.

**Che cos'è, a parole semplici.** «Sessione» è una parola che il repository usa senza averla mai definita. ADR-0016 dice che
un sì vale *«per la tripla concessa e per la sessione corrente»*, e che *«non vale domani»*; il codice dice di sé che una
sessione non esiste, e un permesso concesso oggi resta concesso per sempre, anche dopo un riavvio. **Chi** la costruisce è
già scritto — il 3, che porta le run —; **che cosa** sia, no.

**Che cosa esiste già.**

| | Che cosa dice | Dove, e il comando |
|---|---|---|
| ADR-0016, il punto 3 | un sì vale per la tripla e *«per la sessione corrente»*; approvare `(file, ~/progetti/x, scrittura)` *«non vale domani»* | `grep -n 'non si estende' docs/adr/0016-*.md` |
| ADR-0011 | una conversazione è una **run** interattiva di lunga durata, e ogni messaggio è un passo; la contabilità *«per messaggio, per sessione, per run agentica e per sub-agente»* è fatta di aggregazioni della stessa gerarchia, passo → run → run padre | `grep -n 'per sessione' docs/adr/0011-*.md`; la voce della §5 del compendio |
| il kernel | *«NOTHING HERE IS SCOPED TO A SESSION»*: nessuna sessione, nessuna revoca; e il registro: un sì concesso sopravvive a un riavvio | i due comandi di K37 |
| il disegno del 2 | il confine di sessione dei permessi è del **3**, con le run | il terzo comando di K37 |
| D3 | una zona di lavoro dura la sessione | la risposta D3 |
| Claude Code | una sessione è **una conversazione**, legata a una cartella, e si riprende; il sì alle modifiche dei file vale fino alla fine della sessione | *How Claude Code works* e *Configure permissions*, lette alla fonte il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa fanno le app di oggi — letto alla fonte il 2026-09-29**, la provenienza in [`riferimenti.md`](../../riferimenti.md).

| Fonte | Che cosa dice |
|---|---|
| OWASP, *Session Management Cheat Sheet* | una sessione ha **due scadenze automatiche**: per **inattività** — da 2–5 minuti per le applicazioni di valore alto a 15–30 per quelle a basso rischio — e **assoluta**, dal momento in cui nasce — da 4 a 8 ore per chi la usa una giornata di lavoro —; più la chiusura **a mano**, con un comando visibile; e le scadenze le fa rispettare il **server**, mai il client |
| NIST SP 800-63B-4, 26 agosto 2025 | la riautenticazione: al livello AAL2 una scadenza complessiva di **non più di 24 ore** e una per inattività di **non più di un'ora**; ad AAL3, 12 ore e 15 minuti; ad AAL1, 30 giorni e nessuna per inattività |
| Claude Code, *Configure permissions* e *How Claude Code works* | una sessione è una **conversazione**; il sì alle modifiche dei file vale **fino alla fine della sessione**, quello a un comando o a un dominio si salva **per sempre** nel repository |
| VS Code, *Manage approvals and permissions*, pagina del 2026-09-16 | un'approvazione vale **una volta**, **per la sessione**, **per lo spazio di lavoro** o **per sempre**, e un comando le **azzera** tutte; la pagina non definisce la sessione |
| Android, *Request runtime permissions* | il permesso *«solo questa volta»* vale finché l'app è in uso e per poco dopo, poi si richiede; e i permessi di un'app non usata per qualche mese si **azzerano da soli** |

**Che cosa ne segue: la definizione.**

| | La sessione |
|---|---|
| che cos'è | la **run** che il proprietario apre — una conversazione, o un compito dell'agente — **coi suoi sotto-agenti**: la stessa cosa sul lato della chat e su quello del coding, come la conversazione di Claude Code e di VS Code |
| quando finisce | alla prima di tre cose: il proprietario la **chiude**, con un comando visibile; passa un tempo di **inattività**; passa un tempo **massimo** da quando è nata — le due scadenze di OWASP e di NIST |
| al riavvio del core | la sessione aperta **finisce**, e la riconciliazione all'avvio ne scrive la fine con la causa «riavvio»: le decisioni del kernel usano solo il tempo monotono, che non attraversa un riavvio — `time.rs`. ⚠️ **Aggiunta del 2026-09-29, dalla prova alla radice**, RR3 |
| chi la fa rispettare | il **core**, mai la GUI: è il «server» di OWASP, e lo stato vive solo nel core, I1 |
| i due tempi | **parametri consegnati** al kernel, ADR-0034, misurati con l'orologio iniettabile di ADR-0021; i valori li sceglie il 3, e il riferimento, per analogia, è il livello AAL2 di NIST — un'ora di inattività, 24 ore al massimo —, perché l'assistente scrive file ed esegue comandi |
| che cosa si porta via | alla fine della sessione cadono i suoi sì — ADR-0016, punto 3 — e si chiudono le sue zone di lavoro, D3; la run invece resta nel giornale e si riprende, e ripresa chiede di nuovo |
| che cosa si vede | i sì attivi di una sessione, con la revoca: il seguito di ADR-0016, e il comando che li azzera di VS Code |

⚠️ **Dove lo stato dell'arte urta il progetto, e si segnala senza applicarlo — K38.** Claude Code e VS Code offrono anche un sì
**oltre** la sessione — per lo spazio di lavoro, o per sempre, con un comando che li azzera —, e Android azzera da solo i
permessi non usati. ADR-0016 dice il contrario — *«un'approvazione non si estende»* — e fra le sue alternative non ha mai
valutato la durata: ha scelto la tripla contro lo strumento, e basta. Riaprirlo vuol dire un ADR nuovo, ed è del proprietario.

**I cinque criteri, sul risultato.**

| Criterio | La definizione |
|---|---|
| correttezza verificata | cinque fonti primarie lette oggi; ADR-0016, ADR-0011 e il codice letti |
| coerenza | nessun ADR cambia: la sessione riempie la parola che ADR-0016 usa, con la run di ADR-0011; la scadenza vive nel core, I1; i tempi sono consegnati, ADR-0034, e l'orologio è iniettabile, ADR-0021 |
| debito | i valori dei due tempi e la revoca, al 3, com'era già assegnato; K38, registrato |
| stato dell'arte | le due scadenze di OWASP e di NIST, e la sessione come conversazione di Claude Code e di VS Code |
| proporzione | due parametri e una chiusura a mano: nessun meccanismo che le fonti non abbiano |
| di chi è | **del proprietario**, che l'ha delegata allo stato dell'arte con una condizione |

**Verificato, dedotto, assunto.** **Verificati**: le cinque fonti, lette il 2026-09-29; ADR-0016, ADR-0011, `permission.rs`,
`registry.rs` e il disegno del 2. **Dedotti**: che la sessione dei permessi sia la conversazione anche da noi — le fonti degli
agenti lo fanno, e ADR-0011 mette la sessione nella gerarchia delle run —; che le scadenze delle sessioni di accesso valgano
anche per i sì dati a un agente — per analogia, e il permesso *«solo questa volta»* di Android è della stessa famiglia.
**Assunto**: che il livello AAL2 sia il riferimento giusto per i valori; lo verifica il 3.

**Dove va, perché la definizione non resti solo qui.** Il proprietario, rispondendo a D7, ha chiesto: *«la sessione è stata
integrata nei permessi esistenti?»*. **Non ancora**: oggi vive solo in questo file.

| Dove | Che cosa | Chi |
|---|---|---|
| ADR-0016 | un rimando datato in testa: la «sessione» del punto 3 è quella di D11; nessuna riga superata | il disegno di questa revisione, riletto contro i fratelli — gotcha #59 |
| ADR-0011 | un rimando datato: la «sessione» della contabilità è la stessa | idem |
| la voce di ADR-0016 nella §5 del compendio | la riga del rimando | idem |
| il codice | oggi `permission::is_granted` rilegge tutto il giornale, un sì vale per sempre — anche dopo un riavvio — e la revoca non c'è; il 3 lega il sì alla sessione, con le due scadenze consegnate e la chiusura a mano, coi test nel simulatore sull'orologio iniettabile | il **3**, che porta le run, com'era assegnato dal disegno del 2 |
| V21, nella §8 della spec del sotto-progetto 1 | resta ⚠️ parziale col suo innesco, C (4): il 3 ne costruisce la metà della sessione, e il ciclo d'approvazione è scaglionato al 4. ⚠️ La §8 è spec: non si tocca senza il proprietario | — |

### D8, posta il 2026-09-29

**Che cos'è, a parole semplici.** Il grafo della UI mostra i file come punti. Le linee, oggi, sono di due specie e si
ricavano da sole: un file **sta in** una cartella, e un file chiave **appartiene** a un'area. Il documento lascia aperta una
terza specie — la sua decisione aperta 4 —: i **link** che il proprietario scrive dentro un file verso un altro, come
`[testo](percorso)` in markdown.

**Che cosa esiste già.**

| | Che cosa dice | Dove, e il comando |
|---|---|---|
| il documento del proprietario | il grafo si raggruppa per cartella e per area; *«Il contenimento in cartella e l'appartenenza a un'area si ricavano da soli»* | la decisione aperta 4 |
| il disegno del 2026-09-04 | le frecce della mappa — router → gruppo → foglia → skill, e il ritorno — e i segnali **orfano** e **rotto**; i collegamenti puntano a **file**, non ad ancore | la §4.1 e la §4.2 |
| D6 | un file chiave perso si segna rotto, coi candidati | la risposta D6 |
| questo repository | `check-docs.sh` controlla i link fra i documenti, e un link rotto è un rosso; un'ancora non la vede — la trappola 6 della §10 del compendio | `scripts/check-docs.sh` |
| la stella polare della GUI | la rete al centro della Home, un modulo che si mette a fuoco a pagina intera | `grep -n 'la rete al centro' docs/superpowers/specs/2026-09-07-direzione-gui-design.md` |
| Obsidian | nel suo grafo i cerchi sono le note e le linee i **link interni** fra due note; una nota più citata è più grande; i filtri tengono o tolgono gli **orfani** e i link verso file che non esistono; i formati sono due, `[[nota]]` e `[testo](percorso)`; e aggiorna i link quando si rinomina un file dentro Obsidian | *Graph view* e *Internal links*, lette alla fonte il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa arriva.** Il **6** costruisce il livello strutturale — la scansione che legge il testo — e il pannello del grafo;
la Home della GUI lo mette al centro.

**Regge crescendo?** Con molti file le linee diventano tante, e servono i filtri — per specie di linea, per area, per gli
orfani —, come quelli di Obsidian. Senza i link, invece, il grafo di una cartella di note mostra solo le cartelle, e la
struttura che il proprietario ha scritto non si vede.

**In tutte e due le risposte**: le linee di cartella e di area restano, ricavate da sole.

**La domanda: i link che il proprietario scrive fra i file diventano linee del grafo?**

| | **A — sì, una terza specie di linea** | **B — no, solo cartella e area** |
|---|---|---|
| com'è | la scansione, senza modello, legge i link dei file di testo — i due formati, `[testo](percorso)` e `[[nota]]` — e ne fa linee; un link verso un file che non c'è è un segnale **rotto**, come nel disegno del 2026-09-04; un link che esce dalla root punta alla scheda della sua zona, D3, o non si disegna | il grafo mostra cartelle e aree |
| costo | la scansione legge i link, poco e sul solo testo; più linee, e i filtri che le tengono leggibili | i link scritti non si vedono; e un link rotto in una nota non lo segnala nessuno |
| che cosa si rifà dopo | niente | aggiungerli dopo: un lettore dei link nella scansione |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | Obsidian letto alla fonte; il segnale rotto esiste già nel disegno del 2026-09-04 | — |
| coerenza | un link rotto è trattato come un puntatore dei router e come un link di questo repository | due pesi: i link dei router controllati, quelli delle note no |
| debito | il lettore dei link e i filtri, al 6 | i link rotti che nessuno vede |
| stato dell'arte | è il grafo di Obsidian | un grafo senza link è un albero di cartelle |
| proporzione | un lettore deterministico dei link | niente |
| di chi è | **del proprietario**: la sua decisione aperta 4 | idem |

**Verificato, dedotto, assunto.** **Verificati**: Obsidian alla fonte, la §4.1 e la §4.2 del 2026-09-04, `check-docs.sh`, la
stella polare della GUI. **Dedotti**: che leggere i link costi poco; che il grafo abbia bisogno di filtri. **Assunto**: che il
proprietario scriva link fra i suoi file.

**Il consiglio: A.** I link sono la struttura che il proprietario scrive da sé: leggerli non costa quasi niente, e un link
rotto diventa visibile invece di mentire.

### D9, posta il 2026-09-29 e riformulata lo stesso giorno

⚠️ **Il proprietario ha risposto alla prima forma — «un file-guida approvato vale anche quando cambia, o si riapprova?» — con
una consegna:** *«come funziona per i software stato dell'arte? tutto quello che puoi rispondere tramite il funzionamento di
essi usalo. Voglio se segui lo stato dell'arte odierno.»*. La prima forma sta nel commit `f797545`. Da qui la regola vale per
le domande che restano: lo stato dell'arte, letto alla fonte, risponde; al proprietario resta ciò che **urta** una decisione
del progetto.

**Che cos'è, a parole semplici.** Una repo aperta come zona di lavoro può contenere file scritti **per gli agenti**:
`CLAUDE.md`, `AGENTS.md`, `.cursorrules`, cartelle di skill. In una repo clonata da un altro quei file li ha scritti **un
altro**: leggerli come istruzioni vorrebbe dire che la repo di uno sconosciuto dà ordini all'agente — K15, la sorella di
AUD-004.

**Che cosa esiste già.**

| | Che cosa dice | Dove |
|---|---|---|
| ADR-0014 | il contenuto non fidato **informa, mai autorizza**; diventa istruzione solo con un passaggio esplicito, **giornalato** | la voce della §5 del compendio |
| ADR-0015 | la descrizione di uno strumento si mostra **intera** all'approvazione e se ne registra l'**impronta**; se cambia, lo strumento è **sospeso** finché non si riapprova, col diff mostrato | idem |
| ADR-0016 | i permessi sono triple tenute dal kernel, e *«una descrizione non concede permessi»* — ADR-0015, *«è testo, non autorità»* | idem |
| il disegno del 2026-09-04 | la regola 3: il registro delle guide **rifiuta** una guida senza impronta; la pretesa 1.1e: provenienza e impronta **all'approvazione**, e «approvate ora» come proiezione del giornale | la §2.3 e la §1.1 |
| AUD-004 | se le difese di ADR-0015 valgano anche per le skill: un ADR del proprietario, che **sbarra il 13** | la §6 del compendio |
| D3 | *«Se il `CLAUDE.md` di una repo si legga da solo, come fa Claude Desktop, lo decide D9»* | la sezione di D3 |

**Che cosa fanno i software di oggi — letto alla fonte il 2026-09-29**, la provenienza in [`riferimenti.md`](../../riferimenti.md).

| Fonte | Che cosa dice |
|---|---|
| Claude Code, *How Claude remembers your project* | il `CLAUDE.md` si carica **a ogni sessione**, e le istruzioni sono **contesto, non configurazione imposta**: per bloccare un'azione servono i permessi o un hook; un file importato da **fuori** dalla cartella chiede un'approvazione la prima volta — lo fa, dice la pagina, per proteggere dai file che altri committano in un progetto condiviso |
| Claude Code, *Configure permissions* | la **fiducia alla cartella**: finché non c'è, le regole della repo che **concedono** — i permessi *allow* e le cartelle in più — non valgono, mentre quelle che **negano** valgono sempre, perché restringono soltanto; i server MCP della repo si chiedono prima di connetterli; la fiducia si tiene per la radice della repo e si scrive su disco — nella sola cartella home vale per la sessione e basta. ⚠️ **Richiamo del 2026-09-29, revisione:** diceva *«le regole di permesso … non valgono»*, e sono solo quelle che concedono — riletto alla fonte |
| Gemini CLI, *Trusted Folders* | la fiducia si chiede **una volta per cartella** e si salva in un file centrale; in una cartella non fidata — la *safe mode* — le impostazioni e le variabili d'ambiente della repo si ignorano, le approvazioni automatiche degli strumenti si spengono, il **caricamento automatico della memoria** si spegne, i server MCP non si connettono e i comandi su misura non si caricano |
| VS Code, *Trust and safety for AI agents*, pagina del 2026-09-16 | una cartella non fidata gira in **modalità ristretta**, che spegne anche gli agenti; il contenuto dei file può tentare di dirottare l'agente |

Nessuna delle quattro riapprova un file-guida quando cambia.

**Che cosa ne segue.**

| | Il file-guida di una repo |
|---|---|
| la fiducia | si chiede **una volta per zona**, la prima volta che il proprietario la apre, e si scrive nel giornale; il proprietario la toglie quando vuole |
| una zona non fidata | **modalità ristretta**: i file-guida non si caricano da soli, le impostazioni e i server MCP della repo non valgono, nessuna approvazione automatica; i file-guida si possono leggere, e informano |
| una zona fidata | i file-guida si caricano a **ogni** sessione, anche quando cambiano, e l'impronta della versione caricata si scrive nel giornale: si sa sempre **quale** testo l'agente ha letto — la provenienza di ADR-0014 |
| che cosa non possono fare | concedere permessi: i permessi stanno nel kernel, ADR-0016, e un testo non li cambia — come in Claude Code, dove le istruzioni sono contesto e i permessi stanno nelle impostazioni |
| un import che esce dalla zona | chiede un'approvazione sua, come in Claude Code |
| la fiducia e la sessione | la fiducia **non** è un permesso di ADR-0016: non concede triple; decide se i file-guida e le impostazioni della repo si caricano, e se valgono le approvazioni automatiche del preset. Per questo resta oltre la sessione, come in Claude Code, che la scrive su disco, mentre il permesso di aprire la zona si chiede a ogni sessione — D3, D11. ⚠️ **Aggiunta del 2026-09-29, revisione:** senza questa riga D9 e D11 — *«alla fine cadono i suoi sì»* — sembravano dirsi il contrario |

**Dove urta il progetto, ed è la sola domanda.** La regola 3 del disegno del 2026-09-04 dice che il registro delle guide
**rifiuta una guida senza impronta**, e la pretesa 1.1e vuole l'impronta **all'approvazione**: la forma di ADR-0015, che
**riapprova** a ogni cambio. Lo stato dell'arte approva la **cartella**, non la versione del file.

**La domanda: per il file-guida di una zona si segue lo stato dell'arte — fiducia alla cartella, nessuna riapprovazione —
correggendo la regola 3 del 2026-09-04, o si tiene la regola 3?**

| | **A — lo stato dell'arte, e la regola 3 si corregge** | **B — la regola 3 com'è** |
|---|---|---|
| com'è | la tabella qui sopra; la regola 3 riceve un richiamo datato: per una guida di zona l'approvazione è la fiducia alla cartella, e l'impronta si scrive a ogni caricamento, per la provenienza | la fiducia alla cartella e la modalità ristretta, **più** la riapprovazione a ogni cambio, col diff |
| costo | un cambio del file — un `git pull` — arriva all'agente senza che il proprietario lo guardi, come in tutti e quattro i software letti; resta scritto nel giornale quale versione l'agente ha letto, e il testo non concede permessi | una conferma a ogni cambio, anche nelle repo del proprietario; più stretto di tutti i software letti |
| che cosa si rifà dopo | niente | niente |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | quattro fonti primarie lette oggi; la regola 3, la pretesa 1.1e e ADR-0015 letti | idem |
| coerenza | cambia una regola di un disegno, col richiamo datato; ADR-0014 e ADR-0016 restano: il testo informa e non concede | nessuna regola cambia |
| debito | il richiamo alla regola 3, e l'ADR di AUD-004 da scrivere sapendolo, perché per le skill decide la stessa cosa | la vista del diff, al 13 |
| stato dell'arte | è quello di Claude Code, Gemini CLI e VS Code | più stretto di tutti |
| proporzione | nessuna conferma in più | una conferma a ogni cambio |
| di chi è | **del proprietario**: corregge una sua regola del 2026-09-04 | idem |

**Verificato, dedotto, assunto.** **Verificati**: le quattro fonti, lette il 2026-09-29; ADR-0014, ADR-0015, ADR-0016, la
regola 3 e la pretesa 1.1e del 2026-09-04. **Dedotti**: che un file-guida non possa concedere permessi anche da noi — il
kernel li tiene a parte, ADR-0016 —; che il «caricamento automatico della memoria» di Gemini CLI copra i suoi file di
istruzioni — la pagina non li nomina. **Assunto**: che il proprietario lavorerà anche su repo scritte da altri.

**Il consiglio: A.** È ciò che fanno tutti i software letti: il testo non concede permessi, come da loro, e in più il giornale
dice sempre quale versione l'agente ha letto.

### D10, posta il 2026-09-29

⚠️ **Il proprietario ha risposto lo stesso giorno con una consegna:** *«attenzione, la selezione del modello deve funzionare come in cladue desktop (sia parte coding agentico che chat/cowork)»*. Letto alla fonte, Claude Desktop fa il cambio **da sé**, lo dichiara e non taglia il contesto: è la A, con la scelta a mano del modello e tre regole in più. La risposta sta in *«Come funziona in Claude Desktop, e che cosa ne segue»*, in fondo a questa sezione; qui sotto la domanda com'era posta.

**Che cos'è, a parole semplici.** Il contesto che il kernel manda al modello — la proiezione — si compone **per un modello**:
il budget è una fetta della sua finestra, ADR-0010, e la guida è la sua, il `claude.md` o il `deepseek.md` del 2026-09-04. Ma
se il routing ripiega su un altro modello — perché il primo è giù, limita, rifiuta, o il contesto è troppo grande —, il
contesto composto per il primo arriverebbe al secondo: con la guida sbagliata, e magari troppo grande — K16.

**Che cosa esiste già.**

| | Che cosa dice | Dove |
|---|---|---|
| ADR-0008 | il contesto è una **proiezione** dello stato durevole, ricalcolata a ogni passo; **mai sacrificabili** obiettivo, vincoli, piano, decisioni, fatti — l'unica cosa sacrificabile è la trascrizione grezza. È fra le decisioni che la §7 del compendio dice non rilitigabili | la voce della §5 e la §7 del compendio |
| ADR-0010 | il budget è **per modello**, e la ricomposizione è continua e proattiva | la voce della §5 |
| ADR-0011 | il record di routing risolto: modello, provider, **catena di riserva valutata, tentativi**, esito | idem |
| ADR-0012 | un candidato che viola un vincolo **non è un fallback** e si scarta prima; fra le cause l'indisponibilità, il **contesto eccessivo**, la moderazione; un ritentativo **non è un passo nuovo** | `grep -n 'contesto eccessivo' docs/adr/0012-*.md` |
| il disegno del 2026-09-04 | la chiave della proiezione è (ambito, run, **modello**), e c'è una guida per modello | la §1.1 |

**Che cosa fanno i software di oggi — letto alla fonte il 2026-09-29**, la provenienza in [`riferimenti.md`](../../riferimenti.md).

| Fonte | Che cosa dice |
|---|---|
| OpenRouter, *Model Fallbacks* | una lista di modelli in ordine: se il primo dà errore — contesto troppo lungo, moderazione, limiti, indisponibilità — prova il successivo, **dentro OpenRouter**; il modello usato davvero lo dice la risposta, e si paga quello |
| OpenRouter, *Message Transforms* | la compressione *middle-out* toglie o accorcia i messaggi **dal centro** del prompt, perché i modelli guardano meno il centro; è **accesa da sola** sulle destinazioni con finestra fino a 8 192 token, e un parametro la spegne |
| LiteLLM, *Fallbacks* | un fallback apposta per il contesto troppo grande, `context_window_fallbacks`, controllabile **prima** della chiamata; i fallback generici per gli altri errori |

**Che cosa ne segue**, dove lo stato dell'arte e i nostri ADR vanno d'accordo:

| | |
|---|---|
| per ogni candidato | la proiezione si compone **per quel modello** — il suo budget e la sua guida — e si controlla **prima** che entri, come i controlli prima della chiamata di LiteLLM; se non entra, il candidato si salta |
| il contesto eccessivo | si passa a un modello con la finestra più grande, come fa LiteLLM |
| nel giornale | il tentativo resta nello stesso passo, ADR-0012; il record dice quale modello e quale proiezione, con la misura per categoria di ADR-0010 |
| al 13 | la proiezione riceve il **modello** come ingresso — com'è già la chiave del 2026-09-04 — e si compone su richiesta del gateway, un candidato alla volta |

**Dove urta il progetto, ed è la domanda.** Due pratiche dei gateway contraddicono ADR già presi. La compressione
*middle-out* taglia il centro, cioè proprio ciò che ADR-0008 vuole **mai sacrificabile** — K39. E il fallback **dentro**
OpenRouter lascia al gateway il solo esito: non valuta i vincoli di ogni candidato, ADR-0012, né ricompone, ADR-0010, e il
modello lo sa a cose fatte — K40.

**La domanda: si seguono i nostri ADR — il fallback lo fa il gateway, un modello per richiesta, ricomponendo per ciascuno, e
la compressione dei gateway si spegne — o le pratiche dei gateway, con un ADR nuovo?**

| | **A — i nostri ADR, con ciò che dello stato dell'arte li rispetta** | **B — le pratiche dei gateway** |
|---|---|---|
| com'è | la tabella qui sopra: il gateway fa la catena da sé, ricompone per ogni candidato, controlla prima che entri, e manda a OpenRouter un modello per richiesta con la compressione **spenta** | la lista di modelli di OpenRouter e la sua compressione, lasciate come sono |
| costo | il gateway fa da sé ciò che OpenRouter offrirebbe già fatto; una richiesta per candidato | un ADR nuovo che superi ADR-0008 e ADR-0012; il centro del contesto si perde in silenzio sui modelli piccoli; la guida del primo modello arriva al secondo |
| che cosa si rifà dopo | niente | tornare indietro vuol dire rifare il gateway |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | ADR-0008, 0010, 0011 e 0012 letti; tre fonti primarie lette oggi | contraddice ADR-0008 e ADR-0012, letti |
| coerenza | nessun ADR cambia; prende dallo stato dell'arte il controllo prima della chiamata e il modello più grande | due ADR superati, uno dei quali non rilitigabile |
| debito | la catena del gateway e il parametro che spegne la compressione, al 3 — K39 e K40 | un contesto che perde il centro senza dirlo, contro il «nessun degrado silenzioso» di ADR-0019 |
| stato dell'arte | il controllo prima della chiamata di LiteLLM e il passaggio al modello più grande | la lista e la compressione di OpenRouter |
| proporzione | una composizione per candidato, deterministica | niente da costruire |
| di chi è | **del proprietario** | idem |

**Verificato, dedotto, assunto.** **Verificati**: ADR-0008, ADR-0010, ADR-0011, ADR-0012, la chiave del 2026-09-04, e alla
fonte OpenRouter e LiteLLM. **Dedotti**: che col fallback dentro OpenRouter il gateway non possa ricomporre — il prompt lo
manda una volta sola —; che la pagina di OpenRouter, che non lo dice per esteso, rimandi al modello di riserva la stessa
richiesta. **Assunto**: che il proprietario userà anche modelli con finestre piccole, dove la compressione scatta da sola.

**Il consiglio: A.** Il contesto è la cosa che il progetto ha deciso di non perdere mai: lo stato dell'arte si prende dove lo
rispetta — controllare prima, passare al modello più grande —, e si spegne dove lo taglierebbe.

#### Come funziona in Claude Desktop, e che cosa ne segue — 2026-09-29

**Che cosa dice la fonte**, letta il 2026-09-29 — la provenienza in [`riferimenti.md`](../../riferimenti.md).

| Fonte | Che cosa dice |
|---|---|
| Claude Code, *Model configuration* | il modello si sceglie **nella sessione**, col selettore: per la sola sessione, o salvato come default delle prossime; i sotto-agenti prendono il modello della sessione, salvo quello scritto nella loro definizione. La **catena di riserva** — al più tre modelli, in ordine — la percorre **Claude Code**, non il fornitore; scatta su sovraccarico, indisponibilità e altri errori del server, **mai** su autenticazione, fatturazione, limiti di frequenza e dimensione della richiesta; mostra un **avviso** quando cambia; il cambio dura **il turno**, e il messaggio dopo riprova il primo; nella compattazione **non** ripiega su un modello con la finestra più piccola, *«since summarizing there would cut off part of the conversation first»*; vale anche per i sotto-agenti, e il modello della sessione non cambia. Cowork, la scheda del lavoro agentico di Claude Desktop, gira su Claude Code |
| Claude, *Change the model, effort, and thinking settings* | nella chat il modello sta **accanto al pulsante di invio**, si cambia in qualunque momento della conversazione, e vale dalla risposta dopo |
| Claude, *Why Claude switched models in your conversation with Opus 5 or Opus 5.5* | un cambio automatico per **contenuto** è trasparente: un avviso, e la risposta porta il nome del modello che l'ha data; dopo, il selettore resta sul modello di riserva per il resto della conversazione, e si torna indietro dal selettore |

**Che cosa ne segue: la scelta del modello, uguale in chat, nel coding e nel lavoro agentico.**

| | |
|---|---|
| chi sceglie | il **proprietario**, col selettore accanto al pulsante di invio, in qualunque momento della sessione di D11, e vale dalla risposta dopo; per la sola sessione, o salvato come **default** — il default sta nella configurazione, ADR-0006 e ADR-0034; il cambio dentro la sessione è una sostituzione di parametro, cioè un record del giornale |
| i sotto-agenti | prendono il modello della sessione, salvo quello scritto nella loro definizione |
| la riserva | una catena in ordine, **percorsa dal nostro gateway** e mai dal fornitore: a OpenRouter va **un modello per richiesta**; i vincoli di ADR-0012 la filtrano prima — il gateway la risolve già per chiamata, RR10 |
| quando scatta | sovraccarico, indisponibilità, errore del server; **non** i limiti di frequenza né la dimensione — K48 —; il rifiuto per contenuto, ADR-0012, ripiega e **resta** sul modello di riserva per la sessione, e il selettore lo mostra |
| quanto dura | un ripiego per disponibilità dura **il turno**: il messaggio dopo riprova il modello scelto; un ritentativo resta nello stesso passo, ADR-0012 |
| il contesto | per ogni candidato la proiezione si compone **per quel modello** — la sua finestra e la sua guida, ADR-0010 — e si controlla prima di mandarla; un candidato la cui finestra non tiene ciò che ADR-0008 dice mai sacrificabile si **salta**, come Claude Code non ripiega su una finestra che taglierebbe; la compressione *middle-out* di OpenRouter **spenta**, perché nessuna delle fonti di Claude taglia il contesto in silenzio — K39 |
| che cosa si vede | un **avviso** quando il modello cambia, e ogni risposta porta il **nome del modello** che l'ha data, dal record di routing — ADR-0011, ADR-0019; il codice lo sa già dire: `Conforming::was_degraded` in `crates/kernel/src/gateway/mod.rs` |

**Letto contro il codice**, su `c12a170`: la catena per chiamata c'è — RR10 —, e il degrado dichiarato pure; due pezzi no — RR12 e RR13, K47.

**I cinque criteri, sul risultato.**

| Criterio | La risposta |
|---|---|
| correttezza verificata | tre pagine di Claude lette il 2026-09-29, la prima dal sorgente; ADR-0008, 0010, 0011, 0012 e `gateway/mod.rs` letti |
| coerenza | nessun ADR cambia: è la A, e Claude Desktop fa ciò che i nostri ADR chiedono — la catena la fa il cliente, il ripiego si dichiara, il contesto non si taglia |
| debito | il candidato che cresce, K47; quali errori fanno scattare la catena su OpenRouter, K48; il selettore e l'etichetta nella GUI, col disegno del modulo Chat |
| stato dell'arte | Claude Desktop, come chiede il proprietario |
| proporzione | nessun meccanismo che Claude Desktop non abbia |
| di chi è | **del proprietario**, che l'ha data a Claude Desktop |

**Verificato, dedotto, assunto.** **Verificati**: le tre pagine, il gateway, i quattro ADR. **Dedotti**: che la «sessione» di Claude Code sia la sessione di D11 — la stessa lettura di D11 —; che il rifiuto per contenuto di OpenRouter vada trattato come quello di Claude, che resta. **Assunto**: che *«come in Claude Desktop»* chieda il comportamento, non gli stessi modelli.

### D12, posta il 2026-09-29 — trovata dalla revisione, prima di D10

**Che cos'è, a parole semplici.** Il backup è la copia che il programma tiene per non perdere niente se il disco si rompe.
ADR-0022 dice che cosa ci va: ciò che non si può rifare — il giornale, gli **artefatti** delle run, la configurazione e le
**guide** —, e non ciò che si rifà, come l'indice, né i segreti, mai. D4 ha scritto, come cosa comune alle due risposte, che
il programma salva **solo** i suoi dati e i router, e che il resto della root lo salva il proprietario coi suoi backup. Ma
nella root stanno anche gli artefatti delle run — K4 — e le guide: la frase di D4 cambia due righe di ADR-0022 senza dirlo,
e la sezione di D4 la dava per coerente — K42.

**Che cosa esiste già.**

| | Che cosa dice | Dove, e il comando |
|---|---|---|
| ADR-0022 | nel backup: il giornale, gli **artefatti** e *«configurazione, guide, profili»*; fuori: indici e pesi, e i segreti **mai**; il backup contiene *«solo l'irriproducibile»*; e la base di conoscenza *«sopravvive alla reinstallazione perché i documenti sorgente e la configurazione sono nel backup»* | `grep -n -e '^. artefatti' -e 'irriproducibile' docs/adr/0022-*.md` |
| ADR-0022, il rimando del 2026-09-08 | le guide sono file della cartella della knowledge base, e la politica della riga — nel backup — non cambia | `grep -n 'nel backup, permanente' docs/adr/0022-*.md` |
| il disegno del 2026-09-04 e design/09 | la cartella della knowledge base **nel backup** | la §2.2 del 2026-09-04; `grep -n 'col 6, la cartella della knowledge base' docs/design/09-l0-fisico.md` |
| `tracciabilita.md` | `Backup della KB indipendente dall'app`: *«documenti nel backup, indice ricostruito»* | `grep -n 'Backup della KB' docs/tracciabilita.md` |
| il documento del proprietario | *«Il programma è una finestra su quella directory»*, che può essere *«la cartella in cui tengo già tutto quello che ho sul PC»* | la sezione *«Presupposti»* |
| la risposta D4 | *«il programma salva nel proprio backup i suoi dati e i router»* | la tabella delle risposte |
| Obsidian | tiene le note in locale e **non** le salva: il suo recupero dei file è limitato e per dispositivo, e la guida chiede all'utente un sistema di backup suo; la sincronizzazione non è un backup | *Back up your Obsidian files*, letta dal sorgente il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa arriva.** L'**11** costruisce backup e ripristino; il **7** scrive asset 3D grandi, K21; il **6** l'indice.

**Regge crescendo?** Col backup di ADR-0022 com'è, il programma deve sapere quali file della root sono artefatti — lo dice il
giornale — anche dopo che il proprietario li sposta o li riscrive da fuori, e un asset 3D grande finisce in due backup,
quello del programma e quello del proprietario. Col backup di D4, quello del programma resta piccolo.

**In tutte e due le risposte**, e non è una domanda: i segreti **mai** nel backup, e l'indice fuori, perché si rifà —
ADR-0022. *«I suoi dati»* di D4 si legge così.

**La domanda: il backup del programma salva anche gli artefatti delle run e le guide che stanno nella root, come dice
ADR-0022, o solo i suoi dati e i router, come dice D4?**

| | **A — come D4 e lo stato dell'arte: la root è del proprietario** | **B — come ADR-0022: anche artefatti e guide** |
|---|---|---|
| com'è | il programma salva il giornale, la configurazione e i router; **ogni** file della root — artefatti e guide compresi — sta nei backup del proprietario, e il programma lo **dice** quando crea il backup, come chiede il seguito di ADR-0022 | il programma salva il giornale, la configurazione e i router, **e** gli artefatti delle run e le guide che stanno nella root; il resto della root è del proprietario |
| costo | un **ADR nuovo** che superi, per i file della root, le righe degli artefatti e delle guide di ADR-0022 e la sua conseguenza sulla base di conoscenza — un ADR `Accepted` si cambia solo così, la §7 del compendio —; e il disegno del 2026-09-04, design/09 e la tracciabilità riletti | la regola «quali file della root sono artefatti», tenuta anche quando il proprietario li cambia da fuori — un file prodotto da una run e poi riscritto a mano è ancora un artefatto? —; i file grandi in due backup; nessun ADR cambia |
| che cosa si rifà dopo | niente: rimettere gli artefatti nel backup, un giorno, è un ADR come questo | togliere la regola, se il confine si rivela sfumato |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | ADR-0022 col rimando, design/09, la tracciabilità e la risposta D4 letti; Obsidian letto dal sorgente | idem; ma il confine «artefatto» con due attori non è sicuro: da solo il riconciliatore segue solo gli spostamenti esatti — D6 |
| coerenza | segue il documento — *«una finestra»* — e la risposta D4, e cambia due righe di un ADR con un ADR, com'è la regola | ADR-0022 resta com'è; la risposta D4 riceve un richiamo |
| debito | l'ADR da scrivere, dichiarato | il confine sfumato, da tenere nell'11 |
| stato dell'arte | è Obsidian: la cartella è dell'utente, e il suo backup pure | contro la fonte letta |
| proporzione | il backup del programma resta piccolo | un inseguitore di artefatti, e i file grandi due volte |
| di chi è | **del proprietario**: cambia un suo ADR | **del proprietario**: cambia una sua risposta |

**Verificato, dedotto, assunto.** **Verificati**: ADR-0022 col suo rimando, design/09, la tracciabilità, la risposta D4, e
alla fonte Obsidian. **Dedotti**: che il confine «artefatto» si sfumi quando il proprietario riscrive un file da fuori; che
un asset grande finisca in due backup. **Assunto**: che il proprietario abbia già un backup suo della cartella che userà
come root.

**Il consiglio: A.** È la risposta che il proprietario ha già dato in D4, col costo che allora non era scritto: un ADR che
dica apertamente che il backup della root è suo, come in Obsidian.

### D13, posta il 2026-09-29 — trovata dalla revisione, prima di D10

⚠️ **Riposta lo stesso giorno con un esempio.** Alla prima forma — anello, rete, scheda, zona — il proprietario ha risposto
*«non ho capito spiega meglio»*. La seconda l'ha detta con un caso: un progetto in `C:\progetti\app`, fuori dalla root,
dove l'agente scrive `main.rs`; la mappa della Home con la sola scheda del progetto, o con `main.rs` appeso alla scheda; e
l'elenco dei recenti, che lo mostra in tutti e due i casi. Il merito è quello qui sotto.

**Che cos'è, a parole semplici.** Nella Home della GUI ci sono due cose: l'**anello**, coi file prodotti dalle run in ordine
di data, e la **rete** al centro, il grafo della knowledge base. La decisione 1 della stella polare della GUI dice che la rete
ha *«tutto: artefatti e file della knowledge base»*; D3 dice che una zona di lavoro **fuori** dalla root non entra nel grafo.
Quando l'agente scrive codice in una repo aperta come zona fuori dalla root, quel codice è un artefatto: per la decisione 1
sta nella rete, per D3 no — K43.

**Che cosa esiste già.**

| | Che cosa dice | Dove, e il comando |
|---|---|---|
| la stella polare della GUI, decisione 1 | nell'anello solo **file** prodotti dalle run, per data — l'artefatto è un file sul disco, riferito dal giornale —; la rete al centro ha **tutto** | `grep -n 'Nell.anello solo' docs/superpowers/specs/2026-09-07-direzione-gui-design.md` |
| la stella polare della GUI, decisione 2 | la rete, a pagina intera, è il grafo della knowledge base | `grep -n 'la rete al centro è un modulo' docs/superpowers/specs/2026-09-07-direzione-gui-design.md` |
| D3 | una zona fuori dalla root non entra nel grafo né nella ricerca; nella knowledge base c'è la sua **scheda progetto**; la zona non ha un indice, e nessuno la sorveglia | la risposta D3 |
| il documento del proprietario | *«Un puntatore vecchio è peggio di nessun puntatore»* | la sezione *«Coerenza dei router»* |
| Obsidian | il grafo disegna le note e i link interni fra di esse | *Graph view*, letta il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa arriva.** Il **5** apre le zone e vi scrive codice; il **6** costruisce la rete; l'anello è della Home, nel
disegno della GUI.

**Regge crescendo?** Una repo su cui si lavora molto produce centinaia di file. Nella rete la riempirebbero, e, siccome la
zona non è sorvegliata, invecchierebbero appena il proprietario li sposta fuori dal programma. Nell'anello stanno per data, e
un file che non c'è più si vede quando lo si apre.

**La domanda: gli artefatti che l'agente scrive in una zona fuori dalla root stanno anche nella rete della Home, o solo
nell'anello?**

| | **A — solo nell'anello; nella rete la scheda** | **B — anche nella rete, appesi alla scheda** |
|---|---|---|
| com'è | l'anello li mostra, dal giornale, per data; nella rete la zona è la sua scheda, e da lì si apre; un file che non c'è più si mostra mancante quando lo si apre. La decisione 1 riceve un richiamo: la rete ha tutto ciò che sta **nella root** | la rete li mostra come nodi appesi alla scheda della zona, presi dal giornale; D3 riceve un richiamo: la zona non entra nel grafo, i file che le run vi scrivono sì |
| costo | nella rete il lavoro fatto in una repo esterna non si vede file per file | nodi che invecchiano senza che nessuno lo sappia, finché un controllo a ogni apertura non li segna rotti; la rete piena dei file di una repo |
| che cosa si rifà dopo | niente: aggiungerli un giorno si può | toglierli, se la rete diventa illeggibile |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | la decisione 1, D3 e il documento letti | idem; ma la freschezza dei nodi non è sicura: la zona non è sorvegliata |
| coerenza | D3 resta; la decisione 1 si legge «tutto ciò che sta nella root» | la decisione 1 resta; D3 si apre a metà |
| debito | nessuno | il controllo di freschezza sui nodi delle zone |
| stato dell'arte | è Obsidian: il grafo disegna le note della cartella che apre | un grafo di file fuori dalla cartella che mappa |
| proporzione | niente da costruire in più | un pezzo di sorveglianza per le zone |
| di chi è | **del proprietario**: cambia la lettura della sua decisione 1 | **del proprietario**: cambia la sua risposta D3 |

**Verificato, dedotto, assunto.** **Verificati**: le decisioni 1 e 2 della stella polare, D3, il documento, e Obsidian alla
fonte. **Dedotti**: che i nodi di una zona non sorvegliata invecchino; che la rete si riempia. **Assunto**: che al
proprietario basti l'anello per ritrovare il lavoro fatto in una repo esterna.

**Il consiglio: A.** Tiene la rete una mappa che dice il vero, e l'anello mostra comunque tutto ciò che le run hanno
prodotto.

### D14, posta il 2026-09-29 — trovata dalla prova alla radice, prima di D10

⚠️ **Riposta lo stesso giorno sullo stato dell'arte.** Alla prima forma — da sole, o il sì nella prossima sessione — il
proprietario ha risposto *«come farebbero con lo stato dell'arte attuale?»*. Lette alla fonte, Obsidian e VS Code non danno
un permesso di sessione ma un'**impostazione** scelta una volta: Obsidian parte da «aggiorna da solo» e, spenta, chiede; VS
Code parte da «chiedi», con «sempre» e «mai». La seconda forma chiedeva da che valore parte l'impostazione; la risposta sta
nella tabella delle risposte. Qui sotto resta la prima forma, col merito che non cambia: ogni correzione va nel giornale,
con la copia, e si annulla.

**Che cos'è, a parole semplici.** Il riconciliatore è il pezzo che, senza modello, rimette a posto i router quando il
proprietario sposta o cancella un file: toglie la voce di un file cancellato, e aggiorna il percorso di un file spostato che
riconosce dall'impronta. Il documento lo vuole **automatico**, e D6 ha deciso che il caso certo si applica da solo. Ma le
regole dei permessi dicono che una scrittura chiede un sì, e il sì di D7 vale **dentro una sessione** del proprietario; il
riconciliatore invece lavora all'avvio e quando il sorvegliante vede un cambio, cioè fuori da ogni sessione. Nessuna risposta
dice con quale autorità scrive — RR7, K46.

**Che cosa esiste già.**

| | Che cosa dice | Dove, e il comando |
|---|---|---|
| ADR-0016 | i permessi nascono perché il sistema *«esegue codice generato da un modello e strumenti di terze parti»*; col default le scritture chiedono, e un sì vale la sessione | `grep -n 'codice generato da un' docs/adr/0016-*.md` |
| il registro delle funzioni | chiede il permesso a chi **invoca** una funzione — oggi il click, poi gesto, voce e agente | `grep -n 'pub enum Invoker' crates/kernel/src/registry.rs` |
| la porta `filesystem` | scrive dentro un ambito e non chiede nessun permesso: lo chiede chi la chiama | `grep -n 'fn write' crates/kernel/src/ports/filesystem.rs` |
| ADR-0009 | l'anello di miglioramento **propone** e l'utente approva; *«non si auto-modifica in silenzio»* | la voce della §5 del compendio |
| il disegno del 2026-09-04, regola 2 | ogni scrittura nella cartella è un effetto giornalato con classe, dentro l'ambito: niente scrive di lato | la §2.3 |
| il documento, e D4 | la cartella `.<nomeapp>/` è del programma, *«una sola cosa da ignorare o cancellare»*, e i router vi stanno; D6: il caso certo si applica da solo | la decisione aperta 1; la risposta D6 |
| Obsidian | quando si rinomina un file dentro Obsidian, aggiorna da solo i link nelle note, e l'opzione si può spegnere | *Internal links*, letta il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |
| VS Code | per i percorsi degli import dopo uno spostamento, l'impostazione `js/ts.updateImportsOnFileMove.enabled`: `prompt`, il default, chiede a ogni spostamento; `always` aggiorna da solo; `never` né aggiorna né chiede | *Refactoring TypeScript*, pagina del 2026-09-16, letta il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md) |

**Che cosa arriva.** Il **6** costruisce il riconciliatore; il **13** il sorvegliante; il **4** i preset e il ciclo
d'approvazione.

**Regge crescendo?** Più file il proprietario riordina, più correzioni certe: se ciascuna aspetta un sì, i router restano
vecchi sul disco fino alla sessione dopo, e le domande crescono con l'ordine che il proprietario mette.

**In tutte e due le risposte**, e non è una domanda: ogni correzione va nel giornale con la sua classe, ha la sua copia e si
annulla — D2, ADR-0024 —, e il pannello la mostra; ciò che cambia una **scelta** del proprietario — un file chiave nel
dubbio, un candidato fra due — resta a lui, D6.

**La domanda: le correzioni certe del riconciliatore si scrivono da sole, come manutenzione del programma sulla sua
cartella, o aspettano il sì del proprietario nella sua prossima sessione?**

| | **A — da sole: la manutenzione del programma** | **B — aspettano il sì** |
|---|---|---|
| com'è | il riconciliatore rimette a posto `.<nomeapp>/` come il programma tiene in ordine i suoi dati: scrive da solo i due casi certi, e un'opzione lo spegne, come in Obsidian. ADR-0016 riceve un rimando: il permesso lo chiede chi agisce per un modello o invoca una funzione, non la manutenzione deterministica del programma sulla sua cartella | ogni correzione certa si prepara e aspetta: alla prossima sessione del proprietario il sì di D7 la applica; intanto la voce è segnata rotta, e l'agente non la segue |
| costo | un rimando in ADR-0016, che dice il perimetro; e la regola che separa un fatto da una scelta, da tenere nel 6 | i router vecchi sul disco fino alla sessione dopo; una conferma in più; e D6 si legge *«il caso certo si prepara da solo»* |
| che cosa si rifà dopo | niente | togliere l'attesa, un giorno, è questo stesso rimando |

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | il registro, la porta e ADR-0016 letti: oggi nessuna regola copre chi non invoca; Obsidian letto alla fonte | idem |
| coerenza | realizza il documento e D6 com'erano decisi; ADR-0016 lega già i permessi al codice di un modello e agli strumenti | tiene ADR-0016 alla lettera, e rilegge D6 |
| debito | il rimando, dichiarato | le voci rotte fra una sessione e l'altra |
| stato dell'arte | è Obsidian, che aggiorna i link da sé e si può spegnere | più stretto della fonte |
| proporzione | nessun meccanismo nuovo: un effetto giornalato come gli altri | un'attesa e una coda di proposte |
| di chi è | **del proprietario**: il perimetro di un suo ADR | **del proprietario**: una rilettura della sua D6 |

**Verificato, dedotto, assunto.** **Verificati**: `registry.rs`, `filesystem.rs`, `permission.rs`, ADR-0016, ADR-0009, la
regola 2 del 2026-09-04, e Obsidian alla fonte. **Dedotti**: che la manutenzione deterministica stia fuori dal perimetro di
ADR-0016 — il suo contesto parla di codice di un modello e di strumenti di terze parti, e non lo dice per esteso —; che le
correzioni certe non tocchino mai una scelta del proprietario. **Assunto**: che il proprietario voglia i router sempre
freschi sul disco, come il suo documento chiede.

**Il consiglio: A.** È ciò che il documento e D6 hanno già deciso; mancava solo di scrivere con quale autorità, e la più
piccola è quella che il programma ha già sui suoi dati.

## Le risposte del proprietario

| # | Risposta | Data |
|---|---|---|
| D1 | ⛔ **respinta**: il proprietario risponde col suo documento, riportato nella sezione *«Il documento del proprietario»*; le sue decisioni aperte 2 e 3 si accolgono come le propone | 2026-09-28 |
| D2 | ✅ **A** — la copia di ADR-0024 resta, **solo** per le azioni dell'agente, e il giornale resta; nessuno storico dei cambi del proprietario, nessun versioning, nessun sync. Nessun ADR cambia | 2026-09-28 |
| D3 | ✅ **A** — due specie di zona: la knowledge base, una e mappata, e le zone di lavoro, aperte come in Claude Desktop anche fuori dalla root, ciascuna col permesso per la sessione e la copia prima delle modifiche dell'agente; una zona **fuori** dalla root **non** entra nel grafo né nella ricerca, e nella knowledge base c'è la sua **scheda progetto**, che le fa da router; fuori da ogni zona l'agente non legge e non scrive. Posta il 2026-09-28, riformulata lo stesso giorno | 2026-09-29 |
| D4 | ✅ **A** — separati per natura: i **router** in `.<nomeapp>/` alla root, nascosta dal modulo di piattaforma, e l'**indice** nella cartella dati del programma, fra i dati rigenerabili, uno per root; i dati del programma nella cartella dati per utente del sistema, e il programma salva nel proprio backup i suoi dati e i router ⚠️ **2026-09-29, la revisione:** la metà sul backup urta ADR-0022, e torna al proprietario come **D12** — ✅ A, 2026-09-29 | 2026-09-29 |
| D5 | ✅ **A** — due specie: il **rumore**, dove lo scanner non entra ma il file o la cartella restano un nodo del grafo e l'agente li apre se serve; il **privato**, fuori dall'indice e da ciò che l'agente vede, con la porta che rifiuta la lettura e il confinamento dei comandi che nega quei percorsi. Le regole del privato le cambia solo il proprietario: l'agente propone, e ciò che rende leggibile qualcosa chiede conferma a ogni preset | 2026-09-29 |
| D6 | ✅ **A** — nel dubbio, **rotto** e una domanda: il caso certo — la stessa impronta, un solo candidato — si applica da solo; un file chiave che il riconciliatore non ritrova con certezza resta nella mappa segnato rotto, l'agente non lo segue, il pannello lo mostra, e il riconciliatore propone i candidati — lo stesso nome altrove, o un contenuto simile come fa git — fra cui sceglie il proprietario; due file identici sono un caso di dubbio | 2026-09-29 |
| D7 | ✅ **A** — un sì per la sessione: la tripla `(file, .<nomeapp>/, scrittura)` si concede alla prima modifica di un router e vale per la sessione di D11, come il punto 3 di ADR-0016; ogni modifica si vede nel turno e si annulla. Alla prima posa il proprietario aveva risposto con una domanda — la sessione non era definita, K37 —, e D7 si è riposta dopo D11. Con la risposta ha chiesto se la sessione sia già integrata nei permessi: **non ancora**, e dove va lo dice la sezione di D11 | 2026-09-29 |
| D8 | ✅ **A** — una terza specie di linea: la scansione, senza modello, legge i link dei file di testo — `[testo](percorso)` e `[[nota]]` — e ne fa linee del grafo; un link verso un file che non c'è è un segnale **rotto**; un link che esce dalla root punta alla scheda della sua zona, o non si disegna; i filtri — per specie di linea, per area, per gli orfani — al 6 | 2026-09-29 |
| D9 | ✅ **A** — lo stato dell'arte: la fiducia si chiede una volta per zona e sta nel giornale; una zona non fidata va in modalità ristretta, coi file-guida non caricati da soli; in una zona fidata i file-guida si caricano a ogni sessione, anche quando cambiano, con l'impronta della versione caricata nel giornale; un file-guida non concede permessi; un import che esce dalla zona chiede la sua approvazione. La regola 3 e la pretesa 1.1e del disegno del 2026-09-04 ricevono un richiamo datato col disegno di questa revisione: per una guida di zona l'approvazione è la fiducia alla cartella. Posta il 2026-09-29 e riformulata sullo stato dell'arte lo stesso giorno | 2026-09-29 |
| D10 | ✅ **A, come Claude Desktop** — *«la selezione del modello deve funzionare come in cladue desktop (sia parte coding agentico che chat/cowork)»*: il proprietario sceglie il modello col selettore, per la sessione o come default, e i sotto-agenti lo prendono; la catena di riserva la percorre il nostro gateway, un modello per richiesta a OpenRouter, con un avviso, e ogni risposta porta il nome del modello; un ripiego per disponibilità dura il turno, uno per contenuto resta; la proiezione si compone per ogni candidato, un candidato che non la tiene si salta, e la compressione di OpenRouter si spegne. Nessun ADR cambia; K47 e K48 registrati | 2026-09-29 |
| D11 | ✅ **delegata allo stato dell'arte** — *«come le sessioni moderne delle app moderne stato dell'arte, decision-principles devi seguire»*: la sessione è la run coi suoi sotto-agenti, uguale sul lato chat e sul lato coding; finisce quando il proprietario la chiude, dopo un tempo di inattività o dopo un tempo massimo, e la fa rispettare il core; i due tempi sono parametri consegnati, coi valori al 3 e il riferimento di NIST AAL2; alla fine cadono i suoi sì e si chiudono le sue zone. Il sì oltre la sessione delle app di oggi urta ADR-0016: segnalato, K38 | 2026-09-29 |
| D12 | ✅ **A, delegata allo stato dell'arte** — *«stato dell'arte, segui quello»*: la root è del proprietario, e il suo backup pure, come in Obsidian; il programma salva il giornale, la configurazione e i router — mai i segreti, e non l'indice che si rifà —, e quando crea il backup dice che cosa resta fuori. Un ADR nuovo supererà, per i file della root, le righe degli artefatti e delle guide di ADR-0022 e la sua conseguenza sulla base di conoscenza: lo scrive il disegno di questa revisione | 2026-09-29 |
| D13 | ✅ **A** — un file che l'agente scrive in una zona fuori dalla root sta nell'**anello**, per data, dal giornale; nella **rete** la zona è la sua scheda, e da lì si apre; un file che non c'è più si mostra mancante quando lo si apre. La decisione 1 della stella polare della GUI riceverà un richiamo col disegno: la rete ha tutto ciò che sta **nella root**. Riposta con un esempio: alla prima forma il proprietario aveva risposto *«non ho capito spiega meglio»* | 2026-09-29 |
| D14 | ✅ **A, sullo stato dell'arte** — come le app di oggi, il riconciliatore segue un'**impostazione** scelta una volta — da solo, chiedi, mai —, e parte da **«da solo»**, come Obsidian e come D6; ogni correzione va nel giornale, con la copia, e si annulla. ADR-0016 riceverà un rimando col disegno: il permesso lo chiede chi agisce per un modello o invoca una funzione, e la manutenzione deterministica del programma sulla sua cartella segue la sua impostazione. Alla prima forma il proprietario aveva chiesto *«come farebbero con lo stato dell'arte attuale?»*: lette alla fonte Obsidian e VS Code | 2026-09-29 |

## Come si riprende — scritto alla chiusura della seconda sessione del 2026-09-29

⛔ **Da sapere subito: niente è a metà.** Il proprietario ha chiuso dopo D14 — *«dopo questa si continua nella prossima
sessione»* —: la sessione che riprende **pone D10**, scritta e non ancora posta, e poi porta il brainstorming alla chiusura.
La chiusura precedente sta in
[`archivio/consegna-brainstorming-knowledge-base-revisione.md`](../../archivio/consegna-brainstorming-knowledge-base-revisione.md).

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline 561140e..HEAD`: la revisione, la prova alla radice, uno per risposta — D12, D13, D14 — e questa chiusura |
| codice di prodotto | **non toccato**: `git diff --stat 561140e..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura e prima di ogni commit, e `bash scripts/check-docs.sh` → `OK`: si rilanciano, non si citano |
| fine-riga | questo file e l'archivio della consegna **LF**; `riferimenti.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script della sessione, da `rev1.py` a `rev5.py`, stanno in due scratchpad — a metà sessione l'applicazione ne ha aperto uno nuovo —, e chi riprende non ne ha bisogno |
| la memoria dell'agente | due note aggiornate: *«stato dell'arte prima delle domande»* — anche dove urta una decisione, lo stato dell'arte è la A, e prima di chiudere una risposta si legge il codice che la regge —, e *«rilettura: voci A/B con domande semplici»* — quando un'opzione cambia che cosa si vede, lo schizzo delle due schermate |

**Dove si è arrivati.** Lo stato vive nelle tabelle di questo file; qui c'è solo dove guardare.

| | |
|---|---|
| le risposte | la tabella *«Le risposte del proprietario»*: D1 respinta; D2–D8 **A**; D9 **A**, sullo stato dell'arte; D11 delegata allo stato dell'arte; D12 **A**, delegata allo stato dell'arte; D13 **A**; D14 **A**, sullo stato dell'arte |
| la domanda scritta e non posta | **D10**, la proiezione quando il modello cambia per un fallback, col consiglio **A**; la prova alla radice l'ha già letta contro il codice: il gateway risolve la catena per chiamata, RR10 |
| la revisione | la sezione *«La revisione di coerenza e correttezza»*: i rilievi RC1–RC9, la prova alla radice RR1–RR11, e l'elenco di che cosa le risposte cambiano |
| i buchi | K1–K46, nelle due tabelle dei buchi; quelli nati in questa sessione sono K41–K46 |
| le fonti | la sezione datata di [`riferimenti.md`](../../riferimenti.md), *«La revisione della knowledge base — le fonti delle domande, 2026-09-29»* |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi questo file per intero, a blocchi: è la consegna di un brainstorming che si
   chiude.
3. `bash scripts/gate.sh` all'apertura, da solo; se è rosso su `ipc_wire` con `NotFound`, il gotcha #141.
4. **D10**, posta nella forma che ha funzionato oggi: a parole semplici, con un esempio concreto, lo stato dell'arte già
   letto alla fonte come opzione A, e prima riletta contro il codice che la regge.
5. Finite le domande, la **chiusura del brainstorming**, che il proprietario conferma; le righe **F** si leggono alla fonte;
   poi, in una sessione **nuova**, il disegno, che scrive l'elenco della sezione *«Che cosa le risposte cambiano»*: i
   richiami al disegno del 2026-09-04, i rimandi agli ADR riletti contro i fratelli — gotcha #59 —, l'**ADR nuovo** di D12
   che supera due righe di ADR-0022, la voce della §5 del compendio per ciascuno, e le righe di roadmap, tracciabilità e
   stella polare.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema, ripetuta anche in questa sessione. Costo: un `--amend` |
| 2 | i rilievi di forma della revisione, RC3–RC9, corretti qui col richiamo datato, senza chiedere; i due di merito portati come D12 e D13 | la regola della chiusura di prima: la forma si corregge, il merito va al proprietario. Costo: una domanda, se uno di forma era di merito |
| 3 | RR3, la sessione che **finisce al riavvio** del core, scritta in D11 senza domanda | la impone `time.rs`: le decisioni usano solo il tempo monotono. Costo: se il proprietario vuole sessioni che sopravvivono al riavvio, serve l'ora del mondo nelle decisioni, cioè un ADR nuovo |
| 4 | RR8: la regola di D5 sul privato letta come la **classe** della funzione, irripetibile, e non come un rimando in ADR-0016 | un meccanismo che c'è già, invece di una regola nuova. Costo: il rimando, se il proprietario lo vuole scritto |
| 5 | K44 e K45 registrati senza domanda | toccano il tipo del permesso e la porta `filesystem`, cioè la spec del sotto-progetto 1: li decide chi costruisce la porta vera, col proprietario. Costo: zero oggi |
| 6 | K41 chiuso nel principio dallo stato dell'arte, senza domanda | Claude Code applica anche senza fiducia le regole di una repo che negano; nessuna decisione del progetto urta. Costo: una domanda, se il proprietario la vuole |
| 7 | D12, D13 e D14 numerate dopo D11 e poste prima di D10 | un numero non si rinumera, come i K. Costo: zero |
| 8 | la risposta a D12, *«stato dell'arte, segui quello»*, letta come **A** | lo stato dell'arte letto quel giorno era Obsidian, cioè A. Costo: rileggere D12, se il proprietario intendeva altro |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **la revisione sui soli testi degli ADR** | il proprietario ha chiesto se le risposte stessero *«al cuore»*; il codice ha trovato K44, K45, K46 e il riavvio. 📌 *Prima di chiudere una risposta di disegno, si legge il codice del kernel che la regge* |
| **D13 in astratto** — anello, rete, scheda, zona | *«non ho capito spiega meglio»*. 📌 *Quando un'opzione cambia che cosa si vede, un caso concreto e lo schizzo delle due schermate* |
| **D14 come «permesso di sessione, o da solo»** | il proprietario ha chiesto lo stato dell'arte, e le fonti l'hanno riformulata: non è un permesso ma un'**impostazione**, e Obsidian e VS Code partono da valori diversi. 📌 *Prima di un'A/B sull'autorità di un meccanismo, cercare come la danno le app di oggi* |
| un `grep -i` con più di un `-e` | *Aborted*, e nessun risultato, altre due volte: la trappola 14 del disegno del 2026-09-04. 📌 *Un'alternanza si scrive con `-E` e le classi di maiuscola, come `[Rr]enam`* |

**Da verificare alla fonte prima del disegno** — le righe **F** ancora aperte: K6 e K34, i file «solo online» di OneDrive;
K9, gli eventi che il sorvegliante di Windows può perdere. E la pagina di VS Code di D14 è stata letta attraverso lo
strumento che riassume: il nome dell'impostazione si rilegge alla fonte prima di entrare nel disegno.
