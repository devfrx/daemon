# ADR-0038: Il registro delle funzioni del programma — un registro, molti invocatori, lo stesso permesso

- **Status:** Accepted
- **Date:** 2026-09-03
- **Deciders:** proprietario del progetto

> ⚠️ **Rimando del 2026-09-05 — la knowledge base registra le CRUD dei propri file e gruppi come
> funzioni del registro, e «aggiungi al contesto» ha due invocatori.** Il
> [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md) (§1.1g e
> §2.3, sotto accettazione condizionata, riletto dal proprietario il 2026-09-04) è il caso che
> questo ADR prevedeva con *«molti invocatori»*: la capacità del sotto-progetto 6 registra creare,
> leggere, aggiungere al contesto, aggiornare, spostare — dentro lo spazio, dentro da fuori, fuori —
> e cancellare, ciascuna un effetto con classe dichiarata, giornalato e checkpointato dentro
> l'ambito di [ADR-0024](0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md); il pannello della
> mappa e il modello sono **invocatori** delle stesse funzioni, con la stessa tripla di permesso, e
> nessuno dei due tocca un file di lato. Disegnare il grafo, filtrare e cercare sui nomi restano
> **presentazione** e non passano dal registro, come questo ADR già dice della manipolazione della
> GUI. Il registro lo costruisce ancora il primo invocatore, il click del sotto-progetto 2; la
> knowledge base vi si registra col 6. **Nessuna riga di questo ADR è superata.**

## Context

Le funzioni del programma — aprire un pannello, accendere la telecamera, catturare, cambiare
profilo, lanciare un comando rapido — esistono finora come intenzione: *«Comandi rapidi e
slash-command»* in [`tracciabilita.md`](../tracciabilita.md), sede GUI, e nient'altro. Il
brainstorming del riconoscimento gesti
([disegno del 2026-09-03](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md),
sezioni 1 e 3) ha posto la domanda che nessun ADR rispondeva: **chi può invocare una funzione
del programma, e con quale permesso?** Gli invocatori sono già quattro sulla carta — il click e
la tastiera della GUI, il gesto, la voce, e l'agente, perché il proprietario ha chiesto il
*«self-use dell'agente sulle funzioni del programma»* (terza domanda d'apertura del disegno).

Ciò che gli ADR già dicono, letto il 2026-09-03:

| ADR | Che cosa dà | Che cosa non dice |
|---|---|---|
| [ADR-0025](0025-confinamento-a-livelli.md) | il livello 1 *«resta ammesso solo per strumenti interni che non eseguono codice»* | dove quegli strumenti siano registrati, e chi possa invocarli |
| [ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) | due registri di kernel — guide e sensori — col principio *il kernel dà il meccanismo, le capacità portano il contenuto* | un registro delle **funzioni** |
| [ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md) | il permesso come tripla `(strumento × risorsa × operazione)`, e un'approvazione che **non si estende** | che la tripla valga anche per una funzione invocata a gesti o a voce |
| [ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md) | il contenuto non fidato **informa, mai autorizza** | che un evento di percezione sia di quella specie |
| [ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md) | la classe `irripetibile`, che sospende e chiede | chi possa confermare |

**Verificato nel sorgente il 2026-09-03:** non esiste un registro delle funzioni del programma.
`grep -n -i -E 'strument[oi] intern|registro degli strumenti|palette|scorciatoi' docs/superpowers/specs/2026-08-06-kernel-design.md docs/adr/*.md`
rende la sola riga di ADR-0025.

> ⚠️ **RICHIAMO DEL 2026-09-03, lo stesso giorno, dalla revisione del compito 1 del piano — *«la sola
> riga»* È FALSO DALLA NASCITA, e la conclusione regge.** Rilanciato sul repository di **prima** di
> questo file, il comando qui sopra —
> `git grep -n -i -E 'strument[oi] intern|registro degli strumenti|palette|scorciatoi' 4d16f33 -- docs/superpowers/specs/2026-08-06-kernel-design.md 'docs/adr/*.md'`
> — rende **sette** righe in cinque ADR, sei delle quali sul ramo `scorciatoi`; sul repository di
> oggi ne rende di più, perché questo stesso file porta le parole che cerca. Nessuna di quelle righe
> è un registro delle funzioni del programma, e l'unica riga **vicina** è quella di ADR-0025: è ciò
> che il disegno del 2026-09-03 diceva in §6.1, e che il piano ha irrigidito in *«la sola riga»*. Il
> comando che dice ciò che la frase voleva dire, appuntato al commit di prima di questo file perché
> non invecchi:
> `git grep -n -i -E 'strument[oi] intern|registro degli strumenti|palette' 4d16f33 -- docs/superpowers/specs/2026-08-06-kernel-design.md 'docs/adr/*.md'`
> → la sola riga 52 di ADR-0025. La frase qui sopra resta com'è: un ADR è append-only.
> ⚠️ **E tre precisazioni dalla stessa revisione.** La regola 7 attribuisce a G20 *«anche da
> tastiera e click»*: G20 dà la metà *«tastiera»*, e il click segue dalla regola 1. La tabella qui
> sopra dice che ADR-0007 non dice *«chi possa confermare»*: ADR-0007 dice *«chiedi all'utente»*,
> nomina il **chi** e tace la **strada**, che è ciò che la regola 4 decide. E la *Decision* non
> ripete che le funzioni del registro sono gli **strumenti interni** del livello 1 di ADR-0025, come
> la riga A della §3.1 del disegno diceva: lo dice la tabella qui sopra, e vale. Voce **E3**
> dell'errata del piano.

### Alternative considerate

- **Una logica «solo per gesti»** — un vocabolario di gesti che invoca direttamente le proprie
  azioni, dentro la capacità dei gesti.
  *Pro:* nessun meccanismo nuovo nel kernel.
  *Contro:* la stessa funzione avrebbe due strade con due permessi; la strada da tastiera si
  scriverebbe dopo, con un'altra regola. È la scorciatoia verso il kernel che
  [ADR-0001](0001-architettura-a-kernel-con-capacita-paritarie.md) vieta, e produce esattamente il
  **dialetto** che una capacità non può permettersi: una funzione che si comporta diversamente a
  seconda di chi la chiama.
- **Il registro dentro la GUI** — la GUI conosce i propri comandi, e li espone.
  *Pro:* il primo invocatore è la GUI, e sembra il posto naturale.
  *Contro:* la GUI è **sacrificabile** ([ADR-0004](0004-topologia-di-processo.md)): un registro che
  vive in essa muore con essa, e l'agente — che gira nel core, con la GUI chiusa — non lo
  raggiunge. E lo stato autorevole vive solo nel core (I1).
- **Un registro unico, meccanismo di kernel** — la scelta.
  *Pro:* una funzione si scrive una volta e si invoca da ogni strada con lo stesso permesso; il
  permesso e il giornale sono quelli che il kernel ha già.
  *Contro:* un meccanismo di kernel in più, prima di qualunque capacità; e ogni funzione con
  effetto va dichiarata come tripla.

## Decision

> **Il kernel espone un registro unico delle funzioni del programma — registrazione,
> invocazione, il permesso come tripla di ADR-0016, il giornale — e le capacità e la GUI portano
> le funzioni. Molti invocatori, lo stesso permesso: agente, gesto, voce, click. Nessuna logica
> «solo per gesti».**

| # | Regola |
|---|---|
| 1 | il registro è un **meccanismo di kernel** nella forma di ADR-0009: il kernel dà registrazione, invocazione, permesso e giornale; il **contenuto** — quali funzioni esistano — lo portano le capacità e la GUI, come le guide e i sensori |
| 2 | ogni funzione con effetto è dichiarata come **tripla** di ADR-0016, e l'invocazione passa dallo stesso permesso **qualunque sia l'invocatore**: un'azione invocata a gesti chiede ciò che chiederebbe da tastiera |
| 3 | un gesto, una trascrizione o un'istruzione del modello sono **eventi**: **informano, mai autorizzano** — ADR-0014 per analogia. Un evento non concede permessi |
| 4 | un effetto **irripetibile** (ADR-0007) chiede conferma a qualunque invocatore — è già così in ADR-0016 — e **per default la conferma non è gestuale**: un gesto letto male non deve poter confermare sé stesso (decisione 6 del disegno) |
| 5 | la **manipolazione** della GUI — spostare un pannello, aprire un menu virtuale — è stato di presentazione e **non passa dal registro** (ADR-0004: la GUI possiede solo presentazione) |
| 6 | **quali** funzioni siano gestuali, o vocali, lo decide la capacità che porta l'invocatore — per i gesti il sotto-progetto 12 — non questo ADR |
| 7 | un gesto **non è mai l'unica strada**: ogni funzione gestuale si raggiunge anche da tastiera e click (requisito G20 di [`spikes/GUI-REQUISITI.md`](../../spikes/GUI-REQUISITI.md)). Segue dalla regola 1: un registro con molti invocatori non ha funzioni di un invocatore solo |

### Perimetro negativo — cosa questa decisione **non** è

| Non è | |
|---|---|
| il **vocabolario** dei gesti, né l'elenco delle funzioni | contenuto, della capacità |
| un meccanismo **costruito** con questo ADR | nessun codice nasce qui: il registro lo costruisce chi porta il **primo invocatore**, il click del sotto-progetto 2 (§5.1 del disegno); il gesto lo aggiunge il 12, la voce l'8 |
| un **secondo** sistema di permessi | è la tripla di ADR-0016, con le sue tre modalità di supervisione |
| **estensibilità di terzi** | [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md) non cambia: le funzioni le portano le capacità e la GUI, non un plugin |
| le finestre dell'**OS** | fuori perimetro — sotto-progetto 10, con un ADR suo |

## Consequences

- **Positive:**
  - **Una funzione si scrive una volta** e si invoca da quattro strade con lo stesso permesso: la
    superficie di permesso non cresce con gli invocatori.
  - **L'agente usa il programma senza scorciatoie**, che è la parità di ADR-0001 resa concreta su
    un caso: passa dalla stessa porta dell'utente, con la stessa tripla.
  - **Un gesto letto male non può fare danno da solo**: informa, e ciò che ha effetto chiede il
    permesso che chiederebbe comunque.

- **Negative (accettate):**
  - **Un meccanismo di kernel in più prima di ogni capacità**: registrazione, invocazione,
    permesso e giornale vanno costruiti dal sotto-progetto 2 prima che una sola funzione esista.
  - **Ogni funzione con effetto va dichiarata come tripla**, anche quelle banali della GUI: il
    costo di dichiarazione pesa su tutte, e non solo su quelle che un gesto può invocare.
  - **Lo stesso permesso pesa anche sulle funzioni banali invocate dalla GUI**: un click che
    apre un pannello passa dalla stessa strada di un comando con effetto, e la strada deve
    restare leggera o la GUI la aggirerà.
  - **È di livello 0 finché non esiste**: niente impone che una funzione nuova passi dal
    registro finché il sotto-progetto 2 non lo costruisce e gli dà una sonda.

- **Follow-up richiesti:**
  - Il **sotto-progetto 2** costruisce il registro col primo invocatore, il click; il **12**
    aggiunge il gesto; l'**8** la voce. La riga è in [`tracciabilita.md`](../tracciabilita.md),
    sezione 2, accanto ai comandi rapidi.
  - ADR-0039 — la telecamera come sorgente di percezione, scritto insieme a questo — usa questa
    decisione: accendere la telecamera è una funzione del registro, e un gesto di comando entra
    dal registro.
