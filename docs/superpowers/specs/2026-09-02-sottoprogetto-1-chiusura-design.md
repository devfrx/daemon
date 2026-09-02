# Chiusura del sotto-progetto 1 — la consegna del brainstorming, 2026-09-02

> ⚠️ **Che cos'è questo file, e che cosa NON è.** È la **consegna** del brainstorming della
> chiusura del sotto-progetto 1, fatto in chat il 2026-09-02 col proprietario: sei sezioni
> presentate una per volta e **tutte approvate**. ⛔ **Non è ancora il disegno.** Il
> proprietario ha scelto di lasciare la scrittura del disegno alla sessione successiva.
> Questo file sta **già al percorso del disegno**, e la sessione che lo riprende lo
> **riscrive sul posto** in forma di disegno — così il puntatore della §6 del
> [compendio](../../COMPENDIO.md) non cambia mai casa, e il ciclo della consegna si chiude
> da sé: quando questo riquadro sparisce, il disegno esiste.
>
> 📌 **Perché è un file tracciato e non il ledger:** si lavora da **due** macchine, e il
> ledger di `.superpowers/` è ignorato da git — la voce `E43` del piano del Traguardo 6 lo
> ha già misurato. Un'istruzione lasciata lì non raggiunge l'altra macchina.

---

## 0. Stato verificato coi comandi, il 2026-09-02

Nessun codice è stato toccato in questa sessione. L'ultimo commit di **merito** è quello
della chiusura del Traguardo 6; questo file e una riga nella §6 del compendio sono l'unica
aggiunta.

| | Misurato | Comando |
|---|---|---|
| ramo | `spec/sottoprogetto-1-kernel`, allineato a `origin`, zero avanti e zero dietro **prima** del commit che porta questo file | `git status -sb` · `git rev-list --left-right --count origin/spec/sottoprogetto-1-kernel...HEAD` |
| albero | pulito, nessuno stash, nessun'operazione a metà | `git status` · `git stash list` |
| commit di merito | `49b302d` — la seconda ondata del compito 10, Traguardo 6 chiuso | `git log --oneline -1 49b302d` |
| cancello | `GATE GREEN`, exit 0, sul commit di merito | `bash scripts/gate.sh` |
| coerenza dei documenti | `OK` | `bash scripts/check-docs.sh` |
| tetto del compendio | il numero sta in `scripts/check-docs.sh`; il margine lo dà il comando | `echo $(( $(grep -oE '^ceiling=[0-9]+' scripts/check-docs.sh \| cut -d= -f2) - $(wc -c < docs/COMPENDIO.md) ))` |
| file ignorati nell'albero | `crates/kernel/wip/`, `spikes/rust/wip/`, i due `Cargo.lock` degli spike: **preesistenti**, non di questa sessione | `git status --porcelain --ignored` |

⚠️ **Nessun pericolo lasciato indietro:** nessuna migrazione, nessun servizio, nessuna
credenziale, nessun rebase.

---

## 1. Le decisioni prese in chat dal proprietario

| # | Domanda | Scelta |
|---|---|---|
| 1 | che cosa vuol dire «aggiornare `tracciabilita.md` alla chiusura» | **A** — si rilegge ogni riga con la seconda domanda; cambiano solo le righe il cui meccanismo è scaglionato, che prendono la forma 🔶 già in legenda. Nessuna colonna «nel codice»: sarebbe una seconda casa di puntatori che il registro tiene già |
| 2 | il racconto del Traguardo 6 nella §6 del compendio | **A** — la chiusura lo archivia in `archivio/stato-storico.md` parola per parola, con la cernita in due direzioni, e il tetto scende nello stesso commit. Il blocco «voci aperte e racconto del Traguardo 5» **non** si tocca: è la consolidazione del proprietario |
| 3 | il prossimo passo che la §6 scrive dopo la chiusura | **A** — il brainstorming delle **due funzioni core**, prima di aprire il sotto-progetto 2. ⛔ **Con la correzione del proprietario:** l'*Agentic OS* **è il programma intero**, non una funzione da aggiungere. Le aggiunte sono **due cose diverse**: il **riconoscimento gesti** dalla telecamera, e la **knowledge base**. Due brainstorming distinti |
| 4 | come si esegue la chiusura | **proposta 1** — un piano piccolo di **quattro compiti**, un subagente fresco per compito con revisione fra uno e l'altro; pre-controllo di ogni compito in una sessione, esecuzione in una sessione **nuova** |
| 5 | la strada | quella pesante: brainstorming → disegno scritto → piano → esecuzione. Come ogni traguardo |

📌 **Perché il brainstorming delle due funzioni viene DOPO la chiusura e PRIMA del sotto-progetto
2**, verificato col `grep` e non a memoria: la telecamera non è mai stata valutata in nessun
documento del repo; il kernel ha già gli slot che le servono — ADR-0011 tratta la percezione
always-on come **sorgente di eventi** (anello 3), ADR-0005 ha la quota audio **sottratta**,
ADR-0023 il profilo «riservato», ADR-0028 i worker in Python, ADR-0014 il confine dei dati non
fidati; la knowledge base ha già una sede in roadmap, il sotto-progetto 6. Le decisioni di
**kernel** sono piccole e in append: una terza quota nella formula di ADR-0005, il «riservato»
di ADR-0023 esteso alla telecamera, quinto pilastro o parte di Voce in ADR-0001, se il canale
`process` regge uno stream. La **capacità** si progetta quando si apre il suo sotto-progetto:
il compendio vieta di progettare una capacità L2 prima. E la GUI del sotto-progetto 2 deve
ancora fare le misure M1–M5 di ADR-0029: un flusso video nella webview è una misura in più,
meglio saperlo prima di disegnarla.

---

## 2. Le sei sezioni approvate

⛔ **Sono riportate come approvate**, e la sessione successiva le riscrive in forma di disegno
senza cambiarne il merito. Ciò che vuole cambiare torna al proprietario.

### 2.1 Sezione 1 — il perimetro ✅ approvata

**Che cosa la chiusura fa**

| # | Cosa | Dove finisce |
|---|---|---|
| 1 | rilegge le condizioni 1–5 della §0.7 della [spec](2026-08-06-sottoprogetto-1-kernel.md) **contro il codice di oggi**, coi comandi e non a memoria | il verbale, nell'ultima sezione del disegno della chiusura. Precedente: la §8 del [disegno del Traguardo 6](2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) |
| 2 | rilegge [`tracciabilita.md`](../../tracciabilita.md) con la seconda domanda. La fonte di «costruito o scaglionato» è la tabella §0.4 della spec, colonne «Entra» e «Si scaglia» | le righe il cui meccanismo è scaglionato prendono la forma 🔶 col sotto-progetto che lo porta. Il segnaposto in testa diventa lo stato della chiusura |
| 3 | archivia il racconto del Traguardo 6 dalla §6 del compendio, parola per parola, con la cernita in due direzioni | [`archivio/stato-storico.md`](../../archivio/stato-storico.md), e il tetto in `scripts/check-docs.sh` scende con la regola di sempre: misurato più circa l'undici per cento, arrotondato al KiB |
| 4 | aggiorna **nello stesso passaggio** i documenti di stato: roadmap, README, HANDOFF, AVVIO-CHAT, compendio, e il richiamo in `semi-dst.md` per le due campagne del Traguardo 6 | un commit solo. Le righe precise stanno nella sezione 5 |
| 5 | `check-docs.sh` verde, cancello verde, commit e push | condizione 7 della §0.7 |

**Che cosa la chiusura NON fa**

| | Perché |
|---|---|
| non tocca la spec | la §0.7 è una definizione, non uno stato. Lo stato ha già le sue case, e la spec è del proprietario |
| non chiude le voci aperte del proprietario | le dichiara. La consolidazione resta una voce a sé, da presentare una per una |
| non riordina la roadmap | il prossimo passo scritto in §6 è il brainstorming delle due funzioni core. Il riordino, se serve, esce da lì |
| non archivia il blocco «voci aperte e racconto del Traguardo 5» | è la consolidazione del proprietario, e il compendio lo dice |
| non tocca `riferimenti.md` | la voce E146 del piano del Traguardo 5 resta registrata e non presa |
| non rilancia tutte le mutazioni | la condizione 2 si prova leggendo l'evidenza già registrata: lo `.stderr` per il livello 1, la mutazione nel registro per il livello 2. Si rilancia solo dove manca, o dove il file è cambiato dopo la misura |
| non crea righe di catalogo, e non marca ✅ nessuna riga ⚠️ o ⏳ della §8 | §7.4 e §8 sono spec. Chiudere troppo è un modo di fallire, come la condizione 12 del Traguardo 6 |

**Il perimetro in numeri, misurati il 2026-09-02**

| | | Comando |
|---|---|---|
| righe della §8 della spec da rileggere, le ✅ e le ⚠️ | 45: 30 V e 15 Q | `sed -n '3565,3635p' <spec> \| awk -F'\|' '/^\| \*?\*?[VQ][0-9]+/{…}'` — delimitare per **intestazione** di §8.3 e §8.4, non per numero di riga |
| righe di `tracciabilita.md` con uno stato | 171, di cui 71 ✅ e 25 🔶 | `grep -cE '^\| .* \| (✅\|🔶\|📋\|⚠️\|❌) \|' docs/tracciabilita.md` |
| byte da archiviare dalla §6 | 38 307, con 2750 di margine quel giorno | `sed -n '/^### Il prossimo passo/,/^✅ \*\*I finding dell/p' docs/COMPENDIO.md \| head -n -1 \| wc -c` |
| ADR della §0.5 | sette, e ognuno ha la sezione `Negative (accettate)` | `grep -c 'Negative (accettate)' docs/adr/003[1-7]-*.md` |
| campagne DST | cinque, di cui due nuove col Traguardo 6. `semi-dst.md` ne dichiara tre | `ls crates/simulator/tests/*campaign*.rs crates/platform/tests/engine_crash_consistency.rs` |
| spike chiusi dal sotto-progetto 1 | nessuno. SP-1 e SP-2 li chiudono i sotto-progetti 7 e 8 | tabella «Spike aperti» di [`roadmap.md`](../../roadmap.md) |

### 2.2 Sezione 2 — le sette condizioni della §0.7 ✅ approvata

**Tre regole di lettura**

| | Regola |
|---|---|
| a | le condizioni 1 e 3 si leggono sulle sole righe ✅ e sulla metà verificata delle ⚠️ della §8. Una riga ⏳ non le viola: lo dice il rimando della §0.7 |
| b | «controllo statico» nella condizione 2 vuol dire: i casi di livello 1, cioè il compilatore, e i quattro script che leggono sorgenti, manifesti e documenti senza eseguire nulla — `gate-no-os.sh`, `gate-deps.sh`, `gate-attributes.sh`, `check-docs.sh`. I test di livello 2 sono dinamici e stanno sotto la condizione 1 |
| c | prima domanda: «è già fatto?». Ogni conteggio si rifà col comando prima di leggere che cosa i documenti ne dicono. Gotcha #49 |

**Le condizioni**

| # | Condizione | Che cosa conta come evidenza | Come si prova | Già misurato il 2026-09-02 |
|---|---|---|---|---|
| 1 | ogni V in perimetro ha un controllo che gira in automatico | la cella «meccanismo» di §8.3 nomina una delle tre risposte ammesse da §8.1.2: una riga del catalogo §7.4, un test a esempi, la campagna DST con la sua porta. E quella cosa **esiste** | per ogni riga: la riga di catalogo si cerca nel [registro](../../porta-di-qualita.md), che dice il file; il test con `grep -rn 'fn <nome>'`; la campagna col file. È la classe di AUD-026 ricensita su tutte le righe | 30 righe. Il cancello è verde |
| 2 | ogni controllo statico è stato visto fallire, e poi tornare verde | livello 1: ogni caso ha il suo `.stderr` non vuoto e la suite è verde. Script: la direzione «deve scattare» **rilanciata**, perché le loro prove stanno sparse fra piano del Traguardo 1, audit e spec §8.6.3, e gli script sono cambiati dopo | `ls *.rs` contro `ls *.stderr` in `crates/kernel/tests/compile_fail/`, e nessuno vuoto. Per ogni script una violazione deliberata, il rosso, la revoca con `git diff` a zero | 40 casi, 40 `.stderr`, 0 vuoti. Gli script: da rilanciare, quattro prove |
| 3 | ogni Q in perimetro è verificato col metodo di `design/08` | la cella di §8.4 nomina il metodo che la tabella «Mappa requisito → metodo di verifica» di [`design/08`](../../design/08-strategia-di-test.md) assegna a quel Q, non un altro | confronto riga per riga, 15 righe | da fare |
| 4 | ogni difetto trovato in simulazione conserva il seme (V31) | l'elenco di [`semi-dst.md`](../../semi-dst.md) vuoto **e dichiarato**, il cancello verde su tutte le campagne, e la riga V31 di §8.3 già a ✅ «debole per natura» | `bash scripts/gate.sh`, poi il richiamo datato in `semi-dst.md` per le due campagne del Traguardo 6, entrambe a livello 1, con le costanti che rendono leggibile una loro voce: `SHORT_CAMPAIGN_SEEDS` ed `EXPECTED_WORLDS` in `gui_death_campaign.rs`, `SHORT_CAMPAIGN_SEEDS` e `KILL_HORIZON` in `worker_kill_campaign.rs` | verde. Il richiamo manca |
| 5 | gli ADR della §0.5 sono scritti, ciascuno con le `Negative (accettate)` | sette file in `docs/adr/`, ciascuno con l'intestazione esatta; la riga 3 della §0.5 vive in §2.4 senza ADR, ed è dichiarato | `ls docs/adr/003[1-7]-*.md` e `grep -c 'Negative (accettate)'` su ciascuno | sette su sette, e l'intestazione c'è in tutti |
| 6 | roadmap, tracciabilità, spike e HANDOFF aggiornati nello stesso passaggio | **un commit** che li tocca tutti, più compendio, README e AVVIO-CHAT che [`CLAUDE.md`](../../../CLAUDE.md) aggiunge | `git show --stat HEAD` li nomina tutti. Le righe le elenca la sezione 5 | da fare |
| 7 | `check-docs.sh` verde | l'uscita `OK` | `bash scripts/check-docs.sh`, e con esso il cancello | verde |

**Che cosa il verbale scrive per ogni condizione.** Una riga per condizione, come la §8.2 del
disegno del Traguardo 6: ✅ con l'evidenza letta o rilanciata quel giorno, oppure ❌ con che cosa
manca. E una tabella «dove la condizione era scritta troppo larga», se capita: è successo alla 4
del Traguardo 4.

### 2.3 Sezione 3 — la rilettura di `tracciabilita.md` ✅ approvata

**La domanda, e le fonti che rispondono.** Per ogni riga: «quale meccanismo del kernel le
serve, e il codice lo dà oggi o è scaglionato?». Nessuna risposta a memoria: tre fonti, in
quest'ordine.

| # | Fonte | Che cosa dice |
|---|---|---|
| 1 | la tabella §0.4 della spec, colonne «Entra» e «Si scaglia» | se il meccanismo è costruito nel sotto-progetto 1 o rinviato |
| 2 | la colonna «Innesco» di §8.3 e §8.4 | per un meccanismo rinviato, il **sotto-progetto** che lo porta: la lettera è la condizione, il numero fra parentesi il sotto-progetto. Il 2026-09-02 tutte e 37 le righe ⏳ e ⚠️ ne portano uno: A→2, B→3, C→4, D→5, E→7, F→11, SP-2→8 |
| 3 | la tabella dei sotto-progetti di `roadmap.md` | il nome del sotto-progetto, per scriverlo nella cella |

**Che cosa cambia, e che cosa no**

| Riga | Esito |
|---|---|
| ✅ il cui meccanismo è in «Entra» ed esiste in `crates/` | **resta** com'è. La sede resta spec e ADR: nessun puntatore al codice |
| ✅ il cui meccanismo è in «Si scaglia» | diventa **🔶**, con «implementazione → sotto-progetto N» preso dall'innesco. È la forma che la legenda ha già |
| ✅ scaglionato ma senza sotto-progetto in nessuna delle tre fonti | diventa 🔶 con «sede da assegnare», e la sede mancante è una **voce aperta del proprietario** nella tabella della chiusura. Non si inventa |
| 🔶 già oggi | si rilegge, cambia solo se la sede è sbagliata |
| 📋 e ❌ | non dipendono dal sotto-progetto 1: intatte |
| una funzionalità il cui meccanismo **nessuno ha deciso** | riga nuova nella sezione «Lacune», come L-1…L-5. La sezione lo prescrive già |

La legenda non cambia. Il criterio di «esiste in `crates/`»: `ls crates/kernel/src/` dà i
moduli, e un meccanismo costruito ne nomina uno.

**Il segnaposto in testa.** Il riquadro «la tabella si aggiorna alla chiusura del
sotto-progetto, non ora» diventa lo stato della chiusura: la data, la domanda con cui si è
riletta, e il **comando** che conta le righe per stato, non le cifre. L'elenco delle righe
cambiate, con la ragione di ciascuna, vive nel verbale della chiusura e il riquadro vi rimanda:
una casa sola.

**La prova, in due direzioni**

| Direzione | Come |
|---|---|
| ogni riga passata a 🔶 doveva cambiare | il meccanismo che nomina compare in «Si scaglia» di §0.4 o in un innesco di §8, col `grep` |
| ogni riga rimasta ✅ doveva restare | nel verbale le ✅ si **raggruppano per meccanismo**, circa dieci gruppi, e ogni gruppo nomina il modulo che esiste. Settantuno righe, non settantuno frasi |
| il revisore | rifà i conteggi coi comandi, e **ri-deriva da solo** l'insieme delle righe cambiate contro §0.4, prima di leggere il verbale |

**I numeri del 2026-09-02:** 71 righe ✅; circa 26 candidate a 🔶 per euristica sulla colonna
«Sede» — il numero vero lo dà la rilettura, e vince lui; 37 righe su 37 con una sede già scritta
nell'innesco di §8.

### 2.4 Sezione 4 — l'archiviazione del racconto del Traguardo 6, e il tetto ✅ approvata

**Che cosa esce dalla §6.** Il blocco che va da `### Il prossimo passo` fino alla riga
**prima** di `✅ **I finding dell'audit sono chiusi**`, delimitato coi marcatori e non coi
numeri di riga. Circa 38 KB, **parola per parola**. Tre eccezioni restano in §6, dichiarate
nell'archivio:

| Resta | Perché |
|---|---|
| la riga `⏭️` del prossimo passo | è il puntatore vivo. La sezione 5 la riscrive. Un `⏭️` nell'archivio sarebbe un secondo puntatore, morto |
| il blocco «i finding dell'audit sono chiusi», col suo `awk` | è vero adesso, ed è la casa dello stato dell'audit col comando |
| una riga che sia una **regola** per i piani futuri senza altra casa | su un dubbio si conserva. Il compito le elenca nel verbale |

⚠️ **Dal 2026-09-02 il blocco contiene anche la riga che questa consegna aggiunge alla §6** —
«il brainstorming della chiusura è fatto». È un verbale come il resto: parte con il blocco.

**Che cosa NON esce.** La testa della §6, il blocco «voci ancora aperte e racconto del
Traguardo 5», la tabella dei traguardi, la coda.

**La cernita, in due direzioni**

| Direzione | Come |
|---|---|
| ogni voce che il blocco chiama aperta ha un'altra casa | marcatori: «aperta», «non presa», «del proprietario», «registrata», i codici `E<n>`. Per ciascuna, `grep` nelle due tabelle uniche del registro e nella tabella X dell'[audit](../../audit-2026-08-27.md). Trovata: il blocco parte. Non trovata: il blocco **resta**, e la voce entra nella tabella della chiusura |
| ciò che parte è ripassato con marcatori **diversi** | tenendo tutto ciò che pescano. È il precedente del 2026-09-01 |

⚠️ Il 2026-09-02 il blocco nominava 58 codici `E<n>` distinti e ognuno ha una riga nell'errata
del [piano del Traguardo 6](../plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md)
— ⛔ ma un numero d'errata **non è un riferimento** fuori dal proprio piano: la cernita si fa
per voce, leggendo, non per codice.

**Dove finisce, e in che forma.** Una sezione nuova in `archivio/stato-storico.md`: titolo
`## Il racconto del Traguardo 6 — archiviato il <data>`, il riquadro «Verbale, non stato», il
perché in due righe, il criterio di taglio «resta in §6 ciò che è vero adesso», le tre
eccezioni, poi il blocco verbatim. I link relativi che contiene — sei il 2026-09-02 —
prendono il prefisso `../`, e `check-docs.sh` lo verifica.

**Il puntatore che resta in §6.** Un blocco corto, come quello del 2026-09-01: che cosa è
uscito, dove sta, la prova della cernita, e «nulla riassunto, nulla cancellato».

**Il tetto.** Scende nello **stesso commit** dell'archiviazione, con la regola scritta nello
script: misurato più circa l'undici per cento, arrotondato al KiB. Il numero lo dà il comando,
e va nel richiamo datato del blocco di commento di `scripts/check-docs.sh`:

```bash
size=$(wc -c < docs/COMPENDIO.md); echo $(( (size * 111 / 100 + 1023) / 1024 * 1024 ))
```

L'undici per cento copre le righe corte che la sezione 5 aggiunge dopo.

**Fine-riga**

| File | Nell'albero | Regola |
|---|---|---|
| `docs/COMPENDIO.md` | CRLF, un CR per riga | si scrive in Python con `newline=''`, temporaneo e `os.replace`. Il CR dopo deve fare il numero di righe |
| `docs/archivio/stato-storico.md` | LF, zero CR | il blocco entra in LF: zero CR dopo |
| `scripts/check-docs.sh` | CRLF | stesso metodo del compendio |

**Le prove.** `check-docs.sh` verde, quindi link e tetto · `git diff --stat` nomina esattamente
**tre** file · `git ls-files --eol` invariato · il cancello verde.

### 2.5 Sezione 5 — il passaggio unico sui documenti di stato ✅ approvata

⛔ **Trovato preparando questa sezione:** il piano e il disegno del Traguardo 6 non compaiono in
**nessun** indice — né nella tabella dei piani della roadmap, né nella tabella delle spec del
README, né nella mappa dei documenti di HANDOFF, né nella §12 del compendio. Dal 2026-08-30. È
la classe del finding AUD-040, e il passaggio unico la chiude.

**Che cosa cambia, file per file.** Ogni riga si ritrova col `grep` sulla frase, mai col
numero di riga.

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
| `COMPENDIO.md` | §6, la riga `⏭️` | «✅ il sotto-progetto 1 è chiuso il <data> contro la §0.7, verbale nel disegno della chiusura. ⏭️ il prossimo passo: il brainstorming delle **due funzioni core**, gesti dalla telecamera e knowledge base, due brainstorming distinti, prima di aprire il sotto-progetto 2. Nessuna voce aperta lo sbarra» | corto: il tetto |
| `COMPENDIO.md` | §6, dentro il blocco del Traguardo 5: «⛔ Il sotto-progetto 1 NON è chiuso: restano il Traguardo 6 e la §8 di tracciabilita.md» | resta, con un richiamo datato di una riga. Il blocco è tenuto parola per parola, e una riga falsa non può restare muta | richiamo datato |
| `COMPENDIO.md` | §6, «Il ritratto pieno … diciotto ✅ · tredici ⚠️ · sei ⏳ … nove · otto · sette» | le cifre **escono**, resta il comando che le riconta e «si riconta ogni volta». Il 2026-09-02 danno 16·14·7 e 8·7·9: il ritratto era già stantio | gotcha #31 |
| `COMPENDIO.md` | §8, «si aggiorna, e solo alla chiusura del sotto-progetto 1 — quindi non ora», con «centosettantuno» | «riletta alla chiusura del sotto-progetto 1 il <data>; si riaggiorna a ogni sotto-progetto chiuso». La cifra esce e rimanda al comando in testa a `tracciabilita.md` | sottrazione |
| `COMPENDIO.md` | §12, ferma ai disegni e piani del Traguardo 5 | una riga per disegno e piano del Traguardo 6, una per quelli della chiusura | AUD-040 |
| `tracciabilita.md` | il riquadro «la tabella si aggiorna alla chiusura del sotto-progetto, non ora» | lo stato della chiusura, come da sezione 3 | |
| `semi-dst.md` | l'ultimo richiamo, del 2026-08-25, «le campagne sono tre» | un richiamo datato in più: due campagne col Traguardo 6, entrambe a livello 1, con le costanti che rendono leggibile una voce; l'elenco non guadagna righe, e il perché | il precedente del Traguardo 5 |
| il disegno della chiusura | la sezione del verbale, in bozza dal compito 1 | datata e chiusa | |

**Che cosa NON si tocca.** La spec del sotto-progetto 1 · `porta-di-qualita.md`, salvo che
l'audit vi trovi una riga falsa · `riferimenti.md` · `CLAUDE.md`: il modo di lavorare non cambia ·
l'audit del 2026-08-27 · nessun numerale nuovo in prosa, mai: date, comandi e rimandi.

**L'ordine dei commit**

| Compito | Commit | Che cosa può dire |
|---|---|---|
| 1, audit | il verbale in bozza nel disegno | «le condizioni rilette», mai «chiuso» |
| 2, tracciabilità | le righe passate a 🔶 | vere a prescindere dalla chiusura |
| 3, archivio | il blocco, il puntatore, il tetto | «archiviato perché il Traguardo 6 è chiuso e il tetto», mai «sotto-progetto chiuso» |
| 4, il passaggio | **tutto il resto della tabella in un commit solo**, poi il push | è l'unico che dice «chiuso» |

Così fra un commit e l'altro i documenti non si contraddicono mai.

**Le prove.** `git show --stat HEAD` nomina tutti i file della tabella · `grep -rn 'NON è
chiuso\|è in corso' docs/*.md` non rende più nessuna riga viva sul sotto-progetto 1, e le righe
che restano stanno dentro richiami datati · `check-docs.sh` verde, cancello verde · fine-riga
misurati file per file.

### 2.6 Sezione 6 — chi verifica, come, e la Definizione di «fatto» della chiusura ✅ approvata

**Chi fa che cosa**

| Ruolo | Chi | Fa |
|---|---|---|
| coordinatore | la sessione che scrive il piano, poi quella che lo esegue | il pre-controllo delle quattro domande di `CLAUDE.md` su ogni compito **prima** di dispacciarlo, il ledger, i ruling, e la seconda ondata di sola prosa a mano |
| esecutore | un subagente fresco per compito | esegue il compito; si ferma e riporta se il compito dice il falso, non aggira |
| revisore | un subagente fresco per compito | rilancia **ogni comando** accanto a un'affermazione misurabile e li elenca; per la tracciabilità ri-deriva da solo l'insieme delle righe cambiate prima di leggere il verbale |
| proprietario | | approva il disegno e il piano, decide le voci aperte che la chiusura registra |

Il pre-controllo si fa in una sessione; l'esecuzione in una sessione **nuova**, come al
compito 10 del Traguardo 6.

**Le regole per chi chiude**

| # | Regola |
|---|---|
| 1 | si parte dai numeri: ogni conteggio si rifà col comando prima di leggere che cosa i documenti ne dicono |
| 2 | la prima domanda è «è già fatto?», non «come lo faccio?». Ciò che si trova eseguito si riconosce e si scrive che lo era |
| 3 | la §0.7 si rilegge **contro il codice**, mai contro sé stessa: una condizione può risultare scritta troppo larga, e si dice |
| 4 | un rimedio si chiude su tutte le case della frase, non su quella dove lo si è trovato |
| 5 | si **toglie** invece di riscrivere meglio: meno parole, meno superficie per una falsità nuova |
| 6 | il verbale sta nell'ultima sezione del disegno, con la tabella «dove il disegno è stato smentito dall'esecuzione» e «che cosa la chiusura NON ha fatto» |

**La Definizione di «fatto» della chiusura.** Nessuna condizione porta un numerale che il
lavoro può muovere: nomina la sezione, e il conteggio lo fa chi chiude.

| # | Condizione |
|---|---|
| 1 | `bash scripts/gate.sh` → `GATE GREEN`, `check-docs.sh` → `OK`, baseline **rimisurata** col comando |
| 2 | le condizioni 1–5 della §0.7 hanno ciascuna la propria riga nel verbale, con l'evidenza **letta o rilanciata quel giorno**: per i quattro script il rosso rilanciato e revocato con `git diff` a zero |
| 3 | `tracciabilita.md`: ogni riga ✅ poggia su un meccanismo in «Entra» che nomina un modulo esistente, raggruppato nel verbale; ogni riga 🔶 nomina un sotto-progetto o «sede da assegnare» registrata come voce aperta; il riquadro in testa porta la data, la domanda e il comando; la legenda è intatta |
| 4 | il racconto del Traguardo 6 sta in `archivio/stato-storico.md` parola per parola con le tre eccezioni dichiarate, i link riscritti, la cernita provata in due direzioni, e il tetto sceso **nello stesso commit** col richiamo datato |
| 5 | il commit del passaggio unico tocca **tutti** i file della tabella della sezione 5, e il disegno e il piano del Traguardo 6 e della chiusura stanno nei quattro indici |
| 6 | le voci che la chiusura apre stanno in **una** tabella del disegno, con la colonna «chi la chiude», e nessuna è decisa al posto del proprietario |
| 7 | ⛔ **negativa**: la spec non è toccata, `git diff --name-only <base>..HEAD -- docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` non rende niente; nessuna riga ⚠️ o ⏳ della §8 passa a ✅; nessuna cifra nuova in prosa; `riferimenti.md` intatto |
| 8 | ogni compito ha avuto la sua revisione, e nessuna revisione ha lasciato un Important aperto |

**Le ondate.** Una revisione per compito. Se una seconda ondata apre rilievi di **sola prosa**,
la fa il coordinatore a mano e senza ri-revisione: è la regola che il proprietario ha fissato
il 2026-09-02 al compito 8 del Traguardo 6. Se tocca un fatto o un comando, si ri-rivede.

---

## 3. Che cosa il brainstorming ha misurato, e non era scritto da nessuna parte

| | Misurato il 2026-09-02 | Che cosa ne segue |
|---|---|---|
| il piano e il disegno del Traguardo 6 **non stanno in nessun indice**: roadmap, README, HANDOFF, §12 del compendio | `grep -c traguardo-6` sui quattro file: zero, zero, zero, e nel compendio le due occorrenze sono in §6 | il passaggio unico li aggiunge, insieme a quelli della chiusura |
| «Il ritratto pieno» della §6 del compendio è **stantio**: dice diciotto·tredici·sei e nove·otto·sette | le terze colonne di §8.3 e §8.4 danno 16·14·7 e 8·7·9 | le cifre escono, resta il comando |
| `semi-dst.md` dichiara tre campagne | ne esistono cinque: due nate col Traguardo 6, entrambe a livello 1 | un richiamo datato, come quello del 2026-08-25 |
| la telecamera non è mai stata valutata | `grep -rniE 'telecamera\|camera\|webcam\|mediapipe\|gestur'` su `docs/`, `docs/adr/`, `docs/design/`, `spikes/`: niente di pertinente. L'unica riga vicina è ADR-0011, sulla percezione always-on | è un'idea nuova, non una decisione da riaprire |
| `porta-di-qualita.md` non ha sezioni per gli script del cancello | `grep -nE '^#{2,4} .*gate-'`: niente. Le prove in negativo degli script stanno nel piano del Traguardo 1, nell'audit e nella spec §8.6.3 | per la condizione 2 si **rilancia**, quattro prove |
| i fine-riga dei file che la chiusura tocca | `git ls-files --eol`: CRLF nell'albero per compendio, HANDOFF, README, AVVIO-CHAT, semi-dst, `check-docs.sh`; LF per roadmap, tracciabilità, stato-storico. Nell'indice tutti LF | ogni scrittura conserva i fine-riga del proprio file e li rimisura |
| ogni riga ⏳ e ⚠️ di §8 porta già un sotto-progetto nell'innesco | 37 su 37 | la sede di un meccanismo scaglionato **c'è**, quasi sempre |

---

## 4. Le voci che questo brainstorming apre per il proprietario

| | Voce | Chi la chiude |
|---|---|---|
| 1 | il brainstorming del **riconoscimento gesti** dalla telecamera: le decisioni di kernel elencate nella sezione 1 | il proprietario, dopo la chiusura e prima del sotto-progetto 2; candidata la skill `anthropic-skills:decision-map` |
| 2 | il brainstorming della **knowledge base** mista `.md`, grafi, vettori, con file di routing e sezioni macro/micro: se pretenda una specie di record o un meccanismo che «non si aggiunge dopo», o sia tutta L2 nel sotto-progetto 6 | il proprietario, stesso momento, brainstorming **distinto** |
| 3 | le «sedi da assegnare» che la rilettura di `tracciabilita.md` potrà lasciare | il proprietario, nella roadmap; il compito 2 le registra |
| 4 | la consolidazione delle voci aperte della §6 del compendio | già registrata, del proprietario, una per una. **Non** è di questa chiusura |

---

## 5. Vicoli ciechi e scelte scartate, col perché

| Scartata | Perché |
|---|---|
| una colonna «nel codice» in `tracciabilita.md` | 71 celle nuove, seconda casa dei puntatori che il registro tiene già: marciscono |
| archiviare anche il blocco «voci aperte e racconto del Traguardo 5» | il compendio dice che riassumere una decisione del proprietario può perderne una in silenzio: è la sua consolidazione, una per una |
| marcare «chiuso» nella spec, §0.7 | la §0.7 è una definizione; lo stato ha già le sue case. Toccare la spec è del proprietario, vincolo globale 7 |
| rilanciare tutte le mutazioni di livello 2 per la condizione 2 | la condizione parla di controlli **statici**: livello 1 e i quattro script. I test di livello 2 stanno sotto la condizione 1 |
| eseguire i compiti 1, 2 e 3 in parallelo | toccano file disgiunti e si potrebbe; scelta la sequenza per un ledger solo e le revisioni fra uno e l'altro, come ogni traguardo |
| scrivere il disegno in questa sessione | scelta del proprietario: lo scrive la sessione successiva, da questo file |

---

## 6. Il prossimo passo, eseguibile

| # | Passo | Comando o file |
|---|---|---|
| 1 | in una sessione nuova: `git fetch`, poi la lettura obbligatoria di `CLAUDE.md` e del compendio, per intero | `git fetch --all --prune && git status -sb` |
| 2 | verificare **questo** file contro il repo prima di fidarsene: il commit di merito è ancora `49b302d`? il cancello è verde? | `git log --oneline -3` · `bash scripts/gate.sh` |
| 3 | riscrivere questo file **sul posto** in forma di disegno: intestazione come quella del disegno del Traguardo 6, le sei sezioni della §2 con lo stesso merito, poi le tre sezioni di coda — «cosa questo disegno ha misurato», «le voci che apre», «il prossimo passo» — e togliere il riquadro in testa. Rilettura: segnaposti, contraddizioni, ambiguità, perimetro | questo file |
| 4 | `bash scripts/check-docs.sh`, commit, push | |
| 5 | il proprietario rivede il disegno | |
| 6 | `superpowers:writing-plans`: il piano in `docs/superpowers/plans/<data>-sottoprogetto-1-chiusura.md`, **quattro** compiti nell'ordine della sezione 5: audit · tracciabilità · archivio e tetto · passaggio unico. In testa: modalità subagent-driven, errata, pre-controllo | |
| 7 | il pre-controllo delle quattro domande su ciascun compito, nella sessione che scrive il piano | `CLAUDE.md`, «Prima di eseguire un compito di un piano» |
| 8 | l'esecuzione in una sessione **nuova**, un subagente fresco per compito, revisione fra uno e l'altro | `superpowers:subagent-driven-development` |
| 9 | quando il compito 4 è committato e pushato: aggiornare la §6 del compendio col prossimo passo, cioè i due brainstorming | |

---

## 7. Le trappole che mordono scrivendo il disegno e il piano

| | Trappola | Cura |
|---|---|---|
| 1 | `check-docs.sh` confronta ogni «cifra ADR» nei documenti di stato con il totale reale | i numeri piccoli si scrivono **a parole**; gli esempi nei code span, su una riga |
| 2 | il controllo dei duplicati legge `^#{2,6} <numero>` per file | sotto-sotto-sezioni con `####`, mai due intestazioni con lo stesso numero |
| 3 | il controllo dei link legge ogni `.md` linkato, e **non distingue un esempio da un rimando** | un link di esempio si scrive a parole |
| 4 | il compendio ha un tetto in byte nel cancello, e il margine è di poche migliaia di byte | prima di scrivere in §6 si chiede dove va: un verbale va in `archivio/` |
| 5 | i fine-riga sono misti **per file** | Python con `newline=''`, temporaneo, `os.replace`; `tr -cd '\r' \| wc -c` prima e dopo |
| 6 | un `E<n>` è unico dentro **un** piano | nel sorgente e nei documenti si nomina il piano insieme al codice |
| 7 | il marcatore `⏭️` vive in un posto solo, la §6 del compendio | nessun altro documento lo porta |
| 8 | il pre-controllo ha trovato un difetto reale in **tutti** i compiti dispacciati finora | le quattro domande, per ogni compito, prima di dispacciarlo |
