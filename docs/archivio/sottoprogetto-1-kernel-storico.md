# Archivio — le storie della §8 della spec del sotto-progetto 1

⛔ **Non è una lettura obbligatoria.** Qui stanno, parola per parola, le storie che le celle della §8 della
[spec del sotto-progetto 1](../superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) tenevano delle proprie
correzioni — che cosa una cella diceva, chi l'ha trovato, come —, coi link riscritti per questa cartella: la R5 del
[terzo audit](../audit-2026-09-30.md), col `lean-docs`, e il suo ↪ AUD-651. Nella cella resta la riga del richiamo,
con la data e ciò che è vero adesso, e il rimando qui. Un file suo, scelto dal proprietario il 2026-10-07.

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Lo stato di ogni `V` e di ogni `Q` lo dicono la §8.3
e la §8.4 della spec; i cinque disallineamenti della §8.5 restano là, perché sono il registro delle riaperture che la
§8.8 chiede.

## Dalla §8 della spec, il `lean-docs` della R5 — archiviati il 2026-10-07, lotto 5

Il lotto 5 del `lean-docs` della R5 del [terzo audit](../audit-2026-09-30.md), mirato come vuole la P20 del proprietario: le storie che le celle della §8.3 e della §8.4 tenevano delle proprie correzioni — il ↪ AUD-651 — e la riga *«Sulla E»* della §8.2.1, dalla spec a `82d121d`, parola per parola, coi link riscritti per questa cartella. Escono, coi richiami scritti prima del terzo audit, le metà che quei richiami correggevano; restano i richiami del terzo audit, che hanno già la forma della R5. Nella cella resta la riga del richiamo: la data, ciò che è vero adesso, dove vive la storia. Ogni taglio approvato dal proprietario.

### La riga «Sulla E» della §8.2.1, e il suo richiamo del 2026-09-08 — la metà che il richiamo correggeva, e la storia del richiamo

Righe 3633–3645 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo un paragrafo solo, col vero di oggi preso dal richiamo. Dopo «sotto carico GPU spike, dentro 8 — Voce»:

> ⚠️ **Sulla E.** Il primo worker *certo* è la generazione asset, che dichiara un carico GPU
> proprio. Il sotto-progetto 6 potrebbe anticiparla se l'indicizzazione girasse in locale
> invece che su un provider remoto: la condizione resta quella, il numero è il candidato
> odierno.
>
> ⚠️ **RICHIAMO DEL 2026-09-08 — il numero è 12, la condizione non cambia (§8.2.1).** Il 7 era il
> candidato del 2026-08-08. Il 2026-09-03 [ADR-0039](../adr/0039-telecamera-come-sorgente-di-percezione.md)
> e la roadmap (riga *«Gesti dopo GUI minima e Conversazione, e prima di Voce»*) hanno deciso che il
> **primo worker vero lo paga il 12**, il tracciatore della telecamera, e la Voce lo riusa; il 7 la
> anticipa se venisse prima, come il 6 qui sopra. Le **tre** righe che portano il numero — questa
> tabella, la riga `process` di §8.2.2 e la riga Q4 di §8.4 — dicono 12: correggerne una sola avrebbe
> lasciato le altre due a mentire. Trovato scrivendo [design/08](../design/08-strategia-di-test.md)
> nella passata sui diagrammi della stella polare della GUI, e deciso su delega del proprietario.

### La cella di V3, nella §8.3 — la storia di una cella, che AUD-651 nomina

Riga 3701 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la riga del richiamo. Dopo «una transizione interrotta lascia un passo riconciliabile (§5.7).»:

> ⚠️ **riscritta il 2026-08-07 con ADR-0034**: la cella diceva *«la configurazione non ha consumatore»*, e non è più vero — la §2.8 la mette in perimetro.

### La cella di V5, nella §8.3 — la storia di una cella, che AUD-651 nomina

Riga 3703 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la riga del richiamo; il caso, la sua contro-sonda e la seconda metà del vincolo restano nella cella. Dopo «di compilazione fallita (§7.4.1 C, §7.4.4 punto 3).»:

> ✅ **RICHIAMO DEL 2026-09-03, voce 9 di §7.8 del disegno della chiusura: il caso È STATO SCRITTO, e la riga NON è declassata.** Il *«test di compilazione fallita»* che questa cella prometteva **non esisteva**, misurato nella §7.2 del [disegno della chiusura](../superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md); il [registro](../porta-di-qualita.md) lo dichiarava dal 2026-08-10 — *«un tipo che esiste non è un controllo che scatta»*. ⛔ **Declassare sarebbe stato falso, perché nessuno dei due inneschi di §8.2 calzava:** la lacuna non aspettava né un consumatore né una misura, e §8.2 dichiara di aver cercato una terza specie e non averla trovata.

### La cella di V8, nella §8.3 — la metà che il richiamo del terzo audit corregge, e la sua storia

Riga 3706 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la riga del richiamo; il vero di oggi lo dicono l'inizio della cella e il richiamo del 2026-10-03. Dopo «AttesaUmano, su cui gira il test di Q7.»:

> ⛔ **RICHIAMO DEL 2026-09-03, voce 9 di §7.8 del disegno della chiusura: questa cella si attribuiva un merito altrui, e RESTA ✅ per un'altra ragione.** *«I confini di autonomia entrano (§0.4, §4)»* è una **voce di sezione** e non una delle tre risposte ammesse da §8.1.2 — la stessa forma corretta su `V35` il 2026-08-08 — e la **transizione ad `AttesaUmano` non esiste**: `grep -rni awaiting crates/ --include=*.rs` non rende niente, e nemmeno lo stesso comando con `autonomy` e con `attesaumano` (rilanciati il 2026-09-03; ⚠️ **rettifica dello stesso giorno**: stavano in un comando solo con due **tubi**, che §8.3 — letta **per posizione** — conta come colonne). ⛔ Quel test è il metodo di **`Q7`**, non di V8, come `Q17` prima del 2026-08-08. ✅ **Resta ✅ perché per un V l'autorità è il TESTO del vincolo (§8.1.3)**, e il tetto esiste, è **consegnato** ed è provato: `Parameters::executor_turn_limit` in `crates/kernel/src/parameters.rs`, `RunError::TurnLimitReached` in `crates/kernel/src/executor.rs`, e la sonda `the_delivered_turn_limit_is_honoured_by_its_value` in `crates/kernel/tests/executor_determinism.rs`.

### La cella di V9, nella §8.3 — la metà che il richiamo correggeva, e la storia del richiamo

Riga 3707 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui» e la riga del richiamo; i comandi del 2026-09-03 sono qui. Dopo «ingresso in AttesaUmano emette una notifica ⏳ rimandato»:

> test a esempi sull'**evento** emesso e giornalato; **la notifica all'utente** no. ⛔ **DECLASSATA DA ⚠️ A ⏳ IL 2026-09-03, voce 9 di §7.8 del disegno della chiusura:** la metà dichiarata verificata **non ha soggetto**. Nessun tipo evento — `grep -rnE -e 'pub enum [A-Za-z]*Event' -e 'pub struct [A-Za-z]*Event' crates/ --include=*.rs` non rende niente (rilanciato il 2026-09-03; ⚠️ **rettifica dello stesso giorno**: il comando stava con un **tubo**, che §8.3 — letta **per posizione** — conta come colonna) — e l'executor non scrive sul giornale in nessun percorso: `grep -niE 'journal' crates/kernel/src/executor.rs` rende una riga sola, un commento che dice che nulla chiama quel tipo. Senza la metà verificata non resta niente qui, e §8.1 chiama `rimandato` *«nessun controllo qui»* — la stessa lettura che declassò **V16** (§8.5.3.1) e **V34** (AUD-026).

### La cella di V11, nella §8.3 — la storia di una cella, che AUD-651 nomina

Riga 3709 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la riga del richiamo. Dopo «esiste, quindi la seconda metà non ha soggetto.»:

> ⚠️ **Innesco allineato il 2026-08-08:** la cella riscriveva la condizione C con parole proprie — «strumenti e sensori reali» — creando una seconda definizione di una sigla che §8.2.1 raccoglie *«una volta sola»*. La condizione C è stata **allargata alla fonte** per comprendere i sensori, invece di essere riscritta qui

### La cella di V16, nella §8.3 — la metà che il richiamo del terzo audit corregge, e la sua storia

Riga 3714 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la frase sulla metà negativa, com'era, e la riga del richiamo. Dopo «nomi di provider e parametri sì ⏳ rimandato»:

> ⛔ **Ri-giudicato il 2026-08-08, e lo stato torna a `parziale`.** La metà **positiva** — il record *deve* portare nomi di provider e parametri — è verificata qui: è il record **risolto** di §6.2, con lo stesso test a esempi su giornale sintetico che rende ✅ V15 e Q14. La metà **negativa** resta vacua: nessuna credenziale attraversa il sistema in questo perimetro (`secrets` esiste, nessun adattatore la usa), quindi un controllo proverebbe l'assenza di una cosa che non c'è — gotcha #17, e resta vero. ⚠️ **Il declassamento di §8.5.3.1 era corretto sulla metà che aveva davanti, e la metà positiva non ce l'aveva**: la formulazione in questa colonna era troncata. Vedi §8.5.5.

### La cella di V24, nella §8.3 — la metà che il richiamo correggeva, e la storia del richiamo

Riga 3722 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo i permessi, la proiezione trace e la riga del richiamo; i comandi del 2026-09-03 sono qui. Dopo «metriche e costi ne sono proiezioni ⚠️ parziale»:

> test a esempi: il picco di VRAM (§5.2.2) e i permessi attivi (§6.6) si ricavano **rileggendo il giornale**, e non esiste un secondo archivio da cui ricavarli. **La proiezione trace non esiste**, quindi l'altra metà non ha soggetto. ⛔ **RICHIAMO DEL 2026-09-03, voce 9 di §7.8 del disegno della chiusura: la cella dichiarava verificate DUE cose e una sola lo è, quindi il PICCO DI VRAM esce dalla metà verificata.** I **permessi** rileggono davvero il giornale: `is_granted` in `crates/kernel/src/permission.rs` chiama `journal.replay()`, provato da `nothing_is_granted_on_an_empty_journal` in `crates/kernel/tests/permission_triple.rs`. Il **picco** no: le chiamate a `journal.replay()` del kernel — `grep -rn 'journal.replay()' crates/kernel/src --include=*.rs` — stanno in `degradation.rs`, `permission.rs` e `reconcile.rs`, e nessuna legge il picco; `grep -rn 'VramPeak' crates/ --include=*.rs` lo trova in `crates/kernel/src/wire/worker.rs` e nei test del filo, dove `a_vram_peak_survives_the_round_trip` è un giro di codifica e decodifica. ⚠️ **Resta ⚠️ e non ⏳ perché una metà verificata resta** — i permessi — e la proiezione trace resta fuori come già era.

### La cella di V25, nella §8.3 — la metà che il richiamo correggeva, e la storia del richiamo

Riga 3723 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui» e la riga del richiamo; i comandi del 2026-09-03 sono qui. Dopo «default; un solo punto di uscita ⏳ rimandato»:

> il controllo gira a ogni commit e la **sonda scatta** — una chiamata di rete in `daemon` lo accende; **la contro-sonda non esiste**: la lista è vuota e non c'è niente di legittimo da lasciar passare (§7.4.2). È l'unica voce del catalogo provata in una direzione sola. ⛔ **DECLASSATA DA ⚠️ A ⏳ IL 2026-09-03, voce 9 di §7.8 del disegno della chiusura:** la metà dichiarata verificata è **falsa** — nessuno script guarda `daemon`. `grep -n '^CRATES=' scripts/gate-deps.sh` rende `CRATES="kernel simulator"`, `scripts/gate-attributes.sh` dichiara in un proprio commento che `platform`, `secrets` e `daemon` **non sono controllati**, e `grep -n 'daemon' scripts/*.sh` non rende nessuna riga eseguibile. ⚠️ **L'altra metà RESTA vera e non si riscrive:** la contro-sonda non esiste perché la lista è vuota (§7.4.2). È la stessa lacuna che declassò **V34** (AUD-026).

### La cella di V26, nella §8.3 — la metà che i richiami correggevano, e la storia dei richiami

Riga 3724 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo ciò che i due richiami del 2026-08-27 dicevano vero, nelle loro parole, e la riga del richiamo. Dopo «prune è un'operazione della porta journal (§4.1) ·»:

> test a esempi sulle due regole non negoziabili di ADR-0018: un record potato **dichiara** di esserlo, e un passo in dubbio non è potabile (§4.5). ⚠️ **RICHIAMO DEL 2026-08-27:** la §4.5 porta ora il proprio richiamo, e dichiara che **nessuna delle due** regole è tenuta dal codice — la prima è **violata** da entrambe le implementazioni, la seconda è tenuta con un'**altra nozione** di dubbio. Senza questo rimando le due sedi si contraddicono attraverso la citazione che le lega. ⛔ **RI-GIUDICATA IL 2026-08-27, DA ✅ A ⚠️ — finding AUD-005, e la metà che regge è la seconda.** ✅ **Verificato qui:** `prune` rifiuta un passo in dubbio e accetta uno riconciliato, con le sonde del Task 11 del Traguardo 3 — con la nozione di dubbio **divergente** che il richiamo qui sopra dichiara. ⛔ **NON verificato, e peggio che non verificato: VIOLATO.** Entrambe le implementazioni cancellano i record invece di sostituirne il payload, quindi un passo potato e uno mai scritto sono indistinguibili in tre modi — misurato il 2026-08-10, *voce aperta 1* di [`porta-di-qualita.md`](../porta-di-qualita.md) — e l'enunciato stesso (*«mai i record strutturati»*) ne esce smentito. Nessun banco tiene quella regola. ⚠️ **Perché ⚠️ e non ⏳:** §8.1.1 chiama ⏳ *«nessun controllo qui»*, e un controllo c'è. ⚠️ **E §8.1 non ha uno stato per *violato*:** aggiungerlo cambierebbe anche §8.6, quindi il fatto è scritto nella cella invece che nella parola — il costo è dichiarato qui e non taciuto

### La cella di V29, nella §8.3 — la storia di una cella, che AUD-651 nomina

Riga 3727 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la riga del richiamo. Dopo «kernel è deliberatamente non controllato: §7.4.4 punto 1.»:

> ⚠️ **Formulazione allargata il 2026-08-08 chiudendo la §7.1.1**, e confrontata con la fonte come impone §8.5.5: il testo nominava quattro assi, ADR-0034 ne aveva aggiunto un quinto il 2026-08-07. **Lo stato non cambia** — il quinto asse è difeso da un controllo di livello 1 già in perimetro

### La cella di V34, nella §8.3 — la storia di una cella, che AUD-651 nomina

Riga 3732 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui», la misura com'era scritta, e la riga del richiamo. Dopo «di lettura delle credenziali; verificabile staticamente ⏳ rimandato»:

> ⛔ **DECLASSATA DA ✅ IL 2026-08-27, finding AUD-026:** la cella diceva *«livello 2, con sonda e contro-sonda (§7.4.2)»* e quel controllo **non esiste** — `gate-deps.sh` misura i grafi di `kernel` e `simulator`, mai quello di `platform`, e in tutto il repository non esiste nessuna crate di portachiavi. Che `secrets` sia una crate separata resta vero ed è il motivo per cui le crate sono cinque e non quattro (§1.2), ma è una **scelta di scrittura** e non un controllo: §8.1.2, e la stessa lettura che declassò **V16** (§8.5.3.1). Il riquadro sotto la tabella di §7.4.2 porta la misura

### La cella di V35, nella §8.3 — la metà che il richiamo correggeva, e la storia dei richiami — una delle tre celle che si dicevano «un verbale»

Riga 3733 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui», il quarto gettone, e la riga del richiamo. Dopo «o comando sotto il livello 2 ⏳ rimandato»:

> **test a esempi**, gli stessi che rendono ✅ V37: il livello di confinamento è un campo obbligatorio dell'azione e finisce nel giornale. ⚠️ **Meccanismo rinominato il 2026-08-08:** la cella diceva «entrano (§7.4.5)», che è una voce di sezione e non una delle tre risposte ammesse da §8.1.2 — la stessa forma dei difetti §8.5.3 e §8.5.4. **Il quarto gettone si scaglia** perché nessuna porta esegue comandi qui, ed è retrofittabile: §7.4.5. ⛔ **DECLASSATA DA ⚠️ A ⏳ IL 2026-09-03, voce 9 di §7.8 del disegno della chiusura:** questa cella poggia sui *«test a esempi, gli stessi che rendono ✅ `V37`»*, e il **soggetto di `V37` non esiste** — vedi la riga `V37` di questa stessa tabella, declassata lo stesso giorno e sulla stessa misura. Senza quei test non resta niente di verificato qui. ⚠️ **Il capoverso del 2026-08-08 sul rinomino del meccanismo NON si riscrive**: è un verbale, e diceva già il vero

### La cella di V37, nella §8.3 — la metà che il richiamo correggeva, e la storia del richiamo

Riga 3735 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui», i due tipi del codice, l'innesco, e la riga del richiamo; i comandi del 2026-09-03 sono qui. Dopo «entra nel giornale insieme al passo ⏳ rimandato»:

> test a esempi: è la parte che §7.4.5 fa entrare comunque. ⛔ **DECLASSATA DA ✅ A ⏳ IL 2026-09-03, voce 9 di §7.8 del disegno della chiusura:** il **tipo del livello di confinamento non esiste**. `grep -rni confin crates/ --include=*.rs` rende **tre righe di commento** e nessun tipo, e `grep -rni sandbox crates/ --include=*.rs` non rende niente (rilanciati il 2026-09-03; ⚠️ **rettifica dello stesso giorno**: stavano in un comando solo con due **tubi**, che §8.3 — letta **per posizione** — conta come colonne). Una di quelle tre righe è `crates/platform/src/lib.rs:12`, che `grep -n 'pub trait' crates/platform/src/lib.rs` trova, e porta già il proprio comando con la propria risposta, **zero**; ⛔ **quel comando NON è citato qui perché contiene un tubo vero** — una citazione si riporta com'è, o non si riporta. `Permission` in `crates/kernel/src/permission.rs` porta `tool`, `resource` e `operation`, e `PermissionDetail` in `crates/kernel/src/record.rs` porta `tool`, `resource` e `write`: nessun livello di confinamento. È la classe di **AUD-026**, quella che declassò `V34`. ⛔ **L'innesco è NUOVO perché la riga era ✅**, e §8.6 lo pretende ora: la condizione è `D` — si esegue codice o un comando — la stessa di `V35` e `Q23`, che poggiano sullo stesso soggetto

### La cella di Q12, nella §8.4 — la storia di una cella, che AUD-651 nomina

Riga 3752 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la riga del richiamo, col vero di oggi nelle sue parole. Dopo «che emetta la proposta che il metodo verifica.»:

> ⚠️ **Motivazione corretta il 2026-08-08:** diceva *«legge ricorrenze che esistono solo quando qualcosa gira»*, ma il metodo di `design/08` è un **giornale sintetico con ricorrenza** — la ricorrenza si costruisce senza far girare niente, come per Q14. A mancare è l'anello, non il dato

### La cella di Q15, nella §8.4 — la storia di una cella, che AUD-651 nomina

Riga 3755 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo la riga del richiamo. Dopo «il gettone journal sulla conversione (§7.4.1 B, V19).»:

> ⚠️ **Riletta il 2026-08-09:** la cella diceva «riga» al singolare, ed era vera finché la riga era una; dal richiamo di §7.4.1 sono due, e la seconda è quella che vede il ponte di conversione. **Lo stato non cambia**

### La cella di Q17, nella §8.4 — la metà che i richiami correggevano, e la loro storia — una delle tre celle che si dicevano «un verbale», e quella che AUD-651 nomina per i due capoversi opposti

Riga 3757 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui», il canary, e la riga del richiamo. Dopo «segreto compare in contenuto in uscita ⏳ rimandato»:

> lato kernel: §7.4.2, riga V34 · Q24 — solo `secrets` raggiunge il portachiavi, livello 2 provato in due direzioni · **il canary è scaglionato** (§0.4, §6) e non c'è contenuto in uscita da controllare. ⛔ **Precisato il 2026-08-08, perché la cella si attribuiva un merito altrui:** il metodo che `design/08` assegna a Q17 è il **canary a esempi**, e **non ne gira niente**; il controllo di livello 2 qui accreditato è quello che `design/08` assegna a **Q24**. Resta ⚠️ e non ⏳ perché è esattamente la classe di §0.6 — *«verificato solo lato kernel»* — che §8.1.1 dichiara essere una delle due ragioni per cui il quarto stato esiste ⛔ **DECLASSATA DA ⚠️ A ⏳ IL 2026-08-27, finding AUD-026:** la metà *«lato kernel»* era **la riga V34 · Q24**, e quel livello 2 **non esiste** — quindi non resta niente di verificato qui, e la ragione scritta sopra (*«resta ⚠️ perché è la classe di §0.6»*) cade con la propria premessa. ⚠️ **Il capoverso del 2026-08-08 NON si riscrive**: è un verbale, e diceva già il vero su `design/08`

### La cella di Q23, nella §8.4 — la metà che il richiamo correggeva, e la storia dei richiami — una delle tre celle che si dicevano «un verbale»

Riga 3763 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui», le due cose che `design/08` chiede, e la riga del richiamo. Dopo «sotto il livello 2 di confinamento ⏳ rimandato»:

> ⚠️ **Riletto il 2026-08-08: `design/08` chiede _due_ cose, non una** — *«statica: nessun percorso di esecuzione senza livello richiesto»* **più** un *«test negativo: con confinamento indisponibile l'azione non parte»*. **Nessuna delle due gira qui**, e per lo stesso motivo: nessuna porta esegue comandi (§7.4.5). Ciò che entra sono i **test a esempi** su tipo, dichiarazione per azione e registrazione nel giornale — che è V37, ed è meno di quanto `design/08` chiede. ⛔ **DECLASSATA DA ⚠️ A ⏳ IL 2026-09-03, voce 9 di §7.8 del disegno della chiusura:** la cella dichiarava già che **nessuna delle due tecniche** gira qui; ciò che vi entrava erano i **test a esempi di `V37`**, e quel soggetto **non esiste** — vedi la riga `V37` di §8.3, declassata lo stesso giorno. Quindi non resta niente di verificato, e lo stato segue. ⚠️ **Il capoverso del 2026-08-08 NON si riscrive**: è un verbale, e diceva già il vero su `design/08` — come `Q17` porta il proprio

### La cella di Q24, nella §8.4 — la storia di una cella, che AUD-651 nomina

Riga 3764 di `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` a `82d121d`. Nel vivo «nessun controllo qui», il metodo e la riga del richiamo. Dopo «credenziali fuori dal gestore dei segreti ⏳ rimandato»:

> ⛔ **DECLASSATA DA ✅ IL 2026-08-27, finding AUD-026:** questa cella accreditava il livello 2 di §7.4.2, riga V34 · Q24, e quel controllo **non esiste in nessuna delle due direzioni** — vedi la riga V34 di §8.3 e il riquadro sotto la tabella di §7.4.2. Il metodo che `design/08` assegna a Q24 è proprio quella statica sul grafo delle crate, quindi §8.1.3 non lascia un secondo criterio da invocare
