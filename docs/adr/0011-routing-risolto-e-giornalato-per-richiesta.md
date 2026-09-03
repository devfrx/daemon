# ADR-0011: La politica di routing è risolta e giornalata per ogni richiesta

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

> ⚠️ **Rimando del 2026-09-03 — nella tabella dell'inferenza percettiva always-on entra il
> tracciamento delle mani.** Il [disegno del riconoscimento gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md)
> e [ADR-0039](0039-telecamera-come-sorgente-di-percezione.md) leggono la riga *«wake word, VAD,
> trascrizione continua»* con un esempio in più: lo **stato continuo della mano** e il **gesto
> discreto** sono eventi di una sorgente di percezione, non passi — non passano dal gateway e
> non si giornalano per fotogramma. Un gesto di **comando** fa come la trascrizione che diventa
> messaggio: **apre un passo** in una run già aperta; i fotogrammi che l'hanno prodotto no. E
> per decisione 3 del disegno **solo la wake word apre una run**: la cella *«può avviare una
> run»* resta vera della voce, non del gesto. Nessuna riga è superata.

## Context

Il gateway decide, per ogni chiamata a un modello: quale modello, quale destinazione
(locale o remota), quale provider, quali parametri, quali vincoli sui dati, quale
catena di riserva. Queste scelte derivano da una configurazione che **cambia nel
tempo**: si aggiunge un provider, si modifica un ordine di preferenza, si alza un
tetto di spesa.

Il rischio non è l'errore, è il **silenzio**. Una modifica alla configurazione può
allargare il perimetro — per esempio ammettendo un provider che non rispetta un
requisito di ritenzione dati — senza che nessuna richiesta fallisca e senza che
nulla lo segnali. A posteriori diventa impossibile stabilire *dove sia finito cosa*.

Serve inoltre attribuire il costo. La mappa funzionale chiede contabilità per
messaggio, per sessione, **per run agentica e per sub-agente**: quattro granularità
che un contatore piatto non può produrre.

Alternative considerate:

- **Configurazione letta al momento dell'uso, nessun record.** Zero costo.
  *Contro:* nessuna riproducibilità, nessun audit, nessuna attribuzione di costo
  oltre il totale.
- **Sistema di contabilità separato** che intercetta le chiamate.
  *Contro:* una seconda fonte di verità che può divergere dal giornale, e che non sa
  nulla della gerarchia run → sub-run → passo.
- **Record di routing risolto, scritto nel giornale insieme al passo.**
  *Contro:* ogni chiamata scrive più dati.

## Decision

**1. Ogni richiesta produce un _record di routing risolto_**, giornalato insieme al
passo che l'ha originata: modello, destinazione, provider, parametri di generazione,
vincoli richiesti, catena di riserva valutata, tentativi effettuati, esito.

Il record contiene la decisione **risolta**, non un riferimento alla configurazione:
rileggere la configurazione di oggi non dice cosa accadde ieri.

**2. La contabilità cavalca l'identità del giornale.** Token e costo si attribuiscono
al passo; il passo appartiene a una run; una run può appartenere a una run padre.
Le quattro granularità richieste sono **aggregazioni della stessa gerarchia**, non
quattro contatori.

**3. Corollario accettato: ogni richiesta di inferenza _generativa_ è un passo di una
run.** Anche un singolo messaggio di chat. Una conversazione è una run interattiva di
lunga durata; una run agentica è la stessa struttura con più passi e meno attese.

È l'unificazione che rende universali i meccanismi della §4 invece che specifici
dell'agente: annullamento, ripresa, contabilità, tracciamento e tetti di spesa
funzionano allo stesso modo ovunque, senza un secondo percorso per la chat.

**Confine esplicito — l'inferenza percettiva sempre attiva non è un passo.** Wake
word, rilevazione della voce e trascrizione continua non passano dal gateway e non si
giornalano per frammento: sono **sorgenti di eventi** (anello 3, §5). Giornalarle
violerebbe Q1 e riempirebbe il giornale di rumore.

| | Inferenza **generativa** | Inferenza **percettiva** always-on |
|---|---|---|
| Esempi | chat, agente, riformulazioni, sensori inferenziali | wake word, VAD, trascrizione continua |
| Passa dal gateway | sì | no |
| Unità | **passo** di una run | **evento**, che può *avviare* una run |
| Contabilizzata | per token e costo | per risorsa GPU (§2), non per token |

Il confine è netto: la trascrizione che diventa un messaggio dell'utente **apre** un
passo; i frammenti audio che l'hanno prodotta no.

**4. Il costo si registra anche per gli stream interrotti.** Annullare uno stream
remoto può comunque generare addebito: se non venisse registrato, la contabilità
sarebbe sistematicamente ottimistica proprio nei casi in cui l'utente annulla di più.

## Consequences

- **Positive:**
  - Una run è **riproducibile**: si sa esattamente con quale modello, provider e
    parametri ogni passo è stato eseguito.
  - Un cambio di configurazione non riscrive il passato e non allarga il perimetro
    in silenzio: la differenza è visibile confrontando i record.
  - Nessun sottosistema di contabilità separato da tenere allineato.
  - I tetti di spesa possono agire a ogni livello della gerarchia con la stessa
    logica.
- **Negative (accettate):**
  - Ogni chiamata scrive più dati; il giornale cresce più in fretta. Serve una
    politica di ritenzione, che è materia della §7.
  - «Tutto è una run» impone la struttura della §4 anche al caso banale di un
    singolo messaggio. Il costo è due scritture durevoli su una latenza già
    dominata dalla chiamata al modello — accettabile, ma reale.
- **Follow-up richiesti:**
  - Il record di routing contiene nomi di provider e parametri, **mai credenziali**.
    Il mascheramento è un requisito, non un'accortezza (§6).
  - La ritenzione del giornale va decisa in §7: cresce indefinitamente, e nessuno
    vuole un audit trail che riempie il disco.
