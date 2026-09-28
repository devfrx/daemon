# Knowledge base, la revisione: la consegna del brainstorming

⏳ **BRAINSTORMING IN CORSO, aperto il 2026-09-28.** Questo file è la **consegna** del brainstorming di revisione della
knowledge base, al percorso del suo futuro disegno — il precedente è la consegna dei
[modelli decisionali](2026-09-28-modelli-decisionali-design.md). Si aggiorna a **ogni risposta** del proprietario, nella
tabella *«Le risposte del proprietario»*, e si committa ogni volta: una sessione che muore non fa ripresentare niente.

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

## Le domande, una per volta

L'ordine va dalla forma ai dettagli: le prime tre dicono **dove**, le altre **chi e quando**. Ogni domanda si pone **sola**,
col contesto, A/B, i cinque criteri e il consiglio; la risposta va nella tabella *«Le risposte del proprietario»*, e si committa.

| # | Domanda | Chiude |
|---|---|---|
| **D1** | la forma: **tre posti** — i dati del programma, la knowledge base, le repo dove stanno — o **un posto solo** con tutto dentro? | K3, K4, K13, e metà di K1 e K2 |
| **D2** | **una macchina o più**: se la knowledge base deve seguire il proprietario, e come | K5, K6 |
| **D3** | **dove**, in concreto, stanno i dati del programma e la cartella della knowledge base, su Windows e poi su Linux | K1, K2, K6 |
| **D4** | **i cambi che il programma non vede** — a programma spento, fatti da altri, eventi persi: il confronto delle impronte all'avvio, e quando il sorvegliante perde il filo | K7, K8, K9 |
| **D5** | **i file-guida delle repo**: mai iniettati da soli; guida solo se importati e approvati | K15, con AUD-004 |
| **D6** | **la proiezione quando il modello cambia** per un fallback | K16 |
| **D7** | **due scritture sullo stesso file**: la regola della versione letta | K10 |
| — | il resto — K11, K12, K14, K17–K23 — si **registra** col suo chiusore, senza domanda, salvo che il proprietario la chieda | |

### D1, posta il 2026-09-28

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

## Le risposte del proprietario

| # | Risposta | Data |
|---|---|---|
| D1 | ⏳ posta, in attesa | 2026-09-28 |

## Come si riprende — scritto all'apertura del brainstorming, il 2026-09-28

⛔ **Niente è a metà.** Il commit di questo file porta anche il puntatore della §6 del compendio; albero pulito, tutto pushato.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| codice di prodotto | **non toccato**: `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`, all'apertura sull'albero di `f830cb9` e di nuovo prima del commit di questo file: si rilanciano, non si citano |
| il puntatore | la §6 del compendio: la revisione **prima** del 13 e dei modelli decisionali |

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
