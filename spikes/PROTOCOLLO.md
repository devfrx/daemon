# Protocollo degli spike bloccanti

Criteri identici per ogni candidato. Si applicano **prima** di guardare i risultati.

Un candidato **passa** solo se soddisfa tutti i criteri del rispettivo spike.
Un criterio soddisfatto «con un accorgimento» va registrato come **parziale**, non
come passato: la differenza è tutta lì.

## Copertura — perché questi criteri e non altri

La spec [§9.4](../docs/superpowers/specs/2026-08-06-kernel-design.md) fissa cinque
criteri per l'ADR sul linguaggio. **Questa tabella è la prova che il protocollo li
copre**: un criterio della spec senza una riga qui è un buco, non una semplificazione.

| # | Criterio della spec §9.4 | Da | Coperto da |
|---|---|---|---|
| 1 | sostituibilità di **tempo, casualità, I/O e scheduling** | V29 · ADR-0021 | C1–C4 (tempo, casualità) · C5 (assenza di fonti globali) · **C7 (I/O)** · C6 (scheduling) |
| 2 | il sistema di tipi regge il confine dei dati non fidati | V19 · ADR-0014 | T1–T5 |
| 3 | verificabilità statica dell'assenza di chiamate OS nel kernel | I3 · ADR-0002 | **T6** |
| 4 | verificabilità statica dell'assenza di modelli nel percorso decisionale | V28 · ADR-0020 | **T6** — stessa forma: una regola di importazione vietata |
| 5 | adeguatezza a un daemon a vita lunga con **concorrenza reale** | ADR-0004 | C6 per la concorrenza · **O2** per la vita lunga, come osservazione |

I criteri 3 e 4 hanno la stessa forma — una regola sul grafo delle importazioni,
verificabile su tutto il progetto — e si misurano con **un solo** criterio. Sdoppiarli
sarebbe burocrazia, non rigore.

## SP-5 — Iniettabilità e riproducibilità

| # | Criterio | Come si verifica | Soglia |
|---|---|---|---|
| C1 | Due esecuzioni con lo **stesso seed** producono tracce di eventi identiche | confronto byte per byte | identiche |
| C2 | Due esecuzioni con **seed diversi** producono tracce diverse | confronto | diverse |
| C3 | Il tempo è **virtuale**: un'attesa di 5 secondi completa in millisecondi | tempo di parete del test | < 1 s |
| C4 | Un guasto iniettato in un punto scelto dal seed è **riproducibile a comando** | rieseguire con quel seed | stessa traccia, guasto incluso |
| C5 | Nessuna lettura dell'orologio di sistema o del generatore casuale globale nel codice sotto test | verifica statica sul progetto | nessun riscontro |
| C6 | Unità concorrenti **native** del linguaggio, in contesa, producono la stessa traccia a parità di seed | ≥3 unità native, ordine di acquisizione registrato, **100 esecuzioni** stesso seed | 100 tracce byte-identiche |
| **C7** | L'**I/O durevole** è iniettabile, e un crash al confine di persistenza è riproducibile | il «giornale» dello spike scrive attraverso un'interfaccia sostituita da un doppio; il seed sceglie il punto di crash | traccia identica a parità di seed, **crash incluso**; nessuna chiamata diretta al filesystem nel codice sotto test |

### Perché C7 esiste

V29 elenca **quattro** cose iniettabili: tempo, casualità, **I/O**, scheduling. C1–C6
ne coprivano tre. L'I/O non è un dettaglio in coda: la tecnica di verifica di Q5 in
[ADR-0021](../docs/adr/0021-simulazione-deterministica-e-iniettabilita.md) è la
*crash-injection ai confini di persistenza*, e il giornale write-ahead di
[ADR-0007](../docs/adr/0007-giornale-write-ahead-e-riconciliazione.md) è il confine
principale. Un linguaggio in cui l'I/O non è sostituibile rende Q5 non verificabile —
esattamente il rischio RK-3.

C7 verifica anche l'ordine write-ahead: **intento prima dell'effetto, esito dopo**. Un
crash fra i due deve lasciare un passo `InDubbio` rilevabile, non invisibile.

### Regola di applicazione di C6

Fissata **prima** di misurare, perché decide il peso dello spareggio #1 dell'ADR.

| Esito di C6 | Effetto sullo spareggio #1 «controllo posseduto vs fornito» |
|---|---|
| `passa` | **non si applica** a quel candidato: possiede il controllo, comunque lo ottenga |
| `parziale` | si applica, e l'evidenza dice **in quali condizioni** il controllo si perde |
| `non passa` | si applica in pieno, ora con una misura invece che con un'osservazione |

## SP-6 — Confine dei dati non fidati, e confini statici del kernel

| # | Criterio | Come si verifica | Soglia |
|---|---|---|---|
| T1 | Passare un valore non fidato dove è attesa un'istruzione **non compila** | test di compilazione fallita | il file non compila, e il test lo prova |
| T2 | Esiste **un solo** percorso di conversione, nominato ed esplicito | ricerca testuale sul progetto | una sola funzione |
| T3 | L'etichetta è **ereditaria**: una trasformazione su non fidato restituisce non fidato | test di tipo | il tipo restituito resta non fidato |
| T4 | Aggirare il divieto richiede un costrutto **evidente e cercabile** | vedi la regola di decisione sotto | vedi sotto |
| T5 | La violazione del confine dei tipi è rilevabile **su tutto il progetto** | comando unico | fallisce in presenza della violazione |
| **T6** | Una **regola di importazione vietata** è applicabile su tutto il progetto e **provata in negativo** | si introduce una violazione deliberata, il comando deve fallire; la si rimuove, deve passare | un comando solo, esito corretto in **entrambe** le direzioni |

### Perché T6 esiste, e perché è una sola riga

I criteri 3 e 4 della spec — nessuna chiamata OS nel kernel (I3), nessun modello nel
percorso decisionale (V28) — chiedono la stessa capacità: **vietare che un modulo ne
importi un altro, e verificarlo automaticamente su tutto il progetto.**

Senza T6 quei due criteri sarebbero risposti a opinione. I3 è una delle tre proprietà
che [HANDOFF.md](../docs/HANDOFF.md) elenca come non aggiungibili dopo.

**Provato in negativo** è la parte che conta: un controllo che passa sempre non prova
nulla. Vale la stessa lezione già imparata sul Task 7 di TypeScript, e sul gotcha #9
di Go — un controllo che fallisce per il motivo sbagliato è un falso positivo.

### Regola di decisione di T4

T4 era l'unico criterio senza soglia, ed è quello che decide contro i candidati
strutturalmente più deboli. Un criterio senza soglia è un'opinione con una tabella
intorno.

| Esito | Regola |
|---|---|
| `passa` | aggirare richiede un costrutto che il progetto può **vietare con il compilatore o con una toolchain obbligatoria**, e il divieto ricade sotto T6 |
| `parziale` | aggirare richiede un costrutto cercabile, ma il divieto dipende da una **convenzione applicata da uno strumento esterno configurabile**, disattivabile per riga |
| `non passa` | aggirare non richiede alcun costrutto distintivo, oppure il costrutto è **indistinguibile dall'uso legittimo** |

Va registrato **quale** costrutto, e con quale comando lo si cerca. «Facile» e
«difficile» non sono esiti.

## Osservazioni registrate — non sono criteri

Cose che vanno **sapute prima di scegliere**, ma che non giustificano un prototipo:
misurarle costerebbe più di quanto valgano, e trasformarle in criteri sarebbe
sovra-ingegnerizzazione. Si registrano con una fonte, e pesano semmai negli spareggi.

| # | Osservazione | Perché va saputa prima | Come si registra |
|---|---|---|---|
| **O1** | Esiste, nell'ecosistema, un motore di persistenza che soddisfa i **quattro requisiti di §10.6** (scrittura durevole ordinata · lettura concorrente · potatura selettiva · **I/O iniettabile**)? | [ADR-0022](../docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) rimanda la scelta a dopo il linguaggio. Se l'ecosistema non ne ha, lo si scopre **dopo** aver scelto | nome, versione, e quale dei quattro requisiti resta scoperto |
| **O2** | Il linguaggio ha una via consolidata per un **daemon a vita lunga, istanza singola**, con avvio automatico | criterio §9.4 n.5, parte non coperta da C6 | una riga con la fonte |

O1 non viola l'esclusione di scopo del piano — che è *scegliere* il motore. Qui si
registra soltanto che ne **esista** uno, il che è la differenza fra un ADR successivo
e una brutta sorpresa.

## Registrazione dei risultati

Per ogni candidato e ogni criterio: `passa` / `parziale` / `non passa`, più una riga
di evidenza (comando eseguito, output, versione degli strumenti).

**Un risultato senza seed o senza versione registrata non è valido.**

Le evidenze si scrivono da ciò che si misura. Dove il risultato diverge da un'attesa
scritta prima, **si registra la divergenza** invece di allinearsi all'attesa.

## Cosa questo protocollo non misura

Né ergonomia, né gusto, né popolarità, né velocità di scrittura. Non perché non
contino, ma perché non sono **falsificabili con un prototipo**. Entrano semmai negli
spareggi dell'ADR, dichiarati come giudizio e non spacciati per misura.

## Congelamento

**Questo protocollo è congelato al primo commit di codice di spike.** Da quel momento
una modifica richiede di registrare, qui: cosa è cambiato, perché, e **quali candidati
vanno rimisurati**.

Senza questa clausola nulla impedisce di aggiustare il metro dopo aver visto i
risultati — che è il modo esatto in cui uno spike smette di essere una misura e
diventa una giustificazione.
