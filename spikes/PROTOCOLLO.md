# Protocollo degli spike bloccanti

Criteri identici per ogni candidato. Si applicano **prima** di guardare i risultati.

Un candidato **passa** solo se soddisfa tutti i criteri del rispettivo spike.
Un criterio soddisfatto «con un accorgimento» va registrato come **parziale**, non
come passato: la differenza è tutta lì.

## SP-5 — Iniettabilità e riproducibilità

| # | Criterio | Come si verifica |
|---|---|---|
| C1 | Due esecuzioni con lo **stesso seed** producono tracce di eventi identiche | confronto byte per byte delle tracce |
| C2 | Due esecuzioni con **seed diversi** producono tracce diverse | se sono uguali non si sta esplorando nulla |
| C3 | Il tempo è **virtuale**: un'attesa di 5 secondi completa in millisecondi | tempo di parete del test < 1 s |
| C4 | Un guasto iniettato in un punto scelto dal seed è **riproducibile a comando** | rieseguire con quel seed lo riproduce |
| C5 | Nessuna lettura dell'orologio di sistema o del generatore casuale globale nel codice sotto test | verifica statica sul progetto |
| C6 | Unità concorrenti **native** del linguaggio, in contesa, producono la stessa traccia a parità di seed | ≥3 unità native, ordine di acquisizione registrato, **100 esecuzioni** con lo stesso seed: tutte byte-identiche |

C6 misura ciò che C1–C4 non toccano. Un esecutore scritto a mano si può scrivere in
qualunque linguaggio: la domanda vera è se il **parallelismo nativo** resta ordinabile
dal seed, perché è quello che il kernel userà (ADR-0004, concorrenza reale).

### Regola di applicazione di C6

Fissata **prima** di misurare, perché è ciò che decide il peso dello spareggio #1
dell'ADR sul linguaggio.

| Esito di C6 | Effetto sullo spareggio #1 «controllo posseduto vs fornito» |
|---|---|
| `passa` | **non si applica** a quel candidato: possiede il controllo, comunque lo ottenga |
| `parziale` | si applica, e l'evidenza dice **in quali condizioni** il controllo si perde |
| `non passa` | si applica in pieno, ora con una misura invece che con un'osservazione |

## SP-6 — Confine dei dati non fidati

| # | Criterio | Come si verifica |
|---|---|---|
| T1 | Passare un valore non fidato dove è attesa un'istruzione **non compila** | test di compilazione fallita |
| T2 | Esiste **un solo** percorso di conversione, nominato ed esplicito | ricerca testuale: una sola funzione |
| T3 | L'etichetta è **ereditaria**: una trasformazione su non fidato restituisce non fidato | test di tipo |
| T4 | Aggirare il divieto richiede un costrutto **evidente e cercabile** | si annota quale costrutto, e quanto è facile |
| T5 | La violazione è rilevabile **su tutto il progetto**, non solo dove ci si ricorda di controllare | comando unico che fallisce alla presenza di una violazione |

## Registrazione dei risultati

Per ogni candidato e ogni criterio: `passa` / `parziale` / `non passa`, più una riga
di evidenza (comando eseguito, output, versione degli strumenti).

**Un risultato senza seed o senza versione registrata non è valido.**

## Cosa si misura, e cosa no

Il protocollo non giudica ergonomia, gusto, popolarità né velocità di scrittura. Non
perché non contino, ma perché non sono **falsificabili con un prototipo**: entrano
semmai negli spareggi dell'ADR, dichiarati come giudizio e non spacciati per misura.

Le evidenze si scrivono da ciò che si misura. Dove il risultato diverge da un'attesa
scritta prima, si registra la divergenza invece di allinearsi all'attesa.
