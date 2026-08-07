# Handoff — ripresa del progetto

Aggiornato il **2026-08-07**, alla chiusura della **§8** — la copertura V1–V37 e Q1–Q24 —
e poi di nuovo quando rileggere `tracciabilita.md` con un'altra domanda ha **riaperto la
spec su sette voci**, di cui tre chiuse più F1a. Serve a riprendere senza rifare, e senza
rilitigare ciò che è già deciso.

## In trenta secondi

Assistente desktop locale, utente singolo, GPU singola RTX 5080 16 GB. **Piattaforma a
quattro pilastri paritari** su kernel comune. Spec del kernel **§0–§10 completa, 35 ADR**.
Stack deciso **tranne il guscio della GUI**: core in **Rust**, interfaccia web in **Vue 3**,
worker ML in **Python**; Tauri contro Electron è ancora aperto
([ADR-0029](adr/0029-guscio-della-gui.md), `Proposed`) e non blocca nulla.

**La spec del sotto-progetto 1 ha §0–§8 approvate**, ed è **riaperta su sette voci**
trovate rileggendo `tracciabilita.md` con una domanda che nessuno le aveva posto. Il
codice non è ancora iniziato, e vale «spec prima del codice». Il piano viene **dopo** la
chiusura di **quelle ancora aperte** — vedi sotto.

✅ **La lacuna su I2 è chiusa.** La GPU usata dalla GUI è governata da
[ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md): **quota di presentazione
sottratta, con la concessione tenuta dal core.** Il kernel non ha più lacune aperte.

Il vincolo che governa tutto non è funzionale ma di risorsa: quattro aree che si contendono
una sola GPU.

⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
scrive **qui**: fra le due mancano le voci ancora aperte e poi il **piano**. Oggi l'unico
codice presente è in [`../spikes/rust/`](../spikes/rust/): sono **prove**, non il kernel —
ma la §2.5 della spec dice già riga per riga quali pezzi salgono a `kernel/` e quali
restano dove sono.

## Prima cosa da fare

**Chiudere le voci ancora aperte della riapertura, poi scrivere il piano.** Tre sono già
chiuse, e **F1a** — la dichiarazione della porta verso i worker — anche. Nessuna misura
blocca né le une né l'altro.

> ⏭️ **Il prossimo passo è F2 (+F7)**: l'evoluzione del formato durevole del giornale.
> Non è F1b, e la ragione è sotto: F1b progetta messaggi i cui campi finiscono in record
> durevoli, quindi ha bisogno che la regola esista già.
>
> ⏭️ **Poi F1b**: firme e messaggi della porta `process` in §5–§6, l'allargamento delle
> giustificazioni di §6.1.1 e §7.3.1, e la misura che
> [ADR-0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) lascia aperta —
> *`bincode` è decodificabile dal pari **Python**?* Se no, vale l'**esito B di M-1**, già
> misurato: tipi in `kernel`, serializzazione in `daemon`.

### Le sette voci, e come sono state trovate

`tracciabilita.md` risponde a *«dove vive questa funzionalità»*. Rileggendola con un'altra
domanda — ***«di quale meccanismo di kernel ha bisogno, e la spec lo nomina?»*** — sono
emerse sette voci che non reggono. È la stessa mossa della §8, applicata alla tracciabilità
invece che ai V/Q, ed è la terza volta che «rileggere con un'altra domanda» trova qualcosa.

⚠️ **La crepa sta nella legenda.** Un `📋` significa *«pianificata: sotto-progetto
assegnato»*, e **non** significa «non richiede un meccanismo di kernel». Tutto ciò che è
`📋` non era verificato su quel fronte.

| # | Voce | Voluta? | Classe §0.3 | Stato |
|---|---|---|---|---|
| **F3** | i **parametri di decisione** non erano consegnati al kernel | esplicita in §8.3 V3, ma con l'innesco della specie sbagliata | **B** | ✅ **chiusa** — [ADR-0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md), spec §2.8 |
| **F6** | la **VRAM totale** non aveva provenienza | implicita | conseguenza di F3 | ✅ **chiusa** con F3, spec §5.1 |
| **F5** | la porta `network` era descritta «verso i **provider**», V25 promette «un solo punto di uscita **verso la rete**» | implicita | una riga | ✅ **chiusa** — spec §2.3.1 |
| **F1** | nessuna porta per **parlare** con un worker: `process` era «avvio e uccisione» | implicita | **B** | 🔵 **F1a chiusa** ([ADR-0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md), §2.3.1); **F1b aperta** — il progetto in §5–§6 |
| **F2** | l'**evoluzione del formato durevole** del giornale non è decisa | implicita | **B** | ⬜ aperta |
| **F4** | l'**anello 3** non è collocato in §0.4, né dentro né fuori | implicita | **C**, ma va *scritta* | ⬜ aperta |
| **F7** | «il giornale lo consente» per **fork e branching** è un'affermazione della sola tracciabilità | implicita | converge in F2 | ⬜ aperta |

### L'ordine, e perché

Il criterio **non** è «prima la scrittura»: non esiste ancora costruzione, sono tutte
scrittura. È:

> **Si decide prima ciò che vincola una firma, poi ciò che la descrive.** Una descrizione
> scritta prima della decisione che descrive va riscritta comunque.

```
✅ F3 ─▶ F6         F3 chiusa: F6 è caduta con lei
✅ F1a ─▶ F5        stessa tabella, toccata una volta sola. Il bivio su I4
                    è deciso: ADR-0035, «singolo» = per canale privato
⬜ F2 ─▶ F7         F7 è un campo, se F2 dà la regola per aggiungerne
⬜ F1b (§5–§6)      firme, messaggi, formato di filo — DOPO F2
⬜ F4               indipendente, in qualsiasi momento
⛔ §8               per ultima, e UNA VOLTA SOLA
```

Il test che ha deciso l'ordine: **quale, decisa per ultima, costringerebbe a riaprire le
altre?** F3 per ultima riapriva F1 e F2; F2 per ultima non riapre niente. Ma F2 ha una
scadenza **temporale** e non logica: va chiusa **prima della prima riga di codice che
scrive un record**, perché aggiungere un discriminante a record già su disco è una
migrazione dell'unico archivio irriproducibile.

⚠️ **F2 precede F1b anche per una ragione logica, scoperta il 2026-08-07 sbagliando
l'ordine.** Il diagramma era stato letto come una preferenza, e non lo è: **F1b può creare
campi durevoli**. Il picco di VRAM che §5.2.2 fa entrare nel giornale *«accanto al passo»*
arriva dal worker, cioè da un messaggio che F1b progetta. Progettare quei messaggi prima
che F2 abbia dato la regola di evoluzione significa aggiungere campi a record **sotto
nessuna regola** — che è esattamente il modo di fallire per cui F2 esiste.

⚠️ **Tre propedeuticità di processo, che non sono fra le sette:** la §8 si tocca per
ultima e una volta sola, perché ognuna delle sette cambia una sua riga; **nessuna
rinumerazione** di sezioni, perché lo script legge §7.4 e §8 per posizione (gotcha #26);
e ogni correzione a una sezione approvata porta il proprio **richiamo datato**, come §8.5.

### Cosa ha chiuso F3, in tre righe

| | |
|---|---|
| **la regola** | nessuna decisione del kernel legge un parametro che non le è stato consegnato |
| **non è un quinto iniettabile** | i quattro di V29 sono sorgenti di *non determinismo*; un parametro è deterministico. Consegnarlo compra I3 **e** la variabilità sotto il seme, non la riproducibilità |
| **il guadagno che non c'era** | la DST può ora far variare i parametri col seme, quindi lo scenario di **RK-1** — quota audio + quota presentazione contro TRELLIS2 — diventa esplorabile *prima* che M5 lo misuri |

⛔ **Il limite di F3, dichiarato:** il compilatore **non può** vietare una costante scritta
dentro il kernel. Prova che una decisione **riceve** i propri parametri, non che non ne
abbia altri di nascosto — è il limite del gettone (§6.3.2). Lo copre solo la campagna, e
solo per i parametri che fa davvero variare.

### Cosa hanno chiuso F1a e F5, in quattro righe

| | |
|---|---|
| **il divario non era con I4** | era fra la §2.3 e `design/01`, che descriveva già il canale con tre verbi — *«Avvia, **istruisce**, uccide»* — e aggiungeva *«il flusso audio risale al core»*. La §2.3 ne aveva conservati due |
| **una porta sola, non due** | il dialogo entra in `process` invece di nascere accanto: l'oggetto con cui si parla a un worker è quello che restituisce l'avvio, e l'avvio pretende una concessione (§5.6). Spezzarli riaprirebbe la chiusura che ha portato I2 al **compilatore** |
| **la lettura di «singolo»** | un trasporto e uno schema **per canale privato**. I4 si **completa**, non si riformula: ADR-0004 riceve un rimando, come per I2 con ADR-0033 |
| **F5 non era una sfumatura** | `network` «verso i provider» avrebbe lasciato l'**esportazione OTLP** di ADR-0017 *fuori* dall'unico punto di uscita — cioè esattamente ciò che V25 vieta. Un secondo consumatore già deciso, e già escluso da una descrizione |

⛔ **Il limite di F1a, dichiarato:** resta aperta una domanda di stato dell'arte che **non
è stata misurata** — se `bincode` sia decodificabile dal pari **Python**. Non tocca la
dichiarazione della porta; tocca il **formato di filo**, che è F1b. Se la risposta è no,
vale l'esito **B** di M-1, già misurato e già prezzato: il confine di ADR-0031 non cresce.

⚠️ **E un follow-up di ADR-0028 è diventato un presupposto:** *«trattare l'ambiente Python
come artefatto da versionare»*. Il timbro di build regge sul secondo canale **solo se** il
worker è artefatto nostro; se non lo è, «non versionato» cade lì.

⚠️ **Il piano deve decidere anche _dove nasce il workspace_.** Alla radice non c'è nessun
`Cargo.toml`: il workspace delle cinque crate nasce alla radice escludendo gli spike,
oppure accanto ad essi. È l'unica domanda strutturale che la spec ha deliberatamente
lasciato al piano — con **un fatto in più che nessun documento nominava**: sotto `spikes/`
i progetti Cargo sono **due**, non uno — `spikes/rust/`, che è anche un **workspace
annidato**, e `spikes/gui-ipc/`.

✅ **Nessuna misura blocca il piano.** L'unica ancora aperta — M5 — richiede una GUI, cioè
il sotto-progetto 2.

⛔ **Due cose che il piano _non_ deve rifare**, perché la §8 le ha già chiuse: la copertura
V/Q e l'estensione di `check-docs.sh`. Entrambe sono in esercizio e provate in due
direzioni.

#### Cosa la spec consegna al piano — già deciso, e sparso

Nessuna di queste righe è una decisione da prendere: sono decisioni **prese**, che il piano
deve tradurre in passi. Raccolte qui perché cercarle una per una è il modo in cui se ne
perde qualcuna.

| # | Vincolo sul primo commit di codice | Da |
|---|---|---|
| 1 | **cinque crate**: `kernel` · `platform` · `secrets` · `simulator` · `daemon`. `kernel` non dipende da nessuna crate del progetto — è una riga del suo manifesto | §1.2 |
| 2 | `kernel` e `simulator`: `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]`. **`forbid`, non `deny`** | §1.4 · ADR-0026 |
| 3 | il manifesto **appunta `bincode` a `2`**, con la ragione scritta accanto: la `3.0.0` è un `compile_error!` | §6.1.1 · gotcha #22 |
| 4 | `rustup target add x86_64-unknown-none` è un **prerequisito dell'ambiente**, o la porta è rossa per il motivo sbagliato | §7.3.2 |
| 5 | il [`clippy.toml`](../spikes/rust/clippy.toml) di `spikes/rust/` **non sale**: a livello di workspace scatterebbe addosso a `platform` | §7.4.4 |
| 6 | l'aiutante `passo_in_dubbio` dello spike **non sale così com'è**: restituisce un passo, ne servono un insieme | §4.3 · gotcha #20 |
| 7 | il numero di semi della campagna breve è **fissato e versionato**, e il tempo di parete si stampa a ogni corsa | §7.5.3 |
| 8 | la cadenza: livello 1 a ogni compilazione (non «gira»), livello 2 a ogni commit, DST profonda su ciclo lungo | §7.5.1 |
| 9 | riga per riga, **cosa sale da `spikes/rust/` e cosa resta** | §2.5 |
| 10 | ogni regola nuova porta **due** sonde e un caso in `tests/compile_fail/` con il suo `.stderr` — da **leggere**, non da rigenerare in blocco | §7.1.4 · gotcha #25 |
| 11 | **nessuna decisione legge un parametro che non le è stato consegnato**: budget, quote, policy attiva, tetti. In sotto-progetto 1 i default sono letterali in `daemon` | §2.8 · ADR-0034 |

📌 **Cosa la §8 ha deciso, e che vale la pena non riscoprire:**

| | |
|---|---|
| **quattro stati, non tre** | `verificato qui` · `parziale` · `rimandato` · `non controllato`. Il quarto — `parziale` — esiste perché V25 e i «solo lato kernel» con tre stati si possono solo sopravvalutare o sottovalutare |
| **`parziale` e `rimandato` pretendono l'innesco** | ed è lo script a pretenderlo, non la buona volontà: è il gettone della §6.3 applicato a una tabella |
| **l'innesco è la _condizione_, il numero sta fra parentesi** | «esiste un'interfaccia (2)», non «sotto-progetto 2». Se la roadmap cambia, la condizione resta vera |
| **un Q della DST eredita lo stato della porta in cui si inietta** | incrociando §3.3 e §7.4.6: Q2 e Q5 sono ✅ perché `reactor` e `journal` hanno la suite di conformità; Q3, Q4, Q18, Q22 sono `parziale` perché le loro porte no |
| **il livello ⛔ è vuoto**, come il livello 3 del catalogo | nessun V e nessun Q è lasciato deliberatamente senza controllo. Ciò che §7.6.2 non controlla sono **pezzi** di V, dichiarati dentro la riga |
| **la porta non prova la correttezza, e la §8 non prova la verità** | prova che ogni V e ogni Q è stato **giudicato**. Lo script controlla che lo stato sia *espresso*, non che sia *giusto* |

✅ **Tre disallineamenti trovati dalla copertura, tutti chiusi** — §8.5. **Tre sezioni
approvate sono state corrette**, ciascuna con il proprio richiamo datato: §0.4, §0.6 e il
**catalogo §7.4** — la riga V31 in §7.4.2, che era l'unica priva di contro-sonda, e le tre
nuove in §7.4.1.

| # | Trovato | Chiuso come |
|---|---|---|
| 1 | la **§0.6 elencava Q21** fra i «verificati solo lato kernel», ma la **§0.4 non metteva il backup in perimetro**, né dentro né fuori | correzione **doppia**, perché la causa era a monte: §0.4 colloca il backup in «si scaglia» con **regola C** (nuova §0.4.1), e Q21 passa alla riga dei rimandati — §8.5.1 |
| 2 | **nessun sotto-progetto della roadmap collocava il backup** | **sotto-progetto 11 — Backup e ripristino**, dipendente da 5, 6 e 9. L'ordine è derivato: prima che 6 e 9 producano indici e pesi, l'elenco delle esclusioni di V32 è **vuoto**, e verificarlo sarebbe vacuo — gotcha #17. §8.5.2 |
| 3 | il **livello 1 del catalogo non enumerava** tre proprietà che le §5 e §6 avevano già deciso: **V2** (l'ammissione riceve un profilo), **V4** (esito a tre vie), **V10** (artefatto per riferimento immutabile) | entrano nel blocco C di §7.4.1, con sonda e contro-sonda. Il titolo del blocco è passato da «Tipi che non si scambiano» a **«Cosa non è esprimibile»**, perché **tre delle sue sei righe originali non erano scambi di tipo**. E **V16 è stato declassato** da `parziale` a `rimandato`: la metà che dichiarava verificata era vacua. §8.5.3 |

I primi due riguardano lo stesso oggetto — il **backup**, l'unica cosa del progetto di cui
nessuno era proprietario, quindi l'unica che nessuna sezione aveva motivo di nominare. Il
terzo lo ha trovato la §8 **contro sé stessa**, applicando la regola §8.1.2 alla propria
tabella: diciassette celle su sessantuno non la rispettavano.

📌 **La lezione, che vale più delle tre correzioni.** §0.4 e §0.6 erano state scritte nella
**stessa sessione** e rilette più volte: la contraddizione è sopravvissuta. È emersa solo
quando qualcosa ha costretto a rileggerle con una domanda diversa — *«dammi lo stato di
Q21»* — che è ciò che la §8 fa sessantuno volte. **Una tabella di copertura non serve solo
a non dimenticare: serve a rileggere con un'altra domanda.** E una regola che non rifiuta
mai niente è decorazione: la §8.1.2 ha rifiutato tre voci del catalogo e una riga della
propria tabella la prima volta che è stata applicata sul serio.

📌 **Cosa la §7 ha deciso, e che vale la pena non riscoprire:**

| | |
|---|---|
| **il catalogo ha ridotto tre voci invece di aggiungerne** | `HashMap` fuori dal kernel è **tolto** (non difende V29: in DST `platform` non gira affatto) · V28 è un **corollario** dell'allow-list, niente driver · V5 **sale al compilatore** |
| **il livello 3 è vuoto** | nessuna invariante del kernel poggia su un lint. `clippy` resta come igiene, senza voce nella porta |
| **il livello 1 non ha cadenza** | non «gira»: *è* il compilatore. Solo il livello 2 ha una cadenza |
| **la porta non prova la correttezza** | prova che un insieme **nominato** di invarianti regge. Un difetto che non viola nessun V passa verde — §7.6.3 |
| **il quarto gettone si scaglia** | V35/Q23: nessuna porta esegue comandi qui, ed è retrofittabile. L'innesco è scritto in §7.4.5 |

✅ **Le due cose che la §7 aveva lasciato alla §8 sono fatte:**

| # | | Esito |
|---|---|---|
| 1 | **estendere `check-docs.sh`** | ✅ due blocchi nuovi, **quattro asserzioni**: contro-sonda piena nel catalogo §7.4 · completezza e non-duplicazione delle voci in §8 · stato dentro l'insieme chiuso · innesco obbligatorio per `parziale` e `rimandato`. Con una **guardia di non-vacuità** che è la parte importante — §8.6.2 |
| 2 | **registrare i rimandati con l'innesco** | ✅ due specie distinte, e ne ho cercata una terza senza trovarla: una **misura** e un **ADR** tarano, non abilitano. §8.2 |

#### Il conteggio delle ventuno voci — ✅ chiuso, e il grep dà zero

Le ventuno voci che nessuna sezione nominava sono ora tutte giudicate. Rieseguito alla
chiusura della §8, il `grep` su `V<n>` e `Q<n>` restituisce **zero mancanti**.

Il ritratto che ne esce, contato il **2026-08-07** sulla tabella stessa:

| | ✅ verificato qui | ⚠️ parziale | ⏳ rimandato | ⛔ non controllato |
|---|---|---|---|---|
| **V** (37) | 18 | 12 | 7 | **0** |
| **Q** (24) | 9 | 8 | 7 | **0** |

⚠️ **Un terzo delle voci è `parziale`, ed è il ritratto onesto** di un sotto-progetto che
costruisce il kernel senza nessuno dei suoi consumatori. Chi legge la tabella cercando
conforto la leggerà male.

⛔ **Il livello `non controllato` è vuoto**, come il livello 3 del catalogo. Non è una
svista e non contraddice la §7.6.2: quella sezione dice che *la porta* non controlla Q6,
Q11, Q12 e Q16 — e la sua stessa colonna li rimanda alla §8. Qui sono `rimandati`, che è la
traduzione esatta. Il valore ⛔ significa un'altra cosa: *si sceglie di non controllarlo, e
nessun innesco lo riaprirà*. §8.7.1.

### ✅ Le due domande della §7 sono decise

Le aveva sollevate M-1 (§6.8.2) e M-3 le aveva rese concrete con dei numeri. Decise nella
**§7.3** il 2026-08-07.

| # | Domanda | Decisione |
|---|---|---|
| **1** | il controllo della allow-list misura il grafo di **runtime** o quello **totale**? | **entrambi, con due comandi e due rimedi distinti** (§7.3.1). Una violazione fra le crate *spedite* è `I3 violato` e si ripara **togliendo** la dipendenza; un cambiamento fra quelle *di build* è un evento da rivedere e si ripara **aggiungendola alla lista**. Le dipendenze di **sviluppo** sono escluse, e l'esclusione è provata |
| **2** | il **cancello bare-metal** entra fra i controlli automatici? | **si aggiunge alla lista, non la sostituisce** (§7.3.2), e il bersaglio passa a **`x86_64-unknown-none`**. I due falliscono in modo complementare: la lista **nomina il colpevole**, il cancello **prova** invece di enumerare |

📄 [`superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md)

## Stato del sotto-progetto 1

| § | Sezione | Stato | Cosa ha deciso |
|---|---|---|---|
| 0 | Perimetro e criterio di scaglionamento | ✅ | cosa entra e cosa si scaglia, con un criterio falsificabile a tre regole (A/B/C) |
| 1 | Struttura delle crate e regole di importazione | ✅ | cinque crate: `kernel` · `platform` · `secrets` · `simulator` · `daemon`. Più [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md), nato da una misura |
| 2 | Il substrato iniettabile | ✅ | esecutore nel `kernel`, `Reactor` come porta, nessun thread nel percorso decisionale |
| 3 | Il simulatore DST | ✅ | cosa sostituisce, tempo virtuale, iniezione dei guasti, il seme e cosa **non** è |
| 4 | Giornale, riconciliazione, persistenza | ✅ | write-ahead, riconciliazione su un **insieme**, [ADR-0032](adr/0032-motore-di-persistenza.md) `redb` |
| 5 | Arbitro GPU, e la lacuna su I2 | ✅ | tre consumatori GPU nella GUI, quota di presentazione, I2 sui worker imposto dal **compilatore**. Più [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md) |
| 6 | Gateway, sensori, permessi, degrado | ✅ | schema IPC in `kernel` con **`bincode`**, **timbro di build** contro la GUI stantia, il **gettone non falsificabile** nominato una volta, «costo» del sensore separato in due |
| **7** | **La porta di qualità: i controlli automatici** | ✅ | **scala di forza a tre livelli**, evidenze di M-3, le due decisioni sulle dipendenze, il **catalogo** con sonda e contro-sonda, la cadenza, il perimetro negativo. **Il livello 3 è vuoto** |
| **8** | **Copertura V1–V37 e Q1–Q24** | ✅ | **quattro stati** con innesco obbligatorio, **due specie di innesco**, la regola che un Q della DST eredita lo stato della propria porta, l'estensione di `check-docs.sh` provata in due direzioni. **Il livello ⛔ è vuoto** |

**§0–§8 approvate.** ⚠️ La spec è però **riaperta su sette voci** — vedi «Prima cosa da
fare». Le sezioni toccate finora portano ciascuna il proprio richiamo datato: §1.2, §2.0,
§2.3.1 (nuova), §2.8 (nuova), §3.1, §5.1, §7.4.1 e la riga V3 della §8.3.

### Le decisioni aperte dalla §0.5 — tre previste, tre emerse

| # | Decisione | Esito |
|---|---|---|
| 1 | GPU della GUI non arbitrata | ✅ [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md): quota di presentazione sottratta, concessione tenuta dal **core** |
| 2 | Motore di persistenza | ✅ [ADR-0032](adr/0032-motore-di-persistenza.md): `redb` 4.1.0 con backend nostro |
| 3 | Dove vive l'esecutore | ✅ nel `kernel`, con `Reactor` come porta (§2.4) |
| 4 | Dipendenze del kernel nel confine I3 | ✅ [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md) — **non prevista**, emersa da una misura |
| 5 | I parametri di decisione sono consegnati, non letti | ✅ [ADR-0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md) — **non prevista**, emersa dalla riapertura |
| 6 | La porta verso i worker, e la lettura di «singolo» in I4 | ✅ [ADR-0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) — **non prevista**, è la voce F1 della stessa riapertura |

### Misure eseguite, e quelle ancora aperte

Tutte con `rustc 1.95.0` · `cargo 1.95.0` · Windows 11. Evidenze complete nella spec.

| # | Domanda | Esito |
|---|---|---|
| M-4 | un runtime di ecosistema è usabile sotto `no_std`? | ✅ **sì** — l'attesa contraria era falsa. 55 crate nel grafo, fra cui `getrandom` |
| M-5 | un esecutore `no_std` senza `unsafe` fa avanzare `Future` reali? | ✅ **sì, con zero dipendenze**. Un `Waker` su misura invece **non** è costruibile: `E0133` |
| M-7 | quanto costa una decisione dell'arbitro? | `request` ≤ 100 ns · `release` p99 **500 ns** a coda realistica, 86,6 µs a coda 2000 |
| M-2 | `simulator` regge `no_std`? | ✅ **sì**. 100 corse → 1 traccia · 20 000 ms virtuali in **25,8 µs** · crash riproducibile 5/5 |
| M-8 | i quattro requisiti di §10.6 su `redb` | 1 ✅ · 2 ✅ · 3 ⚠️ si stabilizza in alto · 4 ✅ **12/12 crash recuperati** |
| M-6 | `BTreeMap`/`Vec` bastano alle strutture del kernel | ✅ **chiusa dall'esistenza di M-7**: il suo prototipo è `no_std`, zero dipendenze, tutto su `BTreeMap`, e l'arbitro è la struttura più complessa del kernel finora. Resta aperta solo per ciò che introdurrà la §6 |
| M-1 | serializzatore per lo schema IPC con **grafo transitivo** accettabile | ✅ **sì, tutti e cinque i candidati provati.** Scelto `bincode` 2.0.1 (2 crate di runtime). Esito **A**: lo schema sta in `kernel`, il grafo di §1.2 non cambia |
| M-3 | allow-list di ADR-0031 esprimibile con la toolchain standard, provata in negativo | ✅ **sì, esito A** — con `cargo tree`, **non** con `cargo metadata`. Sonde N1–N4 e B1–B3, entrambe le direzioni dell'errore. **Evidenze nella §7.2 della spec**; qui sotto resta solo la correzione al comando |
| **M5** | quanta VRAM prende la presentazione della GUI | ⬜ **aperta e dichiarata tale** — richiede una GUI: sotto-progetto 2, accanto a M1–M4 di ADR-0029 |

#### M-3 — ✅ evidenze trasferite nella spec

Le evidenze complete — esito A, lo scarto fra `cargo metadata` e `cargo tree`, le sonde
N1–N4 e B1–B3 — **vivono ora nella §7.2 della spec del sotto-progetto 1**, che è la loro
sede unica. Qui restano solo le due cose che non stanno lì.

**1 · Una riga di questo documento era sbagliata, e va saputo.** HANDOFF affermava che
`cargo tree -e no-proc-macro` separa il grafo di runtime da quello totale.

> ⛔ **Non li separa.** Da solo toglie i generatori di codice ma lascia dentro l'intero
> sottoalbero delle dipendenze **di sviluppo**, e con esso `windows-sys`. Su un workspace
> con `trybuild` — che il kernel avrà, §2.5 — restituisce **venti** crate invece di due.
> Il comando corretto è **`-e normal,no-proc-macro`**.

**Perché M-3 non poteva accorgersene:** il suo workspace di prova non aveva dipendenze di
sviluppo, e senza quelle i due comandi danno la stessa risposta. La sonda **non poteva
falsificare l'affermazione** — gotcha #17 applicato a M-3 stessa. Riverificato il
2026-08-07, con la contro-sonda che lo dimostra.

**2 · La correzione che M-3 aveva imposto alla §6.1.1**, già applicata: `simulator` non
aggiunge voci proprie, ma la sua lista **non è vuota** — dipende da `kernel`, e la regola 2
è sul grafo *transitivo*. Scritto «resta vuota», misurato `bincode kernel unty`.

Lo stack non è più una domanda aperta:

| ADR | Decisione | Misurata da |
|---|---|---|
| **0026** | core in **Rust** | SP-5 e SP-6 su tre candidati. Rust è l'unico che passa entrambi |
| **0027** | GUI a **interfaccia web**, non toolkit nativo | G7, con P1–P4 misurati su un prototipo IPC |
| **0028** | worker ML in **Python** | non una scelta: i modelli hanno implementazioni Python |
| **0029** | ⚠️ **guscio: aperto** — Tauri o Electron | **niente**: sono argomenti, non misure. È il motivo per cui è `Proposed` |
| **0030** | interfaccia in **Vue 3** | merito + competenza del proprietario, criterio legittimo qui e non in ADR-0026 |

### L'unica cosa aperta, e perché non blocca

| Aperta | Si chiude con | Blocca il sotto-progetto 1? |
|---|---|---|
| **guscio della GUI** (ADR-0029) | cinque misure **M1–M5** su un frontend Vue minimo con scena 3D, sui due gusci | **no**: il sotto-progetto 1 è interamente Rust e non tocca la GUI |

#### Come la lacuna su I2 è stata chiusa — [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md)

Il problema: [ADR-0005](adr/0005-arbitrato-gpu-su-due-dimensioni.md) e
[design/02](design/02-arbitrato-gpu.md) non menzionavano mai la GUI, e la verifica di I2
era scritta sui soli worker.

**Le tre uscite enumerate non erano tre opzioni per un problema: erano tre risposte
parziali per tre consumatori diversi**, che erano stati trattati come uno solo.

| # | Consumo GPU della GUI | Governo | Rifiuto esecutivo? |
|---|---|---|---|
| 1 | compositing della webview | quota di presentazione sottratta, **concessione tenuta dal core** | ❌ no |
| 2 | viewer 3D entro la quota | stessa quota | ❌ no |
| 3 | viewer 3D oltre la quota | concessione ordinaria via IPC, prelazionabile | ✅ sì |

Tre cose da sapere, se qualcuno riapre il tema:

| | |
|---|---|
| **il titolare è il core, non la GUI** | la GUI non può *chiedere*: chi alloca è il compositor, che non ha un percorso di richiesta. Una quota sottratta **senza titolare** lascerebbe I2 falso — gotcha #4 |
| **I2 per la GUI è più debole in natura** | verso un worker il rifiuto è esecutivo, verso il compositor no. La quota è una promessa di budget, non un'imposizione. Dichiarato, non nascosto |
| **una divergenza registrata** | HANDOFF affermava che l'uscita A «incrina I1». **Non regge**: un worker tiene una concessione ed è dichiarato `possiede: nulla`. A è stata scartata per un motivo diverso e più forte |

Vale identico per Tauri e per Electron: **è una questione di kernel, non di guscio** — e
la §5 lo ha confermato con un motivo, esportando verso ADR-0029 il discriminante **M5**
invece di importarne uno.

Toolchain verificata il 2026-08-06: `rustc` 1.95.0 · `cargo` 1.95.0 · `clippy` 0.1.95.

### I quattro vincoli che ADR-0026 impone alla prima riga di codice

Conseguenze **misurate**, non raccomandazioni. Vanno tradotte in controlli automatici.

| # | Vincolo | Perché |
|---|---|---|
| 1 | il kernel è una **crate propria**, la piattaforma un'altra | i confini sono a granularità di crate, non di modulo |
| 2 | `#![forbid(unsafe_code)]`, **non** `deny` | `forbid` non è scavalcabile da un `#[allow]` locale (`E0453`) |
| 3 | la crate del kernel è `#![no_std]` + `alloc` | è ciò che rende `E0433` un errore del **compilatore** e non un lint |
| 4 | **`std::collections::HashMap` vietato** | vedi gotcha #12 |

## Non rilitigabile

34 ADR in stato `Accepted`. Rimetterne in discussione uno **richiede un ADR
nuovo che lo superi** (`Superseded by`), non una conversazione. Le decisioni che
è più probabile qualcuno voglia riaprire per comodità, e la ragione per cui non si fa:

| Decisione | Se la riapri |
|---|---|
| I quattro pilastri sono **paritari**; nessuno ha accesso privilegiato al kernel (ADR-0001) | il kernel diventa il servo di un pilastro e gli altri tre restano cittadini di seconda classe per sempre |
| **Tre** classi di processo, non quattro (ADR-0004) | la quarta si giustifica contro la tabella, o non si fa |
| **Nessun codice di terzi in-process** (ADR-0003) | rientrano contratto pubblico da congelare e superficie d'attacco |
| Default **OpenRouter, VRAM libera** (ADR-0006) | lo swap coordinato passa da eccezione a caso normale e cambia tutta la UX di attesa |
| **Fail-closed** sui vincoli dei dati (ADR-0012) | la protezione torna a essere una preferenza |
| Il **contesto è una proiezione**, non lo stato (ADR-0008) | le run lunghe tornano a perdere informazione in modo irreversibile |
| **Nessun modello** nel percorso decisionale del kernel (ADR-0020) | un fallimento del kernel smette di essere sempre un difetto, e la DST diventa impossibile |
| L'anello 4 **propone**, l'utente approva (ADR-0009) | il harness si auto-modifica in silenzio e diventa indebuggabile |
| Il core è **Rust** (ADR-0026) | riaprirlo significa rifare SP-5 e SP-6, i cui esiti sono misurati e registrati con seed e versioni. Il criterio che ha deciso è lo **spareggio #1**, e discende da V29 e ADR-0021: **rimettere in discussione il linguaggio significa rimettere in discussione la DST**, non il linguaggio |
| Le **dipendenze del kernel** sono parte del confine I3 (ADR-0031) | `no_std` blocca solo il *nominare* `std`. Misurato: una crate con `no_std` **e** `forbid(unsafe_code)` legge un file dal disco attraverso una dipendenza. Senza la lista, I3 è controllato su un lato solo |
| Il motore è **`redb` con backend nostro** (ADR-0032) | il backend nostro non è un dettaglio: è il punto in cui il **livello 2** di crash diventa iniettabile. Prenderne uno con l'I/O non sostituibile rinuncia a metà della verifica |
| L'**esecutore vive nel kernel** (§2.4) | prendere un runtime di ecosistema restituisce a lui l'ordine delle attività — cioè esattamente il controllo che lo spareggio #1 aveva comprato escludendo Go |
| La **concessione di presentazione la tiene il core** (ADR-0033) | la scorciatoia tentante è «esentiamo la GUI e amen». Esentarla rende **I2 falso** e indebolisce Q2 in silenzio; darla in mano alla GUI crea una concessione che si perde ogni volta che la GUI muore — cioè in qualsiasi istante, per G3. Il titolare deve avere vita lunga, e l'unico che ce l'ha è il core |
| Il controllo delle dipendenze misura **due grafi con rimedi opposti** (§7.3.1) | unificarli sembra una semplificazione e non lo è: insegna il riflesso «aggiungi alla lista» **anche per una violazione di I3**, dove aggiungere alla lista non è un rimedio ma la violazione scritta in un modulo. È così che un'invariante si degrada in scartoffia |
| Il **cancello senza OS si aggiunge**, non sostituisce la lista (§7.3.2) | sembrano ridondanti e non lo sono: la lista coglie una crate **nuova**, il cancello una crate **già ammessa** che raggiunge l'OS per una via non prevista. E quando falliscono, **solo la lista dice il nome del colpevole** |
| Il **livello 3 del catalogo è vuoto** (§7.4.3) | la tentazione è aggiungere un lint «tanto non costa niente». Costa: un rosso della porta deve significare sempre «invariante violata», mai «stile discutibile», o si impara a ignorarlo |
| L'**innesco è obbligatorio** per `parziale` e `rimandato` (§8.1) | sembra burocrazia e non lo è: è l'unica cosa che impedisce a `parziale` di diventare la casella comoda in cui parcheggiare tutto. Toglierla riporta alla situazione che la §0.6 chiamava «rimandato tende a diventare dimenticato» — con in più una tabella che sembra dire il contrario |
| La **guardia di non-vacuità** dei controlli nuovi (§8.6.2) | è il pezzo che sembra più togliibile e il solo che non si può togliere. Senza, basta rinumerare una sotto-sezione perché due controlli smettano di controllare **uscendo verdi**: gotcha #26. E il «miglioramento» sbagliato è metterci un numero atteso di righe, che diventa rosso quando la tabella cresce per un motivo legittimo |
| I **parametri di decisione sono consegnati**, non letti (ADR-0034) | la scorciatoia tentante è «tanto il budget è 16 GB, scrivilo e basta». Una costante nel kernel **non fa scattare nessun controllo del catalogo §7**: si scopre solo quando qualcuno prova a farla variare in campagna e non può. È il gotcha #12 su un altro asse — e toglie alla DST l'unico modo di esplorare lo scenario di RK-1 |
| Il **dialogo con un worker vive dentro `process`** (ADR-0035) | la scorciatoia tentante è «è tutto IPC, mettiamolo su `ipc`». Sembra un accorpamento e invece **spezza la vita di un worker fra due porte**: l'avvio pretende una concessione (§5.6), e se il dialogo passa da un'altra porta quella catena non copre più il parlare. Si perde il meccanismo che ha portato I2 dal test al **compilatore**, e lo si perde senza che nulla diventi rosso |

## Le tre proprietà che non si aggiungono dopo

Se le trascuri, la correzione non è una patch: è una riscrittura.

| # | Proprietà | Da |
|---|---|---|
| 1 | Confine dei dati non fidati **nel sistema di tipi** | I6 · ADR-0014 |
| 2 | Nessuna chiamata OS-specifica nel kernel | I3 · ADR-0002 |
| 3 | **Iniettabilità** di tempo, casualità, I/O e scheduling — e dei **parametri di decisione**, che sono l'altro asse | V29 · ADR-0021 · **ADR-0034** |

Più una quarta, di natura diversa ma altrettanto vincolante: **nessuna esecuzione di
codice o comando sotto il livello 2 di confinamento** (V35 · ADR-0025).

## I gotcha

Trappole reali, alcune trovate correggendo errori già commessi in questo progetto.

| # | Trappola | Perché fa male |
|---|---|---|
| 1 | **«Tutto è una run» vale solo per l'inferenza _generativa_** | applicarlo a wake word, VAD e trascrizione continua giornalerebbe migliaia di frammenti: viola Q1 e riempie il giornale di rumore. Quelle sono **sorgenti di eventi**, mai passi |
| 2 | **Ritentativo o passo nuovo?** Il discriminante è: *il modello ha prodotto output?* | no (trasporto, 5xx, rifiuto dell'arbitro) → stesso passo. Sì ma respinto da un sensore → passo nuovo, perché quell'output esiste, è stato pagato e deve restare visibile all'anello 4 |
| 3 | **Policy VRAM ≠ destinazione della richiesta** | V3 riguarda *cosa risiede in memoria*. In policy LOCALE una singola richiesta può finire su un provider remoto senza che la policy cambi |
| 4 | **La quota audio sottratta non esenta da I2** | il worker audio ha una concessione *permanente e non prelazionabile*, non l'assenza di concessione |
| 5 | **I permessi applicativi non sono un confine contro codice eseguito** | un processo figlio non passa dal mediatore: apre ciò che l'utente può aprire. Serve il livello 2 |
| 6 | **«Cifrato a riposo» qui vale quanto l'account OS** | va detto *in interfaccia*, non solo nell'ADR. Una falsa sicurezza è peggio di nessuna sicurezza |
| 7 | **Il giornale è la sorgente; trace, contesto, costi e metriche sono proiezioni** | non costruire un secondo sistema di osservabilità: esiste già, ed è il giornale |
| 8 | **Ogni requisito Q deve avere un metodo di verifica** (V30) | la §10 ha violato questa regola appena scritta, aggiungendo Q21–Q24 senza metodo. `scripts/check-docs.sh` ora lo rileva |
| 9 | **Go non ha test di compilazione fallita di serie** | un driver che compila un file *fuori* dal modulo fallisce per il motivo sbagliato: falso positivo. Va tenuto dentro il modulo, dietro un build tag |
| 10 | **xorshift resta bloccato su zero** | senza guardia sullo stato iniziale, certi seed producono una traccia vuota e lo spike sembra passare |
| 11 | **Il contesto degrada _prima_ che la finestra si riempia** (context rot) | compattare all'overflow significa lavorare degradati per gran parte di una run lunga. Si tiene un **budget target**, non una soglia |
| 12 | **`std::collections::HashMap` viola V29** | `RandomState` è seminato casualmente **per processo**: l'ordine di iterazione non è riproducibile fra esecuzioni. Non compare in nessun elenco di «chiamate OS» e si manifesta come traccia divergente e inspiegabile. Usare `BTreeMap`, o un hasher fissato. *(Vale anche altrove: in Go la randomizzazione delle `map` è deliberata — misurate 8 sequenze distinte su 200 iterazioni della stessa map, e lì non c'è alternativa ordinata nella libreria standard.)* |
| 13 | **Un lint non è il compilatore** | `clippy` ferma la violazione ma `cargo build` no, e un `#[allow]` per riga la annulla. Solo `forbid` e `no_std` producono un divieto non scavalcabile. **Misurato**: la regola clippy ha bloccato un uso *legittimo* di `Instant::now()` in un test, e ha richiesto un `allow` — cioè ha dimostrato di essere aggirabile mentre faceva il proprio lavoro |
| 14 | **Un test negativo va provato _in negativo_** | il piano degli spike conteneva **due sonde di non-vacuità sbagliate su tre**: quella di TypeScript modificava il tipo sbagliato e il controllo passava comunque, quindi non provava nulla. Un controllo che non si è visto fallire **non è un controllo**. Vale per ogni test di compilazione fallita, per ogni regola di importazione, per ogni `grep` di conformità |
| 15 | **Un'evidenza scritta prima della misura è un'ipotesi, non un risultato** | il piano dettava il testo delle evidenze da riportare. Tre di quelle affermazioni sono risultate **false** alla misura — inclusa una che nascondeva un buco reale nel confine dei tipi. Si esegue, si misura, si registra ciò che si è visto; dove diverge, si registra la divergenza |
| 16 | **`no_std` impedisce di _nominare_ `std`, non di _raggiungere_ l'OS** | non è transitivo sul grafo delle dipendenze. **Misurato**: una crate con `#![no_std]` **e** `#![forbid(unsafe_code)]` — gli attributi esatti che ADR-0026 impone al kernel — legge un file dal disco e l'orologio di sistema chiamando una dipendenza, e *compila ed esegue*. ADR-0026 resta corretto: dice che `E0433` blocca `std::fs`, e lo blocca. Ciò che non era mai stato misurato è che bastasse a garantire I3. **Non basta**: la lista delle dipendenze del kernel è l'altra metà del confine. Evidenze e comandi in [`specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) §1.4.1 |
| 17 | **Iniettare un guasto dove il codice non arriva è una prova _vacua_** | è il gotcha #14 travestito da successo: la prima misura sui crash di `redb` iniettava alle operazioni 12, 20 e 33, che quella transazione non raggiungeva mai. Tre prove su cinque non provavano nulla, e stavano per essere riportate come `5/5`. **Si conta prima quante operazioni compie davvero il codice**, e si inietta dentro quel numero — poi si verifica che il guasto sia *scattato*, non solo che il test sia passato |
| 18 | **Misurare il transitorio invece del regime** | la prima misura sulla potatura di `redb` guardava **un solo giro** e concludeva «lo spazio non viene riusato». Falso: a regime si stabilizza (32 900 KiB identici a 4, 6 e 8 giri). Per qualunque proprietà di stato stazionario, un solo giro non è una misura |
| 19 | **Un avanzamento nullo dichiarato riuscito è un ciclo infinito** | nel reattore a orologio virtuale, `advance()` prendeva il minimo di *tutte* le scadenze. Quelle dei task già conclusi sono nel passato: l'orologio non si muoveva e la funzione diceva di aver avanzato. Va filtrato alle scadenze **strettamente future**, e l'esecutore deve avere una **guardia sui giri** — un blocco va visto come errore, non come test che non finisce |
| 20 | **Un crash lascia _più_ passi in dubbio, non uno** | con esecuzione interlacciata due run possono avere entrambe l'intento scritto quando il processo cade. **Misurato**: seme 99 → passi `[3, 7]`. ADR-0007 diceva già «per *ogni* passo in dubbio», quindi la semantica reggeva — ma **l'aiutante `passo_in_dubbio` dello spike non sale così com'è**: restituiva un solo passo perché assumeva sequenzialità, e con l'interlacciamento dà un **falso negativo** |
| 21 | **Il rifiuto dell'arbitro è esecutivo solo verso ciò che avviamo noi** | un worker che riceve `Rifiutata` non parte: il rifiuto *è* il meccanismo. Il compositor della webview **compone lo stesso**, perché non lo avviamo noi, non dichiara un profilo e non ha un percorso di richiesta. Verso di lui una quota è una **promessa di budget, non un'imposizione**, e I2 vale in una forma più debole *in natura* — non per implementazione mancante. Il corollario che quasi sfugge: una quota sottratta **senza titolare della concessione** non salva I2 affatto (è il gotcha #4 letto al contrario). Il titolare dev'essere un processo a vita lunga, e l'unico è il core — ADR-0033 |
| 22 | **Che una versione esista non vuol dire che funzioni** | `cargo add bincode` risolve alla **3.0.0**, che è l'ultima pubblicata e il cui **intero sorgente** è `compile_error!("https://xkcd.com/2347/")`: un segnaposto contro l'occupazione del nome. La versione utile è la `2.0.1`, e il manifesto va **appuntato a `2`** con la ragione scritta accanto, o il prossimo aggiornamento «sistema» il vincolo e rompe la build. È la stessa classe della riga su `sled` in ADR-0032, ma peggiore: lì la versione utile era solo più vecchia, qui **la più recente esiste ed è inutilizzabile**. Corollario: in una misura sui candidati, `cargo add --dry-run` dice che il nome si risolve — **non** che il codice compili |
| 23 | **`cargo metadata` non risolve le feature; `cargo tree` sì** | i due strumenti danno grafi diversi sullo stesso workspace, e la differenza è grande. `cargo metadata` riporta correttamente le *feature attive* di ogni nodo, ma il suo elenco `deps` **le ignora**: elenca anche le dipendenze opzionali **spente**. **Misurato**: sul kernel con `bincode` senza la feature `serde`, `cargo metadata` segnalava **11** crate esterne — fra cui `serde` e `syn`, che non vengono compilate — contro le **2** reali di `cargo tree`. Un controllo di allow-list costruito sull'interfaccia macchina «giusta» sovra-segnala di 5×. Costo dell'alternativa, dichiarato: `cargo tree` è pensato per gli umani e non garantisce la stabilità del formato |
| 24 | **Un controllo si prova in _due_ direzioni, non una** | il gotcha #14 copre metà del problema: un controllo mai visto fallire non è un controllo. L'altra metà è che **un controllo che scatta dove non deve è peggio di uno assente**, perché insegna a ignorare l'audit. In M-3 la sonda decisiva è stata **N4**: mettere `getrandom` dentro `platform` — dove ADR-0031 lo **ammette** — e verificare che il controllo **resti verde**. Senza quella sonda, una regola troppo larga sarebbe passata per una regola che funziona. È la stessa ragione per cui `check-docs.sh` conta le sezioni duplicate **per file** e non sull'insieme |
| 25 | **Rigenerare in blocco le evidenze di un test negativo lo trasforma in una tautologia** | un test di compilazione fallita in Rust confronta l'errore prodotto con un file `.stderr` salvato accanto al caso: è **ciò che gli impedisce di fallire per il motivo sbagliato** (gotcha #9 in forma Rust). Ma `trybuild` offre un modo di riscrivere **tutti** gli `.stderr` sull'output corrente. Serve quando i messaggi cambiano legittimamente; usato senza leggerli, ogni caso diventa «l'errore atteso è quello che è uscito» e la suite **passa per sempre**, restando verde. La rigenerazione è un atto deliberato e **si legge nel diff**, come aggiungere una voce alla lista di ADR-0031. Corollario che vale oltre `trybuild`: ogni volta che l'oracolo di un test è un file generato dal test stesso, aggiornarlo automaticamente **cancella l'oracolo** |
| 26 | **Un controllo che delimita il proprio bersaglio per intestazione si spegne quando qualcuno rinumera — e si spegne _verde_** | è il gotcha #14 in una forma che #14 non copre: quel controllo **è stato visto fallire**, quindi era un controllo vero. Poi qualcuno rinomina `#### 7.4.1`, l'intervallo non trova più righe, e uno script che non ha niente da controllare **esce con successo**. Il segnale è indistinguibile da «tutto a posto». Rimedio, applicato in §8.6.2: **se un delimitatore non si trova, o l'intervallo è vuoto, è un fallimento**. Sonde S6, S6b e S6c. ⚠️ E il rimedio sbagliato è mettere a guardia un **numero atteso** di righe: diventerebbe rosso il giorno in cui la tabella cresce per un motivo legittimo, cioè il gotcha #9 applicato allo script. Si verifica che i delimitatori esistano, non quante righe ci siano — a meno che l'elenco non sia canonico, come i V1–V37, dove la completezza *è* il controllo |
| 27 | **La legenda di una tabella risponde a una domanda sola, e chi la legge ne assume un'altra** | `tracciabilita.md` risponde a *«dove vive questa funzionalità»*. Il suo `📋` significa «sotto-progetto assegnato» — **non** «non richiede un meccanismo di kernel». Nessuno l'aveva mai letta con la seconda domanda, e leggerla così ha **riaperto la spec su sette voci**, tre di classe B. È il gotcha #26 spostato dai controlli ai documenti: una tabella che non ha mai rifiutato niente non sta verificando niente. ⚠️ Il rimedio non è riscrivere la legenda: è **rileggere con un'altra domanda**, che è ciò che la §8 ha fatto sessantuno volte e questa volta ha fatto la tracciabilità |
| 28 | **Un parametro non consegnato è una costante, e una costante è invisibile** | V29 rende sostituibile ciò che il mondo *risponde*. Non dice nulla sui **parametri con cui il kernel è configurato** — budget della GPU, quote sottratte, policy attiva, tetti di autonomia — e ciò che non viene consegnato finisce scritto dentro. Non compare in nessun elenco, non fa scattare nessuna voce del catalogo, e **si manifesta solo come uno scenario che la campagna non può esplorare**: con le quote fisse, RK-1 è irraggiungibile. Chiuso da ADR-0034. ⛔ E il limite resta dichiarato: il compilatore prova che una decisione *riceve* i propri parametri, **non** che non ne abbia altri di nascosto |
| 29 | **La riga di _verifica_ di un'invariante è il punto in cui l'invariante si restringe in silenzio** | Le sei invarianti di ADR-0004 hanno due colonne: l'**enunciato** e **come si verifica**. La seconda è più corta, più concreta, e viene letta al posto della prima — ma è scritta guardando i casi che esistevano quel giorno. È già successo **due volte**: I2 diceva «nessun **worker** si avvia senza concessione», e copriva una classe di processo su tre (chiuso da ADR-0033); I4 dice «nessun **consumatore esterno**», che parla di esternalità e non di quanti dei nostri processi parlino il protocollo (chiuso da ADR-0035). In entrambi i casi l'enunciato era giusto e nessun controllo poteva accorgersene, perché **la riga di verifica _è_ il controllo**. ⚠️ Il rimedio non è riscrivere le invarianti: è che **completare una riga di verifica non è superarla** — ADR-0004 non è mai stato superato, ha ricevuto due rimandi. Chi trova il terzo caso lo aggiunga qui invece di aprire un dibattito sull'invariante |

## Il metodo di lavoro

Non è preferenza estetica: ha prodotto quattro incoerenze reali intercettate prima che
diventassero codice.

| Regola | |
|---|---|
| **Spec prima del codice** | nessun sotto-progetto si implementa senza spec approvata |
| **Sezione per sezione** | si propone, si discute, si approva, si scrive. Mai tutto insieme |
| **Decidere sul merito** | né scorciatoie né sovra-ingegnerizzazione. «Non pigro» **non** significa «più costoso»: la topologia a micro-servizi è stata scartata *perché* più costosa e peggiore |
| **Rendere verificabile** | un principio che non si può controllare è un'intenzione. Le invarianti diventano test |
| **Dichiarare i costi** | ogni ADR elenca cosa peggiora, non solo cosa migliora. Un ADR senza `Negative (accettate)` è incompleto |
| **Stato dell'arte verificato** | se una nozione non è certa si cerca **prima** di scrivere, e la fonte si traccia in [`riferimenti.md`](riferimenti.md). Mai inventare |
| **Schema-first** | tabelle, diagrammi, elenchi numerati. Niente muri di testo |
| **Audit a ogni chiusura** | `bash scripts/check-docs.sh` — link, indici, numerazioni, V30, ADR pendenti |

## Cosa NON rifare

| | |
|---|---|
| ❌ ri-derivare l'architettura | è in **35 ADR**, ciascuno con alternative scartate e motivo |
| ❌ riscrivere `tracciabilita.md` da zero | 170 funzionalità già mappate: si **aggiorna**, non si rigenera |
| ❌ ri-cercare lo stato dell'arte già tracciato | è in `riferimenti.md` con le fonti. Verificane semmai l'invecchiamento |
| ❌ rifare gli spike SP-5 e SP-6 | esiti, seed, versioni e comandi sono in [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md). I prototipi esclusi sono recuperabili dalla storia git, lo SHA è lì |
| ❌ rifare le misure **M-1 · M-2 · M-3 · M-4 · M-5 · M-6 · M-7 · M-8** | tutte chiuse, con comandi, versioni e sonde nella spec: M-1 §6.8 · M-2 §3.6 · M-3 §7.2 · M-4 e M-5 §2.6 · M-6 §5.8.1 · M-7 §2.6 · M-8 §4.7. L'unica aperta è **M5** (senza trattino), e richiede una GUI |
| ❌ riaprire le **due decisioni della §7.3** | erano le uniche domande che la §7 doveva prendere, e sono state prese dopo aver misurato. Riaprirle richiede una misura nuova, non un'opinione |
| ❌ progettare una capacità L2 | prima il kernel deve esistere (ADR-0001) |
| ❌ promuovere l'aiutante `passo_in_dubbio` dello spike | assume esecuzione sequenziale: con l'interlacciamento dà un **falso negativo**. Gotcha #20 |
| ⚠️ fidarsi delle fonti senza data | l'ecosistema si muove a cadenza mensile; `riferimenti.md` riporta la data di consultazione |

## Domande legittimamente aperte

Non sono lacune: sono decisioni **rimandate con criterio**, e ciascuna ha già il metodo
per chiudersi.

| Domanda | Si chiude con | Blocca? |
|---|---|---|
| ~~Linguaggio del core~~ | ✅ **ADR-0026: Rust** | — |
| ~~Interfaccia web o toolkit nativo~~ | ✅ **ADR-0027: web** | — |
| ~~Ecosistema dei worker ML~~ | ✅ **ADR-0028: Python** | — |
| ~~Framework dell'interfaccia~~ | ✅ **ADR-0030: Vue 3** | — |
| ⚠️ **Guscio: Tauri o Electron** | ADR-0029 `Proposed`, misure **M1–M5** | no |
| ~~GPU della GUI non arbitrata~~ | ✅ **ADR-0033**: quota di presentazione, concessione tenuta dal core | — |
| **Quanto vale la quota di presentazione** | **M5**, insieme a M1–M4 | no: default conservativo dichiarato non misurato |
| ~~Motore di persistenza~~ | ✅ **ADR-0032: `redb`** con backend nostro | — |
| CPU della GUI con rendering reale (P3) | rimisura nel sotto-progetto 2 | no: il margine misurato è 21,4 % su 25 %, **stretto** |
| Curva qualità/VRAM di TRELLIS2 | SP-1 | no: tara i profili di risorsa |
| Voce < 600 ms sotto carico | SP-2 | no |
| Budget della proiezione per modello | SP-3 | no: vale un default conservativo, dichiarato |
| Provider con annullamento senza addebito | SP-4 | no |

## Mappa dei documenti

| File | Cosa contiene |
|---|---|
| [`../CLAUDE.md`](../CLAUDE.md) | istruzioni operative per l'agente |
| [`roadmap.md`](roadmap.md) | dodici sotto-progetti, ordine, dipendenze, decisioni aperte |
| [`tracciabilita.md`](tracciabilita.md) | 170 funzionalità → dove vive ciascuna |
| [`README.md`](README.md) | indice di ADR e diagrammi |
| [`adr/`](adr/) | **35 decisioni**. Leggi **0001** e **0004** per primi: tutto il resto ne discende. Poi **0026** (linguaggio) se devi scrivere codice |
| [`design/`](design/) | 9 diagrammi Mermaid della struttura corrente |
| [`superpowers/specs/`](superpowers/specs/) | la spec del kernel §0–§10, **e quella del sotto-progetto 1** — §0–§8 complete, con tutte le evidenze delle misure |
| [`superpowers/plans/`](superpowers/plans/) | il piano dello stack — **eseguito**, con l'errata in testa che documenta cosa il piano sbagliava |
| [`riferimenti.md`](riferimenti.md) | fonti esterne, con data e con **cosa non abbiamo adottato** |
| [`../spikes/`](../spikes/) | **prove, non kernel.** `PROTOCOLLO.md` criteri e soglie · `CANDIDATI.md` pre-selezione · `RISULTATI.md` esiti, seed, versioni, evidenze · `GUI-REQUISITI.md` G1–G21 e P1–P4 |
| [`../spikes/rust/`](../spikes/rust/) | il prototipo vincente: confine dei tipi, esecutore deterministico, `Future` native, giornale write-ahead. **Punto di partenza del simulatore** |
| [`../spikes/gui-ipc/`](../spikes/gui-ipc/) | prototipo IPC con P1–P4 misurati |
| `../scripts/check-docs.sh` | controllo di coerenza, verificato anche in negativo. **Da eseguire prima di ogni commit di documentazione** |

## Come si aggiorna questo handoff

Alla chiusura di ogni sotto-progetto, **nello stesso passaggio**: `roadmap.md`,
`tracciabilita.md`, lo stato degli spike, `CLAUDE.md` se cambia il prossimo passo, e
questo file se emergono gotcha nuovi.

`tracciabilita.md` **non** è stato toccato in questa sessione: nessuna funzionalità ha
cambiato sede, e la regola dice di aggiornarlo alla *chiusura* del sotto-progetto. Resta
quindi da aggiornare **quando il sotto-progetto 1 chiude**, non alla fine della §8.

### Tre trappole di `check-docs.sh`, da sapere prima di scrivere

**1 · I conteggi.** La guardia confronta con la realtà **ogni** occorrenza di
`<cifra> ADR`, `<cifra> ADR in stato ...` e `<cifra> decisioni architetturali` nei
documenti di stato. Scrivere `2 ADR nuovi` la fa scattare, perché legge `2` come il
totale. **Per i numeri piccoli si usano le parole** — «due ADR nuovi» — e le cifre si
riservano ai conteggi veri. Gli esempi vanno nei code span, che la guardia ignora.

⚠️ La guardia ha due punti ciechi **dichiarati**: un numero scritto a parole le è
invisibile, e così pure `<cifra> decisioni` senza «architetturali». Entrambi hanno già
prodotto conteggi stantii in questo repository.

**2 · La numerazione delle sezioni.** Il controllo sui duplicati è **per file**, e il suo
regex cattura `^#{2,3} <numero>`. Quindi `### 7.4.1` verrebbe letto come un duplicato di
`### 7.4`. **Le sotto-sotto-sezioni si scrivono con `####`**, che il regex non cattura —
verificato sulle §5, §6, §7 e §8, che ne hanno una decina ciascuna.

**3 · Due tabelle della spec sono lette _per posizione_.** I controlli aggiunti dalla §8.6
non fanno analisi del testo: contano le celle.

| Tabella | Cosa lo script pretende |
|---|---|
| il **catalogo** §7.4.1 e §7.4.2 | l'**ultima** colonna è la contro-sonda, e la casella non è vuota. Una riga con meno celle dell'intestazione è un errore, non una scorciatoia tipografica — è così che è stata trovata la riga di V31 |
| le tabelle **§8.3 e §8.4** | **cinque** colonne, con l'ID in prima, lo **stato** in terza e l'**innesco** in quinta. Una colonna aggiunta o spostata rompe entrambe le asserzioni |

⚠️ **E i delimitatori sono intestazioni.** Il catalogo è delimitato da `#### 7.4.1` e
`#### 7.4.3`, la copertura da `## 8.`. Rinumerarle non è un ritocco: senza la guardia di
non-vacuità spegnerebbe i controlli **in verde** — gotcha #26. Con la guardia diventa un
rosso che nomina il delimitatore mancante, ed è il comportamento voluto.

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
