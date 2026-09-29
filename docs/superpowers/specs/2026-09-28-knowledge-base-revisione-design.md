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
| K1 | **chiuso**: i dati del programma nella cartella dati per utente del sistema — `%LOCALAPPDATA%\<nomeapp>\` su Windows, le cartelle XDG su Linux —, in sottocartelle per natura, e la configurazione porta il percorso della root. D4, 2026-09-29; lo costruisce il primo sotto-progetto che installa il programma |
| K2 | **chiuso**: la root arriva dalla configurazione |
| K3 | **chiuso**: una repo dentro la root sta nel livello strutturale, meno ciò che le esclusioni tolgono; una repo fuori è una zona di lavoro con la sua scheda progetto — D3, 2026-09-29 |
| K4 | **chiuso**: i file delle run stanno dentro la root, perché l'agente scrive solo lì; resta la visibilità dei file pesanti, K27 |
| K5 | **chiuso**: una knowledge base per installazione, niente sync |
| K6 | **a metà**: chi scrive da fuori è coperto dai due attori; restano i file «solo online», K34 |
| K7 | **chiuso**: due attori, e il riallineamento |
| K8 · K9 | **chiusi nel principio**: il sorvegliante più la scansione; al 13 resta l'evento «riscansiona» |
| K10 | **chiuso**: il controllo prima di scrivere |
| K11 | **a metà**: il privato si esclude, ed è un confine — D5; un segreto in una nota **non** esclusa resta — il sensore sulle scritture, al 6 |
| K12 | **aperto**, registrato: al 4 |
| K13 | **chiuso**: un collegamento che esce dalla root punta a una zona, la scheda progetto, e il modello la legge solo a zona aperta — fuori da ogni zona la porta risponde `OutsideScope`. D3, 2026-09-29 |
| K14 | **chiuso**: niente collegamenti simbolici fuori |
| K15 | **aperto**: D9 |
| K16 | **aperto**: D10 |
| K17 | **a metà**: la cancellazione dell'agente è morbida; «dimentica davvero» resta registrato |
| K18 | **chiuso**: l'hash per il caso certo, e il dubbio segnato rotto coi candidati — K24 e K25, D6 |
| K19 | **a metà**: i percorsi relativi alla root; maiuscole e nomi riservati restano, alla porta vera |
| K20 | **chiuso**: dentro la root i file li porta il proprietario da fuori, e l'agente fuori non arriva |
| K21 | **a metà**: i file pesanti fuori dall'indice; la copia costa solo sui file che l'agente tocca; il limite di ADR-0024 resta da fissare |
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
| **K31** | **il backup**: se la root è il PC intero, il programma non può salvarla tutta, mentre ADR-0022 metteva la cartella della knowledge base nel suo backup. Il programma salva ciò che è **suo** — i router —, e il resto è dei backup del proprietario | V + D | — | **D4** — ✅ chiuso il 2026-09-29: il programma salva i suoi dati e i router, il resto è del proprietario; lo costruisce l'11 |
| **K32** | **i router non si ricostruiscono**: portano le scelte del proprietario — le aree, i file chiave, la riga di descrizione. Rifarli è rifare il setup guidato, coi suoi token e le sue domande | D | — | **D4** — ✅ chiuso il 2026-09-29: i router stanno in `.<nomeapp>/`, lontani dall'indice che si rifà, e il programma li salva |
| **K33** | **l'agente che scrive senza chiedere**: col preset di default di ADR-0016 ogni scrittura chiede conferma, mentre il documento vuole i router aggiornati nello stesso turno | V: ADR-0016, punto 2 | — | **D7** |
| **K34** | **i file «solo online» di OneDrive** dentro la root: leggerli scarica il file, o fallisce senza rete | F | — | registrato: il 6, alla fonte |
| **K35** | **una zona di lavoro si apre e non si chiude**: la porta `filesystem` ha `declare_scope` e nessuna chiusura, mentre la zona dura la sessione; e gli ambiti sono **della porta**, non della run — due run con due zone diverse, gli agenti del 4, alla porta vedrebbero l'una la zona dell'altra, e il confine per run lo dà solo il permesso di ADR-0016 | V: `grep -n '^    fn ' crates/kernel/src/ports/filesystem.rs` rende i cinque metodi del tratto, nessuno che chiuda; D: una porta sola nel daemon | — | registrato: chi costruisce la porta `filesystem` vera, con K23 |
| **K36** | **il privato escluso dalla porta non lo è per i comandi**: uno script che l'agente esegue apre i file da sé, e la porta non lo vede. Le documentazioni di Claude Code e di Cursor lo dicono dei loro prodotti; da noi il livello 1 di ADR-0025, per costruzione, non regge contro codice eseguito | V alla fonte, il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md); D per il nostro caso | — | registrato: il 5, col confinamento di livello 2 che nega i percorsi privati; il 4 per MCP, con K12 |
| **K37** | **la sessione non è definita**: ADR-0016 dice che un sì vale *«per la sessione corrente»* e *«non vale domani»*, ma nessun documento dice che cos'è una sessione; il kernel lo dichiara nel sorgente, e un permesso concesso resta concesso **per sempre**, anche dopo un riavvio; il disegno del 2 ha dato il confine a chi porta le run, il 3, senza definirlo. Trovato dal proprietario, rispondendo a D7 | V: `grep -n 'SCOPED TO A SESSION' crates/kernel/src/permission.rs`, `grep -n 'triple therefore survives' crates/kernel/src/registry.rs`, `grep -n 'il confine di sessione dei permessi' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | — | **D11** — ✅ chiuso il 2026-09-29: la run coi sotto-agenti, con la chiusura a mano e le due scadenze; la costruisce il 3 |
| **K38** | **il sì oltre la sessione**: Claude Code e VS Code offrono anche un sì per lo spazio di lavoro o per sempre, con un comando che li azzera, e Android azzera da solo i permessi non usati; ADR-0016 dice *«un'approvazione non si estende»*, e fra le sue alternative non ha mai valutato la durata | V alla fonte, il 2026-09-29, in [`riferimenti.md`](../../riferimenti.md); `grep -n 'Alternative considerate per i permessi' docs/adr/0016-*.md` | — | registrato: il **proprietario**, con un ADR nuovo se vorrà riaprire il punto 3 di ADR-0016 |

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
| **D10** | **la proiezione quando il modello cambia** per un fallback | K16 |
| **D11** | **la sessione**: che cos'è, e se scade col tempo — posta **prima** di D7, che ne dipende | K37 |
| — | registrati col chiusore, senza domanda salvo che il proprietario la chieda: K11, K12, K17, K19, K21, K22, K23, K29, K34, K35, K36, K38 | |

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
| il permesso | concesso per la root | concesso all'apertura, per la sessione — ADR-0016 |
| le regole di privacy | una **lista di base comune** a tutte le zone — `.env`, chiavi, `.ssh` —, più il file della root | la stessa lista di base, più le regole della zona |

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

## Le risposte del proprietario

| # | Risposta | Data |
|---|---|---|
| D1 | ⛔ **respinta**: il proprietario risponde col suo documento, riportato nella sezione *«Il documento del proprietario»*; le sue decisioni aperte 2 e 3 si accolgono come le propone | 2026-09-28 |
| D2 | ✅ **A** — la copia di ADR-0024 resta, **solo** per le azioni dell'agente, e il giornale resta; nessuno storico dei cambi del proprietario, nessun versioning, nessun sync. Nessun ADR cambia | 2026-09-28 |
| D3 | ✅ **A** — due specie di zona: la knowledge base, una e mappata, e le zone di lavoro, aperte come in Claude Desktop anche fuori dalla root, ciascuna col permesso per la sessione e la copia prima delle modifiche dell'agente; una zona **fuori** dalla root **non** entra nel grafo né nella ricerca, e nella knowledge base c'è la sua **scheda progetto**, che le fa da router; fuori da ogni zona l'agente non legge e non scrive. Posta il 2026-09-28, riformulata lo stesso giorno | 2026-09-29 |
| D4 | ✅ **A** — separati per natura: i **router** in `.<nomeapp>/` alla root, nascosta dal modulo di piattaforma, e l'**indice** nella cartella dati del programma, fra i dati rigenerabili, uno per root; i dati del programma nella cartella dati per utente del sistema, e il programma salva nel proprio backup i suoi dati e i router | 2026-09-29 |
| D5 | ✅ **A** — due specie: il **rumore**, dove lo scanner non entra ma il file o la cartella restano un nodo del grafo e l'agente li apre se serve; il **privato**, fuori dall'indice e da ciò che l'agente vede, con la porta che rifiuta la lettura e il confinamento dei comandi che nega quei percorsi. Le regole del privato le cambia solo il proprietario: l'agente propone, e ciò che rende leggibile qualcosa chiede conferma a ogni preset | 2026-09-29 |
| D6 | ✅ **A** — nel dubbio, **rotto** e una domanda: il caso certo — la stessa impronta, un solo candidato — si applica da solo; un file chiave che il riconciliatore non ritrova con certezza resta nella mappa segnato rotto, l'agente non lo segue, il pannello lo mostra, e il riconciliatore propone i candidati — lo stesso nome altrove, o un contenuto simile come fa git — fra cui sceglie il proprietario; due file identici sono un caso di dubbio | 2026-09-29 |
| D7 | ⏳ il proprietario ha risposto con una domanda — la sessione non è definita, K37 —: si ripone dopo D11 | 2026-09-29 |
| D11 | ✅ **delegata allo stato dell'arte** — *«come le sessioni moderne delle app moderne stato dell'arte, decision-principles devi seguire»*: la sessione è la run coi suoi sotto-agenti, uguale sul lato chat e sul lato coding; finisce quando il proprietario la chiude, dopo un tempo di inattività o dopo un tempo massimo, e la fa rispettare il core; i due tempi sono parametri consegnati, coi valori al 3 e il riferimento di NIST AAL2; alla fine cadono i suoi sì e si chiudono le sue zone. Il sì oltre la sessione delle app di oggi urta ADR-0016: segnalato, K38 | 2026-09-29 |

## Come si riprende — scritto alla chiusura della sessione del 2026-09-28

📌 **In corsa dal 2026-09-29:** quali domande hanno risposta lo dice la tabella *«Le risposte del proprietario»*, che **vince**
su questa sezione; la sezione si riscrive alla chiusura della sessione in corso.

⛔ **Da sapere subito: niente è a metà, ma D3 è senza risposta.** Albero pulito dopo il commit di questa chiusura, tutto
pushato, nessuno stash, nessun codice toccato. Il proprietario ha chiuso la sessione — *«continuiamo l'analisi e le domande
nella prossima sessione»* — sulla forma **riformulata** di D3, prima di leggerla: si ripone quella. La chiusura precedente sta
in [`archivio/consegna-brainstorming-knowledge-base-revisione.md`](../../archivio/consegna-brainstorming-knowledge-base-revisione.md).

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline f830cb9..HEAD`: l'avvio col puntatore della §6; il documento del proprietario col confronto; D2 con D3 posta; questa chiusura, con D3 riformulata |
| codice di prodotto | **non toccato**: `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`, all'apertura sull'albero di `f830cb9` e prima di ogni commit: si rilanciano, non si citano |
| il puntatore | la §6 del compendio: la revisione **prima** del 13 e dei modelli decisionali — non cambia con questa chiusura |
| fine-riga | questo file e l'archivio della consegna **LF**; compendio, archivio dello stato e `riferimenti.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script di questa sessione stanno nello scratchpad |
| la guida ARMS | **non** è nel repository: il riassunto sta nella sezione *«La fonte del documento»*, la provenienza in [`riferimenti.md`](../../riferimenti.md) |

**Dove si è arrivati.** Lo stato vive nelle tabelle di questo file; qui c'è solo dove guardare.

| | |
|---|---|
| la forma della knowledge base | il **documento del proprietario**: una root qualsiasi, due attori, il livello strutturale e quello semantico, il riconciliatore meccanico, le esclusioni, il confinamento, il controllo prima di scrivere |
| che cosa cambia del 2026-09-04 | la tabella *«Il documento contro ciò che esiste»*: le risposte 1, 3, 4 e 10 |
| le risposte | D1 respinta, D2 **A** — la tabella *«Le risposte del proprietario»* |
| la domanda aperta | **D3 riformulata**: il modello a due zone, e *«una repo aperta come zona di lavoro entra nel grafo e nella ricerca?»*, col consiglio **A** |
| le domande dopo | D4–D10, nella tabella *«Le domande, una per volta»* |
| i buchi | K1–K34: la tabella *«Lo stato dei buchi dopo il documento»* e quella dei casi nuovi |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, e il
   [disegno del 2026-09-04](2026-09-04-knowledge-base-design.md) per intero — la §12 del compendio lo chiede a chi riprende il
   fronte della knowledge base.
3. Rilanciare i comandi della tabella *«Che cosa esiste oggi»*, e quello di D3 sulla porta `filesystem`: il codice può essersi
   mosso.
4. **Porre D3 com'è scritta qui**, nella forma riformulata. Poi D4–D10, una alla volta: ciascuna si scrive **qui**, nella forma
   di D2 e D3, **prima** di porla — contesto, A/B col costo e ciò che si rifà, i cinque criteri, verificato, dedotto e assunto,
   il consiglio —; in chat a parole semplici, poi `AskUserQuestion` con due opzioni e il consiglio per primo.
5. A ogni risposta: la riga nella tabella delle risposte, lo stato dei K che tocca, il cancello, un commit, un push.
6. Finite le domande: la chiusura del brainstorming e, in una sessione **nuova**, il disegno. Scrive i richiami datati al
   disegno del 2026-09-04 — le risposte 1, 3, 4 e 10, le decisioni 13 e 15 —; i rimandi agli ADR che la revisione tocca,
   ciascuno riletto contro i fratelli, gotcha #59; le righe di `roadmap.md` e di `tracciabilita.md`, fra cui
   `Multi-repo/multi-progetto` e `Mappa del progetto`; e la voce della §5 del compendio per ogni ADR che riceve un rimando.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema. Costo: un `--amend` |
| 2 | le decisioni aperte 2 e 3 del proprietario **accolte** come le propone, senza domanda | coerenti con la risposta 10 del 2026-09-04 e con ADR-0024, e senza un'alternativa da porgli. Costo: una domanda, se le vuole riaprire |
| 3 | le prime forme di D1 e di D3 **non** vanno in archivio | sono domande, non verbali di correzione: stanno nei commit `e714720` e `9bdbb59`, nominati dove servono. Costo: zero |
| 4 | la guida ARMS in `riferimenti.md` come **origine dell'idea**, non come prova | è una guida di pratica, non una norma. Costo: zero |
| 5 | i casi nuovi **continuano** la numerazione, da K24 | un caso tiene il suo numero per tutta la revisione. Costo: zero |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **D1 com'era posta**: tre posti, con la knowledge base scritta dal solo assistente | il proprietario l'ha respinta e ha risposto col suo documento: la forma giusta non era fra le due opzioni. 📌 *Prima di porre una domanda sulla forma, chiedersi se la domanda assume una decisione vecchia — qui la risposta 3 del 2026-09-04 — che il proprietario può voler rovesciare* |
| **D3 nella prima forma**: la root come confine di tutto l'assistente | non reggeva col coding stile Claude Desktop, che apre una cartella qualsiasi — una capacità della roadmap, il 5, che il consiglio ignorava. 📌 *La prova «che cosa arriva» di `CLAUDE.md` si fa anche sul consiglio, non solo sullo schema* |

**Da verificare alla fonte prima del disegno** — le righe **F** delle tabelle: K6 e K34, i file «solo online» di OneDrive; K9,
gli eventi che il sorvegliante di Windows può perdere; e, per K1, le cartelle dati per utente dei due sistemi — quella di
Windows, e la specifica XDG per Linux.
