# Archivio — le misure dei pesi dei documenti

⛔ **Non è una lettura obbligatoria.** Verbale datato, estratto dalla §12 del
compendio il **2026-08-28** dal piano di sfoltimento
([`../superpowers/plans/2026-08-28-sfoltimento-compendio.md`](../superpowers/plans/2026-08-28-sfoltimento-compendio.md)).
Si apre con una domanda storica in mano, non per farsi un'idea.

⚠️ **I numeri qui dentro erano veri il giorno in cui furono presi.** Il peso di oggi
lo dà il comando, che non marcisce:

```
find docs -name '*.md' | xargs wc -c | sort -n
```

📌 **Quanti riquadri siano lo dice il comando, non questa riga:**
`grep -cE '^> 🔁 \*\*' docs/archivio/misure-dimensioni.md`.

⚠️ **E il primo conto era sbagliato di uno.** Il piano di sfoltimento diceva
*«cinquantanove»*, contate con un pattern che pretendeva la parola *«misura»* dopo
l'ordinale — e un riquadro si intitola *«Rimisurati lo stesso giorno»*. ⛔ **La cifra è TOLTA e non
riallineata il 2026-08-28**, dal riquadro che l'aveva resa stantia: il conto lo dà il comando
qui sopra, che sta già in questa pagina e non marcisce.
È il gotcha **#31** commesso dentro il lavoro che lo prende di mira: un numero messo a
sostegno di una regola giusta non viene mai rimisurato. Colto da un'asserzione dello
script di taglio, che si è fermato **prima di scrivere**.

---

> 🔁 **Rimisurati lo stesso giorno, chiudendo la §7.1.1 — e il gotcha #31 si è ripetuto sul
> paragrafo scritto per chiuderlo.** Cinque pesi su dodici erano di nuovo fuori, e non tutti
> per la crescita:
>
> | | |
> |---|---|
> | **cresciuti** | spec del sotto-progetto 1 `245 → 253` · HANDOFF `84 → 92` · roadmap `11 → 13`. HANDOFF era **già** stantio prima di questa sessione: due commit l'avevano toccato dopo la misura |
> | ⛔ **sovrastimati, su file mai toccati** | il piano degli spike `72 → 68` · `design/08` `9 → 8` · il minimo degli ADR dichiarato `4 KB` per un file da **2441 byte**. La crescita non spiega un numero che **scende**: la prima misura contava lo spazio **allocato** — che su NTFS arrotonda ogni file al cluster — invece dei byte |
>
> 📌 **Il rimedio, e stavolta è il metodo invece del numero:** i pesi si misurano con
> **`wc -c`, arrotondati a KiB**. Scritto qui, il prossimo che riconta ottiene la stessa
> cifra o scopre una crescita vera — e non un artefatto dello strumento.

> 🔁 **Terza misura, alla chiusura del Traguardo 1 — e il metodo ha retto.** Tutti gli
> scarti sono **crescite vere**, nessuno è un artefatto dello strumento: è la differenza
> fra le prime due misure e questa.
>
> | | |
> |---|---|
> | **cresciuti** | HANDOFF `92 → 104`, per i quattro gotcha nuovi · spec del sotto-progetto 1 `253 → 259` · roadmap `13 → 14` · README `9 → 10` |
> | **voci nuove in tabella** | [`porta-di-qualita.md`](../porta-di-qualita.md) **9 KB** e il piano del Traguardo 1 **50 KB**: esistono da oggi |
> | **invariati** | kernel-design 44 · tracciabilità 15 · riferimenti 25 · `design/08` 8 · il piano degli spike 68 · gli ADR 2–19 |
>
> Il totale dell'insieme *«HANDOFF + spec del sotto-progetto 1 + `adr/`»* era **577 KB**.
> ⚠️ Cresce a ogni chiusura: la frase in testa dice «oltre mezzo megabyte» apposta.

> 🔁 **Quarta misura, il 2026-08-09, chiudendo la passata di coerenza.** Quattro scarti,
> tutti **crescite vere**: spec del sotto-progetto 1 `259 → 263` e
> [`porta-di-qualita.md`](../porta-di-qualita.md) `9 → 11` per la riga del build script ·
> HANDOFF `104 → 105` e [`riferimenti.md`](../riferimenti.md) `25 → 30` per le misure del
> Traguardo 1, che il §13 pretendeva lì e non c'erano. L'insieme passa da **577** a
> **581 KB**, ed è la cifra in testa a questo file e in `CLAUDE.md`. ⚠️ **E i due file
> obbligatori sono passati da 85 a 87 KB**: la cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), e va rifatta ogni volta che uno dei due cresce.
> 📌 Due misure di seguito senza artefatti dello strumento: il metodo `wc -c` regge.

> 🔁 **Quinta misura, il 2026-08-09, chiudendo la sessione dei Task 1–6 del Traguardo 2.**
> Tutti scarti sono **crescite vere**; il metodo `wc -c` regge da tre misure di seguito.
>
> | | |
> |---|---|
> | **cresciuti** | HANDOFF `105 → 109` · spec del sotto-progetto 1 `263 → 266` (le tre righe nuove del catalogo) · [`porta-di-qualita.md`](../porta-di-qualita.md) `11 → 13` · roadmap `14 → 15` |
> | **voce nuova** | il **piano del Traguardo 2**, che è il più grande scritto finora |
> | **invariati** | kernel-design 44 · tracciabilità 15 · riferimenti 30 · `design/08` 8 · README 10 · il piano degli spike 68 |
>
> L'insieme *«HANDOFF + spec del sotto-progetto 1 + `adr/`»* passa da **581** a **589 KB**.
> ⚠️ **E i due file obbligatori sono passati da 87 a 88 KB**: la cifra vive in `CLAUDE.md` e
> in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), e va rifatta ogni volta che uno dei due cresce.

> 🔁 **Sesta misura, il 2026-08-09 — e il gotcha #31 aveva cambiato forma.** Nata di
> rimbalzo: toccando [`porta-di-qualita.md`](../porta-di-qualita.md) per registrarvi il rifiuto
> di `rustfmt`, la regola *«i pesi si rimisurano quando si toccano i file che contano»*
> obbligava a rifare quella riga — che diceva **11 KB** mentre la **quinta misura, due
> riquadri più su, aveva già registrato `11 → 13`**.
>
> ⛔ **Non era una riga: erano tutte e quattro.** La quinta misura dichiarava quattro
> crescite, e **nessuna** delle quattro era stata riportata nella tabella qui sopra. Il
> riquadro e la tabella sono **due posti**, si aggiorna il primo, e il secondo continua a
> rispondere a chi deve decidere se aprire un file.
>
> | Riga | Diceva | Misurata ora | |
> |---|---|---|---|
> | spec del sotto-progetto 1 | 263 | **267** | la quinta diceva 266 |
> | [`HANDOFF.md`](../HANDOFF.md) | 105 | **111** | la quinta diceva 109 |
> | [`riferimenti.md`](../riferimenti.md) | 30 | **32** | mai corretta |
> | [`roadmap.md`](../roadmap.md) | 14 | **15** | la quinta diceva 15 |
> | [`porta-di-qualita.md`](../porta-di-qualita.md) | 11 | **15** | la quinta diceva 13; cresce oggi per il riquadro su `rustfmt` |
>
> **Le altre sette righe reggono** — kernel-design 44 · tracciabilità 15 · `design/08` 8 ·
> `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 — e così
> il minimo e il massimo degli ADR, `2–19 KB`, ricontati sui file (2441 B e 19291 B).
>
> ⚠️ **Anche i due aggregati erano fuori, e uno è colpa di questo commit.** L'insieme
> *«HANDOFF + spec + `adr/`»* è **591 KB** (604910 B) contro i 589 dichiarati — cresciuto per
> conto suo. I **due file obbligatori** passano da 88 a **94 KB**: **91 erano già a HEAD**, e
> il resto lo aggiunge questo riquadro. La cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), ed è aggiornata in entrambi.
>
> ⛔ **Questa cifra descrive il file che la contiene, e si scrive per ultima.** Misurata
> mentre scrivevo ha dato 92, poi 93, poi **95946 B**: ogni riga aggiunta la spostava, e due
> volte l'ho scritta già falsa. Converge solo perché l'ultima correzione è **di sole cifre**,
> che non cambiano la lunghezza della riga. 📌 Chi tocca questo paragrafo **rimisura dopo
> averlo chiuso**, e cambia solo il numero.
>
> 📌 **La forma nuova del #31, ed è quella da ricordare:** il numero non era *mai stato*
> rimisurato — lo era stato, e scritto **in uno solo dei due posti in cui vive**. Rimisurare
> non basta: si scrive **dove qualcuno legge per decidere**, che è la tabella, non il verbale
> della misura.

> 🔁 **Settima misura, il 2026-08-09, chiudendo la voce dello SHA — e la sesta aveva corretto
> i numeri lasciando fuori una riga intera.** Il rimedio scritto nella sesta — *«si scrive dove
> qualcuno legge per decidere, che è la tabella»* — è stato applicato ai **pesi esistenti** e
> non alla **voce mancante**: la quinta misura dichiarava *«voce nuova: il piano del Traguardo
> 2»*, e in tabella quella riga **non è mai entrata**.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | il **piano del Traguardo 2**, **131 KB** — il secondo file più grande del repository, e proprio quello da cui si riprende. Chi doveva decidere se aprirlo non aveva né la voce né il peso |
> | **cresciuto** | [`HANDOFF.md`](../HANDOFF.md) `111 → 113`, per il gotcha #43 di questa voce |
> | **invariati, ricontati** | spec del sotto-progetto 1 267 · kernel-design 44 · roadmap 15 · tracciabilità 15 · [`porta-di-qualita.md`](../porta-di-qualita.md) 15 · riferimenti 32 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **591** a **593 KB** (607118 B). I **due file
> obbligatori** passano da 94 a **97 KB**: la cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), ed è aggiornata in entrambi.
>
> 📌 **La forma successiva del #31, e completa quella della sesta misura:** una tabella si
> aggiorna in **due modi** — correggendo una cella, e **aggiungendo una riga**. Il secondo si
> dimentica, e per una ragione che vale la pena scrivere: rileggendo, **una riga assente non si
> vede**, mentre una cella sbagliata sì. Chi rimisura conta anche le **righe**, non solo i
> numeri dentro di esse.

> 🔁 **Ottava misura, il 2026-08-09, chiudendo il Task 7 — e stavolta il #31 non si è ripetuto.**
> È la prima rimisura fatta **applicando la lezione della settima**: si aggiornano le celle **e**
> si contano le **righe**, e si scrive nella tabella prima che nel verbale. Nessuna riga mancava:
> quella del piano del Traguardo 2, aggiunta due misure fa, ha retto al primo controllo.
>
> | | |
> |---|---|
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `113 → 117` per i gotcha **#44** e **#45** · [`porta-di-qualita.md`](../porta-di-qualita.md) `15 → 17` per le sei sonde `R` · il **piano del Traguardo 2** `131 → 135` per l'errata del Task 7 |
> | **invariati, ricontati** | spec del sotto-progetto 1 267 · kernel-design 44 · roadmap 15 · tracciabilità 15 · riferimenti 32 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **593** a **597 KB** (611081 B). I **due file
> obbligatori** passano da 97 a **101 KB**: la cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), ed è aggiornata in entrambi.
>
> 📌 **E una cifra tonda è una trappola in arrivo**, quindi si dice adesso: i due file
> obbligatori hanno passato i **101 KB**, e la frase che li accompagna li prezza *«circa
> venticinquemila token»* col rapporto della prima volta. Il rapporto **non è stato rimisurato
> da quando fu fissato**, e nessuno lo dubita perché la regola che sostiene è giusta — che è la
> definizione esatta del **#31**. ⚠️ Resta comunque il confronto che conta, ed è **101 KB contro
> 597**.

> 🔁 **Nona misura, il 2026-08-09, chiudendo la sessione dei Task 8–10 del Traguardo 2 — e la
> riga mancante della settima non si è ripetuta.** Contate le **righe** prima dei numeri dentro
> di esse, come pretende la lezione della settima: ogni file citato in questa sezione ha la sua
> voce in tabella, verificato uno per uno. Nessuna riga da aggiungere.
>
> | | |
> |---|---|
> | **cresciuto** | [`HANDOFF.md`](../HANDOFF.md) `117 → 119`, per il gotcha **#46** — YAGNI che, su una porta mai implementata, cancella ciò che serve a implementarla |
> | ⛔ **e tre righe che questo riquadro dichiarava invariate lo erano solo a metà passata** | [`porta-di-qualita.md`](../porta-di-qualita.md) `17 → 27` per gli undici artefatti registrati · [`riferimenti.md`](../riferimenti.md) `32 → 43` per le misure dei Task 7–10 · il **piano del Traguardo 2** `135 → 147` per l'errata E19–E35 |
> | **invariati, ricontati a passata chiusa** | spec del sotto-progetto 1 267 · kernel-design 44 · roadmap 15 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> ⛔ **E la riga qui sopra è essa stessa il #31, colto mentre accadeva.** Questo riquadro fu
> scritto **a metà passata**, quando quei tre file erano ancora a HEAD, e dichiarava
> «invariati, ricontati» tre file che di lì a poco sarebbero cresciuti — per un lavoro **già
> in corso in parallelo**, e quindi prevedibile. 📌 **La regola che ne esce, e costa zero:** un
> verbale di misura si scrive **quando la passata è chiusa**, mai mentre altri stanno ancora
> scrivendo; e se lo si scrive prima, si rimisura prima di committare. Una misura vera **di un
> momento sbagliato** è indistinguibile da una misura falsa per chi la legge dopo.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **597** a **599 KB** (613166 B). I **due file
> obbligatori** passano da 101 a **108 KB**: **103 erano già a HEAD**, ed è la cifra che
> `CLAUDE.md` e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) portano **oggi**; il resto lo aggiungono questo
> riquadro e le due questioni aperte della §6. ⚠️ **In quei due file la cifra dice ancora 103**,
> e va rifatta lì: vive in **tre** posti, e questa passata ne ha aggiornato **uno**.
>
> ⛔ **E il verbale è arrivato dopo i numeri, che è la variante successiva del #31.** Le cifre
> erano **giuste e già in tabella**; a mancare era il riquadro, perché la misura era stata
> scritta **solo nel messaggio del commit** — un posto che nessuno rilegge per decidere se
> aprire un file. Un numero corretto senza il proprio verbale non si può né rifare né dubitare:
> chi rimisura non sa da cosa parte. 📌 Il verbale è parte della misura, non il suo racconto.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi si rimisura **dopo**
> aver chiuso questo riquadro e si corregge **di sole cifre** — il metodo della sesta misura,
> alla seconda applicazione.

> 🔁 **Decima misura, il 2026-08-09, chiudendo la voce della regola B — e per la prima volta
> nessuna riga era stantia se non quelle che questa passata ha fatto crescere.** È anche la prima
> scritta **a passata chiusa**, che è la regola uscita dalla nona: le tre righe qui sotto sono
> state misurate quando non restava niente da scrivere, non mentre qualcuno scriveva ancora.
>
> | | |
> |---|---|
> | **cresciuti** | spec del sotto-progetto 1 `267 → 271` per la riga della **regola B**, il suo richiamo e i riallineamenti di §7.4.7 e §8.3 · [`HANDOFF.md`](../HANDOFF.md) `119 → 120` per la terza occorrenza del **#36** · [`porta-di-qualita.md`](../porta-di-qualita.md) `27 → 28` per la riga nuova e la nota di chiusura |
> | **invariati, ricontati** | kernel-design 44 · roadmap 15 · tracciabilità 15 · [`riferimenti.md`](../riferimenti.md) 43 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il **piano del Traguardo 2** 147 · README 10 · ADR `2–19` (2441 B e 19291 B) |
> | ✅ **righe contate prima dei numeri** | ogni file citato in §12 ha la propria voce in tabella, verificato uno per uno. **Nessuna riga da aggiungere** — la lezione della settima, alla seconda applicazione riuscita |
> | ✅ **e il residuo dichiarato dalla nona è chiuso** | la nona misura lasciava scritto *«in quei due file la cifra dice ancora 103»*: `CLAUDE.md` e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) sono stati allineati, e la cifra oggi è la stessa in **tutti e tre** i posti |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **599** a **605 KB** (619105 B). I **due file
> obbligatori** passano da 108 a **111 KB**. Le due cifre vivono in `CLAUDE.md`, in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) e qui, e sono aggiornate in tutti e tre — insieme alla spec,
> che in testa a questo file e in `CLAUDE.md` è nominata per nome (`270`).
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla terza
> applicazione. Regge perché ogni correzione qui è fra numeri della **stessa lunghezza**.
>
> 📌 **Cosa dice il #31 questa volta, ed è una notizia buona:** non si è ripetuto. Quattro misure
> di seguito senza artefatti dello strumento, due di seguito senza righe mancanti, e la prima
> senza celle stantie estranee alla passata. ⚠️ Il che è esattamente il momento in cui si smette
> di rimisurare — quindi resta scritto che la **prossima** si fa lo stesso.

> 🔁 **Undicesima misura, il 2026-08-09, chiudendo il Task 11 — e la crescita più grande non è
> quella che ci si aspetta.** Contate le **righe** prima dei numeri dentro di esse: ogni file
> citato in §12 ha la sua voce, nessuna da aggiungere. Scritta **a passata chiusa**, come pretende
> la nona.
>
> | | |
> |---|---|
> | **cresciuti** | ⛔ [`porta-di-qualita.md`](../porta-di-qualita.md) `28 → 38` — **il salto più grande di ogni misura finora**, e non per una porta nuova: dieci kilobyte sono la **campagna di mutazione** e i quattro esiti credibili e falsi del banco · [`HANDOFF.md`](../HANDOFF.md) `120 → 127` per i gotcha **#47** e **#48** e le tre occorrenze nuove · [`riferimenti.md`](../riferimenti.md) `43 → 47` per le misure del Task 11 · il **piano del Traguardo 2** `147 → 153` per l'errata E36–E41 |
> | **invariati, ricontati** | spec del sotto-progetto 1 271 · kernel-design 44 · roadmap 15 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` (2441 B e 19291 B) |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **605** a **611 KB** (625805 B). I **due file
> obbligatori** passano da 111 a **117 KB**; la cifra vive in `CLAUDE.md`, in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) e qui, ed è aggiornata in tutti e tre.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla quarta
> applicazione.
>
> 📌 **E il rapporto che la §12 esiste per difendere non si è mosso: 117 KB contro 611.** ⚠️ Ma un
> registro che cresce di dieci kilobyte in un compito solo è il **prossimo** candidato al #31 di
> un genere nuovo — non un numero stantio, un **documento che smette di essere letto perché è
> diventato troppo lungo**. Scritto qui perché chi lo noterà per primo abbia da dove partire.

> 🔁 **Dodicesima misura, il 2026-08-10, chiudendo il Task 12 — e per la prima volta un file è
> _sceso_.** Scritta a passata chiusa, righe contate prima dei numeri: nessuna voce da aggiungere.
>
> | | |
> |---|---|
> | ⛔ **sceso** | [`porta-di-qualita.md`](../porta-di-qualita.md) **47 → 40**, e la storia è tutta lì: era arrivato a 47 KB e **531 righe**, di cui **228 — il 43%** — di prosa su **una riga di tabella su tre**, mentre le due righe vicine nella stessa tabella ne hanno **zero**. Riportato a **449 righe** con quindici intestazioni invece di cinque. In tabella la cella passa da 38 a **40**, perché 38 era il valore dell'undicesima misura: il picco a 47 non è mai stato scritto qui |
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `127 → 129` per le tre forme nuove del **#48** · il **piano del Traguardo 2** `153 → 158` per l'errata E42–E46 |
> | **invariati, ricontati** | spec del sotto-progetto 1 271 · kernel-design 44 · roadmap 15 · tracciabilità 15 · [`riferimenti.md`](../riferimenti.md) 47 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> ⛔ **E questa misura ha trovato un residuo dell'undicesima, che è il #31 nella forma che la
> SESTA misura aveva scritto per impedirlo.** L'undicesima aveva portato l'aggregato da 605 a
> **611** e lo aveva scritto **solo nel proprio riquadro**: in testa a questo file e in
> `CLAUDE.md` — cioè **dove qualcuno legge per decidere se aprire** — era rimasto **605**. La
> cifra dei due file obbligatori, invece, era stata propagata in tutti e tre i posti. 📌 Quindi
> non è che la lezione non ci fosse: **c'era, scritta da me, e ho applicato metà del rimedio**.
> Un aggregato ha **due** case e un numero solo ne ha una, e la seconda si dimentica proprio
> perché la prima è stata fatta.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **611** a **614 KB** (628305 B), ed è corretto
> **in tutti e tre i posti** questa volta. I **due file obbligatori** passano da 117 a **123 KB**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla quinta
> applicazione.
>
> 📌 **E il registro che scende è la notizia della misura.** Undici misure di seguito hanno
> registrato solo crescite, e questa è la prima volta che un documento è stato **ridotto perché
> aveva smesso di rispondere alla propria domanda**. ⚠️ La regola che ne esce vale oltre il caso:
> un file che cresce non è un problema, un file che cresce **in una sezione sola** lo è — e il
> segnale non è il peso, è **la sproporzione fra righe vicine della stessa tabella**.

> 🔁 **Tredicesima misura, il 2026-08-10, chiudendo la sessione — ed è la passata di
> _manutenzione_, quella che di solito non si misura.** Nessun compito eseguito: solo il
> riallineamento dei documenti di stato che erano rimasti al **Task 6**.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](../riferimenti.md) `47 → 52` per le misure del Task 12 · [`HANDOFF.md`](../HANDOFF.md) `129 → 131` per lo stato e i richiami · [`roadmap.md`](../roadmap.md) `15 → 16` e [`README.md`](../README.md) `10 → 11`, che erano **fermi sei compiti indietro** |
> | **invariati, ricontati** | spec 271 · kernel-design 44 · tracciabilità 15 · [`porta-di-qualita.md`](../porta-di-qualita.md) 40 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 158 · ADR `2–19` |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **614** a **615 KB** (630190 B). I **due file
> obbligatori** passano da 123 a **124 KB**, mossi da questo riquadro stesso; rimisurati
> **dopo** averlo chiuso e corretti **di sole cifre**, come sempre.
>
> ⛔ **E la notizia di questa misura è quali file erano stantii.** `roadmap.md` e `README.md`
> dicevano *«eseguito fino al Task 6, si riprende dal Task 7»* mentre il repository era al
> **12**: sei compiti di scarto, per **due sessioni**. Nessuno se n'era accorto perché la §6
> del compendio era **giusta**, e chi lavora legge quella. ⚠️ **Ma non è chi lavora il loro
> lettore**: `README.md` è la porta d'ingresso di chi arriva, e `roadmap.md` è dove si guarda
> per decidere **cosa viene dopo**. 📌 La regola che ne esce, ed è la ragione per cui questa
> passata esiste: **la §6 giusta non protegge gli altri documenti di stato — li nasconde**,
> perché toglie a chi lavora ogni occasione di incontrarli stantii.

> 🔁 **Quattordicesima misura, il 2026-08-10, chiudendo il Traguardo 2 — e porta la misura che
> tredici riquadri avevano rimandato.** Scritta a passata chiusa, righe contate prima dei numeri
> dentro di esse: ogni file citato in questa sezione ha la sua voce, nessuna da aggiungere.
>
> | | |
> |---|---|
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `131 → 137` per il gotcha **#49**, il blocco di lascito del Traguardo 2 e lo stato · il **piano del Traguardo 2** `158 → 162` per l'errata E47–E49 · [`roadmap.md`](../roadmap.md) `16 → 17` e [`porta-di-qualita.md`](../porta-di-qualita.md) `40 → 41` |
> | **invariati, ricontati** | spec del sotto-progetto 1 271 · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 11 · ADR `2–19` (2441 B e 19291 B) |
> | ⛔ **e una riga che questo riquadro aveva già dichiarato invariata** | [`riferimenti.md`](../riferimenti.md) `52 → 54`, per i comandi del riconteggio del catalogo, scritti lì **dopo** che questo riquadro era chiuso. È il difetto della **nona** misura — *«un verbale si scrive quando la passata è chiusa, mai mentre altri stanno ancora scrivendo»* — ripetuto da me contro me stesso, a distanza di quattro riquadri e con la regola scritta di mia mano. 📌 Colto **prima** di committare, e questo è l'unico merito: la nona prescriveva *«e se lo si scrive prima, si rimisura prima di committare»*, ed è quella metà del rimedio ad aver funzionato |
> | ⛔ **una riga riscritta, non un numero** | la voce del piano del Traguardo 2 diceva *«il compito da cui si riprende — è il piano in corso, e **il Task 11** sta lì»*: era stantia di **due compiti** già prima di questa sessione, e la §6 nel frattempo era giusta. È il difetto della **tredicesima** misura — la §6 giusta nasconde gli altri documenti — spostato **dentro questo stesso file**, fra la §6 e la §12 |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **615** a **621 KB** (635880 B), corretto in
> **tutti e tre** i posti. I **due file obbligatori** passano da 124 a **133 KB**.
>
> ⛔ **E il #31 che `CLAUDE.md` dichiarava «il prossimo in arrivo» è arrivato, ed è peggio di
> come la riga lo prezzava.** Quella riga dice *«circa venticinquemila token»* per i due file,
> col rapporto della prima volta, e ammette che **il rapporto non è mai stato rimisurato**.
> Misurato oggi, indirettamente ma senza ambiguità: leggendo questo file, **quattrocento righe
> hanno pesato `25148` token** — cioè da sole quanto la riga attribuisce ai **due file interi**.
> Questo file ne ha **milleduecentosessantaquattro**.
>
> ⚠️ **Non scrivo il totale, e la ragione è la regola di questo repository.** Lo strumento che
> ha prodotto quel numero è il lettore di file di un agente, non un contatore di token dedicato,
> e la densità cambia per sezione — le tabelle costano più della prosa. Un totale calcolato da
> un campione sarebbe **un'ipotesi scritta come misura**, cioè il gotcha #15. Quel che è
> **certo** è il limite inferiore, ed è quello che serve a decidere: i due file obbligatori
> costano **almeno tre volte** ciò che la loro riga dichiara. 📌 La cifra esatta la scriverà chi
> passerà un contatore vero su entrambi; fino ad allora la riga porta il limite inferiore, come
> la frase in testa porta «oltre mezzo megabyte» invece di un totale.
>
> 📌 **E il confronto che la §12 esiste per difendere regge lo stesso, anzi meglio di prima:**
> **133 KB contro 621**. Il rapporto sbagliato non ha mai messo in pericolo la regola che
> sosteneva — ed è precisamente perché la regola era giusta che nessuno ha dubitato del numero.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo**
> aver chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> sesta applicazione.

> 🔁 **Quindicesima misura, il 2026-08-10, scrivendo il piano del Traguardo 3 — ed è una passata
> con una riga nuova, che è il modo di aggiornare una tabella che si dimentica.** Scritta a
> passata chiusa; righe contate prima dei numeri dentro di esse.
>
> | | |
> |---|---|
> | ⛔ **tre righe aggiunte** | il **piano del Traguardo 3**, **92 KB** — il file da cui si riprende · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) **12 KB** · e i **pesi degli spike**, `RISULTATI.md` **23** e `GUI-REQUISITI.md` **6**, che avevano la voce e la **cella vuota** da sempre |
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `137 → 138` · [`roadmap.md`](../roadmap.md) `17 → 18` |
> | **invariati, ricontati** | README 11 · [`riferimenti.md`](../riferimenti.md) 54 · [`porta-di-qualita.md`](../porta-di-qualita.md) 41 · spec 271 · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · ADR `2–19` |
> | **e un conteggio stantio trovato di rimbalzo** | la tabella dei piani di [`roadmap.md`](../roadmap.md) diceva *«errata di quarantasei voci in quattro passate»*: con E47–E49 sono **quarantanove in sei**. Non l'ha trovato un controllo — l'ha trovato il fatto che quella riga andava toccata comunque per aggiungerne una accanto |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **621** a **622 KB** (637165 B). I **due file
> obbligatori** passano da 133 a **139 KB**.
>
> ⛔ **E la riga di `AVVIO-CHAT.md` mancava da sempre, mancata da SEI misure di seguito che
> dichiaravano di averla cercata.** Dalla nona in poi ogni riquadro porta la frase *«ogni file
> citato in questa sezione ha la sua voce in tabella, verificato uno per uno»* — e
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) è citato **dieci volte dentro la §12 stessa**, oltre che in
> §6, §9 e §13. ⚠️ **Non l'ha trovata chi rimisurava: l'ha trovata una revisione esterna**, e la
> differenza è il punto. 📌 **La forma del #31 che ne esce, ed è nuova:** una verifica ripetuta
> uguale sei volte **non è sei verifiche** — è una sola, ripetuta. Chi controlla la propria
> tabella parte dalle righe che ci sono e ne verifica i numeri; per accorgersi di una riga
> **assente** bisogna partire dall'**altro capo** — dall'elenco dei file citati — ed è un
> movimento che nessuna delle sei ha fatto. Chi rimisura la prossima volta parta di lì.
>
> 📌 **Altre due righe assenti, fuori dalla §12 e pre-esistenti, chiuse nella stessa passata:**
> la mappa dei documenti di [`HANDOFF.md`](../HANDOFF.md) non elencava **questo file** né
> `AVVIO-CHAT.md`, e la tabella «Dove va cosa» di [`README.md`](../README.md) non elencava
> `superpowers/plans/`, cioè **la cartella da cui si riprende il lavoro**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla settima
> applicazione.

> 🔁 **Sedicesima misura, il 2026-08-10, chiudendo il Task 2 del Traguardo 3 — ed è la prima
> aperta perché una revisione ha chiesto il verbale, non perché chi scriveva se ne fosse
> accorto.** Scritta a passata chiusa; righe contate prima dei numeri dentro di esse, e
> **partendo dall'elenco dei file citati** — il movimento che la quindicesima ha prescritto:
> diciotto citazioni nella §12, diciotto righe, **nessuna assente**.
>
> | | |
> |---|---|
> | **cresciuti** | spec del sotto-progetto 1 `271 → 274` · [`HANDOFF.md`](../HANDOFF.md) `138 → 142` · [`riferimenti.md`](../riferimenti.md) `54 → 62` · [`porta-di-qualita.md`](../porta-di-qualita.md) `41 → 44` · il piano del Traguardo 3 `92 → 95`, per l'errata che il Task 1 gli ha messo in testa |
> | **invariati, ricontati** | kernel-design 44 · roadmap 18 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · README 11 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 12 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) · `design/` nove file `4–9` |
> | **una cella di testo, non di cifre** | la riga del piano del Traguardo 3 diceva *«dodici compiti, e il primo è il record durevole»*. Il primo **è eseguito**: ora dice da dove si riprende, che è la sola cosa che quella riga serve a dire |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **622** a **629 KB** (643910 B). I **due file
> obbligatori** passano da 139 a **143 KB**, e quella cifra vive in **cinque** posti — uno in
> `CLAUDE.md` e **quattro** in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) — più il riquadro della
> quindicesima misura, che è **verbale** e non si tocca.
>
> ⛔ **E cinque cifre vivevano in cinque posti con tre valori diversi, che è la forma peggiore
> del #31.** Il Task 2 aveva rimisurato *«622 → 630»* e l'aveva scritto in **un posto solo**,
> lasciando `CLAUDE.md` a contraddirsi **a tre righe di distanza** — 622 alla riga 25, 630 alla
> 28 — più il riquadro in testa a questo file e la riga della spec in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md). ⚠️ **È letteralmente ciò che la sesta misura aveva scritto
> per impedirlo** — *«rimisurare non basta: si scrive dove qualcuno legge per decidere»* —
> ripetuto **dentro `CLAUDE.md`**, il file che quella regola contiene. Chi rimisura cerca la
> cifra vecchia con un `grep` **su tutto il repository**, non nel file che ha in mano.
>
> ⚠️ **E due delle cifre di quella rimisura erano sbagliate, ciascuna a modo suo.** La spec era
> **troncata** invece che arrotondata — il metodo della §12 è *«`wc -c`, arrotondati a KiB»* — e
> poi il numero è **sceso ancora**, perché la stessa revisione ha fatto accorciare un richiamo
> datato di venti righe: `276` alla revisione, **274** a passata chiusa. 📌 **La lezione è
> sull'ordine, non sull'aritmetica:** una rimisura fatta **prima** dell'ultima correzione è
> un'ipotesi, e vale anche quando le correzioni **riducono** invece di far crescere.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, all'ottava
> applicazione.

> 🔁 **Diciassettesima misura, il 2026-08-10, chiudendo il Task 8 del Traguardo 3 — e la notizia
> non è quanto sono cresciuti i file, è che NESSUNO li aveva rimisurati per cinque compiti.**
> I Task 3, 4+5, 6 e 7 hanno fatto crescere sei righe della §12 e **nessuna** è stata toccata:
> la tabella era già sbagliata **prima** che questa sessione cominciasse, e chi l'ha letta in
> mezzo ha deciso su cifre vecchie. ⚠️ **È il #31 nella forma che la sedicesima misura aveva
> descritto** — *«il numeratore lo muove chi esegue, e chi esegue non apre la §12»* — con la
> differenza che qui non è una riga: sono **sei**.
>
> | | |
> |---|---|
> | **quanto erano indietro AL COMMIT PRECEDENTE**, cioè prima di questo compito | [`porta-di-qualita.md`](../porta-di-qualita.md) `44 → 71` (**+27**) · il piano del Traguardo 3 `95 → 125` (**+30**) · [`riferimenti.md`](../riferimenti.md) `62 → 84` (**+22**) · [`HANDOFF.md`](../HANDOFF.md) `142 → 153` (**+11**) · spec del sotto-progetto 1 `274 → 277` · [`README.md`](../README.md) `11 → 12` |
> | **cresciuti da questo compito**, sopra quei valori | [`porta-di-qualita.md`](../porta-di-qualita.md) `71 → 73` per la sonda nuova · il piano `125 → 136` per le dodici voci d'errata · [`riferimenti.md`](../riferimenti.md) `84 → 89` per le misure del Task 8 · [`HANDOFF.md`](../HANDOFF.md) `153 → 159` per i gotcha **#51** e **#52** e le tre righe del traguardo · [`roadmap.md`](../roadmap.md) `18 → 19` · questo file `147 → 156` |
> | **invariati, ricontati** | kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 12 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · `adr/` 214 |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **629** a **649 KB** (664851 B) — e **644 era
> già il vero** al commit precedente. I **due file obbligatori** passano da 143 a **165 KB**, e
> **156 era già il vero**: la cifra vive in **cinque** posti, uno in `CLAUDE.md` e **quattro** in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), cercati col `grep` su **tutto il repository** come la
> sedicesima misura prescrive.
>
> 📌 **Cosa cambierebbe se qualcuno volesse che questo non ricapiti**, dichiarato invece che
> promesso: la §13 dice di aggiornare la §12 *«se cambia dove guardare»*, e una crescita di
> venti KB **non cambia dove guardare** — quindi la regola, letta alla lettera, non è stata
> violata da nessuno dei cinque compiti. ⛔ Il rimedio non è un'esortazione ma un **controllo**:
> `check-docs.sh` sa già confrontare due insiemi, e confrontare una cifra dichiarata con
> `wc -c` è lo stesso mestiere. Non è stato scritto qui perché sarebbe una riga di catalogo
> nuova, e quella è una decisione del proprietario — **registrata, non presa**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla nona
> applicazione.

> 🔁 **Diciottesima misura, il 2026-08-10, chiudendo il Traguardo 3 — ed è la prima passata in cui
> il metodo accumulato è stato eseguito PER INTERO e ha retto senza correzioni.** Scritta a
> **passata chiusa**, cioè dopo l'ultima riga di documentazione e non prima; righe contate
> **partendo dall'elenco dei file citati** e non dalle righe presenti — **diciotto** citazioni,
> **diciotto** righe, **nessuna assente**; `wc -c` **arrotondato** a KiB e non troncato;
> l'aggregato con le **due case**, cifra tonda e byte esatti.
>
> | | |
> |---|---|
> | **cresciuti** | [`porta-di-qualita.md`](../porta-di-qualita.md) `73 → 95` — di nuovo il salto più grande, ed è il registro che assorbe la chiusura di un traguardo · [`HANDOFF.md`](../HANDOFF.md) `159 → 175` per i gotcha **#53**, **#54**, **#55**, **#56** e le due tabelle di chiusura · il piano del Traguardo 3 `136 → 168` per le voci d'errata da **E50** a **E77** · [`riferimenti.md`](../riferimenti.md) `89 → 106` per le misure dei Task 9, 10, 11 e 12 · [`roadmap.md`](../roadmap.md) `19 → 22` · [`README.md`](../README.md) `12 → 13` · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `12 → 13`, che non si muoveva **da sedici misure** · questo file `156 → 183` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** — e l'invarianza è **il dato**, perché il Traguardo 3 ha toccato la spec l'ultima volta al Task 2 · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) · `design/` nove file `4–9` |
> | ⚠️ **una cella corretta DALLA MISURA e non dalla stima** | la riga del piano del Traguardo 3 era stata riscritta **prima** di misurare, con `162` preso dal piano gemello che gli somiglia. Il `wc -c` dice **168**. ⛔ È il gotcha **#15** dentro la passata che esiste per impedirlo: una cifra plausibile scritta prima della misura è un'ipotesi, e **somigliare a un file vicino non è misurarlo** |
> | ⚠️ **una cella di testo, non di cifre** | la riga del piano del Traguardo 3 diceva *«il compito da cui si riprende … i primi **otto** sono eseguiti, si riprende dal **Task 9**»* con **dodici** eseguiti: era stantia di quattro compiti, ed è la stessa specie che la dodicesima misura registrò per il piano del Traguardo 2. Ora dice cos'è quel file **adesso** — il modello di come si chiude un traguardo — che è la sola cosa che quella riga serve a dire quando il piano non è più in corso |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **649** a **665 KB** (681073 B). ⚠️ **Il
> denominatore è cresciuto senza che nessuno abbia scritto una riga di spec o un ADR:** i sedici
> kilobyte sono **tutti** di `HANDOFF.md`, cioè il costo di chiudere un traguardo, non di
> progettare.
>
> 📌 **E una cosa che questa passata dice e le altre diciassette no:** il rimedio proposto dalla
> diciassettesima — *«`check-docs.sh` sa già confrontare due insiemi, e confrontare una cifra
> dichiarata con `wc -c` è lo stesso mestiere»* — **non è stato scritto**, e in questa passata
> **sette** righe su diciotto erano di nuovo fuori. La proposta resta **registrata e non presa**:
> è una riga di catalogo nuova, e quella è una decisione del proprietario.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla decima
> applicazione.

> 🔁 **Diciannovesima misura, il 2026-08-10, chiudendo la sessione con l'audit di coerenza — ed è
> una passata NUOVA e non una correzione della diciottesima.** ⛔ **Dichiarato perché, ché la
> scelta era fra le due:** i numeri della diciottesima erano **veri a `07815a3`**, verificati uno
> per uno prima di toccare qualsiasi cosa; riscriverli avrebbe fatto descrivere a quel verbale uno
> stato che non ha mai misurato, che è il difetto della **nona** — *«una misura vera di un momento
> sbagliato è indistinguibile da una misura falsa per chi la legge dopo»*. Un verbale si aggiunge,
> non si riscrive; si corregge **di sole cifre** soltanto quando descrive **il file che lo
> contiene**, ed è l'unica correzione fatta qui.
>
> Scritta a **passata chiusa**; righe contate **partendo dall'elenco dei file citati** — **diciannove**
> bersagli, **venti** righe, perché `HANDOFF.md` ne ha due; **nessuna assente**. ⚠️ **La diciottesima
> diceva *«diciotto citazioni, diciotto righe»***, e la coppia era già di venti a diciannove: la
> verifica prescritta dalla quindicesima è stata **fatta**, e il suo esito **non è stato ricontato**.
>
> | | |
> |---|---|
> | **cresciuti** | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `13 → 16` — il salto più grande in proporzione, ed è il file che questa passata esisteva per rifare · [`HANDOFF.md`](../HANDOFF.md) `175 → 176` per le tre affermazioni di stato false e le due cifre d'errata · [`porta-di-qualita.md`](../porta-di-qualita.md) `95 → 96` per le date mancanti alle due campagne · `CLAUDE.md` `9 → 10` per il pre-controllo dei compiti · questo file `183 → 190` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · roadmap 22 · [`README.md`](../README.md) 13 · [`riferimenti.md`](../riferimenti.md) 106 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) · `adr/` 214 · `design/` nove file `4–9` |
> | ⚠️ **una cella che non è un peso** | quella di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) dice ora **due** numeri, il file e il **messaggio** dentro di esso: chi apre quel file per incollarlo non gliene importa dei sedici kilobyte, gliene importa dei **7,7** che finiscono nella chat. Il peso del file non rispondeva alla domanda per cui la §12 esiste |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **665** a **667 KB** (682795 B), corretto in
> **tutti e tre** i posti. I **due file obbligatori** passano da 192 a **200 KB**.
>
> 📌 **E la notizia di questa misura è che il rimedio proposto dalla diciassettesima è ora chiesto
> per la TERZA volta.** *«`check-docs.sh` sa già confrontare due insiemi, e confrontare una cifra
> dichiarata con `wc -c` è lo stesso mestiere»*: la diciottesima registrò che senza di esso sette
> righe su diciotto erano di nuovo fuori, e questa passata — che non ha eseguito **nessun compito**,
> solo riletto documenti — ne ha mosse **cinque**. ⛔ Resta **registrata e non presa**: è una riga di
> catalogo nuova, e quella è una decisione del proprietario. Ma il conto delle volte in cui sarebbe
> servita è ora scritto.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, all'undicesima
> applicazione.

> 🔁 **Ventesima misura, il 2026-08-11, chiudendo il brainstorming del Traguardo 4 — ed è la
> prima aperta da un BRAINSTORMING e non da un compito o da un audit.** Nessuna riga di codice
> scritta, nessun compito eseguito: solo un disegno e i documenti di stato che lo incassano. Ha
> mosso **sei** righe. Scritta a **passata chiusa**; righe contate **partendo dall'elenco dei file
> citati** — **venti** bersagli, **ventuno** righe, perché `HANDOFF.md` ne ha due; **nessuna
> assente**.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | il **disegno del Traguardo 4**, **23 KB** — il file da cui si riprende, e che si legge **prima** di scriverne il piano |
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `176 → 181` per il gotcha **#57**, il blocco del prossimo passo e due celle stantie · [`riferimenti.md`](../riferimenti.md) `106 → 110` per le **otto** misure del brainstorming · [`roadmap.md`](../roadmap.md) `22 → 23` · [`README.md`](../README.md) `13 → 14` · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `16 → 17`, che non si muoveva **da diciassette misure** e cresce oggi per il riconteggio delle proprie case · questo file `190 → 200` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · tracciabilità 15 · [`porta-di-qualita.md`](../porta-di-qualita.md) 96 · `design/08` 8 · `design/01` 4 · `design/` nove file `4–9` · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) |
> | ⛔ **una cella corretta DALLA MISURA, e per la seconda volta di seguito** | la riga del disegno del Traguardo 4 era stata scritta **`15 KB`** — un numero plausibile, buttato lì mentre si aggiungeva la riga. Il `wc -c` dice **23**. È esattamente il rilievo della **diciottesima** misura, ripetuto quattro riquadri dopo e da chi l'aveva letto: una cifra scritta prima della misura è un'ipotesi, e **una riga nuova nasce senza peso** finché qualcuno non lo misura |
>
> ⛔ **E la notizia di questa passata è di specie nuova: era stantio il conteggio di IN QUANTI
> POSTI VIVE UNA CIFRA.** La diciassettesima dichiara che l'aggregato *«vive in tre posti»* e la
> cifra dei due file obbligatori *«in cinque, uno in `CLAUDE.md` e quattro in `AVVIO-CHAT.md`»*.
> Cercate col `grep` **su tutto il repository** — che è il metodo che la sedicesima prescrive — le
> case sono **quattro** e **sei**: `AVVIO-CHAT.md` ne ha guadagnata una per ciascuna, e nessuno
> l'ha registrato perché **si contano le case una volta sola, quando si scrive il rimedio**.
> 📌 **La forma del #31 che ne esce:** il rimedio della sesta misura — *«si scrive dove qualcuno
> legge per decidere»* — porta con sé un **elenco delle case**, e quell'elenco è **esso stesso una
> cifra dentro una frase**. Chi rimisura non si fidi del numero di case scritto nel verbale
> precedente: lo **rifaccia col `grep`**, che costa un comando.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **667** a **675 KB** (691423 B), corretto in
> **tutte e quattro** le case. I **due file obbligatori** passano da 200 a **210 KB**, corretti in
> **tutte e sei**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla dodicesima
> applicazione.
>
> 📌 **E il rimedio proposto dalla diciassettesima è ora chiesto per la QUARTA volta.** Questa
> passata non ha eseguito nessun compito e ha comunque mosso **sei** righe su ventuno, più i due
> aggregati e i loro conteggi di case. Resta **registrata e non presa**: è una riga di catalogo
> nuova, e quella è una decisione del proprietario.

> 🔁 **Ventunesima misura, il 2026-08-11, chiudendo il piano del Traguardo 4 — ed è la SECONDA
> passata dello stesso giorno**, perché la voce si è chiusa due volte: il disegno e poi il piano.
> Scritta a passata chiusa; righe contate **partendo dall'elenco dei file citati** — **ventuno**
> bersagli, **ventidue** righe, perché `HANDOFF.md` ne ha due.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | il **piano del Traguardo 4**, **71 KB** — il compito da cui si riprende |
> | **cresciuti** | il **disegno del Traguardo 4** `23 → 27`, per il richiamo della §11 in cui il codice lo ha smentito · questo file `200 → 206` |
> | **invariati, ricontati** | [`HANDOFF.md`](../HANDOFF.md) 181 · [`riferimenti.md`](../riferimenti.md) 110 · [`roadmap.md`](../roadmap.md) 23 · [`README.md`](../README.md) 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 17 · [`porta-di-qualita.md`](../porta-di-qualita.md) 96 · spec del sotto-progetto 1 **277** · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · `design/` nove file `4–9` · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è che il difetto della ventesima si è ripetuto identico, alla PRIMA occasione
> utile — che è la TERZA volta di seguito.** La riga del piano è stata scritta **`47 KB`** mentre
> si aggiungeva, e il `wc -c` dice **71**. La diciottesima misura lo registrò per il piano del
> Traguardo 3 (`162` scritto, **168** misurato, *«somigliare a un file vicino non è misurarlo»*), la
> ventesima per il disegno (`15` scritto, **23** misurato), e questa per il piano. **Tre passate
> consecutive, lo stesso gesto**: si aggiunge una riga, la cella del peso vuole un numero, e la mano
> ne scrive uno **plausibile** invece di lasciarla vuota fino alla misura.
>
> 📌 **Il rimedio, e stavolta non è un'esortazione: una riga nuova nasce SENZA peso.** Si scrive la
> cella `—`, si chiude la passata, si misura, e si riempie insieme a tutte le altre. Una cella
> vuota è visibilmente incompleta; una cella con dentro un numero verosimile **non lo è**, ed è
> precisamente per questo che sopravvive al commit. È il gotcha **#43** — *«un valore d'esempio
> valido viene incollato così com'è: non si distingue da un dato»* — applicato al peso invece che
> allo SHA.
>
> ⚠️ **E l'errore cresce:** `15 → 23` era il 53 % in più, `47 → 71` è il 51 %. Non è una stima che
> migliora avvicinandosi al vero: è **sempre la stessa stima sbagliata**, fatta guardando un file
> vicino che non c'entra.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **675** a **676 KB** (692398 B) — l'unico
> movimento è la precisazione in [ADR-0032](../adr/0032-motore-di-persistenza.md) — corretto in **tutte
> e quattro** le case. I **due file obbligatori** passano da 210 a **216 KB**, corretti in tutte e
> sei.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla tredicesima
> applicazione.

> 🔁 **Ventiduesima misura, il 2026-08-11, chiudendo la sessione — ed è la TERZA dello stesso
> giorno**, perché la voce si è chiusa tre volte: il disegno, il piano, e la consegna al prossimo
> agente. Scritta a passata chiusa; righe contate partendo dall'elenco dei file citati —
> **ventuno** bersagli, **ventidue** righe, nessuna assente.
>
> | | |
> |---|---|
> | ✅ **nessuna riga aggiunta**, e il rimedio della ventunesima **non ha avuto occasione di fallire** | va detto invece che spacciato per successo: la ventunesima prescrive che *«una riga nuova nasce senza peso»*, e questa passata **non ne ha aggiunte**. La regola resta **non provata** |
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `181 → 184` per il gotcha **#58** e lo stato · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `17 → 19`, rifatto per la consegna · [`riferimenti.md`](../riferimenti.md) `110 → 112` per **D4-9** e **D4-10** · [`roadmap.md`](../roadmap.md) `23 → 24` per la riga del piano nella tabella dei piani · `CLAUDE.md` `10 → 11` · questo file `206 → 211` |
> | **invariati, ricontati** | [`README.md`](../README.md) 14 · [`porta-di-qualita.md`](../porta-di-qualita.md) 96 · [`tracciabilita.md`](../tracciabilita.md) 15 · il **disegno del Traguardo 4** 27 · il **piano del Traguardo 4** 71 · spec del sotto-progetto 1 **277** · kernel-design 44 · `design/08` 8 · `design/01` 4 · `design/` nove file `4–9` · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
> | ⛔ **e una riga di [`riferimenti.md`](../riferimenti.md) mancava, non un numero** | le misure **D4-9** e **D4-10** — quelle che hanno corretto il disegno — erano state scritte **nel disegno e nell'ADR** ma non nel file che raccoglie le misure, perché arrivarono **dopo** che quella sezione era già chiusa. È il difetto della **nona** misura — *«un verbale si scrive quando la passata è chiusa»* — nella forma in cui la passata si **riapre** |
>
> ⛔ **E la notizia di questa misura è il MESSAGGIO, che è la cifra per cui `AVVIO-CHAT.md`
> esiste.** Diceva **7,7 KB** in due posti; misurato ora è **9,8** — cresciuto del **27 %** in una
> sessione sola, perché la consegna al prossimo agente ha aggiunto il blocco dei due file da
> aprire, le sette decisioni del piano e il gotcha **#58**. ⚠️ **Il rapporto che quel file difende
> regge comunque — 9,8 KB di messaggio che ordinano 242 KB di lettura, contro 689 di corpus** —
> ma la crescita è il **prossimo candidato** al difetto che la dodicesima misura registrò per
> [`porta-di-qualita.md`](../porta-di-qualita.md): non un numero stantio, **un documento che smette di
> essere letto perché è diventato troppo lungo**. Un messaggio da incollare ha un limite naturale
> che una tabella non ha, ed è la pazienza di chi lo rilegge.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **676** a **689 KB** (695206 B), corretto in tutte
> e quattro le case. I **due file obbligatori** passano da 216 a **242 KB**, corretti in tutte e
> sei.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> quattordicesima applicazione.

> 🔁 **Ventitreesima misura, il 2026-08-11, chiudendo il Traguardo 4 — ed è la prima passata in
> cui a crescere di più non è `HANDOFF.md`.** Scritta a **passata chiusa**; righe contate
> **partendo dall'elenco dei file citati**, che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ⛔ **cresciuti, e il primo è il dato** | [`riferimenti.md`](../riferimenti.md) `112 → 143` — **+31 KB in un traguardo solo**, ed è dove sono finite le misure che hanno smentito il piano · [`porta-di-qualita.md`](../porta-di-qualita.md) `96 → 117` · [`HANDOFF.md`](../HANDOFF.md) `184 → 194` · il **piano del Traguardo 4** `71 → 114`, per le settanta voci d'errata · questo file `211 → 229` |
> | ⛔ **riga aggiunta** | [`semi-dst.md`](../semi-dst.md), **6 KB** — misurata **prima** di scrivere la cella, che è il rimedio della ventunesima e la **prima volta che ha avuto occasione di essere applicato** |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · il disegno del Traguardo 4 **27** · il piano del Traguardo 3 168 · kernel-design 44 · roadmap 24 · README 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 19 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è che il rapporto per cui la §12 esiste si è MOSSO, per la prima volta nella
> direzione sbagliata.** L'insieme *«HANDOFF + spec + `adr/`»* passa da **679** a **689 KB**, ma i
> **due file obbligatori** passano da **222** a **242** — cioè il denominatore è cresciuto
> dell'1,5 % e il numeratore del **8,1 %**. ⚠️ La ragione è che il Traguardo 4 ha prodotto
> soprattutto **decisioni e misure**, che vivono nel compendio, e poco perimetro nuovo, che vivrebbe
> nella spec. 📌 Non è un difetto oggi — 242 contro 689 regge — ma è la **prima misura in cui la
> lettura obbligatoria cresce più in fretta del corpus che risparmia**, e chi rimisura la prossima
> volta guardi quel rapporto e non solo le celle.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> quindicesima applicazione.

> 🔁 **Ventiquattresima misura, il 2026-08-11, chiudendo la sessione con la consegna al prossimo
> agente — ed è una passata di sola documentazione, come la tredicesima.** Scritta a passata
> chiusa; righe contate partendo dall'elenco dei file citati.
>
> | | |
> |---|---|
> | **cresciuti** | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `19 → 21`, rifatto per la consegna · [`roadmap.md`](../roadmap.md) `24 → 26` · [`riferimenti.md`](../riferimenti.md) `143 → 145` · [`HANDOFF.md`](../HANDOFF.md) `194 → 195` |
> | **invariati, ricontati** | questo file **231** · [`porta-di-qualita.md`](../porta-di-qualita.md) 117 · [`README.md`](../README.md) 14 · [`semi-dst.md`](../semi-dst.md) 6 · `CLAUDE.md` 11 · spec 277 · il disegno del Traguardo 4 27 · i piani 168, 162, 114, 50, 68 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è il MESSAGGIO, che è la cifra per cui quel file esiste: è passato da 9,8 a
> 12,2 KB, +24 % in una sessione — la SECONDA crescita a due cifre di seguito.** La ventiduesima
> misura lo aveva registrato come *«il prossimo candidato»* al difetto che la dodicesima trovò per
> il registro: non un numero stantio, **un documento che smette di essere letto perché è diventato
> troppo lungo**. 📌 **La consegna successiva COMPRIMA invece di appendere:** le lezioni di un
> traguardo chiuso da due traguardi si spostano nel compendio, dove chi legge le trova comunque, e
> il messaggio tiene solo ciò che serve a **non sbagliare il primo passo**. ⚠️ Un messaggio da
> incollare ha un limite che nessuna tabella ha, ed è la pazienza di chi lo rilegge.
>
> ✅ **L'aggregato è fermo — 689 → 690 KB — e i due file obbligatori pure, a 242.** Il rapporto che
> la §12 esiste per difendere non si è mosso in questa passata: a muoverlo era stato il traguardo,
> non la consegna.

> 🔁 **Venticinquesima misura, il 2026-08-11, chiudendo l'AUDIT COMPLETO — ed è la prima passata
> aperta da qualcosa che non è né un compito né una chiusura di traguardo.** Scritta a passata
> chiusa; righe contate **partendo dall'elenco dei file citati**, che è il movimento della
> quindicesima.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | [`audit-2026-08-11.md`](../audit-2026-08-11.md), **22 KB** — misurata **prima** di scrivere la cella, che è il rimedio della ventunesima alla **seconda** applicazione riuscita. ⚠️ Ed è messa **in cima** alla tabella e non in coda: è il **prossimo passo**, non un documento di consultazione |
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `196 → 203` per i gotcha **#59–#64** e il punto di ripresa · [`riferimenti.md`](../riferimenti.md) `145 → 152` per le misure dell'audit · [`porta-di-qualita.md`](../porta-di-qualita.md) `117 → 120` per i tre controlli riparati · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `21 → 24` · [`roadmap.md`](../roadmap.md) `26 → 27` · `CLAUDE.md` `11 → 12` · questo file `231 → 244` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · il disegno del Traguardo 4 30 · kernel-design 44 · [`README.md`](../README.md) 15 · tracciabilità 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 10 · `design/01` 4 · i piani 168, 162, 114, 68, 50 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
> | ✅ **`tracciabilita.md` NON è stato toccato, ed è una decisione** | l'audit non ha spostato nessuna funzionalità, quindi la mappa non cambia; e la §8 dice che si aggiorna **solo alla chiusura del sotto-progetto 1**. Le sue **171** funzionalità sono state **ricontate** — la cifra è giusta — e la crepa della legenda `📋` porta già i propri due riquadri |
>
> ⛔ **E la notizia di questa misura è il MESSAGGIO, che cresce per la QUARTA volta di seguito e
> per la terza a due cifre percentuali: 7,7 → 9,8 → 12,2 → 14,7 KB.** La 24ª aveva prescritto
> *«la consegna successiva COMPRIMA invece di appendere»*, e la compressione **è stata fatta** —
> le lezioni del Traguardo 4 e le tre forme del gotcha #48 sono state portate qui, dove chi legge
> le trova comunque. Ha recuperato **344 byte**, e il blocco dell'audit li ha più che compensati.
> 📌 **La regola che ne esce, e va scritta perché il rimedio della 24ª non è bastato:**
> *comprimere ciò che è vecchio non basta quando ciò che è nuovo pesa di più.* La prossima
> consegna deve decidere **cosa TOGLIERE**, non cosa accorciare — e il candidato naturale è il
> blocco delle decisioni ribaltabili, che è un rimando a due errata già scritte.
>
> ⛔ **E il rapporto che la §12 esiste per difendere si è mosso ancora nella direzione sbagliata,
> per la seconda misura di seguito.** L'insieme *«HANDOFF + spec + `adr/`»* passa da **691** a
> **698 KB**; la lettura obbligatoria passa da **242** a **256 KB**, e con l'audit — che ora è il
> **terzo file da leggere** — a **278**. Il denominatore è cresciuto dell'1 %, il numeratore del
> **15,7 %**. ⚠️ Non è un difetto oggi — 278 contro 698 regge — ma è la ragione per cui
> quel blocco va tolto e non accorciato.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> sedicesima applicazione.

> 🔁 **Ventiseiesima misura, il 2026-08-17, chiudendo la prima delle otto decisioni dell'audit —
> ed è la prima passata in cui il MESSAGGIO non è cresciuto.** Scritta a passata chiusa; righe
> contate **partendo dall'elenco dei file citati**, che è il movimento della quindicesima —
> ventidue bersagli, ventitré righe, perché `HANDOFF.md` ne ha due; nessuna assente.
>
> | | |
> |---|---|
> | ✅ **nessuna riga aggiunta** | nessun documento nuovo: la decisione 1 ha prodotto **codice di banco** e voci dentro documenti che esistevano. Il rimedio della ventunesima — *«una riga nuova nasce senza peso»* — **non ha avuto occasione di fallire**, e va detto invece che spacciato per successo |
> | **cresciuti** | [`porta-di-qualita.md`](../porta-di-qualita.md) `120 → 127` — il salto più grande, ed è il registro che assorbe tre bugiardi e la campagna delle sei mutazioni · [`riferimenti.md`](../riferimenti.md) `152 → 156` · [`HANDOFF.md`](../HANDOFF.md) `203 → 206` per il gotcha **#65** e il punto di ripresa · [`audit-2026-08-11.md`](../audit-2026-08-11.md) `22 → 25` per il richiamo in testa alla §5 · questo file `244 → 253` · [`README.md`](../README.md) `15 → 16` · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `24 → 25`, benché il **messaggio** dentro di esso sia fermo · `CLAUDE.md` `12 → 12` e [`roadmap.md`](../roadmap.md) `27 → 27`, mossi di poche righe |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · il disegno del Traguardo 4 30 · kernel-design 44 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 10 · `design/01` 4 · i piani 168, 162, 114, 68, 50 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ✅ **E LA NOTIZIA È IL MESSAGGIO, che per quattro misure di seguito era la cattiva.** La serie
> era 7,7 → 9,8 → 12,2 → 14,7 KB; oggi il blocco fra le due recinzioni passa da **15036 a 15040
> byte** — **`+4 B` e una riga** — pur avendo incassato una decisione intera dell'audit.
> ⛔ **Ha funzionato la prescrizione della venticinquesima, alla lettera:** *«la prossima consegna
> deve decidere cosa TOGLIERE, non cosa accorciare — e il candidato naturale è il blocco delle
> decisioni ribaltabili, che è un rimando a due errata già scritte»*. Quel blocco è stato **tolto**
> e ridotto a un rimando di sei righe, e il finding V6 — nel frattempo **chiuso** — è passato da
> diciotto righe a due. 📌 **La regola che ne esce, ed è la prima volta che questa serie ne produce
> una che funziona:** si toglie un **rimando duplicato** o una voce **chiusa**; una **lezione** non
> si toglie, si sposta nel compendio, dove chi legge la trova comunque.
>
> ⛔ **E le case sono state ricontate col `grep`, non riprese dal verbale precedente** — che è il
> rimedio della ventesima, e per la seconda volta il conteggio era stantio. La venticinquesima
> dichiarava **sei** case per la cifra dei due file obbligatori: sono **tre** (una in `CLAUDE.md`,
> due in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md)). L'aggregato ne ha **quattro**, e la cifra **coi tre
> file da leggere** altre **quattro**. 📌 L'elenco delle case è esso stesso una cifra dentro una
> frase, e invecchia come tutte.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **698** a **701 KB** (718041 B), corretto in tutte
> e quattro le case. I **due file obbligatori** passano da 256 a **265 KB**, e coi tre da 278 a
> **290**, corretti in tutte le loro.
>
> ⚠️ **Il rapporto che la §12 difende si è mosso ancora nella direzione sbagliata, per la terza
> misura di seguito:** il denominatore è cresciuto dello **0,4 %**, il numeratore del **2,9 %**.
> Molto meno delle due volte precedenti — la decisione 1 ha prodotto soprattutto **codice** — ma la
> direzione è la stessa, e chi rimisura guardi quel rapporto e non solo le celle.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> diciassettesima applicazione.

> 🔁 **Ventisettesima misura, il 2026-08-17, subito dopo la ventiseiesima — ed è una passata di
> sola documentazione, aperta per verificare che la precedente avesse finito. NON AVEVA FINITO.**
> Scritta a passata chiusa; cercata col `grep` invece che a memoria, che è l'unica ragione per cui
> ha trovato qualcosa.
>
> | | |
> |---|---|
> | ⛔ **quattro affermazioni di stato FALSE, tutte in [`HANDOFF.md`](../HANDOFF.md)** | tre righe dicevano *«il prossimo passo è il brainstorming del Traguardo 5»* — nella «In trenta secondi», nel blocco del lascito e sotto il titolo **«Prima cosa da fare»** — mentre il **Punto di ripresa**, settanta righe più su **nello stesso file**, diceva *«non il Traguardo 5»*. ⛔ **È il finding D-1 dell'audit, che le aveva lasciate:** quella passata corresse il Punto di ripresa e non le tre gemelle. Radice **R1** — *una correzione attraversa il documento in cui nasce, non gli altri* — e qui nemmeno tutto il documento |
> | ⛔ **e la ventiseiesima ha commesso R1 dentro la passata che chiudeva R1** | la cella di `journal_contract_real.rs` in [`porta-di-qualita.md`](../porta-di-qualita.md) ha avuto corretta l'**intestazione** (`dodici → quindici` test) e **non il corpo**, che ha continuato a dire *«nove bugiardi»*, *«dodici test»* e *«nove per corsa»* per un commit intero. Una cella lunga ha **due** posti in cui vive lo stesso numero, e chi corregge quello in cima non vede l'altro |
> | **altre tre voci stantie** | *«le sonde sono J1…J13»* nella riga di copertura del registro (sono **J1…J16**) · il peso dell'audit `22 → 25 KB` in `HANDOFF.md` · e in **questo file** un *«⏭️ il prossimo è il Task 2»* fermo da **dieci** compiti, dentro il racconto del Traguardo 4 |
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `206 → 208` · [`porta-di-qualita.md`](../porta-di-qualita.md) `127 → 128` · `CLAUDE.md` `12 → 13` per la **settima** domanda del pre-controllo · questo file `253 → 257` |
> | **invariati, ricontati** | [`riferimenti.md`](../riferimenti.md) 156 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 25 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 25 · [`roadmap.md`](../roadmap.md) 27 · [`README.md`](../README.md) 16 · spec **277** · disegno T4 30 · kernel-design 44 · tracciabilità 15 · `semi-dst.md` 6 · `design/08` 10 · `design/01` 4 · i piani 168, 162, 114, 68, 50 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è che una passata di verifica APERTA SUBITO DOPO ne ha trovate sei.** La
> ventiseiesima aveva fatto tutto ciò che la §13 pretende — compendio, handoff, riferimenti,
> registro, e i pesi rimisurati — e restava **falsa in quattro punti**, uno dei quali sotto il
> titolo *«Prima cosa da fare»*, che è la frase più autorevole del file più autorevole.
> 📌 **La regola che ne esce, e costa un comando:** quando si sposta il **prossimo passo**, non si
> corregge la riga che si ha davanti — si cerca **`grep '⏭️'` su tutti i documenti di stato** e si
> guardano **tutte** le case. Vale per il prossimo passo come la sesta misura lo scrisse per i
> pesi, e le case sono di più di quante ne ricordi chi ha appena scritto una di esse.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **701** a **703 KB**. I **due file obbligatori**
> passano da 265 a **270 KB**, e coi tre da 290 a **295**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> diciottesima applicazione.

> 🔁 **Ventottesima misura, il 2026-08-18, chiudendo la decisione 8 dell'audit (G-5) — ed è la
> prima passata in cui NESSUNA cella era stantia e lo strumento sbagliava lo stesso.** Scritta a
> passata chiusa; righe contate partendo dall'elenco dei file citati.
>
> ⛔ **LA NOTIZIA È IL METODO, NON I NUMERI: «`wc -c`» NON DICE QUALI BYTE, E QUESTO REPOSITORY SI
> LAVORA DA DUE MACCHINE.** Rimisurando con `wc -c` sull'albero di lavoro, **nove** celle
> risultavano fuori di 1–4 KB **su file non toccati da giorni** — la spec `277 → 281`,
> `kernel-design 44 → 45`, il disegno del Traguardo 4 `30 → 31`, tutti e cinque i piani. La
> lettura ovvia era il gotcha **#31** nella forma peggiore: *«invariati, ricontati»* dichiarato
> per tre verbali di seguito su un numero sbagliato, cioè il finding **D-6** dell'audit ripetuto.
>
> ✅ **Ed era falsa, e a smentirla è stata la FORMA dello scarto e non il suo valore:** era
> **uniforme**, e l'invecchiamento non lo è. I fine-riga di questo albero sono **CRLF**; le misure
> precedenti furono prese su byte **LF**. `wc -c` conta **un byte in più per riga**, quindi su un
> file di **3961** righe fa esattamente i quattro kilobyte dello scarto.
>
> | File | `wc -c` qui (CRLF) | senza i CR | La cella diceva |
> |---|---|---|---|
> | spec del sotto-progetto 1 | 281 | **276,9** | **277** ✅ |
> | kernel-design | 45 | **44,3** | **44** ✅ |
> | disegno del Traguardo 4 | 31 | **30,5** | **30** ✅ |
> | il piano degli spike | 70 | **67,9** | **68** ✅ |
> | i quattro piani dei traguardi | 52 · 165 · 171 · 115 | **50 · 162 · 168 · 114** | **50 · 162 · 168 · 114** ✅ |
>
> 📌 **Quindi nessuna cella era stantia, e il verbale precedente aveva ragione.** È il gotcha
> **#48** — *«il banco con cui misuri sbaglia»* — nella forma che si coglie: lo scarto uniforme.
> Un difetto di misura ha una **firma**, l'invecchiamento no; e la lezione che vale oltre il caso è
> che **prima di correggere nove celle si guarda se sbagliano tutte della stessa quantità
> relativa.** Nove correzioni sbagliate sarebbero state committate in un colpo solo.
>
> ⛔ **E resta una decisione APERTA, registrata e non presa:** la §12 dichiara il metodo come
> *«`wc -c`, arrotondati a KiB»*, e quella formula **non è machine-indipendente**. Su questa
> macchina l'albero è CRLF, e i due percorsi del repository in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md)
> dicono che le macchine sono **due**: chi rimisura dall'altra otterrebbe numeri diversi, li
> correggerebbe in buona fede, e le cifre **oscillerebbero per sempre** senza che nessuno dei due
> sbagli. La cura è **una riga di metodo** — i pesi si misurano sui byte **LF**, cioè
> `wc -c` meno i `CR` — ed è scritta qui invece che nella riga del metodo perché **cambiare il
> metodo della §12 è una decisione del proprietario**, come la guardia sui pesi che la
> diciassettesima misura ha chiesto quattro volte.
>
> | | |
> |---|---|
> | **cresciuti** (byte LF) | [`riferimenti.md`](../riferimenti.md) `156 → 162` — il salto più grande, ed è la sezione delle misure di G-5 · [`porta-di-qualita.md`](../porta-di-qualita.md) `128 → 130` per le sonde **N6** e **N7** · [`HANDOFF.md`](../HANDOFF.md) `208 → 209` · [`roadmap.md`](../roadmap.md) `27 → 28` · [`audit-2026-08-11.md`](../audit-2026-08-11.md) `25 → 26` per le due righe barrate della §8 · questo file `257 → 261` |
> | ⚠️ **e una cella stantia VERA, l'unica** | [`README.md`](../README.md) `15 → 16`. È l'unica delle dieci che lo scarto uniforme non spiegava, e si vedeva solo **dopo** aver tolto i CR: il difetto di strumento **nascondeva** il difetto di dato |
> | **invariati, ricontati** | spec **277** · kernel-design 44 · disegno T4 30 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 25 · tracciabilità 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 10 · `design/01` 4 · `design/` nove file `4–10` · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19584 B) |
>
> ⛔ **E il MESSAGGIO è cresciuto, dopo una passata sola di fermo: da 15040 a 15361 byte, +321 B
> e +2,1 %.** La compressione prescritta dalla 25ª **è stata fatta** — il blocco di T-2/T-1 è
> diventato un elenco di due voci — e non è bastata a pagare il blocco di G-5. ⚠️ Va detto
> invece che spacciato per stabilità: la 26ª aveva chiuso a `+4 B` e questa non ci è riuscita.
> 📌 La regola della 26ª regge lo stesso — *si toglie un rimando duplicato o una voce chiusa* — ma
> il candidato da togliere alla prossima consegna è ora **il blocco delle due decisioni chiuse**,
> che diventerà un rimando alla §6 appena una terza si chiude.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **703** a **704 KB**. I **due file obbligatori**
> passano da 270 a **279 KB**, e coi tre da 295 a **305**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> diciannovesima applicazione.

> 🔁 **Ventinovesima misura, il 2026-08-18, chiudendo la decisione 6 dell'audit (A-1, A-2, A-4,
> A-7) — ed è la SECONDA passata dello stesso giorno**, come la ventunesima: la voce si è chiusa
> due volte. Scritta a passata chiusa, in **byte LF** — che è il metodo che la ventottesima ha
> dovuto scoprire per non correggere nove celle giuste.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](../riferimenti.md) `162 → 165` per le misure dei quattro richiami · [`HANDOFF.md`](../HANDOFF.md) `209 → 210` · [`audit-2026-08-11.md`](../audit-2026-08-11.md) `26 → 27` per le tre righe barrate della §8 · `design/08` `10 → 11` e `design/01` `4 → 5`, che portano ora il proprio richiamo · questo file `261 → 268` |
> | ⛔ **e una riga di INTERVALLO che nessuno aggiorna mai** | `design/` nove file `4–10 → 4–11`: è la cella che invecchia in silenzio, perché un intervallo *sembra* sempre giusto. Ricontata su tutti e nove i file, non dedotta dai due che ho toccato |
> | **invariati, ricontati** | spec **277** · kernel-design 44 · disegno T4 30 · [`porta-di-qualita.md`](../porta-di-qualita.md) 130 · [`roadmap.md`](../roadmap.md) 28 · [`README.md`](../README.md) 16 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 25 · tracciabilità 15 · [`semi-dst.md`](../semi-dst.md) 6 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B in byte LF — ⚠️ con `wc -c` su questo albero il massimo dice **19584**, che sono i **293 CR** di quel file: la ventottesima, applicata) |
>
> ✅ **E il MESSAGGIO non si è mosso: 15361 byte, invariato.** Non per virtù ma per compenso — le
> tre correzioni di conteggio si annullano fra loro (`SEI → CINQUE` due volte, `la prima e
> l'ottava → la 1, la 6 e l'8` una). Va detto così invece di attribuirselo: la 26ª chiuse a
> `+4 B` **per una compressione decisa**, questa a `0` **per caso**.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **704** a **707 KB**. I **due file obbligatori**
> passano da 279 a **284 KB**, e coi tre da 305 a **311**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventesima applicazione.

> 🔁 **Trentesima misura, il 2026-08-18, chiudendo la decisione 5 (C-1) — TERZA passata dello
> stesso giorno, e volutamente CORTA.** In byte LF, a passata chiusa.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](../riferimenti.md) `165 → 167` · [`audit-2026-08-11.md`](../audit-2026-08-11.md) `27 → 28` · questo file `268 → 272` |
> | **invariati** | [`HANDOFF.md`](../HANDOFF.md) 210 · [`roadmap.md`](../roadmap.md) 28 — ⚠️ **e l'invarianza è il dato**: la cella del sotto-progetto 1 è stata **accorciata** mentre la si aggiornava, sostituendo l'elenco delle decisioni chiuse con un **rimando alla §6**. Ricopiarlo lì è ciò che ha fatto invecchiare quella riga **tre volte** · [`README.md`](../README.md) 16 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 25 · `CLAUDE.md` 13 · [`porta-di-qualita.md`](../porta-di-qualita.md) 130 · spec **277** · tutto il resto come alla ventinovesima |
>
> ⚠️ **Il messaggio: 15361 → 15369 byte, `+8 B`.** Il conteggio è passato a QUATTRO decisioni e il
> blocco delle chiuse ha guadagnato una voce: è il segnale che la 29ª aveva previsto — **il
> blocco delle decisioni chiuse va tolto e sostituito da un rimando alla §6** appena la quinta
> si chiude, o cresce di una voce per decisione fino alla fine.
>
> L'insieme resta **707 KB**. I **due file obbligatori** passano da 284 a **287 KB**, e coi tre da
> 311 a **315**.

> 🔁 **Trentunesima misura, il 2026-08-18, chiudendo la decisione 4 (PL-1) — QUARTA passata dello
> stesso giorno.** In byte LF, a passata chiusa.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](../riferimenti.md) `167 → 170` · [`porta-di-qualita.md`](../porta-di-qualita.md) `130 → 133` per la settima sonda di `file_journal.rs` · [`HANDOFF.md`](../HANDOFF.md) `210 → 211` · [`audit-2026-08-11.md`](../audit-2026-08-11.md) `28 → 29` · [`README.md`](../README.md) `16 → 17` · questo file `272 → 277` |
> | **invariati, ricontati** | [`roadmap.md`](../roadmap.md) 28 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 25 · `CLAUDE.md` 13 · spec **277** · kernel-design 44 · disegno T4 30 · tracciabilità 15 · [`semi-dst.md`](../semi-dst.md) 6 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⚠️ **Il messaggio: 15369 → 15367 byte, `−2 B`.** Il conteggio delle decisioni si è accorciato
> (`QUATTRO → TRE`) più di quanto il blocco delle chiuse sia cresciuto. ⛔ **Non è la compressione
> che la 29ª prescriveva**, è aritmetica: il blocco ha guadagnato **un'altra voce**, che è il quinto
> giro dello stesso meccanismo. La prescrizione resta **non applicata** e va detto così.
>
> 📌 **E questa passata ha prodotto una cifra di specie nuova per il registro: un conteggio che
> DIPENDE DAL SISTEMA.** `file_journal.rs` porta **sei** test su Windows e **sette** su Linux,
> perché il settimo è `cfg(unix)`. Dichiarato invece di sceglierne uno — ed è la stessa lezione
> della ventottesima, un piano sotto: **una misura ha bisogno di dire su quale macchina è stata
> presa**, o due lettori onesti ottengono due numeri e si correggono a vicenda per sempre.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **707** a **711 KB**. I **due file obbligatori**
> passano da 287 a **292 KB**, e coi tre da 315 a **321**.

> 🔁 **Trentaduesima misura, il 2026-08-18, chiudendo la decisione 3 (K-1 con B-1) — QUINTA
> passata dello stesso giorno.** In byte LF, a passata chiusa.
>
> | | |
> |---|---|
> | **cresciuti** | [`porta-di-qualita.md`](../porta-di-qualita.md) `133 → 140` per il verbale di K-1/B-1 · [`riferimenti.md`](../riferimenti.md) `170 → 174` · [`HANDOFF.md`](../HANDOFF.md) `211 → 213` per il gotcha **#66** · questo file `277 → 283` · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `25 → 26` |
> | **invariati, ricontati** | [`audit-2026-08-11.md`](../audit-2026-08-11.md) 29 · [`README.md`](../README.md) 17 · [`roadmap.md`](../roadmap.md) 28 · `CLAUDE.md` 13 · spec **277** |
>
> ⛔ **E questa passata ha trovato un difetto di METODO che i numeri nascondevano, sul banco di
> misura e non sui documenti.** La prima corsa dava l'audit a **28 KB** contro i **29** della 31ª,
> su un file che `git diff` dichiara **non toccato**. Non era il file: era **l'arrotondamento**.
> `29469` byte fanno `28,78` KB, che **tronca a 28** e **arrotonda a 29** — e la serie di queste
> trentadue misure arrotonda. ⚠️ **Il metodo non era scritto da nessuna parte**, quindi due lettori
> onesti ottengono due numeri e si correggono a vicenda per sempre: è la lezione della 31ª — *«una
> misura ha bisogno di dire su quale macchina è stata presa»* — sull'asse dell'**operazione**
> invece che della macchina. 📌 **Da qui in poi: byte LF, `int(n/1024 + 0.5)`.**
> ⛔ **E una seconda cifra falsa è stata colta prima di entrare nel documento**, che è il punto:
> la prima misura dell'insieme dava **743 KB** perché il glob `*sottoprogetto-1*` rastrellava
> **anche** il disegno del Traguardo 4. L'insieme è *«HANDOFF + LA spec + `adr/`»*, e la
> contro-prova che il glob fosse sbagliato era già lì: la spec da sola deve fare **277**, e faceva
> 308. Gotcha **#48**, quattordicesima occorrenza.
>
> ⚠️ **Il messaggio: 15367 → 15940 byte, `+573 B`** — la crescita più grande da quando si conta,
> ed è **tutta** nel blocco delle decisioni chiuse, che ha guadagnato la sua **terza** voce.
> ⛔ **La prescrizione della 29ª è ora dovuta e non più solo prevista:** quel blocco va **tolto e
> sostituito da un rimando alla §6**. Resta **non applicata**, e va detto così invece di rimandarla
> in silenzio una quinta volta.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **711** a **713 KB**. I **due file obbligatori**
> passano da 292 a **296 KB**, e coi tre da 321 a **325**.

> 🔁 **Trentatreesima misura, il 2026-08-18, chiudendo la decisione 2 (P-1) — SESTA passata dello
> stesso giorno.** In byte LF, arrotondati — `int(n/1024 + 0.5)`, il metodo fissato dalla 32ª.
>
> | | |
> |---|---|
> | **cresciuti** | [`COMPENDIO.md`](../COMPENDIO.md) `283 → 289` · [`HANDOFF.md`](../HANDOFF.md) `213 → 215` per il gotcha **#67** · [`porta-di-qualita.md`](../porta-di-qualita.md) `140 → 145` · [`riferimenti.md`](../riferimenti.md) `174 → 178` · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `26 → 27` |
> | **invariati, ricontati** | [`audit-2026-08-11.md`](../audit-2026-08-11.md) 29 · [`README.md`](../README.md) 17 · [`roadmap.md`](../roadmap.md) 28 · `CLAUDE.md` 13 · spec **277** |
>
> ⛔ **IL MESSAGGIO: 15940 → 16659 byte, `+719 B`, la crescita più grande mai registrata — e la
> prescrizione della 29ª è ora SCADUTA, non più dovuta.** Il blocco delle decisioni chiuse ha la
> sua **quarta** voce, e cresce di una a ogni chiusura come la 29ª aveva previsto **cinque passate
> fa**. ⚠️ **Va detto come sta:** non è stata applicata perché ogni singola passata la trovava più
> economica da rimandare che da eseguire, ed è precisamente il modo in cui un debito dichiarato
> resta dichiarato per sempre. 📌 **Con l'ultima decisione della §8 il blocco arriva a cinque, e a
> quel punto TOGLIERLO è il lavoro di chiusura dell'audit, non una rifinitura.**
>
> ⚠️ **E una cifra di questa passata NON è in KB e vale rileggerla:** i casi di `compile_fail`
> passano da **diciassette a diciotto**, ed è il primo caso nuovo dal Traguardo 2.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **713** a **715 KB**. I **due file obbligatori**
> passano da 296 a **303 KB**, e coi tre da 325 a **332**.

> 🔁 **Trentaquattresima misura, il 2026-08-18, chiudendo la decisione 7 — e con essa l'AUDIT
> INTERO. Settima passata dello stesso giorno.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | **cresciuti** | [`porta-di-qualita.md`](../porta-di-qualita.md) `145 → 156` — la passata che l'ha fatto crescere di più, e sono due sezioni: la decisione 7 e la **voce aperta consolidata** · [`riferimenti.md`](../riferimenti.md) `178 → 183` · questo file `289 → 295` |
> | **calato** | ⚡ [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `27 → 25` |
> | **invariati, ricontati** | [`HANDOFF.md`](../HANDOFF.md) 215 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 29 · [`README.md`](../README.md) 17 · [`roadmap.md`](../roadmap.md) 28 · `CLAUDE.md` 13 · spec **277** |
>
> ✅ **IL MESSAGGIO È CALATO PER LA PRIMA VOLTA DA QUANDO SI CONTA: 16659 → 14947 byte, `−1712 B`,
> cioè −10,3 %.** La serie era 7,7 → 9,8 → 12,2 → 14,7 → 15,0 → 15,4 → 15,9 → **16,7 KB**, sette
> crescite e un solo fermo. ⛔ **Ha funzionato la prescrizione della 29ª — applicata alla settima
> occasione, che è la prima in cui era DOVUTA e non solo prevista:** il blocco delle decisioni
> chiuse, arrivato a **quattro** voci e destinato alla quinta, è stato **tolto** e sostituito da
> una riga sola — *«il rimedio si prezza leggendo il codice, non il rapporto, e può risultare più
> piccolo, più grande o di specie diversa»* — più il rimando alla §6.
> 📌 **La lezione è sopravvissuta, il verbale no**, ed è la regola della 25ª: *si toglie un
> rimando duplicato o una voce chiusa, mai una lezione*. Le quattro chiusure per esteso stanno
> nella §6, che chi legge apre comunque.
> ⚠️ **E il messaggio ha cambiato SPECIE oltre che dimensione:** la skill che serve subito non è
> più `repo-audit` ma `brainstorming`, perché il prossimo passo non è più esecuzione ma lavoro
> creativo. Era la ragione per cui quel blocco esisteva; chiuso l'audit, la ragione è finita.
>
> L'insieme *«HANDOFF + spec + `adr/`»* resta **715 KB**. I **due file obbligatori** passano da
> 303 a **308 KB**, e coi tre da 332 a **337**.

> 🔁 **Trentacinquesima misura, il 2026-08-18, chiudendo la passata di coerenza sul puntatore
> al prossimo passo.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | **cresciuti** | questo file `295 → 304` · [`riferimenti.md`](../riferimenti.md) `183 → 188` per il censimento · [`audit-2026-08-11.md`](../audit-2026-08-11.md) `29 → 31`, i timbri delle tre decisioni che erano rimaste senza · [`roadmap.md`](../roadmap.md) `28 → 29` |
> | ⚡ **calato** | [`HANDOFF.md`](../HANDOFF.md) — quattro blocchi **tolti** e sostituiti da rimandi alla §6: `220527 → 220362 B`, che arrotondato resta **215 KB** |
> | **invariati, ricontati** | [`README.md`](../README.md) 17 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 25 · [`porta-di-qualita.md`](../porta-di-qualita.md) 156 · `CLAUDE.md` 13 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · spec **277** |
>
> ⛔ **E LA NOTIZIA NON È UNA CRESCITA: È CHE TRE CELLE DELLA TABELLA QUI SOPRA ERANO GIÀ
> STANTIE A `HEAD`, e nessuna delle tre l'ha fatta questa passata.**
> [`HANDOFF.md`](../HANDOFF.md) diceva **211** dove la 33ª misura aveva già registrato **215**;
> [`porta-di-qualita.md`](../porta-di-qualita.md) diceva **133** dove la 34ª aveva registrato
> **156**, uno scarto di **ventitré** kilobyte; [`riferimenti.md`](../riferimenti.md) diceva
> **170** dove la 34ª aveva registrato **183**.
> 📌 **È il difetto che la SESTA misura descrisse per esteso, e che da allora nessuno ha più
> cercato:** *«il riquadro e la tabella sono due posti, si aggiorna il primo, e il secondo
> continua a rispondere a chi deve decidere se aprire un file»*. Le tre celle sbagliate sono
> esattamente quelle che servono a **decidere se aprire**, ed erano fuori di 4, 13 e 23 KB.
> ⚠️ **E la diagnosi è la STESSA della passata che questo verbale chiude, su un oggetto
> diverso:** una cifra vive in due posti, si scrive nel verbale, e la tabella diverge in
> silenzio. Radice **R1**. 📌 Chi rimisura non legge i verbali: **rimisura i file**, e poi
> confronta con **tutte** le celle, non con l'ultimo riquadro.
>
> ⛔ **IL MESSAGGIO: 14947 → 15014 byte, `+67 B` e una riga** — quella che dice che l'audit è
> **chiuso** e non è più il compito. ✅ **È la crescita più piccola mai registrata**, ed è nel
> verso giusto: la 34ª aveva tolto 1712 byte, questa aggiunge un **fatto** senza riaprire il
> blocco che era stato chiuso. ⚠️ **Due celle che DESCRIVONO il messaggio erano stantie e sono
> rifatte:** la tabella *«Perché è così corto»* di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) era ferma
> al 2026-08-17 — *«da 15036 a 15040 byte»* — e la cella della §12 era ferma a due passate
> prima, *«15,0, ed è tornato a crescere, `+321 B`»*. Gotcha **#31**, e sono **due** case
> della stessa cifra.
>
> L'insieme *«HANDOFF + spec + `adr/`»* resta **715 KB**. I **due file obbligatori** passano
> da 308 a **317 KB**, e coi tre da 337 a **348**.

> 🔁 **Trentaseiesima misura, il 2026-08-18, chiudendo la manutenzione che il §13 pretende e
> che la passata precedente aveva saltato.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `215 → 219` — il testo integrale dei due gotcha nuovi, che è la voce più cara di questa passata · questo file `304 → 308` · `CLAUDE.md` `13 → 14`, la riga di metodo |
> | ⚡ **calato di un byte** | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), che resta **25 KB**: il **messaggio** passa da 15014 a **15013**, perché *«sessantasette»* è una lettera più lungo di *«sessantanove»* |
> | **invariati, ricontati** | [`riferimenti.md`](../riferimenti.md) 188 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · [`roadmap.md`](../roadmap.md) 29 · [`README.md`](../README.md) 17 · [`porta-di-qualita.md`](../porta-di-qualita.md) 156 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · spec **277** |
>
> ⛔ **E l'aggregato si muove per la prima volta da tre passate: 715 → 719 KB**, tutto da
> [`HANDOFF.md`](../HANDOFF.md). ⚠️ **La cifra vive in QUATTRO case** — la testa di questo file,
> `CLAUDE.md`, e **due** punti di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) — ricontate col `grep` e
> aggiornate tutte e quattro nella stessa passata. 📌 È la 20ª misura applicata a sé stessa:
> *le case si contano una volta sola, quando si scrive il rimedio*, e chi riconta non si fida
> del numero scritto nel verbale precedente.
>
> ⚠️ **I due file obbligatori passano da 317 a 322 KB, e coi tre da 348 a 353** — cioè la
> lettura d'avvio è cresciuta di **tre** kilobyte per incassare due gotcha e una riga di
> metodo. ⛔ **Va detto come sta, perché è un costo e non un guadagno:** la 25ª misura
> prescrive di **togliere**, e questa passata ha **aggiunto**. La differenza è che ciò che
> entra è una **lezione** — e la stessa 25ª dice che una lezione non si toglie mai: si toglie
> un rimando duplicato o una voce chiusa. Il rapporto che questo file difende resta
> **353 KB contro 719**.

> 🔁 **Trentasettesima misura, il 2026-08-18, chiudendo il brainstorming del Traguardo 5 e il
> suo disegno.** In byte LF, `int(n/1024 + 0.5)`, a passata chiusa; righe contate **partendo
> dall'elenco dei file citati**, che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ✅ **riga aggiunta** | il **disegno del Traguardo 5**, **31 KB** — misurata **prima** di scrivere la cella, che è il rimedio della ventunesima alla **terza** applicazione riuscita. ⚠️ Ed è messa **sopra** quella del Traguardo 4: è il file da cui si riprende, non un documento di consultazione |
> | **cresciuti** | questo file `308 → 320` · [`riferimenti.md`](../riferimenti.md) `188 → 192` per le quattro misure `D5` · [`HANDOFF.md`](../HANDOFF.md) `219 → 222` per il testo integrale del **#70** · [`roadmap.md`](../roadmap.md) `29 → 30` · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `25 → 26` · [`README.md`](../README.md) `17 → 18` |
> | **invariati, ricontati** | [`porta-di-qualita.md`](../porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · spec **277** · kernel-design 44 · disegno T4 **30** · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
> | ⚠️ **e una cella che questo riquadro aveva già dichiarato invariata** | [`README.md`](../README.md) `17 → 18`. Il riquadro era stato scritto **prima** di aggiungervi la riga del disegno, ed è il difetto della **nona** misura — *«un verbale si scrive quando la passata è chiusa»* — colto **prima di committare**, che è la metà del rimedio che la nona prescrive |
>
> ⛔ **E LA NOTIZIA È DI SPECIE NUOVA: UNA CIFRA PUÒ AVERE CASE FALSE, E UN `grep` CIECO LE
> CORREGGEREBBE.** Cercando le case di **353** — la cifra dei tre file da leggere — il `grep` ne
> riporta sei, e **due non sono pesi**: `audit-2026-08-11.md:222` e `riferimenti.md:1546` scrivono
> `COMPENDIO.md:353`, che è un **numero di riga**. Correggerle avrebbe rotto due citazioni esatte
> del finding **A-2** mentre si credeva di riallineare un peso.
> 📌 **Il contro-verso, e costa una lettura:** il `grep` trova le **candidate**, non le case. Ogni
> occorrenza si guarda in faccia prima di toccarla. ⛔ **Ed è il gotcha nuovo #70**, perché il
> difetto vive **dentro il rimedio** che la ventesima e la ventiseiesima misura prescrivono contro
> il #31 — non è il **#41** (là un filtro *esclude*, qui *include* roba di un'altra specie) né il
> **#48** (là sbaglia un banco di *misura*, qui lo strumento ha risposto giusto).
>
> ✅ **E le case sono state ricontate col `grep`, non riprese dal verbale precedente** — che è il
> rimedio della ventesima. La cifra dei **due file obbligatori** ne ha **tre** (una in `CLAUDE.md`,
> due in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md)); quella **coi tre file** ne ha **quattro** (una in
> `CLAUDE.md`, tre in `AVVIO-CHAT.md`); l'**aggregato** ne ha **quattro**. I riquadri della §12
> sono **verbali** e non si riscrivono.
>
> ⛔ **E QUESTO RIQUADRO DICEVA *«l'aggregato non si è mosso: resta 719»*, ED ERA FALSO — scritto
> prima di aver finito di scrivere.** `HANDOFF.md` entra nell'aggregato, e il testo integrale del
> gotcha **#70** ce l'ha messo dentro: l'insieme *«HANDOFF + spec + `adr/`»* passa da **719** a
> **721 KB** (738453 B). 📌 È la nona misura per la terza volta — *una misura vera di un momento
> sbagliato è indistinguibile da una misura falsa per chi la legge dopo* — e a coglierla è stata
> la rimisura prima del commit, che è l'altra metà di quel rimedio.
> I **due file obbligatori** passano da 322 a **334 KB**, e coi tre da 353 a **365**.
>
> ⚠️ **E il rapporto che la §12 difende si muove ancora nella direzione sbagliata**, per la quarta
> misura di seguito: il denominatore cresce dello **0,3 %**, il numeratore del **3,4 %**. La
> ragione è la stessa della ventitreesima — un lavoro creativo produce **decisioni**, che vivono
> qui, e poco perimetro nuovo, che vivrebbe nella spec.
>
> ⚠️ **E il messaggio è cresciuto di `+573 B`, da 15013 a 15586**, cioè `+3,8 %`. Due voci: la
> **specie** del lavoro è cambiata — non più creativo, ma la traduzione di un disegno in compiti,
> e la skill che serve subito con essa — e il gotcha **#70**. ⛔ **Nessuna compressione fatta**, e
> va detto così: il blocco delle lezioni dell'audit è già un rimando, e togliere una **lezione**
> la 25ª misura lo vieta.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventunesima applicazione.

> 🔁 **Trentottesima misura, il 2026-08-18, chiudendo la consegna alla sessione successiva — ed è
> una passata di sola coerenza, come la ventisettesima, aperta per verificare che la precedente
> avesse finito. NON AVEVA FINITO.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | ⛔ **una cella con DUE affermazioni false insieme** | [`roadmap.md`](../roadmap.md), tabella dei **dodici sotto-progetti**: *«il prossimo è l'ESECUZIONE DELL'AUDIT … ne restano tre»* e *«l'arbitro GPU riparte dopo, e si comincia dal brainstorming»*. Entrambe false, e la seconda lo è diventata **in questa stessa sessione** |
> | **cresciuti** | [`HANDOFF.md`](../HANDOFF.md) `222 → 223` per la seconda forma del **#70** · questo file `320 → 323` |
> | ⚡ **calato** | [`roadmap.md`](../roadmap.md) `30 → 29`: la cella riscritta è **più corta** di quella che portava il puntatore e i conteggi. Togliere costa meno che ricorreggere, anche in byte |
> | **invariati, ricontati** | [`riferimenti.md`](../riferimenti.md) 192 · [`README.md`](../README.md) 18 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 26 · `CLAUDE.md` 14 · [`porta-di-qualita.md`](../porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · spec **277** · il disegno del Traguardo 5 **31** |
>
> ⛔ **E LA NOTIZIA È COME QUELLA CASA È SOPRAVVISSUTA A DUE CENSIMENTI.** Il primo è la **passata
> di coerenza del 2026-08-18**, il cui verbale dichiara *«cinque documenti, nove case»* con
> `roadmap.md` **fra i cinque**: le case sistemate erano quelle della tabella dei traguardi, e
> questa vive in un'**altra tabella dello stesso file**. 📌 *Un censimento che trova il **file** non
> ha trovato le sue **case**.* Il secondo censimento è di **oggi**, e ha riportato la riga giusta —
> `roadmap.md:128` — che è stata scartata dopo **centoventi caratteri**, per via del filtro messo lì
> a rendere leggibile l'elenco. ⛔ **È la seconda forma del gotcha #70, nata mezz'ora dopo la
> prima:** là il difetto è nell'**uscita** del `grep`, qui nel **filtro** che gli si mette dopo. Una
> riga trovata da un censimento **si legge intera**, o il censimento non è stato fatto.
>
> ⚠️ **E la cella lo aveva scritto di sé stessa**, che è ciò che la rende utile: *«questa cella si
> limita al conteggio: ricopiarlo qui è ciò che ha fatto invecchiare questa riga tre volte»* — e lo
> ricopiava. Gotcha **#68**, la regola violata dentro il documento che la contiene, alla seconda
> occorrenza misurata. ✅ **Rimedio: tolto**, non ricorretto — il puntatore e i conteggi vivono
> nella §6, in un posto solo.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da 721 a **722 KB**. I **due file obbligatori** vanno
> da 334 a **337 KB**, e coi tre da 365 a **368**. ✅ **Il messaggio non si è mosso: 15586 byte** — questa
> passata non lo ha toccato.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventiduesima applicazione.

> 🔁 **Trentanovesima misura, il 2026-08-18, chiudendo il PIANO del Traguardo 5 — ed è la
> seconda passata dello stesso giorno**, come la ventunesima e la ventinovesima. In byte LF,
> `int(n/1024 + 0.5)`, a passata chiusa; righe contate **partendo dall'elenco dei file citati**,
> che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ✅ **riga aggiunta** | il **piano del Traguardo 5**, **174 KB** — misurato **prima** di scrivere la cella, rimedio della ventunesima alla **quarta** applicazione riuscita. ⚠️ Ed è messo **sopra** il disegno del Traguardo 4: è il file da cui si riprende |
> | **cresciuti** | [`riferimenti.md`](../riferimenti.md) `192 → 198` per le sette misure `P5` · questo file `323 → 335` · [`HANDOFF.md`](../HANDOFF.md) `223 → 226` per il testo integrale del **#71** · [`roadmap.md`](../roadmap.md) `29 → 30` |
> | **invariati, ricontati** | [`README.md`](../README.md) 18 — ⚠️ **toccato e invariato**, otto righe non bastano a muovere l'arrotondamento · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 26 · [`porta-di-qualita.md`](../porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · spec **277** · il disegno del Traguardo 5 **31** · disegno T4 30 · kernel-design 44 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E LA NOTIZIA È IL PIANO STESSO: NASCE PIÙ GRANDE DI QUANTO OGNI PIANO PRECEDENTE SIA MAI
> DIVENTATO.** I due più grandi sono il Traguardo 3 a **168 KB** e il Traguardo 2 a **162** —
> e quelle cifre sono **dopo** le rispettive errata, settantasette voci in nove passate e
> quarantanove in sei. Questo parte da **174** con l'errata **vuota**.
> 📌 **La causa è misurabile e non è prolissità:** il perimetro è *«l'arbitro intero»* (§0.1 del
> disegno), i compiti sono **tredici** contro dodici e dieci, e i vincoli globali di questo
> repository pretendono in **ogni** passo il codice per esteso, le mutazioni con la propria
> attesa, e il comando col proprio esito. ⚠️ **Va detto come sta invece di essere spiegato:** se
> l'errata cresce come le due precedenti, quel file passa i **200 KB** — e la §12 esiste per
> dire a chi legge **se aprire**, non per giustificare. La riga della tabella dice *«a compiti,
> mai intero»*, che è l'unica difesa che ha.
>
> ⚠️ **E il rapporto che la §12 difende si muove ancora nella direzione sbagliata, per la quinta
> misura di seguito:** il denominatore cresce dello **0,6 %**, il numeratore dell'**1,8 %**. La
> ragione è la stessa della ventitreesima e della trentasettesima — un lavoro di **disegno o di
> piano** produce decisioni e misure, che vivono qui, e poco perimetro nuovo, che vivrebbe nella
> spec.
>
> ⛔ **E QUESTA PASSATA HA CREDUTO DI AVER COLTO IL GOTCHA #10 E SI SBAGLIAVA — la divergenza si
> registra invece di allinearla all'attesa.** Lo strumento di edit ha davvero **normalizzato i
> fine-riga** di `COMPENDIO.md` e `roadmap.md` da LF a **CRLF**, misurato: `CR` da **0** a `2894`
> e `282`, cioè **ogni riga**. La conclusione scritta di getto era *«`git diff` dichiarerà
> duemilanovecento righe cambiate»*. ✅ **Misurata, è FALSA:** questo repository ha
> `core.autocrlf=true`, quindi git normalizza CRLF→LF entrando nell'indice, e il diff è rimasto
> **pulito** — sessantadue righe su un file di 2894.
> ⛔ **E il perché vale più dell'errore, perché dice DOVE la trappola morde davvero.** Censiti i
> blob committati: i file con `CR` nell'indice sono **quattro in tutto il repository**, e sono
> **tutti sorgenti Rust** — `crates/kernel/src/ports/process.rs` (291),
> `crates/kernel/tests/ports_are_implementable.rs` (971),
> `crates/kernel/tests/reactor_contract.rs` (669), `crates/platform/src/reactor.rs` (123).
> **Nessun documento.** Su un documento LF la normalizzazione è assorbita da `autocrlf`; su
> quei quattro **no**, ed è per questo che il caso di `G-5` — `sed -i` su un `Cargo.toml` — si
> vide e questo no.
> 📌 **La regola di `CLAUDE.md` regge e diventa più precisa:** *chi tocca uno di quei quattro file
> conserva i fine-riga di quel file, e li rimisura dopo*. ⛔ **E due dei quattro sono file che il
> piano del Traguardo 5 modifica al Task 4** — `ports/process.rs` e `ports_are_implementable.rs`
> — quindi l'avvertenza è scritta **dentro il piano**, dove serve, e non solo qui.
> 📌 **E la riga di `CLAUDE.md` nomina `sed -i`, mentre lo strumento colpevole qui è un altro:** la
> regola vale per **qualunque** strumento che riscriva un file, non per il comando che l'ha
> insegnata.
> ⚠️ **E un secondo tentativo è stato fermato dalla decima forma del #48:** una sostituzione via
> `python - <<'PY'` con testo non-ASCII **non ha trovato nulla** — lo stdin è decodificato nel
> codepage di sistema — ed è fallita sull'`assert` invece di applicare metà delle modifiche. La
> difesa era già scritta nel gotcha; a farla scattare è stato l'`assert`, non la memoria.
>
> ⛔ **Il MESSAGGIO: 15586 → 15741 byte, `+155 B` e `+1,0 %`** — e la crescita è **tutta di
> specie**, non di contenuto. Il messaggio ordinava *«traduci un disegno in un piano»* e ora
> ordina *«esegui un piano, un compito per volta»*: `writing-plans` e
> `subagent-driven-development` si **scambiano di posto** fra le skill che servono subito e
> quelle che serviranno, e `test-driven-development` sale, perché il lavoro che viene **è
> codice**. ✅ **Nessun blocco appeso:** le due voci che entrano ne sostituiscono due che escono,
> che è la regola della 26ª — *si toglie un rimando duplicato o una voce chiusa, mai una
> lezione*.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **722** a **726 KB** (742950 B). I **due file
> obbligatori** passano da 337 a **349 KB**, e coi tre da 368 a **380**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventitreesima applicazione.

> 🔁 **Quarantesima misura, il 2026-08-18, chiudendo la CONSEGNA alla sessione che eseguirà —
> ed è una passata di sola coerenza, come la ventisettesima e la trentottesima, aperta per
> verificare che il messaggio d'avvio reggesse per una specie di lavoro diversa. NON REGGEVA,
> in cinque punti.** In byte LF, `int(n/1024 + 0.5)`, a passata chiusa.
>
> | | |
> |---|---|
> | ⛔ **la falsità più cara** | il blocco delle **questioni aperte di QUALITÀ** elencava **sei** difetti — il turn limit ignorabile, quattro gruppi su cinque della conformità `reactor`, il finto filesystem sui `CheckpointId`, la via **A3**, il giornale a `0644`, `bincode` non mantenuto — e **tutti e sei sono chiusi dal 2026-08-18**, dalle otto decisioni dell'audit. Il messaggio ordinava al prossimo agente di credere aperti sei difetti che non lo sono |
> | **altre quattro stantie** | il peso dell'audit `29 → 31 KB` · *«i TRE piani più grandi 168, 162 e 114»*, dove il più grande è ora **174** · *«le QUATTRO domande»* del pre-controllo, mentre trenta righe più su lo stesso messaggio ne diceva **SETTE** — due cifre della stessa cosa **dentro lo stesso file** · e il blocco sul gotcha **#58** citava solo il caso del Traguardo 4 |
> | ⚡ **calato** | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `26 → 25` |
> | **cresciuto** | questo file `335 → 339` |
> | **invariati, ricontati** | `CLAUDE.md` 14 · [`HANDOFF.md`](../HANDOFF.md) 226 · [`riferimenti.md`](../riferimenti.md) 198 · [`roadmap.md`](../roadmap.md) 30 · [`README.md`](../README.md) 18 · il **piano del Traguardo 5** 174 · [`porta-di-qualita.md`](../porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · spec **277** · il disegno del Traguardo 5 31 |
>
> ✅ **IL MESSAGGIO: 15741 → 14460 byte, `−1281 B`, cioè `−8,1 %`** — ed è il **secondo** calo da
> quando si conta, dopo quello della 34ª. ⛔ **Ha pagato la regola della 26ª applicata alla
> lettera:** ciò che è stato tolto è il **riassunto dell'audit** — le due falle del cancello, il
> #59, ciò che l'audit ha trovato sano, ciò che lascia al Traguardo 5 — cioè un **rimando
> duplicato**, perché quel file è uno dei **tre che il messaggio ordina di leggere per intero**.
> ✅ **Le lezioni non sono state tolte ma spostate**, che è la distinzione della 26ª: il
> **#71** entra accanto al #58, e le **sette** domande del pre-controllo sostituiscono le quattro.
>
> ⛔ **E LA NOTIZIA DI QUESTA MISURA È CHE IL DIFETTO ERA GIÀ STATO SEGNALATO E NON CHIUSO.** Le
> sei questioni chiuse furono riportate al proprietario **all'apertura della sessione
> precedente**, confrontando il messaggio incollato con la §5 dell'audit — *«sei su sei»* — e la
> sessione ha poi scritto un piano, aggiornato nove documenti e committato **senza toccarle**.
> 📌 **La classe è nuova e vale oltre il caso: una divergenza SEGNALATA A VOCE non è una
> divergenza REGISTRATA.** Il verbale la conserva, la conversazione no — e chi l'ha detta è
> esattamente chi smette di cercarla, perché ricorda di averne parlato. ⚠️ **Non è la radice R1**,
> dove una correzione non attraversa gli altri documenti: qui la correzione **non è mai
> esistita**, è esistito solo il suo annuncio.
>
> L'insieme *«HANDOFF + spec + `adr/`»* resta **726 KB**. I **due file obbligatori** passano da
> 349 a **353 KB**, e coi tre da 380 a **384**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventiquattresima applicazione.

> 🔁 **Quarantunesima misura, il 2026-08-19, chiudendo il Task 5 del Traguardo 5 e la sessione —
> ed è la prima passata in cui la tabella era STANTIA PRIMA DI COMINCIARE per una ragione
> nominabile: la consegna precedente NON HA SCRITTO LA PROPRIA MISURA.** In byte LF,
> `int(n/1024 + 0.5)`, a passata chiusa; righe contate **partendo dall'elenco dei file citati**.
>
> ⛔ **La notizia, e non è una crescita.** Il commit `0a6f743` del 2026-08-19 — la consegna del
> Task 4 — ha toccato **cinque** documenti (`AVVIO-CHAT.md`, questo file, `HANDOFF.md`,
> `README.md`, `roadmap.md`) e **nessun riquadro di §12**. La 40ª misura è quindi rimasta
> l'ultima per una sessione intera, e sette celle erano fuori **prima** che questa passata
> scrivesse una riga. ⚠️ È la specie della **17ª** misura — *«nessuno li aveva rimisurati per
> cinque compiti»* — con una causa più precisa: là mancava l'occasione, qui l'occasione c'è
> stata ed è stata **saltata**. 📌 La §13 dice cosa aggiornare a ogni voce e **non nomina la
> §12**: è la stessa lacuna che la 17ª misura registrò, e il rimedio proposto allora — un
> controllo che confronti la cifra dichiarata con `wc -c` — è **chiesto ora per la quinta
> volta** e resta **registrato e non preso**, perché è una riga di catalogo nuova.
>
> | | |
> |---|---|
> | ⛔ **cresciuti, e il primo è il dato** | [`porta-di-qualita.md`](../porta-di-qualita.md) `156 → 210` — **+54 KB, il salto più grande mai registrato in questa serie**, e sono cinque compiti di verbali di mutazione · il **piano del Traguardo 5** `174 → 216`, che è l'errata da `E1` a `E39` · questo file `339 → 359` · [`HANDOFF.md`](../HANDOFF.md) `223 → 233` — il gotcha **#73** e la quarta forma del **#31**, per esteso · [`riferimenti.md`](../riferimenti.md) `192 → 198` · [`roadmap.md`](../roadmap.md) `29 → 31` · [`README.md`](../README.md) `18 → 19` · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `25 → 26` |
> | ⛔ **e un ESTREMO stantio, che nessuna crescita spiega** | `design/` diceva **`4–11 KB l'uno`** e il più piccolo dei nove è `03-run-durevoli.md` a **4915 byte**, cioè **5 KB**. ⚠️ **Non è invecchiamento: è il METODO VECCHIO rimasto in una cella.** Il `4` viene dal **troncamento**, che la **32ª** misura ha sostituito con `int(n/1024 + 0.5)` — e la 29ª, che quella riga l'aveva ricontata *«su tutti e nove i file»*, l'ha ricontata **col metodo di allora**. 📌 **Un cambio di metodo non riattraversa le celle già scritte**, ed è la radice **R1** applicata a un'unità di misura invece che a un fatto |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · disegno T5 **31** · disegno T4 30 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 25 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR **`2–19`** (2441 B e 19291 B) |
>
> ⛔ **E l'invarianza di [`riferimenti.md`](../riferimenti.md) SAREBBE stata il dato, e non lo è: è
> cresciuto di sei kilobyte che NON vengono dai cinque compiti.** Ricontato con `git log`: nessun
> commit del Traguardo 5 lo tocca. Le misure dei cinque compiti — decine, con comandi ed esiti —
> vivono **tutte** in [`porta-di-qualita.md`](../porta-di-qualita.md), accanto alla sonda che
> difendono, mentre la §13 di questo file e `CLAUDE.md` prescrivono *«le fonti e i comandi in
> `riferimenti.md`»*. ⚠️ **Registrata e non presa**, ed è la ragione: scegliere fra *«spostarle»* e
> *«cambiare la regola»* è del proprietario, e cominciare al quinto compito darebbe **due**
> convenzioni invece di una. 📌 È il gotcha **#68** — una regola non vincola chi la ospita — su una
> regola di `CLAUDE.md` invece che su una frase.
>
> ⛔ **IL MESSAGGIO: `14460 → 15326` byte, `+866 B` e `+6,0 %`, ed è la seconda crescita più
> grande della serie — NESSUNA COMPRESSIONE È STATA FATTA, e va detto così.** Due pezzi: `+327 B`
> li aveva già messi la consegna del Task 4 **senza scrivere la propria misura**, e `+539 B` li
> mette questa passata, in **tre** blocchi riscritti — che il quinto traguardo è in esecuzione (con
> il *«a che punto»* lasciato alla §6), che la scadenza del warning è **scattata e rispettata** e
> non va più cercata, e che il piano porta ora **due specie** di voci registrate, quelle del
> pre-controllo e quelle dell'esecuzione. La serie è 7,7 → 9,8 → 12,2 → 14,7 → 15,0 → 15,4 →
> 15,9 → 16,7 → 14,9 → 15,0 → 15,4 → 15,6 → 15,7 → 14,5 → **15,0 KB**.
> ⚠️ **E la prima stesura di questo capoverso diceva *«tolto il blocco delle tre cose da sapere
> prima del Task 5»*, che è FALSO:** quel blocco vive nella **§6 di questo file**, non nel
> messaggio, e a essere sostituito è **lì**. Corretto sulla misura invece che lasciato — gotcha
> **#15** dentro il verbale che esiste per impedirlo. 📌 **Il candidato da togliere alla prossima
> consegna** è il blocco delle *«sei cose che ribadisco»*, che è in buona parte un rimando a
> `CLAUDE.md` e alla §9: **registrato, non preso**, perché toglierlo è una scelta del proprietario
> su ciò che vuole rileggersi ogni volta.
>
> ⛔ **E il candidato al difetto della DODICESIMA misura non è più un candidato:**
> [`porta-di-qualita.md`](../porta-di-qualita.md) è a **210 KB**, cioè *«un documento che smette di
> essere letto perché è diventato troppo lungo»*. La dodicesima lo trovò a **47** e lo riportò a
> 40 accorciando **una** sezione sproporzionata; qui la sproporzione non c'è — cresce dove
> lavorano i compiti — quindi il rimedio di allora non si applica, e va detto invece di applicarlo
> male. **Registrato**, non preso.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **726** a **732 KB** (749967 B). I **due file
> obbligatori** passano da 353 a **373 KB**, e coi tre da 384 a **404**. ⚠️ Le case sono state
> ricontate col `grep`, non riprese dal verbale precedente.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> venticinquesima applicazione.

> 🔁 **Quarantaduesima misura, il 2026-08-19, chiudendo il Task 6 del Traguardo 5 — e la
> tabella era stantia in QUATTRO celle prima che questa passata scrivesse una riga.** In byte
> LF, `int(n/1024 + 0.5)`, a passata chiusa; righe contate **partendo dall'elenco dei file
> citati**, che è il movimento della quindicesima: nessuna riga da aggiungere.
>
> | | |
> |---|---|
> | ⛔ **cresciuti, e il primo è il dato** | [`porta-di-qualita.md`](../porta-di-qualita.md) `210 → 244` — **+34 KB da un compito solo**, ed è il registro che assorbe **quindici** mutazioni e otto sonde · il **piano del Traguardo 5** `216 → 241`, che è l'errata da `E40` a `E56` · questo file `359 → 370` · [`HANDOFF.md`](../HANDOFF.md) `233 → 236`, il testo integrale del gotcha **#74** |
> | ⛔ **e l'invarianza di [`riferimenti.md`](../riferimenti.md) è di nuovo il dato, non una notizia buona** | **198**, immobile: le decine di misure del Task 6 vivono **tutte** nel registro, accanto alla sonda che difendono, mentre la §13 di questo file e `CLAUDE.md` prescrivono *«le fonti e i comandi in `riferimenti.md`»*. È la stessa voce aperta della 41ª, con **una prova in più** e non con una risposta: scegliere fra *«spostarle»* e *«cambiare la regola»* resta del proprietario |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · disegno T5 **31** · disegno T4 30 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 26 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · `design/` nove file **`5–11`** · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR **`2–19`** (2441 B e 19291 B) |
>
> ⛔ **E LA NOTIZIA È UNA DIVERGENZA DI UNO, REGISTRATA INVECE CHE ALLINEATA.** La 41ª misura
> chiude dichiarando i **due file obbligatori** a **373 KB**; rimisurati **prima** di toccare
> qualunque cosa, col metodo che la 32ª ha fissato — byte LF, `int(n/1024 + 0.5)` — davano
> **372**. Uno di scarto, su una cifra che vive in tre case. ⚠️ **Non l'ho inseguita**: o la
> 41ª ha arrotondato per eccesso, o il file è calato di qualche centinaio di byte fra il suo
> verbale e questa passata, e distinguere le due cose costerebbe più di quanto valga la
> risposta. 📌 **Quel che conta è che sia SCRITTA:** un verbale che avesse ripartito da 373
> senza dirlo avrebbe fatto sparire in silenzio l'unica occasione di accorgersene.
>
> ⛔ **E IL PIANO HA PASSATO I 200 KB, che la 39ª misura aveva previsto in questi termini
> esatti:** *«se l'errata cresce come le due precedenti, quel file passa i 200 KB»*. È a
> **241**, al **sesto** compito su tredici, con l'errata a **cinquantasei** voci — cioè la
> previsione si è avverata a **meno di metà** del traguardo. ⚠️ La difesa resta quella scritta
> nella sua riga di tabella, *«a compiti, mai intero»*, ed è l'unica che abbia.
>
> ⚠️ **Il MESSAGGIO: `15326 → 15392` byte, `+66 B` e `+0,4 %`** — la crescita più piccola dopo
> quella della 35ª, e sono **solo cifre più tre parole**: il conteggio dei gotcha, tre pesi, e
> *«TRE decisioni di disegno»* diventato *«CINQUE»* con la nota che due sono **dichiarate non
> tenute da nessuna sonda**. ⛔ **Nessuna compressione fatta, e va detto così:** il candidato da
> togliere resta quello che la 41ª ha nominato — il blocco delle *«sei cose che ribadisco»*, che
> è in buona parte un rimando a `CLAUDE.md` e alla §9 — e resta **registrato e non preso**,
> perché toglierlo è una scelta del proprietario su ciò che vuole rileggersi ogni volta.
>
> ⛔ **E QUESTA PASSATA HA DATO LA DECIMA FORMA DEL #48 DUE VOLTE, e la seconda è la notizia.**
> Lo strumento che ha riallineato le quattordici case dei pesi su tre file è morto **stampando
> il proprio verbale** — `UnicodeEncodeError`, console `cp1252`, testo non-ASCII — con **exit
> 1**. ⚠️ **Le scritture erano già finite tutte e tre**, perché la stampa veniva dopo; ma *«è
> morto dopo aver scritto»* e *«è morto a metà»* sono **indistinguibili dall'errore**, che è
> esattamente il gotcha **#69**. ✅ **Chiusa contando le sostituzioni invece di dedurle:**
> quattordici pattern vecchi cercati su tutti e tre i file → **zero** occorrenze, quattordici
> nuovi → **una** ciascuno.
> ⛔ **E POI È SUCCESSO DI NUOVO, NEL COMANDO SUBITO DOPO — quello che correggeva le cifre di
> QUESTO riquadro — e stavolta la regola era già scritta sopra.** Stesso schema, causa diversa:
> il verbale finiva in un percorso in forma Git-Bash (`/c/Users/...`) che il Python di Windows
> non apre, `FileNotFoundError`, **exit 1**, e le dieci scritture di nuovo **già fatte**.
> 📌 **La regola del #69 non è che il verbale vada scritto in un posto migliore: è che lo
> strumento che muta file NON RIFERISCE AFFATTO.** Il conteggio delle sostituzioni si fa **dopo**
> e **da fuori**, con `grep`, che è ciò che ha chiuso entrambe le volte. ⚠️ **La seconda
> occorrenza è arrivata dopo che avevo scritto la prima in questo stesso riquadro**, ed è il
> gotcha **#68** applicato a un verbale invece che a un documento: una regola non vincola chi la
> sta scrivendo.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **732** a **735 KB** (752701 B), corretto in
> tutte e quattro le case, ricontate col `grep` e **guardate in faccia** una per una (#70). I
> **due file obbligatori** passano da 372 a **384 KB**, e coi tre da 404 a **415**.
>
> ⚠️ **E il rapporto che la §12 difende si muove nella direzione sbagliata per la SESTA misura
> di seguito:** il denominatore cresce dello **0,4 %**, il numeratore del **2,7 %**. ⛔ La
> ragione stavolta **non** è quella della 23ª e della 37ª — non è un lavoro di disegno che
> produce decisioni: è un compito di **codice** che ha prodotto un gotcha nuovo e una tabella
> di previsioni misurate, cioè materia che vive **qui** per costruzione. 📌 Il rapporto regge —
> **415 contro 735** — ma la direzione è la stessa da sei passate, e chi rimisura la prossima
> volta guardi quella e non solo le celle.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo**
> aver chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventiseiesima applicazione.

> 🔁 **Quarantatreesima misura, il 2026-08-19, chiudendo la CONSEGNA alla sessione che eseguirà
> il Task 7 — ed è una passata di sola coerenza sul messaggio d'avvio, come la 27ª, la 38ª e la
> 40ª, aperta per verificare che REGGESSE invece di riscriverlo a memoria. NON REGGEVA, in due
> punti.** In byte LF, `int(n/1024 + 0.5)`, a passata chiusa.
>
> | | |
> |---|---|
> | ⛔ **una cifra FALSA, e vive in due posti** | il messaggio prezzava il gotcha **#48** *«a TREDICI occorrenze»*, mentre la 32ª misura di questa stessa §12 ne registrava già **quattordici** il 2026-08-18 — e questa sessione ne ha aggiunte **altre due**, entrambe della decima forma. ✅ **Tolta, non ricorretta:** un conteggio che vive in due documenti marcisce sempre in quello che non lo muove, e il messaggio non muove nulla. Resta il rimando alla §9, che le forme le tiene per esteso |
> | ⚠️ **una frase STANTIA sulla propria data** | il blocco di `subagent-driven-development` diceva *«SERVE SUBITO, ed è CAMBIATO rispetto alla sessione scorsa»*: la specie del lavoro è la stessa da **due** sessioni, quindi quel *«cambiato»* era vero il giorno in cui fu scritto e falso da allora. Tolto |
> | **cresciuti** | questo file `370 → 376` · [`HANDOFF.md`](../HANDOFF.md) `236 → 237`, la terza occorrenza del **#45** per esteso |
> | **invariati, ricontati** | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) **26** — e l'invarianza del *file* nasconde un movimento del **messaggio**, che è la cifra per cui quel file esiste: vedi sotto · [`porta-di-qualita.md`](../porta-di-qualita.md) 244 · [`riferimenti.md`](../riferimenti.md) 198 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · `CLAUDE.md` 14 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · spec **277** · il piano del Traguardo 5 241 · disegno T5 31 |
>
> ⛔ **IL MESSAGGIO: `15392 → 15940` byte, `+548 B` e `+3,6 %`, ed è una crescita DELIBERATA — si
> scrive perché non sia scambiata per una svista.** Ciò che entra è **un'istruzione**, non un
> racconto: *la revisione non è una formalità, e un'ONDATA DI CORREZIONI si rivede come il
> compito, perché i suoi difetti stanno nella PROSA*. ⚠️ **Senza quella riga il prossimo agente
> rifarebbe ciò che ho quasi fatto io** — saltare la revisione di un'ondata di soli commenti con
> l'argomento *«non tocca righe eseguibili»* — ed è il gotcha **#45**, terza occorrenza.
> ✅ **E la compressione è stata fatta, per la regola della 26ª e non a occhio:** sono usciti un
> **rimando duplicato** — il riassunto dell'audit, in un messaggio che ordina di leggere quel
> file per intero — e una **voce chiusa**, la scadenza di `E10`, che era scattata due compiti
> prima e diceva *«non cercarlo più»* a chi non lo cercava più. Senza quei due tagli la crescita
> era `+984 B`. ⚠️ **E i due tagli hanno lasciato un riferimento PENZOLANTE**, colto rileggendo
> e non da un controllo: tolto il riassunto dell'audit, il blocco *«COSA HANNO INSEGNATO LE
> OTTO»* restava senza antecedente. Chiuso nominandole. 📌 **Togliere un blocco costa una
> rilettura di ciò che lo circonda**, ed è il prezzo che la 26ª misura non nomina. ⛔ **Il candidato che resta è quello che la 41ª e la 42ª hanno già nominato** — il
> blocco delle *«sei cose che ribadisco»*, in buona parte un rimando a `CLAUDE.md` e alla §9 — e
> resta **registrato e non preso**, perché è una scelta del proprietario su ciò che vuole
> rileggersi ogni volta.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **735** a **737 KB** (754560 B), corretto in
> tutte e quattro le case. I **due file obbligatori** passano da 384 a **390 KB**, e coi tre da
> 415 a **421**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventisettesima applicazione.

> 🔁 **Quarantaquattresima misura, il 2026-08-20, chiudendo il Task 7 del Traguardo 5 — e due
> soli file hanno mosso NOVANTUNO kilobyte, da un compito solo.** In byte LF, `int(n/1024 + 0.5)`,
> a passata chiusa; righe contate **partendo dall'elenco dei file citati** e non da quelle
> presenti, che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ⛔ **cresciuti, e i primi due sono il dato** | [`porta-di-qualita.md`](../porta-di-qualita.md) `244 → 288` — **+44 KB da un compito solo**, ed è il registro che assorbe una campagna di **ventisei** righe, rimisurata **da capo due volte** e in parte una terza · il **piano del Traguardo 5** `241 → 288`, che è l'errata da `E57` a `E86`, **trenta voci** · questo file `376 → 394` · [`HANDOFF.md`](../HANDOFF.md) `237 → 242`, il testo integrale del gotcha **#75** e l'intestazione riscritta · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `26 → 27` |
> | ⛔ **e l'invarianza di [`riferimenti.md`](../riferimenti.md) è per la TERZA volta di seguito il dato** | **198**, immobile, mentre il registro cresceva di quarantaquattro. È la voce aperta della 41ª e della 42ª con **la prova più forte finora** e non con una risposta: scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario, e questa passata **non ha toccato il file**, deliberatamente — cominciare a spostarle produrrebbe **due** convenzioni invece di una |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · disegno T5 31 · disegno T4 30 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · `design/` nove file **`5–11`** · gli altri piani 68, 50, 162, 168, 114 · ADR **`2–19`** |
>
> ⛔ **E LA NOTIZIA È UN SORPASSO CHE FALSIFICA UNA FRASE DEL MESSAGGIO D'AVVIO: il PIANO ha
> superato la SPEC.** Il messaggio diceva del piano *«è il file più grande del repository dopo la
> spec»*, ed era falso in **due** direzioni insieme: il piano è a **288** e la spec a **277**,
> quindi non viene dopo di lei; e il file più grande del repository non è nessuno dei due ma
> **questo**, a **394**. ⚠️ **La seconda metà era falsa da prima di questa passata** e nessuno
> l'aveva colta, perché una frase che nomina un primato si rilegge cercando **il primo**, mai il
> secondo. ✅ **Riscritta, non affiancata** (finding **A-2**): il piano è ora dichiarato per
> quello che è — **il secondo file del repository**, dopo il compendio.
>
> ⛔ **E MISURANDO SONO USCITE DUE CELLE STANTIE CHE NON ERANO NEL PERIMETRO, in
> [`porta-di-qualita.md`](../porta-di-qualita.md): il conteggio delle sonde del modulo `arbiter`
> diceva DODICI dove sono TREDICI.** Entrambe erano ferme alla **prima** delle tre ondate del
> 2026-08-20, mentre una **terza** cella dello stesso file portava il numero giusto: la cifra
> viveva in **tre** posti dentro **un** documento. ✅ **Tolta dalle due, non riallineata** — il
> conteggio resta nella sezione del Task 7, in un posto solo — che è il gotcha **#68** applicato
> dentro un file invece che fra file. 📌 È la ragione per cui questa tabella si compila
> **contando**, e non copiando il verbale precedente.
>
> ⚠️ **Il MESSAGGIO: `15940 → 16637` byte, `+697 B` e `+4,4 %`**, e ciò che entra sono **cifre**
> più due istruzioni. La prima: *al Task 7 sono servite TRE revisioni piene, e tutte e tre hanno
> trovato difetti reali NELLA PROSA — il numero di giri non si decide dalla specie del diff*, che
> è il gotcha **#45** alla quarta occorrenza e senza la quale il prossimo agente rifarebbe la
> stessa scelta. La seconda è il **mutante vivo garantito**, che va distinto da una politica
> aperta perché nessuna decisione del proprietario lo chiuderà. ⛔ **Nessuna compressione fatta,
> e va detto così:** il candidato resta quello che la 41ª, la 42ª e la 43ª hanno già nominato — il
> blocco delle *«sei cose che ribadisco»* — e resta **registrato e non preso**, perché è una
> scelta del proprietario su ciò che vuole rileggersi ogni volta.
>
> ✅ **Il censimento del `⏭️` è stato fatto col `grep` su `docs/` e `CLAUDE.md`, e ogni riga
> trovata è stata letta INTERA** (gotcha **#70**, seconda forma): **ventuno** righe, di cui
> **una sola** è il puntatore vivo — quella in cima alla §6 — **sei** sono rimandi che nominano
> la §6 **senza** nominare il passo ([`HANDOFF.md`](../HANDOFF.md) ×4, [`README.md`](../README.md),
> [`roadmap.md`](../roadmap.md)), e le altre **quattordici** sono testo di gotcha, verbali storici,
> la riga *registrata e non presa* sulla guardia che manca, un comando in
> [`riferimenti.md`](../riferimenti.md) e due righe del piano. **Nessuna casa da riallineare** —
> ⚠️ e questo si scrive come **esito di una lettura**, non di un conteggio di righe del `grep`,
> che è ciò che il **#70** pretende.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **737** a **741 KB**, corretto in tutte e cinque
> le case — ricontate col `grep` e guardate in faccia una per una: due in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), due in `CLAUDE.md`, una in questo file, più il verbale della
> 43ª che **non** si tocca perché è la misura di allora. I **due file obbligatori** passano da 390
> a **408 KB**, e coi tre da 421 a **439** — tre case per il primo, quattro per il secondo.
> ⚠️ **E ENTRAMBE le cifre aggregate sono a poche centinaia di byte dal confine
> dell'arrotondamento**, misurato chiudendo il riquadro: correzioni di **poche righe** le hanno
> fatte oscillare — `438`/`439` e `407`/`408` — mentre nessun file cresceva davvero. Si scrive
> perché la prossima passata non legga come **crescita** un `+1` che è soltanto
> `int(n/1024 + 0.5)` che scavalca la soglia.
>
> ⚠️ **E il rapporto che la §12 difende si muove nella direzione sbagliata per la SETTIMA misura
> di seguito:** il denominatore cresce dello **0,5 %**, il numeratore del **4,4 %**. ⛔ La causa
> è la stessa della 42ª — un compito di **codice** che produce un gotcha nuovo e un racconto di
> misure, cioè materia che vive **qui** per costruzione — e il rapporto regge, **439 contro 741**,
> ma sette passate nella stessa direzione non sono più un accidente.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventottesima applicazione.

> 🔁 **Quarantacinquesima misura, il 2026-08-20, chiudendo il Task 8 del Traguardo 5 — e il file
> che cresce di più non è più il registro: è il PIANO.** In byte LF, `int(n/1024 + 0.5)`, a
> passata chiusa; righe contate **partendo dall'elenco dei file citati** e non da quelle presenti,
> che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ⛔ **cresciuti, e i primi due sono il dato** | il **piano del Traguardo 5** `288 → 334` — **+46 KB da un compito solo**, ed è l'errata da `E87` a `E112`, **ventisei** voci · [`porta-di-qualita.md`](../porta-di-qualita.md) `288 → 323`, **+35**, il registro che assorbe una campagna **rieseguita per intero due volte in un giorno** · questo file `394 → 412` · [`HANDOFF.md`](../HANDOFF.md) `242 → 245`, il testo integrale del gotcha **#76** · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `27 → 28` |
> | ⛔ **e l'invarianza di [`riferimenti.md`](../riferimenti.md) è per la QUARTA volta di seguito il dato** | **198**, immobile, mentre gli altri due grandi crescevano di ottantuno kilobyte fra loro. La voce aperta della 41ª, 42ª e 44ª con una prova in più — la campagna del Task 8 è stata **rieseguita da capo due volte**, e in una di quelle riesecuzioni **lo strumento di misura ha sbagliato** — e non con una risposta: scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario, e questa passata **non ha toccato il file**, deliberatamente |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · disegno T5 31 · disegno T4 30 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · `design/` nove file **`5–11`** · gli altri piani 68, 50, 162, 168, 114 · ADR **`2–19`** |
>
> ⛔ **E LA NOTIZIA È CHE `288` AVEVA SMESSO DI IDENTIFICARE UN FILE.** Alla 44ª misura il registro
> **e** il piano leggevano **entrambi** `288`, e la cella li elencava così, uno accanto all'altro:
> oggi sono **323** e **334**. ⚠️ **Finché coincidevano, un `grep '288'` restituiva case di due
> specie diverse senza modo di distinguerle** — è il **#70** nella sua forma più pura, non un numero
> nudo pescato per un'altra ragione ma lo **stesso** numero che nomina due cose. ✅ È il motivo per
> cui in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) la cifra del piano è stata **ridotta a una casa** invece
> che riallineata in tre: ne aveva **tre**, e due lo usavano come **nome** del file da aprire
> (*«il piano da eseguire è il 288»*) invece che come peso. Il piano è ora nominato per
> **posizione** — *il più grande dei quattro* — che è vero mentre cresce.
>
> ⛔ **E QUESTA PASSATA HA TOLTO CIFRE DA UNA SECONDA CASA IN CINQUE PUNTI, invece di
> riallinearle**, che è il gotcha **#68** e la prescrizione della 25ª: ① il conteggio delle **sonde
> per banco** dalla §6 di questo file — [`porta-di-qualita.md`](../porta-di-qualita.md) ne **rivendica
> l'unicità** in una propria cella, quindi la copia qui rendeva **falsa** quella rivendicazione;
> ② il numero dei **gotcha** da [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), che ora rimanda al titolo della
> §9; ③ le due case di troppo del peso del piano, sopra; ④ il **peso del messaggio** dalla tabella
> *«Perché è così corto»*. ⚠️ **Le prime tre non erano ancora sbagliate** — tolte **prima** che
> marcissero, che è la sola volta in cui toglierle non costa una correzione. ⛔ **La quarta invece
> lo era già, da TRE passate:** quella cella diceva *«14,7 KB»* mentre la 42ª, la 43ª e la 44ª
> misura rimisuravano il messaggio in questa §12 e nessuna delle tre l'aveva guardata. 📌 **È la
> radice R1 al contrario:** non una correzione che non attraversa gli altri documenti, ma un
> **verbale** che rimisura fedelmente in casa propria e non sa di avere un gemello fuori.
> ⛔ **⑤ E LA QUINTA NON È UNA CIFRA: È UN NUMERATORE SCRITTO A PAROLE.** [`README.md`](../README.md)
> enumerava ciò che l'esecuzione del Traguardo 5 aveva portato — *«il modulo `arbiter` con `Mib` …
> e le code per corsia»* — e la lista si **fermava al Task 6** con otto compiti chiusi, **due righe
> sopra** la frase che dichiara *«a che punto sia non è scritto qui»*. La passata del 2026-08-18
> aveva tolto da quella riga la **cifra** e lasciato l'**elenco**. 📌 *Un elenco che si allunga a
> ogni compito è un numeratore, e invecchia peggio di uno in cifre perché non c'è nessun numero da
> riconoscere come stantio.* **Tolto, non esteso.** ✅ [`roadmap.md`](../roadmap.md) è stato guardato
> per lo stesso difetto e **non ne aveva**: le sue tre celle del Traguardo 5 rimandano alla §6
> senza enumerare, quindi **non è stato toccato**.
>
> ⚠️ **Il MESSAGGIO: `16637 → 17314` byte, `+677 B` e `+4,1 %`** — quasi identico al `+697` della
> 44ª. Entrano **due** istruzioni operative e una cifra: il **rimedio misurato** del gotcha **#76**
> (*all'ondata si dice TOGLIERE, non riscrivere meglio* — quattordici righe messe contro
> diciassette tolte, e il giro dopo pulito), la **revoca di una mutazione per ripristino e non per
> risostituzione** (forma del **#48** che è costata sette misure buttate al Task 8), e il peso del
> piano. ⛔ **Escono** il conteggio dei gotcha e due case del peso del piano. ⛔ **Nessuna
> compressione fatta, e va detto così:** il candidato resta quello che la 41ª, la 42ª, la 43ª e la
> 44ª hanno già nominato — il blocco delle *«sei cose che ribadisco»* — e resta **registrato e non
> preso**, perché è una scelta del proprietario su ciò che vuole rileggersi ogni volta.
>
> ✅ **Il censimento del `⏭️` è stato fatto col `grep` su `docs/` e `CLAUDE.md`, e ogni riga
> trovata è stata letta INTERA** (gotcha **#70**, seconda forma): **ventuno** righe, di cui
> **una sola** è il puntatore vivo — quella in cima alla §6, che passa al **Task 9** — **sei** sono
> rimandi che nominano la §6 **senza** nominare il passo ([`HANDOFF.md`](../HANDOFF.md) ×4,
> [`README.md`](../README.md), [`roadmap.md`](../roadmap.md)), e le altre **quattordici** sono testo di
> gotcha, verbali storici, la riga *registrata e non presa* sulla guardia che manca, un comando in
> [`riferimenti.md`](../riferimenti.md) e due righe del piano. **Nessuna casa da riallineare** — ⚠️ e
> questo si scrive come **esito di una lettura**, non di un conteggio di righe del `grep`.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **741** a **745 KB**, corretto in tutte le sue
> case — ricontate col `grep` e guardate in faccia una per una. I **due file obbligatori** passano
> da 408 a **426 KB**, e coi tre da 439 a **457**. ⚠️ **E vale ancora l'avvertenza della 44ª:**
> entrambe le aggregate sono a poche centinaia di byte dal confine dell'arrotondamento, quindi un
> `+1` può essere `int(n/1024 + 0.5)` che scavalca la soglia e non una crescita.
>
> ⚠️ **E il rapporto che la §12 difende si muove nella direzione sbagliata per l'OTTAVA misura di
> seguito:** il denominatore cresce dello **0,5 %**, il numeratore del **4,1 %**. ⛔ La causa è
> di nuovo quella della 42ª e della 44ª — un compito di **codice** che produce un gotcha nuovo e un
> racconto di misure, cioè materia che vive **qui** per costruzione — e il rapporto regge, **457
> contro 745**. ⚠️ **Ma otto passate nella stessa direzione hanno smesso di essere un accidente da
> annotare: sono una tendenza**, e chi chiuderà il Traguardo 5 la trova già a nove se non decide
> nulla.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventinovesima applicazione.

> 🔁 **Quarantaseiesima misura, il 2026-08-20, chiudendo il Task 9 del Traguardo 5 e la CONSEGNA
> alla sessione che eseguirà — e il file che si è mosso di più non è né il piano né il registro: è
> QUESTO.** In byte LF, `int(n/1024 + 0.5)`, a passata chiusa; righe contate **partendo
> dall'elenco dei file citati** e non da quelle presenti, che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ⛔ **cresciuti** | questo file `412 → 430` — **+18 KB**, ed è la consegna: la riga del Task 9, il riquadro *«ciò che il Task 9 lascia al Task 10»*, le **tre forme nuove** dentro il #31, il #76 e il #73, e due voci aperte · [`porta-di-qualita.md`](../porta-di-qualita.md) `323 → 334`, il registro che assorbe la campagna e la **sesta** ricontata del blocco C · [`HANDOFF.md`](../HANDOFF.md) `245 → 250`, il **testo integrale** delle tre forme · il **piano del Traguardo 5** `334 → 338`, che è l'errata da `E113` a `E118`, **sei** voci |
> | ⛔ **e l'invarianza di [`riferimenti.md`](../riferimenti.md) è per la QUINTA volta di seguito il dato** | **198**, immobile, mentre gli altri quattro crescevano di trentotto kilobyte fra loro. È la voce aperta della 41ª, 42ª, 44ª e 45ª con una prova in più e non con una risposta: il Task 9 ha portato una campagna di **nove mutazioni**, due mutanti vivi trovati e chiusi, e una misura nuova su `git ls-files --eol` — **tutte** nel registro e **nessuna** lì. ⛔ **Questa passata NON ha toccato il file, deliberatamente:** scegliere fra *«spostare le misure»* e *«cambiare la regola»* è del proprietario, e cominciare produrrebbe **due** convenzioni invece di una |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · disegno T5 31 · disegno T4 30 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 28 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · `design/` nove file **`5–11`** · gli altri piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR **`2–19`** · `adr/` **223** |
>
> ⛔ **E LA NOTIZIA È CHE IL FILE CRESCIUTO DI PIÙ È QUELLO CHE LA §12 ESISTE PER DIFENDERE.** Alla
> 44ª e alla 45ª il primato era del registro e poi del piano — file che **nessuno legge per
> intero** — e la loro crescita non toccava il numeratore del rapporto. Qui il primo è il
> **compendio**, cioè lettura **obbligatoria**: diciotto kilobyte che ogni sessione futura paga.
> ⚠️ **Non è un difetto da correggere di nascosto**, ed è per questo che sta scritto invece di
> essere annotato: è ciò che una **consegna** costa quando un compito produce tre forme nuove e due
> voci aperte, e la compressione è del proprietario.
>
> ⚠️ **Il MESSAGGIO: `17314 → 17652` byte, `+338 B` e `+2,0 %`** — la crescita più piccola delle
> ultime tre passate (`+697`, `+677`). ⛔ **E IL BILANCIO DELLE RIGHE VA DICHIARATO PERCHÉ È IN
> ROSSO: 260 righe contro 256, cioè QUATTRO IN PIÙ, e 2596 parole contro 2552.** Sono **entrate**
> tre cose, tutte operative e tutte per **chi coordina** e non per chi esegue: ① *una cifra
> ASSOLUTA non è un oracolo su un artefatto che cresce* — il difetto `E116`, caduto nel **brief** e
> non nel piano, cioè nel documento che la prossima sessione scriverà; ② *i rilievi si scrivono per
> SPECIE e non per elenco di righe*, che è il guadagno nuovo del #76; ③ *il confine fra dichiarare
> e pinzare*, senza il quale la riga sulle tre frasi non tenute si legge come un permesso generale
> di non provare. Sono **uscite** una correzione **chiusa** (*«il file più grande dopo la spec»*,
> che la 44ª racconta per intero), un blocco che dichiarava aperti **sei difetti** già chiusi,
> l'**elenco** dei valori che la riga della specie del lavoro aveva già detto — tolto e non esteso,
> come la 45ª fece con [`README.md`](../README.md) — e le **due cifre** del rimedio del #76, che
> vivevano anche nella §9. 📌 **Il conto non torna in righe e torna in specie:** ciò che è uscito
> era **storia chiusa** o un **conteggio in due case**, ciò che è entrato è un'**istruzione che
> cambia cosa fa chi legge**. Scritto qui perché la 25ª misura pretende il contrario, e un bilancio
> in rosso taciuto è il modo in cui questo blocco è cresciuto per sette passate.
> ⛔ **Nessuna compressione fatta, e va detto così per la QUINTA volta:** il candidato resta quello
> che la 41ª, la 42ª, la 43ª, la 44ª e la 45ª hanno già nominato — il blocco delle *«sei cose che
> ribadisco»* — e resta **registrato e non preso**, perché è una scelta del proprietario su ciò che
> vuole rileggersi ogni volta.
>
> ✅ **Il censimento del `⏭️` è stato fatto col `grep` su `docs/` e `CLAUDE.md`, e ogni riga
> trovata è stata letta INTERA** (gotcha **#70**, seconda forma): **ventiquattro** righe, di cui **una
> sola** è il puntatore vivo — quella in cima alla §6, che passa al **Task 10** — **sei** sono
> rimandi che nominano la §6 **senza** nominare il passo ([`HANDOFF.md`](../HANDOFF.md) ×4,
> [`README.md`](../README.md), [`roadmap.md`](../roadmap.md)), e le altre **diciassette** sono testo di
> gotcha, verbali storici, la riga *registrata e non presa* sulla guardia che manca, un comando in
> [`riferimenti.md`](../riferimenti.md) e due righe del piano. **Nessuna casa da riallineare** — ⚠️ e
> questo si scrive come **esito di una lettura**, non di un conteggio di righe del `grep`.
>
> ⛔ **E IL CENSIMENTO HA TROVATO UNA CIFRA FALSA DENTRO SE STESSO, che nessuno era andato a
> cercare: la 45ª misura dichiara VENTUNO righe e ne erano VENTIDUE.** La causa non è una
> distrazione, ed è il motivo per cui va scritta: **il verbale del censimento è esso stesso una
> casa del `⏭️` che censisce** — la riga *«Il censimento è stato fatto…»* contiene l'emoji, quindi
> ogni misura che lo scrive **aggiunge una riga al proprio oggetto**. La 44ª contava ventuno e
> aveva ragione **includendo la propria**; la 45ª ha **ripreso quel numero** invece di ricontare
> dopo aver chiuso il riquadro, e da quel momento era basso di uno. ⚠️ **È esattamente ciò che la
> 44ª aveva prescritto contro sé stessa** — *«questa tabella si compila contando, e non copiando
> il verbale precedente»* — applicato alla riga sbagliata: la prescrizione era per la tabella dei
> pesi, e il censimento le stava accanto. 📌 **La forma generale, ed è la stessa del riquadro qui
> sopra sull'arrotondamento:** *un verbale che misura una popolazione di cui fa parte va ricontato
> DOPO essere stato scritto, o è basso di uno per costruzione.*
> ✅ **E LE VENTIQUATTRO DI OGGI HANNO RICHIESTO DUE RICONTATE, che è la prova della regola invece
> della sua enunciazione:** la prima diede **ventitré**, e poi questo paragrafo ne ha aggiunta
> **un'altra** nominando l'emoji per spiegare il difetto. Contate dopo, comprendono questa riga.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **745** a **750 KB**, corretto in tutte le sue
> case — ricontate col `grep` e guardate in faccia una per una: due in
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), due in `CLAUDE.md`, una in cima a questo file, più i verbali
> della 44ª e della 45ª che **non** si toccano perché sono le misure di allora. I **due file
> obbligatori** passano da 426 a **445 KB**, e coi tre da 457 a **476** — tre case per il primo,
> quattro per il secondo. ⚠️ **E vale ancora l'avvertenza della 44ª e della 45ª, misurata anche
> stavolta — e stavolta l'avvertenza si è AVVERATA MENTRE LA SI SCRIVEVA, il che la rende una
> misura e non più una cautela.** Alla prima chiusura del riquadro una delle tre aggregate stava a
> **venti byte** dalla propria soglia d'arrotondamento, e la correzione *«di sole cifre»* di questo
> stesso riquadro — poche centinaia di byte — ne ha fatte **scavalcare due**. ✅ **Sono servite TRE
> ricontate prima che i numeri stessero fermi**, e quelli scritti sopra sono i valori dell'ultima.
> ⛔ **La conseguenza operativa, e vale per la prossima passata:** vicino al confine il metodo della
> sesta misura — *si rimisura dopo aver chiuso il riquadro e si corregge di sole cifre* — **non
> converge in un giro solo**, perché la correzione è essa stessa un **ingresso** della misura. Si
> rimisura **dopo ogni correzione**, finché due giri danno lo stesso numero; e un `+1` o un `−1` di
> quei giri va letto come `int(n/1024 + 0.5)` che scavalca, non come crescita o calo.
>
> ⚠️ **E il rapporto che la §12 difende si muove nella direzione sbagliata per la NONA misura di
> seguito, esattamente come la 45ª aveva previsto:** il denominatore cresce dello **0,7 %**, il
> numeratore del **4,2 %**. ⛔ **La causa è NUOVA rispetto alla 42ª, alla 44ª e alla 45ª**, e va
> distinta o la tendenza si legge come un fatto meteorologico: là il numeratore cresceva perché un
> compito di **codice** produceva un gotcha nuovo e un racconto di misure; qui cresce perché una
> **consegna** riversa nel compendio ciò che il compito ha insegnato, cioè per una ragione che si
> ripresenterà a **ogni** chiusura di compito e non solo a quelle che insegnano. ⚠️ **Il rapporto
> regge — 476 contro 750** — ma la previsione della 45ª (*«chi chiuderà il Traguardo 5 la trova già
> a nove se non decide nulla»*) **si è avverata quattro compiti prima della chiusura**.
> ⛔ **Nessuna decisione presa a riguardo**, e non è una svista: comprimere il compendio è del
> proprietario, e prenderla qui sarebbe la stessa violazione che questa §12 registra da cinque
> passate sul blocco delle *«sei cose che ribadisco»*.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> trentesima applicazione.

> 🔁 **Quarantasettesima misura, il 2026-08-21 — ed è un PRE-CONTROLLO DELLA DELEGA, non una
> passata di pesi.** Nessun file è cresciuto: [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) resta **28**, e il
> **messaggio** scende da `17652` a **`17597` byte**, `−55 B`. Il resto della tabella è quello
> della 46ª, chiusa il giorno prima.
>
> ⛔ **Ciò che l'ha aperta: il messaggio d'avvio si legge contro il repository come ogni altro
> documento (gotcha #58), e una sua frase era a QUATTRO KILOBYTE dal diventare falsa.** Diceva del
> piano *«è il SECONDO file del repository, dopo il compendio»*. Misurato oggi: compendio **434**,
> piano **338**, [`porta-di-qualita.md`](../porta-di-qualita.md) **334**. ⚠️ **E i due che si
> contendono il secondo posto crescono insieme e allo stesso ritmo** — al Task 7 il registro
> guadagnò 44 KB e il piano 47, al Task 8 il registro 35 e il piano 46 — quindi non è una frase
> che *potrebbe* invecchiare: è una frase che si rovescia al primo compito che tocca più il
> registro del piano.
> 📌 **È la stessa frase che la 44ª misura aveva già trovato FALSA IN DUE DIREZIONI**, e che era
> stata **riscritta** invece che tolta. Riscritta una seconda volta sarebbe stata la terza
> occorrenza sulla stessa riga — la forma della radice **R1**. ✅ **Tolta**, e ciò che resta è la
> metà **portante**: *«il piano da eseguire è il più grande dei quattro»*, che nomina il file da
> aprire **senza una classifica**, e regge mentre tutto cresce.
> 📌 **La regola che ne esce, ed è il #31 applicato a una CLASSIFICA invece che a un conteggio:**
> *un ordinamento fra artefatti che crescono insieme non è un fatto, è una qualificazione — e si
> toglie, perché non c'è modo di datarla.* Un conteggio stantio si vede; un *«il secondo»* stantio
> si legge come una descrizione.
>
> ✅ **Le altre affermazioni verificabili della delega sono state ricontate e reggono tutte**, e si
> scrive che sono state **eseguite** e non rilette: **37** ADR · il titolo della §9 a
> **settantasei** · **3791** righe di compendio, quindi *«oltre tremila»* · i **quattro piani più
> grandi** a `338, 168, 162, 114` · **quattro** traguardi su sei eseguiti · i due messaggi di
> commit malformati ancora presenti con gli SHA citati, `8d875f4` e `0fd3ec8` · albero **pulito**,
> ramo **allineato** al remoto, `GATE GREEN`.
>
> ⛔ **E la delega continua a NON nominare il prossimo passo.** Le due occorrenze di *«Task 9»* che
> il `grep` restituisce sono **citazioni di provenienza** di due lezioni — *dove* un difetto è
> caduto — non un puntatore: parlano di un compito **chiuso**, quindi non possono marcire. La riga
> che non deve esistere è quella che nomina il **prossimo**, e non c'è.
>
> ⛔ **E questo riquadro ha mosso il file che lo contiene, che è il caso per cui la regola
> esiste:** togliere **una** riga dalla delega ne ha scritte una trentina qui, quindi il compendio
> passa da **430** a **434**, i **due file obbligatori** da 445 a **448 KB** e **coi tre** da 476 a
> **479**. L'insieme *«HANDOFF + spec + `adr/`»* **non si muove**: resta **750**, perché questa
> passata non ha toccato nessuno dei tre. 📌 **Il rapporto peggiora quindi di nuovo — decima
> misura consecutiva — e stavolta per una passata che ha TOLTO parole dal documento che la §12
> difende**: la tendenza che la 45ª aveva registrato non si inverte nemmeno quando si comprime,
> perché il verbale della compressione vive qui. È il dato, e non una decisione: la compressione
> del compendio è del proprietario.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> trentunesima applicazione.

> 🔁 **Quarantottesima misura, il 2026-08-21, chiudendo l'AUDIT DI COERENZA su codice e
> documenti — e la notizia è che QUESTA TABELLA era ferma alla 42ª misura da CINQUE passate.** In
> byte LF, `int(n/1024 + 0.5)`, a passata chiusa; righe contate partendo dall'elenco dei file
> citati.
>
> ⛔ **CINQUE CELLE STANTIE, E NESSUNA L'HA FATTA QUESTA PASSATA.** È il difetto che la **sesta**
> misura descrisse per esteso — *«il riquadro e la tabella sono due posti, si aggiorna il primo, e
> il secondo continua a rispondere a chi deve decidere se aprire un file»* — e che la **35ª**
> ritrovò su tre celle. Qui sono cinque, e gli scarti sono più grandi:
>
> | Riga | La tabella diceva | Misurata ora | Ferma dalla |
> |---|---|---|---|
> | il **piano del Traguardo 5** | 241 | **338** | 42ª |
> | [`porta-di-qualita.md`](../porta-di-qualita.md) | 244 | **339** | 42ª |
> | [`HANDOFF.md`](../HANDOFF.md) | 236 | **254** | 42ª |
> | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) | 26 | **28** | 42ª |
> | il **messaggio** dentro di esso | `15392` | **`17597`** | 42ª |
>
> ⚠️ **Le verbali della 43ª, 44ª, 45ª, 46ª e 47ª portavano i numeri GIUSTI**, e ciascuna dichiarava
> di aver corretto *«tutte le case»* — degli **aggregati**, che è vero. 📌 **La lezione è più
> stretta di quella della sesta misura, e va scritta: l'elenco delle case di una cifra e la RIGA
> DELLA TABELLA che la ospita sono due cose diverse.** Chi riconta le case parte dal `grep` di un
> numero; la cella della tabella non contiene quel numero — contiene **il proprio**, che è vecchio
> e quindi non compare nel censimento. ⛔ **Una tabella si verifica RIMISURANDO I FILE, non
> censendo le cifre.**
>
> | | |
> |---|---|
> | **cresciuti in questa passata** | questo file `434 → 444` — il riquadro dell'audit in §6, il gotcha **#77** in §9 e questo verbale · [`porta-di-qualita.md`](../porta-di-qualita.md) `334 → 339`, i tre conteggi ricontati, il verbale della campagna `fmt` e la guardia salita al compilatore · [`HANDOFF.md`](../HANDOFF.md) `250 → 254`, il testo integrale del **#77** e la mappa dei piani riscritta |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · il piano del Traguardo 5 **338** · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 28 · [`riferimenti.md`](../riferimenti.md) **198** · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · `CLAUDE.md` 14 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · kernel-design 44 · disegno T5 31 · disegno T4 30 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · gli altri piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19`, `adr/` **223** |
> | ⛔ **e l'invarianza di [`riferimenti.md`](../riferimenti.md) è per la SESTA volta di seguito il dato** | **198**, immobile. Questa passata ha prodotto misure — la guardia `const` provata nelle due direzioni, i ventisette hunk di `fmt`, i `CR` `972 → 0 → 972` — e vivono **tutte** nel registro, accanto al controllo che difendono. ⛔ **NON toccato, deliberatamente:** scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario, ed è la voce aperta della 41ª |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **750** a **753 KB**, corretto in tutte le sue case
> — ricontate col `grep` e **guardate in faccia** una per una (#70): due in `CLAUDE.md`, una in
> testa a questo file, due in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), più i verbali della 46ª e della
> 47ª, che **non** si toccano perché sono le misure di allora. ⚠️ **E il `grep` di `479` ha
> restituito una CANDIDATA che non è una casa** — `47941dd`, uno degli SHA del Task 6 in §6 —
> letta in faccia e lasciata stare: gotcha **#70**, prima forma, alla seconda occorrenza
> registrata. I **due file obbligatori** passano da 448 a **458 KB**, e coi tre da 479 a **490**.
>
> ⚠️ **Il rapporto che la §12 difende si muove nella direzione sbagliata per l'UNDICESIMA misura
> di seguito:** il denominatore cresce dello **0,4 %**, il numeratore dell'**2,2 %**. ⛔ **La causa
> è quella della 47ª e non quella della 46ª:** non una consegna, ma una passata che TOGLIE parole
> dai documenti e le rimette **qui**, perché il verbale della correzione vive nel compendio. Il
> rapporto regge — **490 contro 753** — e la compressione del compendio resta del proprietario.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> trentaduesima applicazione.

> 🔁 **Quarantanovesima misura, il 2026-08-21, chiudendo il Task 10 del Traguardo 5 — e le celle
> stantie erano TRE, tutte e tre mosse da QUESTO compito.** In byte LF, `int(n/1024 + 0.5)`, a
> passata chiusa; righe contate **partendo dall'elenco dei file citati**.
>
> 📌 **È l'opposto della 48ª, e la differenza vale più dei numeri.** Là cinque celle erano ferme
> **dalla 42ª**, cioè per cinque passate, e nessuna delle cinque l'aveva fatta la passata che le
> trovava; qui le tre celle sono esattamente i tre file che il Task 10 ha fatto crescere. ⚠️ Una
> tabella stantia **per il lavoro di oggi** è manutenzione ordinaria; una stantia **per il lavoro
> di cinque passate fa** è il difetto della sesta misura. Vanno distinte, o la seconda si nasconde
> dentro la prima.
>
> | | |
> |---|---|
> | **cresciuti** | [`porta-di-qualita.md`](../porta-di-qualita.md) `339 → 358` — **+19 KB**, il salto più grande: il registro assorbe **quindici** righe di campagna, otto sonde nominate e i verbali di due ondate · questo file `444 → 455` · [`HANDOFF.md`](../HANDOFF.md) `254 → 259`, il testo integrale del **#78** e le due forme nuove dentro il #69 e il #76 · il **piano del Traguardo 5** `338 → 344`, l'errata da `E119` a `E129` |
> | ⛔ **e l'invarianza di [`riferimenti.md`](../riferimenti.md) è per la SETTIMA volta di seguito il dato** | **198**, immobile. Il Task 10 ha prodotto **quindici** righe di campagna di mutazione con l'esito misurato di ciascuna, il percorso a mano dei due rami di `main`, il censimento dei `⏳` e quello della parola di ADR-0033: **tutte** nel registro, accanto al controllo che difendono, e **nessuna** lì. ⛔ **NON toccato, deliberatamente** — è la voce aperta della 41ª, e scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · `adr/` **223** · kernel-design 44 · disegno T5 31 · disegno T4 30 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 28 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · `design/` nove file **`5–11`** · gli altri piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E LA NOTIZIA È CHE QUESTA CONSEGNA AGGIUNGE UN GOTCHA, dopo due che non ne avevano
> aggiunto nessuno.** Il titolo della §9 passa da **settantasette** a **settantotto**. La regola
> non è cambiata — *un gotcha che non insegna niente diluisce quelli che insegnano* — ed è per
> questo che il **#78** entra e le altre due forme no: il #78 prescrive un **gesto** che nessuna
> riga esistente prescrive (*lo stato di un compito si rivede a `HEAD`, non commit per commit*) ed
> è costato un giro pieno di revisione, mentre le altre due sono il **#69** spostato da uno script
> a un subagente e la **terza e quarta misura** sotto la tesi del **#76**. ✅ **E le due misure del
> #76 sono la parte che vale:** `+120` righe con l'istruzione generica, `−6` e `+1` con quella di
> togliere, sullo **stesso** compito e con lo **stesso** revisore. Quattro misure su quattro nella
> stessa direzione.
>
> ✅ **Il MESSAGGIO NON SI È MOSSO: `17597` byte, invariato — e non per virtù.** Le tre cifre
> sostituite dentro le recinzioni — i due aggregati e il peso del piano — hanno lo **stesso numero
> di cifre** di quelle che rimpiazzano, quindi il blocco **non poteva** muoversi. ⚠️ **Va detto
> così invece di attribuirselo:** la 29ª chiuse a `0` per lo stesso accidente, la 26ª a `+4 B` per
> una **compressione decisa**. ⛔ **Il candidato da togliere resta
> quello che la 41ª, la 42ª, la 43ª, la 44ª, la 45ª e la 46ª hanno già nominato** — il blocco delle
> *«sei cose che ribadisco»* — e resta **registrato e non preso**, perché è una scelta del
> proprietario su ciò che vuole rileggersi ogni volta.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **753** a **759 KB**, corretto in tutte le sue
> case — ricontate col `grep` e **guardate in faccia** una per una (#70): due in `CLAUDE.md`, una
> in testa a questo file, due in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), più i verbali della 47ª e della
> 48ª, che **non** si toccano perché sono le misure di allora. I **due file obbligatori** passano
> da 458 a **469 KB**, e coi tre da 490 a **500**. ⚠️ **E il peso del piano aveva una casa fuori
> dalla §12** — la riga dei quattro piani più grandi in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) — che il
> `grep` del solo aggregato non avrebbe trovato: si cerca **ogni** cifra che si muove, non solo
> quelle che si sa di avere.
>
> ⚠️ **E il rapporto che la §12 difende si muove nella direzione sbagliata per la DODICESIMA
> misura di seguito:** il denominatore cresce dello **0,8 %**, il numeratore del **2,0 %**. ⛔ La
> causa è quella della 46ª e non quella della 47ª — è una **consegna**, che riversa nel compendio
> ciò che il compito ha insegnato — e stavolta con un gotcha nuovo dentro. Il rapporto regge,
> **500 contro 759**, e la compressione del compendio resta del proprietario.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> trentatreesima applicazione. ⚠️ **E vale l'avvertenza della 46ª: vicino al confine non converge
> in un giro solo**, perché la correzione è essa stessa un ingresso della misura.

> 🔁 **Cinquantesima misura, il 2026-08-21 — ed è un PRE-CONTROLLO DELLA DELEGA**, come la 40ª,
> la 43ª e la 47ª: il messaggio d'avvio si legge **contro il repository** come ogni altro
> documento (gotcha **#58**), eseguendo invece di rileggendo. **Non reggeva in tre punti.**
>
> | | |
> |---|---|
> | ⛔ **una cifra FALSA** | il blocco di `subagent-driven-development` diceva *«E IL RIMEDIO È MISURATO **DUE VOLTE**»* del **#76**: le misure sono **quattro** — Task 8, Task 9 e **due** sul Task 10. ✅ **Tolta, non ricorretta:** il conteggio vive nella §9 e in [`HANDOFF.md`](../HANDOFF.md), e un numero che sta in tre documenti marcisce sempre in quello che non lo muove |
> | ⚠️ **un ELENCO che è un numeratore** | *«AI TASK 7 E 8 SONO SERVITE TRE REVISIONI PIENE CIASCUNO, e NOVE volte un'ondata ha aperto rilievi nuovi»*. Vero di quei due compiti, e **già superato**: il Task 10 ne ha volute **quattro**. ✅ **Sostituito da una regola che non invecchia** — *si rivede finché una passata non torna pulita* — che è la 45ª misura applicata a un elenco invece che a una cifra |
> | ⛔ **e la trovata nuova: una VOCE DI MANUTENZIONE sopravvissuta alla cosa che manuteneva** | l'elenco *«Cosa aggiornare qui, e quando»* prescriveva *«il numero dei gotcha quando §9 ne guadagna uno»*, e quel numero **non vive più lì**: la **45ª** misura lo aveva tolto dal messaggio, che ora rimanda al titolo della §9. L'istruzione ordinava di mantenere una cifra **inesistente** |
>
> 📌 **La terza è la lezione, ed è di specie nuova per questa serie.** Non è un **numero** stantio
> — quello si riconta — è un **compito** stantio, e nessun controllo lo coglie: `check-docs.sh`
> sa confrontare cifre, non sa che una voce di manutenzione ha perso il proprio oggetto.
> ⚠️ **E il modo in cui è nata va detto, perché è il #68 al rovescio:** a produrla è stata una
> passata che aveva fatto la cosa **giusta** — togliere una cifra da una casa di troppo — senza
> rileggere l'elenco che ne prescriveva il mantenimento. *Chi toglie una cifra guarda anche chi
> ordinava di aggiornarla.*
>
> ✅ **E UNA COSA È STATA AGGIUNTA, pagandola con le due tolte:** l'istruzione del gotcha **#78**
> — *lo stato di un compito si rivede a `HEAD`, mai commit per commit* — senza la quale il
> prossimo coordinatore rifà il difetto che al Task 10 è costato **un giro pieno** di revisione.
> È lo stesso scambio della 43ª: entra un'**istruzione**, escono due **cifre**.
>
> ⚠️ **E il limite inferiore dei token è stato alzato in DUE case** — `CLAUDE.md` e il messaggio —
> da *«oltre tremila»* a **«oltre quattromila»** righe di compendio: **4015**, misurate. ⛔ Un
> limite inferiore **non va falso**, va **stantio**, e questo sottoprezzava del **25 %** la
> lettura che deve far decidere. 📌 *Anche un limite inferiore si rimisura: non mente, ma smette
> di servire.*
>
> ✅ **Tutto il resto della delega è stato ESEGUITO e non riletto, e regge:** **37** ADR ·
> `crates/kernel/tests/frozen/` porta **tre** record e **una** mappa · i due commit malformati
> ci sono ancora con gli SHA citati, `8d875f4` e `0fd3ec8` · **quattro** traguardi su sei
> eseguiti · i quattro piani più grandi a `344, 168, 162, 114` e **il piano da eseguire è
> davvero il più grande dei quattro** · albero **pulito**, ramo **allineato** al remoto,
> `GATE GREEN`. ⛔ **E la delega continua a NON nominare il prossimo passo:** le tredici
> occorrenze di *«Task N»* e *«Traguardo N»* dentro le recinzioni sono state **lette una per
> una** — sono citazioni di **provenienza** di una lezione, cioè parlano di compiti **chiusi**,
> e non possono marcire.
>
> ⚠️ **Il MESSAGGIO: `17597 → 17703` byte, `+106 B` e `+0,6 %`, una riga in più** (260 → 261) e
> 2614 parole. [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) passa da **28** a **29 KB**, e questo file da
> 455 a **459** — quasi tutto è questo riquadro. I **due file obbligatori** passano da 469 a
> **473 KB**, e coi tre da 500 a **504**. L'insieme *«HANDOFF + spec + `adr/`»* resta **759**,
> perché questa passata non ha toccato nessuno dei tre.

> 🔁 **Cinquantunesima misura, il 2026-08-21, chiudendo il finding P-2 dell'audit — ed è la
> prima passata della serie che non è né un compito né una consegna né un pre-controllo della
> delega: è la **chiusura di una voce d'audit**, decisa dal proprietario **prima** di dispacciare
> il Task 11. In byte LF, `int(n/1024 + 0.5)`, a passata chiusa; le celle **rimisurate sui file**
> e non censite sulle cifre, che è la lezione della 48ª.
>
> | | |
> |---|---|
> | **cresciuti** | questo file `459 → 471` — il riquadro di P-2 in §6, il gotcha **#79** in §9 e questo verbale · [`porta-di-qualita.md`](../porta-di-qualita.md) `358 → 364`, la sezione «P-2» col censimento delle sei case e le tre misure · [`HANDOFF.md`](../HANDOFF.md) `259 → 262`, il testo integrale del **#79** · [`audit-2026-08-11.md`](../audit-2026-08-11.md) `31 → 32`, la marca su P-2 · il **piano del Traguardo 5** `344 → 345`, la voce `E130` |
> | ⛔ **e l'invarianza di [`riferimenti.md`](../riferimenti.md) è per l'OTTAVA volta di seguito il dato** | **198**, immobile. Questa passata ha prodotto **tre** misure — la sonda usa-e-getta che implementa `Worker` senza concessioni, la stessa che consegna un `Grant` vero a `start`, e il censimento a zero chiamanti di `.start(` — e vivono **tutte** nel registro, accanto al controllo che difendono. ⛔ **NON toccato, deliberatamente:** è la voce aperta della 41ª, e scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario |
> | **invariati, ricontati sui file** | spec del sotto-progetto 1 **277** · `adr/` **223** · `CLAUDE.md` 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 29 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · kernel-design 44 · disegno T5 31 · disegno T4 30 · `design/08` 11 · `design/01` 5 · `design/` nove file **`5–11`** · gli altri piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E QUESTA PASSATA HA AVUTO IL PROPRIO PRE-CONTROLLO DELLA DELEGA, che ha trovato una frase
> FALSA — la stessa specie di difetto che stava chiudendo.** La delega diceva che della §5
> dell'audit *«resta solo quel che è minore»*: contate sul rapporto, **cinque** voci restano
> **senza marca** — **T-3**, dichiarato ALTO, più S-3, A-3, K-4, PL-5 — e P-2 era la **sesta**.
> 📌 *Se le cinque siano davvero minori non l'ha misurato nessuno*, che è il **#79** un livello
> sopra: la frase giustificava di non guardarle, quindi nessuno le ha guardate. Corretta
> nominandole, senza deciderne nessuna.
>
> ⚠️ **Il MESSAGGIO: `17703 → 18032` byte, `+329 B` e `+1,9 %`, da 261 a 265 righe** — e la sua
> storia dentro questa passata è il **#78 applicato a me stesso**. Le tre cifre sostituite dentro
> le recinzioni — `504 → 517`, `759 → 762`, `344 → 345` — hanno lo **stesso numero di cifre**,
> quindi il blocco **non poteva** muoversi, e questo riquadro chiudeva dicendo *«`17703` byte,
> invariato, e non per virtù»*. Poi il pre-controllo ha corretto la frase falsa, **nel commit
> fratello dello stesso compito**, e la riga era già falsa. 📌 *Lo stato di una passata si scrive
> a `HEAD`, non al proprio primo commit* — anche quando i due commit sono entrambi tuoi.
>
> ⛔ **E IL GOTCHA #48 SI È RIPRESENTATO NELLA SUA DECIMA FORMA, DENTRO QUESTA PASSATA, mentre si
> aggiornava proprio questa tabella.** Uno script passato all'interprete con `python - <<'PY'` è
> morto su `UnicodeEncodeError` — console `cp1252`, e la riga di verbale che stampava conteneva
> un `⚠️` — dopo aver applicato **due** sostituzioni in memoria e **prima** della scrittura.
> ✅ **Verificato invece che assunto** (gotcha **#69**): il file era **byte-identico**, `476733`,
> quattro celle ancora stantie. Rifatto scrivendo lo script **in un file** e stampando **solo
> ASCII**, con un `ABORT` che non scrive se le sostituzioni applicate non sono tutte: **quattro su
> quattro**. 📌 La riga di `CLAUDE.md` sui fine-riga nomina `sed -i`; questa forma non tocca i
> fine-riga — corrompe **lo strumento** prima che giri.
> ⚠️ **RICHIAMO DEL 2026-08-25:** questa riga diceva *«nomina `sed -i` e `cargo fmt`»*, e non era
> una cifra invecchiata: **`cargo fmt` in `CLAUDE.md` non c'è mai stato.** Misurato eseguendo
> `git show <commit>:CLAUDE.md | grep -c "cargo fmt"` su **tutti** i commit che toccano
> quel file — **zero** ovunque. ⛔ **La provenienza dell'errore è la lezione:** l'affermazione fu
> **derivata da un testo** — la frase del registro *«la riga di `CLAUDE.md` nomina `sed -i`; lo
> strumento colpevole qui è `cargo fmt`»* — invece che **letta sul file**, ed è esattamente lo
> scarto che la 52ª misura ha poi contato: le cifre rimisurate sui file reggono, quelle derivate
> da un testo no.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **759** a **762 KB**, corretto in tutte le sue
> case — ricontate col `grep` e **guardate in faccia** una per una (#70): due in `CLAUDE.md`, una
> in testa a questo file, due in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), più i verbali della 49ª e della
> 50ª, che **non** si toccano perché sono le misure di allora. ⚠️ **E il `grep` ha restituito sei
> candidate che non sono case**, tutte dello stesso tipo: `17597`, `17652` e `15040` contengono
> `759` e `504` come sotto-stringhe di un **conteggio di byte del messaggio**. Gotcha **#70**,
> prima forma, alla terza occorrenza registrata. I **due file obbligatori** passano da 473 a
> **485 KB**, e coi tre da 504 a **517**.
>
> ⚠️ **E il rapporto che la §12 difende si muove nella direzione sbagliata per la TREDICESIMA
> misura di seguito:** il denominatore cresce dello **0,4 %**, il numeratore dell'**1,6 %**. ⛔ La
> causa è quella della 47ª e non quella della 46ª — una passata che **toglie** parole dai documenti
> e le rimette **qui**, perché il verbale della correzione vive nel compendio. Il rapporto regge,
> **517 contro 762**, e la compressione del compendio resta del proprietario.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> trentaquattresima applicazione.

> 🔁 **Cinquantaduesima misura, il 2026-08-24, chiudendo il Task 11 del Traguardo 5 — ed è la
> prima consegna della serie che arriva dopo un'intera fila di ondate di correzioni.**
> ⚠️ **Questo riquadro copre l'intero arco della chiusura** — dal commit del prodotto in poi, e
> quante ondate siano lo dice `git log` — e i suoi pesi sono stati **rimisurati** a ogni giro
> invece di essere lasciati al primo. ⛔ **L'ordinale è TOLTO e non riallineato:** ne ha portati
> due, *«nove»* e *«dieci»*, e ciascuno è stato reso falso dall'ondata che lo stava riscrivendo.
> In byte LF, `int(n/1024 + 0.5)`, a passata chiusa; le celle **rimisurate sui file**.
>
> | | |
> |---|---|
> | **cresciuti** | questo file `471 → 492` — il riquadro del Task 11 in §6, il gotcha **#80** e la terza forma del **#70** in §9, e questo verbale · [`HANDOFF.md`](../HANDOFF.md) `262 → 268`, il testo integrale del **#80** e della forma nuova · [`porta-di-qualita.md`](../porta-di-qualita.md) `364 → 377` · il **piano del Traguardo 5** `345 → 356`, le voci `E131`…`E141` |
> | ⛔ **e due di quei delta NON sono di questa passata** | il registro e il piano li ha mossi **l'intero arco del Task 11** — dal commit del prodotto in poi, e quanti siano lo dice `git log` — mentre la 51ª misura li aveva presi a `1049353`, cioè **prima** che il compito cominciasse. Scritto qui perché un delta attribuito alla passata sbagliata è una cifra vera in una casa falsa |
> | ⛔ **`riferimenti.md` immobile a 198 per la NONA volta di seguito** | questa passata ha prodotto misure — le due direzioni del rimedio a `check-docs.sh`, i pesi, il conteggio dei fine-riga — e vivono **tutte** nel registro, accanto al controllo che difendono. **NON toccato, deliberatamente:** è la voce aperta della 41ª, e scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario |
> | **invariati, ricontati sui file** | spec del sotto-progetto 1 **277** · `adr/` **223** · `CLAUDE.md` 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 29 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 32 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · kernel-design 44 · disegno T5 31 · disegno T4 30 · `design/08` 11 · `design/01` 5 · `design/` nove file **`5–11`** · gli altri piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⚠️ **Il MESSAGGIO non si è mosso — `18032` byte, 265 righe — e la ragione va scritta perché
> non è virtù:** le sostituzioni dentro le recinzioni hanno tutte **lo stesso numero di cifre**,
> quindi il blocco **non poteva** muoversi. ⛔ **Quante siano non è più scritto qui, ed è tolto e
> non riallineato:** il conteggio è cresciuto — *«quattro»*, poi *«cinque»* — e ciascuna volta è
> stato lasciato indietro **nello stesso commit** che lo faceva crescere.
> ✅ **Rimisurato a `HEAD` dopo l'ultima scrittura** e non al primo commit, che è il
> gotcha **#78** applicato a questo verbale — la 51ª lo aveva imparato su sé stessa.
>
> ⛔ **E il censimento delle case ha dato una CANDIDATA che non è una casa, la quarta registrata
> del gotcha #70, prima forma.** `grep -rn "485"` riportava una riga del **verbale della
> diciassettesima misura** — nominata e non numerata, perché ogni passata sposta i numeri di riga
> di questo file, e questa stessa riga ne aveva già mancato uno. ⚠️ **Ma un nome va risolto
> ESEGUENDO:** diceva *«ventesima»*, dove `485` non compare affatto — un puntatore che scade
> scambiato con uno che mente per sempre. Lì `485` compare come
> **sotto-stringa di `664851`**, un conteggio di byte. Riscriverla avrebbe rotto un verbale corretto credendo di riallineare un
> peso. 📌 *Il `grep` trova dove guardare, mai cosa cambiare* — e le case si contano **dopo**
> averle guardate una per una: **cinque** per `767`, **quattro** per `538`, **tre** per `506`.
>
> ⛔ **E l'undicesima revisione ha trovato che l'aggregato aveva DUE metodi e nessuno lo sapeva.**
> Misurato sui byte: `785422 / 1024 = 767,0` → **767**. Sommando i tre pesi **già arrotondati**
> — `268 + 277 + 223` — si ottiene **768**. ✅ **Vale il diretto**, perché è ciò che la
> **seconda misura** prescrive — *«i pesi si misurano con `wc -c`, arrotondati a KiB»* — e un
> arrotondamento non si somma.
>
> ⛔ **RICHIAMO DEL 2026-08-24, dalla dodicesima revisione: i due aggregati piccoli erano presi
> col metodo che questo riquadro dichiara scartato, e sono RISCRITTI** nelle case censite qui
> sopra e nel numeratore del rapporto qui sotto. `504` e `536` erano `14 + 490` e `14 + 490 + 32`,
> cioè **pesi già arrotondati sommati**; il diretto sui byte dava **505** e **537**. ⚠️ **E lì i
> due metodi tornavano a COINCIDERE**, perché quella passata aveva portato il compendio a **491**:
> è la coincidenza che nasconde il bivio, e la cifra scritta resta quella del **diretto**.
>
> ⛔ **RICHIAMO DEL 2026-08-24, dalla tredicesima revisione: quella coincidenza era uno STATO, e
> uno stato non è un oracolo su un file che cresce.** Qui ci va la **relazione**, che si calcola e
> non invecchia: un peso cambia cifra a `(K+0,5)·1024` byte, quindi il **diretto** scatta alla
> soglia dell'**aggregato** e la **somma degli arrotondati** a quella di ciascun **file**, senza
> che niente lo dica. 📌 **La prova è questa stessa passata:** a **74** byte di correzioni entrambi
> gli aggregati avevano cambiato cifra e i due metodi **divergevano**; a passata chiusa
> **coincidono di nuovo**, su valori diversi. La coincidenza va e viene dentro una passata sola.
> Le celle di questo riquadro e le case a mano sono **rimisurate**; il verbale qui sopra resta la
> misura di allora.
> ⚠️ **La riga di metodo resta REGISTRATA E NON PRESA, e la ragione va detta giusta** — richiamo
> del 2026-08-24, dalla tredicesima revisione: **non** *«è del proprietario»*, perché quella
> qualificazione l'ha scritta `d13cb98`. Resta aperto **dove** la regola vada scritta: qui vale
> come verdetto su **un caso**, non come riga del **metodo**.
>
> ⛔ **E il #48 si è ripresentato nella sua decima forma, con un guasto NUOVO: non ha corrotto,
> ha APPESO.** Un `python - <<'PY'` lanciato per una misura di supporto è rimasto in attesa su
> stdin fino al **timeout di due minuti**, exit **143**. Nessun file toccato, quindi nessun
> danno — ma è lo **stesso costrutto** che il 2026-08-21 aveva scritto metà delle sostituzioni e
> poi era morto su `UnicodeEncodeError`. 📌 La regola scritta allora — *gli script che mutano
> file si scrivono in un file, mai via stdin* — **è più stretta del pericolo**: quel costrutto
> non è affidabile nemmeno per **leggere**.
>
> ⚠️ **E il rapporto che la §12 difende peggiora per la QUATTORDICESIMA misura di seguito:** il
> denominatore cresce dello **0,7 %**, il numeratore del **4,1 %**. La causa è quella della 47ª
> e della 51ª — una passata che **toglie** parole dai documenti e le rimette **qui**, perché il
> verbale della correzione vive nel compendio. Il rapporto regge, **538 contro 767**, e la
> compressione del compendio resta del proprietario.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo**
> aver chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> trentacinquesima applicazione.

> 🔁 **Cinquantatreesima misura, il 2026-08-25, chiudendo il CICLO DI REVISIONE del Task 11 — e
> non il compito, che la 52ª aveva già chiuso.** ⚠️ **Questa passata non ha eseguito niente:**
> registra che cosa il ciclo ha prodotto e fa la manutenzione che la §13 pretende alla chiusura
> di una voce. In byte LF, `int(n/1024 + 0.5)`, a passata chiusa; le celle **rimisurate sui
> file**, non censite per cifra — una cella stantia contiene **il proprio** numero vecchio, che
> nessun `grep` del numero nuovo trova.
>
> | | |
> |---|---|
> | **cresciuti** | questo file `492 → 507` — il punto ⑤ di §6 riscritto col proprio richiamo, la decisione di §9 con le tre forme messe dentro righe che c'erano, l'intestazione e questo verbale · [`HANDOFF.md`](../HANDOFF.md) `268 → 272`, il testo integrale delle tre forme e la quarta occorrenza del **#31** sulla propria intestazione |
> | ⛔ **e i tre AGGREGATI si muovono per ragioni DIVERSE, quindi si rimisurano separatamente** | i due file `506 → 521` e i tre `538 → 553` crescono perché cresce **il compendio**; il grande `767 → 771` cresce perché cresce **`HANDOFF.md`**, che sta **dentro** quell'aggregato e **fuori** dagli altri due. Spostarli insieme avrebbe dato tre cifre plausibili e una sbagliata |
> | ⛔ **`riferimenti.md` NON toccato, e immobile a `202360` byte — 198** | questa passata ha prodotto misure — i pesi, i conteggi del ciclo, i fine-riga — e vivono **qui**, nel verbale. ⚠️ **Da quante passate lo sia non è scritto:** l'ordinale è **tolto e non riallineato**, per la ragione che la 52ª ha scritto sul conteggio delle ondate. La ragione del non-toccarlo resta quella della 41ª: scegliere fra *«spostare le misure»* e *«cambiare la regola»* è del proprietario, e cominciare adesso produrrebbe **due** convenzioni invece di una |
> | **invariati, ricontati sui file** | spec del sotto-progetto 1 **277** · `adr/` **223** · `CLAUDE.md` 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 29 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 32 · [`porta-di-qualita.md`](../porta-di-qualita.md) 377 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · il **piano del Traguardo 5** 356 · kernel-design 44 · disegno T5 31 · disegno T4 30 · gli altri piani 68, 50, 162, 168, 114 |
>
> ⛔ **E UNA CELLA DELLA TABELLA QUI SOPRA ERA STANTIA, trovata rimisurando e non censendo.**
> La riga di [`HANDOFF.md`](../HANDOFF.md) diceva **268 KB**: è il valore della 52ª, e questa
> passata ha fatto crescere quel file. 📌 **Nessun `grep` del numero nuovo l'avrebbe trovata.**
> Le righe che portano un peso sono **25** su **26** — l'altra rimanda a una sezione — e sono
> state ricontate **una per una contro i file**: le altre **24** reggono, compresi gli intervalli
> `2–19` degli ADR e `5–11` dei nove file di [`design/`](../design/), ricavati dal minimo e dal
> massimo misurati.
> ⚠️ **E il MESSAGGIO non si è mosso — `18032` byte, 265 righe — per la stessa ragione della
> 52ª e non per virtù:** le sostituzioni dentro la recinzione sono **due cifre e una data**,
> tutte lunghe quanto ciò che rimpiazzano, quindi il blocco non poteva muoversi. Rimisurato lo
> stesso, perché *«non poteva»* non è una misura.
>
> ⛔ **E il metodo dell'aggregato è stato riprovato invece che citato:** sui byte
> `789798 / 1024 = 771,3` → **771**; sommando i tre pesi **già arrotondati** — `272 + 277 + 223`
> — si ottiene **772**. Vale il **diretto**, come la 52ª ha deciso, e i due metodi **divergono**
> in questa passata: la relazione scritta là — il diretto scatta alla soglia dell'**aggregato**,
> la somma a quella di **ciascun file** — dà la ragione senza bisogno di uno stato.
>
> ⛔ **E LA PRIMA COSA MISURATA È UNA DIVERGENZA DALLA CONSEGNA, registrata invece che
> appianata** (gotcha **#15**). La consegna prezzava il ciclo come *«cinque giri»* con la serie
> degli Important **`7 · 7 · 4 · 2 · 2 · 1`** — **sei** termini per **cinque** revisioni.
> Contati coi comandi sui rapporti, i giri sono **cinque revisioni e quattro ondate**, e la serie
> delle cinque revisioni del ciclo è **`7 · 4 · 2 · 2 · 1`**. Il **7** in testa è quello
> dell'**undicesima** revisione, che sta **prima** del ciclo: fu chiusa dall'ondata che è poi
> rimasta senza la propria. 📌 **Le due forme sono entrambe vere su popolazioni diverse**, ed è
> il motivo per cui la §6 scrive la propria **nominando la popolazione** invece del solo numero.
>
> ⛔ **Tre candidati a gotcha, zero promossi — e la decisione sta in §9 con la ragione, non qui.**
> Le forme sono andate dentro il **#80** (il perimetro di una **revisione**), il **#70** (una
> clausola universale cercata sul **connettivo** invece che sulla proposizione) e il **#76** (il
> rimedio che regge è **togliere il numero**, e stavolta con la misura: **una** casa col rimando
> a `git log` prima del ciclo, **tre** dopo la prima ondata, e l'ultima revisione le ha rilette
> una per una dichiarandole **senza numero in pagina**).
>
> ⚠️ **E l'intestazione di [`HANDOFF.md`](../HANDOFF.md) era stantia PRIMA di questa passata, di
> quattro commit.** Diceva *«2026-08-20»*, e
> `git log --since=2026-08-21 --before=2026-08-25 -- docs/HANDOFF.md` — la popolazione **chiusa**,
> quella che il commit di oggi non tocca — ne dà **quattro**, due del 21 e due del 24 agosto. È
> il **#31** sulla riga che il **#31** ci ha già portato tre volte, e il rimedio scritto allora —
> *«sono due date»* — separa i significati senza dire **chi** riallinea il primo. ⛔ **Corretta
> qui, e col numero TOLTO da quella riga**, perché lì la popolazione è aperta e il commit che
> riallinea l'intestazione è dentro il file che il comando conta: è `I-1` della sedicesima
> revisione — la ragione per cui il ciclo si è fermato — colto **prima** di commetterlo.
>
> ⚠️ **DUE COSE VISTE E NON PRESE, con la ragione, perché tacerle sarebbe peggio.**
> ① [`roadmap.md`](../roadmap.md) porta *«Ultimo aggiornamento: 2026-08-11»* e l'ultimo commit che
> lo tocca è del **2026-08-19** — misurato con `git log -1 -- docs/roadmap.md`. Non è stato
> toccato **perché non è ciò che questo ciclo ha reso falso**, e perché la riga ha lo stesso
> bivio dell'intestazione di `HANDOFF.md`: *quando l'ho scritto* contro *a che cosa si
> riferisce*. Sceglierne uno qui inventerebbe una convenzione a metà.
> ② Il paragrafo del punto ⑤ che comincia *«E una causa del non-convergere»* è una **voce aperta
> del proprietario** e non è stata toccata; dopo la riscrittura sta accanto a una serie che
> **converge**, e la sua popolazione — la nona, la decima e l'undicesima revisione — è ancorata
> dalla frase che lo precede, che infatti è stata **conservata** per questo.
>
> ⚠️ **E il rapporto che la §12 difende peggiora ancora**, con la causa di sempre: una passata
> che **registra** una chiusura scrive nel compendio e non altrove. Misurato contro la 52ª: il
> numeratore cresce del **2,8 %**, il denominatore dello **0,5 %**. Il rapporto regge, **553
> contro 771**, e la compressione del compendio resta del proprietario. ⛔ **Da quante misure di
> seguito peggiori non è scritto:** la 52ª portava *«per la quattordicesima»*, ed è un cumulativo
> che invecchia a ogni riquadro — **tolto e non incrementato**, come l'ordinale di
> `riferimenti.md` due celle più su.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo**
> aver chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> trentaseiesima applicazione. ⚠️ **E l'ordinale è stato CONTATO, non dedotto dal riquadro
> precedente:** enumerati, gli ordinali scritti vanno dalla *seconda* alla *trentaseiesima* e
> sono **33** — mancano l'*ottava* e l'*undicesima*. ⛔ **Il `grep` diretto ne dava zero**, perché
> le righe spezzano la formula dopo *«alla»* e ogni riga porta il `>` della citazione: va
> lanciato **attraverso gli a-capo** e senza i marcatori. È il **#70** incontrato mentre lo si
> registrava.

> 🔁 **Cinquantaquattresima misura, il 2026-08-25, chiudendo il TASK 12 del Traguardo 5 — e con
> esso il suo ciclo di revisione, che è tornato pulito.** ⚠️ **Questa passata non ha eseguito il
> compito:** registra ciò che il compito ha prodotto e fa la manutenzione che la §13 pretende alla
> chiusura di una voce. ⚠️ **Il riquadro copre l'intero arco della chiusura** — la
> consegna e l'ondata che l'ha corretta — e i suoi pesi sono **rimisurati a ogni giro**, non
> lasciati al primo. In byte LF, `int(n/1024 + 0.5)`, a passata chiusa; le celle **rimisurate
> sui file**, non censite per cifra — una cella stantia contiene **il proprio** numero vecchio,
> che nessun `grep` del numero nuovo trova.
>
> | | |
> |---|---|
> | **cresciuti** | questo file `507 → 528` — la riga del Task 12 in §6, il riquadro *«ciò che il Task 12 lascia al Task 13»*, la decisione di §9 col gotcha **#81**, il richiamo sulla 51ª e questo verbale — e **dall'ondata** il confine del **#18** nella riga #81 e il richiamo in fondo · [`HANDOFF.md`](../HANDOFF.md) `272 → 275`, il testo integrale del **#81** |
> | ⛔ **e DUE celle della tabella dei pesi erano stantie, trovate RIMISURANDO e non censendo** | [`porta-di-qualita.md`](../porta-di-qualita.md) diceva **377** ed è a **398**; il **piano del Traguardo 5** diceva **356** ed è a **370**. ⚠️ **A muoverle è stato il Task 12 stesso** — `+189` righe nel registro e `+11` nell'errata, misurate con `git diff --stat 18cfed7..3f89cad` — e nessuna delle due è stata toccata da questa passata: è la cella a essere ferma, non il file. 📌 **Nessun `grep` del numero nuovo le avrebbe trovate**, ed è la ragione per cui le righe della tabella sono state ricontate **una per una contro i file**: portano una cifra in **25** su **26** — la ventiseiesima rimanda a una **sezione** — e le altre **23** reggono, compresi gli intervalli `2–19` dei trentasette ADR e `5–11` dei nove file di [`design/`](../design/), ricavati dal minimo e dal massimo misurati. ⚠️ **E una ventiseiesima cifra si muove per una ragione diversa e non va contata con le due:** quella di [`HANDOFF.md`](../HANDOFF.md), che era **giusta** all'apertura della passata ed è **questa passata** a spostarla |
> | ⛔ **e i tre AGGREGATI si muovono per ragioni diverse, quindi si rimisurano separatamente** | i due file `521 → 542` e i tre `553 → 574` crescono perché cresce **il compendio**; il grande `771 → 774` cresce perché cresce **`HANDOFF.md`**, che sta **dentro** quell'aggregato e **fuori** dagli altri due |
> | ⛔ **`riferimenti.md` NON toccato, e immobile a `202360` byte — 198** | ⚠️ **È una decisione presa esplicitamente oggi per il Task 12**, non un'omissione: sta nell'errata come voce `E146`, che l'ha misurata invece di argomentarla. ⚠️ **La misura è stata rifatta qui e non citata**, perché l'errata la prese a compito aperto: dei commit dell'esecuzione del Traguardo 5, `git rev-list --count dc6ac4c~1..HEAD -- docs/riferimenti.md` dà **zero**, contro **44** su [`porta-di-qualita.md`](../porta-di-qualita.md) — `E146` scriveva **0 su 63** e **41**, che erano i valori di allora. La ragione resta quella della 41ª: scegliere fra *«spostare le misure»* e *«cambiare la regola»* è del proprietario, e cominciare adesso produrrebbe **due** convenzioni invece di una. ⚠️ **Da quante passate lo sia non è scritto**, per la ragione che la 52ª ha dato sul conteggio delle ondate |
> | **invariati, ricontati sui file** | spec del sotto-progetto 1 **277** · `adr/` **223** · `CLAUDE.md` 14 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 29 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 32 · [`roadmap.md`](../roadmap.md) 31 · [`README.md`](../README.md) 19 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 6 · kernel-design 44 · disegno T5 31 · disegno T4 30 · gli altri piani 68, 50, 162, 168, 114 · [`../spikes/RISULTATI.md`](../../spikes/RISULTATI.md) 23 · [`../spikes/GUI-REQUISITI.md`](../../spikes/GUI-REQUISITI.md) 6 · `design/08` 11 · `design/01` 5 |
>
> ⛔ **E LA PRIMA COSA MISURATA È UNA DIVERGENZA DALLA CONSEGNA, registrata invece che appianata**
> (gotcha **#15**). La consegna prezzava il ciclo del Task 12 con la serie degli Important
> **`5 · 4 · 2 · 1 · 0`** — **cinque** termini. Contati uno per uno sui rapporti, le revisioni sono
> **quattro** e la serie è **`5 · 2 · 1 · 0`** — la riga di conteggio dei quattro rapporti, letta
> una per una, dà `0 Critical` e `5 / 2 / 1 / 0 Important` — e le ondate committate fra la prima
> e l'ultima revisione sono **tre**, più la riformattazione `rustfmt` di `bd103c4`. 📌 **È la stessa specie della divergenza che la 53ª ha registrato**, e per la seconda
> volta di fila il numero sbagliato è quello **ricordato** e non quello **contato**.
>
> ⛔ **E il metodo dell'aggregato è stato riprovato invece che citato:** il diretto sui byte dà
> **774**, la somma dei tre pesi **già arrotondati** — `275 + 277 + 223` — dà **775**. Vale il
> **diretto**, come la 52ª ha deciso, e la relazione scritta là dice perché: il diretto scatta
> alla soglia dell'**aggregato**, la somma a quella di **ciascun file**, quindi quale dei due sia
> più grande dipende da dove cadono i resti — e non c'è uno stato da ricordare.
>
> ⛔ **Due candidati a gotcha, UNO promosso, e la ragione di ciascuno sta nella §9 e non qui.** Il
> **#81** entra; la **terza forma della trappola dei fine-riga** no, e le tre ragioni stanno nella
> §9 insieme a ciò che di quel candidato resta **registrato e non preso**: la regola di
> `CLAUDE.md` quantifica su *«chi scrive uno strumento che tocca file»*, e `git stash` non scrive
> niente di suo.
>
> ⚠️ **E il MESSAGGIO di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) non si è mosso — `18032` byte, 265
> righe** — rimisurato e non dedotto: le **tre** sostituzioni dentro la recinzione sono tutte
> cifre di peso, lunghe quanto ciò che rimpiazzano. ⚠️ **Erano due fino all'ondata**, che
> ne ha aggiunta la terza.
>
> ⚠️ **DUE COSE VISTE E NON PRESE, con la ragione, perché tacerle sarebbe peggio.**
> ① [`roadmap.md`](../roadmap.md) e [`README.md`](../README.md) **non sono stati toccati**: tengono lo
> stato **per traguardo**, e il Traguardo 5 è ancora *«in esecuzione»*, che è vero. ⛔ Entrambi
> **delegano già** il numeratore alla §6 con parole proprie — *«a che punto sia NON è scritto
> qui»* — quindi la chiusura di un compito non li rende falsi: è il rimando che regge, e un
> rimando non può marcire. ⚠️ Resta in piedi il *«Ultimo aggiornamento: 2026-08-11»* di
> `roadmap.md` che la 53ª aveva già visto e non preso, con la ragione che vi è scritta.
> ② [`semi-dst.md`](../semi-dst.md) dice *«il Traguardo 4 ha costruito **due** campagne»*, ed è vero
> **di quel traguardo**; il Task 12 ne ha aggiunta una terza. Se l'elenco debba guadagnare una
> riga per la campagna dell'arbitro è lavoro del **Task 13**, che è l'audit, e il compito non l'ha
> toccato per la sua stessa regola: quel file si tocca *«se e solo se la campagna trova un
> difetto»*, e non ne ha trovati.
>
> ⚠️ **E il rapporto che la §12 difende peggiora ancora**, con la causa di sempre: una passata che
> **registra** una chiusura scrive nel compendio e non altrove. Il rapporto regge, **574 contro
> 774**, e la compressione del compendio resta del proprietario.
>
> ⛔ **E L'ONDATA DEL 2026-08-25 HA TROVATO CHE IL CENSIMENTO DELLE CIFRE MOSSE SI ERA FERMATO
> PRIMA DI DUE RIGHE DI [`AVVIO-CHAT.md`](../AVVIO-CHAT.md).** Portavano **tre** proposizioni che
> questa stessa passata aveva reso false — `553` alla riga *«LEGGI QUESTI TRE FILE»*, **dentro
> la recinzione**, e `553 KB contro 771` nel rapporto. ⛔ **Riscritte e non tolte:** a cambiare
> è il **fatto**, le cifre sono lunghe uguale e il blocco non si muove, e togliere un peso da un
> documento d'ingresso è voce del proprietario. Rifatto cercando il numero **vecchio** su tutti
> i file tracciati — `507`, `272`, `521`, `553`, `771`, `377`, `356` — e le altre case che il
> `grep` restituisce sono **verbali datati**, che non si toccano.
>
> ⛔ **E DUE CONTEGGI DI COMMIT ERANO MISURATI PRIMA DEL PROPRIO COMMIT:** *«tutti gli 84
> commit»* nel richiamo alla 51ª e *«dei 67 commit dell'esecuzione»* nella cella qui sopra —
> misurati a `7832407` sono **85** e **68**, e l'ultimo di ciascuno è il commit che porta la
> frase. ⛔ **Tolti e non riallineati**, per la ragione della 52ª: un totale è falso al commit
> successivo, il **comando** no. ✅ **La sostanza regge, ed è stata rieseguita:** il ciclo ha
> esaminato **85** commit e ha restituito `0` a ognuno; i due sotto-conteggi della cella qui
> sopra riproducono.
>
> ⚠️ **E la riga #81 di §9 aveva perso un confine nella compressione:**
> [`HANDOFF.md`](../HANDOFF.md) la chiude contro il **#18** — *«il gemello con la manopola
> girata»* — e qui la clausola non c'era, mentre il #18 sta nella **stessa tabella**,
> sessantatré righe più su, a dire *«un solo giro non è una misura»* dove il #81 dice
> *«ventimila semi non lo sono lo stesso»*. **Riportata**, perché il compendio è la casa che
> `CLAUDE.md` prescrive di leggere.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura.

> 🔁 **Cinquantacinquesima misura, il 2026-08-25, chiudendo il TASK 13 e con esso il TRAGUARDO
> 5.** ⚠️ **Il compito era un AUDIT e non una scrittura**, quindi la parte che conta di questo
> verbale è **ciò che è stato rieseguito e non ha prodotto uno scarto**. In byte LF,
> `int(n/1024 + 0.5)`, a **passata chiusa**; le celle **rimisurate sui file** e non censite per
> cifra — una cella stantia contiene **il proprio** numero vecchio, che nessun `grep` del numero
> nuovo trova.
>
> | | |
> |---|---|
> | ⛔ **i riconteggi del Passo 1, rieseguiti col comando** | `bash scripts/gate.sh` → `GATE GREEN`, exit 0 · `cargo test --workspace --no-fail-fast --locked` → **37 bersagli, 264 passate, 0 fallite, 2 ignorate**, aggregato sulle righe `test result:` e bersagli contati con `grep -c "^     Running \|^   Doc-tests "` · `ls crates/kernel/tests/compile_fail/*.rs` → **33**, e altrettanti `.stderr` · `git status --short` vuoto. ⛔ **Zero scarti** con ciò che la §6 e [`porta-di-qualita.md`](../porta-di-qualita.md) già scrivevano: non c'era niente da riallineare, ed è la notizia |
> | ⚠️ **la trappola di misura del Passo 1, dichiarata invece di ereditata** | `grep -c "fn " crates/kernel/tests/arbiter_admission.rs` dà **23**, e le sonde sono **20**: `grep -c "^#\[test\]"` sullo stesso file dà venti, e le tre in più sono gli aiutanti. Chi scrive quel numero lo conti sull'attributo, non sulla parola `fn` |
> | ⛔ **le dodici righe di catalogo, ricontate SULLA SPEC** | `3 + 8 + 1`, delimitando per **intestazione** (`**B ·`, `**C ·`, `#### 7.4.2`) e non per posizione. Il conteggio **non diverge** dal disegno. Stato: **undici chiuse**, **una dichiarata PARZIALE** — la campagna DST, voce `E152`. Il catalogo intero resta a **quaranta** righe: `3 + 5 + 19 + 13` |
> | **cresciuti** | questo file `528 → 548` — la chiusura in §6, il riquadro del Task 13, la riga della tabella dei compiti, la voce 13 dell'ordine, la cella del traguardo e questo verbale · [`porta-di-qualita.md`](../porta-di-qualita.md) `398 → 412`, che è **la sola scrittura vera del compito**: la tabella delle voci aperte · [`roadmap.md`](../roadmap.md) `31 → 33` · [`README.md`](../README.md) `19 → 21` · [`semi-dst.md`](../semi-dst.md) `6 → 8` · il **piano del Traguardo 5** `370 → 373`, le tre voci d'errata · il **disegno del Traguardo 5** `31 → 32` |
> | ⚠️ **e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `29 → 30`, che la prima stesura di questo riquadro dava per invariato** | ⛔ **Il messaggio d'avvio dichiara la SPECIE del lavoro che viene — lo dice il file stesso, in testa — e questa chiusura la CAMBIA:** *«il piano è SCRITTO, si ESEGUE»* diventa *«si riparte dal BRAINSTORMING»*. Riscritte **tre** proposizioni dentro la recinzione più lo scambio fra i due blocchi di skill, e il messaggio passa da **18032** a **18483** byte LF, da 265 a **271** righe, da 17,6 a **18,0** KB. ⚠️ **La cella diceva *«mossi in byte ma non di un kilobyte»***, scritta **prima** di quello scambio: è il gotcha **#78** dentro il verbale, una descrizione dello stato di un altro artefatto resa falsa da una modifica **fratella** della stessa passata |
> | **invariati, ricontati sui file** | [`HANDOFF.md`](../HANDOFF.md) **275**, che si è mosso in byte ma non di un kilobyte — la sua correzione **toglie** un elenco e mette un rimando · `CLAUDE.md` 14 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 32 · spec del sotto-progetto 1 **277** · `adr/` **223** · kernel-design 44 · disegno T4 30 · gli altri piani 68, 50, 162, 168, 114 |
> | ⛔ **`riferimenti.md` NON toccato, e immobile a `202360` byte — 198** | ⚠️ **È una decisione presa esplicitamente, non un'omissione:** sta nell'errata come `E153`, che è `E146` alla seconda occorrenza. ✅ **Rimisurata qui e non citata:** `git rev-list --count dc6ac4c~1..HEAD -- docs/riferimenti.md` dà **zero** sui commit dell'esecuzione del traguardo. La ragione resta quella della 41ª: scegliere fra *«spostare le misure»* e *«cambiare la regola»* è del proprietario |
>
> ⛔ **E LA COSA PIÙ UTILE CHE QUESTA PASSATA HA MISURATO NON È UN PESO: è che un CENSIMENTO
> scritto in un documento diventa una CASA di sé stesso.** Il `grep` su *«VOCE APERTA»* dava
> **undici** occorrenze prima della raccolta e ne dà **tredici** dopo — l'intestazione della
> sezione nuova e la riga di comando che la censisce. È la stessa nota che questa §12 porta già
> **tre volte** sul censimento del `⏭️`, incontrata qui su un secondo oggetto. ✅ Scritta
> **dentro** il blocco dei comandi, dove la legge chi lo rilancia.
>
> ⛔ **E DUE CONTEGGI DI QUESTO STESSO VERBALE ERANO SBAGLIATI ALLA PRIMA STESURA.** Il `grep`
> sulle sonde senza riga di catalogo: scritto *«sette righe candidate»*, ed erano **dieci** — il
> numero sette veniva da un filtro **diverso**, provato prima e poi cambiato senza rimisurare;
> quello è stato corretto **rieseguendo**, e la revisione l'ha riprodotto.
>
> ⛔ **RICHIAMO DEL 2026-08-25, E IL SECONDO CONTEGGIO È LA LEZIONE VERA: il filtro sull'errata
> era scritto *«ventisei»*, questo verbale lo dichiarava *«corretto rieseguendo»*, e il valore
> che ci fu scritto NON VIENE DA UNA RIESECUZIONE.** Rilanciato sul piano com'era alla raccolta,
> il comando restituisce un altro numero, e **due** delle voci che l'elenco scritto portava non
> sono fra quelle che restituisce. ⛔ **La cifra è TOLTA da qui e dalla riga del Task 13 in §6, e
> vive nel solo blocco di comandi di [`porta-di-qualita.md`](../porta-di-qualita.md)** — dove chi la
> contesta rilancia il comando che la produce, e dove è stata corretta e datata insieme ai due
> valori che ne discendevano. 📌 **La classe è quella che il ciclo del Task 11 aveva già
> isolato** — le cifre **rimisurate sui file** reggono, quelle **derivate da un testo o da una
> corsa precedente** no — ⛔ **e la forma nuova è peggiore di entrambe: *«corretto rieseguendo»*
> è a sua volta un'affermazione derivata da un testo.** Dichiarare di aver rieseguito è gratis e
> nessuno lo controlla; scrivere il **comando accanto alla cifra** costa una riga e chiunque può
> smentirlo. È la forma che questa passata ha comprato al prezzo di un traguardo chiuso male.
>
> ⛔ **E il filtro sull'errata MANCA voci aperte vere, misurate a zero contro sé stesso** —
> quante e quali sta **accanto al comando**, in [`porta-di-qualita.md`](../porta-di-qualita.md),
> ricontate il 2026-08-25: sono **una in più** di quante questo verbale ne elencava, perché
> `E140` era stata messa fra quelle che il filtro restituisce e il filtro non la restituisce.
> Quindi il censimento non è il `grep`: è il `grep` **più** la lettura della §6. È il gotcha
> **#70** nella forma che costa, *il `grep` non trova una casa che c'è*.
>
> ⚠️ **Il censimento del `⏭️` è stato fatto col `grep` su `docs/` e `CLAUDE.md`, e ogni riga è
> stata letta INTERA. Una** sola è il puntatore vivo — quella in cima alla §6, che passa al
> **Traguardo 6** — **sei** sono rimandi che nominano la §6 senza nominare il passo, e il
> **resto** è testo di gotcha, verbali storici, prescrizioni e comandi. `CLAUDE.md` ne ha
> **zero**.
>
> ⛔ **E IL TOTALE È STATO TOLTO, non riallineato, ed è la cosa che questa passata ha imparato
> sul proprio censimento.** La prima stesura scriveva *«24 righe in tutto»* e *«diciassette»*:
> erano i valori misurati **prima** di scrivere il riquadro del Task 13, la riga del §9 e questo
> verbale, che portano **cinque** occorrenze nuove — il totale a passata chiusa è **29**, e sarà
> un altro al prossimo che scrive qui. ⛔ *Un totale che il proprio commit sposta non può essere
> verde in nessun mondo:* restano le **due** classi che non si muovono, il puntatore e i sei
> rimandi. ⚠️ E questa riga è a sua volta una casa del `⏭️` che censisce, come le tre
> gemelle qui sopra.
>
> ⛔ **E il censimento `git ls-files --eol` è stato rieseguito, ed è la voce `E155`:**
> `140 i/lf w/crlf · 85 i/lf w/lf · 4 i/crlf w/crlf · 3 i/-text w/-text`. La seconda cifra era
> **75** il 2026-08-20 e si è mossa di **dieci** — i file LF/LF che il traguardo ha aggiunto —
> mentre le altre tre reggono. ⛔ **Tolta dalla voce aperta della §6 e non riallineata:** viveva
> in **due** documenti, e nell'altro — [`porta-di-qualita.md`](../porta-di-qualita.md) — sta dentro
> un **verbale datato**, dove è giusta e resta. 📌 *La stessa cifra, due collocazioni, due
> destini: un verbale dice cosa fu misurato quel giorno, una voce aperta si legge come «questo è
> lo stato».*
>
> ⚠️ **E i fine-riga sono stati contati PRIMA e DOPO su ogni file toccato**, perché due dei nove
> sono **CRLF integrali**: [`porta-di-qualita.md`](../porta-di-qualita.md) — `CR` **3399 → 3502**,
> sempre pari al numero di righe — e ⚠️ [`semi-dst.md`](../semi-dst.md), **98 → 127**, che nessun
> documento aveva mai dichiarato tale. Gli altri sette toccati sono LF puro e lo sono rimasti.
> ✅ Le due scritture sono state fatte con uno **script in un file**, in modalità binaria, che
> ricompone i `\\r\\n` riga per riga; `git diff --stat` sul registro nomina le sole righe aggiunte.
>
> ⛔ **E il metodo dell'aggregato è stato riprovato invece che citato**, come la 52ª ha deciso:
> vale il **diretto sui byte**, non la somma dei pesi già arrotondati.
>
> ⚠️ **DUE COSE VISTE E NON PRESE, con la ragione, perché tacerle sarebbe peggio.**
> ① La cella dei sotto-progetti di [`roadmap.md`](../roadmap.md) è la **terza** casa dello stato per
> traguardo **dentro lo stesso file** — le altre due sono la tabella dei piani e quella dei
> traguardi. È stata **allungata** e non tolta, perché toglierla cambia la struttura del
> documento: registrata nella cella stessa, del proprietario. ② La §4 **non è stata toccata**:
> la condizione del Passo 4 non scatta, misurato con
> `git diff --stat 26a529c..HEAD -- '*Cargo.toml' Cargo.lock`, che è **vuoto**.
>
> ⚠️ **E il rapporto che la §12 difende peggiora ancora**, con la causa di sempre: una passata che
> **registra** una chiusura scrive nel compendio e non altrove. ⛔ Ma stavolta cresce di più
> [`porta-di-qualita.md`](../porta-di-qualita.md), che **non** è lettura obbligatoria — ed è
> deliberato: il censimento delle voci aperte sta dove chi lavora lo trova, non dove ogni sessione
> lo paga.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura.

> 🔁 **Cinquantaseiesima misura, il 2026-08-25, chiudendo l'ONDATA DI CORREZIONI sul Task 13 — e
> le due specie che l'hanno prodotta sono una sola cosa detta da due lati.** Due revisioni
> indipendenti sul commit che dichiarava chiuso il Traguardo 5, `1 + 1` Critical, `5 + 3`
> Important, `3 + 4` Minor; chiuse in **una** passata perché tre rilievi erano **la stessa specie
> in file diversi**, e chiuderli uno per volta li riapre.
>
> | | |
> |---|---|
> | ⛔ **la specie ①: una cifra o un'affermazione vive in più case di quante ne debba avere** | il **numeratore** del Traguardo 5 — la §6 dichiara che vive *«QUI e in nessun altro punto di questa sezione»* e il commit lo ha portato a **tre** case dentro la §6: è il **#68 ricreato dentro il riquadro che lo vieta**. ⛔ **Censito e non dedotto:** `grep -rn "tredici compiti su tredici\|tredici su tredici"` su `docs/` e `CLAUDE.md` dà **sette** case in tutto, e la revisione ne aveva viste tre. **Tolte le due della §6**; le altre quattro sono **registrate e non prese**, vedi in fondo |
> | ⛔ **la specie ②: un'affermazione su ciò che un comando restituisce, che dal comando non riproduce** | il blocco dei comandi della tabella delle voci aperte scriveva *«ne dà VENTICINQUE»* con `E52` ed `E140` nell'elenco. **Rilanciato come è scritto** sul piano a `ac65504` ne dà **23**, e nessuno dei sette termini del filtro tocca quelle due (`grep -icE` per termine → **0** su entrambe); i due derivati non sono *«13 aperte, 12 chiuse»* ma **12 e 11**, e le voci che il filtro **manca** non sono quattro ma **cinque**, perché `E140` era contata dalla parte sbagliata. ⛔ **E la §12 lo elencava fra i valori *«corretti RIESEGUENDO»***: quello è il difetto, non il numero |
> | ⛔ **le due specie sono la stessa cosa** | una cifra che ha più case ne ha almeno una che nessuno rilancia, e una cifra che nessuno rilancia è un'affermazione derivata da un testo. 📌 **La cura è una sola: la cifra vive dove vive il COMANDO che la produce**, e le altre case portano il rimando. Applicata a `venticinque` (tre case → **una**), a `32 target e 177 test` (tre case → **zero**, resta il comando), a `512 righe` del `simulator` (**quattro** case → zero) |
> | ⚠️ **e la cifra del `simulator` è la più istruttiva delle tre** | `CLAUDE.md` diceva **512** righe scritte a mano; `find crates/simulator/src -name '*.rs' \| xargs wc -l` dà **519**, e a spostarla è stato `b4f1f03` il 2026-08-21. ⛔ **Nessuno l'aveva mai rimisurata perché SOSTENEVA UNA TESI GIUSTA** — la contraddizione fra ADR-0026 e ADR-0031, gotcha #59 — e una cifra che sostiene una tesi giusta non attira dubbi. ⛔ **E IL CENSIMENTO DI QUESTA PASSATA ERA CORTO ALLA PRIMA STESURA, cioè il #70 dentro l'ondata che lo stava chiudendo:** fatto con `grep -rn '512' CLAUDE.md docs/COMPENDIO.md docs/HANDOFF.md`, cioè su **tre file scelti a mano**, mancava tutte le case che stanno fuori da quei tre; rifatto con `git ls-files -z \| xargs -0 grep -n '512'` sui **tracciati**, le ha trovate. ⛔ **Quante siano non è scritto qui — questo stesso riquadro ne aggiunge**; ciò che è scritto sono le case. ✅ **Tolta dai documenti VIVI** — `CLAUDE.md`, la riga `A-1` della §6, la riga **#59** della §9 e la sua gemella in [`HANDOFF.md`](../HANDOFF.md); ⚖️ **lasciata nei VERBALI DATATI** — il rimando del 2026-08-18 in [ADR-0026](../adr/0026-linguaggio-del-core.md), il finding `A-1` di [`audit-2026-08-11.md`](../audit-2026-08-11.md) e due righe di [`riferimenti.md`](../riferimenti.md), che questa passata non tocca. 📌 **Un censimento fatto su un elenco di file scelto a mano non è un censimento**, e a trovarlo è stata la rilettura finale, non la passata che lo scriveva |
> | ⛔ **una CLAUSOLA UNIVERSALE non si rende vera scrivendola meglio** | il *Punto di ripresa* di [`HANDOFF.md`](../HANDOFF.md) dichiarava *«Quanti traguardi siano eseguiti, quali e con che data, NON è scritto qui»*, e lo stesso file lo scriveva più volte, di cui **una nell'intestazione scritta dallo stesso commit**. ✅ **Tolta la clausola, tenuto il rimando** — che era la parte utile — **e tolto lì dove viveva** ciò che era davvero stantio: l'elenco dei traguardi eseguiti dentro *«In trenta secondi»*, fermo al **quarto** il giorno in cui si è chiuso il quinto. 📌 È la radice **R1**: una correzione attraversa il punto in cui nasce e non gli altri |
> | ⛔ **la stessa R1 in [`roadmap.md`](../roadmap.md), e la casa era la QUARTA dentro lo stesso file** | la chiusura aveva aggiornato la tabella dei sotto-progetti, quella dei piani e quella dei traguardi, e **non** la sezione *«Stato in una riga»*, che è la seconda cosa che si legge e si fermava al **disegno** del Traguardo 5. ✅ **L'enumerazione è tolta e delegata alla tabella dei traguardi dello stesso file**, e la sezione tiene ciò che ciascun traguardo **ha portato** |
> | ⛔ **un NUMERO DI RIGA dentro un documento è un puntatore senza guardia** | [`README.md`](../README.md) citava *«la riga **16** di questo stesso file»* e lo **stesso hunk** aveva inserito sei righe sopra, spostando il bersaglio. ✅ **Sostituito con un'àncora testuale**, e con lui i **sette** rimandi per numero di riga della colonna *«Dove è dichiarata»* della tabella nuova, che puntavano dentro un file di oltre tremilacinquecento righe che cresce a ogni compito |
> | ⛔ **una TABELLA che si affianca non è una tabella sola** | `git diff --stat` sul registro dava **`+103`, `−0`**: i sei riquadri erano **intatti**, nessuno nominava l'indice, e la tabella ne **ripeteva la sostanza**. La lettera della condizione **8** era soddisfatta, lo scopo no. ✅ **Ora ciascuno dei sei nomina la tabella**, e le due direzioni si tengono |
> | ⛔ **una cura si cita dal testo che la contiene, non a memoria** | la §9 giustificava un candidato non promosso con *«la cura è quella che il **#70** prescrive — si legge ogni riga intera e **si guarda chi l'ha scritta**»*. Misurato: `grep -c "si guarda chi"` sulla riga del #70 dà **zero**, e `grep -n "chi l'ha scritta"` su tutto il file restituisce **la sola riga che la attribuisce**. Nessuna delle quattro forme del #70 descrive un censimento che, scrivendosi, muove la propria popolazione — e *leggere ogni riga intera* non impedisce a un totale di spostarsi. ✅ **Riattribuita al #31**, la cui cura — *togliere il totale* — è quella che il commit aveva davvero applicato |
> | ⛔ **UNA VOCE DI MANUTENZIONE SOPRAVVIVE ALLA COSA CHE MANUTENEVA, e ora sono QUATTRO — tre in un elenco, una in un altro** | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) prescriveva di mantenere *«il numero dei traguardi eseguiti»* — che **questo stesso commit** aveva tolto dal messaggio — e *«il peso del messaggio»*, la cui cella era stata svuotata il 2026-08-18; [`roadmap.md`](../roadmap.md) prescriveva di aggiornare `CLAUDE.md` *«se cambia il prossimo passo»*, che `CLAUDE.md` non porta più (`grep -c '⏭️'` → **zero**). ✅ **Tolte tutte e tre**, e ⛔ **tutte e quattro le occorrenze vanno nella stessa direzione:** la cosa manutenuta sparisce, la voce che la manuteneva resta, **perché chi TOGLIE un valore non rilegge l'elenco che prescriveva di mantenerlo** — e togliere è la cura che questo repository applica più spesso. 📌 **La domanda, da fare a ogni chiusura:** *ciò che questa voce ordina di aggiornare esiste ancora?* |
> | ⚠️ **il conteggio dei precedenti, in due case di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md)** | *«dieci su dieci al Traguardo 4 e dodici su dodici al 3»* — nessuna delle due falsa, entrambe omettono il quinto, e una era stata **riemessa riga per riga** dallo stesso commit. ✅ Sostituite con la formulazione che `CLAUDE.md` aveva già scelto: *«tutti quelli eseguiti finora, senza un'eccezione»*, col numeratore nella §6 |
> | ⚠️ **e una disambiguazione di una riga in [`HANDOFF.md`](../HANDOFF.md)** | *«la spec del sotto-progetto 1 è completa e senza voci aperte»* significa **nessuna sezione lasciata da approvare**, non *«nulla in sospeso sulla spec»*: fra le voci aperte del Traguardo 5 ce ne sono che toccano la **§7.4**, che **è** spec. Le due letture divergono, e la riga diceva solo la prima |
>
> ⛔ **RIESEGUITO, non citato:** `bash scripts/check-docs.sh` → `OK — no inconsistencies.` ·
> `bash scripts/gate.sh` → `GATE GREEN`, exit 0 ·
> `cargo test --workspace --no-fail-fast --locked`, aggregato sulle righe `test result:` →
> **37 bersagli, 264 passate, 0 fallite, 2 ignorate** · `find crates/simulator/src -name '*.rs'
> | xargs wc -l` → **519** · il filtro sull'errata a `ac65504` → **23**, a `HEAD` → **24**,
> perché il commit rivisto aggiunge `E153` e il filtro lo cattura.
>
> ⚠️ **E il censimento del `⏭️` è stato rifatto, non riletto:** `grep -rn "⏭️" docs/ CLAUDE.md`,
> ogni riga guardata in faccia. ✅ **Le due classi che non si muovono reggono identiche** — **una**
> casa è il puntatore vivo, in cima alla §6; **sei** sono rimandi che nominano la §6 senza nominare
> il passo; `CLAUDE.md` ne ha **zero**. ⛔ **E il totale si è mosso di nuovo**, perché le righe
> scritte oggi che **citano il comando** — questa fra esse — sono a loro volta case di sé stesse: è
> la prova che la 55ª ha fatto bene a **toglierlo**, e la ragione per cui non lo si riscrive
> nemmeno adesso.
>
> ⚠️ **I fine-riga contati PRIMA e DOPO su ogni file toccato**, e uno dei sette è **CRLF
> integrale**: [`porta-di-qualita.md`](../porta-di-qualita.md), `CR` **3502 → 3546**, sempre pari al
> numero di righe, con **zero** LF nudi e **zero** CR nudi verificati sui byte. Gli altri sei
> sono LF puro e lo sono rimasti. La scrittura CRLF è stata fatta con uno **script in un file**,
> in modalità binaria, che separa e ricompone su `\\r\\n`.
>
> ⛔ **REGISTRATE E NON PRESE, con la ragione — tacerle sarebbe peggio.**
> ① **Le quattro case del numeratore fuori dalla §6** — [`README.md`](../README.md), le due tabelle
> di [`roadmap.md`](../roadmap.md) e l'intestazione del disegno del Traguardo 5: la cifra è
> **terminale**, cioè il traguardo è chiuso e il numeratore non può più muoversi, `CLAUDE.md`
> tiene per iscritto lo stato **per traguardo** nelle tabelle di `roadmap.md` e `README.md`, e i
> Traguardi 1–4 portano la stessa forma nelle stesse celle: toglierla al solo quinto renderebbe
> le tabelle incoerenti, che è una **modifica di struttura** e quindi del proprietario.
> ② ***«Resta il Traguardo 6»* vive in tre documenti d'ingresso.** Non è un puntatore né una
> cifra: è la **fase**, ed è la risposta alla domanda che ciascun documento d'ingresso deve dare
> da solo — *posso considerare chiuso il sotto-progetto?* Toglierla da due su tre farebbe
> concludere il contrario a chi apre quel due su tre. Oggi è vera in tutte e tre.
> ③ **Il soggetto del commit rivisto generalizza *«l'audit non ha trovato uno scarto nei
> conteggi»* oltre il Passo 1**, mentre lo stesso commit apre `E155` per un censimento mosso di
> dieci ed `E154` per una riga falsa dal 2026-08-18. È **storia**: riscriverlo vorrebbe dire
> riscrivere un commit, e questo verbale è il posto in cui la divergenza si legge.
> ④ **`E30` — da decidere PRIMA del Traguardo 6 — e le altre voci del proprietario** non sono
> state toccate; [`riferimenti.md`](../riferimenti.md) non è stato toccato, ed è `E146`/`E153`.
>
> ⚠️ **I pesi, rimisurati SUI FILE a passata chiusa** — byte LF, `int(n/1024 + 0.5)`, e mai
> sommando kilobyte già arrotondati.
>
> | | |
> |---|---|
> | **cresciuti** | questo file `548 → 565` — la §6 riscritta in tre punti, la riga della §9
> riattribuita, la 55ª misura corretta e questo verbale · [`porta-di-qualita.md`](../porta-di-qualita.md)
> `412 → 416`, il blocco dei comandi rilanciato più i sei rimandi e l'esclusione ·
> [`HANDOFF.md`](../HANDOFF.md) `275 → 277` · [`roadmap.md`](../roadmap.md) `33 → 34` ·
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `30 → 31` · `CLAUDE.md` `14 → 15` |
> | ⚠️ **e il MESSAGGIO dentro la recinzione**, che è la cella che nessuno rifà | `18483 →
> 18835` byte LF, da 271 a **276** righe, da 18,0 a **18,4** KB. Cresce per le due proposizioni
> riscritte dentro il blocco delle skill: il conteggio dei precedenti e quello del piano |
> | **invariato in kilobyte** | [`README.md`](../README.md) **21**, mosso in byte e non di un
> kilobyte. Non toccati: [`riferimenti.md`](../riferimenti.md) **198**, la spec del sotto-progetto 1
> **277**, `adr/` **223**, [`semi-dst.md`](../semi-dst.md) **8**, i piani e i disegni |
> | ⛔ **i tre AGGREGATI dei documenti d'ingresso, rimisurati e non dedotti** | `CLAUDE.md` più
> questo file `562 → 580` · con l'audit `594 → 612` · [`HANDOFF.md`](../HANDOFF.md) più la spec
> SP-1 più `adr/` `775 → 777`, che cresce perché cresce `HANDOFF.md` — sta **dentro** quel
> terzo aggregato e **fuori** dagli altri due, quindi i tre si rimisurano separatamente |
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo**
> aver chiuso questo riquadro, e **due giri hanno dato lo stesso numero** — metodo della sesta
> misura, con la convergenza della 55ª.

> 🔁 **Cinquantasettesima misura, il 2026-08-27, chiudendo la RADICE R2 del secondo audit — i
> cinque finding sul contratto della porta `journal`.** In byte LF, `int(n/1024 + 0.5)`, a
> passata chiusa; le celle **rimisurate sui file** e non censite per cifra.
>
> ⛔ **E LA PRIMA COSA DA SCRIVERE È CHE LA TABELLA ERA GIÀ STANTIA PRIMA DI QUESTA PASSATA, e
> non di una svista: la sessione dell'audit non ha scritto la propria misura.** Rimisurato al
> commit da cui questa passata parte — `git show HEAD:<f> | wc -c`, che dà i byte LF senza
> passare dall'albero di lavoro — `HANDOFF.md` era **278** dove la cella diceva 277, `README.md`
> **22** dove diceva 21, e i **tre aggregati** erano fuori di **1 KB** ciascuno. 📌 È la
> distinzione che la **49ª** misura prescrive: *una tabella stantia per il lavoro di oggi è
> manutenzione ordinaria; una stantia per il lavoro di una passata precedente è il difetto della
> sesta misura*. Qui c'erano tutt'e due, e vanno separate o la seconda si nasconde dentro la
> prima.
>
> | | |
> |---|---|
> | ⛔ **LA RIGA CHE MANCAVA, ed è la notizia** | [`audit-2026-08-27.md`](../audit-2026-08-27.md), **236 KB**, non aveva **nessuna** voce in questa tabella — mentre la §6 dice *«si legge PRIMA di riprendere»* e `CLAUDE.md` manda a leggerne la delega. È il **terzo** file più grande del repository dopo questo e [`porta-di-qualita.md`](../porta-di-qualita.md), e chi doveva decidere se aprirlo non aveva né la voce né il peso. 📌 Trovata **partendo dall'elenco dei file citati** e non dalle righe presenti, che è il movimento della **15ª** misura: *rileggendo, una riga assente non si vede* |
> | **mossi da questa passata** | la spec del sotto-progetto 1 `277 → 284` — i richiami di §4.1, §4.4 e §4.5 · [`porta-di-qualita.md`](../porta-di-qualita.md) `416 → 422`, la **voce aperta 3** e le tre ri-conte · [`HANDOFF.md`](../HANDOFF.md) `278 → 281`, la seconda forma del **#64** per esteso · questo file `566 → 575`, i richiami di §6, la forma nella riga #64 e questo verbale · `docs/adr/` `223 → 224`, il rimando in ADR-0018 |
> | **già stantie a `HEAD`, non mosse da qui** | `HANDOFF.md` **277 → 278** e [`README.md`](../README.md) **21 → 22**, entrambe ferme alla 56ª misura del 2026-08-25 |
> | **invariati, ricontati sui file** | [`riferimenti.md`](../riferimenti.md) **198** · [`roadmap.md`](../roadmap.md) 34 · `CLAUDE.md` 15 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 31 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 32 · [`tracciabilita.md`](../tracciabilita.md) 15 · [`semi-dst.md`](../semi-dst.md) 8 · kernel-design 44 · i disegni 32 e 30 · i piani 373, 168, 162, 114, 68, 50 · gli ADR `2–19` (2441 B e 19291 B) · [`design/`](../design/) nove file `5–11` — ricontati tutti e nove, e il 07 che questa passata tocca resta **6** |
> | ⛔ **`riferimenti.md` NON toccato** | ⚠️ **È una decisione, non un'omissione**, ed è la voce aperta della **41ª**: questa passata ha prodotto misure — la divergenza delle due nozioni di dubbio su entrambe le implementazioni, l'`error[E0046]`, i pesi — e vivono **tutte** in [`porta-di-qualita.md`](../porta-di-qualita.md), accanto al controllo che difendono. Scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario |
>
> ⛔ **E I TRE AGGREGATI SI MUOVONO PER RAGIONI DIVERSE, quindi si rimisurano separatamente** —
> col **diretto sui byte** e mai sommando kilobyte già arrotondati, che è la decisione della
> **52ª**. *«HANDOFF + spec + `adr/`»* passa da **778** a **789**, e cresce di più perché due dei
> suoi tre membri sono i file che questa passata ha scritto; *«`CLAUDE.md` + questo file»* da
> **581** a **590** e *«coi tre»* da **613** a **622**, che crescono solo per il compendio — e
> quasi tutto è **questo riquadro**.
>
> ⛔ **Le case sono state ricontate col `grep` sui file TRACCIATI e guardate in faccia una per
> una** (#70): l'aggregato dei tre file ne ha **quattro** — `CLAUDE.md` ×2, questo file, e
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) — i due piccoli **cinque** fra `CLAUDE.md` e `AVVIO-CHAT.md`.
> ⚠️ **RIALLINEATE E NON TOLTE, e la scelta va dichiarata perché la regola di `CLAUDE.md` direbbe
> il contrario:** *«una cifra che vive in PIÙ documenti si TOGLIE»*, e toglierle è la cura vera —
> ma tocca `CLAUDE.md` e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), che sono i **documenti d'ingresso**, ed
> è la decisione che la **53ª** misura ha già registrato come **del proprietario e non presa**.
> Riallinearle lascia il repository senza frasi false; toglierle chiuderebbe la classe. 📌 **La
> prova in più che questa passata porta è che gli aggregati erano già fuori di 1 KB a `HEAD`
> senza che nessuno se ne fosse accorto**, cioè la classe morde anche quando nessuno scrive.
>
> ⚠️ **I fine-riga contati PRIMA e DOPO su ogni file toccato**, perché sei degli undici sono
> **CRLF integrali**: in ciascuno di essi i `CR` restano pari al numero di righe, e i cinque LF
> puri restano a zero `CR`. `git ls-files --eol` è **immutato**: `226 i/lf`, `4 i/crlf`,
> `3 i/-text`. Le scritture sono state fatte con uno **script in un file**, mai via stdin, che
> legge e riscrive con `newline=""` e **aborta senza scrivere** se una sostituzione non cade
> esattamente una volta (gotcha **#48**, decima forma, e **#69**).
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre**, finché due giri danno lo stesso numero —
> metodo della sesta misura con la convergenza della 46ª.

> 🔁 **Cinquantottesima misura, il 2026-08-27, chiudendo le RADICI R3 e R5 del secondo audit — le
> diciassette voci sui documenti di stato e sui modelli.** In byte LF, `int(n/1024 + 0.5)`, a
> passata chiusa; le celle **rimisurate sui file** e non censite per cifra.
>
> ⛔ **E la prima cosa da scrivere è la stessa della 57ª, il che la rende una SERIE e non un
> incidente: la tabella era già stantia prima di questa passata, e per il lavoro di una passata
> precedente.** Le chiusure di R4, R6 e R7 hanno mosso [`audit-2026-08-27.md`](../audit-2026-08-27.md)
> — la cella diceva **236** — e nessuna delle tre ha scritto la propria misura. 📌 È la distinzione
> della **49ª**, ed è la seconda volta di fila: *una tabella stantia per il lavoro di oggi è
> manutenzione ordinaria; una stantia per il lavoro di una passata precedente è il difetto della
> sesta misura*. ⚠️ **Due passate di fila con lo stesso difetto sono una regola che non si applica
> da sola**, e vale la pena dirlo invece di riallineare in silenzio.
>
> | | |
> |---|---|
> | **mossi da questa passata** | questo file `582 → 587` con `CLAUDE.md`, per i richiami di §5, §6 e §11 e per questo verbale · [`porta-di-qualita.md`](../porta-di-qualita.md) `422 → 427`, la riga **32** della tabella unica e il perimetro della voce `R5` · la spec del sotto-progetto 1 `284 → 298`, **sette** richiami datati · [`HANDOFF.md`](../HANDOFF.md) `281 → 284` · [`roadmap.md`](../roadmap.md) `34 → 35` · [`tracciabilita.md`](../tracciabilita.md) `15 → 17` · `docs/adr/` `224 → 230` per il rimando di ADR-0005 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) `31 → 34` |
> | ⛔ **la riga che ha rotto un INTERVALLO** | `design/02` è passata a **14 KB**, e la cella di [`design/`](../design/) diceva *«5–11 KB l'uno»*: un intervallo invecchia come una cifra, e **nessuno lo riconta perché sembra una descrizione**. Ora è `5–14`. Stessa specie per `docs/adr/`, `2–19 → 2–21`: ADR-0036 è cresciuto |
> | **già stantie a `HEAD`, non mosse da qui** | [`audit-2026-08-27.md`](../audit-2026-08-27.md) **236 → 254**, ferma alla 57ª · la riga di `design/01`, che diceva **5** ed era **7** |
> | **invariati, ricontati sui file** | [`riferimenti.md`](../riferimenti.md) 198 · [`README.md`](../README.md) 22 · `CLAUDE.md` 15 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 32 · [`semi-dst.md`](../semi-dst.md) 8 · kernel-design 44 · i disegni 32 e 30 · i piani 373, 168, 162, 114, 68, 50 · `design/08` 11 · gli spike 23 e 6 |
> | ⛔ **`riferimenti.md` NON toccato** | ⚠️ **È una decisione, non un'omissione**, ed è la voce aperta della **41ª**, riaffermata dalla 57ª: le misure di questa passata — il censimento dei nomi italiani, i dieci vincoli della §11 uno per uno, i fine-riga prima e dopo — vivono **accanto al controllo che difendono**. Scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario |
>
> ⛔ **E LA TESTA DELL'AUDIT ERA SBAGLIATA IN DUE CASE CON DUE VALORI, ed è la notizia di questo
> verbale.** `CLAUDE.md` la dava **41 KB** e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) **42**, mentre
> l'aggregato *«coi tre»* della 57ª — `622 − 590` — la implicava a **32**: tre case, tre valori,
> nessuno dei quali è quello vero. ✅ **Misurata: 51402 byte LF, cioè 50 KB**, e il «Dettaglio» che
> segue **204** e non 205. ⛔ **E il conteggio di RIGHE è stato TOLTO da tutte e tre**, non
> riallineato: *«le prime ~340 righe»* è un puntatore a un bersaglio che si muove a ogni rimedio —
> oggi la testa finisce alla **390** — mentre *«tutto ciò che sta prima della sezione Dettaglio»* è
> un delimitatore **semantico** e non marcisce. 📌 È la cura del **#31** applicata a un numero di
> riga invece che a un peso, e la stessa che la 57ª applicò ai numeri di riga della tabella unica.
>
> ⛔ **I tre aggregati, rimisurati col diretto sui byte** e mai sommando kilobyte già arrotondati
> (decisione della **52ª**): *«HANDOFF + spec + `adr/`»* passa da **789** a **811** — cresce di più
> perché due dei tre membri sono i file che questa passata ha scritto — *«`CLAUDE.md` + questo
> file»* da **590** a **603**, e *«coi tre»* da **631** a **653**, che porta con sé la correzione
> della testa.
>
> ⚠️ **Riallineate e non tolte, e la scelta è la stessa della 57ª e per la stessa ragione:**
> `CLAUDE.md` prescrive che *«una cifra che vive in PIÙ documenti si TOGLIE»*, e toglierle è la cura
> vera — ma tocca i **documenti d'ingresso**, ed è la decisione che la **53ª** ha registrato come
> **del proprietario e non presa**. 📌 **La prova in più che questa passata porta:** delle
> **quattro** case dell'aggregato dei tre file, una era stantia di 22 KB e **due** portavano una
> cifra derivata (la testa) che non corrispondeva a nessuna misura. Riallinearle lascia il
> repository senza frasi false; toglierle chiuderebbe la classe.
>
> ⚠️ **I fine-riga contati PRIMA e DOPO su ogni file toccato.** I due CRLF integrali —
> [`porta-di-qualita.md`](../porta-di-qualita.md) e la spec — restano con `CR == LF` e **zero LF nudi**;
> il rapporto d'audit, anch'esso CRLF integrale, passa da 1798 a 1820 righe con la stessa proprietà;
> i file LF puri restano a **CR = 0**. `git ls-files --eol` è **immutato**: `226 i/lf`, `4 i/crlf`,
> `3 i/-text`. ⛔ **E tutte le scritture sono passate da uno script IN UN FILE, mai via stdin** —
> gotcha **#48**, decima forma — che **costruisce e codifica tutto prima**, scrive in un temporaneo e
> fa `os.replace`, e **aborta senza scrivere** se un'ancora non cade esattamente il numero di volte
> atteso (gotcha **#82**).
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura con la convergenza
> della 46ª.

> 🔁 **Cinquantanovesima misura, il 2026-08-27, chiudendo AUD-009, AUD-024 e AUD-062 della
> radice R1 — le tre voci sulla cifra dei siti `cargo` del cancello.** In byte LF,
> `int(n/1024 + 0.5)`, a passata chiusa; le celle **rimisurate sui file** e non censite per cifra.
>
> ✅ **E la prima cosa da scrivere è che la serie di due si è ROTTA, dalla parte buona.** La 57ª e
> la 58ª aprivano entrambe dichiarando la tabella già stantia **per il lavoro di una passata
> precedente**; ricontate una per una contro i file al commit da cui questa parte, **tutte** le celle
> reggevano. Le sole che si muovono sono i file che questo rimedio ha scritto. 📌 È la distinzione
> della **49ª**, e stavolta cade dalla parte della manutenzione ordinaria.
>
> | | |
> |---|---|
> | **mossi da questa passata** | `CLAUDE.md` `15 → 16` · questo file `587 → 594` · [`riferimenti.md`](../riferimenti.md) `198 → 199`, la riga col **comando** che prende il posto della cifra tolta · [`porta-di-qualita.md`](../porta-di-qualita.md) `427 → 428`, la cella **N6** e la tabella dei sette passi · [`audit-2026-08-27.md`](../audit-2026-08-27.md) `254 → 256`, le tre righe di stato e il verbale del rimedio |
> | ⚠️ **mossi in byte e non di un kilobyte** | [`HANDOFF.md`](../HANDOFF.md) **284** e [`roadmap.md`](../roadmap.md) **35**, entrambi scritti da questo rimedio. Rimisurati lo stesso: *«non poteva muoversi»* non è una misura |
> | ⛔ **la TESTA dell'audit cresce, e va detto perché è lettura obbligatoria** | `50 → 53 KB`: le tre righe di stato e il verbale stanno **prima** della sezione «Dettaglio». ⚠️ **Ogni rimedio la farà crescere**, perché lo stato dei finding vive lì ed è la loro casa unica. È il prezzo della casa unica, e si paga a ogni chiusura |
> | ⛔ **e una cella era FALSA CONTRO SÉ STESSA, trovata rimisurando e non censendo** | quella di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) diceva *«il **messaggio** ne è **18,4** (20568 byte LF)»*, e `20568 / 1024` fa **20,1**: due numeri nella stessa cella che si smentiscono, cioè **la forma esatta di AUD-009**, dentro il verbale che lo chiude. ⛔ **Il `18,4` viene dalla 56ª**, dove era vero di `18835` byte, ed è sopravvissuto alla 58ª — che aggiornò i **byte** e non il **derivato**. ✅ **Tolto il derivato, non riallineato:** un KB ricavato da un byte non è una seconda misura, è la stessa divisa per 1024, e una divisione non merita una casa |
> | ⚠️ **e la rimisura dei byte DIVERGE dalla 58ª: registrata invece che appianata** (gotcha **#15**) | il blocco fra le due recinzioni — **303** righe, che questa passata **non ha toccato nel merito** — pesa **20606** byte LF, non 20568. Provati quattro confini (recinzioni incluse, escluse, senza la prima riga, senza l'ultima): **nessuno** dà 20568. Quindi non è un file cresciuto: è un **metodo mai scritto**. ✅ Ora il metodo sta **dentro la cella**, accanto alla cifra — *le righe fra le due recinzioni, escluse* — ed è la stessa cura della **31ª**, *una misura ha bisogno di dire come è stata presa* |
> | **invariati, ricontati sui file** | la spec del sotto-progetto 1 **298** · `docs/adr/` **230**, con gli estremi `2–21` ricontati sui trentasette · [`README.md`](../README.md) 22 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 34 · [`audit-2026-08-11.md`](../audit-2026-08-11.md) 32 · [`tracciabilita.md`](../tracciabilita.md) 17 · [`semi-dst.md`](../semi-dst.md) 8 · kernel-design 44 · i disegni 32 e 30 · i piani 373, 168, 162, 114, 68, 50 · `design/08` 11 · `design/01` 7 · [`design/`](../design/) nove file `5–14` · gli spike 23 e 6 |
>
> ⛔ **I tre aggregati, col diretto sui byte e mai sommando kilobyte già arrotondati** (decisione
> della **52ª**): *«HANDOFF + spec + `adr/`»* passa da **811** a **812**; *«`CLAUDE.md` + questo
> file»* da **603** a **610** e *«coi tre»* da **653** a **663**, e quasi tutto è **questo riquadro**.
> ⛔ **E LA PRIMA STESURA DI QUESTA RIGA DICEVA *«resta 603 mentre entrambi i membri sono
> cresciuti»*, ED ERA VERA DEL SOLO COMMIT DEL RIMEDIO.** Misurato prima di scrivere il verbale,
> `617836 / 1024` dava **603,4**: la soglia dell'arrotondamento nascondeva **due** crescite vere,
> ed è l'avvertenza della **44ª** dal verso opposto — là un `+1` poteva essere una soglia
> scavalcata invece di una crescita, qui uno `0` nascondeva il contrario. Poi il verbale è entrato
> nel file che descrive e l'aggregato si è mosso davvero. 📌 È la **nona** misura ancora una volta
> — *un verbale si scrive quando la passata è chiusa* — e ciò che vale è che si vede **solo
> rimisurando dopo aver scritto**, che è esattamente ciò che la riga in fondo a ogni riquadro
> prescrive: la prima stesura non era pigra, era **inevitabile**.
>
> ⚠️ **Riallineate e non tolte, e la scelta è quella della 57ª e della 58ª:** toglierle è la cura
> vera, e tocca i **documenti d'ingresso** — la decisione che la **53ª** ha registrato come **del
> proprietario e non presa**.
>
> ⚠️ **I fine-riga contati PRIMA e DOPO su ogni file toccato.** I due CRLF integrali —
> [`porta-di-qualita.md`](../porta-di-qualita.md) e il rapporto d'audit — restano con `CR == LF` e
> **zero** LF nudi; i file LF puri restano a `CR = 0`. `git ls-files --eol` è **immutato**:
> `226 i/lf`, `4 i/crlf`, `3 i/-text`. ⛔ Tutte le scritture sono passate da uno script **in un
> file**, mai via stdin, che **costruisce e codifica tutto prima**, scrive in un temporaneo e fa
> `os.replace`, e **aborta senza scrivere nulla** se un'ancora non cade esattamente una volta
> (gotcha **#82** e **#48**, decima forma).
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre**, finché due giri danno lo stesso numero —
> metodo della sesta misura con la convergenza della 46ª.

> 🔁 **Sessantesima misura, il 2026-08-27, chiudendo la CONSEGNA alla sessione che continuerà la
> radice R1 — ed è una passata di sola delega, come la 40ª, la 43ª, la 47ª e la 50ª.** In byte LF,
> `int(n/1024 + 0.5)`, a passata chiusa; le celle **rimisurate sui file**.
>
> ✅ **Il pre-controllo della delega è stato ESEGUITO e non riletto, e il messaggio REGGE:** **37**
> ADR · il titolo della §9 a **ottantadue**, cioè nessun gotcha nuovo da questa sessione · **5150**
> righe di compendio, quindi *«oltre quattromila»* · i quattro piani più grandi a `373, 168, 162,
> 114` · `crates/kernel/tests/frozen/` con **tre** record e **una** mappa · i due commit malformati
> ancora presenti con gli SHA citati · albero **pulito**, ramo **allineato**, `GATE GREEN`.
> ⛔ **E la specie del lavoro NON è cambiata** — resta **RIMEDIO di un audit aperto** — quindi il
> blocco delle skill non si tocca: è la riga che il file dichiara di riscrivere *«per prima quando
> l'audit si chiude»*, e l'audit non è chiuso.
>
> | | |
> |---|---|
> | **mossi da questa passata** | [`audit-2026-08-27.md`](../audit-2026-08-27.md) `256 → 257`, il passo **1** della disciplina e la riga **5º** · [`HANDOFF.md`](../HANDOFF.md) `284 → 285`, l'ancora dell'intestazione · questo file `594 → 598` |
> | ⛔ **la TESTA cresce ancora: `53 → 54 KB`** | e stavolta **non** per lo stato dei finding ma per la **ricetta**: le due righe che questa passata scrive stanno nella sezione *«Come si concludono quelli aperti»*. ⚠️ **È il secondo modo in cui quella testa cresce**, e vale saperlo: uno si paga a ogni **chiusura**, l'altro a ogni **lezione** |
> | ⚠️ **e l'ancora dell'intestazione di [`HANDOFF.md`](../HANDOFF.md) era stantia mentre la DATA era giusta** | diceva *«aggiornato il 2026-08-27, **all'apertura** dell'audit»*, e i rimedi di oggi l'hanno toccato **dopo** l'apertura. 📌 È la metà che il rimedio del 2026-08-11 non copriva: quello separò *«quando l'ho scritto»* da *«a che cosa si riferisce»* e riallineava solo il primo. Stessa specie di **AUD-034**, chiuso poche ore prima sul compendio |
> | **invariati, ricontati sui file** | `CLAUDE.md` 16 · [`riferimenti.md`](../riferimenti.md) 199 · [`porta-di-qualita.md`](../porta-di-qualita.md) 428 · [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) 34 e il **messaggio** a **20606** byte su **303** righe · la spec **298** · `adr/` **230** · tutto il resto come alla 59ª |
>
> ⛔ **Gli aggregati, col diretto sui byte:** *«HANDOFF + spec + `adr/`»* resta **812** — `HANDOFF.md`
> è cresciuto e la soglia non è stata scavalcata — *«`CLAUDE.md` + questo file»* passa da **610** a
> **614**, e *«coi tre»* da **663** a **667**.
>
> ⚠️ **E questa passata ha commesso la decima forma del #48 su sé stessa**, che si scrive perché la
> regola c'era: un `python - <<'PY'` lanciato per una correzione di sole cifre è rimasto in attesa su
> **stdin** fino al timeout di due minuti, **exit 143** — identico al guasto che la **52ª** registra,
> e su un costrutto che la stessa riga vieta. ✅ **Nessuna scrittura era partita**, e il rifacimento
> con lo script **in un file** è passato al primo colpo. 📌 *La regola scritta non protegge chi la
> ha scritta: protegge chi la rilegge.*
>
> ⛔ **Nessun gotcha nuovo, ed è una decisione.** Il candidato era la causa strutturale di AUD-034 —
> *nessuno dei ganci della §13 copre una riscrittura di merito dentro una riga che c'è già* — e non
> entra: è la stessa forma che la **50ª** e la **56ª** registrano come *una voce di manutenzione
> sopravvive alla cosa che manuteneva*, e un gotcha che non insegna niente diluisce quelli che
> insegnano. Sta **registrata e non presa** in testa a questo file, dove il difetto viveva.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre**, finché due giri danno lo stesso numero.

> 🔁 **Sessantunesima misura, il 2026-08-28, chiudendo AUD-013 della radice R1 — il commento di
> `a_grant_inside_its_grace_keeps_its_reservation` che affermava che né `ask_back` né
> `revoking()` spazzano, mentre `ask_back` spazza per primo.** In byte LF,
> `int(n/1024 + 0.5)`, a passata chiusa; le celle **rimisurate sui file** e non censite per cifra.
>
> ⛔ **E LA PRIMA COSA DA SCRIVERE È CHE LA TABELLA ERA GIÀ STANTIA PRIMA DI QUESTA PASSATA, e la
> 60ª non l'ha scritta contro sé stessa — non nel VALORE, nella CASA.** `git show --stat d902c40`
> — il commit da cui questa passata parte — elenca i file che quel commit tocca, e `CLAUDE.md` e
> [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) ci sono **entrambi**; la tabella della 60ª li mette fra gli
> **invariati**. Rimisurati sul contenuto — `CLAUDE.md` **15907** byte prima di quel commit e
> **15907** dopo, il messaggio di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) **20606** prima e **20606**
> dopo — i **valori** reggono: `610→614`, `663→667` e `53→54 KB` sono sostituzioni di cifre
> **della stessa lunghezza**, e un blocco che scambia tre cifre con tre cifre non si sposta di un
> byte. 📌 È **esattamente** il caso che la **49ª** misura già chiamava *«non per virtù»*, per lo
> stesso messaggio: quella volta la riga fu scritta, qui no — e un file toccato che finisce fra
> gli invariati senza quella riga si legge come un file che nessuno ha aperto.
>
> | | |
> |---|---|
> | **mossi da questa passata** | [`HANDOFF.md`](../HANDOFF.md) `285 → 287`, la data e la nuova forma del **#48** per esteso · [`porta-di-qualita.md`](../porta-di-qualita.md) `428 → 430`, i due richiami datati sulla tabella delle sonde e su «Che cosa se ne fa» · questo file `598 → 609` — `603` la §6 già scritta quando questa passata comincia, `609` con questo verbale |
> | ⛔ **e `CLAUDE.md` e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) producono lo stesso fenomeno, stavolta dichiarato da chi lo scrive** | TOCCATI e IMMOTI, cifre della stessa lunghezza: `614→624` e `667→678` in `CLAUDE.md`, **15907** byte prima di questa passata e **15907** dopo; `205→204` in [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), **34 KB**, col **messaggio** a **20606** byte su **303** righe prima e dopo. La cura di due paragrafi sopra si applica alla passata che la trova |
> | ⛔ **e quel `205 → 204` è PERIMETRO AGGIUNTO, non chiesto** | il «Dettaglio» del rapporto misura **208504** byte LF, cioè **204**, ed era già così prima di questa passata: la **58ª** l'aveva scritto e le due case d'ingresso non erano state girate. Assorbito perché vive tre righe sotto la riga che questa passata riallinea — precedenti **AUD-061 dentro AUD-021** e **AUD-034 dentro AUD-009** |
> | **la TESTA del rapporto d'audit NON è stata toccata da questa passata** | resta **257 KB** il file intero, **54 KB** la testa: la cella *Stato* di AUD-013 la gira il coordinatore in un secondo commit |
> | ⛔ **`riferimenti.md` NON toccato** | ⚠️ **È una decisione, non un'omissione**, la voce aperta della **41ª**: questa passata non ha prodotto una misura nuova da portarci accanto. Scegliere fra *«spostare le misure»* e *«cambiare la regola»* resta del proprietario |
> | **invariati, ricontati sui file** | [`riferimenti.md`](../riferimenti.md) 199 · la spec **298** · `adr/` **230** · tutto il resto come alla 60ª |
>
> ⛔ **Le case di `614` e `667` sono state ricontate col `grep` su `docs/` più `CLAUDE.md`, a
> `HEAD`, e guardate in faccia una per una** (#70): **ventidue** occorrenze in tutto, **due
> vive e venti candidate**.
> Le due vive sono **entrambe in `CLAUDE.md`**. Delle candidate, **sei** sono dentro questo stesso
> file — i verbali già datati della 12ª, 13ª, 19ª, 20ª e 60ª misura, che non si toccano
> perché sono le misure di allora — e le rimanenti **quattordici**, in questo file, in [`HANDOFF.md`](../HANDOFF.md), in
> [`porta-di-qualita.md`](../porta-di-qualita.md), in [`riferimenti.md`](../riferimenti.md), nel piano
> del Traguardo 5 e nella spec, sono tutte un `6144` di `Mib`, un conteggio di parole o l'ID
> arXiv `2602.02614` ripetuto — mai il numero cercato. [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) non ne
> porta nessuna: la sua casa vicina è `812`, fuori dallo scopo di questa cifra.
>
> ⛔ **Gli aggregati, col diretto sui byte e mai sommando kilobyte già arrotondati** (decisione
> della **52ª**): *«HANDOFF + spec + `adr/`»* passa da **812** a **814 KB** — cresce solo per
> `HANDOFF.md`, spec e `adr/` sono ferme — *«`CLAUDE.md` + questo file»* da **614** a **624**, e
> *«coi tre»* da **667** a **678**, che crescono solo per il compendio.
>
> ⚠️ **Riallineate e non tolte, e la scelta è quella della 57ª, della 58ª e della 59ª:** la regola
> di `CLAUDE.md` direbbe di **togliere** una cifra che vive in più case, ma toglierle tocca i
> **documenti d'ingresso** — la decisione che la **53ª** misura ha registrato come **del
> proprietario e non presa**. ⚠️ **E `812 KB` in `CLAUDE.md` e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md)
> resta non toccato, per una ragione diversa dalle prime due:** quella riga già porta il proprio
> rimando all'ultima misura in fondo alla §12 invece di pretendersi esatta al giorno — è la cura
> che la riga stessa mostra da quando un ordinale scritto lì invecchiò.
>
> ⚠️ **I fine-riga contati PRIMA e DOPO su ogni file toccato.**
> [`porta-di-qualita.md`](../porta-di-qualita.md) resta **CRLF integrale**, `CR == LF`; `CLAUDE.md`,
> questo file, [`HANDOFF.md`](../HANDOFF.md) e [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) restano **LF
> puri**, `CR = 0`. `git ls-files --eol` è **immutato**. ⛔ Tutte le scritture sono passate da uno script **in un file**, mai via stdin,
> che **costruisce e codifica tutto prima**, scrive in un temporaneo e fa `os.replace`, e
> **aborta senza scrivere** se un'ancora non cade esattamente una volta (gotcha **#82** e **#48**,
> decima forma).
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre**, finché due giri danno lo stesso numero.

---

> 🔁 **La colonna dei kilobyte esce dalla §12 — il 2026-08-28, completando il Compito 1 dello
> sfoltimento, che l'aveva lasciata in piedi.** Il disegno prescriveva alla §2.2 *«resta la
> tabella dei puntatori, **senza la colonna dei kilobyte**, col comando che la rifà»*. Il commit
> `d864286` ha archiviato i sessanta riquadri delle misure, ha scritto sotto la tabella che
> *«i pesi non stanno più in questa tabella»* — e la colonna era ancora lì, con **27** pesi.
>
> ⛔ **Il difetto non era la colonna: era che la sezione DICHIARAVA di non averla.** Una
> documentazione lasciata a descrivere uno stato che non esiste mente con autorevolezza, e qui
> mentiva **tre righe sotto** la tabella che la smentiva. Il commit si legge come la causa: il
> suo messaggio dice *«resta la tabella dei puntatori, escono i sessanta riquadri»*, cioè la
> prima metà della riga del disegno senza la seconda.
>
> ⚠️ **E i pesi erano già stantii dopo UN GIORNO, misurato prima di toglierli.** Il commit
> `3fb252d` del 2026-08-27 si intitola *«i pesi della §12 tornano veri»*; rimisurati il
> 2026-08-28 con `git ls-files | while read -r f; do echo "$(wc -c < "$f") $f"; done`:
> [`../audit-2026-08-27.md`](../audit-2026-08-27.md) **257 → 269 KB**,
> [`../porta-di-qualita.md`](../porta-di-qualita.md) **428 → 435**,
> [`../HANDOFF.md`](../HANDOFF.md) **285 → 292**, la spec del sotto-progetto 1 **298 → 302**.
> È l'argomento della riga che li sostituisce, misurato invece che asserito.
>
> 📌 **Cosa NON si è perso, ed è la parte che una passata frettolosa avrebbe cancellato.** La
> sola cella della colonna che non portava un peso — *«la sezione, non il file»*, sul consuntivo
> di [`../HANDOFF.md`](../HANDOFF.md) — è stata **spostata nella cella centrale**, non tolta. E
> il peso del **messaggio** di [`../AVVIO-CHAT.md`](../AVVIO-CHAT.md) vive nella cella centrale
> col proprio metodo, non nella colonna: rimisurato oggi sulle righe fra le due recinzioni,
> **20606 byte LF su 303 righe**, invariato.
>
> ⚠️ **Archiviato qui insieme alla colonna, il paragrafo della prima misura**, che senza pesi
> non aveva più soggetto — diceva *«i pesi … si rimisurano quando si toccano i file che
> contano»*. Il testo: *«Prima misura il 2026-08-08: tre erano stantii, e il quarto — «insieme
> pesano oltre settecento kilobyte», in testa al compendio, in `CLAUDE.md` e in `AVVIO-CHAT.md`
> — era falso di un quarto. È il gotcha #31: un numero messo a sostegno di una regola giusta non
> viene mai rimisurato, perché nessuno dubita della regola.»*
>
> ⚖️ **Ciò che NON si chiude, dichiarato:** i pesi scritti a mano dentro il **messaggio** di
> [`../AVVIO-CHAT.md`](../AVVIO-CHAT.md) restano, e restano una decisione del proprietario — la
> §6 del compendio la porta col proprio richiamo. Questa passata toglie una casa, non la classe.
>
> ⚠️ **Fine-riga contati prima e dopo su ogni file toccato:**
> [`../COMPENDIO.md`](../COMPENDIO.md) e questo file restano **LF puri**, `CR = 0`. Tutte le
> scritture passano da uno script in un file che **costruisce e codifica tutto prima**, scrive in
> un temporaneo e fa `os.replace`, e **aborta senza scrivere** se un'ancora non cade esattamente
> una volta (gotcha **#82**).
