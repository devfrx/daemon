# Chiusura del sotto-progetto 1: il disegno

✅ **QUESTO DISEGNO È COMPLETO DAL 2026-09-02.** Le sei sezioni sono **approvate** dal
proprietario, una per volta, in chat il 2026-09-02, e la **§6** fissa la Definizione di «fatto»
della chiusura e chi la verifica. Chi riprende ha un disegno intero da tradurre in piano — dopo
che il proprietario lo ha riletto in questa forma.

⚠️ **RICHIAMO DEL 2026-09-02, lo stesso giorno:** questo file è nato come **consegna** del
brainstorming — le sei sezioni approvate, le scelte del proprietario, la strada in quattro
compiti — e il proprietario ha scelto che il disegno lo scrivesse la sessione **successiva**.
Riscritto **sul posto**, allo stesso percorso, perché il puntatore della §6 del
[compendio](../../COMPENDIO.md) non cambiasse casa. Il merito delle sei sezioni è quello
approvato e non è stato toccato; ciò che la riscrittura ha **misurato** in più sta nella sezione
*«Cosa questo disegno ha misurato»*, e **una cifra della consegna vi risulta falsa**.

⚠️ **Non è una spec.** Come i disegni dei Traguardi 4, 5 e 6, fissa il **perimetro** della
chiusura, le **forme** di ciò che la §0.7 della [spec](2026-08-06-sottoprogetto-1-kernel.md)
chiede a parole, e per ogni artefatto **il controllo che lo esercita**. La spec resta
l'autorità, e questa chiusura **non la tocca**.

📌 **Metodo.** Ogni cifra qui è **misurata il 2026-09-02** contro il repository a `f61fd53`, il
cui commit di **merito** è `49b302d` — la chiusura del Traguardo 6 — e porta accanto il comando
che la rifà. Dove un'affermazione è una **deduzione** e non una misura, è scritto. Le cifre
**invecchiano** al primo commit che tocca ciò che misurano; i comandi no, ed è per questo che
chi chiude **rifà** ogni conteggio prima di leggere che cosa questo disegno ne dice.

**Le cinque scelte del proprietario, prese in chat il 2026-09-02**

| # | Domanda | Scelta |
|---|---|---|
| 1 | che cosa vuol dire «aggiornare `tracciabilita.md` alla chiusura» | **A** — si rilegge ogni riga con la seconda domanda; cambiano solo le righe il cui meccanismo è scaglionato, che prendono la forma 🔶 già in legenda. Nessuna colonna «nel codice»: sarebbe una seconda casa di puntatori che il registro tiene già |
| 2 | il racconto del Traguardo 6 nella §6 del compendio | **A** — la chiusura lo archivia in `archivio/stato-storico.md` parola per parola, con la cernita in due direzioni, e il tetto scende nello stesso commit. Il blocco «voci aperte e racconto del Traguardo 5» **non** si tocca: è la consolidazione del proprietario |
| 3 | il prossimo passo che la §6 scrive dopo la chiusura | **A** — il brainstorming delle **due funzioni core**, prima di aprire il sotto-progetto 2. ⛔ **Con la correzione del proprietario:** l'*Agentic OS* **è il programma intero**, non una funzione da aggiungere. Le aggiunte sono **due cose diverse**: il **riconoscimento gesti** dalla telecamera, e la **knowledge base**. Due brainstorming distinti |
| 4 | come si esegue la chiusura | **proposta 1** — un piano piccolo di **quattro compiti**, un subagente fresco per compito con revisione fra uno e l'altro; pre-controllo di ogni compito in una sessione, esecuzione in una sessione **nuova** |
| 5 | la strada | quella pesante: brainstorming → disegno scritto → piano → esecuzione. Come ogni traguardo |

📌 **Perché il brainstorming delle due funzioni viene DOPO la chiusura e PRIMA del sotto-progetto
2**, verificato col `grep` e non a memoria: la telecamera non è mai stata valutata in nessun
documento del repository; il kernel ha già gli slot che le servono — ADR-0011 tratta la
percezione always-on come **sorgente di eventi** (anello 3), ADR-0005 ha la quota audio
**sottratta**, ADR-0023 il profilo «riservato», ADR-0028 i worker in Python, ADR-0014 il confine
dei dati non fidati; la knowledge base ha già una sede in roadmap, il sotto-progetto 6. Le
decisioni di **kernel** sono piccole e in append: una terza quota nella formula di ADR-0005, il
«riservato» di ADR-0023 esteso alla telecamera, quinto pilastro o parte di Voce in ADR-0001, se
il canale `process` regge uno stream. La **capacità** si progetta quando si apre il suo
sotto-progetto: il compendio vieta di progettare una capacità L2 prima. E la GUI del
sotto-progetto 2 deve ancora fare le misure M1–M5 di ADR-0029: un flusso video nella webview è
una misura in più, meglio saperlo prima di disegnarla.

---

## 1. Il perimetro — ✅ approvata

### 1.1 Che cosa la chiusura fa

| # | Cosa | Dove finisce |
|---|---|---|
| 1 | rilegge le condizioni 1–5 della §0.7 della [spec](2026-08-06-sottoprogetto-1-kernel.md) **contro il codice di oggi**, coi comandi e non a memoria | il verbale, nell'ultima sezione di questo disegno — la **§7**, che nasce col compito 1 del piano. Precedente: la §8 del [disegno del Traguardo 6](2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) |
| 2 | rilegge [`tracciabilita.md`](../../tracciabilita.md) con la seconda domanda. La fonte di «costruito o scaglionato» è la tabella §0.4 della spec, colonne «Entra» e «Si scaglia» | le righe il cui meccanismo è scaglionato prendono la forma 🔶 col sotto-progetto che lo porta. Il segnaposto in testa diventa lo stato della chiusura |
| 3 | archivia il racconto del Traguardo 6 dalla §6 del compendio, parola per parola, con la cernita in due direzioni | [`archivio/stato-storico.md`](../../archivio/stato-storico.md), e il tetto in `scripts/check-docs.sh` scende con la regola di sempre: misurato più circa l'undici per cento, arrotondato al KiB |
| 4 | aggiorna **nello stesso passaggio** i documenti di stato: roadmap, README, HANDOFF, AVVIO-CHAT, compendio, e il richiamo in `semi-dst.md` per le due campagne del Traguardo 6 | un commit solo. Le righe precise stanno nella §5 |
| 5 | `check-docs.sh` verde, cancello verde, commit e push | condizione 7 della §0.7 |

### 1.2 Che cosa la chiusura NON fa

| | Perché |
|---|---|
| non tocca la spec | la §0.7 è una definizione, non uno stato. Lo stato ha già le sue case, e la spec è del proprietario (vincolo globale 7) |
| non chiude le voci aperte del proprietario | le dichiara. La consolidazione resta una voce a sé, da presentare una per una |
| non riordina la roadmap | il prossimo passo scritto in §6 del compendio è il brainstorming delle due funzioni core. Il riordino, se serve, esce da lì |
| non archivia il blocco «voci aperte e racconto del Traguardo 5» | è la consolidazione del proprietario, e il compendio lo dice |
| non tocca [`riferimenti.md`](../../riferimenti.md) | la voce `E146` del [piano del Traguardo 5](../plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) resta registrata e non presa. ⚠️ Il piano del Traguardo 6 ha una **propria** `E146` che dice un'altra cosa: un numero d'errata non è un riferimento fuori dal proprio piano |
| non rilancia tutte le mutazioni | la condizione 2 si prova leggendo l'evidenza già registrata: lo `.stderr` per il livello 1, la mutazione nel [registro](../../porta-di-qualita.md) per il livello 2. Si rilancia solo dove manca, o dove il file è cambiato dopo la misura |
| non crea righe di catalogo, e non marca ✅ nessuna riga ⚠️ o ⏳ della §8 | §7.4 e §8 sono spec. Chiudere troppo è un modo di fallire, come la condizione 12 del Traguardo 6 |

### 1.3 Il perimetro in numeri, misurati il 2026-09-02

⚠️ **Si rifanno col comando prima di essere letti.** Ogni riga rimanda al blocco di comandi
qui sotto, che è la sua casa; la cifra è l'istantanea di quel giorno.

| | Misurato il 2026-09-02 | Comando |
|---|---|---|
| righe della §8 della spec da rileggere, le ✅ e le ⚠️ | 45: 30 V (16 ✅ e 14 ⚠️) e 15 Q (8 ✅ e 7 ⚠️) | blocco **A** |
| righe ⏳ e ⚠️ della §8, e quante hanno una sede nell'innesco | 37, di cui **36** con un sotto-progetto. ⛔ **La consegna diceva 37 su 37, ed era falso:** `V26`, la ritenzione, ha l'innesco **senza numero**, e lo dichiara — *«la roadmap non colloca la ritenzione da nessuna parte»* | blocco **B** |
| righe di `tracciabilita.md` con uno stato | 171: 71 ✅ · 25 🔶 · 74 📋 · 0 ⚠️ · 1 ❌ | blocco **C** |
| byte da archiviare dalla §6 del compendio | 38 713, compresa la riga che la consegna vi aggiunse lo stesso giorno | blocco **D** |
| margine sotto il tetto del compendio | 2147 byte | blocco **E** |
| ADR della §0.5 con la sezione `Negative (accettate)` | sette su sette | blocco **F** |
| campagne DST | cinque, di cui due nate col Traguardo 6. `semi-dst.md` ne dichiara tre | blocco **G** |
| casi `compile_fail` e i loro `.stderr` | 40 e 40, nessuno vuoto | blocco **H** |
| piano e disegno del Traguardo 6 nei quattro indici | zero in roadmap, README e HANDOFF; le due occorrenze del compendio stanno in §6, non in §12 | blocco **I** |
| spike chiusi dal sotto-progetto 1 | nessuno: SP-1 e SP-2 li chiudono i sotto-progetti 7 e 8 | tabella «Spike aperti» di [`roadmap.md`](../../roadmap.md) |

**Blocco A** — le righe di §8 per stato, V e Q separati, delimitando per l'intestazione
`## 8.` come fa `check-docs.sh` e non per numero di riga:

```bash
awk '/^## 8\. /{i=1} i && /^\|[[:space:]]*[VQ][0-9]+[[:space:]]*\|/ {r=$0; gsub(/\\\|/,"",r); split(r,c,"|"); id=c[2]; gsub(/ /,"",id); k=(c[4]~/verificato qui/)?"ok":(c[4]~/parziale/)?"parziale":(c[4]~/rimandato/)?"rimandato":"altro"; n[substr(id,1,1)" "k]++} END{for (x in n) print x, n[x]}' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

**Blocco B** — le righe ⏳ e ⚠️ il cui innesco nomina un sotto-progetto, e quali no:

```bash
awk '/^## 8\. /{i=1} i && /^\|[[:space:]]*[VQ][0-9]+[[:space:]]*\|/ {r=$0; gsub(/\\\|/,"",r); split(r,c,"|"); if (c[4]~/parziale|rimandato/) {t++; if (c[6]~/\([0-9]+\)|SP-/) s++; else print "senza sede:", c[2]}} END{print t, "righe,", s, "con una sede"}' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

**Blocco C** — le righe di `tracciabilita.md` per stato:

```bash
for s in ✅ 🔶 📋 ⚠️ ❌; do printf '%s ' "$s"; grep -cE "^\| .* \| $s \|" docs/tracciabilita.md; done
```

**Blocco D** — i byte del blocco della §6 che la §4 archivia, delimitato coi marcatori:

```bash
sed -n '/^### Il prossimo passo/,/^✅ \*\*I finding dell/p' docs/COMPENDIO.md | head -n -1 | wc -c
```

**Blocco E** — il margine sotto il tetto:

```bash
echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh | cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))
```

**Blocco F** — gli ADR della §0.5, ciascuno con le proprie `Negative (accettate)`:

```bash
grep -c 'Negative (accettate)' docs/adr/003[1-7]-*.md
```

**Blocco G** — le campagne DST:

```bash
ls crates/simulator/tests/*campaign*.rs crates/platform/tests/engine_crash_consistency.rs
```

**Blocco H** — i casi `compile_fail`, i loro `.stderr`, e quanti sono vuoti:

```bash
ls crates/kernel/tests/compile_fail/*.rs | wc -l; ls crates/kernel/tests/compile_fail/*.stderr | wc -l; find crates/kernel/tests/compile_fail -name '*.stderr' -empty | wc -l
```

**Blocco I** — il Traguardo 6 nei quattro indici:

```bash
for f in docs/roadmap.md docs/README.md docs/HANDOFF.md docs/COMPENDIO.md; do printf '%s ' "$f"; grep -c 'traguardo-6' "$f"; done
```

---

## 2. Le sette condizioni della §0.7 — ✅ approvata

### 2.1 Tre regole di lettura

| | Regola |
|---|---|
| a | le condizioni 1 e 3 si leggono sulle sole righe ✅ e sulla metà verificata delle ⚠️ della §8. Una riga ⏳ non le viola: lo dice il rimando della §0.7 |
| b | «controllo statico» nella condizione 2 vuol dire: i casi di livello 1, cioè il compilatore, e i quattro script che leggono sorgenti, manifesti e documenti senza eseguire nulla — `gate-no-os.sh`, `gate-deps.sh`, `gate-attributes.sh`, `check-docs.sh`. I test di livello 2 sono dinamici e stanno sotto la condizione 1 |
| c | prima domanda: «è già fatto?». Ogni conteggio si rifà col comando prima di leggere che cosa i documenti ne dicono. Gotcha #49 |

### 2.2 Le condizioni

| # | Condizione | Che cosa conta come evidenza | Come si prova | Già misurato il 2026-09-02 |
|---|---|---|---|---|
| 1 | ogni V in perimetro ha un controllo che gira in automatico | la cella «meccanismo» di §8.3 nomina una delle tre risposte ammesse da §8.1.2: una riga del catalogo §7.4, un test a esempi, la campagna DST con la sua porta. E quella cosa **esiste** | per ogni riga: la riga di catalogo si cerca nel [registro](../../porta-di-qualita.md), che dice il file; il test con `grep -rn 'fn <nome>'`; la campagna col file. È la classe di AUD-026 ricensita su tutte le righe | 30 righe (blocco **A**). Il cancello è verde |
| 2 | ogni controllo statico è stato visto fallire, e poi tornare verde | livello 1: ogni caso ha il suo `.stderr` non vuoto e la suite è verde. Script: la direzione «deve scattare» **rilanciata**, perché le loro prove stanno sparse fra piano del Traguardo 1, audit e spec §8.6.3, e gli script sono cambiati dopo | `ls *.rs` contro `ls *.stderr` in `crates/kernel/tests/compile_fail/`, e nessuno vuoto. Per ogni script una violazione deliberata, il rosso, la revoca con `git diff` a zero | 40 casi, 40 `.stderr`, 0 vuoti (blocco **H**). Gli script: da rilanciare, quattro prove |
| 3 | ogni Q in perimetro è verificato col metodo di `design/08` | la cella di §8.4 nomina il metodo che la tabella «Mappa requisito → metodo di verifica» di [`design/08`](../../design/08-strategia-di-test.md) assegna a quel Q, non un altro | confronto riga per riga | 15 righe (blocco **A**); il confronto è da fare |
| 4 | ogni difetto trovato in simulazione conserva il seme (V31) | l'elenco di [`semi-dst.md`](../../semi-dst.md) vuoto **e dichiarato**, il cancello verde su tutte le campagne, e la riga V31 di §8.3 già a ✅ «debole per natura» | `bash scripts/gate.sh`, poi il richiamo datato in `semi-dst.md` per le due campagne del Traguardo 6, entrambe a livello 1, con le costanti che rendono leggibile una loro voce: `SHORT_CAMPAIGN_SEEDS` ed `EXPECTED_WORLDS` in `gui_death_campaign.rs`, `SHORT_CAMPAIGN_SEEDS` e `KILL_HORIZON` in `worker_kill_campaign.rs` | verde. Il richiamo manca |
| 5 | gli ADR della §0.5 sono scritti, ciascuno con le `Negative (accettate)` | sette file in `docs/adr/`, ciascuno con l'intestazione esatta; la riga 3 della §0.5 — dove vive l'esecutore — è decisa in §2.4 senza ADR, ed è dichiarato | `ls docs/adr/003[1-7]-*.md` e il blocco **F** | sette su sette, e l'intestazione c'è in tutti |
| 6 | roadmap, tracciabilità, spike e HANDOFF aggiornati nello stesso passaggio | **un commit** che li tocca tutti, più compendio, README e AVVIO-CHAT che [`CLAUDE.md`](../../../CLAUDE.md) aggiunge | `git show --stat HEAD` li nomina tutti. Le righe le elenca la §5 | da fare |
| 7 | `check-docs.sh` verde | l'uscita `OK` | `bash scripts/check-docs.sh`, e con esso il cancello | verde |

### 2.3 Che cosa il verbale scrive per ogni condizione

Una riga per condizione, come la §8.2 del disegno del Traguardo 6: ✅ con l'evidenza letta o
rilanciata quel giorno, oppure ❌ con che cosa manca. E una tabella «dove la condizione era
scritta troppo larga», se capita: è successo alla 4 del Traguardo 4.

⚠️ **La condizione 4 è la candidata**, ed è una deduzione e non una misura: la §0.7 dice
*«conserva il proprio seed come caso di regressione permanente»*, mentre ADR-0021 dal
2026-08-08 dice che **a diventare regressione è la proprietà** che il difetto violava, non il
seme — e `semi-dst.md` esiste per dirlo. Se la lettera della condizione risulti più larga della
decisione lo scrive il verbale, e toccare la §0.7 resta del proprietario.

---

## 3. La rilettura di `tracciabilita.md` — ✅ approvata

### 3.1 La domanda, e le fonti che rispondono

Per ogni riga: «quale meccanismo del kernel le serve, e il codice lo dà oggi o è scaglionato?».
Nessuna risposta a memoria: tre fonti, in quest'ordine.

| # | Fonte | Che cosa dice |
|---|---|---|
| 1 | la tabella §0.4 della spec, colonne «Entra» e «Si scaglia» | se il meccanismo è costruito nel sotto-progetto 1 o rinviato |
| 2 | la colonna «Innesco» di §8.3 e §8.4 | per un meccanismo rinviato, il **sotto-progetto** che lo porta: la lettera è la condizione, il numero fra parentesi il sotto-progetto. Misurato col blocco **B**: A→2, B→3, C→4, D→5, E→7, F→11, SP-2→8. ⛔ **Non tutte le righe ne portano uno:** `V26` no |
| 3 | la tabella dei sotto-progetti di [`roadmap.md`](../../roadmap.md) | il nome del sotto-progetto, per scriverlo nella cella |

### 3.2 Che cosa cambia, e che cosa no

| Riga | Esito |
|---|---|
| ✅ il cui meccanismo è in «Entra» ed esiste in `crates/` | **resta** com'è. La sede resta spec e ADR: nessun puntatore al codice |
| ✅ il cui meccanismo è in «Si scaglia» | diventa **🔶**, con «implementazione → sotto-progetto N» preso dall'innesco. È la forma che la legenda ha già |
| ✅ scaglionato ma senza sotto-progetto in nessuna delle tre fonti | diventa 🔶 con «sede da assegnare», e la sede mancante è una **voce aperta del proprietario** nella tabella della chiusura. Non si inventa. ⚠️ **Il caso esiste già, misurato:** la **ritenzione a livelli** di ADR-0018 — `V26` ha l'innesco senza numero, e `roadmap.md` non contiene la parola *ritenzione*. Quante righe di `tracciabilita.md` poggino su quel meccanismo lo dice la rilettura, non questo disegno |
| 🔶 già oggi | si rilegge, cambia solo se la sede è sbagliata |
| 📋 e ❌ | non dipendono dal sotto-progetto 1: intatte |
| una funzionalità il cui meccanismo **nessuno ha deciso** | riga nuova nella sezione «Lacune», come L-1…L-5. La sezione lo prescrive già |

La legenda non cambia. Il criterio di «esiste in `crates/`»: `ls crates/kernel/src/` dà i
moduli, e un meccanismo costruito ne nomina uno.

### 3.3 Il segnaposto in testa

Il riquadro «la tabella si aggiorna alla chiusura del sotto-progetto, non ora» diventa lo stato
della chiusura: la data, la domanda con cui si è riletta, e il **comando** che conta le righe per
stato — il blocco **C** — non le cifre. L'elenco delle righe cambiate, con la ragione di
ciascuna, vive nel verbale della chiusura e il riquadro vi rimanda: una casa sola.

### 3.4 La prova, in due direzioni

| Direzione | Come |
|---|---|
| ogni riga passata a 🔶 doveva cambiare | il meccanismo che nomina compare in «Si scaglia» di §0.4 o in un innesco di §8, col `grep` |
| ogni riga rimasta ✅ doveva restare | nel verbale le ✅ si **raggruppano per meccanismo**, circa dieci gruppi, e ogni gruppo nomina il modulo che esiste. Settantuno righe, non settantuno frasi |
| il revisore | rifà i conteggi coi comandi, e **ri-deriva da solo** l'insieme delle righe cambiate contro §0.4, prima di leggere il verbale |

### 3.5 I numeri del 2026-09-02

71 righe ✅ (blocco **C**); circa 26 candidate a 🔶 per euristica sulla colonna «Sede» — il
numero vero lo dà la rilettura, e vince lui; **36 righe su 37** di §8 con una sede già scritta
nell'innesco (blocco **B**). ⛔ **La consegna scriveva *«37 su 37»* e *«la sede c'è, quasi
sempre»*:** la cifra era falsa, il *«quasi»* era giusto. La riga senza sede è `V26`.

---

## 4. L'archiviazione del racconto del Traguardo 6, e il tetto — ✅ approvata

### 4.1 Che cosa esce dalla §6

Il blocco che va da `### Il prossimo passo` fino alla riga **prima** di `✅ **I finding
dell'audit sono chiusi**`, delimitato coi marcatori e non coi numeri di riga. Circa 38 KB,
**parola per parola** — il blocco **D** li conta. Tre eccezioni restano in §6, dichiarate
nell'archivio:

| Resta | Perché |
|---|---|
| la riga del prossimo passo, quella col marcatore | è il puntatore vivo. La §5 la riscrive. Un marcatore nell'archivio sarebbe un secondo puntatore, morto |
| il blocco «i finding dell'audit sono chiusi», col suo `awk` | è vero adesso, ed è la casa dello stato dell'audit col comando |
| una riga che sia una **regola** per i piani futuri senza altra casa | su un dubbio si conserva. Il compito le elenca nel verbale |

⚠️ **Dal 2026-09-02 il blocco contiene anche la riga che la consegna aggiunse alla §6** — «il
brainstorming della chiusura è fatto», riscritta lo stesso giorno in «il disegno è scritto». È un
verbale come il resto: parte con il blocco.

### 4.2 Che cosa NON esce

La testa della §6, il blocco «voci ancora aperte e racconto del Traguardo 5», la tabella dei
traguardi, la coda.

### 4.3 La cernita, in due direzioni

| Direzione | Come |
|---|---|
| ogni voce che il blocco chiama aperta ha un'altra casa | marcatori: «aperta», «non presa», «del proprietario», «registrata», i codici `E<n>`. Per ciascuna, `grep` nelle due tabelle uniche del [registro](../../porta-di-qualita.md) e nella tabella X dell'[audit](../../audit-2026-08-27.md). Trovata: il blocco parte. Non trovata: il blocco **resta**, e la voce entra nella tabella della chiusura |
| ciò che parte è ripassato con marcatori **diversi** | tenendo tutto ciò che pescano. È il precedente del 2026-09-01 |

⚠️ Il 2026-09-02 il blocco nominava 58 codici `E<n>` distinti e ognuno ha una riga nell'errata
del [piano del Traguardo 6](../plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md)
— ⛔ ma un numero d'errata **non è un riferimento** fuori dal proprio piano: la cernita si fa
per voce, leggendo, non per codice.

### 4.4 Dove finisce, e in che forma

Una sezione nuova in `archivio/stato-storico.md`: titolo `## Il racconto del Traguardo 6 —
archiviato il <data>`, il riquadro «Verbale, non stato», il perché in due righe, il criterio di
taglio «resta in §6 ciò che è vero adesso», le tre eccezioni, poi il blocco verbatim. I link
relativi che contiene prendono **tutti** il prefisso `../`, e `check-docs.sh` lo verifica.
⚠️ **RICHIAMO DEL 2026-09-02, scrivendo il piano:** qui stava *«sei il 2026-09-02»*, e il
comando — il `grep` dei bersagli sul blocco estratto, **P-1** del piano — ne rende **otto**: la
cifra veniva dalla consegna, misurata prima che la sua stessa riga entrasse nel blocco. **Tolta,
non riallineata:** conta la regola, ogni bersaglio.

### 4.5 Il puntatore che resta in §6

Un blocco corto, come quello del 2026-09-01: che cosa è uscito, dove sta, la prova della
cernita, e «nulla riassunto, nulla cancellato».

### 4.6 Il tetto

Scende nello **stesso commit** dell'archiviazione, con la regola scritta nello script: misurato
più circa l'undici per cento, arrotondato al KiB. Il numero lo dà il comando, e va nel richiamo
datato del blocco di commento di `scripts/check-docs.sh`:

```bash
size=$(wc -c < docs/COMPENDIO.md); echo $(( (size * 111 / 100 + 1023) / 1024 * 1024 ))
```

L'undici per cento copre le righe corte che la §5 aggiunge dopo.

### 4.7 Fine-riga

| File | Nell'albero | Regola |
|---|---|---|
| `docs/COMPENDIO.md` | CRLF, un CR per riga | si scrive in Python con `newline=''`, temporaneo e `os.replace`. Il CR dopo deve fare il numero di righe |
| `docs/archivio/stato-storico.md` | LF, zero CR | il blocco entra in LF: zero CR dopo |
| `scripts/check-docs.sh` | CRLF | stesso metodo del compendio |

### 4.8 Le prove

`check-docs.sh` verde, quindi link e tetto · `git diff --stat` nomina esattamente **tre** file ·
`git ls-files --eol` invariato · il cancello verde.

---

## 5. Il passaggio unico sui documenti di stato — ✅ approvata

### 5.1 Trovato preparando questa sezione

⛔ Il piano e il disegno del Traguardo 6 non compaiono in **nessun** indice — né nella tabella
dei piani della roadmap, né nella tabella delle spec del README, né nella mappa dei documenti
di HANDOFF, né nella §12 del compendio. Dal 2026-08-30 (blocco **I**). È la classe del finding
AUD-040, e il passaggio unico la chiude.

### 5.2 Che cosa cambia, file per file

Ogni riga si ritrova col `grep` sulla frase, mai col numero di riga.

| File | La frase di oggi | Diventa | Regola |
|---|---|---|---|
| `roadmap.md` | «Ultimo aggiornamento: 2026-08-25, alla chiusura del Traguardo 5» | la data della chiusura del sotto-progetto 1 | la data segue il file |
| `roadmap.md` | riga 1 della tabella dei sotto-progetti: «🔵 in corso» più il racconto dei traguardi 1–5 nella stessa cella | «✅ chiuso il <data> contro la §0.7», col rimando al verbale. Il racconto **esce**: lo stato per traguardo ha già la sua tabella nello stesso file | sottrazione, una casa sola |
| `roadmap.md` | tabella dei piani, ferma al Traguardo 5 | una riga per il piano del Traguardo 6 e una per il piano della chiusura | AUD-040 |
| `roadmap.md` | tabella degli spike | **invariata**: il sotto-progetto 1 non ne chiude nessuno. Il verbale lo dichiara | regola di manutenzione, punto 3 |
| `README.md` | «i sei traguardi sono ✅ eseguiti … il sotto-progetto NON è chiuso» | «il sotto-progetto 1 è ✅ chiuso il <data>; il prossimo passo lo dice la §6 del compendio» | ciò che è vero adesso |
| `README.md` | tabella delle spec, righe dei disegni fino al Traguardo 5 | una riga per il disegno del Traguardo 6 e una per quello della chiusura | AUD-040 |
| `HANDOFF.md` | intestazione «con la chiusura del Traguardo 6» · «⛔ Il sotto-progetto 1 NON è chiuso» · «il sotto-progetto è in corso» nel punto di ripresa | la chiusura, la data, il rimando al verbale | AUD-039 |
| `HANDOFF.md` | mappa: la cella di `superpowers/specs/` enumera i disegni e si ferma al 4; la cella di `tracciabilita.md` dice «**171**» | la cella smette di enumerare e rimanda alla cartella, come già fa quella dei piani; la cifra esce e rimanda al comando in testa a `tracciabilita.md` | un elenco invecchia, una regola no |
| `AVVIO-CHAT.md` | «e il sotto-progetto 1 è in corso» | lo stato **esce** dalla riga: la frase sotto dice già che a che punto siamo lo dice la §6 | sottrazione |
| `COMPENDIO.md` | intestazione «col Traguardo 6 CHIUSO … l'ultimo contenuto di merito è il compito 10» | «col SOTTO-PROGETTO 1 CHIUSO», la data | AUD-034 |
| `COMPENDIO.md` | §6, la riga del prossimo passo | «✅ il sotto-progetto 1 è chiuso il <data> contro la §0.7, verbale nel disegno della chiusura. ⏭️ il prossimo passo: il brainstorming delle **due funzioni core**, gesti dalla telecamera e knowledge base, due brainstorming distinti, prima di aprire il sotto-progetto 2. Nessuna voce aperta lo sbarra» — nella §6, che è l'unica casa del marcatore | corto: il tetto |
| `COMPENDIO.md` | §6, dentro il blocco del Traguardo 5: «⛔ Il sotto-progetto 1 NON è chiuso: restano il Traguardo 6 e la §8 di tracciabilita.md» | resta, con un richiamo datato di una riga. Il blocco è tenuto parola per parola, e una riga falsa non può restare muta | richiamo datato |
| `COMPENDIO.md` | §6, «Il ritratto pieno … diciotto ✅ · tredici ⚠️ · sei ⏳ … nove · otto · sette» | le cifre **escono**, resta il comando che le riconta — il blocco **A** — e «si riconta ogni volta». Il 2026-09-02 danno 16·14·7 e 8·7·9: il ritratto era già stantio | gotcha #31 |
| `COMPENDIO.md` | §8, «si aggiorna, e solo alla chiusura del sotto-progetto 1 — quindi non ora», con «centosettantuno» | «riletta alla chiusura del sotto-progetto 1 il <data>; si riaggiorna a ogni sotto-progetto chiuso». La cifra esce e rimanda al comando in testa a `tracciabilita.md` | sottrazione |
| `COMPENDIO.md` | §12, ferma ai disegni e piani del Traguardo 5 | una riga per disegno e piano del Traguardo 6, una per quelli della chiusura | AUD-040 |
| `tracciabilita.md` | il riquadro «la tabella si aggiorna alla chiusura del sotto-progetto, non ora» | lo stato della chiusura, come da §3.3 | |
| `semi-dst.md` | l'ultimo richiamo, del 2026-08-25, «le campagne sono tre» | un richiamo datato in più: due campagne col Traguardo 6, entrambe a livello 1, con le costanti che rendono leggibile una voce; l'elenco non guadagna righe, e il perché | il precedente del Traguardo 5 |
| questo disegno | la **§7**, il verbale, in bozza dal compito 1 | datata e chiusa | |

### 5.3 Che cosa NON si tocca

La spec del sotto-progetto 1 · [`porta-di-qualita.md`](../../porta-di-qualita.md), salvo che
l'audit vi trovi una riga falsa · [`riferimenti.md`](../../riferimenti.md) ·
[`CLAUDE.md`](../../../CLAUDE.md): il modo di lavorare non cambia · l'audit del 2026-08-27 ·
nessun numerale nuovo in prosa, mai: date, comandi e rimandi.

### 5.4 L'ordine dei commit

| Compito | Commit | Che cosa può dire |
|---|---|---|
| 1, audit | il verbale in bozza in questo disegno | «le condizioni rilette», mai «chiuso» |
| 2, tracciabilità | le righe passate a 🔶 | vere a prescindere dalla chiusura |
| 3, archivio | il blocco, il puntatore, il tetto | «archiviato perché il Traguardo 6 è chiuso e il tetto», mai «sotto-progetto chiuso» |
| 4, il passaggio | **tutto il resto della tabella in un commit solo**, poi il push | è l'unico che dice «chiuso» |

Così fra un commit e l'altro i documenti non si contraddicono mai.

### 5.5 Le prove

`git show --stat HEAD` nomina tutti i file della tabella · `grep -rn 'NON è chiuso\|è in corso'
docs/*.md` non rende più nessuna riga viva sul sotto-progetto 1, e le righe che restano stanno
dentro richiami datati · `check-docs.sh` verde, cancello verde · fine-riga misurati file per
file.

---

## 6. Chi verifica, come, e la Definizione di «fatto» della chiusura — ✅ approvata

### 6.1 Chi fa che cosa

| Ruolo | Chi | Fa |
|---|---|---|
| coordinatore | la sessione che scrive il piano, poi quella che lo esegue | il pre-controllo delle quattro domande di [`CLAUDE.md`](../../../CLAUDE.md) su ogni compito **prima** di dispacciarlo, il ledger, i ruling, e la seconda ondata di sola prosa a mano |
| esecutore | un subagente fresco per compito | esegue il compito; si ferma e riporta se il compito dice il falso, non aggira |
| revisore | un subagente fresco per compito | rilancia **ogni comando** accanto a un'affermazione misurabile e li elenca; per la tracciabilità ri-deriva da solo l'insieme delle righe cambiate prima di leggere il verbale |
| proprietario | | approva il disegno e il piano, decide le voci aperte che la chiusura registra |

Il pre-controllo si fa in una sessione; l'esecuzione in una sessione **nuova**, come al
compito 10 del Traguardo 6.

### 6.2 Le regole per chi chiude

| # | Regola |
|---|---|
| 1 | si parte dai numeri: ogni conteggio si rifà col comando prima di leggere che cosa i documenti ne dicono |
| 2 | la prima domanda è «è già fatto?», non «come lo faccio?». Ciò che si trova eseguito si riconosce e si scrive che lo era |
| 3 | la §0.7 si rilegge **contro il codice**, mai contro sé stessa: una condizione può risultare scritta troppo larga, e si dice |
| 4 | un rimedio si chiude su tutte le case della frase, non su quella dove lo si è trovato |
| 5 | si **toglie** invece di riscrivere meglio: meno parole, meno superficie per una falsità nuova |
| 6 | il verbale sta nell'ultima sezione di questo disegno — la **§7**, che il compito 1 apre in bozza e il compito 4 data e chiude — con la tabella «dove il disegno è stato smentito dall'esecuzione» e «che cosa la chiusura NON ha fatto». È la decisione **B** del disegno del Traguardo 6, per la stessa ragione: il compendio ha un tetto nel cancello |

### 6.3 La Definizione di «fatto» della chiusura

Nessuna condizione porta un numerale che il lavoro può muovere: nomina la sezione, e il
conteggio lo fa chi chiude.

| # | Condizione |
|---|---|
| 1 | `bash scripts/gate.sh` → `GATE GREEN`, `check-docs.sh` → `OK`, baseline **rimisurata** col comando |
| 2 | le condizioni 1–5 della §0.7 hanno ciascuna la propria riga nel verbale, con l'evidenza **letta o rilanciata quel giorno**: per i quattro script il rosso rilanciato e revocato con `git diff` a zero |
| 3 | `tracciabilita.md`: ogni riga ✅ poggia su un meccanismo in «Entra» che nomina un modulo esistente, raggruppato nel verbale; ogni riga 🔶 nomina un sotto-progetto o «sede da assegnare» registrata come voce aperta; il riquadro in testa porta la data, la domanda e il comando; la legenda è intatta |
| 4 | il racconto del Traguardo 6 sta in `archivio/stato-storico.md` parola per parola con le tre eccezioni dichiarate, i link riscritti, la cernita provata in due direzioni, e il tetto sceso **nello stesso commit** col richiamo datato |
| 5 | il commit del passaggio unico tocca **tutti** i file della tabella della §5.2, e il disegno e il piano del Traguardo 6 e della chiusura stanno nei quattro indici |
| 6 | le voci che la chiusura apre stanno in **una** tabella di questo disegno, con la colonna «chi la chiude», e nessuna è decisa al posto del proprietario |
| 7 | ⛔ **negativa**: la spec non è toccata, `git diff --name-only <base>..HEAD -- docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` non rende niente; nessuna riga ⚠️ o ⏳ della §8 passa a ✅; nessuna cifra nuova in prosa; `riferimenti.md` intatto |
| 8 | ogni compito ha avuto la sua revisione, e nessuna revisione ha lasciato un Important aperto |

### 6.4 Le ondate

Una revisione per compito. Se una seconda ondata apre rilievi di **sola prosa**, la fa il
coordinatore a mano e senza ri-revisione: è la regola che il proprietario ha fissato il
2026-09-02 al compito 8 del Traguardo 6. Se tocca un fatto o un comando, si ri-rivede.

---

## Cosa questo disegno ha misurato, e che non era scritto da nessuna parte

⛔ **È la parte che costa di più riscoprire.** Ognuna è stata verificata con un comando, il
2026-09-02; le prime sette dal brainstorming, le ultime quattro dalla riscrittura in disegno.

| | Scoperta | Che cosa ne segue |
|---|---|---|
| 1 | il piano e il disegno del Traguardo 6 **non stanno in nessun indice**: roadmap, README, HANDOFF, §12 del compendio (blocco **I**) | il passaggio unico li aggiunge, insieme a quelli della chiusura |
| 2 | «Il ritratto pieno» della §6 del compendio è **stantio**: dice diciotto·tredici·sei e nove·otto·sette, e le terze colonne di §8.3 e §8.4 danno 16·14·7 e 8·7·9 (blocco **A**) | le cifre escono, resta il comando |
| 3 | `semi-dst.md` dichiara tre campagne; ne esistono cinque, due nate col Traguardo 6, entrambe a livello 1 (blocco **G**) | un richiamo datato, come quello del 2026-08-25 |
| 4 | la telecamera non è mai stata valutata: `grep -rniE 'telecamera\|camera\|webcam\|mediapipe\|gestur'` su `docs/`, `docs/adr/`, `docs/design/`, `spikes/` non rende niente di pertinente. L'unica riga vicina è ADR-0011, sulla percezione always-on | è un'idea nuova, non una decisione da riaprire |
| 5 | [`porta-di-qualita.md`](../../porta-di-qualita.md) non ha sezioni per gli script del cancello: `grep -nE '^#{2,4} .*gate-'` non rende niente. Le prove in negativo degli script stanno nel piano del Traguardo 1, nell'audit e nella spec §8.6.3 | per la condizione 2 si **rilancia**, quattro prove |
| 6 | i fine-riga dei file che la chiusura tocca — `git ls-files --eol`: CRLF nell'albero per compendio, HANDOFF, README, AVVIO-CHAT, semi-dst, `check-docs.sh`; LF per roadmap, tracciabilità, stato-storico. Nell'indice tutti LF | ogni scrittura conserva i fine-riga del proprio file e li rimisura |
| 7 | ⛔ **le righe ⏳ e ⚠️ di §8 con un sotto-progetto nell'innesco sono 36 su 37, non 37 su 37** come la consegna scriveva (blocco **B**). La riga senza è `V26`, la ritenzione a livelli di ADR-0018: il suo innesco dice *«il numero non è assegnato: la roadmap non colloca la ritenzione da nessuna parte»*, e `grep -ci ritenzion docs/roadmap.md` rende zero | la §3.2 ha già la forma per questo caso — 🔶 «sede da assegnare», voce aperta del proprietario — e la voce **3** qui sotto lo nomina. ⚠️ Una cifra scritta in chat e approvata era falsa lo stesso giorno: è il gotcha **#31**, e la cura è il blocco **B** |
| 8 | il comando della consegna per le righe della §8 portava un **segnaposto** e due numeri di riga — `sed -n '3565,3635p'` più `{…}` — cioè la forma che la stessa consegna vietava una cella più in là | sostituito dal blocco **A**, delimitato per intestazione |
| 9 | il blocco della §6 da archiviare è **cresciuto lo stesso giorno**, da 38 307 a 38 713 byte, perché la riga che la consegna vi aggiunse **è dentro il blocco** (blocco **D**); e il margine sotto il tetto è sceso da 2750 a 2147 byte (blocco **E**) | un numero misurato prima del commit che lo muove è falso al commit dopo — anche dentro la stessa sessione. Le cifre stanno qui con la data, e si rifanno |
| 10 | `HANDOFF.md` **non porta** lo stato «brainstorming fatto, disegno da scrivere»: `grep -nE 'consegna\|brainstorming della chiusura\|chiusura-design' docs/HANDOFF.md` non rende nessuna riga di stato | il puntatore vive nella sola §6 del compendio, come deve; riscrivere questo file ha chiesto **una** riga lì e nessuna altrove |
| 11 | la condizione **4** della §0.7 è scritta col vocabolario di prima di ADR-0021 — *«il seed come caso di regressione»* — mentre dal 2026-08-08 a diventare regressione è la **proprietà**. È una deduzione, non ancora una misura | la §2.3 la dichiara candidata a «scritta troppo larga»; lo decide il verbale, e la §0.7 resta del proprietario |

---

## Le voci che questo disegno apre per il proprietario

Nessuna è un difetto oggi. Tutte sono **registrate e non prese**.

| | Voce | Chi la chiude |
|---|---|---|
| 1 | il brainstorming del **riconoscimento gesti** dalla telecamera: le decisioni di kernel elencate in testa a questo disegno | il proprietario, dopo la chiusura e prima del sotto-progetto 2; candidata la skill `anthropic-skills:decision-map` |
| 2 | il brainstorming della **knowledge base** mista `.md`, grafi, vettori, con file di routing e sezioni macro/micro: se pretenda una specie di record o un meccanismo che «non si aggiunge dopo», o sia tutta L2 nel sotto-progetto 6 | il proprietario, stesso momento, brainstorming **distinto** |
| 3 | le «sedi da assegnare» che la rilettura di `tracciabilita.md` lascerà — e **una è già nota**: la ritenzione a livelli, che `V26` dichiara senza sede e la roadmap non colloca | il proprietario, nella roadmap; il compito 2 le registra nella tabella del verbale |
| 4 | la consolidazione delle voci aperte della §6 del compendio | già registrata, del proprietario, una per una. **Non** è di questa chiusura |
| 5 | se la condizione 4 della §0.7 vada riallineata ad ADR-0021 — la proprietà e non il seme — nel caso in cui il verbale la trovi scritta troppo larga | il proprietario: la §0.7 è spec, vincolo globale 7 |

---

## Vicoli ciechi e scelte scartate, col perché

| Scartata | Perché |
|---|---|
| una colonna «nel codice» in `tracciabilita.md` | 71 celle nuove, seconda casa dei puntatori che il registro tiene già: marciscono |
| archiviare anche il blocco «voci aperte e racconto del Traguardo 5» | il compendio dice che riassumere una decisione del proprietario può perderne una in silenzio: è la sua consolidazione, una per una |
| marcare «chiuso» nella spec, §0.7 | la §0.7 è una definizione; lo stato ha già le sue case. Toccare la spec è del proprietario, vincolo globale 7 |
| rilanciare tutte le mutazioni di livello 2 per la condizione 2 | la condizione parla di controlli **statici**: livello 1 e i quattro script. I test di livello 2 stanno sotto la condizione 1 |
| eseguire i compiti 1, 2 e 3 in parallelo | toccano file disgiunti e si potrebbe; scelta la sequenza per un ledger solo e le revisioni fra uno e l'altro, come ogni traguardo |
| scrivere il disegno nella stessa sessione del brainstorming | scelta del proprietario: lo ha scritto la sessione successiva, da una consegna in un file **tracciato** — il ledger di `.superpowers/` è ignorato da git e non raggiunge la seconda macchina (voce `E43` del piano del Traguardo 6) |
| correggere a mano la cifra «37 su 37» e basta | il numero vive qui con la data e col comando che lo rifà; correggerlo senza il comando lo rimetterebbe nello stato in cui la sola difesa è rileggere |

---

## Le trappole che mordono scrivendo il piano

| | Trappola | Cura |
|---|---|---|
| 1 | `check-docs.sh` confronta ogni «cifra ADR» nei documenti di stato con il totale reale | i numeri piccoli si scrivono **a parole**; gli esempi nei code span, su una riga |
| 2 | il controllo dei duplicati legge `^#{2,6} <numero>` per file | sotto-sotto-sezioni con `####`, mai due intestazioni con lo stesso numero |
| 3 | il controllo dei link legge ogni `.md` linkato, e **non distingue un esempio da un rimando** | un link di esempio si scrive a parole |
| 4 | il compendio ha un tetto in byte nel cancello, e il margine è di poche migliaia di byte — il blocco **E** lo dà | prima di scrivere in §6 si chiede dove va: un verbale va in `archivio/` |
| 5 | i fine-riga sono misti **per file** | Python con `newline=''`, temporaneo, `os.replace`; `tr -cd '\r' \| wc -c` prima e dopo |
| 6 | un `E<n>` è unico dentro **un** piano | nel sorgente e nei documenti si nomina il piano insieme al codice |
| 7 | il marcatore del prossimo passo vive in un posto solo, la §6 del compendio | nessun altro documento lo porta come puntatore |
| 8 | il pre-controllo ha trovato un difetto reale in **tutti** i compiti dispacciati finora | le quattro domande, per ogni compito, prima di dispacciarlo |
| 9 | una cifra scritta in chat e approvata può essere falsa lo stesso giorno: è successo alla «37 su 37» | ogni cifra del piano porta il comando accanto, e chi esegue lo rilancia prima di fidarsi |

---

## Il prossimo passo

✅ **Il disegno è completo.** Ciò che viene dopo è nell'ordine in cui il proprietario lo ha
scelto, e nessuna voce aperta lo sbarra: la colonna «chi la chiude» qui sopra non nomina questa
chiusura in nessuna riga.

| # | Passo | Dove |
|---|---|---|
| 1 | il proprietario rilegge **questo** disegno; ciò che vuole cambiare torna a lui, e il merito delle sei sezioni non si muove senza di lui | questo file |
| 2 | `superpowers:writing-plans`: il piano in `docs/superpowers/plans/<data>-sottoprogetto-1-chiusura.md`, **quattro** compiti nell'ordine della §5.4 — audit · tracciabilità · archivio e tetto · passaggio unico. In testa: modalità subagent-driven, errata, pre-controllo | la sessione che scrive il piano |
| 3 | il pre-controllo delle quattro domande su ciascun compito, nella sessione che scrive il piano | [`CLAUDE.md`](../../../CLAUDE.md), «Prima di eseguire un compito di un piano» |
| 4 | l'esecuzione in una sessione **nuova**, un subagente fresco per compito, revisione fra uno e l'altro | `superpowers:subagent-driven-development` |
| 5 | quando il compito 4 è committato e pushato: la §6 del compendio porta il prossimo passo, cioè i due brainstorming, e questo disegno porta la §7 datata e chiusa | la §5.2 lo prescrive |

✅ **RICHIAMO DEL 2026-09-02: i passi 1, 2 e 3 sono fatti lo stesso giorno** — il proprietario ha
riletto il disegno, il piano vive in
[`plans/2026-09-02-sottoprogetto-1-chiusura.md`](../plans/2026-09-02-sottoprogetto-1-chiusura.md)
e il pre-controllo delle quattro domande sta nella sua errata. Ciò che resta è l'esecuzione, in
una sessione **nuova**; a che punto sia lo dice la §6 del compendio, non questa riga.

📌 **Ciò che questo disegno consegna a chi scriverà il piano**, che è suo e non un puntatore: i
quattro compiti ordinati dalla §5.4, con ciò che ciascun commit può dire; la Definizione di
«fatto» della §6.3; i comandi della §1.3, da rilanciare e non da citare; e il caso `V26`, che il
compito 2 incontra per primo.

⛔ **Che cosa venga dopo la chiusura NON è scritto qui col marcatore, ed è deliberato:** è uno
**stato**, e la sua casa unica è la §6 del [compendio](../../COMPENDIO.md). La scelta 3 del
proprietario dice che cosa vi andrà scritto; a scriverlo sarà il compito 4.
