# Chiusura del sotto-progetto 1: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario (scelta **4** del [disegno](../specs/2026-09-02-sottoprogetto-1-chiusura-design.md)).
> I passi usano le caselle (`- [ ]`) per il tracciamento. ⛔ **Il pre-controllo di ogni compito
> si fa nella sessione che scrive il piano; l'esecuzione in una sessione NUOVA.**

**Obiettivo.** Chiudere il sotto-progetto 1 contro la sua Definizione di «fatto» — la §0.7
della [spec](../specs/2026-08-06-sottoprogetto-1-kernel.md) — rileggendo le condizioni contro il
codice, rileggendo `tracciabilita.md` con la seconda domanda, archiviando il racconto del
Traguardo 6 dalla §6 del compendio, e aggiornando i documenti di stato in un passaggio solo.

**Forma.** La chiusura è un **audit** più le passate documentali che ne seguono, in
sequenza, con un commit che dice «chiuso» soltanto alla fine. Le decisioni sono
già prese dal [disegno](../specs/2026-09-02-sottoprogetto-1-chiusura-design.md); questo piano le
traduce in passi.

⚠️ **RICHIAMO DEL 2026-09-03, voce E12 — questa riga diceva *«Nessuna riga di prodotto: la chiusura è un audit più TRE passate documentali. QUATTRO compiti in sequenza»*, e il compito 3bis l'ha smentita in entrambe le metà:** scrive un caso `compile_fail` in `crates/`, e i compiti non sono più quattro. ⛔ **I numerali sono TOLTI e non riallineati** — precedenti **AUD-007** e **AUD-046**: quanti siano i compiti lo dice la **tabella della posizione**, in una casa sola.

**Strumenti.** `bash`, `awk`, `grep`, `sed -n` in lettura; **Python 3** con `newline=''`,
temporaneo e `os.replace` per ogni scrittura su un file CRLF; `git`. La porta di qualità è
`bash scripts/gate.sh`, e deve stampare `GATE GREEN` **prima di ogni commit**, anche di soli
documenti — è così che il Traguardo 6 ha chiuso il compito 10.

**Disegno:** [`2026-09-02-sottoprogetto-1-chiusura-design.md`](../specs/2026-09-02-sottoprogetto-1-chiusura-design.md)
— si legge **prima** dei compiti, per intero: sono sei sezioni e tre di coda.

## Vincoli globali

Valgono per ogni compito, senza che il compito li ripeta.

| # | Vincolo | Da |
|---|---|---|
| 1 | **la spec non si tocca**: `git diff --name-only <base>..HEAD -- docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` deve restare vuoto. ⛔ **RICHIAMO DEL 2026-09-03: SOSPESO dal proprietario per il solo compito 3bis e per le sole §8.3 e §8.4** — voce **E10** — quindi quel comando rende ora la spec. ⚠️ **Il vincolo resta INTERO per ogni altro compito, il 4 compreso**, e con esso il perimetro: fuori da quelle due tabelle la spec non si tocca, nemmeno per correggere una frase che quel commit ha reso falsa — voce **E15** | disegno §6.3 condizione 7, e §7.7 riga 18 |
| 2 | **nessuna riga ⚠️ o ⏳ della §8 passa a ✅**, e nessuna riga di catalogo nasce | disegno §1.2 |
| 3 | [`riferimenti.md`](../../riferimenti.md), [`porta-di-qualita.md`](../../porta-di-qualita.md) — salvo una riga falsa trovata dall'audit — [`CLAUDE.md`](../../../CLAUDE.md) e l'audit del 2026-08-27 **non si toccano** | disegno §5.3 |
| 4 | **nessuna cifra nuova in prosa**: date, comandi e rimandi. Un numero che sostiene una decisione porta accanto il comando e la data | disegno §5.3, `CLAUDE.md` |
| 5 | **codice in inglese, documenti in italiano**: i commenti di `scripts/check-docs.sh` sono **inglesi**, come tutti gli altri di quel file | §1.0 della spec |
| 6 | **i fine-riga si conservano per file** e si rimisurano dopo ogni scrittura con `tr -cd '\r' < <file> \| wc -c`: CRLF nell'albero per `COMPENDIO.md`, `HANDOFF.md`, `README.md`, `AVVIO-CHAT.md`, `semi-dst.md`, `scripts/check-docs.sh`; LF per `roadmap.md`, `tracciabilita.md`, `archivio/stato-storico.md` e i due file di `superpowers/` di questa chiusura. `git ls-files --eol` **prima e dopo**, invariato | disegno §4.7 e «Cosa ha misurato» riga 6 |
| 7 | **una mutazione si revoca da una copia byte-esatta presa prima**, e la revoca si prova con `cmp` — mai con `git diff` su un file che git non traccia | D7 ed E26 del piano del Traguardo 6 |
| 8 | **ogni conteggio si rifà col comando** prima di leggere che cosa un documento ne dice; le cifre di questo piano sono istantanee del 2026-09-02 | disegno §6.2 regola 1 |
| 9 | **si toglie invece di riscrivere meglio**: una riga falsa in un documento vivo si sottrae o riceve un richiamo datato di una riga; un verbale non si riscrive | disegno §6.2 regola 5 |
| 10 | **si committa e si pusha a ogni compito**, senza chiedere e **senza co-autore** | `CLAUDE.md` |
| 11 | ⛔ **nessun compito prima del 4 scrive «chiuso» del sotto-progetto**, in nessun file e in nessun messaggio di commit | disegno §5.4 |

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

✅ **IL PIANO È SCRITTO IL 2026-09-02 — quattro compiti, nell'ordine della §5.4 del disegno.**
Il pre-controllo delle quattro domande su ciascun compito sta nell'errata qui sotto, fatto nella
sessione che ha scritto il piano, come il disegno prescrive.

| # | Compito | Commit | Stato |
|---|---|---|---|
| **1** | l'audit — le condizioni 1–5 della §0.7 rilette contro il codice, il verbale in bozza nella **§7** del disegno | uno | ✅ 2026-09-03 |
| **2** | `tracciabilita.md` riletta con la seconda domanda | **due** (D3) | ✅ 2026-09-03 |
| **3** | il racconto del Traguardo 6 esce dalla §6, e il tetto scende | **due** (D3) | ✅ 2026-09-03 |
| **3bis** | la voce 9 di §7.8 — la §8 smette di promettere controlli che non esistono | **due** (D3) | ✅ 2026-09-03 |
| **4** | il passaggio unico sui documenti di stato, **l'unico che dice «chiuso»** | uno | ⬜ da eseguire — ⛔ **dopo il compito 3bis**, che è la decisione del proprietario sulla voce 9 di §7.8 (**E9**, **E10**) |

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella §6 del
[`COMPENDIO.md`](../../COMPENDIO.md), in un posto solo. Ciò che resta qui è la **posizione** del
piano — la tabella qui sopra, che chi esegue aggiorna nel commit del compito — e **come** si
esegue.

### ▶️ Come si esegue un compito di questo piano

1. Si legge l'**errata** qui sotto per intero, poi il compito — tutto e nient'altro.
2. Si **rimisura** ciò che il compito dà per misurato: ogni cifra è del 2026-09-02.
3. Se il compito dice il falso, **ci si ferma e si riporta**: non si aggira. Una divergenza è una
   voce d'errata prima di essere un rimedio.
4. Il cancello gira **prima** di ogni commit; il commit dice ciò che il compito ha fatto, con le
   parole che la tabella della §5.4 del disegno gli concede.
5. Il revisore **rilancia ogni comando** accanto a un'affermazione misurabile e li elenca; per il
   compito 2 **ri-deriva da solo** l'insieme delle righe cambiate prima di leggere il verbale.
6. Una seconda ondata di **sola prosa** la fa il coordinatore a mano, senza ri-revisione; se
   tocca un fatto o un comando, si ri-rivede (disegno §6.4).

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota, e non resterà vuota.** Il pre-controllo ha trovato un difetto reale in **tutti**
i compiti dispacciati finora, senza una sola eccezione: quando ne trovi uno, si scrive **qui**,
con il proprio numero, prima di eseguirlo. Un piano è un'ipotesi.

| # | Voce |
|---|---|
| **E1** | ⛔ **COMPITO 2, Passo 3 — la tabella degli esiti non ha una riga per il caso *«in Entra, ma il codice non lo dà»*, e il caso ESISTE.** La §0.4 riga §10 fa **entrare** il lato kernel di segreti, confinamento e checkpoint — *«dichiarare, richiedere, giornalare»* — ma in §8.3 `V34` è **⏳ rimandato** con innesco `B (3)` e `V35` è **⚠️ parziale** con innesco `D (5)`; e `grep -rniE 'confinement\|sandbox' crates/kernel/src --include=*.rs` rende **una riga di commento** e nessun tipo. Le righe di `tracciabilita.md` che poggiano su quei meccanismi — `Sandboxing ed esecuzione`, `Permessi e sandbox policy`, `Gestione segreti e credenziali`, `Storage e cifratura a riposo`, e le altre che nominano `§10` — non cadono in nessuna riga della tabella del Passo 3: non sono in «Si scaglia», e la clausola *«e un modulo lo porta»* della prima riga è falsa. 📌 **Rimedio, ed è una riga in più nella tabella, non una decisione nuova:** *«è in Entra ma nessun modulo di `crates/kernel/src/` lo porta, e la riga V o Q che lo governa è ⏳ o ⚠️ in §8 → 🔶 col sotto-progetto dell'innesco di quella riga (D8), e una riga in §7.7: la §0.4 lo fa entrare e il codice non lo dà»*. ⚠️ **La divergenza fra §0.4 e il codice si REGISTRA, non si appiana:** la §0.4 è spec (vincolo globale 1), e per `V34` la §8 porta già scritto perché — finding AUD-026. Trovato dal pre-controllo, domanda 1: la sonda dettata non copriva il meccanismo. ⚠️ **RICHIAMO DEL 2026-09-03, e questa voce NON si riscrive perché è un verbale del pre-controllo: `V35` non è più ⚠️ parziale.** Il compito 3bis l'ha **declassata a ⏳ rimandato**, innesco `D (5)` invariato (voce **E10**); `V34` resta ⏳ con innesco `B (3)`, come questa voce dice. Il **rimedio** di E1 non cambia: la riga in più nella tabella degli esiti vale per una riga *«⏳ o ⚠️ in §8»*, e `V35` vi ricade ancora. |
| **E2** | ⚠️ **COMPITO 4, Passo 7 — la frase di `semi-dst.md` che il compito cerca sta su DUE righe.** *«Da oggi le»* chiude la riga 71 e *«campagne sono **tre**, e la terza»* apre la 72: `grep -c 'Da oggi le campagne sono' docs/semi-dst.md` rende **zero**. Si cerca `campagne sono \*\*tre\*\*`, che è su una riga sola e unico nel file; il capoverso finisce con `barra torna verde.`, unico anch'esso. Il richiamo nuovo entra **dopo** quella riga, separato da una riga vuota, con l'`\r\n` del file. |
| **E3** | ⚠️ **COMPITO 4, Passo 3 — la frase della fase corrente di `README.md` sta su TRE righe, e la terza continua con un richiamo che deve RESTARE.** Misurato con `sed -n '7,9p' docs/README.md \| cat -A`: la riga 7 è `**Fase corrente: implementazione del kernel — i sei traguardi sono ✅ eseguiti, il 6 il`, la 8 è `2026-09-02; il sotto-progetto NON è chiuso, e la sua Definizione di «fatto» è la §0.7 della`, la 9 comincia con `spec.** ⚠️ **La coda di date che stava qui è TOLTA e non riallineata:**` e prosegue. ⛔ **La stringa da sostituire è quindi dalle prime due righe intere fino a `spec.**` compreso, con i due `\r\n` in mezzo**, e ciò che segue `spec.**` sulla riga 9 resta com'è. Con Python su CRLF e l'aiutante `mutate.py`, che rifiuta se la stringa non è unica. |
| **E4** | ✅ **COMPITO 1, Passo 4 — le quattro violazioni sono state ESEGUITE nel pre-controllo, il 2026-09-02, e i rossi sono MISURATI e non previsti** (gotcha **#57**). `gate-no-os.sh`: `error[E0463]: can't find crate for `std`` e `✗ kernel or simulator do NOT build for x86_64-unknown-none.`, exit 1; dopo il ripristino exit 0. `gate-deps.sh`: ⛔ **TRE righe `✗` e non «entrambi i messaggi»** — `I3 violated -- kernel ships 'unty'`, `I3 violated -- simulator ships 'unty'`, `phantom entry -- 'unty_renamed' is on SHIPPED and in NO graph`, chiusa da `3 violations`, exit 1; dopo il ripristino `OK -- the two graphs match the two lists, and both directions were checked.`, exit 0. La nota del compito su `simulator` è quindi **sostituita** da questa misura: le crate spedite sono due e il ciclo le riporta **entrambe**. `gate-attributes.sh`: le due righe `✗` scritte nel compito, alla lettera, exit 1; poi exit 0. `check-docs.sh`: `✗ broken link: ./docs/zz-sonda-condizione-2.md -> non-esiste.md` e `1 inconsistencies to fix.`, exit 1; dopo `rm` `OK — no inconsistencies.`, exit 0. Dopo tutte e quattro, `git status --porcelain` nominava **solo** i due file di questa sessione. 📌 Chi esegue **rilancia** comunque: la condizione 2 vuole che il controllo sia visto fallire **nel verbale**, e una misura del pre-controllo non è quel verbale. |
| **E5** | ⚠️ **COMPITO 4, Passo 6 — «la riga che comincia col marcatore» è un CAPOVERSO di più righe, e la sostituzione deve prenderlo intero.** Dal 2026-09-02 il capoverso del marcatore nella §6 va dalla riga che apre con `⏭️` fino alla riga **prima** di quella che apre con `✅ **IL BRAINSTORMING DELLA CHIUSURA`; le righe in mezzo sono la continuazione dello stesso capoverso. ⚠️ E il compito **3** archivia il capoverso che segue — quello del brainstorming, del disegno e del piano — perché è un verbale (P-5): al compito 4 quindi il marcatore è seguito dal puntatore dell'archivio, e la sostituzione si ferma alla riga vuota o alla riga che apre con `⛔ **IL RACCONTO DEL TRAGUARDO 6`. In entrambi i casi: dal `⏭️` fino alla fine del capoverso, letto intero prima di sostituirlo. |
| **E6** | ⚠️ **COMPITO 1 — l'esecuzione ha attraversato la MEZZANOTTE, e le misure del verbale portano il 2026-09-03.** Questo piano e il disegno datano ogni cifra al **2026-09-02**; il compito 1 è cominciato la sera del 2026-09-02 e i blocchi **A**, **F**, **G**, **H** sono stati rilanciati alle `2026-09-03T00:06:17`, misurato con `date +%FT%T`. ⛔ **La data non si allinea all'attesa: si registra** — la §7.1 del disegno porta il 2026-09-03 accanto al commit su cui misura, e la riga del compito 1 nella tabella della posizione porta la stessa data. Il commit di **merito** non cambia: il compito 1 non tocca `crates/` né `scripts/` in nessun commit, e `git rev-parse --short HEAD` rendeva `9214255` prima e dopo l'audit. 📌 **Vale per ogni compito che segue:** la data di un compito è quella che `date +%F` rende **quando lo si esegue**, non quella scritta in testa a questo piano. |
| **E7** | ⚠️ **COMPITO 2 — la lettera di D8 contraddice il Passo 4 dello stesso piano, sulle righe che poggiano su un sensore reale.** D8 dice di prendere il sotto-progetto dall'innesco; la condizione **C** della §8.2 della spec copre *«esistono strumenti e permessi da mediare, **e sensori reali da eseguire**»* e vale **4**, *Agenti*. Ma il Passo 4 vuole che una riga 🔶 preesistente cambi **solo se la sede è sbagliata**, e le tre 🔶 che poggiano su un sensore dicono *Conoscenza*, *Coding*, *Coding* — che l'innesco contraddirebbe tutte e tre. ⛔ **La spec scioglie il nodo e va letta prima di D8:** il numero fra parentesi è *«chi la soddisfa per primo oggi»*, non dove la cosa si costruisce, mentre la cella 🔶 dice *«meccanismo deciso, politica o implementazione nel sotto-progetto indicato»*. 📌 **Rimedio, applicato:** per una riga 🔶 di **sensore reale** il sotto-progetto è quello della **capacità**, come il file già fa — `Validazione della mesh prima dell'export` → *Generazione asset*. Il fatto ha casa nella §7.4, §7.4.2 e §7.7 riga 12 del disegno, e la §7.8 lo apre al proprietario; questa voce vi rimanda invece di ripeterlo. |
| **E8** | ⚠️ **COMPITO 3, Passi 4 e 5 — «TRE eccezioni restano in §6» e la terza non ha reso nessuna riga.** Il testo dell'archivio e quello del puntatore, dettati alla lettera, annunciano **tre** eccezioni; la cernita del Passo 3 ne ha trovate **due**. La terza — *«una riga che sia una regola per i piani futuri senza altra casa»* — non ha pescato niente: ogni riga di forma regolativa del blocco ha una casa fuori dal blocco, `E162` compreso, che il Passo 3 dava già per partente. Misurato riga per riga, con le case aperte e lette: la tabella *«La terza eccezione, riga per riga»* della **§7.5** del disegno le elenca con il comando che le rifà. 📌 **Rimedio, univoco e dentro il perimetro, applicato:** il numerale è **tolto** invece che riallineato — il puntatore in §6 dice *«con le eccezioni dichiarate lì»*, e l'intestazione dell'archivio dichiara che la terza non ha reso nessuna riga e rimanda alla §7.5. È la regola 5 della §6.2 del disegno — *si toglie invece di riscrivere meglio* — e il vincolo globale 9. La divergenza sta anche nella §7.7 riga 15. ⚠️ **Seconda divergenza dallo stesso testo dettato, colta dalla revisione e chiusa nella prima ondata:** quel testo chiama *«la riga del marcatore»* ciò che **E5** misura essere un **capoverso** di sei righe. Nel puntatore la parola non compare, e nella §7.5 era già scritta giusta; nell'archivio — che è il documento che sopravvive — è corretta in *«il capoverso del marcatore»*, commit `2f203a2`. |
| **E9** | ⛔ **COMPITO 4 — non si dispaccia finché il proprietario non ha deciso la voce 9 della §7.8 del disegno.** L'audit del compito 1 ha letto ❌ le condizioni **1** e **3** della §0.7: nella §7.2 del disegno le righe `V5`, `V8`, `V9`, `V24`, `V25`, `V35`, `V36`, `V37`, `Q13` e `Q23` nominano un controllo che nessun comando trova, o una tecnica diversa da quella di `design/08` — la classe di **AUD-026** — e la §0.7 dice che il sotto-progetto è fatto quando **tutte** le condizioni sono vere. Il compito 4 è l'unico che scrive «chiuso» (vincolo globale 11), e scriverlo così sarebbe falso; la spec è del proprietario (vincolo globale 1). **Le due vie, nessuna presa:** **A** — ri-giudicare quelle righe in §8, da ✅/⚠️ a ⏳/⚠️ con innesco e richiamo datato, sul precedente di **AUD-026** del 2026-08-27, come compito in più con revisione, poi il compito 4 com'è scritto; **B** — chiudere dichiarando le due ❌ in ogni frase «chiuso» dei documenti di stato, con una voce d'errata che riscriva le frasi del compito 4. Registrata il 2026-09-03 dalla sessione che ha eseguito i compiti 1–3; i compiti 2 e 3 sono proceduti perché i loro commit dicono cose vere a prescindere dalla chiusura (disegno §5.4). |
| **E10** | ✅ **LA VOCE 9 È DECISA DAL PROPRIETARIO IL 2026-09-03, ed è la via A di E9 — con TRE eccezioni MISURATE.** La scelta è **A**: la §8 smette di promettere controlli che non esistono, e il compito 4 va poi com'è scritto. ⛔ **Ma le dieci righe NON sono della stessa specie, e leggerle contro il codice lo mostra** — gotcha **#65** applicato al rapporto del compito 1, e nelle **due** direzioni: chiede **più** del necessario su tre righe e **meno** su una. **(a) `V5` non si declassa: si scrive il caso che la sua riga di catalogo promette dal 2026-08-10.** `RecordV1::intent(effect, trust, payload, reason)` prende la classe **per posizione**, quindi ometterla è `error[E0061]` — la stessa forma del gemello `crates/kernel/tests/compile_fail/record_without_trust_label.rs`, che tiene lo stesso contratto per `trust`; e la seconda metà del vincolo — *«l'assenza vale `irripetibile`»* — è **già tenuta**: `crates/kernel/src/reconcile.rs` manda un record che questa build non decodifica in `Resolution::SuspendAndAsk`, con le sonde in `crates/kernel/tests/reconciliation.rs`. ⛔ **E nessuno dei due inneschi di §8.2 calzava**: la lacuna non aspetta né un consumatore né una misura, e §8.2 dichiara di aver cercato una terza specie e non averla trovata. **(b) `V8` non si declassa: la cella si attribuiva un merito altrui.** Il testo del vincolo è *«ogni run ha un tetto, con default conservativo»*, e §8.1.3 dice che per i V l'autorità è **il testo del vincolo**; il tetto esiste ed è consegnato — `Parameters::executor_turn_limit`, `RunError::TurnLimitReached`, sonda `the_delivered_turn_limit_is_honoured_by_its_value`. La transizione ad `AttesaUmano` che la cella accreditava è il metodo di **Q7**, non di V8 — la stessa forma corretta su `Q17` il 2026-08-08. **(c) `Q13` non si declassa: è la CONDIZIONE 3 a essere scritta più larga della decisione**, e la cura è la §7.3 del disegno, sul precedente della **condizione 4** nella stessa sezione. Il metodo qui non è **assente**: è **più forte**, e **dichiarato dal 2026-08-08**. Declassare una riga il cui controllo eccede il richiesto direbbe il falso nella direzione opposta. **(d) E `V36` NON si tocca affatto** — è la direzione in cui il rapporto chiede meno del vero: il suo ❌ nasce dalla riga di catalogo `V5` senza caso, e sparisce da sé quando il caso esiste. ⛔ **Restano CINQUE righe della specie di AUD-026 — `V9`, `V25`, `V35`, `V37`, `Q23` — che scendono a ⏳ rimandato**, più `V24` che resta ⚠️ e perde il picco VRAM dalla metà verificata. ⛔ **IL VINCOLO GLOBALE 1 È SOSPESO DAL PROPRIETARIO** per il solo compito 3bis e per le sole righe che quel compito elenca: la spec si tocca, e **solo** in §8.3 e §8.4. ⛔ **Il vincolo globale 2 resta INTERO:** nessuna riga ⚠️ o ⏳ passa a ✅, e nessuna riga di catalogo nasce — `V5` è già ✅ e ci resta, e ciò che nasce è il **caso**, non la riga. ⛔ **Il vincolo globale 3 cede su [`porta-di-qualita.md`](../../porta-di-qualita.md)** per la clausola che già porta — *«salvo una riga falsa trovata dall'audit»* — e per il passo **5** della disciplina dell'audit del 2026-08-27: *un rimedio riconta i conteggi che ha reso stantii*. |
| **E11** | ⛔ **IL COMPITO 3bis FA CINQUE I COMPITI DI QUESTO PIANO, E LA CIFRA «QUATTRO» VIVE IN SEI CASE.** Censite col comando il 2026-09-03 — `grep -n 'quattro compiti\|Quattro compiti'` su questo piano e sul disegno. ⚠️ **DUE sono VERBALI DATATI e restano**: la riga *«IL PIANO È SCRITTO IL 2026-09-02 — quattro compiti»* di questo file, e la scelta **4** del disegno, che registra la proposta accolta quel giorno — *dentro un verbale datato una cifra regge, dentro una voce viva mente* (**55ª** misura). ⛔ **QUATTRO sono affermazioni VIVE**: la *«Forma»* in testa a questo piano · la giustificazione di **D2** · la riga di §12 che il **compito 4** detta per il compendio · la §5.4 del disegno. ⛔ **La cura è TOGLIERE il numerale, non riallinearlo a cinque** — precedenti **AUD-007** e **AUD-046**, *un elenco invecchia, una regola no* — perché un piano che guadagna un compito può guadagnarne un altro, e la posizione ha già una **casa unica**: la tabella di questo piano. ⚠️ **La riga di §12 conta più delle altre tre:** è testo che finirebbe in [`COMPENDIO.md`](../../COMPENDIO.md), cioè in un **documento di stato**, dove una cifra stantia è la radice **R1** in persona. 📌 **E `D2` ha una seconda metà da sistemare:** la sua mappa dice quale compito scrive quale sottosezione della §7, e non nomina il 3bis — che **non apre nessuna intestazione nuova** e scrive dentro 7.2, 7.3 e 7.8, tutte già esistenti. Il rimedio di entrambe è il Passo 7 del compito 3bis. Trovata dal pre-controllo del compito 3bis, domanda **5** — *il contratto cresce sotto il piano, e un compito si legge contro il repository di ADESSO*. |
| **E12** | ⛔ **IL COMPITO 3bis SCRIVE PRODOTTO, E TRE AFFERMAZIONI VIVE DI QUESTO PIANO LO NEGAVANO.** **E11** aveva colto il solo numerale *«quattro compiti»*; le altre tre sono della stessa specie e stanno nella stessa pagina. ① La **«Forma»** in testa diceva *«Nessuna riga di prodotto: la chiusura è un audit più TRE passate documentali»*, e il 3bis scrive `crates/kernel/tests/compile_fail/effect_without_its_class.rs` col proprio `.stderr`. ② La **mappa dei file** non aveva nessuna riga per il 3bis — né per la spec, né per il registro, né per i due file nuovi. ③ La riga sotto la mappa diceva *«Nessun file di `crates/` cambia in nessun commit»* e dettava al **compito 4** un comando la cui attesa era *«solo `scripts/check-docs.sh`»*: misurato il 2026-09-03, `git diff --name-only 9214255..HEAD -- crates/ scripts/ Cargo.lock` nomina anche i due file del caso. 📌 **Rimedio, applicato:** i numerali si **tolgono** e non si riallineano (**AUD-007**, **AUD-046**), la mappa guadagna le tre righe, e la riga del comando riceve il **richiamo datato** con l'attesa corretta per il compito 4. ⚠️ **E una quarta cifra cresce di uno e NON si tocca, perché vive dentro un verbale datato:** i casi `compile_fail` contati dal blocco **H** della §1.3 e ripetuti nella riga della condizione **2** della §7.2 sono l'istantanea dell'audit sul commit `9214255`; un rilancio oggi ne rende **uno in più**, ed è il caso che questo compito ha scritto. Trovata dal pre-controllo del compito 3bis, domanda **5** — *il contratto cresce sotto il piano* |
| **E13** | ⚠️ **DUE RITOCCHI AL TESTO DETTATO DEL COMPITO 3bis, dentro il perimetro e dichiarati invece che taciuti.** ① Il **Passo 7** elenca le celle della §7.2 che ricevono una riga e **non nomina `V36`**, la cui cella direbbe ancora *«la riga di catalogo su cui questa poggia non ha nessun caso che la eserciti»* — falso dal commit del prodotto. Le è stata aggiunta **una** riga, della stessa forma delle altre; ⛔ **la §8 NON è toccata**, quindi la lettera di **E10 (d)** — *«`V36` non si tocca affatto»* — resta intera. ② Il **Passo 6** detta per la §7.3 la formula *«già registrata da tre traguardi»*: il numerale non è verificabile e il **vincolo globale 4** vieta una cifra nuova in prosa. Al suo posto c'è la **data** che la cella di `Q13` porta — **2026-08-08** — che non marcisce. 📌 **E una terza cosa NON è stata fatta, ed è dichiarata:** la riga nuova della §7.3 è la **seconda** della tabella e viene dopo la condizione **4**, come il Passo 6 detta (*«se ne aggiunge una seconda»*); riordinarle riscriverebbe un verbale |
| **E14** | ⛔ **COMPITO 4, Passo 2 — la sostituzione della cella di `roadmap.md` si ferma UN CARATTERE PRIMA, e lascia un `**` ORFANO.** Il Passo detta *«da `🔵 **in corso**` fino a `in un posto solo` compreso»*; misurato il 2026-09-03, la riga `| 1 | Implementazione del kernel + simulatore DST |` finisce con `in un posto solo** | 0, 0b, 0c |`, e il testo di sostituzione dettato chiude con *«P-2 del piano»* **senza riaprire nessun grassetto**: fermarsi a `in un posto solo` lascia `**` attaccato alla colonna successiva. 📌 **Rimedio: il confine è `in un posto solo**`, grassetto di chiusura INCLUSO.** ⚠️ **E una cifra del pre-controllo del 2026-09-02 è divergita, registrata e non appianata:** quel pre-controllo dava `in un posto solo` a **tre** occorrenze in `roadmap.md`, oggi sono **quattro** — la quarta nella cella del Traguardo 5, che ne porta due. ✅ **Il ruling regge lo stesso, misurato:** nella riga bersaglio l'occorrenza è **una**, quindi la sostituzione **scopata a quella riga** resta univoca. Trovata dal pre-controllo del compito 4, domanda **5** — *il contratto cresce sotto il piano* |
| **E15** | ⛔ **TRE FRASI VIVE DELLA SPEC, FUORI DA §8.3 E §8.4, CHE IL COMMIT DEL COMPITO 3bis HA RESO FALSE — REGISTRATE E NON CORRETTE.** ⚠️ *La voce **E14** è presa dal coordinatore per un difetto del compito 4: il buco fra E13 ed E15 è voluto.* La sospensione del vincolo globale 1 copre le sole **§8.3** e **§8.4** (**E10**), quindi correggerle sfonderebbe il perimetro fissato dal proprietario — e *un rimedio può fermarsi prima di decidere*, precedente **AUD-004**. ① **§8.1.1**, la tabella *«Perché quattro e non tre»*, che **giustifica il vocabolario a quattro stati** con **due sole** righe d'esempio: il commit ha portato a ⏳ **entrambe** le righe della prima — `V25` e `Q20` — togliendole la propria premessa, *«sottovaluta: il controllo gira già a ogni commit, e la sonda scatta»*, che il richiamo nuovo di `V25` dichiara **falsa** (*«nessuno script guarda `daemon`»*). La trova `grep -n 'Perché quattro e non tre' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`. ② **§8.1.3**, capoverso *«Conseguenza visibile»*: dice che `V9` *«è invece ⚠️»* e che *«quel test è eseguibile qui»*, in prosa **viva e non datata**, e `V9` è ora ⏳ perché l'evento non ha soggetto. La trova `grep -n 'Conseguenza visibile' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`. ③ **§0.4.1**: *«quindi V35 non è rimandabile»*, dentro il capoverso che la spec chiama *«quello su cui voglio più attenzione in revisione»*, e `V35` è ora ⏳. La trova `grep -n 'non è rimandabile' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`. ⚠️ **Sulla ③ le due parole *«rimandabile»* hanno sensi diversi** — scaglionamento del **meccanismo** contro esistenza del **controllo** — e proprio per questo va detta invece che lasciata a chi legge. 📌 **Chi la chiude è il proprietario**, e la voce **14** della §7.8 del [disegno](../specs/2026-09-02-sottoprogetto-1-chiusura-design.md) la apre. Trovata dalla revisione 1 del compito 3bis, rilievo **I3**; i tre comandi rilanciati il 2026-09-03 |
| **E16** | ⛔ **`grep` CON PIÙ DI UN PATTERN E `-i` ABORTISCE SU QUESTA MACCHINA, E LA CORREZIONE DETTATA PER IL RILIEVO C1 LO USAVA.** Misurato il 2026-09-03: `grep --version` dice `grep (GNU grep) 3.0`, e quel binario muore con `Aborted` — uscita **134** — appena riceve più di un pattern **insieme** a `-i`, in qualunque forma (due `-e`, o un pattern solo con un a-capo dentro). Il caso minimo: `printf 'a\nb\n' \| grep -i -e a -e b` → `Aborted`; **senza** `-i` lo stesso comando rende **due** righe e uscita `0`. ⛔ **La consegna dell'ondata di correzione 1 prescriveva `grep -rniE -e confinement -e sandbox crates/ --include=*.rs`, che qui NON gira**; e `-i` è **portante** su quelle celle, perché i soggetti cercati sarebbero `AwaitingHuman` e `Confinement`, che una ricerca sensibile alle maiuscole non troverebbe — toglierlo renderebbe la sonda vacua in un modo nuovo. 📌 **Rimedio, applicato:** dove `-i` serve si scrive **un comando per termine** — `grep -rni confin crates/ --include=*.rs`, e lo stesso con `sandbox` — che non ha tubi, gira, e conserva `-i`; dove `-i` non serve, come su `V9`, la forma `-e` ripetuta è quella dettata e gira. ⚠️ **Non è una preferenza di stile ma una misura**, e vale per chiunque scriva in questo repository un `grep` con alternanza. ⛔ **RICHIAMO DEL 2026-09-03:** qui seguiva *«sopra §8.3 pesa due volte, perché quella tabella è letta per posizione e un tubo le costa una colonna»*, ed è **falso** — un tubo **scappato** non costa nessuna colonna, perché `check-docs.sh` lo toglie davvero. **Tolto, non riscritto:** la misura sta in **E17**. Trovata eseguendo l'ondata di correzione 1 del compito 3bis |
| **E17** | ⛔ **IL RILIEVO «CRITICAL» DELLA REVISIONE DEL COMPITO 3bis ERA FALSO, E LO STESSO ERRORE DI MISURA È STATO COMMESSO DA DUE AGENTI DI SEGUITO — IL REVISORE E IL COORDINATORE.** La revisione dichiarava che le tre celle riscritte in §8.3 rompevano la lettura **per posizione** di `scripts/check-docs.sh` — `V8 n=9`, `V9 n=8`, `V37 n=10` contro **7** — e che la guardia della §8.6 era diventata **vacua**; il coordinatore ha creduto di **riprodurlo** e ha dispacciato l'ondata 1 su quella premessa. ⛔ **È FALSO, e a smentirlo è stata la ri-revisione mirata, rimisurando da un FILE invece che da riga di comando.** ✅ **La causa, isolata sui byte:** lo `gsub` di `check-docs.sh` porta **tre** barre rovesce prima del tubo — `od -An -tx1` rende `5c 5c 5c 7c` — e in quella forma toglie davvero la coppia `\|`; scritto con **due** barre diventa *«barra rovescia OPPURE vuoto»*, toglie la sola barra e lascia il tubo a separare. Provato su tre programmi identici tranne il numero di barre, sulla riga `V8` **vera** del commit `1efc5c7`: due barre → `n=9`, tre barre → `n=7`, una barra → `n=1`. ⛔ **Il difetto non era nel repository ma nel CANALE:** un programma `awk` scritto **in linea** attraversa il quoting della chat e della shell e può perdere una barra rovescia senza che nessuno lo veda, e la misura che ne esce è falsa **in modo plausibile**. 📌 **La cura è una riga: un programma `awk` si scrive in un FILE e si lancia con `awk -f`**, dove nessuno lo ri-scappa. ✅ **Che cosa REGGE dell'ondata, e non si revoca:** il rilievo `I1` è **vero e indipendente** — `\|` dentro `grep -E` è un tubo **letterale**, e la forma vecchia esce **1** senza nessuna riga dove col tubo vero ne rende tre — quindi la riscrittura delle tre celle **era dovuta**; e `I2`, `I3`, `E15` ed `E16` non poggiano su C1. ⛔ **Che cosa CADE:** la guardia della §8.6 **non è mai stata cieca**, nemmeno prima del commit `3042a19` — provato svuotando l'innesco di `V9` e poi di `V37` sul blob pre-correzione, con `check-docs.sh` **rosso** in entrambi i casi. Il **messaggio del commit `ef8b89a`** — *«la guardia della §8.6 torna a mordere»* — resta scritto e **non si riscrive**: è un verbale, e questa voce è la sua correzione. Trovata dalla ri-revisione mirata dell'ondata 1 |

---

## Il pre-controllo del piano — che cosa il disegno dice e il repository smentisce

Letto scrivendo, contro il repository a `e26142b`, il 2026-09-02.

### P-1 — Il disegno conta «sei» link nel blocco da archiviare, e il comando ne rende otto

La §4.4 del disegno dice *«I link relativi che contiene — sei il 2026-09-02 — prendono il
prefisso `../`»*. Rilanciato lo stesso giorno:

```bash
sed -n '/^### Il prossimo passo/,/^✅ \*\*I finding dell/p' docs/COMPENDIO.md | head -n -1 | grep -o '](\([^)#]*\.md\)[^)]*)' | sort -u
```

rende **otto** bersagli distinti — fra cui `../CLAUDE.md`, che col prefisso diventa
`../../CLAUDE.md`. La cifra veniva dalla consegna, misurata prima che la propria riga entrasse nel
blocco. ✅ **Corretta nel disegno con un richiamo datato, nello stesso commit di questo piano:**
la cifra è **tolta**, resta il comando. Per il compito 3 conta la regola, non il numero: **ogni**
bersaglio relativo prende il prefisso.

### P-2 — La cella di `roadmap.md` porta una voce «registrata e non presa» che il disegno chiude

La riga 1 della tabella dei sotto-progetti dice di sé: *«l'elenco per traguardo di questa cella è
la TERZA casa dello stesso stato in QUESTO file … Toglierlo invece di allungarlo tocca la
struttura del documento, quindi è del proprietario»*. La §5.2 del disegno — **approvata dal
proprietario** — prescrive esattamente quella sottrazione: *«Il racconto esce: lo stato per
traguardo ha già la sua tabella nello stesso file»*. ⛔ **Chi esegue il compito 4 non si ferma
su quella frase:** la decisione è presa, e il verbale lo scrive.

### P-3 — «`git diff --stat` nomina esattamente tre file» e il verbale nel disegno non stanno insieme in un commit

La §4.8 del disegno vuole che il diff del compito 3 nomini **tre** file; la §6.2 regola 6 vuole il
verbale nella §7 del **disegno**, che è un quarto file. Le due frasi reggono entrambe se il
compito consegna **due commit**: il prodotto, con tre file esatti, e il verbale. È la **D3**, e
vale anche per il compito 2 per simmetria.

### P-4 — Il blocco della §6 comincia con un'intestazione, e il disegno non dice che cosa ne fa

*«Il blocco che va da `### Il prossimo passo` …»*: il marcatore d'inizio è un'**intestazione**.
Archiviarla metterebbe un titolo *«Il prossimo passo»* dentro un verbale — un secondo puntatore,
morto — e lascerebbe la riga viva del marcatore sotto l'intestazione dei sei traguardi. È la
**D1**: l'intestazione resta in §6 come delimitatore di ciò che rimane.

### P-5 — La §6 porta OGGI due righe di stato dopo la testa del blocco, e una sola resta

La riga che apre con il marcatore del prossimo passo, e sotto di essa la riga *«il brainstorming
è fatto … il disegno è scritto»*, riscritta due volte lo stesso giorno. La prima è l'eccezione
che la §4.1 del disegno tiene in §6; la seconda è un **verbale** e parte col blocco, come la §4.1
dice. ⛔ **Il compito 3 riconosce la riga viva dal marcatore, non dal contenuto:** che cosa dica
quel giorno lo decide la sessione che l'ha scritta per ultima, e il compito 4 la riscrive comunque.

### P-6 — I commenti di `scripts/check-docs.sh` sono inglesi, e il disegno non lo dice

La §4.6 vuole *«il richiamo datato del blocco di commento di `scripts/check-docs.sh`»* e non ne
dice la lingua. Il file è **codice**, e i due richiami che porta — `RECALL OF 2026-09-01` — sono
in inglese: §1.0 della spec. Vincolo globale 5.

### P-7 — La condizione 6 della §0.7 nomina `tracciabilita.md` nel passaggio unico, e il compito 2 la tocca prima

*«roadmap, tracciabilità, spike e HANDOFF aggiornati nello stesso passaggio»*, e l'evidenza del
disegno è *«`git show --stat HEAD` li nomina tutti»*. Se `tracciabilita.md` fosse toccata **solo**
dal compito 2, il commit del compito 4 non la nominerebbe. È la **D4**: il riquadro in testa porta
**due** date — quella della rilettura (compito 2) e quella della chiusura (compito 4) — e la
seconda la scrive il passaggio unico.

### P-8 — Il compito 2 ha già un caso di «sede da assegnare», misurato

La ritenzione a livelli di ADR-0018: `V26` ha l'innesco senza numero e `grep -ci ritenzion
docs/roadmap.md` rende **zero**. Quante righe di `tracciabilita.md` poggino su quel meccanismo lo
dice la rilettura; che almeno il caso esista è già scritto nel disegno (§3.2), e il compito 2
lo incontra per primo.

### P-9 — Il compendio guadagna righe al compito 4, e il tetto sceso al compito 3 deve contenerle

Il passaggio unico aggiunge alla §6 e alla §12 del compendio: quattro righe in §12, il comando
del ritratto pieno, un richiamo in §6, la riga del marcatore riscritta. Il tetto della §4.6 è
*«misurato più circa l'undici per cento»*, e l'undici per cento è dichiarato dal disegno per
questo. ⛔ **Il compito 4 misura il margine col comando del blocco E prima e dopo**, e se il
cancello andasse rosso per il tetto la risposta **non** è alzare il tetto: è togliere prosa dalla
§6, che è dove il compito 4 scrive.

---

## Le decisioni prese da questo piano

⛔ **Sono decisioni del piano, non del disegno, e chi esegue può ribaltarle** portando la misura
che le smentisce — è ciò per cui esiste l'errata.

| | Decisione | Perché |
|---|---|---|
| **D1** | l'intestazione `### Il prossimo passo` **resta in §6** come delimitatore; il blocco archiviato comincia dalla riga **dopo** | P-4. Un'intestazione è struttura, non verbale; e il marcatore vivo deve avere un'intestazione sopra di sé |
| **D2** | il verbale è la **§7 del disegno**, con nove sottosezioni numerate una volta sola qui: **7.1** i conteggi · **7.2** le cinque condizioni · **7.3** dove la condizione era scritta troppo larga · **7.4** la rilettura di `tracciabilita.md` · **7.5** l'archiviazione e il tetto · **7.6** il passaggio unico · **7.7** dove il disegno è stato smentito, e dove ha retto · **7.8** le voci aperte, con chi le chiude · **7.9** che cosa la chiusura NON ha fatto. Il compito 1 scrive 7.1, 7.2, 7.3, 7.7, 7.8; il 2 inserisce 7.4; il 3 inserisce 7.5; il **3bis** non apre nessuna intestazione nuova e scrive **dentro** 7.2, 7.3 e 7.8 (**E11**); il 4 scrive 7.6 e 7.9, completa 7.2 e **data** l'intestazione | disegno §6.2 regola 6. Numerate qui perché più compiti scrivono nello stesso capitolo e nessuno vede gli altri: un compito che inventasse la propria numerazione produrrebbe un duplicato, che `check-docs.sh` respinge |
| **D3** | i compiti **2** e **3** consegnano **due commit**: il prodotto, poi il verbale nel disegno | P-3. Il diff del prodotto resta misurabile come il disegno lo prescrive — un file per il 2, tre per il 3 |
| **D4** | il riquadro in testa a `tracciabilita.md` porta **due** date: la rilettura, scritta dal compito 2, e la chiusura, scritta dal compito 4 | P-7 |
| **D5** | le violazioni deliberate della condizione 2 sono **una per script**, scelte qui: `gate-no-os.sh` ← `extern crate std;` in `crates/kernel/src/lib.rs`; `gate-deps.sh` ← `unty` rinominato nella lista `SHIPPED` dello script, che produce **un intruso e un fantasma insieme**; `gate-attributes.sh` ← `forbid` → `deny`; `check-docs.sh` ← un `.md` **non tracciato** con un link rotto | ciascuna colpisce il meccanismo che quello script esiste per tenere, non un caso accanto; le prime tre sono sostituzioni **su una riga sola**, quindi non toccano i fine-riga; la quarta non tocca nessun file esistente |
| **D6** | ogni compito **rimisura** la baseline — cancello, `check-docs.sh`, e i blocchi della §1.3 del disegno che lo riguardano — e non cita quella scritta qui | gotcha **#31**. Quella di partenza sta scritta **una volta sola**, sotto questa tabella |
| **D7** | la forma della riga 🔶 è quella che la legenda e le venticinque righe già 🔶 usano: `<meccanismo> §N · <che cosa si scaglia> · implementazione → <nome del sotto-progetto>`; senza sede: `… · implementazione → sede da assegnare` | coerenza col file: `grep -E '^\| .* \| 🔶 \|' docs/tracciabilita.md` mostra la forma |
| **D8** | il sotto-progetto di un meccanismo scaglionato si prende dall'**innesco** della riga V o Q che lo governa in §8.3/§8.4 — lettera più numero — e il **nome** dalla tabella dei sotto-progetti di `roadmap.md`. Misurato il 2026-09-02: A→2 *GUI minima*, B→3 *Conversazione*, C→4 *Agenti*, D→5 *Coding*, E→7 *Generazione asset*, F→11 *Backup e ripristino*, SP-2→8 *Voce* | disegno §3.1 fonte 2 e 3; il comando che lo rifà è il blocco **B** della §1.3 del disegno, con la colonna dell'innesco letta intera |
| **D9** | un `.md` di sonda per `check-docs.sh` si chiama `docs/zz-sonda-condizione-2.md`, vive **solo** durante la prova e si cancella prima di qualunque commit | il cancello legge i file non tracciati e non ignorati, ed è la proprietà che la prova esercita |

**La baseline di partenza, misurata il 2026-09-02 e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `bash scripts/check-docs.sh` → `OK` · i blocchi **A**–**I**
della §1.3 del disegno rendono le cifre che quella tabella porta.

---

## La mappa dei file

| File | Chi lo tocca | Fine-riga | Responsabilità |
|---|---|---|---|
| `docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md` | compiti 1, 2, 3, 4 | LF | la **§7**, il verbale (D2) |
| `docs/tracciabilita.md` | compiti 2, 4 | LF | le righe 🔶, il riquadro in testa (D4) |
| `docs/COMPENDIO.md` | compiti 3, 4 | **CRLF** | §6: il blocco esce, il puntatore entra; §6, §8, §12 e l'intestazione al compito 4 |
| `docs/archivio/stato-storico.md` | compito 3 | LF | la sezione nuova col blocco verbatim |
| `scripts/check-docs.sh` | compito 3 | **CRLF** | `ceiling=` e il richiamo datato, in inglese |
| `docs/roadmap.md` | compito 4 | LF | data, riga 1 dei sotto-progetti, due righe nella tabella dei piani |
| `docs/README.md` | compito 4 | **CRLF** | la fase corrente, due righe nella tabella delle spec |
| `docs/HANDOFF.md` | compito 4 | **CRLF** | intestazione, la riga «NON è chiuso», il punto di ripresa, due celle della mappa |
| `docs/AVVIO-CHAT.md` | compito 4 | **CRLF** | una riga del messaggio, per sottrazione |
| `docs/semi-dst.md` | compito 4 | **CRLF** | un richiamo datato per le due campagne del Traguardo 6 |
| `docs/superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md` | ogni compito | LF | la tabella della posizione, e l'errata |
| `crates/kernel/src/lib.rs` · `scripts/gate-deps.sh` | compito 1, **mutati e ripristinati byte-esatti** | CRLF nell'albero, LF nell'indice | le violazioni deliberate della condizione 2. ⛔ **Nessun commit li tocca** |
| `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | compito **3bis**, e **solo** in §8.3 e §8.4 | **CRLF** | le righe della §8 che smettono di promettere controlli che non esistono. ⛔ Il vincolo globale **1** è **sospeso dal proprietario** per quel compito e per quelle due tabelle soltanto (**E10**) |
| `docs/porta-di-qualita.md` | compito **3bis** | **CRLF** | la ricontata del blocco C che il caso nuovo ha reso dovuta, e il richiamo al capoverso diventato falso. ⛔ Il vincolo globale **3** cede per la clausola che già porta — *«salvo una riga falsa trovata dall'audit»* (**E10**) |
| `crates/kernel/tests/compile_fail/effect_without_its_class.rs` e il suo `.stderr` | compito **3bis**, **creati** | CRLF nell'albero, LF nell'indice | il caso che la riga di catalogo `V5` prometteva dal 2026-08-10. ⛔ **Nessuna riga di catalogo nasce**: la §7.4 è spec |

⛔ **RICHIAMO DEL 2026-09-03, voce E12 — questa riga diceva *«Nessun file di `crates/` cambia in
nessun commit»*, e il compito 3bis l'ha smentita.** Il comando che il compito 4 lancia — il
vincolo globale 1 esteso a `crates/ scripts/ Cargo.lock` — nomina ora `scripts/check-docs.sh`
del compito 3 **e i due file del caso `compile_fail`** del compito 3bis, e nient'altro:
misurato il 2026-09-03 con `git diff --name-only 9214255..HEAD -- crates/ scripts/ Cargo.lock`.
⛔ **Nessun file di produzione sotto `crates/*/src/` è cambiato**, e resta vero che i due file
mutati per la condizione 2 non entrano in nessun commit.

---

## Compito 1: l'audit — le condizioni 1–5 della §0.7, rilette contro il codice

**Files:**
- Modify: `docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md` — la **§7** nuova, in coda al file
- Modify: questo piano — la tabella della posizione, e l'errata se il compito trova un difetto
- Read: la spec, §0.7, §8.1.2, §8.3, §8.4, §2.4 · [`porta-di-qualita.md`](../../porta-di-qualita.md) · [`design/08`](../../design/08-strategia-di-test.md) · `crates/kernel/tests/compile_fail/` · `scripts/*.sh` · [`semi-dst.md`](../../semi-dst.md) · `docs/adr/003[1-7]-*.md`
- Mutati e ripristinati, **mai committati**: `crates/kernel/src/lib.rs`, `scripts/gate-deps.sh`

⛔ **È un AUDIT, non una scrittura** — gotcha **#49**. La prima domanda è *«è già fatto?»*, e la
risposta si scrive: ciò che si trova soddisfatto si **riconosce**, non si riesegue. Le
condizioni **non si ricopiano qui**: si leggono nella §0.7 della spec e nella §2.2 del disegno,
che per ciascuna dice che cosa conta come evidenza.

- [ ] **Passo 1: la baseline, e i conteggi, col comando e prima di leggere**

```bash
bash scripts/gate.sh
bash scripts/check-docs.sh
git status --porcelain
```

Atteso: `GATE GREEN`, `OK`, e nessuna riga da `git status`. Poi i blocchi **A**, **F**, **G**,
**H** della §1.3 del disegno, rilanciati: le cifre si scrivono nella **§7.1** con la data di oggi,
accanto al nome del blocco. ⚠️ Se un blocco diverge dal disegno, **vince il comando**, e la
divergenza va in §7.7.

- [ ] **Passo 2: condizione 1 — ogni V in perimetro ha un controllo che esiste**

Le righe sono le ✅ e le ⚠️ di §8.3. Si estraggono così, e si leggono **una per una**:

```bash
awk '/^## 8\. /{i=1} i && /^\|[[:space:]]*V[0-9]+[[:space:]]*\|/ {r=$0; gsub(/\\\|/,"",r); split(r,c,"|"); if (c[4] ~ /verificato qui|parziale/) print c[2] "|" c[4] "|" c[5]}' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

Per ogni riga la cella del meccanismo deve nominare **una delle tre risposte** di §8.1.2 — una
riga del catalogo §7.4.1 o §7.4.2, un test a esempi, una campagna DST con la sua porta — e
quella cosa deve **esistere**:

| Se nomina | Come si prova che esiste |
|---|---|
| una riga di catalogo (`§7.4.1 C, riga V2`, `§7.4.2`, un blocco A/B/C) | si cerca l'identificativo della riga nel [registro](../../porta-di-qualita.md) — `grep -n 'V2 \|riga V2' docs/porta-di-qualita.md` — che nomina il file e le sonde; poi `ls` sul file, e `grep -rn 'fn <sonda>' crates/` su almeno una sonda nominata |
| un test per nome | `grep -rn 'fn <nome>' crates/` |
| una campagna | `ls` sul file: `crates/simulator/tests/*campaign*.rs` o `crates/platform/tests/engine_crash_consistency.rs` |
| nessuna delle tre, o una cosa che non esiste | ❌, con che cosa manca. ⛔ **La spec NON si corregge** (vincolo globale 1): è la classe di AUD-026, e la §8 è del proprietario |

Per una riga ⚠️ **parziale** si prova la sola **metà verificata** — la cella la separa da ciò che
manca. Il risultato è una tabella in **§7.2**, sotto la riga della condizione 1: `V`, stato, che
cosa nomina, dove l'ho trovato, esito.

- [ ] **Passo 3: condizione 2, livello 1 — ogni caso `compile_fail` ha il suo `.stderr` e la suite è verde**

```bash
ls crates/kernel/tests/compile_fail/*.rs | wc -l
ls crates/kernel/tests/compile_fail/*.stderr | wc -l
find crates/kernel/tests/compile_fail -name '*.stderr' -empty | wc -l
for f in crates/kernel/tests/compile_fail/*.rs; do [ -s "${f%.rs}.stderr" ] || echo "SENZA STDERR: $f"; done
cargo test --locked -p kernel --test compile_fail
```

Atteso il 2026-09-02: due cifre uguali, zero vuoti, nessuna riga `SENZA STDERR`, suite verde. Si
scrive in §7.2 con le cifre **di oggi** e il comando.

- [ ] **Passo 4: condizione 2, gli script — ogni controllo statico visto fallire e tornare verde**

⛔ **Si rilancia, non si cita**: le prove in negativo degli script vivono sparse fra il piano del
Traguardo 1, l'audit e la spec, e gli script sono cambiati dopo. Prima, l'aiutante che sostituisce
**una** stringa **una** volta conservando i fine-riga — nello scratchpad, non nel repository:

```python
# mutate.py -- python mutate.py FILE OLD NEW ; refuses unless OLD occurs exactly once
import io, os, sys
p, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
s = io.open(p, encoding="utf-8", newline="").read()
assert s.count(old) == 1, "occurrences: %d" % s.count(old)
out = s.replace(old, new).encode("utf-8")
tmp = p + ".tmp"
with open(tmp, "wb") as f:
    f.write(out)
os.replace(tmp, p)
```

Poi, **una prova per volta**, ciascuna nella forma *copia · muta · rosso letto · ripristina ·
`cmp` · verde*. `$S` è lo scratchpad.

**`gate-no-os.sh`** — il kernel che raggiunge `std` non deve compilare senza OS:

```bash
cp crates/kernel/src/lib.rs "$S/lib.rs.orig"
python "$S/mutate.py" crates/kernel/src/lib.rs 'extern crate alloc;' 'extern crate alloc; extern crate std;'
bash scripts/gate-no-os.sh; echo "exit=$?"
cp "$S/lib.rs.orig" crates/kernel/src/lib.rs && cmp crates/kernel/src/lib.rs "$S/lib.rs.orig" && echo RESTORED
bash scripts/gate-no-os.sh; echo "exit=$?"
```

Atteso: prima corsa `exit=1` con `kernel or simulator do NOT build for x86_64-unknown-none` e,
nell'uscita di cargo, `can't find crate for `std``; `RESTORED`; seconda corsa `exit=0`.

**`gate-deps.sh`** — una crate spedita fuori lista, e una voce di lista che nessun grafo
raggiunge, nella **stessa** mutazione:

```bash
cp scripts/gate-deps.sh "$S/gate-deps.sh.orig"
python "$S/mutate.py" scripts/gate-deps.sh 'unty"' 'unty_renamed"'
bash scripts/gate-deps.sh; echo "exit=$?"
cp "$S/gate-deps.sh.orig" scripts/gate-deps.sh && cmp scripts/gate-deps.sh "$S/gate-deps.sh.orig" && echo RESTORED
bash scripts/gate-deps.sh; echo "exit=$?"
```

Atteso: `exit=1` con **entrambi** i messaggi — `I3 violated -- kernel ships 'unty', which is not
on the list.` e `phantom entry -- 'unty_renamed' is on SHIPPED and in NO graph.` — poi `RESTORED`
ed `exit=0` con `OK -- the two graphs match the two lists`. ⚠️ Se il primo messaggio nomina
`simulator` prima di `kernel`, è lo stesso rosso: la lista delle crate è `kernel simulator` e
il ciclo le percorre in quell'ordine, ma **entrambe** spediscono `unty`.

**`gate-attributes.sh`** — `deny` dove serve `forbid`:

```bash
cp crates/kernel/src/lib.rs "$S/lib.rs.orig"
python "$S/mutate.py" crates/kernel/src/lib.rs '#![forbid(unsafe_code)]' '#![deny(unsafe_code)]'
bash scripts/gate-attributes.sh; echo "exit=$?"
cp "$S/lib.rs.orig" crates/kernel/src/lib.rs && cmp crates/kernel/src/lib.rs "$S/lib.rs.orig" && echo RESTORED
bash scripts/gate-attributes.sh; echo "exit=$?"
```

Atteso: `exit=1` con `declares 'deny(unsafe_code)' where 'forbid' is required` **e** `does not
declare '#![forbid(unsafe_code)]'`; `RESTORED`; `exit=0`.

**`check-docs.sh`** — un documento non tracciato con un link rotto, letto perché il cancello
gira **prima** del commit (D9):

```bash
printf '%s\n' '# sonda' '[rotto](non-esiste.md)' > docs/zz-sonda-condizione-2.md
bash scripts/check-docs.sh; echo "exit=$?"
rm docs/zz-sonda-condizione-2.md
bash scripts/check-docs.sh; echo "exit=$?"
git status --porcelain
```

Atteso: `exit=1` con `broken link: ./docs/zz-sonda-condizione-2.md -> non-esiste.md`; poi
`exit=0` con `OK`, e `git status --porcelain` **vuoto**.

Alla fine, la prova che nulla è rimasto mutato:

```bash
git status --porcelain; git diff --stat
```

Entrambi vuoti. Il risultato è una tabella in §7.2, sotto la riga della condizione 2: script,
violazione, messaggio letto, exit, revoca provata.

- [ ] **Passo 5: condizione 3 — ogni Q in perimetro è verificato col metodo di `design/08`**

Le righe sono le ✅ e le ⚠️ di §8.4:

```bash
awk '/^## 8\. /{i=1} i && /^\|[[:space:]]*Q[0-9]+[[:space:]]*\|/ {r=$0; gsub(/\\\|/,"",r); split(r,c,"|"); if (c[4] ~ /verificato qui|parziale/) print c[2] "|" c[4] "|" c[5]}' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
grep -E '^\| Q[0-9]+ \|' docs/design/08-strategia-di-test.md
```

Per ciascuna: la **tecnica** che `design/08` assegna a quel Q — l'ultima colonna della sua
tabella — e ciò che la cella di §8.4 nomina come controllo. Coincidono, o no. Un Q la cui cella
nomina una tecnica **diversa** è ❌, registrato e non corretto. Tabella in §7.2 sotto la riga
della condizione 3: `Q`, tecnica di `design/08`, che cosa nomina §8.4, coincide.

- [ ] **Passo 6: condizione 4 — il seme, la proprietà, e la lettera che è più larga della decisione**

```bash
grep -nE '^\| V31 ' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | cut -c1-200
awk '/^## L.elenco/{f=1} f && /^\|/' docs/semi-dst.md
grep -c 'campaign' scripts/gate.sh
```

Atteso: la riga di **§8.3** — non quella di §7.4.2, che il primo comando rende pure — porta
`✅ verificato qui` e *«debole per natura»*; l'elenco di `semi-dst.md` ha la sola riga che si
dichiara **vuota e non dimenticata**; il cancello del Passo 1 è verde su tutte le campagne. La
condizione è ✅ **salvo il richiamo** per le due campagne del Traguardo 6, che scrive il compito
4 (disegno §5.2) — si scrive così, non «✅».

⛔ **E si scrive la §7.3.** La lettera della condizione — *«conserva il proprio seed come caso di
regressione permanente»* — è più larga della decisione: ADR-0021 dal 2026-08-08 dice che a
diventare regressione è la **proprietà**, e la cella di V31 lo ripete. La condizione è
soddisfatta **nella lettura di ADR-0021**, e la §0.7 resta del proprietario: è la voce **5** della
tabella del disegno, che il Passo 8 riporta in §7.8.

- [ ] **Passo 7: condizione 5 — gli ADR della §0.5, e la riga 3 che vive in §2.4**

```bash
ls docs/adr/003[1-7]-*.md
grep -c 'Negative (accettate)' docs/adr/003[1-7]-*.md
grep -n 'decisione n\. 3\*\* della §0\.5' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
grep -n "La riga 3 resta l'unica decisione della §0.5 senza ADR" docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

Atteso: sette file, ciascuno con la sezione **una** volta; la §2.4 dichiara di essere la decisione
n. 3 e la §0.7 dichiara che la riga 3 vive lì senza ADR. Se un file avesse la sezione **zero**
volte, la condizione è ❌ — non si scrive l'ADR: si registra.

- [ ] **Passo 8: la §7 del disegno, in bozza**

In coda a `docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md`, dopo la
sezione *«Il prossimo passo»*, con le sottosezioni della **D2** — e **solo** quelle che questo
compito riempie:

```markdown
---

## 7. Il verbale della chiusura — in bozza dal compito 1

⛔ **Sta qui e non nel compendio** — decisione **B** del disegno del Traguardo 6, per la stessa
ragione: il compendio ha un tetto nel cancello. Scritto eseguendo il piano della chiusura, che
è un audit prima di essere una scrittura. ⚠️ **In bozza finché il compito 4 non lo data:** le
righe 6 e 7 della §7.2, la §7.4, la §7.5, la §7.6 e la §7.9 le scrivono i compiti che seguono.

### 7.1 I conteggi, rifatti col comando prima di leggere che cosa i documenti ne dicono

| Blocco della §1.3 | Il 2026-09-02, dal disegno | Oggi, rilanciato | Diverge? |
|---|---|---|---|
| A | … | … | … |

### 7.2 Le cinque condizioni della §0.7, rilette contro il codice

| # | Condizione | Esito | Evidenza letta o rilanciata oggi |
|---|---|---|---|
| 1 | … | ✅ / ❌ | … |

(le tabelle dei Passi 2, 4 e 5, ciascuna sotto la riga della propria condizione)

### 7.3 Dove la condizione era scritta troppo larga

| Condizione | La lettera | La decisione che la restringe | Che cosa ne segue |
|---|---|---|---|

### 7.7 Dove il disegno è stato smentito dall'esecuzione, e dove ha retto

| | Il disegno diceva | L'esecuzione ha trovato |
|---|---|---|

### 7.8 Le voci aperte della chiusura, con chi le chiude

| # | Voce | Dove è dichiarata | Chi la chiude |
|---|---|---|---|
```

Regole di scrittura: nessuna cifra senza comando e data; le **conferme** in 7.7 valgono quanto le
smentite — un verbale di sole smentite fa sembrare il disegno peggiore di com'era; la §7.8 parte
dalle **cinque** voci della tabella *«Le voci che questo disegno apre»* e vi aggiunge ciò che
questo compito ha aperto. ⛔ **Nessuna voce si decide al posto del proprietario.**

- [ ] **Passo 9: fine-riga, cancello, commit, push**

```bash
tr -cd '\r' < docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md | wc -c
bash scripts/check-docs.sh
bash scripts/gate.sh
git status --porcelain
```

Atteso: `0` — il disegno è LF — `OK`, `GATE GREEN`, e `git status` che nomina **solo** il disegno
e questo piano. La tabella della posizione di questo piano passa il compito 1 a ✅ con la data.

```bash
git add docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md docs/superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md
git commit -m "chiusura(compito 1): le condizioni 1-5 della §0.7 rilette contro il codice — il verbale in bozza nella §7 del disegno, i quattro script visti rossi e tornati verdi"
git push
```

⛔ Il messaggio dice *«rilette»*, mai *«chiuso»* (vincolo globale 11).

#### Criterio di chiusura del compito 1

- [ ] ogni conteggio dei blocchi A, F, G, H è stato **rifatto col comando** e sta in §7.1 con la data
- [ ] la §7.2 ha **una riga per condizione da 1 a 5**, con l'evidenza letta o rilanciata oggi
- [ ] ⛔ per i **quattro** script il rosso è stato **letto** — messaggio ed exit — e la revoca provata con `cmp`, con `git status --porcelain` vuoto dopo
- [ ] la tabella della condizione 1 copre **tutte** le righe che il comando del Passo 2 rende, e quella della 3 tutte quelle del Passo 5
- [ ] la §7.3 porta la condizione 4; la §7.8 porta almeno le cinque voci del disegno
- [ ] la spec, `porta-di-qualita.md` e `riferimenti.md` **non** sono toccati: `git diff --name-only` non li nomina
- [ ] `check-docs.sh` verde, `GATE GREEN`, commit pushato, posizione del piano aggiornata

---

## Compito 2: `tracciabilita.md` riletta con la seconda domanda

**Files:**
- Modify: `docs/tracciabilita.md` — le righe il cui meccanismo è scaglionato, il riquadro in testa, eventualmente la sezione «Lacune»
- Modify: il disegno — la **§7.4** inserita fra 7.3 e 7.7, righe nuove in **7.1**, **7.7** e **7.8**
- Modify: questo piano — posizione ed errata
- Read: la spec §0.4 (con §0.4.1, §0.4.2, §0.4.3), §8.3, §8.4 · `roadmap.md`, tabella dei sotto-progetti · `ls crates/kernel/src/`

⛔ **La domanda, per ogni riga:** *«quale meccanismo del kernel le serve, e il codice lo dà oggi
o è scaglionato?»*. Nessuna risposta a memoria: le tre fonti della §3.1 del disegno, in
quest'ordine, e la tabella degli esiti della §3.2.

- [ ] **Passo 1: i numeri, prima delle frasi**

```bash
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
grep -cE '^\| .* \| (✅|🔶|📋|⚠️|❌) \|' docs/tracciabilita.md
ls crates/kernel/src/
tr -cd '\r' < docs/tracciabilita.md | wc -c
```

Il 2026-09-02: `71 · 25 · 74 · 0 · 1`, totale `171`, zero CR. Si scrivono in §7.1 con la data di
oggi.

- [ ] **Passo 2: la mappa dei meccanismi, da §0.4**

Si estrae la tabella di §0.4 — le colonne **Entra** e **Si scaglia**, righe da §1 a §10 — più le
due sotto-tabelle di §0.4.2 (la configurazione) e §0.4.3 (l'anello 3):

```bash
sed -n '/^### 0\.4 /,/^### 0\.5 /p' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | grep -E '^\|'
```

Se ne fa una tabella di lavoro nello scratchpad: per ogni §N del kernel, che cosa **entra** e che
cosa **si scaglia**, parola per parola. ⚠️ **Il criterio di «esiste in `crates/`»** per un
meccanismo che entra: `ls crates/kernel/src/` dà i moduli, e un meccanismo costruito ne nomina
uno — `arbiter`, `gateway`, `sensor`, `permission`, `degradation`, `record`, `reconcile`,
`boundary`, `ports`, `wire`, `client`, `executor`, … si legge la lista di oggi, non questa.

- [ ] **Passo 3: le settantuno righe ✅, una per una**

Per ogni riga ✅ — `grep -nE '^\| .* \| ✅ \|' docs/tracciabilita.md` — la colonna **Sede**
nomina un `§N`, un ADR, un V o un Q. Si risponde alla domanda e si sceglie la riga della §3.2 del
disegno:

| Se il meccanismo | Allora |
|---|---|
| è in **Entra** della sua §N, e un modulo di `crates/kernel/src/` lo porta | la riga **resta** ✅, e va in un **gruppo** del verbale (Passo 5) |
| è in **Si scaglia** | la riga diventa 🔶 nella forma della **D7**, col sotto-progetto preso dall'innesco del V o Q che lo governa (**D8**) |
| è in Si scaglia e nessuna delle tre fonti dà un sotto-progetto | 🔶 con `implementazione → sede da assegnare`, e una riga in §7.8 — **del proprietario**. ⚠️ Il caso della **ritenzione a livelli** (ADR-0018, `V26`) è già misurato: P-8 |
| nessuno l'ha deciso | ⚠️, e una riga nuova nella sezione «Lacune» con la forma di L-1…L-5 — `L-6`, e così via |

⛔ **Le righe senza `§N` nella Sede** — `ADR-0028 · Python`, `ADR-0003 · ADR-0015`, `Q8`, `V9`,
`ADR-0004 · implementazione → L3`, e le altre che `grep -E '^\| .* \| ✅ \|' docs/tracciabilita.md
| grep -vE '\| ✅ \| §[0-9]+'` rende — si giudicano dall'ADR o dalla riga V/Q che nominano: un
ADR `Accepted` che decide un meccanismo di kernel è *deciso*, e la domanda resta se il **codice**
lo dà. ⚠️ `Avvio automatico e daemon in background | ✅ | ADR-0004 · implementazione → L3` porta
già la parola *implementazione*: se resti ✅ o passi a 🔶 lo dice la lettura, e la scelta si
scrive in §7.4.

Ogni riga che cambia si riscrive **in sede**, con Python su un file LF o con l'editor: il file è
LF, e il Passo 6 lo rimisura.

- [ ] **Passo 4: le righe già 🔶, e le altre**

Le venticinque 🔶 si rileggono: cambia **solo** se la sede è sbagliata — cioè se il sotto-progetto
scritto non è quello che l'innesco di §8 dà. Le 📋 e la ❌ **non si toccano**: non dipendono dal
sotto-progetto 1.

- [ ] **Passo 5: il riquadro in testa, e la §7.4 del verbale**

Il riquadro *«La tabella si aggiorna **alla chiusura del sotto-progetto**, non ora: questo
riquadro è il segnaposto che impedisce di leggerla come se fosse già vera»* — l'ultimo capoverso
del secondo riquadro citato — diventa lo stato della rilettura:

```markdown
> ✅ **Riletta il <data> con la seconda domanda** — *«quale meccanismo del kernel le serve, e il
> codice lo dà oggi o è scaglionato?»* — contro la §0.4 della spec e gli inneschi della §8. Le
> righe cambiate, con la ragione di ciascuna, stanno nella §7.4 del
> [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md): una
> casa sola. ⚠️ **Le righe per stato non stanno qui**, le conta il comando:
> `for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done`
```

⚠️ **La data della chiusura NON si scrive qui:** la scrive il compito 4 (**D4**). La legenda **non
cambia**.

La **§7.4** del disegno, inserita fra la 7.3 e la 7.7:

```markdown
### 7.4 La rilettura di `tracciabilita.md` — le righe cambiate, e i gruppi che restano

| Funzionalità | Da → a | Meccanismo | Fonte della sede |
|---|---|---|---|
| … | ✅ → 🔶 | … §N, in «Si scaglia» | innesco `V<n>` — **A (2)** — → GUI minima |

**Le righe rimaste ✅, raggruppate per meccanismo** — circa dieci gruppi, ciascuno col modulo che
esiste:

| Meccanismo | Modulo in `crates/kernel/src/` | Righe |
|---|---|---|
| … | `arbiter/` | … |
```

Settantuno righe, non settantuno frasi: le ✅ si nominano dentro il gruppo, non una per una.

- [ ] **Passo 6: la prova, in due direzioni**

```bash
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
grep -cE '^\| .* \| (✅|🔶|📋|⚠️|❌) \|' docs/tracciabilita.md
tr -cd '\r' < docs/tracciabilita.md | wc -c
git diff --stat
```

Il totale resta `171`; la somma delle ✅ e delle 🔶 resta quella di prima, salvo le righe passate
a ⚠️; zero CR; il diff nomina **solo** `docs/tracciabilita.md`. Poi le due direzioni della §3.4
del disegno, scritte in §7.4: per ogni riga passata a 🔶, il `grep` del suo meccanismo in «Si
scaglia» di §0.4 o nell'innesco di §8; per ogni gruppo di ✅, il modulo che `ls` mostra.

- [ ] **Passo 7: commit del prodotto, poi commit del verbale (D3)**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh
git add docs/tracciabilita.md
git commit -m "chiusura(compito 2): tracciabilita.md riletta con la seconda domanda — le righe il cui meccanismo e' scaglionato passano a 🔶 col sotto-progetto che lo porta, le altre restano sulle fondamenta che esistono"
git add docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md docs/superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md
git commit -m "chiusura(compito 2, verbale): la §7.4 del disegno — le righe cambiate con la fonte della sede, i gruppi delle ✅ col modulo che esiste"
git push
```

Il messaggio dice ciò che è vero **a prescindere** dalla chiusura (disegno §5.4).

#### Criterio di chiusura del compito 2

- [ ] ogni riga ✅ è stata giudicata contro **Entra / Si scaglia** di §0.4, e le rimaste ✅ stanno in un gruppo che nomina un modulo esistente
- [ ] ogni riga 🔶 nomina un sotto-progetto preso dall'innesco di §8, o `sede da assegnare` con una riga in §7.8
- [ ] il riquadro in testa porta la data della rilettura, la domanda e il comando; la legenda è intatta; la data della chiusura **non** c'è (D4)
- [ ] il totale delle righe con stato è invariato, e il revisore ha **ri-derivato da solo** l'insieme delle righe cambiate contro §0.4 prima di leggere §7.4
- [ ] `git diff --stat` del primo commit nomina **un** file; zero CR; `check-docs.sh` verde; `GATE GREEN`; entrambi i commit pushati

---

## Compito 3: il racconto del Traguardo 6 esce dalla §6, e il tetto scende

**Files:**
- Modify: `docs/COMPENDIO.md` — **CRLF**: il blocco esce, il puntatore entra
- Modify: `docs/archivio/stato-storico.md` — LF: la sezione nuova, in coda
- Modify: `scripts/check-docs.sh` — **CRLF**: `ceiling=` e il richiamo, **in inglese**
- Modify: il disegno — la **§7.5** inserita fra 7.4 e 7.6, righe in **7.1**, **7.7**, **7.8**
- Modify: questo piano — posizione ed errata

⛔ **Parola per parola.** Nulla si riassume, nulla si cancella: ciò che esce dalla §6 entra
nell'archivio identico, e la cernita decide **solo** che cosa resta.

- [ ] **Passo 1: le misure di partenza**

```bash
sed -n '/^### Il prossimo passo/,/^✅ \*\*I finding dell/p' docs/COMPENDIO.md | head -n -1 | wc -c
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/COMPENDIO.md docs/archivio/stato-storico.md scripts/check-docs.sh; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
git ls-files --eol docs/COMPENDIO.md docs/archivio/stato-storico.md scripts/check-docs.sh
```

Il 2026-09-02: il blocco pesa quanto il blocco **D** del disegno dice quel giorno; il compendio ha
**un CR per riga** (CR = righe), l'archivio **zero**, lo script un CR per riga. Si scrivono in
§7.1 con la data di oggi.

- [ ] **Passo 2: il blocco, estratto nello scratchpad, e ciò che contiene**

```bash
sed -n '/^### Il prossimo passo/,/^✅ \*\*I finding dell/p' docs/COMPENDIO.md | head -n -1 | tail -n +2 > "$S/blocco.md"
grep -o '](\([^)#]*\.md\)[^)]*)' "$S/blocco.md" | sort -u
grep -oE 'E[0-9]+' "$S/blocco.md" | sort -u | wc -l
grep -n '^⏭️' "$S/blocco.md"
```

Il `tail -n +2` toglie l'intestazione, che **resta** in §6 (**D1**). Il primo `grep` rende i
bersagli relativi da prefissare — **tutti**, quanti siano lo dice il comando (P-1); il secondo
i codici d'errata che il blocco nomina; il terzo la riga del marcatore, che resta in §6 (§4.1
del disegno, prima eccezione — P-5).

- [ ] **Passo 3: la cernita, in due direzioni**

**Direzione 1** — ogni voce che il blocco chiama aperta ha un'altra casa:

```bash
grep -nE 'apert|non pres|del proprietario|registrat|E[0-9]+' "$S/blocco.md" | cut -c1-160
```

Ogni riga resa si legge **intera** (gotcha **#70**), e per ogni voce aperta che nomina si cerca
la casa: le due tabelle uniche di [`porta-di-qualita.md`](../../porta-di-qualita.md) — *«LE VOCI
APERTE DEL TRAGUARDO 5»* e *«… DEL TRAGUARDO 6»* — e la tabella delle voci senza numero AUD di
[`audit-2026-08-27.md`](../../audit-2026-08-27.md); per un codice `E<n>`, l'errata del piano del
Traguardo 6, **per voce e non per codice** (un `E<n>` è unico dentro **un** piano). Trovata: la
riga parte. Non trovata: la riga **resta in §6**, e la voce entra in §7.8.

**Direzione 2** — ciò che parte è ripassato con marcatori **diversi**:

```bash
grep -nE 'decision|da decidere|chiusore|sbarra|aspetta|voce' "$S/blocco.md" | cut -c1-160
```

Tutto ciò che pesca si rilegge intero; su un dubbio **si conserva**. E la terza eccezione: una
riga che sia una **regola per i piani futuri** senza altra casa resta — si elencano in §7.5,
con la ragione. Il precedente è `E162`, che ha già due case (il piano del Traguardo 6 e il
commento di `scripts/gate.sh`) e quindi **parte**.

- [ ] **Passo 4: la sezione nuova nell'archivio, LF, coi link prefissati**

In coda a `docs/archivio/stato-storico.md`, modellata sulla sezione del 2026-09-01:

```markdown

## Il racconto del Traguardo 6 — archiviato il <data>

⚠️ **Verbale, non stato.** Questo blocco stava nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md),
dopo *«Il prossimo passo»*, e ne è uscito il **<data>**: è il racconto del Traguardo 6, chiuso il
2026-09-02, compito per compito. È qui **parola per parola** — nulla è stato riassunto, nulla
cancellato.

⛔ **Perché è uscito:** il Traguardo 6 è chiuso, e il tetto di `scripts/check-docs.sh` lascia al
compendio un margine che il passaggio unico della chiusura consumerebbe. Il criterio di taglio è
quello del disegno del 2026-08-28: **resta in §6 ciò che è vero adesso**.

⚠️ **Tre eccezioni restano in §6, dichiarate qui:** la riga del marcatore del prossimo passo, che è
il puntatore vivo; il blocco *«i finding dell'audit sono chiusi»*, col suo comando; e le righe
che sono una regola per i piani futuri senza altra casa — quali, lo dice la §7.5 del
[disegno della chiusura](../superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md).

<il blocco, verbatim, con ogni link relativo prefissato di `../`>
```

I link si prefissano con Python, **sul blocco estratto**, con la stessa espressione che
`check-docs.sh` usa per estrarli — così ogni bersaglio che il cancello controllerà è uno che il
prefisso ha toccato:

```python
import io, re
p = "<scratchpad>/blocco.md"
s = io.open(p, encoding="utf-8", newline="").read()
s = re.sub(r"\]\((?!https?://)(?!#)", "](../", s)
io.open(p + ".pref", "w", encoding="utf-8", newline="").write(s)
```

Poi si appende `blocco.md.pref` sotto l'intestazione, in **LF**: `tr -cd '\r' < "$S/blocco.md.pref"
| wc -c` deve dare `0` prima di appendere — il blocco viene da un file CRLF, quindi **si
convertono i CR** con `tr -d '\r'` prima, e si rimisura.

- [ ] **Passo 5: la §6 riscritta — il blocco esce, il puntatore entra — con Python su CRLF**

Si costruisce **tutto** il contenuto nuovo prima di scrivere (gotcha **#82**): le righe che
restano (**D1**, le eccezioni del Passo 3) più il puntatore, modellato su quello del 2026-09-01:

```markdown
⛔ **IL RACCONTO DEL TRAGUARDO 6 È USCITO DA QUI il <data>, e non riassunto:** sta in
[`archivio/stato-storico.md`](archivio/stato-storico.md), parola per parola, con le tre eccezioni
dichiarate lì. ⚠️ **La cernita è stata provata in due direzioni**, coi comandi che stanno nella
§7.5 del [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md):
ogni voce che il blocco chiamava aperta ha un'altra casa, e ciò che è partito è stato ripassato
con marcatori diversi. 📌 **Perché adesso:** il Traguardo 6 è chiuso, e il tetto scende nello
stesso commit — un tetto che resta alto dopo uno sfoltimento è il permesso di ricrescere.
```

La scrittura: leggere con `newline=''`, sostituire fra i due delimitatori, scrivere in un
temporaneo, `os.replace`. Il **CR dopo deve fare il numero di righe**:

```bash
printf 'CR='; tr -cd '\r' < docs/COMPENDIO.md | wc -c; printf 'righe='; wc -l < docs/COMPENDIO.md
```

- [ ] **Passo 6: il tetto scende, nello stesso commit**

```bash
size=$(wc -c < docs/COMPENDIO.md); echo $(( (size * 111 / 100 + 1023) / 1024 * 1024 ))
```

Il numero va in `ceiling=` di `scripts/check-docs.sh`, e sotto il richiamo del 2026-09-01 entra un
richiamo nuovo, **in inglese** (vincolo globale 5), nella forma di quello che lo precede:

```text
# ⛔ RECALL OF <date> -- THE CEILING CAME DOWN A SECOND TIME, by the same rule. Closing
# sub-project 1, the story of milestone 6 left §6 word for word for docs/archivio/stato-storico.md
# and the compendium measured <size> bytes: measured + ~11%, aligned to a whole KiB, gives the
# number below. The 11% is what the closure's single pass over the status documents has to fit in.
```

Scritto con lo stesso metodo del compendio — lo script è CRLF — e rimisurato: CR = righe.

- [ ] **Passo 7: le prove, poi il commit del prodotto**

```bash
bash scripts/check-docs.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
git diff --stat
git ls-files --eol docs/COMPENDIO.md docs/archivio/stato-storico.md scripts/check-docs.sh
for f in docs/COMPENDIO.md docs/archivio/stato-storico.md scripts/check-docs.sh; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
bash scripts/gate.sh
```

Atteso: `OK` — quindi link dell'archivio e tetto — un margine **positivo** e di circa l'undici
per cento; `git diff --stat` che nomina **esattamente tre** file; `ls-files --eol` invariato; CR
come al Passo 1; `GATE GREEN`.

```bash
git add docs/COMPENDIO.md docs/archivio/stato-storico.md scripts/check-docs.sh
git commit -m "chiusura(compito 3): il racconto del Traguardo 6 esce dalla §6 del compendio parola per parola, in archivio — archiviato perche' il Traguardo 6 e' chiuso, e il tetto scende nello stesso commit"
```

⛔ Mai *«sotto-progetto chiuso»* (disegno §5.4).

- [ ] **Passo 8: la §7.5 del verbale, e il secondo commit (D3)**

Inserita fra 7.4 e 7.7:

```markdown
### 7.5 L'archiviazione del racconto del Traguardo 6, e il tetto

| | Misurato il <data> | Comando |
|---|---|---|
| byte usciti dalla §6 | … | blocco **D** del disegno, prima |
| tetto prima → dopo | … → … | `grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh` |
| margine dopo | … | blocco **E** |
| link prefissati | … | il `grep` del Passo 2 |

**Le righe rimaste in §6 oltre al puntatore, e perché** — una tabella: la riga, la ragione
(marcatore · regola senza altra casa · voce senza casa). **La cernita:** i due comandi, quante
righe hanno reso e quante voci hanno trovato casa dove.
```

```bash
git add docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md docs/superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md
git commit -m "chiusura(compito 3, verbale): la §7.5 del disegno — la cernita in due direzioni, le eccezioni, il tetto prima e dopo"
git push
```

#### Criterio di chiusura del compito 3

- [ ] il blocco sta nell'archivio **parola per parola**, con l'intestazione modellata su quella del 2026-09-01 e **ogni** link relativo prefissato
- [ ] in §6 restano **solo** l'intestazione (D1), la riga del marcatore, il blocco dell'`awk` dei finding, le regole senza altra casa dichiarate in §7.5, e il puntatore nuovo
- [ ] la cernita è provata in **due** direzioni, coi comandi in §7.5; ogni voce senza casa sta in §7.8
- [ ] il tetto è sceso **nello stesso commit** del blocco, col richiamo datato in inglese, e il margine dopo è positivo
- [ ] il primo commit nomina **tre** file; CR = righe sul compendio e sullo script, zero sull'archivio; `ls-files --eol` invariato
- [ ] `check-docs.sh` verde, `GATE GREEN`, entrambi i commit pushati, posizione del piano aggiornata

---

## Compito 3bis: la voce 9 di §7.8 — la §8 smette di promettere controlli che non esistono

⛔ **Si legge E10 PRIMA di questo compito.** È la decisione del proprietario che lo apre, e porta
le **tre eccezioni misurate**: `V5`, `V8` e `Q13` **non** si declassano, e `V36` non si tocca
affatto. Chi esegue non ri-giudica quelle quattro: sono decise.

**Files:**
- Create: `crates/kernel/tests/compile_fail/effect_without_its_class.rs` e il suo `.stderr`
- Modify, commit **1** — il prodotto: `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` (⛔ **CRLF**) · `docs/porta-di-qualita.md` (⛔ **CRLF**)
- Modify, commit **2** — il verbale: il disegno (LF) · questo piano (LF)
- Read: **§8.1**, **§8.1.2**, **§8.1.3**, **§8.2**, **§8.2.1** della spec — il vocabolario degli stati e come si scrive un innesco; la **§7.2 del disegno**, che porta l'evidenza riga per riga e i comandi che l'hanno prodotta

⛔ **I DUE FILE DEL COMMIT 1 SONO CRLF NELL'ALBERO, E IL VINCOLO GLOBALE 6 NON LI ELENCA** —
misurato il 2026-09-03, perché nessun compito prima di questo li toccava: la spec e il registro
sono entrambi `i/lf w/crlf`, e `tr -cd '\r' < <file> \| wc -c` dà il valore di partenza da
rimisurare dopo. Si scrivono **solo** con Python `io.open(..., encoding="utf-8", newline="")`,
contenuto costruito e codificato **per intero prima**, file temporaneo, `os.replace` — gotcha
**#82**, e il vicolo cieco del 2026-08-27 che svuotò la spec a zero byte. `git ls-files --eol`
prima e dopo, invariato. ⛔ Un `sed -i` no, e `git checkout --` su un file che si sta scrivendo
nemmeno.

⛔ **§8.3 e §8.4 SONO LETTE PER POSIZIONE da `check-docs.sh`:** cinque colonne, lo **stato** in
terza e l'**innesco** in quinta, e i delimitatori sono intestazioni. Una riga che perde una
colonna è un rosso, non un ritocco (trappola **3**).

- [ ] **Passo 1: le misure prima**

```bash
bash scripts/gate.sh; bash scripts/check-docs.sh
git ls-files --eol docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md docs/porta-di-qualita.md
for f in docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md docs/porta-di-qualita.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; done
ls crates/kernel/tests/compile_fail/*.rs | wc -l
grep -c '' crates/kernel/tests/compile_fail/record_without_trust_label.stderr
```

L'ultimo conteggio è il numero di casi **prima**: dopo il Passo 2 deve essere uno in più, e
`tests/compile_fail.rs` li raccoglie con un **glob** — nessun elenco da aggiornare.

- [ ] **Passo 2: il caso `compile_fail` che la riga di catalogo `V5` promette dal 2026-08-10**

La riga del catalogo è, alla lettera:
`| **V5** | un effetto **senza classe dichiarata** — §7.4.4 | un effetto con la classe compila |`
— l'ultima colonna è la **contro-sonda**, e il caso deve nominarla.

⛔ **Il modello è il GEMELLO**, `crates/kernel/tests/compile_fail/record_without_trust_label.rs`:
tiene lo stesso contratto per `trust`, sulla **stessa** funzione, con la **stessa** forma `E0061`.
Si legge per intero prima di scrivere — porta già l'argomento sul perché la mutazione che lo
disarma è la **firma** e non il campo, e il proprio richiamo datato di AUD-050.

| | |
|---|---|
| **Che cosa non deve compilare** | `RecordV1::intent(effect, trust, payload, reason)` chiamata **senza il primo argomento**: tre argomenti su quattro, e a mancare è la classe |
| **L'errore atteso** | `error[E0061]`, con la nota `argument #1 of type `EffectClass` is missing` — ⛔ **si LEGGE l'uscita vera e la si copia**, non si indovina; `TRYBUILD=overwrite` esiste ma il `.stderr` **si legge nel diff** (gotcha **#25**) |
| **La contro-sonda, che esiste già** | `every_effect_class_survives_the_round_trip_and_the_three_differ_in_the_bytes` in `crates/kernel/tests/record_shape.rs` — il caso la **nomina**, come il gemello nomina la propria |
| **La seconda metà del vincolo** | *«l'assenza vale `irripetibile`»* è **già tenuta e non va scritta**: `crates/kernel/src/reconcile.rs` manda un record che questa build non decodifica in `Resolution::SuspendAndAsk`, con le sonde in `crates/kernel/tests/reconciliation.rs`. Il caso lo **dichiara** invece di ripeterlo |
| **Attributi** | ⛔ nomina `kernel::` e **non ridichiara attributi propri** — gotcha **#39**, ed è ciò che `gate-attributes.sh` legge |
| **Il commento va SOTTO `fn main`** | l'oracolo cita un numero di riga: un capoverso in testa sposta il codice e rende il caso `mismatch`. Il gemello lo scrive a chiare lettere |

⛔ **Le due direzioni, provate e scritte nel rapporto:** *deve scattare* — il caso esce `ok` da
`cargo test --locked -p kernel --test compile_fail`, cioè **non compila** con l'errore atteso;
*non deve scattare* — la contro-sonda nominata sopra è verde, e un `intent` **con** la classe
compila. ⚠️ **Se esce `mismatch` invece di `ok`**, l'atteso e il vero divergono: si legge l'uscita
e si corregge il `.stderr`, **non** si rigenera in blocco.

- [ ] **Passo 3: le sei righe della §8 — stato, innesco, e che cosa il richiamo deve dire**

⛔ **La forma del richiamo è quella di `V34` e di `Q24`**, scritte il 2026-08-27 chiudendo
AUD-026: si legge una delle due nella spec prima di scriverne una nuova. Ogni richiamo porta la
data **2026-09-03** e nomina la propria origine — *voce 9 di §7.8 del disegno della chiusura* —
e i comandi che lo sostengono stanno nella **§7.2 del disegno**, da cui si copiano invece di
riscriverli. ⚠️ **Un capoverso che è un VERBALE datato non si riscrive** (vincolo globale 9): il
richiamo nuovo gli si affianca, come fa `Q17`.

| Riga | Stato: da → a | Innesco | Che cosa il richiamo deve dire |
|---|---|---|---|
| `V5` | ✅ → **✅**, invariato | `—` | la cella prometteva un *«test di compilazione fallita»* che **non esisteva**, e il registro lo dichiarava dal 2026-08-10. ✅ **Il caso è stato scritto** invece di declassare la riga, perché nessuno dei due inneschi di §8.2 calzava. La cella nuova nomina il caso del Passo 2, la contro-sonda, e la seconda metà tenuta da `reconcile.rs` |
| `V8` | ✅ → **✅**, invariato | `—` | la cella accreditava *«i confini di autonomia entrano (§0.4, §4)»*, che è una **voce di sezione** e non una delle tre risposte di §8.1.2 — forma già corretta su `V35` il 2026-08-08 — e *«la transizione ad `AttesaUmano`»*, che **non esiste**. ⛔ Quel test è il metodo di **`Q7`**, non di V8: la riga si attribuiva un merito altrui, come `Q17` prima del 2026-08-08. **Resta ✅ perché per un V l'autorità è il TESTO del vincolo** (§8.1.3), e il tetto esiste, è **consegnato** ed è provato |
| `V9` | ⚠️ → **⏳ rimandato** | `A (2)`, invariato | la metà dichiarata verificata — *«l'evento emesso e giornalato»* — **non ha soggetto**: nessun tipo evento, e l'executor non scrive sul giornale in nessun percorso. Senza la metà verificata non resta niente qui, e §8.1 dice `rimandato` — la stessa lettura che declassò **V16** (§8.5.3.1) e **V34** (AUD-026) |
| `V24` | ⚠️ → **⚠️ parziale**, invariato | `A (2)`, invariato | ⛔ **la cella dichiarava verificate DUE cose e una sola lo è.** I **permessi** rileggono davvero il giornale — `is_granted` chiama `journal.replay()`, provato da `nothing_is_granted_on_an_empty_journal`. Il **picco di VRAM** no: le chiamate a `journal.replay()` del kernel stanno in `degradation.rs`, `permission.rs` e `reconcile.rs`, e nessuna legge il picco. Il picco **esce** dalla metà verificata; la proiezione trace resta fuori come già era |
| `V25` | ⚠️ → **⏳ rimandato** | `B (3)`, invariato | la metà dichiarata verificata — *«il controllo gira a ogni commit e la sonda scatta, una chiamata di rete in `daemon` lo accende»* — è **falsa**: nessuno script guarda `daemon`. ⚠️ **L'altra metà RESTA vera e non si riscrive**: la contro-sonda non esiste perché la lista è vuota (§7.4.2). È la stessa lacuna che declassò `V34` |
| `V35` | ⚠️ → **⏳ rimandato** | `D (5)`, invariato | poggiava sui *«test a esempi, gli stessi che rendono ✅ `V37`»*, e il soggetto di `V37` **non esiste**. ⚠️ Il capoverso del 2026-08-08 sul rinomino del meccanismo è un **verbale**: non si riscrive |
| `V37` | ✅ → **⏳ rimandato** | **`D (5)`**, nuovo | il **tipo del livello di confinamento non esiste**: la ricerca rende tre righe di **commento** e nessun tipo, e una di quelle righe porta già il proprio comando con la propria risposta. `Permission` e `PermissionDetail` portano altri campi: nessun livello di confinamento. ⛔ **L'innesco è nuovo perché la riga era ✅** e §8.6 lo pretende ora: la condizione è `D` — si esegue codice o un comando (**5**) — la stessa di `V35` e `Q23`, che poggiano sullo stesso soggetto |
| `Q23` | ⚠️ → **⏳ rimandato** | `D (5)`, invariato | ⛔ **la cella dichiarava già che nessuna delle due tecniche gira qui**; ciò che vi entrava erano i test a esempi di `V37`, che **non esistono**. Quindi non resta niente di verificato, e lo stato segue. ⚠️ Il capoverso del 2026-08-08 è un **verbale**: non si riscrive |

⛔ **`V36` e `Q13` NON si toccano**, ed è una decisione e non una dimenticanza (E10): il ❌ di
`V36` nasceva dalla riga di catalogo `V5` senza caso e sparisce col Passo 2; `Q13` è trattata al
Passo 6, e la sua cura non sta nella §8.

- [ ] **Passo 4: il registro — il conteggio del blocco C, che il Passo 2 ha reso stantio**

⛔ **È l'unica cosa che si tocca in [`porta-di-qualita.md`](../../porta-di-qualita.md)**, ed è
dovuta al passo **5** della disciplina dell'audit del 2026-08-27 — *un rimedio riconta i conteggi
che ha reso stantii*. Il file dichiara oggi il blocco C a **diciotto righe su diciannove**, con
*«scoperta la sola `V5`»* e il capoverso che spiega perché — *«un tipo che esiste non è un
controllo che scatta»*. Col caso scritto, `V5` è **coperta**.

⚠️ **Si ricontà sul catalogo §7.4.1 blocco C, non da questa frase**, com'è scritto nella cella
stessa; e la cella porta già **otto** ricontate datate: la nona si **affianca**, con la propria
data, e non riscrive le precedenti. ⛔ **Il capoverso `⚠️ E `V5` resta scoperta anche adesso…`
è la riga che diventa FALSA**: riceve il proprio richiamo datato.

- [ ] **Passo 5: il commit del prodotto**

```bash
tr -cd '\r' < docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | wc -c
tr -cd '\r' < docs/porta-di-qualita.md | wc -c
git ls-files --eol docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md docs/porta-di-qualita.md
git diff --stat -- docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
bash scripts/gate.sh && bash scripts/check-docs.sh
```

⛔ **Il diff sulla spec deve nominare SOLO righe di §8.3 e §8.4.** Se ne nomina altre, ci si
ferma: il vincolo globale 1 è sospeso per quelle due tabelle e per nient'altro (E10). Il commit
**non** dice «chiuso» (vincolo globale 11).

- [ ] **Passo 6: `Q13` e la condizione 3 — la restrizione va nella §7.3 del disegno**

⛔ **Non si tocca la §8, e non si tocca la §0.7.** La §7.3 del disegno ha **una** riga, per la
**condizione 4**, e la sua forma è: la **lettera** della condizione · la **decisione** che la
restringe · **che cosa ne segue**. Se ne aggiunge una **seconda**, per la **condizione 3**, con
la stessa forma:

| | |
|---|---|
| **la lettera** | la §0.7 dice *«ogni Q in perimetro è verificato col metodo che `design/08` gli assegna, **non con un altro**»* |
| **la decisione che la restringe** | la cella di `Q13` in §8.4 **dichiara la sostituzione dal 2026-08-08**, e la argomenta: il metodo assegnato è una **verifica di proprietà** su catene generate; qui la proprietà è resa **non esprimibile** da un gettone di livello 1, e *«una proprietà provata su N catene lascia scoperta la N+1, un tipo no»*. È la stessa mossa di §5.3 punto 3, e §8.1.3 pretende che si **dica** — cosa che quella cella fa |
| **che cosa ne segue** | la condizione è soddisfatta **in quella lettura, e in quella soltanto**: presa alla lettera rifiuterebbe una sostituzione **più forte**, dichiarata, e già registrata da tre traguardi. ⛔ **La §0.7 non si tocca**: è spec, ed è del proprietario. ✅ **Deciso dal proprietario il 2026-09-03**, voce 9 di §7.8 |

- [ ] **Passo 7: la §7.2, la voce 9, e la voce nuova**

| Dove | Che cosa |
|---|---|
| §7.2 del disegno, riga **1** della tabella delle condizioni | l'esito passa da ❌ a **✅**, e la cella dice che le otto righe scoperte sono state **risolte**: `V5` col caso, `V8` con la cella corretta, `V9` `V25` `V35` `V37` declassate, `V24` con la metà verificata ristretta. ⚠️ **Il conteggio si rifà col comando**, non si riscrive a memoria |
| §7.2 del disegno, riga **3** | l'esito passa da ❌ a **✅**: `Q23` declassata, `Q13` risolta dalla §7.3 |
| §7.2 del disegno, le celle di `V5` `V8` `V9` `V24` `V25` `V35` `V37` `Q13` `Q23` nelle due tabelle riga-per-riga | ⛔ **NON si riscrivono: sono un VERBALE datato 2026-09-03** (vincolo globale 9). Ricevono **una** riga ciascuna che nomina come sono state chiuse |
| §7.8, **voce 9** | passa a **chiusa**, con la data, la decisione del proprietario e il rimando a **E10** — che è la casa unica delle tre eccezioni. ⛔ Le tre eccezioni **non si ricopiano qui** |
| §7.8, **voce nuova** | ⚠️ **REGISTRATA E NON PRESA, ed è del proprietario:** `Q7`, `Q8` e `Q20` di §8.4 restano ✅ sulla **tecnica**, ma le loro celle accreditano soggetti che `V8`, `V9` e `V25` **smettono** di accreditare in questo stesso commit — il test sulla transizione ad `AttesaUmano`, l'evento emesso, e la metà statica che poggia su `V25`. ⛔ **Cade fra le due condizioni:** la 1 parla dei soli **V**, la 3 dei soli **metodi**, quindi nessuna delle due lo coglie. Se le celle di `Q7`, `Q8` e `Q20` debbano essere ri-giudicate è del proprietario |
| la cifra **«quattro compiti»**, quattro case **vive** (**E11**) | il numerale si **TOGLIE** e non si riallinea a cinque: la *«Forma»* in testa a questo piano · la giustificazione di **D2** · la riga di §12 che il **compito 4** detta per il compendio · la §5.4 del disegno. ⛔ **Le DUE case che sono verbali datati NON si toccano**, e il rimedio lo **dichiara** invece di tacerlo: la riga *«IL PIANO È SCRITTO IL 2026-09-02»* e la scelta 4 del disegno. ⚠️ **E la mappa di `D2` guadagna il 3bis**: non apre nessuna intestazione nuova, e scrive dentro 7.2, 7.3 e 7.8 |
| questo piano | la tabella della posizione passa il compito 3bis a ✅ con la data |

- [ ] **Passo 8: il commit del verbale, e le verifiche**

```bash
bash scripts/gate.sh; bash scripts/check-docs.sh
git diff --name-only <base>..HEAD -- docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
grep -c '' crates/kernel/tests/compile_fail/effect_without_its_class.stderr
cargo test --locked -p kernel --test compile_fail
```

- [ ] `GATE GREEN` e `check-docs.sh` verdi **prima di ciascuno** dei due commit
- [ ] le due direzioni del caso `V5` **misurate e scritte nel rapporto**, con l'uscita vera
- [ ] `git ls-files --eol` **invariato** su spec e registro, e i `CR` rimisurati
- [ ] il diff sulla spec nomina **solo** §8.3 e §8.4
- [ ] nessun commit dice «chiuso» del sotto-progetto (vincolo globale 11)
- [ ] entrambi i commit pushati, posizione del piano aggiornata

## Compito 4: il passaggio unico — l'unico che dice «chiuso»

**Files:**
- Modify, **in un commit solo**: `docs/roadmap.md` (LF) · `docs/README.md` (CRLF) · `docs/HANDOFF.md` (CRLF) · `docs/AVVIO-CHAT.md` (CRLF) · `docs/COMPENDIO.md` (CRLF) · `docs/tracciabilita.md` (LF, D4) · `docs/semi-dst.md` (CRLF) · il disegno (LF) · questo piano (LF)
- Read: la tabella della §5.2 del disegno, che è la casa unica di **che cosa** cambia file per file

⛔ **Ogni riga si ritrova col `grep` sulla frase, mai col numero di riga.** Le frasi di oggi sono
citate qui dal disegno, misurate il 2026-09-02; se una non si trova più com'è citata, ci si
ferma e si scrive l'errata — non si cerca «qualcosa di simile».

- [ ] **Passo 1: le misure prima, e il margine del compendio (P-9)**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
git ls-files --eol docs/roadmap.md docs/README.md docs/HANDOFF.md docs/AVVIO-CHAT.md docs/COMPENDIO.md docs/tracciabilita.md docs/semi-dst.md
for f in docs/README.md docs/HANDOFF.md docs/AVVIO-CHAT.md docs/COMPENDIO.md docs/semi-dst.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
grep -rn 'NON è chiuso\|è in corso' docs/*.md | cut -c1-120
```

L'ultimo comando rende le righe **vive** da riscrivere e i richiami datati che restano: si legge
ogni riga intera e si decide quale è quale — la decisione va in §7.6.

- [ ] **Passo 2: `roadmap.md`, LF**

| Trova | Sostituisci con |
|---|---|
| `Ultimo aggiornamento: **2026-08-25**, alla chiusura del **Traguardo 5** del sotto-progetto 1.` | `Ultimo aggiornamento: **<data>**, alla chiusura del **sotto-progetto 1** contro la §0.7 della sua spec.` |
| la cella di stato della riga `\| 1 \| Implementazione del kernel + simulatore DST \|` — da `🔵 **in corso**` fino a `in un posto solo` compreso | `✅ **chiuso il <data> contro la §0.7** della spec — il verbale è la §7 del [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md). Lo stato per traguardo sta nella tabella dei traguardi di questo file, e il puntatore nella §6 del compendio: una casa sola ciascuno. ⚠️ **RICHIAMO DEL <data>:** questa cella portava il racconto dei sei traguardi e una voce *«registrata e non presa, del proprietario»* sul toglierlo; la §5.2 del disegno della chiusura, approvata dal proprietario, l'ha deciso — P-2 del piano` |
| la riga della tabella dei piani che comincia con `\| [Sotto-progetto 1 · Traguardo 5]` | resta; **sotto** di essa due righe nuove: `\| [Sotto-progetto 1 · Traguardo 6](superpowers/plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md) \| gli **altri meccanismi**: la concessione che torna, i due schemi di filo, il sensore, il gateway col gettone, il permesso, il degrado, e le due proprietà di §5.7 che mancavano \| ✅ **eseguito il 2026-09-02**, `GATE GREEN` a ogni commit; il verbale nella §8 del suo disegno \|` e `\| [Sotto-progetto 1 · chiusura](superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md) \| la chiusura contro la §0.7: l'audit delle condizioni, `tracciabilita.md` riletta, l'archivio del Traguardo 6, il passaggio unico \| ✅ **eseguito il <data>** \|` |
| la tabella «Spike aperti» | **invariata**: il sotto-progetto 1 non ne chiude nessuno, e §7.6 lo dichiara |

- [ ] **Passo 3: `README.md`, CRLF — con Python**

| Trova | Sostituisci con |
|---|---|
| `**Fase corrente: implementazione del kernel — i sei traguardi sono ✅ eseguiti, il 6 il 2026-09-02; il sotto-progetto NON è chiuso, e la sua Definizione di «fatto» è la §0.7 della spec.**` — su tre righe nel file, si cerca il frammento `il sotto-progetto NON è chiuso` | `**Fase corrente: il sotto-progetto 1 è ✅ chiuso il <data> contro la §0.7 della sua spec; il prossimo passo lo dice la §6 del [compendio](COMPENDIO.md).**` — i richiami che seguono nel capoverso **restano** |
| la riga della tabella delle spec che comincia con `\| [Traguardo 5 — il disegno]` | resta; sotto, due righe: `\| [Traguardo 6 — il disegno](superpowers/specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) \| gli altri meccanismi \| ⛔ **Non è una spec:** perimetro, forme e il controllo che esercita ciascun artefatto; la **§8** è il verbale della chiusura del traguardo \|` e `\| [La chiusura — il disegno](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md) \| la chiusura del sotto-progetto 1 \| ⛔ **Non è una spec:** come si rilegge la §0.7 contro il codice; la **§7** è il verbale \|` |

- [ ] **Passo 4: `HANDOFF.md`, CRLF — con Python**

| Trova | Sostituisci con |
|---|---|
| `Aggiornato il **2026-09-02**, con la **chiusura del Traguardo 6** e il gotcha **#110**` | `Aggiornato il **<data>**, con la **chiusura del sotto-progetto 1**` — il resto della riga, i richiami, **resta** |
| `⛔ **Il sotto-progetto 1 NON è chiuso:** i sei traguardi sono eseguiti — il **6** il` e la riga che segue fino a `che è la §0.7 della spec.` | `✅ **Il sotto-progetto 1 è chiuso il <data> contro la §0.7 della spec**, e il verbale è la §7 del [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md).` |
| nel punto di ripresa: `e il sotto-progetto è **in corso**.` | `e il sotto-progetto 1 è **chiuso**; il prossimo passo lo dice la §6 del [compendio](COMPENDIO.md).` |
| la cella `\| [`tracciabilita.md`](tracciabilita.md) \| **171** funzionalità → dove vive ciascuna.` | `\| [`tracciabilita.md`](tracciabilita.md) \| le funzionalità della mappa originale → dove vive ciascuna; **quante** lo dice il comando nel riquadro in testa a quel file. ⚠️ **RICHIAMO DEL <data>:** la cifra è **tolta** e non riallineata — viveva qui e nel compendio, ed è la specie che il richiamo qui sotto già registra.` — il richiamo del 2026-08-11 che segue **resta** |
| nella cella di `superpowers/specs/`: `✅ **E dal 2026-08-11 il disegno del Traguardo 4**, che non è una spec nuova: è lo **scaglionamento** che la §3 deliberatamente non fissa — perimetro, dove vive ciascun pezzo, e per ogni artefatto **il controllo che lo esercita**` | `e i **disegni** dei traguardi e della chiusura, che non sono spec nuove: sono lo scaglionamento e le forme che la spec non fissa, e per ogni artefatto il controllo che lo esercita. ⛔ **Quali siano non è scritto qui:** lo dice la cartella, e la tabella delle spec di [`README.md`](README.md) — come già fa la cella dei piani` |

- [ ] **Passo 5: `AVVIO-CHAT.md`, CRLF — per sottrazione**

| Trova | Sostituisci con |
|---|---|
| `⚠️ IL REPOSITORY CONTIENE CODICE RUST, e il sotto-progetto 1 è in corso.` | `⚠️ IL REPOSITORY CONTIENE CODICE RUST.` — la frase sotto dice già che a che punto siamo lo dice la §6 |

- [ ] **Passo 6: `COMPENDIO.md`, CRLF — sei tocchi, col margine misurato prima e dopo**

| Trova | Sostituisci con |
|---|---|
| l'intestazione: `**Aggiornato il 2026-09-02**, col **Traguardo 6 CHIUSO** dal compito 10, eseguito e rivisto; l'ultimo contenuto di **merito** è` e la riga seguente fino a `Manutenzione: §13.` | `**Aggiornato il <data>**, col **SOTTO-PROGETTO 1 CHIUSO** contro la §0.7 della sua spec, dal piano della chiusura eseguito e rivisto; l'ultimo contenuto di **merito** è il passaggio unico. Manutenzione: §13.` — i richiami sotto **restano** |
| in §6, la riga che comincia col marcatore `⏭️` — **qualunque cosa dica quel giorno** (P-5) | `✅ **IL SOTTO-PROGETTO 1 È CHIUSO IL <data> contro la §0.7 della spec**, e il verbale è la §7 del [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md). ⏭️ **IL PROSSIMO PASSO: il brainstorming delle DUE FUNZIONI CORE** — il riconoscimento gesti dalla telecamera, e la knowledge base — **due brainstorming distinti**, prima di aprire il sotto-progetto 2. Nessuna voce aperta lo sbarra: la colonna «chi la chiude» della §7.8 di quel disegno non nomina questo passo.` |
| in §6, dentro il blocco del Traguardo 5: `⛔ **Il sotto-progetto 1 NON è chiuso:** restano il Traguardo 6 e la §8 di` | resta; **subito dopo il capoverso** — che finisce con `non` / `di un traguardo.` — una riga: `⚠️ **RICHIAMO DEL <data>:** il sotto-progetto 1 è chiuso quel giorno contro la §0.7; la riga sopra resta perché questo blocco è tenuto parola per parola.` |
| in §6: `📌 **Il ritratto pieno, per il confronto della prossima volta:** **diciotto ✅ · tredici ⚠️ ·` fino a `si riconta **ogni volta**.` | `📌 **Il ritratto pieno si riconta OGNI VOLTA, e le cifre non stanno qui** — il 2026-09-02 erano già stantie in entrambe le tabelle. Il comando, che è il blocco **A** della §1.3 del disegno della chiusura:` seguito dal comando del blocco **A** in un blocco di codice |
| in §8: `**centosettantuno** funzionalità già mappate — ricontate sulle quindici tabelle il 2026-08-10, ché questa riga diceva *«centosettanta»*: si **aggiorna**, e **solo alla chiusura del sotto-progetto 1** — quindi non ora` | `le funzionalità sono già mappate, e **quante** lo dice il comando nel riquadro in testa a [`tracciabilita.md`](tracciabilita.md): si **aggiorna** — riletta alla chiusura del sotto-progetto 1 il <data>, e si riaggiorna a ogni sotto-progetto chiuso` |
| in §12, la riga che comincia con `\| ⛔ **come si ESEGUE il Traguardo 5**` | resta; sotto, **quattro** righe: il disegno del Traguardo 6 — `\| ⛔ **il perimetro del Traguardo 6** — gli altri meccanismi, le forme, il controllo per artefatto, e la **§8** col verbale della sua chiusura \| [`specs/2026-08-28-…-traguardo-6-altri-meccanismi-design.md`](superpowers/specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) \|` — il piano del Traguardo 6 — `\| ⛔ **come si è ESEGUITO il Traguardo 6** — dieci compiti in cinque parti, con l'errata in testa \| [`plans/2026-08-30-…-traguardo-6-altri-meccanismi.md`](superpowers/plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md) — ⚠️ **a compiti, mai intero** \|` — il disegno della chiusura — `\| ⛔ **come si è CHIUSO il sotto-progetto 1** — le condizioni della §0.7 rilette contro il codice, e la **§7** col verbale \| [`specs/2026-09-02-…-chiusura-design.md`](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md) \|` — il piano della chiusura — `\| il piano della chiusura, con l'errata in testa e la tabella della posizione \| [`plans/2026-09-02-…-chiusura.md`](superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md) \|` |

Dopo ogni tocco: `check-docs.sh` per il tetto e i link, e il margine del blocco **E**. ⛔ **Se il
tetto va rosso, si toglie prosa dalla §6 — non si alza il tetto** (P-9).

- [ ] **Passo 7: `tracciabilita.md` (D4) e `semi-dst.md`**

Nel riquadro in testa a `tracciabilita.md`, sotto la riga `✅ **Riletta il …` scritta dal compito
2, una riga: `> ✅ **Sotto-progetto 1 chiuso il <data> contro la §0.7**: la tabella si riaggiorna
a ogni sotto-progetto chiuso, come dice la riga sotto il titolo.`

In `semi-dst.md`, **dopo** il capoverso che contiene `Da oggi le campagne sono **tre**` e
finisce con `barra torna verde.`, un capoverso nuovo, con Python su CRLF:

```markdown
⚠️ **RICHIAMO DEL <data>, alla chiusura del sotto-progetto 1: le campagne sono CINQUE.** Le due
nate col Traguardo 6 — `crates/simulator/tests/gui_death_campaign.rs` e
`crates/simulator/tests/worker_kill_campaign.rs` — stanno nella **colonna di sinistra** della
tabella qui sopra: livello **1**, un caso si identifica con un **seme**. Le costanti che rendono
interpretabile una loro voce sono le proprie: `SHORT_CAMPAIGN_SEEDS` ed `EXPECTED_WORLDS` nella
prima, `SHORT_CAMPAIGN_SEEDS` e `KILL_HORIZON` nella seconda. ⛔ **L'elenco non guadagna righe:**
nessuna delle due è fallita su un caso, e il verbale della chiusura lo dichiara.
```

Le costanti si **verificano** prima di scriverle: `grep -nE 'const (SHORT_CAMPAIGN_SEEDS|EXPECTED_WORLDS|KILL_HORIZON)' crates/simulator/tests/gui_death_campaign.rs crates/simulator/tests/worker_kill_campaign.rs` — quattro righe.

- [ ] **Passo 8: il disegno — la §7 datata, e questo piano**

Nel disegno: l'intestazione `## 7. Il verbale della chiusura — in bozza dal compito 1` diventa
`## 7. Il verbale della chiusura — <data>`; il capoverso *«In bozza finché…»* diventa un richiamo
datato di una riga; in §7.2 le righe **6** e **7** ricevono l'esito con l'evidenza — `git show
--stat HEAD` per la 6, `OK` per la 7 — ⚠️ scritte **prima** del commit e rilette **dopo**, perché
l'evidenza della 6 è il commit stesso; nascono la **§7.6** (il passaggio unico: i file, le righe
tolte, le righe rimaste dentro richiami datati dal `grep` del Passo 1, il margine del compendio
prima e dopo) e la **§7.9** (che cosa questa chiusura NON ha fatto: la tabella della §1.2 del
disegno, riletta, più ciò che i compiti hanno lasciato). In questo piano la tabella della
posizione passa il compito 4 a ✅ con la data.

- [ ] **Passo 9: le prove, un commit, il push**

```bash
bash scripts/check-docs.sh; bash scripts/gate.sh
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
for f in docs/README.md docs/HANDOFF.md docs/AVVIO-CHAT.md docs/COMPENDIO.md docs/semi-dst.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; printf '   righe='; wc -l < "$f"; done
for f in docs/roadmap.md docs/tracciabilita.md docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md docs/superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md; do printf '%s CR=' "$f"; tr -cd '\r' < "$f" | wc -c; echo; done
git ls-files --eol docs/roadmap.md docs/README.md docs/HANDOFF.md docs/AVVIO-CHAT.md docs/COMPENDIO.md docs/tracciabilita.md docs/semi-dst.md
grep -rn 'NON è chiuso\|è in corso' docs/*.md | cut -c1-120
git diff --name-only -- crates/ scripts/ Cargo.lock docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
git status --porcelain
```

Atteso: `OK`, `GATE GREEN`, margine positivo, CR = righe sui cinque CRLF e zero sui quattro LF,
`ls-files --eol` invariato, ogni riga del `grep` **dentro un richiamo datato** — letta una per
una, e l'elenco in §7.6 — l'ultimo `diff` **vuoto**, e `git status` che nomina i nove file della
mappa e nient'altro.

```bash
git add docs/roadmap.md docs/README.md docs/HANDOFF.md docs/AVVIO-CHAT.md docs/COMPENDIO.md docs/tracciabilita.md docs/semi-dst.md docs/superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md docs/superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md
git commit -m "chiusura(compito 4): il SOTTO-PROGETTO 1 e' CHIUSO contro la §0.7 — il passaggio unico sui documenti di stato in un commit solo, il verbale datato nella §7 del disegno, e il prossimo passo nella §6: i due brainstorming"
git push
git show --stat HEAD
```

`git show --stat HEAD` nomina tutti e nove: è l'evidenza della condizione 6.

#### Criterio di chiusura del compito 4

- [ ] **un** commit tocca tutti i file della tabella della §5.2 del disegno più il piano, e `git show --stat HEAD` li nomina
- [ ] il disegno e il piano del Traguardo 6, e quelli della chiusura, stanno nei **quattro** indici: roadmap, README, HANDOFF, §12 del compendio
- [ ] la §7 del disegno è **datata**, con le righe 6 e 7 della §7.2, la §7.6 e la §7.9 scritte, e la §7.8 in **una** tabella con «chi la chiude»
- [ ] `grep -rn 'NON è chiuso\|è in corso' docs/*.md` non rende nessuna riga viva sul sotto-progetto 1; le rimaste sono elencate in §7.6 come richiami datati
- [ ] ⛔ la spec, `crates/`, `Cargo.lock`, `riferimenti.md`, `porta-di-qualita.md` e `CLAUDE.md` **non** sono toccati; nessuna riga ⚠️ o ⏳ della §8 è passata a ✅
- [ ] il compendio è sotto il tetto con margine positivo; CR = righe su ogni file CRLF; `ls-files --eol` invariato
- [ ] `check-docs.sh` verde, `GATE GREEN`, commit pushato, posizione del piano a quattro ✅

---

## Dopo il compito 4

⛔ **Che cosa venga dopo NON è scritto qui:** è uno stato, e la sua casa unica è la §6 del
[compendio](../../COMPENDIO.md), che il compito 4 scrive con le parole della §5.2 del disegno —
i due brainstorming, distinti, prima del sotto-progetto 2. Le voci che la chiusura lascia aperte
stanno nella §7.8 del disegno, e per nessuna il chiusore è questo piano.
