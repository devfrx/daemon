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

## Che cosa esiste oggi — verificato il 2026-09-28 sull'albero di `f830cb9`

Ogni riga si rifà col comando accanto; il codice può muoversi, e il comando si rilancia prima di fidarsi della riga.

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

| Punto del documento | Contro che cosa | Esito |
|---|---|---|
| la knowledge base è **una cartella qualsiasi**, e la root arriva dalla configurazione | la risposta 1 del 2026-09-04, *«un archivio unico»*; ADR-0034, i parametri li legge il daemon e li consegna | **cambia** la risposta 1: non più un archivio dedicato. Coerente con ADR-0034: la root è un parametro che il daemon consegna |
| **due attori**: il proprietario da fuori, con qualsiasi strumento, e l'agente da dentro | la risposta 3, *«solo il nostro assistente»* | **cambia** la risposta 3. Chiude K7 |
| il **sorvegliante** più la **scansione all'avvio** | i trigger del 13 (ADR-0009); K8 e K9 | chiude K8 e K9 nel principio; al 13 resta un requisito: il meccanismo deve saper dire *«ho perso eventi, riscansiona»* |
| **niente storico**, versioning, audit o sync; una knowledge base per installazione | ADR-0007, il giornale delle azioni; ADR-0024, la copia prima che l'agente tocchi un file | ⚠️ **da conciliare — D2**. Chiude K5: niente sync |
| indici **e router** «derivati, ricostruibili» | ADR-0022, gli indici fuori dal backup perché ricostruibili | ⚠️ vale per l'indice strutturale, **non per i router**, che portano le scelte del proprietario — K32 |
| il **livello strutturale**: l'indice completo, testo compreso, senza modello | il disegno del 2026-09-04: l'indice della mappa (§4.2), e la ricerca per somiglianza come **seconda metà** del 6 (§4.4) | **allarga**: una ricerca testuale senza GPU nasce con la prima metà; la seconda resta per la somiglianza |
| il **livello semantico**: router master → indici d'area → file chiave; le aree non sono cartelle | la risposta 1 e la §4.1: router → gruppi → foglie, e *«un gruppo è una voce di router, non una cartella»* | **uguale** nella sostanza: l'«area» è il «gruppo» del 2026-09-04 |
| l'agente cerca anche nel **livello strutturale**, prima nell'area e poi dappertutto | la risposta 4 e la §4.1: *«l'agente naviga la mappa, mai le cartelle»*, e gli orfani per l'agente *«non esistono»* | **cambia** le risposte 4 e 10: un file fuori dai router si trova lo stesso, con un costo in più |
| il **riconciliatore** corregge i router da solo, senza modello | la §1.4 del 2026-09-04: un sensore trova il puntatore rotto, l'anello **propone**, il proprietario approva | **cambia** per i due casi meccanici — cancellato, spostato con lo stesso hash — e resta deterministico, quindi fuori dal divieto di ADR-0020. Chiude K18 nel principio; apre K24 e K25 |
| un file nuovo entra nei router solo se **promosso** | la §4.1: gli orfani mostrati nel pannello | coerente |
| la **UI**: grafo o griglia per cartella e per area, ricerca, anteprima, e tutte le CRUD **senza passare dall'agente** | la §4.3 del 2026-09-04; ADR-0038, un registro e molti invocatori | **allarga** il pannello. «Senza l'agente» vuol dire senza il **modello**: il click invoca la stessa funzione del registro, e il core la esegue, la giornala e ne controlla il permesso |
| le **esclusioni** in stile `.gitignore` alla root, per l'indice **e** per ciò che l'agente legge | la decisione 13 del 2026-09-04, *«privato ma non segreto»*, aperta; K11 e K21 | chiude la 13 nel principio; apre K26 e K27 |
| il **confinamento**: l'agente scrive solo dentro la root, niente `..` né collegamenti simbolici fuori | ADR-0024, l'ambito; K13 e K14 | chiude K14; K13 a metà, perché dice *scrive* e non *legge* — K28 |
| le **scritture concorrenti**: prima di scrivere si controlla che il file non sia cambiato | K10 | chiude K10 |
| il **setup** in due fasi, idempotente, coi soli percorsi relativi | la guida ARMS, il livello dei router | nuovo: è la capacità, il 6 |
| la decisione aperta 1: **dove vivono** indici e router | ADR-0022; K30, K31, K32 | **D4** |
| la decisione aperta 2: le **aree** proposte dall'agente e confermate dal proprietario | la risposta 10 del 2026-09-04 | coerente: la sua proposta si accoglie com'è |
| la decisione aperta 3: la **cancellazione** dell'agente, morbida e con conferma | ADR-0024, la copia prima; ADR-0016, le scritture che chiedono | coerente, e la copia del kernel è una rete in più: la sua proposta si accoglie com'è |
| la decisione aperta 4: i **link markdown** come archi del grafo | la §4.2 del 2026-09-04, le frecce della mappa | **D8** |

## Lo stato dei buchi dopo il documento

| # | Stato |
|---|---|
| K1 | **aperto**: dove vivono la configurazione che porta la root e gli altri dati del programma — con D4 |
| K2 | **chiuso**: la root arriva dalla configurazione |
| K3 | **chiuso per la knowledge base**: le repo possono stare dentro la root, e il rumore lo toglie il file delle esclusioni; resta il confine, K28 |
| K4 | **chiuso**: i file delle run stanno dentro la root, perché l'agente scrive solo lì; resta la visibilità dei file pesanti, K27 |
| K5 | **chiuso**: una knowledge base per installazione, niente sync |
| K6 | **a metà**: chi scrive da fuori è coperto dai due attori; restano i file «solo online», K34 |
| K7 | **chiuso**: due attori, e il riallineamento |
| K8 · K9 | **chiusi nel principio**: il sorvegliante più la scansione; al 13 resta l'evento «riscansiona» |
| K10 | **chiuso**: il controllo prima di scrivere |
| K11 | **a metà**: il privato si esclude; un segreto in una nota **non** esclusa resta — il sensore sulle scritture, al 6 |
| K12 | **aperto**, registrato: al 4 |
| K13 | **a metà**: lo scrivere è confinato, il leggere no — K28 |
| K14 | **chiuso**: niente collegamenti simbolici fuori |
| K15 | **aperto**: D9 |
| K16 | **aperto**: D10 |
| K17 | **a metà**: la cancellazione dell'agente è morbida; «dimentica davvero» resta registrato |
| K18 | **chiuso nel principio**: l'hash; i suoi casi limite sono K24 e K25 |
| K19 | **a metà**: i percorsi relativi alla root; maiuscole e nomi riservati restano, alla porta vera |
| K20 | **chiuso**: dentro la root i file li porta il proprietario da fuori, e l'agente fuori non arriva |
| K21 | **a metà**: i file pesanti fuori dall'indice; la copia costa solo sui file che l'agente tocca; il limite di ADR-0024 resta da fissare |
| K22 | **a metà**: il costo in token dipende dal percorso, non dalla dimensione; il grafo coi molti nodi resta al 6 |
| K23 | **aperto**, registrato |

### I casi limite nuovi, aperti dal documento

| # | Il caso | Specie | 13? | Chi lo chiude, proposto |
|---|---|---|---|---|
| **K24** | **spostato e modificato insieme** — o salvato da un editor come file nuovo: l'hash cambia, il riconciliatore vede «cancellato» più «nuovo», toglie la voce, e un file chiave esce dalla mappa senza che nessuno lo sappia | D | — | **D6** |
| **K25** | **due file identici**: lo stesso hash in due posti, e lo spostamento diventa ambiguo | D | — | **D6** |
| **K26** | **chi scrive il file delle esclusioni**: se l'agente può toglierne una riga, un file malevolo che l'agente ha letto può convincerlo a scoprire il privato e poi leggerlo — *«un'istruzione trovata nei dati non è mai un'autorizzazione»*, ADR-0014 | D | — | **D5** |
| **K27** | **escluso non vuol dire invisibile**: i «media pesanti» esclusi sparirebbero dal grafo, mentre la rete della Home deve mostrare anche gli asset 3D — decisione 1 della stella polare | V + D | — | **D5** |
| **K28** | **la root è il confine di tutto l'assistente, o solo della knowledge base?** Il coding lavora su repo: dentro la root, o anche fuori con ambiti suoi (ADR-0024) e permessi suoi (ADR-0016)? E il documento confina lo **scrivere**, non il **leggere** | D | ✅ gli ambiti che il piano 0 usa come chiave | **D3** |
| **K29** | **una root enorme** — «tutto quello che ho sul PC»: la prima scansione è lunga, e le build nelle repo inondano il sorvegliante. Serve una scansione incrementale — dimensione e data, l'hash solo se cambiano — e lo stato *«indicizzazione in corso»* dichiarato prima, come vuole ADR-0019 | D | ✅ l'evento «riscansiona» | registrato: il 6, e il 13 per l'evento |
| **K30** | **su Windows il punto nel nome non nasconde una cartella**: `.git` è nascosta perché git le mette l'attributo H, `.github` e `.superpowers` no. La cartella `.<nomeapp>/` va marcata nascosta dal modulo di piattaforma | V: `cmd //c "attrib .git"` e `cmd //c "attrib .github"` nella radice di questo repository | — | **D4** |
| **K31** | **il backup**: se la root è il PC intero, il programma non può salvarla tutta, mentre ADR-0022 metteva la cartella della knowledge base nel suo backup. Il programma salva ciò che è **suo** — i router —, e il resto è dei backup del proprietario | V + D | — | **D4**, poi l'11 |
| **K32** | **i router non si ricostruiscono**: portano le scelte del proprietario — le aree, i file chiave, la riga di descrizione. Rifarli è rifare il setup guidato, coi suoi token e le sue domande | D | — | **D4** |
| **K33** | **l'agente che scrive senza chiedere**: col preset di default di ADR-0016 ogni scrittura chiede conferma, mentre il documento vuole i router aggiornati nello stesso turno | V: ADR-0016, punto 2 | — | **D7** |
| **K34** | **i file «solo online» di OneDrive** dentro la root: leggerli scarica il file, o fallisce senza rete | F | — | registrato: il 6, alla fonte |

## Le domande, una per volta

Ogni domanda si pone **sola**, col contesto, A/B, i cinque criteri e il consiglio; la risposta va nella tabella *«Le risposte
del proprietario»*, e si committa. ⚠️ **L'elenco è stato rifatto il 2026-09-28** dopo il documento del proprietario, che
risponde a D1 e a buona parte delle domande di prima; l'elenco di prima sta nel commit `e714720`.

| # | Domanda | Chiude |
|---|---|---|
| **D1** | la forma: tre posti o uno solo | ⛔ **respinta**: la risposta è il documento del proprietario |
| **D2** | **lo storico**: il giornale e la copia valgono solo per le azioni dell'agente? | la riga *«niente storico»* del confronto |
| **D3** | **il confine**: la root vale per tutto l'assistente, anche per leggere? | K28, K13 |
| **D4** | **dove vivono** indici e router — la decisione aperta 1 del proprietario — e i dati del programma | K1, K30, K31, K32 |
| **D5** | **le esclusioni**: quante specie, e chi scrive il file delle regole | K26, K27 |
| **D6** | **il file chiave che il riconciliatore non ritrova** | K24, K25 |
| **D7** | **l'agente che scrive senza chiedere** | K33 |
| **D8** | **i link markdown come archi** — la decisione aperta 4 del proprietario | — |
| **D9** | **i file-guida delle repo**: mai iniettati da soli; guida solo se importati e approvati | K15, con AUD-004 |
| **D10** | **la proiezione quando il modello cambia** per un fallback | K16 |
| — | registrati col chiusore, senza domanda salvo che il proprietario la chieda: K11, K12, K17, K19, K21, K22, K23, K29, K34 | |

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

### D3, posta il 2026-09-28

**Che cos'è, a parole semplici.** Il documento dice che l'agente **scrive** solo dentro la root. Non dice se può **leggere**
fuori, né se il confine vale anche per le altre capacità — il coding sulle repo, gli asset 3D, le catture.

**Che cosa esiste già.** La porta `filesystem` del kernel legge e scrive **solo dentro un ambito dichiarato**, e fuori risponde
`OutsideScope`, in modo *«fail-closed»*: `grep -n -e 'OutsideScope' -e 'inside a declared scope' crates/kernel/src/ports/filesystem.rs`.
`declare_scope` accetta **più** percorsi, quindi più zone sono possibili. ADR-0024 dice che gli effetti **fuori** dagli ambiti
non sono coperti e chiedono approvazione; ADR-0016 dà un permesso per percorso. ⚠️ **La domanda non riguarda i programmi che
l'agente lancia** — un compilatore legge i suoi file di sistema —: quello è il recinto di ADR-0025, e si decide col 5.

| | **A — un confine solo: la root, per tutto l'assistente, anche per leggere** | **B — la root è il confine della knowledge base; altre zone si aprono per nome** |
|---|---|---|
| com'è | ogni capacità legge e scrive **solo** dentro la root; una repo su cui l'agente lavora sta dentro la root | il coding può dichiarare una repo **fuori** dalla root come zona sua, con permesso e copia suoi |
| costo | la root deve contenere tutto ciò su cui l'agente lavora; se è molto larga — l'intera cartella utente — dentro ci sono anche cose delicate come `AppData` o `.ssh`, e le esclusioni di base devono toglierle, D5 | le esclusioni stanno nella root: una zona fuori non le ha, e l'agente vi leggerebbe tutto, un `.env` compreso — oppure ogni zona vuole le sue regole, due posti da tenere allineati; e una repo fuori non compare nel grafo né nella ricerca |
| che cosa si rifà dopo | niente: una seconda zona si può aggiungere un giorno, con una decisione sua, perché la porta ne accetta più d'una | togliere le zone esterne vorrebbe dire spostare le repo |

**Una conseguenza per il 13, qualunque sia la risposta — dedotta.** Il 2026-09-04 la chiave del piano 0 era *«l'ambito»*,
dedotto come l'ambito di ADR-0024 — decisione 15, aperta. Col documento la chiave naturale è l'**area**, e il kernel la
riceverebbe come una chiave **opaca**, senza sapere che cosa sia: ADR-0001 gli vieta di conoscere «aree» e «router». Si
prova nel 13, e nel disegno si scrive come correzione della decisione 15.

**I cinque criteri.**

| Criterio | A | B |
|---|---|---|
| correttezza verificata | poggia sulla porta com'è: fuori dall'ambito, rifiuto | le esclusioni per zona sono dedotte, non esistono |
| coerenza | una regola di privacy e un confine | regole in più posti |
| debito | nessuno | due liste da tenere uguali, e repo invisibili alla ricerca |
| stato dell'arte | non serve: sono decisioni del repository | idem |
| proporzione | il minimo: una zona | un macchinario per un caso che il documento non chiede |
| di chi è | **del proprietario** | idem |

**Verificato, dedotto, assunto.** **Verificati**: la porta `filesystem`, ADR-0024 e ADR-0016. **Dedotti**: i costi di B, e la
chiave opaca per il 13. **Assunto**: che le repo su cui il proprietario vuole l'agente possano stare sotto una root sola.

**Il consiglio: A.** Un confine solo, e la stessa regola di privacy dappertutto dove l'agente arriva.

## Le risposte del proprietario

| # | Risposta | Data |
|---|---|---|
| D1 | ⛔ **respinta**: il proprietario risponde col suo documento, riportato nella sezione *«Il documento del proprietario»*; le sue decisioni aperte 2 e 3 si accolgono come le propone | 2026-09-28 |
| D2 | ✅ **A** — la copia di ADR-0024 resta, **solo** per le azioni dell'agente, e il giornale resta; nessuno storico dei cambi del proprietario, nessun versioning, nessun sync. Nessun ADR cambia | 2026-09-28 |
| D3 | ⏳ posta, in attesa | 2026-09-28 |

## Come si riprende — scritto all'apertura del brainstorming, il 2026-09-28

⛔ **Niente è a metà.** Il commit di questo file porta anche il puntatore della §6 del compendio; albero pulito, tutto pushato.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| codice di prodotto | **non toccato**: `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`, all'apertura sull'albero di `f830cb9` e di nuovo prima del commit di questo file: si rilanciano, non si citano |
| il puntatore | la §6 del compendio: la revisione **prima** del 13 e dei modelli decisionali |
| la guida ARMS | **non** è nel repository: il riassunto sta nella sezione *«La fonte del documento»*, la provenienza in [`riferimenti.md`](../../riferimenti.md) |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, e il
   [disegno del 2026-09-04](2026-09-04-knowledge-base-design.md) per intero — la §12 del compendio lo chiede a chi riprende il
   fronte della knowledge base.
3. La prima riga della tabella *«Le risposte del proprietario»* ancora ⏳ è la domanda da porre: si ripone **com'è scritta
   qui**, dopo aver rilanciato i comandi della tabella *«Che cosa esiste oggi»* — il codice può essersi mosso.
4. A ogni risposta: la riga nella tabella, un commit, un push. La domanda successiva si scrive **qui**, nella forma di D1, prima
   di porla.
5. Finite le domande: la chiusura del brainstorming — le decisioni prese, le registrate col chiusore — e il disegno in una
   sessione **nuova**, che scrive i richiami datati al disegno del 2026-09-04 e i rimandi agli ADR.
