# ADR-0011: La politica di routing è risolta e giornalata per ogni richiesta

- **Status:** Proposed
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

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

**3. Corollario accettato: ogni interazione con un modello è un passo di una run.**
Anche un singolo messaggio di chat. Una conversazione è una run interattiva di lunga
durata; una run agentica è la stessa struttura con più passi e meno attese.

È l'unificazione che rende universali i meccanismi della §4 invece che specifici
dell'agente: annullamento, ripresa, contabilità, tracciamento e tetti di spesa
funzionano allo stesso modo ovunque, senza un secondo percorso per la chat.

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
