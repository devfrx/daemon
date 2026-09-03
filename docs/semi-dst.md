# I semi della DST — e perché non sono un oracolo

**Creato il 2026-08-11**, col Traguardo 4.

## ⛔ La regola, prima dell'elenco

La §3.4 della [spec del sotto-progetto 1](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md)
è categorica, e questa pagina esiste perché quella regola sopravviva al primo che leggerà la
tabella qui sotto:

> **Un seme riproduce un'esecuzione soltanto finché il codice non cambia.** Modificato il kernel,
> lo stesso seme esplora un cammino diverso.

| Cosa è permanente | Cosa non lo è |
|---|---|
| la **proprietà** verificata | il **cammino** che quella volta la violò |
| il seme come **punto di ripartenza per indagare oggi** | la garanzia che domani ritrovi lo stesso difetto |

⛔ **Quindi questo elenco non si rigioca automaticamente**, e nessuna campagna lo legge.
Rigiocare un seme il cui cammino è cambiato **non prova nulla e costa tempo** — sarebbe la stessa
classe di errore di «cifrato a riposo» dichiarato più forte di quanto sia.

⛔ **E ogni voce nomina il test permanente della propria proprietà.** Una riga senza quella colonna
è una riga che non protegge niente: il seme serve a **indagare**, è la **proprietà** a proteggere.

## ⛔ Le due campagne non si identificano allo stesso modo, e chiamarle entrambe «semi» sarebbe falso

Il Traguardo 4 ha costruito **due** campagne con soggetti diversi, e **solo una delle due ha
semi**. Scriverlo qui costa quattro righe e impedisce di versionare la cosa sbagliata.

| | **Livello 1** — la riconciliazione del kernel | **Livello 2** — la coerenza di `redb` |
|---|---|---|
| dove | `crates/simulator/tests/dst_campaign.rs` | `crates/platform/tests/engine_crash_consistency.rs` |
| che cosa varia | un **seme**, che sceglie l'interlacciamento e — da un generatore **derivato** — il punto di caduta | ⛔ **niente: il ciclo è esaustivo e deterministico.** Percorre ogni punto d'iniezione da `OPERATIONS_TO_OPEN` alla saturazione |
| che cosa identifica un caso | il **seme** | il **punto d'iniezione** e il **numero di record** dello scenario |

⛔ **Quindi al livello 2 la parola «seme» non si usa**, e una voce di questo elenco che ne portasse
uno starebbe descrivendo qualcosa che non esiste.

### ⛔ Che cosa rende ambigua una voce, per ciascun livello

Un identificatore non basta a sé stesso: vale **insieme** alle costanti che definiscono lo spazio.
Se una di esse cambia, le voci già scritte smettono di indicare ciò che indicavano, **e nessun
controllo lo dice** — è il prezzo dichiarato dell'assenza di chiudente.

| Livello | Una voce resta interpretabile finché non cambiano |
|---|---|
| **1** | `SHORT_CAMPAIGN_SEEDS` (⚠️ **abbassarla scarta le voci più alte senza un rosso**) · la costante di mescolamento di `crash_seed` · `ACTIVITIES` e `STEPS`, che fissano `WRITES_PER_RUN` e quindi l'intervallo del punto di caduta. ⚠️ **E il seme è oggi l'indice del ciclo:** se la campagna passasse a un intervallo con offset o passo, ogni seme registrato diventerebbe ambiguo |
| **2** | `OPERATIONS_TO_OPEN` · la saturazione della propria profondità · il numero di record · ⛔ **e la versione di `redb`**, oggi `4.1.0`, perché quei conteggi sono proprietà del suo layout interno e non del nostro codice |

## L'elenco

| Livello | Identificatore | Cosa trovò | Il test permanente della proprietà |
|---|---|---|---|
| — | — | ⚠️ **Nessuna campagna è fallita su un caso, alla chiusura del Traguardo 4.** La riga esiste per dire che l'elenco è **vuoto e non dimenticato** | — |

⛔ **RICHIAMO DEL 2026-08-25 — IL TRAGUARDO 5 HA AGGIUNTO UNA TERZA CAMPAGNA, E L'ELENCO NON
GUADAGNA UNA RIGA.** La decisione è del **Task 13**, che è l'audit di chiusura di quel
traguardo, e gliel'aveva assegnata la §12 del [compendio](COMPENDIO.md).

⛔ **Perché no, e la ragione è la regola di questa pagina e non una preferenza:** l'elenco
raccoglie soltanto ciò che una campagna ha fatto **fallire**, e il punto 1 di *«Come si aggiunge
una voce»* comincia con *«una campagna fallisce»*. La campagna dell'arbitro non ha fatto fallire
nessun caso — ✅ rimisurato il 2026-08-25 e non citato: `bash scripts/gate.sh` esce
`GATE GREEN`, e le **cinque** sonde di `crates/simulator/tests/arbiter_campaign.rs` passano.
Una voce senza una **proprietà violata** sarebbe la falsa sicurezza che il riquadro in testa a
questa pagina esiste per impedire.

⚠️ **Ciò che invece cambia è un FATTO, e va scritto perché il riquadro qui sopra lo usa.** Il
riquadro *«Le due campagne non si identificano allo stesso modo»* dice **due** e parla del
**Traguardo 4**: è vero **di quel traguardo**, e resta com'è perché è un verbale. Da oggi le
campagne sono **tre**, e la terza — `crates/simulator/tests/arbiter_campaign.rs`, dal Task 12
del Traguardo 5 — sta nella **colonna di sinistra** di quella tabella: è di **livello 1**, e un
suo caso si identifica con un **seme**, come `dst_campaign.rs`. ⛔ **Le costanti che rendono
interpretabile una sua voce sono però le proprie:** `SHORT_CAMPAIGN_SEEDS` ed
`EXPECTED_OUTCOMES` di quel file, più lo scenario che `PARTIES` descrive. ⚠️
`EXPECTED_OUTCOMES` è un **rilevatore di cambiamento**: il giorno in cui diventa rosso il
rimedio è **rimisurare lo spazio degli esiti e riscegliere il numero**, non editarlo finché la
barra torna verde.

⚠️ **RICHIAMO DEL 2026-09-03, ALLA CHIUSURA DEL SOTTO-PROGETTO 1 — IL TRAGUARDO 6 HA
AGGIUNTO DUE CAMPAGNE, E IL CUMULATIVO È TOLTO.** Qui stava *«le campagne sono CINQUE»*:
un cumulativo invecchia alla campagna successiva, ed è il gotcha **#31**. Le due
nate col Traguardo 6 — `crates/simulator/tests/gui_death_campaign.rs` e
`crates/simulator/tests/worker_kill_campaign.rs` — stanno nella **colonna di sinistra** della
tabella qui sopra: livello **1**, un caso si identifica con un **seme**. Le costanti che rendono
interpretabile una loro voce sono le proprie: `SHORT_CAMPAIGN_SEEDS` ed `EXPECTED_WORLDS` nella
prima, `SHORT_CAMPAIGN_SEEDS` e `KILL_HORIZON` nella seconda. ⛔ **L'elenco non guadagna righe:**
nessuna delle due è fallita su un caso, e il verbale della chiusura lo dichiara.

⚠️ **E il chiudente di questa pagina resta quello che il riquadro in fondo dichiara: non ce
n'è uno.** Il Task 13 ha deciso **la riga**, non la **guardia**: pretendere che ogni voce nomini
un test esistente sarebbe una riga di catalogo nuova in `scripts/check-docs.sh`, e quella è del
proprietario, registrata e non presa.

⚠️ **«Vuoto» non significa «il traguardo non ha trovato difetti».** Ne ha trovati molti, e stanno
nell'errata in testa al
[piano](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md): erano difetti
**del piano e dei banchi**, colti dal pre-controllo e dalle mutazioni, non dalle campagne. Questo
elenco raccoglie soltanto ciò che una campagna ha fatto **fallire**, ed è la distinzione che gli
dà valore.

## Come si aggiunge una voce

1. Una campagna fallisce. ⛔ **Il messaggio nomina il caso** — il seme al livello 1, il punto
   d'iniezione al livello 2 — perché ogni asserzione di entrambe le campagne lo porta nel proprio
   testo. **Si annota quello.**
2. ⚠️ **Non esiste un modo di rigiocare un caso solo, ed è dichiarato invece di essere scoperto:**
   le campagne percorrono l'intero intervallo e non prendono filtri. Per indagare si scrive una
   **sonda usa-e-getta** che chiami direttamente `run(seed, …)` al livello 1 o
   `crash_then_reopen(records, falls_at)` al livello 2, e la si cancella dopo. ⚠️ **L'ordine dei
   due argomenti è quello, e non l'inverso**: sono entrambi `u64`, quindi scriverli scambiati
   **compila** e indaga uno scenario diverso da quello che ha prodotto il rosso, restando verde.
   Questa riga li dava invertiti — audit del 2026-08-11. Costa tre righe, e
   comprarne di meglio significherebbe aggiungere un meccanismo che nessuno ha chiesto.
3. Si trova la **proprietà** violata — non il cammino.
4. ⛔ **Si scrive un test che tenga quella proprietà**, e che fallisca prima della correzione.
5. Si corregge.
6. Si aggiunge la riga qui, **con il nome del test del punto 4** e con le costanti che la rendono
   interpretabile.

⚠️ Se il punto 4 non si riesce a fare, la voce **non si aggiunge**: un seme senza proprietà è
esattamente la falsa sicurezza che il riquadro in testa vieta.

## ⛔ Questo file non ha un chiudente, ed è una decisione registrata e non presa

**Nessuno script verifica che una voce nomini un test che esiste**, né che le costanti citate
siano ancora quelle. È l'unico artefatto del Traguardo 4 senza un controllo, e la ragione è nella
§10 del [disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md):
un elenco di semi che qualcosa *esercita* è un elenco che qualcuno leggerà come una **rete di
regressione**, cioè precisamente l'errore che il riquadro in testa esiste per impedire.

⚠️ **Il prezzo è dichiarato:** un elenco di semi senza proprietà è l'artefatto che marcisce meglio
di tutti, e qui a difenderlo c'è **solo chi lo rilegge**. Una guardia in `scripts/check-docs.sh`
che pretenda che ogni voce nomini un test esistente sarebbe una **riga di catalogo nuova**, e
quella è una decisione del proprietario — **registrata qui e nella §6 del
[compendio](COMPENDIO.md), non presa**.
