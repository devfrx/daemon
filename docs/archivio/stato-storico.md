# Archivio — gli stati passati del progetto

⛔ **Non è una lettura obbligatoria.** Verbali datati, estratti dalla §6 del compendio
il **2026-08-28** dal piano di sfoltimento
([`../superpowers/plans/2026-08-28-sfoltimento-compendio.md`](../superpowers/plans/2026-08-28-sfoltimento-compendio.md)).

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Lo stato di oggi sta
nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md), e in nessun altro posto.

⛔ **Ciò che NON è qui:** i blocchi che nominano una voce **ancora aperta** sono rimasti
nella §6, parola per parola. La cernita è meccanica e provata in due direzioni, e su un
dubbio ha conservato.

---

✅ **I Traguardi 1 e 2 sono eseguiti — il 2026-08-08 e il 2026-08-10.** Il codice del prodotto
non è più zero righe: esiste il workspace, esiste la porta di qualità, la porta è **verde**, e
sopra di essa sta il **substrato iniettabile** al completo. ✅ **Il Traguardo 3 è eseguito il
2026-08-10: dodici compiti su dodici, `GATE GREEN` a tutti** — il **record durevole**
(`crates/kernel/src/record.rs`), la **riga di catalogo dell'etichetta di fiducia** col proprio
caso negativo, il **doppio in memoria del giornale** (`crates/simulator/src/journal.rs`), la
**suite di conformità** con **nove promesse in dieci blocchi e nove bugiardi**
(`crates/kernel/tests/journal_contract.rs` — ⚠️ **i bugiardi sono DODICI dal 2026-08-17**, e le
promesse restano nove: la cifra qui è quella che il Traguardo 3 consegnò), l'operazione **`replay()`** sulla porta, la
**riconciliazione** (`crates/kernel/src/reconcile.rs`), col Task 7 il **primo record che il
kernel scrive davvero** — la nota di `Untrusted::promote` — col **Task 8** la **seconda
implementazione della porta `journal`**: `redb` col **backend scritto da noi**
(`crates/platform/src/journal.rs`), col **Task 9** la conformità che gira contro **entrambe a
ogni commit** (`crates/platform/tests/journal_contract_real.rs`), col **Task 10** i **byte
congelati** (`crates/kernel/tests/frozen/`) e col **Task 11** `prune`, che rifiuta un passo **in
dubbio** e accetta uno riconciliato, e col **Task 12** il **registro riallineato** e questa
chiusura. ⚠️ **Questa cifra ha detto «nove» a dieci compiti eseguiti e «undici» a dodici**, e la
lista si fermava prima ogni volta: gotcha **#31**, ricontata il 2026-08-10 due volte.
⛔ **Una decisione presa in revisione, e va vista perché è un'aggiunta al contratto di una porta
condivisa:** un **secondo `intent`** sullo stesso passo è ora **rifiutato** — ADR-0007 dice
*«l'intento di ogni passo»*, uno per passo — e `OutOfOrder` si **allarga** invece di guadagnare
una variante vicina. ⛔ **I Task 4 e 5 sono stati eseguiti come uno solo**:
separati non funzionano — il Task 4 scrive una suite che chiama `replay()`, che la porta guadagna
solo al Task 5, quindi chiuderebbe con la **porta rossa**.

✅ **Il brainstorming del Traguardo 4 è chiuso il 2026-08-11, e il disegno è scritto:**
[`specs/2026-08-11-…-traguardo-4-simulatore-dst-design.md`](../superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md).
✅ **E il piano è scritto lo stesso giorno:**
[Traguardo 4](../superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md), **dieci
compiti in tre parti**. ✅ **E IL TRAGUARDO 4 È ESEGUITO lo stesso giorno: dieci compiti su
dieci**, subagent-driven, con le due revisioni e `GATE GREEN` a ciascuno. Il simulatore porta ora
**il guasto**: il giornale che cade e non si riprende, il **backend cadente di `redb`** scritto da
**fuori la crate**, due campagne con **soggetti diversi** — la riconciliazione del kernel e la
coerenza del motore — il gotcha **#51 chiuso nella metà chiudibile**, l'elenco dei semi che nasce
**vuoto e non dimenticato**, e il tempo di parete che il cancello **stampa a ogni corsa**.
⛔ **E il 2026-08-11 il repository ha ricevuto il suo primo AUDIT COMPLETO** — codice, script del
cancello, documenti, ADR, diagrammi — con nove revisori paralleli in sola lettura e ogni finding
grave riverificato sul sorgente. Il rapporto è
[`audit-2026-08-11.md`](../audit-2026-08-11.md), **e si legge prima di riprendere**.
✅ **Sedici finding corretti e provati nella stessa sessione**; il resto è **aperto e assegnato al
proprietario**. ✅ **E L'AUDIT È CHIUSO IL 2026-08-18, otto decisioni su otto** — il racconto di ciascuna sta
nei riquadri qui sotto. ⚠️ **Questa riga diceva *«il prossimo passo è l'ESECUZIONE
DELL'AUDIT — non il Traguardo 5»***, ed era una delle **quattro case di questo solo file**
che il `grep` del 2026-08-18 ha trovato ferme: il puntatore vivo sta **in cima alla §6**,
in un posto solo.

✅ **E LA PRIMA DELLE OTTO DECISIONI È ESEGUITA IL 2026-08-17 — T-2 e T-1, il finding più grave.**
La suite di conformità del giornale provava **tre** promesse su nove — la **1**, la **5** e la
**8** — soltanto nello stato in cui **ogni guardia plausibile passa**: archivio **vuoto**, o con
**un passo solo**. Chiuse con un **passante** in archivio: un passo **diverso** da quello sotto
esame è l'unico stato in cui *«l'archivio è vuoto»* smette di coincidere con *«questo passo non ha
un intento»*, e *«il record di questo passo»* con *«il primo record che c'è»*.
⛔ **E la notizia è quanto è costato, perché l'audit lo prezzava come un'altra cosa.** La §8 dava
la decisione 1 per *«un'aggiunta al contratto di una porta condivisa»*; letta contro il codice di
oggi non è servita **nessuna promessa nuova** e **nessuna riga di prodotto** è stata toccata — le
due implementazioni filtrano già per passo (`has_intent(step)`, `stored == step.get()`) e la porta
lo dichiara già (*«re-reads ONE step BY NAME»*, e le tre vie di `OutOfOrder`). A mancare non era il
contratto: era lo **stato che distingue una guardia sbagliata**. 📌 È la quinta domanda del
pre-controllo che ha pagato — *un compito scritto prima si legge contro il codice di adesso* —
applicata a un **rapporto d'audit** invece che a un piano.
⛔ **E i finding erano TRE e non due, che è il gotcha nuovo #65:** l'audit raggruppa le promesse 5
e 8a in un finding solo, e ha ragione sulla **causa** — `note` e `outcome` condividono `has_intent`
— ma la suite **muore alla prima promessa rotta**, quindi un bugiardo cieco su entrambe muore sulla
5 e il blocco della 8a resta **non provato mentre un test afferma il contrario**. Ne servono
**due**, e sono un tipo solo con **due istanze**, o sarebbero lo stesso difetto scritto due volte.
✅ **Il rosso è stato riprodotto PRIMA di correggere**, che è la disciplina di questo repository: i
tre bugiardi — **J14** `StepBlindJournal`, **J15** e **J16** `BlindGuardJournal` — contro la suite
com'era hanno risposto tutti e tre `THE SUITE IS VACUOUS ON promise 1 / 5 / 8`.
✅ **E la seconda direzione è misurata sulle IMPLEMENTAZIONI VERE e non sui soli bugiardi**, perché
un bugiardo prova che il blocco morde, non che il blocco raggiunga `redb`: **sei** mutazioni — le
due guardie e il predicato di `read_back`, su ciascuna delle due implementazioni — applicate **una
alla volta**, ciascuna compilata ed eseguita a sé e poi revocata. **Sei rosse su sei, ciascuna col
messaggio della propria promessa**; prima del rimedio tutte e sei lasciavano il workspace verde. A
campagna chiusa `git diff --stat` nomina **il solo file della suite**. La tabella sta in
[`porta-di-qualita.md`](../porta-di-qualita.md).
⚠️ **E due conteggi erano stantii PRIMA di questa passata**, trovati contandoli invece che
leggendoli: `journal_contract_real.rs` dichiarava che la suite chiama la fabbrica **nove** volte —
sono **dieci** dal giorno in cui `note` divise la promessa 8 in due blocchi — e **otto** bugiardi,
che erano nove dal Task 11. Gotcha **#31**.
📌 **Baseline dopo il rimedio:** `GATE GREEN`, `cargo test --workspace --no-fail-fast` →
**32 target, 177 passati, 0 falliti, 2 ignorati** (erano 171: tre test nuovi qui e tre nella copia
che `platform` include).
✅ **E LA SECONDA DECISIONE È ESEGUITA IL 2026-08-18 — G-5, `--locked` nel cancello.** Il
`Cargo.lock` **tracciato** era un **effetto** del cancello invece che un suo **ingresso**: un
manifesto derivato faceva ri-risolvere `cargo`, il lockfile veniva riscritto in silenzio, e
`gate-deps.sh` — che misura il grafo transitivo contro la lista di ADR-0031 — misurava allora **il
grafo che `cargo` aveva appena inventato**, credendo di misurare quello approvato.
⛔ **Riprodotto prima di correggere, cosa che il rapporto non aveva fatto:** tolta la riga di
`minicbor` da `crates/kernel/Cargo.toml`, il controllo rispondeva `OK -- the two graphs match the
two lists`, **exit 0**, col lockfile alleggerito di **33 righe**. La guardia di non-vacuità **non**
lo coglieva: i due grafi erano non vuoti e **diversi**.
⛔ **E il rapporto lo prezzava «una riga», cioè al rovescio della decisione 1:** i siti `cargo` del
percorso del cancello sono **più d'uno e stanno su tutti e tre gli script**, perché i due script si
lanciano **anche da soli**, e un controllo che vale solo passando dal cancello è più debole di uno
che vale sempre. ⚠️ **RICHIAMO DEL 2026-08-27, finding AUD-009: qui stavano una cifra e la sua
scomposizione, e la scomposizione smentiva la cifra nella stessa riga** — *«**sei** su **tre**
script — `gate.sh` ×4, `gate-no-os.sh` ×1, `gate-deps.sh` ×3»*, che somma **otto**. ⛔ **Tolte, non
riallineate:** la cifra vive dove vive il **comando** che la produce, cioè in
[`riferimenti.md`](../riferimenti.md), e ciò che regge qui è la **relazione** — *ogni sito eseguibile
passa `--locked`* — che resta vera il giorno in cui il cancello guadagna un passo. 📌 **Il rimedio si prezza leggendo il
codice, non il rapporto** — gotcha **#65**, e stavolta la misura lo ha fatto **crescere** invece
che restringere.
⛔ **E chiuderlo alla lettera avrebbe aperto una vacuità nuova.** Con `--locked` un `cargo tree`
che fallisce lascia **entrambi** i grafi vuoti, quindi coincidenti, quindi la guardia di
non-vacuità diventa rossa **da sola** — dicendo però *«la query era stretta»* dove la verità è
*«il lockfile è stantio»*. Il ramo d'errore esplicito compra la **diagnosi**, non il rosso, ed è
scritto così accanto al codice invece che taciuto. ⚠️ **E l'errore si mostra RI-ESEGUENDO invece
di unire `stderr` alla cattura:** un `cargo tree` che stampa *«Blocking waiting for file lock on
package cache»* darebbe a `names` la parola `Blocking`, che passa la sua classe di caratteri e
verrebbe riportata come **intrusa su I3** — un rosso per la ragione sbagliata, cioè il gotcha
**#41** spostato dallo stesso filtro alla sua sorgente.
✅ **Due direzioni misurate, sonde N6 e N7** di [`porta-di-qualita.md`](../porta-di-qualita.md).
**N6:** stesso guasto col rimedio → **exit 1**, messaggio che nomina il lockfile stantio, lockfile
**intatto**; cancello intero → **`GATE RED -- 5 checks failed`**, con `Cargo.lock` immobile per
tutta la corsa. **N7:** stato pulito → `GATE GREEN`, **32 target, 177 passati, 0 falliti, 2
ignorati**, `git status` vuoto. ⚠️ **N7 non prova che il rimedio morda** — anche prima quella corsa
lasciava l'albero pulito: serve a escludere che `--locked` renda rosso uno stato **corretto**, e le
due si leggono in coppia.
⚠️ **Il costo è dichiarato accanto al codice:** aggiungere o alzare una dipendenza è ora un atto in
**due passi** — il manifesto da solo lascia il cancello **rosso**, e il lockfile va rinfrescato
**fuori** dal cancello e committato **insieme** al manifesto. È il punto e non il prezzo: ADR-0031
chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*, e un lockfile che il cancello
aggiorna da sé non è né l'uno né l'altro.
⚠️ **E il banco ha dato la trappola dei fine-riga mentre la si applicava:** `sed -i` ha
**normalizzato CRLF → LF** su `crates/kernel/Cargo.toml` — da **43 CR a zero** — senza dirlo, che è
la riga di `CLAUDE.md` incontrata eseguendola invece che leggendola. Ripristino da una **copia
byte-esatta** presa prima, mai da `git checkout --` (gotcha **#48**, dodicesima forma), e i tre
script sono stati modificati con uno strumento di edit dopo aver **misurato** che i CR reggessero.

✅ **E LA SESTA DECISIONE È ESEGUITA IL 2026-08-18 — i quattro rimandi datati A-1, A-2, A-4,
A-7.** Nessuna decisione riaperta: cadono quattro **evidenze**, non quattro scelte.

| | Cosa diceva il documento | Cosa dice il codice |
|---|---|---|
| **A-1** | [ADR-0026](../adr/0026-linguaggio-del-core.md): *«esiste `madsim` … quindi il simulatore non va scritto da zero»* | `simulator` ha **una** dipendenza ed è scritto **a mano** — quante righe lo dà `find crates/simulator/src -name '*.rs' \| xargs wc -l`, e la cifra che stava qui è **tolta** il 2026-08-25 (vedi la riga **#59** della §9); `madsim` non è né in `Cargo.lock` né in `crates/`. E a scartarlo fu [ADR-0031](../adr/0031-dipendenze-del-kernel-parte-del-confine.md) — **stessa data** — a 55 crate |
| **A-2** | *«il seme diventa una regressione permanente»* | falsificata in ADR-0021 il **2026-08-08**: a entrare nella suite è la **proprietà**, non il seme |
| **A-4** | [`design/01`](../design/01-topologia-dei-processi.md): canale worker **a senso unico** | il tratto `Worker` ha **sei** verbi contati sul sorgente, e i frame **risalgono** — dentro una **ricevuta** |
| **A-7** | `OpenError`: *«la radice di composizione lo apre, una volta»* | `crates/daemon/src/main.rs` cabla `SequentialRng`, `SystemReactor`, `Parameters`, `Sleep` e l'esecutore — e **nessun giornale**. ⚠️ **RICHIAMO DEL 2026-08-27, finding AUD-008:** questa cella è il **verbale del 2026-08-18** e ciò che misurò era vero allora; **dal 2026-08-21 è falsa**. Il **Task 10** cabla `FileJournal::open` nella radice di composizione — commit `800ffeb`, con `StartupError::Journal` accanto — e la colonna si intitola *«Cosa dice il codice»*, cioè si legge al presente. ⛔ **Il sorgente lo sapeva e questa §6 no:** il doc di `OpenError` in `crates/platform/src/journal.rs` porta il proprio richiamo datato **dal giorno stesso**, e la correzione non ha attraversato la seconda casa — radice **R1**, gotcha **#68**. ✅ **L'argomento di A-7 regge intatto, e a cadere è la sola evidenza che lo illustrava:** `open` davvero non è un'operazione della porta, ed è una proprietà leggibile **oggi** |

⛔ **Due erano più larghi di come il rapporto li scrive, e in due modi diversi.** **A-2** viveva
in **due** case oltre ad ADR-0021 — questa §5 e [`design/08`](../design/08-strategia-di-test.md),
che *si dichiara fonte di verità sulla porta di qualità* ed è quindi l'ultimo posto in cui una
formulazione falsificata dovrebbe sopravvivere: ci è sopravvissuta **dieci giorni**, ed è la
radice **R1**. **A-7** non è una frase imprecisa ma una **previsione citata come misura**: *«la
radice di composizione lo apre»* è scritta al presente e si legge come un fatto, mentre parla di
codice **non ancora scritto** — gotcha **#57** applicato a una **giustificazione** invece che a
una collocazione. 📌 E il richiamo dice la cosa esatta: **l'argomento regge, l'evidenza no.**
`open` davvero non è un'operazione della porta, e quella è una proprietà leggibile **oggi**.
⚠️ **RICHIAMO DEL 2026-08-27, finding AUD-008 — e *«codice non ancora scritto»* è la parte che
è scaduta:** quel codice è scritto dal **2026-08-21**, e il racconto sta nella cella **A-7** qui
sopra, in una casa sola. 📌 **La diagnosi del #57 regge lo stesso, e va detto perché è il
punto:** una previsione citata come misura resta tale **anche quando la previsione si avvera** —
a cambiare è solo che adesso nessuno può più accorgersene leggendo.

✅ **E LA QUINTA È ESEGUITA LO STESSO GIORNO — C-1, e ciò che si esegue è la REGISTRAZIONE.**
`bincode` 2.0.1 è coperto da **RUSTSEC-2025-0141 — «Bincode is unmaintained»**, emesso il
**2026-01-07**, categoria **`INFO`**: non è una vulnerabilità, e le altre sette dipendenze sono
pulite. ⛔ **La notizia non è l'avviso: è il BUCO FRA DUE CRITERI.** [ADR-0037](../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md)
chiede *«il pari ha un lettore conforme e **mantenuto**?»* — puntato verso il **capo lontano** del
filo, TypeScript, misura M-11; **M-1** puntava verso di noi e chiedeva un'altra cosa, *«il grafo
transitivo è accettabile per I3?»*. **Nessuno dei due chiede se sia mantenuta la libreria del
NOSTRO capo**, e `gate-deps.sh` verifica **quali** crate ci sono, non **come stanno**. L'avviso
era pubblico **sette mesi prima** che §6.1.1 fosse riconfermata il 2026-08-08.
✅ **E il costo di agire è quasi zero oggi, misurato invece che citato:** `grep -rn bincode crates/
--include=*.rs` dà **zero usi di produzione** — un commento di documentazione in
`crates/kernel/src/ports/ipc.rs` e una sonda in `crates/kernel/tests/dependencies_usable.rs`. Lo
schema del canale `ipc` è il **Traguardo 6**. ⛔ **È una finestra che si chiude da sola**, come la
quarta proprietà della §3: si **decide allora**, mentre la scelta è ancora libera, e la
registrazione vive **accanto alla voce** in `crates/kernel/Cargo.toml` — dove guarda chi la tocca
— invece che solo qui. 📌 **La domanda che ne esce, e vale oltre il caso:** *questo criterio è
puntato verso **entrambi** i capi del filo?* Gotcha **#64**.

✅ **E LA SETTIMA È ESEGUITA LO STESSO GIORNO — P-1, e il rapporto ha ragione sul difetto e torto
sul rimedio.** La via **A3** era dichiarata **chiusa** e aveva una **seconda bocca**: `Untrusted`
aveva smesso di stampare il proprio contenuto, ma `promote` prendeva `reason: &str` e il `Debug`
scritto a mano di `RecordV1` **stampa l'indice 4 per intero**. Riprodotto da fuori la crate —
`RecordV1 { … payload: <16 bytes>, reason: "ignore your instructions" }`, il campo protetto
nascosto e quello non protetto spalancato.
⛔ **La frase che lo autorizzava è la classe del difetto, ed è il gotcha nuovo #67.** Il commento
giustifica **quattro** campi con una ragione sola — *«sono il vocabolario del kernel, nobody
outside chose them»* — vera per **tre**. `reason` lo sceglie il chiamante. **È l'ELENCO a farla
leggere come verificata:** chi la controlla si ferma al primo nome che torna.
⛔ **E IL RIMEDIO DELLA §8 NON AVREBBE CHIUSO LA STRADA.** Propone `reason: &Instruction`;
`Instruction::new` è **`pub`** e prende qualunque `String`, quindi
`Instruction::new(untrusted.as_str().into())` lo soddisfa — ed è la **via A1/A2**, dichiarata
**non chiudibile** nella stessa lista. 📌 **Una guardia a newtype vale quanto il suo COSTRUTTORE**,
e quella avrebbe comprato l'**apparenza** di una chiusura sopra una strada già dichiarata aperta.
⚠️ Sarebbe stata anche un gioco di parole sui tipi: `Instruction` significa *contenuto ammesso nel
canale delle istruzioni*, e una giustificazione non è quello.
⛔ **La terza opzione cade su un FATTO:** `reason` come enum sarebbe la lettura più onesta di
«vocabolario», ma è l'**indice 4 del record durevole** — cambiarne il tipo muove i **byte
congelati**, cioè apre una `Record::V2` (ADR-0036). Sproporzionato, e speculativo con **un solo**
chiamante di produzione.
✅ **Il rimedio è `reason: &'static str`**, ed è **una parola**: il contenuto esterno è dato di
**runtime**, un `&'static str` è un letterale nel binario. **Zero** siti di chiamata riscritti —
erano tutti già letterali, misurato — **un** oracolo `.stderr` aggiornato a mano, e il **formato
non si muove** (`frozen_bytes.rs` sei su sei). ✅ **La sonda è un caso `compile_fail` nella forma
forte del #42:** rimessa la firma a `&str` il caso **compila**, e `trybuild` risponde `error`
invece di `mismatch` — non lo disarma un `TRYBUILD=overwrite` in blocco.
⚠️ **Resta aperto e dichiarato:** `String::leak` dà ancora un `&'static str` — stesso scambio con
cui **A5** liquida il `transmute` — e un letterale può **mentire**, che è provenienza e non
correttezza, il limite che **A4** già dichiara. Ciò che ha chiuso è la strada che si prende **senza
accorgersene**. ⚠️ **Voce aperta registrata, non presa:** la riga di catalogo, come per K-1/B-1.
📌 **Baseline invariata:** `GATE GREEN`, **32 target, 180 passati, 0 falliti, 2 ignorati**; i casi
di `compile_fail` passano da diciassette a **diciotto**.

✅ **E L'OTTAVA È ESEGUITA LO STESSO GIORNO — la decisione 7, le cinque sonde mancanti. L'AUDIT È
CHIUSO: otto decisioni su otto.** Le cinque voci sono **quattro soggetti diversi**, e ciascuna è la
stessa forma di difetto — *un'asserzione vale solo lo stato in cui è fatta* — su una porta diversa.
⛔ **B-2, e la suite colpita è la più importante del progetto:** la conformità del `reactor`, su cui
poggia la validità dell'intera simulazione deterministica, aveva **due bugiardi per UN gruppo** —
cancellare i blocchi 1, 3, 4 e 5 lasciava il workspace verde. ✅ **Cinque bugiardi nuovi, uno per
ASSERZIONE e non per gruppo** (#65). ⛔ **Il gruppo 5 non ha asserzioni**, quindi era l'unico blocco
la cui cancellazione nessun oracolo poteva notare: un `wall_time` che esplode trasforma *«il blocco
esiste»* in *«il blocco gira»*. ⛔ **E scrivendo i bugiardi è uscito un difetto che l'audit non
aveva visto: l'asserzione 4b è IMPLICATA dalla 4a** — `second_deadline` è calcolata da
`first_reached`, quindi un bugiardo per la 4b **non è scrivibile**. Non è vacua, è **muta**.
Registrata e non presa: toglierla tocca la conformità di una porta condivisa.
⛔ **B-3:** i test tenevano **un solo** checkpoint, e lì *«trova per id»* e *«prendi il primo»* sono
la stessa frase — 13 su 13 verdi sotto la mutazione. 📌 **Il rimedio è il PASSANTE, identico a
quello della prima decisione di questo audit** sulla conformità del giornale: stesso difetto, porta
diversa. E due argomenti nel sorgente vi poggiavano — `CheckpointId` e `ClientId` non hanno getter
*«perché un'implementazione lo ritiene e lo CONFRONTA»*, un argomento su un confronto che nulla
osservava.
⛔ **S-1/S-2:** il percorso di **successo** di `CrashingJournal::note` non era **mai** preso — ogni
`note` del file rispondeva `NotDurable` — e *«il contatore si muove solo su un `Ok`»* era tenuto per
il solo `outcome`. 📌 **La sonda lo aveva scritto di sé stessa** — *«esclusività su un insieme che
cresce è l'affermazione che invecchia in silenzio»* — e ha invecchiato in **sette giorni**.
⛔ **E la terza mutazione ha trovato un buco nella sonda che avevo appena scritto:** controllava il
**contatore** e non che la nota **raggiungesse l'archivio**, e una `note` che risponde `Ok` senza
delegare muove il contatore ugualmente. Chiusa leggendo `replay()`. È il **#66** applicato a sé
stessi.
⛔ **S-5:** `partial > 0` è soddisfatta da **UN** gradino, mentre l'intera ragione per cui la
campagna di livello 2 è **profonda** invece che **larga** è la tabella dei pioli del disegno —
4/4, 11/11, 21/21, 31/31, 41/41 — che era **prosa**. ✅ **Ora è un'uguaglianza**, e la tabella regge
a entrambe le profondità: **`rungs=4/4`** sulla corta, **`rungs=31/31`** sulla profonda, rimisurate
invece che citate.
📌 **Baseline:** `GATE GREEN`, **32 target, 194 passati, 0 falliti, 2 ignorati** — erano 180. I
quattordici in più sono le nove sonde nuove, di cui le cinque del `reactor` contano **doppio**
perché quel file è `include!`d anche da `platform`.
⚠️ **Voce aperta CONSOLIDATA:** le **dieci** sonde permanenti che l'esecuzione dell'audit ha
prodotto **non hanno riga di catalogo**, e la §7.4 è spec (vincolo globale 7). Raccolte in una
tabella sola in [`porta-di-qualita.md`](../porta-di-qualita.md) invece che in quattro riquadri, perché
quattro voci aperte sullo stesso oggetto sono il modo in cui una smette di esserlo senza che
nessuno l'abbia chiusa.

✅ **Il brainstorming è chiuso il
2026-08-18, il disegno è scritto**
([`specs/2026-08-18-…-traguardo-5-arbitro-gpu-design.md`](../superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md),
dodici sezioni) **e lo stesso giorno è scritto il PIANO**:
[`plans/2026-08-18-…-traguardo-5-arbitro-gpu.md`](../superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md),
**tredici compiti in cinque parti**. Si esegue **subagent-driven**, un compito per volta con
revisione fra uno e l'altro.
⚠️ **Questa riga ha detto *«il TRAGUARDO 5, e si riparte dal BRAINSTORMING»***, poi *«il PIANO
del Traguardo 5»*, poi *«l'ESECUZIONE del Traguardo 5»*: è la riga che invecchia per costruzione, e vive **in un posto solo**.

✅ **E IL 2026-08-19 L'ESECUZIONE È COMINCIATA, `GATE GREEN` a ogni compito**, subagent-driven:
un subagente fresco per compito, una revisione fra uno e l'altro, e il pre-controllo delle sette
domande **prima** di ogni dispaccio. ⚠️ **Questa riga portava una SECONDA copia del numeratore** —
*«quattro»*, poi *«cinque»*, poi *«sei»* — ed è **tolta**, non riallineata: la cifra sta in cima
alla §6, in un posto solo.

📌 **I numeri, misurati e non dedotti:** `cargo test --locked --workspace --no-fail-fast` →
**37 target, 264 passate, 0 fallite, 2 ignorate** (erano **32 e 194** all'apertura del
traguardo); la campagna dell'arbitro porta **cinque** sonde in **un** bersaglio nuovo; i casi di
`compile_fail` sono **trentatré**, quattro dei quali dal Task 11 e **nessuno** dal Task 12; e
`cargo build --locked --workspace` è a **zero avvisi**, rimisurato ricostruendo le cinque crate.
⚠️ **RICHIAMO DEL 2026-08-24:** questa riga diceva **«35 target, 248 passate»**, ferma al Task 9,
mentre il riquadro in fondo a questa stessa sezione ne dichiarava **255**: la stessa cifra in
**due punti della stessa sezione** con **due valori**, che è la forma peggiore del gotcha **#31**.
A trovarla fu il pre-controllo del Task 11, che la **registrò senza correggerla** — toglierla
avrebbe lasciato penzolante l'inciso dei valori intermedi. ✅ Ora l'inciso è **tolto insieme alla
cifra**: i valori compito per compito vivono in [`porta-di-qualita.md`](../porta-di-qualita.md), e un
rimando non può marcire. ⚠️ L'altra casa **resta e non è una seconda copia**: dice *«la baseline
da cui il **Task 11** parte»*, cioè si data da sé.
⚠️ **I valori intermedi NON sono più elencati qui:** stanno in
[`porta-di-qualita.md`](../porta-di-qualita.md), compito per compito, e questa riga li ricopiava
allungandosi a ogni chiusura. **Tolti, non estesi** — un rimando non può marcire (gotcha
**#68**).
⚠️ **Quante sonde stiano in ciascun banco NON è più scritto qui:** il conteggio vive in
[`porta-di-qualita.md`](../porta-di-qualita.md), accanto alle sonde — che per giunta ne **rivendica
l'unicità** — e questa riga ne teneva una copia, cioè il gotcha **#68** fra due documenti.
**Tolta, non riallineata.**
⛔ **E il conteggio della baseline che
il piano scrive nei propri passi è STANTIO per costruzione** — dice *«32 target, 194 passati»*,
che era vero prima del Task 1: ogni compito **misura la propria** prima di cominciare, o
confronta contro un numero che i compiti precedenti hanno già spostato.

⛔ **LE CINQUE PREVISIONI DEL TASK 6 SONO STATE MISURATE: QUATTRO CONFERMATE, E LA QUINTA
SBAGLIATA NEL PROPRIO RIMPIAZZO.** Il riquadro che stava qui le elencava come **previsioni**
perché il compito non era stato eseguito, e si citavano come tali e mai come collaudo (gotcha
**#57**). Ora è eseguito, e si scrive che cosa è successo.

| | Previsione del pre-controllo | Esito **misurato** |
|---|---|---|
| **1** | il ramo della coda rompe **tre** sonde del Task 5, e il brief non lo dice | ✅ **confermata, tutte e tre.** `the_sum_of_the_grants_never_exceeds_the_total`, `a_total_smaller_than_the_two_permanent_quotas_…` e `a_grant_still_inside_its_window_is_not_collected` rispondono ora `Queued` dove asserivano `Refused`. Riscritte perché tenessero la **propria** proprietà con la forma nuova della risposta: l'asserzione su `allocated()` — che è il numero che porta la proprietà — **resta**, cambia la variante. Nessuna annacquata |
| **2** | e **una delle tre perde** la proprietà per cui era stata scritta | ✅ **confermata** — `E41`. Con la coda la seconda quota permanente viene **accodata e aspetta per sempre**, che è il degrado silenzioso vietato da ADR-0005 e ADR-0019; l'arbitro **non può ripararlo**, perché *«Permanence is not a type: it is nobody calls release»*. La visibilità si sposta alla **radice di composizione**, e il chiudente resta il **Task 10** |
| **3** | `Held.lane` **non** arriva al Task 6, e il commento accanto a `Held` dice il contrario | ✅ **confermata** — `E42`. `Held` resta a **due** campi; il biglietto in attesa vive in `Waiting`, che porta il `profile` intero, e `promote` non legge mai `held`. Il commento è **corretto con un richiamo datato**, non affiancato da una smentita: lasciarlo in piedi era il finding **A-2** di questo progetto rifatto |
| **4** | la mutazione `1c` va rimisurata, e `E28` prevede che acquisti senso | ✅ **confermata** — `E43`. Cancellata per intero la guardia `asked > ceiling`, `return` compreso, muore ora `a_request_larger_than_the_total_is_refused_and_not_queued`: **18 passati, 1 fallito, e sola**. Al Task 5 la stessa mutazione non uccideva **nessuno** |
| **5** | il conteggio *«tredici test»* del Passo 4 è stantio, **saranno sedici** | ⛔ **SBAGLIATA — sono DICIANNOVE.** Il rimpiazzo contava le **cinque** sonde dettate sulle undici esistenti; ne sono servite **otto**, perché la revisione ne ha chieste tre che il brief non prevedeva — la regola del non-scavalcamento, la riscossione dentro `promote`, e quella nata dal mutante vivo |

📌 **E la quinta è la lezione, non l'errore.** Una cifra stantia **colta** produce un rimpiazzo
che nessuno ridubita, perché chi l'ha corretta si sente il più attento della stanza. È la forma
psicologica del gotcha **#48** — *«chi aveva colto un errore di misura ha creduto al RIMPIAZZO
senza dubitarne»* — spostata su una **previsione** invece che su una misura, e la cura è la
stessa: si riconta **eseguendo**, anche — soprattutto — quando a scrivere il numero nuovo è chi
ha appena smontato quello vecchio.

⛔ **E IL TASK 6 HA INSEGNATO UNA COSA CHE VALE OLTRE IL CASO — il gotcha nuovo #74, e a trovarla
è stata la REVISIONE e non l'esecuzione.** La sonda che prova *«prima la corsia migliore»*
metteva `Interactive` contro `Batch`: ⛔ **la corsia in cima all'ordine — `Realtime`, quella per
cui la regola esiste — non era MAI costruita**, e il mutante che la salta lasciava **34 target su
34 verdi**, cioè sopravviveva all'intero workspace. Chiusa portando la sonda a **tre** corsie:
lo stesso mutante è ora **rosso e solo**, misurato. 📌 La domanda che lo coglie: *questa regola
ordina N cose, e la sonda ne nomina quante — e fra quelle nominate c'è quella in cima?*

⛔ **E il Task 6 è costato DUE giri di revisione, perché il secondo ha trovato difetti nella
PRIMA ONDATA DI CORREZIONI: è la terza occorrenza del gotcha #45.** Quell'ondata aveva chiuso due
rilievi **scrivendo il comportamento in un paragrafo di doc**, e nessuno dei due paragrafi era
tenuto da niente — mutanti `1d` e `4b`, vivi su tutto il workspace. ⚠️ **E la seconda metà del
difetto è del coordinatore:** quell'ondata stava per **non** essere rivista, con l'argomento *«non
tocca nessuna riga eseguibile»* — mentre i due rilievi del giro precedente stavano **entrambi
nella prosa**, cioè in ciò di cui un'ondata di correzioni è fatta. 📌 ***«È solo documentazione»*
è un argomento PER rivedere, non contro.**

⛔ **E IL DATO DEL TASK 7 NON È CIÒ CHE HA PORTATO: È QUANTO È COSTATO PROVARLO. Tre revisioni
piene più una stretta sulla prosa, e CIASCUNA ha trovato difetti reali.** ① La prima: il confine
`lane <= below` — la **guardia centrale** della funzione — non era chiesto da **nessuna** sonda,
in una campagna che i confini di tutte le **altre** guardie li aveva cercati apposta; mutante
**vivo sull'intero workspace**, `34 target su 34 verdi`. ② La seconda: la riscrittura in **due
passate** aveva creato una **promessa nuova** che nessuna sonda teneva, e il mutante che la
rompeva — il ciclo esterno fermato alla prima corsia — **ricreava il difetto che quell'ondata
esisteva per togliere**. ③ La terza: l'ondata mandata a estirpare una cifra stantia **ne ha
introdotta un'altra in sorgente vivo** — quattro conteggi che sommavano a dodici in un file la cui
suite ne ha tredici — e la sua tesi centrale era smentita dalla campagna dello **stesso commit**.
📌 **Per TRE ondate di fila il difetto è atterrato nella PROSA e mai nel codice**, e la terza non
ha toccato una riga eseguibile. È il gotcha **#45** alla **quarta** occorrenza, e la forma nuova
va scritta perché non è quella già registrata: *un'ondata di correzioni ha lo stesso tasso di
difetto del compito, e i suoi difetti stanno dove nessun test guarda*.

⛔ **E LA TERZA REVISIONE HA ISOLATO UNA CAUSA CHE NON ERA MAI STATA SCRITTA — il gotcha nuovo
#75.** Un'asserzione era stata **dichiarata vacua** accanto al codice, con la misura a sostegno;
rimisurata guardando **quale** asserzione va in panico invece che se la sonda sia rossa, era
l'**uccisore solo** su due righe di campagna — `5b` e `5d` — dove le due asserzioni sopra di lei
**passano**. ⛔ **La misura che la dichiarava vacua aveva campionato SOLO le mutazioni di
`ask_back`**, cioè precisamente la classe sotto cui quell'asserzione non può fallire, e non aveva
mai eseguito le due righe in cui è portante. 📌 *Un'esclusività misurata su un campione parziale
si legge come una garanzia.*

⚠️ **E una TERZA cosa, che nessun pre-controllo aveva previsto — né quello del piano né quello
del coordinatore:** un **`#[cfg(test)] mod` non conta come lettore per `dead_code`**, quindi
mettere le sonde **dentro** la crate non spegne gli avvisi che la visibilità ristretta produce.
È la conseguenza della strada scelta, e nessuno l'ha vista prima di misurarla.

⛔ **E IL TASK 13 HA CHIUSO IL TRAGUARDO, ED È UN AUDIT: ciò che ha VERIFICATO conta più di ciò
che ha scritto.** Il compito prescriveva **cinque** passi: i primi due hanno prodotto **zero**
scarti con ciò che i documenti già scrivevano, e la condizione del Passo 4 sulla §4 **non
scatta**. È il gotcha **#49** alla terza occasione, riconosciuto invece che rieseguito alla
lettera.

| | Che cosa il Task 13 ha fatto | Esito, misurato il 2026-08-25 |
|---|---|---|
| **Passo 1** | rieseguire i conteggi **col comando** | `GATE GREEN`; `cargo test --workspace --no-fail-fast --locked` → **37 bersagli, 264 passate, 0 fallite, 2 ignorate**; **33** casi `compile_fail` e **33** `.stderr`; `git status --short` vuoto. ⛔ **Zero scarti** con ciò che i documenti già scrivevano: non c'era niente da riallineare |
| **Passo 2** | ricontare le **dodici** righe di catalogo **sulla §7.4 della spec** | **regge**: `3` nel blocco B dei gettoni, `8` nel blocco C, `1` di livello 2. Delimitato per **intestazione** e non per posizione. Stato: **undici chiuse**, e la dodicesima — la campagna DST — **dichiarata PARZIALE** con l'innesco scritto (`E152`) |
| **Passo 3** | le voci aperte in **una** tabella sola | ⛔ **l'unica cosa che mancava davvero**, ed è la condizione **8**, quella che il piano aggiunge alle sette del disegno. Scritta in [`porta-di-qualita.md`](../porta-di-qualita.md) |
| **Passo 4** | il compendio, e il puntatore | il `⏭️` passa al **Traguardo 6**, e il censimento `grep -rn "⏭️" docs/ CLAUDE.md` è stato eseguito leggendo **ogni riga intera**: **una** sola è il puntatore vivo, **sei** sono rimandi che nominano la §6 senza nominare il passo, e il **resto** è testo di gotcha, verbali storici, prescrizioni e comandi. `CLAUDE.md` ne ha **zero**. ⛔ **Il TOTALE non è scritto, ed è una decisione:** era **24** prima di questa passata e **29** dopo, perché le righe scritte oggi sono a loro volta case del `grep` che censisce — e un totale che il proprio commit sposta non può essere verde in nessun mondo |
| **Passo 4 bis** | la **§4** guadagna l'arbitro **solo se** c'è stata una decisione di stack | ⛔ **la condizione non scatta, e la §4 non si tocca:** `git diff --stat 26a529c..HEAD -- '*Cargo.toml' Cargo.lock` è **vuoto** su tutti i commit dell'esecuzione, e le otto decisioni del piano sono forme di firma e di modulo, non strati dello stack |

⚠️ **La Definizione di «fatto» è stata riletta contro il codice prima di usarla**, come il piano
prescrive di sé — al Traguardo 2 pretendeva *«otto casi `compile_fail`»* dove erano quattordici.
**Nessuna delle otto condizioni è invecchiata**, e la 4 regge sul conteggio ricontato. La **5** ha
un limite che va detto: l'appaiamento `.rs` ↔ `.stderr` è verificabile con un comando, ma
*«letto e non rigenerato in blocco»* **non ha un controllo**, e la difesa dichiarata accanto al
codice è che la rigenerazione **si legge nel diff**.

⛔ **E una decisione che il Task 12 aveva assegnato a questo compito è PRESA:**
[`semi-dst.md`](../semi-dst.md) **non guadagna una riga** per la campagna dell'arbitro, e il perché
è scritto lì. La regola del file è che una voce nasce quando **una campagna fallisce**, e la
campagna dell'arbitro non ha fatto fallire nessun caso: un'entrata senza una proprietà violata
sarebbe la falsa sicurezza che il riquadro in testa a quel file esiste per impedire. Ciò che
quel file guadagna è il **fatto** che le campagne ora sono tre.

⚠️ **E ciò che il Task 13 NON ha toccato, con la ragione, perché tacerlo sarebbe peggio:**
[`riferimenti.md`](../riferimenti.md), che sta fra i `Modify:` del compito — voce `E146`, decisione
del proprietario, e le misure di questa passata vivono nel registro; e
[`tracciabilita.md`](../tracciabilita.md), che non nomina il Traguardo 5 in nessuna riga — misurato
con `grep -i "traguardo 5\|arbitro\|arbiter"`, **zero** — e che comunque si aggiorna alla
chiusura del **sotto-progetto**.

✅ **E IL 2026-08-18 UNA PASSATA DI COERENZA HA TOLTO LA RIGA CHE MARCIVA, invece di
correggerla per la terza volta.** Il puntatore al prossimo passo e il **conteggio delle
decisioni d'audit** vivevano **riscritti**: il puntatore in **cinque** documenti di stato, il
conteggio in **quattro**. ⛔ **Ricontate invece che stimate: il puntatore aveva NOVE case** —
di cui **quattro in questo solo file** — e il conteggio **sette**, con **tre valori distinti**
(*«ne restano tre»*, *«quattro»*, *«cinque»*) contro il vero, che è **zero**: ciascuno fermo
alla decisione che l'aveva scritto. ⛔ **[`HANDOFF.md`](../HANDOFF.md) da solo ne teneva quattro,
con tre valori diversi**, e una di esse portava per giunta un conteggio di **eseguite** —
*«due»* — falso allo stesso modo; la §12 di **questo** file era la settima.
⛔ **E la §8 dell'audit stesso dava TRE decisioni per non eseguite** — la 2 (P-1), la 3 (K-1) e
la 7 (le sonde) — mentre il commit che le chiudeva si intitola *«otto decisioni su otto»*: il
verbale non era stato timbrato. Timbrate ora, e ciascuna col **prezzo vero** del rimedio.
⚠️ **La regola per evitarlo esisteva già, scritta due volte.** `CLAUDE.md` dice *«lo stato
corrente e il prossimo passo stanno nella §6 — non qui, o si disallineano»* — e lo dice
**quarantasei righe sotto** una riga propria che dava l'audit per prossimo passo; e la 25ª
misura della §12 prescrive per intero *«si cerca `grep '⏭️'` su tutti i documenti di stato e
si guardano tutte le case»*. 📌 **Un promemoria non è un controllo**, ed è la **terza**
occorrenza della radice **R1** su questa identica riga: finding **D-1**, poi il 2026-08-17,
poi oggi. 📌 **E la regola violata viveva DENTRO il documento che la violava** — gotcha
**#68** — mentre lo strumento che ha applicato il rimedio ha dato il **#69**.
✅ **Quindi il rimedio non è ricorreggere le case: è TOGLIERLE.** I documenti secondari ora
**rimandano** alla §6 invece di riscriverla — e un rimando non può marcire. ⛔ **E la prova di
quale forma regga era già nel repository, non è un'opinione:**
[`AVVIO-CHAT.md`](../AVVIO-CHAT.md) è l'unico documento che su questa riga non è **mai** marcito,
ed è l'unico che **si rifiuta di nominare il prossimo passo** dichiarandone
la ragione. 📌 È la 25ª misura applicata a un secondo oggetto: *decidere cosa **togliere**,
non cosa accorciare*.
⚠️ **Il conteggio delle passate è TOLTO da questa riga il 2026-08-20** — diceva *«in nove
passate»* e ne sarebbero dieci: è un numero che cresce a ogni consegna, e ne esiste **un
gemello** in [`riferimenti.md`](../riferimenti.md), cioè lo stesso conteggio in **due** documenti
(gotcha **#68**). Il *mai* è il fatto, e regge da solo.
⚠️ **Ciò che NON è stato toccato, e per una ragione:** lo **stato per traguardo** resta nelle
tabelle di [`roadmap.md`](../roadmap.md) e [`README.md`](../README.md). È il loro mestiere, non è la
cifra che è marcita, e toglierlo sarebbe stato invasività senza guadagno — il perimetro di una
passata si prende dal **drift misurato**, non dalla categoria.

✅ **E IL 2026-08-18 IL BRAINSTORMING DEL TRAGUARDO 5 È CHIUSO, E IL DISEGNO È SCRITTO** —
[`specs/2026-08-18-…-traguardo-5-arbitro-gpu-design.md`](../superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md),
**dodici sezioni, 31 KB**. Il perimetro: **l'arbitro intero**, e si prova ciò che si può provare
**senza un secondo meccanismo**.
⛔ **La decisione che governa le altre, e nasce da una formulazione corretta prima di scrivere.**
Delle cinque proprietà della §5.7, tre si iniettano su porte che esistono e due su porte che non
hanno implementazione — ma *«due non si possono fare»* era troppo grosso: quelle due sono **due
metà incollate**, e la metà d'**arbitro** si prova oggi. 📌 **E le due metà d'arbitro sono UNA
SOLA** — *rilasciare restituisce la riserva* — perché all'arbitro non serve sapere **chi** tiene
una concessione: dargli un «titolare» lo accoppierebbe a `ipc` per una cosa che il Traguardo 6
risolve con una mappa dalla propria parte.
⛔ **E il fatto che decide la collocazione di `Grant` è MISURATO, non dedotto:** un modulo
**fratello** di `ports::process` **non può costruire** `Grant` — `error[E0423]`, riprodotto su una
crate usa-e-getta. Quindi il tipo **si sposta nel modulo `arbiter`**, che è chi lo emette; un
costruttore `pub(crate)` dove sta ora avrebbe comprato l'apparenza — *una guardia vale quanto il
suo costruttore*, gotcha **#67**.
✅ **E la notizia buona: il Traguardo 5 NON CREA righe di catalogo, ne CHIUDE dodici già scritte**
— tre nel blocco B dei gettoni, otto nel blocco C, una di livello 2. Contate sul catalogo §7.4 e
non dedotte. Fra queste, le **quattro righe di §6.10.5**, che il **Task 11** ha chiuso.
⚠️ **RICHIAMO DEL 2026-08-21:** qui seguiva la **ragione** di quel fermo — *«perché senza
`Grant` non si ottiene un `Worker`»* — ed è **falsa** e **tolta**: finding **P-2**, chiuso lo
stesso giorno.
⛔ **E il disegno ha trovato una cifra sbagliata in due documenti di stato: «cinque delle nove
righe di guasto».** Contate sulla fonte — la §7 del disegno del Traguardo 4, che è la tabella che
gli indirizzi li **assegna** — sono **una** al Traguardo 5, **una** condivisa e **sette** al
Traguardo 6. 📌 **Da dove viene il cinque:** la §5.7 della spec ha esattamente **cinque** righe, ma
sono le **proprietà che la DST verifica**, non le righe di guasto. Due tabelle diverse, e la cifra
dell'una letta contro l'altra. ⚠️ **Registrata e non presa:** `CLAUDE.md` prescrive di
**toglierla**, non di ricorreggerla, ed è una decisione del proprietario.
⚠️ **Due divergenze dalla lettera della spec, dichiarate perché il proprietario possa ribaltarle
vedendole:** la §5.1 dice *«i tre addendi sono parametri consegnati»* e il disegno ne consegna
**uno** — gli altri due sono la riserva di due **concessioni permanenti**, perché *«la sottrazione
non è un'esenzione»* (gotcha #4) e metterli in `Parameters` darebbe due campi che nessuna decisione
del kernel legge; e la contro-sonda della riga `Q8` nomina una **proiezione di presentazione** che
non esiste, quindi a leggere `cold_start` sarà una **finta**.
📌 **Baseline prima di cominciare:** `GATE GREEN`, **32 target, 194 passati, 0 falliti, 2
ignorati**, albero pulito.

⛔ **Cosa l'audit ha trovato, e la prima voce è la più grave** — ✅ **chiusa il 2026-08-17, vedi il
riquadro qui sopra.** La suite di conformità provava
**V6 solo su un archivio VUOTO**: sostituendo le guardie di `FileJournal::outcome` e `::note`
con *«l'archivio è vuoto?»* invece di *«questo passo ha un intento?»*, `cargo test --workspace`
dà **32 target, 171 passati, ZERO falliti** — e la mutazione **è osservabile**, perché su un
archivio non vuoto accetta un esito e una nota per passi **mai aperti**. È il gotcha **#63**.
⛔ **E due buchi del cancello lo lasciavano verde col confine caduto**, entrambi corretti e
provati coi codici d'uscita veri: `build = 'gen.rs'` fra apici singoli sfuggiva al controllo dei
build script (**#61**), e `check-docs.sh` non verificava che la spec **esistesse**, con le sei
asserzioni di §8.6.1 che vivono in blocchi `END` di `awk` — e `END` non gira (**#60**).
⛔ **Tre documenti di stato dicevano che il Traguardo 4 era da fare**, eseguito da un commit.
📌 **Sei gotcha nuovi, #59–#64**, e il più utile è il **#59**: un ADR può essere falsificato da un
ADR **fratello della stessa data**, e nessuna delle quattro domande del pre-controllo lo coglie
perché guardano tutte il compito contro il **codice**, mai un ADR contro i **fratelli**.
✅ **E ciò che l'audit ha trovato SANO va detto**: le due campagne DST sono solide — la quarta
occorrenza del difetto di vacuità **non c'è** — nessun segreto in centosettantuno commit, il
grafo spedito è esattamente quello dichiarato, e i **quattordici** conteggi di test di
[`porta-di-qualita.md`](../porta-di-qualita.md) sono giusti **tutti e quattordici**.
⛔ **E il disegno ha ricevuto un richiamo PRIMA che il piano fosse scritto, perché il codice lo ha
smentito su due punti** — §11 del disegno: `CrashingBackend` vive in un **banco di prova** di
`platform` e non in `src/`, perché ciò che il Task 8 comprò è che il confine sia raggiungibile **da
fuori la crate**, e il precedente di `abandon_without_commit` **non trasferisce** (quel metodo è
`pub` perché *non* è scrivibile da fuori); e non può avvolgere `redb::InMemoryBackend`, che tiene i
guardiani **privati**, quindi l'archivio **non si riaprirebbe** — e riaprirlo è l'intera domanda del
livello 2. ⚠️ **La prima risposta era già scritta in un commento** di
`crates/platform/tests/file_journal.rs`; la seconda si vedeva solo leggendo la libreria. Nessuna
delle due si vedeva rileggendo il disegno, che era coerente con sé stesso.

⛔ **Cosa ha deciso il disegno, e la prima decisione governa le altre: il Traguardo 4 costruisce
il MOTORE della DST, non tutte le finte della §3.1.** Il fatto che decide è un conteggio, ed è
stato ottenuto leggendo la §3.3 contro il codice di **oggi**: delle **dieci** righe di guasto,
**una sola** ha il proprio soggetto — la caduta fra intento ed esito sulla porta `journal`,
la cui riconciliazione esiste dal Traguardo 3. Le altre nove iniettano un guasto dentro un
meccanismo che **non esiste**: l'arbitro è il Traguardo 5, il canale worker e lo stato di
degrado il Traguardo 6. Costruirle ora è la decisione **D1 del piano del Traguardo 3 al
rovescio**, più il gotcha **#46**.
⚠️ **E il Traguardo 4 non porta il determinismo — quello c'è dal Traguardo 2: porta il guasto.**
**C1, C2, C3** e la **non-vacuità** sono già test permanenti in
`crates/kernel/tests/executor_determinism.rs`; mancano **C7a** e **C7b**, il giornale cadente,
il backend cadente, la campagna e l'elenco dei semi. ⚠️ Un caso è a metà e la distinzione conta:
`a_crash_leaves_more_than_one_step_in_doubt` tiene la **proprietà** su uno stato costruito **a
mano**, non su uno spazio di semi.
⛔ **E i due livelli di crash sono DUE CAMPAGNE con soggetti diversi** — livello 1 esamina la
riconciliazione del kernel, livello 2 la coerenza dopo crash di `redb` — che è la ragione per cui
ADR-0032 collocava male il backend cadente. Le nove righe scoperte hanno **ciascuna il proprio
indirizzo** nella §7 del disegno: un arretrato con un indirizzo è uno scaglionamento.
⚠️ **Questa riga diceva «il prossimo passo è il brainstorming»**, e prima ancora «il Task 12»:
è la riga che invecchia per costruzione, e si riscrive **quando il passo si chiude**, non dopo.
⛔ **E il Task 11 ha lasciato due voci aperte MISURATE, non supposte, in
[`porta-di-qualita.md`](../porta-di-qualita.md).** (1) **ADR-0018 pretende che un payload potato e
uno mai registrato non siano indistinguibili, ed entrambe le implementazioni lo violano**: dopo
la potatura rispondono lo stesso `Err(Missing)`, spariscono entrambe da `replay`, e una seconda
`prune` non le distingue. ⚠️ **La via che sembrava non costare l'impronta è stata cercata e la
misura la uccide:** svuotare il payload lasciando la voce rende i due distinguibili e non costa
nessuna promessa, ma `steps_in_doubt` risponde allora **`SuspendAndAsk`** su un passo riconciliato
e potato — byte vuoti sono indecifrabili, e un record indecifrabile rimette il passo in dubbio —
quindi il sistema si fermerebbe su **ogni** passo potato, a **ogni** ripresa. Una traccia utile
dev'essere **leggibile dalla riconciliazione**, cioè una decisione di **formato**: appartiene al
traguardo della ritenzione, **insieme** alla decisione sull'impronta (una voce nuova in ADR-0031).
(2) `prune` ha **tre** risposte e la conformità ne tiene **due**: `Missing` per un passo mai
scritto è tenuta solo dal doppio in memoria. ⛔ **E `JournalError` ha una QUARTA variante,
`StepInDoubt`, che è un'aggiunta al contratto di una porta condivisa:** non allarga `OutOfOrder`
perché quello è definito da **V6** — *«tutte e tre le sue vie sono V6»*, dice il suo doc — e potare
troppo presto viola **ADR-0018**, un invariante diverso; e perché il chiamante deve distinguere
*«hai rotto il protocollo»* (un difetto) da *«questo passo non è ancora potabile»* (una spazzata
di ritenzione ordinaria). ⛔ **`FileJournal` ha dovuto cambiare il proprio archivio per rispondere**
— la tabella `redb` guadagna il **byte dell'operazione** — perché contare i record è sbagliato:
una **nota** non è un esito.
⛔ **E le tre scadenze di E33 sono scattate col Task 10:** da quel commit nessun campo nuovo di
`RecordV1` può essere obbligatorio — dev'essere `Option` con `#[cbor(default)]` e un indice
nuovo — nessun indice esistente può cambiare significato, e una variante nuova dei tre enum
smette di essere gratis. Stanno scritte accanto al codice, in testa a
`crates/kernel/tests/frozen_bytes.rs` e in `crates/kernel/src/record.rs`.
⚠️ **Questa riga diceva *«il prossimo passo è il Task 7»* fino al 2026-08-10**,
mentre il Task 7 era eseguito da un commit e la riga gemella trenta righe più sotto diceva già
**Task 8**: due frasi di stato **nella stessa sezione** che si contraddicevano, e il gotcha
**#31** nella forma che costa di più — chi legge la §6 dall'alto trova la prima. ✅ **La misura
del Task 8 diceva che il Task 9 sarebbe stato una formalità, e lo è stata a metà:** la suite
girava già verde contro `FileJournal`, e infatti ha compilato e passato al primo colpo — ma la
**fabbrica** dettata dal piano non reggeva, perché cancellava un percorso fisso in una cartella
condivisa (gotcha **#52**) contro un'implementazione che tiene un **lucchetto** e una suite che
la chiama **nove volte**. Il valore del compito sta nelle **contro-sonde**, non nel verde. Il
Traguardo 1 ha lasciato questo:

| | |
|---|---|
| **il workspace** | alla radice, `resolver = "3"`, edition **2024**, `spikes/` fra gli `exclude`. Le cinque crate di §1.2 esistono tutte |
| **`kernel` e `simulator`** | `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]`, e **nessuna logica di prodotto**: è deliberato, non un lavoro lasciato a metà |
| **la porta** | un comando solo — `bash scripts/gate.sh` — con **sei** controlli: build del workspace · test · cancello senza OS · allow-list sui due grafi · **attributi delle crate vincolate** · coerenza della documentazione. La CI lancia lo stesso comando, `.github/workflows/quality-gate.yml` |
| **la mappa dei controlli** | [`porta-di-qualita.md`](../porta-di-qualita.md): ogni riga del catalogo §7.4 → il file che la implementa, con le sonde per nome e ciò che **non** è ancora coperto |
| ⛔ **quattro trappole nuove** | gotcha **#38**, **#39**, **#40**, **#41**, più una **seconda occorrenza** di #26 e una di #25. Non erano deducibili: sono uscite eseguendo |

⛔ **Una revisione ha aggiunto una riga al catalogo il 2026-08-09, ed è stata misurata.** Il
quinto controllo della porta — `gate-attributes.sh` — ne copre ora **due**: che le crate
vincolate **dichiarino** i propri attributi, e che **non abbiano un build script**. Un
`crates/kernel/build.rs` che chiama l'orologio, il filesystem e l'ambiente e inietta il
risultato con `cargo:rustc-env` lasciava la porta **verde su sei controlli su sei** —
`build` e `test` lo compilano perché è il mestiere di un build script, il cancello senza OS
lo compila **per l'host** e lo **esegue**, l'allow-list non vede nodi nuovi, gli attributi
leggevano il solo `src/lib.rs`. Difende **I3 e V29 direttamente**, non è di ramo 1b: è il
gotcha **#28**, un valore del mondo cotto dentro il kernel. Il catalogo **§7.4.2 passa da
dodici a tredici** voci di livello 2; il registro è [`porta-di-qualita.md`](../porta-di-qualita.md).
📌 Lo stesso commit ha rinominato il workflow in `quality-gate.yml`: era l'**ultimo residuo
italiano nel codice**, e costava zero solo finché non fosse mai stato eseguito.

⛔ **Il piano ha ricevuto un'errata in testa, e non si riscrive.** La prima voce sono gli
identificatori: il piano li detta **italiani**, il codice eseguito è in **inglese** perché
lo impone la §1.0 della spec — «spec prima del codice», e il piano non aveva l'autorità per
derogarvi. Le altre tre sono, in ordine, la seconda occorrenza di **#26**, quella di
**#25**, e il gotcha **#39**.

Le sette voci sono emerse rileggendo `tracciabilita.md` con una domanda che nessuno le
aveva posto: ***«di quale meccanismo di kernel ha bisogno questa funzionalità, e la
spec lo nomina?»***. La crepa è la **legenda**: `📋` significa «sotto-progetto
assegnato», **non** «non richiede un meccanismo di kernel».

| Voce | Cosa | Stato |
|---|---|---|
| **F3** | i parametri di decisione non erano consegnati al kernel | ✅ chiusa — ADR-0034, §2.8 |
| **F6** | la VRAM totale non aveva provenienza | ✅ chiusa con F3, §5.1 |
| **F5** | `network` era «verso i provider», V25 promette «verso la **rete**» | ✅ chiusa — §2.3.1 |
| **F1a** | nessuna porta per **parlare** con un worker | ✅ chiusa — ADR-0035, §2.3.1 |
| **F2** | l'evoluzione del formato durevole del giornale | ✅ chiusa — ADR-0036, §4.9 |
| **F7** | fork e branching | ✅ chiusa con F2 — §4.9.5 |
| **F1b** | il **progetto** della porta `process` in §5–§6 | ✅ chiusa — ADR-0037, §6.10 |
| **F4** | l'anello 3 non è collocato in §0.4 | ✅ chiusa — §0.4.3 |

### L'ordine, già deciso

| | |
|---|---|
| **la porta `network` ha una tensione non conciliata** | la firma di `request` è **sincrona e bloccante** — restituisce la risposta intera, quindi qualcuno ha atteso — mentre la regola scritta sotto dice che la prontezza viene dal `reactor` e che **nulla attende dentro `network`**; e l'esecutore di questo kernel è **cooperativo**. Le due si conciliano **il giorno in cui la porta riceve un'implementazione**: finché non ha chiamanti nulla blocca, quindi non è ancora un difetto. La domanda è scritta **aperta** in `crates/kernel/src/ports/network.rs` |
| **il residuo dichiarato su `Untrusted::promote`** | il meccanismo compra **una cosa sola**: che la conversione non si scriva senza **nominare** la porta `journal`. **Non** compra che qualcosa sia stato registrato — `promote` è generico su qualunque `Journal`. Sono **sette** le vie che aggirano il confine e **compilano**, tutte elencate in `crates/kernel/src/boundary.rs`. ✅ **Ricontate il 2026-08-10 col Task 7, e sulle voci invece che dedotte:** ne sono chiuse **tre** — il `Debug` di `Untrusted` (**A3**, livello 1), la conformità `journal_contract.rs` (**A6**, livello 2) e ora l'**etichetta di fiducia nel record** (**A4**, livello 2). ⛔ **A4 è chiusa a livello 2 e non «al formato», ed è la differenza che conta:** la via come `boundary.rs` la scrive passa da **byte grezzi**, non da un `Record` — `Record::decode` risponde `Malformed` e l'andata-e-ritorno funziona lo stesso, perché la porta scambia byte. L'etichetta chiude la via **per chi passa dal formato**, e nulla oggi impone che ogni scrittura sul giornale sia un record. ⛔ **A6 restava chiusa a metà finché la suite non girava contro DUE implementazioni.** ⚠️ **Ricontata il 2026-08-10 col Task 8:** la seconda **esiste** — `platform::journal::FileJournal` — e la suite le gira contro **verde, otto promesse su otto**, misurato; ma il file che la esegue **dentro** il repository era il **Task 9**, e *«misurato una volta»* non è *«tenuto a ogni commit»*. ✅ **Ricontata una terza volta il 2026-08-10, col Task 9: quella metà è CHIUSA.** `crates/platform/tests/journal_contract_real.rs` tiene le due implementazioni alle stesse promesse **a ogni commit**, con tre contro-sonde su tre promesse diverse e una mutazione di controllo. ⚠️ **Questa riga diceva «otto promesse», ed erano otto al Task 9:** il Task 11 ha portato la **7b**, quindi sono **nove promesse in dieci blocchi e nove bugiardi**, ricontati eseguendo il 2026-08-10 — gotcha **#31** su un numero che è cresciuto sotto la frase che lo conteneva. ✅ **Ricontati una QUARTA volta il 2026-08-17, chiudendo T-1 e T-2: i bugiardi sono DODICI, le promesse restano nove.** Ed è la notizia per A6: la suite non è cresciuta di promesse, è cresciuta di **stati** — su tre di quelle nove girava solo dove ogni guardia plausibile passa. A6 resta comunque una regola di **livello 2** — nulla impedisce di scrivere un `Journal` che la suite non incontri mai. ✅ **E le quattro vie che restano — A1, A2, A5, A7 — sono TUTTE dichiarate non chiudibili**, quindi ciò che resta non è un arretrato ma il **pavimento**: è la notizia vera di questo riconteggio |
| **la tesi della porta `process` la tiene l'implementazione, non il compilatore** | *«ogni byte che risale è coperto da una ricevuta»* è la frase su cui la porta è costruita — ma `SingleReceipt::new` e `StreamReceipt::new` sono **`pub`, e devono esserlo**: chi implementa `Worker` è `platform`, cioè un'altra crate, e Rust non ha una visibilità che arrivi fin lì e non oltre. Quindi **una ricevuta si può forgiare**, ed è la ragione per cui `close` deve poter rispondere `UnsolicitedFrame` pur andando core→worker. Il limite è scritto accanto ai due costruttori in `crates/kernel/src/ports/process.rs`. ⛔ **Il contrasto è con `Grant`**, che il costruttore non ce l'ha e la cui garanzia è davvero del compilatore. A chiudere la differenza sarà la **suite di conformità** della porta, che pretende due implementazioni: **Traguardo 6** |
| **`Ipc::accept` non ha un canale d'errore, e il prezzo di dargliene uno è la firma** | `accept` restituisce `Option<ClientId>` e **non può fallire**: nessuna delle due varianti di `IpcError` lo raggiunge — `Disconnected` è un'affermazione **su un `ClientId`**, e `accept` è l'unico metodo che un `ClientId` non lo prende. Corretto oggi, e «nessuno in attesa» è lo stato **ordinario**: la gui è 0..1 e sacrificabile. ⛔ **Ma un _ascoltatore_ rotto — che non è un client — arriverebbe come `None`, cioè un valore sbagliato invece di un errore** (gotcha #30). ⛔ **E il prezzo di chiuderlo va detto giusto, perché la prima stesura lo sbagliava:** aggiungere una terza variante **non basterebbe**, non c'è dove restituirla — costa la **firma**, `Result<Option<ClientId>, IpcError>`, che è la forma che `receive` già usa. Oggi la firma resta perché un `Result` che non può mai essere `Err` è superficie morta. Dichiarato in `crates/kernel/src/ports/ipc.rs`, in testa al file come in `network.rs` |

⛔ **E ce n'era una sesta, che questa tabella non ha mai elencato — chiusa il 2026-08-10, ed è
il gotcha #40 su una questione invece che su una decisione.** `Record::encode` restituiva
`Result<Vec<u8>, RecordError>` con la questione dichiarata **solo accanto alla funzione**, in
`crates/kernel/src/record.rs`: quell'`Err` era **irraggiungibile**, misurato al Task 1 sui tipi —
`Vec<u8>` come `Write` di `minicbor` ha `Error = Infallible`, e le altre due strade (`Message`,
`Custom`) hanno due soli produttori nella 2.3.0, `SystemTime` e un `Path` non-UTF-8, nessuno dei
due nel grafo di questo tipo. ⛔ **Decisione del coordinatore, non del piano: la firma è
`pub fn encode(&self) -> Vec<u8>`**, ed è la stessa posizione già presa per `Ipc::accept` —
*«un `Result` che non può mai essere `Err` è superficie morta»*. Pesano due cose che
`Ipc::accept` non ha: al **Task 7** `promote` lo chiama, e un `.expect` che non può sparare
**dentro il confine dei dati non fidati** è debito; e i chiamanti sono **pochi** oggi e molti
dopo. ⚠️ **Questa riga diceva «due», e il numero era sbagliato:** contati invece che ricordati,
i file chiamanti erano **uno** — `crates/kernel/tests/record_shape.rs`, con **nove** siti — e il
secondo che era stato contato, `compile_fail/record_without_version.rs`, **non è un chiamante**:
nomina `RecordV1::encode`, e la ragione per cui quel caso esiste è che quel metodo inerente
**non c'è**. ✅ L'errore va **a favore** dell'argomento, non contro.
`RecordError` **resta**, ristretto a `decode`. ⚠️ Registrata come **E22** nell'errata del piano,
perché il proprietario possa ribaltarla vedendola. ⚠️ **E il difetto vero non è la firma: è che
una questione aperta nel sorgente non compariva qui**, quindi per chi legge non esisteva — la
tabella qui sopra si popola a mano, come le decisioni.

### Cosa il Traguardo 3 lascia aperto, in un posto solo

### Il Traguardo 2, compito per compito — tutti eseguiti

| # | Compito | Stato |
|---|---|---|
| 1 | i due tempi, `Monotonic` e `WallTime`, e il terzo che li lega | ✅ |
| 2 | la porta `Rng`, e l'implementazione seminata in `simulator` | ✅ |
| 3 | i parametri di decisione, consegnati e non letti | ✅ |
| 4 | la porta `Reactor` | ✅ |
| 5 | **l'esecutore** | ✅ |
| 6 | il reattore finto, e la misura dell'interlacciamento | ✅ |
| 7 | il reattore reale in `platform`, e la prima suite di conformità | ✅ |
| 8 | il cablaggio di produzione in `daemon`, coi default letterali | ✅ |
| 9 | il confine dei tipi, e la promozione che pretende il giornale | ✅ |
| 10 | le porte `filesystem` e `network` | ✅ |
| 11 | `process` coi gettoni e le due ricevute | ✅ |
| 12 | `ipc`, e la tabella completa delle sei famiglie | ✅ |
| 13 | il registro dei controlli — e **il compito era già eseguito**, gotcha **#49** | ✅ |
| 14 | la chiusura del traguardo nei documenti di stato | ✅ |

✅ **I due buchi che il Task 6 aveva lasciato in eredità sono chiusi — ma uno dei due NON era
chiudibile dove era stato assegnato.** Il ramo `deadline <= now → None` di
`VirtualReactor::wait_until` è ora esercitato dalla conformità in **entrambe** le metà, `==` e
`<` (sonde R3 e R4). ⛔ `VirtualReactor::wall_time()` no: la conformità gira contro **tutte e
due** le implementazioni, quindi può asserire solo ciò che **entrambe** promettono, e i due
orologi che si muovono insieme sono una proprietà **della finta** — la vera serve `wall_time`
dall'orologio di sistema, che NTP fa arretrare. Metterla in conformità avrebbe reso **rossa
un'implementazione corretta**. È il gotcha **#44**: il buco si è chiuso in
`crates/simulator/tests/virtual_clock.rs`, e in conformità è rimasta una riga che prova **la
sola chiamabilità**, dichiarata come tale.

📌 **Sei difetti del piano trovati eseguendo, non leggendo**, e il più grave è invisibile
per costruzione: la cella `Sleep` veniva svuotata **solo sul ramo `Pending`**, quindi
un'attività che chiedeva di dormire e poi finiva lasciava la richiesta alla successiva.
**C1 resta verde** — la fuga è deterministica, quindi riproducibile e perciò invisibile a un
controllo di riproducibilità. Regressione permanente su un intervallo di semi, non su uno.

📌 **Il Task 7 ne ha aggiunti cinque, e quattro sono stati colti _leggendo_ il piano prima di
eseguirlo:** l'asserzione su `wall_time()` che sembrava copertura ed era `let _ = …` · la metà
`<` del ramo `deadline <= now` mai esercitata · il `catch_unwind` che accettava **qualunque**
panic invece del proprio · `SequentialRng` che nasceva **senza un test**. ⛔ **Il quinto è
uscito solo dalla revisione, ed è il più istruttivo:** il caso aggiunto per chiudere il
secondo era a sua volta **cancellabile lasciando la porta verde** — gotcha **#45**. ⚠️ E due
mutazioni sono sopravvissute a tutto: una è stata chiusa (`wall_time()` della vera, sonda
**R5**), l'altra è un **residuo dichiarato** invece che un test, perché distinguerla
richiederebbe un controllo non deterministico — e un controllo che scatta a caso è peggio di
uno assente.

📌 **Il Task 8 del Traguardo 3 ne ha aggiunti due, ed entrambi vengono da una misura che ha
smentito chi la faceva.** ⛔ **#51** — una garanzia sulla **morte del processo** non è
osservabile da dentro il processo: `set_durability(Durability::None)` lascia **sei test su sei
verdi**, e non è una lacuna del banco ma la forma del banco. ⛔ **#52** — un difetto di
parallelismo **mascherato dal sistema operativo**: la cancellazione della cartella condivisa
avviene **tre volte su sei**, ma Windows rifiuta di cancellare un file aperto, quindi il rosso
esce su **Linux**, che è il secondo sistema previsto dal progetto. ⚠️ **E una terza occorrenza
del #45:** il rimedio scritto per chiudere una vacuità (`abandon_without_commit` che risponde
`Result`) era **esso stesso vacuo**, e l'ha detto una mutazione — chiuso spostando il controllo
**dentro** il metodo, dove la transazione è ancora aperta.

### Il sotto-progetto 1 si esegue a traguardi, e ciascuno ha il proprio piano

Scrivere ora un piano per codice che non esiste significa inventare. **I Traguardi 1 e 2 sono
eseguiti; quelli dal terzo in poi si scrivono quando si arriva.** ✅ **Il piano del Traguardo 3
è scritto il 2026-08-10**, ed **eseguito lo stesso giorno** — subagent-driven, un
compito per volta con revisione fra uno e l'altro: **dodici su dodici**, `GATE GREEN` a tutti.
⚠️ **Ricontati il 2026-08-10 chiudendo il traguardo:** questa riga diceva *«sei su dodici»*
mentre la §6 era a **undici** e la tabella qui sotto a **otto** — la stessa cifra in **tre** posti
di questo file con **tre valori diversi**, che è la forma peggiore del gotcha **#31** e quella che
il riquadro precedente prometteva di non ripetere. ⚠️ **Ricontati il 2026-08-10:** questa riga
diceva *«due su dodici»* mentre la §6 e la tabella qui sotto erano già a cinque — la stessa cifra
in tre posti, aggiornata in due.

⛔ **Perché la porta di qualità viene prima della logica, e non è pedanteria:** un cancello
costruito **dopo** è un cancello che nessuno ha mai visto fallire, e la §7.1.1 dice che
allora non è un cancello. Su uno scheletro vuoto ogni controllo si prova in **due**
direzioni al costo di poche righe; dopo, la seconda direzione diventa cara e si smette.

📌 **Due scelte prese dal piano**, perché la spec non le fissa e allora costavano zero:
**edition `2024`** su tutte e cinque le crate, e un **`rust-toolchain.toml`** che dichiara
versione e **bersaglio del cancello**, così che il vincolo 4 della §11 si soddisfi da solo
su una macchina pulita. ✅ **Entrambe sono nel repository**, e stanno in §4.

✅ **L'ultima decisione aperta è chiusa — §7.1.1, il 2026-08-08.** Le otto righe del catalogo
§7.4 la cui colonna «Difende» non nominava un `V`, un'`I` o un `Q` non erano un problema solo:
**cinque** sono state **ri-attribuite** (`Q8` · `V29` · `Q2` · `I2` · `V29`), e **tre** —
`forbid(unsafe_code)`, il grafo **di build**, i **test di contratto** — non difendono una
proprietà del sistema ma **il verdetto di altri controlli**. Per queste la regola 1 ha ora un
**ramo 1b**, e `check-docs.sh` una **sesta asserzione** che la verifica. Dettagli in §7.1.1.

### §8 — ✅ chiusa. Cosa ha trovato, in quattro righe

| | |
|---|---|
| **le sette voci non hanno cambiato nessun voto** | hanno cambiato **cosa una cella nomina**. Riallineate `Q4`, `Q5`, `Q14` e la riga `process` di §8.2.2; `V25`, `Q20` e `V29` **rilette e lasciate come stavano** |
| ⛔ **il catalogo aveva saltato cinque controlli** | i cinque della §6.10.5 (F1b) non erano in §7.4, quindi `Q4` non poteva nominarli senza violare §8.1.2. Sono entrati — due gettoni in §7.4.1 B, due voci in C, uno di livello 2 in §7.4.2. **Non sono controlli nuovi**: §8.5.4, gotcha **#36** |
| **e non era gotcha #32** | cercato prima di scrivere: ADR-0037 non nomina mai il catalogo, la chiusura di F1b lo colloca in altre sei sezioni, §7.4.4 riduce tre voci che sono altre. **Mai valutato, mai scartato** |
| **due ritratti di conteggi erano stantii** | §8.8 e §7.4.7. Ricontati sulla tabella, non dedotti |

### L'audit sezione-contro-ADR — ✅ 2026-08-08. Quaranta rilievi, e uno ha ribaltato un voto

Undici revisori in sola lettura, uno per sezione più due sulle formulazioni di V e Q,
ciascuno contro gli ADR e le fonti. **Nessuno stato ⛔ e nessuna decisione riaperta**; tutte
le correzioni portano il proprio richiamo datato.

| | |
|---|---|
| ⛔ **`V16` torna a ⚠️ `parziale`** | la colonna «Vincolo» di §8.3 lo riportava **troncato** — mancava *«nomi di provider e parametri **sì**»*, la metà **positiva**, che è verificata qui dallo stesso test a esempi di `V15` e `Q14`. Il declassamento di §8.5.3.1 era corretto sulla metà che aveva davanti. **§8.5.5** |
| **otto formulazioni di vincolo erano troncate** | `V5` `V16` `V25` `V28` `V30` `V31` `V34` `V36`. Sette innocue, una no. È il gotcha **#29** spostato dalle invarianti alla tabella che le giudica: una riga di verifica stretta lascia scoperto un caso, una **riformulazione** stretta cambia l'oggetto del giudizio |
| ⛔ **due firme erano impossibili** | `Worker::istruisci → Ricevuta` restituiva un tipo che non esiste altrove, mentre le letture prendono `RicevutaSingola` e `RicevutaFlusso`: o era l'enum che la decisione vieta, o **non c'era modo di ottenere un flusso** — cioè l'audio. Ora le istruzioni sono **due** (§6.10.2) |
| **`daemon` non monta il simulatore** | la tabella §1.2 diceva *«sceglie `platform` o `simulator`»*, il grafo accanto non aveva quell'arco, e ADR-0034 dà ragione al grafo: in simulazione il cablaggio lo fa **il banco**. Era l'unica ambiguità strutturale sul verso delle dipendenze, ed è quella che il piano deve tradurre in `Cargo.toml` |
| **`network` era rimasto stretto in §3.1** | F5 aveva allargato la cella in §2.3 e non la gemella nella tabella che si dichiara *«esattamente le porte della §2.3»*: l'esportazione OTLP sarebbe nata fuori dall'unico punto di uscita |
| **sei conteggi stantii** | «tre ADR» in §0.1 (sono sette) · «due regole restano test» in §1.6 (tre) · «cinque ADR che nominano un parametro» in §2.8 (otto) · «cinque regole» in §4.9.2 (sei) · «due voci spedite» in §7.4.4 (tre) · «nove voci di livello 2» in §7.7 (undici) |
| **tre ADR hanno ricevuto un rimando** | **0021** — «il seed è un caso di regressione *permanente*» non regge, protegge la **proprietà** · **0032** — «la lista del kernel resta vuota» è falsa, e lo era già alla sua data · **0035** — «nessun gettone nuovo da inventare», F1b ne ha portati due |
| ✅ **la voce che restava aperta è chiusa** | **otto righe del catalogo** avevano una colonna «Difende» che non nominava un V, un'I o un Q. Chiusa il 2026-08-08: cinque ri-attribuite, tre nel **ramo 1b** della regola 1 — sostengono il **verdetto** di altre righe, non una proprietà. La regola non è più un'intenzione: sesta asserzione di `check-docs.sh`, provata su 8 rosse e 25 verdi alla prima corsa. §7.1.1 |

### F1b — ✅ chiusa. Cosa ha deciso, in sei righe

| | |
|---|---|
| **la tensione di `design/01` si scioglie con un gettone** | *«il worker non risponde di iniziativa propria»* contro *«il flusso audio risale al core»*: ogni byte che risale è coperto da una **ricevuta**, e le ricevute le emette solo un'istruzione. Un frame che nessuna ricevuta copre è un **guasto**, non un dato. Quarto uso del dispositivo di §6.3.1 |
| **la vita del worker sta in un oggetto solo** | l'avvio restituisce il `Worker`, ed è l'unico modo di parlargli; `uccidi` lo **consuma**. Istruire dopo l'uccisione non compila. I2 resta al compilatore |
| **due tipi di ricevuta, non un enum** | singola e di flusso: «una risposta singola diventa un flusso» non è **esprimibile**. Costa una funzione di lettura in più |
| **il formato è `minicbor`, e la porta scambia byte** | perché il pari **Python non sa leggere `bincode`** — ADR-0037, M-10. Voce già spedita: la lista di ADR-0031 **non cresce**. Col byte sulla porta il simulatore esercita davvero la codifica |
| **due regole uscite dalla misura** | il frame **dichiara la propria lunghezza** e la decodifica verifica i byte consumati (gotcha #34) · ogni `Vec<u8>` porta l'**annotazione di stringa di byte**, o il flusso audio raddoppia (gotcha #35) |
| ⛔ **e una divergenza registrata** | l'istruzione diceva di allargare le giustificazioni di `bincode`. Con la misura **non si allargano**: `bincode` serve il solo canale gui, e ad allargarsi sono le righe di `minicbor`. Gotcha #15 |

### F4 — ✅ chiusa. E la classe attesa era sbagliata a metà

La classe attesa era **C**. Scritta invece che assunta, si è spaccata in **due**:

| Pezzo | Regola |
|---|---|
| il **registro dei trigger**, e l'apertura di una run da un evento | **C** — nessun consumatore finché non esiste una capacità che parta da un evento, e la DST prova Q2, Q4 e Q5 aprendo le run direttamente |
| che ogni **sorgente di eventi** entri da una **porta dichiarata**, e che si dica quale | **B** — §3.1 dichiara le porte esaustive e il simulatore le sostituisce tutte: una sorgente scoperta dopo è **una porta aggiunta dopo la campagna**, e nulla diventerebbe rosso |

**Le due righe della tracciabilità che vi pendevano hanno una porta:** *Scheduling* e
*File watching* entrano entrambe da **`reactor`** — la prima è già coperta dal tempo
virtuale (§3.2), la seconda è **dichiarata** con implementazione scaglionata, la stessa
postura di `network`. Sta su `reactor` e non su `filesystem` perché ciò che deve essere
deterministico è **quando arriva la notifica**, non quale percorso.

✅ **Le famiglie di porte restano sei**, ed è la ragione per cui F4 è costata una
sotto-sezione invece di una riscrittura. 📌 **Per la §8 non produce nessuna riga nuova:**
**V29** copre già le sorgenti di eventi, e lo dice la sua stessa riga di verifica — *«C1
fallisce a ogni sorgente nascosta»*. Lo stato non cambia.

### ✅ Dove nasce il workspace — deciso dal piano, ed **eseguito**

Era l'unica domanda strutturale che la spec aveva deliberatamente lasciato al piano.

| | |
|---|---|
| **il workspace nasce alla radice**, da zero | `Cargo.toml` con `crates/{kernel,platform,secrets,simulator,daemon}`. Niente si eredita |
| ⛔ **`spikes/` è fra gli `exclude`** | `spikes/rust/` è a sua volta un **workspace annidato** e porta un `clippy.toml` che a livello di workspace scatterebbe addosso a `platform`, che *deve* chiamare l'orologio — vincolo 5 |
| **«punto di partenza» significa che si copia** | la §2.5 dice riga per riga cosa entra in `crates/kernel/` e cosa **resta** negli spike. Nel Traguardo 1 non era salito niente. ✅ **Onorata dal Traguardo 2, verificata riga per riga il 2026-08-10:** è salito tutto ciò che la §2.5 gli assegnava — `boundary.rs`, `rng.rs` con la seminata in `simulator`, `executor.rs`, la porta `journal`, i casi di `compile_fail`. Restano negli spike le due righe che la §2.5 dichiara **non** debbano salire, e il **doppio cadente** del giornale, che è del **Traguardo 4** |

---

## AUD-013 — il racconto, archiviato il 2026-08-28

⚠️ **Verbale.** Stava in [`../COMPENDIO.md`](../COMPENDIO.md) §6 e ne è uscito perché
quella sezione dichiara di sé che la casa unica dello stato dei finding è la colonna *Stato* di
[`../audit-2026-08-27.md`](../audit-2026-08-27.md). ⛔ **Ciò che era APERTO non è stato
archiviato con esso:** `E64` vive ora come voce **X-4** nella tabella delle voci senza numero AUD
di quel rapporto. Le altre due voci registrate qui sotto avevano già una casa propria e la
conservano — la forma nuova del gotcha **#48** sta in [`../HANDOFF.md`](../HANDOFF.md) **dentro il
#48**, e la voce su [`../riferimenti.md`](../riferimenti.md) è la **41ª misura** di
[`misure-dimensioni.md`](misure-dimensioni.md).

📌 **Le misure che hanno deciso, prese il 2026-08-28** — cifre dentro un verbale datato, quindi
reggono; nel compendio vivo non ce n'è nessuna, e il margine si rifà col comando
`wc -c docs/COMPENDIO.md` contro il tetto scritto in `scripts/check-docs.sh`:

| | |
|---|---|
| il blocco tolto dalla §6 | **4788** byte |
| compendio prima · dopo | **203 053** → **199 298** byte |
| tetto del cancello | **225 280** byte |
| margine prima | **22 227** byte, cioè **quattro** blocchi di quella taglia |
| finding ancora aperti allora | **ventitré** |

⚠️ **E la prima stesura del rimando le aveva scritte nel compendio VIVO** — «4886», «22 227»,
«ventitré» — cioè la radice **R1** commessa dentro il rimedio che la combatte, e con un numero
già sbagliato: **4886** era la misura del blocco *prima* dell'estrazione, il blocco vero ne pesa
**4788**. Corretta prima del commit. 📌 **Una cifra invecchia fra il momento in cui la misuri e
quello in cui la scrivi**, che è il caso limite del gotcha **#31**.

✅ **AUD-013 È CHIUSO IL 2026-08-28 — radice R1, il doc della sonda
`a_grant_inside_its_grace_keeps_its_reservation` in `crates/kernel/src/arbiter/mod.rs`.** Il
paragrafo affermava *«neither `ask_back` nor `revoking()` runs a sweep»* e ne concludeva che una
spazzata capace di liberare subito una prenotazione revocata *«would satisfy every other probe
here»*. ⛔ **Misurate, cadono tutt'e due:** `ask_back` riscuote con `self.collect_expired(now)` come
**prima istruzione**, e quella mutazione uccide **due** sonde e non una — `11 passate, 2 fallite`
contro `13 e 0` — perché `asking_back_twice_does_not_buy_the_room_twice` cade sul proprio
`assert_eq!(arbiter.revoking(), 1)` **prima** dell'`admit` che ha nel corpo, sulla spazzata in testa
al proprio **secondo** `ask_back`. La clausola falsa e le due esclusività sono **tolte e non
sostituite**, col richiamo datato; le due metà **vere** — `allocated()` e `revoking()` non
riscuotono — non sono state riscritte.

⛔ **E il rimedio era PIÙ PICCOLO di come il rapporto lo prezza: delle tre clausole di quella frase
ne era falsa UNA.** È il gotcha **#65** — *un rimedio si prezza leggendo il codice, non il rapporto,
in entrambe le direzioni* — su un altro rapporto e un'altra radice. ⚠️ **È la direzione che costa
di più:** un finding che descrive un paragrafo come sbagliato fa riscrivere il paragrafo, e
riscrivere ciò che è già vero è il modo più rapido di aprire una falsità nuova.

⛔ **Ed è costato TRE giri di revisione: i primi due hanno trovato difetti veri, il terzo è tornato
pulito.** ① Il primo ha colto che il rimedio aveva **aperto una falsità nuova** — un'esclusività
senza misura — dentro la prosa scritta per chiuderne una, e che una **quarta casa** era rimasta:
il paragrafo *«Che cosa se ne fa»* di [`porta-di-qualita.md`](../porta-di-qualita.md), dove la stessa
frase viveva una seconda volta **nello stesso file** che il censimento credeva di aver aperto
(gotcha **#68**). ② Il secondo ha colto che **due proposte del primo revisore erano a loro volta
FALSE**, smentite misurandole. 📌 **Una proposta di revisore è un'affermazione come le altre, e si
prezza leggendo il codice:** è il **#65** applicato a chi rivede, e sta scritto qui invece che in
una riga nuova della §9.

**Tre cose registrate chiudendo la voce, e due di esse non sono prese:**

| | |
|---|---|
| ⛔ **una QUINTA casa, ed è del proprietario** | la voce **`E64`** del [piano del Traguardo 5](../superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) afferma la stessa falsità. **Non toccata:** l'audit dichiara i piani fuori perimetro — *«sono verbali di esecuzione, non contratti»* — e questa §6 ha già stabilito che i due piani e il disegno del Traguardo 5 **restano come sono**. ⚠️ **Ma qui la difesa è più DEBOLE che per una scheda d'audit, e va scritto:** la scheda **cita** la frase come reperto del finding, `E64` la **afferma in proprio, come causa, al presente** — un verbale che registra una **misura** invecchia onestamente, uno che afferma un **meccanismo** sbagliato resta sbagliato. ⛔ **E la riga corretta di [`porta-di-qualita.md`](../porta-di-qualita.md) manda ora il lettore proprio là** per le cifre, che sono invece legittimamente datate. 📌 Se un verbale che afferma un meccanismo sbagliato vada corretto è **decisione del proprietario** |
| ⚠️ **una FORMA nuova del gotcha #48, e non una riga nuova** | ⛔ `git show HEAD:<file>` **non è evidenza sui fine-riga dell'albero di lavoro**, ed è la misura che si prende per prima: rende il blob dell'**indice**, per quel file LF, e chi eseguiva ne ha concluso `CR=0` per [`porta-di-qualita.md`](../porta-di-qualita.md) — che nell'albero di lavoro è **CRLF integrale** — «ripristinandolo» a LF. La **direzione** del difetto era capovolta: la conversione osservata andava verso LF, non verso CRLF. 📌 I `CR` si contano **sul file**, `tr -cd '\r' < <file> \| wc -c`, e la colonna che dice la verità è la **`w/`** di `git ls-files --eol`, non la `i/`. Il testo integrale sta in [`HANDOFF.md`](../HANDOFF.md), **dentro** il #48: la cura non cambia — *il banco con cui misuri sbaglia verso l'attesa* — e un gotcha che non insegna niente diluisce quelli che insegnano |
| ⚠️ **[`riferimenti.md`](../riferimenti.md) NON toccato, ed è una decisione** | la voce ha portato misure — la mutazione col suo esito, il censimento dei fine-riga — e vivono in [`porta-di-qualita.md`](../porta-di-qualita.md), **accanto al controllo che difendono**. È la **voce aperta della 41ª misura**, riaffermata da ogni passata da allora: scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario, e cominciare adesso produrrebbe **due** convenzioni invece di una |

---

## I tredici passi chiusi dei Traguardi 1–5 — archiviati il 2026-09-01

⚠️ **Verbale, non stato.** Questo elenco stava nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md)
e ne è uscito il **2026-09-01**: ogni voce era già **barrata e chiusa**, fra il 2026-08-08 e il
2026-08-25. È qui **parola per parola** — nulla è stato riassunto, nulla cancellato.

⛔ **Perché è uscito:** il tetto di `scripts/check-docs.sh` lasciava ventun byte al compendio
il 2026-09-01, e questo blocco solo ne pesava la maggior parte della §6 dopo *«Il prossimo
passo»*. Il criterio di taglio è quello del disegno del 2026-08-28: **resta in §6 ciò che è
vero adesso**.

1. ~~**§8**~~ — ✅ **chiusa il 2026-08-08**, toccata una volta sola come previsto.
2. ~~**Il piano**~~ — ✅ **scritto**: [Traguardo 1](../superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md).
3. ~~**Il codice del Traguardo 1**~~ — ✅ **eseguito** subagent-driven, otto compiti più quattro di riallineamento alla §1.0. `GATE GREEN`.
4. ~~**Il piano del Traguardo 2**~~ — ✅ **scritto il 2026-08-09**: [Traguardo 2](../superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md), quattordici compiti in due parti.
5. ~~**Il codice del Traguardo 2**~~ — ✅ **eseguito il 2026-08-10**, `GATE GREEN`. **Tutti e quattordici i compiti** subagent-driven, fra il 2026-08-09 e il 2026-08-10, con `GATE GREEN` a ogni compito; alla chiusura `cargo test --workspace` dà **72 target verdi e zero fallimenti**, e dentro il banco `compile_fail` esegue **quattordici** casi via `trybuild`. ✅ **Col Task 12 le sei famiglie di porte sono complete**: `reactor` · `journal` · `filesystem` · `network` · `process` · `ipc`, e §3.1 le dichiara esaustive. ⛔ **E il Task 13 ha scoperto di essere già eseguito**: dettava di *aggiungere* al registro [`porta-di-qualita.md`](../porta-di-qualita.md) righe che i Task 1–12 vi avevano già scritto a ogni passo, e a mancare non era l'aggiunta ma il **riconteggio** — gotcha **#49**. ✅ **E la voce che restava aperta è chiusa il 2026-08-09:** il caso `no_conversion_from_untrusted_to_instruction` del Task 9 — misurato **portante** — ha ora la propria riga nel **catalogo §7.4.1 blocco C**, la **regola B** della coppia `Untrusted`/`Instruction` accanto alla regola A (`Q9 · I6 · V20`), con richiamo datato. ⛔ **Non era una rifinitura, ed è il punto da ricordare:** la regola A è **cieca** proprio a quella via — con `impl From<Untrusted> for Instruction` presente il caso resta `ok` invece di dare il `mismatch` che il gotcha #42 prevede, perché lì lo scarto è fra **riferimenti** — quindi senza la riga B la porta resta verde **col confine già caduto**. Il blocco C passa da diciassette a **diciotto** righe, e §7.4.7, §8.3 (`Q9` e `Q15`) e [`porta-di-qualita.md`](../porta-di-qualita.md) sono riallineate nello stesso passaggio.
6. ~~**Il piano del Traguardo 3**~~ — ✅ **scritto il 2026-08-10**: [Traguardo 3](../superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md), **dodici compiti in due parti**. ⛔ **Sette decisioni prese dal piano, e la prima governa tutte le altre:** i **byte congelati sono l'ultimo compito**, non il primo. Il nome del traguardo li mette in vetrina e la tentazione è produrli subito, ma non si rigenerano mai — congelarli prima che un consumatore vero e **due** implementazioni abbiano esercitato il formato significherebbe congelare la forma sbagliata, che è il difetto del Task 11 del Traguardo 2 nella sua forma più cara. ⚠️ **Due voci aperte del codice trovano risposta**: una promozione **non è un passo proprio** (ADR-0007 fissa la granularità), e l'**etichetta di fiducia entra nel record** perché è l'unico campo che, se manca, porta informazione che **si perde per sempre**. ⛔ **E la porta guadagna `replay()`**, perché `read_back` chiede un passo **per nome** e dopo un crash il kernel non sa i nomi: non era una decisione presa, era una **lacuna** — ADR-0007 dice *«per ogni passo in dubbio»* senza dire come si scoprono.
7. ~~**Il codice del Traguardo 3**~~ — ✅ **eseguito il 2026-08-10**, subagent-driven, un compito per volta con revisione fra uno e l'altro. **Dodici compiti su dodici**, `GATE GREEN` a tutti. ⚠️ **Ricontati una QUARTA volta il 2026-08-10, chiudendo il traguardo:** questa riga diceva *«undici»*, e la cifra è invecchiata **quattro volte su quattro** — è la riga più stantia del file per costruzione, e il rimedio scritto sotto (*«si riconta prima di appendere il capoverso»*) non ha retto nemmeno una volta. ⚠️ **Ricontati una terza volta il 2026-08-10, col Task 10**, che è la terza volta di seguito che questa cifra viene appesa senza essere toccata — gotcha **#31** sul numeratore, e a questo punto è la riga più stantia del file per costruzione: si riconta **prima** di appendere il capoverso, non dopo. ⚠️ **Ricontati una seconda volta il 2026-08-10, col Task 9:** questa riga diceva *«sette compiti»* mentre il capoverso del **Task 8** le era già stato appeso sotto senza toccare la cifra — la **stessa** forma del difetto che la nota qui accanto descrive, ripetuta dallo stesso file due paragrafi dopo averla dichiarata. Una cifra dentro una frase che resta vera è il gotcha **#31**. ⚠️ **Ricontati il 2026-08-10:** questa riga diceva *«tre compiti»* e vi era stato **appeso** il capoverso sui Task 4 e 5 senza toccare la cifra — la §6 dello stesso file era già stata portata a cinque, quindi il documento si contraddiceva **dentro sé stesso**, che è la forma peggiore. I primi tre: il **record durevole** — enum di versione `Record::V1`, quattro campi con indice esplicito, `Debug` scritto a mano che non stampa il payload — la **riga di catalogo dell'etichetta di fiducia** (`Q9 · I6 · V20 · §4.9`) col caso `record_without_trust_label.rs`, e il **doppio in memoria del giornale** con la variante `JournalError::OutOfOrder`, che tiene V6 sulla **porta** invece che sulla diligenza del chiamante. ⛔ **Un'errata in testa al piano, sette voci dal solo Task 1**, e i costi misurati stanno in [`riferimenti.md`](../riferimenti.md): la collisione `record`/`boundary` che ha riscritto **due oracoli pre-esistenti** è permanente. ⛔ **E il Task 3 ha trovato la lacuna di specie 2 più netta del progetto**: il criterio di chiusura che il piano dettava — `4 passed` — è soddisfatto da un giornale che **non registra nessun esito**.
   ✅ **Task 4 e 5 eseguiti il 2026-08-10, e come un compito solo**, perché separati non funzionano: il Task 4 scrive una suite che chiama `replay()`, che la porta guadagna solo al Task 5, e il suo commit lascerebbe la **porta rossa** contro il vincolo globale 8; per giunta il `git add` del Task 5 non nominava il file del Task 4, che così non sarebbe **mai** entrato nel repository. Ne sono usciti **sette bugiardi e non tre**, uno per promessa — sei eseguendo, il settimo dalla decisione presa in revisione: la suite muore alla **prima** promessa violata, quindi con tre bugiardi **due promesse non si vedevano mai fallire** — gotcha **#14**. ⛔ **E due promesse dettate erano vacue contro il proprio bugiardo, misurate e non rilette:** quella sull'ordine di `replay` confrontava le **sole identità** dei passi e la sequenza `1, 2, 1` **è un palindromo**, così `ShuffledJournal` passava la suite intera; e la promessa 1 rileggeva con un `expect` che ingoiava proprio la via **A6**, l'unica cosa per cui la suite esiste. ⚠️ **Due promesse in più che nessun compito chiedeva:** dopo intento **ed** esito, `read_back` deve rendere ancora l'**intento** — senza, al Task 8 una tabella `redb` chiavata sul passo risponde l'esito e **nulla diventa rosso** — e, **decisa dal coordinatore in revisione**, un **secondo `intent`** sullo stesso passo è **rifiutato**: la promessa 2 costringe già `redb` a chiavare più fine del passo, ma è un accordo **per accidente del disegno della chiave** e non per contratto. ✅ **Task 6 eseguito il 2026-08-10**, la **riconciliazione** — e in due commit, perché un passo preliminare toglie a `Record::encode` un `Result` che non poteva essere `Err` (**E22**, decisione del coordinatore). ⛔ **Tre difetti del compito dettato, trovati col pre-controllo e tutti misurati prima di decidere:** la §1.0 violata una **terza** volta nei commenti dettati (**E23**) · lo stesso passo che compariva **due volte** nell'insieme, con **due** produttori e non uno (**E24**) · e le **due verità indipendenti** su «intento o esito», che è la domanda riportata al proprietario (**E25**). ⚠️ **Quattro sonde che nessun compito chiedeva:** il giornale **vuoto**, l'**ordine** — che l'unica sonda dettata teneva per accidente, seconda forma del palindromo di E12 — e le due dell'insieme. ✅ **Task 7 eseguito il 2026-08-10** — `Untrusted::promote` è il **primo codice del kernel che scrive un record vero**, e il compito dettato aveva **due conflitti di formato**, misurati prima di decidere. ⛔ **Primo: la promozione non è un secondo intento, è una TERZA COSA.** Il `promote` dettato scriveva con `intent()` sul passo del chiamante, che da **E19** lo rifiuta — e il test dettato non lo vedeva perché poggiava su `RecordingJournal`, che è **riga per riga il bugiardo J7** della conformità. ⛔ **E anche a guardia rilassata il disegno resta rotto, per una ragione indipendente che nessuno aveva visto:** un secondo record `Intent` sullo stesso passo fa **sostituire** alla riconciliazione la risoluzione del chiamante — misurato, `Idempotent` torna `SuspendAndAsk`, cioè la promozione **declassa in silenzio un passo che non le appartiene**; e la sonda dettata confrontava le **sole identità**, quindi era cieca proprio a quello (terza occorrenza del palindromo di E12). **Decisione del coordinatore, confermata dal proprietario:** la porta guadagna **`note()`** e il record **`RecordKind::Note`**, insieme — una nota deve pur portare un `kind`, e i due esistenti sono i due difetti; la riconciliazione le dà un **arm vuoto**. Scartate e misurate: rilassare la guardia (non ripara nulla), scrivere con `outcome()` (`steps_in_doubt` risponde `[]`, un dubbio vero sparisce in silenzio) e la variante `Note` trasportata da `outcome()` — la più economica, scartata perché istanzia di proposito la divergenza di **E25**. ⛔ **Secondo: l'etichetta di fiducia era attaccata alla stringa sbagliata.** Il record dettato metteva nel `payload` la **ragione scritta dal chiamante** e la marcava `Trust::Untrusted`: nessun byte esterno entrava, e l'etichetta non era decorativa ma **falsa**, perché il doc di `Trust` dice che riguarda **il payload**. Il record guadagna quindi **`reason` all'indice 4**, e l'indice 3 porta il **contenuto non fidato** — assegnazione **forzata**, perché il `Debug` scritto a mano nasconde **solo l'indice 3**. ✅ **Così la via A4 si chiude, ma a LIVELLO 2 e non "al formato":** la via come `boundary.rs` la scrive passa da **byte grezzi**, non da un `Record`, e nulla impone che ogni scrittura sia un record. Le vie chiuse sono **tre** — A3, A4, A6 — e le **quattro** che restano sono tutte dichiarate **non chiudibili**: ciò che resta non è un arretrato, è il **pavimento**. ⛔ **`RecordingJournal` è stato tolto e non riparato**, e l'audit delle **ventuno** implementazioni di porta fuori da `src/` ne ha trovata una seconda che rompe un contratto — `RefusingReactor` — **che resta**, perché la regola giusta è *una finta può rompere un contratto quando il test parla della rottura*. ⚠️ **Costo contato:** dieci `E0046` (undici dopo il Task 8), una promessa nuova in conformità col proprio bugiardo («valida e poi butta»), e **due nomi di test corretti** invece che lasciati stantii. `cargo test --workspace`: **26 target, 127 test**.
   ✅ **Task 8 eseguito il 2026-08-10** — `redb` e il **backend scritto da noi**, in `platform`. ⛔ **Il piano era stantio sul contratto e lo dichiarava: non detta il codice di `redb`**, perché quando fu scritto l'API 4.1.0 non era verificabile e dettarla a memoria avrebbe prodotto codice *plausibile e falso*. Letta nella cache del registro **prima** di scrivere. ⛔ **La chiave è un PROGRESSIVO DELLA SCRITTURA e non il passo**, ed è tutto il disegno: `redb` è un B-tree **ordinato per chiave**, quindi chiavare sul passo darebbe a `replay` l'ordine **delle identità** invece di quello di scrittura, e un passo terrebbe **un solo record** — l'esito sovrascriverebbe l'intento e cadrebbe anche la promessa 2. ⚠️ **Il prezzo della chiave progressiva, che il piano non nomina: le guardie diventano SCANSIONI** — misurato **~56 ns per record** in release, lineare, con il pavimento della scrittura all'`fsync` (**~1,45 ms**): la scansione lo supera solo **oltre ~26 000 record**, quindi **non è ottimizzata** e il rimedio del giorno in cui morderà è lo stesso **checkpoint** che `replay` dichiara già di volere. ⛔ **Due decisioni che cambiano firme pubbliche:** `StepId::get()` **torna** — senza, la porta **non è implementabile fuori da `kernel`**, e il doc di `CheckpointId` aveva già fissato quel giorno *«con quel chiamante»* — e **`open` NON restituisce `JournalError`** ma un errore proprio, perché nessuna delle tre varianti significa *«non ho potuto aprire il file»* e `open` non è un'operazione della porta. ⛔ **La prova che il confine è reale, che il piano non chiedeva:** un confine dichiarato in anticipo non ha chiamanti per costruzione (gotcha **#46**), quindi una **seconda implementazione di `StorageBackend` scritta da fuori** gira nel banco — ed è servito aggiungere `FileJournal::with_backend`, che il piano non nomina: con la sola `open(path)` il confine sarebbe stato **inesistente in pratica**, il difetto del Task 11 del Traguardo 2. ⛔ **Due gotcha nuovi, entrambi dalle mutazioni o dalla misura:** **#51** — una garanzia sulla **morte del processo** non è osservabile da dentro il processo: `Durability::None` lascia **sei test su sei verdi**, ed è dichiarato accanto al codice invece che scoperto al Traguardo 4 — e **#52** — un difetto di parallelismo **mascherato dal sistema operativo**: la cancellazione della cartella condivisa avviene davvero (**tre volte su sei**) ma Windows rifiuta di cancellare un file aperto, quindi il rosso esce **su Linux**. ⚠️ **E una mutazione ha trovato un difetto vero:** il rimedio alla vacuità era esso stesso vacuo — terza occorrenza del **#45**. `cargo test --workspace --no-fail-fast`: **27 target, 133 test**.
   ✅ **Task 9 eseguito il 2026-08-10** — la conformità gira contro **entrambe** le implementazioni **a ogni commit**, `crates/platform/tests/journal_contract_real.rs`, che raggiunge le asserzioni per `include!` come `reactor_contract_real.rs` fa per `reactor`. ✅ **Ricontate leggendo il sorgente e non fidandosi della misura anticipata del Task 8: le promesse sono OTTO e passano tutte e otto**; il binario di `platform` porta **undici** test, i dieci inclusi più quello vero, e il costo — i bugiardi che girano una seconda volta — è **scritto**, non nascosto. ⚠️ **Sono cifre del Task 9, superate dal Task 11 e ricontate eseguendo il 2026-08-10:** con la promessa **7b** le promesse sono **nove** e i bugiardi **nove**, `kernel --test journal_contract` porta **undici** test e il binario di `platform` **dodici**. ⛔ **Tre difetti nel codice dettato, tutti nella fabbrica, e il compito valeva quelli:** il piano cancellava un **percorso fisso in una cartella condivisa** — su Windows la cancellazione **fallisce in silenzio** a file aperto, quindi la fabbrica riaprirebbe **i dati vecchi** (gotcha **#52**, nato il giorno prima); la fabbrica è chiamata **nove volte** e `FileJournal` tiene un **lucchetto esclusivo**; e `assert_journal_contract` prende **`Fn`** e non `FnMut`, quindi numerare le chiamate richiede un `AtomicU64`. Rimedio: **un file nuovo a ogni chiamata** — un nome mai esistito non può essere sporco — in una cartella **per call site** dal `line!()`, con **prefisso diverso** da quello di `file_journal.rs`, perché un numero di riga è unico dentro **un** file solo e i due binari girano insieme. Provato **otto** volte di seguito sull'intero workspace: **28 target, 144 test**, otto su otto verdi. ⛔ **E lo Step 3 è stato fatto in TRE direzioni invece di una**, perché una sola proverebbe una promessa su otto: `read_back` rotta muore sulla **promessa 1**, la guardia del secondo intento tolta sulla **promessa 6** — dopo aver superato cinque promesse sui propri meriti — `replay` rovesciato sulla **promessa 4**, ciascuna **col proprio messaggio**; e la **mutazione di controllo** non muove nulla. ✅ **I due lati sono separati, misurato:** con `FileJournal` rotta, `kernel --test journal_contract` resta **verde 10 su 10** e dentro `platform` restano verdi i dieci test inclusi. ✅ **Con esso la via A6 di `boundary.rs` passa da «chiusa la strada» a «chiuso l'accordo»**: era dichiarata chiusa *«fino a che la seconda implementazione non c'è»*, e ora c'è ed è tenuta.
   ✅ **Task 10 eseguito il 2026-08-10 — i BYTE CONGELATI, l'unico artefatto del progetto che non si corregge.** ⛔ **Il compito dettato era stantio sul record che doveva congelare, e i difetti erano sei più uno.** Il costruttore dettato ha **quattro** campi dove `RecordV1` ne ha cinque; `.encode().expect("encode")` non compila dal Task 6; ⛔ **entrambe le mutazioni dettate sono INDICI DUPLICATI e non compilano** — `error: duplicate index numbers` — quindi quella che «deve scattare» non si sarebbe **mai vista scattare** sull'oracolo che non si rigenera, e quella che «deve restare verde» sarebbe stata un errore di compilazione che il piano dichiara equivalente ad **ADR-0036 smentito**. ⛔ **E il settimo, che nessuno aveva visto: la mappa dettata sbagliava l'OFFSET, non solo l'arità** — il byte 2 è `81`, l'array a **un elemento** del corpo della variante, e l'array dei campi sta al byte **3**; l'inquadratura è `82 00 81 85` e un record misura **21** byte. Misurato sull'uscita vera invece che dedotto. ⛔ **Il difetto grosso era la COPERTURA:** un record solo fissa **tre** indici di variante su **otto** — `RecordKind` 3, `EffectClass` 3, `Trust` 2, ricontate sul sorgente — e il Task 1 aveva già misurato che ogni altra sonda sopravvive a una rinumerazione simmetrica, quindi cinque indici su otto sarebbero rimasti tenuti da **nulla**. ✅ **Congelati TRE record**, il minimo che li copra tutti, con `kind` ed `effect` a **quadrato latino** perché nessuna coppia di campi si possa scambiare senza muovere almeno un file; **le otto varianti rinumerate una per una, otto rossi su otto**. ✅ **L'additività misurata in DUE direzioni, e la seconda è il gotcha nuovo #54:** un campo facoltativo all'indice libero 5 lascia i byte **identici** con `None` — `minicbor` **tronca** un `None` in coda invece di scrivere `null` — e li porta a 22 con `Some(9)`; senza la seconda misura, «i byte non si sono mossi» sarebbe stato compatibile con un campo che sul filo non arriva mai, e ADR-0036 sarebbe stato «confermato» da una misura vuota. ✅ **La mappa è RILETTA dal banco** — offset e byte devono ricostruire il `.cbor` — così un `<fill in>` non può sopravvivere al commit (gotcha #43), e la colonna di prosa è dichiarata **non verificata** dentro la mappa stessa. ⛔ **Nessun percorso di rigenerazione:** i byte sono stati **scritti a mano** dall'uscita di una sonda usa-e-getta, cancellata nello stesso commit. ⚠️ **E un `.gitattributes` di UNA riga** — mai un `* text=auto`, che normalizzerebbe sorgenti che nessuno ha toccato — verificato con `git check-attr` e col blob dell'indice invece che dato per scontato. `cargo test --workspace --no-fail-fast`: **29 target, 150 test**.
   ✅ **Task 11 eseguito il 2026-08-10** — `prune` rifiuta un passo **in dubbio** e accetta uno riconciliato, su **entrambe** le implementazioni; chiude la voce aperta di **E11**, che aspettava `prune` da sei compiti. ⛔ **Lo Step 1 attendeva un ROSSO e la partenza era VERDE**: `prune` rispondeva `Missing` a tutto, quindi `is_err()` passava e la promessa 7 era soddisfatta **per caso** — a renderla non-vacua è la sola contro-sonda nuova, la **7b**, col bugiardo `AlwaysInDoubtJournal` che rifiuta tutto **con la parola giusta**. ⛔ **Il `prune` dettato viola una regola non negoziabile di ADR-0018 e la misura lo conferma su entrambe:** un payload potato e uno mai registrato sono indistinguibili in **tre** modi; la via che sembrava non costare l'impronta funziona ma fa rispondere `SuspendAndAsk` alla riconciliazione su ogni passo potato, quindi non è economica — dichiarata in due **voci aperte** invece che chiusa a metà. ⛔ **`JournalError` guadagna `StepInDoubt`, quarta variante su un tipo dichiarato «deliberatamente povero»**, e non allarga `OutOfOrder` perché quello è definito da **V6** mentre questa è ADR-0018 — invarianti diversi — e perché il chiamante deve distinguere un difetto da una spazzata ordinaria. ⛔ **`FileJournal` NON POTEVA rispondere alla domanda** e la sua tabella `redb` guadagna il **byte dell'operazione**: contare i record è sbagliato perché una **nota** non è un esito, e decodificare i byte è vietato da ADR-0036. ⛔ **Quattro mutazioni su quindici sopravvissute al primo giro, e tre erano difetti veri:** `prune` che risponde `Ok` e **non pota niente**, `prune` che pota **l'intero giornale**, e una **nota archiviata come esito** che rendeva potabile un passo in dubbio. Chiuse tutte e tre **senza decidere nulla sulla ritenzione**. `cargo test --workspace --no-fail-fast` → **29 target, 152 test**.
   ✅ **Task 12 eseguito il 2026-08-10 — ed era un AUDIT, non una scrittura: il gotcha #49 per la seconda volta, previsto dal compito stesso.** ⛔ **Le quattro righe che lo Step 2 dettava di spostare fra le coperte erano già spostate tutte e quattro**, dai compiti che le avevano prodotte; a mancare non era l'aggiunta ma il **riconteggio**, come al Traguardo 2. ⛔ **E la classe che non si vede leggendo ne ha data una vera:** partendo dall'elenco dei bugiardi invece che dalla colonna, **`J13` non era mai entrato** nella colonna «deve scattare» della riga di catalogo dei test di contratto — esisteva nella tabella delle sonde dal Task 11 e da lì risultava inesistente. ⛔ **Cinque conteggi di test erano stantii nel registro** — `boundary_promotion.rs` otto contro **quindici**, `record_shape.rs` dieci contro **dodici**, `reconciliation.rs` nove contro **undici**, `journal_contract_real.rs` undici contro **dodici**, i casi `compile_fail` quattordici contro **diciassette** — e la campagna della conformità si intestava *«otto promesse, otto bugiardi»* dove sono **nove e nove**: la cifra della passata **non è stata alzata per simmetria**, perché sarebbe stata un'ipotesi, e la **7b** è dichiarata misurata altrove, da `M14b`. ⛔ **E la Definizione di «fatto» era stantia in tre condizioni su dodici** — la 4 dice *«tre bugiardi»*, la 6 *«A4 chiusa al formato»* che **E31** aveva già respinto, la 9 detta due direzioni che **E51** ha misurato **incompilabili** — corrette **nell'errata** e non nel testo, che è il registro di ciò che fu deciso. ✅ **Il confronto con ADR-0036 sulla dimensione del record è stato misurato al posto giusto:** i totali non sono confrontabili, perché la forma del record che l'ADR prezzava **non è scritta da nessuna parte**; ciò che è confrontabile è **quanto costa la busta di versione**, e **converge esattamente** — `+3` byte, `82 00 81`, sia su un record pieno (18 → **21**) sia su uno vuoto (6 → **9**), dove l'ADR misurò `27 → 30`. ⚠️ **Diverge la percentuale, e si registra invece di arrotondarla:** `+11 %` là, **`+17 %`** qui, perché la base è più corta. `bash scripts/gate.sh` → `GATE GREEN`; `cargo test --workspace --no-fail-fast` → **29 target, 152 test**.
   ✅ **Audit di chiusura della sessione, il 2026-08-10 — dopo il Task 12 e distinto da esso.** Il
   Task 12 aveva chiuso il **traguardo**; questa passata ha riletto **tutti e nove** i documenti di
   stato contro il repository, partendo dai numeri invece che dalle frasi. ⛔ **La classe di difetto
   della sessione — la stessa cifra in più posti con valori diversi — ne ha dati altri sei**, e il
   più caro attraversava **tre** file: l'errata del Traguardo 3 diceva *«settanta voci in otto
   passate»* in `HANDOFF.md` (due volte) e in [`roadmap.md`](../roadmap.md), col valore giusto —
   **settantasette in nove** — scritto nella §12 di **questo** file. Gli altri: le **promesse** della
   conformità, otto in due punti di questa §6 e nove in un terzo · i **test** dei due binari del
   giornale, undici e dieci · le **funzionalità** di [`tracciabilita.md`](../tracciabilita.md),
   centosettanta in §8 contro **centosettantuno** ricontate · il **peso del messaggio** di
   [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), *«~4 KB»* contro **6,2** misurati · e i **pilastri** di
   [`README.md`](../README.md), che ne elencava cinque dicendo quattro. ⛔ **E tre affermazioni di
   stato erano false in `HANDOFF.md`:** il punto di ripresa nominava il solo **Traguardo 1**, il
   giornale write-ahead *«sale col Traguardo 3»* al futuro, e la mappa dei piani dava il piano del
   Traguardo 3 come **da eseguire**. ⚠️ **Due divergenze sono state REGISTRATE e non appianate,
   perché stanno nel sorgente e questa era una passata documentale:** l'intestazione di
   `crates/platform/tests/journal_contract_real.rs` dice *«le DIECI prove della suite — la finta, il
   vincolo sulle sottostringhe e gli OTTO bugiardi»* dove sono **undici e nove**, e il file gemello
   `crates/kernel/tests/journal_contract.rs` porta le cifre **giuste** dieci righe più su; e le *«72
   target verdi»* attribuite alla chiusura del Traguardo 2 — scritte identiche qui e in `HANDOFF.md`
   — non riconciliano con nessun'altra misura del progetto (**25 target** a quella data, **29**
   oggi), e non sono state riscritte perché rifarle richiederebbe uno stato che non esiste più.
   ⚠️ **Questa riga diceva *«il prossimo è l'ESECUZIONE DELL'AUDIT … e non il Traguardo 5»***,
   ed è corretta il 2026-08-18: l'audit è **chiuso**, e il puntatore vivo sta in cima alla §6. ✅ Il
   brainstorming del Traguardo 4, il disegno, il piano **e la sua esecuzione** si sono chiusi
   tutti il 2026-08-11, e lo stesso giorno il repository ha ricevuto il suo **primo audit
   completo**.
   ⚠️ Questa riga ha detto *«il brainstorming»*, poi *«il piano»*, poi *«l'esecuzione»*, poi il
   Task 2, il 3, il 4 e il 5 **nello stesso giorno**: è la riga che invecchia più in fretta del
   file, e si riscrive **quando il passo si chiude**. ⚠️ **E diceva «una delle TRE in cui il
   prossimo passo vive dentro questa sola sezione»: sono QUATTRO**, ricontate col `grep` invece
   che citate — righe 618, qui, 794 e la riga del Traguardo 4 nella tabella dei sei. È il gotcha
   **#31** nella forma che la **ventesima misura** aveva descritto per i pesi: *le case si contano
   una volta sola, quando si scrive il rimedio*, e chi riconta non si fidi del numero scritto nel
   verbale precedente.
8. ~~**Il brainstorming del Traguardo 4**~~ — ✅ **chiuso il 2026-08-11**, e il disegno è scritto:
   [Traguardo 4 — il disegno](../superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md).
   ⛔ **Ha trovato una collocazione non eseguibile in un ADR `Accepted`**, e non riaprendo una
   decisione ma misurandola: il backend cadente di [ADR-0032](../adr/0032-motore-di-persistenza.md)
   *«vive in `simulator`»*, e `simulator` è `no_std`, si costruisce per `x86_64-unknown-none` e ha
   un grafo spedito a lista chiusa la cui unica cura per un intruso è **togliere la dipendenza**.
   `redb` non ha `no_std`. **Rimando datato**, non `Superseded by`: è sbagliata una **cella**, non
   la decisione — ed è la **seconda** volta per quell'ADR. ⚠️ **La stessa cella viveva anche nella
   §5 di questo file**, ed è stata corretta nello stesso passaggio: la classe di difetto della
   sessione precedente, colta stavolta **prima** di committare.
   ⚠️ **E una lettura affrettata è stata registrata invece che taciuta:** `StorageBackend` ha
   **sei** metodi, non cinque — `close` ha un'implementazione predefinita — e l'ADR li dichiarava
   giusti. L'errore andava **a sfavore** dell'oracolo, non a favore: il conteggio dei punti scattati
   è la non-vacuità della campagna, quindi un metodo in meno sarebbe stato un oracolo più debole
   senza che nulla lo dicesse.
9. ~~**Il piano del Traguardo 4**~~ — ✅ **scritto il 2026-08-11**:
   [Traguardo 4](../superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md), **dieci
   compiti in tre parti**. ⛔ **Sette decisioni prese dal piano**, e la seconda è quella che governa
   il resto: la caduta **non si riprende** — un giornale che rifiuta una volta e poi riparte modella
   un disco cattivo, non un crash, e la permanenza è ciò che fa fermare **tutte** le attività
   interlacciate e non solo quella che ha toccato il confine. ⚠️ **E il punto di caduta si estrae da
   un generatore diverso da quello dell'interlacciamento**, con seme derivato: due `SeededRng`
   costruiti dallo stesso numero danno la **stessa** sequenza, quindi la campagna esplorerebbe una
   **diagonale** dello spazio invece dello spazio.
10. ~~**L'esecuzione del Traguardo 4**~~ — ✅ **ESEGUITO il 2026-08-11**, subagent-driven, un
    compito per volta con revisione fra uno e l'altro: **dieci compiti su dieci**, `GATE GREEN` a
    ciascuno; `cargo test --workspace` → **32 target, 171 test** di cui
    **due** ignorati, le due campagne profonde. ⛔ **E il Task 7 ha stabilito cosa compra una
    campagna PROFONDA, misurandolo: non lo spazzamento.** Allargare l'intervallo oltre la
    saturazione dà corse **indistinguibili da corse senza iniezione** — a ottocento punti ne
    scattano sempre trentacinque — mentre **approfondire lo scenario compra stati nuovi uno per
    record**, e la metrica che lo dimostra sono i **pioli**: le lunghezze di prefisso distinte che
    tornano sono `record + 1` a ogni profondità misurata. ⚠️ **E la saturazione non è lineare:** il
    terzo record costa dieci operazioni dove il primo ne costa sei, e il `Drop` ne costa quattordici
    a due record e dodici altrove — dedurla avrebbe sbagliato. ✅ **E il gotcha #51 è chiuso nella
    metà chiudibile**, col perimetro di ciò che **non** compra scritto per esteso — perché
    *«è chiuso»* nella forma nuda mentirebbe. ⛔ **Ma la lezione del Task 6 è un'altra, ed è la
    terza volta che questo traguardo la impara: un ciclo che verifica la COERENZA e un oracolo che
    verifica che il GUASTO SIA SCATTATO possono essere entrambi verdi mentre la proprietà per cui
    esistono è sparita.** Misurato: togliendo la durabilità, il ciclo di coerenza è **interamente
    verde**; a renderlo un secondo testimone è un'asserzione sui **gradini** — che esistano punti in
    cui torna *qualcosa ma non tutto* — perché senza durabilità la scala collassa a **zero-o-tutto**.
    📌 Serve un controllo che guardi **la forma di ciò che sopravvive**, non solo che qualcosa sia
    sopravvissuto. ⛔ **E il Task 5 ha trovato, prima
    che il Task 6 fosse scritto, che l'oracolo destinato a chiudere il gotcha #51 NON FUNZIONA.**
    Il piano voleva contare le chiamate a `sync_data`: ma **sei sync su sette nascono prima che
    esista un record** — `create_with_backend` nudo ne fa sei — quindi il conteggio è dominato
    dall'apertura ed è **cieco** proprio alla perdita per cui esisterebbe. Misurato: con la
    durabilità tolta, la sonda in quella forma resta **verde**. 📌 **La forma generale: un
    contatore che parte da un valore che il soggetto sotto esame non ha prodotto non è un oracolo
    su quel soggetto** — la cura è il **delta**, non il totale. ⛔ **E lo scenario di livello 2
    SATURA a cinquantotto operazioni**, quindi la campagna profonda che il Task 7 detta
    costerebbe **venti volte tanto esplorando zero stati in più**. ⛔ **E il Task 4 ha stabilito come si sceglie il numero
    di semi, che è la decisione che il vincolo 7 della §11 chiedeva: NON massimizzando.** *«Il più
    grande multiplo di cento sotto il tetto»* insegue una cifra che **satura** — i ventiquattro
    punti di caduta sono coperti già a duecento semi e l'insieme in dubbio massimo vale **tre**,
    che è il suo tetto **strutturale**. Il criterio vero è la **chiusura dello spazio degli
    esiti**: gli insiemi in dubbio distinti sono **centonove**, l'ultimo compare al seme **1038**,
    e ventimila semi in più non ne producono altri. Scelti **duemila**, che costano l'**11 %** del
    tetto. ⛔ **E la guardia su quel criterio è stata adottata solo dopo aver misurato che non
    scattasse dove non deve:** sei costanti di mescolamento diverse danno **centonove tutte e
    sei**, quindi il conteggio è proprietà dello **scenario** e non dei semi che lo campionano.
    ⚠️ **Due misure del coordinatore sono state smentite da chi eseguiva:** il modello di costo
    `2 × semi × costo` sbagliava del **37 %** — una corsa che cade si ferma al proprio punto,
    `C7a` arriva in fondo — e il tetto vero cade a ~19 000 semi. ⛔ **E il Task 3 — `C7b` — ha
    trovato il difetto più istruttivo del traguardo: la non-vacuità che una campagna dichiara non
    è quella che le serve.** *«L'iniezione è avvenuta»* e *«c'era qualcosa da verificare»* sono
    **due** affermazioni, e il piano ne teneva una sola: con un giornale che cade al primo byte,
    duecento semi su duecento cadono davvero **e duecento confronti su duecento sono
    `[] == []`** — verde, e nulla verificato. ⚠️ **Ed era il difetto che il Task 2 aveva chiuso per `C7a` un
    compito prima, reimportato**: chiuderlo in un posto non lo chiude nell'altro. Ora `C7b` ha
    **due** oracoli, e il discrimine fra loro è **provato** — le mutazioni che svuotano la
    campagna sparano sul secondo, quelle che spengono il guasto sul primo. ⛔ **Il Task 2 — lo scenario giornalato e `C7a` — ha trovato che `C7a` era verde
    su un archivio VUOTO:** *«nessun passo è in dubbio»* e *«lo scenario non ha scritto niente»*
    erano lo stesso verde, misurato con un giornale che cade alla scrittura zero, e il piano vi
    rispondeva con una mutazione **una tantum** mentre allo stesso buco su `C7b` dà un oracolo
    **permanente**. ⛔ **E una regola nuova, uscita da una mutazione che nessuno aveva chiesto:**
    quando due mutazioni uccidono la **stessa** asserzione, prima di concludere che la sonda non
    distingue i due difetti (gotcha **#55**) si cerca **una terza mutazione che lasci passare la
    prima asserzione** — se esiste, le due non erano in competizione ma su assi diversi.
    ✅ **E la decisione di NON fissare qui l'interlacciamento è stata verificata in negativo**,
    costruendo il controfattuale sequenziale: il massimo insieme in dubbio scende da **tre** a
    **uno**, quindi la sonda del Task 3 va davvero rossa e una in più qui sarebbe stata
    duplicazione (gotcha **#49**). Il Task 1:
    `CrashingJournal` in `crates/simulator/src/journal.rs`, **dieci** sonde in
    `crates/simulator/tests/crashing_journal.rs`, **cinque mutazioni uccise su cinque**.
    ⛔ **Il pre-controllo ha trovato un difetto vero, ed è la specie che non si vede leggendo:**
    il doc di `may_write` dichiara che il contatore si muove **solo su un `Ok`**, e nessuna delle
    otto sonde dettate faceva mai fallire una scrittura interna — la mutazione corrispondente
    **sopravviveva a tutte e otto**. ⛔ **E la revisione ne ha trovato un secondo, peggiore:
    `prune` mutava l'archivio DOPO la caduta**, unica operazione mutante fuori dalla guardia,
    mentre il doc prometteva *«ogni scrittura successiva è rifiutata»* e il limite dichiarato
    nominava le **sole letture**. 📌 La classe vale oltre il caso — è il gotcha **#29** spostato
    su un limite dichiarato: **una partizione scritta in un doc lascia scoperto il membro che non
    appartiene a nessuna delle due categorie**, e nulla lo segnala perché il doc *sembra*
    esaustivo. Le otto voci d'errata stanno in testa al piano; le misure in
    [`riferimenti.md`](../riferimenti.md), la campagna in [`porta-di-qualita.md`](../porta-di-qualita.md).
    ⚠️ **Questa riga diceva *«il prossimo è il Task 2»* a traguardo CHIUSO da dieci compiti**, ed è
    corretta il 2026-08-17 invece che cancellata: era il puntatore di avanzamento **dentro** il
    racconto del Traguardo 4, e nessuno l'ha spento quando il racconto ha smesso di essere in
    corso. Il puntatore vivo sta **in cima alla §6**, in un posto solo — gotcha **#31** nella forma
    che la §6 stessa dichiara di temere: *la riga che invecchia per costruzione*.
11. ~~**Il brainstorming del Traguardo 5**~~ — ✅ **chiuso il 2026-08-18**, e il disegno è scritto:
    [Traguardo 5 — il disegno](../superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md),
    **dodici sezioni**. Il racconto per esteso sta nel riquadro in cima alla §6; qui basta ciò che
    governa il piano: il perimetro è **l'arbitro intero**, le due proprietà «impossibili» sono
    **due metà** e la loro metà d'arbitro è **una sola**, `Grant` **si sposta** nel modulo
    dell'arbitro perché un fratello non può costruirlo (`E0423`, misurato), e le righe di catalogo
    che il traguardo tocca sono **dodici già scritte** e nessuna nuova.
12. ~~**Il piano del Traguardo 5**~~ — ✅ **scritto il 2026-08-18**:
    [Traguardo 5](../superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md),
    **tredici compiti in cinque parti**. ⛔ **Otto decisioni prese dal piano, e la prima governa
    le altre:** `Arbiter::new` prende **`Parameters`**, non un `Mib` nudo. Le altre: `Grant`
    porta un identificatore privato e `release` lo **consuma** · `release` risponde
    `Result<Mib, ReleaseError>`, e l'`Err` **è raggiungibile** — una concessione rilasciata
    sull'arbitro sbagliato · `ComputeClass` implementa `Ord` **a mano**, da una chiave esplicita ·
    la riscossione delle scadute è **privata** e gira in testa a ogni operazione · la transizione
    di policy prende il giornale **per riferimento** · il modulo è una **cartella con tre file** ·
    `Grant` **non** è ri-esportato da `ports::process`. ⚠️ **Il racconto del pre-controllo — le
    sette voci — sta nel riquadro in cima alla §6.**
13. ~~**L'esecuzione del Traguardo 5**~~ — ✅ **ESEGUITA il 2026-08-25**, subagent-driven, un
    compito per volta con revisione fra uno e l'altro, `GATE GREEN` a ciascuno, dal 2026-08-19.
    ⚠️ **Il numeratore non è scritto qui — richiamo del 2026-08-25:** questa voce lo portava, ed
    era la **seconda** copia dentro la §6 mentre il riquadro in cima dichiara che vive *«QUI e in
    nessun altro punto di questa sezione»*. **Tolto, non riallineato.** Il tredicesimo era un
    **audit** e non una scrittura, e
    ha trovato che **gran parte era già eseguita**: i riconteggi non hanno prodotto uno scarto e
    il *«dodici»* delle righe di catalogo ha retto: a mancare erano le **voci aperte in una
    tabella sola**. Il racconto compito per compito, i numeri e ciò che resta aperto stanno nei
    riquadri in cima alla §6.
